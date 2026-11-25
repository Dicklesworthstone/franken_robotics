#!/usr/bin/env bash
# e2e: bead=N12-03 milestone=M0 labels=lemma,frob-cert seed=0 needs=cpu
#
# End-to-end verification script for N12-03 (`frob-cert` bundle schema frob.cert/1,
# generic rejection rules 1-6, format rules R1-R5, and Merkle root).
#
# Drives the standalone `frob-verify` binary under `env -i` against all golden
# bundles, every generic mutant in tests/golden/mutants/expected.toml, the
# six-entry Appendix L.7 listing (recomputed Merkle root, Tested class, and
# single-entry modification hash.merkle-root control), and the rule-order control.
#
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "lemma_bundle_schema" "N12-03" "$@"

GOLDEN_DIR="${E2E_REPO_ROOT}/crates/frob-cert/tests/golden"
SIX_DIR="${GOLDEN_DIR}/six_entry"
MUTANTS_DIR="${GOLDEN_DIR}/mutants"
EXPECTED_TOML="${MUTANTS_DIR}/expected.toml"

# ---------------------------------------------------------------------------
# Step 1: Build standalone frob-verify in release mode through the cargo shim
# ---------------------------------------------------------------------------
e2e_step "build_frob_verify" "Build frob-verify release binary via cargo shim"

TARGET_DIR="${CARGO_TARGET_DIR:-${E2E_REPO_ROOT}/target}"
e2e_cmd --name "cargo_build_frob_verify_release" --expect-exit "0" -- \
  cargo build --manifest-path "${E2E_REPO_ROOT}/Cargo.toml" -p frob-cert --release --bin frob-verify

if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
  printf '[dry-run] env -i %s/release/frob-verify --json <goldens, mutants, six-entry set>\n' "${TARGET_DIR}"
  exit 0
fi

VERIFY_BIN="${TARGET_DIR}/release/frob-verify"
if [ ! -x "${VERIFY_BIN}" ] && [ -x "${E2E_REPO_ROOT}/target/release/frob-verify" ]; then
  VERIFY_BIN="${E2E_REPO_ROOT}/target/release/frob-verify"
fi
e2e_assert_file "frob_verify_binary_exists" "${VERIFY_BIN}"

# ---------------------------------------------------------------------------
# Step 2: Golden bundles under env -i (expected: unsupported at M0, never reject,
#         and byte-identical --reserialize roundtrip)
# ---------------------------------------------------------------------------
GOLDEN_LIST=(
  "${GOLDEN_DIR}/stance_inclusion.frobcert"
  "${GOLDEN_DIR}/docking_open_threshold.frobcert"
  "${SIX_DIR}/e1_inclusion_c03.frobcert"
  "${SIX_DIR}/e2_inclusion_c06a.frobcert"
  "${SIX_DIR}/e3_exclusion_c01_c04.frobcert"
  "${SIX_DIR}/e4_exclusion_c07_c12.frobcert"
  "${SIX_DIR}/e5_partition_box.frobcert"
  "${SIX_DIR}/e6_qualification_receipt.frobcert"
)

golden_idx=0
for gfile in "${GOLDEN_LIST[@]}"; do
  golden_idx=$((golden_idx + 1))
  gbase="$(basename "${gfile}" .frobcert)"
  e2e_step "generic/golden/${gbase}" "Audit golden bundle ${gbase} under env -i"
  e2e_assert_file "golden_exists_${gbase}" "${gfile}"

  reser_out="${E2E_WORK_DIR}/reser_${golden_idx}_${gbase}.frobcert"
  e2e_cmd --name "verify_golden_${gbase}" --expect-exit "2" -- \
    env -i "${VERIFY_BIN}" --json --reserialize "${reser_out}" "${gfile}"
  e2e_assert_eq "golden_exit_${gbase}" "${E2E_LAST_EXIT}" "2"
  e2e_assert_json "golden_overall_${gbase}" "/overall" "unsupported"
  e2e_assert_json "golden_verdict_${gbase}" "/reports/0/verdict" "unsupported"

  if cmp -s "${gfile}" "${reser_out}"; then
    e2e_assert_eq "golden_reserialize_${gbase}" "byte-identical" "byte-identical"
  else
    e2e_assert_eq "golden_reserialize_${gbase}" "differs" "byte-identical"
  fi
done

# ---------------------------------------------------------------------------
# Step 3: Generic mutants under env -i against expected.toml
# ---------------------------------------------------------------------------
e2e_assert_file "mutants_expected_toml" "${EXPECTED_TOML}"

MUTANT_ROWS="$(python3 - "${EXPECTED_TOML}" "${GOLDEN_DIR}" <<'PY'
import os, sys, tomllib
manifest = tomllib.load(open(sys.argv[1], "rb"))
golden_dir = sys.argv[2]
for m in manifest.get("mutant", []):
    f = m["file"]
    base = m["base"]
    rule = m["rule"]
    ref = m.get("refusal", "")
    path = m.get("path", "")
    base_path = os.path.join(golden_dir, base)
    print(f"{f}\t{base_path}\t{rule}\t{ref}\t{path}")
PY
)"

MUTATIONS_RUN=0
MUTATIONS_MATCHED=0

while IFS= read -r row; do
  [ -n "${row}" ] || continue
  mfile="$(printf '%s\n' "${row}" | cut -f1)"
  bpath="$(printf '%s\n' "${row}" | cut -f2)"
  exp_rule="$(printf '%s\n' "${row}" | cut -f3)"
  exp_refusal="$(printf '%s\n' "${row}" | cut -f4)"
  exp_path="$(printf '%s\n' "${row}" | cut -f5)"
  mpath="${MUTANTS_DIR}/${mfile}"
  mspec="$(basename "${mfile}" .frobcert)"

  e2e_step "generic/${exp_rule}/${mspec}" "Verify mutant ${mspec} triggers ${exp_rule}"
  MUTATIONS_RUN=$((MUTATIONS_RUN + 1))

  base_b3="$( ( env -i "${VERIFY_BIN}" --json "${bpath}" 2>/dev/null || true ) | python3 -c 'import json,sys; print(json.load(sys.stdin)["reports"][0]["blake3"])' 2>/dev/null || echo "unknown" )"
  e2e_cmd --name "verify_mutant_${mspec}" --expect-exit "3" -- \
    env -i "${VERIFY_BIN}" --json "${mpath}"

  obs_exit="${E2E_LAST_EXIT}"
  obs_verdict="$(e2e_get /reports/0/verdict 2>/dev/null || echo "<missing>")"
  obs_rule="$(e2e_get /reports/0/rule 2>/dev/null || echo "<missing>")"
  obs_refusal="$(e2e_get /reports/0/refusal 2>/dev/null || echo "")"
  obs_path="$(e2e_get /reports/0/path 2>/dev/null || echo "<missing>")"
  mut_b3="$(e2e_get /reports/0/blake3 2>/dev/null || echo "<missing>")"

  e2e_assert_eq "mutant_exit_${mspec}" "${obs_exit}" "3"
  e2e_assert_eq "mutant_verdict_${mspec}" "${obs_verdict}" "reject"
  e2e_assert_eq "mutant_rule_${mspec}" "${obs_rule}" "${exp_rule}"
  e2e_assert_eq "mutant_refusal_${mspec}" "${obs_refusal}" "${exp_refusal}"
  e2e_assert_eq "mutant_path_${mspec}" "${obs_path}" "${exp_path}"
  if [ "${base_b3}" != "${mut_b3}" ]; then
    e2e_assert_eq "mutant_digest_changed_${mspec}" "changed" "changed"
  else
    e2e_assert_eq "mutant_digest_changed_${mspec}" "${mut_b3}" "distinct-from-${base_b3}"
  fi

  if [ "${obs_exit}" = "3" ] && [ "${obs_verdict}" = "reject" ] && [ "${obs_rule}" = "${exp_rule}" ] && [ "${obs_refusal}" = "${exp_refusal}" ]; then
    MUTATIONS_MATCHED=$((MUTATIONS_MATCHED + 1))
  fi
done <<EOF
${MUTANT_ROWS}
EOF

e2e_metric "mutations_run" "${MUTATIONS_RUN}" "count" ">=" "20"
e2e_metric "mutations_rejected_expected" "${MUTATIONS_MATCHED}" "count" "==" "${MUTATIONS_RUN}"

# ---------------------------------------------------------------------------
# Step 4: Six-entry listing as a set (Merkle root, Tested class, and mutation)
# ---------------------------------------------------------------------------
e2e_step "generic/hash.merkle-root/six_entry_set" "Audit Appendix L.7 six-entry set: Merkle root, Tested class, and single-entry mutation"

SIX_FILES=(
  "${SIX_DIR}/e1_inclusion_c03.frobcert"
  "${SIX_DIR}/e2_inclusion_c06a.frobcert"
  "${SIX_DIR}/e3_exclusion_c01_c04.frobcert"
  "${SIX_DIR}/e4_exclusion_c07_c12.frobcert"
  "${SIX_DIR}/e5_partition_box.frobcert"
  "${SIX_DIR}/e6_qualification_receipt.frobcert"
)

e2e_cmd --name "verify_six_entry_set" --expect-exit "2" -- \
  env -i "${VERIFY_BIN}" --json "${SIX_FILES[@]}"

e2e_assert_eq "six_entry_set_exit" "${E2E_LAST_EXIT}" "2"
e2e_assert_json "six_entry_set_overall" "/overall" "unsupported"
e2e_assert_json "six_entry_set_recomputed" "/set/recomputed" "true"
e2e_assert_json "six_entry_set_class" "/set/class" "Tested"
ORIG_MERKLE_ROOT="$(e2e_get /set/merkle_root)"
e2e_assert_eq "six_entry_merkle_root_hex64" "${#ORIG_MERKLE_ROOT}" "64"

# Mutate one entry in scratch work dir (leave declared lineage.merkle_root unchanged).
MOD_E1="${E2E_WORK_DIR}/e1_modified.frobcert"
python3 - "${SIX_DIR}/e1_inclusion_c03.frobcert" "${MOD_E1}" <<'PY'
import json, sys
doc = json.load(open(sys.argv[1], "r", encoding="utf-8"))
doc["checker"]["build"] = "0.0.2"
out = json.dumps(doc, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    fh.write(out)
PY

e2e_cmd --name "verify_six_entry_set_mutated" --expect-exit "3" -- \
  env -i "${VERIFY_BIN}" --json \
    "${MOD_E1}" \
    "${SIX_DIR}/e2_inclusion_c06a.frobcert" \
    "${SIX_DIR}/e3_exclusion_c01_c04.frobcert" \
    "${SIX_DIR}/e4_exclusion_c07_c12.frobcert" \
    "${SIX_DIR}/e5_partition_box.frobcert" \
    "${SIX_DIR}/e6_qualification_receipt.frobcert"

e2e_assert_eq "six_entry_mutated_exit" "${E2E_LAST_EXIT}" "3"
e2e_assert_json "six_entry_mutated_overall" "/overall" "reject"
e2e_assert_json "six_entry_mutated_rule" "/set/rule" "hash.merkle-root"
MOD_MERKLE_ROOT="$(e2e_get /set/merkle_root)"
if [ "${MOD_MERKLE_ROOT}" != "${ORIG_MERKLE_ROOT}" ]; then
  e2e_assert_eq "six_entry_merkle_root_changed" "changed" "changed"
else
  e2e_assert_eq "six_entry_merkle_root_changed" "unchanged" "changed"
fi

# ---------------------------------------------------------------------------
# Step 5: Rule-order negative control (true flag + unknown kind -> flags.true)
# ---------------------------------------------------------------------------
e2e_step "generic/flags.true/rule_order_control" "Verify true flag + unknown kind reports flags.true (FROB-REF-833) first"

ORDER_MUTANT="${MUTANTS_DIR}/reject-rule-order-flags-before-kind.frobcert"
e2e_cmd --name "verify_rule_order_control" --expect-exit "3" -- \
  env -i "${VERIFY_BIN}" --json "${ORDER_MUTANT}"
e2e_assert_eq "rule_order_exit" "${E2E_LAST_EXIT}" "3"
e2e_assert_json "rule_order_rule" "/reports/0/rule" "flags.true"
e2e_assert_json "rule_order_refusal" "/reports/0/refusal" "FROB-REF-833"
