# Native hydrology — 0.5.0

This document describes IL code `224a831`, descriptor **10**,
`vanilla-natural-v6`, on Horizon `f42ef42`. Native Q13 checks and optimized solo,
multiplayer and same-process world switching pass. The DH16 shoreline regression
also passes in its copied-world visual test. Final runtime measurements and reviewed images are recorded in [validation](validation-0.5.0.md).
The mod is working software in active development, with bounded coverage and
explicit saved recipes. Final release notes and the installed `native-build.json`
record later launcher delivery; source documentation alone does not prove it.

## Drainage and receiving water

`watershed.rs` builds bounded seed-dependent domains over native input terrain.
Priority-flood depression handling, contributing area and shared confluences
produce graded channels. Marine catchments retain priority inside their coastal
coverage; inland receivers extend drainage outside that coverage rather than
fragmenting an existing coastal river.

An inland receiver must be actual native still water with a usable footprint
and shore. A selected connected drainage root uses that receiver's real water
level. Lava and isolated one-block puddles are excluded. Sea-level water alone
does not make an inland lake an ocean. Receiver discovery is deliberately
bounded; this is not global continental hydrology.

Suitable large rivers on low marine plains can form distributaries. The new
arm starts on an existing channel, shares the fork's water level and takes part
of its discharge. Native checks sample its center and both wet edges. Cliffs,
narrow rock banks, conflicting channels and invalid inland mouths veto a branch.
There are at most two planned branches per bounded basin. Delta frequency and
worldwide coverage are not guaranteed.

Reach smoothing and lateral meanders use shared junction vertices and native
terrain guards. The v10 defaults use meander amplitude 28 and scale 80. Eleven
other previous density inputs remain byte-identical; these two shape values
and the new versioned features belong to the new saved recipe. Some constrained
inland reaches still contain long straight sections and sharp voxel bends.

## Shared rasterization

`hydrology.rs` resolves native ground once and uses a shared column plan for
actual proto terrain, scalar samples and distant grids. Natural rivers no longer
depend on coarse 4×8 density interpolation to cut their final banks. Exact
column changes run before native decoration, with common floor/water rounding,
roof handling and top-material rules.

New wet non-lake beds receive a supported one-block sediment finish, even when
the existing ground needs no excavation. A finish is only placed on an actual
solid block; it cannot create a suspended slab over a native carver opening.
Existing native water and wet lake bottoms keep their geometry.

A planned lake **channel core** may cross a dry saddle up to six blocks above
the lake surface. Its floor becomes `water - 1`, allowing at most seven blocks
of excavation at the limit. The diagnosed water-70 saddle at ground 74–75 now
connects; it is not a license to recut every lake. Non-core land, higher islands
and already wet bottoms remain intact. Saved v8/v9 paths retain their former
three-block threshold; the six-block rule requires `rivers_v5 = 1`.

## Vegetation interaction

The independent `ecotones_v2 = 1` pass supports ten temperate/cool biome IDs.
It changes only allowed native vegetation membership and limited suitable dry
soil. Native vegetation thins on one side while compatible donors gain weight
on the other; native feature ordering and seed indices are retained. Scheduler
aliases are never saved or sent as gameplay biome IDs.

Every IL river profile, including dry banks, takes priority over ecotone soil
painting. Native water, rock, snow and unsupported surfaces are excluded. The
ground policy is shared by near, scalar and grid paths. Native tree placement
can still make its ordinary local support-block changes. Structure and ore
features are outside the vegetation allowlist.

## Cost, cache and compatibility

There is no IL gameplay-frame or tick hook. Bounded caches and spatial bins
reuse the native surface and drainage work. Biome weights use an absolute
world-coordinate lattice and bounded anchor cache, not a distant biome scan
for every block. Seed/coordinate decisions avoid a chunk-local transition seam.

The instance owns a derived raw DEM cache under
`horizon-mods/.horizon-mod-data/immersive_landscapes/hydrology-v1`.
`HORIZON_IL_CACHE_DIR` is a development-only override. The cache checks engine
and native input identity, seed, grid and checksum. Shape-only tuning can reuse
the raw DEM while rebuilding the graph; changing input terrain requires a new
identity. Cache files are disposable, and are not world saves.

Saved recipes control behavior. Descriptor 10 adds exact `rivers_v5 = 1` and
`ecotones_v2 = 1` markers; marker absence retains the corresponding earlier path.
No old world's registry files are rewritten to opt it into v10. Constructor
and decoration scopes restore previous thread-local state, including nested
and error paths. Other mods that hook the same constructors still need a
compatibility review.

DH16 is a separate renderer adaptation. It preserves visited columns during
partial preparation, rejects obsolete publications, retires sessions away from
the frame thread and stops inventing buried boundary faces. Its new fingerprint
rebuilds old derived LOD data that may contain corrupted parent reductions.
This does not change IL terrain, real chunks or the IL DEM cache. The reviewed
copied-world 32 → 8 → 32 sequence no longer shows the former deep shoreline walls
or missing regions, including the delayed 60-second view. Small material/detail
differences remain. Final optimized preparation and frame measurements are
recorded separately; the opt0 visual check is not performance evidence.

Original implementation: dormyu, MIT. Existing [credits](../CREDITS.md) and
third-party notices remain unchanged. Implementation, sampled native coverage,
rendered appearance and owner approval are separate claims.
