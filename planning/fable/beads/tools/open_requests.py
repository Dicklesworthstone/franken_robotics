#!/usr/bin/env python3
"""Wrap-up of polishing: collect every review note that no reviewer applied, per bead.

  open_requests.py <round> [--label]

A note is open when it is addressed to a bead whose slice of <round> was never reviewed, or was made after the
bead's slice had been reviewed (so nobody was shown it). Writes polish/round-<N>/open_requests.json
({bead key: [note, ...]}). With --label, adds the label `review-notes-open` to the staging file of each such bead
(and removes it from beads that have none), so that the tracker can list them.
"""
import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

rnd = int(sys.argv[1])
R = BEADS / "polish" / ("round-%d" % rnd)
P = BEADS / "polish" / ("round-%d" % (rnd - 1))
LABEL = "review-notes-open"
beads, _ = load_all()
live = {k: b for k, b in beads.items() if not b.get("superseded_by")}


def resolve(k):
    k = str(k); seen = set()
    while k in beads and beads[k].get("superseded_by") and k not in seen:
        seen.add(k); k = beads[k]["superseded_by"]
    return k if k in live else None


def requests_of(round_no):
    for f in sorted((BEADS / "polish" / ("round-%d" % round_no) / "out").glob("*.json")):
        try:
            d = json.loads(f.read_text(encoding="utf-8"))
        except Exception:  # noqa: BLE001
            continue
        for r in d.get("requests", []) or []:
            if not isinstance(r, dict):
                continue
            k = r.get("bead") or r.get("to") or r.get("target")
            what = r.get("what") or r.get("request") or r.get("why") or ""
            tag = hashlib.sha1(("%d|%s|%s|%s" % (round_no, f.stem, r.get("bead"), r.get("what"))).encode("utf-8")).hexdigest()
            targets = []
            for t in (k if isinstance(k, list) else [k]):
                t = str(t)
                if t in beads:
                    targets.append(t)
                elif re.search(r"VERBS-(?:<cluster>|\*)", t):
                    targets += sorted(x for x in live if x.startswith("VERBS-"))
                else:
                    targets += [x for x in re.findall(r"[A-Z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)+", t) if x in beads] or [t]
            for t in dict.fromkeys(targets):
                yield tag + ":" + str(t), tag, resolve(t), "request from round %d, slice %s: %s" % (round_no, f.stem, what)


def load_set(p):
    return set(json.loads(p.read_text(encoding="utf-8"))) if p.exists() else set()


delivered_prev = load_set(P / "delivered.json")
delivered = load_set(R / "delivered.json")
slices = json.loads((R / "slices.json").read_text(encoding="utf-8"))
pending = {k for s in slices if not (R / "out" / (s["slice"] + ".json")).exists() for k in s["beads"]}

notes = defaultdict(list)
src = defaultdict(int)
for full, tag, target, text in requests_of(rnd - 1):          # left over from the round before
    if target and tag not in delivered_prev and full not in delivered_prev and target in pending:
        notes[target].append(text); src["earlier round, bead not re-read"] += 1
for full, tag, target, text in requests_of(rnd):
    if not target:
        continue
    if target in pending:
        notes[target].append(text); src["this round, bead not re-read"] += 1
    elif full not in delivered:
        notes[target].append(text); src["this round, made after the bead was re-read"] += 1
for name, label in (("lint_findings.json", "mechanical check of the conventions (can be a false alarm)"),
                    ("lead_notes.json", "note from the lead"), ("lead_notes_wrapup.json", "note from the lead"),
                    ("lint_findings_2.json", "mechanical check of the conventions (can be a false alarm)"),
                    ("lead_notes_graph.json", "note from the lead, from the audit of the dependency graph"),
                    ("lint_findings_3.json", "mechanical check of the conventions (can be a false alarm)"), ("lead_notes_late.json", "note from the lead")):
    f = R / name
    if not f.exists():
        continue
    for k, items in json.loads(f.read_text(encoding="utf-8")).items():
        t = resolve(k)
        for i, item in enumerate(items):
            if not t or (name == "lint_findings.json" and "[C13-pending]" in item):
                continue
            if t in pending or ("%s:%s:%d" % (name, k, i)) not in delivered:
                notes[t].append("%s: %s" % (label, item)); src[label.split(" (")[0]] += 1

out = {k: list(dict.fromkeys(v)) for k, v in sorted(notes.items())}
(R / "open_requests.json").write_text(json.dumps(out, indent=0, ensure_ascii=False), encoding="utf-8")
print("beads with open review notes: %d of %d live; notes: %d" % (len(out), len(live), sum(len(v) for v in out.values())))
print("  beads never re-read this round: %d (of them with notes: %d)" % (len(pending), sum(1 for k in out if k in pending)))
for s, n in sorted(src.items(), key=lambda x: -x[1]):
    print("  %-48s %d" % (s, n))

if "--label" in sys.argv:
    added = removed = 0
    for k, b in live.items():
        p = Path(b["path"])
        lines = p.read_text(encoding="utf-8").split("\n")
        for i, ln in enumerate(lines[:30]):
            if ln.startswith("labels:"):
                labs = json.loads(ln[len("labels:"):].strip())
                want = k in out
                if want and LABEL not in labs:
                    labs.append(LABEL); added += 1
                elif not want and LABEL in labs:
                    labs.remove(LABEL); removed += 1
                else:
                    break
                lines[i] = "labels: " + json.dumps(labs, ensure_ascii=False)
                p.write_text("\n".join(lines), encoding="utf-8")
                break
    print("label %s: added to %d staging files, removed from %d" % (LABEL, added, removed))
