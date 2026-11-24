#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_key_reuse" "STD-E2E"
e2e_step "reuse" "Negative control: issuing one key label twice without --again must exit 2"
k1="$(e2e_key "dup")"
e2e_assert_eq "first_key" "$k1" "e2e-st_key_reuse-dup"
k2="$(e2e_key "dup")"
e2e_assert_eq "should_not_reach" "$k2" ""
