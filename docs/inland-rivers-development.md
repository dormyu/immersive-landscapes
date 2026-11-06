# Inland receivers and river reaches

Development branch: `feat/inland-rivers`. Author: dormyu.

This code is opt-in through the exact density marker
`immersive_landscapes:rivers_v5 = 1`. The shipped v8 preset and the separate v9
ecotone preset do not contain this marker. Integration into a new saved-world
recipe is pending; existing recipes continue through their original branches.

The bounded drainage search can now use an existing native inland lake as a
receiver, at that lake's actual water level. It checks several low points, requires
a water footprint larger than a single block, and searches for a real shore.
Native water at sea level also terminates drainage in river/beach biomes, avoiding
an artificial raised pool beside the sea. Only native still water can be selected;
lava is excluded. The native surface resolver is shared and cached, with the
existing weak pipeline reference and hydrology recursion guard.

Long reaches receive a bounded smoothing pass before the existing meander field.
Tributaries retain a shared confluence vertex; proposed lateral shifts cannot cut
through terrain more than three blocks above the original coarse surface.

A substantial river on a low marine plain can split into a distributary. The
branch connects to the existing river, shares its initial water level, descends to
the sea, and takes a fraction of the trunk's discharge. Both resulting widths are
adjusted. Branches are rejected at high cliffs, inland mouths, basin boundaries
and conflicting channels. At most two branches are planned per bounded basin.

Current evidence: 41 calculation tests and Clippy pass, including native-lake
levels, rejection of an isolated water block, shared fork discharge and rejection
of a cliff-cutting delta. These are calculation checks. The modified native
adapter, actual-game river shapes, scalar/grid parity, cold generation cost and
two-client behavior have not yet been verified for this branch.

Next gate: combine the reviewed opt-in native integration with the independent
ecotone work under a new descriptor, build once, inspect actual inland rivers,
mouths and coastal water first, then run parity, persistence and performance
checks. Do not install this branch or call it an accepted release before those
gates pass. Terrain without a discovered native receiver keeps its native water;
worldwide river coverage remains bounded by receiver discovery.
