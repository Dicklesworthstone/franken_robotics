#!/usr/bin/env bash
# scripts/bootstrap/tests/tree_hash_matches_definition.sh
# Tests BD-001 unit test `tree_hash_matches_definition`:
# - On a synthetic three-file git tree (one executable file, one plain file, one tracked file
#   deleted in the working tree), the script's `tree_hash` equals the BLAKE3 of the item-6 stream
#   assembled independently in the test
# - Flipping the executable bit alone changes `tree_hash` and matches the recomputed stream
# - Untracked files and mtime touches do NOT change `tree_hash`
# - Length prefix disambiguates ("ab","c") vs ("a","bc")

set -euo pipefail
TEST_NAME="tree_hash_matches_definition"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
SIB="${TMP}/synthetic_sib"
WS="${TMP}/repo"

init_git_repo "${SIB}"
printf '#!/bin/sh\necho hi\n' > "${SIB}/exec.sh"
chmod +x "${SIB}/exec.sh"
printf 'plain contents\n' > "${SIB}/plain.txt"
printf 'will be deleted\n' > "${SIB}/gone.txt"
REV="$(git_commit_all "${SIB}" "three tracked files")"

# Remove one tracked file from the working tree (by moving it outside the repo) so the repo is dirty.
mv "${SIB}/gone.txt" "${TMP}/moved_aside_gone.txt"

# Also create an untracked file that must be ignored by tree_hash.
printf 'ignored untracked\n' > "${SIB}/untracked.tmp"

# Build a workspace referencing this dirty sibling so we test BOTH --tree-hash and full receipt.
LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-SYN"
name = "synthetic-sib"
posture = "consume-path"
path = "../synthetic_sib"
git_rev = "${REV}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-SYN-01"
receipt = ""
fallback = "synthetic fallback"
EOF
)"
create_synthetic_workspace "${WS}" "${LOCK_BODY}" >/dev/null

# Independently assemble the exact item-6 stream in Python and hash via b3sum.
EXPECTED_HASH_1="$(python3 - "${SIB}" "${B3SUM_BIN}" <<'PYEOF'
import os, struct, subprocess, sys
sib, b3bin = sys.argv[1], sys.argv[2]
buf = bytearray(b"frob.tree/1")
# Sorted bytewise: exec.sh < gone.txt < plain.txt
for name, kind, data in [
    (b"exec.sh", "exec", open(os.path.join(sib, "exec.sh"), "rb").read()),
    (b"gone.txt", "deleted", None),
    (b"plain.txt", "plain", open(os.path.join(sib, "plain.txt"), "rb").read()),
]:
    buf += struct.pack("<Q", len(name)) + name
    if kind == "deleted":
        buf += b"\xff"
    elif kind == "exec":
        buf += b"\x01" + struct.pack("<Q", len(data)) + data
    else:
        buf += b"\x00" + struct.pack("<Q", len(data)) + data
out = subprocess.check_output([b3bin, "--no-names"], input=bytes(buf))
print(out.decode("ascii").strip())
PYEOF
)"

SCRIPT_TREE_HASH="$("${FIRST_RECEIPT}" --tree-hash "${SIB}")"
assert_eq "direct --tree-hash matches independent stream" "${SCRIPT_TREE_HASH}" "${EXPECTED_HASH_1}"

RECEIPT_JSON="$("${FIRST_RECEIPT}" --stdout --root "${WS}")"
RECEIPT_TREE_HASH="$(printf '%s' "${RECEIPT_JSON}" | python3 -c 'import json, sys; r=json.load(sys.stdin); s=r["siblings"][0]; assert s["dirty"] is True; print(s["tree_hash"])')"
assert_eq "receipt siblings[0].tree_hash matches independent stream" "${RECEIPT_TREE_HASH}" "${EXPECTED_HASH_1}"

# Touch mtime and add another untracked file -> hash must NOT change.
touch -t 202001010000 "${SIB}/plain.txt"
printf 'more untracked\n' > "${SIB}/another_untracked.txt"
HASH_AFTER_UNTRACKED="$("${FIRST_RECEIPT}" --tree-hash "${SIB}")"
assert_eq "untracked file and mtime change do not alter tree_hash" "${HASH_AFTER_UNTRACKED}" "${EXPECTED_HASH_1}"

# Flip executable bit alone on plain.txt -> hash MUST change and match recomputed stream.
chmod +x "${SIB}/plain.txt"
EXPECTED_HASH_2="$(python3 - "${SIB}" "${B3SUM_BIN}" <<'PYEOF'
import os, struct, subprocess, sys
sib, b3bin = sys.argv[1], sys.argv[2]
buf = bytearray(b"frob.tree/1")
for name, kind, data in [
    (b"exec.sh", "exec", open(os.path.join(sib, "exec.sh"), "rb").read()),
    (b"gone.txt", "deleted", None),
    (b"plain.txt", "exec", open(os.path.join(sib, "plain.txt"), "rb").read()),
]:
    buf += struct.pack("<Q", len(name)) + name
    if kind == "deleted":
        buf += b"\xff"
    else:
        buf += b"\x01" + struct.pack("<Q", len(data)) + data
out = subprocess.check_output([b3bin, "--no-names"], input=bytes(buf))
print(out.decode("ascii").strip())
PYEOF
)"

HASH_AFTER_CHMOD="$("${FIRST_RECEIPT}" --tree-hash "${SIB}")"
assert_ne "flipping executable bit changes tree_hash" "${HASH_AFTER_CHMOD}" "${EXPECTED_HASH_1}"
assert_eq "flipped executable bit matches independent stream" "${HASH_AFTER_CHMOD}" "${EXPECTED_HASH_2}"

# Length-prefix disambiguation: ("ab", "c") vs ("a", "bc") with concatenable contents.
SIB_AMB1="${TMP}/amb1"
SIB_AMB2="${TMP}/amb2"
init_git_repo "${SIB_AMB1}"
printf 'x' > "${SIB_AMB1}/ab"
printf 'yz' > "${SIB_AMB1}/c"
git_commit_all "${SIB_AMB1}" "amb1" >/dev/null

init_git_repo "${SIB_AMB2}"
printf 'xy' > "${SIB_AMB2}/a"
printf 'z' > "${SIB_AMB2}/bc"
git_commit_all "${SIB_AMB2}" "amb2" >/dev/null

H_AMB1="$("${FIRST_RECEIPT}" --tree-hash "${SIB_AMB1}")"
H_AMB2="$("${FIRST_RECEIPT}" --tree-hash "${SIB_AMB2}")"
assert_ne "length-prefixed paths and contents prevent ambiguity" "${H_AMB1}" "${H_AMB2}"

pass_test
