# Native development build

The native adapter targets Horizon `f42ef42` and builds together with the separate
DH adaptation. Run `tools/native.py generate`, `stage`, `build` or `package`.
The current Windows workspace uses the sibling `RustCore/development` toolchain,
temp, cache and build guard. Source `Enter-RustDev.ps1`; set `RUSTUP_TOOLCHAIN=stable`.

This Nightly requires horizon-mixins `ddd9700` plus a small local fix: recognize
Rust's built-in `recursion_limit` crate attribute as inert. HEAD's handling of
unchanged body macros is now upstream. Unknown procedural attributes remain
restricted. The patch and its compiled two-mod regression test are in
`tools/patches/horizon-mixins-f42-compat.patch`; the MIT notice is in
`third-party/horizon-mixins-LICENSE`. Tested local tool commit: `d4af4d8`.

To reproduce that tool, clone `https://github.com/horizon-rust/horizon-mixins` onto E,
checkout `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`, and apply the bundled patch:

```powershell
. E:\Games\Minecraft\RustCore\development\scripts\Enter-RustDev.ps1
$env:RUSTUP_TOOLCHAIN = 'stable'
$env:CARGO_TARGET_DIR = 'E:\Games\Minecraft\RustCore\development\mixins-tool-target'
git apply E:\Games\Minecraft\immersive-landscapes\tools\patches\horizon-mixins-f42-compat.patch
cargo test --locked -p horizon-mixins-cli --test integration boundary::crate_recursion_limit_preserves_ordered_head_callbacks -- --test-threads=1
cargo build --release --locked -p horizon-mixins-cli
Copy-Item "$env:CARGO_TARGET_DIR\release\horizon-mixins.exe" E:\Games\Minecraft\RustCore\development\mixins-tool-f42\bin\horizon-mixins.exe
```

Run the commands from that fresh tool checkout. Our build wrapper selects this
binary through `HORIZON_MIXINS_BIN`. The DH build script supports that explicit
override. This Nightly's unpatched tool cannot index the gameplay library.

This development `.hnmod` contains source, not a dynamically loaded game DLL.
The unpatched Horizon launcher cannot build these worldgen hooks yet. Use the
documented patched tool / combined build or the tested prebuilt DormLab executable.
Game resources, accounts and worlds are never part of this source package.

## Current generation path (0.3.0-dev.2)

`preset.generated.rs`, `preset_adapter.rs`, `prepare.rs` and `list.rs` implement
the generation path. The rejected custom ChunkGenerator/LevelShape
hooks have been removed. `tools/prepare_vanilla_natural.py` generates the six
original density overrides in `presets/vanilla-natural`; `tools/native.py generate`
validates their SHA-256 and embeds that directory. The source and world copies
use identical LF bytes. Git's scoped EOL rule and the embedding tool reject
newline drift instead of publishing mismatched checksums.

The original vanilla offset is read from the owner's prepared resources at new
world creation and saved under a private alias in that world's data pack. It is
not included in the mod source or bundle. Climate, biome, decoration, cave, ore,
structure and dimension definitions are not replaced. See
[the generation design](vanilla-natural.md) for the scope and remaining limits.

The cancellable HEAD on `WorldRequest::prepare_world` installs that data pack
before the normal engine loads it. Dedicated servers call the same method, so
their new normal worlds receive the same generation recipe. Existing vanilla
worlds are skipped; descriptor4 and descriptor5 worlds retain their own saved recipe. Older
prototype descriptors are refused rather than silently changing their terrain.

External presets under `<game>/horizon-mods/immersive-landscapes-preset` are
copied only into new worlds. They must contain `pack.mcmeta` and `IL-PRESET.json`
with schema1, descriptor_version5, preset, author, based_on and optional
vanilla_aliases. The marker uses that actual identity. Symbolic links are
refused. Such presets are an explicit development override, not a claim
that arbitrary Java data packs are compatible.

## Named creation-screen type

The enabled mod owns the normal-world generation slot, whose creation-screen
name is now **Immersive Landscapes**. This is the existing candidate recipe under an
accurate display name. The persisted Horizon identity remains `minecraft:normal`,
with descriptor5 and the world-local data pack for new worlds. Old v4 worlds keep
their Epic recipe; this update does not migrate them. Flat, large-biome and other
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
The six numeric terrain functions and descriptor5 are unchanged by this port.
The earlier c9d474d build and its tool patch remain historical rollback material.
