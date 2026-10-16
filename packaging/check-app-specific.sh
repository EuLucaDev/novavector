#!/usr/bin/env bash
# App-specific package assertions retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("br.com.novafoods.novavector.yml", "br.com.novafoods.novavector.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "br.com.novafoods.novavector" and manifest["command"] == "novavector", name
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("NovaVector package identity ok")
PYEOF
