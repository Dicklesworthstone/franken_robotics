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
related: ["GB-29", "GB-30", "N12-13", "N12-37", "N12-48", "N12-52", "N12x-05", "N24b-06", "N24b-15", "N24c-08", "N24c-09", "N28c-23"]
gates: []
plan_refs: ["§12.3 (T-26)", "§10 C-15", "§24.12", "§24.13", "§24.15 (PO-ARENA-02)", "§0.7 (a frozen decision rule is enumerated before sampling)", "Appendix D.8 (error budget for retries)"]
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
the study types of `frob-arena` (BD-124) and the verb `eval.preanalyze` with its enumerator and its conformance
reference (N24b-06) are `related`, not dependencies; neither depends on this card. What the card does depend on is
in `deps`: the card registry and its specification. It closes on its statement document, its registry row, its
own exact enumeration of the reference rule, the level-series check, the registered Lean statement and every
acceptance line that its own fixtures decide. Two outputs are observed on the live verb by the end-to-end script
of N24b-21 (`scripts/e2e/arena_study_preanalysis.sh`), which cites this card: the refusal of a study that needs an
unreachable verdict (step `refuse_unreachable`), and the count-for-count agreement of the production enumerator
with N24b-06's independent re-implementation (step `independent_enumeration`). Neither is a condition of this
card's close.

## Technical approach

1. Part (a), per declared rule and n (class: exact computation on the host, verification tier G0). A fixed-n
   paired rule maps an outcome count (for paired binary outcomes: the numbers of pairs preferring B, preferring
   A and tied, summing to n) to a verdict in a declared set (for example promote, reject, tie, inconclusive).
   The enumeration evaluates the rule on every outcome count in exact rationals (binomial tails as exact
   fractions; no floating-point p-values) and returns the set of verdicts that occur and, per verdict, the number
   of outcome counts that produce it. The pre-analysis attached to a study is this table with its digest. A
   study that needs a verdict outside the reachable set is refused at admission.
2. The stated set is checked, not trusted: the conformance crate holds an independent re-implementation of the
   rule's arithmetic (N24b-06's `preanalysis_reference`), and the two enumerations must agree count for count.
   Where a pre-analysis travels in a bundle, it is part of a `sample-coverage` receipt and the auditor recomputes
   the table for small n within its size limits (larger n: the digest is checked and the claim is `Tested`).
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
   answer). (b) is the finite statement of proof obligation PO-ARENA-02 (§24.15), which raises no new card and is
   listed on this card's row as a consumer (N12-52); the infinite sum is not claimed.
6. Artifacts: `docs/math/cards/T-26.md`; registry row; the enumeration fixtures; `lean/Frob/Statements/T26.lean`.

## Acceptance criteria

- [ ] For the reference rule (a two-sided 97.5% exact Clopper–Pearson rule at n = 64 with a ±5 point indifference
      band) the card's exact enumeration shows that no outcome can return a tie: perfect concordance gives the
      interval [−0.0662, +0.0662], wider than the band; the tie is first reachable at n = 86. The refusal at
      admission of a study that needs that verdict is observed in N24b-21's step `refuse_unreachable`.
- [ ] The card's enumeration and its digest are stable across two runs; agreement count for count with the
      independent re-implementation of N24b-06 on the reference rules is observed in N24b-21's step
      `independent_enumeration`, which logs both table digests.
- [ ] The level series is exact: for α = 1/20, n = 1, 7, 10, 500 give 1/40, 7/160, 1/22, 25/501, each below 1/20.
- [ ] Negative controls: a stated reachable set that includes an unreachable verdict is rejected; a series with
      α_i = α/i² in place of α/[i·(i + 1)] fails the identity; the Lean statement with α·n/(n + 2) on the right side
      fails to compile (this control is registered here and run by BD-381 when it seals the proof).
- [ ] Registry row at this card's close (M1): enumeration route `checked`; Lean route `hypothesis`, with the
      registered statement `lean/Frob/Statements/T26.lean` (equality and the `< α` corollary for α > 0) present.
      BD-381, which depends on this card, seals the proof at M5 and its receipt moves the Lean route to
      `checked`, class `Proved`.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t26.rs`: `level_series_exact_small_n`; `level_series_strictly_below_alpha`
(n up to 10⁴); `telescoping_identity` (1/[i·(i + 1)] = 1/i − 1/(i + 1) for i up to 10³);
`reachability_cp_n64_no_tie`; `tie_first_reachable_n86`; `reachability_table_digest_stable`;
`unreachable_verdict_claim_rejected`; `retry_cost_table` (relative to a single test at α = 0.05, the sample size
grows with ln(1/α_i): about 1.23 times the units at request 1, 2.57 at request 10, 4.08 at request 100 and 5.15
at request 500; checked as ln(1/α_i)/ln(1/α) to two decimals).

### End-to-end script and logging

`scripts/e2e/arena_study_preanalysis.sh` (bead N24b-21), steps `enumerate`, `consequences`,
`refuse_unreachable` and `independent_enumeration`, runs `frob eval preanalyze` on the reference rule and logs
the pre-analysis digest and the reachable set (asserting the concordance interval ±0.0662 at n = 64, that the tie
is first reachable at 86, that a study declaring it at n = 64 is refused, and that the production enumerator and
N24b-06's independent re-implementation agree count for count); `scripts/e2e/lemma_lean_capsule.sh` (bead
N12-37) seals the Lean half at M5. For this card the lines of those four steps (STD-E2E helper) carry `ts`,
`script`, `bead` (N24b-21's key, since a line carries its owning bead; this card's evidence is found by the step
names), `step`, `event`, the exact command line, exit code, wall time, the rule's identity and n, the reachable
verdicts with their outcome counts, the table digest from both implementations (which must agree), and on a
mismatch the first differing outcome count and the path to reproduce. The T-26 stage of the capsule script writes
per-card lines with `bead` "CARD-T-26", the capsule hash and the checker identities. The level-series values are
checked as exact rationals by the card's unit tests above, not by these scripts.

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
