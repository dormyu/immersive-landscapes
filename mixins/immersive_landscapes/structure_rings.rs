// Local ring evaluation by dormyu, MIT.
// Uses Horizon f42ef42's original RingSearch, seed and biome-search ordering.
// The injected call sites retain Horizon's original structure-start behavior.
use super::{BiomeId, ChunkPos, RingWork, Rings, StartContext, StructureState};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::sync::atomic::Ordering;

// A type alias keeps the callback/result lifetimes independent without explicit
// generic parameters in the HEAD helper (unsupported by the mixin contract).
pub(super) type Preferred<'result, 'callback> =
    dyn Fn(usize) -> &'result BTreeSet<BiomeId> + 'callback;

fn ensure(
    work: &RingWork,
    index: usize,
    preferred: &Preferred<'_, '_>,
    biomes: &mut dyn FnMut(&[[i32; 3]]) -> Vec<BiomeId>,
) -> ChunkPos {
    *work.found[index].get_or_init(|| {
        let (set, search) = &work.searches[index];
        if work.fixed {
            search.find_fixed(preferred(*set), biomes)
        } else {
            // Keep the entire ordered quart square in one native lookup.
            // Splitting this call would change the climate resolver's tie order.
            search.find(preferred(*set), biomes)
        }
    })
}

fn near_work(
    work: &RingWork,
    preferred: &Preferred<'_, '_>,
    mut name: impl FnMut(usize) -> String,
    biomes: &mut dyn FnMut(&[[i32; 3]]) -> Vec<BiomeId>,
    chunk: ChunkPos,
    zone_reach: i32,
) -> Rings {
    let mut rings: Rings = work.sets.iter().map(|&set| (name(set), Vec::new())).collect();
    for (index, (set, search)) in work.searches.iter().enumerate() {
        // The native bound includes the search displacement and the complete
        // exclusion-zone chain. A position outside it cannot affect this chunk.
        if search.may_land_near(chunk.x, chunk.z, zone_reach) {
            let found = ensure(work, index, preferred, biomes);
            rings.get_mut(&name(*set)).expect("registered ring set").push(found);
        }
    }
    rings
}

pub(super) fn near<'state>(
    state: &'state StructureState,
    context: &mut impl StartContext,
    chunk: ChunkPos,
) -> Cow<'state, Rings> {
    if let Some(rings) = state.rings.get() {
        return Cow::Borrowed(rings);
    }
    let work = state.ring_work(context);
    let registry = &state.registry;
    Cow::Owned(near_work(
        work,
        &|set| registry.ring_biomes(set),
        |set| registry.sets[set].name.clone(),
        &mut |quarts| context.noise_biomes(quarts),
        chunk,
        state.zone_reach,
    ))
}

pub(super) fn help(
    work: &RingWork,
    preferred: &Preferred<'_, '_>,
    biomes: &mut dyn FnMut(&[[i32; 3]]) -> Vec<BiomeId>,
) {
    // Full /locate queries can still share outstanding work between threads.
    loop {
        let index = work.next.fetch_add(1, Ordering::Relaxed);
        if index >= work.searches.len() {
            break;
        }
        ensure(work, index, preferred, biomes);
    }
    // Join each exact result rather than using the old global left counter:
    // lazy local searches may already have initialized arbitrary positions.
    // get_or_init also retries an initializer that unwound instead of leaving
    // a claimed search and an unfinishable global barrier behind.
    for index in 0..work.searches.len() {
        ensure(work, index, preferred, biomes);
    }
}

pub(super) fn full(state: &StructureState, context: &mut impl StartContext) {
    if state.rings.get().is_some() {
        return;
    }
    let work = state.ring_work(context);
    let registry = &state.registry;
    help(work, &|set| registry.ring_biomes(set), &mut |quarts| {
        context.noise_biomes(quarts)
    });
    state.rings.get_or_init(|| work.rings(|set| registry.sets[set].name.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worldgen::structure::stronghold::{ring_positions, ring_searches};
    use std::sync::atomic::AtomicUsize;

    fn biome_values(quarts: &[[i32; 3]]) -> Vec<BiomeId> {
        // Real RingSearch calls the native resolver once for its full ordered
        // square; its first quart is the seed of the resolver's tie walk.
        let first = quarts.first().expect("nonempty native biome search");
        let side = (quarts.len() as f64).sqrt() as usize;
        assert_eq!(side * side, quarts.len());
        for (index, quart) in quarts.iter().enumerate() {
            assert_eq!(*quart, [first[0] + (index % side) as i32, 0, first[2] + (index / side) as i32]);
        }
        quarts.iter().map(|q| {
            BiomeId((q[0].wrapping_mul(31) ^ q[2].wrapping_mul(17)).rem_euclid(64) as u32)
        }).collect()
    }

    fn preferred() -> BTreeSet<BiomeId> {
        (0..20).map(|b| BiomeId(b * 3)).collect()
    }

    #[test]
    fn a_local_chunk_only_resolves_relevant_searches_with_exact_zone_membership() {
        let preferred = preferred();
        for seed in [0_i64, 1234, -77, i64::MAX] {
            let serial = ring_positions(seed, 32, 3, 128, &preferred, &mut biome_values);
            for index in [0, 1, 17, 127] {
                let work = RingWork::new(vec![(0, ring_searches(seed, 32, 3, 128))]);
                let center = serial[index];
                let mut calls = 0;
                let mut sample = |quarts: &[[i32; 3]]| {
                    calls += 1;
                    biome_values(quarts)
                };
                let local = near_work(&work, &|_| &preferred, |_| "stronghold".into(),
                    &mut sample, center, 3);
                assert!(calls > 0 && calls < 128, "one chunk must not launch all global searches");
                assert_eq!(work.found.iter().filter(|entry| entry.get().is_some()).count(), calls);
                // Starts and chained exclusion zones only test membership in
                // this bounded area; every such answer equals the full oracle.
                for dx in -3..=3 {
                    for dz in -3..=3 {
                        let query = ChunkPos::new(center.x + dx, center.z + dz);
                        assert_eq!(local["stronghold"].contains(&query), serial.contains(&query));
                    }
                }
            }
        }
    }

    #[test]
    fn local_then_full_reuses_results_and_preserves_all_positions_and_empty_sets() {
        let preferred = preferred();
        for seed in [0_i64, 1234, -77] {
            let serial = ring_positions(seed, 32, 3, 128, &preferred, &mut biome_values);
            let work = RingWork::new(vec![
                (0, ring_searches(seed, 32, 3, 128)),
                (1, ring_searches(seed, 10, 4, 0)),
            ]);
            let mut calls = 0;
            let mut sample = |quarts: &[[i32; 3]]| {
                calls += 1;
                biome_values(quarts)
            };
            let local = near_work(&work, &|_| &preferred, |set| format!("set{set}"),
                &mut sample, serial[0], 0);
            assert!(local["set0"].contains(&serial[0]));
            assert!(local["set1"].is_empty());
            help(&work, &|_| &preferred, &mut sample);
            assert_eq!(calls, 128, "the full query must not repeat initialized searches");
            let rings = work.rings(|set| format!("set{set}"));
            assert_eq!(rings["set0"], serial);
            assert!(rings["set1"].is_empty());
        }
    }

    #[test]
    fn simultaneous_local_and_full_queries_compute_each_position_once() {
        let seed = 1234;
        let preferred = preferred();
        let serial = ring_positions(seed, 32, 3, 128, &preferred, &mut biome_values);
        let work = RingWork::new(vec![(0, ring_searches(seed, 32, 3, 128))]);
        let calls = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for thread in 0..4 {
                let work = &work;
                let calls = &calls;
                let preferred = &preferred;
                let serial = &serial;
                scope.spawn(move || {
                    let mut sample = |quarts: &[[i32; 3]]| {
                        calls.fetch_add(1, Ordering::Relaxed);
                        biome_values(quarts)
                    };
                    let local = near_work(work, &|_| preferred, |_| "stronghold".into(),
                        &mut sample, serial[thread], 2);
                    assert!(local["stronghold"].contains(&serial[thread]));
                    help(work, &|_| preferred, &mut sample);
                    assert_eq!(work.rings(|_| "stronghold".into())["stronghold"], *serial);
                });
            }
        });
        assert_eq!(calls.load(Ordering::Relaxed), 128);
    }

    #[test]
    fn an_unwound_search_can_be_completed_without_an_abandoned_global_barrier() {
        let preferred = preferred();
        let serial = ring_positions(1234, 32, 3, 128, &preferred, &mut biome_values);
        let work = RingWork::new(vec![(0, ring_searches(1234, 32, 3, 128))]);
        let interrupted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            help(&work, &|_| &preferred, &mut |_| panic!("interrupted biome search"));
        }));
        assert!(interrupted.is_err());
        assert!(work.found[0].get().is_none());
        // The atomic work index has advanced past zero. The second completion
        // pass must retry that missing position, not wait on an abandoned job.
        help(&work, &|_| &preferred, &mut biome_values);
        assert_eq!(work.rings(|_| "stronghold".into())["stronghold"], serial);
    }

    #[test]
    fn fixed_biome_search_keeps_the_native_rng_draws() {
        let preferred: BTreeSet<_> = [BiomeId(1)].into();
        for allowed in [false, true] {
            let searches = ring_searches(1234, 32, 3, 128);
            let sample = |quarts: &[[i32; 3]]| vec![BiomeId(u32::from(allowed)); quarts.len()];
            let serial: Vec<_> = searches.iter().map(|s| s.find_fixed(&preferred, &mut { sample })).collect();
            let mut work = RingWork::new(vec![(0, searches)]);
            work.fixed = true;
            let local = near_work(&work, &|_| &preferred, |_| "fixed".into(),
                &mut { sample }, serial[0], 0);
            assert!(local["fixed"].contains(&serial[0]));
            help(&work, &|_| &preferred, &mut { sample });
            assert_eq!(work.rings(|_| "fixed".into())["fixed"], serial);
        }
    }
}
