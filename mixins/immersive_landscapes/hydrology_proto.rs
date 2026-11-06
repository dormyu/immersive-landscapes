fn immersive_landscapes_hydrology_proto(
    value: Result<ProtoChunk, GeneratorError>,
    worker: &GeneratorWorker,
) -> Result<ProtoChunk, GeneratorError> {
    value.and_then(|mut proto| {
        let key = std::sync::Arc::as_ptr(&worker.pipeline) as *const () as usize;
        if let Some(atlas) = crate::worldgen::immersive_landscapes_hydrology::for_pipeline(key) {
            crate::worldgen::immersive_landscapes_hydrology::fill_proto(
                &mut proto,
                &atlas,
                worker.pipeline.data().registry,
            )
            .map_err(|error| {
                resource(format!("Immersive Landscapes river placement: {error:?}"))
            })?;
            crate::worldgen::immersive_landscapes_ecotone_runtime::fill_proto(
                &mut proto,
                &atlas,
                worker.pipeline.data().registry,
            )
            .map_err(|error| resource(format!("Immersive Landscapes ecotone ground: {error:?}")))?;
        }
        Ok(proto)
    })
}
