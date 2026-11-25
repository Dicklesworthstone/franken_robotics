#!/usr/bin/env bash
# e2e: bead=N38a-04 milestone=M0 labels=gauntlet,oracle gates=GATE-GAUNT-05 plants=regen-from-candidate
#
# End-to-end verification of the oracle register, pinned fixtures, and capture
# scripts (`N38a-04`, `GATE-GAUNT-05`):
#   1. Validates `crates/frob-conformance/registry/oracles.toml`,
#      `oracle_findings.toml`, and every pinned fixture on the clean tree
#      (or on a planted candidate-regenerated scratch copy when
#      `E2E_PLANT_FAILURE=regen-from-candidate`).
#   2. Logs counts of fixtures per oracle and per status (`active`, `disputed`)
#      as `metric` lines and independently verifies each fixture's `bytes_sha256`
#      as a digest assertion.
#   3. Emits one `assert` per implemented rule (Rules 1, 2, 5, 6).
#   4. Plants `regen-from-candidate` in scratch copies (both raw byte corruption
#      and re-hashed candidate regeneration without a `[[pin_move]]` record) and
#      requires `oracle_register` to catch both via `e2e_expect_caught`.
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "oracle_register" "N38a-04"
e2e_gate "GATE-GAUNT-05"

_make_scratch_ws() {
  local label="$1"
  local dst="${E2E_WORK_DIR}/ws_${label}"
  mkdir -p "${dst}/crates/frob-conformance"
  cp "${E2E_REPO_ROOT}/Cargo.toml" "${dst}/Cargo.toml"
  if [ -f "${E2E_REPO_ROOT}/franken-stack.lock" ]; then
    cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  fi
  if [ -d "${E2E_REPO_ROOT}/crates/frob-conformance/registry" ]; then
    cp -R "${E2E_REPO_ROOT}/crates/frob-conformance/registry" "${dst}/crates/frob-conformance/registry"
  fi
  if [ -d "${E2E_REPO_ROOT}/crates/frob-conformance/fixtures" ]; then
    cp -R "${E2E_REPO_ROOT}/crates/frob-conformance/fixtures" "${dst}/crates/frob-conformance/fixtures"
  fi
  if [ -d "${E2E_REPO_ROOT}/tools/oracle" ]; then
    mkdir -p "${dst}/tools"
    cp -R "${E2E_REPO_ROOT}/tools/oracle" "${dst}/tools/oracle"
  fi
  if [ -d "${E2E_REPO_ROOT}/tests/fixtures/oracle" ]; then
    mkdir -p "${dst}/tests/fixtures"
    cp -R "${E2E_REPO_ROOT}/tests/fixtures/oracle" "${dst}/tests/fixtures/oracle"
  fi
  printf '%s' "${dst}"
}

_independent_fixture_sha256() {
  local fpath="$1"
  python3 - "${fpath}" <<'PY'
import hashlib, re, sys
txt = open(sys.argv[1], "r", encoding="utf-8").read()
zero = "0" * 64
masked, n = re.subn(r'bytes_sha256\s*=\s*"[0-9a-f]{64}"', f'bytes_sha256 = "{zero}"', txt, count=1)
if n != 1:
    sys.exit(1)
print(hashlib.sha256(masked.encode("utf-8")).hexdigest())
PY
}

_plant_rehashed_fixture() {
  local dst_fixture="$1"
  python3 - "${dst_fixture}" <<'PY'
import hashlib, re, sys
p = sys.argv[1]
txt = open(p, "r", encoding="utf-8").read()
txt = txt.replace('period_elliptic_s = "', 'period_elliptic_s = "9', 1)
zero = "0" * 64
masked = re.sub(r'bytes_sha256\s*=\s*"[0-9a-f]{64}"', f'bytes_sha256 = "{zero}"', txt, count=1)
digest = hashlib.sha256(masked.encode("utf-8")).hexdigest()
signed = re.sub(r'bytes_sha256\s*=\s*"[0-9a-f]{64}"', f'bytes_sha256 = "{digest}"', masked, count=1)
open(p, "w", encoding="utf-8").write(signed)
PY
}

_baseline_path="crates/frob-conformance/fixtures/pendulum_period.toml"
_baseline_hash="$(_independent_fixture_sha256 "${E2E_REPO_ROOT}/${_baseline_path}")"

# ---------------------------------------------------------------------------
# Step 1: Validate register and every fixture on the clean tree
#         (or against planted candidate-regenerated copy under E2E_PLANT_FAILURE)
# ---------------------------------------------------------------------------
e2e_step "validate_clean_tree" "Validate oracles.toml and every pinned fixture on the clean tree"

if e2e_plant "regen-from-candidate"; then
  WS_PLANTED="$(_make_scratch_ws "env_plant_regen")"
  _plant_rehashed_fixture "${WS_PLANTED}/${_baseline_path}"
  e2e_cmd --name oracle_register_all_tests -- \
    env "FROB_ORACLE_ROOT=${WS_PLANTED}" \
    "FROB_ORACLE_BASELINE_HASH=${_baseline_path}=${_baseline_hash}" \
    cargo test -p frob-conformance --test oracle_register -- oracle_register_clean_tree --nocapture
else
  e2e_cmd --name oracle_register_all_tests -- \
    cargo test -p frob-conformance --test oracle_register -- --nocapture
fi
e2e_assert_eq "oracle_register_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-05"

if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _parsed="$(python3 - "${E2E_LAST_STDOUT}" <<'PY'
import re, sys

txt = open(sys.argv[1], "r", encoding="utf-8", errors="replace").read()
m = re.search(r"oracle_register:\s+oracles=(\d+)\s+fixtures=(\d+)\s+active=(\d+)\s+disputed=(\d+)\s+findings=(\d+)", txt)
if m:
    print(f"SUMMARY {m.group(1)} {m.group(2)} {m.group(3)} {m.group(4)} {m.group(5)}")
for mm in re.finditer(r"oracle_fixture_count:\s+oracle=(\S+)\s+count=(\d+)", txt):
    print(f"ORACLE {mm.group(1)} {mm.group(2)}")
for mm in re.finditer(r"oracle_fixture:\s+path=(\S+)\s+oracle=(\S+)\s+status=(\S+)\s+bytes_sha256=([0-9a-f]{64})", txt):
    print(f"FIXTURE {mm.group(1)} {mm.group(2)} {mm.group(3)} {mm.group(4)}")
PY
  )"
  while IFS=' ' read -r _kind _a _b _c _d _e; do
    case "${_kind}" in
      SUMMARY)
        e2e_metric "oracles_registered" "${_a}" "oracles"
        e2e_metric "fixtures_total" "${_b}" "fixtures"
        e2e_metric "fixtures_status_active" "${_c}" "fixtures"
        e2e_metric "fixtures_status_disputed" "${_d}" "fixtures"
        e2e_metric "oracle_findings_total" "${_e}" "findings"
        ;;
      ORACLE)
        e2e_metric "fixtures_oracle_${_a}" "${_b}" "fixtures"
        ;;
      FIXTURE)
        _disk_hash="$(_independent_fixture_sha256 "${E2E_REPO_ROOT}/${_a}")"
        e2e_assert_digest_eq "fixture_digest_${_b}" "${_d}" "${_disk_hash}" --gate "GATE-GAUNT-05"
        ;;
    esac
  done <<< "${_parsed}"
fi

# ---------------------------------------------------------------------------
# Step 2: Per-rule assertions (Rules 1, 2, 5, 6)
# ---------------------------------------------------------------------------
e2e_step "per_rule_assertions" "Verify Rules 1, 2, 5, and 6 unit assertions individually"

e2e_cmd --name rule1_capture_and_hash -- \
  cargo test -p frob-conformance --test oracle_register -- fixture_bytes_match_hash
e2e_assert_eq "rule1_capture_never_regenerate" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-05"

e2e_cmd --name rule1_workspace_inputs -- \
  cargo test -p frob-conformance --test oracle_register -- capture_inputs_exclude_workspace
e2e_assert_eq "rule1_exclude_workspace_inputs" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-05"

e2e_cmd --name rule2_pin_move -- \
  cargo test -p frob-conformance --test oracle_register -- oracle_pin_move_needs_record
e2e_assert_eq "rule2_pin_move_requires_record" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-05"

e2e_cmd --name rule5_admission_held_out -- \
  cargo test -p frob-conformance --test oracle_register -- held_out_refused
e2e_assert_eq "rule5_held_out_and_admission" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-05"

e2e_cmd --name rule6_disagreement -- \
  cargo test -p frob-conformance --test oracle_register -- disagreement_is_finding
e2e_assert_eq "rule6_disagreement_is_finding" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-05"

# ---------------------------------------------------------------------------
# Step 3: Negative control plant `regen-from-candidate` (bytes replaced, no pin move)
# ---------------------------------------------------------------------------
e2e_step "plant_regen_from_candidate" "Plant a candidate-regenerated fixture with no [[pin_move]] and require failure"

# Plant 3a: Raw byte replacement without updating hash (catches direct byte tamper).
WS_RAW="$(_make_scratch_ws "regen_raw")"
python3 - "${WS_RAW}/${_baseline_path}" <<'PY'
import sys
p = sys.argv[1]
txt = open(p, "r", encoding="utf-8").read()
txt = txt.replace('period_elliptic_s = "', 'period_elliptic_s = "9', 1)
open(p, "w", encoding="utf-8").write(txt)
PY

e2e_expect_caught "regen-from-candidate:byte-tamper" --gate "GATE-GAUNT-05" -- \
  env "FROB_ORACLE_ROOT=${WS_RAW}" \
  cargo test -p frob-conformance --test oracle_register -- oracle_register_clean_tree --nocapture

# Plant 3b: Candidate regenerates fixture AND recomputes bytes_sha256, with NO [[pin_move]] record.
WS_REHASH="$(_make_scratch_ws "regen_rehash")"
_plant_rehashed_fixture "${WS_REHASH}/${_baseline_path}"

e2e_expect_caught "regen-from-candidate" --gate "GATE-GAUNT-05" -- \
  env "FROB_ORACLE_ROOT=${WS_REHASH}" \
  "FROB_ORACLE_BASELINE_HASH=${_baseline_path}=${_baseline_hash}" \
  cargo test -p frob-conformance --test oracle_register -- oracle_register_clean_tree --nocapture
