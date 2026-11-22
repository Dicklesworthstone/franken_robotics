#!/usr/bin/env python3
"""Small graph helper over the drafted bead definitions (deps plus the mechanical edges).

  graph_check.py path A B     does A depend on B, directly or transitively? prints one path
  graph_check.py deps A       A's direct dependencies
  graph_check.py rdeps A      beads that depend directly on A
  graph_check.py cycles       strongly connected components with more than one bead
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402


def graph():
    beads, _ = load_all()
    g = defaultdict(set)
    for k, b in beads.items():
        if b.get("superseded_by"):
            continue
        for d in b["deps"]:
            if d in beads and d != k:
                g[k].add(d)
    return beads, g


def find_path(g, a, b):
    stack, seen, prev = [a], {a}, {}
    while stack:
        x = stack.pop()
        if x == b:
            path = [b]
            while path[-1] != a:
                path.append(prev[path[-1]])
            return path[::-1]
        for y in g.get(x, ()):
            if y not in seen:
                seen.add(y); prev[y] = x; stack.append(y)
    return None


def sccs(g, nodes):
    index, low, on, stack, out, counter = {}, {}, set(), [], [], [0]
    sys.setrecursionlimit(100000)

    def visit(v):
        index[v] = low[v] = counter[0]; counter[0] += 1
        stack.append(v); on.add(v)
        for w in g.get(v, ()):
            if w not in index:
                visit(w); low[v] = min(low[v], low[w])
            elif w in on:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop(); on.discard(w); comp.append(w)
                if w == v:
                    break
            if len(comp) > 1:
                out.append(comp)

    for v in nodes:
        if v not in index:
            visit(v)
    return out


def main():
    if len(sys.argv) < 2:
        print(__doc__); return
    beads, g = graph()
    cmd = sys.argv[1]
    if cmd == "path":
        a, b = sys.argv[2], sys.argv[3]
        for k in (a, b):
            if k not in beads:
                print("unknown key:", k); return
        p = find_path(g, a, b)
        print("%s depends on %s: %s" % (a, b, " -> ".join(p)) if p else "%s does not depend on %s" % (a, b))
    elif cmd == "deps":
        a = sys.argv[2]
        for d in sorted(g.get(a, ())):
            print(d, "|", beads[d].get("milestone"), "|", beads[d]["title"])
    elif cmd == "rdeps":
        a = sys.argv[2]
        for k in sorted(k for k, ds in g.items() if a in ds):
            print(k, "|", beads[k].get("milestone"), "|", beads[k]["title"])
    elif cmd == "cycles":
        comps = sccs(g, list(beads))
        print("components with a cycle:", len(comps))
        for c in sorted(comps, key=len, reverse=True)[:40]:
            print(len(c), " ".join(sorted(c)[:14]))


if __name__ == "__main__":
    main()
