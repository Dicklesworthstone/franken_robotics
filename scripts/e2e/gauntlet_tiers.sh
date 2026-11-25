#!/usr/bin/env bash
# e2e: bead=N38a-02 milestone=M0 labels=gauntlet gates=GATE-GAUNT-01 plants=drop-control
#
# End-to-end verification of Gauntlet conformance tiers G0-G5, execution sites,
# test selection, and hash-named receipts (`N38a-02`, `frob.gauntlet.receipt/1`):
#   1. Run the `tier_runner` unit-test suite (all 13 tests).
#   2. Run each of the six tier targets (`g0_laws`..`g5_determinism`) with a
#      small filter via `e2e_tier`, parse the final stdout line `receipt: <path>`,
#      verify each receipt file exists and its 64-hex BLAKE3 stem matches `e2e_tier`,
#      record `stack_receipt`, `stack_dev`, `build_identity`, and per-tier counts.
#   3. Plant `drop-control` in a scratch registry (emptying `controls = []` on
#      `GATE-GAUNT-01`), run `g0_laws -- GATE-GAUNT-01` via `e2e_tier`, and require
#      the gate line in the emitted receipt to become `not-eligible`.
#   4. Verify empty selection (`KA-999999`) exits non-zero (caught negative control).
#   5. Emit structured summary + evidence metrics.
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "gauntlet_tiers" "N38a-02"
e2e_gate "GATE-GAUNT-01"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"

_make_scratch_registry() {
  local label="$1"
  local dst="${E2E_WORK_DIR}/ws_${label}"
  mkdir -p "${dst}/registries" "${dst}/crates/frob-conformance/registry"
  cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  cp "${E2E_REPO_ROOT}/rust-toolchain.toml" "${dst}/rust-toolchain.toml"
  cp "${E2E_REPO_ROOT}/registries/gates.toml" "${dst}/registries/gates.toml"
  cp "${E2E_REPO_ROOT}/crates/frob-conformance/registry/controls.toml" \
    "${dst}/crates/frob-conformance/registry/controls.toml"
  printf '%s' "${dst}"
}

_empty_gate_controls() {
  local gates_toml="$1"
  local gate_id="$2"
  python3 - "${gates_toml}" "${gate_id}" <<'PY'
import re, sys
p, target_id = sys.argv[1], sys.argv[2]
txt = open(p, "r", encoding="utf-8").read()
pat = re.compile(
    r'(\[\[gate\]\]\s*\nid\s*=\s*"' + re.escape(target_id) + r'".*?controls\s*=\s*)\[[^\]]*\]',
    re.DOTALL,
)
new_txt, n = pat.subn(r'\1[]', txt, count=1)
if n != 1:
    sys.stderr.write(f"failed to locate controls for {target_id} in {p}\n")
    sys.exit(2)
open(p, "w", encoding="utf-8").write(new_txt)
PY
}

_extract_receipt_path() {
  local stdout_file="$1"
  python3 - "${stdout_file}" "${E2E_REPO_ROOT}" <<'PY'
import os, sys
stdout_path, repo_root = sys.argv[1], sys.argv[2]
lines = [ln.strip() for ln in open(stdout_path, "r", encoding="utf-8", errors="replace").read().splitlines() if ln.strip()]
if not lines:
    sys.exit(1)
last = lines[-1]
if not last.startswith("receipt: "):
    sys.exit(2)
rel_or_abs = last[len("receipt: "):].strip()
full = rel_or_abs if os.path.isabs(rel_or_abs) else os.path.join(repo_root, rel_or_abs)
print(full)
PY
}

_inspect_receipt() {
  local receipt_path="$1"
  python3 - "${receipt_path}" <<'PY'
import json, os, sys
p = sys.argv[1]
stem = os.path.basename(p)
if not stem.endswith(".json") or len(stem) != 69:
    sys.exit(3)
digest = stem[:-5]
if not all(c in "0123456789abcdef" for c in digest):
    sys.exit(4)
data = json.load(open(p, "r", encoding="utf-8"))
counts = data.get("counts", {})
gates = data.get("gates", [])
gate0_line = gates[0]["line"] if gates else ""
print(
    f"{digest}\t{data.get('schema','')}\t{data.get('stack_receipt','')}\t"
    f"{str(data.get('stack_dev','')).lower()}\t{data.get('build_identity','')}\t"
    f"{counts.get('pass',0)}\t{counts.get('fail',0)}\t{counts.get('skipped',0)}\t{gate0_line}"
)
PY
}

# ---------------------------------------------------------------------------
# Step 1: Run unit tests (`tier_runner`)
# ---------------------------------------------------------------------------
e2e_step "tier_runner_units" "Run N38a-02 tier_runner unit tests"
e2e_cmd --name tier_runner_units -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance --test tier_runner
e2e_assert_eq "tier_runner_units_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 2: Run each of the six tier targets (G0..G5) with a small filter
# ---------------------------------------------------------------------------
e2e_step "run_six_tiers" "Run G0..G5 tier targets and verify hash-named receipts"

_run_one_tier() {
  local target="$1" filter="$2"
  e2e_tier "${target}" "${filter}"
  e2e_assert_eq "${target}_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

  local rpath=""
  if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    rpath="$(_extract_receipt_path "${E2E_LAST_STDOUT}" || true)"
  fi
  local has_file="0"
  if [ -n "${rpath}" ] && [ -f "${rpath}" ]; then
    has_file="1"
  fi
  e2e_assert_eq "${target}_receipt_exists" "${has_file}" "1" --gate "GATE-GAUNT-01"

  if [ "${has_file}" = "1" ]; then
    if [ -n "${E2E_LAST_RECEIPT:-}" ]; then
      e2e_assert_eq "${target}_receipt_matches_last_line" \
        "$(basename "${rpath}")" "$(basename "${E2E_LAST_RECEIPT}")" --gate "GATE-GAUNT-01"
    fi
    local info
    info="$(_inspect_receipt "${rpath}")"
    local digest schema stack_receipt stack_dev build_identity c_pass c_fail c_skip gate_line
    IFS=$'\t' read -r digest schema stack_receipt stack_dev build_identity c_pass c_fail c_skip gate_line <<< "${info}"
    e2e_assert_eq "${target}_schema" "${schema}" "frob.gauntlet.receipt/1" --gate "GATE-GAUNT-01"
    local sr_ok="0"
    case "${stack_receipt}" in
      sr:*) sr_ok="1" ;;
    esac
    e2e_assert_eq "${target}_stack_receipt_prefix" "${sr_ok}" "1" --gate "GATE-GAUNT-01"
    local bi_ok="0"
    if [ "${build_identity}" = "dev-salt" ] || [ "${build_identity}" = "depgraph-receipt" ]; then
      bi_ok="1"
    fi
    e2e_assert_eq "${target}_build_identity_valid" "${bi_ok}" "1" --gate "GATE-GAUNT-01"
    e2e_metric "${target}_pass" "${c_pass}" "entries"
    e2e_metric "${target}_fail" "${c_fail}" "entries"
    e2e_metric "${target}_skipped" "${c_skip}" "entries"
    e2e_metric "${target}_receipt_digest" "${digest}" "blake3"
    e2e_metric "${target}_stack_receipt" "${stack_receipt}" "stack-receipt"
    e2e_metric "${target}_stack_dev" "${stack_dev}" "bool"
    e2e_metric "${target}_build_identity" "${build_identity}" "mode"
    e2e_metric "${target}_gate_line" "${gate_line}" "status"
  fi
}

if e2e_plant "drop-control"; then
  # Under E2E_PLANT_FAILURE=drop-control, run G0 against an emptied-controls scratch registry
  # via e2e_tier and assert that GATE-GAUNT-01 is NOT eligible to pass (triggering failure if pass expected).
  WS_ENV_DROP="$(_make_scratch_registry "env_drop_control")"
  _empty_gate_controls "${WS_ENV_DROP}/registries/gates.toml" "GATE-GAUNT-01"
  FROB_REGISTRY_ROOT="${WS_ENV_DROP}" \
    FROB_CONFORMANCE_ROOT="${WS_ENV_DROP}" \
    FROB_GAUNTLET_RECEIPTS_DIR="${E2E_REPO_ROOT}/target/gauntlet/receipts" \
    e2e_tier "g0_laws" "GATE-GAUNT-01"
  _env_rpath="$(_extract_receipt_path "${E2E_LAST_STDOUT}" || true)"
  _env_line=""
  if [ -n "${_env_rpath}" ] && [ -f "${_env_rpath}" ]; then
    _env_line="$(_inspect_receipt "${_env_rpath}" | cut -f9)"
  fi
  e2e_assert_eq "env_drop_control_gate_line" "${_env_line}" "pass" --gate "GATE-GAUNT-01"
else
  _run_one_tier "g0_laws" "law:level-series@host"
  _run_one_tier "g1_known_answers" "KA-501"
  _run_one_tier "g2_parity" "GATE-GAUNT-05"
  _run_one_tier "g3_metamorphic" "GATE-GAUNT-01"
  _run_one_tier "g4_chaos" "receipt-tamper@host"
  _run_one_tier "g5_determinism" "GATE-GAUNT-01"
fi

# ---------------------------------------------------------------------------
# Step 3: Plant `drop-control` in a scratch registry -> gate line becomes `not-eligible`
# ---------------------------------------------------------------------------
e2e_step "plant_drop_control" "Empty controls on GATE-GAUNT-01 in scratch registry -> not-eligible"
WS_DROP="$(_make_scratch_registry "drop_control")"
_empty_gate_controls "${WS_DROP}/registries/gates.toml" "GATE-GAUNT-01"

FROB_REGISTRY_ROOT="${WS_DROP}" \
  FROB_CONFORMANCE_ROOT="${WS_DROP}" \
  FROB_GAUNTLET_RECEIPTS_DIR="${E2E_REPO_ROOT}/target/gauntlet/receipts" \
  e2e_tier "g0_laws" "GATE-GAUNT-01"
e2e_assert_eq "drop_control_g0_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

_drop_rpath=""
if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _drop_rpath="$(_extract_receipt_path "${E2E_LAST_STDOUT}" || true)"
fi
_drop_line=""
if [ -n "${_drop_rpath}" ] && [ -f "${_drop_rpath}" ]; then
  _drop_line="$(_inspect_receipt "${_drop_rpath}" | cut -f9)"
fi
e2e_assert_eq "drop_control_gate_not_eligible" "${_drop_line}" "not-eligible" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 4: Negative control — empty selection (`KA-999999`) must exit non-zero
# ---------------------------------------------------------------------------
e2e_step "empty_selection_control" "Empty selection KA-999999 exits non-zero"
# shellcheck disable=SC2016
e2e_expect_caught "empty-selection-ka-999999" --gate "GATE-GAUNT-01" -- \
  bash -c 'FROB_TEST_SEED="$1" FROB_GAUNTLET_RECEIPTS_DIR="$2" cargo test --manifest-path "$3" -p frob-conformance --test g1_known_answers -- KA-999999' \
  _ "${E2E_SEED}" "${E2E_WORK_DIR}/empty_sel_receipts" "${MANIFEST}"
