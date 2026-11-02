//! Priority-flood drainage, runoff conservation and explicit river geometry.
//! O(N log N) compilation, O(N) storage. Not a time-stepped fluid solver.
use super::terrain::HeightField;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone, Debug)]
pub struct Cell {
    pub ground: f64,
    pub ocean: bool,
    pub spill_height: f64,
    pub downstream: Option<usize>,
    pub discharge: f64,
    pub catchment_area: f64,
    pub lake_depth: f64,
}

#[derive(Clone, Debug)]
pub struct RiverSegment {
    pub from: usize,
    pub to: usize,
    pub discharge: f64,
    pub width: f64,
    pub depth: f64,
    pub floodplain_width: f64,
    pub water_from: f64,
    pub water_to: f64,
}

#[derive(Clone, Debug)]
pub struct Delta {
    pub mouth: usize,
    /// Distributary center lines from the same mouth into the receiving water.
    pub branches: Vec<Vec<[f64; 2]>>,
    pub branch_discharge: Vec<f64>,
    pub incoming_discharge: f64,
}

#[derive(Clone, Debug)]
pub struct Basin {
    pub field: HeightField,
    pub cells: Vec<Cell>,
    pub rivers: Vec<RiverSegment>,
    pub deltas: Vec<Delta>,
    pub runoff_input: f64,
    pub outlet_discharge: f64,
    /// Upstream cells always have a larger rank than their downstream cell.
    pub rank: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
struct Frontier {
    height: f64,
    index: usize,
}
impl PartialEq for Frontier {
    fn eq(&self, o: &Self) -> bool {
        self.height == o.height && self.index == o.index
    }
}
impl Eq for Frontier {}
impl PartialOrd for Frontier {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Frontier {
    fn cmp(&self, o: &Self) -> Ordering {
        o.height
            .total_cmp(&self.height)
            .then_with(|| o.index.cmp(&self.index))
    }
}

impl Basin {
    pub fn compile(field: HeightField) -> Self {
        let n = field.config.side;
        let sea = field.config.sea_level;
        let area = field.config.cell_size.powi(2);
        let count = n * n;
        let mut cells: Vec<Cell> = field
            .ground
            .iter()
            .map(|&h| Cell {
                ground: h,
                ocean: false,
                spill_height: h,
                downstream: None,
                discharge: 0.0,
                catchment_area: 0.0,
                lake_depth: 0.0,
            })
            .collect();
        let mut frontier = BinaryHeap::new();
        let mut seen = vec![false; count];
        let mut order = Vec::with_capacity(count);
        let mut rank = vec![0; count];
        // A depression below sea level is not automatically an ocean. Only
        // submerged cells connected to the region boundary receive that label.
        let mut coast = VecDeque::new();
        for (i, cell) in cells.iter_mut().enumerate() {
            let x = i % n;
            let z = i / n;
            if (x == 0 || z == 0 || x == n - 1 || z == n - 1) && cell.ground <= sea {
                cell.ocean = true;
                coast.push_back(i);
            }
        }
        while let Some(i) = coast.pop_front() {
            for j in neighbors(i, n) {
                if !cells[j].ocean && cells[j].ground <= sea {
                    cells[j].ocean = true;
                    coast.push_back(j);
                }
            }
        }
        for i in 0..count {
            let x = i % n;
            let z = i / n;
            if x == 0 || z == 0 || x == n - 1 || z == n - 1 || cells[i].ocean {
                cells[i].spill_height = cells[i].ground.max(sea);
                seen[i] = true;
                frontier.push(Frontier {
                    height: cells[i].spill_height,
                    index: i,
                });
            }
        }
        while let Some(Frontier { height, index }) = frontier.pop() {
            rank[index] = order.len();
            order.push(index);
            for neighbor in neighbors(index, n) {
                if seen[neighbor] {
                    continue;
                }
                seen[neighbor] = true;
                let filled = cells[neighbor].ground.max(height);
                cells[neighbor].spill_height = filled;
                cells[neighbor].downstream = Some(index);
                cells[neighbor].lake_depth = (filled - cells[neighbor].ground).max(0.0);
                frontier.push(Frontier {
                    height: filled,
                    index: neighbor,
                });
            }
        }
        // Prefer the steepest already-processed neighbor on ordinary slopes.
        // Flood parents are retained across level lake surfaces: no cycles.
        for &i in &order {
            if cells[i].downstream.is_none() {
                continue;
            }
            let mut best = 0.0_f64;
            for j in neighbors(i, n) {
                if rank[j] >= rank[i] {
                    continue;
                }
                let diagonal = i % n != j % n && i / n != j / n;
                let distance = field.config.cell_size
                    * if diagonal {
                        std::f64::consts::SQRT_2
                    } else {
                        1.0
                    };
                let slope = (cells[i].spill_height - cells[j].spill_height) / distance;
                if slope > best {
                    best = slope;
                    cells[i].downstream = Some(j);
                }
            }
        }
        let mut runoff_input = 0.0;
        for (i, c) in cells.iter_mut().enumerate() {
            if !c.ocean {
                // Indicative steady runoff, m^3/s: annual rainfall, runoff fraction,
                // cell area. Climate calibration remains a future task.
                c.discharge =
                    field.climate[i].rainfall_mm_year / 1000.0 * 0.45 * area / 31_557_600.0;
                c.catchment_area = area;
                runoff_input += c.discharge;
            }
        }
        for &i in order.iter().rev() {
            if let Some(j) = cells[i].downstream {
                cells[j].discharge += cells[i].discharge;
                cells[j].catchment_area += cells[i].catchment_area;
            }
        }
        let outlet_discharge = cells
            .iter()
            .filter(|c| c.downstream.is_none())
            .map(|c| c.discharge)
            .sum();
        let mut rivers = Vec::new();
        for (i, c) in cells.iter().enumerate() {
            let Some(j) = c.downstream else {
                continue;
            };
            if c.catchment_area < area * 24.0 || c.lake_depth > 1.5 {
                continue;
            }
            let width = (3.0 + 5.0 * c.discharge.sqrt()).min(180.0);
            let depth = (1.0 + c.discharge.powf(0.3)).min(16.0);
            rivers.push(RiverSegment {
                from: i,
                to: j,
                discharge: c.discharge,
                width,
                depth,
                floodplain_width: width
                    * (3.0 + 10.0 / (1.0 + (c.spill_height - cells[j].spill_height).max(0.0))),
                water_from: c.spill_height,
                water_to: cells[j].spill_height,
            });
        }
        let deltas = make_deltas(&field, &cells, &rivers);
        Self {
            field,
            cells,
            rivers,
            deltas,
            runoff_input,
            outlet_discharge,
            rank,
        }
    }

    /// Bilinear coarse field for previews. The future block adapter must carve
    /// explicit river cross-sections, rather than treating this as final blocks.
    pub fn height(&self, x: f64, z: f64) -> f64 {
        let n = self.field.config.side;
        let half = (n - 1) as f64 * self.field.config.cell_size / 2.0;
        let gx = ((x + half) / self.field.config.cell_size).clamp(0.0, (n - 1) as f64);
        let gz = ((z + half) / self.field.config.cell_size).clamp(0.0, (n - 1) as f64);
        let ix = (gx.floor() as usize).min(n - 2);
        let iz = (gz.floor() as usize).min(n - 2);
        let u = gx - ix as f64;
        let v = gz - iz as f64;
        let a = self.cells[iz * n + ix].ground * (1.0 - u) + self.cells[iz * n + ix + 1].ground * u;
        let b = self.cells[(iz + 1) * n + ix].ground * (1.0 - u)
            + self.cells[(iz + 1) * n + ix + 1].ground * u;
        a * (1.0 - v) + b * v
    }
}

fn neighbors(i: usize, n: usize) -> impl Iterator<Item = usize> {
    let x = (i % n) as isize;
    let z = (i / n) as isize;
    (-1..=1).flat_map(move |dz| {
        (-1..=1).filter_map(move |dx| {
            let nx = x + dx;
            let nz = z + dz;
            if (dx != 0 || dz != 0) && nx >= 0 && nz >= 0 && nx < n as isize && nz < n as isize {
                Some(nz as usize * n + nx as usize)
            } else {
                None
            }
        })
    })
}

fn make_deltas(field: &HeightField, cells: &[Cell], rivers: &[RiverSegment]) -> Vec<Delta> {
    let mut deltas = Vec::new();
    for river in rivers {
        let mouth = &cells[river.to];
        // Prototype river-dominated mouths only. Not every coast gets a delta.
        if !mouth.ocean || river.discharge < 0.8 {
            continue;
        }
        let from = field.point(river.from);
        let to = field.point(river.to);
        let dx = to[0] - from[0];
        let dz = to[1] - from[1];
        let length = dx.hypot(dz);
        if (river.water_from - field.config.sea_level) / length > 0.035 {
            continue;
        }
        let direction = [dx / length, dz / length];
        let branches_n = 3;
        let reach = (300.0 + river.discharge.sqrt() * 150.0).min(1800.0);
        let mut branches = Vec::new();
        for b in 0..branches_n {
            let angle = (b as f64 - 1.0) * 0.6;
            let mut path = vec![to];
            for step in 1..=12 {
                let t = step as f64 / 12.0;
                let forward = reach * t;
                let side = angle * reach * t * t;
                path.push([
                    to[0] + direction[0] * forward - direction[1] * side,
                    to[1] + direction[1] * forward + direction[0] * side,
                ]);
            }
            // Reject branches crossing land: a fan is not permission to cut a mountain.
            if path
                .iter()
                .skip(1)
                .any(|p| nearest_height(field, *p).is_none_or(|h| h > field.config.sea_level + 2.0))
            {
                continue;
            }
            branches.push(path);
        }
        if branches.len() < 2 {
            continue;
        }
        let branch_discharge = vec![river.discharge / branches.len() as f64; branches.len()];
        deltas.push(Delta {
            mouth: river.to,
            branches,
            branch_discharge,
            incoming_discharge: river.discharge,
        });
    }
    deltas
}

fn nearest_height(field: &HeightField, p: [f64; 2]) -> Option<f64> {
    let n = field.config.side;
    let half = (n - 1) as f64 * field.config.cell_size / 2.0;
    // Outside this compiled region there is no evidence of receiving water.
    // Clamping would fabricate an ocean beyond the data we actually have.
    if p[0].abs() > half || p[1].abs() > half {
        return None;
    }
    let x = ((p[0] + half) / field.config.cell_size)
        .round()
        .clamp(0.0, (n - 1) as f64) as usize;
    let z = ((p[1] + half) / field.config.cell_size)
        .round()
        .clamp(0.0, (n - 1) as f64) as usize;
    Some(field.ground[z * n + x])
}
