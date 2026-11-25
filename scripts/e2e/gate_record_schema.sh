#!/usr/bin/env bash
# e2e: bead=N38b-18 milestone=M0 labels=gauntlet,registry gates=GATE-GAUNT-10 seed=0 needs=cpu plants=bad-claim,bad-status
#
# End-to-end verification of registries/gates.toml base record schema (bead N38b-18):
#   1. Run `cargo test -p frob-conformance register::base`.
#   2. Validate committed registries/gates.toml against registry/gate_counts.toml
#      via `gate_base_check` (roundtrip + D0 counts), logging metric lines per status
#      and per claim.
#   3. Validate planted invalid files (bad claim, doubled claim, bad status, extra gate)
#      and assert each fails closed naming the gate.
#   4. Verify schemas/registries/gates.schema.json parses as valid JSON and records
#      the closed claim/status sets and four pass-refusal validator contract clauses.
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "gate_record_schema" "N38b-18"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
GATES_FILE="${E2E_REPO_ROOT}/registries/gates.toml"
COUNTS_FILE="${E2E_REPO_ROOT}/crates/frob-conformance/registry/gate_counts.toml"
SCHEMA_FILE="${E2E_REPO_ROOT}/schemas/registries/gates.schema.json"

# ---------------------------------------------------------------------------
# Step 1: Unit tests (`register::base`)
# ---------------------------------------------------------------------------
e2e_step "unit_tests" "Run frob-conformance register::base unit tests"

e2e_cmd --name "register_base_unit_tests" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance register::base
e2e_assert_eq "register_base_unit_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"

# ---------------------------------------------------------------------------
# Step 2: Validate committed registries/gates.toml (+ plant hooks)
# ---------------------------------------------------------------------------
e2e_step "committed_gates" "Validate committed registries/gates.toml, count expectations, and log per-status/per-claim metrics"

TARGET_GATES="${GATES_FILE}"
if e2e_plant "bad-claim"; then
  TARGET_GATES="${E2E_WORK_DIR}/planted_bad_claim.toml"
  sed 's/claim = "known-answer"/claim = "Performance of the method"/' "${GATES_FILE}" > "${TARGET_GATES}"
  e2e_cmd --name "gate_base_check_planted_bad_claim" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
      --file "${TARGET_GATES}" --counts "${COUNTS_FILE}" --roundtrip --json
  e2e_assert_eq "bad-claim:committed_gates_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
elif e2e_plant "bad-status"; then
  TARGET_GATES="${E2E_WORK_DIR}/planted_bad_status.toml"
  sed 's/status = "not-run"/status = "passed"/' "${GATES_FILE}" > "${TARGET_GATES}"
  e2e_cmd --name "gate_base_check_planted_bad_status" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
      --file "${TARGET_GATES}" --counts "${COUNTS_FILE}" --roundtrip --json
  e2e_assert_eq "bad-status:committed_gates_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
else
  e2e_cmd --name "gate_base_check_committed" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
      --file "${TARGET_GATES}" --counts "${COUNTS_FILE}" --roundtrip --json
  e2e_assert_eq "committed_gates_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
fi

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  _COMMITTED_OUT="${E2E_LAST_STDOUT}"
  e2e_assert_json "committed_ok" "/ok" "true" --gate "GATE-GAUNT-10"
  e2e_assert_json "committed_counts_ok" "/counts_ok" "true" --gate "GATE-GAUNT-10"
  e2e_assert_json "committed_schema" "/schema" "frob.gates/1" --gate "GATE-GAUNT-10"
  e2e_assert_json "plan_inventory_total" "/plan_inventory_total" "421" --gate "GATE-GAUNT-10"

  for st in not-run pass fail killed blocked no-claim superseded; do
    _VAL="$(e2e_get --file "${_COMMITTED_OUT}" "/by_status/${st}")"
    _KEY="$(printf '%s' "${st}" | tr '-' '_')"
    e2e_metric "gates_status_${_KEY}" "${_VAL}" "count" --observe --gate "GATE-GAUNT-10"
  done

  for cl in proof known-answer parity fidelity design-comparison reality performance; do
    _VAL="$(e2e_get --file "${_COMMITTED_OUT}" "/by_claim/${cl}")"
    _KEY="$(printf '%s' "${cl}" | tr '-' '_')"
    e2e_metric "gates_claim_${_KEY}" "${_VAL}" "count" --observe --gate "GATE-GAUNT-10"
  done
fi

# ---------------------------------------------------------------------------
# Step 3: Validate planted invalid files (bad claim, doubled claim, bad status, extra row)
# ---------------------------------------------------------------------------
e2e_step "planted_refusals" "Assert planted invalid claim, status, and extra-row gate files are rejected"

PLANT_BAD_CLAIM="${E2E_WORK_DIR}/plant_bad_claim.toml"
cat > "${PLANT_BAD_CLAIM}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-PLANT-CLAIM"
claim = "Performance of the method"
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000001"
section = "38.13"
threshold = "planted"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
EOF

e2e_cmd --name "plant_bad_claim" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
    --file "${PLANT_BAD_CLAIM}" --json
e2e_assert_eq "plant_bad_claim_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "plant_bad_claim_ok" "/ok" "false" --gate "GATE-GAUNT-10"
  e2e_assert_json "plant_bad_claim_gate" "/gate_id" "GATE-PLANT-CLAIM" --gate "GATE-GAUNT-10"
fi

PLANT_TWO_CLAIMS="${E2E_WORK_DIR}/plant_two_claims.toml"
cat > "${PLANT_TWO_CLAIMS}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-PLANT-TWOCLAIM"
claim = ["parity", "performance"]
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000002"
section = "38.13"
threshold = "planted"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
EOF

e2e_cmd --name "plant_two_claims" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
    --file "${PLANT_TWO_CLAIMS}" --json
e2e_assert_eq "plant_two_claims_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "plant_two_claims_ok" "/ok" "false" --gate "GATE-GAUNT-10"
  e2e_assert_json "plant_two_claims_gate" "/gate_id" "GATE-PLANT-TWOCLAIM" --gate "GATE-GAUNT-10"
fi

PLANT_BAD_STATUS="${E2E_WORK_DIR}/plant_bad_status.toml"
cat > "${PLANT_BAD_STATUS}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-PLANT-STATUS"
claim = "known-answer"
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000003"
section = "38.13"
threshold = "planted"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "passed"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
EOF

e2e_cmd --name "plant_bad_status" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
    --file "${PLANT_BAD_STATUS}" --json
e2e_assert_eq "plant_bad_status_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "plant_bad_status_ok" "/ok" "false" --gate "GATE-GAUNT-10"
fi

PLANT_EXTRA_ROW="${E2E_WORK_DIR}/plant_extra_row.toml"
cp "${GATES_FILE}" "${PLANT_EXTRA_ROW}"
cat >> "${PLANT_EXTRA_ROW}" <<'EOF'

[[gate]]
id = "GATE-PLANT-EXTRA"
claim = "known-answer"
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000099"
section = "38.13"
threshold = "planted extra row"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
EOF

e2e_cmd --name "plant_extra_row" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example gate_base_check -- \
    --file "${PLANT_EXTRA_ROW}" --counts "${COUNTS_FILE}" --json
e2e_assert_eq "plant_extra_row_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "plant_extra_row_ok" "/ok" "false" --gate "GATE-GAUNT-10"
  e2e_assert_json "plant_extra_row_gate" "/gate_id" "GATE-PLANT-EXTRA" --gate "GATE-GAUNT-10"
fi

# ---------------------------------------------------------------------------
# Step 4: Schema contract verification
# ---------------------------------------------------------------------------
e2e_step "schema_contract" "Verify schemas/registries/gates.schema.json parses cleanly and declares base contract"

e2e_assert_file "gates_schema_exists" "${SCHEMA_FILE}" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  _SCHEMA_CONST="$(e2e_get --file "${SCHEMA_FILE}" "/properties/schema/const")"
  e2e_assert_eq "gates_schema_const" "${_SCHEMA_CONST}" "frob.gates/1" --gate "GATE-GAUNT-10"
fi
