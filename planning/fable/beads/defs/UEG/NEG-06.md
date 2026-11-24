---
key: "NEG-06"
title: "discrepancy E7-3 resolved: the zero-cost-solver ceiling is 842.8 ÷ 575.7 = 1.46; the 94% share is withdrawn"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tenfold", "augur", "frob-conformance", "solid", "test", "card"]
parent: "WP-34"
deps: ["BD-043", "N00-17", "NEG-03"]
needs: []
related: []
gates: []
plan_refs: ["Appendix E.7 row 3", "§36.5", "§2.5", "§25.22", "Appendix M.38 row 1", "Appendix H.5 (timing ceiling)"]
---
## Background

At plan time a specialized mission solver was timed against a composed general one on ten fixtures. The median
time ratio was 1.345 warm and 1.088 cold. To show that the test could not have found a large speed-up whatever
the solver did, the text stated a ceiling: "about 94% of the batch was shared work, so a solver of zero cost
would have reached about 1.46". The two halves do not agree: 1.345/0.94 = 1.43, not 1.46. The 1.46 is correct
and comes from different numbers: the composed batch's median is 842.8 µs and the specialized batch holds a
residual of 575.7 µs that the solver does not touch (the 610.1 µs full median less the 34.4 µs solve-only
median: parsing, assembly, checking, serialization and harness, not separately timed), so a zero-cost solver
would reach 842.8 ÷ 575.7 = 1.464. The "94% shared" attribution was an inference about where the residual went
that nothing measured; it is withdrawn. The conclusion (the test had no power to show a large speed-up) is
unchanged. The specification and the card now print the ceiling that way.

## Scope

In: the corrected derivation as a regression test on the archived medians; the rule that a ceiling is
derived from timed quantities only, with the calculator's refusal of a (ratio, share) pair; the two ratio
statistics stored with their definitions; the content of register item E7-3 (its stale patterns, its pinning
tests, its link to REC-01).

Out, with the bead that has it:
- the record REC-01 of the reconciliation register, the derivation file `docs/derivations/solver-ceiling.md`
  and the settled truth of the known-answer rows KA-001 and KA-812 and of CX-001 (the exact rational
  8428/5757, printed "about 1.46", tolerance 0.005): N38b-50. This bead does not set a row's truth or
  tolerance; the derivation file there states the same three medians and both ratio statistics as step 1
  below;
- the pin DX-03 of the killed-idea register, which records the same correction: NAB-41;
- the scan engine: N38b-50; the register file that holds item E7-3: NEG-03;
- the re-run of the plan-time pilots from the reproduction archive (BD-043), which this bead reads; the
  general ceiling calculator (N00-17); the ceiling-before-speed check (BD-354).

## Technical approach

1. **Derivation from timed quantities (FACT about a small plan-time program; re-run before a gate cites it).**
   Numerator 842.8 µs (read here as the composed batch's median; the archive's output file is the authority
   for what each figure denotes, and the test takes its labels from that file). Specialized batch median
   610.1 µs. Specialized solve-only median 34.4 µs. Residual = 610.1 − 34.4 = 575.7 µs. On that reading the
   ratio of batch medians is 842.8/610.1 = 1.3814, while the reported 1.345 is a median of per-fixture
   ratios, a different statistic; both are recorded with their definitions. Ceiling with a zero-cost solver:
   842.8/575.7 = 1.46396.
2. **What is withdrawn.** The statement that 94% of the batch is "shared work". 575.7/610.1 = 94.4% is the
   untouched share of the specialized batch; it is not a share of the composed batch and cannot be divided
   into 1.345.
3. **Rule.** A ceiling in any report is a ratio of two timed totals on one ledger (time before over the least
   time after), never a measured ratio divided by an inferred share. The ceiling calculator accepts a phase
   ledger and refuses a (ratio, share) pair. Concretely (DESIGN, no new formula): the ceiling is computed by
   N00-17's `residual_ceiling(total, untouched)` on two timed totals in integer microseconds, reached through
   its report `cargo run -p frob-conformance --example thesis -- ceilings --ledger <ledger> --json` (the example
   target N00-17 declares; the crate has no default binary), whose ledger reader accepts timed phases only; an
   input that supplies a measured ratio and an inferred share in place of timed totals (a planted
   `{"measured_ratio": "1.345", "shared_share": "0.94"}`) is refused by that reader's strict reading, with the
   calculator named.
4. **Regression test** on the archived timings (read from the reproduction bundle's output file by BD-043's
   loader): residual, ceiling and the two ratio statistics.
5. **Register and scan.** Item E7-3: stale patterns `1.345 ÷ 0.94`, `1.345/0.94`, "94% of the batch was
   shared"; pinned by the test below and by `gb_18_reference`; `same_as = ["REC-01"]` and
   `value = "8428/5757"`, the figure that record settles. The test `share_attribution_rejected` writes the
   withdrawn 1.4309 on a line marked `stale-as-control: E7-3`, which the scan (the engine of N38b-50) exempts.

## Acceptance criteria

- [ ] The test reproduces 575.7 µs and 1.46396 from the three archived medians.
- [ ] The calculator refuses a ceiling requested from a ratio and a share.
- [ ] Both ratio statistics (median of ratios 1.345; ratio of medians 1.3814) are stored with their
      definitions, and reports name which one they print.
- [ ] The stale-value scan finds no hit for E7-3, and item E7-3 resolves to REC-01 with the value 8428/5757.

## Test plan

### Unit tests

- `solver_ceiling_from_residual`: 610.1 − 34.4 = 575.7; 842.8/575.7 = 1.46396 (rel 10⁻⁶).
- `share_attribution_rejected`: the planted (ratio 1.345, share 0.94) input of step 3 offered to the ceiling
  report is a refusal, and the withdrawn value 1.4309 is asserted different from the ceiling.
- `ratio_statistics_distinct`: on the ten archived fixtures the median of per-fixture ratios and the ratio of
  medians are computed separately and labelled.
- `ceiling_exceeds_measured`: the ceiling is at least the measured ratio of medians (1.46 ≥ 1.38).

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) logs GB-18's FACT row with the three medians, the residual and the
ceiling, and the stale-value step logs item E7-3. Where the reproduction archive is present the script replays
its timing check (`python3 -B verify.py` inside the archive, as BD-043 specifies) and logs the freshly measured
medians beside the archived ones, labelled `re-run`, without asserting equality of timings.

### Known answers and negative controls

The known answer is the arithmetic above on recorded medians. Control: the withdrawn derivation (1.43).
Determinism: arithmetic on recorded numbers is D0; re-run timings are measurements and are never compared for
equality.

## Considerations

- The timings are FACT about a small program on a small model at plan time; they say nothing about this
  project's solver, and the ten-fold arithmetic does not rest on them.
- If the re-run finds a residual that the harness can time separately, the ceiling is recomputed from the
  split and recorded as a dated amendment.

## References

Appendix E.7 row 3; §36.5; §2.5; §25.22; Appendix M.38 row 1; Appendix H.5.
