#!/usr/bin/env python3
"""Add one tracker comment to each of a set of beads.

  push_comments.py <comments.json> --workspace <repo root> --state <name> --tag <tag> --author <name> [--dry-run] [--limit N]

<comments.json> is {bead key: comment text}. Uses only `br comments add` and `br sync --flush-only`. Resumable: the keys
already commented are kept in integration/load_state/<state>/comments_<tag>.json.
"""
import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS  # noqa: E402

ap = argparse.ArgumentParser()
ap.add_argument("comments")
ap.add_argument("--workspace", required=True)
ap.add_argument("--state", required=True)
ap.add_argument("--tag", required=True)
ap.add_argument("--author", required=True)
ap.add_argument("--dry-run", action="store_true")
ap.add_argument("--limit", type=int, default=0)
args = ap.parse_args()

WS = Path(args.workspace)
STATE = BEADS / "integration" / "load_state" / args.state
F_DONE = STATE / ("comments_%s.json" % args.tag)
TEXT = STATE / "note.md"
idmap = json.loads((STATE / "idmap.json").read_text(encoding="utf-8"))
comments = json.loads(Path(args.comments).read_text(encoding="utf-8"))
done = set(json.loads(F_DONE.read_text(encoding="utf-8"))) if F_DONE.exists() else set()


def br(*a):
    r = subprocess.run(["br", *a], cwd=WS, capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError((r.stderr or r.stdout).strip()[:400])
    return r.stdout


todo = [k for k in comments if k not in done]
missing = [k for k in todo if k not in idmap]
print("beads to comment: %d (already done %d; without a tracker id: %d %s)" % (len(todo), len(done), len(missing), missing[:8]), flush=True)
t0 = time.time()
n = 0
try:
    for k in todo:
        if k not in idmap:
            continue
        if args.dry_run:
            n += 1
            continue
        TEXT.write_text(comments[k], encoding="utf-8")
        try:
            br("comments", "add", idmap[k], "-f", str(TEXT), "--author", args.author, "--no-auto-flush")
        except RuntimeError as e:
            print("  failed %s: %s" % (k, e), flush=True)
            continue
        done.add(k)
        n += 1
        if n % 150 == 0:
            F_DONE.write_text(json.dumps(sorted(done)), encoding="utf-8")
            br("sync", "--flush-only")
            print("  %d of %d in %.0fs" % (n, len(todo), time.time() - t0), flush=True)
        if args.limit and n >= args.limit:
            break
finally:
    if not args.dry_run:
        F_DONE.write_text(json.dumps(sorted(done)), encoding="utf-8")
        print(br("sync", "--flush-only").strip().replace("\n", " ")[-200:], flush=True)
print("%scommented %d beads in %.0fs" % ("DRY RUN: " if args.dry_run else "", n, time.time() - t0), flush=True)
