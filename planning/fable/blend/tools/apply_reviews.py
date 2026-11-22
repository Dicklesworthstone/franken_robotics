#!/usr/bin/env python3
"""Apply the exact-substitution lists from the two review files to the part files."""
import re
import sys
from pathlib import Path

blend = Path(sys.argv[1])
apply = "--apply" in sys.argv
parts_dir = blend / "parts"
texts = {p.stem: p.read_text(encoding="utf-8") for p in parts_dir.glob("P*.md")}
subs = []  # (source_id, file_stem or None, candidate_files, original, replacement, replace_all)

# ---- REVIEW_MATH
cur_file, cur_id, in_fence, orig = None, None, False, None
for line in (blend / "REVIEW_MATH.md").read_text(encoding="utf-8").split("\n"):
    m = re.match(r"^\*\*(R-[0-9a-z]+)\*\*.*?`parts/(P\d+)\.md`", line)
    if m:
        cur_id, cur_file = m.group(1), m.group(2)
        continue
    if line.startswith("```"):
        in_fence = not in_fence
        continue
    if in_fence and cur_file:
        m = re.match(r"^ORIGINAL:\s+(.*)$", line)
        if m:
            orig = m.group(1).rstrip()
            continue
        m = re.match(r"^REPLACEMENT:\s?(.*)$", line)
        if m and orig is not None:
            subs.append(("M:" + cur_id, cur_file, [cur_file], orig, m.group(1).strip(), False))
            orig = None

# ---- REVIEW_COHERENCE
lines = (blend / "REVIEW_COHERENCE.md").read_text(encoding="utf-8").split("\n")
start = next(i for i, l in enumerate(lines) if l.startswith("## 2. Substitutions"))
end = next(i for i, l in enumerate(lines) if l.startswith("## 3."))
cur_id, cands, in_fence = None, [], False
pending = None  # (file, original, replace_all)
for line in lines[start:end]:
    m = re.match(r"^### (F\d+(?:, F\d+)*) \((.*)\)", line)
    if m:
        cur_id = m.group(1)
        cands = re.findall(r"P\d+", m.group(2))
        continue
    if line.startswith("```"):
        in_fence = not in_fence
        pending = None
        continue
    if not in_fence:
        continue
    m = re.match(r"^\s*(P\d+)?\s*ORIGINAL(\s*\(replace all[^)]*\))?:\s+(.*)$", line)
    if m:
        f = m.group(1)
        rall = bool(m.group(2))
        rest = m.group(3)
        mm = re.match(r"^(.*?)\s{2,}REPLACEMENT:\s?(.*)$", rest)
        if mm:
            subs.append(("C:" + cur_id, f, [f] if f else cands, mm.group(1).rstrip(), mm.group(2).strip(), rall))
            pending = None
        else:
            pending = (f, rest.rstrip(), rall)
        continue
    m = re.match(r"^\s*REPLACEMENT:\s?(.*)$", line)
    if m and pending:
        f, o, rall = pending
        subs.append(("C:" + cur_id, f, [f] if f else cands, o, m.group(1).strip(), rall))
        pending = None

applied, missing, ambiguous = 0, [], []
for sid, f, cand, o, r, rall in subs:
    hits = [c for c in cand if c in texts and o in texts[c]]
    if not hits:
        missing.append((sid, cand, o[:90]))
        continue
    for c in hits:
        n = texts[c].count(o)
        if n > 1 and not rall:
            ambiguous.append((sid, c, n, o[:90]))
            continue
        texts[c] = texts[c].replace(o, r)
        applied += 1

print(f"substitutions parsed: {len(subs)}; applied: {applied}; not found: {len(missing)}; ambiguous: {len(ambiguous)}")
for sid, cand, o in missing:
    print(f"  NOT FOUND {sid} {cand}: {o}")
for sid, c, n, o in ambiguous:
    print(f"  AMBIGUOUS {sid} {c} x{n}: {o}")
if apply:
    for k, v in texts.items():
        (parts_dir / f"{k}.md").write_text(v, encoding="utf-8")
    print("written.")
