---
key: "N35-KA"
title: "frob-conformance: known-answer registry rows KA-785 to KA-805 (numerics and determinism)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "spark", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.6: the known-answer registry format and its loader in `frob-conformance` (rows are append-only; the printed appendix is generated from the registry)"]
related: ["N35-CX", "N35-REF", "N35-30", "BD-177", "BD-181", "BD-269", "BD-179", "BD-180", "BD-207"]
gates: []
plan_refs: ["Appendix M.34", "§35.14", "§35.13", "§35.15", "§38.6"]
---
## Background

A known answer is a case whose truth is fixed by mathematics or by an exact computation before any code runs: a
value, a bit pattern, a verdict. The Gauntlet (the verification harness, crate `frob-conformance`) keeps every
known answer in one registry, `registries/known_answers.toml`, with the tier it runs at (G0 laws, G1 known
answers, G3 metamorphic, G5 determinism), its execution site and the negative control that must fail. A gate may
cite a known answer only when the row has an executable test and its negative control has been shown to fail
(the rule: a gate that nothing can fail validates nothing).

The numerics chapter fixes what a number may claim: which results are exact (determinism class D0), bitwise
across backends (D1), bitwise per backend and independent of the batch (D2), inside a tolerance with agreeing
task decisions (D3), or nothing (D4). Twenty-one known answers, KA-785 to KA-805, pin its rules. Every other
unit cites them by number. This bead enters the 21 rows with their values and binds each to the bead whose test
executes it. Tier: Solid. It serves GOAL-20 (state determinism as a class on every result, and test it), BET-02
(one source, three backends, inside declared determinism classes) and BET-08 (single precision is adequate on
the GPU).

## Scope

In: the 21 rows as data (case, truth with formula or value, tolerance, tier and site, negative control, owner
test); the bit patterns and exact numbers each test asserts; the registry check that every row has an executable
test. Out: the mechanisms and their tests (the owner column); the ten-row battery that runs the smallest-case
forms on the host (N35-30); counterexamples (N35-CX); refusal codes (N35-REF).

## Technical approach

**Row format (DESIGN).** `id`, `section`, `case`, `truth`, `tolerance`, `tier`, `site`, `negative_control`,
`owner_test` (crate and test name), `gate` (optional), `values` (a table of literals the test asserts, so the
registry and the test cannot drift apart: the test reads its expected values from the row).

**Terms used in the rows.** A *world address* is `(run_seed, world, episode)`; it determines every random draw a
world sees. A *reference workload* is Q (reference quadruped: 12 joints, 18 degrees of freedom, contact capacity
32, row capacity 160, step 2 ms) or H (reference humanoid: 29 joints, 35 degrees of freedom, contact capacity
64, row capacity 320). A *per-world digest* is the four-word integer mix of a world's canonical state slabs.
u is the single-precision unit round-off 2⁻²⁴ ≈ 6.0·10⁻⁸. κ₂ is the 2-norm condition number.

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Owner |
|---|---|---|---|---|---|---|
| KA-785 | One world run alone, in 100 random co-tenancies, under slot permutation, at two bucket capacities and on one and two shards | Identical per-world digests: given a policy (weights and normalizers), a world's trajectory is a function of its address, design, scene and task alone (INV-NUM-03) | exact, for 1,000 episodes of each reference workload on `batch-cpu` and each device family (GATE-SPARK-15) | G5 · all tiers | A kernel with a batch-wide floating reduction feeding state | BD-181, N35-15 |
| KA-786 | Sum of the one active element −0.0 at two capacities | −0.0 both times, bit for bit (pattern 0x80000000): padding trails the active lanes and holds −0.0 for sums (INV-NUM-02) | exact (GATE-SPARK-15) | G5 · all tiers | Padding with +0.0: (−0.0) + (+0.0) = +0.0 (pattern 0x00000000); the values compare equal and the bits differ | N35-13, N35-15 |
| KA-787 | Two worlds observing 1 and 3 | Each world's actions equal those of its isolated run: observation normalizers are frozen within a collection segment and are constants of the policy in evaluation | exact (GATE-SPARK-15) | G5 · all tiers | A normalizer updated inside the segment: the batch mean 2 gives −1 and +1; the first world alone sees 0 | N35-14, N35-15 |
| KA-788 | Success predicate x ≥ 1 m; one placement ends at 1.0000001 m, the other at 0.9999999 m | State error 2·10⁻⁷ m, inside any envelope; the verdicts differ; the pair is reported as discordant and `boundary-sensitive` on both placements | exact (verdict) | G1 · all tiers | A check that compares states only | N35-09 |
| KA-789 | Decision-equivalence bound | The one-sided 95% exact binomial (Clopper–Pearson) upper bound on the discordant rate must be at most 1%. With zero discordant worlds the bound is 1 − 0.05^(1/n) ≈ 3/n: 1.0002% at n = 298, 0.9969% at n = 299, so 299 is the smallest admissible n (the round figure is 300). At n = 4,096 the bound is 0.9921% for 30 discordant worlds and 1.0201% for 31, so at most 30 are admitted. The net one-sided rate bound is at most 0.5% | as stated (TARGET, GATE-SPARK-16; OPEN-NUM-2) | G1 · host | A planted bias of one decision in fifty: 82 of 4,096 one-sided, bound 2.40%, must fail | N35-10, BD-177 |
| KA-790 | Offset −2⁻²⁵ m in a cell | The centred convention (offset in [−4, 4) m) leaves the offset unchanged; it never adds the cell side to a small number, and every shift it does make is exact | exact | G0 · all tiers | The uncentred [0, 8) shift returns 8.0 (pattern 0x41000000): 8 − 2⁻²⁵ is not an `f32`, the spacing just below 8 being 2⁻²¹ | N35-30 (host form); the anchored-position bead of the Spark unit and BD-179 (device form) |
| KA-791 | Re-anchoring (PO-NUM-01) | For 4 ≤ abs(x) ≤ 16 subtracting or adding 8.0 is exact, by Sterbenz's lemma (the difference of two floats within a factor of two of each other is representable) | exact; exhaustive over the 2²⁵ values of the two binades [4, 8) and [8, 16) and two signs, on each backend | G0 · all tiers | none stated; the uncentred shift of KA-790 is used | N35-30 (host), BD-179 (per backend) |
| KA-792 | Resolution of anchored positions | Single-precision spacing is 2^(e−23) in [2^e, 2^(e+1)): 2⁻¹⁷ m = 7.63 µm at 100 m from the origin, 2⁻¹⁴ m = 61.0 µm at 1 km; an offset in [−4, 4) m resolves at worst 2⁻²² m = 0.238 µm; an uncentred [0, 8) offset 2⁻²¹ m = 0.477 µm | exact | G1 · host | none stated | N35-30 |
| KA-793 | Sequential draws with one extra leg | Addressed streams give both designs the same push: the push is the draw at (address, step, stream `push`) for both | exact | G5 · all tiers | A sequential stream r₀, r₁, r₂: design A draws its push as r₀; design B, with one more per-leg draw first, draws it as r₁ | N35-24 |
| KA-794 | Integer-to-uniform conversion (PO-NUM-03) | u = (x ≫ 8)·2⁻²⁴ is exact and lies in [0, 1); the largest value is (2²⁴ − 1)·2⁻²⁴ = 0.99999994 (pattern 0x3F7FFFFF) | exact; exhaustive over the 2²⁴ values of x ≫ 8 | G0 · all tiers | none stated; N35-22 adds one: converting the full 32-bit word, `x as f32 · 2⁻³²`, returns 1.0 for x = 0xFFFFFFFF | N35-22 |
| KA-795 | Draw tables across slots, buckets, shards and backends | A random draw is a function of (address, step, stream) alone (INV-NUM-04); adding a stream changes no draw of any other stream | exact (GATE-SPARK-15) | G5 · all tiers | none stated; N35-16 adds two: draws keyed by slot; a renumbered stream registry | N35-16 |
| KA-796 | Computed Cholesky factor of D·H·D against D times the computed factor of H, D a diagonal of powers of two (PO-NUM-02) | Bit-identical, absent overflow and underflow | exact, on 10⁴ matrices per backend (GATE-SPARK-18) | G3 · all tiers | A scaling by 10 instead of a power of two | BD-179 (the conditioning bead of the Spark unit); N35-30 (host form) |
| KA-797 | Exact diagonal scaling by `K-scale` | D_ii = 2^(−⌈e_i/2⌉) with e_i the binary exponent of H_ii; the scaled diagonal lies in [½, 2); no rounding is introduced. Example: H_ii = 10⁻⁸ has e = −27, D = 2¹³ = 8,192, scaled diagonal 0.671 | exact | G0 · all tiers | none stated | BD-156; N35-30 (host form) |
| KA-798 | H = diag(1, 10⁻⁸) with residual (0, 10⁻⁷) | The error in the second unknown is 10⁻⁷/10⁻⁸ = 10. In scaled variables the residual is (0, 8.192·10⁻⁴) with the power-of-two rule (the illustration D = diag(1, 10⁴) gives (0, 10⁻³)), and a tolerance test at 10⁻⁶ fails | exact (verdict) | G1 · host | The unscaled absolute test at 10⁻⁶ passes (‖r‖ = 10⁻⁷) | N35-30; BD-156 |
| KA-799 | Conditioning figure κ̂ = (max L̂_ii / min L̂_ii)² | A lower bound on κ₂ of the scaled matrix: κ̂ never exceeds the κ₂ computed in `f64`; the relative error of the `f32` Newton step against `f64` is bounded | at most 10·κ₂·u on 10⁴ contact states per reference robot; at most 10⁻³ on near-rigid states under the `stiff` profile (GATE-SPARK-18) | G1 · all tiers | none stated | BD-179; N35-29 |
| KA-800 | The set {0, 3, 5, 9} histogrammed in two orders | Identical histograms: a bin is the sign, the exponent and the top five mantissa bits of the `f32` pattern (index = pattern ≫ 18: 0 → 0, 3 → 4,112, 5 → 4,136, 9 → 4,164); a bin is at most 2⁻⁵ = 3.125% wide ("at most 3.2%"); counts are integers; merging is addition | exact | G5 · all tiers | A closest-pair two-centroid sketch: order 0, 3, 5, 9 ends as (8/3 with weight 3; 9 with weight 1), order 0, 9, 5, 3 as (1.5 with weight 2; 7 with weight 2) | N35-21 |
| KA-801 | A policy trained under `fast-training` evaluated twice under `device-replay` | Identical digests | exact (GATE-SPARK-17) | G5 · all tiers | Citing its training return is refused (`FROB-REF-861`) | N35-06, BD-269 |
| KA-802 | Licence refusals on planted cases | A class D4 arm in a paired comparison and a cross-family pair demanded as paired without class D1 are refused (`FROB-REF-561`); attribution below class D2 is refused (`FROB-REF-562`); a cited training return is refused (`FROB-REF-861`) | exact (GATE-SPARK-17) | G1 · daemon | none stated; BD-269 adds the acceptance controls (the same requests at an admissible class succeed) | N35-07, BD-269 |
| KA-803 | Pairing arithmetic (illustrative values) | The variance of a paired difference scales with 1 − ρ: a fall in pair correlation from 0.95 to 0.50 needs (1 − 0.50)/(1 − 0.95) = 10 times the episodes; a difference of one unit in the last place reaches order one after about ln(2²⁴)/λ = 16.64/λ seconds | exact | G1 · host | none stated | N35-08 |
| KA-804 | Class of a combined result | The weakest class among the kernels, placements, profile and inputs that produced it (INV-NUM-06); no conversion upward compiles | exact (compile-fail test) | G0 · host | none stated; N35-01 adds one: a planted `From<D3> for D2` must fail the build check | N35-01 |
| KA-805 | Lint over dialect sources and the Rust of `frob-kernels` | `mul_add`, `sin`, `cos`, `exp`, `ln`, `powf` and their kin are rejected at build time (`FROB-REF-251`); `sqrt` is correctly rounded and allowed | exact | G0 · host | none stated; N35-18 adds the corpus of one file per banned construct, each of which must be rejected | N35-18, BD-013 |

**Registry check.** `frob-conformance` loads the rows; a row without `owner_test`, with an `owner_test` that
does not exist in the workspace, or whose `values` differ from the literals above fails the check. Rows are
append-only: a correction is a new revision of the row with the old one kept.

**Rows for which no negative control is stated.** Ten rows state none: KA-791, -792, -794, -795, -797, -799,
-802, -803, -804 and -805. Where an owner bead adds a control, the table says so; the others are value checks
whose failure mode is a wrong literal, and the registry marks them `control: value-check`.

## Acceptance criteria

- [ ] `registries/known_answers.toml` holds 21 rows KA-785 to KA-805 with the fields above; the loader rejects a duplicate, a missing field and an unknown tier.
- [ ] Every row names an `owner_test` that exists; `cargo test -p frob-conformance known_answers_numerics_bound` lists 21 bound rows and 0 unbound once the owners have landed, and reports the unbound rows by number before that.
- [ ] The literals in `values` equal those in this bead: 0x80000000 and 0x00000000 (KA-786); 299, 30, 31, 0.9921%, 1.0201%, 82, 2.40% (KA-789); 0x41000000 (KA-790); 2²⁵ (KA-791); 2⁻¹⁷, 2⁻¹⁴, 2⁻²², 2⁻²¹ (KA-792); 0x3F7FFFFF (KA-794); 8,192 and 0.671 (KA-797); 8.192·10⁻⁴ (KA-798); 0, 4,112, 4,136, 4,164 (KA-800); 10 and 16.64 (KA-803).
- [ ] Each row with a stated negative control names the control's identifier in `registries/negative_controls.toml`, and the control is recorded as failing its test.
- [ ] The generated appendix table for this chapter is byte-identical on regeneration.

## Test plan

### Unit tests
- `ka_numerics_rows_load`: 21 rows parse; identifiers are consecutive from 785 to 805.
- `ka_numerics_values_match_literals`: each `values` table equals a compiled-in copy of the literals above (the copy is in the test, not in the loader).
- `ka_numerics_owner_tests_exist`: every `owner_test` resolves to a test in the workspace's test listing.
- `ka_numerics_controls_registered`: the eleven rows with a stated control (KA-785 to -790, -793, -796, -798, -800, -801) each name a registered control.
- `ka_row_append_only`: editing a field of an existing row without a new revision fails the registry check (planted edit).

### End-to-end script and logging
No script of its own: the rows are exercised by `scripts/e2e/num_known_answers.sh` (N35-30) and by the gate
scripts of BD-181, BD-177 and BD-269, each of which logs, per row, an `assert` line with the KA number, the
expected and the measured value, through the STD-E2E helper (fields `ts`, `script`, `bead`, `step`, `event`,
command line, exit code, wall time, evidence block, values with thresholds, digests, and on failure the first
differing value with artifact paths).

### Known answers and negative controls
The table is the content. Determinism lanes: the registry file loads to the same in-memory rows on every
platform (integers above 2⁵³ are strings; bit patterns are hexadecimal strings).

## Considerations

- A known answer whose tolerance reads "to set" may not be cited by a gate; none of these 21 has one, but KA-789
  depends on OPEN-NUM-2 (the 1% and 0.5% thresholds and the ensemble size; default as stated, frozen once at M2
  by N35-11), and KA-799 on OPEN-NUM-4 (the ill-conditioning limit and the refinement count; N35-29).
- KA-798's plan text uses D = diag(1, 10⁴) as an illustration; the exact rule gives 2¹³. The registry carries
  both numbers and the test asserts the exact one.
- KA-789's "at least 300 worlds" is a round figure; the exact minimum is 299. The registry carries 299 as the
  value and 300 as the stated figure.

## References

Appendix M.34 (KA-785 to KA-805); §35.13, §35.14, §35.15; §38.6. Gates GATE-SPARK-15, -16, -17, -18.
INV-NUM-02, -03, -04, -06; PO-NUM-01, -02, -03; OPEN-NUM-2, OPEN-NUM-4.
