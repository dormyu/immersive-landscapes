# Immersive Landscapes

Familiar Minecraft terrain with coherent mountain regions, graded valleys and
connected coastal rivers. A native mod for the **Horizon Rust client and Rust
dedicated server**, by **dormyu**.

**Working version 0.4.0, active development.** Create a world normally with type
**Immersive Landscapes** and any seed. No prepared map or manual datapack selection.
World creation/save, old recipes, two Rust clients and installed Prism are checked.

![Immersive Landscapes with Distant Horizons](docs/images/hydrology-after-near32.png)

## Generation

The vanilla-natural-v3 overlay keeps Minecraft's scale, continents, height limits,
climate, biomes, native trees/plants, caves, ores and structures. Shared terrain
fields connect mountain regions and shape broader lowland valleys. Reduced 3D
rock detail retains cliffs and caves while reducing detached surface masses.

Native Rust hydrology adds bounded coastal drainage domains: contributing area,
common tributary confluences, graded channel beds and shared water profiles for
actual chunks and distant sampling. Derived input terrain is persisted locally
with seed/recipe/checksum checks. It adds no gameplay-frame callback.

![Actual generated river, native near32](docs/images/river-tributary-actual-game.png)

Coverage is currently coastal and bounded. **Full inland catchments, delta branches
and gradual vegetation transitions are unfinished.** Existing noise valleys are
not all converted. This is a working mod under development, not a claim that every
landscape requirement or seed has been accepted.

Headwater endpoints and banks can still look too regular; more natural branching
and endpoint shaping remain tuning work.

## Play and update

Run a Horizon build compiled with this mod, enable it, then create a **NEW** world
with type **Immersive Landscapes**. Saved v4/Epic, v5/v1 and v6/v2 worlds retain
their original recipes; ordinary vanilla worlds remain vanilla. Old worlds do not
gain the new hydrology automatically. Archived v1–v3 experiments need their builds.

The local laboratory is **DormLab**. **DormLab-EpicReference** is a separate
comparison instance with IL disabled. Do not attribute its ordinary worlds to IL.

## Build and performance

Pinned Horizon f42ef42 / Minecraft 26.3. A .hnmod contains Rust source and must be
compiled into Horizon. Copying it into a Java mods folder does not activate it.
[Build instructions](docs/native-build.md), [hydrology implementation](docs/hydrology-development.md),
[validation and limits](docs/validation-0.4.0.md), [current handoff](docs/next-agent.md).

The local set uses the separate DH dormyu.7 adaptation and unchanged official
Horizon Zoom 2.0.0. Actual near chunks and DH distance are separate. Z holds Zoom;
Z + wheel changes magnification. [Renderer work](docs/dh-boundary-detail-2026-11-04.md).

Short warmed offscreen1080p near32/DH128 Medium/sim8/cap120: new flight
**119.91 FPS**, P99 8.85 ms, max 13.72 ms.
Long exploration, older-PC profiles and heavy-pack headroom remain unverified.
DH vegetation/material/detail transitions can still differ; no universal seamless
rendering guarantee. [Exact paired test conditions](docs/validation-0.4.0.md).

Own code is [MIT](LICENSE). Historical Epic Terrain data retains wonderfulaichen's
MIT notice. DH is a separate LGPL project; Zoom and the mixins-tool patch retain
their own MIT notices. [Credits and inspirations](CREDITS.md). No game assets,
resources, accounts, worlds or engine executable are distributed. Independent
project, not endorsed by Mojang/Microsoft. The src/ drainage library is historical
research; current native generation lives in mixins/immersive_landscapes.
