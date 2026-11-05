Current0.4.0 adds [native coastal hydrology](hydrology-development.md) and descriptor7
over this preserved density foundation. [Current validation](validation-0.4.0.md).

# Vanilla-based generation — 0.3.1

The owner asked for ordinary Minecraft terrain governed by natural rules.
This supersedes the complete Epic preset as the default for new worlds. It does
not approve the new candidate automatically.

`vanilla-natural-v2` changes seven density functions. The six v1 functions remain;
base_3d_noise is multiplied by0.35 through a private native alias to reduce
detached surface formations. This retains 3D detail and the final cave pipeline. All vanilla climate,
continentalness, height, dimension, biome, vegetation, cave, ore and structure
definitions remain in the underlying generator. The shared ridge field combines
broader and local variation; a monotone valley remap widens eroded lowlands more
than mountain passes. A localized inland floor correction preserves ocean terrain.
The same fields feed terrain and biome selection, including native far sampling.

Original offset and base_3d_noise functions are snapshotted from the owner's prepared resources
into the new world's private data pack. The mod publishes the alias instruction
and its own overlay, not the original registry file. Both embedded and external
presets identify themselves through IL-PRESET.json; external tuning is no longer
mislabelled with a hardcoded Epic identity.

Descriptor6 identifies v2. Descriptor4/Epic and descriptor5/v1 worlds remain readable
with their saved recipes and are not migrated. Reducing the amplitude to zero
was compared and rejected because it made rock detail too smooth. The0.35
variant was checked in two native scenes, including the reported seed
6719129005571520046. This is an assistant-selected tuning value, not an author
requirement or a universal guarantee against floating blocks. Existing vanilla saves are skipped.
The title-screen type remains Immersive Landscapes while the mod is enabled.

Historical c9d474d native probes compared three seeds, including the owner's ordinary-world seed,
on 4,225 points each over a 4,096-block square. The new candidate retained 33–46
biomes in these samples and comparable heights, with zero unsupported generator
features or override warnings. These measurements are limited samples, not proof
of visual quality, FPS, river connectivity or biome count for every spawn.

Remaining limitations: noise rivers have no validated catchment/outlet graph or
deltas; vanilla vegetation/biome boundaries are still discrete. Surface sampling
and the separate DH renderer require actual in-game checks. The generation probe
does not establish smooth movement or complete distant loading.

Current build and QA state is recorded locally in
`RustCore/reports/development-2026-11-03/WORK-IN-PROGRESS.md`.

Current version evidence: [0.3.1 validation](https://github.com/dormyu/immersive-landscapes/blob/v0.3.1/docs/validation-0.3.1.md).
