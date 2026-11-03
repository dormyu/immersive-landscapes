# 0.3.0-dev.2 — Horizon f42ef42 compatibility

3 November 2026. Development candidate, author dormyu. This ports the existing
vanilla-natural-v1 recipe to the newer Horizon; the six numeric density functions
and descriptor5 are unchanged. Visual acceptance remains pending.

## Exact native set

- Horizon `f42ef42ef6ceccb3c6a20bea2481d87949a51c81`, Minecraft26.3.
- IL code `4163dc3`, DH local adaptation `0.1.0+dormyu.4`, code `4226836`.
- horizon-mixins upstream `ddd9700`, local tool `d4af4d8`; bundled MIT patch.
- Windows `fast` optimized profile, one Cargo job. Final executable SHA256:
  `5dd3d0cda5b45bb14ddc436a64f2e7a6e32036abf14ba69877697018edee870d`.

The 22 Rust/TOML bundle inputs were checked against the applied stage: exact Rust
bytes, parsed TOML and original module/helper strings. The source archive may
include later validation documentation; the built code revisions above identify
the tested implementation. Game resources, accounts and saves stay private.

## Observed checks

| Check | Result |
| --- | --- |
| RU/EN creation menu | Immersive Landscapes is displayed; other world types still cycle |
| New world | descriptor5; exact embedded recipe and private vanilla alias |
| Save/reopen | Placed emerald block survives |
| Disabled IL | Default type and a new vanilla world without an IL marker |
| Existing v4 copy | All94 recipe files and descriptor unchanged |
| Two Rust clients + Rust server | Shared new IL world/block, DH geometry, decode errors0, all exit0 |
| DH shutdown regression |7,233,232 resident far quads become0 after `/dh off` |

The shutdown change cancels pending uploads and releases fading tiles. The prior
candidate retained over5 million resident quads after disabling DH. That finding
does not prove that every retained tile remained visible, or that this fixes all
distant visual artifacts.

## Short performance diagnostic

RTX2060, Windows/Vulkan, offscreen1920×1080, near16/simulation8/DH128 medium,
cap120, VSync/audio off. A newly created f42 QA world was copied with its derived
LOD cache omitted. All288 requested sections were drawn by150s; the sampled DH
job queue was0 at180s. Further native feature refinement can continue.

| Measurement | Average FPS | P99 frame | Longest frame |
| --- | ---: | ---: | ---: |
| Static |120.00 |9.03ms |10.13ms |
| Flight1 |118.60 |11.87ms |22.60ms |
| Flight2 |116.10 |14.56ms |25.68ms |
| Flight3 |119.31 |9.93ms |20.16ms |

Each flight is12 seconds and resets the camera to the start, including a roughly
540-block teleport/disk load. These are short samples, not a sustained traversal
or a final visible-game/heavy-pack guarantee. Peak measured memory through the
flights: RSS1991MiB, private commit4147MiB; those counters describe different costs.

32 near chunks at120FPS remains unconfirmed. A separate earlier32-chunk probe
during mesh construction gave75–113FPS, with a159ms longest frame. It is retained
as a failed diagnostic, not replaced by the better16-chunk numbers.

Gray coarse faces, far-water/material boundaries, full drainage networks/deltas
and blended vegetation transitions remain open. Multiplayer QA above checks
functionality at near2, not complete remote refinement or server load.

Local evidence: `RustCore/qa/dh-lifecycle-2026-11-03/reports`. The installed build
and Prism readback are recorded separately in DormLab's `native-build.json`;
source checks and offscreen diagnostics alone do not establish delivery.
