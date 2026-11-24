You are auditing part of the dependency graph of the FrankenRobotics beads: about 3,640 self-contained work items in the project's `br` issue tracker, converted from a 37,000-line plan for a robotics simulation and design system. Your list is **{S}**: /Users/jemanuel/projects/franken_robotics/planning/fable/beads/polish/round-4/audit/{S}.md . You change no bead. You write proposals to /Users/jemanuel/projects/franken_robotics/planning/fable/beads/polish/round-4/audit/out/{S}.json .

**Why this audit.** Implementation is about to begin with a swarm of agents working in parallel. The tracker offers an agent only a bead whose blocking dependencies (`deps` in the bead's front matter) are all closed. Today exactly one work bead can start (`BD-001`, the workspace skeleton); when it closes six can, then two, three, nine, fourteen. The swarm cannot spread out until about six layers are built, and the longest chain in the graph is 169 beads long (it is in `…/polish/round-4/audit/chain.txt`). Part of that is real: a workspace must exist before a crate, and a crate before the code that uses it. Part of it is not: over four review rounds, reviewers added blocking edges for things that are only related, or that a single acceptance item needs.

**Read first:** `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md` sections 4 to 7 (what a bead contains), the section "Direction rules for dependencies" of `…/beads/INTEGRATION_BRIEF.md`, and `…/beads/polish/KNOWN_DRIFTS.md` (the conventions; 7, 8, 9 and 11 matter most here). A bead's staging file is `…/beads/defs/<unit>/<key>.md`; `…/beads/INDEX.tsv` lists every live bead (grep it); `python3 …/beads/tools/graph_check.py deps A`, `rdeps A` and `path A B` read the graph.

**What to do for every bead of your list.** Read the bead in full. For each entry of its `deps` (for a chain list: the entry in bold first), open the dependency far enough to know what it delivers, and decide:

- **real**: the bead cannot be *finished* (its code, its unit tests, its own end-to-end script, its acceptance criteria) unless what the dependency delivers exists. Keep it. A bead that has its own end-to-end script really depends on `STD-E2E` (the harness); never propose removing that. A bead that uses a crate, a type, a file format, a registry row or a verb really depends on the bead that delivers it.
- **false**: nothing in the bead uses what the dependency delivers. It only cites it, follows it in the plan's order, shares a file name or a fixture name with it, or was linked because both belong to one subsystem. Propose moving it to `related`.
- **partial**: one acceptance item, one test or one script step needs the dependency, and the rest of the bead does not. Propose the smallest change that frees the bead: that item moves to the dependency or to a later bead that has both (convention 7), or a small early bead is split off that delivers only the interface, schema or file format the others need. Say exactly which item moves where, and which bead would then be ready sooner.
- **wrong way or missing**: the edge points the wrong way by the direction rules, or the bead plainly uses something it does not depend on. Report it.

Look also for beads that could start with **no prerequisite at all**: a survey of sibling repositories, a decision record, a document, a measurement that does not use this repository's workspace, a shell script over the bead graph. And for the first layers themselves (`BD-001`, `STD-E2E`, `STD-TEST`, `N34p2-01`, `BD-005`, `BD-025` and their like, if they are on your list): could one be split into parts that do not need each other, so that several agents can work from the first hour? Propose a split only when the parts are truly independent, and name what each part delivers.

**Be conservative.** A prerequisite removed wrongly lets an agent start work it cannot finish, which is worse than a little lost parallelism. Every proposal carries its evidence: the sentence of the bead that shows the dependency is not used, or is used by one item only. When in doubt the edge is real.

**Result file** (valid JSON; write it early and rewrite it after every ten beads):

```
{
 "list": "{S}",
 "audited": 35,
 "edges_judged": 120,
 "real": 101,
 "proposals": [
   {"bead": "N44-03", "kind": "false", "edge": "BD-001", "change": "move BD-001 from deps to related",
    "evidence": "the bead is a shell script over the tracker's export; no step builds or reads the Cargo workspace",
    "gain": "ready at the start instead of after BD-001"},
   {"bead": "BD-002", "kind": "partial", "edge": "N34p2-01", "change": "move acceptance item 4 (the status table parses with registry::toml) to N34-16, which already depends on both; then drop N34p2-01 from deps",
    "evidence": "items 1 to 3 and 5 are documents; only item 4 reads a TOML file", "gain": "BD-002 and its 31 dependents start one layer earlier"}
 ],
 "notes": "what you saw about the shape of this part of the graph, in a few sentences"
}
```

`kind` is one of `false`, `partial`, `split`, `start-ready`, `wrong-way`, `missing`. Reply in at most 120 words: beads audited, edges judged, how many real, proposals by kind, and the two or three with the largest gain. Never run a `br` command that writes, never edit or delete a bead file, do not launch sub-agents, do not implement anything.
