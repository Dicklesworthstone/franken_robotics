#!/usr/bin/env bash
# e2e: bead=N12-02 milestone=M0 labels=lemma,frob-cert seed=0 needs=cpu
#
# End-to-end verification of the closed, versioned proof-kind registry
# `frob.cert.kinds/1` in `frob-cert` (bead N12-02).
#
# Drives `frob-cert` from outside through the std-only `kind_registry` example:
#   1. Emits compiled table JSON and checks 27 unique rows, registry version, and caps.
#   2. Regenerates `docs/registries/cert-kinds.md` into scratch and checks digest parity.
#   3. Probes all 7 dispatch/scope/cap/status controls (fabricated kind, unknown version,
#      out of scope, disabled via `registry-fixture`, not implemented, class above cap,
#      and in-scope at cap).
#
# Compatible with bash 3.2 (macOS) and Linux. Writes only under `target/`.
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "lemma_kind_registry" "N12-02" "$@"

MANIFEST="${E2E_REPO_ROOT}/Cargo.toml"
DOC_COMMITTED="${E2E_REPO_ROOT}/docs/registries/cert-kinds.md"
CONTROLS_RUN=0
CONTROLS_REJECTED=0

# ---------------------------------------------------------------------------
# Step 1: Table export & structural invariants (27 unique kinds + caps)
# ---------------------------------------------------------------------------
e2e_step "registry/table" "Export compiled proof-kind registry table as JSON and verify invariants"
e2e_cmd --name "table_json" -- cargo run --locked -p frob-cert --manifest-path "${MANIFEST}" --example kind_registry -- --json

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "registry_version" "/registry_version" "frob.cert.kinds/1"
  e2e_assert_json "kind_count" "/kind_count" "27"

  TABLE_FILE="${E2E_LAST_STDOUT}"
  TABLE_CHECK_JSON="${E2E_WORK_DIR}/table_check.json"
  python3 - "${TABLE_FILE}" "${TABLE_CHECK_JSON}" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as f:
    doc = json.load(f)

kinds = doc.get("kinds", [])
names = [k["name"] for k in kinds]
unique_names = len(set(names))
proved = sorted(k["name"] for k in kinds if k["cap"] == "Proved")
tested = sorted(k["name"] for k in kinds if k["cap"] == "Tested")
certified = sum(1 for k in kinds if k["cap"] == "Certified")
out = {
    "count": len(kinds),
    "unique_names": unique_names,
    "proved": ",".join(proved),
    "tested": ",".join(tested),
    "certified_count": certified,
}
with open(sys.argv[2], "w", encoding="utf-8") as f:
    json.dump(out, f, sort_keys=True)
PY

  e2e_assert_eq "unique_kind_count" "$(e2e_get --file "${TABLE_CHECK_JSON}" /unique_names)" "27"
  e2e_assert_eq "proved_cap_kinds" "$(e2e_get --file "${TABLE_CHECK_JSON}" /proved)" "lean-capsule"
  e2e_assert_eq "tested_cap_kinds" "$(e2e_get --file "${TABLE_CHECK_JSON}" /tested)" "sample-coverage,trace-witness"
  e2e_assert_eq "certified_cap_count" "$(e2e_get --file "${TABLE_CHECK_JSON}" /certified_count)" "24"
fi

# ---------------------------------------------------------------------------
# Step 2: Regenerate docs/registries/cert-kinds.md and verify digest parity
# ---------------------------------------------------------------------------
e2e_step "registry/doc" "Regenerate docs/registries/cert-kinds.md and assert exact digest equality"
e2e_assert_file "committed_doc_exists" "${DOC_COMMITTED}"
e2e_cmd --name "doc_regen" -- cargo run --locked -p frob-cert --manifest-path "${MANIFEST}" --example kind_registry -- --doc

if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  DOC_GEN="${E2E_WORK_DIR}/cert-kinds.generated.md"
  cp "${E2E_LAST_STDOUT}" "${DOC_GEN}"

  if command -v b3sum >/dev/null 2>&1; then
    GEN_DIGEST="$(b3sum --no-names "${DOC_GEN}")"
    COM_DIGEST="$(b3sum --no-names "${DOC_COMMITTED}")"
  else
    GEN_DIGEST="$("${FROB_E2E_TOOL}" cmp sha256 "${DOC_GEN}")"
    COM_DIGEST="$("${FROB_E2E_TOOL}" cmp sha256 "${DOC_COMMITTED}")"
  fi
  e2e_assert_digest_eq "cert_kinds_doc_matches_table" "${GEN_DIGEST}" "${COM_DIGEST}"
fi

# ---------------------------------------------------------------------------
# Step 3: Probe every control through kind_registry --probe
# ---------------------------------------------------------------------------
run_probe() {
  local step_name="$1"
  local title="$2"
  local extra_feat="$3"
  local kind="$4"
  local version="$5"
  local claim_kind="$6"
  local class_name="$7"
  local exp_verdict="$8"
  local exp_rule="$9"
  local exp_refusal="${10}"
  local tag_flag="${11:-}"
  local tag_val="${12:-}"

  e2e_step "registry/${step_name}" "${title}"
  local cmd=(cargo run --locked -p frob-cert --manifest-path "${MANIFEST}")
  if [ -n "${extra_feat}" ]; then
    cmd=("${cmd[@]}" --features "${extra_feat}")
  fi
  cmd=("${cmd[@]}" --example kind_registry -- --probe "${kind}" "${version}" "${claim_kind}" "${class_name}" --json)
  e2e_cmd --name "probe_${step_name}" -- "${cmd[@]}"

  CONTROLS_RUN=$((CONTROLS_RUN + 1))

  if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
    if [ -n "${tag_flag}" ]; then
      e2e_assert_json "${step_name}:verdict" "/verdict" "${exp_verdict}" "${tag_flag}" "${tag_val}"
    else
      e2e_assert_json "${step_name}:verdict" "/verdict" "${exp_verdict}"
    fi
    e2e_assert_json "${step_name}:registry_version" "/registry_version" "frob.cert.kinds/1"
    e2e_assert_json "${step_name}:rule" "/rule" "${exp_rule}"
    e2e_assert_json "${step_name}:refusal" "/refusal" "${exp_refusal}"

    if [ "${exp_verdict}" = "reject" ] && [ "$(e2e_get /verdict)" = "reject" ]; then
      CONTROLS_REJECTED=$((CONTROLS_REJECTED + 1))
    fi
  elif [ "${exp_verdict}" = "reject" ]; then
    CONTROLS_REJECTED=$((CONTROLS_REJECTED + 1))
  fi
}

run_probe \
  "fabricated-kind" \
  "Fabricated proof kind rejects with kind.unknown and FROB-REF-823 (KA-215 / CX-106)" \
  "" \
  "lp-primal-fast" "1" "family-inclusion" "Certified" \
  "reject" "kind.unknown" "FROB-REF-823" \
  "--ka" "KA-215"

run_probe \
  "unknown-version" \
  "Known kind at version 2 rejects with kind.version-unknown and FROB-REF-823" \
  "" \
  "lp-primal" "2" "family-inclusion" "Certified" \
  "reject" "kind.version-unknown" "FROB-REF-823"

run_probe \
  "out-of-scope" \
  "poly-identity under mission claim family-inclusion rejects with kind.out-of-scope" \
  "" \
  "poly-identity" "1" "family-inclusion" "Certified" \
  "reject" "kind.out-of-scope" "FROB-REF-823"

run_probe \
  "disabled" \
  "Disabled kind under registry-fixture rejects with kind.disabled and FROB-REF-823" \
  "registry-fixture" \
  "alpha-root" "1" "kernel-identity" "Certified" \
  "reject" "kind.disabled" "FROB-REF-823"

run_probe \
  "not-implemented" \
  "Not-implemented kind tube-inclusion returns unsupported (never accept)" \
  "" \
  "tube-inclusion" "1" "kernel-identity" "Certified" \
  "unsupported" "null" "null"

run_probe \
  "class-above-cap" \
  "sample-coverage at Certified rejects with class.above-kind-cap and FROB-REF-834" \
  "" \
  "sample-coverage" "1" "qualification-receipt" "Certified" \
  "reject" "class.above-kind-cap" "FROB-REF-834"

run_probe \
  "in-scope-at-cap" \
  "In-scope sample-coverage at Tested passes scope and cap checks (unsupported until checker lands)" \
  "" \
  "sample-coverage" "1" "qualification-receipt" "Tested" \
  "unsupported" "null" "null"
if [ "${E2E_DRY_RUN:-0}" != "1" ]; then
  e2e_assert_json "in-scope-at-cap:scope_check" "/scope_check" "pass"
  e2e_assert_json "in-scope-at-cap:cap_check" "/cap_check" "pass"
fi

e2e_step "registry/metrics" "Record N12-02 control counts"
e2e_metric "controls_run" "${CONTROLS_RUN}" "count" "==" "7"
e2e_metric "controls_rejected" "${CONTROLS_REJECTED}" "count" "==" "5"
