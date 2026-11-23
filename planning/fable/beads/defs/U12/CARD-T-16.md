---
key: "CARD-T-16"
title: "Theorem card T-16: the convention card (ad* = adᵀ with no minus sign; the reflection parity map)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-lemma", "solid", "test"]
parent: "WP-05"
deps: ["BD-027", "N12-06", "CARD-T-04"]
needs: []
related: ["BD-010", "BD-102", "CARD-T-10", "CARD-T-05", "CARD-T-18", "BD-011"]
gates: []
plan_refs: ["§12.3 (T-16)", "§11.2", "§11.4", "§13.3", "§10 C-04", "§10 C-09", "Appendix D.1 (algebra adjoint and coadjoint, reflection parity)"]
---
## Background

Two conventions are easy to get wrong by a sign or a transpose, and both are fixed once for the whole system.

First, the algebra coadjoint. For twists ξ = (ω, v), η = (η_ω, η_v) (angular part first) the bracket is
ad_ξ·η = [ξ, η] = (ω × η_ω, ω × η_v + v × η_ω). The algebra coadjoint is the dual map, defined by
⟨ad*_ξ μ, η⟩ = ⟨μ, [ξ, η]⟩; in coordinates it is the transpose of ad_ξ with no minus sign,
ad*_ξ μ = ad_ξᵀ μ = (π × ω + ℓ × v, ℓ × ω) for a momentum μ = (π, ℓ), and with it the Euler–Poincaré equation of a
left-invariant system in body velocity reads μ̇ = ad*_ξ μ + w, that is π̇ = π × ω + ℓ × v + τ and ℓ̇ = ℓ × ω + f. The map
ξ ↦ −ad_ξᵀ, which some texts call the coadjoint action, is never written ad*; where needed it is written −ad_ξᵀ.

Second, reflection parity. Under an improper isometry x ↦ P·x + t with PᵀP = I and det P = −1, polar vectors map
by P and axial vectors by det(P)·P; twists and wrenches at the origin map by T(P) = diag(det(P)·P, P) applied to
(angular, linear).

Statement (T-16). (i) ⟨ad_ξᵀ μ, η⟩ = ⟨μ, [ξ, η]⟩ for all ξ, η, μ: a polynomial identity in eighteen variables.
(ii) The parity map T(P) of an improper isometry preserves the bracket, [T(P)·ξ, T(P)·η] = T(P)·[ξ, η], and the
pairing, ⟨T(P)·w, T(P)·ξ⟩ = ⟨w, ξ⟩ (the latter because det(P)² = 1).

Route: polynomial identities, with P written as a fixed reflection times cay(Ω), so that an arbitrary improper
orthogonal matrix is covered by a rational parameterization. Tier: Solid. Why a card: a sign error here survives
energy checks. With the wrong sign, a force-free body changes its world-frame velocity with no force, and a
free-body integrator still conserves kinetic energy either way. Rejected alternative: stating the convention in
prose and trusting each recursion's author.

## Scope

In: statement document, registry row, the certificates on the convention kernels (ad, ad*, the parity map),
specimen and mutants. Out: checking the conventions against the sibling's façade at M0 (the algebra crate's
obligation); the group-level statements (CARD-T-04, CARD-T-05); step equivariance (CARD-T-10).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the retractions and coadjoint convention of `frob-lie` (BD-011) is `related`, not a dependency (a card that waited
for its implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What
the card does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier,
and Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants,
and every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live
output (certificates bound to the content hashes of the convention kernels as built) is run inside the
implementing bead, with this card's law tests, as part of that bead's acceptance; the receipt produced there is
what moves the registry row for that output.

## Technical approach

1. Identity (i): expand ⟨ad_ξᵀ μ, η⟩ − ⟨μ, [ξ, η]⟩ = (π × ω + ℓ × v)·η_ω + (ℓ × ω)·η_v − π·(ω × η_ω) − ℓ·(ω × η_v + v × η_ω)
   in the eighteen coordinates of ξ, η, μ; it is the zero polynomial (each pair of terms cancels by the cyclic
   property of the scalar triple product). Certified by normal-form equality on the kernel that implements ad*.
2. Identity (ii): write P = P₀·cay(Ω) with P₀ = diag(1, −1, 1) (a fixed reflection) and Ω skew with three
   parameters; every improper orthogonal matrix with I + cay-parameter defined is of this form. The identities
   are polynomial in Ω's entries and the twists' coordinates after clearing the Cayley denominator; the key
   sub-identity is (P·a) × (P·b) = det(P)·P·(a × b).
3. Bundle kind `poly-identity`; checker FrankenSymPy's proof kernel at build time, `frob-verify` offline.
4. Class and limits: `Certified` on the kernels at their content hashes; no Lean route planned. The card fixes
   mathematics over ℚ; agreement of the implementation's conventions with the sibling library is a separate
   check.
5. Relies on it: the conventions chapter (adjoint, coadjoint and the Euler–Poincaré sign; reflection parity);
   construction C-04 (dynamics on a tree) and C-09 (the signed-permutation representations of a symmetry follow
   from the parity table); cards T-10 and T-18.
6. Artifacts: `docs/math/cards/T-16.md`; registry row; certificates; specimen `poly-identity/accept-T-16-*.frobcert`
   with mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts both T-16 bundles and rejects: ad* implemented as −ad_ξᵀ (identity (i) fails by a
      factor −1 on every term); the parity map with axial vectors mapped by P instead of det(P)·P (the bracket
      identity fails); one changed coefficient.
- [ ] Registry row `checked`, class `Certified`, with certificate hashes and checker identity.
- [ ] The free-body check and the mirror check below pass as exact fixtures.
- [ ] The statement document repeats both conventions as formulas and names the forbidden notation.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t16.rs`:
- `coadjoint_pairing_identity_expands` (eighteen variables, zero polynomial).
- `coadjoint_spot_check`: ξ = (e_x, e_y), η = (e_y, e_z), μ = (π, ℓ) = (e_z, e_x): [ξ, η] = (e_z, −e_y),
  ⟨μ, [ξ, η]⟩ = 1; ad*_ξ μ = (e_y + e_z, 0), ⟨ad*_ξ μ, η⟩ = 1.
- `free_body_sign_check`: inertia diag(1, 2, 3) kg·m², ω = (1, 1, 0) rad/s, π = (1, 2, 0): the correct law gives
  π̇ = π × ω = (0, 0, −1); the reversed sign gives (0, 0, +1); both give ω·π̇ = 0, so kinetic energy is conserved
  either way; the body-frame rate of spatial angular momentum π̇ + ω × π is (0, 0, 0) with the correct sign and
  (0, 0, 2) with the wrong one.
- `parity_preserves_pairing`; `parity_preserves_bracket`; `axial_as_polar_rejects`.
- `sagittal_mirror_table`: P = diag(1, −1, 1): a pitch axis e_y maps to itself, a roll axis e_x to −e_x, a yaw
  axis e_z to −e_z.

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) includes the two specimens and their mutants.

### Known answers and negative controls

CX-098 (the coadjoint in the Euler–Poincaré equation does not carry a minus sign: with the opposite sign
d(R·v)/dt = 2R·(ω × v), a free body changes its velocity with no force); CX-100 (angular velocity does not mirror
like any other vector); CX-116 (a free-body integrator that conserves kinetic energy can still have the wrong
coadjoint sign: the fixture above). Negative controls: the mutants. Determinism: exact.

## Considerations

- Each convention is guarded by at least three independent fixtures in the algebra crate (the mutation rule);
  this card is the certified one among them.
- The parameterization P = P₀·cay(Ω) misses reflections for which the Cayley parameter is undefined (a
  measure-zero set); those are covered by composing with a second fixed reflection, recorded in the certificate.

## References

§12.3, §11.2, §11.4, §13.3; C-04, C-09; CX-098, CX-100, CX-116; PO-MARROW-01.
