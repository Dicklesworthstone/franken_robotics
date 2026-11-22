---
key: "N35-CX"
title: "frob-conformance: counterexample registry rows CX-364 to CX-373 (numerics and determinism)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "spark", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: ["§38: the counterexample catalogue format in `frob-conformance` and the negative-control registry it seeds (`registries/negative_controls.toml`)"]
related: ["N35-KA", "N35-REF", "N35-30", "N35-15", "N35-19", "N35-20", "BD-177", "BD-181", "BD-269"]
gates: []
plan_refs: ["Appendix N.32", "§35.3", "§35.4", "§35.5", "§35.8", "§35.9", "§35.10"]
---
## Background

A counterexample is the smallest concrete case that breaks a tempting shortcut. The catalogue ties each rule of
the design to the case that motivated it, and it seeds the negative-control registry: for every rule there is a
planted defect or a hand-checkable computation that a test must catch or reproduce. Ten counterexamples belong
to the numerics chapter, which fixes what a number may claim (the determinism classes D0 exact, D1 bitwise
across backends, D2 bitwise per backend and independent of the batch, D3 tolerance with agreeing decisions, D4
nothing). Each of the ten is a shortcut that an implementer under schedule pressure would take: compare states
and call the tiers equal; pad with zero; normalize by the batch; keep offsets non-negative; seed the generator
and call it pairing; scale the matrix for accuracy. This bead enters the ten rows, with numbers, and binds each
to a test that reproduces the break. Tier: Solid.

## Scope

In: the ten rows in `registries/counterexamples.toml` (section `numerics`) with the breaking case and its
numbers, the rule protected, the executable test and its owner; the registry check. Out: the mechanisms (owner
column); the known answers (N35-KA); Spark's own counterexamples that this chapter's hazard table also cites
(the bare `f32` strict comparison, the arithmetic mask, the mergeable sketch, the unordered scatter: they are
registered by the Spark unit and exercised again in N35-19 and N35-20).

## Technical approach

**Row format (DESIGN).** `id`, `shortcut`, `case` (with its numbers), `rule`, `test`, `owner`, `tier`, and
`reproduces` (a table of literals: the inputs, the wrong result the shortcut gives and the right one).

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Executable test (owner) |
|---|---|---|---|---|
| CX-364 | A state tolerance implies agreement on the verdict | Success predicate x ≥ 1 m; one placement ends at 1.0000001 m and the other at 0.9999999 m. The state error is 2·10⁻⁷ m, inside any envelope, and the verdicts differ. Conversely two placements may end a metre apart and agree that the robot fell | ADR-NUM-03: under class D3 task decisions are tested beside states; GATE-SPARK-16 | `decision_differs_inside_envelope` and `states_differ_decision_agrees` (N35-09) |
| CX-365 | Zero discordant decisions on a small ensemble shows decision equivalence | With zero discordant worlds the one-sided 95% upper bound on the discordant rate is 1 − 0.05^(1/n) ≈ 3/n: 2.95% at n = 100, 1.0002% at n = 298, 0.9969% at n = 299. Fewer than 299 worlds cannot show 1% at all (round figure 300) | Thresholds on a frozen ensemble of at least 4,096 worlds (GATE-SPARK-16); the verdict `insufficient-ensemble` | `zero_discordant_small_n_is_insufficient` (N35-10) |
| CX-366 | Seeding the generator makes two arms a replayable pair | Seeding fixes the draws, not the order of a reduction. A difference of one unit in the last place grows like e^(λt) and reaches order one after about ln(2²⁴)/λ = 16.64/λ seconds; if uncontrolled arithmetic lowers the pair correlation from 0.95 to 0.50, a comparison needs (1 − 0.50)/(1 − 0.95) = 10 times the episodes (illustrative values) | The minimum classes of the licence table: paired statistics refused with an arm of class D4 (`FROB-REF-561`), attribution refused below D2 (`FROB-REF-562`); GATE-SPARK-17 | `seeded_but_reordered_arm_is_not_d2` (N35-02); `pairing_arithmetic_known_answer` (N35-08); the refusals (N35-07) |
| CX-367 | Padding lanes with +0.0 is neutral for a sum | The sum of the single active element −0.0 is −0.0 alone (0x80000000) and +0.0 (0x00000000) when one +0.0 lane is appended, because (−0.0) + (+0.0) = +0.0; the values compare equal, the bits differ, and a digest over bits changes with capacity | INV-NUM-02: padding holds the neutral element of its consumer (−0.0 for sums, 1.0 for products, +∞ and −∞ for minimum and maximum); GATE-SPARK-15 | `plus_zero_padding_changes_bits` (N35-13); mutant `pad-plus-zero` (N35-15) |
| CX-368 | A batch-wide observation normalizer is harmless | Two worlds observing 1 and 3 and normalized by their batch mean see −1 and +1; the first world alone sees 0, takes a different action and follows a different trajectory | INV-NUM-03: statistics across worlds never feed state; normalizers are frozen within a collection segment; GATE-SPARK-15 | `batch_mean_normalizer_breaks_isolation` (N35-14); mutant `normalizer-in-segment` (N35-15) |
| CX-369 | Anchored offsets can be kept in [0, 8) | A body that drifts to −2⁻²⁵ m must add 8. The number 8 − 2⁻²⁵ is not an `f32` (the spacing just below 8 is 2⁻²¹), so the sum rounds to 8.0: the position has moved and has left the cell's range | ADR-NUM-06: offsets centred in [−4, 4) m; PO-NUM-01 (re-anchoring exact for magnitudes 4 to 16); `FROB-REF-249` | `uncentred_shift_returns_eight` (N35-30); the device form in BD-179 |
| CX-370 | Absolute positions can be held in single precision | Single precision resolves 7.63 µm at 100 m from the origin and 61.0 µm at 1 km, too coarse for contact (a foot carrying 75 N on a stiffness of 10⁵ N/m penetrates 0.75 mm, so 61 µm of position noise is 8% force noise); a cell-relative offset in [−4, 4) m resolves at worst 2⁻²² m = 0.238 µm | Anchored positions: an integer cell index plus an `f32` offset; GATE-SPARK-05 at distances up to 1 km | `f32_spacing_table` (N35-30); the precision battery (BD-179) |
| CX-371 | Equal seeds give two designs the same disturbances | A sequential stream r₀, r₁, r₂. Design A draws a push (r₀) and a friction (r₁). Design B has one more leg and draws a per-leg value first, so its push is r₁ and its friction r₂. The seeds are equal and the two designs were pushed differently | Counter-based streams keyed by address and by the physical entity perturbed; INV-NUM-04 | `extra_leg_shifts_sequential_draws` (N35-24) |
| CX-372 | Scale the matrix to make the Cholesky solve accurate | Cholesky without pivoting is invariant under two-sided scaling by powers of two: the computed factor of D·H·D is exactly D times the computed factor of H, so scaling does not change one bit of the solve. What scaling changes is the meaning of tolerances and regularization around it | ADR-NUM-04; PO-NUM-02 (bitwise metamorphic test); the conditioning figure κ̂ is reported, not improved | `chol_pow2_scaling_is_bitwise_invariant` with the control `chol_scale_by_ten_breaks_bits` (N35-30 host form; BD-179 per backend) |
| CX-373 | One absolute residual tolerance means the same for every row | H = diag(1, 10⁻⁸) and a residual (0, 10⁻⁷): its norm passes an absolute tolerance of 10⁻⁶ while the error in the second unknown is 10⁻⁷/10⁻⁸ = 10. In scaled variables, with D₂₂ = 2¹³ = 8,192 by the power-of-two rule (the illustration uses 10⁴), the residual is (0, 8.192·10⁻⁴) and the test fails, as it should | Tolerances and regularization are stated in scaled variables; GATE-SPARK-18 | `unscaled_tolerance_passes_wrong_answer` (N35-30); the solve kernels (BD-156) |

**How a row becomes a negative control.** For each row the registry holds, under `registries/negative_controls.toml`,
one entry that names the shortcut as a plant (a mutant kernel, a mutant host function or a planted request), the
test that must fail with the plant in place, and the tier. A row whose test does not fail under its plant fails
the registry check.

## Acceptance criteria

- [ ] `registries/counterexamples.toml` holds the ten rows CX-364 to CX-373 with the literals above; the loader rejects a duplicate identifier and a row with no `test`.
- [ ] Each row's test exists and reproduces the break: the wrong result the shortcut gives and the right result are both asserted, bit for bit where the row gives bit patterns.
- [ ] Each row has a negative-control entry whose plant makes its owner test fail, recorded with the failing output.
- [ ] `cargo test -p frob-conformance counterexamples_numerics_bound` reports 10 bound rows and 0 unbound once the owners have landed.

## Test plan

### Unit tests
- `cx_numerics_rows_load`: ten rows, identifiers 364 to 373 consecutive.
- `cx_numerics_literals`: the `reproduces` tables equal compiled-in literals: 2·10⁻⁷ (CX-364); 2.95%, 1.0002%, 0.9969% (CX-365); 16.64 and 10 (CX-366); 0x80000000, 0x00000000 (CX-367); −1, +1, 0 (CX-368); 8.0 = 0x41000000 (CX-369); 2⁻¹⁷, 2⁻¹⁴, 2⁻²² (CX-370); r₀ against r₁ (CX-371); 8,192 and 8.192·10⁻⁴ (CX-373).
- `cx_numerics_tests_exist`: every `test` resolves in the workspace test listing.
- `cx_numerics_controls_fail`: for each row, running the owner test with the plant enabled returns failure (the harness runs the plant in a conformance build only).

### End-to-end script and logging
Exercised by `scripts/e2e/num_known_answers.sh` (N35-30), which runs each row's host form and its plant and
logs, per row, an `assert` line with the CX number, the shortcut's result and the correct result, and a
`metric` line `control_caught: true|false`, through the STD-E2E helper with the standard fields.

### Known answers and negative controls
The ten rows are themselves the negative controls of KA-785 to KA-805 (N35-KA): CX-364 ↔ KA-788; CX-365 ↔
KA-789; CX-366 ↔ KA-802, KA-803; CX-367 ↔ KA-786; CX-368 ↔ KA-787; CX-369 ↔ KA-790, KA-791; CX-370 ↔ KA-792;
CX-371 ↔ KA-793; CX-372 ↔ KA-796; CX-373 ↔ KA-798.

## Considerations

- A related entry elsewhere in the catalogue prints the canonical zero of a padded sum as +0; this chapter and
  the padding rule require −0.0, and CX-367 is the case that shows why. The reductions bead (N35-13) asserts
  −0.0 and the discrepancy is listed for the lead in this unit's coverage file.
- The illustrative numbers of CX-366 (0.95, 0.50, a factor of 10) are not predictions. The measured pair
  correlation and the measured divergence rate per task come from N35-08.
- Plants run only in conformance builds, carry their own kernel identities, and can never reach a citable
  result.

## References

Appendix N.32 (CX-364 to CX-373); §35.3, §35.4, §35.5, §35.8, §35.9, §35.10. ADR-NUM-03, -04, -06; INV-NUM-02,
-03, -04; PO-NUM-01, -02.
