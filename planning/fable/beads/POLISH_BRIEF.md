# Polishing the beads

The FrankenRobotics plan has been converted into about 3,500 beads (self-contained work items) in the project's
`br` tracker. Before anyone implements anything, the beads are polished in repeated rounds until a round changes
almost nothing. You are one reviewer in one round, with one slice of beads. Your prompt names the round and the
slice.

## The owner's instruction for every round

> Reread AGENTS dot md so it's still fresh in your mind. Check over each bead super carefully-- are you sure it
> makes sense? Is it optimal? Could we change anything to make the system work better for users? If so, revise the
> beads. It's a lot easier and faster to operate in "plan space" before we start implementing these things!
>
> DO NOT OVERSIMPLIFY THINGS! DO NOT LOSE ANY FEATURES OR FUNCTIONALITY!
>
> Also, make sure that as part of these beads, we include comprehensive unit tests and e2e test scripts with great,
> detailed logging so we can be sure that everything is working perfectly after implementation. Remember to ONLY
> use the `br` tool to create and modify the beads and to add the dependencies to beads.

Read first, in this order:

1. `/Users/jemanuel/projects/franken_robotics/AGENTS.md` (the project is at the planning stage: beads describe
   work; nobody builds anything) and the owner's ruling
   `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/REVISION_2.md` (every sibling project is the
   owner's own, so siblings are changed in their own repositories; one Rust toolchain T for the whole build graph;
   the MCP server `frob-mcp` is built on fastmcp_rust, sits in the core build graph and runs verbs in process).
2. `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md`, sections 4 to 7: what a bead
   must contain, and the testing standard.
3. The section "Direction rules for dependencies" of
   `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/INTEGRATION_BRIEF.md`.
4. Your slice file (below).

## How a bead is changed

The tracker is changed only through `br`. Thirty reviewers writing to it at once would contend for its write lock,
so each bead has a **staging file**, and after the round the lead's sync tool pushes every changed staging file into
the tracker with `br update`, `br create`, `br dep import`, `br dep remove` and `br close`. Therefore:

- **You edit staging files**: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/<unit>/<key>.md`.
  The front matter holds the bead's title, type, priority, milestone, labels, `parent`, `deps` (blocking
  dependencies) and `related` (non-blocking links): these lines **are** the bead's edges; edit them directly. The
  body is the bead's description.
- **You may read the tracker**, from `/Users/jemanuel/projects/franken_robotics`: `br show <id> --json`,
  `br search "[<key>]"`, `br dep tree <id>`, `br ready --json`, `br blocked --json`, and the graph tool in robot
  mode only (`bv --robot-insights`, `bv --robot-plan`, `bv --robot-priority`, `bv --robot-suggest`). Never run
  bare `bv`. A bead's tracker id is found from its key with `br search "[<key>]"`.
- **You never run a `br` command that writes** (`create`, `update`, `close`, `dep add`, `dep remove`, `sync`,
  `delete`), never delete a file, never launch sub-agents, and never implement anything.

Useful tools: the index of all live beads
`/Users/jemanuel/projects/franken_robotics/planning/fable/beads/INDEX.tsv` (key, unit, type, milestone, priority,
parent, gates, plan sections, title; tab separated; grep it); the graph helper
`python3 /Users/jemanuel/projects/franken_robotics/planning/fable/beads/tools/graph_check.py path A B` (does A
depend on B?), `… deps A`, `… rdeps A`, `… cycles`; the plan itself, for checking that nothing was lost or
simplified: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37,000 lines:
read by line range; each bead's `plan_refs` names the sections it came from; find a section with
`grep -n '^### 18\.3 '`).

## Your slice

`/Users/jemanuel/projects/franken_robotics/planning/fable/beads/polish/round-<N>/<slice>.md` lists your beads (key,
file, title, milestone, parent), and below them: requests that reviewers of earlier rounds addressed to your
beads, conflicts that earlier passes found and could not fix, and findings of the graph tools about your beads.
You edit only the staging files of the beads in your slice (and new beads you create).

## What to check, for every bead of your slice

Read the whole bead. Then ask the owner's three questions (does it make sense; is it optimal; would a change make
the system work better for its users: the engineer at the `frob` command line, and the AI agent driving the same
verbs over MCP), and check:

1. **Self-contained.** An agent who has never seen the plan can implement it: every term defined, formulas with
   every symbol defined, thresholds with their numbers, types, algorithms as steps, file paths, schema names,
   refusal codes with meanings and repairs. No "see §…" in place of content.
2. **Faithful and complete.** Compare with the plan sections in `plan_refs`. Nothing the plan requires is missing
   from your slice's beads; nothing is weakened or simplified; no feature is lost. If something is missing, add it
   to the bead where it belongs, or write a new bead.
3. **Correct.** Recompute the numbers and check the formulas. Units, signs, conventions, identifiers, exit codes
   and names agree inside the bead, with the beads it depends on, and with the registries that own them (the
   refusal registry beads `…-REF`, the known-answer registries `…-KA`, the verb registries `VERBS-…`, the gate
   register). Two beads must never specify the same artifact differently: when you find that, correct the one that
   departs from the plan; if the plan does not settle it, choose, and label the choice DESIGN with its reason.
4. **One coherent piece of work.** Roughly one to five days for one agent. Too large, or mixing two crates or two
   unrelated things: split it (the original keeps its key and narrows its Scope; the new bead depends on it or it
   on the new bead). Two beads for the same work: merge (below). Scope says what is in and what is out, and names
   the bead that has what is out.
5. **Testable, with the tests in it.** Acceptance criteria are observable and checkable by a command. The test
   plan has named unit tests with inputs and expected values (no mocks, no fakes); an end-to-end script
   (`scripts/e2e/<name>.sh`, driving the real `frob` binary on real fixtures) with the structured logging of
   charter section 6 (NDJSON lines with timestamp, script, bead, step, event, the exact command line, exit code,
   wall time, the envelope's evidence block, measured values with units and thresholds, digests, and on failure
   the first differing value and the paths needed to reproduce; a final summary line); known answers with values;
   and negative controls: a planted defect the test must catch, for every gate. A bead whose test plan is thin
   gets a real one.
6. **Dependencies.** Every `deps` entry is a real prerequisite and points the right way by the direction rules;
   no prerequisite is missing (ask of each bead: what must exist before an agent can start this, and before it can
   finish it?); no dependency on a later milestone; no dependency on or from an epic; an end-to-end bead depends on
   `STD-E2E`; a Frontier or Moonshot mechanism depends on its pilot bead; a bead on one branch of a kill-gate
   decision depends on the decision bead. Do not chain beads that could proceed in parallel. A link that is not
   order of work belongs in `related`. Never close a cycle (`graph_check.py path B A` before adding A → B).
7. **Placement.** `parent` is the work-package epic that owns the bead's crate; `milestone` follows the gate
   register (the plan's §40) and is not earlier than that of any dependency; `priority` follows the charter;
   `labels` carry the milestone, subsystem, crate and tier; the title names the artifact and the outcome in at
   most about 110 characters.
8. **Risks and fallbacks.** Considerations names the failure modes, the kill gate with its trigger number and its
   fallback, performance and memory envelopes, security, privacy and licence points, and open questions with their
   fail-closed default.

## Making changes

- **Edit in place** with exact, minimal edits. Keep the seven body sections and the front-matter format (one
  `name: value` per line, strings in double quotes, lists as JSON arrays on one line).
- **A new bead** (a split, or something missing): a new file in the unit directory where it belongs, to the
  charter's full standard. Key: the unit's prefix, then `p`, the round number, a hyphen and a two-digit number, for
  example `N18bp2-01` for unit U18b in round 2 (look at a neighbouring bead's key for the prefix).
- **A merge**: fold everything the dropped bead has into the kept one, then add `superseded_by: "<kept key>"` to
  the dropped bead's front matter. Do not empty or delete it. Beads that depended on the dropped one are redirected
  to the kept one automatically.
- **A bead outside your slice** needs a change: do not edit it. Record a request (below).
- **Do not change for the sake of change.** A bead that is right is left alone. Rewording that changes no meaning
  is not a change. A round that finds little is the goal of polishing, not a failure: report what you found,
  truthfully.

## Your result file

`/Users/jemanuel/projects/franken_robotics/planning/fable/beads/polish/round-<N>/out/<slice>.json`, valid JSON:

```
{
 "round": 2,
 "slice": "U18b",
 "reviewed": 51,
 "changed": [
   {"bead": "BD-156", "kinds": ["correct", "tests"], "what": "K-chol pivot threshold was 1e-6 in the body and 1e-8 in acceptance; plan §18.11 says 1e-8. Added the negative control for a singular block."},
   {"bead": "N18b-07", "kinds": ["deps"], "what": "added dep BD-148 (the model table it reads); moved N18b-22 to related (shares a fixture only)"}
 ],
 "new_beads": [{"bead": "N18bp2-01", "why": "split from N18b-30: the rut memory test fixture was a second piece of work"}],
 "superseded": [{"drop": "N18b-12", "keep": "N18b-09", "why": "same end-to-end script"}],
 "requests": [{"bead": "N21b-08", "what": "its K-chol call passes the block size in bytes; BD-156 specifies elements"}],
 "unchanged": 44,
 "notes": "one or two sentences on the state of the slice"
}
```

`kinds` is any of: `sense`, `optimal`, `users`, `self-contained`, `complete`, `correct`, `scope`, `split`,
`tests`, `e2e-logging`, `deps`, `placement`, `risks`, `title`. Write the result file early and rewrite it after
every ten beads, so that partial work survives an interruption; if it exists when you start, read it and continue
after the last bead it mentions. Every bead of the slice is either in `changed`, in `superseded`, or counted in
`unchanged`.

## When you finish

Check that every file you touched still parses (front matter between two `---` lines, every field present, lists
valid JSON, the seven body sections present) and that your result file is valid JSON. Reply in at most 100 words:
beads reviewed, changed (by kind), new, merged, requests made, and whether the slice looks settled.
