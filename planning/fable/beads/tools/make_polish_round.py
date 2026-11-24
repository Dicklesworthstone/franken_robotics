#!/usr/bin/env python3
"""Cut the live beads into review slices for one polishing round and write each slice's file.

  make_polish_round.py <round> <mode> [--max N] [--only-active]

mode: unit      slices follow the drafting units (chapter view)
      package   slices follow the work-package epics, ordered by milestone (execution view)
      milestone slices follow milestone, then subsystem label (cross-chapter view)
--only-active: leave out slices of the previous round that reported no change at all for two rounds running.

Each slice file lists its beads and carries: requests addressed to those beads by the previous round (for round 1:
the conflicts the reconciliation pass could not fix), and the graph tool's findings about them.
"""
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

rnd = int(sys.argv[1]); mode = sys.argv[2]
MAX = int(sys.argv[sys.argv.index("--max") + 1]) if "--max" in sys.argv else 60
OUT = BEADS / "polish" / ("round-%d" % rnd)
(OUT / "out").mkdir(parents=True, exist_ok=True)

beads, _ = load_all()
live = {k: b for k, b in beads.items() if not b.get("superseded_by")}
MS = {"M%d" % i: i for i in range(9)}


def chunks(keys, name):
    keys = sorted(keys)
    n = max(1, -(-len(keys) // MAX))
    size = -(-len(keys) // n)
    return [("%s%s" % (name, "" if n == 1 else "-%d" % (i + 1)), keys[i * size:(i + 1) * size]) for i in range(n)]


groups = defaultdict(list)
if mode == "unit":
    for k, b in live.items():
        groups[b["unit"]].append(k)
elif mode == "package":
    for k, b in live.items():
        groups[str(b.get("parent") or "none")].append(k)
elif mode == "milestone":
    SUB = ["marrow", "skeleton", "sinew", "kinesis", "phylum", "traction", "habitat", "sensorium", "spark", "reflex", "cortex", "arena",
           "augur", "nemesis", "autopsy", "atelier", "chronicle", "ground", "foundry", "tether", "lemma", "gauntlet", "suite", "workspace",
           "safety", "tenfold", "delivery", "spine"]
    for k, b in live.items():
        sub = next((s for s in SUB if s in [str(x).lower() for x in b.get("labels", [])]), "other")
        groups["%s-%s" % (b.get("milestone"), sub)].append(k)

slices = []
if mode == "unit":
    for g in sorted(groups):
        slices += chunks(groups[g], g)
else:
    # pack small groups together (same milestone first) up to MAX; split big ones
    def gkey(g):
        ks = groups[g]
        return (min(MS.get(str(live[k].get("milestone")), 9) for k in ks), g)
    cur, cur_names = [], []
    for g in sorted(groups, key=gkey):
        ks = sorted(groups[g])
        if len(ks) >= MAX:
            if cur:
                slices.append(("+".join(cur_names), cur)); cur, cur_names = [], []
            slices += chunks(ks, g)
            continue
        if len(cur) + len(ks) > MAX:
            slices.append(("+".join(cur_names), cur)); cur, cur_names = [], []
        cur += ks; cur_names.append(g)
    if cur:
        slices.append(("+".join(cur_names), cur))
    slices = [("S%02d" % i, ks, name) for i, (name, ks) in enumerate(slices, 1)]
if mode == "unit":
    slices = [(name, ks, name) for name, ks in slices]

SL = OUT / "slices.json"
if SL.exists():   # the partition of a round is fixed once made; a re-run only refreshes what is addressed to pending slices
    slices = [(d["slice"], [k for k in d["beads"] if k in live], d["origin"]) for d in json.loads(SL.read_text(encoding="utf-8"))]
    placed = {k for _, ks, _ in slices for k in ks}
    by_unit = defaultdict(list)
    for name, ks, _ in slices:
        for k in ks:
            by_unit[live[k]["unit"]].append(name)
    for k in sorted(set(live) - placed):   # beads written during this round join a slice of their unit that is still pending
        cands = [n for n in by_unit.get(live[k]["unit"], []) if not (OUT / "out" / (n + ".json")).exists()]
        if cands:
            for i, (name, ks, o) in enumerate(slices):
                if name == cands[0]:
                    ks.append(k)
slice_of = {}
for name, ks, _ in slices:
    for k in ks:
        slice_of[k] = name

# ---------------------------------------------------------------- inputs addressed to beads
KEY = re.compile(r"(?<![A-Za-z0-9-])([A-Z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)+)(?![A-Za-z0-9-])")
inbox = defaultdict(list)
if rnd == 1:
    f = BEADS / "integration" / "residual_conflicts.json"
    for x in (json.loads(f.read_text(encoding="utf-8")) if f.exists() else []):
        text = "%s: %s" % (x.get("bead"), x.get("what"))
        named = [k for k in dict.fromkeys(KEY.findall(text)) if k in live]
        for k in named or []:
            inbox[k].append("left by the reconciliation pass (%s): %s" % (x.get("from"), text))

import hashlib  # noqa: E402


def requests_of(round_no):
    d_out = BEADS / "polish" / ("round-%d" % round_no) / "out"
    for f in sorted(d_out.glob("*.json")):
        try:
            d = json.loads(f.read_text(encoding="utf-8"))
        except Exception:  # noqa: BLE001
            continue
        for r in d.get("requests", []) or []:
            k = str(r.get("bead"))
            target = k if k in live else beads.get(k, {}).get("superseded_by")
            tag = hashlib.sha1(("%d|%s|%s|%s" % (round_no, f.stem, k, r.get("what"))).encode("utf-8")).hexdigest()
            yield tag, target, "request from round %d, slice %s: %s" % (round_no, f.stem, r.get("what"))


def delivered_of(round_no):
    f = BEADS / "polish" / ("round-%d" % round_no) / "delivered.json"
    return set(json.loads(f.read_text(encoding="utf-8"))) if f.exists() else set()


delivered = delivered_of(rnd)
if rnd > 1:   # what the previous round asked and could not deliver inside itself
    already = delivered_of(rnd - 1)
    for tag, target, text in requests_of(rnd - 1):
        if target in live and tag not in already:
            inbox[target].append(text)
pending = {n for n, _, _ in slices if not (OUT / "out" / (n + ".json")).exists()}
for tag, target, text in requests_of(rnd):   # requests made earlier in this same round, to slices not yet reviewed
    if target in live and (slice_of.get(target) in pending or tag in delivered):
        inbox[target].append(text)
        delivered.add(tag)
(OUT / "delivered.json").write_text(json.dumps(sorted(delivered)), encoding="utf-8")

findings = defaultdict(list)
f = BEADS / "polish" / ("round-%d" % rnd) / "graph_findings.json"
if f.exists():
    for k, items in json.loads(f.read_text(encoding="utf-8")).items():
        if k in live:
            findings[k] += items

for name, ks, origin in slices:
    if (OUT / "out" / (name + ".json")).exists():
        continue   # already reviewed in this round: its file is left as the reviewer saw it
    out = ["# Polishing round %d, slice %s" % (rnd, name), "", "%d beads (%s)." % (len(ks), origin), "",
           "| Key | Milestone | Type | Parent | Title | File |", "|---|---|---|---|---|---|"]
    for k in ks:
        b = live[k]
        out.append("| %s | %s | %s | %s | %s | `%s` |" % (k, b.get("milestone"), b.get("type"), b.get("parent") or "", str(b["title"]).replace("|", "/"), b["path"]))
    out += ["", "## Addressed to your beads", ""]
    any_in = False
    for k in ks:
        for item in inbox.get(k, []):
            out.append("- `%s`: %s" % (k, item)); any_in = True
    if not any_in:
        out.append("Nothing.")
    out += ["", "## Graph findings about your beads", ""]
    any_f = False
    for k in ks:
        for item in findings.get(k, []):
            out.append("- `%s`: %s" % (k, item)); any_f = True
    if not any_f:
        out.append("None.")
    (OUT / (name + ".md")).write_text("\n".join(out) + "\n", encoding="utf-8")

SL.write_text(json.dumps([{"slice": n, "beads": ks, "origin": o} for n, ks, o in slices], indent=0), encoding="utf-8")
sizes = [len(ks) for _, ks, _ in slices]
print("round %d (%s): %d slices over %d beads; sizes min %d, median %d, max %d; inbox items %d; findings %d"
      % (rnd, mode, len(slices), len(live), min(sizes), sorted(sizes)[len(sizes) // 2], max(sizes),
         sum(len(v) for v in inbox.values()), sum(len(v) for v in findings.values())))
print(" ".join("%s:%d" % (n, len(ks)) for n, ks, _ in slices))
