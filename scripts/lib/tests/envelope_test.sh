#!/bin/sh
# Unit tests for scripts/lib/envelope.sh and scripts/precommit.sh (bead N34-15).
#
# Runs real processes (no mocks) and deletes nothing (COMMON.md 9b).
# POSIX sh compatible; executable directly (`sh scripts/lib/tests/envelope_test.sh`).

set -eu
export LC_ALL=C

SCRIPT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH='' cd -- "${SCRIPT_DIR}/../../.." && pwd)"
ENVELOPE_SH="${REPO_ROOT}/scripts/lib/envelope.sh"
PRECOMMIT_SH="${REPO_ROOT}/scripts/precommit.sh"

FROB_REPO_ROOT="${REPO_ROOT}"
export FROB_REPO_ROOT
# shellcheck source=scripts/lib/envelope.sh
. "${ENVELOPE_SH}"

SCRATCH_BASE="$(frob_scratch_dir "frob-envelope-test")"
mkdir -p "${SCRATCH_BASE}"
TMP_ROOT="$(mktemp -d "${SCRATCH_BASE}/run.XXXXXX")"

PASS=0
FAIL=0

pass() {
  PASS=$((PASS + 1))
  printf 'PASS: %s\n' "$1"
}

fail() {
  FAIL=$((FAIL + 1))
  printf 'FAIL: %s — %s\n' "$1" "$2" >&2
}

now_ms() {
  python3 -c 'import time; print(int(time.monotonic() * 1000))'
}

write_scratch_registry() {
  _wsr_path="$1"
  _wsr_e1_state="$2"
  _wsr_e2_state="$3"
  cat > "$_wsr_path" <<EOF
schema = "frob.compute_envelope/1"
in_force = "E0"

[class.E0]
scope = "default"
limits = { mem_gib = 16, cores = 8, wall_s_per_run = 3600, devices = 1 }
state = "approved"
reference = "OPEN-SPARK-2 default"

[class.E1]
scope = "a named gate measurement"
limits = { mem_gib = 32, cores = 16, wall_s_per_run = 7200, devices = 2, device_hours = 12 }
state = "${_wsr_e1_state}"
reference = "docs/adr/ADR-OPEN-SPARK-2.md"

[class.E2]
scope = "a design campaign"
limits = { mem_gib = 48, cores = 24, wall_s_per_run = 7200, devices = 2, device_hours = 48 }
state = "${_wsr_e2_state}"
reference = "docs/adr/ADR-OPEN-SPARK-2.md"

[[request]]
id = "R-E1-OK"
parent = ""
packages = ["WP-01"]
scope = "gate measurement"
admits = ["gate:GATE-TEST-01"]
class = "E1"
limits = { mem_gib = 24, cores = 12, wall_s_per_run = 5400, devices = 2, device_hours = 12 }
not_approved_fallback = "run E0"
open_question = "OPEN-SPARK-2"
status = "approved"
approved_by = "owner"
date = "2026-11-24"
decision = "docs/adr/ADR-OPEN-SPARK-2.md"
token = "tok1"
expires = "2099-12-31"

[[request]]
id = "R-E2-OK"
parent = ""
packages = ["WP-60"]
scope = "design campaign"
admits = ["campaign:test"]
class = "E2"
limits = { mem_gib = 32, cores = 12, wall_s_per_run = 7200, devices = 2, device_hours = 48 }
not_approved_fallback = "run E0"
open_question = "OPEN-PROG-4"
status = "approved"
approved_by = "owner"
date = "2026-11-24"
decision = "docs/adr/ADR-OPEN-PROG-4.md"
token = "tok2"
expires = "2099-12-31"

[[request]]
id = "R-REQ"
parent = ""
packages = ["WP-60"]
scope = "unapproved request"
admits = ["campaign:req"]
class = "E2"
limits = { device_hours = 48 }
not_approved_fallback = "run E0"
open_question = "OPEN-PROG-4"
status = "requested"
approved_by = ""
date = ""
decision = ""
token = ""
expires = ""

[[request]]
id = "R-EXP"
parent = ""
packages = ["WP-60"]
scope = "expired approval"
admits = ["campaign:exp"]
class = "E2"
limits = { device_hours = 48 }
not_approved_fallback = "run E0"
open_question = "OPEN-PROG-4"
status = "approved"
approved_by = "owner"
date = "2020-01-01"
decision = "docs/adr/ADR-OLD.md"
token = "tok3"
expires = "2020-06-01"
EOF
}

# ---------------------------------------------------------------------------
# 1. timeout_kills
# ---------------------------------------------------------------------------
test_timeout_kills() {
  _log="${TMP_ROOT}/timeout_kills.ndjson"
  _t0="$(now_ms)"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --wall-s 1 -- sleep 5 || _rc=$?
  _t1="$(now_ms)"
  _elapsed_ms=$((_t1 - _t0))

  if [ "$_rc" -eq 0 ]; then
    fail "timeout_kills" "expected non-zero exit, got 0"
    return
  fi
  if [ "$_elapsed_ms" -gt 2000 ]; then
    fail "timeout_kills" "expected termination within 2s, took ${_elapsed_ms} ms"
    return
  fi
  if python3 - "$_log" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], "r", encoding="utf-8") if l.strip()]
has_start = any(o.get("event") == "start" for o in lines)
has_end = any(o.get("event") == "end" for o in lines)
has_metrics = {o.get("name") for o in lines if o.get("event") == "metric"} >= {"peak_rss_mib", "wall_s", "exit_code"}
breach = [o for o in lines if o.get("event") == "assert" and o.get("status") == "fail" and "wall_time" in o.get("name", "") and o.get("observed") is not None and o.get("expected") == 1]
sys.exit(0 if (has_start and has_end and has_metrics and breach) else 1)
PY
  then
    pass "timeout_kills"
  else
    fail "timeout_kills" "structured log missing start/metric/breach-assert/end in ${_log}"
  fi
}

# ---------------------------------------------------------------------------
# 2. live_proof_raising_wall_s_passes
# ---------------------------------------------------------------------------
test_live_proof_raising_wall_s_passes() {
  _log1="${TMP_ROOT}/wall_tight.ndjson"
  _log2="${TMP_ROOT}/wall_raised.ndjson"
  _rc1=0
  FROB_ENVELOPE_LOG_FILE="$_log1" frob_envelope_run --wall-s 0.3 -- sleep 1 || _rc1=$?
  _rc2=0
  FROB_ENVELOPE_LOG_FILE="$_log2" frob_envelope_run --wall-s 5 -- sleep 1 || _rc2=$?
  if [ "$_rc1" -ne 0 ] && [ "$_rc2" -eq 0 ]; then
    pass "live_proof_raising_wall_s_passes"
  else
    fail "live_proof_raising_wall_s_passes" "expected tight!=0 & raised==0, got tight=${_rc1} raised=${_rc2}"
  fi
}

# ---------------------------------------------------------------------------
# 3. memory_limit_kills (real 2 GiB allocator under --mem-gib 1)
# ---------------------------------------------------------------------------
test_memory_limit_kills() {
  _alloc="${TMP_ROOT}/alloc_2gib.py"
  cat > "$_alloc" <<'PY'
import time
chunks = []
# Allocate up to 2 GiB in 64 MiB chunks, touching pages so RSS grows genuinely.
for i in range(32):
    buf = bytearray(64 * 1024 * 1024)
    for off in range(0, len(buf), 4096):
        buf[off] = (i + 1) & 0xFF
    chunks.append(buf)
    if len(chunks) >= 17:
        time.sleep(0.15)
time.sleep(2.0)
PY
  _log="${TMP_ROOT}/memory_limit_kills.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --mem-gib 1 --wall-s 30 -- python3 "$_alloc" || _rc=$?
  if [ "$_rc" -eq 0 ]; then
    fail "memory_limit_kills" "2 GiB allocator exited 0 under --mem-gib 1"
    return
  fi
  if python3 - "$_log" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], "r", encoding="utf-8") if l.strip()]
breach = [o for o in lines if o.get("event") == "assert" and o.get("status") == "fail" and "memory" in o.get("name", "") and o.get("observed") is not None and o.get("expected") == 1]
peak = [o for o in lines if o.get("event") == "metric" and o.get("name") == "peak_rss_mib" and float(o.get("value", 0)) > 512.0]
sys.exit(0 if (breach and peak) else 1)
PY
  then
    pass "memory_limit_kills"
  else
    fail "memory_limit_kills" "memory breach not logged properly in ${_log} (rc=${_rc})"
  fi
}

# ---------------------------------------------------------------------------
# 4. process_group_is_reaped
# ---------------------------------------------------------------------------
test_process_group_is_reaped() {
  _pidf="${TMP_ROOT}/bg_child.pid"
  _log="${TMP_ROOT}/pgrp_reaped.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --wall-s 10 -- sh -c "sleep 30 & echo \$! > '${_pidf}'; exit 0" || _rc=$?
  if [ ! -s "$_pidf" ]; then
    fail "process_group_is_reaped" "background child pid file not created"
    return
  fi
  _cpid="$(cat "$_pidf")"
  sleep 0.2
  if kill -0 "$_cpid" 2>/dev/null; then
    kill -TERM "$_cpid" 2>/dev/null || true
    fail "process_group_is_reaped" "background child ${_cpid} survived guard exit"
  else
    pass "process_group_is_reaped"
  fi
}

# ---------------------------------------------------------------------------
# 5. signal_to_parent_reaps_children
# ---------------------------------------------------------------------------
test_signal_to_parent_reaps_children() {
  _pidf="${TMP_ROOT}/sig_parent_child.pid"
  _log="${TMP_ROOT}/sig_parent.ndjson"
  FROB_ENVELOPE_LOG_FILE="$_log" sh -c "
    . '${ENVELOPE_SH}'
    frob_envelope_run --wall-s 30 -- sh -c 'sleep 30 & echo \$! > \"${_pidf}\"; wait'
  " >/dev/null 2>&1 &
  _parent_pid=$!

  _tries=0
  while [ ! -s "$_pidf" ] && [ "$_tries" -lt 40 ]; do
    sleep 0.1
    _tries=$((_tries + 1))
  done
  if [ ! -s "$_pidf" ]; then
    kill -TERM "$_parent_pid" 2>/dev/null || true
    wait "$_parent_pid" 2>/dev/null || true
    fail "signal_to_parent_reaps_children" "child pid file never appeared"
    return
  fi
  _cpid="$(cat "$_pidf")"
  if ! kill -0 "$_cpid" 2>/dev/null; then
    wait "$_parent_pid" 2>/dev/null || true
    fail "signal_to_parent_reaps_children" "child ${_cpid} was not alive before parent signal"
    return
  fi

  kill -TERM "$_parent_pid" 2>/dev/null || true
  wait "$_parent_pid" 2>/dev/null || true
  sleep 0.4

  if kill -0 "$_cpid" 2>/dev/null; then
    kill -TERM "$_cpid" 2>/dev/null || true
    fail "signal_to_parent_reaps_children" "child ${_cpid} survived SIGTERM to parent shell"
  else
    pass "signal_to_parent_reaps_children"
  fi
}

# ---------------------------------------------------------------------------
# 6-11. Approval & registry tests
# ---------------------------------------------------------------------------
REG_OK="${TMP_ROOT}/compute_envelope_ok.toml"
REG_PROP="${TMP_ROOT}/compute_envelope_proposed.toml"
write_scratch_registry "$REG_OK" "approved" "approved"
write_scratch_registry "$REG_PROP" "approved" "proposed"

test_class_above_default_needs_approval() {
  _sent="${TMP_ROOT}/sent_no_app"
  _log="${TMP_ROOT}/no_app.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --class E2 --registry "$REG_OK" -- touch "$_sent" 2>/dev/null || _rc=$?
  if [ "$_rc" -eq 4 ] && [ ! -e "$_sent" ]; then
    pass "class_above_default_needs_approval"
  else
    fail "class_above_default_needs_approval" "expected exit 4 and no run; got rc=${_rc} sent=$([ -e "$_sent" ] && echo yes || echo no)"
  fi
}

test_approval_class_mismatch_refuses() {
  _sent="${TMP_ROOT}/sent_mismatch"
  _log="${TMP_ROOT}/mismatch.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --class E2 --approval R-E1-OK --registry "$REG_OK" -- touch "$_sent" 2>/dev/null || _rc=$?
  if [ "$_rc" -eq 4 ] && [ ! -e "$_sent" ]; then
    pass "approval_class_mismatch_refuses"
  else
    fail "approval_class_mismatch_refuses" "expected exit 4 and no run; got rc=${_rc}"
  fi
}

test_approval_row_requested_refuses() {
  _sent="${TMP_ROOT}/sent_req"
  _log="${TMP_ROOT}/req.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --class E2 --approval R-REQ --registry "$REG_OK" -- touch "$_sent" 2>/dev/null || _rc=$?
  if [ "$_rc" -eq 4 ] && [ ! -e "$_sent" ]; then
    pass "approval_row_requested_refuses"
  else
    fail "approval_row_requested_refuses" "expected exit 4 and no run; got rc=${_rc}"
  fi
}

test_approval_row_expired_refuses() {
  _sent="${TMP_ROOT}/sent_exp"
  _log="${TMP_ROOT}/exp.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --class E2 --approval R-EXP --registry "$REG_OK" -- touch "$_sent" 2>/dev/null || _rc=$?
  if [ "$_rc" -eq 4 ] && [ ! -e "$_sent" ]; then
    pass "approval_row_expired_refuses"
  else
    fail "approval_row_expired_refuses" "expected exit 4 and no run; got rc=${_rc}"
  fi
}

test_approval_class_still_proposed_refuses() {
  _sent="${TMP_ROOT}/sent_prop"
  _log="${TMP_ROOT}/prop.ndjson"
  _rc=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --class E2 --approval R-E2-OK --registry "$REG_PROP" -- touch "$_sent" 2>/dev/null || _rc=$?
  if [ "$_rc" -eq 4 ] && [ ! -e "$_sent" ]; then
    pass "approval_class_still_proposed_refuses"
  else
    fail "approval_class_still_proposed_refuses" "expected exit 4 and no run; got rc=${_rc}"
  fi
}

test_approved_row_runs_with_row_limits_and_hash() {
  _log="${TMP_ROOT}/approved_ok.ndjson"
  _out=""
  _rc=0
  _out="$(FROB_ENVELOPE_LOG_FILE="$_log" frob_envelope_run --class E2 --approval R-E2-OK --registry "$REG_OK" -- sh -c 'printf "%s:%s:%s:%s" "$FROB_ENVELOPE_MEM_GIB" "$FROB_ENVELOPE_CORES" "$FROB_ENVELOPE_WALL_S" "$CARGO_BUILD_JOBS"')" || _rc=$?
  if [ "$_rc" -ne 0 ] || [ "$_out" != "32:12:7200:12" ]; then
    fail "approved_row_runs_with_row_limits_and_hash" "rc=${_rc} out='${_out}' expected '32:12:7200:12'"
    return
  fi
  if python3 - "$_log" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], "r", encoding="utf-8") if l.strip()]
st = [o for o in lines if o.get("event") == "start"]
ok = bool(st and st[0].get("approval_id") == "R-E2-OK" and (st[0].get("approval_hash", "").startswith("blake3:") or st[0].get("approval_hash", "").startswith("sha256:")))
sys.exit(0 if ok else 1)
PY
  then
    pass "approved_row_runs_with_row_limits_and_hash"
  else
    fail "approved_row_runs_with_row_limits_and_hash" "approval hash not logged in ${_log}"
  fi
}

test_missing_registry_means_e0() {
  _missing="${TMP_ROOT}/does_not_exist.toml"
  _log1="${TMP_ROOT}/miss_e0.ndjson"
  _log2="${TMP_ROOT}/miss_e1.ndjson"
  _out=""
  _rc1=0
  _out="$(FROB_ENVELOPE_LOG_FILE="$_log1" frob_envelope_run --class E0 --registry "$_missing" -- sh -c 'printf "%s:%s:%s:%s" "$FROB_ENVELOPE_MEM_GIB" "$FROB_ENVELOPE_CORES" "$FROB_ENVELOPE_WALL_S" "$FROB_ENVELOPE_DEVICES"')" || _rc1=$?
  _rc2=0
  FROB_ENVELOPE_LOG_FILE="$_log2" frob_envelope_run --class E1 --approval R-E1-OK --registry "$_missing" -- true 2>/dev/null || _rc2=$?
  if [ "$_rc1" -eq 0 ] && [ "$_out" = "16:8:3600:1" ] && [ "$_rc2" -eq 4 ]; then
    pass "missing_registry_means_e0"
  else
    fail "missing_registry_means_e0" "rc1=${_rc1} out='${_out}' rc2=${_rc2}"
  fi
}

# ---------------------------------------------------------------------------
# 12-14. Rule 1, 2, 3 helpers
# ---------------------------------------------------------------------------
test_absolute_cargo_refused() {
  _log1="${TMP_ROOT}/abs_cargo1.ndjson"
  _log2="${TMP_ROOT}/abs_cargo2.ndjson"
  _rc1=0
  FROB_ENVELOPE_LOG_FILE="$_log1" frob_envelope_run -- /usr/bin/cargo --version 2>/dev/null || _rc1=$?
  _rc2=0
  FROB_ENVELOPE_LOG_FILE="$_log2" frob_envelope_run -- env FOO=1 /opt/homebrew/bin/cargo --version 2>/dev/null || _rc2=$?
  if [ "$_rc1" -eq 4 ] && [ "$_rc2" -eq 4 ]; then
    pass "absolute_cargo_refused"
  else
    fail "absolute_cargo_refused" "expected exit 4 for both, got rc1=${_rc1} rc2=${_rc2}"
  fi
}

test_target_dir_convention() {
  _td_custom="$(RCH_TARGET_BASE="/tmp/custom-rch-base/" frob_target_dir x)"
  _td_fallback="$(env -u RCH_TARGET_BASE TMPDIR="/tmp/custom-tmp/" sh -c ". '${ENVELOPE_SH}'; frob_target_dir x")"
  _sd_custom="$(BIG_SCRATCH="/tmp/custom-big-scratch/" frob_scratch_dir y)"
  if [ "$_td_custom" = "/tmp/custom-rch-base/x" ] &&
     [ "$_td_fallback" = "/tmp/custom-tmp/x" ] &&
     [ "$_sd_custom" = "/tmp/custom-big-scratch/y" ]; then
    pass "target_dir_convention"
  else
    fail "target_dir_convention" "got custom='${_td_custom}' fallback='${_td_fallback}' scratch='${_sd_custom}'"
  fi
}

test_disk_preflight_refuses_when_short() {
  _log="${TMP_ROOT}/disk_short.ndjson"
  _rc_short=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_disk_preflight "$TMP_ROOT" 999999999 2>/dev/null || _rc_short=$?
  _rc_ok=0
  FROB_ENVELOPE_LOG_FILE="$_log" frob_disk_preflight "$TMP_ROOT" 0.0001 || _rc_ok=$?
  if [ "$_rc_short" -eq 4 ] && [ "$_rc_ok" -eq 0 ]; then
    pass "disk_preflight_refuses_when_short"
  else
    fail "disk_preflight_refuses_when_short" "rc_short=${_rc_short} rc_ok=${_rc_ok}"
  fi
}

# ---------------------------------------------------------------------------
# 15-17. scripts/precommit.sh tests (in isolated scratch git checkouts)
# ---------------------------------------------------------------------------
make_scratch_precommit_repo() {
  _mpr_dir="$1"
  _mpr_fmt_rc="$2"
  _mpr_usage="${3:-}"
  mkdir -p "${_mpr_dir}/bin" "${_mpr_dir}/scripts/lib/tests" "${_mpr_dir}/src"
  # The no-argument `cargo xtask` usage text the synthetic shim prints: the indented
  # default below, or a caller-supplied file.
  if [ -n "$_mpr_usage" ]; then
    cp "$_mpr_usage" "${_mpr_dir}/bin/xtask_usage.txt"
  else
    cat > "${_mpr_dir}/bin/xtask_usage.txt" <<'USAGE'
usage: cargo xtask <check> [--root <dir>] [--json]

Checks:
  check-alpha    First synthetic check
  check-beta     Second synthetic check
Exit codes: 0 clean, 2 findings, 3 usage error.
USAGE
  fi
  cp "${ENVELOPE_SH}" "${_mpr_dir}/scripts/lib/envelope.sh"

  cat > "${_mpr_dir}/Cargo.toml" <<'EOF'
[package]
name = "scratch-pkg"
version = "0.0.1"
edition = "2024"
EOF
  printf 'pub fn ok() -> i32 { 1 }\n' > "${_mpr_dir}/src/lib.rs"

  # Lightweight unit test inside scratch repo's scripts/lib/tests/
  cat > "${_mpr_dir}/scripts/lib/tests/sample_test.sh" <<'EOF'
#!/bin/sh
echo "sample_test_ran" >> "${PRECOMMIT_TRACE}"
exit 0
EOF
  chmod +x "${_mpr_dir}/scripts/lib/tests/sample_test.sh"

  # Synthetic `cargo` shim on PATH for scratch tree tests (zero real builds).
  cat > "${_mpr_dir}/bin/cargo" <<EOF
#!/bin/sh
echo "cargo \$*" >> "\${PRECOMMIT_TRACE}"
if [ "\$1" = "fmt" ]; then
  exit ${_mpr_fmt_rc}
fi
if [ "\$1" = "xtask" ]; then
  if [ "\$#" -eq 1 ]; then
    cat "${_mpr_dir}/bin/xtask_usage.txt" >&2
    exit 3
  fi
  echo "xtask:\$2" >> "\${PRECOMMIT_XTASK_TRACE}"
  exit 0
fi
exit 0
EOF
  chmod +x "${_mpr_dir}/bin/cargo"

  git -C "$_mpr_dir" init -q
  git -C "$_mpr_dir" config user.email "test@example.com"
  git -C "$_mpr_dir" config user.name "Test"
  git -C "$_mpr_dir" add Cargo.toml src/lib.rs scripts/lib/envelope.sh scripts/lib/tests/sample_test.sh
  git -C "$_mpr_dir" commit -q -m "init"
}

test_precommit_runs_listed_checks_and_prints_block() {
  _repo="${TMP_ROOT}/pc_ok"
  make_scratch_precommit_repo "$_repo" 0
  _trace="${TMP_ROOT}/pc_ok.trace"
  _xtrace="${TMP_ROOT}/pc_ok.xtask"
  _out="${TMP_ROOT}/pc_ok.out"
  : > "$_trace"
  : > "$_xtrace"

  _rc=0
  PATH="${_repo}/bin:${PATH}" \
  PRECOMMIT_TRACE="$_trace" \
  PRECOMMIT_XTASK_TRACE="$_xtrace" \
  FROB_ENVELOPE_LOG_FILE="${TMP_ROOT}/pc_ok.env.ndjson" \
    sh "${PRECOMMIT_SH}" --root "$_repo" --target-dir "${TMP_ROOT}/pc_ok_target" >"$_out" 2>&1 || _rc=$?

  _ran="$(tr '\n' ',' < "$_xtrace" | sed 's/,$//')"
  if [ "$_rc" -eq 0 ] && [ "$_ran" = "xtask:check-alpha,xtask:check-beta" ]; then
    pass "precommit_runs_listed_checks"
  else
    fail "precommit_runs_listed_checks" "rc=${_rc} xtask_ran='${_ran}' out=$(tail -n 10 "$_out")"
  fi

  if grep -Fq "BEGIN PRECOMMIT VERIFICATION BLOCK" "$_out" &&
     grep -Fq "step 1 (ubs)" "$_out" &&
     grep -Fq "step 2 (cargo-fmt)" "$_out" &&
     grep -Fq "step 3 (cargo-clippy-and-test)" "$_out" &&
     grep -Fq "step 4 (shell-tests)" "$_out" &&
     grep -Fq "step 5 (xtask-checks)" "$_out" &&
     grep -Fq "cargo xtask check-alpha" "$_out" &&
     grep -Fq "cargo xtask check-beta" "$_out"; then
    pass "precommit_prints_commit_block"
  else
    fail "precommit_prints_commit_block" "verification block missing expected step/command lines in ${_out}"
  fi
}

test_precommit_fails_on_failing_step() {
  _repo="${TMP_ROOT}/pc_fail"
  make_scratch_precommit_repo "$_repo" 1
  _trace="${TMP_ROOT}/pc_fail.trace"
  _xtrace="${TMP_ROOT}/pc_fail.xtask"
  _out="${TMP_ROOT}/pc_fail.out"
  : > "$_trace"
  : > "$_xtrace"

  _rc=0
  PATH="${_repo}/bin:${PATH}" \
  PRECOMMIT_TRACE="$_trace" \
  PRECOMMIT_XTASK_TRACE="$_xtrace" \
  FROB_ENVELOPE_LOG_FILE="${TMP_ROOT}/pc_fail.env.ndjson" \
    sh "${PRECOMMIT_SH}" --root "$_repo" --target-dir "${TMP_ROOT}/pc_fail_target" >"$_out" 2>&1 || _rc=$?

  if [ "$_rc" -eq 1 ] &&
     grep -Fq "step 2 (cargo-fmt)" "$_out" &&
     [ ! -s "$_xtrace" ] &&
     ! grep -Fq "sample_test_ran" "$_trace"; then
    pass "precommit_fails_on_failing_step"
  else
    fail "precommit_fails_on_failing_step" "expected rc=1 failing at step 2 before steps 4/5; got rc=${_rc}"
  fi
}

# ---------------------------------------------------------------------------
# 18. device_lane_outside_launcher_is_flagged (AC 7 & AC 8 synthetic + live)
# ---------------------------------------------------------------------------
test_device_lane_outside_launcher_is_flagged() {
  _sdir="${TMP_ROOT}/selftest_scripts"
  mkdir -p "${_sdir}/e2e"

  # 1) Labelled `pilot`, runs `python3 train.py` bare -> MUST be flagged with file:line
  cat > "${_sdir}/pilot_bare.sh" <<'EOF'
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 labels=pilot,m0 seed=0 needs=cpu
set -euo pipefail
python3 train.py --epochs 10
EOF

  # 2) Runs `python3 train.py` under `frob_envelope_run --class E2 --approval R1` -> MUST be flagged with file:line
  cat > "${_sdir}/above_e0_bare.sh" <<'EOF'
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 labels=m0 seed=0 needs=cpu
set -euo pipefail
frob_envelope_run --class E2 --approval R1 -- python3 train.py --epochs 10
EOF

  # 3) Labelled `pilot`, runs `frob-launch ... -- python3 train.py` -> MUST NOT be flagged
  cat > "${_sdir}/pilot_launched.sh" <<'EOF'
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 labels=pilot,m0 seed=0 needs=cpu
set -euo pipefail
frob-launch --scope campaign:c1 --request R1 --devices 1 --cores 8 --mem-gib 16 --wall-s 3600 -- python3 train.py
EOF

  # 4) E2E script calling bare `cargo test` outside guard -> MUST be flagged (AC 7)
  cat > "${_sdir}/e2e/unguarded_cargo.sh" <<'EOF'
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 labels=m0 seed=0 needs=cpu
set -euo pipefail
cargo test -p frob-core
EOF

  # 5) E2E script calling `cargo test` inside `frob_envelope_run` -> MUST NOT be flagged
  cat > "${_sdir}/e2e/guarded_cargo.sh" <<'EOF'
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 labels=m0 seed=0 needs=cpu
set -euo pipefail
frob_envelope_run --class E0 -- cargo test -p frob-core
EOF

  _scan_out="${TMP_ROOT}/selftest_scan.txt"
  _rc=0
  frob_envelope_selftest --scripts-dir "${_sdir}" --e2e-dir "${_sdir}/e2e" >"$_scan_out" 2>&1 || _rc=$?

  if [ "$_rc" -ne 1 ]; then
    fail "device_lane_outside_launcher_is_flagged" "expected exit 1 with findings, got rc=${_rc}"
    return
  fi

  if grep -Fq "/pilot_bare.sh:4:" "$_scan_out" &&
     grep -Fq "/above_e0_bare.sh:4:" "$_scan_out" &&
     grep -Fq "/unguarded_cargo.sh:4:" "$_scan_out" &&
     ! grep -Fq "/pilot_launched.sh" "$_scan_out" &&
     ! grep -Fq "/guarded_cargo.sh" "$_scan_out"; then
    pass "device_lane_outside_launcher_is_flagged"
  else
    fail "device_lane_outside_launcher_is_flagged" "unexpected scan findings: $(cat "$_scan_out")"
  fi
}

# Run all unit tests
test_timeout_kills
test_live_proof_raising_wall_s_passes
test_memory_limit_kills
test_process_group_is_reaped
test_signal_to_parent_reaps_children

# Runs scripts/precommit.sh in a scratch repo whose synthetic `cargo xtask` prints the
# usage text in file $2; sets _rc, _ran (comma-joined checks run) and _out.
run_precommit_with_usage() {
  _name="$1"
  _repo="${TMP_ROOT}/${_name}"
  make_scratch_precommit_repo "$_repo" 0 "$2"
  _trace="${TMP_ROOT}/${_name}.trace"
  _xtrace="${TMP_ROOT}/${_name}.xtask"
  _out="${TMP_ROOT}/${_name}.out"
  : > "$_trace"
  : > "$_xtrace"
  _rc=0
  PATH="${_repo}/bin:${PATH}" \
  PRECOMMIT_TRACE="$_trace" \
  PRECOMMIT_XTASK_TRACE="$_xtrace" \
  FROB_ENVELOPE_LOG_FILE="${TMP_ROOT}/${_name}.env.ndjson" \
    sh "${PRECOMMIT_SH}" --root "$_repo" --target-dir "${TMP_ROOT}/${_name}_target" >"$_out" 2>&1 || _rc=$?
  _ran="$(tr '\n' ',' < "$_xtrace" | sed 's/,$//')"
}

# The exact text crates/xtask/src/cli.rs `format_usage` prints after "error: missing
# subcommand": an inline, capitalised `Subcommands:` list. Before the fix the parser
# matched only lower-case inline lists, so step 5 ran zero checks and recorded a pass.
test_precommit_parses_real_xtask_usage() {
  _usage="${TMP_ROOT}/real_usage.txt"
  cat > "$_usage" <<'USAGE'
error: missing subcommand
usage: cargo xtask <check> [--root <dir>] [--json]

Subcommands: check-alpha, check-beta
Exit codes: 0 clean, 2 findings, 3 usage error.
USAGE
  run_precommit_with_usage pc_real "$_usage"
  if [ "$_rc" -eq 0 ] && [ "$_ran" = "xtask:check-alpha,xtask:check-beta" ] &&
     grep -Fq "cargo xtask check-alpha" "$_out" && grep -Fq "cargo xtask check-beta" "$_out"; then
    pass "precommit_parses_real_xtask_usage"
  else
    fail "precommit_parses_real_xtask_usage" "rc=${_rc} xtask_ran='${_ran}' out=$(tail -n 10 "$_out")"
  fi
}

# A usage text that names checks in a form the parser cannot read must fail step 5,
# never pass with zero checks run.
test_precommit_refuses_unparseable_xtask_usage() {
  _usage="${TMP_ROOT}/garbled_usage.txt"
  cat > "$_usage" <<'USAGE'
usage: cargo xtask <check> [--root <dir>] [--json]

Subcommands -> check-alpha | check-beta
USAGE
  run_precommit_with_usage pc_garbled "$_usage"
  if [ "$_rc" -eq 1 ] && [ ! -s "$_xtrace" ] &&
     grep -Fq "[FAIL] step 5 (xtask-checks)" "$_out" &&
     ! grep -Fq "[PASS] step 5 (xtask-checks)" "$_out"; then
    pass "precommit_refuses_unparseable_xtask_usage"
  else
    fail "precommit_refuses_unparseable_xtask_usage" "rc=${_rc} xtask_ran='${_ran}' out=$(tail -n 10 "$_out")"
  fi
}

# `Subcommands: (none)` is the honest empty registry: step 5 passes and runs nothing.
test_precommit_passes_when_xtask_lists_none() {
  _usage="${TMP_ROOT}/none_usage.txt"
  cat > "$_usage" <<'USAGE'
usage: cargo xtask <check> [--root <dir>] [--json]

Subcommands: (none)
Exit codes: 0 clean, 2 findings, 3 usage error.
USAGE
  run_precommit_with_usage pc_none "$_usage"
  if [ "$_rc" -eq 0 ] && [ ! -s "$_xtrace" ] &&
     grep -Fq "no checks registered" "$_out"; then
    pass "precommit_passes_when_xtask_lists_none"
  else
    fail "precommit_passes_when_xtask_lists_none" "rc=${_rc} xtask_ran='${_ran}' out=$(tail -n 10 "$_out")"
  fi
}

test_class_above_default_needs_approval
test_approval_class_mismatch_refuses
test_approval_row_requested_refuses
test_approval_row_expired_refuses
test_approval_class_still_proposed_refuses
test_approved_row_runs_with_row_limits_and_hash
test_missing_registry_means_e0
test_absolute_cargo_refused
test_target_dir_convention
test_disk_preflight_refuses_when_short
test_precommit_runs_listed_checks_and_prints_block
test_precommit_fails_on_failing_step
test_precommit_parses_real_xtask_usage
test_precommit_refuses_unparseable_xtask_usage
test_precommit_passes_when_xtask_lists_none
test_device_lane_outside_launcher_is_flagged

printf '\nenvelope_test.sh: passed=%d failed=%d\n' "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ]
