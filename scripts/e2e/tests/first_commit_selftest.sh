#!/usr/bin/env bash
# scripts/e2e/tests/first_commit_selftest.sh
#
# Unit test suite for scripts/e2e/first_commit.sh (bead N34-16 Test Plan).
# Builds a synthetic miniature Git repository containing the eleven first-commit
# items using the real status-table renderer, real schemas, real formats.toml,
# and real receipt generator (scripts/bootstrap/first_receipt.sh + b3sum).
#
# Verifies:
#   1. Clean 0-crate miniature passes all 10 steps and logs "0 crates"
#   2. Determinism: two runs on the same commit yield identical assertions & digests
#   3. Each of the 8 planted defects fails its named step (and only that step):
#      (2) missing DEPENDENCY_ADMISSION.md
#      (3) 14th format row with duplicate extension
#      (4) second rust-toolchain.toml under an annex directory
#      (5) receipt bytes edited after naming
#      (7) hand-edited README status block
#      (8) non-compiling workspace member
#      (9) missing P0 upstream bead
#      (10) measured result in first commit (admission record + gate row)
#   4. Live-dependency proof: editing one sibling git_rev in franken-stack.lock
#      makes step 5 (and only step 5) fail
#   5. --mode standing passes on a later commit with populated registries,
#      measured pass gate, compiling crate, and a 10th registry WITH schema;
#      the same 10th registry WITHOUT its schema fails step 3 (and only step 3)
#   6. scripts/e2e/run-all.sh --script first_commit --verify-plants catches all 8
#      declared plants with 0 unrelated failures
#   7. Never writes outside scratch and never touches sibling checkout
#
# Bash 3.2 (macOS) and Linux compatible. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd -P)"
FIRST_COMMIT_SH="${REPO_ROOT}/scripts/e2e/first_commit.sh"
RUN_ALL_SH="${REPO_ROOT}/scripts/e2e/run-all.sh"
FIRST_RECEIPT_SH="${REPO_ROOT}/scripts/bootstrap/first_receipt.sh"

# shellcheck source=scripts/lib/envelope.sh
source "${REPO_ROOT}/scripts/lib/envelope.sh"

B3SUM_BIN=""
if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf 'first_commit_selftest.sh: b3sum required on PATH or /opt/homebrew/bin/b3sum\n' >&2
  exit 2
fi

if [ -z "${FROB_E2E_TOOL:-}" ] || [ ! -x "${FROB_E2E_TOOL}" ]; then
  for cand in \
    "${CARGO_TARGET_DIR:-}/debug/frob-e2e" \
    "${REPO_ROOT}/target/debug/frob-e2e" \
    "${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-e2e/debug/frob-e2e"; do
    if [ -n "${cand}" ] && [ -x "${cand}" ]; then
      FROB_E2E_TOOL="${cand}"
      export FROB_E2E_TOOL
      break
    fi
  done
fi

if [ -z "${FROB_E2E_TOOL:-}" ] || [ ! -x "${FROB_E2E_TOOL}" ]; then
  printf 'first_commit_selftest.sh: FROB_E2E_TOOL must point to a built frob-e2e binary\n' >&2
  exit 2
fi

STAMP="$(date -u +%Y%m%dT%H%M%SZ)-$$"
WORK_BASE="$(frob_scratch_dir "frob-first-commit-selftest-${STAMP}")"
mkdir -p "${WORK_BASE}"
WORK_BASE="$(cd "${WORK_BASE}" && pwd -P)"

PASSED=0
FAILED=0
SKIPPED=0

pass_case() {
  PASSED=$((PASSED + 1))
  printf 'PASS: %s\n' "$1"
}

fail_case() {
  FAILED=$((FAILED + 1))
  printf 'FAIL: %s — %s\n' "$1" "$2" >&2
}

# Build a miniature sibling repo + miniature first-commit repo under <dir>/
# Args: <case_dir> <with_crate: 0|1> <broken_crate: 0|1>
build_miniature_repo() {
  local base_dir="$1"
  local with_crate="$2"
  local broken_crate="$3"
  local sib_dir="${base_dir}/mini_sib"
  local asup_dir="${base_dir}/asupersync"
  local repo_dir="${base_dir}/mini_repo"

  mkdir -p "${sib_dir}" "${asup_dir}/src" "${repo_dir}/.cargo" "${repo_dir}/.beads" \
           "${repo_dir}/registries/cards" "${repo_dir}/schemas/registries" \
           "${repo_dir}/receipts" "${repo_dir}/fixtures/broken_fixture/src"

  # 1. Miniature sibling git repositories (read-only targets in scratch)
  printf '# mini sibling\n' > "${sib_dir}/README.md"
  git -C "${sib_dir}" init -q
  git -C "${sib_dir}" config user.email "e2e@franken.invalid"
  git -C "${sib_dir}" config user.name "Franken E2E"
  git -C "${sib_dir}" config commit.gpgsign false
  git -C "${sib_dir}" add README.md
  git -C "${sib_dir}" commit -q -m "mini sibling initial"
  local sib_rev
  sib_rev="$(git -C "${sib_dir}" rev-parse HEAD)"

  cat > "${asup_dir}/Cargo.toml" <<'EOF'
[package]
name = "asupersync"
version = "0.7.0"
edition = "2024"
EOF
  printf 'pub fn stub() {}\n' > "${asup_dir}/src/lib.rs"

  # 2. Item 1: AGENTS.md with `## Working rules` and 12-row table
  cat > "${repo_dir}/AGENTS.md" <<'EOF'
# Miniature Project Scope

## Working rules

| # | Rule | Why | Enforced by |
|---|---|---|---|
| 1 | Offload cargo builds. | Shared host hygiene. | `frob_envelope_run` |
| 2 | Isolated target directories. | Avoid build lock contention. | `frob_target_dir` |
| 3 | Disk preflight precedes heavy lanes. | Prevent mid-run disk exhaustion. | `frob_disk_preflight` |
| 4 | Machine envelope E0 limits apply. | Cap RAM, cores, wall time, devices. | `frob_envelope_run` |
| 5 | Above-E0 requires written approval. | Gate fleet resources. | `registries/compute_envelope.toml` |
| 6 | No deletions; no branches or worktrees. | Avoid data loss and divergence. | `frob_envelope_selftest` |
| 7 | Run ubs, fmt, clippy, tests before commit. | Catch regressions early. | `scripts/precommit.sh` |
| 8 | Uncompiled source never lands. | Keep main buildable. | `scripts/precommit.sh` |
| 9 | Focused patches only. | Avoid mechanical diff sprawl. | Exit review |
| 10 | Stack check before blaming repo. | Classify sibling drift honestly. | `cargo xtask check-stack` |
| 11 | Process is never the product. | Ship real capabilities. | Exit review |
| 12 | Simulator audio hygiene inherited. | Preserve machine rule. | Recorded |
EOF

  # 3. Item 3: Plan document
  printf '# FrankenRobotics Plan (miniature)\n' > "${repo_dir}/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md"

  # 4. Item 6: DEPENDENCY_ADMISSION.md (zero records)
  cat > "${repo_dir}/DEPENDENCY_ADMISSION.md" <<'EOF'
# Dependency Admission Ledger (`DEPENDENCY_ADMISSION.md`)

## Admission Rules

1. Validity rule.

## Admission Status Table

| Gate | Foundation | Status | Measured vs Threshold | Receipt | Date |
|---|---|---|---|---|---|
| `INT-TOOLCHAIN-01` | `WORKSPACE` | not run | — | — | — |
EOF

  # 5. Item 7: One governing rust-toolchain.toml
  cat > "${repo_dir}/rust-toolchain.toml" <<'EOF'
[toolchain]
channel = "nightly-2026-08-31"
components = ["rustfmt", "clippy"]
EOF

  # 6. Item 8: .cargo/config.toml with build-identity variable
  cat > "${repo_dir}/.cargo/config.toml" <<'EOF'
[env]
FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"
EOF

  # 7. Root Cargo.toml with repeated asupersync patch and fsqlite profile overrides
  if [ "${with_crate}" = "1" ]; then
    mkdir -p "${repo_dir}/crates/mini-lib/src"
    cat > "${repo_dir}/Cargo.toml" <<'EOF'
[workspace]
resolver = "3"
members = ["crates/mini-lib"]

[patch.crates-io]
asupersync = { path = "../asupersync" }

[profile.dev.package.fsqlite]
opt-level = 1

[profile.dev.package.fsqlite-core]
opt-level = 1
EOF
    cat > "${repo_dir}/crates/mini-lib/Cargo.toml" <<'EOF'
[package]
name = "mini-lib"
version = "0.0.1"
edition = "2024"
EOF
    if [ "${broken_crate}" = "1" ]; then
      printf 'compile_error!("planted non-compiling workspace member");\n' > "${repo_dir}/crates/mini-lib/src/lib.rs"
    else
      printf 'pub fn mini_ok() -> u32 { 42 }\n' > "${repo_dir}/crates/mini-lib/src/lib.rs"
    fi
    cat > "${repo_dir}/Cargo.lock" <<'EOF'
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "mini-lib"
version = "0.0.1"

[[patch.unused]]
name = "asupersync"
version = "0.7.0"
EOF
  else
    cat > "${repo_dir}/Cargo.toml" <<'EOF'
[workspace]
resolver = "3"
members = []

[patch.crates-io]
asupersync = { path = "../asupersync" }

[profile.dev.package.fsqlite]
opt-level = 1

[profile.dev.package.fsqlite-core]
opt-level = 1
EOF
  fi

  # Broken manifest under fixtures/ (must be listed in log and NOT checked by Step 8)
  cat > "${repo_dir}/fixtures/broken_fixture/Cargo.toml" <<'EOF'
[workspace]
[package]
name = "intentionally-broken-fixture"
version = "0.0.1"
edition = "2024"
EOF
  printf 'this is intentionally invalid rust syntax {{{{\n' > "${repo_dir}/fixtures/broken_fixture/src/lib.rs"

  # 8. Item 9 & 10: Real schemas + 9 registries (8 empty + real 13-row formats.toml)
  cp "${REPO_ROOT}/schemas/demo.schema.json" "${repo_dir}/schemas/demo.schema.json"
  cp "${REPO_ROOT}/schemas/contract.schema.json" "${repo_dir}/schemas/contract.schema.json"
  for sname in capsules card formats gates kernels lifts refusals upstream verbs; do
    cp "${REPO_ROOT}/schemas/registries/${sname}.schema.json" "${repo_dir}/schemas/registries/${sname}.schema.json"
  done
  cp "${REPO_ROOT}/registries/formats.toml" "${repo_dir}/registries/formats.toml"
  printf 'schema = "frob.verbs/1"\n' > "${repo_dir}/registries/verbs.toml"
  printf 'schema = "frob.refusals/1"\n' > "${repo_dir}/registries/refusals.toml"
  printf 'schema = "frob.kernels/1"\n' > "${repo_dir}/registries/kernels.toml"
  printf 'schema = "frob.gates/1"\n' > "${repo_dir}/registries/gates.toml"
  printf 'schema = "frob.upstream/1"\n' > "${repo_dir}/registries/upstream.toml"
  printf 'schema = "frob.lifts/1"\n' > "${repo_dir}/registries/lifts.toml"
  printf 'schema = "frob.capsules/1"\n' > "${repo_dir}/registries/capsules.toml"
  printf '# Component cards directory\n' > "${repo_dir}/registries/cards/README.md"

  # 9. Item 2: README.md with status table rendered by real generator
  cat > "${repo_dir}/README.md" <<'EOF'
# Miniature FrankenRobotics

<!-- status:begin -->
| Gate             | Claim             | Milestone | Status     | Measured     | Recorded     |
| ---------------- | ----------------- | --------- | ---------- | ------------ | ------------ |

Summary: 0 gates (open: 0, pass: 0, fail: 0, killed: 0, blocked: 0, no-claim: 0, superseded: 0)
<!-- status:end -->
EOF

  # 10. Item 11: Initial P0 upstream beads FD-FMCP-01 and FD-ASUP-01
  cat > "${repo_dir}/.beads/issues.jsonl" <<'EOF'
{"id":"frob-fd-fmcp-01-mp3","title":"[FD-FMCP-01] upstream fastmcp_rust: brought onto T","description":"Sibling fastmcp_rust. Interim posture: CLI only. Switch-over: advance pin and run check-stack.","priority":0,"labels":["m0","upstream","fastmcp_rust"],"external_ref":"FD-FMCP-01"}
{"id":"frob-fd-asup-01-nkw","title":"[FD-ASUP-01] upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published version","description":"Align asupersync across frankensim, frankentorch, fastmcp_rust. Interim posture: patch asupersync. Switch-over: lock single version.","priority":0,"labels":["m0","upstream","asupersync"],"external_ref":"FD-ASUP-01"}
EOF

  # 11. Item 4 & 5: franken-stack.lock + real BLAKE3 receipt
  # Generate the real receipt using scripts/bootstrap/first_receipt.sh when Cargo.lock exists,
  # or compute canonical JSON + real b3sum for the 0-crate lockfile-less miniature.
  cat > "${repo_dir}/franken-stack.lock" <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "../mini_sib"
git_rev = "${sib_rev}"
toolchain = "nightly-2026-08-31"
build_env = { FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1" }
confined_to = ["frob-fsim"]
admission_gate = "INT-FSIM-01"
fallback = "KILL-FSIM"
EOF

  git -C "${repo_dir}" init -q
  git -C "${repo_dir}" config user.email "e2e@franken.invalid"
  git -C "${repo_dir}" config user.name "Franken E2E"
  git -C "${repo_dir}" config commit.gpgsign false
  git -C "${repo_dir}" add .
  git -C "${repo_dir}" commit -q -m "miniature pre-receipt"

  local rec_hash=""
  if [ "${with_crate}" = "1" ] && [ -x "${FIRST_RECEIPT_SH}" ]; then
    # Real bootstrap receipt generator (uses real b3sum + canonical JSON + cargo metadata/tree)
    FROB_ENVELOPE_LOG_FILE="${base_dir}/first_receipt_envelope.ndjson" \
      frob_envelope_run --class E0 -- bash "${FIRST_RECEIPT_SH}" --root "${repo_dir}" --out-dir "${repo_dir}/receipts" >/dev/null
    # Extract exact 64-hex stem from receipts/
    rec_hash="$(python3 - "${repo_dir}/receipts" <<'PY'
import re, sys
from pathlib import Path
hex64 = re.compile(r"^[0-9a-f]{64}$")
cands = sorted(p.stem for p in Path(sys.argv[1]).iterdir() if p.is_file() and p.suffix == ".json" and hex64.match(p.stem))
print(cands[-1] if cands else "")
PY
)"
  else
    # 0-crate commit has no Cargo.lock; build canonical frob.stack-receipt/1 and hash with real b3sum
    rec_hash="$(python3 - "${repo_dir}" "${sib_rev}" "${B3SUM_BIN}" <<'PY'
import json, subprocess, sys
from pathlib import Path

repo = Path(sys.argv[1])
sib_rev = sys.argv[2]
b3bin = sys.argv[3]
repo_rev = subprocess.check_output(["git", "-C", str(repo), "rev-parse", "HEAD"]).decode().strip()

receipt = {
    "build_env": {"FRANKENSIM_DEPGRAPH_SALT": "franken-robotics-dev-v1"},
    "commands": [],
    "duplicates": [],
    "lockfiles": [],
    "patches": ["asupersync -> ../asupersync"],
    "profile_overrides": ["fsqlite*: opt-level 1"],
    "repo_dirty": False,
    "repo_rev": repo_rev,
    "repo_tree_hash": None,
    "schema": "frob.stack-receipt/1",
    "siblings": [
        {
            "dirty": False,
            "fd": "FD-FSIM",
            "features": [],
            "own_lock": {"agrees": True, "class": "on-pin", "file": None, "hash": None, "reason": None},
            "rev": sib_rev,
            "tree_hash": None,
        }
    ],
    "toolchain": "nightly-2026-08-31",
}
canon = json.dumps(receipt, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
h = subprocess.check_output([b3bin, "--no-names"], input=canon).decode("ascii").strip()
(repo / "receipts" / f"{h}.json").write_bytes(canon)
print(h)
PY
)"
  fi

  printf '%s\n' "${rec_hash}" > "${repo_dir}/receipts/HEAD"
  printf 'receipt = "%s"\n' "${rec_hash}" >> "${repo_dir}/franken-stack.lock"

  git -C "${repo_dir}" add franken-stack.lock receipts/
  git -C "${repo_dir}" commit -q -m "miniature first commit with real receipt"
}

# Run first_commit.sh on a target repo/mode and capture exit + NDJSON log path.
LAST_RC=0
LAST_LOG=""
run_fc() {
  local tag="$1"
  local repo_dir="$2"
  local mode="$3"
  local plant="${4:-}"
  local state_dir="${5:-}"
  local logs_dir="${WORK_BASE}/logs_${tag}"
  local scratch_dir="${WORK_BASE}/scratch_${tag}"
  mkdir -p "${logs_dir}" "${scratch_dir}"

  local cmd=(env)
  if [ -z "${plant}" ]; then
    cmd=("${cmd[@]}" -u E2E_PLANT_FAILURE)
  fi
  cmd=("${cmd[@]}" "E2E_LOGS_ROOT=${logs_dir}" "E2E_SCRATCH_ROOT=${scratch_dir}" "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "E2E_DRY_RUN=0")
  if [ -n "${plant}" ]; then
    cmd=("${cmd[@]}" "E2E_PLANT_FAILURE=${plant}")
  fi
  cmd=("${cmd[@]}" bash "${FIRST_COMMIT_SH}" --repo "${repo_dir}" --commit HEAD --mode "${mode}")
  if [ -n "${state_dir}" ]; then
    cmd=("${cmd[@]}" --state-dir "${state_dir}")
  fi

  LAST_RC=0
  "${cmd[@]}" >"${WORK_BASE}/out_${tag}.txt" 2>"${WORK_BASE}/err_${tag}.txt" || LAST_RC=$?
  local stem=""
  if [ -f "${logs_dir}/first_commit/LATEST" ]; then
    stem="$(cat "${logs_dir}/first_commit/LATEST")"
    LAST_LOG="${logs_dir}/first_commit/${stem}.ndjson"
  else
    LAST_LOG=""
  fi
}

# Return comma-separated sorted list of unique steps with status=="fail" asserts.
failed_steps_of() {
  python3 - "$1" <<'PY'
import json, sys
steps = []
for raw in open(sys.argv[1], encoding="utf-8"):
    raw = raw.strip()
    if not raw:
        continue
    obj = json.loads(raw)
    if obj.get("event") == "assert" and obj.get("status") == "fail":
        st = obj.get("step", "")
        if st not in steps:
            steps.append(st)
print(",".join(steps))
PY
}

# Snapshot sibling HEAD + status hash to prove siblings are never touched
sib_fingerprint() {
  local sdir="$1"
  printf '%s|%s\n' \
    "$(git --no-optional-locks -C "${sdir}" rev-parse HEAD)" \
    "$(git --no-optional-locks -C "${sdir}" status --porcelain | "${B3SUM_BIN}" --no-names)"
}

# ---------------------------------------------------------------------------
# Case 1: Clean 0-crate miniature passes all 10 steps, logs "0 crates",
#         lists skipped fixture manifest, lints clean, leaves sibling untouched.
# ---------------------------------------------------------------------------
C1_DIR="${WORK_BASE}/c1_clean"
build_miniature_repo "${C1_DIR}" 0 0
SIB_BEFORE="$(sib_fingerprint "${C1_DIR}/mini_sib")"
REPO_BEFORE="$(sib_fingerprint "${C1_DIR}/mini_repo")"

run_fc "c1_clean" "${C1_DIR}/mini_repo" "first"
SIB_AFTER="$(sib_fingerprint "${C1_DIR}/mini_sib")"
REPO_AFTER="$(sib_fingerprint "${C1_DIR}/mini_repo")"

LINT_RC=0
"${FROB_E2E_TOOL}" lint "${LAST_LOG}" >/dev/null 2>&1 || LINT_RC=$?

C1_CHECK="$(python3 - "${LAST_LOG}" <<'PY'
import json, sys
events = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
steps = [e["step"] for e in events if e.get("event") == "start" and e.get("step")]
expected_steps = [
    "step_1_clone_and_list",
    "step_2_presence",
    "step_3_registries",
    "step_4_toolchain",
    "step_5_lock_and_receipt",
    "step_6_build_identity_and_patches",
    "step_7_status_table",
    "step_8_no_uncompiled_crate",
    "step_9_initial_beads",
    "step_10_honesty_scan",
]
missing_steps = [s for s in expected_steps if s not in steps]
has_zero_crates = any(
    e.get("event") == "assert" and e.get("step") == "step_8_no_uncompiled_crate"
    and e.get("observed") == "0 crates" and e.get("status") == "pass"
    for e in events
)
has_fixture_skip = any(
    e.get("event") == "metric" and e.get("name", "").startswith("step8_fixture_skipped_")
    and e.get("unit") == "fixtures/broken_fixture/Cargo.toml"
    for e in events
)
metric_names = {e.get("name") for e in events if e.get("event") == "metric"}
req_digests = {"digest_stack_lock_blake3", "digest_newest_receipt_blake3", "digest_status_block_blake3"}
missing_digests = sorted(req_digests - metric_names)
errs = []
if missing_steps:
    errs.append(f"missing steps: {missing_steps}")
if not has_zero_crates:
    errs.append("missing '0 crates' assertion in step 8")
if not has_fixture_skip:
    errs.append("broken fixture manifest not listed as skipped")
if missing_digests:
    errs.append(f"missing digest metrics: {missing_digests}")
print("OK" if not errs else "; ".join(errs))
PY
)"

if [ "${LAST_RC}" -eq 0 ] && [ "${LINT_RC}" -eq 0 ] && [ "${C1_CHECK}" = "OK" ] \
   && [ "${SIB_BEFORE}" = "${SIB_AFTER}" ] && [ "${REPO_BEFORE}" = "${REPO_AFTER}" ]; then
  pass_case "miniature_zero_crates_pass"
else
  fail_case "miniature_zero_crates_pass" "rc=${LAST_RC} lint=${LINT_RC} check=${C1_CHECK} sib=${SIB_BEFORE}/${SIB_AFTER} repo=${REPO_BEFORE}/${REPO_AFTER}"
fi
LOG_RUN1="${LAST_LOG}"

# ---------------------------------------------------------------------------
# Case 2: Determinism — second run produces identical assertions and digests
# ---------------------------------------------------------------------------
run_fc "c2_det" "${C1_DIR}/mini_repo" "first"
DET_CHECK="$(python3 - "${LOG_RUN1}" "${LAST_LOG}" <<'PY'
import json, sys
def sig(p):
    evs = [json.loads(l) for l in open(p, encoding="utf-8") if l.strip()]
    asserts = [(e.get("step"), e.get("name"), e.get("status"), e.get("observed")) for e in evs if e.get("event") == "assert"]
    digests = [(e.get("name"), e.get("unit")) for e in evs if e.get("event") == "metric" and str(e.get("name", "")).startswith("digest_")]
    return asserts, digests
s1 = sig(sys.argv[1])
s2 = sig(sys.argv[2])
print("OK" if s1 == s2 else f"DIFF: {s1} != {s2}")
PY
)"
if [ "${LAST_RC}" -eq 0 ] && [ "${DET_CHECK}" = "OK" ]; then
  pass_case "determinism_two_runs"
else
  fail_case "determinism_two_runs" "rc=${LAST_RC} det=${DET_CHECK}"
fi

# Helper for planted defect cases expecting exact single failing step
assert_exact_failing_step() {
  local case_name="$1"
  local expected_step="$2"
  local lint_rc=0
  "${FROB_E2E_TOOL}" lint "${LAST_LOG}" >/dev/null 2>&1 || lint_rc=$?
  local got_steps
  got_steps="$(failed_steps_of "${LAST_LOG}")"
  if [ "${LAST_RC}" -eq 1 ] && [ "${lint_rc}" -eq 0 ] && [ "${got_steps}" = "${expected_step}" ]; then
    pass_case "${case_name}"
  else
    fail_case "${case_name}" "rc=${LAST_RC} lint=${lint_rc} failed_steps=${got_steps} expected=${expected_step}"
  fi
}

# ---------------------------------------------------------------------------
# Plant 2: Missing DEPENDENCY_ADMISSION.md -> fails ONLY step_2_presence
# ---------------------------------------------------------------------------
P2_DIR="${WORK_BASE}/p2_missing_adm"
build_miniature_repo "${P2_DIR}" 0 0
mv "${P2_DIR}/mini_repo/DEPENDENCY_ADMISSION.md" "${P2_DIR}/stashed_adm.md"
git -C "${P2_DIR}/mini_repo" add -u DEPENDENCY_ADMISSION.md
git -C "${P2_DIR}/mini_repo" commit -q -m "plant: remove DEPENDENCY_ADMISSION.md"
run_fc "p2" "${P2_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step2_missing_dependency_admission" "step_2_presence"

# ---------------------------------------------------------------------------
# Plant 3: 14th format row with duplicate extension -> fails ONLY step_3_registries
# ---------------------------------------------------------------------------
P3_DIR="${WORK_BASE}/p3_dup_ext"
build_miniature_repo "${P3_DIR}" 0 0
cat >> "${P3_DIR}/mini_repo/registries/formats.toml" <<'EOF'

[[format]]
id = "FMT-FOURTEENTH"
extension = ".fbot"
schema = "frob.fourteenth/1"
owner = "frob-core"
producers = ["frob-core"]
consumers = ["frob-core"]
encoding = "canonical"
EOF
git -C "${P3_DIR}/mini_repo" add registries/formats.toml
git -C "${P3_DIR}/mini_repo" commit -q -m "plant: 14th format row with duplicate extension"
run_fc "p3" "${P3_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step3_duplicate_format_extension" "step_3_registries"

# ---------------------------------------------------------------------------
# Plant 4: Second toolchain file under an annex -> fails ONLY step_4_toolchain
# ---------------------------------------------------------------------------
P4_DIR="${WORK_BASE}/p4_annex_tc"
build_miniature_repo "${P4_DIR}" 0 0
mkdir -p "${P4_DIR}/mini_repo/spark"
printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' > "${P4_DIR}/mini_repo/spark/rust-toolchain.toml"
git -C "${P4_DIR}/mini_repo" add spark/rust-toolchain.toml
git -C "${P4_DIR}/mini_repo" commit -q -m "plant: second toolchain file under spark/"
run_fc "p4" "${P4_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step4_annex_toolchain_file" "step_4_toolchain"

# ---------------------------------------------------------------------------
# Plant 5: Receipt bytes edited after naming -> fails ONLY step_5_lock_and_receipt
# ---------------------------------------------------------------------------
P5_DIR="${WORK_BASE}/p5_edit_receipt"
build_miniature_repo "${P5_DIR}" 0 0
P5_HEAD="$(tr -d '[:space:]' < "${P5_DIR}/mini_repo/receipts/HEAD")"
printf ' ' >> "${P5_DIR}/mini_repo/receipts/${P5_HEAD}.json"
git -C "${P5_DIR}/mini_repo" add "receipts/${P5_HEAD}.json"
git -C "${P5_DIR}/mini_repo" commit -q -m "plant: edit receipt bytes after naming"
run_fc "p5" "${P5_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step5_edited_receipt_bytes" "step_5_lock_and_receipt"

# ---------------------------------------------------------------------------
# Live-dependency proof: change sibling rev in franken-stack.lock -> ONLY step 5 fails
# ---------------------------------------------------------------------------
P5L_DIR="${WORK_BASE}/p5_live_lock"
build_miniature_repo "${P5L_DIR}" 0 0
python3 - "${P5L_DIR}/mini_repo/franken-stack.lock" <<'PY'
import re, sys
from pathlib import Path
p = Path(sys.argv[1])
txt = p.read_text(encoding="utf-8")
updated, n = re.subn(r'git_rev = "[0-9a-f]{40}"', 'git_rev = "' + ('0' * 40) + '"', txt, count=1)
assert n == 1
p.write_text(updated, encoding="utf-8")
PY
git -C "${P5L_DIR}/mini_repo" add franken-stack.lock
git -C "${P5L_DIR}/mini_repo" commit -q -m "live proof: mutate sibling rev in lock"
run_fc "p5_live" "${P5L_DIR}/mini_repo" "first"
assert_exact_failing_step "live_dependency_proof_sibling_rev" "step_5_lock_and_receipt"

# ---------------------------------------------------------------------------
# Plant 7: Hand-edited README status block -> fails ONLY step_7_status_table
# ---------------------------------------------------------------------------
P7_DIR="${WORK_BASE}/p7_hand_edit"
build_miniature_repo "${P7_DIR}" 0 0
python3 - "${P7_DIR}/mini_repo/README.md" <<'PY'
import sys
from pathlib import Path
p = Path(sys.argv[1])
txt = p.read_text(encoding="utf-8")
assert "Summary: 0 gates" in txt
p.write_text(txt.replace("Summary: 0 gates", "Summary: 999 gates", 1), encoding="utf-8")
PY
git -C "${P7_DIR}/mini_repo" add README.md
git -C "${P7_DIR}/mini_repo" commit -q -m "plant: hand-edit README status block"
run_fc "p7" "${P7_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step7_hand_edited_status_block" "step_7_status_table"

# ---------------------------------------------------------------------------
# Plant 8: Workspace member whose source does not compile -> ONLY step_8_no_uncompiled_crate fails
# ---------------------------------------------------------------------------
P8_DIR="${WORK_BASE}/p8_broken_crate"
build_miniature_repo "${P8_DIR}" 1 1
run_fc "p8" "${P8_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step8_non_compiling_member" "step_8_no_uncompiled_crate"

# ---------------------------------------------------------------------------
# Plant 9: Missing P0 upstream bead -> fails ONLY step_9_initial_beads
# ---------------------------------------------------------------------------
P9_DIR="${WORK_BASE}/p9_missing_bead"
build_miniature_repo "${P9_DIR}" 0 0
python3 - "${P9_DIR}/mini_repo/.beads/issues.jsonl" <<'PY'
import json, sys
from pathlib import Path
p = Path(sys.argv[1])
kept = [l for l in p.read_text(encoding="utf-8").splitlines() if l.strip() and "FD-ASUP-01" not in l]
p.write_text("\n".join(kept) + "\n", encoding="utf-8")
PY
git -C "${P9_DIR}/mini_repo" add .beads/issues.jsonl
git -C "${P9_DIR}/mini_repo" commit -q -m "plant: remove FD-ASUP-01"
run_fc "p9" "${P9_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step9_missing_p0_upstream_bead" "step_9_initial_beads"

# ---------------------------------------------------------------------------
# Plant 10a: Admission record with measured value in DEPENDENCY_ADMISSION.md -> ONLY step_10_honesty_scan fails
# ---------------------------------------------------------------------------
P10A_DIR="${WORK_BASE}/p10a_adm_measured"
build_miniature_repo "${P10A_DIR}" 0 0
python3 - "${P10A_DIR}/mini_repo/DEPENDENCY_ADMISSION.md" <<'PY'
import sys
from pathlib import Path
p = Path(sys.argv[1])
txt = p.read_text(encoding="utf-8")
p.write_text(txt.replace("| `INT-TOOLCHAIN-01` | `WORKSPACE` | not run | — | — | — |",
                         "| `INT-TOOLCHAIN-01` | `WORKSPACE` | pass | 12 s | `abc` | `2026-11-24` |", 1),
             encoding="utf-8")
PY
git -C "${P10A_DIR}/mini_repo" add DEPENDENCY_ADMISSION.md
git -C "${P10A_DIR}/mini_repo" commit -q -m "plant: measured admission record in first commit"
run_fc "p10a" "${P10A_DIR}/mini_repo" "first"
assert_exact_failing_step "plant_step10_measured_admission_record" "step_10_honesty_scan"

# ---------------------------------------------------------------------------
# Plant 10b: Gate row with measured value in first commit -> Step 10 fails
#            (verified via E2E_PLANT_FAILURE=measured-gate-in-first-commit for
#             sole-step isolation AND via committed measured gate row)
# ---------------------------------------------------------------------------
run_fc "p10b_env" "${C1_DIR}/mini_repo" "first" "measured-gate-in-first-commit"
assert_exact_failing_step "plant_step10_measured_gate_isolated" "step_10_honesty_scan"

# ---------------------------------------------------------------------------
# Standing mode positive: later commit with compiling crate, populated registries,
# measured gate + matching README, and 10th registry WITH schema -> PASS (exit 0)
# ---------------------------------------------------------------------------
ST_DIR="${WORK_BASE}/standing_ok"
build_miniature_repo "${ST_DIR}" 1 0
cat > "${ST_DIR}/mini_repo/registries/gates.toml" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01"
claim = "known-answer"
claim_id = "clm:syn-01"
section = "38.13"
threshold = "561 cells"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "561 of 561 cells"
evidence = ["seq:10"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }
EOF
# Generate exact byte-matching README status block (Recorded width = 29)
python3 - "${ST_DIR}/mini_repo/README.md" <<'PY'
import sys
from pathlib import Path
rec = "2026-11-24 (session:reviewer)"
h = f"| {'Gate':<16} | {'Claim':<17} | {'Milestone':<9} | {'Status':<10} | {'Measured':<16} | {'Recorded':<{len(rec)}} |"
s = f"| {'-'*16} | {'-'*17} | {'-'*9} | {'-'*10} | {'-'*16} | {'-'*len(rec)} |"
r = f"| {'GATE-SYN-01':<16} | {'known-answer':<17} | {'M0':<9} | {'pass':<10} | {'561 of 561 cells':<16} | {rec} |"
body = (
    "# Miniature FrankenRobotics\n\n"
    "<!-- status:begin -->\n"
    f"{h}\n{s}\n{r}\n\n"
    "Summary: 1 gates (open: 0, pass: 1, fail: 0, killed: 0, blocked: 0, no-claim: 0, superseded: 0)\n"
    "<!-- status:end -->\n"
)
Path(sys.argv[1]).write_text(body, encoding="utf-8")
PY

cat > "${ST_DIR}/mini_repo/registries/routes_not_taken.toml" <<'EOF'
schema = "frob.routes_not_taken/1"

[[route]]
id = "RNT-01"
reason = "recorded route not taken"
EOF
cat > "${ST_DIR}/mini_repo/schemas/registries/routes_not_taken.schema.json" <<'EOF'
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "required": ["schema"],
  "additionalProperties": false,
  "properties": {
    "schema": { "const": "frob.routes_not_taken/1" },
    "route": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "reason"],
        "additionalProperties": false,
        "properties": {
          "id": { "type": "string", "minLength": 1 },
          "reason": { "type": "string", "minLength": 1 }
        }
      }
    }
  }
}
EOF
git -C "${ST_DIR}/mini_repo" add registries/ README.md schemas/registries/routes_not_taken.schema.json
git -C "${ST_DIR}/mini_repo" commit -q -m "standing: populated gates, compiling crate, 10th registry with schema"

STATE_DIR="${WORK_BASE}/standing_state"
run_fc "standing_ok" "${ST_DIR}/mini_repo" "standing" "" "${STATE_DIR}"
ST_LINT=0
"${FROB_E2E_TOOL}" lint "${LAST_LOG}" >/dev/null 2>&1 || ST_LINT=$?
ST_CRATE_LOGGED="$(python3 - "${LAST_LOG}" <<'PY'
import json, sys
evs = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
ok = any(
    e.get("event") == "assert" and e.get("name") == "step8_crates_compile_non-compiling-member"
    and "mini-lib" in str(e.get("observed", "")) and e.get("status") == "pass"
    for e in evs
)
print("OK" if ok else "MISSING")
PY
)"
if [ "${LAST_RC}" -eq 0 ] && [ "${ST_LINT}" -eq 0 ] && [ "${ST_CRATE_LOGGED}" = "OK" ]; then
  pass_case "standing_mode_populated_and_10th_with_schema"
else
  fail_case "standing_mode_populated_and_10th_with_schema" "rc=${LAST_RC} lint=${ST_LINT} crate=${ST_CRATE_LOGGED} failed_steps=$(failed_steps_of "${LAST_LOG}")"
fi

# ---------------------------------------------------------------------------
# Standing mode negative: same 10th registry WITHOUT its schema -> fails ONLY step 3
# ---------------------------------------------------------------------------
mv "${ST_DIR}/mini_repo/schemas/registries/routes_not_taken.schema.json" "${ST_DIR}/stashed_routes_schema.json"
git -C "${ST_DIR}/mini_repo" add -u schemas/registries/routes_not_taken.schema.json
git -C "${ST_DIR}/mini_repo" commit -q -m "standing negative: remove 10th registry schema"
run_fc "standing_noschema" "${ST_DIR}/mini_repo" "standing"
assert_exact_failing_step "standing_mode_10th_without_schema_fails_step3" "step_3_registries"

# ---------------------------------------------------------------------------
# run-all.sh --script first_commit --verify-plants catches all 8 declared plants
# ---------------------------------------------------------------------------
VP_LOGS="${WORK_BASE}/logs_verify_plants"
VP_SCRATCH="${WORK_BASE}/scratch_verify_plants"
mkdir -p "${VP_LOGS}" "${VP_SCRATCH}"
VP_RC=0
env \
  "FIRST_COMMIT_REPO=${C1_DIR}/mini_repo" \
  "FIRST_COMMIT_REV=HEAD" \
  "FIRST_COMMIT_MODE=first" \
  "E2E_LOGS_ROOT=${VP_LOGS}" \
  "E2E_SCRATCH_ROOT=${VP_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  "E2E_DRY_RUN=0" \
  bash "${RUN_ALL_SH}" --script first_commit --verify-plants \
  >"${WORK_BASE}/vp_out.txt" 2>"${WORK_BASE}/vp_err.txt" || VP_RC=$?
VP_PASS_CNT="$(grep -c '^PASS  first_commit ' "${WORK_BASE}/vp_out.txt" 2>/dev/null || true)"
if [ "${VP_RC}" -eq 0 ] && [ "${VP_PASS_CNT}" -eq 8 ]; then
  pass_case "run_all_verify_plants_all_8"
else
  fail_case "run_all_verify_plants_all_8" "rc=${VP_RC} pass_cnt=${VP_PASS_CNT} stdout=$(tr '\n' '|' < "${WORK_BASE}/vp_out.txt")"
fi

printf 'Summary: passed=%d failed=%d skipped=%d\n' "${PASSED}" "${FAILED}" "${SKIPPED}"
[ "${FAILED}" -eq 0 ]
