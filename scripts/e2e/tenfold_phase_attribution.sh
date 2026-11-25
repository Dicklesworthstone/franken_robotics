#!/usr/bin/env bash
# e2e: bead=N39a-23 milestone=M0 seed=1 needs=cpu
#
# End-to-end verification of ten-fold phase attribution (bead N39a-23, §39.6):
#   1. Worked fixture (§5) and `adjudication_owner` fixture exact comparison.
#   2. Single-pair card known-answer campaigns (Arm B 9,000 s; Arm A 1,500 s).
#   3. 10^5-interval synthetic campaign timed against the 1.0 s TARGET.
#   4. Shuffled 10^5-interval campaign byte-for-byte identical canonical JSON.
#   5. Fourteenth unknown phase name refused (exit 3, record-unknown-field).
#
# All generated fixtures are labelled `synthetic`. Sources scripts/e2e/lib.sh
# (STD-E2E) and writes NDJSON to target/e2e-logs/tenfold_phase_attribution/<utc>.ndjson.
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd -P)"

# shellcheck source=scripts/e2e/lib.sh
source "${REPO_ROOT}/scripts/e2e/lib.sh"

e2e_init "tenfold_phase_attribution" "N39a-23" "$@"

WORK="${E2E_WORK_DIR}"
SYNTH="${WORK}/synthetic"
mkdir -p "${SYNTH}"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}"
CONF_TARGET="${CARGO_TARGET_DIR:-${TARGET_BASE}/frob-tenfold-phase-e2e}"

# ----------------------------------------------------------------------------
# Step 1: Build release frob-tenfold binary (for accurate 10^5-interval timing)
# ----------------------------------------------------------------------------
e2e_step "build_frob_tenfold" "Build frob-tenfold binary and verify phase attribute subcommand in --help"

TENFOLD_BIN="${FROB_TENFOLD_BIN:-}"
if [ -z "${TENFOLD_BIN}" ]; then
  e2e_cmd --name build_frob_tenfold_release -- \
    env "CARGO_TARGET_DIR=${CONF_TARGET}" \
    cargo build --manifest-path "${REPO_ROOT}/Cargo.toml" -p frob-conformance --release --bin frob-tenfold
  e2e_assert_eq "build_frob_tenfold_exit" "${E2E_LAST_EXIT}" "0"
  TENFOLD_BIN="${CONF_TARGET}/release/frob-tenfold"
fi

e2e_assert_file "frob_tenfold_binary_exists" "${TENFOLD_BIN}"

e2e_cmd --name frob_tenfold_help -- "${TENFOLD_BIN}" --help
_has_phase_help="0"
if [ -f "${E2E_LAST_STDOUT}" ] && grep -Fq "phase attribute" "${E2E_LAST_STDOUT}"; then
  _has_phase_help="1"
fi
e2e_assert_eq "frob_tenfold_help_lists_phase_attribute" "${_has_phase_help}" "1"

# ----------------------------------------------------------------------------
# Step 2: Generate all synthetic interval fixtures (labelled synthetic)
# ----------------------------------------------------------------------------
e2e_step "generate_synthetic_fixtures" "Generate synthetic worked, adjudication_owner, single-pair Arm B/A, 10^5, shuffled 10^5, and bad-phase fixtures"

python3 - "${SYNTH}" "${E2E_SEED:-1}" <<'PYEOF'
import json, pathlib, random, sys

synth = pathlib.Path(sys.argv[1])
seed = int(sys.argv[2])

def write_ndjson(path, rows):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        for r in rows:
            f.write(json.dumps(r, sort_keys=True, separators=(",", ":")) + "\n")

def iv(id_, kind, phase, start_ms, end_ms, awaits=None, proc=None):
    obj = {
        "id": id_,
        "kind": kind,
        "phase": phase,
        "start_ms": str(start_ms),
        "end_ms": str(end_ms),
    }
    if awaits is not None:
        obj["awaits"] = awaits
    if proc is not None:
        obj["proc"] = proc
    return obj

# 1a. Worked fixture (§5): t0=0, stop=100_000
worked = [
    iv(1, "turn", "propose", 0, 10_000),
    iv(2, "block", None, 10_000, 60_000, awaits=[3, 4, 5]),
    iv(3, "work", "compile", 10_000, 22_000, proc="job-1"),
    iv(4, "work", "screen", 22_000, 40_000, proc="job-1"),
    iv(5, "work", "train", 12_000, 60_000, proc="job-2"),
    iv(6, "turn", "propose", 60_000, 70_000),
    iv(7, "block", None, 70_000, 90_000, awaits=[8]),
    iv(8, "adjudication", None, 70_000, 90_000),
]
write_ndjson(synth / "synthetic_worked.ndjson", worked)

# 1b. adjudication_owner fixture: t0=20_000, stop=80_000
adj_owner = [
    iv(1, "block", None, 20_000, 80_000, awaits=[2, 3]),
    iv(2, "work", "train", 20_000, 50_000, proc="train-1"),
    iv(3, "adjudication", None, 30_000, 80_000),
]
write_ndjson(synth / "synthetic_adjudication_owner.ndjson", adj_owner)

# 2a. Single-pair card Arm B (clock 9,000 s = 9_000_000 ms)
arm_b = [
    iv(1, "turn", "author", 0, 900_000),
    iv(2, "turn", "propose", 900_000, 1_980_000),
    iv(3, "block", None, 1_980_000, 2_220_000, awaits=[4]),
    iv(4, "work", "check", 1_980_000, 2_220_000, proc="b-chk"),
    iv(5, "block", None, 2_220_000, 7_860_000, awaits=[6, 7, 8]),
    iv(6, "work", "compile", 2_400_000, 2_940_000, proc="b-run"),
    iv(7, "work", "train", 2_940_000, 7_860_000, proc="b-run"),
    iv(8, "work", "screen", 2_220_000, 3_000_000, proc="b-shadow"),
    iv(9, "block", None, 7_860_000, 8_220_000, awaits=[10]),
    iv(10, "work", "evaluate", 7_860_000, 8_220_000, proc="b-eval"),
    iv(11, "turn", "diagnose", 8_220_000, 8_820_000),
    iv(12, "block", None, 8_820_000, 9_000_000, awaits=[13]),
    iv(13, "adjudication", None, 8_820_000, 9_000_000),
]
write_ndjson(synth / "synthetic_arm_b.ndjson", arm_b)

# 2b. Single-pair card Arm A (clock 1,500 s = 1_500_000 ms)
arm_a = [
    iv(1, "turn", "author", 0, 600_000),
    iv(2, "turn", "propose", 600_000, 960_000),
    iv(3, "block", None, 960_000, 980_000, awaits=[4]),
    iv(4, "work", "check", 960_000, 980_000, proc="a-chk"),
    iv(5, "block", None, 980_000, 1_130_000, awaits=[6, 7]),
    iv(6, "work", "compile", 1_022_000, 1_034_000, proc="a-sim"),
    iv(7, "work", "screen", 1_034_000, 1_130_000, proc="a-sim"),
    iv(8, "block", None, 1_130_000, 1_280_000, awaits=[9]),
    iv(9, "work", "train", 1_130_000, 1_280_000, proc="a-tr"),
    iv(10, "block", None, 1_280_000, 1_340_000, awaits=[11]),
    iv(11, "work", "evaluate", 1_280_000, 1_340_000, proc="a-ev"),
    iv(12, "block", None, 1_340_000, 1_380_000, awaits=[13]),
    iv(13, "work", "qualify", 1_340_000, 1_380_000, proc="a-ql"),
    iv(14, "turn", "diagnose", 1_380_000, 1_410_000),
    iv(15, "block", None, 1_410_000, 1_500_000, awaits=[16]),
    iv(16, "adjudication", None, 1_410_000, 1_500_000),
]
write_ndjson(synth / "synthetic_arm_a.ndjson", arm_a)

# 3 & 4. 10^5 intervals (20,000 cycles x 5 intervals/cycle = 100,000 intervals; clock = 2_000_000 ms)
lines_100k = []
phases_cycle = ["check", "compile", "screen", "train", "evaluate", "qualify"]
for c in range(20_000):
    base = c * 100
    b_id = c * 5 + 1
    p_run = phases_cycle[c % len(phases_cycle)]
    # Turn [base, base+15)
    lines_100k.append(f'{{"end_ms":"{base+15}","id":{b_id},"kind":"turn","phase":"propose","start_ms":"{base}"}}\n')
    # Block [base+15, base+95) awaiting work1 (compile+run) and shadow work2
    lines_100k.append(f'{{"awaits":[{b_id+2},{b_id+3},{b_id+4}],"end_ms":"{base+95}","id":{b_id+1},"kind":"block","phase":null,"start_ms":"{base+15}"}}\n')
    # Multi-phase owner job: compile [base+20, base+40), run [base+40, base+95)
    lines_100k.append(f'{{"end_ms":"{base+40}","id":{b_id+2},"kind":"work","phase":"compile","proc":"j{c}","start_ms":"{base+20}"}}\n')
    lines_100k.append(f'{{"end_ms":"{base+95}","id":{b_id+3},"kind":"work","phase":"{p_run}","proc":"j{c}","start_ms":"{base+40}"}}\n')
    # Shadow job [base+15, base+60)
    lines_100k.append(f'{{"end_ms":"{base+60}","id":{b_id+4},"kind":"work","phase":"screen","proc":"s{c}","start_ms":"{base+15}"}}\n')

assert len(lines_100k) == 100_000
with open(synth / "synthetic_100k.ndjson", "w", encoding="utf-8", newline="\n") as f:
    f.writelines(lines_100k)

rng = random.Random(seed)
shuffled_100k = list(lines_100k)
rng.shuffle(shuffled_100k)
with open(synth / "synthetic_100k_shuffled.ndjson", "w", encoding="utf-8", newline="\n") as f:
    f.writelines(shuffled_100k)

# 5. Fourteenth unknown phase name
write_ndjson(synth / "synthetic_bad_phase14.ndjson", [
    iv(1, "work", "rehearse", 0, 10_000, proc="bad"),
])

manifest = {
    "kind": "synthetic",
    "bead": "N39a-23",
    "seed": seed,
    "files": [
        "synthetic_worked.ndjson",
        "synthetic_adjudication_owner.ndjson",
        "synthetic_arm_b.ndjson",
        "synthetic_arm_a.ndjson",
        "synthetic_100k.ndjson",
        "synthetic_100k_shuffled.ndjson",
        "synthetic_bad_phase14.ndjson",
    ],
}
(synth / "synthetic_manifest.json").write_text(
    json.dumps(manifest, sort_keys=True, indent=2) + "\n", encoding="utf-8"
)
PYEOF

e2e_assert_file "synthetic_manifest_exists" "${SYNTH}/synthetic_manifest.json"
e2e_assert_file "synthetic_100k_exists" "${SYNTH}/synthetic_100k.ndjson"

# Helper to verify all 14 phase buckets + exact sum == clock, logging side-by-side table
# and printing offending segment + covering intervals on any mismatch.
verify_table() {
  local label="$1"
  local ndjson_path="$2"
  local t0="$3"
  local stop="$4"
  local expected_json="$5"

  e2e_cmd --name "attribute_${label}" -- \
    "${TENFOLD_BIN}" phase attribute "${ndjson_path}" --t0 "${t0}" --stop "${stop}"
  e2e_assert_eq "${label}_exit_0" "${E2E_LAST_EXIT}" "0"
  e2e_assert_json "${label}_auth_false" "/execution_authorized" "false"
  e2e_assert_json "${label}_reality_noclaim" "/reality" "no-claim"

  local report_txt="${WORK}/${label}_comparison.txt"
  local cmp_ok="1"
  if ! python3 - "${E2E_LAST_STDOUT}" "${ndjson_path}" "${t0}" "${stop}" "${expected_json}" "${report_txt}" <<'PYEOF'
import json, sys

actual = json.loads(open(sys.argv[1], encoding="utf-8").read())
ndjson_path = sys.argv[2]
t0 = int(sys.argv[3])
stop = int(sys.argv[4])
expected = json.loads(sys.argv[5])
report_path = sys.argv[6]

phases_order = [
    "author", "propose", "check", "synthesize", "compile", "screen", "train",
    "evaluate", "qualify", "certify", "diagnose", "wait", "adjudicate", "unattributed"
]

lines = []
mismatches = []
sum_ms = 0
for p in phases_order:
    obs_s = actual["phases"][p]
    obs_ms = int(actual["phases_ms"][p])
    exp_s = expected.get(p, "0")
    sum_ms += obs_ms
    mark = "OK" if obs_s == exp_s else "MISMATCH"
    lines.append(f"  {p:14s} observed={obs_s:>10s} s   expected={exp_s:>10s} s   [{mark}]")
    if obs_s != exp_s:
        mismatches.append((p, obs_s, exp_s))

clock_ms = int(actual["clock_ms"])
lines.append(f"  {'SUM':14s} sum_ms={sum_ms}   clock_ms={clock_ms}   expected_clock={stop - t0}")

if sum_ms != clock_ms or clock_ms != (stop - t0):
    mismatches.append(("SUM", str(sum_ms), str(stop - t0)))

if mismatches:
    lines.append("FAILURE DIAGNOSTIC — intervals in fixture:")
    with open(ndjson_path, encoding="utf-8") as f:
        for raw in f:
            raw = raw.strip()
            if not raw:
                continue
            iv = json.loads(raw)
            lines.append(
                f"  id={iv['id']} kind={iv['kind']} phase={iv['phase']} "
                f"[{iv['start_ms']}, {iv['end_ms']}) awaits={iv.get('awaits', [])} proc={iv.get('proc')}"
            )

open(report_path, "w", encoding="utf-8").write("\n".join(lines) + "\n")
if mismatches:
    sys.stderr.write("\n".join(lines) + "\n")
    sys.exit(1)
PYEOF
  then
    cmp_ok="0"
  fi
  e2e_assert_eq "${label}_table_exact_and_sums_to_clock" "${cmp_ok}" "1"
}

# ----------------------------------------------------------------------------
# Step 3: Worked fixture (§5) and adjudication_owner fixture
# ----------------------------------------------------------------------------
e2e_step "worked_and_adjudication_owner" "Verify worked fixture (§5) and adjudication_owner fixture exact tables"

verify_table "worked_fixture" "${SYNTH}/synthetic_worked.ndjson" "0" "100000" \
  '{"propose":"20","wait":"2","train":"48","adjudicate":"20","unattributed":"10"}'

verify_table "adjudication_owner" "${SYNTH}/synthetic_adjudication_owner.ndjson" "20000" "80000" \
  '{"wait":"10","adjudicate":"50"}'

# ----------------------------------------------------------------------------
# Step 4: Single-pair card known answers (Arm B = 9,000 s; Arm A = 1,500 s)
# ----------------------------------------------------------------------------
e2e_step "single_pair_card_known_answers" "Verify single-pair card Arm B (9,000 s) and Arm A (1,500 s) known-answer phase tables"

verify_table "arm_b_known_answer" "${SYNTH}/synthetic_arm_b.ndjson" "0" "9000000" \
  '{"author":"900","propose":"1080","check":"240","synthesize":"0","compile":"540","screen":"0","train":"4920","evaluate":"360","qualify":"0","certify":"0","diagnose":"600","wait":"180","adjudicate":"180","unattributed":"0"}'

verify_table "arm_a_known_answer" "${SYNTH}/synthetic_arm_a.ndjson" "0" "1500000" \
  '{"author":"600","propose":"360","check":"20","synthesize":"0","compile":"12","screen":"96","train":"150","evaluate":"60","qualify":"40","certify":"0","diagnose":"30","wait":"42","adjudicate":"90","unattributed":"0"}'

# ----------------------------------------------------------------------------
# Step 5: 10^5 intervals timed against 1.0 s TARGET + shuffled byte-identity
# ----------------------------------------------------------------------------
e2e_step "scale_100k_and_order_invariance" "Run 10^5 intervals against 1.0 s TARGET and verify byte-identical canonical JSON after shuffling"

OUT_100K="${WORK}/out_100k.json"
OUT_100K_SHUF="${WORK}/out_100k_shuffled.json"

WALL_100K="$(python3 - "${TENFOLD_BIN}" "${SYNTH}/synthetic_100k.ndjson" "${OUT_100K}" <<'PYEOF'
import subprocess, sys, time

bin_path, ndjson_path, out_path = sys.argv[1:4]
t0 = time.perf_counter()
res = subprocess.run(
    [bin_path, "phase", "attribute", ndjson_path, "--t0", "0", "--stop", "2000000"],
    capture_output=True,
    check=False,
)
dt = time.perf_counter() - t0
if res.returncode != 0:
    sys.stderr.write(f"100k run failed exit={res.returncode}: {res.stderr.decode('utf-8', 'replace')}\n")
    sys.exit(res.returncode)
open(out_path, "wb").write(res.stdout)
print(f"{dt:.6f}")
PYEOF
)"

e2e_metric "attribute_100k_wall_s" "${WALL_100K}" "s" "<" "1.0"

e2e_cmd --name attribute_100k_canonical -- \
  "${TENFOLD_BIN}" phase attribute "${SYNTH}/synthetic_100k.ndjson" --t0 0 --stop 2000000
cp "${E2E_LAST_STDOUT}" "${OUT_100K}"
e2e_assert_json "scale_100k_clock_ms" "/clock_ms" "2000000"
e2e_assert_json "scale_100k_propose" "/phases/propose" "300"
e2e_assert_json "scale_100k_wait" "/phases/wait" "100"
e2e_assert_json "scale_100k_unattributed" "/phases/unattributed" "100"

e2e_cmd --name attribute_100k_shuffled -- \
  "${TENFOLD_BIN}" phase attribute "${SYNTH}/synthetic_100k_shuffled.ndjson" --t0 0 --stop 2000000
cp "${E2E_LAST_STDOUT}" "${OUT_100K_SHUF}"

SHA_ORIG="$("${FROB_E2E_TOOL}" cmp sha256 "${OUT_100K}")"
SHA_SHUF="$("${FROB_E2E_TOOL}" cmp sha256 "${OUT_100K_SHUF}")"
e2e_assert_digest_eq "scale_100k_shuffled_byte_identical" "${SHA_ORIG}" "${SHA_SHUF}"

# ----------------------------------------------------------------------------
# Step 6: Fourteenth unknown phase name refused at read time (exit 3)
# ----------------------------------------------------------------------------
e2e_step "unknown_phase_refusal" "Verify a 14th phase name is refused at read time with exit 3 and record-unknown-field"

e2e_cmd --name refuse_phase14 --expect-exit "3" -- \
  "${TENFOLD_BIN}" phase attribute "${SYNTH}/synthetic_bad_phase14.ndjson" --t0 0 --stop 10000
e2e_assert_eq "phase14_exit_3" "${E2E_LAST_EXIT}" "3"
e2e_assert_json "phase14_reason" "/refusal/reason" "record-unknown-field"
e2e_assert_json "phase14_pointer" "/refusal/pointer" "/phase"
