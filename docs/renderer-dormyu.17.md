# DH dormyu.17 — delivered renderer update

## Current verified delivery — IL 0.6.0 / DH dormyu.17 / Zoom 2

DormLab's renderer update is installed, checked through the real Prism/bridge,
and [published in dormyu's fork](https://github.com/dormyu/distant-horizons-for-horizon-client/releases/tag/dormyu.17). IL 0.6.0 Rust and Zoom 2 Rust remain
unchanged; new IL worlds still use descriptor 11 / vanilla-natural-v7. Existing
world recipes and blocks are not migrated. This renderer update does not require
creating a new world; new worlds are required only for a different IL recipe.

DH 17 retains narrow supported trunk footprints, interpolates compatible water
tints, and optimizes planner/perimeter work. These are bounded renderer changes,
not a new terrain generator or village redesign. Solo/save/full process restart,
two clients, native 16 → 8 → 16 / DH 64 handoffs, the original canopy camera and
installed Prism passed on this runtime. **Performance tests were omitted at the
owner's request** (`performance_tested=false`). Installed native 32 / DH 128 settings
remain unchanged. Read [the evidence, scope and remaining limits](https://github.com/dormyu/distant-horizons-for-horizon-client/releases/tag/dormyu.17) for the
visual profile; owner aesthetic approval and full-pack performance remain unverified.

Runtime `a6a3fcfad445c5af67bf9a5e439d3512621e1cae296734b80e090c6f7ade6b07`; compiled DH `47888d3dec2edda918c367a5a60d3afc8f9f2fd0`; IL code-equivalent source
`72ede8a6b2ff6505dd1af74ac6c42199f19f46e0`; published documentation `23b16d7e6f2d53de6284c1cc28460e340b3ad47e`. QA: `E:\Games\Minecraft\RustCore\qa\far-canopy-optimized-2026-11-10`.
The installed transaction preserved 3058 owner-world files and settings;
control is off. All managed files remain on E:; one Cargo job and owner apps
preserved. Prior IL 0.6 / DH 16 source snapshots below retain their original scope.

## Exact runtime and delivery evidence

Status read back on 2026-11-10. Engine `f42ef42ef6ceccb3c6a20bea2481d87949a51c81`,
Minecraft 26.3. Built IL `72ede8a6b2ff6505dd1af74ac6c42199f19f46e0`, DH `47888d3dec2edda918c367a5a60d3afc8f9f2fd0`, Zoom `725ff7a41017c40104273e407264e0ebee87e567`.
The local source package and published release assets match the installed hnmod.
Published documentation commit: `23b16d7e6f2d53de6284c1cc28460e340b3ad47e`. Original authors and licenses remain:
James Seibel/Distant Horizons contributors; Deniz Şahin/Rust-port contributors;
dormyu adaptation; Horizon bodies by Ludovic Marie/contributors under MIT.
The DH adaptation remains LGPL-3.0-only and separate from MIT Immersive Landscapes.

Receipt SHA256 values (under `E:\Games\Minecraft\RustCore\qa\far-canopy-optimized-2026-11-10`):

- `reports/native-build.json`: `9a977662a605974f377e42d7fcc9900717f6d6e2f5887fda3846b50f6dc76a4c`.
- `reports/installed.json`: `4f9646921cade1e01e3d4fd93811f3331b63ce96aa6bf35a378d75f9718b49fa`.
- `reports/native-prism.json`: `ec7273cbaea8672a8c4f69dcc9733cd2f6d8ac93752a3dedd3a5fdc7e760f2ad`.
- `reports/prism-provenance.json`: `5b9f7d0218b5ba8644a12d361f5ed4aa0ed9a3406741201b5ea91e64aea0c78e`.
- `reports/published-release.json`: `8c2976c0d472636452ed16db4490ffcaa564f3bcf1dc6fb28d6c83df74664cd1`.
- `reports/delivery-selection.json`: `d773c92a93ccd7fdccc487637d970410d712da34f7a65c2b53283cda602e729c`.
- `reports/native-functional-solo.json`: `918426752679a5bdec236a75b880b6ff45147c90276e16a9476ecac6ca075994`.
- `reports/native-functional-network.json`: `2b547fb9a4b8a44ce6b4b0abf48f86db550233b24d207af7d46172f81a6e58b1`.
- `reports/delta-handoff-visual-02.json`: `2284c65c827f918038eb58f96f0858c33bc965340a86f65b8f2eefac9e2431e1`.
- `reports/visual-review.json`: `5b9c3f2a503879dce7ad0de04cc1a717bed62a5d93e4bf3f3d3a3532fc755b5c`.
- `water-tint-native-fixture-02.json`: `bd9bede7fe1dad9498a34129d8a19a1374ce9f9040e5be86b1d5b5036b540364`.
- `reports/performance-waiver.json`: `6126fd362a8aef6d57da2a4483091ee22eae4f17474c9d2b6b9de88ddf215e81`.

## Performance tests explicitly waived by the owner

FPS/performance tests were omitted at the owner's request. The selected immutable
waiver records owner_requested=true and performance_tested=false for this exact runtime.
There is no performance PASS, current FPS/1% low/frame-time claim or accepted
cold/warm loading measurement for this delivery. Interrupted/failed attempts
remain unchanged and are excluded from accepted evidence.

Installed native 32 / DH 128 settings are preserved. Visual QA uses an isolated
world and cache from this runtime at native 16 → 8 → 16, DH 64 Medium,
1920 × 1080 and a 30 FPS cap; the cap is a setting,
not a measured FPS result. The original canopy camera is checked separately in
the same visual run. These functional/visual results do not certify performance
at the installed settings, a future heavy pack or owner aesthetic approval.
The DHH2 derived cache still needs rebuilding on first use; its time is not
benchmarked for this delivery.

## Observed limits from the exact candidate's visual review

- Underwater LOD still differs visibly in seabed detail, aquatic vegetation and lighting; the pale diagonal patch is a remaining renderer limitation, not a resolved water-tint defect.
- Coarse distant vegetation, terrain steps and native biome material/colour boundaries remain visible. This check covers the selected cameras and does not certify every seed, distance or scene.
- First-use DH cache rebuilding and newly explored terrain require preparation. Performance tests were explicitly omitted; no new FPS, frame-time,32/128 performance or heavy-pack guarantee is made.
- The owner has not given aesthetic approval of this exact delivered candidate.

The visual review covers the listed scenes and retains the limitations above.
Actual world blocks and saved IL recipes are unchanged by this renderer update.

## Retained history

[IL 0.6.0 validation](validation-0.6.0.md), [matched generation comparison](comparison-0.6.0.md),
tag v0.6.0 and its screenshots remain the original DH 16-era generation evidence.
They are not relabeled as executions of this renderer. The earlier candidate in
`far-canopy-2026-11-10` is also retained; current evidence is the optimized QA
directory named above. Source/asset publication occurred in the
[own DH fork](https://github.com/dormyu/distant-horizons-for-horizon-client), preserving the upstream project and credit chain.
