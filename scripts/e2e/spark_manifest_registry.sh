#!/usr/bin/env bash
# e2e: bead=N21b-01 milestone=M0 labels=m0,spark,frob-kernels,solid,manifest gates=GATE-SPARK-00 seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of frob.kernel/1 manifest schema, static and binding
# validation rules, provenance loader, and registries/kernels.toml doctor check
# (bead N21b-01).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/spark_manifest_registry/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "spark_manifest_registry" "N21b-01" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-spark-manifest-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "schema_and_registry" "Locate frob.kernel/1 schema and registries/kernels.toml"

SCHEMA_FILE="${E2E_REPO_ROOT}/schemas/frob.kernel.1.json"
REG_FILE="${E2E_REPO_ROOT}/registries/kernels.toml"

e2e_assert_eq "kernel_schema_exists" "$(test -f "${SCHEMA_FILE}" && echo yes || echo no)" "yes"
e2e_assert_eq "kernels_registry_exists" "$(test -f "${REG_FILE}" && echo yes || echo no)" "yes"

e2e_step "manifest_suite" "Run frob-kernels manifest static, binding, KA-532/CX-218, and doctor-section tests"

e2e_cmd --name "cargo_test_frob_kernels_manifest" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-kernels" \
  cargo test --offline -j 1 -p frob-kernels --test manifest --test conformance
e2e_assert_eq "cargo_test_frob_kernels_manifest_exit" "${E2E_LAST_EXIT}" "0"

e2e_step "doctor_cli_check" "Check frob doctor --section kernels when linked"

if [ -n "${FROB_BIN:-}" ] && [ -x "${FROB_BIN}" ]; then
  e2e_cmd --name "frob_doctor_sections" --expect-exit 0,2 -- "${FROB_BIN}" doctor --json
  if [ "${E2E_DRY_RUN:-0}" = "1" ] || e2e_get /result/kernels >/dev/null; then
    e2e_cmd --name "frob_doctor_kernels" -- "${FROB_BIN}" doctor --section kernels --json
    e2e_assert_eq "frob_doctor_kernels_exit" "${E2E_LAST_EXIT}" "0"
  else
    e2e_skip "frob_doctor_kernels_cli: kernels section absent from linked frob doctor"
  fi
else
  e2e_skip "frob_doctor_kernels_cli: no executable FROB_BIN was provided"
fi
