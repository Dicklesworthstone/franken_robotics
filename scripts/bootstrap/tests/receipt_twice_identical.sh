#!/usr/bin/env bash
# scripts/bootstrap/tests/receipt_twice_identical.sh
# Tests BD-001 unit test `receipt_twice_identical` (KA-767) plus negative controls:
# - Two runs on unchanged tree produce byte-identical JSON (`cmp`)
# - `--verify` passes on the written receipt (and after committing the receipt + lock on top of repo_rev)
# - Negative control 1: touching one tracked file in sibling flips dirty, changes tree_hash & fails verify
# - Negative control 2: editing one character in the receipt file fails verify
# - Read-only sibling guarantee: `chmod -R a-w` on sibling copy succeeds with unchanged status & HEAD

set -euo pipefail
TEST_NAME="receipt_twice_identical"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
SIB="${TMP}/sib_one"
WS="${TMP}/repo"

REV="$(create_synthetic_sibling "${SIB}" "sib-one")"

LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-ONE"
name = "sib-one"
posture = "consume-path"
path = "../sib_one"
git_rev = "${REV}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-ONE-01"
receipt = ""
fallback = "fb"
EOF
)"
create_synthetic_workspace "${WS}" "${LOCK_BODY}" >/dev/null

# Snapshot sibling state before.
BEFORE_HEAD="$(git --no-optional-locks -C "${SIB}" rev-parse HEAD)"
BEFORE_STATUS="$(git --no-optional-locks -C "${SIB}" status --porcelain)"

# Make sibling read-only to prove no write occurs inside sibling.
chmod -R a-w "${SIB}"

"${FIRST_RECEIPT}" --stdout --root "${WS}" > "${TMP}/r1.json"
"${FIRST_RECEIPT}" --stdout --root "${WS}" > "${TMP}/r2.json"

cmp -s "${TMP}/r1.json" "${TMP}/r2.json" || fail_test "two --stdout runs differed (KA-767)"

HASH1="$("${FIRST_RECEIPT}" --root "${WS}")"
HASH2="$("${FIRST_RECEIPT}" --root "${WS}")"
assert_eq "two write runs return identical hash" "${HASH1}" "${HASH2}"

RECEIPT_FILE="${WS}/receipts/${HASH1}.json"
cmp -s "${TMP}/r1.json" "${RECEIPT_FILE}" || fail_test "written receipt differs from --stdout bytes"

# Verify sibling is byte-identical after runs.
AFTER_HEAD="$(git --no-optional-locks -C "${SIB}" rev-parse HEAD)"
AFTER_STATUS="$(git --no-optional-locks -C "${SIB}" status --porcelain)"
assert_eq "sibling HEAD unchanged" "${AFTER_HEAD}" "${BEFORE_HEAD}"
assert_eq "sibling status unchanged" "${AFTER_STATUS}" "${BEFORE_STATUS}"

# Step 5 simulation: write receipt hash into lock, commit receipt + lock on top of repo_rev,
# and verify that --verify passes (repo_rev is an ancestor of HEAD and all other fields match).
python3 - "${WS}/franken-stack.lock" "${HASH1}" <<'PYEOF'
import sys
p, h = sys.argv[1], sys.argv[2]
txt = open(p, "r", encoding="utf-8").read().replace('receipt = ""', f'receipt = "{h}"')
open(p, "w", encoding="utf-8").write(txt)
PYEOF
git_commit_all "${WS}" "commit receipt and lock" >/dev/null

"${FIRST_RECEIPT}" --verify "${RECEIPT_FILE}" --root "${WS}" >/dev/null || fail_test "--verify failed on valid committed receipt"

# Negative control 2: edit one character in a copy of the receipt -> --verify MUST fail.
TAMPER_DIR="${TMP}/tampered"
mkdir -p "${TAMPER_DIR}"
TAMPER_FILE="${TAMPER_DIR}/${HASH1}.json"
python3 - "${RECEIPT_FILE}" "${TAMPER_FILE}" <<'PYEOF'
import sys
b = bytearray(open(sys.argv[1], "rb").read())
idx = b.index(b"nightly-2026-08-31")
b[idx] = ord("N")
open(sys.argv[2], "wb").write(bytes(b))
PYEOF
if "${FIRST_RECEIPT}" --verify "${TAMPER_FILE}" --root "${WS}" >/dev/null 2>&1; then
  fail_test "edited receipt control was NOT caught by --verify"
fi

# Negative control 1: touch one tracked file in sibling -> dirty flips, tree_hash changes, --verify MUST fail.
chmod -R u+w "${SIB}"
printf '// dirty touch\n' >> "${SIB}/src/lib.rs"
"${FIRST_RECEIPT}" --stdout --root "${WS}" > "${TMP}/dirty.json"
python3 - "${TMP}/dirty.json" "${HASH1}" "${B3SUM_BIN}" <<'PYEOF'
import json, subprocess, sys
dirty_path, orig_hash, b3bin = sys.argv[1], sys.argv[2], sys.argv[3]
raw = open(dirty_path, "rb").read()
new_hash = subprocess.check_output([b3bin, "--no-names"], input=raw).decode("ascii").strip()
assert new_hash != orig_hash, "receipt hash must change when sibling tracked file is touched"
obj = json.loads(raw.decode("utf-8"))
s = obj["siblings"][0]
assert s["dirty"] is True, "sibling dirty must flip to true"
assert isinstance(s["tree_hash"], str) and len(s["tree_hash"]) == 64, "tree_hash must be 64-hex"
assert s["own_lock"]["class"] == "drift" and s["own_lock"]["reason"] == "dirty"
PYEOF

if "${FIRST_RECEIPT}" --verify "${RECEIPT_FILE}" --root "${WS}" >/dev/null 2>&1; then
  fail_test "dirty sibling control was NOT caught by --verify"
fi

pass_test
