// Integration evidence against the native generator, not a terrain preview renderer.
use horizon_core::position::ChunkPos;
use horizon_world::worldgen::{
    generator::{GeneratorData, OverworldGenerator},
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

// A sea-level native aquifer in an enclosed inland depression is not an ocean.
// Only use this bounded connectivity check for a raised candidate, and treat an
// open search boundary conservatively as receiving sea rather than excluding it.
fn closed_native_pool(worker: &mut horizon_world::worldgen::generator::GeneratorWorker, origin: (i32,i32)) -> Result<bool, Error> {
    use std::collections::{HashMap,HashSet,VecDeque};
    let mut chunks=HashMap::new();let mut seen=HashSet::new();let mut queue=VecDeque::from([origin]);
    while let Some((x,z))=queue.pop_front() {
        if !seen.insert((x,z)) {continue;}
        if (x-origin.0).abs()>128 || (z-origin.1).abs()>128 {return Ok(false);}
        let key=(x>>4,z>>4);
        if !chunks.contains_key(&key) { chunks.insert(key,worker.generate_proto(ChunkPos::new(key.0,key.1))?); }
        let c=chunks.get(&key).unwrap();
        if c.surface_height(x&15,z&15)!=63 || c.block_name(x&15,62,z&15)!="minecraft:water" {continue;}
        if c.biome_at(x&15,62,z&15).contains("ocean") {return Ok(false);}
        for p in [(x+1,z),(x-1,z),(x,z+1),(x,z-1)] {queue.push_back(p);}
    }
    Ok(true)
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


    let min_x: i32=args[5].parse()?;let max_x: i32=args[6].parse()?;
    let min_z: i32=args[7].parse()?;let max_z: i32=args[8].parse()?;
    let mut ocean=0;let mut raised=0;let mut far_raised=0;let mut examples=Vec::new();
    let mut enclosed_inland_fills=Vec::new();
    for cz in min_z..=max_z {for cx in min_x..=max_x {
        let old=plain_worker.generate_proto(ChunkPos::new(cx,cz))?;
        let now=worker.generate_proto(ChunkPos::new(cx,cz))?;
        for z in (0..16).step_by(2) {for x in (0..16).step_by(2) {
            if old.surface_height(x,z)!=63 || old.block_name(x,62,z)!="minecraft:water" || old.block_name(x,63,z)!="minecraft:air" {continue;}
            ocean+=1;let pos=[cx*16+x,cz*16+z];
            let above=(63..now.surface_height(x,z).min(320)).any(|y|now.block_name(x,y,z)=="minecraft:water");
            let surface=worker.surface_column(pos[0],pos[1]).0;
            let far_water=surface.and_then(|s|s.water.map(|w|w.0));
            if (above || far_water.is_some_and(|w|w>63)) && closed_native_pool(&mut plain_worker,(pos[0],pos[1]))? {
                ocean-=1;
                enclosed_inland_fills.push(json!({"pos":pos,"far_water":far_water}));
                continue;
            }
            if above {raised+=1;}
            if far_water.is_some_and(|w|w>63) {far_raised+=1;}
            if (above || far_water.is_some_and(|w|w>63)) && examples.len()<24 {
                examples.push(json!({"pos":pos,"native_water_above_sea":above,"far_water":far_water}));
            }
        }}
    } if cz%8==0 {println!("ocean row{cz}: {ocean} checked/{raised} raised/{far_raised} far-raised");}}
    let passed=ocean>100 && raised==0 && far_raised==0;
    let result=json!({"passed":passed,"seed":seed,"ocean_columns_checked":ocean,
        "raised_ocean_columns":raised,"raised_far_columns":far_raised,"examples":examples,"enclosed_inland_fills":enclosed_inland_fills,
        "seconds":start.elapsed().as_secs_f64(),"scope":"native TERRAIN blocks and far columns at actual receiving sea-level water; no render/performance claim"});
    fs::write(report,serde_json::to_vec_pretty(&result)?)?;
    assert!(passed,"elevated coastal water remained: {result}");
    Ok(())
}
