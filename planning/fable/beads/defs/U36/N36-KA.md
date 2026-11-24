---
key: "N36-KA"
title: "registry: performance known answers KA-806 to KA-826 with truth, tolerance, tier, control and carrier"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "gauntlet", "frob-conformance", "registry", "test", "solid", "review-notes-open"]
parent: "WP-34"
deps: ["BD-025", "N00-KA", "N38a-12", "N38b-50", "N38b-51", "STD-E2E", "STD-TEST"]
needs: []
related: ["N36-REF", "N38b-27", "N38b-34", "N38b-58", "N38a-13", "N17a-04", "N17a-07", "N36-33", "N16-KA"]
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

In: the 21 rows as `[[ka]]` records of the suite's one known-answer file, in the row schema of N38a-12; one
derivation file per fixture with the stated truths; the tolerances set here, each with the N38b-27 rule it
follows; the controls added here as `control.text`; a completeness test; the host arithmetic for KA-808. Out:
the row schema, the loader, eligibility and the printout (N38a-12); the run of the rows and the per-site account
(N38a-13); the tests themselves (the carrier beads); turning `control.text` into NC records and filling
`control.nc` (N38b-05's seeding; this bead writes no row of `controls.toml`); the counterexample and refusal
registries (N36-CX, N36-REF); the settlement of the three reconciled values (KA-812: REC-01, N38b-50; KA-818:
REC-02, N38b-51; the 8,192-world clause of KA-816: REC-09, N38b-58, which depends on this bead); the controls of
the five rows left `none stated` here (KA-816, -819, -820, -822, -824), which cannot support a gate until one is
registered and must have one by M3, and second controls for the other rows: N38b-34 (M2, which depends on this
bead; a control registered here stays first). The tolerances set here follow the suite's rules (N38b-27): rule 1
(`exact`) for integers, rationals, verdicts and refusals; rule 2 for printed arithmetic of exact inputs (the exact
value is the truth and the printed value is checked as its rounding, half a unit in the last printed digit);
rule 3 (`gate`) where GATE-SPARK-22 states the tolerance.

## Technical approach

**Records.** One `[[ka]]` row per KA, appended to the suite's one file
`crates/frob-conformance/registry/known_answers.toml` in the row schema of N38a-12; no chapter file (the loader
refuses any other `known_answers*.toml`, or a `.toml` under a `known_answers` directory, with `ka-second-file`).
DESIGN, correcting an earlier draft of this bead that kept `registry/known_answers/perf.toml` with private
fields (`ka`, `negative_control`, `gate`, `carrier`). The columns of the table below map as: KA → `id`; the
"Where stated" list below → `section`; "Gate" → `owner` (empty for the rows with no gate, except KA-808, whose
bounds are the proof obligation PO-PHYLUM-02); "Case" → `case`; "Truth" → `truth`, stored as an exact rational
or integer wherever the arithmetic is exact (the printed decimal stays in the case text); "Tolerance" →
`tolerance = { kind, value, unit, reason }`, the `reason` naming its rule of N38b-27 (`exact` for rule 1;
`printed-digits` for rule 2, half a unit in the last printed digit; `gate` for rule 3); "Tier · site" → `tier`
and `sites`; "Negative control" → `control.text` (the appendix's control first, then the one added here;
`none stated` for the five rows of the Scope), with `control.nc` filled by N38b-05's seeding; "Carrier" → the
front matter of the row's derivation file (for each KA it serves: the carrier bead and the test that checks it)
and the runner binding `ka:<id>@<site>` of N38a-13, supplied by the carrier named first; plus `fixture`,
`same_as`, `derivation`, `label`, `due`, `printed` and `slug` as listed after the table. The row stores no status:
eligibility is computed by the loader. **Tolerance rule for rows the plan left "to set"** (set here, as TARGETs
recorded with the registry, by rule 2 of N38b-27): a computed value must equal the printed value when rounded to
the printed number of significant figures; rows marked *exact* compare integers, rationals or refusal codes
exactly (rule 1).

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Gate | Carrier |
|---|---|---|---|---|---|---|---|
| KA-806 | A report with 400 k env-steps/s of training at d = 10 beside 2.0 M world-steps/s of physics on the same card | Rejected by the ordering check (INV-PERF-01): 400 k env-steps/s at d = 10 is 4.0 M world-steps/s inside training, and training is at most the full environment step, which is at most physics only | exact | G1 · host | The same report with the decimation omitted must also be rejected | GATE-SPARK-19 | N36-03; BD-216; N36-30 |
| KA-807 | Paired lower bound on ten pairs, each with ratio exactly 1.5 | With ℓᵢ = ln(aᵢ/bᵢ) the bound is exp(ℓ̄ − t·s/√n), t = 1.833 for n = 10; here s = 0 and the bound is 1.5 | exact | G1 · host | An A/A campaign must not clear 1.02 (of 100 A/A campaigns at most 9 clear it) | GATE-SPARK-19 | N36-06; N36-30; N38a-08 (the estimator, `known_answer_all_equal`, from M1) |
| KA-808 | Padding overhead Ω_p = L_maxᵖ/E[Lᵖ] for degrees of freedom 18 and 35 mixed evenly | 1.32, 1.58, 1.76 for p = 1, 2, 3 (35/26.5 = 70/53; 1,225/774.5 = 2,450/1,549; 42,875/24,353.5 = 85,750/48,707) | printed precision (two decimals) | G1 · host | none stated; added here: Ω computed with the mean in place of the maximum (gives 1.0) must fail | — | This registry (host arithmetic, M0); N17a-04 (the Ω_p library, M2); BD-171 (the planner's envelope, M2) |
| KA-809 | Family-batch bound: speedup ≤ 1/(1 − f + f/r + h) | f = 0.97, r = 40, h = 0.02 give about 13.5; infinite acceleration of 85% of the time gives at most 6.7; with h = 0.02 no r reaches ten-fold unless f ≥ 0.92 | printed precision (three figures; two for 6.7 and 0.92) | G1 · host | A pruning ratio multiplied by a full-set speedup must be flagged | — | N36-05 |
| KA-810 | The bound applied to a generation of 256 designs at 16 worlds and 10 s each | 20.5 M world-steps, about 10 s of simulation at 2.0 M world-steps/s; padding taken as 30% of simulation time. Switch cost 1 s: f = 256/266 = 0.96 and the bound is 266/13 ≈ 20. Switch cost 0.1 s: f = 0.72 and the bound is 35.6/13 ≈ 2.7 | printed precision (two figures) | G1 · host | none stated; added here: a bound that omits the padding term must fail (on the same rounded inputs it gives 266/10 = 26.6 and 35.6/10 = 3.56, printed 27 and 3.6) | — | N36-05 |
| KA-811 | Pruning ratio and device speedup | 100 candidates of 10 s each; pruning removes 90; the ten survivors accelerate by 2, not by the 5 measured on the full set: 1,000/50 = 20 | exact | G1 · host | The product 10 × 5 = 50 | — | N36-05 |
| KA-812 | Ceiling-first rule | A speed experiment requested without its computed ceiling, or with a ceiling below its goal, is refused (`FROB-REF-866`); recorded pilot (FACT, to be re-run before a gate cites it): composed over specialized 1.345 warm and 1.088 cold; a residual of 575.7 µs that the solver does not touch, so a solver of zero cost would have reached about 1.46 (842.8 ÷ 575.7 = 8,428/5,757 = 1.46396; the settled value of REC-01) | exact (refusal); printed precision for the ceiling: 0.005 on 8,428/5,757 (REC-01) | G1 · host | none stated; added here: an admission check that only tests for the presence of a ceiling field must fail on goal 2.0 against ceiling 1.46 | GATE-SPARK-19 (as exercised by its script) | N36-05; N36-30 |
| KA-813 | Gradient of one sample at 1 and three at 0 on synchronous shards | 0.25 on one shard and on two, however the four are split, and with an empty shard: sums are added in device order and divided by the global valid count | exact | G5 · cuda | The mean of shard means: split as {1} and {0, 0, 0} it gives (1 + 0)/2 = 0.5; with an empty shard it divides by zero or halves the answer | GATE-SPARK-20 | N36-08; BD-199; N36-31 |
| KA-814 | Per-world byte model in `f32` for Q and H | Cache 4·(61·bodies + 17·n): 4,396 and 9,700. Mass matrix, Hessian and factor 12·n²: 3,888 and 14,700. Dense Jacobian 4·R·n: 11,520 and 44,800. Row records 48·R: 7,680 and 15,360. Contact records 96·C: 3,072 and 6,144. State and solver vectors about 4,000 and 6,000. Totals 34.6 kB and 96.7 kB before rounding (about 35 kB and 97 kB) | exact for the formula terms; the estimate within 5% of measured high-water marks | G1 · host | A capacity change must change the estimate by 4·n + 48 bytes per row (120 for Q, 188 for H); the mutation that freezes the Jacobian term at the default capacity fails it | GATE-SPARK-22 | BD-219 (the record's breakdown); N17a-07 (the per-world function: the ten exact products and the capacity deltas) |
| KA-815 | Row capacity of the reference workloads | Four rows per contact plus one limit row and one stiction or backlash row per actuated joint, rounded up: 128 + 12 + 12 gives 160 for Q; 256 + 29 + 29 gives 320 for H | exact | G1 · host | none stated; added here, under N36-02's rule (rounded up to a multiple of 8; Q's rule value 152 raised to 160 by the reference workload's registered `row_cap_override`): for H, the rounding step removed (314, which the rule check of the workload record must reject); for Q, the raise removed (`row_cap = 152` with no override, which satisfies the rule and must fail N36-02's `row_capacity_known_answer`, which pins the registered 160) | — | N36-02 (`row_cap`, M0); N36-33 (through `sim.run --workload`, M2) |
| KA-816 | Consequences of the byte model | 8,192 worlds of Q are 0.28 GB and of H 0.79 GB; PPO rollout storage 4·T·(observations + privileged observations + actions + 4) bytes per world is 24 kB at T = 24 with 48 observations, a 187-ray height scan and 12 actions; the hard-contact R × R operator is 102 kB per world of Q and 410 kB of H; 16 to 19 GB hold about 0.46 to 0.55 M worlds of Q or 0.17 to 0.20 M of H | printed precision (two figures); the integer products exact (24,096; 102,400; 409,600) | G1 · host | none stated | GATE-SPARK-22 | BD-219 |
| KA-817 | Camera frame memory | A 640 × 480 frame with 4 bytes of colour and 4 of depth is 2,457,600 bytes; two cameras on 256 worlds are 1,258,291,200 bytes (1.17 GiB); on 8,192 worlds 37.5 GiB; 16 GB hold at most about 3,200 two-camera worlds (exactly ⌊16·10⁹ / 4,915,200⌋ = 3,255, printed rounded down to two figures, the safe direction for a capacity); a 64 × 64 depth image in `f32` is 16,384 bytes, two per world on 8,192 worlds are 0.27 GB (268,435,456 bytes) | exact (integers, 3,255 among them); printed precision for 1.17 GiB, 37.5 GiB and 0.27 GB | G1 · host | none stated; added here: charging one frame set per device instead of per world must fail | GATE-SPARK-22 | N36-19 |
| KA-818 | Amdahl limits on incumbent stacks | Environment step at 87% of training wall-clock (94 k, 88 k and 82 k env-steps/s nested): at most 1/(1 − 0.87), about 7.7. Physics at about 29% of the environment step (347 k against 1.20 M env-steps/s): at most 1.4. Inference and learning at about 13%: at most 1.15. A step 1.5 times slower lengthens training by 1.15 to 1.44 for a step share of 0.29 to 0.87 | printed precision (two or three figures as printed): 100/13 with 0.05 for the first clause (REC-02) | G1 · host | none stated; added here: multiplying the step limit by the physics limit as independent factors must fail | — | N36-04; N36-24 |
| KA-819 | Compute per decision | Ratio 410/(c_s + f·c_t); ten-fold needs c_s + f·c_t ≤ 41 device-seconds; at c_t = 410: f ≤ 7.6% if c_s = 10, f ≤ 0.7% if c_s = 38, impossible if c_s ≥ 41 (at c_s = 41 exactly the only admissible fraction is f = 0, no design trained, which the carrier prints with that note; above 41 none exists) | printed precision; the threshold 41 exact | G1 · host | none stated | — | N36-04 |
| KA-820 | Design cycle and agent latency | A 120 s cycle against 360 to 420 s is 3.0 to 3.5, and against a 30-minute run about 15; with agent latency L of 60 to 120 s and 300 s of incumbent training per turn the gain is at most (300 + L)/L, between 3.5 and 6; deleting training and compilation from a cycle of about 5.9 minutes gives 6.6 | printed precision | G1 · host | none stated | — | N36-04 |
| KA-821 | Referee coverage | A device produces T·h world-seconds per second and a referee on c cores at real-time factor ρ covers c·ρ: for H, 1.7 M × 2 ms = 3,400, and 24 cores at 0.2 to 1.0 cover 4.8 to 24, that is 0.14 to 0.7%; for Q at the floor 0.12 to 0.6%; `batch-cpu` at 275 k world-steps/s is 550 world-seconds per second, 16% of 3,400 | printed precision | G1 · host | none stated; added here: dividing by T instead of T·h (500 times too small at 2 ms) must fail | — | N36p1-02 (the coverage function and its tests); N36-15 (outcomes on measured inputs) |
| KA-822 | Bandwidth budget arithmetic (inputs are a HYPOTHESIS until measured) | A batch outgrows a 72 MB L2 cache above about 2,100 worlds of Q or 750 of H; 4.0 M world-steps/s allows at most 126 to 202 kB per world-step at an efficiency of 0.5 to 0.8 on 1,008 GB/s; three passes over an 11.5 kB Jacobian in each of four Newton iterations are 138 kB; a quarter of 2.0 M world-steps/s at 240 kB per step needs 120 GB/s | printed precision; bytes touched per world-step are measured per launch | G1 · host | none stated | GATE-SPARK-22 | N36-18 |
| KA-823 | Compute envelope arithmetic | One device-hour at 50 to 500 k env-steps/s is 0.18 to 1.8·10⁹ env-steps; 1.6·10⁹ steps are 0.9 to 9 device-hours, 5·10⁹ are 2.8 to 28, 2·10¹² are 1,100 to 11,000, which is 23 to 230 days on two cards; a job above the class in force is refused before it starts (`FROB-REF-761`) | printed precision (two figures); exact for the refusal | G1 · host | none stated; set by the carrier and registered here first: an arithmetic that forgets the two cards (46 to 463 days) must fail `device_hour_arithmetic`; added here as a second control: admission on the point estimate instead of the interval's upper end must fail | — | N36-20 |
| KA-824 | Designs screened per device-second, derived from target rows 1 and 10 | At 16 worlds and 10 s per design (80,000 world-steps): 17 at the tolerances (2.0 M × 0.70 / 80,000 = 35/2, of which 17 whole designs) and 43 at the ambitions (4.0 M × 0.86 / 80,000 = 43) | exact: the rationals 35/2 and 43, and the printed 17 as the whole-design count ⌊35/2⌋ (DESIGN: a count of designs screened is printed as whole designs, so no rounding rule is involved) | G1 · host | none stated | — | N36-21 |
| KA-825 | Disclosure and admission refusals | A Performance figure without the complete disclosure block is refused (`FROB-REF-862`); a ratio without a paired lower bound is refused (`FROB-REF-863`) | exact | G1 · host | none stated; added here: a completeness check that treats an absent field as not applicable must fail | GATE-SPARK-19 | N36-13 (the `862` clause, M0); N36-06 (the `863` clause, M2); N36-30 |
| KA-826 | Memory and time estimator | `sim.estimate` memory within 5% of measured high-water marks at 1, 8, 64 and 256 worlds and at capacity for Q and H on each device; a job that does not fit is refused before allocation (`FROB-REF-602`); time estimates fall inside their intervals on at least 90% of held-out runs | as stated: `gate` (rule 3), GATE-SPARK-22's 5% and 0.90; the refusal exact | G1 · cuda, metal | none stated; added here: an estimator that sums all slabs instead of taking the live-interval peak, and an admission that allocates before checking | GATE-SPARK-22 | BD-219; N36-17; N36-32; N36p1-01 (the gate record on every device) |

**Where stated (→ `section`, from the appendix).** KA-806 §36.2, §36.16; KA-807 §36.6, §36.16; KA-808 §21.11,
§36.16; KA-809 §36.5, §36.16; KA-810 and KA-811 §36.5; KA-812 §36.5, §35.11; KA-813 §36.13, §36.16; KA-814
§36.10, §36.16; KA-815 §36.3; KA-816 §36.10; KA-817 §36.12, §36.16; KA-818 to KA-820 §36.4; KA-821 §36.9; KA-822
§36.11; KA-823 §36.14; KA-824 §36.15; KA-825 §36.7, §36.16; KA-826 §36.10, §36.16.

**Fixtures and derivation files (one name per physical case).** Six rows restate, with the same printed
constants, a case that an earlier chapter's row already holds; the appendix prints no "≡" mark on them, but its
legend says the marks are printed only where noticed and "the registry holds the complete map", and N38a-12
allows one fixture identity per physical case (a second fixture name for equal constants is one of its
negative controls). So these six take the earlier row's fixture and set `same_as` to it (DESIGN, reading the
legend; it corrects this bead's earlier draft, which left `same_as` empty on all 21): KA-809 → KA-014
(`thesis-ceilings`: f = 0.97, r = 40, h = 0.02 and f = 0.85 with r unbounded; KA-809 adds the clause f ≥ 0.92);
KA-812 → KA-001 (`solver-ceiling`: 1.345, 1.088, 575.7 µs and 842.8 µs; REC-01 already names both rows);
KA-818 → KA-010 (`amdahl-step-share`: the shares 0.87 and 0.29; REC-02 names both rows); KA-819 → KA-013
(`thesis-ceilings`: 410, 38 and 41 device-seconds; KA-819 adds c_s = 10); KA-820 → KA-011
(`thesis-ceilings`: the 120 s cycle against 360 to 420 s and 30 minutes, with KA-012's latency clause; KA-011 is
the earliest row of the fixture with these constants); KA-821 → KA-361 (`referee-coverage`, §16.11: 1.7 M ×
2 ms = 3,400 and 24 cores at 0.2 to 1.0; KA-821 adds the `batch-cpu` clause). The derivation file of a shared
fixture is the earlier row's; this bead appends its rows and their extra clauses to that file's front matter
and body, and writes no second file for the same case. The first five point to rows of N00-KA (a dependency,
M0) and N38b-50/51's files. KA-361's row is N16-KA's (M1, after this bead), so the sixth link is completed by
the bead that lands second: this bead gives KA-821 the fixture `referee-coverage` and writes
`docs/derivations/referee-coverage.md` with KA-821's clauses, and N16-KA gives KA-361 the same
fixture name, adds KA-361 to that file, and sets KA-821's `same_as = "KA-361"` in the same commit, so the
loader's rule that `same_as` names an existing earlier row holds at every commit. The registry's shared-fixture count therefore
rises from the 14 printed marks by these six, reported row by row as N38a-12's count check requires. The
other fifteen rows have fixtures of their own (DESIGN names): `perf-unit-ordering` (KA-806);
`paired-bound-ten-pairs` (KA-807); `padding-18-35-even` (KA-808: separate from the 90-to-10 card of KA-547,
by N38a-12's rule that two illustrative cards of one formula are two fixtures); `family-batch-generation`
(KA-810, KA-811: the 256-design generation and the 100-candidate pruning card, constants of this chapter
only); `shard-gradient-four` (KA-813); `reference-workloads-qh` (KA-814 to KA-816, KA-822 and KA-826: the Q
and H records of N36-02); `camera-frame-memory` (KA-817); `compute-envelope` (KA-823);
`designs-per-device-second` (KA-824); `disclosure-refusals` (KA-825). Each has `docs/derivations/<fixture>.md`
listing in its front matter the KA identifiers it serves, the carrier and the test for each. **Reconciled
clauses.** Three rows hold a value that the reconciliation register settles; the row carries the settled
truth and its derivation file is the settling record's file, so that the corrected number has one home:
KA-812's ceiling is REC-01 (N38b-50, a dependency: truth 8,428/5,757, tolerance 0.005,
`docs/derivations/solver-ceiling.md`; the withdrawn derivation 1.345 ÷ 0.94 = 1.43 lives in the record);
KA-818's first clause is REC-02 (N38b-51, a dependency: truth 100/13, tolerance 0.05,
`docs/derivations/amdahl-step-share.md`; the first-printed 7.8 lives in the record); KA-816's 8,192-world
clause is REC-09 (N38b-58, which depends on this bead and confirms 0.28 GB and 0.79 GB within 0.005 GB, citing
`docs/derivations/world-bytes-share.md` from KA-816's own derivation file). The appendix values of these three
rows already agree with the settlements (1.46, about 7.7, 0.28 and 0.79 GB), so `printed` is empty on all 21.

**Labels.** FACT, with the source and the note "to be re-run or re-read before a gate cites it", for the rows
whose truth is arithmetic on quoted inputs: KA-812 (the recorded solver pilot, Appendix K; its re-run is BD-043),
KA-818 to KA-820 (published incumbent figures). By rule 5 of N38b-27 these pass as arithmetic under the rule-2
tolerance, and the FACT condition is printed beside the result. DESIGN for the other 18 (arithmetic on design
inputs; KA-822's inputs are a HYPOTHESIS until N36-18 measures bytes touched, KA-826 is a measurement against
the gate's tolerance).

**Due (`due`: the milestone at which every clause of the row is bound by a carrier).** M0 (11 rows): KA-806
(N36-03), KA-808 (this bead), KA-809 to KA-812 (N36-05), KA-815 (N36-02), KA-818 to KA-820 (N36-04), KA-823
(N36-20). M1 (1): KA-821 (N36p1-02). M2 (9): KA-807 (N36-06 and the A/A control of N36-30), KA-813 (N36-08),
KA-814 (BD-219, N17a-07), KA-816 (BD-219), KA-817 (N36-19), KA-822 (N36-18), KA-824 (N36-21), KA-825 (its `863`
clause is N36-06's; the `862` clause is asserted by N36-13's own test from M0), KA-826 (BD-219, N36-17). DESIGN
reason: GATE-GAUNT-02 counts a row as passing only when its whole truth is checked, so a row is not due before
its last clause has a bound test; an earlier clause is still asserted by its carrier's unit test from that
carrier's milestone.

**Host arithmetic carried by this bead (KA-808).** `omega_p(sizes, weights, p) = L_maxᵖ / Σ wᵢ·Lᵢᵖ`, with the
bounds 1/r ≤ Ω_p ≤ r⁻ᵖ for r = E[L]/L_max and 1 ≤ p ≤ 3. For sizes {18, 35} with equal weights: r = 26.5/35 =
0.757; Ω₁ = 1.3208, Ω₂ = 1.5817, Ω₃ = 1.7605; bounds 1.32 and 2.30.

**Completeness check.** A test loads the registry through the test kit's one parser (STD-TEST) and fails if any
of KA-806 to KA-826 is missing, if a field of the N38a-12 schema is empty, if a row's derivation file does not
name a carrier and its test, if a row's tolerance is still `to-set` or its `reason` names no rule of N38b-27, if
its derivation file is absent or does not list the row, if a reconciled clause's truth differs from the settled
value of its record (REC-01, REC-02), or if a row marked with a gate names a gate that does not list it. A KA
number is never reused and never renumbered; new rows append.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds the 21 rows KA-806 to KA-826 with every field
      of N38a-12's schema and validates under its loader; each row has its derivation file; no other
      known-answer file exists (`ka-second-file`).
- [ ] No row's tolerance is `to-set`; every tolerance names its rule of N38b-27 in `reason`; the rule above is
      recorded as the TARGET for the rows the plan left open.
- [ ] Every row has at least one carrier bead and the carrier's test name in its derivation file; the ten rows whose negative control the plan left as "none stated" and for which a plausible wrong implementation exists (KA-808, -810, -812, -815, -817, -818, -821, -823, -825, -826) carry the control added here (for KA-823 the carrier's two-cards control first, then this bead's).
- [ ] KA-812 carries 8,428/5,757 with tolerance 0.005 and KA-818's first clause 100/13 with tolerance 0.05, as
      their reconciliation records settle; both are labelled FACT with their sources.
- [ ] The six rows that restate an earlier chapter's case (KA-809, -812, -818, -819, -820, -821) share that
      row's fixture and derivation file through `same_as` (KA-821's link completed in N16-KA's commit, which
      adds KA-361), and the change in the registry's shared-fixture count (14 printed marks, plus these six) is
      reported row by row.
- [ ] The `due` fields give 11 rows at M0, 1 at M1 and 9 at M2, as listed; at the M0 exit the 11 rows due are
      bound and pass, and the other 10 are reported `not yet due` with their carriers.
- [ ] `omega_p` reproduces 1.32, 1.58 and 1.76 and its bounds, and the planted mean-for-maximum variant fails.
- [ ] The completeness check passes, and fails when a row or a derivation file is removed.
- [ ] Appendix-style printouts of the chapter's known answers are generated from the one file (N38a-12's
      printout test), not edited by hand.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry::known_answers::perf`:
- `rows_806_to_826_present`; `no_to_set_tolerances`; `every_row_has_carrier_and_derivation`;
  `schema_fields_complete` (every N38a-12 field present on all 21 rows; `slug` and `printed` empty).
- `shared_fixtures_point_back`: KA-809, KA-812, KA-818, KA-819 and KA-820 carry `same_as` KA-014, KA-001,
  KA-010, KA-013 and KA-011 with those rows' fixture names; KA-821 carries the fixture `referee-coverage` (its
  `same_as` to KA-361 is asserted from the commit that adds KA-361); the other fifteen carry none; each shared
  derivation file lists the row; a planted second fixture name for KA-812 (constants equal to KA-001's) fails
  the loader.
- `tolerance_reasons_name_rules`: every row's `reason` names rule 1, 2 or 3 of N38b-27; a planted reason "looks
  right" fails this test (N38b-27's own meta-test applies the same check suite-wide from M1).
- `omega_p_known_answer` (KA-808): 35/26.5 = 70/53 = 1.3208; 1,225/774.5 = 2,450/1,549 = 1.5817; 42,875/24,353.5 =
  85,750/48,707 = 1.7605, computed in exact rationals.
- `omega_p_bounds`: 1/r = 1.3208 ≤ Ω_p ≤ r⁻ᵖ (1.3208, 1.744, 2.304).
- `omega_p_mean_for_max_is_caught`.
- `printed_precision_rule`: 13.468 matches "13.5"; 13.44 does not; 8,428/5,757 = 1.46396 matches "1.46" at 0.005;
  100/13 = 7.6923 matches "7.7" at 0.05 and not "7.8".
- `whole_designs_exact` (KA-824): 2.0·10⁶ × 0.70 / 80,000 = 35/2 and ⌊35/2⌋ = 17; 4.0·10⁶ × 0.86 / 80,000 = 43.
- `capacity_rounded_down` (KA-817): ⌊16·10⁹ / 4,915,200⌋ = 3,255, printed "about 3,200" as the value rounded
  down to two figures; 3,255 rounded to nearest (3,300) is not the printed form.
- `reconciled_truths_match_records`: KA-812 = 8,428/5,757 and KA-818's first clause = 100/13, read from
  `reconciliations.toml`; a planted truth 1.43 on KA-812 fails.
- `due_counts_11_1_9`: the due fields give 11, 1 and 9 rows at M0, M1 and M2.
- `gate_cross_reference`: rows naming GATE-SPARK-19, -20 or -22 are listed by those gates' registry rows.

### End-to-end script and logging

`scripts/e2e/ka_perf.sh` (this bead, M0; sources the shared helper of STD-E2E; `--milestone <Mn>`, default the
current milestone) runs, in order: the unit tests above (`FROB_TEST_SEED=0 cargo test -p frob-conformance
registry::known_answers::perf`); the G1 run of the chapter's rows due by the milestone with their controls
(`FROB_TEST_SEED=0 FROB_MILESTONE=<Mn> cargo test -p frob-conformance --features gauntlet-mutations --test
g1_known_answers -- ch:36`, the selection of N38a-13); from M2, KA-813's device lane (`FROB_TEST_SEED=0 cargo
test -p frob-conformance --features cuda,gauntlet-mutations --test g5_determinism -- KA-813`, logged
`skipped: <site unavailable>` on a host with no CUDA device, never as a pass); and the printout regeneration
(`cargo test -p frob-conformance --test known_answers_printout`). It logs one `assert` line per row with the KA
number, the computed value (for the host rows this bead or a due carrier computes), the truth, the printed
value, the tolerance and its rule, the carrier and whether a control is registered and was caught; rows due
later are logged `not yet due` with their carrier and milestone; a `summary` line gives rows loaded, rows due,
rows passing, rows without a control and the gates named. From M2, `scripts/e2e/perf_harness_gate19.sh` (N36-30)
runs this script as its first step, and a missing row fails it before any measurement starts.

### Known answers and negative controls

All 21 rows are the content. Controls added by this registry are listed in the table and are registered in the
negative-control registry under the gate of their row, or under the carrier bead's own test where the row has no
gate. Determinism: every row at `host` is exact or closed-form and gives identical results twice on one machine,
in debug and release, and on both platforms; KA-813 is the one row with a device lane.

## Considerations

- Several rows quote published incumbent figures (94 k, 88 k, 82 k; 347 k against 1.20 M; 410 s); the known answer is the arithmetic on those inputs, not the inputs. When the incumbent baseline replaces them with measurements, the rows keep their published inputs as fixtures and the reports print measured values beside them.
- KA-822's inputs are a HYPOTHESIS until bytes touched are measured; the row tests the arithmetic and the bead that measures prints the verdict.
- KA-824 prints 17 for 17.5; the registry records the exact value 35/2 and the whole-design count 17, so the test is not fooled by rounding direction. KA-817's "at most about 3,200" is likewise a capacity rounded down (3,255 exactly), stated in its derivation file.
- The controls added here are owners' controls in the sense of N38b-34 (registered first); N38b-34's proposals for the same rows become linked records where the defect is the same (KA-810, KA-821) and second records otherwise.
- KA-826 is the only row that is a measurement tolerance and not arithmetic; it is satisfied by a gate record (written by N36-32's gate test, recorded on every device by N36p1-01), not by a unit test.

## References

Appendix M (the rows of the performance program), Appendix M.38 (rows 1, 2, 9), §36.16, §38.6; REC-01, REC-02,
REC-09; GATE-SPARK-19, GATE-SPARK-20, GATE-SPARK-22;
INV-PERF-01; FROB-REF-602, -761, -862, -863, -866.
