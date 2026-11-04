# Native development build

The native adapter targets Horizon `f42ef42` and builds together with the separate
DH adaptation. Run `tools/native.py generate`, `stage`, `build` or `package`.
The current Windows workspace uses the sibling `RustCore/development` toolchain,
temp, cache and build guard. Source `Enter-RustDev.ps1`; set `RUSTUP_TOOLCHAIN=stable`.

For the full local three-mod laboratory, pass the existing official Zoom bundle
as an additional input; the base wrapper supplies DH and this tool supplies IL:

```powershell
python tools/native.py stage --with E:\Games\Minecraft\RustCore\development\horizon-mods\native\target\zoom-2.0.0.hnmod
```

An IL/DH-only headless worldgen probe is not a complete three-mod client build.

This Nightly requires horizon-mixins `ddd9700` plus local compatibility fixes:
recognize Rust's built-in `recursion_limit` crate attribute as inert, and resolve
concrete `Self` in cancellable constructor HEAD return types (`Result<Self, E>`).
The latter is required by the unreleased0.4 hydrology branch. HEAD's handling of
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
documented patched tool / combined build or the tested prebuilt DormLab executable.
Game resources, accounts and worlds are never part of this source package.

## Current generation path (0.3.1)

This section describes the installed0.3.1 recipe. The unreleased0.4 branch adds
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
with schema1, descriptor_version5 or6, preset, author, based_on and optional
vanilla_aliases. The marker uses that actual identity. Symbolic links are
refused. Such presets are an explicit development override, not a claim
that arbitrary Java data packs are compatible.

## Named creation-screen type

The enabled mod owns the normal-world generation slot, whose creation-screen
name is now **Immersive Landscapes**. This is the working generation recipe under an
accurate display name. The persisted Horizon identity remains `minecraft:normal`,
with descriptor6 and the world-local data pack for new worlds. Old v4/v5 worlds keep
their Epic/v1 recipe; this update does not migrate them. Flat, large-biome and other
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
cargo --config profile.fast.package.horizon-client.incremental=false --config profile.fast.package.horizon-client.opt-level=2 --config profile.fast.package.horizon-client.codegen-units=64 build --offline --locked -j 1 -p horizon-client --bin horizon-client -p horizon-client-lib --bin horizon-ctl --profile fast
```

Run from the mixed stage, with CARGO_TARGET_DIR pointing at the build target.
The local guarded resume script is RustCore/qa/dh-handoff-2026-11-03/resume_build.py.
The source package also supports the normal upstream fast profile when more
RAM is available. Frame results describe the actual measured binary, not an
assumed benefit from a compiler flag.
