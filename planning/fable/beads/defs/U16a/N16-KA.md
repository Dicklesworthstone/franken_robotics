---
key: "N16-KA"
title: "Known-answer registry of Kinesis: KA-338 to KA-364, with truths, tolerances, tiers, controls and owning beads"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "kinesis", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-14"
deps: ["BD-025", "N36-KA", "N38a-12", "N38a-40", "N38b-04", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N16a-12", "N16a-14", "N16b-02", "N16b-03", "N16b-05", "N16b-06", "N16b-07", "N16b-08", "N16b-09", "N16b-11", "N16b-14", "N16b-16", "N16b-17", "N16b-18", "N16b-21", "N16b-26", "N16b-28", "N16b-29", "N16b-30", "N16b-31", "N16b-32", "N16b-33", "N16b-34", "N16b-35", "N38b-27", "N38b-31", "N38b-47", "N38b-58"]
gates: []
plan_refs: ["Appendix M.15", "Appendix M.1", "Appendix M.37", "§16.16", "§16.19", "§16.20", "§16.21", "§16.22", "§16.23", "§16.24", "§38.6"]
---
## Background

A known answer is a case whose truth is fixed in advance: a formula, an exact count, a verdict, or an equality
of digests. The known-answer suite is the Gauntlet's tier G1, and every row has five things: the case, the truth
with the derivation steps, a tolerance with a reason, a tier and execution site, and a negative control (the
mutation or planted failure the row must catch). The plan's rule is that a gate which nothing can fail validates
nothing, so a row without a control, or with a tolerance still "to set", cannot support a gate until the missing
part is registered.

This bead is the registry of the dynamics chapter's rows: 27 rows numbered KA-338 to KA-364 (numbers are
append-only: never renumbered, never reused). It exists so that no known answer of the chapter is lost when the
mechanisms are implemented by different beads at different milestones. Mechanism beads cite these numbers and
restate the values they need; this bead holds the complete table, creates the registry records, and tracks
which rows are still obligations.

How to read a row. Tier is `G<k> · <site>`: G0 laws and self-tests, G1 known answers, G2 parity, G4 chaos, G5
determinism. Sites: `host` (exact or double-precision arithmetic in a test), `referee`, `strict` (`batch-cpu`
under the `strict` profile), `lockstep` (the throughput tiers), `cuda`, `metal`, `all tiers`. Tolerance `exact`
is for integers, rationals, Booleans, verdicts, labels and digests. Labels: fixture constants are DESIGN; a
truth derived from them by hand is DESIGN arithmetic with a derivation file under `docs/derivations/`; a
threshold inside a row is the TARGET of the gate named beside it; no row is a measurement of this project.

## Scope

In: one `[[ka]]` row per KA below in the suite's one file; the derivation-file stubs with the stated truths;
the list of rows that are obligations (no control, or tolerance to set) with the bead expected to complete each;
the chapter's known answers that have no number yet, entered by slug, with the request for their number block.
Out, with the bead that has it: the row schema, loader, five rules, computed eligibility and printout (N38a-12);
the number blocks (N38a-40); the seeding of controls from the stated control cells, which writes
`controls.toml` and fills each row's `control.nc` (N38b-05; this bead writes no row of `controls.toml`);
running rows on tiers and reporting per site (N38a-13); implementing any row (the owning beads); rows that other
chapters number but this chapter's mechanisms use (listed for reference only).

## Technical approach

1. Append one `[[ka]]` row per KA to `crates/frob-conformance/registry/known_answers.toml`, the suite's one
   file, in the row schema of N38a-12 (no chapter file: the loader refuses any other `known_answers*.toml`, or a
   `.toml` under a `known_answers` directory, with `ka-second-file`). The columns map as: "Where stated" →
   `section`, with `owner` = the gate in the "Gate" column (or the invariant, as INV-KIN-06 for KA-354);
   "The case" → `case`; "The truth" → `truth`; "Tolerance" → `tolerance` (`exact`, `abs`, `rel`,
   `printed-digits`, `gate` or `to-set`); "Tier · site" → `tier` and `sites`; "Negative control" →
   `control.text` (`none stated` where the column says so); "Owning bead" → the derivation file's front matter;
   plus `fixture`, `same_as`, `label` (DESIGN for the hand derivations), `due` and `printed`. The row stores no
   status: eligibility (`eligible`, `obligation: tolerance`, `obligation: control`, `obligation: derivation`,
   `flagged: unreconciled`) is computed by the loader. DESIGN, correcting an earlier draft of this bead that
   kept a chapter file with its own fields (`where_stated`, `gate`, `owner_bead`, a stored `status`).
2. Create one derivation file `docs/derivations/<fixture>.md` per fixture (rows that share a physical case
   share the file), with the truth as stated here; its front matter lists the KA identifiers it serves and, for
   each, the test that checks it and the owning bead. The owning bead completes the error budget.
3. Controls: N38b-05's seeding turns each stated control cell into a control with `seed` = the row's number,
   reusing the control of a Kinesis counterexample (CX-144 to CX-152, N16-CX) that has the same plant and
   target, and fills the row's `control.nc`. This bead writes the `control.text` from which that happens and
   no control record.
4. A test `registry::kinesis_rows_complete` fails if a row below is missing from the file, if a number is
   reused, or if a field of the schema is empty; the loader's computed eligibility is checked against the
   obligation counts below.

### The 27 rows

| KA | Where stated | The case | The truth | Tolerance | Tier · site | Negative control | Gate | Owning bead |
|---|---|---|---|---|---|---|---|---|
| KA-338 | Known answers of the chapter | Ballistic drop from height z₀ onto a plane under a transactional profile | Impact time √(2·z₀/g); outgoing velocity from the law's restitution; remaining flight by the same formula | Impact time within 10⁻⁹ s | G1 · referee, strict | An endpoint-only gap check | GATE-KIN-04 | BD-082 |
| KA-339 | Transactional step; known answers | Two 1 kg sliders joined by a rigid tie reach a wall together at 1 m/s | Wall impulse 2 N·s; both at rest afterwards | Exact to 10⁻¹² N·s | G1 · referee, strict | Solve the wall contact for one slider (1 N·s), then impose the tie: 1 kg·m/s shared by 2 kg, both move into the wall at 0.5 m/s | GATE-KIN-04 | N16a-07, BD-082 |
| KA-340 | Transactional step (PO-KIN-04) | Dyadic refinement of a Brownian driver: root increment S = √ℓ₀·Z₀; a node with increment S over length ℓ gives its left child S/2 + √(ℓ/4)·Z and its right child the remainder | Each half-interval increment has variance ℓ/2; the halves are uncorrelated; their sum equals the parent | Inside 3σ at 10⁶ draws | G1 · referee, strict | Redraw on rejection: with a fair sign rejected when negative and redrawn once, P(positive) = ½ + ½·½ = ¾ | GATE-KIN-04 | N16c-01 (the law and its statistical test), BD-083 (the law under retry) |
| KA-341 | Transactional step (INV-KIN-02) | Forced retry: a transaction that commits on attempt k from checkpoint S | Its digest equals that of a clean run started from S with plan k, on 10⁴ random intervals | Exact (digest) | G5 · referee, strict | Packet queues or warm starts omitted from the restore set | GATE-KIN-04 | BD-083 (warm-start control in BD-081; packet-queue control in N16c-03, which links `frob-sense`) |
| KA-342 | Transactional step (INV-KIN-01) | Faults injected between the checks (T5) and the commit (T6) of a transaction | Zero observed uncommitted reads in 10⁵ injected faults: every read names a `commit_epoch` | Exact (count) | G4 · referee, strict | A reader that sees a candidate interval | GATE-KIN-04 | BD-081 |
| KA-343 | Transactional step | Resolution of the dyadic event grid at L_max = 16 and ℓ₀ = 2 ms | 2 ms / 2¹⁶ = 31 ns, so an approach at 3 m/s is located to within 0.1 µm | (set: N16a-06) exact arithmetic to the printed digits (printed: to set, DESIGN constants, OPEN-KIN-5) | G1 · host | (set: N16a-06) A grid built with L_max = 12, which prints 488 ns and must fail the row | — | N16a-06 |
| KA-344 | Batched step (INV-KIN-04) | Padding and co-tenancy: one world at buffer capacities 32, 64 and 128, solo and in 100 random co-tenancies, capacities and slot assignments | Identical digests, over 1,000 episodes per back end; the path-table Jacobian gives the same digests as the dense one | Exact (digest) | G5 · lockstep, cuda, metal | A reduction tree shaped by capacity | GATE-KIN-05 | N16a-12, N16a-13 (devices: BD-181) |
| KA-345 | Batched step (PO-KIN-02) | Cholesky factor of a system padded with identity rows and a zero right-hand side | The leading block of the padded factor equals the unpadded factor bit for bit | Exact (bits) | G0 · all tiers | (set: N16a-12) A factorization that pivots on the largest remaining diagonal, which lets a padded 1.0 enter the real block's order | — | N16a-12 |
| KA-346 | Batched step (PO-KIN-03) | Pairwise sum aligned to powers of two with the real entries in a prefix and trailing −0.0 entries | Equals the sum over the real prefix, for every capacity (with +0 entries it fails when the prefix sums to −0.0) | Exact (bits) | G0 · all tiers | A grouping that depends on capacity: in `f32` with a = 10⁸, b = −10⁸, c = 1, (a + b) + c = 1 and a + (b + c) = 0, because the spacing near 10⁸ is 8 | — | N16a-12 |
| KA-347 | Batched step | A batch of P parameter variants of one tree (256 per reference robot at the gate) | Equal to P homogeneous runs bit for bit on `batch-cpu` and on each GPU back end | Exact (bits) | G5 · lockstep, cuda, metal | The parameter row read after its first use in the step | GATE-KIN-06 | BD-169 (devices: BD-183) |
| KA-348 | Batched step | Bytes per world in `f32` with a dense Jacobian: 4·(61·n_b + 17·n_v + 3·n_v² + 24·C + R·n_v + 12·R + 10·n_v) | As printed in the suite: quadruped (n_v 18, n_b 13, C 24, R 128): 27 kB; hexapod (24, 19, 32, 168): 41 kB; humanoid (35, 30, 40, 224): 72 kB; wheeled base (10, 7, 16, 72): 12 kB; wheel-leg (22, 17, 32, 160): 37 kB; tracked skid-steer (12, 8, 64, 264): 37 kB. At the capacities the chapter now prints: quadruped (C 32, R 160) 31 kB; humanoid (C 64, R 320) 92 kB; tracked 36 kB | Exact arithmetic, printed to the kilobyte | G1 · host | (set: N16a-14) The Jacobian term computed as R·n_b (links instead of coordinates), which gives 28,076 B for the quadruped at C 32, R 160 and fails | — | N16a-14 (end to end N16a-18) |
| KA-349 | Batched step (INV-KIN-05) | Forced contact or row overflow at every capacity | Every overflow is flagged `Degraded` or refused (`FROB-REF-722`); no contact is dropped silently; `Degraded` world-steps are at most 10⁻⁴ of world-steps per class on the standard battery | Exact for the flag; 10⁻⁴ for the rate | G4 · lockstep, cuda, metal | A silent truncation of the contact set | GATE-KIN-07 | BD-159, N16a-14 |
| KA-350 | Promotion rule | Promotion table at p₀ = 0.05, p₁ = 0.30, α⁺ = 0.01, from ln E⁺ = (n − k)·ln((1 − p₀)/(1 − p₁)) + k·ln(p₀/p₁) | A clean pair multiplies the evidence by 1.36 and an exceedance divides it by 6; promotion takes 16 clean pairs, or 22 with one exceedance; at α⁻ = 0.05 two exceedances in the first three pairs reject | Exact (integer table, regenerated and compared in CI) | G1 · host | An envelope chosen after the outcomes | — | Promotion-rule bead of the fidelity work package (BD-263) |
| KA-351 | Tier correction (PO-KIN-05) | Doubly robust tier correction Zᵢ = mᵢ + Aᵢ·(Y¹ᵢ − mᵢ)/pᵢ | With pᵢ = 1 it equals the plain paired mean; with mᵢ = 0 it is the inverse-probability mean; with a constant tier offset it recovers the offset; type-I error at most α over 10⁴ null comparisons with optional stopping; a planted between-tier reversal is not established | Exact for the identities; α for the error rate | G1 · host | Audit probabilities logged after the draw | GATE-FID-08 | BD-264 |
| KA-352 | Tier correction | Higher-tier episodes saved by the tier correction, 1/(1 − ρ²) | 1.3 times fewer at ρ = 0.5, 2.0 at 0.7, 5.3 at 0.9, 10.3 at 0.95 | Exact arithmetic, printed to two figures | G1 · host | None stated | — | BD-264 |
| KA-353 | Model-form rank check | Model-form rank check on planted winners | The planted pyramid-cone winner (a pyramid grants √2·μ along its diagonals, 41% more than the cone) and the planted penetration-recovery winner return `model-conditional`; a genuine winner returns `model-robust`; cost at most 10% of the campaign's compute | Exact (verdict) | G1 · lockstep, strict | The two planted winners are the controls | GATE-FID-05 | BD-266 |
| KA-354 | Energy audit (INV-KIN-06) | Energy audit of a position-level penetration correction that removes depth d in one step, at m = 1 kg, d = 1 mm, h = 2 ms | The body gains d/h = 0.5 m/s, 0.125 J per event, from nothing, and raises the flag `energy-created` within one episode; CL-1 on the same penetration at k = 10⁵ N/m stores 0.05 J, returns less and creates none; an honest policy under CL-1 is unflagged | To set for the energy; exact for the flag | G1 · lockstep, strict | The residual folded into contact dissipation | GATE-FID-06 | BD-208 (check-level fixture in N16a-08) |
| KA-355 | Integrators; known answers | Free-body variational step (kick, drift, kick with the Cayley retraction) | Order 2 by step halving (the energy error falls by a factor between 3.6 and 4.4 per halving of h); reversal to round-off; discrete momentum map conserved without potential or input | Free-flight momentum to 10⁻¹² relative over 10⁶ steps; Legendre solve to 10⁻¹³ relative (TARGET) | G1 · referee, strict | An additive quaternion update | GATE-KIN-01 | BD-087, BD-088, BD-384 |
| KA-356 | Known answers; gates | Pendulum period at h = 1 ms | The closed-form period (carried from the earlier gate; the suite does not print the expression: T = (4/ω₀)·K(sin(θ₀/2)), ω₀ = √(m·g·ℓ/I_p), K the complete elliptic integral of the first kind, is the form the derivation file states; BD-088's fixture: a solid sphere of 1 kg and radius 0.05 m at ℓ = 1 m, I_p = 1.001 kg·m², θ₀ = 60°, K(½) = 1.6857504, T = 2.15395 s, measured over 20 periods at h = 1 ms) | Within 10⁻⁶ relative | G1 · referee | (set: BD-088) The small-angle period 2π/ω₀ used as truth (7.3% short at 60°); a stepper with h = 10 ms | GATE-KIN-01 | BD-088 |
| KA-357 | Known answers; gates | Symmetric-top precession | The closed-form precession rate (carried from the earlier gate; steady precession satisfies I₁·Ω²·cos θ − I₃·ω₃·Ω + m·g·ℓ = 0, so Ω = m·g·ℓ/(I₃·ω₃) at θ = 90°; BD-088's fixture: m = 1 kg, ℓ = 0.1 m, I₃ = 2.0 × 10⁻³ kg·m², I₁ = 1.1 × 10⁻² kg·m², ω₃ = 100 rad/s, θ = 90°, Ω = 4.905 rad/s) | Within 10⁻⁴ | G1 · referee | (set: BD-088) The coadjoint taken with the wrong sign (ξ ↦ −ad_ξᵀ in the Euler–Poincaré equation), which reverses the gyroscopic terms | GATE-KIN-01 | BD-088 |
| KA-358 | Known answers; gates | Ballistic invariants of the construction "dynamics is the Euler–Poincaré equation on a tree" | In ballistic flight the centre of mass follows a parabola and angular momentum about it is constant; the change of angular momentum minus the external angular impulse about one fixed inertial origin is zero | As GATE-KIN-01 (BD-088 files the round-off bound with the derivation) | G1 · referee | (set: BD-088) A wrench moved between frames with the twist map (the moment arm is lost) | GATE-KIN-01 | BD-088 |
| KA-359 | Gates | Contact-free forward dynamics of each reference robot against the pinned oracle | Joint accelerations agree | Within 10⁻⁹ relative in `f64` | G2 · referee | (set: BD-089) Three: the velocity-product term dropped (exposed by non-zero sampled velocities); one link's inertia about the centre of mass where the frame origin is declared; a wrench moved with the twist map | GATE-KIN-02 | BD-089 |
| KA-360 | Gates | `batch-cpu` in `f64` against `fs-mbd`: kinematics, inverse dynamics, forward dynamics, mass matrix, on 10⁴ states per reference robot | Agreement | Within 10⁻¹⁰ relative | G2 · lockstep | (set: N16a-17) Three planted candidates, each failing only its row: one placement transform inverted (kinematics); the Coriolis term dropped (inverse dynamics at non-zero velocity); a non-symmetric mass matrix filled from an uninitialized buffer (mass matrix) | GATE-KIN-03 | N16a-17, BD-150, BD-159 |
| KA-361 | Background referee | Referee coverage arithmetic: GPU world-seconds per second are T·h | A humanoid batch at 1.7 M world-steps/s (the tolerance of §36.3 row 7) and h = 2 ms produces 3,400 and a quadruped batch at 2 M produces 4,000; a referee at 0.2 to 1.0 times real time per core on 24 cores produces 4.8 to 24; the share is about 0.14 to 0.71% (humanoid) and 0.12 to 0.6% (quadruped), and 2.4% where a device produces only 1,000 world-seconds per second | Exact arithmetic on the performance TARGETs | G1 · host | None stated | — | BD-214 |
| KA-362 | Background referee | Episodes per tier to detect a five-point gap in success rate, 16·p·(1 − p)/δ² | 576 at p = 0.9 and 1,600 at p = 0.5; 11,520 to 32,000 world-seconds at 20 s per episode; 8 minutes to 1.9 hours of background referee time | Exact arithmetic | G1 · host | None stated | — | BD-214 |
| KA-363 | Background referee; gates | Background referee on planted defects | A planted single-kernel defect is localized to its stage from one-step defects; a planted tier-sensitive policy is detected within twice the predicted time; coverage at least 0.1% of GPU world-seconds; GPU throughput loss at most 5% | As GATE-FID-07 | G1 · cuda, metal | The two planted defects are the controls; KILL-SHADOW fires if neither is detected | GATE-FID-07 | BD-214, BD-268 |
| KA-364 | Promotion rule; gates | Promotion rule end to end on a planted tier-sensitive policy | The policy is caught and labeled `provisional: tier-sensitive` with both numbers | Exact (verdict) | G1 · lockstep, referee | The planted policy is the control | GATE-FID-02 | BD-263 |

Counts at plan time: 27 rows; 11 state no negative control (KA-343, -345, -348, -352, -356, -357, -358, -359,
-360, -361, -362); 2 have a tolerance to set (KA-343, KA-354). Each of those is an obligation on its owning
bead. The owners have registered controls for eight of them (KA-343, -345, -348, -356 to -360) and the
tolerance of KA-343; this registry carries those cells marked "(set: <bead>)", with the owner's reason in the
derivation file (N38b-27: a value an owner bead has set stands and is carried here). Still open: the controls of
KA-352, KA-361 and KA-362 (BD-264 and BD-214 name none; N38b-31, which depends on this bead, proposes one for
each: 1/(1 − ρ) giving 2.0 at ρ = 0.5; the step factor omitted, 1.7 M and a share understated 500-fold;
4·p·(1 − p)/δ² giving 144) and the energy tolerance of KA-354 (BD-208). N38b-31 also proposes second controls
for the eight rows the owners covered; those stay second records.

**One fixture per row: KA-356 and KA-357.** BD-088, the bead whose tests measure these rows, fixes the fixtures
carried above (the compound pendulum at θ₀ = 60°, T = 2.15395 s; the top at θ = 90°, Ω = 4.905 rad/s). The
derivation-file bead N38b-47, which depends on BD-088, writes the two derivation files at those fixtures
(BD-088's `pendulum-60` and `top_precession_horizontal`, Ω = 4.905 rad/s), so that the file and the test that
compares with it state one value: one row cannot have two values (N38b-47 applies the same rule to KA-269 and
adopts the fixture of the bead that tests it). N38b-47's tilted top (I₁ = 0.012 kg·m², ω₃ = 200 rad/s at θ = π/3,
slow root Ω = 2.5500 rad/s), where the fast-top shortcut m·g·ℓ/(I₃·ω₃) = 2.4525 rad/s is 3.8% low and is
caught, while at θ = 90° it is exact, is the stronger fixture and enters the same derivation file as a second
case only if BD-088 adds a simulation test for it. Until the derivation file exists and matches the measuring
test, the row is `obligation: derivation`.

**KA-361 shares the fixture `referee-coverage` with KA-821.** The performance chapter states the same referee
coverage arithmetic as KA-821 (N36-KA, M0): 1.7 M × 2 ms = 3,400 GPU world-seconds per second, and 24 cores at
0.2 to 1.0 times real time covering 4.8 to 24. N36-KA writes `docs/derivations/referee-coverage.md` with
KA-821's clauses (its `batch-cpu` clause among them). This bead gives KA-361 the fixture `referee-coverage`, adds
KA-361 and its clauses (the quadruped batch at 2 M, 4,000, and the shares 0.14 to 0.71% and 0.12 to 0.6%) to that
file's front matter and body, writes no second file for the case, and in the same commit sets KA-821's
`same_as = "KA-361"`, so that N38a-12's rule that `same_as` names an existing earlier row with the same fixture
holds at every commit. KA-361's own `same_as` is empty: it is the earlier row.

### Rows numbered by other chapters that Kinesis mechanisms use (reference only)

| KA | Case and truth | Used by |
|---|---|---|
| KA-047 | Ballistic flight of an articulated body: parabola of the centre of mass; constant angular momentum about it; zero origin diagnostic | BD-088 |
| KA-048 | Kinetic energy equals ½·q̇ᵀ·M·q̇; Ṁ − 2C is skew | BD-088, BD-150 |
| KA-049, KA-050 | Planar 2R chain (lengths 0.4 m and 0.3 m, angles 0 and π/2): tool point (0.4, 0.3) m; J = [[−0.3, −0.3], [0.4, 0.0]] m/rad; force (10, 0) N gives torques −3 N·m each; rates (0.2, −0.1) rad/s give velocity (−0.03, 0.08) m/s; power −0.3 W | BD-088, BD-150 |
| KA-051, KA-264 | Limb port packet: I − I·s·(sᵀ·I·s)⁻¹·sᵀ·I; diag(0, 1, 1, 1, 1, 1) for the identity and the first axis | BD-385 |
| KA-052, KA-053 | Cayley retraction lands on the group; residual within four units in the last place over 10⁷ steps | N16a-11 |
| KA-054, KA-172 to KA-174 | Free flight under the structure-preserving step; the inverse Cayley differential; the four-step map with and without gravity | BD-384, N16a-21 |
| KA-396 | Painlevé rod: the referee reports the indeterminate configuration (`FROB-REF-224`) and returns no number | N16a-07 |

### Known answers the chapter states that have no number in the suite

The six late subsections of the chapter state known answers that the suite's table for this chapter does not
number. They are listed here so that they are not lost, and this bead enters them as `[[ka]]` rows at the
finer count of the late subsections' own gate beads, 21 rows: the three known answers of row-class queues (the
two below and the planted tree-sizing kernel, which N16c-16 counts as its third), the two hand cases and three
planted inputs of decoupled stepping, the two branches of each four-bar, the three counts of the ancestor
ledger and the four rows of the held-command table are each one row. Numbers: this bead commits a request
`[[block]] registry = "ka", count = 21, owner = "N16-KA"` in `registry/allocations.toml` (N38a-40), cites the
rows by the slugs below until the block is issued, then maps the slugs to the issued numbers in table order in
one commit, each slug kept in its row's `slug` field; a slug cited in place of its number at the M1 exit fails
(`provisional-tag-past-due`). DESIGN, correcting an earlier draft that left the numbering to the owning beads
("the next free numbers") and kept the rows in a `pending` section the schema does not have: every number above
KA-835 is issued by N38a-40, and the chapter's rows are entered by its registry bead. The beads in the last
column own the mechanisms and the tests that the derivation files name. Slugs, in table order:
`rowq-digests`, `rowq-trot`, `rowq-tree-kernel`; `decouple-rest`, `decouple-headon`, `decouple-message`,
`decouple-lidar`, `decouple-prop`; `adjoint-scalar`, `adjoint-block`; `loop-par-open`, `loop-par-crossed`,
`loop-cr-open`, `loop-cr-crossed` (the refusal of the parallelogram at θ = 0 is part of the two parallelogram
rows); `ancestor-quadruped`, `ancestor-humanoid`, `ancestor-chain`; `held-exact`, `held-one-step`,
`held-two-step`, `held-delay-sweep`. The four planted-input rows have as truth that the plant is caught (a digest
changes; the tile ends at the message's arrival with clause `MessageArrival` or at the lidar's sample instant
with `SensorInstant`; the prop's island merges in a tile that starts before impact); their controls are the
mutants their gate beads name (`ignore-message-clause` for `decouple-message`, `velocity-only-bound` for
`decouple-rest`, N16c-21), and a row whose gate bead names none stays `obligation: control`.

| Subsection | Case | Truth | Control | Owning beads (mechanism or fixture, gate script) |
|---|---|---|---|---|
| Row-class solve queues | Digests with queues on | Equal to the masked path, bit for bit | A kernel that sizes a reduction tree by the class bound | N16c-13, N16c-16 |
| Row-class solve queues | Trot fixture, four samples per foot, C = 24 (class bounds 6, 12, 24) | Per-step assignment follows the alternation of 8 and 16 active contacts | A sort made at reset (caught by a truncation flag or by zero saving) | N16c-13, N16c-16 |
| Decoupled stepping | Two robots at rest, 0.10 m gap, each able to accelerate at 5 m/s² | Gap closes at t = √0.02 s = 0.141 s; a 20 ms tile is certified with γ = 0.098 m; a 0.2 s tile is not | A bound from current velocities alone | N16c-19, N16c-21 |
| Decoupled stepping | Head-on at 1.5 m/s each from a 3.0 m gap | Earliest contact at 1.0 s; fifty 20 ms tiles certified; merge when the gap falls below 0.06 m | A message with latency shorter than the tile; a lidar that sees the other robot; a thrown prop | N16c-19, N16c-21 |
| Step adjoints | Φ(v; z) = ½·m·(v − z)² + ½·g_c·v² | dv⁺/dz = m/(m + g_c): 1/4 at m = 2, g_c = 6; 1 with the contact open; one-sided values 1 and 1/4 at the boundary | A derivative returned without the label `one-sided` | N16c-25, N16c-27 |
| Step adjoints | 1 kg block, μ = 0.5, pushed for one step of 0.01 s | Breakaway 4.905 N; at 4 N, v⁺ = 0 and dv⁺/du = 0; at 6 N, v⁺ = 0.01095 m/s and dv⁺/du = 0.01 s/kg; regularized stick derivative about 2.0 × 10⁻⁴ s/kg | A zero gradient read as a stationary point | N16c-25, N16c-27 |
| Loop elimination | Parallelogram a = c = 1, b = d = 2 | Open branch: φ = θ, n = 1; crossed branch at θ = 90°: coupler end (1.2, −0.6), φ = −143.13° | The crossed-branch root for a knee assembled open | N16c-31, N16c-33 |
| Loop elimination | Crank-rocker (1, 3, 2.5, 3) at θ = 90° | φ = 98.857°, n = 0.3722 (open); φ = −135.727°, n = −0.1722 (crossed); the parallelogram at θ = 0 is refused | A loop driven through its toggle | N16c-31, N16c-33 |
| Ancestor-sparse assembly | Quadruped n = 18; humanoid n = 35; a chain | Factor counts 375 against 969 (2.58) and 1,742 against 7,140 (4.10); foot row 45 against 171 and 78 against 630; a chain's ratio is exactly 1 | A factor that skips the filled entry of the three-coordinate case | N16c-35, N16c-39 |
| Held command | Slider with u = −(3/2)·(x + w), 64 initial pairs, deadline tick 16 | Exact hold: radius 1/2, 64 of 64; one semi-implicit step: (1 + √3)/2 ≈ 1.366, 1 of 64; two steps: (5 + √57)/16 ≈ 0.784, 15 of 64; d = 3, 4, 8, 16, 32: radii 1/2, 1/4, 0.395, 0.451, 0.476, 64 of 64 | A study that halves the control period with the step | N16c-41, N16c-45 |

## Acceptance criteria

- [ ] `known_answers.toml` holds 27 `[[ka]]` rows KA-338 to KA-364 with every field of the N38a-12 schema and
      validates under its loader; no number is reused; no other known-answer file exists (`ka-second-file`
      reports nothing; verified at review: the only other path this bead names is the planted second file of its
      own negative control); the test `registry::kinesis_rows_complete` passes.
- [ ] After N38b-05's seeding, each of the 16 rows with a stated control has a non-empty `control.nc` resolving
      to a control attached to its gate; this bead wrote no row of `controls.toml`.
- [ ] The loader computes `obligation: control` for the 3 rows whose control is still open (KA-352, KA-361,
      KA-362) until N38b-31's records exist, and `obligation: tolerance` for KA-354, each listed with the bead
      expected to complete it; none is counted as passing by any gate until completed; the eight controls and
      the tolerance set by their owners carry the owners' text and are seeded from it.
- [ ] A derivation file exists for every fixture and lists its rows, tests and owning beads; the files for
      KA-356 and KA-357 contain the closed forms at the fixtures BD-088 measures.
- [ ] The block request for the 21 late rows is committed with this bead; the rows are cited by their slugs
      until the block is issued and by the issued numbers after; no slug is cited in place of a number at the M1
      exit.
- [ ] KA-348's record carries both capacity sets and the formula as the truth.
- [ ] KA-361's `fixture` is `referee-coverage`, `docs/derivations/referee-coverage.md` lists KA-361 beside
      KA-821, and KA-821's `same_as` is `"KA-361"`, set in the commit that adds KA-361; no second derivation file
      states the case.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_kinesis.rs` (host; the real registry and scratch copies, no mocks):
- `registry::kinesis_rows_complete`: 27 ids, contiguous, each with case, truth, tolerance, tier, site.
- `registry::no_reuse`: adding a record with an existing id fails.
- `registry::obligations_not_eligible`: the loader computes `obligation: control` for every row whose
  `control.text` is `none stated` and whose `nc` is empty, and no gate counts such a row.
- `registry::kinesis_late_slugs_mapped` (in milestone-exit mode, fails if a slug of the 21 late rows is cited in
  place of its number).
- `registry::arithmetic_rows` (the rows that are pure arithmetic are executed here on the host):
  KA-343: 2 ms / 65,536 = 30.517578125 ns and × 3 m/s = 91.55 nm; KA-348: the byte formula at each capacity
  set (26,668; 41,404; 71,752; 11,860; 37,164; 36,464; and 31,276; 92,104); KA-350: ln(0.95/0.70) = 0.30538 so
  16 clean pairs reach ln 100 = 4.60517 and 15 do not, and 21 clean plus one exceedance (ln(1/6) = −1.79176)
  reach it at n = 22; KA-352: 1/(1 − ρ²) = 1.33, 1.96, 5.26, 10.26; KA-361: 1.7 M × 2 ms = 3,400, 2 M × 2 ms =
  4,000, 4.8/3,400 = 0.141%, 24/3,400 = 0.706%, 4.8/4,000 = 0.12%, 24/4,000 = 0.6%, 24/1,000 = 2.4%; KA-362:
  16·0.9·0.1/0.05² = 576, 16·0.5·0.5/0.05² = 1,600, × 20 s = 11,520 and 32,000 world-seconds, ÷ 24 = 480 s and
  ÷ 4.8 = 6,667 s (8 minutes to 1.9 hours); KA-354: ½·1·0.5² = 0.125 J and ½·10⁵·(10⁻³)² = 0.05 J; the owners'
  controls: KA-343 at L_max = 12, 2 ms / 4,096 = 488.28 ns; KA-348's R·n_b mutant, 28,076 B; KA-346 in `f32`,
  spacing 8 at 10⁸, (a + b) + c = 1 and a + (b + c) = 0; KA-356 K(½) = π/(2·AGM(1, √3/2)) = 1.6857504 and
  T = 2.15395 s, 7.32% above 2π/ω₀; KA-357 0.981/0.2 = 4.905 rad/s.
- `registry::kinesis_fixture_single`: KA-356 and KA-357 each name one fixture, and its derivation file states
  the values BD-088's test uses.
- `registry::ka361_shares_referee_coverage`: KA-361 and KA-821 carry the fixture `referee-coverage`, KA-821's
  `same_as` is `"KA-361"`, the one derivation file lists both, and its constants 3,400 and 4.8 to 24 agree with
  KA-361's truth.

### End-to-end script and logging

`scripts/e2e/kin_registry.sh` (header: `bead=N16-KA milestone=M1 labels=kinesis,registry gates=GATE-GAUNT-02
plants=deleted-row,duplicate-row,unknown-nc,second-file`): sources the STD-E2E helper; runs
`cargo test -p frob-conformance --test ka_kinesis`, N38a-12's loader test and N38a-40's allocation test; then
selects the rows by number through their tier targets (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance
--test <tier> -- <row>`, `<tier>` = `g0_laws`, `g1_known_answers`, `g2_parity`, `g4_chaos` or `g5_determinism`)
and logs each row's per-site outcome as the runner reports it; a row whose owner has not landed is `unbound`,
never passed. It prints, as `metric` lines, the counts of rows, eligible rows, obligations by kind, rows with a
seeded `control.nc`, and the 21 late rows by slug or issued number, and asserts the counts above; it then plants
the four registry defects below on scratch copies under `target/e2e-scratch/kin_registry/` and requires each to
be caught. Log at `target/e2e-logs/kin_registry/<utc>.ndjson` with a human transcript: every line with `ts`,
`script`, `bead`, `step`, `event`, the exact command line, exit code and wall time; per row an `assert` line
(id, eligibility, sites, owner, status per site); the SHA-256 of `known_answers.toml`; on failure the first
differing field and the scratch path; a final `summary`. The rows themselves are exercised by the scripts of
their owning beads.

### Known answers and negative controls

This bead is the table. Its own negative controls, each on a scratch copy of the registry, never the committed
file: KA-349 deleted, or KA-340 duplicated (the completeness test fails); KA-356 given a `control.nc` naming no
control (the loader refuses it: every `nc` must exist); a copy of the chapter's rows written to
`registry/known_answers/kinesis.toml` (`ka-second-file`).

## Considerations

- Printed values are carried as printed and are not improved in transit; where a recomputation disagrees, the
  row keeps the printed value and the disagreement is recorded. For this chapter: KA-348 prints the capacities
  the chapter held before they were raised to the reference-workload values, and prints 37 kB for the tracked
  row where the formula gives 36,464 bytes; the registry records the formula as the truth and both sets.
- The suite's count of 27 rows excludes the late subsections; their known answers are real and gate-bearing and
  are numbered from the block requested here.
- The pendulum and top truths are named in the plan and not written; the closed forms and fixtures are BD-088's
  (recomputed at review), and N38b-47 writes the derivation files at those fixtures.

## References

Appendix M (M.1 how to read; M.15 the 27 rows; M.37 counts and gaps), §16.16, §16.19 to §16.24 (unnumbered
known answers), §38.6 (rules for a row), §38.11 (negative-control registry), CX-144 to CX-152.
