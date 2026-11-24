---
key: "NEG-09"
title: "discrepancy E7-6 resolved: every-thirteenth-step scheduling puts 616 decision instants in 20 s, against 600"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sensorium", "frob-sense", "solid", "test", "card"]
parent: "WP-40"
deps: ["BD-078", "GB-23", "NEG-03"]
needs: []
related: ["N20a-06", "N20a-49"]
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
the conventions battery for time rows (N13-06); the card (GB-23); the reconciliation record REC-06 of the
known-answer suite, with the corrected control text of the two known-answer rows that use the shortcut, of the
clock re-phasing relation and the derivation file `docs/derivations/clock-thirteenth-step.md` (N38b-55, which
records the same correction in the known-answer registry and whose row checks print counts through the function
built here).

## Technical approach

1. **Convention (binding).** A window of instants is half-open, [a, b): decision instants are counted on [0, T),
   and an event at exactly the closing instant belongs to the next window. Exposure windows are not ruled here:
   the conventions chapter writes them (a, b] and the Sensorium chapter [open, close); the owner decision N20a-49
   rules between them, and N20a-06's `window_contains` implements the ruling (`OpensAt`, [open, close), the
   fail-closed default; `ClosesAt`, (open, close], if ruled). No count of this bead depends on that ruling.
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
5. **Register and scan.** Item E7-6, in the engine's entry form (N38b-50):
   `stale = [{ pattern = "\\b615\\b", scope = "decision|thirteenth|30 Hz|20 s" }]`, linked to REC-06 (N38b-55,
   whose retired entry is `615` in decision, thirteenth or 13-steps text); pinned by `gb_23_reference` and the
   tests below. The pattern also matches 615.38, and the engine reads no label, so every line in the walk set
   that prints 615 or 615.38 on purpose carries `stale-as-control: E7-6`: the lines of
   `count_is_not_rate_times_duration` and the mis-count control of GB-23's card file.
6. **Report lint (DESIGN: the rule is by type and by name, so that it is decidable).** Two checks, both in
   `frob-conformance`, which sees every crate and every registered schema. (a) By type: the only formatter that
   prints "k <events> in T seconds" takes its count as the `u64` that `count_firings` returns; a float cannot
   be passed, and a source scan finds no other formatting of such a phrase in a report template. (b) By name:
   in every registered output schema a property named `count`, or whose name ends in `_count`, `_firings`,
   `_decisions` or `_instants`, has JSON type `integer`, never `number`; the lint runs over the schemas
   present in the build, so it grows with them. A rate times a duration is a `number` and cannot enter either.

## Acceptance criteria

- [ ] `count_firings(40, 0, 0, 24000) = 600` and `count_firings(39, 0, 0, 24000) = 616`.
- [ ] Reports print counts only from the function; a float rate × duration formatted as a count is rejected by
      the report lint (a count field must be an integer type in every schema): `count_fields_are_integer`
      passes on the registered schemas and fails on the planted schema below.
- [ ] The stale-value scan finds no hit for E7-6.

## Test plan

### Unit tests

- `count_firings_exact_schedule`: 600 on [0, 20); 601 on [0, 20].
- `count_firings_shortcut`: 616 on [0, 20) and on [0, 20].
- `count_firings_property`: for random periods, phases and windows the function equals a brute-force loop
  (10⁴ cases).
- `count_is_not_rate_times_duration`: ⌊615.38⌋ = 615 and round(615.38) = 615 both differ from 616.
- `overpayment_figures`: 30.769/30 − 1 = 0.02564; 616/600 − 1 = 0.02667.
- `count_fields_are_integer`: the lint of item 6 over every registered output schema; a planted `synthetic`
  schema whose property `decision_count` is declared `number` (the place a 615.38 would be written) fails it,
  naming the schema and the property; `report_count_only_from_function`: the source scan of item 6 (a) finds
  one formatter and no second one.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) through GB-23's surface rows (`frob sense schedule` on the explicit
clock list with a 20 s window, BD-078's form: the policy events of `events[]`, all pages, number exactly 600,
and the count the script prints is taken from the reply, never computed as rate × duration), and the
stale-value step for item E7-6; log lines carry the exact command line, exit code, Q, the periods in quanta,
the window in quanta and the count.

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
