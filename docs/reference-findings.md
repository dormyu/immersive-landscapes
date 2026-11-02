# Reference findings — 2026-11-02

## Result

Epic Terrain is a useful source of actual generation recipes. Its MIT JSON data
can run through Horizon's native Rust world generator after a narrow conversion.
This is more concrete than recreating a promotional image or adjusting the
rejected Gaussian-belt prototype. It does not establish final visual acceptance.

The comparison fixture remains **Epic Terrain**, originally by wonderfulaichen.
It is separate from Immersive Landscapes. Direct reuse in our release must retain
the original MIT notice and identify the derived files, rather than calling all
reused recipes original work.

## Exact inputs and reproducible preparation

- Source: <https://github.com/wonderfulaichen/Epic-Terrain>.
- Revision: `08507bd3d606ad440b40a5fa726e0098d7309347`.
- Source pack: `versions/1.20.5-1.21.4`, v0.1.4 Beta.
- Native target: Horizon `c9d474dd9116e84c243f8992d75c11314df56f06`, Minecraft 26.3.
- Use Horizon's `horizon-port-pack upgrade-datapack`, followed by our
  [reference preparation tool](../tools/prepare_epic_reference.py).

```powershell
python tools/prepare_epic_reference.py --source <Epic-Terrain-checkout> --converter <horizon-port-pack.exe> --output <new-reference-pack-folder>
```

The tool does not download anything or overwrite an existing directory. The
output includes Epic's `LICENSE`, `PROVENANCE.json` and the converter's report.
Game resources, worlds and credentials are not part of the public repository.

Changing only `pack.mcmeta` failed with `invalid world generation field: type`.
Horizon's converter handles the density/noise/material registry changes. Further
compatibility changes are:

1. Flatten old biome carving-step lists.
2. Spell `default_fluid` as the pinned Horizon pipeline actually reads it.
3. Order pumpkin before normal sugar cane to avoid a shared decoration-order cycle.
4. Archive one unreferenced negative-amplitude noise outside the registries,
   after checking that no graph names its identifier.
5. Add explicit lake placement/protection predicates and flatten its old nested
   biased-to-bottom count provider. Historical pond-placement parity is unproven.

Before the last fix, the native generator reported
`etn:lake_water_surface: invalid world generation field: can_place_feature`.
After it, `OverworldGenerator::unsupported_features()` returns an empty list.
This checks registration, not every possible runtime feature placement.

## Runtime and sampling evidence

Local artifacts: `../RustCore/qa/immersive-landscapes-2026-11-02/` relative to the
project checkout. The pinned source checkout is untouched.

- Independent `epic-probe` profile, IL disabled and DH enabled for comparison.
- Seed `8675309`; fresh world opens, real chunks load, decode errors are zero.
- Actual client block query at `-3072 347 -2560` returns stone: the pack's
  extended build height is present in the client, not just in a preview.
- The final reference world's emerald marker survived save, disconnect and
  reopen. An ordinary control-off quick-play smoke completed 60 world frames.
- A native diagnostic compares undecorated proto chunks with direct far sampling.
  Ten selected dry mountain/foothill columns agree exactly in top height.
- Water differs at selected river sites. At `8192 -3072`, the proto chunk's
  water top is Y=105 while the far sampler retains Y=108 with no water field.
  At `8192 -2560` the corresponding heights are 106 and 108.
- A coarse 1,089-point survey across a 16,384-block square locates mountain and
  river candidates. It is not an exhaustive maximum-height or river-connectivity test.

The native far sampler applies material rules only near the top of its density
column and omits slope rules. Epic also sculpts water/terrain through material
rules. This is a confirmed compatibility concern for river rendering. It does
not prove that every visible DH artifact has that cause.

The final preparation tool reproduced all 88 registry JSON files semantically
and retained the original MIT license. It also refused an existing output
directory without changing its contents.

A 10-second visible1080p static sample at near16, DH **off**, on a warmed mountain
view measured 1120.5 average FPS / 1.7423 ms P99. This is a diagnostic baseline;
it says nothing about near32 + DH, movement, new terrain or a heavy modpack.

The local `../RustCore/Show-EpicReference.cmd` opens the pinned reference runtime
and `Epic-Reference-v2` with control off. The profile currently leaves DH off so
the actual terrain can be inspected independently; `/dh on` enables comparison.

Screens from high above the near radius also expose near/far coverage gaps.
Those captures must not be presented as proof of the terrain's actual shape.
Retain both DH-on and DH-off views when diagnosing them.

## Consequences for Immersive Landscapes

- Adapt useful noise/spline and surface recipes with actual attribution.
- Keep procedural seed generation. Tellus demonstrates a geographic-data route,
  but real Earth downloads are not the chosen default.
- Preserve our connected drainage work; Epic's noise-based rivers are not proof
  of a drainage graph, monotonic source-to-mouth flow or distributary deltas.
- Validate ground, water, material layers and canopy against real chunks before
  accepting a distant sampler. Increasing DH quality cannot repair wrong data.
- Batch native LOD queries at the section's true footprint. The current DH local
  adapter calls one width-1 column per point; its remote adapter already batches.
- Do not equate successful import, a static FPS number or an attractive screenshot
  with the owner's requested final result. Representative movement, low-PC settings,
  solo persistence and two-client checks remain acceptance work.
