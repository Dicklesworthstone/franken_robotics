#!/usr/bin/env bash
# scripts/bootstrap/tests/run_all.sh
# Runs all seven BD-001 bootstrap receipt unit tests on synthetic git trees under
# ${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-bd001-tests/.

set -euo pipefail

TESTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TESTS=(
  receipt_is_canonical.sh
  tree_hash_matches_definition.sh
  drift_reason_recorded.sh
  receipt_twice_identical.sh
  receipt_has_no_absolute_path.sh
  lock_parses_and_names_T.sh
  drift_class_table.sh
  receipt_regressions.sh
)

PASSED=0
FAILED=0

for t in "${TESTS[@]}"; do
  if bash "${TESTS_DIR}/${t}"; then
    PASSED=$((PASSED + 1))
  else
    FAILED=$((FAILED + 1))
  fi
done

printf 'Summary: %d passed, %d failed (total %d)\n' "${PASSED}" "${FAILED}" "${#TESTS[@]}"
if [ "${FAILED}" -ne 0 ]; then
  exit 1
fi
