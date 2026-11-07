# Vanilla / Immersive Landscapes 0.5.1 comparison

Fresh separate worlds, seed `4312740467388825473`; same frozen optimized native
client, actual distance16, DH and random block ticks disabled, 1600×900, FOV90,
noon and clear weather.
The vanilla world has no IL descriptor; the IL world has descriptor10/v6.
Both were fully loaded and idle before each unedited native capture.

| Scene | Camera x/y/z | Yaw / pitch |
| --- | --- | --- |
| River valley | `[-263.5, 123, -2328.5]` | `[148.4, 35.8]` |
| Low coastal catchment | `[-1590.5, 145, -2540.5]` | `[148.4, 35.8]` |
| Forest/plains transition | `[-1315.5, 98, 1563.5]` | `[135, 28.46279]` |

- **River valley:** vanilla has a dry grassy hollow. IL adds a continuous narrow watercourse toward the coast and changes the surrounding slopes. The long reach remains fairly straight and its voxel banks angular; this image does not establish fully natural meanders.
- **Coastal catchment:** the vanilla sandy ridge becomes a much lower coastal drainage area with channels around a long island in IL. Shoreline height and native surface/biome placement also change. Abrupt terracotta/sand/grass boundaries and steep water/ground steps remain visible and need further visual work; this is evidence of the difference, not coastal aesthetic acceptance.
- **Forest/plains:** the vanilla sloping clearing becomes a broader, lower clearing beside water and a mixed forest in IL. Because the terrain also changes, this pair does not isolate the vegetation-blending algorithm by itself.

These are comparisons of the complete mod with vanilla, not separate feature ablations. All six captures are fully populated near-terrain views with no HUD, and the differences are visible at the matched cameras. Far-distance fog is expected with DH disabled. Cloud placement and incidental entities can differ between runs.

These frames show specific locations, not every seed or biome. They do not
establish an FPS result or owner aesthetic acceptance. See the paired images
in the [README](../README.md).
