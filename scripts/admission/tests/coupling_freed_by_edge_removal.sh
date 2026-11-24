#!/usr/bin/env bash
# Unit test: freed_by_edge_removal
# - Base fixture: deleting fs-evidence -> fs-qty frees {fs-evidence, i, m}.
# - With added direct edge m -> fs-qty: m stays coupled; freed = {fs-evidence, i}.

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-freed-edge-removal")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"
BASE_SHA="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '2p')"

WORK="$SCRATCH/work"
mkdir -p "$WORK"

OUT1="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$BASE_SHA")"
REC1="$WORK/$(coupling_extract_kv "$OUT1" "record_path")"

# Add a second direct edge m -> fs-qty and commit.
cat > "$REPO/crates/m/Cargo.toml" <<'EOF'
[package]
name = "m"
version = "0.0.1"
edition = "2021"

[dependencies]
i = { path = "../i" }
fs-evidence = { path = "../fs-evidence" }
fs-qty = { path = "../fs-qty" }
EOF

SHA2="$(coupling_refresh_lock_and_commit "$REPO" "$SCRATCH" "add direct edge m -> fs-qty")"

OUT2="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA2")"
REC2="$WORK/$(coupling_extract_kv "$OUT2" "record_path")"

python3 - "$REC1" "$REC2" <<'PY'
import json, sys
from pathlib import Path

r1 = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
r2 = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

# Ensure forbidden phrase never appears in either record.
for p in (sys.argv[1], sys.argv[2]):
    assert "freed in this project's build" not in Path(p).read_text(encoding="utf-8")

for m in r1["modes"]:
    p3 = next(p for p in m["populations"] if p["name"] == "P3")
    assert p3["freed_scope"] == "freed in FrankenSim's graph"
    assert p3["freed_by_fd_fsim_31"] == ["fs-evidence", "i", "m"], p3
    assert p3["still_coupled_after_31"] == [], p3
    assert p3["freed_count"] == 3

for m in r2["modes"]:
    p3 = next(p for p in m["populations"] if p["name"] == "P3")
    assert p3["direct"] == ["fs-evidence", "m"], p3
    assert [x["crate"] for x in p3["indirect"]] == ["i"], p3
    assert p3["coupled_count"] == 3
    assert p3["freed_by_fd_fsim_31"] == ["fs-evidence", "i"], p3
    assert p3["still_coupled_after_31"] == ["m"], p3
    assert p3["freed_count"] == 2
PY

echo "PASS: coupling_freed_by_edge_removal"
