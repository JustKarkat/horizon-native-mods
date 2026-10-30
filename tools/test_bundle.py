#!/usr/bin/env python3
"""Check the bundle's shape without Horizon: manifest, pins and packager agree."""
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def main():
    compat = json.loads((ROOT / "compatibility.json").read_text())
    manifests = sorted((ROOT / "mixins").glob("*/horizon-mixin.toml"))
    assert len(manifests) == 1, "a .hnmod holds one bundle"
    mixin = tomllib.loads(manifests[0].read_text())
    mod = mixin["mod"]
    assert manifests[0].parent.name == mod["id"], "the bundle folder is named after the id"
    assert mod["id"].replace("_", "").isalnum() and mod["id"].islower(), "registry id rule"
    assert mixin["compatibility"]["horizon_revision"] == compat["horizon_revision"], "same pin"
    # Every file a module or hook names exists in the bundle.
    bundle = manifests[0].parent
    sources = {module["source"] for module in mixin.get("module", [])}
    sources |= {injection["helper"] for injection in mixin.get("injection", [])}
    for source in sorted(sources):
        assert (bundle / source).is_file(), f"missing {source}"
    # Every injected-into file is hash-pinned, so a moved target fails here.
    files = {module["file"] for module in mixin.get("module", [])}
    pinned = set(compat["source_sha256"])
    assert files <= pinned, f"not hash-pinned: {sorted(files - pinned)}"
    # The packager produces the same bytes twice.
    first = subprocess.run(["python3", "tools/package.py"], cwd=ROOT, capture_output=True,
                           text=True)
    assert not first.returncode, first.stdout + first.stderr
    one = json.loads(first.stdout)
    second = subprocess.run(["python3", "tools/package.py"], cwd=ROOT, capture_output=True,
                            text=True)
    assert not second.returncode, second.stdout + second.stderr
    assert json.loads(second.stdout)["sha256"] == one["sha256"], "packaging is deterministic"
    print(json.dumps({"status": "ok", "mod": mod["id"], "modules": len(mixin.get("module", [])),
                      "injections": len(mixin.get("injection", [])),
                      "sha256": one["sha256"]}, indent=2))


if __name__ == "__main__":
    main()
