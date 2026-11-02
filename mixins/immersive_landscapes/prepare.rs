fn immersive_landscapes_prepare(
    request: &crate::game::world_loading::WorldRequest,
    settings: &crate::game::Settings,
) -> std::ops::ControlFlow<Result<PreparedWorld>> {
    match crate::game::immersive_landscapes::prepare(request, settings) {
        Ok(()) => std::ops::ControlFlow::Continue(()),
        Err(error) => std::ops::ControlFlow::Break(Err(error)),
    }
}
