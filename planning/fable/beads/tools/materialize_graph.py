#!/usr/bin/env python3
"""Write the final graph back into the definition files, so that each file states its own edges.

After this, a bead's `deps`, `related` and `parent` in its front matter ARE its edges (subject to the safety rules of
apply_integration.py --defs-only), and `needs` is empty: every prerequisite that was free text is now an edge or was
judged not to be one (the record is in integration/out/*.json and integration/worksheets/).

Only the four front-matter lines change. Bodies and all other fields are left byte for byte.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

beads, problems = load_all()
graph = json.loads((BEADS / "integration" / "final_graph.json").read_text(encoding="utf-8"))["beads"]
changed = 0
for k, g in graph.items():
    b = beads[k]
    p = Path(b["path"])
    lines = p.read_text(encoding="utf-8").split("\n")
    end = next(i for i in range(1, len(lines)) if lines[i].strip() == "---")
    want = {"deps": "deps: " + json.dumps(g["deps"], ensure_ascii=False),
            "related": "related: " + json.dumps(g["related"], ensure_ascii=False),
            "needs": "needs: []"}
    if k != "EPIC-PROGRAM" and g["parent"]:
        want["parent"] = "parent: " + json.dumps(g["parent"], ensure_ascii=False)
    seen = set()
    head = []
    for ln in lines[1:end]:
        name = ln.split(":", 1)[0].strip() if ":" in ln else None
        if name in want:
            if name in seen:
                continue
            head.append(want[name]); seen.add(name)
        else:
            head.append(ln)
    for name in ("parent", "deps", "needs", "related"):
        if name in want and name not in seen:
            head.append(want[name])
    new = "\n".join([lines[0]] + head + lines[end:])
    if new != "\n".join(lines):
        p.write_text(new, encoding="utf-8")
        changed += 1
print("definition files rewritten:", changed, "of", len(graph))
