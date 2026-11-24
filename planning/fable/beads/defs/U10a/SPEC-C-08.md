---
key: "SPEC-C-08"
title: "frob-analyze: C-08 specified — locomotion as holonomy, the connection and its curvature sign; law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "atelier", "marrow", "frob-analyze", "frontier", "spine", "spec", "test"]
parent: "WP-11"
deps: ["BD-025", "N10-CX", "N10-KA1", "N10a-01", "N28a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-011", "BD-243", "N10a-16", "N13-05"]
gates: ["GATE-MARROW-01", "GATE-MARROW-04", "GATE-ATEL-02"]
plan_refs: ["§10 C-08", "§11.1", "§11.6", "§13.8", "§28.4 A-09", "§12.3 T-20", "§38.7 MR-41", "Appendix C.1.4", "Appendix D.1", "Appendix M KA-064 KA-065", "Appendix N CX-030"]
---
## Background

Construction C-08 explains how a body moves by changing shape. A robot's configuration splits into a position
in a symmetry group G (where it is, how it is oriented) and a shape s (its joint angles). Mathematically
Q → Q/G is a principal bundle over shape space, and a connection on that bundle splits every motion into shape
change and group motion. For one closed loop in shape space (one gait cycle) the net displacement is the
holonomy of the connection, which to leading order is an integral of its curvature over the enclosed surface.

It replaces trial-and-error gait search for kinematic locomotors.

Payoff:

- A design oracle with no rollouts: curvature integrated over reachable shape space estimates, to leading
  order in loop size, the displacement per cycle, so two morphologies can be ranked before simulation
  (analysis A-09).
- With zero angular momentum the same formula gives the reorientation a robot achieves in flight by cycling
  limbs or a tail.
- Read as an inverse query, it asks for the least costly periodic shape change that gives a specified
  displacement with zero net yaw, and a wheel or foot arrangement that changes the constraint distribution is
  a body edit in the same search.

Tier: Frontier. The sign convention of the curvature, however, is a Solid law of the algebraic core: it is one
of the eight single-sign mutations every milestone must catch, which is why this specification is due at M1
although the analysis lands later.

Foundations (FACT, 2026-11-21): none in the sibling repositories. Derivatives of 𝔸(s) come from `fs-ad`
forward dual numbers (FrankenSim `483d09c682f2`).

Rejected alternative: writing the curvature with +[𝔸ᵢ, 𝔸ⱼ] or leaving the sign to each implementer. The sign
is fixed once, with a fixture on which the wrong sign is wrong by twice the answer.

## Scope

In: `docs/math/C-08.md`; the curvature-sign law in `crates/frob-lie/tests/laws_c08_sign.rs` (exact, SO(3)); the
holonomy fixtures in `crates/frob-analyze/tests/laws_c08.rs` (Elroy's beanie, a planar three-link floating
body); chart and sign controls; the register row; tolerances and controls for KA-064 and KA-065; the statement
of T-20.

Out: the analysis A-09 and its verb (BD-243); the pilot against direct collocation (N10a-16); the shape-space
atlas (Moonshot); minimum-perturbation coordinate optimization as a production feature (the contract requires
it of any consumer and tests chart independence).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-011 (the production
retractions of `frob-lie`): that bead is listed in `related` and is not a prerequisite. The law suite is generic
over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-lie` or `frob-analyze` do not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**Conventions.** Body velocity ξ = g⁻¹·ġ and reconstruction ġ = g·ξ. The connection is a Lie-algebra-valued
one-form on shape space, 𝔸 = 𝔸ᵢ·dsⁱ. The sign convention is ξ = −𝔸(s)·ṡ at zero momentum.

**The object.**

- Splitting of motion: ξ = −𝔸(s)·ṡ + 𝕀(s)⁻¹·p, with 𝕀(s) the locked inertia (the composite rigid-body inertia
  at the base with joints frozen) and p the group momentum. The second term is absent for principally
  kinematic systems and for zero-momentum free flight.
- Where 𝔸 comes from: from the block form of the kinetic energy, 𝔸 = 𝕀⁻¹·𝕀_BS, with 𝕀_BS the coupling block
  between base and shape velocities; or from Pfaffian constraints W_g·ξ + W_s·ṡ = 0 when rank W_g = dim G,
  giving 𝔸 = W_g⁻¹·W_s.
- Holonomy: for a closed loop in shape space the net displacement is the right-ordered path exponential of
  −∮𝔸·ds, which to leading order is minus the integral of the curvature D𝔸 over the enclosed surface.

**Curvature, in components, with its sign.**

```text
(D𝔸)ᵢⱼ = ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ]
g(loop) ≈ exp( −(D𝔸)ᵢⱼ · da )       small loop of area da in the (i, j) plane, traversed from i towards j
```

Derivation the contract carries. The first-order part is Stokes' theorem applied to −∮𝔸·ds. The bracket part
is the second-order term of the ordered exponential: for a rectangle with sides a and b the four legs
contribute −a·𝔸₁, −b·𝔸₂, +a·𝔸₁, +b·𝔸₂ in that order, and half the sum of the brackets of earlier with later
legs is +a·b·[𝔸₁, 𝔸₂]. Large loops keep the ordered product and do not use the area formula.

**Sign control (the fixture of theorem card T-20).** Constant connection 𝔸₁ = e_x, 𝔸₂ = e_y on SO(3), where
the bracket is the cross product. The curl vanishes and (D𝔸)₁₂ = −[e_x, e_y] = −e_z, so a small loop of area
da rotates the body by +da about z. The same follows directly from
exp(−a·e_x)·exp(−b·e_y)·exp(a·e_x)·exp(b·e_y) ≈ exp(a·b·e_z). A formula with +[𝔸ᵢ, 𝔸ⱼ] predicts −da and is
wrong by twice the answer.

**Honest scope.**

1. Exact for principally kinematic systems (kinematic wheeled bases, snake-like crawlers, zero-momentum free
   flight). For dynamic walking it is a diagnostic, not a predictor.
2. Curvature computed in poor coordinates misleads; the minimum-perturbation coordinates of Hatton and Choset
   are required before a curvature plot is read as a displacement map.
3. A connection belongs to one contact stratum. A loop in shape space that crosses slip or lift-off cannot
   borrow its old connection, and each stance stratum of a legged robot has its own connection with switching
   maps between them.
4. The area formula is leading order in loop size and applies to contractible loops. A loop that winds around
   a non-contractible direction of shape space (a full relative turn of a wheel or a tail) has a holonomy
   given by the path exponential and no enclosed surface.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-08.S1 | For a polynomial connection, the second-order coefficient of the holonomy of a coordinate rectangle is −(∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ]) (T-20) | Certified | Truncated-series identity in exact rationals |
| C-08.S2 | SO(3) constant connection: a loop of area da rotates the body by +da about z | Certified | Rational Baker–Campbell–Hausdorff truncation; numerical confirmation |
| C-08.S3 | Elroy's beanie: a relative rotation Δψ rotates body 1 by −I₂/(I₁ + I₂)·Δψ | Certified (ratio); Tested (integration) | Conservation of zero angular momentum |
| C-08.S4 | Planar three-link floating body: the SE(2) holonomy per cycle from the area formula agrees with direct integration (rotation exactly, translation to leading order) | Tested | Loop doubled, reversed and sped up until the approximation breaks |
| C-08.S5 | Loop relations to leading order: doubling the loop doubles the displacement, reversing negates it, speeding it up leaves it unchanged | Tested | Metamorphic test |
| C-08.S6 | The same physical gait reconstructed in another body frame predicts the same motion | Tested | Chart control |
| C-08.S7 | A loop crossing a contact-stratum boundary is refused by the area formula | Tested | Planted loop with a changed contact |

**Fixtures** (DESIGN constants where the construction gives none).

- *Elroy's beanie.* Two coaxial bodies with inertias I₁ = 3 and I₂ = 1 kg·m² about the common axis, zero total
  angular momentum. Shape ψ is the relative angle. I₁·θ̇₁ + I₂·(θ̇₁ + ψ̇) = 0 gives θ̇₁ = −I₂/(I₁ + I₂)·ψ̇, so
  𝔸 = I₂/(I₁ + I₂) = ¼ and a relative rotation Δψ = 2π rotates body 1 by −π/2. The shape space is a circle,
  the group is abelian and the curvature is zero: the displacement after a full relative turn is a holonomy
  around a non-contractible loop, which the area formula cannot see and the path exponential gives exactly.
- *Three-link floating body.* Three identical uniform slender links, mass 1 kg, length 1 m and inertia
  1/12 kg·m² about its centre each, joined end to end by two revolute joints, floating in the plane with zero
  linear and angular momentum. Shape s = (α₁, α₂) in the chain convention αᵢ = θᵢ₊₁ − θᵢ (θ₁, θ₂, θ₃ the link
  angles from one end); group SE(2), the pose of the middle link, body frame at its centre and along it. The
  connection is 𝔸(s) = 𝕀(s)⁻¹·𝕀_BS(s) ∈ 𝔰𝔢(2), computed in the derivation file from the kinetic energy. Loops:
  circles of radius ρ ∈ {0.05, 0.1, 0.2, 0.4, 0.8} rad centred at (0.5, 0.5) rad (the body bent into an arc),
  traversed from α₁ towards α₂. (DESIGN, and a correction of the construction's reading of KA-065, which speaks
  of the net rotation alone: the rotational part of a planar connection is abelian, so its holonomy is exactly
  −∬ curl 𝔸_ω by Stokes' theorem and no bracket enters it; the bracket of 𝔰𝔢(2) is a pure translation, so the
  sign mutant is visible only in the translation of the middle link, and the fixture must compare the whole
  SE(2) element. The centre (0.5, −0.5) in this convention is the point-symmetric S-shape, where the curvature
  vanishes and a circle about it moves the body by nothing, so it tests nothing.)
- *Sign fixture.* As above, with a = b = 1/10: the loop rotates by +1/100 rad about z to leading order.

**Tolerances set by this bead** (TARGET). Area formula against direct integration of ġ = g·ξ, ξ = −𝔸(s)·ṡ,
on the three-link body. Rotation of the middle link: exact by Stokes, so compared to 10⁻⁹ relative (quadrature
and integration error only). Translation of the middle link (the leading-order part): relative error at most
0.1 at ρ = 0.05 rad and at most 0.2 at ρ = 0.1 rad, and the ratio of the errors at ρ and ρ/2 within
[1.5, 3.0] for ρ ∈ {0.1, 0.2} (one order beyond the leading term); the test records the radius at which the
relative error first exceeds 0.25 ("until the approximation breaks"). Chart control: equal to 10⁻⁹ rad. Beanie:
exact ratio; integration to 10⁻¹² relative. Reference magnitudes from an independent `f64` computation made in
review (for orientation; the derivation file reproduces them and does not copy them): at the centre the
rotational curvature is about 0.0246 and the translational part of (D𝔸)₁₂ about 0.0039 against a bracket term
of about 0.073; at ρ = 0.1 rad a cycle turns the middle link by 7.7·10⁻⁴ rad and moves it by 1.1 to
1.3·10⁻⁴ m; the translational relative error is about 0.05 at ρ = 0.05, 0.09 to 0.12 at 0.1 and 0.17 to 0.28
at 0.2 (the two figures are the arc bent either way); at ρ = 0.1 rad the sign mutant's translation is wrong by
a factor of 30 to 40.

**What it does not claim.** No prediction for dynamic walking. No atlas: a shape-space atlas is funded only
after an independent full rollout admits at least one newly proposed gait and a changed-contact test withdraws
a formerly valid one. Ceiling stated before any measurement: the method replaces gait search for kinematic
locomotors only. Fallback: direct collocation over the same loops.

**Implementation map.** Crate `frob-analyze` (connection, curvature, holonomy); the sign law is also a law of
`frob-lie`. Verb: the `analyze.<name>` verb of analysis A-09 (connection curvature). Gates: the "Curvature
sign" law of GATE-MARROW-01; the curvature-bracket mutation among the eight of GATE-MARROW-04 (caught by at
least three fixtures; M4); the A-09 row of GATE-ATEL-02 (analyses that exist at M1, the rest by M4). Card:
T-20. Relation: the holonomy relation of the metamorphic registry (a shape loop doubled doubles the
displacement, reversed negates it, sped up leaves it unchanged, to leading order; mutation: the curvature
computed with +[𝔸ᵢ, 𝔸ⱼ]).

## Acceptance criteria

- [ ] `docs/math/C-08.md` exists with the conventions, the splitting, the curvature formula with its
      derivation, the sign fixture, the four scope limits, the seven statements and the fixtures.
- [ ] `cargo test -p frob-lie --test laws_c08_sign` passes: the rational truncated series of the SO(3)
      commutator equals +a·b·e_z at second order, exactly.
- [ ] `cargo test -p frob-analyze --test laws_c08` passes on the reference connection of the two fixtures.
- [ ] The beanie rotates by −π/2 for Δψ = 2π within 10⁻¹² relative, and the test asserts zero curvature.
- [ ] The three-link holonomy agrees with direct integration within the tolerances above, rotation and
      translation reported separately; the break radius is recorded; the test asserts that the curvature at
      the loop centre is not zero.
- [ ] The flipped-sign mutant fails three independent fixtures: the SO(3) sign fixture, the rectangle series
      of S1 on 𝔰𝔬(3) and 𝔰𝔢(2) connections, and the translation of the three-link loop (an error of order one
      or more relative to the answer). Its rotation on the three-link loop is unchanged, and the test asserts
      that too, so a reader cannot take the rotational check for a sign control. The holonomy relation, whose
      predicted side is the area formula, also fails under the mutant; it is logged and not counted as
      independent of the loop fixture.
- [ ] KA-064 and KA-065 registered with tolerances and controls set; the register row of C-08 filled.

## Test plan

### Unit tests

- `s1_rectangle_series`: random polynomial connections of degree ≤ 2 with values in 𝔰𝔬(3) and 𝔰𝔢(2), rational
  coefficients; the second-order coefficient of the ordered product around a coordinate rectangle equals
  −(D𝔸)₁₂ exactly.
- `s2_so3_sign`: 𝔸₁ = e_x, 𝔸₂ = e_y, a = b = 1/10; rotation about z is +a·b up to third order; with the
  mutant the prediction is −a·b.
- `s3_beanie`: I₁ = 3, I₂ = 1; Δψ ∈ {π/2, π, 2π, −2π}; body 1 turns by −Δψ/4.
- `s4_three_link`: holonomy by curvature integral, exp(−∬ D𝔸) with numerical quadrature of the exact
  curvature over the disc, against direct integration of ġ = g·ξ, ξ = −𝔸(s)·ṡ in SE(2) around each loop;
  rotation and translation against their separate tolerances; the curvature at the centre is nonzero.
- `s4_three_link_sign_mutant`: the same loops with +[𝔸ᵢ, 𝔸ⱼ]: translation fails, rotation unchanged.
- `s5_loop_relations`: doubled (traversed twice), reversed, time-rescaled by 10.
- `s6_chart_independence`: the same loop with the body frame attached to an end link instead of the middle
  link; predicted world-frame rotation of a fixed link equal within 10⁻⁹ rad.
- `s7_stratum_crossing`: a loop annotated with a contact change is refused with a message naming the stratum.

### End-to-end script and logging

`scripts/e2e/spec_c08.sh`. Sources the STD-E2E helper. Steps: both law suites; both per control; G1 rows
KA-064 and KA-065 (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- KA-064`,
and `-- KA-065`); then, logged `pending` until BD-243 lands (M3), `frob --json analyze curvature <the three-link
fixture design> --stratum flight --shape <joint 1>,<joint 2> --loop circle:0.1@0.5,0.5` (the registered inputs
of `analyze.curvature`: the loop is an inline circle or rectangle with its centre, not a file; `flight` is the
zero-momentum stratum of a floating body), asserting the displacement per cycle, the label `leading-order`, the chart named, and an evidence
block of class `Estimated` for the area formula and `Tested` against integration. Log
`target/e2e-logs/spec_c08/<utc>.ndjson` with transcript: commands, exit codes, holonomy by both routes with
units, relative errors against thresholds, the break radius, and on failure the loop and radius at which the
bound failed. Final `summary` line.

### Known answers and negative controls

- KA-064 Elroy's beanie: tolerance set to exact ratio and 10⁻¹² relative on integration; G1 · host. Control
  (set here): the connection's sign flipped (ξ = +𝔸·ṡ), which turns body 1 the wrong way; and I₁ and I₂
  exchanged.
- KA-065 three-link holonomy: tolerances as set above; G1 · host. Control: the component formula with the
  bracket term's sign flipped, which must fail the same loop by an error of order one in its translation.
- CX-030: the curvature written with +[𝔸ᵢ, 𝔸ⱼ].
- For an abelian group, or for the rotational part of any planar connection, the bracket term vanishes and the
  sign mutant is invisible; the suite therefore keeps the SO(3) fixture as the mandatory sign control and says
  so in the test name, and reads the three-link control from the translation only.
- Determinism: exact tests D0; integration tests twice on one machine, debug and release.

## Considerations

- Moonshot dependency: the atlas waits in the moonshot queue behind this construction's Frontier part and its
  own gate; nothing on the critical path depends on C-08.
- Risk: a curvature map read in a poor chart. Scope limit 2 is enforced by S6 on fixtures and by a label on
  every curvature output naming the chart.
- Risk: using the connection of one stance across a touchdown. Scope limit 3 and S7.
- The three-link connection's closed form is long; the derivation file is the single source and both routes
  of S4 use it, so the comparison tests the holonomy formula and the integrator, not the connection itself.
  An independent check of the connection is the momentum residual: the reconstructed motion must keep total
  linear and angular momentum zero to 10⁻¹².

## References

Plan §10 C-08; §11.1, §11.6; §13.8 (curvature-sign law and mutation rule); §28.4 (A-09); §12.3 (T-20); §38.7
(holonomy relation); Appendix C.1.4 (moonshot queue item 9); Appendix D.1; Appendix M KA-064, KA-065;
Appendix N CX-030. Pilot: N10a-16.
