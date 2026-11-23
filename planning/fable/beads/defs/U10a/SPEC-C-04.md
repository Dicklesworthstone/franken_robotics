---
key: "SPEC-C-04"
title: "frob-dyn: C-04 specified — Euler–Poincaré dynamics on a tree, momentum map, Jacobians; contract, law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "kinesis", "frob-dyn", "solid", "spine", "spec", "test"]
parent: "WP-10"
deps: ["BD-007", "BD-025", "N10a-01", "SPEC-C-02", "STD-TEST", "STD-E2E"]
needs: ["§16: the bead that creates crate frob-dyn (the law suite lands in crates/frob-dyn/tests/ and runs first against the referee through frob-fsim)"]
related: ["BD-088", "BD-089", "BD-150", "BD-385", "CARD-T-16", "CARD-T-33", "SPEC-C-05", "N10-KA1", "BD-047"]
gates: ["GATE-KIN-01", "GATE-KIN-02", "GATE-KIN-03", "GATE-MARROW-04"]
plan_refs: ["§10 C-04", "§11.2", "§13.3", "§13.10", "§16", "§12.3 T-16 T-33", "Appendix D.1", "Appendix I.1", "Appendix M KA-047 to KA-051", "Appendix N CX-023 CX-024"]
---
## Background

Construction C-04 states what "the dynamics" is, so that the simulator is not an opaque step function. The
equations of an articulated robot are the Euler–Poincaré equations on its kinematic tree: the base obeys a
momentum equation on SE(3), and Featherstone's recursions (RNEA, CRBA, ABA) evaluate the same equations on the
tree using the adjoint, the algebra adjoint and its dual.

Payoff, each item being something the rest of the system relies on:

- Centroidal momentum is the momentum map, so centroidal quantities and their dynamics are available without
  extra code, and the centroidal controller is the reduced equation.
- In ballistic flight the centre of mass follows a parabola and angular momentum about it is constant, which is
  an exact test of any implementation.
- The change of angular momentum minus the external angular impulse about one fixed inertial origin is a
  diagnostic that separates unexpected yaw caused by a controller or by friction from an accounting defect.
- The recursion is parallel across tree depth, which is the shape a GPU wants.

Tier: Solid. Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission):
`fs-mbd::articulated::{forward_kinematics, inverse_dynamics, forward_dynamics,
free_floating_forward_dynamics, free_floating_momentum}` and `generalized::free_floating_terms` (mass matrix by
a composite-rigid-body pass).

The construction also carries the project's first hand-checkable counterexample: taking the lower three rows of
a space Jacobian as a point Jacobian. It looks right, compiles, and gives zero joint torques on a fixture where
the truth is (−3, −3) N·m.

## Scope

In: `docs/math/C-04.md`; the law suite `crates/frob-dyn/tests/laws_c04.rs`, generic over a `Dynamics` trait and
first run against the referee dynamics exposed by the airlock crate `frob-fsim`; the planar two-link fixture in
exact rationals; ballistic invariants; structure identities; the limb-port elimination; the register row;
tolerances and controls for KA-047 to KA-051.

Out: the referee step and batched kernels (BD-080, BD-150, BD-159); parity gates against oracles (BD-089,
GATE-KIN-02, -03); the tree-operator route for larger mechanisms (BD-385, Frontier); time stepping
(SPEC-C-05); contact (SPEC-C-06).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-047 (the production
frame-typed twists and wrenches): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-dyn` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then. The referee oracle BD-007 (the sibling tree dynamics behind the airlock
crate `frob-fsim`, M0) stays a dependency: the suite runs against it as an independent oracle, and it is not an
implementation of this construction in `frob-dyn`.

## Technical approach

**Conventions.** Twists ξ = (ω, v) angular first, body frame by default: ξ = g⁻¹·ġ. Wrenches (τ, f), torque
first. ad*_ξ = ad_ξᵀ, no minus sign: for a momentum μ = (π, ℓ), ad*_ξ μ = (π × ω + ℓ × v, ℓ × ω). Link
numbering is topological: the parent index of link i is less than i.

**The object.**

- With base body velocity ξ = g⁻¹·ġ and shape s (the joint coordinates), the body momentum is
  μ = 𝕀(s)·(ξ + 𝔸(s)·ṡ). 𝕀(s) is the locked inertia, the composite rigid-body inertia of the whole robot at the
  base with the joints frozen; 𝔸(s) is the mechanical connection.
- The base obeys μ̇ = ad*_ξ μ + w, with w the sum of applied wrenches expressed at the base. In components:
  π̇ = π × ω + ℓ × v + τ, ℓ̇ = ℓ × ω + f.
- In generalized coordinates the smooth equations read M(q)·v̇ + h(q, v) = B(q)·u + J(q)ᵀ·f + f_ext with
  q̇ = 𝒯(q)·v. M is the mass matrix, h collects Coriolis, centrifugal and gravity terms, B maps actuator
  efforts u, J is the contact Jacobian, f the contact forces, 𝒯 the kinematic map from velocities to
  coordinate rates.
- The recursions: RNEA (inverse dynamics: outward pass of velocities and accelerations with Ad and ad, inward
  pass of wrenches with the pullback), CRBA (mass matrix from composite inertias) and ABA (forward dynamics
  from articulated inertias), all on the tree.

**Checks on the sign of the base equation** (hand-derivable; carried in the contract). For a free body with the
frame origin at the centre of mass, π = I·ω and ℓ = m·v give Euler's equation I·ω̇ = (I·ω) × ω and v̇ = v × ω.
The world-frame velocity R·v of a force-free body is then constant, because d(R·v)/dt = R·(ω × v + v̇) = 0;
with the opposite sign it would be 2R·(ω × v). With I = diag(1, 2, 3) kg·m² and ω = (1, 1, 0) rad/s,
π = (1, 2, 0) and π̇ = (0, 0, −1); the reversed sign gives (0, 0, +1) and still conserves kinetic energy.

**Jacobians.** For a space screw column (ωᵢ, vᵢ), the velocity of the point p rigidly carried by that joint is
vᵢ + ωᵢ × p. The space Jacobian pairs with the wrench (p × F, F) about the origin; the point Jacobian pairs
with the force F. They are different objects with different duals.

**Honest limits.**

- The batched tiers materialize a dense mass matrix per world for mechanisms of 18 to 60 degrees of freedom
  (DESIGN); a tree-operator route that avoids it is Frontier and is meant for larger mechanisms.
- A limb summarized at its attachment port by its articulated inertia and bias wrench is positive semidefinite,
  not definite: eliminating a joint axis s from a 6×6 inertia I gives I − I·s·(sᵀ·I·s)⁻¹·sᵀ·I, which removes
  one direction. For I the identity and s the first axis the result is diag(0, 1, 1, 1, 1, 1). So multi-limb
  ground contact still goes through the coupled base solve; limbs cannot be solved independently.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-04.S1 | μ̇ = ad*_ξ μ + w with ad*_ξ = ad_ξᵀ; for a force-free body R·v and the spatial momentum Ad_g⁻ᵀ·μ are constant | Certified (sign identity); Tested (integration) | Polynomial identity ⟨ad_ξᵀ μ, η⟩ = ⟨μ, [ξ, η]⟩ (T-16); free-body fixture |
| C-04.S2 | Kinetic energy equals ½·vᵀ·M(q)·v and equals the sum over bodies of ½·ξᵢᵀ·Iᵢ·ξᵢ | Tested | Property test on random states; exact on the two-link fixture |
| C-04.S3 | Ṁ − 2C is skew-symmetric for the Christoffel factorization h = C(q, v)·v + g(q) | Tested; Certified per limb where the budget allows (T-33, a HYPOTHESIS on budget) | Polynomial identity modulo cᵢ² + sᵢ² = 1 |
| C-04.S4 | In ballistic flight the centre of mass follows a parabola and the angular momentum about it is constant | Tested | Referee rollouts |
| C-04.S5 | ΔL_O − ∫τ_ext,O dt = 0 about one fixed inertial origin O | Tested | Referee rollouts with and without contact |
| C-04.S6 | The point-velocity column at p of a space screw column (ωᵢ, vᵢ) is vᵢ + ωᵢ × p; space Jacobian pairs with (p × F, F), point Jacobian with F | Certified | Linear identity over ℚ |
| C-04.S7 | Eliminating one joint axis from a positive-definite 6×6 inertia leaves a positive-semidefinite matrix of rank 5 | Certified | Exact LDLᵀ with one zero pivot |
| C-04.S8 | The same part graph compiled to a symbolic expression, a CPU operator and a device kernel reproduces the two-link fixture | Tested | Three evaluations, one fixture |
| C-04.S9 | RNEA, CRBA and ABA agree: M·v̇ + h from CRBA and RNEA reproduces the efforts for which ABA returned v̇ | Tested | Property test |

**The two-link fixture** (exact; the first fixture a reader reproduces on paper). A planar 2R chain with link
lengths 0.4 m and 0.3 m, joint angles 0 and π/2, both axes along +z, joint 1 at the origin.

- Tool point: (0.4, 0.3) m.
- Translational Jacobian: column 1 is e_z × (0.4, 0.3) = (−0.3, 0.4); column 2 is e_z × (0, 0.3) = (−0.3, 0).
  J = [[−0.3, −0.3], [0.4, 0.0]] m/rad.
- A tool force (10, 0) N gives joint torques Jᵀ·F = (−3, −3) N·m.
- Joint rates (0.2, −0.1) rad/s give tool velocity J·q̇ = (−0.03, 0.08) m/s.
- Power by both routes: F·v = −0.3 W and τ·q̇ = −0.6 + 0.3 = −0.3 W.
- Space screws: joint 1 is (e_z, 0); joint 2, through (0.4, 0, 0), is (e_z, (0, −0.4, 0)). Their lower three
  rows are [[0, 0], [0, −0.4], [0, 0]], and "torque = lower rowsᵀ·F" gives (0, 0): the shortcut fails. The
  correct pairing uses the wrench about the origin, (p × F, F) with p × F = (0, 0, −3) N·m, and returns
  (−3, −3).

**Ballistic fixture** (DESIGN constants). The built-in reference biped or any articulated body released with
base twist and joint rates nonzero, no contact, joint torques from a fixed open-loop schedule. Truth: the
centre of mass follows the integrator's discrete parabola to round-off and the continuous parabola to the
integrator's order; angular momentum about the centre of mass is constant. For the default semi-implicit update
(v₊ = v + g·h, x₊ = x + v₊·h) the discrete centre-of-mass position after k steps is
x₀ + v₀·k·h + g·h²·k·(k + 1)/2, which differs from the continuous parabola by ½·g·h·t.

**Failure cases.** At a configuration where 𝒯(q) is singular in a chosen chart (Euler angles) the equations
are not evaluated: state is stored as group elements, so the case does not arise in state, only at import and
display. The momentum test is not a test of a nonholonomic system: a Chaplygin sleigh conserves energy while
the direction of its linear momentum turns, so the conservation tests of this construction are applied only
where the symmetry holds.

**What it does not claim.** No statement about integrator accuracy (that is the time-stepping construction),
none about contact, and no symplectic claim.

**Implementation map.** Crate `frob-dyn` (referee lowering through `frob-fsim`; batched kernels in
`frob-kernels`). Verb: `sim.run` (every tier) and `telemetry.energy` for the momentum and energy reports.
Gates: GATE-KIN-01 (TARGET, M1: pendulum period within 10⁻⁶ relative at h = 1 ms; symmetric-top precession
within 10⁻⁴; free-flight momentum conserved to 10⁻¹² relative over 10⁶ steps on the variational lane; the
ballistic invariants of this construction); GATE-KIN-02 (contact-free forward dynamics of each reference robot
against the pinned oracle, joint accelerations within 10⁻⁹ relative in `f64`, M1); GATE-KIN-03 (`batch-cpu` in
`f64` against `fs-mbd`: kinematics, inverse dynamics, forward dynamics and mass matrix within 10⁻¹⁰ relative on
10⁴ states per reference robot, M2). Cards: T-16, T-33.

## Acceptance criteria

- [ ] `docs/math/C-04.md` exists with the equations, the nine statements, the two-link fixture with every
      number above, the ballistic fixture, the limits and the map.
- [ ] `cargo test -p frob-dyn --test laws_c04` passes against the exact reference (two-link chain, free body)
      and against the referee dynamics through `frob-fsim`.
- [ ] The two-link fixture reproduces exactly in rational arithmetic: J, torques (−3, −3) N·m, velocity
      (−0.03, 0.08) m/s, −0.3 W by both routes.
- [ ] The lower-three-rows mutant returns torques (0, 0) and is caught by S6 and by the power check.
- [ ] Free body: π̇ = (0, 0, −1); the reversed-sign mutant is caught by the spatial-momentum test and is not
      caught by the energy test (the test asserts both facts).
- [ ] Ballistic: centre of mass equals the discrete parabola within 10⁻¹² relative and angular momentum about
      it is constant within 10⁻⁹ relative over 10⁴ referee steps (TARGET, set here).
- [ ] Limb elimination: identity inertia and first axis give diag(0, 1, 1, 1, 1, 1) exactly.
- [ ] KA-047 to KA-051 registered with tolerances and controls set; the register row of C-04 filled.

## Test plan

### Unit tests

- `s1_free_body_sign`: I = diag(1, 2, 3), ω = (1, 1, 0) → π̇ = (0, 0, −1); spatial momentum rate zero.
- `s1_force_free_world_velocity`: 10³ random rational (ω, v): d(R·v)/dt = 0 identically.
- `s2_kinetic_energy`: random states of the two-link chain with unit link masses (exact) and of each reference
  robot (`f64`, 10⁻¹⁰ relative).
- `s3_skew`: Ṁ − 2C + (Ṁ − 2C)ᵀ = 0 on the two-link chain exactly at rational points of the circle
  (c, s) = (3/5, 4/5), (5/13, 12/13); on reference robots within 10⁻¹⁰ relative.
- `s4_ballistic`, `s5_angular_impulse`: referee rollouts as in the criteria; S5 also with an applied external
  couple of known impulse.
- `s6_point_column`, `s6_two_link_fixture`: the exact numbers above; both pairings.
- `s7_limb_packet_psd`: the elimination formula on the identity and on 10³ random positive-definite rational
  inertias: one zero pivot, five positive.
- `s8_three_compilations`: symbolic expression, CPU operator, device kernel on the two-link fixture; exact for
  the symbolic route, within the tier's tolerance on devices.
- `s9_recursions_agree`: 10⁴ random states per reference robot.

### End-to-end script and logging

`scripts/e2e/spec_c04.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G0 and G1 rows
KA-047 to KA-051; `frob --json sim run --tier referee --robot builtin:g1 --scenario ballistic --seed 7
--steps 10000` followed by `frob --json telemetry energy <run>`; assertions on the momentum and
centre-of-mass residuals read from the envelope, with its evidence block (claim kind known answer, tier
`referee`, determinism class, model identity, seed). Log `target/e2e-logs/spec_c04/<utc>.ndjson` with
transcript: commands, exit codes, wall times, residuals with units and thresholds, run digests, and on failure
the first step at which a residual exceeded its bound and the run identifier to replay. Final `summary` line.

### Known answers and negative controls

- KA-047 ballistic flight: tolerance set to 10⁻¹² relative against the discrete parabola, 10⁻⁹ relative for
  angular momentum, zero angular-impulse residual to 10⁻⁹; G1 · referee. Control (set here): the reaction
  wrench of one actuator dropped on the parent link, which makes internal torques act as external ones.
- KA-048 structure identities: exact over ℚ on the two-link chain, 10⁻¹⁰ relative in `f64`; G0 · host. Control
  (set here): the Coriolis factor taken as Ṁ (the term −½·∂(vᵀ·M·v)/∂q dropped), so Ṁ − 2C = −Ṁ is symmetric
  and not skew; and a composite inertia accumulated without transforming the child inertia to the parent
  frame, which breaks the energy identity.
- KA-049 two-link chain: exact in rationals; tolerance on devices from the tier; G1 · all tiers. Control:
  taking the lower three rows of the space Jacobian as the point Jacobian.
- KA-050 point-velocity column: exact; G0 · host. Control: the same shortcut.
- KA-051 limb elimination: exact; G1 · host. Control: a limb packet treated as positive definite (a Cholesky
  factorization of the packet must fail; the mutant that inverts it at the port is caught).
- CX-023, CX-024.
- Metamorphic: re-expressing every quantity in another body frame leaves powers and energies unchanged;
  permuting sibling subtrees permutes results; a rigid motion of the whole system with gravity transformed
  leaves joint accelerations unchanged.
- Determinism: referee rollouts twice on one machine and across worker counts with equal digests.

## Considerations

- T-33 (skewness as a certified identity per limb) is Frontier and its symbolic budget is a HYPOTHESIS; until
  it is certified S3 is `Tested`.
- The law suite must not depend on the production step; it calls the referee through the airlock so that it
  exists before `frob-dyn`'s own tiers and then runs unchanged against them (one fixture, two rows).
- Risk: an implementation that passes parity against `fs-mbd` and inherits a shared defect. The exact fixtures
  here are independent of every sibling.
- Performance: none claimed here; the dense mass matrix per world is a DESIGN choice of the batched tiers.

## References

Plan §10 C-04; §11.2; §13.3, §13.10; §16; §12.3 (T-16, T-33); Appendix D.1; Appendix I.1 (limb port packet);
Appendix M KA-047 to KA-051; Appendix N CX-023, CX-024.
