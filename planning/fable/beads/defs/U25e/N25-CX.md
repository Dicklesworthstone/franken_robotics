---
key: "N25-CX"
title: "Augur counterexample registry: CX-270 to CX-287, and the eleven late counterexamples awaiting numbers"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "augur", "frob-conformance", "solid", "registry", "test"]
parent: "WP-42"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the negative-control registry and the counterexample catalogue seeds in frob-conformance (BD-146)", "Appendix N: catalogue numbers for the eleven late counterexamples listed in the second table (the Gauntlet unit that owns the catalogue)"]
related: ["N25-REF", "N25e-45", "BD-136", "BD-146", "BD-294"]
gates: []
plan_refs: ["Appendix N.22", "§25.4.7", "§25.22", "§25.29", "§25.30", "§25.31", "§25.32", "§25.33", "§25.34", "§25.35", "§25.36", "§25.37", "§25.38", "§25.39", "§0.7"]
---
## Background

Counterexamples are first-class in this program: where a plausible shortcut is wrong, the smallest hand-checkable
case that breaks it is written down, and the case enters the negative-control registry and the counterexample
catalogue. Each seeds a negative control: a test in which the shortcut is planted and must be caught. A rule that
ships without such a case has not been thought through.

Augur, the subsystem of mission queries and inverse design, answers questions about sets of designs with an inner
set and an outer set and a checked reason per cell. Its shortcuts are ways of making a family answer say more than
was proved: a convex hull through two witnesses, a per-design command read as a family command, a failed sample
read as an excluded cell, a marginal rate multiplied, a probe trusted outside its mechanism set. The catalogue
gives Augur eighteen numbered entries, CX-270 to CX-287. Eleven further counterexamples are stated by the rules
added to the chapter later; they have no catalogue number yet and are registered here under provisional tags so
that none is lost. Tier Solid. Mechanism beads cite the numbers; this bead holds the rows as data and binds each to
the control test that realizes it.

## Scope

In: the eighteen rows in `registries/counterexamples/augur.toml`; the eleven late rows in the same file under
provisional tags `late-01` to `late-11` until numbers are assigned; the binding of each row to a control test; the
registry check. Out: the control tests' implementations (in the beads named); other chapters' counterexamples.

## Technical approach

Row schema: `id`, `shortcut`, `case` (with its numbers), `rule`, `defined_in`, `control` (bead and test name),
`equivalent_to` (another catalogue entry stating the same case, where one exists). The registry check
(`xtask check-contracts`) verifies that every row names an existing control test and that every control test
tagged with a counterexample identifier names an existing row.

Numbered rows:

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control realized by |
|---|---|---|---|---|
| CX-270 | The midpoint of two feasible designs is feasible | Feet at (±1 − e, ±1) m, each capped at 0.5 N, load W at the origin; the positive-x pair carries W·(1 + e)/2. (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N and pass; the midpoint (0.45, 1.45) needs 841/800 N from a pair capped at 1 N | Convexity is claimed only in a chart where the constraints are affine with design-independent mission vertices (equivalent to CX-010) | BD-130, BD-136 |
| CX-271 | Variants that are pairwise compatible can share a controller | Commands (x, y) ∈ [0, 1]²; three variants need x ≥ 2/3, y ≥ 2/3 and x + y ≤ 1. Every pair has a common command; the triple needs x + y ≥ 4/3 and x + y ≤ 1 | Minimal conflict sets, not pairs | BD-363 |
| CX-272 | A command exists for each design, so one exists for the family | Pusher y = max(0, g·u − d), g ∈ [1/2, 2], d ∈ [0, 1/4], target [3/4, 5/4]: the per-build command set is all of [1/2, 2]; the shared command set is empty | The quantifier prefix is explicit in the query and in the certificate (`FROB-REF-826`, `FROB-REF-881`) | BD-131, BD-136 |
| CX-273 | A feasible allocation exists, so the body is admitted | Stance at half-width 1/4 m: the active allocation (155, 135, 105, 5) N fits 160 N ratings; an equal-spring passive suspension produces (175, 115, 85, 25) N | The authority that realizes the allocation, and the observations that arrive in time, are part of the certificate | BD-130, N25e-28 |
| CX-274 | The support polygon contains the load and total capacity suffices | Of 2,304 bodies in the stance grid, this screen accepts 1,596 that the exact model rejects | Per-foot ratings and moment balance together | BD-130 |
| CX-275 | Sample points in a cell fail, so the cell is excluded | Any cell that straddles a boundary: the 9 boundary cells of the 561-cell docking grid | A certificate valid over the cell; otherwise the cell is unresolved | BD-129, BD-358, N25e-07 |
| CX-276 | Both stages succeed 95% of the time, so the mission succeeds 90.25% | Twenty worlds, each stage failing in one: 18/20 if the failures differ, 19/20 if they coincide | Joint law or additive budgets (`FROB-REF-846`) | BD-128, BD-136 |
| CX-277 | Each projection of a product covers its marginal outcomes, so the dominance check may enclose them separately | Paired outcomes (0, 0) and (1, 1) project onto {0, 1} for both robots and hide the unreachable (1, 0) | Enclose the joint outcome set | BD-359 |
| CX-278 | The per-component limits are met, so the assembly is feasible; and its converse, an additive peak rule | Two fingers each need one powered tick and the bus powers one per tick: zero one-tick schedules, two two-tick schedules. Local motor limits are identical in both cases, and an additive peak rule prunes the feasible two-tick case | The joint, time-dependent resource contract | BD-360, BD-108 |
| CX-279 | The probe separates every admitted mechanism, so its verdict is safe | The three-law probe scores 675 of 675 in class; a fourth mechanism shares the probe's early trace and reverses all 225 held-out decisions | Conditional witness; direct measurement of the target is always admitted | BD-291, N25e-39 |
| CX-280 | A root of the synthesis equations is a mechanism | Crank-rocker 1, 4, 3 on ground 4: B = (11/5, ±12/5) at crank angle π lie on two assembly circuits | Circuit and branch certificates | BD-361 |
| CX-281 | A Lie-bracket direction is reachable | ẋ = u with u ∈ [0, 1]: rank one, and negative x is unreachable from zero | The integrated finite witness | BD-425 |
| CX-282 | A certificate that held for the declared worlds holds when a world is added | Docking with gains {1, 2} admits ε < (U + 1/2)/4; adding gain 33/32 at ε = 1/64 leaves no exact docking, best worst-case error 1/65 m, and 34,590 positive labels on the grid reverse | Certificates are bound to the declared hypothesis support; revalidation polarity (`FROB-REF-854`) | BD-129, BD-292, N25e-32 |
| CX-283 | A static rejection is a rejection | Of 156 sill cases, 94 agree between sampled statics and a planar dynamic scorer and 62 static rejections succeed dynamically | A static rejection is a candidate for a dynamic test, not a verdict | BD-043, N25e-14 |
| CX-284 | Compare strict predicates in unguarded single precision | On a 65,536-row docking-boundary screen: 452 false admissions and 23,438 false exclusions on the near grid; with the guard and exact fallback, zero final errors and identical bytes on every placement | Exact-integer kernel class and guarded single precision | BD-030, BD-355 |
| CX-285 | Run the speed test, then interpret it | Pusher: composed over specialized median time 1.345 warm; a residual of 575.7 µs that the solver does not touch bounds a zero-cost solver at about 1.46 (842.8 ÷ 575.7). The test could not have shown a large ratio | Compute the ceiling before any speed test | BD-354 |
| CX-286 | Repeated reads of a sensor always add information | Majority vote at 3/4 gives 27/32, 459/512, 3807/4096 for 3, 5, 7 independent reads, and stays at 3/4 when the reads share one fault bit | Error dependence is declared (equivalent to CX-052) | BD-290, N25e-34 |
| CX-287 | A memo keyed by a semantic receipt beats a dependency graph, or may omit an input | Four edit states: both do 3 pair evaluations and 2 reductions and match fresh replay; omitting one dependency inherits a stale promotion | Complete memo keys; no reuse claim over a correctly keyed graph | BD-292 |

Late rows (stated by the rules added after the catalogue was compiled; provisional tags until numbered):

| Tag | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control realized by |
|---|---|---|---|---|
| late-01 | A simulation relation transports a chance claim | One action, two outcomes; source succeeds with probability 0.99 and target with 0.01. The relation holds; "at least 0.95" is true of the source and false of the target; every coupling disagrees on at least 0.98 of its mass | A chance or expectation claim crosses only with a coupling budget: P_target ≥ P_source − Σ εₜ | N25e-01, N25e-02, N25e-03 |
| late-02 | Fill every parameter value above the first success | Sill fixture, 75 mm wheel, sharp lip, payload 40 mm back: success, timeout, success at μ = 0.500, 0.525, 0.550; at half the physics step: timeout, timeout, success; stiffer contact: all succeed | No monotone fill without a proof of monotonicity; non-monotone outcomes are `model-conditional` | N25e-07, N25e-08, N25e-09 |
| late-03 | Every term of a sum is at most 1, so the sum is | Two terms 0.6 and 0.6: each passes and the sum is 1.2. With 0.9 and 0.15 (sum 1.05) equal weights give 0.54 and do not exclude; weights (6/7, 1/7) give 1.05⁷ ≈ 1.407 and do | The term-wise screen is not a decision; relaxations only exclude | N25e-11, N25e-12 |
| late-04 | Boxes will find a contradiction between two sizing relations | Leg holding 117.72 N and moving at 1.5 m/s with a 2 N·m, 80 rad/s motor: each constraint alone is satisfiable over ℓ ∈ [0.1, 0.4] m, N ∈ [5, 50]; together they need ℓ/N ≤ 0.01699 and ℓ/N ≥ 0.01875. The multipliers (1, 1) give 8829/8000 > 1 | Monomial screen in exponent coordinates with an integer certificate | N25e-10, N25e-12 |
| late-05 | A relaxation that omits a resource still bounds every controller | Carrier of 10 kg, 300 J, h = 2 m, arrival at up to 1.5 m/s: the complete bound allows 16.22 kg (Ū = 0.622); without approach energy 15.29 kg (0.529), which stops the design against a competitor at 0.60, falsely: 6 kg is feasible (313.92 J ≤ 318 J) | The dominance stop needs a relaxation that encloses every declared state and energy source (`FROB-REF-551`) | N25e-13, N25e-14, N25e-15 |
| late-06 | Minimum energy is convex in torque and gear ratio | g(τ, N) = τ²/N²: g(0, 1) = 0, g(2, 3) = 4/9, midpoint g(1, 2) = 1/4 > 2/9 | Gear ratios are enumerated; a certificate excludes a motion, not a design (rear-drive cart: 4 m/s² infeasible for every battery placement, feasible with all wheels driven) | N25e-18, N25e-20 |
| late-07 | A bounded split set can decide any constraint | g = Σ_{i=1..20} xᵢ − 10 ≤ 0 on [0, 1]²⁰: with any six parameters split the weak width is 14 against a split width of 6; no cell can be decided although half the volume is feasible | Weak-variable enclosure is for weak dependence; stop when the split width falls below the weak width | N25e-23, N25e-24 |
| late-08 | One feasible command per load vertex certifies the controller | Stance family: the four vertex allocations at b = 3/14 m are not values of one affine law ((160, 240, 240, 160) N against (240, 160, 160, 240) N); an affine law needs b = 1/2 m; a constant command works for no b | The controller class is part of a family answer; the certificate carries the law | N25e-28, N25e-29, N25e-30 |
| late-09 | Pairwise independent reads are independent | Seven reads at accuracy 3/4 with K wrong reads distributed (43, 3, 9, 9)/64 on K = 1, 2, 3, 4: every pair has the independent table, and the best success is 55/64, not 3807/4096 | A sensor budget states the joint error law; redundancy is not credited under `pairwise` | N25e-34, N25e-36 |
| late-10 | A label count is enough; and independent extremes make a counterexample | Three labels need 2 bits on a clean channel, 3 under one erasure, 5 under one flip (four bits give a distance sum of at most 8 < 9). Pad margin under the delayed law: 4.72 N with q and τ shared, −0.078 N from independent extremes, which is no trial | A label is a message over a declared channel; shared nuisance parameters stay shared | N25e-35, N25e-37, N25e-38, N25e-41 |
| late-11 | A small increment is a fixed point; a weight sweep finds every nondominated build | Pack F(m) = 1 + 0.99·m: after 527 iterations 99.5 kg with a 5 g step, and F(99.5) = 99.505 > 99.5; the least feasible mass is 100 kg. Builds (0, 6), (4, 4), (6, 0): (4, 4) minimizes no weighted sum, yet a facility budget of four selects it | Sizing on a witness F(U) ≤ U; Pareto claims on dispositions | N25e-42, N25e-43, N25e-44 |

## Acceptance criteria

- [ ] `registries/counterexamples/augur.toml` holds the eighteen numbered rows and the eleven late rows with every field filled.
- [ ] Every row's control test exists, is tagged with the row's identifier, and fails when its shortcut is planted (checked by the negative-control registry's self-test: a gate that nothing can fail validates nothing).
- [ ] The registry check passes; a row without a control, or a control naming a missing row, fails the build.
- [ ] Rows stated as equivalent to entries of other chapters (CX-270 with CX-010; CX-286 with CX-052) carry the cross-reference and share one fixture.
- [ ] When the catalogue assigns numbers to the late rows, the provisional tags are replaced in one commit and the tests' tags follow.

## Test plan

### Unit tests

`crates/frob-conformance/tests/cx_augur.rs`: `registry_has_18_numbered_rows`; `registry_has_11_late_rows`;
`ids_unique`; `every_row_has_control`; `controls_exist_and_are_tagged`; spot checks of the cases' arithmetic in
exact rationals: `cx270_midpoint_841_800` ((1.45)·(1.45)/2 = 841/800 > 1); `cx271_triple_infeasible` (4/3 > 1);
`cx276_18_19_of_20`; `late01_tv_49_50`; `late03_two_terms` (1.2; 0.54; (21/20)⁷); `late04_8829_8000`;
`late06_quarter_above_two_ninths`; `late09_55_64`; `late11_fixed_point_100`.

### End-to-end script and logging

`scripts/e2e/augur_counterexamples.sh`: reads `registries/counterexamples/augur.toml`, runs every control test by
its tag in its planted-shortcut mode (a `cargo test` filter per crate, with the test profile's mutant flags), and
asserts each control fires; for rows whose control is a public-verb behaviour it also runs the `frob` command of
that row's gate script in its mutant wrapper. Logs through the STD-E2E helper to
`target/e2e-logs/augur_counterexamples/<utc-timestamp>.ndjson` with a transcript: `ts`, `script`, `bead`
(`N25-CX`), `step`, `event`, command, exit code, wall time; one `assert` line per row with the row identifier, the
control test, and whether the planted shortcut was caught; the `summary` gives 29 of 29.

### Known answers and negative controls

Every row is itself a negative control. Self-test of the registry: a row whose control is replaced by a test that
always passes must be detected (the planted-shortcut run does not fail), and the script must report it.

## Considerations

- Several late rows pair two cases stated together by one rule (late-10, late-11); the catalogue's owner may
  split them when numbering.
- The cases quoted from plan-time pilots (CX-274, CX-283, CX-284, CX-285, CX-287, late-02) are FACTs about small
  programs, to be re-run from the reproduction bundle before a gate cites them.
- No row is a claim about a physical robot; each is arithmetic on a stated model.

## References

Appendix N.22; §25.4.7; §25.22; §25.29 to §25.39; CX-270 to CX-287; CX-010; CX-052.
