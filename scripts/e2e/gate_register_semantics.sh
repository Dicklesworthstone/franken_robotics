#!/usr/bin/env bash
# e2e: bead=N40-01 milestone=M0 labels=gauntlet,registry seed=0 needs=cpu plants=pass-without-control,control-not-caught,self-certified,evidence-unresolved,target-as-measurement,claim-not-one-of-seven,no-claim-on-non-reality,blocked-without-question,killed-without-decision,duplicate-id,reused-id
#
# End-to-end verification of `registries/gates.toml` register semantics (bead N40-01):
#   1. Run `cargo test -p frob-conformance --test register` (all unit, property, and negative-control tests).
#   2. Validate committed `registries/gates.toml` via `validate_register` (exit 0).
#   3. Plant every one of the 10 write-time refusals (11 reason slugs) in scratch files and assert
#      `validate_register` exits 3 naming the offending row and exact `N38-REF` reason slug.
#   4. Run the `write_path` tests proving `self-certified` and `control-not-caught` leave
#      `gates.toml` bit-identical (BLAKE3 unchanged).
#   5. Verify `GATE-GROUND-10` (`needs = "import-lane"`, `claim = "reality"`) validates in both
#      resting states (`blocked` / `OPEN-GROUND-1` and `no-claim` / `no-dataset-admitted`) and
#      appears in no milestone exit's due set.
#   6. Invoke `frob capabilities --json` if the `frob` binary exists, otherwise log
#      `skipped: capability absent`.
#
# Round-4 Review Notes #2 and #6: `RC-06` (`cargo xtask check-generated` on a hand-edited status
# table) is intentionally omitted here because N40-01 does not reach BD-004/N34p4-03; `N38b-20`
# (`scripts/e2e/status_views.sh` / `tests/status_views`) owns `RC-06`.
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "gate_register_semantics" "N40-01"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
GATES_FILE="${E2E_REPO_ROOT}/registries/gates.toml"

# Exact BLAKE3 digest of the 3-byte JSON receipt "{}\n".
VALID_RECEIPT_HEX="a7c4d54776fcb179f048728766a52d75dc450cc2a5072b7e2c238aa9b2cff3b6"
MISSING_RECEIPT_HEX="0000000000000000000000000000000000000000000000000000000000000000"
VALID_CLAIM_ID="clm:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
VALID_EVIDENCE_ID="ev:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
COMMIT_SHA="0123456789abcdef0123456789abcdef01234567"

SCRATCH_ROOT="${E2E_WORK_DIR}/scratch_repo"
mkdir -p "${SCRATCH_ROOT}/receipts/gates" "${SCRATCH_ROOT}/registries"
printf '{}\n' > "${SCRATCH_ROOT}/receipts/gates/${VALID_RECEIPT_HEX}.json"

CAUGHT_CONTROLS="${SCRATCH_ROOT}/controls_caught.toml"
cat > "${CAUGHT_CONTROLS}" <<'EOF'
schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-001"
gate = ["GATE-KIN-01"]
kind = "mutation"
target = "crates/frob-core/src/lib.rs"
defect = "synthetic defect"
expect = "fails check"
due = "M0"
last_seen = { caught = true, session = "s2", date = "2026-11-24" }
EOF

UNCAUGHT_CONTROLS="${SCRATCH_ROOT}/controls_uncaught.toml"
cat > "${UNCAUGHT_CONTROLS}" <<'EOF'
schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-001"
gate = ["GATE-KIN-01"]
kind = "mutation"
target = "crates/frob-core/src/lib.rs"
defect = "synthetic defect"
expect = "fails check"
due = "M0"
last_seen = { caught = false, session = "s2", date = "2026-11-24" }
EOF

write_planted_file() {
  local slug="$1"
  local out_path="$2"
  case "${slug}" in
    pass-without-control)
      cat > "${out_path}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "${VALID_CLAIM_ID}"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = []

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "rel_err = 1e-15"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01"
receipt = "${VALID_RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2026-11-24", session = "s2", implemented_by = "s1" }
EOF
      ;;
    control-not-caught)
      cat > "${out_path}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "${VALID_CLAIM_ID}"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "rel_err = 1e-15"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01"
receipt = "${VALID_RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2026-11-24", session = "s2", implemented_by = "s1" }
EOF
      ;;
    self-certified)
      cat > "${out_path}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "${VALID_CLAIM_ID}"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "rel_err = 1e-15"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01"
receipt = "${VALID_RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2026-11-24", session = "s3", implemented_by = "s1,s3" }
EOF
      ;;
    evidence-unresolved)
      cat > "${out_path}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "${VALID_CLAIM_ID}"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "rel_err = 1e-15"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01"
receipt = "${MISSING_RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2026-11-24", session = "s2", implemented_by = "s1" }
EOF
      ;;
    target-as-measurement)
      cat > "${out_path}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "${VALID_CLAIM_ID}"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "rel_err <= 1e-12"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01"
receipt = "${VALID_RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2026-11-24", session = "s2", implemented_by = "s1" }
EOF
      ;;
    claim-not-one-of-seven)
      cat > "${out_path}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "method"
claim_id = "INV-KIN-01"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
EOF
      ;;
    no-claim-on-non-reality)
      cat > "${out_path}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "INV-KIN-01"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "no-claim"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "s2", implemented_by = "" }
detail = "no-dataset-admitted"
EOF
      ;;
    blocked-without-question)
      cat > "${out_path}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "INV-KIN-01"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "blocked"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "s2", implemented_by = "" }
detail = "waiting"
EOF
      ;;
    killed-without-decision)
      cat > "${out_path}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "INV-KIN-01"
section = "12.4"
threshold = "rel_err <= 1e-12"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "killed"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "s2", implemented_by = "" }
detail = "stopped"
EOF
      ;;
    duplicate-id)
      cat > "${out_path}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "INV-KIN-01"
section = "12.4"
threshold = "t1"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "INV-KIN-02"
section = "12.5"
threshold = "t2"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
EOF
      ;;
    reused-id)
      cat > "${out_path}" <<'EOF'
schema = "frob.gates/1"
retired_ids = ["GATE-KIN-01"]

[[gate]]
id = "GATE-KIN-01"
kind = "convergence"
claim = "parity"
claim_id = "INV-KIN-01"
section = "12.4"
threshold = "t1"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
EOF
      ;;
  esac
}

run_active_plant() {
  local slug="$1"
  local pf="${SCRATCH_ROOT}/registries/active_${slug}.toml"
  write_planted_file "${slug}" "${pf}"
  local ctrl_file="${CAUGHT_CONTROLS}"
  if [ "${slug}" = "control-not-caught" ]; then
    ctrl_file="${UNCAUGHT_CONTROLS}"
  fi
  e2e_cmd --name "active_plant_${slug}" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
      --file "${pf}" \
      --repo-root "${SCRATCH_ROOT}" \
      --controls "${ctrl_file}" \
      --live-claims "${VALID_CLAIM_ID}" \
      --live-evidence "${VALID_EVIDENCE_ID}" \
      --json
  e2e_assert_eq "${slug}:validate_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
}

# ---------------------------------------------------------------------------
# Step 1: Run `cargo test -p frob-conformance --test register`
# ---------------------------------------------------------------------------
e2e_step "unit_tests" "Run frob-conformance gate register unit and property tests"

e2e_cmd --name "register_unit_tests" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance --test register
e2e_assert_eq "register_unit_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 2: Validate committed registries/gates.toml (and active-plant hooks)
# ---------------------------------------------------------------------------
e2e_step "committed_register" "Validate committed registries/gates.toml under frob.gates/1"

if e2e_plant "pass-without-control"; then
  run_active_plant "pass-without-control"
elif e2e_plant "control-not-caught"; then
  run_active_plant "control-not-caught"
elif e2e_plant "self-certified"; then
  run_active_plant "self-certified"
elif e2e_plant "evidence-unresolved"; then
  run_active_plant "evidence-unresolved"
elif e2e_plant "target-as-measurement"; then
  run_active_plant "target-as-measurement"
elif e2e_plant "claim-not-one-of-seven"; then
  run_active_plant "claim-not-one-of-seven"
elif e2e_plant "no-claim-on-non-reality"; then
  run_active_plant "no-claim-on-non-reality"
elif e2e_plant "blocked-without-question"; then
  run_active_plant "blocked-without-question"
elif e2e_plant "killed-without-decision"; then
  run_active_plant "killed-without-decision"
elif e2e_plant "duplicate-id"; then
  run_active_plant "duplicate-id"
elif e2e_plant "reused-id"; then
  run_active_plant "reused-id"
else
  e2e_cmd --name "validate_committed_gates" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
      --file "${GATES_FILE}" --json
  e2e_assert_eq "committed_gates_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "committed_gates_ok" "/ok" "true"
    e2e_assert_json "committed_gates_schema" "/schema" "frob.gates/1"
    e2e_assert_json "committed_gates_zero_violations" "/violation_count" "0"
  fi
fi

# ---------------------------------------------------------------------------
# Step 3: Verify all 10 write-time refusals (11 reason slugs) fire on planted files
# ---------------------------------------------------------------------------
e2e_step "planted_refusals" "Assert all ten write-time refusals exit 3 naming row and exact N38-REF reason"

PLANT_COUNT=0
for slug in \
  pass-without-control \
  control-not-caught \
  self-certified \
  evidence-unresolved \
  target-as-measurement \
  claim-not-one-of-seven \
  no-claim-on-non-reality \
  blocked-without-question \
  killed-without-decision \
  duplicate-id \
  reused-id; do
  pf="${SCRATCH_ROOT}/registries/plant_${slug}.toml"
  write_planted_file "${slug}" "${pf}"
  ctrl_file="${CAUGHT_CONTROLS}"
  if [ "${slug}" = "control-not-caught" ]; then
    ctrl_file="${UNCAUGHT_CONTROLS}"
  fi
  e2e_cmd --name "refusal_${slug}" --expect-exit "3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
      --file "${pf}" \
      --repo-root "${SCRATCH_ROOT}" \
      --controls "${ctrl_file}" \
      --live-claims "${VALID_CLAIM_ID}" \
      --live-evidence "${VALID_EVIDENCE_ID}" \
      --json
  e2e_assert_eq "refusal_exit_${slug}" "${E2E_LAST_EXIT}" "3"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "refusal_not_ok_${slug}" "/ok" "false"
    e2e_assert_json "refusal_reason_${slug}" "/violations/0/reason" "${slug}"
    e2e_assert_json "refusal_row_${slug}" "/violations/0/gate_id" "GATE-KIN-01"
  fi
  PLANT_COUNT=$((PLANT_COUNT + 1))
done

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_metric "refusal_plants_caught" "${PLANT_COUNT}" "count" --observe
fi

# ---------------------------------------------------------------------------
# Step 4: Verify write-path refusals leave gates.toml bit-identical
# ---------------------------------------------------------------------------
e2e_step "write_path_checks" "Verify write-path refusals leave gates.toml BLAKE3 digest untouched"

e2e_cmd --name "register_write_path_tests" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance --test register -- write_path
e2e_assert_eq "write_path_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 5: Verify GATE-GROUND-10 import-lane resting states and due-set exclusion
# ---------------------------------------------------------------------------
e2e_step "import_lane_ground_10" "Verify GATE-GROUND-10 validates in blocked and no-claim states and enters no due set"

G10_BLOCKED="${SCRATCH_ROOT}/registries/g10_blocked.toml"
cat > "${G10_BLOCKED}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-GROUND-10"
kind = "convergence"
claim = "reality"
claim_id = "INV-GROUND-10"
section = "18.10"
threshold = "physical dataset validation"
milestone = "M3"
needs = "import-lane"
tier = ["G2:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M3"
status = "blocked"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "s2", implemented_by = "" }
detail = "OPEN-GROUND-1"
EOF

G10_NOCLAIM="${SCRATCH_ROOT}/registries/g10_no_claim.toml"
cat > "${G10_NOCLAIM}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-GROUND-10"
kind = "convergence"
claim = "reality"
claim_id = "INV-GROUND-10"
section = "18.10"
threshold = "physical dataset validation"
milestone = "M3"
needs = "import-lane"
tier = ["G2:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M3"
status = "no-claim"
measured = ""
evidence = []
recorded = { date = "2026-11-24", session = "s2", implemented_by = "" }
detail = "no-dataset-admitted"
EOF

e2e_cmd --name "g10_blocked_validate" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${G10_BLOCKED}" --repo-root "${SCRATCH_ROOT}" --controls "${CAUGHT_CONTROLS}" \
    --due-at M3 --json
e2e_assert_eq "g10_blocked_exit" "${E2E_LAST_EXIT}" "0"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "g10_blocked_ok" "/ok" "true"
  e2e_assert_json "g10_blocked_due_empty" "/due_at_count" "0"
fi

e2e_cmd --name "g10_noclaim_validate" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${G10_NOCLAIM}" --repo-root "${SCRATCH_ROOT}" --controls "${CAUGHT_CONTROLS}" \
    --due-at M3 --json
e2e_assert_eq "g10_noclaim_exit" "${E2E_LAST_EXIT}" "0"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "g10_noclaim_ok" "/ok" "true"
  e2e_assert_json "g10_noclaim_due_empty" "/due_at_count" "0"
fi

# ---------------------------------------------------------------------------
# Step 6: Optional frob capabilities probe (logs skip if binary absent)
# ---------------------------------------------------------------------------
e2e_step "frob_capabilities" "Probe frob capabilities --json when binary is present"

if command -v frob >/dev/null 2>&1; then
  e2e_cmd --name "frob_capabilities_json" -- frob capabilities --json
  e2e_assert_eq "frob_capabilities_exit" "${E2E_LAST_EXIT}" "0"
else
  e2e_skip "skipped: capability absent"
fi

e2e_finish
