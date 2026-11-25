#!/usr/bin/env bash
# e2e: bead=VERBS-sim milestone=M0 labels=tether,verbs,verb:sim.estimate,verb:sim.replay,verb:sim.run gates=GATE-TETHER-02 seed=1 timeout_s=120 needs=cpu
# CLI stages. MCP parity, schema snapshots and gate qualification need NJL-02 and BD-023.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"
e2e_init verbs_sim VERBS-sim "$@"

if [ "${E2E_DRY_RUN:-0}" = "1" ] || [ ! -x "${FROB_BIN}" ]; then
  e2e_skip "A built frob is required to read the registry examples"
  exit 77
fi

for verb in estimate replay run; do
  case "${verb}" in
    estimate) kind="read"; milestone=M2 ;;
    replay) kind="run"; milestone=M2 ;;
    run) kind="run"; milestone=M0 ;;
  esac
  name="sim.${verb}"
  e2e_step "${verb}" "Check ${name} in the CLI and run its registry example"
  e2e_cmd --name "${verb}_capability" -- "${FROB_BIN}" capabilities --verb "${name}" --json
  e2e_assert_json "${verb}_capability_name" /result/capabilities/0/verb "${name}"
  e2e_assert_json "${verb}_kind" /result/capabilities/0/kind "${kind}"
  e2e_assert_json "${verb}_milestone" /result/capabilities/0/first_milestone "${milestone}"
  linked="$(e2e_get /result/capabilities/0/linked)"

  e2e_cmd --name "${verb}_help" -- "${FROB_BIN}" sim "${verb}" --help --json
  e2e_assert_json "${verb}_help_entry" /result/entry/name "${name}"
  example="$(e2e_get /result/examples/0)"
  # Registry examples in this cluster are unquoted words. Pass them literally without shell expansion.
  IFS=' ' read -r -a argv <<< "${example}"
  e2e_assert_eq "${verb}_example_path" "${argv[0]} ${argv[1]} ${argv[2]}" "frob sim ${verb}"
  if [ "${linked}" = "true" ]; then
    # Verbatim examples select the default synchronous wait. Exit 8 needs explicit --sync-wait-ms.
    e2e_cmd --name "${verb}_example" --expect-exit 0,2 -- "${FROB_BIN}" "${argv[@]:1}"
    e2e_assert_json "${verb}_example_completed" /ok true
  else
    e2e_assert_eq "${verb}_unlinked" "${linked}" false
    e2e_expect_refusal FROB-REF-724 5 -- "${FROB_BIN}" "${argv[@]:1}"
  fi
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" "${argv[@]:1}" --misspelt-parameter
done

e2e_step release_inputs "Check a conformance-only flag on the release binary"
if [ -n "${FROB_BIN_RELEASE:-}" ] && [ -x "${FROB_BIN_RELEASE}" ]; then
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN_RELEASE}" sim run g1 --tier referee --key run-release --json --perturb-ulp q0
else
  e2e_skip "Set FROB_BIN_RELEASE to the centrally built release CLI to check feature isolation"
fi
e2e_skip "NJL-02 and BD-023 supply snapshot checks and MCP parity"
