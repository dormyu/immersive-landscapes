# Validation — Immersive Landscapes 0.5.1

7 November 2026. Optimized native candidate verified as described below.
Installation and Prism readback are recorded separately in the release notes.

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

## Actual optimized client checks

Built code: `a626cad3800fa1358b92c2ac880969f4987f2260`; Horizon `f42ef42`, DH `0.1.0+dormyu.16`, official Zoom
`2.0.0`. Runtime SHA-256: `808b212ac8ed618c3c49ed658e3bdf979e152a444a34e0cd8c2d7b624d9bea5a`.
One Cargo job, native gameplay opt-level 2, incremental off.

The identical near2 / DH-off regression scene settled in **11.675 s**,
including the five-second idle observation. The old build had zero chunks at
75 seconds and could not finish shutdown within 30 seconds. Both used isolated
copies of the same saved world and the same destination.

The larger recovery check used actual distance **32**, DH **128 Medium**,
simulation distance 4, 1600×900 and a 60 FPS cap. These are functional checks,
not an FPS benchmark or a change to the owner's saved sim8 / unlimited profile.
Random block ticks were disabled in the isolated copy during exploration-0:
all 3725 chunks were already loaded, but incidental remeshing repeatedly reset
the five-second idle gate. That leg's duration includes this fixture wait and
must not be reported as pure terrain-generation time. The adjustment is recorded
separately; subsequent legs/re-entry retain it. The original owner world is untouched.

| Recovery checkpoint | Time to ready and idle | Loaded chunks |
| --- | ---: | ---: |
| owner-location | 92.865 s | 3725 |
| exploration-0 | 296.037 s | 3725 |
| exploration-1 | 41.029 s | 3725 |
| exploration-2 | 44.780 s | 3725 |
| full-process-reopen | 114.508 s | 3725 |
| explore-after-reopen | 25.515 s | 3725 |

The first process exited while new terrain was still loading; the second was a separate
process reading the same QA save. Exit times were
25.239 and 17.831 seconds, both exit 0.
Decode errors remained zero; the saved terrain recipe was unchanged.
Minimum available RAM was 1.684 GiB.

Two Rust clients on a loopback native dedicated server passed block exchange,
Zoom, DH toggle and clean shutdown checks. This near2 smoke does not establish
four-player heavy-pack performance. Fresh vanilla/IL comparison worlds used
one seed and matched cameras/settings; original images were visually reviewed.

Local evidence: `RustCore/qa/hang-recovery-2026-11-07`. These are bounded
reproduction and recovery checks, not exhaustive world exploration. Explicit
full structure searches may still take time; general job cancellation and long
exploration remain separate concerns. Historical FPS remains attached to
[0.5.0](validation-0.5.0.md); no new FPS or owner visual acceptance is inferred.
