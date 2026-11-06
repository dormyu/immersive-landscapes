// Compare every actual/far column around the reported seed42 pond.
// This is native TERRAIN evidence, not decoration or visual acceptance.
use horizon_core::position::ChunkPos;
use horizon_world::worldgen::{
    generator::{GeneratorData, OverworldGenerator, ProtoChunk},
    overrides::WorldgenOverrides,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};
type Error = Box<dyn std::error::Error + Send + Sync>;

fn files(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files(root, &path, out)?;
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push((
                path.strip_prefix(root)?
                    .to_string_lossy()
                    .replace('\\', "/"),
                fs::read_to_string(path)?,
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Column {
    ground: i32,
    water: Option<i32>,
}
fn actual_column(p: &ProtoChunk, x: i32, z: i32) -> Column {
    let min = p.terrain.min_y();
    let max = min + p.terrain.height();
    let mut water = None;
    for y in (min..max).rev() {
        match p.block_name(x, y, z) {
            "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" => {}
            "minecraft:water" => {
                water.get_or_insert(y + 1);
            }
            "minecraft:lava" => {}
            _ => {
                return Column {
                    ground: y + 1,
                    water,
                };
            }
        }
    }
    Column { ground: min, water }
}
fn column_json(c: Column) -> Value {
    json!({"ground": c.ground, "water": c.water})
}

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: native_bank_probe RESOURCES PRESET REPORT SEED".into());
    }
    let resources = PathBuf::from(&args[1]);
    let preset = PathBuf::from(&args[2]);
    let report = PathBuf::from(&args[3]);
    let seed: i64 = args[4].parse()?;
    let registry = Box::leak(Box::new(horizon_world::level::data::load_registry(
        &resources,
    )?));
    let mut ov = WorldgenOverrides::default();
    let mut plain = WorldgenOverrides::default();
    let mut entries = Vec::new();
    files(&preset, &preset.join("data"), &mut entries)?;
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, text) in entries {
        if !name.ends_with("/watershed_v1.json") {
            plain.add_file("immersive-landscapes", &name, || Some(text.clone()));
        }
        ov.add_file("immersive-landscapes", &name, || Some(text));
    }
    let metadata: Value = serde_json::from_slice(&fs::read(preset.join("IL-PRESET.json"))?)?;
    let originals: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(resources.join("worldgen/densities.json"))?)?;
    for (alias, original) in metadata["vanilla_aliases"].as_object().ok_or("aliases")? {
        let path = format!(
            "data/immersive_landscapes/worldgen/density_function/{}.json",
            alias.split_once(':').ok_or("alias")?.1
        );
        let text = originals
            .get(original.as_str().ok_or("original")?)
            .ok_or("private density")?
            .clone();
        plain.add_file("immersive-landscapes", &path, || Some(text.clone()));
        ov.add_file("immersive-landscapes", &path, || Some(text));
    }
    let start = Instant::now();
    let data = GeneratorData::load_with(&resources.join("worldgen"), registry, &ov)?;
    let generator = OverworldGenerator::without_structures(data, seed)?;
    let mut worker = generator.worker();
    let plain_data = GeneratorData::load_with(&resources.join("worldgen"), registry, &plain)?;
    let plain_generator = OverworldGenerator::without_structures(plain_data, seed)?;
    let mut plain_worker = plain_generator.worker();
    let atlas = horizon_world::worldgen::immersive_landscapes_hydrology::live_atlases()
        .into_iter()
        .find(|a| a.seed == seed)
        .ok_or("no native hydrology atlas")?;

    let mut rows = Vec::with_capacity(64 * 256);
    let mut failures = Vec::new();
    let mut failure_count = 0;
    let mut cut_columns = 0;
    let mut dry_cut_columns = 0;
    let mut lake_fill_columns = 0;
    let mut preserved_water_columns = 0;
    let mut roundoff_columns = 0;
    let mut native_ground_discrepancies = 0;
    let mut raw_ground_mismatches = 0;
    let mut raw_water_mismatches = 0;
    let mut lake_cells = BTreeMap::<(i32, i32), (i32, Option<i32>, Option<i32>)>::new();
    for cz in -151..=-144 {
        for cx in -148..=-141 {
            let old = plain_worker.generate_proto(ChunkPos::new(cx, cz))?;
            let now = worker.generate_proto(ChunkPos::new(cx, cz))?;
            for z in 0..16 {
                for x in 0..16 {
                    let pos = (cx * 16 + x, cz * 16 + z);
                    let old_actual = actual_column(&old, x, z);
                    let actual = actual_column(&now, x, z);
                    let old_far = plain_worker.surface_column(pos.0, pos.1).0;
                    let far = worker.surface_column(pos.0, pos.1).0;
                    let profile = atlas.profile_at(pos.0, pos.1);
                    let mut errors = Vec::new();
                    let mut mode = "unaffected";
                    let mut expected = None;
                    if let (Some(p), Some(base), Some(surface)) = (profile, old_far, far) {
                        let floor = p.raster_floor(base.height, true);
                        let water = p.voxel_water(true);
                        let target_water = (water > floor).then_some(water);
                        let cut = !p.lake && floor < base.height;
                        let fill = p.lake && water > base.water.map_or(base.height, |w| w.0);
                        expected = Some(Column {
                            ground: floor,
                            water: target_water,
                        });
                        if old_actual.ground != base.height {
                            native_ground_discrepancies += 1;
                        }
                        if actual.ground != surface.height {
                            raw_ground_mismatches += 1;
                        }
                        if actual.water != surface.water.map(|w| w.0) {
                            raw_water_mismatches += 1;
                        }
                        if cut {
                            mode = "cut";
                            cut_columns += 1;
                            if target_water.is_none() {
                                dry_cut_columns += 1;
                            }
                            if actual.ground != floor {
                                errors.push("actual_cut_floor");
                            }
                            if actual.water != target_water {
                                errors.push("actual_cut_water");
                            }
                            if surface.height != floor {
                                errors.push("far_cut_floor");
                            }
                            if surface.water.map(|w| w.0) != target_water {
                                errors.push("far_cut_water");
                            }
                        } else if p.lake {
                            mode = if fill {
                                "lake_fill"
                            } else {
                                "preserved_native_water"
                            };
                            if fill {
                                lake_fill_columns += 1;
                            } else {
                                preserved_water_columns += 1;
                            }
                            if actual.ground != old_actual.ground {
                                errors.push("changed_native_lake_bottom");
                            }
                            if surface.height != base.height {
                                errors.push("changed_native_far_bottom");
                            }
                            if fill {
                                if actual.water != target_water {
                                    errors.push("actual_lake_water");
                                }
                                if surface.water.map(|w| w.0) != target_water {
                                    errors.push("far_lake_water");
                                }
                            } else if actual.water != old_actual.water {
                                errors.push("changed_preserved_native_water");
                            }
                            if fill && target_water.is_some() {
                                lake_cells
                                    .insert(pos, (water, actual.water, surface.water.map(|w| w.0)));
                            }
                        }
                        if p.water.ceil() as i32 != water {
                            roundoff_columns += 1;
                            if cut || fill {
                                if actual.water != target_water {
                                    errors.push("actual_roundoff_water");
                                }
                                if surface.water.map(|w| w.0) != target_water {
                                    errors.push("far_roundoff_water");
                                }
                            }
                        }
                    } else if profile.is_some() {
                        errors.push("missing_native_or_far_surface");
                    }
                    if !errors.is_empty() {
                        failure_count += 1;
                        if failures.len() < 64 {
                            failures.push(
                                json!({"pos": [pos.0,pos.1], "errors": errors, "mode": mode}),
                            );
                        }
                    }
                    rows.push(json!({
                        "pos": [pos.0,pos.1], "mode": mode,
                        "native_actual": column_json(old_actual), "actual": column_json(actual),
                        "native_far": old_far.map(|s| column_json(Column { ground:s.height, water:s.water.map(|w|w.0) })),
                        "far": far.map(|s| column_json(Column { ground:s.height, water:s.water.map(|w|w.0) })),
                        "expected": expected.map(column_json),
                        "profile": profile.map(|p| json!({"raw":p.raw,"bed":p.bed,"water":p.water,"strength":p.strength,"lake":p.lake})),
                        "errors": errors
                    }));
                }
            }
        }
    }
    let mut lake_neighbor_pairs = 0;
    let mut broken_lake_pairs = 0;
    for (&(x, z), &(water, actual, far)) in &lake_cells {
        for next in [(x + 1, z), (x, z + 1)] {
            if let Some(&(nw, na, nf)) = lake_cells.get(&next)
                && nw == water
            {
                lake_neighbor_pairs += 1;
                if actual != Some(water)
                    || na != Some(water)
                    || far != Some(water)
                    || nf != Some(water)
                {
                    broken_lake_pairs += 1;
                }
            }
        }
    }
    let coverage = cut_columns > 0
        && dry_cut_columns > 0
        && (seed != 42 || (lake_fill_columns > 0 && lake_neighbor_pairs > 0));
    let passed = failure_count == 0 && broken_lake_pairs == 0 && coverage;
    fs::write(
        report,
        serde_json::to_vec_pretty(&json!({
            "passed":passed, "seed":seed, "chunks":64, "columns":rows.len(),
            "cut_columns":cut_columns, "dry_cut_columns":dry_cut_columns,
            "lake_fill_columns":lake_fill_columns, "preserved_water_columns":preserved_water_columns,
            "roundoff_columns":roundoff_columns, "native_ground_discrepancies":native_ground_discrepancies,
            "raw_ground_mismatches":raw_ground_mismatches, "raw_water_mismatches":raw_water_mismatches,
            "lake_neighbor_pairs":lake_neighbor_pairs, "broken_lake_pairs":broken_lake_pairs,
            "failure_count":failure_count, "coverage_passed":coverage, "failures":failures,
            "scope":"Every voxel column in64 native TERRAIN chunks; exact modified-bank parity and planned lake continuity. Unchanged carver/noise differences are reported separately. Not visual acceptance.",
            "seconds":start.elapsed().as_secs_f64(), "rows":rows
        }))?,
    )?;
    if !passed {
        return Err(format!("native bank/lake regression: {failure_count} columns, {broken_lake_pairs} lake edges, coverage={coverage}").into());
    }
    Ok(())
}
