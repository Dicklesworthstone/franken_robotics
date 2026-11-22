---
key: "NAB-30"
title: "M7 exit: sweep of the decision records against the tournament: Impact re-scored from measured phase ledgers"
type: "task"
priority: 2
milestone: "M7"
labels: ["m7", "delivery", "gauntlet", "tenfold", "solid", "registry", "decision-record"]
parent: "EPIC-M7"
deps: ["NAB-22", "NAB-29", "BD-434"]
needs: ["§39: the tournament record with its ledgers, recovery ratios and phase ablations (N40-16, N40-22, BD-432, BD-433)", "§2.4: the lever register with expected ranges by mission class, scored at M7 (N00-20)", "§4: the bet register and the rule that turns gate records into a bet verdict (N00-28)"]
related: ["NAB-17", "NAB-32", "NAB-42", "NAB-47", "NAB-31", "N00-20", "N00-28", "N40-16", "N40-17", "N40-21", "N40-22", "BD-428", "BD-429", "BD-430", "BD-431"]
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
records; the tie rule's consequences. Out: running or adjudicating the tournament; the exit review (BD-434);
the procedure (NAB-22).

## Technical approach

`decision-sweep skeleton --exit M7`, fill, `check --exit M7`, by a session that implemented none of the cited
beads and belongs to neither arm.

**1. No family is due by a kill gate.** The seven kill rules of the ten-fold test itself (KILL-TENFOLD-LEVER,
-SPEED, -TIE, -ADJ, -REACH, -CERT and -LEAK, each with its outcome written in advance) are decided per cell by
the tournament's own beads. The sweep reads their outcomes and decides none.

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

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m7_has_no_kill_gate_family`;
`m7_requires_impact_evaluation_for_every_adopted_family` (a record with 99 evaluations fails
`sweep.row-not-swept`); `impact_change_requires_ledger_reference`; `thin_layer_tie_lists_dependants`;
`ratio_outside_gate_record_is_refused` (the wording lint's finding is surfaced as a sweep finding).

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M7` (NAB-50): the due list (100 families for Impact, six thesis
records), `check`, counts of `stays` and `changed`. Logged per STD-E2E with the tournament's receipt ids; the
log quotes no sealed-suite content.

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
