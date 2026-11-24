#!/usr/bin/env bash
# e2e: bead=BD-001
# e2e: milestone=M0
# e2e: labels=m0,solid,suite,workspace,xtask
# e2e: gates=INT-TOOLCHAIN-01
# e2e: seed=0
# e2e: timeout_s=1200
# e2e: needs=cpu
# e2e: plants=dirty-sibling,edited-receipt,salt-removed
#
# scripts/e2e/suite_workspace_skeleton.sh
# End-to-end verification script for BD-001 (workspace skeleton, toolchain file,
# stack lock, first resolution receipt, KA-767 twice-identical, dirty-sibling control,
# edited-receipt control, and salt-removed fs-la probe).
#
# Emits structured NDJSON lines in STD-E2E's frob.e2elog/1 format (self-contained until
# STD-E2E lands) under target/e2e-logs/suite_workspace_skeleton/.
#
# Options:
#   --skip-cargo-check   Skip `cargo check --workspace --all-targets` and the live `fs-la` probe
#                        (for shell-only verification without invoking compiler offload).
#   --scratch-only       Run the entire flow against a synthetic scratch workspace instead of
#                        the repository root (implies --skip-cargo-check unless --with-probe).
#   --log-dir <dir>      Override NDJSON/transcript log output directory.

set -euo pipefail

SCRIPT_NAME="suite_workspace_skeleton"
BEAD_KEY="BD-001"
SEED="0"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
FIRST_RECEIPT="${REPO_ROOT}/scripts/bootstrap/first_receipt.sh"
UNIT_TESTS_RUNNER="${REPO_ROOT}/scripts/bootstrap/tests/run_all.sh"

SKIP_CARGO_CHECK="0"
SCRATCH_ONLY="0"
LOG_BASE="${REPO_ROOT}/target/e2e-logs/${SCRIPT_NAME}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --skip-cargo-check)
      SKIP_CARGO_CHECK="1"
      shift
      ;;
    --scratch-only)
      SCRATCH_ONLY="1"
      SKIP_CARGO_CHECK="1"
      shift
      ;;
    --log-dir)
      [ "$#" -ge 2 ] || { printf 'usage: %s [--skip-cargo-check] [--scratch-only] [--log-dir <dir>]\n' "$0" >&2; exit 2; }
      LOG_BASE="$2"
      shift 2
      ;;
    -h|--help)
      printf 'Usage: %s [--skip-cargo-check] [--scratch-only] [--log-dir <dir>]\n' "$0"
      exit 0
      ;;
    *)
      printf '%s: unknown option: %s\n' "${SCRIPT_NAME}" "$1" >&2
      exit 2
      ;;
  esac
done

if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf '%s: b3sum not found\n' "${SCRIPT_NAME}" >&2
  exit 2
fi

UTC_STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "${LOG_BASE}"
if [ -e "${LOG_BASE}/${UTC_STAMP}.ndjson" ]; then
  UTC_STAMP="${UTC_STAMP}-$$"
fi
LOG_FILE="${LOG_BASE}/${UTC_STAMP}.ndjson"
TRANSCRIPT_FILE="${LOG_BASE}/${UTC_STAMP}.transcript.txt"
ARTIFACTS_DIR="${LOG_BASE}/${UTC_STAMP}.artifacts"
mkdir -p "${ARTIFACTS_DIR}"
printf '%s\n' "${UTC_STAMP}" > "${LOG_BASE}/LATEST"

RUN_HEX="$(printf '%s-%s' "${UTC_STAMP}" "$$" | "${B3SUM_BIN}" --no-names | cut -c1-8)"
RUN_ID="${UTC_STAMP}-${RUN_HEX}"

SCRATCH_BASE="${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-bd001-e2e"
mkdir -p "${SCRATCH_BASE}"
WORK_TMP="$(mktemp -d "${SCRATCH_BASE}/${RUN_ID}.XXXXXX")"
cleanup() {
  if [ -d "${WORK_TMP}" ]; then
    chmod -R u+w "${WORK_TMP}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

# Mutable counters tracked in files inside WORK_TMP so subshells cannot lose updates.
printf '0' > "${WORK_TMP}/seq"
printf '0' > "${WORK_TMP}/passed"
printf '0' > "${WORK_TMP}/failed"
printf '0' > "${WORK_TMP}/skipped"
printf '0' > "${WORK_TMP}/controls_caught"
printf '0' > "${WORK_TMP}/controls_missed"
printf '' > "${WORK_TMP}/step"
SCRIPT_START_MS="$(python3 -c 'import time; print(int(time.time()*1000))')"
STEP_START_MS="${SCRIPT_START_MS}"
STEP_PASSED=0
STEP_FAILED=0

now_ms() {
  python3 -c 'import time; print(int(time.time()*1000))'
}

now_iso() {
  python3 -c 'import datetime; print(datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z")'
}

next_seq() {
  local s
  s="$(cat "${WORK_TMP}/seq")"
  printf '%s' "$((s + 1))" > "${WORK_TMP}/seq"
  printf '%s' "${s}"
}

emit_json_line() {
  local step="$1"
  local event="$2"
  local extra_json="$3"
  local ts seq
  ts="$(now_iso)"
  seq="$(next_seq)"
  python3 - "${LOG_FILE}" "${TRANSCRIPT_FILE}" "${ts}" "${seq}" "${RUN_ID}" "${SCRIPT_NAME}" "${BEAD_KEY}" "${step}" "${event}" "${SEED}" "${extra_json}" <<'PYEOF'
import json, sys
log_file, tx_file, ts, seq, run_id, script, bead, step, event, seed, extra_s = sys.argv[1:12]
obj = {
    "schema": "frob.e2elog/1",
    "ts": ts,
    "seq": int(seq),
    "run_id": run_id,
    "script": script,
    "bead": bead,
    "step": step,
    "event": event,
    "seed": seed,
}
if extra_s:
    extra = json.loads(extra_s)
    obj.update(extra)
line = json.dumps(obj, separators=(",", ":"), ensure_ascii=False)
with open(log_file, "a", encoding="utf-8") as f:
    f.write(line + "\n")
summary = f"[{ts}] seq={seq} step={step or '-'} event={event}"
if event == "cmd":
    summary += f" exit={obj.get('exit')} wall_ms={obj.get('wall_ms')} cmd={obj.get('cmdline')}"
elif event == "assert":
    summary += f" status={obj.get('status')} name={obj.get('name')}"
    if obj.get("kind") == "negative-control":
        summary += f" caught={obj.get('caught')}"
elif event == "metric":
    summary += f" {obj.get('name')}={obj.get('value')} {obj.get('unit','')}"
elif event == "summary":
    summary += f" passed={obj.get('asserts_passed')} failed={obj.get('asserts_failed')} controls_caught={obj.get('controls_caught')} exit={obj.get('exit')}"
with open(tx_file, "a", encoding="utf-8") as f:
    f.write(summary + "\n")
print(summary)
PYEOF
}

CURRENT_STEP=""
begin_step() {
  local id="$1"
  local title="$2"
  if [ -n "${CURRENT_STEP}" ]; then
    end_step
  fi
  CURRENT_STEP="${id}"
  STEP_START_MS="$(now_ms)"
  STEP_PASSED=0
  STEP_FAILED=0
  emit_json_line "${CURRENT_STEP}" "start" "$(python3 -c 'import json,sys; print(json.dumps({"title":sys.argv[1]}))' "${title}")"
}

end_step() {
  if [ -z "${CURRENT_STEP}" ]; then
    return
  fi
  local end_ms wall_ms
  end_ms="$(now_ms)"
  wall_ms=$((end_ms - STEP_START_MS))
  emit_json_line "${CURRENT_STEP}" "end" "{\"wall_ms\":${wall_ms},\"asserts_passed\":${STEP_PASSED},\"asserts_failed\":${STEP_FAILED}}"
  CURRENT_STEP=""
}

record_assert() {
  local name="$1"
  local kind="$2"
  local status="$3"
  local extra="${4-}"
  [ -n "${extra}" ] || extra='{}'
  local p f sk
  if [ "${status}" = "pass" ]; then
    p="$(cat "${WORK_TMP}/passed")"; printf '%s' "$((p + 1))" > "${WORK_TMP}/passed"
    STEP_PASSED=$((STEP_PASSED + 1))
  elif [ "${status}" = "fail" ]; then
    f="$(cat "${WORK_TMP}/failed")"; printf '%s' "$((f + 1))" > "${WORK_TMP}/failed"
    STEP_FAILED=$((STEP_FAILED + 1))
  else
    sk="$(cat "${WORK_TMP}/skipped")"; printf '%s' "$((sk + 1))" > "${WORK_TMP}/skipped"
  fi
  local payload
  payload="$(python3 - "${name}" "${kind}" "${status}" "${extra}" <<'PYEOF'
import json, sys
d = {"name": sys.argv[1], "kind": sys.argv[2], "status": sys.argv[3]}
d.update(json.loads(sys.argv[4]))
print(json.dumps(d))
PYEOF
)"
  emit_json_line "${CURRENT_STEP}" "assert" "${payload}"
}

record_control() {
  local name="$1"
  local caught="$2"
  local extra="${3-}"
  [ -n "${extra}" ] || extra='{}'
  local cc cm status
  if [ "${caught}" = "true" ]; then
    cc="$(cat "${WORK_TMP}/controls_caught")"; printf '%s' "$((cc + 1))" > "${WORK_TMP}/controls_caught"
    status="pass"
  else
    cm="$(cat "${WORK_TMP}/controls_missed")"; printf '%s' "$((cm + 1))" > "${WORK_TMP}/controls_missed"
    status="fail"
  fi
  local merged
  merged="$(python3 - "${caught}" "${extra}" <<'PYEOF'
import json, sys
d = json.loads(sys.argv[2])
d["caught"] = (sys.argv[1] == "true")
print(json.dumps(d))
PYEOF
)"
  record_assert "${name}" "negative-control" "${status}" "${merged}"
}

LAST_EXIT=0
LAST_WALL_MS=0
LAST_STDOUT=""
LAST_STDERR=""

run_logged_cmd() {
  local label="$1"
  local expected_exit="$2"
  shift 2
  local seq_hint t0 t1 rc out_f err_f
  seq_hint="$(cat "${WORK_TMP}/seq")"
  out_f="${ARTIFACTS_DIR}/${CURRENT_STEP}.${seq_hint}.${label}.stdout"
  err_f="${ARTIFACTS_DIR}/${CURRENT_STEP}.${seq_hint}.${label}.stderr"
  t0="$(now_ms)"
  set +e
  "$@" >"${out_f}" 2>"${err_f}"
  rc=$?
  set -e
  t1="$(now_ms)"
  LAST_EXIT="${rc}"
  LAST_WALL_MS=$((t1 - t0))
  LAST_STDOUT="${out_f}"
  LAST_STDERR="${err_f}"

  local payload
  payload="$(python3 - "${expected_exit}" "${rc}" "${LAST_WALL_MS}" "${out_f}" "${err_f}" "$@" <<'PYEOF'
import hashlib, json, os, shlex, sys
exp_s, rc_s, wall_s, out_f, err_f = sys.argv[1:6]
argv = sys.argv[6:]
sha = hashlib.sha256(open(out_f, "rb").read()).hexdigest()
d = {
    "argv": argv,
    "cmdline": " ".join(shlex.quote(a) for a in argv),
    "exit": int(rc_s),
    "expected_exit": [int(x) for x in exp_s.split(",") if x],
    "wall_ms": int(wall_s),
    "stdout_path": os.path.basename(out_f),
    "stderr_path": os.path.basename(err_f),
    "stdout_sha256": sha,
}
print(json.dumps(d))
PYEOF
)"
  emit_json_line "${CURRENT_STEP}" "cmd" "${payload}"
}

# Script-level start event.
REPO_COMMIT="$(git --no-optional-locks -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo 'unknown')"
REPO_DIRTY_FLAG="false"
if [ -n "$(git --no-optional-locks -C "${REPO_ROOT}" status --porcelain --untracked-files=no 2>/dev/null || true)" ]; then
  REPO_DIRTY_FLAG="true"
fi
emit_json_line "" "start" "$(python3 - "${REPO_COMMIT}" "${REPO_DIRTY_FLAG}" "${SCRATCH_ONLY}" <<'PYEOF'
import json, platform, sys
commit, dirty, scratch_only = sys.argv[1:4]
print(json.dumps({
    "commit": commit,
    "dirty": dirty == "true",
    "toolchain": "nightly-2026-08-31",
    "scale": "scratch" if scratch_only == "1" else "full",
    "host": {"os": platform.system().lower(), "arch": platform.machine()},
    "plant": None,
}))
PYEOF
)"

# ---------------------------------------------------------------------------
# Step 1: Run the seven shell unit tests under scripts/bootstrap/tests/
# ---------------------------------------------------------------------------
begin_step "unit_tests" "Run all seven BD-001 bootstrap receipt unit tests"
run_logged_cmd "unit_tests" "0" bash "${UNIT_TESTS_RUNNER}"
if [ "${LAST_EXIT}" -eq 0 ]; then
  record_assert "bootstrap_unit_tests_all_pass" "exit" "pass" '{"observed":0,"expected":0}'
else
  record_assert "bootstrap_unit_tests_all_pass" "exit" "fail" "{\"observed\":${LAST_EXIT},\"expected\":0}"
fi
end_step

# Choose target workspace: either real repo root or a synthetic scratch workspace.
TARGET_ROOT="${REPO_ROOT}"
if [ "${SCRATCH_ONLY}" = "1" ]; then
  TARGET_ROOT="${WORK_TMP}/scratch_target/repo"
  mkdir -p "${WORK_TMP}/scratch_target"
  bash -c "
    source '${REPO_ROOT}/scripts/bootstrap/tests/common.sh'
    rev_a=\$(create_synthetic_sibling '${WORK_TMP}/scratch_target/asupersync' 'asupersync' '0.6.2')
    rev_f=\$(create_synthetic_sibling '${WORK_TMP}/scratch_target/frankensim' 'frankensim' '0.0.1')
    lock_body=\$(cat <<EOF
toolchain = \"nightly-2026-08-31\"

[[foundation]]
id = \"FD-FSIM\"
name = \"frankensim\"
posture = \"consume-path\"
path = \"../frankensim\"
git_rev = \"\${rev_f}\"
toolchain = \"nightly-2026-07-06\"
features = [\"referee\", \"geometry\", \"ledger\"]
build_env = { FRANKENSIM_DEPGRAPH_SALT = \"franken-robotics-dev-v1\" }
confined_to = [\"frob-fsim\", \"bedrock-direct\"]
admission_gate = \"INT-FSIM-01\"
receipt = \"\"
fallback = \"KILL-FSIM\"

[[foundation]]
id = \"FD-ASUP\"
name = \"asupersync\"
posture = \"consume\"
path = \"../asupersync\"
git_rev = \"\${rev_a}\"
version = \"=0.6.2\"
toolchain = \"nightly-2026-08-31\"
admission_gate = \"INT-ASUP-01\"
receipt = \"\"
fallback = \"FD-ASUP-01\"
EOF
)
    patch_block='[patch.crates-io]
asupersync = { path = \"../asupersync\" }'
    create_synthetic_workspace '${TARGET_ROOT}' \"\${lock_body}\" 'nightly-2026-08-31' \"\${patch_block}\" >/dev/null
  "
fi

# ---------------------------------------------------------------------------
# Step 2: Snapshot sibling git state before any checks (read-only guarantee)
# ---------------------------------------------------------------------------
begin_step "snapshot_siblings_before" "Record HEAD and git status of every sibling before checks"
SIB_BEFORE="${WORK_TMP}/siblings_before.json"
python3 - "${TARGET_ROOT}" "${SIB_BEFORE}" <<'PYEOF'
import json, os, subprocess, sys, tomllib
root, out_p = sys.argv[1], sys.argv[2]
lock_p = os.path.join(root, "franken-stack.lock")
snap = {}
if os.path.isfile(lock_p):
    data = tomllib.load(open(lock_p, "rb"))
    for row in data.get("foundation", []):
        rel = row.get("path")
        if not rel:
            continue
        d = os.path.normpath(os.path.join(root, rel))
        if os.path.isdir(d):
            head = subprocess.check_output(["git", "--no-optional-locks", "-C", d, "rev-parse", "HEAD"]).decode("ascii").strip()
            st = subprocess.check_output(["git", "--no-optional-locks", "-C", d, "status", "--porcelain"]).decode("utf-8", "replace")
            snap[row["id"]] = {"head": head, "status": st}
open(out_p, "w", encoding="utf-8").write(json.dumps(snap, sort_keys=True))
PYEOF
record_assert "siblings_snapshotted_before" "file" "pass" "{\"observed\":\"$(wc -c < "${SIB_BEFORE}" | tr -d ' ')\"}"
end_step

# ---------------------------------------------------------------------------
# Step 3: Verify single rust-toolchain.toml and lock agreement
# ---------------------------------------------------------------------------
begin_step "toolchain_and_lock" "Check rust-toolchain.toml matches franken-stack.lock and is unique"
TC_CHECK_OUT="$(python3 - "${TARGET_ROOT}" <<'PYEOF'
import json, os, sys, tomllib
root = sys.argv[1]
tc_files = []
for dirpath, dirnames, filenames in os.walk(root):
    dirnames[:] = [d for d in dirnames if d not in (".git", "target", "node_modules", ".local-implementation")]
    if "rust-toolchain.toml" in filenames or "rust-toolchain" in filenames:
        rel = os.path.relpath(dirpath, root)
        for fn in ("rust-toolchain.toml", "rust-toolchain"):
            if fn in filenames:
                p = fn if rel == "." else os.path.join(rel, fn)
                if p != "crates/frob-cert/rust-toolchain.toml":
                    tc_files.append(p)
tc = tomllib.load(open(os.path.join(root, "rust-toolchain.toml"), "rb"))
lock = tomllib.load(open(os.path.join(root, "franken-stack.lock"), "rb"))
print(json.dumps({
    "tc_files": sorted(tc_files),
    "channel": tc["toolchain"]["channel"],
    "lock_toolchain": lock["toolchain"],
}))
PYEOF
)"
TC_OK="$(printf '%s' "${TC_CHECK_OUT}" | python3 -c 'import json,sys; d=json.load(sys.stdin); print("1" if d["tc_files"]==["rust-toolchain.toml"] and d["channel"]==d["lock_toolchain"]=="nightly-2026-08-31" else "0")')"
if [ "${TC_OK}" = "1" ]; then
  record_assert "single_toolchain_matches_lock" "eq" "pass" "${TC_CHECK_OUT}"
else
  record_assert "single_toolchain_matches_lock" "eq" "fail" "${TC_CHECK_OUT}"
fi
end_step

# ---------------------------------------------------------------------------
# Step 4: cargo check --workspace --all-targets through the offload shim
# ---------------------------------------------------------------------------
begin_step "cargo_check" "Run cargo check --workspace --all-targets through offload shim"
if [ "${SKIP_CARGO_CHECK}" = "1" ]; then
  record_assert "cargo_check_workspace" "exit" "skip" '{"reason":"--skip-cargo-check or --scratch-only"}'
else
  SKELETON_TARGET_DIR="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-skeleton"
  TIMEOUT_CMD=()
  if command -v timeout >/dev/null 2>&1; then
    TIMEOUT_CMD=(timeout 1200)
  elif command -v gtimeout >/dev/null 2>&1; then
    TIMEOUT_CMD=(gtimeout 1200)
  fi
  run_logged_cmd "cargo_check" "0" env "CARGO_TARGET_DIR=${SKELETON_TARGET_DIR}" "${TIMEOUT_CMD[@]}" cargo check --manifest-path "${TARGET_ROOT}/Cargo.toml" --workspace --all-targets
  if [ "${LAST_EXIT}" -eq 0 ]; then
    record_assert "cargo_check_workspace" "exit" "pass" "{\"observed\":0,\"expected\":0,\"wall_ms\":${LAST_WALL_MS}}"
  else
    record_assert "cargo_check_workspace" "exit" "fail" "{\"observed\":${LAST_EXIT},\"expected\":0,\"wall_ms\":${LAST_WALL_MS}}"
  fi
fi
end_step

# ---------------------------------------------------------------------------
# Step 5: Run receipt generator twice (KA-767) and inspect drift counts
# ---------------------------------------------------------------------------
begin_step "receipt_twice" "Run first_receipt.sh twice on unchanged tree (KA-767)"
R1_FILE="${WORK_TMP}/receipt_run1.json"
R2_FILE="${WORK_TMP}/receipt_run2.json"

run_logged_cmd "first_receipt_run1" "0" "${FIRST_RECEIPT}" --stdout --root "${TARGET_ROOT}"
cp "${LAST_STDOUT}" "${R1_FILE}"
R1_EXIT="${LAST_EXIT}"

run_logged_cmd "first_receipt_run2" "0" "${FIRST_RECEIPT}" --stdout --root "${TARGET_ROOT}"
cp "${LAST_STDOUT}" "${R2_FILE}"
R2_EXIT="${LAST_EXIT}"

if [ "${R1_EXIT}" -eq 0 ] && [ "${R2_EXIT}" -eq 0 ] && cmp -s "${R1_FILE}" "${R2_FILE}"; then
  RECEIPT_HASH="$("${B3SUM_BIN}" --no-names "${R1_FILE}")"
  DRIFT_COUNTS="$(python3 - "${R1_FILE}" <<'PYEOF'
import json, sys
r = json.load(open(sys.argv[1], "r", encoding="utf-8"))
counts = {"on-pin": 0, "stale-lock": 0, "drift": 0}
for s in r.get("siblings", []):
    c = s.get("own_lock", {}).get("class", "unknown")
    counts[c] = counts.get(c, 0) + 1
print(json.dumps(counts, sort_keys=True))
PYEOF
)"
  record_assert "ka_767_receipt_twice_identical" "digest" "pass" "{\"ka\":\"KA-767\",\"receipt_hash\":\"${RECEIPT_HASH}\",\"siblings_by_drift_class\":${DRIFT_COUNTS}}"
else
  DIFF_INFO="$(python3 - "${R1_FILE}" "${R2_FILE}" <<'PYEOF'
import json, sys
p1, p2 = sys.argv[1], sys.argv[2]
b1, b2 = open(p1, "rb").read(), open(p2, "rb").read()
off = next((i for i in range(min(len(b1), len(b2))) if b1[i] != b2[i]), min(len(b1), len(b2)))
print(json.dumps({"first_diff": {"byte_offset": off, "file_a": p1, "file_b": p2}}))
PYEOF
)"
  record_assert "ka_767_receipt_twice_identical" "digest" "fail" "${DIFF_INFO}"
  RECEIPT_HASH=""
fi
end_step

# ---------------------------------------------------------------------------
# Step 6: Verify receipt (and committed receipt if present)
# ---------------------------------------------------------------------------
begin_step "receipt_verify" "Verify receipt hash, canonical form, ancestry and tree match"
if [ -n "${RECEIPT_HASH}" ]; then
  VERIFY_TARGET="${WORK_TMP}/${RECEIPT_HASH}.json"
  cp "${R1_FILE}" "${VERIFY_TARGET}"
  # If a committed receipt already exists under receipts/ and is named in franken-stack.lock, verify that too.
  COMMITTED_RECEIPT="$(python3 - "${TARGET_ROOT}" <<'PYEOF'
import os, sys, tomllib
root = sys.argv[1]
lock = tomllib.load(open(os.path.join(root, "franken-stack.lock"), "rb"))
for row in lock.get("foundation", []):
    h = row.get("receipt", "")
    if isinstance(h, str) and len(h) == 64:
        p = os.path.join(root, "receipts", f"{h}.json")
        if os.path.isfile(p):
            print(p)
            break
PYEOF
)"
  if [ -n "${COMMITTED_RECEIPT}" ]; then
    VERIFY_TARGET="${COMMITTED_RECEIPT}"
  fi

  run_logged_cmd "verify_receipt" "0" "${FIRST_RECEIPT}" --verify "${VERIFY_TARGET}" --root "${TARGET_ROOT}"
  if [ "${LAST_EXIT}" -eq 0 ]; then
    record_assert "receipt_verifies" "eq" "pass" "{\"receipt_hash\":\"${RECEIPT_HASH}\"}"
  else
    record_assert "receipt_verifies" "eq" "fail" "{\"observed\":${LAST_EXIT},\"expected\":0}"
  fi
else
  record_assert "receipt_verifies" "eq" "fail" '{"reason":"no receipt hash from previous step"}'
fi
end_step

# ---------------------------------------------------------------------------
# Step 7: Negative control — dirty sibling on a scratch copy (KA-767 control)
# ---------------------------------------------------------------------------
begin_step "control_dirty_sibling" "Negative control: touch one tracked file in sibling copy"
DIRTY_ROOT="${WORK_TMP}/dirty_control"
mkdir -p "${DIRTY_ROOT}"
bash -c "
  source '${REPO_ROOT}/scripts/bootstrap/tests/common.sh'
  rev=\$(create_synthetic_sibling '${DIRTY_ROOT}/sib' 'sib' '0.1.0')
  lock_body=\$(cat <<EOF
toolchain = \"nightly-2026-08-31\"

[[foundation]]
id = \"FD-CTRL\"
name = \"sib\"
posture = \"consume-path\"
path = \"../sib\"
git_rev = \"\${rev}\"
toolchain = \"nightly-2026-08-31\"
admission_gate = \"INT-CTRL-01\"
receipt = \"\"
fallback = \"fb\"
EOF
)
  create_synthetic_workspace '${DIRTY_ROOT}/repo' \"\${lock_body}\" >/dev/null
"
BASE_HASH="$("${FIRST_RECEIPT}" --root "${DIRTY_ROOT}/repo")"
BASE_FILE="${DIRTY_ROOT}/repo/receipts/${BASE_HASH}.json"

# Touch one tracked file in the sibling copy.
printf '// planted dirty edit\n' >> "${DIRTY_ROOT}/sib/src/lib.rs"
DIRTY_R="${DIRTY_ROOT}/dirty_receipt.json"
"${FIRST_RECEIPT}" --stdout --root "${DIRTY_ROOT}/repo" > "${DIRTY_R}"

DIRTY_CHECK="$(python3 - "${DIRTY_R}" "${BASE_HASH}" "${B3SUM_BIN}" <<'PYEOF'
import json, subprocess, sys
raw = open(sys.argv[1], "rb").read()
base_h, b3 = sys.argv[2], sys.argv[3]
new_h = subprocess.check_output([b3, "--no-names"], input=raw).decode("ascii").strip()
obj = json.loads(raw.decode("utf-8"))
s = obj["siblings"][0]
ok = (
    s["dirty"] is True
    and isinstance(s["tree_hash"], str) and len(s["tree_hash"]) == 64
    and s["own_lock"]["class"] == "drift"
    and s["own_lock"]["reason"] == "dirty"
    and new_h != base_h
)
print("1" if ok else "0")
PYEOF
)"
run_logged_cmd "verify_after_dirty_sibling" "2" "${FIRST_RECEIPT}" --verify "${BASE_FILE}" --root "${DIRTY_ROOT}/repo"
if [ "${DIRTY_CHECK}" = "1" ] && [ "${LAST_EXIT}" -ne 0 ]; then
  record_control "dirty_sibling_control" "true" '{"ka":"KA-767","plant":"dirty-sibling"}'
else
  record_control "dirty_sibling_control" "false" "{\"dirty_check\":${DIRTY_CHECK},\"verify_exit\":${LAST_EXIT}}"
fi
end_step

# ---------------------------------------------------------------------------
# Step 8: Negative control — edited receipt (1-byte flip must fail verify)
# ---------------------------------------------------------------------------
begin_step "control_edited_receipt" "Negative control: receipt edited by one character must not verify"
EDITED_FILE="${DIRTY_ROOT}/edited_${BASE_HASH}.json"
# Restore clean sibling so ONLY the receipt edit triggers failure.
git --no-optional-locks -C "${DIRTY_ROOT}/sib" checkout -- src/lib.rs
cp "${BASE_FILE}" "${EDITED_FILE}"
python3 - "${EDITED_FILE}" <<'PYEOF'
import sys
p = sys.argv[1]
b = bytearray(open(p, "rb").read())
idx = b.index(b"nightly-2026-08-31")
b[idx] = ord("N")
open(p, "wb").write(bytes(b))
PYEOF
# Keep exact expected filename stem in a separate directory so hash mismatch is tested cleanly.
EDITED_DIR="${DIRTY_ROOT}/edited_dir"
mkdir -p "${EDITED_DIR}"
mv "${EDITED_FILE}" "${EDITED_DIR}/${BASE_HASH}.json"
run_logged_cmd "verify_edited_receipt" "2" "${FIRST_RECEIPT}" --verify "${EDITED_DIR}/${BASE_HASH}.json" --root "${DIRTY_ROOT}/repo"
if [ "${LAST_EXIT}" -ne 0 ]; then
  record_control "edited_receipt_control" "true" '{"plant":"edited-receipt"}'
else
  record_control "edited_receipt_control" "false" '{"plant":"edited-receipt","verify_exit":0}'
fi
end_step

# ---------------------------------------------------------------------------
# Step 9: Negative control — salt-removed fs-la probe build (read-only on frankensim)
# ---------------------------------------------------------------------------
begin_step "control_salt_removed" "Negative control: fs-la probe without FRANKENSIM_DEPGRAPH_SALT must fail closed"
if [ "${SKIP_CARGO_CHECK}" = "1" ]; then
  record_assert "salt_removed_fs_la_probe" "negative-control" "skip" '{"reason":"--skip-cargo-check or --scratch-only"}'
elif [ ! -d "${REPO_ROOT}/../frankensim/crates/fs-la" ]; then
  record_assert "salt_removed_fs_la_probe" "negative-control" "skip" '{"reason":"../frankensim/crates/fs-la absent"}'
else
  PROBE_DIR="${WORK_TMP}/salt_probe"
  mkdir -p "${PROBE_DIR}/src"
  FSIM_ABS="$(cd "${REPO_ROOT}/../frankensim" && pwd)"
  ASUP_ABS="$(cd "${REPO_ROOT}/../asupersync" && pwd)"
  cat > "${PROBE_DIR}/Cargo.toml" <<EOF
[package]
name = "frob-salt-probe"
version = "0.0.1"
edition = "2024"

[dependencies]
fs-la = { path = "${FSIM_ABS}/crates/fs-la" }

[patch.crates-io]
asupersync = { path = "${ASUP_ABS}" }
EOF
  printf 'pub fn probe() {}\n' > "${PROBE_DIR}/src/lib.rs"
  cp "${REPO_ROOT}/rust-toolchain.toml" "${PROBE_DIR}/rust-toolchain.toml"
  PROBE_TARGET="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-bd001-salt-probe"
  # Run with FRANKENSIM_DEPGRAPH_SALT and FRANKENSIM_DEPGRAPH_RECEIPT unset, into private target dir.
  # Never modifies ../frankensim (lockfile lives in PROBE_DIR, target in PROBE_TARGET).
  run_logged_cmd "fs_la_salt_removed_probe" "101" env -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT "CARGO_TARGET_DIR=${PROBE_TARGET}" cargo check --manifest-path "${PROBE_DIR}/Cargo.toml" --offline
  if [ "${LAST_EXIT}" -ne 0 ] && grep -q "FRANKENSIM_DEPGRAPH_SALT" "${LAST_STDERR}"; then
    record_control "salt_removed_fs_la_probe" "true" '{"plant":"salt-removed"}'
  else
    record_control "salt_removed_fs_la_probe" "false" "{\"plant\":\"salt-removed\",\"exit\":${LAST_EXIT}}"
  fi
fi
end_step

# ---------------------------------------------------------------------------
# Step 10: Confirm every sibling checkout is byte-identical before and after
# ---------------------------------------------------------------------------
begin_step "snapshot_siblings_after" "Verify no sibling checkout was modified during the e2e run"
SIB_AFTER="${WORK_TMP}/siblings_after.json"
python3 - "${TARGET_ROOT}" "${SIB_AFTER}" <<'PYEOF'
import json, os, subprocess, sys, tomllib
root, out_p = sys.argv[1], sys.argv[2]
lock_p = os.path.join(root, "franken-stack.lock")
snap = {}
if os.path.isfile(lock_p):
    data = tomllib.load(open(lock_p, "rb"))
    for row in data.get("foundation", []):
        rel = row.get("path")
        if not rel:
            continue
        d = os.path.normpath(os.path.join(root, rel))
        if os.path.isdir(d):
            head = subprocess.check_output(["git", "--no-optional-locks", "-C", d, "rev-parse", "HEAD"]).decode("ascii").strip()
            st = subprocess.check_output(["git", "--no-optional-locks", "-C", d, "status", "--porcelain"]).decode("utf-8", "replace")
            snap[row["id"]] = {"head": head, "status": st}
open(out_p, "w", encoding="utf-8").write(json.dumps(snap, sort_keys=True))
PYEOF
if cmp -s "${SIB_BEFORE}" "${SIB_AFTER}"; then
  record_assert "siblings_unmodified" "eq" "pass" '{"observed":"identical","expected":"identical"}'
else
  record_assert "siblings_unmodified" "eq" "fail" "{\"first_diff\":{\"file_a\":\"${SIB_BEFORE}\",\"file_b\":\"${SIB_AFTER}\"}}"
fi
end_step

# ---------------------------------------------------------------------------
# Final summary line
# ---------------------------------------------------------------------------
TOTAL_PASSED="$(cat "${WORK_TMP}/passed")"
TOTAL_FAILED="$(cat "${WORK_TMP}/failed")"
TOTAL_SKIPPED="$(cat "${WORK_TMP}/skipped")"
TOTAL_CC="$(cat "${WORK_TMP}/controls_caught")"
TOTAL_CM="$(cat "${WORK_TMP}/controls_missed")"
END_MS="$(now_ms)"
TOTAL_WALL_MS=$((END_MS - SCRIPT_START_MS))

EXIT_CODE=0
if [ "${TOTAL_FAILED}" -gt 0 ] || [ "${TOTAL_CM}" -gt 0 ]; then
  EXIT_CODE=1
elif [ "${TOTAL_PASSED}" -eq 0 ]; then
  EXIT_CODE=2
fi

BODY_SHA256="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest())' "${LOG_FILE}")"
SUMMARY_EXTRA="$(python3 - "${TOTAL_PASSED}" "${TOTAL_FAILED}" "${TOTAL_SKIPPED}" "${TOTAL_CC}" "${TOTAL_CM}" "${EXIT_CODE}" "${TOTAL_WALL_MS}" "${BODY_SHA256}" <<'PYEOF'
import json, sys
p, f, sk, cc, cm, ex, wall, sha = sys.argv[1:9]
print(json.dumps({
    "asserts_passed": int(p),
    "asserts_failed": int(f),
    "asserts_skipped": int(sk),
    "controls_caught": int(cc),
    "controls_missed": int(cm),
    "gates": ["INT-TOOLCHAIN-01 (provisional T only)"],
    "exit": int(ex),
    "wall_ms": int(wall),
    "body_sha256": sha,
}))
PYEOF
)"
emit_json_line "" "summary" "${SUMMARY_EXTRA}"

exit "${EXIT_CODE}"
