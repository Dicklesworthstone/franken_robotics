---
key: "NAB-30"
title: "M7 exit: sweep of the decision records against the tournament: Impact re-scored from measured phase ledgers"
type: "task"
priority: 2
milestone: "M7"
labels: ["m7", "delivery", "gauntlet", "tenfold", "solid", "exit-review", "decision-record"]
parent: "EPIC-M7"
deps: ["BET-13", "BET-14", "N00-20", "N00-28", "N40-04", "N40-16", "N40-20", "N40-22", "N40-28", "NAB-22", "NAB-27", "NAB-50", "STD-E2E"]
needs: []
related: ["BD-434", "NAB-28", "NAB-29", "NAB-32", "NAB-39", "NAB-42", "NAB-47"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2 (entries on the thesis and on claims)", "Appendix B.5", "§39", "§43 (M7)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the sweep
at the exit of **M7**: the ten-fold tournament, where the thesis of the program is measured against the
strongest composed rival and published whatever it says.

The ten-fold test compares the *restricted mean time* to an independently accepted answer, RMT = E[min(T, H)]
at a common deadline H (an unresolved mission costs H), between this system and the strongest rival that a
competent engineer can compose from leading simulators, an optimizer, analytic bounds, a verifier and the same
agent, on a suite of missions sealed before either arm saw them. It has three arms (this system; the composed
rival; a short robust program over an incumbent engine), reports how much of the gain the thin-layer arm
recovers, and keeps a phase ledger on every arm: the measured breakdown of wall time by phase, device and agent
turn.

No mechanism's kill gate is decided at M7: the tournament decides the thesis, not a mechanism. The sweep is
different in kind. **Impact**, the first factor of every score in the matrix, was defined as "the effect on the
time and compute to an independently accepted design decision". Until M7 every Impact is a judgment about a
loop nobody had timed end to end on a sealed suite. After M7 there is a measured ledger. This sweep re-reads
the Impact of every adopted family against it.

## Scope

In: the M7 sweep record and report; an Impact evaluation for every adopted family; the states of the thesis
records; the tie rule's consequences. Out: running or adjudicating the tournament; the exit review (BD-434),
which comes after this sweep (NAB-22 design decision 1: the sweep is made at the exit candidate commit, before
the exit record), runs its `check` as the attached check `decision-sweep` and copies its section into the exit
record; the procedure (NAB-22).

**Inputs (each a dependency).** The sweep needs the tournament's records, not the exit verdict: the published
report and its ledgers (GATE-TENFOLD-07, N40-16), read through the gate report at the exit candidate commit
(N40-04); the recovery ratios and phase ablations (GATE-TENFOLD-13, N40-22); the rival audit (GATE-TENFOLD-11,
N40-20); the outcomes of the ten-fold kill gates against their fallback ledger (N40-28); the verdicts of BET-13
and BET-14 (their decision beads); the lever register (N00-20) and the bet register (N00-28). The sweep before
it is the M4 sweep (NAB-27): M7 needs M4 and neither M5 nor M6, so the M5 and M6 sweeps (NAB-28, NAB-29) are not
prerequisites; the sweep reads the decision records as they stand when it runs, and if either of those sweeps
was recorded first, its entries are part of that state and are cited, never repeated.

## Technical approach

`decision-sweep skeleton --exit M7`, fill, `check --exit M7`, by a session that implemented none of the cited
beads and belongs to neither arm.

**1. No family is due by a kill gate.** Of the seven kill rules of the ten-fold test itself (each with its
outcome written in advance, N40-28), KILL-TENFOLD-LEVER was decided at M1 on the rival's ledger and at M2 on
ours; the other six (-SPEED, -TIE, -ADJ, -REACH, -CERT and -LEAK) are decided per cell by the tournament's own
beads. The sweep reads their outcomes and decides none.

**2. Impact, family by family.** For each family in tables `solid` and `frontier-moonshot` (100 rows):

1. Name the phase of the design loop the family acts on (authoring, checking, controller acquisition,
   training, evaluation, diagnosis, agent turns, adjudication) and the lever it belongs to in the lever
   register.
2. Read that phase's share s on the rival's arm and on ours, from the phase ledgers, per mission stratum.
3. Record the ceiling of removing the phase entirely, 1/(1 − s), and the measured contribution from the three
   single-change ablations where the family is one of the ablated levers.
4. Judge: Impact stays, or changes with a two-line reason that cites the ledger. A family that is an evidence
   mechanism (it changes what may be trusted, not how long a decision takes) is judged on whether the
   adjudicator relied on its output.

Families adopted for reasons the thesis does not count (structure and manufacture; calibration against
reality) are recorded `not counted in the thesis` and keep their scores.

**3. The thesis records of the killed-idea register.**

| Record | What the tournament shows | What the sweep records |
|---|---|---|
| KI-59 engine speed alone, and information alone | The measured step share and training share on both arms; which regime each mission stratum is in (training-dominated, or agent-turn-dominated once controller acquisition is amortized) | The measured shares beside the plan-time arithmetic (an infinitely fast step gives at most 1/(1 − s): 7.7 at 87%, 1.4 at 29%); the hypothesis's state after the measurement |
| KI-60 headline multipliers | The pooled ratio of restricted mean times with its interval, per cell | The only multiplier ever printed, with its ceiling, its strata and its `not run` cells |
| KI-56 a family query as a capability; the tie rule | The thin-layer arm's recovery ratio | If a short robust program over an incumbent engine ties, the records say so and the layer is what ships; every family whose adoption assumed otherwise is listed |
| KI-58 generative structure as a decision-time lever | Whether any accepted answer's time depended on structure | Stays outside the thesis unless the ledger shows otherwise |
| KI-44 a speed test with no power | The tournament's pre-analysis: reachable verdicts and power against a true ratio of 10 (the draft rule had about 18% power) | That the published rule could have returned each of its verdicts |
| KI-61, KI-62 withdrawn differentiators; corrected targets | The rival audit completed before sealing, each arm within 10% of its recorded figure | The claims registry's state after the tournament |

**4. Contested ideas.** The contested question "where the ten-fold comes from" is settled or stays open with
numbers: the record states the measured share of the gain attributable to engine throughput, to avoided
trainings, to family answers and to fewer agent rounds, each with its interval, and states plainly if no
conjunction reached ten.

**5. Bets and hypotheses.** The bet register's verdicts for the bets that the tournament decides (the ten-fold
bet and the thin-layer tie bet) and the hypothesis register's two thesis rows are read; any family whose
decision text cites one of them gets a history entry.

**6. Queue, ideas, sources.** The Moonshot queue is unchanged by the tournament unless a kill rule of the test
removes a lever that an item depends on. Idea records "ten-fold defined as the cost of an accepted design
decision" and "rival audit before any ratio" take their final state. Every published figure quoted in the
tournament's report resolves to a source record with a primary reading.

## Acceptance criteria

- [ ] `docs/delivery/exits/M7/decision-records.json` exists and `decision-sweep check --exit M7` passes.
- [ ] Each of the 100 adopted or admitted families has an Impact evaluation: phase, measured share per arm, the
      ceiling 1/(1 − s), and `stays` or `changed` with its reason; families outside the thesis are marked so.
- [ ] Each row of step 3 has its recorded state with the tournament's receipt ids.
- [ ] If the thin-layer arm tied, the sweep lists every family and idea record whose text assumed it would not,
      and each has a history entry.
- [ ] No ratio appears in any decision record except as a gate's measured value with its interval and ceiling.
- [ ] The sweep's session belongs to neither arm and implemented no bead it cites.
- [ ] The sweep record exists at the exit candidate commit before the M7 exit record is written, and the exit
      runner's attached check reads it.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m7_has_no_kill_gate_family`;
`m7_requires_impact_evaluation_for_every_adopted_family` (a record with 99 evaluations fails
`sweep.row-not-swept`); `impact_change_requires_ledger_reference`; `thin_layer_tie_lists_dependants`;
`ratio_outside_gate_record_is_refused` (the wording lint's finding is surfaced as a sweep finding).

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M7` (NAB-50, a dependency): `cargo run -p frob-conformance --bin
decision-sweep -- due --exit M7` (the due list: 100 families for Impact, six thesis records, asserted against the
counts above), then `decision-sweep check --exit M7` on the filled record, then `FROB_TEST_SEED=0 cargo test -p
frob-conformance --test decision_sweep m7`; the three negative controls below are planted on scratch copies of
the record (fixtures under `tests/fixtures/sweep/M7/controls/` written by
this bead, the hook of NAB-50), each asserted by its finding code. Logged through the shared helper of STD-E2E to
`target/e2e-logs/decision_records/<utc>.ndjson` with a transcript (`ts`, `script`, `bead` (`NAB-30`), `step`,
`event`, the exact command line, exit code, wall time): counts of `stays` and `changed` per table, each Impact change with the
ledger receipt it cites, the thesis records' states, and the tournament's receipt ids; `assert` lines of kind
`negative-control` with `caught`; on failure the first family without an Impact evaluation, with the path of the
sweep record; a `summary` with assertions passed and failed. The log quotes no sealed-suite content.

### Known answers and negative controls

Known answers: 100 families in the two adopted tables (41 + 59); ceilings 1/(1 − 0.87) = 7.69 and
1/(1 − 0.29) = 1.41; with one design per agent turn and 300 s of training the loop ratio is at most
(300 + t)/t (3.5 at t = 120 s, 6 at t = 60 s). Negative controls: an Impact raised for a family whose phase is
under 5% of our arm's time with a reason that cites no ledger; a sweep performed by a session of one arm
(`sweep.self-certified`); a decision record that prints "ten-fold" as a result when the gate recorded a
different value.

## Considerations

- The tournament may show no ten-fold. The decision records are then the most valuable artifact of the program:
  they say which judgments were wrong, by how much, and on what evidence they were made.
- Impact re-scoring changes the printed order of the tables. The printout keeps the plan-time score beside the
  M7 score so the change is visible.
- The sweep quotes shares and ratios only from published tournament records; it never opens the sealed suite.

## References

Appendix C.1; Appendix C.2 (entries on the thesis, claims and wording); Appendix B.5; §39; the M7 milestone and
its exit review.
