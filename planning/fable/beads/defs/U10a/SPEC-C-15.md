---
key: "SPEC-C-15"
title: "frob-arena: C-15 specified — evidence as a test martingale, two study types, level spending; law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "arena", "frob-arena", "solid", "spine", "spec", "test"]
parent: "WP-41"
deps: ["BD-025", "N10a-01", "N24a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["SPEC-C-26", "SPEC-C-40"]
gates: ["GATE-ARENA-05", "GATE-ATEL-06", "GATE-QUAL-01"]
plan_refs: ["§10 C-15", "§16.8", "§24.10", "§24.12", "§24.13", "§28.6", "§35", "§12.3 T-26", "§38.7", "Appendix D.8", "Appendix E GB-06 GB-14", "Appendix I.9", "Appendix M KA-088 to KA-091", "Appendix N CX-040"]
---
## Background

Construction C-15 says what counts as evidence that one design is better than another. It replaces "mean
reward over five seeds". The comparison is run on paired episodes with common random numbers, and the evidence
is a betting process that is a nonnegative supermartingale under the null hypothesis, so that an agent may
look at the data at any time, stop when it likes, and still have a valid error rate.

Payoff: fewer episodes when the answer is clear; a valid error rate when the agent peeks; cancellation of the
losing arm the moment it loses; error control that survives an agent's adaptive retries.

Two study types are declared and never mixed inside one study. An **anytime-valid study** uses an e-process.
A **fixed-n study** uses an exact test at a sample size chosen in advance and ships a pre-analysis listing
which verdicts are reachable at that n. Time-uniform inference and a sealed fixed-n decision are different
procedures, and neither substitutes for the other.

Tier: Solid. Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission): `fs-race`
and `fs-eproc` exist; counter-based random streams keyed by logical identity exist in `fs-rand`.

Rejected alternatives: a fixed-level test evaluated at every look (its error rate inflates; the registered
peeking control shows 13/256 after eight looks); a fixed-n rule frozen
without enumerating its reachable verdicts (the recorded rule with 64 pairs could return three of its four
verdicts; the fourth was first reachable at 86 pairs).

## Scope

In: `docs/math/C-15.md`; the law suite `crates/frob-arena/tests/laws_c15.rs` with exact rational and integer
arithmetic; the promotion-table fixture; the fixed-n enumeration at n = 64, 86, 128 and 256; the level-spending
series; the null-calibration test; the register row; tolerances and controls for KA-088 to KA-091; the
statement of T-26.

Out: the study machinery and its verbs (BD-124, BD-262); the qualification lane (BD-259); registered
predictions and scoring rules as a product feature (BD-344); the promotion rule of the trust ladder (BD-263);
pairing by physical coordinates (construction C-40).

## Technical approach

**Anytime-valid comparison.** Comparing designs A and B on paired episodes with common random numbers yields
differences dᵢ (B minus A on the same world address). With a bound on |dᵢ| fixed before the first observation,
the normalized differences d̃ᵢ lie in [−1, 1]. A betting e-process is
Eₙ = ∏ᵢ (1 + λᵢ·d̃ᵢ), with predictable bets λᵢ ∈ [0, 1) (each λᵢ is chosen from the first i − 1 outcomes only).
Under the null hypothesis that B is no better (E[d̃ᵢ | past] ≤ 0) Eₙ is a nonnegative supermartingale with
E₀ = 1, so by Ville's inequality P(sup Eₙ ≥ 1/α) ≤ α: the chance it ever exceeds 1/α is at most α. The
comparison can stop at any time.

Sample-size intuition (illustrative; card GB-06): for a standardized paired effect δ a well-tuned betting
e-process crosses 1/α after about 2·ln(1/α)/δ² pairs; at α = 0.05, δ = 1 needs about 6 pairs, δ = 0.5 about
24, δ = 0.2 about 150. Pairing by world address raises δ by removing scene and disturbance variance.

**Registered predictions.** Predictions an agent registers before a run are scored with proper scoring rules
(a rule is proper when reporting the true belief maximizes the expected score: logarithmic and quadratic
rules).

**One bit per pair: the integer table.** For one bit per paired episode (does the disagreement exceed a frozen
envelope?), with exceedance rates p₀ < p₁ fixed in advance, a pair of one-sided e-processes runs on the bits.
After n pairs with k exceedances:

- promoting side, testing "the exceedance rate is at least p₁":
  ln E⁺ = (n − k)·ln((1 − p₀)/(1 − p₁)) + k·ln(p₀/p₁); promote when E⁺ ≥ 1/α⁺;
- rejecting side, testing "the rate is at most p₀":
  E⁻ = (p₁/p₀)ᵏ·((1 − p₁)/(1 − p₀))ⁿ⁻ᵏ; reject when E⁻ ≥ 1/α⁻.

Both compile to an integer table (n, k) → promote, reject or continue, regenerated and compared in CI. The
error rates hold at any stopping time under two conditions: the envelope is hashed before sampling, and
episodes are chosen by address before outcomes exist. Independence between episodes is not needed if the null
is read conditionally.

**Level spending.** Repeated confirmation requests spend α as αᵢ = α/[i·(i + 1)] for the i-th request. Since
1/[i·(i + 1)] = 1/i − 1/(i + 1) the series telescopes: Σ_{i ≤ m} αᵢ = α·m/(m + 1) < α for every m, and it sums
to α in the limit. The request is fixed before the draw, the clusters are fresh, and the level is spent
whatever the answer.

**Fixed-n studies and reachability** (theorem card T-26). For a declared fixed-n rule and sample size, the set
of verdicts reachable over all outcome counts is enumerated before any pair runs. The frozen paired rule used
as the fixture: n complete pairs, b pairs in which only B succeeds and a pairs in which only A succeeds; form
two-sided 97.5% exact Clopper–Pearson intervals [L_b, U_b] and [L_a, U_a] for the two discordance
probabilities; the interval [L_b − U_a, U_b − L_a] covers their difference with probability at least 95%.
Verdicts: prefer B if L_b − U_a > +0.05; B inferior if U_b − L_a < −0.05; practical tie if the interval lies
inside [−0.05, +0.05]; otherwise inconclusive. A study that needs an unreachable verdict is refused at
admission.

**Honest limits.**

1. Pairing by world address shares seeds. It does not make two robots that take different paths meet the same
   friction at the same tick; shared inputs are compiled at physical coordinates.
2. Paired statistics are refused under an uncontrolled determinism class.
3. Time-uniform inference and a sealed fixed-n decision are different procedures, and neither substitutes for
   the other.
4. The pair count of an e-process is an expectation, not a guarantee.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-15.S1 | Under the null with optional stopping, the empirical type-I error of the betting e-process is at or below α | Tested | 10⁴ simulated comparisons |
| C-15.S2 | The integer table at p₀ = 0.05, p₁ = 0.30, α⁺ = 0.01 promotes after 16 clean pairs, or 22 pairs with one exceedance | Certified | Exact rational arithmetic on the likelihood ratios |
| C-15.S3 | For a declared fixed-n rule and sample size, the reachable verdict set is as stated (T-26, first half) | Certified | Exhaustive enumeration in exact rationals |
| C-15.S4 | For every n, Σ_{i ≤ n} α/[i·(i + 1)] = α·n/(n + 1) < α (T-26, second half) | Proved (induction on n); Certified per n | Lean; exact rationals. The infinite sum is an analytic statement and is not needed |
| C-15.S5 | A study that declares both types, or re-reads fixed-n data as a sequential study, is refused | Tested | Planted study |
| C-15.S6 | A fixed-n study whose declared verdict set contains an unreachable verdict is refused at admission | Tested | Planted request |
| C-15.S7 | A paired study under an uncontrolled determinism class, or with arms in different classes, is refused | Tested | Planted study |
| C-15.S8 | Proper scoring rules are maximized in expectation by the true probability | Certified | Calculus identity on the quadratic rule; exact check on a grid for the logarithmic rule |

**Fixtures** (exact; hand-checkable).

- *Promotion table.* p₀ = 1/20, p₁ = 3/10, α⁺ = 1/100. A clean pair multiplies E⁺ by 0.95/0.70 = 19/14 ≈ 1.357
  and an exceedance multiplies it by (1/20)/(3/10) = 1/6. Promotion needs (19/14)ⁿ ≥ 100: n = 16
  ((19/14)¹⁵ ≈ 97.6 < 100 ≤ (19/14)¹⁶ ≈ 132.4). With one exceedance, (19/14)ⁿ⁻¹/6 ≥ 100 needs n − 1 = 21, so
  22 pairs. With two exceedances, 29 pairs.
- *Rejection.* α⁻ = 1/20: two exceedances in the first three pairs give 6²·(14/19) = 26.5 ≥ 20: reject.
- *Fallback rule arithmetic.* A fixed rule of 30 pairs promoting on at most two exceedances promotes wrongly
  with probability 0.0021 at p₁ = 0.30 and promotes with probability 0.81 at p₀ = 0.05 (binomial arithmetic).
- *Reachability at n = 64.* Every count pair (a, b) with a + b ≤ 64 is classified:

| n | Count pairs | Prefer B | B inferior | Practical tie | Inconclusive |
|---|---|---|---|---|---|
| 64 | 2,145 | 521 | 521 | 0 | 1,103 |
| 86 | 3,828 | 1,031 | 1,031 | 1 | 1,765 |
| 128 | 8,385 | 2,517 | 2,517 | 4 | 3,347 |
| 256 | 33,153 | 11,384 | 11,384 | 39 | 10,346 |

  (The count of pairs is (n + 1)·(n + 2)/2.) Perfect concordance (a = b = 0) at n = 64 gives the interval
  [−0.0662, +0.0662], since the upper end of a 97.5% two-sided exact interval at zero successes is
  1 − 0.0125^(1/64) = 0.0662: wider than the ±0.05 band, so no outcome at n = 64 can return a tie. A tie first
  becomes reachable at n = 86 (1 − 0.0125^(1/86) = 0.0497) and only for (a, b) = (0, 0). With no A-only pair
  a preference for B needs at least 15 B-only pairs of 64, and 17, 19, 21 and 23 with one to four A-only
  pairs. (The construction's own text omits the level; the value 0.0662 reconciles only with the two-sided
  97.5% intervals stated here. At a 95% two-sided level the bound would be 0.0560 and the first tie at
  n = 72.)
- *Level spending.* α = 1/20: α₁ = 1/40, α₂ = 1/120, α₃ = 1/240; after m = 3 requests 3/80 = α·3/4.

**Refusal codes this construction's statements exercise.** FROB-REF-553: a comparison that mixes study types,
or re-reads fixed-n data as a sequential study. FROB-REF-555: a fixed-n study whose declared verdict set
contains an unreachable verdict (repair: raise n to the first size at which the verdict is reachable, which
the pre-analysis reports, or drop the verdict from the declared set). FROB-REF-542: a paired study under an
uncontrolled determinism class, or with arms in different classes. FROB-REF-543: an anytime study on a metric
with no declared span. FROB-REF-556: a pairing keyed by event order or by a training counter. These are the
codes of the comparison path (`pairing::admit`, reached through `eval.compare`), which GATE-ATEL-06 counts. The
first three defects also have a code on the study path (`study::admit`, reached through `study.plan`): 552 for
mixed types, 532 for an unreachable verdict or a missing pre-analysis, 533 for an uncontrolled class. Both codes
of each pair stay in the registry, and one request gets one code on one verb (the layer rule of the refusal
registries N24-REF and N28-REF); a test of S5 to S7 asserts the code of the path it drives.

**What it does not claim.** That an e-process or a fixed-n rule delivers its error rate in the running system
without the sampling assumptions: that is checked by null calibration over 10⁴ simulated studies and guarded
by refusals, and it is never presented as `Proved` or `Certified`. That pairing equalizes what two robots
experience.

**Implementation map.** Crate `frob-arena` (studies), with `frob-opt` consuming verdicts. Type sketch (DESIGN):
`enum Study { Anytime(AnytimeStudy), FixedN(FixedNStudy) }`; `struct PreAnalysis { outcomes: u64, reachable:
Set<StudyVerdict>, power: Vec<(Alternative, Ratio)>, hash: Blake3 }`; `enum StudyVerdict { Promote, Inferior,
PracticalTie, Inconclusive }`; the accounting identity a_only + b_only + both + neither + unscored = planned.
Verbs: `study.plan` (binds a manifest and one study type; returns the plan address, the cost and the
reachable verdicts), `eval.preanalyze`, `eval.compare`. Gates: GATE-ARENA-05 (TARGET, M1: `eval.preanalyze`
reproduces the enumeration tables exactly and refuses the unreachable verdict); GATE-ATEL-06 (M1: the fixed-n
planner reproduces the enumeration at n = 64 and n = 86 exactly; the five refusal codes above each fire on
their planted violation, 100 of 100); GATE-QUAL-01 (level spending under adaptive retries; M3). Card: T-26.

## Acceptance criteria

- [ ] `docs/math/C-15.md` exists with both study types, the e-process and Ville's inequality, the integer
      table formulas for both sides, level spending, the fixed-n rule, the four limits, the eight statements
      and the fixtures with every number above.
- [ ] `cargo test -p frob-arena --test laws_c15` passes on the exact reference and on production code once
      BD-124 lands.
- [ ] The integer table reproduces 16, 22 and 29 pairs for zero, one and two exceedances, and rejection at two
      exceedances in three pairs.
- [ ] The enumeration reproduces all four rows of the reachability table exactly, the interval
      [−0.0662, +0.0662] to four decimals, and first tie at n = 86.
- [ ] Null calibration: at most α·10⁴ + 3·√(α·(1 − α)·10⁴) false rejections in 10⁴ null comparisons with
      optional stopping (α = 0.05: at most 565).
- [ ] The level series equals α·m/(m + 1) exactly for m up to 10⁴.
- [ ] Planted violations each produce their refusal code.
- [ ] KA-088 to KA-091 registered with tolerances and controls set; the register row of C-15 filled.

## Test plan

### Unit tests

- `s1_null_calibration`: 10⁴ comparisons, each up to 500 pairs of symmetric bounded differences, stopping at
  the first crossing of 1/α; count crossings.
- `s2_promotion_table`: exact rational powers of 19/14 and 1/6 against 100; also α⁺ = 1/10 or p₁ = 1/2 gives
  eight clean pairs.
- `s2_rejection_table`: 36·(14/19) ≥ 20.
- `s3_reachability_{64,86,128,256}`: exact Clopper–Pearson bounds computed in rational interval arithmetic
  with outward rounding; counts as tabulated.
- `s3_perfect_concordance`: upper bounds 1 − 0.0125^(1/n) for n = 64, 85, 86.
- `s4_level_series`: exact rationals; strict inequality.
- `s5_mixed_types_refused`, `s6_unreachable_verdict_refused`, `s7_uncontrolled_class_refused`: planted
  studies, each driven through both admission paths once they exist: the study path answers 552, 532 (with
  first reachable n = 86 in the repair) and 533; the comparison path answers 553, 555 (same repair) and 542;
  never both codes for one request on one path. Before BD-124 lands the tests assert refusal on the exact
  reference and are marked `path-pending` for the codes.
- `s8_proper_scores`: quadratic and logarithmic rules on a grid of true probabilities and reports.
- `pair_accounting_identity`: random studies; the five counts sum to the planned count.

### End-to-end script and logging

`scripts/e2e/spec_c15.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G0 and G1 rows
KA-088 to KA-091; `frob eval preanalyze --rule paired-clopper-pearson/1 --n 64 --level 0.05 --margin 0.05
--json` (the rule identifier, flags and verdict tokens of N24b-06) asserting counts 2,145, 521, 521, 0, 1,103
and that `practical-tie` is absent from `reachable`; the same at n = 86 asserting one tie; `frob study plan
--json` with a fixed-n study that declares `practical-tie` at n = 64, asserting refusal FROB-REF-532 with the
repair "first reachable at n = 86" (study path), and the same declaration through `frob eval compare … --key
spec-c15-unreachable --json` asserting FROB-REF-555 with the same repair (comparison path); a study declaring
both types, asserting FROB-REF-552 through `study plan` and FROB-REF-553 through `eval compare … --key
spec-c15-mixed --json` (`eval.compare` is a run verb: each call carries its own idempotency key). Each refusal
line logs the verb, the
path and the code expected for that path. Log `target/e2e-logs/spec_c15/<utc>.ndjson` with transcript: commands, exit codes, counts
against expected, refusal codes and repairs, the pre-analysis hash, the evidence block, and on failure the
first count pair classified differently. Final `summary` line.

### Known answers and negative controls

- KA-088 error rate of the betting e-process under the null with optional stopping: at or below α over 10⁴
  comparisons; G0 · host. Control (set here): a fixed-level test evaluated at every look, whose error rate
  must exceed α; and bets chosen after seeing the outcome they multiply.
- KA-089 integer promotion table: exact; G1 · host. Control (set here): the table generated with the
  exceedance factor inverted (6 in place of 1/6), and an envelope changed after the first outcome.
- KA-090 reachability at n = 64: exact verdict, interval to four decimals; G0 · host. Control: a request for
  a tie verdict at n = 64.
- KA-091 level spending: exact; G0 · host. Control (set here): a constant level α per request, whose total
  after m requests is m·α.
- CX-040.
- Metamorphic relation: an anytime-valid study's level holds at every stopping time, and a fixed-n study's
  verdict is computed once (mutation: peeking).
- Determinism: enumeration and tables are D0; the null calibration is seeded and reproduces its count
  exactly.

## Considerations

- The reachability figures are FACT from an executed enumeration at plan time and are re-run here; the
  enumeration in `s3` is that re-run, and a disagreement is a finding filed against the derivation file, not
  silently "fixed".
- Risk: an envelope or a bound tuned after outcomes voids every guarantee. The hash-before-sampling rule is
  enforced by the ledger; this contract states it as a precondition of S1 and S2.
- Risk: exact Clopper–Pearson bounds computed in floating point misclassify boundary count pairs. The
  reference uses outward-rounded rational intervals and compares strictly.
- A stratum whose true exceedance rate lies between p₀ and p₁ reaches its pair budget and stays provisional;
  that is the intended behaviour, not a defect.

## References

Plan §10 C-15; §16.8; §24.10, §24.12, §24.13; §28.6; §35; §12.3 (T-26); §38.7; Appendix D.8 (Ville's
inequality, betting e-process, error budget for retries); Appendix E GB-06, GB-14; Appendix I.9; Appendix M
KA-088 to KA-091 and the reconciliation note on the level; Appendix N CX-040.
