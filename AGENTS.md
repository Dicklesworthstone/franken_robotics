# Project scope

FrankenRobotics is in the implementation stage. On 2026-11-23 the project owner changed the scope: the plan has
been converted into Beads, and implementing those Beads is now the work. Work on the plans, their research evidence
and their review continues beside it.

## Where the work is

- The work items are the Beads in this repository's `br` tracker (`.beads/`, prefix `frob`): about 3,640 of them,
  each self-contained, with its acceptance criteria, unit tests and end-to-end script. Find work with
  `br ready --json` or `bv --robot-triage` (never bare `bv`), take Beads in dependency order, and close a Bead only
  when everything its acceptance criteria name passes.
- `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` is the specification the Beads came from. A Bead is meant to be
  enough on its own; where a Bead and the plan disagree, say so in the Bead before building either reading.
- Registry Beads are the authority for what they register: the verb registries `VERBS-<cluster>`, the refusal,
  known-answer and counterexample registries (`…-REF`, `…-KA`, `…-CX`), and the owner Beads `NJL-01`, `STD-E2E`,
  `STD-TEST` and `BD-025`. Where a mechanism Bead and its registry row differ, the registry row holds. The
  program-wide conventions are in `planning/fable/beads/polish/KNOWN_DRIFTS.md`.

## Changing Beads

- Every Bead has a staging file, `planning/fable/beads/defs/<unit>/<key>.md`, and the tracker is filled from those
  files by `planning/fable/beads/tools/sync_beads.py` (which uses only `br`). A change made with `br` alone to a
  Bead's description, labels or dependencies is overwritten by the next sync unless the staging file carries it too.
  Status changes (`br update --status`, `br close`) are not touched by the sync.
- Every sibling project (frankensim, fastmcp_rust, asupersync and the rest) is the owner's: a change a Bead needs
  in a sibling is made in that sibling's repository, by the upstream Beads (`FD-…`), not worked around here.
