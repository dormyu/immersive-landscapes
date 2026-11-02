# Native development build

The native adapter targets Horizon `c9d474d` and builds together with the separate
DH adaptation. Run `tools/native.py generate`, `stage`, `build` or `package`.
The current Windows workspace uses the sibling `RustCore/development` toolchain,
temp, cache and build guard. Source `Enter-RustDev.ps1`; set `RUSTUP_TOOLCHAIN=stable`.

The generator's boundary hooks require a small local change to horizon-mixins
`0d6b103`: HEAD preserves macros in an otherwise untouched function body.
Interior hooks and attribute macros remain restricted. The patch and its regression
test are in `tools/patches/horizon-mixins-boundary-head.patch`; upstream MIT notice is
in `third-party/horizon-mixins-LICENSE`. Tested local commit: `19140348`.

To reproduce that tool, clone `https://github.com/horizon-rust/horizon-mixins` onto E,
checkout `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`, and apply the bundled patch:

```powershell
. E:\Games\Minecraft\RustCore\development\scripts\Enter-RustDev.ps1
$env:RUSTUP_TOOLCHAIN = 'stable'
$env:CARGO_TARGET_DIR = 'E:\Games\Minecraft\RustCore\development\mixins-tool-target'
git apply E:\Games\Minecraft\immersive-landscapes\tools\patches\horizon-mixins-boundary-head.patch
cargo test --locked -p horizon-mixins-cli --test integration boundary::head_preserves_body_macros_without_expanding_them -- --test-threads=1
cargo build --release --locked -p horizon-mixins-cli
Copy-Item "$env:CARGO_TARGET_DIR\release\horizon-mixins.exe" E:\Games\Minecraft\RustCore\development\mixins-tool\bin\horizon-mixins.exe
```

Run the commands from that fresh tool checkout. Our build wrapper selects this
binary through `HORIZON_MIXINS_BIN`. The DH build script supports that explicit
override; its original pinned tool remains available for DH-only builds.

This development `.hnmod` contains source, not a dynamically loaded game DLL.
The unpatched Horizon launcher cannot build these worldgen hooks yet. Use the
documented patched tool / combined build or the tested prebuilt DormLab executable.
Game resources, accounts and worlds are never part of this source package.

## Current generation path (0.2.0-dev.2)

`preset.generated.rs`, `preset_adapter.rs`, `prepare.rs` and `list.rs` implement
the generation path. The rejected custom ChunkGenerator/LevelShape
hooks have been removed. `tools/native.py generate` embeds the complete
`presets/epic-compact` directory, including its upstream license and provenance.
Registry checksums in `IL-PRESET.json` cover the vendored source-file bytes.
Git preserves those bytes; embedding text normalizes line endings to LF before
the native loader writes the world's copy, without changing registry values.

The cancellable HEAD on `WorldRequest::prepare_world` installs that data pack
before the normal engine loads it. Dedicated servers call the same method, so
their new normal worlds receive the same generation recipe. Existing vanilla
worlds are skipped; descriptor4 worlds retain their own saved recipe. Older
prototype descriptors are refused rather than silently changing their terrain.

External presets under `<game>/horizon-mods/immersive-landscapes-preset` are
copied only into new worlds. They must contain `pack.mcmeta`; symbolic links
are refused. Such presets are an explicit development override, not a claim
that arbitrary Java data packs are compatible.

## Named creation-screen type

The enabled mod owns the normal-world generation slot, whose creation-screen
name is now **Immersive Landscapes**. This is the accepted recipe under an
accurate display name; it is not a second generator or a save-format migration.
The persisted Horizon identity remains `minecraft:normal`, descriptor4 and the
world-local data pack. Flat, large-biome and other slots keep their own behavior.

`world_type_ui.rs` keeps a presentation flag, synchronized at native-mod discovery
and whenever mod settings are saved. `world_type_name.rs` updates only the
creation menu's normal-type translation, including next-choice narration.
`world_type_button.rs` fits the full name in the existing cycle button by omitting
the redundant localized prefix for this one value. Disabling IL restores Default
for new worlds; existing IL worlds keep their recipe. No per-frame callback,
generation work or added filesystem reads are involved in displaying the label.
