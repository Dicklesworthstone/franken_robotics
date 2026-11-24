"""Check that every U27b staging file parses: front matter one `name: value` per line, strings quoted,
lists one-line JSON, and the seven body sections in order. Also validate the result JSON."""
import json
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/jemanuel/projects/franken_robotics/planning/fable/beads")
SECTIONS = ["## Background", "## Scope", "## Technical approach", "## Acceptance criteria",
            "## Test plan", "## Considerations", "## References"]
LISTS = {"labels", "deps", "needs", "related", "gates", "plan_refs"}
bad = 0
for p in sorted((ROOT / "defs/U27b").glob("*.md")):
    if p.name == "coverage.md":
        continue
    text = p.read_text()
    m = re.match(r"---\n(.*?)\n---\n", text, re.S)
    if not m:
        print(p.name, "no front matter"); bad += 1; continue
    for line in m.group(1).splitlines():
        k, sep, v = line.partition(": ")
        if not sep:
            print(p.name, "bad line", line); bad += 1; continue
        if k in LISTS:
            try:
                val = json.loads(v)
                assert isinstance(val, list)
            except Exception as e:
                print(p.name, "bad list", k, e); bad += 1
        elif k == "priority":
            if not v.isdigit():
                print(p.name, "bad priority", v); bad += 1
        else:
            try:
                assert isinstance(json.loads(v), str)
            except Exception:
                print(p.name, "bad string", k, v); bad += 1
    body = text[m.end():]
    heads = [l for l in body.splitlines() if l.startswith("## ")]
    if heads != SECTIONS:
        print(p.name, "sections", heads); bad += 1
try:
    r = json.loads((ROOT / "polish/round-1/out/U27b.json").read_text())
    print("result json ok:", r["reviewed"], "reviewed,", len(r["changed"]), "changed,", r["unchanged"], "unchanged")
except Exception as e:
    print("result json BAD", e); bad += 1
print("problems:", bad)
sys.exit(1 if bad else 0)
