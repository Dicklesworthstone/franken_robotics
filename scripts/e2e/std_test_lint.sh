#!/usr/bin/env bash
# e2e: bead=STD-TEST milestone=M0 labels=gauntlet,harness gates=GATE-GAUNT-01
#
# End-to-end verification of unit-test conventions lint (STD-TEST §9):
#   1. Plants each of the 8 lint violations one at a time in scratch copies of
#      the workspace and requires `std_test_lint` to catch each via
#      `e2e_expect_caught`.
#   2. Runs `std_test_lint` on the clean tree.
#   3. Loads the four registries of record via `frob-core` `registry_of_record_loads`
#      and logs row counts, obligation counts, and bound-test prefix counts as
#      `metric` lines.
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "std_test_lint" "STD-TEST"

# Copy workspace source layout into a fresh scratch directory without target/ or .git/.
_make_scratch_ws() {
  local label="$1"
  local dst="${E2E_WORK_DIR}/ws_${label}"
  mkdir -p "${dst}"
  cp "${E2E_REPO_ROOT}/Cargo.toml" "${dst}/Cargo.toml"
  if [ -f "${E2E_REPO_ROOT}/franken-stack.lock" ]; then
    cp "${E2E_REPO_ROOT}/franken-stack.lock" "${dst}/franken-stack.lock"
  fi
  local d
  for d in crates docs fixtures registries; do
    if [ -d "${E2E_REPO_ROOT}/${d}" ]; then
      cp -R "${E2E_REPO_ROOT}/${d}" "${dst}/${d}"
    fi
  done
  printf '%s' "${dst}"
}

_minimal_ka_toml() {
  local id="$1"
  local due="$2"
  cat <<EOF
schema = "frob.gauntlet.known-answers/1"

[[ka]]
id = "${id}"
section = "§20.16"
owner = "GATE-GAUNT-01"
fixture = "docking-grid"
same_as = ""
case = "synthetic docking grid count"
truth = { kind = "int", value = 344 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
control = { text = "planted wrong count", nc = ["NC-0412"] }
derivation = "docs/derivations/docking-fixture.md"
label = "DESIGN"
due = "${due}"
printed = ""
slug = ""
EOF
}

# ---------------------------------------------------------------------------
# Step 1: Plant each of the eight Section 9 violations in scratch copies
# ---------------------------------------------------------------------------
e2e_step "plant_eight_violations" "Plant all 8 STD-TEST §9 violations one at a time in scratch workspaces"

# Plant 1: forbidden mocking crate in a manifest
WS1="$(_make_scratch_ws "plant1_mock")"
mkdir -p "${WS1}/crates/planted-mock/src"
cat <<'EOF' > "${WS1}/crates/planted-mock/Cargo.toml"
[package]
name = "planted-mock"
version = "0.0.1"
edition = "2024"

[dev-dependencies]
mockall = "0.13"
EOF
printf '\n' > "${WS1}/crates/planted-mock/src/lib.rs"
e2e_expect_caught "forbidden-mock-crate" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS1}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 2: #[ignore] without a reason string
WS2="$(_make_scratch_ws "plant2_ignore")"
mkdir -p "${WS2}/crates/frob-core/tests"
cat <<'EOF' > "${WS2}/crates/frob-core/tests/planted_bare_ignore.rs"
#[test]
#[ignore]
fn planted_bare_ignore_test() {}
EOF
e2e_expect_caught "ignore-without-reason" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS2}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 3: ka_/cx_/nc_/mr_ test naming an identifier absent from its registry
WS3="$(_make_scratch_ws "plant3_unbound_id")"
mkdir -p "${WS3}/crates/frob-core/tests"
cat <<'EOF' > "${WS3}/crates/frob-core/tests/planted_unbound_id.rs"
#[test]
fn ka_999_planted_unknown_row() {}
EOF
e2e_expect_caught "unbound-registry-test-id" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS3}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 4: registry row due at or before current milestone with no bound test or runner entry
WS4="$(_make_scratch_ws "plant4_due_row")"
mkdir -p "${WS4}/crates/frob-conformance/registry"
if [ -f "${WS4}/crates/frob-conformance/registry/known_answers.toml" ]; then
  _minimal_ka_toml "KA-9999" "M0" | tail -n +2 >> "${WS4}/crates/frob-conformance/registry/known_answers.toml"
else
  _minimal_ka_toml "KA-9999" "M0" > "${WS4}/crates/frob-conformance/registry/known_answers.toml"
fi
e2e_expect_caught "due-row-without-test" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS4}" FROB_STD_TEST_LINT_MILESTONE="M0" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 5: fixture file lacking its kind header
WS5="$(_make_scratch_ws "plant5_fixture_header")"
mkdir -p "${WS5}/fixtures"
printf 'value = 42\n' > "${WS5}/fixtures/planted_no_header.toml"
e2e_expect_caught "fixture-missing-kind-header" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS5}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 6: gate identifier printed with `pass` outside GateReport
WS6="$(_make_scratch_ws "plant6_gate_pass")"
mkdir -p "${WS6}/crates/frob-core/tests"
cat <<'EOF' > "${WS6}/crates/frob-core/tests/planted_gate_pass.rs"
#[test]
fn planted_prints_gate_pass() {
    println!("GATE-GAUNT-01: pass");
}
EOF
e2e_expect_caught "gate-pass-outside-gate-report" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS6}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 7: two physical copies of a registry
WS7="$(_make_scratch_ws "plant7_dup_registry")"
mkdir -p "${WS7}/crates/frob-conformance/registry" "${WS7}/registries"
printf 'schema = "frob.gauntlet.known-answers/1"\n' > "${WS7}/crates/frob-conformance/registry/known_answers.toml"
printf 'schema = "frob.gauntlet.known-answers/1"\n' > "${WS7}/registries/known_answers.toml"
e2e_expect_caught "duplicate-registry-copy" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS7}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# Plant 8: evidence line with kind=gate status=pass without a caught control line
WS8="$(_make_scratch_ws "plant8_evidence_pass")"
mkdir -p "${WS8}/target/test-evidence"
printf '{"schema":"frob.testevidence/1","ts":"2026-11-24T00:00:00.000Z","crate":"frob-core","test":"planted","kind":"gate","id":"GATE-GAUNT-01","status":"pass","reason":null,"observed":"0","expected":"0","tolerance":null,"unit":null,"tier":"G0","site":"host","seed":0,"determinism":"D0","synthetic":true}\n' \
  > "${WS8}/target/test-evidence/planted.ndjson"
e2e_expect_caught "gate-pass-evidence-without-caught" --gate "GATE-GAUNT-01" -- \
  env FROB_STD_TEST_LINT_ROOT="${WS8}" \
  cargo test -p frob-conformance --test std_test_lint -- lint_workspace_clean

# ---------------------------------------------------------------------------
# Step 2: Run std_test_lint on the clean workspace tree
# ---------------------------------------------------------------------------
e2e_step "clean_tree_lint" "Run std_test_lint on the clean repository tree"
e2e_cmd "clean_std_test_lint" -- cargo test -p frob-conformance --test std_test_lint
e2e_assert_eq "clean_std_test_lint_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

# ---------------------------------------------------------------------------
# Step 3: Load the four registries of record and emit row / obligation metrics
# ---------------------------------------------------------------------------
e2e_step "registries_of_record" "Load the four registries of record and record counts"
e2e_cmd "registry_of_record_loads" -- \
  cargo test -p frob-core --features test-support --test testkit -- registry_of_record_loads --nocapture
e2e_assert_eq "registry_of_record_loads_exit" "${E2E_LAST_EXIT}" "0" --gate "GATE-GAUNT-01"

if [ -n "${E2E_LAST_STDOUT}" ] && [ -f "${E2E_LAST_STDOUT}" ]; then
  _reg_kv="$(python3 - "${E2E_LAST_STDOUT}" <<'PY'
import re, sys

text = open(sys.argv[1], "r", encoding="utf-8", errors="replace").read()
vals = {
    "ka_rows": 0,
    "ka_obl_tol": 0,
    "ka_obl_ctrl": 0,
    "ctrl_rows": 0,
    "ctrl_obl": 0,
    "rel_rows": 0,
    "cx_rows": 0,
}
m = re.search(r"registry:known_answers\s+rows=(\d+)\s+obligation_tolerance=(\d+)\s+obligation_control=(\d+)", text)
if m:
    vals["ka_rows"], vals["ka_obl_tol"], vals["ka_obl_ctrl"] = int(m.group(1)), int(m.group(2)), int(m.group(3))
m = re.search(r"registry:controls\s+rows=(\d+)\s+obligation=(\d+)", text)
if m:
    vals["ctrl_rows"], vals["ctrl_obl"] = int(m.group(1)), int(m.group(2))
m = re.search(r"registry:relations\s+rows=(\d+)", text)
if m:
    vals["rel_rows"] = int(m.group(1))
m = re.search(r"registry:counterexamples\s+rows=(\d+)", text)
if m:
    vals["cx_rows"] = int(m.group(1))
for k, v in vals.items():
    print(f"{k}={v}")
PY
  )"
  while IFS='=' read -r _k _v; do
    case "${_k}" in
      ka_rows)     e2e_metric "registry_known_answers_rows" "${_v}" "rows" ;;
      ka_obl_tol)  e2e_metric "registry_ka_obligation_tolerance" "${_v}" "rows" ;;
      ka_obl_ctrl) e2e_metric "registry_ka_obligation_control" "${_v}" "rows" ;;
      ctrl_rows)   e2e_metric "registry_controls_rows" "${_v}" "rows" ;;
      ctrl_obl)    e2e_metric "registry_controls_obligation" "${_v}" "rows" ;;
      rel_rows)    e2e_metric "registry_relations_rows" "${_v}" "rows" ;;
      cx_rows)     e2e_metric "registry_counterexamples_rows" "${_v}" "rows" ;;
    esac
  done <<< "${_reg_kv}"
fi

# ---------------------------------------------------------------------------
# Step 4: Count bound tests per prefix across the workspace
# ---------------------------------------------------------------------------
e2e_step "bound_test_counts" "Count bound tests per STD-TEST §2 prefix across crates/"
_prefix_kv="$(python3 - "${E2E_REPO_ROOT}/crates" <<'PY'
import os, re, sys

root = sys.argv[1]
patterns = [
    ("ka", re.compile(r"\bfn\s+ka_[0-9]+_[A-Za-z0-9_]*\s*\(")),
    ("cx", re.compile(r"\bfn\s+cx_[0-9]+_[A-Za-z0-9_]*\s*\(")),
    ("nc", re.compile(r"\bfn\s+nc_[0-9]+_[A-Za-z0-9_]*\s*\(")),
    ("mr", re.compile(r"\bfn\s+mr_c?[0-9]+_[A-Za-z0-9_]*\s*\(")),
    ("inv", re.compile(r"\bfn\s+inv_[a-z0-9]+_[0-9]+_[A-Za-z0-9_]*\s*\(")),
    ("po", re.compile(r"\bfn\s+po_[a-z0-9]+_[0-9]+_[A-Za-z0-9_]*\s*\(")),
    ("law", re.compile(r"\bfn\s+law_[A-Za-z0-9_]+\s*\(")),
    ("det", re.compile(r"\bfn\s+det_(?:twice|workers|profiles|platforms|backends)_[A-Za-z0-9_]+\s*\(")),
    ("refusal", re.compile(r"\bfn\s+refusal_[0-9]+_[A-Za-z0-9_]*\s*\(")),
]
counts = {name: 0 for name, _ in patterns}
for dirpath, dirnames, filenames in os.walk(root):
    dirnames[:] = sorted(d for d in dirnames if d not in ("target", ".git"))
    for fname in sorted(filenames):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(dirpath, fname)
        try:
            text = open(fpath, "r", encoding="utf-8", errors="replace").read()
        except OSError:
            continue
        for name, pat in patterns:
            counts[name] += len(pat.findall(text))
for name, _ in patterns:
    print(f"{name}={counts[name]}")
PY
)"
while IFS='=' read -r _p _c; do
  if [ -n "${_p}" ]; then
    e2e_metric "bound_tests_${_p}" "${_c}" "tests"
  fi
done <<< "${_prefix_kv}"
