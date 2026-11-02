fn immersive_landscapes_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "immersive_landscapes".into(),
        title: format!("Immersive Landscapes {}", crate::game::immersive_landscapes::VERSION),
        description: "Mountain ranges, connected rivers and continuous climates. By dormyu. New normal worlds use this generator; existing worlds retain theirs.".into(),
        keys: Vec::new(),
    });
}
