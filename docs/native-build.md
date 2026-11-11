# Native development build

<!-- CURRENT-DH18-DELIVERY -->
## Current verified delivery — IL 0.6.0 / DH dormyu.18 / Zoom 2

DormLab's underwater renderer repair is installed, checked through actual
Prism/bridge and [published](https://github.com/dormyu/distant-horizons-for-horizon-client/releases/tag/dormyu.18). IL and Zoom Rust are unchanged;
new worlds retain descriptor11 / vanilla-natural-v7 and existing recipes remain intact.
This DH update needs no new world.

The reproduced pale bare seabed strip is absent in reviewed native16→8→16/DH64,
full-process reopen and native32→16→32/DH128 images. Native water-light/model
fixtures, two Rust clients and installed Prism passed. A separate distant
warm-ocean/deep-lukewarm-ocean colour boundary remains visible: saved biome data
and DH debug views distinguish it from the repaired native/DH handoff.
No universal renderer or owner aesthetic acceptance is claimed.

**Performance tests were waived by the owner.** Preserve native32/DH128 Medium,
sim8, maxFps260 (unlimited), VSync off, nativeLOD0, workers Auto and control off.
3058 owner-world files and settings were preserved. All managed files stay on E:;
one Cargo job, no concurrent full builds, Teams and owner applications untouched.

Runtime `33992c7848f91b2a3a730c5e0d267edf57e188d2461ed152aa73129b651a2047`; compiled DH `3a8aa870971a3213f8436fe8e299eb95d30b4c73`;
published source/docs `d4c1d537611b1b0790d2b81bd2708b1fcd0bdc2a`. [Evidence and limits](renderer-dormyu.18.md).
QA: `E:\Games\Minecraft\RustCore\qa\underwater-seam-aquatic-2026-11-11`.
The retained DH17 report and older generator snapshots keep their historical scope.

## Select the current renderer source

Use [https://github.com/dormyu/distant-horizons-for-horizon-client](https://github.com/dormyu/distant-horizons-for-horizon-client) at tag `dormyu.18`
(commit `d4c1d537611b1b0790d2b81bd2708b1fcd0bdc2a`) for the DH checkout.
In the local workspace the wrapper reads `RustCore/development/distant-horizons`.
Its `tools/horizon.py package` creates the DH 18 bundle; the combined IL wrapper
then stages DH, IL and the explicitly supplied Zoom bundle together.
The patched-tool instructions below still apply to this engine pin. The DH 16
runtime hashes and performance numbers in the historical section are not DH 18 results.

The retained 0.6.0 / DH16 build snapshot below targets Horizon f42ef42 / Minecraft 26.3 with separate
DH dormyu.16 and official Zoom 2.0.0. New worlds use descriptor 11 / vanilla-natural-v7.
Read [validation](validation-0.6.0.md) for exact code pins, source match,
verified runtime checks and the scope of performance evidence. Final installation,
Prism launch and distribution readback are recorded separately in release notes
and local provenance, after the documentation/source bundle is frozen.
Run tools/native.py generate, stage, build or package as appropriate. On this
Windows workspace source RustCore/development/scripts/Enter-RustDev.ps1, use
stable Rust, E: temp/cache/output and one Cargo job. Source bundles require the
documented patched native tooling and privately prepared game resources.

For the full local three-mod laboratory, pass the existing official Zoom bundle
as an additional input; the base wrapper supplies DH and this tool supplies IL:

```powershell
python tools/native.py stage --with E:\Games\Minecraft\RustCore\development\horizon-mods\native\target\zoom-2.0.0.hnmod
```

An IL/DH-only headless worldgen probe is not a complete three-mod client build.

This Nightly requires horizon-mixins `ddd9700` plus local compatibility fixes:
recognize Rust's built-in `recursion_limit` crate attribute as inert, and resolve
concrete `Self` in cancellable constructor HEAD return types (`Result<Self, E>`).
The latter is required by the 0.4 native hydrology adapter. HEAD's handling of
unchanged body macros is now upstream. Unknown procedural attributes remain
restricted. The patch and its compiled two-mod regression test are in
`tools/patches/horizon-mixins-f42-compat.patch`; the MIT notice is in
`third-party/horizon-mixins-LICENSE`. Tested local tool commit: `46844bc`.

To reproduce that tool, clone `https://github.com/horizon-rust/horizon-mixins` onto E,
checkout `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`, and apply the bundled patch:

```powershell
. E:\Games\Minecraft\RustCore\development\scripts\Enter-RustDev.ps1
$env:RUSTUP_TOOLCHAIN = 'stable'
$env:CARGO_TARGET_DIR = 'E:\Games\Minecraft\RustCore\development\mixins-tool-target'
git apply E:\Games\Minecraft\immersive-landscapes\tools\patches\horizon-mixins-f42-compat.patch
cargo test --locked -p horizon-mixins-cli --test integration boundary::crate_recursion_limit_preserves_ordered_head_callbacks -- --test-threads=1
cargo test --locked -p horizon-mixins-cli --test integration boundary::concrete_self_head_constructors_compile_and_preserve_two_mod_order -- --test-threads=1
cargo build --release --locked -p horizon-mixins-cli
Copy-Item "$env:CARGO_TARGET_DIR\release\horizon-mixins.exe" E:\Games\Minecraft\RustCore\development\mixins-tool-hydrology\bin\horizon-mixins.exe
```

Run the commands from that fresh tool checkout. Our build wrapper selects this
binary through `HORIZON_MIXINS_BIN`. The DH build script supports that explicit
override. This Nightly's unpatched tool cannot index the gameplay library.

The focused constructor fixture and tool Clippy pass. The broader local Windows
tool run recorded88/99 integration tests passing; remaining failures report
Linux-only atomic stage replacement, Windows path diagnostics and missing
`foldhash0.1.5` in offline fixture dependencies. This is not an all-green tool-suite
claim. Native adapter checks are recorded separately in the candidate status.

This development `.hnmod` contains source, not a dynamically loaded game DLL.
The unpatched Horizon launcher cannot build these worldgen hooks yet. Use the
documented patched tool / combined build or an executable verified against the
matching frozen native-build receipt. Do not infer its installation from source
validation alone.
Game resources, accounts and worlds are never part of this source package.

## Historical IL0.6.0 / DH16 compiled source identity

The tested 0.6.0 code is `c43610a2fc73b2d092930ad47c4ba5b60ebe83db` with unchanged
DH dormyu.16 and official Zoom2. The frozen executable is
`4d7c4fec1e668fff43272a25696cd5ffe5b8846e78038382053e5e7b6c9e6a0c`.
The successful one-job build binds the same before/after stage hash across4,521
files to the resulting executable. The local `reports/native-build.json` pins
every compiled mod input; later functional/visual receipts pin that executable.
A later documentation-only commit may package the same source hashes; it is
not represented as a different gameplay build.

Current frozen-runtime cold/warm checks passed at actual32/DH128 Medium,
1080p/simulation8/VSync off. Static:368.4/368.9FPS; walking:345.6/397.5FPS;
fast480-block streaming:295.1/313.6FPS. Four further480-block legs completed
with247.5–301.8FPS and no chunk-streaming stop. Worst measured frame12.65ms.
See [validation](validation-0.6.0.md) for frame-time, load/settle and memory
measurements and their bounded scope; this is not heavy-pack acceptance.

## Native generation 0.6.0

Bounded coastal/inland hydrology and conservative delta branches now use the
opt-in `rivers_v6=1` policy for transverse bends, dry banks and bounded waterfall
joins. Regional vegetation ecotones use `ecotones_v3=1`, with six compatible
climate neighborhoods. Descriptor 11 adds these two exact markers without
changing the previous sixteen density inputs. Read the
[generation design](vanilla-natural.md) and [validation](validation-0.6.0.md).
Saved descriptors 4–10 retain their exact recipes; new features need a new IL world. DH16
regenerates derived LOD cache only. Historical notes below do not override the
current manifest or claim migration of existing worlds.

## Historical density foundation (0.3.1)

This section describes the preserved0.3.1 foundation. The0.4 adapter adds
the hydrology modules, watershed marker, execution-local cache and descriptor7;
see [its status and limits](hydrology-development.md). It retains saved v4/v5/v6.

`preset.generated.rs`, `preset_adapter.rs`, `prepare.rs` and `list.rs` implement
the generation path. The rejected custom ChunkGenerator/LevelShape
hooks have been removed. `tools/prepare_vanilla_natural.py` generates the seven
original density overrides in `presets/vanilla-natural`; `tools/native.py generate`
validates their SHA-256 and embeds that directory. The source and world copies
use identical LF bytes. Git's scoped EOL rule and the embedding tool reject
newline drift instead of publishing mismatched checksums.

Original vanilla offset and base_3d_noise definitions are read from the owner's
prepared resources when creating a world and saved under two private aliases
in that world's data pack. These definitions are not included in the source or bundle. Climate, biome, decoration, cave, ore,
structure and dimension definitions are not replaced. See
[the generation design](vanilla-natural.md) for the scope and remaining limits.

The cancellable HEAD on `WorldRequest::prepare_world` installs that data pack
before the normal engine loads it. Dedicated servers call the same method, so
their new normal worlds receive the same generation recipe. Existing vanilla
worlds are skipped; descriptor4, descriptor5 and descriptor6 worlds retain their own saved recipe. Older
prototype descriptors are refused rather than silently changing their terrain.

External presets under `<game>/horizon-mods/immersive-landscapes-preset` are
copied only into new worlds. They must contain `pack.mcmeta` and `IL-PRESET.json`
with schema1, a descriptor accepted by the current validator, preset, author, based_on and optional
vanilla_aliases. The marker uses that actual identity. Symbolic links are
refused. Such presets are an explicit development override, not a claim
that arbitrary Java data packs are compatible.

## Named creation-screen type

The enabled mod owns the normal-world generation slot, whose creation-screen
name is now **Immersive Landscapes**. This is the working generation recipe under an
accurate display name. The persisted Horizon identity remains `minecraft:normal`,
with descriptor 11 / vanilla-natural-v7 and the world-local data pack for new worlds. Old v4–v10 worlds keep
their own Epic/natural recipes; this update does not migrate them. Flat, large-biome and other
slots keep their own behavior.

`world_type_ui.rs` keeps a presentation flag, synchronized at native-mod discovery
and whenever mod settings are saved. `world_type_name.rs` updates only the
creation menu's normal-type translation, including next-choice narration.
`world_type_button.rs` fits the full name in the existing cycle button by omitting
the redundant localized prefix for this one value. Disabling IL restores Default
for new worlds; existing IL worlds keep their recipe. No per-frame callback,
generation work or added filesystem reads are involved in displaying the label.

On f42ef42 the creation screen belongs to `horizon-client-lib`. Its two UI
injections target that actual crate; gameplay hooks remain in `horizon-client`.
The original f42 port retained the six v1 functions and descriptor5. Version0.3.1
adds a seventh override (base_3d_noise amplitude0.35) and descriptor6/v2.
The native world-creation hooks and menu integration remain the same.
The earlier c9d474d build and its tool patch remain historical rollback material.

## Memory-limited Windows check

The verified16GiB lab build uses the fast profile with per-package overrides
for horizon-client: incremental=false, opt-level=2, codegen-units=64. Other
engine libraries retain their existing optimized profile. After staging, these
are ordinary Cargo overrides; use one job and an E: target/temp directory:

```powershell
cargo --config profile.fast.package.horizon-client.incremental=false --config profile.fast.package.horizon-client.opt-level=2 --config profile.fast.package.horizon-client.codegen-units=64 rustc --offline --locked -j 1 -p horizon-client --bin horizon-client --features horizon-client-lib/default --profile fast -- -C linker=E:\Games\Minecraft\RustCore\development\toolchain\rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe -C link-arg=/threads:1
```

Run from the mixed stage, with CARGO_TARGET_DIR pointing at the build target.
The local 0.6.0 receipts are
`RustCore/qa/completion-2026-11-08-bankfix`. The build driver records the exact
compiler working-set limit and memory floor; no user applications are closed.
This is a local build accommodation,
not a gameplay memory limit. The frozen control executable is pinned separately
in `reports/native-build.json`; it contains no new generator code.
The source package also supports the normal upstream fast profile when more
RAM is available. Frame results describe the actual measured binary, not an
assumed benefit from a compiler flag.
