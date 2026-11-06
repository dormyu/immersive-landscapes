// Private saved-world data is inspected in its owning module; no registry dump
// or extra public engine API is needed. V8 has no marker and returns None.
pub(crate) fn width(data: &super::GeneratorData) -> Option<i32> {
    crate::worldgen::immersive_landscapes_ecotones::recipe_width(
        data.densities
            .get("immersive_landscapes:ecotones_v1")
            .and_then(|s| serde_json::from_str::<f64>(s).ok()),
        data.densities
            .get("immersive_landscapes:ecotone_width")
            .and_then(|s| serde_json::from_str::<f64>(s).ok()),
    )
}
