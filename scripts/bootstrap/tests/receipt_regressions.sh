#!/usr/bin/env bash
# scripts/bootstrap/tests/receipt_regressions.sh
# Regression tests for three defects caught in multi-pass bug hunting:
# 1. Annex workspace lockfiles (spark/frob-spark-cuda, spark/frob-spark-metal,
#    derive/frob-derive, bindings/frob-gymnasium per N34p4-02) must be discovered
#    and recorded in lockfiles[].
# 2. Checkout-location independence when `cargo tree -d` outputs canonicalized
#    (realpath) paths under symlinked directories (e.g. /var -> /private/var on macOS)
#    or differently named repo directories.
# 3. `suite_workspace_skeleton.sh` step 9 (salt-removed fs-la probe) must run with
#    cwd inside PROBE_DIR (outside REPO_ROOT) and with a standalone [workspace]
#    manifest so Cargo does not inherit FRANKENSIM_DEPGRAPH_SALT from REPO_ROOT/.cargo/config.toml.

set -euo pipefail
TEST_NAME="receipt_regressions"
# shellcheck source=scripts/bootstrap/tests/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

TMP="$(make_test_dir "${TEST_NAME}")"
E2E_SCRIPT="${REPO_ROOT}/scripts/e2e/suite_workspace_skeleton.sh"

# ---------------------------------------------------------------------------
# Regression 1: Seven-workspace annex discovery (spark-cuda, derive, etc.)
# ---------------------------------------------------------------------------
R1_BASE="${TMP}/annex_ws"
mkdir -p "${R1_BASE}"
REV_SIB="$(create_synthetic_sibling "${R1_BASE}/asupersync" "asupersync" "0.7.0")"
LOCK_R1="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-ASUP"
name = "asupersync"
posture = "consume"
path = "../asupersync"
git_rev = "${REV_SIB}"
version = "=0.7.0"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-ASUP-01"
receipt = ""
fallback = "fb"
EOF
)"
create_synthetic_workspace "${R1_BASE}/repo" "${LOCK_R1}" "nightly-2026-08-31" "" >/dev/null

# Add two real annex workspaces at their N34p4-02 paths.
for pair in "spark/frob-spark-cuda:frob-spark-cuda" "derive/frob-derive:frob-derive"; do
  rel="${pair%%:*}"
  pkg="${pair##*:}"
  mkdir -p "${R1_BASE}/repo/${rel}/src"
  cat > "${R1_BASE}/repo/${rel}/Cargo.toml" <<EOF
[workspace]
resolver = "3"
members = ["."]

[package]
name = "${pkg}"
version = "0.0.1"
edition = "2024"
EOF
  printf 'pub fn annex() {}\n' > "${R1_BASE}/repo/${rel}/src/lib.rs"
  cargo generate-lockfile --offline --manifest-path "${R1_BASE}/repo/${rel}/Cargo.toml" >/dev/null 2>&1
done
git_commit_all "${R1_BASE}/repo" "add spark-cuda and derive annexes" >/dev/null

"${FIRST_RECEIPT}" --stdout --root "${R1_BASE}/repo" > "${TMP}/annex_receipt.json"
python3 - "${TMP}/annex_receipt.json" "${R1_BASE}/repo" "${B3SUM_BIN}" <<'PYEOF'
import json, os, subprocess, sys
r = json.load(open(sys.argv[1], "r", encoding="utf-8"))
repo, b3 = sys.argv[2], sys.argv[3]
ws_map = {item["workspace"]: item for item in r["lockfiles"]}
expected = ["core", "derive", "spark-cuda"]
actual = sorted(ws_map.keys())
assert actual == expected, f"lockfiles workspaces mismatch: expected {expected}, got {actual}"
for ws, rel in (("core", "Cargo.lock"), ("derive", "derive/frob-derive/Cargo.lock"), ("spark-cuda", "spark/frob-spark-cuda/Cargo.lock")):
    want_h = subprocess.check_output([b3, "--no-names", os.path.join(repo, rel)]).decode("ascii").strip()
    assert ws_map[ws]["hash"] == want_h, f"{ws} lockfile hash mismatch: {ws_map[ws]['hash']} != {want_h}"
PYEOF

# ---------------------------------------------------------------------------
# Regression 2: Symlinked / differently named checkout with cargo tree -d duplicates
# ---------------------------------------------------------------------------
build_dup_workspace() {
  local parent_dir="$1"
  local repo_name="$2"
  mkdir -p "${parent_dir}"
  local rev1 rev2
  rev1="$(create_synthetic_sibling "${parent_dir}/dup_v1" "dup_dep" "0.1.0")"
  rev2="$(create_synthetic_sibling "${parent_dir}/dup_v2" "dup_dep" "0.2.0")"
  local lock_body
  lock_body="$(cat <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-DUPA"
name = "dup_v1"
posture = "consume-path"
path = "../dup_v1"
git_rev = "${rev1}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-DUPA-01"
receipt = ""
fallback = "fb"

[[foundation]]
id = "FD-DUPB"
name = "dup_v2"
posture = "consume-path"
path = "../dup_v2"
git_rev = "${rev2}"
toolchain = "nightly-2026-08-31"
admission_gate = "INT-DUPB-01"
receipt = ""
fallback = "fb"
EOF
)"
  local ws="${parent_dir}/${repo_name}"
  init_git_repo "${ws}"
  mkdir -p "${ws}/.cargo" "${ws}/crates/a/src" "${ws}/crates/b/src"
  cat > "${ws}/Cargo.toml" <<'EOF'
[workspace]
resolver = "3"
members = ["crates/*"]
exclude = ["spark", "apps", "derive", "bindings", "worker"]

[workspace.lints.rust]
unsafe_code = "deny"
EOF
  cat > "${ws}/rust-toolchain.toml" <<'EOF'
[toolchain]
channel = "nightly-2026-08-31"
components = ["rustfmt", "clippy"]
EOF
  cat > "${ws}/.cargo/config.toml" <<'EOF'
[env]
FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"

[alias]
xtask = "run --locked --package xtask --"
EOF
  cat > "${ws}/crates/a/Cargo.toml" <<'EOF'
[package]
name = "crate_a"
version = "0.0.1"
edition = "2024"

[dependencies]
dup_dep = { path = "../../../dup_v1" }
EOF
  printf 'pub fn a() {}\n' > "${ws}/crates/a/src/lib.rs"
  cat > "${ws}/crates/b/Cargo.toml" <<'EOF'
[package]
name = "crate_b"
version = "0.0.1"
edition = "2024"

[dependencies]
dup_dep = { path = "../../../dup_v2" }
EOF
  printf 'pub fn b() {}\n' > "${ws}/crates/b/src/lib.rs"
  cargo generate-lockfile --offline --manifest-path "${ws}/Cargo.toml" >/dev/null 2>&1
  printf '%s\n' "${lock_body}" > "${ws}/franken-stack.lock"
  git_commit_all "${ws}" "init dup workspace" >/dev/null
}

# Create two checkouts in differently named parents & repo names, one accessed via symlink.
build_dup_workspace "${TMP}/dup_loc_one" "repo_alpha"
build_dup_workspace "${TMP}/dup_loc_two/nested" "repo_beta"
ln -s "${TMP}/dup_loc_one" "${TMP}/dup_loc_one_link"

"${FIRST_RECEIPT}" --stdout --root "${TMP}/dup_loc_one_link/repo_alpha" > "${TMP}/dup_a.json"
"${FIRST_RECEIPT}" --stdout --root "${TMP}/dup_loc_two/nested/repo_beta" > "${TMP}/dup_b.json"

python3 - "${TMP}/dup_a.json" "${TMP}/dup_b.json" <<'PYEOF'
import json, sys
ba = open(sys.argv[1], "rb").read()
bb = open(sys.argv[2], "rb").read()
ra = json.loads(ba.decode("utf-8"))
rb = json.loads(bb.decode("utf-8"))
assert ra["duplicates"] == ["dup_dep 0.1.0", "dup_dep 0.2.0"], f"unexpected duplicates: {ra['duplicates']}"
assert ra["commands"] == rb["commands"], f"commands output_hash differed across checkout paths: {ra['commands']} vs {rb['commands']}"
assert ba == bb, "receipt bytes with cargo tree -d duplicates differed across symlinked/renamed checkouts"
PYEOF

# ---------------------------------------------------------------------------
# Regression 3: suite_workspace_skeleton.sh salt-removed probe isolates cwd
# ---------------------------------------------------------------------------
# Verify empirically that Cargo inherits [env] from REPO_ROOT/.cargo/config.toml when
# cwd is REPO_ROOT (even with env -u), and does NOT when cwd is PROBE_DIR outside REPO_ROOT.
SALT_PROBE_CHECK="${TMP}/salt_cfg_check"
mkdir -p "${SALT_PROBE_CHECK}"
FROM_REPO_OUT="$(cd "${REPO_ROOT}" && env -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT cargo -Z unstable-options config get env.FRANKENSIM_DEPGRAPH_SALT 2>/dev/null || true)"
FROM_OUTSIDE_OUT="$(cd "${SALT_PROBE_CHECK}" && env -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT cargo -Z unstable-options config get env.FRANKENSIM_DEPGRAPH_SALT 2>/dev/null || true)"
assert_eq "Cargo inherits repo salt when cwd is REPO_ROOT even under env -u" "${FROM_REPO_OUT}" 'env.FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"'
assert_eq "Cargo has no salt when cwd is outside REPO_ROOT" "${FROM_OUTSIDE_OUT}" ""

# Assert suite_workspace_skeleton.sh isolates the probe with [workspace] and runs it with cwd=PROBE_DIR.
python3 - "${E2E_SCRIPT}" <<'PYEOF'
import re, sys
src = open(sys.argv[1], "r", encoding="utf-8").read()
m = re.search(r"begin_step \"control_salt_removed\".*?end_step", src, re.S)
assert m is not None, "control_salt_removed step not found"
block = m.group(0)
assert "[workspace]" in block, "salt_probe Cargo.toml must declare standalone [workspace]"
assert "run_logged_cmd_in_dir" in block or 'cd "${PROBE_DIR}"' in block, (
    "control_salt_removed must run cargo check with cwd=PROBE_DIR (outside REPO_ROOT) so REPO_ROOT/.cargo/config.toml is not inherited"
)
PYEOF

pass_test
