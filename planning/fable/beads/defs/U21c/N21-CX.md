---
key: "N21-CX"
title: "Spark counterexample registry: CX-216 to CX-233 with breaking cases, protected rules and executable tests"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "spark", "frob-conformance", "frob-kernels", "solid", "registry", "test"]
parent: "WP-46"
deps: ["BD-025", "BD-146", "STD-TEST", "STD-E2E"]
needs: ["§21.5 to §21.15: the mechanism beads of the other two Spark drafting units that own the tests for CX-216 to CX-222 (exact kernels, placement cost model, stride converter, dialect lint, canaries, bucket planner); this registry names the test each must provide"]
related: ["N21c-25", "N21c-26", "N21c-30", "N21c-31", "N21c-10", "N21c-16", "N21c-36", "N21c-27", "N21-REF", "N21-KA", "BD-030", "BD-053", "BD-178", "BD-036", "BD-013", "BD-171"]
gates: []
plan_refs: ["Appendix N.18", "§21.5", "§21.6", "§21.8", "§21.11", "§21.12", "§21.14", "§21.17", "§21.18", "§21.20", "§38"]
---
## Background

A counterexample is the smallest concrete case that breaks a tempting shortcut. The plan keeps a catalogue of them
so that each rule in the design is tied to the case that motivated it, and so that the test suite contains, for
each rule, an input on which the shortcut is demonstrably wrong. Eighteen counterexamples belong to Spark (GPU
backends and the kernel toolchain). They seed the negative-control registry: for every rule there is a planted
defect or a hand-checkable computation that a test must catch or reproduce. This bead is the registry of those
eighteen. Mechanism beads cite these numbers in their test plans; the registry check fails if a row has no
executable test.

## Scope

In: the eighteen rows in `registries/counterexamples.toml` (section `spark`) with the breaking case and its
numbers, the rule protected, the executable test and its owner; the registry check; one end-to-end run over all
rows. Out: the mechanisms and their own tests (owners named per row); the known-answer registry of the chapter
(another unit's bead).

## Technical approach

**Rows** (`id`, `shortcut`, `case`, `rule`, `test`, `owner`, `tier`).

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Executable test (owner) |
|---|---|---|---|---|
| CX-216 | A strict inequality can be decided by a bare `f32` comparison | Is 16,777,216 less than 16,777,217? In integers, yes. In `f32` the second number (2²⁴ + 1) is not representable and rounds to the first, so the strict comparison is false. In the recorded pilot on a 65,536-row grid dense near a boundary, the bare `f32` sign gave 452 false inclusions and 23,438 false exclusions (36%), identically on every device; the exact integer kernel gave none | ADR-SPARK-03: exact-integer and guarded kernels for strict predicates and digests; a strict predicate in bare floating point is refused at build time (`FROB-REF-252`); PO-SPARK-02; GATE-SPARK-08 | `negctl spark.bare-f32-strict`, `spark.bare-f32-corpus` (N21c-30); the boundary corpus (BD-030, BD-053) |
| CX-217 | A kernel belongs on the GPU because it is a kernel | The same boundary pilot end to end: the host integer loop took about 34 µs against about 290 µs on Metal (of which under 13 µs was device time) and over 1.2 ms on one RTX 4090; two-card sharding was slower than one card | Representation before placement; a small host-resident job stays on the host; the planner's cost model sends a job to a device only if predicted device time including dispatch, transfer and synchronization is below host time | `placement::small_host_resident_job_stays_on_host` (BD-178) |
| CX-218 | Strides can be copied verbatim between array libraries | A 2 × 2 contiguous `f32` view has strides (8, 4) in signed bytes and (2, 1) in unsigned elements. Copying (8, 4) as element strides onto a 16-element buffer reads element 8 + 4 = 12 at index [1, 1], where element 3 belongs; both reads are in bounds, so nothing faults | Views carry stride unit and signedness; a view the receiver cannot represent is materialized with the copy charged to the receipt, or refused under a zero-copy contract (`FROB-REF-617`); the two array-library admission tests | `negctl spark.verbatim-strides` (N21c-30); the typed stride converter (BD-036) |
| CX-219 | A mask can be applied as an arithmetic blend m·a + (1 − m)·b | An infinity in the unselected arm becomes a NaN, since 0·∞ is NaN | Dialect rule 8: a mask is a select; masks apply to every kernel, reward and gradient | `dialect::mask_is_select_infinity_case` (BD-013) |
| CX-220 | A double-float accumulator works on any compiler | The error-free sum of 1 and 2⁻³⁰ must return (1, 2⁻³⁰); a compiler that simplifies the error term algebraically returns (1, 0), and the accumulator silently loses its low word. The product canary: (1 + 2⁻¹²)² must return (1 + 2⁻¹¹, 2⁻²⁴) | ADR-SPARK-10: split product (splitter 4097 = 2¹² + 1), safe math mode, start-up canaries; `FROB-REF-609` | `negctl spark.fast-math` (N21c-30); the canary kernels |
| CX-221 | A persistent launch with a grid-wide barrier is a safe way to step a whole stretch | In the suite's piano renderer such a barrier on Metal "very rarely" leaves one thread group unscheduled, "about once per few hundred launches", and the host reruns the stretch | Dialect rule 6: no grid-wide barrier; a stage that needs every world to finish is two launches (ADR-SPARK-09) | `dialect::lint_rejects_grid_barrier` (BD-013) |
| CX-222 | Padding a mixed batch to its largest member costs little | Ω_p = L_max^p / E[L^p]: degrees of freedom 18 and 35 mixed evenly cost 1.3, 1.6 and 1.8 for p = 1, 2, 3; mixed 90 to 10 they cost 1.8, 3.0 and 4.5 | Size buckets (the catalogue quotes capacities 12, 18, 24, 32, 48, 64 with Ω₁ ≤ 1.45 and Ω₃ ≤ 3.1 inside a bucket; the chapter's default set has 36 in place of 32, with Ω₁ ≤ 1.44 and Ω₃ ≤ 3.0); KILL-BUCKET | `bucket_decision::omega_even_mix`, `omega_90_10_mix`, `worst_member_*` (N21c-36); the bucket planner (BD-171) |
| CX-223 | Adding the second machine's GPU to training gives fleet speedup | The Mac's GPU adds at most its share of the pair of cards: 0.125 at 25% of one RTX 4090, 0.30 at 60%. Gain can come only from moving non-training work off the training cards, at most 1/(1 − g); a factor of 1.5 needs g ≥ 1/3 | The fleet scheduler is funded only if the phase-ledger pilot shows g ≥ 1/3; KILL-FLEET | `fleet_pilot::g_hand_case_*` (N21c-10); `kill_fleet_first_reading_table` (N21c-11) |
| CX-224 | Per-design specialized kernels should be the default path | On an incumbent's event trace of a humanoid, of 262 ns per world-step the stages a specialized recursion could touch sum to 53 ns (20%): an infinitely fast recursion gives 262/209 = 1.25, twice as fast 1.11; a manipulation scene gives 1.19; cold kernel compilation takes about 96 s | ADR-SPARK-08: survivors only, only at a specializable share of at least one third, bit-identical to the generic kernel; KILL-SPECIALIZE | `specializable_share::share_incumbent_trace` (N21c-16); `rejected::structure::per_design_*` (N21c-26) |
| CX-225 | Factor the contact Hessian in half precision on tensor cores and refine in single precision | Half precision has unit round-off 2⁻¹¹ ≈ 4.9·10⁻⁴ and refinement needs κ·u < 1. On contact states of three robots after unit-diagonal scaling, κ·u ≥ 1 in 37 to 64% of states and refinement fails outright in 9 to 18%. A rank-one worst case has κ = 1 + 48/0.01 = 4,801, so κ·u ≈ 2.3. Each refinement step costs about 3n² against n³/3, a ratio of 9k/n: 2.3 at n = 27 and k = 7 | Single precision in scaled variables; half precision is not used anywhere in the step | `rejected::precision::half_*` (N21c-25) |
| CX-226 | Copy-on-write forks of device world state save copying | State is write-dense: every step rewrites every world's configuration, velocity, contacts and warm starts, so a fork has copied everything after one step and paid for indirection in every read. The whole copy is small: 8,192 worlds at 30 kB are 0.25 GB, about half a millisecond at an RTX 4090's bandwidth | Fork by indexed copy of chosen rows into free slots (`K-snapshot`) | `rejected::structure::cow_*` (N21c-26) |
| CX-227 | Brick directories need minimal perfect hashing or succinct structures | A dense directory with 4-byte slots is 63 × 63 × 19 = 75,411 slots, 0.3 MB, for a 10 × 10 × 3 m interior at 2 cm and 18.8 MB for a 60 × 40 × 8 m warehouse at 2 cm, under 1.5% of the scene it indexes; a hash adds a second, verifying read to every lookup | A dense directory per scene variant over a shared brick pool | `rejected::structure::directory_*` (N21c-26) |
| CX-228 | A mergeable quantile sketch gives a statistic that does not depend on arrival | A two-centroid sketch that merges its closest pair, fed {0, 3, 5, 9}: in the order 0, 3, 5, 9 it ends as (8/3 with weight 3; 9 with weight 1); in the order 0, 9, 5, 3 it ends as (1.5 with weight 2; 7 with weight 2) | Integer log-bin histograms (`K-x-hist`), merged by addition; a quantile is reported as a bin interval | `rejected::precision::sketch_order_*`, `histogram_merge_order_independent` (N21c-25) |
| CX-229 | On unified memory the CPU can read live GPU state with zero copy | Shared storage must be quiescent before the other processor reads it; and the data are tiny: a humanoid's integration state is about 540 bytes, so shadowing 256 worlds at 50 Hz moves 6.9 MB/s | Fenced snapshots under the readback rule and epoch leases (`FROB-REF-614`); INV-SPARK-06 | `rejected::structure::shadow_stream_rate`, `host_read_outside_lease_refused` (N21c-26); the lease tests |
| CX-230 | Unordered floating-point scatter is acceptable if 99.9% of windows replay bitwise | 0.999¹⁰⁰⁰ = 0.37: a campaign of a thousand windows would replay exactly about one time in three | Stable-key sort and segmented reduction; floating-point atomics banned | `rejected::precision::scatter_replay_probability`, `f32_sum_order_changes_bits` (N21c-25) |
| CX-231 | Tensor-core products for a 256-wide policy speed training | The learner and inference together are about 13% of training wall-clock on one incumbent stack, so their removal gives at most 1.15; reduced precision also leaves class D2 | Plain `f32` products in the dialect; device matrix paths only inside the labelled D4 profile | `rejected::precision::policy_amdahl_bound`, `gemm_device_path_refused_under_d2` (N21c-25) |
| CX-232 | Per-world adaptive time tiles between interactions save work | Separate environments are already independent; policy and sensor frames at 50 to 100 Hz bound a tile to 5 to 20 steps; shapes become dynamic | Fixed decimation in a static launch graph (INV-SPARK-05) | `rejected::structure::tile_bounds` (N21c-26) |
| CX-233 | Full-physics sampling control can be the default evaluator | At the smallest published setting (30 samples, 3 iterations, 0.9 s horizon, 50 Hz) a simulated second costs 2.0 M world-steps against 500 for a policy, a factor of 4·10³: a battery of 256 twenty-second episodes takes about 86 minutes of a device at 2.0 M world-steps/s against 1.3 s | A sampling controller for single worlds and as a fallback; never a stand-in evaluator for a trained policy | `rejected::structure::sampling_control_cost`, `sampling_battery_refused_for_promotion` (N21c-26) |

**Registry check** (in `frob-conformance`): every row's `test` resolves to at least one existing test; each test
either reproduces the breaking case's numbers exactly (for arithmetic rows) or plants the shortcut and shows the
rule catching it (for mechanism rows); a row whose test is missing after its owner's milestone fails the check.
Arithmetic rows are tier G0 or G1 on the host; mechanism rows run on each backend against its class.

**Recorded measurements.** Three rows quote measurements that are context and not assertions: the pilot's 452 and
23,438 (CX-216), its timings of 34 µs, 290 µs and 1.2 ms (CX-217), and the 37 to 64% and 9 to 18% shares (CX-225).
Their tests assert the qualitative statement (at least one mislabelled row; the host placement for a small
host-resident job; κ·u > 1 on the rank-one case) and log the re-measured values beside the recorded ones.

## Acceptance criteria

- [ ] Eighteen rows, CX-216 to CX-233, exist with shortcut, case, rule, test and owner.
- [ ] Every row's test exists and passes by its owner's milestone; the registry check fails on a planted row with no test.
- [ ] Each arithmetic value in the table is asserted somewhere in the named tests (exact or rational arithmetic where the value is exact).
- [ ] Each mechanism row has a planted shortcut that its rule catches, registered in the negative-control registry.
- [ ] The generated counterexample page of the documentation prints the eighteen rows with their test status.

## Test plan

### Unit tests

`cargo test -p frob-conformance counterexamples_spark`: `register_has_18_rows`; `ids_are_cx_216_to_233`;
`every_row_resolves_to_a_test`; `check_fails_on_orphan_row`; and the row-level tests named in the table, which live
with their owners.

### End-to-end script and logging

`scripts/e2e/spark_counterexamples.sh`: runs every test named in the registry section through the conformance
harness (host rows always; device rows on each available backend) and prints, per CX, the breaking case's expected
and observed values. Logs per the end-to-end standard: one `assert` event per row with the CX number, the test
name, the values compared and the device; refusal codes where a rule refuses; on failure the first failing row and
its artifacts; a final `summary` with rows passed, failed and pending by milestone.

### Known answers and negative controls

Related known answers: KA-529, KA-530 (CX-216), KA-532 (CX-218), KA-533 (CX-219), KA-527, KA-528 (CX-220), KA-547
(CX-222), KA-550 (CX-223), KA-549 (CX-224), KA-552 (CX-225), KA-551 (CX-227). The registry's own negative control
is the orphan-row check. Determinism lanes: host rows on Linux and macOS; device rows on each backend.

## Considerations

- CX-222's catalogue row and the chapter disagree on one bucket capacity (32 against 36); both sets' worst-member
  values are tested and the bucket decision records which set is frozen.
- CX-221 cannot be reproduced on demand (the stall is rare by nature); its test is the lint that makes the
  construct unwritable, not a stress run.
- Rows owned by other units are registered here with the test name this registry expects; an owner may rename a
  test by updating the row.

## References

Plan Appendix N (section for Spark, CX-216 to CX-233) and the defining subsections §21.5, §21.6, §21.8, §21.11, §21.12, §21.14, §21.17, §21.18, §21.20.
