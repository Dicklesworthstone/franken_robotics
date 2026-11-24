---
key: "CARD-T-08"
title: "Theorem card T-08: the fast-tier contact objective has a positive-semidefinite Hessian"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["BD-381", "N12-06", "N12-23", "N12-24", "N12-37", "N12-51", "SPEC-C-06"]
needs: []
related: ["N12-16", "N12-43", "N15a-05", "N15a-08", "N16b-29", "N18a-16", "N18b-28", "N18b-31"]
gates: []
plan_refs: ["§12.3 (T-08)", "§10 C-06", "§16.15 (PO-KIN-01)", "§18.15 (INV-TRAC-01)", "§15.6", "Appendix D.3 (fast-tier contact problem)"]
---
## Background

The fast (lockstep) tiers solve contact as a convex, compliant problem in generalized velocities:
v = argmin ½·(v − v*)ᵀ·A·(v − v*) + Σ_c ℓ_c(J_c·v), where A is positive definite, J_c is the Jacobian of contact c,
ℓ_c is a convex potential and minus its gradient is the contact impulse γ_c. Because the problem is strictly
convex it has one solution, and its gradient norm bounds the distance to it, so solver error is a recorded number
per step. All of that depends on convexity being true by construction, for every admitted law and every
parameter row.

Statement (T-08). The Hessian of the fast-tier contact objective, H(v) = A + Σ_c J_cᵀ·∇²ℓ_c(J_c·v)·J_c, is positive
semidefinite wherever it exists; with A positive definite it is positive definite. Hypotheses: A ≻ 0; each ℓ_c
convex (its Hessian positive semidefinite where defined). Joint-level terms keep the property: a regularized
stiction term contributes h·τ_b/ε inside its kink and zero outside, so the objective stays a sum of
positive-semidefinite terms.

Route: structural (a sum of positive-semidefinite terms, each of the form JᵀHJ with H ⪰ 0), with the general
lemma proved in Lean. Tier: Frontier (it needs the Lean lane). The matching obligations elsewhere: the lockstep
objective stays strictly convex for every admitted parameter row and lagged friction coefficient (certified per
law, tested per parameter row); every law on a lockstep tier is a convex potential, or is the hard-contact law
and reports its residual. Rejected alternative: projected Gauss–Seidel with softness knobs that have no physical
meaning and no convexity statement.

## Scope

In: the Lean lemma and its capsule; the card's statement document and registry row; the check, at M5, that every
law admitted on a lockstep tier by then has a per-law convexity receipt under this card's instance route.
Out: the per-law convexity certificates themselves, their generator and the admission guard, which are N12-51 at
M2 (the default contact law and the joint rows are admitted on lockstep tiers at M2, and PO-KIN-01 needs them
certified per law then; steps 2 and 3 below state that work and it is carried out there); the laws and the solver
(contact and actuator beads); the per-parameter-row tests; the projection identities (CARD-T-07).

Order of work. The beads that implement or use the theorem (the contact laws, the joint rows, the batched solve)
are `related`, never dependencies of this card. Its dependencies are the card registry, the specification of the
contact construction, the Lean lane with its pilot and statement binding, the capsule script N12-37 whose
per-card stage seals this card's lemma, and N12-51, whose receipts the card's row cites. A bead at M2 that needs
a law's convexity certificate depends on N12-51, not on this card.

## Technical approach

1. General lemma (class `Proved` when sealed; stated in elementary form to keep the closure inside algebra and
   order, without analysis modules):

```lean
-- DESIGN: statements fix intent. PSD is stated as ∀ x, 0 ≤ x ⬝ᵥ M.mulVec x over a linearly ordered field.
theorem psd_congr {m n : ℕ} {K : Type} [LinearOrderedField K]
    (H : Matrix (Fin m) (Fin m) K) (J : Matrix (Fin m) (Fin n) K)
    (hH : ∀ y, 0 ≤ y ⬝ᵥ H.mulVec y) : ∀ x, 0 ≤ x ⬝ᵥ (Jᵀ * H * J).mulVec x := …
theorem psd_add_sum … -- a positive-definite A plus finitely many PSD terms is positive definite
```

   Proof: x ⬝ᵥ (JᵀHJ)·x = (J·x) ⬝ᵥ H·(J·x) ≥ 0; sums of nonnegative terms are nonnegative; adding a positive-definite
   A gives strict positivity for x ≠ 0.
2. Per-law instance certificates (class `Certified`): for each admitted potential ℓ_c, a certificate that its
   Hessian block is positive semidefinite on each smooth piece of its domain. For a polynomial or rational piece
   this is a `sos-gram` or `rational-ldlt` certificate in the law's parameters (stiffness, dissipation time,
   friction coefficient over their declared intervals) or, where the Hessian is a nonnegative scalar times a
   fixed PSD matrix, a sign certificate (`interval-box`) for the scalar. Pieces are separated by the law's own
   guards, one certificate per piece.
3. Structural check in the kernel generator: the objective is assembled only through a combinator that adds terms
   of the form JᵀHJ with a certified H; a law without a convexity certificate cannot be registered for a lockstep
   tier.
4. Class and limits: the composition (lemma plus per-law certificates) supports "Hessian positive semidefinite by
   construction" at class `Certified` for the model, and `Proved` for the general lemma. It says nothing about
   floating-point factorization (the two-sided scaling and refinement tests cover that) and nothing about the
   hard-contact law, which is a complementarity problem and not one convex program.
5. Relies on it: construction C-06; the batched Cholesky solve; INV-TRAC-01; PO-KIN-01.
6. Artifacts: the registered statement `lean/Frob/Statements/T08.lean` (written first, emitted and bound under
   N12-24's rule, so the proof is `theorem T08_proof : Frob.Statements.T08.Statement`); `lean/Frob/Cards/T08.lean`
   and its capsule `target/lean/T08.flncap`; `docs/math/cards/T-08.md`; registry row; per-law certificates beside
   the law definitions (N12-51).

## Acceptance criteria

- [ ] `psd_congr` and the sum lemma are compiled by stock Lean, re-checked by `fln`, bound to their registered
      statements and sealed; no `sorry`, no axiom outside the whitelist, no `native_decide`.
- [ ] Each law admitted on a lockstep tier has a convexity certificate that `frob-verify` accepts; a planted
      non-convex law (a potential with a negative-curvature piece) is rejected and cannot be registered. (The
      certificates and the guard are delivered by N12-51 at M2; this card re-reads the receipts for every law
      admitted by M5 and fails if one is missing or bound to a stale definition hash.)
- [ ] Negative controls: the lemma with H's hypothesis removed fails to compile; a per-law certificate with one
      Gram entry changed is rejected; a weakened statement (PSD only for diagonal H) fails the binding check.
- [ ] Registry row shows both routes with their classes, hashes and checker identities.

## Test plan

### Unit tests

`lean/tests/cards/t08/`: `psd_congr_seals`; `psd_sum_seals`; `closure_avoids_blocked_modules`;
`psd_congr_without_hypothesis_fails` (the statement with `hH` deleted does not compile: H = −I, J = I is a
counterexample); `weakened_statement_binding_fails` (a capsule proving PSD only for diagonal H fails the
statement binding of N12-24).
`crates/frob-lemma/tests/cards/t08.rs`: `law_certificate_accepts` (a quadratic penalty ℓ(y) = ½·k·y² with k > 0:
Hessian k, certified by sign); `nonconvex_law_rejected` (ℓ(y) = −½·k·y² on any piece);
`stiction_term_psd` (Hessian h·τ_b/ε ≥ 0 inside the kink, 0 outside);
`assembled_hessian_small_exact` (A = diag(2, 3), one contact with J = (1, 1), H = 4: A + JᵀHJ = [[6, 4], [4, 7]],
LDLᵀ pivots 6 and 13/3, both positive); `gram_entry_changed_rejects` (one Gram entry of a per-law `sos-gram`
certificate changed by 1/7: `reject`); `stale_law_receipt_fails` (a law whose definition hash changed after its
N12-51 receipt was issued: the M5 re-read fails with the law named).

### End-to-end script and logging

`scripts/e2e/lemma_lean_capsule.sh --card T-08` (the per-card stage of bead N12-37, a dependency) seals the
lemma, bundles it and verifies it from the empty environment; this card adds to that stage, for T-08 only, the
re-read of the per-law receipts: `frob prove list --json`, then `frob prove verify <law bundle> --json` for each
law admitted on a lockstep tier, asserting `accept` and a definition hash equal to the law's current one. (The
coverage script N12-36 counts derived kernels, not contact laws, so it does not carry these lines.) The stage
writes NDJSON lines (STD-E2E helper) with `ts`, `script`,
`bead` ("CARD-T-08"), `step` (`T-08/lemma/<stage>` or `T-08/law/<law>/<piece>`), `event`, the exact command line,
exit code, wall time, peak memory, the capsule hash or certificate hash with the checker identity, the law's
definition hash, the expected and observed verdict, and on a failure the first failing rule (or `fln` diagnostic)
and the path to reproduce; the `summary` counts laws admitted on lockstep tiers and laws with a current receipt.

### Known answers and negative controls

The exact small case above (pivots 6 and 13/3). The planted non-convex law is the negative control; so is a
pyramid-cone potential, which the model-form check catches elsewhere. Determinism: exact.

## Considerations

- Convexity is the model's property; creep of a stuck contact and energy stored in compliance are named artifacts
  of the convex model, measured against the referee in the fidelity ledger.
- OPEN-LEMMA-1: the matrix lemma's closure is measured before it is counted.
- If the Lean lane is unavailable, the card stays at its structural check and per-law certificates (`Certified`).

## References

§12.3; C-06; PO-KIN-01; INV-TRAC-01; T-07; ADR-LEMMA-02.
