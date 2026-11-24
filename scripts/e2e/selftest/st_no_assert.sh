#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_no_assert" "STD-E2E"
e2e_step "empty" "Negative control: zero assertions must yield harness exit 2"
e2e_cmd --name noop -- true
