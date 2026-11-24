#!/usr/bin/env bash
# Unit test: revision_source
# - `--rev <sha>` -> rev_source: "argument", frankensim_rev == <sha>
# - no `--rev` + scratch stack lock with FD-FSIM row naming <sha> ->
#   rev_source: "stack-lock", same counts (and resolves repo path from lock row)
# - no `--rev` and no lock -> exit 3, no file created under receipts/coupling/
# - no `--rev` and lock with non-40-hex git_rev -> exit 3, no file created
# - invalid `--rev` (not 40 hex) -> exit 3, no file created

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-revision-source")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"
SHA="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '2p')"

# 1. --rev argument path
WORK_ARG="$SCRATCH/work-arg"
mkdir -p "$WORK_ARG"
OUT_ARG="$(cd "$WORK_ARG" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA")"
REC_ARG="$WORK_ARG/$(coupling_extract_kv "$OUT_ARG" "record_path")"

# 2. stack-lock path (both via explicit --stack-lock and reading path from FD-FSIM row)
WORK_LOCK="$SCRATCH/work-lock"
mkdir -p "$WORK_LOCK"
LOCK_FILE="$SCRATCH/franken-stack.lock"
cat > "$LOCK_FILE" <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "$REPO"
git_rev = "$SHA"
EOF

OUT_LOCK="$(cd "$WORK_LOCK" && "$QTY_COUPLING_SCRIPT" --stack-lock "$LOCK_FILE")"
REC_LOCK="$WORK_LOCK/$(coupling_extract_kv "$OUT_LOCK" "record_path")"

python3 - "$REC_ARG" "$REC_LOCK" "$SHA" <<'PY'
import json, sys
from pathlib import Path

d_arg = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
d_lock = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
sha = sys.argv[3]

assert d_arg["frankensim_rev"] == sha
assert d_arg["rev_source"] == "argument"

assert d_lock["frankensim_rev"] == sha
assert d_lock["rev_source"] == "stack-lock"

assert d_arg["modes"] == d_lock["modes"], "counts differed between argument and stack-lock"
assert d_arg["reproduced_by"] == d_lock["reproduced_by"]
PY

# 3. No --rev and no lock -> exit 3 and nothing written under receipts/coupling/
WORK_NONE="$SCRATCH/work-none"
mkdir -p "$WORK_NONE"
set +e
(cd "$WORK_NONE" && "$QTY_COUPLING_SCRIPT" --repo "$REPO") >"$SCRATCH/none.out" 2>"$SCRATCH/none.err"
RC_NONE=$?
set -e
test "$RC_NONE" -eq 3
test ! -e "$WORK_NONE/receipts/coupling"

# 4. No --rev and lock row with non-40-hex git_rev -> exit 3 and nothing written
WORK_BADLOCK="$SCRATCH/work-badlock"
mkdir -p "$WORK_BADLOCK"
BAD_LOCK="$SCRATCH/bad-stack.lock"
cat > "$BAD_LOCK" <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "$REPO"
git_rev = "not-a-40-hex-sha"
EOF

set +e
(cd "$WORK_BADLOCK" && "$QTY_COUPLING_SCRIPT" --stack-lock "$BAD_LOCK") >"$SCRATCH/badlock.out" 2>"$SCRATCH/badlock.err"
RC_BADLOCK=$?
set -e
test "$RC_BADLOCK" -eq 3
test ! -e "$WORK_BADLOCK/receipts/coupling"

# 5. Invalid --rev -> exit 3 and nothing written
WORK_BADREV="$SCRATCH/work-badrev"
mkdir -p "$WORK_BADREV"
set +e
(cd "$WORK_BADREV" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "deadbeef") >"$SCRATCH/badrev.out" 2>"$SCRATCH/badrev.err"
RC_BADREV=$?
set -e
test "$RC_BADREV" -eq 3
test ! -e "$WORK_BADREV/receipts/coupling"

echo "PASS: coupling_revision_source"
