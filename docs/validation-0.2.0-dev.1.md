# Native candidate validation — 2026-11-02

Experimental build for Horizon c9d474d / Minecraft26.3. Author: dormyu.
Full Epic Terrain data (wonderfulaichen, MIT), with compact climate tuning.

## Functional checks

Normal Create New World installed the preset automatically, with no manual
datapack selection. Forests, surface decoration, caves, ores and structures
come from Horizon's complete native generator. A placed emerald block survived
world reload and a complete process restart.

A fresh Rust dedicated server usedseed482193 and installeddescriptor4 itself.
Two Rust clients received49chunks each, listed both players, hadzero chunk
decode errors and both received the same emerald-block change. This was a
loopback near2 smoke test, not a multiplayer performance or Internet test.

Two625-point native biome surveys over2304-block-wide areas found17→21 and
11→17 distinct biomes compared with the original preset. All1250 sampled
heights were unchanged. Climate frequency does not shrink continents/oceans.

The supplied agent-skill validator reported92 missing vanilla placed-feature
references because it treated a partially overridden Minecraft namespace as
fully local. All92 resolved against the actual prepared26.3 vanilla registries;
no unresolved references remained. The full pack also loaded in the game.

## Performance observations

Visible1920×1080 Vulkan, i5-9400F, RTX2060,16GiB RAM, E:HDD.
Audio/Discord disabled in QA. No compiler or second client during measurements.

| Scene | Near chunks | DH | Average FPS | P99 frame | Longest frame |
| --- | ---: | --- | ---: | ---: | ---: |
| Settled mountain view | 32 | 128 medium | 307.6 | 4.32ms | 5.07ms |
| Fast flight into new chunks,20s | 32 | 128 medium | 263.0 | 15.80ms | 63.77ms |
| Warm flight on the cached route,20s | 16 | 128 medium | 488.8 | 6.47ms | 25.19ms |

These are different loading conditions. They do not isolate the effect of near
distance, prove stable120FPS or establish headroom for the future heavy pack.
Near16 warm1%low was127.4FPS; near32 cold1%low was50.8FPS.

An explicit4-section/1MiB-per-frame upload cap was rejected. Switching to32
near chunks under that cap accumulated over100k pending sections and memory
pressure. It is not shipped as an optimization. Queue size and queue-processing
time both need bounds before32-chunk smoothness acceptance.

## Acceptance correction — 3 November 2026

The earlier acceptance paragraph was incorrect. The positive feedback came
from DormLab-EpicReference, with IL and DH disabled, not this candidate's
near16/DH128 profile. The owner subsequently identified the liked result as
ordinary generation and reported poor terrain/DH loading in DormLab.
Visual quality, distant loading and gameplay smoothness remain unaccepted.
See [the correction](feedback-correction.md). The measured QA above is retained;
it does not establish the owner's experience or a future heavy-pack budget.
Epic's noise rivers are not a validated drainage/delta simulation.

The Windows DH thread-local shutdown issue was fixed by stopping its session
atGame::shut_down before process teardown. Solo and both clients with DH enabled
then exited with code0; so did the server. Both network clients received actual
DH geometry, although their short runs did not settle every distant section.

Raw reports and unedited screenshots live in the local RustCore QA folder:
`qa/immersive-landscapes-2026-11-02/reports`. Game resources and worlds are local.
