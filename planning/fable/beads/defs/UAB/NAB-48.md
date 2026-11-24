---
key: "NAB-48"
title: "registries/ideas.toml: the 27 blind spots found only under cross-examination, review or execution"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-15", "NAB-42"]
needs: []
related: []
gates: []
plan_refs: ["Appendix B.6", "Appendix B.6.1", "Appendix B.6.2", "Appendix B.6.3", "§0.7"]
---
## Background

The most valuable output of the idea passes behind FrankenRobotics was not an idea. It was the list of things
that **no list contained**: gaps that every independent planner missed, and that surfaced only when the
planners were asked what was absent, or when a reviewer, an audit or an executed check found them. They are
blind spots in the strict sense: plausible, competent plans did not see them. Several became core mechanisms:
proving before any training that every policy on a sensor interface must fail; treating confirmation data as
a consumable that retries spend; engineering replay determinism instead of asserting it; ranking designs under
more than one contact formulation; making the building a design variable; computing the ceiling of a speed
test before running it; enumerating the verdicts a frozen decision rule can reach.

This bead carries the 27 blind spots into the idea-pass register (`registries/ideas.toml`, schema
`frob.ideapass/1`, NAB-42) as records `BS-01` to `BS-27`, in three groups: six named when the passes were asked
what no list contained; thirteen surfaced by review, audit or execution; and eight named after the last
cross-examination and not examined by a second pass. For each: the gap, why it was missed or the evidence that
exposed it, where it now lives, and the beads that carry it. The record exists so that the *reason* a
mechanism exists is not lost: an implementer who finds the blinded qualification lane cumbersome should be
able to read that agents which retry against held-out worlds spend the benchmark. Tier: Solid (a decision
record).

## Scope

In: records `BS-01` to `BS-27` with carriers and, for the third group, their standing after the later-added
subsections. Out: the schema and lint (NAB-42); the placed rows (NAB-44 to NAB-46); the dispositions of ideas
still unplaced (NAB-49).

## Technical approach

**Transcription rule.** One record per row. Fields: `gap`, `why_missed` (first group) or `evidence` (second
group) or `mechanism` (third group), `lives` (verbatim), `carriers`, `group` (`asked`, `found`, `late`),
`placed_rows` where the text names a row, `plan_checks` (group 2: the `PC-nn` ids of the source register,
NAB-15, joined as in `group2_checks_resolve`), `counts_toward_tally = false`. `disposition`: `placed` for the
first two groups; for the third,
`placed`, `placed-narrowed` or the link to a not-yet-placed record.

**Group 1: named when the passes were asked what no list contained** (`BS-01` to `BS-06`).

| Gap | Why no list had it | Where it now lives | Carried by |
|---|---|---|---|
| No one proved when every policy must fail because of sensor aliasing | Training compute was spent before the question was asked | IP-21: §20.12, preflight stage 8, §25.10 | BD-134, BD-240, BD-244, BD-290 |
| No one checked the orderings of capture, packet, estimator, watchdog and commit | Timing was treated as a rate, not as a schedule | IP-71: §20.4.7; the seven instants of §11.8; §29.15 | BD-078, BD-096, BD-189 |
| Agents that retry against held-out worlds spend the benchmark | Confirmation data was not treated as a consumable | IP-82: the blinded qualification lane of §24.13 | BD-259, BD-260, BD-261 |
| Replay determinism was asserted in several proposals and engineered in none | Seeding the random numbers does not fix the arithmetic or the reduction order | IP-58: §35; batch-composition invariance (GATE-SPARK-15) | BD-177, BD-181, BD-203 |
| Every list ranked designs under one contact formulation | Rank stability across model forms was not a concept in any list | IP-29: §16.9, §16.10, A-19 | BD-208, BD-266, BD-346 |
| Every repair considered lived in the robot | The building was not a design variable | IP-63: the retrofit catalog of §19.12; BET-37 | BD-369 |

**Group 2: surfaced by review, audit or execution** (`BS-07` to `BS-19`). The evidence column cites executed
plan-time checks where it says FACT; those checks have records in the source register and are re-run before a
gate cites them.

| Gap | Evidence | Where it now lives | Carried by |
|---|---|---|---|
| A speed test can be powerless | A pilot's shared overhead capped the measurable ratio at 1.46 before the test ran (FACT, the planning review log) | The ceiling-first rule of §36.5; the anti-goal of §6.3; GATE-AUGUR-14 | BD-354, N25a-27, N40-18 |
| A frozen decision rule may be unable to return some verdicts | At 64 pairs one of four verdicts was unreachable; it is first reachable at 86 (FACT, the planning review log) | `eval.preanalyze` and `study.plan` (§24.12, §32.6.1); GATE-ARENA-05, GATE-ATEL-06 | BD-124, N24b-06, N28a-19 |
| Local contracts hide time correlation | Two fingers on one bus have identical local summaries, and a rule that adds peaks prunes a feasible schedule | C-12; §14.6; the negative control of §25.21.1 | BD-060, BD-077, BD-108 (A-23), BD-360, SPEC-C-12 |
| In-family checks cannot expose a structural error | A probe that was right on every case inside its law family was wrong on every case of an omitted law (FACT, the planning review log) | §25.14: verdicts conditional on the law family; the direct measurement always offered; GATE-AUGUR-11 | BD-291, N25c-17 |
| The actuator authority model changes the hardware decision | Independent forces and equal passive springs give different foot loads on the same body | §25.13; authority is part of the claim (FROB-REF-851); A-08 names its load sharing | BD-100, BD-130, N25c-11 |
| Widening a hypothesis set by a hair invalidates certificates wholesale | Adding one nearby gain to a docking family reversed every admitted cell | C-18 (assumption lattice and polarity); §20.16; `result.revalidate` | BD-129, BD-190, BD-292, SPEC-C-18 |
| Controller rate and rotor inertia flip benchmark outcomes | In an executed planar study two success fractions both became 1 with a faster loop or added inertia (FACT, §23.16) | A-22; the `rate-sensitive` detector of §23.16; reflected rotor inertia on actuator cards (§15) | BD-108 (A-22), BD-267, N15b-11, N23b-06 |
| Contact counts diverge inside a cohort of one morphology | Homogeneous structure does not give homogeneous work | §18.11 and the `capacity-biased` label; contact capacity in the bucket signature (§17.5) | BD-168, BD-171, N18b-22 |
| A screen can discard the winner | A stopping test between two survivors says nothing about discarded designs | Blinded stratified audits of discards and a planted winner (§26.10, §28.12); GATE-ATEL-09, GATE-NEMESIS-07 | BD-345, N26b-05, N28b-30 |
| A monitor cannot see a breach until it is too late | Two friction regimes give identical histories until the first slip | Independent breach labels (GATE-SHIELD-03); the pre-slip aliasing fixture of the counterexample catalogue | BD-339, N22c-27 (the pre-slip aliasing fixture) |
| Once the engine is fast, the agent's turn dominates | The agent's serial rounds become the largest term of the loop (§2.2) | Population verbs and maps per dispatch (§17.12); the cascade (§28.11); `solve.run` (§32.6.2); GATE-PHYLUM-08 | BD-342, BD-352, BD-372 |
| The earlier draft measured itself where it trailed | Targets sat below published incumbent figures | Targets restated as ratios to the incumbent measured on this hardware (§36.3); GATE-SPARK-21; GATE-CORTEX-05 | BD-039, BD-040, BD-217, BD-218 |
| Additions proposed by examiners | A constrained allocator that reports refusals under any learned policy; a shield fixture with a verified backup; a native paired-intervention replay harness before any browser bundle; edit to first accepted result as the unit for batching claims | §22.10 and §22.6; GATE-SHIELD-02; §27 and §29.8 ahead of §32.15; §17.8 and GATE-PHYLUM-04 | BD-251, BD-336 (allocator refusals); BD-338 (backup fixture); BD-205, BD-279 (replay harness before the browser bundle); BD-184 (edit to first accepted result) |

**Group 3: named after the last cross-examination, and not examined by a second pass** (`BS-20` to `BS-27`).
Each rested on its proposer alone when it was recorded, so none was adopted on that evidence. Two of them
answer the strongest standing objection to sampling-based control, that a fast evaluation scores body and
controller together (ranking designs by achievable performance; bracketing what the embodiment can achieve).
Since the list was written, later-added subsections of the specification placed all eight in full or in a
narrowed form; the last column records that and the carrying beads.

| Idea | Mechanism in one line | Standing in the plan | Standing after the late additions, and carriers |
|---|---|---|---|
| Rank designs by achievable performance | Run each design at several controller budgets on the same world addresses, fit a saturation curve, and escalate only the contenders | Not yet placed (the not-yet-placed list (NAB-49)). The flag `controller-sensitive` exists in §8.2 with no procedure that sets it | Placed by a later subsection: controller-budget curves and the `controller-limited` flag. N28c-32 (pilot), N28c-35, N28c-37, N28c-39 (GATE-ATEL-17), N28c-41 (KILL-BUDGETCURVE) |
| Bracket what the embodiment can achieve | A sound optimistic bound over all admissible controllers beside an achieved result from a rollout | Placed in a narrowed form: inner and outer design sets (§25.4), and exclusions that hold for every controller (C-31, §22.11; `mission.refute`) | Placed as stated, and extended by a later subsection on utility brackets and the dominance stop: BD-128, BD-135, BD-337; N25e-13 to N25e-17 (GATE-AUGUR-28, KILL-AUGUR-BRACKET) |
| Attack the task specification first | Run deliberately capable or degenerate bodies against the task's own predicate to find loopholes | Not yet placed (the not-yet-placed list (NAB-49)). `task.check` runs labelled examples and a mutation check (§24.5) | Placed by a later subsection: `task.audit` with foil bodies and intent invariants. N24c-20 (pilot), N24c-21, N24c-23 (GATE-ARENA-11, GATE-ARENA-15, KILL-FOIL) |
| A calibrated item bank for the test battery | A response model over battery items, so uninformative scenes are skipped | Not yet placed (the not-yet-placed list (NAB-49)). Battery selection by weighted coverage is §26.11 | Placed by a later subsection: the item bank. N24c-29 (pilot), N24c-31 (GATE-ARENA-12, GATE-ARENA-13, KILL-ITEMBANK) |
| Common randomness compiled from the task | Key disturbances by place, time and mission event, so pairing survives edits and rate changes | Placed: space-indexed fields and (world, replicate) addresses (C-40, §25.16, §35.9); a pairing keyed by event order is refused (FROB-REF-556) | Placed as stated, and extended by exposure domains and the coupling test: BD-180, BD-359; N24c-36 (GATE-ARENA-14), N25e-06 (KILL-COUPLING) |
| Convex synthesis from inverse dynamics | Inverse dynamics is affine in the inertial parameters, so mass placement, gearing and actuator sizing along nominal trajectories form one cone program | Not yet placed (the not-yet-placed list (NAB-49)). Neighbours: mass placement (IP-79) and trajectory optimization with design variables (IP-52) | Placed by a later subsection: placement and sizing along a prescribed motion. N25e-18 (pilot) to N25e-22 (GATE-AUGUR-29, KILL-SYNTH-REGRESSOR) |
| Pre-characterized sub-assembly models | Characterize limbs and modules once at their ports and screen assemblies by composition | Placed in a narrowed form: the limb port interface (IP-53) and module contracts that carry time correlation (IP-01) | Placed as stated, and extended by module capability tables and the composed quasi-static screen: BD-060, N14a-22; N14b-28, N14b-30 (GATE-SKEL-09, GATE-SKEL-10, KILL-SKEL-CELL) |
| Sensor co-design from estimator observability and self-occlusion | Observability bounds from the invariant filter, with occlusion by the robot's own body, optimized with the morphology | Partly placed: §20.11, A-17, and an estimated-state twin of every return (§23.17). The self-occlusion term is in the not-yet-placed list (NAB-49) | Placed as stated; the self-occlusion term is now placed by a later subsection: BD-120, BD-239; N20b-32, N20a-35 (pilot), N20b-37, N20b-38 (GATE-SENS-16, GATE-SENS-17, KILL-VISIBILITY) |

**Numbers quoted in the evidence that tests pin.** A pilot's shared overhead capped the measurable ratio of a
speed test at 1.46 before the test ran (842.8 ÷ 575.7). At 64 pairs one of four verdicts of a frozen decision
rule was unreachable; it is first reachable at 86. Two fingers on one unit-current bus have identical local
summaries, with zero one-tick schedules and two two-tick schedules, and a rule that adds peaks prunes a
feasible schedule. A probe that was right on every case inside its law family (675 of 675) was wrong on every
case of an omitted law (0 of 225). Independent forces and equal passive springs give different foot loads on
the same body ((175, 115, 85, 25) N against a 160 N rating under springs). Adding one nearby gain (33/32) to a
docking family reversed every admitted cell. In an executed planar study two success fractions both became 1
with a faster loop or added inertia.

## Acceptance criteria

- [ ] Records `BS-01` … `BS-27` exist in three groups of 6, 13 and 8 with the columns verbatim and carriers as
      listed.
- [ ] Every carrier key exists as a bead.
- [ ] Each record of group 2 that cites a plan-time check names the check's source-register id (`PC-nn`), and
      the citation lint treats it as uncitable by a gate until re-run.
- [ ] Each record of group 3 has a standing (`placed`, `placed-narrowed`) and carriers, and the test that
      compares group 3 with the not-yet-placed list (NAB-49) finds no idea in both states.
- [ ] The traceability lint of NAB-42 passes on these records.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `blind_spots`:

- `twenty_seven_records_in_three_groups` (6, 13, 8).
- `carriers_resolve`.
- `group1_placed_rows`: BS-01 ↔ IP-21; BS-02 ↔ IP-71; BS-03 ↔ IP-82; BS-04 ↔ IP-58; BS-05 ↔ IP-29;
  BS-06 ↔ IP-63.
- `group2_checks_resolve`: the speed-test ceiling ↔ PC-09; the unreachable verdict ↔ PC-18; the shared bus ↔
  PC-12; the in-family probe ↔ PC-03; the authority model ↔ PC-02; the widened hypothesis set ↔ PC-07.
- `group3_all_have_carriers`; `group3_consistent_with_not_yet_placed_list`.
- `pinned_numbers`: 842.8/575.7 = 1.464; 64 and 86; 675 and 225; 175 > 160.
- `rows_frozen` against `tests/fixtures/ideas/blind_spots.tsv`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 27 records, zero unresolved carriers, and the consistency
of group 3 with the not-yet-placed list. Logged per STD-E2E.

### Known answers and negative controls

Known answers: 27 records (6 + 13 + 8); six joins to placed rows in group 1; six joins to plan-time checks in
group 2. Negative controls: a group-3 record marked `placed` with no carrier; a group-2 record whose evidence
is cited by a gate before the check is re-run (`cite.plan-check-not-reproduced`, from the citation lint); a
blind spot's row deleted because "the mechanism exists now" (the frozen-id comparison fails: the reason is the
point of the record).

## Considerations

- Blind spots are the best argument for the independence rules of the program (fresh-context exit reviews;
  sweeps by non-implementers): every one of these was invisible to the party that held the plan.
- The eighth record of group 2 (contact counts diverge inside a cohort of one morphology) is why comparisons
  carry a `capacity-biased` label; the twelfth (the earlier draft measured itself where it trailed) is why
  targets are ratios to the incumbent measured on this hardware.
- Group 3's standing changed between the writing of the list and the final specification; the record keeps
  both the original standing and the later one so the history is visible.

## References

Appendix B.6 (all three parts); §0.7 (counterexamples are first-class; the ceiling is computed before any
speed test; a frozen decision rule is enumerated before sampling).
