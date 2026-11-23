---
key: "SPEC-C-36"
title: "spec C-36 core: the trapezoidal variational step on SE(3): Cayley differential, four-step map, its laws"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "frontier", "kinesis", "frob-dyn", "frob-lie", "test"]
parent: "WP-10"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-088", "BD-161", "BD-384", "N10c-08", "N10-CX", "N10c-10", "CARD-T-06", "CARD-T-13", "BD-011", "BD-087"]
gates: ["GATE-KIN-01"]
plan_refs: ["§10 C-36", "§10 C-05", "§16.7", "§12.3 T-06 T-13", "§38.18", "Appendix D.1", "Appendix D.2", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-36 of the FrankenRobotics spine gives two time-stepping algorithms: the trapezoidal variational
step for a rigid body on SE(3), and the discrete nonholonomic step for rolling constraints. This bead is the core
specification and covers the variational step; the nonholonomic step is bead N10c-08, which extends the same
contract file. Owner: the Kinesis subsystem (dynamics on three tiers), crate `frob-dyn`, with the Cayley map and
its differential in `frob-lie`. Tier: Frontier [F] for the variational lane as a whole; the free-body step itself
is used from M1 as a Solid known-answer oracle.

What it replaces: a midpoint step followed by a projection. Payoff, for a body in a potential: no secular energy
drift over long horizons, exact momentum along the symmetries the potential keeps, and a reversible map that an
adjoint can run backwards. The lane exists for trust in cost of transport over minutes of simulated time; no
throughput is claimed.

Design reasoning. Construction C-05 fixes that time stepping stays on the group through a retraction, and that the
Cayley map cay(x) = (I − x/2)⁻¹·(I + x/2) is rational and lands exactly on SO(3) and SE(3). A discrete Lagrangian
on the group then yields a step that conserves the discrete momentum map (discrete Noether) and is symmetric in
time. Three boundaries are fixed in advance. (1) The four steps are derived for one rigid body. For an articulated
tree the mass matrix depends on configuration, a kick–drift splitting is no longer the variational step, and no
symplectic claim is made; "the articulated variational step is symplectic" was considered as a theorem card and
rejected because no checker decides an analytic property of a map. (2) Dissipative contact ends energy
conservation by design. (3) Step 2 is implicit, so the lane serves the referee, the strict CPU profile, adjoints
and evaluation, while the throughput tiers keep the semi-implicit default. FACT (FrankenSim `483d09c682f2`,
observed 2026-11-21): `fs-time` has a discrete Euler–Poincaré step on SE(3); the sibling multibody stepper is a
second-order Lie midpoint step that its own contract calls neither energy-exact nor variational.

Terms. g ∈ SE(3) is the body pose; ξ = (ω, v) the body twist, angular part first; 𝕄 the 6×6 spatial inertia in
the body frame; μ a body momentum (a covector); h the step; Ad_c the adjoint of a group element c; x̂ the 3×3
cross-product matrix of a vector. *Left-trivialized differential* of the Cayley map: cay(x)⁻¹·δcay(x) =
Dcay_x(δx). Proof classes: Proved (sealed Lean capsule), Certified (instance certificate re-checked exactly),
Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-36.md` part A; law tests in `crates/frob-lie/tests/laws_c36_dcay.rs` (the differential) and
`crates/frob-dyn/tests/laws_c36_step.rs` (the four-step map); registry entries for KA-172, KA-173, KA-174, the
late-adopted row "the Cayley map on SE(3)", and the control seeded from CX-087.

Out: the free-body step as an oracle in the product (BD-087); the dynamics known-answer battery and GATE-KIN-01
(BD-088); the articulated variational integrator, its cost pilot and KILL-VARIATIONAL (BD-384); the Cayley
retraction and its certificates (BD-011, theorem cards T-01, T-02); the nonholonomic step (N10c-08).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-36: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`sim.run` on the referee tier), `failing_test` (a
test of `frob-dyn::laws_c36_step` with its registered control), `payoff.gate` (GATE-KIN-01), `statements`,
`known_answers` (KA-172 to KA-177), `cards` (T-06, T-13), `gates`, `pilot` (the record `PILOT-C-36` of the pilot
register, required because the tier is Frontier) and `generic_comparator` (the off-the-shelf method named in the
pilot paragraph of this bead). Doctrine D-2 (no ornamental mathematics) is checked on these fields: a construction
with no verb or no failing test is moved to the parked register by a decision record. The part bead N10c-08 adds
their statements and tests to the same row. The contract `docs/math/C-36.md` has the ten sections of the template
`docs/math/TEMPLATE.md` of the same bead, in the template's order (object and definitions; what it replaces and
the payoff; conventions; statements; formulas; fixtures and known answers; failure cases, refusals and retirement;
non-claims; foundations; implementation map); content that this bead lists under other headings goes into the
template section that matches. Statement identifiers are entered in the contract and in the register with the
construction prefix (`C-36.S1` and so on, as the template requires); the tables of this bead and of its part beads
abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-011 (the production
retractions of `frob-lie`) or BD-087 (the free-body variational step, which implements this construction): those
beads are listed in `related` and are not prerequisites. The law suite is generic over its subject trait and
closes on the exact reference model and the mutants in its own test support. If `frob-lie` or `frob-dyn` do not
exist when this bead starts, the suite and its support files are written under `crates/frob-conformance/tests/`
with the same file names (that crate exists from BD-025) and the commands below read `-p frob-conformance`; the
bead that implements the construction moves them into the owning crate, binds its production types to the subject
trait and runs the unchanged suite as part of its own acceptance. A criterion or script step below that names
production code or a verb of an implementing bead is owed at that bead's acceptance and is logged `pending` here
until then.

## Technical approach

### 1. Definitions

Discrete Lagrangian for a rigid body with inertia 𝕄, potential V and applied body wrench u:

```text
L_d(g_k, g_k₊₁) = (h/2)·ξ_kᵀ·𝕄·ξ_k − (h/2)·(V(g_k) + V(g_k₊₁)),      g_k₊₁ = g_k·cay(h·ξ_k)
```

For x = (ω, v) the inverse of the left-trivialized differential is

```text
(Dcay_x)⁻¹ = [[ I + ½·ω̂ + ¼·ω·ωᵀ ,  0        ],
              [ ½·v̂ + ¼·ω̂·v̂      ,  I + ½·ω̂ ]]
```

Write f(g) for the body-frame wrench of the potential. The discrete momentum at node k is
μ_k = (Dcay_{−hξ_k})⁻ᵀ·𝕄·ξ_k − (h/2)·f(g_k).

### 2. The map (g_k, μ_k) ↦ (g_k₊₁, μ_k₊₁), four steps

1. Half kick: μ⁺ = μ_k + (h/2)·(f(g_k) + u_k).
2. Implicit solve: find ξ_k with (Dcay_{−hξ_k})⁻ᵀ·𝕄·ξ_k = μ⁺, by Newton's method started from the previous ξ, with
   the closed-form 6×6 Jacobian.
3. Drift and transport: c = cay(h·ξ_k), g_k₊₁ = g_k·c, μ° = Ad_cᵀ·μ⁺.
4. Half kick: μ_k₊₁ = μ° + (h/2)·(f(g_k₊₁) + u_k₊₁).

Explicit form of step 2, derived here from the matrix above with 𝕄·ξ_k = (π, ℓ) (angular and linear parts), to
be confirmed by the differential law test: the angular part of μ⁺ is (I + (h/2)·ω̂ + (h²/4)·ω·ωᵀ)·π +
((h/2)·v̂ + (h²/4)·v̂·ω̂)·ℓ, and the linear part is (I + (h/2)·ω̂)·ℓ. The Jacobian of the residual is polynomial in
ξ; deriving it and writing it into the contract is part of this bead.

### 3. Statements

| Id | Statement | Class |
|---|---|---|
| V1 | The printed matrix is the inverse of the left-trivialized differential: (Dcay_x)⁻¹·Dcay_x = I. Its rotation block I + ½·ω̂ + ¼·ω·ωᵀ inverts to 2/(4 + ‖ω‖²)·(2·I − ω̂). | Tested exactly (the map is rational, so its differential is computed by dual numbers over ℚ); a polynomial identity |
| V2 | The left-trivialized differential at −x equals the right-trivialized differential at +x. The operator of step 2 is therefore the printed matrix at x = −h·ξ_k, transposed, or equally the right-trivialized one at +h·ξ_k. | Tested exactly |
| V3 | Discrete Noether: step 3 leaves the spatial momentum Ad_g⁻ᵀ·μ unchanged, because Ad_{g·c}⁻ᵀ·Ad_cᵀ = Ad_g⁻ᵀ. This holds for any ξ. | Tested exactly; card T-06 (identity modulo the update equations, [F], budget a HYPOTHESIS); the general theorem is card T-13 ([M], Lean) |
| V4 | The map is a symmetric composition of kick, drift and kick, so it is time-reversible and of second order. | Tested (reversal to round-off; error ratio per halving of h) |
| V5 | For a free body, spatial momentum is conserved exactly; with a potential, the momentum of every symmetry the potential keeps is conserved (with uniform gravity: vertical angular momentum and horizontal linear momentum). | Tested |
| V6 | The energy error stays bounded over long horizons and is divided by four when h is halved. | Tested (no secular drift; ratio between 3.6 and 4.4) |
| V7 | Evaluating step 2 with the differential at +h·ξ_k (the other trivialization) leaves momentum exact, because step 3 conserves it for any ξ, and destroys the energy bound. The step is therefore tested by energy and reversibility, never by momentum alone. | Tested (the negative control) |

### 4. Conventions, failure cases and non-claims

- SE(3) Cayley map: rotation cay(ω̂) = I + 4/(4 + ‖ω‖²)·(ω̂ + ω̂²/2), translation (I − ω̂/2)⁻¹·v. Check: ω = (0, 0, 2),
  v = (1, 0, 0) give a quarter turn about z and translation (1/2, 1/2, 0); a map that pairs cay(ω̂) with v
  unchanged returns (1, 0, 0) and fails V1.
- Use is bounded by ‖ω‖·h ≤ 0.2 per substep and refused beyond (the Cayley step rotates by 2·atan(‖ω‖·h/2), not
  ‖ω‖·h).
- Nodal energy for the tests: H(g, μ) = ½·μᵀ·𝕄⁻¹·μ + V(g) at nodes.
- Not claimed: anything for an articulated tree; any symplectic property beyond one rigid body; any energy
  statement with dissipative contact; any throughput.

### 5. Law-test layout, registry, pilot and gates

`crates/frob-lie/tests/laws_c36_dcay.rs` for V1 and V2 in exact rationals; `crates/frob-dyn/tests/laws_c36_step.rs`
with `tests/support/c36_ref.rs`: the four steps written directly in `f64` with a Newton solve to 10⁻¹³ relative,
and the transport identity in exact rationals. Generic over a trait `C36StepSubject` so the production step of
BD-087 runs the same laws. Mutants in `tests/support/c36_mutants.rs`. Registry: KA-172 to KA-174 with derivation
files stating the tolerances below; the SE(3) Cayley row (no printed KA number; appended); the control from CX-087.

Pilot of the construction, for the variational step on the batch tiers (run by BD-384): cost of step 2 against the
semi-implicit step at matched energy error on one free body and one heavy top. Kill (TARGET): more than twice the
cost at matched accuracy, in which case the step stays in the referee and the strict profile only. The owning
chapter's pilot adds: the same two fixtures at h = 2 ms and 0.25 ms; falsifier, an energy error that does not fall
by a factor between 3.6 and 4.4 per halving of h. Ceiling: energy fidelity of long rollouts. Gate GATE-KIN-01 at
M1 includes "free-flight momentum conserved to 10⁻¹² relative over 10⁶ steps on the variational lane". Kill gate
KILL-VARIATIONAL (M5): the implicit solve costs more than twice a semi-implicit step on the CPU batch tier, or the
articulated form fails its momentum row after one repair cycle; fallback: the free-body step is kept as a
known-answer oracle and every rollout uses the default integrator.

## Acceptance criteria

- [ ] `docs/math/C-36.md` exists with part A: the definitions, the four steps, the explicit form of step 2 and
      the derived 6×6 Jacobian, V1 to V7, the conventions and the non-claims.
- [ ] `cargo test -p frob-lie --test laws_c36_dcay` and `cargo test -p frob-dyn --test laws_c36_step` pass on the
      reference model and on the production step of BD-087.
- [ ] Every mutant below fails its named law, and `wrong_trivialization` is shown to pass the momentum law.
- [ ] KA-172, KA-173 and KA-174 are registered with derivation files and stated tolerances (they were printed
      `to set`); KA-174, printed with no control, has the control named below.
- [ ] `scripts/e2e/spec_c36_step.sh` exits 0.
- [ ] Row C-36 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `ka_172_dcay_inverse_identity`: 10³ seeded rational x with ‖ω‖ ≤ 2: the printed matrix times the exact
  differential is the identity, exactly; the finite-difference variant agrees to 10⁻⁷.
- `rotation_block_inverse`: (I + ½·ω̂ + ¼·ω·ωᵀ)·(2/(4 + ‖ω‖²))·(2·I − ω̂) = I exactly for 10³ rational ω.
- `left_at_minus_equals_right_at_plus`: V2 exactly.
- `cayley_se3_worked`: ω = (0, 0, 2), v = (1, 0, 0) give the quarter turn and (1/2, 1/2, 0).
- `noether_transport_identity`: 10³ seeded rational g and c: Ad_{g·c}⁻ᵀ·Ad_cᵀ = Ad_g⁻ᵀ exactly.
- `jacobian_matches_finite_difference`: the derived Jacobian of the step-2 residual agrees with central
  differences to 10⁻⁷ relative.
- `newton_converges`: for ‖h·ξ‖ ≤ 0.2 the solve reaches 10⁻¹³ relative in at most three iterations (TARGET).
- `ka_173_free_asymmetric_body`: rotational inertia diag(1, 2, 3) kg·m², mass 2 kg, ξ₀ = ((0.3, 1.0, 0.2),
  (0.1, −0.2, 0.05)), h = 2 ms: over 10⁵ steps (10⁶ in the conformance row) the spatial momentum drifts by at most
  10⁻¹² relative; the maximum energy error at h and at h/2 has ratio in [3.6, 4.4]; 10⁴ steps forward and 10⁴
  with −h return the initial state to 10⁻¹⁰ relative.
- `ka_174_gravity_symmetries`: the same body with its centre of mass 0.1 m from the body origin in uniform
  gravity: vertical angular momentum about the world origin and horizontal linear momentum drift by at most 10⁻¹²
  relative over 10⁵ steps.
- `second_order_in_pose`: the pose error at a fixed final time has ratio in [3.6, 4.4] per halving of h.
- `substep_bound_refused`: a step with ‖ω‖·h = 0.25 is refused.

### End-to-end script and logging

`scripts/e2e/spec_c36_step.sh`: `frob --json doctor`; the two cargo test targets; `cargo test -p frob-conformance
--test g0 -- KA-172`; `cargo test -p frob-conformance --test g1 -- KA-173 KA-174`; `frob --json prove list`
filtered to cards T-06 and T-13; file and registry checks. Log to `target/e2e-logs/spec_c36_step/<utc>.ndjson`
with a transcript; each line has `ts`, `script`, `bead`, `step`, `event`, command line, exit code, wall time, the
evidence block (claim Known answer, class Tested, tier, determinism D0 for rational rows and the host class for
`f64` rows, seed), momentum drift, energy-error ratio and reversal error with their thresholds, and on failure
the first step index at which a bound is exceeded. The `summary` line counts assertions and names GATE-KIN-01.

### Known answers and negative controls

| Row | Truth | Tolerance (stated here; printed `to set`) | Tier | Control |
|---|---|---|---|---|
| KA-172 | The printed matrix times the left-trivialized differential is the identity; the rotation block inverts to 2/(4 + ‖ω‖²)·(2·I − ω̂) | exact in rationals; 10⁻⁷ for the finite-difference form | G0 host | The differential taken at +h·ξ_k |
| KA-173 | Free asymmetric body: spatial momentum constant; energy error bounded and divided by four per halving; reversal returns the initial state | momentum 10⁻¹² relative over 10⁶ steps; ratio in [3.6, 4.4]; reversal 10⁻¹⁰ relative | G1 referee, strict | Mutant `wrong_trivialization`: momentum still exact, energy ratio and reversal fail |
| KA-174 | With gravity: vertical angular momentum and horizontal linear momentum constant | 10⁻¹² relative | G1 referee, strict | Mutant `kick_in_wrong_frame` (new): applies the potential wrench without transport, breaking the symmetry momenta |
| The Cayley map on SE(3) (late-adopted row) | Quarter turn about z; translation (1/2, 1/2, 0) | exact | G0 host | Mutant `translation_unchanged`: returns (1, 0, 0) and fails KA-172 |

Other mutants: `transport_by_inverse_adjoint` (uses Ad_c⁻ᵀ in step 3; momentum fails); `single_kick`
(kick–drift only; first order). Counterexample carried: CX-087 (momentum conservation does not show that the
implicit solve is right). Related, owned elsewhere: the entry that the articulated step is not symplectic.
Determinism lanes: twice on one machine; debug and release; the production binding on the referee and the strict
profile.

## Considerations

- Wording to pin down. The plan describes the control three ways ("the differential at +h·ξ_k", "the
  right-trivialized form", "a right-trivialized differential in the solve"). By V2 the right-trivialized
  differential at +h·ξ_k is the correct operator; the defect is the left-trivialized matrix at +h·ξ_k (equally
  the right-trivialized one at −h·ξ_k). The contract states the operator by its matrix and its argument so the
  name cannot mislead.
- The tolerances in the table are DESIGN proposals of this bead where the plan printed `to set`; the momentum
  figure is the gate's. The Kinesis lead owns the final values.
- Cost: step 2 is a 6×6 Newton solve per body per step; that is why the lane is not on the throughput tiers.
- If KILL-VARIATIONAL fires, this contract is unchanged: the free-body step remains the oracle.

## References

Plan §10 C-36 (variational step), C-05; §16.7; theorem cards T-01, T-02, T-06, T-13; Appendix D.1 "Cayley map",
"Inverse Cayley differential"; Appendix D.2 "Trapezoidal variational step"; §38.18 row "The Cayley map on SE(3)";
KA-172 to KA-174; CX-087; GATE-KIN-01; KILL-VARIATIONAL; BD-087, BD-384.
