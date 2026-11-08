fn immersive_landscapes_ecotone_plan_build(
    data: &std::sync::Arc<GeneratorData>,
    source: &ResolvedBiomeSource,
    with_features: bool,
) -> std::ops::ControlFlow<Result<Option<Decoration>, GeneratorError>> {
    use crate::worldgen::immersive_landscapes_ecotone_runtime as e;
    if crate::worldgen::generator::immersive_landscapes_ecotone_settings::width(data).is_none() {
        return std::ops::ControlFlow::Continue(());
    }
    let key = std::sync::Arc::as_ptr(data) as usize;
    if e::reentry(e::ScopeKind::Plan, key) {
        return std::ops::ControlFlow::Continue(());
    }
    let _scope = e::enter(e::ScopeKind::Plan, key, None);
    let result = Decoration::new_with(data, source, with_features);
    std::ops::ControlFlow::Break(result.map(|decoration| {
        decoration.map(|mut decoration| {
            let expanded =
                crate::worldgen::generator::immersive_landscapes_ecotone_settings::expanded(data);
            let regional =
                crate::worldgen::generator::immersive_landscapes_ecotone_settings::regional(data);
            let mut ids = crate::worldgen::immersive_landscapes_ecotone_regions::NAMES
                .map(|name| data.biome_id(name));
            if !regional {
                ids[if expanded {
                    crate::worldgen::immersive_landscapes_ecotone_groups::COUNT
                } else {
                    4
                }..]
                    .fill(None);
            }
            let valid: Vec<_> = decoration
                .features
                .iter()
                .map(|f| {
                    f.as_ref().is_ok_and(|f| {
                        e::safe_vegetation_definition_with_regions(
                            f,
                            expanded || regional,
                            regional,
                        )
                    })
                })
                .collect();
            crate::worldgen::decoration::immersive_landscapes_ecotone_plan::add_aliases(
                &mut decoration.plan,
                ids,
                &decoration.names,
                &valid,
                expanded,
                regional,
            );
            decoration
        })
    }))
}
