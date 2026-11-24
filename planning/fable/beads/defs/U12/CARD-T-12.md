---
key: "CARD-T-12"
title: "Theorem card T-12: monotone co-design on finite chains has a least fixed point and Kleene iteration reaches it"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["BD-052", "N12-06", "N12-23", "SPEC-C-12", "STD-E2E"]
needs: []
related: ["BD-381", "CARD-T-15", "CARD-T-29", "GB-03", "N10a-11", "N12-37", "N12-45", "N31b-41"]
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

In: the statement document; the registered statement `lean/Frob/Statements/T12.lean` (`def Statement : Prop`, the
form the binding rule of N12-24 requires), which must elaborate in stock Lean with a closure inside order theory;
the negative-control variants (the non-monotone statement, the weakened statement); the registry row; the binding
of the co-design closure analysis to the card. Out: the Lean proof and its capsule, which BD-381 writes and seals
in its first batch of three cards and which depends on this card (sealing binds the proof to the registered
statement, so the statement must exist first); the poset and Kleene implementation (the algebra crate bead); the
co-design closure analysis and the co-design synthesis lane; the continuous (ω-complete) case, which is not
needed and not claimed.

Order of work. The poset and Kleene-iteration code of `frob-algebra` (BD-052) lands four milestones earlier, at
M1, and cannot wait on an M5 card, so no loop is possible: it is a dependency, and this card runs its instance-side
law tests and the check of the closure analysis's citations against that code itself. What the card further
depends on is the card registry, its specification and Lemma's Lean measurement (N12-23). It closes on its
statement document, its registered statement, its registry row, its instance tests and every acceptance line
that its own fixtures decide. The Lean route is the one part observed later: the capsule lines below are observed
when BD-381, which depends on this card, seals the proof against this card's registered statement and runs this
card's negative controls, and BD-381's receipt moves the row from `hypothesis` to `checked`.

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

- [ ] The registered statement `lean/Frob/Statements/T12.lean` elaborates under stock Lean at the pinned
      Lean and Mathlib versions, and its generated closure list contains no blocked module (checked by this card).
- [ ] The negative-control variants are written and registered for BD-381, which runs them when it seals the
      proof: the statement without monotonicity (f an arbitrary function) fails to compile; a weakened statement
      (existence of some fixed point, not the least) fails the binding check; a capsule byte flip is rejected.
      This card checks that each variant elaborates or fails to elaborate as stated under stock Lean.
- [ ] Registry row at this card's close (M5): Lean route `hypothesis`, with the registered statement
      `lean/Frob/Statements/T12.lean` and its elaborated-type hash present; instance side `Tested` with this card's
      law-test receipt. BD-381, which depends on this card, seals the proof (compiled by stock Lean, re-checked by
      `fln`, bound, sealed, verified from an empty environment, its `lean-capsule` bundle accepted by
      `frob-verify` with the capsule file), and its receipt moves the Lean route to `checked`, class `Proved`, with
      capsule hash and both checker identities.
- [ ] The closure analysis cites the card only for monotone fragments and reports non-monotone designs as
      outside it.

## Test plan

### Unit tests

`lean/tests/cards/t12/`: `statement_elaborates` (the registered `Statement` elaborates under the pinned stock Lean
and Mathlib; its elaborated-type hash equals the one in the registry row); `closure_within_order_theory` (the
generated closure list of the statement names no blocked module); `variants_elaborate_as_stated` (the
non-monotone variant, with `f : A → A`, and the weakened variant, existence of some fixed point, each elaborate as
Props and are registered with the outcome BD-381 must observe: the proof fails to compile against the first and
fails the binding check against the second). Sealing the proof and running the variants against it are BD-381's
tests (`t12_kleene_seals` and its negative controls), not this card's.
`crates/frob-lemma/tests/cards/t12.rs`: `t12_registry_row` (at this card's close the row has the Lean route
`hypothesis` with the registered statement and its hash, and the instance side `Tested` with the law-test
receipt; a row edited to `checked` without a capsule receipt fails the registry check).
`crates/frob-algebra/tests/kleene.rs` (instance side): `iteration_reaches_lfp_on_product_of_chains` (two chains
of heights 5 and 7, f monotone by construction, result equals the brute-force least fixed point);
`non_monotone_map_detected` (a map that decreases somewhere is refused by the monotonicity check);
`iteration_count_bounded_by_heights`.

### End-to-end script and logging

This card's own script, `scripts/e2e/card_t12.sh` (header `bead=CARD-T-12 milestone=M5 labels=lemma,frob-lemma
seed=0 needs=cpu`; sources the STD-E2E helper, idempotent, writes only under `target/`), runs at the card's close:
`statement`, `lake env lean lean/Frob/Statements/T12.lean`, exit 0, with the closure list checked against the
blocked modules; `variants`, each negative-control variant elaborated with its registered expected outcome;
`instance`, `cargo test -p frob-algebra --test kleene`; `row`, `cargo test -p frob-lemma --test cards --
t12_registry_row`. Its NDJSON lines (`target/e2e-logs/card_t12/<utc>.ndjson`, transcript beside it) carry `ts`,
`script`, `bead` ("CARD-T-12"), `step` (`T-12/<stage>`), `event`, the exact command line, exit code, wall time,
peak memory in GiB, the closure's module count, the elaborated-type hash of the registered statement, the row's
route statuses, and on a failure the Lean diagnostic or failing test and the path to reproduce; the `summary` line
counts assertions passed and failed. `scripts/e2e/lemma_lean_capsule.sh` (bead N12-37, which comes after BD-381
and therefore after this card) later seals this card as one of the three and logs, under the same `bead` value,
its closure size, peak memory, capsule hash, both checker identities and, for each negative control, the refusal
and its message.

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
