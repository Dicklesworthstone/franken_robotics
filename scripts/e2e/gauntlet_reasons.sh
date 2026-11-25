#!/usr/bin/env bash
# e2e: bead=N38-REF milestone=M0 labels=gauntlet,registry gates=GATE-GAUNT-01,GATE-GAUNT-10 plants=missing-reason
#
# End-to-end verification of the Gauntlet refusal vocabulary (N38-REF):
#   1. Run `FROB_TEST_SEED=0 cargo test -p frob-conformance --test reasons` on the
#      clean tree and assert exit 0.
#   2. Copy the registries into a scratch workspace without `defect-or-expect-empty`
#      in `reasons.toml`, run the same test target through the scratch registry root,
#      and assert exit 101 with `reason=` naming `defect-or-expect-empty` on the first
#      failure line; log the expected harness `refusal` event with `reason=removed`.
#   3. Scan every `refusal` event of the newest log of each script under
#      `target/e2e-logs/` and assert every `reason` is registered in `reasons.toml`
#      and every `code` is a row of `registries/refusals.toml` (threshold: 0 unknowns).
#
# Idempotent; no device time; bash 3.2 compatible; never deletes any file or directory.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "gauntlet_reasons" "N38-REF"
e2e_gate "GATE-GAUNT-01"
e2e_gate "GATE-GAUNT-10"

_make_scratch_reasons_root() {
  local label="$1"
  local missing_id="$2"
  local dst="${E2E_WORK_DIR}/ws_${label}"
  mkdir -p "${dst}/crates/frob-conformance/registry" "${dst}/crates/frob-conformance/tests" "${dst}/crates/frob-conformance/src" "${dst}/registries" "${dst}/scripts/e2e"
  cp "${E2E_REPO_ROOT}/Cargo.toml" "${dst}/Cargo.toml"
  if [ -f "${E2E_REPO_ROOT}/franken-stack.lock" ]; then
    cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  fi
  cp -R "${E2E_REPO_ROOT}/registries/." "${dst}/registries/"
  cp -R "${E2E_REPO_ROOT}/crates/frob-conformance/registry/." "${dst}/crates/frob-conformance/registry/"
  cp -R "${E2E_REPO_ROOT}/crates/frob-conformance/src/." "${dst}/crates/frob-conformance/src/"
  cp -R "${E2E_REPO_ROOT}/crates/frob-conformance/tests/." "${dst}/crates/frob-conformance/tests/"
  cp -R "${E2E_REPO_ROOT}/scripts/e2e/." "${dst}/scripts/e2e/"

  python3 - "${dst}/crates/frob-conformance/registry/reasons.toml" "${missing_id}" <<'PY'
import sys
path, target_id = sys.argv[1], sys.argv[2]
txt = open(path, "r", encoding="utf-8").read()
needle = f'id = "{target_id}"'
parts = txt.split("[[reason]]")
kept = [parts[0]]
removed = 0
for blk in parts[1:]:
    if needle in blk:
        removed += 1
    else:
        kept.append(blk)
if removed != 1:
    sys.stderr.write(f"expected 1 block for {target_id}, found {removed}\n")
    sys.exit(2)
open(path, "w", encoding="utf-8").write("[[reason]]".join(kept))
PY
  printf '%s' "${dst}"
}

_combined_output() {
  if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    cat "${E2E_LAST_STDOUT}"
  fi
  if [ -n "${E2E_LAST_STDERR:-}" ] && [ -f "${E2E_LAST_STDERR}" ]; then
    cat "${E2E_LAST_STDERR}"
  fi
}

# ---------------------------------------------------------------------------
# Step 1: Run `FROB_TEST_SEED=0 cargo test -p frob-conformance --test reasons`
# ---------------------------------------------------------------------------
e2e_step "clean_reasons_suite" "Run frob-conformance --test reasons on the clean tree"
if e2e_plant "missing-reason"; then
  WS_PLANT="$(_make_scratch_reasons_root "env_plant_missing" "defect-or-expect-empty")"
  e2e_cmd --name clean_reasons_test -- \
    env FROB_TEST_SEED=0 "FROB_REGISTRY_ROOT=${WS_PLANT}" \
    cargo test -p frob-conformance --test reasons -- --nocapture
else
  e2e_cmd --name clean_reasons_test -- \
    env FROB_TEST_SEED=0 cargo test -p frob-conformance --test reasons -- --nocapture
fi
e2e_assert_eq "clean_reasons_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"
e2e_assert_eq "clean_reasons_exit_g10" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"

# ---------------------------------------------------------------------------
# Step 2: Plant missing `defect-or-expect-empty` in scratch `reasons.toml`
# ---------------------------------------------------------------------------
e2e_step "plant_missing_record" "Planted scratch reasons.toml missing defect-or-expect-empty must exit 101 and print reason="
WS_MISSING="$(_make_scratch_reasons_root "missing_defect_or_expect_empty" "defect-or-expect-empty")"

e2e_expect_caught "missing-reason" --gate "GATE-GAUNT-01" -- \
  env FROB_TEST_SEED=0 "FROB_REGISTRY_ROOT=${WS_MISSING}" \
  cargo test -p frob-conformance --test reasons -- enumeration_matches_registry --nocapture

e2e_assert_eq "missing_reason_exit_101" "${E2E_LAST_EXIT}" "101" --gate "GATE-GAUNT-01"
e2e_assert_eq "missing_reason_exit_101_g10" "${E2E_LAST_EXIT}" "101" --gate "GATE-GAUNT-10"

_first_fail_ok="0"
if _combined_output | python3 -c '
import sys
txt = sys.stdin.read()
for line in txt.splitlines():
    if "reason=" in line:
        sys.exit(0 if "defect-or-expect-empty" in line else 1)
sys.exit(1)
'; then
  _first_fail_ok="1"
fi
e2e_assert_eq "missing_reason_first_failure_line" "${_first_fail_ok}" "1" --gate "GATE-GAUNT-01"

# Emit the expected harness `refusal` event (carrying `reason`) into this script's log
# so both `frob-e2e lint` and Step 3 exercise the Gauntlet refusal-event schema.
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  "${FROB_E2E_TOOL}" emit \
    --log "${E2E_LOG_FILE}" \
    --transcript "${E2E_TRANSCRIPT_FILE}" \
    --event refusal \
    --run-id "${E2E_RUN_ID}" \
    --script "${E2E_SCRIPT}" \
    --bead "${E2E_BEAD}" \
    --step "${E2E_CURRENT_STEP}" \
    --seed "${E2E_SEED}" \
    --reason "removed" \
    --exit 101 \
    --expected true \
    --message "planted scratch reasons.toml missing defect-or-expect-empty" \
    --repairs "Restore the removed identifier" >/dev/null
fi

# ---------------------------------------------------------------------------
# Step 3: Scan newest E2E logs for unknown harness reasons or public codes,
#         and emit registry inventory metrics
# ---------------------------------------------------------------------------
e2e_step "scan_e2e_refusal_logs" "Verify all refusal events in newest target/e2e-logs/* logs resolve against reasons.toml and refusals.toml"

SCAN_REPORT="${E2E_WORK_DIR}/scan_report.txt"
LOGS_ROOT="${E2E_REPO_ROOT}/target/e2e-logs"
if [ -n "${E2E_LOG_FILE:-}" ]; then
  _log_parent="$(dirname "$(dirname "${E2E_LOG_FILE}")")"
  if [ -d "${_log_parent}" ]; then
    LOGS_ROOT="${_log_parent}"
  fi
fi
_scan_rc=0
python3 - \
  "${E2E_REPO_ROOT}/crates/frob-conformance/registry/reasons.toml" \
  "${E2E_REPO_ROOT}/registries/refusals.toml" \
  "${E2E_REPO_ROOT}/crates/frob-conformance/src/reasons.rs" \
  "${LOGS_ROOT}" \
  > "${SCAN_REPORT}" <<'PY' || _scan_rc=$?
import json, os, re, sys

reasons_path, refusals_path, reasons_rs_path, logs_root = sys.argv[1:5]

reasons_txt = open(reasons_path, "r", encoding="utf-8").read()
blocks = reasons_txt.split("[[reason]]")[1:]
registered_reasons = set()
by_exit = {}
by_tool = {}
for blk in blocks:
    m_id = re.search(r'(?m)^id\s*=\s*"([^"]+)"', blk)
    m_exit = re.search(r'(?m)^exit\s*=\s*(\d+)', blk)
    m_tool = re.search(r'(?m)^tool\s*=\s*\[([^\]]*)\]', blk)
    if m_id:
        registered_reasons.add(m_id.group(1))
    if m_exit:
        ex = int(m_exit.group(1))
        by_exit[ex] = by_exit.get(ex, 0) + 1
    if m_tool:
        for t in re.findall(r'"([^"]+)"', m_tool.group(1)):
            by_tool[t] = by_tool.get(t, 0) + 1

refusals_txt = open(refusals_path, "r", encoding="utf-8").read()
registered_codes = set(re.findall(r'(?m)^code\s*=\s*"(FROB-REF-\d{3})"', refusals_txt))

reasons_rs_txt = open(reasons_rs_path, "r", encoding="utf-8").read()
public_codes_cited = len(set(re.findall(r'code:\s*"(FROB-REF-\d{3})"', reasons_rs_txt)))

per_script = {}
total_events = 0
unknowns = []

if os.path.isdir(logs_root):
    for name in sorted(os.listdir(logs_root)):
        sdir = os.path.join(logs_root, name)
        if not os.path.isdir(sdir):
            continue
        ndjsons = sorted(f for f in os.listdir(sdir) if f.endswith(".ndjson"))
        if not ndjsons:
            continue
        newest = os.path.join(sdir, ndjsons[-1])
        ev_count = 0
        with open(newest, "r", encoding="utf-8", errors="replace") as fh:
            for ln, raw in enumerate(fh, 1):
                s = raw.strip()
                if not s:
                    continue
                try:
                    obj = json.loads(s)
                except Exception:
                    continue
                if obj.get("event") != "refusal":
                    continue
                ev_count += 1
                total_events += 1
                reason = obj.get("reason")
                code = obj.get("code")
                if not reason and not code:
                    unknowns.append(f"{name}:{ln}: refusal missing both reason and code")
                    continue
                if reason and reason not in registered_reasons:
                    unknowns.append(f"{name}:{ln}: unknown reason={reason}")
                if code and code not in registered_codes:
                    unknowns.append(f"{name}:{ln}: unknown code={code}")
        per_script[name] = ev_count

print(f"REASONS_REGISTERED={len(registered_reasons)}")
print(f"REASONS_EXIT_0={by_exit.get(0, 0)}")
print(f"REASONS_EXIT_3={by_exit.get(3, 0)}")
print(f"REASONS_EXIT_101={by_exit.get(101, 0)}")
print(f"TOOLS_COVERED={len(by_tool)}")
print(f"PUBLIC_CODES_CITED={public_codes_cited}")
print(f"EVENTS_SCANNED={total_events}")
print(f"UNKNOWNS_FOUND={len(unknowns)}")
for k in sorted(per_script):
    print(f"SCRIPT_EVENTS:{k}={per_script[k]}")
for u in unknowns:
    print(f"UNKNOWN:{u}")
PY

e2e_assert_eq "scan_script_exit" "${_scan_rc}" "0" --gate "GATE-GAUNT-01"

REASONS_REGISTERED="$(awk -F= '/^REASONS_REGISTERED=/{print $2}' "${SCAN_REPORT}")"
REASONS_EXIT_0="$(awk -F= '/^REASONS_EXIT_0=/{print $2}' "${SCAN_REPORT}")"
REASONS_EXIT_3="$(awk -F= '/^REASONS_EXIT_3=/{print $2}' "${SCAN_REPORT}")"
REASONS_EXIT_101="$(awk -F= '/^REASONS_EXIT_101=/{print $2}' "${SCAN_REPORT}")"
TOOLS_COVERED="$(awk -F= '/^TOOLS_COVERED=/{print $2}' "${SCAN_REPORT}")"
PUBLIC_CODES_CITED="$(awk -F= '/^PUBLIC_CODES_CITED=/{print $2}' "${SCAN_REPORT}")"
EVENTS_SCANNED="$(awk -F= '/^EVENTS_SCANNED=/{print $2}' "${SCAN_REPORT}")"
UNKNOWNS_FOUND="$(awk -F= '/^UNKNOWNS_FOUND=/{print $2}' "${SCAN_REPORT}")"

e2e_metric "reasons_registered" "${REASONS_REGISTERED}" "reasons"
e2e_metric "reasons_exit_0" "${REASONS_EXIT_0}" "reasons"
e2e_metric "reasons_exit_3" "${REASONS_EXIT_3}" "reasons"
e2e_metric "reasons_exit_101" "${REASONS_EXIT_101}" "reasons"
e2e_metric "tools_covered" "${TOOLS_COVERED}" "tools"
e2e_metric "public_codes_cited" "${PUBLIC_CODES_CITED}" "codes"
e2e_metric "refusal_events_scanned" "${EVENTS_SCANNED}" "events"
e2e_metric "unknown_reasons_or_codes" "${UNKNOWNS_FOUND}" "unknowns" --op "==" --threshold 0 --gate "GATE-GAUNT-01"
e2e_assert_eq "zero_unknown_reasons_or_codes_g10" "${UNKNOWNS_FOUND}" "0" --gate "GATE-GAUNT-10"

while IFS= read -r _sline; do
  case "${_sline}" in
    SCRIPT_EVENTS:*)
      _rest="${_sline#SCRIPT_EVENTS:}"
      _sname="${_rest%%=*}"
      _scnt="${_rest#*=}"
      e2e_metric "script_refusal_events_${_sname}" "${_scnt}" "events"
      ;;
  esac
done < "${SCAN_REPORT}"

