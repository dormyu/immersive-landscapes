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

// Upstream body SHA-256 (LF): 34c666e00cb2954576485bb2052dbebc28cfb3a7ec7b16e42a7d13558221b889
fn immersive_landscapes_structure_build(
    state: &StructureState,
    context: &mut impl super::structure::start::StartContext,
    chunk: ChunkPos,
    wait: bool,
) -> std::ops::ControlFlow<Option<Starts>> {
    std::ops::ControlFlow::Break((|| {
        if let Some(found) = state.starts.get((chunk.x, chunk.z)) {
            return Some(found);
        }
        if context.is_old_chunk(chunk) {
            // Decoded once: the neighbours ask for these starts again and again.
            let starts = state.saved_starts_at(context, chunk);
            return Some(state.starts.insert_first((chunk.x, chunk.z), starts));
        }
        // Ring searches enlist other workers and wait for them: never hold a
        // chunk-work lock while helping, even when this chunk has no starts.
        let rings = immersive_landscapes_structure_rings::near(state, context, chunk);
        let work = state.start_work.at((chunk.x, chunk.z));
        let _work = match work.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::WouldBlock) if !wait => return None,
            Err(_) => work.lock().unwrap(),
        };
        if let Some(found) = state.starts.get((chunk.x, chunk.z)) {
            return Some(found);
        }
        let starts: Starts = Arc::new(
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
            .collect(),
        );
        Some(state.starts.insert_first((chunk.x, chunk.z), starts))
    })())
}
