#!/usr/bin/env bash
# e2e: bead=N34p4-02 milestone=M0 labels=workspace,xtask,solid seed=1 needs=cpu
#
# End-to-end verification of the FrankenRobotics workspace layout, seven-workspace
# table, core lockfile forbidden-package boundary, core-to-cudarc plant, and
# one-toolchain-file rule (bead N34p4-02).
#
# N34-01 appends the steps that build and run the `apps/` device-enabled binaries.
# Claims no gates on its own; supports INV-WS-01 and ADR-SPARK-01.
# Bash 3.2 compatible; never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/lib.sh"

e2e_init "workspace_layout" "N34p4-02" "$@"

XTASK_TARGET_DIR="${RCH_TARGET_BASE:-${E2E_WORK_DIR}}/workspace_layout_xtask"
mkdir -p "${XTASK_TARGET_DIR}"

WORKSPACES_RS="${E2E_REPO_ROOT}/crates/xtask/src/workspaces.rs"

_hash_file() {
  local fpath="$1"
  if command -v b3sum >/dev/null 2>&1; then
    b3sum --no-names "${fpath}"
  else
    "${FROB_E2E_TOOL}" cmp sha256 "${fpath}"
  fi
}

# ---------------------------------------------------------------------------
# Step 1: Parse the single workspace table & forbidden-package list from
#         crates/xtask/src/workspaces.rs (zero duplicate table in shell).
# ---------------------------------------------------------------------------
e2e_step "workspace_table" "Read the seven workspaces and forbidden-package patterns from crates/xtask/src/workspaces.rs"

TABLE_JSON="${E2E_WORK_DIR}/workspace_table.json"
python3 - "${WORKSPACES_RS}" "${TABLE_JSON}" <<'PY'
import json, re, sys
from pathlib import Path

src = Path(sys.argv[1]).read_text(encoding="utf-8")

ws_match = re.search(
    r"pub const WORKSPACES:\s*&\[WorkspaceRow\]\s*=\s*&\[(.*?)\];",
    src,
    re.S,
)
if not ws_match:
    raise SystemExit("failed to locate WORKSPACES table in workspaces.rs")

workspaces = []
for block in re.findall(r"WorkspaceRow\s*\{(.*?)\}", ws_match.group(1), re.S):
    name = re.search(r'name:\s*"([^"]+)"', block).group(1)
    root_dir = re.search(r'root_dir:\s*"([^"]*)"', block).group(1)
    lockfile = re.search(r'lockfile:\s*"([^"]+)"', block).group(1)
    kind = re.search(r"kind:\s*WorkspaceKind::([A-Za-z]+)", block).group(1)
    workspaces.append(
        {"name": name, "root_dir": root_dir, "lockfile": lockfile, "kind": kind}
    )

forb_match = re.search(
    r"pub const FORBIDDEN_CORE_PACKAGES:\s*&\[ForbiddenPackage\]\s*=\s*&\[(.*?)\];",
    src,
    re.S,
)
if not forb_match:
    raise SystemExit("failed to locate FORBIDDEN_CORE_PACKAGES in workspaces.rs")

forbidden = []
for fn_kind, pkg, reason_id in re.findall(
    r'ForbiddenPackage::(annex|route|nongoal)\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)',
    forb_match.group(1),
):
    forbidden.append({"package": pkg, "why_kind": fn_kind, "why_id": reason_id})

out = {
    "workspaces": workspaces,
    "forbidden_core_packages": forbidden,
}
Path(sys.argv[2]).write_text(json.dumps(out, indent=2) + "\n", encoding="utf-8")
print(f"workspaces={len(workspaces)} forbidden={len(forbidden)}")
PY

WS_COUNT="$(python3 -c 'import json,sys; print(len(json.load(open(sys.argv[1]))["workspaces"]))' "${TABLE_JSON}")"
LINKING_COUNT="$(python3 -c 'import json,sys; print(sum(1 for w in json.load(open(sys.argv[1]))["workspaces"] if w["kind"] != "Retarget"))' "${TABLE_JSON}")"
RETARGET_COUNT="$(python3 -c 'import json,sys; print(sum(1 for w in json.load(open(sys.argv[1]))["workspaces"] if w["kind"] == "Retarget"))' "${TABLE_JSON}")"
FORB_COUNT="$(python3 -c 'import json,sys; print(len(json.load(open(sys.argv[1]))["forbidden_core_packages"]))' "${TABLE_JSON}")"

e2e_assert_eq "seven_workspaces_listed" "${WS_COUNT}" "7"
e2e_assert_eq "six_workspaces_link_code" "${LINKING_COUNT}" "6"
e2e_assert_eq "one_workspace_retargets" "${RETARGET_COUNT}" "1"
e2e_metric "workspaces_declared" "${WS_COUNT}" "count"
e2e_metric "forbidden_core_patterns" "${FORB_COUNT}" "count"

# ---------------------------------------------------------------------------
# Step 2: Run `cargo metadata --locked --offline` on every present workspace,
#         log absent rows as `absent`, hash lockfiles, and scan core Cargo.lock.
# ---------------------------------------------------------------------------
e2e_step "metadata_and_lockfiles" "Resolve present workspaces with --locked, log absent rows, digest and audit lockfiles"

ROWS_TSV="${E2E_WORK_DIR}/workspace_rows.tsv"
python3 - "${TABLE_JSON}" "${ROWS_TSV}" <<'PY'
import json, sys
data = json.load(open(sys.argv[1], encoding="utf-8"))
with open(sys.argv[2], "w", encoding="utf-8") as fh:
    for w in data["workspaces"]:
        fh.write(f'{w["name"]}\t{w["root_dir"]}\t{w["lockfile"]}\t{w["kind"]}\n')
PY

PRESENT_COUNT=0
ABSENT_COUNT=0

while IFS="$(printf '\t')" read -r ws_name ws_root ws_lock ws_kind; do
  if [ -z "${ws_root}" ]; then
    manifest="${E2E_REPO_ROOT}/Cargo.toml"
    lock_abs="${E2E_REPO_ROOT}/${ws_lock}"
  else
    manifest="${E2E_REPO_ROOT}/${ws_root}/Cargo.toml"
    lock_abs="${E2E_REPO_ROOT}/${ws_lock}"
  fi

  if [ ! -f "${manifest}" ]; then
    ABSENT_COUNT=$((ABSENT_COUNT + 1))
    e2e_metric "workspace_absent_${ws_name}" "1" "absent"
    continue
  fi

  PRESENT_COUNT=$((PRESENT_COUNT + 1))
  e2e_cmd --name "cargo_metadata_${ws_name}" -- \
    cargo metadata --locked --offline --format-version 1 --manifest-path "${manifest}"
  e2e_assert_eq "metadata_exit_0_${ws_name}" "${E2E_LAST_EXIT}" "0"

  e2e_assert_file "lockfile_exists_${ws_name}" "${lock_abs}"
  lock_digest="$(_hash_file "${lock_abs}")"
  e2e_metric "lockfile_blake3_${ws_name}" "1" "${lock_digest}"

  if [ "${ws_kind}" = "Core" ]; then
    CORE_GREP_OUT="${E2E_WORK_DIR}/core_lock_scan.txt"
    python3 - "${TABLE_JSON}" "${lock_abs}" "${CORE_GREP_OUT}" <<'PY'
import json, sys, tomllib
from pathlib import Path

table = json.load(open(sys.argv[1], encoding="utf-8"))
patterns = table["forbidden_core_packages"]
lock = tomllib.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

pkgs = [p.get("name", "") for p in lock.get("package", []) if isinstance(p, dict)]
unused = lock.get("patch", {}).get("unused", [])
pkgs.extend(p.get("name", "") for p in unused if isinstance(p, dict))

hits = []
for pkg in sorted(set(pkgs)):
    for row in patterns:
        pat = row["package"]
        matched = pkg.startswith(pat[:-1]) if pat.endswith("*") else (pkg == pat)
        if matched:
            hits.append(f'{pkg} ({row["why_kind"]} = "{row["why_id"]}")')

Path(sys.argv[3]).write_text("\n".join(hits) + ("\n" if hits else ""), encoding="utf-8")
print(len(hits))
PY
    CORE_HITS="$(wc -l < "${CORE_GREP_OUT}" | tr -d ' ')"
    e2e_assert_eq "core_lockfile_zero_forbidden_packages" "${CORE_HITS}" "0"
    e2e_metric "core_forbidden_hits" "${CORE_HITS}" "count"
  fi
done < "${ROWS_TSV}"

e2e_metric "workspaces_present" "${PRESENT_COUNT}" "count"
e2e_metric "workspaces_absent" "${ABSENT_COUNT}" "count"

# ---------------------------------------------------------------------------
# Step 3: Plant core -> cudarc in an isolated offline scratch copy and assert
#         `cargo xtask check-deps --root <copy> --json` exits 2 with both
#         L-4.foundation-confined (on the edge) and core.forbidden-package.
# ---------------------------------------------------------------------------
e2e_step "plant_core_cudarc" "Plant cudarc in a scratch core crate and assert check-deps exits 2 with L-4 and core.forbidden-package"

PLANT_BASE="${E2E_WORK_DIR}/plant_core_cudarc"
PLANT_REPO="${PLANT_BASE}/repo"
PLANT_REG="${PLANT_BASE}/local-reg"
mkdir -p "${PLANT_REPO}/crates/frob-core/src" "${PLANT_REG}/cudarc-0.19.9/src" "${PLANT_BASE}/.cargo"

cp "${E2E_REPO_ROOT}/franken-stack.lock" "${PLANT_REPO}/franken-stack.lock"
if [ -f "${E2E_REPO_ROOT}/registries/siblings.toml" ]; then
  mkdir -p "${PLANT_REPO}/registries"
  cp "${E2E_REPO_ROOT}/registries/siblings.toml" "${PLANT_REPO}/registries/siblings.toml"
fi

cat > "${PLANT_BASE}/.cargo/config.toml" <<EOF
[source.crates-io]
replace-with = "local-reg"

[source.local-reg]
directory = "${PLANT_REG}"
EOF

cat > "${PLANT_REG}/cudarc-0.19.9/Cargo.toml" <<'EOF'
[package]
name = "cudarc"
version = "0.19.9"
edition = "2021"
EOF
printf '// synthetic cudarc registry crate\n' > "${PLANT_REG}/cudarc-0.19.9/src/lib.rs"
printf '{"files":{}}\n' > "${PLANT_REG}/cudarc-0.19.9/.cargo-checksum.json"

cat > "${PLANT_REPO}/Cargo.toml" <<'EOF'
[workspace]
resolver = "3"
members = ["crates/*"]
EOF

cat > "${PLANT_REPO}/crates/frob-core/Cargo.toml" <<'EOF'
[package]
name = "frob-core"
version = "0.0.1"
edition = "2024"

[package.metadata.frob]
layer = 0
rank = 1

[dependencies]
cudarc = { version = "=0.19.9", default-features = false }
EOF
printf '// synthetic core crate with planted cudarc dependency\n' > "${PLANT_REPO}/crates/frob-core/src/lib.rs"

PREV_DIR="${PWD}"
cd "${PLANT_REPO}" || exit 1
e2e_cmd --name "gen_lock_plant_cudarc" -- \
  cargo generate-lockfile --offline --manifest-path "${PLANT_REPO}/Cargo.toml"
cd "${PREV_DIR}" || exit 1
e2e_assert_eq "gen_lock_plant_cudarc_exit_0" "${E2E_LAST_EXIT}" "0"

e2e_cmd --name "check_deps_plant_cudarc" --expect-exit 2 -- \
  env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
  cargo xtask check-deps --root "${PLANT_REPO}" --json
e2e_assert_eq "plant_cudarc_exit_2" "${E2E_LAST_EXIT}" "2"

PLANT_OUT="${E2E_WORK_DIR}/plant_cudarc_report.json"
cp "${E2E_LAST_STDOUT}" "${PLANT_OUT}"

PLANT_CHECK="$(python3 - "${PLANT_OUT}" <<'PY'
import json, sys
doc = json.load(open(sys.argv[1], encoding="utf-8"))
findings = doc.get("findings", [])
has_edge = any(
    f.get("rule") == "L-4.foundation-confined" and f.get("subject") == "frob-core -> cudarc"
    for f in findings
)
has_forb = any(
    f.get("rule") == "core.forbidden-package"
    and "cudarc" in f.get("message", "")
    and 'annex = "spark-cuda"' in f.get("message", "")
    for f in findings
)
print("OK" if (has_edge and has_forb) else f"FAIL:edge={has_edge},forb={has_forb},findings={findings}")
PY
)"
e2e_assert_eq "plant_cudarc_both_findings" "${PLANT_CHECK}" "OK"

# ---------------------------------------------------------------------------
# Step 4: Clean repository passes check-layers (L-3) and check-deps.
# ---------------------------------------------------------------------------
e2e_step "clean_tree_checks" "Verify clean tree passes check-layers (L-3) and check-deps with absent rows in facts"

e2e_cmd --name "clean_tree_check_layers" -- \
  env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
  cargo xtask check-layers --root "${E2E_REPO_ROOT}" --json
e2e_assert_eq "clean_check_layers_exit_0" "${E2E_LAST_EXIT}" "0"
e2e_assert_json "clean_check_layers_ok" "/ok" "true"

e2e_cmd --name "clean_tree_check_deps" -- \
  env "CARGO_TARGET_DIR=${XTASK_TARGET_DIR}" \
  cargo xtask check-deps --root "${E2E_REPO_ROOT}" --json
e2e_assert_eq "clean_check_deps_exit_0" "${E2E_LAST_EXIT}" "0"
e2e_assert_json "clean_check_deps_ok" "/ok" "true"

# ---------------------------------------------------------------------------
# Step 5: Verify rust-toolchain.toml locations across the clean tree.
# ---------------------------------------------------------------------------
e2e_step "toolchain_files" "Verify rust-toolchain.toml exists only at root and crates/frob-cert/ (and worker/ only if active in stack lock)"

TOOLCHAIN_CHECK="$(python3 - "${E2E_REPO_ROOT}" <<'PY'
import os, sys, tomllib
from pathlib import Path

repo = Path(sys.argv[1])
skip = {
    ".git", "target", ".beads", ".ruff_cache", ".ee",
    ".local-implementation", ".ntm", "repro", "planning",
    "node_modules", "lean", "viewer",
}
found = []
for root, dirs, files in os.walk(repo):
    dirs[:] = sorted(d for d in dirs if d not in skip)
    if "rust-toolchain.toml" in files:
        rel = Path(root, "rust-toolchain.toml").relative_to(repo).as_posix()
        found.append(rel)

found.sort()
worker_fallback = False
lock_file = repo / "franken-stack.lock"
if lock_file.is_file():
    lock_doc = tomllib.loads(lock_file.read_text(encoding="utf-8"))
    worker_tbl = lock_doc.get("worker")
    if isinstance(worker_tbl, dict) and worker_tbl.get("toolchain"):
        worker_fallback = True
    for f in lock_doc.get("foundation", []):
        if isinstance(f, dict) and f.get("worker_fallback") is True:
            worker_fallback = True

allowed = {"rust-toolchain.toml", "crates/frob-cert/rust-toolchain.toml"}
if worker_fallback:
    allowed.add("worker/rust-toolchain.toml")

unexpected = [p for p in found if p not in allowed]
missing_root = "rust-toolchain.toml" not in found
if missing_root or unexpected:
    print(f"FAIL:found={found},unexpected={unexpected}")
else:
    print("OK:" + ",".join(found))
PY
)"
case "${TOOLCHAIN_CHECK}" in
  OK:*)
    e2e_assert_eq "toolchain_files_valid" "OK" "OK"
    e2e_metric "toolchain_files_count" "$(printf '%s' "${TOOLCHAIN_CHECK#OK:}" | awk -F',' '{print NF}')" "count"
    ;;
  *)
    e2e_assert_eq "toolchain_files_valid" "${TOOLCHAIN_CHECK}" "OK"
    ;;
esac
