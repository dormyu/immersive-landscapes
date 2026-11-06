// Injected as a child of worldgen::decoration, so native scheduling data stays
// private. No feature order is rebuilt and no biome registry is modified.
use super::*;

fn vegetation_name(name: &str, expanded: bool) -> bool {
    if expanded {
        crate::worldgen::immersive_landscapes_ecotone_groups::vegetation(name)
    } else {
        crate::worldgen::immersive_landscapes_ecotones::vegetation(name)
    }
}

pub(crate) fn add_aliases(
    plan: &mut FeatureDecorationPlan,
    ids: [Option<DecorationBiome>; crate::worldgen::immersive_landscapes_ecotone_groups::COUNT],
    names: &[String],
    valid: &[bool],
    expanded: bool,
) {
    use crate::worldgen::immersive_landscapes_ecotone_runtime::ALIASES;
    for (index, source) in ids.into_iter().enumerate() {
        let alias = DecorationBiome(ALIASES[index]);
        // Graceful collision/missing-resource behavior: do not override anything.
        if plan.biomes.contains_key(&alias) {
            continue;
        }
        let Some(source) = source.and_then(|id| plan.biomes.get(&id)) else {
            continue;
        };
        let filtered: Vec<Vec<_>> = source
            .iter()
            .enumerate()
            .map(|(step, roots)| {
                if step != 9 {
                    return Vec::new();
                }
                roots
                    .iter()
                    .copied()
                    .filter(|f| {
                        names
                            .get(f.0 as usize)
                            .is_some_and(|n| vegetation_name(n, expanded))
                            && valid.get(f.0 as usize) == Some(&true)
                    })
                    .collect()
            })
            .collect();
        if filtered.iter().all(Vec::is_empty) {
            continue;
        }
        // plan.steps contains native step-local indices used by set_feature_seed.
        // Only this lookup map gains aliases; the order and seeds are untouched.
        debug_assert!(filtered.iter().flatten().all(|f| plan.seed(*f).is_some()));
        plan.biomes.insert(alias, filtered);
    }
}

pub(crate) fn allowed_root(plan: &FeatureDecorationPlan, feature: FeatureIdentity) -> bool {
    crate::worldgen::immersive_landscapes_ecotone_runtime::ALIASES
        .iter()
        .any(|id| {
            plan.biomes
                .get(&DecorationBiome(*id))
                .and_then(|steps| steps.get(9))
                .is_some_and(|roots| roots.contains(&feature))
        })
}

pub(crate) fn safe_alias(
    plan: &FeatureDecorationPlan,
    alias: DecorationBiome,
    names: &[String],
    features: &[Result<crate::worldgen::feature_graph::PlacedFeatureDefinition, String>],
    biome_count: usize,
    expanded: bool,
) -> bool {
    use crate::worldgen::immersive_landscapes_ecotone_runtime::ALIASES;
    if !ALIASES.contains(&alias.0) || (alias.0 as usize) < biome_count {
        return false;
    }
    plan.biomes.get(&alias).is_some_and(|steps| {
        steps.iter().enumerate().all(|(step, roots)| {
            roots.is_empty()
                || step == 9
                    && roots.iter().all(|f| {
                        names
                            .get(f.0 as usize)
                            .is_some_and(|n| vegetation_name(n, expanded))
                            && features.get(f.0 as usize).is_some_and(|f| {
                                f.as_ref().is_ok_and(|f| {
                                    crate::worldgen::immersive_landscapes_ecotone_runtime::safe_vegetation_definition_for(f, expanded)
                                })
                            })
                    })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aliases_keep_native_order_seeds_and_exclude_nonvegetation() {
        let mut roots = vec![Vec::new(); 10];
        roots[6] = vec![FeatureIdentity(0)];
        roots[9] = vec![FeatureIdentity(1), FeatureIdentity(2)];
        let mut plan = FeatureDecorationPlan::new(vec![(DecorationBiome(5), roots)]).unwrap();
        let before_steps = format!("{:?}", plan.steps);
        let before_seeds: Vec<_> = (0..3).map(|i| plan.seed(FeatureIdentity(i))).collect();
        let names = [
            "minecraft:ore_emerald",
            "minecraft:trees_taiga",
            "custom:grass",
        ]
        .map(str::to_owned);
        let mut ids = [None; crate::worldgen::immersive_landscapes_ecotone_groups::COUNT];
        ids[2] = Some(DecorationBiome(5));
        add_aliases(&mut plan, ids, &names, &[true; 3], false);
        assert_eq!(format!("{:?}", plan.steps), before_steps);
        assert_eq!(
            (0..3)
                .map(|i| plan.seed(FeatureIdentity(i)))
                .collect::<Vec<_>>(),
            before_seeds
        );
        let alias =
            DecorationBiome(crate::worldgen::immersive_landscapes_ecotone_runtime::ALIASES[2]);
        assert!(plan.biomes[&alias][6].is_empty());
        assert_eq!(plan.biomes[&alias][9], vec![FeatureIdentity(1)]);
        let snapshot = format!("{:?}", plan);
        add_aliases(&mut plan, ids, &names, &[false; 3], false);
        assert_eq!(
            format!("{:?}", plan),
            snapshot,
            "alias collisions are never overwritten"
        );
    }

    #[test]
    fn expanded_aliases_only_add_valid_roots_and_keep_native_feature_seeds() {
        let mut roots = vec![Vec::new(); 10];
        roots[6] = vec![FeatureIdentity(0)];
        roots[9] = vec![FeatureIdentity(1), FeatureIdentity(2), FeatureIdentity(3)];
        let mut plan = FeatureDecorationPlan::new(vec![(DecorationBiome(7), roots)]).unwrap();
        let steps = format!("{:?}", plan.steps);
        let seeds: Vec<_> = (0..4).map(|i| plan.seed(FeatureIdentity(i))).collect();
        let names = [
            "minecraft:ore_emerald",
            "minecraft:dark_forest_vegetation",
            "minecraft:patch_leaf_litter",
            "custom:trees",
        ]
        .map(str::to_owned);
        let mut ids = [None; crate::worldgen::immersive_landscapes_ecotone_groups::COUNT];
        ids[6] = Some(DecorationBiome(7));
        // Missing biome ids and rejected/custom executable graphs do not leak
        // into a donor alias, even when their registry name looks familiar.
        add_aliases(&mut plan, ids, &names, &[true, true, false, true], true);
        let alias =
            DecorationBiome(crate::worldgen::immersive_landscapes_ecotone_runtime::ALIASES[6]);
        assert_eq!(plan.biomes[&alias][9], vec![FeatureIdentity(1)]);
        assert!(plan.biomes[&alias][6].is_empty());
        assert_eq!(format!("{:?}", plan.steps), steps);
        assert_eq!(
            (0..4)
                .map(|i| plan.seed(FeatureIdentity(i)))
                .collect::<Vec<_>>(),
            seeds
        );
    }
}
