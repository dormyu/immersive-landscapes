"""Read-only v8 preservation and new ecotone recipe checks; no engine build."""
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
BASE = "8a5460e949e1d4311407dd98ae0637ad20554a67"
PREFIX = "presets/vanilla-natural/"


def old(path):
    return subprocess.check_output(["git", "-C", str(ROOT), "show", BASE + ":" + path])


def main():
    before = json.loads(old(PREFIX + "IL-PRESET.json"))
    current = json.loads((ROOT / PREFIX / "IL-PRESET.json").read_text(encoding="utf-8"))
    assert before["descriptor_version"] == 8 and current["descriptor_version"] == 9
    assert current["preset"] == "vanilla-natural-v5"
    compatibility = json.loads((ROOT / "compatibility.json").read_text(encoding="utf-8"))
    assert compatibility["generator_version"] == current["descriptor_version"]
    for name, digest in before["registry_sha256"].items():
        assert hashlib.sha256((ROOT / PREFIX / name).read_bytes()).hexdigest() == digest, name
    actual = {p.relative_to(ROOT / PREFIX).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
              for p in (ROOT / PREFIX / "data").rglob("*.json")}
    assert actual == current["registry_sha256"]
    added = set(actual) - set(before["registry_sha256"])
    density = "data/immersive_landscapes/worldgen/density_function/"
    assert added == {density + "ecotones_v1.json", density + "ecotone_width.json"}
    assert json.loads((ROOT / PREFIX / (density + "ecotones_v1.json")).read_text()) == 1.0
    assert json.loads((ROOT / PREFIX / (density + "ecotone_width.json")).read_text()) == 32.0
    watershed = "mixins/immersive_landscapes/watershed.rs"
    assert (ROOT / watershed).read_bytes() == old(watershed)
    adapter = (ROOT / "mixins/immersive_landscapes/preset_adapter.rs").read_text()
    assert "4 | 5 | 6 | 7 | 8 | 9" in adapter
    assert adapter.index("return Ok(());") < adapter.index("let destination = root.join")
    manifest = tomllib.loads((ROOT / "mixins/immersive_landscapes/horizon-mixin.toml").read_text())
    assert manifest["mod"]["version"] == "0.5.0-dev.1"
    ids = [i["id"] for i in manifest["injection"]]
    assert len(ids) == len(set(ids))
    print(json.dumps({"passed": True, "base": BASE, "unchanged_v8_density_inputs": len(before["registry_sha256"]),
                      "new_descriptor": current["descriptor_version"], "watershed_unchanged": True,
                      "scope": "source and recipe only; native/game verification remains required"}))


if __name__ == "__main__":
    main()
