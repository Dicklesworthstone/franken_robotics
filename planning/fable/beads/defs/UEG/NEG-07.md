---
key: "NEG-07"
title: "discrepancy E7-4 resolved: referee share for the humanoid follows the 1.7 M planning rate: 0.14 to 0.71%"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "kinesis", "frob-dyn", "solid", "test", "card"]
parent: "WP-50"
deps: ["GB-16", "N16b-26", "N36-01", "NEG-03"]
needs: []
related: []
gates: []
plan_refs: ["Appendix E.7 row 4", "§16.11", "§36.9", "§36.3 row 7", "Appendix M KA-361, KA-821"]
---
## Background

The share of a GPU run that a background referee can re-simulate is c·ρ_rt/(T·h): cores times real-time factor
over world-seconds produced per second. The dynamics chapter illustrated it with a humanoid batch at 0.5 M
world-steps/s (1,000 world-seconds per second, a share of 0.5 to 2.4%). That rate is not a row of the
performance programme's planning table, whose humanoid row is 1.7 M world-steps/s (3,400 world-seconds per
second, a share of 0.14 to 0.71%), and the performance chapter already printed 0.14 to 0.7%. The illustration
was stale: it predated the planning table. The order-of-magnitude statement "0.1 to 2%" covers both, but two
chapters gave different numbers for the same quantity. The card follows the planning table. The root cause is
structural, a rate typed into prose instead of read from the one table that owns it, so the resolution is to
make every coverage figure derive from that table.

## Scope

In: the single source for planning rates in coverage arithmetic; the regression test; the scan. Out: the
coverage model in `sim.estimate` (N16b-26); the target register that holds the planning rates (N36-01, a
dependency); the background referee (BD-214); the card (GB-16).

## Technical approach

1. **Single source.** The planning rates of the reference workloads live in one registry table, the target
   register `registries/perf_targets.toml` of N36-01, keyed by workload and tier; the workload records
   themselves are `registries/workloads.toml` (N36-02). The coverage model of `sim.estimate` takes a workload
   key and reads T from that row (or from the device's measured rate once one exists), never a literal rate.
   The number read is the one in the row's binding (tolerance) column while no incumbent has been measured:
   for row 7 (H, physics only, `gpu-cuda`) the tolerance's planning figure, 1.7 M world-steps/s exactly; for
   row 1 (Q, physics only, `gpu-cuda`) the interim floor, 2.0 M, which N36-01's `threshold(row, None)` returns
   with status `interim-floor`. It is not the row's ambition planning value (2.55 M for H; 4.0 M for Q, which is
   what `planning_value(row 1)` returns): those give 0.09 to 0.47% and 0.06 to 0.3%, the ambition case the card
   prints beside the others, and the estimate labels which figure it used. A unit test fails if a numeric rate
   appears as a literal in the coverage code path.
2. **Values (from the table's TARGETs; h = 2 ms; 24 cores at 0.2 to 1.0 of real time).** Humanoid 1.7 M:
   3,400 world-seconds per second, 4.8/3,400 = 0.1412% to 24/3,400 = 0.7059%. Quadruped 2.0 M: 4,000, 0.12% to
   0.6%. The stale 0.5 M gives 1,000 and 0.48% to 2.4%.
3. **Receipts.** Every `fidelity.shadow` receipt prints the measured share in BD-214's block `referee_coverage`
   (`share` = `shadowed_world_seconds` / `gpu_world_seconds`: world-seconds actually re-simulated over
   world-seconds produced, so a dropped snapshot lowers it); the planning value appears only in `sim.estimate` and
   is labelled TARGET.
4. **Register and scan.** Item E7-4: stale patterns `0.5 M world-steps` and `0.5 to 2.4%`, each in the scope
   of referee, shadow or coverage text (the engine's entry form, N38b-50, searches no value without a scope;
   NEG-03 writes both entries); pinned by `gb_16_reference` and the test below. The planted wrong golden of
   GB-16's card file states the stale figure on purpose and carries `stale-as-control: E7-4`.

## Acceptance criteria

- [ ] `sim.estimate` derives the humanoid share from the table row and returns 0.0014 to 0.0071.
- [ ] No literal throughput appears in the coverage code path (lint).
- [ ] Changing the table row in a scratch registry changes the estimate accordingly (the estimate is not a
      cached constant).
- [ ] The stale-value scan finds no hit for E7-4.

## Test plan

### Unit tests

- `coverage_from_table_humanoid`: 4.8/3,400 and 24/3,400 as rationals (T = 1.7 M, row 7's tolerance figure).
- `coverage_from_table_quadruped`: 4.8/4,000 and 24/4,000 (T = 2.0 M, row 1's interim floor); reading
  `planning_value(row 1)` = 4.0 M instead gives 4.8/8,000 and must differ.
- `coverage_tracks_table`: a scratch table with the humanoid at 2.55 M yields 0.0941% to 0.4706%.
- `coverage_no_literal_rates`: source lint.
- `stale_rate_differs`: the 0.5 M inputs give 0.48 to 2.4%, asserted different from the table-driven result.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) through GB-16's surface row (`frob sim estimate <humanoid> --tier
gpu-cuda --worlds 8192 --json` on the reference host, c = 24 background cores; the verb has no flag for the
core count), which logs the target-register row used, its tag (TARGET), the share range, and item E7-4 in the
stale-value step.

### Known answers and negative controls

KA-361 and KA-821 (3,400 and 4,000 world-seconds per second; 0.14 to 0.7% for the humanoid). Control: the
stale rate. Determinism: rational arithmetic, D0.

## Considerations

- All of these rates are TARGETs until the M2 performance rows are measured; the measured share on receipts
  supersedes them without changing this bead's rule.
- A faster engine makes the share smaller; the test `coverage_tracks_table` doubles as documentation of that
  direction.

## References

Appendix E.7 row 4; §16.11; §36.9; §36.3 row 7; KA-361, KA-821.
