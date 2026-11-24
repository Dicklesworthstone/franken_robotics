#!/usr/bin/env bash
# Shared end-to-end test harness helper library (bead STD-E2E).
#
# Sourced by scripts/e2e/<name>.sh. Compatible with bash 3.2 (macOS) and Linux.
# Every log line (schema frob.e2elog/1) is written and read through the std-only
# companion binary `frob-e2e` (crates/frob-conformance/src/bin/frob-e2e.rs).
# Never deletes files or directories (COMMON.md 9b).
#
# Reserved public prefix `e2e_` (17 helpers):
#   e2e_init, e2e_step, e2e_cmd, e2e_get, e2e_assert_*, e2e_expect_refusal,
#   e2e_expect_caught, e2e_metric, e2e_gate, e2e_skip, e2e_plant,
#   e2e_scene_ready, e2e_key, e2e_fixture, e2e_tier, e2e_check,
#   e2e_registries_scratch, e2e_session.

# Guard against direct execution
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
  printf 'scripts/e2e/lib.sh must be sourced, not executed\n' >&2
  exit 2
fi

_E2E_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
E2E_REPO_ROOT="$(cd "${_E2E_LIB_DIR}/../.." && pwd)"
export E2E_REPO_ROOT

# Global harness state (initialized by e2e_init; preserve caller E2E_SEED if set)
E2E_SCRIPT=""
E2E_BEAD=""
E2E_RUN_ID=""
E2E_SEED="${E2E_SEED:-}"
E2E_CURRENT_STEP=""
E2E_LOG_FILE=""
E2E_TRANSCRIPT_FILE=""
E2E_ARTIFACTS_DIR=""
E2E_SCRATCH_DIR=""
E2E_WORK_DIR=""
E2E_META_DIR=""
E2E_SCALE="full"
E2E_REGISTRIES_MODE="of-record"
E2E_LAST_EXIT="0"
E2E_LAST_STDOUT=""
E2E_LAST_STDERR=""
E2E_LAST_CMD_SEQ="0"
E2E_LAST_RECEIPT=""
E2E_LAST_SECRET_WRITTEN="0"
_E2E_FINISHED="0"
_E2E_INITIALIZED="0"

_e2e_harness_error() {
  local msg="$1"
  printf 'e2e harness error: %s\n' "$msg" >&2
  if [ -n "${E2E_META_DIR}" ] && [ -d "${E2E_META_DIR}" ]; then
    printf '2\n' > "${E2E_META_DIR}/harness_exit"
  fi
  exit 2
}

_e2e_ensure_tool() {
  if [ -n "${FROB_E2E_TOOL:-}" ] && [ -x "${FROB_E2E_TOOL}" ]; then
    return 0
  fi

  local tbase="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-e2e"
  local cand
  for cand in \
    "${E2E_REPO_ROOT}/target/debug/frob-e2e" \
    "${E2E_REPO_ROOT}/target/release/frob-e2e" \
    "${tbase}/debug/frob-e2e" \
    "${tbase}/release/frob-e2e"; do
    if [ -x "$cand" ]; then
      FROB_E2E_TOOL="$cand"
      export FROB_E2E_TOOL
      return 0
    fi
  done

  # Build via PATH cargo shim (never invoke cargo by absolute path).
  if ! command -v cargo >/dev/null 2>&1; then
    _e2e_harness_error "cargo not found on PATH and FROB_E2E_TOOL is unset"
  fi
  mkdir -p "$tbase"
  if ! CARGO_TARGET_DIR="$tbase" cargo build -p frob-conformance --bin frob-e2e --manifest-path "${E2E_REPO_ROOT}/Cargo.toml" >&2; then
    _e2e_harness_error "failed to build frob-e2e"
  fi
  if [ -x "${tbase}/debug/frob-e2e" ]; then
    FROB_E2E_TOOL="${tbase}/debug/frob-e2e"
    export FROB_E2E_TOOL
    return 0
  fi
  _e2e_harness_error "frob-e2e binary not found after build"
}

_e2e_read_header_key() {
  local script_path="$1"
  local key="$2"
  [ -f "$script_path" ] || return 0
  local line token
  while IFS= read -r line; do
    case "$line" in
      "# e2e:"*)
        line="${line#\# e2e:}"
        for token in $line; do
          case "$token" in
            "${key}="*)
              printf '%s' "${token#*=}"
              return 0
              ;;
          esac
        done
        ;;
      "#"*|"")
        ;;
      *)
        break
        ;;
    esac
  done < "$script_path"
}

_e2e_read_toolchain() {
  local tc_file="${E2E_REPO_ROOT}/rust-toolchain.toml"
  if [ -f "$tc_file" ]; then
    local ch
    ch="$(awk -F'"' '/^[[:space:]]*channel[[:space:]]*=/{print $2; exit}' "$tc_file")"
    if [ -n "$ch" ]; then
      printf '%s' "$ch"
      return 0
    fi
  fi
  printf 'unknown'
}

_e2e_host_fingerprint() {
  local os arch raw
  os="$(uname -s 2>/dev/null || echo unknown)"
  arch="$(uname -m 2>/dev/null || echo unknown)"
  raw="${os}:${arch}"
  if [ -x "${FROB_E2E_TOOL:-}" ]; then
    local tmpf="${E2E_META_DIR}/host_fp.in"
    printf '%s' "$raw" > "$tmpf"
    "${FROB_E2E_TOOL}" cmp sha256 "$tmpf" 2>/dev/null | cut -c1-16
  else
    printf '0000000000000000'
  fi
}

_e2e_parse_kv_output() {
  local kv_text="$1"
  local line k v
  while IFS= read -r line; do
    case "$line" in
      *=*)
        k="${line%%=*}"
        v="${line#*=}"
        case "$k" in
          E2E_LAST_EXIT) E2E_LAST_EXIT="$v"; export E2E_LAST_EXIT ;;
          E2E_LAST_STDOUT) E2E_LAST_STDOUT="$v"; export E2E_LAST_STDOUT ;;
          E2E_LAST_STDERR) E2E_LAST_STDERR="$v"; export E2E_LAST_STDERR ;;
          E2E_LAST_CMD_SEQ) E2E_LAST_CMD_SEQ="$v"; export E2E_LAST_CMD_SEQ ;;
          E2E_LAST_RECEIPT) E2E_LAST_RECEIPT="$v"; export E2E_LAST_RECEIPT ;;
          E2E_LAST_SECRET_WRITTEN) E2E_LAST_SECRET_WRITTEN="$v" ;;
        esac
        ;;
    esac
  done <<EOF
$kv_text
EOF
}

_e2e_record_gate() {
  local g="$1"
  [ -n "$g" ] || return 0
  if ! grep -Fxq "$g" "${E2E_META_DIR}/gates.txt" 2>/dev/null; then
    printf '%s\n' "$g" >> "${E2E_META_DIR}/gates.txt"
  fi
}

_e2e_gates_csv() {
  if [ ! -s "${E2E_META_DIR}/gates.txt" ]; then
    printf ''
    return 0
  fi
  tr '\n' ',' < "${E2E_META_DIR}/gates.txt" | sed 's/,$//'
}

_e2e_extract_tags() {
  _E2E_TAG_ARGS=()
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --gate)
        [ "$#" -ge 2 ] || _e2e_harness_error "--gate requires an argument"
        _e2e_record_gate "$2"
        _E2E_TAG_ARGS=(${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"} --gate "$2")
        shift 2
        ;;
      --ka|--cx|--nc|--mr)
        [ "$#" -ge 2 ] || _e2e_harness_error "$1 requires an argument"
        _E2E_TAG_ARGS=(${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"} "$1" "$2")
        shift 2
        ;;
      *)
        _e2e_harness_error "unexpected trailing tag option: $1"
        ;;
    esac
  done
}

_e2e_finish() {
  local raw_rc=$?
  if [ "${_E2E_FINISHED}" = "1" ] || [ "${_E2E_INITIALIZED}" != "1" ]; then
    exit "$raw_rc"
  fi
  _E2E_FINISHED="1"

  # Close active step if any
  if [ -n "${E2E_CURRENT_STEP}" ]; then
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event end \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${E2E_CURRENT_STEP}" \
      --seed "${E2E_SEED}" >/dev/null 2>&1 || true
    E2E_CURRENT_STEP=""
  fi

  local harness_rc=""
  if [ -f "${E2E_META_DIR}/harness_exit" ]; then
    harness_rc="$(cat "${E2E_META_DIR}/harness_exit")"
  fi
  local reg_mode="${E2E_REGISTRIES_MODE}"
  if [ -f "${E2E_META_DIR}/registries_mode" ]; then
    reg_mode="$(cat "${E2E_META_DIR}/registries_mode")"
  fi

  local force_exit=""
  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    force_exit="77"
  elif [ -n "$harness_rc" ]; then
    force_exit="$harness_rc"
  elif [ "$raw_rc" -eq 2 ] || [ "$raw_rc" -eq 77 ]; then
    force_exit="$raw_rc"
  elif [ "$raw_rc" -ne 0 ]; then
    force_exit="1"
  fi

  local gates_csv
  gates_csv="$(_e2e_gates_csv)"

  local emit_args=(
    emit
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --event summary
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step ""
    --seed "${E2E_SEED}"
    --registries "${reg_mode}"
  )
  if [ -n "$gates_csv" ]; then
    emit_args=("${emit_args[@]}" --gates "$gates_csv")
  fi
  if [ -n "$force_exit" ]; then
    emit_args=("${emit_args[@]}" --exit "$force_exit")
  fi

  local sum_out sum_rc=0
  sum_out="$("${FROB_E2E_TOOL}" "${emit_args[@]}" 2>&1)" || sum_rc=$?
  if [ "$sum_rc" -ne 0 ]; then
    printf 'e2e harness error: failed to emit summary: %s\n' "$sum_out" >&2
    exit 2
  fi

  local final_exit="" line
  while IFS= read -r line; do
    case "$line" in
      E2E_SUMMARY_EXIT=*)
        final_exit="${line#E2E_SUMMARY_EXIT=}"
        ;;
    esac
  done <<EOF
$sum_out
EOF

  if [ -n "$force_exit" ]; then
    exit "$force_exit"
  elif [ -n "$final_exit" ]; then
    if [ "$final_exit" -eq 0 ] && [ "$raw_rc" -ne 0 ]; then
      exit 1
    fi
    exit "$final_exit"
  else
    exit "$raw_rc"
  fi
}

# ---------------------------------------------------------------------------
# 1. e2e_init <script> <bead>
# ---------------------------------------------------------------------------
e2e_init() {
  [ "$#" -ge 2 ] || _e2e_harness_error "e2e_init requires <script> <bead>"
  E2E_SCRIPT="$1"
  E2E_BEAD="$2"
  shift 2

  _e2e_ensure_tool

  # Resolve product binaries
  if [ -z "${FROB_BIN:-}" ]; then
    if command -v frob >/dev/null 2>&1; then
      FROB_BIN="$(command -v frob)"
    elif [ -x "${E2E_REPO_ROOT}/target/release/frob" ]; then
      FROB_BIN="${E2E_REPO_ROOT}/target/release/frob"
    else
      FROB_BIN="${E2E_REPO_ROOT}/target/debug/frob"
    fi
  fi
  FROBD_BIN="${FROBD_BIN:-$(command -v frobd 2>/dev/null || echo "${E2E_REPO_ROOT}/target/release/frobd")}"
  FROB_MCP_BIN="${FROB_MCP_BIN:-$(command -v frob-mcp 2>/dev/null || echo "${E2E_REPO_ROOT}/target/release/frob-mcp")}"
  export FROB_BIN FROBD_BIN FROB_MCP_BIN

  # Determine seed: FROB_E2E_SEED -> E2E_SEED -> header seed -> 0
  local hdr_seed=""
  if [ -n "${BASH_SOURCE[1]:-}" ]; then
    hdr_seed="$(_e2e_read_header_key "${BASH_SOURCE[1]}" "seed")"
  fi
  if [ -z "$hdr_seed" ] && [ -f "$0" ]; then
    hdr_seed="$(_e2e_read_header_key "$0" "seed")"
  fi
  E2E_SEED="${FROB_E2E_SEED:-${E2E_SEED:-${hdr_seed:-0}}}"
  case "$E2E_SEED" in
    ''|*[!0-9]*)
      _e2e_harness_error "seed must be an unsigned decimal integer, got: ${E2E_SEED}"
      ;;
  esac
  FROB_E2E_SEED="$E2E_SEED"
  export FROB_E2E_SEED E2E_SEED

  # Create log directory layout
  local logs_root="${E2E_LOGS_ROOT:-${E2E_REPO_ROOT}/target/e2e-logs}"
  local script_log_dir="${logs_root}/${E2E_SCRIPT}"
  mkdir -p "$script_log_dir"

  local utc_base utc_stamp suffix=0
  utc_base="$(date -u +%Y%m%dT%H%M%SZ)"
  utc_stamp="$utc_base"
  # Reserve the artifacts directory atomically with mkdir so concurrent runs
  # starting in the same second never share a log, transcript, or artifact dir.
  if ! mkdir "${script_log_dir}/${utc_stamp}.artifacts" 2>/dev/null; then
    utc_stamp="${utc_base}-$$"
    while ! mkdir "${script_log_dir}/${utc_stamp}.artifacts" 2>/dev/null; do
      suffix=$((suffix + 1))
      utc_stamp="${utc_base}-$$-${suffix}"
    done
  fi

  E2E_LOG_FILE="${script_log_dir}/${utc_stamp}.ndjson"
  E2E_TRANSCRIPT_FILE="${script_log_dir}/${utc_stamp}.transcript.txt"
  E2E_ARTIFACTS_DIR="${script_log_dir}/${utc_stamp}.artifacts"
  printf '%s\n' "${utc_stamp}" > "${script_log_dir}/.LATEST.$$"
  mv "${script_log_dir}/.LATEST.$$" "${script_log_dir}/LATEST"

  # Create scratch directory layout
  local scratch_root="${E2E_SCRATCH_ROOT:-${E2E_REPO_ROOT}/target/e2e-scratch}"
  local id_seed_dir="${scratch_root}/${E2E_SCRIPT}"
  mkdir -p "$id_seed_dir"
  local tmp_id_in="${id_seed_dir}/.id_${utc_stamp}_$$"
  printf '%s:%s:%s' "${E2E_SCRIPT}" "${utc_stamp}" "$$" > "$tmp_id_in"
  local hex8
  hex8="$("${FROB_E2E_TOOL}" cmp sha256 "$tmp_id_in" | cut -c1-8)"
  E2E_RUN_ID="${utc_stamp}-${hex8}"

  E2E_SCRATCH_DIR="${scratch_root}/${E2E_SCRIPT}/${E2E_RUN_ID}"
  E2E_WORK_DIR="${E2E_SCRATCH_DIR}/work"
  E2E_META_DIR="${E2E_SCRATCH_DIR}/_meta"
  FROB_STATE="${E2E_SCRATCH_DIR}/state"

  mkdir -p "${FROB_STATE}" "${E2E_WORK_DIR}" "${E2E_META_DIR}"
  chmod 0700 "${FROB_STATE}"
  : > "${E2E_META_DIR}/keys.txt"
  : > "${E2E_META_DIR}/gates.txt"
  printf 'of-record\n' > "${E2E_META_DIR}/registries_mode"
  export FROB_STATE E2E_RUN_ID E2E_LOG_FILE E2E_TRANSCRIPT_FILE E2E_ARTIFACTS_DIR E2E_SCRATCH_DIR E2E_WORK_DIR

  # Scale, devices, plant
  if [ "${E2E_SMALL:-0}" = "1" ] || [ "${E2E_QUICK:-0}" = "1" ]; then
    E2E_SCALE="small"
  else
    E2E_SCALE="full"
  fi
  local devices="${E2E_DEVICES:-cpu}"
  local plant="${E2E_PLANT_FAILURE:-}"

  # Repo provenance (strictly read-only)
  local commit dirty toolchain frob_ver host_os host_arch host_fp
  commit="$(git --no-optional-locks -C "${E2E_REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
  if [ -n "$(git --no-optional-locks -C "${E2E_REPO_ROOT}" status --porcelain --untracked-files=no 2>/dev/null || true)" ]; then
    dirty="true"
  else
    dirty="false"
  fi
  toolchain="$(_e2e_read_toolchain)"
  frob_ver="unbuilt"
  if [ -x "${FROB_BIN}" ]; then
    frob_ver="$("${FROB_BIN}" --version 2>/dev/null | head -n 1 || echo unknown)"
  fi
  host_os="$(uname -s 2>/dev/null | tr 'ABCDEFGHIJKLMNOPQRSTUVWXYZ' 'abcdefghijklmnopqrstuvwxyz' || echo unknown)"
  host_arch="$(uname -m 2>/dev/null || echo unknown)"
  host_fp="$(_e2e_host_fingerprint)"

  cd "${E2E_WORK_DIR}" || _e2e_harness_error "cannot cd to ${E2E_WORK_DIR}"

  local start_args=(
    emit
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --event start
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step ""
    --seed "${E2E_SEED}"
    --commit "${commit}"
    --dirty "${dirty}"
    --toolchain "${toolchain}"
    --frob-version "${frob_ver}"
    --host-os "${host_os}"
    --host-arch "${host_arch}"
    --host-fingerprint "${host_fp}"
    --scale "${E2E_SCALE}"
    --devices "${devices}"
    --plant "${plant}"
  )
  local arg
  for arg in "$@"; do
    start_args=("${start_args[@]}" --arg "$arg")
  done

  if ! "${FROB_E2E_TOOL}" "${start_args[@]}" >/dev/null; then
    _e2e_harness_error "failed to emit script start line"
  fi

  _E2E_INITIALIZED="1"
  trap _e2e_finish EXIT
}

# ---------------------------------------------------------------------------
# 2. e2e_step <id> <title>
# ---------------------------------------------------------------------------
e2e_step() {
  [ "$#" -ge 2 ] || _e2e_harness_error "e2e_step requires <id> <title>"
  local step_id="$1"
  local title="$2"

  if [ -n "${E2E_CURRENT_STEP}" ]; then
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event end \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${E2E_CURRENT_STEP}" \
      --seed "${E2E_SEED}" >/dev/null || _e2e_harness_error "failed to emit step end"
  fi

  E2E_CURRENT_STEP="$step_id"
  "${FROB_E2E_TOOL}" emit \
    --log "${E2E_LOG_FILE}" \
    --transcript "${E2E_TRANSCRIPT_FILE}" \
    --event start \
    --run-id "${E2E_RUN_ID}" \
    --script "${E2E_SCRIPT}" \
    --bead "${E2E_BEAD}" \
    --step "${E2E_CURRENT_STEP}" \
    --seed "${E2E_SEED}" \
    --title "${title}" >/dev/null || _e2e_harness_error "failed to emit step start"
}

# ---------------------------------------------------------------------------
# 3. e2e_cmd [--name n] [--expect-exit 0,2] -- <command...>
# ---------------------------------------------------------------------------
e2e_cmd() {
  local name="" expect_exit="0"
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --name)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_cmd --name requires a value"
        name="$2"
        shift 2
        ;;
      --expect-exit)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_cmd --expect-exit requires a value"
        expect_exit="$2"
        shift 2
        ;;
      --)
        shift
        break
        ;;
      *)
        _e2e_harness_error "e2e_cmd: unexpected option '$1' (missing '--'?)"
        ;;
    esac
  done
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_cmd requires a command after --"

  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    printf '[dry-run] %s\n' "$*"
    return 0
  fi

  local run_args=(
    run
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --artifacts-dir "${E2E_ARTIFACTS_DIR}"
    --scratch-root "${E2E_SCRATCH_DIR}"
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step "${E2E_CURRENT_STEP}"
    --seed "${E2E_SEED}"
    --expect-exit "${expect_exit}"
  )
  if [ -n "$name" ]; then
    run_args=("${run_args[@]}" --name "$name")
  fi
  if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
    run_args=("${run_args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
  fi

  local kv_out rc=0
  kv_out="$("${FROB_E2E_TOOL}" "${run_args[@]}" -- "$@")" || rc=$?
  if [ "$rc" -eq 2 ]; then
    _e2e_harness_error "frob-e2e run failed for command: $*"
  fi
  _e2e_parse_kv_output "$kv_out"
}

# ---------------------------------------------------------------------------
# 4. e2e_get [--file <path>] <json-pointer>
# ---------------------------------------------------------------------------
e2e_get() {
  local file="${E2E_LAST_STDOUT}"
  if [ "${1:-}" = "--file" ]; then
    [ "$#" -ge 3 ] || _e2e_harness_error "e2e_get --file requires <path> <json-pointer>"
    file="$2"
    shift 2
  fi
  [ "$#" -eq 1 ] || _e2e_harness_error "e2e_get requires <json-pointer>"
  local ptr="$1"
  if [ "$file" != "-" ]; then
    [ -n "$file" ] && [ -f "$file" ] || _e2e_harness_error "e2e_get: input file missing: ${file}"
  fi
  "${FROB_E2E_TOOL}" get --file "$file" "$ptr"
}

# ---------------------------------------------------------------------------
# 5. Assertions
# ---------------------------------------------------------------------------
_e2e_emit_assert() {
  local name="$1" kind="$2" status="$3"
  shift 3
  local args=(
    emit
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --event assert
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step "${E2E_CURRENT_STEP}"
    --seed "${E2E_SEED}"
    --name "${name}"
    --kind "${kind}"
    --status "${status}"
  )
  if [ "$status" = "fail" ]; then
    args=("${args[@]}" --cmd-seq "${E2E_LAST_CMD_SEQ:-0}")
    if [ -n "${E2E_LAST_STDOUT:-}" ]; then
      args=("${args[@]}" --artifact "${E2E_LAST_STDOUT}")
    fi
  fi
  if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
    args=("${args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
  fi
  "${FROB_E2E_TOOL}" "${args[@]}" "$@" >/dev/null || _e2e_harness_error "failed to emit assert ${name}"
}

e2e_assert_eq() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_assert_eq requires <name> <observed> <expected>"
  local name="$1" obs="$2" exp="$3"
  shift 3
  _e2e_extract_tags "$@"
  if "${FROB_E2E_TOOL}" cmp eq "$obs" "$exp" >/dev/null 2>&1; then
    _e2e_emit_assert "$name" "eq" "pass" --observed "$obs" --expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "eq" "fail" --observed "$obs" --expected "$exp" \
      --diff-path "$name" --diff-observed "$obs" --diff-expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_num() {
  [ "$#" -ge 4 ] || _e2e_harness_error "e2e_assert_num requires <name> <observed> <op> <threshold> [unit]"
  local name="$1" obs="$2" op="$3" thr="$4" unit=""
  shift 4
  if [ "$#" -gt 0 ]; then
    case "$1" in
      --*) ;;
      *) unit="$1"; shift ;;
    esac
  fi
  _e2e_extract_tags "$@"
  local extra=(--observed "$obs" --expected "$thr" --op "$op")
  if [ -n "$unit" ]; then
    extra=("${extra[@]}" --unit "$unit")
  fi
  if "${FROB_E2E_TOOL}" cmp num "$obs" "$op" "$thr" >/dev/null 2>&1; then
    _e2e_emit_assert "$name" "num" "pass" "${extra[@]}" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "num" "fail" "${extra[@]}" \
      --diff-path "$name" --diff-observed "$obs" --diff-expected "${op} ${thr}" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_close() {
  [ "$#" -ge 5 ] || _e2e_harness_error "e2e_assert_close requires <name> <observed> <expected> <abs|rel> <tol> [unit]"
  local name="$1" obs="$2" exp="$3" mode="$4" tol="$5" unit=""
  shift 5
  if [ "$#" -gt 0 ]; then
    case "$1" in
      --*) ;;
      *) unit="$1"; shift ;;
    esac
  fi
  _e2e_extract_tags "$@"
  local extra=(--observed "$obs" --expected "$exp" --op "$mode" --tolerance "$tol")
  if [ -n "$unit" ]; then
    extra=("${extra[@]}" --unit "$unit")
  fi
  if "${FROB_E2E_TOOL}" cmp close "$obs" "$exp" "$mode" "$tol" >/dev/null 2>&1; then
    _e2e_emit_assert "$name" "close" "pass" "${extra[@]}" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "close" "fail" "${extra[@]}" \
      --diff-path "$name" --diff-observed "$obs" --diff-expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_rational() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_assert_rational requires <name> <observed> <expected>"
  local name="$1" obs="$2" exp="$3"
  shift 3
  _e2e_extract_tags "$@"
  if "${FROB_E2E_TOOL}" cmp rational "$obs" "$exp" >/dev/null 2>&1; then
    _e2e_emit_assert "$name" "rational" "pass" --observed "$obs" --expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "rational" "fail" --observed "$obs" --expected "$exp" \
      --diff-path "$name" --diff-observed "$obs" --diff-expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_digest_eq() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_assert_digest_eq requires <name> <a> <b>"
  local name="$1" a="$2" b="$3"
  shift 3
  _e2e_extract_tags "$@"
  if "${FROB_E2E_TOOL}" cmp digest "$a" "$b" >/dev/null 2>&1; then
    _e2e_emit_assert "$name" "digest" "pass" --observed "$a" --expected "$b" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "digest" "fail" --observed "$a" --expected "$b" \
      --diff-path "$name" --diff-observed "$a" --diff-expected "$b" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_json() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_assert_json requires <name> <pointer> <expected>"
  local name="$1" ptr="$2" exp="$3"
  shift 3
  _e2e_extract_tags "$@"
  local obs="" rc=0
  if [ -n "${E2E_LAST_STDOUT}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    obs="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "$ptr" 2>/dev/null)" || rc=$?
  else
    rc=1
  fi
  if [ "$rc" -eq 0 ] && "${FROB_E2E_TOOL}" cmp eq "$obs" "$exp" >/dev/null 2>&1; then
    _e2e_emit_assert "$name" "json" "pass" --observed "$obs" --expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    [ "$rc" -eq 0 ] || obs="<missing:${ptr}>"
    _e2e_emit_assert "$name" "json" "fail" --observed "$obs" --expected "$exp" \
      --diff-path "$ptr" --diff-observed "$obs" --diff-expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_file() {
  [ "$#" -ge 2 ] || _e2e_harness_error "e2e_assert_file requires <name> <path>"
  local name="$1" fpath="$2"
  shift 2
  _e2e_extract_tags "$@"
  if [ -e "$fpath" ]; then
    local sha
    sha="$("${FROB_E2E_TOOL}" cmp sha256 "$fpath" 2>/dev/null || echo present)"
    _e2e_emit_assert "$name" "file" "pass" --observed "$sha" --expected "exists" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "file" "fail" --observed "missing" --expected "exists" \
      --diff-path "$fpath" --diff-observed "missing" --diff-expected "exists" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

e2e_assert_repairs_order() {
  [ "$#" -ge 2 ] || _e2e_harness_error "e2e_assert_repairs_order requires <name> <code1,code2,...>"
  local name="$1" codes="$2"
  shift 2
  _e2e_extract_tags "$@"
  local obs="" rc=0
  if [ -n "${E2E_LAST_STDOUT}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    obs="$("${FROB_E2E_TOOL}" cmp repairs "${E2E_LAST_STDOUT}" "$codes" 2>&1)" || rc=$?
  else
    obs="<no-stdout>"
    rc=1
  fi
  if [ "$rc" -eq 0 ]; then
    _e2e_emit_assert "$name" "refusal" "pass" --observed "$codes" --expected "$codes" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    _e2e_emit_assert "$name" "refusal" "fail" --observed "$obs" --expected "$codes" \
      --diff-path "/refusal/repairs" --diff-observed "$obs" --diff-expected "$codes" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

# ---------------------------------------------------------------------------
# 6. e2e_expect_refusal <FROB-REF-nnn> [exit] -- <command...>
# ---------------------------------------------------------------------------
e2e_expect_refusal() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_expect_refusal requires <CODE> [exit] -- <command...>"
  local code="$1"
  shift
  local exp_exit="3"
  if [ "${1:-}" != "--" ]; then
    exp_exit="$1"
    shift
  fi
  [ "${1:-}" = "--" ] || _e2e_harness_error "e2e_expect_refusal requires '--' before command"
  shift
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_expect_refusal requires a command after --"

  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    printf '[dry-run expect-refusal %s exit=%s] %s\n' "$code" "$exp_exit" "$*"
    return 0
  fi

  local run_args=(
    run
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --artifacts-dir "${E2E_ARTIFACTS_DIR}"
    --scratch-root "${E2E_SCRATCH_DIR}"
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step "${E2E_CURRENT_STEP}"
    --seed "${E2E_SEED}"
    --expect-refusal "$code"
    --expect-exit "$exp_exit"
  )
  if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
    run_args=("${run_args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
  fi

  local kv_out rc=0
  kv_out="$("${FROB_E2E_TOOL}" "${run_args[@]}" -- "$@")" || rc=$?
  if [ "$rc" -eq 2 ]; then
    _e2e_harness_error "frob-e2e run failed for e2e_expect_refusal $code"
  fi
  _e2e_parse_kv_output "$kv_out"
}

# ---------------------------------------------------------------------------
# 7. e2e_expect_caught <control> [--gate G] [--nc NC] -- <command...>
# ---------------------------------------------------------------------------
e2e_expect_caught() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_expect_caught requires <control> [tags...] -- <command...>"
  local control="$1"
  shift
  local extra_tags=()
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --gate)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_expect_caught --gate requires a value"
        _e2e_record_gate "$2"
        extra_tags=(${extra_tags[@]+"${extra_tags[@]}"} --gate "$2")
        shift 2
        ;;
      --nc|--cx|--ka|--mr)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_expect_caught $1 requires a value"
        extra_tags=(${extra_tags[@]+"${extra_tags[@]}"} "$1" "$2")
        shift 2
        ;;
      --)
        shift
        break
        ;;
      *)
        _e2e_harness_error "e2e_expect_caught: unexpected arg '$1' before --"
        ;;
    esac
  done
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_expect_caught requires a command after --"

  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    printf '[dry-run expect-caught %s] %s\n' "$control" "$*"
    return 0
  fi

  local run_args=(
    run
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --artifacts-dir "${E2E_ARTIFACTS_DIR}"
    --scratch-root "${E2E_SCRATCH_DIR}"
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step "${E2E_CURRENT_STEP}"
    --seed "${E2E_SEED}"
    --expect-caught "$control"
    ${extra_tags[@]+"${extra_tags[@]}"}
  )
  if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
    run_args=("${run_args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
  fi

  local kv_out rc=0
  kv_out="$("${FROB_E2E_TOOL}" "${run_args[@]}" -- "$@")" || rc=$?
  if [ "$rc" -eq 2 ]; then
    _e2e_harness_error "frob-e2e run failed for e2e_expect_caught $control"
  fi
  _e2e_parse_kv_output "$kv_out"
}

# ---------------------------------------------------------------------------
# 8. e2e_metric <name> <value> <unit> [op threshold] [--observe] [--gate G]
#               [--interval lo hi] [--n count]
#    (Includes review note 5 / Convention 15: --observe logs asserted:false +
#     verdict met|not-met and NEVER emits an assertion.)
# ---------------------------------------------------------------------------
e2e_metric() {
  [ "$#" -ge 3 ] || _e2e_harness_error "e2e_metric requires <name> <value> <unit>"
  local name="$1" val="$2" unit="$3"
  shift 3

  local op="" thr="" observe="0" gate="" iv_lo="" iv_hi="" n_cnt=""
  if [ "$#" -ge 2 ]; then
    case "$1" in
      "<"|"<="|">"|">="|"==")
        op="$1"
        thr="$2"
        shift 2
        ;;
    esac
  fi

  while [ "$#" -gt 0 ]; do
    case "$1" in
      --observe)
        observe="1"
        shift
        ;;
      --gate)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_metric --gate requires a value"
        gate="$2"
        _e2e_record_gate "$gate"
        shift 2
        ;;
      --interval)
        [ "$#" -ge 3 ] || _e2e_harness_error "e2e_metric --interval requires <lo> <hi>"
        iv_lo="$2"
        iv_hi="$3"
        shift 3
        ;;
      --n)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_metric --n requires <count>"
        n_cnt="$2"
        shift 2
        ;;
      *)
        _e2e_harness_error "e2e_metric: unknown option '$1'"
        ;;
    esac
  done

  local m_args=(
    emit
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --event metric
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step "${E2E_CURRENT_STEP}"
    --seed "${E2E_SEED}"
    --name "${name}"
    --value "${val}"
    --unit "${unit}"
  )
  local cmp_ok="0"
  if [ -n "$op" ]; then
    m_args=("${m_args[@]}" --op "$op" --threshold "$thr")
    if [ "$observe" = "1" ]; then
      # frob-e2e emit computes the exact rational/decimal verdict when --observe is set.
      m_args=("${m_args[@]}" --observe)
    else
      if "${FROB_E2E_TOOL}" cmp num "$val" "$op" "$thr" >/dev/null 2>&1; then
        cmp_ok="1"
      fi
    fi
  elif [ "$observe" = "1" ]; then
    m_args=("${m_args[@]}" --observe)
  fi
  if [ -n "$gate" ]; then
    m_args=("${m_args[@]}" --gate "$gate")
  fi
  if [ -n "$iv_lo" ]; then
    m_args=("${m_args[@]}" --interval-lo "$iv_lo" --interval-hi "$iv_hi")
  fi
  if [ -n "$n_cnt" ]; then
    m_args=("${m_args[@]}" --n "$n_cnt")
  fi

  "${FROB_E2E_TOOL}" "${m_args[@]}" >/dev/null || _e2e_harness_error "failed to emit metric ${name}"

  # Emit threshold assertion ONLY when threshold is given and NOT --observe
  if [ -n "$op" ] && [ "$observe" != "1" ]; then
    local a_tags=()
    if [ -n "$gate" ]; then
      a_tags=(--gate "$gate")
    fi
    if [ "$cmp_ok" = "1" ]; then
      _e2e_emit_assert "metric:${name}" "num" "pass" --observed "$val" --expected "$thr" --op "$op" --unit "$unit" ${a_tags[@]+"${a_tags[@]}"}
    else
      _e2e_emit_assert "metric:${name}" "num" "fail" --observed "$val" --expected "$thr" --op "$op" --unit "$unit" \
        --diff-path "$name" --diff-observed "$val" --diff-expected "${op} ${thr}" ${a_tags[@]+"${a_tags[@]}"}
    fi
  fi
}

# ---------------------------------------------------------------------------
# 9. e2e_gate <GATE-...>  and  e2e_skip <reason>
# ---------------------------------------------------------------------------
e2e_gate() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_gate requires <GATE-...>"
  _e2e_record_gate "$1"
}

e2e_skip() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_skip requires <reason>"
  local reason="$1"
  _e2e_emit_assert "skip" "check" "skip" --observed "$reason" --reason "$reason"
}

# ---------------------------------------------------------------------------
# 10. e2e_plant <name>
# ---------------------------------------------------------------------------
e2e_plant() {
  [ "$#" -eq 1 ] || _e2e_harness_error "e2e_plant requires <name>"
  [ "${E2E_PLANT_FAILURE:-}" = "$1" ]
}

# ---------------------------------------------------------------------------
# 11. e2e_scene_ready <scene_id> [--tiers ...]
#     (Review note 6: shipped as logged no-op skip naming scene.accept; BD-228
#      fills in the accept body when scene.accept lands.)
# ---------------------------------------------------------------------------
e2e_scene_ready() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_scene_ready requires <scene_id>"
  local scene_id="$1"
  _e2e_emit_assert "scene_ready:${scene_id}" "check" "skip" \
    --observed "missing verb: scene.accept" \
    --reason "missing verb: scene.accept"
}

# ---------------------------------------------------------------------------
# 12. e2e_key <label> [--again]
# ---------------------------------------------------------------------------
e2e_key() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_key requires <label>"
  local label="$1" again="0"
  shift
  if [ "${1:-}" = "--again" ]; then
    again="1"
    shift
  fi
  [ "$#" -eq 0 ] || _e2e_harness_error "e2e_key: unexpected extra arguments: $*"

  # Validate label grammar: [a-z0-9][a-z0-9-]*
  case "$label" in
    [a-z0-9]*)
      case "$label" in
        *[!a-z0-9-]*)
          _e2e_harness_error "e2e_key: invalid label '${label}'"
          ;;
      esac
      ;;
    *)
      _e2e_harness_error "e2e_key: invalid label '${label}'"
      ;;
  esac

  if grep -Fxq "$label" "${E2E_META_DIR}/keys.txt" 2>/dev/null; then
    if [ "$again" != "1" ]; then
      _e2e_harness_error "e2e_key: label '${label}' already issued in this run (pass --again to replay)"
    fi
  else
    printf '%s\n' "$label" >> "${E2E_META_DIR}/keys.txt"
  fi
  printf 'e2e-%s-%s\n' "${E2E_SCRIPT}" "${label}"
}

# ---------------------------------------------------------------------------
# 13. e2e_fixture <name> [<file>] [--copy]
# ---------------------------------------------------------------------------
e2e_fixture() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_fixture requires <name>"
  local name="$1" file="" copy="0"
  shift
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --copy)
        copy="1"
        shift
        ;;
      --*)
        _e2e_harness_error "e2e_fixture: unknown option '$1'"
        ;;
      *)
        if [ -z "$file" ]; then
          file="$1"
          shift
        else
          _e2e_harness_error "e2e_fixture: unexpected argument '$1'"
        fi
        ;;
    esac
  done

  # Validate fixture name grammar (segments [a-z0-9][a-z0-9._-]*, <=255 bytes, no leading /, empty, . or ..)
  case "$name" in
    /*|*//*)
      _e2e_harness_error "e2e_fixture: invalid fixture name '${name}'"
      ;;
  esac
  local stripped="${name%/}"
  if [ -z "$stripped" ] || [ "${#name}" -gt 255 ]; then
    _e2e_harness_error "e2e_fixture: invalid fixture name '${name}'"
  fi
  if [ -n "$file" ]; then
    case "$file" in
      /*|*//*|..|../*|*/..|*/../*)
        _e2e_harness_error "e2e_fixture: invalid file path '${file}'"
        ;;
    esac
  fi
  local IFS_SAVED="$IFS" seg
  IFS="/"
  for seg in $stripped; do
    IFS="$IFS_SAVED"
    if [ -z "$seg" ] || [ "$seg" = "." ] || [ "$seg" = ".." ]; then
      _e2e_harness_error "e2e_fixture: invalid segment in fixture name '${name}'"
    fi
    case "$seg" in
      [a-z0-9]*)
        case "$seg" in
          *[!a-z0-9._-]*)
            _e2e_harness_error "e2e_fixture: invalid characters in fixture name '${name}'"
            ;;
        esac
        ;;
      *)
        _e2e_harness_error "e2e_fixture: segment must start with [a-z0-9] in '${name}'"
        ;;
    esac
    IFS="/"
  done
  IFS="$IFS_SAVED"

  local fixtures_root="${E2E_FIXTURES:-${E2E_REPO_ROOT}/fixtures}"
  local target="${fixtures_root}/${stripped}"
  if [ -n "$file" ]; then
    target="${target}/${file}"
  fi
  if [ ! -e "$target" ]; then
    _e2e_harness_error "e2e_fixture: fixture not found: ${target}"
  fi

  local sha assert_label="fixture:${name}"
  if [ -n "$file" ]; then
    assert_label="fixture:${name}/${file}"
  fi
  sha="$("${FROB_E2E_TOOL}" cmp sha256 "$target")" || _e2e_harness_error "e2e_fixture: sha256 failed on ${target}"
  _e2e_emit_assert "${assert_label}" "file" "pass" --observed "$sha" --expected "$target"

  if [ "$copy" = "1" ]; then
    local dest="${E2E_WORK_DIR}/fixtures/${stripped}"
    if [ -d "$target" ]; then
      mkdir -p "$dest"
      cp -R "${target}/." "${dest}/"
      printf '%s\n' "$dest"
    else
      local dest_file="${dest}"
      if [ -n "$file" ]; then
        dest_file="${dest}/${file}"
      fi
      mkdir -p "$(dirname "$dest_file")"
      cp "$target" "$dest_file"
      printf '%s\n' "$dest_file"
    fi
  else
    printf '%s\n' "$target"
  fi
}

# ---------------------------------------------------------------------------
# 14. e2e_tier <target> <filter> [--mutations]
# ---------------------------------------------------------------------------
e2e_tier() {
  [ "$#" -ge 2 ] || _e2e_harness_error "e2e_tier requires <target> <filter> [--mutations]"
  local target="$1" filter="$2" mutations="0"
  shift 2
  if [ "${1:-}" = "--mutations" ]; then
    mutations="1"
    shift
  fi
  [ "$#" -eq 0 ] || _e2e_harness_error "e2e_tier: unexpected argument '$1'"

  case "$target" in
    g0_laws|g1_known_answers|g2_parity|g3_metamorphic|g4_chaos|g5_determinism) ;;
    *)
      _e2e_harness_error "e2e_tier: invalid target '${target}' (must be g0_laws..g5_determinism)"
      ;;
  esac

  local cmd=(env "FROB_TEST_SEED=${E2E_SEED}" cargo test -p frob-conformance --test "${target}")
  if [ "$mutations" = "1" ]; then
    cmd=("${cmd[@]}" --features gauntlet-mutations)
  fi
  cmd=("${cmd[@]}" -- "${filter}")

  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    printf '[dry-run tier] %s\n' "${cmd[*]}"
    return 0
  fi

  local receipts_dir="${E2E_REPO_ROOT}/target/gauntlet/receipts"
  mkdir -p "$receipts_dir"

  local run_args=(
    run
    --log "${E2E_LOG_FILE}"
    --transcript "${E2E_TRANSCRIPT_FILE}"
    --artifacts-dir "${E2E_ARTIFACTS_DIR}"
    --scratch-root "${E2E_SCRATCH_DIR}"
    --run-id "${E2E_RUN_ID}"
    --script "${E2E_SCRIPT}"
    --bead "${E2E_BEAD}"
    --step "${E2E_CURRENT_STEP}"
    --seed "${E2E_SEED}"
    --name "tier:${target}"
    --expect-exit "0"
    --tier-receipts-dir "${receipts_dir}"
  )
  if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
    run_args=("${run_args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
  fi

  local kv_out rc=0
  kv_out="$("${FROB_E2E_TOOL}" "${run_args[@]}" -- "${cmd[@]}")" || rc=$?
  if [ "$rc" -eq 2 ]; then
    _e2e_harness_error "frob-e2e run failed for e2e_tier ${target}"
  fi
  _e2e_parse_kv_output "$kv_out"
}

# ---------------------------------------------------------------------------
# 15. e2e_check <name> [tags...] -- <args...>
# ---------------------------------------------------------------------------
e2e_check() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_check requires <name> [tags...] -- <args...>"
  local name="$1"
  shift
  local raw_tags=()
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --)
        shift
        break
        ;;
      --gate|--ka|--cx|--nc|--mr)
        [ "$#" -ge 2 ] || _e2e_harness_error "e2e_check $1 requires a value"
        raw_tags=(${raw_tags[@]+"${raw_tags[@]}"} "$1" "$2")
        shift 2
        ;;
      *)
        _e2e_harness_error "e2e_check: unexpected arg '$1' before --"
        ;;
    esac
  done
  _e2e_extract_tags ${raw_tags[@]+"${raw_tags[@]}"}

  local chk_file="${E2E_META_DIR}/check_${name}_$$.json"
  local rc=0
  "${FROB_E2E_TOOL}" check "$name" "$@" > "$chk_file" 2>&1 || rc=$?
  local obs exp dpath dobs dexp
  obs="$("${FROB_E2E_TOOL}" get --file "$chk_file" /observed 2>/dev/null || cat "$chk_file")"
  exp="$("${FROB_E2E_TOOL}" get --file "$chk_file" /expected 2>/dev/null || echo ok)"
  if [ "$rc" -eq 0 ]; then
    _e2e_emit_assert "check:${name}" "check" "pass" --observed "$obs" --expected "$exp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  else
    dpath="$("${FROB_E2E_TOOL}" get --file "$chk_file" /first_diff/path 2>/dev/null || echo "check:${name}")"
    dobs="$("${FROB_E2E_TOOL}" get --file "$chk_file" /first_diff/observed 2>/dev/null || echo "$obs")"
    dexp="$("${FROB_E2E_TOOL}" get --file "$chk_file" /first_diff/expected 2>/dev/null || echo "$exp")"
    _e2e_emit_assert "check:${name}" "check" "fail" --observed "$obs" --expected "$exp" \
      --diff-path "$dpath" --diff-observed "$dobs" --diff-expected "$dexp" ${_E2E_TAG_ARGS[@]+"${_E2E_TAG_ARGS[@]}"}
  fi
}

# ---------------------------------------------------------------------------
# 16. e2e_registries_scratch
# ---------------------------------------------------------------------------
e2e_registries_scratch() {
  local reg_dst="${E2E_SCRATCH_DIR}/registries"
  mkdir -p "$reg_dst"
  if [ -d "${E2E_REPO_ROOT}/registries" ]; then
    cp -R "${E2E_REPO_ROOT}/registries/." "${reg_dst}/"
  fi
  FROB_REGISTRIES_ROOT="$reg_dst"
  E2E_REGISTRIES_MODE="override"
  printf 'override\n' > "${E2E_META_DIR}/registries_mode"
  export FROB_REGISTRIES_ROOT
}

# ---------------------------------------------------------------------------
# 17. e2e_session <grant|open|derive|use> ...
#     (Includes review note 7: optional --principal, --ttl-s, --capture on
#      open, and the fourth form `derive` keeping current FROB_SESSION_TOKEN.)
# ---------------------------------------------------------------------------
_e2e_session_has_open_verb() {
  if [ -z "${FROB_BIN:-}" ] || [ ! -x "${FROB_BIN}" ]; then
    return 1
  fi
  "${FROB_BIN}" capabilities 2>/dev/null | grep -q 'session\.open'
}

e2e_session() {
  [ "$#" -ge 1 ] || _e2e_harness_error "e2e_session requires subcommand: grant|open|derive|use"
  local sub="$1"
  shift
  case "$sub" in
    grant)
      [ "$#" -ge 1 ] || _e2e_harness_error "e2e_session grant requires <principal> --scopes <list>"
      local principal="$1"
      shift
      # Refuse if daemon socket already exists/is live
      if [ -S "${FROB_STATE}/frobd.sock" ] || [ -e "${FROB_STATE}/frobd.sock" ]; then
        _e2e_harness_error "e2e_session grant refused: daemon socket present at ${FROB_STATE}/frobd.sock"
      fi
      local safety="${FROB_STATE}/safety.toml"
      if [ -f "$safety" ] && grep -Fxq "[principal.${principal}]" "$safety"; then
        _e2e_harness_error "e2e_session grant refused: principal '${principal}' already granted"
      fi

      local scopes="" namespace="" trace_bytes="" max_lifetime=""
      while [ "$#" -gt 0 ]; do
        case "$1" in
          --scopes) [ "$#" -ge 2 ] || _e2e_harness_error "--scopes requires a value"; scopes="$2"; shift 2 ;;
          --namespace) [ "$#" -ge 2 ] || _e2e_harness_error "--namespace requires a value"; namespace="$2"; shift 2 ;;
          --trace-bytes) [ "$#" -ge 2 ] || _e2e_harness_error "--trace-bytes requires a value"; trace_bytes="$2"; shift 2 ;;
          --max-lifetime-s) [ "$#" -ge 2 ] || _e2e_harness_error "--max-lifetime-s requires a value"; max_lifetime="$2"; shift 2 ;;
          *) _e2e_harness_error "e2e_session grant: unknown option '$1'" ;;
        esac
      done
      [ -n "$scopes" ] || _e2e_harness_error "e2e_session grant requires --scopes"

      # Format comma-separated scopes as TOML array of basic strings
      local scopes_toml="" item first=1 IFS_SAVED="$IFS"
      IFS=","
      for item in $scopes; do
        IFS="$IFS_SAVED"
        if [ "$first" -eq 1 ]; then
          scopes_toml="\"${item}\""
          first=0
        else
          scopes_toml="${scopes_toml}, \"${item}\""
        fi
        IFS=","
      done
      IFS="$IFS_SAVED"

      (
        umask 077
        {
          printf '\n[principal.%s]\n' "$principal"
          printf 'scopes = [%s]\n' "$scopes_toml"
          [ -z "$namespace" ] || printf 'namespace = "%s"\n' "$namespace"
          [ -z "$trace_bytes" ] || printf 'trace_bytes = %s\n' "$trace_bytes"
          [ -z "$max_lifetime" ] || printf 'max_lifetime_s = %s\n' "$max_lifetime"
        } >> "$safety"
      )
      chmod 0600 "$safety"
      ;;

    open)
      [ "$#" -ge 1 ] || _e2e_harness_error "e2e_session open requires <label>"
      local label="$1"
      shift
      if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
        printf '[dry-run session open] %s %s\n' "$label" "$*"
        return 0
      fi
      if ! _e2e_session_has_open_verb; then
        e2e_skip "missing verb: session.open"
        return 0
      fi
      local principal="" scopes="" budget="" namespace="" ttl_s="" capture=""
      while [ "$#" -gt 0 ]; do
        case "$1" in
          --principal) [ "$#" -ge 2 ] || _e2e_harness_error "--principal requires a value"; principal="$2"; shift 2 ;;
          --scopes) [ "$#" -ge 2 ] || _e2e_harness_error "--scopes requires a value"; scopes="$2"; shift 2 ;;
          --budget) [ "$#" -ge 2 ] || _e2e_harness_error "--budget requires a value"; budget="$2"; shift 2 ;;
          --namespace) [ "$#" -ge 2 ] || _e2e_harness_error "--namespace requires a value"; namespace="$2"; shift 2 ;;
          --ttl-s) [ "$#" -ge 2 ] || _e2e_harness_error "--ttl-s requires a value"; ttl_s="$2"; shift 2 ;;
          --capture) [ "$#" -ge 2 ] || _e2e_harness_error "--capture requires a value"; capture="$2"; shift 2 ;;
          *) _e2e_harness_error "e2e_session open: unknown option '$1'" ;;
        esac
      done

      local key
      key="$(e2e_key "$label")"
      local cmd=("${FROB_BIN}" --json session open)
      [ -z "$principal" ] || cmd=("${cmd[@]}" --principal "$principal")
      [ -z "$scopes" ] || cmd=("${cmd[@]}" --scopes "$scopes")
      [ -z "$budget" ] || cmd=("${cmd[@]}" --budget "$budget")
      [ -z "$namespace" ] || cmd=("${cmd[@]}" --namespace "$namespace")
      [ -z "$ttl_s" ] || cmd=("${cmd[@]}" --ttl-s "$ttl_s")
      [ -z "$capture" ] || cmd=("${cmd[@]}" --capture "$capture")
      cmd=("${cmd[@]}" --key "$key")

      local sess_dir="${FROB_STATE}/sessions"
      mkdir -p "$sess_dir"
      chmod 0700 "$sess_dir"
      local tok_file="${sess_dir}/${label}.token"

      local run_args=(
        run
        --log "${E2E_LOG_FILE}"
        --transcript "${E2E_TRANSCRIPT_FILE}"
        --artifacts-dir "${E2E_ARTIFACTS_DIR}"
        --scratch-root "${E2E_SCRATCH_DIR}"
        --run-id "${E2E_RUN_ID}"
        --script "${E2E_SCRIPT}"
        --bead "${E2E_BEAD}"
        --step "${E2E_CURRENT_STEP}"
        --seed "${E2E_SEED}"
        --name "session_open:${label}"
        --expect-exit "0"
        --unset-env FROB_SESSION_TOKEN
        --secret "/result/token"
        --secret-out "${tok_file}"
      )
      if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
        run_args=("${run_args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
      fi

      local kv_out rc=0
      kv_out="$("${FROB_E2E_TOOL}" "${run_args[@]}" -- "${cmd[@]}")" || rc=$?
      if [ "$rc" -eq 2 ]; then
        _e2e_harness_error "frob-e2e run failed for e2e_session open ${label}"
      fi
      _e2e_parse_kv_output "$kv_out"

      if [ "${E2E_LAST_SECRET_WRITTEN}" = "1" ] && [ -f "$tok_file" ]; then
        chmod 0600 "$tok_file"
        FROB_SESSION_TOKEN="$(cat "$tok_file")"
        export FROB_SESSION_TOKEN
      elif [ "${E2E_LAST_EXIT}" = "0" ]; then
        unset FROB_SESSION_TOKEN 2>/dev/null || true
        e2e_skip "missing /result/token"
      fi
      ;;

    derive)
      [ "$#" -ge 1 ] || _e2e_harness_error "e2e_session derive requires <label>"
      local label="$1"
      shift
      if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
        printf '[dry-run session derive] %s %s\n' "$label" "$*"
        return 0
      fi
      if ! _e2e_session_has_open_verb; then
        e2e_skip "missing verb: session.open"
        return 0
      fi
      local scopes="" budget="" ttl_s=""
      while [ "$#" -gt 0 ]; do
        case "$1" in
          --scopes) [ "$#" -ge 2 ] || _e2e_harness_error "--scopes requires a value"; scopes="$2"; shift 2 ;;
          --budget) [ "$#" -ge 2 ] || _e2e_harness_error "--budget requires a value"; budget="$2"; shift 2 ;;
          --ttl-s) [ "$#" -ge 2 ] || _e2e_harness_error "--ttl-s requires a value"; ttl_s="$2"; shift 2 ;;
          *) _e2e_harness_error "e2e_session derive: unknown option '$1'" ;;
        esac
      done

      local key
      key="$(e2e_key "$label")"
      local cmd=("${FROB_BIN}" --json session open --derive)
      [ -z "$scopes" ] || cmd=("${cmd[@]}" --scopes "$scopes")
      [ -z "$budget" ] || cmd=("${cmd[@]}" --budget "$budget")
      [ -z "$ttl_s" ] || cmd=("${cmd[@]}" --ttl-s "$ttl_s")
      cmd=("${cmd[@]}" --key "$key")

      local sess_dir="${FROB_STATE}/sessions"
      mkdir -p "$sess_dir"
      chmod 0700 "$sess_dir"
      local tok_file="${sess_dir}/${label}.token"

      # Keep current FROB_SESSION_TOKEN in env; on refusal, leave it unchanged.
      local run_args=(
        run
        --log "${E2E_LOG_FILE}"
        --transcript "${E2E_TRANSCRIPT_FILE}"
        --artifacts-dir "${E2E_ARTIFACTS_DIR}"
        --scratch-root "${E2E_SCRATCH_DIR}"
        --run-id "${E2E_RUN_ID}"
        --script "${E2E_SCRIPT}"
        --bead "${E2E_BEAD}"
        --step "${E2E_CURRENT_STEP}"
        --seed "${E2E_SEED}"
        --name "session_derive:${label}"
        --expect-exit "0"
        --secret "/result/token"
        --secret-out "${tok_file}"
      )
      if [ -n "${FROB_PRIVATE_ROOT:-}" ]; then
        run_args=("${run_args[@]}" --private-root "${FROB_PRIVATE_ROOT}")
      fi

      local kv_out rc=0
      kv_out="$("${FROB_E2E_TOOL}" "${run_args[@]}" -- "${cmd[@]}")" || rc=$?
      if [ "$rc" -eq 2 ]; then
        _e2e_harness_error "frob-e2e run failed for e2e_session derive ${label}"
      fi
      _e2e_parse_kv_output "$kv_out"

      if [ "${E2E_LAST_EXIT}" = "0" ] && [ "${E2E_LAST_SECRET_WRITTEN}" = "1" ] && [ -f "$tok_file" ]; then
        chmod 0600 "$tok_file"
        FROB_SESSION_TOKEN="$(cat "$tok_file")"
        export FROB_SESSION_TOKEN
      fi
      ;;

    use)
      [ "$#" -eq 1 ] || _e2e_harness_error "e2e_session use requires <label>"
      local label="$1"
      local tok_file="${FROB_STATE}/sessions/${label}.token"
      if [ ! -f "$tok_file" ]; then
        _e2e_harness_error "e2e_session use: unknown session label '${label}'"
      fi
      FROB_SESSION_TOKEN="$(cat "$tok_file")"
      export FROB_SESSION_TOKEN
      ;;

    *)
      _e2e_harness_error "e2e_session: unknown subcommand '${sub}' (expected grant|open|derive|use)"
      ;;
  esac
}
