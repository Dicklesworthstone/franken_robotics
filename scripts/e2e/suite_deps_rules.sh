#!/usr/bin/env bash
# e2e: bead=N33c-09 milestone=M0 labels=suite,xtask,workspace seed=1 needs=cpu
#
# End-to-end verification of `xtask check-deps` suite deny rules (bead N33c-09):
#   - Plant 1 (`asup.v0_5_0`): path dependency resolving `asupersync 0.5.0`
#   - Plant 2 (`ftui.executor-feature`): `frob-tui` enabling `ftui-runtime/asupersync-executor`
#   - Plant 3 (`suite.forbidden-package`): `frob-design` depending on `fnx-algorithms`
#   - Plant 4 (`L-4.foundation-confined`): `frob-core` normal dep on `franken-kernel` (FD-FKER)
#   - Plant 5 (`suite.forbidden-package`): `derive/` depending on `fj-egraph`
#   - Plant 6 (`suite.forbidden-package`): optional feature `cloth-oracle` adding `ftx-drape`
#   - Plant 7 (`asup.v0_5_0`): unlinked `asupersync 0.5.0` entry in `Cargo.lock`
#   - Plant 8 (`suite.egg`): `frob-kir` depending on `egg`
#   - Clean tree: `cargo xtask check-deps` on the clean repository with all optional features
#
# Owned by N33c-09; N33c-29 and N33c-38 extend this script with their own plants.
# Claims no gates (INT-ASUP-01 of BD-008 cites the clean-tree result).
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "suite_deps_rules" "N33c-09" "$@"

XTASK_TARGET_DIR="${RCH_TARGET_BASE:-${E2E_WORK_DIR}}/suite_deps_rules_xtask"
mkdir -p "${XTASK_TARGET_DIR}"

PLANTS_TOTAL=0
PLANTS_CAUGHT=0
LOCK_SEQ=0

# Compute BLAKE3 digest of a file (falls back to frob-e2e sha256 if b3sum is absent).
_hash_manifest() {
  local fpath="$1"
  if command -v b3sum >/dev/null 2>&1; then
    b3sum --no-names "${fpath}"
  else
    "${FROB_E2E_TOOL}" cmp sha256 "${fpath}"
  fi
}

# Build an offline synthetic scratch workspace copying `franken-stack.lock` and
# `registries/siblings.toml` from the repository (never writes into any sibling repo).
_init_plant_env() {
  local label="$1"
  local base="${E2E_WORK_DIR}/plant_${label}"
  local repo="${base}/repo"
  local reg="${base}/local-reg"
  mkdir -p "${repo}/crates" "${repo}/registries" "${reg}" "${base}/.cargo"
  cp "${E2E_REPO_ROOT}/franken-stack.lock" "${repo}/franken-stack.lock"
  cp "${E2E_REPO_ROOT}/registries/siblings.toml" "${repo}/registries/siblings.toml"
  cat > "${base}/.cargo/config.toml" <<EOF
[source.crates-io]
replace-with = "local-reg"

[source.local-reg]
directory = "${reg}"
EOF
  printf '%s' "${base}"
}

_write_reg_crate() {
  local reg="$1" name="$2" ver="$3" extra="$4"
  local dir="${reg}/${name}-${ver}"
  mkdir -p "${dir}/src"
  cat > "${dir}/Cargo.toml" <<EOF
[package]
name = "${name}"
version = "${ver}"
edition = "2021"

${extra}
EOF
  printf '// synthetic registry crate\n' > "${dir}/src/lib.rs"
  printf '{"files":{}}\n' > "${dir}/.cargo-checksum.json"
}

_write_sib_crate() {
  local base="$1" rel="$2" name="$3" ver="$4" extra="$5"
  local dir="${base}/${rel}"
  mkdir -p "${dir}/src"
  cat > "${dir}/Cargo.toml" <<EOF
[package]
name = "${name}"
version = "${ver}"
edition = "2021"

${extra}
EOF
  printf '// synthetic sibling crate\n' > "${dir}/src/lib.rs"
}

_write_ws_crate() {
  local repo="$1" rel="$2" name="$3" layer_toml="$4" rank="$5" extra="$6"
  local dir="${repo}/${rel}"
  mkdir -p "${dir}/src"
  {
    printf '[package]\nname = "%s"\nversion = "0.0.1"\nedition = "2024"\n\n' "${name}"
    if [ -n "${layer_toml}" ]; then
      printf '[package.metadata.frob]\nlayer = %s\nrank = %s\n\n' "${layer_toml}" "${rank}"
    fi
    printf '%s\n' "${extra}"
  } > "${dir}/Cargo.toml"
  printf '// synthetic workspace crate\n' > "${dir}/src/lib.rs"
}

_write_root_workspace() {
  local repo="$1" exclude_line="$2"
  {
    printf '[workspace]\nresolver = "3"\nmembers = ["crates/*"]\n'
    if [ -n "${exclude_line}" ]; then
      printf '%s\n' "${exclude_line}"
    fi
  } > "${repo}/Cargo.toml"
}

_gen_lock() {
  local ws_dir="$1"
  local prev_dir="${PWD}"
  LOCK_SEQ=$((LOCK_SEQ + 1))
  local lock_name="gen_lock_${LOCK_SEQ}"
  cd "${ws_dir}" || return 1
  e2e_cmd --name "${lock_name}" -- \
    cargo generate-lockfile --offline --manifest-path "${ws_dir}/Cargo.toml"
  cd "${prev_dir}" || return 1
  e2e_assert_eq "${lock_name}_exit_0" "${E2E_LAST_EXIT}" "0"
}

# Run `cargo xtask check-deps --root <repo> --json`, assert exit 2 and expected rule,
# and log expected/reported rule, exit code, dependency path, and manifest BLAKE3.
_run_plant_check() {
  local label="$1"
  local expected_rule="$2"
  local repo_dir="$3"
  local manifest_path="$4"
  local diff_note="$5"
  local exact_only="${6:-0}"

  PLANTS_TOTAL=$((PLANTS_TOTAL + 1))

  local manifest_b3
  manifest_b3="$(_hash_manifest "${manifest_path}")"

  local out_copy="${E2E_WORK_DIR}/out_${label}.json"
  e2e_cmd --name "check_deps_${label}" --expect-exit 2 -- \
    env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
    cargo xtask check-deps --root "${repo_dir}" --json

  cp "${E2E_LAST_STDOUT}" "${out_copy}" 2>/dev/null || : > "${out_copy}"

  local parsed
  parsed="$(python3 - "${out_copy}" "${expected_rule}" "${exact_only}" <<'PY'
import json, sys
path, expected, exact_only = sys.argv[1], sys.argv[2], sys.argv[3] == "1"
try:
    doc = json.load(open(path, "r", encoding="utf-8"))
except Exception as exc:
    print(f"PARSE_ERR:{exc}\tNONE\tNONE")
    sys.exit(0)
findings = doc.get("findings", [])
rules = [f.get("rule", "") for f in findings]
matched = [f for f in findings if f.get("rule") == expected]
dep_path = matched[0].get("subject", "") if matched else (findings[0].get("subject", "") if findings else "NONE")
rules_csv = ",".join(rules) if rules else "NONE"
ok = (rules == [expected]) if exact_only else (expected in rules)
print(f"{'CAUGHT' if ok else 'MISSED'}\t{rules_csv}\t{dep_path}")
PY
)"
  local verdict reported_rules dep_path
  IFS="$(printf '\t')" read -r verdict reported_rules dep_path <<EOF
${parsed}
EOF

  local obs_summary="rule=${reported_rules} exit=${E2E_LAST_EXIT} dep_path=${dep_path} manifest_blake3=${manifest_b3}"
  local exp_summary="rule=${expected_rule} exit=2 dep_path=${dep_path} manifest_blake3=${manifest_b3}"

  if [ "${E2E_LAST_EXIT}" -eq 2 ] && [ "${verdict}" = "CAUGHT" ]; then
    PLANTS_CAUGHT=$((PLANTS_CAUGHT + 1))
    e2e_assert_eq "plant_${label}" "caught(${expected_rule}) ${obs_summary}" "caught(${expected_rule}) ${obs_summary}" \
      --nc "NC-N33c-09-${label}"
  else
    local miss_detail="MISSED scratch=${repo_dir} diff=${diff_note} output=${out_copy} observed=(${obs_summary}) expected=(${exp_summary})"
    e2e_assert_eq "plant_${label}" "${miss_detail}" "${exp_summary}" \
      --nc "NC-N33c-09-${label}"
  fi
}

# ---------------------------------------------------------------------------
# Step 1: Plant 1 — path dependency resolving asupersync 0.5.0 (`asup.v0_5_0`)
# ---------------------------------------------------------------------------
e2e_step "plant_asup_0_5_0" "Path dependency resolving asupersync 0.5.0 must fail with asup.v0_5_0"
B1="$(_init_plant_env "asup_0_5_0")"
_write_root_workspace "${B1}/repo" ""
_write_reg_crate "${B1}/local-reg" "asupersync" "0.5.0" ""
_write_sib_crate "${B1}" "frankensim/crates/fs-ga" "fs-ga" "0.1.0" '[dependencies]
asupersync = "=0.5.0"'
_write_ws_crate "${B1}/repo" "crates/frob-lie" "frob-lie" "1" "1" '[dependencies]
fs-ga = { path = "../../../frankensim/crates/fs-ga", default-features = false }'
_gen_lock "${B1}/repo"
_run_plant_check "asup_0_5_0" "asup.v0_5_0" "${B1}/repo" "${B1}/repo/crates/frob-lie/Cargo.toml" \
  "frob-lie -> fs-ga -> asupersync =0.5.0" "0"

# ---------------------------------------------------------------------------
# Step 2: Plant 2 — frob-tui enabling ftui-runtime/asupersync-executor (`ftui.executor-feature`)
# ---------------------------------------------------------------------------
e2e_step "plant_ftui_executor" "frob-tui enabling ftui-runtime/asupersync-executor must fail with ftui.executor-feature"
B2="$(_init_plant_env "ftui_executor")"
_write_root_workspace "${B2}/repo" ""
_write_reg_crate "${B2}/local-reg" "ftui-runtime" "0.11.0" '[features]
default = []
runtime = []
native-backend = []
asupersync-executor = []'
_write_ws_crate "${B2}/repo" "crates/frob-tui" "frob-tui" "7" "2" '[dependencies]
ftui-runtime = { version = "=0.11.0", default-features = false, features = ["runtime", "native-backend", "asupersync-executor"] }'
_gen_lock "${B2}/repo"
_run_plant_check "ftui_executor" "ftui.executor-feature" "${B2}/repo" "${B2}/repo/crates/frob-tui/Cargo.toml" \
  "frob-tui enables ftui-runtime/asupersync-executor" "1"

# ---------------------------------------------------------------------------
# Step 3: Plant 3 — frob-design depending on fnx-algorithms (`suite.forbidden-package`)
# ---------------------------------------------------------------------------
e2e_step "plant_fnx_algorithms" "frob-design depending on fnx-algorithms must fail with suite.forbidden-package"
B3="$(_init_plant_env "fnx_algorithms")"
_write_root_workspace "${B3}/repo" ""
_write_sib_crate "${B3}" "franken_networkx/crates/fnx-algorithms" "fnx-algorithms" "0.1.0" ""
_write_ws_crate "${B3}/repo" "crates/frob-design" "frob-design" "6" "3" '[dependencies]
fnx-algorithms = { path = "../../../franken_networkx/crates/fnx-algorithms" }'
_gen_lock "${B3}/repo"
_run_plant_check "fnx_algorithms" "suite.forbidden-package" "${B3}/repo" "${B3}/repo/crates/frob-design/Cargo.toml" \
  "frob-design -> ../franken_networkx/crates/fnx-algorithms" "1"

# ---------------------------------------------------------------------------
# Step 4: Plant 4 — frob-core normal dep on franken-kernel (`L-4.foundation-confined`)
# ---------------------------------------------------------------------------
e2e_step "plant_franken_kernel" "Normal dep of frob-core on franken-kernel must fail as L-4.foundation-confined only"
B4="$(_init_plant_env "franken_kernel")"
_write_root_workspace "${B4}/repo" ""
_write_sib_crate "${B4}" "asupersync/franken_kernel" "franken-kernel" "0.6.0" ""
_write_ws_crate "${B4}/repo" "crates/frob-core" "frob-core" "0" "1" '[dependencies]
franken-kernel = { path = "../../../asupersync/franken_kernel" }'
_gen_lock "${B4}/repo"
_run_plant_check "franken_kernel_normal" "L-4.foundation-confined" "${B4}/repo" "${B4}/repo/crates/frob-core/Cargo.toml" \
  "frob-core normal dep on ../asupersync/franken_kernel" "1"

# ---------------------------------------------------------------------------
# Step 5: Plant 5 — derive/ dependency on fj-egraph (`suite.forbidden-package`)
# ---------------------------------------------------------------------------
e2e_step "plant_derive_fj_egraph" "derive/ dependency on fj-egraph must fail with suite.forbidden-package"
B5="$(_init_plant_env "derive_fj_egraph")"
_write_root_workspace "${B5}/repo" 'exclude = ["derive"]'
_write_ws_crate "${B5}/repo" "crates/frob-core" "frob-core" "0" "1" ""
_write_sib_crate "${B5}" "frankenjax/crates/fj-egraph" "fj-egraph" "0.1.0" ""
_write_ws_crate "${B5}/repo" "derive/frob-derive" "frob-derive" '"annex"' "1" '[dependencies]
fj-egraph = { path = "../../../frankenjax/crates/fj-egraph" }'
_gen_lock "${B5}/repo"
_gen_lock "${B5}/repo/derive/frob-derive"
_run_plant_check "derive_fj_egraph" "suite.forbidden-package" "${B5}/repo" "${B5}/repo/derive/frob-derive/Cargo.toml" \
  "derive/frob-derive -> ../frankenjax/crates/fj-egraph" "1"

# ---------------------------------------------------------------------------
# Step 6: Plant 6 — optional feature cloth-oracle adding ftx-drape (`suite.forbidden-package`)
# ---------------------------------------------------------------------------
e2e_step "plant_optional_cloth_oracle" "Optional feature cloth-oracle adding ftx-drape must fail with suite.forbidden-package"
B6="$(_init_plant_env "cloth_oracle")"
_write_root_workspace "${B6}/repo" ""
_write_sib_crate "${B6}" "franken_textiles/crates/ftx-drape" "ftx-drape" "0.1.0" ""
_write_ws_crate "${B6}/repo" "crates/frob-dyn" "frob-dyn" "3" "3" '[features]
default = []
cloth-oracle = ["dep:ftx-drape"]

[dependencies]
ftx-drape = { path = "../../../franken_textiles/crates/ftx-drape", optional = true }'
_gen_lock "${B6}/repo"
_run_plant_check "optional_cloth_oracle" "suite.forbidden-package" "${B6}/repo" "${B6}/repo/crates/frob-dyn/Cargo.toml" \
  "frob-dyn optional feature cloth-oracle -> ../franken_textiles/crates/ftx-drape" "1"

# ---------------------------------------------------------------------------
# Step 7: Plant 7 — unlinked asupersync 0.5.0 in Cargo.lock (`asup.v0_5_0`)
# ---------------------------------------------------------------------------
e2e_step "plant_unlinked_lockfile" "Unlinked asupersync 0.5.0 entry in Cargo.lock must fail with asup.v0_5_0"
B7="$(_init_plant_env "unlinked_lock")"
_write_root_workspace "${B7}/repo" ""
_write_ws_crate "${B7}/repo" "crates/frob-core" "frob-core" "0" "1" ""
_gen_lock "${B7}/repo"
cat >> "${B7}/repo/Cargo.lock" <<'EOF'

[[patch.unused]]
name = "asupersync"
version = "0.5.0"
EOF
_run_plant_check "unlinked_asup_lock" "asup.v0_5_0" "${B7}/repo" "${B7}/repo/Cargo.lock" \
  "Cargo.lock [[patch.unused]] asupersync 0.5.0" "1"

# ---------------------------------------------------------------------------
# Step 8: Plant 8 — third-party egg crate (`suite.egg`)
# ---------------------------------------------------------------------------
e2e_step "plant_egg" "Third-party egg crate must fail with suite.egg"
B8="$(_init_plant_env "egg")"
_write_root_workspace "${B8}/repo" ""
_write_reg_crate "${B8}/local-reg" "egg" "0.9.5" ""
_write_ws_crate "${B8}/repo" "crates/frob-kir" "frob-kir" "2" "1" '[dependencies]
egg = "=0.9.5"'
_gen_lock "${B8}/repo"
_run_plant_check "egg_crate" "suite.egg" "${B8}/repo" "${B8}/repo/crates/frob-kir/Cargo.toml" \
  "frob-kir -> egg =0.9.5" "1"

# ---------------------------------------------------------------------------
# Step 9: Clean repository tree passes check-deps with all optional features
# ---------------------------------------------------------------------------
e2e_step "clean_tree_all_features" "Clean tree passes cargo xtask check-deps with all optional features enabled"
e2e_cmd --name clean_tree_check_deps -- \
  env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
  cargo xtask check-deps --root "${E2E_REPO_ROOT}" --json
e2e_assert_eq "clean_tree_exit_0" "${E2E_LAST_EXIT}" "0"
e2e_assert_json "clean_tree_ok_true" "/ok" "true"

e2e_metric "plants_planted" "${PLANTS_TOTAL}" "count"
e2e_metric "plants_caught" "${PLANTS_CAUGHT}" "count"
e2e_assert_eq "all_plants_caught" "${PLANTS_CAUGHT}" "${PLANTS_TOTAL}"
