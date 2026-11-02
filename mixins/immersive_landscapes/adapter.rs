// Immersive Landscapes, original MIT code by dormyu.
use super::immersive_landscapes_core::{Basin, Config, HeightField, Landscape, Surface};
use super::{Options, PackResources, Resources, Settings};
use horizon_client::{
    Result,
    biome::Biomes,
    chunk::{BIOME_STRATEGY, Chunk, LevelShape, Section, share_uniform},
    lod,
    save::{Persistent, SaveDir, WorldPreset},
    streaming::{ChunkGenerator, LodSampler, Proto, Streaming},
    vanillagen::VanillaTerrain,
};
use horizon_core::{
    biome::{BiomeZoom, obfuscate_seed},
    paletted::PalettedContainer,
};
use horizon_world::registry::{BlockRegistry, BlockStateId};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};

pub(super) const ID: &str = "immersive_landscapes";
const FILE: &str = "immersive-landscapes.json";

#[derive(Serialize, Deserialize)]
struct Descriptor {
    version: u32,
    seed: Option<i64>,
    side: usize,
    cell_size: f64,
    relief: f64,
}
impl Default for Descriptor {
    fn default() -> Self {
        Self {
            version: 1,
            seed: None,
            side: 257,
            cell_size: 128.0,
            relief: 1400.0,
        }
    }
}
impl Descriptor {
    fn write(&self, root: &Path) -> Result<()> {
        std::fs::create_dir_all(root)?;
        let temporary = root.join("immersive-landscapes.json.tmp");
        std::fs::write(&temporary, serde_json::to_vec_pretty(self)?)?;
        std::fs::rename(temporary, root.join(FILE))?;
        Ok(())
    }
}

pub(super) fn is_world(root: &Path) -> bool {
    root.join(FILE).is_file()
}

pub(super) fn shape() -> LevelShape {
    LevelShape::new(-64, 2096, 2096, true).expect("valid IL height range")
}

pub(super) fn prepare(
    request: &super::world_loading::WorldRequest,
    settings: &Settings,
) -> Result<()> {
    let root = &request.save_root;
    if is_world(root) {
        if settings.mods.disabled.contains(ID) {
            return Err(
                "This world uses Immersive Landscapes. Enable the mod before opening it.".into(),
            );
        }
        return Ok(());
    }
    // Only new normal worlds. Existing saves and other world presets keep their generator.
    if settings.mods.disabled.contains(ID)
        || root.join("level.dat").exists()
        || root.join("level.dat_old").exists()
        || root.join("region").exists()
        || root.join("dimensions").exists()
        || request
            .create
            .as_ref()
            .is_some_and(|o| o.world_gen().preset != WorldPreset::Normal)
    {
        return Ok(());
    }
    Descriptor::default().write(root)?;
    log::info!("Immersive Landscapes: new world {}", root.display());
    Ok(())
}

pub(super) type StreamResult = Result<(Streaming, Arc<SaveDir>, Option<Arc<VanillaTerrain>>)>;

#[allow(clippy::too_many_arguments)]
pub(super) fn streaming(
    res: &Resources,
    pack: &PackResources,
    options: &Options,
    settings: &Settings,
    root: &Path,
    shape: LevelShape,
    seed: i64,
    features: &[String],
) -> StreamResult {
    let mut descriptor: Descriptor = serde_json::from_slice(&std::fs::read(root.join(FILE))?)?;
    if descriptor.version != 1 || descriptor.seed.is_some_and(|saved| saved != seed) {
        return Err("Immersive Landscapes: incompatible generator version or world seed".into());
    }
    if shape != self::shape() {
        return Err("Immersive Landscapes: incompatible dimension height".into());
    }
    if descriptor.seed.is_none() {
        descriptor.seed = Some(seed);
        descriptor.write(root)?;
    }
    let start = std::time::Instant::now();
    let config = Config {
        seed: seed as u64,
        side: descriptor.side,
        cell_size: descriptor.cell_size,
        relief: descriptor.relief,
        sea_level: 63.0,
    };
    let mut field = HeightField::generate(config).map_err(str::to_owned)?;
    // The experimental island meets a level ocean at its boundary, never a clamped wall.
    let extent = (config.side - 1) as f64 * config.cell_size / 2.0;
    for i in 0..field.ground.len() {
        let [x, z] = field.point(i);
        let fade = ((extent - x.abs().max(z.abs())) / (config.cell_size * 8.0)).clamp(0.0, 1.0);
        let fade = fade * fade * (3.0 - 2.0 * fade);
        field.ground[i] = -12.0 + (field.ground[i] + 12.0) * fade;
    }
    let landscape = Arc::new(Landscape::new(Basin::compile(field)));
    log::info!(
        "Immersive Landscapes: region compiled in {:.1}ms, {} rivers, {} delta mouths",
        start.elapsed().as_secs_f64() * 1000.0,
        landscape.basin().rivers.len(),
        landscape.basin().deltas.len()
    );
    let biomes =
        super::world_loading::packed_biomes(root, &options.resources, res.biomes(), features);
    let inner = Arc::new(Terrain::new(landscape, pack.assets.registry, &biomes)?);
    let save = Arc::new(SaveDir::new(
        horizon_client::save::dimension_dir(root, "minecraft:overworld"),
        shape,
    ));
    let mut streaming = Streaming::new(
        Arc::new(Persistent {
            inner,
            save: save.clone(),
            registry: pack.assets.clone(),
            biomes,
        }),
        pack.context.clone(),
        res.threads().clamp(1, 2),
    );
    streaming.forced = save.load_forced_chunks();
    streaming.radius = settings.render_distance;
    streaming.biome_blend = settings.biome_blend;
    streaming.smooth_lighting = settings.smooth_lighting;
    streaming.cutout_leaves = settings.cutout_leaves;
    streaming.leaf_culling = settings
        .horizon
        .leaf_culling_effective(&settings.resource_packs);
    streaming.biome_zoom = BiomeZoom::new(obfuscate_seed(seed));
    streaming.sky_light = true;
    streaming.mesh = options.execution_role.is_client();
    Ok((streaming, save, None))
}

struct Terrain {
    landscape: Arc<Landscape>,
    stone: u32,
    dirt: u32,
    grass: u32,
    water: u32,
    sand: u32,
    gravel: u32,
    bedrock: u32,
    snow: u32,
    log: u32,
    leaves: u32,
    short_grass: u32,
    plains: u16,
    forest: u16,
    meadow: u16,
    peaks: u16,
    river: u16,
    ocean: u16,
}

#[derive(Clone, Copy)]
struct Column {
    height: i32,
    water: i32,
    top: u32,
    biome: u16,
    forest: f64,
}

impl Terrain {
    fn new(landscape: Arc<Landscape>, registry: &BlockRegistry, biomes: &Biomes) -> Result<Self> {
        let id = |name: &str| -> Result<u32> {
            Ok(registry
                .default_state(name)
                .ok_or_else(|| format!("Missing {name}"))?
                .0)
        };
        Ok(Self {
            landscape,
            stone: id("minecraft:stone")?,
            dirt: id("minecraft:dirt")?,
            grass: id("minecraft:grass_block")?,
            water: id("minecraft:water")?,
            sand: id("minecraft:sand")?,
            gravel: id("minecraft:gravel")?,
            bedrock: id("minecraft:bedrock")?,
            snow: id("minecraft:snow_block")?,
            log: id("minecraft:oak_log")?,
            leaves: registry
                .resolve(&horizon_world::chunk::BlockState {
                    name: "minecraft:oak_leaves".into(),
                    properties: [
                        ("distance".into(), "1".into()),
                        ("persistent".into(), "false".into()),
                        ("waterlogged".into(), "false".into()),
                    ]
                    .into(),
                })?
                .0,
            short_grass: id("minecraft:short_grass")?,
            plains: biomes.id("minecraft:plains"),
            forest: biomes.id("minecraft:forest"),
            meadow: biomes.id("minecraft:meadow"),
            peaks: biomes.id("minecraft:stony_peaks"),
            river: biomes.id("minecraft:river"),
            ocean: biomes.id("minecraft:ocean"),
        })
    }

    fn surface(&self, x: i32, z: i32) -> Surface {
        self.landscape
            .sample(f64::from(x), f64::from(z))
            .unwrap_or(Surface {
                ground: -12.0,
                water: Some(63.0),
                climate: super::immersive_landscapes_core::Climate {
                    temperature_c: 16.0,
                    rainfall_mm_year: 1000.0,
                    moisture: 0.5,
                },
            })
    }

    fn column(&self, x: i32, z: i32) -> Column {
        let s = self.surface(x, z);
        let height = s.ground.floor().clamp(-63.0, 2000.0) as i32;
        let water = s
            .water
            .map_or(height + 1, |w| w.ceil() as i32)
            .max(height + 1);
        let slope = (self.surface(x + 4, z).ground - self.surface(x - 4, z).ground)
            .hypot(self.surface(x, z + 4).ground - self.surface(x, z - 4).ground)
            / 8.0;
        let snowline = 1120.0 + variation(x, z, 128.0, 37) * 160.0;
        let alpine = ((s.ground - 650.0) / 430.0).clamp(0.0, 1.0);
        let forest = ((s.climate.moisture - 0.15) * 1.2).clamp(0.0, 0.8)
            * (1.0 - alpine)
            * (1.0 - (slope / 0.75).clamp(0.0, 1.0));
        let top = if height > snowline as i32 && slope < 0.85 {
            self.snow
        } else if slope > 0.65 || alpine > 0.75 {
            self.stone
        } else if water > height + 1 {
            if height <= 64 { self.sand } else { self.gravel }
        } else {
            self.grass
        };
        let biome = if water > height + 1 {
            if height < 61 { self.ocean } else { self.river }
        } else if alpine > 0.75 {
            self.peaks
        } else if variation(x, z, 96.0, 5) < forest {
            self.forest
        } else if alpine > 0.25 {
            self.meadow
        } else {
            self.plains
        };
        Column {
            height,
            water,
            top,
            biome,
            forest,
        }
    }

    fn generate(&self, cx: i32, cz: i32) -> Chunk {
        let columns: [Column; 256] = std::array::from_fn(|i| {
            self.column(cx * 16 + (i % 16) as i32, cz * 16 + (i / 16) as i32)
        });
        let mut biomes = PalettedContainer::new(BIOME_STRATEGY, u32::from(columns[0].biome));
        for by in 0..4 {
            for bz in 0..4 {
                for bx in 0..4 {
                    biomes.set(
                        (by * 4 + bz) * 4 + bx,
                        u32::from(columns[(bz * 4 + 2) * 16 + bx * 4 + 2].biome),
                    );
                }
            }
        }
        let mut chunk = Chunk::with_layout(cx, cz, shape().column());
        let minimum = columns.iter().map(|c| c.height).min().unwrap() - 4;
        let maximum = columns
            .iter()
            .map(|c| c.water.max(c.height + 1))
            .max()
            .unwrap();
        for si in 0..chunk.sections.len() {
            let low = -64 + si as i32 * 16;
            let mut section = Section {
                biomes: biomes.clone(),
                ..Section::default()
            };
            if low >= maximum {
                // Air stays a uniform, shared section, including most of the tall sky.
            } else if low > -64 && low + 15 < minimum {
                section.blocks =
                    PalettedContainer::new(horizon_client::chunk::block_strategy(), self.stone);
            } else {
                let blocks: [u32; 4096] = std::array::from_fn(|i| {
                    let y = low + (i / 256) as i32;
                    let c = columns[i % 256];
                    if y == -64 {
                        self.bedrock
                    } else if y > c.height {
                        if y < c.water { self.water } else { 0 }
                    } else if y < c.height - 3 {
                        self.stone
                    } else if c.top == self.sand || c.top == self.gravel {
                        c.top
                    } else if y == c.height {
                        c.top
                    } else if c.top == self.stone || c.top == self.snow {
                        self.stone
                    } else {
                        self.dirt
                    }
                });
                section.blocks = Section::from_blocks(&blocks);
            }
            chunk.sections[si] = Arc::new(section);
        }
        for z in 0..16 {
            for x in 0..16 {
                let c = columns[(z * 16 + x) as usize];
                if c.top == self.grass
                    && c.water <= c.height + 1
                    && hash(cx * 16 + x, cz * 16 + z, 17).is_multiple_of(17)
                {
                    chunk.set(x, c.height + 1, z, self.short_grass);
                }
            }
        }
        // Global cells, clipped into each chunk: canopies continue across chunk borders.
        for gz in (cz * 16 - 4).div_euclid(12)..=(cz * 16 + 19).div_euclid(12) {
            for gx in (cx * 16 - 4).div_euclid(12)..=(cx * 16 + 19).div_euclid(12) {
                let h = hash(gx, gz, self.landscape.basin().field.config.seed);
                let wx = gx * 12 + 3 + (h % 6) as i32;
                let wz = gz * 12 + 3 + ((h >> 8) % 6) as i32;
                let c = self.column(wx, wz);
                if c.top != self.grass
                    || c.water > c.height + 1
                    || ((h >> 16) % 1000) as f64 >= c.forest * 1000.0
                {
                    continue;
                }
                let (x, z) = (wx - cx * 16, wz - cz * 16);
                let trunk = 5 + ((h >> 32) % 3) as i32;
                for dy in 1..=trunk + 2 {
                    let radius: i32 = if dy < trunk - 2 {
                        0
                    } else if dy > trunk {
                        1
                    } else {
                        3
                    };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if (dx * dx + dz * dz > radius * radius + 1)
                                || !(0..16).contains(&(x + dx))
                                || !(0..16).contains(&(z + dz))
                            {
                                continue;
                            }
                            let block = if dx == 0 && dz == 0 && dy <= trunk {
                                self.log
                            } else {
                                self.leaves
                            };
                            if radius > 0 || block == self.log {
                                chunk.set(x + dx, c.height + dy, z + dz, block);
                            }
                        }
                    }
                }
            }
        }
        for section in &mut chunk.sections {
            Arc::make_mut(section).compact();
        }
        share_uniform(&mut chunk.sections);
        chunk
    }
}

struct Sampler(Arc<Terrain>);
impl LodSampler for Sampler {
    fn fork(&self) -> Option<Box<dyn LodSampler>> {
        Some(Box::new(Self(self.0.clone())))
    }
    fn sample(&mut self, x: i32, z: i32) -> Option<lod::Surface> {
        self.0.lod_surface(&mut (), x, z)
    }
}
impl ChunkGenerator for Terrain {
    type Worker = ();
    fn worker(&self) {}
    fn proto(&self, _: &mut (), x: i32, z: i32) -> Proto {
        Box::new(self.generate(x, z))
    }
    fn finish(&self, proto: Proto) -> Chunk {
        let mut chunk = *proto.downcast::<Chunk>().expect("IL chunk");
        chunk.fresh = true;
        chunk.generated = true;
        chunk
    }
    fn lod_sampler(self: Arc<Self>) -> Box<dyn LodSampler> {
        Box::new(Sampler(self))
    }
    fn lod_surface(&self, _: &mut (), x: i32, z: i32) -> Option<lod::Surface> {
        let c = self.column(x, z);
        Some(lod::Surface {
            height: c.height + 1,
            state: BlockStateId(c.top),
            biome: c.biome,
            water: (c.water > c.height + 1).then_some((c.water, BlockStateId(self.water))),
        })
    }
}

fn hash(x: i32, z: i32, seed: u64) -> u64 {
    let mut h = seed
        ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
        ^ (z as u64).wrapping_mul(0xd1b54a32d192ed03);
    h = (h ^ (h >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94d049bb133111eb);
    h ^ (h >> 31)
}
fn variation(x: i32, z: i32, scale: f64, seed: u64) -> f64 {
    let (x, z) = (f64::from(x) / scale, f64::from(z) / scale);
    let (ix, iz) = (x.floor() as i32, z.floor() as i32);
    let smooth = |v: f64| v * v * (3.0 - 2.0 * v);
    let (u, v) = (
        smooth(x.fract().rem_euclid(1.0)),
        smooth(z.fract().rem_euclid(1.0)),
    );
    let at = |x, z| (hash(x, z, seed) >> 11) as f64 / (1u64 << 53) as f64;
    (at(ix, iz) * (1.0 - u) + at(ix + 1, iz) * u) * (1.0 - v)
        + (at(ix, iz + 1) * (1.0 - u) + at(ix + 1, iz + 1) * u) * v
}
