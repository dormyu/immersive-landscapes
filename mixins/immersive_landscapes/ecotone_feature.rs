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
    let Some(name) = decoration.names.get(feature.0 as usize).filter(|n| {
        if pass.regional {
            crate::worldgen::immersive_landscapes_ecotone_regions::vegetation(n)
        } else if pass.expanded {
            crate::worldgen::immersive_landscapes_ecotone_groups::vegetation(n)
        } else {
            e::vegetation(n)
        }
    }) else {
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
    if blend.needs_dry_warm_surface() {
        // Read only the existing local placement origin, not more distant
        // columns. The native temperature includes altitude-dependent snow.
        let Ok(ground) =
            PlacementContext::block_state(world, [position[0], position[1] - 1, position[2]])
        else {
            return std::ops::ControlFlow::Continue(());
        };
        let Ok(above) = PlacementContext::block_state(world, position) else {
            return std::ops::ControlFlow::Continue(());
        };
        let Ok(sea_level) = world.sea_level() else {
            return std::ops::ControlFlow::Continue(());
        };
        let Some(temperature) = world.temperatures.get_mut(biome.0 as usize) else {
            return std::ops::ControlFlow::Continue(());
        };
        let can_snow = !temperature.warm_enough_to_rain(
            BlockPos::new(position[0], position[1], position[2]),
            sea_level,
        );
        let registry = decoration.bindings.registry();
        if !registry
            .state(ground)
            .zip(registry.state(above))
            .is_some_and(|(ground, above)| {
                crate::worldgen::immersive_landscapes_ecotone_groups::crossing_surface(
                    ground.name.as_str(),
                    above.name.as_str(),
                    can_snow,
                )
            })
        {
            return std::ops::ControlFlow::Continue(());
        }
    }
    if !decoration
        .features
        .get(feature.0 as usize)
        .is_some_and(|f| {
            f.as_ref().is_ok_and(|f| {
                runtime::safe_vegetation_definition_with_regions(
                    f,
                    pass.expanded || pass.regional,
                    pass.regional,
                )
            })
        })
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
