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
            if atlas.ecotones.get().is_some() {
                for (i, column) in columns.iter_mut().enumerate() {
                    crate::worldgen::immersive_landscapes_ecotone_runtime::fill_column(
                        column,
                        &atlas,
                        worker.pipeline.data().registry,
                        x + (i % side) as i32 * width,
                        z + (i / side) as i32 * width,
                    );
                }
            }
        }
        columns
    })
}
