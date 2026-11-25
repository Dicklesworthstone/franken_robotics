#!/usr/bin/env bash
# e2e: bead=N32a-04 milestone=M0 labels=m0,tether,dispatch,solid seed=0 timeout_s=900 needs=cpu
#
# End-to-end verification of the Tether dispatch interface (bead N32a-04, ADR-TETHER-03):
#   1. Unit tests (`dispatch` and `rpc` modules) + dual-backend conformance suite under both
#      default (`handlers`) and `--no-default-features --features registry-only`.
#   2. Live server/client parity over a mode-0600 Unix domain socket via `frob_rpc_serve` and
#      `frob_rpc_call` across every entry of the synthetic dispatch fixture, diffing masked
#      envelopes byte-for-byte.
#   3. 20 mid-call client kill/drop rounds at varied byte offsets, confirming the server loop
#      continues answering cleanly afterwards.
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/tether_dispatch/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Compatible with bash 3.2 (macOS) and Linux.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "tether_dispatch" "N32a-04" "$@"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
TARGET_DEFAULT="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-tether-dispatch-default-${E2E_RUN_ID}"
TARGET_REGONLY="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-tether-dispatch-regonly-${E2E_RUN_ID}"
mkdir -p "${TARGET_DEFAULT}" "${TARGET_REGONLY}"

# ---------------------------------------------------------------------------
# Step 1: Unit tests and dual-backend conformance under default and registry-only
# ---------------------------------------------------------------------------
e2e_step "unit_and_conformance" "Run frob-verbs dispatch/rpc unit tests and conformance suite under default and registry-only"

e2e_cmd --name "cargo_test_dispatch_lib" -- \
  env "CARGO_TARGET_DIR=${TARGET_DEFAULT}" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-verbs --lib -- dispatch
e2e_assert_eq "dispatch_lib_tests_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "cargo_test_dispatch_conformance_default" -- \
  env "CARGO_TARGET_DIR=${TARGET_DEFAULT}" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-verbs --test dispatch_conformance
e2e_assert_eq "dispatch_conformance_default_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "cargo_test_dispatch_conformance_registry_only" -- \
  env "CARGO_TARGET_DIR=${TARGET_REGONLY}" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-verbs \
  --no-default-features --features registry-only --test dispatch_conformance
e2e_assert_eq "dispatch_conformance_registry_only_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 2: Live server/client parity and 20 mid-call drop rounds (when binaries materialize)
# ---------------------------------------------------------------------------
e2e_step "live_socket_parity_and_drops" "Exercise frob_rpc_serve and frob_rpc_call over a mode-0600 Unix socket"

e2e_cmd --name "build_rpc_examples" -- \
  env "CARGO_TARGET_DIR=${TARGET_DEFAULT}" \
  cargo build --manifest-path "${MANIFEST}" --offline -j 1 -p frob-verbs \
  --example frob_rpc_serve --example frob_rpc_call
e2e_assert_eq "build_rpc_examples_exit" "${E2E_LAST_EXIT}" "0"

SERVE_BIN="${TARGET_DEFAULT}/debug/examples/frob_rpc_serve"
CALL_BIN="${TARGET_DEFAULT}/debug/examples/frob_rpc_call"

if [ "${E2E_DRY_RUN:-0}" = "1" ] || [ ! -x "${SERVE_BIN}" ] || [ ! -x "${CALL_BIN}" ]; then
  e2e_skip "live_rpc_examples: frob_rpc_serve/frob_rpc_call binaries not materialized locally (remote rch target); unit & conformance tests verified parity and 100 client drops in-crate"
  exit 0
fi

# Short Unix socket path (< 104 bytes) in /tmp; never deleted (COMMON.md 9b).
SOCK_DIR="/tmp/frob-e2e-dispatch-$$-${E2E_RUN_ID}"
mkdir -p "${SOCK_DIR}"
chmod 700 "${SOCK_DIR}"
SOCK_PATH="${SOCK_DIR}/frobd.sock"
SERVE_OUT="${E2E_SCRATCH_DIR}/serve.stdout"
SERVE_ERR="${E2E_SCRATCH_DIR}/serve.stderr"
mkdir -p "${E2E_SCRATCH_DIR}"

"${SERVE_BIN}" "${SOCK_PATH}" >"${SERVE_OUT}" 2>"${SERVE_ERR}" &
SERVE_PID=$!
cleanup_server() {
  local rc=$?
  if kill -0 "${SERVE_PID}" >/dev/null 2>&1; then
    kill "${SERVE_PID}" >/dev/null 2>&1 || true
    wait "${SERVE_PID}" >/dev/null 2>&1 || true
  fi
  (exit "$rc")
  _e2e_finish
}
trap cleanup_server EXIT

READY="0"
for _ in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20; do
  if [ -s "${SERVE_OUT}" ] && grep -Fq "READY " "${SERVE_OUT}"; then
    READY="1"
    break
  fi
  sleep 0.1
done
e2e_assert_eq "rpc_server_ready" "${READY}" "1"

check_parity_case() {
  local case_id="$1" verb="$2" args_json="$3" want_exit="$4" use_daemon_for_inproc="$5"
  local inproc_file="${E2E_SCRATCH_DIR}/${case_id}.inproc.json"
  local sock_file="${E2E_SCRATCH_DIR}/${case_id}.socket.json"

  if [ "${use_daemon_for_inproc}" = "1" ]; then
    e2e_cmd --name "${case_id}_inproc" --expect-exit "${want_exit}" -- \
      "${CALL_BIN}" --backend in-process --socket "${SOCK_PATH}" --verb "${verb}" --args "${args_json}" --mask
  else
    e2e_cmd --name "${case_id}_inproc" --expect-exit "${want_exit}" -- \
      "${CALL_BIN}" --backend in-process --verb "${verb}" --args "${args_json}" --mask
  fi
  e2e_assert_eq "${case_id}_inproc_exit" "${E2E_LAST_EXIT}" "${want_exit}"
  cp "${E2E_LAST_STDOUT}" "${inproc_file}"

  e2e_cmd --name "${case_id}_socket" --expect-exit "${want_exit}" -- \
    "${CALL_BIN}" --backend socket --socket "${SOCK_PATH}" --verb "${verb}" --args "${args_json}" --mask
  e2e_assert_eq "${case_id}_socket_exit" "${E2E_LAST_EXIT}" "${want_exit}"
  cp "${E2E_LAST_STDOUT}" "${sock_file}"

  if cmp -s "${inproc_file}" "${sock_file}"; then
    e2e_assert_eq "${case_id}_masked_parity" "equal" "equal"
  else
    e2e_assert_eq "${case_id}_masked_parity" "differs" "equal"
  fi
}

check_parity_case "read_design_check" "design.check" '{"design":"quad-a"}' "0" "0"
check_parity_case "mutate_design_edit" "design.edit" '{"design":"quad-a","param":"knee.ratio","value":1.9}' "0" "0"
check_parity_case "run_cpu_sim_run" "sim.run" '{"design":"quad-a","duration_s":2.0,"worlds":4,"tier":"cpu"}' "0" "1"
check_parity_case "run_device_sim_gpu" "sim.gpu" '{"design":"quad-a","duration_s":1.5,"worlds":2,"tier":"device"}' "0" "1"
check_parity_case "forbidden_key" "export.packet" '{"artifact":"a:1","meta":{"released":true}}' "3" "0"
check_parity_case "unimpl_724" "unimpl.verb" '{}' "5" "0"
check_parity_case "refuse_exit3" "refuse.exit3" '{}' "3" "0"
check_parity_case "refuse_exit4" "refuse.exit4" '{}' "4" "0"
check_parity_case "refuse_exit5" "refuse.exit5" '{}' "5" "0"
check_parity_case "refuse_exit6" "refuse.exit6" '{}' "6" "0"
check_parity_case "refuse_exit9" "refuse.exit9" '{}' "9" "0"

# ---------------------------------------------------------------------------
# Step 3: 20 mid-call drop rounds + post-drop health check
# ---------------------------------------------------------------------------
e2e_step "mid_call_drops" "Drop 20 client connections at varied byte offsets and verify server stays healthy"

ROUND=0
for cut in 0 1 2 5 8 11 15 19 24 31 40 52 64 77 90 105 120 140 160 200; do
  ROUND=$((ROUND + 1))
  e2e_cmd --name "drop_round_${ROUND}_byte_${cut}" -- \
    "${CALL_BIN}" --drop-at-byte "${cut}" --socket "${SOCK_PATH}" --verb "design.check" --args '{"design":"quad-a"}'
  e2e_assert_eq "drop_round_${ROUND}_exit" "${E2E_LAST_EXIT}" "0"
done

e2e_cmd --name "post_drop_health_call" -- \
  "${CALL_BIN}" --backend socket --socket "${SOCK_PATH}" --verb "design.check" --args '{"design":"quad-a"}' --mask
e2e_assert_eq "post_drop_health_exit" "${E2E_LAST_EXIT}" "0"
e2e_metric "mid_call_drop_rounds" "${ROUND}" "rounds" ">=" "20"
