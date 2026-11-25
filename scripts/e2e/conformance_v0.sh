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
  for d in crates registries scripts; do
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

# ---------------------------------------------------------------------------
# Step 1: Build the crate (host-only, no device annex)
# ---------------------------------------------------------------------------
e2e_step "build_crate" "Build frob-conformance test targets with no device annex"
e2e_cmd --name build_frob_conformance -- \
  cargo test -p frob-conformance --no-run
e2e_assert_eq "build_frob_conformance_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 2: Run registry_check on the clean tree and log gate/control counts
#         (or against planted scratch workspace under E2E_PLANT_FAILURE)
# ---------------------------------------------------------------------------
e2e_step "clean_registry_check" "Run registry_check on the clean repository tree"
if e2e_plant "empty-controls"; then
  WS_PLANT_EMPTY="$(_make_scratch_ws "env_plant_empty_controls")"
  python3 - "${WS_PLANT_EMPTY}/registries/gates.toml" "${WS_PLANT_EMPTY}/crates/frob-conformance/registry/controls.toml" <<'PY'
import re, sys
gates_path, ctrls_path = sys.argv[1], sys.argv[2]
gates_txt = open(gates_path, "r", encoding="utf-8").read()
ctrls_txt = open(ctrls_path, "r", encoding="utf-8").read()
gates_txt = re.sub(r'controls\s*=\s*\[[^\]]*\]', 'controls = []', gates_txt, count=1)
m = re.search(r'\[\[gate\]\][^\[]*?id\s*=\s*"([^"]+)"', gates_txt, flags=re.S)
if m:
    gid = m.group(1)
    blocks = ctrls_txt.split("[[control]]")
    ctrls_txt = "[[control]]".join([blocks[0]] + [b for b in blocks[1:] if f'"{gid}"' not in b])
gates_txt = re.sub(r'status\s*=\s*"not-run"', 'status = "pass"', gates_txt, count=1)
gates_txt = re.sub(r'measured\s*=\s*""', 'measured = "planted-empty"', gates_txt, count=1)
gates_txt = re.sub(r'session\s*=\s*""', 'session = "e2e-plant"', gates_txt, count=1)
open(gates_path, "w", encoding="utf-8").write(gates_txt)
open(ctrls_path, "w", encoding="utf-8").write(ctrls_txt)
PY
  e2e_cmd --name clean_registry_check -- \
    env "FROB_CONFORMANCE_ROOT=${WS_PLANT_EMPTY}" \
    cargo test -p frob-conformance --test registry_check -- registry_check_clean_tree --nocapture
elif e2e_plant "dangling-control"; then
  WS_PLANT_DANGLE="$(_make_scratch_ws "env_plant_dangling_control")"
  python3 - "${WS_PLANT_DANGLE}/registries/gates.toml" <<'PY'
import re, sys
p = sys.argv[1]
txt = open(p, "r", encoding="utf-8").read()
def _add(m):
    inner = m.group(1).strip()
    return 'controls = ["NC-9999"]' if not inner else f'controls = [{inner}, "NC-9999"]'
txt = re.sub(r'controls\s*=\s*\[([^\]]*)\]', _add, txt, count=1)
open(p, "w", encoding="utf-8").write(txt)
PY
  e2e_cmd --name clean_registry_check -- \
    env "FROB_CONFORMANCE_ROOT=${WS_PLANT_DANGLE}" \
    cargo test -p frob-conformance --test registry_check -- registry_check_clean_tree --nocapture
else
  e2e_cmd --name clean_registry_check -- \
    cargo test -p frob-conformance --test registry_check -- --nocapture
fi
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

python3 - "${WS_EMPTY}/registries/gates.toml" "${WS_EMPTY}/crates/frob-conformance/registry/controls.toml" <<'PY'
import re, sys

gates_path, ctrls_path = sys.argv[1], sys.argv[2]
gates_txt = open(gates_path, "r", encoding="utf-8").read()
ctrls_txt = open(ctrls_path, "r", encoding="utf-8").read()

if "[[gate]]" not in gates_txt:
    gates_txt = gates_txt.rstrip() + """

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "§38.1"
threshold = "every pass gate has a caught negative control"
milestone = "M0"
tier = ["G0:host"]
controls = []

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
"""
else:
    # Empty controls on the first [[gate]] block.
    gates_txt = re.sub(r'controls\s*=\s*\[[^\]]*\]', 'controls = []', gates_txt, count=1)
    # Remove the emptied gate from controls.toml references so only not-eligible / no-control fires.
    m = re.search(r'\[\[gate\]\][^\[]*?id\s*=\s*"([^"]+)"', gates_txt, flags=re.S)
    if m:
        gid = m.group(1)
        blocks = ctrls_txt.split("[[control]]")
        kept = [blocks[0]]
        for blk in blocks[1:]:
            if f'"{gid}"' not in blk:
                kept.append(blk)
        ctrls_txt = "[[control]]".join(kept)

open(gates_path, "w", encoding="utf-8").write(gates_txt)
open(ctrls_path, "w", encoding="utf-8").write(ctrls_txt)
PY

e2e_cmd --name empty_controls_not_run -- \
  env FROB_CONFORMANCE_ROOT="${WS_EMPTY}" \
  cargo test -p frob-conformance --test registry_check -- registry_check_clean_tree --nocapture
e2e_assert_eq "empty_controls_not_run_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"
_has_not_eligible="0"
if _combined_output | grep -Fq "not-eligible"; then
  _has_not_eligible="1"
fi
e2e_assert_eq "empty_controls_prints_not_eligible" "${_has_not_eligible}" "1" --gate "GATE-GAUNT-01"

# Flip that same gate part from not-run to pass -> must fail with `no-control`.
python3 - "${WS_EMPTY}/registries/gates.toml" <<'PY'
import re, sys

p = sys.argv[1]
txt = open(p, "r", encoding="utf-8").read()
txt = re.sub(r'status\s*=\s*"not-run"', 'status = "pass"', txt, count=1)
txt = re.sub(r'measured\s*=\s*""', 'measured = "planted-empty"', txt, count=1)
txt = re.sub(r'session\s*=\s*""', 'session = "e2e-plant"', txt, count=1)
open(p, "w", encoding="utf-8").write(txt)
PY

e2e_expect_caught "empty-controls" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_EMPTY}" \
  cargo test -p frob-conformance --test registry_check -- registry_check_clean_tree --nocapture
_has_no_control="0"
if _combined_output | grep -Fq "no-control"; then
  _has_no_control="1"
fi
e2e_assert_eq "empty_controls_reason_no_control" "${_has_no_control}" "1" --gate "GATE-GAUNT-01"

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

e2e_expect_caught "dangling-control" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_DANGLING}" \
  cargo test -p frob-conformance --test registry_check -- registry_check_clean_tree --nocapture
_has_dangling="0"
if _combined_output | grep -Fq "dangling-control"; then
  _has_dangling="1"
fi
e2e_assert_eq "dangling_control_reason" "${_has_dangling}" "1" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 5: Run no_conformance_command and binaries_declared on the clean tree
# ---------------------------------------------------------------------------
e2e_step "clean_command_and_binaries" "Run no_conformance_command and binaries_declared on clean tree"
if e2e_plant "wrong-command-forms"; then
  WS_PLANT_CMD="$(_make_scratch_ws "env_plant_wrong_cmd")"
  {
    printf '#!/usr/bin/env bash\n'
    printf '%s %s run GATE-ARENA-05\n' "frob" "conformance"
  } > "${WS_PLANT_CMD}/scripts/planted_env_rule1.sh"
  e2e_cmd --name clean_no_conformance_command -- \
    env "FROB_CONFORMANCE_ROOT=${WS_PLANT_CMD}" \
    cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
else
  e2e_cmd --name clean_no_conformance_command -- \
    cargo test -p frob-conformance --test no_conformance_command -- --nocapture
fi
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

e2e_cmd --name clean_binaries_declared -- \
  cargo test -p frob-conformance --test binaries_declared
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
e2e_expect_caught "wrong-command-forms:rule-1" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R1}" \
  cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_r1_ok="0"
if _combined_output | grep -Fq -- "--test <tier>"; then _r1_ok="1"; fi
e2e_assert_eq "cmd_rule1_replacement_printed" "${_r1_ok}" "1" --gate "GATE-GAUNT-01"

# Rule 2: frob-conformance in command position
WS_R2="$(_make_scratch_ws "cmd_rule2")"
{
  printf '#!/usr/bin/env bash\n'
  printf '%s run ground.identify --json\n' "${_fc}"
  printf 'x=1 %s record --gate GATE-X\n' "${_fc}"
} > "${WS_R2}/scripts/planted_rule2.sh"
e2e_expect_caught "wrong-command-forms:rule-2" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R2}" \
  cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_r2_ok="0"
if _combined_output | grep -Fq -- "--example record_gate"; then _r2_ok="1"; fi
e2e_assert_eq "cmd_rule2_replacement_printed" "${_r2_ok}" "1" --gate "GATE-GAUNT-01"

# Rule 3: cargo run -p frob-conformance without --bin or --example
WS_R3="$(_make_scratch_ws "cmd_rule3")"
{
  printf '#!/usr/bin/env bash\n'
  printf 'cargo run -p %s -- pilots fundable --all --json\n' "${_fc}"
} > "${WS_R3}/scripts/planted_rule3.sh"
e2e_expect_caught "wrong-command-forms:rule-3" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R3}" \
  cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_r3_ok="0"
if _combined_output | grep -Fq -- "--example pilots"; then _r3_ok="1"; fi
e2e_assert_eq "cmd_rule3_replacement_printed" "${_r3_ok}" "1" --gate "GATE-GAUNT-01"

# Rule 4: short tier target --test g1
WS_R4="$(_make_scratch_ws "cmd_rule4")"
{
  printf '#!/usr/bin/env bash\n'
  printf 'cargo test -p %s --test %s -- KA-501\n' "${_fc}" "g1"
} > "${WS_R4}/scripts/planted_rule4.sh"
e2e_expect_caught "wrong-command-forms:rule-4" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R4}" \
  cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_r4_ok="0"
if _combined_output | grep -Fq "g1_known_answers"; then _r4_ok="1"; fi
e2e_assert_eq "cmd_rule4_replacement_printed" "${_r4_ok}" "1" --gate "GATE-GAUNT-01"

# Rule 5: unknown target (--example no_such_tool / --bin frob-conformance)
WS_R5="$(_make_scratch_ws "cmd_rule5")"
{
  printf '#!/usr/bin/env bash\n'
  printf 'cargo run -p %s --example %s\n' "${_fc}" "no_such_tool"
  printf 'cargo run -p %s --bin %s\n' "${_fc}" "${_fc}"
} > "${WS_R5}/scripts/planted_rule5.sh"
e2e_expect_caught "wrong-command-forms:rule-5" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_R5}" \
  cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture
_r5_ok="0"
if _combined_output | grep -Fq "unknown-target"; then _r5_ok="1"; fi
e2e_assert_eq "cmd_rule5_replacement_printed" "${_r5_ok}" "1" --gate "GATE-GAUNT-01"

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
e2e_expect_caught "wrong-command-forms" --gate "GATE-GAUNT-01" -- \
  env FROB_CONFORMANCE_ROOT="${WS_ALL}" \
  cargo test -p frob-conformance --test no_conformance_command -- no_conformance_command_clean_tree --nocapture

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
