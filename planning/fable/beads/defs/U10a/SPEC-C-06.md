---
key: "SPEC-C-06"
title: "frob-algebra: C-06 specified — contact as a cone program, friction cone as a spin-factor cone; law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "traction", "marrow", "frob-algebra", "frob-contact", "solid", "frontier", "spine", "spec", "test"]
parent: "WP-03"
deps: ["BD-025", "N10a-01", "STD-TEST", "STD-E2E"]
needs: ["§18.16: the contact known-answer battery, whose rows are the 'known answers of the contact chapter' this construction's test names (BD-085 on the referee, BD-168 on batched tiers)", "§18.19: the definition of the rounding margin ρ of a computed residual, used in the factor-reuse bound", "§10 C-37: the four-dimensional contact cone with a spin component and the joint-level rows (SPEC-C-37, another unit)"]
related: ["BD-079", "BD-080", "BD-085", "BD-115", "BD-153", "BD-266", "BD-367", "BD-386", "N10a-14", "CARD-T-07", "CARD-T-08", "N10-KA1", "BD-050"]
gates: ["GATE-MARROW-01", "GATE-TRAC-01", "GATE-TRAC-09"]
plan_refs: ["§10 C-06", "§13.1", "§13.8", "§18", "§12.3 T-07 T-08", "Appendix D.3", "Appendix I.1", "Appendix M KA-055 to KA-057", "Appendix N CX-026"]
---
## Background

Construction C-06 states what contact is, mathematically: a cone program. At a contact frame the impulse must
lie in the friction cone, with complementarity along the normal and maximal dissipation in the tangent plane.
The friction cone is a linear image of the second-order cone, and the second-order cone is the cone of squares
of a Jordan algebra. Naming that structure is not ornament: one `Cone` abstraction (Jordan product, spectral
decomposition, projection, barrier) then serves contact, joint limits, the control programs of the estimation
and control subsystem, and the certificates of the proof subsystem.

It replaces projected Gauss–Seidel with softness knobs that have no physical meaning.

Payoff:

- One cone abstraction for four consumers.
- The fast solve is strictly convex, so it has one solution and its gradient norm bounds the distance to it:
  solver error is a recorded number per step.
- Parameters are physical: stiffness, dissipation time, friction coefficient.
- The known artifacts of the convex model (bounded creep of a stuck contact, energy stored in compliance) are
  named and measured against the referee in the fidelity ledger.
- On a fixed active set the converged factor is reused for sensitivity and counterfactual right-hand sides
  (Frontier), each column with an error bound and a guard.

Tier: Solid for the convex compliant model; Frontier for pressure-field patches, barrier contact, the
hard-contact batch law and factor reuse (pilot N10a-14).

Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission):
`fs-contact::frictional_multicontact::solve_frictional_contacts` (simultaneous Signorini–Coulomb by semismooth
Newton–Krylov; `MAX_CONTACTS = 21` in `fs-contact/src/sustained/solve.rs`; an event solve without persistent
time stepping); elliptic friction; Hertz, Hunt–Crossley and JKR normal laws; `fs-tribo` friction laws.
FrankenSim's persistent stepper lanes (Moreau–Jean, IPC) are open work items without code. The sustained
contact step is reported to assume a fixed witness (FACT to recheck; it becomes an admission test and an
upstream work item before anything depends on it).

## Scope

In: `docs/math/C-06.md`; the law suite `crates/frob-algebra/tests/laws_c06.rs` (cone laws, exact reference) and
`crates/frob-contact/tests/laws_c06_model.rs` (model-form statements, once that crate exists); the pyramid
counterexample; the factor-reuse bound; the register row; tolerances and controls for KA-055 to KA-057; the
statements of T-07 (three-dimensional cone) and T-08.

Out: the `Cone` trait and its instances (BD-050); the cone-program solver (BD-115); collision and contact sets
(BD-079); the referee step and contact battery (BD-080, BD-085); the model-form rank check (BD-266); the
response batch (BD-367) and the hard-contact batch law (BD-386); the four-dimensional cone and joint rows
(construction C-37).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-050 (the `Cone` trait and
the friction cone of `frob-algebra`): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-algebra` or `frob-contact` do not exist when this bead starts, the suite and its support files are written
under `crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands
below read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate,
binds its production types to the subject trait and runs the unchanged suite as part of its own acceptance. A
criterion or script step below that names production code or a verb of an implementing bead is owed at that bead's
acceptance and is logged `pending` here until then.

## Technical approach

**The object.**

- *Friction cone.* At a contact frame the impulse is γ = (γₙ, γₜ) with γₙ the normal component and γₜ ∈ ℝ² the
  tangential part. It lies in 𝒦_μ = {‖γₜ‖ ≤ μ·γₙ}, μ the friction coefficient. The dual cone is 𝒦_{1/μ}.
- *Contact law.* Signorini complementarity along the normal (no penetration, no adhesion, and gap times normal
  impulse is zero) and maximal dissipation in the tangent plane (the friction impulse opposes sliding and
  maximizes the dissipation rate among admissible impulses).
- *Jordan structure.* 𝒦_μ is the image of the second-order cone {(x₀, x̄) : ‖x̄‖ ≤ x₀} under the linear map
  (γₙ, γₜ) ↦ (μ·γₙ, γₜ). The second-order cone is the cone of squares of the spin-factor Jordan algebra with
  product (x₀, x̄) ∘ (y₀, ȳ) = (x₀·y₀ + x̄·ȳ, x₀·ȳ + y₀·x̄). Its elements have eigenvalues λ± = x₀ ± ‖x̄‖ and
  spectral decomposition x = λ₊·c₊ + λ₋·c₋ with c± = ½·(1, ±x̄/‖x̄‖). The projection onto the cone is
  P(x) = max(λ₊, 0)·c₊ + max(λ₋, 0)·c₋.
- *The other symmetric cones that occur.* The nonnegative orthant (joint limits) and the
  positive-semidefinite cone (pseudo-inertia, sum-of-squares certificates).
- *The fast tiers* solve a convex, compliant problem in generalized velocities:
  v = argmin ½·(v − v*)ᵀ·A·(v − v*) + Σ_c ℓ_c(J_c·v), where v* is the unconstrained velocity, A is positive
  definite (the mass matrix plus implicit terms), J_c the Jacobian of contact c, and ℓ_c a convex potential;
  minus the gradient of ℓ_c is the contact impulse, γ_c = −∇ℓ_c.
- *The referee tier* solves non-associated Coulomb friction with exact complementarity. Its written
  specification is the hard-contact law, a nonlinear complementarity problem and not one convex program; that
  law is also offered as an optional batch law (Frontier).

**Euclidean projection onto 𝒦_μ in closed form** (the contract's working formula). For x = (n, t):
(i) if ‖t‖ ≤ μ·n, P(x) = x; (ii) if μ·‖t‖ ≤ −n, P(x) = 0 (x is in the polar cone); (iii) otherwise
P(x) = s·(1, μ·t/‖t‖) with s = (n + μ·‖t‖)/(1 + μ²).

**Consequences the contract states.**

- *Strict convexity and the error number.* The Hessian of the objective is A + Σ_c J_cᵀ·∇²ℓ_c·J_c, a sum of a
  positive-definite matrix and positive-semidefinite terms. Hence one minimizer v°, and for any iterate v,
  ‖v − v°‖ ≤ ‖∇F(v)‖/λ_min(A), where F is the objective. The step records ‖∇F‖ and the bound.
- *Factor reuse on a fixed active set* (Frontier). For a system A·x = b solved with a reused factor, each
  column x̂ carries the bound ‖x̂ − x*‖ ≤ (‖b − A·x̂‖ + ρ)/λ, with ρ the rounding margin of the computed
  residual and λ a lower bound on the smallest eigenvalue of A; and a guard reroutes any column that crosses a
  stick, slip or open boundary to the full solve.
- *Model-form honesty.* A design ranking that reverses between the compliant law and the hard-contact law is
  reported as a fidelity-dependent ranking and cannot be promoted.
- *No pyramid.* A pyramid approximation of the cone is not used: the box |γₜ₁| ≤ μ·γₙ, |γₜ₂| ≤ μ·γₙ admits
  (μ·γₙ, μ·γₙ), of norm √2·μ·γₙ, which is 41% of traction that does not exist along the diagonals
  (√2 − 1 = 0.4142).

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-06.S1 | The friction-cone projection satisfies P(x) ∈ 𝒦, P(x) − x ∈ 𝒦*, ⟨P(x), P(x) − x⟩ = 0 (T-07, three-dimensional cone) | Certified per case | Per-case polynomial identities and inequalities (cases i to iii), with ‖t‖ replaced by a witness r and r² = t·t |
| C-06.S2 | P(P(x)) = P(x) (idempotence); Moreau decomposition x = P_𝒦(x) + P_𝒦°(x) with orthogonal parts | Tested | Property tests; exact at rational points with rational norms |
| C-06.S3 | Jordan identities of the spin factor: commutativity, x ∘ (x² ∘ y) = x² ∘ (x ∘ y), and the cone is {x ∘ x} | Certified | Polynomial identities |
| C-06.S4 | The fast-tier objective has a positive-semidefinite Hessian by construction (T-08) | Certified structurally; Proved in general | A sum of positive-semidefinite terms; Lean statement; exact LDLᵀ on the rational fixture |
| C-06.S5 | ‖v − v°‖ ≤ ‖∇F(v)‖/λ_min(A) for every iterate | Tested | Property test on random strictly convex instances |
| C-06.S6 | Each reused column satisfies ‖x̂ − x*‖ ≤ (‖b − A·x̂‖ + ρ)/λ; a column crossing a stick, slip or open boundary is rerouted | Tested | Property test with planted boundary crossings |
| C-06.S7 | The pyramid cone admits impulses of norm up to √2·μ·γₙ; a ranking won by diagonal traction is caught by the model-form check | Certified (the arithmetic); Tested (the catch) | Exact arithmetic; planted design |
| C-06.S8 | γ_c = −∇ℓ_c lies in 𝒦_μ for every admissible ℓ_c | Tested | Property test per contact law |

**Fixtures** (exact; hand-checkable).

- *Projection, boundary case.* μ = ½, x = (n, t) = (0, (1, 0)). Not inside (1 > 0); not polar (½ > 0);
  s = (0 + ½)/(1 + ¼) = 2/5; P(x) = (2/5, (1/5, 0)). Checks: ‖(1/5, 0)‖ = μ·(2/5); P(x) − x = (2/5, (−4/5, 0))
  lies on the boundary of the dual cone 𝒦₂ (4/5 = 2·(2/5)); ⟨P(x), P(x) − x⟩ = 4/25 − 4/25 = 0.
- *Projection, polar case.* μ = ½, x = (−1, (1, 0)): μ·‖t‖ = ½ ≤ 1, so P(x) = 0, and −x = (1, (−1, 0)) ∈ 𝒦₂.
- *Pyramid.* μ = 1, γₙ = 1: the box admits (1, 1) of norm √2; the cone's limit along the diagonal is
  (1/√2, 1/√2). Excess 41.4%.
- *Convex instance.* A = diag(2, 3) and one contact with ℓ(u) = ½·k·max(0, −u)² on u = v₁ (a one-sided
  quadratic penalty with k = 4): Hessian diag(2 + 4·[v₁ < 0], 3), exact pivots positive.

**Failure cases.** At the apex of the cone (x = 0) the spectral frames c± are not unique; the projection is
still 0 and the Jacobian of the projection is set-valued, so a derivative query there is labelled generalized
or refused by the derivative-labelling rule. A contact set above the referee's capacity of 21 contacts is a
typed refusal, never a truncated solve.

**What it does not claim.** The convex compliant law is not Coulomb friction with exact complementarity: a
stuck contact creeps within a bound, and compliance stores energy; both are measured, not denied. Strict
convexity gives uniqueness of the velocity solution, not physical correctness. The factor-reuse bound holds on
a fixed active set only.

**Implementation map.** Crates `frob-algebra` (`Cone`) and `frob-contact` (laws, solver wiring). Type sketch
(DESIGN): `trait Cone { type Point; fn jordan(a, b) -> Point; fn spectral(a) -> Spectrum; fn project(a) ->
Point; fn barrier(a) -> f64; }` and `trait ContactLaw { fn potential(&self, vc: ContactVelocity, p:
&InterfaceCard) -> ConvexTerm; }`. Verbs: `sim.run`; `contact.probe` and `contact.laws`; the control verbs that
solve cone programs. Gates: the "Cones" law of GATE-MARROW-01 (P(P(x)) = P(x); Moreau decomposition with
orthogonal parts); GATE-TRAC-01 (every Solid row of the contact battery passes on the strict and batched CPU
tiers and on the referee where expressible; fast-law rows pass on GPU tiers within the stated creep; M1 and
M2); GATE-TRAC-09 (hard-contact batch law: residual at most 10⁻⁴ within eight passes on at least 95% of battery
contact states and 10⁻⁶ under the strict profile; M5). Cards: T-07, T-08.

## Acceptance criteria

- [ ] `docs/math/C-06.md` exists with the object, the projection formula, the four consequences, the eight
      statements and the fixtures.
- [ ] `cargo test -p frob-algebra --test laws_c06` passes on the exact reference and on the production cones
      once BD-050 lands.
- [ ] S1 holds exactly on 10³ rational points whose tangential norm is rational (Pythagorean tangents) in each
      of the three cases, and within 64 ulp in `f64` on 10⁶ random points.
- [ ] The boundary and polar fixtures reproduce exactly: (2/5, (1/5, 0)) and 0.
- [ ] The Hessian of the convex instance has positive exact pivots; a planted negative-curvature term is
      rejected.
- [ ] The pyramid arithmetic gives √2 and the planted pyramid law is caught by the model-form check.
- [ ] A planted column crossing a boundary without reroute is caught by S6.
- [ ] KA-055 to KA-057 registered with tolerances and controls set; the register row of C-06 filled.

## Test plan

### Unit tests

- `s1_projection_three_cases_exact`: generated rational points, e.g. t = (3, 4)·q with ‖t‖ = 5·q; all three
  identities per case.
- `s1_fixtures`: the boundary and polar fixtures.
- `s2_idempotent_moreau`: 10⁶ random points; x − P_𝒦(x) equals the projection onto the polar cone and the two
  parts are orthogonal.
- `s3_jordan_identities`: 10³ random rational pairs.
- `s4_hessian_psd`: the convex instance; 10³ random instances with exact LDLᵀ (nonnegative pivots; positive
  when A ≻ 0).
- `s5_gradient_bound`: solve each instance to high accuracy, perturb, check the bound.
- `s6_factor_reuse_bound`: reuse a factor on right-hand sides that keep the active set (bound holds) and on
  one that moves a contact from stick to slip (guard reroutes).
- `s7_pyramid_excess`: norm √2 exactly as a squared norm of 2.
- `s8_impulse_in_cone`: per admitted contact law, random contact velocities.

### End-to-end script and logging

`scripts/e2e/spec_c06.sh`. Sources the STD-E2E helper. Steps: law suites; law suites per control; G0 and G1
rows KA-055 to KA-057; when the verbs exist, `frob --json contact probe` on a block on an incline at the
friction threshold (asserting the cone used is the second-order cone and the recorded solver error bound is
present), and `frob --json fidelity modelform` on the planted pyramid design (asserting the ranking is typed
`model-conditional` or the exploit is flagged). Log `target/e2e-logs/spec_c06/<utc>.ndjson` with transcript:
commands, exit codes, residuals and bounds with units, the evidence block, and on failure the first point at
which an identity failed. Final `summary` line.

### Known answers and negative controls

- KA-055 pyramid-cone exploit: exact; G1 · lockstep. Control: the pyramid cone planted as the contact law.
- KA-056 cone identities: tolerance set to exact over ℚ at rational-norm points and 64 ulp in `f64`; Hessian
  pivots nonnegative exactly; G0 · host. Controls (set here): a projection that only raises the normal
  component (γₙ ← max(γₙ, ‖γₜ‖/μ)), which lands in the cone and violates orthogonality; a planted contact
  potential with negative curvature.
- KA-057 factor reuse: the stated bound (the registry row prints it without ρ; the test uses the form with
  ρ); G1 · lockstep. Control: a column that crosses a boundary and is not rerouted.
- CX-026: the friction cone approximated by a pyramid.
- Further controls: the dual cone taken as 𝒦_μ instead of 𝒦_{1/μ} (fails S1 for μ ≠ 1); a non-idempotent
  projection.
- Metamorphic: rotating the tangent basis leaves projections and impulses unchanged (the pyramid fails this);
  a ranking survives the change of law with top-k overlap at least k − 1 or is typed `model-conditional`.
- Determinism: exact tests D0; float tests twice, across worker counts, debug and release.

## Considerations

- The hard-contact batch law and factor reuse can be killed: falsifier (TARGET), a residual above the declared
  tolerance at a fixed iteration budget on more than 1% of contact islands, in which case the law stays the
  referee's specification and is not offered in batch. Ceiling: fidelity for the law; for factor reuse, the
  solve share of a counterfactual burst.
- The reported fixed-witness assumption in the upstream sustained step is unconfirmed; nothing in this contract
  depends on it.
- Risk: creep and stored compliance energy hidden inside tolerances. The fidelity ledger rows are the guard,
  and the contract names both artifacts.
- The four-dimensional cone inherits every statement here through its own linear map.

## References

Plan §10 C-06; §13.1, §13.8; §18; §12.3 (T-07, T-08); Appendix D.3 (friction cone, spin-factor product,
fast-tier contact problem); Appendix I.1; Appendix M KA-055 to KA-057; Appendix N CX-026. Pilot: N10a-14.
