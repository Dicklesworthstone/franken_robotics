---
key: "CARD-T-07"
title: "Theorem card T-07: the friction-cone projection satisfies the Moreau identities (3- and 4-dimensional cones)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["N12-22", "N12-20", "BD-154", "N12-06", "SPEC-C-06", "SPEC-C-37"]
needs: ["§18.6: the cone kernels K-cone-eval and the Cone trait's project function on which the certificate is issued (BD-050, BD-153)"]
related: ["BD-379", "N12-16", "CARD-T-08", "N12-21"]
gates: []
plan_refs: ["§12.3 (T-07)", "§10 C-06", "§10 C-37", "§18.12 (PO-TRAC-01)", "Appendix D.3 (friction cone, four-dimensional friction cone)", "Appendix I.1 (Cone::project)"]
---
## Background

At a contact the impulse γ = (γₙ, γₜ) lies in the friction cone 𝒦_μ = {‖γₜ‖ ≤ μ·γₙ}, whose dual is 𝒦_{1/μ}. The
cone is a linear image of the second-order cone. With a spin component the admissible set is the
four-dimensional cone 𝒦 = {‖(λ_t1, λ_t2, λ_s/r)‖ ≤ μ·λ_n}, with dual {μ·‖(v_t1, v_t2, r·ω_n)‖ ≤ v_n}, for an
effective patch radius r (2R/3 for uniform pressure on a disc of radius R, 3πR/16 for a Hertzian distribution).
The contact solvers and the control programs project onto these cones in every iteration; a projection that is
slightly wrong yields impulses outside the cone or energy that does not exist.

Statement (T-07). For the three-dimensional and the four-dimensional friction cone 𝒦 with dual cone 𝒦*, the
projection P onto 𝒦 satisfies, for every x: P(x) ∈ 𝒦; P(x) − x ∈ 𝒦*; and ⟨P(x), P(x) − x⟩ = 0. These three
conditions characterize the Euclidean projection onto a closed convex cone (Moreau's decomposition).

Tier: Frontier. Route: per-case polynomial identities and inequalities. The matching proof obligation of the
contact chapter: the four-dimensional cone is a linear image of the second-order cone, and projection and Moreau
identities hold; certified by polynomial identity, with G0 property tests. Rejected alternative: a pyramid (box)
approximation of the cone, |γₜ₁| ≤ μ·γₙ and |γₜ₂| ≤ μ·γₙ, which admits (μ·γₙ, μ·γₙ) of norm √2·μ·γₙ, 41% of traction
that does not exist along the diagonals.

## Scope

In: statement document, registry row, the per-case certificates on the cone projection kernels (three- and
four-dimensional), specimens and mutants. Out: the `Cone` trait and the kernels (the algebra and contact beads);
the solver; the positive-semidefinite Hessian (CARD-T-08); the G0 property tests in floating point.

## Technical approach

1. The projection has three cases, selected by guards; each case is one branch-free piece, so each gets its own
   certificate under its guard. For 𝒦_μ in the coordinates x = (n, t) with s = ‖t‖ (a square-root witness with
   the side relation s² = t·t, s ≥ 0):
   - Case A (x ∈ 𝒦: s ≤ μ·n): P(x) = x. The three conditions are trivial (P − x = 0).
   - Case B (x in the polar cone: μ·s ≤ −n): P(x) = 0. Then P − x = −x, and −x ∈ 𝒦* = 𝒦_{1/μ} is the guard
     itself.
   - Case C (otherwise): P(x) = ((n + μ·s)/(1 + μ²))·(1, μ·t/s). Identities to certify, with s² = t·t:
     ‖P_t‖² = μ²·P_n² (P lies on the cone's boundary); the normal part of P − x equals μ·(s − μ·n)/(1 + μ²) and its
     tangential part has squared norm (s − μ·n)²/(1 + μ²)², so ‖(P − x)_t‖ = (1/μ)·(P − x)_n (P − x lies on the
     boundary of the dual cone); and ⟨P, P − x⟩ = 0. The inequalities P_n ≥ 0 and (P − x)_n ≥ 0 follow from the
     guard (n + μ·s > 0 and s − μ·n > 0) and are certified as exact rational inequalities over the guard region,
     as sign conditions of the same polynomials that define the guard.
2. Four-dimensional cone: the linear change of variables λ_s ↦ λ_s/r maps it to a second-order cone in ℝ⁴ with a
   three-dimensional tangential part; the projection is taken in the scaled coordinates, and the same three
   cases apply with t = (λ_t1, λ_t2, λ_s/r). The certificate states the identities in the scaled coordinates and
   the scaling explicitly (a typed field, so r cannot be dropped).
3. Bundle kind: `poly-identity` for the equalities (modulo s² = t·t), with the guard and the witness relation as
   side conditions. Checker: FrankenSymPy's proof kernel; `frob-verify` offline.
4. Class and limits: `Certified` per case on the kernel at its content hash. No Lean route. Kernels whose
   selects cannot be split are `Tested` by the G0 property tests. No floating-point claim.
5. Relies on it: constructions C-06 and C-37; the cone-program solver; the contact step; proof obligation
   PO-TRAC-01.
6. Artifacts: `docs/math/cards/T-07.md`; registry row; certificates per case and per cone; specimens and mutants.

## Acceptance criteria

- [ ] The pilot N12-22 (arm C) is on record before work starts.
- [ ] `frob-verify` accepts the case A, B and C bundles for both cones and rejects: the pyramid projection
      offered for case C (P_t clipped per component); a case C formula with 1 − μ² in the denominator; a missing
      witness relation s² = t·t; a four-dimensional certificate without the factor r.
- [ ] Registry row `checked`, class `Certified`, with certificate hashes and checker identity; or `fell-back`
      with class `Tested` if the pilot or KILL-LEMMA says so.
- [ ] The three cases' guards cover all of space and overlap only on their boundaries (checked as a partition of
      the (n, s) half-plane).

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t07.rs`: `t07_case_c_identities_expand`; `t07_orthogonality_exact`
(μ = ½, x = (n, t) = (1, (3, 4)): s = 5, P = ((1 + 5/2)/(5/4))·(1, (3/10, 4/10)) = (14/5, (21/25, 28/25));
‖P_t‖ = 7/5 = μ·P_n; P − x = (9/5, (−54/25, −72/25)); ⟨P, P − x⟩ = 126/25 − 1134/625 − 2016/625 = 0);
`t07_dual_membership_exact` (‖(P − x)_t‖ = 18/5 = (1/μ)·(P − x)_n = 2·9/5); `t07_case_guards_partition`;
`t07_pyramid_projection_rejects`; `t07_four_dimensional_scaling_required`;
`t07_spin_coupling_value` (at v_t = r·ω_n the tangential friction magnitude is μ·λ_n/√2, tested numerically as a
related known answer, not part of the certificate).

### End-to-end script and logging

`scripts/e2e/lemma_coverage.sh` (bead N12-36) covers the cone kernels and logs each case's status.

### Known answers and negative controls

The exact case above. The box approximation (√2·μ·γₙ on the diagonals) is the planted wrong law. Pure spin: the
limiting moment is μ·λ_n·r, which for uniform pressure is (2/3)·μ·N·R. Determinism: exact.

## Considerations

- The ellipsoidal four-dimensional cone approximates the exact sliding–spinning law of a finite patch, and r is a
  card parameter, `Estimated` unless calibrated; the certificate is about the projection onto the cone as
  declared, not about the cone's physical fidelity.
- Budget HYPOTHESIS for case C with the witness (ideal membership); see the pilot.

## References

§12.3; C-06, C-37; PO-TRAC-01; T-08; BET-11.
