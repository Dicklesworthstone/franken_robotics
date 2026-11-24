#!/usr/bin/env bash
# Unit test: direct_indirect_split (plus canonical JSON, BLAKE3 hash,
# byte-identical repo check, and twice-run determinism).

set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/coupling_fixture_lib.sh"

SCRATCH="$(coupling_make_scratch "test-direct-indirect")"
FIXTURE_OUT="$(coupling_create_base_fixture_repo "$SCRATCH")"
REPO="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '1p')"
SHA="$(printf '%s\n' "$FIXTURE_OUT" | sed -n '2p')"

WORK="$SCRATCH/work"
mkdir -p "$WORK"

BEFORE_HEAD="$(git -C "$REPO" rev-parse HEAD)"
BEFORE_STAT="$(git -C "$REPO" status --porcelain)"

OUT1="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA")"
REC1="$WORK/$(coupling_extract_kv "$OUT1" "record_path")"
HASH1="$(coupling_extract_kv "$OUT1" "record_blake3")"

OUT2="$(cd "$WORK" && "$QTY_COUPLING_SCRIPT" --repo "$REPO" --rev "$SHA")"
REC2="$WORK/$(coupling_extract_kv "$OUT2" "record_path")"
HASH2="$(coupling_extract_kv "$OUT2" "record_blake3")"

AFTER_HEAD="$(git -C "$REPO" rev-parse HEAD)"
AFTER_STAT="$(git -C "$REPO" status --porcelain)"

test "$BEFORE_HEAD" = "$AFTER_HEAD"
test "$BEFORE_STAT" = "$AFTER_STAT"

# Determinism lane: two runs produce byte-identical JSON and identical BLAKE3.
cmp -s "$REC1" "$REC2"
test "$HASH1" = "$HASH2"
ACTUAL_HASH="$(b3sum --no-names "$REC1")"
test "$HASH1" = "$ACTUAL_HASH"

python3 - "$REC1" "$SHA" <<'PY'
import json, sys
from pathlib import Path

rec_path = Path(sys.argv[1])
expected_sha = sys.argv[2]
raw = rec_path.read_text(encoding="utf-8")
data = json.loads(raw)

# Canonical JSON check: sorted keys, no insignificant whitespace.
reserialized = json.dumps(data, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
assert raw == reserialized, "record is not canonical JSON"

assert data["schema"] == "frob.qty-coupling/1"
assert data["status"] == "ok"
assert data["frankensim_rev"] == expected_sha
assert data["rev_source"] == "argument"
assert data["dirty"] is False
assert data["tree_hash"] is None
assert data["reported"] == {"coupled": 22, "direct": 12, "indirect": 10, "freed": 8}
assert data["reproduced_by"] == "none"

modes = {m["feature_mode"]: m for m in data["modes"]}
assert set(modes.keys()) == {"default", "all"}

for fmode, m in modes.items():
    pops = {p["name"]: p for p in m["populations"]}
    assert set(pops.keys()) == {"P1", "P2", "P3"}
    p3 = pops["P3"]
    assert p3["size"] == 5, f"{fmode}: expected P3 size 5, got {p3['size']}"
    assert p3["direct"] == ["fs-evidence"], f"{fmode}: unexpected D={p3['direct']}"
    indirect_crates = [x["crate"] for x in p3["indirect"]]
    assert indirect_crates == ["i", "m"], f"{fmode}: unexpected I={indirect_crates}"
    coupled = sorted(p3["direct"] + indirect_crates)
    assert coupled == ["fs-evidence", "i", "m"], f"{fmode}: unexpected C={coupled}"
    assert p3["direct_count"] == 1
    assert p3["indirect_count"] == 2
    assert p3["coupled_count"] == 3
    assert p3["coupled_count"] == p3["direct_count"] + p3["indirect_count"]
    assert "fs-qty" not in coupled
    assert "z" not in coupled
    # P1 contains only fs-qty and fs-evidence in this synthetic fixture.
    p1 = pops["P1"]
    assert p1["size"] == 2
    assert p1["direct"] == ["fs-evidence"]
    assert p1["indirect"] == []
    assert p1["coupled_count"] == 1
    # P2 has no M1 group seeds present in the synthetic fixture.
    p2 = pops["P2"]
    assert p2["size"] == 0
    assert p2["coupled_count"] == 0
PY

echo "PASS: coupling_direct_indirect_split"
