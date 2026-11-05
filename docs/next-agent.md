# Current checkpoint — IL0.4.0 / DHdormyu.7 / Zoom 2.0.0

DormLab runtime `add9b6a55dab0fae1b76dae5be395a7076a61ef7618515209edbbce0b7fee62d`, Horizon f42ef42 / Minecraft 26.3.
Working mod, active development. Built Rust code16250c2; later commits are docs.
New world type Immersive Landscapes → vanilla-natural-v3 / descriptor7. Saved
v4/v5/v6 and vanilla recipes remain unchanged. All project files/temp/cache E:.

[Implementation](hydrology-development.md), [scoped validation](validation-0.4.0.md).
22 calculation tests/Clippy, native seeds/cache/two workers, 3093 wet points,
RU/EN menu/save/old recipes, real game water, two Rust clients and installed Prism
pass. No normal/far decode warnings in the checked network run. Control is off.
5127 owner save files and preferences preserved. Do not close Teams.
Personal near32/DH128 Medium/sim8/maxFps260(unlimited)/VSync off/nativeLOD0/workers Auto preserved.
Short warmed cap120 flight119.91 FPS/P998.85 ms is not heavy-pack acceptance.

Next: owner review of a NEW v7 world; bounded full inland catchments/outlets,
delta branching and biome ecotones. Current graph is bounded coastal coverage;
not every noise valley becomes a river. Keep Minecraft scale/native decoration.
DH7 still has potential vegetation/material/detail transitions; do not promise
all-seed zero seams. Future structures target versioned IL geography, not yet rewritten.
Constructor recursive scope is guarded, but additional mods hooking it need review.
No survival/classes, DormCore framework or new engine in this mod's scope.

Exact files: ../RustCore/packs/DormLab/rustcore/native-build.json.
QA: ../RustCore/qa/hydrology-v3-2026-11-04. Rollback: before-hydrology-install
to prior7292006b/IL0.3.1. Initial network false-positive is revoked and archived;
the corrected add9b6a5 client passed strict far-warning checks. Historical v4
owner acceptance remains revoked: that feedback concerned vanilla in EpicReference.

Source Enter-RustDev.ps1, stable, one Cargo job. Full builds must not overlap.
The combined46844bc mixer patch is required; DH's older patch alone is insufficient.
Never push the DH/tool upstream remotes. Public own repo is dormyu/immersive-landscapes.
