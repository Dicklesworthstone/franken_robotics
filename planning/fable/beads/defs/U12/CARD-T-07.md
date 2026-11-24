---
key: "CARD-T-07"
title: "Theorem card T-07: the friction-cone projection satisfies the Moreau identities (3- and 4-dimensional cones)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["N12-06", "N12-20", "N12-22", "SPEC-C-06", "SPEC-C-37"]
needs: []
related: ["CARD-T-08", "N12-16", "N12-21", "N12-43", "N12-51", "N18b-31"]
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
four-dimensional), specimens and mutants. This card is the only issuer of the `poly-identity` certificates of
T-07, and with them of the certification of proof obligation PO-TRAC-01 of the contact chapter: the bead that
builds the four-dimensional cone (BD-154, M2) carries that obligation at class `Tested` and issues no bundle.
Out: the `Cone` trait and the kernels (the algebra and contact beads: BD-050, BD-153, BD-154); the solver; the
positive-semidefinite Hessian (CARD-T-08); the G0 property tests in floating point and the exact-rational case
checks at M2 (BD-154, with the law tests of SPEC-C-37).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the four-dimensional friction cone of `frob-contact` (BD-154), like the `Cone` trait and the cone kernels is
`related`, not a dependency (a card that waited for its implementer would close a loop, because the implementer's
own acceptance runs the card's law tests). What the card does depend on is in `deps`: the card registry, its
specification, its pilot where the route is Frontier, and Lemma's own proof machinery. It closes on its statement
document, its registry row, its specimens and mutants, and every acceptance line that its own fixtures decide. An
acceptance line that can only be observed on live output (certificates bound to the content hashes of the cone
kernels as built) is run inside the implementing bead, with this card's law tests, as part of that bead's
acceptance; the receipt produced there is what moves the registry row for that output. For this card the
implementing beads land three milestones earlier (BD-050 at M1, BD-153 and BD-154 at M2) and are not reopened:
the certificates on their kernels are issued at M5 by the instance-certificate pipeline (N12-20), which is a
dependency, and the acceptance of that run moves the rows of T-07 and of PO-TRAC-01 from `Tested` to
`Certified`.

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
   the scaling explicitly (a typed field, so r cannot be dropped). This is the projection the kernels of BD-154
   compute for the isotropic cone: the Euclidean projection in spin-scaled coordinates, not the conjugate of
   the second-order projection by diag(μ, 1, 1, 1/r) (which is a projection in another metric and fails
   ⟨P(x), P(x) − x⟩ = 0 whenever μ ≠ 1: at μ = ½ and x = (0, 1, 0, 0) it returns (1, ½, 0, 0) with inner
   product ¾; it is a registered mutant there and a rejected specimen here). For the anisotropic cone of
   BD-154 the scaling field is the full diagonal T = diag(1, 1/μ₁, 1/μ₂, 1/(μ_s·r)) and the cone in scaled
   coordinates is the case μ = 1 of the same certificates.
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
      witness relation s² = t·t; a four-dimensional certificate without the factor r; the conjugated projection
      ((n·μ + s)/(2·μ), ((n·μ + s)/2)·t/s) offered for case C at μ ≠ 1.
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
`t07_one_minus_mu_squared_rejects` (case C with 1 − μ² in the denominator: at the exact point above P_n would be
14/3 and ⟨P, P − x⟩ ≠ 0); `t07_missing_witness_relation_rejects` (FROB-REF-829);
`t07_conjugated_projection_rejects` (μ = ½, x = (0, 1, 0, 0): the conjugated map returns (1, ½, 0, 0) with
⟨P, P − x⟩ = ¾ ≠ 0, while the Euclidean projection is (2/5, 1/5, 0, 0) with inner product 0);
`t07_spin_coupling_value` (at v_t = r·ω_n the tangential friction magnitude is μ·λ_n/√2, tested numerically as a
related known answer, not part of the certificate).

### End-to-end script and logging

`scripts/e2e/lemma_coverage.sh` (bead N12-36) covers the cone kernels and logs each case's status. For this card
its NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-07"), `step` (`T-07/<cone>/<case>`),
`event`, the exact command line, exit code, wall time, peak memory, the envelope's `evidence` block (claim Proof,
class `Certified` or `Tested`, `KernelId`, certificate hash, checker identity), the expected and observed verdict
of each specimen and mutant, and on a mismatch the first failing rule and the bundle path to reproduce.

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
