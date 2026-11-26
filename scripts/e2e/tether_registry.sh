#!/usr/bin/env bash
# e2e: bead=N32a-02 milestone=M0 labels=m0,tether,frob-verbs,solid,registry,verb gates=GATE-TETHER-02 seed=0 timeout_s=900 needs=cpu
#
# End-to-end verification of frob-verbs registry build script, deterministic codegen,
# registry-only feature boundary, unit lint suite, and five planted FROB_VERBS_REGISTRY
# negative controls (bead N32a-02; generation half of GATE-TETHER-02).
#
# Logs through STD-E2E (scripts/e2e/lib.sh) to target/e2e-logs/tether_registry/<utc>.ndjson.
# Never deletes any file or directory (COMMON.md 9b).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "tether_registry" "N32a-02" "$@"

TARGET_BASE="${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-tether-registry-${E2E_RUN_ID}"
mkdir -p "${TARGET_BASE}"

# ---------------------------------------------------------------------------
# Step 1: Build frob-verbs in both feature sets (registry-only and default)
# ---------------------------------------------------------------------------
e2e_step "feature_builds" "Build frob-verbs under registry-only and default features and check dependency boundary"

TD_REGONLY="${TARGET_BASE}/registry-only"
TD_DEFAULT_1="${TARGET_BASE}/default-run1"
TD_DEFAULT_2="${TARGET_BASE}/default-run2"
mkdir -p "${TD_REGONLY}" "${TD_DEFAULT_1}" "${TD_DEFAULT_2}"

e2e_cmd --name "build_registry_only" -- \
  env "CARGO_TARGET_DIR=${TD_REGONLY}" \
  cargo build -p frob-verbs --no-default-features --features registry-only --locked
e2e_assert_eq "build_registry_only_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "tree_registry_only" -- \
  cargo tree -p frob-verbs --no-default-features --features registry-only -e normal --locked --offline
e2e_assert_eq "tree_registry_only_exit" "${E2E_LAST_EXIT}" "0"
physics_edges() {
  awk '/(^|[[:space:]])frob-fsim([[:space:]]|$)/ { n++ } END { print n+0 }' "$1"
}
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_eq "registry_only_no_frob_fsim" "$(physics_edges "${E2E_LAST_STDOUT}")" "0"
fi
printf '%s\n' 'frob-verbs v0.0.1' '└── frob-fsim v0.0.1' > "${E2E_SCRATCH_DIR}/physics-tree.txt"
e2e_assert_eq "planted_frob_fsim_edge_detected" "$(physics_edges "${E2E_SCRATCH_DIR}/physics-tree.txt")" "1"

e2e_cmd --name "build_default_run1" -- \
  env "CARGO_TARGET_DIR=${TD_DEFAULT_1}" \
  cargo run -p frob-verbs --locked --example registry_table
e2e_assert_eq "build_default_run1_exit" "${E2E_LAST_EXIT}" "0"
GEN_1="${E2E_LAST_STDOUT}"

e2e_cmd --name "build_default_run2" -- \
  env "CARGO_TARGET_DIR=${TD_DEFAULT_2}" \
  cargo run -p frob-verbs --locked --example registry_table
e2e_assert_eq "build_default_run2_exit" "${E2E_LAST_EXIT}" "0"
GEN_2="${E2E_LAST_STDOUT}"

# ---------------------------------------------------------------------------
# Step 2: Byte-identical verbs_table.rs across two independent builds (D0)
# ---------------------------------------------------------------------------
e2e_step "determinism_check" "Compare generated verbs_table.rs byte-for-byte across two builds"

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  if cmp -s "${GEN_1}" "${GEN_2}"; then
    e2e_assert_eq "verbs_table_byte_identical" "equal" "equal"
  else
    e2e_assert_eq "verbs_table_byte_identical" "differs" "equal"
  fi
  registry_stat() {
    awk -v name="$1:" '$1 == "pub" && $2 == "const" && $3 == name { sub(/;$/, "", $6); print $6 }' "${GEN_1}"
  }
  DIGEST_HEX="$(grep '^pub const REGISTRY_DIGEST_HEX:' "${GEN_1}" | sed -E 's/.*"([0-9a-f]{64})".*/\1/')"
  ENTRY_COUNT="$(grep '^pub static VERBS:' "${GEN_1}" | sed -E 's/.*\[VerbEntry; ([0-9]+)\].*/\1/')"
  e2e_metric "entry_count" "${ENTRY_COUNT}" "count"
  e2e_metric "cluster_count" "$(registry_stat REGISTRY_CLUSTER_COUNT)" "count"
  e2e_metric "largest_task_cluster" "$(registry_stat REGISTRY_LARGEST_TASK_CLUSTER)" "count" "<=" "7"
  e2e_metric "longest_description" "$(registry_stat REGISTRY_LONGEST_DESCRIPTION)" "count" "<=" "800"
  e2e_assert_eq "registry_digest_len" "${#DIGEST_HEX}" "64"
fi

# ---------------------------------------------------------------------------
# Step 3: Run unit lint suite and conformance tests
# ---------------------------------------------------------------------------
e2e_step "unit_and_conformance_tests" "Run frob-verbs registry unit lints and crate conformance checks"

e2e_cmd --name "cargo_test_registry" -- \
  env "CARGO_TARGET_DIR=${TD_DEFAULT_1}" \
  cargo test -p frob-verbs --locked -- registry
e2e_assert_eq "cargo_test_registry_exit" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "cargo_test_conformance" -- \
  env "CARGO_TARGET_DIR=${TD_DEFAULT_1}" \
  cargo test -p frob-verbs --test conformance --locked
e2e_assert_eq "cargo_test_conformance_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 4: Plant five scratch registry violations via FROB_VERBS_REGISTRY
# ---------------------------------------------------------------------------
e2e_step "planted_negative_controls" "Verify build.rs fails closed with named lints on five planted registries"

PLANTS_DIR="${E2E_SCRATCH_DIR}/plants"
SCHEMAS_SCRATCH="${PLANTS_DIR}/schemas"
mkdir -p "${PLANTS_DIR}" "${SCHEMAS_SCRATCH}"

# Minimal scratch schemas referenced by planted registries.
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.robot-docs.in.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.robot-docs.out.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.robot.docs.in.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.robot.docs.out.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.doctor.in.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.doctor.out.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.sim.run.in.1.json"
printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.sim.run.out.1.json"
for idx in 1 2 3 4 5 6 7 8; do
  printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.design.op${idx}.in.1.json"
  printf '{"type":"object","properties":{}}\n' > "${SCHEMAS_SCRATCH}/frob.design.op${idx}.out.1.json"
done

# Plant 1: collision (RL-02 tool-name-collision: robot-docs vs robot.docs)
cat > "${PLANTS_DIR}/collision.toml" <<'EOF'
schema = "frob.verbs/1"

[[verb]]
name = "robot-docs"
cluster = "operator"
kind = "read"
tier = "S"
capabilities = []
public = true
input_schema = "frob.robot-docs.in/1"
output_schema = "frob.robot-docs.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 10 ms"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob robot-docs --json"]
purpose = "Print agent guide."
use_when = "Starting a session."
owner = "frob-verbs"

[[verb]]
name = "robot.docs"
cluster = "robot"
kind = "read"
tier = "S"
capabilities = ["robot:read"]
input_schema = "frob.robot.docs.in/1"
output_schema = "frob.robot.docs.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 10 ms"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob robot docs --json"]
purpose = "Colliding robot.docs verb."
use_when = "Negative control."
owner = "frob-verbs"
EOF

# Plant 2: unknown-field (RL-13)
cat > "${PLANTS_DIR}/unknown_field.toml" <<'EOF'
schema = "frob.verbs/1"

[[verb]]
name = "doctor"
cluster = "operator"
kind = "read"
tier = "S"
capabilities = []
public = true
input_schema = "frob.doctor.in/1"
output_schema = "frob.doctor.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob doctor --json"]
purpose = "Health check."
use_when = "Negative control."
owner = "frob-verbs"
unknown_typo_key = true
EOF

# Plant 3: underscore name grammar (RL-01 name-grammar)
cat > "${PLANTS_DIR}/underscore.toml" <<'EOF'
schema = "frob.verbs/1"

[[verb]]
name = "design.check_all"
cluster = "design"
kind = "read"
tier = "S"
capabilities = ["design:read"]
input_schema = "frob.doctor.in/1"
output_schema = "frob.doctor.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M0"
task_clusters = ["author"]
examples = ["frob design check_all --json"]
purpose = "Underscore negative control."
use_when = "Negative control."
owner = "frob-verbs"
EOF

# Plant 4: eight tools in one task cluster (RL-04 empty-task-clusters)
{
  printf 'schema = "frob.verbs/1"\n\n'
  for idx in 1 2 3 4 5 6 7 8; do
    cat <<EOF
[[verb]]
name = "design.op${idx}"
cluster = "design"
kind = "read"
tier = "S"
capabilities = ["design:read"]
input_schema = "frob.design.op${idx}.in/1"
output_schema = "frob.design.op${idx}.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 10 ms"
first_milestone = "M0"
task_clusters = ["author"]
examples = ["frob design op${idx} --json"]
purpose = "Synthetic cluster tool ${idx}."
use_when = "Negative control."
owner = "frob-verbs"

EOF
  done
} > "${PLANTS_DIR}/eight_tools.toml"

# Plant 5: wait out of range (RL-09 mcp-wait-out-of-range)
cat > "${PLANTS_DIR}/wait_range.toml" <<'EOF'
schema = "frob.verbs/1"

[[verb]]
name = "sim.run"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.run.in/1"
output_schema = "frob.sim.run.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "sim_estimator"
first_milestone = "M0"
task_clusters = ["simulate"]
examples = ["frob sim run --key k1 --json"]
purpose = "Run simulation."
use_when = "Negative control."
owner = "frob-arena"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 60000 }
EOF

run_plant() {
  local plant_name="$1"
  local plant_file="$2"
  local expect_row="$3" expect_lint="$4" expect_slug="$5" diagnostics
  local td="${TARGET_BASE}/plant-${plant_name}"
  mkdir -p "${td}"

  e2e_cmd --name "plant_${plant_name}" --expect-exit "101" -- \
    env \
      "CARGO_TARGET_DIR=${td}" \
      "FROB_VERBS_REGISTRY=${plant_file}" \
      "FROB_VERBS_SCHEMAS_DIR=${SCHEMAS_SCRATCH}" \
      cargo check -p frob-verbs --locked

  [ "${E2E_DRY_RUN:-0}" != "1" ] || return 0
  diagnostics="$(awk -v expected="entry \`${expect_row}\` failed ${expect_lint} (${expect_slug})" \
    'index($0, expected) { n++ } END { print n+0 }' "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}")"
  e2e_assert_eq "plant_${plant_name}_exit" "${E2E_LAST_EXIT}" "101"
  e2e_assert_eq "plant_${plant_name}_diagnostic_count" "${diagnostics}" "1"
}

run_plant "collision" "${PLANTS_DIR}/collision.toml" "robot.docs" "RL-02" "tool-name-collision"
run_plant "unknown_field" "${PLANTS_DIR}/unknown_field.toml" "doctor" "RL-13" "unknown-field"
run_plant "underscore" "${PLANTS_DIR}/underscore.toml" "design.check_all" "RL-01" "name-grammar"
run_plant "eight_tools" "${PLANTS_DIR}/eight_tools.toml" "design.op8" "RL-04" "empty-task-clusters"
run_plant "wait_range" "${PLANTS_DIR}/wait_range.toml" "sim.run" "RL-09" "mcp-wait-out-of-range"

e2e_skip "plant_bare_int" "RL-INT integer-type lint activates at M1 when N32a-07 lands"
