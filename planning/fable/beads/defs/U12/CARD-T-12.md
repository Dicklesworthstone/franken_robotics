---
key: "CARD-T-12"
title: "Theorem card T-12: monotone co-design on finite chains has a least fixed point and Kleene iteration reaches it"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["BD-381", "BD-052", "N12-06", "SPEC-C-12"]
needs: []
related: ["BD-103", "BD-360", "N12-37", "CARD-T-15", "CARD-T-29"]
gates: []
plan_refs: ["§12.3 (T-12)", "§10 C-12", "§12.1 (Order.FixedPoints re-checks)", "Appendix D.5 (Kleene iteration)", "Appendix E GB-03"]
---
## Background

A robot is a composition of components over typed ports, and one of the functors that gives a design meaning is
the budget functor: it maps components to monotone maps between posets of resources and functionality. A design
with feedback (heavier actuators need more torque, which needs heavier actuators) is then a fixed-point problem:
find the least resource vector x with x = f(x) for a monotone f (monotone co-design). Kleene iteration answers
"does the design close?" before any simulation: starting from the least element ⊥, iterate x_{k+1} = f(x_k); the
sequence converges to the least fixed point, giving either the minimal mass and power or a proof of
infeasibility when the iteration leaves the catalogue.

Statement (T-12). Monotone co-design on finite chains has a least fixed point and Kleene iteration reaches it:
for a monotone map f on a finite partial order with a least element ⊥ (in particular a finite product of finite
chains, such as discretized mass, torque and power levels), the sequence ⊥, f(⊥), f²(⊥), … is non-decreasing,
becomes stationary after finitely many steps, and its stationary value is the least fixed point of f.

Route: Lean (order theory; `Order.FixedPoints` re-checks in FrankenLean). Tier: Frontier, because it uses the
Lean lane; the mathematics is standard. This card is one of the first three sealed capsules. Rejected
alternative: a sizing spreadsheet iterated by hand, with no statement that the result is the least solution.

## Scope

In: the registered statement, the Lean proof and its capsule (written within BD-381's first batch), the registry
row, the binding of the co-design closure analysis to the card. Out: the poset and Kleene implementation (the
algebra crate bead); the co-design closure analysis and the co-design synthesis lane; the continuous (ω-complete)
case, which is not needed and not claimed.

## Technical approach

```lean
-- DESIGN: statements fix intent, not signatures.
theorem kleene_reaches_lfp {A : Type} [PartialOrder A] [OrderBot A] [Fintype A] (f : A →o A) :
    ∃ n, f^[n] ⊥ = f^[n + 1] ⊥ ∧ ∀ x, f x = x → f^[n] ⊥ ≤ x := …
theorem kleene_chain_monotone {A : Type} [PartialOrder A] [OrderBot A] (f : A →o A) (k : ℕ) :
    f^[k] ⊥ ≤ f^[k + 1] ⊥ := …
```

1. Proof route: ⊥ ≤ f(⊥); by monotonicity and induction f^[k] ⊥ ≤ f^[k+1] ⊥; in a finite order a non-decreasing
   sequence cannot increase strictly more than |A| − 1 times, so it is stationary at some n; a stationary value is
   a fixed point; for any fixed point x, ⊥ ≤ x and induction give f^[k] ⊥ ≤ x for all k. The closure stays inside
   order theory (`Mathlib.Order.FixedPoints`, `Mathlib.Order.Hom.Basic`, finiteness), all accepted modules.
2. Pipeline: compiled by stock Lean, re-checked by `fln`, bound to the registered `Statement`, sealed; the capsule
   travels in a `lean-capsule` bundle.
3. Class claimed: `Proved` for the general statement once sealed. The statement is about finite orders; it does
   not claim convergence in a bounded number of steps other than |A| − 1, nor anything for non-monotone maps:
   where a change of gearing trades a larger static moment for a blocked swing or a new contact mode, the map is
   not monotone and the alternatives stay in separate cells.
4. Instance side (`Tested`, in the algebra crate): the implementation's iteration on the co-design fixture is
   compared with the closed form of the worked card (Kleene iteration matches the closed form), and the iteration
   count is bounded by the chain heights.
5. Relies on it: construction C-12; the co-design closure analysis (does the design close, with minimal mass and
   power or a proof of infeasibility); the co-design fixed-point lane of mission synthesis.
6. Artifacts: `lean/Frob/Cards/T12.lean`; `lean/Frob/Statements/T12.lean`; the capsule; `docs/math/cards/T-12.md`;
   registry row.

## Acceptance criteria

- [ ] The theorem is compiled by stock Lean, re-checked by `fln`, bound and sealed; the capsule verifies from an
      empty environment; `frob-verify` accepts the `lean-capsule` bundle with the capsule file.
- [ ] Negative controls: the statement without monotonicity (f an arbitrary function) fails to compile; a
      weakened statement (existence of some fixed point, not the least) fails the binding check; a capsule byte
      flip is rejected.
- [ ] Registry row: Lean route `checked`, class `Proved`, capsule hash, both checker identities.
- [ ] The closure analysis cites the card only for monotone fragments and reports non-monotone designs as
      outside it.

## Test plan

### Unit tests

`lean/tests/cards/t12/`: `kleene_reaches_lfp_seals`; `kleene_chain_monotone_seals`; `non_monotone_variant_fails`;
`closure_within_order_theory`.
`crates/frob-algebra/tests/kleene.rs` (instance side): `iteration_reaches_lfp_on_product_of_chains` (two chains
of heights 5 and 7, f monotone by construction, result equals the brute-force least fixed point);
`non_monotone_map_detected` (a map that decreases somewhere is refused by the monotonicity check);
`iteration_count_bounded_by_heights`.

### End-to-end script and logging

`scripts/e2e/lemma_lean_capsule.sh` (bead N12-37) seals this card as one of the three and logs its closure size,
peak memory and capsule hash.

### Known answers and negative controls

Counterexample showing monotonicity is needed: on the chain {0, 1} the map f(0) = 1, f(1) = 0 has no fixed point
and the iteration oscillates. Negative controls as listed. Determinism: capsule hash reproducible.

## Considerations

- The budget functor applies in the monotone fragment only; a port contract must also carry time correlation
  (two fingers on one unit-current bus have identical local summaries, and a rule that adds their peak demands
  rejects a task that a two-tick schedule completes), which is outside this theorem.
- OPEN-LEMMA-1: closure size measured before the card is counted.

## References

§12.3; C-12; GATE-LEMMA-02 (as one of the three sealed cards); A-10.
