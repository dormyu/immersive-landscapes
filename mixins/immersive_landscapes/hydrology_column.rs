fn immersive_landscapes_hydrology_column(
    mut value: (Option<SurfaceSample>, Vec<(i32, i32, BlockStateId)>),
    worker: &GeneratorWorker,
    x: i32,
    z: i32,
) -> (Option<SurfaceSample>, Vec<(i32, i32, BlockStateId)>) {
    let key = std::sync::Arc::as_ptr(&worker.pipeline) as *const () as usize;
    if let Some(atlas) = crate::worldgen::immersive_landscapes_hydrology::for_pipeline(key) {
        crate::worldgen::immersive_landscapes_hydrology::fill_column(
            &mut value,
            &atlas,
            worker.pipeline.data().registry,
            x,
            z,
        );
    }
    value
}
