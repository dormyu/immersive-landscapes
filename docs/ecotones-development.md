# Biome ecotones — isolated 0.5.0-dev.1 candidate

This branch starts from the checked 0.4.1 code `8a5460e`. It is not installed,
merged or visually accepted. The primary 0.4.1 delivery remains separate.
New worlds use `vanilla-natural-v5`, descriptor 9, with the explicit saved
`ecotones_v1 = 1` marker and `ecotone_width`. Existing v8 recipes have no marker
and keep the previous generation paths. The watershed algorithm is unchanged.

## Native integration

The pinned f42 API provides these concrete hooks:

- `generator::GeneratorWorker::decorate`: a scoped context around the native
  FEATURES pass over a 3×3 chunk region. An unwind-safe, thread-local guard masks
  nested old/vanilla worlds and restores the previous scope on return or error.
- `generator_decoration::Decoration::new_with`: append scheduler-only vegetation
  aliases after the native plan is built. `FeatureDecorationPlan.steps` is never
  rebuilt or edited, preserving native step-local feature indices and seeds.
- `generator_decoration::World::chunk_biomes`: introduce only those aliases whose
  pair touches the bounded transition field. Aliases do not enter BiomeChunk,
  the biome registry, network/save data, gameplay IDs or structure queries.
- `generator_decoration::World::biome_allows_feature`: deterministic weighted
  native vegetation membership for forest/plains and taiga/meadow only. The
  native feature placement stream is not consumed by the ecotone decision.
- Existing proto/surface/column/grid hooks: one shared dry ground policy after
  hydrology. Every IL river profile (including dry banks), native water, rock,
  snow and unsupported top is excluded. Forest/plains retain their grass; the
  taiga/meadow band can mix supported grass/podzol patches without changing height.

Constructor and scheduler integration use cancellable HEAD wrappers with one
scoped reentry into the original function. The pinned mixins tool cannot apply
RETURN to the constructor's closure/`?` control flow. Trait methods are addressed
as `<World<'_> as DecorationWorld>` and `<World<'_> as PlacementContext>`.
Scopes distinguish hook kinds as well as pipeline/object identity.

The vegetation allowlist contains verified 26.3 tree, grass, flower, fern, berry
and mushroom roots. Only step 9 is admitted. Missing roots, unknown executable
graphs and foreign identifiers retain native behavior. Graph validation rejects
ore, lake, structure and other nonvegetation feature types even if a resource
reuses a familiar name. Namespace collisions never overwrite a scheduler entry;
aliases are rechecked before use.
The actual 26.3 forest/plains/taiga selectors include fallen trees, which are
accepted as vegetation alongside standing trees and simple plant placement.

## Cost and determinism

An absolute world-coordinate lattice and tent kernel provide continuous pair
weights. A bounded 4096-anchor cache shares those samples; native surface queries
reuse the existing bounded cache and bypass wrappers. Locks are released before
sampling. Exact native columns for ground painting are requested only inside an
eligible transition band. There is no tick/frame hook or per-block distant-biome
neighborhood scan. Native plant density and species are selected by a stable
seed/position/feature-name hash, not chunk-local ecotone randomness.
Ground eligibility uses the same zoomed native biome for near/scalar/grid paths;
raw proto quart IDs are not used as a separate near-only boundary filter.

## Source checks completed

`python -B -X utf8 tools/check_ecotone_recipe.py` confirms that all 13 v8 density
inputs and `watershed.rs` are byte-identical to `8a5460e`, and the only new density
inputs are the two explicit ecotone markers. This is source/recipe evidence,
not native runtime equivalence. No engine stage or installed pack was changed.

All six standalone ecotone policy tests pass, as does targeted offline locked
Clippy with warnings denied (`--test native_ecotones`). These cover the exact
version marker, bounded continuous weights across negative chunk boundaries,
deterministic vegetation admission, nonvegetation rejection, unrelated biome
exclusion and soil policy. Receipts are in this worktree's ignored `target/`
directory (`ecotone-policy-tests.log`, `ecotone-policy-clippy.log`). Native hook
compilation and its separate scheduler/scope tests have not run yet.

The read-only native mixins plan also passes against the revision-marked f42
source: 15 injections and 10 modules, including all four new ecotone hook targets.
The plan and receipt are `target/ecotones-plan.json` and `target/ecotones-plan.log`.
This confirms target resolution and injection planning, not native Rust type
checking or game behavior. No mixins were applied to an engine stage.

The independent future river work will use `rivers_v5 = 1`. Descriptor 9 does not
contain that marker; a combined ecotone/new-river recipe will use descriptor 10
so existing v8/v9 geography is preserved.

## Validation still required

- Compile the native hooks in an isolated combined stage. The primary DH10
  delivery and its installed-pack checks remain separate.
- Run native scheduler-order/seed and scope-unwind tests included in the modules.
- Compare native v8 terrain/decorations and save/reopen behavior with the exact
  base; marker absence must leave the native ecotone path inactive.
- Find actual forest/plains and taiga/meadow boundaries, including chunk edges;
  compare native/scalar/grid ground, two-worker and cache-reopen results.
- Inspect actual new-world vegetation and DH views. Native grass tint and every
  other biome pair are not claimed fixed by this limited vegetation pass.
- Measure generation time, memory and paired frame times; repeat solo/save and
  two-client checks before installation or publication.

Neither policy tests nor source implementation establishes visual acceptance.
Inland drainage, deltas and broader ecotone coverage remain separate open work.
