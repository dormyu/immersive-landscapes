fn immersive_landscapes_hydrology_grid(
    value: Result<Vec<LodColumn>, GeneratorError>,
    worker: &GeneratorWorker,
    x: i32,
    z: i32,
    width: i32,
    side: usize,
) -> Result<Vec<LodColumn>, GeneratorError> {
    value.map(|mut columns| {
        let key = std::sync::Arc::as_ptr(&worker.pipeline) as *const () as usize;
        if let Some(atlas) = crate::worldgen::immersive_landscapes_hydrology::for_pipeline(key) {
            crate::worldgen::immersive_landscapes_hydrology::fill_grid(
                &mut columns,
                &atlas,
                worker.pipeline.data().registry,
                x,
                z,
                width,
                side,
            );
        }
        columns
    })
}
