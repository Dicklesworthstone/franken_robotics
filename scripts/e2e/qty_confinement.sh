#!/usr/bin/env bash
# e2e: bead=N33c-03 milestone=M0 labels=suite,marrow plants=fs-qty-direct,qty-generated-byte seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of frob_core::qty closed alias set, compile-fail pair
# tests, alias-count gate (<= 64), xtask check-generated row, and L-2.fs-qty-direct
# confinement (bead N33c-03).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/qty_confinement/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "qty_confinement" "N33c-03" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-qty-confinement-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "qty_unit_and_compile_fail" "Run frob-core qty registry, compile-fail, and doc tests"

e2e_metric "alias_count" "36" "aliases"
e2e_metric "equal_dimension_pairs" "9" "pairs"

e2e_cmd --name "cargo_test_frob_core_qty" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core --test qty
e2e_assert_eq "cargo_test_frob_core_qty_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "cargo_test_frob_core_qty_doc" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core --doc qty
e2e_assert_eq "cargo_test_frob_core_qty_doc_exit" "${E2E_LAST_EXIT}" "0"

e2e_step "xtask_qty_confinement_and_generated" "Run xtask qty_confinement and check_generated suites (including planted L-2.fs-qty-direct and gen.differs controls)"

e2e_cmd --name "cargo_test_xtask_qty" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo test --offline -j 1 -p xtask --test qty_confinement --test check_generated
e2e_assert_eq "cargo_test_xtask_qty_exit" "${E2E_LAST_EXIT}" "0"

fs_qty_caught="no"
if printf '%s\n' "${E2E_LAST_STDOUT}" | grep -q "check_deps_rejects_fs_qty_outside_allowed ... ok"; then
  fs_qty_caught="yes"
fi
e2e_assert_eq "planted_fs_qty_import_caught" "${fs_qty_caught}" "yes"

qty_byte_caught="no"
if printf '%s\n' "${E2E_LAST_STDOUT}" | grep -q "qty_generator_row ... ok"; then
  qty_byte_caught="yes"
fi
e2e_assert_eq "planted_qty_generated_byte_caught" "${qty_byte_caught}" "yes"

e2e_cmd --name "xtask_check_generated_real" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo run --offline -j 1 -p xtask -- check-generated --json
e2e_assert_eq "xtask_check_generated_real_exit" "${E2E_LAST_EXIT}" "0"
