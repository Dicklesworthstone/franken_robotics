#!/usr/bin/env bash
# e2e: bead=NJL-04 milestone=M0 labels=tether,verbs,registry gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=alias-row,cli-alias
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"
e2e_init verbs_one_name NJL-04 "$@"
e2e_gate GATE-TETHER-02

registry="${E2E_REPO_ROOT}/registries/verbs.toml"
dry="${E2E_DRY_RUN:-0}"

# Retired spellings and targets are bare registry names inside quoted TOML strings.
retired_rows() {
  awk '
    /^\[\[/ { retired = ($0 == "[[retired_name]]") }
    retired && /^name[ ]*=/ { split($0, names, "\""); name = names[2] }
    retired && /^use[ ]*=/ { split($0, targets, "\""); print name "|" targets[2] }
  ' "${registry}"
}

if e2e_plant alias-row; then
  e2e_step alias_row "Build a registry containing a retired command name"
  plant="${E2E_WORK_DIR}/alias-row.toml"
  cp "${registry}" "${plant}"
  # RL-03 fires on the name before the input and output schemas are loaded.
  cat >> "${plant}" <<'TOML'

[[verb]]
name = "autopsy.fork"
cluster = "autopsy"
kind = "read"
tier = "S"
capabilities = []
public = true
input_schema = "frob.autopsy.fork.in/1"
output_schema = "frob.autopsy.fork.out/1"
refusals = []
idempotent = true
cost_model = "static: read"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob autopsy fork --json"]
purpose = "Synthetic retired name."
use_when = "Testing RL-03."
owner = "frob-verbs"
TOML
  e2e_expect_caught alias-row -- env "FROB_VERBS_REGISTRY=${plant}" \
    "CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-alias-${E2E_RUN_ID}" \
    cargo check --offline -j 1 -p frob-verbs --manifest-path "${E2E_REPO_ROOT}/Cargo.toml"
  if [ "${dry}" != "1" ]; then
    caught=missing
    if grep -E 'retired-name.*autopsy\.fork|autopsy\.fork.*retired-name' "${E2E_LAST_STDERR}"; then
      caught=retired-name
    fi
    e2e_assert_eq alias_row_lint "${caught}" retired-name
  fi
  exit 0
fi

if e2e_plant cli-alias; then
  e2e_step cli_alias "Plant an alias attribute in the CLI source scan"
  plant="${E2E_WORK_DIR}/cli-alias.rs"
  printf '%s\n' '#[command(visible_alias = "old-name")]' 'fn generated() {}' > "${plant}"
  e2e_expect_caught cli-alias -- env "FROB_CLI_ALIAS_PLANT=${plant}" \
    "CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-alias-${E2E_RUN_ID}" \
    cargo test --offline -j 1 -p frob-cli --test no_aliases no_alias_attributes \
    --manifest-path "${E2E_REPO_ROOT}/Cargo.toml"
  if [ "${dry}" != "1" ]; then
    caught=missing
    if grep -F 'surface-alias: visible_alias' "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}"; then
      caught=surface-alias
    fi
    e2e_assert_eq cli_alias_lint "${caught}" surface-alias
  fi
  exit 0
fi

e2e_step retired_cli "Retired spellings return 801 with a replacement command"
if [ "${dry}" != "1" ] && [ ! -x "${FROB_BIN}" ]; then
  e2e_skip "missing binary: frob (BD-022)"
  exit 77
fi
while IFS='|' read -r name target; do
  IFS=. read -r -a spellings <<< "${name}"
  e2e_expect_refusal FROB-REF-801 3 -- "${FROB_BIN}" --json "${spellings[@]}"
  if [ "${dry}" != "1" ]; then
    e2e_assert_json "no_charge_${name}" /cost/cpu_core_s 0
    if [ -n "${target}" ]; then
      repair="$(e2e_get /refusal/repairs/0/action)"
      read -r binary cluster verb _ <<< "${repair}"
      e2e_assert_eq "first_repair_${name}" "${binary} ${cluster} ${verb}" "frob ${target//./ }"
    else
      e2e_assert_json "no_repair_${name}" /refusal/repairs '[]'
    fi
  fi
done < <(retired_rows)
e2e_expect_refusal FROB-REF-801 3 -- "${FROB_BIN}" --json nosuch verb
if [ "${dry}" != "1" ]; then
  e2e_assert_json unknown_has_no_replacement /refusal/repairs '[]'
fi

e2e_step retired_mcp "Check unknown retired MCP tools and tool-list exclusion"
e2e_skip "BD-023 supplies the MCP client and live tool registration"

e2e_step robot_docs "Check robot-docs for retired spellings"
e2e_cmd --name robot_docs -- "${FROB_BIN}" --json robot-docs
if [ "${dry}" != "1" ]; then
  e2e_assert_json robot_docs_ok /ok true
  while IFS='|' read -r name _; do
    found=absent
    if grep -Fq -e "${name}" -e "frob ${name//./ } " "${E2E_LAST_STDOUT}"; then
      found=present
    fi
    e2e_assert_eq "robot_docs_${name}" "${found}" absent
  done < <(retired_rows)
fi
