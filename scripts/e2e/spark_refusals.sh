#!/usr/bin/env bash
# e2e: bead=N21-REF milestone=M0 labels=m0,spark,frob-core,solid,refusals gates=GATE-SPARK-00 seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of the Spark refusal registry FROB-REF-601..618
# (bead N21-REF). Runs the frob-core Spark registry unit suite and logs
# staged milestone status for all eighteen 6xx codes plus device-count-wrap.
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/spark_refusals/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

BACKEND="batch-cpu"
FILTERED_ARGS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --backend)
      shift
      BACKEND="${1:-batch-cpu}"
      ;;
    --backend=*)
      BACKEND="${1#--backend=}"
      ;;
    *)
      FILTERED_ARGS+=("$1")
      ;;
  esac
  shift || true
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

if [ "${#FILTERED_ARGS[@]}" -gt 0 ]; then
  e2e_init "spark_refusals" "N21-REF" "${FILTERED_ARGS[@]}"
else
  e2e_init "spark_refusals" "N21-REF"
fi

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-spark-refusals-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "registry_suite" "Verify FROB-REF-601..618 registry rows, exit code 6, repairs, and code_gen parity"

REG_FILE="${E2E_REPO_ROOT}/registries/refusals.toml"
e2e_assert_eq "refusals_registry_exists" "$(test -f "${REG_FILE}" && echo yes || echo no)" "yes"
REG_DIGEST="$(b3sum "${REG_FILE}" | awk '{print $1}')"
e2e_assert_eq "refusals_registry_digest_len" "${#REG_DIGEST}" "64"
e2e_metric "spark_refusals_backend_len" "${#BACKEND}" "chars"

e2e_cmd --name "cargo_test_frob_core_spark_refusals" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core --test refusal_registry_spark --test refusal_registry_numerics
e2e_assert_eq "cargo_test_frob_core_spark_refusals_exit" "${E2E_LAST_EXIT}" "0"

e2e_step "staged_provocation_matrix" "Record M0/M2/M4 staged status for FROB-REF-601..618 and device-count-wrap"

# M0 raising sites (601, 606, 609, 617) and 603 (M2 CLI, M0 annex test):
# logged skipped: capability absent until raising mechanisms (BD-014/BD-015/BD-016/BD-029/N21b-36/BD-036/N21c-05) land.
e2e_skip "refusal_601_provoke" "skipped: capability absent (M0 device-open / daemon-dispatch raising sites pending BD-014/BD-029/N32a-04; backend=${BACKEND})"
e2e_skip "refusal_603_provoke" "skipped (M2): no M0 CLI compiles caller source; M0 annex test compile_error_refuses_603 pending BD-015"
e2e_skip "refusal_606_provoke" "skipped: capability absent (M0 fingerprint currency pending N21c-05/BD-014)"
e2e_skip "refusal_609_provoke" "skipped: capability absent (M0 double-float start-up canary pending N21b-36)"
e2e_skip "refusal_617_provoke" "skipped: capability absent (M0 typed stride converter pending BD-036)"

# M2 and M4 codes plus allocation-ledger rows (N35-33):
for code in 602 604 605 607 608 610 611 612 614 615 616 618; do
  e2e_skip "refusal_${code}_provoke" "skipped (M2): mechanism not yet built in M0"
done
e2e_skip "refusal_613_provoke" "skipped (M4): specialization bit-identity fence pending N21c-19"
e2e_skip "refusal_device_count_wrap_provoke" "skipped (M2): allocation ledger pending N35-33 and N21c-39"

e2e_finish
