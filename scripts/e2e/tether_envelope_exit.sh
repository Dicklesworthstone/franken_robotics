#!/usr/bin/env bash
# e2e: bead=N32a-03 milestone=M0 labels=m0,tether,frob-verbs,solid,envelope gates=GATE-TETHER-03 seed=0 timeout_s=3600 needs=cpu plants=nc-bare-unknown-verb,nc-second-envelope
#
# End-to-end check of frob.envelope/1 and the exit-code table (bead N32a-03, GATE-TETHER-03,
# KA-755). Three steps:
#   1. the frob-verbs envelope unit tests and compile-fail doctests (goldens, exit table
#      against the registry, fixture rows through the envelope builder and the schema
#      validator, the one-envelope property);
#   2. the negative controls: a build that prints a bare error for an unknown verb and a build
#      that prints a second envelope must each fail their one-envelope tests;
#   3. every row of crates/frob-conformance/fixtures/tether/exit_table.toml through the real
#      `frob` binary: process exit against `exit`, the output against the one-envelope rule
#      and the schema (examples/envelope_check), `refusal.code`, `ok`, and a stack receipt.
#      The checker's report, evidence block included, is kept as the step's stdout artifact.
# The `frob` binary is assembled on frob-verbs by BD-022. Until it exists, and for rows whose
# mechanism has not landed (`exercised = false`), step 3 records "not exercised" (a skip with
# the reason), never a pass, and GATE-TETHER-03 is not claimed.
#
# Plants: E2E_PLANT_FAILURE=nc-bare-unknown-verb or nc-second-envelope builds step 1 with
# that planted defect; the script must then fail.
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "tether_envelope_exit" "N32a-03"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
FIXTURE="${E2E_REPO_ROOT}/crates/frob-conformance/fixtures/tether/exit_table.toml"

PLANT_FEATURE=""
if e2e_plant "nc-bare-unknown-verb"; then
  PLANT_FEATURE="nc-bare-unknown-verb"
elif e2e_plant "nc-second-envelope"; then
  PLANT_FEATURE="nc-second-envelope"
fi

# ---------------------------------------------------------------------------
# Step 1: envelope unit tests (under a plant, built with the planted defect)
# ---------------------------------------------------------------------------
e2e_step "envelope_unit_tests" "Run the frob-verbs envelope unit tests and compile-fail doctests"

# The envelope's tests are the library's unit tests (`envelope::tests`, `envelope::regex`,
# `envelope::validate`) and its doctests (the compile-fail checks); `--lib` and `--doc` select
# exactly those, so an unrelated integration-test target of the crate cannot decide this step.
FEATURE_ARGS=()
if [ -n "${PLANT_FEATURE}" ]; then
  FEATURE_ARGS=(--features "${PLANT_FEATURE}")
fi
e2e_cmd --name "envelope_unit_tests${PLANT_FEATURE:+_planted_${PLANT_FEATURE}}" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-verbs ${FEATURE_ARGS[@]+"${FEATURE_ARGS[@]}"} --lib -- envelope
e2e_assert_eq "envelope_unit_tests_exit" "${E2E_LAST_EXIT}" "0"
e2e_cmd --name "envelope_doctests${PLANT_FEATURE:+_planted_${PLANT_FEATURE}}" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-verbs ${FEATURE_ARGS[@]+"${FEATURE_ARGS[@]}"} --doc -- envelope
e2e_assert_eq "envelope_doctests_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 2: negative controls (each planted build must fail the named test, not the build)
# ---------------------------------------------------------------------------
e2e_step "negative_controls" "Planted bare error and second envelope must fail the one-envelope tests"

run_control() {
  local feature="$1" test_name="$2" observed="not-observed"
  e2e_expect_caught "${feature}" --gate GATE-TETHER-03 --nc "${feature}" -- \
    cargo test --manifest-path "${MANIFEST}" -p frob-verbs --features "${feature}" --lib -- \
    --exact "${test_name}"
  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    return 0
  fi
  if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ] &&
    grep -Fq "test ${test_name} ... FAILED" "${E2E_LAST_STDOUT}"; then
    observed="failed"
  fi
  e2e_assert_eq "${feature}_fails_${test_name##*::}" "${observed}" "failed" --nc "${feature}"
}

run_control "nc-bare-unknown-verb" "envelope::tests::control_bare_error_unknown_verb"
run_control "nc-bare-unknown-verb" "envelope::tests::one_envelope_always"
run_control "nc-second-envelope" "envelope::tests::control_second_envelope_serve"
run_control "nc-second-envelope" "envelope::tests::one_envelope_always"

# ---------------------------------------------------------------------------
# Step 3: the fixture rows through the real binary
# ---------------------------------------------------------------------------
e2e_step "cli_exit_table" "Each fixture row through frob: exit, one schema-valid envelope, refusal.code, ok"

ROWS_FILE="${E2E_SCRATCH_DIR}/exit_table_rows.tsv"
mkdir -p "${E2E_SCRATCH_DIR}"
# One line per row: id, ok, exit, code, exercised, mechanism, argv joined by US (0x1f).
# The fixture keeps one `key = value` per line and `argv` on one line for this reader.
awk '
  function val(line) { sub(/^[a-z_]+ = /, "", line); gsub(/^"|"$/, "", line); return line }
  function flush() {
    if (id != "") { printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n", id, ok, ex, code, exercised, mech, argv }
    id = ""; ok = ""; ex = ""; code = ""; exercised = ""; mech = ""; argv = ""
  }
  /^\[\[row\]\]/ { flush(); next }
  /^id = /        { id = val($0) }
  /^ok = /        { ok = val($0) }
  /^exit = /      { ex = val($0) }
  /^code = /      { code = val($0) }
  /^exercised = / { exercised = val($0) }
  /^mechanism = / { mech = val($0) }
  /^argv = \[/    {
    line = $0; sub(/^argv = \[/, "", line); sub(/\][ \t]*$/, "", line)
    n = split(line, parts, /", "/)
    argv = ""
    for (i = 1; i <= n; i++) { p = parts[i]; gsub(/^"|"$/, "", p); argv = (i == 1 ? p : argv "\037" p) }
  }
  END { flush() }
' "${FIXTURE}" > "${ROWS_FILE}"

ROW_COUNT="$(wc -l < "${ROWS_FILE}" | tr -d ' ')"
e2e_assert_eq "fixture_row_count" "${ROW_COUNT}" "5" --gate GATE-TETHER-03

HAVE_FROB="0"
if [ -x "${FROB_BIN}" ]; then
  HAVE_FROB="1"
fi

while IFS="$(printf '\t')" read -r id want_ok want_exit want_code exercised mechanism argv_joined <&3; do
  if [ "${HAVE_FROB}" != "1" ]; then
    e2e_skip "not exercised: ${id}: no frob binary at ${FROB_BIN} (BD-022 assembles it)"
    continue
  fi
  if [ "${exercised}" != "true" ]; then
    e2e_skip "not exercised: ${id}: ${mechanism} has not landed"
    continue
  fi
  ARGV=()
  IFS="$(printf '\037')" read -r -a ARGV <<< "${argv_joined}"

  e2e_cmd --name "row_${id}" --expect-exit "${want_exit}" -- "${FROB_BIN}" "${ARGV[@]}"
  e2e_assert_eq "row_${id}_exit" "${E2E_LAST_EXIT}" "${want_exit}" --gate GATE-TETHER-03 --ka KA-755
  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    continue
  fi
  ROW_STDOUT="${E2E_LAST_STDOUT}"

  e2e_cmd --name "row_${id}_envelope_check" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-verbs --example envelope_check -- "${ROW_STDOUT}"
  e2e_assert_eq "row_${id}_envelope_check_exit" "${E2E_LAST_EXIT}" "0" --gate GATE-TETHER-03
  e2e_assert_json "row_${id}_one_schema_valid_envelope" "/ok" "true" --gate GATE-TETHER-03
  e2e_assert_json "row_${id}_envelope_exit" "/exit" "${want_exit}" --gate GATE-TETHER-03 --ka KA-755
  e2e_assert_json "row_${id}_refusal_code" "/code" "${want_code}" --gate GATE-TETHER-03 --ka KA-755
  e2e_assert_json "row_${id}_ok" "/envelope_ok" "${want_ok}" --gate GATE-TETHER-03
  RECEIPT="$(e2e_get --file "${E2E_LAST_STDOUT}" /evidence/stack_receipt 2>/dev/null || printf 'absent')"
  case "${RECEIPT}" in
    sr:*) RECEIPT_FORM="sr" ;;
    *) RECEIPT_FORM="${RECEIPT}" ;;
  esac
  e2e_assert_eq "row_${id}_stack_receipt_form" "${RECEIPT_FORM}" "sr" --gate GATE-TETHER-03
done 3< "${ROWS_FILE}"

if [ "${HAVE_FROB}" != "1" ]; then
  e2e_skip "GATE-TETHER-03 not claimed: no fixture row ran through the frob binary"
fi
