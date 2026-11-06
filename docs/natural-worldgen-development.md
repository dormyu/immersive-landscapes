# Combined natural generation

Development branch `feat/natural-worldgen-v5`, author **dormyu**.

The combined recipe is `vanilla-natural-v6`, saved descriptor **10**. It adds
independent exact markers `rivers_v5 = 1` and `ecotones_v2 = 1`. All 13 density
inputs from descriptor 8 remain byte-identical. Saved descriptors 4–9 retain
their own recipes; reopening a world does not insert either new marker.

The river change adds bounded discovery of native inland receiving water,
terrain-constrained reach smoothing and conservative coastal distributaries.
Distributaries split the trunk's discharge and check native terrain across their
wet width. They are rejected at rock banks, high cliffs or conflicting channels.
This is bounded receiver-based drainage, not a planet-wide simulation.

The independent vegetation pass changes placement membership near compatible
biome boundaries. Native feature ordering, biome identities, structures and ores
remain native. Scheduler-only donor aliases never enter saved or network biome
data. The expanded policy covers ten native temperate/cool-climate biomes. It also
allows a direct forest/birch-to-conifer transition where dry native ground and
local temperature permit it. Snowy, hot and wetland boundaries remain native.

Current status: 53 calculation tests, all-target Clippy and combined recipe checks
pass. These are source/calculation checks only. Native compilation,
actual decorated terrain, save compatibility, two-client behavior and performance
have not been accepted for this combined recipe. This branch is not the installed
DormLab version. The separate 0.4.1 terrain fixes have native evidence, but DH10
failed the fresh-world render-distance visual check. DH11 is under visual review;
neither candidate has been delivered to the lab.

The next checks, in order:

1. Compile native hooks and exercise scheduler seed/order and scope cleanup.
2. Discover actual lake receivers, distributaries and compatible biome borders;
   verify native chunk/scalar/grid agreement and worker/cache determinism.
3. Inspect actual new-world river, lake, mountain and vegetation views. Retest the
   known straight reaches and angular pond shores, not only the best-looking view.
4. Build the optimized client once the visual checkpoints pass; check ordinary
   world creation, save/reopen, old recipes, two Rust clients and near32/DH128.
5. Install with rollback, preserve personal worlds/settings, verify Prism and
   publish matching source bundles and documentation.

No successful calculation, source check or image alone substitutes for these
runtime checks or for the owner's separate visual acceptance.
