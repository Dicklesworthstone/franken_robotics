---
key: "N24-CX"
title: "registry: Arena counterexamples CX-259 to CX-269 and the unnumbered ones of the late subsections"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "arena", "gauntlet", "frob-arena", "registry", "test", "solid"]
parent: "WP-41"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-TEST"]
needs: []
related: ["N24-KA", "N24-REF", "N24c-08", "N24c-10", "N24c-17", "N24c-22", "N24c-27", "N24c-31", "N24c-36", "N38b-05"]
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

In: the eleven numbered counterexamples and the eight unnumbered ones as entries of the counterexample catalogue
`crates/frob-conformance/registry/counterexamples.toml`, in N38b-05's record schema, each with its kind of control,
its plant and the bead that builds it; the block request for the eight late numbers; a host test that recomputes
the numbers and checks every entry.
Out, with the bead that has it: the catalogue's schema and loader, the seeding tool that turns entries into rows of
`controls.toml` and the join (N38b-05; this bead writes no row of `controls.toml`); the control record format
(BD-025); running controls and recording that they were caught (N38b-04); the allocation ledger (N38a-40); the
mechanisms, their tests and the plants (the beads named per row); the chapter's known-answer registry (N24-KA).

## Technical approach

**Numbered counterexamples.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Mutant (negative control) | Caught by |
|---|---|---|---|---|---|
| CX-259 | Compose raw requirement margins with a minimum | Clearance 0.02 m and thermal margin 3 K: the raw minimum is 0.02 (clearance binds); in millimetres it is 3 (thermal binds). With scales 0.10 m and 10 K the margins are 0.2 and 0.3 in either system | Unit-normalised predicates (C-30); the unit-scaling metamorphic relation of §38.7 | An evaluator that takes the minimum before dividing by scales | Verdict and evaluator tests (`binding_atom_unit_invariant`); known-answer row KA-594 |
| CX-260 | Multiply stage pass rates to get the mission rate | Twenty equally likely worlds, two stages each passing in 19: 18 of 20 if the failures differ, 19 of 20 if they coincide; 0.95² = 0.9025 is neither | Stage pass rates are reported jointly | A report that prints the product of stage rates | `joint_stage_counts`; KA-595 |
| CX-261 | Report energy per completed trial only | Design A with one 10 J success and one 20 J failure against design B with two 12 J successes: 10 J against 12 J conditional on completion, 30 J against 12 J per completion; the two figures order the designs oppositely | Both energy figures are published | A scorecard that drops failed attempts from the total | `energy_two_figures`; KA-598 |
| CX-262 | Integrate the current, then the wheel speed, explicitly | The wheel-loop row has spectral radius 0.9207 with the specified feedforward R·i; at a 0.5 ms step explicit Euler that updates the current before the wheel speed gives 1.497 (and 1.854 for the R·i_ref case of 1.4820), turning the stable case unstable | Actuator stage integration; GATE-ARENA-04, the fixture-1 gate | An actuator stage using explicit current-then-wheel Euler | The cart fixture's wheel-loop row; KA-610 |
| CX-263 | A static screen can settle the sill question | In the creep limit design A docks in about one condition in ninety and B in fewer; in a planar pilot of 156 rows, 62 passed the dynamic scorer and failed the quasi-static screen, and none passed the static screen alone | A static rejection is a candidate for a dynamic test; momentum, impact and drive transients are in scope | A pipeline that discards statically rejected conditions before the batch | The cart fixture's batch accounting (every condition runs for both designs) |
| CX-264 | A comparison may leave rotor inertia and controller rate unpinned | In an exploratory planar sweep the two designs' success fractions were 0.093 and 0.780 at 100 Hz with zero rotor inertia and 1.000 and 1.000 at 1 kHz or with added inertia: the comparison measured loop stability | Mechanism controls of fixture 1; the pinned controller arm of a study | A task file with no declared rotor inertia or control cadence | Fixture-hash test (both are constants of the fixture); the study's controller-arm declaration |
| CX-265 | A fixed-n study can return any of its verdicts | At n = 64 the rule of fixture 1 has 521, 521, 0 and 1,103 outcomes: no outcome is a practical tie, since perfect concordance gives [−0.0662, +0.0662]; a tie is first reachable at n = 86, for (0, 0) only | The reachability pre-analysis (`eval.preanalyze`); FROB-REF-532 | A planner that admits a tie verdict at n = 64 | `po_arena_03_preanalysis_exact`; KA-612 |
| CX-266 | Look at a fixed-n test after every pair and stop when it rejects | An exact one-sided sign test at nominal 5% on null pairs: 13/256 = 5.08% after 8 looks, 8.76% after 16, 12.9% after 32, 17.4% after 64 | Study types are never mixed (INV-ARENA-05); FROB-REF-552 | A fixed-n study that accepts an early stop on its running p-value | `inv_arena_05_study_type_fixed`; KA-618 |
| CX-267 | Resubmit candidates to the same held-out scenes until one passes | Independent attempts at 5% each: fourteen null submissions give a 51% chance of at least one false promotion | Level α_i = α/[i·(i + 1)] on fresh sealed clusters (INV-QUAL-02, -03); GATE-QUAL-01, the 500-retry gate | A lane with reused clusters at a constant level (false promotion in more than 99% of campaigns under 500 adaptive retries) | `inv_qual_02_clusters_fresh`, `inv_qual_03_level_ledger_append_only`; KA-620, KA-622 |
| CX-268 | More contact area means more traction | Limiting model on a split-grip boundary: at z = +12 mm the 40 mm sole has capacity 0.690 against 0.619 for the 70 mm sole; at z = −12 mm it has 0.360 against 0.431. The ranking reverses with the sign of the offset | Fixture 2's decision rule; fidelity-dependent rankings are reported as such | A capacity model proportional to contact area | Fixture 2's plate-drag rows; KA-616 |
| CX-269 | Concatenate separately recorded stage episodes to show a mission | The end state of one stage need not lie in the entry set of the next (the chapter gives no numeric case) | Missions are single episodes with until-clauses | A verdict assembled from two episodes | `stage_concatenation_refused` |

**Counterexamples stated in the late subsections (no catalogue number yet).** Numbers for them come from the
allocation ledger `crates/frob-conformance/registry/allocations.toml` (N38a-40), never from "the next free
number" at commit time: this bead commits one request `[[block]] registry = "cx", count = 8, owner = "N24-CX", section = "§24.19 to §24.23"`,
cites the rows by the local labels below until the block is issued, then maps the labels to the block's numbers
in table order (ARENA-W1 first, ARENA-E2 last) in one commit and keeps each label in the entry's `slug`. A label
still used outside `slug` at this bead's milestone exit fails the ledger's `provisional-tag-past-due` check.

| Local label | The tempting shortcut | The smallest case that breaks it | The rule it protects | Mutant | Caught by |
|---|---|---|---|---|---|
| ARENA-W1 | Rank contact geometries by scalar friction capacity Σ μᵢ·Nᵢ | A two-point sole with 50 N per point at lateral offsets ±20 mm on μ = 0.25 and μ = 0.8: 52.5 N forward with an unavoidable yaw moment of 0.55 N·m; with zero yaw only 25 N; a uniform μ = 0.4 sole gives 40 N. The split sole wins by scalar capacity (52.5 against 40) and loses at the commanded wrench (25 against 40) | Capacity is a slice of the wrench set at the commanded wrench; scorecards name `capacity, moment free` | A traction program without the moment row | `two_point_zero_yaw`; the wrench-slice gate |
| ARENA-W2 | The zero-yaw capacity of a sole on a friction boundary is the average of the two coefficients | Uniform-pressure strip with the boundary through the centre, μ_L = 0.25, μ_H = 0.8: (μ_L + √(μ_L·μ_H))/2 = 0.3486 with zero yaw when longitudinal traction never points backward (0.3731 when it may, N24c-17; higher still with lateral traction), against 0.525 with the moment free, independent of the sole width | The same | A closed form using (μ_L + μ_H)/2 for the free plate | `strip_zero_yaw_lower_bound`, `strip_zero_yaw_signed_longitudinal`; the free-yaw plate rig |
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

**Registration (the catalogue record of N38b-05).** Each row is one entry of the suite's one counterexample
catalogue, `crates/frob-conformance/registry/counterexamples.toml`, with the fields of N38b-05 and no field of the
chapter's own: `id` (the CX number; for a late entry the number issued from the block, with its label in `slug`),
`shortcut`, `case` (with its numbers; the expected observation stays in it), `numeric`, `plan_case` (`"numbers"`
for every entry except CX-269, which is `"words"`), `case_supplied_by`, `rule` (the text of the rule column),
`rule_ids` (the identifiers of the rule column and the gates below), `defined_in`, `same_as` (`"CX-179"` for
CX-268), `label`, `rerun` (on the FACT entries), `control_kind` and `plant`. This bead writes no row of the
negative-control registry `controls.toml`: N38b-05's seeding tool derives one control per entry (identifier
`NC-nnnn`, gates resolved from `rule_ids`, directly or through `registry/rule_gates.toml` where the rule names no
gate), and the join proves that every entry has exactly one. DESIGN, correcting an earlier text of this bead that
registered each row in the negative-control registry with a mutant switch and a catching test of its own: the
catalogue is the one place an entry is written, and which test catches the plant is the control's gate, derived by
the tool. The "Caught by" columns above are planning provenance.

`plant` is written `<planting bead>:cx-<nnn>` (or `:<label>` in lower case for a late entry): the Cargo feature (for
a `mutation`) or the `synthetic` fixture (for the other kinds) under which that bead builds the mutant of the
"Mutant" column in the real component; it may refine the name, keeping the number or label in it.

| Entry | `control_kind` | `plant` | Gates in `rule_ids` | `label` |
|---|---|---|---|---|
| CX-259 | mutation | `N24c-02:cx-259` (KA-594's battery row, N24c-10) | through `rule_gates.toml` (§24.5, C-30) | DESIGN |
| CX-260 | mutation | `N24c-02:cx-260` (KA-595's battery row, N24c-10) | through `rule_gates.toml` (§24.5) | DESIGN |
| CX-261 | mutation | `N24c-04:cx-261` | through `rule_gates.toml` (§24.8) | DESIGN |
| CX-262 | mutation | `BD-123:cx-262` (the wheel-loop row of N24b-04) | GATE-ARENA-04 | DESIGN |
| CX-263 | mutation | `BD-123:cx-263` (the cart fixture's batch accounting, with N24b-04) | GATE-ARENA-04 | FACT: `PILOT-PT-16` (the planar sill pilot of 156 rows, re-run by BD-043) |
| CX-264 | planted-design | `N24b-05:cx-264` (a task file with no declared rotor inertia or control cadence) | GATE-ARENA-04 | FACT: `PILOT-PT-18` (the exploratory planar sweep, row 18 of N38a-11, the rotor-and-cadence pilot it re-runs) |
| CX-265 | mutation | `N24c-08:cx-265` (BD-124's fixed-n engine admitting a tie at n = 64) | GATE-ARENA-05 | DESIGN |
| CX-266 | a-a | `N24c-08:cx-266` (the early-stopping adversary against a fixed-n study) | through `rule_gates.toml` (INV-ARENA-05, FROB-REF-552) | DESIGN |
| CX-267 | a-a | `N24c-09:cx-267` (the resubmitting adversary on reused clusters) | GATE-QUAL-01 | DESIGN |
| CX-268 | mutation | `N24b-22:cx-268` (fixture 2's plate-drag rows) | GATE-ARENA-06 | DESIGN |
| CX-269 | planted-input | `N24c-02:cx-269` (a verdict assembled from two episodes) | through `rule_gates.toml` (§24.5) | DESIGN; `numeric = false` until N24c-02 records the fixture of `stage_concatenation_refused` in `case_supplied_by` |
| ARENA-W1 | mutation | `N24c-17:arena-w1` | GATE-ARENA-10 | DESIGN |
| ARENA-W2 | mutation | `N24c-17:arena-w2` | GATE-ARENA-10 | DESIGN |
| ARENA-A1 | planted-design | `N24c-22:arena-a1` (a task with no dwell) | GATE-ARENA-11 | DESIGN |
| ARENA-A2 | mutation | `N24c-27:arena-a2` (an audit that runs no idle law) | GATE-ARENA-15 | DESIGN |
| ARENA-I1 | mutation | `N24c-31:arena-i1` (ε = 0; a planner that reports forecast variance) | GATE-ARENA-12 | DESIGN |
| ARENA-I2 | planted-input | `N24c-31:arena-i2` (a promotion offered item-bank episodes) | through `rule_gates.toml` (§24.21) | DESIGN |
| ARENA-E1 | mutation | `N24c-37:arena-e1` (a tick-keyed adapter) | GATE-ARENA-14 | DESIGN |
| ARENA-E2 | mutation | `N24c-37:arena-e2` (a footfall-keyed adapter) | GATE-ARENA-14, FROB-REF-556 | DESIGN |

The two FACT entries are pilot-derived (N38b-05): CX-263 carries `rerun = true` with `rerun_receipt` equal to the
receipt of BD-043's sill result on `PILOT-PT-16` when `pilots::rerun_status("PILOT-PT-16")` reads `reproduced`,
and CX-264 carries `rerun = true` with `rerun_receipt` equal to the receipt of N38a-11's result on `PILOT-PT-18`
when `pilots::rerun_status("PILOT-PT-18")` reads `reproduced`; each carries `rerun = false` only while its record
reads not run or `differs`, and a pair that disagrees with the register is the load error
`rerun-disagrees-with-pilot`. A FACT entry's control with `rerun = false`, and CX-269's while it has no numeric
case, is written by the seeding tool as an `obligation`: not runnable, and it makes no gate eligible. An entry that resolves to no gate is written to
`registry/seed_findings.toml` with `gate-added` requested and reported at the milestone exit, never dropped.

## Acceptance criteria

- [ ] All eleven numbered counterexamples and the eight local ones are entries of `counterexamples.toml` with every field of N38b-05's schema filled and the `control_kind`, `plant`, gates and `label` of the registration table; N38b-05's loader accepts them and counts 11 for the Arena chapter; no row of `controls.toml` is written by this bead.
- [ ] After the seeding tool of N38b-05 is re-run, each of the 19 entries has exactly one control whose `seed` names it, the tool's second run changes no byte, and the join passes; CX-263 and CX-264 (FACT) carry the `rerun` pair their pilot records `PILOT-PT-16` and `PILOT-PT-18` give through `pilots::rerun_status`, and with CX-269 (no numeric case) are obligations while `rerun = false` or the fixture is missing; an entry resolving to no gate is in `seed_findings.toml`.
- [ ] The rule cells carry the plan's identifiers (C-30, the relation of §38.7, GATE-ARENA-04, `eval.preanalyze`, GATE-QUAL-01).
- [ ] The case numbers in the registry equal those printed here (values are not improved in transit).
- [ ] The block request for eight `cx` numbers owned by N24-CX is committed in `allocations.toml`; once issued, the eight late entries carry its numbers in table order with their labels in `slug`, `registry::alloc::validate` passes, and no CX number is invented outside the block or reused.
- [ ] CX-269 is flagged as having no numeric case (`plan_case = "words"`, `numeric = false`), with the refusal test as its only control once N24c-02 supplies the fixture.

## Test plan

### Unit tests
`cargo test -p frob-conformance --test arena_counterexamples`: `registry_has_19_arena_entries`;
`every_entry_has_plant_and_gate` (every entry has a `control_kind` from the closed list, a `plant` naming its
planting bead, and either a gate in `rule_ids` or a rule identifier that `rule_gates.toml` resolves or that the
seeding findings list); `entries_in_n38b05_schema` (every entry parses with N38b-05's loader; a scratch entry with a
field `mutant` or `caught_by` is rejected as unknown); `fact_entries_follow_pilots` (CX-263 names `PILOT-PT-16` and CX-264
names `PILOT-PT-18` in `label`; on a scratch register with the record `registered` the entry carries `rerun =
false` and seeds as an obligation; with the record `reproduced` it carries `rerun = true` and the result's receipt;
`rerun = true` against a `registered` record is refused `rerun-disagrees-with-pilot`, and `rerun = true` with no
receipt is refused);
`cx268_same_as_cx179`; `case_numbers_match` (the values above are recomputed where they follow from
a formula: 0.95² = 0.9025; 12.5 + 40 = 52.5 and (40 − 12.5) × 0.020 = 0.55; (0.25 + √0.2)/2 = 0.3486;
1 − 0.95¹⁴ = 0.512; 13/256 = 0.0508; tick 1,500 × 2 ms = 3.0 s and × 1 ms = 1.5 s; ten footfalls × 0.5 s = 5.0 s
and × 0.4 s = 4.0 s; (0 − 0.15)/0.50 = −0.3; 0.1 × 10 − 0.0025 = 0.9975); `not_run_entries_have_milestones`;
`late_entries_in_block` (after the block is issued, each late entry's number lies inside N24-CX's block in table
order; a planted entry numbered outside it is refused with `unallocated-number`).

### End-to-end script and logging
No script of its own: the unit tests above and N38b-05's join cover the registry. The controls seeded from these
entries are run by the control runner of N38b-04, invoked by the capability scripts of the planting beads (the
fixtures battery, the wrench-slice, audit, item-bank and coupling scripts), each of which logs for every control
it runs: the entry's identifier, the control's `NC-` identifier, the plant, the gate, and `control_caught:
true|false`.

### Known answers and negative controls
The table is the content. Determinism: every case here is exact or hand-checkable arithmetic and must reproduce
identically on every platform.

## Considerations

- Two rows rest on facts recorded from exploratory pilots (CX-263, CX-264); the pilots kept their own law and scorer, so those rows protect a rule and are not product measurements. They are FACT entries whose `rerun` pair follows their pilot records: BD-043 re-runs the sill pilot (`PILOT-PT-16`) and N38a-11 the rotor-and-cadence pilot (`PILOT-PT-18`) here.
- Owed by the planting beads and the control runner, not by this registry: for every entry whose mechanism exists at the current milestone, enabling the plant makes its gate's entries fail and disabling it makes them pass; entries whose plant arrives later are reported `stale` with the milestone and make their gates ineligible.
- A counterexample proves a shortcut wrong on one case; it does not bound how wrong. The gates measure the mechanisms; these entries keep the shortcuts from returning unnoticed.
- The catalogue marks CX-268 as identical to CX-179 of the Traction chapter; the entry carries `same_as = "CX-179"`, both entries keep their own controls (one control per protected rule), and the fixture is shared.

## References

Appendix N (N.21: CX-259 to CX-269), §24.5, §24.8, §24.10 to §24.13, §24.19 to §24.23, §0.7; KA-594, KA-595, KA-598, KA-610, KA-612, KA-616, KA-618, KA-620, KA-622.
