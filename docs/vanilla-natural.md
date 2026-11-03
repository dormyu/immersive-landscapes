# Vanilla-based candidate — 3 November 2026

The owner asked for ordinary Minecraft terrain governed by natural rules.
This supersedes the complete Epic preset as the default for new worlds. It does
not approve the new candidate automatically.

`vanilla-natural-v1` changes only six density functions. All vanilla climate,
continentalness, height, dimension, biome, vegetation, cave, ore and structure
definitions remain in the underlying generator. The shared ridge field combines
broader and local variation; a monotone valley remap widens eroded lowlands more
than mountain passes. A localized inland floor correction preserves ocean terrain.
The same fields feed terrain and biome selection, including native far sampling.

An original offset function is snapshotted from the owner's prepared resources
into the new world's private data pack. The mod publishes the alias instruction
and its own overlay, not the original registry file. Both embedded and external
presets identify themselves through IL-PRESET.json; external tuning is no longer
mislabelled with a hardcoded Epic identity.

Descriptor5 identifies this generation family. Descriptor4 worlds remain readable
with their saved recipe and are not migrated. Existing vanilla saves are skipped.
The title-screen type remains Immersive Landscapes while the mod is enabled.

Native probes compared three seeds, including the owner's ordinary-world seed,
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
