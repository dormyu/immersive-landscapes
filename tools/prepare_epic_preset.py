#!/usr/bin/env python3
"""Derive the Minecraft-scale IL preset from the verified native Epic fixture."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--reference", required=True, type=Path)
parser.add_argument("--output", required=True, type=Path)
parser.add_argument("--climate-frequency", type=float, default=0.8)
args = parser.parse_args()
source, out = args.reference.resolve(), args.output.resolve()
if out.exists():
    raise SystemExit("Refusing to overwrite an existing preset")
if not 0.2 <= args.climate_frequency <= 1.6:
    raise SystemExit("Climate frequency must be in 0.2..1.6")
provenance = json.loads((source / "PROVENANCE.json").read_text(encoding="utf-8"))
if provenance["revision"] != "08507bd3d606ad440b40a5fa726e0098d7309347":
    raise SystemExit("Unexpected Epic source revision")
shutil.copytree(source, out)
changes = []
for name in ("temperature1", "vegetation"):
    path = out / f"data/etn/worldgen/density_function/excessive_biomes/{name}.json"
    value = json.loads(path.read_text(encoding="utf-8"))
    count = [0]
    def tune(node):
        if isinstance(node, dict):
            if node.get("type") == "minecraft:noise" and node.get("noise") in {"minecraft:temperature", "minecraft:vegetation"}:
                if node.get("xz_scale") != 0.2:
                    raise ValueError("Unexpected Epic climate scale")
                node["xz_scale"] = args.climate_frequency
                count[0] += 1
            for child in node.values():
                tune(child)
        elif isinstance(node, list):
            for child in node:
                tune(child)
    tune(value)
    if count[0] != 2:
        raise ValueError("Expected two climate layers")
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    changes.append({"file": path.relative_to(out).as_posix(), "old_frequency": 0.2,
                    "new_frequency": args.climate_frequency})
meta = json.loads((out / "pack.mcmeta").read_text(encoding="utf-8"))
meta["pack"]["description"] = "Immersive Landscapes by dormyu; terrain based on Epic Terrain by wonderfulaichen (MIT)"
(out / "pack.mcmeta").write_text(json.dumps(meta, indent=2) + "\n", encoding="utf-8")
tuning = {"preset": "immersive-landscapes-epic-compact-v1", "author": "dormyu",
          "based_on": provenance, "changes": changes,
          "terrain_horizontal_scale": 1.0, "terrain_vertical_scale": 1.0,
          "native_pipeline": "Horizon full terrain, caves, ores, biomes, decoration and structures",
          "note": "Climate wavelength is four times shorter by default; terrain and rivers retain Epic's original Minecraft scale."}
tuning["registry_sha256"] = {p.relative_to(out).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                             for p in sorted((out / "data").rglob("*.json"))}
(out / "IL-PRESET.json").write_text(json.dumps(tuning, indent=2) + "\n", encoding="utf-8")
print(json.dumps({"output": str(out), "registry_files": len(tuning["registry_sha256"]),
                  "climate_wavelength_ratio": 0.2 / args.climate_frequency}))
