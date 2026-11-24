#!/usr/bin/env python3
"""Cut a frontier round of polishing.

  make_frontier_round.py <round> hubs     the registry and owner beads, with every request addressed to them
  make_frontier_round.py <round> rest     every other bead still on the frontier, by chapter unit

The frontier of round N is every live bead that round N-1 changed or created, or that has an unanswered request.
A bead leaves the frontier when a reviewer reads it in full and changes nothing; it comes back when it is edited
or a request names it. Steady state is an empty (or nearly empty) frontier, confirmed by a final full sweep.

`hubs` must be run, reviewed and finished before `rest` is cut: `rest` carries the requests the hub reviewers made.
"""
import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

rnd = int(sys.argv[1]); phase = sys.argv[2]
MAX = int(sys.argv[sys.argv.index("--max") + 1]) if "--max" in sys.argv else 46
R = BEADS / "polish" / ("round-%d" % rnd)
P = BEADS / "polish" / ("round-%d" % (rnd - 1))
(R / "out").mkdir(parents=True, exist_ok=True)

beads, _ = load_all()
live = {k: b for k, b in beads.items() if not b.get("superseded_by")}

OWNERS = {"N40-03", "N35x-02", "N35-33", "BD-025", "NJL-01", "NJL-05", "N38a-09", "N38a-40", "N41-16", "N41-17", "N44-01",
          "STD-E2E", "STD-TEST", "N38a-02", "N38a-12", "N38b-19", "N40-01", "N40-26"}


def is_hub(k):
    return bool(re.match(r"^VERBS-", k) or re.search(r"-(REF|KA\d?|CX)$", k)) or k in OWNERS


def resolve(k):
    k = str(k)
    seen = set()
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
                else:                                  # "A, B and C" in one string
                    targets += [x for x in re.findall(r"[A-Z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)+", t) if x in beads] or [t]
            for t in dict.fromkeys(targets):
                yield tag + ":" + str(t), tag, resolve(t), "request from round %d, slice %s: %s" % (round_no, f.stem, what)


def load_set(p):
    return set(json.loads(p.read_text(encoding="utf-8"))) if p.exists() else set()


delivered_prev = load_set(P / "delivered.json")       # tags delivered inside the previous round
delivered = load_set(R / "delivered.json")            # tag:target delivered in this round
inbox = defaultdict(list)
for full, tag, target, text in requests_of(rnd - 1):
    if target and tag not in delivered_prev and full not in delivered_prev:
        inbox[target].append((full, text))
for name, label in (("lint_findings.json", "mechanical check of the conventions"), ("lead_notes.json", "note from the lead")):
    f = R / name                                      # {bead key: [text, ...]} written before the round is cut
    if f.exists():
        for k, items in json.loads(f.read_text(encoding="utf-8")).items():
            t = resolve(k)
            for i, item in enumerate(items):
                if t:
                    inbox[t].append(("%s:%s:%d" % (name, k, i), "%s: %s" % (label, item)))
for full, tag, target, text in requests_of(rnd):      # made earlier in this round (by the hub reviewers)
    if target:
        inbox[target].append((full, text))

changed_prev = load_set(P / "changed.json")
prev_keys = set(json.loads((P / "hashes.json").read_text(encoding="utf-8"))) if (P / "hashes.json").exists() else set(live)
frontier = {k for k in live if k in changed_prev or k not in prev_keys or k in inbox}

SL = R / "slices.json"
slices = json.loads(SL.read_text(encoding="utf-8")) if SL.exists() else []
placed = {k for s in slices for k in s["beads"]}


def chunks(keys, name, cap):
    keys = sorted(keys)
    n = max(1, -(-len(keys) // cap))
    size = -(-len(keys) // n)
    return [("%s%s" % (name, "" if n == 1 else "-%d" % (i + 1)), keys[i * size:(i + 1) * size]) for i in range(n)]


new = []
if phase == "hubs":
    hubs = sorted(k for k in live if is_hub(k) and (k in frontier))
    groups = defaultdict(list)
    for k in hubs:
        if k.startswith("VERBS-"):
            groups["H-verbs"].append(k)
        elif k in OWNERS:
            groups["H-owners"].append(k)
        elif k.endswith("-REF"):
            groups["H-ref"].append(k)
        elif k.endswith("-CX"):
            groups["H-cx"].append(k)
        else:
            groups["H-ka"].append(k)
    for g in sorted(groups):
        ks = [k for k in groups[g] if k not in placed]
        if not ks:
            continue
        # balance by request load: a bead with many requests is heavy
        ks.sort()
        cur, load, i = [], 0, 1
        for k in ks:
            w = 1 + 0.5 * len(inbox.get(k, []))
            if cur and (load + w > 24 or len(cur) >= 14):
                new.append(("%s-%d" % (g, i), cur, "hubs")); cur, load, i = [], 0, i + 1
            cur.append(k); load += w
        if cur:
            new.append(("%s-%d" % (g, i), cur, "hubs"))
elif phase == "rest":
    rest = [k for k in frontier if k not in placed]
    by_unit = defaultdict(list)
    for k in rest:
        by_unit[live[k]["unit"]].append(k)
    for u in sorted(by_unit):
        for name, ks in chunks(by_unit[u], "F-" + u, MAX):
            new.append((name, ks, "rest"))
    # small leftovers ride with a neighbour of the same chapter
    merged = []
    for name, ks, ph in new:
        if merged and len(ks) < 8 and len(merged[-1][1]) + len(ks) <= MAX + 6 and merged[-1][0][:5] == name[:5]:
            merged[-1] = (merged[-1][0], merged[-1][1] + ks, ph)
        else:
            merged.append((name, ks, ph))
    new = merged

if phase == "mopup":     # hub beads that reviewers of this same round addressed after their own slice was cut
    pend = defaultdict(list)
    for k, items in inbox.items():
        if is_hub(k) and k in placed:
            pend[k] = [(full, text) for full, text in items if full not in delivered]
    pend = {k: v for k, v in pend.items() if v}
    n_pass = 1 + len({x["slice"].split("-")[0] for x in slices if x["origin"] == "mopup"})
    cur, load, i = [], 0, 1
    for k in sorted(pend):
        w = 1 + 0.5 * len(pend[k])
        if cur and (load + w > 22 or len(cur) >= 12):
            new.append(("M%d-%d" % (n_pass, i), cur, "mopup")); cur, load, i = [], 0, i + 1
        cur.append(k); load += w
    if cur:
        new.append(("M%d-%d" % (n_pass, i), cur, "mopup"))
    inbox = pend         # a mop-up slice lists only what its beads have not yet been shown

if phase == "refresh":   # slices not yet reviewed get the requests made since they were cut
    # --slices a,b,c : also refresh these, whose result file is partial (an interrupted reviewer's)
    FORCE = set(sys.argv[sys.argv.index("--slices") + 1].split(",")) if "--slices" in sys.argv else set()
    done = lambda s: (R / "out" / (s["slice"] + ".json")).exists() and s["slice"] not in FORCE  # noqa: E731
    new = [(s["slice"], [k for k in s["beads"] if k in live], s["origin"]) for s in slices if not done(s)]
    slices = [s for s in slices if done(s)]

for name, ks, ph in new:
    out = ["# Polishing round %d, slice %s" % (rnd, name), "", "%d beads." % len(ks), "",
           "| Key | Milestone | Type | Parent | Changed last round | Title | File |", "|---|---|---|---|---|---|---|"]
    for k in ks:
        b = live[k]
        out.append("| %s | %s | %s | %s | %s | %s | `%s` |" % (k, b.get("milestone"), b.get("type"), b.get("parent") or "",
                                                              "yes" if (k in changed_prev or k not in prev_keys) else "no (here for a request)",
                                                              str(b["title"]).replace("|", "/"), b["path"]))
    out += ["", "## Addressed to your beads", ""]
    n = 0
    for k in ks:
        for full, text in inbox.get(k, []):
            out.append("- `%s`: %s" % (k, text)); delivered.add(full); n += 1
    if not n:
        out.append("Nothing.")
    (R / (name + ".md")).write_text("\n".join(out) + "\n", encoding="utf-8")
    slices.append({"slice": name, "beads": ks, "origin": ph})

if phase == "refresh":
    order = {s["slice"]: i for i, s in enumerate(json.loads(SL.read_text(encoding="utf-8")))}
    slices.sort(key=lambda s: order.get(s["slice"], 1e9))
SL.write_text(json.dumps(slices, indent=0), encoding="utf-8")
(R / "delivered.json").write_text(json.dumps(sorted(delivered)), encoding="utf-8")
sizes = [len(ks) for _, ks, _ in new] or [0]
print("round %d %s: frontier %d of %d live beads; %d new slices over %d beads (sizes %d..%d); requests delivered so far %d"
      % (rnd, phase, len(frontier), len(live), len(new), sum(sizes), min(sizes), max(sizes), len(delivered)))
print(" ".join("%s:%d" % (n, len(ks)) for n, ks, _ in new))
