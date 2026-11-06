// Original bounded biome ecotone field. MIT, dormyu.
// Pure policy: no game resources, per-frame work or chunk-local randomness.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    Forest,
    Plains,
    Taiga,
    Meadow,
    Other,
}

impl Biome {
    pub fn named(name: &str) -> Self {
        match name {
            "minecraft:forest" => Self::Forest,
            "minecraft:plains" => Self::Plains,
            "minecraft:taiga" => Self::Taiga,
            "minecraft:meadow" => Self::Meadow,
            _ => Self::Other,
        }
    }
    pub fn index(self) -> Option<usize> {
        match self {
            Self::Forest => Some(0),
            Self::Plains => Some(1),
            Self::Taiga => Some(2),
            Self::Meadow => Some(3),
            Self::Other => None,
        }
    }
}

pub const NAMES: [&str; 4] = [
    "minecraft:forest",
    "minecraft:plains",
    "minecraft:taiga",
    "minecraft:meadow",
];

/// Absence (all saved v8 recipes) and unknown marker versions are no-ops.
pub fn recipe_width(marker: Option<f64>, width: Option<f64>) -> Option<i32> {
    (marker == Some(1.)).then(|| {
        width
            .filter(|v| v.is_finite())
            .unwrap_or(32.)
            .clamp(16., 64.) as i32
    })
}

/// Verified surface vegetation roots in the pinned 26.3 resources. Membership
/// in the donor biome is checked separately; absent/custom roots are not guessed.
pub fn vegetation(name: &str) -> bool {
    matches!(
        name,
        "minecraft:forest_flowers"
            | "minecraft:trees_birch_and_oak_leaf_litter"
            | "minecraft:flower_default"
            | "minecraft:patch_grass_forest"
            | "minecraft:patch_tall_grass_2"
            | "minecraft:trees_plains"
            | "minecraft:flower_plains"
            | "minecraft:patch_grass_plain"
            | "minecraft:patch_large_fern"
            | "minecraft:trees_taiga"
            | "minecraft:patch_grass_taiga_2"
            | "minecraft:patch_berry_common"
            | "minecraft:brown_mushroom_taiga"
            | "minecraft:red_mushroom_taiga"
            | "minecraft:patch_grass_meadow"
            | "minecraft:flower_meadow"
            | "minecraft:trees_meadow"
            | "minecraft:wildflowers_meadow"
    )
}

#[derive(Clone, Copy, Debug)]
pub struct Blend {
    /// Forest/plains = 0; taiga/meadow = 2.
    pub first: usize,
    pub wooded: f64,
}

impl Blend {
    pub fn from_weights(base: Biome, weights: [f64; 4]) -> Option<Self> {
        let first = base.index()? & !1;
        let total = weights[first] + weights[first + 1];
        // Do not smear a target pair through a river, desert or mountain biome.
        if total < 0.75 {
            return None;
        }
        let wooded = (weights[first] / total).clamp(0., 1.);
        (wooded > 0.000_001 && wooded < 0.999_999).then_some(Self { first, wooded })
    }
    pub fn admits(self, seed: i64, x: i32, z: i32, feature: &str, member: [bool; 4]) -> bool {
        let a = member[self.first];
        let b = member[self.first + 1];
        let chance = if a { self.wooded } else { 0. } + if b { 1. - self.wooded } else { 0. };
        random(seed, x, z, name_hash(feature)) < chance
    }
}

/// A 3×3 tent kernel on an absolute lattice. Callers cache these anchors.
pub fn anchor(x: i32, z: i32, step: i32, mut sample: impl FnMut(i32, i32) -> Biome) -> [f64; 4] {
    let mut weights = [0.; 4];
    for (dz, wz) in [(-1, 1.), (0, 2.), (1, 1.)] {
        for (dx, wx) in [(-1, 1.), (0, 2.), (1, 1.)] {
            if let Some(i) =
                sample(x.saturating_add(dx * step), z.saturating_add(dz * step)).index()
            {
                weights[i] += wx * wz / 16.;
            }
        }
    }
    weights
}

pub fn weights(x: i32, z: i32, step: i32, mut get: impl FnMut(i32, i32) -> [f64; 4]) -> [f64; 4] {
    let bx = x.div_euclid(step) * step;
    let bz = z.div_euclid(step) * step;
    let tx = f64::from(x.rem_euclid(step)) / f64::from(step);
    let tz = f64::from(z.rem_euclid(step)) / f64::from(step);
    let mut out = [0.; 4];
    for (dx, wx) in [(0, 1. - tx), (step, tx)] {
        for (dz, wz) in [(0, 1. - tz), (step, tz)] {
            let v = get(bx.saturating_add(dx), bz.saturating_add(dz));
            for i in 0..4 {
                out[i] += v[i] * wx * wz;
            }
        }
    }
    out
}

pub fn soil(name: &str) -> bool {
    matches!(
        name,
        "minecraft:grass_block" | "minecraft:podzol" | "minecraft:coarse_dirt" | "minecraft:dirt"
    )
}

/// Forest/plains already share grass. Only dry soil in a taiga/meadow band is
/// eligible; caller checks water/support. Smooth four-block patches avoid
/// independently speckling every voxel. The patch effect fades at both band ends.
pub fn podzol(blend: Blend, seed: i64, x: i32, z: i32) -> bool {
    if blend.first != 2 {
        return false;
    }
    let edge = 4. * blend.wooded * (1. - blend.wooded);
    let value = weights(x, z, 4, |ax, az| {
        [random(seed, ax, az, 0x534f_494c), 0., 0., 0.]
    })[0];
    value < 0.4 * blend.wooded * edge
}

fn name_hash(name: &str) -> u64 {
    name.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
    })
}

/// Shared coordinate hash for the separately versioned expanded policy.
pub fn admits_chance(seed: i64, x: i32, z: i32, feature: &str, chance: f64) -> bool {
    random(seed, x, z, name_hash(feature)) < chance
}

fn random(seed: i64, x: i32, z: i32, salt: u64) -> f64 {
    let mut h = (seed as u64)
        ^ (x as u32 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (z as u32 as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9)
        ^ salt;
    h = (h ^ (h >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    h ^= h >> 31;
    (h >> 11) as f64 / ((1_u64 << 53) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boundary(x: i32, z: i32) -> [f64; 4] {
        weights(x, z, 8, |a, b| {
            anchor(
                a,
                b,
                8,
                |xx, _| if xx < 0 { Biome::Forest } else { Biome::Plains },
            )
        })
    }

    #[test]
    fn only_the_new_exact_recipe_marker_enables_ecotones() {
        for marker in [None, Some(0.), Some(2.), Some(f64::NAN)] {
            assert_eq!(recipe_width(marker, Some(32.)), None);
        }
        assert_eq!(recipe_width(Some(1.), None), Some(32));
        assert_eq!(recipe_width(Some(1.), Some(f64::NAN)), Some(32));
        assert_eq!(recipe_width(Some(1.), Some(1.)), Some(16));
        assert_eq!(recipe_width(Some(1.), Some(1000.)), Some(64));
        for (i, name) in NAMES.iter().enumerate() {
            assert_eq!(Biome::named(name).index(), Some(i));
        }
        assert_eq!(Biome::named("custom:forest").index(), None);
    }

    #[test]
    fn band_is_monotone_bounded_and_continuous_across_negative_chunks() {
        assert_eq!(boundary(-32, 7), [1., 0., 0., 0.]);
        assert_eq!(boundary(32, 7), [0., 1., 0., 0.]);
        let mut previous = 1.;
        for x in -32..=32 {
            let w = boundary(x, 7);
            assert!((w[0] + w[1] - 1.).abs() < 1e-12);
            assert!(w[0] <= previous && previous - w[0] <= 0.07);
            assert_eq!(w, boundary(x, 16));
            previous = w[0];
        }
    }

    #[test]
    fn unrelated_biomes_are_not_blended_or_used_as_shortcuts() {
        assert!(Blend::from_weights(Biome::Other, [0.5, 0.5, 0., 0.]).is_none());
        assert!(Blend::from_weights(Biome::Forest, [0.3, 0.3, 0., 0.]).is_none());
        assert!(Blend::from_weights(Biome::Forest, [1., 0., 0., 0.]).is_none());
    }

    #[test]
    fn weighted_vegetation_is_coordinate_stable_and_retains_shared_features() {
        let blend = Blend {
            first: 0,
            wooded: 0.25,
        };
        let members = [true, false, false, false];
        let forward: Vec<_> = (-4096..4096)
            .map(|x| blend.admits(9, x, -17, "minecraft:trees_plains", members))
            .collect();
        let reverse: Vec<_> = (-4096..4096)
            .rev()
            .map(|x| blend.admits(9, x, -17, "minecraft:trees_plains", members))
            .collect();
        assert!(forward.iter().eq(reverse.iter().rev()));
        let count = forward.iter().filter(|v| **v).count();
        assert!((1800..2300).contains(&count), "{count}");
        for x in -50..50 {
            assert!(blend.admits(9, x, 3, "shared", [true, true, false, false]));
            assert!(!blend.admits(9, x, 3, "absent", [false; 4]));
        }
    }

    #[test]
    fn vegetation_allowlist_excludes_other_world_generation() {
        for name in [
            "minecraft:ore_emerald",
            "minecraft:lake_lava_surface",
            "minecraft:spring_water",
            "custom:trees",
            "minecraft:glow_lichen",
        ] {
            assert!(!vegetation(name), "{name}");
        }
        for name in [
            "minecraft:trees_taiga",
            "minecraft:trees_meadow",
            "minecraft:forest_flowers",
        ] {
            assert!(vegetation(name));
        }
    }

    #[test]
    fn ground_changes_are_soil_only_and_fade_out_of_the_band() {
        assert!(!soil("minecraft:stone") && !soil("minecraft:water") && !soil("minecraft:air"));
        assert!(soil("minecraft:grass_block") && soil("minecraft:podzol"));
        for x in -64..64 {
            assert!(!podzol(
                Blend {
                    first: 0,
                    wooded: 0.5
                },
                1,
                x,
                3
            ));
            assert!(!podzol(
                Blend {
                    first: 2,
                    wooded: 0.
                },
                1,
                x,
                3
            ));
            assert!(!podzol(
                Blend {
                    first: 2,
                    wooded: 1.
                },
                1,
                x,
                3
            ));
        }
    }
}
