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
