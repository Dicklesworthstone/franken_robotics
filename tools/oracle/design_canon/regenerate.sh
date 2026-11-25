#!/usr/bin/env bash
# Regenerate golden canonical expectations (`*.canon`) and `manifest.toml` from
# the independent reference canonicalizer (`tools/oracle/design_canon/canon.py`).
#
# This script is the ONLY legal way golden expectations are produced (N14a-26).
#
# Usage:
#   tools/oracle/design_canon/regenerate.sh [--corpus-dir <dir>] [--out-dir <dir>]
#
# When `--out-dir` is omitted, expectations and `manifest.toml` are written into
# `--corpus-dir` (default: `crates/frob-conformance/fixtures/design_golden`).
# When `--out-dir` is provided (e.g. a scratch directory in e2e verification),
# inputs are read from `--corpus-dir` without modifying the repository tree.
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd -P)"
CORPUS_DIR="${REPO_ROOT}/crates/frob-conformance/fixtures/design_golden"
OUT_DIR=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --corpus-dir)
      [ "$#" -ge 2 ] || { echo "error: --corpus-dir requires a path" >&2; exit 2; }
      CORPUS_DIR="$2"
      shift 2
      ;;
    --out-dir)
      [ "$#" -ge 2 ] || { echo "error: --out-dir requires a path" >&2; exit 2; }
      OUT_DIR="$2"
      shift 2
      ;;
    -h|--help)
      echo "usage: $0 [--corpus-dir <dir>] [--out-dir <dir>]"
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [ -z "${OUT_DIR}" ]; then
  OUT_DIR="${CORPUS_DIR}"
fi

mkdir -p "${OUT_DIR}"

python3 - "${SCRIPT_DIR}" "${CORPUS_DIR}" "${OUT_DIR}" <<'PY'
from pathlib import Path
import re
import sys

script_dir = Path(sys.argv[1])
corpus_dir = Path(sys.argv[2])
out_dir = Path(sys.argv[3])

sys.path.insert(0, str(script_dir))

import blake3_ref
import canon

# Script hash covers all three Python modules of the independent reference in fixed order.
REF_FILES = ("blake3_ref.py", "units.py", "canon.py")


def compute_oracle_blake3() -> str:
    h = blake3_ref.Blake3Hasher()
    for fname in REF_FILES:
        data = (script_dir / fname).read_bytes()
        h.update(f"file:{fname}:{len(data)}\n".encode("utf-8"))
        h.update(data)
        h.update(b"\n")
    return h.finalize().hex()


def read_existing_metadata(manifest_path: Path):
    """Read (group, purpose, expect, reason, same_id_as) per case id from existing manifest.toml."""
    if not manifest_path.is_file():
        raise SystemExit(f"error: base manifest metadata not found at {manifest_path}")
    text = manifest_path.read_text(encoding="utf-8")
    cases = []
    blocks = text.split("[[case]]")[1:]
    for blk in blocks:
        row = {}
        for m in re.finditer(r'^\s*([a-z_]+)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$', blk, flags=re.M):
            k = m.group(1)
            v = m.group(2).replace('\\"', '"').replace("\\\\", "\\")
            row[k] = v
        if row:
            cases.append(row)
    return cases


def toml_escape(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


oracle_hash = compute_oracle_blake3()
meta_rows = read_existing_metadata(corpus_dir / "manifest.toml")
meta_rows.sort(key=lambda r: r["input"])

input_files = sorted(p.name for p in corpus_dir.glob("*.fbot"))
meta_inputs = [r["input"] for r in meta_rows]
if input_files != meta_inputs:
    missing = sorted(set(input_files) - set(meta_inputs))
    extra = sorted(set(meta_inputs) - set(input_files))
    raise SystemExit(f"error: manifest/input mismatch: missing={missing} extra={extra}")

zero_hash = "0" * 64
lines = [
    "# Golden corpus manifest for `frob.design/1` canonical form (`N14a-26`).",
    "# Generated exclusively by `tools/oracle/design_canon/regenerate.sh`.",
    "# Never hand-edit expectations to match product code; see tools/oracle/design_canon/README.md.",
    "",
    'schema = "frob.design.golden-manifest/1"',
    'oracle = "tools/oracle/design_canon/canon.py"',
    f'oracle_blake3 = "{oracle_hash}"',
    f'manifest_blake3 = "{zero_hash}"',
]

accepted_count = 0
refused_count = 0

for row in meta_rows:
    cid = row["id"]
    group = row["group"]
    inp_name = row["input"]
    purpose = row["purpose"]
    expect = row["expect"]
    same_id_as = row.get("same_id_as", "")

    inp_bytes = (corpus_dir / inp_name).read_bytes()
    inp_hash = blake3_ref.hash_hex(inp_bytes)

    if expect == "accept":
        canon_bytes, did = canon.canonicalize_bytes(inp_bytes, mode="ref")
        # Idempotence sanity check: reference canonicalizer must be idempotent on every accepted input.
        canon_bytes_2, did_2 = canon.canonicalize_bytes(canon_bytes, mode="ref")
        if canon_bytes != canon_bytes_2 or did != did_2:
            raise SystemExit(f"error: reference not idempotent on {inp_name}")
        canon_name = inp_name[:-len(".fbot")] + ".canon"
        (out_dir / canon_name).write_bytes(canon_bytes)
        canon_hash = blake3_ref.hash_hex(canon_bytes)
        reason = ""
        accepted_count += 1
    elif expect == "refuse":
        expected_reason = row["reason"]
        try:
            canon.canonicalize_bytes(inp_bytes, mode="ref")
        except canon.CanonRefusal as ref:
            if ref.reason != expected_reason:
                raise SystemExit(
                    f"error: {inp_name} refused with `{ref.reason}`, expected `{expected_reason}` ({ref.detail})"
                )
        else:
            raise SystemExit(f"error: refusal fixture {inp_name} unexpectedly accepted")
        canon_name = ""
        canon_hash = ""
        did = ""
        reason = expected_reason
        refused_count += 1
    else:
        raise SystemExit(f"error: invalid expect={expect!r} for {cid}")

    lines.extend(
        [
            "",
            "[[case]]",
            f'id = "{toml_escape(cid)}"',
            f'group = "{toml_escape(group)}"',
            f'input = "{toml_escape(inp_name)}"',
            f'purpose = "{toml_escape(purpose)}"',
            f'expect = "{toml_escape(expect)}"',
            f'input_blake3 = "{inp_hash}"',
            f'canon = "{toml_escape(canon_name)}"',
            f'canon_blake3 = "{canon_hash}"',
            f'design_id = "{did}"',
            f'reason = "{toml_escape(reason)}"',
            f'same_id_as = "{toml_escape(same_id_as)}"',
        ]
    )

template = "\n".join(lines) + "\n"
manifest_digest = blake3_ref.hash_hex(template.encode("utf-8"))
final_manifest = template.replace(
    f'manifest_blake3 = "{zero_hash}"',
    f'manifest_blake3 = "{manifest_digest}"',
    1,
)
(out_dir / "manifest.toml").write_text(final_manifest, encoding="utf-8", newline="\n")
print(
    f"regenerate: wrote {accepted_count} .canon files + manifest.toml "
    f"(total={accepted_count + refused_count}, accepted={accepted_count}, refused={refused_count}, "
    f"oracle_blake3={oracle_hash}, manifest_blake3={manifest_digest})"
)
PY
