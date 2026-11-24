#!/usr/bin/env bash
# scripts/bootstrap/tests/drift_reason_recorded.sh
# Tests BD-001 unit test `drift_reason_recorded`:
# - On dirty fixture: `class` is `drift` and `reason` is `dirty`
# - On clean fixtures (`on-pin` and `stale-lock`): `reason` is `null`
# - On `retreated`, `diverged`, and `unknown-pin`: `class` is `drift` with the exact sub-reason
# - Precedence: dirty + retreated records `dirty`

set -euo pipefail
TEST_NAME="drift_reason_recorded"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
WS="${TMP}/repo"

# Helper: build a 3-commit sibling repo with a side branch.
# 1. Clean on-pin
SIB_ONPIN="${TMP}/sib_onpin"
REV_ONPIN="$(create_synthetic_sibling "${SIB_ONPIN}" "sib-onpin")"

# 2. Clean fast-forward (stale-lock)
SIB_FF="${TMP}/sib_ff"
REV_FF_PIN="$(create_synthetic_sibling "${SIB_FF}" "sib-ff")"
printf '// commit 2\n' >> "${SIB_FF}/src/lib.rs"
git_commit_all "${SIB_FF}" "advance ff" >/dev/null

# 3. Dirty at pin
SIB_DIRTY="${TMP}/sib_dirty"
REV_DIRTY="$(create_synthetic_sibling "${SIB_DIRTY}" "sib-dirty")"
printf '// uncommitted edit\n' >> "${SIB_DIRTY}/src/lib.rs"

# 4. Retreated (HEAD is ancestor of pin)
SIB_RET="${TMP}/sib_ret"
REV_RET_OLD="$(create_synthetic_sibling "${SIB_RET}" "sib-ret")"
printf '// newer commit\n' >> "${SIB_RET}/src/lib.rs"
REV_RET_PIN="$(git_commit_all "${SIB_RET}" "newer commit")"
git -C "${SIB_RET}" checkout -q "${REV_RET_OLD}"

# 5. Diverged (side branch vs pin on main)
SIB_DIV="${TMP}/sib_div"
REV_DIV_BASE="$(create_synthetic_sibling "${SIB_DIV}" "sib-div")"
printf '// main branch\n' >> "${SIB_DIV}/src/lib.rs"
REV_DIV_PIN="$(git_commit_all "${SIB_DIV}" "main commit")"
git -C "${SIB_DIV}" checkout -q -b side "${REV_DIV_BASE}"
printf '// side branch\n' >> "${SIB_DIV}/src/lib.rs"
git_commit_all "${SIB_DIV}" "side commit" >/dev/null

# 6. Unknown pin (40-hex commit absent from repo)
SIB_UNK="${TMP}/sib_unk"
create_synthetic_sibling "${SIB_UNK}" "sib-unk" >/dev/null
REV_UNK_PIN="deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"

# 7. Dirty AND retreated -> precedence requires reason == "dirty"
SIB_BOTH="${TMP}/sib_both"
REV_BOTH_OLD="$(create_synthetic_sibling "${SIB_BOTH}" "sib-both")"
printf '// newer\n' >> "${SIB_BOTH}/src/lib.rs"
REV_BOTH_PIN="$(git_commit_all "${SIB_BOTH}" "newer")"
git -C "${SIB_BOTH}" checkout -q "${REV_BOTH_OLD}"
printf '// dirty on top of retreated\n' >> "${SIB_BOTH}/src/lib.rs"

LOCK_BODY="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-01-ONPIN"
name = "sib-onpin"
posture = "consume-path"
path = "../sib_onpin"
git_rev = "${REV_ONPIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-01"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-02-FF"
name = "sib-ff"
posture = "consume-path"
path = "../sib_ff"
git_rev = "${REV_FF_PIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-02"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-03-DIRTY"
name = "sib-dirty"
posture = "consume-path"
path = "../sib_dirty"
git_rev = "${REV_DIRTY}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-03"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-04-RET"
name = "sib-ret"
posture = "consume-path"
path = "../sib_ret"
git_rev = "${REV_RET_PIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-04"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-05-DIV"
name = "sib-div"
posture = "consume-path"
path = "../sib_div"
git_rev = "${REV_DIV_PIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-05"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-06-UNK"
name = "sib-unk"
posture = "consume-path"
path = "../sib_unk"
git_rev = "${REV_UNK_PIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-06"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-07-BOTH"
name = "sib-both"
posture = "consume-path"
path = "../sib_both"
git_rev = "${REV_BOTH_PIN}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-07"
receipt = ""
fallback = "fb"
EOF
)"

create_synthetic_workspace "${WS}" "${LOCK_BODY}" >/dev/null
"${FIRST_RECEIPT}" --stdout --root "${WS}" > "${TMP}/receipt.json"

python3 - "${TMP}/receipt.json" <<'PYEOF'
import json, sys
r = json.load(open(sys.argv[1], "r", encoding="utf-8"))
by_fd = {s["fd"]: s for s in r["siblings"]}

expected = {
    "FD-01-ONPIN": (False, "on-pin", None, True),
    "FD-02-FF":    (False, "stale-lock", None, False),
    "FD-03-DIRTY": (True,  "drift", "dirty", False),
    "FD-04-RET":   (False, "drift", "retreated", False),
    "FD-05-DIV":   (False, "drift", "diverged", False),
    "FD-06-UNK":   (False, "drift", "unknown-pin", False),
    "FD-07-BOTH":  (True,  "drift", "dirty", False),
}

for fd, (exp_dirty, exp_cls, exp_reason, exp_agrees) in expected.items():
    s = by_fd[fd]
    assert s["dirty"] is exp_dirty, f"{fd}: dirty={s['dirty']} expected {exp_dirty}"
    if exp_dirty:
        assert isinstance(s["tree_hash"], str) and len(s["tree_hash"]) == 64, f"{fd}: missing tree_hash"
    else:
        assert s["tree_hash"] is None, f"{fd}: clean sibling must have null tree_hash"
    ol = s["own_lock"]
    assert ol["class"] == exp_cls, f"{fd}: class={ol['class']} expected {exp_cls}"
    assert ol["reason"] == exp_reason, f"{fd}: reason={ol['reason']} expected {exp_reason}"
    assert ol["agrees"] is exp_agrees, f"{fd}: agrees={ol['agrees']} expected {exp_agrees}"
PYEOF

pass_test
