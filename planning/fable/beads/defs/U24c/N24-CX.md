---
key: "N24-CX"
title: "registry: Arena counterexamples CX-259 to CX-269 and the unnumbered ones of the late subsections"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "arena", "gauntlet", "frob-arena", "registry", "test", "solid"]
parent: "WP-41"
deps: ["STD-TEST"]
needs: []
related: ["N24c-10", "N24c-08", "N24-REF", "BD-146", "BD-025", "N24c-17", "N24c-22", "N24c-31", "N24c-36", "N24c-27"]
gates: []
plan_refs: ["Appendix N (N.21)", "§24.5", "§24.8", "§24.10", "§24.11", "§24.12", "§24.13", "§24.19", "§24.20", "§24.21", "§24.22", "§24.23", "§0.7"]
---
## Background

Counterexamples are first-class in this programme: where a plausible shortcut is wrong, the plan gives the
smallest hand-checkable case that breaks it, and the case enters the negative-control registry and the
counterexample catalogue. A rule that ships without such a case has not been thought through. A counterexample is
used twice: as a **known answer** (the correct mechanism must reproduce the case's numbers) and as a **negative
control** (a mutant that implements the tempting shortcut must be caught by a named test).

The catalogue lists eleven counterexamples for the Arena chapter (CX-259 to CX-269). The five subsections added
late to the chapter (the wrench slice, the task audit, the item bank, exposure domains, the idle arm) state
further counterexamples in their own text that have no catalogue number yet. This bead is the chapter's
counterexample registry: every case with its numbers, the rule it protects, the mutant that realises the
shortcut, and the test that must catch it. Mechanism beads cite these and restate the values they need.

## Scope

In: the registry entries for the eleven numbered counterexamples and the eight unnumbered ones; for each, the
mutant and the catching test; a check that every entry has both.
Out: the mechanisms and their tests (named per row); the chapter's known-answer registry (its own bead).

## Technical approach

**Numbered counterexamples.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Mutant (negative control) | Caught by |
|---|---|---|---|---|---|
| CX-259 | Compose raw requirement margins with a minimum | Clearance 0.02 m and thermal margin 3 K: the raw minimum is 0.02 (clearance binds); in millimetres it is 3 (thermal binds). With scales 0.10 m and 10 K the margins are 0.2 and 0.3 in either system | Unit-normalised predicates; the unit-scaling metamorphic relation | An evaluator that takes the minimum before dividing by scales | Verdict and evaluator tests (`binding_atom_unit_invariant`); known-answer row KA-594 |
| CX-260 | Multiply stage pass rates to get the mission rate | Twenty equally likely worlds, two stages each passing in 19: 18 of 20 if the failures differ, 19 of 20 if they coincide; 0.95² = 0.9025 is neither | Stage pass rates are reported jointly | A report that prints the product of stage rates | `joint_stage_counts`; KA-595 |
| CX-261 | Report energy per completed trial only | Design A with one 10 J success and one 20 J failure against design B with two 12 J successes: 10 J against 12 J conditional on completion, 30 J against 12 J per completion; the two figures order the designs oppositely | Both energy figures are published | A scorecard that drops failed attempts from the total | `energy_two_figures`; KA-598 |
| CX-262 | Integrate the current, then the wheel speed, explicitly | The wheel-loop row has spectral radius 0.9207 with the specified feedforward R·i; at a 0.5 ms step explicit Euler that updates the current before the wheel speed gives 1.497 (and 1.854 for the R·i_ref case of 1.4820), turning the stable case unstable | Actuator stage integration; the fixture-1 gate | An actuator stage using explicit current-then-wheel Euler | The cart fixture's wheel-loop row; KA-610 |
| CX-263 | A static screen can settle the sill question | In the creep limit design A docks in about one condition in ninety and B in fewer; in a planar pilot of 156 rows, 62 passed the dynamic scorer and failed the quasi-static screen, and none passed the static screen alone | A static rejection is a candidate for a dynamic test; momentum, impact and drive transients are in scope | A pipeline that discards statically rejected conditions before the batch | The cart fixture's batch accounting (every condition runs for both designs) |
| CX-264 | A comparison may leave rotor inertia and controller rate unpinned | In an exploratory planar sweep the two designs' success fractions were 0.093 and 0.780 at 100 Hz with zero rotor inertia and 1.000 and 1.000 at 1 kHz or with added inertia: the comparison measured loop stability | Mechanism controls of fixture 1; the pinned controller arm of a study | A task file with no declared rotor inertia or control cadence | Fixture-hash test (both are constants of the fixture); the study's controller-arm declaration |
| CX-265 | A fixed-n study can return any of its verdicts | At n = 64 the rule of fixture 1 has 521, 521, 0 and 1,103 outcomes: no outcome is a practical tie, since perfect concordance gives [−0.0662, +0.0662]; a tie is first reachable at n = 86, for (0, 0) only | The reachability pre-analysis; FROB-REF-532 | A planner that admits a tie verdict at n = 64 | `po_arena_03_preanalysis_exact`; KA-612 |
| CX-266 | Look at a fixed-n test after every pair and stop when it rejects | An exact one-sided sign test at nominal 5% on null pairs: 13/256 = 5.08% after 8 looks, 8.76% after 16, 12.9% after 32, 17.4% after 64 | Study types are never mixed (INV-ARENA-05); FROB-REF-552 | A fixed-n study that accepts an early stop on its running p-value | `inv_arena_05_study_type_fixed`; KA-618 |
| CX-267 | Resubmit candidates to the same held-out scenes until one passes | Independent attempts at 5% each: fourteen null submissions give a 51% chance of at least one false promotion | Level α_i = α/[i·(i + 1)] on fresh sealed clusters (INV-QUAL-02, -03); the 500-retry gate | A lane with reused clusters at a constant level (false promotion in more than 99% of campaigns under 500 adaptive retries) | `inv_qual_02_clusters_fresh`, `inv_qual_03_level_ledger_append_only`; KA-620, KA-622 |
| CX-268 | More contact area means more traction | Limiting model on a split-grip boundary: at z = +12 mm the 40 mm sole has capacity 0.690 against 0.619 for the 70 mm sole; at z = −12 mm it has 0.360 against 0.431. The ranking reverses with the sign of the offset | Fixture 2's decision rule; fidelity-dependent rankings are reported as such | A capacity model proportional to contact area | Fixture 2's plate-drag rows; KA-616 |
| CX-269 | Concatenate separately recorded stage episodes to show a mission | The end state of one stage need not lie in the entry set of the next (the chapter gives no numeric case) | Missions are single episodes with until-clauses | A verdict assembled from two episodes | `stage_concatenation_refused` |

**Counterexamples stated in the late subsections (no catalogue number yet).** Local labels are for this
registry only; the catalogue owner assigns CX numbers at implementation.

| Local label | The tempting shortcut | The smallest case that breaks it | The rule it protects | Mutant | Caught by |
|---|---|---|---|---|---|
| ARENA-W1 | Rank contact geometries by scalar friction capacity Σ μᵢ·Nᵢ | A two-point sole with 50 N per point at lateral offsets ±20 mm on μ = 0.25 and μ = 0.8: 52.5 N forward with an unavoidable yaw moment of 0.55 N·m; with zero yaw only 25 N; a uniform μ = 0.4 sole gives 40 N. The split sole wins by scalar capacity (52.5 against 40) and loses at the commanded wrench (25 against 40) | Capacity is a slice of the wrench set at the commanded wrench; scorecards name `capacity, moment free` | A traction program without the moment row | `two_point_zero_yaw`; the wrench-slice gate |
| ARENA-W2 | The zero-yaw capacity of a sole on a friction boundary is the average of the two coefficients | Uniform-pressure strip with the boundary through the centre, μ_L = 0.25, μ_H = 0.8: (μ_L + √(μ_L·μ_H))/2 = 0.3486 with zero yaw, against 0.525 with the moment free, independent of the sole width | The same | A closed form using (μ_L + μ_H)/2 for the free plate | `strip_zero_yaw_lower_bound`; the free-yaw plate rig |
| ARENA-A1 | A requirement that says "eventually in the goal" means the robot arrives | Goal region x ∈ [4.9, 5.1] m, requirement `(F (0 s 10 s) in_goal)`: a sled at a constant 1 m/s is inside for t ∈ [4.9, 5.1] s and never stops; satisfied with a dwell of 0.2 s. With `(F (0 s 10 s) (H (0 s 2 s) in_goal))` the sled violates and a body at rest at x = 5 m from t = 6 s satisfies from t = 8 s | Intent invariants (dwell); the task audit before any campaign | A task with no dwell | `hand_case_sled`; the audit gate |
| ARENA-A2 | A requirement built from safety atoms is a mission | A delivery task rewarding `(H (0 s 2 s) (and clear-margin thermal-ok spill-ok))` over 60 s: a parked robot collects the full reward; a robot that carries the tray and loses its spill margin for 1 s collects less. A progress atom (approach speed above 0.15 m/s on a scale of 0.50 m/s) gives the parked robot a margin of −0.3 | The idle arm; `rewards-idleness` findings | An audit that runs no idle law | `hand_case_parked_collects_full_reward`; the idle-arm gate |
| ARENA-I1 | A good item model makes importance sampling safe by itself | The model places q* on items disjoint from the discriminating ones: each discriminating item has q = ε·p and weight 10; second moment f × 10 = 1.0; variance 0.9975 against 0.0975 under uniform sampling, 10.2 times worse, still unbiased. With no mixture (ε = 0) those items are never drawn and the estimate is biased | The defensive mixture; the gate measures realised width, not the model's forecast | ε = 0; a planner that reports forecast variance | `wrong_model_unbiased`; the item-bank known-answer gate |
| ARENA-I2 | Importance weights from two estimators may be multiplied | Item-bank weights are bounded by 1/ε = 10 and tier-correction weights by 20; composed, 10 × 20 = 200 | Item-bank sampling is not composed with the tier-correction estimator | A promotion that accepts item-bank episodes | `refuse_with_tier_correction` |
| ARENA-E1 | Key a timed disturbance by control tick | A push keyed by control tick 1,500 occurs at 3.0 s with a 2 ms step and at 1.5 s with a 1 ms step | `process` draws are keyed by physical time; the rate-invariance check | A tick-keyed adapter | `rate_detects_tick_key`; the coupling fixture |
| ARENA-E2 | Key a disturbance by event order | A push keyed by "the tenth footfall" occurs at 5.0 s under a gait period of 0.5 s and at 4.0 s under 0.4 s: a disturbance meant to be timed follows gait phase, and the two arms were pushed in different states | Keying by the order of contacts, footfalls or draws is refused (FROB-REF-556); the retiming check | A footfall-keyed adapter | `retiming_detects_footfall_key`; the coupling fixture |

**Related counterexamples catalogued under other chapters and used by Arena tests.** CX-384 (letting the
qualification receipt say which worlds failed would let an agent search near those worlds, patch and resubmit,
so the level schedule would bound an error rate for a procedure no longer being run; the receipt carries only
decision, index, level, units and a hash); CX-046 and CX-213 (a strict requirement evaluated with a non-strict
comparator: at U = 3/4 and ε = 5/16 both docking worlds can report 15/16 while needing −1/4 and −3/4); CX-123
(a strict inequality decided in plain single precision); CX-144 (a rejected step that redraws its random input:
the accepted sign is positive with probability 3/4).

**Registration.** Each row becomes an entry of the negative-control registry (`frob-conformance`): identifier,
shortcut, case, protected rule, the mutant's switch (a test-only feature flag or a planted fixture), and the test
that must fail when the mutant is on. An entry with no catching test fails the registry's own check.

## Acceptance criteria

- [ ] All eleven numbered counterexamples and the eight local ones are registered with case numbers, mutant switch and catching test.
- [ ] For every entry whose mechanism exists at the current milestone, enabling the mutant makes the named test fail and disabling it makes the test pass; entries whose mechanism arrives later are registered `not run` with the milestone.
- [ ] The case numbers in the registry equal those printed here (values are not improved in transit).
- [ ] The local labels are reported to the catalogue owner for numbering; no CX number is invented or reused.
- [ ] CX-269 is flagged as having no numeric case, with the refusal test as its only control.

## Test plan

### Unit tests
`cargo test -p frob-conformance arena_counterexamples`: `registry_has_19_arena_entries`;
`every_entry_has_mutant_and_test`; `case_numbers_match` (the values above are recomputed where they follow from
a formula: 0.95² = 0.9025; 12.5 + 40 = 52.5 and (40 − 12.5) × 0.020 = 0.55; (0.25 + √0.2)/2 = 0.3486;
1 − 0.95¹⁴ = 0.512; 13/256 = 0.0508; tick 1,500 × 2 ms = 3.0 s and × 1 ms = 1.5 s; ten footfalls × 0.5 s = 5.0 s
and × 0.4 s = 4.0 s; (0 − 0.15)/0.50 = −0.3; 0.1 × 10 − 0.0025 = 0.9975); `not_run_entries_have_milestones`.

### End-to-end script and logging
No script of its own. The conformance harness's mutation switch is driven by the capability scripts (the
fixtures battery, the wrench-slice, audit, item-bank and coupling scripts), each of which logs for every control
it runs: the entry's identifier, the mutant switch, the test run, and `control_caught: true|false`.

### Known answers and negative controls
The table is the content. Determinism: every case here is exact or hand-checkable arithmetic and must reproduce
identically on every platform.

## Considerations

- Two rows rest on facts recorded from exploratory pilots (CX-263, CX-264); the pilots kept their own law and scorer, so those rows protect a rule and are not product measurements.
- A counterexample proves a shortcut wrong on one case; it does not bound how wrong. The gates measure the mechanisms; these entries keep the shortcuts from returning unnoticed.
- The catalogue marks CX-268 as identical to a counterexample catalogued under another chapter; the registry stores one mutant for both identifiers.

## References

Appendix N (N.21: CX-259 to CX-269), §24.5, §24.8, §24.10 to §24.13, §24.19 to §24.23, §0.7; KA-594, KA-595, KA-598, KA-610, KA-612, KA-616, KA-618, KA-620, KA-622.
