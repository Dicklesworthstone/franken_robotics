---
key: "N39-CX"
title: "frob-conformance: the counterexample and negative-control registry of the ten-fold harness"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tenfold", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-60"
deps: ["BD-025", "BD-146", "STD-TEST"]
needs: ["§38.11: the negative-control registry `frob-conformance/registry/controls.toml` with its record format (id, seed, gate, kind, target, defect, expect, last_seen) and its five rules"]
related: ["N39-KA", "N39-REF", "N39b-22", "N39b-01", "N39b-02", "N39b-36", "N00-CX"]
gates: ["GATE-TENFOLD-03"]
plan_refs: ["§39.17", "§39.16", "§39.20", "§39.9", "§39.14", "§39.21", "§39.22", "Appendix N.3", "§38.11"]
---
## Background

A gate that nothing can fail validates nothing. Every gate of the ten-fold test therefore has planted defects it
must catch, registered before the passing test is written. This bead is that registry for the harness: the
tempting shortcut, the smallest case with its numbers, and the rule the case protects. The program's
counterexample catalogue files two entries that name this chapter under the thesis chapter (CX-003, the
per-brief point-estimate rule; CX-008, reporting speed on the missions that finished); the remaining entries
below are new, are appended to the catalogue and the control registry, and receive CX and NC numbers on append.
Until then they carry the local labels `TF-CX-01` to `TF-CX-24`. A control is a defect, not a boundary: each
entry names the build or input in which the rule was removed.

## Scope

In: the entries, their records in `controls.toml` (kind `mutation`, `planted-input` or `a-a`), the gates each must
be caught by, and the planted scorers and inputs as files. Out: running them (N39b-22 and the gate beads).

## Technical approach

Planted scorers are compiled variants of the scoring module selected by a test-only feature; planted inputs are
fixture directories under `crates/frob-conformance/fixtures/tenfold/controls/`. Each record states `expect`: the
wrong value the defect produces and the observable by which the gate fails.

| Label | Tempting shortcut | Smallest case | Rule protected | Gate |
|---|---|---|---|---|
| TF-CX-01 | Average the per-family ratios | The six-family card: mean of 6, 7, 6.857, 7.714, 0.8125, 9 is 6.23; truth 2.647 | The estimate is a ratio of sums over pairs | -03 |
| TF-CX-02 | Drop the run that never finished | The same card without family 5: 42,300 ÷ 6,000 = 7.05; truth 2.647 | No run is dropped; a censored run contributes H (INV-TENFOLD-05) | -03 |
| TF-CX-03 | Compute a ceiling on our own ledger | Single pair: arm A's train share is 10% (ceiling 1.11); the rival's is 54.67% (2.206) | Ceilings are arithmetic on the rival's closed ledger | -03, -09 |
| TF-CX-04 | Quote the warm limit as the warm ratio | Warm card: limit 8.617 against R_warm(10) = 2.120 | The warm verdict is taken at N = 10 and never quoted without S and N* | -03 |
| TF-CX-05 | Leave out a standing asset | Warm card with S_A = 0: 8.617 at every N, N* = 1; truth N* = 5 | Standing assets are declared with their build cost | -03 |
| TF-CX-06 | Use a normal approximation for the guard | Without a continuity correction it passes 9 of 12 (z = 1.732, tail 0.0416) where the exact tail is 299/4,096 = 0.0730 and 10 of 12 is needed; and 21 of 32 (z = 1.768, tail 0.0385) where the exact tail is 0.0551 and 22 is needed | The guard is exact binomial arithmetic at fixed m | -03 |
| TF-CX-07 | Open a confirmatory cell with four families | 4 of 4 has tail 0.0625 > 0.05: the word is unreachable | A cell that cannot reach its verdicts is refused or opened descriptive | -03, -04 |
| TF-CX-08 | Bootstrap 24 zeros | Zero misses in 24 families: bootstrap width 0; valid bound 0.1174 | Rare failures get an interval valid at zero counts | -03 |
| TF-CX-09 | Stop the fixed-n study when the monitor stops | Ask for the fixed-n verdict at the monitor's stopping time | The two study types are never mixed (INV-TENFOLD-07) | -03 |
| TF-CX-10 | Stop for futility on a running p-value | Seven consecutive families below 3: E = 17.09 < 20; a mutant that stops there has no anytime validity | The monitor stops at E ≥ 20 only (eight: 25.63) | -03 |
| TF-CX-11 | Accept a certificate because the builders' checker did | A planted false certificate: accepted by its declared checker, contradicted by direct evaluation at a sealed probe point | One false certificate disqualifies the arm from the scoped claim | -03 |
| TF-CX-12 | Treat a sealed value in a transcript as harmless | A canary value planted in a sealed world appears in an arm's transcript | Any leak voids the touched cells and retires the families | -03 |
| TF-CX-13 | Keep a run whose ledger does not close | A ledger with 3% unattributed | Flagged `ledger-open`, kept in the ratio, excluded from ceilings (INV-TENFOLD-04) | -03 |
| TF-CX-14 | Freeze after seeing the suite | A manifest whose freeze time follows the suite opening | No paired run without all arms frozen and published first (INV-TENFOLD-03) | -03, -04 |
| TF-CX-15 | Tune against the judge | An arm whose logs show the adjudicating simulator during tuning | Use of the adjudicating simulator in development disqualifies | -03, -05 |
| TF-CX-16 | Certify a population ratio by resampling | Rare-tail population (true ratio 1.8215): the percentile bootstrap certifies ten-fold in 78.6% of 24-family suites; the contrast test in none | A population claim rests on a valid bound and enough families | -10 |
| TF-CX-17 | Cite decisions per hour as the time ratio | 25-mission case: R_rate = 9.677 where R_time = 10.081 | The word attaches to R_time only | -10 |
| TF-CX-18 | Count missions, not families | 24 missions on one site entered as 24 units | The family is the unit of inference; 24 rows from one site are one replicate | -10, -12 |
| TF-CX-19 | Build a kit from a private scene | A kit input carrying the mark `private` | A kit with a private or non-redistributable input is refused | -08 |
| TF-CX-20 | Retune after the audit | An arm's pack digest changes after sign-off | A changed digest voids the arm | -11 |
| TF-CX-21 | Keep a verdict the truth withdraws | An amendment sequence with one missed withdrawal | An arm keeps its scoped claim only with zero missed withdrawals | -12 |
| TF-CX-22 | Report an ablation above its bound | An information-only replay reporting 1.6 where unremoved training is 70% (bound 1.43) | A ratio above 1/s or 1/(1 − s) voids the stratum as an accounting error | -13 |
| TF-CX-23 | Award the word against the declared incumbent alone | F against B gives 12 while the composed incumbent W is faster than B and F against W gives 4 | The word is governed by F against the stronger of B and W | -13, -06 |
| TF-CX-24 | Patch a frozen arm mid-gate | A defect fixed in arm A after the suite opened | The affected cells are void and re-run at the next gate | -03 |

Catalogue entries reused: **CX-003** (declare ten-fold when the point estimate is at least 10 on every brief: that
rule confirmed a true ratio of 10 in at most 18% of simulated campaigns; protects the pooled rule and its
pre-analysis) and **CX-008** (report speed on the missions that finished: with a fraction q of deadlines missed
R ≤ 1/q, so 12% unfinished caps the ratio at 25/3; protects restricted mean time).

TF-CX-06 is planted as a mutant scorer using the uncorrected normal tail; it must be caught on the two rows where
its critical count differs from the exact one (m = 12 and m = 32). With a continuity correction the approximation
gives the exact critical counts for every m in the guard table, so only the uncorrected form is a usable control.

## Acceptance criteria

- [ ] 24 new records plus CX-003 and CX-008 are in `controls.toml`, each attached to its gates and each with an
      `expect` value computed in a derivation file.
- [ ] Each of the seven controls the chapter names (TF-CX-11, -12, -13, -14, -07, -09, -15) is attached to
      GATE-TENFOLD-03.
- [ ] No gate of the ten-fold test has an empty control list.
- [ ] Every control has `last_seen.caught = true` from a recorded run before its gate records a pass.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry::tenfold_controls`: `controls_load`; `each_gate_has_control`;
`planted_scorers_compile_and_differ` (each mutant scorer returns its `expect` value on its case: 6.23, 7.05,
8.617, N* = 1); `planted_inputs_have_digests`.

### End-to-end script and logging

Exercised by `scripts/e2e/tenfold_known_answers.sh` (N39b-22) through `e2e_expect_caught`, one line per control
with `caught: true|false`; a control not caught fails the script.

### Known answers and negative controls

The table is the content. The registry's own control: a record whose `expect` equals the true value (a control
that plants nothing) is rejected by `planted_scorers_compile_and_differ`.

## Considerations

- A control that has never failed its gate in any recorded run is reviewed at each milestone exit.
- A false certificate or leak found in real use becomes a permanent control with its counterexample.
- Planted scorers exist only under a test feature and can never be linked into the kit's scorer.

## References

§39.17, §39.16, §39.20, §39.9, §39.14, §39.21, §39.22; GATE-TENFOLD-03, -08, -10 to -13; CX-003, CX-008;
INV-TENFOLD-03, -04, -05, -07.
