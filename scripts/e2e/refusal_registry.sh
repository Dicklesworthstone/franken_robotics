#!/usr/bin/env bash
# e2e: bead=N35-33 milestone=M0 labels=m0,frob-core,registry,solid,tether,spark seed=0 timeout_s=600 needs=cpu plants=hand-edited-printout
#
# End-to-end verification of the refusal allocation ledger, cross-chapter checks
# 1..6, and generated docs/generated/refusals.md printout (bead N35-33).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/refusal_registry/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "refusal_registry" "N35-33" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-refusal-registry-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "stage1_tests_and_generated" "Run allocation tests and check the refusal printout"

e2e_cmd --name "cargo_test_frob_core_refusals" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core --test refusal_allocations
e2e_assert_eq "cargo_test_frob_core_refusals_exit" "${E2E_LAST_EXIT}" "0"

printout="docs/generated/refusals.md"
scratch="${E2E_WORK_DIR}/scratch-printout"
mkdir -p "${scratch}/registries" "${scratch}/docs/generated"
for path in registries/refusals.toml registries/refusal_allocations.toml "${printout}"; do
  cp "${E2E_REPO_ROOT}/${path}" "${scratch}/${path}"
done
check=(env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" cargo run --offline -j 1 -p xtask --
  check-generated --generator refusal-printout --root "${scratch}" --json)
e2e_cmd --name "refusal_printout_baseline" -- "${check[@]}"
e2e_assert_eq "refusal_printout_baseline_exit" "${E2E_LAST_EXIT}" "0"
expected_table="${E2E_LAST_STDOUT}"
if [ "${E2E_DRY_RUN:-0}" != "1" ] && [ "${E2E_LAST_EXIT}" = 0 ]; then
  while read -r metric field; do
    e2e_metric "${metric}" "$(e2e_get --file "${expected_table}" "/facts/rows/0/${field}")" rows
  done <<'METRICS'
registry_rows_present registry_rows
printed_rows_target printed_rows_target
allocations_assigned assigned
allocations_merged merged
interim_unsettled interim_unsettled
METRICS
fi

printf '<!-- planted hand edit -->\n' >> "${scratch}/${printout}"
e2e_cmd --name "refusal_printout_edit" --expect-exit 2 -- "${check[@]}"

if [ "${E2E_DRY_RUN:-0}" = "1" ]; then exit 0; fi
caught=0
if [ "${E2E_LAST_EXIT}" = 2 ] && python3 - "${E2E_LAST_STDOUT}" "${printout}" <<'PY'
import json
import sys

with open(sys.argv[1]) as report:
    findings = json.load(report)["findings"]
actual = [(f["rule"], f["subject"], f["path"]) for f in findings]
sys.exit(actual != [("gen.differs", "refusal-printout", sys.argv[2])])
PY
then
  caught=1
fi
e2e_assert_eq "refusal_printout_edit_detected" "${caught}" "1"
expected=1
if e2e_plant "hand-edited-printout"; then expected=0; fi
e2e_assert_eq "hand-edited-printout" "${caught}" "${expected}"

schema_matches() {
  e2e_cmd --name "schema_${1}_matches_registry" -- python3 -c '
import json
import sys
from pathlib import Path

rows = json.loads(Path(sys.argv[1]).read_text())["facts"]["rows"]
registry = next(row for row in rows if row["name"] == "refusal-printout")
result = json.loads(Path(sys.argv[2]).read_text())["result"]
if [row["code"] for row in result["refusals"]] != registry["codes"]:
    sys.exit("served refusal codes differ from the registry")
if sys.argv[3] == "named" and result["interims"] != registry["interims"]:
    sys.exit("served interims differ from the validated ledger")
' "${expected_table}" "${E2E_LAST_STDOUT}" "$1"
  e2e_assert_eq "schema_${1}_registry_content" "${E2E_LAST_EXIT}" "0"
}

can_raise() {
  local code="$1"
  shift
  e2e_cmd --name "capability_${code}" --expect-exit 0,3 -- "${FROB_BIN}" "$@" --help --json
  [ "${E2E_LAST_EXIT}" = 0 ] && python3 -c '
import json
import sys
from pathlib import Path

rows = json.loads(Path(sys.argv[1]).read_text())["result"]["refusals"]
sys.exit(not any(row["code"] == sys.argv[2] for row in rows))
' "${E2E_LAST_STDOUT}" "${code}"
}

if [ -n "${FROB_BIN:-}" ] && [ -x "${FROB_BIN}" ]; then
  e2e_step "stage2_schema_codes" "Compare the bare schema refusal codes with the registry"
  e2e_cmd --name "frob_json_schema" -- "${FROB_BIN}" --json schema
  schema_matches bare

  e2e_step "stage3_refusal_provocations" "Require the registered code and exit from each available raising site"
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" schema n35-33-unknown-schema --json
  if can_raise FROB-REF-103 design check; then
    fixture="${E2E_REPO_ROOT}/crates/frob-conformance/fixtures/design_golden/42_ref_dup_key.fbot"
    e2e_expect_refusal FROB-REF-103 3 -- "${FROB_BIN}" design check "${fixture}" --json
  else
    e2e_skip "capability absent: design.check FROB-REF-103 (N14-REF)"
  fi
  if can_raise FROB-REF-761 sim run; then
    e2e_expect_refusal FROB-REF-761 4 -- "${FROB_BIN}" sim run @fixture:envelope-two-device --key "$(e2e_key ref761)" --json
  else
    e2e_skip "capability absent: sim.run FROB-REF-761 (N36-20)"
  fi

  e2e_step "stage4_schema_interims" "Compare the named schema refusal codes and the ledger interim list"
  e2e_cmd --name "frob_schema_refusals" -- "${FROB_BIN}" schema --refusals --json
  schema_matches named
else
  e2e_skip "binary absent: stage 2 frob --json schema (delivered by BD-022)"
  e2e_skip "binary absent: stage 3 three-family refusal provocations (delivered by BD-022)"
  e2e_skip "binary absent: stage 4 frob schema --refusals --json interims (delivered by BD-022)"
fi
