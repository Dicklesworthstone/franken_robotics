#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_self_record" "STD-E2E"
e2e_step "self_record" "Negative control: logging record_gate command must fail lint as self-recording"
mkdir -p "${E2E_WORK_DIR}/fakebin"
cat > "${E2E_WORK_DIR}/fakebin/cargo" <<'EOF'
#!/usr/bin/env bash
printf 'recorded\n'
exit 0
EOF
chmod +x "${E2E_WORK_DIR}/fakebin/cargo"
PATH="${E2E_WORK_DIR}/fakebin:${PATH}" e2e_cmd --name record_gate_call -- cargo run -p frob-conformance --example record_gate -- --gate GATE-TEST-01 --receipt 00000000
e2e_assert_eq "cmd_exited_zero" "${E2E_LAST_EXIT}" "0"
