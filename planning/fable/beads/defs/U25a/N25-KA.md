---
key: "N25-KA"
title: "Augur known-answer registry: KA-624 to KA-658 with truth, tolerance, tier and control; eighteen late answers awaiting numbers"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "augur", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-42"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§38.6 and §38.11: the loader and row schema of the known-answer registry and of the negative-control registry in frob-conformance (BD-146)", "Appendix M: suite numbers for the eighteen late known answers of §25.29 to §25.39 listed in the second table (the Gauntlet unit that owns the suite)", "§38.6 rule 1: the derivation files under docs/derivations/, one per fixture, shared with the rows of the constructions chapter that use the same fixture (KA-101, KA-113, KA-127, KA-154)", "§25.26: the known-answer battery beads of the chapter (U25d), which hold the tests these rows are bound to at M1, at M3 and M4, and at M6 and M8"]
related: ["N25-CX", "N25-REF", "BD-130", "BD-131", "BD-129", "BD-136", "BD-146", "BD-042", "BD-041", "BD-043", "N25a-08"]
gates: []
plan_refs: ["Appendix M.24", "Appendix M.1", "Appendix M.37", "Appendix M.38", "§25.26", "§25.22", "§25.27", "§25.29 to §25.39 (late known answers)", "§38.2", "§38.6"]
---
## Background

A known answer is an analytic or synthetic truth the implementation must recover: a case with a truth that a
test can compare against, the tolerance of the comparison, the verification tier and execution site on which
it runs, and the negative control that must make it fail (a gate that nothing can fail validates nothing).
The known-answer suite is the content of verification tier G1 and its rows are cited by gates at other tiers.
Numbers are append-only: a row is never renumbered and a number is never reused.

Augur, the subsystem of mission queries and inverse design (crates `frob-mission` and `frob-synth`), answers
questions about sets of robot designs with an inner set, an outer set and a checked reason per cell. Its known
answers are small exact fixtures: a stance family whose boundary is a rational, a pusher whose answer flips
with the order of two quantifiers, a docking game whose feasible region is one strict inequality, and their
relatives. Every number inside a fixture is an exact consequence of the fixture's stated model (DESIGN: the
fixture is a specification of a known answer and is hand-checkable from its equations). Numbers that report
what a plan-time pilot observed are FACT about a small program on a small model, to be re-run from the
planning reproduction bundle before a gate cites them. Thresholds inside a row are the TARGET of the gate
named beside it. No row is a measurement of this project.

The suite gives Augur thirty-five rows, KA-624 to KA-658. Eight state no negative control and one states no
tolerance; such a row is an obligation and not a test, and cannot support a gate until the missing part is
registered. The rules added to the chapter later (§25.29 to §25.39) state eighteen further known answers that
have no suite number yet; they are registered here under provisional tags so that none is lost. Tier Solid.
Mechanism beads cite these numbers and restate the values they need; this bead holds the rows as data and
binds each to the test that realizes it.

## Scope

In: the thirty-five rows in `registries/known-answers/augur.toml` (the schema of the suite's other chapters);
the eighteen late rows in the same file under tags `late-01` to `late-18` until numbers are assigned; the
binding of each row to a test identifier; a registered control for each of the eight rows that state none and
a tolerance for the one row that states none (DESIGN proposals of this bead, marked as such); the registry
check; the derivation outlines below. Out: the tests themselves (owned by the beads named in the last
column); the counterexample and refusal registries (N25-CX, N25-REF); known answers of other chapters.

## Technical approach

Row schema: `id`, `stated_in`, `case`, `truth`, `tolerance`, `tier`, `site`, `negative_control`, `exercises`
(the verb or proof kind the fixture is for), `same_fixture_as` (the mark "≡ KA-nnn": an earlier row in another
chapter uses the same fixture; one tests the mathematics, the other the subsystem; the derivation file is
shared), `provenance` (`design` | `fact-pilot` | `fact-literature`), `test` (bead and test name).

Tolerances: `exact` for integers, rationals, Booleans, verdicts, labels and digests; "to the printed digits"
means half a unit in the last digit printed. Tiers: G0 laws and self-tests; G1 known answers; G2 parity with an
independent oracle; G3 metamorphic relations; G4 fault injection; G5 determinism across backends. Sites: `host`
(exact or double-precision arithmetic in a test), `auditor` (`frob-verify`), `referee`, `cuda`, `metal`. A row
runs first on the weakest site that can decide it and then on every tier its gate names.

**Numbered rows.**

| KA | Stated in | The case | The truth | Tolerance | Tier · site | Negative control | Exercises | Realized by |
|---|---|---|---|---|---|---|---|---|
| KA-624 | §25.13, §25.26 | Stance family: load 400 N, feet at (±1/3, ±b) m, 160 N ratings, centre-of-mass rectangle 150 mm × 75 mm | Admitted iff b ≥ 3/14 m; forces (160, 130, 110, 0) N at the (60, 30) N·m vertex; 165 N needed at b = 1/5 m | exact (rational) | G1 · host, auditor | The support-polygon-plus-total-capacity screen; a perturbed dual multiplier at b = 1/5 m | `mission.synthesize`, `lp-primal`, `lp-farkas`, `partition` | BD-130, N25a-21 |
| KA-625 | §25.22, §25.26 | Stance grid: u, v ∈ {2, 3, 4} per metre, ratings ∈ {0, 80, 160, 240}⁴ N, four vertices and centroid | 2,254 of 11,520 states feasible; 114 of 2,304 bodies cover the mission | exact (count) | G1 · host | The weak screen falsely accepts 1,596 bodies | Lowering; negative control for static stability | BD-130, BD-042, N25a-08 |
| KA-626 | §25.4.7, §25.26 | Passive equal springs at b = 1/4 m | (175, 115, 85, 25) N against the active allocation (155, 135, 105, 5) N and a 160 N rating | exact | G1 · host | A certificate that omits the authority realizing the allocation admits the body | Authority in certificates | BD-130, N25a-13, N25a-08 |
| KA-627 | §25.4.7, §25.26 ≡ KA-101 | Design-dependent load: feet (±1 − e, ±1) m, 0.5 N ratings, load W at the origin; the positive-x pair carries W·(1 + e)/2 | (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N and pass; the midpoint (0.45, 1.45) needs 841/800 N from a pair capped at 1 N | exact (rational) | G1 · host | A convexity claim outside an affine chart admits the midpoint | Chart and convexity admission | N25a-08, BD-130, N25a-15 |
| KA-628 | §25.4.2, §25.26 | Pusher y = max(0, g·u − d), g ∈ [1/2, 2], u ∈ [0, 2], d ∈ [0, 1/4], target y ∈ [3/4, 5/4] | Per-build command set: all of [1/2, 2]; shared command set: empty | exact | G1 · host, auditor | The prefix ∀g ∃u offered for the query ∃u ∀g is rejected (`FROB-REF-826` by the auditor; `FROB-REF-881` by the verb) | Quantifier order | BD-131, N25a-03, N25a-22 |
| KA-629 | §25.26 | Pusher battery, ten frozen cases | Twenty intervals, including `shared-tight`: shared set {1}; `both-ends`: [2/3, 5/4]; `branch-crossing`: shared [0, 3/8] | exact (closed ends, singletons) | G1 · host, auditor | Six corrupted certificates, six of six rejected | Closed ends, singletons, saturation branch | BD-131, N25a-22 |
| KA-630 | §25.26 | Persistent gain: out-and-back with g ∈ {1, 2} | Net 0 with a fixed gain; {−1, 0, 1} when the gain is resampled | exact | G1 · host | Resampling a declared-persistent parameter (`world.persistence`) | `world.persistence` | N25a-03, N25a-02 |
| KA-631 | §25.3.5, §25.26 | Twenty worlds, two stages each passing 19 of 20 | 18/20 if the failures differ, 19/20 if they coincide; 0.9025 only under independence | exact | G1 · host | Multiplying stage rates | Chance constraints | N25a-11 |
| KA-632 | §25.4.5, §25.26 ≡ KA-154 | Mandatory work: lift 8 kg over 12 mm against a 0.90 J store | 8·9.81·0.012 = 0.94176 J > 0.90 J, excluded; 0.05 J of approach energy removes the exclusion | exact to the printed digits | G1 · host, auditor | An energy budget with an omitted source | Rule 1 scope | N25a-07 |
| KA-633 | §25.10, §25.26 ≡ KA-113 | Binary sensor correct with probability 3/4, majority vote | 27/32, 459/512 and 3807/4096 for 3, 5 and 7 reads; 3/4 with a common fault bit; a sequential stop-at-gap-two rule reaches 9/10 in 16/5 expected reads; 225/256 when forced at six | exact (rational) | G1 · host | Independent coins assumed for a shared error bit | `alias-pair`, deadlines | BD-290 |
| KA-634 | §25.11, §25.26 (the text of the fixture is in §25.10) | Passive jaw: k = 1 and 4 N/mm, force bands [2, 4] and [7, 9] N, stroke 6 mm | Feasible iff max{2(c + 1), 7(c + 1/4)} ≤ x ≤ min{4(c + 1), 9(c + 1/4), 6}; K ≥ 28/17 N/mm; K ≥ 4/3 N/mm without the stroke cap | exact (rational) | G1 · host, auditor | Dropping the stroke cap | Shared-policy inner cell | BD-290 |
| KA-635 | §25.15, §25.26 | Docking, g ∈ {1, 2}, packet on time | Command sequences (1/4, 1/2, −3/4) and (1/4, 0, −1/4) reach (1, 0) | exact | G1 · host, auditor | The same bundle with quantifiers reordered | `finite-game` | BD-129 |
| KA-636 | §25.15, §25.26 | Docking, packet late | No common command vector; all 561 late-packet cells excluded | exact | G1 · host, auditor | none stated (proposed below) | `alias-pair` | BD-129, BD-134 |
| KA-637 | §25.15, §25.26 | Docking family | Admitted iff ε < (U + 1/2)/4 on U ∈ [1/2, 1]; of 561 cells 344 admitted, 217 excluded, 9 on the boundary; at U = 3/4, ε = 5/16 both worlds can read 15/16 | exact; strict comparator | G1 · host, auditor | A non-strict comparator admits the equality case | Strict comparators; unattained infimum | BD-129, BD-041 |
| KA-638 | §25.22, §25.26 | Docking with a third gain 33/32 at ε = 1/64 | No exact docking; best worst-case error 1/65 m | exact (rational) | G1 · host | A certificate reused outside its declared hypothesis support | Eligibility and withdrawal | BD-129, BD-041, N25a-16, N25a-26 |
| KA-639 | §25.14, §25.26 | Pad probe, four laws, 900 deterministic cases | 29.66 N compact; 24.72 N and 41.15 N broad; three-law probe 675 of 675 in class and 0 of 225 on the withheld law; direct measurement 900 of 900 | exact (counts); forces to the printed digits | G1 · host | A fourth mechanism that shares the probe's early trace reverses all 225 held-out decisions | Conditional witnesses | BD-291, BD-043 |
| KA-640 | §25.14, §25.26 | Pad family, 75 N pull | 300 g under law V, 240 g under law P at centred readings; an eccentric pull needs 43.125 N at e/s = 0.15; an admitted reading drops the three-compact bound to 73.79 N | to the printed digits | G1 · host | A certificate applied outside its reading set | Additivity and wrench balance | BD-291, BD-043 |
| KA-641 | §25.16, §25.26 | Coupled A/B on z ∈ [0, 1]: A succeeds iff z ≥ 3/5, B iff z ≥ 2/5 | Outcome (1, 0) is unreachable; B gains on [2/5, 3/5) | exact | G1 · host, auditor | Outcomes enclosed separately: (0, 0) and (1, 1) project onto {0, 1} for both and hide (1, 0) | `product-unreachable` | BD-359 |
| KA-642 | §25.17, §25.26 | Handoff bit | Pooled history wins both cases; every local deterministic choice loses one; a timely bit restores both | exact (verdict) | G1 · host | A late bit | Decision makers | BD-435 |
| KA-643 | §25.17, §25.26 | Shared bus: two fingers each need one powered tick; the bus powers one finger per tick | Zero one-tick schedules; two two-tick schedules | exact (count) | G1 · host | An additive peak rule prunes the feasible two-tick case; local motor limits are identical in both cases | Joint resource contracts | BD-360, BD-435, N25a-08 |
| KA-644 | §25.18, §25.26 | Three variants on commands (x, y) ∈ [0, 1]²: x ≥ 2/3, y ≥ 2/3, x + y ≤ 1 | Every pair intersects; the triple does not (x + y ≥ 4/3 against x + y ≤ 1). Two labels suffice, (2/3, 2/3) and (0, 0); one is impossible | exact | G1 · host, auditor | Pairwise compatibility taken as joint | Minimal conflict sets | BD-363, N25a-08 |
| KA-645 | §25.18, §25.26 | Three variants, three readings | Two labels per reading; three for a fixed label map across histories | exact | G1 · host | none stated (proposed below) | Label map across histories | BD-363 |
| KA-646 | §25.18, §25.26 | Station records {left, shared} and {shared, right} | No tag count suffices | exact (verdict) | G1 · host | An encoder premise assumed without a witness | Encoder premise | BD-363 |
| KA-647 | §25.18, §25.26 | Label deadline: error e = u − bᵢ − q − δ, bᵢ ∈ {−1/4, 0, 1/4} mm, q ∈ [−1/32, 1/32] mm, δ ∈ [−w, w] mm, success abs(e) ≤ 3/8 mm | Largest w is 3/32, 7/32 and 11/32 mm with one, two and three labels before commitment; 3/32 mm for any number delivered late | exact (rational) | G1 · host, auditor | Labels counted without their delivery instants | Deadlines | BD-363 |
| KA-648 | §25.7, §25.26 ≡ KA-127 | Ballast quotient | 735,471 placements fall into 24,546 classes; {0, 3} and {1, 2} must split | exact (count) | G1 · host | A quotient by the mechanical group that merges {0, 3} with {1, 2} | Mission-observable quotient | BD-357, BD-285 |
| KA-649 | §25.19, §25.26 | Differential-drive loop, half-track w, wheel radius ρ, wheel angle δ | Sideways displacement 4w·(1 − cos(ρδ/2w)); zero net yaw and advance; the reversed loop gives the opposite | to set (proposed below) | G1 · host, referee | none stated (proposed below) | Holonomy query | BD-425 |
| KA-650 | §25.19, §25.26 | Skid-steer base under a no-slip connection | ω = 0 | exact | G1 · host | Admitting the connection for a skid-steer base | Connection admission | BD-425 |
| KA-651 | §25.4.7, §25.26 | ẋ = u, u ∈ [0, 1] | Bracket rank one; negative x is unreachable from zero | exact | G1 · host | Bracket rank read as reachability | Finite witness rule | N25a-08, BD-425 |
| KA-652 | §25.21, §25.26 | Mass placement: 1 kg inside a ball of radius 1/10 m with principal moments 1/125 kg·m² | No placement: mean squared radius 3/250 m² is needed and at most 1/100 m² is available | exact (rational) | G1 · host, auditor | none stated (proposed below) | Support-aware inertia synthesis | the mass-placement lane of `frob-synth` (U25d) |
| KA-653 | §25.21.3, §25.26 | Three-pose dyad through (0, 0), (2, 0), (0, 2) | Pivot (1, 1), radius² = 2 | exact | G1 · host | none stated (proposed below) | Linkage lane | BD-361 |
| KA-654 | §25.21.3, §25.26 | Crank-rocker 1, 4, 3 on ground 4 | B = (11/5, ±12/5) at crank angle π; the two signs are different circuits | exact (rational) | G1 · host, auditor | A root accepted without a circuit certificate | Circuit certificate | BD-361, N25a-08 |
| KA-655 | §25.21.4, §25.26 | Stewart–Gough forward kinematics; 6R inverse kinematics | 40 and 16 solutions (FACT from the literature, to be re-read from the primary source) | exact (count) | G2 · host | none stated (proposed below) | Continuation labels | BD-437 |
| KA-656 | §25.12, §25.26 | Guarded strict predicate on 65,536 rows of the docking boundary | 34,590 admitted and 30,946 excluded, of which 1,820 are exact ties; no error after exact fallback; identical bytes on all placements | exact (count, digest) | G5 · host, cuda, metal | Unguarded single precision: 452 false admissions and 23,438 false exclusions on the near grid | Exact-integer and guarded kernels | BD-355, BD-030, N25a-28 |
| KA-657 | §25.22 | Docking family against a scenario-tree program | 51 linear programs agree with the exact answer | exact (verdicts) | G2 · host | none stated (proposed below) | Conventional comparator | BD-127, BD-129 |
| KA-658 | §25.22 | Stance classifiers against a conventional solver | 11,520 of 11,520 states agree between two exact classifiers; 102 of 102 spot checks agree | exact (verdicts) | G2 · host | none stated (proposed below) | Second lowering (PO-AUGUR-01) | BD-130, BD-042 |

Provenance: the rows whose truth quotes a pilot count are `fact-pilot` and are re-run before a gate cites them
(KA-625, KA-639, KA-640, KA-648, KA-656, KA-657, KA-658, and the pilot counts inside KA-637 and KA-638);
KA-655 is `fact-literature`; all others are `design`.

**Controls and the tolerance proposed for the rows that state none** (DESIGN of this bead; each is a mutation,
not a boundary behaviour, and each must be shown to fail before its row supports a gate):

| Row | Proposed control | Why it must fail |
|---|---|---|
| KA-636 | A lowering in which the late packet is usable before the second command | It returns the on-time answer (344 admitted cells) where every cell is excluded |
| KA-645 | A solver that minimizes labels per reading and reports that minimum for the fixed-map question | It answers two where a map fixed across histories needs three |
| KA-649 | A loop integrator that ignores the orientation of the loop; and the small-angle form 2w·θ² with θ = ρδ/2w evaluated at θ = 1 | The reversed loop must give the opposite sign; 2w differs from 4w·(1 − cos 1) ≈ 1.8388w by far more than the tolerance |
| KA-652 | A synthesizer that checks total mass and the three principal moments and drops the support (the ball) | It returns point masses at radius √(3/250) ≈ 0.1095 m, outside the 0.1 m ball |
| KA-653 | A solver that returns the centroid (2/3, 2/3) of the three poses as the pivot | The squared distances are 8/9, 20/9 and 20/9, not equal |
| KA-655 | A tracker with one path deliberately dropped | It reports 39 roots; the trace test must disagree and the result must be labelled `Partial`, never `Complete` |
| KA-657 | One of the 34 optimal programs given the actuator bound of its neighbour (U + 1/32) | Its optimum moves by 1/32 (least first command) or 1/64 (greatest first command) and no longer equals the exact value for its cell |
| KA-658 | The weak screen (support polygon and total capacity) substituted for the second classifier | It disagrees on the states of 1,596 bodies |

Tolerance proposed for KA-649: absolute error at most 10⁻⁹ m on the sideways displacement on the host, and
loop-reversal and chart-change residuals below 10⁻⁸ m on the referee, which are the thresholds of the holonomy
gate (GATE-AUGUR-20, TARGET); the derivation file must state the fixture's w, ρ and δ.

**Derivation outlines** (each goes on file as `docs/derivations/<fixture>.md`; all were recomputed for this
bead and agree with the printed values).
- KA-624. Write u = 3 and v = 1/b per metre, r = (W + u·P_x)/2 for the two +x feet and s = (W + v·P_y)/2 for the
  two +y feet. With t the force on the ++ foot the solutions are (t, r − t, s − t, W − r − s + t), feasible iff
  max{0, r − c, s − c, r + s − W} ≤ min{c, r, s, c + r + s − W} for rating c. At (W, P) = (400, 60, 30):
  r = 290, s = 200 + 15v, and the binding pair is 90 + 15v ≤ 160, so v ≤ 14/3 and b ≥ 3/14 m; at v = 14/3 the
  forces are (160, 130, 110, 0) N; at v = 5 the ++ foot needs 165 N.
- KA-625. The same inequality with four independent ratings, over 9 geometries × 256 rating vectors × 5 loads:
  2,254 feasible states and 114 bodies feasible at all loads. The weak screen reduces to "ratings sum to at
  least 400 N" on this grid (the load is always inside the support polygon): 190 of 256 rating vectors, 1,710
  bodies, of which 1,710 − 114 = 1,596 are wrong.
- KA-626. Four equal springs on rigid ground add f₊₊ + f₋₋ = f₊₋ + f₋₊, so fᵢ = W/4 ± u·P_x/4 ± v·P_y/4 =
  100 ± 45 ± 30 N at v = 4. Under independent forces t ∈ [150, 160] N and the midpoint gives (155, 135, 105, 5).
- KA-627. Moment balance about the origin gives the +x pair W·(1 + e)/2: 0.95 N at (0, 1.9) and at (0.9, 1);
  1.45 × 1.45 / 2 = 841/800 N at the midpoint, above the pair's 1 N.
- KA-628. For known g the command u = 1/g gives g·u = 1 and y ∈ [3/4, 1]. For one shared command, g = 1/2 needs
  u ≥ 2 and g = 2 needs u ≤ 5/8.
- KA-630. Out and back with a fixed gain: g − g = 0. Resampled per move: g₁ − g₂ ∈ {−1, 0, 1}.
- KA-631. Failed worlds disjoint: 20 − 2 = 18 pass; coincident: 19 pass; independent: (19/20)² = 0.9025.
- KA-633. Majority of n independent reads at 3/4: 54/64, 918/1024 and 15,228/16,384. Stop at a lead of two:
  success p²/(p² + q²) = 9/10 and 2/(p² + q²) = 16/5 expected reads. Forced at six with a fair guess on a tie:
  (9/16)·(1 + 3/8 + 9/64) + (1/2)·(3/8)³ = 900/1024 = 225/256.
- KA-634. With jaw compliance c = 1/K in mm/N and stroke x in mm the grip force on an object of stiffness k is
  x/(c + 1/k). The two bands give 2(c + 1) ≤ x ≤ 4(c + 1) and 7(c + 1/4) ≤ x ≤ 9(c + 1/4). Without the stroke
  cap the binding pair is 7(c + 1/4) ≤ 4(c + 1), c ≤ 3/4, K ≥ 4/3. With x ≤ 6: 7(c + 1/4) ≤ 6, c ≤ 17/28,
  K ≥ 28/17.
- KA-635 to KA-637. Three unit ticks from rest with acceleration g·u_k: v₃ = g·(u₀ + u₁ + u₂) and
  x₃ = g·(5u₀/2 + 3u₁/2 + u₂/2). (1/4, 1/2, −3/4) with g = 1 and (1/4, 0, −1/4) with g = 2 give (1, 0). A common
  vector gives positions s and 2s, which cannot both be 1. With first command p the actuator bounds give
  1 − U ≤ p ≤ (U + 1/2)/2 and the reading sets centred at p and 2p must be disjoint, p > 2ε, so
  ε < (U + 1/2)/4. Grid: U = 1/2 + i/32 for i = 0 … 16 and ε = j/64 for j = 0 … 32; admitted iff 2j < 32 + i,
  which holds in 344 cells and is an equality in 9. At U = 3/4, ε = 5/16, p = 5/8: readings 5/8 + 5/16 and
  5/4 − 5/16 are both 15/16.
- KA-638. Gains 1 and 33/32 with ε = 1/64 share the reading (65/64)·p, so they receive the same commands and
  end at c and 33c/32; min over c of max{abs(c − 1), abs(33c/32 − 1)} is 1/65 at c = 64/65.
- KA-641. (1, 0) needs z ≥ 3/5 and z < 2/5.
- KA-644. x ≥ 2/3 and y ≥ 2/3 give x + y ≥ 4/3 > 1; pairs meet at (2/3, 2/3), (2/3, 0) and (0, 2/3).
- KA-647. One command per label group; the worst error is half the spread of bᵢ in the group plus 1/32 plus w.
  One group: 1/4 + 1/32 + w ≤ 3/8, w ≤ 3/32. Two groups ({−1/4, 0}, {1/4}): w ≤ 7/32. Three: w ≤ 11/32.
- KA-652. The trace of the inertia tensor is 2·m·⟨r²⟩, so three moments of 1/125 need ⟨r²⟩ = 3/250 m², and no
  mass distribution inside a ball of radius 1/10 m exceeds 1/100 m².
- KA-653, KA-654. The circle through (0, 0), (2, 0), (0, 2) has centre (1, 1). With the crank tip at (−1, 0) and
  the rocker pivot at (4, 0): (x + 1)² + y² = 16 and (x − 4)² + y² = 9 give x = 11/5, y = ±12/5.
- KA-656. S = 2²⁸, H = 2²⁷; for i = 0 … 65,535, U_i = H + (i·65,537 mod (H + 1)) and
  E_i = floor((U_i + H)/4) + ((i mod 9) − 4)·k for k = 1 and k = 2,048; admitted iff 4·E_i < U_i + H.

**Late known answers, not yet numbered** (stated in full, with fixtures and derivations, in the beads of the
unit that owns §25.29 to §25.39; registered here by tag so that the registry is complete):

| Tag | Subsection, gate | Known answer | Tolerance | Stated in full by |
|---|---|---|---|---|
| late-01 | §25.29, GATE-AUGUR-25 | Two-outcome reversal: Σ ε = 0.98, the transported bound is 0.01, the claim is refused | exact | N25e-01, N25e-02, N25e-05 |
| late-02 | §25.29, GATE-AUGUR-25 | Ten-step chain with ε = 0.003 per step: a claim of 0.99 transports to at least 0.96 | exact | N25e-01, N25e-02, N25e-05 |
| late-03 | §25.30, GATE-AUGUR-26 | Banded sill mount: pass, fail, pass along the swept scalar; the licensed creep fill equals the exhaustive grid; an unlicensed fill is refused | exact (verdicts); 100 of 100 seeds | N25e-09 |
| late-04 | §25.31, GATE-AUGUR-27 | A leg holds 12 kg · 9.81 m/s² = 117.72 N at lever arm ℓ and moves the foot at 1.5 m/s through a reduction N with a motor of 2 N·m and 80 rad/s: constraints (117.72/2)·ℓ·N⁻¹ ≤ 1 and (1.5/80)·ℓ⁻¹·N ≤ 1. The multipliers λ = (1, 1) cancel both parameters and give 8829/8000 > 1, so no (ℓ, N) exists (176.58 W needed against 160 W). Control: with 2.5 N·m the product is 8829/10000, no certificate exists, and ℓ = 0.2 m with N = 10 satisfies both (0.94176 and 0.9375) | exact (rational) | N25e-10, N25e-12 |
| late-05 | §25.31, GATE-AUGUR-27 | Two-term relaxation cases (0.6, 0.6) and (0.9, 0.15) | exact | N25e-11, N25e-12 |
| late-06 | §25.32, GATE-AUGUR-28 | A 10 kg carrier lifts payload p through 2 m with 300 J stored and arrival at up to 1.5 m/s; utility p/10 kg. Sound bound: 10 + p ≤ 300/(19.62 − 1.125) = 16.22 kg, upper utility 0.622. The relaxation without approach energy gives 15.29 kg and 0.529 and issues a false dominance stop against a competitor at 0.60: p = 6 kg is feasible with a run-up, 16 × 9.81 × 2 = 313.92 J ≤ 300 + ½·16·1.5² = 318 J | to the printed digits | N25e-13, N25e-14, N25e-16 |
| late-07 | §25.33, GATE-AUGUR-29 | Rear-drive cart, wheelbase 1 m, 15 kg fixed at 0.5 m from the rear axle and a 5 kg battery at x_b, μ = 1/2, g = 981/100 m/s²: at 3 m/s² feasible exactly when x_b ≤ 35/654 m; the largest acceleration over all placements is 3.066 m/s²; at 4 m/s² infeasible for every placement, with the traction inequality as dual; the same motion is feasible with all wheels driven (4.905 m/s²) | exact (35/654); printed digits otherwise | N25e-18, N25e-20, N25e-21 |
| late-08 | §25.33 | τ²/N² is not convex: 1/4 > 2/9 at the midpoint | exact | N25e-18, N25e-20 |
| late-09 | §25.34, GATE-AUGUR-30 | On [0, 1]²⁰ with m = (1/19)·Σ_{i=2..20} (2·xᵢ − 1) and δ = 0.01·m², require x₁ + δ − 0.8 ≤ 0 and 0.2 + δ − x₁ ≤ 0. With split set {x₁}: x₁ < 0.2 excluded, [0.2, 0.21) unresolved, [0.21, 0.79] inner, (0.79, 0.8] unresolved, x₁ > 0.8 excluded. Five slabs: 0.58 of the volume inner, 0.40 excluded, 0.02 unresolved | exact | N25e-23, N25e-24, N25e-25 |
| late-10 | §25.34, GATE-AUGUR-30 | Negative control Σ xᵢ − 10 ≤ 0 on [0, 1]²⁰: unresolved for the whole box after one enclosure and no bisection | exact (verdict) | N25e-23, N25e-24, N25e-25 |
| late-11 | §25.35, GATE-AUGUR-31 | Stance family under three law classes: `adjustable` at b = 3/14 m is inner with the bilinear law (100 N per foot at P = 0); `affine` at b = 1/2 m is inner with (160, 130, 70, 40) N at the vertex (60, 30); `affine` at b = 1/4 m is excluded (the symmetric law needs 175 N); `constant` is excluded for every b. Mutations that must fail: the four vertex commands labelled `affine`; a hull certificate with no law object; a constant command accepted because each vertex has some feasible command | exact (rational) | N25e-28, N25e-29, N25e-30 |
| late-12 | §25.36, GATE-AUGUR-32 | Docking to a tolerance over the gain interval [1, 2] at U = 3/4, ε = 1/64: least tolerance δ* = (√1745 − 41)/32 m, the root of 16·δ² + 41·δ − 1, about 24.2 mm; p* = (39 + √1745)/128; inclusion at 1/40, exclusion at 1/50; 133,185 exact cases | enclosure no wider than 10⁻⁹ m; exact for verdicts and counts | N25e-31, N25e-32, N25e-33 |
| late-13 | §25.37, GATE-AUGUR-33 | Seven repeated reads: 3807/4096 independent, 3/4 with a shared fault, 55/64 under the pairwise law (128 histories) | exact (rational) | N25e-34, N25e-36 |
| late-14 | §25.37, GATE-AUGUR-33 | Three labels: 2, 3 and 5 bits under clean, erasure and flip channels, with codes; four bits under one flip excluded by counting | exact | N25e-35, N25e-37 |
| late-15 | §25.38, GATE-AUGUR-34 | Margin table: 4.50, 4.72 and −13.06 N with the nuisance parameter shared; 0.28, −0.078 and −15.08 N with independent extremes | 10⁻³ N | N25e-38, N25e-40, N25e-41 |
| late-16 | §25.38, GATE-AUGUR-34 | Four-law probe: r = 3 at a dwell of 1/2 s with gap 0.1090; 900 of 900 decided; no probe at 1/4 s or less; ratio 5/3 | exact (counts); printed digits for the gap | N25e-39, N25e-40, N25e-41 |
| late-17 | §25.39, GATE-AUGUR-35 | Pack fixed point: 100 kg; an increment-stopping rule returns 99.5 kg after 527 steps of 5 g, where F(99.5) = 99.505 | within 1 g of 100 kg; exact for the mutant | N25e-42, N25e-44 |
| late-18 | §25.39, GATE-AUGUR-35 | Builds (0, 6), (4, 4), (6, 0): (4, 4) minimizes no weighted sum; a budget of four selects it | exact | N25e-43, N25e-44 |

**The registry check** (`xtask check-contracts`, extended by this bead's rows only): every row names an
existing test; every test tagged with a KA identifier names an existing row; a row with `negative_control`
stated has a registered control test that fails when the defect is planted; rows marked "none stated" and "to
set" are listed and reported as unable to support a gate until their proposed control or tolerance is
registered; a late row is reported `unnumbered` and cannot be cited by number.

## Acceptance criteria

- [ ] `registries/known-answers/augur.toml` holds exactly KA-624 to KA-658 with the columns above, and the
      eighteen late rows under their tags.
- [ ] Every row is bound to at least one existing test; the check passes at the milestone where the row's
      realizing bead lands and reports `not run` rows before that.
- [ ] Every row with a stated negative control has a registered control test that fails when the defect is
      planted.
- [ ] The eight rows with no stated control (KA-636, -645, -649, -652, -653, -655, -657, -658) are listed by the
      check; each gains the proposed control, or a better one, before a gate cites it; KA-649 gains a tolerance.
- [ ] The four shared fixtures (KA-627 with KA-101, KA-632 with KA-154, KA-633 with KA-113, KA-648 with KA-127)
      point at one derivation file each.
- [ ] A derivation file exists for every `design` row; every `fact-pilot` row names the path in the planning
      reproduction bundle from which it is re-run.
- [ ] No identifier is renumbered; printed values are carried unchanged.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_registry_augur.rs`: `augur_rows_624_to_658_present`; `each_row_has_test`;
`stated_controls_registered`; `none_stated_rows_listed` (expects the eight rows above before controls are
added); `to_set_rows_listed` (expects KA-649); `late_rows_are_unnumbered_and_listed` (eighteen);
`shared_fixture_marks_resolve` (four); `row_missing_test_fails_check` (the check's own negative control: delete
one binding). Recomputation tests on the host, in exact rationals, for the derivation outlines:
`stance_boundary_is_3_over_14`; `stance_grid_counts_2254_114_1596`; `passive_springs_175_115_85_25`;
`midpoint_needs_841_over_800`; `pusher_per_build_full_shared_empty`; `majority_27_32_459_512_3807_4096`;
`gap_two_rule_9_10_in_16_5_and_225_256`; `jaw_stiffness_28_17_and_4_3`; `docking_grid_344_217_9`;
`third_gain_error_1_over_65`; `label_deadline_3_7_11_over_32`; `mass_placement_3_250_exceeds_1_100`;
`crank_rocker_11_5_12_5`; `boundary_screen_34590_30946_1820`.

### End-to-end script and logging

`scripts/e2e/augur_ka_registry.sh` runs the registry check and the recomputation tests through
`cargo test -p frob-conformance` and prints the table of rows with their status (`bound`, `not run`,
`no control`, `no tolerance`, `unnumbered`). Logging through the helper of STD-E2E to
`target/e2e-logs/augur_ka_registry/<utc>.ndjson`: one `assert` line per row with its identifier, status and
bound test; `metric` lines for the counts (35 numbered, 18 late, 8 without control, 1 without tolerance); a
`summary`. The rows themselves are asserted by the scripts of their realizing beads (`mission_stance.sh`,
`mission_pusher.sh`, `mission_admission.sh`, `mission_family_screens.sh` and the scripts of the other units),
each of which logs the KA identifiers it asserts.

### Known answers and negative controls

This bead is the registry. Its own negative controls: the deleted binding; a row whose printed value is edited
(the recomputation test for that row must fail); a late row given a number by hand (the check refuses a number
outside the suite's range). Determinism: every numbered row except KA-639, KA-640 and KA-649 is an exact
rational, count, verdict or digest.

## Considerations

- Values are not improved in transit: where a recomputation disagrees with a printed value the row keeps the
  printed value, is listed among the unreconciled values of the suite, and its gate cites the derivation file.
  One such entry touches this chapter: the timing ceiling of the pusher pilot, first printed as "about 1.46"
  from a ratio of 1.345 and a shared share of 94% (which gives 1.43), is corrected to 842.8 ÷ 575.7 = 1.46 with
  the share attribution withdrawn; the conclusion (the test had no power) is unchanged.
- KA-628 and its counterexample cite `FROB-REF-826` for evidence offered under the wrong prefix, while the
  chapter's refusal table assigns `FROB-REF-881`. Both are right in their place: 826 is the auditor's rejection
  of a bundle whose prefix differs from the query, 881 the mission verb's refusal of such evidence. The row
  lists both.
- KA-634 is filed under the subsection on transfer of control, while the passive-jaw fixture is described with
  the alias witnesses; the row keeps the suite's reference and adds the other.
- The pilot rows are facts about small programs on small models. None used the project's dynamics, contact or
  sensor pipeline, and every pilot's strong baseline tied or won; a row that reproduces says the implementation
  matches the fixture, nothing about a physical robot.
- New Augur known answers continue the suite's numbering at its end; they are never inserted here by hand.

## References

Appendix M.1, M.24, M.37, M.38; §25.22, §25.26, §25.27, §38.2, §38.6; GATE-AUGUR-01 to -04, -10, -11, -15 to -17,
-19, -20, -22, -25 to -35; GATE-GAUNT-01, GATE-GAUNT-02; OPEN-GAUNT-2, OPEN-GAUNT-3; ADR-GAUNT-02, ADR-GAUNT-06;
INV-GAUNT-07.
