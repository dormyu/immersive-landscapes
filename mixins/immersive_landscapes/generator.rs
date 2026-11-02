fn immersive_landscapes_generator(
    res: &crate::game::Resources,
    pack: &crate::game::PackResources,
    options: &crate::game::Options,
    settings: &crate::game::Settings,
    root: &std::path::Path,
    dim: crate::game::Dim,
    shape: horizon_client::chunk::LevelShape,
    seed: i64,
    features: &[String],
) -> std::ops::ControlFlow<Result<(Streaming, Arc<SaveDir>, Option<Arc<VanillaTerrain>>)>> {
    if dim == crate::game::Dim::Overworld && crate::game::immersive_landscapes::is_world(root) {
        std::ops::ControlFlow::Break(crate::game::immersive_landscapes::streaming(
            res, pack, options, settings, root, shape, seed, features,
        ))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
