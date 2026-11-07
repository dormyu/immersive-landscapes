// Adapted from Horizon f42ef42ef6ceccb3c6a20bea2481d87949a51c81
// crates/horizon-world/src/worldgen/generator_structures.rs.
// Original behavior retained; only ring lookup is localized by dormyu.
//
// MIT License
//
// Copyright (c) 2026 Ludovic Marie and the Horizon contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Upstream body SHA-256 (LF): 2a9958cb5ef5474a7e7c13ef3988369cd244fcb80fd471aac114a1eb467da64f
fn immersive_landscapes_structure_saved(
    state: &StructureState,
    context: &mut impl super::structure::start::StartContext,
    chunk: ChunkPos,
) -> std::ops::ControlFlow<Starts> {
    std::ops::ControlFlow::Break((|| {
        let saved = context.saved_starts(chunk);
        let mut generated: Option<Vec<Arc<StructureStart>>> = None;
        let starts = saved
            .into_iter()
            .filter_map(|start| {
                let index = (state.registry.structures.iter()).position(|s| s.name == start.name)?;
                let def = &state.registry.structures[index];
                if !start.pieces.is_empty()
                    && let Some(pieces) = start
                        .pieces
                        .iter()
                        .map(|piece| state.saved_pool_piece(piece))
                        .collect::<Option<Vec<_>>>()
                {
                    return Some(Arc::new(StructureStart::new(index, def, chunk, pieces)));
                }
                if let Some(bounds) = start.bounds {
                    let made = generated.get_or_insert_with(|| {
                        let rings = immersive_landscapes_structure_rings::near(state, context, chunk);
                        create_starts(
                            &state.registry,
                            &state.resources,
                            &state.bindings,
                            context,
                            &state.possible_sets,
                            &rings,
                            chunk,
                        )
                        .into_iter()
                        .map(Arc::new)
                        .collect()
                    });
                    let same = made.iter().find(|made| {
                        made.name == start.name && pieces_box(&made.pieces) == Some(bounds)
                    });
                    if let Some(same) = same {
                        return Some(same.clone());
                    }
                }
                let [x0, y0, z0, x1, y1, z1] = start.bounds.unwrap_or([
                    chunk.x * 16,
                    0,
                    chunk.z * 16,
                    chunk.x * 16 + 15,
                    0,
                    chunk.z * 16 + 15,
                ]);
                Some(Arc::new(StructureStart {
                    structure: index,
                    name: start.name,
                    chunk,
                    pieces: Vec::new(),
                    bounding_box: crate::worldgen::structure::BoundingBox::new(
                        [x0, y0, z0],
                        [x1, y1, z1],
                    ),
                }))
            })
            .collect();
        Arc::new(starts)
    })())
}
