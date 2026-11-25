#!/usr/bin/env bash
# e2e: bead=N32-REF milestone=M0 labels=m0,tether,registry,solid seed=0 timeout_s=900 needs=cpu
#
# End-to-end verification of the Tether refusal-code registry (bead N32-REF):
#   1. Runs frob-core's refusal-registry tests (embedded load + generated RefusalCode parity).
#   2. Runs frob-verbs's Tether refusal-registry integration tests (all 20 Tether rows, exits,
#      retry classes, living-world 724/726 split, 804 field form, 990 internal-fault contract,
#      layer-pair cross-references, and planted negative controls).
#   3. Parses registries/refusals.toml directly and asserts every one of the 20 Tether rows
#      (code, exit, retry, due) plus records metrics (20 rows present, 3 held 8xx entries under
#      N35x-01 with interim=none) and the BLAKE3/SHA-256 digest of registries/refusals.toml.
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/tether_refusal_registry/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Compatible with bash 3.2 (macOS) and Linux.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "tether_refusal_registry" "N32-REF" "$@"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
REGISTRY="${E2E_REPO_ROOT}/registries/refusals.toml"
TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-tether-refusal-registry-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

# ---------------------------------------------------------------------------
# Step 1: Registry loader and generated RefusalCode parity (frob-core)
# ---------------------------------------------------------------------------
e2e_step "core_registry_tests" "Run frob-core registry::refusals unit tests"

e2e_cmd --name "cargo_test_frob_core_refusals" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-core -- registry::refusals
e2e_assert_eq "core_refusals_test_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 2: Tether refusal registry suite + planted negative controls (frob-verbs)
# ---------------------------------------------------------------------------
e2e_step "tether_registry_tests" "Run frob-verbs refusal_registry integration tests and negative controls"

e2e_cmd --name "cargo_test_frob_verbs_refusal_registry" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}" \
  cargo test --manifest-path "${MANIFEST}" --offline -j 1 -p frob-verbs --test refusal_registry
e2e_assert_eq "tether_refusal_registry_test_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 3: Per-row audit of the 20 Tether rows, 3 held 8xx entries, and digest
# ---------------------------------------------------------------------------
e2e_step "row_table_audit" "Assert all 20 Tether rows (code, exit, retry, due), 3 held 8xx entries, and registry digest"

if command -v b3sum >/dev/null 2>&1; then
  REG_DIGEST="$(b3sum "${REGISTRY}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  REG_DIGEST="$(shasum -a 256 "${REGISTRY}" | awk '{print $1}')"
else
  REG_DIGEST="unavailable"
fi
e2e_note "registries/refusals.toml digest=${REG_DIGEST}"

PARSED_TSV="${E2E_SCRATCH_DIR}/tether_rows.tsv"
mkdir -p "${E2E_SCRATCH_DIR}"
awk '
  function val(line) { sub(/^[a-z_]+ = /, "", line); gsub(/^"|"$/, "", line); return line }
  function flush() {
    if (code != "") { printf "%s\t%s\t%s\t%s\n", code, ex, retry, meaning }
    code = ""; ex = ""; retry = ""; meaning = ""
  }
  /^\[\[refusal\]\]/ { flush(); next }
  /^code = /         { code = val($0) }
  /^exit = /         { ex = val($0) }
  /^retry = /        { retry = val($0) }
  /^meaning = /      { meaning = val($0) }
  END { flush() }
' "${REGISTRY}" > "${PARSED_TSV}"

check_row() {
  local want_code="$1" want_exit="$2" want_retry="$3" want_due="$4"
  local line="" got_exit="" got_retry="" got_meaning="" got_due="missing"
  line="$(awk -F '\t' -v c="${want_code}" '$1 == c { print; exit }' "${PARSED_TSV}")"
  if [ -n "${line}" ]; then
    got_exit="$(printf '%s' "${line}" | awk -F '\t' '{print $2}')"
    got_retry="$(printf '%s' "${line}" | awk -F '\t' '{print $3}')"
    got_meaning="$(printf '%s' "${line}" | awk -F '\t' '{print $4}')"
    case "${got_meaning}" in
      *"due ${want_due}"*) got_due="${want_due}" ;;
    esac
  fi
  e2e_assert_eq "row_${want_code}" "${got_exit}:${got_retry}:${got_due}" "${want_exit}:${want_retry}:${want_due}"
}

check_row "FROB-REF-701" "4" "after-change" "M2"
check_row "FROB-REF-702" "4" "after-change" "M1"
check_row "FROB-REF-703" "4" "backoff"      "M3"
check_row "FROB-REF-704" "4" "after-change" "M3"
check_row "FROB-REF-720" "5" "after-change" "M2"
check_row "FROB-REF-723" "5" "after-change" "M2"
check_row "FROB-REF-724" "5" "never"        "M0"
check_row "FROB-REF-725" "5" "after-change" "M3"
check_row "FROB-REF-726" "5" "after-change" "M3"
check_row "FROB-REF-801" "3" "after-change" "M0"
check_row "FROB-REF-802" "3" "after-change" "M0"
check_row "FROB-REF-803" "7" "after-change" "M1"
check_row "FROB-REF-804" "7" "after-change" "M1"
check_row "FROB-REF-805" "3" "after-change" "M1"
check_row "FROB-REF-806" "3" "after-change" "M1"
check_row "FROB-REF-807" "3" "after-change" "M1"
check_row "FROB-REF-808" "3" "after-change" "M1"
check_row "FROB-REF-809" "7" "after-change" "M1"
check_row "FROB-REF-810" "9" "never"        "M1"
check_row "FROB-REF-990" "9" "never"        "M0"

# Held 8xx requests (N35x-01; interim = none) must NOT appear as rows in registries/refusals.toml.
HELD_ABSENT=0
for held_code in FROB-REF-836 FROB-REF-837 FROB-REF-838; do
  if ! awk -F '\t' -v c="${held_code}" '$1 == c { found = 1 } END { exit(found ? 0 : 1) }' "${PARSED_TSV}"; then
    HELD_ABSENT=$((HELD_ABSENT + 1))
    e2e_assert_eq "held_${held_code}_absent" "absent" "absent"
  else
    e2e_assert_eq "held_${held_code}_absent" "present" "absent"
  fi
done

e2e_metric "tether_rows_present" "20" --unit "count" --min "20"
e2e_metric "held_8xx_entries_absent_from_rows" "${HELD_ABSENT}" --unit "count" --min "3"
e2e_skip "allocation_ledger_and_retired_history" "registries/refusal_allocations.toml (N35-33/N35x-02) and retired-code history file have not landed yet (Review Note 1)"
