// Original expanded ecotone policy. MIT, dormyu.
// Opt-in ecotones_v2=1; the saved v1 policy remains separate.
use super::immersive_landscapes_ecotones as legacy;

pub const COUNT: usize = 10;
// The first four indices are the unchanged v1 order and scheduler aliases.
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
];
pub const TEMPERATE: &[usize] = &[0, 1, 4, 5, 6, 7];
pub const COOL: &[usize] = &[2, 3, 8, 9];
pub const FAMILIES: [&[usize]; 2] = [TEMPERATE, COOL];
// Direct, snow-free deciduous/conifer borders only. This is not a transitive
// climate graph: meadow and dark forest do not become mutual donors via forest.
pub const MIXED_FOREST: &[usize] = &[0, 4, 2, 8, 9];
pub fn mixed_sides_present(present: &[bool; COUNT]) -> bool {
    (present[0] || present[4]) && (present[2] || present[8] || present[9])
}
pub type Weights = [f64; COUNT];

pub fn enabled(marker: Option<f64>) -> bool {
    marker == Some(1.)
}
pub fn named(name: &str) -> Option<usize> {
    NAMES.iter().position(|n| *n == name)
}
pub fn vegetation(name: &str) -> bool {
    legacy::vegetation(name)
        || matches!(
            name,
            "minecraft:trees_birch"
                | "minecraft:wildflowers_birch_forest"
                | "minecraft:trees_flower_forest"
                | "minecraft:flower_forest_flowers"
                | "minecraft:flower_flower_forest"
                | "minecraft:patch_grass_badlands"
                | "minecraft:dark_forest_vegetation"
                | "minecraft:patch_leaf_litter"
                | "minecraft:patch_sunflower"
                | "minecraft:trees_old_growth_pine_taiga"
                | "minecraft:trees_old_growth_spruce_taiga"
                | "minecraft:patch_grass_taiga"
                | "minecraft:brown_mushroom_old_growth"
                | "minecraft:red_mushroom_old_growth"
                | "minecraft:brown_mushroom_normal"
                | "minecraft:red_mushroom_normal"
        )
}

#[derive(Clone, Copy, Debug)]
pub struct Blend {
    weights: Weights,
    pub cross_family: bool,
}
impl Blend {
    pub fn from_weights(base: usize, raw: Weights) -> Option<Self> {
        let present = raw.map(|w| w > 0.000_001);
        if MIXED_FOREST.contains(&base)
            && mixed_sides_present(&present)
            && let Some(blend) = Self::within(MIXED_FOREST, raw, true)
        {
            return Some(blend);
        }
        Self::within(FAMILIES.iter().find(|f| f.contains(&base))?, raw, false)
    }
    fn within(family: &[usize], raw: Weights, cross_family: bool) -> Option<Self> {
        let total: f64 = family.iter().map(|&i| raw[i]).sum();
        if total < 0.75 || family.iter().filter(|&&i| raw[i] > 0.000_001).count() < 2 {
            return None;
        }
        let mut weights = [0.; COUNT];
        for &i in family {
            weights[i] = (raw[i] / total).clamp(0., 1.);
        }
        Some(Self {
            weights,
            cross_family,
        })
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

pub fn crossing_surface(ground: &str, above: &str, can_snow: bool) -> bool {
    legacy::soil(ground) && above == "minecraft:air" && !can_snow
}

// Same nine-sample kernel and four-anchor interpolation as v1. Extra biome
// weights do not add any native samples or increase the 4096-anchor limit.
pub fn anchor(
    x: i32,
    z: i32,
    step: i32,
    mut sample: impl FnMut(i32, i32) -> Option<usize>,
) -> Weights {
    let mut weights = [0.; COUNT];
    for (dz, wz) in [(-1, 1.), (0, 2.), (1, 1.)] {
        for (dx, wx) in [(-1, 1.), (0, 2.), (1, 1.)] {
            if let Some(i) = sample(x.saturating_add(dx * step), z.saturating_add(dz * step)) {
                weights[i] += wx * wz / 16.;
            }
        }
    }
    weights
}
pub fn weights(x: i32, z: i32, step: i32, mut get: impl FnMut(i32, i32) -> Weights) -> Weights {
    let bx = x.div_euclid(step) * step;
    let bz = z.div_euclid(step) * step;
    let tx = f64::from(x.rem_euclid(step)) / f64::from(step);
    let tz = f64::from(z.rem_euclid(step)) / f64::from(step);
    let mut out = [0.; COUNT];
    for (dx, wx) in [(0, 1. - tx), (step, tx)] {
        for (dz, wz) in [(0, 1. - tz), (step, tz)] {
            let v = get(bx.saturating_add(dx), bz.saturating_add(dz));
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
    fn edge(x: i32, left: usize, right: usize) -> Weights {
        weights(x, -17, 8, |a, b| {
            anchor(a, b, 8, |xx, _| Some(if xx < 0 { left } else { right }))
        })
    }
    #[test]
    fn expanded_marker_and_families_exclude_dry_hot_wet_and_snowy_biomes() {
        for marker in [None, Some(0.), Some(2.), Some(f64::NAN)] {
            assert!(!enabled(marker));
        }
        assert!(enabled(Some(1.)));
        for name in [
            "desert",
            "savanna",
            "jungle",
            "swamp",
            "snowy_taiga",
            "snowy_plains",
            "river",
            "ocean",
        ] {
            assert!(named(&format!("minecraft:{name}")).is_none());
        }
        let mut raw = [0.; COUNT];
        raw[6] = 0.5;
        raw[2] = 0.5;
        assert!(Blend::from_weights(6, raw).is_none() && Blend::from_weights(2, raw).is_none());
        raw[2] = 0.;
        raw[1] = 0.2;
        assert!(
            Blend::from_weights(0, raw).is_none(),
            "unknown terrain is not a shortcut"
        );
    }
    #[test]
    fn direct_forest_taiga_borders_mix_both_sides_only_on_dry_snow_free_ground() {
        for (left, right) in [(0, 2), (4, 8), (0, 9)] {
            for x in -7..=7 {
                let raw = edge(x, left, right);
                let a = Blend::from_weights(left, raw).unwrap();
                let b = Blend::from_weights(right, raw).unwrap();
                assert!(a.cross_family && b.cross_family);
                let mut member = [false; COUNT];
                member[left] = true;
                assert_eq!(a.chance(member), b.chance(member));
                assert!(a.chance(member) > 0. && a.chance(member) < 1.);
            }
        }
        let mut raw = [0.; COUNT];
        raw[3] = 0.5;
        raw[6] = 0.5;
        assert!(Blend::from_weights(3, raw).is_none());
        assert!(Blend::from_weights(6, raw).is_none());
        assert!(crossing_surface("minecraft:podzol", "minecraft:air", false));
        assert!(!crossing_surface("minecraft:podzol", "minecraft:air", true));
        for ground in [
            "minecraft:air",
            "minecraft:water",
            "minecraft:snow_block",
            "minecraft:stone",
        ] {
            assert!(!crossing_surface(ground, "minecraft:air", false));
        }
        for above in ["minecraft:water", "minecraft:snow", "minecraft:oak_leaves"] {
            assert!(!crossing_surface("minecraft:grass_block", above, false));
        }
    }
    #[test]
    fn both_native_and_donor_vegetation_change_continuously_on_both_sides() {
        for (left, right) in [(6, 1), (4, 0), (5, 7), (8, 3), (9, 2)] {
            let mut left_only = [false; COUNT];
            left_only[left] = true;
            let mut right_only = [false; COUNT];
            right_only[right] = true;
            let mut shared = left_only;
            shared[right] = true;
            let mut prior = 1.;
            for x in -7..=7 {
                let raw = edge(x, left, right);
                let base = if x < 0 { left } else { right };
                let blend = Blend::from_weights(base, raw).unwrap();
                let a = blend.chance(left_only);
                let b = blend.chance(right_only);
                assert!(
                    a > 0. && a < 1. && b > 0. && b < 1.,
                    "native vegetation must thin too"
                );
                assert!((a + b - 1.).abs() < 1e-12);
                assert!(a <= prior && prior - a <= 0.45);
                assert_eq!(blend.chance(shared), 1.);
                // Selecting the biome on either side does not jump the field.
                assert_eq!(a, Blend::from_weights(left, raw).unwrap().chance(left_only));
                assert_eq!(
                    a,
                    Blend::from_weights(right, raw).unwrap().chance(left_only)
                );
                prior = a;
            }
        }
    }
    #[test]
    fn expanded_admission_is_order_independent_and_matches_the_weighted_density() {
        let mut raw = [0.; COUNT];
        raw[4] = 0.25;
        raw[6] = 0.75;
        let blend = Blend::from_weights(4, raw).unwrap();
        let mut members = [false; COUNT];
        members[4] = true;
        let forward: Vec<_> = (-4096..4096)
            .map(|x| blend.admits(7, x, -16, "minecraft:trees_birch", members))
            .collect();
        let reverse: Vec<_> = (-4096..4096)
            .rev()
            .map(|x| blend.admits(7, x, -16, "minecraft:trees_birch", members))
            .collect();
        assert!(forward.iter().eq(reverse.iter().rev()));
        assert!((1800..2300).contains(&forward.iter().filter(|v| **v).count()));
    }
    #[test]
    fn larger_weight_arrays_keep_the_v1_kernel_exact_and_do_not_add_sample_queries() {
        for x in -32..=32 {
            let mut calls = 0;
            let newer = weights(x, -17, 8, |a, b| {
                anchor(a, b, 8, |xx, _| {
                    calls += 1;
                    Some(if xx < 0 { 0 } else { 1 })
                })
            });
            let older = legacy::weights(x, -17, 8, |a, b| {
                legacy::anchor(a, b, 8, |xx, _| {
                    if xx < 0 {
                        legacy::Biome::Forest
                    } else {
                        legacy::Biome::Plains
                    }
                })
            });
            assert_eq!(&newer[..4], &older);
            assert_eq!(calls, 36, "same four cached anchors, nine samples each");
        }
        for name in [
            "minecraft:ore_emerald",
            "minecraft:lake_lava_surface",
            "minecraft:patch_sugar_cane",
            "custom:trees_birch",
        ] {
            assert!(!vegetation(name));
        }
    }
}
