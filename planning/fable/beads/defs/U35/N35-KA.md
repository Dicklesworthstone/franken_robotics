---
key: "N35-KA"
title: "frob-conformance: known-answer registry rows KA-785 to KA-805 (numerics and determinism)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "spark", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "N38a-12", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N35-30", "N35-CX", "N35-REF", "N38b-34"]
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

In: the 21 rows as records of the one known-answer file (case, truth with formula or value, tolerance, tier and
sites, negative control), with their derivation files (owner test); each row's section, owner and due; the bit
patterns and exact numbers each test asserts; the control text of KA-791, which this chapter sets; the registry
check that every row has an executable test; a registry-only end-to-end script. Out: the row schema, loader and
eligibility rules (N38a-12); the `NC-` records of the eleven stated controls and of KA-791's, issued by the
seeding tool of N38b-05 (this bead writes no control record and types no `NC-` number); the controls the owner beads add (N35-01, N35-16,
N35-18, N35-22) and those proposed for the remaining rows at M2 (N38b-34); the mechanisms and their tests (the
owner column); the ten-row battery that runs the smallest-case forms on the host (N35-30); counterexamples
(N35-CX); refusal codes (N35-REF).

## Technical approach

**Row format.** The record of N38a-12 in `crates/frob-conformance/registry/known_answers.toml` (the logical
name `registries/known_answers.toml` resolves to it): `id`, `section` with `owner` (the gate, PO or INV of the
row, when it has one), `fixture`, `same_as`, `case`, `truth` (its `value` is the table of literals the test
asserts, so the registry and the test cannot drift apart: the test reads its expected values from the row),
`tolerance` (`exact`, or `gate` for KA-789 and KA-799), `tier`, `sites`, `control` (`text` = the cell below;
`nc` filled by the seeding tool or by the registering bead), `derivation` (`docs/derivations/<fixture>.md`,
whose front matter lists for each KA its owner test, crate and test name, and its owner bead), `label`
(`DESIGN`), `due` and `printed`. The fields of an earlier draft map as: `site` → `sites`, `negative_control` →
`control.text`, `owner_test` → the derivation front matter, `gate` → `owner`, `values` → `truth.value`.

**Section, owner, due and sites (DESIGN, from Appendix M.34 and the gate register).**
- **`section`.** M.34's "Where stated" cell as printed: §35.5, §35.14 for KA-785 to KA-787; §35.4, §35.14 for
  KA-788; §35.4 for KA-789; §35.8, §35.14 for KA-790; §35.8, §35.13 for KA-791; §35.8 for KA-792; §35.9,
  §35.14 for KA-793; §35.9, §35.13 for KA-794 and KA-795; §35.10, §35.14 for KA-796 and KA-798; §35.10 for
  KA-797; §35.10, §35.15 for KA-799; §35.7, §35.14 for KA-800; §35.2, §35.14 for KA-801; §35.3, §35.15 for
  KA-802; §35.3 for KA-803; §35.1, §35.13 for KA-804; §35.6 for KA-805.
- **`owner`.** The rule is the gate the row names, else the obligation or invariant it names, else the gate of
  its carrier. The PO or INV a row also names stays in its truth.
  - GATE-SPARK-15 for KA-785, -786, -787, -793 ("draws depend on address alone") and -795.
  - GATE-SPARK-16 for KA-788 and KA-789.
  - GATE-SPARK-05 for KA-790 and KA-792 (BD-179's precision battery to 1 km).
  - PO-NUM-01 for KA-791; PO-NUM-03 for KA-794.
  - GATE-SPARK-18 for KA-796 and KA-799.
  - GATE-TRAC-10 for KA-797 and KA-798 (BD-156's scaling gate).
  - GATE-SPARK-17 for KA-801 and KA-802.
  - INV-NUM-06 for KA-804.
  - KA-800, KA-803 and KA-805 name no gate, obligation or invariant, and their carriers carry none. Their owner
    is empty: they support no gate until one cites them, and their own tests run them.
- **`due`.** The milestone of the owner gate, else of the last carrier. This bead lands at M1, so no row is due
  earlier.
  - M1 for KA-804. N35-01's compile-fail test exists from M0 as a developer-level unit test.
  - M2 for KA-785 to KA-800 and KA-805.
  - M3 for KA-801 to KA-803 (GATE-SPARK-17, N35-06, N35-08).
  - The host forms of KA-793 and KA-794 (N35-24, N35-22, M1) are developer-level bindings before then.
- **`sites`.** A row whose site column reads "all tiers" lists every execution site of N38a-12's vocabulary. A
  site the machine lacks is reported `skipped: <site unavailable>` (N38a-13), never `pass`. `daemon` (KA-802)
  and `host` are as printed.

**Terms used in the rows.** A *world address* is the workspace's one `WorldAddr` (N35-22: run seed, namespace,
world, episode, fork; the plan's `(run_seed, world, episode)` is its first, third and fourth word); it
determines every random draw a world sees. A *reference workload* is Q (reference quadruped: 12 joints, 18 degrees of freedom, contact capacity
32, row capacity 160, step 2 ms) or H (reference humanoid: 29 joints, 35 degrees of freedom, contact capacity
64, row capacity 320). A *per-world digest* is the four-word integer mix of a world's canonical state slabs.
u is the single-precision unit round-off 2⁻²⁴ ≈ 6.0·10⁻⁸. κ₂ is the 2-norm condition number.

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Owner |
|---|---|---|---|---|---|---|
| KA-785 | One world run alone, in 100 random co-tenancies, under slot permutation, at two bucket capacities and on one and two shards | Identical per-world digests: given a policy (weights and normalizers), a world's trajectory is a function of its address, design, scene and task alone (INV-NUM-03) | exact, for 1,000 episodes of each reference workload on `batch-cpu` and each device family (GATE-SPARK-15) | G5 · all tiers | A kernel with a batch-wide floating reduction feeding state | BD-181, N35-15; N35-17 (the script) |
| KA-786 | Sum of the one active element −0.0 at two capacities | −0.0 both times, bit for bit (pattern 0x80000000): padding trails the active lanes and holds −0.0 for sums (INV-NUM-02) | exact (GATE-SPARK-15) | G5 · all tiers | Padding with +0.0: (−0.0) + (+0.0) = +0.0 (pattern 0x00000000); the values compare equal and the bits differ | N35-13, N35-15; BD-181, N35-17 |
| KA-787 | Two worlds observing 1 and 3 | Each world's actions equal those of its isolated run: observation normalizers are frozen within a collection segment and are constants of the policy in evaluation | exact (GATE-SPARK-15) | G5 · all tiers | A normalizer updated inside the segment: the batch mean 2 gives −1 and +1; the first world alone sees 0 | N35-14, N35-15; BD-181, N35-17 |
| KA-788 | Success predicate x ≥ 1 m; one placement ends at 1.0000001 m, the other at 0.9999999 m | State error 2·10⁻⁷ m, inside any envelope; the verdicts differ; the pair is reported as discordant and `boundary-sensitive` on both placements | exact (verdict) | G1 · all tiers | A check that compares states only | N35-09; BD-177, N35-12 |
| KA-789 | Decision-equivalence bound | The one-sided 95% exact binomial (Clopper–Pearson) upper bound on the discordant rate must be at most 1%. With zero discordant worlds the bound is 1 − 0.05^(1/n) ≈ 3/n: 1.0002% at n = 298, 0.9969% at n = 299, so 299 is the smallest admissible n (the round figure is 300). At n = 4,096 the bound is 0.9921% for 30 discordant worlds and 1.0201% for 31, so at most 30 are admitted. The net one-sided rate bound is at most 0.5% | as stated (TARGET, GATE-SPARK-16; OPEN-NUM-2) | G1 · host | A planted bias of one decision in fifty: 82 of 4,096 one-sided, bound 2.40%, must fail | N35-10, BD-177; N35-11 (freezes OPEN-NUM-2), N35-12 |
| KA-790 | Offset −2⁻²⁵ m in a cell | The centred convention (offset in [−4, 4) m) leaves the offset unchanged; it never adds the cell side to a small number, and every shift it does make is exact | exact | G0 · all tiers | The uncentred [0, 8) shift returns 8.0 (pattern 0x41000000): 8 − 2⁻²⁵ is not an `f32`, the spacing just below 8 being 2⁻²¹ | N35-30 (host form); N21b-37 (anchored positions), BD-157 (`K-anchor`) and BD-179 (device form) |
| KA-791 | Re-anchoring (PO-NUM-01) | For 4 ≤ abs(x) ≤ 16 subtracting or adding 8.0 is exact, by Sterbenz's lemma (the difference of two floats within a factor of two of each other is representable) | exact; exhaustive over the 2²⁵ values of the two binades [4, 8) and [8, 16) and two signs, on each backend | G0 · all tiers | none stated; the uncentred shift of KA-790 is used | N35-30 (host); N21b-37 (the exact re-anchoring), BD-157 and BD-179 (per backend) |
| KA-792 | Resolution of anchored positions | Single-precision spacing is 2^(e−23) in [2^e, 2^(e+1)): 2⁻¹⁷ m = 7.63 µm at 100 m from the origin, 2⁻¹⁴ m = 61.0 µm at 1 km; an offset in [−4, 4) m resolves at worst 2⁻²² m = 0.238 µm; an uncentred [0, 8) offset 2⁻²¹ m = 0.477 µm | exact | G1 · host | none stated | N35-30; N21b-37, BD-179 |
| KA-793 | Sequential draws with one extra leg | Addressed streams give both designs the same push: the push is the draw at (address, physical-time tick, stream `disturbance-process` with the disturbance's name hash as entity; N35-23, N35-24) for both | exact | G5 · all tiers | A sequential stream r₀, r₁, r₂: design A draws its push as r₀; design B, with one more per-leg draw first, draws it as r₁ | N35-24; BD-180 (`K-x-philox`), BD-200 |
| KA-794 | Integer-to-uniform conversion (PO-NUM-03) | u = (x ≫ 8)·2⁻²⁴ is exact and lies in [0, 1); the largest value is (2²⁴ − 1)·2⁻²⁴ = 0.99999994 (pattern 0x3F7FFFFF) | exact; exhaustive over the 2²⁴ values of x ≫ 8 | G0 · all tiers | none stated; N35-22 adds one: converting the full 32-bit word, `x as f32 · 2⁻³²`, returns 1.0 for x = 0xFFFFFFFF | N35-22; BD-180 (device form) |
| KA-795 | Draw tables across slots, buckets, shards and backends | A random draw is a function of (address, step, stream) alone (INV-NUM-04); adding a stream changes no draw of any other stream | exact (GATE-SPARK-15) | G5 · all tiers | none stated; N35-16 adds two: draws keyed by slot; a renumbered stream registry | N35-16; N35-22, N35-23, BD-180 |
| KA-796 | Computed Cholesky factor of D·H·D against D times the computed factor of H, D a diagonal of powers of two (PO-NUM-02) | Bit-identical, absent overflow and underflow | exact, on 10⁴ matrices per backend (GATE-SPARK-18) | G3 · all tiers | A scaling by 10 instead of a power of two | BD-179; N21b-38 (the conditioning certificate, PO-NUM-02); N35-30 (host form) |
| KA-797 | Exact diagonal scaling by `K-scale` | D_ii = 2^(−⌈e_i/2⌉) with e_i the binary exponent of H_ii; the scaled diagonal lies in [½, 2); no rounding is introduced. Example: H_ii = 10⁻⁸ has e = −27, D = 2¹³ = 8,192, scaled diagonal 0.671 | exact | G0 · all tiers | none stated | BD-156; N35-30 (host form) |
| KA-798 | H = diag(1, 10⁻⁸) with residual (0, 10⁻⁷) | The error in the second unknown is 10⁻⁷/10⁻⁸ = 10. In scaled variables the residual is (0, 8.192·10⁻⁴) with the power-of-two rule (the illustration D = diag(1, 10⁴) gives (0, 10⁻³)), and a tolerance test at 10⁻⁶ fails | exact (verdict) | G1 · host | The unscaled absolute test at 10⁻⁶ passes (‖r‖ = 10⁻⁷) | N35-30; BD-156 |
| KA-799 | Conditioning figure κ̂ = (max L̂_ii / min L̂_ii)² | A lower bound on κ₂ of the scaled matrix: κ̂ never exceeds the κ₂ computed in `f64`; the relative error of the `f32` Newton step against `f64` is bounded | at most 10·κ₂·u on 10⁴ contact states per reference robot; at most 10⁻³ on near-rigid states under the `stiff` profile (GATE-SPARK-18) | G1 · all tiers | none stated | BD-179, N21b-38; N35-29; N35-27 (the certificate block) |
| KA-800 | The set {0, 3, 5, 9} histogrammed in two orders | Identical histograms: a bin is the sign, the exponent and the top five mantissa bits of the `f32` pattern (index = pattern ≫ 18: 0 → 0, 3 → 4,112, 5 → 4,136, 9 → 4,164); a bin is at most 2⁻⁵ = 3.125% wide ("at most 3.2%"); counts are integers; merging is addition | exact | G5 · all tiers | A closest-pair two-centroid sketch: order 0, 3, 5, 9 ends as (8/3 with weight 3; 9 with weight 1), order 0, 9, 5, 3 as (1.5 with weight 2; 7 with weight 2) | N35-21; N29a-13 (Chronicle's integer histograms) |
| KA-801 | A policy trained under `fast-training` evaluated twice under `device-replay` | Identical digests | exact (GATE-SPARK-17) | G5 · all tiers | Citing its training return is refused (`FROB-REF-861`) | N35-06, BD-269; N35-32 (the script) |
| KA-802 | Licence refusals on planted cases | A class D4 arm in a paired comparison and a cross-family pair demanded as paired without class D1 are refused (`FROB-REF-561`); attribution below class D2 is refused (`FROB-REF-562`); a cited training return is refused (`FROB-REF-861`) | exact (GATE-SPARK-17) | G1 · daemon | none stated; BD-269 adds the acceptance controls (the same requests at an admissible class succeed) | N35-07, BD-269; N35-04, N35-32 |
| KA-803 | Pairing arithmetic (illustrative values) | The variance of a paired difference scales with 1 − ρ: a fall in pair correlation from 0.95 to 0.50 needs (1 − 0.50)/(1 − 0.95) = 10 times the episodes; a difference of one unit in the last place reaches order one after about ln(2²⁴)/λ = 16.64/λ seconds | exact | G1 · host | none stated | N35-08; N35-07 |
| KA-804 | Class of a combined result | The weakest class among the kernels, placements, profile and inputs that produced it (INV-NUM-06); no conversion upward compiles | exact (compile-fail test) | G0 · host | none stated; N35-01 adds one: a planted `From<D3> for D2` must fail the build check | N35-01; N35-02 (the classifier) |
| KA-805 | Lint over dialect sources and the Rust of `frob-kernels` | `mul_add`, `sin`, `cos`, `exp`, `ln`, `powf` and their kin are rejected at build time (`FROB-REF-251`); `sqrt` is correctly rounded and allowed | exact | G0 · host | none stated; N35-18 adds the corpus of one file per banned construct, each of which must be rejected | N35-18, BD-013 |

**Registry check.** `frob-conformance` loads the rows through N38a-12's loader; a row whose derivation file
names no owner test, names one that does not exist in the workspace, or whose `truth.value` differs from the
literals above fails the check. Identifiers are append-only and never renumbered; a value is never rewritten in
transit: where a recomputation disagrees, the row keeps the printed value in `printed` and enters the
reconciliation table (N38a-12, rule 5). KA-789's round figure 300 and KA-798's illustration (0, 10⁻³) are part of
the printed truth, not disagreements, and are carried in the derivation files.

**Rows for which no negative control is stated.** Ten rows state none: KA-791, -792, -794, -795, -797, -799,
-802, -803, -804 and -805. Their `control.text` is `none stated` with the owner's addition where the table
names one, and N38a-12 reports them `obligation: control` (never `eligible`, and no `value-check` exemption)
until a record with `seed = "KA-nnn"` exists. This bead sets the control of KA-791: its `control.text` reads "the
uncentred [0, 8) shift of KA-790 applied by the re-anchoring under test" (a stated control, no longer `none
stated`). The seeding tool of N38b-05 then issues its record with `seed = "KA-791"`. The plant is KA-790's, but
the target is the re-anchoring rather than the offset convention, so it is a new control under step 3. This is
the first record by N38b-34's table, and N38b-34's is the second. The owner beads register theirs: KA-794 (N35-22), KA-795 (N35-16), KA-804 (N35-01), KA-805 (N35-18). KA-792, -797, -799, -802 and -803
take N38b-34's proposals at M2 (BD-269's acceptance controls for KA-802 succeed on admissible requests and are
not negative controls).

## Acceptance criteria

- [ ] `known_answers.toml` holds 21 rows KA-785 to KA-805 with the fields above, valid under N38a-12's loader (which rejects a duplicate, a missing field and an unknown tier); no chapter file exists.
- [ ] Each row's `section`, `owner`, `due` and `sites` equal the lists above (KA-800, KA-803 and KA-805 with an empty owner).
- [ ] Every row's derivation file names an owner test that exists; `cargo test -p frob-conformance --test ka_numerics` lists 21 bound rows and 0 unbound once the owners have landed, and reports the rows not yet bound by number before that (`not due` before their milestone, `unbound` after it, N38a-13).
- [ ] The literals in `truth.value` equal those in this bead: 0x80000000 and 0x00000000 (KA-786); 299, 30, 31, 0.9921%, 1.0201%, 82, 2.40% (KA-789); 0x41000000 (KA-790); 2²⁵ (KA-791); 2⁻¹⁷, 2⁻¹⁴, 2⁻²², 2⁻²¹ (KA-792); 0x3F7FFFFF (KA-794); 8,192 and 0.671 (KA-797); 8.192·10⁻⁴ (KA-798); 0, 4,112, 4,136, 4,164 (KA-800); 10 and 16.64 (KA-803).
- [ ] After the seeding tool of N38b-05 is re-run, each of the eleven rows with a stated negative control has its `NC-` identifier in `control.nc`, and the control is recorded as failing its test; KA-791's record exists with `seed = "KA-791"`; the other nine rows that state none are reported `obligation: control` until their owners' records exist.
- [ ] The generated appendix table for this chapter is byte-identical on regeneration.

## Test plan

### Unit tests (`cargo test -p frob-conformance --test ka_numerics`, `crates/frob-conformance/tests/ka_numerics.rs`)
- `ka_numerics_rows_load`: 21 rows parse; identifiers are consecutive from 785 to 805.
- `ka_numerics_sections_owners_due`: each row's section, owner, due and site list equal the lists above; no row is due before M1.
- `ka_numerics_values_match_literals`: each `truth.value` table equals a compiled-in copy of the literals above (the copy is in the test, not in the loader).
- `ka_numerics_owner_tests_exist`: every owner test named in the derivation front matter resolves to a test in the workspace's test listing.
- `ka_numerics_controls_registered`: the eleven rows with a stated control (KA-785 to -790, -793, -796, -798, -800, -801) each name a registered control; KA-791's record exists; a row of the ten without a record is `obligation: control`, never `eligible`.
- `ka_row_value_not_rewritten`: a planted edit of an existing row's `truth` fails `ka_numerics_values_match_literals`, and a planted renumbering fails the loader's append-only rule.

### End-to-end script and logging
`scripts/e2e/numerics_ka_registry.sh` checks the registry and nothing a later bead delivers.
- **Header and log.** Its header is `bead=N35-KA milestone=M1 labels=spark,registry gates=`. It sources the
  STD-E2E helper and writes NDJSON to `target/e2e-logs/numerics_ka_registry/<utc>.ndjson` with a transcript.
- **Steps.** It runs `cargo test -p frob-conformance --test registry_check` and `--test ka_numerics`.
- **Metric lines.** It logs:
  - the row count (21);
  - per row: the computed eligibility, owner, due, the `control.nc` count after seeding, and whether the owner
    test is bound, `not due` or `unbound`;
  - the digest of `known_answers.toml`.
- **Planted scratch defects.** On scratch copies it plants KA-786's bit pattern rewritten to 0x00000000, KA-791's
  control text deleted, and KA-789's 299 changed to 300, and requires each to be caught.
- **Failure policy.** Rows whose owners have not landed are logged, not failed (KNOWN_DRIFTS rule 7).
- **Line fields.** Every line carries `ts`, `script`, `bead`, `step`, `event`, the exact command, the exit code
  and the wall time; on failure it also logs the first differing value. The `summary` gives the assertions
  passed and failed and `gates: []`.

The rows themselves are exercised by `scripts/e2e/num_known_answers.sh` (N35-30) and by the gate
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
