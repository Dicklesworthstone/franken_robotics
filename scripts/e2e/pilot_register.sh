#!/usr/bin/env bash
# e2e: bead=N38a-09 milestone=M0 labels=gauntlet,pilot,gate gates=GATE-GAUNT-08 plants=unfunded-without-pilot,protocol-changed,receipt-uncommitted
#
# End-to-end verification of the Gauntlet pilot register (N38a-09, GATE-GAUNT-08):
#   1. In a scratch tree, register a pilot with a frozen protocol hash, commit its
#      receipt byte-identical at receipts/pilots/<hash>.json, append its result,
#      query fundable for the package and the pilot's own bead (pilot-bead), and
#      log status-count metrics.
#   2. Plant all three negative controls (unfunded upper-tier package with no pilot,
#      changed protocol after registration, uncommitted receipt) and require each to
#      be caught with its registered N38-REF reason.
#   3. Step `all_upper_tier_packages_at_m0`: run `fundable --all --json` against the
#      real register, assert every [F]/[M] work package at M0 is refused with a named
#      reason, and log per-package asserts plus fundable/partial/refused counts.
#   4. Step `partial_multi_mechanism_package`: build a scratch package with two
#      Frontier mechanisms (one falsifier-holds, one falsifier-rejected), assert
#      `Partial`, and verify per-bead funding (including Review Note 3 fallback bead).
#
# Convention 15: a pilot's script exits 0 when its measurement was made and its record
# written whichever status the record carries, and non-zero only when a step could not
# run, a sanity assertion failed, or a negative control was not caught.
#
# Idempotent; no device time; bash 3.2 compatible; never deletes any file or directory.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "pilot_register" "N38a-09"
e2e_gate "GATE-GAUNT-08"

_combined_output() {
  if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    cat "${E2E_LAST_STDOUT}"
  fi
  if [ -n "${E2E_LAST_STDERR:-}" ] && [ -f "${E2E_LAST_STDERR}" ]; then
    cat "${E2E_LAST_STDERR}"
  fi
}

_HASH_RESULT=""
_hash_file() {
  local label="$1"
  local target_file="$2"
  if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
    _HASH_RESULT="0000000000000000000000000000000000000000000000000000000000000000"
    return 0
  fi
  e2e_cmd --name "hash_${label}" -- \
    cargo run -q -p frob-conformance --example pilots -- hash "${target_file}"
  _HASH_RESULT=""
  if [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
    _HASH_RESULT="$(tr -d ' \t\r\n' < "${E2E_LAST_STDOUT}")"
  fi
}

_write_gates_toml() {
  local dst="$1"
  mkdir -p "${dst}/registries"
  cat > "${dst}/registries/gates.toml" <<'EOF'
schema = "frob.gates/1"

[[gate]]
id = "KILL-DEMO-01"
claim = "known-answer"
claim_id = "KILL-DEMO-01"
section = "38.5"
threshold = "pilot falsifier holds"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0003"]

[[gate]]
id = "KILL-PARTA-01"
claim = "known-answer"
claim_id = "KILL-PARTA-01"
section = "38.5"
threshold = "pilot falsifier holds"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0003"]

[[gate]]
id = "KILL-PARTB-01"
claim = "known-answer"
claim_id = "KILL-PARTB-01"
section = "38.5"
threshold = "pilot falsifier holds"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0003"]
EOF
}

_write_work_packages_toml() {
  local dst="$1"
  mkdir -p "${dst}/crates/frob-conformance/registry"
  cat > "${dst}/crates/frob-conformance/registry/work_packages.toml" <<'EOF'
schema = "frob.gauntlet.work_packages/1"

[[package]]
work_package = "WP-91"
milestone = "M0"
tier = ["F"]
mechanisms = ["demo-frontier-mech"]
beads = ["N91-01", "N91-02"]

[[package]]
work_package = "WP-92"
milestone = "M0"
tier = ["F"]
mechanisms = ["unpiloted-frontier-mech"]
beads = ["N92-01"]

[[package]]
work_package = "WP-93"
milestone = "M0"
tier = ["F"]
mechanisms = ["partial-mech-a", "partial-mech-b"]
beads = ["N93-01", "N93-02", "N93-10", "N93-11", "N93-99"]
EOF
}

_setup_demo_scratch() {
  local dst="$1"
  local include_receipt="$2"
  local mutate_protocol="$3"
  local omit_pilot="$4"

  mkdir -p "${dst}/crates/frob-conformance/registry" "${dst}/docs/pilots" "${dst}/receipts/pilots"
  if [ -f "${E2E_REPO_ROOT}/franken-stack.lock" ]; then
    cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  fi
  cp "${E2E_REPO_ROOT}/crates/frob-conformance/registry/pilots_expected.toml" \
    "${dst}/crates/frob-conformance/registry/pilots_expected.toml"
  _write_gates_toml "${dst}"
  _write_work_packages_toml "${dst}"

  local dst_tag
  dst_tag="$(basename "${dst}")"
  local proto_file="${dst}/docs/pilots/PILOT-DEMO-01.md"
  printf '# PILOT-DEMO-01 frozen protocol\nsampling_episodes_per_design = 16\n' > "${proto_file}"
  _hash_file "${dst_tag}_proto" "${proto_file}"
  local proto_hash="${_HASH_RESULT}"

  local receipt_staging="${dst}/docs/pilots/receipt_staging.json"
  printf '{"pilot":"PILOT-DEMO-01","verdict":"falsifier-rejected","ratio":"2.4"}\n' > "${receipt_staging}"
  _hash_file "${dst_tag}_receipt" "${receipt_staging}"
  local receipt_hash="${_HASH_RESULT}"

  if [ "${include_receipt}" = "1" ]; then
    cp "${receipt_staging}" "${dst}/receipts/pilots/${receipt_hash}.json"
  fi

  if [ "${omit_pilot}" = "1" ]; then
    cat > "${dst}/crates/frob-conformance/registry/pilots.toml" <<'EOF'
schema = "frob.gauntlet.pilot/1"
EOF
  else
    cat > "${dst}/crates/frob-conformance/registry/pilots.toml" <<EOF
schema = "frob.gauntlet.pilot/1"

[[pilot]]
id = "PILOT-DEMO-01"
mechanism = "demo-frontier-mech"
tier = "F"
owner = "§38.5"
bead = "N91-01"
work_package = "WP-91"
milestone = "M0"
gated_beads = ["N91-02"]
protocol = { path = "docs/pilots/PILOT-DEMO-01.md", blake3 = "${proto_hash}", frozen = "2026-11-24", ledger_seq = "10", sampling_episodes_per_design = 16 }
falsifier = "speedup ratio below 1.5 on the demo fixture"
ceiling = { statement = "upper bound from Amdahl floor", value = "2.5x", basis = "exact arithmetic model" }
fallback = { ships = "solid baseline solver", tier = "S", kill_gate = "KILL-DEMO-01" }
cost = { cpu_core_h = "1", gpu_h = "0", wall_h = "0.5", envelope = "default" }
tool = { name = "frob-demo-pilot", pin = "v1", site = "host" }
status = "falsifier-rejected"

[[pilot.result]]
date = "2026-11-25"
ledger_seq = "11"
protocol_blake3 = "${proto_hash}"
site = "host"
environment = "cpu-ref"
commands = ["pilot-demo run --seed 0"]
values = [{ name = "speedup", observed = "2.4", expected = ">= 1.5", kind = "ratio" }]
verdict = "falsifier-rejected"
limits = ["paper-scale fixture"]
receipt = "blake3:${receipt_hash}"
synthetic = true
EOF
  fi

  if [ "${mutate_protocol}" = "1" ]; then
    printf '# PILOT-DEMO-01 edited after registration\nsampling_episodes_per_design = 32\n' > "${proto_file}"
  fi
}

# ---------------------------------------------------------------------------
# Step 1: Register pilot, commit receipt, query funding & pilot-bead note
# ---------------------------------------------------------------------------
e2e_step "register_and_query_scratch" "Register a pilot in scratch tree, commit receipt, query funding and pilot-bead"

WS_HAPPY="${E2E_WORK_DIR}/ws_happy"
_plant_omit="0"
_plant_mutate="0"
_plant_receipt="1"
if e2e_plant "unfunded-without-pilot"; then
  _plant_omit="1"
fi
if e2e_plant "protocol-changed"; then
  _plant_mutate="1"
fi
if e2e_plant "receipt-uncommitted"; then
  _plant_receipt="0"
fi

_setup_demo_scratch "${WS_HAPPY}" "${_plant_receipt}" "${_plant_mutate}" "${_plant_omit}"

e2e_cmd --name scratch_check -- \
  cargo run -p frob-conformance --example pilots -- check --json --root "${WS_HAPPY}"
e2e_assert_eq "scratch_check_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"

if [ "${E2E_DRY_RUN:-0}" != "1" ] && [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _cnt_reg="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/registered" 2>/dev/null || echo 0)"
  _cnt_run="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/running" 2>/dev/null || echo 0)"
  _cnt_rej="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/falsifier_rejected" 2>/dev/null || echo 0)"
  _cnt_hld="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/falsifier_holds" 2>/dev/null || echo 0)"
  _cnt_void="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/void" 2>/dev/null || echo 0)"
  _cnt_tot="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/total" 2>/dev/null || echo 0)"
  e2e_metric "pilots_registered" "${_cnt_reg}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "pilots_running" "${_cnt_run}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "pilots_falsifier_rejected" "${_cnt_rej}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "pilots_falsifier_holds" "${_cnt_hld}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "pilots_void" "${_cnt_void}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "pilots_total" "${_cnt_tot}" "count" --observe --gate "GATE-GAUNT-08"
fi

e2e_cmd --name scratch_fundable_wp91 -- \
  cargo run -p frob-conformance --example pilots -- fundable --package WP-91 --require-fundable --json --root "${WS_HAPPY}"
e2e_assert_eq "scratch_fundable_wp91_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"
e2e_assert_json "scratch_fundable_wp91_status" "/status" "Fundable" --gate "GATE-GAUNT-08"

e2e_cmd --name scratch_fundable_pilot_bead -- \
  cargo run -p frob-conformance --example pilots -- fundable --bead N91-01 --json --root "${WS_HAPPY}"
e2e_assert_eq "scratch_pilot_bead_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"
e2e_assert_json "scratch_pilot_bead_fundable" "/fundable" "true" --gate "GATE-GAUNT-08"
e2e_assert_json "scratch_pilot_bead_note" "/note" "pilot-bead" --gate "GATE-GAUNT-08"

# ---------------------------------------------------------------------------
# Step 2: Three planted negative controls caught with registered reasons
# ---------------------------------------------------------------------------
e2e_step "negative_controls" "Catch unfunded package, changed protocol, and uncommitted receipt"

WS_UNFUNDED="${E2E_WORK_DIR}/ws_unfunded"
_setup_demo_scratch "${WS_UNFUNDED}" "1" "0" "0"
e2e_expect_caught "unfunded-without-pilot" --gate "GATE-GAUNT-08" -- \
  cargo run -p frob-conformance --example pilots -- fundable --package WP-92 --require-fundable --json --root "${WS_UNFUNDED}"
_nc1_ok="0"
if _combined_output | grep -Fq "reason=missing"; then
  _nc1_ok="1"
fi
e2e_assert_eq "nc_unfunded_reason_missing" "${_nc1_ok}" "1" --gate "GATE-GAUNT-08"

WS_PROTO="${E2E_WORK_DIR}/ws_proto_changed"
_setup_demo_scratch "${WS_PROTO}" "1" "1" "0"
e2e_expect_caught "protocol-changed" --gate "GATE-GAUNT-08" -- \
  cargo run -p frob-conformance --example pilots -- fundable --package WP-91 --require-fundable --json --root "${WS_PROTO}"
_nc2_ok="0"
if _combined_output | grep -Fq "reason=void"; then
  _nc2_ok="1"
fi
e2e_assert_eq "nc_protocol_changed_reason_void" "${_nc2_ok}" "1" --gate "GATE-GAUNT-08"

WS_PROTO_RES="${E2E_WORK_DIR}/ws_proto_result_mismatch"
_setup_demo_scratch "${WS_PROTO_RES}" "1" "0" "0"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  _other_proto="${WS_PROTO_RES}/docs/pilots/other.md"
  printf '# different protocol\n' > "${_other_proto}"
  _hash_file "other_proto" "${_other_proto}"
  _other_hash="${_HASH_RESULT}"
  _pilots_file="${WS_PROTO_RES}/crates/frob-conformance/registry/pilots.toml"
  _pilots_mut="${WS_PROTO_RES}/crates/frob-conformance/registry/pilots.mut.toml"
  awk -v h="${_other_hash}" '!done && /^protocol_blake3 = "[0-9a-f]+"/ { print "protocol_blake3 = \"" h "\""; done = 1; next } { print }' "${_pilots_file}" > "${_pilots_mut}"
  cp "${_pilots_mut}" "${_pilots_file}"
fi
e2e_expect_caught "protocol-changed" --gate "GATE-GAUNT-08" -- \
  cargo run -p frob-conformance --example pilots -- check --json --root "${WS_PROTO_RES}"
_nc2b_ok="0"
if _combined_output | grep -Fq "reason=result-protocol-mismatch"; then
  _nc2b_ok="1"
fi
e2e_assert_eq "nc_protocol_result_mismatch_reason" "${_nc2b_ok}" "1" --gate "GATE-GAUNT-08"

WS_RECEIPT="${E2E_WORK_DIR}/ws_no_receipt"
_setup_demo_scratch "${WS_RECEIPT}" "0" "0" "0"
e2e_expect_caught "receipt-uncommitted" --gate "GATE-GAUNT-08" -- \
  cargo run -p frob-conformance --example pilots -- check --json --root "${WS_RECEIPT}"
_nc3_ok="0"
if _combined_output | grep -Fq "reason=pilot-receipt-uncommitted"; then
  _nc3_ok="1"
fi
e2e_assert_eq "nc_receipt_uncommitted_reason" "${_nc3_ok}" "1" --gate "GATE-GAUNT-08"

# ---------------------------------------------------------------------------
# Step 3: all_upper_tier_packages_at_m0 (real register)
# ---------------------------------------------------------------------------
e2e_step "all_upper_tier_packages_at_m0" "Verify every [F]/[M] work package at M0 is refused with a named reason"

e2e_cmd --name real_fundable_all_m0 -- \
  cargo run -p frob-conformance --example pilots -- fundable --all --json --root "${E2E_REPO_ROOT}"
e2e_assert_eq "real_fundable_all_m0_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"

if [ "${E2E_DRY_RUN:-0}" != "1" ] && [ -n "${E2E_LAST_STDOUT:-}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _m0_fundable="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/counts/fundable" 2>/dev/null || echo 0)"
  _m0_partial="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/counts/partial" 2>/dev/null || echo 0)"
  _m0_refused="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/counts/refused" 2>/dev/null || echo 0)"
  _m0_total="$("${FROB_E2E_TOOL}" get --file "${E2E_LAST_STDOUT}" "/counts/total" 2>/dev/null || echo 0)"

  e2e_metric "m0_packages_fundable" "${_m0_fundable}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "m0_packages_partial" "${_m0_partial}" "count" --observe --gate "GATE-GAUNT-08"
  e2e_metric "m0_packages_refused" "${_m0_refused}" "count" --observe --gate "GATE-GAUNT-08"

  e2e_assert_num "m0_upper_tier_total_positive" "${_m0_total}" ">" "0" "packages" --gate "GATE-GAUNT-08"
  e2e_assert_eq "m0_packages_fundable_zero" "${_m0_fundable}" "0" --gate "GATE-GAUNT-08"
  e2e_assert_eq "m0_packages_partial_zero" "${_m0_partial}" "0" --gate "GATE-GAUNT-08"
  e2e_assert_eq "m0_packages_all_refused" "${_m0_refused}" "${_m0_total}" --gate "GATE-GAUNT-08"

  # Log one assert per upper-tier package naming its mechanisms, pilot ids, status, and refusal reason.
  _real_json="${E2E_LAST_STDOUT}"
  _i=0
  while [ "${_i}" -lt "${_m0_total}" ]; do
    _wp="$("${FROB_E2E_TOOL}" get --file "${_real_json}" "/packages/${_i}/work_package" 2>/dev/null || true)"
    _st="$("${FROB_E2E_TOOL}" get --file "${_real_json}" "/packages/${_i}/status" 2>/dev/null || true)"
    _rsns=""
    _pids=""
    _j=0
    while _mech="$("${FROB_E2E_TOOL}" get --file "${_real_json}" "/packages/${_i}/mechanisms/${_j}/mechanism" 2>/dev/null)"; do
      [ -n "${_mech}" ] || break
      _r="$("${FROB_E2E_TOOL}" get --file "${_real_json}" "/packages/${_i}/mechanisms/${_j}/reason" 2>/dev/null || true)"
      if [ -z "${_r}" ] || [ "${_r}" = "null" ]; then
        _r="none"
      fi
      if [ -n "${_rsns}" ]; then _rsns="${_rsns},${_r}"; else _rsns="${_r}"; fi
      _k=0
      while _pid="$("${FROB_E2E_TOOL}" get --file "${_real_json}" "/packages/${_i}/mechanisms/${_j}/pilot_ids/${_k}" 2>/dev/null)"; do
        [ -n "${_pid}" ] || break
        if [ -n "${_pids}" ]; then _pids="${_pids},${_pid}"; else _pids="${_pid}"; fi
        _k=$((_k+1))
      done
      _j=$((_j+1))
    done
    [ -n "${_rsns}" ] || _rsns="none"
    [ -n "${_pids}" ] || _pids="none"
    _ok="0"
    if [ "${_st}" = "Blocked" ] && [ "${_rsns}" != "none" ]; then
      _ok="1"
    fi
    e2e_assert_eq "m0_pkg_${_wp}_refused_named_reason" \
      "${_st}:${_rsns}:pilots=${_pids}:ok=${_ok}" \
      "Blocked:${_rsns}:pilots=${_pids}:ok=1" \
      --gate "GATE-GAUNT-08"
    _i=$((_i+1))
  done
fi

# ---------------------------------------------------------------------------
# Step 4: partial_multi_mechanism_package
# ---------------------------------------------------------------------------
e2e_step "partial_multi_mechanism_package" "Assert Partial verdict and per-bead funding on two-mechanism scratch package"

WS_PARTIAL="${E2E_WORK_DIR}/ws_partial"
mkdir -p "${WS_PARTIAL}/crates/frob-conformance/registry" "${WS_PARTIAL}/docs/pilots" "${WS_PARTIAL}/receipts/pilots"
if [ -f "${E2E_REPO_ROOT}/franken-stack.lock" ]; then
  cp "${E2E_REPO_ROOT}/franken-stack.lock" "${WS_PARTIAL}/franken-stack.lock"
fi
cp "${E2E_REPO_ROOT}/crates/frob-conformance/registry/pilots_expected.toml" \
  "${WS_PARTIAL}/crates/frob-conformance/registry/pilots_expected.toml"
_write_gates_toml "${WS_PARTIAL}"
_write_work_packages_toml "${WS_PARTIAL}"

_pa_proto="${WS_PARTIAL}/docs/pilots/PILOT-PARTA-01.md"
_pb_proto="${WS_PARTIAL}/docs/pilots/PILOT-PARTB-01.md"
printf '# PILOT-PARTA-01 protocol\n' > "${_pa_proto}"
printf '# PILOT-PARTB-01 protocol\n' > "${_pb_proto}"
_hash_file "parta_proto" "${_pa_proto}"
_pa_phash="${_HASH_RESULT}"
_hash_file "partb_proto" "${_pb_proto}"
_pb_phash="${_HASH_RESULT}"

_pa_rec="${WS_PARTIAL}/docs/pilots/parta_receipt.json"
_pb_rec="${WS_PARTIAL}/docs/pilots/partb_receipt.json"
printf '{"pilot":"PILOT-PARTA-01","verdict":"falsifier-holds"}\n' > "${_pa_rec}"
printf '{"pilot":"PILOT-PARTB-01","verdict":"falsifier-rejected"}\n' > "${_pb_rec}"
_hash_file "parta_receipt" "${_pa_rec}"
_pa_rhash="${_HASH_RESULT}"
_hash_file "partb_receipt" "${_pb_rec}"
_pb_rhash="${_HASH_RESULT}"
cp "${_pa_rec}" "${WS_PARTIAL}/receipts/pilots/${_pa_rhash}.json"
cp "${_pb_rec}" "${WS_PARTIAL}/receipts/pilots/${_pb_rhash}.json"

cat > "${WS_PARTIAL}/crates/frob-conformance/registry/pilots.toml" <<EOF
schema = "frob.gauntlet.pilot/1"

[[pilot]]
id = "PILOT-PARTA-01"
mechanism = "partial-mech-a"
tier = "F"
owner = "§38.5"
bead = "N93-10"
work_package = "WP-93"
milestone = "M0"
gated_beads = ["N93-01"]
protocol = { path = "docs/pilots/PILOT-PARTA-01.md", blake3 = "${_pa_phash}", frozen = "2026-11-24", ledger_seq = "20" }
falsifier = "gain below 1.5x on fixture A"
ceiling = { statement = "bounded gain A", value = "2.0x", basis = "analytic" }
fallback = { ships = "solid fallback A", tier = "S", kill_gate = "KILL-PARTA-01" }
cost = { cpu_core_h = "1", gpu_h = "0", wall_h = "0.5", envelope = "default" }
tool = { name = "pilot-a", pin = "v1", site = "host" }
status = "falsifier-holds"

[[pilot.result]]
date = "2026-11-25"
ledger_seq = "21"
protocol_blake3 = "${_pa_phash}"
site = "host"
environment = "cpu"
commands = ["pilot-a run"]
values = [{ name = "gain", observed = "1.1", expected = ">= 1.5", kind = "ratio" }]
verdict = "falsifier-holds"
limits = ["paper scale"]
receipt = "blake3:${_pa_rhash}"
synthetic = true

[[pilot]]
id = "PILOT-PARTB-01"
mechanism = "partial-mech-b"
tier = "F"
owner = "§38.5"
bead = "N93-11"
work_package = "WP-93"
milestone = "M0"
gated_beads = ["N93-02"]
protocol = { path = "docs/pilots/PILOT-PARTB-01.md", blake3 = "${_pb_phash}", frozen = "2026-11-24", ledger_seq = "22" }
falsifier = "gain below 1.5x on fixture B"
ceiling = { statement = "bounded gain B", value = "3.0x", basis = "analytic" }
fallback = { ships = "solid fallback B", tier = "S", kill_gate = "KILL-PARTB-01" }
cost = { cpu_core_h = "1", gpu_h = "0", wall_h = "0.5", envelope = "default" }
tool = { name = "pilot-b", pin = "v1", site = "host" }
status = "falsifier-rejected"

[[pilot.result]]
date = "2026-11-25"
ledger_seq = "23"
protocol_blake3 = "${_pb_phash}"
site = "host"
environment = "cpu"
commands = ["pilot-b run"]
values = [{ name = "gain", observed = "2.6", expected = ">= 1.5", kind = "ratio" }]
verdict = "falsifier-rejected"
limits = ["paper scale"]
receipt = "blake3:${_pb_rhash}"
synthetic = true
EOF

e2e_cmd --name partial_pkg_query -- \
  cargo run -p frob-conformance --example pilots -- fundable --package WP-93 --json --root "${WS_PARTIAL}"
e2e_assert_eq "partial_pkg_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"
e2e_assert_json "partial_pkg_status" "/status" "Partial" --gate "GATE-GAUNT-08"

e2e_cmd --name partial_bead_b_query -- \
  cargo run -p frob-conformance --example pilots -- fundable --bead N93-02 --json --root "${WS_PARTIAL}"
e2e_assert_eq "partial_bead_b_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"
e2e_assert_json "partial_bead_b_fundable" "/fundable" "true" --gate "GATE-GAUNT-08"

e2e_cmd --name partial_bead_a_query -- \
  cargo run -p frob-conformance --example pilots -- fundable --bead N93-01 --json --root "${WS_PARTIAL}"
e2e_assert_eq "partial_bead_a_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"
e2e_assert_json "partial_bead_a_blocked" "/fundable" "false" --gate "GATE-GAUNT-08"
e2e_assert_json "partial_bead_a_reason" "/reason" "falsifier-holds" --gate "GATE-GAUNT-08"

e2e_cmd --name partial_bead_fallback_query -- \
  cargo run -p frob-conformance --example pilots -- fundable --bead N93-99 --json --root "${WS_PARTIAL}"
e2e_assert_eq "partial_bead_fallback_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-08"
e2e_assert_json "partial_bead_fallback_fundable" "/fundable" "true" --gate "GATE-GAUNT-08"
