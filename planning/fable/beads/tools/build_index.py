#!/usr/bin/env python3
"""Build the global bead index, apply nothing, and write one integration worksheet per group.

Run:  python3 tools/build_index.py
Writes: INDEX.tsv, integration/groups.json, integration/auto_edges.json, integration/worksheets/<group>.md
"""
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

INT = BEADS / "integration"
(INT / "worksheets").mkdir(parents=True, exist_ok=True)
(INT / "out").mkdir(parents=True, exist_ok=True)

beads, problems = load_all()
keys = set(beads)
MS = {"M%d" % i: i for i in range(9)}


def group_of(unit):
    m = re.match(r"^U(\d\d)([a-e]?)$", unit)
    if m:
        c, part = m.group(1), m.group(2)
        if c == "25":
            return "G25ab" if part in ("a", "b") else "G25cde"
        return "G" + c
    return "G" + unit[1:]


groups = defaultdict(list)
for b in beads.values():
    b["group"] = group_of(b["unit"])
    if b["unit"] not in groups[b["group"]]:
        groups[b["group"]].append(b["unit"])
for g in groups:
    groups[g].sort()

# ------------------------------------------------------------------ index
rows = ["key\tunit\ttype\tmilestone\tpriority\tparent\tgates\tplan_refs\ttitle"]
for k in sorted(beads):
    b = beads[k]
    if b.get("superseded_by"):
        continue
    rows.append("\t".join([k, b["unit"], str(b.get("type")), str(b.get("milestone")), str(b.get("priority")),
                           str(b.get("parent") or ""), ",".join(b["gates"]), ",".join(b["plan_refs"]),
                           b["title"].replace("\t", " ")]))
(BEADS / "INDEX.tsv").write_text("\n".join(rows) + "\n", encoding="utf-8")

# ------------------------------------------------------------------ section and gate maps
SEC = re.compile(r"§(\d+(?:\.\d+)*)")
by_sec = defaultdict(set)
by_gate = defaultdict(set)
for k, b in beads.items():
    if b.get("superseded_by"):
        continue
    for r in b["plan_refs"]:
        for s in SEC.findall(str(r)):
            by_sec[s].add(k)
    for g in b["gates"]:
        by_gate[g].add(k)

KEY = re.compile(r"(?<![A-Za-z0-9-])([A-Z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)+)(?![A-Za-z0-9-])")
GATE = re.compile(r"(?:GATE|KILL|INT)-[A-Z0-9]+(?:-[A-Z0-9]+)*")
STOP = set("the a an of and or to in for with on by its is are as that which this at from be it into per each "
           "their not no than then when where under over one two bead beads unit units".split())


def toks(text):
    return {w for w in re.findall(r"[a-z][a-z0-9_.-]{2,}", text.lower()) if w not in STOP}


auto = []          # mechanical edges from needs that name an existing key
work = defaultdict(list)   # group -> list of (bead, n, need, candidates)
stats = Counter()
for k in sorted(beads):
    b = beads[k]
    if b.get("superseded_by"):
        continue
    for n, need in enumerate(b["needs"], 1):
        named = list(dict.fromkeys(x for x in KEY.findall(need) if x in keys and x != k))
        if named:
            stats["named"] += 1
            work[b["group"]].append((k, n, need, named, True))
            stats["sheet"] += 1
            continue
        secs = SEC.findall(need)
        cand = Counter()
        for s in secs:
            for c in by_sec.get(s, ()):
                cand[c] += 6
            parent = s.rsplit(".", 1)[0] if "." in s else None
            if parent and len(by_sec.get(s, ())) < 3:
                for c in by_sec.get(parent, ()):
                    cand[c] += 2
            for s2, ks in by_sec.items():
                if s2.startswith(s + "."):
                    for c in ks:
                        cand[c] += 3
        for g in GATE.findall(need):
            for c in by_gate.get(g, ()):
                cand[c] += 5
        nt = toks(need)
        for c in list(cand):
            if c == k:
                del cand[c]; continue
            cb = beads[c]
            cand[c] += 1.5 * len(nt & toks(cb["title"]))
            if re.search(r"-(KA\d?|CX|REF)$", c):
                cand[c] -= 4
        top = [c for c, _ in cand.most_common(10)]
        work[b["group"]].append((k, n, need, top, False))
        stats["sheet"] += 1
        if not top:
            stats["no_candidates"] += 1

(INT / "auto_edges.json").write_text(json.dumps(auto, indent=0, ensure_ascii=False), encoding="utf-8")
(INT / "groups.json").write_text(json.dumps(groups, indent=1), encoding="utf-8")

for g, units in sorted(groups.items()):
    out = ["# Integration worksheet for group %s (units %s)" % (g, ", ".join(units)), ""]
    mine = [b for b in beads.values() if b["group"] == g and not b.get("superseded_by")]
    out.append("## Beads in this group (%d)" % len(mine))
    out.append("")
    out.append("| Key | Unit | Milestone | Type | Title |")
    out.append("|---|---|---|---|---|")
    for b in sorted(mine, key=lambda x: (x["unit"], x["key"])):
        out.append("| %s | %s | %s | %s | %s |" % (b["key"], b["unit"], b.get("milestone"), b.get("type"), b["title"].replace("|", "/")))
    out.append("")
    out.append("## Prerequisites to resolve (%d)" % len(work.get(g, [])))
    out.append("")
    out.append("Each entry: the bead that needs something, the need as its drafter wrote it, and candidate carriers. Where the")
    out.append("need names its carrier by key, the keys named are listed: confirm each and decide dep, related or none. Otherwise")
    out.append("the candidates were found mechanically (by plan section, gate and title words): suggestions, not answers.")
    out.append("")
    last = None
    for k, n, need, top, is_named in work.get(g, []):
        if k != last:
            out.append("### %s (%s) %s" % (k, beads[k].get("milestone"), beads[k]["title"]))
            last = k
        out.append("- **need %d**%s: %s" % (n, " (names its carrier)" if is_named else "", need))
        if top:
            for c in top:
                cb = beads[c]
                out.append("  - `%s` (%s, %s, %s) %s" % (c, cb.get("milestone"), cb["unit"], cb.get("type"), cb["title"]))
        else:
            out.append("  - (no mechanical candidates: search INDEX.tsv)")
    (INT / "worksheets" / (g + ".md")).write_text("\n".join(out) + "\n", encoding="utf-8")

print("beads indexed:", len(rows) - 1, " groups:", len(groups))
print("needs on worksheets %d (naming a key: %d; without candidates: %d)" % (stats["sheet"], stats["named"], stats["no_candidates"]))
for g, units in sorted(groups.items()):
    n = sum(1 for b in beads.values() if b["group"] == g)
    sz = (INT / "worksheets" / (g + ".md")).stat().st_size
    print("  %-7s units=%-22s beads=%3d needs=%3d sheet=%dKB" % (g, ",".join(units), n, len(work.get(g, [])), sz // 1024))
