# Validation — Immersive Landscapes 0.5.1

7 November 2026. Candidate under verification; the installed release is still
0.5.0 until the guarded installation and Prism check complete.

## Diagnosed regression

On the reported world, approaching a stronghold neighborhood stopped new chunks
from appearing. Restarting did not avoid the expensive work. A live native stack
also showed final shutdown waiting for a far-decoration worker calculating
hydrology through a stronghold biome search.

The frozen 0.5.0 runtime reproduced the failure on an isolated copy: native
distance 2, DH disabled, zero chunks after teleport for the 75-second observation
window. The UI still answered control requests, but exit did not finish within
30 seconds. Available RAM stayed above 4.45 GiB. This reproduction isolates
generation from DH rendering and memory exhaustion.

Horizon f42 deferred all 128 concentric-ring biome searches until a nearby chunk
needed any ring. IL's biome sampling then calculated catchments throughout those
distant search regions. Short spawn-area and cached-scene checks had missed it.

## Fix and invariants

Ordinary structure-start and saved-start queries now resolve only searches whose
native displacement/exclusion bounds can affect that chunk. Each result has one
shared initialization slot. A full lookup reuses these exact results and only
publishes the complete global table after every search is ready.

The native per-search random generator, ordered biome sampling, saved-piece
handling and structure placement rules remain unchanged. The full query joins
individual results instead of relying on a counter that can remain incomplete
after an initializer unwinds. Five native tests cover exact positions, local
work, reuse, concurrent queries, unwind recovery and fixed-biome behavior.

This patch keeps descriptor 10 and `vanilla-natural-v6`. It does not migrate a
world, change its terrain recipe or redesign structures. Explicit full-world
structure searches can still be expensive, and this is not a general cancellation
mechanism for every native generation job.

## Native checks

All five native tests passed in the actual staged Horizon world crate. They
executed in 0.23 seconds after compilation; minimum available RAM during that
build was 2.589 GiB. This validates the algorithm, not a gameplay or FPS result.

## Evidence still required

- Compile the optimized client.
- Recheck the same failing location, then near32/DH128 travel, busy exit and
  re-entry in a separate process on an isolated copy of the reported world.
- Recheck two Rust clients against the native server.
- Capture separate fresh vanilla/IL worlds with the same seed, cameras and
  settings; preserve the original images and inspect them before publishing.
- Verify source/runtime identity, preserve user saves/settings, install into
  DormLab and verify its actual Prism launch.

Local evidence directory: `RustCore/qa/hang-recovery-2026-11-07`.
Historical performance numbers remain attached to [0.5.0](validation-0.5.0.md);
they are not measurements of this candidate. Owner visual acceptance is separate.
