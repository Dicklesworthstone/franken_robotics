#!/usr/bin/env python3
"""Bring a br workspace into agreement with the bead definitions. Every change goes through the `br` command.

  sync_beads.py --workspace DIR --state NAME [--limit N] [--dry-run]

Sources: the definition files (defs/<unit>/<key>.md) and integration/final_graph.json (tools/apply_integration.py).
State (integration/load_state/<NAME>/): idmap.json (key -> tracker id), hashes.json (content hash at last sync),
edges.json (edges present in the tracker, as the tool added them), closed.json (beads closed as superseded).

What a run does, in this order:
  1. creates the beads the tracker lacks, prerequisites first, with their edges;
  2. updates beads whose title, type, priority, labels or body changed;
  3. removes edges that are no longer wanted, then adds the missing ones;
  4. closes beads that were superseded since the last run (nothing is ever deleted);
  5. flushes the tracker's JSONL export.
A second run with nothing changed does nothing. The run is resumable at any point.
"""
import argparse
import hashlib
import heapq
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
ap.add_argument("--dry-run", action="store_true")
args = ap.parse_args()

WS = Path(args.workspace)
STATE = BEADS / "integration" / "load_state" / args.state
STATE.mkdir(parents=True, exist_ok=True)
F_ID, F_HASH, F_EDGE, F_CLOSED, BODY = (STATE / "idmap.json", STATE / "hashes.json", STATE / "edges.json",
                                        STATE / "closed.json", STATE / "body.md")


def load(p, default):
    return json.loads(p.read_text(encoding="utf-8")) if p.exists() else default


idmap, hashes, edges, closed = load(F_ID, {}), load(F_HASH, {}), load(F_EDGE, {}), load(F_CLOSED, {})


def save():
    if args.dry_run:
        return
    F_ID.write_text(json.dumps(idmap, indent=0), encoding="utf-8")
    F_HASH.write_text(json.dumps(hashes, indent=0), encoding="utf-8")
    F_EDGE.write_text(json.dumps(edges, indent=0), encoding="utf-8")
    F_CLOSED.write_text(json.dumps(closed, indent=0), encoding="utf-8")


beads, problems = load_all()
if problems:
    sys.exit("definition files with problems: %s" % problems[:5])
final = json.loads((BEADS / "integration" / "final_graph.json").read_text(encoding="utf-8"))
graph, superseded = final["beads"], final.get("superseded", {})
MS = {"M%d" % i: i for i in range(9)}


WRITES = [0]


def br(*cmd, retries=8):
    last = None
    if cmd[0] in ("create", "update", "close") or cmd[:2] in (("dep", "remove"), ("dep", "add"), ("dep", "import")):
        WRITES[0] += 1
        if WRITES[0] % 150 == 0:   # the write-ahead log grows without bound until a flush checkpoints it, and every command slows down
            subprocess.run(["br", "sync", "--flush-only"], cwd=WS, capture_output=True, text=True)
    for attempt in range(retries):
        p = subprocess.run(["br", *cmd], cwd=WS, capture_output=True, text=True)
        if p.returncode == 0:
            return p.stdout
        last = (p.stdout + p.stderr).strip()
        if re.search(r"lock|busy|timed out", last, flags=re.I):
            time.sleep(0.5 * (attempt + 1)); continue
        break
    raise RuntimeError("br %s failed: %s" % (" ".join(cmd[:3]), (last or "")[:600]))


def labels_of(b):
    out = []
    for x in b.get("labels", []):
        x = re.sub(r"[^a-z0-9._:-]+", "-", str(x).lower()).strip("-")
        if x and x not in out:
            out.append(x)
    return out


def title_of(k, b):
    t = str(b["title"]).strip()
    if t.startswith(k):
        t = t[len(k):].lstrip(" :.-·")
    return "[%s] %s" % (k, t)


def body_of(k, b):
    body = b["body"].rstrip("\n") + "\n"
    if b.get("type") == "epic":   # `br lint` expects this heading on epics
        body = body.replace("\n## Acceptance criteria\n", "\n## Success Criteria\n", 1)
    foot = ["", "---", "",
            "**Bead key:** `%s`. Other beads refer to this one by that key; find any bead by its key with `br search \"[<key>]\"`." % k,
            "**Milestone:** %s. **Drafting unit:** %s." % (b.get("milestone"), b.get("unit"))]
    if b.get("gates"):
        foot.append("**Gates measured or decided here:** " + ", ".join(str(x) for x in b["gates"]) + ".")
    if b.get("plan_refs"):
        foot.append("**Plan sections this came from (provenance only):** " + ", ".join(str(x) for x in b["plan_refs"]) + ".")
    return body + "\n".join(foot) + "\n"


def prio(b):
    p = b.get("priority")
    return str(p if p is not None else 2)


def hash_of(k, b):
    h = hashlib.sha256()
    for part in (title_of(k, b), str(b.get("type") or "task"), prio(b), ",".join(labels_of(b)), body_of(k, b)):
        h.update(part.encode("utf-8")); h.update(b"\x00")
    return h.hexdigest()


# ---------------------------------------------------------------- order: prerequisites first
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


heap = [(sort_key(k), k) for k in live if indeg[k] == 0]
heapq.heapify(heap)
order = []
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

# ---------------------------------------------------------------- desired edges
def ekey(src, dst, t):
    if t == "related":
        a, b = sorted((src, dst))
        return "related|%s|%s" % (a, b)
    return "%s|%s|%s" % (t, src, dst)


want = {}   # normalized key -> (src, dst, type)
for k in order:
    g = graph[k]
    if g["parent"] and g["parent"] in chosen:
        want[ekey(k, g["parent"], "parent-child")] = (k, g["parent"], "parent-child")
    for d in g["deps"]:
        if d in chosen:
            want[ekey(k, d, "blocks")] = (k, d, "blocks")
    for r in g["related"]:
        if r in chosen:
            want.setdefault(ekey(k, r, "related"), (k, r, "related"))
want_by_bead = defaultdict(list)
for key, (s, d, t) in want.items():
    want_by_bead[s].append((key, d, t))
    if t == "related":
        want_by_bead[d].append((key, s, t))

t0 = time.time()
n_create = n_update = n_add = n_remove = n_close = 0

# ---------------------------------------------------------------- 0. adopt beads an interrupted run created
import atexit  # noqa: E402
import signal  # noqa: E402
atexit.register(save)
signal.signal(signal.SIGTERM, lambda *_: sys.exit("terminated: state saved"))
listing = json.loads(br("list", "--all", "--limit", "0", "--json"))
adopted = 0
for it in (listing.get("issues") if isinstance(listing, dict) else listing) or []:
    ref = it.get("external_ref")
    if ref and ref in graph and ref not in idmap:
        idmap[ref] = it["id"]; adopted += 1
if adopted:
    print("adopted %d beads that an interrupted run had created" % adopted, flush=True)
    save()

# ---------------------------------------------------------------- 1. create
todo = [k for k in order if k not in idmap]
print("to create: %d   known: %d" % (len(todo), len(idmap)), flush=True)
for i, k in enumerate(todo, 1):
    b = beads[k]
    if args.dry_run:
        n_create += 1; continue
    BODY.write_text(body_of(k, b), encoding="utf-8")
    cmd = ["create", "--title", title_of(k, b), "-t", str(b.get("type") or "task"), "-p", prio(b),
           "--slug", k.lower(), "--external-ref", k, "--description-file", str(BODY), "--json", "--no-auto-flush"]
    lab = labels_of(b)
    if lab:
        cmd += ["-l", ",".join(lab)]
    j = json.loads(br(*cmd))   # edges are added afterwards in bulk: a create that carries edges is ten times slower
    new_id = j.get("id") or (j.get("issue") or {}).get("id")
    if not new_id:
        sys.exit("no id in br output for %s" % k)
    idmap[k] = new_id
    hashes[k] = hash_of(k, b)
    n_create += 1
    if n_create % 25 == 0:
        save()
    if n_create % 250 == 0:
        print("  created %d of %d in %.0fs" % (n_create, len(todo), time.time() - t0), flush=True)
save()

# ---------------------------------------------------------------- 2. update changed beads
for k in order:
    if k not in idmap:
        continue
    b = beads[k]
    h = hash_of(k, b)
    if hashes.get(k) == h:
        continue
    n_update += 1
    if args.dry_run:
        continue
    BODY.write_text(body_of(k, b), encoding="utf-8")
    cmd = ["update", idmap[k], "--title", title_of(k, b), "-t", str(b.get("type") or "task"), "-p", prio(b),
           "--description-file", str(BODY), "--json", "--no-auto-flush"]
    lab = labels_of(b)
    if lab:
        cmd += ["--set-labels", ",".join(lab)]
    br(*cmd)
    hashes[k] = h
    if n_update % 25 == 0:
        save()
save()

# ---------------------------------------------------------------- 3. edges: remove, then add
for key in sorted(set(edges) - set(want)):
    s, d, t = edges[key]
    if args.limit and (s not in chosen or d not in chosen):
        continue
    n_remove += 1
    if args.dry_run:
        continue
    if s in idmap and d in idmap:
        try:
            br("dep", "remove", idmap[s], idmap[d], "-t", t, "--json", "--no-auto-flush")
        except RuntimeError as e:
            if not re.search(r"not found|no such|does not exist", str(e), flags=re.I):
                print("  remove failed %s: %s" % (key, str(e)[:160]), flush=True); continue
    del edges[key]
    if n_remove % 50 == 0:
        save()
save()
RANK = {"parent-child": 0, "blocks": 1, "related": 2}
missing = sorted((key for key in set(want) - set(edges) if want[key][0] in idmap and want[key][1] in idmap),
                 key=lambda key: (RANK[want[key][2]], key))
BATCH = STATE / "edges_batch.jsonl"


def import_batch(keys):
    """Add edges through `br dep import`. Returns the number recorded."""
    global n_add
    if not keys:
        return
    BATCH.write_text("".join(json.dumps({"issue_id": idmap[want[k][0]], "depends_on_id": idmap[want[k][1]], "type": want[k][2]}) + "\n"
                             for k in keys), encoding="utf-8")
    try:
        out = json.loads(br("dep", "import", str(BATCH), "--json", "--no-auto-flush"))
    except RuntimeError as e:
        if len(keys) == 1:
            print("  edge refused %s: %s" % (keys[0], str(e)[:200]), flush=True)
            return
        mid = len(keys) // 2
        import_batch(keys[:mid]); import_batch(keys[mid:])
        return
    for k in keys:
        edges[k] = list(want[k])
    n_add += len(keys)
    save()
    print("  edges: %d added (%s imported, %s skipped by br) in %.0fs" % (n_add, out.get("imported"), out.get("skipped"), time.time() - t0), flush=True)


if args.dry_run:
    n_add = len(missing)
else:
    for i in range(0, len(missing), 2500):
        import_batch(missing[i:i + 2500])
save()

# ---------------------------------------------------------------- 4. close superseded beads
if not args.limit:
    for k, keep in sorted(superseded.items()):
        if k in idmap and k not in closed and k not in graph:
            n_close += 1
            if args.dry_run:
                continue
            reason = "Superseded by [%s]: its content was folded into that bead." % keep
            br("close", idmap[k], "--reason", reason, "--force", "--json", "--no-auto-flush")
            closed[k] = keep
    save()

if not args.dry_run:
    print(br("sync", "--flush-only").strip().replace("\n", " ")[-260:])
print("%screated %d, updated %d, edges added %d, edges removed %d, closed %d; tracker holds %d beads and %d edges; %.0fs"
      % ("DRY RUN: would have " if args.dry_run else "", n_create, n_update, n_add, n_remove, n_close, len(idmap), len(edges), time.time() - t0))
