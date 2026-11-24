#!/usr/bin/env bash
# Unit test: optional_edges_two_modes
# - An optional normal dependency z -> fs-qty (not enabled by default) appears
#   ONLY in feature mode `all`, never in feature mode `default`.

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-optional-two-modes")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"

cat > "$REPO/crates/z/Cargo.toml" <<'EOF'
[package]
name = "z"
version = "0.0.1"
edition = "2021"

[dependencies]
fs-qty = { path = "../fs-qty", optional = true }
EOF

SHA="$(coupling_refresh_lock_and_commit "$REPO" "$SCRATCH" "add optional normal edge z -> fs-qty")"

WORK="$SCRATCH/work"
mkdir -p "$WORK"

OUT="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA")"
REC="$WORK/$(coupling_extract_kv "$OUT" "record_path")"

python3 - "$REC" <<'PY'
import json, sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
modes = {m["feature_mode"]: m for m in data["modes"]}
assert set(modes.keys()) == {"default", "all"}

p3_def = next(p for p in modes["default"]["populations"] if p["name"] == "P3")
assert p3_def["direct"] == ["fs-evidence"], p3_def
assert [x["crate"] for x in p3_def["indirect"]] == ["i", "m"], p3_def
assert p3_def["coupled_count"] == 3
assert p3_def["freed_by_fd_fsim_31"] == ["fs-evidence", "i", "m"]
assert p3_def["still_coupled_after_31"] == []

p3_all = next(p for p in modes["all"]["populations"] if p["name"] == "P3")
assert p3_all["direct"] == ["fs-evidence", "z"], p3_all
assert [x["crate"] for x in p3_all["indirect"]] == ["i", "m"], p3_all
assert p3_all["coupled_count"] == 4
assert p3_all["freed_by_fd_fsim_31"] == ["fs-evidence", "i", "m"]
assert p3_all["still_coupled_after_31"] == ["z"]
PY

echo "PASS: coupling_optional_edges_two_modes"
