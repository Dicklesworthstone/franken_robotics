#!/usr/bin/env bash
# Oracle capture script: FrankenSim in-tree Unitree G1 forward-dynamics and
# 2-second rollout digest capture (BD-007, INT-FSIM-01).
# inputs: none (exports pinned FrankenSim and Asupersync revisions via git archive; zero workspace build products)
#
# Usage:
#   tools/oracle/fsim_g1_digest.sh [--pin <rev>] [--out <fixture.toml>] [--diagnostic]
#
# Guarantees:
#   - Never modifies, switches, cleans, or moves ../frankensim or ../asupersync
#     (asserts HEAD and `git status --porcelain` are byte-identical before and
#     after the read-only `git archive` export).
#   - Never deletes files or directories (each export uses a fresh scratch dir).
#   - Writes only under ${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-fsim-intree/<pin>/
#     and the output fixture directory (default tests/fixtures/oracle/fsim/).
#   - Offloads compilation and execution through `rch exec --job` onto a remote
#     worker carrying FrankenSim's own pinned toolchain, never building locally
#     or calling `cargo` by absolute path.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
FSIM_REPO="$(cd "${REPO_ROOT}/../frankensim" && pwd)"
ASUP_REPO="$(cd "${REPO_ROOT}/../asupersync" && pwd)"
LOCK_FILE="${REPO_ROOT}/franken-stack.lock"

OUT_TOML="${REPO_ROOT}/tests/fixtures/oracle/fsim/g1_fd_digest.toml"
PIN_ARG=""
DIAGNOSTIC=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --pin)
      [ "$#" -ge 2 ] || { echo "error: --pin requires a commit revision" >&2; exit 2; }
      PIN_ARG="$2"
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || { echo "error: --out requires a fixture path" >&2; exit 2; }
      OUT_TOML="$2"
      shift 2
      ;;
    --diagnostic)
      DIAGNOSTIC=1
      shift
      ;;
    -h|--help)
      echo "usage: $0 [--pin <rev>] [--out <fixture.toml>] [--diagnostic]"
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [ ! -f "${LOCK_FILE}" ]; then
  echo "error: missing ${LOCK_FILE}" >&2
  exit 1
fi

read_lock_rev() {
  local foundation_id="$1"
  python3 - "${LOCK_FILE}" "${foundation_id}" <<'PY'
import re, sys
text = open(sys.argv[1], "r", encoding="utf-8").read()
want = sys.argv[2]
blocks = re.split(r"(?m)^\[\[foundation\]\]\s*$", text)
for blk in blocks[1:]:
    m_id = re.search(r'(?m)^id\s*=\s*"([^"]+)"', blk)
    if m_id and m_id.group(1) == want:
        m_rev = re.search(r'(?m)^git_rev\s*=\s*"([^"]+)"', blk)
        if m_rev:
            print(m_rev.group(1))
            sys.exit(0)
sys.exit(1)
PY
}

LOCK_PIN="$(read_lock_rev FD-FSIM)"
ASUP_PIN="$(read_lock_rev FD-ASUP)"

if [ -z "${LOCK_PIN}" ] || [ -z "${ASUP_PIN}" ]; then
  echo "error: could not parse FD-FSIM / FD-ASUP git_rev from ${LOCK_FILE}" >&2
  exit 1
fi

if [ -n "${PIN_ARG}" ]; then
  if [ "${PIN_ARG}" != "${LOCK_PIN}" ]; then
    printf 'error: pin mismatch: --pin "%s" differs from FD-FSIM git_rev "%s" in franken-stack.lock\n' "${PIN_ARG}" "${LOCK_PIN}" >&2
    exit 3
  fi
  PIN="${PIN_ARG}"
else
  PIN="${LOCK_PIN}"
fi

# Verify requested commits exist in sibling repos (strictly read-only).
if ! git -C "${FSIM_REPO}" cat-file -e "${PIN}^{commit}" 2>/dev/null; then
  printf 'error: commit %s not found in %s\n' "${PIN}" "${FSIM_REPO}" >&2
  exit 3
fi
if ! git -C "${ASUP_REPO}" cat-file -e "${ASUP_PIN}^{commit}" 2>/dev/null; then
  printf 'error: commit %s not found in %s\n' "${ASUP_PIN}" "${ASUP_REPO}" >&2
  exit 3
fi

hash_blake3_file() {
  if command -v b3sum >/dev/null 2>&1; then
    b3sum --no-names "$1"
  else
    echo "error: b3sum not found on PATH" >&2
    exit 1
  fi
}

hash_blake3_stdin() {
  if command -v b3sum >/dev/null 2>&1; then
    b3sum --no-names
  else
    echo "error: b3sum not found on PATH" >&2
    exit 1
  fi
}

# Snapshot ../frankensim HEAD and status BEFORE read-only git archive (AC 7).
FSIM_HEAD_BEFORE="$(git -C "${FSIM_REPO}" rev-parse HEAD)"
FSIM_STATUS_BEFORE="$(git -C "${FSIM_REPO}" status --porcelain | hash_blake3_stdin)"

# Export pinned commits into a self-contained scratch directory under
# ${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-fsim-intree/<pin>/current (zero deletions).
SCRATCH_BASE="${BIG_SCRATCH:-${TMPDIR:-/tmp}}"
EXPORT_ROOT="${SCRATCH_BASE}/frob-fsim-intree/${PIN}/current"
EXPORT_FSIM="${EXPORT_ROOT}/frankensim"
EXPORT_ASUP="${EXPORT_ROOT}/asupersync"
PROBE_DIR="${EXPORT_FSIM}/apps/frob-digest-probe"

mkdir -p "${EXPORT_FSIM}" "${EXPORT_ASUP}"

git -C "${FSIM_REPO}" archive "${PIN}" | tar -xf - -C "${EXPORT_FSIM}"

# Verify ../frankensim was byte-identically untouched by the read-only export.
FSIM_HEAD_AFTER="$(git -C "${FSIM_REPO}" rev-parse HEAD)"
FSIM_STATUS_AFTER="$(git -C "${FSIM_REPO}" status --porcelain | hash_blake3_stdin)"
if [ "${FSIM_HEAD_AFTER}" != "${FSIM_HEAD_BEFORE}" ] || [ "${FSIM_STATUS_AFTER}" != "${FSIM_STATUS_BEFORE}" ]; then
  printf 'FATAL: ../frankensim changed during git archive (HEAD %s->%s, status %s->%s)\n' \
    "${FSIM_HEAD_BEFORE}" "${FSIM_HEAD_AFTER}" "${FSIM_STATUS_BEFORE}" "${FSIM_STATUS_AFTER}" >&2
  exit 9
fi

# Export only the Rust crate workspace files of pinned asupersync (omits tests/docs/artifacts/.beads).
git -C "${ASUP_REPO}" archive "${ASUP_PIN}" -- \
  Cargo.lock Cargo.toml LICENSE README.md build.rs assets src \
  asupersync-browser-core asupersync-macros asupersync-tokio-compat \
  asupersync-wasm asupersync-zstd conformance drop_unwrap_finder \
  franken_decision franken_evidence franken_kernel frankenlab \
  | tar -xf - -C "${EXPORT_ASUP}"

# Copy probe crate and exact byte-for-byte digest.rs source into apps/frob-digest-probe/.
PROBE_SRC_ROOT="${REPO_ROOT}/crates/frob-fsim/intree-probe"
DIGEST_SRC="${REPO_ROOT}/crates/frob-fsim/src/referee/digest.rs"

mkdir -p "${PROBE_DIR}/src"
cp "${PROBE_SRC_ROOT}/Cargo.toml" "${PROBE_DIR}/Cargo.toml"
cp "${PROBE_SRC_ROOT}/src/main.rs" "${PROBE_DIR}/src/main.rs"
cp "${DIGEST_SRC}" "${PROBE_DIR}/src/digest.rs"
if [ -f "${EXPORT_FSIM}/Cargo.lock" ]; then
  cp "${EXPORT_FSIM}/Cargo.lock" "${PROBE_DIR}/Cargo.lock"
fi

# Verify byte-for-byte digest source parity before offloading.
DIGEST_SRC_B3="$(hash_blake3_file "${DIGEST_SRC}")"
DIGEST_PROBE_B3="$(hash_blake3_file "${PROBE_DIR}/src/digest.rs")"
if [ "${DIGEST_SRC_B3}" != "${DIGEST_PROBE_B3}" ]; then
  echo "error: digest.rs copy hash mismatch" >&2
  exit 1
fi

# Read FrankenSim's own pinned toolchain channel from the exported rust-toolchain.toml.
FSIM_TOOLCHAIN="$(python3 - "${EXPORT_FSIM}/rust-toolchain.toml" <<'PY'
import re, sys
text = open(sys.argv[1], "r", encoding="utf-8").read()
m = re.search(r'(?m)^channel\s*=\s*"([^"]+)"', text)
if not m:
    sys.exit(1)
print(m.group(1))
PY
)"

# Select remote RCH Linux workers that have FrankenSim's pinned toolchain installed.
CAPS_JSON="${EXPORT_ROOT}/rch_caps.json"
rch workers capabilities --json >"${CAPS_JSON}" 2>/dev/null
ELIGIBLE_WORKERS="$(python3 - "${CAPS_JSON}" "${FSIM_TOOLCHAIN}" <<'PY'
import json, sys
d = json.load(open(sys.argv[1], "r", encoding="utf-8"))
tc = sys.argv[2]
workers = []
for w in d.get("data", {}).get("workers", []):
    tcs = w.get("capabilities", {}).get("rustup_toolchains", [])
    if any(t.startswith(tc + "-x86_64-unknown-linux-gnu") for t in tcs):
        workers.append(w["id"])
print(",".join(workers))
PY
)"

if [ -z "${ELIGIBLE_WORKERS}" ]; then
  printf 'error: no remote Linux RCH worker reports toolchain %s\n' "${FSIM_TOOLCHAIN}" >&2
  exit 4
fi

# Write a small self-contained probe runner inside EXPORT_ROOT so the worker
# executes inside frankensim/apps/frob-digest-probe, inheriting FrankenSim's own
# rust-toolchain.toml and .cargo/config.toml verbatim.
RUNNER_SH="${EXPORT_ROOT}/run_probe.sh"
cat >"${RUNNER_SH}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
unset RUSTUP_TOOLCHAIN FRANKENSIM_DEPGRAPH_SALT FRANKENSIM_DEPGRAPH_RECEIPT
export CARGO_TARGET_DIR="\${TMPDIR:-/tmp}/frob-fsim-intree-${PIN}"
mkdir -p "\${CARGO_TARGET_DIR}"
cd frankensim/apps/frob-digest-probe
BUILD_LOG="\${CARGO_TARGET_DIR}/probe_build.log"
echo "=== RUSTC_VV_BEGIN ==="
rustc -vV
echo "=== RUSTC_VV_END ==="
echo "=== CARGO_VER_BEGIN ==="
cargo --version
echo "=== CARGO_VER_END ==="
cargo build -j 1 >"\${BUILD_LOG}" 2>&1 || {
  tail -n 80 "\${BUILD_LOG}" >&2
  exit 101
}
echo "=== BUILD_SUMMARY_BEGIN ==="
grep -E '^(   Compiling (frob-digest-probe|fs-mbd|fs-ga|fs-blake3)|    Finished )' "\${BUILD_LOG}" || tail -n 10 "\${BUILD_LOG}"
echo "=== BUILD_SUMMARY_END ==="
echo "=== PROBE_OUT_BEGIN ==="
cargo run --quiet --offline -j 1 "\$@"
echo "=== PROBE_OUT_END ==="
EOF
chmod +x "${RUNNER_SH}"

# Prepare output paths strictly under the fixture directory.
OUT_DIR="$(dirname "${OUT_TOML}")"
mkdir -p "${OUT_DIR}"
OUT_BASE="$(basename "${OUT_TOML}" .toml)"
CAPTURE_LOG="${OUT_DIR}/${OUT_BASE}.capture.log"
PIN_TXT="${OUT_DIR}/PIN.txt"
JOB_RAW_LOG="${EXPORT_ROOT}/rch_job.log"

printf 'oracle-capture-tool: frankensim-intree-g1 (pin=%s; toolchain=%s; eligible-workers=%s)\n' \
  "${PIN}" "${FSIM_TOOLCHAIN}" "${ELIGIBLE_WORKERS}"

# Execute on an eligible remote RCH worker via `rch exec --job` (strict remote).
(
  cd "${EXPORT_ROOT}"
  if [ "${DIAGNOSTIC}" -eq 1 ]; then
    env -u RUSTUP_TOOLCHAIN -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT \
      RCH_REQUIRE_REMOTE=1 \
      RCH_BUILD_SLOTS=1 \
      RCH_WORKERS="${ELIGIBLE_WORKERS}" \
      RCH_CANONICAL_PROJECT_ROOT="${SCRATCH_BASE}" \
      RCH_DAEMON_WAIT_RESPONSE_TIMEOUT_SECS="${RCH_DAEMON_WAIT_RESPONSE_TIMEOUT_SECS:-1800}" \
      rch exec --job -- ./run_probe.sh -- --diagnostic \
      >"${JOB_RAW_LOG}" 2>&1
  else
    env -u RUSTUP_TOOLCHAIN -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT \
      RCH_REQUIRE_REMOTE=1 \
      RCH_BUILD_SLOTS=1 \
      RCH_WORKERS="${ELIGIBLE_WORKERS}" \
      RCH_CANONICAL_PROJECT_ROOT="${SCRATCH_BASE}" \
      RCH_DAEMON_WAIT_RESPONSE_TIMEOUT_SECS="${RCH_DAEMON_WAIT_RESPONSE_TIMEOUT_SECS:-1800}" \
      rch exec --job -- ./run_probe.sh \
      >"${JOB_RAW_LOG}" 2>&1
  fi
)

CAPTURED_ON="$(date -u +%Y-%m-%d)"
CAPTURED_UTC="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
PROBE_STDOUT_FILE="${EXPORT_ROOT}/probe.stdout"
RUSTC_VV_FILE="${EXPORT_ROOT}/rustc_vv.txt"
CARGO_VER_FILE="${EXPORT_ROOT}/cargo_ver.txt"

# Extract delimited sections from the remote job log.
python3 - "${JOB_RAW_LOG}" "${RUSTC_VV_FILE}" "${CARGO_VER_FILE}" "${PROBE_STDOUT_FILE}" <<'PY'
import sys

raw = open(sys.argv[1], "r", encoding="utf-8", errors="replace").read().replace("\r\n", "\n")

def extract(begin_marker: str, end_marker: str) -> str:
    i = raw.find(begin_marker)
    if i < 0:
        raise SystemExit(f"error: missing marker `{begin_marker}` in remote job log")
    i += len(begin_marker)
    if i < len(raw) and raw[i] == "\n":
        i += 1
    j = raw.find(end_marker, i)
    if j < 0:
        raise SystemExit(f"error: missing marker `{end_marker}` in remote job log")
    return raw[i:j].strip() + "\n"

open(sys.argv[2], "w", encoding="utf-8").write(extract("=== RUSTC_VV_BEGIN ===", "=== RUSTC_VV_END ==="))
open(sys.argv[3], "w", encoding="utf-8").write(extract("=== CARGO_VER_BEGIN ===", "=== CARGO_VER_END ==="))
open(sys.argv[4], "w", encoding="utf-8").write(extract("=== PROBE_OUT_BEGIN ===", "=== PROBE_OUT_END ==="))
PY

RUSTC_FIRST_LINE="$(head -n 1 "${RUSTC_VV_FILE}")"
CARGO_VER="$(head -n 1 "${CARGO_VER_FILE}")"

# Write the capture record beside the fixture (contains full worker rustc -vV,
# full rch/cargo build log, and exact probe stdout including both digest lines).
{
  printf '# FrankenSim in-tree oracle capture log (BD-007, INT-FSIM-01)\n'
  printf 'pin=%s\n' "${PIN}"
  printf 'asupersync_pin=%s\n' "${ASUP_PIN}"
  printf 'toolchain=%s\n' "${FSIM_TOOLCHAIN}"
  printf 'captured_utc=%s\n' "${CAPTURED_UTC}"
  printf 'digest_source_blake3=%s\n' "${DIGEST_SRC_B3}"
  printf 'cargo_version=%s\n' "${CARGO_VER}"
  printf '%s\n' '--- worker rustc -vV ---'
  cat "${RUSTC_VV_FILE}"
  printf '%s\n' '--- probe stdout ---'
  cat "${PROBE_STDOUT_FILE}"
  printf '%s\n' '--- full remote build & run log ---'
  cat "${JOB_RAW_LOG}"
} >"${CAPTURE_LOG}"

CAPTURE_LOG_B3="$(hash_blake3_file "${CAPTURE_LOG}")"
SCRIPT_SHA256="$(python3 - "${REPO_ROOT}/tools/oracle/fsim_g1_digest.sh" <<'PY'
import hashlib, sys
print(hashlib.sha256(open(sys.argv[1], "rb").read()).hexdigest())
PY
)"

# Emit strict frob_core::registry::toml fixture (no TOML floats; all floats as strings).
python3 - \
  "${PROBE_STDOUT_FILE}" \
  "${OUT_TOML}" \
  "${PIN}" \
  "${FSIM_TOOLCHAIN}" \
  "${RUSTC_FIRST_LINE}" \
  "${CAPTURED_ON}" \
  "${CAPTURE_LOG_B3}" \
  "${DIGEST_SRC_B3}" \
  "${SCRIPT_SHA256}" <<'PY'
import sys

(
    probe_stdout_path,
    out_toml_path,
    pin,
    toolchain,
    rustc_version,
    captured_on,
    capture_log_b3,
    digest_src_b3,
    script_sha256,
) = sys.argv[1:10]

kv = {}
for raw in open(probe_stdout_path, "r", encoding="utf-8"):
    line = raw.strip()
    if "=" in line and not line.startswith("diag."):
        k, v = line.split("=", 1)
        kv[k] = v

required = [
    "target_arch", "target_os",
    "links", "nq", "nv", "actuators", "mass_kg",
    "orientation_parameterization", "rollout_steps",
    "fd_digest", "rollout_2s_digest",
    "link_names", "joint_names", "joint_kinds",
]
for k in required:
    if k not in kv or not kv[k]:
        raise SystemExit(f"error: probe output missing required field `{k}`")

def toml_str(s: str) -> str:
    esc = s.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{esc}"'

def toml_str_array(items):
    return "[" + ", ".join(toml_str(x) for x in items) + "]"

link_names = kv["link_names"].split(",")
joint_names = kv["joint_names"].split(",")
joint_kinds = kv["joint_kinds"].split(",")

pos_coords = [
    "0:base_x:base-position:m",
    "1:base_y:base-position:m",
    "2:base_z:base-position:m",
    "3:base_qw:base-orientation-quat-wxyz:1",
    "4:base_qx:base-orientation-quat-wxyz:1",
    "5:base_qy:base-orientation-quat-wxyz:1",
    "6:base_qz:base-orientation-quat-wxyz:1",
]
vel_coords = [
    "0:base_wx_body:base-angular-velocity-body:rad/s",
    "1:base_wy_body:base-angular-velocity-body:rad/s",
    "2:base_wz_body:base-angular-velocity-body:rad/s",
    "3:base_vx_body:base-linear-velocity-body:m/s",
    "4:base_vy_body:base-linear-velocity-body:m/s",
    "5:base_vz_body:base-linear-velocity-body:m/s",
]
kind_map = {
    "revolute": ("revolute-joint", "rad", "rad/s"),
    "prismatic": ("prismatic-joint", "m", "m/s"),
    "helical": ("helical-joint", "rad", "rad/s"),
}
for idx, (jname, jkind) in enumerate(zip(joint_names, joint_kinds)):
    ckind, punit, vunit = kind_map[jkind]
    pos_coords.append(f"{7 + idx}:{jname}:{ckind}:{punit}")
    vel_coords.append(f"{6 + idx}:{jname}_vel:{ckind}:{vunit}")

lines = [
    "# oracle",
    "# Captured directly inside a read-only git archive export of FrankenSim at the pinned commit.",
    "# Produced exclusively by tools/oracle/fsim_g1_digest.sh (BD-007, INT-FSIM-01).",
    "# Never hand-edit or regenerate from candidate output.",
    'kind = "oracle"',
    'oracle = "frankensim-intree-g1"',
    f"pin = {toml_str(pin)}",
    f"toolchain = {toml_str(toolchain)}",
    f"rustc_version = {toml_str(rustc_version)}",
    f"target_arch = {toml_str(kv['target_arch'])}",
    f"target_os = {toml_str(kv['target_os'])}",
    f"captured_on = {toml_str(captured_on)}",
    'capture_script = "tools/oracle/fsim_g1_digest.sh"',
    f"capture_script_sha256 = {toml_str(script_sha256)}",
    'capture_log = "tests/fixtures/oracle/fsim/g1_fd_digest.capture.log"',
    f"capture_log_blake3 = {toml_str(capture_log_b3)}",
    f"digest_source_blake3 = {toml_str(digest_src_b3)}",
    f"fd_digest = {toml_str(kv['fd_digest'])}",
    f"rollout_2s_digest = {toml_str(kv['rollout_2s_digest'])}",
    f"rollout_steps = {int(kv['rollout_steps'])}",
    "rollout_h = [1, 480]",
    'gravity_world_m_s2 = ["0.0", "0.0", "-9.80665"]',
    f"links = {int(kv['links'])}",
    f"nq = {int(kv['nq'])}",
    f"nv = {int(kv['nv'])}",
    f"actuators = {int(kv['actuators'])}",
    f"mass_kg = {toml_str(kv['mass_kg'])}",
    f"orientation_parameterization = {toml_str(kv['orientation_parameterization'])}",
    "",
    "[layout]",
    f"link_names = {toml_str_array(link_names)}",
    f"joint_names = {toml_str_array(joint_names)}",
    f"joint_kinds = {toml_str_array(joint_kinds)}",
    f"position_coords = {toml_str_array(pos_coords)}",
    f"velocity_coords = {toml_str_array(vel_coords)}",
    "",
]

with open(out_toml_path, "w", encoding="utf-8") as fh:
    fh.write("\n".join(lines))
PY

{
  printf 'oracle: frankensim-intree-g1\n'
  printf 'pin: %s\n' "${PIN}"
  printf 'toolchain: %s\n' "${FSIM_TOOLCHAIN}"
  printf 'rustc_version: %s\n' "${RUSTC_FIRST_LINE}"
  printf 'captured_on: %s\n' "${CAPTURED_ON}"
  printf 'producer: tools/oracle/fsim_g1_digest.sh\n'
  printf 'capture_log_blake3: %s\n' "${CAPTURE_LOG_B3}"
  printf 'note: Unitree G1 forward-dynamics and 2s rollout digests captured inside FrankenSim at the pinned commit (BD-007, INT-FSIM-01). Never hand-edited or regenerated from candidate output.\n'
} >"${PIN_TXT}"

printf 'captured fixture: %s\n' "${OUT_TOML}"
printf 'captured log:     %s (blake3=%s)\n' "${CAPTURE_LOG}" "${CAPTURE_LOG_B3}"
grep -E '^(target_arch|target_os|fd_digest|rollout_2s_digest|links|nq|nv|actuators|mass_kg)=' "${PROBE_STDOUT_FILE}"
if [ "${DIAGNOSTIC}" -eq 1 ]; then
  grep -E '^diag\.' "${PROBE_STDOUT_FILE}" || true
fi
