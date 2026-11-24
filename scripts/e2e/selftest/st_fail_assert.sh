#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_fail_assert" "STD-E2E"
e2e_step "known_answer" "Two passing assertions and one failing assertion (known answer: 2 passed, 1 failed)"
e2e_assert_eq "pass_one" "alpha" "alpha"
e2e_assert_num "pass_two" "0.30" "==" "0.3" "s"
e2e_assert_eq "fail_three" "observed-value" "expected-value"
