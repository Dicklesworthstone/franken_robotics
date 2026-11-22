#!/usr/bin/env python3
"""Shared reader for bead definition files (planning/fable/beads/defs/<unit>/<key>.md)."""
import json
import re
from pathlib import Path

BEADS = Path("/Users/jemanuel/projects/franken_robotics/planning/fable/beads")
DEFS = BEADS / "defs"
LIST_FIELDS = ("labels", "deps", "needs", "gates", "plan_refs", "related")
REQUIRED = ("key", "title", "type", "priority", "milestone", "labels", "parent", "deps")
SECTIONS = ("## Background", "## Scope", "## Technical approach", "## Acceptance criteria", "## Test plan",
            "## Considerations", "## References")


def parse_value(name, raw):
    raw = raw.strip()
    if name in LIST_FIELDS:
        if raw == "":
            return []
        try:
            v = json.loads(raw)
            return v if isinstance(v, list) else [v]
        except Exception:
            inner = raw.strip("[]")
            return [x.strip().strip('"').strip("'") for x in inner.split(",") if x.strip()]
    if name == "priority":
        m = re.search(r"\d", raw)
        return int(m.group(0)) if m else None
    if len(raw) >= 2 and raw[0] == '"' and raw[-1] == '"':
        try:
            return json.loads(raw)
        except Exception:
            return raw[1:-1]
    return raw


def read_def(path):
    text = Path(path).read_text(encoding="utf-8")
    lines = text.split("\n")
    problems = []
    if not lines or lines[0].strip() != "---":
        return None, ["no front matter"]
    try:
        end = next(i for i in range(1, len(lines)) if lines[i].strip() == "---")
    except StopIteration:
        return None, ["front matter not closed"]
    fm = {}
    for ln in lines[1:end]:
        if not ln.strip():
            continue
        m = re.match(r"^([a-z_]+):\s?(.*)$", ln)
        if not m:
            problems.append("bad front-matter line: " + ln[:60])
            continue
        fm[m.group(1)] = parse_value(m.group(1), m.group(2))
    for f in LIST_FIELDS:
        fm.setdefault(f, [])
    body = "\n".join(lines[end + 1:]).strip("\n") + "\n"
    for r in REQUIRED:
        if r not in fm or fm[r] in (None, ""):
            if r == "parent" and fm.get("key") == "EPIC-PROGRAM":
                continue
            problems.append("missing field " + r)
    fm["body"] = body
    fm["path"] = str(path)
    fm["unit"] = Path(path).parent.name
    return fm, problems


def load_all(units=None):
    beads, problems = {}, []
    for d in sorted(DEFS.iterdir()):
        if not d.is_dir() or (units and d.name not in units):
            continue
        for f in sorted(d.glob("*.md")):
            if f.name.lower() == "coverage.md":
                continue
            fm, pr = read_def(f)
            if fm is None:
                problems.append((str(f), pr)); continue
            if pr:
                problems.append((str(f), pr))
            k = fm.get("key")
            if not k:
                continue
            if k != f.stem:
                problems.append((str(f), ["key %s differs from file name" % k]))
            if k in beads:
                problems.append((str(f), ["duplicate key %s also in %s" % (k, beads[k]["path"])]))
                continue
            beads[k] = fm
    return beads, problems
