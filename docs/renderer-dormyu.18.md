# DH dormyu.18 — delivered underwater handoff repair

<!-- CURRENT-DH18-DELIVERY -->
## Current verified delivery — IL 0.6.0 / DH dormyu.18 / Zoom 2

DormLab's underwater renderer repair is installed, checked through actual
Prism/bridge and [published](https://github.com/dormyu/distant-horizons-for-horizon-client/releases/tag/dormyu.18). IL and Zoom Rust are unchanged;
new worlds retain descriptor11 / vanilla-natural-v7 and existing recipes remain intact.
This DH update needs no new world.

The reproduced pale bare seabed strip is absent in reviewed native16→8→16/DH64,
full-process reopen and native32→16→32/DH128 images. Native water-light/model
fixtures, two Rust clients and installed Prism passed. A separate distant
warm-ocean/deep-lukewarm-ocean colour boundary remains visible: saved biome data
and DH debug views distinguish it from the repaired native/DH handoff.
No universal renderer or owner aesthetic acceptance is claimed.

**Performance tests were waived by the owner.** Preserve native32/DH128 Medium,
sim8, maxFps260 (unlimited), VSync off, nativeLOD0, workers Auto and control off.
3058 owner-world files and settings were preserved. All managed files stay on E:;
one Cargo job, no concurrent full builds, Teams and owner applications untouched.

Runtime `33992c7848f91b2a3a730c5e0d267edf57e188d2461ed152aa73129b651a2047`; compiled DH `3a8aa870971a3213f8436fe8e299eb95d30b4c73`;
published source/docs `d4c1d537611b1b0790d2b81bd2708b1fcd0bdc2a`. [Evidence and limits](renderer-dormyu.18.md).
QA: `E:\Games\Minecraft\RustCore\qa\underwater-seam-aquatic-2026-11-11`.
The retained DH17 report and older generator snapshots keep their historical scope.


## Change and actual checks

The final runtime retains native kelp, kelp-plant, seagrass and tall-seagrass
models at width1 in the existing Medium-or-higher handoff ring. Geometry, UV,
texture layers, ambient occlusion and vertex lighting match the native fixture.
Sampled water and the adjoining floor attenuate skylight with depth; block light
and the tested roof/ice behavior are preserved. The resident-chunk cutout is shared.

- 70 standalone logic tests and Clippy passed before the native build.
- Exact final-library native hooks, four aquatic model cases and nine water-light
  cases passed. Library-only and rejected earlier candidates retain their status.
- Five native16→8→16/DH64 captures, a full-process reopen and five
  native32→16→32/DH128 captures were inspected. The former bare pale strip is absent.
- Two Rust clients received the same new IL world and block update; DH off/on,
  independent Zoom and clean client/server shutdown passed without transport errors.
- Actual installed Prism→bridge→Rust reached the named world type and generated
  a fresh isolated QA world. No owner world was opened by this Prism check.
- Installation and Prism preserved3058 owner-world files/settings; control is off.
- Own-fork main and annotated tag `dormyu.18` point to `d4c1d537611b1b0790d2b81bd2708b1fcd0bdc2a`;
  published source assets were downloaded and their hashes matched.

The initial installer completed its transaction and receipt, then failed only
while printing a Russian waiver to a cp1252 output stream. The installed binary,
metadata and before/after owner snapshots were read back successfully; it was
not reinstalled or rolled back. This is retained in `installation-output-note.json`.

## Visual scope and remaining limits

All handoff images are unedited1920×1080 native captures, Overworld smooth lighting,
no shader pack, native Horizon LOD off, simulation5 and capture cap30. The cap and
readiness states are not performance measurements. Owner settings remain32/128
Medium, simulation8, maxFps260(unlimited), VSync off. No FPS or loading-time claim.

The distant turquoise/blue line remains visible. Four projected pixel pairs and
the saved surface data identify warm_ocean (#43d5ee) versus deep_lukewarm_ocean
(#45adf2); both retain water state89 at height62–63. Most sampled pairs lie inside
the same DH tile, and debug detail boundaries do not coincide with the line.
Current colour blending spans only a few blocks; broad ocean-colour gradients
are not implemented by this repair. DH64 only masks the line with stronger fog.

More distant LOD detail remains approximate. Other lighting/shader modes, all
seeds, old hardware and a full heavy pack are not certified. Owner aesthetic
approval remains separate. Derived caches rebuild on first use; saved Minecraft
blocks, player edits and IL recipes do not migrate. [Actual before/after gallery](https://github.com/dormyu/distant-horizons-for-horizon-client/blob/dormyu.18/docs/validation-dormyu.18.md).

## Test the delivered instance

Open the desktop Prism shortcut → **DormLab**. An existing world can test the
renderer repair. For fresh IL geography, create a world and explicitly select
**Immersive Landscapes**. Native32 / DH128 Medium are already configured.
Allow initial derived-cache rebuilding to finish. **Z** holds Zoom; **Z + wheel**
changes magnification. `/dh status` identifies `0.1.0+dormyu.18`.

## Local evidence and rollback

QA root: `E:\Games\Minecraft\RustCore\qa\underwater-seam-aquatic-2026-11-11`. Rollback files: `before-dh18-install`; the old DH17 runtime and
source package are retained there. Receipt hashes below refer to the completed
delivery, not the historical generator0.6.0 tests.

- `native-build.json`: `f81d893b0924a26cdd18ea16b27d901390eb9403c2657aafc56c826d17590b39`.
- `delivery-selection.json`: `d66e81517d75fd200d12c559a7936593db4cbc3802f0d796482802e45b62a580`.
- `visual-review.json`: `becc840c32327ed0f885ce606cbd4827f2dbdba5caa92b4f5e0f3dac808060b9`.
- `waterline-diagnosis.json`: `a11c1d598c011321bee9062d02a79e3c3bd3862bf9a8c34e74b38666bbfb0cd3`.
- `native-functional-network.json`: `284298a6db433bd3bb489e2f48bd7b79d9775aab9fd57e8677ecb01e8618c0ba`.
- `installed.json`: `691acd0aedf908ef9d4e48c5b19df3475d4921393c9b60742be1cde46e829694`.
- `native-prism.json`: `d5a550e27825c5c34d307af2b8d0a7fb93ad902785d51f3432cc5482e663cd6c`.
- `prism-provenance.json`: `ee8cc534f322181d419f9754b5e5b30ee6021efdf5dff9fc24d088efb190e098`.
- `published-release.json`: `22411dd3297f238532d4e5c07d99ee2200d0bd3977671d3ba8b3d7f7ced21347`.
- `performance-waiver.json`: `a29c1e43e8c3b17395310feec6f81575447d702a098553aea50214a24fe57548`.

[DH17 delivery](renderer-dormyu.17.md), [IL0.6.0 validation](validation-0.6.0.md)
and the original v0.6.0 screenshots remain historical. This update does not
redesign villages or structures. Future structures read each world's saved recipe.
Original DH/Rust-port/Horizon/tooling authors and LGPL/GPL/MIT notices are preserved.
