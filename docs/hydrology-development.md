# Native coastal hydrology — 0.4.0

This native Rust adapter is installed in the checked DormLab0.4.0 set. It extends
vanilla-natural terrain at Minecraft scale. New worlds use descriptor7/v3; old
v4/v5/v6 recipes have no watershed marker and remain inactive in this adapter.
See [validation](validation-0.4.0.md) for exact scope and measurements.

Priority-flood depression handling, contributing-area accumulation, shared
confluences and graded channels operate in bounded coastal domains. Only the
full-strength interior publishes channels, so fading domain borders do not
produce uncut dry river segments. Wet cores take priority over nearby bank
feathers; lake crossings preserve cut/liner continuity. Full inland drainage and
delta branching are not implemented; native biome vegetation boundaries remain.

Native density wrappers retain Horizon's bulk sampling path and query one profile
per horizontal column. Actual blocks, public surface/columns and distant grids
share bed/water profiles, roof trimming and a four-block liner. Native decoration
runs afterward. Top-first material runs are retained for the client wire adapter.

Each running instance owns its derived cache under
horizon-mods/.horizon-mod-data/immersive_landscapes/hydrology-v1 in its working
directory. HORIZON_IL_CACHE_DIR is a developer-only override, not registry data.
The raw input DEM is persisted with engine/input identity, seed, grid and checksum;
graphs rebuild from it. Invalid/corrupt files are disposable and recomputed.
Bounded shared caches, spatial bins and thread-local numeric profiles avoid
unbounded retention. There is no gameplay-frame hook.

The pinned mixins API rejects generic Pipeline<R> targets and some state
constructor rewriting. The adapter uses nongeneric worker hooks, weak
pipeline-to-atlas records and scoped world construction. Guarded recursive
constructor entry can execute other mods' hooks twice; review compatibility
before adding another mod that hooks this constructor. Current DH/Zoom do not.

Original implementation dormyu, MIT. Concept references, no implementation copied:
[Priority-Flood](https://doi.org/10.1016/j.cageo.2013.04.024) and
[Plate-Local River Generation](https://arxiv.org/abs/2610.00301). The latter inspires
bounded domains/lazy preprocessing, not a claim that this mod simulates plates.
