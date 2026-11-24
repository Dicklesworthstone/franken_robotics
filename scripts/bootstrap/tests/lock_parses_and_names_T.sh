#!/usr/bin/env bash
# scripts/bootstrap/tests/lock_parses_and_names_T.sh
# Tests BD-001 unit test `lock_parses_and_names_T`:
# - `franken-stack.lock` parses as valid TOML
# - Top-level `toolchain` equals `[toolchain].channel` in `rust-toolchain.toml`
# - Negative control: mismatched toolchain between lock and rust-toolchain.toml fails closed
# - Also checks the real repository's lock + rust-toolchain.toml when both are present

set -euo pipefail
TEST_NAME="lock_parses_and_names_T"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
SIB="${TMP}/frankensim"
WS="${TMP}/repo"

REV="$(create_synthetic_sibling "${SIB}" "frankensim")"

LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "../frankensim"
git_rev = "${REV}"
toolchain = "nightly-2026-07-06"
features = ["referee", "geometry", "ledger"]
build_env = { FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1" }
confined_to = ["frob-fsim", "bedrock-direct"]
admission_gate = "INT-FSIM-01"
receipt = ""
fallback = "KILL-FSIM"
EOF
)"
create_synthetic_workspace "${WS}" "${LOCK_BODY}" "nightly-2026-08-31" >/dev/null

# Verify synthetic lock parses and matches rust-toolchain.toml.
python3 - "${WS}/franken-stack.lock" "${WS}/rust-toolchain.toml" <<'PYEOF'
import sys, tomllib
lock = tomllib.load(open(sys.argv[1], "rb"))
tc = tomllib.load(open(sys.argv[2], "rb"))
assert lock["toolchain"] == tc["toolchain"]["channel"] == "nightly-2026-08-31"
assert len(lock["foundation"]) == 1
assert lock["foundation"][0]["id"] == "FD-FSIM"
PYEOF

RECEIPT_TC="$("${FIRST_RECEIPT}" --stdout --root "${WS}" | python3 -c 'import json,sys; print(json.load(sys.stdin)["toolchain"])')"
assert_eq "receipt toolchain equals T" "${RECEIPT_TC}" "nightly-2026-08-31"

# Negative control: mismatch lock toolchain vs rust-toolchain.toml -> must fail closed.
python3 - "${WS}/franken-stack.lock" <<'PYEOF'
import sys
p = sys.argv[1]
txt = open(p, "r", encoding="utf-8").read().replace('toolchain = "nightly-2026-08-31"', 'toolchain = "nightly-2026-07-06"', 1)
open(p, "w", encoding="utf-8").write(txt)
PYEOF

if "${FIRST_RECEIPT}" --stdout --root "${WS}" >/dev/null 2>&1; then
  fail_test "mismatched toolchain between franken-stack.lock and rust-toolchain.toml was NOT refused"
fi

# If the real repository already has franken-stack.lock and rust-toolchain.toml, validate them too (read-only).
if [ -f "${REPO_ROOT}/franken-stack.lock" ] && [ -f "${REPO_ROOT}/rust-toolchain.toml" ]; then
  python3 - "${REPO_ROOT}/franken-stack.lock" "${REPO_ROOT}/rust-toolchain.toml" <<'PYEOF'
import sys, tomllib
lock = tomllib.load(open(sys.argv[1], "rb"))
tc = tomllib.load(open(sys.argv[2], "rb"))
lock_t = lock.get("toolchain")
file_t = tc.get("toolchain", {}).get("channel")
assert lock_t == file_t, f"real repo toolchain mismatch: lock={lock_t!r} file={file_t!r}"
assert lock_t == "nightly-2026-08-31", f"expected provisional T nightly-2026-08-31, got {lock_t!r}"
assert len(lock.get("foundation", [])) >= 2, "expected at least FD-FSIM and FD-ASUP rows"
PYEOF
fi

pass_test
