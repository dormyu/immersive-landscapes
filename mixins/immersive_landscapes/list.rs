fn immersive_landscapes_list(
    game: &crate::game::Game<'_>,
    mods: &mut Vec<crate::game::native_hooks::NativeMod>,
) {
    horizon_client_render::immersive_landscapes_ui::sync(&game.settings.mods);
    mods.push(crate::game::native_hooks::NativeMod {
        id: "immersive_landscapes".into(),
        title: format!("Immersive Landscapes {}", crate::game::immersive_landscapes::VERSION),
        description: "Vanilla Minecraft terrain with coherent mountain regions and graded river valleys. Vanilla biomes, vegetation, caves and structures. Select Immersive Landscapes when creating a world. Original terrain overlay and integration by dormyu.".into(),
        keys: Vec::new(),
    });
}
