---
key: "NEG-04"
title: "discrepancy E7-1 resolved: the tenth qualification request needs 76,959 clean episodes for 10⁻⁴, not 76,963"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "solid", "test", "card"]
parent: "WP-55"
deps: ["NEG-03", "GB-32", "BD-276"]
needs: []
related: ["N26a-30", "GB-29"]
gates: []
plan_refs: ["Appendix E.7 row 1", "§26.9", "Appendix M.38 rows 7 and 8", "Appendix M KA-664"]
---
## Background

The rare-failure certification rule says how many nominal episodes with no failure certify a failure rate
below p⋆ at confidence 1 − δ. The table of clean-episode counts uses the exact rule n = ⌈ln δ/ln(1 − p⋆)⌉,
which gives 29,956 for p⋆ = 10⁻⁴ at δ = 0.05. The sentence after that table first printed 76,963 for the tenth
qualification request (δ = 0.05/110), which is ⌈ln(1/δ)/p⋆⌉, the approximation. Two rules had been mixed in one
paragraph. The specification now prints 76,959. This matters beyond four episodes: a planner that uses the
approximation in one place and the exact rule in another will disagree with itself about whether a run was
long enough, and a certificate checker that recomputes the count would reject its own producer.

## Scope

In: the single rule for the count, its implementation point, the regression test, and the replacement of a
negative control that could not fail. Out: the certification lane itself (BD-276); the card (GB-32).

## Technical approach

1. **One function.** `frob-nemesis` exposes exactly one function for the count,
   `clean_episodes_required(p_star, delta) -> u64`, and every caller (the planner of `nemesis.bound`, the
   qualification lane when it sizes a request, the report) uses it. It returns the least n with
   (1 − p⋆)ⁿ ≤ δ, decided in exact rational arithmetic at the candidate and at candidate − 1; floating-point
   logarithms only propose the candidate.
2. **Values (exact).** δ = 0.05: 29,956 (10⁻⁴), 299,572 (10⁻⁵), 2,995,731 (10⁻⁶). δ = 0.05/110 (request 10):
   76,959 for 10⁻⁴. The approximation gives 29,958, 299,574, 2,995,733 and 76,963: always too many here, by 2
   and by 4.
3. **The control that could not fail.** A registered negative control for the count was "an episode count
   from ln(1/δ)/p⋆ rounded down". At p⋆ = 10⁻⁴ that is 29,957, which is above the exact 29,956: the planted
   defect errs on the safe side and cannot make the row fail. It is replaced by an undercounting control:
   ⌊ln(1/δ)/p⋆·(1 − p⋆/2)⌋ = 29,955, for which (1 − p⋆)ⁿ = 0.050003 > δ.
4. **Register and scan.** Item E7-1 of the discrepancy register: stale pattern `76,963` in the scope of
   rare-failure and qualification text; pinned by `gb_32_reference_counts` and the test below.

## Acceptance criteria

- [ ] One function computes the count; a search of the crate finds no other use of ln(1/δ)/p⋆ for sizing.
- [ ] The function returns 29,956, 299,572, 2,995,731 and 76,959 for the four inputs.
- [ ] The undercounting control is registered and is caught; the old control is marked withdrawn with the
      reason.
- [ ] The stale-value scan finds no `76,963` outside labelled controls.

## Test plan

### Unit tests

- `clean_episodes_exact_rule`: the four values; for each, (1 − p⋆)ⁿ ≤ δ < (1 − p⋆)ⁿ⁻¹ in exact rationals.
- `clean_episodes_grid`: for p⋆ ∈ {10⁻², 10⁻³, 10⁻⁴} and δ ∈ {0.1, 0.05, 0.01, 0.05/110} the result satisfies
  the same two inequalities.
- `clean_episodes_single_source`: a source scan asserts no second implementation.
- `undercount_control_caught`: 29,955 fails the inequality; `old_control_cannot_fail`: 29,957 passes it
  (recorded as the reason for withdrawal).

### End-to-end script and logging

Covered by `scripts/e2e/worked_cards.sh` (NEG-02) through GB-32's surface row (`frob nemesis bound
--plan-only` inside a qualification campaign at request 10 returns 76,959) and by the stale-value step; log
lines carry δ, p⋆, the count, and the item id E7-1.

### Known answers and negative controls

KA-664 (29,956; 299,572; 2,995,731; the tenth request 76,959; tier G0). Controls: the approximation (76,963)
and the undercount (29,955). Determinism: exact integer arithmetic, D0.

## Considerations

- For p⋆ this small the approximation is conservative, so the earlier figure was never unsafe; the defect is
  inconsistency between producer and checker, not optimism.
- The unreconciled-values list of the known-answer suite carries the same correction and the remark about the
  control; this bead implements both.

## References

Appendix E.7 row 1; §26.9; Appendix M.38 rows 7 and 8; KA-664.
