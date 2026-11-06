fn immersive_landscapes_ecotone_schedule(
    chunk: ChunkPos,
    world: &mut World<'_>,
) -> std::ops::ControlFlow<Result<Vec<BiomeId>, RegionError>> {
    use crate::worldgen::immersive_landscapes_ecotone_runtime as e;
    let key = world as *mut World<'_> as usize;
    if e::reentry(e::ScopeKind::Biomes, key) {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(atlas) = e::active() else {
        return std::ops::ControlFlow::Continue(());
    };
    let Some(pass) = atlas.ecotones.get() else {
        return std::ops::ControlFlow::Continue(());
    };
    let aliases: Vec<_> = pass
        .aliases_for_chunk(&atlas, chunk.x, chunk.z)
        .into_iter()
        .filter(|id| {
            crate::worldgen::decoration::immersive_landscapes_ecotone_plan::safe_alias(
                &world.decoration.plan,
                *id,
                &world.decoration.names,
                &world.decoration.features,
                world.biome_names.len(),
            )
        })
        .collect();
    if aliases.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let _scope = e::enter(e::ScopeKind::Biomes, key, Some(atlas));
    std::ops::ControlFlow::Break(world.chunk_biomes(chunk).map(|mut ids| {
        ids.extend(aliases);
        ids
    }))
}
