---
key: "N17-KA"
title: "frob-conformance: known-answer registry for Phylum (KA-365 to KA-386) with derivations and controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "phylum", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: []
related: ["N17-CX", "N17-REF", "BD-144"]
gates: []
plan_refs: ["§17.18", "Appendix M.16", "§38.6", "§38.1"]
---
## Background

Phylum is the subsystem (crate `frob-phylum`, layer 3) that makes a population of different robots, not one model,
the unit of batched simulation: cohorts, size buckets, family lanes, a morphology generator, campaign quotienting,
quality-diversity maps and paired design gradients. Almost every claim Phylum makes is either exact arithmetic
(orbit counts, padding bounds, class counts) or a bit-equality (a world's digests do not depend on its cohort).
That makes it unusually testable: the truths below are closed forms or enumerations, and a wrong implementation
cannot hide behind a tolerance.

The project's rule is that a known answer is a row in a registry with a derivation on file and a pinned truth that
was never regenerated from the candidate implementation; the printed appendix of known answers is generated from
the registry, never the reverse. A KA number is never reused and never renumbered; new rows append. A second rule
(ADR-GAUNT-03) is that a gate is registered with its failing control before its passing test is written, because a
gate that no deficient implementation can fail validates nothing. Nine of the 22 Phylum rows were stated without a
negative control. This bead closes that gap by registering a planted defect for each (labelled DESIGN below).

This bead is the single place where the 22 rows live as data. Mechanism beads cite the numbers and restate the
values they need inline; this bead owns the registry rows, the derivation files and the cross-row checks.

## Scope

In: the 22 rows KA-365 to KA-386 in `frob-conformance/registry/known_answers.toml`; one derivation file per row in
`docs/derivations/phylum/`; a registered negative control per row in `frob-conformance/registry/controls.toml`; a
host-only self-test that recomputes every arithmetic truth from first principles in exact rationals or by brute
force. Out: the mechanisms that must reproduce the rows (the beads named in the last column); the counterexample
registry `N17-CX` and the refusal registry `N17-REF` (written by the other Phylum drafting unit).

## Technical approach

Registry row format (DESIGN, following the registry of `frob-conformance`): `id`, `section`, `case`, `truth`
(value or formula, exact where possible), `tolerance`, `tier` (G0 proof or property, G1 host known answer, G3
metamorphic, G5 determinism lane, with the profile or back end), `control` (id in `controls.toml`), `derivation`
(path), `carried_by` (bead keys), `fixture` (identity, shared with any construction row that states the same
physical case: one fixture identity per physical case).

| KA | Case | Truth | Tolerance | Tier | Negative control (planted defect) | Carried by |
|---|---|---|---|---|---|---|
| KA-365 | Hexapod sweep: 10 leg types on 6 slots under the dihedral group of order 12 (PO-PHYLUM-01) | (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = 86,185 canonical representatives, reduction 11.6; enumerated count equals the cycle-index count | exact | G1 · host | Counting under the rotation subgroup only gives (10⁶ + 2·10 + 2·10² + 10³)/6 = 166,870 | N17a-23, BD-285 |
| KA-366 | Same sweep when sensors or task leave only the sagittal mirror | (10⁶ + 10³)/2 = 500,500, reduction 2.0 | exact | G1 · host | Quotienting under the mechanical group with a front-only sensor mounted; must refuse `FROB-REF-161` | N17a-23, BD-285 |
| KA-367 (≡ KA-127, one fixture) | Ballast: 8 identical slugs in 24 collinear slots, entering rigid-body dynamics only through count, Σx, Σx² | C(24, 8) = 735,471 placements; 24,546 distinct (Σx, Σx²) pairs = 24,546 mission-digest classes; reduction 30.0 | exact | G1 · host | Two placements with equal Σx and different Σx² merged (slots {1, 4} against {2, 3} beside six common slugs: Σx = 5 twice, Σx² = 17 against 13) | N17a-26, N17a-25, BD-285 |
| KA-368 | Padding overhead Ω_p = L_maxᵖ/E[Lᵖ], r = E[L]/L_max, three populations | Half 18, half 35: r = 0.76, Ω₁ = 70/53 = 1.32, Ω₂ = 2450/1549 = 1.58, Ω₃ = 85750/48707 = 1.76, bounds 1.32 and 2.30. 90% at 18, 10% at 35: r = 0.56, 350/197 = 1.78, 12250/4141 = 2.96, 428750/95363 = 4.50, bounds 1.78 and 5.61. 89% at 18, 10% at 35, 1% at 66: r = 0.31, 3300/1009 = 3.27, 217800/22721 = 9.59, 14374800/617647 = 23.3, bounds 3.27 and 35.0 | exact rationals, printed to three figures | G1 · host | DESIGN: E[L]ᵖ used in place of E[Lᵖ] (gives 2.30 instead of 1.76 in row one) | N17a-04, BD-144, BD-171 |
| KA-369 | Padding bound for 1 ≤ p ≤ 3 (PO-PHYLUM-02) | 1/r ≤ Ω_p ≤ r⁻ᵖ ≤ r⁻³; lower from Lᵖ ≤ L_maxᵖ⁻¹·L, upper from Jensen E[Lᵖ] ≥ E[L]ᵖ | exact inequality | G0 · host | DESIGN: L_min used in place of L_max (then Ω_p < 1 ≤ 1/r on every mixed population and the lower bound fails) | N17a-04 |
| KA-370 | Bucket set with ratio q between successive sizes | Overhead inside a bucket ≤ q (linear) and ≤ q³ (cubic): 1.25 and 1.95 at q = 1.25; ⌈ln(66/12)/ln 1.25⌉ = 8 buckets span 12 to 66 degrees of freedom | exact | G1 · host | DESIGN: q² reported as the cubic cap (1.56) | N17a-04, N17a-05 |
| KA-371 | Worked card: 8,192 worlds, 90% at 18 and 10% at 35; stage shares ½ cubic, 3/10 quadratic, 1/5 linear; τ = 0.5 µs; c_L = 0.45 ms per plan (illustrative inputs) | Ω = 0.5·4.50 + 0.3·2.96 + 0.2·1.78 = 3.5. One bucket 14.8 ms, two buckets 5.0 ms. 512 worlds: 1.35 ms against 1.16 ms. 128 worlds: 0.67 ms against 0.96 ms. Crossover N = c_L/(τ·(Ω − Ω_b)) = 360 worlds; c_L ten times smaller: 36 worlds | exact arithmetic on illustrative inputs | G1 · host | DESIGN: B launches charged in place of B − 1 in the pay-off inequality (crossover 720) | N17a-04, N17a-05 |
| KA-372 | Mixing ratio against D designs run one after another with W worlds each: D·(c_L + W·τ)/(c_L + Ω·D·W·τ) at D = 512, Ω = 1.3, card figures | 40.6 at W = 16; 11.3 at 64; 3.46 at 256; 1.44 at 1,024; 1.11 at 2,048 (printed as about 40, 11, 3.5, 1.4, 1.1) | exact arithmetic on illustrative inputs | G1 · host | DESIGN: Ω dropped from the denominator (ratio never falls toward 1) | N17a-04 |
| KA-373 | Speedup bound: fraction f of matched wall time improved by r with relative overhead h | speedup ≤ 1/(1 − f + f/r + h): 6.7 for f = 0.85, r → ∞, h = 0; 13.5 for f = 0.97, r = 40, h = 0.02; 1.10 to 1.32 for the family lane at f = 0.15 to 0.35, r = 4, h = 0.02 | exact | G1 · host | A pruning ratio multiplied by a speedup measured on work the pruning already removed (100 candidates of 10 s, 90 pruned, survivors 2× faster: 1,000/50 = 20, not 10 × 5 = 50) | BD-356, N17a-11, N17a-13 |
| KA-374 | Ceiling of per-design kernel specialization from a published step trace (53 of 262 ns, 20% of the step) | 262/209 = 1.25 for an infinitely fast specialized recursion; 1.11 for one twice as fast; 1.19 on a manipulation scene with a 16% share | exact arithmetic on a FACT to be re-read at its primary source | G1 · host | DESIGN: ceiling computed as 1 + s (1.20) in place of 1/(1 − s) | N17a-15, N17a-16 |
| KA-375 | Each of 512 designs in a mixed cohort against the same world in a homogeneous batch of its own model, at three bucket sizes, under slot dispatch with 100 random co-tenancies (INV-PHYLUM-02, GATE-PHYLUM-02) | Identical digests per world, on `batch-cpu` and per GPU back end | exact (digest) | G5 · lockstep, cuda, metal | A reduction tree shaped by capacity (in `f32`: a = 10⁸, b = −10⁸, c = 1 gives (a + b) + c = 1 and a + (b + c) = 0) | N17a-10, BD-170, BD-183 |
| KA-376 | Slots and co-tenants of a cohort permuted (INV-PHYLUM-01) | Envelopes compare equal after sorting by world address; slot, cohort and arrival order appear in no result | exact | G3 · lockstep, cuda, metal | DESIGN: a slot index written into a result record | N17a-03, N17a-10 |
| KA-377 | Mirror swap of a design under an element of Γ_obs | Every distributional result reused; per-address results reused after the address is mapped through the group action (g·d in world w behaves as d in world g⁻¹·w) | exact | G3 · host | A shared-bus edit reused; an unmapped per-address result reused | N17a-24, BD-286 |
| KA-378 | Coupled bus: two fingers with identical local summaries on a bus limited to one unit of current | Zero feasible one-tick schedules and two feasible two-tick schedules; the design is feasible | exact (counts) | G1 · host | Pruning by summed peak demand | BD-286, N17a-26 |
| KA-379 | Invalidation contract under 10⁴ random edit sequences (GATE-PHYLUM-05) | Results equal full recomputation; the positive control reuses; a shared-bus edit invalidates; a reversed port sign is refused | exact | G3 · host | The three negative controls of the contract (shared-bus edit, reversed port sign, coupled bus) | BD-286 |
| KA-380 | 10⁴ sampled pairs of designs inside one mission-digest class, replayed (GATE-PHYLUM-01, INV-PHYLUM-03) | Identical digests in every pair: zero false quotients | exact (digest) | G5 · lockstep | The lidar and equal-Σx controls; one false quotient surviving a repair cycle fires KILL-QUOTIENT | N17a-28, BD-285 |
| KA-381 | Paired finite-difference gradient on a ballistic body, range R = v²·sin 2α/g | ∂R/∂v = 2·v·sin 2α/g, recovered to O(δ²) | within 1% (GATE-PHYLUM-06) | G1 · lockstep | A binary success metric admitted; must refuse `FROB-REF-540` | BD-353 |
| KA-382 | Paired stencil on a bouncing body near grazing | Each pair flagged at the first step where the active contact sets of its two worlds differ, and truncated there (`truncated-horizon`) | exact (flag) | G1 · lockstep | Divergence ignored and the full horizon averaged | BD-353 |
| KA-383 | Stencil size for 20 parameters and 64 seeds | (2·20 + 1) × 64 = 41 × 64 = 2,624 worlds | exact | G1 · host | DESIGN: the centre world omitted (2·20 × 64 = 2,560) | BD-353 |
| KA-384 | Worlds of one stencil seed (INV-PHYLUM-07) | Every world shares every random draw: stream addresses compare equal | exact | G5 · lockstep | A planted unshared stream | BD-353 |
| KA-385 | Morphology generator (INV-PHYLUM-06, GATE-PHYLUM-07) | The same `(GeneratorId, seed, index)` gives the same `DesignId` on Linux and macOS; log-probabilities exact and summing to one on enumerable sub-grammars; no orbit straddles a `DesignSplit` | exact | G5 · host | A generator that reads the wall clock or a hash-map order | BD-287, N17a-19, N17a-21 |
| KA-386 | Merge of quality-diversity maps, each cell keeping the maximum under the total order (score, then `DesignId`) (PO-PHYLUM-03) | Commutative, associative and idempotent | exact | G0 · host | DESIGN: ties broken by arrival order | BD-342 |

Small-case truths added for fast unit tests (DESIGN; each brute-forced once and pinned): two leg types on the
hexapod under the group of order 12 give 13 orbits, three give 92; a quadruped with slots at four corners under
C₂ × C₂ gives (k⁴ + 3·k²)/4 (27 at k = 3) and under the left-right mirror alone (k⁴ + k²)/2 (45 at k = 3); three
slugs in eight slots give 56 placements in 54 classes, the first collision being {1, 5, 6} and {2, 3, 7} with
Σx = 12 and Σx² = 62; four slugs in twelve slots give 495 placements in 395 classes.

Derivations. Each row has a file `docs/derivations/phylum/KA-<n>.md` with the formula, every symbol defined, the
substituted numbers and the independent method used to pin the truth: exact rational arithmetic for KA-368 to
KA-374 and KA-383, brute-force enumeration for KA-365 to KA-367, the closed form for KA-381. The self-test
recomputes each pinned truth by that independent method; it does not call `frob-phylum`.

## Acceptance criteria

- [ ] All 22 rows exist in `known_answers.toml` with id, case, truth, tolerance, tier, control, derivation and
      carrying beads; the registry lint rejects a renumbered or reused id.
- [ ] Every row has a registered control in `controls.toml`, including the nine rows that had none (KA-368, -369,
      -370, -371, -372, -374, -376, -383, -386), each marked DESIGN as added here.
- [ ] `cargo test -p frob-conformance --test known_answers -- phylum` recomputes the arithmetic truths (KA-365 to
      KA-374, KA-383) by brute force or exact rationals and matches the pinned values exactly: 86,185; 500,500;
      735,471 and 24,546; the nine Ω fractions; 360; 40.6, 11.3, 3.46, 1.44, 1.11; 6.7, 13.5, 1.10, 1.32; 1.25, 1.11,
      1.19; 2,624.
- [ ] KA-367 and KA-127 resolve to one fixture identity; changing one constant changes both rows.
- [ ] The printed known-answer table for Phylum regenerates from the registry byte for byte (generated-artifact
      check).
- [ ] Each mechanism bead named in "Carried by" has a test that cites its KA id; the lint lists any row cited by no
      test at that row's milestone.

## Test plan

### Unit tests

- `ka_phylum_orbit_counts_bruteforce`: enumerate all 10⁶ labelings of six slots, canonicalise under the 12 elements
  of the dihedral group, count distinct canonical forms; expect 86,185. Repeat under the mirror alone; expect
  500,500. Small cases 13 and 92.
- `ka_phylum_ballast_classes_bruteforce`: enumerate C(24, 8) placements, collect (Σx, Σx²); expect 735,471 and
  24,546; small cases 56 → 54 and 495 → 395.
- `ka_phylum_padding_fractions`: the nine Ω values as exact fractions listed in the table; bounds 1/r and r⁻³.
- `ka_phylum_card_and_mixing`: 14.8, 5.0, 1.35, 1.16, 0.67, 0.96 ms; crossover 360 and 36; five mixing ratios.
- `ka_phylum_bounds`: 1/(1 − 0.85) = 6.67; 1/0.07425 = 13.47; 1/0.9075 = 1.102; 1/0.7575 = 1.320; 262/209 = 1.2536;
  262/235.5 = 1.1125; 1/0.84 = 1.190.
- `ka_registry_lint_phylum`: ids unique, contiguous 365 to 386, every row has a control and a derivation path that
  exists.

### End-to-end script and logging

`scripts/e2e/phylum_known_answers.sh` runs the registry self-test and the lint, then asks the built `frob` binary
for each known answer that a verb can already produce at the milestone in force (`frob phylum plan --json` for
KA-368 to KA-372 from M2; `frob phylum quotient --json` for KA-365 to KA-367 from M3) and compares the envelope's
figures with the registry. It sources the shared helper of `STD-E2E` and writes newline-delimited JSON to
`target/e2e-logs/phylum_known_answers/<utc-timestamp>.ndjson` with a transcript beside it. Each line carries `ts`,
`script`, `bead` (`N17-KA`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time, the envelope's `evidence` block (claim, class, tier, determinism, identities,
seed), the measured value with its unit beside the pinned truth, and on failure the first differing value and the
path of the derivation file. Rows whose verb does not exist yet are logged as `skipped` with the milestone, never
as passed. The final `summary` line gives assertions passed, failed and skipped.

### Known answers and negative controls

The rows are the known answers. Controls: for each row the planted defect in the table is applied to the
independent recomputation (not to the product code) and the self-test must report a mismatch; a control that
passes is a test failure. Determinism lane: the self-test output is byte-identical twice on one machine, in debug
and release, and on Linux and macOS (class D0).

## Considerations

- KA-374 rests on a published incumbent step trace (53 of 262 ns). It is a FACT to be re-read at its primary source
  before any gate cites it; the row's arithmetic is exact given the inputs.
- KA-371 and KA-372 use illustrative inputs (τ, c_L, stage shares are HYPOTHESES). They test the arithmetic, not
  the machine; the measured values replace the inputs in the planner's cost model, not in these rows.
- The plan's type sketch of the shape class prints the default bucket set once as {12, 18, 24, 32, 48, 64}; the
  chapter, the device chapter and the open-question register print {12, 18, 24, 36, 48, 64}. The registry uses 36.
- Rows KA-375, KA-376, KA-380 and KA-384 need devices or replay and become live at M2 or later; the registry
  records the milestone per row so that a missing test is reported at the right exit, not earlier.

## References

Plan §17.18, Appendix M.16, §38.1, §38.6; PO-PHYLUM-01 to -03; INV-PHYLUM-01, -02, -03, -06, -07; GATE-PHYLUM-01,
-02, -05, -06, -07; ADR-GAUNT-02, -03, -06.
