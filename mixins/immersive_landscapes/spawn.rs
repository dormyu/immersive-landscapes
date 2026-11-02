fn immersive_landscapes_spawn(
    game: &crate::game::Game<'_>,
) -> std::ops::ControlFlow<Result<((i32, i32), i32)>> {
    match crate::game::immersive_landscapes::spawn_origin(&game.world_root) {
        Some(origin) => std::ops::ControlFlow::Break(Ok(origin)),
        None => std::ops::ControlFlow::Continue(()),
    }
}
