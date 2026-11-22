---
key: "N36-CX"
title: "registry: performance counterexamples CX-374 to CX-383 with breaking cases, rules and executable tests"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-34"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.1: the negative-control registry `frob-conformance/registry/controls.toml` and the counterexample catalogue format (the Gauntlet's unit), which these rows seed"]
related: ["N36-KA", "N36-REF", "N36-03", "N36-04", "N36-05", "N36-08", "N36-09", "N36-15", "N36-16", "N36-18", "N36-19", "BD-199", "BD-219", "BD-216"]
gates: []
plan_refs: ["Appendix N.33", "§36.2", "§36.3", "§36.4", "§36.5", "§36.6", "§36.9", "§36.10", "§36.11", "§36.12", "§36.13", "§38.11"]
---
## Background

A counterexample is the smallest concrete case that breaks a tempting shortcut. The plan keeps a catalogue of them
so that each rule in the design is tied to the case that motivated it, and so that the test suite contains, for
each rule, an input on which the shortcut is demonstrably wrong. Ten counterexamples belong to the performance
program. They are a compact statement of why the program is built the way it is: every one is a way a speed
figure, a speed argument or a memory argument goes wrong in practice.

They seed the negative-control registry: for every rule there is a planted defect or a hand-checkable computation
that a test must catch or reproduce. This bead is the registry of the ten. Mechanism beads cite these numbers in
their test plans; the registry check fails if a row has no executable test.

Terms. *World-step*: one physics step of one world. *Env-step*: one control tick, d world-steps (d the
decimation). *Ceiling*: the largest speedup an experiment could show, computed beforehand as 1/(1 − f + f/r + h)
for an improved fraction f, a local factor r and an added overhead h. *Shard*: the part of a batch held by one
device. *Referee*: the slow, trusted CPU tier that adjudicates results of the fast tiers.

## Scope

In: the ten rows in the counterexample catalogue (section `perf`) with the breaking case and its numbers, the
rule protected, the executable test and its owner; the registry check. Out: the mechanisms and their own tests
(owners named per row); the known-answer registry (N36-KA).

## Technical approach

Rows live in `crates/frob-conformance/registry/counterexamples/perf.toml` with fields `id`, `shortcut`, `case`,
`rule`, `test`, `owner`, `tier`.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Executable test (owner) |
|---|---|---|---|---|
| CX-374 | Env-steps and world-steps can be quoted side by side without a decimation | 400 k env-steps/s of training at d = 10 is 4.0 M world-steps/s inside training, and cannot stand beside a physics-only figure of 2.0 M world-steps/s on the same card. Incumbents do not share a decimation either: one publishes d = 5, another reports 1.20 M env-steps/s of physics as 4.8 M physics steps per second (d = 4) | ADR-PERF-02 and INV-PERF-01: two units, each written with its decimation, with the ordering checked on every report; GATE-SPARK-19 | `perf::units::ordering_known_answer`, `native_world_steps_not_comparable` (N36-03) |
| CX-375 | A faster environment step delivers the ten-fold | On one incumbent stack the environment step is 87% of training wall-clock, so removing it entirely gives at most 1/(1 − 0.87), about 7.7; on another physics is about 29% of the environment step, so a faster physics kernel alone gives at most 1.4; removing inference and learning gives at most 1.15. On quadruped briefs no single phase reaches ten-fold | Engine throughput is a parity requirement; ADR-PERF-05, ceiling first; `FROB-REF-866` | `perf::amdahl::no_single_phase_reaches_ten`, `nested_shares_do_not_multiply` (N36-04) |
| CX-376 | A pruning ratio multiplies with a device speedup | 100 candidates of 10 s each; pruning removes 90; the ten survivors are the hard ones and accelerate by 2, not by the 5 measured on the full set. The result is 1,000/50 = 20, not 10 × 5 = 50 | Rule 2 of the speedup bound: a pruning ratio is never multiplied by a speedup measured on work already removed | `perf::bound::pruning_composition_flagged` (N36-05) |
| CX-377 | Family batching gives a large speedup by itself | A generation of 256 designs at 16 worlds and 10 s each is about 10 s of simulation; a mixed batch removes switching and adds padding (30%). If a switch costs 1 s, f = 256/266 = 0.96 and the bound is 266/13 ≈ 20; if a switch costs 0.1 s, f = 0.72 and the bound is 35.6/13 ≈ 2.7. The gain depends almost entirely on the incumbent's cost of switching designs | speedup ≤ 1/(1 − f + f/r + h), with f, r and h measured on the strong baseline's critical path; the switch cost is measured in the incumbent baseline before the mixed-batch experiment is funded | `perf::bound::family_batch_switch_1s`, `family_batch_switch_0_1s` (N36-05); the measured switch cost (BD-040) |
| CX-378 | Optimize the component first and measure the gain afterwards | A specialized solver missed its speed gate at 1.345 warm and 1.088 cold; a later measurement showed a residual of 575.7 µs in the specialized batch that the solver does not touch (parsing, assembly, checking, serialization and harness, not separately timed), so a solver of zero cost would have reached about 1.46 (842.8 ÷ 575.7). The ceiling was computable before the experiment ran | Ceiling first: an experiment whose ceiling is below its goal is refused (`FROB-REF-866`) | `perf::bound::recorded_pilot_ceiling`, `late_record_rejected` (N36-05); the ledger's landing check (N36-07) |
| CX-379 | Two cards train twice as fast | An incumbent's own two-card run takes about 325 s against about 410 s on one card, a gain of 1.26: about 45 s of the 410 s is compilation, the learner runs on one stream, and sample efficiency falls as the batch grows | Weak and strong scaling are measured separately (target rows 5 and 6); KILL-SHARD below a paired lower bound of 1.6 | `perf::shard_ceiling::incumbent_fixture`, `local_factor_from_size_curve` (N36-09); the scaling campaigns (BD-199) |
| CX-380 | Gradients from shards can be combined as the mean of shard means | One sample with gradient 1 and three with gradient 0 give 0.25. Split as {1} and {0, 0, 0}, the mean of shard means gives (1 + 0)/2 = 0.5; with an empty shard it divides by zero or halves the answer | The distributed normalization rule: a sum of sums in device order divided by the global valid count; GATE-SPARK-20 | `shards::device::gradient_fixture_all_splits`, `mean_of_shard_means_is_wrong` (N36-08); the learning unit's reducer tests |
| CX-381 | Device memory limits the number of worlds | By the byte model a world is about 35 kB (quadruped) or 97 kB (humanoid): 8,192 worlds are 0.28 GB and 0.79 GB, 1 to 3% of a 24 GB card. Memory limits distinct scenes and camera frames, not worlds: two 640 × 480 cameras on 8,192 worlds are 37.5 GiB | The memory arithmetic per world, per scene and per camera; camera resolution and cadence are part of the task identity, not a performance knob | `estimate::worlds_are_not_the_binding_term` (BD-219); `estimate::scene::one_scene_per_world_is_614_gb` (N36-16); `estimate::frames::repairs_never_touch_resolution_or_cadence` (N36-19) |
| CX-382 | A derived bandwidth ceiling can be used as a planning figure | With assumed traffic of 0.2 to 1.5 MB per humanoid world-step the estimate's central ceiling is 0.7 to 1.1 M world-steps/s, below the 2.55 M an incumbent measures on the same class of card: either a step touches fewer bytes than assumed or the efficiency is higher | Bytes touched per world-step is measured per launch and reported in the disclosure block (GATE-SPARK-22) | `perf::bandwidth::efficiency_contradiction_fixture` (N36-18) |
| CX-383 | The referee can check every device world | A device produces T·h world-seconds per second: for the humanoid row 1.7 M × 2 ms = 3,400. Twenty-four cores at a real-time factor of 0.2 to 1.0 cover 4.8 to 24 of them, 0.14 to 0.7% | The referee is a sampler; its share is reported (target row 17); `batch-cpu` shadows more (550 world-seconds per second, 16%) | `perf::coverage::coverage_humanoid`, `referee_cannot_cover_everything` (N36-15) |

**What "executable" means here.** Each row has (i) a test that reproduces the case's numbers exactly from the
stated inputs, and (ii) where the shortcut is an implementation choice, a planted variant of the real component
that takes the shortcut and that a named test must reject. Rows whose shortcut is an argument, not code
(CX-375, CX-377, CX-379, CX-382, CX-383), have (i) and a guard test that a report or a record making the argument
is flagged: a speed experiment whose goal exceeds its computed ceiling is refused; a two-card prediction filed
after its measurement is rejected; a bandwidth ceiling cited without a measured bytes-touched figure keeps its
HYPOTHESIS label in generated documentation.

**Registry check.** A test loads the catalogue and fails if any of CX-374 to CX-383 is missing, if a row has no
test name or owner, or if a named test does not exist in the workspace's test listing. A CX number is never
reused or renumbered.

## Acceptance criteria

- [ ] `counterexamples/perf.toml` holds ten rows, CX-374 to CX-383, each with the case's numbers, the rule and at least one executable test with its owner.
- [ ] The registry check passes and fails when a row, a test name or a named test is removed.
- [ ] For the five rows whose shortcut is code (CX-374, -376, -378, -380, -381), the planted variant exists behind a test-only feature and is rejected by its named test; each is also entered in the negative-control registry under its gate (GATE-SPARK-19, -20 or -22) where it has one.
- [ ] For the five rows whose shortcut is an argument, the numbers are reproduced by a test and the guard test exists.
- [ ] The printed catalogue of the chapter's counterexamples is generated from this file.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry::counterexamples::perf`:
- `rows_374_to_383_present`; `every_row_has_test_and_owner`; `named_tests_exist`.
- `numbers_reproduce` (one sub-test per row): 4.0 M against 2.0 M; 7.7, 1.4 and 1.15; 20 against 50; 20 and 2.7; 1.46; 1.26; 0.25 against 0.5; 0.28 GB, 0.79 GB and 37.5 GiB; an implied efficiency above 1 for the central assumption; 0.14 to 0.7%.
- `planted_variants_are_rejected`: runs the five planted variants through their named tests and expects each to fail.
- `argument_guards`: a ceiling-less experiment, a late prediction and an unlabelled bandwidth ceiling are each flagged.

### End-to-end script and logging

`scripts/e2e/perf_harness_gate19.sh` (N36-30) runs the catalogue's check as part of its `preflight` and logs one
`assert` line per row with the CX number, the test names and their results. The scripts of the scaling and
estimator gates log the rows they exercise (CX-379 and CX-380; CX-381 and CX-382) beside their measurements.

### Known answers and negative controls

The ten rows are the content; each is both a known computation and the specification of a negative control.
Cross-references to known answers: CX-374 ↔ KA-806; CX-375 ↔ KA-818; CX-376 ↔ KA-811; CX-377 ↔ KA-810; CX-378 ↔
KA-812; CX-380 ↔ KA-813; CX-381 ↔ KA-814, KA-816, KA-817; CX-382 ↔ KA-822; CX-383 ↔ KA-821. Determinism: all
number reproductions are exact or closed-form host arithmetic.

## Considerations

- Three of these cases (CX-375, CX-378 and the program-level entry on running a speed comparison before accounting for shared work) are the reason the program computes ceilings first; they are deliberately kept as separate rows because each breaks a different instinct.
- CX-379's numbers are published figures of an incumbent, to be re-read and then re-measured on `trj`; the row keeps the published pair as its fixture and the measured pair is printed beside it by the scaling bead.
- CX-382 is a counterexample to one of the plan's own estimates, kept on purpose: it is why bytes touched are measured.
- The learning chapter has its own rows for the shard-gradient and shard-statistics shortcuts; CX-380 is this chapter's statement of the first and the two registries cross-reference each other so that one planted reducer serves both.

## References

Appendix N (the rows of the performance program), §36.2, §36.4, §36.5, §36.6, §36.9 to §36.13, §38.11;
ADR-PERF-02, ADR-PERF-05; INV-PERF-01; GATE-SPARK-19, -20, -22; KILL-SHARD; FROB-REF-866.
