---
key: "N21-KA"
title: "registry: Spark known answers KA-525 to KA-552 with truth, tolerance, tier, negative control and carrier"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-14"
deps: ["BD-002", "BD-025", "N38a-12", "N38b-05", "N40-02", "STD-E2E", "STD-TEST"]
needs: []
related: ["N17a-04", "N21-CX", "N21-REF", "N21a-05", "N21a-10", "N21a-21", "N21a-23", "N21a-25", "N21b-07", "N21b-16", "N21b-18", "N21b-33", "N21b-36", "N21b-40", "N21bp2-01", "N21c-02", "N21c-08", "N21c-09", "N21c-25", "N21c-26", "N21c-30", "N21c-31", "N21c-32", "N38-KA", "N38a-10", "N38a-12", "N38b-27", "N38b-32"]
gates: []
plan_refs: ["Appendix M.20", "§21.23", "§21.24", "§38.6"]
---
## Background

A known answer is a case whose truth is known by hand, in closed form or by exact arithmetic, checked against an
oracle that is never regenerated from the candidate. The verification program runs known answers at tier G1, with
some rows at G0 (laws and self-tests), G4 (chaos: cancellation, injected faults) and G5 (determinism: twice on one
machine, across worker counts, debug and release, across platforms and backends, each against its class). The
plan's rule is that a gate that nothing can fail validates nothing, so every row with a plausible wrong
implementation names a **negative control**: a planted defect the test must catch.

Spark is the subsystem between a derived kernel and a device result: the kernel library, the kernel intermediate
representation (IR) and its emitters, the CUDA and Metal backends, the device arena, the placement planner, the
binary kernel cache, the hardware fingerprint and the CPU-versus-device check. Its 28 known answers are split over
many beads owned by three drafting units. This bead enters all 28 into the known-answer registry of
`frob-conformance` as data, so that each mechanism bead can cite a row by number and no row is lost. It implements
no kernel and no backend.

Terms used in the table. *Dialect*: the common subset of CUDA C++ and Metal Shading Language in which kernel
skeletons are written once for both devices. `batch-cpu`: the CPU backend whose kernels are Rust functions; it is
the permanent reference. Determinism classes: D0 exact (integers); D1 the same bits on `batch-cpu`, CUDA and Metal;
D2 the same bits run to run on one device family; D3 agreement inside a recorded tolerance. *ulp*: unit in the last
place. `trj`: the Linux machine with two RTX 4090 cards; "this Mac": the M5 Ultra with a Metal device.

## Scope

In: the 28 rows below as records of the suite's one known-answer file in the row schema of N38a-12, with their
derivation files; a completeness test; the reconciliation notes at the end; the registry's own end-to-end check.
Out: the fixtures and tests themselves (the carrier beads); the row schema, loader and eligibility rules
(N38a-12); the `NC-` records of the stated controls, which the seeding tool of N38b-05 derives from the control
cells; the counterexample registry of the chapter (`N21-CX`) and the refusal registry (`N21-REF`), which another
drafting unit writes.

## Technical approach

**Records.** One `[[ka]]` record per row in the suite's one physical file,
`crates/frob-conformance/registry/known_answers.toml` (DESIGN correction: an earlier draft named a chapter file
`registry/known_answers/spark.toml`, which N38a-12's one-file rule refuses with `ka-second-file`). Fields are
those of N38a-12: `id`, `section` (with `owner` = the row's gate where it has one), `fixture`, `same_as`, `case`,
`truth`, `tolerance` (`exact`, `abs`, `rel`, `printed-digits`, `gate` or `to-set`; a determinism-class rule such as
"bit-equal for D1, at most 4 ulp for D3" is entered as `gate` with the rule in the derivation file), `tier`,
`sites`, `control` (`text` from the table, including the controls "this registry adds"; `nc` filled by the
seeding tool of N38b-05), `derivation` (`docs/derivations/<fixture>.md`), `label` (DESIGN arithmetic; FACT for the
pilot counts of KA-529 and KA-530 and the starting values of KA-546, whose derivation files name the plan-time pilot
they quote: `pilot = "PILOT-PT-15"`, the boundary screen re-run by BD-030, for KA-529 and KA-530, and
`pilot = "PILOT-PT-S2"`, the Metal working-set limit and largest buffer re-run by N00-35, for KA-546; N38a-10, so
that a FACT row is `obligation: rerun` until its pilot is `reproduced`), `due`, `printed` (14,292 for KA-552, whose
exact value is 14,291.67). The table's "Gate" column is the row's `owner`; its "Carrier" column goes into the
derivation file's front matter (for each KA it serves, the test that checks it and the carrier bead); the
reconciliation notes below go into the derivation files of KA-547 and of KA-529 to KA-531. A row whose tolerance
the plan leaves as "to set" is entered as `tolerance = { kind = "to-set" }`; its carrier bead must replace it with
a number and record that number as a TARGET before the row can be cited. Sites: `host` (a CPU test in exact or
`f32` arithmetic), `cuda`, `metal`, `all tiers`.

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Gate | Carrier |
|---|---|---|---|---|---|---|---|
| KA-525 | One dialect source for `K-cay-step` (the Cayley group update) compiled for `batch-cpu`, CUDA and Metal | Matches the Rust reference inside its determinism class on all three backends | Bit-equal for D1; at most 4 ulp per kernel output for D3 | G1 · cuda, metal | A prelude with floating-point contraction switched on (plant `N21c-30:spark.contraction-prelude`) | GATE-SPARK-00, GATE-SPARK-02 | BD-020, BD-176; N21a-25 (CPU half) |
| KA-526 | Contraction detector in `f32`: (1 + 2⁻¹²)·(1 + 2⁻¹²) − (1 + 2⁻¹¹) | 0 without contraction (the product 1 + 2⁻¹¹ + 2⁻²⁴ is a tie and rounds to even, 1 + 2⁻¹¹); 2⁻²⁴ with contraction | exact | G0 · cuda, metal | The fused form itself (`fma(a, a, −c)`; plant `N21c-30:spark.fused-form`) | GATE-SPARK-01 | BD-017; BD-013 |
| KA-527 | Double-float canary, sum (`K-canary`) | The error-free sum of 1 and 2⁻³⁰ returns the pair (1, 2⁻³⁰); a compiler that simplifies the error term algebraically returns (1, 0); a failed canary refuses the device for double-float kernels (`FROB-REF-609`) | exact | G0 · cuda, metal | The folded sum, whose error term is replaced by 0, returns (1, 0) and must be caught (plant `N21c-30:spark.folded-eft`). The plan's control, a build with fast math on, is kept as a recorded experiment, not as a control (note 8) | — | N21a-21 (CPU and IR lowering); N21b-36 (device canaries); N21c-02 (fingerprint canary probe) |
| KA-528 | Double-float canary, product by the split product (splitter 4097 = 2¹² + 1) | The error-free product of 1 + 2⁻¹² with itself returns (1 + 2⁻¹¹, 2⁻²⁴), since the exact product 1 + 2⁻¹¹ + 2⁻²⁴ is a tie that rounds to even | exact | G0 · cuda, metal | The folded product, whose error term is replaced by 0, returns (1 + 2⁻¹¹, 0) and must be caught (plant `N21c-30:spark.folded-eft`). The plan's control, a build with fast math on, is kept as a recorded experiment, not as a control (note 8) | — | N21a-21; N21b-36 (device canaries); N21c-02 (fingerprint canary probe) |
| KA-529 | Strict predicate 2²⁴ < 2²⁴ + 1 (16,777,216 < 16,777,217) | True in the exact and guarded kernels | exact; identical bytes on `batch-cpu`, both RTX 4090 and Metal | G0 · all tiers | The bare `f32` kernel answers false, since 16,777,217 rounds to 16,777,216 (plant `N21c-30:spark.bare-f32-strict`) | GATE-SPARK-08 | BD-030, BD-053, N21b-40 |
| KA-530 | Boundary corpus of 2 × 65,536 rows with exact ties | Exact kernel: zero errors, identical bytes on every backend. Guarded kernel: zero errors after fallback, fallback fraction reported (recorded pilot: guard deferred whenever the absolute residual was at most 2⁻²¹; it decided none of the dense grid on the device and 58,254 of 65,536 rows of the wider grid) | exact | G0 · all tiers | The bare `f32` kernel must mislabel at least one row (pilot: 452 false inclusions and 23,438 false exclusions; plant `N21c-30:spark.bare-f32-corpus`) | GATE-SPARK-08 | BD-030, N21b-40 |
| KA-531 | Guard of a guarded predicate (PO-SPARK-02) | If abs(r̂) > ε the sign of the computed residual r̂ is the sign of the exact residual r; for the pilot's conversion and addition sequence the rounding error is at most 5·2⁻²⁵, below the deferral threshold 2⁻²¹ | exact (certified bound; exhaustive check on the boundary corpus) | G0 · host | none stated; this registry adds: a bound scaled by ½ must be rejected by the exhaustive check | — | BD-053 |
| KA-532 | Stride conventions on a 2 × 2 contiguous `f32` view: byte strides (8, 4) against element strides (2, 1) | Index [1, 1] reads element 3; a view crossing the boundary carries its stride unit and signedness, is materialized with the copy charged to the receipt, or is refused under a zero-copy contract (`FROB-REF-617`) | exact | G1 · host | A verbatim stride copy onto a 16-element buffer reads element 8 + 4 = 12, in bounds (plant `N21c-30:spark.verbatim-strides`) | — | BD-036, N21a-05 |
| KA-533 | Mask with an infinity in the unselected arm | `select(pred, a, b)` returns the selected arm unchanged | exact | G0 · all tiers | The arithmetic blend m·a + (1 − m)·b returns NaN, since 0·∞ is NaN (plant `N21a-10:blend-mutant`) | — | BD-018, N21a-10 |
| KA-534 | Strict placement with one device kernel withheld, for every stage | Refusal by the planner (`FROB-REF-604`) before any world is stepped; zero episodes; receipt counters for transfers and synchronizations match an instrumented shim | exact for the refusal; within one count for the counters | G1 · cuda, metal | The same request under `mixed-allowed` must show actual ≠ requested (plant `N21c-31:spark.mixed-allowed-counterpart`) | GATE-SPARK-09 | BD-178; N21b-28 (counters and shim), N21b-29 (planner refusal) |
| KA-535 | Run digest under slot permutation | Unchanged: the run digest is the wrapping sum of per-world digests keyed by address | exact | G5 · all tiers | A digest that chains in slot order (plant `N21c-31:spark.digest-slot-chain`) | GATE-SPARK-15 (note 9) | N21a-23, N21b-33 |
| KA-536 | Binary kernel cache | A one-byte change of source, options, compiler or driver identity is a miss; a warm start loads every kernel with zero compiler invocations | exact | G1 · cuda, metal | A corrupted entry must be rejected by digest (plant `N21c-31:spark.cache-corrupt`) | GATE-SPARK-11 | BD-173; N21b-12 |
| KA-537 | Commit fence under injected command-buffer faults (rule R2) | Final digests equal those of the unfaulted run; at most four attempts with growing waits, then `FROB-REF-607`; every retry counted in the receipt | exact under 10³ injected faults | G4 · metal | With the fence disabled the mismatch must be detected (plant `N21c-31:spark.fence-disabled`) | GATE-SPARK-12 | N21b-07, BD-175; BD-172 (CUDA launch-fault row) |
| KA-538 | Specialized kernel against its generic kernel on 10⁶ states | Bit-identical (INV-SPARK-09) | exact | G5 · cuda, metal | A reassociated variant must be refused (`FROB-REF-613`; plant `N21c-31:spark.specialized-reassociated`) | GATE-SPARK-14 | BD-371 |
| KA-539 | Scene regenerated on the other machine from its seed | Same `SceneId` | exact | G5 · host | A generator with one changed constant (plant `N21c-31:spark.generator-constant`) | GATE-SPARK-13 | BD-370 |
| KA-540 | IEEE probe per device family | `f32` square root and reciprocal agree bit for bit with the correctly rounded reference over all 2³² inputs; +, −, ×, ÷ over 10⁹ sampled operand pairs including subnormal and overflow edges; subnormal and NaN behaviour recorded; the outcome sets the device's best attainable determinism class | exact | G0 · cuda, metal | none stated; this registry adds: a kernel compiled with contraction on must change the recorded class | GATE-SPARK-01 | BD-017 |
| KA-541 | Precision battery over 10⁶ steps at distances up to 1 km from the origin | Group-membership error within 4 ulp; closure and contact violations inside their envelopes; energy-ledger residual under 10⁻⁴ relative | as stated | G1 · cuda, metal | none stated; (set: BD-179) the uncentred [0, 8) shift, absolute `f32` positions without anchoring, and normalization removed, each failing its measure; (set: N21bp2-01) plain `f32` accumulators in `K-ledger-step`, which push the energy residual past 10⁻⁴ | GATE-SPARK-05 | BD-179 (every clause but the energy-ledger residual); N21bp2-01 (the energy-ledger residual clause; records GATE-SPARK-05) |
| KA-542 | Cancellation and fault injection on a device | Every injected fault yields its typed refusal; no leaked device memory after 10³ cancel cycles; the drain report is executor-minted | exact | G4 · cuda, metal | none stated; this registry adds: a dropped handle presented as a drain report must be rejected | GATE-SPARK-06 | BD-175; N21b-22, N21b-23 |
| KA-543 | Device allocation after environment creation | Allocated device bytes are flat over 10⁶ steps (INV-SPARK-03) | exact | G1 · cuda, metal | none stated; this registry adds: one planted allocation in a hot-path kernel launch must move the counter | GATE-SPARK-11 | BD-173; N21b-10 |
| KA-544 | Launch graphs, pools and snapshot reads | Zero graph captures after warm-up; zero synchronizations inside a chunk; no pool memory leaked after 10³ cancel cycles; 10⁵ snapshot reads each match the digest emitted for their epoch | exact | G4 · cuda, metal | none stated; this registry adds: a shape change planted mid-rollout must raise the capture counter and be refused | GATE-SPARK-12 | BD-172; N21b-02 |
| KA-545 | Admission of a model variant into a free slot; 256 topology variants of one bucket | No other world's digest changes; one captured graph with zero compiles after warm-up | exact | G5 · cuda, metal | none stated; (set: N21b-16) a planted admission that writes rows of a slot still referenced by a live world, which changes that world's digest | GATE-SPARK-10 | BD-183; N21b-16 |
| KA-546 | Hardware fingerprint of both machines | Reproduces the starting values: 36 CPU cores, an 80-core GPU and 256 GB of unified memory; a Metal working-set limit of about 222.7 GiB and a largest single buffer of about 167.0 GiB; two RTX 4090 of 24 GB with no link. Every field measured or `unavailable` with a reason | Two runs agree on every discrete field and within 5% on every rate | G1 · cuda, metal | none stated; this registry adds: a record with one field silently absent must fail completeness | GATE-SPARK-07 | BD-029; N21c-01 |
| KA-547 | Padding overhead Ω_p = L_max^p / E[L^p], with 1/r ≤ Ω_p ≤ r⁻ᵖ for r the mean size over the maximum | Degrees of freedom 18 and 35 mixed evenly give 1.32, 1.58, 1.76 for p = 1, 2, 3; mixed 90 to 10 they give 1.78, 2.96, 4.50 (printed 1.8, 3.0, 4.5); worst member 25 in a 36 bucket gives Ω₁ ≤ 1.44 and Ω₃ ≤ 2.99; worst member 33 in a 48 bucket gives Ω₁ ≤ 1.45 and Ω₃ ≤ 3.08 (printed 3.1) | to-set | G1 · host | none stated; (set: N17a-04) E[L]ᵖ in place of E[Lᵖ], and the mean size in place of the maximum as the capacity (Ω₁ below 1/r) | — | BD-171; N17a-04 |
| KA-548 | Break-even for an extra bucket: splitting N worlds off pays when N·(1 − 1/Ω)·t > L | With about 60 launches of 5 to 10 µs each (L = 300 to 600 µs), t = 0.5 µs per world-step and Ω = 1.3, the break-even is 2,600 to 5,200 worlds (inputs are a HYPOTHESIS; the launch cost is measured by the fingerprint) | to-set | G1 · host | none stated | — | BD-171; N17a-04 |
| KA-549 | Specialization ceiling from a step trace | Of 262 ns per world-step the specializable stages sum to 53 ns (20%): an infinitely fast recursion gives 262/209 = 1.25, a recursion twice as fast 1.11; a share of 16% gives 1.19; a ceiling of 1.5 needs a share of one third | to-set | G1 · host | none stated | — | BD-371 and the three-arm pilot N21c-17 |
| KA-550 | Fleet ceiling | Adding the Mac's GPU to training adds at most its share of the pair of cards: 0.125 at 25% of one RTX 4090, 0.30 at 60%; removing a fraction g of the work on the critical machine speeds a campaign by at most 1/(1 − g), so 1.5 needs g ≥ 1/3 | exact | G1 · host | none stated | — | BD-370 |
| KA-551 | Dense brick directory size with 8³-voxel bricks and 4-byte slots | 63 × 63 × 19 = 75,411 slots, 0.3 MB, for a 10 × 10 × 3 m interior at 2 cm; 587,500 slots, 2.3 MB, for a 60 × 40 × 8 m warehouse at 4 cm; 18.8 MB for that warehouse at 2 cm; under 1.5% of the scene it indexes | exact | G1 · host | none stated; (set: N21c-26) brick side 16 in place of 8, which changes the slot counts | — | BD-164; N19c-16, N21c-26 |
| KA-552 | Cholesky operation count n³/3 | 1,944 operations at n = 18 and 14,292 at n = 35; a refinement step costs about 3n² against n³/3, a ratio of 9k/n for k steps: 2.3 at n = 27 and k = 7, 1.5 at n = 35 and k = 6 | exact | G1 · host | none stated; (set: N21c-25) an operation count of n³/6, which fails `chol_op_counts` | — | BD-156; N21c-25 |

**Reconciliation notes carried with the registry.**

1. KA-547: the chapter text quotes the worst case of the 36 bucket (25 degrees of freedom) and the appendix row
   the 48 bucket (33). Both are arithmetic and both are entered. The default capacity set is itself stated two
   ways in the plan ({12, 18, 24, 36, 48, 64} and {12, 18, 24, 32, 48, 64}); the registry stores the formula and
   the four worked cases, and the bucket-set decision (OPEN-SPARK-3, frozen at M2) decides which bucket rows apply.
2. Rows marked "this registry adds" had no negative control in the plan. The added control is DESIGN; a carrier
   may replace it with a stronger one but may not delete it.
3. KA-529 to KA-531 rest on a recorded plan-time pilot (a one-inequality boundary classifier on a 2⁻²⁸ grid, run
   as a Metal command buffer on this Mac and as a CUDA kernel on each RTX 4090 of `trj`), registered as
   PILOT-PT-15. The pilot's counts are FACT as recorded and must be reproduced at GATE-SPARK-08 before any gate
   cites them.
4. Controls set by their owners. Of the 14 rows printed with no control, six carry the control this registry adds
   (KA-531, KA-540, KA-542 to KA-544, KA-546); five carry one their owners state, marked "(set: <bead>)": KA-541
   (BD-179 for the membership, closure and contact clauses, N21bp2-01 for the energy clause), KA-545 (N21b-16),
   KA-547 (N17a-04), KA-551 (N21c-26) and KA-552 (N21c-25). KA-548, KA-549 and KA-550 are entered `none stated`,
   and a row with no registered control is computed `obligation: control` (N38a-12). Their controls are records
   of N38b-32, an M2 bead that depends on this one, with `seed = "KA-nnn"`; no owner's record exists for these
   rows, so N38b-32's are their first controls: the launch cost omitted, so that splitting always pays (KA-548);
   the specializable share inverted, 4.9 against 1.25 (KA-549); the share taken of one card, 0.25 against 0.125
   (KA-550). When the seeding tool of N38b-05 runs, the owner's or this registry's control is the row's first
   record, and N38b-32 links its proposal to it (the same defect) or adds it as a second record (a different
   one).
5. KA-541 has two carriers. BD-179 measures every clause of the precision battery except the energy-ledger
   residual and closes on those; N21bp2-01, which depends on BD-179 and on the batched energy ledger (BD-208),
   measures the residual clause and records GATE-SPARK-05 from the receipts of both. A row result for KA-541 is a
   pass only when both carriers' clauses are.
6. Recomputed at review: KA-526 (1 + 2⁻¹²)² = 1 + 2⁻¹¹ + 2⁻²⁴, a tie at half an ulp that rounds to even;
   KA-529 16,777,217 rounds to 16,777,216 in `f32`; KA-531 5·2⁻²⁵ < 2⁻²¹; KA-532 1·2 + 1·1 = 3 and 1·8 + 1·4 = 12;
   KA-547 35/26.5 = 1.321, 1,225/774.5 = 1.582, 42,875/24,353.5 = 1.761, 35/19.7 = 1.777, 1,225/414.1 = 2.958,
   42,875/9,536.3 = 4.496, 36/25 = 1.44 and 2.986, 48/33 = 1.455 and 3.077; KA-548 300 and 600 µs over
   (1 − 1/1.3)·0.5 µs = 2,600 and 5,200; KA-549 262/209 = 1.254, 262/235.5 = 1.113, 1/0.84 = 1.19; KA-550
   0.25/2 and 0.60/2, 1/(1 − 1/3) = 1.5; KA-551 ⌈10/0.16⌉² · ⌈3/0.16⌉ = 63² · 19 = 75,411 (301,644 bytes),
   188 · 125 · 25 = 587,500 (2.35 MB) and 375 · 250 · 50 · 4 bytes = 18.75 MB; KA-552 18³/3 = 1,944,
   35³/3 = 14,291.67, 63/27 = 2.33 and 54/35 = 1.54.
7. Plants. Where a bead builds the planted defect of a control, the cell names it by the plant identifier the
   seeding tool of N38b-05 resolves, `<planting bead>:<label>`: the labels of N21c-30 (the M0 controls of KA-525,
   KA-526, KA-529, KA-530 and KA-532, and the folded mutants of KA-527 and KA-528), of N21c-31 (KA-534 to KA-539)
   and of N21a-10 (KA-533). A plant that an entry of N21-CX also names (CX-216 with KA-529 and KA-530, CX-218
   with KA-532, CX-219 with KA-533, CX-220 with KA-527 and KA-528) has one control, seeded from that entry, and
   the row's `control.nc` lists it; every other plant gets one control seeded from its row. KA-532, which has no
   owner gate, thereby takes the gates of CX-218's record.
8. KA-527 and KA-528. The plan's control for both is a build with fast math on. Whether a given compiler's
   fast-math mode breaks a canary is a HYPOTHESIS (N21b-36), and a control must be caught on every run, so the
   rows' control is the folded mutant of N21b-36, which cannot survive: `two_sum_folded` returns (1, 0) and
   `two_prod_folded` returns (1 + 2⁻¹¹, 0). The fast-math build is kept as N21c-30's recorded experiment
   `spark.fast-math`: its outcome (`caught` or `survived`) is logged per compiler identity, it blocks no gate
   and it has no record in `controls.toml`.
9. KA-535's owner (DESIGN). The plan prints no gate for the run digest under slot permutation. The row's owner
   is GATE-SPARK-15, the batch-composition invariance gate (M2), whose arrangements include slot permutation and
   which checks INV-NUM-01 (results are placed by index or address, never by arrival); the row's carrier N21b-33
   runs the slot sweep kernel by kernel, and the gate's bead BD-181 depends on it. Reason: a control seeded from
   a row with no gate attaches to nothing, so the slot-order chain could survive without blocking any gate.

## Acceptance criteria

- [ ] `known_answers.toml` holds exactly the 28 rows KA-525 to KA-552 for this chapter, valid under the loader of
      N38a-12; no file `registry/known_answers/spark.toml` exists; `cargo test -p frob-conformance ka_registry_spark`
      passes.
- [ ] Every row has a non-empty `truth`, `tolerance` (a number, `exact`, a class rule as `gate`, or `to-set`),
      `tier`, `sites` and a derivation file naming its test and carrier.
- [ ] Every row whose gate is a GATE-SPARK identifier names that gate, and the gate exists in `registries/gates.toml`
      (its records are N40-02's, a dependency).
- [ ] Each of the 28 rows either names a negative control in `control.text` or carries the text `none stated`
      and is reported `obligation: control`; no row leaves the field blank; after the seeding tool of N38b-05 is
      re-run every stated control has its `NC-` identifier in `control.nc`. The five owner-set controls (KA-541,
      KA-545, KA-547, KA-551, KA-552) carry the owner's text and key; KA-548 to KA-550 are `obligation: control`.
- [ ] KA-541's derivation file names both carriers with the clauses each measures; the FACT rows' derivation files
      name PILOT-PT-15 (KA-529, KA-530) and PILOT-PT-S2 (KA-546), and those rows are `obligation: rerun` while
      their pilot is not `reproduced`.
- [ ] The registry check fails when a row is deleted, when a carrier key does not resolve, or when a `to-set` tolerance is cited by a recorded gate PASS.
- [ ] The three reconciliation notes are stored in the derivation files of the rows they concern.

## Test plan

### Unit tests
- `ka_registry_spark_is_complete`: loads `known_answers.toml` through the test kit's reader; asserts the key set
  of this chapter's rows equals {KA-525 … KA-552} (28 keys).
- `ka_registry_spark_fields_present`: every required field non-empty; tier in {G0, G1, G4, G5}.
- `ka_registry_spark_arithmetic_rows_recompute`: recomputes in exact rational arithmetic and compares with the stored truth: 63·63·19 = 75,411; 18³/3 = 1,944; 35³/3 = 14,291.67 (stored 14,292); 9·7/27 = 2.33; 9·6/35 = 1.54; 262/209 = 1.2536; 0.25/2 = 0.125; 0.60/2 = 0.30; Ω for (18, 35) even mix = (35/26.5, 35²/774.5, 35³/24,353.5) = (1.321, 1.582, 1.761); break-even 300 µs/(0.2308·0.5 µs) = 2,600 and 600 µs/(0.2308·0.5 µs) = 5,200.
- `ka_registry_spark_f32_rows_recompute` (host `f32`): KA-526 detector equals 0 with separate multiply and subtract and 2⁻²⁴ with a fused operation computed in exact arithmetic; KA-529 `16777217_f32 == 16777216_f32`.
- `ka_registry_spark_padding_recompute` (exact rationals): KA-547's six mixed-bucket values and the worst-member bounds (36/25)ᵖ and (48/33)ᵖ for p = 1, 3, and the bound 1/r ≤ Ω_p ≤ r⁻ᵖ on both mixes; KA-551's warehouse counts 587,500 and 4,687,500 slots.
- `ka_registry_spark_owner_cells`: the five owner-set controls hold the owner's text and key; KA-548 to KA-550 are `obligation: control`; KA-541's derivation lists BD-179 and N21bp2-01; KA-529, KA-530 and KA-546 name their pilots and are `obligation: rerun` while the pilot register says `not-run`.
- `ka_registry_spark_plant_cells`: the cells of KA-525 to KA-530 and KA-532 to KA-539 each name one plant as `<bead>:<label>`, with the bead N21c-30, N21c-31 or N21a-10 as listed in note 7; KA-527 and KA-528 name `N21c-30:spark.folded-eft` and no fast-math plant; KA-535's `owner` is GATE-SPARK-15.
- `ka_registry_rejects_to_set_citation` (negative control): a synthetic gate record citing KA-547 while its tolerance is `to-set` fails the registry check.
- `ka_registry_rejects_dangling_carrier` (negative control): a row whose carrier is `BD-999` fails.

### End-to-end script and logging
`scripts/e2e/spark_ka_registry.sh` (header: `bead=N21-KA milestone=M0 labels=spark,registry gates=GATE-GAUNT-02
plants=deleted-row,dangling-carrier,to-set-citation`; sources the STD-E2E helper; NDJSON at
`target/e2e-logs/spark_ka_registry/<utc>.ndjson` with a transcript). The conformance harness has no agent verb by
design (a harness driven through the surface it judges is a path to self-certification, ADR-GAUNT-01), so the
script runs `cargo test -p frob-conformance --test registry_check` (BD-025) and the unit tests above, then selects
the 28 rows through their tier targets (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> --
<row>`, `<tier>` = `g0_laws`, `g1_known_answers`, `g4_chaos` or `g5_determinism`, as the row's tier says), which
must report each row with its eligibility (`eligible`, `obligation: tolerance`, `obligation: control`,
`obligation: derivation`, `obligation: rerun`) and its per-site outcome as the runner reports it (`unbound` until
its carrier binds it, never passed). It does not read the logs of the carriers' scripts (`spark_probe.sh` of
N21b-40, `kernel_toolchain.sh` of N21a-25): those beads do not depend on this one and log their own `assert` lines
with the KA number, the stored truth and the measured value. It plants on scratch copies under
`target/e2e-scratch/spark_ka_registry/` a deleted row, a row whose carrier is `BD-999` and a synthetic gate record
citing KA-547 while its tolerance is `to-set`, and requires each to be caught. Each line carries the helper's
standard fields (`ts`, `script`, `bead`, `step`, `event`, command line, exit code, wall time); one `assert` per row
with its id, tier, sites, tolerance state, control state (stated, added by this registry, owner-set or open) and
eligibility; the SHA-256 of `known_answers.toml`; on failure the first differing field and the path of the scratch
copy; `metric` lines give rows per eligibility state; the `summary` gives rows per state and the open controls (3)
and tolerances (3).

### Known answers and negative controls
The rows are the content. Negative controls of the registry itself: the two unit tests above. Determinism: the
registry file is canonical TOML with sorted keys; its BLAKE3 digest is identical on Linux and macOS.

## Considerations

- The counts quoted from the plan-time pilot (452, 23,438, 58,254) are FACT as recorded, not yet reproduced here;
  a gate may cite them only after GATE-SPARK-08 reproduces the pilot.
- Three rows have `to-set` tolerances; they are planning arithmetic, not measurements, and their carriers decide
  whether they are checked exactly or to the printed digits.
- If a carrier bead is split or renamed when the beads are loaded, the carrier named in the derivation file is updated in the same change;
  the completeness test guards against a silent loss.

## References

Appendix M.20 (KA-525 to KA-552); §21.23; §21.24; §38.6. Gates GATE-SPARK-00 to -15. INV-SPARK-03, INV-SPARK-09,
PO-SPARK-02. OPEN-SPARK-3. PILOT-PT-15, PILOT-PT-S2 (N38a-10).
