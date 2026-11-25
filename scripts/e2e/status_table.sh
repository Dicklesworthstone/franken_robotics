#!/usr/bin/env bash
# e2e: bead=BD-002 milestone=M0 labels=delivery,gate,m0,registry,solid,workspace gates=GATE-GAUNT-10 seed=0 timeout_s=600 needs=cpu plants=hand-edited-block,pass-without-measured
#
# scripts/e2e/status_table.sh
# End-to-end verification of the README honest-status table generator (BD-002,
# GATE-GAUNT-10, INV-GAUNT-06). Copies the repository into a scratch workspace,
# plants the 5-row synthetic gate fixture (not-run, valid pass, pass with empty
# measured, killed, no-claim), verifies the empty-measured pass refusal, blesses
# the 4-row fixture, verifies the printed statuses and block digest, verifies
# clean check without FROB_BLESS, verifies hand-edit refusal naming the first
# differing line, and logs N34p4-03 check-generated steps as not due.
#
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "status_table" "BD-002" "$@"
e2e_gate "GATE-GAUNT-10"

REPO_COPY="${E2E_WORK_DIR}/repo_copy"
TARGET_DIR="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-status-table-e2e"
mkdir -p "${REPO_COPY}" "${TARGET_DIR}"

# ---------------------------------------------------------------------------
# Step 1: Copy workspace into scratch directory (read-only on repo; no deletions)
# ---------------------------------------------------------------------------
e2e_step "copy_workspace" "Copy workspace skeleton, xtask, frob-core, README, and registries into scratch copy"

python3 - "${E2E_REPO_ROOT}" "${REPO_COPY}" <<'PY'
import os, shutil, sys

src, dst = sys.argv[1:3]
top_files = [
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "README.md",
    "AGENTS.md",
    "DEPENDENCY_ADMISSION.md",
    "LICENSE",
    "franken-stack.lock",
]
for fn in top_files:
    sp = os.path.join(src, fn)
    if os.path.isfile(sp):
        shutil.copy2(sp, os.path.join(dst, fn))

for rel_dir in [".cargo", "crates", "registries", "schemas"]:
    sp = os.path.join(src, rel_dir)
    if os.path.isdir(sp):
        shutil.copytree(sp, os.path.join(dst, rel_dir), dirs_exist_ok=True)
PY

e2e_assert_file "scratch_readme_exists" "${REPO_COPY}/README.md"
e2e_assert_file "scratch_gates_exists" "${REPO_COPY}/registries/gates.toml"

# ---------------------------------------------------------------------------
# Step 2: Plant 5-row synthetic fixture (includes invalid pass with empty measured)
# ---------------------------------------------------------------------------
e2e_step "planted_pass_empty_measured" "Verify FROB_BLESS=1 fails on pass row with empty measured value"

cat > "${REPO_COPY}/registries/gates.toml" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01-NOTRUN"
claim = "known-answer"
claim_id = "clm:syn-01"
section = "38.13"
threshold = "exact"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }

[[gate]]
id = "GATE-SYN-02-PASS"
claim = "known-answer"
claim_id = "clm:syn-02"
section = "38.13"
threshold = "561 cells"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0002"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "561 of 561 cells"
evidence = ["seq:10"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }

[[gate]]
id = "GATE-SYN-03-BADPASS"
claim = "known-answer"
claim_id = "clm:syn-03"
section = "38.13"
threshold = "exact"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0003"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = ""
evidence = ["seq:11"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }

[[gate]]
id = "GATE-SYN-04-KILLED"
claim = "parity"
claim_id = "clm:syn-04"
section = "38.13"
threshold = "parity"
milestone = "M1"
tier = ["G2:host"]
controls = ["NC-0004"]

[[gate.part]]
milestone = "M1"
status = "killed"
measured = ""
evidence = ["seq:12"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }
kill_gate = "KILL-FSIM"
fallback = "pinned worker process"

[[gate]]
id = "GATE-SYN-05-NOCLAIM"
claim = "reality"
claim_id = "clm:syn-05"
section = "38.13"
threshold = "bench"
milestone = "M5"
tier = ["G1:host"]
controls = []

[[gate.part]]
milestone = "M5"
status = "no-claim"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "" }
EOF

if e2e_plant "pass-without-measured"; then
  # Planted negative-control verification: invert expectation so --verify-plants confirms failure detection.
  e2e_cmd --name bless_5row_planted --expect-exit "0" -- \
    env FROB_BLESS=1 "CARGO_TARGET_DIR=${TARGET_DIR}" \
    cargo test -p xtask --manifest-path "${REPO_COPY}/Cargo.toml" --test status_table readme_block_is_current -- --exact
else
  e2e_expect_caught "pass_without_measured" --gate "GATE-GAUNT-10" -- \
    env FROB_BLESS=1 "CARGO_TARGET_DIR=${TARGET_DIR}" \
    cargo test -p xtask --manifest-path "${REPO_COPY}/Cargo.toml" --test status_table readme_block_is_current -- --exact
  if grep -q "GATE-SYN-03-BADPASS" "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" 2>/dev/null; then
    e2e_assert_eq "empty_measured_names_row" "named" "named" --gate "GATE-GAUNT-10"
  else
    e2e_assert_eq "empty_measured_names_row" "missing" "named" --gate "GATE-GAUNT-10"
  fi
fi

# ---------------------------------------------------------------------------
# Step 3: Remove invalid row, bless README block, verify 4 statuses + digest
# ---------------------------------------------------------------------------
e2e_step "bless_valid_4row" "Bless 4-row fixture and assert open, pass, killed, no-claim statuses and digest"

cat > "${REPO_COPY}/registries/gates.toml" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01-NOTRUN"
claim = "known-answer"
claim_id = "clm:syn-01"
section = "38.13"
threshold = "exact"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }

[[gate]]
id = "GATE-SYN-02-PASS"
claim = "known-answer"
claim_id = "clm:syn-02"
section = "38.13"
threshold = "561 cells"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0002"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "561 of 561 cells"
evidence = ["seq:10"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }

[[gate]]
id = "GATE-SYN-04-KILLED"
claim = "parity"
claim_id = "clm:syn-04"
section = "38.13"
threshold = "parity"
milestone = "M1"
tier = ["G2:host"]
controls = ["NC-0004"]

[[gate.part]]
milestone = "M1"
status = "killed"
measured = ""
evidence = ["seq:12"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }
kill_gate = "KILL-FSIM"
fallback = "pinned worker process"

[[gate]]
id = "GATE-SYN-05-NOCLAIM"
claim = "reality"
claim_id = "clm:syn-05"
section = "38.13"
threshold = "bench"
milestone = "M5"
tier = ["G1:host"]
controls = []

[[gate.part]]
milestone = "M5"
status = "no-claim"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "" }
EOF

e2e_cmd --name bless_4row --expect-exit "0" -- \
  env FROB_BLESS=1 "CARGO_TARGET_DIR=${TARGET_DIR}" \
  cargo test -p xtask --manifest-path "${REPO_COPY}/Cargo.toml" --test status_table readme_block_is_current -- --exact

BLOCK_FILE="${E2E_WORK_DIR}/blessed_block.md"
python3 - "${REPO_COPY}/README.md" "${BLOCK_FILE}" <<'PY'
import sys
txt = open(sys.argv[1], encoding="utf-8").read()
b = txt.index("<!-- status:begin -->\n") + len("<!-- status:begin -->\n")
e = txt.index("<!-- status:end -->")
open(sys.argv[2], "w", encoding="utf-8").write(txt[b:e])
PY

BLOCK_SHA="$("${FROB_E2E_TOOL}" cmp sha256 "${BLOCK_FILE}")"
e2e_assert_digest_eq "blessed_block_digest_recorded" "${BLOCK_SHA}" "${BLOCK_SHA}" --gate "GATE-GAUNT-10"

for token in "open" "pass" "561 of 561 cells" "killed (KILL-FSIM: pinned worker process)" "no-claim" "Summary: 4 gates"; do
  if grep -Fq "$token" "${BLOCK_FILE}"; then
    e2e_assert_eq "status_token_${token%% *}" "present" "present" --gate "GATE-GAUNT-10"
  else
    e2e_assert_eq "status_token_${token%% *}" "missing" "present" --gate "GATE-GAUNT-10"
  fi
done

# ---------------------------------------------------------------------------
# Step 4: Verify clean check without FROB_BLESS (exit 0)
# ---------------------------------------------------------------------------
e2e_step "verify_clean_check" "Run readme_block_is_current without FROB_BLESS and assert exit 0"

e2e_cmd --name check_clean --expect-exit "0" -- \
  env "CARGO_TARGET_DIR=${TARGET_DIR}" \
  cargo test -p xtask --manifest-path "${REPO_COPY}/Cargo.toml" --test status_table readme_block_is_current -- --exact

# ---------------------------------------------------------------------------
# Step 5: Negative control — hand-edit README block and assert diff names line
# ---------------------------------------------------------------------------
e2e_step "hand_edit_control" "Edit README status block by hand and verify non-zero exit naming first differing line"

python3 - "${REPO_COPY}/README.md" <<'PY'
import sys
p = sys.argv[1]
txt = open(p, encoding="utf-8").read()
assert "Summary: 4 gates" in txt
txt = txt.replace("Summary: 4 gates", "Summary: 999 gates", 1)
open(p, "w", encoding="utf-8").write(txt)
PY

if e2e_plant "hand-edited-block"; then
  e2e_cmd --name check_hand_edited_planted --expect-exit "0" -- \
    env "CARGO_TARGET_DIR=${TARGET_DIR}" \
    cargo test -p xtask --manifest-path "${REPO_COPY}/Cargo.toml" --test status_table readme_block_is_current -- --exact
else
  e2e_expect_caught "hand_edited_readme_block" --gate "GATE-GAUNT-10" -- \
    env "CARGO_TARGET_DIR=${TARGET_DIR}" \
    cargo test -p xtask --manifest-path "${REPO_COPY}/Cargo.toml" --test status_table readme_block_is_current -- --exact
  if grep -q "first differing line: line " "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" 2>/dev/null; then
    e2e_assert_eq "hand_edit_names_diff_line" "named" "named" --gate "GATE-GAUNT-10"
  else
    e2e_assert_eq "hand_edit_names_diff_line" "missing" "named" --gate "GATE-GAUNT-10"
  fi
fi

# ---------------------------------------------------------------------------
# Step 6: N34p4-03 check-generated steps (not due until N34p4-03 lands)
# (Per BD-002 round-4 review note slice F-U34: check-generated --write is N34p4-03's)
# ---------------------------------------------------------------------------
e2e_step "check_generated_not_due" "Log N34p4-03 cargo xtask check-generated [--write] steps as not due"

e2e_skip "not due: cargo xtask check-generated --write and check-generated arrive in N34p4-03"
