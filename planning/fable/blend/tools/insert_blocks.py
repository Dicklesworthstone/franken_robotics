#!/usr/bin/env python3
"""Insert the completeness-audit blocks as new subsections at the ends of their chapters."""
import re
import sys
from collections import defaultdict
from pathlib import Path

blend = Path(sys.argv[1])
apply = "--apply" in sys.argv
files = sorted((blend / "parts").glob("P*.md")) + sorted((blend / "parts2").glob("P*.md"))
texts = {p: p.read_text(encoding="utf-8") for p in files}

# existing gate numbers per subsystem
maxnum = defaultdict(int)
for t in texts.values():
    for m in re.finditer(r"GATE-([A-Z0-9]+)-(\d+)", t):
        n = int(m.group(2))
        if n < 700:
            maxnum[m.group(1)] = max(maxnum[m.group(1)], n)

blocks = []
for src in [a for a in sys.argv[2:] if a.endswith(".md")]:
    t = (blend / src).read_text(encoding="utf-8")
    for m in re.finditer(r'<<<INSERT chapter=(\d+) title="([^"]+)">>>\n(.*?)\n<<<END>>>', t, flags=re.S):
        blocks.append([src, int(m.group(1)), m.group(2).strip(), m.group(3).strip("\n")])

# renumber provisional gates (8NN / 9NN) into each subsystem's sequence
gate_map = {}
for b in blocks:
    for m in re.finditer(r"GATE-([A-Z0-9]+)-([789]\d\d)", b[3]):
        old = m.group(0)
        if old not in gate_map:
            maxnum[m.group(1)] += 1
            gate_map[old] = f"GATE-{m.group(1)}-{maxnum[m.group(1)]:02d}"
for b in blocks:
    for old, new in gate_map.items():
        b[3] = b[3].replace(old, new)


def chapter_span(text, chap):
    lines = text.split("\n")
    start = None
    for i, l in enumerate(lines):
        if re.match(rf"^## {chap}\. ", l):
            start = i
            break
    if start is None:
        return None
    end = len(lines)
    for j in range(start + 1, len(lines)):
        if re.match(r"^## ", lines[j]) or lines[j].strip().startswith("<!-- LEDGER"):
            end = j
            break
    return lines, start, end


report = []
for src, chap, title, body in blocks:
    target = None
    for p in files:
        sp = chapter_span(texts[p], chap)
        if sp:
            target = p
            break
    if target is None:
        report.append((src, chap, title, "NO CHAPTER"))
        continue
    lines, start, end = chapter_span(texts[target], chap)
    ks = [int(m.group(1)) for l in lines[start:end] for m in [re.match(rf"^### {chap}\.(\d+)[ .]", l)] if m]
    k = (max(ks) if ks else 0) + 1
    # trim trailing blank lines before the insertion point
    ins = end
    while ins > start and lines[ins - 1].strip() == "":
        ins -= 1
    new = ["", f"### {chap}.{k} {title}", "", body, ""]
    lines[ins:end] = new + ([""] if end < len(lines) else [])
    texts[target] = "\n".join(lines)
    gates = sorted(set(re.findall(r"(?:GATE|KILL)-[A-Z0-9-]+", body)))
    report.append((src, chap, f"{chap}.{k}", title, target.name, len(body.split("\n")), ", ".join(gates[:6])))

print(f"blocks: {len(blocks)}; gates renumbered: {len(gate_map)}")
for r in report:
    print("  ", " | ".join(str(x) for x in r)[:230])
if apply:
    for p, t in texts.items():
        p.write_text(t, encoding="utf-8")
    out = ["# Late insertions (completeness audits)\n", "| § | Title | File | Lines | Gates named |", "|---|---|---|---|---|"]
    for r in report:
        if len(r) == 7:
            out.append(f"| {r[2]} | {r[3]} | {r[4]} | {r[5]} | {r[6]} |")
    prev = (blend / "LATE_INSERTIONS.md").read_text(encoding="utf-8") if (blend / "LATE_INSERTIONS.md").exists() else "\n".join(out[:3]) + "\n"
    (blend / "LATE_INSERTIONS.md").write_text(prev + "\n".join(out[3:]) + "\n", encoding="utf-8")
    print("written.")
