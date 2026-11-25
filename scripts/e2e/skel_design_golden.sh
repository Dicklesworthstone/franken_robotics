#!/usr/bin/env bash
# e2e: bead=N14a-26 milestone=M1 labels=skeleton,solid,oracle,gauntlet plants=expected-byte-flip
#
# End-to-end verification of the independent `frob.design/1` reference
# canonicalizer, golden corpus, manifest integrity, and comparison harness
# (`N14a-26`):
#   1. selftest: BLAKE3 published vectors + reference unit self-tests.
#   2. regenerate: fresh regeneration into a scratch directory via
#      `tools/oracle/design_canon/regenerate.sh --out-dir ...` and byte-for-byte
#      comparison of every `.canon` and `DesignId` against stored expectations
#      (logging input hash, expected/regenerated DesignId, and 20-byte diff
#      context on any mismatch).
#   3. manifest: oracle script hash, manifest self-hash, per-group counts, zero
#      stray files.
#   4. harness: Rust `design_golden` conformance test suite (reference + all 4
#      planted wrong canonicalizers + flipped expected byte control).
#   5. summary: corpus counts and manifest hash verification.
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "skel_design_golden" "N14a-26"

CORPUS_DIR="${E2E_REPO_ROOT}/crates/frob-conformance/fixtures/design_golden"
ORACLE_DIR="${E2E_REPO_ROOT}/tools/oracle/design_canon"

# ---------------------------------------------------------------------------
# Step 1: selftest (BLAKE3 test vectors and reference self-tests)
# ---------------------------------------------------------------------------
e2e_step "selftest" "Run BLAKE3 published test vectors and reference canonicalizer self-tests"
e2e_cmd --name reference_selftest -- bash "${ORACLE_DIR}/selftest.sh"
e2e_assert_eq "reference_selftest_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 2: regenerate (fresh regeneration into scratch + byte-for-byte compare)
# ---------------------------------------------------------------------------
e2e_step "regenerate" "Regenerate golden corpus into scratch directory and compare every .canon and DesignId"
REGEN_DIR="${E2E_WORK_DIR}/regen_corpus"
mkdir -p "${REGEN_DIR}"

e2e_cmd --name regenerate_into_scratch -- \
  bash "${ORACLE_DIR}/regenerate.sh" --corpus-dir "${CORPUS_DIR}" --out-dir "${REGEN_DIR}"
e2e_assert_eq "regenerate_into_scratch_exit" "${E2E_LAST_EXIT}" "0"

COMPARE_CORPUS="${CORPUS_DIR}"
if e2e_plant "expected-byte-flip"; then
  COMPARE_CORPUS="${E2E_WORK_DIR}/planted_expected_flip"
  mkdir -p "${COMPARE_CORPUS}"
  cp "${CORPUS_DIR}"/*.fbot "${CORPUS_DIR}"/*.canon "${CORPUS_DIR}/manifest.toml" "${CORPUS_DIR}/PIN.txt" "${COMPARE_CORPUS}/"
  python3 - "${COMPARE_CORPUS}/10_qty_0_42m.canon" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
b = bytearray(p.read_bytes())
b[10] = ord("Y") if b[10] == ord("X") else ord("X")
p.write_bytes(bytes(b))
PY
fi

e2e_cmd --name compare_regenerated_expectations -- \
  python3 - "${ORACLE_DIR}" "${COMPARE_CORPUS}" "${REGEN_DIR}" <<'PY'
from pathlib import Path
import re, sys

oracle_dir = Path(sys.argv[1])
stored_dir = Path(sys.argv[2])
regen_dir = Path(sys.argv[3])
sys.path.insert(0, str(oracle_dir))
import blake3_ref

def parse_manifest(path: Path):
    text = path.read_text(encoding="utf-8")
    top = {}
    for k in ("schema", "oracle", "oracle_blake3", "manifest_blake3"):
        m = re.search(rf'^{k}\s*=\s*"([^"]*)"\s*$', text, flags=re.M)
        if m:
            top[k] = m.group(1)
    rows = []
    for blk in text.split("[[case]]")[1:]:
        r = {}
        for m in re.finditer(r'^\s*([a-z_]+)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$', blk, flags=re.M):
            r[m.group(1)] = m.group(2).replace('\\"', '"').replace("\\\\", "\\")
        if r:
            rows.append(r)
    return top, rows

stored_top, stored_rows = parse_manifest(stored_dir / "manifest.toml")
regen_top, regen_rows = parse_manifest(regen_dir / "manifest.toml")

if len(stored_rows) != len(regen_rows):
    print(f"MISMATCH case count stored={len(stored_rows)} regen={len(regen_rows)}", file=sys.stderr)
    sys.exit(1)

mismatches = 0
for srow, rrow in zip(stored_rows, regen_rows):
    cid = srow["id"]
    if srow["expect"] == "accept":
        sb = (stored_dir / srow["canon"]).read_bytes()
        rb = (regen_dir / rrow["canon"]).read_bytes()
        s_did = blake3_ref.design_id(sb)
        r_did = blake3_ref.design_id(rb)
        if sb != rb or s_did != r_did or s_did != srow["design_id"]:
            mismatches += 1
            n = min(len(sb), len(rb))
            off = next((i for i in range(n) if sb[i] != rb[i]), n)
            s_ctx = sb[max(0, off - 10) : off + 10]
            r_ctx = rb[max(0, off - 10) : off + 10]
            print(
                f"MISMATCH file={srow['canon']} offset={off} "
                f"input_blake3={srow['input_blake3']} expected_id={srow['design_id']} regen_id={r_did} "
                f"expected_ctx={s_ctx!r} regen_ctx={r_ctx!r}",
                file=sys.stderr,
            )
        else:
            print(
                f"OK id={cid} group={srow['group']} input_blake3={srow['input_blake3']} "
                f"expected_id={srow['design_id']} regen_id={r_did}"
            )
    else:
        print(
            f"OK_REFUSE id={cid} group={srow['group']} input_blake3={srow['input_blake3']} "
            f"reason={srow['reason']}"
        )

if stored_top.get("manifest_blake3") != regen_top.get("manifest_blake3"):
    mismatches += 1
    print(
        f"MISMATCH manifest_blake3 stored={stored_top.get('manifest_blake3')} "
        f"regen={regen_top.get('manifest_blake3')}",
        file=sys.stderr,
    )

print(
    f"SUMMARY total={len(stored_rows)} oracle_blake3={regen_top.get('oracle_blake3')} "
    f"manifest_blake3={regen_top.get('manifest_blake3')}"
)
sys.exit(1 if mismatches else 0)
PY
e2e_assert_eq "regenerate_byte_identical_exit" "${E2E_LAST_EXIT}" "0"

# ---------------------------------------------------------------------------
# Step 3: manifest (script hash, per-group counts, zero stray files)
# ---------------------------------------------------------------------------
e2e_step "manifest" "Validate manifest completeness, reference script hash, and per-group counts"
e2e_cmd --name manifest_checks -- \
  cargo test -p frob-conformance --test design_golden -- manifest_is_complete expectations_from_reference_only --nocapture
e2e_assert_eq "manifest_checks_exit" "${E2E_LAST_EXIT}" "0"

_group_kv="$(python3 - "${CORPUS_DIR}/manifest.toml" <<'PY'
from collections import Counter
from pathlib import Path
import re, sys

txt = Path(sys.argv[1]).read_text(encoding="utf-8")
groups = re.findall(r'^\s*group\s*=\s*"([^"]+)"\s*$', txt, flags=re.M)
expects = re.findall(r'^\s*expect\s*=\s*"([^"]+)"\s*$', txt, flags=re.M)
m_hash = re.search(r'^manifest_blake3\s*=\s*"([0-9a-f]{64})"\s*$', txt, flags=re.M)
o_hash = re.search(r'^oracle_blake3\s*=\s*"([0-9a-f]{64})"\s*$', txt, flags=re.M)
c = Counter(groups)
ec = Counter(expects)
print(f"total={len(groups)}")
print(f"accepted={ec['accept']}")
print(f"refused={ec['refuse']}")
for g in sorted(c):
    print(f"group_{g}={c[g]}")
if m_hash:
    print(f"manifest_blake3={m_hash.group(1)}")
if o_hash:
    print(f"oracle_blake3={o_hash.group(1)}")
PY
)"

_manifest_hash=""
_oracle_hash=""
while IFS='=' read -r _k _v; do
  case "${_k}" in
    total)    e2e_metric "design_golden_cases_total" "${_v}" "cases" ;;
    accepted) e2e_metric "design_golden_cases_accepted" "${_v}" "cases" ;;
    refused)  e2e_metric "design_golden_cases_refused" "${_v}" "cases" ;;
    group_*)  e2e_metric "design_golden_${_k}" "${_v}" "cases" ;;
    manifest_blake3) _manifest_hash="${_v}" ;;
    oracle_blake3)   _oracle_hash="${_v}" ;;
  esac
done <<< "${_group_kv}"

# ---------------------------------------------------------------------------
# Step 4: harness (reference + 4 planted wrong canonicalizers + byte-flip control)
# ---------------------------------------------------------------------------
e2e_step "harness" "Run full design_golden Rust harness against reference and planted wrong canonicalizers"
e2e_cmd --name design_golden_harness_suite -- \
  cargo test -p frob-conformance --test design_golden -- --nocapture
e2e_assert_eq "design_golden_harness_suite_exit" "${E2E_LAST_EXIT}" "0"

# Negative control: flip one byte in a scratch copy and require regeneration comparison to catch it.
WS_FLIP="${E2E_WORK_DIR}/ctrl_byte_flip"
mkdir -p "${WS_FLIP}"
cp "${CORPUS_DIR}"/*.fbot "${CORPUS_DIR}"/*.canon "${CORPUS_DIR}/manifest.toml" "${CORPUS_DIR}/PIN.txt" "${WS_FLIP}/"
python3 - "${WS_FLIP}/10_qty_0_42m.canon" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
b = bytearray(p.read_bytes())
b[10] = ord("Y") if b[10] == ord("X") else ord("X")
p.write_bytes(bytes(b))
PY

e2e_expect_caught "expected-byte-flip" -- \
  cmp -s "${WS_FLIP}/10_qty_0_42m.canon" "${REGEN_DIR}/10_qty_0_42m.canon"

# ---------------------------------------------------------------------------
# Step 5: summary (verify regenerated manifest hash matches stored manifest hash)
# ---------------------------------------------------------------------------
e2e_step "summary" "Verify regenerated manifest hash and oracle hash match stored manifest"
_regen_manifest_hash="$(python3 - "${REGEN_DIR}/manifest.toml" <<'PY'
from pathlib import Path
import re, sys
txt = Path(sys.argv[1]).read_text(encoding="utf-8")
m = re.search(r'^manifest_blake3\s*=\s*"([0-9a-f]{64})"\s*$', txt, flags=re.M)
print(m.group(1) if m else "")
PY
)"
e2e_assert_digest_eq "manifest_blake3_matches_regen" "${_regen_manifest_hash}" "${_manifest_hash}"
e2e_assert_eq "oracle_blake3_nonempty" "$([ -n "${_oracle_hash}" ] && echo 1 || echo 0)" "1"
