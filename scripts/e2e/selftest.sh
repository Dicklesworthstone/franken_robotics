#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=gauntlet,harness seed=1 needs=cpu
#
# Harness self-test and negative-control suite (bead STD-E2E).
# Exercises scripts/e2e/lib.sh, scripts/e2e/run-all.sh, schemas/e2e-log.schema.json,
# all fixture scripts under scripts/e2e/selftest/st_*.sh, review notes 5/6/7,
# linter negative controls, idempotence, isolation, session token redaction,
# tier target validation, single-envelope check, runner, and gate report.
#
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "selftest" "STD-E2E" "$@"

SELFTEST_DIR="${SCRIPT_DIR}/selftest"
INNER_LOGS="${E2E_WORK_DIR}/inner-logs"
INNER_SCRATCH="${E2E_WORK_DIR}/inner-scratch"
mkdir -p "${INNER_LOGS}" "${INNER_SCRATCH}"

# Helper: read the latest log path for a sub-script under a given logs root.
latest_log_for() {
  local logs_root="$1" sname="$2"
  local latest="${logs_root}/${sname}/LATEST"
  if [ -f "$latest" ]; then
    local stem
    stem="$(cat "$latest")"
    printf '%s/%s/%s.ndjson' "$logs_root" "$sname" "$stem"
  fi
}

# Helper: read the latest transcript path for a sub-script under a given logs root.
latest_transcript_for() {
  local logs_root="$1" sname="$2"
  local latest="${logs_root}/${sname}/LATEST"
  if [ -f "$latest" ]; then
    local stem
    stem="$(cat "$latest")"
    printf '%s/%s/%s.transcript.txt' "$logs_root" "$sname" "$stem"
  fi
}

# Helper: read the last line of a file (summary line of an NDJSON log).
last_line_of() {
  tail -n 1 "$1"
}

# ---------------------------------------------------------------------------
# Step 1: Schema match, std-only source scan, Cargo.toml & CONTRACT.md checks
# ---------------------------------------------------------------------------
e2e_step "schema_and_packaging" "Verify frob-e2e schema matches schemas/e2e-log.schema.json, std-only imports, and manifest/CONTRACT"

SCHEMA_OUT="${E2E_WORK_DIR}/fresh-schema.json"
e2e_cmd --name frob_e2e_schema -- "${FROB_E2E_TOOL}" schema
cp "${E2E_LAST_STDOUT}" "${SCHEMA_OUT}"
SCHEMA_REPO="${E2E_REPO_ROOT}/schemas/e2e-log.schema.json"
e2e_assert_file "schema_file_exists" "${SCHEMA_REPO}"
if cmp -s "${SCHEMA_OUT}" "${SCHEMA_REPO}"; then
  e2e_assert_eq "schema_matches_tool_exact" "identical" "identical"
else
  e2e_assert_eq "schema_matches_tool_exact" "differs" "identical"
fi

# Source scan: frob-e2e modules must import nothing outside std / crate / super / self / frob_conformance.
STD_SCAN_BAD="$(python3 - "${E2E_REPO_ROOT}/crates/frob-conformance" <<'PY'
import os, re, sys
root = sys.argv[1]
allowed = {"std", "core", "alloc", "crate", "super", "self", "frob_conformance"}
paths = [os.path.join(root, "src", "bin", "frob-e2e.rs")]
e2e_dir = os.path.join(root, "src", "e2e")
for dp, _dn, fns in os.walk(e2e_dir):
    for fn in sorted(fns):
        if fn.endswith(".rs"):
            paths.append(os.path.join(dp, fn))
bad = []
use_re = re.compile(r"^\s*(?:pub\s+)?use\s+([A-Za-z_][A-Za-z0-9_]*)\s*::")
ext_re = re.compile(r"^\s*extern\s+crate\s+([A-Za-z_][A-Za-z0-9_]*)")
for p in paths:
    if not os.path.isfile(p):
        bad.append(f"missing:{p}")
        continue
    with open(p, "r", encoding="utf-8") as fh:
        for ln, line in enumerate(fh, 1):
            m = use_re.match(line) or ext_re.match(line)
            if m and m.group(1) not in allowed:
                bad.append(f"{os.path.relpath(p, root)}:{ln}:{m.group(1)}")
print(",".join(bad))
PY
)"
e2e_assert_eq "tool_imports_std_only" "${STD_SCAN_BAD}" ""

# Cargo.toml declares autobins = false and [[bin]] frob-e2e; CONTRACT.md lists frob-e2e.
PKG_OK="$(python3 - "${E2E_REPO_ROOT}/crates/frob-conformance" <<'PY'
import os, sys, tomllib
root = sys.argv[1]
manifest = tomllib.load(open(os.path.join(root, "Cargo.toml"), "rb"))
autobins_ok = manifest.get("package", {}).get("autobins") is False
bin_ok = any(
    b.get("name") == "frob-e2e" and b.get("path") == "src/bin/frob-e2e.rs"
    for b in manifest.get("bin", [])
)
contract = open(os.path.join(root, "CONTRACT.md"), "r", encoding="utf-8").read()
contract_ok = "frob-e2e" in contract and "src/bin/frob-e2e.rs" in contract
print("ok" if (autobins_ok and bin_ok and contract_ok) else f"autobins={autobins_ok},bin={bin_ok},contract={contract_ok}")
PY
)"
e2e_assert_eq "manifest_and_contract_declare_frob_e2e" "${PKG_OK}" "ok"

# ---------------------------------------------------------------------------
# Step 2: st_pass, Review Note 6 (e2e_scene_ready), idempotence & isolation
# ---------------------------------------------------------------------------
e2e_step "st_pass_idempotence_isolation" "Run st_pass twice for idempotence, check e2e_scene_ready skip, and verify state isolation"

IDEMP_LOGS="${INNER_LOGS}/idemp"
IDEMP_SCRATCH="${INNER_SCRATCH}/idemp"
mkdir -p "${IDEMP_LOGS}" "${IDEMP_SCRATCH}"

e2e_cmd --name st_pass_run_a -- \
  env "E2E_LOGS_ROOT=${IDEMP_LOGS}" "E2E_SCRATCH_ROOT=${IDEMP_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "FROB_E2E_SEED=1" \
  bash "${SELFTEST_DIR}/st_pass.sh"
PASS_LOG_A="$(latest_log_for "${IDEMP_LOGS}" "st_pass")"
e2e_assert_file "st_pass_a_log_exists" "${PASS_LOG_A}"
e2e_cmd --name lint_st_pass_a -- "${FROB_E2E_TOOL}" lint "${PASS_LOG_A}"

# Review note 6: e2e_scene_ready emitted a logged skip naming scene.accept.
SCENE_READY_OK="$(python3 - "${PASS_LOG_A}" <<'PY'
import json, sys
found = False
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "assert" and obj.get("status") == "skip" and "scene.accept" in json.dumps(obj):
        found = True
        break
print("ok" if found else "missing")
PY
)"
e2e_assert_eq "scene_ready_noop_skip_logged" "${SCENE_READY_OK}" "ok"

e2e_cmd --name st_pass_run_b -- \
  env "E2E_LOGS_ROOT=${IDEMP_LOGS}" "E2E_SCRATCH_ROOT=${IDEMP_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "FROB_E2E_SEED=1" \
  bash "${SELFTEST_DIR}/st_pass.sh"
PASS_LOG_B="$(latest_log_for "${IDEMP_LOGS}" "st_pass")"
e2e_cmd --name lint_st_pass_b -- "${FROB_E2E_TOOL}" lint "${PASS_LOG_B}"

# Idempotence: strip ts, wall_ms, body_sha256 and normalize this run's own run_id / artifacts stem.
IDEMP_DIFF="$(python3 - "${PASS_LOG_A}" "${PASS_LOG_B}" <<'PY'
import json, os, sys
def norm(path):
    lines = [json.loads(l) for l in open(path, "r", encoding="utf-8") if l.strip()]
    rid = lines[0]["run_id"]
    stem = os.path.basename(path)[:-len(".ndjson")]
    out = []
    for obj in lines:
        for k in ("ts", "wall_ms", "body_sha256"):
            obj.pop(k, None)
        if obj.get("run_id") == rid:
            obj["run_id"] = "<RUN_ID>"
        for pk in ("stdout_path", "stderr_path"):
            if isinstance(obj.get(pk), str):
                obj[pk] = obj[pk].replace(f"/{stem}.artifacts/", "/<UTC>.artifacts/")
        if isinstance(obj.get("cwd"), str):
            obj["cwd"] = obj["cwd"].replace(rid, "<RUN_ID>")
        s = json.dumps(obj, sort_keys=True, separators=(",", ":"))
        out.append(s)
    return out
a, b = norm(sys.argv[1]), norm(sys.argv[2])
if a == b:
    print("equal")
else:
    for i, (la, lb) in enumerate(zip(a, b)):
        if la != lb:
            print(f"line {i}: {la[:160]} != {lb[:160]}")
            break
    else:
        print(f"len {len(a)} != {len(b)}")
PY
)"
e2e_assert_eq "st_pass_idempotent_across_runs" "${IDEMP_DIFF}" "equal"

# Isolation: FROB_STATE mode 0700 under scratch, cwd is work/, concurrent runs share no file.
CONC_LOGS="${INNER_LOGS}/concurrent"
CONC_SCRATCH="${INNER_SCRATCH}/concurrent"
mkdir -p "${CONC_LOGS}" "${CONC_SCRATCH}"
env "E2E_LOGS_ROOT=${CONC_LOGS}" "E2E_SCRATCH_ROOT=${CONC_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "FROB_E2E_SEED=1" \
  bash "${SELFTEST_DIR}/st_pass.sh" >/dev/null 2>&1 &
PID_C1=$!
env "E2E_LOGS_ROOT=${CONC_LOGS}" "E2E_SCRATCH_ROOT=${CONC_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "FROB_E2E_SEED=1" \
  bash "${SELFTEST_DIR}/st_pass.sh" >/dev/null 2>&1 &
PID_C2=$!
wait "$PID_C1"
RC_C1=$?
wait "$PID_C2"
RC_C2=$?
e2e_assert_eq "concurrent_run_1_exit" "${RC_C1}" "0"
e2e_assert_eq "concurrent_run_2_exit" "${RC_C2}" "0"

CONC_CHECK="$(python3 - "${CONC_LOGS}/st_pass" "${CONC_SCRATCH}/st_pass" <<'PY'
import glob, json, os, stat, sys
logs = sorted(glob.glob(os.path.join(sys.argv[1], "*.ndjson")))
rids = set()
for p in logs:
    first = json.loads(open(p, "r", encoding="utf-8").readline())
    rids.add(first["run_id"])
modes_ok = True
work_ok = True
for rid in rids:
    st_dir = os.path.join(sys.argv[2], rid, "state")
    wk_dir = os.path.join(sys.argv[2], rid, "work")
    if not os.path.isdir(st_dir) or (stat.S_IMODE(os.stat(st_dir).st_mode) != 0o700):
        modes_ok = False
    if not os.path.isdir(wk_dir):
        work_ok = False
print("ok" if (len(logs) >= 2 and len(rids) == len(logs) and modes_ok and work_ok) else f"logs={len(logs)},rids={len(rids)},modes={modes_ok},work={work_ok}")
PY
)"
e2e_assert_eq "concurrent_runs_isolated_0700" "${CONC_CHECK}" "ok"

# ---------------------------------------------------------------------------
# Step 3: st_fail_assert (Known Answer: passed=2, failed=1, first_diff+repro)
# ---------------------------------------------------------------------------
e2e_step "st_fail_assert" "Verify failing assertion exits 1 with KA passed=2 failed=1 and first_diff"

e2e_cmd --name st_fail_assert --expect-exit 1 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_fail_assert.sh"
e2e_assert_eq "st_fail_assert_exit" "${E2E_LAST_EXIT}" "1"
FAIL_LOG="$(latest_log_for "${INNER_LOGS}" "st_fail_assert")"
e2e_cmd --name lint_st_fail_assert -- "${FROB_E2E_TOOL}" lint "${FAIL_LOG}"

FAIL_SUM="${E2E_WORK_DIR}/fail_summary.json"
last_line_of "${FAIL_LOG}" > "${FAIL_SUM}"
e2e_assert_eq "ka_st_fail_assert_passed" "$("${FROB_E2E_TOOL}" get --file "${FAIL_SUM}" /asserts_passed)" "2"
e2e_assert_eq "ka_st_fail_assert_failed" "$("${FROB_E2E_TOOL}" get --file "${FAIL_SUM}" /asserts_failed)" "1"

FAIL_DIFF_OK="$(python3 - "${FAIL_LOG}" <<'PY'
import json, sys
ok = False
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "assert" and obj.get("status") == "fail":
        fd = obj.get("first_diff")
        rp = obj.get("repro")
        if isinstance(fd, dict) and all(k in fd for k in ("path", "observed", "expected")) and isinstance(rp, dict) and "cmd_seq" in rp:
            ok = True
print("ok" if ok else "missing")
PY
)"
e2e_assert_eq "st_fail_assert_has_first_diff_and_repro" "${FAIL_DIFF_OK}" "ok"

# ---------------------------------------------------------------------------
# Step 4: st_no_assert (exit 2) and st_skip_only (exit 77 -> report not-run)
# ---------------------------------------------------------------------------
e2e_step "st_no_assert_and_skip_only" "Verify zero assertions exits 2 and skip-only exits 77"

e2e_cmd --name st_no_assert --expect-exit 2 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_no_assert.sh"
e2e_assert_eq "st_no_assert_exit" "${E2E_LAST_EXIT}" "2"

e2e_cmd --name st_skip_only --expect-exit 77 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_skip_only.sh"
e2e_assert_eq "st_skip_only_exit" "${E2E_LAST_EXIT}" "77"

# ---------------------------------------------------------------------------
# Step 5: Private canary, token canary, private path redaction & forbidden fields
# ---------------------------------------------------------------------------
e2e_step "redaction_and_canaries" "Verify private/token canaries fail lint, private paths redact, and forbidden members fail emit"

e2e_cmd --name st_private_leak --expect-exit 0 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_private_leak.sh"
PRIV_LOG="$(latest_log_for "${INNER_LOGS}" "st_private_leak")"
e2e_expect_caught "private_canary_in_log" -- "${FROB_E2E_TOOL}" lint "${PRIV_LOG}"

e2e_cmd --name st_token_leak --expect-exit 0 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_token_leak.sh"
TOK_LOG="$(latest_log_for "${INNER_LOGS}" "st_token_leak")"
e2e_expect_caught "token_canary_in_log" -- "${FROB_E2E_TOOL}" lint "${TOK_LOG}"

# Private path redaction: path under FROB_PRIVATE_ROOT becomes <private:12hex>.
PRIV_ROOT="${E2E_WORK_DIR}/fake_private_root"
mkdir -p "${PRIV_ROOT}/home_scan"
printf 'secret-scan\n' > "${PRIV_ROOT}/home_scan/room.dat"
PRIV_PATH_SCRIPT="${E2E_WORK_DIR}/st_priv_path.sh"
cat > "${PRIV_PATH_SCRIPT}" <<EOF
#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_priv_path" "STD-E2E"
e2e_step "priv_path" "Run command with argument under FROB_PRIVATE_ROOT"
e2e_cmd --name echo_priv -- printf '%s\n' "${PRIV_ROOT}/home_scan/room.dat"
e2e_assert_eq "cmd_ok" "\${E2E_LAST_EXIT}" "0"
EOF
chmod +x "${PRIV_PATH_SCRIPT}"
e2e_cmd --name run_priv_path_script -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "FROB_PRIVATE_ROOT=${PRIV_ROOT}" \
  bash "${PRIV_PATH_SCRIPT}"
PRIV_PATH_LOG="$(latest_log_for "${INNER_LOGS}" "st_priv_path")"
e2e_cmd --name lint_priv_path_log -- "${FROB_E2E_TOOL}" lint --private-root "${PRIV_ROOT}" "${PRIV_PATH_LOG}"
PRIV_REDACT_OK="$(python3 - "${PRIV_PATH_LOG}" "${PRIV_ROOT}" <<'PY'
import re, sys
txt = open(sys.argv[1], "r", encoding="utf-8").read()
raw = sys.argv[2]
has_raw = raw in txt
has_tag = bool(re.search(r"<private:[0-9a-f]{12}>", txt))
print("ok" if (not has_raw and has_tag) else f"has_raw={has_raw},has_tag={has_tag}")
PY
)"
e2e_assert_eq "private_path_redacted_12hex" "${PRIV_REDACT_OK}" "ok"

# Forbidden private member name (`voxels`) and >4096-byte string refused by emit.
# Generate the 5,000-byte string inside the child shell so selftest's own cmdline stays <4096 bytes.
BAD_EMIT_LOG="${E2E_WORK_DIR}/bad_emit.ndjson"
: > "${BAD_EMIT_LOG}"
e2e_expect_caught "emit_refuses_voxels_member" -- \
  "${FROB_E2E_TOOL}" emit --log "${BAD_EMIT_LOG}" --event assert \
  --run-id "20261124T000000Z-00000000" --script "selftest" --bead "STD-E2E" --step "s" --seed "1" \
  --raw-json '{"name":"x","kind":"eq","status":"pass","voxels":[1,2,3]}'

# shellcheck disable=SC2016
e2e_expect_caught "emit_refuses_5000_byte_string" -- \
  bash -c 'long="$(python3 -c "print(\"A\" * 5000)")"; exec "$1" emit --log "$2" --event assert --run-id "20261124T000000Z-00000000" --script "selftest" --bead "STD-E2E" --step "s" --seed "1" --name "oversize" --kind "eq" --status "pass" --observed "$long" --expected "ok"' \
  _ "${FROB_E2E_TOOL}" "${BAD_EMIT_LOG}"

# ---------------------------------------------------------------------------
# Step 6: Refusals (unexpected FROB-REF-805 + hint, expect_refusal, repairs)
# ---------------------------------------------------------------------------
e2e_step "refusals_and_repairs" "Verify unexpected refusal, FROB-REF-805 hint, e2e_expect_refusal, and repairs order"

e2e_cmd --name st_unexpected_refusal --expect-exit 1 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_unexpected_refusal.sh"
e2e_assert_eq "st_unexpected_refusal_exit" "${E2E_LAST_EXIT}" "1"
REF_TX="$(latest_transcript_for "${INNER_LOGS}" "st_unexpected_refusal")"
# shellcheck disable=SC2016
if [ -f "${REF_TX}" ] && grep -Fq 'hint: pass --key "$(e2e_key <label>)"' "${REF_TX}"; then
  e2e_assert_eq "refusal_805_hint_in_transcript" "present" "present"
else
  e2e_assert_eq "refusal_805_hint_in_transcript" "missing" "present"
fi

# Matching e2e_expect_refusal + e2e_assert_repairs_order
REF_ENV='{"schema":"frob.envelope/1","verb":"sim.run","ok":false,"exit":3,"refusal":{"code":"FROB-REF-802","exit":3,"message":"fixture missing","pointer":"/design","observed":"fixture-unknown:foo","required":"known fixture","unit":"","retry":false,"repairs":[{"code":"CHECK_FIXTURE","summary":"verify name"},{"code":"LIST_FIXTURES","summary":"list"}]}}'
e2e_expect_refusal FROB-REF-802 3 -- bash -c "printf '%s\n' '${REF_ENV}'; exit 3"
e2e_assert_repairs_order "repairs_rank_order" "CHECK_FIXTURE,LIST_FIXTURES"

# Negative control: e2e_expect_refusal fails on a success envelope and on wrong code
REF_NEG_SUCCESS="${E2E_WORK_DIR}/st_ref_neg_success.sh"
cat > "${REF_NEG_SUCCESS}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_ref_neg_success" "STD-E2E"
e2e_step "s" "Expect refusal on success envelope must fail"
e2e_expect_refusal FROB-REF-802 3 -- bash -c 'printf "{\"schema\":\"frob.envelope/1\",\"verb\":\"sim.run\",\"ok\":true,\"exit\":0,\"result\":{}}\n"; exit 0'
EOF
chmod +x "${REF_NEG_SUCCESS}"
e2e_expect_caught "expect_refusal_fails_on_success" -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${REF_NEG_SUCCESS}"

REF_NEG_WRONG="${E2E_WORK_DIR}/st_ref_neg_wrong.sh"
cat > "${REF_NEG_WRONG}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_ref_neg_wrong" "STD-E2E"
e2e_step "s" "Expect refusal on wrong code must fail"
e2e_expect_refusal FROB-REF-801 3 -- bash -c 'printf "{\"schema\":\"frob.envelope/1\",\"verb\":\"sim.run\",\"ok\":false,\"exit\":3,\"refusal\":{\"code\":\"FROB-REF-802\",\"exit\":3,\"message\":\"other\",\"pointer\":\"/\",\"observed\":null,\"required\":\"\",\"unit\":\"\",\"retry\":false,\"repairs\":[]}}\n"; exit 3'
EOF
chmod +x "${REF_NEG_WRONG}"
e2e_expect_caught "expect_refusal_fails_on_wrong_code" -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${REF_NEG_WRONG}"

# ---------------------------------------------------------------------------
# Step 7: st_plant, run-all.sh --verify-plants & empty selection (exit 2)
# ---------------------------------------------------------------------------
e2e_step "plants_and_runner_controls" "Verify st_plant, --verify-plants positive & negative control, and empty selection exit 2"

e2e_cmd --name st_plant_clean --expect-exit 0 -- \
  env -u E2E_PLANT_FAILURE "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_plant.sh"

e2e_cmd --name st_plant_planted --expect-exit 1 -- \
  env "E2E_PLANT_FAILURE=boundary" "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_plant.sh"
e2e_assert_eq "st_plant_planted_exit" "${E2E_LAST_EXIT}" "1"

e2e_cmd --name run_all_verify_plants -- \
  env "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  bash "${SCRIPT_DIR}/run-all.sh" \
    --scripts-dir "${SELFTEST_DIR}" \
    --logs-dir "${INNER_LOGS}/verify_plants" \
    --script st_plant \
    --verify-plants

# Negative control: --verify-plants catches a script whose declared plant does NOT make it fail.
UNWIRED_DIR="${E2E_WORK_DIR}/unwired_scripts"
mkdir -p "${UNWIRED_DIR}"
cat > "${UNWIRED_DIR}/st_unwired_plant.sh" <<EOF
#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu plants=unwired
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_unwired_plant" "STD-E2E"
e2e_step "noop" "Declares plant unwired but never fails"
e2e_assert_eq "always_pass" "1" "1"
EOF
chmod +x "${UNWIRED_DIR}/st_unwired_plant.sh"
e2e_expect_caught "verify_plants_catches_unwired_plant" -- \
  env "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  bash "${SCRIPT_DIR}/run-all.sh" \
    --scripts-dir "${UNWIRED_DIR}" \
    --logs-dir "${INNER_LOGS}/unwired_plants" \
    --verify-plants

# Negative control: --verify-plants refuses a planted run when an unrelated clean-path
# assertion also fails (the old "any failed assert names the plant" rule passed this).
MASKED_DIR="${E2E_WORK_DIR}/masked_plant_scripts"
mkdir -p "${MASKED_DIR}"
cat > "${MASKED_DIR}/st_masked_plant.sh" <<EOF
#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu plants=boundary
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_masked_plant" "STD-E2E"
e2e_step "clean_regression" "Unrelated clean-path assertion fails alongside the plant"
e2e_assert_eq "clean_path_invariant" "broken" "clean"
e2e_step "plant_check" "Planted assertion also fails naming boundary"
if e2e_plant "boundary"; then
  e2e_assert_eq "plant:boundary" "planted-defect" "clean-boundary"
else
  e2e_assert_eq "plant:boundary" "clean-boundary" "clean-boundary"
fi
EOF
chmod +x "${MASKED_DIR}/st_masked_plant.sh"
e2e_expect_caught "verify_plants_refuses_unrelated_failure" -- \
  env "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  bash "${SCRIPT_DIR}/run-all.sh" \
    --scripts-dir "${MASKED_DIR}" \
    --logs-dir "${INNER_LOGS}/masked_plants" \
    --verify-plants

# Empty selection exits 2.
e2e_cmd --name run_all_empty_selection --expect-exit 2 -- \
  env "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SCRIPT_DIR}/run-all.sh" \
    --scripts-dir "${SELFTEST_DIR}" \
    --logs-dir "${INNER_LOGS}/empty_sel" \
    --milestone M8
e2e_assert_eq "run_all_empty_selection_exit" "${E2E_LAST_EXIT}" "2"

# ---------------------------------------------------------------------------
# Step 8: st_killed (missing summary) and st_self_record (self-recording)
# ---------------------------------------------------------------------------
e2e_step "killed_and_self_record" "Verify SIGKILL script fails lint and record_gate fails lint as self-recording"

e2e_cmd --name st_killed --expect-exit 137,9 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_killed.sh"
KILLED_LOG="$(latest_log_for "${INNER_LOGS}" "st_killed")"
e2e_expect_caught "killed_missing_summary" -- "${FROB_E2E_TOOL}" lint "${KILLED_LOG}"

e2e_cmd --name st_self_record --expect-exit 0 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_self_record.sh"
SELFREC_LOG="$(latest_log_for "${INNER_LOGS}" "st_self_record")"
e2e_expect_caught "self_recording_caught" -- "${FROB_E2E_TOOL}" lint "${SELFREC_LOG}"
if grep -q "self-recording" "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" 2>/dev/null; then
  e2e_assert_eq "self_recording_named" "named" "named"
else
  e2e_assert_eq "self_recording_named" "unnamed" "named"
fi

# ---------------------------------------------------------------------------
# Step 9: Keys, verbatim argv, and fixtures (including source byte-identity)
# ---------------------------------------------------------------------------
e2e_step "keys_argv_and_fixtures" "Verify key reuse exit 2, verbatim argv logging, and fixture copy/missing behavior"

e2e_cmd --name st_key_reuse --expect-exit 2 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_key_reuse.sh"
e2e_assert_eq "st_key_reuse_exit" "${E2E_LAST_EXIT}" "2"

# Verbatim argv (spaces, quotes, empty argument)
e2e_cmd --name verbatim_argv_check -- printf '[%s]|[%s]|[%s]\n' 'hello world' 'a"b'\''c' ''
e2e_assert_eq "verbatim_argv_stdout" "$(cat "${E2E_LAST_STDOUT}")" '[hello world]|[a"b'\''c]|[]'

# Fixture source sha256 before vs after st_fixture (which copies & mutates copy, then exits 2 on missing)
FIX_BEFORE="$("${FROB_E2E_TOOL}" cmp sha256 "${SELFTEST_DIR}/fixtures")"
e2e_cmd --name st_fixture --expect-exit 2 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_fixture.sh"
e2e_assert_eq "st_fixture_missing_exit" "${E2E_LAST_EXIT}" "2"
FIX_AFTER="$("${FROB_E2E_TOOL}" cmp sha256 "${SELFTEST_DIR}/fixtures")"
e2e_assert_digest_eq "fixture_source_unchanged" "${FIX_BEFORE}" "${FIX_AFTER}"

# ---------------------------------------------------------------------------
# Step 10: Review Note 5 (--observe metric with not-met exits 0 and lints ok)
# ---------------------------------------------------------------------------
e2e_step "observe_metric_convention_15" "Verify --observe metric not-met exits 0 and passes lint (Review Note 5)"

e2e_cmd --name st_observe_not_met --expect-exit 0 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" \
  bash "${SELFTEST_DIR}/st_observe_not_met.sh"
OBS_LOG="$(latest_log_for "${INNER_LOGS}" "st_observe_not_met")"
e2e_cmd --name lint_st_observe_not_met -- "${FROB_E2E_TOOL}" lint "${OBS_LOG}"

OBS_CHECK="$(python3 - "${OBS_LOG}" <<'PY'
import json, sys
ok = False
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "metric" and obj.get("name") == "pilot_falsifier_ratio":
        if obj.get("asserted") is False and obj.get("verdict") == "not-met":
            ok = True
print("ok" if ok else "missing")
PY
)"
e2e_assert_eq "observe_metric_fields_ok" "${OBS_CHECK}" "ok"

# ---------------------------------------------------------------------------
# Step 11: Linter negative controls (edited summary, seq gap, small/scratch/no-control)
# ---------------------------------------------------------------------------
e2e_step "lint_negative_controls" "Verify linter catches edited summary, seq gap, small-claims-gate, scratch-registries, and no-control"

MUT_DIR="${E2E_WORK_DIR}/mutated_logs"
mkdir -p "${MUT_DIR}"

# 11a. Hand-edited summary count (recount mismatch)
EDITED_SUM_LOG="${MUT_DIR}/edited_summary.ndjson"
python3 - "${PASS_LOG_A}" "${EDITED_SUM_LOG}" <<'PY'
import hashlib, json, sys
lines = open(sys.argv[1], "r", encoding="utf-8").read().splitlines()
body = "\n".join(lines[:-1]) + "\n"
sum_obj = json.loads(lines[-1])
sum_obj["asserts_passed"] = sum_obj["asserts_passed"] + 1
sum_obj["body_sha256"] = hashlib.sha256(body.encode("utf-8")).hexdigest()
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    fh.write(body + json.dumps(sum_obj, separators=(",", ":")) + "\n")
PY
e2e_expect_caught "lint_edited_summary_count" -- "${FROB_E2E_TOOL}" lint "${EDITED_SUM_LOG}"

# 11b. Removed middle line (sequence gap)
SEQ_GAP_LOG="${MUT_DIR}/seq_gap.ndjson"
python3 - "${PASS_LOG_A}" "${SEQ_GAP_LOG}" <<'PY'
import sys
lines = open(sys.argv[1], "r", encoding="utf-8").read().splitlines()
kept = [lines[0]] + lines[2:]
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    fh.write("\n".join(kept) + "\n")
PY
e2e_expect_caught "lint_seq_gap" -- "${FROB_E2E_TOOL}" lint "${SEQ_GAP_LOG}"

# Helper script that touches a gate with a passing metric and zero controls (no-control)
GATE_SCRIPT="${MUT_DIR}/st_gate_nocontrol.sh"
cat > "${GATE_SCRIPT}" <<EOF
#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture gates=GATE-SELFTEST-01 seed=1 needs=cpu
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_gate_nocontrol" "STD-E2E"
e2e_step "measure" "Touch GATE-SELFTEST-01 with no negative control"
e2e_gate "GATE-SELFTEST-01"
e2e_metric "err_m" "0.001" "m" "<" "0.01" --gate "GATE-SELFTEST-01"
EOF
chmod +x "${GATE_SCRIPT}"

# 11c. Gate with passing value and zero caught controls -> lint catches no-control & report lists no-control
NOCONTROL_LOGS="${MUT_DIR}/nocontrol_logs"
mkdir -p "${NOCONTROL_LOGS}"
e2e_cmd --name run_gate_nocontrol -- \
  env "E2E_LOGS_ROOT=${NOCONTROL_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${GATE_SCRIPT}"
NC_LOG="$(latest_log_for "${NOCONTROL_LOGS}" "st_gate_nocontrol")"
e2e_expect_caught "lint_no_control" -- "${FROB_E2E_TOOL}" lint "${NC_LOG}"

NC_REPORT_DIR="${MUT_DIR}/nocontrol_report"
mkdir -p "${NC_REPORT_DIR}"
e2e_cmd --name report_no_control -- \
  "${FROB_E2E_TOOL}" report \
    --scripts-dir "${MUT_DIR}" \
    --logs-dir "${NOCONTROL_LOGS}" \
    --out-dir "${NC_REPORT_DIR}"
NC_GATE_STATUS="$(python3 - "${NC_REPORT_DIR}/gates.json" <<'PY'
import json, sys
data = json.load(open(sys.argv[1], "r", encoding="utf-8"))
rows = data.get("gates", data) if isinstance(data, dict) else data
res = ""
for r in rows:
    if r.get("gate") == "GATE-SELFTEST-01" or r.get("id") == "GATE-SELFTEST-01":
        res = r.get("result", "")
        break
print(res)
PY
)"
e2e_assert_eq "report_gate_no_control" "${NC_GATE_STATUS}" "no-control"

# 11d. Gate claimed in small mode (forced gates_claimable=true in mutated log) -> small-claims-gate
SMALL_LOGS="${MUT_DIR}/small_logs"
mkdir -p "${SMALL_LOGS}"
e2e_cmd --name run_small_mode -- \
  env "E2E_SMALL=1" "E2E_LOGS_ROOT=${SMALL_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${GATE_SCRIPT}"
SMALL_LOG="$(latest_log_for "${SMALL_LOGS}" "st_gate_nocontrol")"
SMALL_CLAIM_LOG="${MUT_DIR}/small_claims_gate.ndjson"
python3 - "${SMALL_LOG}" "${SMALL_CLAIM_LOG}" <<'PY'
import hashlib, json, sys
lines = open(sys.argv[1], "r", encoding="utf-8").read().splitlines()
body = "\n".join(lines[:-1]) + "\n"
s = json.loads(lines[-1])
s["gates_claimable"] = True
s["body_sha256"] = hashlib.sha256(body.encode("utf-8")).hexdigest()
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    fh.write(body + json.dumps(s, separators=(",", ":")) + "\n")
PY
e2e_expect_caught "lint_small_claims_gate" -- "${FROB_E2E_TOOL}" lint "${SMALL_CLAIM_LOG}"

# 11e. Gate claimed under e2e_registries_scratch -> registries=override & gates_claimable=false; mutating to claim fails as scratch-registries
SCRATCH_REG_SCRIPT="${MUT_DIR}/st_scratch_reg.sh"
cat > "${SCRATCH_REG_SCRIPT}" <<EOF
#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture gates=GATE-SELFTEST-02 seed=1 needs=cpu
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_scratch_reg" "STD-E2E"
e2e_step "override" "Switch to scratch registries"
e2e_registries_scratch
e2e_gate "GATE-SELFTEST-02"
e2e_assert_eq "reg_override_active" "\${E2E_REGISTRIES_MODE}" "override"
e2e_expect_caught "dummy_ctrl" --gate "GATE-SELFTEST-02" -- false
EOF
chmod +x "${SCRATCH_REG_SCRIPT}"
SCRATCH_REG_LOGS="${MUT_DIR}/scratch_reg_logs"
mkdir -p "${SCRATCH_REG_LOGS}"
e2e_cmd --name run_scratch_reg -- \
  env "E2E_LOGS_ROOT=${SCRATCH_REG_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${SCRATCH_REG_SCRIPT}"
SR_LOG="$(latest_log_for "${SCRATCH_REG_LOGS}" "st_scratch_reg")"
SR_SUM="${MUT_DIR}/sr_sum.json"
last_line_of "${SR_LOG}" > "${SR_SUM}"
e2e_assert_eq "scratch_reg_summary_mode" "$("${FROB_E2E_TOOL}" get --file "${SR_SUM}" /registries)" "override"
e2e_assert_eq "scratch_reg_gates_claimable_false" "$("${FROB_E2E_TOOL}" get --file "${SR_SUM}" /gates_claimable)" "false"

SR_MUT_LOG="${MUT_DIR}/scratch_claims_gate.ndjson"
python3 - "${SR_LOG}" "${SR_MUT_LOG}" <<'PY'
import hashlib, json, sys
lines = open(sys.argv[1], "r", encoding="utf-8").read().splitlines()
body = "\n".join(lines[:-1]) + "\n"
s = json.loads(lines[-1])
s["gates_claimable"] = True
s["body_sha256"] = hashlib.sha256(body.encode("utf-8")).hexdigest()
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    fh.write(body + json.dumps(s, separators=(",", ":")) + "\n")
PY
e2e_expect_caught "lint_scratch_registries" -- "${FROB_E2E_TOOL}" lint "${SR_MUT_LOG}"

# ---------------------------------------------------------------------------
# Step 12: e2e_tier validation & dry run; e2e_check single-envelope
# ---------------------------------------------------------------------------
e2e_step "tier_and_single_envelope" "Verify e2e_tier target validation/dry-run and single-envelope check"

TIER_BAD_SCRIPT="${E2E_WORK_DIR}/st_tier_bad.sh"
cat > "${TIER_BAD_SCRIPT}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_tier_bad" "STD-E2E"
e2e_step "tier" "Invalid tier target must exit 2"
e2e_tier "\$1" "x"
EOF
chmod +x "${TIER_BAD_SCRIPT}"

for bad_t in g1 g7 e2e_scripts; do
  e2e_cmd --name "tier_refuse_${bad_t}" --expect-exit 2 -- \
    env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
    "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${TIER_BAD_SCRIPT}" "${bad_t}"
  e2e_assert_eq "tier_${bad_t}_exit_2" "${E2E_LAST_EXIT}" "2"
done

TIER_DRY_SCRIPT="${E2E_WORK_DIR}/st_tier_dry.sh"
cat > "${TIER_DRY_SCRIPT}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_tier_dry" "STD-E2E"
e2e_step "tier_dry" "Dry run valid tier"
e2e_tier g1_known_answers x
EOF
chmod +x "${TIER_DRY_SCRIPT}"
e2e_cmd --name tier_dry_run --expect-exit 77 -- \
  env "E2E_DRY_RUN=1" "FROB_E2E_SEED=1" "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${TIER_DRY_SCRIPT}"
e2e_assert_eq "tier_dry_run_exit_77" "${E2E_LAST_EXIT}" "77"
if grep -Fq "FROB_TEST_SEED=1 cargo test -p frob-conformance --test g1_known_answers -- x" "${E2E_LAST_STDOUT}"; then
  e2e_assert_eq "tier_dry_run_cmdline" "matched" "matched"
else
  e2e_assert_eq "tier_dry_run_cmdline" "$(cat "${E2E_LAST_STDOUT}")" "FROB_TEST_SEED=1 cargo test -p frob-conformance --test g1_known_answers -- x"
fi

# e2e_check single-envelope
ONE_OBJ="${E2E_WORK_DIR}/one_envelope.json"
TWO_OBJ="${E2E_WORK_DIR}/two_envelopes.json"
printf '{"schema":"frob.envelope/1","ok":true}\n' > "${ONE_OBJ}"
printf '{"schema":"frob.envelope/1","ok":true}\n{"extra":2}\n' > "${TWO_OBJ}"
e2e_check single-envelope -- "${ONE_OBJ}"
e2e_expect_caught "single_envelope_rejects_two_objects" -- "${FROB_E2E_TOOL}" check single-envelope "${TWO_OBJ}"

# ---------------------------------------------------------------------------
# Step 13: e2e_session (grant, open, derive, use + Review Note 7 + redaction)
# ---------------------------------------------------------------------------
e2e_step "session_helper_and_note7" "Verify e2e_session grant, open (optional principal, ttl-s, capture), derive, use, and token redaction"

FAKE_FROB="${E2E_WORK_DIR}/fake_frob"
cat > "${FAKE_FROB}" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [ "${1:-}" = "capabilities" ]; then
  printf 'session.open\nsim.run\n'
  exit 0
fi
if [ "${1:-}" = "--json" ] && [ "${2:-}" = "session" ] && [ "${3:-}" = "open" ]; then
  shift 3
  is_derive=0
  refuse_me=0
  for a in "$@"; do
    case "$a" in
      --derive) is_derive=1 ;;
      refuse-scope) refuse_me=1 ;;
    esac
  done
  if [ "$refuse_me" -eq 1 ]; then
    printf '{"schema":"frob.envelope/1","verb":"session.open","ok":false,"exit":3,"refusal":{"code":"FROB-REF-724","exit":3,"message":"scope exceeds parent","pointer":"/scopes","observed":"admin","required":"subset","unit":"","retry":false,"repairs":[]}}\n'
    exit 3
  fi
  if [ "$is_derive" -eq 1 ]; then
    if [ -z "${FROB_SESSION_TOKEN:-}" ]; then
      printf '{"schema":"frob.envelope/1","verb":"session.open","ok":false,"exit":3,"refusal":{"code":"FROB-REF-723","exit":3,"message":"missing parent token","pointer":"/token","observed":null,"required":"parent token","unit":"","retry":false,"repairs":[]}}\n'
      exit 3
    fi
    printf '{"schema":"frob.envelope/1","verb":"session.open","ok":true,"exit":0,"result":{"token":"tok_selftest_child_55667788","parent_seen":true}}\n'
    exit 0
  fi
  if [ -n "${FROB_SESSION_TOKEN:-}" ]; then
    printf '{"schema":"frob.envelope/1","verb":"session.open","ok":false,"exit":3,"refusal":{"code":"FROB-REF-723","exit":3,"message":"root open must not send token","pointer":"/token","observed":"present","required":"absent","unit":"","retry":false,"repairs":[]}}\n'
    exit 3
  fi
  printf '{"schema":"frob.envelope/1","verb":"session.open","ok":true,"exit":0,"result":{"token":"tok_selftest_root_11223344"}}\n'
  exit 0
fi
printf '{"schema":"frob.envelope/1","verb":"unknown","ok":false,"exit":3,"refusal":{"code":"FROB-REF-801","exit":3,"message":"unknown verb","pointer":"/verb","observed":"","required":"known","unit":"","retry":false,"repairs":[]}}\n'
exit 3
EOF
chmod +x "${FAKE_FROB}"

SESS_SCRIPT="${E2E_WORK_DIR}/st_session_flow.sh"
cat > "${SESS_SCRIPT}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
export FROB_BIN="${FAKE_FROB}"
e2e_init "st_session_flow" "STD-E2E"
e2e_step "grant_open_derive_use" "Exercise grant, open (optional principal/ttl-s/capture), derive, and use"
e2e_session grant agent-a --scopes read,cpu:core_seconds=600 --namespace ns1 --max-lifetime-s 60
e2e_assert_file "safety_toml_exists" "\${FROB_STATE}/safety.toml"
# Note 7: open without --principal, with --ttl-s and --capture (never pass raw token to e2e_assert_eq!)
e2e_session open root-a --ttl-s 10 --capture strict
if [ "\${FROB_SESSION_TOKEN}" = "tok_selftest_root_11223344" ]; then
  e2e_assert_eq "root_token_exported" "matched" "matched"
else
  e2e_assert_eq "root_token_exported" "mismatch" "matched"
fi
# Note 7: derive keeps current FROB_SESSION_TOKEN and exports child token
e2e_session derive child-a --scopes read --ttl-s 5
if [ "\${FROB_SESSION_TOKEN}" = "tok_selftest_child_55667788" ]; then
  e2e_assert_eq "child_token_exported" "matched" "matched"
else
  e2e_assert_eq "child_token_exported" "mismatch" "matched"
fi
# Switch back with use
e2e_session use root-a
if [ "\${FROB_SESSION_TOKEN}" = "tok_selftest_root_11223344" ]; then
  e2e_assert_eq "switched_back_to_root" "matched" "matched"
else
  e2e_assert_eq "switched_back_to_root" "mismatch" "matched"
fi
EOF
chmod +x "${SESS_SCRIPT}"

SESS_LOGS="${INNER_LOGS}/session_flow"
SESS_SCRATCH="${INNER_SCRATCH}/session_flow"
mkdir -p "${SESS_LOGS}" "${SESS_SCRATCH}"
e2e_cmd --name run_session_flow -- \
  env "E2E_LOGS_ROOT=${SESS_LOGS}" "E2E_SCRATCH_ROOT=${SESS_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${SESS_SCRIPT}"
SESS_LOG="$(latest_log_for "${SESS_LOGS}" "st_session_flow")"
SESS_TX="$(latest_transcript_for "${SESS_LOGS}" "st_session_flow")"
e2e_cmd --name lint_session_flow -- "${FROB_E2E_TOOL}" lint "${SESS_LOG}"

# Verify safety.toml & token files mode 0600, and token redacted to <token:12hex> in log, transcript, and captures!
SESS_AUDIT="$(python3 - "${SESS_LOG}" "${SESS_TX}" "${SESS_SCRATCH}/st_session_flow" <<'PY'
import glob, json, os, re, stat, sys
log_p, tx_p, scr_root = sys.argv[1:4]
first = json.loads(open(log_p, "r", encoding="utf-8").readline())
rid = first["run_id"]
state_dir = os.path.join(scr_root, rid, "state")
safety = os.path.join(state_dir, "safety.toml")
tok_root = os.path.join(state_dir, "sessions", "root-a.token")
tok_child = os.path.join(state_dir, "sessions", "child-a.token")
errs = []
for label, p in (("safety", safety), ("tok_root", tok_root), ("tok_child", tok_child)):
    if not os.path.isfile(p):
        errs.append(f"missing:{label}")
    elif stat.S_IMODE(os.stat(p).st_mode) != 0o600:
        errs.append(f"mode:{label}:{oct(stat.S_IMODE(os.stat(p).st_mode))}")
if os.path.isfile(safety):
    stxt = open(safety, "r", encoding="utf-8").read()
    if "[principal.agent-a]" not in stxt or '"cpu:core_seconds=600"' not in stxt:
        errs.append("safety_content")
if os.path.isfile(tok_root) and open(tok_root).read().strip() != "tok_selftest_root_11223344":
    errs.append("root_tok_val")
if os.path.isfile(tok_child) and open(tok_child).read().strip() != "tok_selftest_child_55667788":
    errs.append("child_tok_val")
art_dir = log_p[:-len(".ndjson")] + ".artifacts"
check_files = [log_p, tx_p] + sorted(glob.glob(os.path.join(art_dir, "*")))
saw_placeholder = False
for cf in check_files:
    if not os.path.isfile(cf):
        continue
    txt = open(cf, "r", encoding="utf-8", errors="replace").read()
    if "tok_selftest_root_11223344" in txt or "tok_selftest_child_55667788" in txt:
        errs.append(f"raw_token_leak:{os.path.basename(cf)}")
    if re.search(r"<token:[0-9a-f]{12}>", txt):
        saw_placeholder = True
if not saw_placeholder:
    errs.append("no_token_placeholder")
print("ok" if not errs else ",".join(errs))
PY
)"
e2e_assert_eq "session_permissions_and_token_redaction" "${SESS_AUDIT}" "ok"

# Review Note 7 item 3: refused derivation leaves FROB_SESSION_TOKEN unchanged!
SESS_REFUSE_DERIVE="${E2E_WORK_DIR}/st_sess_refuse_derive.sh"
cat > "${SESS_REFUSE_DERIVE}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
export FROB_BIN="${FAKE_FROB}"
e2e_init "st_sess_refuse_derive" "STD-E2E"
e2e_step "refuse_derive" "Refused derive keeps parent token unchanged"
e2e_session open root-b --ttl-s 10
before_tok="\${FROB_SESSION_TOKEN}"
e2e_session derive child-bad --scopes refuse-scope
if [ "\${FROB_SESSION_TOKEN}" = "\${before_tok}" ]; then
  e2e_assert_eq "token_unchanged_after_refusal" "unchanged" "unchanged"
else
  e2e_assert_eq "token_unchanged_after_refusal" "changed" "unchanged"
fi
EOF
chmod +x "${SESS_REFUSE_DERIVE}"
e2e_cmd --name run_session_refuse_derive --expect-exit 1 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${SESS_REFUSE_DERIVE}"
SD_LOG="$(latest_log_for "${INNER_LOGS}" "st_sess_refuse_derive")"
SD_KEEP="$(python3 - "${SD_LOG}" <<'PY'
import json, sys
ok = False
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "assert" and obj.get("name") == "token_unchanged_after_refusal" and obj.get("status") == "pass":
        ok = True
print("ok" if ok else "failed")
PY
)"
e2e_assert_eq "refused_derive_leaves_token_unchanged" "${SD_KEEP}" "ok"

# Duplicate grant exits 2
SESS_DUP="${E2E_WORK_DIR}/st_sess_dup.sh"
cat > "${SESS_DUP}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_sess_dup" "STD-E2E"
e2e_step "dup" "Duplicate grant must exit 2"
e2e_session grant agent-a --scopes read
e2e_session grant agent-a --scopes read
EOF
chmod +x "${SESS_DUP}"
e2e_cmd --name session_dup_grant --expect-exit 2 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${SESS_DUP}"
e2e_assert_eq "session_dup_grant_exit_2" "${E2E_LAST_EXIT}" "2"

# Grant while daemon socket is live exits 2 (bind relative path to stay under macOS 104-byte sun_path limit)
SESS_SOCK="${E2E_WORK_DIR}/st_sess_sock.sh"
cat > "${SESS_SOCK}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_sess_sock" "STD-E2E"
e2e_step "sock" "Grant while frobd.sock listens must exit 2"
(cd "\${FROB_STATE}" && python3 -c 'import socket; s=socket.socket(socket.AF_UNIX); s.bind("frobd.sock"); s.listen(1)')
e2e_session grant agent-b --scopes read
EOF
chmod +x "${SESS_SOCK}"
e2e_cmd --name session_live_sock_grant --expect-exit 2 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${SESS_SOCK}"
e2e_assert_eq "session_live_sock_exit_2" "${E2E_LAST_EXIT}" "2"

# Unknown label on e2e_session use exits 2
SESS_UNK="${E2E_WORK_DIR}/st_sess_unk.sh"
cat > "${SESS_UNK}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
source "${SCRIPT_DIR}/lib.sh"
e2e_init "st_sess_unk" "STD-E2E"
e2e_step "unk" "Unknown label on e2e_session use must exit 2"
e2e_session use no-such-session
EOF
chmod +x "${SESS_UNK}"
e2e_cmd --name session_use_unknown --expect-exit 2 -- \
  env "E2E_LOGS_ROOT=${INNER_LOGS}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  "FROB_E2E_TOOL=${FROB_E2E_TOOL}" bash "${SESS_UNK}"
e2e_assert_eq "session_use_unknown_exit_2" "${E2E_LAST_EXIT}" "2"

# ---------------------------------------------------------------------------
# Step 14: run-all.sh --label harness and frob-e2e report
# ---------------------------------------------------------------------------
e2e_step "runner_and_report" "Run run-all.sh --label harness and frob-e2e report"

RUNNER_LOGS="${INNER_LOGS}/runner_harness"
REPORT_OUT="${E2E_WORK_DIR}/report_out"
mkdir -p "${RUNNER_LOGS}" "${REPORT_OUT}"

e2e_cmd --name run_all_harness_label -- \
  env "FROB_E2E_TOOL=${FROB_E2E_TOOL}" "E2E_SCRATCH_ROOT=${INNER_SCRATCH}" \
  bash "${SCRIPT_DIR}/run-all.sh" \
    --scripts-dir "${SELFTEST_DIR}" \
    --logs-dir "${RUNNER_LOGS}" \
    --label harness
e2e_assert_eq "run_all_harness_exit_0" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name frob_e2e_report -- \
  "${FROB_E2E_TOOL}" report \
    --scripts-dir "${SELFTEST_DIR}" \
    --logs-dir "${RUNNER_LOGS}" \
    --out-dir "${REPORT_OUT}"
e2e_assert_file "report_gates_json_exists" "${REPORT_OUT}/gates.json"
e2e_assert_file "report_gates_md_exists" "${REPORT_OUT}/gates.md"

# ---------------------------------------------------------------------------
# Step 15: Live frob binary stage (runs when frob exists, otherwise logged skip)
# ---------------------------------------------------------------------------
e2e_step "live_frob_stage" "Run frob doctor --json and unknown verb (FROB-REF-801) when frob exists, else log skip"

if [ -x "${FROB_BIN}" ]; then
  e2e_cmd --name frob_doctor -- "${FROB_BIN}" doctor --json
  e2e_assert_json "frob_doctor_ok" "/ok" "true"
  e2e_expect_refusal FROB-REF-801 3 -- "${FROB_BIN}" --json __selftest_unknown_verb__
else
  e2e_skip "frob binary absent (${FROB_BIN}); live stage skipped until frob exists"
fi
