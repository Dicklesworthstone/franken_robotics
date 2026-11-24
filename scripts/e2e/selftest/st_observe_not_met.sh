#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=gauntlet,harness seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_observe_not_met" "STD-E2E"
e2e_step "observe" "Convention 15 / review note 5: --observe metric with not-met threshold is data, not a failure"
e2e_assert_eq "measurement_completed" "ok" "ok"
e2e_metric "pilot_falsifier_ratio" "0.42" "ratio" ">=" "0.80" --observe
