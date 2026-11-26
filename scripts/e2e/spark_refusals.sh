#!/usr/bin/env bash
# e2e: bead=N21-REF milestone=M0 labels=m0,spark,frob-core,solid,refusals gates=GATE-SPARK-00 seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of the Spark refusal registry FROB-REF-601..618
# (bead N21-REF). Runs the registry suite and the live no-daemon dispatch path.
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/spark_refusals/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

BACKEND=""
FILTERED_ARGS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --backend)
      if [ "$#" -lt 2 ]; then
        printf '%s\n' '--backend requires gpu-cuda or gpu-metal' >&2
        exit 2
      fi
      shift
      BACKEND="$1"
      ;;
    --backend=*)
      BACKEND="${1#--backend=}"
      ;;
    *)
      FILTERED_ARGS+=("$1")
      ;;
  esac
  shift
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

if [ "${#FILTERED_ARGS[@]}" -gt 0 ]; then
  e2e_init "spark_refusals" "N21-REF" "${FILTERED_ARGS[@]}"
else
  e2e_init "spark_refusals" "N21-REF"
fi

case "${BACKEND}" in
  ""|batch-cpu|gpu-cuda|gpu-metal) ;;
  *) _e2e_harness_error "unknown Spark backend: ${BACKEND}" ;;
esac

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"

# Cargo and its offload shim discover toolchain and config from their cwd.
_in_workspace() {
  local work_dir="$PWD" status=0
  cd "${E2E_REPO_ROOT}" || return
  "$@" || status=$?
  cd "${work_dir}" || return
  return "${status}"
}

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-spark-refusals-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "registry_suite" "Verify FROB-REF-601..618 registry rows, exit code 6, repairs, and code_gen parity"

REG_FILE="${E2E_REPO_ROOT}/registries/refusals.toml"
_in_workspace e2e_cmd --name "cargo_test_frob_core_spark_refusals" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-core --test refusal_registry_spark --test refusal_registry_numerics
e2e_assert_eq "cargo_test_frob_core_spark_refusals_exit" "${E2E_LAST_EXIT}" "0"

e2e_step "refusal_601_no_daemon" "Submit a device-tier run to the in-process dispatcher without a daemon"
# The example supplies a fixture verb and executes the real dispatch path.
_in_workspace e2e_expect_refusal FROB-REF-601 6 -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/dispatch" \
  cargo run --manifest-path "${MANIFEST}" --offline -j 1 -p frob-verbs --example frob_rpc_call -- \
    --backend in-process --verb sim.gpu --args '{"design":"quad-a","duration_s":1.0,"tier":"device"}'
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_eq "refusal_601_exit" "${E2E_LAST_EXIT}" "6"
  e2e_assert_json "refusal_601_code" /refusal/code FROB-REF-601
  e2e_assert_json "refusal_601_pointer" /refusal/pointer daemon
  e2e_assert_json "refusal_601_first_repair" /refusal/repairs/0/action 'frobd --state-root ~/.frob'
fi

e2e_step "refusal_606_receipt" "Check the missing-fingerprint refusal at the receipt boundary"
_in_workspace e2e_cmd --name "cargo_test_missing_fingerprint" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-kernels" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-kernels --test backend -- receipt_missing_field_rejected --exact
e2e_assert_eq "refusal_606_receipt_test_exit" "${E2E_LAST_EXIT}" "0"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  receipt_test_count="$(awk '/^test receipt_missing_field_rejected \.\.\. ok$/ { count++ } END { print count+0 }' "${E2E_LAST_STDOUT}")"
  e2e_assert_eq "refusal_606_receipt_test_count" "${receipt_test_count}" "1"
fi

e2e_step "pending_device_provocations" "Report device raising paths absent from this build"
# The current registry stores milestones in the meaning field.
pending_rows="$(awk '
  /^code = "FROB-REF-6[0-9][0-9]"$/ { code = substr($0, 9, 12) }
  /^code = / && $0 !~ /^code = "FROB-REF-6[0-9][0-9]"$/ { code = "" }
  /^meaning = / && code != "" {
    if (!match($0, /\(due M[0-9]+(\/M[0-9]+)?\)"$/)) exit 1
    milestone = substr($0, RSTART, RLENGTH)
    sub(/^\(due /, "", milestone)
    sub(/\)"$/, "", milestone)
    print code, milestone
    code = ""
  }
' "${REG_FILE}")" || _e2e_harness_error "Spark refusal registry row lacks a due milestone"
while read -r code milestone; do
  case "${code}" in
    FROB-REF-601) capability="device open/loss";;
    FROB-REF-603) capability="device compiler CLI";;
    FROB-REF-606) capability="device admission with fingerprint currency";;
    FROB-REF-602|FROB-REF-60[45789]|FROB-REF-61[0-8]) capability="device raising path";;
    *) _e2e_harness_error "unhandled Spark refusal: ${code}";;
  esac
  e2e_skip "capability absent: ${code} (${milestone}), ${capability}${BACKEND:+ for ${BACKEND}}"
done <<< "${pending_rows}"
