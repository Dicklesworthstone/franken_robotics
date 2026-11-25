#!/usr/bin/env bash
# e2e: bead=N34-15 milestone=M0 labels=m0,workspace,solid,envelope seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of the machine-envelope guard (scripts/lib/envelope.sh),
# the pre-commit recipe (scripts/precommit.sh), and the static self-test scanner
# (bead N34-15).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/envelope/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "envelope" "N34-15" "$@"

ENVELOPE_SH="${E2E_REPO_ROOT}/scripts/lib/envelope.sh"
PRECOMMIT_SH="${E2E_REPO_ROOT}/scripts/precommit.sh"

# ---------------------------------------------------------------------------
# Step 1: Known-answer E0 defaults & helper conventions (Rules 1, 2, 3)
# ---------------------------------------------------------------------------
e2e_step "defaults_and_helpers" "Verify E0 known-answer limits (16 GiB, 8 cores, 3600 s, 1 device), target/scratch dirs, disk preflight, and absolute cargo refusal"

e2e_assert_eq "ka_e0_mem_gib" "${FROB_ENVELOPE_E0_MEM_GIB}" "16"
e2e_assert_eq "ka_e0_cores" "${FROB_ENVELOPE_E0_CORES}" "8"
e2e_assert_eq "ka_e0_wall_s" "${FROB_ENVELOPE_E0_WALL_S}" "3600"
e2e_assert_eq "ka_e0_devices" "${FROB_ENVELOPE_E0_DEVICES}" "1"

TD_VAL="$(RCH_TARGET_BASE="/tmp/frob-e2e-tbase" frob_target_dir "x")"
e2e_assert_eq "target_dir_under_rch_base" "${TD_VAL}" "/tmp/frob-e2e-tbase/x"

e2e_cmd --name "disk_preflight_ok" -- sh "${ENVELOPE_SH}" disk-preflight "${E2E_WORK_DIR}" "0.0001"
e2e_assert_eq "disk_preflight_ok_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "disk_preflight_short" --expect-exit "4" -- \
  env "FROB_ENVELOPE_LOG_FILE=${E2E_WORK_DIR}/disk_short.ndjson" \
  sh "${ENVELOPE_SH}" disk-preflight "${E2E_WORK_DIR}" "999999999"
e2e_assert_eq "disk_preflight_short_exit" "${E2E_LAST_EXIT}" "4"

e2e_cmd --name "abs_cargo_refused" --expect-exit "4" -- \
  env "FROB_ENVELOPE_LOG_FILE=${E2E_WORK_DIR}/abs_cargo.ndjson" \
  sh "${ENVELOPE_SH}" run -- /usr/bin/cargo --version
e2e_assert_eq "abs_cargo_refused_exit" "${E2E_LAST_EXIT}" "4"

# ---------------------------------------------------------------------------
# Step 2: Positive live case — cargo metadata under E0 guard (BD-001 skeleton)
# ---------------------------------------------------------------------------
e2e_step "positive_cargo_metadata" "Run cargo metadata --format-version 1 --no-deps --locked --offline under E0 guard"

META_ENV_LOG="${E2E_WORK_DIR}/positive_meta.ndjson"
e2e_cmd --name "guarded_cargo_metadata" -- \
  env "FROB_ENVELOPE_LOG_FILE=${META_ENV_LOG}" \
  sh "${ENVELOPE_SH}" run --class E0 -- \
    cargo metadata --manifest-path "${E2E_REPO_ROOT}/Cargo.toml" --format-version 1 --no-deps --locked --offline
e2e_assert_eq "positive_cargo_metadata_exit" "${E2E_LAST_EXIT}" "0"
e2e_assert_file "positive_cargo_metadata_env_log" "${META_ENV_LOG}"

META_WALL_S="$(python3 - "${META_ENV_LOG}" <<'PY'
import json, sys
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "metric" and obj.get("name") == "wall_s":
        print(obj["value"])
        break
PY
)"
META_RSS_MIB="$(python3 - "${META_ENV_LOG}" <<'PY'
import json, sys
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "metric" and obj.get("name") == "peak_rss_mib":
        print(obj["value"])
        break
PY
)"
e2e_metric "cargo_metadata_wall_s" "${META_WALL_S:-0}" "s" "<=" "3600"
e2e_metric "cargo_metadata_peak_rss_mib" "${META_RSS_MIB:-0}" "MiB" "<=" "16384"

# ---------------------------------------------------------------------------
# Step 3: Negative controls (over-time, over-memory, orphan child, parent signal)
# ---------------------------------------------------------------------------
e2e_step "negative_controls_runtime" "Verify wall-time breach, 2 GiB memory breach, orphan child reaping, and live-proof wall-s raise"

# NC1: over-time command (sleep 5 under --wall-s 1)
TIMEOUT_LOG="${E2E_WORK_DIR}/nc_timeout.ndjson"
e2e_expect_caught "over_time_command" -- \
  env "FROB_ENVELOPE_LOG_FILE=${TIMEOUT_LOG}" \
  sh "${ENVELOPE_SH}" run --wall-s 1 -- sleep 5
e2e_assert_eq "nc_timeout_exit_124" "${E2E_LAST_EXIT}" "124"

TIMEOUT_CHECK="$(python3 - "${TIMEOUT_LOG}" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], "r", encoding="utf-8") if l.strip()]
ok = any(o.get("event") == "assert" and o.get("status") == "fail" and "wall_time" in o.get("name", "") and o.get("expected") == 1 for o in lines)
print("ok" if ok else "missing")
PY
)"
e2e_assert_eq "nc_timeout_logged_breach" "${TIMEOUT_CHECK}" "ok"

# Live-dependency proof: raising --wall-s above command duration passes
e2e_cmd --name "live_proof_wall_raised" -- \
  env "FROB_ENVELOPE_LOG_FILE=${E2E_WORK_DIR}/live_proof_ok.ndjson" \
  sh "${ENVELOPE_SH}" run --wall-s 5 -- sleep 1
e2e_assert_eq "live_proof_wall_raised_exit_0" "${E2E_LAST_EXIT}" "0"

# NC2: over-memory command (2 GiB allocator under --mem-gib 1)
ALLOC_PY="${E2E_WORK_DIR}/alloc_2gib.py"
cat > "${ALLOC_PY}" <<'PY'
import time
chunks = []
for i in range(32):
    buf = bytearray(64 * 1024 * 1024)
    for off in range(0, len(buf), 4096):
        buf[off] = (i + 1) & 0xFF
    chunks.append(buf)
    if len(chunks) >= 17:
        time.sleep(0.15)
time.sleep(2.0)
PY
MEM_LOG="${E2E_WORK_DIR}/nc_mem.ndjson"
e2e_expect_caught "over_memory_command" -- \
  env "FROB_ENVELOPE_LOG_FILE=${MEM_LOG}" \
  sh "${ENVELOPE_SH}" run --mem-gib 1 --wall-s 30 -- python3 "${ALLOC_PY}"

MEM_CHECK="$(python3 - "${MEM_LOG}" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], "r", encoding="utf-8") if l.strip()]
ok = any(o.get("event") == "assert" and o.get("status") == "fail" and "memory" in o.get("name", "") and o.get("expected") == 1 for o in lines)
print("ok" if ok else "missing")
PY
)"
e2e_assert_eq "nc_memory_logged_breach" "${MEM_CHECK}" "ok"

# NC3: background orphan child is reaped when guard exits
ORPHAN_PID_FILE="${E2E_WORK_DIR}/orphan.pid"
e2e_cmd --name "pgrp_orphan_spawn" -- \
  env "FROB_ENVELOPE_LOG_FILE=${E2E_WORK_DIR}/pgrp_orphan.ndjson" \
  sh "${ENVELOPE_SH}" run --wall-s 10 -- sh -c "sleep 30 & echo \$! > '${ORPHAN_PID_FILE}'; exit 0"
e2e_assert_file "orphan_pid_recorded" "${ORPHAN_PID_FILE}"
ORPHAN_PID="$(cat "${ORPHAN_PID_FILE}")"
sleep 0.2
e2e_expect_caught "orphan_child_reaped" -- kill -0 "${ORPHAN_PID}"

# ---------------------------------------------------------------------------
# Step 4: Approval gating & hash logging against scratch compute_envelope.toml
# ---------------------------------------------------------------------------
e2e_step "approval_gating" "Verify E2 refusal without/invalid approval and acceptance with approved row + logged row hash"

SCRATCH_REG="${E2E_WORK_DIR}/compute_envelope.toml"
cat > "${SCRATCH_REG}" <<'EOF'
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
state = "approved"
reference = "docs/adr/ADR-OPEN-SPARK-2.md"

[class.E2]
scope = "a design campaign"
limits = { mem_gib = 48, cores = 24, wall_s_per_run = 7200, devices = 2, device_hours = 48 }
state = "approved"
reference = "docs/adr/ADR-OPEN-SPARK-2.md"

[[request]]
id = "R-E2-APPROVED"
parent = ""
packages = ["WP-60"]
scope = "e2e approved E2 campaign"
admits = ["campaign:e2e"]
class = "E2"
limits = { mem_gib = 32, cores = 12, wall_s_per_run = 7200, devices = 2, device_hours = 48 }
not_approved_fallback = "run E0"
open_question = "OPEN-PROG-4"
status = "approved"
approved_by = "owner"
date = "2026-11-24"
decision = "docs/adr/ADR-OPEN-PROG-4.md"
token = "tok-e2e"
expires = "2099-12-31"
EOF

# NC4: campaign-class E2 with no approval refused exit 4
e2e_expect_caught "campaign_without_approval" -- \
  env "FROB_ENVELOPE_LOG_FILE=${E2E_WORK_DIR}/nc_no_approval.ndjson" \
  sh "${ENVELOPE_SH}" run --class E2 --registry "${SCRATCH_REG}" -- true
e2e_assert_eq "nc_no_approval_exit_4" "${E2E_LAST_EXIT}" "4"

# Approved E2 row runs with row limits and logs approval row hash
APP_LOG="${E2E_WORK_DIR}/approved_e2.ndjson"
e2e_cmd --name "approved_e2_runs" -- \
  env "FROB_ENVELOPE_LOG_FILE=${APP_LOG}" \
  sh "${ENVELOPE_SH}" run --class E2 --approval R-E2-APPROVED --registry "${SCRATCH_REG}" -- \
    sh -c 'printf "%s:%s:%s" "$FROB_ENVELOPE_MEM_GIB" "$FROB_ENVELOPE_CORES" "$FROB_ENVELOPE_WALL_S"'
e2e_assert_eq "approved_e2_exit_0" "${E2E_LAST_EXIT}" "0"
e2e_assert_eq "approved_e2_limits" "$(cat "${E2E_LAST_STDOUT}")" "32:12:7200"

APP_HASH="$(python3 - "${APP_LOG}" <<'PY'
import json, sys
for line in open(sys.argv[1], "r", encoding="utf-8"):
    obj = json.loads(line)
    if obj.get("event") == "start":
        print(obj.get("approval_hash", ""))
        break
PY
)"
if [ -n "${APP_HASH}" ] && [ "${APP_HASH}" != "none" ]; then
  e2e_assert_eq "approved_e2_hash_logged" "logged" "logged"
else
  e2e_assert_eq "approved_e2_hash_logged" "${APP_HASH}" "blake3-or-sha256-hash"
fi

# ---------------------------------------------------------------------------
# Step 5: Self-test scanner (AC 7 & AC 8) — live scripts/ and synthetic NC5
# ---------------------------------------------------------------------------
e2e_step "selftest_scanner" "Verify static scanner passes on repository scripts/ and catches training outside frob-launch"

SYN_SCRIPTS="${E2E_WORK_DIR}/syn_scripts"
mkdir -p "${SYN_SCRIPTS}/e2e"
cat > "${SYN_SCRIPTS}/pilot_bad.sh" <<'EOF'
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 labels=pilot seed=0 needs=cpu
python3 train.py
EOF

# NC5: training command started outside frob-launch is caught
e2e_expect_caught "training_outside_launcher" -- \
  sh "${ENVELOPE_SH}" selftest --scripts-dir "${SYN_SCRIPTS}" --e2e-dir "${SYN_SCRIPTS}/e2e"

# Positive: live repository scripts/ scan clean
e2e_cmd --name "live_repo_selftest" -- \
  sh "${ENVELOPE_SH}" selftest --scripts-dir "${E2E_REPO_ROOT}/scripts" --e2e-dir "${E2E_REPO_ROOT}/scripts/e2e"
e2e_assert_eq "live_repo_selftest_exit_0" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 6: scripts/precommit.sh on a clean scratch checkout + unit test suite
# ---------------------------------------------------------------------------
e2e_step "precommit_and_unit_suite" "Run scripts/precommit.sh on a clean scratch checkout and run scripts/lib/tests/envelope_test.sh"

PC_SCRATCH="${E2E_WORK_DIR}/clean_checkout"
mkdir -p "${PC_SCRATCH}/bin" "${PC_SCRATCH}/scripts/lib/tests" "${PC_SCRATCH}/src"
cp "${ENVELOPE_SH}" "${PC_SCRATCH}/scripts/lib/envelope.sh"
cat > "${PC_SCRATCH}/Cargo.toml" <<'EOF'
[package]
name = "e2e-clean-checkout"
version = "0.0.1"
edition = "2024"
EOF
printf 'pub fn clean() -> u32 { 42 }\n' > "${PC_SCRATCH}/src/lib.rs"
cat > "${PC_SCRATCH}/scripts/lib/tests/ok_test.sh" <<'EOF'
#!/bin/sh
exit 0
EOF
chmod +x "${PC_SCRATCH}/scripts/lib/tests/ok_test.sh"
cat > "${PC_SCRATCH}/bin/cargo" <<'EOF'
#!/bin/sh
if [ "$1" = "xtask" ] && [ "$#" -eq 1 ]; then
  printf 'usage: cargo xtask <check>\n\nChecks:\n  check-one   first\n  check-two   second\n' >&2
  exit 3
fi
exit 0
EOF
chmod +x "${PC_SCRATCH}/bin/cargo"
git -C "${PC_SCRATCH}" init -q
git -C "${PC_SCRATCH}" config user.email "e2e@example.com"
git -C "${PC_SCRATCH}" config user.name "E2E"
git -C "${PC_SCRATCH}" add Cargo.toml src/lib.rs scripts/lib/envelope.sh scripts/lib/tests/ok_test.sh
git -C "${PC_SCRATCH}" commit -q -m "clean scratch checkout"

e2e_cmd --name "precommit_clean_scratch" -- \
  env "PATH=${PC_SCRATCH}/bin:${PATH}" \
      "FROB_ENVELOPE_LOG_FILE=${E2E_WORK_DIR}/precommit_env.ndjson" \
  sh "${PRECOMMIT_SH}" --root "${PC_SCRATCH}" --target-dir "${E2E_WORK_DIR}/pc_target"
e2e_assert_eq "precommit_clean_scratch_exit_0" "${E2E_LAST_EXIT}" "0"

if grep -Fq "BEGIN PRECOMMIT VERIFICATION BLOCK" "${E2E_LAST_STDOUT}" &&
   grep -Fq "cargo xtask check-one" "${E2E_LAST_STDOUT}" &&
   grep -Fq "cargo xtask check-two" "${E2E_LAST_STDOUT}"; then
  e2e_assert_eq "precommit_block_complete" "ok" "ok"
else
  e2e_assert_eq "precommit_block_complete" "missing" "ok"
fi

e2e_cmd --name "envelope_unit_tests" -- sh "${E2E_REPO_ROOT}/scripts/lib/tests/envelope_test.sh"
e2e_assert_eq "envelope_unit_tests_exit_0" "${E2E_LAST_EXIT}" "0"
