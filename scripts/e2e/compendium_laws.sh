#!/usr/bin/env bash
# e2e: bead=N12-41 milestone=M1 labels=lemma,frob-conformance seed=0 needs=cpu plants=coadjoint-minus,doc-drift
#
# Shared end-to-end verification script for the Mathematics Compendium (D.1..D.10).
# Written by bead N12-41 and extended automatically as D.2..D.10 land.
#
# Steps:
#   1. registry : registry_check + compendium_registry; log row counts vs plan
#                 (21, 12, 10, 14, 10, 15, 7, 19, 14, 4); unlanded sections are skip.
#   2. laws     : run each landed compendium_dN target with FROB_TEST_EVIDENCE_DIR
#                 and map every frob.testevidence/1 line to an E2E assert line
#                 (negative controls with kind="negative-control" and caught).
#   3. control  : verify the coadjoint-minus mutant is caught by d1_ad_star_pairing.
#   4. doc      : regenerate docs/math/compendium.md via example compendium_doc
#                 and assert byte-for-byte digest equality with the committed file.
#   5. summary  : log rows covered, controls caught, sections landed/skipped, gates (0).
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "compendium_laws" "N12-41"

# Plan row counts for sections D.1 through D.10.
_SECTION_SPECS="1:21 2:12 3:10 4:14 5:10 6:15 7:7 8:19 9:14 10:4"

_TOTAL_ROWS_COVERED=0
_TOTAL_CONTROLS_CAUGHT=0
_SECTIONS_LANDED=0
_SECTIONS_SKIPPED=0

# Emit one E2E assert line per frob.testevidence/1 record written during a test run.
_ingest_evidence_dir() {
  local sec_num="$1"
  local ev_dir="$2"
  local plant_coadjoint="$3"
  local count_file="${E2E_WORK_DIR}/ev_counts_d${sec_num}.txt"
  : > "${count_file}"

  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    return 0
  fi

  local ev_file=""
  for ev_file in "${ev_dir}"/*.ndjson; do
    [ -f "${ev_file}" ] || continue
    python3 - "${ev_file}" "${sec_num}" "${plant_coadjoint}" "${FROB_E2E_TOOL}" \
      "${E2E_LOG_FILE}" "${E2E_TRANSCRIPT_FILE}" "${E2E_RUN_ID}" "${E2E_SCRIPT}" \
      "${E2E_BEAD}" "${E2E_CURRENT_STEP}" "${E2E_SEED}" "${E2E_LAST_CMD_SEQ:-0}" \
      "${count_file}" <<'PY'
import json
import subprocess
import sys

(
    ev_path,
    sec_num,
    plant_coadjoint,
    e2e_tool,
    log_file,
    transcript_file,
    run_id,
    script_name,
    bead_key,
    step_name,
    seed_str,
    cmd_seq,
    count_file,
) = sys.argv[1:14]

nc_caught = 0
saw_planted_fail = False

with open(ev_path, "r", encoding="utf-8") as fh:
    for raw in fh:
        line = raw.strip()
        if not line:
            continue
        rec = json.loads(line)
        if rec.get("schema") != "frob.testevidence/1":
            continue
        kind = rec.get("kind", "")
        test_name = rec.get("test", "unnamed")
        row_id = rec.get("id", "")
        site = rec.get("site") or ""
        status = rec.get("status", "fail")
        obs = rec.get("observed") or ""
        exp = rec.get("expected") or ""
        tol = rec.get("tolerance")
        unit = rec.get("unit")

        assert_name = f"d{sec_num}:{test_name}:{row_id}"
        if plant_coadjoint == "1" and test_name == "d1_ad_star_pairing" and status != "pass":
            assert_name = "coadjoint-minus:d1_ad_star_pairing"
            saw_planted_fail = True

        cmd = [
            e2e_tool,
            "emit",
            "--log", log_file,
            "--transcript", transcript_file,
            "--event", "assert",
            "--run-id", run_id,
            "--script", script_name,
            "--bead", bead_key,
            "--step", step_name,
            "--seed", seed_str,
            "--name", assert_name,
        ]

        if kind == "nc":
            caught = status == "caught"
            if caught:
                nc_caught += 1
            a_status = "pass" if caught else "fail"
            cmd += [
                "--kind", "negative-control",
                "--status", a_status,
                "--caught", "true" if caught else "false",
                "--observed", obs,
                "--expected", exp,
            ]
        else:
            a_status = "pass" if status == "pass" else "fail"
            cmd += [
                "--kind", "check",
                "--status", a_status,
                "--observed", obs,
                "--expected", exp,
            ]

        if tol:
            cmd += ["--tolerance", str(tol)]
        if unit:
            cmd += ["--unit", str(unit)]
        if a_status == "fail":
            diff_path = f"{row_id} ({site})" if site else row_id
            cmd += [
                "--diff-path", diff_path,
                "--diff-observed", obs or "fail",
                "--diff-expected", exp or "pass",
                "--cmd-seq", cmd_seq,
                "--artifact", ev_path,
            ]

        subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL)

with open(count_file, "w", encoding="utf-8") as cf:
    cf.write(f"{nc_caught} {1 if saw_planted_fail else 0}\n")
PY
  done
}

# ---------------------------------------------------------------------------
# Step 1: registry
# ---------------------------------------------------------------------------
e2e_step "registry" "Run registry_check, compendium_registry, and check plan row counts"

e2e_cmd --name "registry_check" -- \
  cargo test -p frob-conformance --test registry_check
e2e_assert_eq "registry_check_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "compendium_registry" -- \
  cargo test -p frob-conformance --test compendium_registry
e2e_assert_eq "compendium_registry_exit" "${E2E_LAST_EXIT}" "0"

for _spec in ${_SECTION_SPECS}; do
  _sec_num="${_spec%%:*}"
  _exp_rows="${_spec##*:}"
  _toml="${E2E_REPO_ROOT}/registries/compendium/d${_sec_num}.toml"
  if [ -f "${_toml}" ]; then
    _actual_rows="$(grep -c '^\[\[row\]\]' "${_toml}" || true)"
    e2e_metric "d${_sec_num}_rows" "${_actual_rows}" "rows"
    e2e_assert_eq "d${_sec_num}_row_count" "${_actual_rows}" "${_exp_rows}"
    _TOTAL_ROWS_COVERED=$((_TOTAL_ROWS_COVERED + _actual_rows))
    _SECTIONS_LANDED=$((_SECTIONS_LANDED + 1))
  else
    e2e_skip "section D.${_sec_num} not yet landed (expected ${_exp_rows} rows)"
    _SECTIONS_SKIPPED=$((_SECTIONS_SKIPPED + 1))
  fi
done

# ---------------------------------------------------------------------------
# Step 2: laws
# ---------------------------------------------------------------------------
e2e_step "laws" "Run landed compendium law/KA/negative-control targets with evidence stream"

for _spec in ${_SECTION_SPECS}; do
  _sec_num="${_spec%%:*}"
  _target="compendium_d${_sec_num}"
  _test_src="${E2E_REPO_ROOT}/crates/frob-conformance/tests/${_target}.rs"
  if [ ! -f "${_test_src}" ]; then
    continue
  fi

  _ev_dir="${E2E_ARTIFACTS_DIR}/evidence_d${_sec_num}"
  mkdir -p "${_ev_dir}"

  _plant_coad="0"
  if [ "${_sec_num}" = "1" ] && e2e_plant "coadjoint-minus"; then
    _plant_coad="1"
    e2e_cmd --name "compendium_d1_planted" --expect-exit "0,1" -- \
      env "FROB_TEST_EVIDENCE_DIR=${_ev_dir}" "FROB_TEST_SEED=${E2E_SEED}" \
      cargo test -p frob-conformance --features plant-coadjoint-minus --test "${_target}"
  else
    e2e_cmd --name "${_target}" -- \
      env "FROB_TEST_EVIDENCE_DIR=${_ev_dir}" "FROB_TEST_SEED=${E2E_SEED}" \
      cargo test -p frob-conformance --test "${_target}"
    e2e_assert_eq "${_target}_exit" "${E2E_LAST_EXIT}" "0"
  fi

  _ingest_evidence_dir "${_sec_num}" "${_ev_dir}" "${_plant_coad}"

  _sec_nc="0"
  _saw_plant_fail="0"
  if [ -s "${E2E_WORK_DIR}/ev_counts_d${_sec_num}.txt" ]; then
    read -r _sec_nc _saw_plant_fail < "${E2E_WORK_DIR}/ev_counts_d${_sec_num}.txt"
  fi
  e2e_metric "d${_sec_num}_controls_caught" "${_sec_nc}" "controls"
  _TOTAL_CONTROLS_CAUGHT=$((_TOTAL_CONTROLS_CAUGHT + _sec_nc))

  # Guarantee the planted failure assertion is logged even if evidence was absent.
  if [ "${_plant_coad}" = "1" ] && [ "${_saw_plant_fail}" != "1" ] && [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_eq "coadjoint-minus:d1_ad_star_pairing" "exit:${E2E_LAST_EXIT}" "exit:0"
  fi
done

# ---------------------------------------------------------------------------
# Step 3: control (mandatory mutant check on every run)
# ---------------------------------------------------------------------------
e2e_step "control" "Verify plant-coadjoint-minus mutant is caught by d1_ad_star_pairing"

e2e_expect_caught "coadjoint-minus" -- \
  cargo test -p frob-conformance --features plant-coadjoint-minus --test compendium_d1 -- d1_ad_star_pairing

# ---------------------------------------------------------------------------
# Step 4: doc
# ---------------------------------------------------------------------------
e2e_step "doc" "Regenerate docs/math/compendium.md via example compendium_doc and compare bytes"

_GEN_DOC="${E2E_WORK_DIR}/compendium.md"
_COMMITTED_DOC="${E2E_REPO_ROOT}/docs/math/compendium.md"

e2e_cmd --name "compendium_doc" -- \
  cargo run -p frob-conformance --example compendium_doc -- --out "${_GEN_DOC}"
e2e_assert_eq "compendium_doc_exit" "${E2E_LAST_EXIT}" "0"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  if e2e_plant "doc-drift"; then
    # One-character drift planted in the regenerated scratch document.
    printf ' ' >> "${_GEN_DOC}"
  fi

  e2e_assert_file "compendium_doc_generated" "${_GEN_DOC}"
  e2e_assert_file "compendium_doc_committed" "${_COMMITTED_DOC}"

  _GEN_SHA="$("${FROB_E2E_TOOL}" cmp sha256 "${_GEN_DOC}")"
  _DOC_SHA="$("${FROB_E2E_TOOL}" cmp sha256 "${_COMMITTED_DOC}")"

  if e2e_plant "doc-drift"; then
    e2e_assert_digest_eq "doc-drift:compendium-bytes" "${_GEN_SHA}" "${_DOC_SHA}"
  else
    e2e_assert_digest_eq "compendium_doc_bytes" "${_GEN_SHA}" "${_DOC_SHA}"
  fi
fi

# ---------------------------------------------------------------------------
# Step 5: summary
# ---------------------------------------------------------------------------
e2e_step "summary" "Record compendium coverage summary (reference only, zero gates)"

e2e_metric "compendium_rows_covered" "${_TOTAL_ROWS_COVERED}" "rows"
e2e_metric "compendium_controls_caught" "${_TOTAL_CONTROLS_CAUGHT}" "controls"
e2e_metric "compendium_sections_landed" "${_SECTIONS_LANDED}" "sections"
e2e_metric "compendium_sections_skipped" "${_SECTIONS_SKIPPED}" "sections"
e2e_metric "compendium_gates_touched" "0" "gates"
