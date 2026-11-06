# Credits

**dormyu** — project author and designer of Immersive Landscapes.
GitHub account: [dormyu](https://github.com/dormyu).

## Original implementation

The native world-creation integration, vanilla-natural density overlay, preset tuning/tools, drainage
and vegetation-transition code, tests and preview tools are original code by dormyu, licensed under MIT.
The old height-only prototypes were rejected and do not drive the current native
preset. Priority-flood, drainage graphs,
value noise and spatial indexing are general algorithmic techniques; the project
does not claim to have invented them.

## Design inspirations

- [TerraForged](https://github.com/TerraForged/TerraForged) and
  [ReTerraForged](https://github.com/racoonman2/ReTerraForged): landscape composition,
  rivers and climate at a larger scale.
- [FreeTerraForged](https://github.com/ETcodehome/FreeTerraForged): further research
  into that family of generators.
- [Tectonic](https://modrinth.com/mod/tectonic): large mountain ranges and continents.
- [Lithosphere](https://modrinth.com/mod/lithosphere): broad valleys and smooth
  regional transitions.
- [Epic Terrain](https://github.com/wonderfulaichen/Epic-Terrain), by
  wonderfulaichen: spline/noise composition, alpine relief, coasts and river
  surface rules. MIT source revision `08507bd3d606ad440b40a5fa726e0098d7309347`
  was adapted into an independent local Horizon comparison fixture. The
  [preparation tool](tools/prepare_epic_reference.py) preserves its license and
  records every compatibility change. Starting with `0.2.0-dev.1`, the full
  converted generation data is included in [presets/epic-compact](presets/epic-compact),
  retaining wonderfulaichen's [MIT notice](presets/epic-compact/LICENSE).
  The original terrain/river scale and complete native decoration are retained;
  dormyu's v4 preset reduces the climate wavelength. Since `0.3.0-dev.1`, new
  worlds instead use the original vanilla-natural overlay; existing v4 worlds
  keep their saved Epic-derived recipe. [Exact historical provenance and tuning](presets/epic-compact/IL-PRESET.json)
  identify the source revision, conversion changes and every registry checksum.
- [Tellus](https://github.com/Yucareux/Tellus): real-world elevation, climate,
  vegetation and direct distant-terrain sampling as visual/architectural
  references. No Tellus code is included in this MIT project.

Epic Terrain's data was directly reused in v4 and remains in its preserved recipe.
The current vanilla-natural overlay uses the owner's installed Minecraft/Horizon
registries and our own density functions. Its vanilla offset and base_3d_noise aliases are materialized
locally when creating a world; game registry files are not redistributed in the
native bundle. The other terrain
projects remain references; their Java implementation is not bundled.

## Separate platform and rendering projects

- [Horizon](https://github.com/horizon-rust/horizon) — Ludovic Marie and contributors;
  Rust client/server platform, MIT for its original code.
- [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) — levzzz5154 and
  contributors; native integration tooling.
  Our current build uses the original MIT code at `ddd9700` with dormyu's small
  inert `recursion_limit` attribute fix. The0.4 hydrology adapter also
  resolves concrete constructor `Self` in cancellable HEAD results, with an
  original two-mod regression fixture. Earlier boundary HEAD macro preservation
  is now upstream. The patch, compiled regression test and original MIT notice
  are bundled; see [native build notes](docs/native-build.md).
- [Distant Horizons](https://gitlab.com/distant-horizons-team/distant-horizons) —
  James Seibel and contributors; original distant-terrain project.
- [Distant Horizons for Horizon Client](https://github.com/denizsahin7744/distant-horizons-for-horizon-client)
  — Deniz Şahin and contributors; Rust port, LGPL-3.0-only. The modpack's local
  adaptation by dormyu is based on commit `53e116bad8bbe55e3e4d76504af721f1260afb81`.
  It remains separate from this repository and retains its upstream notices.
- [Official Horizon Zoom](https://github.com/horizon-rust/horizon-mods/tree/725ff7a41017c40104273e407264e0ebee87e567/native/zoom)
  — Ludovic Marie and the Horizon contributors; unchanged native Zoom 2.0.0, MIT.
  Its source and original license are distributed separately from Immersive Landscapes.
- Minecraft — Mojang/Microsoft. This project is not affiliated with or endorsed
  by Mojang, Microsoft or the projects named above.

Optional preview tooling uses Pillow, under its own license; it is not a runtime
dependency of the Rust generator.

The original native hydrology adapter uses general priority-flood and bounded
drainage-domain ideas. Concept references are Barnes, Lehman and Mulla,
[Priority-Flood](https://doi.org/10.1016/j.cageo.2013.04.024), and Michael K. Davis III,
[Plate-Local River Generation](https://arxiv.org/abs/2610.00301). Their implementation
code is not copied into this project; see [hydrology scope](docs/hydrology-development.md).

[minecraft-agent-skills](https://github.com/Jahrome907/minecraft-agent-skills),
by Jahrome907, was consulted for world-generation schema/reference validation.
Its Java loader instructions are not used by the Rust runtime. Its validator's
Minecraft namespace references were checked against the actual prepared26.3
vanilla registries; no skill code is included in the native mod.
