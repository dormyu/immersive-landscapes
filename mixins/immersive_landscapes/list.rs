fn immersive_landscapes_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "immersive_landscapes".into(),
        title: format!("Immersive Landscapes {}", crate::game::immersive_landscapes::VERSION),
        description: "Minecraft-scale landscapes with varied biomes and full native decoration. Integration and tuning by dormyu; terrain based on Epic Terrain by wonderfulaichen (MIT). Applies to new default worlds.".into(),
        keys: Vec::new(),
    });
}
