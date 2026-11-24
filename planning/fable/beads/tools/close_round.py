#!/usr/bin/env python3
"""Close a polishing round: validate every staging file, total the reviewers' results, write the round summary.

  close_round.py <round>
"""
import json
import re
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, SECTIONS, load_all  # noqa: E402

rnd = int(sys.argv[1])
R = BEADS / "polish" / ("round-%d" % rnd)
beads, problems = load_all()
live = {k: b for k, b in beads.items() if not b.get("superseded_by")}

bad = []
for k, b in live.items():
    miss = [s for s in SECTIONS if s.lower() not in b["body"].lower()]
    if miss:
        bad.append((k, "missing " + ", ".join(miss)))
    if not re.match(r"^[A-Za-z0-9-]+$", k):
        bad.append((k, "odd key"))
    if b.get("type") not in ("task", "feature", "epic"):
        bad.append((k, "type %r" % b.get("type")))
    if b.get("priority") not in (0, 1, 2, 3, 4):
        bad.append((k, "priority %r" % b.get("priority")))
    if str(b.get("milestone")) not in ["M%d" % i for i in range(9)]:
        bad.append((k, "milestone %r" % b.get("milestone")))
    for f in ("deps", "related"):
        for d in b[f]:
            if d not in beads:
                bad.append((k, "%s names unknown key %s" % (f, d)))
    p = b.get("parent")
    if k != "EPIC-PROGRAM" and (not p or p not in beads):
        bad.append((k, "parent %r unknown" % p))
    if b["needs"]:
        bad.append((k, "needs is not empty (%d entries): prerequisites belong in deps or related" % len(b["needs"])))
for k, b in beads.items():
    s = b.get("superseded_by")
    if s and (s not in beads or s == k):
        bad.append((k, "superseded_by names unknown key %r" % s))

slices = json.loads((R / "slices.json").read_text(encoding="utf-8"))
tot = Counter(); kinds = Counter(); per_slice = []; missing = []
changed_keys = set()
for s in slices:
    f = R / "out" / (s["slice"] + ".json")
    if not f.exists():
        missing.append(s["slice"]); continue
    try:
        d = json.loads(f.read_text(encoding="utf-8"))
    except Exception as e:  # noqa: BLE001
        bad.append((s["slice"], "result file unreadable: %s" % e)); continue
    ch = d.get("changed") or []
    n_changed = len(ch) if isinstance(ch, list) else int(ch)
    if isinstance(ch, list):
        for c in ch:
            if isinstance(c, dict):
                changed_keys.add(str(c.get("bead")))
                for kk in c.get("kinds") or []:
                    kinds[str(kk)] += 1
    nb = d.get("new_beads") or []; sp = d.get("superseded") or []; rq = d.get("requests") or []
    n = lambda v: len(v) if isinstance(v, list) else int(v or 0)  # noqa: E731
    rv = d.get("reviewed")
    tot["reviewed"] += len(rv) if isinstance(rv, list) else int(rv or len(s["beads"]))
    tot["changed"] += n_changed; tot["new"] += n(nb); tot["superseded"] += n(sp); tot["requests"] += n(rq)
    per_slice.append({"slice": s["slice"], "beads": len(s["beads"]), "changed": n_changed, "new": n(nb), "superseded": n(sp), "requests": n(rq)})

summary = {"round": rnd, "slices": len(slices), "slices_without_result": missing, "live_beads": len(live), "superseded_total": len(beads) - len(live),
           "totals": dict(tot), "kinds": dict(kinds.most_common()), "definition_problems": len(problems), "validation_failures": bad[:200],
           "per_slice": per_slice}
(R / "summary.json").write_text(json.dumps(summary, indent=1, ensure_ascii=False), encoding="utf-8")
import hashlib  # noqa: E402
snap = {k: hashlib.sha256(Path(b["path"]).read_bytes()).hexdigest() for k, b in live.items()}
prev_f = BEADS / "polish" / ("round-%d" % (rnd - 1)) / "hashes.json"
if prev_f.exists() and "--no-snapshot" not in sys.argv:
    prev = json.loads(prev_f.read_text(encoding="utf-8"))
    changed_now = sorted(k for k, h in snap.items() if prev.get(k) != h)
    (R / "changed.json").write_text(json.dumps(changed_now), encoding="utf-8")
    (R / "hashes.json").write_text(json.dumps(snap), encoding="utf-8")
    print("by content: %d beads differ from the end of round %d (%d of them new); %d identical" % (
        len(changed_now), rnd - 1, sum(1 for k in changed_now if k not in prev), len(snap) - len(changed_now)))
print("round %d: %d slices (%d without a result), %d live beads, %d superseded in all" % (rnd, len(slices), len(missing), len(live), len(beads) - len(live)))
print("reviewed %d, changed %d (%.0f%%), new %d, merged %d, requests %d" % (tot["reviewed"], tot["changed"], 100.0 * tot["changed"] / max(1, tot["reviewed"]), tot["new"], tot["superseded"], tot["requests"]))
print("kinds:", dict(kinds.most_common(14)))
print("definition problems: %d   validation failures: %d" % (len(problems), len(bad)))
for k, why in bad[:25]:
    print("   ", k, "|", why)
quiet = [p["slice"] for p in per_slice if p["changed"] == 0 and p["new"] == 0 and p["superseded"] == 0]
print("slices with no change at all:", len(quiet), quiet[:20])
