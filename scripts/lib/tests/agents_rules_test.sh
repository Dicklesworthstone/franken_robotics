#!/bin/sh
# Unit tests and text check for AGENTS.md ## Working rules (bead N34p4-04).
#
# Runs directly (`sh scripts/lib/tests/agents_rules_test.sh`) and via
# `scripts/precommit.sh` Step 4 using real files and scratch copies (no mocks).
# Emits STD-E2E structured NDJSON (one `assert` line per rule row 1..12 with
# rule number and enforcement point, plus negative-control asserts and a
# `summary` line).
# POSIX sh compatible; never deletes any file or directory (COMMON.md 9b).

set -eu
export LC_ALL=C

SCRIPT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH='' cd -- "${SCRIPT_DIR}/../../.." && pwd)"
ENVELOPE_SH="${REPO_ROOT}/scripts/lib/envelope.sh"
AGENTS_MD="${REPO_ROOT}/AGENTS.md"

FROB_REPO_ROOT="${REPO_ROOT}"
export FROB_REPO_ROOT
# shellcheck source=scripts/lib/envelope.sh
. "${ENVELOPE_SH}"

SCRATCH_BASE="$(frob_scratch_dir "frob-agents-rules-test")"
mkdir -p "${SCRATCH_BASE}"
TMP_ROOT="$(mktemp -d "${SCRATCH_BASE}/run.XXXXXX")"

LOG_FILE="${AGENTS_RULES_LOG_FILE:-${TMP_ROOT}/agents_rules.ndjson}"
: > "${LOG_FILE}"
RUN_ID="${E2E_RUN_ID:-agents-rules-$$-$(date -u +%Y%m%dT%H%M%SZ)}"
LOG_SEQ=0
LOG_PASSED=0
LOG_FAILED=0

PASS=0
FAIL=0

emit_json_line() {
  printf '%s\n' "$1" >> "${LOG_FILE}"
  LOG_SEQ=$((LOG_SEQ + 1))
}

emit_start() {
  _es_ts="$(_frob_now_utc_ms)"
  emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_es_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"\",\"event\":\"start\",\"seed\":\"0\"}"
}

emit_step_start() {
  _ess_step="$1"
  _ess_title="$(_frob_json_escape "$2")"
  _ess_ts="$(_frob_now_utc_ms)"
  emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_ess_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"${_ess_step}\",\"event\":\"start\",\"seed\":\"0\",\"title\":\"${_ess_title}\"}"
}

emit_step_end() {
  _ese_step="$1"
  _ese_ts="$(_frob_now_utc_ms)"
  emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_ese_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"${_ese_step}\",\"event\":\"end\",\"seed\":\"0\"}"
}

emit_rule_assert() {
  _era_step="$1"
  _era_num="$2"
  _era_enf="$3"
  _era_status="$4"
  _era_obs="$5"
  _era_exp="$6"
  _era_ts="$(_frob_now_utc_ms)"
  _era_name="working_rule_${_era_num}"
  _era_ej="$(_frob_json_escape "$_era_enf")"
  _era_oj="$(_frob_json_escape "$_era_obs")"
  _era_xj="$(_frob_json_escape "$_era_exp")"

  if [ "$_era_status" = "pass" ]; then
    LOG_PASSED=$((LOG_PASSED + 1))
    emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_era_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"${_era_step}\",\"event\":\"assert\",\"seed\":\"0\",\"name\":\"${_era_name}\",\"kind\":\"eq\",\"status\":\"pass\",\"rule_number\":${_era_num},\"enforcement\":\"${_era_ej}\",\"observed\":\"${_era_oj}\",\"expected\":\"${_era_xj}\"}"
  else
    LOG_FAILED=$((LOG_FAILED + 1))
    emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_era_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"${_era_step}\",\"event\":\"assert\",\"seed\":\"0\",\"name\":\"${_era_name}\",\"kind\":\"eq\",\"status\":\"fail\",\"rule_number\":${_era_num},\"enforcement\":\"${_era_ej}\",\"observed\":\"${_era_oj}\",\"expected\":\"${_era_xj}\",\"first_diff\":{\"path\":\"${_era_name}\",\"observed\":\"${_era_oj}\",\"expected\":\"${_era_xj}\"},\"repro\":{\"cmd_seq\":0,\"artifacts\":[]}}"
  fi

  if _frob_use_e2e_tool; then
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event assert \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${E2E_CURRENT_STEP:-agents_rules}" \
      --seed "${E2E_SEED}" \
      --name "${_era_name}" \
      --kind eq \
      --status "${_era_status}" \
      --observed "${_era_obs}" \
      --expected "${_era_exp}" >/dev/null 2>&1 || true
  fi
}

emit_check_assert() {
  _eca_step="$1"
  _eca_name="$2"
  _eca_status="$3"
  _eca_obs="$4"
  _eca_exp="$5"
  _eca_ts="$(_frob_now_utc_ms)"
  _eca_nj="$(_frob_json_escape "$_eca_name")"
  _eca_oj="$(_frob_json_escape "$_eca_obs")"
  _eca_xj="$(_frob_json_escape "$_eca_exp")"

  if [ "$_eca_status" = "pass" ]; then
    LOG_PASSED=$((LOG_PASSED + 1))
    emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_eca_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"${_eca_step}\",\"event\":\"assert\",\"seed\":\"0\",\"name\":\"${_eca_nj}\",\"kind\":\"eq\",\"status\":\"pass\",\"observed\":\"${_eca_oj}\",\"expected\":\"${_eca_xj}\"}"
  else
    LOG_FAILED=$((LOG_FAILED + 1))
    emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_eca_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"${_eca_step}\",\"event\":\"assert\",\"seed\":\"0\",\"name\":\"${_eca_nj}\",\"kind\":\"eq\",\"status\":\"fail\",\"observed\":\"${_eca_oj}\",\"expected\":\"${_eca_xj}\",\"first_diff\":{\"path\":\"${_eca_nj}\",\"observed\":\"${_eca_oj}\",\"expected\":\"${_eca_xj}\"},\"repro\":{\"cmd_seq\":0,\"artifacts\":[]}}"
  fi
}

emit_summary() {
  _esu_rc="$1"
  _esu_ts="$(_frob_now_utc_ms)"
  emit_json_line "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_esu_ts}\",\"seq\":${LOG_SEQ},\"run_id\":\"${RUN_ID}\",\"script\":\"agents_rules_test\",\"bead\":\"N34p4-04\",\"step\":\"\",\"event\":\"summary\",\"seed\":\"0\",\"exit\":${_esu_rc},\"passed\":${LOG_PASSED},\"failed\":${LOG_FAILED}}"
}

pass() {
  PASS=$((PASS + 1))
  printf 'PASS: %s\n' "$1"
  emit_check_assert "unit_tests" "$1" "pass" "pass" "pass"
}

fail() {
  FAIL=$((FAIL + 1))
  printf 'FAIL: %s — %s\n' "$1" "$2" >&2
  emit_check_assert "unit_tests" "$1" "fail" "$2" "pass"
}

# Pure validator: parses `## Working rules` from `$1` (AGENTS.md path), checks
# enforcement function definitions against `$2/scripts/lib/envelope.sh` and
# script paths against `$2/`.
# Prints machine-readable findings (`FINDING: ...`) on failure, and writes
# extracted rows (`<num>\t<rule>\t<why>\t<enforced_by>`) to `$3` when given.
validate_agents_rules() {
  _var_agents="$1"
  _var_root="$2"
  _var_rows_out="${3:-}"
  _var_env_sh="${_var_root}/scripts/lib/envelope.sh"

  if [ ! -f "$_var_agents" ]; then
    printf 'FINDING: missing AGENTS.md at %s\n' "$_var_agents"
    return 1
  fi
  if [ ! -f "$_var_env_sh" ]; then
    printf 'FINDING: missing envelope.sh at %s\n' "$_var_env_sh"
    return 1
  fi

  _var_work="$(mktemp -d "${TMP_ROOT}/check.XXXXXX")"
  _var_sec="${_var_work}/section.md"
  _var_rows="${_var_work}/rows.tsv"
  _var_defs="${_var_work}/envelope_defs.txt"
  _var_findings="${_var_work}/findings.txt"
  : > "$_var_findings"

  # 1. Extract exact `## Working rules` section.
  awk '
    /^## Working rules[[:space:]]*$/ { in_sec = 1; next }
    in_sec && /^## / { exit }
    in_sec { print }
  ' "$_var_agents" > "$_var_sec"

  if [ ! -s "$_var_sec" ]; then
    printf 'FINDING: missing exact level-2 section "## Working rules"\n' >> "$_var_findings"
    cat "$_var_findings"
    return 1
  fi

  # 2. Extract table rows and check header, column count, non-empty reason/enforcement,
  #    contiguous numbering 1..12, and no duplicates.
  awk -v rows_file="$_var_rows" -v findings_file="$_var_findings" '
    BEGIN {
      in_table = 0
      table_done = 0
      saw_header = 0
      saw_sep = 0
      row_count = 0
    }
    !table_done && /^[[:space:]]*\|/ {
      line = $0
      sub(/^[[:space:]]*\|/, "", line)
      sub(/\|[[:space:]]*$/, "", line)
      n = split(line, cols, /\|/)
      for (k = 1; k <= n; k++) {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", cols[k])
      }
      if (!saw_header) {
        c1 = tolower(cols[1])
        c2 = tolower(cols[2])
        c3 = tolower(cols[3])
        c4 = tolower(cols[4])
        if (n < 4 || c1 != "#" || c2 != "rule" || c3 != "why" || c4 != "enforced by") {
          print "FINDING: table header must have columns #, Rule, Why, Enforced by" >> findings_file
        }
        saw_header = 1
        in_table = 1
        next
      }
      if (!saw_sep) {
        if (line ~ /^[-:|[:space:]]+$/) {
          saw_sep = 1
          next
        }
        print "FINDING: table missing markdown separator row" >> findings_file
        saw_sep = 1
      }
      if (n != 4) {
        print "FINDING: table row " (row_count + 1) " has " n " columns (expected 4)" >> findings_file
        next
      }
      num = cols[1]
      rule = cols[2]
      why = cols[3]
      enf = cols[4]
      row_count++
      if (num !~ /^[0-9]+$/) {
        print "FINDING: row " row_count " has non-numeric rule number: " num >> findings_file
      } else {
        nval = num + 0
        if (seen[nval]++) {
          print "FINDING: duplicate rule number " nval >> findings_file
        }
        if (nval != row_count) {
          print "FINDING: rule number out of order at position " row_count " (got " nval ")" >> findings_file
        }
      }
      if (rule == "") {
        print "FINDING: rule " num " has empty Rule cell" >> findings_file
      }
      if (why == "") {
        print "FINDING: rule " num " has empty Why cell" >> findings_file
      }
      if (enf == "") {
        print "FINDING: rule " num " has empty Enforced by cell" >> findings_file
      }
      printf "%s\t%s\t%s\t%s\n", num, rule, why, enf >> rows_file
      next
    }
    in_table && !/^[[:space:]]*\|/ {
      in_table = 0
      table_done = 1
    }
    END {
      if (!saw_header) {
        print "FINDING: ## Working rules contains no markdown table" >> findings_file
      } else {
        if (row_count != 12) {
          print "FINDING: expected 12 rule rows, found " row_count >> findings_file
        }
        for (i = 1; i <= 12; i++) {
          if (!seen[i]) {
            print "FINDING: missing rule number " i >> findings_file
          }
        }
      }
    }
  ' "$_var_sec"

  if [ -n "$_var_rows_out" ] && [ -f "$_var_rows" ]; then
    cp "$_var_rows" "$_var_rows_out"
  fi

  # 3. Collect all function names defined in scripts/lib/envelope.sh.
  awk '
    /^[[:space:]]*[a-zA-Z_][a-zA-Z0-9_]*[[:space:]]*\(\)/ {
      name = $0
      sub(/^[[:space:]]*/, "", name)
      sub(/[[:space:]]*\(\).*/, "", name)
      print name
    }
  ' "$_var_env_sh" > "$_var_defs"

  # 4. Verify every frob_* shell function name and every scripts/*.sh path in
  #    each row's `Enforced by` cell resolves against envelope.sh / repo root.
  if [ -f "$_var_rows" ]; then
    while IFS='	' read -r _r_num _r_rule _r_why _r_enf; do
      [ -n "$_r_num" ] || continue

      # Check all frob_* shell function identifiers mentioned in Enforced by.
      _r_fns="$(printf '%s\n' "$_r_enf" | awk '
        {
          s = $0
          while (match(s, /frob_[A-Za-z0-9_]+/)) {
            print substr(s, RSTART, RLENGTH)
            s = substr(s, RSTART + RLENGTH)
          }
        }
      ' | LC_ALL=C sort -u)"
      for _fn in $_r_fns; do
        if ! grep -Fxq "$_fn" "$_var_defs"; then
          printf 'FINDING: rule %s Enforced by names undefined envelope.sh function: %s\n' "$_r_num" "$_fn" >> "$_var_findings"
        fi
      done

      # Check all scripts/.../*.sh paths mentioned in Enforced by.
      _r_scripts="$(printf '%s\n' "$_r_enf" | awk '
        {
          s = $0
          while (match(s, /scripts\/[A-Za-z0-9_.\/-]+\.sh/)) {
            print substr(s, RSTART, RLENGTH)
            s = substr(s, RSTART + RLENGTH)
          }
        }
      ' | LC_ALL=C sort -u)"
      for _sp in $_r_scripts; do
        case "$_sp" in
          *\**) continue ;;
        esac
        if [ ! -f "${_var_root}/${_sp}" ]; then
          printf 'FINDING: rule %s Enforced by names missing script path: %s\n' "$_r_num" "$_sp" >> "$_var_findings"
        fi
      done
    done < "$_var_rows"
  fi

  # 5. Check Rule 4 envelope details and Rule 10 build-failure recipe.
  for _req in "16 GiB" "8 cores" "one hour" "one device" "frob_envelope_run" "frob-launch" "registries/compute_envelope.toml"; do
    if ! grep -Fq "$_req" "$_var_sec"; then
      printf 'FINDING: ## Working rules missing required rule-4 detail: %s\n' "$_req" >> "$_var_findings"
    fi
  done

  if ! awk '
    BEGIN { in_recipe = 0; step1_ok = 0; step2_ok = 0; step3_ok = 0 }
    /^###[[:space:]]+Build-failure recipe/ { in_recipe = 1; next }
    in_recipe && /^##/ { in_recipe = 0 }
    in_recipe && /^1\.[[:space:]]+.*cargo xtask check-stack/ { step1_ok = 1 }
    in_recipe && /^2\.[[:space:]]+.*fast-forward.*divergence/ { step2_ok = 1 }
    in_recipe && /^3\.[[:space:]]+.*[Nn]ever move a sibling checkout back/ { step3_ok = 1 }
    END { exit !(step1_ok && step2_ok && step3_ok) }
  ' "$_var_sec"; then
    printf 'FINDING: ## Working rules missing rule-10 build-failure recipe starting with "1. ... cargo xtask check-stack"\n' >> "$_var_findings"
  fi

  if [ -s "$_var_findings" ]; then
    cat "$_var_findings"
    return 1
  fi
  return 0
}

# ---------------------------------------------------------------------------
# Run suite
# ---------------------------------------------------------------------------
emit_start
emit_step_start "clean_agents_md" "Validate live AGENTS.md ## Working rules rows and enforcement points"

CLEAN_ROWS="${TMP_ROOT}/clean_rows.tsv"
CLEAN_OUT="${TMP_ROOT}/clean_out.txt"
clean_rc=0
validate_agents_rules "${AGENTS_MD}" "${REPO_ROOT}" "${CLEAN_ROWS}" >"${CLEAN_OUT}" 2>&1 || clean_rc=$?

# Emit one STD-E2E assert line per rule row (with rule number and enforcement).
if [ -f "${CLEAN_ROWS}" ]; then
  while IFS='	' read -r _cr_num _cr_rule _cr_why _cr_enf; do
    case "$_cr_num" in
      ''|*[!0-9]*) continue ;;
    esac
    if [ "$clean_rc" -eq 0 ]; then
      emit_rule_assert "clean_agents_md" "$_cr_num" "$_cr_enf" "pass" "rule_${_cr_num}:valid" "rule_${_cr_num}:valid"
    else
      emit_rule_assert "clean_agents_md" "$_cr_num" "$_cr_enf" "fail" "rule_${_cr_num}:invalid" "rule_${_cr_num}:valid"
    fi
  done < "${CLEAN_ROWS}"
fi
emit_step_end "clean_agents_md"

emit_step_start "unit_tests" "Run positive and planted-negative checks on AGENTS.md working rules"

# 1. twelve_rows_present
if [ "$clean_rc" -eq 0 ] && [ "$(wc -l < "${CLEAN_ROWS}" | tr -d ' ')" -eq 12 ]; then
  pass "twelve_rows_present"
else
  fail "twelve_rows_present" "clean validation failed (rc=${clean_rc}): $(tr '\n' '; ' < "${CLEAN_OUT}")"
fi

# 2. numbers_contiguous (positive + duplicate-number negative)
CONTIG_OK=0
if [ -f "${CLEAN_ROWS}" ] && [ "$(cut -f1 "${CLEAN_ROWS}" | tr '\n' ',')" = "1,2,3,4,5,6,7,8,9,10,11,12," ]; then
  DUP_AGENTS="${TMP_ROOT}/agents_dup_num.md"
  sed 's/^| 5 |/| 4 |/' "${AGENTS_MD}" > "${DUP_AGENTS}"
  dup_rc=0
  dup_out="$(validate_agents_rules "${DUP_AGENTS}" "${REPO_ROOT}" 2>&1)" || dup_rc=$?
  if [ "$dup_rc" -ne 0 ] && printf '%s\n' "$dup_out" | grep -Fq "duplicate rule number 4"; then
    CONTIG_OK=1
  fi
fi
if [ "$CONTIG_OK" -eq 1 ]; then
  pass "numbers_contiguous"
else
  fail "numbers_contiguous" "contiguous 1..12 or duplicate-number rejection failed"
fi

# 3. enforcement_names_exist
if [ "$clean_rc" -eq 0 ] \
  && grep -Fq "frob_envelope_run" "${CLEAN_ROWS}" \
  && grep -Fq "frob_target_dir" "${CLEAN_ROWS}" \
  && grep -Fq "frob_scratch_dir" "${CLEAN_ROWS}" \
  && grep -Fq "frob_disk_preflight" "${CLEAN_ROWS}" \
  && grep -Fq "scripts/precommit.sh" "${CLEAN_ROWS}" \
  && grep -Fq "scripts/lib/envelope.sh" "${CLEAN_ROWS}"; then
  pass "enforcement_names_exist"
else
  fail "enforcement_names_exist" "expected enforcement functions/scripts missing or invalid"
fi

# 4. build_failure_recipe_present
if [ "$clean_rc" -eq 0 ] && grep -Fq "cargo xtask check-stack" "${AGENTS_MD}"; then
  pass "build_failure_recipe_present"
else
  fail "build_failure_recipe_present" "rule-10 build-failure recipe check failed"
fi

# 5. missing_row_fails (synthetic copy without row 4)
NO_ROW4="${TMP_ROOT}/agents_missing_row4.md"
awk '!/^\| 4 \|/' "${AGENTS_MD}" > "${NO_ROW4}"
no4_rc=0
no4_out="$(validate_agents_rules "${NO_ROW4}" "${REPO_ROOT}" 2>&1)" || no4_rc=$?
if [ "$no4_rc" -ne 0 ] && printf '%s\n' "$no4_out" | grep -Fq "missing rule number 4"; then
  pass "missing_row_fails"
else
  fail "missing_row_fails" "expected failure naming missing rule 4, got rc=${no4_rc}: ${no4_out}"
fi

# 6. misnamed_function_fails (synthetic copy naming frob_envelope_runx in row 4 Enforced by)
BAD_FN="${TMP_ROOT}/agents_misnamed_fn.md"
awk '/^\| 4 \|/ { sub(/frob_envelope_run/, "frob_envelope_runx") } { print }' "${AGENTS_MD}" > "${BAD_FN}"
badfn_rc=0
badfn_out="$(validate_agents_rules "${BAD_FN}" "${REPO_ROOT}" 2>&1)" || badfn_rc=$?
if [ "$badfn_rc" -ne 0 ] && printf '%s\n' "$badfn_out" | grep -Fq "undefined envelope.sh function: frob_envelope_runx"; then
  pass "misnamed_function_fails"
else
  fail "misnamed_function_fails" "expected failure naming frob_envelope_runx, got rc=${badfn_rc}: ${badfn_out}"
fi

# 7. missing_script_path_fails (synthetic copy naming nonexistent script path)
BAD_SCRIPT="${TMP_ROOT}/agents_missing_script.md"
sed 's|scripts/precommit.sh|scripts/nonexistent_precommit.sh|g' "${AGENTS_MD}" > "${BAD_SCRIPT}"
badsc_rc=0
badsc_out="$(validate_agents_rules "${BAD_SCRIPT}" "${REPO_ROOT}" 2>&1)" || badsc_rc=$?
if [ "$badsc_rc" -ne 0 ] && printf '%s\n' "$badsc_out" | grep -Fq "missing script path: scripts/nonexistent_precommit.sh"; then
  pass "missing_script_path_fails"
else
  fail "missing_script_path_fails" "expected failure naming missing script path, got rc=${badsc_rc}: ${badsc_out}"
fi

# 8. empty_why_and_enforcement_fail
EMPTY_WHY="${TMP_ROOT}/agents_empty_why.md"
awk -F'|' '
  /^\| 9 \|/ { printf "| %s | %s |  | %s |\n", $2, $3, $5; next }
  { print }
' "${AGENTS_MD}" > "${EMPTY_WHY}"
ew_rc=0
ew_out="$(validate_agents_rules "${EMPTY_WHY}" "${REPO_ROOT}" 2>&1)" || ew_rc=$?
EMPTY_ENF="${TMP_ROOT}/agents_empty_enf.md"
awk -F'|' '
  /^\| 9 \|/ { printf "| %s | %s | %s |  |\n", $2, $3, $4; next }
  { print }
' "${AGENTS_MD}" > "${EMPTY_ENF}"
ee_rc=0
ee_out="$(validate_agents_rules "${EMPTY_ENF}" "${REPO_ROOT}" 2>&1)" || ee_rc=$?
if [ "$ew_rc" -ne 0 ] && printf '%s\n' "$ew_out" | grep -Fq "empty Why cell" \
  && [ "$ee_rc" -ne 0 ] && printf '%s\n' "$ee_out" | grep -Fq "empty Enforced by cell"; then
  pass "empty_why_and_enforcement_fail"
else
  fail "empty_why_and_enforcement_fail" "ew_rc=${ew_rc} ee_rc=${ee_rc}"
fi

# 9. recipe_missing_check_stack_fails
BAD_RECIPE="${TMP_ROOT}/agents_bad_recipe.md"
sed 's/cargo xtask check-stack/cargo test/' "${AGENTS_MD}" > "${BAD_RECIPE}"
br_rc=0
br_out="$(validate_agents_rules "${BAD_RECIPE}" "${REPO_ROOT}" 2>&1)" || br_rc=$?
if [ "$br_rc" -ne 0 ] && printf '%s\n' "$br_out" | grep -Fq "build-failure recipe"; then
  pass "recipe_missing_check_stack_fails"
else
  fail "recipe_missing_check_stack_fails" "expected recipe failure, got rc=${br_rc}: ${br_out}"
fi

# 10. deterministic_extraction (pure function of AGENTS.md and envelope.sh)
CLEAN_ROWS_2="${TMP_ROOT}/clean_rows_2.tsv"
if validate_agents_rules "${AGENTS_MD}" "${REPO_ROOT}" "${CLEAN_ROWS_2}" >/dev/null 2>&1 \
  && cmp -s "${CLEAN_ROWS}" "${CLEAN_ROWS_2}"; then
  pass "deterministic_extraction"
else
  fail "deterministic_extraction" "two runs produced differing extracted tables"
fi

emit_step_end "unit_tests"

if [ "$FAIL" -eq 0 ]; then
  emit_summary 0
else
  emit_summary 1
fi

printf '\nagents_rules_test: passed=%d failed=%d log=%s\n' "$PASS" "$FAIL" "${LOG_FILE}"
[ "$FAIL" -eq 0 ]
