#!/usr/bin/env python3
"""Compute the final bead graph from the definition files and the integration results.

Writes integration/final_graph.json and integration/graph_log.tsv. Changes no definition file.

Rules applied, in order, to every candidate dependency (bead -> prerequisite):
  superseded keys are remapped to the bead that replaced them; self-edges are dropped;
  an edge from or to an epic becomes `related` (a blocked epic would block all its children);
  an edge to a later milestone becomes `related`;
  a seed bead never depends on a higher-numbered seed bead: such an edge becomes `related`;
  an integrator's explicit `related` overrides a drafted dependency between the same two beads;
  an edge that would close a cycle becomes `related` (drafted edges are admitted before integration edges).
"""
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

INT = BEADS / "integration"
beads, problems = load_all()
MS = {"M%d" % i: i for i in range(9)}
log = []


def ms(k):
    return MS.get(str(beads[k].get("milestone")), 0)


def seed_no(k):
    m = re.match(r"^BD-(\d{3})$", k)
    return int(m.group(1)) if m else None


DEFS_ONLY = "--defs-only" in sys.argv   # after materialize_graph.py the definition files state every edge
results = {}
for f in ([] if DEFS_ONLY else sorted((INT / "out").glob("*.json")) + sorted((INT / "xout").glob("*.json"))):
    try:
        results[f.stem] = json.loads(f.read_text(encoding="utf-8"))
    except Exception as e:  # noqa: BLE001
        print("UNREADABLE result file", f.name, e)

# ------------------------------------------------------------------ superseded beads
sup = {}
for k, b in beads.items():
    s = b.get("superseded_by")
    if s:
        if s in beads and s != k:
            sup[k] = s
        else:
            log.append(("bad-superseded-target", k, str(s), "front matter"))
for g, d in results.items():
    for s in d.get("superseded", []) or []:
        drop, keep = s.get("drop"), s.get("keep")
        if drop in beads and keep in beads and drop != keep:
            if drop not in sup:
                sup[drop] = keep
                log.append(("superseded-only-in-result-file", drop, keep, g))
        else:
            log.append(("bad-superseded-entry", str(drop), str(keep), g))


def res(k):
    seen = set()
    while k in sup and k not in seen:
        seen.add(k)
        k = sup[k]
    return k


live = sorted(k for k in beads if k not in sup)
liveset = set(live)
is_epic = {k: (beads[k].get("type") == "epic") for k in beads}

# ------------------------------------------------------------------ candidates
cand = []          # (rank, order, src, dst, origin)
rel = set()        # unordered pairs
override_related = set()
order = [0]


def add_dep(src, dst, origin, rank):
    order[0] += 1
    cand.append((rank, order[0], src, dst, origin))


def add_rel(a, b):
    if a != b and a in liveset and b in liveset:
        rel.add((a, b) if a < b else (b, a))


for k in live:
    b = beads[k]
    for d in b["deps"]:
        d2 = res(d)
        if d2 not in beads:
            log.append(("unknown-dep-target", k, d, "drafted")); continue
        s1, s2 = seed_no(k), seed_no(d2)
        rank = 0 if (s1 is not None and s2 is not None and s2 < s1) else 1
        add_dep(k, d2, "drafted", rank)
    for r in b["related"]:
        r2 = res(r)
        if r2 in beads:
            add_rel(k, r2)

unresolved_needs = Counter()
for g, d in sorted(results.items()):
    for n in d.get("needs", []) or []:
        src = res(n.get("bead", ""))
        kind = n.get("as")
        if src not in beads:
            log.append(("unknown-bead-in-needs", str(n.get("bead")), "", g)); continue
        if kind in ("gap", "none"):
            unresolved_needs[kind] += 1
            continue
        for key in n.get("keys", []) or []:
            dst = res(key)
            if dst not in beads:
                log.append(("unknown-need-target", src, str(key), g)); continue
            if kind == "dep":
                add_dep(src, dst, "need:" + g, 2)
            else:
                add_rel(src, dst)
    for e in d.get("edges", []) or []:
        src, dst = res(e.get("bead", "")), res(e.get("dep", ""))
        if src not in beads or dst not in beads:
            log.append(("unknown-key-in-edges", str(e.get("bead")), str(e.get("dep")), g)); continue
        if e.get("as") == "related":
            override_related.add((src, dst))
            add_rel(src, dst)
        else:
            add_dep(src, dst, "edge:" + g, 2)

# ------------------------------------------------------------------ admission
graph = defaultdict(set)
deps = defaultdict(list)
demoted = Counter()


def reaches(a, b):
    """True if a depends on b through the admitted graph."""
    stack, seen = [a], {a}
    while stack:
        x = stack.pop()
        if x == b:
            return True
        for y in graph.get(x, ()):
            if y not in seen:
                seen.add(y); stack.append(y)
    return False


def demote(src, dst, why, origin):
    demoted[why] += 1
    log.append(("demoted:" + why, src, dst, origin))
    add_rel(src, dst)


seen_pairs = set()
for rank, _, src, dst, origin in sorted(cand):
    if src == dst or src not in liveset or dst not in liveset:
        continue
    if (src, dst) in seen_pairs:
        continue
    seen_pairs.add((src, dst))
    if (src, dst) in override_related:
        demote(src, dst, "integrator-override", origin); continue
    if is_epic[src] or is_epic[dst]:
        demote(src, dst, "epic", origin); continue
    if ms(dst) > ms(src):
        demote(src, dst, "later-milestone", origin); continue
    s1, s2 = seed_no(src), seed_no(dst)
    if s1 is not None and s2 is not None and s2 > s1 and not DEFS_ONLY:
        # during integration the plan's own numbering settled disputes about direction; once reviewers state an
        # edge in a bead's own file it stands, and only the cycle and milestone rules below can demote it
        demote(src, dst, "seed-order", origin); continue
    if reaches(dst, src):
        demote(src, dst, "cycle", origin); continue
    graph[src].add(dst)
    deps[src].append(dst)

# related: never duplicate a dependency in either direction
rel_final = defaultdict(list)
for a, b in sorted(rel):
    if b in graph.get(a, ()) or a in graph.get(b, ()):
        continue
    rel_final[a].append(b)

# ------------------------------------------------------------------ parents
parent = {}
for k in live:
    p = beads[k].get("parent")
    if k == "EPIC-PROGRAM":
        parent[k] = None; continue
    p2 = res(p) if p else None
    if not p2 or p2 not in liveset or p2 == k:
        fallback = "EPIC-" + str(beads[k].get("milestone"))
        log.append(("parent-replaced", k, str(p), fallback))
        p2 = fallback if fallback in liveset and fallback != k else "EPIC-PROGRAM"
    parent[k] = p2
for k in live:  # parent chains must end at the root
    seen, x = set(), k
    while x is not None and x not in seen:
        seen.add(x); x = parent.get(x)
    if x is not None:
        log.append(("parent-cycle", k, x, ""))
        parent[k] = "EPIC-PROGRAM"
non_epic_parent = sum(1 for k in live if parent[k] and not is_epic[parent[k]])

out = {"beads": {k: {"deps": sorted(deps.get(k, [])), "related": sorted(rel_final.get(k, [])), "parent": parent[k]} for k in live},
       "superseded": {k: res(k) for k in sup}}
(INT / "final_graph.json").write_text(json.dumps(out, indent=0, ensure_ascii=False), encoding="utf-8")
(INT / "graph_log.tsv").write_text("\n".join("\t".join(x) for x in log) + "\n", encoding="utf-8")

n_dep = sum(len(v) for v in deps.values())
n_rel = sum(len(v) for v in rel_final.values())
print("result files:", len(results), " definition problems:", len(problems))
print("beads: %d live, %d superseded" % (len(live), len(sup)))
print("dependencies admitted: %d   related pairs: %d   parents not of type epic: %d" % (n_dep, n_rel, non_epic_parent))
print("demoted to related:", dict(demoted))
print("needs left unresolved in result files:", dict(unresolved_needs))
print("log entries by kind:", dict(Counter(x[0] for x in log)))
roots = [k for k in live if not deps.get(k) and not is_epic[k]]
print("non-epic beads with no dependency (ready at start): %d; by milestone %s" % (len(roots), dict(sorted(Counter(str(beads[k].get("milestone")) for k in roots).items()))))
depth = {}


def d_of(k):
    if k in depth:
        return depth[k]
    depth[k] = 0
    depth[k] = 1 + max((d_of(x) for x in graph.get(k, ())), default=0)
    return depth[k]


sys.setrecursionlimit(100000)
print("longest dependency chain:", max(d_of(k) for k in live))
