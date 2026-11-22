---
key: "NEG-08"
title: "discrepancy E7-5 resolved: the six-survivor ladder costs 2.0 h, with the audit lane reported separately"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "atelier", "frob-opt", "solid", "test", "card"]
parent: "WP-62"
deps: ["NEG-03", "GB-35", "BD-329"]
needs: []
related: ["BD-352", "N28a-25"]
gates: []
plan_refs: ["Appendix E.7 row 5", "§28.9 (the ceiling)", "Appendix E.4 GB-35"]
---
## Background

The ladder's benefit is illustrated on 81 designs with rungs of 10 s, 3 min and 30 min: training all costs
40.5 h; successive halving at η = 3 costs 6.1 h; with a calibrated survivor margin that lets six designs
through the cut, of which three are trained, the cost is lower still. The chapter first printed "about 2.6 h
with the audit lane" for that last case. The stated inputs do not determine 2.6 h: the ladder itself costs
81 × 10 s + 6 × 3 min + 3 × 30 min = 121.5 min = 2.0 h; adding a tenth for the audit gives 2.2 h; adding the 19
audit designs scored at the second rung gives 2.0 h + 57 min = 3.0 h. No reading of the inputs gives 2.6 h. The
specification now prints 2.0 h and 2.2 h with a tenth added, and the card prints 2.0 h and the audit lane
separately. The lesson recorded here: a total that folds in an unstated component cannot be checked, so cost
reports itemize.

## Scope

In: the itemized cost model for a ladder plan; the regression test; the scan. Out: the calibrated layer
(BD-329); the cascade that consumes the plan (BD-352); the card (GB-35).

## Technical approach

1. **Itemized plan cost (DESIGN).** The ladder planner returns a cost record with one line per component and
   no pre-summed figure that mixes them:
   `rung_costs` (per rung: count × unit time), `audit_lane` (count × rung time, with the rung named),
   `calibration_overhead` (if any), `total` (the sum, computed, with its formula echoed).
2. **Values (exact minutes).** All trained: 81 × 30 = 2,430 min = 40.5 h. Halving at η = 3:
   13.5 + 81 + 270 = 364.5 min = 6.075 h. Six survivors, three trained: 13.5 + 18 + 90 = 121.5 min = 2.025 h.
   Audit lane, 19 designs at the second rung: 57 min; total 178.5 min = 2.975 h. A tenth added to the ladder:
   133.65 min = 2.2275 h.
3. **Report rule.** A ladder cost in a report is printed as "2.0 h + audit 57 min", or as the total with its
   components beside it. A bare total is rejected by the report lint when the plan has an audit lane.
4. **Register and scan.** Item E7-5: stale pattern `2.6 h` in the scope of ladder, survivor or audit-lane
   text; pinned by `gb_35_reference_ladder_costs` and the test below.

## Acceptance criteria

- [ ] The planner's cost record for the card's inputs has the four lines above and `total = 178.5 min` with
      the audit lane, `121.5 min` without.
- [ ] The report lint rejects a bare "2.6 h" and a bare total that hides an audit lane.
- [ ] The stale-value scan finds no hit for E7-5.

## Test plan

### Unit tests

- `ladder_cost_all_trained`, `ladder_cost_halving`, `ladder_cost_six_survivors`: 2,430; 364.5; 121.5 minutes.
- `ladder_cost_audit_lane_itemized`: 57 minutes as its own line; total 178.5.
- `ladder_cost_no_reading_gives_156`: none of {121.5, 133.65, 178.5} equals 156 min (2.6 h): the stale figure
  is not derivable from the inputs under any of the three readings.
- `ladder_cost_ratio`: halving over ladder = 364.5/121.5 = 3 exactly ("a factor of three over halving").
- `report_lint_requires_itemized_audit`.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) through GB-35 (the cascade planner's `--plan-only` output where
available): logs each cost line in minutes, the total and its formula, and item E7-5 in the stale-value step.

### Known answers and negative controls

The cost arithmetic above (exact). Control: the unitemized 2.6 h. Determinism: integer arithmetic in tenths
of a minute, D0.

## Considerations

- The audit lane's cost depends on which rung the audit designs are scored at; the record names the rung so
  that a different choice is visible rather than silently changing the total.
- These are serial sums of rung times; wall-clock under parallel devices is a different figure and carries a
  different label in the plan.

## References

Appendix E.7 row 5; §28.9; GB-35.
