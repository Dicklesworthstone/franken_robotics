#!/usr/bin/env bash
# scripts/bootstrap/tests/drift_class_table.sh
# Tests BD-001 unit test `drift_class_table`:
# - Three synthetic sibling fixtures:
#   1. Clean checkout at its lock -> `on-pin` (`reason: null`, `agrees: true`)
#   2. Clean fast-forward of its lock -> `stale-lock` (`reason: null`, `agrees: false`)
#   3. Dirty tree -> `drift` (`reason: "dirty"`, `agrees: false`)
# - Also tests a sibling carrying `constellation.lock` (FrankenSim format) against a neighbor:
#   - neighbor fast-forwarded past constellation.lock -> `stale-lock`, `file: "constellation.lock"`, `agrees: false`
#   - neighbor at constellation.lock pin -> `on-pin`, `file: "constellation.lock"`, `agrees: true`

set -euo pipefail
TEST_NAME="drift_class_table"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
WS="${TMP}/repo"

# Fixture 1: clean checkout at its lock (on-pin)
SIB_1="${TMP}/sib_onpin"
REV_1="$(create_synthetic_sibling "${SIB_1}" "sib-onpin")"

# Fixture 2: clean fast-forward of its lock (stale-lock)
SIB_2="${TMP}/sib_ff"
REV_2_PIN="$(create_synthetic_sibling "${SIB_2}" "sib-ff")"
printf '// fast-forward commit\n' >> "${SIB_2}/src/lib.rs"
git_commit_all "${SIB_2}" "ff commit" >/dev/null

# Fixture 3: dirty tree (drift)
SIB_3="${TMP}/sib_dirty"
REV_3="$(create_synthetic_sibling "${SIB_3}" "sib-dirty")"
printf '// dirty modification\n' >> "${SIB_3}/src/lib.rs"

# Fixture 4: neighbor + constellation.lock sibling (FrankenSim own_lock shape)
NEIGHBOR="${TMP}/dep_neighbor"
REV_NB_OLD="$(create_synthetic_sibling "${NEIGHBOR}" "dep-neighbor")"
printf '// neighbor fast-forward\n' >> "${NEIGHBOR}/src/lib.rs"
REV_NB_NEW="$(git_commit_all "${NEIGHBOR}" "neighbor advance")"

SIB_CONST_STALE="${TMP}/sib_const_stale"
init_git_repo "${SIB_CONST_STALE}"
cat > "${SIB_CONST_STALE}/constellation.lock" <<EOF
{
  "schema": "frankensim-constellation-lock-v2",
  "libraries": [
    {"lib": "dep_neighbor", "version": "0.1.0", "git_head": "${REV_NB_OLD}", "path": "/ignored/abs/path"}
  ]
}
EOF
REV_CONST_STALE="$(git_commit_all "${SIB_CONST_STALE}" "add stale constellation.lock")"

SIB_CONST_ONPIN="${TMP}/sib_const_onpin"
init_git_repo "${SIB_CONST_ONPIN}"
cat > "${SIB_CONST_ONPIN}/constellation.lock" <<EOF
{
  "schema": "frankensim-constellation-lock-v2",
  "libraries": [
    {"lib": "dep_neighbor", "version": "0.1.0", "git_head": "${REV_NB_NEW}", "path": "/ignored/abs/path"}
  ]
}
EOF
REV_CONST_ONPIN="$(git_commit_all "${SIB_CONST_ONPIN}" "add on-pin constellation.lock")"

LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-A-ONPIN"
name = "sib-onpin"
posture = "consume-path"
path = "../sib_onpin"
git_rev = "${REV_1}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-1"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-B-STALE"
name = "sib-ff"
posture = "consume-path"
path = "../sib_ff"
git_rev = "${REV_2_PIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-2"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-C-DRIFT"
name = "sib-dirty"
posture = "consume-path"
path = "../sib_dirty"
git_rev = "${REV_3}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-3"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-D-CONST-STALE"
name = "sib-const-stale"
posture = "consume-path"
path = "../sib_const_stale"
git_rev = "${REV_CONST_STALE}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-4"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-E-CONST-ONPIN"
name = "sib-const-onpin"
posture = "consume-path"
path = "../sib_const_onpin"
git_rev = "${REV_CONST_ONPIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-5"
receipt = ""
fallback = "fb"
EOF
)"

create_synthetic_workspace "${WS}" "${LOCK_BODY}" >/dev/null
"${FIRST_RECEIPT}" --stdout --root "${WS}" > "${TMP}/receipt.json"

python3 - "${TMP}/receipt.json" "${SIB_CONST_STALE}/constellation.lock" "${B3SUM_BIN}" <<'PYEOF'
import json, subprocess, sys
receipt_path, const_path, b3bin = sys.argv[1], sys.argv[2], sys.argv[3]
expected_const_hash = subprocess.check_output([b3bin, "--no-names", const_path]).decode("ascii").strip()

r = json.load(open(receipt_path, "r", encoding="utf-8"))
by_fd = {s["fd"]: s for s in r["siblings"]}

assert by_fd["FD-A-ONPIN"]["own_lock"]["class"] == "on-pin"
assert by_fd["FD-A-ONPIN"]["own_lock"]["reason"] is None
assert by_fd["FD-A-ONPIN"]["own_lock"]["agrees"] is True

assert by_fd["FD-B-STALE"]["own_lock"]["class"] == "stale-lock"
assert by_fd["FD-B-STALE"]["own_lock"]["reason"] is None
assert by_fd["FD-B-STALE"]["own_lock"]["agrees"] is False

assert by_fd["FD-C-DRIFT"]["own_lock"]["class"] == "drift"
assert by_fd["FD-C-DRIFT"]["own_lock"]["reason"] == "dirty"
assert by_fd["FD-C-DRIFT"]["own_lock"]["agrees"] is False

cs = by_fd["FD-D-CONST-STALE"]["own_lock"]
assert cs["file"] == "constellation.lock"
assert cs["hash"] == expected_const_hash
assert cs["class"] == "stale-lock"
assert cs["reason"] is None
assert cs["agrees"] is False

co = by_fd["FD-E-CONST-ONPIN"]["own_lock"]
assert co["file"] == "constellation.lock"
assert isinstance(co["hash"], str) and len(co["hash"]) == 64
assert co["class"] == "on-pin"
assert co["reason"] is None
assert co["agrees"] is True
PYEOF

pass_test
