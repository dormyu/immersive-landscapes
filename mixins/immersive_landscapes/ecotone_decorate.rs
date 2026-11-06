fn immersive_landscapes_ecotone_decorate(
    worker: &mut GeneratorWorker,
    region: &mut [ProtoChunk; 9],
) -> std::ops::ControlFlow<Result<DecorateStats, GeneratorError>> {
    use crate::worldgen::{
        immersive_landscapes_ecotone_runtime as e, immersive_landscapes_hydrology as h,
    };
    let key = std::sync::Arc::as_ptr(&worker.pipeline) as *const () as usize;
    if e::reentry(e::ScopeKind::Decoration, key) {
        return std::ops::ControlFlow::Continue(());
    }
    let atlas = h::for_pipeline(key).filter(|a| a.ecotones.get().is_some());
    // A nested vanilla/old-world decoration must mask an outer v9 scope, not
    // accidentally inherit its biome ids or vegetation policy.
    if atlas.is_none() && !e::has_scope() {
        return std::ops::ControlFlow::Continue(());
    }
    let _scope = e::enter(e::ScopeKind::Decoration, key, atlas);
    std::ops::ControlFlow::Break(worker.decorate(region))
}
