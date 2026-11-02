#!/usr/bin/env python3
"""Prepare our source bundle and build it with the already adapted DH on E:."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "mixins/immersive_landscapes"
VERSION = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]["version"]


def generate():
    preset = ROOT / "presets/epic-compact"
    parts = ["// Generated native preset. Epic Terrain: wonderfulaichen, MIT. Tuning/integration: dormyu.\n",
             "pub const FILES: &[(&str, &str)] = &[\n"]
    for path in sorted(preset.rglob("*")):
        if path.is_file():
            name = path.relative_to(preset).as_posix()
            value = path.read_text(encoding="utf-8")
            if '"#####' in value:
                raise ValueError("Unexpected preset raw string delimiter")
            parts.append(f'({json.dumps(name)}, r#####"{value}"#####),\n')
    parts.append("];\n")
    (BUNDLE / "preset.generated.rs").write_text("".join(parts), encoding="utf-8", newline="\n")


def package():
    out = ROOT / f"target/immersive-landscapes-{VERSION}.hnmod"
    out.parent.mkdir(parents=True, exist_ok=True)
    manifest = {
        "schema": 1, "id": "immersive_landscapes", "name": "Immersive Landscapes",
        "version": VERSION, "authors": ["dormyu"], "license": "MIT",
        "game": "26.3",
        "horizon": json.loads((ROOT / "compatibility.json").read_text())["horizon_revision"],
        "mixins": json.loads((ROOT / "compatibility.json").read_text())["horizon_mixins_revision"],
    }
    files = {"horizon-native.json": json.dumps(manifest, indent=2).encode()}
    for name in ("horizon-mixin.toml", "preset.generated.rs", "preset_adapter.rs", "prepare.rs", "list.rs"):
        files["mixins/immersive_landscapes/" + name] = (BUNDLE / name).read_bytes()
    for name in ("LICENSE", "CREDITS.md", "compatibility.json", "docs/native-build.md",
                 "tools/patches/horizon-mixins-boundary-head.patch", "third-party/horizon-mixins-LICENSE"):
        files[name] = (ROOT / name).read_bytes()
    for path in sorted((ROOT / "presets/epic-compact").rglob("*")):
        if path.is_file():
            files[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    with zipfile.ZipFile(out, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, value in sorted(files.items()):
            entry = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            entry.compress_type = zipfile.ZIP_DEFLATED
            entry.external_attr = 0o644 << 16
            archive.writestr(entry, value)
    print(f"{out} sha256={hashlib.sha256(out.read_bytes()).hexdigest()}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("generate", "stage", "check", "build", "package"))
    args = parser.parse_args()
    generate()
    if args.action == "generate":
        print(BUNDLE)
    elif args.action == "package":
        package()
    else:
        dev = ROOT.parent / "RustCore/development"
        os.environ["HORIZON_MIXINS_BIN"] = str(dev / "mixins-tool/bin/horizon-mixins.exe")
        if args.action == "build":
            command = [sys.executable, "-X", "utf8", str(dev / "scripts/Run-GuardedBuild.py"), "--with", str(BUNDLE)]
        else:
            command = [sys.executable, "-X", "utf8", str(dev / "distant-horizons/tools/horizon.py"), args.action, "--with", str(BUNDLE)]
        if os.environ.get("CARGO_BUILD_JOBS") != "1" or not Path(os.environ.get("TEMP", "/")).resolve().is_relative_to(dev):
            raise SystemExit("Source RustCore/development/scripts/Enter-RustDev.ps1 first (all temp/cache on E, jobs=1).")
        raise SystemExit(subprocess.call(command, cwd=dev))


if __name__ == "__main__":
    main()
