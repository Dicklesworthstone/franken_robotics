#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_private_leak" "STD-E2E"
e2e_step "leak" "Negative control: log private canary; script exits 0 and linter must fail"
e2e_assert_eq "canary_echo" "FROB-PRIVATE-CANARY-7f3a" "FROB-PRIVATE-CANARY-7f3a"
