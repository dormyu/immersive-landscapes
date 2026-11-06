// Integration evidence against the native generator, not a terrain preview renderer.
use horizon_core::position::ChunkPos;
use horizon_world::worldgen::immersive_landscapes_hydrology as hydro;
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
    let atlas = hydro::live_atlases()
        .into_iter()
        .find(|a| a.seed == seed)
        .ok_or("native hydrology did not bind")?;

    let mut requests: BTreeMap<(i32,i32), BTreeMap<(i32,i32), (i32,f64,bool)>> = BTreeMap::new();
    let mut missing = 0;
    let mut segments = 0;
    let mut showcase = Vec::new();
    for z in -2..=2 { for x in -2..=2 {
        let Some(b) = atlas.inspect_coast(x,z) else { continue; };
        if let Some(s)=b.segments.iter().filter(|s|s.cells>=16. && s.water[0]>=63. && s.water[0]<130.
            && (s.water[0]-s.water[1]).abs()/(s.to[0]-s.from[0]).hypot(s.to[1]-s.from[1]).max(0.01)<0.15)
            .max_by(|a,b|a.cells.total_cmp(&b.cells)) {
            showcase.push(json!({"pos":[(s.from[0]+s.to[0])*0.5,(s.from[1]+s.to[1])*0.5],
                "water":(s.water[0]+s.water[1])*0.5,"direction":[s.to[0]-s.from[0],s.to[1]-s.from[1]],"cells":s.cells,
                "config":{"amplitude":b.config.meander_amplitude,"wavelength":b.config.meander_scale,"width":b.config.width_scale,"depth":b.config.depth_scale,"banks":b.config.bank_scale}}));
        }
        for s in b.segments.iter().filter(|s|s.cells >= 16.) {
            segments += 1;
            let n = ((s.to[0]-s.from[0]).hypot(s.to[1]-s.from[1])/4.).ceil() as usize;
            for i in 0..=n {
                let t=i as f64/n as f64;
                let p=[(s.from[0]*(1.-t)+s.to[0]*t).round() as i32,
                    (s.from[1]*(1.-t)+s.to[1]*t).round() as i32];
                let Some(plan)=atlas.profile_at(p[0],p[1]) else { missing+=1;continue; };
                requests.entry((p[0]>>4,p[1]>>4)).or_default().insert((p[0],p[1]),(plan.voxel_water(true),plan.strength,plan.lake));
            }
        }
    }}
    let mut total=0; let mut wet=0; let mut gaps=Vec::new(); let mut chunks=0;
    for ((cx,cz), columns) in requests {
        let proto=worker.generate_proto(ChunkPos::new(cx,cz))?; chunks+=1;
        for ((x,z),(water,strength,lake)) in columns {
            total+=1;
            let top=water;
            let state=proto.block_name(x&15,top-1,z&15);
            if matches!(state,"minecraft:water"|"minecraft:ice"|"minecraft:packed_ice") {wet+=1;}
            else if gaps.len()<80 {
                let far=worker.surface_column(x,z).0.map(|s|json!({"height":s.height,"water":s.water.map(|w|w.0)}));
                gaps.push(json!({"pos":[x,z],"water":water,"strength":strength,"lake":lake,
                    "block":state,"actual_top":proto.surface_height(x&15,z&15),"far":far}));
            }
        }
        if chunks%25==0 { println!("routes: {chunks} chunks, {wet}/{total} wet centreline points"); }
    }
    let result=json!({"showcase":showcase,"dem_cache":atlas.cache_counts(),"seed":seed,"segments":segments,"chunks":chunks,"points":total,"wet":wet,"dry":total-wet,
        "missing_profile":missing,"examples":gaps,"seconds":start.elapsed().as_secs_f64(),
        "scope":"TERRAIN-status river centreline diagnostic; not visual, decoration, or frame performance acceptance"});
    fs::write(report,serde_json::to_vec_pretty(&result)?)?;
    println!("routes finished: {wet}/{total} wet, {missing} missing profiles");
    Ok(())
}
