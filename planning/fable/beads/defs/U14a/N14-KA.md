---
key: "N14-KA"
title: "Known-answer registry of Skeleton (§14): KA-261 to KA-292 with truths, tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-07"
deps: ["BD-025", "N38a-12", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N13-22", "N13-KA", "N14b-03", "N14b-04", "N14b-18", "N14b-29", "N14b-33", "N14b-34", "N14bp1-01", "N14bp2-01", "N15b-03", "N29ax-01", "N38b-27", "N38b-30", "N38b-47"]
gates: []
plan_refs: ["Appendix M.13", "§14.5", "§14.6", "§14.10", "§14.11", "§14.13", "§14.14", "§14.15", "§38.6"]
---
## Background

Skeleton is the subsystem that owns what a robot is before anything moves: the design term (`frob-design`), the
model compiler (`frob-model`), import and export (`frob-io`), the eighteen archetypes and the staging of the
preflight `design.check`. A known answer (KA) is a case whose truth is computed by hand or in exact arithmetic,
independently of the code under test. The program's rule is that a gate which nothing can fail validates
nothing, so every known answer travels with a negative control: a planted defect that the test must catch.

This bead is the registry of the 32 known answers that belong to Skeleton. It exists so that no row is lost
when mechanism beads are written and closed one at a time: each mechanism bead cites the rows it needs and
restates the values inline, and this registry is the one place where all of them are listed with their owner.
A row is never regenerated from the candidate implementation: its truth is the hand value written here, or a
pinned oracle output captured out of process.

Tier vocabulary (from the verification program): G0 laws and self-tests; G1 known answers; G2 parity with a
pinned oracle; G3 metamorphic relations; G4 chaos (faults, cancellation, malformed input); G5 determinism. The
execution site `host` means exact or double-precision arithmetic in a test binary; `referee` is the most
trusted simulation tier (FrankenSim's articulated dynamics behind the airlock); `strict` is the `batch-cpu`
tier under the transactional, event-located step profile.

## Scope

In: the rows KA-261 to KA-292 in the suite's one registry file, with the case, the truth and its formula, the
tolerance, the tier and site, the negative control's text, and the bead whose test consumes the row; a derivation
file per fixture; the block request for the five late answers and their mapping; a meta-test that every row's
owner test exists once its owning bead has closed and that every control is registered.

Out: the row schema, the loader, its five rules and the printout (N38a-12); the number blocks (N38a-40); the
seeding of controls into the negative-control registry (N38b-05: this bead writes no row of `controls.toml`);
the tests themselves (each is in the bead named in the last column); the counterexample registry of §14
(`N14-CX`, written by the unit that owns §14.10 to §14.17); the refusal registry (`N14-REF`, same unit).

## Technical approach

File: the suite's one registry file, `crates/frob-conformance/registry/known_answers.toml`, to which each
chapter's registry bead appends its rows (N38a-12: no chapter keeps a file of its own, and the loader fails with
`ka-second-file` on any other; an earlier draft of this bead named `registries/known_answers/skeleton.toml`). One
`[[ka]]` row per KA in N38a-12's schema: `id`, `section` (with `owner` = the gate or obligation that owns the
row), `fixture`, `same_as`, `case`, `truth`, `tolerance` (`exact`, `abs`, `rel`, `printed-digits`, `gate` or
`to-set`), `tier`, `sites`, `control` (`text`; the `nc` list is filled by N38b-05's seeding tool), `derivation`
(`docs/derivations/<fixture>.md`), `label` (DESIGN for hand derivations, FACT for reported figures), `due`,
`printed`. The columns below map as: "Tier · site" → `tier`, `sites`; "Negative control" → `control.text`;
"Owner" → the derivation file, whose front matter lists the KA ids it serves and, for each, the test that checks
it (crate and test name) and the owning bead. A row whose tolerance is `to-set` is `obligation: tolerance` under
N38a-12's eligibility rule and is counted as passing by no gate until its derivation file states a tolerance
(OPEN: tolerances of real-valued truths; fail-closed).

The rows. "Exact" means exact rational comparison, a verdict, a bit comparison or a count, as the row says.

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Owner |
|---|---|---|---|---|---|---|
| KA-261 | Torso box 0.42 × 0.20 × 0.10 m at 2,700 kg/m³, read as solid and as a 2.5 mm shell | Solid: 0.0084 m³ × 2,700 = 22.68 kg (567/25 kg). Shell: inner box 0.415 × 0.195 × 0.095 m removes 0.007687875 m³, leaving 0.000712125 m³ and 1.9227375 kg (prints 1.923). Vertical-axis radius of gyration squared: (0.42² + 0.20²)/12 = 0.018033 m² solid, 0.023566 m² shell (31% larger). The solid reading is 11.8 times too heavy | exact rational (PO-SKEL-02, GATE-SKEL-07) | G1 · host | A solid default fill; density rescaled to the right mass in place of a shell (inertia stays wrong) | BD-062, N14a-27 |
| KA-262 | Mass spiral applied to the fill error (GB-03) | With loop gain κ = 0.59, each kilogram of structure costs 1/(1 − κ) = 2.4 kg of robot, so the 20.8 kg error of KA-261 becomes about 50 kg of robot | to set (illustrative) | G1 · host | none stated; this registry adds: κ taken as 0 (no spiral) must fail | N14a-10 |
| KA-263 | A shell thicker than half the Region's smallest dimension | Refused as a solid written as a shell, `FROB-REF-105` | exact (verdict) | G1 · host | none stated; this registry adds: the check compares against the largest dimension | BD-055 |
| KA-264 | Limb port packet after eliminating a joint with axis s from an articulated inertia I | I − I·s·(sᵀ·I·s)⁻¹·sᵀ·I sends s to zero. With I the 6×6 identity and s the first basis vector the result is diag(0, 1, 1, 1, 1, 1): symmetric positive semidefinite, 27 scalars (21 inertia entries, 6 bias entries; 108 bytes in single precision), equal to the limb's reaction from the full articulated recursion | exact for the fixture; round-off on random limbs, to set | G1 · host | A packet typed or treated as positive definite, or inverted at the port | N14a-22, N14a-23 |
| KA-265 | Raised threshold: wheel radius r = 0.060 m and 0.075 m at a lip h = 0.012 m | Lever arm √(2·r·h − h²) = 36.0 mm and 40.7 mm; horizontal push over axle load √(2rh − h²)/(r − h) = 0.750 and 0.646 | (set: BD-109) 10⁻⁹ relative against the closed form (`docs/derivations/raised-threshold.md`; to more digits 40.693980 mm and 0.645936) | G1 · host | A wheel edit judged by one of its two effects (CX-134) | BD-245; the `diff-drive` archetype bead (BD-109) |
| KA-266 | Ackermann base, wheelbase L, steer angle δ | Turning radius L/tan δ; growth vector (2, 3, 4) | (set: N14b-03) 10⁻⁹ relative for the radius (printed `to set`); exact for the growth vector | G1 · host | A steering linkage with the wrong closure (the tie rod on the wrong arm, which breaks cot δ_o − cot δ_i = w/L) | the `ackermann` archetype bead (N14b-03; its group row in BD-246); BD-106 |
| KA-267 | Differential drive over one step, δ = ω·Δt | Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)), sinc(0) = 1; growth vector (2, 3) and the sideways cost of GB-05 | to set for the chord; (set: BD-109) 10⁻⁶ relative for the sideways cost (0.178512 m and 0.553669 m at half-track 0.20 m for shifts of 0.01 and 0.10 m) | G1 · host | `fs-mbd::planar_drive` used as the oracle (reported defect near straight-line motion; the admission test is in §33, INT-FSIM-05 by BD-033) | BD-109, BD-033 |
| KA-268 | Mecanum base with all four roller axes parallel | Twist-to-wheel map of rank 2 instead of 3; growth vector (2, 3) where an omnidirectional base must have (3); preflight stage 4 refuses against the declared class (`FROB-REF-118`); a curvature test on the same base passes | exact | G1 · host | Curvature used as the wheel-layout test | BD-106, BD-110; rank row also in BD-245 |
| KA-269 | `balancer-2w`: two-wheeled inverted pendulum | The open-loop pole equals its closed form (the expression is not written in the chapter; it belongs to the derivation file of the archetype: `docs/derivations/balancer-2w-pole.md`, N38b-47, p = √(m_b·g·l/(I_b + m_b·l² − (m_b·l)²/M_e)) with M_e = m_b + m_w + I_w/r², 8.49628 s⁻¹ at the template's defaults m_b = 10 kg, l = 0.30 m, I_b = 0.20 kg·m², m_w = 2 kg, I_w = 0.01 kg·m², r = 0.10 m) | (set: N14b-03) 10⁻⁹ relative for the closed form; the referee's free response from a 0.01 rad tilt follows cosh(p·t) within 1% for t ≤ 0.2 s (2.826 at 0.2 s) | G1 · referee | none stated; this registry adds (the row's first control, as N38b-47 records): the pole evaluated with gravity reversed (the hanging pendulum, which has no real positive pole) must fail the check that the open-loop system has a real positive pole; second record (N14b-03's, registered by N38b-30): the wheel inertia I_w/r² dropped, which gives 9.1698 s⁻¹ | N14b-03 (the template and its fixture); N38b-47 (derivation); BD-246 (group row) |
| KA-270 | Γ_mech as built for the eighteen archetypes, before any sensor is mounted | `quad-mammal`, `quad-sprawl`, `mecanum`, `tracked-flipper`, `wheel-leg`: C₂ × C₂; `hexapod-radial`: D₆ (C₆ with handed legs); `octopod`: D₈ radial, C₂ in a spider layout; `biped-point`, `humanoid`, `diff-drive`, `ackermann`, `rocker-bogie`, `balancer-2w`: C₂; `omni-3`: D₃; `swerve-4`: D₄; `tri-wheel-stair`: C₂ with C₃ or D₃ in each cluster; `mobile-manipulator`: trivial with the arm off-centre; `snake-crawler`: C₂ under head-to-tail reversal (OPEN for this element, below) | exact | G1 · host | The 24 planted asymmetries of GATE-MARROW-03 | BD-102, N14a-11, BD-109 (first four), N14b-02, N14b-03, N14b-04 (the M3 archetypes), N14bp1-01 (`snake-crawler`), BD-246, N13-22 (GATE-MARROW-03) |
| KA-271 | Two fingers on a bus limited to one unit of current, each with peak demand one unit, drawing at different ticks | The schedule is feasible; the peak-adding rule concludes two units and excludes it | exact (verdict) | G1 · host | Exclusion from peak-only contracts | BD-060, BD-077 |
| KA-272 | Mirror swap of a leg module (a leaf-local substitution) | Every sibling product is reused and is bit-identical to its recompiled value | exact (bits) | G3 · host | An invalidation rule that is too wide; a reversed port sign must refuse | BD-077, N14a-30 |
| KA-273 | Edit of the shared bus (a global substitution) | Every electrical and thermal product is invalidated | exact (verdict) | G1 · host | An invalidation rule that is too narrow | BD-077, N14a-30 |
| KA-274 | Two range beacons at (−1, 0) and (1, 0); robots at (0, 1) and (0, −1); the task requires moving toward the positive side | Both robots read √2 from each beacon and the local information matrix is the identity at both points, so a local observability test passes; the two worlds need opposite actions: an alias. A third beacon at (0, 2) reads 1 and 3 and separates them when the range error is below 1 | exact (verdict) | G1 · host | Local observability taken for global | BD-244 |
| KA-275 | Stage 8 bound for two equally likely aliased worlds with noisy sensors | No policy succeeds with probability above (1 + TV)/2, TV the total-variation distance between the two transcript distributions | exact (bound) | G1 · host | A returned witness that a hand-built policy defeats in both worlds (KILL-SKEL-ALIAS) | BD-244 |
| KA-276 | Forward lidar on `quad-mammal` with `assert_obs: C2xC2` | Refused (`FROB-REF-109`) with the lidar mount as breaker | exact (verdict) | G1 · host | A trusted symmetry declaration | N14a-11 |
| KA-277 | Rejection fixture, rigid link: principal moments that violate the triangle inequalities | Refused at stage 2 (`FROB-REF-104`) | exact (verdict) | G1 · host | The fixture is the planted failure; a check that lets it through is broken | N14a-13, BD-100; fixture in N14b-18's corpus |
| KA-278 | Rejection fixture, joint: a prismatic travel in millimetres read as radians | Refused at stage 1 (`FROB-REF-102`) | exact (verdict) | G1 · host | As above | N14a-08, N14a-04; fixture in N14b-18's corpus |
| KA-279 | Rejection fixture, transmission: reverse drive that produces free electrical energy | Refused by the card check (`FROB-REF-130`) | exact (verdict) | G1 · host | As above | BD-074, BD-069; fixture in N14b-18's corpus |
| KA-280 | Rejection fixture, actuator: stall torque available indefinitely at high speed | Refused at stage 6 (`FROB-REF-131`) | exact (verdict) | G1 · host | As above | BD-103; fixture in N14b-18's corpus |
| KA-281 | Rejection fixture, drive wheel: a changed wheel that intersects the body at its nominal mount pose | Refused at stage 3 (`FROB-REF-123`) | exact (verdict) | G1 · host | As above | BD-104; fixture in N14b-18's corpus |
| KA-282 | Rejection fixture, track: turning in place at zero scrub work on a frictional floor | Caught by the belt-law known answer: a pivot turn costs scrub work μ·W·L·Δψ/4 (derived in BD-245), and under exact lateral no-slip the yaw rate is zero | exact (verdict) | G1 · strict | As above | BD-245, N14a-33 |
| KA-283 | Rejection fixture, foot: a point foot that silently acquires torsional support (torsion radius above the foot's own contact radius) | Refused at stage 1 (`FROB-REF-121`) | exact (verdict) | G1 · host | As above | N14a-05; fixture in N14b-18's corpus |
| KA-284 | Rejection fixture, sensor: measurements with no timestamp or an unknown frame | Refused at stage 1 (`FROB-REF-126`) | exact (verdict) | G1 · host | As above | BD-055, N14a-08; fixture in N14b-18's corpus |
| KA-285 | Rejection fixture, power source: an empty battery that keeps supplying commanded power | Run-time refusal (`FROB-REF-135`) | exact (verdict) | G1 · all tiers | As above | N15b-03 (the exhaustion event and its refusal), N15b-30 (end to end); BD-075 (the bus); fixture named in N14b-18's corpus |
| KA-286 | Rejection fixture, passive part: swapping a payload that leaves the assembly inertia unchanged | Caught by INV-SKEL-06 (the invalidation set of an edit contains every product that differs after a full recompile) | exact (verdict) | G1 · host | As above | BD-063, N14a-17 |
| KA-287 | Incremental compilation of a design after an edit (INV-SKEL-02, GATE-SKEL-02) | Equal to full compilation bit for bit, on 10⁴ random designs and edits | exact (bits) | G3 · host | none stated; BD-063 registers three planted defects | BD-063; N14bp2-01 from the command line |
| KA-288 | JSON and S-expression views of a design (INV-SKEL-04, PO-SKEL-01, GATE-SKEL-05) | The same `DesignId`; conversion is the identity on canonical forms; canonicalization is idempotent, on 10⁴ random designs and the eighteen archetypes | exact (hash) | G3 · host | none stated; KILL-SKEL-SEXPR removes the second view if the invariant cannot be held; BD-057 registers planted defects | BD-057, N14a-28 |
| KA-289 | Influence record of a compiled product (PO-SKEL-03, INV-SKEL-06) | Mutation of every field outside a product's record leaves the product bit-identical; the invalidation set of an edit contains every product that differs after a full recompile, on 10⁴ random edits | exact (bits) | G3 · host | A reused product that differs from its recompiled value (KILL-SKEL-SUBST) | BD-063, BD-077; N14bp2-01 from the command line |
| KA-290 | Import of each reference robot (GATE-SKEL-01) | Forward-kinematics parity against the oracle within 10⁻⁹ m and 10⁻⁹ rad over 10⁴ configurations; the loss report lists every dropped field found by a field-coverage diff | 10⁻⁹ m, 10⁻⁹ rad | G2 · host | none stated; BD-066 registers: half-sizes read as full sizes, `armature` silently dropped, a wrong Euler sequence | BD-066, N14a-31 |
| KA-291 | 100 seeded defects for stages 1 to 6, at least two per refusal code; 40 planted cases for stages 7 and 8 (GATE-SKEL-04) | All 100 refused, at least 95 by the stage and code that own them; zero refusals on the eighteen archetypes and on 10³ random valid designs; at least 36 of 40 decided correctly within budget and none decided wrongly | exact (counts) | G1 · host | The seeded defects are the controls | BD-110, BD-244; the corpus N14b-18; stage-1 defects from N14a-08 |
| KA-292 | Design patches (GATE-SKEL-08) | A stale parent refuses in 100 of 100 trials (`FROB-REF-112`); a repeated idempotency key returns the same `DesignId`; a schema migration leaves forward kinematics and mass properties bit-identical and old runs replayable | exact | G4 · host | none stated; BD-059 registers a non-atomic lookup-then-insert key binding, a head moved without compare-and-swap and operations stored one by one; N14a-18 registers a migration that rewrites a quantity through binary64; N29ax-01 registers re-execution that resolves the design by name (the "old runs replayable" clause) | BD-059, N14a-18, N29ax-01, N14a-29 |

Counts: 32 rows. By tier: G1 26, G2 1, G3 4, G4 1; KA-272 and KA-287 to KA-289 are the G3 rows. Rows whose
tolerance was printed "to set": KA-262, KA-264 (random limbs), KA-265, KA-266 (radius), KA-267, KA-269. Their
owners have since set three, which this registry carries marked "(set: <bead>)" with the owner's reason in the
derivation file: KA-265 (BD-109), KA-266 (N14b-03) and KA-269 (N14b-03, with N38b-47's derivation); KA-267's
sideways-cost clause is set by BD-109 and its chord clause is not. Three stay `obligation: tolerance`: KA-262
(illustrative; N14a-10), KA-264 on random limbs (N14a-22, N14a-23) and KA-267's chord (BD-109).

**KA-270, element `snake-crawler` (OPEN, owner: Skeleton lead, stated in N14bp1-01).** The printed cell is "C₂
under head-to-tail reversal". N14bp1-01's hand derivation finds the sagittal mirror in every left-right
symmetric chain: if the compiled chain is palindromic (body k and body 8 − k equal, joint i and joint 9 − i
equal) the group is C₂ × C₂ (the sagittal mirror, the transverse mirror and the half-turn); if an actuator
card's mass sits on one side (carried by each joint's child body) the reversal is lost and the group is C₂
generated by the sagittal mirror; in neither case is it the printed cell. Options there: amend the cell to the
computed group, or stagger the wheel pairs so that only the half-turn survives. This registry keeps the printed
cell as the row's truth and changes it only by a dated amendment that cites the decision. Fail-closed while
open: the row's test reports per archetype; the `snake-crawler` element is reported `blocked` with the question
named, never passed, and KA-270 is not counted as passing while that element is blocked; the other seventeen
elements are measured and reported. The zoo expectation (ZOO-snake-crawler) and the tables of BD-246, N13-22 and
N14b-04 move with the same decision.

Late known answers without a number. Two subsections added to the chapter after the catalogue was numbered
(module capability tables with the composed quasi-static screen, and folding of fixed joints with reduced
models) state five known answers that Appendix M does not list. Their numbers, as N13-KA does for its late
answers: this bead commits a request `[[block]] registry = "ka", count = 5, owner = "N14-KA", section = "§14.16, §14.17"` in
`crates/frob-conformance/registry/allocations.toml` (N38a-40; the allocator fills `first` and `last`), cites the rows by the tags below until the block is issued, then enters
them as rows with the issued numbers in table order in one commit, each keeping its tag in its row's `slug` field
(N38a-12; the one place N38a-40's rule 5 lets a tag survive numbering) so that a log that cited the tag still
resolves; a tag still cited at the M1 exit fails (`provisional-tag-past-due`).

| Tag | Case | Truth | Tolerance | Tier · site | Negative control | Owner |
|---|---|---|---|---|---|---|
| `skel-late-01` | Four identical one-joint legs, each a horizontal link of 0.25 m with a torque limit of 10 N·m (so at most 40 N per foot), under a 16 kg body (156.96 N at 9.81 m/s²) on a square stance with feet at (±0.2, ±0.2) m, centre of mass centred | 39.24 N per foot: admitted, with a witness force distribution | exact (verdict and rational forces) | G1 · host | A table with one outer row tightened by 5% must violate inner ⊆ outer | N14b-29, N14b-32 (pilot N14b-27) |
| `skel-late-02` | The same assembly with the centre of mass 0.05 m forward | The front pair must carry 0.625 × 156.96 = 98.1 N, above 2 × 40 N: excluded for every force distribution; the certificate is the vertical force balance with the pitch moment balance | exact (verdict) | G1 · host | Composition with the twist map in place of the wrench map loses the moment arm and admits the case | N14b-29, N14b-32 |
| `skel-late-03` | The same assembly in a trot: two diagonal feet in stance | Each stance foot must carry 78.48 N against 40 N: excluded | exact (verdict) | G1 · host | A screen that ignores the stance set (always four feet) admits the case | N14b-29, N14b-32 |
| `skel-late-04` | Fold of a welded child: parent 2 kg with rotational inertia 0.02·I kg·m² about its origin; child 1 kg with 0.01·I about its own centre at (0.3, 0, 0) m | Mass 3 kg; centre of mass (0.1, 0, 0) m; inertia diag(0.03, 0.12, 0.12) kg·m² about the parent origin and diag(0.03, 0.09, 0.09) about the combined centre | exact (rational) | G1 · host | Adding the two inertias without the offset term gives 0.03·I, wrong by 0.09 kg·m² on two axes; dropping the child gives 2 kg | N14b-33 |
| `skel-late-05` | Reduced model of the reference humanoid: the 30-link profile `g1-reduced-30` (31.96314202 kg) against the 38-link source (33.34014202 kg), on a static support rated at 320 N | At 9.81 m/s² the reduced model weighs 313.5584 N and passes; the source assembly weighs 327.0668 N and fails. A result computed on the reduced model carries the reduction in its evidence | to set (the masses are reported figures, re-read from the primary source before a gate cites them) | G1 · host | An importer option that drops links silently must be refused by the mass accounting; the reduced profile used as the import-parity reference must be rejected | N14b-34, N14b-24 |

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds exactly the 32 rows KA-261 to KA-292 with the
      values above, every field of N38a-12's schema filled, and validates under its loader (no second file);
      each row has a derivation file under `docs/derivations/` that lists it, its test and its owning bead.
- [ ] The block request for the five late answers is committed with this bead; they are cited by their tags
      `skel-late-01` to `skel-late-05` until the block is issued and by the issued numbers after; no tag is cited
      at the M1 exit.
- [ ] The meta-test passes: each row's derivation file names its test and owning bead; the test exists for every
      owning bead that has closed (the others are listed as pending on that bead, by name); every test tagged
      with one of these numbers has a row.
- [ ] A row whose tolerance is `to-set` is `obligation: tolerance` and a gate that cites it cannot record PASS;
      the three cells set by their owners (KA-265, KA-266, KA-269, and KA-267's sideways-cost clause) carry the
      owner's value and the derivation file names the owner and its reason.
- [ ] KA-270's `snake-crawler` element is reported `blocked` naming N14bp1-01's open question, and KA-270 is not
      counted as passing while it is; the other seventeen elements are reported per archetype.
- [ ] The planted registry defect (a derivation file naming a test that does not exist, for an owning bead
      marked closed) fails the meta-test.
- [ ] The seven rows whose catalogue entry states no negative control (KA-262, KA-263, KA-269, KA-287, KA-288,
      KA-290, KA-292) carry a control text: the one this bead adds (KA-262, KA-263, KA-269) or the owner bead's
      (the other four); after N38b-05's seeding tool runs every row's `control.nc` is non-empty, and no row of
      `controls.toml` was written by this bead.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_registry_skeleton.rs` (host, exact rationals; the real registry and scratch
copies, no mocks):
- `ka_registry_skeleton::row_count`: asserts 32 numbered rows and the identifier range KA-261..=KA-292 with no
  gap, every N38a-12 field present, and the five late tags distinct and either cited by tag or mapped to an
  issued block in table order (`ka_late_tags_mapped`, which fails in milestone-exit mode if a tag is cited).
- `ka_registry_skeleton::late_hand_values`: recomputes in exact rationals 156.96/4 = 39.24, 0.625 × 156.96 = 98.1,
  156.96/2 = 78.48, the fold (3 kg; diag(0.03, 0.12, 0.12) and diag(0.03, 0.09, 0.09) kg·m²) and the two weights
  31.96314202 × 9.81 = 313.5584 N and 33.34014202 × 9.81 = 327.0668 N to four decimals.
- `ka_registry_skeleton::owner_tests_exist`: resolves each test named by a derivation file to a compiled test
  name when its owning bead is closed, and lists the others as pending on their bead.
- `ka_registry_skeleton::controls_registered`: after seeding, each row's `control.nc` maps to a registered mutant
  or fixture.
- `ka_registry_skeleton::hand_values`: recomputes the closed-form rows in exact rationals inside the test:
  KA-261 (567/25 kg; 153819/80000 kg; the two gyration radii 0.018033 and 0.023566 m², ratio 1.307; 11.80
  times), KA-265 (√0.001296 = 0.036 exactly; √0.001656 = 0.040694; ratios 0.036/0.048 = 0.75 and
  0.040694/0.063 = 0.6459), KA-262 (1/(1 − 0.59) = 2.439; 20.76 × 2.439 = 50.6), KA-264 (I − e₁e₁ᵀ =
  diag(0, 1, 1, 1, 1, 1); 21 + 6 = 27 scalars, 108 bytes), KA-267 (the half turn v = 1 m/s, ω = π rad/s,
  Δt = 1 s gives (0, 2/π) m; the sideways cost at its minimizer α = 2·asin(√(d/(4b))), 0.178512 and 0.553669 m),
  KA-269 (p² = 29.43/0.407692 = 72.1868, p = 8.49628 s⁻¹; the wheel-inertia mutant 9.1698 s⁻¹;
  cosh(0.2·p) = 2.826), KA-274 (√2 from both beacons; the information matrix Σ uuᵀ = I at both robots; the third
  beacon reads 1 and 3).
- `ka_registry_skeleton::snake_element_blocked`: with the open question unresolved, the KA-270 report lists
  `snake-crawler` as `blocked` and the row as not passing.
- `ka_registry_skeleton::tolerance_open_blocks_gate`: a synthetic gate citing KA-269 reports `obligation:
  tolerance` and records no PASS.

### End-to-end script and logging

The rows are exercised by the end-to-end scripts of N14a-27 to N14a-33, N14bp2-01 and N41p2-02, each of which
logs, per assertion, the `ka` identifier, the truth, the measured value, the tolerance and the tier, in the
structured NDJSON form of the shared end-to-end helper. This bead's own script, `scripts/e2e/skel_ka_registry.sh`
(header: `bead=N14-KA milestone=M1 labels=skeleton,registry gates=GATE-GAUNT-02
plants=missing-test-closed-bead,duplicate-id,second-file`), sources the STD-E2E helper and runs
`cargo test -p frob-conformance --test ka_registry_skeleton`, N38a-12's loader test and N38a-40's allocation
test, then the rows and the late tags through their tier targets (`FROB_TEST_SEED=<seed> cargo test -p
frob-conformance --test <tier> -- <row>`, `<tier>` = `g1_known_answers`, `g2_parity`, `g3_metamorphic` or
`g4_chaos`), logging each row's per-site outcome as the runner reports it; a row whose owner has not landed is
`unbound`, never passed. It plants, on scratch copies under `target/e2e-scratch/skel_ka_registry/`, the three
registry defects below and requires each to be caught. Log at `target/e2e-logs/skel_ka_registry/<utc>.ndjson`
with a human transcript: every line with `ts`, `script`, `bead`, `step`, `event`, the exact command line, exit
code and wall time; one `assert` line per KA and late tag (id, eligibility, status per site and per archetype
for KA-270, measured, expected, tolerance, tier, sites, owner); the SHA-256 of `known_answers.toml` and
`allocations.toml`; on failure the first differing field and the scratch path that reproduces it; a `summary`
of rows passed, failed, blocked, unbound and not run.

### Known answers and negative controls

The registry is itself the list. Negative control of the registry: a planted derivation file naming a missing
test for a closed bead must fail `owner_tests_exist`; a planted duplicate identifier must fail `row_count`; a
planted second file `registries/known_answers/skeleton.toml` must fail N38a-12's loader with `ka-second-file`.

## Considerations

- OPEN (tolerances): six rows were printed "to set"; three are set by their owners and three remain (KA-262,
  KA-264 on random limbs, KA-267's chord). Owner: the bead in the row's last column, under the Gauntlet lead.
  Fail-closed default: such a row runs and reports its measured error and cannot be cited by a gate.
- KA-269's closed form, which the archetype chapter names without writing, is written by N38b-47
  (`docs/derivations/balancer-2w-pole.md`) at the template's defaults of N14b-03; until that file is on record
  the row is `obligation: derivation`.
- OPEN (KA-270, `snake-crawler`): above; owner the Skeleton lead; decided before N14b-04 and BD-246 measure the
  snake's group row.
- KA-282 points at a belt-law known answer that the Traction battery does not list under that name; BD-245
  writes the derivation (scrub moment μ·W·L/4 for uniformly loaded belts) and this row cites it.
- A row is never renumbered. A retired row is tombstoned.
- The five late rows have no catalogue number. Their numbers come from N38a-40's block (above); until it is
  issued they run under their tags and are reported in the printout under the tag, so that they cannot be
  mistaken for rows of another chapter.

## References

Appendix M.13 (KA-261 to KA-292); §14.5, §14.6, §14.10, §14.11, §14.13, §14.14, §14.15; §38.6; GATE-SKEL-01 to -08.
