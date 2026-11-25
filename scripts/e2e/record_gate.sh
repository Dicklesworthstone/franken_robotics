#!/usr/bin/env bash
# e2e: bead=N38b-19 milestone=M0 labels=gauntlet,gate gates=GATE-GAUNT-10 plants=self-certify,missed-control,tampered-receipt seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of the gate recorder (bead N38b-19, GATE-GAUNT-10).
# Operates strictly inside an isolated scratch repository under ${E2E_WORK_DIR}
# and never writes the checkout's registries/gates.toml (whose digest is asserted
# unchanged before and after).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/record_gate/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "record_gate" "N38b-19" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-record-gate-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

CHECKOUT_GATES="${E2E_REPO_ROOT}/registries/gates.toml"
hash_file() {
  if command -v b3sum >/dev/null 2>&1; then
    b3sum --no-names "$1"
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

e2e_step "checkout_digest_before" "Record digest of checkout registries/gates.toml before running recorder"
BEFORE_CHECKOUT_DIGEST="$(hash_file "${CHECKOUT_GATES}")"
e2e_assert_file "checkout_gates_exists" "${CHECKOUT_GATES}"

e2e_step "recorder_unit_suite" "Run the full N38b-19 recorder test suite in isolated scratch Git repos"
e2e_cmd --name "cargo_test_recorder" -- \
  env "CARGO_TARGET_DIR=${TARGET_BASE}/conformance" \
  cargo test --offline -j 1 -p frob-conformance --test recorder
e2e_assert_eq "cargo_test_recorder_exit" "${E2E_LAST_EXIT}" "0"

e2e_step "scratch_cli_recording_and_plants" "Exercise record_gate example CLI on scratch Git repo with happy path and three plants"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  SCRATCH="${E2E_WORK_DIR}/scratch-repo"
  mkdir -p "${SCRATCH}/registries" \
           "${SCRATCH}/crates/frob-conformance/registry" \
           "${SCRATCH}/target/gauntlet/receipts" \
           "${SCRATCH}/receipts/gates" \
           "${SCRATCH}/docs/delivery/logs"

  git -C "${SCRATCH}" init -q
  git -C "${SCRATCH}" config user.email "e2e@franken.invalid"
  git -C "${SCRATCH}" config user.name "Franken E2E"
  git -C "${SCRATCH}" config commit.gpgsign false

  printf 'impl\n' > "${SCRATCH}/capability.txt"
  git -C "${SCRATCH}" add capability.txt
  git -C "${SCRATCH}" commit -q -m "implement capability" -m "Frob-Session: session:impl-e2e"
  SCRATCH_HEAD="$(git -C "${SCRATCH}" rev-parse HEAD)"

  cat > "${SCRATCH}/registries/gates.toml" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-10"
kind = "convergence"
claim = "known-answer"
claim_id = "clm:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
section = "38.15"
threshold = "spectral radius <= 0.95"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
EOF

  cat > "${SCRATCH}/crates/frob-conformance/registry/controls.toml" <<'EOF'
schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-10"
gate = ["GATE-GAUNT-10"]
kind = "planted-input"
target = "frob-conformance::test::e2e"
defect = "planted self-certification defect"
expect = "fails with self-certified"
fixture = "fixtures/e2e.toml"
due = "M0"
state = "runnable"
EOF

  STACK_HEAD="$(tr -d '[:space:]' < "${E2E_REPO_ROOT}/receipts/HEAD")"
  GOOD_BODY="{\"build_identity\":\"depgraph-receipt\",\"commit\":\"${SCRATCH_HEAD}\",\"controls\":[{\"gate\":\"GATE-GAUNT-10\",\"id\":\"NC-0001\",\"outcome\":\"caught\"}],\"counts\":{\"fail\":0,\"obligations\":0,\"pass\":1,\"refused\":0,\"skipped\":0},\"dirty\":false,\"entries\":[{\"determinism\":\"D0\",\"id\":\"ka:e2e\",\"measured\":{\"name\":\"rho\",\"threshold\":\"<= 0.95\",\"unit\":\"1\",\"value\":\"0.9207\"},\"outcome\":\"pass\",\"site\":\"host\",\"synthetic\":false}],\"filter\":\"gate:GATE-GAUNT-10\",\"fingerprint\":\"host-e2e\",\"gates\":[{\"id\":\"GATE-GAUNT-10\",\"line\":\"pass\"}],\"profile\":\"dev\",\"schema\":\"frob.gauntlet.receipt/1\",\"seed\":42,\"session\":\"session:runner\",\"stack_dev\":false,\"stack_receipt\":\"sr:${STACK_HEAD}\",\"started\":\"2026-11-25T12:00:00Z\",\"tier\":\"G1\",\"toolchain\":\"rustc-nightly-2026-07-06\",\"wall_ms\":5}"
  GOOD_TMP="${E2E_WORK_DIR}/good_receipt.json"
  printf '%s' "${GOOD_BODY}" > "${GOOD_TMP}"
  GOOD_HASH="$(b3sum --no-names "${GOOD_TMP}")"
  cp "${GOOD_TMP}" "${SCRATCH}/target/gauntlet/receipts/${GOOD_HASH}.json"

  MISSED_BODY="{\"build_identity\":\"depgraph-receipt\",\"commit\":\"${SCRATCH_HEAD}\",\"controls\":[{\"gate\":\"GATE-GAUNT-10\",\"id\":\"NC-0001\",\"outcome\":\"missed\"}],\"counts\":{\"fail\":0,\"obligations\":0,\"pass\":1,\"refused\":0,\"skipped\":0},\"dirty\":false,\"entries\":[{\"determinism\":\"D0\",\"id\":\"ka:e2e\",\"measured\":{\"name\":\"rho\",\"threshold\":\"<= 0.95\",\"unit\":\"1\",\"value\":\"0.9207\"},\"outcome\":\"pass\",\"site\":\"host\",\"synthetic\":false}],\"filter\":\"gate:GATE-GAUNT-10\",\"fingerprint\":\"host-e2e\",\"gates\":[{\"id\":\"GATE-GAUNT-10\",\"line\":\"pass\"}],\"profile\":\"dev\",\"schema\":\"frob.gauntlet.receipt/1\",\"seed\":42,\"session\":\"session:runner\",\"stack_dev\":false,\"stack_receipt\":\"sr:${STACK_HEAD}\",\"started\":\"2026-11-25T12:00:00Z\",\"tier\":\"G1\",\"toolchain\":\"rustc-nightly-2026-07-06\",\"wall_ms\":5}"
  MISSED_TMP="${E2E_WORK_DIR}/missed_receipt.json"
  printf '%s' "${MISSED_BODY}" > "${MISSED_TMP}"
  MISSED_HASH="$(b3sum --no-names "${MISSED_TMP}")"
  cp "${MISSED_TMP}" "${SCRATCH}/target/gauntlet/receipts/${MISSED_HASH}.json"

  TAMPER_HASH="1111111111111111111111111111111111111111111111111111111111111111"
  cp "${GOOD_TMP}" "${SCRATCH}/target/gauntlet/receipts/${TAMPER_HASH}.json"

  # Plant 1: self-certify (recorder session matches implementer session:impl-e2e)
  e2e_cmd --name "plant_self_certify" --allow-fail -- \
    env "CARGO_TARGET_DIR=${TARGET_BASE}/conformance" "FROB_SESSION=session:impl-e2e" \
    cargo run --offline -j 1 -p frob-conformance --example record_gate -- \
      --repo-root "${SCRATCH}" --gate GATE-GAUNT-10 --receipt "${GOOD_HASH}" --bead N38b-19 --date 2026-11-25
  e2e_assert_eq "plant_self_certify_exit" "${E2E_LAST_EXIT}" "3"
  if grep -q "reason=self-certified" "${E2E_LAST_STDERR}"; then got="ok"; else got="missing"; fi
  e2e_assert_eq "plant_self_certify_reason" "${got}" "ok"

  # Plant 2: missed-control
  e2e_cmd --name "plant_missed_control" --allow-fail -- \
    env "CARGO_TARGET_DIR=${TARGET_BASE}/conformance" "FROB_SESSION=session:reviewer-e2e" \
    cargo run --offline -j 1 -p frob-conformance --example record_gate -- \
      --repo-root "${SCRATCH}" --gate GATE-GAUNT-10 --receipt "${MISSED_HASH}" --bead N38b-19 --date 2026-11-25
  e2e_assert_eq "plant_missed_control_exit" "${E2E_LAST_EXIT}" "3"
  if grep -q "reason=control-not-caught" "${E2E_LAST_STDERR}"; then got="ok"; else got="missing"; fi
  e2e_assert_eq "plant_missed_control_reason" "${got}" "ok"

  # Plant 3: tampered-receipt
  e2e_cmd --name "plant_tampered_receipt" --allow-fail -- \
    env "CARGO_TARGET_DIR=${TARGET_BASE}/conformance" "FROB_SESSION=session:reviewer-e2e" \
    cargo run --offline -j 1 -p frob-conformance --example record_gate -- \
      --repo-root "${SCRATCH}" --gate GATE-GAUNT-10 --receipt "${TAMPER_HASH}" --bead N38b-19 --date 2026-11-25
  e2e_assert_eq "plant_tampered_receipt_exit" "${E2E_LAST_EXIT}" "3"
  if grep -q "reason=receipt-hash-mismatch" "${E2E_LAST_STDERR}"; then got="ok"; else got="missing"; fi
  e2e_assert_eq "plant_tampered_receipt_reason" "${got}" "ok"

  # Happy path: distinct reviewer session records pass and commits byte-identical receipt.
  e2e_cmd --name "record_happy_path" -- \
    env "CARGO_TARGET_DIR=${TARGET_BASE}/conformance" "FROB_SESSION=session:reviewer-e2e" \
    cargo run --offline -j 1 -p frob-conformance --example record_gate -- \
      --repo-root "${SCRATCH}" --gate GATE-GAUNT-10 --receipt "${GOOD_HASH}" --bead N38b-19 --date 2026-11-25
  e2e_assert_eq "record_happy_path_exit" "${E2E_LAST_EXIT}" "0"
  e2e_assert_file "committed_receipt_exists" "${SCRATCH}/receipts/gates/${GOOD_HASH}.json"
  e2e_assert_digest_eq "committed_receipt_matches" \
    "$(b3sum --no-names "${SCRATCH}/receipts/gates/${GOOD_HASH}.json")" "${GOOD_HASH}"

  # Unknown mode word refuses without altering scratch gates.toml.
  SCRATCH_GATES_BEFORE_BADMODE="$(hash_file "${SCRATCH}/registries/gates.toml")"
  e2e_cmd --name "unknown_mode_word" --allow-fail -- \
    env "CARGO_TARGET_DIR=${TARGET_BASE}/conformance" \
    cargo run --offline -j 1 -p frob-conformance --example record_gate -- \
      frobnicate --gate GATE-GAUNT-10 --registry "${SCRATCH}/registries/gates.toml"
  e2e_assert_eq "unknown_mode_word_exit" "${E2E_LAST_EXIT}" "3"
  SCRATCH_GATES_AFTER_BADMODE="$(hash_file "${SCRATCH}/registries/gates.toml")"
  e2e_assert_digest_eq "unknown_mode_word_unchanged" "${SCRATCH_GATES_BEFORE_BADMODE}" "${SCRATCH_GATES_AFTER_BADMODE}"
fi

e2e_step "checkout_digest_after" "Verify checkout registries/gates.toml is byte-identical"
AFTER_CHECKOUT_DIGEST="$(hash_file "${CHECKOUT_GATES}")"
e2e_assert_digest_eq "checkout_gates_unchanged" "${BEFORE_CHECKOUT_DIGEST}" "${AFTER_CHECKOUT_DIGEST}"
