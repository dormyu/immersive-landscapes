// Original bounded coastal drainage, MIT, dormyu.
// Priority-flood concepts: Barnes/Lehman/Mulla (2014); bounded-domain inspiration:
// Davis, Plate-Local River Generation (2026). No reference implementation copied.
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Clone, Copy, Debug)]
pub struct Land {
    pub height: f64,
    pub ocean: bool,
}

pub trait Terrain: std::fmt::Debug + Send + Sync {
    fn land(&self, x: i32, z: i32) -> Land;

    /// An existing inland water surface, resolved without this drainage overlay.
    /// The optional default keeps synthetic DEM tools and older recipes exact.
    fn inland_water(&self, _x: i32, _z: i32) -> Option<f64> {
        None
    }
}

// Only the expensive input DEM is persisted. Drainage is rebuilt from that DEM
// so changes to routing never reuse a stale graph. This is disposable world data.
#[derive(Debug)]
pub struct DemCache {
    pub directory: PathBuf,
    identity: u64,
    pub hits: AtomicUsize,
    pub misses: AtomicUsize,
}

impl DemCache {
    pub fn new(directory: PathBuf, identity: u64) -> Self {
        Self {
            directory,
            identity,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    fn checksum(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf29ce484222325_u64, |h, &b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        })
    }

    fn samples(
        &self,
        seed: u64,
        coast: Coast,
        config: Config,
        sample: impl FnOnce() -> Vec<Land>,
    ) -> Vec<Land> {
        use std::io::Write;
        let n = (config.radius * 2 / config.step + 1) as usize;
        let count = n * n;
        let mut header = b"ILDEM001".to_vec();
        header.extend_from_slice(&self.identity.to_le_bytes());
        header.extend_from_slice(&seed.to_le_bytes());
        for value in [
            coast.key.0,
            coast.key.1,
            coast.x,
            coast.z,
            config.radius,
            config.step,
        ] {
            header.extend_from_slice(&value.to_le_bytes());
        }
        let name = format!(
            "dem-{:016x}-{}-{}.bin",
            Self::checksum(&header),
            coast.key.0,
            coast.key.1
        );
        let path = self.directory.join(name);
        let expected = header.len() + count * 9 + 8;
        let safe = self
            .directory
            .ancestors()
            .all(|p| match std::fs::symlink_metadata(p) {
                Ok(m) => !m.file_type().is_symlink(),
                Err(e) => e.kind() == std::io::ErrorKind::NotFound,
            });
        if safe
            && let Ok(meta) = std::fs::symlink_metadata(&path)
            && meta.is_file()
            && !meta.file_type().is_symlink()
            && meta.len() == expected as u64
            && let Ok(bytes) = std::fs::read(&path)
            && bytes.len() == expected
            && bytes.starts_with(&header)
        {
            let end = expected - 8;
            let checksum =
                u64::from_le_bytes(bytes[end..].try_into().expect("eight checksum bytes"));
            if checksum == Self::checksum(&bytes[..end]) {
                let mut data = Vec::with_capacity(count);
                for cell in bytes[header.len()..end].as_chunks::<9>().0 {
                    let height =
                        f64::from_le_bytes(cell[..8].try_into().expect("eight height bytes"));
                    if !height.is_finite() || cell[8] > 1 {
                        break;
                    }
                    data.push(Land {
                        height,
                        ocean: cell[8] == 1,
                    });
                }
                if data.len() == count {
                    self.hits.fetch_add(1, AtomicOrdering::Relaxed);
                    return data;
                }
            }
        }
        self.misses.fetch_add(1, AtomicOrdering::Relaxed);
        let data = sample();
        assert_eq!(data.len(), count);
        if !safe || !data.iter().all(|v| v.height.is_finite()) {
            return data;
        }
        let mut bytes = header;
        for land in &data {
            bytes.extend_from_slice(&land.height.to_le_bytes());
            bytes.push(u8::from(land.ocean));
        }
        bytes.extend_from_slice(&Self::checksum(&bytes).to_le_bytes());
        static NEXT_FILE: AtomicUsize = AtomicUsize::new(0);
        let temp = self.directory.join(format!(
            ".dem-{}-{}.tmp",
            std::process::id(),
            NEXT_FILE.fetch_add(1, AtomicOrdering::Relaxed)
        ));
        // create_new avoids following an existing temporary-file link. A bad
        // final cache file is derived data; a final symlink is never replaced.
        let _ = (|| -> std::io::Result<()> {
            std::fs::create_dir_all(&self.directory)?;
            if std::fs::symlink_metadata(&path)
                .is_ok_and(|m| m.file_type().is_symlink() || !m.is_file())
            {
                return Ok(());
            }
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp)?;
            file.write_all(&bytes)?;
            drop(file);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
            std::fs::rename(&temp, &path)
        })();
        // Only our unique file, never a recursive delete or a world file.
        let _ = std::fs::remove_file(&temp);
        data
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub sea: f64,
    pub step: i32,
    pub radius: i32,
    pub root_cell: i32,
    pub margin: f64,
    pub min_cells: f64,
    pub basin_cache: usize,
    pub natural: bool,
    /// Explicit rivers_v5 recipe marker; absent in every v8/v9 saved world.
    pub advanced_rivers: bool,
    /// Explicit rivers_v6 recipe marker; keep the published v10 shape without it.
    pub refined_rivers: bool,
    pub meander_amplitude: f64,
    pub meander_scale: f64,
    pub width_scale: f64,
    pub depth_scale: f64,
    pub bank_scale: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sea: 63.,
            step: 16,
            radius: 1024,
            root_cell: 768,
            margin: 24.,
            min_cells: 8.,
            basin_cache: 8,
            natural: true,
            advanced_rivers: false,
            refined_rivers: false,
            meander_amplitude: 14.,
            meander_scale: 96.,
            width_scale: 1.,
            depth_scale: 1.,
            bank_scale: 3.,
        }
    }
}

impl Config {
    fn fades(self) -> (f64, f64) {
        let radial = (f64::from(self.radius) * 0.25).min(64.);
        (radial.min(32.), radial)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(pub i32, pub i32);

#[derive(Clone, Copy, Debug)]
pub struct Coast {
    pub key: Key,
    pub x: i32,
    pub z: i32,
    /// Only fallback roots may terminate in a native inland lake.
    pub inland_level: Option<f64>,
}

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub from: [f64; 2],
    pub to: [f64; 2],
    pub water: [f64; 2],
    pub cells: f64,
    pub half_width: f64,
    pub depth: f64,
    pub mouth: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Profile {
    pub bed: f64,
    pub water: f64,
    pub strength: f64,
    pub raw: f64,
    /// Keep the existing terrain below this water body, including native sea.
    pub lake: bool,
    /// A planned channel still crosses this column if a lake overlays its bed.
    pub channel_core: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct NativeWater {
    pub ground: i32,
    pub water: i32,
    pub ocean: bool,
}

/// An unmodified native water level distinguishes a sea column from an inland
/// lake whose BED happens to lie below sea. Density sign alone cannot do that.
pub fn reconcile_native_water(mut p: Profile, native: Option<NativeWater>, sea: i32) -> Profile {
    if let Some(native) = native
        && native.ground < native.water
        && native.water >= sea
    {
        let inland_fill = p.lake && !native.ocean;
        p.raw = f64::from(native.ground);
        p.bed = f64::from(native.ground);
        p.water = if inland_fill {
            p.water.max(f64::from(native.water))
        } else {
            f64::from(native.water)
        };
        p.strength = 1.;
        // Existing native water already has a real bottom/material palette.
        // The renderer/final terrain pass must not recut it as a river liner.
        p.lake = true;
        p.channel_core = false;
    }
    p
}

fn voxel_ceil(height: f64, natural: bool) -> i32 {
    // Interpolating two equal integer levels can produce 63.00000000000001.
    // That is numerical error, not an extra metre of water. Saved v7 worlds
    // retain their exact old ceil behavior.
    let integer = height.round();
    if natural && (height - integer).abs() <= 1e-7 {
        integer as i32
    } else {
        height.ceil() as i32
    }
}

impl Profile {
    pub fn voxel_water(self, natural: bool) -> i32 {
        voxel_ceil(self.water, natural)
    }

    pub fn raster_floor(self, native_ground: i32, natural: bool) -> i32 {
        self.raster_floor_versioned(native_ground, natural, false)
    }

    pub fn raster_floor_versioned(
        self,
        native_ground: i32,
        natural: bool,
        advanced_rivers: bool,
    ) -> i32 {
        if !natural {
            return self.voxel_floor(false);
        }
        if self.lake {
            let water = self.voxel_water(true);
            // A coarse DEM may miss a low native saddle on the channel route.
            // Join it at the lake level, never restore the older deep river bed
            // through an existing lake or turn a high island into a deep slot.
            // v10 routes include inland lake outlets. The native seed
            // 4312740467388825473 has a four-column saddle at y74..75
            // across a y70 lake; the legacy three-block limit left its two
            // water components disconnected. Bound the new cut to six blocks
            // above water (at most seven excavated), and keep v8/v9 unchanged.
            let saddle_height = if advanced_rivers { 6 } else { 3 };
            if self.channel_core && native_ground >= water && native_ground - water <= saddle_height
            {
                return water - 1;
            }
            return native_ground;
        }
        let strength = self.strength.clamp(0., 1.);
        let target = f64::from(native_ground) + (self.bed - f64::from(native_ground)) * strength;
        let floor = voxel_ceil(target, true).min(native_ground);
        if strength >= 0.7 && target < self.water {
            floor.min(self.voxel_water(true) - 1)
        } else {
            floor
        }
    }

    pub fn voxel_floor(self, natural: bool) -> i32 {
        let floor = voxel_ceil(self.bed, natural);
        if natural && !self.lake && self.bed < self.water {
            // A wet continuous cross-section can be less than one block deep.
            // Rasterize it consistently in chunks AND far columns, rather than
            // rounding away the final water voxel at a shallow headwater.
            floor.min(self.voxel_water(natural) - 1)
        } else {
            floor
        }
    }
}

#[derive(Debug)]
pub struct Basin {
    pub seed: u64,
    pub coast: Coast,
    pub side: usize,
    pub config: Config,
    pub raw: Vec<f64>,
    pub filled: Vec<f64>,
    pub parent: Vec<Option<usize>>,
    pub discharge: Vec<f64>,
    pub drains_ocean: Vec<bool>,
    pub ocean: Vec<bool>,
    pub active: Vec<bool>,
    pub segments: Vec<Segment>,
    pub input: f64,
    pub output: f64,
    bins: Vec<Vec<usize>>,
    bin_side: usize,
}

#[derive(Clone, Copy, Debug)]
struct Front {
    height: f64,
    index: usize,
}
impl PartialEq for Front {
    fn eq(&self, b: &Self) -> bool {
        self.height == b.height && self.index == b.index
    }
}
impl Eq for Front {}
impl PartialOrd for Front {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Ord for Front {
    fn cmp(&self, b: &Self) -> Ordering {
        b.height
            .total_cmp(&self.height)
            .then_with(|| b.index.cmp(&self.index))
    }
}

fn neighbors(i: usize, n: usize) -> impl Iterator<Item = usize> {
    let x = i % n;
    let z = i / n;
    (-1_i32..=1).flat_map(move |dz| {
        (-1_i32..=1).filter_map(move |dx| {
            let xx = x as i32 + dx;
            let zz = z as i32 + dz;
            ((dx != 0 || dz != 0) && xx >= 0 && zz >= 0 && xx < n as i32 && zz < n as i32)
                .then_some((zz.max(0) as usize) * n + xx.max(0) as usize)
        })
    })
}

fn smooth(x: f64) -> f64 {
    let t = x.clamp(0., 1.);
    t * t * (3. - 2. * t)
}

fn hash(seed: u64, key: Key) -> u64 {
    let mut a = seed
        ^ (key.0 as u64).wrapping_mul(0x9e3779b97f4a7c15)
        ^ (key.1 as u64).wrapping_mul(0xd1b54a32d192ed03);
    a = (a ^ (a >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    a = (a ^ (a >> 27)).wrapping_mul(0x94d049bb133111eb);
    a ^ (a >> 31)
}

fn noise(seed: u64, p: [f64; 2], scale: f64) -> f64 {
    let x = p[0] / scale;
    let z = p[1] / scale;
    let ix = x.floor() as i32;
    let iz = z.floor() as i32;
    let t = smooth(x - x.floor());
    let u = smooth(z - z.floor());
    let at = |dx, dz| {
        (hash(seed, Key(ix + dx, iz + dz)) >> 11) as f64 / ((1_u64 << 53) - 1) as f64 * 2. - 1.
    };
    (at(0, 0) * (1. - t) + at(1, 0) * t) * (1. - u) + (at(0, 1) * (1. - t) + at(1, 1) * t) * u
}

fn bezier(points: [[f64; 2]; 4], t: f64) -> [f64; 2] {
    let u = 1. - t;
    [0, 1].map(|axis| {
        u * u * u * points[0][axis]
            + 3. * u * u * t * points[1][axis]
            + 3. * u * t * t * points[2][axis]
            + t * t * t * points[3][axis]
    })
}

fn nearest(point: [f64; 2], roots: &[Coast], config: Config) -> Option<(Coast, f64, f64)> {
    // Inland receivers extend drainage beyond the coastal catchments. They
    // must not steal an established coast's drainage area or introduce a new
    // dry Voronoi boundary through a river that already reaches the sea.
    let marine_distance = if config.advanced_rivers {
        roots
            .iter()
            .filter(|c| c.inland_level.is_none())
            .map(|c| (point[0] - f64::from(c.x)).hypot(point[1] - f64::from(c.z)))
            .fold(f64::INFINITY, f64::min)
    } else {
        f64::INFINITY
    };
    let marine = marine_distance < f64::from(config.radius);
    let mut best: Option<(Coast, f64)> = None;
    let mut second = f64::INFINITY;
    for &c in roots {
        if config.advanced_rivers && c.inland_level.is_none() != marine {
            continue;
        }
        let d = (point[0] - f64::from(c.x)).hypot(point[1] - f64::from(c.z));
        if best.is_none_or(|(b, old)| d < old || (d == old && c.key < b.key)) {
            if let Some((_, old)) = best {
                second = second.min(old);
            }
            best = Some((c, d));
        } else {
            second = second.min(d);
        }
    }
    best.map(|(c, d)| {
        if config.advanced_rivers && c.inland_level.is_some() {
            // Reuse the existing bank fade across the coastal radius instead
            // of starting an inland channel abruptly at that boundary.
            second = second.min(d + (marine_distance - f64::from(config.radius)).max(0.));
        }
        (c, d, second)
    })
}

impl Basin {
    pub fn compile(coast: Coast, config: Config, terrain: &dyn Terrain, roots: &[Coast]) -> Self {
        Self::compile_cached(coast, config, terrain, roots, None, 0)
    }

    fn compile_cached(
        coast: Coast,
        config: Config,
        terrain: &dyn Terrain,
        roots: &[Coast],
        cache: Option<&DemCache>,
        seed: u64,
    ) -> Self {
        assert!(config.step > 0 && config.radius > 0 && config.radius % config.step == 0);
        let n = (config.radius * 2 / config.step + 1) as usize;
        let count = n * n;
        let mut out = Self {
            seed,
            coast,
            side: n,
            config,
            raw: vec![0.; count],
            filled: vec![0.; count],
            parent: vec![None; count],
            discharge: vec![0.; count],
            drains_ocean: vec![false; count],
            ocean: vec![false; count],
            active: vec![false; count],
            segments: Vec::new(),
            input: 0.,
            output: 0.,
            bins: Vec::new(),
            bin_side: 0,
        };
        let sample = || {
            (0..count)
                .map(|i| {
                    let p = out.point(i);
                    terrain.land(p[0] as i32, p[1] as i32)
                })
                .collect()
        };
        let dem = if let Some(cache) = cache {
            cache.samples(seed, coast, config, sample)
        } else {
            sample()
        };
        let inland_receiver = coast.inland_level.filter(|_| config.advanced_rivers);
        let mut native_lake = vec![false; count];
        for (i, land) in dem.into_iter().enumerate() {
            let p = out.point(i);
            out.raw[i] = land.height;
            out.filled[i] = land.height.max(config.sea);
            out.ocean[i] = land.ocean && land.height <= config.sea;
            if let Some(level) = inland_receiver
                && land.height < level + 1.
                && (p[0] - f64::from(coast.x))
                    .abs()
                    .max((p[1] - f64::from(coast.z)).abs())
                    <= 192.
            {
                native_lake[i] = terrain
                    .inland_water(p[0] as i32, p[1] as i32)
                    .is_some_and(|water| (water - level).abs() < 0.25 && water > land.height);
            }
            if let Some((owner, d, other)) = nearest(p, roots, config) {
                let (edge_fade, radial_fade) = config.fades();
                let curve_margin = if config.natural { 2. } else { 0. };
                // Publish drainage only inside the fully carved core. Its
                // outer bank may feather, but its wet centreline must not fade
                // into solid terrain before it reaches receiving water.
                out.active[i] = owner.key == coast.key
                    && d < f64::from(config.radius) - radial_fade - curve_margin
                    && other - d > config.margin + edge_fade + curve_margin * 2.;
            }
        }
        if let Some(level) = inland_receiver {
            // Only water connected to the selected root is a terminal.
            // Treating every native pond as one prematurely intercepted the
            // coastal runoff, erased rivers and reopened filled lake bottoms.
            let centre = (n / 2) * n + n / 2;
            let mut pending = vec![centre];
            while let Some(i) = pending.pop() {
                if !native_lake[i] {
                    continue;
                }
                native_lake[i] = false;
                out.ocean[i] = true;
                out.filled[i] = level;
                pending.extend(neighbors(i, n).filter(|&j| native_lake[j]));
            }
        }
        let mut queue = BinaryHeap::new();
        let mut seen = vec![false; count];
        let mut order = Vec::new();
        let mut rank = vec![usize::MAX; count];
        for (i, mark) in seen.iter_mut().enumerate() {
            if !out.active[i] {
                continue;
            }
            let edge = i % n == 0
                || i / n == 0
                || i % n == n - 1
                || i / n == n - 1
                || neighbors(i, n).any(|j| !out.active[j]);
            if out.ocean[i] || edge {
                *mark = true;
                out.drains_ocean[i] = out.ocean[i];
                queue.push(Front {
                    height: out.filled[i],
                    index: i,
                });
            }
        }
        while let Some(Front { height, index: i }) = queue.pop() {
            rank[i] = order.len();
            order.push(i);
            for j in neighbors(i, n) {
                if seen[j] || !out.active[j] {
                    continue;
                }
                seen[j] = true;
                out.filled[j] = out.raw[j].max(height);
                out.parent[j] = Some(i);
                out.drains_ocean[j] = out.drains_ocean[i];
                queue.push(Front {
                    height: out.filled[j],
                    index: j,
                });
            }
        }
        // Prefer a real descending slope to a flood traversal's lateral route.
        // Ties on lakes retain the lower-rank parent, giving a strict DAG.
        for &i in &order {
            if out.parent[i].is_none() {
                continue;
            }
            let mut best = 0.;
            for j in neighbors(i, n) {
                if rank[j] >= rank[i] || out.drains_ocean[j] != out.drains_ocean[i] {
                    continue;
                }
                let diagonal = i % n != j % n && i / n != j / n;
                let d = if diagonal {
                    std::f64::consts::SQRT_2
                } else {
                    1.
                };
                let slope = (out.filled[i] - out.filled[j]) / d;
                if slope > best {
                    best = slope;
                    out.parent[i] = Some(j);
                }
            }
        }
        // Two diagonal D8 links may geometrically cross without sharing a
        // vertex. Merge those links at their lower-rank endpoint instead.
        for &i in &order {
            let Some(j) = out.parent[i] else {
                continue;
            };
            if i % n == j % n || i / n == j / n || !out.drains_ocean[i] {
                continue;
            }
            let k = i / n * n + j % n;
            let l = j / n * n + i % n;
            let other = if out.parent[k] == Some(l) {
                Some((k, l))
            } else if out.parent[l] == Some(k) {
                Some((l, k))
            } else {
                None
            };
            if let Some((from, to)) = other
                && out.drains_ocean[from]
            {
                let target = if rank[j] < rank[to] { j } else { to };
                out.parent[i] = Some(target);
                out.parent[from] = Some(target);
            }
        }
        for &i in &order {
            if out.drains_ocean[i] && !out.ocean[i] {
                out.discharge[i] = 1.;
                out.input += 1.;
            }
        }
        for &i in order.iter().rev() {
            if let Some(j) = out.parent[i] {
                out.discharge[j] += out.discharge[i];
            } else if out.drains_ocean[i] {
                out.output += out.discharge[i];
            }
        }
        for &i in &order {
            let Some(j) = out.parent[i] else {
                continue;
            };
            let cells = out.discharge[i] * (f64::from(config.step) / 32.0).powi(2);
            if !out.drains_ocean[i] || out.ocean[i] || cells < config.min_cells {
                continue;
            }
            out.segments.push(Segment {
                from: out.point(i),
                to: out.point(j),
                water: [out.filled[i], out.filled[j]],
                cells,
                half_width: (1.6 + cells.sqrt() * 0.19).clamp(2., 10.),
                depth: (2. + cells.sqrt() * 0.085).clamp(2., 6.),
                mouth: out.ocean[j],
            });
        }
        if config.natural {
            out.naturalize(roots);
            if config.advanced_rivers {
                out.branch_deltas(terrain, roots);
            }
        }
        out.index();
        out
    }

    pub fn point(&self, i: usize) -> [f64; 2] {
        [
            f64::from(
                self.coast.x - self.config.radius + (i % self.side) as i32 * self.config.step,
            ),
            f64::from(
                self.coast.z - self.config.radius + (i / self.side) as i32 * self.config.step,
            ),
        ]
    }

    pub fn raw_at(&self, x: i32, z: i32) -> f64 {
        let origin = self.point(0);
        let gx = ((f64::from(x) - origin[0]) / f64::from(self.config.step))
            .clamp(0., (self.side - 1) as f64);
        let gz = ((f64::from(z) - origin[1]) / f64::from(self.config.step))
            .clamp(0., (self.side - 1) as f64);
        let ix = gx.floor() as usize;
        let iz = gz.floor() as usize;
        let at = |dx: usize, dz: usize| {
            self.raw[(iz + dz).min(self.side - 1) * self.side + (ix + dx).min(self.side - 1)]
        };
        let t = gx - ix as f64;
        let u = gz - iz as f64;
        (at(0, 0) * (1. - t) + at(1, 0) * t) * (1. - u) + (at(0, 1) * (1. - t) + at(1, 1) * t) * u
    }

    fn naturalize(&mut self, roots: &[Coast]) {
        let count = self.raw.len();
        let mut upstream = vec![None; count];
        for (i, parent) in self.parent.iter().enumerate() {
            if let Some(j) = *parent
                && upstream[j].is_none_or(|old| self.discharge[i] > self.discharge[old])
            {
                upstream[j] = Some(i);
            }
        }
        let (edge_fade, radial_fade) = self.config.fades();
        let valid = |p: [f64; 2]| {
            nearest(p, roots, self.config).is_some_and(|(owner, d, other)| {
                owner.key == self.coast.key
                    && d < f64::from(self.config.radius) - radial_fade - 1.
                    && other - d > self.config.margin + edge_fade + 2.
            })
        };
        let positions: Vec<_> = (0..count)
            .map(|i| {
                let p = self.point(i);
                if self.ocean[i] || !self.active[i] {
                    return p;
                }
                let slope = self.parent[i].map_or(0., |j| {
                    let q = self.point(j);
                    (self.filled[i] - self.filled[j]).abs()
                        / (p[0] - q[0]).hypot(p[1] - q[1]).max(1.)
                });
                let mut shore_distance = 0.;
                let mut node = i;
                for _ in 0..4 {
                    if self.ocean[node] {
                        break;
                    }
                    let Some(parent) = self.parent[node] else {
                        shore_distance = 48.;
                        break;
                    };
                    let a = self.point(node);
                    let b = self.point(parent);
                    shore_distance += (a[0] - b[0]).hypot(a[1] - b[1]);
                    node = parent;
                }
                // Flat estuaries still meander. Fade by travel distance to the
                // receiving sea, not by elevation above sea level.
                let slope_limit = if self.config.refined_rivers {
                    0.30
                } else {
                    0.18
                };
                let amplitude = self.config.meander_amplitude
                    * (1. - smooth(slope / slope_limit))
                    * smooth(shore_distance / 48.);
                let mut shift = [
                    amplitude * noise(self.seed, p, self.config.meander_scale),
                    amplitude * noise(self.seed ^ 0xd1b54a32d192ed03, p, self.config.meander_scale),
                ];
                if self.config.advanced_rivers {
                    // Smooth the routing lattice over a whole reach before
                    // applying the small meander field. Every tributary uses
                    // the SAME relocated confluence vertex.
                    let mut sum = [p[0] * 4., p[1] * 4.];
                    let mut weight = 4.;
                    let mut up = upstream[i];
                    let mut down = self.parent[i];
                    for w in [3., 2., 1.] {
                        let (Some(a), Some(b)) = (up, down) else {
                            break;
                        };
                        if self.ocean[a] || self.ocean[b] {
                            break;
                        }
                        let pa = self.point(a);
                        let pb = self.point(b);
                        sum[0] += (pa[0] + pb[0]) * w;
                        sum[1] += (pa[1] + pb[1]) * w;
                        weight += 2. * w;
                        up = upstream[a];
                        down = self.parent[b];
                    }
                    let delta = [sum[0] / weight - p[0], sum[1] / weight - p[1]];
                    let gain = (f64::from(self.config.step) * 0.75
                        / delta[0].hypot(delta[1]).max(0.001))
                    .min(1.)
                        * (1. - smooth(slope / 0.25))
                        * smooth(shore_distance / 48.);
                    shift[0] += delta[0] * gain;
                    shift[1] += delta[1] * gain;
                }
                if self.config.refined_rivers
                    && let Some(j) = self.parent[i]
                {
                    // Move a reach ACROSS its flow. The old two-axis field
                    // also compressed/extended it along the flow: at the v10
                    // amplitude (28) the shift could exceed a DEM edge (16),
                    // producing tight hooks and abrupt changes of water grade.
                    // Shared vertices still keep every tributary connected.
                    let down = self.point(j);
                    let up = upstream[i].map_or([2. * p[0] - down[0], 2. * p[1] - down[1]], |k| {
                        self.point(k)
                    });
                    let direction = [down[0] - up[0], down[1] - up[1]];
                    let length = direction[0].hypot(direction[1]).max(0.001);
                    let across = [-direction[1] / length, direction[0] / length];
                    let limit =
                        f64::from(self.config.step) * 0.9 + self.config.meander_amplitude * 0.25;
                    let lateral =
                        (shift[0] * across[0] + shift[1] * across[1]).clamp(-limit, limit);
                    shift = [across[0] * lateral, across[1] * lateral];
                }
                for scale in [1., 0.5, 0.25] {
                    let candidate = [p[0] + shift[0] * scale, p[1] + shift[1] * scale];
                    if valid(candidate)
                        && (!self.config.advanced_rivers
                            || self
                                .raw_at(candidate[0].round() as i32, candidate[1].round() as i32)
                                <= self.raw[i] + 3.)
                    {
                        return candidate;
                    }
                }
                p
            })
            .collect();
        let origin = self.point(0);
        let node = |p: [f64; 2]| {
            let x = ((p[0] - origin[0]) / f64::from(self.config.step)).round() as usize;
            let z = ((p[1] - origin[1]) / f64::from(self.config.step)).round() as usize;
            z * self.side + x
        };
        let mut curved = Vec::with_capacity(self.segments.len() * 4);
        for s in &self.segments {
            let i = node(s.from);
            let j = node(s.to);
            let a = positions[i];
            let b = positions[j];
            let previous =
                upstream[i].map_or([2. * a[0] - b[0], 2. * a[1] - b[1]], |k| positions[k]);
            let next =
                self.parent[j].map_or([2. * b[0] - a[0], 2. * b[1] - a[1]], |k| positions[k]);
            let length = (b[0] - a[0]).hypot(b[1] - a[1]);
            let handle = |p: [f64; 2], mut v: [f64; 2], sign: f64| {
                if self.config.refined_rivers {
                    // A relocated neighbour can point a cubic handle back
                    // upstream at a sharp turn. Keep handles progressing
                    // along this edge; their cross-stream tangent survives.
                    let direction = [
                        (b[0] - a[0]) / length.max(0.001),
                        (b[1] - a[1]) / length.max(0.001),
                    ];
                    let along = v[0] * direction[0] + v[1] * direction[1];
                    let forward = along.max(v[0].hypot(v[1]) * 0.25);
                    v[0] += direction[0] * (forward - along);
                    v[1] += direction[1] * (forward - along);
                }
                let scale = length / (3. * v[0].hypot(v[1]).max(0.001));
                [p[0] + v[0] * scale * sign, p[1] + v[1] * scale * sign]
            };
            let mut controls = [
                a,
                handle(a, [b[0] - previous[0], b[1] - previous[1]], 1.),
                handle(b, [next[0] - a[0], next[1] - a[1]], -1.),
                b,
            ];
            if (0..=8).any(|step| !valid(bezier(controls, f64::from(step) / 8.))) {
                controls = [
                    a,
                    [a[0] + (b[0] - a[0]) / 3., a[1] + (b[1] - a[1]) / 3.],
                    [
                        a[0] + 2. * (b[0] - a[0]) / 3.,
                        a[1] + 2. * (b[1] - a[1]) / 3.,
                    ],
                    b,
                ];
            }
            let source = upstream[i].is_none_or(|k| {
                self.discharge[k] * (f64::from(self.config.step) / 32.).powi(2)
                    < self.config.min_cells
            });
            for part in 0..4 {
                let t0 = f64::from(part) / 4.;
                let t1 = f64::from(part + 1) / 4.;
                let t = (t0 + t1) * 0.5;
                let middle = bezier(controls, t);
                let taper = if source { smooth(t) } else { 1. };
                curved.push(Segment {
                    from: bezier(controls, t0),
                    to: bezier(controls, t1),
                    water: [
                        s.water[0] * (1. - t0) + s.water[1] * t0,
                        s.water[0] * (1. - t1) + s.water[1] * t1,
                    ],
                    cells: s.cells,
                    half_width: (s.half_width
                        * self.config.width_scale
                        * (1. + 0.18 * noise(self.seed ^ 0x85ebca6b, middle, 72.))
                        * (0.35 + 0.65 * taper))
                        .max(0.85),
                    depth: 1.2 + (s.depth * self.config.depth_scale - 1.2).max(0.) * taper,
                    mouth: s.mouth && part == 3,
                });
            }
        }
        self.segments = curved;
    }

    /// Add a distributary only where a substantial river meets a low, open
    /// marine plain. A cliff coast or an inland lake never gets a forced fork.
    fn branch_deltas(&mut self, terrain: &dyn Terrain, roots: &[Coast]) {
        if self.coast.inland_level.is_some() {
            return;
        }
        let mut mouths: Vec<_> = self
            .segments
            .iter()
            .enumerate()
            .filter(|(_, s)| s.mouth && s.cells >= 48. && s.water[1] <= self.config.sea + 0.25)
            .map(|(i, _)| i)
            .collect();
        mouths.sort_by(|&a, &b| self.segments[b].cells.total_cmp(&self.segments[a].cells));
        let mut accepted: Vec<[f64; 2]> = Vec::new();
        let config = self.config;
        let coast_key = self.coast.key;
        let (edge_fade, radial_fade) = config.fades();
        let valid = |p: [f64; 2]| {
            nearest(p, roots, self.config).is_some_and(|(owner, d, other)| {
                owner.key == coast_key
                    && d < f64::from(config.radius) - radial_fade - 2.
                    && other - d > config.margin + edge_fade + 4.
            })
        };
        for mouth in mouths.into_iter().take(8) {
            if accepted.len() >= 2 {
                break;
            }
            let end = self.segments[mouth].to;
            if accepted
                .iter()
                .any(|p| (p[0] - end[0]).hypot(p[1] - end[1]) < 96.)
            {
                continue;
            }
            let mut chain = vec![mouth];
            let mut length = 0.;
            for _ in 0..64 {
                let current = self.segments[*chain.last().expect("mouth")];
                length += (current.to[0] - current.from[0]).hypot(current.to[1] - current.from[1]);
                if length >= 72. {
                    break;
                }
                let previous = self
                    .segments
                    .iter()
                    .enumerate()
                    .filter(|(i, s)| s.to == current.from && !chain.contains(i))
                    .max_by(|(_, a), (_, b)| a.cells.total_cmp(&b.cells))
                    .map(|(i, _)| i);
                let Some(previous) = previous else {
                    break;
                };
                chain.push(previous);
            }
            let start_segment = self.segments[*chain.last().expect("mouth")];
            if length < 48.
                || start_segment.cells < 48.
                || start_segment.water[0] > self.config.sea + 4.
            {
                continue;
            }
            let start = start_segment.from;
            let direct = (end[0] - start[0]).hypot(end[1] - start[1]);
            if direct < 32. {
                continue;
            }
            let direction = [(end[0] - start[0]) / direct, (end[1] - start[1]) / direct];
            let first_side = if hash(self.seed, Key(end[0] as i32, end[1] as i32)) & 1 == 0 {
                1.
            } else {
                -1.
            };
            let mut chosen = None;
            'side: for side in [first_side, -first_side] {
                let normal = [-direction[1] * side, direction[0] * side];
                for offset in [24., 40., 56.] {
                    let terminal = [
                        end[0] + normal[0] * offset + direction[0] * 16.,
                        end[1] + normal[1] * offset + direction[1] * 16.,
                    ];
                    let receiving =
                        terrain.land(terminal[0].round() as i32, terminal[1].round() as i32);
                    if !receiving.ocean || receiving.height > self.config.sea {
                        continue;
                    }
                    let controls = [
                        start,
                        [
                            start[0] + direction[0] * direct * 0.33 + normal[0] * offset * 0.25,
                            start[1] + direction[1] * direct * 0.33 + normal[1] * offset * 0.25,
                        ],
                        [
                            terminal[0] - direction[0] * direct * 0.25,
                            terminal[1] - direction[1] * direct * 0.25,
                        ],
                        terminal,
                    ];
                    let admissible = (0..=24).all(|step| {
                        let t = f64::from(step) / 24.;
                        let p = bezier(controls, t);
                        let water = start_segment.water[0] * (1. - t) + self.config.sea * t;
                        if !valid(p)
                            || self.raw_at(p[0].round() as i32, p[1].round() as i32) > water + 2.5
                        {
                            return false;
                        }
                        // A narrow rock bank can fall between coarse DEM
                        // samples. Check the native terrain across the full
                        // wet width before approving the distributary.
                        let before = bezier(controls, (t - 0.01).max(0.));
                        let after = bezier(controls, (t + 0.01).min(1.));
                        let tangent = [after[0] - before[0], after[1] - before[1]];
                        let length = tangent[0].hypot(tangent[1]).max(0.001);
                        let margin =
                            (start_segment.half_width * 0.6 * (1. + 0.15 * t)).max(1.2) + 3.;
                        if [-margin, 0., margin].into_iter().any(|offset| {
                            let sample = [
                                p[0] - tangent[1] / length * offset,
                                p[1] + tangent[0] / length * offset,
                            ];
                            !valid(sample)
                                || terrain
                                    .land(sample[0].round() as i32, sample[1].round() as i32)
                                    .height
                                    > water + 5.
                        }) {
                            return false;
                        }
                        if step <= 5 || step >= 22 {
                            return true;
                        }
                        !self.segments.iter().enumerate().any(|(i, s)| {
                            if chain.contains(&i) {
                                return false;
                            }
                            let v = [s.to[0] - s.from[0], s.to[1] - s.from[1]];
                            let length2 = v[0] * v[0] + v[1] * v[1];
                            let u = (((p[0] - s.from[0]) * v[0] + (p[1] - s.from[1]) * v[1])
                                / length2.max(0.001))
                            .clamp(0., 1.);
                            (p[0] - s.from[0] - v[0] * u).hypot(p[1] - s.from[1] - v[1] * u)
                                < s.half_width + 2.
                        })
                    });
                    if admissible {
                        chosen = Some(controls);
                        break 'side;
                    }
                }
            }
            let Some(controls) = chosen else {
                continue;
            };
            let diverted = start_segment.cells * 0.35;
            for i in chain {
                let s = &mut self.segments[i];
                let old = s.cells;
                s.cells -= diverted;
                s.half_width = (s.half_width * (s.cells / old).sqrt()).max(1.);
                s.depth = (s.depth * (s.cells / old).powf(0.15)).max(1.2);
            }
            for part in 0..16 {
                let a = f64::from(part) / 16.;
                let b = f64::from(part + 1) / 16.;
                self.segments.push(Segment {
                    from: bezier(controls, a),
                    to: bezier(controls, b),
                    water: [
                        start_segment.water[0] * (1. - a) + self.config.sea * a,
                        start_segment.water[0] * (1. - b) + self.config.sea * b,
                    ],
                    cells: diverted,
                    half_width: (start_segment.half_width * 0.6 * (1. + 0.15 * b)).max(1.2),
                    depth: (start_segment.depth * 0.7).clamp(1.5, 3.),
                    mouth: part == 15,
                });
            }
            accepted.push(end);
        }
    }

    fn index(&mut self) {
        self.bin_side = (self.config.radius * 2 / 64 + 1) as usize;
        self.bins = vec![Vec::new(); self.bin_side * self.bin_side];
        for (id, s) in self.segments.iter().enumerate() {
            let pad = if self.config.natural {
                s.half_width * 1.25 * self.config.bank_scale
                    + 14.
                    + if self.config.refined_rivers { 24. } else { 0. }
            } else {
                s.half_width * 2. + 8.
            };
            let lo = [s.from[0].min(s.to[0]) - pad, s.from[1].min(s.to[1]) - pad];
            let hi = [s.from[0].max(s.to[0]) + pad, s.from[1].max(s.to[1]) + pad];
            let [lx, lz] = self.bin(lo);
            let [hx, hz] = self.bin(hi);
            for z in lz..=hz {
                for x in lx..=hx {
                    self.bins[z * self.bin_side + x].push(id);
                }
            }
        }
    }

    fn bin(&self, p: [f64; 2]) -> [usize; 2] {
        let origin = self.point(0);
        [0, 1].map(|i| {
            (((p[i] - origin[i]) / 64.).floor() as i64).clamp(0, self.bin_side as i64 - 1) as usize
        })
    }

    pub fn sample(&self, x: i32, z: i32, raw: f64) -> Option<Profile> {
        let p = [f64::from(x), f64::from(z)];
        let [bx, bz] = self.bin(p);
        let mut result: Option<Profile> = None;
        let mut core_distance = f64::INFINITY;
        for &id in &self.bins[bz * self.bin_side + bx] {
            let s = self.segments[id];
            let v = [s.to[0] - s.from[0], s.to[1] - s.from[1]];
            let length = v[0] * v[0] + v[1] * v[1];
            let t =
                (((p[0] - s.from[0]) * v[0] + (p[1] - s.from[1]) * v[1]) / length).clamp(0., 1.);
            let d = (p[0] - s.from[0] - v[0] * t).hypot(p[1] - s.from[1] - v[1] * t);
            let signed =
                ((p[0] - s.from[0]) * v[1] - (p[1] - s.from[1]) * v[0]) / length.sqrt().max(0.001);
            let width = if self.config.natural {
                s.half_width * (1. + 0.18 * noise(self.seed ^ 0x9e3779b9, p, 48.) * signed.signum())
            } else {
                s.half_width
            };
            let w = s.water[0] * (1. - t) + s.water[1] * t;
            let outer = if self.config.natural {
                let original = width * self.config.bank_scale + 10.;
                if self.config.refined_rivers {
                    // A width-only shoulder squeezed a high native bank into
                    // the same narrow strip as a flat floodplain. Give high
                    // banks room to rise, bounded to 24 extra blocks so this
                    // never becomes a blanket mountain/shore flattening pass.
                    original
                        .max(width + (raw - w).max(0.) * 2.)
                        .min(original + 24.)
                } else {
                    original
                }
            } else {
                width * 2. + 8.
            };
            if d >= outer {
                continue;
            }
            let strength = 1. - smooth((d - width) / (outer - width));
            let target = if self.config.natural {
                let bed = if d <= width {
                    w - s.depth * (1. - (d / width).powi(2)).max(0.).powf(1.2)
                } else {
                    w + (raw - w).max(0.) * smooth((d - width) / (outer - width))
                };
                bed.min(raw)
            } else {
                let bed = w - s.depth * (1. - 0.3 * (d / width).min(1.).powi(2));
                raw + (bed.min(raw) - raw) * strength
            };
            let core = d <= width;
            let choose = if core {
                d < core_distance
                    || (d == core_distance && result.is_none_or(|old| target < old.bed))
            } else {
                core_distance.is_infinite() && result.is_none_or(|old| target < old.bed)
            };
            if choose {
                if core {
                    core_distance = d;
                }
                result = Some(Profile {
                    bed: target,
                    water: w,
                    strength,
                    raw,
                    lake: false,
                    channel_core: core,
                });
            }
        }
        // Actual filled depressions connected to a receiving ocean: level lakes,
        // not independent random puddles. Bilinear dry shores use the raw terrain.
        let origin = self.point(0);
        let gx = (p[0] - origin[0]) / f64::from(self.config.step);
        let gz = (p[1] - origin[1]) / f64::from(self.config.step);
        let level = self.lake_level(gx, gz);
        if let Some(water) = level
            && raw < water - 0.25
        {
            let lake = Profile {
                bed: raw,
                water,
                strength: 1.,
                raw,
                lake: true,
                channel_core: self.config.natural && core_distance.is_finite(),
            };
            if self.config.natural {
                // The same lake level covers a crossing channel and its banks.
                // Retaining a river profile here would reset only its core to
                // the original native water level and punch a hole in the lake.
                result = Some(lake);
            } else if core_distance.is_finite() {
                if let Some(channel) = &mut result {
                    channel.water = channel.water.max(lake.water);
                }
            } else if result.is_none_or(|old| lake.water > old.water) {
                result = Some(lake);
            }
        }
        result
    }

    fn lake_level(&self, gx: f64, gz: f64) -> Option<f64> {
        if !self.config.natural {
            let x = gx.round() as i32;
            let z = gz.round() as i32;
            if x < 0 || z < 0 || x >= self.side as i32 || z >= self.side as i32 {
                return None;
            }
            let i = z as usize * self.side + x as usize;
            return (self.active[i]
                && self.drains_ocean[i]
                && !self.ocean[i]
                && self.filled[i] > self.raw[i] + 1.5)
                .then_some(self.filled[i]);
        }
        let x = gx.floor() as i32;
        let z = gz.floor() as i32;
        if x < 0 || z < 0 || x + 1 >= self.side as i32 || z + 1 >= self.side as i32 {
            return None;
        }
        let i = z as usize * self.side + x as usize;
        let corners = [i, i + 1, i + self.side, i + self.side + 1];
        // A nearest dry raster cell must not flood the ocean half of its tile.
        // All four corners must belong to this inland catchment. High dry
        // banks need not share the water level of the depressed corners.
        if !corners
            .iter()
            .all(|&j| self.active[j] && self.drains_ocean[j] && !self.ocean[j])
        {
            return None;
        }
        let depressed = || {
            corners
                .iter()
                .copied()
                .filter(|&j| self.filled[j] > self.raw[j] + 1.5)
        };
        let lo = depressed()
            .map(|j| self.filled[j])
            .fold(f64::INFINITY, f64::min);
        let hi = depressed()
            .map(|j| self.filled[j])
            .fold(f64::NEG_INFINITY, f64::max);
        // Different spill levels are separate ponds, not a stair of square lakes.
        (lo.is_finite() && hi - lo < 0.25 && lo > self.config.sea).then_some(lo)
    }
}

type Slot<T> = Arc<OnceLock<T>>;
type RootSlots = HashMap<Key, Slot<Option<Coast>>>;
type BasinSlots = HashMap<Key, (u64, Slot<Arc<Basin>>)>;
type QuerySlots = HashMap<Key, (u64, Slot<Arc<Vec<Coast>>>)>;

#[derive(Debug)]
pub struct World<T: Terrain> {
    pub terrain: T,
    pub seed: u64,
    pub config: Config,
    pub cache: Option<DemCache>,
    roots: Mutex<RootSlots>,
    basins: Mutex<(u64, BasinSlots)>,
    queries: Mutex<(u64, QuerySlots)>,
}

impl<T: Terrain> World<T> {
    pub fn new(terrain: T, seed: u64, config: Config) -> Self {
        Self {
            terrain,
            seed,
            config,
            cache: None,
            roots: Mutex::new(HashMap::new()),
            basins: Mutex::new((0, HashMap::new())),
            queries: Mutex::new((0, HashMap::new())),
        }
    }

    pub fn with_dem_cache(mut self, directory: PathBuf, identity: u64) -> Self {
        self.cache = Some(DemCache::new(directory, identity));
        self
    }

    pub fn coast(&self, key: Key) -> Option<Coast> {
        let slot = {
            let mut roots = self.roots.lock().unwrap_or_else(|e| e.into_inner());
            // Root records are tiny. The immutable candidate can be recomputed.
            if roots.len() >= 512 && !roots.contains_key(&key) {
                let victim = roots
                    .iter()
                    .find(|(_, slot)| Arc::strong_count(slot) == 1 && slot.get().is_some())
                    .map(|(key, _)| *key);
                if let Some(key) = victim {
                    roots.remove(&key);
                }
            }
            roots
                .entry(key)
                .or_insert_with(|| Arc::new(OnceLock::new()))
                .clone()
        };
        *slot.get_or_init(|| self.find_coast(key))
    }

    fn find_coast(&self, key: Key) -> Option<Coast> {
        let size = self.config.root_cell;
        let stride = size / 4;
        let mut best: Option<(u64, Coast)> = None;
        for z in 0..=4 {
            for x in 0..=4 {
                let p = [key.0 * size + x * stride, key.1 * size + z * stride];
                let a = self.terrain.land(p[0], p[1]);
                for axis in 0..2 {
                    if (axis == 0 && x == 4) || (axis == 1 && z == 4) {
                        continue;
                    }
                    let mut q = p;
                    q[axis] += stride;
                    let b = self.terrain.land(q[0], q[1]);
                    let (mut wet, mut dry) = if a.ocean && b.height > self.config.sea + 2. {
                        (p, q)
                    } else if b.ocean && a.height > self.config.sea + 2. {
                        (q, p)
                    } else {
                        continue;
                    };
                    for _ in 0..5 {
                        let mid = [
                            (wet[0] + dry[0]).div_euclid(2),
                            (wet[1] + dry[1]).div_euclid(2),
                        ];
                        if self.terrain.land(mid[0], mid[1]).ocean {
                            wet = mid;
                        } else {
                            dry = mid;
                        }
                    }
                    let candidate = Coast {
                        inland_level: None,
                        key,
                        x: wet[0],
                        z: wet[1],
                    };
                    // A coastline exactly on a search-cell edge must have one
                    // canonical owner, not two coincident Voronoi seeds.
                    if wet[0].div_euclid(size) != key.0 || wet[1].div_euclid(size) != key.1 {
                        continue;
                    }
                    let score = hash(self.seed, Key(wet[0], wet[1]));
                    if best.is_none_or(|(old, _)| score < old) {
                        best = Some((score, candidate));
                    }
                }
            }
        }
        best.map(|(_, c)| c).or_else(|| {
            self.config
                .advanced_rivers
                .then(|| self.find_inland_coast(key))
                .flatten()
        })
    }

    /// Prefer native lakes to fabricated water at a tile edge. Only a handful
    /// of local low points need the expensive native surface resolver.
    fn find_inland_coast(&self, key: Key) -> Option<Coast> {
        let size = self.config.root_cell;
        let stride = (size / 8).max(self.config.step);
        let mut points = Vec::new();
        for z in 0..=8 {
            for x in 0..=8 {
                let p = [key.0 * size + x * stride, key.1 * size + z * stride];
                points.push((p, self.terrain.land(p[0], p[1])));
            }
        }
        let mut lows: Vec<_> = points
            .iter()
            .enumerate()
            .filter(|(i, (_, land))| {
                !land.ocean && neighbors(*i, 9).all(|j| land.height <= points[j].1.height + 1.)
            })
            .collect();
        lows.sort_by_key(|(_, (p, _))| hash(self.seed, Key(p[0], p[1])));
        let mut best = None;
        for (_, &(p, land)) in lows.into_iter().take(12) {
            let Some(level) = self.terrain.inland_water(p[0], p[1]) else {
                continue;
            };
            if !level.is_finite() || level <= land.height || level < self.config.sea {
                continue;
            }
            // A water block or tiny aquifer opening is not a drainage terminal.
            let wet_neighbors = [[16, 0], [-16, 0], [0, 16], [0, -16]]
                .into_iter()
                .filter(|d| {
                    self.terrain
                        .inland_water(p[0] + d[0], p[1] + d[1])
                        .is_some_and(|w| (w - level).abs() < 0.25)
                })
                .count();
            if wet_neighbors < 2 {
                continue;
            }
            for direction in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
                let q = (1..=4)
                    .map(|distance| {
                        [
                            p[0] + direction[0] * stride * distance,
                            p[1] + direction[1] * stride * distance,
                        ]
                    })
                    .find(|q| self.terrain.inland_water(q[0], q[1]).is_none());
                let Some(q) = q else {
                    continue;
                };
                let shore = self.terrain.land(q[0], q[1]);
                if shore.height < level - 1. {
                    continue;
                }
                let mut wet = p;
                let mut dry = q;
                for _ in 0..5 {
                    let mid = [
                        (wet[0] + dry[0]).div_euclid(2),
                        (wet[1] + dry[1]).div_euclid(2),
                    ];
                    if self
                        .terrain
                        .inland_water(mid[0], mid[1])
                        .is_some_and(|w| (w - level).abs() < 0.25)
                    {
                        wet = mid;
                    } else {
                        dry = mid;
                    }
                }
                if wet[0].div_euclid(size) != key.0 || wet[1].div_euclid(size) != key.1 {
                    continue;
                }
                let score = hash(self.seed, Key(wet[0], wet[1]));
                if best.is_none_or(|(old, _)| score < old) {
                    best = Some((
                        score,
                        Coast {
                            inland_level: Some(level),
                            key,
                            x: wet[0],
                            z: wet[1],
                        },
                    ));
                }
            }
        }
        best.map(|(_, coast)| coast)
    }

    fn roots_around(&self, x: i32, z: i32, radius: i32) -> Vec<Coast> {
        let size = self.config.root_cell;
        let mut roots = Vec::new();
        for zz in (z - radius).div_euclid(size)..=(z + radius).div_euclid(size) {
            for xx in (x - radius).div_euclid(size)..=(x + radius).div_euclid(size) {
                if let Some(c) = self.coast(Key(xx, zz)) {
                    roots.push(c);
                }
            }
        }
        roots
    }

    fn query_roots(&self, x: i32, z: i32) -> Arc<Vec<Coast>> {
        let size = self.config.root_cell;
        let key = Key(x.div_euclid(size), z.div_euclid(size));
        let slot = {
            let mut queries = self.queries.lock().unwrap_or_else(|e| e.into_inner());
            queries.0 += 1;
            let tick = queries.0;
            if queries.1.len() >= 64 && !queries.1.contains_key(&key) {
                let victim = queries
                    .1
                    .iter()
                    .filter(|(_, (_, slot))| Arc::strong_count(slot) == 1 && slot.get().is_some())
                    .min_by_key(|(_, (age, _))| *age)
                    .map(|(key, _)| *key);
                if let Some(victim) = victim {
                    queries.1.remove(&victim);
                }
            }
            let entry = queries
                .1
                .entry(key)
                .or_insert_with(|| (tick, Arc::new(OnceLock::new())));
            entry.0 = tick;
            entry.1.clone()
        };
        slot.get_or_init(|| {
            let center = [key.0 * size + size / 2, key.1 * size + size / 2];
            Arc::new(self.roots_around(center[0], center[1], self.config.radius + size * 2))
        })
        .clone()
    }

    pub fn basin(&self, coast: Coast) -> Arc<Basin> {
        let slot = {
            let mut cache = self.basins.lock().unwrap_or_else(|e| e.into_inner());
            cache.0 += 1;
            let tick = cache.0;
            if !cache.1.contains_key(&coast.key) && cache.1.len() >= self.config.basin_cache {
                let victim = cache
                    .1
                    .iter()
                    .filter(|(_, (_, slot))| Arc::strong_count(slot) == 1 && slot.get().is_some())
                    .min_by_key(|(_, (age, _))| *age)
                    .map(|(key, _)| *key);
                if let Some(key) = victim {
                    cache.1.remove(&key);
                }
            }
            let entry = cache
                .1
                .entry(coast.key)
                .or_insert_with(|| (tick, Arc::new(OnceLock::new())));
            entry.0 = tick;
            entry.1.clone()
        };
        slot.get_or_init(|| {
            let roots =
                self.roots_around(coast.x, coast.z, self.config.radius * 2 + self.config.step);
            Arc::new(Basin::compile_cached(
                coast,
                self.config,
                &self.terrain,
                &roots,
                self.cache.as_ref(),
                self.seed,
            ))
        })
        .clone()
    }

    pub fn sample(&self, x: i32, z: i32) -> Option<Profile> {
        let roots = self.query_roots(x, z);
        let (coast, distance, second) = nearest([f64::from(x), f64::from(z)], &roots, self.config)?;
        if distance >= f64::from(self.config.radius) || second - distance < self.config.margin {
            return None;
        }
        let basin = self.basin(coast);
        let mut profile = basin.sample(x, z, basin.raw_at(x, z))?;
        let (edge_fade, radial_fade) = self.config.fades();
        let border = smooth((second - distance - self.config.margin) / edge_fade)
            * smooth((f64::from(self.config.radius) - distance) / radial_fade);
        profile.bed = profile.raw + (profile.bed - profile.raw) * border;
        profile.strength *= border;
        (profile.strength > 0.0001).then_some(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug)]
    struct CoastHill;
    impl Terrain for CoastHill {
        fn land(&self, x: i32, z: i32) -> Land {
            let height = 63. + f64::from(x) * 0.12 + (f64::from(z) / 70.).sin() * 3.;
            Land {
                height,
                ocean: height <= 63.,
            }
        }
    }
    fn config() -> Config {
        Config {
            step: 16,
            radius: 256,
            root_cell: 192,
            margin: 8.,
            min_cells: 3.,
            ..Config::default()
        }
    }

    #[derive(Debug)]
    struct HighlandLake;
    impl Terrain for HighlandLake {
        fn land(&self, x: i32, z: i32) -> Land {
            let radius = f64::from(x - 96).hypot(f64::from(z - 96));
            Land {
                height: 90. + radius * 0.12,
                ocean: false,
            }
        }
        fn inland_water(&self, x: i32, z: i32) -> Option<f64> {
            (self.land(x, z).height < 96.).then_some(96.)
        }
    }

    #[test]
    fn inland_catchment_ends_in_real_highland_water_not_sea_or_tile_edge() {
        let world = World::new(
            HighlandLake,
            42,
            Config {
                advanced_rivers: true,
                ..config()
            },
        );
        let coast = world
            .coast(Key(0, 0))
            .expect("existing highland lake shoreline");
        assert_eq!(HighlandLake.inland_water(coast.x, coast.z), Some(96.));
        let basin = world.basin(coast);
        assert!(
            basin.segments.len() > 12,
            "lake must receive actual tributaries"
        );
        assert!(
            basin
                .segments
                .iter()
                .all(|s| s.water[0] >= s.water[1] && s.water[1] >= 96.)
        );
        let mouths: Vec<_> = basin.segments.iter().filter(|s| s.mouth).collect();
        assert!(!mouths.is_empty());
        for mouth in mouths {
            assert_eq!(mouth.water[1], 96.);
            assert_eq!(
                HighlandLake.inland_water(mouth.to[0] as i32, mouth.to[1] as i32),
                Some(96.)
            );
        }
        assert!((basin.input - basin.output).abs() < 1e-8);
        assert!(
            World::new(HighlandLake, 42, config())
                .coast(Key(0, 0))
                .is_none(),
            "saved coastal recipe must remain unchanged"
        );
    }

    #[test]
    fn isolated_inland_water_block_is_not_a_catchment_outlet() {
        #[derive(Debug)]
        struct Puddle;
        impl Terrain for Puddle {
            fn land(&self, x: i32, z: i32) -> Land {
                HighlandLake.land(x, z)
            }
            fn inland_water(&self, x: i32, z: i32) -> Option<f64> {
                (x == 96 && z == 96).then_some(96.)
            }
        }
        let world = World::new(
            Puddle,
            42,
            Config {
                advanced_rivers: true,
                ..config()
            },
        );
        assert!(world.coast(Key(0, 0)).is_none());
    }

    #[test]
    fn inland_roots_extend_but_do_not_fragment_existing_coastal_drainage() {
        let config = Config {
            advanced_rivers: true,
            radius: 512,
            ..config()
        };
        let coast = Coast {
            key: Key(0, 0),
            x: -4,
            z: 0,
            inland_level: None,
        };
        let lake = Coast {
            key: Key(2, 0),
            x: 384,
            z: 0,
            inland_level: Some(96.),
        };
        let original = Basin::compile(coast, config, &CoastHill, &[coast]);
        let extended = Basin::compile(coast, config, &CoastHill, &[coast, lake]);
        assert!(!original.segments.is_empty());
        assert_eq!(
            original.active, extended.active,
            "a nearby lake must not cut a new dry border through coastal rivers"
        );
        assert_eq!(original.parent, extended.parent);
        assert_eq!(original.drains_ocean, extended.drains_ocean);
        assert_eq!(original.discharge, extended.discharge);
        assert_eq!(original.segments.len(), extended.segments.len());
        let (owner, distance, second) = nearest([600., 0.], &[coast, lake], config).unwrap();
        assert_eq!(
            owner.key, lake.key,
            "inland drainage remains available beyond the coast's range"
        );
        assert!(second - distance > config.margin);
    }

    #[test]
    fn native_pond_inside_marine_catchment_is_not_a_premature_terminal() {
        #[derive(Debug)]
        struct Pond(bool);
        impl Terrain for Pond {
            fn land(&self, x: i32, z: i32) -> Land {
                let mut land = CoastHill.land(x, z);
                if f64::from(x - 128).hypot(f64::from(z - 64)) < 24. {
                    land.height = 58.;
                    land.ocean = false;
                }
                land
            }
            fn inland_water(&self, x: i32, z: i32) -> Option<f64> {
                (self.0 && f64::from(x - 128).hypot(f64::from(z - 64)) < 24.).then_some(63.)
            }
        }
        let config = Config {
            advanced_rivers: true,
            ..config()
        };
        let coast = Coast {
            key: Key(0, 0),
            x: 0,
            z: 0,
            inland_level: None,
        };
        let unfilled = Basin::compile(coast, config, &Pond(false), &[coast]);
        let native_water = Basin::compile(coast, config, &Pond(true), &[coast]);
        assert_eq!(
            unfilled.ocean, native_water.ocean,
            "a pre-existing inland puddle must still drain through the basin spill"
        );
        assert_eq!(unfilled.filled, native_water.filled);
        assert_eq!(unfilled.discharge, native_water.discharge);
        assert_eq!(unfilled.segments.len(), native_water.segments.len());
        assert!(native_water.segments.iter().any(|s| s.mouth));
        assert!((native_water.input - native_water.output).abs() < 1e-8);
    }

    #[test]
    fn low_coast_delta_splits_flow_but_does_not_cut_a_cliff() {
        #[derive(Debug)]
        struct LowCoast;
        impl Terrain for LowCoast {
            fn land(&self, x: i32, _z: i32) -> Land {
                Land {
                    height: 63. + f64::from(x) * 0.02,
                    ocean: x <= 0,
                }
            }
        }
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let mut basin = Basin::compile(coast, config(), &LowCoast, &[coast]);
        basin.segments = (0..32)
            .map(|i| {
                let x = 96. - f64::from(i) * 3.;
                Segment {
                    from: [x, 0.],
                    to: [x - 3., 0.],
                    water: [63. + x * 0.02, 63. + (x - 3.) * 0.02],
                    cells: 120.,
                    half_width: 6.,
                    depth: 3.,
                    mouth: i == 31,
                }
            })
            .collect();
        let original = basin.segments.clone();
        basin.branch_deltas(&LowCoast, &[coast]);
        assert_eq!(
            basin.segments.iter().filter(|s| s.mouth).count(),
            2,
            "low receiving marine plain should permit a distributary"
        );
        let branch = basin.segments[original.len()];
        let trunk = basin.segments[..original.len()]
            .iter()
            .find(|s| s.from == branch.from)
            .expect("shared bifurcation");
        assert!(
            (trunk.cells + branch.cells - 120.).abs() < 1e-8,
            "fork must split, not duplicate flow"
        );
        assert!(basin.segments.iter().all(|s| s.water[0] >= s.water[1]));
        for mouth in basin.segments.iter().filter(|s| s.mouth) {
            assert!(
                LowCoast
                    .land(mouth.to[0].round() as i32, mouth.to[1].round() as i32)
                    .ocean
            );
            assert_eq!(mouth.water[1], 63.);
        }
        basin.segments = original.clone();
        for i in 0..basin.raw.len() {
            if basin.point(i)[0] > 0. {
                basin.raw[i] += 20.;
            }
        }
        basin.branch_deltas(&LowCoast, &[coast]);
        assert_eq!(
            basin.segments.len(),
            original.len(),
            "do not fabricate a delta through a cliff"
        );
        // The coarse lattice can miss a narrow ridge beside the centreline.
        // Native cross-section probes must still reject that route.
        #[derive(Debug)]
        struct FineBanks;
        impl Terrain for FineBanks {
            fn land(&self, x: i32, z: i32) -> Land {
                let mut land = LowCoast.land(x, z);
                if (20..90).contains(&x) && (3..=10).contains(&z.abs()) {
                    land.height += 20.;
                }
                land
            }
        }
        let mut fine = Basin::compile(coast, config(), &LowCoast, &[coast]);
        fine.segments = original.clone();
        fine.branch_deltas(&FineBanks, &[coast]);
        assert_eq!(
            fine.segments.len(),
            original.len(),
            "native narrow banks veto the coarse-DEM route"
        );
    }

    #[test]
    fn natural_channels_bend_and_keep_their_confluence_vertices() {
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: -4,
            z: 0,
        };
        let mut basin = Basin::compile(
            coast,
            Config {
                radius: 512,
                natural: false,
                ..config()
            },
            &CoastHill,
            &[coast],
        );
        let original = basin.segments.clone();
        assert!(original.len() > 10);
        basin.config.natural = true;
        basin.naturalize(&[coast]);
        assert_eq!(basin.segments.len(), original.len() * 4);
        let mut bent = 0;
        for (old, parts) in original
            .iter()
            .zip(basin.segments.as_chunks::<4>().0.iter())
        {
            for pair in parts.windows(2) {
                assert_eq!(pair[0].to, pair[1].from);
                assert_eq!(pair[0].water[1], pair[1].water[0]);
            }
            let a = parts[0].from;
            let b = parts[3].to;
            let p = parts[1].to;
            if ((b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])).abs() > 0.1 {
                bent += 1;
            }
            for (other, other_parts) in original
                .iter()
                .zip(basin.segments.as_chunks::<4>().0.iter())
            {
                if old.to == other.from {
                    assert_eq!(
                        parts[3].to, other_parts[0].from,
                        "tributary disconnected at a smoothed junction"
                    );
                }
            }
        }
        assert!(
            bent > 5,
            "a natural river must not remain a chain of straight D8 cuts"
        );
    }

    #[test]
    fn natural_headwaters_start_shallow_instead_of_a_full_depth_pit() {
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: -4,
            z: 0,
        };
        let mut basin = Basin::compile(
            coast,
            Config {
                radius: 512,
                natural: false,
                ..config()
            },
            &CoastHill,
            &[coast],
        );
        let original = basin.segments.clone();
        basin.config.natural = true;
        basin.naturalize(&[coast]);
        let mut sources = 0;
        for (old, parts) in original
            .iter()
            .zip(basin.segments.as_chunks::<4>().0.iter())
        {
            if !original.iter().any(|s| s.to == old.from) {
                assert!(parts[0].depth < 1.5 && parts[3].depth > parts[0].depth);
                assert!(parts[3].half_width > parts[0].half_width);
                sources += 1;
            }
        }
        assert!(sources > 0);
    }

    #[test]
    fn flat_estuaries_meander_even_when_their_water_is_at_sea_level() {
        #[derive(Debug)]
        struct FlatCoast;
        impl Terrain for FlatCoast {
            fn land(&self, x: i32, _: i32) -> Land {
                Land {
                    height: if x < 0 { 61. } else { 63. },
                    ocean: x < 0,
                }
            }
        }
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: -4,
            z: 0,
        };
        let mut basin = Basin::compile(
            coast,
            Config {
                radius: 512,
                natural: false,
                ..config()
            },
            &FlatCoast,
            &[coast],
        );
        let original = basin.segments.clone();
        assert!(original.len() > 10);
        basin.config.natural = true;
        basin.naturalize(&[coast]);
        assert!(
            original
                .iter()
                .zip(basin.segments.as_chunks::<4>().0.iter())
                .any(|(old, parts)| old.water[0] == 63.
                    && (old.from[0] - parts[0].from[0]).hypot(old.from[1] - parts[0].from[1]) > 1.),
            "sea-level rivers were left on the straight DEM lattice"
        );
    }

    fn straight_reach(refined: bool) -> Basin {
        #[derive(Debug)]
        struct Grade;
        impl Terrain for Grade {
            fn land(&self, x: i32, _: i32) -> Land {
                Land {
                    height: 63. + f64::from(x) * 0.12,
                    ocean: x <= 0,
                }
            }
        }
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let mut b = Basin::compile(
            coast,
            Config {
                natural: false,
                advanced_rivers: true,
                refined_rivers: refined,
                meander_amplitude: 28.,
                meander_scale: 80.,
                ..config()
            },
            &Grade,
            &[coast],
        );
        b.parent.fill(None);
        b.discharge.fill(0.);
        b.segments.clear();
        let centre = b.side / 2 * b.side + b.side / 2;
        for n in 1..=11 {
            let i = centre + n;
            let j = i - 1;
            b.parent[i] = Some(j);
            b.discharge[i] = 256.;
            b.segments.push(Segment {
                from: b.point(i),
                to: b.point(j),
                water: [b.raw[i], b.raw[j]],
                cells: 64.,
                half_width: 3.,
                depth: 3.,
                mouth: n == 1,
            });
        }
        b.config.natural = true;
        b.naturalize(&[coast]);
        b
    }

    #[test]
    fn refined_moderate_grade_meanders_without_compressing_downstream_stations() {
        let old = straight_reach(false);
        let new = straight_reach(true);
        let excursion = |b: &Basin| {
            b.segments
                .iter()
                .map(|s| s.from[1].abs())
                .fold(0., f64::max)
        };
        assert!(excursion(&new) > excursion(&old) * 1.3);
        assert!(
            excursion(&new) > 3.,
            "a gentle valley still needs visible lateral bends"
        );
        for (n, parts) in new.segments.as_chunks::<4>().0.iter().enumerate() {
            assert_eq!(
                parts[0].from[0],
                ((n + 1) * 16) as f64,
                "the meander must not move a straight reach's station up/downstream"
            );
            assert_eq!(parts[3].to[0], (n * 16) as f64);
            for (a, b) in parts.iter().zip(&old.segments[n * 4..n * 4 + 4]) {
                assert_eq!(
                    a.water, b.water,
                    "shape tuning cannot change the longitudinal grade"
                );
                assert!(a.water[0] >= a.water[1]);
            }
            if n > 0 {
                assert_eq!(parts[3].to, new.segments[n * 4 - 4].from);
            }
        }
    }

    #[test]
    fn refined_curves_keep_confluences_and_never_hook_back_along_their_edge() {
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: -4,
            z: 0,
        };
        for seed in 0..8 {
            let mut b = Basin::compile_cached(
                coast,
                Config {
                    radius: 512,
                    natural: false,
                    advanced_rivers: true,
                    refined_rivers: true,
                    meander_amplitude: 32.,
                    meander_scale: 48.,
                    ..config()
                },
                &CoastHill,
                &[coast],
                None,
                seed,
            );
            let original = b.segments.clone();
            assert!(original.len() > 10, "confluence fixture has no channels");
            b.config.natural = true;
            b.naturalize(&[coast]);
            let mut junctions = HashMap::new();
            for (s, parts) in original.iter().zip(b.segments.as_chunks::<4>().0.iter()) {
                for (old, new) in [(s.from, parts[0].from), (s.to, parts[3].to)] {
                    let key = (old[0] as i32, old[1] as i32);
                    if let Some(previous) = junctions.insert(key, new) {
                        assert_eq!(previous, new, "separate arms moved their shared confluence");
                    }
                }
                let direction = [
                    parts[3].to[0] - parts[0].from[0],
                    parts[3].to[1] - parts[0].from[1],
                ];
                for part in parts {
                    let progress = (part.to[0] - part.from[0]) * direction[0]
                        + (part.to[1] - part.from[1]) * direction[1];
                    assert!(
                        progress >= -1e-8,
                        "seed {seed}: cubic makes an upstream hook"
                    );
                    assert!(part.water[0] >= part.water[1]);
                }
            }
        }
    }

    #[test]
    fn refined_channels_do_not_add_crossings_in_the_seeded_confluence_fixture() {
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: -4,
            z: 0,
        };
        let crossings = |b: &Basin| {
            let orient = |a: [f64; 2], b: [f64; 2], c: [f64; 2]| {
                (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
            };
            let mut count = 0;
            for (i, a) in b.segments.iter().enumerate() {
                for c in &b.segments[i + 1..] {
                    if orient(a.from, a.to, c.from) * orient(a.from, a.to, c.to) < -1e-8
                        && orient(c.from, c.to, a.from) * orient(c.from, c.to, a.to) < -1e-8
                    {
                        count += 1;
                    }
                }
            }
            count
        };
        for seed in 0..8 {
            let cfg = Config {
                radius: 512,
                advanced_rivers: true,
                meander_amplitude: 28.,
                meander_scale: 80.,
                ..config()
            };
            let old = Basin::compile_cached(coast, cfg, &CoastHill, &[coast], None, seed);
            let new = Basin::compile_cached(
                coast,
                Config {
                    refined_rivers: true,
                    ..cfg
                },
                &CoastHill,
                &[coast],
                None,
                seed,
            );
            let a = crossings(&old);
            let b = crossings(&new);
            assert!(old.segments.len() > 40 && new.segments.len() > 40);
            eprintln!(
                "seed {seed}: crossings {a}->{b}, indexed references {}->{}",
                old.bins.iter().map(Vec::len).sum::<usize>(),
                new.bins.iter().map(Vec::len).sum::<usize>()
            );
            assert!(
                b <= a,
                "seed {seed}: refined channels introduced intersecting branches ({a} -> {b})"
            );
        }
    }

    fn bank_fixture(refined: bool, height: f64, x: f64) -> Basin {
        let coast = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let mut b = Basin::compile(
            coast,
            Config {
                natural: false,
                ..config()
            },
            &CoastHill,
            &[coast],
        );
        b.raw.fill(height);
        b.filled.fill(height);
        b.ocean.fill(false);
        b.config.natural = true;
        b.config.advanced_rivers = true;
        b.config.refined_rivers = refined;
        b.segments = vec![Segment {
            from: [x, -96.],
            to: [x, 96.],
            water: [63., 63.],
            cells: 64.,
            half_width: 3.,
            depth: 3.,
            mouth: false,
        }];
        b.index();
        b
    }

    #[test]
    fn refined_high_banks_rise_in_smaller_voxel_steps_and_keep_a_bounded_footprint() {
        let old = bank_fixture(false, 83., 0.);
        let new = bank_fixture(true, 83., 0.);
        let cross_section = |b: &Basin, sign: i32| {
            (5..=70)
                .map(|d| {
                    b.sample(d * sign, 0, 83.).map_or(83, |p| {
                        assert_eq!(p.voxel_water(true), 63);
                        p.raster_floor_versioned(83, true, true)
                    })
                })
                .collect::<Vec<_>>()
        };
        for sign in [-1, 1] {
            let old_profile = cross_section(&old, sign);
            let new_profile = cross_section(&new, sign);
            let step = |p: &[i32]| p.windows(2).map(|v| (v[1] - v[0]).abs()).max().unwrap();
            assert!(
                step(&old_profile) >= 3,
                "fixture must reproduce an abrupt old bank"
            );
            assert!(step(&new_profile) <= 2, "high bank still rises as a wall");
            assert!(new_profile.iter().all(|y| *y >= 63 && *y <= 83));
            assert!(
                new.sample(60 * sign, 0, 500.).is_none(),
                "bank influence escapes its finite cap"
            );
        }
    }

    #[test]
    fn refined_bank_index_covers_extended_shoulders_and_flat_floodplains_stay_exact() {
        let mut b = bank_fixture(true, 87., 96.);
        let indexed: Vec<_> = (100..=150)
            .map(|x| b.sample(x, 0, 87.).map(|p| p.raster_floor(87, true)))
            .collect();
        assert!(
            indexed[30].is_some(),
            "fixture reaches the neighbouring spatial bin"
        );
        b.bins.iter_mut().for_each(|bin| *bin = vec![0]);
        let all_segments: Vec<_> = (100..=150)
            .map(|x| b.sample(x, 0, 87.).map(|p| p.raster_floor(87, true)))
            .collect();
        assert_eq!(
            indexed, all_segments,
            "spatial bin boundary clips the wider bank"
        );
        let old = bank_fixture(false, 67., 0.);
        let new = bank_fixture(true, 67., 0.);
        for x in -64..=64 {
            let value = |b: &Basin| b.sample(x, 0, 67.).map(|p| (p.bed, p.strength, p.water));
            assert_eq!(
                value(&old),
                value(&new),
                "a flat floodplain needs no wider cut"
            );
        }
    }

    #[test]
    fn wet_sub_block_cross_sections_keep_a_water_voxel_without_changing_v7() {
        let p = Profile {
            bed: 74.02634862624755,
            water: 74.91644262735065,
            raw: 76.,
            strength: 1.,
            lake: false,
            channel_core: true,
        };
        assert_eq!(p.voxel_floor(false), 75);
        assert_eq!(p.voxel_floor(true), 74);
        assert!(p.voxel_floor(true) < p.voxel_water(true));
    }

    #[test]
    fn equal_water_levels_do_not_gain_a_voxel_from_roundoff() {
        let mut p = river_profile();
        p.water = 63.00000000000001;
        p.bed = 62.00000000000001;
        assert_eq!(p.voxel_water(true), 63);
        assert_eq!(p.voxel_floor(true), 62);
        assert_eq!(p.voxel_water(false), 64);
        assert_eq!(p.voxel_floor(false), 63);
        p.water = 63.001;
        assert_eq!(p.voxel_water(true), 64, "keep a real level increase");
        p.water = 63.00000000000001;
        p.bed = 62.99999999999999;
        assert_eq!(p.voxel_floor(true), 62, "keep the final wet voxel");
    }

    #[test]
    fn final_raster_banks_fade_to_native_ground_without_raising_it() {
        let mut p = river_profile();
        p.bed = 60.;
        p.water = 63.;
        p.strength = 0.;
        assert_eq!(p.raster_floor(68, true), 68);
        p.strength = 0.001;
        assert_eq!(p.raster_floor(68, true), 68);
        p.strength = 0.5;
        assert_eq!(p.raster_floor(68, true), 64);
        p.strength = 1.;
        assert_eq!(p.raster_floor(68, true), 60);
        p.bed = 75.;
        assert_eq!(p.raster_floor(68, true), 68, "do not raise dry land");
        assert_eq!(p.raster_floor(68, false), 75, "saved v7 keeps its floor");
        p.lake = true;
        assert_eq!(
            p.raster_floor(47, true),
            47,
            "preserve a native lake bottom"
        );
    }

    #[test]
    fn final_raster_keeps_shallow_wet_cores_and_dry_banks_distinct() {
        let mut p = river_profile();
        p.bed = 64.02;
        p.water = 64.05;
        p.strength = 1.;
        assert_eq!(p.raster_floor(68, true), 64);
        assert!(p.raster_floor(68, true) < p.voxel_water(true));
        p.water = 64.0;
        assert_eq!(
            p.raster_floor(68, true),
            65,
            "do not wet a dry cross-section"
        );
    }

    #[test]
    fn advanced_lake_outlet_connects_native_saddle_without_recutting_lake() {
        let mut p = river_profile();
        p.lake = true;
        p.water = 70.;
        p.bed = 42.;
        assert!(p.channel_core);
        for ground in [74, 75, 76] {
            assert_eq!(p.raster_floor_versioned(ground, true, true), 69);
            assert_eq!(p.raster_floor_versioned(ground, true, false), ground);
        }
        assert_eq!(p.raster_floor_versioned(77, true, true), 77);
        assert_eq!(p.raster_floor_versioned(47, true, true), 47);
        assert_eq!(
            p.raster_floor_versioned(75, false, true),
            p.raster_floor(75, false)
        );
        p.channel_core = false;
        assert_eq!(p.raster_floor_versioned(75, true, true), 75);
    }

    #[test]
    fn a_lake_channel_crosses_only_a_shallow_dry_saddle() {
        let mut p = river_profile();
        p.lake = true;
        p.water = 70.;
        p.bed = 42.; // A pre-lake channel must not determine the new cut depth.
        assert!(p.channel_core);
        assert_eq!(
            p.raster_floor(72, true),
            69,
            "one water voxel through the saddle"
        );
        assert_eq!(
            p.raster_floor(73, true),
            69,
            "include the three-block saddle bound"
        );
        assert_eq!(
            p.raster_floor(74, true),
            74,
            "stop at the shallow-saddle bound"
        );
        assert_eq!(p.raster_floor(80, true), 80, "preserve high islands");
        assert_eq!(
            p.raster_floor(47, true),
            47,
            "preserve an already wet lake bottom"
        );
        let legacy_floor = p.raster_floor(72, false);
        p.channel_core = false;
        assert_eq!(
            p.raster_floor(72, true),
            72,
            "preserve non-channel islands and shores"
        );
        assert_eq!(
            p.raster_floor(72, false),
            legacy_floor,
            "saved legacy recipe ignores the new flag"
        );
        p.channel_core = true;
        let ocean = reconcile_native_water(
            p,
            Some(NativeWater {
                ground: 52,
                water: 63,
                ocean: true,
            }),
            63,
        );
        assert!(ocean.lake && !ocean.channel_core);
        assert_eq!(
            (ocean.bed, ocean.water, ocean.raster_floor(52, true)),
            (52., 63., 52)
        );
    }

    fn lake_fixture() -> Basin {
        let c = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let mut b = Basin::compile(c, config(), &CoastHill, &[c]);
        b.segments.clear();
        b.index();
        b.raw.fill(66.);
        b.filled.fill(70.);
        b.active.fill(true);
        b.drains_ocean.fill(true);
        b.ocean.fill(false);
        b
    }

    fn river_profile() -> Profile {
        Profile {
            raw: 68.,
            bed: 64.,
            water: 67.,
            strength: 1.,
            lake: false,
            channel_core: true,
        }
    }
    #[test]
    fn native_sea_preserves_actual_underwater_depth() {
        let p = reconcile_native_water(
            river_profile(),
            Some(NativeWater {
                ground: 52,
                water: 63,
                ocean: true,
            }),
            63,
        );
        assert_eq!((p.bed, p.water, p.lake), (52., 63., true));
    }
    #[test]
    fn a_native_inland_lake_below_sea_is_not_lowered_into_a_pit() {
        let p = reconcile_native_water(
            river_profile(),
            Some(NativeWater {
                ground: 56,
                water: 70,
                ocean: false,
            }),
            63,
        );
        assert_eq!((p.bed, p.water, p.lake), (56., 70., true));
    }

    #[test]
    fn a_planned_inland_lake_can_fill_to_its_spill_without_raising_the_sea() {
        let mut planned = river_profile();
        planned.lake = true;
        let native = NativeWater {
            ground: 47,
            water: 63,
            ocean: false,
        };
        let inland = reconcile_native_water(planned, Some(native), 63);
        assert_eq!((inland.bed, inland.water, inland.lake), (47., 67., true));
        let ocean = reconcile_native_water(
            planned,
            Some(NativeWater {
                ocean: true,
                ..native
            }),
            63,
        );
        assert_eq!((ocean.bed, ocean.water, ocean.lake), (47., 63., true));
        let higher = reconcile_native_water(
            planned,
            Some(NativeWater {
                water: 70,
                ..native
            }),
            63,
        );
        assert_eq!((higher.bed, higher.water, higher.lake), (47., 70., true));
    }
    #[test]
    fn dry_native_columns_keep_the_planned_river_profile() {
        let p = reconcile_native_water(river_profile(), None, 63);
        assert_eq!((p.bed, p.water, p.lake), (64., 67., false));
    }

    #[test]
    fn lake_shore_does_not_copy_a_dry_cell_over_the_ocean() {
        let mut b = lake_fixture();
        let i = 3 * b.side + 3;
        b.ocean[i + 1] = true;
        assert_eq!(b.lake_level(3.2, 3.2), None);
        b.config.natural = false;
        assert_eq!(
            b.lake_level(3.2, 3.2),
            Some(70.),
            "saved v7 keeps its exact nearest-cell recipe"
        );
    }

    #[test]
    fn inland_lake_uses_one_shared_spill_level_without_square_steps() {
        let mut b = lake_fixture();
        assert_eq!(b.lake_level(3.1, 3.1), Some(70.));
        assert_eq!(b.lake_level(3.9, 3.9), Some(70.));
        b.filled[4 * b.side + 4] = 72.;
        assert_eq!(b.lake_level(3.2, 3.2), None);
    }

    #[test]
    fn high_dry_banks_do_not_erase_the_inland_lake_shore_tile() {
        let mut b = lake_fixture();
        let i = 3 * b.side + 3;
        let corners = [i, i + 1, i + b.side, i + b.side + 1];
        // Seed42 regression: a native depression next to an above-spill bank.
        for (j, (raw, filled)) in
            corners
                .into_iter()
                .zip([(67., 67.), (25., 66.), (59., 66.), (25., 66.)])
        {
            b.raw[j] = raw;
            b.filled[j] = filled;
        }
        assert_eq!(b.lake_level(3.2, 3.2), Some(66.));
        b.ocean[i + 1] = true;
        assert_eq!(b.lake_level(3.2, 3.2), None, "do not flood a sea tile");
        b.ocean[i + 1] = false;
        for j in corners {
            b.filled[j] = b.raw[j];
        }
        assert_eq!(b.lake_level(3.2, 3.2), None, "no invented dry-land pond");
    }

    #[test]
    fn a_coarse_subsea_dem_does_not_decide_an_inland_lakes_water_level() {
        let mut b = lake_fixture();
        b.active.fill(false);
        b.segments = vec![Segment {
            from: [-16., 0.],
            to: [16., 0.],
            water: [68., 66.],
            cells: 24.,
            half_width: 3.,
            depth: 3.,
            mouth: true,
        }];
        b.index();
        assert_eq!(b.sample(0, 0, 60.).unwrap().water, 67.);
        b.config.natural = false;
        assert_eq!(b.sample(0, 0, 60.).unwrap().water, 67.);
    }
    #[test]
    fn flood_routes_conserve_and_descend_to_receiving_water() {
        let c = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: -4,
            z: 0,
        };
        let b = Basin::compile(
            c,
            Config {
                radius: 512,
                ..config()
            },
            &CoastHill,
            &[c],
        );
        assert!(b.input > 100. && b.segments.len() > 10);
        assert!((b.input - b.output).abs() < 1e-9);
        for (i, parent) in b.parent.iter().enumerate() {
            if !b.drains_ocean[i] {
                continue;
            }
            let mut p = i;
            for count in 0..=b.raw.len() {
                assert!(count < b.raw.len(), "cycle");
                if let Some(j) = b.parent[p] {
                    assert!(b.filled[p] >= b.filled[j]);
                    p = j;
                } else {
                    assert!(b.ocean[p]);
                    break;
                }
            }
            if let Some(j) = parent {
                assert!(b.discharge[*j] >= b.discharge[i]);
            }
        }
        assert!(b.segments.iter().any(|s| s.mouth));
    }
    #[test]
    fn independent_query_orders_and_cache_eviction_preserve_profiles() {
        let a = World::new(
            CoastHill,
            42,
            Config {
                basin_cache: 1,
                ..config()
            },
        );
        let b = World::new(
            CoastHill,
            42,
            Config {
                basin_cache: 2,
                ..config()
            },
        );
        let mut points = vec![(-8, -40), (40, 0), (100, 110), (10, -220), (180, -130)];
        for z in -1..=1 {
            for x in -1..=1 {
                if let Some(c) = a.coast(Key(x, z)) {
                    let basin = a.basin(c);
                    if let Some(s) = basin.segments.first() {
                        points.push((
                            ((s.from[0] + s.to[0]) * 0.5).round() as i32,
                            ((s.from[1] + s.to[1]) * 0.5).round() as i32,
                        ));
                    }
                }
            }
        }
        let expected: Vec<_> = points.iter().map(|&(x, z)| a.sample(x, z)).collect();
        for (i, &(x, z)) in points.iter().enumerate().rev() {
            let actual = b.sample(x, z);
            assert_eq!(expected[i].is_some(), actual.is_some());
            if let (Some(a), Some(b)) = (expected[i], actual) {
                assert_eq!((a.bed, a.water, a.strength), (b.bed, b.water, b.strength));
            }
        }
        assert!(expected.iter().any(Option::is_some));
    }
    #[test]
    fn closed_highland_has_no_invented_sea_outlet() {
        #[derive(Debug)]
        struct Plateau;
        impl Terrain for Plateau {
            fn land(&self, _: i32, _: i32) -> Land {
                Land {
                    height: 120.,
                    ocean: false,
                }
            }
        }
        let world = World::new(Plateau, 1, config());
        assert!(world.sample(-100, 300).is_none());
        assert!(world.coast(Key(-1, -1)).is_none());
    }

    #[test]
    fn a_lower_riverbank_cannot_erase_a_nearby_wet_channel() {
        let c = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let mut basin = Basin::compile(c, config(), &CoastHill, &[c]);
        basin.segments = vec![
            Segment {
                from: [-16., 0.],
                to: [16., 0.],
                water: [100., 100.],
                cells: 8.,
                half_width: 2.,
                depth: 3.,
                mouth: false,
            },
            Segment {
                from: [-16., 10.],
                to: [16., 10.],
                water: [70., 70.],
                cells: 100.,
                half_width: 4.,
                depth: 3.,
                mouth: false,
            },
        ];
        basin.index();
        let p = basin.sample(0, 0, 100.).unwrap();
        assert_eq!(
            p.water, 100.,
            "a dry feather from the lower river replaced the wet centreline"
        );
        assert!(p.bed < p.water && p.strength == 1.);
    }

    #[test]
    fn published_channel_centrelines_stay_outside_the_boundary_fade() {
        let world = World::new(
            CoastHill,
            42,
            Config {
                radius: 512,
                root_cell: 768,
                ..config()
            },
        );
        let mut count = 0;
        for z in -2..=2 {
            for x in -1..=1 {
                let Some(c) = world.coast(Key(x, z)) else {
                    continue;
                };
                let basin = world.basin(c);
                for s in &basin.segments {
                    let p = [
                        ((s.from[0] + s.to[0]) * 0.5).round() as i32,
                        ((s.from[1] + s.to[1]) * 0.5).round() as i32,
                    ];
                    let profile = world
                        .sample(p[0], p[1])
                        .expect("published channel disappeared");
                    assert!(
                        profile.strength > 0.999,
                        "dry boundary centreline: {p:?} {profile:?}"
                    );
                    assert!(profile.bed < profile.water);
                    assert!(
                        profile.voxel_floor(true) < profile.voxel_water(true),
                        "a wet centreline disappeared when quantized into blocks"
                    );
                    count += 1;
                }
            }
        }
        assert!(count > 3, "fixture needs actual published river paths");
    }

    #[test]
    fn a_filled_lake_covers_a_crossing_channel_without_recutting_its_bottom() {
        let c = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let mut basin = Basin::compile(c, config(), &CoastHill, &[c]);
        basin.raw.fill(100.);
        basin.filled.fill(150.);
        basin.active.fill(true);
        basin.drains_ocean.fill(true);
        basin.ocean.fill(false);
        basin.segments = vec![Segment {
            from: [-16., 0.],
            to: [16., 0.],
            water: [100., 100.],
            cells: 8.,
            half_width: 2.,
            depth: 3.,
            mouth: false,
        }];
        basin.index();
        let profile = basin.sample(0, 0, 100.).unwrap();
        assert!(profile.lake && profile.strength == 1.);
        assert!(
            profile.channel_core,
            "lake overlay retains the channel's core"
        );
        assert_eq!(profile.water, 150.);
        assert_eq!(profile.bed, 100.);
        let bank = basin.sample(0, 3, 100.).unwrap();
        assert!(bank.lake);
        assert!(!bank.channel_core, "lake shores are not channel cores");
        assert_eq!(bank.water, profile.water);
        let native = NativeWater {
            ground: 47,
            water: 63,
            ocean: false,
        };
        assert_eq!(
            reconcile_native_water(profile, Some(native), 63).water,
            150.
        );
        basin.config.natural = false;
        let legacy = basin.sample(0, 0, 100.).unwrap();
        assert!(!legacy.lake, "saved v7 retains its old channel profile");
        assert_eq!(legacy.water, 150.);
        assert!(legacy.bed < 100.);
    }

    #[test]
    fn coastline_on_search_cell_edge_has_unique_outlet_owners() {
        #[derive(Debug)]
        struct Straight;
        impl Terrain for Straight {
            fn land(&self, x: i32, _: i32) -> Land {
                Land {
                    height: 63. + f64::from(x) * 0.2,
                    ocean: x <= 0,
                }
            }
        }
        let w = World::new(Straight, 17, config());
        let roots = w.roots_around(0, 0, 500);
        assert!(roots.len() > 2);
        let positions: std::collections::BTreeSet<_> = roots.iter().map(|r| (r.x, r.z)).collect();
        assert_eq!(positions.len(), roots.len());
        for r in roots {
            assert_eq!(r.x.div_euclid(w.config.root_cell), r.key.0);
            assert_eq!(r.z.div_euclid(w.config.root_cell), r.key.1);
        }
    }

    #[test]
    fn persisted_dem_reopens_and_rejects_corruption_seed_and_recipe_changes() {
        let directory = std::env::temp_dir().join(format!(
            "il-dem-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let config = config();
        let c = Coast {
            inland_level: None,
            key: Key(0, 0),
            x: 0,
            z: 0,
        };
        let count = (config.radius * 2 / config.step + 1).pow(2) as usize;
        let data = || {
            vec![
                Land {
                    height: 72.,
                    ocean: false
                };
                count
            ]
        };
        let cache = DemCache::new(directory.clone(), 100);
        assert_eq!(cache.samples(42, c, config, data).len(), count);
        let reopened = DemCache::new(directory.clone(), 100);
        assert_eq!(
            reopened.samples(42, c, config, || panic!("valid DEM was recalculated"))[0].height,
            72.
        );
        assert_eq!(reopened.hits.load(AtomicOrdering::Relaxed), 1);
        let file = std::fs::read_dir(&directory)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let mut bytes = std::fs::read(&file).unwrap();
        bytes[65] ^= 1;
        std::fs::write(&file, bytes).unwrap();
        assert_eq!(reopened.samples(42, c, config, data)[0].height, 72.);
        assert_eq!(reopened.misses.load(AtomicOrdering::Relaxed), 1);
        let recipe = DemCache::new(directory.clone(), 101);
        recipe.samples(42, c, config, data);
        assert_eq!(recipe.misses.load(AtomicOrdering::Relaxed), 1);
        reopened.samples(43, c, config, data);
        assert_eq!(reopened.misses.load(AtomicOrdering::Relaxed), 2);
        // This unique test directory contains only the files created above.
        for entry in std::fs::read_dir(&directory).unwrap() {
            std::fs::remove_file(entry.unwrap().path()).unwrap();
        }
        std::fs::remove_dir(directory).unwrap();
    }
}
