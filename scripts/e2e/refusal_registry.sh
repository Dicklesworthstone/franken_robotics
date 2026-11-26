#!/usr/bin/env bash
# e2e: bead=N35-33 milestone=M0 labels=m0,frob-core,registry,solid,tether,spark seed=0 timeout_s=600 needs=cpu plants=hand-edited-printout
#
# End-to-end verification of the refusal allocation ledger, cross-chapter checks
# 1..6, and generated docs/generated/refusals.md printout (bead N35-33).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/refusal_registry/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "refusal_registry" "N35-33" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-refusal-registry-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "stage1_tests_and_generated" "Run frob-core refusal_* tests and xtask check-generated"

e2e_cmd --name "cargo_test_frob_core_refusals" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core refusal_
e2e_assert_eq "cargo_test_frob_core_refusals_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "xtask_check_generated" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo run --offline -j 1 -p xtask -- check-generated --root "${E2E_REPO_ROOT}" --json
e2e_assert_eq "xtask_check_generated_exit" "${E2E_LAST_EXIT}" "0"
e2e_assert_file "docs_generated_refusals_exists" "${E2E_REPO_ROOT}/docs/generated/refusals.md"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  live_rows="$(grep -c '^code = "FROB-REF-' "${E2E_REPO_ROOT}/registries/refusals.toml" || true)"
  assigned_rows="$(grep -c '^status = "assigned"$' "${E2E_REPO_ROOT}/registries/refusal_allocations.toml" || true)"
  merged_rows="$(grep -c '^status = "merged"$' "${E2E_REPO_ROOT}/registries/refusal_allocations.toml" || true)"
  unsettled_rows="$(grep -c '^status = "requested"$' "${E2E_REPO_ROOT}/registries/refusal_allocations.toml" || true)"
  e2e_metric "registry_rows_present" "${live_rows}" "rows"
  e2e_metric "printed_rows_target" "294" "rows"
  e2e_metric "allocations_assigned" "${assigned_rows}" "rows"
  e2e_metric "allocations_merged" "${merged_rows}" "rows"
  e2e_metric "interim_unsettled" "${unsettled_rows}" "rows"
fi

if e2e_plant "hand-edited-printout"; then
  e2e_step "plant_hand_edited_printout" "Verify xtask check-generated catches a hand-edited docs/generated/refusals.md in a scratch copy"
  SCRATCH_REPO="${E2E_WORK_DIR}/scratch-printout"
  mkdir -p "${SCRATCH_REPO}/registries" "${SCRATCH_REPO}/docs/generated" "${SCRATCH_REPO}/crates/frob-core/src/refusal"
  cp "${E2E_REPO_ROOT}/registries/refusals.toml" "${SCRATCH_REPO}/registries/refusals.toml"
  cp "${E2E_REPO_ROOT}/registries/refusal_allocations.toml" "${SCRATCH_REPO}/registries/refusal_allocations.toml"
  cp "${E2E_REPO_ROOT}/registries/formats.toml" "${SCRATCH_REPO}/registries/formats.toml"
  cp "${E2E_REPO_ROOT}/registries/verbs.toml" "${SCRATCH_REPO}/registries/verbs.toml"
  cp "${E2E_REPO_ROOT}/registries/gates.toml" "${SCRATCH_REPO}/registries/gates.toml"
  cp "${E2E_REPO_ROOT}/registries/kernels.toml" "${SCRATCH_REPO}/registries/kernels.toml"
  cp "${E2E_REPO_ROOT}/crates/frob-core/src/refusal/code_gen.rs" "${SCRATCH_REPO}/crates/frob-core/src/refusal/code_gen.rs"
  cp "${E2E_REPO_ROOT}/docs/generated/refusals.md" "${SCRATCH_REPO}/docs/generated/refusals.md"
  printf '\n<!-- planted hand edit -->\n' >> "${SCRATCH_REPO}/docs/generated/refusals.md"
  e2e_cmd --name "xtask_check_generated_planted" --expect-exit 2 -- \
    env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
    cargo run --offline -j 1 -p xtask -- check-generated --root "${SCRATCH_REPO}" --json
  e2e_assert_eq "plant_hand_edited_printout_caught" "${E2E_LAST_EXIT}" "2"
fi

e2e_step "stage2_to_4_cli_surface" "Check frob --json schema, three-family provocations, and schema --refusals interims when frob binary exists"

if [ -n "${FROB_BIN:-}" ] && [ -x "${FROB_BIN}" ]; then
  e2e_cmd --name "frob_json_schema" -- "${FROB_BIN}" --json schema
  e2e_assert_eq "frob_json_schema_exit" "${E2E_LAST_EXIT}" "0"

  e2e_skip "capability absent: stage 3 three-family CLI provocation (761 and two sibling families) awaiting BD-022/N35-REF handlers"

  e2e_cmd --name "frob_schema_refusals" -- "${FROB_BIN}" schema --refusals --json
  e2e_assert_eq "frob_schema_refusals_exit" "${E2E_LAST_EXIT}" "0"
else
  e2e_skip "binary absent: stage 2 frob --json schema (delivered by BD-022)"
  e2e_skip "binary absent: stage 3 three-family refusal provocations (delivered by BD-022)"
  e2e_skip "binary absent: stage 4 frob schema --refusals --json interims (delivered by BD-022)"
fi
