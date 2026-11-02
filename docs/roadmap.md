# Roadmap

Execution order and acceptance gates: [detailed plan (Russian)](implementation-plan.ru.md).
Next action: **G0, integrate and launch the existing generator with Horizon and DH**.
The replacement G0-G3 plan covers world generation only. The old broad G0-G7 plan
and its time estimate were withdrawn after the owner's scope correction.
The owner's performance aspiration is 32 near chunks / 120 FPS with farther DH
terrain, adjustable for older hardware. It is not an achieved benchmark.

## Implemented in the standalone prototype

- [x] Mountain belt, massifs, foothills and continuous climate fields.
- [x] Ocean connectivity, drainage graph, runoff accumulation and lake spill levels.
- [x] Tributary convergence and explicit river cross-sections.
- [x] Positive delta fixture plus steep-mouth and out-of-region rejection tests.
- [x] Spatially indexed surface queries with channel beds, banks and level lakes.

## Before the first playable experimental mod

- [ ] A versioned Horizon native bundle, using the same source for chunks and LOD.
- [ ] Explicit finite-world experiment and a design for seamless neighbouring regions.
- [ ] Height-layout, lighting, collision and safe-spawn validation for large relief.
- [ ] Water voxelization, river junctions and actual delta/estuary terrain.
- [ ] Continuous biome materials and vegetation transitions.
- [ ] Save/reload and deterministic generator-version handling.
- [ ] Solo and two-client Rust-server integration tests, including player edits.
- [ ] CPU/RAM/frame-time/network budgets with room for other heavy mods.

## Later

Refine terrain and hydrology according to actual in-game results. Survival systems,
progression, classes, combat and DormCore are outside this generator task.
