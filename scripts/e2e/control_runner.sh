#!/usr/bin/env bash
# e2e: bead=N38b-04 milestone=M0 labels=gauntlet,registry gates=GATE-GAUNT-01 plants=noop-mutation,stale-feature,never-caught
#
# End-to-end verification of the negative-control runner and the five rules of
# the negative-control registry (`N38b-04`, `GATE-GAUNT-01`):
#   1. Run `control_runner`, `mutation_guard`, and `control_review` unit suites.
#   2. Run `GATE-GAUNT-01` via `e2e_tier g0_laws GATE-GAUNT-01` on the clean tree,
#      expecting `pass` and emitting per-control caught/missed/stale/not-yet-due metrics.
#   3. Plant three negative controls in scratch copies:
#      - `noop-mutation`: no-op mutation -> `missed` -> gate line `fail`.
#      - `stale-feature`: removed feature -> `stale` -> gate line `not-eligible`.
#      - `never-caught`: never-caught control at a simulated milestone exit
#        (`FROB_MILESTONE_EXIT=1`) -> `control_review` blocks with `suspect-undisposed`.
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "control_runner" "N38b-04"
e2e_gate "GATE-GAUNT-01"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"

_make_scratch_exit_ws() {
  local label="$1"
  local dst="${E2E_WORK_DIR}/ws_${label}"
  mkdir -p "${dst}/registries" "${dst}/crates/frob-conformance/registry"
  cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  cp "${E2E_REPO_ROOT}/rust-toolchain.toml" "${dst}/rust-toolchain.toml"
  cp "${E2E_REPO_ROOT}/registries/gates.toml" "${dst}/registries/gates.toml"
  cp "${E2E_REPO_ROOT}/crates/frob-conformance/registry/controls.toml" \
    "${dst}/crates/frob-conformance/registry/controls.toml"
  cp "${E2E_REPO_ROOT}/crates/frob-conformance/registry/control_reviews.toml" \
    "${dst}/crates/frob-conformance/registry/control_reviews.toml"
  : > "${dst}/crates/frob-conformance/registry/control_history.ndjson"
  printf '%s' "${dst}"
}

_inspect_tier_receipt() {
  local stdout_file="$1"
  python3 - "${stdout_file}" "${E2E_REPO_ROOT}" <<'PY'
import json, os, sys
stdout_path, repo_root = sys.argv[1], sys.argv[2]
lines = [ln.strip() for ln in open(stdout_path, "r", encoding="utf-8", errors="replace").read().splitlines() if ln.strip()]
if not lines or not lines[-1].startswith("receipt: "):
    sys.exit(2)
rel = lines[-1][len("receipt: "):].strip()
full = rel if os.path.isabs(rel) else os.path.join(repo_root, rel)
data = json.load(open(full, "r", encoding="utf-8"))
controls = data.get("controls", [])
caught = sum(1 for c in controls if c.get("outcome") == "caught")
missed = sum(1 for c in controls if c.get("outcome") == "missed")
stale = sum(1 for c in controls if c.get("outcome") == "stale")
not_yet_due = sum(1 for c in controls if c.get("outcome") == "not-yet-due")
gates = {g["id"]: g["line"] for g in data.get("gates", [])}
gline = gates.get("GATE-GAUNT-01", "")
print(f"{os.path.basename(full)[:-5]}\t{gline}\t{caught}\t{missed}\t{stale}\t{not_yet_due}")
PY
}

# ---------------------------------------------------------------------------
# Step 1: Run control_runner, mutation_guard, and control_review suites
# ---------------------------------------------------------------------------
e2e_step "unit_suites" "Run control_runner, mutation_guard, and control_review tests"
e2e_cmd --name control_runner_units -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
    --test control_runner --test mutation_guard --test control_review
e2e_assert_eq "unit_suites_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 2: Clean-tree gate run (`GATE-GAUNT-01` via `e2e_tier`)
# ---------------------------------------------------------------------------
e2e_step "clean_gate_run" "Run GATE-GAUNT-01 with its controls on the clean tree"

if e2e_plant "noop-mutation"; then
  # Under E2E_PLANT_FAILURE=noop-mutation, invert the missed-control check so the
  # script fails when a no-op mutation is planted.
  e2e_cmd --name plant_noop_mutation --expect-exit 0 -- \
    cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
      --test control_runner -- missed_makes_gate_fail --exact
  e2e_assert_eq "noop_mutation_must_fail_gate" "missed" "caught" --gate "GATE-GAUNT-01"
elif e2e_plant "stale-feature"; then
  # Under E2E_PLANT_FAILURE=stale-feature, require `pass` where stale yields `not-eligible`.
  e2e_cmd --name plant_stale_feature --expect-exit 0 -- \
    cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
      --test control_runner -- stale_control --exact
  e2e_assert_eq "stale_feature_must_not_pass" "not-eligible" "pass" --gate "GATE-GAUNT-01"
elif e2e_plant "never-caught"; then
  # Under E2E_PLANT_FAILURE=never-caught, run control_review at a simulated M0 exit
  # with empty history and require exit 0 (which fails because review blocks the exit).
  WS_ENV_NC="$(_make_scratch_exit_ws "env_never_caught")"
  e2e_cmd --name plant_never_caught --expect-exit 0 -- \
    env "FROB_MILESTONE=M0" "FROB_MILESTONE_EXIT=1" \
    "FROB_CONFORMANCE_ROOT=${WS_ENV_NC}" "FROB_REGISTRY_ROOT=${WS_ENV_NC}" \
    cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
      --test control_review -- control_review_workspace --exact
  e2e_assert_eq "never_caught_env_plant" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"
else
  e2e_tier "g0_laws" "GATE-GAUNT-01"
  e2e_assert_eq "clean_g0_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"
  INFO="$(_inspect_tier_receipt "${E2E_LAST_STDOUT}")"
  IFS=$'\t' read -r DIGEST GATE_LINE C_CAUGHT C_MISSED C_STALE C_NYD <<< "${INFO}"
  e2e_assert_eq "clean_gate_line" "${GATE_LINE}" "pass" --gate "GATE-GAUNT-01"
  e2e_metric "controls_caught" "${C_CAUGHT}" "controls" ">=" "1" --gate "GATE-GAUNT-01"
  e2e_metric "controls_missed" "${C_MISSED}" "controls" "==" "0" --gate "GATE-GAUNT-01"
  e2e_metric "controls_stale" "${C_STALE}" "controls" "==" "0" --gate "GATE-GAUNT-01"
  e2e_metric "controls_not_yet_due" "${C_NYD}" "controls"
  e2e_metric "clean_receipt_digest" "${DIGEST}" "blake3"
fi

# ---------------------------------------------------------------------------
# Step 3: Three planted negative controls in scratch copies
# ---------------------------------------------------------------------------
e2e_step "planted_controls" "Verify noop-mutation, stale-feature, and never-caught review block"

# 3a. No-op mutation (expect missed -> gate line fail; verified by real scratch build + caught control).
e2e_cmd --name verify_noop_mutation -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
    --test control_runner -- missed_makes_gate_fail --exact
e2e_assert_eq "noop_mutation_verified" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# 3b. Stale feature (expect stale -> gate line not-eligible; verified by real scratch build).
e2e_cmd --name verify_stale_feature -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
    --test control_runner -- stale_control --exact
e2e_assert_eq "stale_feature_verified" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# 3c. Real cx-262 mutation build on the host stand-in wheel-loop row (expect failure = caught!).
e2e_expect_caught "nc-0412-wheel-loop-cx-262" --gate "GATE-GAUNT-01" --nc "NC-0412" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
    --features gauntlet-mutations,cx-262 \
    --test control_runner -- wheel_loop_host_standin_row --exact

# 3d. Simulated M0 milestone exit with a never-caught control in a scratch copy (must block!).
WS_EXIT="$(_make_scratch_exit_ws "never_caught_exit")"
e2e_expect_caught "never-caught-blocks-exit" --gate "GATE-GAUNT-01" -- \
  env "FROB_MILESTONE=M0" "FROB_MILESTONE_EXIT=1" \
  "FROB_CONFORMANCE_ROOT=${WS_EXIT}" "FROB_REGISTRY_ROOT=${WS_EXIT}" \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance \
    --test control_review -- control_review_workspace --exact

# 3e. Negative control: release-profile build with mutation feature cx-262 must not compile!
e2e_expect_caught "release-mutation-compile-fail" --gate "GATE-GAUNT-01" -- \
  env "CARGO_TARGET_DIR=${E2E_WORK_DIR}/release_guard_target" \
  cargo check --manifest-path "${MANIFEST}" -p frob-conformance --release --features cx-262 --offline
