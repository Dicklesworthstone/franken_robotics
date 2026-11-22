#!/usr/bin/env python3
"""Final mechanical checks on the assembled plan."""
import re
import sys
from collections import defaultdict

path = sys.argv[1]
with open(path, encoding="utf-8") as handle:
    text = handle.read()
lines = text.split("\n")

# chapter map
chap_of = []
cur = "0"
fence = False
for l in lines:
    if l.lstrip().startswith("```"):
        fence = not fence
    if not fence:
        m = re.match(r"^## (\d+)\. ", l)
        a = re.match(r"^## Appendix ([A-Z])\. ", l)
        if m:
            cur = m.group(1)
        elif a:
            cur = "App" + a.group(1)
    chap_of.append(cur)

print("== size ==")
print(" lines:", len(lines), " bytes:", len(text.encode("utf-8")), " words:", len(text.split()))

print("== leftover markers ==")
for pat in [r"<!-- LEDGER", r"<<LEAD", r"<!-- lead", r"<<<INSERT", r"<<<END", r"\bTODO\b", r"\bTBD\b", r"\bFIXME\b"]:
    hits = [i + 1 for i, l in enumerate(lines) if re.search(pat, l)]
    print(f" {pat}: {len(hits)} {hits[:5]}")

print("== attribution words outside §0.6 / Appendix K ==")
pat = re.compile(r"\b(ASTRA|NEON|Astra|Neon|Codex|Gemini|GPT|Claude|Fable|BASE plan|wizard|Wizard)\b")
bad = []
for i, l in enumerate(lines):
    if pat.search(l) and chap_of[i] not in ("AppK",):
        bad.append((i + 1, chap_of[i], l.strip()[:120]))
# §0.6 is inside chapter "0": allow lines that mention the plan file names
bad = [b for b in bad if "COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__" not in b[2]]
print(" count:", len(bad))
for b in bad[:25]:
    print("  ", b)

print("== LaTeX ==")
latex = [i + 1 for i, l in enumerate(lines) if re.search(r"\\(frac|mathbb|mathfrak|mathrm|cdot|times|leq|geq|sum|int)\b|\$[^$ `][^$`]*\$", l) and "${" not in l]
print(" count:", len(latex), latex[:10])

print("== exclamation marks outside code ==")
ex = []
fence = False
for i, l in enumerate(lines):
    if l.lstrip().startswith("```"):
        fence = not fence
        continue
    if not fence and "!" in re.sub(r"`[^`]*`", "", l):
        ex.append((i + 1, l.strip()[:110]))
print(" count:", len(ex))
for e in ex[:12]:
    print("  ", e)

print("== headings ==")
chs = [int(m.group(1)) for l in lines for m in [re.match(r"^## (\d+)\. ", l)] if m]
print(" chapters in order:", chs == sorted(chs), " count:", len(chs), " missing:", sorted(set(range(1, 48)) - set(chs)))
# duplicate subsection numbers
subs = defaultdict(int)
fence = False
for l in lines:
    if l.lstrip().startswith("```"):
        fence = not fence
    if not fence:
        m = re.match(r"^### (\d+\.\d+) ", l)
        if m:
            subs[m.group(1)] += 1
dups = {k: v for k, v in subs.items() if v > 1}
print(" subsections:", len(subs), " duplicated numbers:", dups)

print("== identifiers: defined in more than one chapter (table-row or heading definitions) ==")
pats = [r"GATE-[A-Z0-9]+-\d+", r"KILL-[A-Z0-9-]+", r"INT-[A-Z0-9]+-\d+", r"BET-\d{2}", r"(?<!NON)GOAL-\d{2}", r"(?<![A-Z-])NS-\d{2}",
        r"(?<![A-Z-])C-\d{2}", r"(?<![A-Z-])T-\d{2}", r"BD-\d{3}", r"WP-\d{2}", r"RISK-\d{2}", r"KA-\d{3}", r"CX-\d{3}", r"GB-\d{2}"]
defs = defaultdict(set)
refs = defaultdict(set)
for i, l in enumerate(lines):
    s = l.strip()
    for p in pats:
        for m in re.finditer(p, l):
            ident = m.group(0)
            refs[ident].add(chap_of[i])
            if re.match(r"^\|\s*(✖\s*)?(\*\*)?`?" + re.escape(ident) + r"`?(\*\*)?\s*[|,]", s) or (s.startswith("#") and ident in s) \
               or re.match(r"^(-\s+)?\*\*`?" + re.escape(ident) + r"\b", s):
                defs[ident].add(chap_of[i])
kinds = defaultdict(lambda: [0, 0])
for ident in refs:
    k = re.match(r"[A-Z]+", ident).group(0)
    kinds[k][1] += 1
    if ident in defs:
        kinds[k][0] += 1
for k in sorted(kinds):
    print(f" {k}: defined {kinds[k][0]} / referenced {kinds[k][1]}")
undef = sorted(i for i in refs if i not in defs)
print(" referenced but with no definition row/heading:", len(undef))
print("  ", undef[:60])
# kill gates defined in more than two chapters (owner + register is expected)
multi = {i: sorted(c) for i, c in defs.items() if i.startswith("KILL-") and len(c - {"40"}) > 1}
print(" kill gates defined in more than one chapter besides §40:", len(multi))
for i, c in list(multi.items())[:20]:
    print("  ", i, c)

print("== long prose lines (>125 chars, not table/code/url) ==")
fence = False
longl = []
for i, l in enumerate(lines):
    if l.lstrip().startswith("```"):
        fence = not fence
        continue
    if not fence and len(l) > 125 and not l.lstrip().startswith("|") and "http" not in l:
        longl.append(i + 1)
print(" count:", len(longl), longl[:12])
