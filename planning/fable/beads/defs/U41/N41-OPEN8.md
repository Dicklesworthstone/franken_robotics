---
key: "N41-OPEN8"
title: "registries/open.toml, part 8: program questions and those adopted from Gauntlet and ten-fold (20 rows, 2 aliases)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "tenfold", "registry", "solid", "owner-decision"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: []
related: ["N41-04", "N41-OPEN1", "N41-OPEN2", "N41-OPEN3", "N41-OPEN4", "N41-OPEN5", "N41-OPEN6", "N41-OPEN7"]
gates: []
plan_refs: ["§42.2", "§42.3", "§42.4", "§38.16", "§39.19"]
---
## Background

Eighth and last part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1).
It holds the questions that belong to no subsystem:

- **Program level** (OPEN-PROG-1 to -5 and OPEN-GAUNT-1): decisions that no measurement inside the program can
  take. Who outside can replicate the result; who adjudicates the ten-fold test; whether an adapter is a supported
  product surface; whether compute may be spent on the decisive early experiment; in what order Moonshot items
  become active; which real dataset enters the Reality lane.
- **Adopted from the Gauntlet** (the verification harness; OPEN-GAUNT-2 to -8): tolerances and negative controls
  still to be stated for known answers, which incumbent release serves as the development oracle, runtime budgets.
- **Adopted from the ten-fold test** (OPEN-TENFOLD-1 to -9): the adjudicator, rival advocates, compute, deadlines,
  the agent model, licences, ratification of thresholds, the fourth arm.

The ten-fold test in one paragraph: on a sealed suite of mission families, fresh agents using this product and the
strongest composed incumbent tooling (advocated by sessions whose job is to make the rival strong) each go from a
brief to an independently accepted answer; the result is the ratio of restricted mean times per stratum, with a
decision rule and its constants fixed before sampling (the wording thresholds 3 and 10, a guard floor of 5, a tie
factor of 1.5, a margin of 0.05 and at most four confirmatory cells), a partial gate at M3 and the tournament at
M7; the result is published whatever it is.

## Scope

In: the twenty rows and two aliases below as records of `registries/open.toml`. Out: the schema and loader
(N41-03); the decisions (N41-05 to N41-10 and N41-15 in this unit; the others as named); the owner calendar and
the cost of each owner default printed in it (N40-35); the checks that need the whole register or another
registry, run by N41-04, which depends on all eight parts, on the owner calendar and, through its other
dependencies, on the known-answer registry (N38a-12) and the gate register (N40-02): the total of 161 with
`check --complete`, the owner's table resolved on the real rows, the counts of OPEN-GAUNT-2 and -3 from the
known-answer registry, the ten-fold constants against the gate register, and the calendar's two listings of
OPEN-TENFOLD-4.

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-PROG-1 | Public or private repository, the licence file, and who outside the program can replicate the ten-fold result | Private; the suite's licence file; the replication kit and `frob-verify` are the artifacts prepared for a third party | Owner | First push; the pin freeze of M7 | M0; again before the M7 pin freeze | N41-05 |
| OPEN-PROG-2 | The adjudicating simulator and the neutral party for the ten-fold test: which evaluator, independent of both arms, runs the acceptance batteries; who seals the suite, holds the receipts and releases them after both arms close | A pinned incumbent CPU engine that neither arm trains in, run by a fresh-context session with no write access to either arm; ratios labelled `self-adjudicated` | Owner | The M3 partial gate; the M7 tournament | M2, before any sealed world family exists | N41-06 |
| OPEN-PROG-3 | Whether the Gymnasium-compatible adapter ships as a supported binding | Built and gated for the ten-fold test and for conformance only; not a supported public surface; no interpreter in any `frob` binary | Owner | M3 | M3 | N41-07 |
| OPEN-PROG-4 | Approval of compute for the rank-agreement experiment (about forty per-design trainings in one morphology class; about 5 device-hours where specialists are reused and about 45 otherwise) | Not run above the default envelope; cheap rungs are `uninformative` | Owner | GATE-CORTEX-12; GATE-ATEL-07 | M1 | N41-08 |
| OPEN-PROG-5 | The order in which admitted Moonshot items become active under the cap of four | None before its pilot passes; then the order in which the frontier milestones list their epics | Owner | M6, M8 | M5 | N41-09 |
| OPEN-GAUNT-1 | Which real-robot dataset, if any, is admitted to the Reality lane first, with what licence and provenance | `reality: no-claim` everywhere; the second-simulator stand-in is labelled `synthetic` | Owner | M5; GATE-GROUND-10 | M5 | N41-10 (owner packet); N38b-26 |
| OPEN-GAUNT-2 | Tolerances for the 133 known-answer rows printed `to set` | The row is not counted as passing | Each owning chapter | GATE-GAUNT-02 | The milestone of each row's owner section | N38b-27; each chapter's known-answer registry bead |
| OPEN-GAUNT-3 | Negative controls for the 325 known-answer rows printed `none stated`, and mutations for the rows whose stated control is a boundary behaviour | The row supports no gate | Each owning chapter | GATE-GAUNT-01 | The milestone of each row's owner section | N38b-28; each chapter's known-answer registry bead |
| OPEN-GAUNT-4 | Which incumbent release is the development oracle, and which releases and container digests form the comparator set | No development oracle: pilots that need dynamics before M1 wait for the CPU referee | The rival audit of the ten-fold test | GATE-GAUNT-05; every pilot that needs dynamics before M1 | M0 | N38b-35 (with N38a-04, N38a-06) |
| OPEN-GAUNT-5 | The Gauntlet's runtime budget per change, per night and per release on each machine | The `smoke` and `fixtures` tiers per change; everything else nightly | Performance program | The schedule of verification tiers G1 to G5 | M1 | N38b-36 |
| OPEN-GAUNT-6 | Reconciliation of the known-answer values flagged as inconsistent between a chapter and the suite | The row keeps the printed value and is flagged; the gate cites the derivation file | The lead; the owning chapters | Any gate that cites one of them | The milestone of the citing gate | N38b-61 |
| OPEN-GAUNT-7 | The scenario lists of the standard battery for wheeled, tracked, wheel-legged and manipulating classes | The class has no fidelity claim until its battery is frozen | Arena with Kinesis | GATE-FID-01 for those classes | M2 | N38b-37 (with N38a-20, which superseded N16b-01) |
| OPEN-GAUNT-8 | Whether 10⁴ scenarios per relation per tier per release is affordable for the metamorphic relations that need the referee | Referee relations run at 10³ and print the reduced count | Performance program | GATE-GAUNT-03 | M2 | N38b-38 |
| OPEN-TENFOLD-1 | Which simulator adjudicates each stratum, and whether it can represent terrain, films and tracks | A pinned incumbent CPU engine that no arm trains in; where it cannot represent the mission, cross-home adjudication, labelled | The adjudicator with the Gauntlet lead | GATE-TENFOLD-05 | M2 | alias of OPEN-PROG-2 (N41-06); N39b-26 |
| OPEN-TENFOLD-2 | Who the adjudicator is | A fresh-context session with no write access to any arm; every ratio is then labelled `self-adjudicated`, and the word waits for an external re-run of the kit | Owner | Before any sealed world family exists | M2 | alias of OPEN-PROG-2 (N41-06); N39b-27 |
| OPEN-TENFOLD-3 | Who the rival advocates are, and their budget | Agent sessions that have contributed nothing to FrankenRobotics, 40 device-hours per stratum, and the label `rival: internally advocated` on every result | Owner | GATE-TENFOLD-04 | M2 | N41-15 (owner packet); N39b-28 |
| OPEN-TENFOLD-4 | The compute envelope for the partial gate and the tournament: a campaign class for each run, and the pretraining class where an arm builds a standing asset | Nothing above the default class runs, the gate is recorded as `not run`, and M3 does not exit | Owner | GATE-TENFOLD-06, GATE-TENFOLD-07 | M2 | N41-15 (owner packet); N39b-29 |
| OPEN-TENFOLD-5 | The deadline H of each stratum | Three times the frozen rival's median development time, rounded up to half an hour, and raised until the horizon cap is at least 10 | The adjudicator | Decided at GATE-TENFOLD-04 from the development suite | M3 | N39b-30 |
| OPEN-TENFOLD-6 | The agent model, its revision and its caps | The agent the project itself is developed with at the gate's date, pinned; one agent only, with the result stated as conditional on it | Owner | Every freeze | M2 | N41-15 (owner packet); N39b-31 |
| OPEN-TENFOLD-7 | Incumbent licences: whether each permits publishing a comparison, and what the kit may carry | Revisions and fetch instructions only; an incumbent whose licence forbids publication is replaced by the next strongest, and the replacement is printed | Project lead | GATE-TENFOLD-01 | M0 | N39b-32 |
| OPEN-TENFOLD-8 | Ratification of the thresholds 3 and 10, the guard floor 5, the tie factor 1.5, the margin 0.05 and the limit of four confirmatory cells | As written | Owner | The first suite hash | M2 | N41-15 (owner packet); N39b-33 |
| OPEN-TENFOLD-9 | Whether arm D, the product's own layer over an incumbent engine, is built | Not built unless arm C ties on a stratum | Tether lead | Nothing | none | considerations of N39a-21 and N39b-07 (the arms epic is BD-430) |

Deadline values as records (the forms of N41-03, rule 3). Three rows stand for many small obligations and carry
`deadline = "per-row"`: OPEN-GAUNT-2 and OPEN-GAUNT-3, with `tracked_by` the known-answer registry's counts of
rows still printed `to set` and `none stated`, by the milestone of each row's owner section; OPEN-GAUNT-6, with
`tracked_by` the reconciliation register's flagged values, by the milestone of the gate that cites each. At
every exit the rows of these three that are due and still open are answered or named in a default ADR, like any
other question. OPEN-TENFOLD-9 blocks nothing and has `deadline = "none"`. OPEN-PROG-1 has `deadline = "M0"`,
and its second asking before the M7 pin freeze is recorded in `note` and enforced by N41-05's `require` check
(`cargo run -p frob-conformance --example open_questions -- require OPEN-PROG-1`); OPEN-PROG-2 has `deadline =
"M2"`.

Status at entry: twenty `open` rows and two aliases. Counting: this part completes the register at 161 questions
(147 in the subsystem table, including the six program-level rows above, plus OPEN-GAUNT-2 to -8 and
OPEN-TENFOLD-3 to -9; OPEN-TENFOLD-1 and -2 are counted with OPEN-PROG-2). The total is checked on the real file
by N41-04 once all eight parts exist; this part loads on its own under the plain load (rules 1 to 7 and 9 of
N41-03).

What the owner defaults cost (the record `frob.open/1` has no cost field: the cost is the `cost_of_default` of the
question's `[[owner_input]]` row in the owner calendar, N40-35, whose load check (e) requires a calendar row for
each of the ten owner rows here): OPEN-PROG-1, a private repository cannot be independently
confirmed; OPEN-PROG-2, the words "independently accepted" are not used for the result and the partial gate at M3
is a rehearsal; OPEN-PROG-3, external learning stacks have no supported route into the environment, and if the
adapter fails its gate the rival drives whole rollouts through the job interface; OPEN-PROG-4, the decisive early
experiment of the program is late and the lever "train few designs" stays a hypothesis; OPEN-PROG-5, none on the
critical path; OPEN-GAUNT-1, no Reality claim and the first-real-cell gate stays an import lane; OPEN-TENFOLD-4,
M3 does not exit, which makes this the one default in the register that blocks a milestone and the reason it is
asked two milestones early.

## Acceptance criteria

- [ ] `registries/open.toml` contains the twenty records and the two aliases; the aliases resolve to OPEN-PROG-2;
      the part loads under the plain load of N41-03 (rules 1 to 7 and 9) with no other part present.
- [ ] Each of the ten owner rows here (OPEN-PROG-1 to -5, OPEN-GAUNT-1, OPEN-TENFOLD-3, -4, -6, -8) has owner
      `Owner` and names its decision bead in this unit as `decision_bead`.
- [ ] OPEN-TENFOLD-4 is the default that blocks an exit: its `blocks` holds GATE-TENFOLD-06 and GATE-TENFOLD-07
      (the partial gate of M3 and the tournament), its deadline is M2 and its default leaves the gate `not run`.
- [ ] OPEN-GAUNT-2, OPEN-GAUNT-3 and OPEN-GAUNT-6 have `deadline = "per-row"` and a non-empty `tracked_by` naming
      the query that counts their open obligations (OPEN-GAUNT-2 and -3: the known-answer registry's rows printed
      `to set` and `none stated`).
- [ ] The checks that need every part or another registry are N41-04's (Scope), which depends on this bead, so
      none is an acceptance condition here: the 161 with `check --complete`, the owner's 32 questions resolving
      (K41-15), OPEN-GAUNT-2 and -3 reporting 133 and 325 from the known-answer registry, the ten-fold constants
      agreeing with the gate register, and the calendar listing OPEN-TENFOLD-4 at M1 and M2.

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part8.rs`, on this part's rows alone: `twenty_rows_two_aliases`;
`ids_match_expected_set`; `part_loads_alone` (the plain load on a scratch file holding only this part);
`per_row_rows_have_tracked_by` (OPEN-GAUNT-2, -3, -6); `aliases_resolve_to_prog_2`; `owner_rows_link_to_this_unit`
(PROG-1 to -5, GAUNT-1, TENFOLD-3, -4, -6, -8); `tenfold_4_blocks_m3_exit` (the row's `blocks`, deadline and
default as in the acceptance item). The tests `register_total_is_161`, `owner_table_questions_resolve`,
`gaunt_2_and_3_counts_come_from_ka_registry` and `thresholds_agree_with_gate_register` read all eight parts, the
known-answer registry and the gate register, and are N41-04's (`crates/frob-conformance/tests/open_register_complete.rs`).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04); the M3 walk asserts that with OPEN-TENFOLD-4 at its default
and no stratum of the partial gate recorded the exit is blocked and the message names the question
(`open-default-blocks-exit`), and that it clears when the gate has a record: either the strata that fit the
default class measured and the others `not run` with the reason (the rule of N41-15), or an approval followed
by the run. An approval with no gate record does not clear it.

### Known answers and negative controls
Known answers: twenty rows and two aliases here; 161 in total and 133 and 325 open rows of the known-answer suite
at the start, both asserted by N41-04. Negative controls: a
result worded "independently accepted" while OPEN-PROG-2 is at its default; the M3 exit attempted with
OPEN-TENFOLD-4 at its default; a known-answer row with tolerance `to set` counted as passing.

## Considerations

- OPEN-GAUNT-2 and -3 are not single decisions; they are 458 small obligations spread over every chapter. The
  register tracks them by count, and each chapter's known-answer registry bead carries its share.
- The owner's ten questions here are few and consequential; each has its own bead so that an answer, or a default
  taken, has one place to be recorded.

## References

§42.2 (questions for the owner), §42.3 (the program rows), §42.4 (what the defaults add up to), §38.16 (the
Gauntlet's questions), §39.19 (the ten-fold questions).
