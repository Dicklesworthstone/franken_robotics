#!/usr/bin/env python3
"""Resolve cross-part collisions of FROB-REF-nnn codes by renumbering inside the losing part files."""
import re
import sys
from collections import defaultdict
from pathlib import Path

parts_dir = Path(sys.argv[1])
apply = len(sys.argv) > 2 and sys.argv[2] == "--apply"
PRIORITY = ["P02", "P03", "P04", "P05", "P06", "P08", "P07", "P09", "P10", "P11", "P12", "P01", "P00"]
SHARED = {"101", "104", "107", "110", "113", "212", "301", "502"}  # carried codes with one meaning everywhere

texts = {p.stem: p.read_text(encoding="utf-8") for p in sorted(parts_dir.glob("P*.md"))}
code_re = re.compile(r"FROB-REF-(\d{3})(?!\d)")


def defs_in(text):
    out = {}
    for n, line in enumerate(text.split("\n"), 1):
        s = line.strip()
        m = re.match(r"^\|\s*`?FROB-REF-(\d{3})`?\s*\|", s)
        if m:
            out.setdefault(m.group(1), (n, s[:110]))
    return out


defs = {k: defs_in(v) for k, v in texts.items()}
used = set()
refs = defaultdict(set)
for k, v in texts.items():
    for m in code_re.finditer(v):
        used.add(m.group(1))
        refs[m.group(1)].add(k)

by_code = defaultdict(list)
for part in PRIORITY:
    for code in defs.get(part, {}):
        by_code[code].append(part)

remap = defaultdict(dict)  # part -> {old: new}
for code in sorted(by_code):
    owners = by_code[code]
    if len(owners) < 2 or code in SHARED:
        continue
    for loser in owners[1:]:
        fam = int(code) // 100
        cand = int(code) + 1
        while True:
            if cand // 100 != fam:
                cand = fam * 100 + 1  # wrap inside the family
            c = f"{cand:03d}"
            if c not in used:
                break
            cand += 1
        used.add(c)
        remap[loser][code] = c

print("collisions resolved:", sum(len(v) for v in remap.values()))
for part in PRIORITY:
    if remap.get(part):
        print(f"  {part}: " + ", ".join(f"{o}->{n}" for o, n in sorted(remap[part].items())))

# ambiguity: a part references a colliding code without defining it
print("\nambiguous cross-references (part references a multiply-defined code it does not define):")
amb = 0
for code, owners in by_code.items():
    if len(owners) < 2 or code in SHARED:
        continue
    for part in refs[code]:
        if part not in owners:
            amb += 1
            print(f"  {part} references FROB-REF-{code}, defined in {owners}")
print("  count:", amb)

if apply:
    for part, m in remap.items():
        t = texts[part]
        # two-phase replace to avoid chains
        for old in m:
            t = re.sub(rf"FROB-REF-{old}(?!\d)", f"FROB-REF-@@{old}@@", t)
        for old, new in m.items():
            t = t.replace(f"FROB-REF-@@{old}@@", f"FROB-REF-{new}")
        (parts_dir / f"{part}.md").write_text(t, encoding="utf-8")
    print("\napplied.")
