#!/usr/bin/env python3
"""Mechanical registers from the part files: gates with their chapter, refusal codes, open questions, verbs."""
import re
import sys
from collections import OrderedDict, defaultdict
from pathlib import Path

parts_dir = Path(sys.argv[1])
out_dir = Path(sys.argv[2])
parts = sorted(parts_dir.glob("P*.md"))


def cell_split(row):
    return [c.strip() for c in row.strip().strip("|").split("|")]


gates = OrderedDict()      # id -> list of (chapter, subsection, row)
refusals = OrderedDict()   # code -> (chapter, row)
opens = OrderedDict()
verbs = defaultdict(set)   # verb -> chapters
chapter_titles = {}

for p in parts:
    lines = p.read_text(encoding="utf-8").split("\n")
    chap, sub = None, None
    for line in lines:
        if line.strip().startswith("<!-- LEDGER"):
            break
        m = re.match(r"^## (\d+)\. (.*)", line)
        if m:
            chap, sub = int(m.group(1)), None
            chapter_titles[chap] = m.group(2).strip()
            continue
        m = re.match(r"^#{3,4} (\d+\.\d+(?:\.\d+)?)\s", line)
        if m:
            sub = m.group(1)
        s = line.strip()
        m = re.match(r"^\|\s*(✖\s*)?`?((?:GATE|KILL|INT)-[A-Z0-9-]+?)`?\s*\|", s)
        if m and chap is not None:
            gid = m.group(2)
            gates.setdefault(gid, []).append((chap, sub, s))
        if s.startswith("|") and "FROB-REF-" in s and chap is not None:
            cs = cell_split(s)
            for i, c in enumerate(cs):
                mm = re.fullmatch(r"`?(FROB-REF-\d{3})`?", c)
                if mm and i + 1 < len(cs):
                    # meaning = following cells up to the next code cell
                    rest = []
                    for c2 in cs[i + 1:]:
                        if re.fullmatch(r"`?FROB-REF-\d{3}`?", c2):
                            break
                        rest.append(c2)
                    refusals.setdefault(mm.group(1), (chap, sub, "| " + mm.group(1) + " | " + " | ".join(rest) + " |"))
        m = re.match(r"^- \*\*(OPEN-[A-Z0-9]+-\d+)", s)
        if m and chap is not None:
            opens.setdefault(m.group(1), (chap, sub, s))
        m = re.match(r"^\|\s*`?(OPEN-[A-Z0-9]+-\d+)`?\s*\|", s)
        if m and chap is not None:
            opens.setdefault(m.group(1), (chap, sub, s))
        if chap is not None:
            for vm in re.finditer(r"`((?:design|analyze|scene|control|sim|train|eval|telemetry|explain|opt|predict|prove|spark|job|report|"
                                  r"mission|study|cert|result|experiment|run|autopsy|nemesis|ground|foundry|sense|session|solve|gait|skill|"
                                  r"shield|phylum|family|policy|clear|retrofit|lineage|ledger|card|module|task|qual)\.[a-z][a-z0-9_-]+)`", line):
                verbs[vm.group(1)].add(chap)

# ---- gates inventory
out = ["# Gate inventory (mechanical extraction from the Phase 1 part files)\n",
       f"Total identifiers: {len(gates)}. A gate defined in two chapters appears twice; the chapter that owns the mechanism governs.\n",
       "| ID | § | Definition row (cells joined, truncated) |", "|---|---|---|"]
for gid in sorted(gates, key=lambda g: (g.split('-')[0], g)):
    for chap, sub, row in gates[gid]:
        cells = cell_split(row)[1:]
        txt = " ¦ ".join(cells)[:420]
        out.append(f"| {gid} | {sub or chap} | {txt} |")
(out_dir / "GATES_INVENTORY.md").write_text("\n".join(out) + "\n", encoding="utf-8")

# ---- refusal registry (this is final appendix text)
fam_names = {1: "Design and model", 2: "Numerics and integrator admission", 3: "Scene", 4: "Control and estimation",
             5: "Learning and evaluation protocol", 6: "Backend and device", 7: "Budget and capability",
             8: "Evidence and protocol", 9: "Internal"}
app = ["## Appendix O. Refusal code registry\n",
       "Every refusal has a stable code, a message and ranked repairs (§35). This registry is compiled from the chapters",
       "that define the codes; the chapter governs the meaning and the repairs. The machine-readable form is",
       "`registries/refusals.toml`, and an unknown code is a load-time error. Codes are never reused.\n"]
by_fam = defaultdict(list)
for code, (chap, sub, row) in refusals.items():
    by_fam[int(code[-3])].append((code, chap, sub, row))
for fam in sorted(by_fam):
    app.append(f"### O.{fam} `{fam}xx` {fam_names.get(fam, '')}\n")
    app.append("| Code | Meaning | Defined in |")
    app.append("|---|---|---|")
    for code, chap, sub, row in sorted(by_fam[fam]):
        cells = cell_split(row)
        meaning = cells[1] if len(cells) > 1 else ""
        if len(cells) > 2 and len(meaning) < 34 and cells[2] and not cells[2].isdigit():
            meaning = meaning + ": " + cells[2]
        meaning = meaning[:260]
        app.append(f"| {code} | {meaning} | §{sub or chap} |")
    app.append("")
(out_dir / "APPENDIX_O_REFUSALS.md").write_text("\n".join(app) + "\n", encoding="utf-8")

# ---- open questions and verbs inventories
oq = ["# Open questions inventory\n", "| ID | § | Text (truncated) |", "|---|---|---|"]
for oid, (chap, sub, row) in sorted(opens.items()):
    oq.append(f"| {oid} | {sub or chap} | {row.replace('|', '¦')[:330]} |")
(out_dir / "OPENS_INVENTORY.md").write_text("\n".join(oq) + "\n", encoding="utf-8")

vb = ["# Verb inventory (every `cluster.verb` in backticks, with the chapters that mention it)\n", "| Verb | Chapters |", "|---|---|"]
for v in sorted(verbs):
    vb.append(f"| `{v}` | {', '.join('§' + str(c) for c in sorted(verbs[v]))} |")
(out_dir / "VERBS_INVENTORY.md").write_text("\n".join(vb) + "\n", encoding="utf-8")

print("gates/kills/ints:", len(gates), "rows:", sum(len(v) for v in gates.values()))
print("refusal codes:", len(refusals), "open questions:", len(opens), "verbs:", len(verbs))
print("chapters:", sorted(chapter_titles))
