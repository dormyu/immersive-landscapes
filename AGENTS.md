# Immersive Landscapes implementation

## Current source version —0.3.1 / SRC-018

The owner identifies this as a working mod in active development; do not label
the whole project a prototype. Source now uses vanilla-natural-v2 / descriptor6,
seven overrides and two private native aliases. New worlds suppress the reported
detached formation through base_3d_noise amplitude0.35; this is tested tuning,
not a universal terrain guarantee. Existing v4/v5 recipes stay byte-identical.
DHdormyu.5 is separate LGPL code; Zoom2.0.0 remains unmodified official MIT code.
UI/save/old recipes, two native clients, water-core regressions and near32
diagnostics passed. Exact installed/Prism status must be read from native-build.json.
QA: ../RustCore/qa/dh-handoff-2026-11-03. Hydrology/deltas/ecotones and universal
near/far matching remain unfinished. Preserve the owner's current cap/VSync;
the120/on restoration below is historical. Older source/profile notes below
describe previous checkpoints and do not override this section.


- Deliver a playable **Horizon native mod**, not a replacement engine or a CLI-only
  terrain library. The existing Rust library is an internal component of that mod.
- Scope is ONLY world generation: realistic mountains, connected rivers/waterbodies,
  seamless biome transitions, and matching DH terrain with low overhead. Do not add
  survival systems, progression, classes or a DormCore framework to acceptance.
- Latest direction (2026-11-03): ordinary Minecraft landscapes following natural
  rules. Use the native vanilla generator as the foundation; preserve its scale,
  climate, decoration and varied biomes. The new vanilla-natural-v1 overlay
  changes shared ridge/erosion/valley fields, not the entire recipe. Epic remains
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
- The old G0-G7 / up-to-100-hour plan was rejected as over-scoped. Use the replacement
  G0-G3 plan; prioritize existing engine APIs and selective reuse of reference code.
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

## Latest lab supplement — 2026-11-03, SRC-017

The owner gave positive feedback about the current v5 world except the transient
near/far water boundary. This is not complete renderer acceptance. They requested
the launch graphics profile with near32 and clarified the OptiFine-like feature
as Zoom. Current DormLab runtime 44f1baf1 adds official Horizon Zoom2.0.0/MIT
(unmodified725ff7a source), separate from generation-only Immersive Landscapes.
Near32/DH128 Medium/sim8/cap120/VSync on/nativeLOD0/workers Auto. Z holds4x,
the wheel adjusts it. Solo, two-client Zoom and installed Prism checks passed;
current reports: RustCore/qa/dh-transition-2026-11-03/reports. Older5dd3/near16
metrics above are historical. Read native-build.json for current hashes and QA.
No complete water-seam,32/120 or heavy-pack acceptance claim. Preserve the
current terrain recipe while working on the remaining boundary issue.
