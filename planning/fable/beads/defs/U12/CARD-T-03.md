---
key: "CARD-T-03"
title: "Theorem card T-03: the quaternion update preserves the norm (‖q·c‖² = ‖q‖²·‖c‖²)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "frob-lemma", "solid", "test"]
parent: "WP-05"
deps: ["BD-027", "N12-06", "SPEC-C-05"]
needs: []
related: ["N12-33", "N12-39"]
gates: ["GATE-LEMMA-00"]
plan_refs: ["§12.3 (T-03)", "§12.6", "§11.1 (rotation storage)", "§10 C-05"]
---
## Background

Rotations are stored as unit quaternions (component order and sign rule are FrankenSim's, confirmed at M0; the
double cover is resolved by a deterministic sign rule). A rotation update multiplies the stored quaternion q by
an increment c. If quaternion multiplication did not preserve the norm, every step would move the state off the
unit sphere by an amount depending on the data; because it does, the state leaves the sphere only by round-off,
which one normalization per step removes.

Statement (T-03). For all quaternions q = (q₀, q₁, q₂, q₃) and c = (c₀, c₁, c₂, c₃) with real (or rational)
components and Hamilton product q·c, ‖q·c‖² = ‖q‖²·‖c‖², where ‖q‖² = q₀² + q₁² + q₂² + q₃². In particular a unit
q times a unit c is a unit quaternion, and with a Cayley increment c ∝ (1, h·ω/2) the update's norm is
‖q‖²·(1 + h²·‖ω‖²/4) before normalization.

It is the four-square identity: a polynomial identity of degree four in eight variables. Tier: Solid for the
instance route; Frontier for the Lean route through `Mathlib.Algebra.Quaternion`, an accepted module (the
analytic quaternion module is blocked and is not needed). Rejected alternative: treating norm preservation as a
tested tolerance only.

## Scope

In: statement document, registry row, instance certificates on the M0 kernels: the quaternion update on
`K-cay-step` (the kernel that forms q′ = q ⊗ c with c = (1, a/2), a = h·ω; BD-019) and the normalization clause on
`K-normalize`; specimen and mutants. Out: certificate machinery (BD-021, BD-027); the Lean theorem (N12-39);
the sign rule of the double cover (a convention test of the algebra crate).

## Technical approach

1. Instance route (class `Certified`): the identity
   (q₀c₀ − q₁c₁ − q₂c₂ − q₃c₃)² + (q₀c₁ + q₁c₀ + q₂c₃ − q₃c₂)² + (q₀c₂ − q₁c₃ + q₂c₀ + q₃c₁)² +
   (q₀c₃ + q₁c₂ − q₂c₁ + q₃c₀)² = (q₀² + q₁² + q₂² + q₃²)·(c₀² + c₁² + c₂² + c₃²),
   extracted from the straight-line program of `K-cay-step` by symbolic execution and checked by normal-form
   equality (bundle kind `poly-identity`); at the program's c = (1, a/2) it reads ‖q′‖² = ‖q‖²·d with
   d = 1 + a·a/4, an identity in seven variables. The normalization clause is certified on `K-normalize`, whose
   input is the q′ of the update: the square root is replaced by a witness t with the side relation t² = ‖q′‖²,
   and the certified statement is that the output components squared sum to ‖q′‖²/t² = 1.
2. Checker: FrankenSymPy's proof kernel at build time; `frob-verify` offline.
3. General route (class `Proved`, Frontier): the norm-squared map of a quaternion algebra over a commutative
   ring is multiplicative; carried by N12-39.
4. Class and limits: `Certified` for the kernel at its content hash. No floating-point claim: the residual
   ‖q‖ − 1 after a step is measured by tests with a dimensionless tolerance.
5. Relies on it: construction C-05; every kernel that composes rotations as quaternions; the M0 marrow gate,
   which cites the certificates of T-01, T-03 and T-04.
6. Artifacts: `docs/math/cards/T-03.md`; registry row; the T-03 entries of `K-cay-step.cert` and
   `K-normalize.cert` (BD-021) and their bundles; specimen
   `poly-identity/accept-T-03.frobcert` with mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the two T-03 bundles for the M0 kernels (`K-cay-step`, `K-normalize`); GATE-LEMMA-00
      (TARGET, M0) records T-03 as certified and replaying offline.
- [ ] Negative controls rejected: one changed coefficient (a sign flipped in one product term, which is the
      Hamilton product with a wrong handedness in one component); the witness side relation removed; a statement
      claiming ‖q·c‖² = ‖q‖² + ‖c‖².
- [ ] Registry row `checked`, class `Certified`, with certificate hash and checker identity.
- [ ] The statement document fixes the component order and product convention used by the kernel.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t03.rs`: `t03_four_square_identity_expands`; `t03_specimen_accepts`;
`t03_flipped_sign_rejects`; `t03_witness_relation_missing_rejects` (FROB-REF-829); `t03_sum_statement_rejects`
(the statement ‖q·c‖² = ‖q‖² + ‖c‖²: `reject`; at the spot check below 150 ≠ 35); `t03_rational_spot_check`
(q = (1, 2, 3, 4), c = (2, 0, 1, 0): ‖q‖² = 30, ‖c‖² = 5, q·c = (−1, 0, 7, 10) with norm squared 150);
`t03_unit_times_unit` (two rational unit quaternions, for example (3/5, 4/5, 0, 0) and (0, 0, 5/13, 12/13), give a
product of norm exactly 1).

### End-to-end script and logging

`scripts/e2e/lemma_kernel_cert.sh` (bead N12-33) certifies, replays offline and logs verdicts and hashes. Its
NDJSON lines for this card (STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-03"), `step`
(`T-03/<specimen-or-mutant>`), `event`, the exact command line, exit code, wall time, the envelope's `evidence`
block (claim Proof, class `Certified`, `KernelId` of `K-cay-step` or `K-normalize`, certificate hash, checker
identity), the
expected and observed verdict, and on a mismatch the first failing rule and the bundle path to reproduce.

### Known answers and negative controls

KA-220 (T-01, T-03 and T-04 certified and replaying offline; one changed coefficient gives `reject`). The spot
checks are exact. Negative controls are the mutants. Determinism: certificate bytes identical across
regenerations and machines.

## Considerations

- CX-112: a certificate over ℚ is not a statement about floating-point output; the normalization step exists
  because of round-off.
- A wrong product convention (the opposite handedness) still preserves the norm, so this card does not detect a
  convention error; the convention is fixed against the sibling's façade by a separate M0 obligation.

## References

§12.3, §12.6; C-05; GATE-LEMMA-00; KA-220; GATE-MARROW-01.
