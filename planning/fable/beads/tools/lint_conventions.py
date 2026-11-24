#!/usr/bin/env python3
"""Mechanical checks of the settled conventions (polish/KNOWN_DRIFTS.md) over every live bead.

  lint_conventions.py [--write <round>]     prints counts; with --write, saves polish/round-<N>/lint_findings.json

A finding is something for a reviewer to look at, not a verdict: the owner beads legitimately quote the retired forms
when they say what is retired. Findings are worded so that a reviewer can settle each in one look.
"""
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from defs_lib import BEADS, load_all  # noqa: E402

beads, _ = load_all()
live = {k: b for k, b in beads.items() if not b.get("superseded_by")}
graph = json.loads((BEADS / "integration" / "final_graph.json").read_text(encoding="utf-8"))["beads"]

OWNERS_OF_RETIRED_FORMS = {"BD-025", "N38a-02", "N38a-09", "STD-E2E", "STD-TEST", "N40-03", "N38b-19", "N38a-12"}
RUN_VERBS = r"frob (?:sim run|eval run|train start|study (?:start|run)|opt (?:start|run|cascade)|nemesis (?:search|bound)|mission (?:query|refute|synthesize)|phylum sweep|ground (?:fit|calibrate)|solve run|autopsy (?:intervene|rerun))\b"


_REACH = {}


def reaches(src, target):
    memo = _REACH.setdefault(target, {})
    if src in memo:
        return memo[src]
    stack, seen = [src], {src}
    found = False
    while stack:
        x = stack.pop()
        if x == target:
            found = True; break
        for y in graph.get(x, {}).get("deps", ()):
            if y not in seen:
                seen.add(y); stack.append(y)
    memo[src] = found
    return found


def not_negated(line):
    return not re.search(r"\b(no|not|never|retired|there is no|refuses?|lint|means the cargo form|wrong form|is not a|instead of|replac|superseded|drift|forbid)", line, flags=re.I)


find = defaultdict(list)
count = Counter()
for k, b in live.items():
    body = b["body"]
    labels = {str(x).lower() for x in b.get("labels", [])}
    is_epic = b.get("type") == "epic"
    lines = body.split("\n")

    def flag(code, text):
        find[k].append("[%s] %s" % (code, text)); count[code] += 1

    if k not in OWNERS_OF_RETIRED_FORMS:
        hits = [ln.strip()[:140] for ln in lines if re.search(r"frob[- ]conformance (run|record)|\bfrob conformance\b|\bfrob gauntlet\b|\bfrob gates\b", ln) and not_negated(ln)]
        if hits:
            flag("C1-command", "uses a Gauntlet command that does not exist (convention 1), e.g. `%s`" % hits[0])
        hits = [ln.strip()[:140] for ln in lines if re.search(r"--test g[0-5](?![_a-z0-9])", ln) and not_negated(ln)]
        if hits:
            flag("C1-target", "names a tier target by its short name (`--test g1`); the targets are g0_laws, g1_known_answers, g2_parity, g3_metamorphic, g4_chaos, g5_determinism (convention 1), e.g. `%s`" % hits[0])
        if re.search(r"frob\.pilot/1", body) and any(not_negated(ln) for ln in lines if "frob.pilot/1" in ln):
            flag("C3-pilot", "writes the retired pilot schema name `frob.pilot/1`; it is `frob.gauntlet.pilot/1` (convention 3)")
        if "docs/delivery/adr/" in body and any(not_negated(ln) for ln in lines if "docs/delivery/adr/" in ln):
            flag("C4-adr", "puts a decision record under `docs/delivery/adr/`; every decision record lives in `docs/adr/` (convention 4)")
        hits = [ln.strip()[:140] for ln in lines if re.search(r"known_answers/[a-z_]+\.toml|registry/(?:controls|counterexamples)/[a-z_]+\.toml", ln) and not_negated(ln)]
        if hits:
            flag("C2-file", "keeps registry rows in a private or per-chapter file (convention 2), e.g. `%s`" % hits[0])
    hits = [ln.strip()[:150] for ln in lines if re.search(r"--features[ =]\"?(?:[a-z-]+,)*mutants\b|feature `mutants`", ln)]
    if hits and k != "BD-025":
        flag("C1-feature", "builds planted defects with a feature called `mutants`; the feature is `gauntlet-mutations` (convention 1), e.g. `%s`" % hits[0])
    hits = []
    for m in re.finditer(r"\b(?:is|are|was|were|being)? ?requested (?:of|from) `?[A-Z][A-Za-z0-9-]+|\ba request (?:to|of) `?[A-Z][A-Za-z0-9-]+`? asks\b|"
                         r"\buntil `?[A-Z][A-Za-z0-9-]+`? (?:registers|adds|allocates|answers)\b|\bonce `?[A-Z][A-Za-z0-9-]+`? (?:registers|allocates)\b|"
                         r"logs? [`\"]?not[- ]run[`\"]? (?:until|while|unless)", body):
        hits.append(body[max(0, m.start() - 60):m.end() + 60].replace("\n", " "))
    if hits:
        flag("C13-pending", "%d place(s) narrate a pending request instead of stating the owner's form (convention 13), e.g. \"…%s…\"" % (len(hits), hits[0]))
    # forms the registries dropped in round 4 (NJL-01's action rule, VERBS-eval's battery form, the six run properties)
    hits = [ln.strip()[:150] for ln in lines if "--battery" in ln and re.search(r"--run-seed\b|--worlds\b", ln) and not_negated(ln)]
    if hits and k != "VERBS-eval":
        flag("R4-battery", "passes `--run-seed` or `--worlds` beside `--battery`: the battery form of `eval.run` takes neither (VERBS-eval; it is refused with 802, "
             "because a battery's items carry their own world addresses), e.g. `%s`" % hits[0])
    hits = [ln.strip()[:150] for ln in lines if re.search(
        r"frob ground band [^`\n]*--retire\b|frob spark cache [^`\n]*--(?:evict|list)\b|frob task check [^`\n]*--coupling\b|frob scene bank (?:admit|retire|list|show)\b|"
        r"frob nemesis hardset (?:add|remove|list|show)\b|frob eval ledger (?:open|close|seal)\b|frob nemesis audit (?:prepare|run)\b", ln) and not_negated(ln)]
    if hits and not k.startswith("VERBS-") and k != "NJL-01":
        flag("R4-action", "selects a verb's action with a per-action flag or a positional word; NJL-01's action rule admits only `--action <name>` (and an action of kind admin "
             "takes no `--key`): check the verb's row in its VERBS bead, e.g. `%s`" % hits[0])
    if re.search(r"variations 10 to 13|\bfive (?:common )?run properties", body) and k != "NJL-01":
        flag("R4-count", "says 'variations 10 to 13' or 'five run properties': NJL-01 now has variations 10 to 16 and six common run properties (with `envelope_scope`)")
    hits = [ln.strip()[:170] for ln in lines if re.search(r"exit(?:s| code| status)? 8\b", ln) and "frob " in ln and "sync-wait" not in ln and "MCP" not in ln]
    if hits and k not in ("BD-022", "NJL-01"):
        flag("R4-exit8", "expects exit 8 (a job handle) from a `frob` call that shows no `--sync-wait-ms`: the command line waits to completion by default (BD-022's flag table; convention 16), "
             "so either pass `--sync-wait-ms <n>` or drop the exit-8 expectation and the `frob job wait`, e.g. `%s`" % hits[0])
    hits = [ln.strip()[:170] for ln in lines if "check-stack" in ln and "--write-receipt" not in ln
            and re.search(r"writes? (?:a |the )?(?:new |fresh )?(?:stack |resolution )?receipt|new (?:resolution|stack) receipt", ln)]
    if hits and k not in ("N33a-05", "N33a-04", "BD-004"):
        flag("R4-receipt", "says plain `cargo xtask check-stack` writes a new stack receipt: plain `check-stack` only checks, and a receipt is written by the `--write-receipt` form with its reason and gate "
             "(see N33a-05 for the exact options; the same error was corrected in thirteen upstream beads this round), e.g. `%s`" % hits[0])
    hits = [ln.strip()[:170] for ln in lines if "--release" in ln and re.search(r"gauntlet-mutations|--features[ =]\"?[a-z0-9,-]*conformance", ln) and not_negated(ln)]
    if hits:
        flag("R4-release", "builds planted defects or conformance features in a release build, where they do not compile (convention 17), e.g. `%s`" % hits[0])
    if re.search(r"exits? non-?zero (?:when|if|whenever)[^.\n]{0,80}falsifier (?:holds|is met|fires)", body):
        flag("R4-pilot", "its script exits non-zero when the falsifier holds: a pilot's verdict is data (convention 15), so the script exits zero once the measurement is made and the pilot record written, "
             "and logs falsifier values with `e2e_metric … --observe`")
    if is_epic:
        continue
    # run verbs without a key: only whole commands inside fenced blocks, continuation lines joined
    nokey = []
    in_code = False
    cmd = ""
    for ln in lines:
        if ln.lstrip().startswith("```"):
            in_code = not in_code; cmd = ""; continue
        if not in_code:
            continue
        cmd = (cmd + " " + ln.strip()) if cmd else ln.strip()
        if cmd.endswith("\\"):
            cmd = cmd[:-1].rstrip(); continue
        if re.search(RUN_VERBS, cmd) and "--key" not in cmd and not cmd.lstrip().startswith("#") and "--help" not in cmd \
                and "--plan" not in cmd and "--dry-run" not in cmd and "..." not in cmd and "…" not in cmd:
            nokey.append(cmd[:150])
        cmd = ""
    if nokey:
        flag("C5-key", "%d complete command(s) of a run verb in a code block carry no `--key` (convention 5), e.g. `%s`" % (len(nokey), nokey[0]))
    has_script = bool(re.search(r"scripts/e2e/[A-Za-z0-9_./<>-]+\.sh", body))
    if has_script and not reaches(k, "STD-E2E"):
        flag("C11-std", "names an end-to-end script but does not depend, directly or through its dependencies, on `STD-E2E`")
    mechanism = b.get("type") == "feature" or (b.get("type") == "task" and not (labels & {"registry", "owner-decision", "kill-gate", "exit-review", "decision-record", "spec", "card", "bet", "gate"}))
    if mechanism and not has_script and not re.search(r"-(REF|KA\d?|CX)$|^VERBS-|^WP-|^EPIC-", k):
        flag("C11-script", "has no end-to-end script of its own (`scripts/e2e/<name>.sh`): convention 11 gives each mechanism bead one, unless it delivers nothing a `frob` command can reach (then say so in the test plan)")
    if re.search(r"target/gates/[A-Za-z0-9_.*-]+\.json|writes? (?:the|a|its) gate (?:row|record)", body) and "record_gate" not in body:
        flag("C9-recorder", "appears to write a gate record itself; a script produces a receipt and the gate is recorded with `record_gate` by the one measuring bead (convention 9)")

if "--write" in sys.argv:
    rnd = int(sys.argv[sys.argv.index("--write") + 1])
    d = BEADS / "polish" / ("round-%d" % rnd)
    d.mkdir(parents=True, exist_ok=True)
    (d / "lint_findings.json").write_text(json.dumps(find, indent=0, ensure_ascii=False), encoding="utf-8")
if "--out" in sys.argv:      # --out <path> [--only <code prefix>] : findings of some codes to a file of their own
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
    sel = {k: [t for t in v if t.startswith("[" + only)] for k, v in find.items()}
    Path(sys.argv[sys.argv.index("--out") + 1]).write_text(json.dumps({k: v for k, v in sel.items() if v}, indent=0, ensure_ascii=False), encoding="utf-8")
print("live beads: %d; beads with at least one finding: %d; findings: %d" % (len(live), len(find), sum(count.values())))
for c, n in count.most_common():
    print("  %-12s %d" % (c, n))
