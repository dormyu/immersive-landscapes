fn immersive_landscapes_ecotone_feature(
    world: &mut World<'_>,
    position: [i32; 3],
    feature: FeatureIdentity,
) -> std::ops::ControlFlow<Result<bool, PlacementQueryError>> {
    use crate::worldgen::{
        immersive_landscapes_ecotone_runtime as runtime, immersive_landscapes_ecotones as e,
    };
    let Some(atlas) = runtime::active() else {
        return std::ops::ControlFlow::Continue(());
    };
    let Some(pass) = atlas.ecotones.get() else {
        return std::ops::ControlFlow::Continue(());
    };
    let decoration = world.decoration;
    let Some(name) = decoration
        .names
        .get(feature.0 as usize)
        .filter(|n| e::vegetation(n))
    else {
        return std::ops::ControlFlow::Continue(());
    };
    if !crate::worldgen::decoration::immersive_landscapes_ecotone_plan::allowed_root(
        &decoration.plan,
        feature,
    ) {
        return std::ops::ControlFlow::Continue(());
    }
    let biome = match world.biome_identity(position) {
        Ok(b) => b,
        Err(e) => return std::ops::ControlFlow::Break(Err(e)),
    };
    let Some(blend) = pass.blend(&atlas, position[0], position[2], biome) else {
        return std::ops::ControlFlow::Continue(());
    };
    if !decoration
        .features
        .get(feature.0 as usize)
        .is_some_and(|f| f.as_ref().is_ok_and(runtime::safe_vegetation_definition))
    {
        return std::ops::ControlFlow::Continue(());
    }
    let members = pass.ids.map(|id| {
        id.and_then(|id| decoration.biome_features.get(id.0 as usize))
            .is_some_and(|set| set.contains(&feature))
    });
    std::ops::ControlFlow::Break(Ok(blend.admits(
        atlas.seed,
        position[0],
        position[2],
        name,
        members,
    )))
}
