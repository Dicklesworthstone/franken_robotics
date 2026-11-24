#!/usr/bin/env bash
# scripts/bootstrap/tests/receipt_has_no_absolute_path.sh
# Tests BD-001 unit test `receipt_has_no_absolute_path`:
# - No leading `/` path, `/Users/`, `/home/`, `/tmp/`, or `$HOME` in the generated receipt
# - Generating from two distinct checkout locations produces byte-identical receipts
# - Negative control: an absolute path in `[patch.crates-io]` is refused and writes no receipt

set -euo pipefail
TEST_NAME="receipt_has_no_absolute_path"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"

# Build two distinct directory roots holding identical commits & relative sibling layout.
LOC_A="${TMP}/location_alpha/nested"
LOC_B="${TMP}/location_beta"
mkdir -p "${LOC_A}" "${LOC_B}"

REV_A_SIB="$(create_synthetic_sibling "${LOC_A}/asupersync" "asupersync" "0.6.2")"
REV_B_SIB="$(create_synthetic_sibling "${LOC_B}/asupersync" "asupersync" "0.6.2")"
assert_eq "deterministic synthetic sibling commits match" "${REV_A_SIB}" "${REV_B_SIB}"

LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-ASUP"
name = "asupersync"
posture = "consume"
path = "../asupersync"
git_rev = "${REV_A_SIB}"
version = "=0.6.2"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-ASUP-01"
receipt = ""
fallback = "fb"
EOF
)"

PATCH_BLOCK='[patch.crates-io]
asupersync = { path = "../asupersync" }'

create_synthetic_workspace "${LOC_A}/repo" "${LOCK_BODY}" "nightly-2026-08-31" "${PATCH_BLOCK}" >/dev/null
create_synthetic_workspace "${LOC_B}/repo" "${LOCK_BODY}" "nightly-2026-08-31" "${PATCH_BLOCK}" >/dev/null

"${FIRST_RECEIPT}" --stdout --root "${LOC_A}/repo" > "${TMP}/loc_a.json"
"${FIRST_RECEIPT}" --stdout --root "${LOC_B}/repo" > "${TMP}/loc_b.json"

cmp -s "${TMP}/loc_a.json" "${TMP}/loc_b.json" || fail_test "receipts from two checkout locations differed"

# Verify zero absolute paths or home-directory leaks anywhere in JSON strings.
python3 - "${TMP}/loc_a.json" <<'PYEOF'
import json, os, sys
raw = open(sys.argv[1], "r", encoding="utf-8").read()
home = os.path.expanduser("~")
for bad in ("/Users/", "/home/", "/private/", "/tmp/"):
    assert bad not in raw, f"forbidden path fragment {bad!r} found in receipt"
if home and len(home) > 1:
    assert home not in raw, f"HOME ({home!r}) leaked into receipt"

obj = json.loads(raw)
def walk(v, p="$"):
    if isinstance(v, str):
        assert not v.startswith("/"), f"leading slash at {p}: {v!r}"
        assert not v.startswith("~"), f"tilde path at {p}: {v!r}"
    elif isinstance(v, list):
        for i, x in enumerate(v):
            walk(x, f"{p}[{i}]")
    elif isinstance(v, dict):
        for k, val in v.items():
            walk(k, f"{p}.<key>")
            walk(val, f"{p}.{k}")
walk(obj)
PYEOF

# Negative control: inject an absolute path in [patch.crates-io] -> script MUST fail closed and write no receipt.
LOC_BAD="${TMP}/location_bad"
mkdir -p "${LOC_BAD}"
create_synthetic_sibling "${LOC_BAD}/asupersync" "asupersync" "0.6.2" >/dev/null
BAD_PATCH="[patch.crates-io]
asupersync = { path = \"${LOC_BAD}/asupersync\" }"
create_synthetic_workspace "${LOC_BAD}/repo" "${LOCK_BODY}" "nightly-2026-08-31" "${BAD_PATCH}" >/dev/null

if "${FIRST_RECEIPT}" --root "${LOC_BAD}/repo" >/dev/null 2>&1; then
  fail_test "absolute patch path was NOT refused"
fi
if [ -d "${LOC_BAD}/repo/receipts" ] && [ -n "$(ls -A "${LOC_BAD}/repo/receipts" 2>/dev/null)" ]; then
  fail_test "receipt file was written despite absolute patch path refusal"
fi

pass_test
