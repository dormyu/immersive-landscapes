// Seed-bound native adapter. MIT, dormyu. No gameplay-frame hook.
use super::generator::{LodColumn, ProtoChunk, SurfaceSample};
use super::immersive_landscapes_spans::replace_run;
use super::immersive_landscapes_watershed::{Config, Land, NativeWater, Profile, Terrain, World};
use super::terrain::TerrainError;
use crate::registry::{BlockRegistry, BlockStateId};
use horizon_core::density::{DensitySampler, SampleError, SamplerContext};
use horizon_core::{
    geometry::Axis,
    noise::{DensityError, DensityVolume},
};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

static NEXT: AtomicUsize = AtomicUsize::new(1);
static PIPELINES: OnceLock<Mutex<HashMap<usize, Weak<Atlas>>>> = OnceLock::new();

#[derive(Debug)]
struct NativeTerrain {
    id: usize,
    preliminary: Arc<dyn DensitySampler>,
    continents: Arc<dyn DensitySampler>,
    density: Arc<dyn DensitySampler>,
    natural: bool,
    native_surface: Arc<OnceLock<NativeSurface>>,
    failed: AtomicBool,
    samples: Mutex<TerrainSamples>,
}

#[derive(Debug, Default)]
struct TerrainSamples {
    values: HashMap<(i32, i32), Land>,
    order: VecDeque<(i32, i32)>,
}

#[derive(Default)]
struct NativeWaterCache {
    values: HashMap<(i32, i32), Option<(SurfaceSample, bool)>>,
    order: VecDeque<(i32, i32)>,
}
struct NativeSurface {
    sample: Box<dyn Fn(i32, i32) -> Option<(SurfaceSample, bool)> + Send + Sync>,
    still_water: BlockStateId,
    cache: Mutex<NativeWaterCache>,
}
impl std::fmt::Debug for NativeSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("native unmodified surface resolver")
    }
}
impl NativeSurface {
    fn get(&self, x: i32, z: i32) -> Option<(SurfaceSample, bool)> {
        let key = (x, z);
        if let Some(value) = self
            .cache
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .values
            .get(&key)
        {
            return *value;
        }
        let sample = (self.sample)(x, z);
        let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        if cache.values.insert(key, sample).is_none() {
            cache.order.push_back(key);
            if cache.values.len() > 32768
                && let Some(old) = cache.order.pop_front()
            {
                cache.values.remove(&old);
            }
        }
        sample
    }
}

thread_local! {
    static RAW_DEPTH:Cell<u32>=const {Cell::new(0)};
    static DEM: RefCell<(usize, SamplerContext)> = RefCell::new((0, SamplerContext::cached()));
    static BUILD: RefCell<Vec<BuildFrame>> = const { RefCell::new(Vec::new()) };
    static PROFILES: RefCell<Vec<CachedProfile>> = RefCell::new(vec![CachedProfile::default(); 256]);
}

pub fn without_hydrology<T>(run: impl FnOnce() -> T) -> T {
    struct Restore(u32);
    impl Drop for Restore {
        fn drop(&mut self) {
            RAW_DEPTH.set(self.0);
        }
    }
    let old = RAW_DEPTH.get();
    RAW_DEPTH.set(old + 1);
    let restore = Restore(old);
    let value = run();
    drop(restore);
    value
}

impl Terrain for NativeTerrain {
    fn inland_water(&self, x: i32, z: i32) -> Option<f64> {
        let source = self.native_surface.get()?;
        let (sample, ocean) = source.get(x, z)?;
        let (level, state) = sample.water?;
        (!ocean && state == source.still_water && level > sample.height).then_some(f64::from(level))
    }

    fn land(&self, x: i32, z: i32) -> Land {
        let key = (x, z);
        if self.natural {
            let cache = self.samples.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(&land) = cache.values.get(&key) {
                return land;
            }
        }
        let land = DEM.with_borrow_mut(|state| {
            if state.0 != self.id { *state = (self.id, SamplerContext::cached()); }
            state.1.reset();
            // ChunkSurfaceLevel is an aquifer lower-bound estimate, NOT the
            // land surface. Using it as a DEM turns whole forests into a flat
            // below-sea catchment and cuts grid-shaped channels through them.
            let surface = if self.natural {
                let volume = DensityVolume::new([1,49,1],[x,-64,z],[1,8,1])
                    .expect("fixed native surface lattice");
                let mut samples = [0.0_f32;49];
                self.density.sample_volume(&mut state.1,&mut samples,volume).map(|()| {
                    let Some(i) = samples.iter().rposition(|&d|d>0.0) else {return -64.0;};
                    let bottom = -64.0+i as f32*8.0;
                    let crossing = if i+1<samples.len() {
                        samples[i]/(samples[i]-samples[i+1]).max(f32::EPSILON)
                    } else {0.0};
                    ((bottom+crossing*8.0).floor()+1.0).min(320.0)
                })
            } else {self.preliminary.sample_value(&mut state.1,[x,0,z])};
            let values = surface
                .and_then(|height| {
                    state.1.reset();
                    self.continents.sample_value(&mut state.1, [x, 0, z]).map(|c| (height, c))
                });
            match values {
                Ok((h, c)) if h.is_finite() && c.is_finite() => Land {
                    height: f64::from(h), ocean: h <= 63.0 && c < -0.08 },
                _ => {
                    if !self.failed.swap(true, Ordering::Relaxed) {
                        log::error!("Immersive Landscapes: hydrology's native terrain sampler failed; no invented coastal outlet will be used");
                    }
                    Land { height: 128.0, ocean: false }
                }
            }
        });
        if self.natural {
            let mut cache = self.samples.lock().unwrap_or_else(|e| e.into_inner());
            if cache.values.insert(key, land).is_none() {
                cache.order.push_back(key);
                if cache.values.len() > 32768
                    && let Some(old) = cache.order.pop_front()
                {
                    cache.values.remove(&old);
                }
            }
        }
        land
    }
}

#[derive(Debug)]
pub struct Atlas {
    id: usize,
    pub seed: i64,
    pub cache: Option<PathBuf>,
    world: World<NativeTerrain>,
    native_surface: Arc<OnceLock<NativeSurface>>,
    pub ecotones: OnceLock<super::immersive_landscapes_ecotone_runtime::Pass>,
}

#[derive(Clone, Copy, Default)]
struct CachedProfile {
    atlas: usize,
    x: i32,
    z: i32,
    profile: Option<Profile>,
}

impl Atlas {
    pub fn raster_floor(&self, profile: Profile, native_ground: i32) -> i32 {
        profile.raster_floor_versioned(
            native_ground,
            self.world.config.natural,
            self.world.config.advanced_rivers,
        )
    }

    pub fn raster_water(&self, profile: Profile, floor: i32, native_water: Option<i32>) -> i32 {
        profile.raster_water_versioned(
            floor,
            native_water,
            self.world.config.natural,
            self.world.config.refined_rivers,
        )
    }

    /// Final floor shared by actual chunks and both LOD column paths. This is
    /// deliberately outside profile()'s RefCell borrow: neighboring profiles
    /// use their unmodified base raster and cannot recurse into this method.
    pub fn raster_floor_at(&self, p: Profile, native_ground: i32, x: i32, z: i32) -> i32 {
        use super::immersive_landscapes_watershed::{WaterJoinColumn, waterfall_join_floor};
        let floor = self.raster_floor(p, native_ground);
        if !self.world.config.natural || !self.world.config.refined_rivers {
            return floor;
        }
        let native = self.native_land(x, z);
        if !p.channel_core && !p.lake && native.is_some_and(|s| s.water.is_some()) {
            return native_ground;
        }
        let water = self.raster_water(p, floor, native.and_then(|s| s.water.map(|w| w.0)));
        if water <= floor {
            return floor;
        }
        let current = WaterJoinColumn {
            floor,
            water,
            river_core: p.channel_core && p.strength >= 0.7,
        };
        let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)].map(|(dx, dz)| {
            let (nx, nz) = (x.checked_add(dx)?, z.checked_add(dz)?);
            let q = self.profile(nx, nz);
            // Most wet columns are flat. Their cached profile levels reject
            // the neighbor before an exact native surface query is needed.
            if let Some(q) = q {
                if !q.channel_core && !q.lake {
                    if !current.river_core {
                        return None;
                    }
                    // The envelope itself contributes no water, but an
                    // unchanged native pond/ice interval may exist underneath.
                    let native = self.native_land(nx, nz)?;
                    return Some(WaterJoinColumn {
                        floor: native.height,
                        water: native.water?.0,
                        river_core: false,
                    });
                }
                if q.voxel_water(true) > floor
                    || !(current.river_core || (q.channel_core && q.strength >= 0.7))
                {
                    return None;
                }
                let native = self.native_land(nx, nz)?;
                let base = self.raster_floor(q, native.height);
                Some(WaterJoinColumn {
                    floor: base,
                    water: self.raster_water(q, base, native.water.map(|w| w.0)),
                    river_core: q.channel_core && q.strength >= 0.7,
                })
            } else if current.river_core {
                // A native receiving water column can lie just outside the
                // river's footprint; its existing level/bed is authoritative.
                let native = self.native_land(nx, nz)?;
                Some(WaterJoinColumn {
                    floor: native.height,
                    water: native.water?.0,
                    river_core: false,
                })
            } else {
                None
            }
        });
        waterfall_join_floor(current, neighbors, true)
    }

    pub fn native_land(&self, x: i32, z: i32) -> Option<SurfaceSample> {
        self.native_surface.get()?.get(x, z).map(|v| v.0)
    }
    fn profile(&self, x: i32, z: i32) -> Option<Profile> {
        PROFILES.with_borrow_mut(|slots| {
            let hash = (x as u32).wrapping_mul(0x9e3779b9)
                ^ (z as u32).wrapping_mul(0x85ebca6b).rotate_left(13);
            let slot = (hash ^ (hash >> 16)) as usize % slots.len();
            let old = slots[slot];
            if old.atlas == self.id && old.x == x && old.z == z {
                return old.profile;
            }
            let mut profile = self.world.sample(x, z);
            if self.world.config.natural
                && let Some(p) = &mut profile
                && let Some(source) = self.native_surface.get()
            {
                let native = source.get(x, z).and_then(|(s, ocean)| {
                    s.water
                        .filter(|(_, state)| {
                            !self.world.config.advanced_rivers || *state == source.still_water
                        })
                        .map(|w| NativeWater {
                            ground: s.height,
                            water: w.0,
                            // Sea-level inland ponds are not automatically
                            // marine water. Keep their planned spill level;
                            // native ocean biomes retain the exact sea level.
                            ocean,
                        })
                });
                *p = super::immersive_landscapes_watershed::reconcile_native_water(
                    *p,
                    native,
                    self.world.config.sea as i32,
                );
            }
            slots[slot] = CachedProfile {
                atlas: self.id,
                x,
                z,
                profile,
            };
            profile
        })
    }
}

struct BuildFrame {
    token: usize,
    seed: i64,
    reentry: bool,
    cache: Option<PathBuf>,
    cache_identity: u64,
    config: Config,
    atlas: Option<Arc<Atlas>>,
}

pub struct BuildGuard {
    depth: usize,
    token: usize,
}
impl Drop for BuildGuard {
    fn drop(&mut self) {
        BUILD.with_borrow_mut(|stack| {
            assert_eq!(stack.len(), self.depth);
            assert_eq!(stack.last().map(|f| f.token), Some(self.token));
            stack.pop();
        });
    }
}

pub fn build_reentry(token: usize) -> bool {
    BUILD.with_borrow_mut(|stack| {
        if let Some(frame) = stack.last_mut()
            && frame.token == token
            && frame.reentry
        {
            frame.reentry = false;
            true
        } else {
            false
        }
    })
}

pub fn begin_build(token: usize, seed: i64, cache_identity: u64, config: Config) -> BuildGuard {
    // The server's worldgen data cannot choose a client's filesystem path.
    let cache = std::env::var_os("HORIZON_IL_CACHE_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::current_dir()
                .ok()
                .map(|p| p.join("horizon-mods/.horizon-mod-data/immersive_landscapes/hydrology-v1"))
        })
        .filter(|p| p.is_absolute());
    let depth = BUILD.with_borrow_mut(|stack| {
        stack.push(BuildFrame {
            token,
            seed,
            reentry: true,
            cache,
            cache_identity,
            config,
            atlas: None,
        });
        stack.len()
    });
    BuildGuard { depth, token }
}

pub fn building() -> bool {
    BUILD.with_borrow(|stack| !stack.is_empty())
}

pub fn bind_pipeline(
    pipeline: usize,
    sample: impl Fn(i32, i32) -> Option<(SurfaceSample, bool)> + Send + Sync + 'static,
    still_water: BlockStateId,
) {
    let atlas = BUILD.with_borrow(|stack| stack.last().and_then(|f| f.atlas.clone()));
    if let Some(atlas) = atlas {
        let _ = atlas.native_surface.set(NativeSurface {
            sample: Box::new(sample),
            still_water,
            cache: Mutex::new(NativeWaterCache::default()),
        });
        PROFILES.with_borrow_mut(|slots| slots.fill(CachedProfile::default()));
        let mut map = PIPELINES
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        map.retain(|_, weak| weak.strong_count() > 0);
        map.insert(pipeline, Arc::downgrade(&atlas));
    }
}

pub fn for_pipeline(pipeline: usize) -> Option<Arc<Atlas>> {
    PIPELINES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&pipeline)
        .and_then(Weak::upgrade)
}

/// Called from the native router's return hook, before aquifers/material rules.
pub fn wrap_router(samplers: &mut [Arc<dyn DensitySampler>; 8]) {
    let atlas = BUILD.with_borrow_mut(|stack| {
        let frame = stack.last_mut()?;
        if let Some(atlas) = &frame.atlas {
            return Some(atlas.clone());
        }
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let native_surface = Arc::new(OnceLock::new());
        let terrain = NativeTerrain {
            id,
            preliminary: samplers[6].clone(),
            continents: samplers[2].clone(),
            density: samplers[7].clone(),
            natural: frame.config.natural,
            native_surface: native_surface.clone(),
            failed: AtomicBool::new(false),
            samples: Mutex::new(TerrainSamples::default()),
        };
        let mut world = World::new(terrain, frame.seed as u64, frame.config);
        if let Some(directory) = &frame.cache {
            world = world.with_dem_cache(directory.clone(), frame.cache_identity);
        }
        let atlas = Arc::new(Atlas {
            id,
            seed: frame.seed,
            cache: frame.cache.clone(),
            world,
            native_surface,
            ecotones: OnceLock::new(),
        });
        frame.atlas = Some(atlas.clone());
        log::info!(
            "Immersive Landscapes: bound hydrology-v{} to native terrain for seed {}",
            if frame.config.natural { 2 } else { 1 },
            frame.seed
        );
        Some(atlas)
    });
    let Some(atlas) = atlas else {
        return;
    };
    let natural = atlas.world.config.natural;
    for field in [4, 5, 6, 7] {
        // Narrow rivers cannot be safely rasterized through the native 4x8x4
        // density lattice: it invents cuts in adjacent columns. New recipes
        // finish the exact banks and water before decoration in fill_proto.
        if natural && (field == 6 || field == 7) {
            continue;
        }
        samplers[field] = Arc::new(HydroSampler {
            base: samplers[field].clone(),
            field,
            atlas: atlas.clone(),
        });
    }
}

#[derive(Debug)]
struct HydroSampler {
    base: Arc<dyn DensitySampler>,
    field: usize,
    atlas: Arc<Atlas>,
}

fn river_density(base: f32, p: Profile, y: i32) -> f32 {
    if p.lake {
        return base;
    }
    let yy = f64::from(y);
    let carve = ((p.bed - yy) * 0.2) as f32;
    let mut density = base.min(carve);
    if yy <= p.bed && yy > p.bed - 4.0 {
        let floor = ((p.bed - yy) * 0.25).min((yy - p.bed + 4.0) * 0.5) as f32;
        density = density.max(floor);
    }
    if yy < p.bed - 4.0 {
        density = base;
    }
    base + (density - base) * p.strength as f32
}

impl DensitySampler for HydroSampler {
    fn sample_value(
        &self,
        context: &mut SamplerContext,
        [x, y, z]: [i32; 3],
    ) -> Result<f32, SampleError> {
        if RAW_DEPTH.get() > 0 {
            return self.base.sample_value(context, [x, y, z]);
        }
        let Some(p) = self.atlas.profile(x, z) else {
            return self.base.sample_value(context, [x, y, z]);
        };
        match self.field {
            // Climate depth follows the carved surface instead of classifying
            // the new riverbank as an underground biome.
            4 => {
                let shift = (p.raw - p.bed).round().clamp(-128.0, 128.0) as i32;
                self.base
                    .sample_value(context, [x, y.saturating_add(shift), z])
            }
            // The biome classifier sees a gradual riparian zone. Underlying
            // terrain nodes were compiled before this sampler was wrapped.
            5 => {
                let base = self.base.sample_value(context, [x, y, z])?;
                Ok(if self.atlas.world.config.natural && p.lake {
                    base
                } else {
                    base * (1.0 - p.strength as f32)
                })
            }
            6 => Ok(p.bed as f32),
            7 => {
                let base = self.base.sample_value(context, [x, y, z])?;
                // A four-block liner keeps a river above a cave from hanging
                // in air; deeper native caverns and aquifers remain intact.
                Ok(river_density(base, p, y))
            }
            _ => unreachable!(),
        }
    }

    fn sample_volume(
        &self,
        context: &mut SamplerContext,
        buffer: &mut [f32],
        volume: DensityVolume,
    ) -> Result<(), SampleError> {
        if buffer.len() < volume.size() {
            return Err(DensityError::BufferTooShort.into());
        }
        // Keep the native vectorized/cache-aware path, including its floating
        // point behavior, throughout unchanged terrain. Profile lookup is 2D.
        self.base.sample_volume(context, buffer, volume)?;
        if RAW_DEPTH.get() > 0 {
            return Ok(());
        }
        let [sx, sy, sz] = volume.sizes();
        for iz in 0..sz {
            let z = volume.block(Axis::Z, iz).expect("validated volume");
            for ix in 0..sx {
                let x = volume.block(Axis::X, ix).expect("validated volume");
                let Some(p) = self.atlas.profile(x, z) else {
                    continue;
                };
                for iy in 0..sy {
                    let y = volume.block(Axis::Y, iy).expect("validated volume");
                    let value = &mut buffer[(iz * sx + ix) * sy + iy];
                    *value = match self.field {
                        4 => {
                            let shift = (p.raw - p.bed).round().clamp(-128., 128.) as i32;
                            self.base
                                .sample_value(context, [x, y.saturating_add(shift), z])?
                        }
                        5 => {
                            if self.atlas.world.config.natural && p.lake {
                                *value
                            } else {
                                *value * (1. - p.strength as f32)
                            }
                        }
                        6 => p.bed as f32,
                        7 => river_density(*value, p, y),
                        _ => unreachable!(),
                    };
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Paint {
    air: BlockStateId,
    stone: BlockStateId,
    water: BlockStateId,
    gravel: BlockStateId,
    sand: BlockStateId,
    clay: BlockStateId,
}
impl Paint {
    fn new(registry: &BlockRegistry) -> Self {
        Self {
            air: registry
                .default_state("minecraft:air")
                .expect("native air state"),
            stone: registry
                .default_state("minecraft:stone")
                .expect("native stone state"),
            water: registry
                .default_state("minecraft:water")
                .expect("native water state"),
            gravel: registry
                .default_state("minecraft:gravel")
                .expect("native gravel state"),
            sand: registry
                .default_state("minecraft:sand")
                .expect("native sand state"),
            clay: registry
                .default_state("minecraft:clay")
                .expect("native clay state"),
        }
    }
    fn bed(self, x: i32, z: i32, seed: i64) -> BlockStateId {
        let h = (x.div_euclid(4) as u64).wrapping_mul(0x9e3779b97f4a7c15)
            ^ (z.div_euclid(4) as u64).wrapping_mul(0xd1b54a32d192ed03)
            ^ seed as u64;
        match (h ^ (h >> 29)) % 10 {
            0 => self.clay,
            1..=3 => self.sand,
            _ => self.gravel,
        }
    }
}

#[derive(Clone, Copy)]
struct RasterColumn {
    floor: i32,
    water: i32,
    top: BlockStateId,
    cut: bool,
    bed_finish: bool,
}

fn raster_column(atlas: &Atlas, p: Profile, paint: Paint, x: i32, z: i32) -> Option<RasterColumn> {
    let (native, _) = atlas.native_surface.get()?.get(x, z)?;
    if atlas.world.config.refined_rivers && !p.channel_core && !p.lake && native.water.is_some() {
        // A dry bank envelope is not authority over an existing native pond,
        // ice or aquifer. Leave all three native paths unchanged, including the
        // original fluid block state, instead of repainting it as source water.
        return None;
    }
    let floor = atlas.raster_floor_at(p, native.height, x, z);
    let water = atlas.raster_water(p, floor, native.water.map(|w| w.0));
    let cut = floor < native.height;
    let bed_finish = (cut || !p.lake) && water > floor;
    let top = if bed_finish {
        paint.bed(x, z, atlas.seed)
    } else {
        native.state
    };
    Some(RasterColumn {
        floor,
        water,
        top,
        cut,
        bed_finish,
    })
}

fn fill_natural_proto(
    proto: &mut ProtoChunk,
    atlas: &Atlas,
    registry: &BlockRegistry,
) -> Result<(), TerrainError> {
    let paint = Paint::new(registry);
    let pos = proto.position();
    let min = proto.terrain.min_y();
    let max = min + proto.terrain.height();
    for z in 0..16 {
        for x in 0..16 {
            let wx = pos.x * 16 + x;
            let wz = pos.z * 16 + z;
            let Some(p) = atlas.profile(wx, wz) else {
                continue;
            };
            let Some(plan) = raster_column(atlas, p, paint, wx, wz) else {
                continue;
            };
            let floor = plan.floor.clamp(min + 1, max);
            let water = plan.water.clamp(min, max);
            let old_top = proto.surface_height(x, z).clamp(min, max);
            if plan.cut {
                for y in floor.max(water)..old_top {
                    proto.terrain.set_block_worldgen(x, y, z, paint.air)?;
                }
                for y in (floor - 4).max(min)..floor - 1 {
                    proto.terrain.set_block_worldgen(x, y, z, paint.stone)?;
                }
            }
            // Newly wet river floors receive the same sediment in real chunks
            // and both LOD sampling paths, even when no lowering was needed.
            // Never cap an existing carver opening with an unsupported block.
            let supported_floor = !matches!(
                proto.block_name(x, floor - 1, z),
                "minecraft:air"
                    | "minecraft:cave_air"
                    | "minecraft:void_air"
                    | "minecraft:water"
                    | "minecraft:lava"
            );
            if plan.cut || (plan.bed_finish && supported_floor) {
                proto
                    .terrain
                    .set_block_worldgen(x, floor - 1, z, plan.top)?;
            }
            // Preserved lakes/ocean keep native materials and aquifers. Only a
            // receiving inland depression may fill above its original surface.
            let start = if plan.cut { floor } else { old_top };
            if water > floor {
                for y in start..water {
                    proto.terrain.set_block_worldgen(x, y, z, paint.water)?;
                }
                if water > start {
                    proto.terrain.mark_postprocessing(x, water - 1, z);
                }
            }
        }
    }
    Ok(())
}

pub fn fill_proto(
    proto: &mut ProtoChunk,
    atlas: &Atlas,
    registry: &BlockRegistry,
) -> Result<(), TerrainError> {
    if atlas.world.config.natural {
        return fill_natural_proto(proto, atlas, registry);
    }
    let paint = Paint::new(registry);
    let pos = proto.position();
    let min = proto.terrain.min_y();
    let max = min + proto.terrain.height();
    for z in 0..16 {
        for x in 0..16 {
            let wx = pos.x * 16 + x;
            let wz = pos.z * 16 + z;
            let Some(p) = atlas.profile(wx, wz) else {
                continue;
            };
            let water = p.voxel_water(false).clamp(min, max);
            let strong = p.strength >= 0.7 && !p.lake;
            let floor = if strong {
                p.voxel_floor(atlas.world.config.natural)
            } else {
                proto.surface_height(x, z)
            }
            .clamp(min + 1, max);
            if water <= floor {
                continue;
            }
            if strong {
                // Native density cells are wider than the smallest streams.
                // Finish the cut per block, removing any interpolated roof and
                // backing falling sediment with the same four-block liner.
                let old_top = proto.surface_height(x, z).clamp(min, max);
                for y in water..old_top {
                    proto.terrain.set_block_worldgen(x, y, z, paint.air)?;
                }
                for y in (floor - 4).max(min)..floor - 1 {
                    proto.terrain.set_block_worldgen(x, y, z, paint.stone)?;
                }
                proto
                    .terrain
                    .set_block_worldgen(x, floor - 1, z, paint.bed(wx, wz, atlas.seed))?;
            }
            for y in floor..water {
                proto.terrain.set_block_worldgen(x, y, z, paint.water)?;
            }
            proto.terrain.mark_postprocessing(x, water - 1, z);
        }
    }
    Ok(())
}

pub fn fill_surface(
    surface: &mut Option<SurfaceSample>,
    atlas: &Atlas,
    registry: &BlockRegistry,
    x: i32,
    z: i32,
) {
    let Some(sample) = surface else {
        return;
    };
    let Some(p) = atlas.profile(x, z) else {
        return;
    };
    let paint = Paint::new(registry);
    if atlas.world.config.natural {
        if let Some(plan) = raster_column(atlas, p, paint, x, z) {
            sample.height = plan.floor;
            // Dry uncut columns and preserved water keep the engine palette;
            // modified river surfaces share the exact finish with real chunks.
            if plan.cut || plan.bed_finish {
                sample.state = plan.top;
            }
            sample.water = (plan.water > plan.floor).then_some((plan.water, paint.water));
        }
        return;
    }
    let strong = p.strength >= 0.7 && !p.lake;
    let floor = if strong {
        p.voxel_floor(atlas.world.config.natural)
    } else {
        sample.height
    };
    let water = p.voxel_water(false);
    if water <= floor {
        return;
    }
    if strong {
        sample.height = floor;
        sample.state = paint.bed(x, z, atlas.seed);
    }
    sample.water = Some((water, paint.water));
}

pub fn fill_column(
    column: &mut LodColumn,
    atlas: &Atlas,
    registry: &BlockRegistry,
    x: i32,
    z: i32,
) {
    let Some(p) = atlas.profile(x, z) else {
        return;
    };
    let Some(sample) = &column.0 else {
        return;
    };
    let paint = Paint::new(registry);
    if atlas.world.config.natural {
        if let Some(plan) = raster_column(atlas, p, paint, x, z) {
            if plan.cut {
                column.1.retain_mut(|run| {
                    run.1 = run.1.min(plan.floor);
                    run.0 < run.1
                });
                replace_run(&mut column.1, plan.floor - 4, plan.floor - 1, paint.stone);
            }
            if plan.cut || plan.bed_finish {
                replace_run(&mut column.1, plan.floor - 1, plan.floor, plan.top);
            }
            if plan.water > plan.floor {
                replace_run(&mut column.1, plan.floor, plan.water, paint.water);
            }
            fill_surface(&mut column.0, atlas, registry, x, z);
        }
        return;
    }
    let strong = p.strength >= 0.7 && !p.lake;
    let floor = if strong {
        p.voxel_floor(atlas.world.config.natural)
    } else {
        sample.height
    };
    let water = p.voxel_water(false);
    if water <= floor {
        return;
    }
    if strong {
        column.1.retain_mut(|run| {
            run.1 = run.1.min(water);
            run.0 < run.1
        });
        replace_run(&mut column.1, floor - 4, floor - 1, paint.stone);
        replace_run(&mut column.1, floor - 1, floor, paint.bed(x, z, atlas.seed));
    }
    replace_run(&mut column.1, floor, water, paint.water);
    fill_surface(&mut column.0, atlas, registry, x, z);
}

pub fn fill_grid(
    columns: &mut [LodColumn],
    atlas: &Atlas,
    registry: &BlockRegistry,
    x: i32,
    z: i32,
    width: i32,
    side: usize,
) {
    for (i, column) in columns.iter_mut().enumerate() {
        fill_column(
            column,
            atlas,
            registry,
            x + (i % side) as i32 * width,
            z + (i / side) as i32 * width,
        );
    }
}

pub fn live_atlases() -> Vec<Arc<Atlas>> {
    let mut atlases: Vec<_> = PIPELINES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .values()
        .filter_map(Weak::upgrade)
        .collect();
    atlases.sort_unstable_by_key(|a| a.id);
    atlases.dedup_by_key(|a| a.id);
    atlases
}

impl Atlas {
    pub fn profile_at(&self, x: i32, z: i32) -> Option<Profile> {
        self.profile(x, z)
    }

    pub fn cache_counts(&self) -> (usize, usize) {
        self.world.cache.as_ref().map_or((0, 0), |c| {
            (
                c.hits.load(Ordering::Relaxed),
                c.misses.load(Ordering::Relaxed),
            )
        })
    }

    pub fn inspect_coast(
        &self,
        x: i32,
        z: i32,
    ) -> Option<Arc<super::immersive_landscapes_watershed::Basin>> {
        self.world
            .coast(super::immersive_landscapes_watershed::Key(x, z))
            .map(|c| self.world.basin(c))
    }
}
