---
key: "NAB-17"
title: "registries/ev_matrix.toml: the expected-value matrix (frob.evmatrix/1), score rule, table rules and lints"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["BD-002", "BD-005", "BD-025", "N40-03", "STD-TEST"]
needs: []
related: ["NAB-23", "NAB-24", "NAB-25", "NAB-26", "NAB-27", "NAB-28", "NAB-29", "NAB-30", "NAB-31", "NAB-42"]
gates: []
plan_refs: ["Appendix C (preamble)", "Appendix C.1", "§0.7", "§0.8", "§9 (D-6, D-10)"]
---
## Background

FrankenRobotics considered far more mechanisms than it builds. For every *family* of mechanism that was
considered, the planning record holds a score and a decision: adopted at a maturity tier (Solid [S], Frontier
[F], Moonshot [M]), admitted to a waiting queue, parked, deferred or rejected. The scores are judgments, written
down so that they can be argued with. The decisions are binding: a rejected family is not built, a parked one
is reopened only by the evidence its row names, and a Frontier or Moonshot family is funded only after its
paper-scale pilot has run and its kill gate (a trigger written in advance, with the fallback that replaces the
mechanism) is on file.

This bead turns that matrix into a registry, `registries/ev_matrix.toml`, with the rules that govern it and the
lints that keep it honest. The 143 rows are carried by NAB-18 (adopted, Solid: 41 rows), NAB-19 (adopted or
admitted, Frontier and Moonshot: 59 rows) and NAB-20 (parked, deferred or rejected: 43 rows). The Moonshot
queue is NAB-21. The rule and record for re-scoring at each milestone exit is NAB-22.

**The score.** Score = (Impact × Confidence) ÷ Effort, each an integer from 1 to 5, printed to one decimal.

- *Impact* counts the effect on the goal of the program, which is the time and compute to an independently
  accepted design decision, and, for evidence mechanisms, the effect on what may be trusted.
- *Confidence* is the judgment that the mechanism delivers that effect, taken after the evidence of the
  hypothesis register (the list of planning hypotheses with their states ACTIVE, NARROWED, REFUTED, DEFERRED).
- *Effort* is engineering cost, with 5 the largest.

**Labels that hold throughout the matrix and the killed-idea register.** A score is a DESIGN judgment. A figure
quoted from a publication, from sibling code or from a planning pilot is a FACT with an entry in the source
register, and it is re-read from its primary source, or re-run here, before a gate cites it. Arithmetic on
stated assumptions is "DESIGN arithmetic": a reader can redo it by hand, and it is not a measurement. A range
in a "survives" line, and a ceiling quoted in a decision, is a HYPOTHESIS about this system until the named
gate records a measurement, and every threshold is a TARGET. A number inside a "strong form" line is the
proposal's own figure: it is quoted, not endorsed. Nothing in the matrix was measured on FrankenRobotics code,
because none exists.

Design decisions (DESIGN): the matrix is data with a derived score, so that a re-score is a recorded change
with its evidence and not an edit of a table in prose; a row's decision is machine-readable, so that the
funding check can refuse a package whose family is parked or rejected. Rejected: keeping only the decisions and
dropping the scores, because the scores are what let a later reader see *how close* a rejected family was (a
family rejected at 3.0 for a stated reason is a different object from one rejected at 0.4). Tier: Solid.

## Scope

In: the file, its schema, the loader, the score function, the three table rules, the sort rule, the lints, the
generated printout. Out: the rows (NAB-18 to NAB-20); the Moonshot queue (NAB-21); re-scoring (NAB-22 and the
sweeps NAB-23 to NAB-31); the killed-idea register that rejected rows point into (NAB-32 onward).

## Technical approach

**1. File and schema.** `registries/ev_matrix.toml`, schema `frob.evmatrix/1`, JSON-Schema at
`schemas/registries/ev_matrix.schema.json`.

```toml
schema = "frob.evmatrix/1"

[[family]]
id          = "EV-F26"                    # EV-Snn adopted Solid; EV-Fnn adopted or admitted Frontier/Moonshot;
                                          # EV-Pnn parked, deferred or rejected. Never renumbered.
table       = "frontier-moonshot"         # solid | frontier-moonshot | parked
family      = "Derived kernels with dimension typing"
where       = ["C-14", "§21.6", "§21.7"]  # constructions, sections, gates named in the row's Where column
impact      = 4
confidence  = 3
effort      = 4
score       = "3.0"                       # stored as printed; must equal the derived value
decision    = "adopted"                   # adopted | admitted | parked | deferred | rejected | kept-as-summary |
                                          #   left-to-sibling
tiers       = ["F"]                       # the tiers the decision names, larger part first
decision_text = "Adopted [F] for correctness, derivatives and certificates; as throughput capped near 1.2; KILL-KIR"
kill_gates  = ["KILL-KIR"]
gates       = []
bets        = []
killed_idea = []                          # KI-nn records a rejection rests on
reason      = ""                          # rejected rows: why it is not built (the text after the colon of the
                                          #   decision, or the qualifier after the decision word: "under D-2")
reopen_on   = ""                          # parked and deferred rows: the evidence that reopens the question
needed_by   = ""                          # an adopted row below 2.0: what needs it
carried     = true                        # true when the row existed in the earlier version of the matrix
rescored_from = ""                        # the earlier printed score when a verdict changed it ("3.0")
queue       = []                          # the MQ-nn items of the Moonshot queue that wait for this row (EV-F45
                                          #   has two); filled from the queue records' `family` (NAB-21)

  [[family.history]]                      # appended by a re-score (NAB-22); never edited
  exit       = ""                         # the milestone exit that re-scored it ("M3")
  impact     = 0
  confidence = 0
  effort     = 0
  score      = ""
  decision   = ""
  reason     = ""
  evidence   = []                         # gate records, kill-gate ADRs, hypothesis-register changes, pilot ids
  session    = ""                         # the sweep session that wrote the entry (NAB-22 independence rule)
  unchanged  = false                      # true for "looked at and unchanged"; then `looked_at` is non-empty
  looked_at  = []                         # the evidence examined when nothing changed
  move       = ""                         # "frontier-moonshot → parked" when a fired kill gate moves the row
```

A history entry must carry non-empty `evidence`, or `unchanged = true` with non-empty `looked_at`; the loader
refuses any other entry (`ev.history-without-evidence`), which is what makes "unchanged" and "not swept"
different objects.

**2. The score function.** `score(i, c, e)` = the exact rational i·c/e rounded to one decimal, half away from
zero, formatted with one digit after the point. It is computed in integer arithmetic: `t = (20·i·c + e) / (2·e)`
(integer division), printed as `t/10` with one decimal. Examples that fix the rounding: 3·3/4 = 2.25 → "2.3";
5·3/4 = 3.75 → "3.8"; 5·4/3 = 6.67 → "6.7"; 4·4/3 = 5.33 → "5.3"; 2·2/3 = 1.33 → "1.3"; 3·1/5 = 0.6 → "0.6".

**3. The three table rules, as checks.**

1. *Below 2.0.* A family that scores below 2.0 is rejected, parked or deferred **unless something else needs
   it**. The one exception is a Moonshot: under the goal that reserves room for Moonshots (GOAL-43) an [M] item
   may be admitted below 2.0, behind a feature flag and off the critical path, and it then waits in the Moonshot
   queue until its pilot is funded. Check: a row of table `solid` or `frontier-moonshot` with score below 2.0
   must have `"M"` in `tiers`, or a non-empty `needed_by`.
2. *Carried rows.* A row carried from the earlier version keeps its scores unless a verdict of the hypothesis
   register changed them, and a changed score says so. Check: `carried = true` with scores different from the
   frozen plan-time copy requires `rescored_from` or a `history` entry whose `evidence` names a hypothesis or a
   gate record.
3. *Coverage.* Every construction C-16 to C-40 has a row, and so does every major mechanism that is not a
   construction. Where a construction has a Solid part and a Frontier part, the row sits in the table of its
   larger part and the decision names both. Check: each of C-16 … C-40 occurs in some row's `where` (at plan time
   all of C-01 … C-40 do); a row whose `tiers` has two entries sits in the table of the first.

**4. Sort rule.** Within each table the rows are sorted by score, highest first; ties keep the plan-time order.
The loader refuses a file whose table is out of order (`ev.unsorted`).

**5. Vocabulary of decisions.** "Adopted" means the work package is funded once the pilot has run. "Admitted"
means the item is in the catalogue and waits its turn under the Moonshot cap (at most four [M] items active at
once). A *parked* row may be reopened by the evidence named in `reopen_on`. A *rejected* row is not built;
where the rejection rests on a calculation, `killed_idea` names the record that holds it.

**6. Lints** (`crates/frob-conformance/tests/ev_matrix.rs`; `cargo test -p frob-conformance --test ev_matrix`).
Findings: `ev.score-mismatch`, `ev.unsorted`, `ev.below-two-unjustified`, `ev.carried-changed-silently`,
`ev.construction-missing`, `ev.kill-gate-unknown` (a kill gate not in the gate register), `ev.rejected-without-
reason` (a rejected row with neither `killed_idea` nor a non-empty `reason`; three rejected rows of the plan,
EV-P14, EV-P39 and EV-P40, have no colon in their decision text, which is why the reason is a field),
`ev.parked-without-reopen`, `ev.tier-table-mismatch`, `ev.queue-mismatch` (checked in both directions against
the queue records of NAB-21: an `admitted` row, or a row whose decision text says "queued", with an empty
`queue`; a row whose `queue` lists an item whose `family` does not name the row; a queue item whose `family`
names a row that does not list it, or a row that does not exist. A queue item with `family = []` has no row and
is not a finding), `ev.history-edited`, `ev.history-without-evidence`.

*Kill rows that land later (DESIGN).* The kill rows of the gate register are entered in seven ledger groups: two
at M0 (the foundations; Chronicle, Ground, Foundry and Tether) and five at M1 (the body; control, learning and
tasks; Augur, synthesis, Nemesis, Autopsy and Atelier; the ten-fold test; the late subsections). The matrix is an
M0 registry and names kill gates of all seven. The kill gates the three tables and the queue name (82 at plan
time) are therefore frozen, each with its ledger group and that group's milestone, in
`tests/fixtures/ev/kill_gates.tsv`. A name of the frozen list that is absent from the register is reported with
a recorded `skip` (the name and the group still to land) while the exit record of its group's milestone does
not exist; once that exit record exists it is `ev.kill-gate-unknown`. A name that is in neither the register nor
the frozen list is `ev.kill-gate-unknown` at once. This is how the row beads (NAB-18, NAB-19, NAB-21) meet the
criterion "resolves in the gate register" at M0; the M1 sweep (NAB-24) requires that no such skip remains.

**7. Generated printout.** `xtask::generate::ev_matrix` renders `docs/decisions/ev-matrix.md` (three tables and
the Moonshot queue) from the registry under `check-generated`. It is a row of the generator table, not a new
check.

**8. Readers.** The funding check (a work package whose family row is `parked`, `deferred` or `rejected` is
refused funding without an ADR that changes the row); the pilot ledger's Moonshot cap; the milestone-exit
sweeps (NAB-23 to NAB-31); the revive lint of the killed-idea register (NAB-32). Determinism class D0.

## Acceptance criteria

- [ ] `registries/ev_matrix.toml` validates and loads; with NAB-18 to NAB-20 landed it holds 143 `[[family]]`
      records: 41, 59 and 43 per table.
- [ ] `score` reproduces every printed score of the 143 rows, including "2.3" for (3, 3, 4) and "3.8" for
      (5, 3, 4).
- [ ] Rule 1 holds: no row of table `solid` is below 2.0; exactly 16 rows of table `frontier-moonshot` are
      below 2.0 and each has an [M] tier or a `needed_by`; 32 rows of table `parked` are below 2.0 and 11 are at
      2.0 or above with a stated reason.
- [ ] Rule 3 holds: C-01 to C-40 each occur in a row's `where`.
- [ ] Each of the twelve findings of step 6 is produced by its planted fixture.
- [ ] `docs/decisions/ev-matrix.md` is current under `check-generated`; `xtask` still has six subcommands.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ev_matrix.rs`:

- `score_examples`: (4,5,1) → "20.0"; (3,3,4) → "2.3"; (5,3,4) → "3.8"; (5,4,3) → "6.7"; (4,4,3) → "5.3";
  (3,2,4) → "1.5"; (2,2,3) → "1.3"; (2,2,5) → "0.8"; (3,1,5) → "0.6"; (2,1,5) → "0.4"; (1,4,3) → "1.3".
- `score_half_rounds_away_from_zero`: (3,3,4) is "2.3", not "2.2".
- `all_143_scores_match` (once the rows exist).
- `tables_sorted_descending`.
- `below_two_rule`: the 16 rows of the Frontier and Moonshot table below 2.0 are exactly: the tree-operator
  route (1.5, needed by hybrids above 64 degrees of freedom), skill transport (1.5), symmetry at the instant of
  contact (1.3), reachable-set tables (1.3), kernel reflection (1.0, needed as the one route from an instance
  certificate to class Proved), the geometric-algebra tree policy, numerical continuation, Study-quadric
  synthesis, tactile hands, firmware export, lifecycle compression, the shape-space atlas, label minimization
  from contact, the fused per-world step kernel (all 1.0), coupled multiphysics islands (0.8) and facility
  co-design (0.6).
- `constructions_1_to_40_covered`.
- `planted_findings`: one fixture per finding of step 6.
- `rescored_row_says_so`: the connection-curvature row carries `rescored_from = "3.0"` and prints 2.0.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) loads the matrix, recomputes every score, checks the rules and
regenerates the printout. Logged per STD-E2E: row counts per table, the number of rows below 2.0 per table,
mismatches (expected 0), each as a `metric` with its expected value; digests of the registry and the printout.

### Known answers and negative controls

Known answers: 143 rows (41 + 59 + 43); 0, 16 and 32 rows below 2.0; the highest score is 20.0 (five rows) and
the lowest is 0.4 (continuation on the GPU with root certificates checked inside a proof kernel). Negative
controls: a row whose printed score is "2.2" for (3,3,4); a Solid row at 1.5 with no `needed_by`; a carried row
whose confidence was changed with no history; a rejected row with an empty reason; a kill gate name that does
not exist; a history entry with empty `evidence` and `unchanged = false` (`ev.history-without-evidence`). Metamorphic relation: permuting rows of equal score within a table changes no lint result except
the frozen-order comparison.

## Considerations

- Scores are judgments. The lints check arithmetic, ordering and the three rules; they do not and cannot check
  that an Impact of 4 is right. The record of *why* a score changed (NAB-22) is what makes the judgments
  arguable.
- The printout is for the sceptical reader who asks which ideas were killed and how close they were.
- If nothing reads a row's decision by M3 (no funding check consults it), the registry-must-have-a-reader rule
  applies to this file as to any other, and the matrix is reduced to the printout.

## References

Appendix C preamble and C.1; §0.7 (killed ideas stay on the page; process is never the product); §0.8;
doctrine rules D-6 (Moonshot cap) and D-10 (pilot and ceiling before funding); GOAL-43.
