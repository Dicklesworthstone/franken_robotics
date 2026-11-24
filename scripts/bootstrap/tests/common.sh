#!/usr/bin/env bash
# scripts/bootstrap/tests/common.sh
# Shared helpers for BD-001 bootstrap receipt shell tests.
# All synthetic git trees live under ${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-bd001-tests/;
# real sibling repositories are never touched.

set -euo pipefail

TESTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BOOTSTRAP_DIR="$(cd "${TESTS_DIR}/.." && pwd)"
REPO_ROOT="$(cd "${BOOTSTRAP_DIR}/../.." && pwd)"
FIRST_RECEIPT="${BOOTSTRAP_DIR}/first_receipt.sh"

if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf 'common.sh: b3sum not found\n' >&2
  exit 2
fi

SCRATCH_BASE="${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-bd001-tests"
mkdir -p "${SCRATCH_BASE}"

TEST_TMP=""
make_test_dir() {
  local name="$1"
  TEST_TMP="$(mktemp -d "${SCRATCH_BASE}/${name}.XXXXXX")"
  trap 'cleanup_test_dir' EXIT
  printf '%s' "${TEST_TMP}"
}

cleanup_test_dir() {
  if [ -n "${TEST_TMP}" ] && [ -d "${TEST_TMP}" ]; then
    chmod -R u+w "${TEST_TMP}" 2>/dev/null || true
  fi
}

fail_test() {
  printf 'FAIL [%s]: %s\n' "${TEST_NAME:-unknown}" "$1" >&2
  exit 1
}

pass_test() {
  printf 'PASS [%s]\n' "${TEST_NAME:-unknown}"
}

assert_eq() {
  local label="$1"
  local actual="$2"
  local expected="$3"
  if [ "${actual}" != "${expected}" ]; then
    fail_test "${label}: expected '${expected}', got '${actual}'"
  fi
}

assert_ne() {
  local label="$1"
  local a="$2"
  local b="$3"
  if [ "${a}" = "${b}" ]; then
    fail_test "${label}: expected values to differ, both were '${a}'"
  fi
}

# Initialize a synthetic git repository with deterministic author/committer identity.
init_git_repo() {
  local dir="$1"
  mkdir -p "${dir}"
  git -C "${dir}" init -q -b main
  git -C "${dir}" config user.name "Synthetic Test"
  git -C "${dir}" config user.email "synthetic@example.invalid"
  git -C "${dir}" config commit.gpgsign false
}

git_commit_all() {
  local dir="$1"
  local msg="$2"
  git -C "${dir}" add -A
  GIT_AUTHOR_DATE="2026-11-24T00:00:00Z" \
  GIT_COMMITTER_DATE="2026-11-24T00:00:00Z" \
    git -C "${dir}" commit -q -m "${msg}"
  git -C "${dir}" rev-parse HEAD
}

# Create a minimal synthetic sibling crate repo (e.g. asupersync or frankensim).
create_synthetic_sibling() {
  local dir="$1"
  local crate_name="$2"
  local version="${3:-0.1.0}"
  init_git_repo "${dir}"
  mkdir -p "${dir}/src"
  cat > "${dir}/Cargo.toml" <<EOF
[package]
name = "${crate_name}"
version = "${version}"
edition = "2024"
EOF
  cat > "${dir}/src/lib.rs" <<EOF
pub fn marker() -> &'static str { "${crate_name}" }
EOF
  git_commit_all "${dir}" "init ${crate_name}"
}

# Create a minimal synthetic FrankenRobotics core workspace beside sibling repos.
# Arguments:
#   $1 = workspace dir (e.g. "${tmp}/root/repo")
#   $2 = franken-stack.lock contents
#   $3 = optional toolchain channel (default nightly-2026-08-31)
#   $4 = optional patch block (default empty)
create_synthetic_workspace() {
  local ws_dir="$1"
  local lock_body="$2"
  local channel="${3:-nightly-2026-08-31}"
  local patch_block="${4:-}"

  init_git_repo "${ws_dir}"
  mkdir -p "${ws_dir}/.cargo" "${ws_dir}/crates/xtask/src"

  cat > "${ws_dir}/Cargo.toml" <<EOF
[workspace]
resolver = "3"
members = ["crates/*"]
exclude = ["spark", "apps", "derive", "bindings", "worker"]

[workspace.lints.rust]
unsafe_code = "deny"

${patch_block}

[profile.dev.package.fsqlite]
opt-level = 1

[profile.dev.package.fsqlite-core]
opt-level = 1
EOF

  cat > "${ws_dir}/rust-toolchain.toml" <<EOF
[toolchain]
channel = "${channel}"
components = ["rustfmt", "clippy"]
EOF

  cat > "${ws_dir}/.cargo/config.toml" <<'EOF'
[env]
FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"

[alias]
xtask = "run --locked --package xtask --"
EOF

  cat > "${ws_dir}/crates/xtask/Cargo.toml" <<'EOF'
[package]
name = "xtask"
version = "0.0.1"
edition = "2024"
EOF

  cat > "${ws_dir}/crates/xtask/src/main.rs" <<'EOF'
fn main() {
    eprintln!("usage: cargo xtask <check>");
    std::process::exit(3);
}
EOF

  # Generate exact Cargo.lock offline (covers [[patch.unused]] when [patch.crates-io] is set).
  cargo generate-lockfile --offline --manifest-path "${ws_dir}/Cargo.toml" >/dev/null 2>&1

  printf '%s\n' "${lock_body}" > "${ws_dir}/franken-stack.lock"

  git_commit_all "${ws_dir}" "init synthetic workspace"
}
