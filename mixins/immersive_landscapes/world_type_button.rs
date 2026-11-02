fn immersive_landscapes_world_type_button(key: &str, value: &str) -> std::ops::ControlFlow<String> {
    // The 150px cycle button must fit the full name even in Russian. The World
    // tab supplies the context; omit only this preset's redundant type prefix.
    if key == "selectWorld.mapType"
        && value == horizon_client_render::immersive_landscapes_ui::NAME
        && horizon_client_render::immersive_landscapes_ui::enabled()
    {
        return std::ops::ControlFlow::Break(value.to_owned());
    }
    std::ops::ControlFlow::Continue(())
}
