---
key: "N24-KA"
title: "Known-answer registry for Arena (KA-594 to KA-623): requirements, metrics, fixtures, studies, qualification"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "arena", "frob-conformance", "frob-spec", "frob-arena", "registry", "test", "solid"]
parent: "WP-41"
deps: ["BD-025", "N38a-12", "N38a-40", "N38b-05", "N41-KA", "STD-E2E", "STD-TEST"]
needs: []
related: ["N24a-05", "N24a-07", "N24a-08", "N24a-09", "N24a-10", "N24a-11", "N24a-12", "N24a-16", "N24a-18", "N24a-20", "N24a-22", "N24a-33", "N24a-34", "N24a-37", "N24a-42", "N24c-10", "N24c-17", "N24c-18", "N24c-19", "N24c-22", "N24c-27", "N24c-28", "N24c-29", "N24c-31", "N24c-32", "N24c-37", "N24c-38", "N38-KA", "N38a-12"]
gates: []
plan_refs: ["Appendix M.23", "§24.16", "§24.5", "§24.7", "§24.8", "§24.10", "§24.11", "§24.12", "§24.13", "§24.19", "§24.20", "§24.21", "§24.22", "§24.23", "§38"]
---
## Background

A known answer is a case whose correct result is known independently of the code under test: from a closed form,
an exact enumeration or a hand calculation. The program's rule is that every capability has known-answer tests
against a truth that was never regenerated from the candidate, and that every gate has at least one negative
control: a planted defect the test must catch, because a gate that nothing can fail validates nothing. This bead is
the registry of the thirty known answers of the task-and-scoring subsystem (Arena: crates `frob-spec` and
`frob-arena`), numbered KA-594 to KA-623. It exists so that none is lost between the specification and the tests,
so that every test states the same value, and so that a completeness check can tie each row to a test and each
negative-control cell to a registered control.

A note on the numbers. In this subsystem a value that follows from a stated formula is a derived value: it is
recomputed by its test, not copied. The registry therefore carries the formula and the inputs beside each printed
value, and its own unit test recomputes every derived value in exact or double-precision arithmetic.

Tiers named in the rows are the tiers of the verification harness: G0 laws and self-tests; G1 known answers; G2
parity with pinned oracles; G3 metamorphic relations; G4 chaos (cancellation, faults, fuzzing); G5 determinism.
"host" is plain host arithmetic, "referee" the reference simulation tier, "lockstep" the batched tier in lockstep
mode, "cuda" and "metal" the device backends, "daemon" the long-running service, "auditor" the offline verifier.

## Scope

In: the thirty rows as machine-readable data in the suite's one file; the chapter's completeness check (the row
schema, the loader and its five rules are N38a-12's, the number blocks N38a-40's, the seeding of controls
N38b-05's: all dependencies); the derived-value recomputation test;
the completeness test that ties rows to tests and controls; the mapping of rows to the beads that carry them.

Also in: the eight known-answer groups of the late subsections (§24.19 to §24.23), which have no row in the
appendix. They are registered here under local labels, in the same file and under the same loader, until
the block of numbers this bead requests of the allocation ledger is issued (the table "Late rows" below).

Out: the tests themselves (owned by the beads in the last column); the counterexample registry and the refusal-code
registry of this chapter (N24-CX and N24-REF).

## Technical approach

**Storage.** One `[[ka]]` record per row appended to the suite's one physical file,
`crates/frob-conformance/registry/known_answers.toml`, in the row schema of N38a-12 (a dependency): `id`,
`section` with `owner`, `fixture`, `same_as`, `case`, `truth`, `tolerance` (`exact`, `abs`, `rel`,
`printed-digits`, `gate` or `to-set`), `tier`, `sites`, `control` (`text`; `nc` filled by the seeding tool of
N38b-05, never by hand), `derivation`, `label`, `due`, `printed`, `slug`. No chapter file: the loader refuses
any other `known_answers*.toml`, or a `.toml` under a directory named `known_answers`, with `ka-second-file`
(DESIGN correction: an earlier draft kept `registry/known_answers/arena.toml` with its own fields `where`,
`formula`, `negative_control`, `tests`, `gates` and `carried_by`). The columns map as: "Where" → `section` (the
subsections listed below) with `owner` = the gate; "The case" → `case`; "The truth (formula or value)" → `truth`,
whose `value` carries the formula and its inputs beside the printed value (the formula is part of the truth, not
a field of its own); "Tolerance" → `tolerance` ("to the printed digits" is `printed-digits`; a gate's threshold is
`gate` with the gate named); "Tier" → `tier` and `sites`; "Negative control" → `control.text` (`none stated` where
the column says so); "Carried by" and the test names → the front matter of the row's derivation file
under `docs/derivations/` (for each KA it serves: the test that checks it, crate and test name, and the owning
bead), never a field of the record. Fixture identities: `cart-sill-dock` for fixture 1 (KA-602 to KA-614, whose
derivation files N24b-04 writes one per row under `docs/derivations/fixture-cart-sill-dock/`, each listing its
row), `quad-split-grip` for fixture 2 (KA-616, KA-617, files under `docs/derivations/fixture-quad-split-grip/`;
KA-616 has `same_as = "KA-413"` and shares that row's fixture and derivation file), and one fixture each, at
`docs/derivations/<fixture>.md`, for the other rows.
Two rows of other registries restate cases of this one and share its fixtures through `same_as`, which must name
a row already in the file. K41-33 of N41-KA (a dependency, M0; the first-session row with fixture 1's masses,
tread caps, lip levers and front lift-off friction demands) carries `fixture = "cart-sill-dock"` from M0, when
KA-602 does not yet exist: the commit of this bead that adds KA-602 sets `same_as = "KA-602"` on that row (found
by its `slug`, `K41-33`), and the row's derivation file stays in `docs/derivations/fixture-cart-sill-dock/`
beside N24b-04's per-row files, listing K41-33 in its front matter. TF-24 of N39-KA (an M3 bead that depends on
this one; the retry-cost ratios 1.23, 2.57, 4.08 and 5.15) carries KA-621's fixture with `same_as = "KA-621"`,
and N39-KA adds TF-24 to the front matter of KA-621's derivation file when its row lands.
Exact values are stored as exact strings (rationals such as `"13/256"`, `"625/256"`; integers; decimals as
printed). The row stores no status: eligibility is computed by the loader. Eleven rows state no control (KA-602 to
KA-605, KA-608, KA-609, KA-611, KA-613, KA-614, KA-617, KA-621): they carry `control.text = "none stated"` and are
`obligation: control`, never `eligible`, until a control is registered (the chapter's owed controls are N38b-33's);
this bead's completeness test, not the loader, refuses a record whose source states a control and whose
`control.text` is empty.

**Where stated (the `section` field) and owner.** KA-594, KA-595, KA-596 §24.5, §24.16; KA-597 §24.5, owner
GATE-ARENA-03; KA-598 §24.8, §24.16; KA-599 §24.8, §24.16, GATE-ARENA-06; KA-600 §24.8, GATE-ARENA-07; KA-601
§24.7, GATE-ARENA-08; KA-602 to KA-609 and KA-611 §24.10, GATE-ARENA-04; KA-610 §24.10, §24.16, GATE-ARENA-04;
KA-612 §24.10, §24.16, GATE-ARENA-05; KA-613 §24.10, GATE-ARENA-05; KA-614 §24.10; KA-615 §24.10, §24.16; KA-616
§24.11, §24.16, ≡ KA-413, GATE-ARENA-06; KA-617 §24.11, GATE-ARENA-06; KA-618 §24.12, §24.16; KA-619 §24.13,
§24.16, PO-ARENA-02; KA-620 and KA-621 §24.13; KA-622 §24.13, GATE-QUAL-01; KA-623 §24.13, GATE-QUAL-02 and
GATE-QUAL-03. Where the source prints no gate, `owner` is the gate of the carrying bead (DESIGN, so that every row
joins a gate where one exists): GATE-ARENA-03 for KA-594, KA-596 and KA-615; GATE-ARENA-05 for KA-614 and KA-621;
GATE-QUAL-01 for KA-620; KA-595, KA-598 and KA-618 have no gate (their carrying beads N24a-11, N24a-18 and
N24b-14 record none). A row's `due` is the milestone of its gate in the gate register: M1 for GATE-ARENA-03, -04
(referee half), -05, for the rows without a gate and for KA-619 (row 11 of N24c-10's `fixtures` battery); M2 for
GATE-ARENA-06 and -08; M3 for GATE-ARENA-07 and GATE-QUAL-01, -02, and so for KA-620; M5 for the replay half of
KA-623 (GATE-QUAL-03), whose firewall half is due at M3.

**Symbols used by the fixture rows.** Fixture 1 is a four-wheeled cart crossing a door sill of height h = 12 mm:
wheelbase L = 0.32 m; chassis 5.0 kg; payload 2.2 kg at fore-aft offset δ; rear wheels of radius 60 mm and 0.2 kg
each; front (driven) wheels of radius r = 60 mm and 0.2 kg (design A) or r = 75 mm at the same density and width
(design B); motor torque constant k_t = 0.2 N·m/A, back-EMF constant k_e = 0.2 V·s/rad, resistance R = 1 Ω,
inductance 10 mH, current limit I_max = 6 A; outer-loop gain K_ω = 2 A·s/rad; shaft damping 0.002 N·m·s/rad; g =
9.81 m/s²; W total weight; F₀ static load on the climbing axle on level ground; x = √(2rh − h²) the horizontal
distance from axle to lip at lift-off; tan θ = x/(r − h). Fixture 2 is a quadruped sole of width w whose centre is
displaced by z into the high-grip side (μ = 0.8) of a boundary with a low-grip strip (μ = 0.25).

**The rows.**

| KA | Where | The case | The truth (formula or value) | Tolerance | Tier | Negative control | Carried by |
|---|---|---|---|---|---|---|---|
| KA-594 | Requirement language | Normalization: clearance margin 0.02 m and thermal margin 3 K with scales 0.10 m and 10 K | Margins 0.02/0.10 = 0.2 and 3/10 = 0.3 in either unit system; the clearance binds in metres and in millimetres | exact | G1 · host | Composition of raw margins: the raw minimum is 0.02 in metres (clearance binds) and 3 with the clearance in millimetres (thermal binds) | N24a-05, BD-121, N24c-10 (battery row 6) |
| KA-595 | Requirement language | Joint stage accounting: 20 equally likely worlds, two stages each passing in 19 | 18 of 20 if the failing worlds differ, 19 of 20 if they coincide; never 0.95² = 0.9025 (18.05 of 20) | exact | G1 · host | Multiplying stage rates | N24a-11, N24c-10 (battery row 10); N24a-28 and N24a-30 (the two-stage task families) |
| KA-596 | Requirement language | Online and offline agreement: a past-time signal sampled at the end of the trace | Equal to the offline evaluation of the same formula, bit for bit | exact | G1 · host | A monitor that pads a short window | N24a-07, N24a-08, N24a-09, BD-121, N24c-10 (battery row 8) |
| KA-597 | GATE-ARENA-03 | The three requirement evaluators on 10⁶ random traces per operator, including multi-rate signals and windows with rounding | Agreement bit for bit; every requirement-language refusal (FROB-REF-520, -535, -536, -523, -538, -525, -526, -539 and -528) is triggered by its planted task file | exact | G5 · host | The planted task files | BD-121, BD-122, N24a-42 |
| KA-598 | Metrics | Energy accounting: design A with one 10 J success and one 20 J failure; design B with two 12 J successes | Conditional on completion: 10 J against 12 J. Per completion: (10 + 20)/1 = 30 J against (12 + 12)/2 = 12 J. Both are published | exact | G1 · host | Dropping failed attempts from the total | N24a-18, N24a-37, N24c-10 (battery row 3), BD-254 |
| KA-599 | GATE-ARENA-06 | Slip metric under duplication of a quadrature point with halved weights | Unchanged | 10⁻¹² relative in double precision | G3 · referee | A metric that sums unweighted point speeds | N24a-20, N24b-11 (the metric), N24b-22 (e2e), BD-202 |
| KA-600 | GATE-ARENA-07 | Scorecard accounting on 100 random batteries | A planted infeasible scenario appears in its column and in the all-scenario denominator; columns sum to the battery size | exact | G1 · host | A silently shortened battery must change identity | N24a-22, BD-254, N24b-23 (e2e) |
| KA-601 | GATE-ARENA-08 | Settled-pose reset cache on all 64 states for each reference robot | Penetration at most 0.5 mm; first contact impulse within 5% of static support | as stated | G1 · lockstep | A design edit must invalidate the cache | BD-195, N24a-16 |
| KA-602 | Fixture 1 | Mass roll-up: design A (60 mm front wheels, 0.2 kg each) and B (75 mm, same density and width) | Total mass 5.0 + 2.2 + 4·0.2 = 8.000 kg and 5.0 + 2.2 + 2·0.2 + 2·0.3125 = 8.225 kg; front-wheel mass ratio (75/60)² = 1.5625 (0.3125 kg each); spin-inertia ratio (75/60)⁴ = 625/256 (½·m·r²: 3.6 × 10⁻⁴ and 8.7890625 × 10⁻⁴ kg·m²); B pitches up by asin(15/320) = 2.69° | exact to the printed digits | G1 · host | none stated | N24b-04 (host), BD-123 (referee rigs), N24b-09 (batch tiers), N24b-20 (e2e) |
| KA-603 | Fixture 1 | Tread force cap: two wheels at the current limit | 2·k_t·I_max/r = 2·0.2·6/0.060 = 40 N for A; 2·0.2·6/0.075 = 32 N for B | GATE-ARENA-04 | G1 · referee | none stated | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-604 | Fixture 1 | Force slope: unsaturated flat rolling at settled current, per unit speed error | 2·k_t·K_ω/r² = 222.22 N/(m/s) for A and 142.22 N/(m/s) for B; a gain of 3.125 A·s/rad for B matches the slopes | GATE-ARENA-04 | G1 · referee | none stated | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-605 | Fixture 1 | Lip lever x = √(2rh − h²) for a sharp lip of h = 12 mm | √1296 = 36.000 mm at r = 60 mm; √1656 = 40.694 mm at r = 75 mm | exact to the printed digits | G1 · host | none stated | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-606 | Fixture 1 | Front lift-off in the creep limit: driven axle at the lip, trailing axle free on the floor. The lip force is vertical, so the friction demand is μ ≥ tan θ; total drive torque F₀·x/(1 + x/L) | μ ≥ 36/48 = 0.7500 for A and 40.694/63 = 0.6459 for B; with 2 mm fillets 0.7332 and 0.6351; torque for A at δ = 0: 39.24 N · 0.036 m / 1.1125 = 1.270 N·m of 2·0.2·6 = 2.4 N·m available | The outcome flips between friction grid points of spacing 0.01 that bracket the threshold, excluding points within 0.005 on the referee and 0.01 on the fast tiers (TARGET) | G1 · referee, lockstep | A run exactly at the threshold must carry the `boundary` flag | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-607 | Fixture 1 | Rear pull-over in the creep limit: free axle at the lip, driven axle on the floor; V = W·d/(L − x − h·tan θ), d the horizontal distance from the driven contact to the mass centre; the lip force's horizontal part is V·tan θ | For A: V/W = 32/55 − δ (δ in metres) and the friction demand is ¾·(32/55 − δ)/(23/55 + δ): 24/23 = 1.043 at δ = 0, 0.887 at δ = +0.04 m. For B at δ = 0 the demand of 34.2 N exceeds the 32 N force cap | GATE-ARENA-04 | G1 · referee, lockstep | A creep rig with the rear lip removed | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-608 | Fixture 1 | Creep-feasible set for A, both axles | μ ≥ ¾·(32/55 − δ)/(23/55 + δ) lies inside the admitted friction range [0.35, 0.95] only for δ ≥ 23.0 mm: 1.13% of the development distribution | to the printed digits | G1 · host | none stated | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-609 | Fixture 1 | Energy: raising the front axle of A by h takes F₀·h; kinetic energy at 0.30 m/s is ½·m·v² plus wheel spin | 39.24·0.012 = 0.471 J against ½·8·0.09 = 0.360 J plus 4·½·3.6 × 10⁻⁴·5² = 0.018 J: momentum alone does not carry one axle over | to the printed digits | G1 · host, referee | none stated | N24b-04, BD-123, N24b-09, N24b-20 |
| KA-610 | Fixture 1 | Wheel-loop stability: the 75 mm wheel with 0.0002 kg·m² added rotor inertia (0.00107890625 kg·m² in all), slipping against a constant torque, rate sampled and current target held for 10 ms; spectral radius of the one-tick map | 1.4820 with the feedforward term R·i_ref; 0.9207 with R·i; an instantaneous-current approximation gives 2.6917 for both (2.7075 if the shaft damping is left out) | to the printed digits | G1 · host, referee | At a 0.5 ms step, explicit Euler that updates the current before the wheel speed gives 1.854 and 1.497, turning the stable case unstable | N24b-04, BD-123, N24b-08 (the mechanism controls), N24b-09, N24b-20 |
| KA-611 | Fixture 1 | Power ledger: a blocked winding at 3 A and 3 V; a drive at 50 rad/s and −2 A | 9 W dissipated, 9/24 = 0.375 A from the 24 V bus; terminal voltage R·i + k_e·ω = −2 + 10 = 8 V, 16 W returned, 20 W from the shaft, 4 W in copper | exact | G1 · host, referee | none stated | N24b-04, BD-123, N24b-09, N24b-20, N24a-37 |
| KA-612 | GATE-ARENA-05 | Fixture 1, rule enumeration at n = 64, 86, 128, 256: count pairs (a, b) with a + b ≤ n classified as prefer B, B inferior, practical tie, inconclusive | (n + 1)(n + 2)/2 pairs. 2,145: 521, 521, 0, 1,103. 3,828: 1,031, 1,031, 1, 1,765. 8,385: 2,517, 2,517, 4, 3,347. 33,153: 11,384, 11,384, 39, 10,346. Perfect concordance at n = 64 gives [−0.0662, +0.0662]; a tie is first reachable at n = 86 and only for (0, 0), with chance 0.9⁸⁶ = 0.0116% at 5% discordance each way; with no A-only pair a preference needs 15 B-only pairs of 64, and 17, 19, 21, 23 with one to four A-only pairs | exact | G1 · host | A request for a tie verdict at n = 64 is refused (FROB-REF-532) | N24b-06 (the enumerator), N24b-07, N24b-21 (e2e), BD-124 |
| KA-613 | GATE-ARENA-05 | Fixture 1, power of the rule: P(prefer B) at discordance (0.17, 0), (0.25, 0.02), (0.40, 0.02); P(tie) at (0.02, 0.02) and (0, 0) | n = 64: 0.117, 0.406, 0.962, 0, 0. n = 128: 0.775, 0.906, above 0.999, 0.072, 1. n = 256: 0.998, above 0.999, above 0.999, 0.427, 1 | exact to the printed digits | G1 · host | none stated | N24b-06, N24b-10, N24b-21, BD-124 |
| KA-614 | Fixture 1 | Standalone equivalence test: zero discordant pairs in 64 | Exact one-sided 95% bound 1 − 0.05^(1/64) = 0.0457, inside the five-point band | exact to the printed digits | G1 · host | none stated | N24b-06, N24b-10, N24b-21 |
| KA-615 | Requirement language | Boundary comparator on the docking requirement at command limit U = 3/4 and read error ε = 5/16 | The separation between the two worlds' readings is exactly zero and the strict requirement fails; the 65,536-case boundary screen has no final error with the 2⁻²¹ guard and exact fallback | exact | G1 · host, cuda, metal | A non-strict comparator admits the equality case; unguarded single-precision comparison misclassifies the screen | N24a-10, BD-190 |
| KA-616 | Fixture 2 (the same case as KA-413) | Limiting model on the plate-drag rig: share on the high-grip side f = clip(½ + z/w, 0, 1); capacity per unit normal load c(z, w) = 0.8·f + 0.25·(1 − f) = 0.525 + 0.55·clip(z/w, −½, ½); yaw moment 0.275·N·(w/4 − z²/w) for ‖z‖ ≤ w/2 | z = 0: 0.525 and 0.525; 2.75 mm and 4.81 mm per newton. z = +12 mm: 0.690 (40 mm sole) and 0.619 (70 mm sole); 1.76 mm and 4.25 mm. z = −12 mm: 0.360 and 0.431; 1.76 mm and 4.25 mm | 1% on the referee (GATE-ARENA-06) | G1 · referee | A pyramidal friction cone; a sole with one quadrature point | N24b-12 (limiting model and plate-drag rig), N24b-22 (e2e), BD-202 |
| KA-617 | Fixture 2 | Reflected cases | The mirrored robot on the mirrored course returns the mirrored result | inside the determinism class (GATE-ARENA-06) | G3 · referee, lockstep | none stated | N24b-13, N24b-22, BD-202 |
| KA-618 | Study types | Peeking: an exact one-sided sign test at nominal 5% on null pairs, looked at after every pair | False-rejection chance 13/256 = 5.08% after 8 looks, 8.76% after 16, 12.9% after 32, 17.4% after 64 | exact for 13/256; printed digits otherwise | G0 · host | The same data under a confidence sequence stay below their level | N24b-14 (study admission), N24b-21, N24c-08 |
| KA-619 | Qualification | Level series α_i = α/[i·(i + 1)] | Exact partial sums α·m/(m + 1) for m up to 500 (telescoping, since 1/[i·(i + 1)] = 1/i − 1/(i + 1)); limit α (proof obligation PO-ARENA-02) | exact (rational) | G0 · host, auditor | A constant level per request | N24b-15 (the level ledger), N24c-10 (battery row 11), BD-259, BD-260 |
| KA-620 | Qualification | Independent retries at a constant 5% level | Fourteen null submissions give 1 − 0.95¹⁴ = 51% chance of at least one false promotion | to the printed digits | G0 · host | The same adversary against reused clusters at a constant level obtains a false promotion in more than 99% of campaigns | N24b-18, BD-260, N24a-34 |
| KA-621 | Qualification | Cost of a retry relative to one test at α = 0.05 | ln(1/α_i)/ln(1/α): about 1.23, 2.57, 4.08 and 5.15 times the units at requests 1, 10, 100 and 500; for the rule of fixture 1 a tie is first reachable at n = 99, 178 and 328 for requests 1, 10 and 500; at 64 pairs request 1 has 478 preferring outcomes of 2,145 | exact for the counts; printed digits for the ratios | G1 · host | none stated | N24b-06 (tie reachability, 478 of 2,145), N24b-15, N24b-16 (retry cost), BD-259 |
| KA-622 | GATE-QUAL-01 | 2,000 simulated campaigns on no-signal worlds at α = 0.05, each with 500 adaptive retries by a scripted adversary that sees every receipt and all discovery data | A false promotion in at most 123 campaigns (an exact one-sided test of a rate at or below 0.05 rejects at the 1% level from 124); a planted true gain of the declared size is promoted within 10 requests in at least 80% of campaigns | as stated | G0 · daemon | Reused clusters at a constant level: false promotion in more than 99% of campaigns | BD-260, N24b-18 (the campaign simulator), N24a-34, N24b-24 (e2e), BD-259 |
| KA-623 | GATE-QUAL-02, GATE-QUAL-03 | Firewall and replay of a campaign | 10⁴ fuzzed read attempts across every verb return no sealed field; the offline verifier replays the levels, the commitments and, after close, the cluster derivation | exact | G4 · daemon, auditor | A planted leak path is found by the audit; a reordered index, a changed request and a reused cluster are each rejected | BD-261, N24b-17, N24b-19, N24b-24 (firewall, e2e); BD-383, N24b-25 (replay, M5) |

**Derived values the tests need (stated once, so that no test re-derives them differently).**

- Lift-off geometry. Sharp lip: tan θ = x/(r − h). With a fillet of radius ρ = 2 mm on the lip the wheel touches
  the fillet arc; the reading that reproduces the printed digits is tan θ = √((r + ρ)² − (r − h + ρ)²)/(r − h + ρ):
  √1344/50 = 0.7332 for r = 60 mm and √1704/65 = 0.6351 for r = 75 mm. The fixture's derivation file governs if it
  differs.
- Rear pull-over for A. L − x − h·tan θ = 0.32 − 0.036 − 0.009 = 0.275 m; the mass centre lies 2.2·δ/8 = 0.275·δ
  ahead of mid-wheelbase, so d = 0.16 − 0.275·δ and V/W = 32/55 − δ; the driven axle carries W − V and must supply
  V·tan θ, which gives the demand ¾·(32/55 − δ)/(23/55 + δ).
- Creep-feasible set. Setting the demand to 0.95 gives δ = (¾·32/55 − 0.95·23/55)/(0.95 + ¾) = 22.99 mm; with μ
  uniform on [0.35, 0.95] and δ uniform on [−40, 40] mm the admitted share is 1.13%.
- Wheel loop. In deviation coordinates with state (ω, i): J·ω̇ = k_t·i − b·ω and L·i̇ = c·(i_ref − i), where the
  driver's 4 Ω current gain gives c = 4 Ω with the feedforward R·i and c = R + 4 Ω = 5 Ω with R·i_ref; i_ref =
  −K_ω·ω is sampled and held for T = 10 ms. The spectral radius of the resulting one-tick map is 0.9207 (c = 4 Ω)
  and 1.4820 (c = 5 Ω). With instantaneous current the map is the scalar e^(−bT/J) − (k_t·K_ω/b)·(1 − e^(−bT/J)) =
  −2.6917, and 1 − k_t·K_ω·T/J = −2.7075 without damping.
- Reachability of a tie. The rule forms two-sided (1 − α/2) exact Clopper–Pearson intervals for the two
  discordance probabilities. With zero discordant pairs each upper limit is 1 − (α/4)^(1/n), so a practical tie
  (interval inside ±0.05) needs n ≥ ln(α/4)/ln(0.95): 86 at α = 0.05; 99, 178 and 328 at α_1 = α/2, α_10 = α/110
  and α_500 = α/250,500.
- Retry cost. ln(40)/ln(20) = 1.231; ln(2,200)/ln(20) = 2.569; ln(202,000)/ln(20) = 4.078; ln(5,010,000)/ln(20) =
  5.150.
- The 123-campaign threshold. For X ~ Binomial(2,000, 0.05): P(X ≥ 124) = 0.0095 and P(X ≥ 123) = 0.0123.
- Peeking. The chance is computed by exact dynamic programming over the count of successes with absorption at the
  first look whose one-sided p-value is at most 0.05.

**Late rows.** The subsections added to the chapter after its known-answer appendix was compiled state their known
answers in their own text. They are registered with local labels of the form `KA-ARENA-<letter><n>` (W for the
wrench slice, A for the task audit, I for the item bank, E for exposure domains), mirroring the local labels of the
chapter's counterexample registry. Numbers for them come from the allocation ledger
`crates/frob-conformance/registry/allocations.toml` (N38a-40), never from "the next free number" at commit time:
this bead commits one request `[[block]] registry = "ka", count = 8, owner = "N24-KA"`, cites the rows by their
labels until the block is issued, and then maps the labels to the block's numbers in the table order below
(`KA-ARENA-W1` first, `KA-ARENA-E2` last) in one commit, keeping each label in the record's `slug` field. The
record's content does not change. Each late row has one tier and several sites (N38a-12 rule 4: the cone program
on the host and the rig on the referee and the batch tiers are results per site of one row, and a G3 row would have
to be a registered relation, which these are not); DESIGN, correcting an earlier draft that gave W1 and W2 a
second tier G3. A label still used outside `slug` at this bead's milestone exit fails the ledger's
`provisional-tag-past-due` check.

| Label | Where | The case | The truth (formula or value) | Tolerance | Tier | Negative control | Carried by |
|---|---|---|---|---|---|---|---|
| KA-ARENA-W1 | GATE-ARENA-10 | A two-point sole carrying 50 N on each point at lateral offsets ±20 mm, one point on μ = 0.25 and the other on μ = 0.8; and a sole on uniform μ = 0.4 | Largest forward force with the yaw moment free: 12.5 + 40 = 52.5 N, and it comes with a yaw moment of (40 − 12.5) N × 0.020 m = 0.55 N·m. Under a pure forward-force command (zero yaw) the two longitudinal forces are equal: 2 × 12.5 = 25 N. Uniform sole: 40 N both ways. By scalar capacity the split sole wins (52.5 against 40); at the commanded wrench it loses (25 against 40) | 10⁻⁹ in double precision for the cone program; the rig slips within 2% of each value on the referee and inside the fidelity envelope on the batch tiers | G1 · host, referee, lockstep | A scorecard that ranks the two soles by moment-free capacity when the task commands zero yaw must be rejected by `task.check` | N24c-17, N24c-18, N24c-19 |
| KA-ARENA-W2 | GATE-ARENA-10, fixture 2 | Uniform-pressure strip model with the boundary through the centre and longitudinal traction only, μ_L = 0.25 and μ_H = 0.8, for both sole widths | Zero-yaw capacity per unit normal load (μ_L + √(μ_L·μ_H))/2 = (0.25 + √0.2)/2 = 0.3486 when longitudinal traction never points backward, against 0.525 with the moment free; independent of the sole width. With signed longitudinal traction (the low-grip half may push backward) the exact value is 0.3731, and lateral traction carrying moment raises it further, so the closed form is a lower bound; the cone program gives the exact value (as in the counterexample row ARENA-W2 of N24-CX) | 0.525·N guided and at least 0.3486·N free, each within 2% on the referee | G1 · host, referee | A rig run guided against yaw reported as the zero-yaw capacity | N24c-19, N24c-17 |
| KA-ARENA-A1 | GATE-ARENA-11 (hand case) | Goal region x ∈ [4.9, 5.1] m and requirement `(F (0 s 10 s) in_goal)`; a sled foil at a constant 1 m/s | The sled is inside the region for t ∈ [4.9, 5.1] s: verdict `satisfied` with a dwell of 0.2 s. Under the proposed clause `(F (0 s 10 s) (H (0 s 2 s) in_goal))` the sled's longest dwell 0.2 s < 2 s, so the verdict is `violated`; a body at rest at x = 5 m from t = 6 s satisfies it from t = 8 s | exact | G1 · host | An audit that scores only the candidate design never produces the finding | N24c-22, N24c-28, N24a-12, N24a-33 |
| KA-ARENA-A2 | GATE-ARENA-15 (hand case) | A delivery task whose online reward over 60 s is `(H (0 s 2 s) (and clear-margin thermal-ok spill-ok))`; a parked robot | The parked robot keeps all three margins positive for the whole episode and collects the full reward, so the task is flagged `rewards-idleness`. The proposed progress atom, approach speed above 0.15 m/s on a scale of 0.50 m/s, gives the parked robot a margin of (0 − 0.15)/0.50 = −0.3; after the clause is accepted its verdict is `violated` | exact | G1 · host | A task whose idleness is marked intended (station keeping) must not be refused | N24c-27, N24c-28 |
| KA-ARENA-I1 | GATE-ARENA-12 | Item-bank two-value fixture: D is ±1 on a fraction f = 0.1 of items and 0 elsewhere, with mean μ = 0.05 | Uniform sampling: variance f − μ² = 0.0975. Under q*: weight f on the discriminating items, variance f² − μ² = 0.0075 (ratio 13). With the defensive mixture at ε = 0.1: weight 1/(0.9/f + 0.1) = 0.1099, variance 0.1 × 0.1099 − 0.0025 = 0.0085 (ratio 11.5) | exact arithmetic (rationals; 0.1099 and 0.0085 to the printed digits) | G1 · host | A `fixed-n` request with item-bank sampling must be refused | N24c-29, N24c-31, N24c-32 |
| KA-ARENA-I2 | GATE-ARENA-12 | The same fixture under a wrong model that places q* on items disjoint from the discriminating ones | Each discriminating item has q = ε·p and weight 10; the second moment is f × 10 = 1.0 and the variance 1.0 − 0.0025 = 0.9975, against 0.0975 under uniform sampling (10.2 times worse); the estimate stays unbiased. The provable bound 1/ε = 10 holds for the second-moment ratio (exactly 10 here), not for the variance ratio (10.23), so the gate is checked on the second moment and the variance ratio is printed beside it | exact for the moments; the mean of Ĉ over 10⁴ replays within its Monte Carlo error | G1 · host | An estimator that drops the weights is biased on this fixture | N24c-31, N24c-32, N24c-29 |
| KA-ARENA-E1 | GATE-ARENA-14 | A push keyed by control tick 1,500 | It occurs at 3.0 s with a 2 ms step and at 1.5 s with a 1 ms step; the keying fails check 3 (rate invariance) and is refused | exact | G1 · host | The same push declared in the `process` domain occurs at 3 s at both steps | N24c-37, N24c-38 |
| KA-ARENA-E2 | GATE-ARENA-14 | A push keyed by "the tenth footfall" | It occurs at 5.0 s under a gait period of 0.5 s and at 4.0 s under 0.4 s; the keying fails check 4 (retiming) and is refused | exact | G1 · host | The same push declared as an `event` at a gate crossing occurs at each arm's own crossing | N24c-37, N24c-38 |

**Completeness test.** `cargo test -p frob-conformance --test known_answers_arena` (the unit tests below): reads
the suite file through the test kit's one parser (STD-TEST); asserts 30 records KA-594 … KA-623 with no gap and
the eight late records `KA-ARENA-W1` … `KA-ARENA-E2`; the test names that each row's derivation file lists exist
in the test tree once their owning beads have landed (until then the derivation file lists the row as
`pending: <bead key>` and the test reports the pending count instead of failing);
each negative-control cell other than "none stated" maps to an entry of the negative-control registry attached to
the gate it protects; KA-616 and KA-413 point at one shared fixture record and cannot diverge.

## Acceptance criteria

- [ ] The eight late rows are present with the values as printed, each with its owning beads; the derived-value
      test recomputes 52.5, 0.55, 25, 0.3486, 0.0975, 0.0075, 0.1099, 0.0085 and 0.9975.
- [ ] The block request for eight `ka` numbers owned by N24-KA is committed in `allocations.toml`; once issued, the
      eight rows carry its numbers in table order with their labels in `slug`, `registry::alloc::validate` passes,
      and no label remains outside `slug` at the M1 exit.
- [ ] `crates/frob-conformance/registry/known_answers.toml` holds exactly the thirty numbered rows above with the
      values as printed, every field of the N38a-12 schema filled, and validates under N38a-12's loader; no other
      known-answer file exists (`ka-second-file` reports nothing). The loader rejects a duplicate identifier and a
      missing truth; this bead's completeness test rejects a row whose source states a control and whose
      `control.text` is empty; the eleven rows whose source states none are computed `obligation: control`.
- [ ] The derived-value test recomputes, in exact rational or double-precision arithmetic, every value in the
      "Derived values" list and the arithmetic inside rows KA-594, -595, -598, -602 to -609, -611, -614, -616,
      -618 (13/256), -619, -620, -621 and the 123-campaign threshold, and matches the printed digits.
- [ ] Every row names at least one owning bead and, once that bead has landed, at least one test; no row whose
      gate belongs to the first milestone is `pending` at the first-milestone exit review.
- [ ] Each stated negative control has an entry in the negative-control registry with its gate.
- [ ] After the commit that adds KA-602, K41-33 (N41-KA's row) carries `same_as = "KA-602"` on fixture
      `cart-sill-dock` and the loader accepts the file; KA-621's derivation file accepts further rows that share
      its fixture (TF-24 of N39-KA).
- [ ] The eleven rows of the chapter's own known-answer table (fixture 1 analytic layer; fixture 1 rule
      enumeration; fixture 1 energy accounting; fixture 2 limiting model; slip metric; normalization; boundary
      comparator; online and offline agreement; peeking; joint stage accounting; level series) each map to at least
      one registry row, and the map is asserted.

## Test plan

### Unit tests

In `crates/frob-conformance/tests/known_answers_arena.rs`, on the real registry file and on scratch copies:
- `arena_registry_loads`: 38 records: the 30 numbered records KA-594 … KA-623 with contiguous identifiers and the
  eight late records `KA-ARENA-W1` … `KA-ARENA-E2` (by label before the block is issued; after it, by the block's
  numbers in table order with the labels in `slug`); schema valid.
- `arena_late_rows_in_block`: once the block exists, each late row's number lies inside the block issued to
  N24-KA, the order of numbers equals the table order, and a planted late row numbered outside the block is
  refused by `registry::alloc::validate` (`unallocated-number`).
- `arena_registry_rejects_duplicates_and_gaps`: a copy with KA-600 duplicated, and one with KA-611 removed, fail.
- `derived_values_recompute`: one assertion per item of the acceptance list, for example: 625/256 as an exact
  rational; x(60) = 36 exactly and x(75) = 40.694 to three decimals; 24/23 = 1.043; 1 − 0.05^(1/64) = 0.0457;
  Σ_{i=1..500} α/[i·(i + 1)] = α·500/501 as rationals; peeking after 8 looks = 13/256 exactly.
- `wheel_loop_spectral_radius`: 0.9207 and 1.4820 from the two-state map; −2.6917 and −2.7075 for the scalar maps.
- `capacity_table_fixture2`: c(0, w) = 0.525; c(+12 mm, 40 mm) = 0.690; c(+12 mm, 70 mm) = 0.619; c(−12 mm, 40 mm)
  = 0.360; c(−12 mm, 70 mm) = 0.431; yaw moments 2.75, 4.81, 1.76 and 4.25 mm per newton.
- `chapter_table_maps_to_rows`: each of the eleven known answers of the chapter's own table maps to at least one
  row (the fixture-1 analytic layer to KA-602 to KA-611, the rule enumeration to KA-612, the energy accounting to
  KA-598, the limiting model to KA-616, the slip metric to KA-599, normalization to KA-594, the boundary
  comparator to KA-615, online and offline agreement to KA-596, peeking to KA-618, joint stage accounting to
  KA-595, the level series to KA-619).
- `negative_controls_registered`: after N38b-05's seeding, each of the 19 rows with a stated control has a
  non-empty `control.nc` resolving to a control attached to its gate; the 11 rows without one are `obligation:
  control`.
- `shared_fixture_ka616_ka413`: KA-616 has `same_as = "KA-413"`, the same `fixture` as KA-413 and the same
  derivation file; a scratch copy that gives KA-616 a second fixture name is refused by the loader.
- `no_chapter_file`: a scratch copy of these rows written to `registry/known_answers/arena.toml` fails the load
  with `ka-second-file` naming that path.
- `shared_fixture_k41_33`: with this bead's rows in the file, the row whose `slug` is `K41-33` has
  `same_as = "KA-602"` and `fixture = "cart-sill-dock"`, and a file of `docs/derivations/fixture-cart-sill-dock/`
  lists it; a scratch copy that sets the link while KA-602 is absent is refused by the loader.

### End-to-end script and logging

`scripts/e2e/known_answers_arena.sh` (header `bead=N24-KA milestone=M1 labels=arena,registry gates=`). There is
no `frob gauntlet`, `frob gates` or `frob conformance` command (ADR-GAUNT-01: the harness is never driven through
the surface it judges), so the script runs the registry in its cargo form: `cargo test -p frob-conformance --test
registry_check`, `cargo test -p frob-conformance --test known_answers_arena`, then the rows on their tiers with
`FROB_TEST_SEED=2401 cargo test -p frob-conformance --test <tier> -- <rows>`, where `<tier>` follows each row's
tier column: `g1_known_answers` for the G1 rows (host and referee sites), `g0_laws` for KA-618 to KA-620 and
KA-622, `g3_metamorphic` for KA-599 and KA-617, `g4_chaos` for KA-623 and `g5_determinism` for KA-597; and the
same with `--features gauntlet-mutations` so that each registered negative control runs and must be caught. It reads the receipts those runs write and the registry rows (`known_answers.toml`, `controls.toml`)
read-only; it records no gate (the gates are recorded by their measuring beads with the `record_gate` example). It
asserts that every row due at or before the current milestone passes or is reported as an obligation with its
kind, that no row cited by a recorded gate is `to-set` or `obligation: control`, and that each caught control is
named in the receipt. Through the shared end-to-end helper (STD-E2E) it writes newline-delimited JSON to
`target/e2e-logs/known_answers_arena/<utc-timestamp>.ndjson` with a transcript beside it: per row one line with
`ts`, `script`, `bead`, `step`, `event`, the KA identifier, the exact command, exit code, wall time, the receipt
hash, the site, the measured value with unit, the expected value, the tolerance, pass, fail or obligation, and for
a control the defect planted and whether it was caught; on a failure the first differing value and the receipt
path; the summary line gives rows passed, failed, obligations and unbound and the gates touched (GATE-ARENA-03 to
-08 and, through the late rows, -10 to -12, -14 and -15; GATE-QUAL-01 to -03). Rows whose carrying bead has not
landed have no runner entry and are logged `unbound` (N38a-13's outcome) with their carrying bead and milestone,
never as passed; the script does not fail on them, so this bead's close never waits on a carrier, and it fails
only on `fail` or `control-missed`.

### Known answers and negative controls

The registry is the list. Negative controls of the registry itself: a row whose truth was edited to match a failing
candidate (caught because derived values are recomputed and because the file's digest is recorded in each gate
receipt); a row silently dropped (caught by the contiguity check); a control cell left unregistered (caught by
`negative_controls_registered`). Metamorphic relations referenced by rows: unit normalization (KA-594),
streaming equals offline (KA-596), slip invariance under quadrature refinement (KA-599), batch-composition
invariance (KA-601), qualification under adaptive retries (KA-622). Determinism: the loader's output is
byte-identical across platforms.

## Considerations

- Rows KA-599, KA-601, KA-616 and KA-617 belong to gates of the second milestone, KA-600 and KA-622 to the third,
  and the replay half of KA-623 to the fifth; they stay `pending` until then and the completeness test reports
  them.
- KA-610 doubles as a mutation test of the actuator stage (its negative control is a wrong integration order);
  the registry marks it so that the actuator bead also lists it.
- Where a row's tolerance is "to the printed digits", the test compares after rounding to the digits printed and
  never to a looser bound.
- The fillet formula and the reading of the wheel loop above are reconstructions that reproduce the printed
  values; the fixture's own derivation files (`docs/derivations/fixture-cart-sill-dock/`, one per row, written by
  N24b-04) are authoritative.
- The eight late rows carry local labels because the appendix assigns them no number; the numbers come from the
  block this bead requests of the allocation ledger (N38a-40). Until the block is issued a late row is identified
  by its label in `slug` and carries no number, as the other chapters' late rows are (the loader accepts no
  identifier that is not a KA number, N38a-12); from the mapping commit on it carries its number and keeps the
  label in `slug`.
- A change to any fixture constant creates a new fixture identity; the registry then gains new rows and keeps the
  old ones attached to the old identity.

## References

Appendix M.23 (rows KA-594 to KA-623); §24.16 (the chapter's eleven known answers); §24.5, §24.7, §24.8, §24.10,
§24.11, §24.12, §24.13; §38 (tiers G0 to G5; metamorphic relations MR-01, MR-13, MR-19, MR-20, MR-27);
PO-ARENA-02; GATE-ARENA-03 to -08; GATE-QUAL-01 to -03.
