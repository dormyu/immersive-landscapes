# Immersive Landscapes implementation

- Deliver a playable **Horizon native mod**, not a replacement engine or a CLI-only
  terrain library. The existing Rust library is an internal component of that mod.
- Scope is ONLY world generation: realistic mountains, connected rivers/waterbodies,
  seamless biome transitions, and matching DH terrain with low overhead. Do not add
  survival systems, progression, classes or a DormCore framework to acceptance.
- Feedback rejected v3 entirely: one biome, bare terrain, isolated
  oversized peaks and broken far chunks. Keep Minecraft's gameplay scale and
  varied fantasy-friendly geography. Epic Terrain's existing world was judged
  promising, but its biome regions are too large. Reuse the complete native
  generation recipe with proper credit; do not rebuild another simplified
  height-only approximation or scale mountain relief fourfold.
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
