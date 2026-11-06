// Original native ecotone integration. MIT, dormyu.
use super::{immersive_landscapes_ecotones as e, immersive_landscapes_hydrology::Atlas};
use crate::registry::{BlockRegistry, BlockStateId};
use crate::worldgen::{
    biome::BiomeId,
    generator::{GeneratorData, LodColumn, ProtoChunk, SurfaceSample},
    terrain::TerrainError,
};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

// These ids exist ONLY in FeatureDecorationPlan's scheduling map. They never
// enter BiomeChunk, a biome registry, network/save data or gameplay queries.
pub const ALIASES: [u32; 4] = [u32::MAX - 16, u32::MAX - 15, u32::MAX - 14, u32::MAX - 13];
const MAX_ANCHORS: usize = 4096;

/// Unknown/custom executable graphs keep their native behavior. A familiar
/// registry name alone must not turn an ore/lake/template into a donor feature.
pub fn safe_vegetation_definition(
    root: &crate::worldgen::feature_graph::PlacedFeatureDefinition,
) -> bool {
    use crate::worldgen::{feature_graph::FeatureDefinition as F, placement::PlacementModifier};
    fn safe(f: &F, depth: usize, remaining: &mut usize) -> bool {
        if depth > 16 || *remaining == 0 {
            return false;
        }
        *remaining -= 1;
        match f {
            // The pinned 26.3 forest/plains/taiga selectors also contain fallen
            // trees. They remain vegetation; rejecting the whole selector
            // would silently disable every dense-forest donor root.
            F::Tree(_) | F::FallenTree(_) | F::SimpleBlock(_) | F::NoOp => true,
            F::Sequence(v) | F::Overlay(v) | F::RandomChoice(v) => {
                v.iter().all(|p| safe(&p.feature, depth + 1, remaining))
            }
            F::BooleanChoice(v) => v.iter().all(|p| safe(&p.feature, depth + 1, remaining)),
            F::ChanceChoice { features, fallback } => {
                features
                    .iter()
                    .all(|(_, p)| safe(&p.feature, depth + 1, remaining))
                    && safe(&fallback.feature, depth + 1, remaining)
            }
            _ => false,
        }
    }
    root.placement
        .iter()
        .any(|p| matches!(p, PlacementModifier::Biome))
        && safe(&root.feature, 0, &mut 256)
}

#[derive(Debug, Default)]
struct Cache {
    points: HashMap<(i32, i32), [f64; 4]>,
    order: VecDeque<(i32, i32)>,
}

#[derive(Debug)]
pub struct Pass {
    pub ids: [Option<BiomeId>; 4],
    step: i32,
    cache: Mutex<Cache>,
}

impl Pass {
    pub fn new(data: &GeneratorData, width: i32) -> Self {
        Self {
            ids: e::NAMES.map(|name| data.biome_id(name)),
            step: (width.clamp(16, 64) / 4).max(4),
            cache: Mutex::new(Cache::default()),
        }
    }
    fn classify(&self, id: BiomeId) -> e::Biome {
        self.ids
            .iter()
            .position(|v| *v == Some(id))
            .map_or(e::Biome::Other, |i| e::Biome::named(e::NAMES[i]))
    }
    fn anchor(&self, atlas: &Atlas, x: i32, z: i32) -> [f64; 4] {
        if let Some(v) = self
            .cache
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .points
            .get(&(x, z))
            .copied()
        {
            return v;
        }
        // Never hold the cache mutex while sampling the native pipeline. Its
        // callback bypasses hydrology and ecotones, avoiding recursive sampling.
        let value = e::anchor(x, z, self.step, |a, b| {
            atlas
                .native_land(a, b)
                .filter(|s| s.water.is_none())
                .map_or(e::Biome::Other, |s| self.classify(s.biome))
        });
        let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = cache.points.get(&(x, z)) {
            return *v;
        }
        while cache.points.len() >= MAX_ANCHORS {
            if let Some(key) = cache.order.pop_front() {
                cache.points.remove(&key);
            }
        }
        cache.points.insert((x, z), value);
        cache.order.push_back((x, z));
        value
    }
    pub fn blend(&self, atlas: &Atlas, x: i32, z: i32, biome: BiomeId) -> Option<e::Blend> {
        let base = self.classify(biome);
        base.index()?;
        e::Blend::from_weights(
            base,
            e::weights(x, z, self.step, |a, b| self.anchor(atlas, a, b)),
        )
    }
    fn ground_band(&self, atlas: &Atlas, x: i32, z: i32) -> Option<e::Blend> {
        self.ids[2]?;
        self.ids[3]?;
        // Both near and far use the same coarse eligibility field before
        // asking for the exact zoomed native biome. Stored proto quart ids
        // differ from that biome at a boundary and cannot gate only near paint.
        e::Blend::from_weights(
            e::Biome::Taiga,
            e::weights(x, z, self.step, |a, b| self.anchor(atlas, a, b)),
        )
    }
    pub fn aliases_for_chunk(&self, atlas: &Atlas, cx: i32, cz: i32) -> Vec<BiomeId> {
        let x = cx.saturating_mul(16);
        let z = cz.saturating_mul(16);
        let mut present = [false; 4];
        // Cover every lattice cell intersecting this chunk, including both
        // endpoints. At the smallest width this is only 5×5 cached anchors.
        let ax = x.div_euclid(self.step) * self.step;
        let az = z.div_euclid(self.step) * self.step;
        let ex = x.saturating_add(15).div_euclid(self.step).saturating_add(1) * self.step;
        let ez = z.saturating_add(15).div_euclid(self.step).saturating_add(1) * self.step;
        for b in (az..=ez).step_by(self.step as usize) {
            for a in (ax..=ex).step_by(self.step as usize) {
                let w = self.anchor(atlas, a, b);
                for i in 0..4 {
                    present[i] |= w[i] > 0.;
                }
            }
        }
        let mut out = Vec::new();
        for first in [0, 2] {
            if present[first]
                && present[first + 1]
                && self.ids[first].is_some()
                && self.ids[first + 1].is_some()
            {
                out.extend([BiomeId(ALIASES[first]), BiomeId(ALIASES[first + 1])]);
            }
        }
        out
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Decoration,
    Plan,
    Biomes,
}

struct Frame {
    kind: ScopeKind,
    pipeline: usize,
    reentry: bool,
    atlas: Option<Arc<Atlas>>,
}
thread_local! { static SCOPES: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) }; }
pub struct Scope {
    depth: usize,
    not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}
impl Drop for Scope {
    fn drop(&mut self) {
        SCOPES.with_borrow_mut(|frames| {
            debug_assert_eq!(frames.len(), self.depth);
            frames.truncate(self.depth - 1);
        });
    }
}
pub fn enter(kind: ScopeKind, pipeline: usize, atlas: Option<Arc<Atlas>>) -> Scope {
    let depth = SCOPES.with_borrow_mut(|frames| {
        frames.push(Frame {
            kind,
            pipeline,
            reentry: true,
            atlas,
        });
        frames.len()
    });
    Scope {
        depth,
        not_send: std::marker::PhantomData,
    }
}
pub fn reentry(kind: ScopeKind, pipeline: usize) -> bool {
    SCOPES.with_borrow_mut(|frames| {
        frames.last_mut().is_some_and(|f| {
            if f.kind == kind && f.pipeline == pipeline && f.reentry {
                f.reentry = false;
                true
            } else {
                false
            }
        })
    })
}
pub fn has_scope() -> bool {
    SCOPES.with_borrow(|frames| !frames.is_empty())
}
pub fn active() -> Option<Arc<Atlas>> {
    SCOPES.with_borrow(|frames| frames.last().and_then(|f| f.atlas.clone()))
}

fn ground(
    atlas: &Atlas,
    registry: &BlockRegistry,
    x: i32,
    z: i32,
    sample: SurfaceSample,
) -> Option<BlockStateId> {
    let pass = atlas.ecotones.get()?;
    if sample.water.is_some() || !e::soil(registry.state(sample.state)?.name.as_str()) {
        return None;
    }
    let blend = pass.ground_band(atlas, x, z)?;
    // Hydrology owns wet beds AND dry bank margins. Never repaint a checked
    // river raster, including a no-cut river whose native valley was deeper.
    if atlas.profile_at(x, z).is_some() {
        return None;
    }
    let native = atlas.native_land(x, z)?;
    if native.water.is_some()
        || native.height != sample.height
        || !e::soil(registry.state(native.state)?.name.as_str())
    {
        return None;
    }
    if pass
        .classify(native.biome)
        .index()
        .is_none_or(|i| i & !1 != blend.first)
    {
        return None;
    }
    registry.default_state(if e::podzol(blend, atlas.seed, x, z) {
        "minecraft:podzol"
    } else {
        "minecraft:grass_block"
    })
}

pub fn fill_surface(
    surface: &mut Option<SurfaceSample>,
    atlas: &Atlas,
    registry: &BlockRegistry,
    x: i32,
    z: i32,
) {
    if let Some(sample) = surface
        && let Some(top) = ground(atlas, registry, x, z, *sample)
    {
        sample.state = top;
    }
}
pub fn fill_column(
    column: &mut LodColumn,
    atlas: &Atlas,
    registry: &BlockRegistry,
    x: i32,
    z: i32,
) {
    if let Some(sample) = column.0
        && let Some(top) = ground(atlas, registry, x, z, sample)
    {
        super::immersive_landscapes_spans::replace_run(
            &mut column.1,
            sample.height - 1,
            sample.height,
            top,
        );
        column.0.as_mut().unwrap().state = top;
    }
}
pub fn fill_proto(
    proto: &mut ProtoChunk,
    atlas: &Atlas,
    registry: &BlockRegistry,
) -> Result<(), TerrainError> {
    let Some(pass) = atlas.ecotones.get() else {
        return Ok(());
    };
    let pos = proto.position();
    for z in 0..16 {
        for x in 0..16 {
            let wx = pos.x * 16 + x;
            let wz = pos.z * 16 + z;
            let height = proto.surface_height(x, z);
            let y = height - 1;
            // Existing dry native soil only: no water, rocks, cave opening, river
            // liner or raised ground. The same native height is used in far samples.
            if !e::soil(proto.block_name(x, y, z)) {
                continue;
            }
            // Coarse cached field first; exact native-column sampling occurs only
            // inside a target transition band, never as a whole-world biome scan.
            if pass.ground_band(atlas, wx, wz).is_none() {
                continue;
            }
            let Some(native) = atlas.native_land(wx, wz).filter(|s| s.height == height) else {
                continue;
            };
            if let Some(top) = ground(atlas, registry, wx, wz, native) {
                proto.terrain.set_block_worldgen(x, y, z, top)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod scope_tests {
    use super::*;
    #[test]
    fn decoration_scopes_restore_on_nested_worlds_panics_and_threads() {
        assert!(!has_scope());
        let parent = enter(ScopeKind::Decoration, 10, None);
        assert!(!reentry(ScopeKind::Plan, 10));
        assert!(reentry(ScopeKind::Decoration, 10));
        assert!(!reentry(ScopeKind::Decoration, 10));
        std::thread::spawn(|| assert!(!has_scope())).join().unwrap();
        let failed = std::panic::catch_unwind(|| {
            let _nested_vanilla = enter(ScopeKind::Biomes, 20, None);
            assert!(!reentry(ScopeKind::Decoration, 10) && reentry(ScopeKind::Biomes, 20));
            assert!(active().is_none());
            panic!("test early decoration failure");
        });
        assert!(failed.is_err());
        assert_eq!(SCOPES.with_borrow(Vec::len), 1);
        drop(parent);
        assert!(!has_scope() && active().is_none());
    }
}
