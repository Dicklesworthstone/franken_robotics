#!/usr/bin/env python3
"""Extract the seed beads of §45 and map each to its owner chapter."""
import json, re
from collections import Counter, defaultdict
from pathlib import Path

blend = Path("blend")
p15 = (blend / "parts2/P15.md").read_text(encoding="utf-8").split("\n")

# gate -> chapter from all part files (definition rows or prose "**Gate.** GATE-X")
gate_ch = {}
chap_title = {}
for f in sorted((blend / "parts").glob("P*.md")) + sorted((blend / "parts2").glob("P*.md")):
    chap = None
    for line in f.read_text(encoding="utf-8").split("\n"):
        if line.startswith("<!-- LEDGER"):
            break
        m = re.match(r"^## (\d+)\. (.*)", line)
        if m:
            chap = int(m.group(1)); chap_title[chap] = m.group(2).strip(); continue
        if chap is None or chap >= 38:
            continue
        s = line.strip()
        m = re.match(r"^\|\s*(✖\s*)?`?((?:GATE|KILL|INT)-[A-Z0-9]+(?:-[A-Z0-9]+)*)`?\s*\|", s)
        if m:
            gate_ch.setdefault(m.group(2), chap)
        for m in re.finditer(r"\*\*(?:Gate|Kill gate)[^*]*\*\*\.?\s*(?:✖\s*)?((?:GATE|KILL)-[A-Z0-9]+(?:-[A-Z0-9]+)*)", s):
            gate_ch.setdefault(m.group(1), chap)

seeds = []
ms = None
for line in p15:
    m = re.match(r"^### 45\.\d+ .*?\b(M[0-8])\b", line)
    if m:
        ms = m.group(1)
    m = re.match(r"^\|\s*(BD-\d{3})\s*\|(.*)\|\s*$", line)
    if m:
        cells = [c.strip() for c in m.group(2).split("|")]
        if len(cells) < 3:
            continue
        title, deps, done = cells[0], cells[1], " | ".join(cells[2:])
        text = title + " " + done
        refs = Counter()
        for g in re.findall(r"(?:GATE|KILL|INT)-[A-Z0-9]+(?:-[A-Z0-9]+)*", text):
            if g in gate_ch:
                refs[gate_ch[g]] += 2
        for s in re.findall(r"§(\d+)(?:\.\d+)*", text):
            if 9 <= int(s) <= 47:
                refs[int(s)] += 1
        seeds.append({"bd": m.group(1), "milestone": ms, "title": title, "deps": re.findall(r"BD-\d{3}", deps),
                      "done_when": done, "chapter_refs": dict(refs),
                      "owner_chapter": (refs.most_common(1)[0][0] if refs else None)})
Path("beads/seed.json").write_text(json.dumps(seeds, indent=1, ensure_ascii=False), encoding="utf-8")
by_ch = defaultdict(list)
for s in seeds:
    by_ch[s["owner_chapter"]].append(s["bd"])
print("seed beads:", len(seeds), " milestones:", dict(Counter(s["milestone"] for s in seeds)))
for ch in sorted(by_ch, key=lambda x: (x is None, x)):
    print(f"  §{ch} {chap_title.get(ch, '')[:38]:38s} beads={len(by_ch[ch])}")
print("unmapped sample:", [(s['bd'], s['title'][:60]) for s in seeds if s['owner_chapter'] is None][:14])
