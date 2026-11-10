# Immersive Landscapes 0.6.0 handoff

<!-- CURRENT-DH17-DELIVERY -->
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
remain unchanged. Read [the evidence, scope and remaining limits](renderer-dormyu.17.md) for the
visual profile; owner aesthetic approval and full-pack performance remain unverified.

Runtime `a6a3fcfad445c5af67bf9a5e439d3512621e1cae296734b80e090c6f7ade6b07`; compiled DH `47888d3dec2edda918c367a5a60d3afc8f9f2fd0`; IL code-equivalent source
`72ede8a6b2ff6505dd1af74ac6c42199f19f46e0`; published documentation `23b16d7e6f2d53de6284c1cc28460e340b3ad47e`. QA: `E:\Games\Minecraft\RustCore\qa\far-canopy-optimized-2026-11-10`.
The installed transaction preserved 3058 owner-world files and settings;
control is off. All managed files remain on E:; one Cargo job and owner apps
preserved. Prior IL 0.6 / DH 16 source snapshots below retain their original scope.

Do not repeat the completed installation/publication or substitute a previous renderer
candidate. The following handoff is retained as the original IL0.6/DH16 snapshot.

## Historical IL0.6.0 / DH16 handoff

Source/runtime validation snapshot: 10 November 2026. Working native mod,
new-world descriptor **11 / vanilla-natural-v7**. Generation scope is mountains,
connected waterbodies and compatible vegetation transitions at Minecraft scale.
Village/structure redesign belongs to a later project stage.

## Pinned implementation and evidence

- IL code `c43610a2fc73b2d092930ad47c4ba5b60ebe83db`.
- Horizon `f42ef42ef6ceccb3c6a20bea2481d87949a51c81`, game 26.3.
- DH `0.1.0+dormyu.16`, source/documentation `6d9744701eef2c4bf9ab0f1643caca7d152cd083`.
- Official Zoom 2.0.0, `725ff7a41017c40104273e407264e0ebee87e567`.
- Frozen client `4d7c4fec1e668fff43272a25696cd5ffe5b8846e78038382053e5e7b6c9e6a0c`.
- QA root `RustCore/qa/completion-2026-11-08-bankfix`.

Current source passes 71 standalone tests/Clippy, the exact eight-column bank
regression, bounded native valley/delta runs, five native vegetation boundaries,
71 donor graphs and determinism repeats. Four dry native65 shores now end at63;
four adjacent water columns remain unchanged. The new optimized client passes
named world creation, fresh v11 save/full process restart, saved-v10 recipe
preservation, vanilla toggles and two-client Rust loopback checks.

Current pond42 gameplay verifies nine loaded water blocks and the descending
flow. Four regional game images have been reviewed. The repaired delta passes
32→8→32 with 10 real scene/status PNGs and 16 before/after block observations.
The original failed receipt is preserved: a separate exact screen-oracle
revalidation accepts intentional chat overlays without altering raw observations
or claiming a new game run. Read [validation](validation-0.6.0.md).

The ocean band is classified: equal-height gen_step8 water changes native
biome tint from deep_lukewarm_ocean to warm_ocean, matching three pixel boundaries
in both native32/native8 images. It is not the repaired water wall or an actual/
predictive seam. Interpolating water tints is a separate possible improvement;
no causal visible defect was proven for the separate underwater-lighting finding.

## Completed runtime gates and delivery procedure

The cold/warm measurements, four-leg1.92km route and current image review are
complete; do not repeat them merely because this source document predates the
installation transaction. Read the exact bounded results in
[validation](validation-0.6.0.md) and the [matched comparison](comparison-0.6.0.md).

All38 current game PNGs were actually reviewed, covering all30 mandatory
images and8 supplementary captures. The targeted bank/flow/regional-vegetation
and geometry-handoff review passes. Coarse water colours, pale distant
underwater patches and isolated predictive far-canopy fragments remain visible
DH limitations; this is not complete renderer or owner aesthetic acceptance.
The final `reports/visual-review.json` records those findings explicitly.

The remaining renderer work includes isolated predictive canopy spans beyond
native32: SURFACE/detail9/stride8 samples can retain jungle leaves while missing
thin trunks. This is not evidence of floating blocks in actual saved chunks.
Do not hide it under a global foliage/rendering PASS or assume waiting fixes it.

For delivery, freeze a documentation-only descendant, package and validate its
unchanged source bundle, perform guarded installation and actual Prism readback,
then merge/publish and verify downloaded asset hashes. Package/install/publication
bind one documentation commit; later delivery evidence belongs to release notes
and local provenance. Update Obsidian from those actual receipts.

Final delivery status comes from the release notes and local `installed.json`,
`native-prism.json` and `prism-provenance.json`, not inference from this earlier
source snapshot. Do not claim distribution from a successful build.

## Preserve and do not repeat

The old bfbaa93/ac2e4919 candidate in `completion-2026-11-08` is historical; its
rendered bank defect motivated c43610a and its stopped performance run supplies
no accepted FPS. The [0.5.1 streaming fix](validation-0.5.1.md) remains inherited
with its own evidence. Do not repeat broad porting/research or silently change
saved recipes. Existing v10 stays v10; descriptors4–9 remain supported but were
not all freshly retested in 0.6.0.

All files/temp/caches/worlds stay on E:. Source Enter-RustDev.ps1 before Rust;
stable toolchain, one Cargo job, one root-owned full build/game slot. No Teams or
other owner apps closed. Preserve saves/accounts/settings and rollback. Keep the
delivered profile actual32/DH128 Medium/simulation8/maxFps260 (unlimited)/VSync off,
with control off. Owner aesthetic and heavy-pack acceptance remain separate.
Git author is dormyu; preserve CREDITS and licenses; no DH/tooling upstream push.
