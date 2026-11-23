---
key: "CARD-T-20"
title: "Theorem card T-20: curvature sign of the connection (second-order holonomy of a coordinate rectangle)"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "frob-lemma", "frob-analyze", "frontier", "test"]
parent: "EPIC-M3"
deps: ["BD-027", "N12-06", "SPEC-C-08"]
needs: ["§0.7: the paper-scale budget pilot arm for the truncated-series identity (N12-22, arm E, which is scheduled at M4; at M3 this card runs that arm itself on its one fixture before any other work)"]
related: ["N12-22", "BD-425", "CARD-T-16", "BD-243"]
gates: []
plan_refs: ["§12.3 (T-20)", "§11.6", "§10 C-08", "Appendix D.1 (curvature of the connection, reconstruction)"]
---
## Background

Locomotion of a principally kinematic system is holonomy. Configuration space is a principal bundle over shape
space; a connection 𝔸 splits motion into shape change and group motion by the reconstruction rule ġ = g·ξ with
ξ = −𝔸(s)·ṡ (plus 𝕀(s)⁻¹·p when momentum is not zero), where 𝔸 = 𝔸ᵢ ds^i takes values in the Lie algebra. For a
closed loop in shape space (one gait cycle) the net displacement is the right-ordered path exponential of
−∮𝔸·ds, which to leading order is minus the integral of the curvature over the enclosed surface. Curvature
integrated over reachable shape space ranks two morphologies before any simulation. The sign and the bracket
term of the curvature formula are easy to get wrong, and a wrong sign reverses every predicted displacement.

Statement (T-20). Curvature sign: for a polynomial connection the second-order coefficient of the holonomy of a
coordinate rectangle is −(∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ]). That is, with (D𝔸)ᵢⱼ = ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ], a small rectangle with
sides a and b in the (i, j) plane, traversed from i towards j, has holonomy g = exp(−(D𝔸)ᵢⱼ·a·b + higher order).

Why the bracket term has that sign: the first-order part is Stokes' theorem applied to −∮𝔸·ds; the bracket part
is the second-order term of the ordered exponential. For a rectangle the four legs contribute −a·𝔸₁, −b·𝔸₂, +a·𝔸₁,
+b·𝔸₂ in that order, and half the sum of the brackets of earlier with later legs is +a·b·[𝔸₁, 𝔸₂].

Route: truncated-series identity in exact rationals. Tier: Frontier. Rejected alternative: the formula
∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ + [𝔸ᵢ, 𝔸ⱼ], which is wrong by twice the answer on the SO(3) fixture.

## Scope

In: statement document, registry row, the certificate for the connection-curvature analysis's formula (on the
fixture and on a general polynomial connection of bounded degree), specimen and mutants. Out: computing
connections for real robots and the curvature analysis itself (the static-analysis bead); large loops (which keep
the ordered product and do not use the area formula); minimum-perturbation coordinates; dynamic walking, for
which curvature is a diagnostic and not a predictor.

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the connection-curvature analysis A-09 (BD-243) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(the certificate on the analysis's formula as implemented) is run inside the implementing bead, with this card's
law tests, as part of that bead's acceptance; the receipt produced there is what moves the registry row for that
output.

## Technical approach

1. Setup: the connection's components 𝔸₁(s), 𝔸₂(s) are matrices whose entries are polynomials in the shape
   coordinates with rational coefficients (matrix Lie algebra: 𝔰𝔬(3) or 𝔰𝔢(2), 𝔰𝔢(3)), expanded about a base point.
2. Compute the holonomy of the rectangle (0,0) → (a,0) → (a,b) → (0,b) → (0,0) as the ordered product of four
   leg solutions of ġ = −g·𝔸(s)·ṡ, each as a power series in (a, b) truncated at total degree 2, with matrix
   coefficients over ℚ (Picard iteration to second order per leg; products truncated).
3. Statement certified: the coefficient of a·b in the logarithm of that product (equivalently, in the product
   itself, since the first-order terms cancel around a closed loop) equals −(∂₁𝔸₂ − ∂₂𝔸₁ − [𝔸₁, 𝔸₂]) at the base
   point, entry by entry, as a polynomial identity in the connection's Taylor coefficients treated as symbols
   (bundle kind `poly-identity`). One certificate covers every polynomial connection of the given matrix size.
4. Checker: FrankenSymPy's proof kernel; `frob-verify` offline. The analysis kernel that evaluates curvature is
   bound to the certificate by its content hash.
5. Class and limits: `Certified` for the leading-order coefficient. The statement is local and second order; it
   says nothing about loops that are not small, and a connection belongs to one contact stratum (a loop that
   crosses slip or lift-off cannot borrow its old connection).
6. Relies on it: construction C-08 (locomotion is holonomy); the connection-curvature analysis; the wrong-sign
   negative control of that construction.
7. Artifacts: `docs/math/cards/T-20.md`; registry row; certificate; specimen `poly-identity/accept-T-20.frobcert`
   with mutants.

## Acceptance criteria

- [ ] Before other work, the budget arm for this identity has run on the fixture and is on record (operations,
      time, memory, verdict), with its control failing.
- [ ] `frob-verify` accepts the certificate and rejects: the plus-bracket formula; a formula without the bracket;
      the traversal reversed without changing the sign; one changed coefficient.
- [ ] Fixture: the constant connection 𝔸₁ = e_x, 𝔸₂ = e_y on SO(3) (bracket = cross product): the curl vanishes,
      (D𝔸)₁₂ = −[e_x, e_y] = −e_z, and a small loop of area da rotates the body by +da about z.
- [ ] Registry row `checked`, class `Certified`, with hash and checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t20.rs`: `rectangle_legs_bch_second_order` (legs X = (−a·A₁, −b·A₂, +a·A₁, +b·A₂):
ΣX = 0 and ½·Σ_{i<j}[Xᵢ, Xⱼ] = a·b·[A₁, A₂]); `so3_constant_connection_fixture`
(exp(−a·e_x)·exp(−b·e_y)·exp(a·e_x)·exp(b·e_y) agrees with exp(a·b·e_z) through second order, in truncated exact
rational series); `plus_bracket_predicts_minus_da` (the wrong formula gives −da: wrong by twice the answer);
`non_constant_connection_curl_term` (𝔸₂ = s₁·e_z, 𝔸₁ = 0: holonomy coefficient −∂₁𝔸₂ = −e_z);
`traversal_reversal_flips_sign`; `specimen_accepts`.

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) includes the specimen and mutants; the analysis's own
script logs the certificate hash bound to the curvature kernel.

### Known answers and negative controls

The SO(3) fixture (+da about z; the plus-sign formula is wrong by twice the answer). Related known answer of the
construction: Elroy's beanie, two coaxial bodies with inertias I₁, I₂ and zero momentum, where a relative
rotation Δψ rotates body 1 by −I₂/(I₁ + I₂)·Δψ. CX-102. Negative controls: the mutants. Determinism: exact.

## Considerations

- Chart control belongs to the analysis: the same physical gait reconstructed in another body frame predicts the
  same motion; curvature computed in poor coordinates misleads.
- The identity's symbolic size grows with the matrix dimension; 3×3 and 4×4 homogeneous matrices are within
  budget on paper, which the budget arm confirms or refutes.
- The geometric-phase epic for locomotion (M6) reuses this card unchanged.

## References

§12.3, §11.6; C-08; CX-102; A-09.
