//! Shared surface queries for future chunk and distant-terrain adapters.
//! Drainage and the spatial index are compiled once; queries do no basin solving.
use super::{Basin, Climate};

#[derive(Clone, Copy, Debug)]
pub struct Surface {
    /// River beds and floodplains are carved into the original macro terrain.
    pub ground: f64,
    pub water: Option<f64>,
    pub climate: Climate,
}

#[derive(Clone, Copy, Debug)]
struct Channel {
    a: [f64; 2],
    b: [f64; 2],
    water: [f64; 2],
    radius: f64,
    depth: f64,
    flood_radius: f64,
}

/// Immutable, bounded region. Share it between workers with `Arc`.
/// Outside its known extent a query returns `None`, never a fabricated edge.
pub struct Landscape {
    basin: Basin,
    channels: Vec<Channel>,
    buckets: Vec<Vec<usize>>,
}

impl Landscape {
    pub fn new(basin: Basin) -> Self {
        let n = basin.field.config.side;
        let mut result = Self {
            basin,
            channels: Vec::new(),
            buckets: vec![Vec::new(); n * n],
        };
        for i in 0..result.basin.rivers.len() {
            let r = &result.basin.rivers[i];
            result.index(Channel {
                a: result.basin.field.point(r.from),
                b: result.basin.field.point(r.to),
                water: [r.water_from, r.water_to],
                radius: r.width / 2.0,
                depth: r.depth,
                flood_radius: r.floodplain_width / 2.0,
            });
        }
        // Distributaries affect the same surface queried by voxel chunks and LOD.
        let mut branches = Vec::new();
        for delta in &result.basin.deltas {
            for (path, discharge) in delta.branches.iter().zip(&delta.branch_discharge) {
                let radius = (3.0 + 5.0 * discharge.sqrt()).min(180.0) / 2.0;
                for pair in path.windows(2) {
                    branches.push(Channel {
                        a: pair[0],
                        b: pair[1],
                        water: [result.basin.field.config.sea_level; 2],
                        radius,
                        depth: (1.0 + discharge.powf(0.3)).min(16.0),
                        flood_radius: radius * 3.0,
                    });
                }
            }
        }
        for channel in branches {
            result.index(channel);
        }
        result
    }

    pub fn basin(&self) -> &Basin {
        &self.basin
    }

    fn grid(&self, p: [f64; 2]) -> [f64; 2] {
        let c = self.basin.field.config;
        let half = (c.side - 1) as f64 / 2.0;
        [p[0] / c.cell_size + half, p[1] / c.cell_size + half]
    }

    fn index(&mut self, channel: Channel) {
        let n = self.basin.field.config.side;
        let lo = self.grid([
            channel.a[0].min(channel.b[0]) - channel.flood_radius,
            channel.a[1].min(channel.b[1]) - channel.flood_radius,
        ]);
        let hi = self.grid([
            channel.a[0].max(channel.b[0]) + channel.flood_radius,
            channel.a[1].max(channel.b[1]) + channel.flood_radius,
        ]);
        let clamp = |v: f64| v.floor().clamp(0.0, (n - 1) as f64) as usize;
        let id = self.channels.len();
        self.channels.push(channel);
        for z in clamp(lo[1])..=clamp(hi[1]) {
            for x in clamp(lo[0])..=clamp(hi[0]) {
                self.buckets[z * n + x].push(id);
            }
        }
    }

    pub fn sample(&self, x: f64, z: f64) -> Option<Surface> {
        let c = self.basin.field.config;
        let n = c.side;
        let extent = (n - 1) as f64 * c.cell_size / 2.0;
        if !x.is_finite() || !z.is_finite() || x.abs() > extent || z.abs() > extent {
            return None;
        }
        let [gx, gz] = self.grid([x, z]);
        let ix = (gx.floor() as usize).min(n - 2);
        let iz = (gz.floor() as usize).min(n - 2);
        let (u, v) = (gx - ix as f64, gz - iz as f64);
        let ids = [
            iz * n + ix,
            iz * n + ix + 1,
            (iz + 1) * n + ix,
            (iz + 1) * n + ix + 1,
        ];
        let weights = [(1.0 - u) * (1.0 - v), u * (1.0 - v), (1.0 - u) * v, u * v];
        let mut ground = 0.0;
        let mut water = f64::NEG_INFINITY;
        let mut climate = Climate {
            temperature_c: 0.0,
            rainfall_mm_year: 0.0,
            moisture: 0.0,
        };
        for (id, weight) in ids.into_iter().zip(weights) {
            let cell = &self.basin.cells[id];
            let local = self.basin.field.climate[id];
            ground += cell.ground * weight;
            climate.temperature_c += local.temperature_c * weight;
            climate.rainfall_mm_year += local.rainfall_mm_year * weight;
            climate.moisture += local.moisture * weight;
            // Keep lakes level; interpolate the shore/bathymetry, not their water.
            if weight > 0.0 && (cell.ocean || cell.lake_depth > 0.0) {
                water = water.max(cell.spill_height);
            }
        }
        let terrain_ground = ground;
        let bucket = (gz.floor() as usize).min(n - 1) * n + (gx.floor() as usize).min(n - 1);
        for &id in &self.buckets[bucket] {
            let channel = self.channels[id];
            let dx = channel.b[0] - channel.a[0];
            let dz = channel.b[1] - channel.a[1];
            let length2 = dx * dx + dz * dz;
            if length2 == 0.0 {
                continue;
            }
            let t = (((x - channel.a[0]) * dx + (z - channel.a[1]) * dz) / length2).clamp(0.0, 1.0);
            let distance = (x - channel.a[0] - t * dx).hypot(z - channel.a[1] - t * dz);
            if distance > channel.flood_radius {
                continue;
            }
            let level = channel.water[0] * (1.0 - t) + channel.water[1] * t;
            let bed = if distance <= channel.radius {
                water = water.max(level);
                level - channel.depth * (1.0 - (distance / channel.radius).powi(2))
            } else {
                let a = (distance - channel.radius) / (channel.flood_radius - channel.radius);
                let smooth = a * a * (3.0 - 2.0 * a);
                level + (terrain_ground - level) * smooth
            };
            ground = ground.min(bed);
        }
        Some(Surface {
            ground,
            water: (water > ground + 1e-6).then_some(water),
            climate,
        })
    }
}
