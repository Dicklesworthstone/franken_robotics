---
key: "SPEC-C-03"
title: "frob-algebra: C-03 specified — pseudo-inertia cone, shell rule, mass-support bound; contract and law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "marrow", "frob-algebra", "solid", "frontier", "spine", "spec", "test"]
parent: "WP-03"
deps: ["BD-025", "N10a-01", "STD-TEST", "STD-E2E"]
needs: ["Appendix G: the reference-robot fixtures (mass, centre of mass, inertia per body) audited by statement C-03.S8; BD-067 captures them with pin and date"]
related: ["BD-051", "BD-062", "BD-100", "N10a-13", "CARD-T-19", "N10-KA1", "BD-050"]
gates: ["GATE-MARROW-01", "GATE-SKEL-07", "GATE-ATEL-02"]
plan_refs: ["§10 C-03", "§11.4", "§11.5", "§13.1", "§13.10", "§12.3 T-19", "§14.5", "Appendix D.1", "Appendix M KA-043 to KA-046", "Appendix N CX-021 CX-022"]
---
## Background

Construction C-03 says where inertial parameters live. A rigid body has ten inertial parameters (mass, three
first moments, six second moments). Treated as ten free floats they admit values no physical body could have,
and optimizers and system-identification routines routinely walk into such values. The construction replaces
the ten floats with a point of a cone: the 4×4 pseudo-inertia matrix, which is physically consistent exactly
when it is positive definite (Wensing, Kim and Slotine).

Payoff. Import validation becomes an eigenvalue test with a certificate (theorem card T-19). Optimization and
identification steps follow geodesics of the cone and cannot leave the feasible set. By default inertias are
not parameters at all: they are computed from geometry and material density, so an agent designs shapes and
materials. A body with a declared wall thickness is a shell computed by a difference of solids, and a solid
fill of a large enclosure raises a density warning instead of passing silently.

Two refinements exist because positivity alone is too weak. (1) Positivity is necessary and not sufficient for
a declared envelope: a body whose mass lies inside a ball must also satisfy a mass-support bound. (2) A solid
default for an enclosure is wrong by an order of magnitude, so a body has no default fill.

Tier: Solid for validation and geometry-derived inertia; Frontier for Riemannian identification on the cone,
which has a pilot (N10a-13) and a fallback (a Euclidean fit projected onto the cone).

Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission):
`fs-mbd::SpatialInertia`; `fs-mbd-material` (material-derived inertia, about 60 thousand lines); `fs-opt`
manifold variables with retractions.

Rejected alternatives: ten free floats with a post-hoc "triangle inequality" check (misses the mass-support
case below and gives optimizers no feasible geometry); a solid default fill (the 18.8-fold error below).

## Scope

In: `docs/math/C-03.md`; the law suite `crates/frob-algebra/tests/laws_c03.rs` with exact rational reference;
the mass-support and shell fixtures; the register row; tolerances and controls for KA-043 to KA-046; the
statement of T-19.

Out: the production inertia cone and mass-support predicate (BD-051); the geometry and inertia functor with
fills and shells (BD-062); the mass roll-up analysis A-02 (BD-100); the identification pilot (N10a-13) and the
calibration lane that would consume Riemannian steps.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-050 (the `Cone` trait of
`frob-algebra`): that bead is listed in `related` and is not a prerequisite. The law suite is generic over its
subject trait and closes on the exact reference model and the mutants in its own test support. If `frob-algebra`
does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**Definitions.** For a body with mass measure dm in a frame with origin O: mass m = ∫ dm; first moment
h = ∫ x dm = m·c with c the centre of mass; second moment Σ = ∫ x·xᵀ dm; rotational inertia about the origin
Ī = tr(Σ)·I − Σ, and conversely Σ = ½·tr(Ī)·I − Ī. I_C is the rotational inertia about the centre of mass.

**The pseudo-inertia.** 𝖯 = [[Σ, h], [hᵀ, m]], a symmetric 4×4 matrix. (The construction's own text writes it
P; the sans-serif letter is used wherever the reflection matrix P also appears.)

- *Consistency.* The ten parameters are physically consistent exactly when 𝖯 ≻ 0 (positive definite). This is
  equivalent to m > 0 and Σ_C ≻ 0 for the second moment about the centre of mass, which in turn is the triangle
  inequalities on the principal moments (each principal moment is less than the sum of the other two).
- *Frame change.* Under a rigid transform or an isometry with homogeneous matrix G = [[R or P, t], [0, 1]] the
  pseudo-inertia maps by 𝖯 ↦ G·𝖯·Gᵀ. No parity sign is needed for a reflection, because the second moment is a
  polar quantity.
- *Geometry of the cone.* The cone of positive-definite 4×4 matrices with its affine-invariant metric
  d(𝖯, 𝖰) = ‖log(𝖯^(−½)·𝖰·𝖯^(−½))‖_F is a Riemannian symmetric space; the geodesic from 𝖯 to 𝖰 is
  γ(s) = 𝖯^(½)·(𝖯^(−½)·𝖰·𝖯^(−½))^s·𝖯^(½), s ∈ [0, 1], and stays positive definite. The log-Euclidean
  parametrization S = log 𝖯 is an equivalent chart for optimizers; either may be used.
- *Mass-support bound.* A body whose mass lies inside a ball of centre o and radius ρ satisfies
  ‖c − o‖² + tr(I_C)/(2m) ≤ ρ². The left side is the mass-weighted mean squared distance from o, because
  ∫‖x − c‖² dm = tr(Σ_C) = ½·tr(I_C). The bound is applied to a declared mass envelope and never to an
  unrelated collision proxy.
- *Shell rule.* A body with outer region Ω and declared wall thickness t has the mass and inertia of Ω minus
  its inward offset Ω_t at the same density: 𝖯_shell = 𝖯(Ω, ρ) − 𝖯(Ω_t, ρ), both in the same frame, computed in
  exact rational arithmetic at model compile time so the subtraction of two nearly equal matrices loses
  nothing. A body without a wall thickness is solid. Limit check: as t → 0 a spherical shell of radius r tends
  to I = (2/3)·m·r², against (2/5)·m·r² for the solid ball.
- *No silent solid.* A solid body whose mass exceeds the plausibility bound of its part class is refused
  (FROB-REF-168: a solid body exceeds the mass plausibility bound of its part class and no fill is declared;
  repairs, in order: declare a wall thickness, declare the fill as intended, change the material) unless the
  design declares the fill as intended.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-03.S1 | The maps between (m, h, Ī) and 𝖯 are mutually inverse | Certified | Linear identity over ℚ |
| C-03.S2 | 𝖯 ≻ 0 exactly when the parameters are consistent; every body's 𝖯 is positive definite (T-19, first half) | Certified | Exact rational LDLᵀ with positive pivots; the certificate is the factorization |
| C-03.S3 | A declared envelope satisfies the mass-support bound (T-19, second half) | Certified | One rational inequality |
| C-03.S4 | 𝖯 ↦ G·𝖯·Gᵀ is the frame-change law and preserves positive definiteness, including for reflections | Certified | Congruence over ℚ |
| C-03.S5 | The affine-invariant geodesic between two consistent pseudo-inertias stays positive definite | Tested | Property test; ten points along the geodesic of a diagonal pair by hand |
| C-03.S6 | Shell moments equal the closed forms for box, cylinder, sphere and capsule shells | Certified | Exact rational comparison (for the box; with π kept symbolic for the others) |
| C-03.S7 | A solid fill of a large enclosure is flagged, never passed silently | Tested | Planted design |
| C-03.S8 | Every reference robot on file passes S2 and S3, or the audit names the body that fails | Tested | Audit over the reference-robot fixtures |

**Fixtures** (exact arithmetic on the stated numbers; a reader can check them by hand).

- *Mass-support.* A centred 1 kg part with three principal moments of 0.008 kg·m² each, declared inside a ball
  of radius 0.10 m. It passes positivity and the triangle tests (0.008 < 0.016). It needs
  tr(I_C)/(2m) = 0.024/2 = 0.012 m² where the ball allows ρ² = 0.010 m². Verdict: refused by the mass-support
  predicate (FROB-REF-106: declared mass properties violate the mass-support bound; repairs: enlarge the
  declared envelope, or correct the inertia).
- *Equality case.* An isotropic thin shell of radius 0.10 m and mass 1 kg has moments
  (2/3)·1·0.01 = 1/150 kg·m², so tr(I_C)/(2m) = (3/150)/2 = 0.010 m²: the bound holds with equality and the
  part is accepted.
- *Shell.* An aluminium box of 0.4 × 0.3 × 0.2 m at 2,700 kg/m³. Solid: 0.024 m³ × 2,700 = 64.8 kg. With a
  2.5 mm wall the inner box is 0.395 × 0.295 × 0.195 m = 0.022722375 m³, the wall volume is 0.001277625 m³ and
  the mass is 3.4495875 kg = 275,967/80,000 kg, printed 3.45 kg. Ratio 64.8/3.4495875 = 18.78…, printed 18.8.
- *Indefinite matrix.* A pseudo-inertia with one eigenvalue below zero (take the consistent diagonal
  𝖯 = diag(0.004, 0.004, 0.004, 1) and replace the first entry by −10⁻⁶): the LDLᵀ meets a negative pivot and
  the smallest eigenvalue is reported (FROB-REF-104: inertia not physically consistent).
- *Triangle violation.* Principal moments (0.010, 0.002, 0.002) kg·m² at unit mass: 0.010 > 0.004, so
  Σ_C has a negative eigenvalue (½·(0.002 + 0.002 − 0.010) = −0.003); detected.

**Failure cases.** The mass-support bound needs a declared envelope; without one the predicate is not evaluated
and the finding says so. A shell whose wall thickness exceeds half its smallest dimension is a solid and is
refused as written (FROB-REF-105: a body without a fill; a solid written as a shell). Tolerance-route imports
are classed `Tested`.

**What it does not claim.** Positivity and mass support together are still necessary conditions: they do not
say the stated inertia is the inertia of the declared geometry. That is the job of the geometry-derived
default. The Riemannian lane claims feasibility of iterates and better identification at equal data
(HYPOTHESIS, settled by the pilot); it claims no speed.

**Implementation map.** Crates `frob-algebra` (the cone, the predicate) and `frob-model` (geometry-derived
inertia; shells). Verbs: `design.check` (stage findings) and the `analyze.<name>` verb of analysis A-02, mass
roll-up. Gates: GATE-MARROW-01 (cone laws among "all laws"), GATE-SKEL-07 (shell moments equal closed forms
exactly; archetype masses within 15% of the reference robot's mass on file; TARGET; M1 and M3), the A-02 row
of GATE-ATEL-02. Card: T-19. Invariant served: every inertia in a compiled model passes C-03, and every
parameter has a unit and a provenance (INV-SKEL-03).

## Acceptance criteria

- [ ] `docs/math/C-03.md` exists with the definitions, the eight statements, all five fixtures with their exact
      numbers, the limits and the map.
- [ ] `cargo test -p frob-algebra --test laws_c03` passes on the exact reference and on the production cone
      once BD-051 lands.
- [ ] Round trip (m, h, Ī) ↔ 𝖯 is the identity exactly on 10³ random rational bodies and within 10⁻¹² relative
      in `f64`.
- [ ] The mass-support fixture is refused with 0.012 m² needed and 0.010 m² available; the shell of radius
      0.10 m is accepted at equality.
- [ ] The box shell mass equals 275,967/80,000 kg exactly and the solid mass 64.8 kg; the ratio prints 18.8.
- [ ] The indefinite matrix and the triangle violation are rejected with the offending eigenvalue reported.
- [ ] All controls below are registered and caught; KA-043 to KA-046 have tolerances and controls set.
- [ ] The audit of reference robots produces a table of bodies with pass or fail and the failing quantity.
- [ ] The register row of C-03 is filled.

## Test plan

### Unit tests

- `s1_roundtrip_exact`, `s1_roundtrip_f64`: as in the criteria.
- `s2_ldlt_certificate`: consistent matrices yield four positive pivots; the certificate re-multiplies to 𝖯;
  a certificate with one pivot edited is rejected by the checker.
- `s2_indefinite_rejected`, `s2_triangle_violation`: the two fixtures.
- `s3_mass_support_refused`, `s3_mass_support_equality`: the two fixtures; plus an off-centre case, c at
  0.06 m from o with moments 0.001 each at 1 kg: 0.0036 + 0.0015 = 0.0051 ≤ 0.010, accepted.
- `s4_congruence`: random rigid motions and the mirror diag(1, −1, 1): G·𝖯·Gᵀ equals the pseudo-inertia of the
  moved body computed from moved mass points (eight point masses, exact).
- `s5_geodesic_pd`: 𝖯 = diag(1, 2, 3, 4)·10⁻³ and 𝖰 = diag(4, 3, 2, 1)·10⁻³; ten points on the geodesic are
  diagonal with entries aᵢ^(1−s)·bᵢ^s, all positive.
- `s6_shell_closed_forms`: box shell against the difference of two solid boxes; the thin-sphere limit tends to
  (2/3)·m·r² as t → 0 (ratio within 10⁻⁶ at t = 10⁻⁶·r).
- `s7_solid_enclosure_flagged`: the aluminium box declared solid in an enclosure part class is refused with
  FROB-REF-168; the same with a declared fill passes.

### End-to-end script and logging

`scripts/e2e/spec_c03.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G0 and G1 rows
KA-043 to KA-046; when the verbs exist, `frob --json design check fixtures/inertia/part_in_ball.fbot` (expects
refusal FROB-REF-106 with both numbers), `…/shell_equality.fbot` (accepted), `…/alu_box_solid.fbot` (refused,
FROB-REF-168) and `…/alu_box_shell.fbot` (mass 3.45 kg). The log
`target/e2e-logs/spec_c03/<utc>.ndjson` records per step the command, exit code, the refusal code and its
ranked repairs, masses with units against expected values, the envelope's evidence block, and on failure the
first differing rational. Final `summary` line.

### Known answers and negative controls

- KA-043 round trip and consistency: tolerance set to exact over ℚ, 10⁻¹² relative in `f64`; G0 · host.
  Control: a seeded triangle-inequality violation, which must be detected.
- KA-044 mass-support fixture: exact; G1 · host. Control: a check that stops at positivity and the triangle
  inequalities.
- KA-045 equality shell: exact. Control (set here): the factor ½ dropped from tr(I_C)/(2m), which refuses the
  equality shell.
- KA-046 shell box: tolerance set to exact rationals (printed to three significant figures). Control: a solid
  fill of the enclosure that passes without a density warning.
- CX-021 (accept an inertia because it passes positivity and triangle tests); CX-022 (fill every enclosure
  solid).
- Further controls: the shell computed in `f32` as a difference of two nearly equal masses (loses digits; the
  exact comparison fails); the frame change applied as GᵀPG instead of G·𝖯·Gᵀ.
- Metamorphic: consistency is invariant under rigid motion and reflection; scaling lengths by λ and masses by
  κ scales 𝖯 block-wise by (κλ², κλ, κ) and preserves verdicts.
- Determinism: exact, D0; identical twice, across platforms.

## Considerations

- The plausibility bound per part class is data of the design language, not of this construction; this spec
  only requires that a solid enclosure cannot pass silently.
- The Frontier lane's kill rule is in its pilot: no fewer infeasible iterates and no better recovery at equal
  data means the projected Euclidean fit ships.
- Risk: exact shell arithmetic for curved solids needs π kept symbolic; the reference keeps rational
  coefficients of π and compares coefficient-wise.
- Interaction: symmetry detection colours bodies by the characteristic polynomial of 𝖯 in exact rationals, so
  the pseudo-inertia must be available exactly at model compile time.

## References

Plan §10 C-03; §11.4, §11.5; §13.1, §13.10; §12.3 (T-19); §14.5; Appendix D.1; Appendix M KA-043 to KA-046;
Appendix N CX-021, CX-022. Analysis A-02. Pilot: N10a-13.
