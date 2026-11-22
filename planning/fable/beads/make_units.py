#!/usr/bin/env python3
"""Cut the plan into drafting units and write one brief per unit.

Run from planning/fable:  python3 beads/make_units.py
Writes beads/units.json, beads/seed_index.md, beads/wp_index.md, beads/briefs/<unit>.md
"""
import json
import re
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path("/Users/jemanuel/projects/franken_robotics")
PLAN = ROOT / "COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md"
OUT = ROOT / "planning/fable/beads"
L = PLAN.read_text(encoding="utf-8").split("\n")
N = len(L)

# ---------------------------------------------------------------- structure
chap = {}          # number -> dict(title, start, end)   (1-indexed, end inclusive)
app = {}           # letter -> dict(title, start, end)
order = []
fence = False
for i, line in enumerate(L, 1):
    if line.startswith("```"):
        fence = not fence
        continue
    if fence:
        continue
    m = re.match(r"^## (\d+)\. (.*)", line)
    if m:
        order.append(("c", int(m.group(1)), m.group(2).strip(), i)); continue
    m = re.match(r"^## Appendix ([A-O])\. (.*)", line)
    if m:
        order.append(("a", m.group(1), m.group(2).strip(), i))
for j, (kind, key, title, start) in enumerate(order):
    end = order[j + 1][3] - 1 if j + 1 < len(order) else N
    (chap if kind == "c" else app)[key] = {"title": title, "start": start, "end": end}

def subsections(start, end, pat):
    out = []
    fence = False
    for i in range(start, end + 1):
        line = L[i - 1]
        if line.startswith("```"):
            fence = not fence; continue
        if fence:
            continue
        m = re.match(pat, line)
        if m:
            out.append({"id": m.group(1), "title": m.group(2).strip(), "start": i})
    for j, s in enumerate(out):
        s["end"] = out[j + 1]["start"] - 1 if j + 1 < len(out) else end
    return out

subs = {}
for c, d in chap.items():
    pat = r"^### (C-\d\d) (.*)" if c == 10 else r"^### (%d\.\d+) (.*)" % c
    subs[c] = subsections(d["start"], d["end"], pat)

def app_sections(letter):
    d = app[letter]
    return subsections(d["start"], d["end"], r"^### (%s\.\d+) (.*)" % letter)

M_secs, N_secs = app_sections("M"), app_sections("N")
def by_chapter(secs):
    out = {}
    for s in secs:
        m = re.match(r"^§(\d+) ", s["title"])
        if m:
            out[int(m.group(1))] = s
        elif s["title"].startswith("Part 0"):
            out[0] = s
    return out
M_by, N_by = by_chapter(M_secs), by_chapter(N_secs)

# refusal codes by defining chapter
ref_by = defaultdict(list)
d = app["O"]
for i in range(d["start"], d["end"] + 1):
    m = re.match(r"^\| (FROB-REF-\d+) \| (.*) \| ([^|]*) \|\s*$", L[i - 1])
    if m:
        for c in sorted({int(x) for x in re.findall(r"§(\d+)", m.group(3))}):
            ref_by[c].append((m.group(1), m.group(2).strip(), m.group(3).strip()))

# late subsections of §45.12
late = set()
for s in subs[45]:
    if s["id"] == "45.12":
        for i in range(s["start"], s["end"] + 1):
            m = re.match(r"^\| §(\d+\.\d+) \|", L[i - 1])
            if m:
                late.add(m.group(1))

# gates: defining chapter, and §40 row line
GATE = r"(?:GATE|KILL|INT)-[A-Z0-9]+(?:-[A-Z0-9]+)*"
gate_def = {}
for c, dch in chap.items():
    if c >= 38 and c != 39:
        continue
    for i in range(dch["start"], dch["end"] + 1):
        s = L[i - 1].strip()
        m = re.match(r"^\|\s*(?:✖\s*)?`?(" + GATE + r")`?\s*\|", s)
        if m:
            gate_def.setdefault(m.group(1), c)
        for m in re.finditer(r"\*\*(?:Gate|Kill gate)[^*]*\*\*\.?\s*(?:✖\s*)?(" + GATE + ")", s):
            gate_def.setdefault(m.group(1), c)
gate_row40 = {}
d40 = chap[40]
for i in range(d40["start"], d40["end"] + 1):
    m = re.match(r"^\|\s*(?:✖\s*)?`?(" + GATE + r")`?\s*\|", L[i - 1].strip())
    if m:
        gate_row40.setdefault(m.group(1), i)
gates_by_ch = defaultdict(list)
for g, c in gate_def.items():
    gates_by_ch[c].append(g)

# crate home chapter
crate_count = defaultdict(Counter)
for c, dch in chap.items():
    if not (12 <= c <= 32 or 35 <= c <= 39):
        continue
    for i in range(dch["start"], dch["end"] + 1):
        for cr in re.findall(r"`(frob-[a-z0-9-]+)`", L[i - 1]):
            crate_count[cr][c] += 1
crate_home = {cr: cnt.most_common(1)[0][0] for cr, cnt in crate_count.items()}
for cr, c in {"frob-fsim": 33, "frob-core": 34, "frob-conformance": 38, "frob-mcp": 32, "frob-verbs": 32, "frob-cli": 32}.items():
    crate_home[cr] = c

# ---------------------------------------------------------------- seeds -> chapter
seeds = json.loads((OUT / "seed.json").read_text(encoding="utf-8"))
KW = [
 (r"exit review|program gate|first session|release rule", 43), (r"ten-fold|tournament|sealed suite|incumbent harness|phase ledger", 39),
 (r"theorem card|\bT-\d\d\b|Lean|olean|offline auditor", 12), (r"URDF|MJCF|\.fbot|archetype|preflight", 14),
 (r"actuator|battery|thermal|transmission", 15), (r"model variant|bucket|mixed-batch", 17), (r"contact|friction|tyre|track|terrain", 18),
 (r"referee step|batched step|integrator|fidelity ledger|promotion", 16), (r"scene|interior|clearance|retrofit", 19),
 (r"sensor|clock|camera|lidar|IMU", 20), (r"CUDA|Metal|dialect|kernel|backend|cudarc", 21), (r"controller|filter|gait|skill|shield", 22),
 (r"PPO|policy|VecEnv|gym", 23), (r"requirement|fixture|qualification|scorecard", 24), (r"mission", 25), (r"adversar|rare fail|counterexample world", 26),
 (r"autopsy|intervention|fork", 27), (r"analysis|\bA-\d\d\b|lineage|compare|ladder|cascade", 28), (r"telemetry|replay|energy ledger|digest", 29),
 (r"calibrat|reality", 30), (r"build packet|topology optim|fatigue|manufactur", 31), (r"verb|envelope|daemon|MCP|viewer|dashboard", 32),
 (r"admission|upstream|sibling|stack lock|toolchain|asupersync|FrankenSim", 33), (r"xtask|workspace|registr|format|Companion documents", 34),
 (r"determinis|anchored|precision", 35), (r"throughput|parity|performance|profile", 36), (r"private|licen|capabilit|security", 37),
 (r"Gauntlet|negative-control|known-answer|pilot|metamorphic", 38),
]
for s in seeds:
    text = s["title"] + " " + s["done_when"]
    score = Counter()
    for g in re.findall(GATE, text):
        if g in gate_def:
            score[gate_def[g]] += 3
    for x in re.findall(r"§(\d+)(?:\.\d+)*", text):
        if 9 <= int(x) <= 39:
            score[int(x)] += 2
    for cr in re.findall(r"`(frob-[a-z0-9-]+)`", s["title"]):
        if cr in crate_home:
            score[crate_home[cr]] += 3
    for cr in re.findall(r"`(frob-[a-z0-9-]+)`", s["done_when"]):
        if cr in crate_home:
            score[crate_home[cr]] += 1
    if score:
        s["chapter"] = score.most_common(1)[0][0]
    else:
        s["chapter"] = None
        for pat, c in KW:
            if re.search(pat, text, flags=re.I):
                s["chapter"] = c; break

def tokens(text):
    toks = Counter()
    for g in re.findall(GATE, text):
        toks[g] += 5
    for b in re.findall(r"`([^`]{3,40})`", text):
        toks[b] += 3
    for w in re.findall(r"[A-Za-z][A-Za-z-]{6,}", text):
        toks[w.lower()] += 1
    return toks

def best_sub(seed, c):
    toks = tokens(seed["title"] + " " + seed["done_when"])
    best, best_score = None, 0
    REG = re.compile(r"^(gates?\b|gates and|kill gate|invariants|open questions|refusal|what it owns|decisions|tiers|milestones|known answers)", re.I)
    cands = [s for s in subs.get(c, []) if not REG.search(s["title"])] or subs.get(c, [])
    for s in cands:
        body = "\n".join(L[s["start"] - 1:s["end"]])
        low = body.lower()
        sc = 0
        for t, w in toks.items():
            if (t in body) or (t in low):
                sc += w
        for x in re.findall(r"§(\d+\.\d+)", seed["title"] + " " + seed["done_when"]):
            if x == s["id"]:
                sc += 20
        if sc > best_score:
            best, best_score = s["id"], sc
    return best

# ---------------------------------------------------------------- units
SUBSYS = {12: "lemma", 13: "marrow", 14: "skeleton", 15: "sinew", 16: "kinesis", 17: "phylum", 18: "traction", 19: "habitat",
          20: "sensorium", 21: "spark", 22: "reflex", 23: "cortex", 24: "arena", 25: "augur", 26: "nemesis", 27: "autopsy",
          28: "atelier", 29: "chronicle", 30: "ground", 31: "foundry", 32: "tether", 33: "suite", 34: "workspace", 35: "numerics",
          36: "performance", 37: "safety", 38: "gauntlet", 39: "tenfold", 9: "spine", 10: "spine", 11: "conventions"}
import math
for s in seeds:
    c = s["chapter"]
    s["best_sub"] = best_sub(s, c) if (c is not None and 12 <= c <= 39 and c != 11) else None
seeds_in_sub = Counter(s["best_sub"] for s in seeds if s["best_sub"])
seeds_in_ch = Counter(s["chapter"] for s in seeds)
def weight(s):
    return (s["end"] - s["start"] + 1) + 35 * seeds_in_sub.get(s["id"], 0) + 25
units = []

def split_subs(c, k):
    ss = subs[c]
    total = sum(weight(s) for s in ss)
    groups, cur, acc, target = [], [], 0, total / k
    for s in ss:
        cur.append(s); acc += weight(s)
        if acc >= target * (len(groups) + 1) and len(groups) < k - 1:
            groups.append(cur); cur = []
    if cur:
        groups.append(cur)
    return groups

PARTS = {}
for c in sorted(chap):
    if not (9 <= c <= 39) or c == 9 or c == 11:
        continue
    w = sum(weight(s) for s in subs[c])
    k = max(1, math.ceil(w / 1150))
    if c == 13:
        k = 1
    groups = split_subs(c, k) if k > 1 else [subs[c]]
    k = len(groups)
    PARTS[c] = k
    for gi, grp in enumerate(groups):
        letter = "abcdefg"[gi] if k > 1 else ""
        u = {"unit": "U%02d%s" % (c, letter), "prefix": "N%02d%s-" % (c, letter), "chapters": [c], "subsystem": SUBSYS.get(c, ""),
             "subs": [s["id"] for s in grp], "part": gi, "parts": k, "seeds": [], "extra": ""}
        units.append(u)
U = {u["unit"]: u for u in units}
# fold §9 into U10a, §11 into U13
U["U10a"]["chapters"] = [9, 10]; U["U13"]["chapters"] = [11, 13]
U["U13"]["subs"] = [s["id"] for s in subs[11]] + U["U13"]["subs"]

special = [
 {"unit": "U00", "prefix": "N00-", "what": "Part 0 and §1–§8 (the thesis, claims, bets, North-Star scenarios, principles) and Appendix K"},
 {"unit": "UAB", "prefix": "NAB-", "what": "Appendix A (glossary), B (idea passes), C (EV matrix and killed-idea register), H (source register)"},
 {"unit": "U40", "prefix": "N40-", "what": "§40 (the gate register) and §43 (milestones)"},
 {"unit": "U41", "prefix": "N41-", "what": "§41 (risks), §42 (open questions), §45.10–45.11, §46 (first session), §47 (definition of done)"},
 {"unit": "U44", "prefix": "N44-", "what": "§44 (work packages)"},
 {"unit": "UEG", "prefix": "NEG-", "what": "Appendix E (worked cards) and Appendix G (reference robots, scene assets, licences, hazards)"},
 {"unit": "UJL", "prefix": "NJL-", "what": "Appendix J (verb registry) and Appendix L (worked artifacts)"},
 {"unit": "UF", "prefix": "NF-", "what": "Appendix F (upstream work items)"},
]
for sp in special:
    sp.update({"chapters": [], "subs": [], "part": 0, "parts": 1, "seeds": [], "extra": "", "subsystem": ""})
    units.append(sp); U[sp["unit"]] = sp

sub_unit = {}
for u in units:
    for sid in u["subs"]:
        sub_unit[sid] = u["unit"]

for s in seeds:
    c = s["chapter"]
    if c in (9, 10):
        unit = sub_unit.get(best_sub(s, 10), "U10a")
    elif c in (11, 13):
        unit = "U13"
    elif c is not None and 12 <= c <= 39:
        first = [x for x in units if x["chapters"] and x["chapters"][-1] == c][0]["unit"]
        unit = sub_unit.get(s.get("best_sub"), first)
    elif c in (40, 43):
        unit = "U40"
    else:
        unit = "U41"
    s["unit"] = unit
    U[unit]["seeds"].append(s["bd"])
(OUT / "seed.json").write_text(json.dumps(seeds, indent=1, ensure_ascii=False), encoding="utf-8")

# ---------------------------------------------------------------- indexes
rows = ["# Seed beads of §45 (all 442), with the drafting unit that writes each", "",
        "You may name any of these keys in `deps`. Only the owning unit writes the bead file.", "",
        "| Key | Milestone | Unit | Title |", "|---|---|---|---|"]
for s in seeds:
    t = s["title"] if len(s["title"]) <= 170 else s["title"][:167] + "…"
    rows.append("| %s | %s | %s | %s |" % (s["bd"], s["milestone"], s["unit"], t.replace("|", "/")))
(OUT / "seed_index.md").write_text("\n".join(rows) + "\n", encoding="utf-8")

wp_rows = []
d44 = chap[44]
for i in range(d44["start"], d44["end"] + 1):
    if re.match(r"^\| *(WP-\d+|WP|---)", L[i - 1]):
        wp_rows.append(L[i - 1])
(OUT / "wp_index.md").write_text("# Work packages (§44), verbatim rows. Use `WP-nn` as `parent`.\n\n" + "\n".join(wp_rows) + "\n", encoding="utf-8")

# ---------------------------------------------------------------- briefs
EXTRA = json.loads((OUT / "unit_extras.json").read_text(encoding="utf-8")) if (OUT / "unit_extras.json").exists() else {}
seed_by = {s["bd"]: s for s in seeds}
(OUT / "briefs").mkdir(exist_ok=True)

def rng(d):
    return "lines %d–%d" % (d["start"], d["end"])

for u in units:
    b = []
    b.append("# Brief for drafting unit %s" % u["unit"])
    b.append("")
    b.append("Read `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md` first. This brief says what you own.")
    b.append("")
    b.append("- Plan file: `%s` (%d lines). Read by line range with the Read tool (`offset`, `limit`)." % (PLAN, N))
    b.append("- Your output directory: `%s/defs/%s/` (create it; one `<key>.md` per bead, plus `coverage.md`)." % (OUT, u["unit"]))
    b.append("- Key prefix for your new beads: `%s` followed by a two-digit number (`%s01`, `%s02`, …)." % (u["prefix"], u["prefix"], u["prefix"]))
    b.append("- All seed beads, with the unit that writes each: `%s/seed_index.md`. Work packages: `%s/wp_index.md`." % (OUT, OUT))
    b.append("- Other keys you may depend on without writing them: `EPIC-PROGRAM`, `EPIC-M0`…`EPIC-M8`, `WP-01`…`WP-80`, `STD-E2E`, `STD-TEST`,")
    b.append("  `SPEC-C-01`…`SPEC-C-40` (the specification bead of construction C-nn), `CARD-T-01`…`CARD-T-35` (theorem card T-nn), and any `BD-nnn`.")
    b.append("")
    if u["chapters"]:
        b.append("## What you own")
        b.append("")
        for c in u["chapters"]:
            b.append("Chapter §%d %s: %s." % (c, chap[c]["title"], rng(chap[c])))
        b.append("")
        b.append("| Subsection | Title | Lines | Added late (no seed bead exists yet) |")
        b.append("|---|---|---|---|")
        for c in u["chapters"]:
            for s in subs[c]:
                if s["id"] in u["subs"]:
                    b.append("| §%s | %s | %d–%d | %s |" % (s["id"], s["title"], s["start"], s["end"], "yes" if s["id"] in late else ""))
        b.append("")
        if u["parts"] > 1:
            others = [x for x in units if x["chapters"] and x["chapters"][-1] == u["chapters"][-1] and x["unit"] != u["unit"]]
            b.append("This chapter is drafted by %d units. The others: %s." % (u["parts"], "; ".join(
                "%s owns %s to %s" % (x["unit"], "§" + x["subs"][0] if not x["subs"][0].startswith("C-") else x["subs"][0],
                                      "§" + x["subs"][-1] if not x["subs"][-1].startswith("C-") else x["subs"][-1]) for x in others)))
            b.append("Read the whole chapter once for context (skim the other units' subsections), but write beads only for your")
            b.append("subsections and your seed beads. If a seed bead of yours draws on a subsection another unit owns, read that")
            b.append("subsection and write the bead completely. For a prerequisite the other unit will create, use `needs`.")
            if u["part"] == 0:
                b.append("The chapter's preamble (before its first subsection) is yours.")
            b.append("")
        c = u["chapters"][-1]
        first, last = (u["part"] == 0), (u["part"] == u["parts"] - 1)
        b.append("## Registries")
        b.append("")
        for cc in u["chapters"]:
            if cc in M_by:
                who = "You write" if first else "Unit %s writes" % (("U%02da" % c) if u["parts"] > 1 else u["unit"])
                b.append("- Known answers of §%d: Appendix M, %s. %s the registry bead `N%02d-KA` (split as `-KA1`, `-KA2` if long). "
                         "Every unit cites KA numbers and values in its test plans." % (cc, rng(M_by[cc]), who, cc))
            if cc in N_by:
                who = "You write" if last else "Unit %s writes" % ("U%02d%s" % (c, "abcdefg"[u["parts"] - 1]))
                b.append("- Counterexamples of §%d: Appendix N, %s. %s the registry bead `N%02d-CX`." % (cc, rng(N_by[cc]), who, cc))
            if ref_by.get(cc):
                who = "You write" if last else "Unit %s writes" % ("U%02d%s" % (c, "abcdefg"[u["parts"] - 1]))
                b.append("- Refusal codes defined in §%d (%d codes, Appendix O lines %d–%d; the chapter's own refusal table has triggers and "
                         "repairs). %s the registry bead `N%02d-REF`: %s." % (cc, len(ref_by[cc]), app["O"]["start"], app["O"]["end"], who, cc,
                                                                              ", ".join(r[0].replace("FROB-REF-", "") for r in ref_by[cc])))
        b.append("")
        gl = sorted(g for cc in u["chapters"] for g in gates_by_ch.get(cc, []))
        if gl:
            b.append("## Gates defined in your chapter(s)")
            b.append("")
            b.append("The §40 register (lines %d–%d) governs each gate's milestone and pass criterion; the number after each gate is its" % (d40["start"], d40["end"]))
            b.append("row's line in §40. Every gate below must be named in the `gates` field of at least one bead that measures or decides it.")
            if u["parts"] > 1:
                b.append("A gate belongs to the unit whose subsection holds the mechanism it measures; the unit that owns the chapter's gates")
                b.append("subsection also writes a gate-measurement bead for any gate that would otherwise be carried by nobody, and lists in")
                b.append("`coverage.md` the gates it expects another unit to carry.")
            b.append("")
            b.append(", ".join("%s (%s)" % (g, gate_row40.get(g, "not in §40")) for g in gl))
            b.append("")
        b.append("## Also search for your material elsewhere in the plan")
        b.append("")
        b.append("- Appendix I (type sketches, %s): grep for your crate names and carry the sketches into your beads." % rng(app["I"]))
        b.append("- Appendix J (verb registry, %s): grep for your verbs; each verb your chapter owns needs a bead that implements it." % rng(app["J"]))
        b.append("- Appendix D (%s) and §10 (the constructions, %s): the mathematics your mechanisms implement. State the formulas you" % (rng(app["D"]), rng(chap[10])))
        b.append("  need in the bead itself, and add `SPEC-C-nn` to `deps` where a mechanism implements a construction.")
        b.append("- §41 (risks, %s) and §42 (open questions, %s): rows that name your chapter go into the Considerations of the bead" % (rng(chap[41]), rng(chap[42])))
        b.append("  they concern. An open question that blocks work gets its own decision bead (label `owner-decision`).")
        b.append("- §43 (milestones, %s) for what your subsystem owes at each milestone." % rng(chap[43]))
        b.append("")
    else:
        b.append("## What you own")
        b.append("")
        b.append(u["what"] + ".")
        b.append("")
    if u["unit"] in EXTRA:
        b.append("## Specific instructions for this unit")
        b.append("")
        ex = EXTRA[u["unit"]].strip()
        ex = re.sub(r"\{app:([A-O])\}", lambda m: "lines %d–%d" % (app[m.group(1)]["start"], app[m.group(1)]["end"]), ex)
        ex = re.sub(r"\{ch:(\d+)\}", lambda m: "lines %d–%d" % (chap[int(m.group(1))]["start"], chap[int(m.group(1))]["end"]), ex)
        ex = re.sub(r"\{end:(\d+)\}", lambda m: str(chap[int(m.group(1))]["end"]), ex)
        b.append(ex)
        b.append("")
    b.append("## Seed beads you own (%d): write one bead file for each, keeping key, milestone and dependencies" % len(u["seeds"]))
    b.append("")
    for bd in u["seeds"]:
        s = seed_by[bd]
        b.append("- **%s** (%s; depends on: %s%s)  " % (bd, s["milestone"], ", ".join(s["deps"]) or "nothing",
                                                    "; best-matching subsection §%s" % s["best_sub"] if s.get("best_sub") else ""))
        b.append("  Title: %s  " % s["title"])
        b.append("  Done when: %s" % s["done_when"])
    b.append("")
    (OUT / "briefs" / (u["unit"] + ".md")).write_text("\n".join(b) + "\n", encoding="utf-8")

json.dump({"units": units, "chapters": {str(k): v for k, v in chap.items()}, "appendices": app,
           "late": sorted(late), "gate_def": gate_def, "gate_row40": gate_row40,
           "subs": {str(k): v for k, v in subs.items()}},
          open(OUT / "units.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)

print("units:", len(units), " late subsections:", len(late), " gates with defining chapter:", len(gate_def), " in §40:", len(gate_row40))
for u in units:
    lines = sum(s["end"] - s["start"] + 1 for c in u["chapters"] for s in subs[c] if s["id"] in u["subs"])
    print("  %-5s subs=%2d lines=%4d seeds=%2d late=%d" % (u["unit"], len(u["subs"]), lines, len(u["seeds"]),
                                                          sum(1 for x in u["subs"] if x in late)))
print("seeds by unit total:", sum(len(u["seeds"]) for u in units))
print("late subsections not owned by a unit:", sorted(x for x in late if x not in sub_unit))
