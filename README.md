# Immersive Landscapes

Familiar Minecraft terrain with connected mountain regions, rivers that follow
their catchments, and gradual vegetation transitions. A working native mod for
the **Horizon Rust client and Rust dedicated server**, by **dormyu**.

**0.5.1 — native streaming fix.** Approaching a stronghold could stop new
chunks and make re-entry or shutdown appear stuck. Ordinary chunk generation now
calculates only the relevant ring searches, preserving the seed and saved terrain.
The reproduced failure, near32/DH128 travel and full process restart, and two Rust
clients pass the targeted checks. See [validation](docs/validation-0.5.1.md) for
exact scope. This fix works in existing IL worlds; no new world is required.
Source bundles need the documented native build and contain no game executable
or assets.

## Generation

The `vanilla-natural-v6` recipe keeps Minecraft's scale, continents, climate,
height limits and gameplay biome identities. Shared ridge, erosion and valley
fields shape broader mountain regions and connected lowlands. Caves, ores and
structure definitions remain native; this release does not redesign structures.

Native Rust hydrology builds bounded drainage graphs with tributaries, shared
confluences and graded beds. In addition to coastal rivers, suitable native
inland lakes can receive a catchment at their actual water level. Large rivers
on suitable low coastal plains can split into conservative delta distributaries.
The branches share the trunk's discharge and are rejected where native terrain
would require cutting through a cliff or narrow rock bank.

River reaches receive terrain-constrained smoothing and meanders. Lake crossings
preserve existing wet bottoms and islands outside the channel core. A low saddle
on a planned lake channel can be opened when it is at most six blocks above the
water surface; the new floor is one block below water. Higher obstacles remain.

Vegetation blends across compatible boundaries among ten temperate and cool
biomes: forest, plains, birch forest, flower forest, dark forest, sunflower plains,
taiga, meadow, old-growth pine taiga and old-growth spruce taiga. Both native
vegetation density and neighboring species change through the transition.
Direct deciduous/conifer borders are supported where native soil and temperature
permit them. Deserts, jungles, savannas, wetlands, snowy and custom biomes retain
their native transition behavior. Gameplay biome IDs are unchanged.

## Same seed, same camera

These are separate **fresh** vanilla and IL worlds, seed `4312740467388825473`,
captured with the same optimized 0.5.1 client, camera, noon lighting, 1600×900
resolution, FOV 90, random block ticks disabled and actual distance 16. DH is off in both so the comparison
isolates generation. Images are unedited native screenshots, not performance
measurements. Minecraft's scale and native biome identities are intentionally
recognizable; this is not a replacement biome pack.

| Scene | Vanilla | Immersive Landscapes |
| --- | --- | --- |
| River valley | ![River valley, vanilla](docs/images/0.5.1-river-valley-vanilla.png) | ![River valley, il](docs/images/0.5.1-river-valley-il.png) |
| Low coastal catchment | ![Low coastal catchment, vanilla](docs/images/0.5.1-delta-vanilla.png) | ![Low coastal catchment, il](docs/images/0.5.1-delta-il.png) |
| Forest/plains transition | ![Forest/plains transition, vanilla](docs/images/0.5.1-forest-plains-vanilla.png) | ![Forest/plains transition, il](docs/images/0.5.1-forest-plains-il.png) |

See [capture coordinates and observed differences](docs/comparison-0.5.1.md).

## Play and update

Use a Horizon build compiled with the matching native mod set, enable Immersive
Landscapes. To start fresh, create a world with type **Immersive Landscapes**. Choose
any seed; no prepared map or manual data-pack selection is needed. New worlds
save descriptor **10** and the exact `vanilla-natural-v6` recipe. Updating an existing
IL world from 0.5.0 to this streaming fix preserves its saved recipe.

Saved descriptors 4–9 keep their own recipes; reopening a world does not add the
new river or vegetation markers. Ordinary vanilla worlds remain vanilla. Older
v1–v3 experiments require their historical builds. The optimized 0.5.0 client
passed new-world/save/reopen checks and unchanged legacy v4–v8 fixtures. The v9
recipe path remains supported by the saved-marker policy; it was not a separate
fixture in this runtime check.

The paired rendering set uses **Distant Horizons dormyu.16**, a separate LGPL
adaptation, and unchanged official **Horizon Zoom 2.0.0**. Hold **Z** to zoom;
use **Z + mouse wheel** to adjust magnification. DH16 repairs partial LOD
preparation and boundary walls. It rebuilds derived LOD caches on first use;
real world chunks and player edits are preserved.

## Performance and limits

The target is **32 actual chunks with 128 DH chunks at Medium quality**, while
leaving room for a larger modpack. These are separate render distances.
The following historical measurements are from **0.5.0**, not a new 0.5.1 benchmark.
On an i5-9400F / RTX 2060 / 16 GiB PC, that optimized runtime at
1920×1080, actual distance 32, DH128 Medium, simulation 8 and VSync off measured
**333.4 FPS stationary** (20 s; P99 3.97 ms,
worst 6.61 ms) and **319.2 FPS in a 130-block,
30-second movement at walking speed** (P99 6.30 ms,
worst 97.94 ms). The movement run still contained a roughly 98 ms hitch. This is a cached, offscreen scene with an aerial
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

Drainage and lake discovery are bounded; not every valley becomes a river and
not every coast produces a delta. Some narrow inland reaches and voxel shores
remain angular. The coastal comparison also shows steep water/ground steps and
abrupt surface-material boundaries that still need visual work.
The vegetation policy does not cover every biome or smooth all
grass, water and sky tint changes. Distant terrain approximates blocks and
materials. Long exploration, low-end hardware and heavy-pack headroom are not
established by a short local test. Owner aesthetic acceptance remains separate.

[Build instructions](docs/native-build.md) ·
[Hydrology](docs/hydrology-development.md) ·
[Validation and exact scope](docs/validation-0.5.1.md) ·
[Development handoff](docs/next-agent.md)

Own code is [MIT](LICENSE). Historical Epic Terrain data retains wonderfulaichen's
MIT notice; DH, Zoom and the native tooling retain their respective notices.
[Credits and inspirations](CREDITS.md) remain authoritative. Source packages
contain no game assets, prepared resources, accounts, worlds or engine executable.
Independent project, not endorsed by Mojang or Microsoft.
