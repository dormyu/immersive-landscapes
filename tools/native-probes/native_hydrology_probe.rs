// Integration evidence against the native generator, not a terrain preview renderer.
use horizon_core::position::ChunkPos;
use horizon_world::worldgen::immersive_landscapes_hydrology as hydro;
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

fn geometry_class(name: &str) -> u8 {
    match name {
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" => 0,
        "minecraft:water" => 1,
        "minecraft:lava" => 2,
        _ => 3,
    }
}

fn solid_ground(proto: &ProtoChunk, x: i32, z: i32) -> i32 {
    let min = proto.terrain.min_y();
    let max = min + proto.terrain.height();
    (min..max)
        .rev()
        .find(|&y| {
            !matches!(
                proto.block_name(x, y, z),
                "minecraft:air"
                    | "minecraft:cave_air"
                    | "minecraft:void_air"
                    | "minecraft:water"
                    | "minecraft:lava"
            )
        })
        .map_or(min, |y| y + 1)
}

fn parallel_points(
    mut worker: horizon_world::worldgen::generator::GeneratorWorker,
    jobs: Vec<([i32; 2], i32)>,
) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for (p, water) in jobs {
        let proto = worker
            .generate_proto(ChunkPos::new(p[0] >> 4, p[1] >> 4))
            .map_err(|e| e.to_string())?;
        out.push(
            json!({"pos":p,"height":proto.surface_height(p[0]&15,p[1]&15),
            "top":proto.block_name(p[0]&15,water-1,p[1]&15)}),
        );
    }
    out.sort_by_key(|p| (p["pos"][0].as_i64(), p["pos"][1].as_i64()));
    Ok(out)
}

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

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
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
    let mut timings = [0.; 4];
    let atlas = hydro::live_atlases()
        .into_iter()
        .find(|a| a.seed == seed)
        .ok_or("native hydrology did not bind")?;
    let mut basins = Vec::new();
    let mut showcase=Vec::new();
    let mut points = Vec::new();
    for z in -4..=4 {
        for x in -4..=4 {
            let before = Instant::now();
            let Some(b) = atlas.inspect_coast(x, z) else {
                continue;
            };
            assert!((b.input - b.output).abs() < 1e-6, "runoff mismatch");
            let mouths = b.segments.iter().filter(|s| s.mouth).count();
            println!(
                "coast{x},{z}: {} segments/{mouths} mouths; {:.2}s",
                b.segments.len(),
                before.elapsed().as_secs_f64()
            );
            basins.push(json!({"key":[x,z],"coast":[b.coast.x,b.coast.z],"segments":b.segments.len(),
            "mouths":mouths,"input":b.input,"output":b.output,"seconds":before.elapsed().as_secs_f64()}));
            if let Some(s)=b.segments.iter().filter(|s|s.cells>=16. && s.water[0]>=64. && s.water[0]<130.
                && (s.water[0]-s.water[1]).abs()/(s.to[0]-s.from[0]).hypot(s.to[1]-s.from[1]).max(0.01)<0.15)
                .max_by(|a,b|a.cells.total_cmp(&b.cells)) {
                showcase.push(json!({"pos":[(s.from[0]+s.to[0])*0.5,(s.from[1]+s.to[1])*0.5],
                    "water":(s.water[0]+s.water[1])*0.5,"direction":[s.to[0]-s.from[0],s.to[1]-s.from[1]],"cells":s.cells,
                    "config":{"amplitude":b.config.meander_amplitude,"wavelength":b.config.meander_scale,"width":b.config.width_scale,"depth":b.config.depth_scale,"banks":b.config.bank_scale}}));
            }
            let mut in_basin = 0;
            for s in b
                .segments
                .iter()
                .filter(|s| s.water[0] > 65. && s.water[0] < 180.)
            {
                if in_basin >= 3 {
                    break;
                }
                let p = [
                    ((s.from[0] + s.to[0]) * 0.5).round() as i32,
                    ((s.from[1] + s.to[1]) * 0.5).round() as i32,
                ];
                let Some(profile) = atlas.profile_at(p[0], p[1]) else {
                    continue;
                };
                if profile.strength < 0.8 || profile.lake {
                    continue;
                }
                if points.iter().any(|point: &Value| point["pos"] == json!(p)) {
                    continue;
                }
                let t = Instant::now();
                let column = worker.surface_column(p[0], p[1]);
                timings[0] += t.elapsed().as_secs_f64();
                let t = Instant::now();
                let grid = worker.lod_grid(p[0], p[1], 1, 1, false)?.remove(0);
                timings[1] += t.elapsed().as_secs_f64();
                let t = Instant::now();
                let proto = worker.generate_proto(ChunkPos::new(p[0] >> 4, p[1] >> 4))?;
                timings[2] += t.elapsed().as_secs_f64();
                let t = Instant::now();
                let plain_proto =
                    plain_worker.generate_proto(ChunkPos::new(p[0] >> 4, p[1] >> 4))?;
                timings[3] += t.elapsed().as_secs_f64();
                let planned = profile.voxel_water(true);
                let original_surface = plain_worker.surface_column(p[0], p[1]).0.unwrap();
                let floor = profile.raster_floor(original_surface.height, true);
                let liner_required = floor < original_surface.height;
                let actual_ground = solid_ground(&proto, p[0] & 15, p[1] & 15);
                let native_ground = solid_ground(&plain_proto, p[0] & 15, p[1] & 15);
                let block = proto.block_name(p[0] & 15, planned - 1, p[1] & 15);
                let roof = (planned..320)
                    .filter(|&y| {
                        !matches!(
                            proto.block_name(p[0] & 15, y, p[1] & 15),
                            "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
                        )
                    })
                    .count();
                let spans_water = column.1.iter().any(|&(a, b, state)| {
                    a <= planned - 1
                        && b >= planned
                        && registry.state(state).unwrap().name == "minecraft:water"
                });
                let aligned = column.0.as_ref().and_then(|s| s.water).map(|w| w.0) == Some(planned)
                    && grid.0.as_ref().and_then(|s| s.water).map(|w| w.0) == Some(planned)
                    && proto.surface_height(p[0] & 15, p[1] & 15) == planned
                    && roof == 0
                    && spans_water
                    && column.1.windows(2).all(|r|r[1].1 <= r[0].0)
                    && grid.1.windows(2).all(|r|r[1].1 <= r[0].0);
                let liner = (floor - 4..floor - 1)
                    .all(|y| proto.block_name(p[0] & 15, y, p[1] & 15) == "minecraft:stone");
                // The plain reference checks geometry, but intentionally lacks
                // riparian biome changes. Scalar native LOD spans paint only the
                // top voxel, so their default-stone interior is not a palette
                // oracle. Retain material diagnostics; require exact top parity.
                // Only a real cut receives our stone liner.
                let native_materials_unchanged =
                    (proto.terrain.min_y()..native_ground).all(|y| {
                        proto.block(p[0] & 15, y, p[1] & 15)
                            == plain_proto.block(p[0] & 15, y, p[1] & 15)
                    });
                let span_state = |y| column.1.iter()
                    .find(|&&(a,b,_)| a<=y && y<b).map(|&(_,_,state)| state);
                let native_geometry_unchanged =
                    (proto.terrain.min_y()..native_ground).all(|y| {
                        geometry_class(proto.block_name(p[0]&15,y,p[1]&15))
                            == geometry_class(plain_proto.block_name(p[0]&15,y,p[1]&15))
                    });
                let changed_materials_match_native_spans =
                    (proto.terrain.min_y()..native_ground).all(|y| {
                        let actual = proto.block(p[0]&15,y,p[1]&15);
                        let original = plain_proto.block(p[0]&15,y,p[1]&15);
                        actual == original || (
                            geometry_class(&registry.state(actual).unwrap().name)==3
                            && geometry_class(&registry.state(original).unwrap().name)==3
                            && span_state(y)==Some(actual))
                    });
                let native_material_mismatches: Vec<_> =
                    (proto.terrain.min_y()..native_ground).filter_map(|y| {
                        let actual = proto.block(p[0]&15,y,p[1]&15);
                        let original = plain_proto.block(p[0]&15,y,p[1]&15);
                        (actual != original).then(|| json!({"y":y,
                            "actual_id":actual.0,"native_id":original.0,
                            "actual":registry.state(actual).unwrap().name,
                            "native":registry.state(original).unwrap().name,
                            "far_span_id":span_state(y).map(|s|s.0),
                            "far_span":span_state(y).map(|s|registry.state(s).unwrap().name.as_str()),
                            "actual_biome":proto.biome_at(p[0]&15,y,p[1]&15),
                            "native_biome":plain_proto.biome_at(p[0]&15,y,p[1]&15)}))
                    }).collect();
                let near_top = proto.block(p[0]&15,actual_ground-1,p[1]&15);
                let top_materials_aligned = span_state(floor-1)==Some(near_top)
                    && column.0.as_ref().map(|s|s.state)==Some(near_top)
                    && grid.0.as_ref().map(|s|s.state)==Some(near_top);
                let column_top = span_state(floor-1)
                    .map(|state|json!({"id":state.0,"name":registry.state(state).unwrap().name}));
                let liner_valid = if liner_required {
                    liner && actual_ground == floor
                } else {
                    actual_ground == native_ground && native_geometry_unchanged
                };
                let surface = column
                    .0
                    .map(|s| json!({"height":s.height,"water":s.water.map(|w|w.0),"state_id":s.state.0,
                        "state":registry.state(s.state).unwrap().name,"biome":proto.data().biome_name(s.biome)}));
                let far = grid
                    .0
                    .map(|s| json!({"height":s.height,"water":s.water.map(|w|w.0),"state_id":s.state.0,
                        "state":registry.state(s.state).unwrap().name,"biome":proto.data().biome_name(s.biome)}));
                points.push(json!({"pos":p,"plan_water":planned,"plan_bed":profile.bed,"surface":surface,
                "grid":far,"top_block":block,"proto_height":proto.surface_height(p[0]&15,p[1]&15),
                "solid_roof_blocks":roof,"water_span":spans_water,"aligned":aligned,"stone_liner":liner,
                "raster_floor":floor,"liner_required":liner_required,"liner_valid":liner_valid,
                "actual_ground":actual_ground,"native_ground":native_ground,
                "native_surface":{"height":original_surface.height,"water":original_surface.water.map(|w|w.0),
                    "state_id":original_surface.state.0,"state":registry.state(original_surface.state).unwrap().name,
                    "biome":plain_proto.data().biome_name(original_surface.biome)},
                "native_materials_unchanged":native_materials_unchanged,"native_material_mismatches":native_material_mismatches,
                "native_geometry_unchanged":native_geometry_unchanged,
                "changed_materials_match_native_spans":changed_materials_match_native_spans,
                "top_materials_aligned":top_materials_aligned,
                "near_top":{"id":near_top.0,"name":registry.state(near_top).unwrap().name},"column_top":column_top,
                "profile":{"raw":profile.raw,"bed":profile.bed,"water":profile.water,"lake":profile.lake,"strength":profile.strength}}));
                println!("  point {p:?}: water{planned}, actual{block}");
                in_basin += 1;
            }
            if points.len() >= 12 {
                break;
            }
        }
        if points.len() >= 12 {
            break;
        }
    }
    let wet = points
        .iter()
        .filter(|p| p["top_block"] == "minecraft:water")
        .count();
    let aligned = points
        .iter()
        .all(|p| p["aligned"] == true && p["liner_valid"] == true && p["top_materials_aligned"] == true);
    let jobs: Vec<_> = points
        .iter()
        .map(|p| {
            (
                [
                    p["pos"][0].as_i64().unwrap() as i32,
                    p["pos"][1].as_i64().unwrap() as i32,
                ],
                p["plan_water"].as_i64().unwrap() as i32,
            )
        })
        .collect();
    let a = generator.worker();
    let b = generator.worker();
    let mut reverse = jobs.clone();
    reverse.reverse();
    let parallel = std::thread::scope(|scope| {
        let left = scope.spawn(move || parallel_points(a, jobs));
        let right = scope.spawn(move || parallel_points(b, reverse));
        (
            left.join().expect("first native worker panicked"),
            right.join().expect("second native worker panicked"),
        )
    });
    let parallel_left = parallel.0?;
    let parallel_right = parallel.1?;
    let parallel_ok = parallel_left == parallel_right
        && parallel_left.iter().all(|p| {
            points.iter().any(|gold| {
                gold["pos"] == p["pos"]
                    && gold["proto_height"] == p["height"]
                    && gold["top_block"] == p["top"]
            })
        });
    let result = json!({"showcase":showcase,"seed":seed,"bound":true,"basins":basins,"points":points,"aligned":aligned,
        "dem_cache":atlas.cache_counts(), "timings_seconds_column_grid_proto_reference":timings,
        "two_native_workers_equal":parallel_ok,
        "native_wet_points":wet,"seconds":start.elapsed().as_secs_f64(),
        "scope":"native library integration only; exact modified floor/water/top-material parity, no-cut native geometry occupancy retained. Scalar native LOD spans approximate subsurface materials, so their lower-layer palette comparison is diagnostic only. No render/FPS/decoration acceptance"});
    fs::write(report, serde_json::to_vec_pretty(&result)?)?;
    assert!(
        wet >= 3,
        "not enough planned higher rivers became actual water; inspect report"
    );
    assert!(aligned, "near/far/water/liner disagreement; inspect report");
    assert!(
        parallel_ok,
        "concurrent native workers produced different block columns"
    );
    Ok(())
}
