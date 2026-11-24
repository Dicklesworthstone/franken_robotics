#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu plants=boundary
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_plant" "STD-E2E"
e2e_step "plant_check" "Passes unplanted; fails naming 'boundary' when E2E_PLANT_FAILURE=boundary"
if e2e_plant "boundary"; then
  e2e_assert_eq "plant:boundary" "planted-defect" "clean-boundary"
else
  e2e_assert_eq "plant:boundary" "clean-boundary" "clean-boundary"
fi
