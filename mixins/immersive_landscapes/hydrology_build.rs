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
    // Invalidate raw samples when moving this adapter to another engine revision.
    "immersive-landscapes/native-dem-v1/f42ef42ef6ceccb3c6a20bea2481d87949a51c81"
        .hash(&mut identity);
    for (key, value) in &data.densities {
        if key != "immersive_landscapes:watershed_runtime" {
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
    let scope = h::begin_build(token, seed, identity.finish());
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
