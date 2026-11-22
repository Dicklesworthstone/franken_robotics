#!/usr/bin/env python3
"""Assemble the final plan from the part files."""
import re
import sys
from collections import OrderedDict
from pathlib import Path

blend = Path(sys.argv[1])
out_path = Path(sys.argv[2])


def strip_ledger(text):
    lines = text.split("\n")
    for i, l in enumerate(lines):
        if l.strip().startswith("<!-- LEDGER"):
            return "\n".join(lines[:i]).rstrip() + "\n"
    return text.rstrip() + "\n"


def split_chapters(text):
    """Split on level-2 headings outside code fences. Returns OrderedDict key -> text."""
    chapters = OrderedDict()
    key, buf, fence = "pre", [], False
    for line in text.split("\n"):
        if line.lstrip().startswith("```"):
            fence = not fence
        if not fence:
            m = re.match(r"^## (\d+)\. ", line)
            a = re.match(r"^## Appendix ([A-Z])\. ", line)
            if m or a:
                if buf:
                    chapters[key] = "\n".join(buf).rstrip() + "\n"
                key = int(m.group(1)) if m else a.group(1)
                buf = []
        buf.append(line)
    if buf:
        chapters[key] = "\n".join(buf).rstrip() + "\n"
    return chapters


ch = {}
for f in sorted((blend / "parts").glob("P*.md")) + sorted((blend / "parts2").glob("P*.md")):
    t = strip_ledger(f.read_text(encoding="utf-8"))
    if f.name == "P00.md":
        p00 = t
        continue
    for k, v in split_chapters(t).items():
        if k == "pre":
            if v.strip():
                print("WARNING: preamble text in", f.name, repr(v[:80]))
            continue
        if k in ch:
            print("WARNING: duplicate chapter", k, "in", f.name)
        ch[k] = v

# ---- P00: markers, provenance, contents
prov = (blend / "LEAD_PROVENANCE.md").read_text(encoding="utf-8").rstrip()
assert p00.count("<<LEAD-PROVENANCE>>") == 1
p00 = p00.replace("<<LEAD-PROVENANCE>>", prov)
p00 = re.sub(r"<!-- lead: PART 0 heading goes here -->", "# PART 0. DOCUMENT CONTROL", p00)
p00 = re.sub(r"<!-- lead: PART I heading goes here -->", "# PART I. THESIS", p00)
p00 = p00.replace("· M known-answer suite · N counterexample catalogue",
                  "· M known-answer suite · N counterexample catalogue · O refusal code registry")

# ---- appendices K and O
ch["K"] = (blend / "APPENDIX_K.md").read_text(encoding="utf-8").rstrip() + "\n"
ch["O"] = (blend / "APPENDIX_O_REFUSALS.md").read_text(encoding="utf-8").rstrip() + "\n"

# ---- §40 addendum: gates defined in the document but absent from the register
reg = ch[40]
reg_ids = set(re.findall(r"(?:GATE|KILL|INT)-[A-Z0-9]+(?:-[A-Z0-9]+)*", reg))
late = OrderedDict()
for k, v in ch.items():
    if k == 40 or not isinstance(k, int):
        continue
    sub = None
    for line in v.split("\n"):
        m = re.match(r"^#{3,4} (\d+\.\d+(?:\.\d+)?)\s", line)
        if m:
            sub = m.group(1)
        s = line.strip()
        m = re.match(r"^\|\s*(✖\s*)?`?((?:GATE|KILL|INT)-[A-Z0-9]+(?:-[A-Z0-9]+)*)`?\s*\|(.*)\|\s*$", s)
        if m and m.group(2) not in reg_ids and m.group(2) not in late:
            cells = [c.strip() for c in m.group(3).split("|")]
            late[m.group(2)] = (sub or str(k), " ¦ ".join(c for c in cells if c)[:330])
if late:
    nsub = max(int(x) for x in re.findall(r"^### 40\.(\d+)", reg, flags=re.M)) + 1
    add = [f"\n### 40.{nsub} Gates defined in subsections added by the final audit\n",
           "These gates are defined in subsections that were added after the tables above were compiled. Each is listed",
           "with its defining subsection, which governs its threshold until the next revision of this register folds",
           "it into the tables of its subsystem. A kill gate here names its fallback in its defining subsection.\n",
           "| Gate | Defined in | Statement (abridged) |", "|---|---|---|"]
    for gid, (sub, txt) in late.items():
        add.append(f"| {gid} | §{sub} | {txt} |")
    ch[40] = reg.rstrip() + "\n" + "\n".join(add) + "\n"
    print("late gates added to §40:", len(late))

# ---- §45 addendum: late subsections to place in the bead graph
li = (blend / "LATE_INSERTIONS.md").read_text(encoding="utf-8").split("\n")
rows = [l for l in li if l.startswith("| ") and not l.startswith("| §") and not l.startswith("|---")]
if rows:
    beads = ch[45]
    nsub = max(int(x) for x in re.findall(r"^### 45\.(\d+)", beads, flags=re.M)) + 1
    add = [f"\n### 45.{nsub} Subsections added by the audits, to be placed during conversion\n",
           "The subsections below were added by the completeness audits after the bead tables were compiled. During",
           "conversion each becomes one or more beads in the milestone of its gate, with the same done-condition form:",
           "its gate records a value and its negative control is caught. Until then they are not counted in the graph",
           "facts above.\n",
           "| Subsection | Title | Gates it names |", "|---|---|---|"]
    for r in rows:
        c = [x.strip() for x in r.strip().strip("|").split("|")]
        add.append(f"| §{c[0]} | {c[1]} | {c[4] if len(c) > 4 else ''} |")
    ch[45] = beads.rstrip() + "\n" + "\n".join(add) + "\n"
    print("late subsections listed in §45:", len(rows))

order = [
    ("# PART II. THE MATHEMATICAL SPINE", [9, 10, 11, 12]),
    ("# PART III. THE BODY", [13, 14, 15, 16, 17, 18, 19, 20, 21]),
    ("# PART IV. THE MIND", [22, 23, 24, 25, 26, 27, 28]),
    ("# PART V. THE RECORD AND THE WORLD", [29, 30, 31, 32]),
    ("# PART VI. SUBSTRATE AND ENGINEERING", [33, 34, 35, 36, 37]),
    ("# PART VII. VERIFICATION, THE TEN-FOLD TEST, GATES AND RISKS", [38, 39, 40, 41, 42]),
    ("# PART VIII. DELIVERY", [43, 44, 45, 46, 47]),
    ("# APPENDICES", list("ABCDEFGHIJKLMNO")),
]
parts = [p00.rstrip() + "\n"]
missing = []
for title, keys in order:
    parts.append("\n---\n\n" + title + "\n")
    for k in keys:
        if k not in ch:
            missing.append(k)
            continue
        parts.append("\n" + ch[k].rstrip() + "\n")
doc = "".join(parts)
doc = re.sub(r"\n{4,}", "\n\n\n", doc)
out_path.write_text(doc, encoding="utf-8")
print("missing chapters:", missing)
print("unused chapters:", [k for k in ch if not any(k in keys for _, keys in order)])
print("lines:", doc.count("\n"), "bytes:", len(doc.encode("utf-8")))
