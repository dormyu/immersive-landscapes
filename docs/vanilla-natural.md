# Vanilla-based generation — Immersive Landscapes 0.6.0

New worlds use **descriptor 11 / `vanilla-natural-v7`**. Immersive Landscapes
keeps Minecraft's scale and native generator as its foundation, then adds shared
mountain/valley shaping, bounded catchment hydrology and gradual vegetation
transitions. It is a native mod for the pinned Horizon Rust runtime.

The new river and regional vegetation behavior is saved with each new world's
recipe. Installing 0.6.0 does not replace an existing world's geography.

## Terrain foundation

The overlay keeps native continents, climate scale, height limits, sea level
and gameplay biome identities. Shared ridge, erosion and valley fields shape
broader mountain regions and connected lowlands. Terrain and biome placement
consume the same fields. Base 3D rock-detail amplitude remains 0.35; it was not
retuned in this release. Native cave, ore and structure definitions remain in
use. Nether and End generation are outside this overlay.

This is not a plate-tectonic simulation or an all-height terrain replacement.
Recognizable native formations, including eroded-badlands pillars, can remain.
Rivers may alter local ground and sediment; retaining native cave/ore recipes
does not mean every block touched by a river remains unchanged.

## Catchments, rivers and shores

Native Rust hydrology constructs bounded, seed-dependent drainage graphs with
tributaries, shared confluences and graded channel beds. Coastal catchments,
suitable inland lake receivers and conservative delta distributaries remain
part of the recipe. A delta is not forced onto every river mouth. Lake discovery
and drainage coverage are bounded rather than a global continental simulation.

`rivers_v6=1` adds the 0.6.0 behavior over the retained `rivers_v5` policy:

- Meanders move across a reach instead of compressing its downstream stations.
  Forward-oriented curve handles preserve confluence positions and reduce
  hooks and sharp reversals.
- High banks receive a wider, bounded transition. Low floodplains remain
  conservative; nearby mountains are not globally flattened.
- A dry bank margin no longer creates extra water merely because its profile
  lies below a planned water level. Existing native water is preserved.
- Dry-bank excavation stops at its planned voxel waterline. If native ground
  is already lower, that lower ground is preserved instead of being raised.
  A coarse height sample can therefore no longer cut a real high shore deep
  below the river and expose an aquifer's side. This rule excludes channel
  cores and lake columns, leaves water levels unchanged, and applies only to
  the natural `rivers_v6` recipe. Saved v10 recipes retain their previous rule.
- Where the profile detects two disjoint wet intervals and a river core is
  involved, a bounded opening through the higher bed can connect them without
  raising the receiving water level. The extra opening is limited to 12 blocks.

The geometry policy is shared by real terrain and scalar/grid distant sampling.
This does not make native surface approximations exact replicas of carved
chunks. Native cavities and aquifers may have a deeper real bottom than the
surface approximation reports. Fill-only lake columns preserve those bottoms
and add water above them. Fresh current-source seed 42 native diagnostics confirm
both exact fill-only cases and the preserved adjacent aquifer. The current
4d7c4fec client separately passes nine loaded water-block checks and two reviewed
pond/outlet images, including the descending flowing connection.
Scheduled fluid updates can complete a waterfall after a chunk enters the world;
a static pre-physics water-interval test is not the final liquid state.

Some voxel banks and native cliffs remain steep. In the fresh c43610a bounded
river-valley fixture, the 95th percentile of bank height above water changes 9→3
blocks while its maximum remains 10. The exact delta regression separately
protects four dry banks 65→63 and leaves four adjacent water columns unchanged. Those measurements do not establish that
every shore is smooth. See [0.6.0 validation](validation-0.6.0.md).

## Regional vegetation transitions

`ecotones_v3=1` extends the previous temperate/cool policy into six explicit,
overlapping climate neighborhoods:

| Neighborhood | Examples |
| --- | --- |
| Temperate | Plains, forests, meadow, cherry grove, dappled forest, tall birch |
| Boreal and cool hills | Taiga, old-growth conifers, compatible forests/plains, windswept terrain |
| Snowy and alpine | Snowy taiga/plains, grove, snowy slopes, cold peaks, ice spikes |
| Dry and warm | Savannas, desert and badlands |
| Tropical and seasonal | Jungles, bamboo jungle and compatible savanna/plains edges |
| Wetlands | Swamp/mangrove and compatible forest/jungle neighbors |

The policy recognizes 38 native biome IDs; 36 participate in those neighborhoods.
Pale gardens and mushroom fields deliberately retain their special vegetation.
Aquatic, cave and foreign biomes are not made generic transition biomes.

At a supported boundary, native vegetation becomes less likely as its local
weight falls, and eligible neighboring features become more likely. Features
shared by multiple biomes are scheduled together instead of independently
stacking both full densities. Native placement, soil and climate constraints
still apply. This is a probability policy, not a promise of a fixed tree count.

At junctions, all participants choose the same supported neighborhood. A climate
outside that neighborhood keeps its native vegetation. Unrelated climates are
not blended through a chain of intermediate memberships. Recursive donor graphs
are checked with finite depth/work limits before use.

The existing bounded cache and sampling lattice are retained. Expanding the
weight vector adds approximately 0.9 MiB at the 4096-entry cache limit, not a
cache that grows with every explored column. This estimate is for the expanded
weight storage, not the complete mod or renderer's memory use.

Gameplay biome IDs remain native. Grass, foliage, water and sky tint handling is
also native; this release does not smooth every tint boundary. Surface blending
is restricted to eligible dry native soil, primarily the grass/podzol policy.
It is not universal sand/terracotta/stone repainting. River beds, wet margins and
native openings are excluded from that soil repaint. Snow can change when the
native weather feature sees different tree cover or plants.

## Saved recipes and updates

| World | Behavior in 0.6.0 |
| --- | --- |
| New world with type **Immersive Landscapes** | Saves descriptor 11 and the exact `vanilla-natural-v7` recipe |
| Existing descriptor 10 / `vanilla-natural-v6` | Retains its saved recipe; no `rivers_v6` or `ecotones_v3` marker is added |
| Existing descriptors 4–9 | Supported saved-recipe paths remain; no silent migration |
| Existing ordinary vanilla world | Remains vanilla |
| Archived descriptor 1–3 experiments | Require their historical pinned builds |

The current 4d7c4fec client reopened a descriptor10 world and verified its saved
descriptor and all20 recipe files unchanged. A newly created v11 world's placed
block survived a full process restart; vanilla saves remained vanilla across
mod disable/re-enable. Older descriptors4–9 remain supported by the code; they
were not all rerun as new0.6.0 gameplay fixtures.
Historical runtime results stay attached to their original releases.

Create a new world to see the new geography. Do not edit a saved descriptor or
copy new markers into an old world to force an upgrade. A missing saved IL data
pack produces a restore-from-backup error rather than regenerating it silently.

New worlds receive the embedded overlay automatically. The two original
`offset` and `base_3d_noise` aliases are copied from the owner's prepared native
resources into that world's private data pack. Public source packages contain
the alias instructions and original overlay, not a dump of Minecraft resources.
Optional external presets are read and copied only at world creation, with their
actual identity validated instead of being stamped as the embedded recipe.

## Runtime and rendering scope

Use the matching native Horizon build and enable Immersive Landscapes. The
creation-screen world type is explicitly **Immersive Landscapes**; no prepared
map or manual data-pack selection is needed. The current frozen4d7c4fec executable
passes solo/save/restart and two-client loopback checks. Four regional images and
the corrected delta32→8→32 sequence have been reviewed with bounded scope.
Structures and villages still use
the native definitions; a village redesign is a separate future task.

Distant Horizons dormyu.16 and official Horizon Zoom 2 remain separate mods.
Initial DH filling/refinement can be incomplete, and changing render distances
requires additional work. A fixed diagonal ocean color boundary in the reviewed
delta was traced to equal-height deep-lukewarm/warm-ocean biome tints, not a
geometric handoff. This release does not add interpolation to those water tints. Native water/cave bottoms and distant approximations
are not identical in every untouched column. These limits, performance and
installed Prism verification belong to the [versioned validation report](validation-0.6.0.md).
Current-runtime performance results belong to the versioned validation report.
Final installation, Prism and publication readback are recorded separately in
release notes/local provenance; source documentation alone is not distribution proof.
Owner aesthetic acceptance of0.6.0 has not been recorded.

[Hydrology details](hydrology-development.md) · [Native build](native-build.md) ·
[Credits and inspirations](../CREDITS.md)
