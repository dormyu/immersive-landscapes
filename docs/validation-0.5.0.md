# Validation — Immersive Landscapes 0.5.0

6 November 2026. Working native release; bounded lab review passed. Owner aesthetic
approval is not recorded. Installation/Prism/publication happen after the source
documentation commit and are recorded in the final release notes and local
installed `native-build.json`; this document does not predict those later results.

## Identity

| Component | Built source |
| --- | --- |
| IL 0.5.0 | `224a8316f56bff5c119c57c4e4a909c81101a893` |
| DH dormyu.16 | `623dd716217c059f9f9aa47f225cd6894dd00c1b` |
| Official Zoom 2.0.0 | `725ff7a41017c40104273e407264e0ebee87e567` |
| Horizon / Minecraft | `f42ef42ef6ceccb3c6a20bea2481d87949a51c81` / 26.3 |
| New-world recipe | descriptor 10 / `vanilla-natural-v6`; `rivers_v5=1`, `ecotones_v2=1` |
| Final optimized executable SHA-256 | `3bb2fb54bc677e1ba3fe3bfdf4f0b18b034c55fbb2285111ab8831a81da4efcd` |

Build uses `fast`, modified crates opt2 / CGU64, incremental off, one Cargo job.
Final build took 843.2 seconds with minimum 2.29 GiB available RAM. The source
manifest and compiled Rust inputs were compared. The mixed client has a nonfatal
unused `generate_missing` warning; standalone Clippy results do not mean the
whole engine is warning-free. Documentation commits are recorded separately from
the unchanged compiled-code pins in package manifests.

## Generation and visual checks

Native QA `river-dh-final-2026-11-06` pins IL224 and actual native libraries:

| Check | Observed result |
| --- | --- |
| IL source | 56 tests and all-target Clippy pass |
| Native hydrology | Three seeds; wet floors/water/top and second-worker parity pass |
| Continuous river routes | 3,120/3,120 and 1,702/1,702 wet points; zero missing profiles |
| Bank/lake area | 16,384 columns; 1,472 lake-neighbor pairs, zero broken pairs |
| Ocean precedence | Three checked receiving boxes; no raised ocean water |
| Diagnosed lake saddle | Four former gaps wet; 419 four-neighbor-connected water columns |
| Coastal regressions | Both former catchments retained |
| Actual native ecotones | Three decorated boundaries; vegetation changes, unchanged heights/biome IDs; repeated/second-worker parity |

The ecotone seed is 4312740467388825473: forest/taiga near (-1484,1408),
forest/plains near (-1344,1536), birch/dark forest near (-980,256). Native FEATURES
fixtures disable structures and cover a central 3×3 region plus spill. Actual
optimized images of all three were reviewed at near8/DHoff; trees are grounded
and vegetation overlaps, but some grass/color borders remain abrupt. This is
not every biome or every feature-order permutation.

Q13 actual river, lake saddle, inland receiver, delta and pond images were also
reviewed. The disconnected saddle and hanging pond-water failure are corrected.
The [long reach](images/0.5-river-long-reach.png) and
[inland receiver](images/0.5-inland-receiver.png) use IL224/v10, seed 4312740467388825473,
near16/DHoff in an opt0 development client: terrain evidence, not final FPS.
Some inland reaches remain narrow/straight and voxel peninsulas remain angular.

DH has 44 passing standalone tests and Clippy, with failing counterfactuals
retained. Sparse preparation no longer erases sampled columns; unknown data
does not invent deep cliff walls. An offline reconstruction reduced 173 tall
border faces to one actual slope. Derived LOD caches are regenerated; real
chunks and player edits are preserved.

The original copied-world visual gate (`dh-shore-fix-2026-11-06`) was reviewed
by two agents. Near 8 after 1/12/60 seconds and return to 32 after 1/12 seconds show
the former deep black/white walls gone, with minor water/material differences.
It retains its original opt0 runtime 446d935f hash. The exact source change
896318d→623dd71 is solely the `.15`→`.16` displayed version byte. A strict
source/hash proof allows only that original geometry evidence to carry forward.

On the earlier optimized build, a detached distant shelf was visible while DH
coverage was incomplete. At 309/309 tiles it was connected, independently checked
by both reviewers. Do not call that initial-fill artifact impossible. The final
optimized runtime's five 32→8→32 images were separately inspected and accepted for
this scenario; initial far coverage, material transitions and other seeds remain
bounded limitations rather than a universal seamless-rendering guarantee.

## Final runtime checks

Local QA: `release-0.5.0-final-2026-11-06/reports`. All four runtime receipts below
belong to the final executable above:

- RU/EN explicit Immersive Landscapes type, ordinary new-world creation,
  placed-block save/reopen, disabled-to-vanilla and vanilla-after-reenable pass.
- Saved v4–v8 fixtures retain exact recipes; v9 is supported by saved-marker
  policy but has no separate runtime fixture in this suite.
- Two Rust clients and a loopback Rust dedicated server, near2: shared edits,
  independent Zoom Z/wheel, DH toggling, zero decode errors, normal exits pass.
  This is functional multiplayer validation, not a heavy-server benchmark.
- One process A(v10)→menu→B(v8)→menu→A: distinct seeds/cache roots, inactive-cache
  stability, recipe preservation, cleared menu geometry and saved block pass.
- Final cached near32/DH128 run, distance handoffs, DH off/on and normal exit pass.

## Performance

On an i5-9400F / RTX 2060 / 16 GiB PC, the final optimized runtime at
1920×1080, actual distance 32, DH128 Medium, simulation 8 and VSync off measured
**333.4 FPS stationary** (20 s; P99 3.97 ms,
worst 6.61 ms) and **319.2 FPS in a 130-block,
30-second movement at walking speed** (P99 6.30 ms,
worst 97.94 ms). This is a cached, offscreen scene with an aerial
camera and maxFps260 (unlimited), not first-time exploration or heavy-pack proof.

The earlier code-equivalent optimized build measured 114.0 FPS stationary and
99.3 FPS while flying 480 blocks into new terrain in 30 seconds, capped at 120.
Its native distance-32 preparation took 539.75 seconds; DH was still preparing.
The only subsequent source change was the one-byte DH version-label correction,
verified separately. These earlier numbers retain their original runtime hash;
they are not relabeled as fresh measurements of the final executable.
Full distant coverage needs additional initial work and can look incomplete
while filling. A universal, stutter-free 120 FPS result has **not** been established.
Lower actual distance, DH distance and DH quality are available for slower PCs.

The original optimized runtime 5b38eeb1 had full 309/309 distant coverage after
additional preparation. Its paired static test at the same camera measured
DH ON 160.34 FPS (P99 9.22 ms, max 16.42 ms) versus OFF 281.36 FPS (P99 5.34 ms), 20 seconds
each, uncapped. This bounds the actual cost of fully loaded distant geometry;
it is not proof of free DH, constant 120, heavy-pack headroom or new exploration.

Final cached preparation after native settle took 71.7 s;
minimum available system RAM during the run was 0.85 GiB.
That is free system memory, not the process's total usage. Resolution 1080p,
FOV90, nativeLOD0, offscreen mode and audio off apply to these renderer tests.
The walking-speed test moves an aerial spectator camera 130 blocks in 30 seconds;
it does not measure mob-heavy ground combat. The movement run included a
97.94 ms maximum frame (CPU 95.65 ms); its cause was not isolated. The high
average FPS must not hide that remaining hitch. The change in average versus
the earlier 160 FPS scene is not attributed to the one-byte version label.

## Exact final receipts

| Receipt | SHA-256 |
| --- | --- |
| `native-build.json` | `5f2d95d02f7d42b6874513deab6de43ccab67405f9e93ae5506e45255c60d123` |
| `native-functional-world.json` | `d526602ac429b51672a634c8d547e9bb57ad823b8279ba0dec37dbe8a816a152` |
| `native-functional-network.json` | `fb365ce5e09a1fe077aa7f59aaece88b9f7f0e0a3c2e9065720a5816c5d88836` |
| `world-switch.json` | `67c374a029ebf9e2384685ff0937fb93ba3f73a1072b287dd5beba460775da6d` |
| `performance-after.json` | `a066d34cb67cb381e5a31b977d717a356d3a777eef3e3db86af6dde5f8d9a4b9` |
| `candidate-review.json` | `92e93347b6bd1813dc8024ca7c8735654e28bd4ebac3f6cf1d31c847a12da8ff` |
| `version-label-reuse.json` | `3b918c49157939731b3494c9f0e28bac686c19c034a549870a592d1624ceff2a` |

QA roots retain raw data, failed earlier attempts and image hashes. The source
release excludes game resources, executables, accounts, world data and tokens.
All-seed, owner-aesthetic, low-end-hardware and heavy-pack acceptance are not
inferred from the passing local procedure. A future structure system must use
the saved recipe identity and the actual native surface, not assume every world
uses descriptor 10. Existing world recipes are never silently migrated.
