#!/usr/bin/env bash
# e2e: bead=N34-16 milestone=M0 labels=delivery,e2e,m0,solid,test,workspace seed=0 timeout_s=600 needs=cpu plants=missing-dependency-admission,duplicate-format-extension,annex-toolchain-file,edited-receipt-bytes,hand-edited-status-block,non-compiling-member,missing-p0-upstream-bead,measured-gate-in-first-commit
#
# scripts/e2e/first_commit.sh
# End-to-end verification of the first commit's inventory (and standing
# consistency on later commits via `--mode standing`) from a clean clone in an
# isolated scratch directory (bead N34-16).
#
# Usage:
#   scripts/e2e/first_commit.sh [--commit <rev>] [--mode first|standing]
#                               [--repo <path>] [--state-dir <dir>]
#
# Never writes outside its scratch directory (except `--state-dir` if passed),
# never deletes files or directories (COMMON.md 9b), and never touches a sibling
# checkout. All heavy commands run through `e2e_cmd` / `frob_envelope_run` (E0).
# Bash 3.2 (macOS) and Linux compatible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

usage() {
  cat <<'EOF'
Usage: scripts/e2e/first_commit.sh [OPTIONS]

Options:
  --commit <rev>           Commit revision to clone and inspect (default: HEAD)
  --mode <first|standing>  Inventory mode: first (default) or standing
  --repo <path>            Source git repository to clone (default: repo root)
  --state-dir <dir>        Optional dir storing standing-mode last passing commit
  -h, --help               Show this help and exit 0
EOF
}

ORIG_ARGS=("$@")
COMMIT_REV="${FIRST_COMMIT_REV:-HEAD}"
MODE="${FIRST_COMMIT_MODE:-first}"
REPO_SRC="${FIRST_COMMIT_REPO:-}"
STATE_DIR="${FIRST_COMMIT_STATE_DIR:-}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --commit)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      COMMIT_REV="$2"
      shift 2
      ;;
    --mode)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      MODE="$2"
      shift 2
      ;;
    --repo)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      REPO_SRC="$2"
      shift 2
      ;;
    --state-dir)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      STATE_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'first_commit.sh: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

case "${MODE}" in
  first|standing) ;;
  *)
    printf 'first_commit.sh: invalid --mode `%s` (expected first or standing)\n' "${MODE}" >&2
    exit 2
    ;;
esac

# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "first_commit" "N34-16" ${ORIG_ARGS[@]+"${ORIG_ARGS[@]}"}

if [ -z "${REPO_SRC}" ]; then
  REPO_SRC="${E2E_REPO_ROOT}"
fi
if [ ! -d "${REPO_SRC}" ]; then
  printf 'first_commit.sh: source repository not found: %s\n' "${REPO_SRC}" >&2
  exit 2
fi
REPO_SRC="$(cd "${REPO_SRC}" && pwd -P)"
REPO_PARENT="$(dirname "${REPO_SRC}")"

B3SUM_BIN=""
if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
fi

hash_b3() {
  "${B3SUM_BIN}" --no-names "$1"
}

if [ "${E2E_DRY_RUN:-0}" = "1" ]; then
  printf '[dry-run] first_commit.sh --repo %s --commit %s --mode %s\n' "${REPO_SRC}" "${COMMIT_REV}" "${MODE}"
  exit 77
fi

# ---------------------------------------------------------------------------
# Preflight: disk space (Rule 3) and b3sum tool availability
# ---------------------------------------------------------------------------
frob_disk_preflight "${E2E_WORK_DIR}" "1"

if [ -z "${B3SUM_BIN}" ]; then
  e2e_step "step_1_clone_and_list" "Clone and list"
  e2e_assert_eq "b3sum_required" "missing" "present"
  printf 'first_commit.sh: b3sum is required for BLAKE3 digests (not found on PATH)\n' >&2
  exit 1
fi

SANDBOX_DIR="${E2E_WORK_DIR}/sandbox"
CLONE_DIR="${SANDBOX_DIR}/clone"
TARGET_BASE="${RCH_TARGET_BASE:-${E2E_WORK_DIR}}/frob-first-commit-${E2E_RUN_ID}"
mkdir -p "${SANDBOX_DIR}" "${TARGET_BASE}"

FAIL_COUNT=0
note_fail() {
  FAIL_COUNT=$((FAIL_COUNT + 1))
}

# ---------------------------------------------------------------------------
# Step 1: Clone and list
# ---------------------------------------------------------------------------
e2e_step "step_1_clone_and_list" "Clone commit under test with --no-hardlinks into scratch and record commit"

TARGET_SHA=""
if ! TARGET_SHA="$(git --no-optional-locks -C "${REPO_SRC}" rev-parse --verify "${COMMIT_REV}^{commit}" 2>/dev/null)"; then
  e2e_assert_eq "step1_resolve_commit" "unresolved:${COMMIT_REV}" "40-hex commit"
  exit 1
fi

e2e_cmd --name "git_clone_no_hardlinks" -- \
  git --no-optional-locks clone --no-hardlinks --no-checkout "${REPO_SRC}" "${CLONE_DIR}"
e2e_assert_eq "step1_git_clone_exit" "${E2E_LAST_EXIT}" "0"
if [ "${E2E_LAST_EXIT}" != "0" ]; then
  exit 1
fi

e2e_cmd --name "git_checkout_commit" -- \
  git --no-optional-locks -C "${CLONE_DIR}" checkout --detach "${TARGET_SHA}"
e2e_assert_eq "step1_git_checkout_exit" "${E2E_LAST_EXIT}" "0"
if [ "${E2E_LAST_EXIT}" != "0" ]; then
  exit 1
fi

CLONED_SHA="$(git --no-optional-locks -C "${CLONE_DIR}" rev-parse HEAD)"
e2e_assert_eq "step1_clone_commit_sha" "${CLONED_SHA}" "${TARGET_SHA}"

TRACKED_LIST="${E2E_WORK_DIR}/tracked_files.txt"
git --no-optional-locks -C "${CLONE_DIR}" ls-files > "${TRACKED_LIST}"
TRACKED_COUNT="$(wc -l < "${TRACKED_LIST}" | tr -d ' ')"
e2e_metric "step1_tracked_files" "${TRACKED_COUNT}" "count"
e2e_metric "step1_mode_standing" "$([ "${MODE}" = "standing" ] && echo 1 || echo 0)" "bool"

# Expose existing sibling repositories read-only beside ${CLONE_DIR} via
# symlinks inside scratch so relative `../<sibling>` paths in Cargo.toml and
# franken-stack.lock resolve cleanly without copying or modifying any sibling.
python3 - "${CLONE_DIR}" "${REPO_PARENT}" "${SANDBOX_DIR}" <<'PY'
import os, sys, tomllib
from pathlib import Path

clone_dir = Path(sys.argv[1])
repo_parent = Path(sys.argv[2])
sandbox_dir = Path(sys.argv[3])
names = set()

lock_p = clone_dir / "franken-stack.lock"
if lock_p.is_file():
    try:
        doc = tomllib.loads(lock_p.read_text(encoding="utf-8"))
        for row in doc.get("foundation", []):
            if isinstance(row, dict):
                rel = row.get("path", "")
                if isinstance(rel, str) and rel.startswith("../"):
                    seg = rel[3:].split("/", 1)[0]
                    if seg and seg not in (".", ".."):
                        names.add(seg)
    except Exception:
        pass

cargo_p = clone_dir / "Cargo.toml"
if cargo_p.is_file():
    try:
        doc = tomllib.loads(cargo_p.read_text(encoding="utf-8"))
        for src in doc.get("patch", {}).values():
            if isinstance(src, dict):
                for spec in src.values():
                    if isinstance(spec, dict):
                        rel = spec.get("path", "")
                        if isinstance(rel, str) and rel.startswith("../"):
                            seg = rel[3:].split("/", 1)[0]
                            if seg and seg not in (".", ".."):
                                names.add(seg)
    except Exception:
        pass

for seg in sorted(names):
    src = repo_parent / seg
    dst = sandbox_dir / seg
    if src.is_dir() and not dst.exists() and not dst.is_symlink():
        os.symlink(str(src), str(dst))
PY

# Apply pre-Step-2 planted mutations strictly inside ${CLONE_DIR} (never rm).
if e2e_plant "missing-dependency-admission"; then
  if [ -e "${CLONE_DIR}/DEPENDENCY_ADMISSION.md" ]; then
    mv "${CLONE_DIR}/DEPENDENCY_ADMISSION.md" "${E2E_WORK_DIR}/stashed_DEPENDENCY_ADMISSION.md"
  fi
elif e2e_plant "duplicate-format-extension"; then
  if [ -f "${CLONE_DIR}/registries/formats.toml" ]; then
    cat >> "${CLONE_DIR}/registries/formats.toml" <<'EOF'

[[format]]
id = "FMT-PLANTED"
extension = ".fbot"
schema = "frob.planted/1"
owner = "frob-core"
producers = ["frob-core"]
consumers = ["frob-core"]
encoding = "canonical"
EOF
  fi
elif e2e_plant "annex-toolchain-file"; then
  mkdir -p "${CLONE_DIR}/spark"
  cat > "${CLONE_DIR}/spark/rust-toolchain.toml" <<'EOF'
[toolchain]
channel = "nightly-2026-08-31"
EOF
elif e2e_plant "edited-receipt-bytes"; then
  PLANT_REC="$(python3 - "${CLONE_DIR}" <<'PY'
import re, sys
from pathlib import Path

clone = Path(sys.argv[1])
rdir = clone / "receipts"
head_p = rdir / "HEAD"
if head_p.is_file():
    h = head_p.read_text(encoding="utf-8").strip()
    cand = rdir / f"{h}.json"
    if cand.is_file():
        print(cand)
        sys.exit(0)
hex64 = re.compile(r"^[0-9a-f]{64}\.json$")
cands = sorted(p for p in rdir.iterdir() if p.is_file() and hex64.match(p.name)) if rdir.is_dir() else []
if cands:
    print(cands[-1])
PY
)"
  if [ -n "${PLANT_REC}" ] && [ -f "${PLANT_REC}" ]; then
    # Append one trailing space: JSON fields remain identical, BLAKE3 changes.
    printf ' ' >> "${PLANT_REC}"
  fi
elif e2e_plant "hand-edited-status-block"; then
  if [ -f "${CLONE_DIR}/README.md" ]; then
    python3 - "${CLONE_DIR}/README.md" <<'PY'
import re, sys
from pathlib import Path
p = Path(sys.argv[1])
txt = p.read_text(encoding="utf-8")
updated, n = re.subn(r"Summary: \d+ gates", "Summary: 999 gates", txt, count=1)
if n == 0:
    updated = txt.replace("<!-- status:end -->", "<!-- hand-edited -->\n<!-- status:end -->", 1)
p.write_text(updated, encoding="utf-8")
PY
  fi
elif e2e_plant "non-compiling-member"; then
  mkdir -p "${CLONE_DIR}/planted_broken_ws/src"
  cat > "${CLONE_DIR}/planted_broken_ws/Cargo.toml" <<'EOF'
[workspace]

[package]
name = "planted-broken-member"
version = "0.0.1"
edition = "2024"
EOF
  cat > "${CLONE_DIR}/planted_broken_ws/Cargo.lock" <<'EOF'
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "planted-broken-member"
version = "0.0.1"
EOF
  cat > "${CLONE_DIR}/planted_broken_ws/src/lib.rs" <<'EOF'
compile_error!("planted non-compiling workspace member for N34-16");
EOF
elif e2e_plant "missing-p0-upstream-bead"; then
  if [ -f "${CLONE_DIR}/.beads/issues.jsonl" ]; then
    python3 - "${CLONE_DIR}/.beads/issues.jsonl" <<'PY'
import json, sys
from pathlib import Path
p = Path(sys.argv[1])
kept = []
for raw in p.read_text(encoding="utf-8").splitlines():
    if not raw.strip():
        continue
    obj = json.loads(raw)
    title = str(obj.get("title", ""))
    ext = str(obj.get("external_ref", ""))
    bid = str(obj.get("id", ""))
    if "FD-FMCP-01" in title or ext == "FD-FMCP-01" or bid.startswith("frob-fd-fmcp-01"):
        continue
    kept.append(raw)
p.write_text("\n".join(kept) + ("\n" if kept else ""), encoding="utf-8")
PY
  fi
fi

# ---------------------------------------------------------------------------
# Step 2: Presence
# ---------------------------------------------------------------------------
e2e_step "step_2_presence" "Check presence of items 1..10, registries layout, schemas, and AGENTS.md working-rules table"

if [ -f "${CLONE_DIR}/DEPENDENCY_ADMISSION.md" ]; then
  e2e_assert_eq "step2_presence_missing-dependency-admission" "present" "present"
else
  e2e_assert_eq "step2_presence_missing-dependency-admission" "missing:DEPENDENCY_ADMISSION.md" "present"
  note_fail
fi

STEP2_OUT="${E2E_WORK_DIR}/step2.json"
python3 - "${CLONE_DIR}" "${MODE}" "${STEP2_OUT}" <<'PY'
import json, re, sys
from pathlib import Path

clone = Path(sys.argv[1])
mode = sys.argv[2]
out_p = Path(sys.argv[3])

required_files = [
    "AGENTS.md",
    "README.md",
    "COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md",
    "franken-stack.lock",
    "rust-toolchain.toml",
    ".cargo/config.toml",
    "schemas/demo.schema.json",
    "schemas/contract.schema.json",
]
missing_files = [rel for rel in required_files if not (clone / rel).is_file()]

# Receipt presence under receipts/
rdir = clone / "receipts"
hex64_re = re.compile(r"^[0-9a-f]{64}\.json$")
receipt_files = sorted(
    p.name for p in rdir.iterdir() if p.is_file() and hex64_re.match(p.name)
) if rdir.is_dir() else []
if not receipt_files:
    missing_files.append("receipts/<64-hex>.json")

# README status markers presence
readme_p = clone / "README.md"
if readme_p.is_file():
    rtxt = readme_p.read_text(encoding="utf-8", errors="replace")
    if "<!-- status:begin -->" not in rtxt or "<!-- status:end -->" not in rtxt:
        missing_files.append("README.md:status-markers")

# registries/ entries
expected_reg_files = {
    "verbs.toml",
    "refusals.toml",
    "kernels.toml",
    "gates.toml",
    "formats.toml",
    "upstream.toml",
    "lifts.toml",
    "capsules.toml",
}
expected_reg_all = expected_reg_files | {"cards"}
reg_dir = clone / "registries"
reg_errors = []
if not reg_dir.is_dir():
    reg_errors.append("missing directory registries/")
else:
    actual_entries = sorted(p.name for p in reg_dir.iterdir() if not p.name.startswith("."))
    actual_set = set(actual_entries)
    for req in sorted(expected_reg_files):
        if not (reg_dir / req).is_file():
            reg_errors.append(f"missing registries/{req}")
    if not (reg_dir / "cards").is_dir():
        reg_errors.append("missing directory registries/cards/")
    if mode == "first":
        extras = sorted(actual_set - expected_reg_all)
        if extras:
            reg_errors.append(f"unexpected entries in registries/ for first commit: {', '.join(extras)}")

# AGENTS.md heading `## Working rules` and 12 numbered rows (1..12)
agents_err = ""
agents_p = clone / "AGENTS.md"
if not agents_p.is_file():
    agents_err = "AGENTS.md missing"
else:
    lines = agents_p.read_text(encoding="utf-8", errors="replace").splitlines()
    start_idx = None
    for idx, line in enumerate(lines):
        if line.strip() == "## Working rules":
            start_idx = idx + 1
            break
    if start_idx is None:
        agents_err = "AGENTS.md missing exact heading `## Working rules`"
    else:
        section_lines = []
        for line in lines[start_idx:]:
            if line.startswith("## "):
                break
            section_lines.append(line)
        # Parse markdown pipe table rows after header+separator
        row_nums = []
        in_table = False
        saw_sep = False
        for line in section_lines:
            s = line.strip()
            if s.startswith("|") and s.endswith("|"):
                cells = [c.strip() for c in s.strip("|").split("|")]
                if not in_table:
                    in_table = True
                    continue
                if not saw_sep and all(re.match(r"^:?-{2,}:?$", c) for c in cells if c):
                    saw_sep = True
                    continue
                if saw_sep and cells:
                    row_nums.append(cells[0])
            elif in_table and saw_sep:
                break
        expected_nums = [str(i) for i in range(1, 13)]
        if row_nums != expected_nums:
            agents_err = f"AGENTS.md `## Working rules` table rows={row_nums!r}, expected 1..12"

# Schemas demo.schema.json and contract.schema.json valid JSON
schema_errs = []
for srel in ("schemas/demo.schema.json", "schemas/contract.schema.json"):
    sp = clone / srel
    if sp.is_file():
        try:
            obj = json.loads(sp.read_text(encoding="utf-8"))
            if not isinstance(obj, dict) or "$schema" not in obj:
                schema_errs.append(f"{srel}: missing $schema object")
        except Exception as exc:
            schema_errs.append(f"{srel}: invalid JSON ({exc})")

out_p.write_text(
    json.dumps(
        {
            "missing_files": missing_files,
            "reg_errors": reg_errors,
            "agents_err": agents_err,
            "schema_errs": schema_errs,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
PY

S2_FILES="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["missing_files"] else "; ".join(d["missing_files"]))' "${STEP2_OUT}")"
e2e_assert_eq "step2_presence_required_files" "${S2_FILES}" "ok"
[ "${S2_FILES}" = "ok" ] || note_fail

S2_REGS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["reg_errors"] else "; ".join(d["reg_errors"]))' "${STEP2_OUT}")"
e2e_assert_eq "step2_presence_registries" "${S2_REGS}" "ok"
[ "${S2_REGS}" = "ok" ] || note_fail

S2_AGENTS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["agents_err"] else d["agents_err"])' "${STEP2_OUT}")"
e2e_assert_eq "step2_presence_agents_working_rules" "${S2_AGENTS}" "ok"
[ "${S2_AGENTS}" = "ok" ] || note_fail

S2_SCHEMAS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["schema_errs"] else "; ".join(d["schema_errs"]))' "${STEP2_OUT}")"
e2e_assert_eq "step2_presence_schemas_valid_json" "${S2_SCHEMAS}" "ok"
[ "${S2_SCHEMAS}" = "ok" ] || note_fail

# ---------------------------------------------------------------------------
# Step 3: Registries
# ---------------------------------------------------------------------------
e2e_step "step_3_registries" "Parse and schema-validate every registry; enforce empty-eight and 13-format invariants"

STEP3_OUT="${E2E_WORK_DIR}/step3.json"
python3 - "${CLONE_DIR}" "${MODE}" "${STEP3_OUT}" <<'PY'
import json, re, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
mode = sys.argv[2]
out_p = Path(sys.argv[3])

# Minimal Draft 2020-12 JSON Schema validator (stdlib only).
def resolve_ref(root_schema, ref):
    if not isinstance(ref, str) or not ref.startswith("#/"):
        return None
    cur = root_schema
    for part in ref[2:].split("/"):
        part = part.replace("~1", "/").replace("~0", "~")
        if isinstance(cur, dict) and part in cur:
            cur = cur[part]
        else:
            return None
    return cur

def type_matches(val, tname):
    if tname == "null":
        return val is None
    if tname == "boolean":
        return isinstance(val, bool)
    if tname == "integer":
        return isinstance(val, int) and not isinstance(val, bool)
    if tname == "number":
        return isinstance(val, (int, float)) and not isinstance(val, bool)
    if tname == "string":
        return isinstance(val, str)
    if tname == "array":
        return isinstance(val, list)
    if tname == "object":
        return isinstance(val, dict)
    return True

def validate_schema(val, schema, root_schema, path="$"):
    if not isinstance(schema, dict):
        return []
    if "$ref" in schema:
        target = resolve_ref(root_schema, schema["$ref"])
        if target is None:
            return [f"{path}: unresolved $ref {schema['$ref']}"]
        errs = validate_schema(val, target, root_schema, path)
        if len(schema) == 1:
            return errs
    else:
        errs = []

    if "const" in schema:
        if val != schema["const"] or type(val) is not type(schema["const"]):
            errs.append(f"{path}: expected const {schema['const']!r}, got {val!r}")
    if "enum" in schema:
        if not any(val == e and type(val) is type(e) for e in schema["enum"]):
            errs.append(f"{path}: value {val!r} not in enum {schema['enum']!r}")
    if "type" in schema:
        t = schema["type"]
        allowed = [t] if isinstance(t, str) else list(t)
        if not any(type_matches(val, tn) for tn in allowed):
            errs.append(f"{path}: expected type {t!r}, got {type(val).__name__}")
            return errs

    if isinstance(val, str):
        if "minLength" in schema and len(val) < schema["minLength"]:
            errs.append(f"{path}: length {len(val)} < minLength {schema['minLength']}")
        if "maxLength" in schema and len(val) > schema["maxLength"]:
            errs.append(f"{path}: length {len(val)} > maxLength {schema['maxLength']}")
        if "pattern" in schema and re.search(schema["pattern"], val) is None:
            errs.append(f"{path}: {val!r} does not match pattern {schema['pattern']!r}")

    if isinstance(val, (int, float)) and not isinstance(val, bool):
        if "minimum" in schema and val < schema["minimum"]:
            errs.append(f"{path}: {val} < minimum {schema['minimum']}")
        if "maximum" in schema and val > schema["maximum"]:
            errs.append(f"{path}: {val} > maximum {schema['maximum']}")

    if isinstance(val, list):
        if "minItems" in schema and len(val) < schema["minItems"]:
            errs.append(f"{path}: len {len(val)} < minItems {schema['minItems']}")
        if "maxItems" in schema and len(val) > schema["maxItems"]:
            errs.append(f"{path}: len {len(val)} > maxItems {schema['maxItems']}")
        if "items" in schema and isinstance(schema["items"], dict):
            for idx, item in enumerate(val):
                errs.extend(validate_schema(item, schema["items"], root_schema, f"{path}[{idx}]"))
        if "contains" in schema and isinstance(schema["contains"], dict):
            if not any(not validate_schema(item, schema["contains"], root_schema, f"{path}[{i}]") for i, item in enumerate(val)):
                errs.append(f"{path}: no item satisfied contains schema")

    if isinstance(val, dict):
        for req in schema.get("required", []):
            if req not in val:
                errs.append(f"{path}: missing required key {req!r}")
        props = schema.get("properties", {})
        pat_props = schema.get("patternProperties", {})
        addl = schema.get("additionalProperties", True)
        for k, v in val.items():
            matched = False
            if k in props:
                matched = True
                errs.extend(validate_schema(v, props[k], root_schema, f"{path}.{k}"))
            for pat, pschema in pat_props.items():
                if re.search(pat, k) is not None:
                    matched = True
                    errs.extend(validate_schema(v, pschema, root_schema, f"{path}.{k}"))
            if not matched:
                if addl is False:
                    errs.append(f"{path}: unexpected additional property {k!r}")
                elif isinstance(addl, dict):
                    errs.extend(validate_schema(v, addl, root_schema, f"{path}.{k}"))

    if "allOf" in schema:
        for sub in schema["allOf"]:
            errs.extend(validate_schema(val, sub, root_schema, path))
    if "anyOf" in schema:
        if not any(not validate_schema(val, sub, root_schema, path) for sub in schema["anyOf"]):
            errs.append(f"{path}: failed all anyOf branches")
    if "oneOf" in schema:
        hits = sum(1 for sub in schema["oneOf"] if not validate_schema(val, sub, root_schema, path))
        if hits != 1:
            errs.append(f"{path}: matched {hits} oneOf branches (expected 1)")
    if "not" in schema:
        if not validate_schema(val, schema["not"], root_schema, path):
            errs.append(f"{path}: matched forbidden `not` schema")
    if "if" in schema:
        if not validate_schema(val, schema["if"], root_schema, path):
            if "then" in schema:
                errs.extend(validate_schema(val, schema["then"], root_schema, path))
        else:
            if "else" in schema:
                errs.extend(validate_schema(val, schema["else"], root_schema, path))

    return errs

reg_dir = clone / "registries"
schemas_reg_dir = clone / "schemas" / "registries"

schema_errors = []
empty_errors = []
fmt_id_schema_errors = []
fmt_ext_count_errors = []
parsed_docs = {}
validated_count = 0

if not reg_dir.is_dir():
    schema_errors.append("registries/ directory missing")
else:
    toml_files = sorted(p for p in reg_dir.iterdir() if p.is_file() and p.suffix == ".toml")
    for tf in toml_files:
        stem = tf.stem
        sp = schemas_reg_dir / f"{stem}.schema.json"
        if not sp.is_file():
            schema_errors.append(f"registries/{tf.name}: missing schema schemas/registries/{stem}.schema.json")
            continue
        try:
            schema_doc = json.loads(sp.read_text(encoding="utf-8"))
        except Exception as exc:
            schema_errors.append(f"schemas/registries/{stem}.schema.json: invalid JSON ({exc})")
            continue
        try:
            doc = tomllib.loads(tf.read_text(encoding="utf-8"))
        except Exception as exc:
            schema_errors.append(f"registries/{tf.name}: TOML parse error ({exc})")
            continue
        parsed_docs[tf.name] = doc
        verrs = validate_schema(doc, schema_doc, schema_doc, f"registries/{tf.name}")
        if verrs:
            schema_errors.append(verrs[0])
        else:
            validated_count += 1

    # cards/ directory + card.schema.json
    card_schema_p = schemas_reg_dir / "card.schema.json"
    if not card_schema_p.is_file():
        schema_errors.append("registries/cards/: missing schemas/registries/card.schema.json")
    else:
        try:
            card_schema = json.loads(card_schema_p.read_text(encoding="utf-8"))
        except Exception as exc:
            schema_errors.append(f"schemas/registries/card.schema.json: invalid JSON ({exc})")
            card_schema = None
        cards_dir = reg_dir / "cards"
        if cards_dir.is_dir() and card_schema is not None:
            card_files = sorted(
                p for p in cards_dir.iterdir() if p.is_file() and p.suffix in (".toml", ".fcard")
            )
            for cf in card_files:
                try:
                    cdoc = tomllib.loads(cf.read_text(encoding="utf-8"))
                except Exception as exc:
                    schema_errors.append(f"registries/cards/{cf.name}: TOML parse error ({exc})")
                    continue
                verrs = validate_schema(cdoc, card_schema, card_schema, f"registries/cards/{cf.name}")
                if verrs:
                    schema_errors.append(verrs[0])
            if mode == "first" and card_files:
                empty_errors.append(f"registries/cards/: expected 0 card files in first commit, found {len(card_files)}")

# Check eight empty registries in first-commit mode
first_empty_tomls = [
    "verbs.toml",
    "refusals.toml",
    "kernels.toml",
    "gates.toml",
    "upstream.toml",
    "lifts.toml",
    "capsules.toml",
]
if mode == "first":
    for fname in first_empty_tomls:
        fpath = reg_dir / fname
        if not fpath.is_file():
            empty_errors.append(f"registries/{fname}: missing")
            continue
        doc = parsed_docs.get(fname)
        if doc is None:
            try:
                doc = tomllib.loads(fpath.read_text(encoding="utf-8"))
            except Exception as exc:
                empty_errors.append(f"registries/{fname}: unparseable ({exc})")
                continue
        row_total = 0
        for k, v in doc.items():
            if k == "schema":
                continue
            if isinstance(v, list):
                row_total += len(v)
        if row_total != 0:
            empty_errors.append(f"registries/{fname}: expected 0 rows in first commit, found {row_total}")

# Check formats.toml invariants (both modes)
REQUIRED_FMT_IDS = [
    "FMT-FBOT",
    "FMT-FMODULE",
    "FMT-FCARD",
    "FMT-FSCENE",
    "FMT-FCLEAR",
    "FMT-FTASK",
    "FMT-FMISSION",
    "FMT-FENERGY",
    "FMT-FSKILL",
    "FMT-FMCW",
    "FMT-FRUN",
    "FMT-FBUILD",
    "FMT-FROBCERT",
]
fmt_doc = parsed_docs.get("formats.toml")
if fmt_doc is None and (reg_dir / "formats.toml").is_file():
    try:
        fmt_doc = tomllib.loads((reg_dir / "formats.toml").read_text(encoding="utf-8"))
    except Exception:
        fmt_doc = None

fmt_count = 0
if not isinstance(fmt_doc, dict) or not isinstance(fmt_doc.get("format"), list):
    fmt_id_schema_errors.append("registries/formats.toml: missing [[format]] array")
    fmt_ext_count_errors.append("registries/formats.toml: missing [[format]] array")
else:
    rows = fmt_doc["format"]
    fmt_count = len(rows)
    seen_ids = set()
    seen_schemas = set()
    seen_exts = {}
    schema_re = re.compile(r"^frob\.[a-z][a-z0-9.]*/1$")
    for idx, row in enumerate(rows):
        if not isinstance(row, dict):
            fmt_id_schema_errors.append(f"format[{idx}]: not a table")
            continue
        fid = str(row.get("id", ""))
        ext = str(row.get("extension", ""))
        fschema = str(row.get("schema", ""))
        owner = str(row.get("owner", "")).strip()
        if not fid or fid in seen_ids:
            fmt_id_schema_errors.append(f"format[{idx}]: missing or duplicate id `{fid}`")
        else:
            seen_ids.add(fid)
        if not schema_re.match(fschema) or fschema in seen_schemas:
            fmt_id_schema_errors.append(f"format[{idx}] ({fid}): invalid or duplicate schema `{fschema}` (expected unique frob.<name>/1)")
        else:
            seen_schemas.add(fschema)
        if not owner:
            fmt_id_schema_errors.append(f"format[{idx}] ({fid}): empty owner crate")
        if not ext:
            fmt_ext_count_errors.append(f"format[{idx}] ({fid}): empty extension")
        elif ext in seen_exts:
            fmt_ext_count_errors.append(f"format[{idx}] ({fid}): duplicate extension `{ext}` (already owned by {seen_exts[ext]})")
        else:
            seen_exts[ext] = fid

    missing_req_ids = [rid for rid in REQUIRED_FMT_IDS if rid not in seen_ids]
    if missing_req_ids:
        fmt_id_schema_errors.append(f"formats.toml missing required IDs: {', '.join(missing_req_ids)}")
    if mode == "first" and fmt_count != 13:
        fmt_ext_count_errors.append(f"formats.toml expected 13 rows in first commit, found {fmt_count}")

out_p.write_text(
    json.dumps(
        {
            "validated_count": validated_count,
            "format_count": fmt_count,
            "schema_errors": schema_errors,
            "empty_errors": empty_errors,
            "fmt_id_schema_errors": fmt_id_schema_errors,
            "fmt_ext_count_errors": fmt_ext_count_errors,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
PY

S3_SCHEMA="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["schema_errors"] else "; ".join(d["schema_errors"]))' "${STEP3_OUT}")"
e2e_assert_eq "step3_registries_schema_valid" "${S3_SCHEMA}" "ok"
[ "${S3_SCHEMA}" = "ok" ] || note_fail

if [ "${MODE}" = "first" ]; then
  S3_EMPTY="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["empty_errors"] else "; ".join(d["empty_errors"]))' "${STEP3_OUT}")"
  e2e_assert_eq "step3_registries_eight_empty" "${S3_EMPTY}" "ok"
  [ "${S3_EMPTY}" = "ok" ] || note_fail
fi

S3_FMT_IDS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["fmt_id_schema_errors"] else "; ".join(d["fmt_id_schema_errors"]))' "${STEP3_OUT}")"
e2e_assert_eq "step3_formats_ids_and_schemas" "${S3_FMT_IDS}" "ok"
[ "${S3_FMT_IDS}" = "ok" ] || note_fail

S3_FMT_EXT="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["fmt_ext_count_errors"] else "; ".join(d["fmt_ext_count_errors"]))' "${STEP3_OUT}")"
e2e_assert_eq "step3_formats_duplicate-format-extension" "${S3_FMT_EXT}" "ok"
[ "${S3_FMT_EXT}" = "ok" ] || note_fail

S3_FMT_COUNT="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["format_count"])' "${STEP3_OUT}")"
e2e_metric "step3_format_rows" "${S3_FMT_COUNT}" "count"

# ---------------------------------------------------------------------------
# Step 4: Toolchain
# ---------------------------------------------------------------------------
e2e_step "step_4_toolchain" "Verify exactly one governing rust-toolchain.toml matching franken-stack.lock"

STEP4_OUT="${E2E_WORK_DIR}/step4.json"
python3 - "${CLONE_DIR}" "${STEP4_OUT}" <<'PY'
import json, os, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
out_p = Path(sys.argv[2])

skip_dirs = {".git", "target", ".beads", "node_modules", "planning"}
found = []
for root, dirs, files in os.walk(clone):
    rel_dir = Path(root).relative_to(clone)
    if "fixtures" in rel_dir.parts:
        dirs[:] = []
        continue
    dirs[:] = sorted(d for d in dirs if d not in skip_dirs and d != "fixtures")
    for fname in ("rust-toolchain.toml", "rust-toolchain"):
        if fname in files:
            found.append((Path(root) / fname).relative_to(clone).as_posix())

found.sort()
allowed = {"rust-toolchain.toml"}
cert_exists = (clone / "crates" / "frob-cert" / "Cargo.toml").is_file()
if cert_exists:
    allowed.add("crates/frob-cert/rust-toolchain.toml")

# Worker fallback exception only if recorded in franken-stack.lock
lock_p = clone / "franken-stack.lock"
lock_channel = ""
lock_err = ""
if lock_p.is_file():
    try:
        lock_doc = tomllib.loads(lock_p.read_text(encoding="utf-8"))
        lock_channel = str(lock_doc.get("toolchain", ""))
        wt = lock_doc.get("worker")
        if isinstance(wt, dict) and wt.get("toolchain"):
            allowed.add("worker/rust-toolchain.toml")
        for row in lock_doc.get("foundation", []):
            if isinstance(row, dict) and row.get("worker_fallback") is True:
                allowed.add("worker/rust-toolchain.toml")
    except Exception as exc:
        lock_err = f"franken-stack.lock parse error: {exc}"
else:
    lock_err = "franken-stack.lock missing"

file_errors = []
if "rust-toolchain.toml" not in found:
    file_errors.append("missing root rust-toolchain.toml")
unexpected = [p for p in found if p not in allowed]
if unexpected:
    file_errors.append(f"unexpected toolchain file(s): {', '.join(unexpected)}")

channel_errors = []
root_tc_p = clone / "rust-toolchain.toml"
root_channel = ""
if root_tc_p.is_file():
    try:
        tc_doc = tomllib.loads(root_tc_p.read_text(encoding="utf-8"))
        root_channel = str(tc_doc.get("toolchain", {}).get("channel", ""))
    except Exception as exc:
        channel_errors.append(f"rust-toolchain.toml parse error: {exc}")
else:
    channel_errors.append("root rust-toolchain.toml missing")

if lock_err:
    channel_errors.append(lock_err)
elif not root_channel:
    channel_errors.append("rust-toolchain.toml missing [toolchain].channel")
elif root_channel != lock_channel:
    channel_errors.append(
        f"rust-toolchain.toml channel `{root_channel}` != franken-stack.lock toolchain `{lock_channel}`"
    )

# If frob-cert's toolchain file exists, verify it is a stable channel (not nightly).
cert_tc_p = clone / "crates" / "frob-cert" / "rust-toolchain.toml"
if cert_exists and cert_tc_p.is_file():
    try:
        ctc = tomllib.loads(cert_tc_p.read_text(encoding="utf-8"))
        cch = str(ctc.get("toolchain", {}).get("channel", ""))
        if not cch or cch.startswith("nightly"):
            channel_errors.append(f"crates/frob-cert/rust-toolchain.toml must pin stable channel, got `{cch}`")
    except Exception as exc:
        channel_errors.append(f"crates/frob-cert/rust-toolchain.toml parse error: {exc}")

out_p.write_text(
    json.dumps(
        {
            "found": found,
            "root_channel": root_channel,
            "lock_channel": lock_channel,
            "file_errors": file_errors,
            "channel_errors": channel_errors,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
PY

S4_FILES="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["file_errors"] else "; ".join(d["file_errors"]))' "${STEP4_OUT}")"
e2e_assert_eq "step4_toolchain_annex-toolchain-file" "${S4_FILES}" "ok"
[ "${S4_FILES}" = "ok" ] || note_fail

S4_CHAN="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["channel_errors"] else "; ".join(d["channel_errors"]))' "${STEP4_OUT}")"
e2e_assert_eq "step4_toolchain_channel_matches_lock" "${S4_CHAN}" "ok"
[ "${S4_CHAN}" = "ok" ] || note_fail

# ---------------------------------------------------------------------------
# Step 5: Lock and receipt
# ---------------------------------------------------------------------------
e2e_step "step_5_lock_and_receipt" "Verify franken-stack.lock, receipt BLAKE3 names, lock/tree agreement, and zero machine paths"

# Probe whether the cloned commit provides xtask checks (call those that exist,
# skip by name those that do not yet).
XTASK_AVAILABLE_CHECKS=""
XTASK_TARGET_DIR="${TARGET_BASE}/xtask"
if [ -f "${CLONE_DIR}/crates/xtask/Cargo.toml" ] && [ -f "${CLONE_DIR}/Cargo.toml" ]; then
  e2e_cmd --name "xtask_discover_checks" --expect-exit "0,3,101" -- \
    env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
    cargo run --manifest-path "${CLONE_DIR}/Cargo.toml" --offline -j 1 -p xtask
  if [ "${E2E_LAST_EXIT}" = "0" ] || [ "${E2E_LAST_EXIT}" = "3" ]; then
    XTASK_AVAILABLE_CHECKS="$(python3 - "${E2E_LAST_STDOUT}" "${E2E_LAST_STDERR}" <<'PY'
import re, sys
from pathlib import Path
text = ""
for p in sys.argv[1:3]:
    if p and Path(p).is_file():
        text += Path(p).read_text(encoding="utf-8", errors="replace") + "\n"
found = sorted(set(re.findall(r"\b(check-(?:layers|deps|unsafe|stack|contracts|generated))\b", text)))
print(" ".join(found))
PY
)"
  fi
fi

has_xtask_check() {
  case " ${XTASK_AVAILABLE_CHECKS} " in
    *" $1 "*) return 0 ;;
    *) return 1 ;;
  esac
}

for chk_name in check-layers check-deps check-unsafe check-stack check-contracts check-generated; do
  chk_slug="$(printf '%s' "${chk_name}" | tr '-' '_')"
  if has_xtask_check "${chk_name}"; then
    e2e_metric "xtask_${chk_slug}_present" "1" "present"
  else
    e2e_metric "xtask_${chk_slug}_present" "0" "skipped_absent"
  fi
done

LOCK_B3=""
if [ -f "${CLONE_DIR}/franken-stack.lock" ]; then
  LOCK_B3="$(hash_b3 "${CLONE_DIR}/franken-stack.lock")"
  e2e_metric "digest_stack_lock_blake3" "1" "b3:${LOCK_B3}"
fi

XTASK_STACK_JSON=""
if has_xtask_check "check-stack"; then
  e2e_cmd --name "xtask_check_stack" --expect-exit "0,2" -- \
    env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
    cargo run --manifest-path "${CLONE_DIR}/Cargo.toml" --offline -j 1 -p xtask -- \
      check-stack --root "${CLONE_DIR}" --json
  XTASK_STACK_JSON="${E2E_LAST_STDOUT}"
fi

STEP5_OUT="${E2E_WORK_DIR}/step5.json"
python3 - "${CLONE_DIR}" "${B3SUM_BIN}" "${XTASK_STACK_JSON}" "${STEP5_OUT}" <<'PY'
import json, os, re, subprocess, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
b3bin = sys.argv[2]
xtask_json_p = sys.argv[3]
out_p = Path(sys.argv[4])

HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")
WS_REL = {
    "core": ".",
    "apps": "apps",
    "bindings": "bindings/frob-gymnasium",
    "derive": "derive/frob-derive",
    "spark-cuda": "spark/frob-spark-cuda",
    "spark-metal": "spark/frob-spark-metal",
    "worker": "worker",
}

def b3_file(path: Path) -> str:
    proc = subprocess.run([b3bin, "--no-names", str(path)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
    return proc.stdout.decode("ascii").strip()

lock_errors = []
receipt_exist_errors = []
receipt_hash_errors = []
machine_path_errors = []
agreement_errors = []
newest_b3 = ""

lock_p = clone / "franken-stack.lock"
lock_doc = None
if not lock_p.is_file():
    lock_errors.append("franken-stack.lock missing")
else:
    try:
        lock_doc = tomllib.loads(lock_p.read_text(encoding="utf-8"))
    except Exception as exc:
        lock_errors.append(f"franken-stack.lock parse error: {exc}")

foundations = []
lock_toolchain = ""
if isinstance(lock_doc, dict):
    lock_toolchain = str(lock_doc.get("toolchain", "")).strip()
    if not lock_toolchain:
        lock_errors.append("franken-stack.lock missing top-level toolchain")
    raw_f = lock_doc.get("foundation", [])
    if not isinstance(raw_f, list) or not raw_f:
        lock_errors.append("franken-stack.lock missing [[foundation]] rows")
    else:
        seen_ids = set()
        for idx, row in enumerate(raw_f):
            if not isinstance(row, dict):
                lock_errors.append(f"foundation[{idx}]: not a table")
                continue
            fid = str(row.get("id", "")).strip()
            fname = str(row.get("name", "")).strip()
            if not fid or not fname:
                lock_errors.append(f"foundation[{idx}]: missing id or name")
            elif fid in seen_ids:
                lock_errors.append(f"foundation[{idx}]: duplicate id {fid}")
            else:
                seen_ids.add(fid)
            foundations.append(row)

rdir = clone / "receipts"
receipt_files = sorted(
    p for p in rdir.iterdir() if p.is_file() and p.suffix == ".json" and HEX64.match(p.stem)
) if rdir.is_dir() else []

if not receipt_files:
    receipt_exist_errors.append("receipts/ contains no <64-hex>.json receipt")

# Every `receipt` hash named in franken-stack.lock exists under receipts/
named_receipts = []
for row in foundations:
    rh = row.get("receipt")
    if rh is None or rh == "":
        continue
    rh_s = str(rh).strip()
    if not HEX64.match(rh_s):
        receipt_exist_errors.append(f"foundation {row.get('id')}: invalid receipt hash `{rh_s}`")
    else:
        named_receipts.append(rh_s)
        if not (rdir / f"{rh_s}.json").is_file():
            receipt_exist_errors.append(f"receipts/{rh_s}.json named by {row.get('id')} is missing")

# Check each receipt file's name equals BLAKE3 of its bytes, and scan for machine paths.
home = os.path.expanduser("~")
def scan_paths(obj, where):
    if isinstance(obj, str):
        if obj.startswith("/"):
            machine_path_errors.append(f"{where}: absolute path `{obj}`")
            return
        for bad in ("/Users/", "/home/", "/private/", "C:\\"):
            if bad in obj:
                machine_path_errors.append(f"{where}: machine path `{bad}` in `{obj}`")
                return
        if home and len(home) > 1 and home in obj:
            machine_path_errors.append(f"{where}: home directory path leaked")
    elif isinstance(obj, list):
        for i, item in enumerate(obj):
            scan_paths(item, f"{where}[{i}]")
    elif isinstance(obj, dict):
        for k, v in obj.items():
            scan_paths(k, f"{where}.<key>")
            scan_paths(v, f"{where}.{k}")

parsed_receipts = {}
for rf in receipt_files:
    actual = b3_file(rf)
    if actual != rf.stem:
        receipt_hash_errors.append(f"receipts/{rf.name}: blake3={actual} != filename={rf.stem}")
    try:
        robj = json.loads(rf.read_text(encoding="utf-8"))
        parsed_receipts[rf.stem] = robj
        scan_paths(robj, f"receipts/{rf.name}")
    except Exception as exc:
        receipt_hash_errors.append(f"receipts/{rf.name}: invalid JSON ({exc})")

# Determine newest receipt
newest_stem = None
head_p = rdir / "HEAD"
if head_p.is_file():
    h = head_p.read_text(encoding="utf-8").strip()
    if HEX64.match(h):
        newest_stem = h
        if not (rdir / f"{h}.json").is_file():
            receipt_exist_errors.append(f"receipts/HEAD points to missing receipts/{h}.json")
    else:
        receipt_exist_errors.append(f"receipts/HEAD contains invalid hash `{h}`")
elif len(receipt_files) == 1:
    newest_stem = receipt_files[0].stem
elif receipt_files:
    # Prefer a receipt named in franken-stack.lock, else last sorted
    lock_named_present = [h for h in named_receipts if (rdir / f"{h}.json").is_file()]
    newest_stem = lock_named_present[-1] if lock_named_present else receipt_files[-1].stem

if newest_stem and (rdir / f"{newest_stem}.json").is_file():
    newest_b3 = b3_file(rdir / f"{newest_stem}.json")

# Direct comparison of newest receipt against franken-stack.lock and tree
if newest_stem and newest_stem in parsed_receipts and isinstance(lock_doc, dict):
    rec = parsed_receipts[newest_stem]
    rec_tc = str(rec.get("toolchain", ""))
    if rec_tc != lock_toolchain:
        agreement_errors.append(f"newest receipt toolchain `{rec_tc}` != lock toolchain `{lock_toolchain}`")

    by_fd = {str(r.get("id", "")): r for r in foundations}
    rec_sibs = rec.get("siblings", [])
    seen_rec_fds = set()
    if isinstance(rec_sibs, list):
        for s in rec_sibs:
            if not isinstance(s, dict):
                continue
            fd = str(s.get("fd", ""))
            srev = str(s.get("rev", ""))
            seen_rec_fds.add(fd)
            row = by_fd.get(fd)
            if row is None:
                agreement_errors.append(f"receipt sibling `{fd}` not found in franken-stack.lock")
                continue
            lrev = str(row.get("git_rev", ""))
            if lrev and lrev != "unadmitted" and srev != lrev:
                agreement_errors.append(f"sibling `{fd}` rev mismatch: receipt `{srev}` != lock `{lrev}`")

    # Every lock foundation with a path, 40-hex git_rev, and receipt == newest_stem must appear in siblings
    for row in foundations:
        fd = str(row.get("id", ""))
        lrev = str(row.get("git_rev", ""))
        lpath = str(row.get("path", ""))
        lrec = str(row.get("receipt", ""))
        if lpath and HEX40.match(lrev) and lrec == newest_stem and fd not in seen_rec_fds:
            agreement_errors.append(f"foundation `{fd}` references newest receipt but is absent from receipt.siblings")

    # Lockfile hashes agree with tree
    rec_lfs = rec.get("lockfiles", [])
    seen_ws = set()
    if isinstance(rec_lfs, list):
        for lf in rec_lfs:
            if not isinstance(lf, dict):
                continue
            ws = str(lf.get("workspace", ""))
            expected_h = str(lf.get("hash", ""))
            seen_ws.add(ws)
            rel = WS_REL.get(ws, ws)
            lf_path = clone / "Cargo.lock" if rel == "." else clone / rel / "Cargo.lock"
            if not lf_path.is_file():
                agreement_errors.append(f"receipt lockfile for workspace `{ws}` missing at {lf_path.relative_to(clone)}")
            else:
                actual_h = b3_file(lf_path)
                if actual_h != expected_h:
                    agreement_errors.append(
                        f"workspace `{ws}` Cargo.lock blake3 `{actual_h}` != receipt `{expected_h}`"
                    )
    if (clone / "Cargo.lock").is_file() and "core" not in seen_ws:
        agreement_errors.append("core Cargo.lock exists in tree but is absent from newest receipt lockfiles")

# Fold in `cargo xtask check-stack --json` when present (excluding Step 4 / Step 6 rules
# which are checked and failed by their own steps so planted defects stay step-isolated).
if xtask_json_p and Path(xtask_json_p).is_file():
    raw_xt = Path(xtask_json_p).read_text(encoding="utf-8", errors="replace").strip()
    if raw_xt:
        try:
            xt_doc = json.loads(raw_xt)
            for finding in xt_doc.get("findings", []):
                rule = str(finding.get("rule", ""))
                msg = str(finding.get("message", ""))
                if rule in ("stack.toolchain-file", "stack.patch", "stack.profile", "stack.env"):
                    continue
                if rule == "stack.receipt-hash":
                    if not receipt_hash_errors:
                        receipt_hash_errors.append(f"check-stack ({rule}): {msg}")
                elif rule == "stack.receipt-missing":
                    if not receipt_exist_errors:
                        receipt_exist_errors.append(f"check-stack ({rule}): {msg}")
                elif rule == "stack.lock-schema":
                    if not lock_errors:
                        lock_errors.append(f"check-stack ({rule}): {msg}")
                else:
                    # Avoid duplicating identical mismatch already captured directly
                    agreement_errors.append(f"check-stack ({rule}): {msg}")
        except Exception as exc:
            agreement_errors.append(f"check-stack JSON parse error: {exc}")

# Deduplicate agreement_errors preserving order
dedup_agree = []
for e in agreement_errors:
    if e not in dedup_agree:
        dedup_agree.append(e)

out_p.write_text(
    json.dumps(
        {
            "newest_b3": newest_b3,
            "lock_errors": lock_errors,
            "receipt_exist_errors": receipt_exist_errors,
            "receipt_hash_errors": receipt_hash_errors,
            "machine_path_errors": machine_path_errors,
            "agreement_errors": dedup_agree,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
PY

NEWEST_REC_B3="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["newest_b3"])' "${STEP5_OUT}")"
if [ -n "${NEWEST_REC_B3}" ]; then
  e2e_metric "digest_newest_receipt_blake3" "1" "b3:${NEWEST_REC_B3}"
fi

S5_LOCK="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["lock_errors"] else "; ".join(d["lock_errors"]))' "${STEP5_OUT}")"
e2e_assert_eq "step5_lock_parses" "${S5_LOCK}" "ok"
[ "${S5_LOCK}" = "ok" ] || note_fail

S5_EX="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["receipt_exist_errors"] else "; ".join(d["receipt_exist_errors"]))' "${STEP5_OUT}")"
e2e_assert_eq "step5_lock_receipts_exist" "${S5_EX}" "ok"
[ "${S5_EX}" = "ok" ] || note_fail

S5_HASH="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["receipt_hash_errors"] else "; ".join(d["receipt_hash_errors"]))' "${STEP5_OUT}")"
e2e_assert_eq "step5_receipt_hash_edited-receipt-bytes" "${S5_HASH}" "ok"
[ "${S5_HASH}" = "ok" ] || note_fail

S5_PATHS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["machine_path_errors"] else "; ".join(d["machine_path_errors"]))' "${STEP5_OUT}")"
e2e_assert_eq "step5_receipt_no_machine_paths" "${S5_PATHS}" "ok"
[ "${S5_PATHS}" = "ok" ] || note_fail

S5_AGREE="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["agreement_errors"] else "; ".join(d["agreement_errors"]))' "${STEP5_OUT}")"
e2e_assert_eq "step5_receipt_matches_lock_and_tree" "${S5_AGREE}" "ok"
[ "${S5_AGREE}" = "ok" ] || note_fail

# ---------------------------------------------------------------------------
# Step 6: Build identity and patches
# ---------------------------------------------------------------------------
e2e_step "step_6_build_identity_and_patches" "Verify .cargo/config.toml build identity, root asupersync patch, and fsqlite profile overrides"

STEP6_OUT="${E2E_WORK_DIR}/step6.json"
python3 - "${CLONE_DIR}" "${STEP6_OUT}" <<'PY'
import json, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
out_p = Path(sys.argv[2])

env_err = ""
patch_err = ""
prof_err = ""

cfg_p = clone / ".cargo" / "config.toml"
cfg_env = {}
if not cfg_p.is_file():
    env_err = ".cargo/config.toml missing"
else:
    try:
        cfg = tomllib.loads(cfg_p.read_text(encoding="utf-8"))
        raw_env = cfg.get("env", {})
        if isinstance(raw_env, dict):
            for k, v in raw_env.items():
                cfg_env[k] = str(v.get("value", "")) if isinstance(v, dict) else str(v)
        salt = cfg_env.get("FRANKENSIM_DEPGRAPH_SALT", "").strip()
        rcpt = cfg_env.get("FRANKENSIM_DEPGRAPH_RECEIPT", "").strip()
        if not salt and not rcpt:
            env_err = ".cargo/config.toml [env] missing FRANKENSIM_DEPGRAPH_SALT (or FRANKENSIM_DEPGRAPH_RECEIPT)"
    except Exception as exc:
        env_err = f".cargo/config.toml parse error: {exc}"

# Cross-check against FD-FSIM build_env in franken-stack.lock when present
lock_p = clone / "franken-stack.lock"
if not env_err and lock_p.is_file():
    try:
        ldoc = tomllib.loads(lock_p.read_text(encoding="utf-8"))
        for row in ldoc.get("foundation", []):
            if isinstance(row, dict) and row.get("id") == "FD-FSIM":
                benv = row.get("build_env", {})
                if isinstance(benv, dict):
                    for k, expected in benv.items():
                        if cfg_env.get(k) != str(expected):
                            env_err = f".cargo/config.toml [env].{k}={cfg_env.get(k)!r} != franken-stack.lock {expected!r}"
    except Exception:
        pass

cargo_p = clone / "Cargo.toml"
if not cargo_p.is_file():
    patch_err = "root Cargo.toml missing"
    prof_err = "root Cargo.toml missing"
else:
    try:
        cdoc = tomllib.loads(cargo_p.read_text(encoding="utf-8"))
        patch_cio = cdoc.get("patch", {}).get("crates-io", {})
        if not isinstance(patch_cio, dict) or "asupersync" not in patch_cio:
            patch_err = "root Cargo.toml missing [patch.crates-io].asupersync"
        else:
            asup = patch_cio["asupersync"]
            if not isinstance(asup, dict) or not (asup.get("path") or asup.get("git")):
                patch_err = "root Cargo.toml [patch.crates-io].asupersync missing path or git"

        dev_pkgs = cdoc.get("profile", {}).get("dev", {}).get("package", {})
        if not isinstance(dev_pkgs, dict):
            prof_err = "root Cargo.toml missing [profile.dev.package] fsqlite* overrides"
        else:
            fsq = {k: v for k, v in dev_pkgs.items() if k == "fsqlite" or k.startswith("fsqlite-")}
            if "fsqlite" not in fsq:
                prof_err = "root Cargo.toml missing [profile.dev.package.fsqlite] opt-level = 1"
            else:
                bad = [
                    k for k, v in sorted(fsq.items())
                    if not isinstance(v, dict) or v.get("opt-level") != 1
                ]
                if bad:
                    prof_err = f"root Cargo.toml fsqlite profile overrides without opt-level=1: {', '.join(bad)}"
    except Exception as exc:
        patch_err = f"root Cargo.toml parse error: {exc}"
        prof_err = f"root Cargo.toml parse error: {exc}"

out_p.write_text(
    json.dumps({"env_err": env_err, "patch_err": patch_err, "prof_err": prof_err}, indent=2) + "\n",
    encoding="utf-8",
)
PY

S6_ENV="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["env_err"] else d["env_err"])' "${STEP6_OUT}")"
e2e_assert_eq "step6_build_identity_env" "${S6_ENV}" "ok"
[ "${S6_ENV}" = "ok" ] || note_fail

S6_PATCH="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["patch_err"] else d["patch_err"])' "${STEP6_OUT}")"
e2e_assert_eq "step6_root_asupersync_patch" "${S6_PATCH}" "ok"
[ "${S6_PATCH}" = "ok" ] || note_fail

S6_PROF="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["prof_err"] else d["prof_err"])' "${STEP6_OUT}")"
e2e_assert_eq "step6_root_fsqlite_profiles" "${S6_PROF}" "ok"
[ "${S6_PROF}" = "ok" ] || note_fail

# ---------------------------------------------------------------------------
# Step 7: Status table
# ---------------------------------------------------------------------------
e2e_step "step_7_status_table" "Regenerate README status block from registries/gates.toml and verify byte-identity and honesty"

XTASK_GEN_JSON=""
if has_xtask_check "check-generated"; then
  e2e_cmd --name "xtask_check_generated_status" --expect-exit "0,2" -- \
    env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
    cargo run --manifest-path "${CLONE_DIR}/Cargo.toml" --offline -j 1 -p xtask -- \
      check-generated --root "${CLONE_DIR}" --generator honest-status-table --json
  XTASK_GEN_JSON="${E2E_LAST_STDOUT}"
fi

STEP7_OUT="${E2E_WORK_DIR}/step7.json"
python3 - "${CLONE_DIR}" "${B3SUM_BIN}" "${MODE}" "${XTASK_GEN_JSON}" "${STEP7_OUT}" <<'PY'
import json, subprocess, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
b3bin = sys.argv[2]
mode = sys.argv[3]
xtask_gen_p = sys.argv[4]
out_p = Path(sys.argv[5])

def b3_bytes(data: bytes) -> str:
    proc = subprocess.run([b3bin, "--no-names"], input=data, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
    return proc.stdout.decode("ascii").strip()

MILESTONES = {f"M{i}": i for i in range(9)}
VALID_CLAIMS = {"proof", "known-answer", "parity", "fidelity", "design-comparison", "reality", "performance"}

def escape_cell(s: str) -> str:
    return s.replace("|", "\\|").replace("\n", " ").replace("\r", "")

def pad_cell(s: str, w: int) -> str:
    return s if len(s) >= w else s + (" " * (w - len(s)))

def fmt_recorded(rec: dict) -> str:
    d = str(rec.get("date", "")).strip()
    s = str(rec.get("session", "")).strip()
    if not d and not s:
        return "—"
    if d and s:
        return f"{d} ({s})"
    return d or s

def render_status_table(gates_bytes: bytes):
    doc = tomllib.loads(gates_bytes.decode("utf-8"))
    if doc.get("schema") != "frob.gates/1":
        raise ValueError(f"unexpected gates.toml schema {doc.get('schema')!r}")
    raw_gates = doc.get("gate", [])
    if not isinstance(raw_gates, list):
        raise ValueError("gates.toml gate must be array")
    active = []
    seen = set()
    for g in raw_gates:
        if not isinstance(g, dict):
            raise ValueError("gate entry not a table")
        gid = str(g.get("id", ""))
        if gid in seen:
            raise ValueError(f"duplicate gate id `{gid}`")
        seen.add(gid)
        if g.get("kind") == "kill":
            continue
        claim = str(g.get("claim", ""))
        if claim not in VALID_CLAIMS:
            raise ValueError(f"gate `{gid}` invalid claim `{claim}`")
        active.append(g)

    active.sort(key=lambda g: str(g["id"]).encode("utf-8"))
    counts = {"open": 0, "pass": 0, "fail": 0, "killed": 0, "blocked": 0, "no-claim": 0, "superseded": 0}
    rows = []
    for g in active:
        parts = g.get("part", []) or []
        latest = None
        for idx, p in enumerate(parts):
            key = (MILESTONES.get(str(p.get("milestone", "")), 0), idx)
            if latest is None or key > latest[0]:
                latest = (key, p)
        if latest is None:
            ms, st, meas, rec, bucket = str(g.get("milestone", "")), "open", "—", "—", "open"
        else:
            p = latest[1]
            ms = str(p.get("milestone", ""))
            pst = str(p.get("status", ""))
            mtrim = str(p.get("measured", "")).strip()
            rmeta = p.get("recorded", {}) if isinstance(p.get("recorded"), dict) else {}
            if pst == "not-run":
                st, meas, rec, bucket = "open", "—", "—", "open"
            elif pst == "pass":
                if not mtrim:
                    raise ValueError(f"gate `{g['id']}` status `pass` with empty measured")
                st, meas, rec, bucket = "pass", mtrim, fmt_recorded(rmeta), "pass"
            elif pst == "fail":
                if not mtrim:
                    st, meas, rec, bucket = "open", "—", "—", "open"
                else:
                    st, meas, rec, bucket = "fail", mtrim, fmt_recorded(rmeta), "fail"
            elif pst == "killed":
                kg = str(p.get("kill_gate", "") or "").strip()
                fb = str(p.get("fallback", "") or "").strip()
                if kg and fb:
                    st = f"killed ({kg}: {fb})"
                else:
                    st = f"killed ({str(p.get('detail', '') or '').strip()})"
                meas, rec, bucket = (mtrim or "—"), fmt_recorded(rmeta), "killed"
            elif pst == "blocked":
                blk = str(p.get("open_question", "") or "").strip() or str(p.get("detail", "") or "").strip()
                st, meas, rec, bucket = f"blocked ({blk})", (mtrim or "—"), fmt_recorded(rmeta), "blocked"
            elif pst == "no-claim":
                st, meas, rec, bucket = "no-claim", (mtrim or "—"), fmt_recorded(rmeta), "no-claim"
            elif pst == "superseded":
                st, meas, rec, bucket = "superseded", (mtrim or "—"), fmt_recorded(rmeta), "superseded"
            else:
                raise ValueError(f"gate `{g['id']}` invalid status `{pst}`")
        counts[bucket] += 1
        rows.append([
            escape_cell(str(g["id"])),
            escape_cell(str(g["claim"])),
            escape_cell(ms),
            escape_cell(st),
            escape_cell(meas),
            escape_cell(rec),
        ])

    headers = ["Gate", "Claim", "Milestone", "Status", "Measured", "Recorded"]
    widths = [16, 17, 9, 10, 12, 12]
    for r in rows:
        for i in range(6):
            if len(r[i]) > widths[i]:
                widths[i] = len(r[i])

    out = []
    out.append("|" + "".join(f" {pad_cell(headers[i], widths[i])} |" for i in range(6)))
    out.append("|" + "".join(f" {'-' * widths[i]} |" for i in range(6)))
    for r in rows:
        out.append("|" + "".join(f" {pad_cell(r[i], widths[i])} |" for i in range(6)))
    out.append("")
    out.append(
        f"Summary: {len(rows)} gates (open: {counts['open']}, pass: {counts['pass']}, "
        f"fail: {counts['fail']}, killed: {counts['killed']}, blocked: {counts['blocked']}, "
        f"no-claim: {counts['no-claim']}, superseded: {counts['superseded']})"
    )
    return "\n".join(out) + "\n", len(rows), counts

block_err = ""
honesty_err = ""
expected_b3 = ""
actual_b3 = ""
gate_row_count = 0

gates_p = clone / "registries" / "gates.toml"
readme_p = clone / "README.md"

if not gates_p.is_file():
    block_err = "registries/gates.toml missing"
elif not readme_p.is_file():
    block_err = "README.md missing"
else:
    try:
        expected_block, gate_row_count, counts = render_status_table(gates_p.read_bytes())
        expected_b3 = b3_bytes(expected_block.encode("utf-8"))
        readme_txt = readme_p.read_text(encoding="utf-8")
        b_mark = "<!-- status:begin -->"
        e_mark = "<!-- status:end -->"
        if readme_txt.count(b_mark) != 1 or readme_txt.count(e_mark) != 1:
            block_err = "README.md missing unique status markers"
        else:
            b_idx = readme_txt.index(b_mark) + len(b_mark)
            if readme_txt[b_idx:].startswith("\n"):
                inner_start = b_idx + 1
            elif readme_txt[b_idx:].startswith("\r\n"):
                inner_start = b_idx + 2
            else:
                inner_start = -1
            e_idx = readme_txt.index(e_mark)
            if inner_start < 0 or e_idx < inner_start:
                block_err = "README.md malformed status markers"
            else:
                actual_block = readme_txt[inner_start:e_idx]
                actual_b3 = b3_bytes(actual_block.encode("utf-8"))
                first_inner_line = readme_txt[:inner_start].count("\n") + 1
                if actual_block != expected_block:
                    al = actual_block.split("\n")
                    el = expected_block.split("\n")
                    diff_off = 0
                    for i in range(max(len(al), len(el))):
                        if (al[i] if i < len(al) else None) != (el[i] if i < len(el) else None):
                            diff_off = i
                            break
                    block_err = f"README.md differs at line {first_inner_line + diff_off} (actual={actual_b3}, expected={expected_b3})"

        # Honesty checks on the generated table in first-commit mode
        if mode == "first":
            if counts["pass"] != 0:
                honesty_err = f"first commit has {counts['pass']} passed gate(s) in status table"
            elif gate_row_count == 0:
                expected_summary = "Summary: 0 gates (open: 0, pass: 0, fail: 0, killed: 0, blocked: 0, no-claim: 0, superseded: 0)"
                exp_lines = expected_block.splitlines()
                if len(exp_lines) != 4 or not exp_lines[0].startswith("| Gate") or exp_lines[3] != expected_summary:
                    honesty_err = "empty status table lacks expected header/zero-row/zero-count shape"
    except Exception as exc:
        block_err = f"status table generation error: {exc}"

if not block_err and xtask_gen_p and Path(xtask_gen_p).is_file():
    raw_g = Path(xtask_gen_p).read_text(encoding="utf-8", errors="replace").strip()
    if raw_g:
        try:
            gdoc = json.loads(raw_g)
            if not gdoc.get("ok", False):
                findings = gdoc.get("findings", [])
                first = findings[0] if findings else {}
                block_err = f"xtask check-generated ({first.get('rule', 'unknown')}): {first.get('message', 'failed')}"
        except Exception as exc:
            block_err = f"xtask check-generated JSON parse error: {exc}"

out_p.write_text(
    json.dumps(
        {
            "expected_b3": expected_b3,
            "actual_b3": actual_b3,
            "gate_row_count": gate_row_count,
            "block_err": block_err,
            "honesty_err": honesty_err,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
PY

STATUS_EXP_B3="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["expected_b3"])' "${STEP7_OUT}")"
STATUS_ACT_B3="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["actual_b3"])' "${STEP7_OUT}")"
if [ -n "${STATUS_EXP_B3}" ]; then
  e2e_metric "digest_status_block_blake3" "1" "b3:${STATUS_EXP_B3}"
fi

S7_BLOCK="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["block_err"] else d["block_err"])' "${STEP7_OUT}")"
if [ "${S7_BLOCK}" = "ok" ] && [ -n "${STATUS_ACT_B3}" ] && [ -n "${STATUS_EXP_B3}" ]; then
  e2e_assert_digest_eq "step7_status_block_hand-edited-status-block" "${STATUS_ACT_B3}" "${STATUS_EXP_B3}"
else
  e2e_assert_eq "step7_status_block_hand-edited-status-block" "${S7_BLOCK}" "ok"
  note_fail
fi

if [ "${MODE}" = "first" ]; then
  S7_HON="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["honesty_err"] else d["honesty_err"])' "${STEP7_OUT}")"
  e2e_assert_eq "step7_status_table_first_commit_honesty" "${S7_HON}" "ok"
  [ "${S7_HON}" = "ok" ] || note_fail
fi

# ---------------------------------------------------------------------------
# Step 8: No uncompiled crate
# ---------------------------------------------------------------------------
e2e_step "step_8_no_uncompiled_crate" "Verify every workspace member compiles under cargo check --locked --offline (skipping fixtures/)"

STEP8_DISCOVERY="${E2E_WORK_DIR}/step8_workspaces.json"
python3 - "${CLONE_DIR}" "${STEP8_DISCOVERY}" <<'PY'
import json, os, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
out_p = Path(sys.argv[2])

skip_dirs = {".git", "target", ".beads", "planning", "node_modules"}
fixtures_manifests = []
workspaces = []

for root, dirs, files in os.walk(clone):
    rel_dir = Path(root).relative_to(clone)
    dirs[:] = sorted(d for d in dirs if d not in skip_dirs)
    if "Cargo.toml" not in files:
        continue
    rel_manifest = (rel_dir / "Cargo.toml").as_posix() if rel_dir != Path(".") else "Cargo.toml"
    if "fixtures" in rel_dir.parts:
        fixtures_manifests.append(rel_manifest)
        continue
    mpath = Path(root) / "Cargo.toml"
    try:
        doc = tomllib.loads(mpath.read_text(encoding="utf-8"))
    except Exception as exc:
        workspaces.append({
            "manifest": rel_manifest,
            "dir": rel_dir.as_posix(),
            "empty_virtual": False,
            "parse_err": str(exc),
        })
        continue
    if "workspace" in doc:
        ws_tbl = doc["workspace"]
        has_pkg = "package" in doc
        members = ws_tbl.get("members", []) if isinstance(ws_tbl, dict) else []
        empty_virtual = False
        if not has_pkg:
            if not members:
                empty_virtual = True
            else:
                # Check if glob patterns match zero member Cargo.toml files
                matched = False
                for pat in members:
                    for cand in Path(root).glob(pat):
                        if (cand / "Cargo.toml").is_file():
                            matched = True
                            break
                    if matched:
                        break
                if not matched:
                    empty_virtual = True
        workspaces.append({
            "manifest": rel_manifest,
            "dir": rel_dir.as_posix(),
            "empty_virtual": empty_virtual,
            "parse_err": "",
        })

fixtures_manifests.sort()
workspaces.sort(key=lambda w: w["manifest"])

out_p.write_text(
    json.dumps({"fixtures": fixtures_manifests, "workspaces": workspaces}, indent=2) + "\n",
    encoding="utf-8",
)
PY

FIXTURE_COUNT="$(python3 -c 'import json,sys; print(len(json.load(open(sys.argv[1]))["fixtures"]))' "${STEP8_DISCOVERY}")"
e2e_metric "step8_fixture_manifests_skipped" "${FIXTURE_COUNT}" "count"
while IFS= read -r fx_rel; do
  [ -n "${fx_rel}" ] || continue
  fx_slug="$(printf '%s' "${fx_rel}" | tr '/.-' '___')"
  e2e_metric "step8_fixture_skipped_${fx_slug}" "1" "${fx_rel}"
done <<EOF
$(python3 -c 'import json,sys; [print(x) for x in json.load(open(sys.argv[1]))["fixtures"]]' "${STEP8_DISCOVERY}")
EOF

# Standing-mode changed-path set (if --state-dir has a valid previous passing commit)
CHANGED_PATHS_FILE="${E2E_WORK_DIR}/step8_changed_paths.txt"
: > "${CHANGED_PATHS_FILE}"
USE_INCREMENTAL="0"
if [ "${MODE}" = "standing" ] && [ -n "${STATE_DIR}" ] && [ -f "${STATE_DIR}/last_pass_commit" ]; then
  PREV_PASS_SHA="$(tr -d '[:space:]' < "${STATE_DIR}/last_pass_commit")"
  if [ -n "${PREV_PASS_SHA}" ] && git --no-optional-locks -C "${CLONE_DIR}" cat-file -e "${PREV_PASS_SHA}^{commit}" 2>/dev/null; then
    git --no-optional-locks -C "${CLONE_DIR}" diff --name-only "${PREV_PASS_SHA}" "${TARGET_SHA}" > "${CHANGED_PATHS_FILE}" || true
    # If root Cargo.toml, Cargo.lock, rust-toolchain.toml, or .cargo/config.toml changed, check all crates.
    if ! grep -Eq '^(Cargo\.toml|Cargo\.lock|rust-toolchain\.toml|\.cargo/config\.toml)$' "${CHANGED_PATHS_FILE}"; then
      USE_INCREMENTAL="1"
    fi
  fi
fi

STEP8_META_ERRORS=""
STEP8_CHECK_ERRORS=""
STEP8_CHECKED_LIST=""
STEP8_CHECKED_COUNT=0

WS_TSV="${E2E_WORK_DIR}/step8_ws.tsv"
python3 - "${STEP8_DISCOVERY}" "${WS_TSV}" <<'PY'
import json, sys
data = json.load(open(sys.argv[1], encoding="utf-8"))
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    for w in data["workspaces"]:
        fh.write(f'{w["manifest"]}\t{w["dir"]}\t{1 if w["empty_virtual"] else 0}\t{w["parse_err"]}\n')
PY

while IFS="$(printf '\t')" read -r ws_manifest ws_rel_dir ws_empty ws_perr; do
  [ -n "${ws_manifest}" ] || continue
  if [ -n "${ws_perr}" ]; then
    STEP8_META_ERRORS="${STEP8_META_ERRORS:+${STEP8_META_ERRORS}; }${ws_manifest}: ${ws_perr}"
    continue
  fi
  if [ "${ws_empty}" = "1" ]; then
    continue
  fi

  if [ "${ws_rel_dir}" = "." ]; then
    ws_abs_dir="${CLONE_DIR}"
    ws_slug="core"
  else
    ws_abs_dir="${CLONE_DIR}/${ws_rel_dir}"
    ws_slug="$(printf '%s' "${ws_rel_dir}" | tr '/.-' '___')"
  fi

  prev_pwd="${PWD}"
  cd "${ws_abs_dir}" || exit 1
  e2e_cmd --name "cargo_metadata_${ws_slug}" --expect-exit "0,101" -- \
    cargo metadata --no-deps --locked --offline --format-version 1
  cd "${prev_pwd}" || exit 1

  if [ "${E2E_LAST_EXIT}" != "0" ]; then
    STEP8_META_ERRORS="${STEP8_META_ERRORS:+${STEP8_META_ERRORS}; }${ws_manifest}: cargo metadata exited ${E2E_LAST_EXIT}"
    continue
  fi

  PKGS_TSV="${E2E_WORK_DIR}/step8_pkgs_${ws_slug}.tsv"
  python3 - "${E2E_LAST_STDOUT}" "${CLONE_DIR}" "${USE_INCREMENTAL}" "${CHANGED_PATHS_FILE}" "${PKGS_TSV}" <<'PY'
import json, sys
from pathlib import Path

meta = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
clone = Path(sys.argv[2]).resolve()
use_inc = sys.argv[3] == "1"
changed = [l.strip() for l in Path(sys.argv[4]).read_text(encoding="utf-8").splitlines() if l.strip()]
out_p = Path(sys.argv[5])

members = set(meta.get("workspace_members", []))
rows = []
for pkg in meta.get("packages", []):
    if members and pkg.get("id") not in members:
        continue
    name = str(pkg.get("name", ""))
    mp = Path(pkg.get("manifest_path", "")).resolve()
    try:
        rel_dir = mp.parent.relative_to(clone).as_posix()
    except ValueError:
        rel_dir = "."
    check_it = True
    if use_inc:
        prefix = "" if rel_dir == "." else rel_dir + "/"
        check_it = any(c.startswith(prefix) for c in changed)
    rows.append((name, rel_dir, 1 if check_it else 0))

rows.sort(key=lambda r: r[0])
with out_p.open("w", encoding="utf-8") as fh:
    for name, rdir, chk in rows:
        fh.write(f"{name}\t{rdir}\t{chk}\n")
PY

  ws_target="${TARGET_BASE}/ws_${ws_slug}"
  mkdir -p "${ws_target}"

  while IFS="$(printf '\t')" read -r pkg_name _pkg_rdir pkg_chk; do
    [ -n "${pkg_name}" ] || continue
    pkg_slug="$(printf '%s' "${pkg_name}" | tr '/.-' '___')"
    if [ "${pkg_chk}" != "1" ]; then
      e2e_metric "step8_crate_unchanged_skipped_${pkg_slug}" "1" "${pkg_name}"
      continue
    fi

    STEP8_CHECKED_COUNT=$((STEP8_CHECKED_COUNT + 1))
    STEP8_CHECKED_LIST="${STEP8_CHECKED_LIST:+${STEP8_CHECKED_LIST},}${pkg_name}"

    prev_pwd="${PWD}"
    cd "${ws_abs_dir}" || exit 1
    e2e_cmd --name "cargo_check_${ws_slug}_${pkg_slug}" --expect-exit "0,101" -- \
      env "CARGO_TARGET_DIR=${ws_target}" \
      cargo check --locked --offline -j 1 -p "${pkg_name}"
    cd "${prev_pwd}" || exit 1

    if [ "${E2E_LAST_EXIT}" != "0" ]; then
      STEP8_CHECK_ERRORS="${STEP8_CHECK_ERRORS:+${STEP8_CHECK_ERRORS}; }${ws_manifest}:${pkg_name} (exit ${E2E_LAST_EXIT})"
    fi
  done < "${PKGS_TSV}"
done < "${WS_TSV}"

e2e_metric "step8_crates_checked" "${STEP8_CHECKED_COUNT}" "count"

if [ -z "${STEP8_META_ERRORS}" ]; then
  e2e_assert_eq "step8_workspace_metadata_ok" "ok" "ok"
else
  e2e_assert_eq "step8_workspace_metadata_ok" "${STEP8_META_ERRORS}" "ok"
  note_fail
fi

if [ -n "${STEP8_CHECK_ERRORS}" ]; then
  e2e_assert_eq "step8_crates_compile_non-compiling-member" "failed: ${STEP8_CHECK_ERRORS}" "0 failures"
  note_fail
elif [ "${STEP8_CHECKED_COUNT}" -eq 0 ]; then
  e2e_assert_eq "step8_crates_compile_non-compiling-member" "0 crates" "0 crates"
else
  e2e_assert_eq "step8_crates_compile_non-compiling-member" "checked: ${STEP8_CHECKED_LIST}" "checked: ${STEP8_CHECKED_LIST}"
fi

# ---------------------------------------------------------------------------
# Step 9: Initial beads
# ---------------------------------------------------------------------------
e2e_step "step_9_initial_beads" "Verify bead store lists P0 upstream beads FD-FMCP-01 and FD-ASUP-01 with sibling, interim posture, and switch-over rule"

STEP9_OUT="${E2E_WORK_DIR}/step9.json"
python3 - "${CLONE_DIR}" "${SANDBOX_DIR}" "${STEP9_OUT}" <<'PY'
import json, sys
from pathlib import Path

clone = Path(sys.argv[1])
sandbox = Path(sys.argv[2])
out_p = Path(sys.argv[3])

jsonl_p = clone / ".beads" / "issues.jsonl"
errors = []
mirrors = {}

if not jsonl_p.is_file():
    errors.append(".beads/issues.jsonl missing")
else:
    records = []
    for ln, raw in enumerate(jsonl_p.read_text(encoding="utf-8", errors="replace").splitlines(), 1):
        if not raw.strip():
            continue
        try:
            records.append(json.loads(raw))
        except Exception as exc:
            errors.append(f".beads/issues.jsonl:{ln}: invalid JSON ({exc})")

    def find_bead(key: str):
        prefix = f"frob-{key.lower()}"
        for rec in records:
            if not isinstance(rec, dict):
                continue
            ext = str(rec.get("external_ref", ""))
            title = str(rec.get("title", ""))
            bid = str(rec.get("id", ""))
            if ext == key or f"[{key}]" in title or bid.startswith(prefix):
                return rec
        return None

    specs = [
        ("FD-FMCP-01", ["fastmcp_rust"]),
        ("FD-ASUP-01", ["asupersync"]),
    ]
    for key, req_siblings in specs:
        rec = find_bead(key)
        if rec is None:
            errors.append(f"missing P0 upstream bead `{key}` in .beads/issues.jsonl")
            continue
        prio = rec.get("priority")
        if prio != 0:
            errors.append(f"`{key}` priority is {prio!r}, expected 0 (P0)")
        labels = rec.get("labels", [])
        if not isinstance(labels, list) or "upstream" not in labels:
            errors.append(f"`{key}` missing label `upstream` (labels={labels!r})")
        hay = (str(rec.get("title", "")) + "\n" + str(rec.get("description", ""))).lower()
        for sib in req_siblings:
            if sib not in hay:
                errors.append(f"`{key}` does not name sibling repository `{sib}`")
        if "interim posture" not in hay:
            errors.append(f"`{key}` does not state its interim posture")
        if "switch-over" not in hay and "switch over" not in hay:
            errors.append(f"`{key}` does not state its switch-over rule")

    # Optional read-only mirror probe (never fails inventory if sibling tracker unavailable)
    for sib_name in ("fastmcp_rust", "asupersync"):
        sib_beads = sandbox / sib_name / ".beads" / "issues.jsonl"
        mirrors[sib_name] = "present" if sib_beads.is_file() else "unchecked"

out_p.write_text(json.dumps({"errors": errors, "mirrors": mirrors}, indent=2) + "\n", encoding="utf-8")
PY

S9_ERR="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["errors"] else "; ".join(d["errors"]))' "${STEP9_OUT}")"
e2e_assert_eq "step9_beads_missing-p0-upstream-bead" "${S9_ERR}" "ok"
[ "${S9_ERR}" = "ok" ] || note_fail

for sib_m in fastmcp_rust asupersync; do
  m_val="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["mirrors"].get(sys.argv[2], "unchecked"))' "${STEP9_OUT}" "${sib_m}")"
  e2e_metric "step9_sibling_mirror_${sib_m}" "$([ "${m_val}" = "present" ] && echo 1 || echo 0)" "${m_val}"
done

# ---------------------------------------------------------------------------
# Step 10: Honesty scan (and optional frob doctor --json probe)
# ---------------------------------------------------------------------------
e2e_step "step_10_honesty_scan" "Verify zero claimed measurements in first commit (or pass/measured honesty in standing mode)"

if e2e_plant "measured-gate-in-first-commit"; then
  if [ -f "${CLONE_DIR}/registries/gates.toml" ]; then
    cat >> "${CLONE_DIR}/registries/gates.toml" <<'EOF'

[[gate]]
id = "GATE-PLANTED-MEASURED"
claim = "known-answer"
claim_id = "clm:planted-measured"
section = "38.13"
threshold = "561 cells"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = "561 of 561 cells"
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
EOF
  fi
fi

STEP10_OUT="${E2E_WORK_DIR}/step10.json"
python3 - "${CLONE_DIR}" "${MODE}" "${STEP10_OUT}" <<'PY'
import json, re, sys, tomllib
from pathlib import Path

clone = Path(sys.argv[1])
mode = sys.argv[2]
out_p = Path(sys.argv[3])

admission_errors = []
gate_measured_errors = []
admission_records_count = 0
measured_gates_count = 0

# 1. Inspect DEPENDENCY_ADMISSION.md and receipts/admissions/
adm_dir = clone / "receipts" / "admissions"
if adm_dir.is_dir():
    adm_files = sorted(p for p in adm_dir.rglob("*.json") if p.is_file())
    admission_records_count += len(adm_files)
    if mode == "first" and adm_files:
        admission_errors.append(f"receipts/admissions/ has {len(adm_files)} record(s) in first commit")

adm_doc = clone / "DEPENDENCY_ADMISSION.md"
if adm_doc.is_file():
    lines = adm_doc.read_text(encoding="utf-8", errors="replace").splitlines()
    for ln, line in enumerate(lines, 1):
        s = line.strip()
        if re.match(r"^##\s+(Admission Records|Bootstrap Observation Record)\b", s):
            admission_records_count += 1
            if mode == "first":
                admission_errors.append(f"DEPENDENCY_ADMISSION.md:{ln}: contains record section `{s}`")
        if s.startswith("|") and s.endswith("|"):
            cells = [c.strip() for c in s.strip("|").split("|")]
            if len(cells) >= 6 and cells[0].startswith("`INT-"):
                status_cell = cells[2].lower()
                measured_cell = cells[3]
                receipt_cell = cells[4]
                date_cell = cells[5]
                if status_cell != "not run" or measured_cell not in ("—", "-", "") or receipt_cell not in ("—", "-", "") or date_cell not in ("—", "-", ""):
                    admission_records_count += 1
                    if mode == "first":
                        admission_errors.append(
                            f"DEPENDENCY_ADMISSION.md:{ln}: gate {cells[0]} claims recorded status/measurement ({status_cell}, {measured_cell})"
                        )

# 2. Inspect registries/gates.toml for measured values
gates_p = clone / "registries" / "gates.toml"
if gates_p.is_file():
    try:
        gdoc = tomllib.loads(gates_p.read_text(encoding="utf-8"))
        for g in gdoc.get("gate", []) or []:
            if not isinstance(g, dict):
                continue
            gid = str(g.get("id", "<unknown>"))
            top_m = str(g.get("measured", "") or "").strip()
            if top_m:
                measured_gates_count += 1
                if mode == "first":
                    gate_measured_errors.append(f"registries/gates.toml: gate `{gid}` has measured value `{top_m}`")
            for pidx, p in enumerate(g.get("part", []) or []):
                if not isinstance(p, dict):
                    continue
                pm = str(p.get("measured", "") or "").strip()
                pst = str(p.get("status", "") or "").strip()
                if pm:
                    measured_gates_count += 1
                    if mode == "first":
                        gate_measured_errors.append(
                            f"registries/gates.toml: gate `{gid}` part[{pidx}] has measured value `{pm}`"
                        )
                if mode == "standing" and pst == "pass" and not pm:
                    gate_measured_errors.append(
                        f"registries/gates.toml: gate `{gid}` part[{pidx}] has status `pass` without measured value"
                    )
    except Exception as exc:
        gate_measured_errors.append(f"registries/gates.toml parse error: {exc}")

out_p.write_text(
    json.dumps(
        {
            "admission_records_count": admission_records_count,
            "measured_gates_count": measured_gates_count,
            "admission_errors": admission_errors,
            "gate_measured_errors": gate_measured_errors,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
PY

S10_ADM_CNT="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["admission_records_count"])' "${STEP10_OUT}")"
S10_MEAS_CNT="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["measured_gates_count"])' "${STEP10_OUT}")"
e2e_metric "step10_admission_records" "${S10_ADM_CNT}" "count"
e2e_metric "step10_gates_measured" "${S10_MEAS_CNT}" "count"

if [ "${MODE}" = "first" ]; then
  S10_ADM="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["admission_errors"] else "; ".join(d["admission_errors"]))' "${STEP10_OUT}")"
  e2e_assert_eq "step10_honesty_admission_zero_records" "${S10_ADM}" "ok"
  [ "${S10_ADM}" = "ok" ] || note_fail

  S10_MEAS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["gate_measured_errors"] else "; ".join(d["gate_measured_errors"]))' "${STEP10_OUT}")"
  e2e_assert_eq "step10_honesty_measured-gate-in-first-commit" "${S10_MEAS}" "ok"
  [ "${S10_MEAS}" = "ok" ] || note_fail
else
  S10_MEAS="$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print("ok" if not d["gate_measured_errors"] else "; ".join(d["gate_measured_errors"]))' "${STEP10_OUT}")"
  e2e_assert_eq "step10_standing_pass_has_measured" "${S10_MEAS}" "ok"
  [ "${S10_MEAS}" = "ok" ] || note_fail
fi

# Optional `frob doctor --json` probe when the binary already exists.
if [ -n "${FROB_BIN:-}" ] && [ -x "${FROB_BIN}" ]; then
  e2e_metric "frob_doctor_present" "1" "present"
  e2e_cmd --name "frob_doctor_json" --expect-exit "0,3" -- "${FROB_BIN}" doctor --json
else
  e2e_metric "frob_doctor_present" "0" "skipped_absent"
fi

if [ "${MODE}" = "standing" ] && [ -n "${STATE_DIR}" ] && [ "${FAIL_COUNT}" -eq 0 ]; then
  mkdir -p "${STATE_DIR}"
  printf '%s\n' "${TARGET_SHA}" > "${STATE_DIR}/last_pass_commit"
fi
