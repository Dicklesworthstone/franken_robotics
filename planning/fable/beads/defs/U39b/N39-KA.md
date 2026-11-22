---
key: "N39-KA"
title: "frob-conformance: the known-answer registry of the ten-fold harness (cards, guard, estimands, recovery)"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tenfold", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-60"
deps: ["BD-025", "BD-146", "STD-TEST"]
needs: ["§38.6: the known-answer loader over `frob-conformance/registry/known_answers.toml` and the rule that each row has a derivation file under `docs/derivations/`"]
related: ["N39-CX", "N39-REF", "N39b-22", "N39b-19", "N39b-36", "N39b-43", "N00-17", "N00-KA"]
gates: ["GATE-TENFOLD-03", "GATE-TENFOLD-10"]
plan_refs: ["§39.16", "§39.17", "§39.20", "§39.22", "§39.12", "§39.7", "§39.8", "Appendix M.4", "Appendix D.9", "§38.6"]
---
## Background

The harness of the ten-fold test turns phase ledgers and receipts into ratios, intervals and verdicts. A harness
that computes the wrong ratio would misstate the program's central result, so it has its own known answers:
small hand-checkable cases with exact values, each paired with the wrong computation it guards against. The gate
GATE-TENFOLD-03 requires the harness to reproduce every one exactly and to catch every negative control before a
sealed suite is opened (M3, repeated at M7); GATE-TENFOLD-10 requires the pre-analysis rows before the partial
gate. This bead is the registry: the rows as data, with derivations. The printed known-answer appendix of the
program has no rows filed under this chapter (its only related rows are KA-009, the completion cap, and KA-014,
the general speed-up bound, filed under the thesis chapter); the rows below are appended to the registry and
receive KA numbers on append (numbers are never reused or renumbered). Until then they are cited by the local
labels `TF-01` to `TF-22`.

Symbols. H: the stratum deadline. T_A, T_B: clock times of arms A (this system) and B (the rival); "c" marks a
censored run, charged H. R̂ = Σ min(T_B, H) ÷ Σ min(T_A, H). ρ: a per-family ratio. S: build cost of a standing
asset. ā, b̄: pooled restricted mean time per amendment. R_warm(N) = (S_B + N·b̄) ÷ (S_A + N·ā). N*: the smallest
N with S_A + N·ā ≤ S_B + N·b̄. r_T = (T_B − T_W)/(T_B − T_F): recovery ratio of the composed incumbent W.

## Scope

In: the rows, their derivation files, their fixture files (ledger records for the cards), and their entry in the
registry with tier `G1 · host` and tolerance. Out: the test target that runs them (N39b-22); the negative-control
records (N39-CX); the thesis rows KA-009 to KA-016 (the thesis registry).

## Technical approach

Each row becomes one record of `known_answers.toml` (id, section, case, truth as an exact rational where one
exists, printed value, tolerance, tier, control ids) and one file `docs/derivations/tenfold/<label>.md`. Fixture
inputs live in `crates/frob-conformance/fixtures/tenfold/` as `frob.tenfold.ledger/1` records (labelled
`synthetic`). Exact rows are compared as rationals; printed decimals are checked to the digits shown.

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
| TF-05 | Fixture A pair ratio | 9,000 ÷ 1,500 = 6 | exact | — |
| TF-06 | Fixture A, training removed | Share 4,920/9,000 = 54.67%; ceiling 9,000 ÷ 4,080 = 75/34 = 2.206 | exact | A ceiling computed on the wrong arm's ledger (arm A's own train share is 10%, giving 1.11) |
| TF-07 | Fixture A, unremovable floor | `author` + `propose` + `adjudicate` of arm A = 1,050 s = 11.67% of the rival's clock; cap 9,000 ÷ 1,050 = 60/7 = 8.571 | exact | Ignoring cost no mechanism removes |
| TF-08 | Fixture A, a 2.67% phase | `check` is 240/9,000; removing it gives at most 1/(1 − 2/75) = 75/73 = 1.027; not fundable as a speed lever (under 10%, where the cap is 1/0.9 = 1.11) | exact | Funding a lever with no removable share |
| TF-09 | Fixture C warm ratios | R_warm(1) = 4,050/14,870 = 0.272; R_warm(10) = 40,500/19,100 = 2.120; R_warm(100) = 405,000/61,400 = 6.596; limit 4,050/470 = 8.617 | exact; decimals to three places | Quoting the limit as the warm ratio; omitting a standing asset |
| TF-10 | Fixture C crossover | N* = 5 (14,400/3,580 = 4.02); warm verdict taken at N = 10 where the class measures 2.12 | exact | Summarizing a class by its limit |
| TF-11 | Exact guard, critical counts at α = 0.05 | 5 of 5, 6 of 6, 7 of 8, 10 of 12, 12 of 16, 17 of 24, 22 of 32; P(X ≥ 17 at 24) = 0.03196, P(X ≥ 16 at 24) = 0.07579 | exact (binomial arithmetic) | An approximate test |
| TF-12 | Guard under Holm with four cells | 18 of 24 at 0.0125 (tail 0.01133) | exact | An unadjusted guard on several cells |
| TF-13 | Guard at m = 4 | 4 of 4 has tail 1/16 = 0.0625: no count passes; a four-family confirmatory cell is refused | exact | A verdict that cannot be reached |
| TF-14 | Zero-count bound | 0 misses in 24 independent families: one-sided 95% upper bound 1 − 0.05^(1/24) = 0.1174 (at 12: 0.2209; at 32: 0.0894; at 46: 0.0630) | 10⁻⁴ | A zero-width interval from 24 zeros |
| TF-15 | Completion cap | 12% unfinished caps R at 25/3 = 8.33 (KA-009) | exact | A ratio claimed above its ceiling |
| TF-16 | Monitor | E multiplies by 1.5 for each family below 3 and by 0.5 otherwise; 1.5⁸ = 25.63 ≥ 20 stops, 1.5⁷ = 17.09 does not | exact | An early stop without anytime validity |
| TF-17 | Two estimands, 25 serial missions, H = 100 s | Rival 50 s each (1,250 s, 25 accepted); candidate 24 at 1 s and one timeout (124 s, 24 accepted): R_time = 625/62 = 10.081; R_rate = (24/124) ÷ (25/1,250) = 300/31 = 9.677 | exact | Citing a rate ratio as the time ratio |
| TF-18 | Rare tail, H = 1,000 s | Rival always 20 s; candidate 1 s on 99% of families, 999 s on 1%: population ratio 20/10.98 = 1.8215; P(a 24-family suite has no slow family) = 0.99²⁴ = 0.78568, and then every percentile-bootstrap resample reports 20 | 10⁻⁴ and 10⁻⁵ | A percentile bootstrap used as a population certificate |
| TF-19 | Family counts for a population claim | n ≥ ln(1/α)/ln(1.1): 32 at 0.05, 46 at 0.0125, 49 at 0.01; the Hoeffding bound X̄ − 11·H·√(ln(1/α)/(2n)) needs n ≥ 121·ln(1/α)/2 = 182 at 0.05 (nonnegative at 182, negative at 181, on the all-fast sample) | exact integers | A population claim from 24 families |
| TF-20 | Rare-tail campaigns | 2,000 simulated campaigns of 24 families: the contrast test certifies ten-fold in 0; the percentile bootstrap certifies it in 78.6% ± 3 points | counts; ± 3 points | The same |
| TF-21 | Recovery identity | If T_F = 0.10·T_B then r_T ≥ 0.8 exactly when T_W ≤ 0.28·T_B; T_F ≥ T_B gives "no saving to recover" (r undefined); T_W < T_F gives r > 1 | exact | Dividing by a non-positive saving |
| TF-22 | Ablation bounds | Unremoved training share s = 0.7 caps an ablation at 1/s = 1.43; removing it entirely caps at 1/(1 − s) = 3.33. 50 designs at 35 min with 1.5 min of agent time each: training 1,750 of 1,825 min = 95.9%, information alone capped at 1.04; with acquisition at 45 s per design, agent time is 75 of 112.5 min = 2/3 | exact | An ablation ratio above its accounting bound |

Planning arithmetic also checked by the registry (not gate rows): a cell of 24 families, three arms and H = 4 h
costs at most 288 device-hours; at restricted mean times of 1.5 h, 1.0 h and 0.4 h it costs 24 × 2.9 = 69.6.

## Acceptance criteria

- [ ] 22 records exist in the registry with derivation files and fixture files; each names at least one control.
- [ ] No row has tolerance `to set` or control `none stated`.
- [ ] Every exact row is stored as a rational and compared as one.
- [ ] The registry loader refuses a row whose fixture digest differs from the recorded digest.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry::tenfold_rows`: `rows_load_and_are_unique`; `every_row_has_control`;
`derivations_exist`; `fixture_b_sums` (54,000 and 20,400 recomputed from the fixture file);
`binomial_tails_exact` (integer arithmetic over 2²⁴ and 2³²).

### End-to-end script and logging

None of its own: the rows are exercised by `scripts/e2e/tenfold_known_answers.sh` (N39b-22), which logs each row
as an assertion with the `--ka` tag (the local label until a number is assigned).

### Known answers and negative controls

The table. Negative control for the registry itself: a planted edit of fixture B (family 5 changed to 1,440 s)
must be refused by the digest check, not silently produce R̂ = 54,000 ÷ 7,440.

## Considerations

- The rows are harness arithmetic, not results; every value is illustrative of the computation.
- The illustrative power table of the decision rule (lognormal times, 20,000 simulated gates per entry) is owned
  by the statistics beads and is a planning value, not a registry truth.
- TF-15 duplicates KA-009 by design: one fixture identity, two rows (host mathematics and harness).

## References

§39.16, §39.17, §39.20, §39.22; GATE-TENFOLD-03, -10; KA-009, KA-014; CX-003, CX-008.
