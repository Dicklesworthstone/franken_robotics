# Reconciliation pass: overlaps between groups, and gaps

About 3,500 bead definition files were drafted from the FrankenRobotics plan by 71 independent drafters and then
integrated group by group. The integrators could not edit beads outside their own group, so they **reported** two
kinds of leftover: pairs of beads in different groups that overlap, and things the plan requires that no bead
carries. This pass settles both before the beads are loaded into the tracker. Your prompt gives you one packet:
either overlaps (`dup-NN`) or gaps (`gap-X`).

Common rules:

- Bead content rules: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md`, sections
  4 to 7. Direction rules for dependencies: the section "Direction rules for dependencies" of
  `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/INTEGRATION_BRIEF.md`. Read both before editing.
- The index of all live beads is `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/INDEX.tsv` (key,
  unit, type, milestone, priority, parent, gates, plan sections, title; tab separated; grep it). A bead's file is
  `…/beads/defs/<unit>/<key>.md`. The plan is
  `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37,000 lines; read by
  line range; find a chapter with `grep -n '^## '`).
- Graph helper: `python3 /Users/jemanuel/projects/franken_robotics/planning/fable/beads/tools/graph_check.py path A B`
  (does A already depend on B?), `… deps A`, `… rdeps A`.
- Never delete a file. Never run `br` or `bv`. Do not launch sub-agents. Do not implement anything: the project is
  at the planning stage. DO NOT OVERSIMPLIFY and DO NOT LOSE ANY FEATURE OR FUNCTIONALITY: when two beads are
  merged, everything either one holds survives in the kept bead.
- Write your result file early and rewrite it as you go, so that partial work survives an interruption. If it
  exists when you start, read it and continue.

## Overlap packets (`dup-NN`)

Your packet `…/beads/integration/xwork/dup-NN.md` lists clusters of beads connected by reported overlaps, with what
each integrator said. You may edit the files of the beads named in your packet, and no others. For each cluster,
read every bead in it, then decide each pair:

- **merge**: the two specify the same piece of work. Keep one: the seed bead (`BD-nnn`) if one is involved;
  otherwise the bead in the chapter that owns the mechanism; otherwise the one more beads depend on
  (`graph_check.py rdeps`); otherwise the fuller one. Fold into the kept bead everything the other has that it
  lacks (acceptance criteria, tests, numbers, refusals, considerations, dependencies, gates, plan references),
  then mark the other by adding the line `superseded_by: "<kept key>"` to its front matter. Do not empty it.
- **rescope**: they overlap but each has work the other lacks. Keep both. Rewrite the Scope section of each so
  that it states exactly what is in this bead and names the bead that has the rest, remove from each body the
  specification that now belongs to the other (move it there if the other lacks it), and connect them: a
  dependency in the direction the rules give, or `related`.
- **link**: they are different work on a shared subject (a bet and the kill gate that decides it; a registry and a
  mechanism; a card and the mechanism it tests). No body change; record the edge the direction rules give.
- **none**: the report was mistaken.

Where two beads state **different values or conventions for the same thing** (a number, a formula, a file path, a
schema name, a refusal code, an exit code, a default), that is a defect whatever you decide: read the plan section
they cite, correct the one that departs from the plan, and if the plan does not settle it choose one, label the
choice DESIGN with its reason in both beads, and record it under `fixes`. Two beads must never specify the same
artifact differently.

Result file `…/beads/integration/xout/dup-NN.json`:

```
{
 "group": "dup-07",
 "decisions": [
   {"a": "N00-07", "b": "N38a-09", "action": "merge", "keep": "N38a-09", "note": "pilot ledger; completeness check and Moonshot cap folded in"},
   {"a": "N36-20", "b": "N21c-35", "action": "rescope", "note": "N21c-35 records the decision; N36-20 enforces it"},
   {"a": "BET-17", "b": "N16b-12", "action": "link", "note": "the bet reads the kill-gate record"}
 ],
 "superseded": [{"drop": "N00-07", "keep": "N38a-09", "note": "…"}],
 "edges": [{"bead": "N36-20", "dep": "N21c-35", "as": "dep", "why": "enforcement follows the decision"},
           {"bead": "BET-17", "dep": "N16b-12", "as": "dep", "why": "the bet's decision reads the kill-gate record"}],
 "fixes": [{"bead": "N38b-47", "what": "fixture inertia set to 1.1 kg·m² to agree with N14b-03 and the plan's §14.x"}]
}
```

Every pair of your packet appears once under `decisions`. An `edges` entry with `"as": "related"` overrides an
existing dependency between the same two beads.

## Gap packets (`gap-X`)

Your packet `…/beads/integration/xwork/gap-X.md` lists reported gaps: something the plan requires that the reporter
found no bead for. For each gap:

1. **Check that it is still open.** Later integrators wrote new beads; grep `INDEX.tsv` (and the bead files, for
   an identifier) for a carrier. If one exists, record it under `already`.
2. **Check that it is real.** Read the plan section the gap cites. If the plan does not require it, or another
   part of the plan forbids it, record it under `rejected` with the reason and the plan section.
3. **Otherwise write the missing bead**, to the charter's full standard (front matter with every field; the seven
   body sections; a real test plan with unit tests, an end-to-end script with structured logging, known answers
   and negative controls). Put it in the unit directory of the chapter that owns the subject (find the chapter's
   units with `ls …/beads/defs/`; where a chapter has several units, the one whose beads cover the neighbouring
   subsections). Its key is that unit's prefix followed by `x` and a two-digit number, for example `N33bx-01`
   (the `x` keeps your keys from colliding with anyone else's). Choose its `parent` (a work-package epic `WP-nn`
   that owns the crate: look at the parent of neighbouring beads), its milestone (the gate register of the plan's
   §40 governs), its `deps` (real keys, by the direction rules), and record under `edges` a dependency from each
   bead that needs it (`needed_by` in the packet) to the new bead, unless the direction rules make it `related`.
   Several reports of the same gap get one bead. A gap that is a decision for the project owner gets a decision
   bead (label `owner-decision`) stating the options, the recommendation and the fail-closed default that holds
   until it is decided.

You create new files only; you do not edit existing bead files.

Result file `…/beads/integration/xout/gap-X.json`:

```
{
 "group": "gap-A",
 "filled": [{"gaps": [12, 31], "bead": "N33bx-01", "note": "admission of fs-checker as a re-check oracle"}],
 "already": [{"gaps": [5], "bead": "N35-33", "note": "the refusal-number registry now exists"}],
 "rejected": [{"gaps": [44], "why": "§33.9 forbids any cargo edge to fs-checker; the need is met out of process by N33bx-01"}],
 "new_beads": ["N33bx-01"],
 "edges": [{"bead": "N25a-32", "dep": "N33bx-01", "as": "dep", "why": "gap 12"}]
}
```

Every gap of your packet appears exactly once, under `filled`, `already` or `rejected`.

## When you finish

Check that your result file parses and that every key in it exists (in `INDEX.tsv`, or among your new beads).
Reply in at most 100 words: for an overlap packet, pairs merged, rescoped, linked and dismissed, and conflicting
values fixed; for a gap packet, gaps filled (and beads written), already carried, and rejected.
