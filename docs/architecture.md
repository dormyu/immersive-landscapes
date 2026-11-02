# Terrain core and integration boundary

## Current pipeline

1. `HeightField::generate` creates a finite, centred regional raster. A curved
   uplift belt, broad shoulders and coherent noise define macro relief. This is
   an artistic tectonic approximation, not a plate-collision solver.
2. Climate remains continuous. A prevailing west-to-east wind approximates a
   rain shadow; altitude affects temperature. No biome selection is applied yet.
3. `Basin::compile` identifies boundary-connected ocean and runs priority-flood.
   Its drainage parents form an acyclic graph. Descending accumulation conserves
   the simplified rainfall/runoff budget.
4. Lakes retain spill elevations. River records describe a centre line, discharge,
   width, depth and floodplain. Gentle high-flow mouths can propose distributary
   fans; unknown water outside the compiled region is rejected.
5. `Landscape::new` builds buckets for river geometry. `sample(x, z)` interpolates
   macro terrain and climate, applies channel cross-sections, and returns ground
   and optional water elevation. The immutable result can be shared with `Arc`.

Drainage is O(N log N) with O(N) raster storage. A surface query examines the
channels indexed in its local bucket; it does not traverse the river network.
Dense junctions can still have more candidates and need profiling. There is no
per-frame water simulation, thread-per-chunk work or Java runtime dependency.

## Required Horizon adapter

Both chunk generation and far-terrain sampling must use the same versioned
landscape data and coordinate convention. The server owns seed, generator
version and generation parameters. Clients need compatible registries and a
bounded LOD protocol; they must not guess a multiplayer seed or generate a
different terrain when the server refuses data.

Do not regenerate edited terrain from pristine samples. Full loaded columns and
persisted player edits need precedence in distant data. New generation settings
must not silently reinterpret old saves. The integration still needs deliberate
height-layout, lighting, save/reload, reconnect and two-client tests.

## Deliberate limits of this first prototype

- It is one finite regional landform, not yet an infinite seamless world. Changing
  grid size/cell size changes the region and its climate. A query outside the
  compiled extent returns `None`; adapters must handle that explicitly.
- Heights and hydrology use floating-point calculations. Repeated runs with the
  same build are tested; cross-platform bit-identical persisted worlds are not
  promised yet. Versioned serialization and golden fixtures remain work.
- Rainfall, runoff fraction, river width and delta thresholds are provisional
  engineering parameters, not final gameplay balance or calibrated hydrology.
- Delta fans conserve their assigned flow, but there is no sediment deposition,
  tidal estuary model, erosion chronology or final delta voxel geometry.
- Lake shore interpolation and overlapping channel junctions need more varied
  fixtures. Climate fields are continuous, but final biome materials/vegetation
  have not been implemented.
- There is no cave, aquifer, structure, ore, spawn-placement or voxel adapter yet.
  No playable mod, height-limit compatibility, multiplayer or FPS claim follows
  from a passing standalone test.

## Performance evidence

The CLI reports regional compilation time separately from CSV output. A default
65,049-cell sample was previously measured near 28 ms on the author's machine;
this is a single offline measurement, not an in-game benchmark. Record hardware,
build, seed, resolution and repetitions before using a number for comparison.

Next profiling targets: regional compilation peak memory, indexed sample cost,
chunk generation latency, server tick time, bounded LOD bandwidth, and frame-time
percentiles with two clients. Keep substantial headroom for the rest of the pack.
