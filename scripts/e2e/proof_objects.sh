#!/usr/bin/env bash
# e2e: bead=N25b-01 milestone=M1 labels=augur,frob-core gates= seed=0 needs=cpu plants=torque-alias,empty-support
#
# End-to-end verification of frob-core plain proof objects (bead N25b-01):
#   1. Run unit tests (`cargo test -p frob-core --test proof_objects`).
#   2. Run `cargo run -p frob-core --example proof_specimens -- --json` and assert
#      the stance KA-624 specimens (kinds, four force vectors, dual (0, 1/2, 1/2),
#      cell ends (open, closed), and prefix).
#   3. Repeat step 2 under `--release` and assert byte-identical output and digests.
#   4. Run `--swap-ports` mode and assert the `lp-primal` encoding digest changes (CX-108).
#   5. Run the two negative-control feature builds (`nc-torque-alias-first-moment` and
#      `nc-allow-empty-support`) via `e2e_expect_caught`.
#
# Compatible with bash 3.2 (macOS) and Linux. Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "proof_objects" "N25b-01" "$@"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-proof-${E2E_RUN_ID}}"

caught_named_failure() {
  local status="$1" output="$2" name="$3" assertion="$4"
  [ "$status" = 101 ] && [ -f "$output" ] || return 1
  awk -v name="$name" -v assertion="$assertion" '
    /^running [0-9]+ tests?$/ { runs++; one = ($0 == "running 1 test") }
    /^test .* \.\.\. / { results++; failed = one && ($0 == "test " name " ... FAILED") }
    /^failures:$/ { named = 0 }
    /^---- .* stdout ----$/ { named = failed && ($0 == "---- " name " stdout ----") }
    {
      if (panic) {
        message = $0
        sub(/^assertion .* failed: /, "", message)
        if (index(message, assertion) == 1) matches++
      }
      header = named && /^thread \047.*\047.* panicked at .+:$/
      if (header) panics++
      panic = header && index($0, "thread \047" name "\047") == 1
    }
    /^test result:/ {
      summaries++
      matching_summary = (panics == 1 && matches == 1) && /^test result: FAILED\. 0 passed; 1 failed; 0 ignored; 0 measured;/
      named = 0
    }
    END { exit !(runs == 1 && results == 1 && summaries == 1 && matching_summary) }
  ' "$output"
}

run_mutant() {
  local plant="$1" feature="$2" name="$3" assertion="$4" caught=0
  e2e_cmd --name "$name" --expect-exit "0,1,101" -- \
    cargo test --offline -j 1 --manifest-path "$MANIFEST" -p frob-core \
      --features "$feature" --test proof_objects "$name" -- --exact
  if caught_named_failure "$E2E_LAST_EXIT" "$E2E_LAST_STDOUT" "$name" "$assertion"; then caught=1; fi
  if e2e_plant "$plant"; then
    e2e_assert_eq "${plant}:${name}" "$caught" "0"
  else
    e2e_expect_caught "$feature" -- test "$caught" = 0
  fi
}

torque_mutant() {
  run_mutant torque-alias nc-torque-alias-first-moment first_moment_is_not_a_torque \
    'FirstMoment and TorqueVec must be distinct Rust types'
}

support_mutant() {
  run_mutant empty-support nc-allow-empty-support support_must_not_be_empty \
    'non-partition proof object with empty Support must be refused'
}

# ---------------------------------------------------------------------------
# Step 1: Unit tests (with plant hooks for --verify-plants)
# ---------------------------------------------------------------------------
e2e_step "unit_tests" "Run frob-core proof_objects unit test suite"

if e2e_plant "torque-alias"; then
  torque_mutant
  exit
elif e2e_plant "empty-support"; then
  support_mutant
  exit
else
  e2e_cmd --name "proof_objects_unit" -- \
    cargo test --offline -j 1 --manifest-path "${MANIFEST}" -p frob-core --test proof_objects
  e2e_assert_eq "proof_objects_unit_exit" "${E2E_LAST_EXIT}" "0"
fi

# ---------------------------------------------------------------------------
# Step 2: Stance KA-624 specimens (debug build)
# ---------------------------------------------------------------------------
e2e_step "specimens_debug" "Run proof_specimens example in debug mode and assert KA-624 fields"

e2e_cmd --name "proof_specimens_debug" -- \
  cargo run --offline -j 1 --manifest-path "${MANIFEST}" -p frob-core --example proof_specimens -- --json
e2e_assert_eq "proof_specimens_debug_exit" "${E2E_LAST_EXIT}" "0" --ka "KA-624"

_DBG_OUT="${E2E_LAST_STDOUT}"
_DBG_PRIMAL_DIGEST=""

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "lp_primal_kind" "/lp_primal/kind" "lp-primal" --ka "KA-624"
  e2e_assert_json "lp_farkas_kind" "/lp_farkas/kind" "lp-farkas" --ka "KA-624"

  e2e_assert_json "lp_primal_v0_forces" "/lp_primal/forces/0" "160,130,110,0" --ka "KA-624"
  e2e_assert_json "lp_primal_v1_forces" "/lp_primal/forces/1" "130,160,0,110" --ka "KA-624"
  e2e_assert_json "lp_primal_v2_forces" "/lp_primal/forces/2" "110,0,160,130" --ka "KA-624"
  e2e_assert_json "lp_primal_v3_forces" "/lp_primal/forces/3" "0,110,130,160" --ka "KA-624"

  e2e_assert_json "lp_farkas_dual_y" "/lp_farkas/dual_y" "0,1/2,1/2" --ka "KA-624"
  e2e_assert_json "lp_farkas_cell_ends" "/lp_farkas/cell_ends" "open,closed" --ka "KA-624"

  e2e_assert_json "lp_primal_prefix_0" "/lp_primal/prefix/0" "exists:policy:per-nominal-design" --ka "KA-624"
  e2e_assert_json "lp_primal_prefix_1" "/lp_primal/prefix/1" "forall:world" --ka "KA-624"
  e2e_assert_json "lp_primal_prefix_2" "/lp_primal/prefix/2" "forall:execution" --ka "KA-624"

  _DBG_PRIMAL_DIGEST="$(e2e_get --file "${_DBG_OUT}" "/lp_primal/encoding_digest")"
  _DBG_FARKAS_DIGEST="$(e2e_get --file "${_DBG_OUT}" "/lp_farkas/encoding_digest")"
  _DBG_PRIMAL_EQ_DIGEST="$(e2e_get --file "${_DBG_OUT}" "/lp_primal/equation_digests/0")"
  _DBG_FARKAS_EQ_DIGEST="$(e2e_get --file "${_DBG_OUT}" "/lp_farkas/equation_digests/0")"

  e2e_assert_eq "lp_primal_digest_len" "${#_DBG_PRIMAL_DIGEST}" "64" --ka "KA-624"
  e2e_assert_eq "lp_farkas_digest_len" "${#_DBG_FARKAS_DIGEST}" "64" --ka "KA-624"
  e2e_assert_eq "lp_primal_eq_digest_len" "${#_DBG_PRIMAL_EQ_DIGEST}" "64" --ka "KA-624"
  e2e_assert_eq "lp_farkas_eq_digest_len" "${#_DBG_FARKAS_EQ_DIGEST}" "64" --ka "KA-624"
fi

# ---------------------------------------------------------------------------
# Step 3: Debug vs release determinism
# ---------------------------------------------------------------------------
e2e_step "specimens_release" "Run proof_specimens in release mode and assert identical bytes and digests"

e2e_cmd --name "proof_specimens_release" -- \
  cargo run --offline -j 1 --manifest-path "${MANIFEST}" -p frob-core --release --example proof_specimens -- --json
e2e_assert_eq "proof_specimens_release_exit" "${E2E_LAST_EXIT}" "0" --ka "KA-624"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  _REL_OUT="${E2E_LAST_STDOUT}"
  _DBG_SHA="$("${FROB_E2E_TOOL}" cmp sha256 "${_DBG_OUT}")"
  _REL_SHA="$("${FROB_E2E_TOOL}" cmp sha256 "${_REL_OUT}")"
  e2e_assert_digest_eq "debug_release_json_bytes" "${_REL_SHA}" "${_DBG_SHA}" --ka "KA-624"

  _REL_PRIMAL_DIGEST="$(e2e_get --file "${_REL_OUT}" "/lp_primal/encoding_digest")"
  _REL_FARKAS_DIGEST="$(e2e_get --file "${_REL_OUT}" "/lp_farkas/encoding_digest")"
  e2e_assert_digest_eq "debug_release_lp_primal_digest" "${_REL_PRIMAL_DIGEST}" "${_DBG_PRIMAL_DIGEST}" --ka "KA-624"
  e2e_assert_digest_eq "debug_release_lp_farkas_digest" "${_REL_FARKAS_DIGEST}" "${_DBG_FARKAS_DIGEST}" --ka "KA-624"
fi

# ---------------------------------------------------------------------------
# Step 4: Port-order sensitivity (--swap-ports, CX-108)
# ---------------------------------------------------------------------------
e2e_step "swap_ports" "Verify swapping ports changes lp-primal canonical encoding digest (CX-108)"

e2e_cmd --name "proof_specimens_swap_ports" -- \
  cargo run --offline -j 1 --manifest-path "${MANIFEST}" -p frob-core --example proof_specimens -- --json --swap-ports
e2e_assert_eq "proof_specimens_swap_ports_exit" "${E2E_LAST_EXIT}" "0" --cx "CX-108"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  _SWAPPED_DIGEST="$(e2e_get "/lp_primal/encoding_digest")"
  _CHANGED="0"
  if [ "${_SWAPPED_DIGEST}" != "${_DBG_PRIMAL_DIGEST}" ]; then
    _CHANGED="1"
  fi
  e2e_assert_eq "swap_ports_changes_digest" "${_CHANGED}" "1" --cx "CX-108"
fi

# ---------------------------------------------------------------------------
# Step 5: Negative-control builds (two mutant features)
# ---------------------------------------------------------------------------
e2e_step "negative_controls" "Verify mutant builds are caught by proof_objects tests"

torque_mutant
support_mutant

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  name=first_moment_is_not_a_torque
  assertion='FirstMoment and TorqueVec must be distinct Rust types'
  for bad in build-error unrelated-failure captured-marker multiple-runs expected-failure caught-then-terminal; do
    transcript="${E2E_WORK_DIR}/${bad}.txt"
    expected=0
    if [ "$bad" = expected-failure ]; then expected=1; fi
    {
      if [ "$bad" = build-error ]; then
        printf 'error: could not compile frob-core\n'
      else
        if [ "$bad" = multiple-runs ]; then
          printf '%s\n' 'running 1 test' "test $name ... ok" 'test result: ok. 1 passed; 0 failed;'
        fi
        printf '%s\n' 'running 1 test' "test $name ... FAILED" '' 'failures:' '' "---- $name stdout ----"
        if [ "$bad" = captured-marker ]; then printf 'checking: %s\n' "$assertion"; fi
        message='unrelated assertion'
        case "$bad" in
          expected-failure|multiple-runs|caught-then-terminal)
            message="assertion \`left != right\` failed: $assertion" ;;
        esac
        printf '%s\n' "thread '$name' panicked at proof_objects.rs:1:1:" "$message" ''
        if [ "$bad" = caught-then-terminal ]; then
          printf '%s\n' "thread '$name' panicked at proof_objects.rs:2:1:" 'unrelated terminal panic' ''
        fi
        printf '%s\n' 'failures:' "    $name" \
          'test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;'
      fi
    } > "$transcript"
    caught=0
    if caught_named_failure 101 "$transcript" "$name" "$assertion"; then caught=1; fi
    e2e_assert_eq "matcher_${bad}" "$caught" "$expected"
  done
fi
