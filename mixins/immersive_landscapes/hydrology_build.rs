fn immersive_landscapes_hydrology_build(
    data: &std::sync::Arc<GeneratorData>,
    seed: i64,
    preset: &str,
    dimension: &str,
    structures: bool,
    stem: Option<&Value>,
) -> std::ops::ControlFlow<Result<OverworldGenerator, GeneratorError>> {
    use crate::worldgen::immersive_landscapes_hydrology as h;
    if dimension != "minecraft:overworld"
        || !data
            .densities
            .contains_key("immersive_landscapes:watershed_v1")
    {
        return std::ops::ControlFlow::Continue(());
    }
    let token = std::sync::Arc::as_ptr(data) as usize;
    if h::build_reentry(token) {
        return std::ops::ControlFlow::Continue(());
    }
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut identity = DefaultHasher::new();
    let natural = data
        .densities
        .get("immersive_landscapes:watershed_v1")
        .and_then(|s| serde_json::from_str::<f64>(s).ok())
        .is_some_and(|v| v >= 2.0);
    let mut config = crate::worldgen::immersive_landscapes_watershed::Config {
        natural,
        ..Default::default()
    };
    if natural {
        let read = |name: &str, default: f64, min: f64, max: f64| {
            data.densities
                .get(&format!("immersive_landscapes:river_{name}"))
                .and_then(|s| serde_json::from_str::<f64>(s).ok())
                .filter(|v| v.is_finite())
                .map_or(default, |v| v.clamp(min, max))
        };
        config.meander_amplitude = read("meander_amplitude", 14., 0., 32.);
        config.meander_scale = read("meander_scale", 96., 48., 256.);
        config.width_scale = read("width_scale", 1., 0.75, 1.75);
        config.depth_scale = read("depth_scale", 1., 0.5, 1.5);
        config.bank_scale = read("bank_scale", 3., 2., 5.);
    }
    // Invalidate raw samples when moving this adapter to another engine revision.
    "immersive-landscapes/native-dem-v1/f42ef42ef6ceccb3c6a20bea2481d87949a51c81"
        .hash(&mut identity);
    if natural {
        "density-surface-v2".hash(&mut identity);
    }
    for (key, value) in &data.densities {
        if key != "immersive_landscapes:watershed_runtime"
            && !key.starts_with("immersive_landscapes:river_")
        {
            (key, value).hash(&mut identity);
        }
    }
    data.noises.hash(&mut identity);
    data.settings.hash(&mut identity);
    data.dimensions.hash(&mut identity);
    data.dimension_types.hash(&mut identity);
    data.world_presets.hash(&mut identity);
    preset.hash(&mut identity);
    dimension.hash(&mut identity);
    stem.map(ToString::to_string).hash(&mut identity);
    let scope = h::begin_build(token, seed, identity.finish(), config);
    let result = OverworldGenerator::for_world_with_settings(
        data.clone(),
        seed,
        preset,
        dimension,
        structures,
        stem,
    );
    if let Ok(generator) = &result {
        h::bind_pipeline(std::sync::Arc::as_ptr(&generator.pipeline) as *const () as usize);
    }
    drop(scope);
    std::ops::ControlFlow::Break(result)
}
