#!/usr/bin/env bash
# e2e: bead=N40-03 milestone=M0 labels=gauntlet,registry gates=GATE-GAUNT-10 seed=0 needs=cpu plants=amended-after-measurement,changed-without-amendment,fired-renamed,self-certified,decision-scope-invalid,decision-entry-duplicate
#
# End-to-end verification of kill-gate records, dated amendments, decision ADRs,
# scope kinds, renaming (`apply_decisions`), and `--live --ids <file>` close-time checks
# (bead N40-03, gate GATE-GAUNT-10, N40-KA R-10/R-32, N40-CX RC-10..RC-13/RC-57/RC-68..RC-70).
#
# Modes:
#   Default (synthetic scratch register of three kill gates):
#     (0) Run `cargo test -p frob-conformance --lib register::kill::` (26 unit tests).
#     (1) Record a `not-fired` decision with receipts and assert the exit query lists zero
#         missing kill-gate decisions (recording register digests before and after).
#     (2) Lower a trigger via an amendment dated AFTER the governing receipt and assert
#         `amended-after-measurement` (logging both timestamps compared).
#     (3) Edit a fallback with no amendment against seeded rows and assert
#         `changed-without-amendment`.
#     (4) Fire a gate and assert `apply_decisions` README table and register printout print
#         the fallback's name and never print the killed capability as passed (per Round-4
#         Review Notes #1 and #7, asserted directly on `apply_decisions` outputs without
#         invoking `cargo xtask check-generated`).
#     (5) Record an ADR from a session in the implementer set and assert `self-certified`.
#     (6) Record per-scope decisions (`backend:gpu-cuda` fired, `backend:gpu-metal` not-fired)
#         and a `reading = "final"` second entry, asserting the scope in force and both ADR
#         paths under `docs/adr/`, then plant `family:cuda` and a duplicate entry and assert
#         `decision-scope-invalid` and `decision-entry-duplicate`.
#     (7) Exercise `--live-ids` on the synthetic register (valid pass, one row missing its
#         fallback, and one row with a changed trigger), asserting each offending ID is named.
#
#   Live read-only close-time mode:
#     `scripts/e2e/kill_gate_records.sh --live --ids <file>`
#     Validates every listed `<KILL-ID> <seed_hash>` against the committed
#     `registries/gates.toml` without modifying any file; exits 0 when every listed ID
#     is one complete `kind = "kill"` row matching its seeded hash, or non-zero naming the
#     first absent, duplicated, incomplete, or changed ID.
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

LIVE_MODE=0
LIVE_IDS_FILE=""
ORIG_ARGS=("$@")

while [ "$#" -gt 0 ]; do
  case "$1" in
    --live)
      LIVE_MODE=1
      shift
      ;;
    --ids)
      if [ "$#" -lt 2 ]; then
        printf 'kill_gate_records.sh: --ids requires a file path\n' >&2
        exit 2
      fi
      LIVE_IDS_FILE="$2"
      shift 2
      ;;
    *)
      printf 'kill_gate_records.sh: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

if [ "${LIVE_MODE}" -eq 1 ] && [ -z "${LIVE_IDS_FILE}" ]; then
  printf 'kill_gate_records.sh: --live requires --ids <file>\n' >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "kill_gate_records" "N40-03" ${ORIG_ARGS[@]+"${ORIG_ARGS[@]}"}
e2e_gate "GATE-GAUNT-10"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
COMMITTED_GATES="${E2E_REPO_ROOT}/registries/gates.toml"

# ---------------------------------------------------------------------------
# Live read-only mode (`--live --ids <file>`): close-time check for N40-23..N40-29
# ---------------------------------------------------------------------------
if [ "${LIVE_MODE}" -eq 1 ]; then
  e2e_step "live_ids_check" "Read-only close-time verification of committed registries/gates.toml"
  e2e_assert_file "live_ids_file_exists" "${LIVE_IDS_FILE}" --gate "GATE-GAUNT-10"
  e2e_assert_file "committed_gates_exists" "${COMMITTED_GATES}" --gate "GATE-GAUNT-10"

  e2e_cmd --name "validate_live_ids" --expect-exit "0,3" -- \
    cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
      --file "${COMMITTED_GATES}" \
      --repo-root "${E2E_REPO_ROOT}" \
      --live-ids "${LIVE_IDS_FILE}" \
      --json
  e2e_assert_eq "live_ids_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "live_ids_ok" "/ok" "true" --gate "GATE-GAUNT-10"
    e2e_metric "evidence_claim_known_answer" "1" "count" --observe --gate "GATE-GAUNT-10"
    e2e_metric "evidence_tier_g0_host_d0_seed0" "0" "seed" --observe --gate "GATE-GAUNT-10"
  fi
  exit 0
fi

# ---------------------------------------------------------------------------
# Synthetic scratch workspace setup (three kill gates + read convergence gate)
# ---------------------------------------------------------------------------
SCRATCH_ROOT="${E2E_WORK_DIR}/scratch_repo"
mkdir -p \
  "${SCRATCH_ROOT}/receipts/gates" \
  "${SCRATCH_ROOT}/receipts/pilots" \
  "${SCRATCH_ROOT}/docs/adr" \
  "${SCRATCH_ROOT}/registries"

# Exact BLAKE3 digest of "{}\n" (3 bytes).
RECEIPT_HEX="a7c4d54776fcb179f048728766a52d75dc450cc2a5072b7e2c238aa9b2cff3b6"
printf '{}\n' > "${SCRATCH_ROOT}/receipts/gates/${RECEIPT_HEX}.json"
printf '{}\n' > "${SCRATCH_ROOT}/receipts/pilots/${RECEIPT_HEX}.json"

for adr_name in \
  ADR-KILL-SYN-A-M1.md \
  ADR-KILL-SYN-B-M1.md \
  ADR-KILL-SYN-C-M1.md \
  ADR-KILL-SYN-C-M1-final.md; do
  cat > "${SCRATCH_ROOT}/docs/adr/${adr_name}" <<EOF
# ${adr_name}

- Outcome: recorded in synthetic N40-03 E2E run
- Receipt: ${RECEIPT_HEX}
EOF
done

COMMIT_SHA="0123456789abcdef0123456789abcdef01234567"
VALID_CLAIM_ID="clm:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
VALID_EVIDENCE_ID="ev:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

CONTROLS_FILE="${SCRATCH_ROOT}/controls.toml"
cat > "${CONTROLS_FILE}" <<'EOF'
schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-001"
gate = ["GATE-SYN-01"]
kind = "mutation"
target = "crates/frob-core/src/lib.rs"
defect = "synthetic defect"
expect = "fails check"
due = "M1"
last_seen = { caught = true, session = "s-review-1", date = "2027-03-01" }
EOF

SEEDS_FILE="${SCRATCH_ROOT}/seeds.toml"
cat > "${SEEDS_FILE}" <<'EOF'
[[seed]]
id = "KILL-SYN-A"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]

[[seed]]
id = "KILL-SYN-B"
trigger = "backend parity gap exceeds 5%"
fallback = "CPU-only backend fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]

[[seed]]
id = "KILL-SYN-C"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
EOF

RECEIPT_TIMES_FILE="${SCRATCH_ROOT}/receipt_times.txt"
cat > "${RECEIPT_TIMES_FILE}" <<'EOF'
GATE-SYN-01 2027-03-01T08:00:00Z
EOF

REG_BEFORE="${SCRATCH_ROOT}/registries/00_before.toml"
cat > "${REG_BEFORE}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01"
kind = "convergence"
claim = "known-answer"
claim_id = "INV-SYN-01"
section = "12.1"
threshold = "err <= 1e-6"
milestone = "M1"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]

[[gate]]
id = "KILL-SYN-B"
kind = "kill"
section = "12.2"
trigger = "backend parity gap exceeds 5%"
fallback = "CPU-only backend fallback"
decided = "M1"
mode = "exit"
scope_kind = "backend"
reads = ["GATE-SYN-01"]

[[gate]]
id = "KILL-SYN-C"
kind = "kill"
section = "12.3"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
EOF

REG_NOT_FIRED="${SCRATCH_ROOT}/registries/01_not_fired.toml"
cat > "${REG_NOT_FIRED}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01"
kind = "convergence"
claim = "known-answer"
claim_id = "${VALID_CLAIM_ID}"
section = "12.1"
threshold = "err <= 1e-6"
milestone = "M1"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "err = 1e-9"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --lib register::kill::"
receipt = "${RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2027-03-01", session = "s-review-1", implemented_by = "s-impl-1" }

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
decision = { outcome = "not-fired", adr = "docs/adr/ADR-KILL-SYN-A-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] }

[[gate]]
id = "KILL-SYN-B"
kind = "kill"
section = "12.2"
trigger = "backend parity gap exceeds 5%"
fallback = "CPU-only backend fallback"
decided = "M1"
mode = "exit"
scope_kind = "backend"
reads = ["GATE-SYN-01"]
decision = [
  { outcome = "not-fired", scope = "backend:gpu-cuda", adr = "docs/adr/ADR-KILL-SYN-B-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] },
  { outcome = "not-fired", scope = "backend:gpu-metal", adr = "docs/adr/ADR-KILL-SYN-B-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] },
]

[[gate]]
id = "KILL-SYN-C"
kind = "kill"
section = "12.3"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
decision = { outcome = "not-fired", adr = "docs/adr/ADR-KILL-SYN-C-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] }
EOF

# ---------------------------------------------------------------------------
# Step 0: Run all 26 N40-03 unit tests (`register::kill::`)
# ---------------------------------------------------------------------------
e2e_step "unit_tests" "Run frob-conformance register::kill unit tests (26 tests)"

e2e_cmd --name "kill_unit_tests" -- \
  cargo test --manifest-path "${MANIFEST}" -p frob-conformance --lib register::kill::
e2e_assert_eq "kill_unit_tests_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"

# ---------------------------------------------------------------------------
# Step 1: Not-fired decisions with receipts -> exit query lists zero missing
# ---------------------------------------------------------------------------
e2e_step "not_fired_due_zero" "Record not-fired decisions with receipts and assert zero missing at M1"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  DIGEST_BEFORE="$("${FROB_E2E_TOOL}" cmp sha256 "${REG_BEFORE}")"
  DIGEST_AFTER="$("${FROB_E2E_TOOL}" cmp sha256 "${REG_NOT_FIRED}")"
  e2e_assert_eq "register_changed_after_decision" "$([ "${DIGEST_BEFORE}" != "${DIGEST_AFTER}" ] && printf yes || printf no)" "yes" --gate "GATE-GAUNT-10"
fi

e2e_cmd --name "validate_not_fired" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_NOT_FIRED}" \
    --repo-root "${SCRATCH_ROOT}" \
    --controls "${CONTROLS_FILE}" \
    --live-claims "${VALID_CLAIM_ID}" \
    --live-evidence "${VALID_EVIDENCE_ID}" \
    --seeds "${SEEDS_FILE}" \
    --check-adr-files \
    --kill-due M1 \
    --json
e2e_assert_eq "not_fired_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "not_fired_ok" "/ok" "true" --gate "GATE-GAUNT-10"
  e2e_assert_json "not_fired_due_total" "/kill_due_count" "3" --gate "GATE-GAUNT-10"
  e2e_assert_json "not_fired_due_missing_zero" "/kill_due_missing_count" "0" --gate "GATE-GAUNT-10"
fi

# ---------------------------------------------------------------------------
# Step 2: Lower trigger after receipt exists -> amended-after-measurement (RC-10)
# ---------------------------------------------------------------------------
e2e_step "amended_after_measurement" "Lower trigger after receipt timestamp and assert amended-after-measurement"

REG_LATE_AMD="${SCRATCH_ROOT}/registries/02_late_amendment.toml"
cat > "${REG_LATE_AMD}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01"
kind = "convergence"
claim = "known-answer"
claim_id = "INV-SYN-01"
section = "12.1"
threshold = "err <= 1e-6"
milestone = "M1"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after three repair cycles"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]

[[amendment]]
id = "AMD-SYN-001"
gate = "KILL-SYN-A"
field = "trigger"
old = "GATE-SYN-01 fails after one repair cycle"
new = "GATE-SYN-01 fails after three repair cycles"
reason = "post-hoc weakening after reading receipt"
date = "2027-03-05T12:00:00Z"
commit = "${COMMIT_SHA}"
EOF

e2e_cmd --name "plant_amended_after_measurement" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_LATE_AMD}" \
    --repo-root "${SCRATCH_ROOT}" \
    --seeds "${SEEDS_FILE}" \
    --receipt-times "${RECEIPT_TIMES_FILE}" \
    --json
if e2e_plant "amended-after-measurement"; then
  e2e_assert_eq "plant_active_amended_after" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10" --cx "RC-10"
else
  e2e_assert_eq "late_amendment_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10" --cx "RC-10"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "late_amendment_reason" "/violations/0/reason" "amended-after-measurement" --gate "GATE-GAUNT-10" --cx "RC-10"
    e2e_assert_json "late_amendment_gate" "/violations/0/gate_id" "KILL-SYN-A" --gate "GATE-GAUNT-10" --cx "RC-10"
    e2e_metric "amendment_after_receipt_delta_days" "4" "days" --observe --gate "GATE-GAUNT-10"
  fi
fi

# ---------------------------------------------------------------------------
# Step 3: Edit fallback with no amendment -> changed-without-amendment (RC-11)
# ---------------------------------------------------------------------------
e2e_step "changed_without_amendment" "Edit fallback without amendment and assert changed-without-amendment"

REG_UNAMENDED="${SCRATCH_ROOT}/registries/03_unamended_fallback.toml"
cat > "${REG_UNAMENDED}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "silently altered fallback text"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
EOF

e2e_cmd --name "plant_changed_without_amendment" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_UNAMENDED}" \
    --repo-root "${SCRATCH_ROOT}" \
    --seeds "${SEEDS_FILE}" \
    --json
if e2e_plant "changed-without-amendment"; then
  e2e_assert_eq "plant_active_unamended" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10" --cx "RC-11"
else
  e2e_assert_eq "unamended_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10" --cx "RC-11"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "unamended_reason" "/violations/0/reason" "changed-without-amendment" --gate "GATE-GAUNT-10" --cx "RC-11"
    e2e_assert_json "unamended_gate" "/violations/0/gate_id" "KILL-SYN-A" --gate "GATE-GAUNT-10" --cx "RC-11"
  fi
fi

# ---------------------------------------------------------------------------
# Step 4: Fire KILL-SYN-A -> apply_decisions README & register printouts use fallback name
#         (Round-4 Review Notes #1 & #7: asserted directly on apply_decisions outputs)
# ---------------------------------------------------------------------------
e2e_step "fired_renaming" "Fire kill gate and assert apply_decisions README table and register printout print fallback name"

REG_FIRED="${SCRATCH_ROOT}/registries/04_fired.toml"
cat > "${REG_FIRED}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "GATE-SYN-01"
kind = "convergence"
claim = "known-answer"
claim_id = "INV-SYN-01"
section = "12.1"
threshold = "err <= 1e-6"
milestone = "M1"
tier = ["G0:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "fail"
measured = "err = 3.2e-2"
commit = "${COMMIT_SHA}"
command = "cargo test -p frob-conformance --lib register::kill::"
receipt = "${RECEIPT_HEX}"
fingerprint = "fp:host-0001"
seed = "0"
evidence = ["${VALID_EVIDENCE_ID}"]
recorded = { date = "2027-03-01", session = "s-review-1", implemented_by = "s-impl-1" }

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
decision = { outcome = "fired", adr = "docs/adr/ADR-KILL-SYN-A-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] }
EOF

e2e_cmd --name "validate_fired_apply_decisions" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_FIRED}" \
    --repo-root "${SCRATCH_ROOT}" \
    --controls "${CONTROLS_FILE}" \
    --check-adr-files \
    --apply-decisions \
    --json
e2e_assert_eq "fired_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10" --cx "RC-12"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "fired_ok" "/ok" "true" --gate "GATE-GAUNT-10" --cx "RC-12"
  README_TBL="$(e2e_get /readme_table)"
  REG_PRINTOUT="$(e2e_get /register_printout)"
  HAS_FB_README="no"
  HAS_FB_PRINTOUT="no"
  HAS_KILLED_STATUS="no"
  NO_PASS_LEAK="yes"
  case "${README_TBL}" in
    *"rigid-link fallback"*) HAS_FB_README="yes" ;;
  esac
  case "${REG_PRINTOUT}" in
    *"rigid-link fallback"*) HAS_FB_PRINTOUT="yes" ;;
  esac
  case "${README_TBL}" in
    *"killed: fallback adopted"*) HAS_KILLED_STATUS="yes" ;;
  esac
  case "${README_TBL}" in
    *"| pass |"*) NO_PASS_LEAK="no" ;;
  esac
  if e2e_plant "fired-renamed"; then
    e2e_assert_eq "plant_active_fired_renamed" "${HAS_FB_README}" "no" --gate "GATE-GAUNT-10" --cx "RC-12"
  else
    e2e_assert_eq "readme_prints_fallback_name" "${HAS_FB_README}" "yes" --gate "GATE-GAUNT-10" --cx "RC-12"
    e2e_assert_eq "printout_prints_fallback_name" "${HAS_FB_PRINTOUT}" "yes" --gate "GATE-GAUNT-10" --cx "RC-12"
    e2e_assert_eq "readme_prints_killed_fallback_adopted" "${HAS_KILLED_STATUS}" "yes" --gate "GATE-GAUNT-10" --cx "RC-12"
    e2e_assert_eq "readme_never_prints_killed_as_pass" "${NO_PASS_LEAK}" "yes" --gate "GATE-GAUNT-10" --cx "RC-12"
  fi
fi

# ---------------------------------------------------------------------------
# Step 5: ADR recorded from an implementer session -> self-certified (INV-GAUNT-05)
# ---------------------------------------------------------------------------
e2e_step "self_certified_adr" "Record decision ADR from implementer session and assert self-certified"

REG_SELF_CERT="${SCRATCH_ROOT}/registries/05_self_certified.toml"
cat > "${REG_SELF_CERT}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
decision = { outcome = "not-fired", adr = "docs/adr/ADR-KILL-SYN-A-M1.md", date = "2027-03-02", session = "s-impl-1", receipts = ["${RECEIPT_HEX}"] }
EOF

e2e_cmd --name "plant_self_certified_adr" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_SELF_CERT}" \
    --repo-root "${SCRATCH_ROOT}" \
    --implementers "s-impl-1,s-impl-2" \
    --check-adr-files \
    --json
if e2e_plant "self-certified"; then
  e2e_assert_eq "plant_active_self_cert" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
else
  e2e_assert_eq "self_cert_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "self_cert_reason" "/violations/0/reason" "self-certified" --gate "GATE-GAUNT-10"
    e2e_assert_json "self_cert_gate" "/violations/0/gate_id" "KILL-SYN-A" --gate "GATE-GAUNT-10"
  fi
fi

# ---------------------------------------------------------------------------
# Step 6: Per-scope decision (`backend:gpu-cuda` fired, `backend:gpu-metal` not-fired)
#         + `reading = "final"` second entry + plants RC-68 & RC-69
# ---------------------------------------------------------------------------
e2e_step "scoped_and_second_reading" "Verify per-scope decisions, second reading ADR paths, and plants RC-68/RC-69"

REG_SCOPED_OK="${SCRATCH_ROOT}/registries/06_scoped_ok.toml"
cat > "${REG_SCOPED_OK}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-B"
kind = "kill"
section = "12.2"
trigger = "backend parity gap exceeds 5%"
fallback = "CPU-only backend fallback"
decided = "M1"
mode = "exit"
scope_kind = "backend"
reads = ["GATE-SYN-01"]
decision = [
  { outcome = "fired", scope = "backend:gpu-cuda", adr = "docs/adr/ADR-KILL-SYN-B-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] },
  { outcome = "not-fired", scope = "backend:gpu-metal", adr = "docs/adr/ADR-KILL-SYN-B-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] },
]

[[gate]]
id = "KILL-SYN-C"
kind = "kill"
section = "12.3"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
decision = [
  { outcome = "not-fired", exit = "M1", adr = "docs/adr/ADR-KILL-SYN-C-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] },
  { outcome = "fired", exit = "M1", reading = "final", adr = "docs/adr/ADR-KILL-SYN-C-M1-final.md", date = "2027-03-04", session = "s-review-2", receipts = ["${RECEIPT_HEX}"] },
]
EOF

e2e_cmd --name "validate_scoped_and_final" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_SCOPED_OK}" \
    --repo-root "${SCRATCH_ROOT}" \
    --check-adr-files \
    --apply-decisions \
    --json
e2e_assert_eq "scoped_ok_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "scoped_ok" "/ok" "true" --gate "GATE-GAUNT-10"
  SCOPED_PRINTOUT="$(e2e_get /register_printout)"
  CUDA_FIRED="no"
  METAL_KEPT="no"
  FINAL_ADR_IN_FORCE="no"
  case "${SCOPED_PRINTOUT}" in
    *"scope=backend:gpu-cuda capability=\"CPU-only backend fallback\" status=fired adr=docs/adr/ADR-KILL-SYN-B-M1.md"*) CUDA_FIRED="yes" ;;
  esac
  case "${SCOPED_PRINTOUT}" in
    *"scope=backend:gpu-metal status=not-fired adr=docs/adr/ADR-KILL-SYN-B-M1.md"*) METAL_KEPT="yes" ;;
  esac
  case "${SCOPED_PRINTOUT}" in
    *"KILL-SYN-C [kill] capability=\"finite-difference stencil fallback\" status=fired adr=docs/adr/ADR-KILL-SYN-C-M1-final.md"*) FINAL_ADR_IN_FORCE="yes" ;;
  esac
  e2e_assert_eq "backend_gpu_cuda_fired" "${CUDA_FIRED}" "yes" --gate "GATE-GAUNT-10" --ka "R-32"
  e2e_assert_eq "backend_gpu_metal_not_fired" "${METAL_KEPT}" "yes" --gate "GATE-GAUNT-10" --ka "R-32"
  e2e_assert_eq "second_reading_final_adr_in_force" "${FINAL_ADR_IN_FORCE}" "yes" --gate "GATE-GAUNT-10"
fi

# Plant RC-68: `family:cuda` on a `backend` row -> decision-scope-invalid
REG_BAD_SCOPE="${SCRATCH_ROOT}/registries/06b_bad_scope.toml"
cat > "${REG_BAD_SCOPE}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-B"
kind = "kill"
section = "12.2"
trigger = "backend parity gap exceeds 5%"
fallback = "CPU-only backend fallback"
decided = "M1"
mode = "exit"
scope_kind = "backend"
reads = ["GATE-SYN-01"]
decision = { outcome = "fired", scope = "family:cuda", adr = "docs/adr/ADR-KILL-SYN-B-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] }
EOF

e2e_cmd --name "plant_bad_scope" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_BAD_SCOPE}" \
    --repo-root "${SCRATCH_ROOT}" \
    --json
if e2e_plant "decision-scope-invalid"; then
  e2e_assert_eq "plant_active_bad_scope" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10" --cx "RC-68"
else
  e2e_assert_eq "bad_scope_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10" --cx "RC-68"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "bad_scope_reason" "/violations/0/reason" "decision-scope-invalid" --gate "GATE-GAUNT-10" --cx "RC-68"
  fi
fi

# Plant RC-69: duplicate entry at one exit without `reading` -> decision-entry-duplicate
REG_DUP_ENTRY="${SCRATCH_ROOT}/registries/06c_dup_entry.toml"
cat > "${REG_DUP_ENTRY}" <<EOF
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-C"
kind = "kill"
section = "12.3"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
decision = [
  { outcome = "not-fired", exit = "M1", adr = "docs/adr/ADR-KILL-SYN-C-M1.md", date = "2027-03-02", session = "s-review-1", receipts = ["${RECEIPT_HEX}"] },
  { outcome = "fired", exit = "M1", adr = "docs/adr/ADR-KILL-SYN-C-M1.md", date = "2027-03-04", session = "s-review-2", receipts = ["${RECEIPT_HEX}"] },
]
EOF

e2e_cmd --name "plant_dup_entry" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_DUP_ENTRY}" \
    --repo-root "${SCRATCH_ROOT}" \
    --json
if e2e_plant "decision-entry-duplicate"; then
  e2e_assert_eq "plant_active_dup_entry" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10" --cx "RC-69"
else
  e2e_assert_eq "dup_entry_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10" --cx "RC-69"
  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    e2e_assert_json "dup_entry_reason" "/violations/0/reason" "decision-entry-duplicate" --gate "GATE-GAUNT-10" --cx "RC-69"
  fi
fi

# ---------------------------------------------------------------------------
# Step 7: Exercise `--live-ids` on synthetic register (valid, missing fallback, changed trigger)
# ---------------------------------------------------------------------------
e2e_step "live_ids_self_test" "Exercise --live --ids check on synthetic register (pass, missing fallback, changed trigger)"

SYN_IDS_FILE="${SCRATCH_ROOT}/synthetic_ids.txt"
e2e_cmd --name "dump_synthetic_seed_hashes" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_BEFORE}" \
    --print-seed-hashes
e2e_assert_eq "dump_seed_hashes_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  cp "${E2E_LAST_STDOUT}" "${SYN_IDS_FILE}"
else
  printf 'KILL-SYN-A 0000000000000000000000000000000000000000000000000000000000000000\n' > "${SYN_IDS_FILE}"
fi

# 7a. Clean synthetic register matches all three seed hashes -> exit 0.
e2e_cmd --name "live_ids_syn_clean" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_BEFORE}" \
    --live-ids "${SYN_IDS_FILE}" \
    --json
e2e_assert_eq "live_ids_syn_clean_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "live_ids_syn_clean_ok" "/ok" "true" --gate "GATE-GAUNT-10"
fi

# 7b. Missing fallback on KILL-SYN-B -> exit 3 naming KILL-SYN-B.
REG_MISSING_FB="${SCRATCH_ROOT}/registries/07a_missing_fallback.toml"
cat > "${REG_MISSING_FB}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after one repair cycle"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]

[[gate]]
id = "KILL-SYN-B"
kind = "kill"
section = "12.2"
trigger = "backend parity gap exceeds 5%"
decided = "M1"
mode = "exit"
scope_kind = "backend"
reads = ["GATE-SYN-01"]

[[gate]]
id = "KILL-SYN-C"
kind = "kill"
section = "12.3"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
EOF

e2e_cmd --name "live_ids_syn_missing_fb" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_MISSING_FB}" \
    --live-ids "${SYN_IDS_FILE}" \
    --json
e2e_assert_eq "live_ids_missing_fb_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  ERR_FB="$(e2e_get /live_ids_error)"
  NAMES_B="no"
  case "${ERR_FB}" in
    *KILL-SYN-B*fallback*) NAMES_B="yes" ;;
  esac
  e2e_assert_eq "live_ids_names_missing_fallback_row" "${NAMES_B}" "yes" --gate "GATE-GAUNT-10"
fi

# 7c. Changed trigger on KILL-SYN-A -> exit 3 naming KILL-SYN-A.
REG_CHANGED_TRIG="${SCRATCH_ROOT}/registries/07b_changed_trigger.toml"
cat > "${REG_CHANGED_TRIG}" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "KILL-SYN-A"
kind = "kill"
section = "12.1"
trigger = "GATE-SYN-01 fails after nine repair cycles"
fallback = "rigid-link fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]

[[gate]]
id = "KILL-SYN-B"
kind = "kill"
section = "12.2"
trigger = "backend parity gap exceeds 5%"
fallback = "CPU-only backend fallback"
decided = "M1"
mode = "exit"
scope_kind = "backend"
reads = ["GATE-SYN-01"]

[[gate]]
id = "KILL-SYN-C"
kind = "kill"
section = "12.3"
trigger = "short-horizon gradient cosine below 0.5"
fallback = "finite-difference stencil fallback"
decided = "M1"
mode = "exit"
reads = ["GATE-SYN-01"]
EOF

e2e_cmd --name "live_ids_syn_changed_trigger" --expect-exit "3" -- \
  cargo run --manifest-path "${MANIFEST}" -p frob-conformance --example validate_register -- \
    --file "${REG_CHANGED_TRIG}" \
    --live-ids "${SYN_IDS_FILE}" \
    --json
e2e_assert_eq "live_ids_changed_trig_exit" "${E2E_LAST_EXIT}" "3" --gate "GATE-GAUNT-10"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  ERR_TR="$(e2e_get /live_ids_error)"
  NAMES_A="no"
  case "${ERR_TR}" in
    *KILL-SYN-A*"text differs"*) NAMES_A="yes" ;;
  esac
  e2e_assert_eq "live_ids_names_changed_trigger_row" "${NAMES_A}" "yes" --gate "GATE-GAUNT-10"
fi

# ---------------------------------------------------------------------------
# Evidence block summary metrics (claim: known-answer, class: Tested, tier: G0:host, D0, seed 0)
# ---------------------------------------------------------------------------
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_metric "evidence_claim_known_answer" "1" "count" --observe --gate "GATE-GAUNT-10"
  e2e_metric "evidence_class_tested" "1" "count" --observe --gate "GATE-GAUNT-10"
  e2e_metric "evidence_tier_g0_host_d0_seed0" "0" "seed" --observe --gate "GATE-GAUNT-10"
fi
