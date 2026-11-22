---
key: "NEG-09"
title: "discrepancy E7-6 resolved: every-thirteenth-step scheduling puts 616 decision instants in 20 s, against 600"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sensorium", "frob-sense", "solid", "test", "card"]
parent: "WP-40"
deps: ["NEG-03", "GB-23", "BD-078"]
needs: []
related: ["N13-06", "N20a-44", "GB-40"]
gates: []
plan_refs: ["Appendix E.7 row 6", "§20.4.6", "§20.20", "Appendix M.38 row 6", "Appendix M KA-471"]
---
## Background

A 30 Hz policy against 400 Hz physics has a period of 40/3 physics steps. The shortcut "act every thirteenth
step" runs the policy at 400/13 = 30.77 Hz. The sensing chapter first said this gives "615 decisions in 20 s
instead of 600". 615.4 is the rate times the duration; it is not a count. The number of decision instants in
the half-open window [0, 20) s is the number of multiples of 13 below 8,000 steps: ⌈8,000/13⌉ = 616. The rate
error is 2.56% (printed 2.6%); on counts the overpayment is 16/600 = 2.67% (printed 2.7%). The specification
and the card now print 616. The slip is small and the kind of slip matters: a count over a window depends on
whether the window is half-open and on whether the instant at t = 0 is included, and the project's convention
(windows are half-open, an event at exactly the closing instant belongs to the next window) must be applied
by one function, not re-derived in prose.

## Scope

In: the counting function and its convention; the regression test; the scan. Out: the clock contract (BD-078);
the conventions battery for time rows (N13-06); the card (GB-23).

## Technical approach

1. **Convention (binding).** A window is half-open: [a, b) for instants, (a, b] for exposures that integrate
   up to and including their closing instant, as the conventions state for each stream kind. Decision instants
   are counted on [0, T).
2. **One function.** `frob-sense` exposes `count_firings(period_quanta, phase_quanta, window_start,
   window_end) -> u64` in integer quanta, = number of n in [start, end) with n ≡ phase (mod period). Every
   report that prints "k decisions in T seconds" calls it.
3. **Values (exact).** Q = 1,200 quanta per second; policy period 40 quanta; window 24,000 quanta: 600
   firings. Shortcut: period 13 steps = 39 quanta; firings at 0, 39, …, below 24,000: ⌈24,000/39⌉ = 616. Rate
   400/13 = 30.769 Hz, 2.564% above 30 Hz. Count ratio 616/600 = 1.02667. Rate × duration = 615.38, which is
   not an integer and is never printed as a count.
4. **Closed window check.** On [0, 20] (closed) the exact schedule has 601 instants and the shortcut still 616
   (7,995 is the last multiple of 13 at or below 8,000); the test records both so that the convention's
   effect is explicit.
5. **Register and scan.** Item E7-6: stale pattern `615` in the scope of decision, thirteenth, 30 Hz or 20 s
   text; pinned by `gb_23_reference` and the tests below.

## Acceptance criteria

- [ ] `count_firings(40, 0, 0, 24000) = 600` and `count_firings(39, 0, 0, 24000) = 616`.
- [ ] Reports print counts only from the function; a float rate × duration formatted as a count is rejected by
      the report lint (a count field must be an integer type in every schema).
- [ ] The stale-value scan finds no hit for E7-6.

## Test plan

### Unit tests

- `count_firings_exact_schedule`: 600 on [0, 20); 601 on [0, 20].
- `count_firings_shortcut`: 616 on [0, 20) and on [0, 20].
- `count_firings_property`: for random periods, phases and windows the function equals a brute-force loop
  (10⁴ cases).
- `count_is_not_rate_times_duration`: ⌊615.38⌋ = 615 and round(615.38) = 615 both differ from 616.
- `overpayment_figures`: 30.769/30 − 1 = 0.02564; 616/600 − 1 = 0.02667.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) through GB-23's surface row (`frob sense schedule … --horizon 20
--json`, field `decision_instants = 600`), and the stale-value step for item E7-6; log lines carry Q, the
periods in quanta, the window in quanta and the count.

### Known answers and negative controls

KA-471 (400 Hz physics and a 30 Hz policy under `hold`: decisions at quanta 0, 40, 80, 120 realized at steps
0, 14, 27, 40). Controls: the shortcut (616) and the mis-count (615). Determinism: integer arithmetic, D0.

## Considerations

- The same function serves sensor packets, reward windows and telemetry decimation; a second counting rule
  anywhere is a defect.
- The unreconciled-values list of the known-answer suite records the same correction (rate error 2.6%, count
  overpayment 2.7%).

## References

Appendix E.7 row 6; §20.4.6; §20.20; Appendix M.38 row 6; KA-471.
