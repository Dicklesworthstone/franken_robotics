#!/usr/bin/env bash
# e2e: bead=N39a-01 milestone=M0 seed=1 needs=cpu
#
# End-to-end verification for the ten-fold harness skeleton, five harness
# record schemas, canonical bytes, content addresses, strict refusals, and
# standalone offline scorer package (bead N39a-01).
#
# Uses scripts/e2e/lib.sh (STD-E2E). Writes NDJSON to
# target/e2e-logs/tenfold_records/<utc>.ndjson with a transcript beside it.
# Contains zero sealed material and never deletes files or directories (Rule 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd -P)"

# shellcheck source=scripts/e2e/lib.sh
source "${REPO_ROOT}/scripts/e2e/lib.sh"

e2e_init "tenfold_records" "N39a-01"

WORK="${E2E_WORK_DIR}"
FIXTURES="${WORK}/fixtures"
BAD="${WORK}/bad"
mkdir -p "${FIXTURES}" "${BAD}"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}"
CONF_TARGET="${CARGO_TARGET_DIR:-${TARGET_BASE}/frob-tenfold-e2e}"
SCORER_TARGET="${TARGET_BASE}/frob-tenfold-kit-scorer-e2e"

# ----------------------------------------------------------------------------
# Step 1: Build both ten-fold binaries (frob-tenfold and frob-tenfold-score)
# ----------------------------------------------------------------------------
e2e_step "build_binaries" "Build frob-tenfold and frob-tenfold-score binaries and verify --help"

TENFOLD_BIN="${FROB_TENFOLD_BIN:-}"
SCORER_BIN="${FROB_TENFOLD_SCORE_BIN:-}"

if [ -z "${TENFOLD_BIN}" ] || [ -z "${SCORER_BIN}" ]; then
  e2e_cmd --name build_tenfold_bins -- \
    env "CARGO_TARGET_DIR=${CONF_TARGET}" \
    cargo build -p frob-conformance --bin frob-tenfold --bin frob-tenfold-score
  e2e_assert_eq "build_tenfold_bins_exit" "${E2E_LAST_EXIT}" "0"
  TENFOLD_BIN="${CONF_TARGET}/debug/frob-tenfold"
  SCORER_BIN="${CONF_TARGET}/debug/frob-tenfold-score"
fi

e2e_assert_file "frob_tenfold_binary_exists" "${TENFOLD_BIN}"
e2e_assert_file "frob_tenfold_score_binary_exists" "${SCORER_BIN}"

e2e_cmd --name frob_tenfold_help -- "${TENFOLD_BIN}" --help
_has_tf_help="0"
if [ -f "${E2E_LAST_STDOUT}" ] && grep -Fq "record check" "${E2E_LAST_STDOUT}"; then
  _has_tf_help="1"
fi
e2e_assert_eq "frob_tenfold_help_subcommands" "${_has_tf_help}" "1"

e2e_cmd --name frob_tenfold_score_help -- "${SCORER_BIN}" --help
_has_sc_help="0"
if [ -f "${E2E_LAST_STDOUT}" ] && grep -Fq "check <file>" "${E2E_LAST_STDOUT}"; then
  _has_sc_help="1"
fi
e2e_assert_eq "frob_tenfold_score_help_subcommands" "${_has_sc_help}" "1"

# ----------------------------------------------------------------------------
# Step 2: Generate 5 valid non-sealed fixtures (one per schema) and check each
# ----------------------------------------------------------------------------
e2e_step "valid_fixtures" "Generate and validate all 5 non-sealed ten-fold harness record fixtures"

python3 - "${FIXTURES}" "${BAD}" "${TENFOLD_BIN}" <<'PYEOF'
import json, pathlib, subprocess, sys

fixtures_dir = pathlib.Path(sys.argv[1])
bad_dir = pathlib.Path(sys.argv[2])
tenfold_bin = sys.argv[3]

hex_a = "a" * 64
hex_b = "b" * 64

def canon(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def write_json(path, obj):
    path.write_text(canon(obj), encoding="utf-8")

def stamp_hash(obj, field):
    draft = dict(obj)
    draft[field] = ""
    tmp = fixtures_dir / f".draft_{field}.json"
    write_json(tmp, draft)
    res = subprocess.run([tenfold_bin, "record", "address", "--allow-blank", str(tmp)],
                         capture_output=True, text=True)
    if res.returncode != 0:
        raise RuntimeError(f"failed computing blanked address for {field}: {res.stdout} {res.stderr}")
    addr = res.stdout.strip()
    stamped = dict(obj)
    stamped[field] = addr
    return stamped, addr

# 1. Mission
mission = {
    "schema": "frob.tenfold.mission/1",
    "gate": "rehearsal-m0-e2e",
    "family_id": f"f:{hex_a}",
    "stratum": "legged-quad",
    "deadline_s": "1500",
    "mission_id": "e2e-mission-01",
    "regime": "cold",
    "execution_authorized": False,
    "hardware_safety_signoff": False,
    "sim_to_real_certified": False,
    "reality": "no-claim",
}
write_json(fixtures_dir / "mission.json", mission)

# 2. Freeze (self-hashed via manifest_hash)
freeze_base = {
    "schema": "frob.tenfold.freeze/1",
    "gate": "rehearsal-m0-e2e",
    "arm": "A",
    "stratum": "legged-quad",
    "regime": "cold",
    "wall_budget_s": "14400",
    "manifest_hash": "",
    "execution_authorized": False,
    "hardware_safety_signoff": False,
    "sim_to_real_certified": False,
    "reality": "no-claim",
}
freeze, freeze_addr = stamp_hash(freeze_base, "manifest_hash")
write_json(fixtures_dir / "freeze.json", freeze)

# 3. Ledger (known answer: 14 exclusive phases summing to 1500 s)
ledger = {
    "schema": "frob.tenfold.ledger/1",
    "gate": "rehearsal-m0-e2e",
    "arm": "A",
    "stratum": "legged-quad",
    "family_id": f"f:{hex_a}",
    "campaign": 1,
    "regime": "cold",
    "clock_s": "1500",
    "phases": {
        "author": "600",
        "propose": "360",
        "check": "20",
        "synthesize": "0",
        "compile": "12",
        "screen": "96",
        "train": "150",
        "evaluate": "60",
        "qualify": "40",
        "certify": "0",
        "diagnose": "30",
        "wait": "42",
        "adjudicate": "90",
        "unattributed": "0",
    },
    "execution_authorized": False,
    "hardware_safety_signoff": False,
    "sim_to_real_certified": False,
    "reality": "no-claim",
}
write_json(fixtures_dir / "ledger.json", ledger)

# 4. Receipt (self-hashed via receipt_hash)
receipt_base = {
    "schema": "frob.tenfold.receipt/1",
    "gate": "rehearsal-m0-e2e",
    "arm": "A",
    "stratum": "legged-quad",
    "family_id": f"f:{hex_a}",
    "campaign": 1,
    "mission_hash": f"b3:{hex_b}",
    "elapsed_s": "1.5",
    "submission_index": 0,
    "receipt_hash": "",
    "execution_authorized": False,
    "hardware_safety_signoff": False,
    "sim_to_real_certified": False,
    "reality": "no-claim",
}
receipt, _ = stamp_hash(receipt_base, "receipt_hash")
write_json(fixtures_dir / "receipt.json", receipt)

# 5. Report (known answers: 45/17 -> 2.647; CI 15255/130000 -> 0.1173 / 0.1174)
report_base = {
    "schema": "frob.tenfold.report/1",
    "gate": "rehearsal-m0-e2e",
    "regime": "cold",
    "ratio_exact": "45/17",
    "ratio_decimal": "2.647",
    "ci_lower_exact": "3051/26000",
    "ci_lower_decimal": "0.1173",
    "ci_upper_exact": "3051/26000",
    "ci_upper_decimal": "0.1174",
    "report_hash": "",
    "execution_authorized": False,
    "hardware_safety_signoff": False,
    "sim_to_real_certified": False,
    "reality": "no-claim",
}
report, _ = stamp_hash(report_base, "report_hash")
write_json(fixtures_dir / "report.json", report)

# Eight planted bad fixtures (negative controls):
# 1. Unknown field (exit 3)
bad1 = dict(mission, unknown_extra="x")
write_json(bad_dir / "01_unknown_field.json", bad1)

# 2. Missing field (exit 3)
bad2 = dict(mission)
del bad2["deadline_s"]
write_json(bad_dir / "02_missing_field.json", bad2)

# 3. Duplicate key (exit 3)
(bad_dir / "03_duplicate_key.json").write_text(
    f'{{"deadline_s":"1500","deadline_s":"1500","execution_authorized":false,'
    f'"family_id":"f:{hex_a}","gate":"rehearsal-m0-e2e","hardware_safety_signoff":false,'
    f'"reality":"no-claim","schema":"frob.tenfold.mission/1","sim_to_real_certified":false,'
    f'"stratum":"legged-quad"}}',
    encoding="utf-8",
)

# 4. Duration given as JSON number (exit 3)
bad4 = dict(mission, deadline_s=1500)
write_json(bad_dir / "04_number_duration.json", bad4)

# 5. Non-canonical duration with trailing zero "1.50" (exit 3)
bad5 = dict(mission, deadline_s="1.50")
write_json(bad_dir / "05_noncanonical_duration.json", bad5)

# 6. Unknown schema (exit 3)
bad6 = dict(mission, schema="frob.tenfold.bogus/1")
write_json(bad_dir / "06_unknown_schema.json", bad6)

# 7. Self-hash mismatch / computed without blanking (exit 9)
bad7 = dict(freeze, arm="B")
write_json(bad_dir / "07_hash_mismatch.json", bad7)

# 8. Authorization flag set to true (exit 3)
bad8 = dict(mission, execution_authorized=True)
write_json(bad_dir / "08_auth_true.json", bad8)
PYEOF

for name in mission freeze ledger receipt report; do
  fpath="${FIXTURES}/${name}.json"
  e2e_cmd --name "check_valid_${name}" -- "${TENFOLD_BIN}" record check "${fpath}"
  e2e_assert_json "valid_${name}_schema" "/schema" "frob.tenfold.${name}/1"
  _addr="$(e2e_get "/address" || true)"
  _has_b3="0"
  case "${_addr}" in
    b3:*) _has_b3="1" ;;
  esac
  e2e_assert_eq "valid_${name}_b3_address" "${_has_b3}" "1"
done

# ----------------------------------------------------------------------------
# Step 3: Eight planted bad records (refusals with exact exit code & pointer)
# ----------------------------------------------------------------------------
e2e_step "planted_bad_records" "Verify all 8 planted bad records refuse with exact exit code, reason, and pointer"

check_bad() {
  local fname="$1"
  local exp_exit="$2"
  local exp_reason="$3"
  local exp_ptr="$4"
  local fpath="${BAD}/${fname}"
  local stem="${fname%.json}"
  e2e_cmd --name "bad_${stem}" --expect-exit "${exp_exit}" -- "${TENFOLD_BIN}" record check "${fpath}"
  e2e_assert_eq "bad_${stem}_exit" "${E2E_LAST_EXIT}" "${exp_exit}"
  e2e_assert_json "bad_${stem}_reason" "/refusal/reason" "${exp_reason}"
  if [ -n "${exp_ptr}" ]; then
    e2e_assert_json "bad_${stem}_pointer" "/refusal/pointer" "${exp_ptr}"
  fi
}

check_bad "01_unknown_field.json"        "3" "record-unknown-field"       "/unknown_extra"
check_bad "02_missing_field.json"        "3" "record-missing-field"       "/deadline_s"
check_bad "03_duplicate_key.json"        "3" "record-duplicate-key"       ""
check_bad "04_number_duration.json"      "3" "record-noncanonical-number" "/deadline_s"
check_bad "05_noncanonical_duration.json" "3" "record-noncanonical-number" "/deadline_s"
check_bad "06_unknown_schema.json"       "3" "record-unknown-schema"      "/schema"
check_bad "07_hash_mismatch.json"        "9" "record-hash-mismatch"       "/manifest_hash"
check_bad "08_auth_true.json"            "3" "record-noncanonical-number" "/execution_authorized"

# ----------------------------------------------------------------------------
# Step 4: Build standalone scorer package kit/scorer offline & compare output
# ----------------------------------------------------------------------------
e2e_step "standalone_scorer_offline" "Build standalone kit/scorer offline and verify identical ratio bytes"

KIT_SCORER_BIN="${FROB_KIT_SCORER_BIN:-}"
if [ -z "${KIT_SCORER_BIN}" ]; then
  e2e_cmd --name build_kit_scorer_offline -- \
    env "CARGO_TARGET_DIR=${SCORER_TARGET}" \
    cargo build --manifest-path "${REPO_ROOT}/kit/scorer/Cargo.toml" --offline
  e2e_assert_eq "kit_scorer_offline_build_exit" "${E2E_LAST_EXIT}" "0"
  KIT_SCORER_BIN="${SCORER_TARGET}/debug/frob-tenfold-scorer"
fi
e2e_assert_file "standalone_kit_scorer_binary_exists" "${KIT_SCORER_BIN}"

e2e_cmd --name workspace_scorer_ratio -- "${SCORER_BIN}" ratio 54000 20400 --places 3 --mode point
WS_RATIO_OUT="$(cat "${E2E_LAST_STDOUT}")"
e2e_cmd --name kit_scorer_ratio -- "${KIT_SCORER_BIN}" ratio 54000 20400 --places 3 --mode point
KIT_RATIO_OUT="$(cat "${E2E_LAST_STDOUT}")"
e2e_assert_eq "workspace_and_kit_scorer_ratio_identical" "${WS_RATIO_OUT}" "${KIT_RATIO_OUT}"
e2e_assert_json "ratio_54000_20400_exact" "/exact" "45/17"
e2e_assert_json "ratio_54000_20400_decimal" "/decimal" "2.647"

# ----------------------------------------------------------------------------
# Step 5: Print each valid fixture's content address twice and compare D0 bytes
# ----------------------------------------------------------------------------
e2e_step "address_determinism" "Verify D0 content address determinism across repeated invocations"

for name in mission freeze ledger receipt report; do
  fpath="${FIXTURES}/${name}.json"
  a1_file="${WORK}/${name}.addr1"
  a2_file="${WORK}/${name}.addr2"

  e2e_cmd --name "address_${name}_run1" -- "${TENFOLD_BIN}" record address "${fpath}"
  ADDR1="$(tr -d '\r\n' < "${E2E_LAST_STDOUT}")"
  cp "${E2E_LAST_STDOUT}" "${a1_file}"

  e2e_cmd --name "address_${name}_run2" -- "${TENFOLD_BIN}" record address "${fpath}"
  ADDR2="$(tr -d '\r\n' < "${E2E_LAST_STDOUT}")"
  cp "${E2E_LAST_STDOUT}" "${a2_file}"

  if [ "${ADDR1}" != "${ADDR2}" ]; then
    DIFF_OFFSET="$(python3 - "${a1_file}" "${a2_file}" <<'PYEOF'
import pathlib, sys
b1 = pathlib.Path(sys.argv[1]).read_bytes()
b2 = pathlib.Path(sys.argv[2]).read_bytes()
n = min(len(b1), len(b2))
off = next((i for i in range(n) if b1[i] != b2[i]), n)
print(f"first_diff_byte={off} path1={sys.argv[1]} path2={sys.argv[2]}")
PYEOF
)"
    e2e_assert_eq "d0_address_${name}_identical" "${ADDR1} (${DIFF_OFFSET})" "${ADDR2}"
  else
    e2e_assert_eq "d0_address_${name}_identical" "${ADDR1}" "${ADDR2}"
  fi
done
