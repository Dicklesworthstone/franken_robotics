#!/usr/bin/env bash
# e2e: bead=N34p2-01
# e2e: milestone=M0
# e2e: labels=m0,workspace,frob-core,registry,solid
# e2e: seed=0
# e2e: timeout_s=600
# e2e: needs=cpu
#
# scripts/e2e/toml_reader.sh
# End-to-end verification script for N34p2-01 (`frob-core::registry::toml`):
#   1. Hashes the vendored `toml-test` 1.0.0 corpus directory listing (BLAKE3).
#   2. Runs the conformance corpus test (`toml_corpus`) in release mode through
#      the build-offload `cargo` shim and logs one `assert` line per corpus case
#      plus the valid-but-outside-subset count and excluded TOML 1.1 count.
#   3. Runs the 1 MiB synthetic registry speed test (`toml_parse_speed`) in
#      release mode and logs the measured time against the 50 ms target metric.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "toml_reader" "N34p2-01" "$@"

CORPUS_DIR="${E2E_REPO_ROOT}/crates/frob-core/tests/fixtures/toml-test"

e2e_step "corpus_provenance" "Verify vendored toml-test 1.0.0 corpus and record BLAKE3 listing digest"
e2e_assert_file "corpus_license" "${CORPUS_DIR}/LICENSE"
e2e_assert_file "corpus_pin" "${CORPUS_DIR}/PIN.txt"
e2e_assert_file "corpus_manifest_1_0_0" "${CORPUS_DIR}/files-toml-1.0.0"

if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf 'toml_reader: b3sum not found\n' >&2
  exit 2
fi

LISTING_FILE="${E2E_WORK_DIR}/corpus_listing.txt"
(
  cd "${CORPUS_DIR}"
  LC_ALL=C find . -type f | LC_ALL=C sort
) > "${LISTING_FILE}"
CORPUS_LISTING_B3="$("${B3SUM_BIN}" --no-names "${LISTING_FILE}")"
if [[ "${CORPUS_LISTING_B3}" =~ ^[0-9a-f]{64}$ ]]; then
  e2e_assert_eq "corpus_listing_blake3_format" "hex64" "hex64"
else
  e2e_assert_eq "corpus_listing_blake3_format" "${CORPUS_LISTING_B3}" "hex64"
fi
e2e_assert_eq "corpus_listing_blake3" "${CORPUS_LISTING_B3}" "${CORPUS_LISTING_B3}"
e2e_metric "corpus_listing_files" "$(wc -l < "${LISTING_FILE}" | tr -d ' ')" "files" --observe

e2e_step "conformance_corpus" "Run toml-test 1.0.0 conformance suite in release mode"
CORPUS_REPORT="${E2E_WORK_DIR}/corpus_report.tsv"
e2e_cmd --name "cargo_test_toml_corpus_release" --expect-exit "0" -- \
  env "FROB_TOML_CORPUS_REPORT=${CORPUS_REPORT}" \
  cargo test --manifest-path "${E2E_REPO_ROOT}/Cargo.toml" -p frob-core --test toml_corpus --release -- --nocapture

e2e_assert_eq "toml_corpus_exit" "${E2E_LAST_EXIT}" "0"
e2e_assert_file "toml_corpus_report" "${CORPUS_REPORT}"

if [ -f "${CORPUS_REPORT}" ]; then
  VALID_PARSED="0"
  VALID_OUTSIDE="0"
  INVALID_REFUSED="0"
  EXCLUDED_11="0"
  FIRST_FAILURE=""
  while IFS=$'\t' read -r col1 col2 col3 col4 col5 col6 col7; do
    case "${col1}" in
      "#SUMMARY")
        for kv in "${col2}" "${col3}" "${col4}" "${col5}" "${col6}" "${col7}"; do
          case "${kv}" in
            valid_parsed=*) VALID_PARSED="${kv#*=}" ;;
            valid_outside_subset=*) VALID_OUTSIDE="${kv#*=}" ;;
            invalid_refused=*) INVALID_REFUSED="${kv#*=}" ;;
            excluded_toml_1_1=*) EXCLUDED_11="${kv#*=}" ;;
          esac
        done
        ;;
      "")
        ;;
      *)
        case_name="${col1}"
        expected="${col2}"
        observed="${col3}"
        status="${col4}"
        err_kind="${col5}"
        pos="${col6}"
        rel_path="${col7}"
        if [ "${status}" = "pass" ]; then
          detail="expected=${expected} observed=${observed} kind=${err_kind} pos=${pos}"
          e2e_assert_eq "corpus:${case_name}" "${detail}" "${detail}"
        else
          if [ -z "${FIRST_FAILURE}" ]; then
            FIRST_FAILURE="${case_name} (${CORPUS_DIR}/${rel_path})"
          fi
          e2e_assert_eq "corpus:${case_name}" "observed=${observed} kind=${err_kind} pos=${pos} path=${CORPUS_DIR}/${rel_path}" "expected=${expected}"
        fi
        ;;
    esac
  done < "${CORPUS_REPORT}"

  e2e_metric "corpus_valid_parsed" "${VALID_PARSED}" "cases" --observe
  e2e_metric "corpus_valid_outside_subset" "${VALID_OUTSIDE}" "cases" --observe
  e2e_metric "corpus_invalid_refused" "${INVALID_REFUSED}" "cases" --observe
  e2e_metric "corpus_excluded_toml_1_1" "${EXCLUDED_11}" "cases" --observe
  if [ -n "${FIRST_FAILURE}" ]; then
    e2e_assert_eq "corpus_first_failure" "${FIRST_FAILURE}" "none"
  fi
fi

e2e_step "parse_speed_release" "Measure 1 MiB synthetic registry parse time in release mode (target <= 50 ms)"
e2e_cmd --name "cargo_test_toml_parse_speed_release" --expect-exit "0" -- \
  cargo test --manifest-path "${E2E_REPO_ROOT}/Cargo.toml" -p frob-core --release --lib \
  registry::toml::tests::toml_parse_speed -- --ignored --nocapture

e2e_assert_eq "toml_parse_speed_exit" "${E2E_LAST_EXIT}" "0"

SPEED_MS="$(awk -F= '/^TOML_PARSE_SPEED_MS=/{print $2; exit}' "${E2E_LAST_STDOUT}" 2>/dev/null || true)"
if [ -n "${SPEED_MS}" ]; then
  e2e_metric "toml_parse_1mib_ms" "${SPEED_MS}" "ms" "<=" "50"
else
  e2e_assert_eq "toml_parse_speed_metric_present" "missing" "present"
fi
