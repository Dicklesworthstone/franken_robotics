---
key: "NEG-04"
title: "discrepancy E7-1 resolved: the tenth qualification request needs 76,959 clean episodes for 10⁻⁴, not 76,963"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "solid", "test", "card", "review-notes-open"]
parent: "WP-55"
deps: ["BD-276", "GB-32", "N26a-21", "N26a-26", "N38b-56", "NEG-03", "STD-E2E", "VERBS-nemesis"]
needs: []
related: []
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

In (this bead, M3, `frob-nemesis` and the discrepancy register): item E7-1; the proof that every product caller
uses the one function for the count; the regression test through the verb; the registration of the stale
pattern; and the replacement of a negative control that could not fail.

Out: the function itself, `rare::zero_failure::required` (N26a-21, a dependency, the Nemesis chapter's bead
for the zero-failure rule: its method, its exactness and its own tests); the same correction as record REC-07
of the reconciliation register, with the truth of the known-answer row, its derivation file
`docs/derivations/zero-failure-count.md` and the reference computation in the conformance crate (N38b-56, at
M1, a dependency); the reconciliation record of the control that errs on the safe side (N38b-57); the
certification lane itself (BD-276); the card (GB-32).

## Technical approach

1. **One function.** `frob-nemesis` exposes exactly one function for the count, `rare::zero_failure::required`
   of N26a-21, `required(p_star: Rational, level: Level) -> u64`, whose level is `Level::delta(δ)` or, in a
   qualification campaign, `Level::request(α, i)` with δ = α/[i·(i + 1)] computed exactly, and every caller (the
   planner of `nemesis.bound`, the qualification lane when it sizes a request, the report) uses it. It returns
   the least n with (1 − p⋆)ⁿ ≤ δ, certified at the candidate and at candidate − 1; floating-point logarithms
   only propose the candidate. This bead adds no function: its first draft named one of its own,
   `clean_episodes_required(p_star, delta)`, and N38b-56 named a third, `zero_failure_episodes`; the chapter's
   function governs, N38b-56's is the conformance crate's reference, and this bead's name is withdrawn. What
   this bead builds is the check that the function is alone: a source scan of the product crates for any
   other computation of the count, and for any use of ln(1/δ)/p⋆ to size a run.
2. **Values (exact).** δ = 0.05: 29,956 (10⁻⁴), 299,572 (10⁻⁵), 2,995,731 (10⁻⁶). δ = 0.05/110 (request 10):
   76,959 for 10⁻⁴. The approximation gives 29,958, 299,574, 2,995,733 and 76,963: always too many here, by 2
   and by 4.
3. **The control that could not fail.** A registered negative control for the count was "an episode count
   from ln(1/δ)/p⋆ rounded down". At p⋆ = 10⁻⁴ that is 29,957, which is above the exact 29,956: the planted
   defect errs on the safe side and cannot make the row fail. It is replaced by an undercounting control:
   ⌊ln(1/δ)/p⋆·(1 − p⋆/2)⌋ = 29,955, for which (1 − p⋆)ⁿ = 0.0500041 > δ (the value N38b-56 prints for
   29,955; a first draft here printed 0.050003).
4. **Register and scan.** Item E7-1 of the discrepancy register: stale pattern `76,963` in the scope of
   rare-failure and qualification text, linked `same_as` REC-07 with `value = "76959"`; pinned by
   `gb_32_reference_counts` and the test below. The places where `76,963` may stand are one list, shared word
   for word with N38b-56's acceptance line: labelled controls (marker `stale-as-control: E7-1` or `REC-07`),
   item E7-1 of the discrepancy register, and the reconciliation table's record REC-07; beside them the scan
   engine's general exemptions (N38b-50: the derivation file a record names, here
   `docs/derivations/zero-failure-count.md`, and the registers' declared printouts) apply to both beads alike
   and are not a second list.

## Acceptance criteria

- [ ] One function computes the count in product code, N26a-21's; a search of the product crates finds no second computation and no other use of ln(1/δ)/p⋆ for sizing.
- [ ] The function returns 29,956, 299,572, 2,995,731 and 76,959 for the four inputs, and agrees with the conformance crate's reference (N38b-56) on them and on the grid of the test below.
- [ ] The undercounting control is registered and is caught; the old control is marked withdrawn with the
      reason.
- [ ] The stale-value scan finds no `76,963` outside labelled controls, this register item and the reconciliation table of record REC-07 (the same three exceptions as N38b-56's acceptance line, with the engine's general exemptions of N38b-50 applying to both); a planted `76,963` in a schema example fails the scan naming E7-1 and REC-07.

## Test plan

### Unit tests

- `clean_episodes_exact_rule`: the four values; for each, (1 − p⋆)ⁿ ≤ δ < (1 − p⋆)ⁿ⁻¹ in exact rationals.
- `clean_episodes_grid`: for p⋆ ∈ {10⁻², 10⁻³, 10⁻⁴} and δ ∈ {0.1, 0.05, 0.01, 0.05/110} the result satisfies
  the same two inequalities.
- `clean_episodes_single_source`: a source scan asserts no second implementation in a product crate; the conformance crate's reference is the one allowed exception and is not reachable from product code.
- `product_matches_reference`: the function and the reference of N38b-56 agree on the grid above.
- `undercount_control_caught`: 29,955 fails the inequality; `old_control_cannot_fail`: 29,957 passes it
  (recorded as the reason for withdrawal).

### End-to-end script and logging

`scripts/e2e/neg_e7_1_clean_count.sh [--seed S]` (shared end-to-end helper; N38b-56 names it as the place where
the count is checked through the verb). Conventions for every `frob nemesis bound` call, as in N26a-30's script:
its own key; `--finalist` (BD-276's recorded attestation: the script's `synthetic` candidate is a finalist of no
recorded campaign, and without the flag N26a-26's finalists-only rule refuses the call with
`rare-bound-not-finalist`, exit 3, before the count is reached); and a scratch configuration with
`nemesis.bound.policy = baseline` (N26a-26: the nominal share is the whole budget; under the adopted table
policy it is 80%, and step 2's budgets of 2,995 and 2,994 episodes would not decide at the count).
1. `frob nemesis bound <c> --task <t> --tier batch-cpu --target 1e-4 --delta 1/2200 --mode certify --finalist
   --estimate --key e71-1 --json`: `required` is 76,959 (the level of the tenth request) and no field of the
   envelope carries 76,963; then δ = 1/20 at 10⁻⁴, 10⁻⁵ and 10⁻⁶ (`--key e71-2` to `e71-4`): 29,956, 299,572 and
   2,995,731, each with the design N26a-22 would use.
2. The planner uses the exact count, not the approximation: at `--target 1e-3 --delta 1/20`, where the exact
   rule gives 2,995 and ⌈ln(1/δ)/p⋆⌉ gives 2,996, `--budget episodes=2995 --estimate --key e71-5` reports
   `required` 2,995 and the design `fixed-n` with n_declared 2,995, so the budget buys the claim; the same
   request submitted with `--budget episodes=2994 --key e71-6` and no `--estimate` (FROB-REF-760 is raised at
   admission, and an estimate-only call admits nothing) is refused with FROB-REF-760, exit 4, `required` 2,995
   against `observed` 2,994, the reachable bound attached and zero episodes run. A planner sized by the
   approximation would report 2,996 for the first and refuse a budget of 2,995.
3. `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g0_laws -- KA-664` (the four counts) and the same
   with `--features gauntlet-mutations`, where the undercount control (29,955) is reported caught.
4. The stale-value scan of NEG-03 over the tree (zero hits outside the three listed places), then over a scratch
   copy with `76,963` planted in a schema example (one hit naming E7-1 and REC-07).
The same counts also stand in `scripts/e2e/worked_cards.sh` (NEG-02) through GB-32's surface rows. Every NDJSON
line, through the shared helper: `ts`, `script`, `bead` (`NEG-04`), `step`, `event`, the exact command line, exit
code, wall time, δ, p⋆, the count and the design, and the item id E7-1; on failure the first count or refusal
that differs, with both values; final `summary` with assertions passed and failed and `gates: []`. Logs under
`target/e2e-logs/neg_e7_1_clean_count/`.

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
