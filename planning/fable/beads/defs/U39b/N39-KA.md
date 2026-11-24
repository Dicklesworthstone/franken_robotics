---
key: "N39-KA"
title: "frob-conformance: the known-answer registry of the ten-fold harness (cards, guard, estimands, recovery)"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tenfold", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-60"
deps: ["BD-025", "N00-KA", "N24-KA", "N38a-12", "N38a-40", "STD-E2E", "STD-TEST"]
needs: []
related: ["N39-REF", "N39-CX", "N39b-22", "N39a-06", "N39a-09", "N39a-28", "N39a-30", "N39a-31", "N39a-33", "N39a-34", "N39a-36", "N39b-16", "N39b-34", "N38b-27", "N38a-13", "N41-KA", "N32-KA"]
gates: ["GATE-TENFOLD-03", "GATE-TENFOLD-10", "GATE-TENFOLD-13"]
plan_refs: ["§39.16", "§39.17", "§39.20", "§39.22", "§39.12", "§39.7", "§39.8", "§39.18", "§24.13", "Appendix M.4", "Appendix D.9", "§38.6", "§39.3"]
---
## Background

The harness of the ten-fold test turns phase ledgers and receipts into ratios, intervals and verdicts. A harness
that computes the wrong ratio would misstate the program's central result, so it has its own known answers:
small hand-checkable cases with exact values, each paired with the wrong computation it guards against. The gate
GATE-TENFOLD-03 requires the harness to reproduce every one exactly and to catch every negative control before a
sealed suite is opened (M3, repeated at M7); GATE-TENFOLD-10 requires the pre-analysis rows before the partial
gate. This bead is the registry: the rows as data, with derivations. The printed known-answer appendix of the
program has no rows filed under this chapter (its only related rows are KA-009, the completion cap, and KA-014,
the general speed-up bound, filed under the thesis chapter); §39.17 says its known answers are "numbered in §38",
that is, by the verification chapter's allocation ledger (N38a-40). This bead files one request in that ledger
for 30 numbers and maps its labels to them in table order; numbers are never reused or renumbered. Until the
block is issued the rows are cited by the local labels `TF-01` to `TF-22`, and the rows of the first half of the
protocol by the labels `TF-23` to `TF-30`; each label is kept afterwards in its row's `slug`.

Symbols. H: the stratum deadline. T_A, T_B: clock times of arms A (this system) and B (the rival); "c" marks a
censored run, charged H. R̂ = Σ min(T_B, H) ÷ Σ min(T_A, H). ρ: a per-family ratio. S: build cost of a standing
asset. ā, b̄: pooled restricted mean time per amendment. R_warm(N) = (S_B + N·b̄) ÷ (S_A + N·ā). N*: the smallest
N with S_A + N·ā ≤ S_B + N·b̄. r_T = (T_B − T_W)/(T_B − T_F): recovery ratio of the composed incumbent W.

## Scope

In: the 30 rows as `[[ka]]` records of the suite's one file in N38a-12's row schema; the block request and the
label-to-number mapping (N38a-40); their 30 derivation files (TF-23 to TF-30 restating the derivations their
mechanism beads state, so that no row waits on a bead that does not precede this one); their fixture files
(ledger records for the cards); a supplied control for each row whose control was blank; the loader test and
this bead's own registry script. Out: the test target that runs them through the harness (N39b-22, which
depends on this bead); the runner bindings of TF-23 to TF-30 (each mechanism bead's unit test, bound as
`ka:<id>@host`); the counterexample records (N39-CX) and the NC records seeded from `control.text` (N38b-05);
the thesis rows KA-009 to KA-016 (N00-KA) and the arena row KA-621 (N24-KA), which two rows here share.

## Technical approach

**Records.** Each row becomes one `[[ka]]` record of the suite's one file
`crates/frob-conformance/registry/known_answers.toml`, in N38a-12's row schema: `id` (the block's number),
`section` (below), `owner` (GATE-TENFOLD-03 for TF-01 to TF-16, the known answers of §39.17; GATE-TENFOLD-10
for TF-17 to TF-20, the pre-analysis known answers of §39.20 that gate lists; GATE-TENFOLD-13 for TF-21 and
TF-22, the recovery ratio and ablation bounds of §39.22; empty for TF-23 to TF-30, which no gate battery
lists), `fixture`,
`same_as`, `case`, `truth` (an exact rational or integer where one exists), `tolerance` with a `reason` naming
its rule of N38b-27, `tier = "G1"` and `sites = ["host"]`, `control.text` (the "Guards against" column, or the
control supplied below; `control.nc` filled by N38b-05's seeding, which links an NC to the TF-CX record of N39-CX
where the plant is the same), `derivation`, `label`, `due`, `printed` (empty on all 30: no recomputation
disagrees with a printed value) and `slug` (the TF label). The derivation file is
`docs/derivations/<fixture>.md`, one per fixture, listing in its front matter the rows it serves and for each
the test that checks it. Fixture inputs live in `crates/frob-conformance/fixtures/tenfold/` as
`frob.tenfold.ledger/1` records (labelled `synthetic`), with their digests recorded in the derivation file.
Exact rows are compared as rationals; printed decimals are checked to the digits shown.

**Numbers.** One request in the allocation ledger of N38a-40 (`[[block]] registry = "ka", count = 30,
owner = "N39-KA", section = "§39.16, §39.17, §39.20, §39.22"`); when the block is issued the labels map to its
numbers in table order (TF-01 the first, TF-30 the last) in one commit, before the M3 exit; a label cited in
place of its number in a test name, a derivation file or another bead's record at the M3 exit fails
(`provisional-tag-past-due`). This bead prints no number before the block is issued.

**Fixture A, one pair, cold, H = 14,400 s** (exclusive seconds; B then A): `author` 900, 600; `propose` 1,080,
360; `check` 240, 20; `synthesize` 0, 0; `compile` 540, 12; `screen` 0, 96; `train` 4,920, 150; `evaluate` 360,
60; `qualify` 0, 40; `certify` 0, 0; `diagnose` 600, 30; `wait` 180, 42; `adjudicate` 180, 90; clocks 9,000 and
1,500. **Fixture B, six families** (T_B, T_A): (9,000, 1,500), (6,300, 900), (14,400 c, 2,100), (5,400, 700),
(11,700, 14,400 c), (7,200, 800). **Fixture C, warm:** one amendment costs arm B 4,050 s and arm A 470 s;
S_A = 14,400 s, S_B = 0.

| Label | Case | Truth | Tolerance | Guards against |
|---|---|---|---|---|
| TF-01 | Fixture B pooled ratio | Σ T_B = 54,000, Σ T_A = 20,400, R̂ = 45/17 = 2.647 | exact | Averaging ratios (6.2307, prints 6.23); dropping the censored family 5 (42,300 ÷ 6,000 = 7.05) |
| TF-02 | Fixture B per-family ratios | 6, 7, 48/7 = 6.857 (a lower bound: B censored), 54/7 = 7.714, 13/16 = 0.8125, 9 | exact | Treating a censored rival time as an observed time |
| TF-03 | Fixture B completion ceiling | One miss in six: R̂ ≤ 54,000 ÷ 14,400 = 3.75 on these rival times; at most 1/q = 6 on any | exact | A ratio claimed above its ceiling |
| TF-04 | Fixture B verdict | None: n = 6 is below 10, and the guard needs 6 of 6 above 5 where 5 are. In a registered cell with these proportions: `withdrawn` | exact | A verdict issued under the floor |
| TF-05 | Fixture A pair ratio | 9,000 ÷ 1,500 = 6 (each clock equals its exclusive phases' sum: 9,000 and 1,500, so the ledger closes, INV-TENFOLD-04) | exact | Supplied here (DESIGN; the plan's card names only the ceiling control, which TF-06 carries): a ratio of "active" phase sums that leaves out `wait` and `adjudicate` (8,640 ÷ 1,368 = 6.316) |
| TF-06 | Fixture A, training removed | Share 4,920/9,000 = 54.67%; ceiling 9,000 ÷ 4,080 = 75/34 = 2.206 | exact | A ceiling computed on the wrong arm's ledger (arm A's own train share is 10%, giving 1.11) |
| TF-07 | Fixture A, unremovable floor | `author` + `propose` + `adjudicate` of arm A = 1,050 s = 11.67% of the rival's clock; cap 9,000 ÷ 1,050 = 60/7 = 8.571 | exact | Ignoring cost no mechanism removes |
| TF-08 | Fixture A, a 2.67% phase | `check` is 240/9,000; removing it gives at most 1/(1 − 2/75) = 75/73 = 1.027; not fundable as a speed lever (under 10%, where the cap is 1/0.9 = 1.11) | exact | Funding a lever with no removable share |
| TF-09 | Fixture C warm ratios | R_warm(1) = 4,050/14,870 = 0.272; R_warm(10) = 40,500/19,100 = 2.120; R_warm(100) = 405,000/61,400 = 6.596; limit 4,050/470 = 8.617 | exact; decimals to three places | Quoting the limit as the warm ratio; omitting a standing asset |
| TF-10 | Fixture C crossover | N* = 5 (14,400/3,580 = 4.02); warm verdict taken at N = 10 where the class measures 2.12 | exact | Summarizing a class by its limit |
| TF-11 | Exact guard, critical counts at α = 0.05 | 5 of 5, 6 of 6, 7 of 8, 10 of 12, 12 of 16, 17 of 24, 22 of 32; P(X ≥ 17 at 24) = 0.03196, P(X ≥ 16 at 24) = 0.07579 | exact (binomial arithmetic) | An approximate test |
| TF-12 | Guard under Holm with four cells | 18 of 24 at 0.0125 (tail 0.01133) | exact | An unadjusted guard on several cells |
| TF-13 | Guard at m = 4 | 4 of 4 has tail 1/16 = 0.0625: no count passes; a four-family confirmatory cell is refused | exact | A verdict that cannot be reached |
| TF-14 | Zero-count bound | 0 misses in 24 independent families: one-sided 95% upper bound 1 − 0.05^(1/24) = 0.117346, printed 0.1174 (at 12: 0.220922, printed 0.2210; at 32: 0.089368, printed 0.0894; at 46: 0.063049, printed 0.0631; an upper bound is printed rounded up, the rule for every bound in a harness record) | 10⁻⁴ | A zero-width interval from 24 zeros |
| TF-15 | Completion cap | 12% unfinished caps R at 25/3 = 8.33 (KA-009: `same_as = "KA-009"`, fixture `thesis-ceilings`) | exact | A ratio claimed above its ceiling |
| TF-16 | Monitor | E multiplies by 1.5 for each family below 3 and by 0.5 otherwise; 1.5⁸ = 25.63 ≥ 20 stops, 1.5⁷ = 17.09 does not | exact | An early stop without anytime validity |
| TF-17 | Two estimands, 25 serial missions, H = 100 s | Rival 50 s each (1,250 s, 25 accepted); candidate 24 at 1 s and one timeout (124 s, 24 accepted): R_time = 625/62 = 10.081; R_rate = (24/124) ÷ (25/1,250) = 300/31 = 9.677 | exact | Citing a rate ratio as the time ratio |
| TF-18 | Rare tail, H = 1,000 s | Rival always 20 s; candidate 1 s on 99% of families, 999 s on 1%: population ratio 20/10.98 = 1.8215; P(a 24-family suite has no slow family) = 0.99²⁴ = 0.78568, and then every percentile-bootstrap resample reports 20 | 10⁻⁴ and 10⁻⁵ | A percentile bootstrap used as a population certificate |
| TF-19 | Family counts for a population claim | n ≥ ln(1/α)/ln(1.1): 32 at 0.05, 46 at 0.0125, 49 at 0.01; the Hoeffding bound X̄ − 11·H·√(ln(1/α)/(2n)) needs n ≥ 121·ln(1/α)/2 = 182 at 0.05 (nonnegative at 182, negative at 181, on the all-fast sample) | exact integers | A population claim from 24 families |
| TF-20 | Rare-tail campaigns | 2,000 simulated campaigns of 24 families: the contrast test certifies ten-fold in 0; the percentile bootstrap certifies it in 78.6% ± 3 points | counts; ± 3 points | The same |
| TF-21 | Recovery identity | If T_F = 0.10·T_B then r_T ≥ 0.8 exactly when T_W ≤ 0.28·T_B; T_F ≥ T_B gives "no saving to recover" (r undefined); T_W < T_F gives r > 1 | exact | Dividing by a non-positive saving |
| TF-22 | Ablation bounds | Unremoved training share s = 0.7 caps an ablation at 1/s = 1.43; removing it entirely caps at 1/(1 − s) = 3.33. 50 designs at 35 min with 1.5 min of agent time each: training 1,750 of 1,825 min = 95.9%, information alone capped at 1.04; with acquisition at 45 s per design, agent time is 75 of 112.5 min = 2/3 | exact | An ablation ratio above its accounting bound |

**Rows of the first half of the protocol (acceptance, submissions, bootstrap, non-inferiority, pre-analysis,
report).** Their derivations are in the mechanism beads named in the last column, which assert them in their own
unit tests; they are registered here so that the chapter has one list. They are not among the rows the gate
battery of GATE-TENFOLD-03 must reproduce (that battery is rows TF-01 to TF-16, with TF-17 to TF-22 run beside it).

| Label | Case | Truth | Tolerance | Derived and tested in |
|---|---|---|---|---|
| TF-23 | Acceptance battery sizes with zero allowed failures, pass threshold p⋆ | p⋆ = 0.99: 368, 477, 546 worlds at submissions 1, 2, 3 and 766 at submission 10; 299 at a single test of level 0.05. p⋆ = 0.9: 36, 46, 53 and 74. p⋆ = 0.9999: 29,956 at level 0.05 and 76,959 at α₁₀ = 0.05/110 (each n_i = ⌈ln α_i ÷ ln p⋆⌉) | exact | N39a-06 |
| TF-24 | Cost of a retry: battery size over the single-test size in the large-p⋆ limit, ln α_i ÷ ln 0.05 with α_i = 0.05/[i·(i + 1)] (the limit, as N39a-06 states it, not the ratio of TF-23's integer sizes, which would miss two printed values) | 1.2314, 2.5691, 4.0778, 5.1496 at submissions 1, 10, 100, 500, printed 1.23, 2.57, 4.08, 5.15; the integer sizes give 1.23, 2.56, 4.07 and 5.13 (`same_as = "KA-621"`, the arena row of §24.13 that prints the same four ratios, with that row's fixture) | printed digits (rule 2 of N38b-27: half a unit in the last printed digit, 0.005; it replaces this bead's earlier 0.01, which named no rule, and matches KA-621) | N39a-06 (and N39a-09 for the series) |
| TF-25 | The level series of committed submissions (the α = 0.05 card of the general identity KA-760, §32.20: a separate fixture by N38a-12's rule for two cards of one formula) | α_i = 0.05/[i·(i + 1)]: 1/40, 1/120, 1/240, 1/400, 1/600 for i = 1…5; spent after m submissions (1/20)·m/(m + 1) | exact | N39a-09 |
| TF-26 | Bootstrap reference on fixture B (six families), seed 39, B = 10,000 | p₀ = 9,569/20,000; k_L = 398, k_U = 9,380; L = 612/469 (printed 1.304), U = 126/17 (printed 7.412) | exact | N39a-28 |
| TF-27 | Attainability of non-inferiority at margin 0.05, level 0.05 | Smallest m with W = 0, 1, 2, 3 families worse: 59, 93, 124, 153 (0.95⁵⁹ = 0.04849 < 0.05 ≤ 0.95⁵⁸ = 0.05105); at m = 24 the best attainable bound is −0.1174 | exact | N39a-30, N39a-31 |
| TF-28 | Planning table of the pooled rule (P(`ten-fold`) at true ratios 5 / 7 / 8.5 / 10 / 12 / 15 / 20; P(`withdrawn`) at 2 / 3 / 5) | m = 24, s = 0.5: 0.000 / 0.002 / 0.08 / 0.50 / 0.94 / 1.00 / 1.00 and 1.00 / 0.50 / 0.000. m = 24, s = 0.8: 0.000 / 0.03 / 0.20 / 0.49 / 0.83 / 0.98 / 1.00 and 0.98 / 0.50 / 0.005. m = 12, s = 0.8: 0.002 / 0.05 / 0.19 / 0.39 / 0.65 / 0.89 / 0.98 and 0.93 / 0.50 / 0.03. m = 6, s = 0.8: 0.006 / 0.05 / 0.12 / 0.23 / 0.38 / 0.57 / 0.77 and 0.86 / 0.49 / 0.08. m = 4: `ten-fold` unreachable; 0.81 / 0.50 / 0.13 | 0.015 (rule 4 of N38b-27, a numerical-method budget: a planning simulation, the condition omitting completion non-inferiority) | N39a-33; re-run by the pilot N39b-16 |
| TF-29 | Holm levels after one of four proposed cells opens as descriptive | k = 3: 0.016667, 0.025, 0.05 (that is 1/60, 1/40, 1/20) | exact | N39a-34 |
| TF-30 | Per-phase ratio on fixture A | `train` 4,920/150 = 32.8; `compile` 540/12 = 45; `screen` 0/96 = 0; a phase with 0 ÷ 0 (`synthesize`, `certify`) prints `n/a` | exact | N39a-36 |

**Controls of TF-23 to TF-30** (the second table had no control column, and N38a-12 counts a row with no
control as `obligation: control`; each control below is the mechanism bead's own negative control, or, where
the bead names none for this value, one supplied here and labelled DESIGN):

| Label | Negative control that must be caught | From |
|---|---|---|
| TF-23 | A battery at a constant level across submissions: 299 worlds at every index for p⋆ = 0.99, where 368, 477, 546 and 766 are required | N39a-06 |
| TF-24 | The same constant-level battery: ratio 1 at every index | N39a-06 |
| TF-25 | An index reset by reopening the campaign: submission 6 spends 1/40 again (refused: the index is keyed by mission and campaign) | N39a-09 |
| TF-26 | A bootstrap that resamples pairs instead of families, or a library random generator in place of the reference stream: either differs from k_L = 398, k_U = 9,380 | N39a-28 |
| TF-27 | A Wald interval for the difference, which reports [0, 0] for two arms both 24 of 24 and declares non-inferiority at any m | N39a-30 |
| TF-28 | A model with the median per-family ratio set to R instead of the ratio of means: P(`withdrawn`) at 3 near 0.07 at m = 24, s = 0.8, not 0.50 | N39a-33 |
| TF-29 | Holm levels left at k = 4 after the cell opened descriptive (0.0125 first) | DESIGN, supplied here |
| TF-30 | A phase ratio that prints 0 or NaN for 0 ÷ 0 instead of `n/a` | DESIGN, supplied here |

**Sections (`section`).** TF-01 to TF-16: §39.17 (TF-11 to TF-13 and TF-16 also §39.7); TF-17 to TF-20:
§39.20, Appendix D.9; TF-21 and TF-22: §39.22 (TF-21 also Appendix D.9); TF-23 to TF-25: §39.3, §24.13; TF-26 and
TF-27: §39.7, §39.20; TF-28 and TF-29: §39.7, §39.18; TF-30: §39.7, §39.17.

**Fixtures (DESIGN names) and shared rows.** `tenfold-pair-a` (fixture A: TF-05 to TF-08, TF-30);
`tenfold-families-b` (fixture B: TF-01 to TF-04, TF-26); `tenfold-warm-c` (fixture C: TF-09, TF-10);
`exact-guard` (TF-11 to TF-13, TF-29); `zero-count-bound` (TF-14); `thesis-ceilings` (TF-15, `same_as =
"KA-009"`, N00-KA's row and fixture, a dependency); `anytime-monitor` (TF-16); `two-estimands-25` (TF-17);
`rare-tail-1pct` (TF-18 and TF-20, and the delivery chapter's K41-10 of N41-KA, which states the same
population: deadline 1,000 s, rival 20 s, candidate 1 s on 99% and 999 s on 1%); `population-family-counts`
(TF-19); `recovery-identity` (TF-21); `ablation-bounds` (TF-22); KA-621's fixture (TF-24, `same_as = "KA-621"`,
N24-KA's row, a dependency: N24-KA, at M1, cannot list a row that does not exist until this M3 bead, so this bead
adds TF-24 to the front matter of KA-621's derivation file, N24-KA's, in the commit that adds TF-24, by its
number, or by its `slug` until the block is issued and then by the number in the mapping commit, and writes no
second file for that case); `level-series-0.05` (TF-25); `noninferiority-attainability` (TF-27);
`pooled-rule-planning` (TF-28). Within the chapter `same_as` stays empty. Between TF-18 and K41-10, both above
KA-835 and numbered from two blocks of the ledger, the row that receives the higher number carries `same_as`
naming the lower (DESIGN: N38a-12's rule that `same_as` names an earlier row, applied without assuming the
order in which the ledger issues the two blocks); N41-KA applies the same rule. The shared derivation file is
written by whichever of the two beads lands first and appended to by the other.

**Labels.** FACT, with the source and "to be re-run or re-read before a gate cites it", for TF-20 (2,000
simulated campaigns: a planning computation, re-run when GATE-TENFOLD-10 is measured through N39b-22's harness), TF-26 (a seeded bootstrap computed at
planning time: reproduced only by running the reference generator) and TF-28 (a planning simulation, re-run by
the pilot N39b-16 at M7). DESIGN for the other 27 (arithmetic on synthetic fixtures).

**Tolerance reasons (the rule of N38b-27 each names).** Rule 1 for every `exact` row and for the exact rational
behind each printed decimal; rule 2 for the printed decimals (TF-09's three places, TF-18's 1.8215 and 0.78568,
TF-24); TF-14 is rule 2 applied outward (DESIGN): an upper bound is printed rounded up, so the printed value is
never below the exact one and lies within one unit of its last digit, 10⁻⁴ (0.117346 prints 0.1174, 5.4 × 10⁻⁵
above; half a unit would reject it); rule 4 (a Monte Carlo budget) for TF-20's ± 3 points and TF-28's 0.015.
A tolerance that would accept its row's control is refused (`tolerance-swallows-control`): none here does
(the nearest is TF-24, whose control gives 1 against 1.23).

**Due.** M3 for all 30: GATE-TENFOLD-03, GATE-TENFOLD-10 and GATE-TENFOLD-13 are due at M3 before the suite
opens, and every mechanism bead of TF-23 to TF-30 lands at M3.

Registry rows of other chapters that the first half cites with their values: KA-009, KA-012, KA-014, KA-015 (the
thesis arithmetic) and KA-809, KA-811, KA-812 (the performance chapter); they stay in their own registries.

Planning arithmetic also checked by this bead's unit tests (not registry rows): a cell of 24 families, three arms
and H = 4 h costs at most 288 device-hours; at restricted mean times of 1.5 h, 1.0 h and 0.4 h it costs
24 × 2.9 = 69.6.

## Acceptance criteria

- [ ] 30 records exist in `crates/frob-conformance/registry/known_answers.toml` with every field of N38a-12's
      schema: TF-01 to TF-22 with derivation files and fixture files, TF-23 to TF-30 with derivation files that
      this bead writes from the derivations their mechanism beads state; each names at least one control
      (TF-05, TF-23 to TF-30 with the controls supplied above). No other known-answer file exists
      (`ka-second-file`).
- [ ] The block request of 30 is filed in N38a-40's ledger; once issued, the labels map to its numbers in table
      order with each label in `slug`; at the M3 exit no label is cited in place of its number.
- [ ] No row has tolerance `to-set` or control `none stated`; every `reason` names its rule of N38b-27; no
      tolerance swallows its control.
- [ ] Every exact row is stored as a rational and compared as one.
- [ ] `same_as` is `KA-009` on TF-15 and `KA-621` on TF-24 with those rows' fixtures; the commit that adds TF-24
      also adds it to the front matter of KA-621's derivation file (N24-KA's), with its test; TF-18 and K41-10 share
      `rare-tail-1pct`, the higher-numbered of the two naming the lower; the change in the shared-fixture count
      is reported row by row (N38a-12).
- [ ] TF-20, TF-26 and TF-28 carry the label FACT with their sources and the re-run note.
- [ ] The registry loader refuses a row whose fixture digest differs from the recorded digest.

## Test plan

### Unit tests

`FROB_TEST_SEED=0 cargo test -p frob-conformance registry::tenfold_rows`: `rows_load_and_are_unique`;
`schema_fields_complete` (30 rows; owners GATE-TENFOLD-03 × 16, -10 × 4, -13 × 2, empty × 8; due M3 × 30);
`every_row_has_control` (a planted TF-05 with an empty control is reported `obligation: control`);
`tolerance_reasons_name_rules` (rule 1, 2 or 4 on each; TF-14's outward rounding: 0.117346 → 0.1174 accepted,
0.1173 rejected); `derivations_exist`; `fixture_a_closes` (B's thirteen phases sum to 9,000 and A's to 1,500;
without `wait` and `adjudicate` 8,640 and 1,368, ratio 6.316, which TF-05's tolerance rejects);
`fixture_b_sums` (54,000 and 20,400 recomputed from the fixture file); `binomial_tails_exact` (integer
arithmetic over 2²⁴ and 2³²: P(X ≥ 17 at 24) = 0.03196, P(X ≥ 16) = 0.07579; Holm 18 of 24 tail 0.01133);
`retry_cost_limit` (ln α_i ÷ ln 0.05 at i = 1, 10, 100, 500 to the printed digits; the integer-size ratios
2.56, 4.07 and 5.13 fail them); `battery_sizes` (TF-23's eleven sizes from ⌈ln α_i ÷ ln p⋆⌉);
`level_series_exact` (TF-25 in rationals); `noninferiority_counts` (59, 93, 124, 153); `shared_fixtures`
(TF-15 → KA-009, TF-24 → KA-621, and KA-621's derivation file lists TF-24's row with its test; the TF-18/K41-10
rule applied to a scratch ledger in each order);
`labels_mapped_in_table_order` (once the block is issued; a planted test name citing TF-07 after the mapping
fails `provisional-tag-past-due`); `planning_cell_cost` (288 and 69.6).

### End-to-end script and logging

`scripts/e2e/ka_tenfold_registry.sh` (this bead, M3; sources the shared helper of STD-E2E) checks what this bead
delivers: it runs the unit tests above and the printout regeneration (`cargo test -p frob-conformance --test
known_answers_printout`), and logs one `assert` line per row with the label and number, the owner gate, the
fixture and its digest, the truth as stored, the value recomputed from the fixture file, the tolerance and its
rule, the control and the control's computed wrong value with its distance from the truth, the label (FACT or
DESIGN) and the due milestone; a `summary` line gives rows loaded, rows reproduced, controls separated, FACT
rows awaiting their re-run, and labels still unmapped. The rows are then exercised through the harness by
`scripts/e2e/tenfold_known_answers.sh` (N39b-22, which depends on this bead), which logs each row as an
assertion with its number (and slug) and runs this script as its first stage.

### Known answers and negative controls

The tables. Negative controls for the registry itself: a planted edit of fixture B (family 5 changed to 1,440 s)
must be refused by the digest check, not silently produce R̂ = 54,000 ÷ 7,440; a copy of the file with TF-24's
tolerance widened to 0.3 must be refused as swallowing its control (ratio 1 against 1.23); a copy with TF-15's
`same_as` naming a row of another fixture must fail `same_as_requires_same_fixture`.

## Considerations

- The rows are harness arithmetic, not results; every value is illustrative of the computation.
- The illustrative power table of the decision rule (lognormal times, 20,000 simulated gates per entry) is owned
  by the statistics beads and is a planning value, not a registry truth.
- TF-15 duplicates KA-009 by design: one fixture identity, two rows (host mathematics and harness); TF-24 and
  KA-621 likewise (the arena's retry cost and the harness's).
- TF-24 was first entered as a ratio of battery sizes with tolerance 0.01; the integer sizes at p⋆ = 0.99 miss
  two of the four printed values, and N39a-06 states the large-p⋆ limit, so the row is that limit.
- The derivations of TF-23 to TF-30 are restated here rather than read from the mechanism beads' files, because
  those beads do not precede this one; each mechanism bead's unit test is the row's binding, and a difference
  between the two is a failing test, not a second source of truth.

## References

§39.16, §39.17, §39.20, §39.22, §39.3, §39.7, §39.18, §24.13, Appendix D.9; GATE-TENFOLD-03, -10, -13; KA-009,
KA-014, KA-621, KA-760; CX-003, CX-008; N38a-12, N38a-40, N38b-27.
