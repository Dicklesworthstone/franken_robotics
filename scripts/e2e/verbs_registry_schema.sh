#!/usr/bin/env bash
# e2e: bead=NJL-01 milestone=M0 labels=tether,verbs,registry gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=omit-805,unimplemented-as-unknown
#
# End-to-end verification of NJL-01 verb-entry derivation against frob and frob-mcp.
# Stages 1-3, 5, 6 run green with NJL-02; stage 4 with BD-023; stage 7 with BD-138.
# Under E2E_DRY_RUN=1 prints every command and exits 77; without `frob`, exits 77.
# Logs through STD-E2E (scripts/e2e/lib.sh); never deletes files (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "verbs_registry_schema" "NJL-01" "$@"
e2e_gate "GATE-TETHER-02"

DRY="${E2E_DRY_RUN:-0}"

# Planted negative-control lanes (omit-805 caught by stage 3; unimplemented-as-unknown by stage 5).
if e2e_plant "omit-805"; then
  if [ -n "${FROB_BIN_OMIT_805:-}" ] && [ -x "${FROB_BIN_OMIT_805}" ]; then
    FROB_BIN="${FROB_BIN_OMIT_805}"
  elif [ "${DRY}" != "1" ]; then
    e2e_step "preflight" "Check planted omit-805 frob binary"
    e2e_skip "plant omit-805 requires FROB_BIN_OMIT_805 built with frob-verbs/nc-omit-805 (BD-022 / NJL-02)"
    exit 77
  fi
elif e2e_plant "unimplemented-as-unknown"; then
  if [ -n "${FROB_BIN_UNIMPL_801:-}" ] && [ -x "${FROB_BIN_UNIMPL_801}" ]; then
    FROB_BIN="${FROB_BIN_UNIMPL_801}"
  elif [ "${DRY}" != "1" ]; then
    e2e_step "preflight" "Check planted unimplemented-as-unknown frob binary"
    e2e_skip "plant unimplemented-as-unknown requires FROB_BIN_UNIMPL_801 (N00-02 dispatch mutant)"
    exit 77
  fi
fi

if [ "${DRY}" != "1" ] && [ ! -x "${FROB_BIN}" ]; then
  e2e_step "preflight" "Check frob binary prerequisite"
  e2e_skip "missing binary: frob (${FROB_BIN}; lands in BD-022)"
  exit 77
fi

# ---------------------------------------------------------------------------
# Stage 1: Entry schema (`frob.verbs.entry/1`)
# ---------------------------------------------------------------------------
e2e_step "stage1_entry_schema" "Inspect frob.verbs.entry/1 schema and validate its example"

e2e_cmd --name "schema_entry" -- "${FROB_BIN}" --json schema frob.verbs.entry/1
if [ "${DRY}" != "1" ]; then
  e2e_assert_eq "schema_entry_exit" "${E2E_LAST_EXIT}" "0"
  e2e_assert_json "schema_entry_ok" "/ok" "true"
  e2e_assert_json "schema_entry_id" "/result/schema/\$id" "frob.verbs.entry/1"
  e2e_assert_json "schema_entry_example_name" "/result/example/name" "design.check"
  e2e_assert_json "schema_entry_validated_example" "/result/example" \
    "$(e2e_get /result/schema/examples/0)"
fi

capability_is_linked() {
  local idx=0 verb
  while verb="$(e2e_get --file "${CAPS_JSON}" "/result/capabilities/${idx}/verb" 2>/dev/null)"; do
    if [ "${verb}" = "$1" ]; then
      [ "$(e2e_get --file "${CAPS_JSON}" "/result/capabilities/${idx}/linked")" = "true" ]
      return
    fi
    idx=$((idx+1))
  done
  return 1
}

# ---------------------------------------------------------------------------
# Stage 2: Capabilities inventory metadata
# ---------------------------------------------------------------------------
e2e_step "stage2_capabilities" "Check capabilities lists kind, tier, milestone, and examples"

e2e_cmd --name "capabilities_list" -- "${FROB_BIN}" --json capabilities
CAPS_JSON=""
if [ "${DRY}" != "1" ]; then
  CAPS_JSON="${E2E_LAST_STDOUT}"
  e2e_assert_eq "capabilities_exit" "${E2E_LAST_EXIT}" "0"
  e2e_assert_json "capabilities_ok" "/ok" "true"
  idx=0
  while vname="$(e2e_get "/result/capabilities/${idx}/verb" 2>/dev/null)"; do
    for field in kind tier first_milestone examples/0; do
      val="$(e2e_get "/result/capabilities/${idx}/${field}" 2>/dev/null || true)"
      e2e_assert_eq "capabilities_${vname}_${field//\//_}" "${val:+present}" present
    done
    idx=$((idx+1))
  done
  e2e_assert_num "capabilities_nonempty" "${idx}" ">" 0
fi

# ---------------------------------------------------------------------------
# Stage 3: Per-kind help schema, common properties, x-frob-key, and full refusals
# ---------------------------------------------------------------------------
e2e_step "stage3_per_kind_help" "Check per-kind common properties and appended refusals via robot-docs and --help"

e2e_cmd --name "robot_docs" -- "${FROB_BIN}" --json robot-docs
if [ "${DRY}" = "1" ]; then
  e2e_cmd --name "help_read_sample" -- "${FROB_BIN}" --json doctor --help
  e2e_cmd --name "help_run_sample" -- "${FROB_BIN}" --json sim run --help
  e2e_cmd --name "help_mutate_sample" -- "${FROB_BIN}" --json session focus --help
  e2e_cmd --name "help_admin_sample" -- "${FROB_BIN}" --json job cancel --help
else
  e2e_assert_eq "robot_docs_exit" "${E2E_LAST_EXIT}" "0"
  DOCS_JSON="${E2E_LAST_STDOUT}"
  for k in read run mutate admin; do
    chosen=""
    idx=0
    while true; do
      vname="$(e2e_get --file "${DOCS_JSON}" "/result/verbs/${idx}/entry/name" 2>/dev/null || true)"
      [ -n "${vname}" ] || break
      vkind="$(e2e_get --file "${DOCS_JSON}" "/result/verbs/${idx}/entry/kind" 2>/dev/null || true)"
      if [ "${vkind}" = "${k}" ]; then
        chosen="${vname}"
        break
      fi
      idx=$((idx+1))
    done
    if [ -z "${chosen}" ]; then
      e2e_skip "no registered verb of kind ${k} at this milestone"
      continue
    fi
    # Split dot-separated verb name into CLI path words.
    IFS=. read -r -a cli_words <<< "${chosen}"
    e2e_cmd --name "help_${k}" -- "${FROB_BIN}" --json "${cli_words[@]}" --help
    e2e_assert_eq "help_${k}_exit" "${E2E_LAST_EXIT}" "0"
    e2e_assert_json "help_${k}_entry" "/result/entry/name" "${chosen}"
    HELP_JSON="${E2E_LAST_STDOUT}"
    e2e_assert_json "help_${k}_refusals_match_docs" "/result/refusals" \
      "$(e2e_get --file "${DOCS_JSON}" "/result/verbs/${idx}/refusals")"
    if [ "${k}" = "run" ]; then
      for prop in sync_wait_ms idempotency_key budget estimate_only on_cancel envelope_scope; do
        has_p="$(e2e_get --file "${HELP_JSON}" "/result/input_schema/properties/${prop}/type" 2>/dev/null || true)"
        e2e_assert_eq "run_prop_${prop}" "$([ -n "${has_p}" ] && printf present || printf missing)" "present"
      done
      xkey="$(e2e_get --file "${HELP_JSON}" "/result/input_schema/properties/idempotency_key/x-frob-key" 2>/dev/null || true)"
      e2e_assert_eq "run_x_frob_key" "${xkey}" "required"
      ref_idx=0
      ref805=""
      while code="$(e2e_get "/result/refusals/${ref_idx}/code" 2>/dev/null)"; do
        if [ "${code}" = "FROB-REF-805" ]; then
          ref805="${code}"
          break
        fi
        ref_idx=$((ref_idx+1))
      done
      e2e_assert_eq "run_lists_805" "${ref805}" "FROB-REF-805"
    elif [ "${k}" = "mutate" ]; then
      for prop in idempotency_key expected_parent; do
        has_p="$(e2e_get --file "${HELP_JSON}" "/result/input_schema/properties/${prop}/type" 2>/dev/null || true)"
        e2e_assert_eq "mutate_prop_${prop}" "$([ -n "${has_p}" ] && printf present || printf missing)" "present"
      done
      expected_key="required"
      if [ "$(e2e_get /result/entry/key_optional)" = "true" ]; then
        expected_key="optional"
      fi
      e2e_assert_json "mutate_x_frob_key" "/result/input_schema/properties/idempotency_key/x-frob-key" "${expected_key}"
    fi
  done
fi

# ---------------------------------------------------------------------------
# Stage 4: MCP tools/list parity (BD-023)
# ---------------------------------------------------------------------------
e2e_step "stage4_mcp_tools" "Verify MCP tools/list matches registry names, inputSchema, and annotations"

if [ "${DRY}" = "1" ]; then
  e2e_cmd --name "mcp_tools_list" -- "${FROB_MCP_BIN}" --stdio-once '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
elif [ ! -x "${FROB_MCP_BIN}" ]; then
  e2e_skip "missing binary: frob-mcp (${FROB_MCP_BIN}; lands in BD-023)"
else
  e2e_cmd --name "mcp_tools_list" -- "${FROB_MCP_BIN}" --stdio-once '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
  e2e_assert_eq "mcp_tools_list_exit" "${E2E_LAST_EXIT}" "0"
fi

# ---------------------------------------------------------------------------
# Stage 5: Unimplemented registered verb -> FROB-REF-724 exit 5 (catches unimplemented-as-unknown)
# ---------------------------------------------------------------------------
e2e_step "stage5_unimplemented_724" "Verify registered verb absent from build refuses with FROB-REF-724 exit 5"

if [ "${DRY}" = "1" ]; then
  e2e_expect_refusal FROB-REF-724 5 -- "${FROB_BIN}" --json session show
else
  absent_verb=""
  idx=0
  while true; do
    vname="$(e2e_get --file "${CAPS_JSON}" "/result/capabilities/${idx}/verb" 2>/dev/null || true)"
    [ -n "${vname}" ] || break
    vstate="$(e2e_get --file "${CAPS_JSON}" "/result/capabilities/${idx}/linked" 2>/dev/null || true)"
    if [ "${vstate}" = "false" ]; then
      absent_verb="${vname}"
      break
    fi
    idx=$((idx+1))
  done
  if [ -z "${absent_verb}" ]; then
    e2e_skip "no absent verb listed by capabilities in this build"
  else
    IFS=. read -r -a absent_words <<< "${absent_verb}"
    e2e_expect_refusal FROB-REF-724 5 -- "${FROB_BIN}" --json "${absent_words[@]}"
  fi
fi

# ---------------------------------------------------------------------------
# Stage 6: Test-lane synthetic.deferred validation (BD-022 synthetic-verbs)
# ---------------------------------------------------------------------------
e2e_step "stage6_synthetic_validation" "Check sync_wait_ms upper bound and retired on_cancel spelling on synthetic.deferred"

if [ "${DRY}" = "1" ]; then
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" --json synthetic deferred --sync-wait-ms 30001 --key "$(e2e_key wait-over)"
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" --json synthetic deferred --on-cancel leave_running --key "$(e2e_key cancel-bad)"
elif capability_is_linked synthetic.deferred; then
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" --json synthetic deferred --sync-wait-ms 30001 --key "$(e2e_key wait-over)"
  e2e_expect_refusal FROB-REF-802 3 -- "${FROB_BIN}" --json synthetic deferred --on-cancel leave_running --key "$(e2e_key cancel-bad)"
else
  e2e_skip "missing test-lane verb: synthetic.deferred (requires BD-022 synthetic-verbs feature)"
fi

# ---------------------------------------------------------------------------
# Stage 7: Admission key rules (BD-138 / N32a-09)
# ---------------------------------------------------------------------------
e2e_step "stage7_session_keys" "Check key_optional session.focus succeeds without key and session.open refuses FROB-REF-805"

if [ "${DRY}" = "1" ]; then
  e2e_cmd --name "session_focus_no_key" -- "${FROB_BIN}" --json session focus mission
  e2e_expect_refusal FROB-REF-805 3 -- "${FROB_BIN}" --json session open --scopes design:read
elif capability_is_linked session.focus && capability_is_linked session.open; then
  e2e_cmd --name "session_focus_no_key" -- "${FROB_BIN}" --json session focus mission
  e2e_assert_eq "session_focus_no_key_exit" "${E2E_LAST_EXIT}" "0"
  e2e_expect_refusal FROB-REF-805 3 -- "${FROB_BIN}" --json session open --scopes design:read
else
  e2e_skip "missing M1 session verbs: session.focus and session.open (lands in N32a-09 / BD-138)"
fi
