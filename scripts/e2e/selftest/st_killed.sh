#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_killed" "STD-E2E"
e2e_step "abrupt" "Negative control: killed by SIGKILL before summary is written"
e2e_assert_eq "before_kill" "1" "1"
kill -9 "$$"
