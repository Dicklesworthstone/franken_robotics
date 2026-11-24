#!/usr/bin/env bash
# scripts/bootstrap/tests/receipt_is_canonical.sh
# Tests BD-001 unit test `receipt_is_canonical`:
# - Object keys in code-point order at every level (`build_env` first, `toolchain` last at top level)
# - No whitespace outside strings (and no trailing newline)
# - `schema` equals `frob.stack-receipt/1`
# - `siblings` sorted in `fd` order even when lock rows are reversed
# - Parse and re-serialize with sorted keys gives identical bytes
# - File name equals BLAKE3 of bytes

set -euo pipefail
TEST_NAME="receipt_is_canonical"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
SIB_Z="${TMP}/sib_z"
SIB_A="${TMP}/sib_a"
WS="${TMP}/repo"

REV_Z="$(create_synthetic_sibling "${SIB_Z}" "sib-z" "0.1.0")"
REV_A="$(create_synthetic_sibling "${SIB_A}" "asupersync" "0.6.2")"

# Intentionally list FD-ZZZ before FD-AAA in franken-stack.lock to prove sorting by fd.
LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-ZZZ"
name = "sib-z"
posture = "consume-path"
path = "../sib_z"
git_rev = "${REV_Z}"
toolchain = "nightly-2026-08-31"
features = ["referee", "geometry", "ledger"]
admission_gate = "INT-Z-01"
receipt = ""
fallback = "synthetic fallback z"

[[foundation]]
id = "FD-AAA"
name = "asupersync"
posture = "consume"
path = "../sib_a"
git_rev = "${REV_A}"
version = "=0.6.2"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-A-01"
receipt = ""
fallback = "synthetic fallback a"
EOF
)"

PATCH_BLOCK='[patch.crates-io]
asupersync = { path = "../sib_a" }'

create_synthetic_workspace "${WS}" "${LOCK_BODY}" "nightly-2026-08-31" "${PATCH_BLOCK}" >/dev/null

HASH="$("${FIRST_RECEIPT}" --root "${WS}")"
RECEIPT_FILE="${WS}/receipts/${HASH}.json"

[ -f "${RECEIPT_FILE}" ] || fail_test "receipt file not written: ${RECEIPT_FILE}"

ACTUAL_B3="$("${B3SUM_BIN}" --no-names "${RECEIPT_FILE}")"
assert_eq "filename hash equals b3sum" "${HASH}" "${ACTUAL_B3}"

python3 - "${RECEIPT_FILE}" <<'PYEOF'
import json, sys, unicodedata

path = sys.argv[1]
raw = open(path, "rb").read()
assert not raw.endswith(b"\n"), "canonical JSON must have no trailing newline"
assert not raw.startswith(b"\xef\xbb\xbf"), "canonical JSON must have no BOM"
text = raw.decode("utf-8")

# Check no whitespace outside JSON strings.
in_str = False
esc = False
for i, ch in enumerate(text):
    if in_str:
        if esc:
            esc = False
        elif ch == "\\":
            esc = True
        elif ch == '"':
            in_str = False
    else:
        if ch == '"':
            in_str = True
        elif ch in (" ", "\t", "\r", "\n"):
            raise AssertionError(f"insignificant whitespace at byte {i}: {text[max(0,i-10):i+10]!r}")

# Parse preserving key order to verify code-point ordering at every object level.
def check_pairs(pairs):
    keys = [k for k, _ in pairs]
    assert keys == sorted(keys), f"keys not sorted by code point: {keys}"
    return dict(pairs)

obj = json.loads(text, object_pairs_hook=check_pairs)
top_keys = list(obj.keys())
assert top_keys[0] == "build_env", f"first top-level key must be build_env, got {top_keys[0]}"
assert top_keys[-1] == "toolchain", f"last top-level key must be toolchain, got {top_keys[-1]}"
assert obj["schema"] == "frob.stack-receipt/1", f"unexpected schema: {obj['schema']}"

fds = [s["fd"] for s in obj["siblings"]]
assert fds == ["FD-AAA", "FD-ZZZ"], f"siblings not sorted by fd: {fds}"
assert obj["profile_overrides"] == ["fsqlite*: opt-level 1"], f"unexpected profile_overrides: {obj['profile_overrides']}"
assert obj["patches"] == ["asupersync -> ../sib_a"], f"unexpected patches: {obj['patches']}"
assert obj["build_env"] == {"FRANKENSIM_DEPGRAPH_SALT": "franken-robotics-dev-v1"}

# Independent canonical re-serialization must match byte-for-byte.
def canon_str(s):
    s = unicodedata.normalize("NFC", s)
    out = ['"']
    for ch in s:
        cp = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif cp < 0x20:
            out.append(f"\\u{cp:04x}")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)

def canon(v):
    if v is None:
        return "null"
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, str):
        return canon_str(v)
    if isinstance(v, list):
        return "[" + ",".join(canon(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{" + ",".join(f"{canon_str(k)}:{canon(v[k])}" for k in sorted(v.keys())) + "}"
    raise AssertionError(type(v))

reserialized = canon(obj).encode("utf-8")
assert reserialized == raw, "re-serialized canonical JSON differs from receipt bytes"
PYEOF

pass_test
