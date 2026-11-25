#!/usr/bin/env bash
# scripts/admission/toolchain_matrix.sh
#
# Admission toolchain matrix harness (bead N33c-01, gate INT-TOOLCHAIN-01).
# Runs six build sets across four Rust nightly channels (19 run cells, 5 not-applicable),
# appends one canonical JSON record per cell (schema: frob.toolchain-matrix/1) to
# receipts/matrix/<utc>.ndjson, writes receipts/matrix/<utc>.summary.json, prints
# the matrix table, and logs execution through scripts/e2e/lib.sh (STD-E2E).
#
# Read-only toward sibling repositories: never runs cargo inside a sibling checkout,
# never edits any sibling file, and never deletes files or directories.
# Build-identity settings are inherited strictly from repo-root .cargo/config.toml.

set -euo pipefail

# ===========================================================================
# Matrix channel table (ordered left-to-right).
# Update this table after a toolchain train (NF-02); summary.json names them.
# ===========================================================================
MATRIX_CHANNELS=(
  "nightly-2026-07-06"
  "nightly-2026-08-20"
  "nightly-2026-08-25"
  "nightly-2026-08-31"
)

# ===========================================================================
# Internal cell executor mode (invoked via e2e_cmd so lib.sh records each cell).
# Dispatched BEFORE sourcing scripts/e2e/lib.sh so it is lightweight and never
# recurses into e2e_init.
# ===========================================================================
if [ "${1:-}" = "--_exec-cell" ]; then
  shift
  exec python3 - "$@" <<'PY_EXEC_CELL'
import json
import os
import signal
import subprocess
import sys
import threading
import time

EXPECTED_SHIM_EXITS = {0, 1, 2, 101, 102, 103, 124, 134, 137, 139, 143}


def parse_args(argv):
    timeout_s = 3600
    mem_limit_bytes = 16 * 1024 * 1024 * 1024
    meta_out = ""
    output_out = ""
    i = 0
    cmd = []
    while i < len(argv):
        tok = argv[i]
        if tok == "--":
            cmd = argv[i + 1 :]
            break
        elif tok == "--timeout-s":
            timeout_s = int(argv[i + 1])
            i += 2
        elif tok == "--mem-limit-bytes":
            mem_limit_bytes = int(argv[i + 1])
            i += 2
        elif tok == "--meta-out":
            meta_out = argv[i + 1]
            i += 2
        elif tok == "--output-out":
            output_out = argv[i + 1]
            i += 2
        else:
            sys.stderr.write(f"toolchain_matrix --_exec-cell: unknown arg: {tok}\n")
            sys.exit(2)
    if not cmd or not meta_out or not output_out:
        sys.stderr.write("toolchain_matrix --_exec-cell: missing required arguments\n")
        sys.exit(2)
    return timeout_s, mem_limit_bytes, meta_out, output_out, cmd


def group_rss_bytes(pgid: int) -> int:
    """Return sum of RSS in bytes for all processes in process group `pgid`."""
    try:
        proc = subprocess.run(
            ["ps", "-o", "pgid=,rss=", "-ax"],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        if proc.returncode != 0:
            return 0
        total_kib = 0
        for line in proc.stdout.decode("ascii", "replace").splitlines():
            parts = line.split()
            if len(parts) >= 2:
                try:
                    p_pgid = int(parts[0])
                    p_rss = int(parts[1])
                except ValueError:
                    continue
                if p_pgid == pgid:
                    total_kib += p_rss
        return total_kib * 1024
    except Exception:
        return 0


def kill_pgroup(pgid: int) -> None:
    try:
        os.killpg(pgid, signal.SIGTERM)
    except ProcessLookupError:
        return
    except Exception:
        pass
    deadline = time.monotonic() + 1.5
    while time.monotonic() < deadline:
        try:
            os.killpg(pgid, 0)
        except ProcessLookupError:
            return
        except Exception:
            break
        time.sleep(0.05)
    try:
        os.killpg(pgid, signal.SIGKILL)
    except Exception:
        pass


def waitstatus_to_code(status: int) -> int:
    if hasattr(os, "waitstatus_to_exitcode"):
        code = os.waitstatus_to_exitcode(status)
        return (128 - code) if code < 0 else code
    if os.WIFEXITED(status):
        return os.WEXITSTATUS(status)
    if os.WIFSIGNALED(status):
        return 128 + os.WTERMSIG(status)
    return 1


def main() -> int:
    timeout_s, mem_limit_bytes, meta_out, output_out, cmd = parse_args(sys.argv[1:])
    os.makedirs(os.path.dirname(os.path.abspath(output_out)), exist_ok=True)
    os.makedirs(os.path.dirname(os.path.abspath(meta_out)), exist_ok=True)

    t0 = time.monotonic()
    with open(output_out, "wb") as out_f:
        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            start_new_session=True,
        )
        pgid = proc.pid

        def drain() -> None:
            assert proc.stdout is not None
            while True:
                chunk = proc.stdout.read(65536)
                if not chunk:
                    break
                out_f.write(chunk)
                out_f.flush()

        reader = threading.Thread(target=drain, daemon=True)
        reader.start()

        timed_out = False
        oom_killed = False
        peak_group_rss = 0
        last_rss_check = 0.0
        raw_status = None
        rusage = None

        while True:
            try:
                wpid, wstatus, wrusage = os.wait4(proc.pid, os.WNOHANG)
            except ChildProcessError:
                break
            if wpid == proc.pid:
                raw_status = wstatus
                rusage = wrusage
                break

            now = time.monotonic()
            elapsed = now - t0

            if now - last_rss_check >= 0.25:
                last_rss_check = now
                cur_rss = group_rss_bytes(pgid)
                if cur_rss > peak_group_rss:
                    peak_group_rss = cur_rss
                if mem_limit_bytes > 0 and cur_rss > mem_limit_bytes:
                    oom_killed = True
                    kill_pgroup(pgid)
                    break

            if timeout_s > 0 and elapsed >= timeout_s:
                timed_out = True
                kill_pgroup(pgid)
                break

            time.sleep(0.05)

        if raw_status is None:
            try:
                _, raw_status, rusage = os.wait4(proc.pid, 0)
            except ChildProcessError:
                raw_status = 0

        # Ensure any lingering members of the process group are stopped.
        if timed_out or oom_killed:
            kill_pgroup(pgid)

        reader.join(timeout=5.0)
        elapsed_total = max(0.0, time.monotonic() - t0)

    wait4_rss_bytes = 0
    if rusage is not None:
        raw_rss = int(getattr(rusage, "ru_maxrss", 0))
        if sys.platform == "darwin":
            wait4_rss_bytes = raw_rss
        else:
            wait4_rss_bytes = raw_rss * 1024

    peak_rss_bytes = max(wait4_rss_bytes, peak_group_rss)
    raw_exit = waitstatus_to_code(raw_status) if raw_status is not None else 1

    if timed_out:
        effective_exit = 124
    elif oom_killed:
        effective_exit = 137
    else:
        effective_exit = raw_exit

    meta = {
        "argv": cmd,
        "exit": effective_exit,
        "oom_killed": oom_killed,
        "peak_rss_bytes": peak_rss_bytes,
        "raw_exit": raw_exit,
        "timed_out": timed_out,
        "wall_s": round(elapsed_total, 3),
    }
    with open(meta_out, "w", encoding="utf-8") as mf:
        json.dump(meta, mf, sort_keys=True)
        mf.write("\n")

    if effective_exit in EXPECTED_SHIM_EXITS:
        return effective_exit
    return 101


sys.exit(main())
PY_EXEC_CELL
fi

# ===========================================================================
# Normal harness CLI parsing (before sourcing lib.sh so --help / bad usage
# exit cleanly without initializing E2E logs).
# ===========================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
SELF_SCRIPT="${SCRIPT_DIR}/$(basename "${BASH_SOURCE[0]}")"
DEFAULT_REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd -P)"

usage() {
  cat <<'EOF'
Usage: scripts/admission/toolchain_matrix.sh [OPTIONS]

Run the admission toolchain matrix (six build sets x four nightlies) and write
canonical NDJSON cell records (frob.toolchain-matrix/1) plus summary JSON.

Options:
  --sets <csv>            Comma-separated set IDs (1..6) or names (default: 1,2,3,4,5,6)
  --channels <csv>        Comma-separated rustup channels (default: matrix channel table)
  --test-channel <c>      Also run `cargo +<c> test --locked -j 8` for sets 3..6 on channel <c>
  --timeout-s <n>         Per-cell timeout in seconds (default: 3600)
  --jobs <n>              Cargo parallel jobs per cell, 1..8 (default: 8)
  --mem-limit-gib <n>     Per-cell memory limit in GiB (default: 16)
  --min-free-gib <n>      Minimum free GiB required on target filesystem (default: 2)
  --force                 Re-run cells even if a prior pass/fail record exists for the same receipt
  --root <dir>            Workspace root directory (default: repository root)
  --out-dir <dir>         Output directory for matrix NDJSON and summary (default: <root>/receipts/matrix)
  --target-base <dir>     Override CARGO_TARGET_DIR base (default: ${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-matrix)
  --receipt <hash>        Override stack receipt hash (64-hex BLAKE3; for fixture unit tests)
  --set-dir <id>=<dir>    Override workspace directory for build set <id> (repeatable; for unit tests)
  --date <utc>            Override UTC run stamp (YYYYMMDDTHHMMSSZ)
  -h, --help              Show this help and exit 0
EOF
}

ORIG_ARGS=("$@")
SETS_CSV=""
CHANNELS_CSV=""
TEST_CHANNEL=""
TIMEOUT_S="3600"
JOBS="8"
MEM_LIMIT_GIB="16"
MIN_FREE_GIB="2"
FORCE="0"
ROOT_DIR=""
OUT_DIR=""
TARGET_BASE=""
RECEIPT_OVERRIDE=""
DATE_OVERRIDE=""
SET_DIR_OVERRIDES=()

while [ "$#" -gt 0 ]; do
  case "$1" in
    --sets)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --sets requires a value\n' >&2; exit 2; }
      SETS_CSV="$2"
      shift 2
      ;;
    --channels)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --channels requires a value\n' >&2; exit 2; }
      CHANNELS_CSV="$2"
      shift 2
      ;;
    --test-channel)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --test-channel requires a value\n' >&2; exit 2; }
      TEST_CHANNEL="$2"
      shift 2
      ;;
    --timeout-s)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --timeout-s requires a value\n' >&2; exit 2; }
      TIMEOUT_S="$2"
      shift 2
      ;;
    --jobs)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --jobs requires a value\n' >&2; exit 2; }
      JOBS="$2"
      shift 2
      ;;
    --mem-limit-gib)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --mem-limit-gib requires a value\n' >&2; exit 2; }
      MEM_LIMIT_GIB="$2"
      shift 2
      ;;
    --min-free-gib)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --min-free-gib requires a value\n' >&2; exit 2; }
      MIN_FREE_GIB="$2"
      shift 2
      ;;
    --force)
      FORCE="1"
      shift
      ;;
    --root)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --root requires a value\n' >&2; exit 2; }
      ROOT_DIR="$2"
      shift 2
      ;;
    --out-dir)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --out-dir requires a value\n' >&2; exit 2; }
      OUT_DIR="$2"
      shift 2
      ;;
    --target-base)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --target-base requires a value\n' >&2; exit 2; }
      TARGET_BASE="$2"
      shift 2
      ;;
    --receipt)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --receipt requires a value\n' >&2; exit 2; }
      RECEIPT_OVERRIDE="$2"
      shift 2
      ;;
    --set-dir)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --set-dir requires <id>=<dir>\n' >&2; exit 2; }
      SET_DIR_OVERRIDES=(${SET_DIR_OVERRIDES[@]+"${SET_DIR_OVERRIDES[@]}"} "$2")
      shift 2
      ;;
    --date)
      [ "$#" -ge 2 ] || { printf 'toolchain_matrix.sh: --date requires a value\n' >&2; exit 2; }
      DATE_OVERRIDE="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'toolchain_matrix.sh: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

# Validate numeric flags early.
case "$TIMEOUT_S" in
  ''|*[!0-9]*|0)
    printf 'toolchain_matrix.sh: --timeout-s must be a positive integer, got: %s\n' "$TIMEOUT_S" >&2
    exit 2
    ;;
esac
case "$JOBS" in
  ''|*[!0-9]*|0)
    printf 'toolchain_matrix.sh: --jobs must be an integer in 1..8, got: %s\n' "$JOBS" >&2
    exit 2
    ;;
esac
if [ "$JOBS" -gt 8 ]; then
  printf 'toolchain_matrix.sh: --jobs exceeds 8-core envelope: %s\n' "$JOBS" >&2
  exit 2
fi
case "$MEM_LIMIT_GIB" in
  ''|*[!0-9]*|0)
    printf 'toolchain_matrix.sh: --mem-limit-gib must be a positive integer, got: %s\n' "$MEM_LIMIT_GIB" >&2
    exit 2
    ;;
esac
case "$MIN_FREE_GIB" in
  ''|*[!0-9]*)
    printf 'toolchain_matrix.sh: --min-free-gib must be a non-negative integer, got: %s\n' "$MIN_FREE_GIB" >&2
    exit 2
    ;;
esac

# Resolve paths to absolute paths BEFORE sourcing lib.sh (e2e_init changes cwd).
resolve_abs_path() {
  python3 - "$1" <<'PY'
import os, sys
print(os.path.abspath(sys.argv[1]))
PY
}

if [ -z "$ROOT_DIR" ]; then
  ROOT_DIR="$DEFAULT_REPO_ROOT"
else
  if [ ! -d "$ROOT_DIR" ]; then
    printf 'toolchain_matrix.sh: --root directory does not exist: %s\n' "$ROOT_DIR" >&2
    exit 2
  fi
  ROOT_DIR="$(cd "$ROOT_DIR" && pwd -P)"
fi

if [ -z "$OUT_DIR" ]; then
  OUT_DIR="${ROOT_DIR}/receipts/matrix"
else
  OUT_DIR="$(resolve_abs_path "$OUT_DIR")"
fi

if [ -z "$TARGET_BASE" ]; then
  TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-matrix"
else
  TARGET_BASE="$(resolve_abs_path "$TARGET_BASE")"
fi

# Normalize --set-dir overrides to absolute paths while still in caller cwd.
ABS_SET_DIR_OVERRIDES=()
if [ "${#SET_DIR_OVERRIDES[@]}" -gt 0 ]; then
  for ov in "${SET_DIR_OVERRIDES[@]}"; do
    case "$ov" in
      *=*)
        ov_k="${ov%%=*}"
        ov_v="${ov#*=}"
        ov_abs="$(resolve_abs_path "$ov_v")"
        ABS_SET_DIR_OVERRIDES=(${ABS_SET_DIR_OVERRIDES[@]+"${ABS_SET_DIR_OVERRIDES[@]}"} "${ov_k}=${ov_abs}")
        ;;
      *)
        printf 'toolchain_matrix.sh: --set-dir must be <id>=<dir>, got: %s\n' "$ov" >&2
        exit 2
        ;;
    esac
  done
fi

# Resolve required host utilities.
B3SUM_BIN=""
if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf 'toolchain_matrix.sh: b3sum not found on PATH or at /opt/homebrew/bin/b3sum\n' >&2
  exit 2
fi

for req_bin in python3 cargo rustc git; do
  if ! command -v "$req_bin" >/dev/null 2>&1; then
    printf 'toolchain_matrix.sh: required tool not found on PATH: %s\n' "$req_bin" >&2
    exit 2
  fi
done

MEM_LIMIT_BYTES=$((MEM_LIMIT_GIB * 1024 * 1024 * 1024))
FIRST_RECEIPT_SCRIPT="${DEFAULT_REPO_ROOT}/scripts/bootstrap/first_receipt.sh"

# Build default channels CSV from MATRIX_CHANNELS table.
DEFAULT_CHANNELS_CSV=""
for ch in "${MATRIX_CHANNELS[@]}"; do
  if [ -z "$DEFAULT_CHANNELS_CSV" ]; then
    DEFAULT_CHANNELS_CSV="$ch"
  else
    DEFAULT_CHANNELS_CSV="${DEFAULT_CHANNELS_CSV},${ch}"
  fi
done

# ===========================================================================
# Initialize STD-E2E harness logging (scripts/e2e/lib.sh).
# ===========================================================================
# shellcheck source=scripts/e2e/lib.sh
source "${DEFAULT_REPO_ROOT}/scripts/e2e/lib.sh"
e2e_init "toolchain_matrix" "N33c-01" ${ORIG_ARGS[@]+"${ORIG_ARGS[@]}"}

# ===========================================================================
# Step 1: Preflight — plan matrix, resolve stack receipt & sibling drift,
# check rustc versions, scan resumption records, and run disk preflight.
# ===========================================================================
e2e_step "preflight" "Stack check, toolchain versions, resumption scan, and disk preflight"

PLAN_JSON="${E2E_META_DIR}/matrix_plan.json"
RUN_STATE_JSON="${E2E_META_DIR}/matrix_state.json"

python3 - \
  "$PLAN_JSON" \
  "$ROOT_DIR" \
  "$OUT_DIR" \
  "$TARGET_BASE" \
  "$DEFAULT_CHANNELS_CSV" \
  "$SETS_CSV" \
  "$CHANNELS_CSV" \
  "$TEST_CHANNEL" \
  "$FORCE" \
  "$RECEIPT_OVERRIDE" \
  "$MIN_FREE_GIB" \
  "$DATE_OVERRIDE" \
  "$B3SUM_BIN" \
  "$FIRST_RECEIPT_SCRIPT" \
  ${ABS_SET_DIR_OVERRIDES[@]+"${ABS_SET_DIR_OVERRIDES[@]}"} <<'PY_PLAN'
import datetime
import json
import os
import platform
import re
import shutil
import stat
import struct
import subprocess
import sys
import tomllib
import unicodedata

(
    plan_json_path,
    root_dir,
    out_dir,
    target_base,
    default_channels_csv,
    sets_csv,
    channels_csv,
    test_channel,
    force_str,
    receipt_override,
    min_free_gib_str,
    date_override,
    b3sum_bin,
    first_receipt_script,
) = sys.argv[1:15]
set_dir_overrides_raw = sys.argv[15:]

HEX40_RE = re.compile(r"^[0-9a-f]{40}$")
HEX64_RE = re.compile(r"^[0-9a-f]{64}$")
DRIFT_REASON_ORDER = ("dirty", "retreated", "diverged", "unknown-pin")

DEFAULT_CHANNELS = [c for c in default_channels_csv.split(",") if c]
FORCE = force_str == "1"
MIN_FREE_GIB = int(min_free_gib_str)
IS_DARWIN = sys.platform == "darwin"

# Six build sets (id, name, label, default_rel_workspace, default_rel_marker, missing_bead, applicable_channels).
# When MATRIX_CHANNELS is updated by a toolchain train, channels outside the table's
# historic inapplicable entries default to applicable for new columns.
BUILD_SETS = [
    {
        "id": 1,
        "name": "fs-qty",
        "label": "1. fs-qty alone",
        "workspace": "tests/admission/toolchain/set1_fs_qty",
        "marker": "tests/admission/toolchain/set1_fs_qty/Cargo.toml",
        "missing_bead": "N33c-01",
        "inapplicable": set(),
    },
    {
        "id": 2,
        "name": "bedrock-direct",
        "label": "2. Bedrock-direct list",
        "workspace": "tests/admission/toolchain/set2_bedrock",
        "marker": "tests/admission/toolchain/set2_bedrock/Cargo.toml",
        "missing_bead": "N33c-01",
        "inapplicable": set(),
    },
    {
        "id": 3,
        "name": "m1-closure",
        "label": "3. M1 closure",
        "workspace": ".",
        "marker": "crates/frob-fsim/Cargo.toml",
        "missing_bead": "BD-006",
        "inapplicable": {"nightly-2026-08-20"},
    },
    {
        "id": 4,
        "name": "metal-annex",
        "label": "4. Metal annex",
        "workspace": "spark/frob-spark-metal",
        "marker": "spark/frob-spark-metal/Cargo.toml",
        "missing_bead": "BD-016",
        "inapplicable": {"nightly-2026-08-20", "nightly-2026-08-25"},
    },
    {
        "id": 5,
        "name": "frob-mcp",
        "label": "5. surface/frob-mcp",
        "workspace": ".",
        "marker": "surface/frob-mcp/Cargo.toml",
        "missing_bead": "BD-023",
        "inapplicable": {"nightly-2026-08-20"},
    },
    {
        "id": 6,
        "name": "frob-derive",
        "label": "6. derive/frob-derive",
        "workspace": "derive/frob-derive",
        "marker": "derive/frob-derive/Cargo.toml",
        "missing_bead": "BD-019",
        "inapplicable": {"nightly-2026-08-25"},
    },
]
SET_BY_ID = {s["id"]: s for s in BUILD_SETS}
SET_BY_NAME = {s["name"]: s for s in BUILD_SETS}


def fail(msg: str, code: int = 2) -> None:
    sys.stderr.write(f"toolchain_matrix.sh: {msg}\n")
    sys.exit(code)


def b3sum_bytes(data: bytes) -> str:
    proc = subprocess.run([b3sum_bin, "--no-names"], input=data, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if proc.returncode != 0:
        fail(f"b3sum failed: {proc.stderr.decode('utf-8', 'replace').strip()}")
    d = proc.stdout.decode("ascii").strip()
    if not HEX64_RE.match(d):
        fail(f"unexpected b3sum digest: {d!r}")
    return d


def b3sum_file(path: str) -> str:
    with open(path, "rb") as f:
        return b3sum_bytes(f.read())


def git_ro(repo_dir: str, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    proc = subprocess.run(
        ["git", "--no-optional-locks", "-C", repo_dir, *args],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if check and proc.returncode != 0:
        fail(f"git {' '.join(args)} failed in {os.path.basename(repo_dir)}: {proc.stderr.decode('utf-8', 'replace').strip()}")
    return proc


def is_dirty_tracked(repo_dir: str) -> bool:
    p = git_ro(repo_dir, "status", "--porcelain", "--untracked-files=no", check=False)
    return p.returncode == 0 and len(p.stdout.strip()) > 0


def head_rev(repo_dir: str):
    p = git_ro(repo_dir, "rev-parse", "HEAD", check=False)
    if p.returncode != 0:
        return None
    rev = p.stdout.decode("ascii", "replace").strip()
    return rev if HEX40_RE.match(rev) else None


def classify_checkout(repo_dir: str, pin_rev):
    if is_dirty_tracked(repo_dir):
        return ("drift", "dirty")
    if pin_rev == "unadmitted":
        return ("on-pin", None)
    if not isinstance(pin_rev, str) or not HEX40_RE.match(pin_rev):
        return ("drift", "unknown-pin")
    h = head_rev(repo_dir)
    if not h:
        return ("drift", "unknown-pin")
    if h == pin_rev:
        return ("on-pin", None)
    if git_ro(repo_dir, "cat-file", "-e", f"{pin_rev}^{{commit}}", check=False).returncode != 0:
        return ("drift", "unknown-pin")
    if git_ro(repo_dir, "merge-base", "--is-ancestor", pin_rev, h, check=False).returncode == 0:
        return ("stale-lock", None)
    if git_ro(repo_dir, "merge-base", "--is-ancestor", h, pin_rev, check=False).returncode == 0:
        return ("drift", "retreated")
    return ("drift", "diverged")


def combine_classifications(pairs):
    drift_reasons = [r for c, r in pairs if c == "drift"]
    if drift_reasons:
        for pref in DRIFT_REASON_ORDER:
            if pref in drift_reasons:
                return ("drift", pref)
        return ("drift", drift_reasons[0])
    if any(c == "stale-lock" for c, _ in pairs):
        return ("stale-lock", None)
    return ("on-pin", None)


def inspect_siblings_from_lock(base_dir: str):
    """Read-only git inspection of siblings declared in franken-stack.lock (zero cargo calls)."""
    lock_path = os.path.join(base_dir, "franken-stack.lock")
    if not os.path.isfile(lock_path):
        return False, {}, {}
    with open(lock_path, "rb") as f:
        lock_data = tomllib.load(f)
    siblings_dirty = False
    sibling_drift = {}
    sibling_heads = {}
    for row in lock_data.get("foundation", []):
        if not isinstance(row, dict):
            continue
        fd = row.get("id")
        rel = row.get("path")
        if not isinstance(fd, str) or not fd or not isinstance(rel, str) or not rel:
            continue
        sib_dir = os.path.normpath(os.path.join(base_dir, rel))
        if not os.path.isdir(sib_dir):
            continue
        dirty = is_dirty_tracked(sib_dir)
        rev = head_rev(sib_dir)
        cls, reason = classify_checkout(sib_dir, row.get("git_rev"))
        if dirty:
            siblings_dirty = True
        sibling_drift[fd] = {"class": cls, "dirty": dirty, "reason": reason}
        sibling_heads[fd] = {"dirty": dirty, "rev": rev}
    return siblings_dirty, sibling_drift, sibling_heads


def find_matching_existing_receipt(base_dir: str):
    """Check if an existing receipts/<hash>.json matches the current clean/dirty tree
    and lockfile hashes without invoking cargo (supports zero-cargo resume)."""
    receipts_dir = os.path.join(base_dir, "receipts")
    lock_path = os.path.join(base_dir, "franken-stack.lock")
    core_lock = os.path.join(base_dir, "Cargo.lock")
    tc_path = os.path.join(base_dir, "rust-toolchain.toml")
    if not (os.path.isdir(receipts_dir) and os.path.isfile(lock_path) and os.path.isfile(core_lock) and os.path.isfile(tc_path)):
        return None
    repo_rev = head_rev(base_dir)
    if not repo_rev:
        return None
    repo_dirty = is_dirty_tracked(base_dir)
    if repo_dirty:
        # Dirty root tree requires computing frob.tree/1 via first_receipt.sh for exactness.
        return None
    core_lock_hash = b3sum_file(core_lock)
    with open(tc_path, "rb") as f:
        tc_data = tomllib.load(f)
    tc_chan = tc_data.get("toolchain", {}).get("channel")
    sibs_dirty, sib_drift, sib_heads = inspect_siblings_from_lock(base_dir)
    if sibs_dirty:
        # Dirty sibling tree changes sibling tree_hash; ask first_receipt.sh for exact receipt.
        return None

    candidates = sorted(
        fn for fn in os.listdir(receipts_dir) if fn.endswith(".json") and HEX64_RE.match(fn[:-5])
    )
    for fn in reversed(candidates):
        full = os.path.join(receipts_dir, fn)
        try:
            with open(full, "rb") as f:
                raw = f.read()
            digest = b3sum_bytes(raw)
            if digest != fn[:-5]:
                continue
            obj = json.loads(raw.decode("utf-8"))
            if obj.get("schema") != "frob.stack-receipt/1":
                continue
            if obj.get("repo_rev") != repo_rev or obj.get("repo_dirty") is not False:
                continue
            if obj.get("toolchain") != tc_chan:
                continue
            lock_map = {lf.get("workspace"): lf.get("hash") for lf in obj.get("lockfiles", []) if isinstance(lf, dict)}
            # Verify all lockfiles present in the receipt match current bytes on disk.
            locks_ok = True
            ws_paths = {
                "core": ".",
                "apps": "apps",
                "bindings": "bindings/frob-gymnasium",
                "derive": "derive/frob-derive",
                "spark-cuda": "spark/frob-spark-cuda",
                "spark-metal": "spark/frob-spark-metal",
                "worker": "worker",
            }
            for ws_name, ws_rel in ws_paths.items():
                ws_manifest = os.path.join(base_dir, ws_rel, "Cargo.toml")
                ws_lock = os.path.join(base_dir, ws_rel, "Cargo.lock")
                if ws_name == "core" or os.path.isfile(ws_manifest):
                    if not os.path.isfile(ws_lock) or lock_map.get(ws_name) != b3sum_file(ws_lock):
                        locks_ok = False
                        break
            if not locks_ok:
                continue
            # Verify all siblings match rev, clean state, and drift class.
            sib_ok = True
            rec_sibs = obj.get("siblings", [])
            if len(rec_sibs) != len(sib_heads):
                continue
            rec_drift = {}
            for rs in rec_sibs:
                fd = rs.get("fd")
                cur = sib_heads.get(fd)
                cur_d = sib_drift.get(fd)
                if not cur or not cur_d or rs.get("rev") != cur["rev"] or bool(rs.get("dirty")) != cur["dirty"]:
                    sib_ok = False
                    break
                ol = rs.get("own_lock") or {}
                if ol.get("class") != cur_d["class"] or ol.get("reason") != cur_d["reason"]:
                    sib_ok = False
                    break
                rec_drift[fd] = {
                    "class": ol.get("class", "on-pin"),
                    "dirty": bool(rs.get("dirty")),
                    "reason": ol.get("reason"),
                }
            if sib_ok:
                return digest, False, rec_drift
        except Exception:
            continue
    return None


def compute_stack_state(base_dir: str, override: str):
    if override:
        if not HEX64_RE.match(override):
            fail(f"--receipt must be 64 lowercase hex characters, got: {override!r}")
        sibs_dirty, sib_drift, _ = inspect_siblings_from_lock(base_dir)
        return override, sibs_dirty, sib_drift, "override"

    matched = find_matching_existing_receipt(base_dir)
    if matched is not None:
        digest, sibs_dirty, sib_drift = matched
        return digest, sibs_dirty, sib_drift, "cached-receipt"

    if not os.path.isfile(first_receipt_script):
        fail(f"stack check script missing: {first_receipt_script}")

    proc = subprocess.run(
        [first_receipt_script, "--stdout", "--root", base_dir],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        fail(
            f"stack check (first_receipt.sh --stdout) failed (exit {proc.returncode}): "
            f"{proc.stderr.decode('utf-8', 'replace').strip()}"
        )
    raw = proc.stdout
    digest = b3sum_bytes(raw)
    obj = json.loads(raw.decode("utf-8"))
    sib_drift = {}
    sibs_dirty = False
    for rs in obj.get("siblings", []):
        fd = rs.get("fd")
        if not fd:
            continue
        dirty = bool(rs.get("dirty"))
        if dirty:
            sibs_dirty = True
        ol = rs.get("own_lock") or {}
        sib_drift[fd] = {
            "class": ol.get("class", "on-pin"),
            "dirty": dirty,
            "reason": ol.get("reason"),
        }
    return digest, sibs_dirty, sib_drift, "first_receipt"


# Parse --sets selection.
if sets_csv:
    selected_set_ids = []
    for tok in sets_csv.split(","):
        tok = tok.strip()
        if not tok:
            continue
        if tok.isdigit() and int(tok) in SET_BY_ID:
            sid = int(tok)
        elif tok in SET_BY_NAME:
            sid = SET_BY_NAME[tok]["id"]
        else:
            fail(f"unknown build set in --sets: {tok!r} (valid: 1..6 or set names)")
        if sid not in selected_set_ids:
            selected_set_ids.append(sid)
    if not selected_set_ids:
        fail("--sets selected no build sets")
else:
    selected_set_ids = [s["id"] for s in BUILD_SETS]

# Parse --channels selection.
if channels_csv:
    selected_channels = []
    for tok in channels_csv.split(","):
        tok = tok.strip()
        if tok and tok not in selected_channels:
            selected_channels.append(tok)
    if not selected_channels:
        fail("--channels selected no channels")
else:
    selected_channels = list(DEFAULT_CHANNELS)

# Parse --set-dir overrides.
set_dir_overrides = {}
for item in set_dir_overrides_raw:
    k, v = item.split("=", 1)
    k = k.strip()
    if k.isdigit() and int(k) in SET_BY_ID:
        sid = int(k)
    elif k in SET_BY_NAME:
        sid = SET_BY_NAME[k]["id"]
    else:
        fail(f"unknown build set in --set-dir: {k!r}")
    set_dir_overrides[sid] = v

# Query rustc +<c> -V once per selected channel.
rustc_versions = {}
for ch in selected_channels:
    proc = subprocess.run(
        ["rustc", f"+{ch}", "-V"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        err = proc.stderr.decode("utf-8", "replace").strip()
        fail(
            f"channel '{ch}' is not available (`rustc +{ch} -V` exited {proc.returncode}: {err}); "
            f"install it via rustup"
        )
    ver = proc.stdout.decode("utf-8", "replace").strip().splitlines()[0]
    rustc_versions[ch] = ver

# Compute receipt & sibling drift state.
receipt_hash, siblings_dirty, sibling_drift, receipt_source = compute_stack_state(root_dir, receipt_override)

# Scan existing receipts/matrix/*.ndjson for resumption keyed by (set, channel, receipt).
# `pass`, `fail`, and `not-applicable` are resumable; `not-run` is ALWAYS retried.
prior_done = {}
prior_na = {}
if os.path.isdir(out_dir):
    ndjson_files = sorted(fn for fn in os.listdir(out_dir) if fn.endswith(".ndjson"))
    for fn in ndjson_files:
        fpath = os.path.join(out_dir, fn)
        if not os.path.isfile(fpath):
            continue
        try:
            with open(fpath, "r", encoding="utf-8") as fh:
                for raw_line in fh:
                    raw_line = raw_line.strip()
                    if not raw_line:
                        continue
                    rec = json.loads(raw_line)
                    if rec.get("schema") != "frob.toolchain-matrix/1":
                        continue
                    r_rec = rec.get("receipt")
                    r_set = rec.get("set")
                    r_ch = rec.get("channel")
                    r_st = rec.get("status")
                    if r_rec == receipt_hash and isinstance(r_set, int) and isinstance(r_ch, str):
                        if r_st in ("pass", "fail"):
                            prior_done[f"{r_set}|{r_ch}"] = rec
                        elif r_st == "not-applicable":
                            prior_na[f"{r_set}|{r_ch}"] = rec
        except Exception as exc:
            sys.stderr.write(f"toolchain_matrix.sh: warning: skipping unreadable prior log {fn}: {exc}\n")

# Build execution plan for every selected (set, channel).
cells = []
need_cargo_exec = False
for sid in selected_set_ids:
    sdef = SET_BY_ID[sid]
    if sid in set_dir_overrides:
        ws_dir = set_dir_overrides[sid]
        marker_path = os.path.join(ws_dir, "Cargo.toml")
        target_exists = os.path.isfile(marker_path)
    else:
        marker_path = os.path.join(root_dir, sdef["marker"])
        target_exists = os.path.isfile(marker_path)
        ws_dir = os.path.normpath(os.path.join(root_dir, sdef["workspace"]))
        # Set 5 (surface/frob-mcp): if it declares its own [workspace], run inside it; else core workspace.
        if sid == 5 and target_exists:
            try:
                with open(marker_path, "rb") as mf:
                    mdata = tomllib.load(mf)
                if "workspace" in mdata:
                    ws_dir = os.path.dirname(marker_path)
            except Exception:
                pass

    for ch in selected_channels:
        key = f"{sid}|{ch}"
        applicable = ch not in sdef["inapplicable"]
        # Set 4 (Metal annex) is macOS-only when target exists on non-macOS hosts.
        if sid == 4 and target_exists and not IS_DARWIN:
            applicable = False

        if not applicable:
            if not FORCE and key in prior_na:
                cells.append(
                    {
                        "action": "resume",
                        "channel": ch,
                        "missing_bead": None,
                        "prior_record": prior_na[key],
                        "run_tests": False,
                        "set": sid,
                        "set_name": sdef["name"],
                        "ws_dir": ws_dir,
                    }
                )
            else:
                cells.append(
                    {
                        "action": "not-applicable",
                        "channel": ch,
                        "missing_bead": None,
                        "prior_record": None,
                        "run_tests": False,
                        "set": sid,
                        "set_name": sdef["name"],
                        "ws_dir": ws_dir,
                    }
                )
            continue

        if not FORCE and key in prior_done:
            cells.append(
                {
                    "action": "resume",
                    "channel": ch,
                    "missing_bead": None,
                    "prior_record": prior_done[key],
                    "run_tests": False,
                    "set": sid,
                    "set_name": sdef["name"],
                    "ws_dir": ws_dir,
                }
            )
            continue

        if not target_exists:
            cells.append(
                {
                    "action": "not-run",
                    "channel": ch,
                    "missing_bead": sdef["missing_bead"],
                    "prior_record": None,
                    "run_tests": False,
                    "set": sid,
                    "set_name": sdef["name"],
                    "ws_dir": ws_dir,
                }
            )
            continue

        need_cargo_exec = True
        run_tests = bool(test_channel and ch == test_channel and sid in (3, 4, 5, 6))
        cells.append(
            {
                "action": "run",
                "channel": ch,
                "missing_bead": None,
                "prior_record": None,
                "run_tests": run_tests,
                "set": sid,
                "set_name": sdef["name"],
                "ws_dir": ws_dir,
            }
        )

# Disk preflight before running any cargo build cells.
os.makedirs(out_dir, exist_ok=True)
if need_cargo_exec:
    os.makedirs(target_base, exist_ok=True)
    usage = shutil.disk_usage(target_base)
    free_gib = usage.free / (1024 ** 3)
    if MIN_FREE_GIB > 0 and free_gib < MIN_FREE_GIB:
        fail(
            f"disk preflight failed: only {free_gib:.2f} GiB free on target filesystem "
            f"(minimum required: {MIN_FREE_GIB} GiB)"
        )

# Allocate unique UTC run stamp under out_dir (never overwrites existing runs).
base_stamp = date_override or datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
stamp = base_stamp
suffix = 0
while (
    os.path.exists(os.path.join(out_dir, f"{stamp}.ndjson"))
    or os.path.exists(os.path.join(out_dir, f"{stamp}.summary.json"))
):
    suffix += 1
    stamp = f"{base_stamp}-{suffix:02d}"

plan = {
    "cells": cells,
    "channels": selected_channels,
    "ndjson_path": os.path.join(out_dir, f"{stamp}.ndjson"),
    "out_dir": out_dir,
    "receipt": receipt_hash,
    "receipt_source": receipt_source,
    "root_dir": root_dir,
    "rustc_versions": rustc_versions,
    "selected_sets": selected_set_ids,
    "set_labels": {str(s["id"]): s["label"] for s in BUILD_SETS},
    "set_names": {str(s["id"]): s["name"] for s in BUILD_SETS},
    "sibling_drift": sibling_drift,
    "siblings_dirty": siblings_dirty,
    "stamp": stamp,
    "summary_path": os.path.join(out_dir, f"{stamp}.summary.json"),
    "target_base": target_base,
    "test_channel": test_channel,
}
with open(plan_json_path, "w", encoding="utf-8") as pf:
    json.dump(plan, pf, sort_keys=True)
    pf.write("\n")
PY_PLAN

# Read cell count & plan metadata and emit TSV rows to loop cleanly in bash 3.2.
PLAN_ROWS_FILE="${E2E_META_DIR}/matrix_plan_rows.tsv"
python3 - "$PLAN_JSON" "$PLAN_ROWS_FILE" <<'PY_ROWS'
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    plan = json.load(f)
with open(sys.argv[2], "w", encoding="utf-8") as out:
    for idx, c in enumerate(plan["cells"]):
        out.write(
            f"{idx}\t{c['action']}\t{c['set']}\t{c['set_name']}\t{c['channel']}\t"
            f"{c['ws_dir']}\t{1 if c['run_tests'] else 0}\t{c['missing_bead'] or ''}\n"
        )
PY_ROWS

NDJSON_PATH="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["ndjson_path"])' "$PLAN_JSON")"
SUMMARY_PATH="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["summary_path"])' "$PLAN_JSON")"
RUN_STAMP="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["stamp"])' "$PLAN_JSON")"

# Helper: run python record builder/appender for one cell and return key=value metrics.
record_cell() {
  python3 - \
    "$PLAN_JSON" \
    "$RUN_STATE_JSON" \
    "$B3SUM_BIN" \
    "$FIRST_RECEIPT_SCRIPT" \
    "$RECEIPT_OVERRIDE" \
    "$TIMEOUT_S" \
    "$@" <<'PY_RECORD'
import json
import math
import os
import re
import subprocess
import sys
import unicodedata

(
    plan_json_path,
    state_json_path,
    b3sum_bin,
    first_receipt_script,
    receipt_override,
    timeout_s_str,
    cell_idx_str,
    check_meta_path,
    check_out_path,
    test_meta_path,
    test_out_path,
) = sys.argv[1:12]

TIMEOUT_S = int(timeout_s_str)
CELL_IDX = int(cell_idx_str)
INT53_MAX = (1 << 53) - 1
HEX64_RE = re.compile(r"^[0-9a-f]{64}$")
ANSI_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")


def nfc(s: str) -> str:
    return unicodedata.normalize("NFC", s)


def canon_str(s: str) -> str:
    s = nfc(s)
    out = ['"']
    for ch in s:
        cp = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif cp < 0x20:
            out.append(f"\\u{cp:04x}")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def canon_json(val) -> str:
    """N34-18 canonical JSON: sorted keys, NFC strings, minimal escapes, no whitespace."""
    if val is None:
        return "null"
    if isinstance(val, bool):
        return "true" if val else "false"
    if isinstance(val, int):
        if abs(val) > INT53_MAX:
            raise ValueError(f"integer out of int53 range: {val}")
        return str(val)
    if isinstance(val, float):
        if not math.isfinite(val):
            raise ValueError(f"non-finite float in canonical JSON: {val}")
        if val == 0.0:
            val = 0.0
        return json.dumps(val, allow_nan=False)
    if isinstance(val, str):
        return canon_str(val)
    if isinstance(val, list):
        return "[" + ",".join(canon_json(item) for item in val) + "]"
    if isinstance(val, dict):
        norm = {}
        for k, v in val.items():
            nk = nfc(k)
            norm[nk] = v
        return "{" + ",".join(f"{canon_str(k)}:{canon_json(norm[k])}" for k in sorted(norm.keys())) + "}"
    raise TypeError(f"unsupported canonical JSON type: {type(val).__name__}")


def redact_machine_paths(s: str) -> str:
    home = os.path.expanduser("~")
    if home and len(home) > 1 and home in s:
        s = s.replace(home, "~")
    return s


def truncate_utf8_bytes(s: str, max_bytes: int = 400) -> str:
    s = nfc(redact_machine_paths(s.replace("\r", " ").replace("\n", " ").strip()))
    b = s.encode("utf-8")
    if len(b) <= max_bytes:
        return s
    cut = b[:max_bytes]
    while cut:
        try:
            return cut.decode("utf-8")
        except UnicodeDecodeError:
            cut = cut[:-1]
    return ""


def b3sum_file(path: str) -> str:
    with open(path, "rb") as f:
        proc = subprocess.run([b3sum_bin, "--no-names"], input=f.read(), stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if proc.returncode != 0:
        raise RuntimeError(f"b3sum failed on {path}: {proc.stderr.decode('utf-8', 'replace')}")
    d = proc.stdout.decode("ascii").strip()
    if not HEX64_RE.match(d):
        raise RuntimeError(f"unexpected b3sum digest: {d!r}")
    return d


def load_metadata(ws_dir: str, channel: str):
    """Read package count & package directory map via cargo metadata --locked --offline."""
    proc = subprocess.run(
        ["cargo", f"+{channel}", "metadata", "--format-version", "1", "--locked", "--offline"],
        cwd=ws_dir,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        return None, [], None
    try:
        meta = json.loads(proc.stdout.decode("utf-8"))
        pkgs = meta.get("packages", [])
        pkg_dirs = []
        root_pkg = None
        ws_real = os.path.realpath(ws_dir)
        ws_abs = os.path.abspath(ws_dir)
        for p in pkgs:
            name = p.get("name")
            mpath = p.get("manifest_path")
            if isinstance(name, str) and isinstance(mpath, str):
                pdir = os.path.dirname(mpath)
                for variant in {os.path.abspath(pdir), os.path.realpath(pdir)}:
                    pkg_dirs.append((variant, name))
                if os.path.realpath(pdir) == ws_real or os.path.abspath(pdir) == ws_abs:
                    root_pkg = name
        pkg_dirs.sort(key=lambda pair: len(pair[0]), reverse=True)
        return len(pkgs), pkg_dirs, root_pkg
    except Exception:
        return None, [], None


def refresh_sibling_drift_on_failure(root_dir: str, default_dirty: bool, default_drift: dict):
    """Per N33c-01 §5: call stack check before attributing a failure to this repository."""
    if receipt_override or not os.path.isfile(os.path.join(root_dir, "franken-stack.lock")):
        return default_dirty, default_drift
    if not os.path.isfile(first_receipt_script):
        return default_dirty, default_drift
    proc = subprocess.run(
        [first_receipt_script, "--stdout", "--root", root_dir],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        return default_dirty, default_drift
    try:
        obj = json.loads(proc.stdout.decode("utf-8"))
        sib_drift = {}
        sibs_dirty = False
        for rs in obj.get("siblings", []):
            fd = rs.get("fd")
            if not fd:
                continue
            dirty = bool(rs.get("dirty"))
            if dirty:
                sibs_dirty = True
            ol = rs.get("own_lock") or {}
            sib_drift[fd] = {
                "class": ol.get("class", "on-pin"),
                "dirty": dirty,
                "reason": ol.get("reason"),
            }
        return sibs_dirty, sib_drift
    except Exception:
        return default_dirty, default_drift


def match_path_to_crate(file_ref: str, ws_dir: str, pkg_dirs: list):
    candidates = []
    if os.path.isabs(file_ref):
        candidates.append(os.path.abspath(file_ref))
        candidates.append(os.path.realpath(file_ref))
    else:
        joined = os.path.join(ws_dir, file_ref)
        candidates.append(os.path.abspath(joined))
        candidates.append(os.path.realpath(joined))
    for cand in candidates:
        for pdir, name in pkg_dirs:
            if cand == pdir or cand.startswith(pdir + os.sep):
                return name
    return None


def classify_failure(output_text: str, meta: dict, ws_dir: str, pkg_dirs: list, root_pkg):
    clean = ANSI_RE.sub("", output_text)
    lines = clean.splitlines()

    # Collect active crates and `could not compile` crates.
    compile_crates = []
    for ln in lines:
        m = re.match(r"^\s*(?:Compiling|Checking)\s+([A-Za-z0-9_-]+)\s+v", ln)
        if m:
            compile_crates.append(m.group(1))
    failed_compile_crates = []
    for ln in lines:
        m = re.search(r"error:\s+could not compile `([A-Za-z0-9_-]+)`", ln)
        if m and m.group(1) not in failed_compile_crates:
            failed_compile_crates.append(m.group(1))
    last_active = failed_compile_crates[0] if failed_compile_crates else (compile_crates[-1] if compile_crates else root_pkg)

    # 1. Timeout
    if meta.get("timed_out"):
        return "timeout", {
            "code": None,
            "crate": last_active,
            "line": truncate_utf8_bytes(f"cell exceeded timeout of {TIMEOUT_S}s"),
        }

    # 2. Memory limit / OOM
    oom_re = re.compile(
        r"(memory allocation of \d+ bytes failed|LLVM ERROR:\s*out of memory|fatal runtime error:.*out of memory|Cannot allocate memory)",
        re.IGNORECASE,
    )
    oom_match_line = None
    for ln in lines:
        if oom_re.search(ln):
            oom_match_line = ln.strip()
            break
    if meta.get("oom_killed") or meta.get("exit") == 137 or oom_match_line is not None:
        return "memory", {
            "code": None,
            "crate": last_active,
            "line": truncate_utf8_bytes(oom_match_line or "cell exceeded memory limit"),
        }

    # 3. Rust-version (MSRV) mismatch
    for idx, ln in enumerate(lines):
        # Classic cargo: error: package `<crate> v...` cannot be built because it requires rustc <ver>
        m_old = re.search(
            r"error:\s+package `([A-Za-z0-9_-]+)\s+v[^`]*`\s+cannot be built because it requires rustc\s+([^\s,]+)",
            ln,
        )
        if m_old:
            return "rust-version", {
                "code": None,
                "crate": m_old.group(1),
                "line": truncate_utf8_bytes(ln.strip()),
            }
        # Modern cargo: error: rustc <ver> is not supported by the following package(s):
        #   <crate>@<ver> requires rustc <msrv>
        if re.search(r"error:\s+rustc\s+\S+\s+is not supported by the following package", ln):
            crate_name = root_pkg
            detail = ln.strip()
            for nxt in lines[idx + 1 : idx + 6]:
                m_pkg = re.search(r"^\s*([A-Za-z0-9_-]+)@[^\s]+\s+requires rustc\s+([^\s]+)", nxt)
                if m_pkg:
                    crate_name = m_pkg.group(1)
                    detail = f"{detail} {nxt.strip()}"
                    break
            return "rust-version", {
                "code": None,
                "crate": crate_name,
                "line": truncate_utf8_bytes(detail),
            }
        m_req = re.search(r"^\s*([A-Za-z0-9_-]+)@[^\s]+\s+requires rustc\s+([^\s]+)", ln)
        if m_req:
            return "rust-version", {
                "code": None,
                "crate": m_req.group(1),
                "line": truncate_utf8_bytes(ln.strip()),
            }

    # 4. Build script failure
    for idx, ln in enumerate(lines):
        m_bs = re.search(r"error:\s+failed to run custom build command for `([A-Za-z0-9_-]+)\s+v", ln)
        if m_bs:
            bs_crate = m_bs.group(1)
            panic_detail = ""
            for ni, nxt in enumerate(lines[idx + 1 : idx + 40], start=idx + 1):
                s = nxt.strip()
                if s.startswith("thread ") and "panicked" in s:
                    panic_detail = s
                    if ni + 1 < len(lines) and lines[ni + 1].strip() and not lines[ni + 1].strip().startswith("note:"):
                        panic_detail = f"{s}: {lines[ni + 1].strip()}"
                    break
                if s.startswith("cargo:warning=") or s.startswith("error:"):
                    if not panic_detail:
                        panic_detail = s
            msg = f"failed to run custom build command for `{bs_crate}`"
            if panic_detail:
                msg = f"{msg}: {panic_detail}"
            return "build-script", {
                "code": None,
                "crate": bs_crate,
                "line": truncate_utf8_bytes(msg),
            }

    # 5. Cargo resolution / manifest / lockfile failure
    resolve_patterns = [
        (r"error:\s+failed to get `([A-Za-z0-9_-]+)` as a dependency of package `([A-Za-z0-9_-]+)", 1),
        (r"error:\s+failed to load manifest for dependency `([A-Za-z0-9_-]+)`", 1),
        (r"error:\s+failed to load source for dependency `([A-Za-z0-9_-]+)`", 1),
        (r"error:\s+no matching package named `([A-Za-z0-9_-]+)`", 1),
        (r"error:\s+failed to select a version for `([A-Za-z0-9_-]+)`", 1),
        (r"error:\s+the lock file .* needs to be updated but --locked was passed", 0),
        (r"error:\s+failed to parse manifest at", 0),
        (r"error:\s+manifest path .* does not exist", 0),
        (r"error:\s+could not find `Cargo\.toml`", 0),
        (r"error:\s+Unable to update", 0),
        (r"error:\s+cannot find manifest", 0),
        (r"error:\s+failed to load manifest for workspace member", 0),
    ]
    for idx, ln in enumerate(lines):
        for pat, grp in resolve_patterns:
            m_res = re.search(pat, ln)
            if m_res:
                r_crate = m_res.group(grp) if grp > 0 else root_pkg
                detail = ln.strip()
                for nxt in lines[idx + 1 : idx + 6]:
                    ns = nxt.strip()
                    if ns and ns != "Caused by:":
                        detail = f"{detail} ({ns})"
                        break
                return "resolve", {
                    "code": None,
                    "crate": r_crate,
                    "line": truncate_utf8_bytes(detail),
                }

    # 6. Rustc compiler diagnostic
    skip_error_re = re.compile(
        r"^error:\s+(could not compile|aborting due to|build failed|process didn't exit successfully|compilation failed)"
    )
    for idx, ln in enumerate(lines):
        if skip_error_re.match(ln):
            continue
        m_err = re.match(r"^error(?:\[(E[0-9]{4})\])?:\s+(.+)$", ln)
        if not m_err:
            continue
        err_code = m_err.group(1)
        err_line = ln.strip()
        block_lines = lines[idx : min(len(lines), idx + 15)]
        block_text = "\n".join(block_lines)

        err_crate = None
        for bl in block_lines[1:6]:
            m_loc = re.match(r"^\s*-->\s+(.+?):\d+:\d+\s*$", bl)
            if m_loc:
                err_crate = match_path_to_crate(m_loc.group(1).strip(), ws_dir, pkg_dirs)
                break
        if not err_crate:
            if len(failed_compile_crates) == 1:
                err_crate = failed_compile_crates[0]
            elif failed_compile_crates:
                err_crate = failed_compile_crates[0]
            else:
                err_crate = last_active

        if err_crate == "fs-qty" and (
            "generic_const_exprs" in block_text
            or "generic_const_exprs" in clean
            or err_code in ("E0635", "E0557")
            or "incomplete_features" in block_text
        ):
            kind = "feature-regressed"
        else:
            kind = "other"

        return kind, {
            "code": err_code,
            "crate": err_crate,
            "line": truncate_utf8_bytes(err_line),
        }

    # 7. Fallback
    fallback_line = ""
    for ln in lines:
        if ln.strip().startswith("error"):
            fallback_line = ln.strip()
            break
    if not fallback_line:
        for ln in reversed(lines):
            if ln.strip():
                fallback_line = ln.strip()
                break
    return "other", {
        "code": None,
        "crate": last_active,
        "line": truncate_utf8_bytes(fallback_line or f"command exited with code {meta.get('exit')}"),
    }


with open(plan_json_path, "r", encoding="utf-8") as pf:
    plan = json.load(pf)

cell = plan["cells"][CELL_IDX]
sid = cell["set"]
sname = cell["set_name"]
ch = cell["channel"]
action = cell["action"]
ws_dir = cell["ws_dir"]

if os.path.isfile(state_json_path):
    with open(state_json_path, "r", encoding="utf-8") as sf:
        state = json.load(sf)
else:
    state = {"records": {}, "executed": 0, "resumed": 0}

key = f"{sid}|{ch}"

if action == "resume":
    rec = cell["prior_record"]
    state["records"][key] = rec
    state["resumed"] += 1
    with open(state_json_path, "w", encoding="utf-8") as sf:
        json.dump(state, sf, sort_keys=True)
        sf.write("\n")
    print(f"STATUS={rec['status']}")
    print(f"PEAK_RSS={rec.get('peak_rss_bytes') or 0}")
    print(f"PACKAGES={rec.get('packages') or 0}")
    sys.exit(0)

if action in ("not-applicable", "not-run"):
    rec = {
        "argv": [],
        "channel": ch,
        "exit": None,
        "failure_kind": None,
        "first_error": None,
        "missing_bead": cell["missing_bead"] if action == "not-run" else None,
        "output_file": None,
        "output_hash": None,
        "packages": None,
        "peak_rss_bytes": None,
        "receipt": plan["receipt"],
        "rustc_version": plan["rustc_versions"][ch],
        "schema": "frob.toolchain-matrix/1",
        "set": sid,
        "set_name": sname,
        "sibling_drift": plan["sibling_drift"],
        "siblings_dirty": plan["siblings_dirty"],
        "status": action,
        "wall_s": 0.0,
    }
    line = canon_json(rec)
    with open(plan["ndjson_path"], "a", encoding="utf-8") as nf:
        nf.write(line + "\n")
        nf.flush()
        os.fsync(nf.fileno())
    state["records"][key] = rec
    with open(state_json_path, "w", encoding="utf-8") as sf:
        json.dump(state, sf, sort_keys=True)
        sf.write("\n")
    print(f"STATUS={action}")
    print("PEAK_RSS=0")
    print("PACKAGES=0")
    sys.exit(0)

# action == "run"
with open(check_meta_path, "r", encoding="utf-8") as mf:
    check_meta = json.load(mf)

combined_out_path = check_out_path
active_meta = check_meta
total_wall_s = float(check_meta["wall_s"])
peak_rss = int(check_meta["peak_rss_bytes"])
test_argv = None

if test_meta_path and os.path.isfile(test_meta_path):
    with open(test_meta_path, "r", encoding="utf-8") as tmf:
        test_meta = json.load(tmf)
    test_argv = test_meta["argv"]
    total_wall_s = round(total_wall_s + float(test_meta["wall_s"]), 3)
    peak_rss = max(peak_rss, int(test_meta["peak_rss_bytes"]))
    # Append test output to the cell's sidecar log file so one output_hash covers both.
    if test_out_path and os.path.isfile(test_out_path):
        with open(check_out_path, "ab") as dst, open(test_out_path, "rb") as src:
            dst.write(b"\n--- cargo test ---\n")
            while True:
                buf = src.read(65536)
                if not buf:
                    break
                dst.write(buf)
    if check_meta["exit"] == 0:
        active_meta = test_meta

packages_count, pkg_dirs, root_pkg = load_metadata(ws_dir, ch)
output_hash = b3sum_file(combined_out_path)
output_file_rel = os.path.basename(combined_out_path)

passed = (active_meta["exit"] == 0) and (not active_meta.get("timed_out")) and (not active_meta.get("oom_killed"))
sibs_dirty = plan["siblings_dirty"]
sib_drift = plan["sibling_drift"]

if passed:
    status = "pass"
    failure_kind = None
    first_error = None
else:
    status = "fail"
    sibs_dirty, sib_drift = refresh_sibling_drift_on_failure(plan["root_dir"], sibs_dirty, sib_drift)
    with open(combined_out_path, "rb") as of:
        raw_out = of.read().decode("utf-8", "replace")
    failure_kind, first_error = classify_failure(raw_out, active_meta, ws_dir, pkg_dirs, root_pkg)

rec = {
    "argv": active_meta["argv"],
    "channel": ch,
    "exit": int(active_meta["exit"]),
    "failure_kind": failure_kind,
    "first_error": first_error,
    "missing_bead": None,
    "output_file": output_file_rel,
    "output_hash": output_hash,
    "packages": packages_count,
    "peak_rss_bytes": peak_rss,
    "receipt": plan["receipt"],
    "rustc_version": plan["rustc_versions"][ch],
    "schema": "frob.toolchain-matrix/1",
    "set": sid,
    "set_name": sname,
    "sibling_drift": sib_drift,
    "siblings_dirty": sibs_dirty,
    "status": status,
    "wall_s": round(total_wall_s, 3),
}
if test_argv is not None:
    rec["test_argv"] = test_argv

line = canon_json(rec)
with open(plan["ndjson_path"], "a", encoding="utf-8") as nf:
    nf.write(line + "\n")
    nf.flush()
    os.fsync(nf.fileno())

state["records"][key] = rec
state["executed"] += 1
with open(state_json_path, "w", encoding="utf-8") as sf:
    json.dump(state, sf, sort_keys=True)
    sf.write("\n")

print(f"STATUS={status}")
print(f"PEAK_RSS={peak_rss}")
print(f"PACKAGES={packages_count if packages_count is not None else 0}")
PY_RECORD
}

# ===========================================================================
# Step 2: Execute planned cells one at a time (resuming finished cells).
# ===========================================================================
EXPECTED_CELL_EXITS="0,1,2,101,102,103,124,134,137,139,143"

while IFS=$'\t' read -r cell_idx action sid sname ch ws_dir run_tests _missing_bead; do
  if [ "$action" = "resume" ] || [ "$action" = "not-applicable" ] || [ "$action" = "not-run" ]; then
    record_cell "$cell_idx" "" "" "" "" >/dev/null
    continue
  fi

  e2e_step "cell_s${sid}_${ch}" "Build set ${sid} (${sname}) on ${ch}"

  cell_target_dir="${TARGET_BASE}/${sid}/${ch}"
  mkdir -p "$cell_target_dir"

  cell_meta="${E2E_META_DIR}/cell_s${sid}_${ch}.check.meta.json"
  cell_out="${OUT_DIR}/${RUN_STAMP}.set${sid}.${ch}.log"

  prev_pwd="$PWD"
  cd "$ws_dir"
  CARGO_TARGET_DIR="$cell_target_dir" \
    e2e_cmd --name "cell_s${sid}_${ch}" --expect-exit "$EXPECTED_CELL_EXITS" -- \
      "$SELF_SCRIPT" --_exec-cell \
      --timeout-s "$TIMEOUT_S" \
      --mem-limit-bytes "$MEM_LIMIT_BYTES" \
      --meta-out "$cell_meta" \
      --output-out "$cell_out" \
      -- cargo "+${ch}" check --all-targets --locked -j "$JOBS"

  test_meta=""
  test_out=""
  if [ "$run_tests" = "1" ] && [ "${E2E_LAST_EXIT}" = "0" ]; then
    test_meta="${E2E_META_DIR}/cell_s${sid}_${ch}.test.meta.json"
    test_out="${E2E_META_DIR}/cell_s${sid}_${ch}.test.log"
    CARGO_TARGET_DIR="$cell_target_dir" \
      e2e_cmd --name "cell_s${sid}_${ch}_test" --expect-exit "$EXPECTED_CELL_EXITS" -- \
        "$SELF_SCRIPT" --_exec-cell \
        --timeout-s "$TIMEOUT_S" \
        --mem-limit-bytes "$MEM_LIMIT_BYTES" \
        --meta-out "$test_meta" \
        --output-out "$test_out" \
        -- cargo "+${ch}" test --locked -j "$JOBS"
  fi
  cd "$prev_pwd"

  rec_kv="$(record_cell "$cell_idx" "$cell_meta" "$cell_out" "$test_meta" "$test_out")"
  cell_peak_rss="$(printf '%s\n' "$rec_kv" | awk -F= '$1=="PEAK_RSS"{print $2; exit}')"
  cell_packages="$(printf '%s\n' "$rec_kv" | awk -F= '$1=="PACKAGES"{print $2; exit}')"

  e2e_metric "cell_s${sid}_${ch}.peak_rss_bytes" "${cell_peak_rss:-0}" "bytes"
  e2e_metric "cell_s${sid}_${ch}.packages" "${cell_packages:-0}" "count"
done < "$PLAN_ROWS_FILE"

# ===========================================================================
# Step 3: Write canonical summary JSON, print matrix table, emit E2E summary.
# ===========================================================================
e2e_step "summary" "Matrix summary, table output, and completeness verification"

SUMMARY_KV="$(python3 - "$PLAN_JSON" "$RUN_STATE_JSON" <<'PY_SUMMARY'
import json
import math
import os
import sys
import tempfile
import unicodedata

plan_path, state_path = sys.argv[1:3]
INT53_MAX = (1 << 53) - 1


def nfc(s: str) -> str:
    return unicodedata.normalize("NFC", s)


def canon_str(s: str) -> str:
    s = nfc(s)
    out = ['"']
    for ch in s:
        cp = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif cp < 0x20:
            out.append(f"\\u{cp:04x}")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def canon_json(val) -> str:
    if val is None:
        return "null"
    if isinstance(val, bool):
        return "true" if val else "false"
    if isinstance(val, int):
        if abs(val) > INT53_MAX:
            raise ValueError(f"integer out of int53 range: {val}")
        return str(val)
    if isinstance(val, float):
        if not math.isfinite(val):
            raise ValueError(f"non-finite float: {val}")
        if val == 0.0:
            val = 0.0
        return json.dumps(val, allow_nan=False)
    if isinstance(val, str):
        return canon_str(val)
    if isinstance(val, list):
        return "[" + ",".join(canon_json(item) for item in val) + "]"
    if isinstance(val, dict):
        norm = {nfc(k): v for k, v in val.items()}
        return "{" + ",".join(f"{canon_str(k)}:{canon_json(norm[k])}" for k in sorted(norm.keys())) + "}"
    raise TypeError(type(val).__name__)


with open(plan_path, "r", encoding="utf-8") as pf:
    plan = json.load(pf)
if os.path.isfile(state_path):
    with open(state_path, "r", encoding="utf-8") as sf:
        state = json.load(sf)
else:
    state = {"records": {}, "executed": 0, "resumed": 0}

records = state["records"]
channels = plan["channels"]
selected_sets = plan["selected_sets"]

counts = {"fail": 0, "not-applicable": 0, "not-run": 0, "pass": 0}
missing_run_cells = 0
any_dirty = bool(plan["siblings_dirty"])
columns = {}

for ch in channels:
    col_counts = {"fail": 0, "not-applicable": 0, "not-run": 0, "pass": 0}
    rows = {}
    col_dirty = bool(plan["siblings_dirty"])
    run_rows = 0
    for sid in selected_sets:
        rec = records.get(f"{sid}|{ch}")
        if rec is None:
            missing_run_cells += 1
            rows[str(sid)] = "missing"
            continue
        st = rec["status"]
        rows[str(sid)] = st
        if st in counts:
            counts[st] += 1
            col_counts[st] += 1
        if st != "not-applicable":
            run_rows += 1
            if st not in ("pass", "fail", "not-run"):
                missing_run_cells += 1
        if rec.get("siblings_dirty"):
            any_dirty = True
            col_dirty = True
    all_run_passed = (run_rows > 0) and (col_counts["pass"] == run_rows)
    columns[ch] = {
        "all_run_passed": all_run_passed,
        "citeable_for_pass": all_run_passed and (not col_dirty),
        "counts": col_counts,
        "rows": rows,
        "rustc_version": plan["rustc_versions"][ch],
    }

# Summary JSON is a pure canonical function of (channels, sets, receipt, merged statuses, dirty)
# so re-running on an unchanged tree reproduces byte-identical summary.json.
summary_obj = {
    "channels": channels,
    "columns": columns,
    "counts": counts,
    "receipt": plan["receipt"],
    "schema": "frob.toolchain-matrix-summary/1",
    "sets": selected_sets,
    "siblings_dirty": any_dirty,
}
raw = canon_json(summary_obj).encode("utf-8")
os.makedirs(plan["out_dir"], exist_ok=True)
fd, tmp_path = tempfile.mkstemp(prefix=".summary-", suffix=".tmp", dir=plan["out_dir"])
with os.fdopen(fd, "wb") as tf:
    tf.write(raw)
    tf.flush()
    os.fsync(tf.fileno())
os.replace(tmp_path, plan["summary_path"])

# Print human-readable matrix table to stderr/stdout? Print table directly to stdout.
display = {"pass": "pass", "fail": "fail", "not-run": "not-run", "not-applicable": "—", "missing": "MISSING"}
label_w = max([len("Build set")] + [len(plan["set_labels"][str(sid)]) for sid in selected_sets])
col_w = {ch: max(len(ch), 7) for ch in channels}

hdr = f"{'Build set':<{label_w}}" + "".join(f" | {ch:<{col_w[ch]}}" for ch in channels)
sep = "-" * label_w + "".join("-+-" + "-" * col_w[ch] for ch in channels)
table_lines = [hdr, sep]
for sid in selected_sets:
    lbl = plan["set_labels"][str(sid)]
    row_s = f"{lbl:<{label_w}}"
    for ch in channels:
        st = columns[ch]["rows"].get(str(sid), "missing")
        row_s += f" | {display.get(st, st):<{col_w[ch]}}"
    table_lines.append(row_s)

# Emit table to stdout (via /dev/fd/3 if available, else stderr; wait: print to stdout after KV capture).
table_path = os.path.join(os.path.dirname(plan_path), "matrix_table.txt")
with open(table_path, "w", encoding="utf-8") as tf:
    tf.write("\n".join(table_lines) + "\n")

print(f"CNT_PASS={counts['pass']}")
print(f"CNT_FAIL={counts['fail']}")
print(f"CNT_NOT_RUN={counts['not-run']}")
print(f"CNT_NA={counts['not-applicable']}")
print(f"MISSING_RUN={missing_run_cells}")
print(f"EXECUTED={state['executed']}")
print(f"RESUMED={state['resumed']}")
print(f"TABLE_PATH={table_path}")
PY_SUMMARY
)"

CNT_PASS="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="CNT_PASS"{print $2; exit}')"
CNT_FAIL="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="CNT_FAIL"{print $2; exit}')"
CNT_NOT_RUN="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="CNT_NOT_RUN"{print $2; exit}')"
CNT_NA="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="CNT_NA"{print $2; exit}')"
MISSING_RUN="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="MISSING_RUN"{print $2; exit}')"
EXECUTED_CELLS="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="EXECUTED"{print $2; exit}')"
RESUMED_CELLS="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="RESUMED"{print $2; exit}')"
TABLE_PATH="$(printf '%s\n' "$SUMMARY_KV" | awk -F= '$1=="TABLE_PATH"{print $2; exit}')"

e2e_metric "cells_pass" "${CNT_PASS:-0}" "count"
e2e_metric "cells_fail" "${CNT_FAIL:-0}" "count"
e2e_metric "cells_not_run" "${CNT_NOT_RUN:-0}" "count"
e2e_metric "cells_not_applicable" "${CNT_NA:-0}" "count"
e2e_metric "cells_executed" "${EXECUTED_CELLS:-0}" "count"
e2e_metric "cells_resumed" "${RESUMED_CELLS:-0}" "count"

e2e_assert_eq "all_run_cells_recorded" "${MISSING_RUN:-1}" "0"

cat "$TABLE_PATH"
printf 'ndjson=%s\n' "$NDJSON_PATH"
printf 'summary=%s\n' "$SUMMARY_PATH"
printf 'executed=%s resumed=%s pass=%s fail=%s not_run=%s not_applicable=%s\n' \
  "${EXECUTED_CELLS:-0}" "${RESUMED_CELLS:-0}" "${CNT_PASS:-0}" "${CNT_FAIL:-0}" "${CNT_NOT_RUN:-0}" "${CNT_NA:-0}"
