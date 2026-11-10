# Validation — Immersive Landscapes 0.6.0

Source/runtime validation snapshot: **10 November 2026**. The optimized
client below has passed current-source native, solo/save/restart, two-client,
pond-flow, regional-capture, matched vanilla/IL and delta-handoff checks. Cold/warm
performance and the1.92km route completed;38 game PNGs were reviewed with the
remaining DH limitations stated below. Installation, Prism and publication readback are recorded in final
release notes/local provenance after the source documentation is frozen.
These documents alone do not claim delivery. QA receipts live locally under
`RustCore/qa/completion-2026-11-08-bankfix`; older candidate results are separate.

## Exact source and current status

| Item | Identity / status |
| --- | --- |
| Current IL source | `c43610a2fc73b2d092930ad47c4ba5b60ebe83db` |
| Horizon engine / game | `f42ef42ef6ceccb3c6a20bea2481d87949a51c81` / 26.3 |
| New-world recipe | descriptor 11 / `vanilla-natural-v7` |
| New markers | `rivers_v6=1`, `ecotones_v3=1`; existing v5/v2 markers retained |
| DH | `0.1.0+dormyu.16`, source/documentation `6d9744701eef2c4bf9ab0f1643caca7d152cd083`; adapted code unchanged |
| Zoom | official 2.0.0, unchanged `725ff7a41017c40104273e407264e0ebee87e567` |
| Standalone checks | **71 tests passed; Clippy passed** |
| Native libraries | `native-probes-build-02.json`: successful current-source Cargo build |
| Optimized client / SHA-256 | `4d7c4fec1e668fff43272a25696cd5ffe5b8846e78038382053e5e7b6c9e6a0c` |
| Delivery evidence | Read final release notes and installed provenance; source validation alone does not certify an installed runtime |

The successful library build used one Cargo job and a 1.5 GiB compiler working-set
cap. It took 390.11 seconds, with a recorded minimum 1.615 GiB available RAM.
These are library-build measurements, not gameplay performance.
The interrupted `native-probes-build-01` remains interrupted evidence.

The optimized full client build passed in 735.68 seconds with one Cargo job,
opt-level2, incremental disabled and codegen-units64 for the modded client crate.
Its recorded compiler physical working-set cap was 2.5 GiB; minimum available
RAM was 1.995 GiB. Before/after staged-tree hashes agree across 4,521 files.
`client-build-01.json` binds that build to the exact executable hash; the frozen
`reports/native-build.json` verifies source match. Its build-time pending notes
are superseded only by the later, separately pinned receipts below.

The fresh combined native receipt contains seven successful compile/run phases:
three direct-linked programs, exact delta columns, two separate seed 42 diagnoses,
and the four existing weighted-donor assertions. Its source identity, staged
inputs, library fingerprints and executable hashes are recorded; it does not
claim that the renderer or full game has run.

## Exact dry-bank regression — current native PASS

The former candidate excavated a high dry shore beside an unchanged aquifer:
coarse raw 40.65625 could turn actual native 65 into floor 42; another checked
column became 46. Current source stops the dry-bank floor at
`max(old_floor, min(native_ground, planned_waterline))`. It does not raise
already lower native ground and excludes channel cores and lakes. The rule
applies only to the refined natural recipe, leaving saved v10 behavior intact.

The new diagnostic uses seed `4312740467388825473` and compares actual native
TERRAIN with the same IL density fields, aliases and registry; only
`watershed_v1` is omitted from the BASE generator.

| Exact dry X,Z | Actual BASE ground | Current candidate ground | Water in the dry column |
| --- | ---: | ---: | --- |
| −1684,−2646 | 65 | 63 | None |
| −1685,−2647 | 65 | 63 | None |
| −1686,−2647 | 65 | 63 | None |
| −1687,−2648 | 65 | 63 | None |

Four adjacent native water columns remain byte-for-byte unchanged: their bottoms
are 32, 37, 31 and 32, with water top 63. All **eight** exact columns pass actual,
scalar and grid agreement for ground, water and top block state. This closes the
identified native excavation regression. The current client separately verifies the rendered bank and native32→8→32 handoff below; this native probe alone does not supply that evidence.

## Rivers — new full bounded M1 run

One freshly linked native executable generated separate v10 and v11 recipes,
seed `4312740467388825473`. The valley covers 130 chunks / **33,280 columns**;
the delta covers 156 chunks / **39,936 columns**. Both recipes have zero dry
planned river-core columns in both fixtures.

| Metric | Valley v10 | Valley v11 | Delta v10 | Delta v11 |
| --- | ---: | ---: | ---: | ---: |
| Planned wet core columns | 716 | 743 | 695 | 681 |
| Physically disjoint adjacent wet intervals | 36 | 0 | 0 | 0 |
| Core fraction in largest connected component | 92.877% | 100% | 100% | 100% |
| Connected components containing core | 2 | 1 | 1 | 1 |
| Sampled near/scalar/grid columns | 104 | 111 | 75 | 63 |
| Sampled parity failures | 0 | 0 | 0 | 0 |
| Shore height above water: 95th percentile | 9 | 3 | 1 | 1 |
| Shore height above water: maximum | 10 | 10 | 1 | 2 |
| Adjacent bank-ground step: maximum | 3 | 3 | 2 | 2 |

The current v11 delta has 917 cut columns, rather than the earlier candidate's
deep-cut footprint. The new delta run no longer reports the former 23-block
bank-ground step; its maximum is two blocks. Counts describe different modified
footprints and are not like-for-like aesthetic scores.

A supplementary wet/dry-face audit of these same native rectangles found zero
exposed wet/dry faces in the delta for either v10 or v11. In the v11 valley it
found 77 exposed faces, maximum eight blocks; faces adjacent to a cut dry bank
were at most two blocks. An exposed face may be an intended waterfall, so these
counts require image interpretation. They neither prove every bank smooth nor
replace the eight exact actual-base columns above. These are bounded static
TERRAIN results, not global drainage, fluid simulation or complete DH acceptance.

## Seed 42 cavities/outlet — current native and real-fluid checks passed

The two exact fill-only lake cases were rerun against current c43610a libraries:

| X,Z | Actual BASE TERRAIN | Current candidate TERRAIN | Only changed blocks |
| --- | --- | --- | --- |
| −2288,−2344 | Ground 50, water top 63 | Same ground 50, water top66 | AIR→WATER at Y63–65 |
| −2276,−2320 | Ground 60, dry | Same ground 60, water top66 | AIR→WATER at Y60–65 |

Their distant surface approximations report ground 65 and63 respectively. The
actual cavity bottoms are retained; they are not reconstructed from an inaccurate
surface height. These are two explicit exceptions, not a general exemption from
near/far agreement.

The outlet diagnostic preserves the adjacent actual aquifer at ground 62/water 63,
although its native surface approximation still omits that water. The upper core
has ground 65/water 66. The only documented adjacent material differences are
12 buried terracotta-to-stone changes below Y62; water and occupancy are retained.
The current `pond42-game.json` passes all nine required loaded water-block
observations, including the descending source/flowing/falling-water path and
both deeper native bottoms. Both current images were reviewed: the short outlet
visibly joins the lower basin, with no disconnected dry gap or floating water
sheet at this target. Native8/DHoff, base FOV90, simulation12 and cap60 were used;
this is a bounded fluid/shore check, not a performance measurement.

## Regional vegetation — new full M2 run

Current `ecotones_v3` was compared on/off while retaining the same current v11
hydrology and terrain input. A separate legacy-v2 boundary checks its previous
marker policy. Each case uses nine native FEATURES passes inside a 5×5 TERRAIN
neighborhood, including neighboring spill; structures are disabled.

| Boundary, seed 4312740467388825473 | X,Z | Changed vegetation blocks | Native soil changes | Other explained native changes |
| --- | --- | ---: | ---: | --- |
| Desert / eroded badlands | −1466,−1536 | 63 | 0 | None |
| Snowy plains / snowy taiga | −2990,1280 | 903 | 212 | 549 supported snow-layer changes |
| Forest / swamp | 768,−50 | 3538 | 34 | None |
| Sparse / bamboo jungle | 5258,1536 | 8868 | 130 | 3 anchored 2×2 trunk-support cells |
| Retained legacy-v2 forest / taiga | −1484,1408 | 2770 | 37 | None |

All five boundary reports pass. All **71 native donor graphs** decode and pass
the bounded recursive safety check, with no missing biomes or donor failures.
Both a recreated generator with fixed decoration order and another worker with
reversed terrain order reproduce exact terrain, feature and statistics results.

Across all five cases, height changes, gameplay quart-biome changes, forbidden
terrain changes, introduced/repainted top disagreements and unexplained
nonvegetation changes are zero. Baseline native top disagreements remain
respectively 0, 2, 23, 446 and 125 in both sides of these comparisons: zero new
disagreements does not mean zero native approximation differences.

Snow changes require native weather-pass evidence, snowy climate, valid support
and an identified plant/canopy change. The three jungle support cells require a
complete anchored 2×2 dirt/log base with two log layers. They are not blanket
exceptions for floating snow or dirt. Counts are observed changes, not a density
target or aesthetic score. This is native feature integration, with no renderer,
FPS, structures or multiplayer acceptance implied.

## Current frozen-client checks

All receipts below pin runtime `4d7c4fec` and source c43610a. Automated functional
results and manual observations are stated separately.

| Check | Verified result | Scope |
| --- | --- | --- |
| Solo / creation UI | PASS | Russian/English **Immersive Landscapes** type; fresh v11 recipe/aliases; placed block retained after a full process restart |
| Saved recipes / mod toggle | PASS | v10 descriptor and all20 recipe files unchanged; disabling IL restores Default/new vanilla; re-enabling does not convert that vanilla save |
| Two Rust clients + loopback server | PASS | Shared block, independent Zoom controls, DH off/on lifecycle, no decode/far-transport errors, three clean exits |
| Seed42 pond | PASS plus two reviewed images | Nine actual water blocks and visible descending connection at native8/DHoff |
| Four regional scenes | Capture PASS plus manual review | Dry/warm, snowy, wetland and bamboo scenes; no obvious detached canopy in these views; native pillars/material borders remain |
| Delta32→8→32 | Revalidated PASS plus reviewed images | Ten original scene/status PNGs,16 bank/water observations, clean exit; exact repaired bank crosses the near/far boundary |
| Matched vanilla/IL | PASS plus six reviewed images | Fresh paired valley/delta/forest scenes on the same executable; published PNG copies match original hashes |
| Final image-review gate | Bounded PASS | All38 actual PNGs reviewed; geometry regression absent at tested locations; open DH findings retained, no whole-renderer acceptance |

The network smoke uses native2/DH64, not the target performance profile. Minimum
recorded available RAM was 4.49 GiB for solo,3.88 GiB for network,1.34 GiB for pond
and1.36 GiB for regional capture. These are host-availability observations, not
memory budgets for the complete modpack.

### Delta handoff and the ocean color boundary

The regression camera `[-1460.5,145,-2422.5]` looks toward the repaired bank
14 chunks away on both axes, outside native8 but inside native32. Ten original
PNGs capture32→8→32 with scene and `/dh status` views. Four dry bank and four wet
positions are checked before and after. The target remains connected; the former
tall water curtain, rectangular void and major shoreline hole are absent in
these views. Small-plant/detail loss at8 reverses at32 without moving the river.

The original post-run oracle incorrectly required `playing` for status images
whose chat overlay was intentionally open. The original failed receipt and
helper are retained unchanged. `delta-handoff-candidate-01-screen-revalidation.json`
accepts only this exact predicate correction, verifies source/evidence hashes and
all existing queues/blocks/PNGs, and records **no new native execution**. The
native run itself completed its observations and exited0; the correction does
not invent a second successful run or grant visual/FPS acceptance.

Manual status review showed298/298 sections and0 queued jobs before the switch,
324/324 and0 at settled8, and298/298 shown at settled32 with472 refinement jobs
still queued. Immediate transitions were still preparing. This is a successful
bounded handoff, not a claim that all DH work has finished instantly.

The fixed diagonal ocean color boundary was traced independently to
`deep_lukewarm_ocean` (`#45adf2`) → `warm_ocean` (`#43d5ee`). Three pairs of pixels
match that biome change exactly in both native32 and native8 images. Both sides
have gen_step8, state89 water, top blockY62, sky15 and block-light0. DH takes the
quad's biome water tint directly. This is a tint transition, not an actual-versus-
predictive boundary or height discontinuity. The diagnosis hashes its DHS,
options, selected QA-player inputs, PNGs and source; effective spectator FOV99 is
derived from the saved state/base90, not claimed as captured GPU-uniform data.
Water-color interpolation remains a separate possible extension. Differences in
actual underwater span lighting have no demonstrated causal link to this band.

### Controlled performance and delivery scope

Hardware readback: Intel Core i5-9400F (6 cores/6 threads), NVIDIA RTX2060,
Windows driver32.0.16.1088, OS-visible physical RAM15.94 GiB.
This identifies the measured host, not free RAM or a minimum system requirement.

The measured controlled performance profile is1920×1080, actual32/DH128 Medium,
simulation8, VSync off, maxFps260 (Horizon's unlimited value), offscreen spectator
and random ticks0. Cold generation and a separate-process warm reopen must remain
separate. Delta captures instead use1600×900, simulation5 and cap60; pond/regional
captures use native8/DHoff and cap60. None is an FPS result. The cold run's first
8 seconds showed an empty/incomplete scene; retain that as a loading observation,
not as loaded-game frame-rate evidence. Its raw high FPS must not be published as
gameplay performance.

All38 current game PNGs were actually reviewed, covering all30 mandatory
images and8 supplementary captures. The targeted bank/flow/regional-vegetation
and geometry-handoff review passes. Coarse water colours, pale distant
underwater patches and isolated predictive far-canopy fragments remain visible
DH limitations; this is not complete renderer or owner aesthetic acceptance.
The final `reports/visual-review.json` records those findings explicitly.
| Run / scene | Average FPS | 1% low FPS | P99 frame, ms | Worst frame, ms |
| --- | ---: | ---: | ---: | ---: |
| Cold / static20s | 368.4 | 295.7 | 3.05 | 4.80 |
| Cold / walking130 blocks/30s | 345.6 | 162.4 | 5.32 | 12.65 |
| Cold / fast480 blocks/30s | 295.1 | 133.3 | 6.68 | 10.43 |
| Warm / static20s | 368.9 | 294.8 | 3.08 | 4.84 |
| Warm / walking130 blocks/30s | 397.5 | 177.8 | 4.98 | 12.25 |
| Warm / fast480 blocks/30s | 313.6 | 135.9 | 6.63 | 10.20 |

The first loaded native32 scene required136.19s after the test teleport; DH
then needed458.32s to reach the stationary gate. The separate-process reopen
needed16.23s and42.87s respectively. Both prepared DH overlays show315/315
sections and0 queued jobs. These intervals follow initial client readiness and
are not total launch times. The first8s incomplete view is excluded from this
table. Gameplay benchmarks begin only after native32 has loaded.

| Extra warm leg,480 blocks/30s | Average FPS | 1% low FPS | P99 / worst, ms | Settle after stopping, s |
| --- | ---: | ---: | ---: | ---: |
| 1 | 301.8 | 140.1 | 6.33 / 8.90 | 20.53 |
| 2 | 283.3 | 146.3 | 6.14 / 9.96 | 37.67 |
| 3 | 247.5 | 130.2 | 6.93 / 11.96 | 35.51 |
| 4 | 273.3 | 143.8 | 6.30 / 9.41 | 16.95 |

The four extra legs form a continuous1.92km route. Each returned to loaded
native32 with no decode errors; returning to the original camera settled in
11.32s. This is a bounded traversal, not a multi-hour endurance test.

Peak whole-client working set/private memory: cold2.790/5.992GiB,
warm2.230/5.826GiB. Minimum free system RAM:2.454/2.967GiB. These are
whole-client measurements, not the incremental memory cost of IL. Owner apps
were left open, with no artificial client working-set trimming. Cold and warm
processes exited normally in5.76/11.11s; saved-world/cache hashes were retained.

Both complete receipts pass. The largest measured frame in these windows is
12.65ms, so this is not a promise that every frame stays within an8.33ms120FPS
budget. No universal-FPS, future heavy-pack or owner aesthetic acceptance is
inferred. Reproduce from `reports/performance-candidate-01-{cold,warm}.json`;
`reports/performance-summary.json` retains their exact hashes.


Installation, actual Prism→bridge→native provenance, preserved owner saves/settings,
control-off state and source-asset publication/download readback are recorded
in final release notes and local delivery receipts. They occur after the fixed
documentation commit/source bundle; do not infer them from these runtime checks.

## Historical client evidence — not current bankfix acceptance

The previous source `bfbaa93b771811d6b177b30951cd27c58abc479f`, runtime
`ac2e4919b53092e80cad2a5b5e64e4d95913f859fc198338f9ac467138f39d2e`, lives under
`completion-2026-11-08`. Its solo, two-client, seed 42 fluid, regional and matched
A/B checks were executed, but image review found the delta dry-bank water wall.
Its cold performance run was deliberately stopped and has `passed:false`; no
warm performance or accepted FPS result exists. Those reports remain attached
to that old binary and are not final 0.6.0 evidence.

The inherited 0.5.1 stronghold-ring fix remains in source; its ring/reproducer
results retain their [0.5.1 scope](validation-0.5.1.md). Saved descriptors 4–9 are
supported but not all freshly retested here. No owner aesthetic approval of the
replacement 0.6.0 runtime has been recorded.

## Current evidence and limits

Current root: `RustCore/qa/completion-2026-11-08-bankfix`:

- `project-tests.log`, `project-clippy.log`, `native-probes-build-02.json`.
- `runs/actual-base-final-01/receipt.json` and its exact delta/pond diagnoses.
- `refined-rivers-final-01/summary.json`, four native rectangle reports, and
  `wet-dry-face-audit-01.json`.
- `runs/regional-final-01/receipt.json`, native donors and five boundary reports.
- `client-build-01.json`, `reports/native-build.json`.
- `reports/native-functional-solo.json`, `reports/native-functional-network.json`.
- `pond42-game.json`, `regional-game.json`, `ab-vanilla.json`, `ab-il.json` and
  `reports/visual-review-ab-partial.json`; copied images match original SHA-256.
- Original `reports/delta-handoff-candidate-01.json`, its separate screen
  revalidation, archived helper and all ten hashed original PNGs.
- `reports/visual-review.json`, `visual-observations.md`, all16 performance images,
  both completed performance receipts and `reports/performance-summary.json`.
- The partial delta/pond review receipts and
  `ocean-tint-diagnosis.md` / `.json` for the derived pixel/cache correspondence.

The performance handoff at near8 shows coarse stepped water colours; after
restoring32, pale distant underwater patches remain while128 DH jobs are queued.
Their cause is not proved by the separate delta tint diagnosis. The final route
image also contains four isolated jungle-leaf spans beyond native32: detail9,
stride8, predictive SURFACE/gen_step5; finer data and real saved chunks are absent.
Predictive sampling/downsampling can omit their thin trunks. This is an observed
far-renderer limitation, not proof of floating real-world blocks, and waiting is
not proven to remove it.

Native tint and some sand/terracotta/stone boundaries remain. Drainage is bounded,
not every coast receives a delta, and native cliffs/voxel banks and cavity
approximation differences remain. Initial DH filling/refinement is observable,
not instant. Future heavy-pack, all-seed, long-session and owner aesthetic
acceptance are separate from this bounded validation. Villages have not been
redesigned. Source bundles retain credits/licenses and exclude game assets,
accounts, worlds and engine binaries.
