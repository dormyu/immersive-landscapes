# Credits

**dormyu** — project author and designer of Immersive Landscapes.
GitHub account: [dormyu](https://github.com/dormyu).

## Original implementation

The terrain, drainage, surface sampling, tests and preview tools in this repository
are original project code, licensed under MIT. Priority-flood, drainage graphs,
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

These are inspirations, not bundled dependencies or claims that their code was
ported. Future code or asset reuse must be recorded separately with its actual
source, revision and license.

## Separate platform and rendering projects

- [Horizon](https://github.com/horizon-rust/horizon) — Ludovic Marie and contributors;
  Rust client/server platform, MIT for its original code.
- [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) — levzzz5154 and
  contributors; native integration tooling.
  Our build uses the original MIT code at `0d6b103` with a small boundary HEAD
  macro-preservation patch by dormyu. The patch, regression test and original MIT
  notice are bundled; see [native build notes](docs/native-build.md).
- [Distant Horizons](https://gitlab.com/distant-horizons-team/distant-horizons) —
  James Seibel and contributors; original distant-terrain project.
- [Distant Horizons for Horizon Client](https://github.com/denizsahin7744/distant-horizons-for-horizon-client)
  — Deniz Şahin and contributors; Rust port, LGPL-3.0-only. The modpack's local
  adaptation by dormyu is based on commit `53e116bad8bbe55e3e4d76504af721f1260afb81`.
  It remains separate from this repository and retains its upstream notices.
- Minecraft — Mojang/Microsoft. This project is not affiliated with or endorsed
  by Mojang, Microsoft or the projects named above.

Optional preview tooling uses Pillow, under its own license; it is not a runtime
dependency of the Rust generator.
