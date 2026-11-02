fn immersive_landscapes_world_type(key: &str) -> std::ops::ControlFlow<String> {
    if key == "generator.minecraft.normal"
        && horizon_client_render::immersive_landscapes_ui::enabled()
    {
        return std::ops::ControlFlow::Break(
            horizon_client_render::immersive_landscapes_ui::NAME.into(),
        );
    }
    std::ops::ControlFlow::Continue(())
}
