#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_unexpected_refusal" "STD-E2E"
e2e_step "refusal" "Negative control: unexpected FROB-REF-805 refusal outside e2e_expect_refusal must fail script"
e2e_assert_eq "setup_ok" "1" "1"
e2e_cmd --name unkeyed_call --expect-exit 0,3 -- bash -c 'printf "{\"schema\":\"frob.envelope/1\",\"verb\":\"sim.run\",\"ok\":false,\"exit\":3,\"refusal\":{\"code\":\"FROB-REF-805\",\"exit\":3,\"message\":\"missing idempotency key\",\"pointer\":\"/key\",\"observed\":null,\"required\":\"non-empty key\",\"unit\":\"\",\"retry\":true,\"repairs\":[{\"code\":\"ADD_KEY\",\"summary\":\"pass --key\"}]}}\n"; exit 3'
