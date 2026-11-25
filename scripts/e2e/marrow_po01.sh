#!/usr/bin/env bash
# e2e: bead=BD-010 milestone=M0 seed=1 needs=cpu
#
# End-to-end verification of PO-MARROW-01 (`frob-lie` bit-parity against `fs-ga`,
# bead BD-010):
#   1. Runs standalone `tools/oracle/marrow_fsga_capture` into a temporary file
#      and asserts the BLAKE3 of its record section equals the committed
#      `tests/fixtures/oracle/marrow/po_marrow_01.frobfix` (proving the oracle
#      is reproducible from `fs-ga` alone, without `frob-lie`).
#   2. Runs `cargo test -p frob-lie --test po_marrow_01` (all 1,000 records
#      bit-for-bit, plus coad/KA-193/KA-229/sign-rule/refusal unit tests).
#   3. Reads the FrankenSim pin (`FD-FSIM.git_rev`) from `franken-stack.lock`
#      and compares it with the fixture `[payload].frankensim_commit`:
#      equal passes; different logs a `pin_moved` metric and passes only if
#      step (1) passed.
#   4. Emits a final summary naming PO-MARROW-01.
#
# Sources scripts/e2e/lib.sh (STD-E2E) and writes structured NDJSON to
# target/e2e-logs/marrow_po01/<utc>.ndjson.
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd -P)"

# shellcheck source=scripts/e2e/lib.sh
source "${REPO_ROOT}/scripts/e2e/lib.sh"

e2e_init "marrow_po01" "BD-010" "$@"

WORK="${E2E_WORK_DIR}"
FIXTURE="${REPO_ROOT}/tests/fixtures/oracle/marrow/po_marrow_01.frobfix"
STACK_LOCK="${REPO_ROOT}/franken-stack.lock"
CAPTURE_MANIFEST="${REPO_ROOT}/tools/oracle/marrow_fsga_capture/Cargo.toml"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}"
CAPTURE_TARGET="${TARGET_BASE}/frob-marrow-capture-e2e"
LIE_TARGET="${CARGO_TARGET_DIR:-${TARGET_BASE}/frob-lie-e2e}"

e2e_assert_file "committed_fixture_exists" "${FIXTURE}"
e2e_assert_file "stack_lock_exists" "${STACK_LOCK}"
e2e_assert_file "capture_manifest_exists" "${CAPTURE_MANIFEST}"

# ----------------------------------------------------------------------------
# Step 1: Run standalone fs-ga capture into temp file & compare record BLAKE3
# ----------------------------------------------------------------------------
e2e_step "capture_reproducibility" "Run standalone marrow_fsga_capture and assert record-section BLAKE3 equals committed fixture"

FRESH_FIXTURE="${WORK}/po_marrow_01.captured.frobfix"

e2e_cmd --name run_marrow_fsga_capture -- \
  env "CARGO_TARGET_DIR=${CAPTURE_TARGET}" \
  "RCH_ALLOW_FOREIGN_ARTIFACTS=1" \
  cargo run -q --offline --manifest-path "${CAPTURE_MANIFEST}" -j 1 -- --stdout
e2e_assert_eq "capture_exit_zero" "${E2E_LAST_EXIT}" "0"

# Extract pure fixture body from captured stdout (or stderr if a build-offload shim merged streams)
# and independently hash the record section (header excluded) with BLAKE3.
CAPTURE_CHECK_OUT="${WORK}/capture_check.kv"
python3 - "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" "${FRESH_FIXTURE}" "${FIXTURE}" "${CAPTURE_CHECK_OUT}" "${REPO_ROOT}/tools/oracle/design_canon" <<'PYEOF'
import pathlib, re, sys

sys.path.insert(0, sys.argv[6])
import blake3_ref

stdout_txt = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
stderr_txt = pathlib.Path(sys.argv[2]).read_text(encoding="utf-8")
raw_out = stdout_txt if "# PO-MARROW-01" in stdout_txt else stderr_txt
idx = raw_out.find("# PO-MARROW-01")
end = raw_out.find("\n]\n", idx) if idx >= 0 else -1
if idx < 0 or end < 0:
    sys.stderr.write("marrow_fsga_capture output missing PO-MARROW-01 fixture body\n")
    sys.exit(1)
fresh_raw = raw_out[idx:end + 3].replace("\r\n", "\n")
# Drop any stray blank line injected inside records = [...] by terminal streaming.
rec_start = fresh_raw.find("records = [\n")
if rec_start >= 0:
    head = fresh_raw[:rec_start + len("records = [\n")]
    tail_lines = [l for l in fresh_raw[rec_start + len("records = [\n"):].splitlines(keepends=True) if l != "\n"]
    fresh_text = head + "".join(tail_lines)
else:
    fresh_text = fresh_raw
pathlib.Path(sys.argv[3]).write_text(fresh_text, encoding="utf-8", newline="\n")

committed_text = pathlib.Path(sys.argv[4]).read_text(encoding="utf-8")

def extract_field(txt, key):
    m = re.search(rf'^{re.escape(key)}\s*=\s*"([^"]+)"\s*$', txt, re.M)
    return m.group(1) if m else ""

def extract_records(txt):
    m = re.search(r'^records\s*=\s*\[\n(.*?)\n\]', txt, re.S | re.M)
    if not m:
        return []
    return [line.strip().strip('",') for line in m.group(1).splitlines() if line.strip()]

def hash_record_section(recs):
    payload = "".join(r + "\n" for r in recs).encode("utf-8")
    return blake3_ref.hash_hex(payload)

fresh_recs = extract_records(fresh_text)
committed_recs = extract_records(committed_text)
fresh_b3 = hash_record_section(fresh_recs)
committed_b3 = hash_record_section(committed_recs)
declared_committed_b3 = extract_field(committed_text, "records_blake3")
committed_sha = extract_field(committed_text, "bytes_sha256")
fixture_commit = extract_field(committed_text, "frankensim_commit")
records_exact = (fresh_recs == committed_recs) and (len(committed_recs) == 1000) and (committed_b3 == declared_committed_b3)

with open(sys.argv[5], "w", encoding="utf-8", newline="\n") as out:
    out.write(f"fresh_b3={fresh_b3}\n")
    out.write(f"committed_b3={committed_b3}\n")
    out.write(f"committed_sha={committed_sha}\n")
    out.write(f"fixture_commit={fixture_commit}\n")
    out.write(f"records_exact={1 if records_exact else 0}\n")
    out.write(f"record_count={len(committed_recs)}\n")
PYEOF

FRESH_B3=""
COMMITTED_B3=""
COMMITTED_SHA=""
FIXTURE_COMMIT=""
RECORDS_EXACT="0"
RECORD_COUNT="0"
while IFS='=' read -r k v; do
  case "${k}" in
    fresh_b3) FRESH_B3="${v}" ;;
    committed_b3) COMMITTED_B3="${v}" ;;
    committed_sha) COMMITTED_SHA="${v}" ;;
    fixture_commit) FIXTURE_COMMIT="${v}" ;;
    records_exact) RECORDS_EXACT="${v}" ;;
    record_count) RECORD_COUNT="${v}" ;;
  esac
done < "${CAPTURE_CHECK_OUT}"

e2e_assert_eq "record_count_is_1000" "${RECORD_COUNT}" "1000"
e2e_assert_eq "capture_records_blake3_matches_committed" "${FRESH_B3}" "${COMMITTED_B3}"
e2e_assert_eq "capture_records_section_exact_match" "${RECORDS_EXACT}" "1"
e2e_metric "po_marrow_01_record_count" "${RECORD_COUNT}" "records" "committed=${COMMITTED_B3} bytes_sha256=${COMMITTED_SHA}"

STEP1_PASSED="1"
if [ "${FRESH_B3}" != "${COMMITTED_B3}" ] || [ "${RECORDS_EXACT}" != "1" ]; then
  STEP1_PASSED="0"
fi

# ----------------------------------------------------------------------------
# Step 2: Run cargo test -p frob-lie --test po_marrow_01
# ----------------------------------------------------------------------------
e2e_step "po_marrow_01_bit_parity_test" "Run cargo test -p frob-lie --test po_marrow_01"

e2e_cmd --name cargo_test_po_marrow_01 -- \
  env "CARGO_TARGET_DIR=${LIE_TARGET}" \
  cargo test --offline --manifest-path "${REPO_ROOT}/Cargo.toml" -p frob-lie --test po_marrow_01 -j 1
e2e_assert_eq "po_marrow_01_test_exit_zero" "${E2E_LAST_EXIT}" "0"

# ----------------------------------------------------------------------------
# Step 3: Compare franken-stack.lock FD-FSIM git_rev with fixture commit
# ----------------------------------------------------------------------------
e2e_step "frankensim_pin_check" "Compare franken-stack.lock FD-FSIM git_rev with fixture header frankensim_commit"

LOCK_COMMIT="$(python3 - "${STACK_LOCK}" <<'PYEOF'
import pathlib, re, sys
txt = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
blocks = txt.split("[[foundation]]")
for blk in blocks[1:]:
    if re.search(r'^\s*id\s*=\s*"FD-FSIM"\s*$', blk, re.M):
        m = re.search(r'^\s*git_rev\s*=\s*"([0-9a-f]{40})"\s*$', blk, re.M)
        if m:
            print(m.group(1))
            sys.exit(0)
sys.exit(1)
PYEOF
)"

e2e_assert_ne "stack_lock_fd_fsim_rev_present" "${LOCK_COMMIT}" ""
e2e_assert_ne "fixture_frankensim_commit_present" "${FIXTURE_COMMIT}" ""

if [ "${LOCK_COMMIT}" = "${FIXTURE_COMMIT}" ]; then
  e2e_metric "pin_moved" "0" "flag" "lock=${LOCK_COMMIT} fixture=${FIXTURE_COMMIT}"
else
  e2e_metric "pin_moved" "1" "flag" "lock=${LOCK_COMMIT} fixture=${FIXTURE_COMMIT} step1_passed=${STEP1_PASSED}"
  e2e_assert_eq "pin_moved_requires_step1_bit_parity" "${STEP1_PASSED}" "1"
fi

# ----------------------------------------------------------------------------
# Step 4: Summary naming PO-MARROW-01
# ----------------------------------------------------------------------------
e2e_step "summary_po_marrow_01" "Record PO-MARROW-01 completion summary and fixture digests"
e2e_metric "PO-MARROW-01" "1" "pass" "records_blake3=${COMMITTED_B3} bytes_sha256=${COMMITTED_SHA}"
