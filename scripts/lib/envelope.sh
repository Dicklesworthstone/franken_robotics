#!/bin/sh
# Machine-envelope guard and workspace working-rule helpers (bead N34-15).
#
# POSIX sh compatible; sourced standalone or through scripts/e2e/lib.sh.
# Never deletes any file or directory (COMMON.md 9b).
# Signals ONLY processes started by this guard (never kills unrelated PIDs).
#
# Public functions:
#   frob_target_dir <name>
#   frob_scratch_dir <name>
#   frob_disk_preflight <path> <needed_gib>
#   frob_envelope_run [--class E0|E1|E2|E3] [--mem-gib N] [--cores N]
#                     [--wall-s N] [--approval ROW-ID] [--registry FILE]
#                     -- CMD...
#   frob_envelope_selftest [--scripts-dir DIR] [--e2e-dir DIR]

# Fail-closed E0 defaults: the ONLY limits hardcoded in the guard.
FROB_ENVELOPE_E0_MEM_GIB=16
FROB_ENVELOPE_E0_CORES=8
FROB_ENVELOPE_E0_WALL_S=3600
FROB_ENVELOPE_E0_DEVICES=1

# Resolve repository root once without mutating caller state.
_frob_resolve_repo_root() {
  if [ -n "${FROB_REPO_ROOT:-}" ] && [ -f "${FROB_REPO_ROOT}/scripts/lib/envelope.sh" ]; then
    printf '%s\n' "${FROB_REPO_ROOT}"
    return 0
  fi
  if [ -n "${E2E_REPO_ROOT:-}" ] && [ -f "${E2E_REPO_ROOT}/scripts/lib/envelope.sh" ]; then
    printf '%s\n' "${E2E_REPO_ROOT}"
    return 0
  fi
  _frr_git="$(git --no-optional-locks rev-parse --show-toplevel 2>/dev/null || true)"
  if [ -n "$_frr_git" ] && [ -f "${_frr_git}/scripts/lib/envelope.sh" ]; then
    printf '%s\n' "$_frr_git"
    return 0
  fi
  _frr_cand="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." 2>/dev/null && pwd || true)"
  if [ -n "$_frr_cand" ] && [ -f "${_frr_cand}/scripts/lib/envelope.sh" ]; then
    printf '%s\n' "$_frr_cand"
    return 0
  fi
  pwd
}

_frob_validate_rel_name() {
  _fvrn_name="$1"
  _fvrn_fn="$2"
  if [ -z "$_fvrn_name" ]; then
    printf '%s: name must not be empty\n' "$_fvrn_fn" >&2
    return 2
  fi
  case "$_fvrn_name" in
    /*|*".."*|*" "*|*"	"*|*"
"*)
      printf '%s: invalid relative name: %s\n' "$_fvrn_fn" "$_fvrn_name" >&2
      return 2
      ;;
  esac
  return 0
}

frob_target_dir() {
  if [ "$#" -ne 1 ]; then
    printf 'usage: frob_target_dir <name>\n' >&2
    return 2
  fi
  _frob_validate_rel_name "$1" "frob_target_dir" || return $?
  _ftd_base="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}"
  _ftd_base="${_ftd_base%/}"
  [ -n "$_ftd_base" ] || _ftd_base="/"
  if [ "$_ftd_base" = "/" ]; then
    printf '/%s\n' "$1"
  else
    printf '%s/%s\n' "$_ftd_base" "$1"
  fi
}

frob_scratch_dir() {
  if [ "$#" -ne 1 ]; then
    printf 'usage: frob_scratch_dir <name>\n' >&2
    return 2
  fi
  _frob_validate_rel_name "$1" "frob_scratch_dir" || return $?
  _fsd_base="${BIG_SCRATCH:-${TMPDIR:-/tmp}}"
  _fsd_base="${_fsd_base%/}"
  [ -n "$_fsd_base" ] || _fsd_base="/"
  if [ "$_fsd_base" = "/" ]; then
    printf '/%s\n' "$1"
  else
    printf '%s/%s\n' "$_fsd_base" "$1"
  fi
}

_frob_is_positive_number() {
  case "$1" in
    ''|.|*[!0-9.]*|*.*.*)
      return 1
      ;;
  esac
  awk -v v="$1" 'BEGIN { exit !(v + 0 > 0) }'
}

_frob_is_positive_int() {
  case "$1" in
    ''|*[!0-9]*)
      return 1
      ;;
  esac
  [ "$1" -gt 0 ]
}

_frob_json_escape() {
  printf '%s' "$1" | awk '
    BEGIN { ORS="" }
    {
      if (NR > 1) printf "\\n"
      gsub(/\\/, "\\\\")
      gsub(/"/, "\\\"")
      gsub(/\r/, "\\r")
      gsub(/\t/, "\\t")
      printf "%s", $0
    }
  '
}

_frob_now_utc_ms() {
  date -u +%Y-%m-%dT%H:%M:%S.000Z
}

_frob_log_emit_standalone() {
  _fles_line="$1"
  if [ -n "${FROB_ENVELOPE_LOG_FILE:-}" ]; then
    printf '%s\n' "$_fles_line" >> "${FROB_ENVELOPE_LOG_FILE}"
  else
    printf '%s\n' "$_fles_line" >&2
  fi
}

_frob_next_seq() {
  if [ -n "${FROB_ENVELOPE_LOG_FILE:-}" ] && [ -f "${FROB_ENVELOPE_LOG_FILE}" ]; then
    wc -l < "${FROB_ENVELOPE_LOG_FILE}" | tr -d ' '
  else
    _frob_env_seq="${_frob_env_seq:-0}"
    printf '%s' "$_frob_env_seq"
    _frob_env_seq=$((_frob_env_seq + 1))
  fi
}

_frob_use_e2e_tool() {
  [ -z "${FROB_ENVELOPE_LOG_FILE:-}" ] &&
    [ "${_E2E_INITIALIZED:-0}" = "1" ] &&
    [ -n "${E2E_LOG_FILE:-}" ] &&
    [ -n "${FROB_E2E_TOOL:-}" ] &&
    [ -x "${FROB_E2E_TOOL}" ]
}

_frob_emit_refusal_assert() {
  _fera_name="$1"
  _fera_obs="$2"
  _fera_exp="$3"
  _fera_unit="$4"
  _fera_msg="$5"
  _fera_ts="$(_frob_now_utc_ms)"

  printf 'frob_envelope_run: refused (%s): %s\n' "$_fera_name" "$_fera_msg" >&2

  if _frob_use_e2e_tool; then
    _fera_step="${E2E_CURRENT_STEP:-envelope}"
    if [ -n "$_fera_unit" ]; then
      "${FROB_E2E_TOOL}" emit \
        --log "${E2E_LOG_FILE}" \
        --transcript "${E2E_TRANSCRIPT_FILE}" \
        --event assert \
        --run-id "${E2E_RUN_ID}" \
        --script "${E2E_SCRIPT}" \
        --bead "${E2E_BEAD}" \
        --step "${_fera_step}" \
        --seed "${E2E_SEED}" \
        --name "${_fera_name}" \
        --kind "check" \
        --status "fail" \
        --observed "${_fera_obs}" \
        --expected "${_fera_exp}" \
        --unit "${_fera_unit}" \
        --diff-path "${_fera_name}" \
        --diff-observed "${_fera_obs}" \
        --diff-expected "${_fera_exp}" \
        --cmd-seq "${E2E_LAST_CMD_SEQ:-0}" >/dev/null 2>&1 || true
    else
      "${FROB_E2E_TOOL}" emit \
        --log "${E2E_LOG_FILE}" \
        --transcript "${E2E_TRANSCRIPT_FILE}" \
        --event assert \
        --run-id "${E2E_RUN_ID}" \
        --script "${E2E_SCRIPT}" \
        --bead "${E2E_BEAD}" \
        --step "${_fera_step}" \
        --seed "${E2E_SEED}" \
        --name "${_fera_name}" \
        --kind "check" \
        --status "fail" \
        --observed "${_fera_obs}" \
        --expected "${_fera_exp}" \
        --diff-path "${_fera_name}" \
        --diff-observed "${_fera_obs}" \
        --diff-expected "${_fera_exp}" \
        --cmd-seq "${E2E_LAST_CMD_SEQ:-0}" >/dev/null 2>&1 || true
    fi
    return 0
  fi

  _fera_seq="$(_frob_next_seq)"
  _fera_rid="${E2E_RUN_ID:-env-$$}"
  _fera_scr="${E2E_SCRIPT:-envelope}"
  _fera_bead="${E2E_BEAD:-N34-15}"
  _fera_seed="${E2E_SEED:-0}"
  _fera_n="$(_frob_json_escape "$_fera_name")"
  _fera_o="$(_frob_json_escape "$_fera_obs")"
  _fera_e="$(_frob_json_escape "$_fera_exp")"
  _fera_u="$(_frob_json_escape "$_fera_unit")"
  _fera_m="$(_frob_json_escape "$_fera_msg")"
  _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fera_ts}\",\"seq\":${_fera_seq},\"run_id\":\"${_fera_rid}\",\"script\":\"${_fera_scr}\",\"bead\":\"${_fera_bead}\",\"step\":\"envelope\",\"event\":\"assert\",\"seed\":\"${_fera_seed}\",\"name\":\"${_fera_n}\",\"kind\":\"check\",\"status\":\"fail\",\"observed\":\"${_fera_o}\",\"expected\":\"${_fera_e}\",\"unit\":\"${_fera_u}\",\"reason\":\"${_fera_m}\",\"first_diff\":{\"path\":\"${_fera_n}\",\"observed\":\"${_fera_o}\",\"expected\":\"${_fera_e}\"},\"repro\":{\"cmd_seq\":0,\"artifacts\":[]}}"
}

frob_disk_preflight() {
  if [ "$#" -ne 2 ]; then
    printf 'usage: frob_disk_preflight <path> <needed_gib>\n' >&2
    return 2
  fi
  _fdp_path="$1"
  _fdp_need="$2"
  if ! _frob_is_positive_number "$_fdp_need"; then
    printf 'frob_disk_preflight: needed_gib must be a positive number, got: %s\n' "$_fdp_need" >&2
    return 2
  fi

  _fdp_probe="$_fdp_path"
  while [ ! -e "$_fdp_probe" ]; do
    _fdp_parent="$(dirname -- "$_fdp_probe")"
    if [ "$_fdp_parent" = "$_fdp_probe" ]; then
      break
    fi
    _fdp_probe="$_fdp_parent"
  done
  if [ ! -e "$_fdp_probe" ]; then
    printf 'frob_disk_preflight: cannot resolve filesystem for path: %s\n' "$_fdp_path" >&2
    return 4
  fi

  _fdp_avail_kib="$(df -Pk "$_fdp_probe" 2>/dev/null | awk 'NR==2 { print $4 }')"
  case "$_fdp_avail_kib" in
    ''|*[!0-9]*)
      printf 'frob_disk_preflight: failed to query available space for %s\n' "$_fdp_probe" >&2
      return 4
      ;;
  esac

  _fdp_avail_gib="$(awk -v k="$_fdp_avail_kib" 'BEGIN { printf "%.3f", k / 1048576.0 }')"
  if ! awk -v k="$_fdp_avail_kib" -v need="$_fdp_need" 'BEGIN { exit !(k >= need * 1048576.0) }'; then
    _frob_emit_refusal_assert \
      "disk_preflight" \
      "${_fdp_avail_gib}" \
      ">= ${_fdp_need}" \
      "GiB" \
      "insufficient disk space on ${_fdp_probe}: available ${_fdp_avail_gib} GiB < required ${_fdp_need} GiB"
    return 4
  fi
  return 0
}

_frob_class_rank() {
  case "$1" in
    E0) printf '0' ;;
    E1) printf '1' ;;
    E2) printf '2' ;;
    E3) printf '3' ;;
    *)  printf -- '-1' ;;
  esac
}

_frob_host_cores() {
  if command -v nproc >/dev/null 2>&1; then
    nproc 2>/dev/null || printf '8'
  elif command -v sysctl >/dev/null 2>&1; then
    sysctl -n hw.ncpu 2>/dev/null || printf '8'
  else
    printf '8'
  fi
}

_frob_host_mem_gib() {
  if command -v sysctl >/dev/null 2>&1 && sysctl -n hw.memsize >/dev/null 2>&1; then
    _fhm_b="$(sysctl -n hw.memsize 2>/dev/null || printf '17179869184')"
    awk -v b="$_fhm_b" 'BEGIN { printf "%d", int(b / 1073741824) }'
  elif [ -r /proc/meminfo ]; then
    awk '/^MemTotal:/ { printf "%d", int($2 / 1048576); exit }' /proc/meminfo
  else
    printf '16'
  fi
}

_frob_hash_text() {
  if command -v b3sum >/dev/null 2>&1; then
    printf 'blake3:%s' "$(printf '%s' "$1" | b3sum --no-names)"
  elif command -v sha256sum >/dev/null 2>&1; then
    printf 'sha256:%s' "$(printf '%s' "$1" | sha256sum | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    printf 'sha256:%s' "$(printf '%s' "$1" | shasum -a 256 | awk '{print $1}')"
  else
    printf 'none'
  fi
}

# Resolve effective first executable word, unwrapping leading `env` options/assignments.
_frob_unwrap_cmd0() {
  while [ "$#" -gt 0 ]; do
    case "$1" in
      env)
        shift
        while [ "$#" -gt 0 ]; do
          case "$1" in
            -i|-0|--ignore-environment|--null)
              shift
              ;;
            -u|--unset)
              shift
              [ "$#" -gt 0 ] && shift
              ;;
            -u*)
              shift
              ;;
            -C|--chdir)
              shift
              [ "$#" -gt 0 ] && shift
              ;;
            --)
              shift
              break
              ;;
            *=*)
              shift
              ;;
            -*)
              shift
              ;;
            *)
              break
              ;;
          esac
        done
        ;;
      *=*)
        shift
        ;;
      *)
        printf '%s' "$1"
        return 0
        ;;
    esac
  done
  printf ''
}

# Line-oriented reader for registries/compute_envelope.toml (no external TOML parser).
# Prints KEY=VALUE lines for the requested [[request]] row and its class table.
_frob_read_registry_row() {
  _frr_file="$1"
  _frr_row_id="$2"
  awk -v target_id="$_frr_row_id" '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }
    function strip_comment(line,    i, ch, in_dq, in_sq, esc, out) {
      in_dq = 0; in_sq = 0; esc = 0; out = ""
      for (i = 1; i <= length(line); i++) {
        ch = substr(line, i, 1)
        if (esc) {
          out = out ch
          esc = 0
          continue
        }
        if (in_dq) {
          if (ch == "\\") esc = 1
          else if (ch == "\"") in_dq = 0
          out = out ch
        } else if (in_sq) {
          if (ch == "\x27") in_sq = 0
          out = out ch
        } else {
          if (ch == "#") break
          if (ch == "\"") in_dq = 1
          else if (ch == "\x27") in_sq = 1
          out = out ch
        }
      }
      return trim(out)
    }
    function unquote(v) {
      v = trim(v)
      if (substr(v, 1, 1) == "\"" && substr(v, length(v), 1) == "\"" && length(v) >= 2) {
        return substr(v, 2, length(v) - 2)
      }
      if (substr(v, 1, 1) == "\x27" && substr(v, length(v), 1) == "\x27" && length(v) >= 2) {
        return substr(v, 2, length(v) - 2)
      }
      return v
    }
    function finalize_row() {
      if (in_req && req_id == target_id) {
        found = 1
        found_class = req_class
        found_status = req_status
        found_approved_by = req_approved_by
        found_date = req_date
        found_expires = req_expires
        found_limits = req_limits
        found_packages = req_packages
        found_admits = req_admits
        found_ceiling = req_ceiling
        found_kill_rule = req_kill_rule
        found_raw = req_raw
      }
    }
    BEGIN {
      section = "top"
      in_req = 0
      found = 0
      in_force = ""
      schema = ""
    }
    {
      raw_line = $0
      line = strip_comment($0)
      if (line == "") {
        next
      }
      if (line == "[[request]]") {
        finalize_row()
        section = "request"
        in_req = 1
        req_id = ""; req_class = ""; req_status = ""
        req_approved_by = ""; req_date = ""; req_expires = ""
        req_limits = ""; req_packages = ""; req_admits = ""
        req_ceiling = ""; req_kill_rule = ""
        req_raw = raw_line "\n"
        next
      }
      if (line ~ /^\[class\.[A-Za-z0-9_]+\]$/) {
        finalize_row()
        in_req = 0
        section = substr(line, 8, length(line) - 8)
        next
      }
      if (line ~ /^\[/) {
        finalize_row()
        in_req = 0
        section = "other"
        next
      }
      if (in_req) {
        req_raw = req_raw raw_line "\n"
      }
      eq = index(line, "=")
      if (eq == 0) next
      key = trim(substr(line, 1, eq - 1))
      val = trim(substr(line, eq + 1))
      if (section == "top") {
        if (key == "in_force") in_force = unquote(val)
        else if (key == "schema") schema = unquote(val)
      } else if (in_req) {
        if (key == "id") req_id = unquote(val)
        else if (key == "class") req_class = unquote(val)
        else if (key == "status") req_status = unquote(val)
        else if (key == "approved_by") req_approved_by = unquote(val)
        else if (key == "date") req_date = unquote(val)
        else if (key == "expires") req_expires = unquote(val)
        else if (key == "limits") req_limits = val
        else if (key == "packages") req_packages = val
        else if (key == "admits") req_admits = val
        else if (key == "ceiling") req_ceiling = unquote(val)
        else if (key == "kill_rule") req_kill_rule = unquote(val)
      } else if (section != "other") {
        if (key == "state") class_state[section] = unquote(val)
        else if (key == "limits") class_limits[section] = val
      }
    }
    END {
      finalize_row()
      printf "SCHEMA=%s\n", schema
      printf "IN_FORCE=%s\n", in_force
      printf "FOUND=%d\n", found
      if (found) {
        printf "ROW_CLASS=%s\n", found_class
        printf "ROW_STATUS=%s\n", found_status
        printf "ROW_APPROVED_BY=%s\n", found_approved_by
        printf "ROW_DATE=%s\n", found_date
        printf "ROW_EXPIRES=%s\n", found_expires
        printf "ROW_LIMITS=%s\n", found_limits
        printf "ROW_PACKAGES=%s\n", found_packages
        printf "ROW_ADMITS=%s\n", found_admits
        printf "ROW_CEILING=%s\n", found_ceiling
        printf "ROW_KILL_RULE=%s\n", found_kill_rule
        printf "CLASS_STATE=%s\n", class_state[found_class]
        printf "CLASS_LIMITS=%s\n", class_limits[found_class]
        printf "ROW_RAW_BEGIN\n%sROW_RAW_END\n", found_raw
      }
    }
  ' "$_frr_file"
}

# Extract a key's value from a TOML inline table string `{ k = v, ... }`.
_frob_inline_table_get() {
  _fitg_tbl="$1"
  _fitg_key="$2"
  printf '%s\n' "$_fitg_tbl" | awk -v k="$_fitg_key" '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }
    {
      s = trim($0)
      sub(/^\{/, "", s)
      sub(/\}$/, "", s)
      n = split(s, parts, ",")
      for (i = 1; i <= n; i++) {
        p = trim(parts[i])
        eq = index(p, "=")
        if (eq == 0) continue
        pk = trim(substr(p, 1, eq - 1))
        pv = trim(substr(p, eq + 1))
        if (pk == k) {
          if (substr(pv, 1, 1) == "\"" && substr(pv, length(pv), 1) == "\"" && length(pv) >= 2) {
            pv = substr(pv, 2, length(pv) - 2)
          }
          print pv
          exit 0
        }
      }
    }
  '
}

frob_envelope_run() {
  _fer_class="E0"
  _fer_req_mem=""
  _fer_req_cores=""
  _fer_req_wall=""
  _fer_approval=""
  _fer_registry=""

  while [ "$#" -gt 0 ]; do
    case "$1" in
      --class)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_run: --class requires a value\n' >&2; return 2; }
        _fer_class="$2"
        shift 2
        ;;
      --mem-gib)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_run: --mem-gib requires a value\n' >&2; return 2; }
        _fer_req_mem="$2"
        shift 2
        ;;
      --cores)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_run: --cores requires a value\n' >&2; return 2; }
        _fer_req_cores="$2"
        shift 2
        ;;
      --wall-s)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_run: --wall-s requires a value\n' >&2; return 2; }
        _fer_req_wall="$2"
        shift 2
        ;;
      --approval)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_run: --approval requires a value\n' >&2; return 2; }
        _fer_approval="$2"
        shift 2
        ;;
      --registry)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_run: --registry requires a value\n' >&2; return 2; }
        _fer_registry="$2"
        shift 2
        ;;
      --)
        shift
        break
        ;;
      *)
        printf 'frob_envelope_run: unknown option before --: %s\n' "$1" >&2
        return 2
        ;;
    esac
  done

  if [ "$#" -lt 1 ]; then
    printf 'frob_envelope_run: missing command after --\n' >&2
    return 2
  fi

  _fer_req_rank="$(_frob_class_rank "$_fer_class")"
  if [ "$_fer_req_rank" -lt 0 ]; then
    printf 'frob_envelope_run: invalid class: %s (expected E0|E1|E2|E3)\n' "$_fer_class" >&2
    return 2
  fi

  if [ -n "$_fer_req_mem" ] && ! _frob_is_positive_number "$_fer_req_mem"; then
    printf 'frob_envelope_run: --mem-gib must be a positive number, got: %s\n' "$_fer_req_mem" >&2
    return 2
  fi
  if [ -n "$_fer_req_cores" ] && ! _frob_is_positive_int "$_fer_req_cores"; then
    printf 'frob_envelope_run: --cores must be a positive integer, got: %s\n' "$_fer_req_cores" >&2
    return 2
  fi
  if [ -n "$_fer_req_wall" ] && ! _frob_is_positive_number "$_fer_req_wall"; then
    printf 'frob_envelope_run: --wall-s must be a positive number, got: %s\n' "$_fer_req_wall" >&2
    return 2
  fi

  # Rule 1: refuse any absolute-path /cargo before starting anything.
  _fer_cmd0="$(_frob_unwrap_cmd0 "$@")"
  case "$_fer_cmd0" in
    /*/cargo)
      _frob_emit_refusal_assert \
        "envelope:absolute_cargo" \
        "$_fer_cmd0" \
        "cargo (via PATH offload shim)" \
        "" \
        "cargo must not be invoked by absolute path: $_fer_cmd0"
      return 4
      ;;
  esac

  _fer_repo_root="$(_frob_resolve_repo_root)"
  if [ -z "$_fer_registry" ]; then
    _fer_registry="${FROB_ENVELOPE_REGISTRY:-${_fer_repo_root}/registries/compute_envelope.toml}"
  fi

  # Resolve effective limits.
  _fer_eff_mem="$FROB_ENVELOPE_E0_MEM_GIB"
  _fer_eff_cores="$FROB_ENVELOPE_E0_CORES"
  _fer_eff_wall="$FROB_ENVELOPE_E0_WALL_S"
  _fer_eff_devices="$FROB_ENVELOPE_E0_DEVICES"
  _fer_approval_hash=""
  _fer_row_class="E0"

  # Determine whether approval is required (class > E0 or explicit override > E0).
  _fer_needs_approval=0
  if [ "$_fer_req_rank" -gt 0 ]; then
    _fer_needs_approval=1
  else
    if [ -n "$_fer_req_mem" ] && awk -v a="$_fer_req_mem" -v b="$FROB_ENVELOPE_E0_MEM_GIB" 'BEGIN { exit !(a + 0 > b + 0) }'; then
      _fer_needs_approval=1
    fi
    if [ -n "$_fer_req_cores" ] && [ "$_fer_req_cores" -gt "$FROB_ENVELOPE_E0_CORES" ]; then
      _fer_needs_approval=1
    fi
    if [ -n "$_fer_req_wall" ] && awk -v a="$_fer_req_wall" -v b="$FROB_ENVELOPE_E0_WALL_S" 'BEGIN { exit !(a + 0 > b + 0) }'; then
      _fer_needs_approval=1
    fi
  fi

  if [ "$_fer_needs_approval" -eq 1 ]; then
    if [ -z "$_fer_approval" ]; then
      _frob_emit_refusal_assert \
        "envelope:approval_required" \
        "${_fer_class} (no --approval)" \
        "approved [[request]] row in compute_envelope.toml" \
        "" \
        "class ${_fer_class} (or limits above E0) requires --approval ROW-ID"
      return 4
    fi
    if [ ! -r "$_fer_registry" ]; then
      _frob_emit_refusal_assert \
        "envelope:registry_missing" \
        "${_fer_registry}" \
        "readable compute_envelope.toml" \
        "" \
        "registry absent or unreadable (${_fer_registry}); failing closed to E0"
      return 4
    fi

    _fer_parsed="$(_frob_read_registry_row "$_fer_registry" "$_fer_approval")"
    _fer_in_force=""
    _fer_found="0"
    _fer_row_class=""
    _fer_row_status=""
    _fer_row_approved_by=""
    _fer_row_date=""
    _fer_row_expires=""
    _fer_row_limits=""
    _fer_row_ceiling=""
    _fer_row_kill_rule=""
    _fer_class_state=""
    _fer_class_limits=""
    _fer_row_raw=""
    _fer_in_raw=0

    while IFS= read -r _fer_pl; do
      if [ "$_fer_in_raw" -eq 1 ]; then
        if [ "$_fer_pl" = "ROW_RAW_END" ]; then
          _fer_in_raw=0
        else
          _fer_row_raw="${_fer_row_raw}${_fer_pl}
"
        fi
        continue
      fi
      case "$_fer_pl" in
        ROW_RAW_BEGIN) _fer_in_raw=1 ;;
        IN_FORCE=*) _fer_in_force="${_fer_pl#IN_FORCE=}" ;;
        FOUND=*) _fer_found="${_fer_pl#FOUND=}" ;;
        ROW_CLASS=*) _fer_row_class="${_fer_pl#ROW_CLASS=}" ;;
        ROW_STATUS=*) _fer_row_status="${_fer_pl#ROW_STATUS=}" ;;
        ROW_APPROVED_BY=*) _fer_row_approved_by="${_fer_pl#ROW_APPROVED_BY=}" ;;
        ROW_DATE=*) _fer_row_date="${_fer_pl#ROW_DATE=}" ;;
        ROW_EXPIRES=*) _fer_row_expires="${_fer_pl#ROW_EXPIRES=}" ;;
        ROW_LIMITS=*) _fer_row_limits="${_fer_pl#ROW_LIMITS=}" ;;
        ROW_CEILING=*) _fer_row_ceiling="${_fer_pl#ROW_CEILING=}" ;;
        ROW_KILL_RULE=*) _fer_row_kill_rule="${_fer_pl#ROW_KILL_RULE=}" ;;
        CLASS_STATE=*) _fer_class_state="${_fer_pl#CLASS_STATE=}" ;;
        CLASS_LIMITS=*) _fer_class_limits="${_fer_pl#CLASS_LIMITS=}" ;;
      esac
    done <<EOF
$_fer_parsed
EOF

    if [ -n "$_fer_in_force" ] && [ "$_fer_in_force" != "E0" ]; then
      _frob_emit_refusal_assert \
        "envelope:invalid_in_force" \
        "$_fer_in_force" \
        "E0" \
        "" \
        "registry in_force must be E0, got: $_fer_in_force"
      return 4
    fi

    if [ "$_fer_found" != "1" ]; then
      _frob_emit_refusal_assert \
        "envelope:approval_not_found" \
        "$_fer_approval" \
        "existing [[request]] id" \
        "" \
        "approval row '${_fer_approval}' not found in ${_fer_registry}"
      return 4
    fi

    _fer_approval_hash="$(_frob_hash_text "$_fer_row_raw")"

    case "$_fer_row_status" in
      approved|amended) ;;
      *)
        _frob_emit_refusal_assert \
          "envelope:approval_status" \
          "${_fer_row_status:-empty}" \
          "approved|amended" \
          "" \
          "approval row '${_fer_approval}' has status '${_fer_row_status:-empty}' (must be approved or amended)"
        return 4
        ;;
    esac

    if [ -z "$_fer_row_approved_by" ] || [ -z "$_fer_row_date" ]; then
      _frob_emit_refusal_assert \
        "envelope:approval_metadata" \
        "approved_by=${_fer_row_approved_by:-empty},date=${_fer_row_date:-empty}" \
        "non-empty approved_by and date" \
        "" \
        "approval row '${_fer_approval}' missing approved_by or date"
      return 4
    fi

    # Check expiration date when expires carries an ISO YYYY-MM-DD date or literal 'expired'.
    _fer_today="${FROB_ENVELOPE_TODAY:-$(date -u +%Y-%m-%d)}"
    case "$_fer_row_expires" in
      expired)
        _frob_emit_refusal_assert \
          "envelope:approval_expired" \
          "$_fer_row_expires" \
          ">= ${_fer_today}" \
          "" \
          "approval row '${_fer_approval}' has expired (${_fer_row_expires})"
        return 4
        ;;
      [0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]*)
        _fer_exp_date="$(printf '%s' "$_fer_row_expires" | cut -c1-10)"
        if awk -v e="$_fer_exp_date" -v t="$_fer_today" 'BEGIN { exit !(e < t) }'; then
          _frob_emit_refusal_assert \
            "envelope:approval_expired" \
            "$_fer_exp_date" \
            ">= ${_fer_today}" \
            "" \
            "approval row '${_fer_approval}' expired on ${_fer_exp_date} (today ${_fer_today})"
          return 4
        fi
        ;;
    esac

    _fer_row_rank="$(_frob_class_rank "$_fer_row_class")"
    if [ "$_fer_row_rank" -lt 0 ] || [ "$_fer_row_rank" -lt "$_fer_req_rank" ]; then
      _frob_emit_refusal_assert \
        "envelope:approval_class_mismatch" \
        "${_fer_row_class:-empty}" \
        ">= ${_fer_class}" \
        "" \
        "approval row '${_fer_approval}' class '${_fer_row_class:-empty}' is below requested class '${_fer_class}'"
      return 4
    fi

    case "$_fer_class_state" in
      approved|amended) ;;
      *)
        _frob_emit_refusal_assert \
          "envelope:class_not_approved" \
          "${_fer_row_class}:${_fer_class_state:-missing}" \
          "approved|amended" \
          "" \
          "class ${_fer_row_class} state is '${_fer_class_state:-missing}' (must be approved or amended)"
        return 4
        ;;
    esac

    if [ "$_fer_row_class" = "E3" ] && { [ -z "$_fer_row_ceiling" ] || [ -z "$_fer_row_kill_rule" ]; }; then
      _frob_emit_refusal_assert \
        "envelope:e3_ceiling_or_kill_rule_missing" \
        "ceiling=${_fer_row_ceiling:-empty},kill_rule=${_fer_row_kill_rule:-empty}" \
        "non-empty ceiling and kill_rule" \
        "" \
        "E3 approval row '${_fer_approval}' requires both ceiling and kill_rule"
      return 4
    fi

    # Extract per-run limits from the row's limits table, falling back to class limits.
    _fer_l_mem="$(_frob_inline_table_get "$_fer_row_limits" "mem_gib")"
    [ -n "$_fer_l_mem" ] || _fer_l_mem="$(_frob_inline_table_get "$_fer_row_limits" "mem")"
    [ -n "$_fer_l_mem" ] || _fer_l_mem="$(_frob_inline_table_get "$_fer_class_limits" "mem_gib")"
    [ -n "$_fer_l_mem" ] || _fer_l_mem="$(_frob_inline_table_get "$_fer_class_limits" "mem")"

    _fer_l_cores="$(_frob_inline_table_get "$_fer_row_limits" "cores")"
    [ -n "$_fer_l_cores" ] || _fer_l_cores="$(_frob_inline_table_get "$_fer_class_limits" "cores")"

    _fer_l_wall="$(_frob_inline_table_get "$_fer_row_limits" "wall_s_per_run")"
    [ -n "$_fer_l_wall" ] || _fer_l_wall="$(_frob_inline_table_get "$_fer_row_limits" "wall_s")"
    [ -n "$_fer_l_wall" ] || _fer_l_wall="$(_frob_inline_table_get "$_fer_class_limits" "wall_s_per_run")"
    [ -n "$_fer_l_wall" ] || _fer_l_wall="$(_frob_inline_table_get "$_fer_class_limits" "wall_s")"

    _fer_l_dev="$(_frob_inline_table_get "$_fer_row_limits" "devices")"
    [ -n "$_fer_l_dev" ] || _fer_l_dev="$(_frob_inline_table_get "$_fer_class_limits" "devices")"

    if [ "$_fer_l_mem" = "device budget" ] || [ "$_fer_l_mem" = "all" ]; then
      _fer_eff_mem="$(_frob_host_mem_gib)"
    elif [ -n "$_fer_l_mem" ] && _frob_is_positive_number "$_fer_l_mem"; then
      _fer_eff_mem="$_fer_l_mem"
    fi

    if [ "$_fer_l_cores" = "all" ]; then
      _fer_eff_cores="$(_frob_host_cores)"
    elif [ -n "$_fer_l_cores" ] && _frob_is_positive_int "$_fer_l_cores"; then
      _fer_eff_cores="$_fer_l_cores"
    fi

    if [ -n "$_fer_l_wall" ] && _frob_is_positive_number "$_fer_l_wall"; then
      _fer_eff_wall="$_fer_l_wall"
    fi

    if [ -n "$_fer_l_dev" ] && _frob_is_positive_int "$_fer_l_dev"; then
      _fer_eff_devices="$_fer_l_dev"
    fi
  fi

  # Apply caller's explicit tighter bounds, or refuse if caller exceeds resolved row limits.
  if [ -n "$_fer_req_mem" ]; then
    if awk -v a="$_fer_req_mem" -v b="$_fer_eff_mem" 'BEGIN { exit !(a + 0 > b + 0) }'; then
      _frob_emit_refusal_assert \
        "envelope:mem_exceeds_approval" \
        "$_fer_req_mem" \
        "<= ${_fer_eff_mem}" \
        "GiB" \
        "requested --mem-gib ${_fer_req_mem} exceeds resolved limit ${_fer_eff_mem} GiB"
      return 4
    fi
    _fer_eff_mem="$_fer_req_mem"
  fi
  if [ -n "$_fer_req_cores" ]; then
    if [ "$_fer_req_cores" -gt "$_fer_eff_cores" ]; then
      _frob_emit_refusal_assert \
        "envelope:cores_exceeds_approval" \
        "$_fer_req_cores" \
        "<= ${_fer_eff_cores}" \
        "cores" \
        "requested --cores ${_fer_req_cores} exceeds resolved limit ${_fer_eff_cores} cores"
      return 4
    fi
    _fer_eff_cores="$_fer_req_cores"
  fi
  if [ -n "$_fer_req_wall" ]; then
    if awk -v a="$_fer_req_wall" -v b="$_fer_eff_wall" 'BEGIN { exit !(a + 0 > b + 0) }'; then
      _frob_emit_refusal_assert \
        "envelope:wall_exceeds_approval" \
        "$_fer_req_wall" \
        "<= ${_fer_eff_wall}" \
        "s" \
        "requested --wall-s ${_fer_req_wall} exceeds resolved limit ${_fer_eff_wall} s"
      return 4
    fi
    _fer_eff_wall="$_fer_req_wall"
  fi

  # Emit structured start event.
  _fer_cmdline="$*"
  _fer_ts_start="$(_frob_now_utc_ms)"
  _fer_e2e_mode=0
  if _frob_use_e2e_tool; then
    _fer_e2e_mode=1
    _fer_step="${E2E_CURRENT_STEP:-envelope}"
    _fer_title="envelope class=${_fer_class} mem_gib=${_fer_eff_mem} cores=${_fer_eff_cores} wall_s=${_fer_eff_wall}"
    if [ -n "$_fer_approval" ]; then
      _fer_title="${_fer_title} approval=${_fer_approval} hash=${_fer_approval_hash}"
    fi
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event start \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${_fer_step}" \
      --seed "${E2E_SEED}" \
      --title "${_fer_title}" >/dev/null 2>&1 || true
    if [ -n "$_fer_approval" ]; then
      "${FROB_E2E_TOOL}" emit \
        --log "${E2E_LOG_FILE}" \
        --transcript "${E2E_TRANSCRIPT_FILE}" \
        --event assert \
        --run-id "${E2E_RUN_ID}" \
        --script "${E2E_SCRIPT}" \
        --bead "${E2E_BEAD}" \
        --step "${_fer_step}" \
        --seed "${E2E_SEED}" \
        --name "envelope:approval:${_fer_approval}" \
        --kind "check" \
        --status "pass" \
        --observed "${_fer_approval_hash}" \
        --expected "approved:${_fer_class}" >/dev/null 2>&1 || true
    fi
  else
    _fer_seq_start="$(_frob_next_seq)"
    _fer_rid="${E2E_RUN_ID:-env-$$}"
    _fer_scr="${E2E_SCRIPT:-envelope}"
    _fer_bead="${E2E_BEAD:-N34-15}"
    _fer_seed="${E2E_SEED:-0}"
    _fer_cmd_esc="$(_frob_json_escape "$_fer_cmdline")"
    _fer_app_esc="$(_frob_json_escape "$_fer_approval")"
    _fer_hash_esc="$(_frob_json_escape "$_fer_approval_hash")"
    _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fer_ts_start}\",\"seq\":${_fer_seq_start},\"run_id\":\"${_fer_rid}\",\"script\":\"${_fer_scr}\",\"bead\":\"${_fer_bead}\",\"step\":\"envelope\",\"event\":\"start\",\"seed\":\"${_fer_seed}\",\"title\":\"envelope ${_fer_class}\",\"class\":\"${_fer_class}\",\"limits\":{\"mem_gib\":${_fer_eff_mem},\"cores\":${_fer_eff_cores},\"wall_s\":${_fer_eff_wall},\"devices\":${_fer_eff_devices}},\"approval_id\":\"${_fer_app_esc}\",\"approval_hash\":\"${_fer_hash_esc}\",\"cmdline\":\"${_fer_cmd_esc}\"}"
  fi

  # Prepare a scratch result file (never deleted; kept under scratch root).
  _fer_meta_root="${E2E_META_DIR:-$(frob_scratch_dir "frob-envelope-meta")}"
  mkdir -p "$_fer_meta_root"
  _fer_res_file="${_fer_meta_root}/run.$$.$(date -u +%s).kv"

  # Run guarded command inside a new process group under python3 supervisor.
  # Start supervisor directly in background so the calling shell's INT/TERM/HUP
  # trap fires immediately and forwards SIGTERM ONLY to the supervisor we started,
  # while leaving caller's EXIT trap untouched.
  _fer_rc=0
  if command -v python3 >/dev/null 2>&1; then
    env \
      "CARGO_BUILD_JOBS=$_fer_eff_cores" \
      "RUST_TEST_THREADS=$_fer_eff_cores" \
      "RAYON_NUM_THREADS=$_fer_eff_cores" \
      "OMP_NUM_THREADS=$_fer_eff_cores" \
      "OPENBLAS_NUM_THREADS=$_fer_eff_cores" \
      "MKL_NUM_THREADS=$_fer_eff_cores" \
      "VECLIB_MAXIMUM_THREADS=$_fer_eff_cores" \
      "NUMEXPR_NUM_THREADS=$_fer_eff_cores" \
      "FROB_ENVELOPE_CLASS=$_fer_class" \
      "FROB_ENVELOPE_CORES=$_fer_eff_cores" \
      "FROB_ENVELOPE_MEM_GIB=$_fer_eff_mem" \
      "FROB_ENVELOPE_WALL_S=$_fer_eff_wall" \
      "FROB_ENVELOPE_DEVICES=$_fer_eff_devices" \
      "FROB_ENVELOPE_APPROVAL=$_fer_approval" \
      "FROB_ENVELOPE_APPROVAL_HASH=$_fer_approval_hash" \
      python3 - "$_fer_eff_wall" "$_fer_eff_mem" "$_fer_res_file" "$$" "$@" <<'PY' &
import os
import resource
import signal
import subprocess
import sys
import time

wall_limit_s = float(sys.argv[1])
mem_limit_gib = float(sys.argv[2])
res_file = sys.argv[3]
caller_pid = int(sys.argv[4])
cmd = sys.argv[5:]
mem_limit_bytes = int(mem_limit_gib * 1024 * 1024 * 1024)
orig_ppid = os.getppid()
my_pgid = os.getpgrp()

def pid_alive(pid):
    if pid <= 1:
        return True
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True

def pgrp_rss_and_alive(pgid):
    if pgid <= 1 or pgid == my_pgid:
        return 0, False
    try:
        out = subprocess.check_output(
            ["ps", "-eo", "pid=,pgid=,rss="],
            stderr=subprocess.DEVNULL,
            text=True,
        )
    except Exception:
        return 0, False
    total_kib = 0
    alive = False
    for line in out.splitlines():
        parts = line.split()
        if len(parts) < 3:
            continue
        try:
            _p = int(parts[0])
            pg = int(parts[1])
            rss = int(parts[2])
        except ValueError:
            continue
        if pg == pgid:
            alive = True
            total_kib += rss
    return total_kib * 1024, alive

def kill_owned_pgrp(pgid, proc_ref=None, grace_s=0.35):
    if pgid <= 1 or pgid == my_pgid:
        return
    try:
        os.killpg(pgid, signal.SIGTERM)
    except (ProcessLookupError, PermissionError):
        if proc_ref is not None:
            proc_ref.poll()
        return
    deadline = time.monotonic() + grace_s
    while time.monotonic() < deadline:
        if proc_ref is not None:
            proc_ref.poll()
        try:
            os.killpg(pgid, 0)
        except (ProcessLookupError, PermissionError):
            return
        time.sleep(0.02)
    try:
        os.killpg(pgid, signal.SIGKILL)
    except (ProcessLookupError, PermissionError):
        pass
    if proc_ref is not None:
        try:
            proc_ref.wait(timeout=0.2)
        except Exception:
            pass

def child_preexec():
    os.setsid()
    # Apply RLIMIT_AS only on Linux where the kernel honors address-space caps
    # without breaking Mach VM; macOS uses the resident-memory sampler below.
    if sys.platform.startswith("linux") and os.environ.get("FROB_ENVELOPE_NO_RLIMIT_AS") != "1":
        try:
            _soft, hard = resource.getrlimit(resource.RLIMIT_AS)
            cap = mem_limit_bytes
            if hard != resource.RLIM_INFINITY and cap > hard:
                cap = hard
            resource.setrlimit(resource.RLIMIT_AS, (cap, hard))
        except (ValueError, OSError):
            pass

t0 = time.monotonic()
proc = None
child_pgid = 0
breach_kind = ""
breach_obs = ""
breach_thr = ""
breach_unit = ""
peak_rss_bytes = 0
signal_received = 0
exit_code = 0

def on_signal(signum, _frame):
    global signal_received
    signal_received = signum
    if child_pgid > 1:
        kill_owned_pgrp(child_pgid, proc, grace_s=0.35)

signal.signal(signal.SIGTERM, on_signal)
signal.signal(signal.SIGINT, on_signal)
signal.signal(signal.SIGHUP, on_signal)

try:
    proc = subprocess.Popen(cmd, preexec_fn=child_preexec)
    child_pgid = proc.pid
except FileNotFoundError:
    exit_code = 127
    breach_kind = ""
except PermissionError:
    exit_code = 126
    breach_kind = ""
except Exception:
    exit_code = 126

if proc is not None:
    last_rss_sample = 0.0
    while True:
        if signal_received:
            kill_owned_pgrp(child_pgid, proc, grace_s=0.35)
            try:
                proc.wait(timeout=0.5)
            except Exception:
                pass
            exit_code = 128 + signal_received
            break

        # Parent-death detection (covers SIGTERM/SIGKILL of calling shell).
        if os.getppid() != orig_ppid or not pid_alive(caller_pid):
            kill_owned_pgrp(child_pgid, proc, grace_s=0.35)
            try:
                proc.wait(timeout=0.5)
            except Exception:
                pass
            exit_code = 143
            break

        rc = proc.poll()
        now = time.monotonic()
        elapsed = now - t0

        if rc is None and (now - last_rss_sample) >= 0.12:
            last_rss_sample = now
            rss_b, _alive = pgrp_rss_and_alive(child_pgid)
            if rss_b > peak_rss_bytes:
                peak_rss_bytes = rss_b
            if rss_b > mem_limit_bytes:
                breach_kind = "memory"
                breach_obs = f"{rss_b / (1024.0 ** 3):.3f}"
                breach_thr = f"{mem_limit_gib:g}"
                breach_unit = "GiB"
                kill_owned_pgrp(child_pgid, proc, grace_s=0.35)
                try:
                    proc.wait(timeout=0.5)
                except Exception:
                    pass
                exit_code = 137
                break

        if rc is None and elapsed >= wall_limit_s:
            breach_kind = "wall_time"
            breach_obs = f"{elapsed:.3f}"
            breach_thr = f"{wall_limit_s:g}"
            breach_unit = "s"
            kill_owned_pgrp(child_pgid, proc, grace_s=0.35)
            try:
                proc.wait(timeout=0.5)
            except Exception:
                pass
            exit_code = 124
            break

        if rc is not None:
            exit_code = rc if rc >= 0 else (128 - rc)
            # Reap any lingering background children still belonging to child_pgid.
            rss_b, alive = pgrp_rss_and_alive(child_pgid)
            if rss_b > peak_rss_bytes:
                peak_rss_bytes = rss_b
            if alive:
                kill_owned_pgrp(child_pgid, proc, grace_s=0.35)
            # Detect RLIMIT_AS-induced allocation failure on Linux if rss sampler saw high usage.
            if sys.platform.startswith("linux") and exit_code != 0 and not breach_kind:
                ru = resource.getrusage(resource.RUSAGE_CHILDREN)
                ru_bytes = ru.ru_maxrss * 1024
                if ru_bytes > peak_rss_bytes:
                    peak_rss_bytes = ru_bytes
                if ru_bytes > mem_limit_bytes:
                    breach_kind = "memory"
                    breach_obs = f"{ru_bytes / (1024.0 ** 3):.3f}"
                    breach_thr = f"{mem_limit_gib:g}"
                    breach_unit = "GiB"
            break

        time.sleep(0.05)

    # Fold child rusage maxrss for short-lived commands.
    ru = resource.getrusage(resource.RUSAGE_CHILDREN)
    ru_bytes = ru.ru_maxrss if sys.platform == "darwin" else ru.ru_maxrss * 1024
    if ru_bytes > peak_rss_bytes:
        peak_rss_bytes = ru_bytes

wall_s = time.monotonic() - t0
wall_ms = int(round(wall_s * 1000.0))
peak_rss_mib = peak_rss_bytes / (1024.0 * 1024.0)

with open(res_file, "w", encoding="utf-8") as fh:
    fh.write(f"EXIT={exit_code}\n")
    fh.write(f"WALL_S={wall_s:.3f}\n")
    fh.write(f"WALL_MS={wall_ms}\n")
    fh.write(f"PEAK_RSS_MIB={peak_rss_mib:.3f}\n")
    fh.write(f"BREACH_KIND={breach_kind}\n")
    fh.write(f"BREACH_OBS={breach_obs}\n")
    fh.write(f"BREACH_THR={breach_thr}\n")
    fh.write(f"BREACH_UNIT={breach_unit}\n")

sys.exit(exit_code)
PY
    _frob_sup_pid=$!
    _frob_sig_caught=0
    _frob_on_parent_sig() {
      _frob_sig_caught="$1"
      kill -TERM "$_frob_sup_pid" 2>/dev/null || true
      wait "$_frob_sup_pid" 2>/dev/null || true
      trap - INT TERM HUP
      exit $((128 + _frob_sig_caught))
    }
    trap '_frob_on_parent_sig 2' INT
    trap '_frob_on_parent_sig 15' TERM
    trap '_frob_on_parent_sig 1' HUP
    wait "$_frob_sup_pid" || _fer_rc=$?
    trap - INT TERM HUP
  else
    "$@" || _fer_rc=$?
    printf 'EXIT=%s\nWALL_S=0.000\nWALL_MS=0\nPEAK_RSS_MIB=0.000\nBREACH_KIND=\nBREACH_OBS=\nBREACH_THR=\nBREACH_UNIT=\n' "$_fer_rc" > "$_fer_res_file"
  fi

  _fer_exit="$_fer_rc"
  _fer_wall_s="0.000"
  _fer_wall_ms="0"
  _fer_peak_mib="0.000"
  _fer_breach_kind=""
  _fer_breach_obs=""
  _fer_breach_thr=""
  _fer_breach_unit=""
  if [ -r "$_fer_res_file" ]; then
    while IFS= read -r _fer_rl; do
      case "$_fer_rl" in
        EXIT=*) _fer_exit="${_fer_rl#EXIT=}" ;;
        WALL_S=*) _fer_wall_s="${_fer_rl#WALL_S=}" ;;
        WALL_MS=*) _fer_wall_ms="${_fer_rl#WALL_MS=}" ;;
        PEAK_RSS_MIB=*) _fer_peak_mib="${_fer_rl#PEAK_RSS_MIB=}" ;;
        BREACH_KIND=*) _fer_breach_kind="${_fer_rl#BREACH_KIND=}" ;;
        BREACH_OBS=*) _fer_breach_obs="${_fer_rl#BREACH_OBS=}" ;;
        BREACH_THR=*) _fer_breach_thr="${_fer_rl#BREACH_THR=}" ;;
        BREACH_UNIT=*) _fer_breach_unit="${_fer_rl#BREACH_UNIT=}" ;;
      esac
    done < "$_fer_res_file"
  fi

  _fer_ts_end="$(_frob_now_utc_ms)"
  _fer_mem_mib_thr="$(awk -v g="$_fer_eff_mem" 'BEGIN { printf "%.3f", g * 1024.0 }')"

  if [ "$_fer_e2e_mode" -eq 1 ]; then
    _fer_step="${E2E_CURRENT_STEP:-envelope}"
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event metric \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${_fer_step}" \
      --seed "${E2E_SEED}" \
      --name "envelope_peak_rss_mib" \
      --value "${_fer_peak_mib}" \
      --unit "MiB" \
      --op "<=" \
      --threshold "${_fer_mem_mib_thr}" \
      --observe >/dev/null 2>&1 || true
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event metric \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${_fer_step}" \
      --seed "${E2E_SEED}" \
      --name "envelope_wall_s" \
      --value "${_fer_wall_s}" \
      --unit "s" \
      --op "<=" \
      --threshold "${_fer_eff_wall}" \
      --observe >/dev/null 2>&1 || true
    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event metric \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${_fer_step}" \
      --seed "${E2E_SEED}" \
      --name "envelope_exit_code" \
      --value "${_fer_exit}" \
      --unit "code" \
      --observe >/dev/null 2>&1 || true

    if [ -n "$_fer_breach_kind" ]; then
      printf 'frob_envelope_run: limit breach (%s): observed=%s %s threshold=<= %s %s\n' \
        "$_fer_breach_kind" "$_fer_breach_obs" "$_fer_breach_unit" "$_fer_breach_thr" "$_fer_breach_unit" >&2
      "${FROB_E2E_TOOL}" emit \
        --log "${E2E_LOG_FILE}" \
        --transcript "${E2E_TRANSCRIPT_FILE}" \
        --event assert \
        --run-id "${E2E_RUN_ID}" \
        --script "${E2E_SCRIPT}" \
        --bead "${E2E_BEAD}" \
        --step "${_fer_step}" \
        --seed "${E2E_SEED}" \
        --name "envelope:${_fer_breach_kind}" \
        --kind "num" \
        --status "fail" \
        --observed "${_fer_breach_obs}" \
        --expected "${_fer_breach_thr}" \
        --op "<=" \
        --unit "${_fer_breach_unit}" \
        --diff-path "envelope:${_fer_breach_kind}" \
        --diff-observed "${_fer_breach_obs}" \
        --diff-expected "<= ${_fer_breach_thr}" \
        --cmd-seq "${E2E_LAST_CMD_SEQ:-0}" >/dev/null 2>&1 || true
    fi

    "${FROB_E2E_TOOL}" emit \
      --log "${E2E_LOG_FILE}" \
      --transcript "${E2E_TRANSCRIPT_FILE}" \
      --event end \
      --run-id "${E2E_RUN_ID}" \
      --script "${E2E_SCRIPT}" \
      --bead "${E2E_BEAD}" \
      --step "${_fer_step}" \
      --seed "${E2E_SEED}" >/dev/null 2>&1 || true
  else
    _fer_rid="${E2E_RUN_ID:-env-$$}"
    _fer_scr="${E2E_SCRIPT:-envelope}"
    _fer_bead="${E2E_BEAD:-N34-15}"
    _fer_seed="${E2E_SEED:-0}"
    _fer_af=0

    _fer_s1="$(_frob_next_seq)"
    _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fer_ts_end}\",\"seq\":${_fer_s1},\"run_id\":\"${_fer_rid}\",\"script\":\"${_fer_scr}\",\"bead\":\"${_fer_bead}\",\"step\":\"envelope\",\"event\":\"metric\",\"seed\":\"${_fer_seed}\",\"name\":\"peak_rss_mib\",\"value\":${_fer_peak_mib},\"unit\":\"MiB\",\"threshold\":{\"op\":\"<=\",\"value\":${_fer_mem_mib_thr}}}"

    _fer_s2="$(_frob_next_seq)"
    _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fer_ts_end}\",\"seq\":${_fer_s2},\"run_id\":\"${_fer_rid}\",\"script\":\"${_fer_scr}\",\"bead\":\"${_fer_bead}\",\"step\":\"envelope\",\"event\":\"metric\",\"seed\":\"${_fer_seed}\",\"name\":\"wall_s\",\"value\":${_fer_wall_s},\"unit\":\"s\",\"threshold\":{\"op\":\"<=\",\"value\":${_fer_eff_wall}}}"

    _fer_s3="$(_frob_next_seq)"
    _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fer_ts_end}\",\"seq\":${_fer_s3},\"run_id\":\"${_fer_rid}\",\"script\":\"${_fer_scr}\",\"bead\":\"${_fer_bead}\",\"step\":\"envelope\",\"event\":\"metric\",\"seed\":\"${_fer_seed}\",\"name\":\"exit_code\",\"value\":${_fer_exit},\"unit\":\"code\"}"

    if [ -n "$_fer_breach_kind" ]; then
      _fer_af=1
      _fer_s4="$(_frob_next_seq)"
      _fer_bk="$(_frob_json_escape "$_fer_breach_kind")"
      _fer_bu="$(_frob_json_escape "$_fer_breach_unit")"
      _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fer_ts_end}\",\"seq\":${_fer_s4},\"run_id\":\"${_fer_rid}\",\"script\":\"${_fer_scr}\",\"bead\":\"${_fer_bead}\",\"step\":\"envelope\",\"event\":\"assert\",\"seed\":\"${_fer_seed}\",\"name\":\"envelope:${_fer_bk}\",\"kind\":\"num\",\"status\":\"fail\",\"observed\":${_fer_breach_obs},\"expected\":${_fer_breach_thr},\"op\":\"<=\",\"unit\":\"${_fer_bu}\",\"first_diff\":{\"path\":\"envelope:${_fer_bk}\",\"observed\":\"${_fer_breach_obs} ${_fer_bu}\",\"expected\":\"<= ${_fer_breach_thr} ${_fer_bu}\"},\"repro\":{\"cmd_seq\":0,\"artifacts\":[]}}"
    fi

    _fer_s5="$(_frob_next_seq)"
    _frob_log_emit_standalone "{\"schema\":\"frob.e2elog/1\",\"ts\":\"${_fer_ts_end}\",\"seq\":${_fer_s5},\"run_id\":\"${_fer_rid}\",\"script\":\"${_fer_scr}\",\"bead\":\"${_fer_bead}\",\"step\":\"envelope\",\"event\":\"end\",\"seed\":\"${_fer_seed}\",\"wall_ms\":${_fer_wall_ms},\"asserts_passed\":0,\"asserts_failed\":${_fer_af}}"
  fi

  return "$_fer_exit"
}

# Static self-test scanner covering N34-15 ACs 7 & 8 and Rule 6:
#  1. scripts/e2e/*.sh must run heavy steps (`cargo` or `frob` run verbs) through
#     the guard (`frob_envelope_run`, `e2e_cmd`, `e2e_tier`, `e2e_expect_caught`,
#     `e2e_expect_refusal`).
#  2. Any script under scripts/ whose header carries label `pilot` or `train`,
#     and any `frob_envelope_run --class E1|E2|E3` call, must invoke only `frob`,
#     `frob-launch`, `cargo`, or a function of the shared helper (`e2e_*`/`frob_*`).
#  3. No deleting commands (`rm`, `rmdir`, `git clean`, `shutil.rmtree`, etc.)
#     in scripts/.
frob_envelope_selftest() {
  _fes_repo_root="$(_frob_resolve_repo_root)"
  _fes_scripts_dir="${_fes_repo_root}/scripts"
  _fes_e2e_dir=""
  _fes_skip_delete=0

  while [ "$#" -gt 0 ]; do
    case "$1" in
      --scripts-dir)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_selftest: --scripts-dir requires a value\n' >&2; return 2; }
        _fes_scripts_dir="$2"
        shift 2
        ;;
      --e2e-dir)
        [ "$#" -ge 2 ] || { printf 'frob_envelope_selftest: --e2e-dir requires a value\n' >&2; return 2; }
        _fes_e2e_dir="$2"
        shift 2
        ;;
      --no-delete-check)
        _fes_skip_delete=1
        shift
        ;;
      *)
        printf 'frob_envelope_selftest: unknown option: %s\n' "$1" >&2
        return 2
        ;;
    esac
  done

  if [ -z "$_fes_e2e_dir" ]; then
    if [ -d "${_fes_scripts_dir}/e2e" ]; then
      _fes_e2e_dir="${_fes_scripts_dir}/e2e"
    fi
  fi

  python3 - "$_fes_scripts_dir" "$_fes_e2e_dir" "$_fes_skip_delete" <<'PY'
import os
import re
import shlex
import sys

scripts_dir = sys.argv[1]
e2e_dir = sys.argv[2]
skip_delete = sys.argv[3] == "1"

SHELL_SCAFFOLDING = {
    # POSIX / bash control & builtins
    "set", "unset", "export", "local", "readonly", "typeset", "declare",
    "source", ".", "if", "then", "else", "elif", "fi", "for", "in", "do",
    "done", "while", "until", "case", "esac", "function", "return", "exit",
    "shift", "trap", "true", "false", "break", "continue", "eval", "exec",
    "command", "builtin", "type", "hash", "umask", "ulimit", "wait", "read",
    "printf", "echo", "test", "[", "[[", "cd", "pwd", "pushd", "popd",
    # Lightweight POSIX plumbing utilities (not compute/training workloads)
    "mkdir", "cp", "mv", "chmod", "cat", "date", "uname", "dirname",
    "basename", "awk", "sed", "grep", "rg", "jq", "cut", "tr", "sort",
    "uniq", "head", "tail", "wc", "cmp", "diff", "tee", "b3sum", "shasum",
    "sha256sum", "mktemp", "touch", "sleep", "ls", "find", "stat", "git",
    "env", "timeout", "gtimeout", "xargs", "id", "hostname", "df",
}

FROB_READONLY_VERBS = {
    "doctor", "capabilities", "version", "--version", "-V", "help", "--help",
    "-h", "status", "list", "show", "inspect", "session", "schema",
}

GUARD_WRAPPERS = {
    "frob_envelope_run",
    "e2e_cmd",
    "e2e_tier",
    "e2e_expect_caught",
    "e2e_expect_refusal",
}

ALLOWED_ABOVE_E0_OR_PILOT = {"frob", "frob-launch", "cargo"}

def is_shared_helper(word):
    return word.startswith("e2e_") or word.startswith("frob_")

def is_frob_bin(word):
    w = word.strip("\"'")
    return w in ("frob", "$FROB_BIN", "${FROB_BIN}") or w.endswith("/frob")

def is_frob_launch_bin(word):
    w = word.strip("\"'")
    return w in ("frob-launch", "$FROB_LAUNCH_BIN", "${FROB_LAUNCH_BIN}") or w.endswith("/frob-launch")

def is_cargo_bin(word):
    w = word.strip("\"'")
    return w == "cargo" or w.endswith("/cargo")

def header_has_pilot_or_train(lines):
    for line in lines:
        s = line.strip()
        if not s:
            continue
        if not s.startswith("#"):
            break
        # Match `# e2e: ... labels=pilot,foo` or `# labels: pilot` or `# label: train`
        m = re.search(r"(?:\blabels?\s*[:=]\s*)([A-Za-z0-9_, -]+)", s)
        if m:
            raw = m.group(1)
            # Stop before next key=value token if `# e2e:` multi-field header
            first_field = raw.split()[0] if "=" in s else raw
            toks = [t.strip().lower() for t in re.split(r"[,;\s]+", first_field) if t.strip()]
            if "pilot" in toks or "train" in toks:
                return True
    return False

def collect_local_funcs(lines):
    funcs = set()
    for line in lines:
        m = re.match(r"^\s*(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*\)", line)
        if m:
            funcs.add(m.group(1))
    return funcs

def join_logical_lines(lines):
    """Yield (start_lineno, text) skipping pure comments and heredoc bodies."""
    i = 0
    n = len(lines)
    heredoc_end = None
    while i < n:
        raw = lines[i]
        lineno = i + 1
        i += 1
        if heredoc_end is not None:
            if raw.strip() == heredoc_end:
                heredoc_end = None
            continue
        stripped = raw.strip()
        if not stripped or stripped.startswith("#"):
            continue
        buf = raw.rstrip("\r\n")
        while buf.endswith("\\") and i < n:
            buf = buf[:-1] + " " + lines[i].strip()
            i += 1
        # Detect heredoc opener on this logical line so body lines are skipped.
        hm = re.search(r"<<-?\s*['\"]?([A-Za-z_][A-Za-z0-9_]*)['\"]?", buf)
        if hm:
            heredoc_end = hm.group(1)
        yield lineno, buf

def split_commands(stmt):
    """Split a shell logical line on unquoted ;, &&, ||, |."""
    cmds = []
    cur = []
    in_sq = False
    in_dq = False
    esc = False
    j = 0
    m = len(stmt)
    while j < m:
        ch = stmt[j]
        if esc:
            cur.append(ch)
            esc = False
            j += 1
            continue
        if in_sq:
            if ch == "'":
                in_sq = False
            cur.append(ch)
            j += 1
            continue
        if in_dq:
            if ch == "\\":
                esc = True
            elif ch == '"':
                in_dq = False
            cur.append(ch)
            j += 1
            continue
        if ch == "\\":
            esc = True
            cur.append(ch)
            j += 1
            continue
        if ch == "'":
            in_sq = True
            cur.append(ch)
            j += 1
            continue
        if ch == '"':
            in_dq = True
            cur.append(ch)
            j += 1
            continue
        if ch == "#":
            break
        if ch == ";":
            cmds.append("".join(cur).strip())
            cur = []
            j += 1
            continue
        if ch in ("&", "|") and j + 1 < m and stmt[j + 1] == ch:
            cmds.append("".join(cur).strip())
            cur = []
            j += 2
            continue
        if ch == "|":
            cmds.append("".join(cur).strip())
            cur = []
            j += 1
            continue
        cur.append(ch)
        j += 1
    tail = "".join(cur).strip()
    if tail:
        cmds.append(tail)
    return cmds

def tokenize(cmd):
    try:
        return shlex.split(cmd, posix=True)
    except ValueError:
        return cmd.split()

def strip_leading_env_and_assigns(tokens):
    idx = 0
    n = len(tokens)
    while idx < n:
        t = tokens[idx]
        if t in ("if", "then", "else", "elif", "do", "!", "time"):
            idx += 1
            continue
        if re.match(r"^[A-Za-z_][A-Za-z0-9_]*=", t):
            idx += 1
            continue
        if t == "env":
            idx += 1
            while idx < n:
                et = tokens[idx]
                if et == "--":
                    idx += 1
                    break
                if et in ("-u", "--unset", "-C", "--chdir"):
                    idx += 2
                    continue
                if et.startswith("-") or re.match(r"^[A-Za-z_][A-Za-z0-9_]*=", et):
                    idx += 1
                    continue
                break
            continue
        if t in ("timeout", "gtimeout"):
            idx += 1
            while idx < n and tokens[idx].startswith("-"):
                idx += 1
            if idx < n:
                idx += 1  # duration
            continue
        break
    return tokens[idx:]

def frob_subverb(tokens):
    """Return the first non-flag token after frob, or ''."""
    i = 1
    while i < len(tokens):
        t = tokens[i]
        if t.startswith("-"):
            i += 1
            continue
        return t
    return ""

findings = []

def scan_file(path, is_e2e_script):
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            lines = fh.readlines()
    except OSError:
        return
    pilot_or_train = header_has_pilot_or_train(lines)
    local_funcs = collect_local_funcs(lines)
    base = os.path.basename(path)
    is_harness_infra = base in ("lib.sh", "run-all.sh", "selftest.sh", "envelope.sh")

    for lineno, logical in join_logical_lines(lines):
        # Skip case branch patterns like `foo) ...` by stripping leading `pattern)`
        for raw_cmd in split_commands(logical):
            cmd = raw_cmd.strip()
            if not cmd or cmd.startswith("#"):
                continue
            # Handle inline `case` pattern prefix: `foo|bar) cmd`
            cm = re.match(r"^[^()\s\"']+\)\s*(.*)$", cmd)
            if cm and not cmd.startswith("("):
                cmd = cm.group(1).strip()
                if not cmd:
                    continue
            # Strip leading subshell paren
            cmd = cmd.lstrip("(").rstrip(")").strip()
            if not cmd:
                continue
            tokens = tokenize(cmd)
            if not tokens:
                continue

            # Rule 6: deleting commands in scripts/
            if not skip_delete and base != "envelope.sh":
                ut = strip_leading_env_and_assigns(tokens)
                if ut:
                    w0 = ut[0]
                    if w0 in ("rm", "rmdir", "unlink") or (w0 == "git" and len(ut) > 1 and ut[1] == "clean"):
                        findings.append((path, lineno, "delete-command-forbidden", f"deleting command '{w0}'"))

            ut = strip_leading_env_and_assigns(tokens)
            if not ut:
                continue
            w0 = ut[0]

            # Ignore pure `command -v cargo` / `command -v frob` existence probes.
            if w0 == "command":
                continue

            # Check B1: frob_envelope_run --class E1|E2|E3 with non-allowed inner command.
            if w0 == "frob_envelope_run":
                cls = "E0"
                inner = []
                k = 1
                while k < len(ut):
                    if ut[k] == "--class" and k + 1 < len(ut):
                        cls = ut[k + 1]
                        k += 2
                        continue
                    if ut[k] == "--":
                        inner = strip_leading_env_and_assigns(ut[k + 1:])
                        break
                    k += 1
                if cls in ("E1", "E2", "E3") and inner:
                    iw0 = inner[0]
                    if not (
                        is_frob_bin(iw0)
                        or is_frob_launch_bin(iw0)
                        or is_cargo_bin(iw0)
                        or is_shared_helper(iw0)
                    ):
                        findings.append((
                            path,
                            lineno,
                            "device-lane-outside-launcher",
                            f"above-E0 frob_envelope_run (--class {cls}) starts '{iw0}' outside frob-launch",
                        ))
                continue

            # Check A (AC 7): scripts/e2e/*.sh calling cargo or frob run-verb outside guard.
            if is_e2e_script and not is_harness_infra:
                # Guarded if any guard wrapper precedes cargo/frob on the command line.
                has_guard = any(t in GUARD_WRAPPERS for t in ut) or (
                    len(ut) >= 3 and ut[0] in ("sh", "bash") and ut[1].endswith("envelope.sh") and ut[2] == "run"
                )
                is_meta_call = w0 in (
                    "echo", "printf", "grep", "rg", "awk", "sed", "cat", "local",
                    "export", "record_assert", "begin_step", "e2e_step", "e2e_assert_eq",
                    "e2e_assert_file", "e2e_assert_json", "e2e_metric", "e2e_skip",
                    "[", "[[", "test",
                )
                if not has_guard and not is_meta_call:
                    for idx_t, tok_t in enumerate(ut):
                        if tok_t == "--":
                            continue
                        if is_cargo_bin(tok_t):
                            findings.append((
                                path,
                                lineno,
                                "unguarded-heavy-command",
                                f"e2e script calls '{tok_t}' outside frob_envelope_run / e2e_cmd",
                            ))
                            break
                        if is_frob_bin(tok_t):
                            sv = frob_subverb(ut[idx_t:])
                            if sv and sv not in FROB_READONLY_VERBS:
                                findings.append((
                                    path,
                                    lineno,
                                    "unguarded-heavy-command",
                                    f"e2e script calls 'frob {sv}' outside frob_envelope_run / e2e_cmd",
                                ))
                            break

            # Check B2 (AC 8): pilot/train script calling a command outside allowed launchers.
            if pilot_or_train:
                if (
                    w0 in SHELL_SCAFFOLDING
                    or w0 in local_funcs
                    or is_frob_bin(w0)
                    or is_frob_launch_bin(w0)
                    or is_cargo_bin(w0)
                    or is_shared_helper(w0)
                    or w0.endswith(")")
                    or w0 in ("{", "}", ";;")
                ):
                    continue
                findings.append((
                    path,
                    lineno,
                    "device-lane-outside-launcher",
                    f"pilot/train script starts '{w0}' outside frob-launch",
                ))

seen = set()

if os.path.isdir(scripts_dir):
    for dp, dns, fns in os.walk(scripts_dir):
        # Skip test fixture/unit-test directories unless scripts_dir itself IS that dir.
        if os.path.abspath(dp) != os.path.abspath(scripts_dir):
            dns[:] = sorted(d for d in dns if d not in ("tests", "selftest", "fixtures"))
        else:
            dns[:] = sorted(d for d in dns if d not in ("tests", "selftest", "fixtures"))
        for fn in sorted(fns):
            if not fn.endswith(".sh"):
                continue
            p = os.path.join(dp, fn)
            ap = os.path.abspath(p)
            seen.add(ap)
            is_e2e = bool(e2e_dir) and os.path.abspath(dp) == os.path.abspath(e2e_dir)
            scan_file(p, is_e2e)

if e2e_dir and os.path.isdir(e2e_dir):
    for fn in sorted(os.listdir(e2e_dir)):
        if not fn.endswith(".sh"):
            continue
        p = os.path.join(e2e_dir, fn)
        ap = os.path.abspath(p)
        if ap not in seen and os.path.isfile(p):
            seen.add(ap)
            scan_file(p, True)

for path, lineno, rule, detail in sorted(findings):
    print(f"{path}:{lineno}: {rule}: {detail}")

sys.exit(1 if findings else 0)
PY
}

_frob_envelope_cli() {
  if [ "$#" -lt 1 ]; then
    printf 'usage: scripts/lib/envelope.sh <run|target-dir|scratch-dir|disk-preflight|selftest> [args...]\n' >&2
    return 2
  fi
  _fec_sub="$1"
  shift
  case "$_fec_sub" in
    run) frob_envelope_run "$@" ;;
    target-dir) frob_target_dir "$@" ;;
    scratch-dir) frob_scratch_dir "$@" ;;
    disk-preflight) frob_disk_preflight "$@" ;;
    selftest) frob_envelope_selftest "$@" ;;
    *)
      printf 'envelope.sh: unknown subcommand: %s\n' "$_fec_sub" >&2
      return 2
      ;;
  esac
}

case "$0" in
  */e2e/envelope.sh|e2e/envelope.sh) ;;
  */lib/envelope.sh|./envelope.sh|envelope.sh)
    _frob_envelope_cli "$@"
    exit $?
    ;;
esac
