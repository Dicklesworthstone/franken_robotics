#!/usr/bin/env python3
"""Load the bead definitions into a br workspace. Every change to the tracker goes through the `br` command.

  load_beads.py --workspace DIR --state NAME [--limit N] [--no-related] [--phase create|edges|all]

Content comes from the definition files; edges from integration/final_graph.json (tools/apply_integration.py).
Resumable: the key->id map and the edges already added are kept in integration/load_state/<NAME>/.
"""
import argparse
import json
import re
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

ap = argparse.ArgumentParser()
ap.add_argument("--workspace", required=True)
ap.add_argument("--state", required=True)
ap.add_argument("--limit", type=int, default=0)
ap.add_argument("--no-related", action="store_true")
ap.add_argument("--phase", default="all")
args = ap.parse_args()

WS = Path(args.workspace)
STATE = BEADS / "integration" / "load_state" / args.state
STATE.mkdir(parents=True, exist_ok=True)
IDMAP = STATE / "idmap.json"
EDGES = STATE / "edges_done.txt"
BODY = STATE / "body.md"

beads, problems = load_all()
graph = json.loads((BEADS / "integration" / "final_graph.json").read_text(encoding="utf-8"))["beads"]
MS = {"M%d" % i: i for i in range(9)}
idmap = json.loads(IDMAP.read_text(encoding="utf-8")) if IDMAP.exists() else {}
edges_done = set(EDGES.read_text(encoding="utf-8").split("\n")) if EDGES.exists() else set()


def br(*cmd, retries=6):
    last = None
    for attempt in range(retries):
        p = subprocess.run(["br", *cmd], cwd=WS, capture_output=True, text=True)
        if p.returncode == 0:
            return p.stdout
        last = (p.stdout + p.stderr).strip()
        if re.search(r"lock|busy|timed out", last, flags=re.I):
            time.sleep(0.5 * (attempt + 1)); continue
        break
    raise RuntimeError("br %s failed: %s" % (" ".join(cmd[:3]), last[:600]))


def label_list(b):
    out = []
    for x in b.get("labels", []):
        x = re.sub(r"[^a-z0-9._:-]+", "-", str(x).lower()).strip("-")
        if x and x not in out:
            out.append(x)
    return out


def footer(k, b):
    lines = ["", "---", "", "**Bead key:** `%s`. Other beads refer to this one by that key; find any bead by its key with `br search \"[<key>]\"`." % k,
             "**Milestone:** %s. **Drafting unit:** %s." % (b.get("milestone"), b.get("unit"))]
    if b.get("gates"):
        lines.append("**Gates measured or decided here:** " + ", ".join(b["gates"]) + ".")
    if b.get("plan_refs"):
        lines.append("**Plan sections this came from (provenance only):** " + ", ".join(str(x) for x in b["plan_refs"]) + ".")
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------- creation order: prerequisites first
live = list(graph)
indeg = {k: 0 for k in live}
rdeps = defaultdict(list)
for k in live:
    for d in graph[k]["deps"]:
        indeg[k] += 1
        rdeps[d].append(k)


def sort_key(k):
    b = beads[k]
    return (0 if b.get("type") == "epic" else 1, MS.get(str(b.get("milestone")), 9), k)


rel_all = defaultdict(set)   # related is symmetric: attach each pair when its later bead is created
for k in live:
    for r in graph[k]["related"]:
        if r in graph:
            rel_all[k].add(r); rel_all[r].add(k)


def title_of(k, b):
    t = str(b["title"]).strip()
    if t.startswith(k):
        t = t[len(k):].lstrip(" :.-·")
    return "[%s] %s" % (k, t)


def body_of(k, b):
    body = b["body"].rstrip("\n") + "\n"
    if b.get("type") == "epic":   # `br lint` expects this heading on epics
        body = body.replace("\n## Acceptance criteria\n", "\n## Success Criteria\n", 1)
    return body + footer(k, b)


ready = sorted((k for k in live if indeg[k] == 0), key=sort_key)
order = []
import heapq  # noqa: E402
heap = [(sort_key(k), k) for k in ready]
heapq.heapify(heap)
while heap:
    _, k = heapq.heappop(heap)
    order.append(k)
    for r in rdeps[k]:
        indeg[r] -= 1
        if indeg[r] == 0:
            heapq.heappush(heap, (sort_key(r), r))
if len(order) != len(live):
    sys.exit("the dependency graph has a cycle: %d of %d beads ordered" % (len(order), len(live)))
if args.limit:
    order = order[:args.limit]
chosen = set(order)

t0 = time.time()
created = 0
pending = []   # (src, dst, type) edges whose target did not exist yet
if args.phase in ("all", "create"):
    for i, k in enumerate(order, 1):
        b, g = beads[k], graph[k]
        want = []
        if g["parent"]:
            want.append((g["parent"], "parent-child"))
        want += [(d, "blocks") for d in g["deps"]]
        if k in idmap:
            pending += [(k, d, t) for d, t in want]
            if not args.no_related:
                pending += [(k, r, "related") for r in g["related"]]
            continue
        now, later = [], []
        for d, t in want:
            (now if d in idmap else later).append((d, t))
        if not args.no_related:
            for r in sorted(rel_all[k]):
                if r in idmap and "%s|%s|related" % (r, k) not in edges_done:
                    now.append((r, "related"))
        BODY.write_text(body_of(k, b), encoding="utf-8")
        cmd = ["create", "--title", title_of(k, b), "-t", str(b.get("type") or "task"),
               "-p", str(b.get("priority") if b.get("priority") is not None else 2),
               "--slug", k.lower(), "--external-ref", k, "--description-file", str(BODY),
               "--json", "--no-auto-flush"]
        labels = label_list(b)
        if labels:
            cmd += ["-l", ",".join(labels)]
        if now:
            cmd += ["--deps", ",".join("%s:%s" % (t, idmap[d]) for d, t in now)]
        out = br(*cmd)
        j = json.loads(out)
        new_id = j.get("id") or (j.get("issue") or {}).get("id")
        if not new_id:
            sys.exit("no id in br output for %s: %s" % (k, out[:300]))
        idmap[k] = new_id
        with EDGES.open("a", encoding="utf-8") as fh:
            for d, t in now:
                fh.write("%s|%s|%s\n" % (k, d, t)); edges_done.add("%s|%s|%s" % (k, d, t))
        pending += [(k, d, t) for d, t in later]
        created += 1
        if created % 20 == 0:
            IDMAP.write_text(json.dumps(idmap, indent=0), encoding="utf-8")
        if created % 200 == 0:
            print("  created %d (%d of %d) in %.0fs" % (created, i, len(order), time.time() - t0), flush=True)
    IDMAP.write_text(json.dumps(idmap, indent=0), encoding="utf-8")
    print("created %d beads in %.0fs (%d already present)" % (created, time.time() - t0, len(order) - created))
else:
    for k in order:
        g = graph[k]
        pending += [(k, g["parent"], "parent-child")] if g["parent"] else []
        pending += [(k, d, "blocks") for d in g["deps"]]
        if not args.no_related:
            pending += [(k, r, "related") for r in g["related"]]

pending = [(s, d, t) for s, d, t in pending if not (t == "related" and "%s|%s|related" % (d, s) in edges_done)]
added = skipped = 0
if args.phase in ("all", "edges"):
    t1 = time.time()
    with EDGES.open("a", encoding="utf-8") as fh:
        for src, dst, t in pending:
            tag = "%s|%s|%s" % (src, dst, t)
            if tag in edges_done or src not in idmap or dst not in idmap:
                skipped += 1; continue
            try:
                br("dep", "add", idmap[src], idmap[dst], "-t", t, "--json", "--no-auto-flush")
            except RuntimeError as e:
                if re.search(r"already|exists|duplicate", str(e), flags=re.I):
                    pass
                else:
                    print("  edge failed %s: %s" % (tag, str(e)[:200]), flush=True); continue
            fh.write(tag + "\n"); edges_done.add(tag); added += 1
            if added % 500 == 0:
                print("  added %d edges in %.0fs" % (added, time.time() - t1), flush=True)
    print("edges added afterwards: %d (skipped %d) in %.0fs" % (added, skipped, time.time() - t1))

print(br("sync", "--flush-only").strip()[-300:])
print("total: %d beads mapped, %d edges recorded, %.0fs" % (len(idmap), len(edges_done) - (1 if "" in edges_done else 0), time.time() - t0))
