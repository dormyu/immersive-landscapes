// Original dormyu integration, MIT. Populate the exact full result; the
// original rings() immediately returns its existing cached reference.
fn immersive_landscapes_structure_rings_help(
    state: &StructureState,
    context: &mut impl super::structure::start::StartContext,
) {
    immersive_landscapes_structure_rings::full(state, context);
}
