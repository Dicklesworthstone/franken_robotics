---
key: "SPEC-C-05"
title: "frob-lie: C-05 specified — group-valued stepping, Cayley and exponential retractions; contract, law tests"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "marrow", "kinesis", "frob-lie", "solid", "frontier", "spine", "spec", "test"]
parent: "WP-03"
deps: ["BD-010", "BD-025", "N10a-01", "STD-TEST", "STD-E2E"]
needs: ["§10 C-36: the pilot, kill gate and ceiling of the articulated variational lane, which C-05 delegates to construction C-36 (SPEC-C-36 and its pilot bead, written by the unit that owns C-28 to C-40)"]
related: ["BD-011", "BD-012", "BD-017", "BD-019", "BD-020", "BD-021", "BD-087", "BD-384", "SPEC-C-36", "CARD-T-01", "CARD-T-02", "CARD-T-03", "CARD-T-06", "CARD-T-13", "N10-KA1"]
gates: ["GATE-MARROW-01", "GATE-LEMMA-00", "GATE-SPARK-00", "GATE-SPARK-01", "GATE-KIN-01"]
plan_refs: ["§10 C-05", "§13.2", "§13.3", "§13.7", "§13.8", "§13.9", "§12.3 T-01 T-02 T-03 T-06 T-13", "Appendix D.1", "Appendix E GB-01", "Appendix M KA-052 to KA-054", "Appendix N CX-025"]
---
## Background

Construction C-05 fixes how state advances in time: by a group operation, g₊ = g·τ(h·ξ), for a retraction τ
from the Lie algebra to the group. It replaces adding a quaternion increment and renormalizing, and
Runge–Kutta on Euler angles.

Payoff. In exact arithmetic the update is group-valued, so the only thing left to remove is round-off: one
normalization per step by a correctly rounded square root, a correction of order 10⁻⁷ in single precision
instead of the order-h² projection that an additive update needs. The Cayley update uses only +, −, ×, ÷ and
√, which IEEE 754 requires to be correctly rounded, so it is a candidate for bit-identical execution on CPU,
CUDA and Metal (HYPOTHESIS; the test that settles it is the device arithmetic probe GATE-SPARK-01). A rational
update is an object a polynomial identity checker can certify (theorem cards T-01, T-02). Bounded energy error
makes cost of transport trustworthy over long rollouts.

Decision ADR-MARROW-02 governs the choice of retraction: Cayley on the batched tiers, exponential on the
referee. Both exist on every tier and the choice is part of the model identity. Reason: the batched kernels may
contain no sine, cosine or fused multiply-add; the exponential needs trigonometric functions. Rejected: the
exponential everywhere (no bit-identical candidate across back ends, transcendental calls in the hot path);
additive quaternion updates (leave the group; need an order-h² projection).

Tier: Solid for the retractions and the semi-implicit lane; Frontier for the articulated variational lane,
whose pilot, kill gate and ceiling are those of construction C-36.

Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission): `fs-time` has a discrete
Euler–Poincaré step on SE(3) with `dep_momentum_adjoint` and a RATTLE projection hook; `fs-mbd` stepping is a
second-order Lie midpoint step that its own contract calls "neither an energy-exact nor a variational/symplectic
integrator".

## Scope

In: `docs/math/C-05.md`; the law suite `crates/frob-lie/tests/laws_c05.rs` with an exact rational reference for
the Cayley map; exact hand fixtures; the long-rollout residual test; the admission bound and its refusal; the
register row; tolerances and controls for KA-052 to KA-054; the statements of T-01, T-02, T-03 and what T-06
and T-13 would add.

Out: the production retractions and their derivatives (BD-011), SO(2) with its turn counter (BD-012), the
derived kernel `K-cay-step` (BD-019, BD-020), the free-body variational step as an oracle (BD-087), the
variational step as an algorithm (construction C-36), the default rollout integrator of the dynamics chapter
(semi-implicit Euler with implicit joint damping and a group update), which this construction constrains and
does not define.

## Technical approach

**The update.** g₊ = g·τ(h·ξ), with g a group element, ξ a body-frame algebra element (a twist for SE(3)), h
the step. Two retractions are supported.

- *Exponential* exp, the group exponential. On the referee its coefficient functions sin θ/θ, (1 − cos θ)/θ²
  and (θ − sin θ)/θ³ are evaluated by branch-free even polynomials in θ² below a declared small-angle
  threshold and by deterministic elementary functions above it, the two arms joined by a select; degree and
  threshold are chosen so the truncation error is below half a unit in the last place, and both are recorded
  in the kernel manifest (DESIGN: it removes the 0/0 forms without a branch). Referee only.
- *Cayley* cay(x) = (I − x/2)⁻¹·(I + x/2), which is rational and lands exactly on SO(3) and SE(3).
  - On SO(3), for ω ∈ ℝ³ with ω̂ its cross-product matrix: R = I + 4/(4 + ‖ω‖²)·(ω̂ + ω̂²/2).
  - On SE(3), cay(ω, v) has rotation cay(ω̂) and translation (I − ω̂/2)⁻¹·v.
  - On SO(2), with a unit complex number z and a = ω·h: z₊ = z·((1 − a²/4) + i·a)/(1 + a²/4), followed by one
    normalization; a turn counter is updated by a deterministic branch rule (ADR-MARROW-03).
  - Quaternion form: q₊ = q·c with c = (1, ω·h/2) normalized, and ‖q·c‖² = ‖q‖²·‖c‖² (T-03).

**The structure-preserving lane.** A discrete Lagrangian on the group. For a free rigid body, and for a rigid
body in a potential, its discrete Euler–Poincaré equations conserve exactly the discrete momentum map of every
symmetry the potential keeps, which is all of it for a free body (discrete Noether); the step is symplectic
and time-reversible. The step as an algorithm belongs to construction C-36.

**Honest limits (each is a statement the contract must carry).**

1. Cayley is second-order: it rotates by 2·atan(‖ω‖·h/2) instead of ‖ω‖·h. The relative shortfall is
   (‖ω‖·h)²/12 to leading order. Worked values (illustrative): ω = 20 rad/s and h = 2 ms give ω·h = 0.04, a
   relative error of 1.33 × 10⁻⁴ and a lag of 0.0027 rad/s; at ω·h = 0.2 the error is 0.33%; a requirement
   below 10⁻⁵ relative needs ω·h < 0.011.
2. Use of the Cayley map is bounded by ‖ω‖·h ≤ 0.2 per substep (DESIGN) and refused beyond: FROB-REF-212, "a
   Cayley update with ‖ω‖·h above 0.2 in one substep", returned with the substep count that admits it. The
   step planner avoids the refusal by substepping. The bound is tested as (ω·ω)·h² ≤ 0.04, a guarded predicate
   (single precision with a certified error bound and an exact integer fallback), because it decides a
   discrete outcome.
3. The default rollout integrator is semi-implicit Euler with implicit joint damping and a group update; the
   articulated variational integrator is Frontier.
4. Group membership and symplecticity are separate properties with separate tests.
5. No symplectic or energy claim is made for an articulated tree, whose mass matrix depends on configuration,
   and the property is moot once dissipative contact impulses enter.
6. A conservation claim names the quantity, the symmetry, the boundary conditions, the integrator and the
   tolerance.
7. The error is a lag in angle, not a drift off the group: the update stays on SO(3).

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-05.S1 | For skew-symmetric Ω, cay(Ω)ᵀ·cay(Ω) = I and det cay(Ω) = 1 (T-01) | Certified per kernel; Proved in general | Polynomial identity in the entries of Ω after clearing 4 + ‖ω‖²; Lean statement |
| C-05.S2 | The SE(3) Cayley update maps the group to itself (T-02) | Certified | Polynomial identity |
| C-05.S3 | The quaternion update preserves the norm: ‖q·c‖² = ‖q‖²·‖c‖² (T-03) | Certified; Proved through the quaternion algebra of Mathlib | Polynomial identity |
| C-05.S4 | Retraction laws: τ(0) = identity; dτ(0) = identity; τ(x)·τ(−x) = identity, for cay and exp | Tested | Property tests; exact for cay |
| C-05.S5 | Round trips: log(exp(x)) = x inside the injectivity radius; cay⁻¹(cay(x)) = x | Tested | Property tests |
| C-05.S6 | Ad(exp x) = exp(ad x) to tolerance; dexp and dcay match dual-number derivatives | Tested | Property tests against `fs-ad` |
| C-05.S7 | The Cayley step rotates by 2·atan(‖ω‖·h/2) | Certified (as the rational identity tan(θ/2) = ‖ω‖·h/2) | Half-angle identity on the rational map |
| C-05.S8 | A substep with ‖ω‖·h above 0.2 is refused | Tested | Guarded predicate against exact evaluation |
| C-05.S9 | Group-membership residual stays within four units in the last place over 10⁷ steps with the per-step normalization on, and grows no faster than a random walk with it off | Tested | Long rollout |
| C-05.S10 | Free flight under the structure-preserving step: momentum conserved; energy error bounded; measured order two by step halving | Tested | Referee rollouts |
| C-05.S11 | The Cayley discrete Euler–Poincaré step for a free rigid body conserves spatial momentum (T-06) | Certified if the ideal-membership computation fits its budget (HYPOTHESIS) | Identity modulo the update equations |

T-13 (discrete Noether for left-invariant discrete Lagrangians on a matrix Lie group, general, in Lean) is
Moonshot and not scheduled while the manifold and matrix-exponential modules it needs are blocked in the
re-checker; the contract lists it as not claimed.

**Hand fixtures (exact rational arithmetic).**

- ω·h = (0, 0, 2): R = I + ½·(ω̂ + ω̂²/2) = [[0, −1, 0], [1, 0, 0], [0, 0, 1]], a rotation by exactly π/2
  where the exponential would rotate by 2 rad. (An identity fixture; far outside the admission bound.)
- ω·h = (0, 0, 2), v·h = (1, 0, 0): translation (I − ω̂/2)⁻¹·v = (½, ½, 0).
- ω·h = (0, 0, 1/5): cos = (1 − 1/100)/(1 + 1/100) = 99/101 and sin = (1/5)/(101/100) = 20/101, with
  99² + 20² = 101²; the angle is 2·atan(0.1) = 0.199337 rad against 0.2, a relative shortfall of 0.33%.
- Planar form with a = 1/5 gives the same (99/101, 20/101).

**Residual definitions.** For a quaternion q the group-membership residual is |‖q‖² − 1|; for a rotation
matrix, the largest entry of |RᵀR − I|; for a unit complex number, |c² + s² − 1|. "No faster than a random
walk" is fixed here (DESIGN, TARGET) as residual(N) ≤ 8·u·√N for unit round-off u and step count N, checked at
N = 10³, 10⁴, …, 10⁷.

**Implementation map.** Crates `frob-lie` (retractions and derivatives) and `frob-dyn` (integrators). Type
sketch (DESIGN): `trait LieGroup { fn exp(x) -> Self; fn log(self) -> Algebra; fn cay(x) -> Self; fn
cay_inv(self) -> Algebra; … }`, with `cay` restricted to +, −, ×, ÷, √. Verbs: `sim.run` (the retraction is
part of the model identity and is echoed in the evidence block), `prove.kernel` for the certificate of
`K-cay-step`. Gates: GATE-MARROW-01 (retraction and round-trip laws, 10⁶ cases, 64 ulp `f64`, 16 ulp `f32`;
T-01, T-03, T-04 certified; M0); GATE-LEMMA-00 (T-01 and T-03 certified by FrankenSymPy for the M0 kernels;
certificates replay offline; M0); GATE-SPARK-00 (one dialect source for `K-cay-step` compiles and runs on CUDA
and on Metal and matches the Rust reference inside its class; M0); GATE-SPARK-01 (IEEE probe per device
family, whose outcome sets each device's best attainable determinism class; M0); GATE-KIN-01 (free-flight
momentum to 10⁻¹² relative over 10⁶ steps on the variational lane; M1).

## Acceptance criteria

- [ ] `docs/math/C-05.md` exists with both retractions, all formulas, the seven limits, the eleven statements
      and the hand fixtures.
- [ ] `cargo test -p frob-lie --test laws_c05` passes on the exact reference and on `frob-lie`.
- [ ] S1, S2, S3 hold exactly on 10³ random rational inputs; the three hand fixtures reproduce exactly.
- [ ] The Cayley angle equals 2·atan(‖ω‖·h/2) and the shortfall at ω·h = 0.2 is 0.33% to two digits.
- [ ] A substep with (ω·ω)·h² > 0.04 returns FROB-REF-212 with the admitting substep count
      ⌈‖ω‖·h/0.2⌉; at (ω·ω)·h² = 0.04 exactly it is admitted.
- [ ] Residual: at most 4 ulp over 10⁷ steps with normalization on; at most 8·u·√N with it off.
- [ ] Free flight: momentum conserved to 10⁻¹² relative over 10⁶ steps on the variational lane; energy error
      bounded with no secular drift; order 2.0 ± 0.1 by halving h over {4, 2, 1, 0.5} ms.
- [ ] Controls below registered and caught; KA-052 to KA-054 registered with tolerances and controls set; the
      register row of C-05 filled.

## Test plan

### Unit tests

- `s1_cayley_orthogonal_exact`: 10³ rational ω; RᵀR = I and det R = 1 exactly; fixture ω = (0, 0, 2).
- `s2_se3_cayley_group`: the bottom row stays (0, 0, 0, 1) and the rotation block satisfies S1; fixture
  translation (½, ½, 0).
- `s3_quaternion_norm`: exact on rational quaternions.
- `s4_retraction_laws`, `s5_round_trips`, `s6_derivatives`: over the boundary sample set (identity, tiny
  angles, rotations near π from both sides, large translations, q and −q), ulp bounds as in the gate.
- `s7_cayley_angle`: tan(θ/2) = ‖ω‖·h/2 exactly on rational inputs; numerical angle against 2·atan in `f64`.
- `s8_admission_bound`: inputs straddling 0.04 within 4 ulp; the guarded predicate agrees with exact rational
  evaluation on every input; the refusal carries the substep count.
- `s9_membership_residual`: 10⁷ steps of random bounded ω in `f32` and `f64`, normalization on and off.
- `s10_free_flight`: an asymmetric free body, inertia diag(1, 2, 3) kg·m², ω₀ = (1, 1, 0) rad/s; spatial
  angular momentum, energy and convergence order.
- `small_angle_polynomials`: the three coefficient functions against exact rational series at θ below and
  above the threshold; continuity across the select within half an ulp.

### End-to-end script and logging

`scripts/e2e/spec_c05.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G0 and G1 rows
KA-052 to KA-054; `frob --json prove kernel K-cay-step` and `frob --json prove verify` on the certificate
(class `Certified`, hash and checker identity present); `frob --json sim run --tier referee --robot
builtin:free-body --steps 1000000 --seed 3` with assertions on the momentum and membership residuals from the
envelope; one deliberately fast-spinning run that must end in refusal FROB-REF-212 or in a substepped run
whose record shows the substep count. Log `target/e2e-logs/spec_c05/<utc>.ndjson` with transcript, per-step
residuals in ulp against 4, energy-error series summary, measured order against 2, digests of the certificate,
evidence block, and on failure the first step and state at which a bound was exceeded. Final `summary` line.

### Known answers and negative controls

- KA-052 Cayley retraction: exact as polynomial identities; G0 · host. Control: an additive quaternion
  increment followed by renormalization (leaves the group before projection; fails S1 before normalization
  and shows an order-h² correction).
- KA-053 membership residual: four units in the last place; G1 · all tiers. Control (set here): the per-step
  normalization removed while the four-ulp bound is still asserted.
- KA-054 free flight: tolerances set to 10⁻¹² relative momentum over 10⁶ steps, bounded energy error with a
  drift slope indistinguishable from zero, order 2.0 ± 0.1; G1 · referee. Controls (set here): the implicit
  solve of the variational step evaluated with the differential at +h·ξ instead of −h·ξ, which keeps momentum
  exact and destroys the energy bound and reversibility; and an update that leaves the group, which breaks
  momentum.
- CX-025: Cayley used as if it were the exponential at any step size.
- Metamorphic: halving the time step moves results by the integrator's order.
- Determinism: the Cayley fixtures are exact; float tests twice on one machine, across worker counts, debug and
  release; across back ends each against the class the arithmetic probe assigns.

## Considerations

- The bit-identical hypothesis can fail on a device family; then that family is class D2 for itself and D3
  against others (kill gate KILL-D1) and nothing in this contract changes: the identities are over ℚ.
- OPEN-MARROW-1: quaternion sign is not canonicalized in stored state by default.
- The exponential's polynomial arm is referee-only; the Cayley lane needs no small-angle rule.
- Risk: reading "symplectic" into the default integrator. Limits 3 to 5 are in the contract to prevent it.
- The 0.2 bound is DESIGN; revisit if profiles show substepping dominates a class of robots (fast wheels), in
  which case the exponential lane on that tier is the alternative, not a looser bound.

## References

Plan §10 C-05; §13.2 (ADR-MARROW-02, -03), §13.3, §13.7, §13.8, §13.9; §12.3 (T-01, T-02, T-03, T-06, T-13);
Appendix D.1 (Cayley map, Cayley angle, planar Cayley); Appendix E GB-01; Appendix M KA-052 to KA-054;
Appendix N CX-025.
