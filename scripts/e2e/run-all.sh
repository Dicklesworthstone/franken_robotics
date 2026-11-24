#!/usr/bin/env bash
# End-to-end test suite runner (bead STD-E2E).
#
# Selects scripts under scripts/e2e/ (or --scripts-dir) by `# e2e:` headers via
# `frob-e2e list`, runs them one at a time under their per-script timeouts (with
# process-group termination on timeout or interrupt), lints every produced log,
# writes a runner index at target/e2e-logs/_runs/<utc>.run.ndjson, and prints
# one summary line per script.
#
# Exit codes:
#   0  every selected script passed and linted (or verified all plants)
#   1  one or more selected scripts failed or failed lint
#   2  harness error, invalid usage, empty selection, or unapproved >3600s timeout
#
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

SCRIPTS_DIR="${SCRIPT_DIR}"
LOGS_DIR="${E2E_LOGS_ROOT:-${REPO_ROOT}/target/e2e-logs}"
MILESTONE=""
THROUGH=""
LABEL=""
GATE=""
BEAD=""
SCRIPT_FILTER=""
LIST_ONLY="0"
SMALL_MODE="0"
DEVICES=""
FAIL_FAST="0"
VERIFY_PLANTS="0"
APPROVED_ENVELOPE=""

usage() {
  cat <<'EOF'
Usage: scripts/e2e/run-all.sh [OPTIONS]

Options:
  --milestone M<n>           Select scripts with exact milestone M0..M8
  --through M<n>             Select scripts with milestone M0..M<n>
  --label a,b                Select scripts carrying all listed labels
  --gate GATE-...            Select scripts touching the named gate
  --bead <key>               Select scripts for the named bead key
  --script <name>            Select a single script by name (without .sh)
  --scripts-dir <dir>        Override scripts directory (default: scripts/e2e)
  --logs-dir <dir>           Override log root (default: target/e2e-logs)
  --list                     List selected scripts and exit 0
  --small                    Run with E2E_SMALL=1
  --devices <csv>            Run with E2E_DEVICES=<csv>
  --fail-fast                Stop after the first failing script
  --verify-plants            Run each script once per declared plant and assert exit 1
  --approved-envelope <id>   Required when any selected script declares timeout_s > 3600
  -h, --help                 Show this help and exit 0
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --milestone) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; MILESTONE="$2"; shift 2 ;;
    --through) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; THROUGH="$2"; shift 2 ;;
    --label) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; LABEL="$2"; shift 2 ;;
    --gate) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; GATE="$2"; shift 2 ;;
    --bead) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; BEAD="$2"; shift 2 ;;
    --script) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; SCRIPT_FILTER="$2"; shift 2 ;;
    --scripts-dir) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; SCRIPTS_DIR="$2"; shift 2 ;;
    --logs-dir) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; LOGS_DIR="$2"; shift 2 ;;
    --list) LIST_ONLY="1"; shift ;;
    --small) SMALL_MODE="1"; shift ;;
    --devices) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; DEVICES="$2"; shift 2 ;;
    --fail-fast) FAIL_FAST="1"; shift ;;
    --verify-plants) VERIFY_PLANTS="1"; shift ;;
    --approved-envelope) [ "$#" -ge 2 ] || { usage >&2; exit 2; }; APPROVED_ENVELOPE="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'run-all.sh: unknown option: %s\n' "$1" >&2; usage >&2; exit 2 ;;
  esac
done

ensure_tool() {
  if [ -n "${FROB_E2E_TOOL:-}" ] && [ -x "${FROB_E2E_TOOL}" ]; then
    return 0
  fi
  local tbase="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-e2e"
  local cand
  if [ -n "${CARGO_TARGET_DIR:-}" ]; then
    for cand in "${CARGO_TARGET_DIR}/debug/frob-e2e" "${CARGO_TARGET_DIR}/release/frob-e2e"; do
      if [ -x "$cand" ]; then
        FROB_E2E_TOOL="$cand"
        export FROB_E2E_TOOL
        return 0
      fi
    done
  fi
  for cand in \
    "${REPO_ROOT}/target/debug/frob-e2e" \
    "${REPO_ROOT}/target/release/frob-e2e" \
    "${tbase}/debug/frob-e2e" \
    "${tbase}/release/frob-e2e"; do
    if [ -x "$cand" ]; then
      FROB_E2E_TOOL="$cand"
      export FROB_E2E_TOOL
      return 0
    fi
  done
  if ! command -v cargo >/dev/null 2>&1; then
    printf 'run-all.sh: cargo not found and FROB_E2E_TOOL is unset\n' >&2
    exit 2
  fi
  mkdir -p "$tbase"
  CARGO_TARGET_DIR="$tbase" cargo build -p frob-conformance --bin frob-e2e --manifest-path "${REPO_ROOT}/Cargo.toml" >&2 || exit 2
  FROB_E2E_TOOL="${tbase}/debug/frob-e2e"
  export FROB_E2E_TOOL
  [ -x "$FROB_E2E_TOOL" ] || exit 2
}

ensure_tool

LIST_ARGS=(list --scripts-dir "${SCRIPTS_DIR}")
[ -z "$MILESTONE" ] || LIST_ARGS=("${LIST_ARGS[@]}" --milestone "$MILESTONE")
[ -z "$THROUGH" ] || LIST_ARGS=("${LIST_ARGS[@]}" --through "$THROUGH")
[ -z "$LABEL" ] || LIST_ARGS=("${LIST_ARGS[@]}" --label "$LABEL")
[ -z "$GATE" ] || LIST_ARGS=("${LIST_ARGS[@]}" --gate "$GATE")
[ -z "$BEAD" ] || LIST_ARGS=("${LIST_ARGS[@]}" --bead "$BEAD")
[ -z "$SCRIPT_FILTER" ] || LIST_ARGS=("${LIST_ARGS[@]}" --script "$SCRIPT_FILTER")

if [ "$LIST_ONLY" = "1" ]; then
  out="$("${FROB_E2E_TOOL}" "${LIST_ARGS[@]}")" || exit 2
  if [ -z "$out" ]; then
    printf 'run-all.sh: empty selection\n' >&2
    exit 2
  fi
  printf '%s\n' "$out"
  exit 0
fi

# Fetch tab-separated table from `frob-e2e list`:
# path<TAB>bead<TAB>milestone<TAB>seed<TAB>timeout_s<TAB>labels<TAB>gates<TAB>plants
SELECTED="$("${FROB_E2E_TOOL}" "${LIST_ARGS[@]}")" || exit 2
if [ -z "$SELECTED" ]; then
  printf 'run-all.sh: no scripts matched the selection\n' >&2
  exit 2
fi

# Run a single script in its own process group with a wall-clock timeout and
# clean SIGINT/SIGTERM process-group teardown. Sets RUN_RC.
run_with_timeout() {
  local timeout_s="$1"
  shift
  RUN_RC=0
  if command -v python3 >/dev/null 2>&1; then
    python3 - "$timeout_s" "$@" <<'PY' || RUN_RC=$?
import os, signal, subprocess, sys
timeout_s = float(sys.argv[1])
cmd = sys.argv[2:]
proc = subprocess.Popen(cmd, start_new_session=True)

def _terminate_pg(signum, _frame):
    try:
        os.killpg(proc.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        proc.wait(timeout=2.0)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
    sys.exit(128 + signum)

signal.signal(signal.SIGINT, _terminate_pg)
signal.signal(signal.SIGTERM, _terminate_pg)

try:
    rc = proc.wait(timeout=timeout_s)
    sys.exit(rc)
except subprocess.TimeoutExpired:
    try:
        os.killpg(proc.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        proc.wait(timeout=2.0)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
    sys.exit(124)
PY
  else
    "$@" || RUN_RC=$?
  fi
}

RUN_UTC="$(date -u +%Y%m%dT%H%M%SZ)"
RUNS_DIR="${LOGS_DIR}/_runs"
mkdir -p "$RUNS_DIR"
RUN_INDEX="${RUNS_DIR}/${RUN_UTC}.run.ndjson"
if ! (set -C; : > "$RUN_INDEX") 2>/dev/null; then
  RUN_INDEX="${RUNS_DIR}/${RUN_UTC}-$$.run.ndjson"
  run_idx_n=0
  while ! (set -C; : > "$RUN_INDEX") 2>/dev/null; do
    run_idx_n=$((run_idx_n + 1))
    RUN_INDEX="${RUNS_DIR}/${RUN_UTC}-$$-${run_idx_n}.run.ndjson"
  done
fi

append_index_line() {
  local sname="$1" sbead="$2" plant="$3" rc="$4" lint_rc="$5" log_path="$6"
  printf '{"ts":"%s","script":"%s","bead":"%s","plant":"%s","exit":%s,"lint_exit":%s,"log":"%s"}\n' \
    "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$sname" "$sbead" "$plant" "$rc" "$lint_rc" "$log_path" \
    >> "$RUN_INDEX"
}

OVERALL_RC=0
COUNT=0
PLANTS_RUN=0

run_one_invocation() {
  local sname="$1" spath="$2" sbead="$3" timeout_s="$4" plant="$5"

  if [ "${timeout_s:-600}" -gt 3600 ] && [ -z "${APPROVED_ENVELOPE}" ]; then
    printf 'ERROR %-32s timeout_s=%s > 3600 without --approved-envelope\n' "$sname" "$timeout_s" >&2
    exit 2
  fi

  local env_cmd=(env "E2E_LOGS_ROOT=${LOGS_DIR}" "FROB_E2E_TOOL=${FROB_E2E_TOOL}")
  if [ "$SMALL_MODE" = "1" ]; then
    env_cmd=("${env_cmd[@]}" "E2E_SMALL=1")
  fi
  if [ -n "$DEVICES" ]; then
    env_cmd=("${env_cmd[@]}" "E2E_DEVICES=${DEVICES}")
  fi
  if [ -n "$plant" ]; then
    env_cmd=("${env_cmd[@]}" "E2E_PLANT_FAILURE=${plant}")
  else
    env_cmd=("${env_cmd[@]}" -u E2E_PLANT_FAILURE)
  fi

  run_with_timeout "${timeout_s:-600}" "${env_cmd[@]}" bash "$spath"
  local rc="$RUN_RC"

  local latest_file="${LOGS_DIR}/${sname}/LATEST"
  local log_path="" lint_rc=0
  if [ -f "$latest_file" ]; then
    local stem
    stem="$(cat "$latest_file")"
    log_path="${LOGS_DIR}/${sname}/${stem}.ndjson"
  fi

  if [ -n "$log_path" ] && [ -f "$log_path" ]; then
    "${FROB_E2E_TOOL}" lint "$log_path" >/dev/null 2>&1 || lint_rc=$?
  else
    lint_rc=1
  fi

  append_index_line "$sname" "$sbead" "$plant" "$rc" "$lint_rc" "$log_path"

  if [ -n "$plant" ]; then
    PLANTS_RUN=$((PLANTS_RUN + 1))
    # Planted run MUST exit 1, lint cleanly, and contain a failed assertion naming the plant
    local plant_named=0
    if [ -n "$log_path" ] && [ -f "$log_path" ]; then
      if grep -E '"event"[[:space:]]*:[[:space:]]*"assert"' "$log_path" \
        | grep -E '"status"[[:space:]]*:[[:space:]]*"fail"' \
        | grep -Fq "$plant"; then
        plant_named=1
      fi
    fi
    if [ "$rc" -eq 1 ] && [ "$lint_rc" -eq 0 ] && [ "$plant_named" -eq 1 ]; then
      printf 'PASS  %-32s plant=%s exit=1 (plant caught)\n' "$sname" "$plant"
    else
      printf 'FAIL  %-32s plant=%s exit=%s lint=%s plant_named=%s log=%s\n' "$sname" "$plant" "$rc" "$lint_rc" "$plant_named" "$log_path"
      OVERALL_RC=1
      [ "$FAIL_FAST" = "1" ] && exit 1
    fi
  else
    if [ "$rc" -eq 0 ] && [ "$lint_rc" -eq 0 ]; then
      printf 'PASS  %-32s exit=0 lint=ok log=%s\n' "$sname" "$log_path"
    elif [ "$rc" -eq 77 ]; then
      printf 'SKIP  %-32s exit=77 lint=%s log=%s\n' "$sname" "$lint_rc" "$log_path"
      # A skip is never counted as a pass when running a selected suite
      OVERALL_RC=1
      [ "$FAIL_FAST" = "1" ] && exit 1
    elif [ "$rc" -eq 2 ]; then
      printf 'ERROR %-32s exit=2 lint=%s log=%s\n' "$sname" "$lint_rc" "$log_path"
      [ "$OVERALL_RC" -eq 1 ] || OVERALL_RC=2
      [ "$FAIL_FAST" = "1" ] && exit 2
    else
      printf 'FAIL  %-32s exit=%s lint=%s log=%s\n' "$sname" "$rc" "$lint_rc" "$log_path"
      OVERALL_RC=1
      [ "$FAIL_FAST" = "1" ] && exit 1
    fi
  fi
}

while IFS= read -r row; do
  [ -n "$row" ] || continue
  # Use `cut -f` rather than `IFS=$'\t' read` because POSIX read collapses consecutive tabs.
  spath="$(printf '%s\n' "$row" | cut -f1)"
  sbead="$(printf '%s\n' "$row" | cut -f2)"
  stimeout="$(printf '%s\n' "$row" | cut -f5)"
  splants="$(printf '%s\n' "$row" | cut -f8)"
  [ -n "$spath" ] || continue
  sname="$(basename "$spath" .sh)"
  COUNT=$((COUNT + 1))
  if [ "$VERIFY_PLANTS" = "1" ]; then
    if [ -z "$splants" ] || [ "$splants" = "-" ]; then
      continue
    fi
    IFS_SAVED="$IFS"
    IFS=","
    for p in $splants; do
      IFS="$IFS_SAVED"
      [ -n "$p" ] && run_one_invocation "$sname" "$spath" "$sbead" "$stimeout" "$p"
      IFS=","
    done
    IFS="$IFS_SAVED"
  else
    run_one_invocation "$sname" "$spath" "$sbead" "$stimeout" ""
  fi
done <<EOF
$SELECTED
EOF

if [ "$COUNT" -eq 0 ]; then
  printf 'run-all.sh: empty selection\n' >&2
  exit 2
fi
if [ "$VERIFY_PLANTS" = "1" ] && [ "$PLANTS_RUN" -eq 0 ]; then
  printf 'run-all.sh: --verify-plants matched zero declared plants\n' >&2
  exit 2
fi

exit "$OVERALL_RC"
