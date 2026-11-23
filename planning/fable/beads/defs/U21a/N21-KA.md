---
key: "N21-KA"
title: "registry: Spark known answers KA-525 to KA-552 with truth, tolerance, tier, negative control and carrier"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: ["§21.14: the start-up canary kernel `K-canary` and `FROB-REF-609`, carrier of KA-527 and KA-528 on devices", "§21.17: the transfer-class meter and scene regeneration across machines, carrier of KA-539 and KA-550", "§21.11: the bucket planner that carries KA-547 and KA-548"]
related: ["N21-CX", "N21-REF", "BD-013", "BD-017", "BD-020", "BD-029", "BD-030", "BD-036", "BD-053", "BD-156", "BD-164", "BD-171", "BD-172", "BD-173", "BD-174", "BD-175", "BD-176", "BD-178", "BD-179", "BD-183", "BD-203", "BD-370", "BD-371", "N21a-05", "N21a-21", "N21b-40", "N21a-25"]
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

In: the 28 rows below as registry entries with fields `ka`, `case`, `truth`, `tolerance`, `tier`, `sites`,
`negative_control`, `gate`, `carrier`; a completeness test; the reconciliation notes at the end.
Out: the fixtures and tests themselves (the carrier beads); the counterexample registry of the chapter (`N21-CX`)
and the refusal registry (`N21-REF`), which another drafting unit writes.

## Technical approach

Entries live in `crates/frob-conformance/registry/known_answers/spark.toml`, one table per row. A row whose
tolerance the plan leaves as "to set" is entered as `tolerance = "to-set"`; its carrier bead must replace it with a
number and record that number as a TARGET before the row can be cited. Sites: `host` (a CPU test in exact or
`f32` arithmetic), `cuda`, `metal`, `all tiers`.

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Gate | Carrier |
|---|---|---|---|---|---|---|---|
| KA-525 | One dialect source for `K-cay-step` (the Cayley group update) compiled for `batch-cpu`, CUDA and Metal | Matches the Rust reference inside its determinism class on all three backends | Bit-equal for D1; at most 4 ulp per kernel output for D3 | G1 · cuda, metal | A prelude with floating-point contraction switched on | GATE-SPARK-00, GATE-SPARK-02 | BD-020, BD-176; N21a-25 (CPU half) |
| KA-526 | Contraction detector in `f32`: (1 + 2⁻¹²)·(1 + 2⁻¹²) − (1 + 2⁻¹¹) | 0 without contraction (the product 1 + 2⁻¹¹ + 2⁻²⁴ is a tie and rounds to even, 1 + 2⁻¹¹); 2⁻²⁴ with contraction | exact | G0 · cuda, metal | The fused form itself (`fma(a, a, −c)`) | GATE-SPARK-01 | BD-017; BD-013 |
| KA-527 | Double-float canary, sum (`K-canary`) | The error-free sum of 1 and 2⁻³⁰ returns the pair (1, 2⁻³⁰); a compiler that simplifies the error term algebraically returns (1, 0); a failed canary refuses the device for double-float kernels (`FROB-REF-609`) | exact | G0 · cuda, metal | A build with fast math on | — | N21a-21 (CPU and IR lowering); the device canary bead of the precision subsection (drafted as N21b-36) |
| KA-528 | Double-float canary, product by the split product (splitter 4097 = 2¹² + 1) | The error-free product of 1 + 2⁻¹² with itself returns (1 + 2⁻¹¹, 2⁻²⁴) | exact | G0 · cuda, metal | A build with fast math on | — | N21a-21; the device canary bead of the precision subsection (drafted as N21b-36) |
| KA-529 | Strict predicate 2²⁴ < 2²⁴ + 1 (16,777,216 < 16,777,217) | True in the exact and guarded kernels | exact; identical bytes on `batch-cpu`, both RTX 4090 and Metal | G0 · all tiers | The bare `f32` kernel answers false, since 16,777,217 rounds to 16,777,216 | GATE-SPARK-08 | BD-030, BD-053, N21b-40 |
| KA-530 | Boundary corpus of 2 × 65,536 rows with exact ties | Exact kernel: zero errors, identical bytes on every backend. Guarded kernel: zero errors after fallback, fallback fraction reported (recorded pilot: guard deferred whenever the absolute residual was at most 2⁻²¹; it decided none of the dense grid on the device and 58,254 of 65,536 rows of the wider grid) | exact | G0 · all tiers | The bare `f32` kernel must mislabel at least one row (pilot: 452 false inclusions and 23,438 false exclusions) | GATE-SPARK-08 | BD-030, N21b-40 |
| KA-531 | Guard of a guarded predicate (PO-SPARK-02) | If abs(r̂) > ε the sign of the computed residual r̂ is the sign of the exact residual r; for the pilot's conversion and addition sequence the rounding error is at most 5·2⁻²⁵, below the deferral threshold 2⁻²¹ | exact (certified bound; exhaustive check on the boundary corpus) | G0 · host | none stated; this registry adds: a bound scaled by ½ must be rejected by the exhaustive check | — | BD-053 |
| KA-532 | Stride conventions on a 2 × 2 contiguous `f32` view: byte strides (8, 4) against element strides (2, 1) | Index [1, 1] reads element 3; a view crossing the boundary carries its stride unit and signedness, is materialized with the copy charged to the receipt, or is refused under a zero-copy contract (`FROB-REF-617`) | exact | G1 · host | A verbatim stride copy onto a 16-element buffer reads element 8 + 4 = 12, in bounds | — | BD-036, N21a-05 |
| KA-533 | Mask with an infinity in the unselected arm | `select(pred, a, b)` returns the selected arm unchanged | exact | G0 · all tiers | The arithmetic blend m·a + (1 − m)·b returns NaN, since 0·∞ is NaN | — | BD-018, N21a-10 |
| KA-534 | Strict placement with one device kernel withheld, for every stage | Refusal by the planner (`FROB-REF-604`) before any world is stepped; zero episodes; receipt counters for transfers and synchronizations match an instrumented shim | exact for the refusal; within one count for the counters | G1 · cuda, metal | The same request under `mixed-allowed` must show actual ≠ requested | GATE-SPARK-09 | BD-178 |
| KA-535 | Run digest under slot permutation | Unchanged: the run digest is the wrapping sum of per-world digests keyed by address | exact | G5 · all tiers | A digest that chains in slot order | — | N21a-23, BD-203 |
| KA-536 | Binary kernel cache | A one-byte change of source, options, compiler or driver identity is a miss; a warm start loads every kernel with zero compiler invocations | exact | G1 · cuda, metal | A corrupted entry must be rejected by digest | GATE-SPARK-11 | BD-173 |
| KA-537 | Commit fence under injected command-buffer faults | Final digests equal those of the unfaulted run; at most four attempts with growing waits, then `FROB-REF-607`; every retry counted in the receipt | exact under 10³ injected faults | G4 · metal | With the fence disabled the mismatch must be detected | GATE-SPARK-12 | BD-174, BD-175; BD-172 (CUDA launch-fault row) |
| KA-538 | Specialized kernel against its generic kernel on 10⁶ states | Bit-identical (INV-SPARK-09) | exact | G5 · cuda, metal | A reassociated variant must be refused (`FROB-REF-613`) | GATE-SPARK-14 | BD-371 |
| KA-539 | Scene regenerated on the other machine from its seed | Same `SceneId` | exact | G5 · host | A generator with one changed constant | GATE-SPARK-13 | BD-370 |
| KA-540 | IEEE probe per device family | `f32` square root and reciprocal agree bit for bit with the correctly rounded reference over all 2³² inputs; +, −, ×, ÷ over 10⁹ sampled operand pairs including subnormal and overflow edges; subnormal and NaN behaviour recorded; the outcome sets the device's best attainable determinism class | exact | G0 · cuda, metal | none stated; this registry adds: a kernel compiled with contraction on must change the recorded class | GATE-SPARK-01 | BD-017 |
| KA-541 | Precision battery over 10⁶ steps at distances up to 1 km from the origin | Group-membership error within 4 ulp; closure and contact violations inside their envelopes; energy-ledger residual under 10⁻⁴ relative | as stated | G1 · cuda, metal | none stated | GATE-SPARK-05 | BD-179 |
| KA-542 | Cancellation and fault injection on a device | Every injected fault yields its typed refusal; no leaked device memory after 10³ cancel cycles; the drain report is executor-minted | exact | G4 · cuda, metal | none stated; this registry adds: a dropped handle presented as a drain report must be rejected | GATE-SPARK-06 | BD-175 |
| KA-543 | Device allocation after environment creation | Allocated device bytes are flat over 10⁶ steps (INV-SPARK-03) | exact | G1 · cuda, metal | none stated; this registry adds: one planted allocation in a hot-path kernel launch must move the counter | GATE-SPARK-11 | BD-173 |
| KA-544 | Launch graphs, pools and snapshot reads | Zero graph captures after warm-up; zero synchronizations inside a chunk; no pool memory leaked after 10³ cancel cycles; 10⁵ snapshot reads each match the digest emitted for their epoch | exact | G4 · cuda, metal | none stated; this registry adds: a shape change planted mid-rollout must raise the capture counter and be refused | GATE-SPARK-12 | BD-172 |
| KA-545 | Admission of a model variant into a free slot; 256 topology variants of one bucket | No other world's digest changes; one captured graph with zero compiles after warm-up | exact | G5 · cuda, metal | none stated | GATE-SPARK-10 | BD-183 |
| KA-546 | Hardware fingerprint of both machines | Reproduces the starting values: 36 CPU cores, an 80-core GPU and 256 GB of unified memory; a Metal working-set limit of about 222.7 GiB and a largest single buffer of about 167.0 GiB; two RTX 4090 of 24 GB with no link. Every field measured or `unavailable` with a reason | Two runs agree on every discrete field and within 5% on every rate | G1 · cuda, metal | none stated; this registry adds: a record with one field silently absent must fail completeness | GATE-SPARK-07 | BD-029 |
| KA-547 | Padding overhead Ω_p = L_max^p / E[L^p], with 1/r ≤ Ω_p ≤ r⁻ᵖ for r the mean size over the maximum | Degrees of freedom 18 and 35 mixed evenly give 1.32, 1.58, 1.76 for p = 1, 2, 3; mixed 90 to 10 they give 1.78, 2.96, 4.50 (printed 1.8, 3.0, 4.5); worst member 25 in a 36 bucket gives Ω₁ = 1.44 and Ω₃ = 2.99; worst member 33 in a 48 bucket gives Ω₁ = 1.45 and Ω₃ = 3.08 | to-set | G1 · host | none stated | — | BD-171 |
| KA-548 | Break-even for an extra bucket: splitting N worlds off pays when N·(1 − 1/Ω)·t > L | With about 60 launches of 5 to 10 µs each (L = 300 to 600 µs), t = 0.5 µs per world-step and Ω = 1.3, the break-even is 2,600 to 5,200 worlds (inputs are a HYPOTHESIS; the launch cost is measured by the fingerprint) | to-set | G1 · host | none stated | — | BD-171 |
| KA-549 | Specialization ceiling from a step trace | Of 262 ns per world-step the specializable stages sum to 53 ns (20%): an infinitely fast recursion gives 262/209 = 1.25, a recursion twice as fast 1.11; a share of 16% gives 1.19; a ceiling of 1.5 needs a share of one third | to-set | G1 · host | none stated | — | BD-371 and the three-arm pilot N21c-17 |
| KA-550 | Fleet ceiling | Adding the Mac's GPU to training adds at most its share of the pair of cards: 0.125 at 25% of one RTX 4090, 0.30 at 60%; removing a fraction g of the work on the critical machine speeds a campaign by at most 1/(1 − g), so 1.5 needs g ≥ 1/3 | exact | G1 · host | none stated | — | BD-370 |
| KA-551 | Dense brick directory size with 8³-voxel bricks and 4-byte slots | 63 × 63 × 19 = 75,411 slots, 0.3 MB, for a 10 × 10 × 3 m interior at 2 cm; 587,500 slots, 2.3 MB, for a 60 × 40 × 8 m warehouse at 4 cm; 18.8 MB for that warehouse at 2 cm; under 1.5% of the scene it indexes | exact | G1 · host | none stated | — | BD-164 |
| KA-552 | Cholesky operation count n³/3 | 1,944 operations at n = 18 and 14,292 at n = 35; a refinement step costs about 3n² against n³/3, a ratio of 9k/n for k steps: 2.3 at n = 27 and k = 7, 1.5 at n = 35 and k = 6 | exact | G1 · host | none stated | — | BD-156 |

**Reconciliation notes carried with the registry.**

1. KA-547: the chapter text quotes the worst case of the 36 bucket (25 degrees of freedom) and the appendix row
   the 48 bucket (33). Both are arithmetic and both are entered. The default capacity set is itself stated two
   ways in the plan ({12, 18, 24, 36, 48, 64} and {12, 18, 24, 32, 48, 64}); the registry stores the formula and
   the four worked cases, and the bucket-set decision (OPEN-SPARK-3, frozen at M2) decides which bucket rows apply.
2. Rows marked "this registry adds" had no negative control in the plan. The added control is DESIGN; a carrier
   may replace it with a stronger one but may not delete it.
3. KA-529 to KA-531 rest on a recorded plan-time pilot (a one-inequality boundary classifier on a 2⁻²⁸ grid, run
   as a Metal command buffer on this Mac and as a CUDA kernel on each RTX 4090 of `trj`). The pilot's counts are
   FACT as recorded and must be reproduced at GATE-SPARK-08 before any gate cites them.

## Acceptance criteria

- [ ] `spark.toml` holds exactly the 28 rows KA-525 to KA-552; `cargo test -p frob-conformance ka_registry_spark` passes.
- [ ] Every row has a non-empty `truth`, `tolerance` (a number, `exact`, a class rule or `to-set`), `tier`, `sites` and `carrier`.
- [ ] Every row whose gate is a GATE-SPARK identifier names that gate, and the gate exists in `registries/gates.toml`.
- [ ] Each of the 28 rows either names a negative control or is explicitly `negative_control = "none"`; no row leaves the field blank.
- [ ] The registry check fails when a row is deleted, when a carrier key does not resolve, or when a `to-set` tolerance is cited by a recorded gate PASS.
- [ ] The three reconciliation notes are stored beside the rows as `notes`.

## Test plan

### Unit tests
- `ka_registry_spark_is_complete`: loads `spark.toml`; asserts the key set equals {KA-525 … KA-552} (28 keys).
- `ka_registry_spark_fields_present`: every required field non-empty; tier in {G0, G1, G4, G5}.
- `ka_registry_spark_arithmetic_rows_recompute`: recomputes in exact rational arithmetic and compares with the stored truth: 63·63·19 = 75,411; 18³/3 = 1,944; 35³/3 = 14,291.67 (stored 14,292); 9·7/27 = 2.33; 9·6/35 = 1.54; 262/209 = 1.2536; 0.25/2 = 0.125; 0.60/2 = 0.30; Ω for (18, 35) even mix = (35/26.5, 35²/774.5, 35³/24,353.5) = (1.321, 1.582, 1.761); break-even 300 µs/(0.2308·0.5 µs) = 2,600 and 600 µs/(0.2308·0.5 µs) = 5,200.
- `ka_registry_spark_f32_rows_recompute` (host `f32`): KA-526 detector equals 0 with separate multiply and subtract and 2⁻²⁴ with a fused operation computed in exact arithmetic; KA-529 `16777217_f32 == 16777216_f32`.
- `ka_registry_rejects_to_set_citation` (negative control): a synthetic gate record citing KA-547 while its tolerance is `to-set` fails the registry check.
- `ka_registry_rejects_dangling_carrier` (negative control): a row whose carrier is `BD-999` fails.

### End-to-end script and logging
No script of its own. The registry is read by every Spark end-to-end script (for example
`scripts/e2e/spark_probe.sh`, bead N21b-40, and `scripts/e2e/kernel_toolchain.sh`, bead N21a-25): each
`assert` line they log carries the `ka` number, the stored truth and tolerance, and the measured value, so a log
line can be joined to this registry. The conformance harness has no agent verb by design (a harness driven through
the surface it judges is a path to self-certification); the rows are listed and run by
`cargo test -p frob-conformance --test g1 -- spark`, which must report the 28 rows.

### Known answers and negative controls
The rows are the content. Negative controls of the registry itself: the two unit tests above. Determinism: the
registry file is canonical TOML with sorted keys; its BLAKE3 digest is identical on Linux and macOS.

## Considerations

- The counts quoted from the plan-time pilot (452, 23,438, 58,254) are FACT as recorded, not yet reproduced here;
  a gate may cite them only after GATE-SPARK-08 reproduces the pilot.
- Three rows have `to-set` tolerances; they are planning arithmetic, not measurements, and their carriers decide
  whether they are checked exactly or to the printed digits.
- If a carrier bead is split or renamed when the beads are loaded, the `carrier` field is updated in the same change;
  the completeness test guards against a silent loss.

## References

Appendix M.20 (KA-525 to KA-552); §21.23; §21.24; §38.6. Gates GATE-SPARK-00 to -14. INV-SPARK-03, INV-SPARK-09,
PO-SPARK-02. OPEN-SPARK-3.
