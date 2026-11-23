---
key: "CARD-T-26"
title: "Theorem card T-26: reachable verdicts of a fixed-n rule, and the level series Σ α/[i·(i + 1)] < α"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-lemma", "frob-arena", "solid", "test"]
parent: "WP-41"
deps: ["N12-06", "SPEC-C-15"]
needs: []
related: ["BD-381", "BD-259", "N12-13", "BD-383", "N12-37", "BD-124"]
gates: []
plan_refs: ["§12.3 (T-26)", "§10 C-15", "§24.12", "§24.13", "§0.7 (a frozen decision rule is enumerated before sampling)", "Appendix D.8 (error budget for retries)"]
---
## Background

Two study types exist and are never mixed inside one study. An anytime-valid study uses an e-process and can
stop at any time. A fixed-n study uses an exact test at a sample size chosen in advance, and ships a
pre-analysis listing which verdicts are reachable at that n. The pre-analysis is a hygiene rule learned the hard
way: a frozen decision rule is enumerated before sampling, because a rule can be unable to return a verdict the
study needs (one pilot rule with 64 pairs could return three of its four verdicts; the fourth was first
reachable at 86 pairs). Separately, the blinded qualification lane lets an agent retry: request i of a campaign
with total error budget α receives the level α_i = α/[i·(i + 1)], and the campaign-wide chance of any false
promotion stays below α for any number of retries.

Statement (T-26). (a) For a declared fixed-n rule and sample size, the set of verdicts reachable over all
outcome counts is as stated. (b) For every n, Σ_{i ≤ n} α/[i·(i + 1)] = α·n/(n + 1) < α.

Part (b) holds because 1/[i·(i + 1)] = 1/i − 1/(i + 1), so the sum telescopes; the infinite sum equals α but is an
analytic statement and is not needed: every finite prefix is strictly below α. Route: exhaustive enumeration in
exact rationals for (a) and for instances of (b) (Solid); Lean by induction on n for (b) (Frontier; one of the
first three sealed capsules). Rejected alternatives: discovering after sampling that a needed verdict was
unreachable; spending a constant level per retry (with independent attempts at 5% each, fourteen null
submissions give a 51% chance of at least one false promotion).

## Scope

In: statement document, registry row, the exact enumeration test for reachable verdicts bound to this card, the
exact-rational check of the level series, the registered Lean statement. Out: the study runner and
`eval.preanalyze` (the study-type bead); the qualification lane; the receipt arithmetic checker (N12-13); the
Lean proof (written in BD-381's first batch).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the study types of `frob-arena` (BD-124) is `related`, not a dependency (a card that waited for its implementer
would close a loop, because the implementer's own acceptance runs the card's law tests). What the card does depend
on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and Lemma's own
proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and every
acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output (the
enumeration of a declared study rule at admission, and the refusal of a study that needs an unreachable verdict)
is run inside the implementing bead, with this card's law tests, as part of that bead's acceptance; the receipt
produced there is what moves the registry row for that output.

## Technical approach

1. Part (a), per declared rule and n (class: exact computation on the host, verification tier G0). A fixed-n
   paired rule maps an outcome count (for paired binary outcomes: the numbers of pairs preferring B, preferring
   A and tied, summing to n) to a verdict in a declared set (for example promote, reject, tie, inconclusive).
   The enumeration evaluates the rule on every outcome count in exact rationals (binomial tails as exact
   fractions; no floating-point p-values) and returns the set of verdicts that occur and, per verdict, the number
   of outcome counts that produce it. The pre-analysis attached to a study is this table with its digest. A
   study that needs a verdict outside the reachable set is refused at admission.
2. The stated set is checked, not trusted: the conformance crate holds an independent re-implementation of the
   rule's arithmetic, and the two enumerations must agree count for count. Where a pre-analysis travels in a
   bundle, it is part of a `sample-coverage` receipt and the auditor recomputes the table for small n within its
   size limits (larger n: the digest is checked and the claim is `Tested`).
3. Part (b), instances: for the campaign's α and every index up to the declared maximum, Σ α_i is computed as an
   exact rational and compared with α·n/(n + 1); the auditor's `sample-coverage` rule performs the same
   recomputation on receipts.
4. Part (b), general (class `Proved` when sealed):
   `theorem level_series (α : ℚ) (n : ℕ) : (Finset.Icc 1 n).sum (fun i => α / (i * (i + 1))) = α * n / (n + 1)`
   and its corollary `< α` for α > 0, by induction on n.
5. Class and limits: (a) is an exact computation about a rule, `Certified` when re-derived by the auditor in a
   bundle and `Tested` otherwise; it says which verdicts can occur, not how likely they are. (b) bounds the
   total level; the error-rate guarantee of the lane also needs three operational conditions that no checker
   decides (the request is fixed before the draw, the clusters are fresh, the level is spent whatever the
   answer).
6. Artifacts: `docs/math/cards/T-26.md`; registry row; the enumeration fixtures; `lean/Frob/Statements/T26.lean`.

## Acceptance criteria

- [ ] For the reference rule (a two-sided 97.5% exact Clopper–Pearson rule at n = 64 with a ±5 point indifference
      band) the enumeration shows that no outcome can return a tie: perfect concordance gives the interval
      [−0.0662, +0.0662], wider than the band; a study that needs that verdict is refused at admission.
- [ ] The independent re-implementation agrees with the enumeration count for count on the reference rules.
- [ ] The level series is exact: for α = 1/20, n = 1, 7, 10, 500 give 1/40, 7/160, 1/22, 25/501, each below 1/20.
- [ ] Negative controls: a stated reachable set that includes an unreachable verdict is rejected; a series with
      α_i = α/i² in place of α/[i·(i + 1)] fails the identity; the Lean statement with α·n/(n + 2) on the right side
      fails to compile.
- [ ] Registry row: enumeration route `checked`; Lean route `checked`, class `Proved`, after BD-381 seals it.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t26.rs`: `level_series_exact_small_n`; `level_series_strictly_below_alpha`
(n up to 10⁴); `telescoping_identity` (1/[i·(i + 1)] = 1/i − 1/(i + 1) for i up to 10³);
`reachability_cp_n64_no_tie`; `reachability_table_digest_stable`; `second_implementation_agrees`;
`unreachable_verdict_claim_rejected`; `retry_cost_table` (relative to a single test at α = 0.05, the sample size
grows with ln(1/α_i): about 1.23 times the units at request 1, 2.57 at request 10, 4.08 at request 100 and 5.15
at request 500; checked as ln(1/α_i)/ln(1/α) to two decimals).

### End-to-end script and logging

The study scripts log the pre-analysis digest and the reachable set through `frob study plan`;
`scripts/e2e/lemma_lean_capsule.sh` (bead N12-37) seals the Lean half.

### Known answers and negative controls

KA-226 (the level series; the reachable-verdict set by exhaustive enumeration in exact rationals). Table known
answer of the one-bit e-process, related: with p₀ = 0.05, p₁ = 0.30 and α = 0.01 promotion needs 16 clean pairs,
or 22 pairs with one exceedance. Negative controls as listed. Determinism: exact.

## Considerations

- The qualification lane's guarantee is statistical and is tested by null calibration (2,000 no-signal campaigns
  with 500 adaptive retries); this card contributes only the arithmetic.
- Enumeration cost grows as n² for paired ternary outcomes; n in the hundreds is immediate.

## References

§12.3; C-15; §24.12, §24.13; KA-226; PO-ARENA-02; GATE-LEMMA-02 (through BD-381).
