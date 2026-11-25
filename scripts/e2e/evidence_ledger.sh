#!/usr/bin/env bash
# e2e: bead=N00-10 milestone=M0 labels=gauntlet,registry,tenfold seed=0 needs=cpu plants=refuted-strong-form,unlabelled-number,deferred-lever,h15-no-history
#
# End-to-end verification of the hypothesis verdict register and wording lint (bead N00-10):
#   1. Run `cargo test -p frob-conformance --test hypotheses`.
#   2. Print the state table and verify exact conversion counts (4 REFUTED, 1 DEFERRED,
#      11 ACTIVE, 16 NARROWED; 32 total; 8 open disagreements).
#   3. Run the wording lint over the real repository tree (assert 0 findings).
#   4. Run the wording lint over scratch clones carrying the three planted sentences
#      (refuted strong form, unlabelled surviving number, deferred lever) and run the
#      change-rule negative control (H-15 changed to ACTIVE with no history entry).
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "evidence_ledger" "N00-10"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
REG_FILE="${E2E_REPO_ROOT}/crates/frob-conformance/registry/hypotheses.toml"

# ---------------------------------------------------------------------------
# Step 1: Run `cargo test -p frob-conformance --test hypotheses`
# ---------------------------------------------------------------------------
e2e_step "unit_tests" "Run frob-conformance hypotheses unit and conformance tests"

e2e_cmd --name "hypotheses_unit_tests" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance --test hypotheses
e2e_assert_eq "hypotheses_unit_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 2: Load register, print state table, and verify exact conversion counts
# ---------------------------------------------------------------------------
e2e_step "state_table" "Load hypotheses.toml, print state table, and log state/disagreement counts"

e2e_cmd --name "print_state_table_json" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
    --register "${REG_FILE}" --print-table --json
e2e_assert_eq "state_table_exit" "${E2E_LAST_EXIT}" "0"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "register_ok" "/ok" "true"
  e2e_assert_json "register_schema" "/schema" "frob.hypothesis/1"
  e2e_assert_json "count_total" "/total" "32"
  e2e_assert_json "count_refuted" "/refuted" "4"
  e2e_assert_json "count_deferred" "/deferred" "1"
  e2e_assert_json "count_active" "/active" "11"
  e2e_assert_json "count_narrowed" "/narrowed" "16"
  e2e_assert_json "disagreements_open" "/disagreements_open" "8"

  e2e_metric "hypotheses_total" "32" "count" --observe
  e2e_metric "hypotheses_refuted" "4" "count" --observe
  e2e_metric "hypotheses_deferred" "1" "count" --observe
  e2e_metric "hypotheses_active" "11" "count" --observe
  e2e_metric "hypotheses_narrowed" "16" "count" --observe
  e2e_metric "disagreements_open" "8" "count" --observe
fi

# ---------------------------------------------------------------------------
# Step 3: Run wording lint over repository tree (and active-plant branches)
# ---------------------------------------------------------------------------
e2e_step "tree_wording_lint" "Run hypothesis wording lint over workspace docs and contracts"

if e2e_plant "refuted-strong-form"; then
  PLANT_DIR="${E2E_WORK_DIR}/plant_active_refuted"
  mkdir -p "${PLANT_DIR}/docs"
  printf 'Goal: the simulator uses half precision for contact solves.\n' > "${PLANT_DIR}/docs/planted.md"
  e2e_cmd --name "plant_active_refuted_lint" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
      --register "${REG_FILE}" --lint-tree "${PLANT_DIR}" --json
  e2e_assert_eq "refuted-strong-form:lint_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
elif e2e_plant "unlabelled-number"; then
  PLANT_DIR="${E2E_WORK_DIR}/plant_active_unlabelled"
  mkdir -p "${PLANT_DIR}/docs"
  printf 'H-04b cuts factor operation counts by 2.6x on contact solves.\n' > "${PLANT_DIR}/docs/planted.md"
  e2e_cmd --name "plant_active_unlabelled_lint" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
      --register "${REG_FILE}" --lint-tree "${PLANT_DIR}" --json
  e2e_assert_eq "unlabelled-number:lint_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
elif e2e_plant "deferred-lever"; then
  PLANT_DIR="${E2E_WORK_DIR}/plant_active_deferred"
  mkdir -p "${PLANT_DIR}/docs"
  printf 'Ten-fold levers: family batches and generative structure.\n' > "${PLANT_DIR}/docs/planted.md"
  e2e_cmd --name "plant_active_deferred_lint" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
      --register "${REG_FILE}" --lint-tree "${PLANT_DIR}" --json
  e2e_assert_eq "deferred-lever:lint_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
elif e2e_plant "h15-no-history"; then
  PLANT_REG="${E2E_WORK_DIR}/plant_active_h15.toml"
  awk '
    /^id = "H-15"/ { in_h15 = 1 }
    in_h15 && /^state = "REFUTED"/ { print "state = \"ACTIVE\""; in_h15 = 0; next }
    { print }
  ' "${REG_FILE}" > "${PLANT_REG}"
  e2e_cmd --name "plant_active_h15_change" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
      --check-change-rule "${REG_FILE}" "${PLANT_REG}" --json
  e2e_assert_eq "h15-no-history:change_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    exit 1
  fi
else
  e2e_cmd --name "repo_tree_lint" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
      --register "${REG_FILE}" --lint-tree "${E2E_REPO_ROOT}" --json
  e2e_assert_eq "repo_tree_lint_exit" "${E2E_LAST_EXIT}" "0"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "repo_tree_lint_ok" "/ok" "true"
    e2e_assert_json "repo_tree_lint_zero" "/finding_count" "0"
  fi
fi

# ---------------------------------------------------------------------------
# Step 4: Scratch-clone negative controls (3 planted sentences + H-15 change-rule plant)
# ---------------------------------------------------------------------------
e2e_step "planted_controls" "Assert scratch clones catch all 3 planted sentences and H-15 unlogged state change"

SCRATCH_CLONE="${E2E_WORK_DIR}/scratch_clone"
mkdir -p "${SCRATCH_CLONE}/docs"
cat > "${SCRATCH_CLONE}/docs/planted_claims.md" <<'EOF'
Goal: the simulator uses half precision for contact solves.
H-04b cuts factor operation counts by 2.6x across the step.
Ten-fold levers: family batches and generative structure.
EOF

e2e_cmd --name "scratch_clone_lint" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
    --register "${REG_FILE}" --lint-tree "${SCRATCH_CLONE}" --json
e2e_assert_eq "scratch_clone_lint_exit" "${E2E_LAST_EXIT}" "3"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "scratch_clone_not_ok" "/ok" "false"
  e2e_assert_json "scratch_clone_three_findings" "/finding_count" "3"
  e2e_assert_json "plant_1_rule" "/findings/0/rule" "hyp.banned-wording"
  e2e_assert_json "plant_1_row" "/findings/0/row_id" "H-15"
  e2e_assert_json "plant_2_rule" "/findings/1/rule" "hyp.unlabelled-number"
  e2e_assert_json "plant_2_row" "/findings/1/row_id" "H-04b"
  e2e_assert_json "plant_3_rule" "/findings/2/rule" "hyp.deferred-as-lever"
  e2e_assert_json "plant_3_row" "/findings/2/row_id" "H-12"
fi

PLANT_H15_REG="${E2E_WORK_DIR}/planted_h15_active.toml"
awk '
  /^id = "H-15"/ { in_h15 = 1 }
  in_h15 && /^state = "REFUTED"/ { print "state = \"ACTIVE\""; in_h15 = 0; next }
  { print }
' "${REG_FILE}" > "${PLANT_H15_REG}"

e2e_cmd --name "planted_h15_no_history" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example evidence_ledger -- \
    --check-change-rule "${REG_FILE}" "${PLANT_H15_REG}" --json
e2e_assert_eq "planted_h15_exit" "${E2E_LAST_EXIT}" "3"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "planted_h15_not_ok" "/ok" "false"
  e2e_metric "plants_caught" "4" "count" --observe
fi
