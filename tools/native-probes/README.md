# Native generator probes

These standalone Rust examples exercise the **staged Horizon f42ef42 world crate
with Immersive Landscapes compiled into it**. They are separate from this
repository's calculation tests. Use the combined mixin-tool compatibility patch
and staging procedure in [native-build.md](../../docs/native-build.md).

You supply your own privately prepared Minecraft 26.3 resources, including the
registry and `worldgen/densities.json`. No game assets, accounts or saved worlds
are included. The probes generate TERRAIN chunks in memory and write JSON reports
and optional DEM cache files; they do not open a player's save.

| Source | Check | Arguments after executable |
| --- | --- | --- |
| `native_hydrology_probe.rs` | Selected river columns: actual blocks, scalar/grid water and top materials, cut liners, preserved uncut geometry, two independent workers | `RESOURCES PRESET REPORT SEED` |
| `native_bank_probe.rs` | All 16,384 columns in the 64-chunk reported pond region; dry-bank parity and adjacent planned lake surfaces | `RESOURCES PRESET REPORT 42` |
| `native_route_probe.rs` | Selected drainage-route centreline points become actual water | `RESOURCES PRESET REPORT SEED` |
| `native_ocean_probe.rs` | Native receiving sea-level water is not raised in a chosen chunk rectangle | `RESOURCES PRESET REPORT SEED MIN_CX MAX_CX MIN_CZ MAX_CZ` |

The bank rectangle is fixed at chunks x=-148..-141, z=-151..-144. Ocean bounds are
inclusive chunk coordinates, not block coordinates. The ocean probe targets
receiving sea-level water, so choose a coastal region rather than an arbitrary
inland depression.

## Build on Windows

Adjust the example project root and resource location. Keep the checkout, staged
source, resources, toolchains, Cargo caches, TEMP/TMP, reports and generated caches
on your chosen build drive. In DormLab that drive is E:, never C:. Install/select
stable Rust in the indicated toolchain directories before staging and building.
Use **one Cargo job** on a 16 GiB machine and no concurrent engine build.

Run from PowerShell after completing the pinned native staging procedure:

```powershell
$buildRoot = 'E:/MinecraftDev' # Replace with your own build-drive directory.
$il = Join-Path $buildRoot 'immersive-landscapes'
$stage = Join-Path $buildRoot 'distant-horizons/target/stage'
$resources = Join-Path $buildRoot 'prepared-resources' # Your private resource set.
$preset = Join-Path $il 'presets/vanilla-natural'
$qa = Join-Path $buildRoot ('native-probe-results/' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
$env:CARGO_HOME = Join-Path $buildRoot 'toolchain/cargo'
$env:RUSTUP_HOME = Join-Path $buildRoot 'toolchain/rustup'
$env:RUSTUP_TOOLCHAIN = 'stable'
$env:CARGO_TARGET_DIR = Join-Path $buildRoot 'target-probes'
$env:CARGO_BUILD_JOBS = '1'
$env:TEMP = Join-Path $buildRoot 'temp'
$env:TMP = $env:TEMP
$env:PATH = "$env:CARGO_HOME/bin;$env:PATH"
$examples = Join-Path $stage 'crates/horizon-world/examples'
New-Item -ItemType Directory -Force -Path $qa,$env:TEMP,$examples | Out-Null

foreach ($name in @('hydrology','bank','route','ocean')) {
    $source = Join-Path $il "tools/native-probes/native_${name}_probe.rs"
    Copy-Item -LiteralPath $source -Destination (Join-Path $examples "il_${name}_probe.rs")
}
Push-Location $stage
try {
    cargo build --locked --jobs 1 --profile fast -p horizon-world --example il_hydrology_probe --example il_bank_probe --example il_route_probe --example il_ocean_probe
    if ($LASTEXITCODE -ne 0) { throw 'Native probe build failed' }
} finally { Pop-Location }
$bin = Join-Path $env:CARGO_TARGET_DIR 'fast/examples'
```

No manifest changes or extra dependencies are needed: the staged world crate
already provides `horizon-core` and `serde_json`. Add `--offline` when all locked
dependencies are already available on the build drive. Restaging may remove the
copied examples; copy them again afterwards.

## Run and inspect

Use a fresh cache for a cold run and the same directory for reopen. The optional
cache path is an environment variable, not another positional argument:

```powershell
$env:HORIZON_IL_CACHE_DIR = Join-Path $qa 'cache-seed9072026'
& "$bin/il_hydrology_probe.exe" $resources $preset "$qa/hydrology-9072026-cold.json" 9072026
if ($LASTEXITCODE -ne 0) { throw 'Cold probe failed; inspect the written report' }
& "$bin/il_hydrology_probe.exe" $resources $preset "$qa/hydrology-9072026-reopen.json" 9072026
if ($LASTEXITCODE -ne 0) { throw 'Reopen probe failed; inspect the written report' }

$env:HORIZON_IL_CACHE_DIR = Join-Path $qa 'cache-seed42'
& "$bin/il_bank_probe.exe" $resources $preset "$qa/banks-42.json" 42
if ($LASTEXITCODE -ne 0) { throw 'Bank probe failed' }
& "$bin/il_route_probe.exe" $resources $preset "$qa/route-42.json" 42
if ($LASTEXITCODE -ne 0) { throw 'Route probe failed' }
& "$bin/il_ocean_probe.exe" $resources $preset "$qa/ocean-42.json" 42 143 160 -184 -162
if ($LASTEXITCODE -ne 0) { throw 'Ocean probe failed' }
```

For cache verification, compare the two hydrology reports' `points` arrays;
they must match, and reopen `dem_cache` must show hits and zero misses. Other
regression seeds are 4312740467388825473 and 42. Coastal ocean boxes used for
9072026 and 4312740467388825473 are respectively `-90 -65 -195 -165` and
`-150 -130 -175 -150`.

Keep failing reports. A native far column intentionally approximates subsurface
material layers: arbitrary lower spans are **not** an exact material oracle.
The hydrology probe records those differences while asserting the mod's affected
geometry, water, riverbed top and liner guarantees. These probes do not establish
decorated-world appearance, DH rendering, FPS, save/reopen gameplay, multiplayer
or owner acceptance; those require separate actual-game checks.

Original probe code: dormyu, under the repository's MIT license. Native Horizon
and separately staged mods retain their own licenses.
