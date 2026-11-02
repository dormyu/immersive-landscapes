fn immersive_landscapes_list(
    game: &crate::game::Game<'_>,
    mods: &mut Vec<crate::game::native_hooks::NativeMod>,
) {
    horizon_client_render::immersive_landscapes_ui::sync(&game.settings.mods);
    mods.push(crate::game::native_hooks::NativeMod {
        id: "immersive_landscapes".into(),
        title: format!("Immersive Landscapes {}", crate::game::immersive_landscapes::VERSION),
        description: "Minecraft-scale landscapes with varied biomes and full native decoration. Select Immersive Landscapes when creating a world. Integration and tuning by dormyu; terrain based on Epic Terrain by wonderfulaichen (MIT).".into(),
        keys: Vec::new(),
    });
}
