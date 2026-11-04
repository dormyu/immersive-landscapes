fn immersive_landscapes_hydrology_router(
    router: NoiseRouter,
    compiler: &mut DensityCompiler,
) -> std::ops::ControlFlow<Result<RouterSamplers, CompileError>> {
    use crate::worldgen::immersive_landscapes_hydrology as h;
    if !h::building() {
        return std::ops::ControlFlow::Continue(());
    }
    let result = (|| {
        let mut samplers = router
            .functions
            .iter()
            .map(|id| compiler.get_sampler(*id))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .expect("eight native router fields");
        h::wrap_router(&mut samplers);
        Ok(RouterSamplers { samplers })
    })();
    std::ops::ControlFlow::Break(result)
}
