"""Build a local Epic Terrain reference pack for pinned Horizon 26.3 QA.

This is a comparison fixture, not the Immersive Landscapes release generator.
No downloads, game resources, account access or world creation are performed.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

SOURCE_URL = "https://github.com/wonderfulaichen/Epic-Terrain"
SOURCE_REVISION = "08507bd3d606ad440b40a5fa726e0098d7309347"
SOURCE_VERSION = "versions/1.20.5-1.21.4"
HORIZON_REVISION = "c9d474dd9116e84c243f8992d75c11314df56f06"


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def adapt(pack):
    changes = []
    for path in sorted(pack.glob("data/*/worldgen/feature/*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        if data.get("type") == "minecraft:lake" and "can_place_feature" not in data:
            # The native lake implementation retains the lower solid/fluid
            # and upper liquid checks. Make the legacy protection policy
            # explicit in 26.3's additional predicate fields.
            protected = {"type": "minecraft:not", "predicate": {
                "type": "minecraft:matching_block_tag", "tag": "minecraft:features_cannot_replace"}}
            data["can_place_feature"] = {"type": "minecraft:true"}
            data["can_replace_with_air_or_fluid"] = protected
            data["can_replace_with_barrier"] = protected
            write_json(path, data)
            changes.append({"file": path.relative_to(pack).as_posix(), "change": "explicit lake placement/protection predicates; historical pond parity is unverified"})

    def int_providers(value):
        count = 0
        if isinstance(value, list):
            return sum(int_providers(x) for x in value)
        if not isinstance(value, dict):
            return 0
        if value.get("type") == "minecraft:biased_to_bottom" and isinstance(value.get("value"), dict):
            inner = value.pop("value")
            value.update(inner)
            count += 1
        return count + sum(int_providers(x) for x in value.values())

    for path in sorted(pack.glob("data/*/worldgen/placed_feature/*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        if int_providers(data):
            write_json(path, data)
            changes.append({"file": path.relative_to(pack).as_posix(), "change": "flatten legacy biased-to-bottom integer provider"})
    for path in sorted(pack.glob("data/*/worldgen/biome/*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        changed = False
        carvers = data.get("carvers")
        if isinstance(carvers, dict) and set(carvers) <= {"air", "liquid"}:
            data["carvers"] = [entry for group in carvers.values() for entry in group]
            changes.append({"file": path.relative_to(pack).as_posix(), "change": "flatten legacy carving-step holder lists"})
            changed = True
        for step in data.get("features", []):
            cane, pumpkin = "minecraft:patch_sugar_cane", "minecraft:patch_pumpkin"
            if cane in step and pumpkin in step and step.index(cane) < step.index(pumpkin):
                a, b = step.index(cane), step.index(pumpkin)
                step[a], step[b] = step[b], step[a]
                changes.append({"file": path.relative_to(pack).as_posix(), "change": "pumpkin before normal sugar cane, matching 26.3 decoration ordering"})
                changed = True
        if changed:
            write_json(path, data)

    for path in sorted(pack.glob("data/*/worldgen/noise_settings/*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        changed = False
        for key in ("default_block", "default_fluid"):
            value = data.get(key)
            # This pinned Horizon pipeline still reads NBT-style keys in
            # these two fields. Material-rule states retain 26.3's spelling.
            if isinstance(value, dict) and "id" in value:
                data[key] = {"Name": value["id"], "Properties": value.get("properties", {})}
                changes.append({"file": path.relative_to(pack).as_posix(), "change": f"{key}: pinned Horizon pipeline block-state spelling"})
                changed = True
        if changed:
            write_json(path, data)

    unused = pack / "data/etn/worldgen/noise/r/magma_caves.json"
    if unused.is_file():
        identifier = "etn:r/magma_caves"
        references = [p.relative_to(pack).as_posix() for p in pack.rglob("*.json")
                      if identifier in p.read_text(encoding="utf-8")]
        if references:
            raise ValueError(f"Negative-amplitude noise is referenced; needs semantic conversion: {references}")
        archive = pack / "compatibility-originals"
        archive.mkdir()
        unused.rename(archive / unused.name)
        changes.append({"file": unused.relative_to(pack).as_posix(), "change": "archive unreferenced negative-amplitude noise outside registries"})
    return changes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True, help="Pinned Epic-Terrain Git checkout")
    parser.add_argument("--converter", type=Path, required=True, help="Horizon horizon-port-pack executable")
    parser.add_argument("--output", type=Path, required=True, help="New, non-existent pack directory")
    args = parser.parse_args()
    source, converter, output = args.source.resolve(), args.converter.resolve(), args.output.resolve()
    if output.exists():
        parser.error("output already exists; use a new directory (no files will be overwritten)")
    revision = subprocess.run(["git", "-C", str(source), "rev-parse", "HEAD"],
                              check=True, capture_output=True, text=True).stdout.strip()
    if revision != SOURCE_REVISION:
        parser.error(f"source revision {revision} differs from tested pin {SOURCE_REVISION}")
    dirty = subprocess.run(["git", "-C", str(source), "status", "--porcelain"],
                           check=True, capture_output=True, text=True).stdout
    if dirty:
        parser.error("Epic source checkout is modified; use the untouched pinned reference")
    if not converter.is_file():
        parser.error("converter executable is missing")
    result = subprocess.run([str(converter), "upgrade-datapack", str(source / SOURCE_VERSION), str(output)],
                            capture_output=True, text=True, encoding="utf-8", check=True)
    (output / "CONVERSION.txt").write_text(result.stdout + result.stderr, encoding="utf-8")
    shutil.copyfile(source / "LICENSE", output / "LICENSE")
    changes = adapt(output)
    meta = json.loads((output / "pack.mcmeta").read_text(encoding="utf-8"))
    meta["pack"]["description"] = "Epic Terrain reference / wonderfulaichen; Horizon 26.3 QA adaptation by dormyu"
    write_json(output / "pack.mcmeta", meta)
    provenance = {
        "project": "Epic Terrain (reference fixture, not Immersive Landscapes)",
        "original_author": "wonderfulaichen",
        "license": "MIT",
        "source": SOURCE_URL,
        "revision": revision,
        "version_path": SOURCE_VERSION,
        "tested_horizon_revision": HORIZON_REVISION,
        "converter_sha256": hashlib.sha256(converter.read_bytes()).hexdigest(),
        "changes": changes,
        "known_limits": [
            "Lake protection predicates are explicit; exact historical pond-placement parity is unverified.",
            "Native far sampling differs from real water columns at tested river sites.",
            "Visual and performance acceptance, solo persistence and two-client QA are separate checks.",
        ],
    }
    write_json(output / "PROVENANCE.json", provenance)
    print(f"Prepared reference pack: {output}")
    print(f"Applied {len(changes)} narrow compatibility changes; preserve LICENSE and PROVENANCE.json.")


if __name__ == "__main__":
    main()
