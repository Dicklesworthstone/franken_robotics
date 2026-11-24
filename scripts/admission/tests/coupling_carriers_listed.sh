#!/usr/bin/env bash
# Unit test: carriers_listed
# - i's carrier is fs-evidence; m's carriers are fs-evidence and i.
# - Adding a non-carrying direct dependency m -> z does NOT add z as a carrier.

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-carriers-listed")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"
BASE_SHA="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '2p')"

WORK="$SCRATCH/work"
mkdir -p "$WORK"

OUT1="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$BASE_SHA")"
REC1="$WORK/$(coupling_extract_kv "$OUT1" "record_path")"

# Add non-carrying direct dependency m -> z to verify z is excluded from m's carriers.
cat > "$REPO/crates/m/Cargo.toml" <<'EOF'
[package]
name = "m"
version = "0.0.1"
edition = "2021"

[dependencies]
i = { path = "../i" }
fs-evidence = { path = "../fs-evidence" }
z = { path = "../z" }
EOF

SHA2="$(coupling_refresh_lock_and_commit "$REPO" "$SCRATCH" "add non-carrying edge m -> z")"
OUT2="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA2")"
REC2="$WORK/$(coupling_extract_kv "$OUT2" "record_path")"

python3 - "$REC1" "$REC2" <<'PY'
import json, sys
from pathlib import Path

for label, rec_file in (("base", sys.argv[1]), ("with_non_carrier", sys.argv[2])):
    data = json.loads(Path(rec_file).read_text(encoding="utf-8"))
    for m in data["modes"]:
        p3 = next(p for p in m["populations"] if p["name"] == "P3")
        indirect = {row["crate"]: row["carriers"] for row in p3["indirect"]}
        assert set(indirect.keys()) == {"i", "m"}, f"{label}/{m['feature_mode']}: {indirect}"
        assert indirect["i"] == ["fs-evidence"], f"{label}/{m['feature_mode']}: {indirect}"
        assert indirect["m"] == ["fs-evidence", "i"], f"{label}/{m['feature_mode']}: {indirect}"
        for crate, carriers in indirect.items():
            assert len(carriers) >= 1, f"{crate} has empty carriers"
            assert "z" not in carriers
PY

echo "PASS: coupling_carriers_listed"
