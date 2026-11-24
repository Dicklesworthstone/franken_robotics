---
key: "SPEC-C-11"
title: "frob-analyze: C-11 specified — kinematics as algebraic geometry, certified inverse kinematics, singular loci"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "atelier", "marrow", "frob-analyze", "frob-lie", "solid", "frontier", "moonshot", "spine", "spec", "test"]
parent: "WP-11"
deps: ["BD-025", "N10a-01", "N28a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-010", "BD-105", "N10a-18", "N10ax-01", "SPEC-C-34"]
gates: ["GATE-ATEL-02"]
plan_refs: ["§10 C-11", "§12.1", "§28.4 A-05 A-06", "Appendix C.1.3", "Appendix C.1.4", "Appendix D.1", "Appendix M KA-074"]
---
## Background

Construction C-11 states that kinematics is algebra, and uses the fact. Forward kinematics is a product of
exponentials, which is a polynomial map in the cosines and sines of the joint angles subject to
cᵢ² + sᵢ² = 1. Inverse-kinematics solution sets, workspace boundaries and singular loci are therefore real
algebraic varieties, and SE(3) embeds as the Study quadric through dual quaternions.

It replaces numerical inverse kinematics with no statement of how many solutions exist.

Payoff:

- Closed-form inverse kinematics for three-joint legs, derived once by elimination and emitted as a kernel.
- Certified inverse kinematics: all solutions in a box with existence and uniqueness certificates by interval
  Newton.
- Singular loci det J = 0 for leg Jacobians, giving singularity-free workspace volume and
  distance-to-singularity along a nominal gait (analyses A-05 and A-06).
- Linkage synthesis is the same algebra read as a design problem.

Tier: Solid for product-of-exponentials and leg inverse kinematics; Frontier for certified inverse kinematics
(pilot N10a-18; fallback, numerical inverse kinematics with multistart, labelled incomplete); Moonshot for
Study-quadric synthesis, which waits in the moonshot queue behind the continuation lane of linkage synthesis
and is piloted with it.

Foundations (FACT, FrankenSymPy `29f847c1` and FrankenSim `483d09c682f2`, 2026-11-21; existence is not
admission): FrankenSymPy `fsym-polys` (Gröbner bases over ℚ with certificates, elimination, real rational
univariate representation), `CertifyRootBox` (at most 16 coordinates); `fs-ivl` Krawczyk operator; `fs-ga`
projective-geometric-algebra motors as dual quaternions.

Tool limits that bound the claim (FACT, same pins): Buchberger with a cap of 1,024 basis polynomials, no F4,
and univariate resultants only, so general 6R inverse kinematics is not promised; the real solver `real_rur`
is capped at 8 variables, 64 equations, degree 256 and 4,096-bit coefficients and handles zero-dimensional
ideals only; there is no homotopy solver. General 6R inverse kinematics by Gröbner bases is in the rejected
register at current tool limits; inside the caps, certified inverse kinematics is Frontier.

## Scope

In: `docs/math/C-11.md`; the law suite `crates/frob-analyze/tests/laws_c11.rs`; the product-of-exponentials
law in `crates/frob-lie/tests/laws_c11_poe.rs`; the three-joint positioning fixture with its four solutions
and its singular locus; the register row; the tolerance and control of KA-074.

Out: the workspace and singularity analyses and the derived leg kernel (BD-105); the certified-solver pilot
(N10a-18); the certified solver itself (N10ax-01, M4, funded by that pilot), whose acceptance binds the
production solver to this suite's subject trait and runs S3 and S6 unchanged, so the certified-solver criteria
below are met here on the exact reference with its planted solvers and logged `pending` for the production
solver until then; linkage synthesis lanes (BD-361, BD-437; construction C-34); admission of the symbolic
tools.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-010 (creates `frob-lie`):
that bead is listed in `related` and is not a prerequisite. The law suite is generic over its subject trait and
closes on the exact reference model and the mutants in its own test support. If `frob-lie` or `frob-analyze` do
not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**The object.**

- Product of exponentials: g(θ) = exp(θ₁·Ŝ₁)·…·exp(θₙ·Ŝₙ)·g₀, with Sᵢ the joint screws in the base frame at
  the reference configuration, Ŝᵢ their 4×4 matrix form, and g₀ the reference pose of the end frame. For
  revolute joints each factor is linear in (cᵢ, sᵢ) = (cos θᵢ, sin θᵢ), so g is a polynomial map in the
  (cᵢ, sᵢ) with cᵢ² + sᵢ² = 1. With the half-angle substitution tᵢ = sᵢ/(1 + cᵢ) it is rational in tᵢ.
- Inverse kinematics is the solution set of g(θ) = g_target (or of its position part), a real algebraic
  variety. For a non-redundant chain in general position it is zero-dimensional: finitely many solutions.
- The singular locus is det J(θ) = 0, with J the Jacobian of the task map; the workspace boundary is contained
  in the image of the singular locus.
- SE(3) embeds in projective space ℙ⁷ as the Study quadric through dual quaternions (the Moonshot part).

**The three-joint positioning fixture** (DESIGN constants; the standard waist–shoulder–elbow arrangement,
which is also a leg with hip yaw, hip pitch and knee). Joint 1 about e_z through the origin; joints 2 and 3
about parallel horizontal axes; upper link a₂ = 3/10 m, lower link a₃ = 1/4 m; no offsets. With
r = a₂·c₂ + a₃·c₂₃ and z = a₂·s₂ + a₃·s₂₃ (c₂₃, s₂₃ the cosine and sine of θ₂ + θ₃), the tool point is
p = (c₁·r, s₁·r, z).

- *Closed form.* θ₁ = atan2(p_y, p_x) or that plus π (the tool reached "forwards" or "over the back", with
  r = ±√(p_x² + p_y²)). For each choice, the planar two-link problem in (r, z) has two solutions (elbow up,
  elbow down) when |a₂ − a₃| < √(r² + z²) < a₂ + a₃: cos θ₃ = (r² + z² − a₂² − a₃²)/(2·a₂·a₃). In total up to
  four solutions, exactly four at a regular reachable target off the axis.
- *Exact targets.* A target is generated forward from rational points of the circle, for example
  (c₁, s₁) = (3/5, 4/5), (c₂, s₂) = (4/5, 3/5), (c₃, s₃) = (5/13, 12/13), so the generating triple is a known
  rational solution and the other three are algebraic of degree at most two over ℚ.
- *Singular locus.* det J = ± a₂·a₃·s₃·(a₂·c₂ + a₃·c₂₃): the elbow singularity s₃ = 0 (arm straight or folded)
  and the shoulder singularity r = 0 (tool on the waist axis). FACT (textbook result, to be re-derived in the
  derivation file by symbolic expansion; the overall sign depends on the axis convention, so the test asserts
  the factorization up to sign).
- *Counting at special targets.* On the axis (r = 0) θ₁ is free and the solution set is one-dimensional; on
  the outer boundary (√(r² + z²) = a₂ + a₃) the two elbow solutions merge. Both are reported as non-regular,
  not as a count.

**Certified inverse kinematics** (Frontier). For a box B of joint values: (1) subdivide; (2) on each sub-box
apply an interval Newton or Krawczyk operator K to the residual; (3) K(B′) ⊂ interior(B′) proves existence and
uniqueness of one solution in B′; K(B′) ∩ B′ = ∅ proves none; otherwise subdivide to a declared depth; (4)
return the list of certified boxes, the list of boxes proved empty, and any box left undecided. The answer is
two-sided: `Complete` only if no box is undecided.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-11.S1 | The product of exponentials is a polynomial map in (cᵢ, sᵢ), and equals the composition of joint transforms | Certified | Polynomial identity on a rational reference chain |
| C-11.S2 | The three-joint positioning chain has up to four solutions; at a regular reachable target exactly four | Certified | Closed form; exact count from the two quadratics |
| C-11.S3 | The certified solver finds exactly the solutions the closed form gives: none missed, none spurious | Certified per instance | Interval certificates per solution box, empty-box certificates elsewhere |
| C-11.S4 | det J factors as ± a₂·a₃·s₃·(a₂·c₂ + a₃·c₂₃) for the fixture | Certified | Polynomial identity modulo cᵢ² + sᵢ² = 1 |
| C-11.S5 | Distance to singularity along a path is the minimum over the path of the smallest singular value of J, and is zero exactly on the locus | Tested | Property test on the fixture |
| C-11.S6 | Problems outside the stated tool caps are refused or labelled incomplete, never answered silently | Tested | Planted over-size problem |
| C-11.S7 | Every solution returned by the closed-form kernel reproduces the target under forward kinematics | Tested | Round-trip property test |

**Failure cases and refusals.** A symbolic problem beyond a declared cap (more than 8 variables or 64
equations, degree above 256, coefficients above 4,096 bits, a basis above 1,024 polynomials, more than 16
coordinates for a root box) is refused with the cap named; the numerical fallback is multistart inverse
kinematics, whose result is labelled incomplete. A target on a singular set returns the non-regular
description. An undecided box makes the answer `Partial` with the box listed.

**What it does not claim.** General 6R inverse kinematics; completeness of a numerical multistart; any
statement about positive-dimensional solution sets beyond detecting them; anything about the Study-quadric
lane until its pilot runs. Ceiling stated before measurement: problems inside the caps above.

**Implementation map.** Crates `frob-lie` (product of exponentials) and `frob-analyze` (workspace,
singularities, certified solver); the leg kernel is derived at build time and emitted through the kernel
toolchain. Verbs: the `analyze.<name>` verbs of analyses A-05 (workspace) and A-06 (singularities); stages 3
and 7 of `design.check`. Gate: the A-05 and A-06 rows of GATE-ATEL-02 (each analysis passes its known answers;
M1 for the analyses that exist). No theorem card is specific to this construction; its certificates are
interval and polynomial certificates.

## Acceptance criteria

- [ ] `docs/math/C-11.md` exists with the object, the fixture and its closed form, the singular locus, the
      certified-solver procedure, the tool caps, the seven statements and the limits.
- [ ] `cargo test -p frob-analyze --test laws_c11` and `cargo test -p frob-lie --test laws_c11_poe` pass on
      the exact reference.
- [ ] For ten exact targets generated forward, the closed form returns four solutions, the generating triple
      among them, and each reproduces the target exactly (rational case) or within 10⁻¹² m.
- [ ] The certified solver, on the box [−π, π]³ for the same targets, returns four certified boxes and no
      undecided box; a solver with one planted missed box or one planted spurious box fails the comparison.
- [ ] det J equals the stated factorization up to sign as a polynomial identity.
- [ ] A target on the waist axis and a target on the outer boundary are reported as non-regular.
- [ ] A planted problem with nine variables is refused with the cap named.
- [ ] KA-074 registered with its tolerance (exact solution set; root values 10⁻¹² rad, set here) and control;
      the register row of C-11 filled.

## Test plan

### Unit tests

- `s1_poe_polynomial`: the fixture chain; g(θ) from the product of exponentials equals the explicit
  (c₁·r, s₁·r, z) at 10³ rational points of the three circles, exactly.
- `s2_four_solutions`: the ten targets; counts and membership.
- `s3_certified_matches_closed_form`: as in the criteria; also a joint-limit box that excludes the θ₁ + π
  branch, for which exactly two certified boxes are returned and the excluded region is certified empty.
- `s4_det_factorization`: symbolic expansion compared coefficient-wise modulo the circle relations.
- `s5_distance_to_singularity`: along a straight-line knee extension the smallest singular value reaches zero
  at s₃ = 0 and nowhere else on the path.
- `s6_caps`: nine variables; a degree-257 polynomial; each refused with its cap.
- `s7_round_trip`: 10⁴ random reachable targets in `f64`; forward of inverse within 10⁻¹² m.

### End-to-end script and logging

`scripts/e2e/spec_c11.sh`. Sources the STD-E2E helper. Steps: both law suites; both per control; the G1 row
KA-074; when the verbs exist, `frob --json analyze singularities fixtures/kin/leg3r.fbot` (the design is the
positional argument of every `analyze` verb, N28a-05; there is no `--design` flag) asserting the two
components of the locus, and `frob --json analyze workspace fixtures/kin/leg3r.fbot --target-step-height <h>`
(the row's task-level height input) at a reachable height, asserting the solution count reported for it (the
surviving branches); the evidence block must be `Certified` with certificate
hashes for the certified solver and `Tested` with the label `incomplete` for a multistart answer. Log
`target/e2e-logs/spec_c11/<utc>.ndjson` with transcript: commands, exit codes, solution counts against four,
box widths, residuals with units, and on failure the target and the first missing or spurious solution. Final
`summary` line.

### Known answers and negative controls

- KA-074 spatial 3R positioning chain: exact solution set, root values within 10⁻¹² rad (set here); G1 · host.
  Control: a missed or a spurious solution against the closed form (planted by removing one sub-box from the
  solver's queue, and by accepting an undecided box as a solution).
- Further controls: θ₁ + π branch omitted from the closed form (two solutions instead of four); the elbow
  cosine clipped into [−1, 1] without a reachability test (returns a "solution" for an unreachable target);
  a certified box whose Krawczyk image is not contained (the checker must reject the certificate).
- Metamorphic: rotating the target about the waist axis rotates θ₁ of every solution and changes nothing
  else; scaling lengths and target by a power of two leaves joint solutions unchanged.
- Determinism: exact tests D0; interval tests give identical boxes twice and across platforms (outward
  rounding with a declared implementation).

## Considerations

- The Moonshot lane is not on any critical path and is funded only after the continuation lane of linkage
  synthesis passes its own gate.
- Risk: a numerical answer presented as complete. The label `incomplete` on every multistart result and
  statement S6 are the guard.
- Risk: the derived kernel differs from the contract by an axis convention. Statement S7 runs on the emitted
  kernel as well as on the reference.
- Tool caps are facts about pinned siblings and may move; the contract cites them with their pins and the
  refusal names the cap that was hit.

## References

Plan §10 C-11; §12.1 (tool capabilities); §28.4 (A-05, A-06); Appendix C.1.3 (general 6R rejected at current
limits), C.1.4 (moonshot queue item 15); Appendix D.1 (product of exponentials); Appendix M KA-074. Pilot:
N10a-18.
