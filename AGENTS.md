# Immersive Landscapes implementation

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
published source/docs `d4c1d537611b1b0790d2b81bd2708b1fcd0bdc2a`. [Evidence and limits](docs/renderer-dormyu.18.md).
QA: `E:\Games\Minecraft\RustCore\qa\underwater-seam-aquatic-2026-11-11`.
The retained DH17 report and older generator snapshots keep their historical scope.

## Historical 0.6.0 / DH16 source/runtime validation

Working native mod, descriptor11 / vanilla-natural-v7. IL code
`c43610a2fc73b2d092930ad47c4ba5b60ebe83db`, engine f42ef42,
DH0.1.0+dormyu.16 source/documentation6d974470, unchanged Zoom725ff7a.
Frozen runtime `4d7c4fec1e668fff43272a25696cd5ffe5b8846e78038382053e5e7b6c9e6a0c`.
QA: `../RustCore/qa/completion-2026-11-08-bankfix`.

71 standalone tests/Clippy, exact dry-bank protection and unchanged native water,
bounded M1/M2 native probes, solo/save/full restart/v10 preservation, two-client
Rust loopback, pond42 flow and regional captures pass. Delta32→8→32 has ten
original reviewed PNGs and16 block observations. Its original failed screen
oracle receipt is retained; exact hash-bound revalidation accepts intentional
chat overlays without a new native run. The visible diagonal ocean band is
proven biome water tint, not a geometry or actual/predictive source seam.

Three current matched A/B pairs are reviewed and repository PNG hashes match the
originals. Read the versioned validation report for the current performance and
final image-review gate results. Use [next-agent](docs/next-agent.md),
[validation](docs/validation-0.6.0.md) and
[completion milestones](docs/completion-milestones.ru.md); do not repeat completed
porting or silently relabel old bfbaa93/ac2e4919 or 0.5.x evidence.

Cold/warm native32/DH128 and four further480-block legs completed;38 actual
PNGs were reviewed. Coarse DH water colours/underwater patches and isolated
predictive far canopies remain open renderer findings. Do not claim all foliage,
all render distances or owner aesthetics accepted. Read the final release notes
and local delivery receipts before repeating completed work.

Package/install/publication pin one documentation commit and unchanged native
source bundle. Complete runtime evidence before that freeze. Later installation,
Prism and publication readback belong to final release notes/local provenance;
source validation is not proof of installed delivery. Preserve saved recipes,
near32/DH128 Medium/simulation8/maxFps260 (unlimited)/VSync off/control off,
owner data and the verified rollback. Owner aesthetic/heavy-pack acceptance is
not inferred from technical checks. Versioned 0.5.x documents remain historical.

- Deliver a playable **Horizon native mod**, not a replacement engine or a CLI-only
  terrain library. The existing Rust library is an internal component of that mod.
- Scope is ONLY world generation: realistic mountains, connected rivers/waterbodies,
  seamless biome transitions, and matching DH terrain with low overhead. Do not add
  survival systems, progression, classes or a DormCore framework to acceptance.
- Latest direction (2026-11-03): ordinary Minecraft landscapes following natural
  rules. Use the native vanilla generator as the foundation; preserve its scale,
  climate, decoration and varied biomes. Current vanilla-natural-v7 preserves
  that density foundation and adds versioned hydrology/regional vegetation. Epic remains
  a historical reference and a saved v4-world recipe, not the new-world default.
  Do not return to bare height-only prototypes or fourfold mountain amplification.
- The previous v4 owner-acceptance claim was WRONG: the positive feedback was
  from DormLab-EpicReference with IL and DH disabled, not the installed DormLab.
  The owner identified the liked result as ordinary generation on 2026-11-03.
  IL visuals and DH loading are not accepted. Read docs/feedback-correction.md.
  Preserve user saves and settings while diagnosing; do not silently switch
  profiles. The mod's creation-screen name remains Immersive Landscapes.
  Future structures must target the eventually approved geography, not assume
  that descriptor4 / epic-compact-v1 has already been approved.
- The old G0-G7 / up-to-100-hour plan was rejected as over-scoped. Follow the
  current M1–M4 completion milestones; prioritize proven APIs and bounded fixes.
- Start with `docs/next-agent.md`, then `docs/implementation-plan.ru.md`. Resume the
  first incomplete gate; do not repeat completed DH porting or broad research.
- The owner's acceptance is a world created from the delivered build, impressive
  terrain with Distant Horizons, and measured smooth performance. A passing unit
  test, preview image, successful build or screenshot alone is not completion.
- Performance aspiration: **32 actual near chunks, 120 FPS**, with farther DH
  terrain. Keep near distance distinct from DH distance. This is a target, not an
  achieved result. Neither near16/DH128 nor near32 has owner acceptance for
  this mod. Offer settings for older PCs; preserve pack headroom.
- The owner accepts even roughly 10x longer world preparation to improve gameplay
  performance. Prefer persistent preprocessing and bounded asynchronous work;
  never hide a warm-only test as fresh-world performance.
- In this Windows workspace, all project-controlled files, temp, caches, builds,
  worlds and evidence must remain on E:. Read the parent AGENTS.md and source
  `../RustCore/development/scripts/Enter-RustDev.ps1` before Rust/build commands.
  One Cargo job; no concurrent full engine builds. Never reboot automatically.
- Preserve existing saves, accounts, clean RustCore, and the tested DH rollback.
  Read `../RustCore/AGENTS.md` before runtime work. Read source-tree instructions
  before changing an engine stage; never hand-edit the pristine engine snapshot.
- Author/Git identity: dormyu. Use the verified local Git identity, not invented
  email or changes to global Git. Repository: dormyu/immersive-landscapes.
- Original terrain code is MIT; DH adaptation remains separate with its LGPL
  notices. Do not commit game resources, credentials, control tokens or worlds.
- Rust client and Rust dedicated server; solo and at least two clients required.
  No Java compatibility work and no DormCore framework without a real consumer.
- Work in small, verifiable commits. Update gate status with evidence and the next
  exact action. Do not mark a task complete while an acceptance check fails.

## Historical lab supplement — 2026-11-03, SRC-017

The owner gave positive feedback about the current v5 world except the transient
near/far water boundary. This is not complete renderer acceptance. They requested
the launch graphics profile with near32 and clarified the OptiFine-like feature
as Zoom. Current DormLab runtime 44f1baf1 adds official Horizon Zoom 2.0.0/MIT
(unmodified725ff7a source), separate from generation-only Immersive Landscapes.
Near32/DH128 Medium/sim8/cap120/VSync on/nativeLOD0/workers Auto. Z holds4x,
the wheel adjusts it. Solo, two-client Zoom and installed Prism checks passed;
current reports: RustCore/qa/dh-transition-2026-11-03/reports. Older5dd3/near16
metrics above are historical. Read native-build.json for current hashes and QA.
No complete water-seam,32/120 or heavy-pack acceptance claim. Preserve the
current terrain recipe while working on the remaining boundary issue.
