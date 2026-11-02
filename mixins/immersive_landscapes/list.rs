fn immersive_landscapes_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "immersive_landscapes".into(),
        title: "Immersive Landscapes 0.1.0-dev.1".into(),
        description: "Mountain ranges, connected rivers and continuous climates. By dormyu. New normal worlds use this generator; existing worlds retain theirs.".into(),
        keys: Vec::new(),
    });
}
