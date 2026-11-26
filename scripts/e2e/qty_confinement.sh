#!/usr/bin/env bash
# e2e: bead=N33c-03 milestone=M0 labels=suite,marrow plants=fs-qty-direct,qty-generated-byte seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of frob_core::qty closed alias set, compile-fail pair
# tests, alias-count gate (<= 64), xtask check-generated row, and L-2.fs-qty-direct
# confinement (bead N33c-03).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/qty_confinement/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "qty_confinement" "N33c-03" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-qty-confinement-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

e2e_step "qty_unit_and_compile_fail" "Run frob-core qty registry, compile-fail, and doc tests"

e2e_metric "alias_count" "36" "aliases"
e2e_metric "equal_dimension_pairs" "9" "pairs"

e2e_cmd --name "cargo_test_frob_core_qty" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core --test qty
e2e_assert_eq "cargo_test_frob_core_qty_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "cargo_test_frob_core_qty_doc" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/frob-core" \
  cargo test --offline -j 1 -p frob-core --doc qty
e2e_assert_eq "cargo_test_frob_core_qty_doc_exit" "${E2E_LAST_EXIT}" "0"

e2e_step "xtask_qty_confinement_and_generated" "Run xtask qty_confinement and check_generated suites (including planted L-2.fs-qty-direct and gen.differs controls)"

e2e_cmd --name "cargo_test_xtask_qty" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo test --offline -j 1 -p xtask --test qty_confinement
e2e_assert_eq "cargo_test_xtask_qty_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "xtask_check_generated_real" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo run --offline -j 1 -p xtask -- check-generated --generator quantity-aliases-and-products --json
e2e_assert_eq "xtask_check_generated_real_exit" "${E2E_LAST_EXIT}" "0"

_caught_finding() {
  [ "${E2E_LAST_EXIT}" = 2 ] && [ -f "${E2E_LAST_STDOUT}" ] &&
    python3 - "${E2E_LAST_STDOUT}" "$@" <<'PY'
import json
import sys

report, rule, path, subject = sys.argv[1:]
with open(report, encoding="utf-8") as source:
    findings = json.load(source)["findings"]
sys.exit(not any(
    row["rule"] == rule and row["path"] == path
    and (not subject or row["subject"] == subject)
    for row in findings
))
PY
}

e2e_step "plant_direct_dependency" "Check a scratch frob-lie manifest with a direct fs-qty dependency"
deps_root="${E2E_WORK_DIR}/qty_deps"
mkdir -p "${deps_root}/crates/frob-lie/src"
cp "${E2E_REPO_ROOT}/franken-stack.lock" "${deps_root}/franken-stack.lock"
cat > "${deps_root}/Cargo.toml" <<'EOF'
[workspace]
resolver = "3"
members = ["crates/frob-lie"]
EOF
cat > "${deps_root}/crates/frob-lie/Cargo.toml" <<EOF
[package]
name = "frob-lie"
version = "0.0.1"
edition = "2024"

[package.metadata.frob]
layer = 1
rank = 1

[dependencies]
fs-qty = { path = "${E2E_REPO_ROOT}/../frankensim/crates/fs-qty", default-features = false }
EOF
printf '// Scratch crate for the confinement check.\n' > "${deps_root}/crates/frob-lie/src/lib.rs"
e2e_cmd --name "qty_plant_lockfile" -- \
  cargo generate-lockfile --offline --manifest-path "${deps_root}/Cargo.toml"
e2e_assert_eq "qty_plant_lockfile_exit" "${E2E_LAST_EXIT}" "0"
e2e_cmd --name "check_direct_qty_dependency" --expect-exit 2 -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo run --offline -j 1 -p xtask -- check-deps --root "${deps_root}" --json
fs_qty_caught=0
if _caught_finding "L-2.fs-qty-direct" "crates/frob-lie/Cargo.toml" "frob-lie -> fs-qty"; then fs_qty_caught=1; fi
e2e_assert_eq "planted_fs_qty_import_caught" "${fs_qty_caught}" "1"
expected=1
if e2e_plant "fs-qty-direct"; then expected=0; fi
e2e_assert_eq "fs-qty-direct" "${fs_qty_caught}" "${expected}"

e2e_step "plant_generated_byte" "Check a one-byte mutation of generated quantities in scratch"
gen_root="${E2E_WORK_DIR}/qty_generated"
mkdir -p "${gen_root}"
for path in registries/qty_aliases.toml registries/qty_products.toml crates/frob-core/src/qty_gen.rs; do
  mkdir -p "${gen_root}/$(dirname "${path}")"
  cp "${E2E_REPO_ROOT}/${path}" "${gen_root}/${path}"
done
python3 - "${gen_root}/crates/frob-core/src/qty_gen.rs" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
source = path.read_bytes()
old = b"ALIAS_COUNT: usize = 36;"
if source.count(old) != 1:
    raise SystemExit("expected one starter alias count declaration")
path.write_bytes(source.replace(old, b"ALIAS_COUNT: usize = 37;"))
PY
e2e_cmd --name "check_generated_qty_byte" --expect-exit 2 -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/xtask" \
  cargo run --offline -j 1 -p xtask -- check-generated --generator quantity-aliases-and-products --root "${gen_root}" --json
qty_byte_caught=0
if [ "${E2E_LAST_EXIT}" = 2 ] && [ -f "${E2E_LAST_STDOUT}" ] && python3 - "${E2E_LAST_STDOUT}" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as source:
    findings = json.load(source)["findings"]
actual = [(row["rule"], row["subject"], row["path"]) for row in findings]
sys.exit(actual != [("gen.differs", "quantity-aliases-and-products", "crates/frob-core/src/qty_gen.rs")])
PY
then qty_byte_caught=1; fi
e2e_assert_eq "planted_qty_generated_byte_caught" "${qty_byte_caught}" "1"
expected=1
if e2e_plant "qty-generated-byte"; then expected=0; fi
e2e_assert_eq "qty-generated-byte" "${qty_byte_caught}" "${expected}"
