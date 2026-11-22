---
key: "N36-KA"
title: "registry: performance known answers KA-806 to KA-826 with truth, tolerance, tier, control and carrier"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-34"
deps: ["BD-025", "STD-TEST"]
needs: ["§17.5 and §21.11: the bucket planner that computes the padding overhead Ω_p for a population (seed BD-171), the mechanism carrier of KA-808; this registry carries the row's host arithmetic itself", "§38.6: the known-answer registry format and one derivation file per row under `docs/derivations/` (the Gauntlet's unit)"]
related: ["N36-CX", "N36-REF", "N36-01", "N36-02", "N36-03", "N36-04", "N36-05", "N36-06", "N36-08", "N36-13", "N36-15", "N36-17", "N36-18", "N36-19", "N36-20", "N36-21", "BD-216", "BD-219", "BD-199", "BD-171"]
gates: []
plan_refs: ["Appendix M.35", "§36.16", "§36.2", "§36.3", "§36.4", "§36.5", "§36.6", "§36.9", "§36.10", "§36.11", "§36.12", "§36.13", "§36.14", "§36.15", "§38.6"]
---
## Background

A known answer is a case whose truth is known by hand, in closed form or by exact arithmetic, checked against a
derivation on file and never regenerated from the candidate. The verification program runs known answers at tier
G1, with some at G5 (determinism lanes on devices). The plan's rule is that a gate that nothing can fail validates
nothing, so every row with a plausible wrong implementation names a **negative control**: a planted defect the
test must catch.

The performance program is unusual among the chapters: most of its known answers are arithmetic about
measurement itself (units, ratios, ceilings, bytes), because its job is to keep speed figures honest. It has 21
known answers, KA-806 to KA-826. This bead enters all of them into the known-answer registry of
`frob-conformance` as data, sets the tolerances the plan left "to set", names the bead that carries each test, and
adds a completeness check, so that each mechanism bead can cite a row by number and no row is lost. It implements
no mechanism.

Terms used in the table. *World-step*: one physics step of one world. *Env-step*: one control tick, d world-steps
(d the decimation). *Q* and *H*: the reference quadruped workload (18 degrees of freedom, 13 bodies, contact
capacity C = 32, row capacity R = 160) and humanoid workload (35, 30, C = 64, R = 320), both at step 2 ms and
d = 10. *Paired lower bound*: the one-sided 95% lower confidence bound of a ratio over at least ten interleaved
pairs. *Sites*: `host` (a CPU test in exact or double-precision arithmetic), `cuda`, `metal`.

## Scope

In: the 21 rows as registry entries with fields `ka`, `case`, `truth`, `tolerance`, `tier`, `sites`,
`negative_control`, `gate`, `carrier`; the tolerances set here; a completeness test; the host arithmetic for
KA-808. Out: the tests themselves (the carrier beads); the counterexample and refusal registries (N36-CX,
N36-REF).

## Technical approach

Entries live in `crates/frob-conformance/registry/known_answers/perf.toml`, one table per row, each with a
derivation file `docs/derivations/KA-8nn.md` that shows the arithmetic. **Tolerance rule for rows the plan left
"to set"** (set here, as TARGETs recorded with the registry): a computed value must equal the printed value when
rounded to the printed number of significant figures; rows marked *exact* compare integers, rationals or refusal
codes exactly.

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Gate | Carrier |
|---|---|---|---|---|---|---|---|
| KA-806 | A report with 400 k env-steps/s of training at d = 10 beside 2.0 M world-steps/s of physics on the same card | Rejected by the ordering check (INV-PERF-01): 400 k env-steps/s at d = 10 is 4.0 M world-steps/s inside training, and training is at most the full environment step, which is at most physics only | exact | G1 · host | The same report with the decimation omitted must also be rejected | GATE-SPARK-19 | N36-03; BD-216; N36-30 |
| KA-807 | Paired lower bound on ten pairs, each with ratio exactly 1.5 | With ℓᵢ = ln(aᵢ/bᵢ) the bound is exp(ℓ̄ − t·s/√n), t = 1.833 for n = 10; here s = 0 and the bound is 1.5 | exact | G1 · host | An A/A campaign must not clear 1.02 (of 100 A/A campaigns at most 9 clear it) | GATE-SPARK-19 | N36-06; N36-30 |
| KA-808 | Padding overhead Ω_p = L_maxᵖ/E[Lᵖ] for degrees of freedom 18 and 35 mixed evenly | 1.32, 1.58, 1.76 for p = 1, 2, 3 (35/26.5; 1,225/774.5; 42,875/24,353.5) | printed precision (two decimals) | G1 · host | none stated; added here: Ω computed with the mean in place of the maximum (gives 1.0) must fail | — | This registry (host arithmetic); BD-171 (planner) |
| KA-809 | Family-batch bound: speedup ≤ 1/(1 − f + f/r + h) | f = 0.97, r = 40, h = 0.02 give about 13.5; infinite acceleration of 85% of the time gives at most 6.7; with h = 0.02 no r reaches ten-fold unless f ≥ 0.92 | printed precision (three figures; two for 6.7 and 0.92) | G1 · host | A pruning ratio multiplied by a full-set speedup must be flagged | — | N36-05 |
| KA-810 | The bound applied to a generation of 256 designs at 16 worlds and 10 s each | 20.5 M world-steps, about 10 s of simulation at 2.0 M world-steps/s; padding taken as 30% of simulation time. Switch cost 1 s: f = 256/266 = 0.96 and the bound is 266/13 ≈ 20. Switch cost 0.1 s: f = 0.72 and the bound is 35.6/13 ≈ 2.7 | printed precision (two figures) | G1 · host | none stated; added here: a bound that omits the padding term (gives 26 and 3.5) must fail | — | N36-05 |
| KA-811 | Pruning ratio and device speedup | 100 candidates of 10 s each; pruning removes 90; the ten survivors accelerate by 2, not by the 5 measured on the full set: 1,000/50 = 20 | exact | G1 · host | The product 10 × 5 = 50 | — | N36-05 |
| KA-812 | Ceiling-first rule | A speed experiment requested without its computed ceiling, or with a ceiling below its goal, is refused (`FROB-REF-866`); recorded pilot: composed over specialized 1.345 warm and 1.088 cold; a residual of 575.7 µs that the solver does not touch, so a solver of zero cost would have reached about 1.46 (842.8 ÷ 575.7) | exact (refusal); printed precision (1.46) | G1 · host | none stated; added here: an admission check that only tests for the presence of a ceiling field must fail on goal 2.0 against ceiling 1.46 | GATE-SPARK-19 (as exercised by its script) | N36-05; N36-30 |
| KA-813 | Gradient of one sample at 1 and three at 0 on synchronous shards | 0.25 on one shard and on two, however the four are split, and with an empty shard: sums are added in device order and divided by the global valid count | exact | G5 · cuda | The mean of shard means: split as {1} and {0, 0, 0} it gives (1 + 0)/2 = 0.5; with an empty shard it divides by zero or halves the answer | GATE-SPARK-20 | N36-08; BD-199; N36-31 |
| KA-814 | Per-world byte model in `f32` for Q and H | Cache 4·(61·bodies + 17·n): 4,396 and 9,700. Mass matrix, Hessian and factor 12·n²: 3,888 and 14,700. Dense Jacobian 4·R·n: 11,520 and 44,800. Row records 48·R: 7,680 and 15,360. Contact records 96·C: 3,072 and 6,144. State and solver vectors about 4,000 and 6,000. Totals 34.6 kB and 96.7 kB before rounding (about 35 kB and 97 kB) | exact for the formula terms; the estimate within 5% of measured high-water marks | G1 · host | A capacity change must change the estimate by 4·n + 48 bytes per row (120 for Q, 188 for H) | GATE-SPARK-22 | BD-219 |
| KA-815 | Row capacity of the reference workloads | Four rows per contact plus one limit row and one stiction or backlash row per actuated joint, rounded up: 128 + 12 + 12 gives 160 for Q; 256 + 29 + 29 gives 320 for H | exact | G1 · host | none stated; added here: a record with 152 for Q must fail the rule check | — | N36-02 |
| KA-816 | Consequences of the byte model | 8,192 worlds of Q are 0.28 GB and of H 0.79 GB; PPO rollout storage 4·T·(observations + privileged observations + actions + 4) bytes per world is 24 kB at T = 24 with 48 observations, a 187-ray height scan and 12 actions; the hard-contact R × R operator is 102 kB per world of Q and 410 kB of H; 16 to 19 GB hold about 0.46 to 0.55 M worlds of Q or 0.17 to 0.20 M of H | printed precision (two figures); the integer products exact (24,096; 102,400; 409,600) | G1 · host | none stated | GATE-SPARK-22 | BD-219 |
| KA-817 | Camera frame memory | A 640 × 480 frame with 4 bytes of colour and 4 of depth is 2,457,600 bytes; two cameras on 256 worlds are 1,258,291,200 bytes (1.17 GiB); on 8,192 worlds 37.5 GiB; 16 GB hold at most about 3,200 two-camera worlds; a 64 × 64 depth image in `f32` is 16,384 bytes, two per world on 8,192 worlds are 0.27 GB | exact (integers); printed precision for the rounded figures | G1 · host | none stated; added here: charging one frame set per device instead of per world must fail | GATE-SPARK-22 | N36-19 |
| KA-818 | Amdahl limits on incumbent stacks | Environment step at 87% of training wall-clock (94 k, 88 k and 82 k env-steps/s nested): at most 1/(1 − 0.87), about 7.7. Physics at about 29% of the environment step (347 k against 1.20 M env-steps/s): at most 1.4. Inference and learning at about 13%: at most 1.15. A step 1.5 times slower lengthens training by 1.15 to 1.44 for a step share of 0.29 to 0.87 | printed precision (two or three figures as printed) | G1 · host | none stated; added here: multiplying the step limit by the physics limit as independent factors must fail | — | N36-04; N36-24 |
| KA-819 | Compute per decision | Ratio 410/(c_s + f·c_t); ten-fold needs c_s + f·c_t ≤ 41 device-seconds; at c_t = 410: f ≤ 7.6% if c_s = 10, f ≤ 0.7% if c_s = 38, impossible if c_s ≥ 41 | printed precision | G1 · host | none stated | — | N36-04 |
| KA-820 | Design cycle and agent latency | A 120 s cycle against 360 to 420 s is 3.0 to 3.5, and against a 30-minute run about 15; with agent latency L of 60 to 120 s and 300 s of incumbent training per turn the gain is at most (300 + L)/L, between 3.5 and 6; deleting training and compilation from a cycle of about 5.9 minutes gives 6.6 | printed precision | G1 · host | none stated | — | N36-04 |
| KA-821 | Referee coverage | A device produces T·h world-seconds per second and a referee on c cores at real-time factor ρ covers c·ρ: for H, 1.7 M × 2 ms = 3,400, and 24 cores at 0.2 to 1.0 cover 4.8 to 24, that is 0.14 to 0.7%; for Q at the floor 0.12 to 0.6%; `batch-cpu` at 275 k world-steps/s is 550 world-seconds per second, 16% of 3,400 | printed precision | G1 · host | none stated; added here: dividing by T instead of T·h (500 times too small at 2 ms) must fail | — | N36-15 |
| KA-822 | Bandwidth budget arithmetic (inputs are a HYPOTHESIS until measured) | A batch outgrows a 72 MB L2 cache above about 2,100 worlds of Q or 750 of H; 4.0 M world-steps/s allows at most 126 to 202 kB per world-step at an efficiency of 0.5 to 0.8 on 1,008 GB/s; three passes over an 11.5 kB Jacobian in each of four Newton iterations are 138 kB; a quarter of 2.0 M world-steps/s at 240 kB per step needs 120 GB/s | printed precision; bytes touched per world-step are measured per launch | G1 · host | none stated | GATE-SPARK-22 | N36-18 |
| KA-823 | Compute envelope arithmetic | One device-hour at 50 to 500 k env-steps/s is 0.18 to 1.8·10⁹ env-steps; 1.6·10⁹ steps are 0.9 to 9 device-hours, 5·10⁹ are 2.8 to 28, 2·10¹² are 1,100 to 11,000, which is 23 to 230 days on two cards; a job above the class in force is refused before it starts (`FROB-REF-761`) | printed precision; exact for the refusal | G1 · host | none stated; added here: admission on the point estimate instead of the interval's upper end must fail | — | N36-20 |
| KA-824 | Designs screened per device-second, derived from target rows 1 and 10 | At 16 worlds and 10 s per design (80,000 world-steps): 17 at the tolerances (2.0 M × 0.70) and 43 at the ambitions (4.0 M × 0.86) | printed precision (17.5 is printed 17) | G1 · host | none stated | — | N36-21 |
| KA-825 | Disclosure and admission refusals | A Performance figure without the complete disclosure block is refused (`FROB-REF-862`); a ratio without a paired lower bound is refused (`FROB-REF-863`) | exact | G1 · host | none stated; added here: a completeness check that treats an absent field as not applicable must fail | GATE-SPARK-19 | N36-13; N36-06; N36-30 |
| KA-826 | Memory and time estimator | `sim.estimate` memory within 5% of measured high-water marks at 1, 8, 64 and 256 worlds and at capacity for Q and H on each device; a job that does not fit is refused before allocation (`FROB-REF-602`); time estimates fall inside their intervals on at least 90% of held-out runs | as stated | G1 · cuda, metal | none stated; added here: an estimator that sums all slabs instead of taking the live-interval peak, and an admission that allocates before checking | GATE-SPARK-22 | BD-219; N36-17; N36-32 |

**Host arithmetic carried by this bead (KA-808).** `omega_p(sizes, weights, p) = L_maxᵖ / Σ wᵢ·Lᵢᵖ`, with the
bounds 1/r ≤ Ω_p ≤ r⁻ᵖ for r = E[L]/L_max and 1 ≤ p ≤ 3. For sizes {18, 35} with equal weights: r = 26.5/35 =
0.757; Ω₁ = 1.3208, Ω₂ = 1.5817, Ω₃ = 1.7605; bounds 1.32 and 2.30.

**Completeness check.** A test loads the registry and fails if any of KA-806 to KA-826 is missing, if a row has no
carrier, if a row's tolerance is still `to-set`, if its derivation file is absent, or if a row marked with a gate
names a gate that does not list it. A KA number is never reused and never renumbered; new rows append.

## Acceptance criteria

- [ ] `known_answers/perf.toml` holds 21 rows, KA-806 to KA-826, each with every field and a derivation file.
- [ ] No row's tolerance is `to-set`; the rule above is recorded as the TARGET for the rows the plan left open.
- [ ] Every row has at least one carrier bead and the carrier's test name; the ten rows whose negative control the plan left as "none stated" and for which a plausible wrong implementation exists (KA-808, -810, -812, -815, -817, -818, -821, -823, -825, -826) carry the control added here.
- [ ] `omega_p` reproduces 1.32, 1.58 and 1.76 and its bounds, and the planted mean-for-maximum variant fails.
- [ ] The completeness check passes, and fails when a row or a derivation file is removed.
- [ ] Appendix-style printouts of the chapter's known answers are generated from this file, not edited by hand.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry::known_answers::perf`:
- `rows_806_to_826_present`; `no_to_set_tolerances`; `every_row_has_carrier_and_derivation`.
- `omega_p_known_answer` (KA-808): 35/26.5 = 1.3208; 1,225/774.5 = 1.5817; 42,875/24,353.5 = 1.7605.
- `omega_p_bounds`: 1/r = 1.3208 ≤ Ω_p ≤ r⁻ᵖ (1.3208, 1.744, 2.304).
- `omega_p_mean_for_max_is_caught`.
- `printed_precision_rule`: 13.468 matches "13.5"; 13.44 does not; 17.5 against the printed "17" is accepted only through the row's explicit note.
- `gate_cross_reference`: rows naming GATE-SPARK-19, -20 or -22 are listed by those gates' registry rows.

### End-to-end script and logging

`scripts/e2e/perf_harness_gate19.sh` (N36-30) runs `cargo test -p frob-conformance --test g1 -- known_answers::perf`
as its first step and logs one `assert` line per row with the KA number, the computed value, the printed value
and the tolerance; a missing row fails the script before any measurement starts.

### Known answers and negative controls

All 21 rows are the content. Controls added by this registry are listed in the table and are registered in the
negative-control registry under the gate of their row, or under the carrier bead's own test where the row has no
gate. Determinism: every row at `host` is exact or closed-form and gives identical results twice on one machine,
in debug and release, and on both platforms; KA-813 is the one row with a device lane.

## Considerations

- Several rows quote published incumbent figures (94 k, 88 k, 82 k; 347 k against 1.20 M; 410 s); the known answer is the arithmetic on those inputs, not the inputs. When the incumbent baseline replaces them with measurements, the rows keep their published inputs as fixtures and the reports print measured values beside them.
- KA-822's inputs are a HYPOTHESIS until bytes touched are measured; the row tests the arithmetic and the bead that measures prints the verdict.
- KA-824 prints 17 for 17.5; the registry records the exact value 17.5 and the printed form, so the test is not fooled by rounding direction.
- KA-826 is the only row that is a measurement tolerance and not arithmetic; it is satisfied by a gate record, not by a unit test.

## References

Appendix M (the rows of the performance program), §36.16, §38.6; GATE-SPARK-19, GATE-SPARK-20, GATE-SPARK-22;
INV-PERF-01; FROB-REF-602, -761, -862, -863, -866.
