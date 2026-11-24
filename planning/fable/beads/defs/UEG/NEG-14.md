---
key: "NEG-14"
title: "carried-card corrections E7-K1, E7-K2 pinned: 0.24 µm in a centred 8 m cell; 0.55 m exact beside 0.57 m"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "spark", "atelier", "frob-conformance", "solid", "test", "card"]
parent: "WP-47"
deps: ["BD-106", "GB-04", "GB-05", "N21b-37", "NEG-03"]
needs: []
related: []
gates: []
plan_refs: ["Appendix E.7 (corrections to carried cards)", "Appendix E.1 GB-04, GB-05", "§35.8", "Appendix M KA-792, KA-267"]
---
## Background

Seven cards were carried from an earlier edition of the specification with their identifiers. Three were
corrected when they were recomputed, and two of those corrections changed a printed number without any chapter
being wrong. They are recorded here so that the earlier figures, which exist in older notes and are easy to
copy, cannot come back.

First, the anchoring card printed 0.48 µm for the worst single-precision spacing inside an 8 m cell. That is
the spacing for an offset in [4, 8) m. With the centred offset the specification adopts, [−4, 4) m, the
largest magnitude lies in [2, 4) and the worst spacing is 2⁻²² m = 0.24 µm. The centred convention is the
decision of record (its re-anchoring shift is exact; the uncentred one's is not), so 0.48 µm describes a
rejected alternative.

Second, the sideways-cost card printed only the small-angle cost 4·√(d·b) = 0.57 m for a lateral shift of
0.10 m at half-track 0.20 m. The optimal rotation there is 0.72 rad, which is not small; the exact cost of the
same four-move manoeuvre, 2·α·b + d·cot(α/2) at cos α = 1 − d/(2b), is 0.55 m. The card now prints both.

## Scope

In: regression tests that pin the corrected figures and label the earlier ones; the register entries and the
scan. Out: the cards (GB-04, GB-05); anchored positions (N21b-37) and the maneuverability analysis (BD-106),
both dependencies because the acceptance checks the anchored type's documented resolution and the analysis's
report of both costs.

## Technical approach

1. **E7-K1 (exact).** Spacing of `f32` for magnitudes in [2^e, 2^(e+1)) is 2^(e−23). Centred offset
   |x| < 4: e ≤ 1, worst 2⁻²² m = 0.2384185791015625 µm. Uncentred offset in [4, 8): e = 2, worst 2⁻²¹ m =
   0.476837158203125 µm. The anchored-position type's documented resolution, the precision battery's stated
   floor and any report string must use 2⁻²² m. Test: enumerate the spacing at 10⁴ offsets across [−4, 4) by
   adjacent-float differences; the maximum is exactly 2⁻²² m and is attained on [2, 4) and (−4, −2].
2. **E7-K2.** d = 0.10 m, b = 0.20 m. Small-angle: α = √(d/b) = 0.7071 rad, cost 4·√(d·b) = 0.5656854249 m
   (prints 0.57). Exact: cos α = 0.75, α = 0.7227342478 rad, cost 2·α·b + d·cot(α/2) = 0.5536688302 m (prints
   0.55); ratio to the shift 5.54. The relative error of the small-angle form is 2.17% here and 0.21% at
   d = 0.01 m (0.178885 against 0.178512 m). Rule: the analysis reports the exact cost of the manoeuvre and,
   beside it, the small-angle estimate with its validity note: the small-angle form overstates the exact cost by
   under 1% while the optimal rotation α is below about 0.49 rad (d/b below about 0.235; at b = 0.20 m a shift up
   to about 0.047 m), and by 0.37% at α = 0.3 rad. The note is computed from the two forms, not hard-coded.
3. **Register and scan.** Items E7-K1 and E7-K2 of `registries/cards/discrepancies.toml`, whose `stale`
   entries NEG-03 writes at M0 in the scan engine's form (N38b-50: `{ pattern, scope, all_of }`, a 200-character
   window, no negative condition):
   `stale = [{ pattern = "\\b0\\.48 ?[µμu]m", scope = "cell|offset|anchor|spacing" }]` for E7-K1 and
   `stale = [{ pattern = "\\b0\\.57 ?m\\b", scope = "sideways|lateral|shift" }]` for E7-K2 (the leading `\b`
   keeps 10.48 µm and 10.57 m from matching). The engine cannot tell
   a labelled uncentred figure, or a small-angle 0.57 m printed beside the exact 0.55 m, from a stale one, so
   every line that prints either figure legitimately carries the marker `stale-as-control: E7-K1` or
   `stale-as-control: E7-K2` (exemption (d) of the engine): the uncentred row of `registries/cards/GB-04.toml`,
   the small-angle row of `registries/cards/GB-05.toml` (marked from GB-05's milestone, M1, since the pattern
   is scanned from M0), and any test line of this bead that prints the uncentred or small-angle figure. An
   unmarked line fires. Pinned by `gb_04_reference`, `gb_05_reference` and the tests below.

## Acceptance criteria

- [ ] The maximum spacing over the centred range is exactly 2⁻²² m; the documented resolution of the anchored
      type equals it.
- [ ] The maneuverability analysis returns 0.5536688 m for the 0.10 m shift and labels 0.5656854 m as the
      small-angle estimate.
- [ ] The stale-value scan finds no 0.48 µm and no 0.57 m in their scopes outside the lines marked
      `stale-as-control: E7-K1` or `E7-K2`; a scratch file with each figure in scope and unmarked fires once,
      and the same line marked does not.

## Test plan

### Unit tests

- `centred_cell_worst_spacing`: maximum of adjacent-float gaps over [−4, 4) is 2⁻²² m, by bits.
- `uncentred_cell_worst_spacing`: over [0, 8) it is 2⁻²¹ m, labelled as the rejected alternative.
- `sideways_cost_exact_and_small_angle`: 0.5536688302 and 0.5656854249 for d = 0.10 m; 0.1785120583 and
  0.1788854382 for d = 0.01 m.
- `small_angle_error_grows`: relative error 0.21% at d = 0.01 m and 2.17% at d = 0.10 m.
- `small_angle_validity_threshold`: at b = 0.20 m the relative error reaches 1% at d = 0.0471 m (α = 0.490 rad) and
  is 0.375% at α = 0.3 rad; the report's validity note marks d = 0.04 m inside and d = 0.05 m outside.
- `analysis_reports_both`: the report structure has `cost_exact` and `cost_small_angle` with a validity note.
- `e7k_stale_entries_marked`: the engine run over the two entries on scratch files: "worst spacing in an 8 m cell
  is 0.48 µm" and "the sideways shift of 0.10 m costs 0.57 m" each fire once; the same lines with the marker do
  not; the GB-04 and GB-05 card files raise no hit. The test's own source lines that hold those strings carry
  the marker in a comment, since `crates/*/tests/` is in the walk set.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) covers GB-04 and GB-05 and runs the stale-value step for both items;
log lines carry the spacing as hexadecimal bits and both costs with their labels.

### Known answers and negative controls

KA-792 (an offset in [−4, 4) m resolves at worst 2⁻²² m, about 0.24 µm); KA-267 (the sideways cost of the
differential drive). Controls: the uncentred spacing; the small-angle cost reported as exact. Determinism:
bit-exact for E7-K1; `acos` and `tan` within 1 ulp for E7-K2.

## Considerations

- A third carried card (the scene-memory card) was also corrected, by moving the 600 MB figure from 2 cm to
  4 cm; that correction is pinned inside its own card bead by a planted wrong golden and needs no separate
  item.
- If the cell side or the offset convention ever changes, E7-K1's golden changes by a recorded decision and
  the precision battery is re-run.

## References

Appendix E.7 (corrections to carried cards); GB-04, GB-05; §35.8; KA-792, KA-267.
