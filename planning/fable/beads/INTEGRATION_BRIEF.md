# Integration pass: connecting the drafted beads to each other

The FrankenRobotics plan has been drafted into about 3,500 bead definition files by 71 independent drafters, one
unit each. Each drafter could see only its own unit, so three things are still wrong, and this pass fixes them
before the beads are loaded into the tracker:

1. **Prerequisites left as free text.** A drafter who needed something another unit owns wrote a `needs` entry in
   prose instead of a dependency. Each must become a real edge to the bead that delivers it.
2. **Duplicates and overlaps.** Two drafters sometimes wrote a bead for the same piece of work (two units of one
   chapter, or a seed bead and another unit's component bead).
3. **Requests addressed to other units.** Every unit's `coverage.md` lists, in its sections (d) and (e), edges that
   other units' beads should gain and contradictions it found.

You own one **group**: one or more unit directories under
`/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/`. Your prompt names it.

## What you may touch

- Edit files only inside your own group's unit directories, and write your result file
  `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/integration/out/<group>.json`.
- Never delete a file. Never run `br` or `bv`. Do not launch sub-agents. Do not implement anything: the project is
  at the planning stage.
- The rules for bead content are in `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md`
  (sections 4 to 7). Read them before editing or writing a bead. The plan itself is
  `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37,000 lines: read by
  line range, find a chapter with `grep -n '^## '`).

## Your inputs

| Input | Path |
|---|---|
| Your worksheet: the beads of your group, and every unresolved prerequisite with mechanical candidates | `…/beads/integration/worksheets/<group>.md` |
| The index of all beads: key, unit, type, milestone, priority, parent, gates, plan sections, title (tab separated; grep it, do not read it whole) | `…/beads/INDEX.tsv` |
| Any bead's full definition | `…/beads/defs/<unit>/<key>.md` |
| Your units' coverage files | `…/beads/defs/<unit>/coverage.md` |
| A graph helper | `python3 /Users/jemanuel/projects/franken_robotics/planning/fable/beads/tools/graph_check.py path A B` prints whether bead A already depends on bead B, directly or through other beads (so that making B depend on A would be a cycle); `… deps A` lists A's dependencies; `… rdeps A` lists the beads that depend on A |

The graph helper sees the dependencies the drafters wrote in `deps`; it does not see what other integrators
are deciding now. A final global check demotes any edge that would close a cycle, so follow the direction rules
below: they are what keeps the graph acyclic.

## Direction rules for dependencies

The drafted prerequisites are loosely worded: many say "this bead and that bead use the same thing", which is
not a dependency. A dependency means order of work: the carrier is finished first. These rules decide direction.

1. **Time.** A bead never depends on a bead of a later milestone.
2. **Seed order.** Among seed beads (`BD-nnn`) the plan's own graph points only from a higher number to a lower
   number. Keep that: a seed never depends on a higher-numbered seed. If it seems to need one, the link is
   `related`, or the real prerequisite is a smaller bead.
3. **Capstones.** A seed bead whose done-condition names a gate, with component beads that build its parts,
   depends on its components. A component never depends on its capstone. A bead elsewhere that needs the whole
   capability depends on the capstone; one that needs a single part depends on that component.
4. **Tests follow mechanisms.** A test, end-to-end, gate-measurement or exit-review bead depends on the mechanism
   it exercises. A mechanism never depends on its own test bead, and never on an end-to-end bead.
5. **Registries come first.** A registry bead (`…-KA`, `…-CX`, `…-REF`, `VERBS-…`) is data entry that can be done
   early: it depends only on the conformance or registry skeleton. A mechanism or test bead may depend on it. A
   registry bead never depends on a mechanism.
6. **Specifications come first.** A specification bead (`SPEC-C-nn`) or theorem card (`CARD-T-nn`) states the
   contract and defines the law tests before the code exists. Mechanism beads depend on it. It never depends on
   the bead that implements it: the law tests run in the implementing crate as part of that bead's acceptance.
7. **Decisions sit between.** A kill-gate or owner-decision bead depends on the beads that produce the
   measurement it decides on. Beads that exist on only one branch depend on the decision. The mechanism being
   measured never depends on the decision about it.
8. **Pilots come first.** A Frontier or Moonshot mechanism depends on its pilot bead; the pilot depends only on
   what it needs to run on paper or at small scale.
9. **Epics carry no dependencies** and nothing depends on an epic.
10. **Two beads that each seem to need the other** are not both dependencies. Decide which one can be finished
    without the other (usually the one that defines an interface, a schema or a type); the other depends on it.
    If neither can, they are one bead, or the shared piece is a third bead both depend on.

## Task 1: resolve every prerequisite on your worksheet

Some needs name their carrier by key: confirm that the named bead really delivers the need, and decide the kind
of link by the direction rules. For the others, find the bead or beads that deliver what is needed. The candidates
are found by plan section, gate
and title words, so the right carrier is often among them but not always: when no candidate fits, grep `INDEX.tsv`
by section (`§20.4`), gate, crate, verb or keyword, and open the candidate's file when its title is not enough.
Then decide how the need is recorded:

- `dep`: a real prerequisite. The needing bead cannot start, or cannot be finished, until the carrier is finished.
- `related`: connected but not blocking. Use it when the two only share a fixture, a convention or a registry;
  when the carrier is in a **later milestone** than the needing bead (a dependency on later work is wrong: either
  an earlier bead delivers the minimal piece, and that is the `dep`, or the need is not blocking); and when the
  carrier already depends on the needing bead (check with `graph_check.py path <carrier> <needing>`), so that a
  dependency would close a cycle.
- `none`: the need is already satisfied inside the bead itself, or by one of its existing dependencies, or it is
  not a prerequisite at all. Say which.
- `gap`: nothing in the index delivers it. See Task 4.

Prefer the most specific carrier: the mechanism bead, not its registry, its test or its epic. Never resolve a need
to an epic (`EPIC-…`, `WP-…`). A need may resolve to more than one bead.

## Task 2: duplicates and overlaps inside your group

Find beads in your group that specify the same piece of work: read section (e) of your coverage files, where
drafters flagged overlaps, and scan the bead table of your worksheet for the same gate, kill-gate decision,
open-question decision, pilot, verb, registry or end-to-end script appearing twice.

- **True duplicates.** Keep one: the seed bead (`BD-nnn`) if one is involved, otherwise the more complete. Fold
  into the kept bead everything the other has that it lacks: acceptance criteria, tests, numbers, refusals,
  considerations, dependencies. Nothing is lost. Then mark the other as superseded by adding one line to its front
  matter, `superseded_by: "<kept key>"`. Do not delete it and do not empty it.
- **Overlaps with distinct scope.** Keep both. Rewrite the Scope section of each so that it says exactly what is
  in this bead and which bead has the rest, and connect them with a dependency or a `related` entry.
- **A seed bead and its components.** When another unit wrote component beads of a seed bead (the seed's
  done-condition names a gate; the components build the parts), the seed becomes the capstone: it depends on its
  components, and its Scope says so. This is not a duplicate.
- **Across groups** (one bead is yours, the other is not): do not edit either. Report the pair.

## Task 3: requests in your coverage files

Read sections (d) and (e) of each of your units' `coverage.md`. Turn every request for a dependency into an edge
in your result file, whoever owns the beads (for example "BD-044 should depend on N32a-05"): record it, do not
edit the other unit's file. Check each against the graph first: an edge that would close a cycle, or that points
to a later milestone, is recorded as `related`. Where section (e) records a contradiction in the plan that the
drafter settled with a design choice, check that the beads of your group agree with each other about the choice;
fix those that do not.

## Task 4: gaps

A gap is something the plan requires that no bead carries: a need with no carrier, a gate or kill gate of your
chapters that no bead's `gates` field names, a subsection that your coverage files map to no bead.

- If it belongs to your chapters, write the missing bead now, in the unit directory where it fits, to the charter's
  full standard, with a new key: your unit's prefix and the next free number. List it under `new_beads`.
- If it belongs to another chapter, report it under `gaps` with the plan section and what is missing.

## Task 5: consistency inside your group

Fix what you find and list what you changed under `fixes`:

- a bead whose `milestone` is earlier than that of one of its own `deps` (correct the milestone if the gate
  register of §40 says so, otherwise move the dependency to `related`);
- a bead that depends on its own parent epic, or whose `parent` is not an epic;
- an end-to-end bead that does not depend on `STD-E2E`;
- a Frontier or Moonshot mechanism bead that does not depend on its pilot bead;
- a bead that exists on only one branch of a kill-gate decision and does not depend on that decision bead;
- a title over about 110 characters (shorten it without losing the artifact and the outcome);
- a body that points at the plan ("see §18.3") in place of content;
- a dependency cycle: run `graph_check.py cycles`. For each component that contains beads of your group, find the
  edge that breaks the direction rules. If its source bead is yours, move that key from `deps` to `related` in the
  file. If the source bead is another group's, record it under `edges` with `"as": "related"` and
  `"why": "replaces the drafted dep: closes a cycle"`; an `edges` entry with `"as": "related"` overrides a drafted
  dependency between the same two beads.

## Your result file

Write `integration/out/<group>.json`: one JSON object, valid JSON, exactly these fields.

```
{
 "group": "G18",
 "needs": [
   {"bead": "BD-086", "n": 1, "as": "dep", "keys": ["N18b-43"], "note": ""},
   {"bead": "BD-156", "n": 1, "as": "related", "keys": ["BD-179"], "note": "measured on the same kernels; not blocking"},
   {"bead": "N18a-07", "n": 2, "as": "none", "keys": [], "note": "already delivered by its dependency BD-036"},
   {"bead": "N18a-09", "n": 1, "as": "gap", "keys": [], "note": "§20.6: nobody carries the wet-floor sensor model"}
 ],
 "edges": [
   {"bead": "BD-044", "dep": "N32a-05", "as": "dep", "why": "coverage (d) of U32a"}
 ],
 "superseded": [
   {"drop": "N18b-12", "keep": "N18a-19", "note": "both decide KILL-CONTACT; trigger table and fallback folded into N18a-19"}
 ],
 "cross_group_duplicates": [
   {"mine": "N18a-22", "other": "N33c-44", "note": "same arc fixture"}
 ],
 "new_beads": ["N18b-45"],
 "gaps": [
   {"section": "§20.6", "what": "wet-floor sensor model", "needed_by": ["N18a-09"]}
 ],
 "fixes": [
   {"bead": "N18a-31", "what": "title shortened"}
 ]
}
```

`bead` and `n` in `needs` are the key and the need number printed on the worksheet. Every need on your worksheet
appears exactly once. Every key you write must exist in `INDEX.tsv` or be one of your new beads.

## When you finish

Check your result file: it parses as JSON; every need of the worksheet is present; every key exists. Then reply in
at most 120 words: needs resolved as dep, related, none and gap; edges recorded; beads superseded; overlaps
re-scoped; new beads written; cross-group duplicates and gaps reported.
