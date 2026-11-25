#!/usr/bin/env bash
# e2e: bead=BD-022 milestone=M0 labels=m0,tether,frob-cli,solid,operator gates=GATE-TETHER-03 seed=0 timeout_s=120 needs=cpu
# Requires FROB_BIN to point to the centrally built frob-cli binary.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"
e2e_init frob_operator_verbs BD-022 "$@"

e2e_step stack_identity "Doctor reports both embedded stack identities"
e2e_cmd --name doctor -- "${FROB_BIN}" doctor --section stack_receipt --json
e2e_assert_eq doctor_exit "${E2E_LAST_EXIT}" 0
e2e_assert_json doctor_receipt /result/stack_receipt/receipt "sr:$(cat "${E2E_REPO_ROOT}/receipts/HEAD")"
e2e_assert_json doctor_lock /result/stack_receipt/lock "sl:$(cat "${E2E_REPO_ROOT}/receipts/LOCK_DIGEST")"

e2e_step operator_catalogue "Exercise capabilities, schema, help and generated documentation"
e2e_cmd --name capabilities -- "${FROB_BIN}" capabilities --verb doctor --json
e2e_assert_eq capabilities_exit "${E2E_LAST_EXIT}" 0
e2e_assert_json linked_status /result/capabilities/0/status partial
e2e_cmd --name doctor_schema -- "${FROB_BIN}" schema doctor --out --json
e2e_assert_eq doctor_schema_exit "${E2E_LAST_EXIT}" 0
e2e_assert_json doctor_schema_id "/result/schema/\$id" frob.doctor.out/1
e2e_cmd --name doctor_help -- "${FROB_BIN}" doctor --help --json
e2e_assert_eq doctor_help_exit "${E2E_LAST_EXIT}" 0
e2e_assert_json doctor_help_entry /result/entry/name doctor
e2e_cmd --name robot_docs -- "${FROB_BIN}" robot-docs --json
e2e_assert_eq robot_docs_exit "${E2E_LAST_EXIT}" 0
e2e_assert_json surface_claim /result/surface_claim \
  "every verb carries a compute budget, provenance, and a typed refusal, and both CLI and MCP surfaces are generated from one registry."

e2e_step parser_refusals "Malformed and unknown invocations produce typed refusals"
e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" doctor --sektion stack --json
e2e_expect_refusal FROB-REF-801 3 -- "${FROB_BIN}" desgin check --json
e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" schema --refusals --verbs --json
e2e_skip "GATE-TETHER-03 qualification pending" "budget overlay and registered negative controls are pending"
