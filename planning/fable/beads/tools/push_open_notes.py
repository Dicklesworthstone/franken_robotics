#!/usr/bin/env python3
"""Attach the review notes that polishing left unapplied to their beads, as one tracker comment per bead.

  push_open_notes.py <round> --workspace <repo root> --state <name> [--dry-run] [--limit N]

Reads polish/round-<N>/open_requests.json (from open_requests.py). Uses only `br comments add`. Resumable: the keys
already commented are kept in integration/load_state/<state>/notes_r<N>.json.
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
ap.add_argument("round", type=int)
ap.add_argument("--workspace", required=True)
ap.add_argument("--state", required=True)
ap.add_argument("--dry-run", action="store_true")
ap.add_argument("--limit", type=int, default=0)
args = ap.parse_args()

WS = Path(args.workspace)
STATE = BEADS / "integration" / "load_state" / args.state
F_DONE = STATE / ("notes_r%d.json" % args.round)
TEXT = STATE / "note.md"
idmap = json.loads((STATE / "idmap.json").read_text(encoding="utf-8"))
notes = json.loads((BEADS / "polish" / ("round-%d" % args.round) / "open_requests.json").read_text(encoding="utf-8"))
done = set(json.loads(F_DONE.read_text(encoding="utf-8"))) if F_DONE.exists() else set()

HEAD = ("Review notes from polishing round %d that are NOT yet applied to this bead's description.\n\n"
        "Polishing was closed on 2026-11-23 before this bead was re-read against them. Apply each note, or confirm that it no "
        "longer holds, before or while implementing this bead. Where a note and a registry bead differ (the verb registries "
        "`VERBS-<cluster>`, the refusal, known-answer and counterexample registries `…-REF`, `…-KA`, `…-CX`, and the owner beads "
        "`NJL-01`, `STD-E2E`, `STD-TEST`, `BD-025`), the registry bead as it stands is the authority. A note that begins "
        "`ANSWER:` is a registry's ruling on something this bead asked for. When the notes are dealt with, remove the label "
        "`review-notes-open`.\n\n") % args.round


def br(*a):
    r = subprocess.run(["br", *a], cwd=WS, capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError((r.stderr or r.stdout).strip()[:400])
    return r.stdout


todo = [k for k in notes if k not in done]
missing = [k for k in todo if k not in idmap]
print("beads to comment: %d (already done %d; without a tracker id: %d %s)" % (len(todo), len(done), len(missing), missing[:8]), flush=True)
t0 = time.time(); n = 0
try:
    for k in todo:
        if k not in idmap:
            continue
        body = HEAD + "\n".join("%d. %s" % (i, t.strip()) for i, t in enumerate(notes[k], 1)) + "\n"
        if args.dry_run:
            n += 1; continue
        TEXT.write_text(body, encoding="utf-8")
        try:
            br("comments", "add", idmap[k], "-f", str(TEXT), "--author", "polishing-round-%d" % args.round, "--no-auto-flush")
        except RuntimeError as e:
            print("  failed %s: %s" % (k, e), flush=True); continue
        done.add(k); n += 1
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
