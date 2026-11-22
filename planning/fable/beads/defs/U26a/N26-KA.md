---
key: "N26-KA"
title: "Nemesis known-answer registry: KA-659 to KA-669 with truths, tolerances, tiers, controls and owner beads"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "frob-conformance", "solid", "registry", "test"]
parent: "WP-55"
deps: ["STD-TEST"]
needs: ["§26.10: the bead that implements the screening audit and carries KA-668 (seed bead BD-345 at M4)", "§26.11: the tests of coverage selection that carry KA-669 (seed bead BD-278)", "§26.5, §26.7: the tests of the oracle and the shrinker in their seed beads (BD-271, BD-274) that carry KA-660, KA-661 and KA-663 beside the component beads named below"]
related: ["BD-270", "BD-271", "BD-272", "BD-273", "BD-274", "BD-275", "BD-276", "BD-277", "BD-278", "BD-345", "BD-318", "N26a-04", "N26a-13", "N26a-14", "N26a-15", "N26a-18", "N26a-21", "N26a-24", "N26a-27", "N26-CX", "N26-REF"]
gates: []
plan_refs: ["Appendix M M.25", "§26.14", "§26.3", "§26.5", "§26.6", "§26.7", "§26.9", "§26.10", "§26.11", "§38.6", "§10 C-27", "§10 C-28"]
---
## Background

A known answer is a case with a truth a test can compare against: an analytic formula, a hand-derived value or a
planted truth, with the tolerance of the comparison, the tier and site on which it runs, and the negative control
that must make it fail. The known-answer registry of `frob-conformance` is the authority for all of them; numbers
are append-only (a row is never renumbered and a number is never reused). The chapter on adversarial worlds and
rare failures states a rule of its own: a value in the chapter that follows from a stated formula is a derived
value and a known answer; every other number carries its label. This bead enters the chapter's eleven rows,
KA-659 to KA-669, so that none can be lost between the mechanism beads, and names for each the beads whose tests
carry it. It also lists the seven known answers of the two constructions behind the chapter (rare failures;
minimal counterexample worlds), which the chapter's beads cite and the constructions' own registry owns.

Reading the columns. *Tolerance:* `exact` for integers, verdicts, labels and identities; "inside the interval"
where the truth must lie inside the estimator's own confidence set in a stated share of repetitions; a gate's
threshold where the row quotes one. *Tier:* G0 laws and self-tests; G1 known answers; G5 determinism. Sites:
`host` (exact or double-precision arithmetic in a test binary), `referee`, `lockstep` (the throughput tiers).
*Negative control:* the mutation or planted failure the row must catch.

Labels. Planted fixtures and their constants are DESIGN choices. Truths derived from formulas are DESIGN
arithmetic with a derivation file under `docs/derivations/`. Gate thresholds are TARGETs. Nothing here is a
measurement: no row has been run on FrankenRobotics code.

## Scope

In: the eleven registry records; one derivation file per derived truth; the map from row to owning bead; seeding
each stated negative control into the negative-control registry.
Out: the tests themselves (owned by the beads in the last column); the chapter's counterexample catalogue and
refusal registry (`N26-CX`, `N26-REF`, written by the other half of the chapter).

## Technical approach

**Registry records.** `crates/frob-conformance/registry/known_answers/nemesis.toml`, one record per row with the
fields of the table, the derivation path and the owning test's path.

| KA | Stated in | The case | The truth (formula or value) | Tolerance | Tier · site | Negative control | Carried by |
|---|---|---|---|---|---|---|---|
| KA-659 | Search space; known answers | Planted illegal edit | Refused with FROB-REF-360 and the violated bound named; 100 of 100 planted illegal edits refused (GATE-NEMESIS-01) | exact | G1 · host | A profile with the bound removed admits it | BD-270, N26a-04, N26a-05 |
| KA-660 | Solvability oracle | Planted impossible trap: a doorway narrower than the body (body 0.75 m, door 0.70 m clear) | `infeasible-for-candidate` with the bottleneck named; never a counterexample; 0 of 100 planted impossible traps counted (GATE-NEMESIS-01) | exact | G1 · host, referee | An oracle that checks a point robot calls it solvable | N26a-13, N26a-04, N26a-33; BD-271 |
| KA-661 | Solvability oracle | Planted solvable trap of known nominal probability (trigger box z₁ > 2.3263 and z₂ > 2.7478: 0.01 × 0.003 = 3 × 10⁻⁵) | Found, with a causal witness attached; at least 95 of 100 planted solvable traps receive a witness (GATE-NEMESIS-01) | the gate's threshold | G1 · referee | Card-uniform randomization at the same budget, as the baseline | N26a-14, N26a-04, N26a-33, BD-273, N26a-07, N26a-11; BD-271 |
| KA-662 | Artifact guard | Planted artifacts: box-cone traction (a box-linearized friction cone grants √2·μ along its diagonals: 49.05 N against 69.37 N for 10 kg at μ = 0.5); energy-pumping penetration recovery (1 kg, 1 mm removed in a 2 ms step: 0.5 m/s, 0.125 J per event; the compliant law at 10⁵ N/m stores 0.05 J) | Each labelled `artifact-candidate`, with the variant that removed it | exact (label) | G1 · lockstep, referee | A genuine planted failure survives every variant | BD-272, N26a-15, N26a-16, N26a-14 |
| KA-663 | Shrinking | Planted causes of size 1, 2 and 3 among 8 to 32 edits | The planted set exactly. Reference traces: n = 8, 16, 32 with planted {2}, {2, n − 1}, {1, n/2, n} take 4, 16, 26; 5, 23, 41; 6, 30, 56 distinct tests | exact | G1 · referee | The order-dependence example: with edits listed (a, c, d, e, b) and failure when {a, b} or {c, d, e} is present, delta debugging ends at {c, d, e} before the pair check (15 distinct tests) and at {a, b} after it (20 in all; at most 78 tests of one- and two-edit subsets for 12 edits) | N26a-18, N26a-17, N26a-34; BD-274 |
| KA-664 | Rare failures | Zero-failure rule at δ = 0.05 for p⋆ = 10⁻⁴, 10⁻⁵, 10⁻⁶: n = ⌈ln δ/ln(1 − p⋆)⌉ | 29,956, 299,572 and 2,995,731 nominal episodes with no failure; the tenth qualification request (δ = 0.05/110) needs 76,959 for 10⁻⁴ | exact (count) | G0 · host | An episode count from ln(1/δ)/p⋆ rounded down (29,957; 299,573; 2,995,732) | N26a-21, N26a-30 |
| KA-665 | Rare failures | Gaussian tail by defensive mixture q = ε·p + (1 − ε)·p_shifted, shift 4, ε = 0.1; w = 1/(ε + (1 − ε)·exp(4z − 8)) | P(z > 4) = 3.167 × 10⁻⁵ recovered inside the confidence sequence; every weight at most 1/ε = 10 (9.970 at z = 0; 1 at z = 2; 3.727 × 10⁻⁴ at z = 4) | inside the confidence sequence | G0 · host | A mixture without its nominal share on a mis-centred proposal (failure when \|z\| > 4, truth 6.334 × 10⁻⁵, proposal shifted to +4 only) | BD-276, N26a-24, N26a-25, N26a-30 |
| KA-666 | Rare failures | Splitting on a biased random walk: up-probability 0.4, start at level 1; P(reach b before 0 from a) = (1 − 1.5ᵃ)/(1 − 1.5ᵇ) | Probability of reaching level 20 before level 0 is 0.5/(1.5²⁰ − 1) = 1.504 × 10⁻⁴ (level-to-level factors for levels 2, 4, 8, 16, 20: 0.4, 0.3077, 0.1649, 0.03755, 0.1973) | inside the estimator's interval | G0 · host | A walk with no noise after the fork returns identical children and is refused | BD-277, N26a-28, N26a-29, N26a-31 |
| KA-667 | Rare failures | Good–Turing monitor on a synthetic catalogue with known mode probabilities (1/2, 1/4, 1/8, 1/16, 1/32, 1/32) | Unseen mass inside its interval; exact expectations at N = 20: E[N₁/N] = Σ pᵢ(1 − pᵢ)¹⁹ = 0.0635, realized unseen mass Σ pᵢ(1 − pᵢ)²⁰ = 0.0598 | inside the interval | G0 · host | A monitor that counts distinct modes instead of modes seen once (0.230 at N = 20) | N26a-27 |
| KA-668 | Screening audit | A deliberately biased screen with a planted winner; exact bound 1 − δ^(1/m) | The planted winner's removal is reported; no top-k design among 30 audited discards bounds the false-discard rate at 9.5% with 95% confidence, and 59 bound it at 5.0% (the approximation ln(1/δ)/m gives 10.0% at m = 30 and is not used) | exact (verdict); printed digits for the bounds | G0 · host | A screen with no bias passes | Expected from the screening-audit bead (BD-345) |
| KA-669 | Coverage selection | Worked instance: 40 modes of total weight 100; greedy gains 31, 22, 14, 9, 6, 4; the six largest remaining gains sum to 12 | 86 covered; OPT₆ ≤ 86 + 12 = 98; the selection is within 86/98 = 88% of the best six-scene battery, against a worst-case guarantee of 1 − 1/e = 63% | exact | G1 · host | A selection that double-counts a mode | Expected from the coverage bead (BD-278) |

**Known answers of the two constructions, cited by the chapter's beads** (registered with the constructions'
specification beads; listed here so a reader of one chapter finds them):

| KA | The case | The truth | Cited by |
|---|---|---|---|
| KA-138 | Zero-failure bound | p ≤ 1 − δ^(1/n) ≤ ln(1/δ)/n at confidence 1 − δ; about 3 × 10⁴ episodes for p⋆ = 10⁻⁴ at δ = 0.05 | N26a-21 |
| KA-139 | Defensive mixture with zero observed failures | Weights at most 1/ε; the bound is only p ≤ ln(1/δ)/(ε·n), never better than crude sampling on the nominal share (ε = 0.1, n = 29,956: about 10⁻³) | N26a-25, BD-276 |
| KA-140 | Splitting with L = 6, p₀ = 0.1, N = 10⁴ | cv² near L·(1 − p₀)/(p₀·N); cv = √(6 × 0.9/1,000) = 7.3% | BD-277, N26a-29 |
| KA-141 | A random walk with a closed-form hitting probability | Crude and bounded-weight bounds bracket the truth in at least 1 − δ of 10⁴ repetitions; fork control: children sharing the parent's streams are detected as duplicates | N26a-22, N26a-30, BD-277 |
| KA-142 | Eight edits, failure exactly when edits 3 and 6 are present | The search returns {3, 6} (16 distinct tests); non-monotone control: a grease patch (edit 1) fails and a rug over it (edit 2) masks it, so {1} fails and {1, 2} passes; the tool returns {1} and claims no distance | N26a-18, N26a-17 |
| KA-143 | Impossible-world control: a doorway narrower than the body | Rejected as a counterexample for lack of a solvability witness | N26a-13 |
| KA-144 | Replay control of a stored counterexample file | The stored world reproduces its state hash | N26a-19 |

**Derivations on file** (`docs/derivations/nemesis/`): the zero-failure count and its two approximations; the
weight bound and unbiasedness of the defensive mixture; the gambler's-ruin formula for the walk; the Good–Turing
expectations; the delta-debugging reference traces (generated by a 60-line reference script kept with the
derivation, independent of the product code); the box-cone and penetration-pump arithmetic.

**Negative-control seeding.** Each row's control is entered in the negative-control registry with the test that
must fail under it, so the program-wide check "every gate has a registered negative control that it catches"
finds one for every Nemesis gate.

## Acceptance criteria

- [ ] `nemesis.toml` holds eleven records, KA-659 to KA-669, each with truth, tolerance, tier, site, negative
      control, derivation path and owning test path; the registry loader accepts it and rejects a duplicate or
      renumbered row.
- [ ] Every derived value in the table is reproduced by a test from its formula: 29,956; 299,572; 2,995,731;
      76,959; 3.167 × 10⁻⁵; 6.334 × 10⁻⁵; 1.504 × 10⁻⁴; 0.0635 and 0.0598; 9.5% and 5.0%; 86, 98 and 88%; 78; the
      delta-debugging test counts.
- [ ] Each of the eleven negative controls is registered and its owning test fails under it (demonstrated once
      per control and recorded).
- [ ] The two rows owned outside this unit (KA-668, KA-669) have an owning test path filled in by their beads
      before the M3 and M4 exits; until then the registry marks them `pending owner`.
- [ ] The reference script for the delta-debugging traces is independent of `frob-nemesis` and its output is
      pinned.

## Test plan

### Unit tests
`crates/frob-conformance/tests/ka_nemesis.rs`:
- `registry_loads_eleven_rows`; `numbers_are_append_only` (a renumbered fixture file is rejected).
- `derived_values`: one assertion per value in the acceptance list, computed in exact or interval arithmetic
  (the zero-failure counts by certified enclosure at n and n − 1; the Gaussian tails from the complementary error
  function to 4 significant figures; the walk by the closed form; the Good–Turing sums; 1 − 0.05^(1/30) = 0.0950
  and 1 − 0.05^(1/59) = 0.0495; 12 + 66 = 78).
- `controls_registered`: every row has an entry in the negative-control registry.
- `owner_paths_exist`: every owning test path resolves to a test in the workspace (for rows not `pending`).

### End-to-end script and logging
None of its own. `scripts/e2e/nemesis_*.sh` name the KA numbers they exercise in their `assert` events (field
`ka`), so the conformance report can show, per row, the last script run that touched it and its outcome.

### Known answers and negative controls
The table is the content. Negative control of the registry itself: a row whose truth was regenerated from the
candidate implementation instead of derived (the derivation file is then missing; the loader refuses a row
without one).

## Considerations

- Three formulas for the zero-failure count appear in the plan: the exact ⌈ln δ/ln(1 − p⋆)⌉ (29,956), the
  construction's ⌈ln 20/p⋆⌉ (29,958) and the control's rounded-down value (29,957). The row pins the exact
  count; the others are sufficient and are not the known answer.
- Rows whose tolerance is "inside the interval" are statistical: their tests fix seeds and repetition counts so
  that a correct implementation passes with probability above 0.999, and say so in the test.
- The corpus of planted fixtures behind several rows is discovery data and is frozen before any mechanism is
  tuned on it.

## References

Plan Appendix M (M.25: KA-659 to KA-669; KA-138 to KA-144), §26.14, §38.6, §10 C-27 and C-28.
