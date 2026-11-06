# Immersive Landscapes 0.5.0 handoff

Working native mod for Horizon f42ef42 / Minecraft 26.3, IL224a831 / DH623dd71 /
official Zoom725ff7a. Final runtime `3bb2fb54bc677e1ba3fe3bfdf4f0b18b034c55fbb2285111ab8831a81da4efcd`.
New worlds: descriptor 10 / vanilla-natural-v6. Saved4–9 recipes stay unchanged;
v4–v8 runtime fixtures pass; v9 has no separate fixture. Java support, survival,
classes, DormCore and structure redesign are outside this generation release.

Generation, native river/lake/ecotone gates, actual scenes, final solo/two-client/
world-switch and near32/DH128 checks are completed. Read
[validation](validation-0.5.0.md) for exact receipts, cache warmth and FPS limits.
Do not repeat completed porting, rebuild a different engine, or represent a
library/preview/prepared map as the playable mod.

Source documentation precedes package/install/Prism/publication. For those later
actions read the v0.5.0 release notes and installed native-build.json. Local QA
`RustCore/qa/release-0.5.0-final-2026-11-06` has guarded delivery and publication
scripts, exact backups, source/runtime hashes and fresh launcher verification.
Resume only a genuinely incomplete receipt; never rerun a completed transaction.

Known limits for subsequent work: initial DH fill can leave a temporary partial
horizon; water/material detail may differ; some narrow channels are straight or
angular; vegetation blending covers ten temperate/cool biomes and does not blend
all tints. First-use fast flight did not sustain120 FPS. Owner aesthetic and
heavy-pack acceptance remain unrecorded. These are documented next improvements,
not reasons to relabel the working release as a prototype.

Future structures must read the saved descriptor/recipe and native terrain.
Never rewrite an old world to v10. Use a new Immersive Landscapes world to see
the new hydrology/ecotones. Z and Z+wheel use the official separate Zoom mod.

All project files/caches/temp/worlds remain on E:. Source Enter-RustDev.ps1,
stable Rust, one Cargo job and one full-build/game owner. Keep Teams and owner
applications open. Preserve worlds/accounts and near32/DH128 Medium/sim8/
maxFps260/VSync off/control off. Never push DH or tooling upstream. Own author
and GitHub identity is dormyu; source licenses and credits remain separated.
