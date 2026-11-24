#!/usr/bin/env bash
# Shared helper library for scripts/admission/tests/coupling_*.sh unit tests.
# Never deletes files or directories (all scratch dirs live under
# ${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-CopperDove/ and are left intact).

set -euo pipefail

COUPLING_TESTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "$COUPLING_TESTS_DIR/../../.." && pwd -P)"
QTY_COUPLING_SCRIPT="$REPO_ROOT/scripts/admission/qty_coupling.sh"

coupling_make_scratch() {
  local purpose="$1"
  local base="${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-${FROB_SCRATCH_ACTOR:-CopperDove}"
  mkdir -p "$base"
  local ts
  ts="$(date -u +%Y%m%dT%H%M%SZ)"
  mktemp -d "$base/${purpose}-${ts}-XXXXXX"
}

# Build the base synthetic five-crate workspace git repository:
#   fs-qty (q)
#   fs-evidence (e) -> fs-qty
#   i -> fs-evidence
#   m -> i, fs-evidence
#   z (no edge to fs-qty)
# Prints two lines: <repo_path> and <commit_sha>.
coupling_create_base_fixture_repo() {
  local scratch="$1"
  local repo="$scratch/frankensim"
  mkdir -p "$repo/crates"

  cat > "$repo/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = ["crates/*"]
EOF

  for crate in fs-qty fs-evidence i m z; do
    mkdir -p "$repo/crates/$crate/src"
    cat > "$repo/crates/$crate/src/lib.rs" <<EOF
pub fn marker() -> &'static str { "$crate" }
EOF
  done

  cat > "$repo/crates/fs-qty/Cargo.toml" <<'EOF'
[package]
name = "fs-qty"
version = "0.0.1"
edition = "2021"

[dependencies]
EOF

  cat > "$repo/crates/fs-evidence/Cargo.toml" <<'EOF'
[package]
name = "fs-evidence"
version = "0.0.1"
edition = "2021"

[dependencies]
fs-qty = { path = "../fs-qty" }
EOF

  cat > "$repo/crates/i/Cargo.toml" <<'EOF'
[package]
name = "i"
version = "0.0.1"
edition = "2021"

[dependencies]
fs-evidence = { path = "../fs-evidence" }
EOF

  cat > "$repo/crates/m/Cargo.toml" <<'EOF'
[package]
name = "m"
version = "0.0.1"
edition = "2021"

[dependencies]
i = { path = "../i" }
fs-evidence = { path = "../fs-evidence" }
EOF

  cat > "$repo/crates/z/Cargo.toml" <<'EOF'
[package]
name = "z"
version = "0.0.1"
edition = "2021"

[dependencies]
EOF

  CARGO_TARGET_DIR="$scratch/cargo-target" \
    cargo generate-lockfile --offline --manifest-path "$repo/Cargo.toml" >/dev/null 2>&1

  git -C "$repo" init -q
  git -C "$repo" config user.name "CopperDove Test"
  git -C "$repo" config user.email "copperdove@test.local"
  git -C "$repo" add -A
  git -C "$repo" commit -q -m "base five-crate fixture"

  local sha
  sha="$(git -C "$repo" rev-parse HEAD)"
  printf '%s\n%s\n' "$repo" "$sha"
}

coupling_refresh_lock_and_commit() {
  local repo="$1"
  local scratch="$2"
  local msg="$3"
  CARGO_TARGET_DIR="$scratch/cargo-target" \
    cargo generate-lockfile --offline --manifest-path "$repo/Cargo.toml" >/dev/null 2>&1
  git -C "$repo" add -A
  git -C "$repo" commit -q -m "$msg"
  git -C "$repo" rev-parse HEAD
}

coupling_extract_kv() {
  local output="$1"
  local key="$2"
  printf '%s\n' "$output" | awk -F= -v k="$key" '$1 == k { print substr($0, length(k) + 2); exit }'
}
