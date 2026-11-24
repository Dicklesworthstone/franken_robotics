#!/usr/bin/env bash
# Unit test: populations P1/P2/P3, reproduced_by detection (22/12/10/8),
# bedrock-direct membership checks, and dirty --working-tree tree_hash.

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-populations-reproduced")"
REPO="$SCRATCH/frankensim"
mkdir -p "$REPO/crates"

# Construct a synthetic workspace whose P3 reproduces 22 / 12 / 10 / 8 exactly:
# - Target: fs-qty (not counted in C)
# - Direct (12):
#   * fs-evidence -> fs-qty (freed by removing fs-evidence -> fs-qty: 1)
#   * d01..d11 -> fs-qty (11 direct deps that stay coupled after cut)
# - Indirect (10):
#   * fs-ivl -> fs-evidence (freed: +1 -> 2)
#   * fs-mbd -> helper-m1 -> fs-evidence (both in P2 M1 closure; freed: +2 -> 4)
#   * i01..i04 -> fs-evidence (freed: +4 -> 8 total freed!)
#   * i05..i07 -> d01 (stay coupled via d01 -> fs-qty: 3 indirect stay coupled)
# - Uncoupled bedrock-direct crates: fs-ga, fs-math, fs-rand, fs-blake3, fs-simd, fs-exec, fs-ad
# Total P3 coupled = 12 + 10 = 22; direct = 12; indirect = 10; freed = 8; still = 14.

cat > "$REPO/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = ["crates/*"]
EOF

python3 - "$REPO" <<'PY'
import sys
from pathlib import Path

repo = Path(sys.argv[1])
crates = {
    "fs-qty": [],
    "fs-evidence": ["fs-qty"],
    "fs-ivl": ["fs-evidence"],
    "fs-mbd": ["helper-m1"],
    "helper-m1": ["fs-evidence"],
    "fs-ga": [],
    "fs-math": [],
    "fs-rand": [],
    "fs-blake3": [],
    "fs-simd": [],
    "fs-exec": [],
    "fs-ad": [],
}
for k in range(1, 12):
    crates[f"d{k:02d}"] = ["fs-qty"]
for k in range(1, 5):
    crates[f"i{k:02d}"] = ["fs-evidence"]
for k in range(5, 8):
    crates[f"i{k:02d}"] = ["d01"]

for name, deps in crates.items():
    cdir = repo / "crates" / name
    (cdir / "src").mkdir(parents=True)
    dep_lines = "\n".join(f'{d} = {{ path = "../{d}" }}' for d in deps)
    (cdir / "Cargo.toml").write_text(
        f'[package]\nname = "{name}"\nversion = "0.0.1"\nedition = "2021"\n\n[dependencies]\n{dep_lines}\n',
        encoding="utf-8",
    )
    (cdir / "src" / "lib.rs").write_text(f'pub fn id() -> &\'static str {{ "{name}" }}\n', encoding="utf-8")
PY

CARGO_TARGET_DIR="$SCRATCH/cargo-target" \
  cargo generate-lockfile --offline --manifest-path "$REPO/Cargo.toml" >/dev/null 2>&1

git -C "$REPO" init -q
git -C "$REPO" config user.name "CopperDove Test"
git -C "$REPO" config user.email "copperdove@test.local"
git -C "$REPO" add -A
git -C "$REPO" commit -q -m "reproduced 22/12/10/8 fixture"
SHA="$(git -C "$REPO" rev-parse HEAD)"

WORK="$SCRATCH/work"
mkdir -p "$WORK"
OUT="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA")"
REC="$WORK/$(coupling_extract_kv "$OUT" "record_path")"

python3 - "$REC" <<'PY'
import json, sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert data["reproduced_by"] == "P3", data["reproduced_by"]

for m in data["modes"]:
    assert m["reproduced_by"] == "P3"
    pops = {p["name"]: p for p in m["populations"]}

    # P3 reproduces 22 / 12 / 10 / 8.
    p3 = pops["P3"]
    assert p3["coupled_count"] == 22, p3
    assert p3["direct_count"] == 12, p3
    assert p3["indirect_count"] == 10, p3
    assert p3["freed_count"] == 8, p3
    assert len(p3["still_coupled_after_31"]) == 14, p3

    # P2 (M1 closure from fs-mbd): {fs-mbd, helper-m1, fs-evidence, fs-qty} -> size 4.
    p2 = pops["P2"]
    assert p2["size"] == 4, p2
    assert p2["direct"] == ["fs-evidence"], p2
    assert [x["crate"] for x in p2["indirect"]] == ["fs-mbd", "helper-m1"], p2
    assert p2["freed_by_fd_fsim_31"] == ["fs-evidence", "fs-mbd", "helper-m1"], p2

    # P1 (consumed crates): 10 bedrock-direct + fs-mbd = 11.
    p1 = pops["P1"]
    assert p1["size"] == 11, p1
    p1_coupled = set(p1["direct"]) | {x["crate"] for x in p1["indirect"]}
    assert "fs-evidence" in p1_coupled
    assert "fs-ivl" in p1_coupled
    for uncoupled in ("fs-ga", "fs-math", "fs-rand", "fs-blake3", "fs-simd", "fs-exec", "fs-ad"):
        assert uncoupled not in p1_coupled, f"unexpected coupling for {uncoupled}"
PY

# Test --working-tree on a dirty modification (modifies tracked file without committing).
printf '\n// dirty edit\n' >> "$REPO/crates/fs-ga/src/lib.rs"
OUT_DIRTY1="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA" --working-tree)"
REC_DIRTY1="$WORK/$(coupling_extract_kv "$OUT_DIRTY1" "record_path")"

printf '\n// second dirty edit\n' >> "$REPO/crates/fs-ga/src/lib.rs"
OUT_DIRTY2="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA" --working-tree)"
REC_DIRTY2="$WORK/$(coupling_extract_kv "$OUT_DIRTY2" "record_path")"

python3 - "$REC_DIRTY1" "$REC_DIRTY2" <<'PY'
import json, re, sys
from pathlib import Path

d1 = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
d2 = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
assert d1["status"] == "ok-dirty"
assert d1["source_tree"] == "working-tree"
assert d1["dirty"] is True
assert re.match(r"^[0-9a-f]{64}$", d1["tree_hash"])
assert d2["status"] == "ok-dirty"
assert d2["dirty"] is True
assert re.match(r"^[0-9a-f]{64}$", d2["tree_hash"])
assert d1["tree_hash"] != d2["tree_hash"], "tree_hash must change when a tracked file changes"
PY

echo "PASS: coupling_reproduced_and_populations"
