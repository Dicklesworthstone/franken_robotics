#!/usr/bin/env bash
# Negative controls for scripts/admission/qty_coupling.sh:
# 1. Dev-dependency control (from N33c-02 Test Plan):
#    Adding a [dev-dependencies] edge z -> fs-qty (and [build-dependencies]
#    edge z -> fs-qty) MUST NOT change the real script's counts in either mode,
#    while a mutated variant of the script that counts dev-dependencies MUST
#    change the fixture counts (proving the fixture catches the defect).
# 2. Fail-closed resolution control:
#    A workspace with an unresolvable sibling path dependency MUST exit non-zero,
#    write a status="resolution-failed" record with exact cargo stderr and no
#    invented counts, and leave the repo byte-identical.

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-negative-controls")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"

# Add dev-dependency and build-dependency edges z -> fs-qty.
cat > "$REPO/crates/z/Cargo.toml" <<'EOF'
[package]
name = "z"
version = "0.0.1"
edition = "2021"

[dependencies]

[dev-dependencies]
fs-qty = { path = "../fs-qty" }

[build-dependencies]
fs-qty = { path = "../fs-qty" }
EOF

SHA="$(coupling_refresh_lock_and_commit "$REPO" "$SCRATCH" "add dev and build dep z -> fs-qty")"

WORK_REAL="$SCRATCH/work-real"
mkdir -p "$WORK_REAL"
OUT_REAL="$(cd "$WORK_REAL" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA")"
REC_REAL="$WORK_REAL/$(coupling_extract_kv "$OUT_REAL" "record_path")"

# Build a planted-defect mutant copy of qty_coupling.sh that counts dev-dependencies.
MUTANT="$SCRATCH/qty_coupling_mutant_dev.sh"
python3 - "$QTY_COUPLING_SCRIPT" "$MUTANT" <<'PY'
import sys
from pathlib import Path

src = Path(sys.argv[1]).read_text(encoding="utf-8")
needle = 'is_normal = any(dk.get("kind") is None for dk in dep_kinds)'
replacement = 'is_normal = any(dk.get("kind") in (None, "dev") for dk in dep_kinds)'
assert src.count(needle) == 1, "expected exact normal-edge filter line in qty_coupling.sh"
dst = Path(sys.argv[2])
dst.write_text(src.replace(needle, replacement), encoding="utf-8")
dst.chmod(0o755)
PY

WORK_MUT="$SCRATCH/work-mutant"
mkdir -p "$WORK_MUT"
OUT_MUT="$(cd "$WORK_MUT" && "$MUTANT" --repo "$REPO" --rev "$SHA")"
REC_MUT="$WORK_MUT/$(coupling_extract_kv "$OUT_MUT" "record_path")"

python3 - "$REC_REAL" "$REC_MUT" <<'PY'
import json, sys
from pathlib import Path

real = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
mut = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

for m in real["modes"]:
    p3 = next(p for p in m["populations"] if p["name"] == "P3")
    assert p3["direct"] == ["fs-evidence"], f"real script leaked dev/build dep: {p3}"
    assert [x["crate"] for x in p3["indirect"]] == ["i", "m"], p3
    assert p3["coupled_count"] == 3

for m in mut["modes"]:
    p3 = next(p for p in m["populations"] if p["name"] == "P3")
    assert p3["direct"] == ["fs-evidence", "z"], f"mutant failed to catch dev edge: {p3}"
    assert p3["coupled_count"] == 4
PY

# Control 2: fail-closed when a sibling version/path cannot resolve under --locked --offline.
BROKEN_STACK="$SCRATCH/broken-stack"
mkdir -p "$BROKEN_STACK/fakescipy/crates/fsci-special/src"
cat > "$BROKEN_STACK/fakescipy/crates/fsci-special/Cargo.toml" <<'EOF'
[package]
name = "fsci-special"
version = "0.5.1"
edition = "2021"
EOF
echo "" > "$BROKEN_STACK/fakescipy/crates/fsci-special/src/lib.rs"

BROKEN_REPO="$BROKEN_STACK/frankensim"
mkdir -p "$BROKEN_REPO/crates/fs-qty/src"
cat > "$BROKEN_REPO/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = ["crates/*"]
EOF
cat > "$BROKEN_REPO/crates/fs-qty/Cargo.toml" <<'EOF'
[package]
name = "fs-qty"
version = "0.0.1"
edition = "2021"

[dependencies]
fsci-special = { path = "../../../fakescipy/crates/fsci-special", version = "=0.4.0" }
EOF
echo "" > "$BROKEN_REPO/crates/fs-qty/src/lib.rs"
echo 'version = 4' > "$BROKEN_REPO/Cargo.lock"

git -C "$BROKEN_REPO" init -q
git -C "$BROKEN_REPO" config user.name "CopperDove Test"
git -C "$BROKEN_REPO" config user.email "copperdove@test.local"
git -C "$BROKEN_REPO" add -A
git -C "$BROKEN_REPO" commit -q -m "unresolvable sibling version mismatch"
BROKEN_SHA="$(git -C "$BROKEN_REPO" rev-parse HEAD)"

BEFORE_HEAD="$(git -C "$BROKEN_REPO" rev-parse HEAD)"
BEFORE_STAT="$(git -C "$BROKEN_REPO" status --porcelain)"

WORK_BROKEN="$SCRATCH/work-broken"
mkdir -p "$WORK_BROKEN"
set +e
OUT_BROKEN="$(cd "$WORK_BROKEN" && "$QTY_COUPLING_SCRIPT" --repo "$BROKEN_REPO" --rev "$BROKEN_SHA" 2>"$SCRATCH/broken.err")"
RC_BROKEN=$?
set -e

test "$RC_BROKEN" -eq 1
test "$BEFORE_HEAD" = "$(git -C "$BROKEN_REPO" rev-parse HEAD)"
test "$BEFORE_STAT" = "$(git -C "$BROKEN_REPO" status --porcelain)"

REC_BROKEN="$WORK_BROKEN/$(coupling_extract_kv "$OUT_BROKEN" "record_path")"
HASH_BROKEN="$(coupling_extract_kv "$OUT_BROKEN" "record_blake3")"
test "$HASH_BROKEN" = "$(b3sum --no-names "$REC_BROKEN")"

python3 - "$REC_BROKEN" "$BROKEN_SHA" <<'PY'
import json, sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert data["schema"] == "frob.qty-coupling/1"
assert data["status"] == "resolution-failed"
assert data["frankensim_rev"] == sys.argv[2]
assert data["modes"] == [], "must not invent counts on resolution failure"
assert data["reproduced_by"] == "none"
assert len(data["cargo_errors"]) == 2
for err in data["cargo_errors"]:
    assert err["exit"] != 0
    assert "fsci-special" in err["stderr"]
    assert "0.4.0" in err["stderr"]
    assert "0.5.1" in err["stderr"]
PY

echo "PASS: coupling_negative_control_dev_deps"
