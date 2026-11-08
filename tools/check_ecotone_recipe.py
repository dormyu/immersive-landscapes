"""Preserved v10 density inputs and explicit v11 opt-ins; no engine build."""
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
BASE = "197235209620422913bc84fca80cd594efccf99a"
PREFIX = "presets/vanilla-natural/"


def old(path):
    return subprocess.check_output(["git", "-C", str(ROOT), "show", BASE + ":" + path])


def main():
    before = json.loads(old(PREFIX + "IL-PRESET.json"))
    current = json.loads((ROOT / PREFIX / "IL-PRESET.json").read_text(encoding="utf-8"))
    assert before["descriptor_version"] == 10 and current["descriptor_version"] == 11
    assert current["preset"] == "vanilla-natural-v7"
    compatibility = json.loads((ROOT / "compatibility.json").read_text(encoding="utf-8"))
    assert compatibility["generator_version"] == current["descriptor_version"]
    density = "data/immersive_landscapes/worldgen/density_function/"
    tuning = {}
    for name, digest in before["registry_sha256"].items():
        value = (ROOT / PREFIX / name).read_bytes()
        if name in tuning:
            assert json.loads(value) == tuning[name], name
        else:
            assert hashlib.sha256(value).hexdigest() == digest, name
    actual = {p.relative_to(ROOT / PREFIX).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
              for p in (ROOT / PREFIX / "data").rglob("*.json")}
    assert actual == current["registry_sha256"]
    added = set(actual) - set(before["registry_sha256"])
    assert added == {density + "ecotones_v3.json", density + "rivers_v6.json"}
    for marker in ("ecotones_v2", "rivers_v5", "ecotones_v3", "rivers_v6"):
        assert json.loads((ROOT / PREFIX / (density + marker + ".json")).read_text()) == 1.0
    assert json.loads((ROOT / PREFIX / (density + "ecotone_width.json")).read_text()) == 32.0
    adapter = (ROOT / "mixins/immersive_landscapes/preset_adapter.rs").read_text()
    assert "4 | 5 | 6 | 7 | 8 | 9 | 10 | 11" in adapter
    assert adapter.index("return Ok(());") < adapter.index("let destination = root.join")
    manifest = tomllib.loads((ROOT / "mixins/immersive_landscapes/horizon-mixin.toml").read_text())
    assert manifest["mod"]["version"] == "0.6.0"
    ids = [i["id"] for i in manifest["injection"]]
    assert len(ids) == len(set(ids))
    print(json.dumps({"passed": True, "base": BASE, "unchanged_v10_density_inputs": len(before["registry_sha256"]) - len(tuning),
                      "new_world_shape_tuning": tuning,
                      "new_descriptor": current["descriptor_version"], "independent_opt_in_markers": True,
                      "scope": "source and recipe only; native/game verification remains required"}))


if __name__ == "__main__":
    main()
