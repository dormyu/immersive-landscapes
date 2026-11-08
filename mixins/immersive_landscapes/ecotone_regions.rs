// Original climate-region ecotones. MIT, dormyu.
// Only ecotones_v3=1 uses this policy. v1/v2 kernels and admission stay intact.
use super::{
    immersive_landscapes_ecotone_groups as previous, immersive_landscapes_ecotones as legacy,
};

pub const COUNT: usize = 38;
pub const NAMES: [&str; COUNT] = [
    "minecraft:forest",
    "minecraft:plains",
    "minecraft:taiga",
    "minecraft:meadow",
    "minecraft:birch_forest",
    "minecraft:flower_forest",
    "minecraft:dark_forest",
    "minecraft:sunflower_plains",
    "minecraft:old_growth_pine_taiga",
    "minecraft:old_growth_spruce_taiga",
    "minecraft:dappled_forest",
    "minecraft:cherry_grove",
    "minecraft:pale_garden",
    "minecraft:snowy_taiga",
    "minecraft:snowy_plains",
    "minecraft:grove",
    "minecraft:snowy_slopes",
    "minecraft:jagged_peaks",
    "minecraft:frozen_peaks",
    "minecraft:stony_peaks",
    "minecraft:windswept_hills",
    "minecraft:windswept_gravelly_hills",
    "minecraft:windswept_forest",
    "minecraft:savanna",
    "minecraft:savanna_plateau",
    "minecraft:windswept_savanna",
    "minecraft:desert",
    "minecraft:badlands",
    "minecraft:wooded_badlands",
    "minecraft:eroded_badlands",
    "minecraft:jungle",
    "minecraft:sparse_jungle",
    "minecraft:bamboo_jungle",
    "minecraft:swamp",
    "minecraft:mangrove_swamp",
    "minecraft:mushroom_fields",
    "minecraft:ice_spikes",
    "minecraft:old_growth_birch_forest",
];
// Explicit local climate neighborhoods, not a transitive graph. Pale gardens
// and mushroom islands deliberately keep their special vegetation/identity.
pub const FAMILIES: [&[usize]; 6] = [
    &[0, 1, 3, 4, 5, 6, 7, 10, 11, 37],
    &[0, 1, 2, 3, 4, 7, 8, 9, 19, 20, 21, 22, 37],
    &[2, 8, 9, 13, 14, 15, 16, 17, 18, 20, 21, 22, 36],
    &[23, 24, 25, 26, 27, 28, 29],
    &[1, 23, 24, 25, 30, 31, 32],
    &[0, 4, 30, 31, 33, 34],
];
pub type Weights = [f64; COUNT];
pub fn enabled(marker: Option<f64>) -> bool {
    marker == Some(1.)
}
pub fn named(name: &str) -> Option<usize> {
    NAMES.iter().position(|n| *n == name)
}

pub fn vegetation(name: &str) -> bool {
    previous::vegetation(name)
        || matches!(
            name,
            "minecraft:trees_dappled_forest"
                | "minecraft:brown_mushroom_dappled_forest"
                | "minecraft:patch_red_shrub"
                | "minecraft:trees_cherry"
                | "minecraft:flower_cherry"
                | "minecraft:birch_tall"
                | "minecraft:trees_birch_and_oak_leaf_litter"
                | "minecraft:trees_snowy"
                | "minecraft:trees_grove"
                | "minecraft:trees_windswept_hills"
                | "minecraft:trees_windswept_forest"
                | "minecraft:patch_bush"
                | "minecraft:trees_savanna"
                | "minecraft:trees_windswept_savanna"
                | "minecraft:flower_warm"
                | "minecraft:patch_grass_savanna"
                | "minecraft:patch_tall_grass"
                | "minecraft:patch_dry_grass_desert"
                | "minecraft:patch_dry_grass_badlands"
                | "minecraft:patch_dead_bush"
                | "minecraft:patch_dead_bush_2"
                | "minecraft:patch_dead_bush_badlands"
                | "minecraft:patch_cactus_desert"
                | "minecraft:patch_cactus_decorated"
                | "minecraft:trees_badlands"
                | "minecraft:trees_jungle"
                | "minecraft:trees_sparse_jungle"
                | "minecraft:patch_grass_jungle"
                | "minecraft:bamboo"
                | "minecraft:bamboo_light"
                | "minecraft:bamboo_vegetation"
                | "minecraft:vines"
                | "minecraft:trees_swamp"
                | "minecraft:trees_mangrove"
                | "minecraft:flower_swamp"
                | "minecraft:patch_waterlily"
                | "minecraft:brown_mushroom_swamp"
                | "minecraft:red_mushroom_swamp"
        )
}

#[derive(Clone, Copy, Debug)]
pub struct Blend {
    weights: Weights,
}
impl Blend {
    pub fn needs_dry_warm_surface(&self) -> bool {
        (self.weights[0] + self.weights[1] + self.weights[4] + self.weights[7] + self.weights[37])
            > 0.
            && (self.weights[2] + self.weights[8] + self.weights[9]) > 0.
    }
    pub fn from_weights(base: usize, raw: Weights) -> Option<Self> {
        if base >= COUNT || raw.iter().any(|w| !w.is_finite() || *w < 0.) {
            return None;
        }
        // All participating biomes use the SAME neighborhood at a junction.
        // A climate outside that neighborhood retains its native vegetation;
        // membership must not grow transitively across incompatible climates.
        let mut best = None;
        let mut best_total = 0.;
        for family in FAMILIES {
            let total: f64 = family.iter().map(|&i| raw[i]).sum();
            if total >= 0.75
                && total > best_total
                && family.iter().filter(|&&i| raw[i] > 0.000_001).count() >= 2
            {
                best = Some(family);
                best_total = total;
            }
        }
        let family = best.filter(|f| f.contains(&base))?;
        let mut weights = [0.; COUNT];
        for &i in family {
            weights[i] = (raw[i] / best_total).clamp(0., 1.);
        }
        Some(Self { weights })
    }
    pub fn chance(self, members: [bool; COUNT]) -> f64 {
        self.weights
            .iter()
            .zip(members)
            .filter_map(|(&w, m)| m.then_some(w))
            .sum::<f64>()
            .clamp(0., 1.)
    }
    pub fn admits(self, seed: i64, x: i32, z: i32, feature: &str, members: [bool; COUNT]) -> bool {
        legacy::admits_chance(seed, x, z, feature, self.chance(members))
    }
}

// Same lattice and arithmetic order as saved v1/v2. More IDs add no native
// surface queries; the existing 4096-entry FIFO bounds the extra ~0.9 MiB.
pub fn anchor(
    x: i32,
    z: i32,
    step: i32,
    mut sample: impl FnMut(i32, i32) -> Option<usize>,
) -> Weights {
    let mut out = [0.; COUNT];
    for (dz, wz) in [(-step, 1.), (0, 2.), (step, 1.)] {
        for (dx, wx) in [(-step, 1.), (0, 2.), (step, 1.)] {
            if let Some(i) =
                sample(x.saturating_add(dx), z.saturating_add(dz)).filter(|i| *i < COUNT)
            {
                out[i] += wx * wz / 16.;
            }
        }
    }
    out
}
pub fn weights(x: i32, z: i32, step: i32, mut sample: impl FnMut(i32, i32) -> Weights) -> Weights {
    let ax = x.div_euclid(step) * step;
    let az = z.div_euclid(step) * step;
    let tx = f64::from(x.rem_euclid(step)) / f64::from(step);
    let tz = f64::from(z.rem_euclid(step)) / f64::from(step);
    let mut out = [0.; COUNT];
    for (dx, wx) in [(0, 1. - tx), (step, tx)] {
        for (dz, wz) in [(0, 1. - tz), (step, tz)] {
            let v = sample(ax.saturating_add(dx), az.saturating_add(dz));
            for i in 0..COUNT {
                out[i] += v[i] * wx * wz;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_kernel_and_queries_are_exact() {
        assert_eq!(&NAMES[..previous::COUNT], &previous::NAMES);
        for x in -40..40 {
            let old = previous::weights(x, -17, 8, |a, b| {
                previous::anchor(a, b, 8, |xx, _| Some(if xx < 0 { 2 } else { 4 }))
            });
            let mut calls = 0;
            let new = weights(x, -17, 8, |a, b| {
                anchor(a, b, 8, |xx, _| {
                    calls += 1;
                    Some(if xx < 0 { 2 } else { 4 })
                })
            });
            assert_eq!(&new[..previous::COUNT], &old);
            assert_eq!(calls, 36);
        }
    }
    #[test]
    fn new_climates_blend_both_sides_without_duplicating_density() {
        for (left, right) in [
            (26, 23),
            (27, 28),
            (30, 31),
            (31, 32),
            (33, 34),
            (13, 14),
            (15, 17),
            (3, 11),
            (10, 4),
        ] {
            for x in -15..16 {
                let w = weights(x, -3, 8, |a, b| {
                    anchor(a, b, 8, |xx, _| Some(if xx < 0 { left } else { right }))
                });
                if w[left] == 0. || w[right] == 0. {
                    continue;
                }
                let a = Blend::from_weights(left, w).unwrap();
                let b = Blend::from_weights(right, w).unwrap();
                let l = std::array::from_fn(|i| i == left);
                let r = std::array::from_fn(|i| i == right);
                assert_eq!(a.chance(l), b.chance(l));
                assert!((a.chance(l) + a.chance(r) - 1.).abs() < 1e-12);
                assert!(a.chance(l) > 0. && a.chance(l) < 1.);
            }
        }
    }
    #[test]
    fn climate_conflicts_special_biomes_and_foreign_features_stay_native() {
        for (left, right) in [(26, 13), (32, 14), (0, 12), (0, 35), (26, 34)] {
            let mut w = [0.; COUNT];
            w[left] = 0.5;
            w[right] = 0.5;
            assert!(Blend::from_weights(left, w).is_none());
            assert!(Blend::from_weights(right, w).is_none());
        }
        for name in [
            "minecraft:ore_emerald",
            "minecraft:lake_lava_surface",
            "minecraft:pale_garden_vegetation",
            "custom:trees_jungle",
        ] {
            assert!(!vegetation(name));
        }
        assert!(!enabled(Some(2.)) && !enabled(None));
    }
    #[test]
    fn triple_junctions_choose_one_common_distribution_and_are_order_independent() {
        let mut w = [0.; COUNT];
        w[23] = 0.4;
        w[26] = 0.3;
        w[27] = 0.3;
        let member = std::array::from_fn(|i| i == 23);
        let b = Blend::from_weights(23, w).unwrap();
        for base in [26, 27] {
            assert_eq!(
                b.chance(member),
                Blend::from_weights(base, w).unwrap().chance(member)
            );
        }
        let forward: Vec<_> = (-2048..2048)
            .map(|x| b.admits(9, x, 23, "minecraft:trees_savanna", member))
            .collect();
        let reverse: Vec<_> = (-2048..2048)
            .rev()
            .map(|x| b.admits(9, x, 23, "minecraft:trees_savanna", member))
            .collect();
        assert!(forward.iter().eq(reverse.iter().rev()));
        assert!((1500..1800).contains(&forward.iter().filter(|v| **v).count()));
    }

    #[test]
    fn forest_taiga_and_open_plain_share_the_transition_at_a_triple_junction() {
        let mut w = [0.; COUNT];
        w[0] = 0.4;
        w[2] = 0.4;
        w[1] = 0.2;
        for base in [0, 1, 2] {
            let blend = Blend::from_weights(base, w).unwrap();
            for source in [0, 1, 2] {
                assert!(
                    (blend.chance(std::array::from_fn(|i| i == source)) - w[source]).abs() < 1e-12
                );
            }
        }
    }
}
