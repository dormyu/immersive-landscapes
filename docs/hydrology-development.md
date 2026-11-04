# Native hydrology work — 4 November 2026

This is active candidate development, not an installed or accepted 0.4.0 release.
The installed laboratory remains IL0.3.1 + DHdormyu.7, client7292006b.
Branch: `feature/native-hydrology-v3`; QA: `RustCore/qa/hydrology-v3-2026-11-04`.

The candidate adds a Rust coastal drainage graph over the existing vanilla-natural
terrain, with priority-flood depression handling, contributing-area accumulation,
common confluences and graded channel profiles. Spatial bins and bounded, shared
basin caches keep repeated queries cheap. World generation, surface columns and
native distant grids consume the same riverbed/water profiles. Native decoration
runs after water is placed. This is not a replacement height-only world generator.

New worlds use vanilla-natural-v3 / descriptor7. Saved v4/v5/v6 recipes are retained;
their density registry has no watershed-v1 marker, so the adapter stays inactive.
Existing noise valleys are not all converted by the bounded coastal feature.
Delta branches, full inland coverage, biome ecotones and performance acceptance
remain unfinished. Do not turn a passing calculation test into visual acceptance.

Twenty-two calculation tests pass (ten exercise the actual native support modules): descending acyclic drainage, runoff
conservation, true receiving-water outlets, query-order/cache independence,
unique ownership of coastline roots, no invented outlet on a closed highland,
persisted DEM reopen/corruption/seed/recipe invalidation, wet-channel priority,
full-strength published paths, lake crossings and top-first material runs.
Native integration is being checked; calculation tests do not replace game checks.

The pinned mixins tool rejects generic Pipeline<R> injection targets. It also
rejects state constructor rewriting when the constructor contains unsupported
control flow. Current implementation avoids both: a scoped nongeneric world
constructor, router binding, weak pipeline-to-atlas records, and nongeneric
worker return hooks. The world constructor uses a guarded recursive entry;
compatibility with additional mods hooking that same constructor still needs
review before those mods are included. There is no gameplay-frame hook.

Each running instance owns its local derived cache under
`horizon-mods/.horizon-mod-data/immersive_landscapes/hydrology-v1` in the instance
working directory. The developer-only `HORIZON_IL_CACHE_DIR` overrides that root.
No absolute host path is included in the worldgen registry or sent to another
client. The expensive input DEM is persisted with engine/input identity, seed,
grid geometry and checksum validation.
Graphs are rebuilt from it; corrupt/obsolete files are disposable and recomputed.
The core cache tests and full-game save/reopen pass; gameplay cost is being checked.
The block pass finishes narrow cuts after native density interpolation, removes
leftover roofs and backs sediment with a four-block liner. Far columns mirror
that cut and liner. No installed worlds, clean RustCore, accounts or preferences
were changed by this candidate work. All source, stage, cache and QA stay on E:.

The native wrapper retains Horizon's bulk density-sampling path and performs
one profile query per horizontal volume column. The original per-point path
remains for individual queries. The optimized native probes over three seeds, cache reopen and two workers pass.
A wider native-proto route test first found197 dry points/1603; the corrected
full-strength domains and channel priority produced1218 wet points/1218 with
zero missing profiles. This is scoped centreline coverage, not all terrain or lakes.
The actual menu/save checks retained old v4/v5/v6. Two-client testing found a
separate span-order bug that its first smoke result omitted: generator runs are
top-first and the client reverses them into bottom-first network order.
The adapter now retains that contract; two pure regression tests pass. Corrected
client add9b6a5 has been built and passed normal menu creation/save/reopen and
old v4/v5/v6 recipe retention. Three native centreline routes contain3093 wet
points/3093 with zero missing profiles. The strict zero-far-warning network run
and matched near32 performance comparison are still pending.
The installed laboratory has not been replaced by this candidate.

Concept references, not copied implementation code:

- [Barnes, Lehman and Mulla, Priority-Flood](https://doi.org/10.1016/j.cageo.2013.04.024):
  ordered depression filling and drainage labels.
- [Davis, Plate-Local River Generation](https://arxiv.org/abs/2610.00301): bounded
  drainage domains and lazy shared preprocessing. Our terrain remains native;
  these coastal domains do not replace Minecraft's continents with tectonic plates.

Original implementation: dormyu, MIT. Existing DH/Zoom licenses stay separate.
