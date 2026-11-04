#!/usr/bin/env python3
"""Author the small vanilla-based terrain overlay; no copied game resources.

Shared ridge/erosion fields drive both vanilla terrain and biome selection.
The valley remap preserves the river field's zero crossings, widens eroded
lowlands and leaves mountain passes narrower. It is not a drainage simulation.
"""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "presets/vanilla-natural"


def noise(identifier, scale):
    return {"type": "minecraft:noise", "noise": identifier,
            "shift_x": "minecraft:shift_x", "shift_z": "minecraft:shift_z",
            "xz_scale": scale, "y_scale": 0.0}


def binary(kind, left, right):
    return {"type": "minecraft:" + kind, "left": left, "right": right}


def cache(value):
    return {"type": "minecraft:cache", "input": value}


def spline(coordinate, points):
    return {"type": "minecraft:spline", "spline": {
        "coordinate": coordinate,
        "points": [{"location": x, "value": y, "derivative": slope}
                   for x, y, slope in points]}}


def recipe():
    # Retain fine vanilla variation while giving the same mountain/river field
    # a larger coherent component. Continental and climatic scales are untouched.
    ridge = cache(binary("add", binary("mul", .72, noise("minecraft:ridge", .16)),
                                  binary("mul", .28, noise("minecraft:ridge", .25))))
    erosion = cache(binary("add", binary("mul", .7, noise("minecraft:erosion", .20)),
                                    binary("mul", .3, noise("minecraft:erosion", .25))))
    field = "immersive_landscapes:range_field"
    valley = spline(field, [(-1., -1., 1.), (-.35, -.35, 1.),
                           (-.18, -.14, 1.1), (-.08, -.04, .6),
                           (0., 0., .45), (.08, .04, .6),
                           (.18, .14, 1.1), (.35, .35, 1.), (1., 1., 1.)])
    erosion_strength = cache(spline("minecraft:overworld/erosion",
        [(-1., .2, 0.), (-.4, .25, 0.), (0., .55, 0.), (.5, 1., 0.), (1., 1., 0.)]))
    ridges = cache({"type": "minecraft:lerp",
                   "alpha": "immersive_landscapes:valley_strength",
                   "first": field, "second": "immersive_landscapes:valley_profile"})
    # Inland river beds should not become ocean-depth slots. Keep the original
    # coast/ocean and raise only the submerged floor near a river's centre.
    # The vanilla offset is snapshotted from the owner's resources at creation,
    # not vendored in our repository or compiled bundle.
    inland = spline("minecraft:overworld/continents",
                    [(-1., 0., 0.), (-.16, 0., 0.), (-.10, 1., 0.), (1., 1., 0.)])
    river = spline({"type": "minecraft:abs", "input": "minecraft:overworld/ridges"},
                   [(0., 1., 0.), (.045, 1., 0.), (.12, 0., 0.), (1., 0., 0.)])
    original_offset = "immersive_landscapes:vanilla_offset"
    offset = cache({"type": "minecraft:lerp", "alpha": binary("mul", inland, river),
                    "first": original_offset,
                    "second": binary("max", original_offset, -.54)})
    return {
        # Presence enables the Rust watershed adapter only for this saved
        # recipe. Its value is an ordinary unused vanilla constant.
        "data/immersive_landscapes/worldgen/density_function/watershed_v1.json": 1.0,
        "data/immersive_landscapes/worldgen/density_function/range_field.json": ridge,
        "data/immersive_landscapes/worldgen/density_function/valley_profile.json": valley,
        "data/immersive_landscapes/worldgen/density_function/valley_strength.json": erosion_strength,
        "data/minecraft/worldgen/density_function/overworld/ridges.json": ridges,
        "data/minecraft/worldgen/density_function/overworld/erosion.json": erosion,
        "data/minecraft/worldgen/density_function/overworld/offset.json": offset,
        # Retain three-dimensional rock detail without the full vanilla amplitude
        # that can detach a large surface mass above a narrow ridge. Caverns and
        # carvers remain in the final-density pipeline; this is not a height-only generator.
        "data/minecraft/worldgen/density_function/overworld/base_3d_noise.json":
            binary("mul", .35, "immersive_landscapes:vanilla_base_3d"),
    }


def main():
    files = recipe()
    for name, value in files.items():
        path = OUT / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8", newline="\n")
    (OUT / "pack.mcmeta").write_text(json.dumps({"pack": {
        "description": "Immersive Landscapes: vanilla terrain with coherent ranges and graded valleys, by dormyu",
        "pack_format": 121, "min_format": 121, "max_format": 121}}, indent=2) + "\n", encoding="utf-8", newline="\n")
    metadata = {
        "schema": 1, "preset": "vanilla-natural-v3", "author": "dormyu",
        "based_on": "Horizon f42ef42 native vanilla generator; original density overlay by dormyu",
        "generation_family": "vanilla-natural", "descriptor_version": 7,
        "vanilla_aliases": {"immersive_landscapes:vanilla_offset": "minecraft:overworld/offset",
                            "immersive_landscapes:vanilla_base_3d": "minecraft:overworld/base_3d_noise"},
        "preserves": ["vanilla continents and climate scale", "vanilla height limits and sea level",
                      "biomes and surface rules", "trees and vegetation", "caves and ores",
                      "structures", "Nether and End"],
        "changes": ["coherent low-frequency component in the shared mountain/river field",
                    "gentler erosion-field variation across mountain regions",
                    "wider valley profiles in eroded lowlands, narrower mountain passes",
                    "inland river-floor protection, fading out before ocean terrain",
                    "terrain and biome placement consume the same fields",
                    "base 3D rock-detail amplitude .35 to suppress detached surface masses",
                    "seed-bound coastal watershed graphs and connected graded channels in native Rust",
                    "shared riverbed and water profiles for actual terrain and distant sampling"],
        "known_limits": ["Coastal drainage coverage is bounded; not every pre-existing noise valley is converted",
                         "Delta branches and full inland catchments are not implemented",
                         "Vanilla biome identity and vegetation boundaries remain discrete",
                         "Native near/far surface sampling differences still require checking"],
        "registry_sha256": {name: hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in sorted(files)},
    }
    (OUT / "IL-PRESET.json").write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"preset": metadata["preset"], "registry_files":len(files), "output":str(OUT)}))


if __name__ == "__main__":
    main()
