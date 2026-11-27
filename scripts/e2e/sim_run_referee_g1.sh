#!/usr/bin/env bash
# e2e: bead=BD-024 milestone=M0 labels=chronicle,referee,verb:sim.run seed=1 timeout_s=180 needs=cpu
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"
e2e_init sim_run_referee_g1 BD-024 "$@"

if [ "${E2E_DRY_RUN:-0}" = "1" ] || [ ! -x "${FROB_BIN}" ]; then
  e2e_skip "A built frob is required for the G1 referee rollout"
  exit 77
fi
bin="$(cd "$(dirname "${FROB_BIN}")" && pwd)/$(basename "${FROB_BIN}")"
store="${E2E_SCRATCH_DIR}/ledger.db"

e2e_step original "Run the built-in G1 for two seconds and check the pinned oracle"
e2e_cmd --name original --expect-exit 0,2 -- env -i "FROB_STORE=${store}" \
  "${bin}" sim run g1 --tier referee --seconds 2 --key run-m0 --json
first="${E2E_LAST_STDOUT}"
e2e_assert_eq envelope_count "$(jq -rs length "${first}")" 1
e2e_assert_json fd_oracle /result/oracle_match equal
e2e_assert_json rollout_oracle /result/rollout_match equal
e2e_assert_json steps /result/steps 960
e2e_assert_json seed /evidence/seed 0
for flag in execution_authorized hardware_safety_signoff sim_to_real_certified; do
  e2e_assert_json "${flag}" "/result/${flag}" false
done
e2e_assert_digest_eq forward_digest "$(e2e_get /result/t0/forward_dynamics_digest)" \
  "$(e2e_get /result/oracle_fd_digest)"
e2e_assert_digest_eq rollout_digest "$(e2e_get /result/final_state_digest)" \
  "$(e2e_get /result/oracle_rollout_digest)"
e2e_assert_num com_drop "$(e2e_get /result/t0/com_drop_m)" lt 0 m
e2e_assert_num vertical_momentum "$(e2e_get /result/t0/vertical_momentum_kg_m_s)" lt 0 kg-m/s
e2e_assert_num wall_per_sim_second "$(e2e_get /result/wall_s_per_sim_s)" ge 0 s/s
run="$(e2e_get /result/run)"
trace="$(e2e_get /result/trace)"

e2e_step reproduce "Replay the recorded argv in a fresh environment with a fresh key"
argv=()
while IFS= read -r -d '' arg; do argv+=("${arg}"); done \
  < <(jq -j '.result.recorded_command[] | . + "\u0000"' "${first}")
e2e_assert_eq executable "${argv[0]}" frob
e2e_cmd --name reproduce --expect-exit 0,2 -- env -i "FROB_STORE=${store}" \
  "${bin}" "${argv[@]:1}" --key run-m0-reexec
e2e_assert_json original_run /result/run "${run}"
e2e_assert_json reproduced /result/reproduced true
e2e_assert_digest_eq identical_final "$(e2e_get /result/final_state_digest)" \
  "$(e2e_get --file "${first}" /result/final_state_digest)"
e2e_assert_eq identical_chunks "$(jq -cS '.result.chunks' "${E2E_LAST_STDOUT}")" \
  "$(jq -cS '.result.chunks' "${first}")"
reproduced_trace="$(e2e_get /result/trace)"
e2e_assert_eq fresh_trace "$([ "${reproduced_trace}" != "${trace}" ] && printf true || printf false)" true

e2e_step refusals "Check typed admission failures before writes"
e2e_expect_refusal FROB-REF-724 5 -- env -i "FROB_STORE=${store}" \
  "${bin}" sim run g1 --tier batch-cpu --key run-m0-tier --json
e2e_expect_refusal FROB-REF-802 3 -- env -i "FROB_STORE=${store}" \
  "${bin}" sim run g1 --tier refere --key run-m0-typo --json
e2e_assert_json typo_pointer /refusal/pointer /tier
e2e_expect_refusal FROB-REF-802 3 -- env -i "FROB_STORE=${store}" \
  "${bin}" sim run g1 --tier referee --seconds 0.01 --key run-m0-seconds --json
e2e_assert_json seconds_pointer /refusal/pointer /seconds

e2e_step ledger "Count the committed run, episode, and execution rows"
e2e_cmd --name ledger_rows -- python3 -c '
import json
import pathlib
import sqlite3
import sys

store, run, first, second = sys.argv[1:]
def key(table, value):
    return f"frob/v0/{table}/{value.encode().hex()}"

with sqlite3.connect(pathlib.Path(store).absolute().as_uri() + "?mode=ro", uri=True) as db:
    def count(table, kind):
        return db.execute(f"select count(*) from {table} where name like ?", (f"frob/v0/{kind}/%",)).fetchone()[0]
    def has(table, name):
        return db.execute(f"select count(*) from {table} where name = ?", (name,)).fetchone()[0] == 1
    print(json.dumps({
        "run": count("scenarios", "run"),
        "episode": count("constraints", "episode"),
        "execution": count("scenarios", "execution"),
        "keyed_rows": all((
            has("scenarios", key("run", run)), has("constraints", key("episode", run + "/0/0")),
            has("scenarios", key("execution", first)), has("scenarios", key("execution", second)),
        )),
    }))
' "${store}" "${run}" "${trace}" "${reproduced_trace}"
e2e_assert_json run_rows /run 1
e2e_assert_json episode_rows /episode 1
e2e_assert_json execution_rows /execution 2
e2e_assert_json expected_keys /keyed_rows true

e2e_step perturb "Run the conformance-only one-ULP control"
if [ -n "${FROB_BIN_CONFORMANCE:-}" ] && [ -x "${FROB_BIN_CONFORMANCE}" ]; then
  e2e_cmd --name perturb --expect-exit 2 -- env -i "FROB_STORE=${store}" \
    "${FROB_BIN_CONFORMANCE}" sim run g1 --tier referee --seconds 2 --key run-m0-perturb --perturb-ulp q0 --json
  e2e_assert_json perturbed_fd /result/oracle_match different
  e2e_assert_json perturbed_rollout /result/rollout_match different
  e2e_assert_eq distinct_perturbed_run "$([ "$(e2e_get /result/run)" != "${run}" ] && printf true || printf false)" true
  e2e_assert_eq distinct_perturbed_digest "$([ "$(e2e_get /result/final_state_digest)" != "$(e2e_get --file "${first}" /result/final_state_digest)" ] && printf true || printf false)" true
else
  e2e_skip "Set FROB_BIN_CONFORMANCE to the centrally built one-ULP control binary"
fi
e2e_skip "BD-023 supplies the MCP stdio parity call and the third execution row"
