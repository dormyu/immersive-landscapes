fn immersive_landscapes_shape(
    root: &std::path::Path,
) -> std::ops::ControlFlow<horizon_client::chunk::LevelShapes> {
    if crate::game::immersive_landscapes::is_world(root) {
        let mut shapes = horizon_client::chunk::LevelShapes::VANILLA;
        shapes.overworld = crate::game::immersive_landscapes::shape();
        std::ops::ControlFlow::Break(shapes)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
