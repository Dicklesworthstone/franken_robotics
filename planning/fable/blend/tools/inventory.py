#!/usr/bin/env python3
"""Build a consolidated ledger, an identifier inventory and a collision report from the plan part files."""
import re
import sys
from collections import defaultdict
from pathlib import Path

parts_dir = Path(sys.argv[1])
out_dir = Path(sys.argv[2])
parts = sorted(parts_dir.glob("P*.md"))

ID_PATTERNS = [
    ("GATE", r"GATE-[A-Z0-9]+-\d+[a-z]?"),
    ("KILL", r"KILL-[A-Z0-9-]+"),
    ("INT", r"INT-[A-Z0-9]+-\d+"),
    ("OPEN", r"OPEN-[A-Z0-9]+-\d+"),
    ("REF", r"FROB-REF-\d{3}"),
    ("PO", r"PO-[A-Z0-9]+-\d+"),
    ("INV", r"INV-[A-Z0-9]+-\d+"),
    ("ADR", r"ADR-[A-Z0-9]+-\d+"),
    ("BET", r"BET-\d{2}"),
    ("GOAL", r"(?<!NON)GOAL-\d{2}"),
    ("NS", r"(?<![A-Z-])NS-\d{2}"),
    ("C", r"(?<![A-Z-])C-\d{2}"),
    ("T", r"(?<![A-Z-])T-\d{2}"),
    ("A", r"(?<![A-Z-])A-\d{2}"),
    ("DC", r"(?<![A-Z-])DC-\d{2}"),
    ("GB", r"(?<![A-Z-])GB-\d{2}"),
    ("FMT", r"FMT-[A-Z0-9-]+"),
    ("L", r"(?<![A-Z-])L-\d{1,2}(?!\d)"),
    ("D", r"(?<![A-Z-])D-\d{1,2}(?!\d)"),
]


def is_def(ident, line):
    s = line.strip()
    if re.match(r"^\|\s*(✖\s*)?(\*\*)?`?" + re.escape(ident) + r"`?(\*\*)?\s*[,|]", s):
        return True
    if re.match(r"^\|\s*(✖\s*)?(\*\*)?`?" + re.escape(ident) + r"`?(\*\*)?\s", s) and s.count("|") >= 3:
        # first cell begins with the id (allow trailing words such as a title)
        first = s.split("|")[1]
        return first.strip().lstrip("✖ *`").startswith(ident)
    if s.startswith("#") and ident in s:
        return True
    if re.match(r"^(-\s+)?\*\*`?" + re.escape(ident) + r"`?\b", s):
        return True
    return False


ledgers = []
defs = defaultdict(list)  # ident -> [(part, line_no, text)]
refs = defaultdict(set)   # ident -> {part}
for p in parts:
    text = p.read_text(encoding="utf-8")
    lines = text.split("\n")
    body_end = len(lines)
    for i, l in enumerate(lines):
        if l.strip().startswith("<!-- LEDGER"):
            body_end = i
            break
    ledgers.append(f"\n\n# ===== {p.name} =====\n" + "\n".join(lines[body_end:]))
    for n, line in enumerate(lines[:body_end], 1):
        for kind, pat in ID_PATTERNS:
            for m in re.finditer(pat, line):
                ident = m.group(0)
                refs[ident].add(p.name)
                if is_def(ident, line):
                    defs[ident].append((p.name, n, line.strip()[:150]))

(out_dir / "LEDGERS.md").write_text("# Consolidated LEDGER blocks from the Phase 1 part files\n" + "".join(ledgers), encoding="utf-8")

# inventory of gates / kills / ints, grouped
inv = ["# Gate inventory extracted from Phase 1 part files (definition rows)\n",
       "| ID | Part | Line | Row text (truncated) |", "|---|---|---|---|"]
for ident in sorted(defs):
    if ident.startswith(("GATE-", "KILL-", "INT-")):
        for part, n, t in defs[ident]:
            inv.append(f"| {ident} | {part} | {n} | {t.replace('|', '¦')[:130]} |")
(out_dir / "GATES_INVENTORY.md").write_text("\n".join(inv) + "\n", encoding="utf-8")

rep = ["# Identifier collision and reference report\n"]
rep.append("## Identifiers defined in more than one part (possible collisions)\n")
for ident in sorted(defs):
    ps = sorted({d[0] for d in defs[ident]})
    if len(ps) > 1:
        rep.append(f"- **{ident}**")
        for part, n, t in defs[ident]:
            rep.append(f"    - {part}:{n}: {t[:120]}")
rep.append("\n## Counts of definitions by kind\n")
kinds = defaultdict(int)
for ident in defs:
    k = re.match(r"[A-Z]+(-REF)?", ident).group(0)
    kinds[k] += 1
for k in sorted(kinds):
    rep.append(f"- {k}: {kinds[k]}")
rep.append("\n## Referenced but never defined in any Phase 1 part (GATE/KILL/INT/OPEN/REF/PO/INV/ADR/C/T/A/DC/BET/GOAL/NS/FMT)\n")
for ident in sorted(refs):
    if ident not in defs and not ident.startswith(("GB-", "L-", "D-")):
        rep.append(f"- {ident}: referenced in {sorted(refs[ident])}")
(out_dir / "COLLISIONS.md").write_text("\n".join(rep) + "\n", encoding="utf-8")

print("parts:", len(parts))
print("defined ids:", len(defs), " referenced ids:", len(refs))
multi = [i for i in defs if len({d[0] for d in defs[i]}) > 1]
print("defined in >1 part:", len(multi))
for k in sorted(kinds):
    print(f"  {k}: {kinds[k]}")
undefined = [i for i in refs if i not in defs and not i.startswith(("GB-", "L-", "D-"))]
print("referenced-never-defined:", len(undefined))
