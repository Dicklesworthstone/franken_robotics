#!/usr/bin/env bash
# Regression tests for real defects found during multi-pass audit of
# scripts/admission/qty_coupling.sh:
# 1. --working-tree on a clean tracked tree with an untracked file must report
#    dirty=false, tree_hash=null, status="ok" (tracked-only dirty per N33a-04).
# 2. --working-tree with --rev != HEAD must fail closed (exit 2) rather than
#    measuring HEAD's working tree while stamping the receipt with an older rev.
# 3. INV-SUITE-04 read-only sibling guarantee: running against a chmod a-w repo
#    uses `git --no-optional-locks` and succeeds with unchanged HEAD/status.

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

export FROB_SCRATCH_ACTOR="${FROB_SCRATCH_ACTOR:-BrownHeron}"
SCRATCH="$(coupling_make_scratch "test-readonly-and-working-tree")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"
REV1="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '2p')"

# Advance repo to a second commit REV2 that adds a direct edge z -> fs-qty.
cat > "$REPO/crates/z/Cargo.toml" <<'EOF'
[package]
name = "z"
version = "0.0.1"
edition = "2021"

[dependencies]
fs-qty = { path = "../fs-qty" }
EOF
REV2="$(coupling_refresh_lock_and_commit "$REPO" "$SCRATCH" "second commit adding z -> fs-qty")"

# Case 1: Add ONLY an untracked file to the clean working tree at REV2.
printf 'untracked scratch\n' > "$REPO/untracked_note.tmp"
WORK1="$SCRATCH/work-untracked"
mkdir -p "$WORK1"
OUT1="$(cd "$WORK1" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$REV2" --working-tree)"
REC1="$WORK1/$(coupling_extract_kv "$OUT1" "record_path")"

python3 - "$REC1" <<'PY'
import json, sys
from pathlib import Path
d = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert d["dirty"] is False, f"untracked file wrongly marked dirty=True: {d}"
assert d["tree_hash"] is None, f"untracked-only tree must have tree_hash=None: {d}"
assert d["status"] == "ok", f"expected status='ok' on tracked-clean tree, got {d['status']!r}"
PY

# Case 2: --working-tree with --rev pointing at older REV1 != HEAD (REV2) must fail closed (exit 2).
WORK2="$SCRATCH/work-mismatch"
mkdir -p "$WORK2"
set +e
OUT2="$(cd "$WORK2" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$REV1" --working-tree 2>&1)"
RC2=$?
set -e
if [ "$RC2" -ne 2 ]; then
  printf 'FAIL: expected exit 2 when --working-tree rev (%s) != HEAD (%s), got exit %d:\n%s\n' "$REV1" "$REV2" "$RC2" "$OUT2" >&2
  exit 1
fi
if [ -d "$WORK2/receipts/coupling" ] && [ -n "$(ls -A "$WORK2/receipts/coupling" 2>/dev/null)" ]; then
  printf 'FAIL: wrote a receipt despite --working-tree rev != HEAD\n' >&2
  exit 1
fi

# Case 3: Read-only sibling repo (chmod -R a-w) succeeds using --no-optional-locks.
# Restore u+w on exit so scratch remains inspectable (never deleting anything per COMMON.md 9b).
trap 'chmod -R u+w "$REPO" 2>/dev/null || true' EXIT
chmod -R a-w "$REPO"

WORK3="$SCRATCH/work-readonly"
mkdir -p "$WORK3"
OUT3="$(cd "$WORK3" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$REV1")"
REC3="$WORK3/$(coupling_extract_kv "$OUT3" "record_path")"

python3 - "$REC3" "$QTY_COUPLING_SCRIPT" <<'PY'
import json, sys
from pathlib import Path
d = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert d["status"] == "ok", d
script_src = Path(sys.argv[2]).read_text(encoding="utf-8")
assert "--no-optional-locks" in script_src, "qty_coupling.sh must pass --no-optional-locks to git"
PY

echo "PASS: coupling_readonly_and_working_tree"
