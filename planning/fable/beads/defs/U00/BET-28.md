---
key: "BET-28"
title: "Decide BET-28: calibration bands (conformal bands reach stated coverage within one calibrated family)"
type: "task"
priority: 2
milestone: "M5"
labels: ["m5", "ground", "frob-ground", "frontier", "bet", "gate"]
parent: "WP-68"
deps: ["N00-28", "N00-07", "BD-388", "BD-393", "BD-394", "STD-E2E"]
needs: ["§42 (OPEN-GAUNT-1): which real-robot dataset, if any, is admitted to the Reality lane first; without one the Reality gate rests at `no-claim`", "§30.4: the exchangeability alarm and the omitted-law fixture of the synthetic rehearsal"]
related: ["BD-387", "BD-392", "BD-397", "BD-139", "NS-21", "N00-12", "N00-KA", "N00-CX"]
gates: ["GATE-GROUND-06", "GATE-GROUND-07", "GATE-GROUND-10", "KILL-BAND"]
plan_refs: ["§4 (BET-28)", "§40.5", "§0.8 (H-14)", "§30.22", "§8.1 (Reality)", "§43.12"]
---
## Background

**The bet.** Everywhere by default a report says `reality: no-claim`: agreement with another simulator or with
a published figure is Parity, never Reality. The Reality lane is the one way to say more. Bench measurements of
a built robot are imported with their provenance; predictions for held-out episodes are frozen before the
comparison; and for that robot and task family a *conformal band* is issued on the gap between simulation and
measurement: an interval that contains the measured value with a stated nominal coverage, under the assumption
that calibration and test episodes are exchangeable. The bet is that such bands reach their stated coverage
inside a calibrated family. Tier: Frontier.

**What the evidence supports.** Coverage holds within one calibrated robot and task family and varies by about
six points at 20 calibration episodes. Nothing supports a band for a design that was never built. A probe that
is perfect in class can be useless out of class: in a plan-time pilot a fitted probe was correct in 675 of 675
in-class cases and in 0 of 225 cases governed by a law it had not seen.

This is an **import lane**: the gate needs measurements the program does not have. Without data its resting
state is `no-claim`, which is recorded as the state and not as a pass.

## Scope

In: pre-registration, the M5 reading on the synthetic rehearsal, the reading on real data whenever a dataset is
admitted, verdict and decision record. Out: ingestion, identification, frozen predictions, the band
computation, drift detection.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. On held-out real episodes in the family, empirical coverage misses nominal by more than the finite-sample
      allowance (about six points at n = 20).
   b. The exchangeability alarm fires and a band is issued anyway.
   c. A band is issued for a design that was never built.
2. **Competing arms.** Uncalibrated simulation with randomization ranges; measuring the target directly.
   **Potency control.** A fixture with an omitted law that passes every in-family residual check and reverses
   the decision must trigger the alarm or a refusal.
3. **Deciding gates (TARGETs).**
   - GATE-GROUND-06 (Known answer, M5; on the synthetic rehearsal): mean in-cell coverage within 0.01 of 19/21
     (the exact conformal coverage at n = 20 for a 0.90 target: ⌈21 × 0.9⌉/21); the fraction of calibration
     draws with coverage below 0.80 within 0.03 of 0.069; zero bands for the shifted cell, the short cell or the
     unbuilt design.
   - GATE-GROUND-07 (Known answer, M5): a planted drift retires the calibration within 20 episodes in at least
     90% of resamples; false retirement at most 5% over 10⁴ null runs; every scope change fires.
   - GATE-GROUND-10 (Reality; when data is admitted): the first real cell: a frozen prediction scored on sealed
     data and a band issued. It blocks nothing and rests at `no-claim`.
4. **Kill gate and fallback.** KILL-BAND (M5): GATE-GROUND-06 or -07 fails. Fallback: bands are withdrawn;
   reports print `reality: no-claim` with the raw residual table.
5. **Downstream if falsified.** Ground keeps admission, clocks, frozen predictions and direct measurements; no
   band is printed anywhere; the scenario "a calibrated reality band" is demonstrated as a frozen prediction
   with its residual table. If no data ever arrives the bet is `Untested: no data` on clause (a) and decided on
   the rehearsal for (b) and (c).
6. **Procedure.** Register the rehearsal's planted parameters, the nominal level, n and the allowance before
   the rehearsal; for real data, freeze predictions (digest dated before the sealed data is opened); read at M5
   exit and again at each admitted dataset; `docs/adr/bet-28-m5.md`.

## Acceptance criteria

- [ ] Pre-registered; every frozen prediction's digest predates the opening of its sealed data.
- [ ] Rehearsal: mean coverage against 19/21 ± 0.01; low-coverage fraction against 0.069 ± 0.03; zero bands in
      the three refusal cases; drift retirement and false-retirement rates.
- [ ] The omitted-law fixture triggered the alarm or a refusal (no band issued on those cases).
- [ ] Real data, if admitted: empirical coverage with the allowance; otherwise the recorded `no-claim`.
- [ ] Verdict and the KILL-BAND decision recorded as an ADR.

## Test plan

### Unit tests
`bets.rs`: `bet28_conformal_index_19_of_21` (n = 20, target 0.90 → rank ⌈18.9⌉ = 19 of 21, coverage 0.9048);
`bet28_mean_coverage_0_902_holds`; `bet28_coverage_0_82_at_n20_falsifies_a` (misses 0.905 by more than 0.06);
`bet28_band_issued_after_alarm_falsifies_b`; `bet28_band_for_unbuilt_design_falsifies_c`;
`bet28_no_data_is_untested_no_data_not_pass`.

### End-to-end script and logging
`scripts/e2e/bet_28_decide.sh` runs the synthetic rehearsal through `frob ground ingest`, `predict`, `score` and
`band`, requests a band for an unbuilt design (expects a refusal naming the missing calibrated family), runs
the omitted-law fixture, and recomputes the verdict. Logged per STD-E2E: cell, n, nominal and empirical
coverage, alarm state, refusal codes, prediction digest and data-opening times; `summary` naming
GATE-GROUND-06, -07, -10 and KILL-BAND. Logs carry no private measurement data: only digests and aggregates.

### Known answers and negative controls
19/21 = 0.9048; the pilot counts 675 of 675 and 0 of 225 (KA-017, CX-013). Negative controls: the omitted-law
fixture; a prediction "frozen" after the data was opened; a holdout reused for a second score (refused); a
pooled shifted regime, which must be flagged.

## Considerations

- A band is a statement about one calibrated family; reports name the family and n every time.
- A Reality claim that rests on a card field marked `assumed` is refused.
- M5 is off the critical path; the ten-fold tournament does not wait for it.

## References

§4 (BET-28), §40.5, §0.8 (H-14), §30.22, §8.1; GATE-GROUND-06, -07, -10; KILL-BAND; KA-017; CX-013.
