# River form correction — 0.4.1 candidate

Owner feedback rejects the straight trench and perforated forest coast in0.4.0.
The three screenshots were reproduced with DH disabled on seed4312740467388825473.
This is a native terrain defect, not something corrected by waiting for distant tiles.

Thev7 adapter mistakenly treated ChunkSurfaceLevel, an aquifer estimate, as the
actual surface DEM. In the reported forest it returned56 while native ground was
around68. The false flat below-sea catchment generated many intersecting cuts.

New v8 / vanilla-natural-v4 uses the unwrapped final-density surface. Aquifer
preliminary levels remain native. The saved v7 mode retains its old behaviour;
existing user chunks are neither overwritten nor silently repaired.

Initial native TERRAIN area comparison: DEM median error11→1 block; samples cut
deeper than8 blocks2303→14; small land components235→14 versus11 in the unchanged
foundation. These are scoped diagnostics, not final client or visual acceptance.

Channels use shared curved junctions, lowland meanders, variable width, asymmetric
banks, rounded cross-sections and shallow headwater starts.29 calculation tests
and Clippy pass. Final client images, network/save and performance checks are pending.

The first candidate's actual-game review also found raised rectangular water at
some mouths. New lake placement requires four dry corners sharing one spill level,
and receiving sea columns cap channel/lake water at sea level. Saved v7 retains
its exact old selection. A bounded32768-point native surface cache avoids repeated
density evaluations in coast search and overlapping DEMs; values are unchanged.
The first candidate was not installed.

Five original constant density files under immersive_landscapes:river_* store
amplitude, wavelength, width, depth and bank spread. They are snapshotted with
each world's recipe and bounded by the native adapter. Shape tuning can use
horizon-mods/immersive-landscapes-preset in an isolated test instance, without
recompiling Horizon. Update IL-PRESET.json checksums when editing such a preset.
Raw DEM cache identity excludes these shape-only parameters; drainage rebuilds.

Full inland catchments/deltas/ecotones and remaining DH artifacts are separate
open work. This candidate is not yet installed. Author dormyu, own code MIT.
