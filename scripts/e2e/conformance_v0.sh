#!/usr/bin/env bash
# e2e: bead=BD-025 milestone=M0 labels=gauntlet,registry gates=GATE-GAUNT-01 plants=empty-controls,dangling-control,wrong-command-forms
#
# End-to-end verification of frob-conformance v0 (BD-025):
#   1. Build frob-conformance (all targets, host-only, no device annex).
#   2. Run registry_check on the clean tree (exit 0) and record gate/control counts.
#   3. In a scratch copy, empty the controls list of one not-run gate (exit 0,
#      prints `not-eligible`), then set that gate's status to `pass` (non-zero,
#      reason `no-control`).
#   4. In a fresh scratch copy, plant a dangling control identifier (non-zero,
#      reason `dangling-control`).
#   5. Run no_conformance_command and binaries_declared on the clean tree (exit 0)
#      and record scripts_scanned.
#   6. Plant each of the five refused command forms in scratch scripts (non-zero,
#      each line and its replacement named in output) and once more together.
#   7. Emit and verify the evidence block (claim=known-answer, class=Tested, D0, seed=0).
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "conformance_v0" "BD-025"
e2e_gate "GATE-GAUNT-01"

CARGO_TEST=(cargo test --manifest-path "${E2E_REPO_ROOT}/Cargo.toml" -p frob-conformance)

# Cargo and its offload shim discover toolchain and config from their cwd.
_in_workspace() {
  local work_dir="$PWD" status=0
  cd "${E2E_REPO_ROOT}" || return
  "$@" || status=$?
  cd "${work_dir}" || return
  return "${status}"
}

# Copy workspace layout into a fresh scratch directory (without target/ or .git/).
_make_scratch_ws() {
  local label="$1"
  local dst="${E2E_WORK_DIR}/ws_${label}"
  mkdir -p "${dst}"
  cp "${E2E_REPO_ROOT}/Cargo.toml" "${dst}/Cargo.toml"
  if [ -f "${E2E_REPO_ROOT}/franken-stack.lock" ]; then
    cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  fi
  local d
  for d in crates fixtures registries scripts xtask; do
    if [ -d "${E2E_REPO_ROOT}/${d}" ]; then
      cp -R "${E2E_REPO_ROOT}/${d}" "${dst}/${d}"
    fi
  done
  printf '%s' "${dst}"
}

_combined_output() {
  if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    cat "${E2E_LAST_STDOUT}"
  fi
  if [ -n "${E2E_LAST_STDERR:-}" ] && [ -f "${E2E_LAST_STDERR}" ]; then
    cat "${E2E_LAST_STDERR}"
  fi
}

_lint_diagnostic() {
  local rule="$1" file="$2" line="$3" replacement="$4"
  _combined_output | grep -F -- "[rule-${rule}] scripts/${file}:${line}:" \
    | grep -F -- ' -> use: ' | grep -F -- "${replacement}" >/dev/null
}

_assert_lint_diagnostic() {
  local detected=0
  if _lint_diagnostic "$@"; then detected=1; fi
  e2e_assert_eq "cmd_rule${1}_line${3}_replacement_printed" "${detected}" "1" --gate "GATE-GAUNT-01"
}

# ---------------------------------------------------------------------------
# Step 1: Build the crate (host-only, no device annex)
# ---------------------------------------------------------------------------
e2e_step "build_crate" "Build frob-conformance test targets with no device annex"
_in_workspace e2e_cmd --name build_frob_conformance -- \
  "${CARGO_TEST[@]}" --no-run
e2e_assert_eq "build_frob_conformance_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 2: Run registry_check on the clean tree and log gate/control counts
# ---------------------------------------------------------------------------
e2e_step "clean_registry_check" "Run registry_check on the clean repository tree"
_in_workspace e2e_cmd --name clean_registry_check -- \
  "${CARGO_TEST[@]}" --test registry_check -- --nocapture
e2e_assert_eq "clean_registry_check_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _counts="$(python3 - "${E2E_LAST_STDOUT}" <<'PY'
import re, sys
txt = open(sys.argv[1], "r", encoding="utf-8", errors="replace").read()
m = re.search(r"registry_check:\s+gates=(\d+)\s+controls=(\d+)", txt)
if m:
    print(f"{m.group(1)} {m.group(2)}")
else:
    print("0 0")
PY
  )"
  read -r _gates_loaded _controls_loaded <<< "${_counts}"
  e2e_metric "gates_loaded" "${_gates_loaded}" "gates"
  e2e_metric "controls_loaded" "${_controls_loaded}" "controls"
fi

# ---------------------------------------------------------------------------
# Step 3: Empty controls of one not-run gate (not-eligible), then set status=pass (no-control)
# ---------------------------------------------------------------------------
e2e_step "plant_empty_controls" "Empty controls on one not-run gate -> not-eligible, then status=pass -> no-control"
WS_EMPTY="$(_make_scratch_ws "empty_controls")"

_pass_gates="${E2E_WORK_DIR}/empty_controls_pass.toml"
_empty_gate="$(python3 - "${WS_EMPTY}" "${_pass_gates}" <<'PY'
import json, re, sys, tomllib
from pathlib import Path

root, pass_path = map(Path, sys.argv[1:])
gates_path = root / "registries/gates.toml"
ctrls_path = root / "crates/frob-conformance/registry/controls.toml"

def split_rows(text, table):
    return re.split(rf"(?m)(?=^[ \t]*\[\[{table}\]\])", text)

def replace_array(row, field, values):
    row, count = re.subn(rf"(?m)^[ \t]*{field}[ \t]*=[ \t]*\[[^\]]*\]",
                          f"{field} = {json.dumps(values)}", row, count=1)
    if not count:
        raise SystemExit(f"missing {field} array in scratch registry row")
    return row

gates = split_rows(gates_path.read_text(encoding="utf-8"), "gate")
controls_text = ctrls_path.read_text(encoding="utf-8")
controls = split_rows(controls_text, "control")
control_rows = tomllib.loads(controls_text).get("control", [])

# A first, scratch-only pass activates fixture validation and tests gate selection.
fixture = "fixtures/gauntlet/v0_empty_controls.toml"
fixture_text = (root / fixture).read_text(encoding="utf-8")
if "# synthetic" not in fixture_text.splitlines()[:8]:
    raise SystemExit(f"scratch fixture {fixture} lacks its synthetic header")
sentinel = "GATE-GAUNT-99"
if any(tomllib.loads(row)["gate"][0]["id"] == sentinel for row in gates[1:]):
    raise SystemExit(f"scratch pass gate ID {sentinel} is already registered")
ordinal = 1 + max((int(c["id"].removeprefix("NC-")) for c in control_rows), default=0)
if ordinal >= 9999:
    raise SystemExit("no unused control ID before the dangling-control plant")
control_id = f"NC-{ordinal:04d}"
pass_row = split_rows(fixture_text, "gate")[1]
pass_row = re.sub(r'(?m)^id = ".*"$', f'id = "{sentinel}"', pass_row, count=1)
gates.insert(1, replace_array(pass_row, "controls", [control_id]).rstrip() + "\n\n")
recorded = tomllib.loads(pass_row)["gate"][0]["part"][0]["recorded"]
controls.append(f'''
[[control]]
id = "{control_id}"
seed = "GATE-GAUNT-01"
gate = ["{sentinel}"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted pass gate with an empty controls list"
expect = "registry_check fails with no-control"
fixture = "{fixture}"
due = "M0"
state = "runnable"
last_seen = {{ caught = true, date = "{recorded['date']}", session = "{recorded['session']}" }}
''')

for index, row in enumerate(gates[1:], 1):
    gate = tomllib.loads(row)["gate"][0]
    parts = gate.get("part", [])
    if all(part["status"] == "not-run" for part in parts):
        break
else:
    raise SystemExit("empty-controls plant requires a gate with only not-run parts")

gate_id = gate["id"]
gates[index] = replace_array(row, "controls", [])
kept = controls[:1]
for block in controls[1:]:
    linked = tomllib.loads(block)["control"][0]["gate"]
    if gate_id not in linked:
        kept.append(block)
    elif remaining := [other for other in linked if other != gate_id]:
        kept.append(replace_array(block, "gate", remaining))

gates_path.write_text("".join(gates), encoding="utf-8")
ctrls_path.write_text("".join(kept), encoding="utf-8")
if parts:
    gates[index] = re.sub(r'(?m)^[ \t]*status[ \t]*=[ \t]*"not-run"',
                          'status = "pass"', gates[index], count=1)
else:
    gates[index] = gates[index].rstrip() + f'''

[[gate.part]]
milestone = "{gate['milestone']}"
status = "pass"
measured = "planted-empty"
evidence = []
recorded = {{ date = "", session = "e2e-plant", implemented_by = "" }}

'''
pass_path.write_text("".join(gates), encoding="utf-8")
print(gate_id)
PY
)"

_in_workspace e2e_cmd --name empty_controls_not_run -- \
  env FROB_CONFORMANCE_ROOT="${WS_EMPTY}" \
  "${CARGO_TEST[@]}" --test registry_check -- registry_check_clean_tree --nocapture
e2e_assert_eq "empty_controls_not_run_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"
_has_not_eligible="0"
if _combined_output | grep -F "  ${_empty_gate} (" | grep -F "not-eligible (no listed controls)" >/dev/null; then
  _has_not_eligible="1"
fi
e2e_assert_eq "empty_controls_prints_not_eligible" "${_has_not_eligible}" "1" --gate "GATE-GAUNT-01"

# Flip that same gate part from not-run to pass -> must fail with `no-control`.
cp "${_pass_gates}" "${WS_EMPTY}/registries/gates.toml"

_in_workspace e2e_expect_caught "empty_controls_refusal" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_EMPTY}" \
  "${CARGO_TEST[@]}" --test registry_check -- registry_check_clean_tree --nocapture
_has_no_control="0"
if [ "${E2E_LAST_EXIT}" = 101 ] && _combined_output | grep -F "[no-control] ${_empty_gate}:" >/dev/null; then
  _has_no_control="1"
fi
# The external lane expects the detected plant to fail an assertion named for it.
_expected=1
if e2e_plant "empty-controls"; then _expected=0; fi
e2e_assert_eq "empty-controls" "${_has_no_control}" "${_expected}" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 4: Plant a dangling control identifier -> must fail with `dangling-control`
# ---------------------------------------------------------------------------
e2e_step "plant_dangling_control" "Plant a dangling control identifier in scratch gates.toml"
WS_DANGLING="$(_make_scratch_ws "dangling_control")"

python3 - "${WS_DANGLING}/registries/gates.toml" <<'PY'
import re, sys

p = sys.argv[1]
txt = open(p, "r", encoding="utf-8").read()
if "[[gate]]" not in txt:
    txt = txt.rstrip() + """

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "§38.1"
threshold = "every pass gate has a caught negative control"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-9999"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
"""
else:
    def _add_dangling(m):
        inner = m.group(1).strip()
        if not inner:
            return 'controls = ["NC-9999"]'
        return f'controls = [{inner}, "NC-9999"]'
    txt = re.sub(r'controls\s*=\s*\[([^\]]*)\]', _add_dangling, txt, count=1)
open(p, "w", encoding="utf-8").write(txt)
PY

_in_workspace e2e_expect_caught "dangling_control_refusal" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_DANGLING}" \
  "${CARGO_TEST[@]}" --test registry_check -- registry_check_clean_tree --nocapture
_has_dangling="0"
if [ "${E2E_LAST_EXIT}" = 101 ] && _combined_output | grep -F '[dangling-control] ' >/dev/null; then
  _has_dangling="1"
fi
_expected=1
if e2e_plant "dangling-control"; then _expected=0; fi
e2e_assert_eq "dangling-control" "${_has_dangling}" "${_expected}" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 5: Run no_conformance_command and binaries_declared on the clean tree
# ---------------------------------------------------------------------------
e2e_step "clean_command_and_binaries" "Run no_conformance_command and binaries_declared on clean tree"
_in_workspace e2e_cmd --name clean_no_conformance_command -- \
  "${CARGO_TEST[@]}" --test no_conformance_command -- --nocapture
e2e_assert_eq "clean_no_conformance_command_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _scanned="$(python3 - "${E2E_LAST_STDOUT}" <<'PY'
import re, sys
txt = open(sys.argv[1], "r", encoding="utf-8", errors="replace").read()
m = re.search(r"scripts_scanned=(\d+)", txt)
print(m.group(1) if m else "0")
PY
  )"
  e2e_metric "scripts_scanned" "${_scanned}" "scripts"
fi

_in_workspace e2e_cmd --name clean_binaries_declared -- \
  "${CARGO_TEST[@]}" --test binaries_declared
e2e_assert_eq "clean_binaries_declared_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 6: Plant the five refused command forms in scratch copies (one per rule + combined)
# ---------------------------------------------------------------------------
e2e_step "plant_wrong_command_forms" "Plant all five refused command forms in scratch scripts"

_frob="frob"
_conf="conformance"
_fc="frob-conformance"

# Rule 1: frob conformance run GATE-ARENA-05
WS_R1="$(_make_scratch_ws "cmd_rule1")"
{
  printf '#!/usr/bin/env bash\n'
  printf '%s %s run GATE-ARENA-05\n' "${_frob}" "${_conf}"
} > "${WS_R1}/scripts/planted_rule1.sh"
_in_workspace e2e_expect_caught "command_rule_1" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R1}" \
  "${CARGO_TEST[@]}" --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_assert_lint_diagnostic 1 planted_rule1.sh 2 '--test <tier>'

# Rule 2: frob-conformance in command position
WS_R2="$(_make_scratch_ws "cmd_rule2")"
{
  printf '#!/usr/bin/env bash\n'
  printf '%s run ground.identify --json\n' "${_fc}"
  printf 'x=1 %s record --gate GATE-X\n' "${_fc}"
} > "${WS_R2}/scripts/planted_rule2.sh"
_in_workspace e2e_expect_caught "command_rule_2" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R2}" \
  "${CARGO_TEST[@]}" --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_assert_lint_diagnostic 2 planted_rule2.sh 2 '--example record_gate'
_assert_lint_diagnostic 2 planted_rule2.sh 3 '--example record_gate'

# Rule 3: cargo run -p frob-conformance without --bin or --example
WS_R3="$(_make_scratch_ws "cmd_rule3")"
{
  printf '#!/usr/bin/env bash\n'
  printf 'cargo run -p %s -- pilots fundable --all --json\n' "${_fc}"
} > "${WS_R3}/scripts/planted_rule3.sh"
_in_workspace e2e_expect_caught "command_rule_3" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R3}" \
  "${CARGO_TEST[@]}" --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_assert_lint_diagnostic 3 planted_rule3.sh 2 '--example pilots'

# Rule 4: short tier target --test g1
WS_R4="$(_make_scratch_ws "cmd_rule4")"
{
  printf '#!/usr/bin/env bash\n'
  printf 'cargo test -p %s --test %s -- KA-501\n' "${_fc}" "g1"
} > "${WS_R4}/scripts/planted_rule4.sh"
_in_workspace e2e_expect_caught "command_rule_4" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R4}" \
  "${CARGO_TEST[@]}" --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_assert_lint_diagnostic 4 planted_rule4.sh 2 'g1_known_answers'

# Rule 5: unknown target (--example no_such_tool / --bin frob-conformance)
WS_R5="$(_make_scratch_ws "cmd_rule5")"
{
  printf '#!/usr/bin/env bash\n'
  printf 'cargo run -p %s --example %s\n' "${_fc}" "no_such_tool"
  printf 'cargo run -p %s --bin %s\n' "${_fc}" "${_fc}"
} > "${WS_R5}/scripts/planted_rule5.sh"
_in_workspace e2e_expect_caught "command_rule_5" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R5}" \
  "${CARGO_TEST[@]}" --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_assert_lint_diagnostic 5 planted_rule5.sh 2 'unknown-target'
_assert_lint_diagnostic 5 planted_rule5.sh 3 'unknown-target'

# Combined scratch script holding all five refused forms together
WS_ALL="$(_make_scratch_ws "cmd_all_five")"
{
  printf '#!/usr/bin/env bash\n'
  printf '%s %s run GATE-ARENA-05\n' "${_frob}" "${_conf}"
  printf '%s run ground.identify --json\n' "${_fc}"
  printf 'x=1 %s record --gate GATE-X\n' "${_fc}"
  printf 'cargo run -p %s -- pilots fundable --all --json\n' "${_fc}"
  printf 'cargo test -p %s --test %s -- KA-501\n' "${_fc}" "g1"
  printf 'cargo run -p %s --example %s\n' "${_fc}" "no_such_tool"
  printf 'cargo run -p %s --bin %s\n' "${_fc}" "${_fc}"
} > "${WS_ALL}/scripts/planted_all_five.sh"
_in_workspace e2e_expect_caught "combined_command_refusal" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_ALL}" \
  "${CARGO_TEST[@]}" --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_combined_detected=0
if [ "${E2E_LAST_EXIT}" = 101 ] &&
    _lint_diagnostic 1 planted_all_five.sh 2 '--test <tier>' &&
    _lint_diagnostic 2 planted_all_five.sh 3 '--example record_gate' &&
    _lint_diagnostic 2 planted_all_five.sh 4 '--example record_gate' &&
    _lint_diagnostic 3 planted_all_five.sh 5 '--example pilots' &&
    _lint_diagnostic 4 planted_all_five.sh 6 'g1_known_answers' &&
    _lint_diagnostic 5 planted_all_five.sh 7 'unknown-target' &&
    _lint_diagnostic 5 planted_all_five.sh 8 'unknown-target'; then
  _combined_detected=1
fi
_expected=1
if e2e_plant "wrong-command-forms"; then _expected=0; fi
e2e_assert_eq "wrong-command-forms" "${_combined_detected}" "${_expected}" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 7: Evidence block (claim=known-answer, class=Tested, D0, seed=0)
# ---------------------------------------------------------------------------
e2e_step "evidence_block" "Record and assert BD-025 evidence metadata"
_ev_file="${E2E_ARTIFACTS_DIR}/evidence.json"
printf '{"claim":"known-answer","class":"Tested","determinism":"D0","seed":%s,"gate":"GATE-GAUNT-01"}\n' "${E2E_SEED}" > "${_ev_file}"
e2e_cmd --name emit_evidence_json -- cat "${_ev_file}"
e2e_assert_json "evidence_claim" "/claim" "known-answer" --gate "GATE-GAUNT-01"
e2e_assert_json "evidence_class" "/class" "Tested" --gate "GATE-GAUNT-01"
e2e_assert_json "evidence_determinism" "/determinism" "D0" --gate "GATE-GAUNT-01"
e2e_assert_json "evidence_seed" "/seed" "0" --gate "GATE-GAUNT-01"
