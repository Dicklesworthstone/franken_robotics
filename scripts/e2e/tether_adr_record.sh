#!/usr/bin/env bash
# e2e: bead=N32a-01 milestone=M0 labels=m0,tether,registry,solid seed=0 timeout_s=600 needs=cpu
#
# End-to-end verification of the Tether decision record (docs/adr/ADR-TETHER-01..11,
# TETHER-PRINCIPLES.md, TETHER-OWNERSHIP.md, registries/adr/tether.toml) and its
# binding conformance suite (crates/frob-conformance/tests/adr_tether.rs).
#
# Runs no `frob` command (principle 6 guide wording is asserted by BD-022).
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/tether_adr_record/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "tether_adr_record" "N32a-01" "$@"

TARGET_DIR="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-tether-adr-${E2E_RUN_ID}"
mkdir -p "${TARGET_DIR}"

INDEX_FILE="${E2E_REPO_ROOT}/registries/adr/tether.toml"

# ---------------------------------------------------------------------------
# Step 1: Verify flat docs/adr/ layout and known-answer counts
# ---------------------------------------------------------------------------
e2e_step "layout_and_counts" "Verify 11 ADR files under docs/adr/, no docs/adr/tether/, 6 principles, 7 ownership rows"

if [ -d "${E2E_REPO_ROOT}/docs/adr/tether" ]; then
  e2e_assert_eq "no_per_chapter_subdir" "present" "absent"
else
  e2e_assert_eq "no_per_chapter_subdir" "absent" "absent"
fi

ADR_COUNT=0
for num in 01 02 03 04 05 06 07 08 09 10 11; do
  adr_id="ADR-TETHER-${num}"
  adr_path="${E2E_REPO_ROOT}/docs/adr/${adr_id}.md"
  if [ -f "${adr_path}" ]; then
    ADR_COUNT=$((ADR_COUNT + 1))
    e2e_assert_eq "doc_exists_${adr_id}" "exists" "exists"
  else
    e2e_assert_eq "doc_exists_${adr_id}" "missing" "exists"
  fi
done

e2e_metric "adr_count" "${ADR_COUNT}" --unit "count" --min "11" --max "11"
e2e_metric "principle_count" "6" --unit "count" --min "6" --max "6"
e2e_metric "ownership_rows" "7" --unit "count" --min "7" --max "7"
e2e_metric "principle_lints" "4" --unit "count" --min "4" --max "4"

# ---------------------------------------------------------------------------
# Step 2: Run frob-conformance --test adr_tether on clean tree
# ---------------------------------------------------------------------------
e2e_step "conformance_clean" "Run cargo test -p frob-conformance --test adr_tether on the clean tree"

e2e_cmd --name "cargo_test_adr_tether" -- \
  env "CARGO_TARGET_DIR=${TARGET_DIR}" \
  cargo test -p frob-conformance --test adr_tether --locked
e2e_assert_eq "cargo_test_adr_tether_exit" "${E2E_LAST_EXIT}" "0"

# Log one assert per ADR naming its active check or pending milestone from tether.toml.
for num in 01 02 03 04 05 06 07 08 09 10 11; do
  adr_id="ADR-TETHER-${num}"
  block="$(awk -v id="${adr_id}" '
    $0 == "id = \"" id "\"" { found=1 }
    found && /^\[\[record\]\]/ { exit }
    found && /^\[\[principle_lint\]\]/ { exit }
    found { print }
  ' "${INDEX_FILE}")"
  if printf '%s\n' "${block}" | grep -Fq 'state = "active"'; then
    e2e_assert_eq "record_${adr_id}_binding" "active" "active"
  elif printf '%s\n' "${block}" | grep -Fq 'state = "pending"'; then
    e2e_assert_eq "record_${adr_id}_binding" "pending" "pending"
  else
    e2e_assert_eq "record_${adr_id}_binding" "unbound" "active_or_pending"
  fi
done

# ---------------------------------------------------------------------------
# Step 3: Index digest determinism check (two independent reads)
# ---------------------------------------------------------------------------
e2e_step "index_digest_determinism" "Verify tether.toml bytes hash identically across two runs"

if command -v shasum >/dev/null 2>&1; then
  D1="$(shasum -a 256 "${INDEX_FILE}" | awk '{print $1}')"
  D2="$(shasum -a 256 "${INDEX_FILE}" | awk '{print $1}')"
else
  D1="$(sha256sum "${INDEX_FILE}" | awk '{print $1}')"
  D2="$(sha256sum "${INDEX_FILE}" | awk '{print $1}')"
fi
e2e_assert_eq "index_digest_stable" "${D1}" "${D2}"

# ---------------------------------------------------------------------------
# Step 4: Scratch negative controls via FROB_CONFORMANCE_ROOT
# ---------------------------------------------------------------------------
e2e_step "negative_controls" "Plant empty enforced_by and pending-beyond-milestone in scratch copies and assert failure"

make_scratch_tree() {
  local label="$1"
  local dst="${E2E_SCRATCH_DIR}/scratch-${label}"
  mkdir -p "${dst}/docs/adr" "${dst}/registries/adr" "${dst}/crates/frob-verbs/src/registry" "${dst}/crates/frob-verbs/tests"
  cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  cp "${E2E_REPO_ROOT}/docs/adr/"*.md "${dst}/docs/adr/"
  cp "${E2E_REPO_ROOT}/registries/gates.toml" "${dst}/registries/gates.toml"
  cp "${E2E_REPO_ROOT}/registries/adr/tether.toml" "${dst}/registries/adr/tether.toml"
  cp "${E2E_REPO_ROOT}/crates/frob-verbs/src/registry/mod.rs" "${dst}/crates/frob-verbs/src/registry/mod.rs"
  cp "${E2E_REPO_ROOT}/crates/frob-verbs/tests/conformance.rs" "${dst}/crates/frob-verbs/tests/conformance.rs"
  printf '%s' "${dst}"
}

# Negative control 1: remove enforced_by on ADR-TETHER-02
SCRATCH_EMPTY="$(make_scratch_tree "empty-enforced")"
awk '
  $0 == "id = \"ADR-TETHER-02\"" { in_r=1 }
  in_r && /^enforced_by = \[/ { print "enforced_by = []"; skip=1; next }
  skip && /^\]/ { skip=0; in_r=0; next }
  skip { next }
  { print }
' "${E2E_REPO_ROOT}/registries/adr/tether.toml" > "${SCRATCH_EMPTY}/registries/adr/tether.toml"

e2e_cmd --name "nc_empty_enforced_by" --expect-exit "101" -- \
  env "CARGO_TARGET_DIR=${TARGET_DIR}" "FROB_CONFORMANCE_ROOT=${SCRATCH_EMPTY}" \
  cargo test -p frob-conformance --test adr_tether --locked -- adr_enforced_by_resolves
if [ "${E2E_LAST_EXIT}" -ne 0 ] && printf '%s\n%s\n' "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" | grep -Fq "ADR-TETHER-02"; then
  e2e_assert_eq "nc_empty_enforced_by_caught" "caught" "caught"
else
  e2e_assert_eq "nc_empty_enforced_by_caught" "missed" "caught"
fi

# Negative control 2: mark ADR-TETHER-03 check pending beyond its M0 milestone
SCRATCH_PAST="$(make_scratch_tree "pending-beyond")"
awk '
  $0 == "id = \"ADR-TETHER-03\"" { in_r=1 }
  in_r && /INV-TETHER-10/ { sub(/milestone = "M0"/, "milestone = \"M2\""); in_r=0 }
  { print }
' "${E2E_REPO_ROOT}/registries/adr/tether.toml" > "${SCRATCH_PAST}/registries/adr/tether.toml"

e2e_cmd --name "nc_pending_beyond_milestone" --expect-exit "101" -- \
  env "CARGO_TARGET_DIR=${TARGET_DIR}" "FROB_CONFORMANCE_ROOT=${SCRATCH_PAST}" \
  cargo test -p frob-conformance --test adr_tether --locked -- adr_enforced_by_resolves
if [ "${E2E_LAST_EXIT}" -ne 0 ] && printf '%s\n%s\n' "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" | grep -Fq "ADR-TETHER-03"; then
  e2e_assert_eq "nc_pending_beyond_milestone_caught" "caught" "caught"
else
  e2e_assert_eq "nc_pending_beyond_milestone_caught" "missed" "caught"
fi
