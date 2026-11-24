---
key: "N35-CX"
title: "frob-conformance: counterexample registry rows CX-364 to CX-373 (numerics and determinism)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "spark", "gauntlet", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-14"
deps: ["BD-025", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-156", "BD-179", "N20-CX", "N35-02", "N35-07", "N35-08", "N35-09", "N35-10", "N35-13", "N35-14", "N35-15", "N35-24", "N35-30", "N35-KA", "N35-REF", "N38b-04"]
gates: []
plan_refs: ["Appendix N.32", "§35.3", "§35.4", "§35.5", "§35.8", "§35.9", "§35.10", "§35.15"]
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
to a plant and to a test that reproduces the break. Tier: Solid.

## Scope

In: the ten entries in the counterexample catalogue `crates/frob-conformance/registry/counterexamples.toml`
with the breaking case and its numbers, the rule protected, the plant and its gate; the table that binds each
entry to its executable test and owner; the registry check and its script. Out: the catalogue's record schema and
the seeding tool that writes the controls (N38b-05: this bead writes no row of `controls.toml`); the control runner
(N38b-04); the mechanisms and the plants (owner column); the known answers (N35-KA); Spark's own counterexamples
that this chapter's hazard table also cites (the bare `f32` strict comparison, the arithmetic mask, the mergeable
sketch, the unordered scatter: they are registered by the Spark unit and exercised again in N35-19 and N35-20).

## Technical approach

**Entry format.** The record of N38b-05: `id`, `shortcut`, `case` (with its numbers), `numeric = true`,
`plan_case = "numbers"`, `case_supplied_by` (empty), `rule`, `rule_ids` (the ADR, INV, PO, gate and refusal
identifiers of the rule column), `defined_in` (§35.3 to §35.10 as Appendix N.32 gives them), `same_as` (empty for
all ten: Appendix N marks none of them ≡, and the catalogue's plan-time count of thirteen ≡ entries is asserted by
the loader), `label` (`DESIGN`: every number is arithmetic), `control_kind` (`mutation` for a mutant kernel or
host function, `planted-input` for a planted request) and `plant` (the identifier of the control column). The
target, defect and expected observation of the control column are the texts the seeding tool copies into the
control's `target`, `defect` and `expect`. The fields an earlier draft of this bead put in the row (`test`,
`owner`, `tier` and `reproduces`, a table of literals: the inputs, the wrong result the shortcut gives and the
right one) are not catalogue fields, and N38b-05's loader admits no others; they are kept as one compiled-in table
keyed by CX number in this bead's test file `crates/frob-conformance/tests/counterexamples_numerics.rs`, which the
registry check reads.

**CX-371 and the sensorium entry CX-215.** CX-215 (N20-CX) states the same shortcut in words and is word-only;
its `case_supplied_by` names CX-371, which supplies the numeric case. That link is carried by CX-215's field
alone: CX-371 takes no `same_as` (DESIGN, correcting an earlier draft of this bead that set `same_as = "CX-215"`:
CX-371 is not marked ≡ in Appendix N, and a fourteenth ≡ entry would fail the loader's count of thirteen). Each
keeps its own control.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Executable test (owner) |
|---|---|---|---|---|
| CX-364 | A state tolerance implies agreement on the verdict | Success predicate x ≥ 1 m; one placement ends at 1.0000001 m and the other at 0.9999999 m. The state error is 2·10⁻⁷ m, inside any envelope, and the verdicts differ. Conversely two placements may end a metre apart and agree that the robot fell | ADR-NUM-03: under class D3 task decisions are tested beside states; GATE-SPARK-16 | `decision_differs_inside_envelope` and `states_differ_decision_agrees` (N35-09) |
| CX-365 | Zero discordant decisions on a small ensemble shows decision equivalence | With zero discordant worlds the one-sided 95% upper bound on the discordant rate is 1 − 0.05^(1/n) ≈ 3/n: 2.95% at n = 100, 1.0002% at n = 298, 0.9969% at n = 299. Fewer than 299 worlds cannot show 1% at all (round figure 300) | Thresholds on a frozen ensemble of at least 4,096 worlds (GATE-SPARK-16); the verdict `insufficient-ensemble` | `zero_discordant_small_n_is_insufficient` (N35-10) |
| CX-366 | Seeding the generator makes two arms a replayable pair | Seeding fixes the draws, not the order of a reduction. A difference of one unit in the last place grows like e^(λt) and reaches order one after about ln(2²⁴)/λ = 16.64/λ seconds; if uncontrolled arithmetic lowers the pair correlation from 0.95 to 0.50, a comparison needs (1 − 0.50)/(1 − 0.95) = 10 times the episodes (illustrative values) | The minimum classes of the licence table: paired statistics refused with an arm of class D4 (`FROB-REF-561`), attribution refused below D2 (`FROB-REF-562`); GATE-SPARK-17 | `seeded_but_reordered_arm_is_not_d2` (N35-02); `horizon_known_answer` and `episode_multiplier_known_answer` (N35-08); the refusals (N35-07) |
| CX-367 | Padding lanes with +0.0 is neutral for a sum | The sum of the single active element −0.0 is −0.0 alone (0x80000000) and +0.0 (0x00000000) when one +0.0 lane is appended, because (−0.0) + (+0.0) = +0.0; the values compare equal, the bits differ, and a digest over bits changes with capacity | INV-NUM-02: padding holds the neutral element of its consumer (−0.0 for sums, 1.0 for products, +∞ and −∞ for minimum and maximum); GATE-SPARK-15 | `plus_zero_padding_changes_bits` (N35-13); mutant `pad-plus-zero` (N35-13; also run by N35-15) |
| CX-368 | A batch-wide observation normalizer is harmless | Two worlds observing 1 and 3 and normalized by their batch mean see −1 and +1; the first world alone sees 0, takes a different action and follows a different trajectory | INV-NUM-03: statistics across worlds never feed state; normalizers are frozen within a collection segment; GATE-SPARK-15 | `batch_mean_normalizer_breaks_isolation` (N35-14); mutant `normalizer-in-segment` (N35-15) |
| CX-369 | Anchored offsets can be kept in [0, 8) | A body that drifts to −2⁻²⁵ m must add 8. The number 8 − 2⁻²⁵ is not an `f32` (the spacing just below 8 is 2⁻²¹), so the sum rounds to 8.0: the position has moved and has left the cell's range | ADR-NUM-06: offsets centred in [−4, 4) m; PO-NUM-01 (re-anchoring exact for magnitudes 4 to 16); `FROB-REF-249` | `uncentred_shift_returns_eight` (N35-30); the device form in BD-179 |
| CX-370 | Absolute positions can be held in single precision | Single precision resolves 7.63 µm at 100 m from the origin and 61.0 µm at 1 km, too coarse for contact (a foot carrying 75 N on a stiffness of 10⁵ N/m penetrates 0.75 mm, so 61 µm of position noise is 8% force noise); a cell-relative offset in [−4, 4) m resolves at worst 2⁻²² m = 0.238 µm | Anchored positions: an integer cell index plus an `f32` offset; GATE-SPARK-05 at distances up to 1 km | `f32_spacing_table` (N35-30); the precision battery (BD-179) |
| CX-371 | Equal seeds give two designs the same disturbances | A sequential stream r₀, r₁, r₂. Design A draws a push (r₀) and a friction (r₁). Design B has one more leg and draws a per-leg value first, so its push is r₁ and its friction r₂. The seeds are equal and the two designs were pushed differently | Counter-based streams keyed by address and by the physical entity perturbed; INV-NUM-04 | `extra_leg_shifts_sequential_draws` (N35-24) |
| CX-372 | Scale the matrix to make the Cholesky solve accurate | Cholesky without pivoting is invariant under two-sided scaling by powers of two: the computed factor of D·H·D is exactly D times the computed factor of H, so scaling does not change one bit of the solve. What scaling changes is the meaning of tolerances and regularization around it | ADR-NUM-04; PO-NUM-02 (bitwise metamorphic test); the conditioning figure κ̂ is reported, not improved | `chol_pow2_scaling_is_bitwise_invariant` with the control `chol_scale_by_ten_breaks_bits` (N35-30 host form; BD-179 per backend) |
| CX-373 | One absolute residual tolerance means the same for every row | H = diag(1, 10⁻⁸) and a residual (0, 10⁻⁷): its norm passes an absolute tolerance of 10⁻⁶ while the error in the second unknown is 10⁻⁷/10⁻⁸ = 10. In scaled variables with D = diag(1, 10⁴) the residual is (0, 10⁻³) and the test fails, as it should. The power-of-two rule the implementation uses gives D₂₂ = 2¹³ = 8,192 (scaled diagonal 0.671, in [½, 2)) and the residual (0, 8.192·10⁻⁴), which fails the same test | Tolerances and regularization are stated in scaled variables (§35.10 item 2); GATE-SPARK-18 | `unscaled_tolerance_passes_wrong_answer` (N35-30); the solve kernels (BD-156) |

CX-373's case of record is the plan's illustration with D = diag(1, 10⁴); the exact power-of-two figures are
added beside it because they are what the implemented fixture computes (N35-KA carries the same pair for KA-798).
Both are asserted.

**Controls.** One per entry, derived by the seeding tool. The gate is the one the rule column names; for CX-369,
CX-371 and CX-372, whose rules name a proof obligation or an invariant but no gate, the gate is the one the plan
ties to that obligation, so the seeding resolves it from `rule_ids` and does not attach it to an unrelated gate:
GATE-SPARK-05 for ADR-NUM-06 and PO-NUM-01 (the explainer GB-04 supports §35.8, ADR-NUM-06, PO-NUM-01 and
GATE-SPARK-05 together), GATE-SPARK-15 for INV-NUM-04 (KA-795, "draws depend on address alone" in the gate's
text), GATE-SPARK-18 for PO-NUM-02 (named in the gate's text). Each of the three gates is written into the entry's
`rule_ids`.

| CX | Kind; plant | Target | Defect | Expected observation | Gate | Planted by; caught by |
|---|---|---|---|---|---|---|
| CX-364 | mutation; `states-only-comparator` | the D3 pair comparison of `frob-chronicle` | task decisions are not compared, only states | the pair at 1.0000001 m and 0.9999999 m is reported as agreeing instead of discordant and `boundary-sensitive` | GATE-SPARK-16 | N35-09; N35-30 row 4 |
| CX-365 | mutation; `zero-discordant-small-n` | the decision-equivalence verdict | zero discordant worlds accepted as equivalence at any n | n = 100 returns `equivalent` instead of `insufficient-ensemble` | GATE-SPARK-16 | N35-10 |
| CX-366 | planted-input; `seeded-reordered-arm` | the determinism classifier and the licence check | an arm with fixed seeds and a reordered reduction offered as class D2 for attribution | the classifier assigns it D2 and no `FROB-REF-562` is raised | GATE-SPARK-17 | N35-02 (the arm); N35-07 (the refusal) |
| CX-367 | mutation; `pad-plus-zero` | the initializer of one summed buffer | +0.0 written into padding instead of −0.0 | the tree sum of [−0.0] at capacity 2 is 0x00000000, so the `capacity` axis finds different digests | GATE-SPARK-15 | N35-13; N35-15 (run by name on `neg-zero-sum`), N35-30 row 2, BD-160 (through the tile path) |
| CX-368 | mutation; `normalizer-in-segment` | the observation normalizer | updated from batch statistics at every control interval instead of at segment boundaries | the `two-observers` fixture gives −1 and +1 in the pair against 0 alone, and the `alone` axis fails | GATE-SPARK-15 | N35-15; N35-30 row 3 |
| CX-369 | mutation; `uncentred-offset-range` | the re-anchoring of anchored positions | offsets kept in [0, 8) | the shift of −2⁻²⁵ m returns 8.0 and the position moves; the exhaustive re-anchoring check fails | GATE-SPARK-05 | N35-30 (host form); BD-179 (device form) |
| CX-370 | mutation; `absolute-f32-positions` | the position representation of the precision battery | absolute `f32` positions instead of cell index plus offset | at 1 km the spacing is 2⁻¹⁴ m and the battery's contact violations leave their envelope | GATE-SPARK-05 | BD-179 |
| CX-371 | mutation; `sequential-stream` | the stream keying of `frob-core` | draws taken in call order from one sequential stream | design B's push is r₁ against design A's r₀, so the two designs are pushed differently | GATE-SPARK-15 | N35-24; N35-30 row 6 |
| CX-372 | mutation; `chol-scale-by-ten` | the diagonal scaling `K-scale` | scaling by 10 instead of a power of two | the factor of D·H·D differs from D times the factor of H in at least one bit | GATE-SPARK-18 | N35-30 (host form); BD-179 (per backend) |
| CX-373 | mutation; `unscaled-residual-tolerance` | the residual test of the solve | the tolerance applied to the unscaled residual | the residual (0, 10⁻⁷) passes at 10⁻⁶ while the second unknown is wrong by 10 | GATE-SPARK-18 | BD-156 (the kernels); N35-30 (host form) |

**How a row becomes a negative control.** The seeding tool of N38b-05 derives, from each entry, one record of
`controls.toml` whose `seed` is the CX number and whose plant is the entry's `plant` (its `id` is `<planting
bead>:<plant identifier>`, N38b-05, the planting bead being the first bead of the "Planted by" column:
`N35-09:states-only-comparator`, `N35-10:zero-discordant-small-n`, `N35-02:seeded-reordered-arm`,
`N35-13:pad-plus-zero`, `N35-15:normalizer-in-segment`, `N35-30:uncentred-offset-range`,
`BD-179:absolute-f32-positions`, `N35-24:sequential-stream`, `N35-30:chol-scale-by-ten` and
`BD-156:unscaled-residual-tolerance`); the test that must fail with
the plant in place and its tier come from this bead's table. Before the planting bead lands the control is
`stale` (its plant is not built) and its gate cannot record a pass; once it lands, a row whose test does not fail
under its plant fails the gate.

## Acceptance criteria

- [ ] `counterexamples.toml` holds the ten entries CX-364 to CX-373 with the literals above, valid under N38b-05's loader (a duplicate identifier is rejected); none carries `same_as`; the registry check fails an entry of §35 with no test in this bead's table.
- [ ] The arithmetic of each row is reproduced by this bead's test file from the literals alone, bit for bit where the row gives bit patterns; the wrong result the shortcut gives and the right result are both asserted.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control whose `seed` is its CX number, with the gate of the control table and non-empty target, defect and expectation; the second run changes no byte; this bead writes no row of `controls.toml`.
- [ ] `cargo test -p frob-conformance --test counterexamples_numerics` passes; it reports each row as bound (owner test present) or unbound with its owner bead, and fails only for a row whose owner bead is closed while its test or plant is missing.
- [ ] `scripts/e2e/registry_n35_cx.sh` passes and rejects each of its planted registry defects.

## Test plan

### Unit tests

`crates/frob-conformance/tests/counterexamples_numerics.rs` (real registry files and scratch copies, no mocks):
- `cx_numerics_rows_load`: ten rows, identifiers 364 to 373 consecutive; every entry has `plan_case = "numbers"`, a `plant`, a `control_kind` and a gate; no entry carries `same_as`; CX-215's `case_supplied_by` names CX-371.
- `cx_numerics_literals`: the `reproduces` tables of the test file equal the numbers in each entry's `case`, and each is recomputed: 2·10⁻⁷ (CX-364); 1 − 0.05^(1/n) = 2.95%, 1.0002%, 0.9969% at n = 100, 298, 299 (CX-365); ln(2²⁴) = 16.64 and (1 − 0.50)/(1 − 0.95) = 10 (CX-366); 0x80000000, 0x00000000 (CX-367); −1, +1, 0 (CX-368); 8.0 = 0x41000000 from `(-2⁻²⁵f32) + 8.0f32` (CX-369); 2⁻¹⁷, 2⁻¹⁴, 2⁻²² and 75/10⁵ = 0.75 mm (CX-370); r₀ against r₁ (CX-371); (0, 10⁻³) for D = diag(1, 10⁴), and 8,192, 0.671 and 8.192·10⁻⁴ for the power-of-two rule (CX-373).
- `cx_numerics_gates_exist`: every gate of the control table is in the gate register.
- `cx_numerics_owner_binding`: every owner test of the table resolves in the workspace test listing once its owner bead is closed; before that it is reported `unbound` with the bead.
- `cx_numerics_seeded_once`: on a scratch copy, seeding twice yields one control per entry and no byte of change.

### End-to-end script and logging

`scripts/e2e/registry_n35_cx.sh` (header `bead=N35-CX milestone=M1 labels=spark,registry
plants=drop-plant,unknown-gate,digit-changed,extra-same-as`): runs the test file; re-runs the seeding tool
(`cargo test -p frob-conformance --test seed_controls -- --ignored seed`) and the join (`cargo test -p
frob-conformance --test seed_join`) on a scratch copy of `registry/` and asserts by digest that a second seeding
run is a no-op; plants, one at a time on scratch copies, an entry with its plant removed, a control gate absent
from the register (GATE-SPARK-99), a case literal with one changed digit (0.9969 → 0.9996), and `same_as =
"CX-215"` on CX-371 (the loader's count of thirteen must reject it), and asserts each is rejected. For each control
whose planting bead has landed it runs the gate's tier target in the form of ADR-GAUNT-01 (`FROB_TEST_SEED=<seed>
cargo test -p frob-conformance --test <tier from the gate record> -- <gate>`, then with `--features
gauntlet-mutations` and the plant's feature) and asserts pass, then fail; it calls no `frob` verb. The host forms
and their plants are also run by `scripts/e2e/num_known_answers.sh` (N35-30), which logs per row an `assert` line
with the CX number, the shortcut's result and the correct result, and a `metric` line `control_caught`. This
script sources the STD-E2E helper and writes `target/e2e-logs/registry_n35_cx/<utc-timestamp>.ndjson` with a
transcript: every line has `ts`, `script`, `bead`, `step`, `event`; `cmd` lines with the exact command line, exit
code and wall time; the evidence block (claim Known answer, class Tested, determinism D0); `metric` lines with
entries, controls seeded, bound, unbound and stale; the digests of the catalogue and the scratch `controls.toml`
before and after the second seeding; per planted defect an `assert` with the expected and observed failing check;
on failure the first differing entry and the scratch path; a `summary` line with assertions passed and failed.

### Known answers and negative controls

The ten rows are themselves the negative controls of known answers of N35-KA: CX-364 ↔ KA-788; CX-365 ↔ KA-789;
CX-366 ↔ KA-802, KA-803; CX-367 ↔ KA-786; CX-368 ↔ KA-787; CX-369 ↔ KA-790, KA-791; CX-370 ↔ KA-792; CX-371 ↔
KA-793; CX-372 ↔ KA-796; CX-373 ↔ KA-798. Controls of the registry itself: the four planted defects of the script.
Determinism: D0, the test file's output identical twice and on both platforms.

## Considerations

- A related entry elsewhere in the catalogue prints the canonical zero of a padded sum as +0; this chapter and
  the padding rule require −0.0, and CX-367 is the case that shows why. The reductions bead (N35-13) asserts
  −0.0 and the discrepancy is listed for the lead in this unit's coverage file.
- The illustrative numbers of CX-366 (0.95, 0.50, a factor of 10) are not predictions. The measured pair
  correlation and the measured divergence rate per task come from N35-08.
- Plants run only in conformance builds (Cargo features under `gauntlet-mutations`, BD-025), carry their own
  kernel identities, and can never reach a citable result.
- The plant identifiers reuse the names the owner beads already give their mutants (`pad-plus-zero` of N35-13,
  `normalizer-in-segment` of N35-15; the states-only comparator, the uncentred convention, `SequentialStream`, the
  scaling by 10 and the unscaled test of N35-30's table), so one plant is not built twice under two names.

## References

Appendix N.32 (CX-364 to CX-373); §35.3, §35.4, §35.5, §35.8, §35.9, §35.10, §35.15. ADR-NUM-03, -04, -06;
INV-NUM-02, -03, -04; PO-NUM-01, -02; GATE-SPARK-05, -15, -16, -17, -18.
