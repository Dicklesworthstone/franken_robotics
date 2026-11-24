---
key: "N36-CX"
title: "registry: performance counterexamples CX-374 to CX-383 with breaking cases, rules and executable tests"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-34"
deps: ["BD-025", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-040", "BD-199", "BD-219", "N23-CX", "N36-03", "N36-04", "N36-05", "N36-07", "N36-08", "N36-09", "N36-15", "N36-16", "N36-18", "N36-19", "N36-30", "N36-KA", "N36-REF", "N36p1-02", "N38b-04"]
gates: []
plan_refs: ["Appendix N.33", "§36.2", "§36.3", "§36.4", "§36.5", "§36.6", "§36.9", "§36.10", "§36.11", "§36.12", "§36.13", "§38.11", "§39.8"]
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

In: the ten entries in the counterexample catalogue `crates/frob-conformance/registry/counterexamples.toml` with
the breaking case and its numbers, the rule protected, the plant and its gates; the compiled-in table of
executable tests, owners and due milestones; the registry check and its script. Out, with the bead that has it:
the catalogue's record schema and the seeding tool that writes `crates/frob-conformance/registry/controls.toml`,
allocates `NC-` numbers and proves completeness by the join (N38b-05; this bead writes no row of
`controls.toml`); the control runner (N38b-04); the mechanisms, their plants and their own tests (owners named per
row); the known-answer registry (N36-KA).

## Technical approach

**Entry format.** Each row is one entry in the record schema of N38b-05 and no other field: `id`, `shortcut`,
`case` (with its numbers), `numeric = true`, `plan_case = "numbers"`, `case_supplied_by` (empty), `rule`,
`rule_ids` (the ADR, INV, gate, kill and refusal identifiers of the rule and the gates of the control table),
`defined_in` (the sections Appendix N.33 gives), `same_as` (empty: none of the ten is ≡), `label` (DESIGN: the
recorded figures of CX-378 and CX-379 are fixtures of the arithmetic, and neither control depends on them being
re-measured), `control_kind`, `plant`. The target, defect and expected observation of the control
table are the texts the seeding tool copies into each control's `target`, `defect` and `expect`. DESIGN,
correcting an earlier draft of this bead that kept the rows in a private file
`registry/counterexamples/perf.toml` with fields `test`, `owner`, `tier` and `due`: the registries named in the
conventions are the only registry files, and the catalogue's loader admits only its schema. Those four fields are
kept as one compiled-in table keyed by CX number in this bead's test file
`crates/frob-conformance/tests/counterexamples_perf.rs`, which the registry check reads; per named test `due` is
the milestone of the bead that owns it (M0 for N36-03, N36-04, N36-05 and BD-040; M1 for N36p1-02; M2 for N36-07,
N36-08, N36-09, N36-15, N36-16, N36-18, N36-19, BD-199 and BD-219).

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Executable test (owner) |
|---|---|---|---|---|
| CX-374 | Env-steps and world-steps can be quoted side by side without a decimation | 400 k env-steps/s of training at d = 10 is 4.0 M world-steps/s inside training, and cannot stand beside a physics-only figure of 2.0 M world-steps/s on the same card. Incumbents do not share a decimation either: one publishes d = 5, another reports 1.20 M env-steps/s of physics as 4.8 M physics steps per second (d = 4) | ADR-PERF-02 and INV-PERF-01: two units, each written with its decimation, with the ordering checked on every report; GATE-SPARK-19 | `perf::units::ordering_known_answer`, `native_world_steps_not_comparable` (N36-03) |
| CX-375 | A faster environment step delivers the ten-fold | On one incumbent stack the environment step is 87% of training wall-clock, so removing it entirely gives at most 1/(1 − 0.87), about 7.7; on another physics is about 29% of the environment step, so a faster physics kernel alone gives at most 1.4; removing inference and learning gives at most 1.15. On quadruped briefs no single phase reaches ten-fold | Engine throughput is a parity requirement (§36.1); ADR-PERF-05, ceiling first; `FROB-REF-866` | `perf::amdahl::no_single_phase_reaches_ten`, `nested_shares_do_not_multiply` (N36-04); the guard `perf::bound::planted_inputs_are_refused` on the fixture `phase-ceiling-below-goal` (N36-05) |
| CX-376 | A pruning ratio multiplies with a device speedup | 100 candidates of 10 s each; pruning removes 90; the ten survivors are the hard ones and accelerate by 2, not by the 5 measured on the full set. The result is 1,000/50 = 20, not 10 × 5 = 50 | Rule 2 of the speedup bound: a pruning ratio is never multiplied by a speedup measured on work already removed (§36.5) | `perf::bound::pruning_composition_flagged` (N36-05) |
| CX-377 | Family batching gives a large speedup by itself | A generation of 256 designs at 16 worlds and 10 s each is about 10 s of simulation; a mixed batch removes switching and adds padding (30%). If a switch costs 1 s, f = 256/266 = 0.96 and the bound is 266/13 ≈ 20; if a switch costs 0.1 s, f = 0.72 and the bound is 35.6/13 ≈ 2.7. The gain depends almost entirely on the incumbent's cost of switching designs | speedup ≤ 1/(1 − f + f/r + h), with f, r and h measured on the strong baseline's critical path; the switch cost is measured at GATE-SPARK-21 before the experiment of §17 is funded | `perf::bound::family_batch_switch_1s`, `family_batch_switch_0_1s` and the guard `perf::bound::planted_inputs_are_refused` on the fixture `family-batch-no-switch-cost` (N36-05); the measured switch cost (BD-040, GATE-SPARK-21) |
| CX-378 | Optimize the component first and measure the gain afterwards | A specialized solver missed its speed gate at 1.345 warm and 1.088 cold; a later measurement showed a residual of 575.7 µs in the specialized batch that the solver does not touch (parsing, assembly, checking, serialization and harness, not separately timed), so a solver of zero cost would have reached about 1.46 (842.8 ÷ 575.7). The ceiling was computable before the experiment ran | Ceiling first: an experiment whose ceiling is below its goal is refused (`FROB-REF-866`) | `perf::bound::recorded_pilot_ceiling`, `late_record_rejected` (N36-05); the ledger's landing check (N36-07) |
| CX-379 | Two cards train twice as fast | An incumbent's own two-card run takes about 325 s against about 410 s on one card, a gain of 1.26: about 45 s of the 410 s is compilation, the learner runs on one stream, and sample efficiency falls as the batch grows | Weak and strong scaling are measured separately (rows 5 and 6 of §36.3); KILL-SHARD below a paired lower bound of 1.6 | `perf::shard_ceiling::incumbent_fixture`, `local_factor_from_size_curve` and the guard `perf::shard_ceiling::prediction_must_precede_measurement` (N36-09); the scaling campaigns (BD-199) |
| CX-380 | Gradients from shards can be combined as the mean of shard means | One sample with gradient 1 and three with gradient 0 give 0.25. Split as {1} and {0, 0, 0}, the mean of shard means gives (1 + 0)/2 = 0.5; with an empty shard it divides by zero or halves the answer | The distributed normalization rule: a sum of sums in device order divided by the global valid count (§36.13); GATE-SPARK-20 | `shards::device::gradient_fixture_all_splits`, `mean_of_shard_means_is_wrong` (N36-08); the learning unit's reducer tests |
| CX-381 | Device memory limits the number of worlds | By the byte model a world is about 35 kB (quadruped) or 97 kB (humanoid): 8,192 worlds are 0.28 GB and 0.79 GB, 1 to 3% of a 24 GB card. Memory limits distinct scenes and camera frames, not worlds: two 640 × 480 cameras on 8,192 worlds are 37.5 GiB | The memory arithmetic per world, per scene and per camera (§36.10, §36.12); camera resolution and cadence are part of the task identity, not a performance knob | `estimate::worlds_are_not_the_binding_term` (BD-219); `estimate::scene::one_scene_per_world_is_614_gb` (N36-16); `estimate::frames::repairs_never_touch_resolution_or_cadence` (N36-19) |
| CX-382 | A derived bandwidth ceiling can be used as a planning figure | With assumed traffic of 0.2 to 1.5 MB per humanoid world-step the estimate's central ceiling is 0.7 to 1.1 M world-steps/s, below the 2.55 M an incumbent measures on the same class of card: either a step touches fewer bytes than assumed or the efficiency is higher | Bytes touched per world-step is measured per launch and reported in the disclosure block (GATE-SPARK-22) | `perf::bandwidth::efficiency_contradiction_fixture` and the guard `perf::bandwidth::unmeasured_ceiling_keeps_hypothesis_label` (N36-18) |
| CX-383 | The referee can check every device world | A device produces T·h world-seconds per second: for the humanoid row 1.7 M × 2 ms = 3,400. Twenty-four cores at a real-time factor of 0.2 to 1.0 cover 4.8 to 24 of them, 0.14 to 0.7% | The referee is a sampler; its share is reported (row 17 of §36.3); `batch-cpu` shadows more (550 world-seconds per second, 16%) | `perf::coverage::coverage_humanoid`, `referee_cannot_cover_everything`, whose `check_claim` is the guard on the fixture `full-referee-coverage-claim` (N36p1-02, registered before the measurement; N36-15 scores it) |

**What "executable" means here.** Each row has (i) a test that reproduces the case's numbers exactly from the
stated inputs, and (ii) where the shortcut is an implementation choice, a planted variant of the real component
that takes the shortcut and that a named test must reject (`control_kind = "mutation"`). Rows whose shortcut is an
argument, not code (CX-375, CX-377, CX-379, CX-382, CX-383), have (i) and a guard test that a report or a record
making the argument is flagged (`control_kind = "planted-input"`, the plant being the record): a speed experiment
whose goal exceeds its computed ceiling is refused; a two-card prediction filed after its measurement is rejected;
a bandwidth ceiling cited without a measured bytes-touched figure keeps its HYPOTHESIS label in generated
documentation. Each planted input is a `synthetic` fixture file under `crates/frob-conformance/fixtures/perf/`
named by its plant (`phase-ceiling-below-goal.json`, `family-batch-no-switch-cost.json`,
`late-two-card-prediction.toml`, `unmeasured-bandwidth-ceiling.json`, `full-referee-coverage-claim.json`), and
each plant's `id` is `<planting bead>:<plant>` (N38b-05), the planting bead being the bead that builds it:
`N36-03:units-without-decimation`, `N36-05:phase-ceiling-below-goal`, `N36-05:pruning-times-speedup`,
`N36-05:family-batch-no-switch-cost`, `N36-05:ceiling-recorded-after`, `N36-09:late-two-card-prediction`,
`N36-08:mean-of-shard-means`, `BD-219:per-world-bytes-only`, `N36-18:unmeasured-bandwidth-ceiling` and
`N36p1-02:full-referee-coverage-claim`.

**Controls.** One per entry, derived by the seeding tool. Where Appendix N names a gate in the rule the control
takes it; for CX-375, CX-376, CX-378, CX-381 and CX-383, whose rules name a refusal, a section or a target row but
no gate, the gate is the one the plan ties to that rule, written into `rule_ids` so that the seeding resolves it
and never attaches the control to an unrelated gate (DESIGN): GATE-TENFOLD-09 ("ceiling before speed,
program-wide": every registered speed comparison records its ceiling and is refused below it, §39.8, the gate the
plan names for the ceiling-first refusal) for CX-375, CX-376, CX-377 and CX-378; GATE-SPARK-22 (the memory
estimate within 5% of measured high-water marks) for CX-381; GATE-FID-07 (the background referee's coverage
reported and its throughput loss at most 5%, the two clauses of row 17) for CX-383; GATE-SPARK-20 (weak and strong
scaling as paired lower bounds, rows 5 and 6) beside KILL-SHARD for CX-379.

| CX | Kind; plant | Target | Defect | Expected observation | Gates | Planted by; caught by |
|---|---|---|---|---|---|---|
| CX-374 | mutation; `units-without-decimation` | the throughput units and ordering check of `frob-core` perf | a report quoting env-steps beside world-steps with no decimation is accepted | the harness accepts the report; `native_world_steps_not_comparable` fails | GATE-SPARK-19 | N36-03; N36-30 at M2 |
| CX-375 | planted-input; `phase-ceiling-below-goal` | the ceiling-first check | an experiment registered for 10× from removing the environment step, whose ceiling is 7.7 | it is admitted instead of refused with `FROB-REF-866` | GATE-TENFOLD-09 | N36-04 (the limits); N36-05 (the refusal) |
| CX-376 | mutation; `pruning-times-speedup` | the composition of the speedup bound | the pruning ratio multiplied by the speedup measured on the full set | the bound reports 50 instead of 20 and `pruning_composition_flagged` fails | GATE-TENFOLD-09 | N36-05 |
| CX-377 | planted-input; `family-batch-no-switch-cost` | the ceiling-first check | a mixed-batch experiment registered with no measured incumbent switch cost | it is admitted with an uncomputable ceiling instead of refused | GATE-TENFOLD-09, GATE-SPARK-21 | N36-05; BD-040 measures the switch cost |
| CX-378 | mutation; `ceiling-recorded-after` | the optimization ledger's landing check | a ceiling recorded after the measurement it bounds is accepted | the late record lands; `late_record_rejected` fails | GATE-TENFOLD-09 | N36-05 at M0; N36-07 at M2 |
| CX-379 | planted-input; `late-two-card-prediction` | the two-card pilot record | a strong-scaling prediction filed after its measurement | it is accepted as a prediction instead of rejected | GATE-SPARK-20, KILL-SHARD | N36-09; BD-199 |
| CX-380 | mutation; `mean-of-shard-means` | the device-order gradient reducer | shard means averaged instead of a sum of sums divided by the global valid count | 0.5 instead of 0.25 for {1} and {0, 0, 0}; a division by zero or a halved answer with an empty shard | GATE-SPARK-20 | N36-08; BD-199 |
| CX-381 | mutation; `per-world-bytes-only` | the memory model of `sim.estimate` | the estimate counts per-world bytes only, omitting scenes and camera frames | two 640 × 480 cameras on 8,192 worlds are admitted where 37.5 GiB does not fit; `worlds_are_not_the_binding_term` fails | GATE-SPARK-22 | BD-219; N36-16 and N36-19 |
| CX-382 | planted-input; `unmeasured-bandwidth-ceiling` | the disclosure block and generated documentation | a bandwidth ceiling cited with no measured bytes-touched figure | it is printed without its HYPOTHESIS label | GATE-SPARK-22 | N36-18 |
| CX-383 | planted-input; `full-referee-coverage-claim` | the referee coverage record | a record claiming the referee checks every device world | it is accepted instead of flagged by `referee_cannot_cover_everything` | GATE-FID-07 | N36p1-02 at M1; N36-15 scores it at M2 |

**Registry check.** The test file loads the catalogue and fails if any of CX-374 to CX-383 is missing, if an
entry has no `plant` or no gate, or if a row of the compiled-in table has no test name, owner or `due`. A named test
whose owner bead is closed must exist in the workspace's test listing; a named test whose owner has not landed is
listed `not yet` with its owner and milestone (the pattern of N00-CX) and is neither run nor counted; from its
milestone on, a missing test fails the check. At M0 the tests that must exist are those of CX-374, -375, -376,
-377 and -378 (N36-03, N36-04, N36-05); the planted variants of CX-380 and CX-381, and the guards of CX-379, -382
and -383, arrive with their owners. A CX number is never reused or renumbered.

## Acceptance criteria

- [ ] `counterexamples.toml` holds ten entries, CX-374 to CX-383, in the schema of N38b-05 and no other field, each with the case's numbers, the rule, a plant and its gates; no private file `registry/counterexamples/perf.toml` exists.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control whose `seed` names it, with the gates of the control table; the second run changes no byte; the join passes; no entry is in the findings file.
- [ ] The registry check passes at M0 with the M1 and M2 tests listed `not yet`, and fails when an entry, a plant, a test name, or a named test that is due is removed.
- [ ] For the five rows whose shortcut is code (CX-374, -376, -378, -380, -381), the planted variant exists behind its plant feature under `gauntlet-mutations` and is rejected by its named test, each from its owner's milestone (CX-374, -376, -378 at M0; CX-380 and -381 at M2).
- [ ] For the five rows whose shortcut is an argument, the numbers are reproduced by this bead's test at M0 and the guard test exists from its owner's milestone.
- [ ] The printed catalogue of the chapter's counterexamples is generated from the catalogue file.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test counterexamples_perf` (real registry files and scratch copies, no mocks):
- `rows_374_to_383_present`; `every_entry_has_plant_and_gate`; `every_row_has_test_owner_and_due`;
  `named_tests_exist` (tests whose owners are closed); `not_yet_listed_until_due` (with the milestone set to M0
  the M1 and M2 tests are listed `not yet` and the check passes; with it set to M2 and one of them absent from a
  scratch test listing, the check fails); `schema_fields_only` (a scratch entry carrying `owner` is rejected by
  the loader); `gates_resolve` (every gate of the control table is in `rule_ids` and in the gate register).
- `numbers_reproduce` (one sub-test per row): 4.0 M against 2.0 M, and 1.20 M × 4 = 4.8 M; 1/(1 − 0.87) = 7.7,
  1/(1 − 0.29) = 1.4 and 1.15; 1,000/50 = 20 against 10 × 5 = 50; 266/13 = 20.46 and 35.6/13 = 2.74; 842.8/575.7 =
  1.46; 410/325 = 1.26; 0.25 against 0.5; 35 kB and 97 kB × 8,192 = 0.28 GB and 0.79 GB, and 37.5 GiB for the two
  cameras; an implied efficiency above 1 for the central assumption; 1.7 M × 2 ms = 3,400, 4.8/3,400 = 0.14% and
  24/3,400 = 0.71%, 550/3,400 = 16%.
- `planted_variants_are_rejected`: for each planted variant whose owner has landed, runs its named test in the
  form of ADR-GAUNT-01 with `--features gauntlet-mutations` and the plant's feature and expects failure (three at
  M0; all five from M2).
- `argument_guards`: the guard tests of the compiled-in table run on their fixtures: a ceiling-less experiment
  and a ceiling below its goal are refused by `perf::bound::planted_inputs_are_refused` (N36-05, M0); from their
  owners' milestones, a late two-card prediction is rejected by
  `perf::shard_ceiling::prediction_must_precede_measurement` (N36-09, M2), an unlabelled bandwidth ceiling by
  `perf::bandwidth::unmeasured_ceiling_keeps_hypothesis_label` (N36-18, M2), and a full-coverage claim by the
  `check_claim` of `referee_cannot_cover_everything` (N36p1-02, M1).

### End-to-end script and logging

`scripts/e2e/cx_perf.sh` (this bead; header `bead=N36-CX milestone=M0 labels=spark,registry
plants=drop-plant,private-field,digit-changed,missing-due-test`; sources the shared helper of STD-E2E;
`--milestone <m>`): runs the catalogue check; re-runs the seeding tool (`cargo test -p frob-conformance --test
seed_controls -- --ignored seed`) and the join (`cargo test -p frob-conformance --test seed_join`) on a scratch
copy of `registry/` and asserts by digest that a second seeding run is a no-op; plants, one at a time on scratch
copies, an entry with its plant removed, an entry carrying a private field, a case literal with one changed digit
(575.7 → 557.7) and a due test removed from a scratch test listing, and asserts each is rejected; then runs each
due planted variant against its named test (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier
from the gate record> -- <gate>`, without and with `--features gauntlet-mutations` and the plant's feature),
asserting that the test fails with the variant and passes without it. It calls no `frob` verb. Logged per STD-E2E
to `target/e2e-logs/cx_perf/<utc-timestamp>.ndjson` with a transcript: every line has `ts`, `script`, `bead`,
`step`, `event`; `cmd` lines with the exact command line, exit code and wall time; the evidence block (claim Known
answer, class Tested, determinism D0); one `assert` line per row with the CX number, the test names, `due`, the
result (`caught`, `passed`, or `not yet`) and the first differing value; the catalogue and scratch-controls digests
before and after the second seeding; a `summary` line with rows loaded, variants caught, tests not yet due and the
gates named. From M2, `scripts/e2e/perf_harness_gate19.sh` (N36-30) runs this script in its `preflight`, and the
scripts of the scaling and estimator gates log the rows they exercise (CX-379 and CX-380; CX-381 and CX-382) beside
their measurements.

### Known answers and negative controls

The ten rows are the content; each is both a known computation and the specification of a negative control.
Cross-references to known answers: CX-374 ↔ KA-806; CX-375 ↔ KA-818; CX-376 ↔ KA-811; CX-377 ↔ KA-810; CX-378 ↔
KA-812; CX-380 ↔ KA-813; CX-381 ↔ KA-814, KA-816, KA-817; CX-382 ↔ KA-822; CX-383 ↔ KA-821. Controls of the
registry itself: the four planted defects of the script. Determinism: all number reproductions are exact or
closed-form host arithmetic.

## Considerations

- Three of these cases (CX-375, CX-378 and the program-level entry on running a speed comparison before accounting for shared work) are the reason the program computes ceilings first; they are deliberately kept as separate rows because each breaks a different instinct.
- CX-379's numbers are published figures of an incumbent, to be re-read and then re-measured on `trj`; the row keeps the published pair as its fixture and the measured pair is printed beside it by the scaling bead.
- CX-382 is a counterexample to one of the plan's own estimates, kept on purpose: it is why bytes touched are measured.
- The learning chapter states the shard-gradient case as CX-253 (N23-CX, P-10); CX-380 is this chapter's statement of it. Each keeps its own entry and control against its own rule's gates (neither is ≡ in Appendix N), and one planted reducer fixture serves both, built once by N36-08.
- Five of the ten gates are attached by the plan's own ties rather than by the rule text of Appendix N; if the lead prefers that those entries wait in the findings file for a gate named in the rule, the fail-closed default is the findings file, and this bead's acceptance then counts five findings instead of zero.

## References

Appendix N.33 (CX-374 to CX-383), §36.2, §36.3 (rows 5, 6, 17), §36.4, §36.5, §36.6, §36.9 to §36.13, §38.11,
§39.8; ADR-PERF-02, ADR-PERF-05; INV-PERF-01; GATE-SPARK-19, -20, -21, -22; GATE-TENFOLD-09; GATE-FID-07;
KILL-SHARD; FROB-REF-866.
