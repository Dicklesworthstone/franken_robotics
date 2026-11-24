---
key: "N21-CX"
title: "Spark counterexample registry: CX-216 to CX-233 with breaking cases, protected rules and executable tests"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "spark", "frob-conformance", "frob-kernels", "solid", "registry", "test"]
parent: "WP-46"
deps: ["BD-025", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-146", "N21-KA", "N21-REF", "N21a-05", "N21a-07", "N21a-09", "N21a-10", "N21a-21", "N21a-22", "N21b-18", "N21b-30", "N21b-36", "N21b-40", "N21c-10", "N21c-16", "N21c-17", "N21c-25", "N21c-26", "N21c-27", "N21c-30", "N21c-31", "N38b-04"]
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
executable test. Tier: Solid.

## Scope

In: the eighteen entries CX-216 to CX-233 in the program's one counterexample catalogue,
`crates/frob-conformance/registry/counterexamples.toml`, in the record schema of its owner N38b-05, with the
breaking case and its numbers, the rule protected, the kind of control and the plant; the table that binds each
entry to its executable test and owner; the registry check; one end-to-end run over all rows. Out, with the bead
that has each: the catalogue's record schema and loader, and the seeding tool that turns entries into rows of
`controls.toml`, allocates their control identifiers and resolves a rule that names no gate (N38b-05: this bead
writes no row of `controls.toml` by hand); running controls (N38b-04); the mechanisms and their own tests (owners
named per row); the known-answer registry of the chapter (N21-KA).

## Technical approach

**Entry format (the record of N38b-05; no private file and no private fields).** Each row below is one entry of
`registry/counterexamples.toml`: `id` (the CX number, append-only), `shortcut`, `case` (with its numbers),
`numeric = true` and `plan_case = "numbers"` for all eighteen (none of the chapter's entries states its case in
words only), `case_supplied_by` (empty), `rule` (the text of the rule column), `rule_ids` (the ADR, INV, PO, gate,
kill-gate, admission-gate and refusal identifiers the rule column names; where it names none, the defining
section, which the seeding tool resolves to a gate through `registry/rule_gates.toml`), `defined_in` (the sections
Appendix N.18 gives: §21.5, §21.6, §21.8, §21.10, §21.11, §21.12, §21.14, §21.17, §21.18, §21.20), `same_as`
(empty), `label`, `rerun` and `rerun_receipt` (on FACT entries only), `control_kind` and `plant`. An earlier draft
of this bead wrote the rows into a file `registries/counterexamples.toml` with a section `spark` and the fields
`test`, `owner` and `tier`; that file and those fields are withdrawn: N38b-05's loader admits one file and no
other fields, and a chapter's private file or row format is a lint failure. The binding of an entry to its
executable test, owner and tier, and the literals the test must reproduce (the inputs, the wrong result the
shortcut gives and the right one), are kept as one compiled-in table keyed by CX number in this bead's test file
`crates/frob-conformance/tests/counterexamples_spark.rs`, which the registry check reads.

**Labels.** `label = "DESIGN"` where the breaking case is arithmetic that the bound test recomputes. `label =
"FACT"` with its source where the case quotes a measurement or another system's behaviour: CX-216 and CX-217 (the
planning pilot of the boundary screen), CX-221 (a comment in the suite's piano renderer, `metal.rs`), CX-224 and
CX-231 (an incumbent's published trace, compile time and wall-clock shares), CX-225 (recorded shares over contact
states) and CX-233 (the smallest published setting of a sampling controller). A FACT entry carries `rerun = false`
until the bead named in its row has reproduced the measurement here, or, for a published or sibling-source fact,
re-read it at the pinned source; until then the seeding tool writes its control as an `obligation`, which is not
runnable and makes no gate eligible. The recorded figures of a FACT row are context: its test asserts the
qualitative statement and logs the re-measured values beside the recorded ones.

**Kinds and plants (DESIGN, so that every entry can become a runnable control; a plant owned by another bead is
named with that bead's key, and the owner may rename it by updating the entry).** `mutation`: a test build in
which the shortcut replaces the rule in the real component. `planted-input`: a request or source that asks for
the shortcut and must be refused. For a row whose rule is a calculation with no mechanism to mutate (CX-222,
CX-223, CX-227, CX-232), the plant is the shortcut's own claim offered as a planted input: the compiled-in table
holds the figure the shortcut asserts, and the row's check passes only if the owner's computation rejects that
figure and returns the right one.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Executable test (owner) |
|---|---|---|---|---|
| CX-216 | A strict inequality can be decided by a bare `f32` comparison | Is 16,777,216 less than 16,777,217? In integers, yes. In `f32` the second number (2²⁴ + 1) is not representable and rounds to the first, so the strict comparison is false. In the recorded pilot on a 65,536-row grid dense near a boundary, the bare `f32` sign gave 452 false inclusions and 23,438 false exclusions (36%), identically on every device; the exact integer kernel gave none | ADR-SPARK-03: exact-integer and guarded kernels for strict predicates and digests; a strict predicate in bare floating point is refused at build time (`FROB-REF-252`); PO-SPARK-02; GATE-SPARK-08 | `bare_f32_predicate_wrong_host` and the controls `spark.bare-f32-strict`, `spark.bare-f32-corpus` (N21c-30); the boundary corpus (BD-030, BD-053) |
| CX-217 | A kernel belongs on the GPU because it is a kernel | The same boundary pilot end to end: the host integer loop took about 34 µs against about 290 µs on Metal (of which under 13 µs was device time) and over 1.2 ms on one RTX 4090; two-card sharding was slower than one card | Representation before placement; a small host-resident job stays on the host; the planner's cost model sends a job to a device only if predicted device time including dispatch, transfer and synchronization is below host time | `cost_model_small_job_stays_on_host` (BD-178); `crossover_known_answer` and `transferless_model_mutant_detected` (N21b-30) |
| CX-218 | Strides can be copied verbatim between array libraries | A 2 × 2 contiguous `f32` view has strides (8, 4) in signed bytes and (2, 1) in unsigned elements. Copying (8, 4) as element strides onto a 16-element buffer reads element 8 + 4 = 12 at index [1, 1], where element 3 belongs; both reads are in bounds, so nothing faults | Views carry stride unit and signedness; a view the receiver cannot represent is materialized with the copy charged to the receipt, or refused under a zero-copy contract (`FROB-REF-617`); the two array-library admission tests INT-FNP-01 and INT-FT-01 | `verbatim_stride_reads_12` and the control `spark.verbatim-strides` (N21c-30); the typed stride converter (BD-036); the kernel boundary's `bypass_control` (N21a-05) |
| CX-219 | A mask can be applied as an arithmetic blend m·a + (1 − m)·b | An infinity in the unselected arm becomes a NaN, since 0·∞ is NaN | Dialect rule 8: a mask is a select; masks apply to every kernel, reward and gradient | `rule8_select_keeps_infinity` and `rule8_rejects_blend_pattern` (N21a-10, which completes dialect rule 8 of BD-013) |
| CX-220 | A double-float accumulator works on any compiler | The error-free sum of 1 and 2⁻³⁰ must return (1, 2⁻³⁰); a compiler that simplifies the error term algebraically returns (1, 0), and the accumulator silently loses its low word. The product canary: (1 + 2⁻¹²)² must return (1 + 2⁻¹¹, 2⁻²⁴) | ADR-SPARK-10: split product (splitter 4097 = 2¹² + 1), safe math mode, start-up canaries; `FROB-REF-609` | `canary_expected_pairs_host`, `folded_eft_pairs_host` and the must-catch control `spark.folded-eft` (N21c-30; its row `spark.fast-math` is a hypothesis recorded per compiler and binds nothing here); the canary kernels (N21b-36) |
| CX-221 | A persistent launch with a grid-wide barrier is a safe way to step a whole stretch | In the suite's piano renderer such a barrier on Metal "very rarely" leaves one thread group unscheduled, "about once per few hundred launches", and the host reruns the stretch | Dialect rule 6: no grid-wide barrier; a stage that needs every world to finish is two launches (ADR-SPARK-09) | `lint_rejects_spin_loop` (BD-013: a spin wait on an integer atomic is the only way to build a grid barrier in the dialect, and rule 6 rejects it) |
| CX-222 | Padding a mixed batch to its largest member costs little | Ω_p = L_max^p / E[L^p]: degrees of freedom 18 and 35 mixed evenly cost 1.3, 1.6 and 1.8 for p = 1, 2, 3; mixed 90 to 10 they cost 1.8, 3.0 and 4.5 | Size buckets (the catalogue quotes capacities 12, 18, 24, 32, 48, 64 with Ω₁ ≤ 1.45 and Ω₃ ≤ 3.1 inside a bucket; the chapter's default set has 36 in place of 32, with Ω₁ ≤ 1.44 and Ω₃ ≤ 3.0); KILL-BUCKET | `bucket_decision::omega_even_mix`, `omega_90_10_mix`, `worst_member_*` (N21b-18); the bucket planner (BD-171) |
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
| CX-233 | Full-physics sampling control can be the default evaluator | At the smallest published setting (30 samples, 3 iterations, 0.9 s horizon, 50 Hz) a simulated second costs 2.0 M world-steps against 500 for a policy, a factor of 4·10³: a battery of 256 twenty-second episodes takes about 86 minutes of a device at 2.0 M world-steps/s against 1.3 s | A sampling controller for single worlds and as a fallback; never a stand-in evaluator for a trained policy | `rejected::structure::sampling_control_cost` (N21c-26); from M3 the guard `FROB-REF-817`, whose raising test is N22a-25's (N21c-26's row `row_817_pending_until_m3`) |

**Kind, plant, label and rule identifiers per entry.**

| CX | `control_kind` | `plant` | `label` (and who sets `rerun`) | `rule_ids` |
|---|---|---|---|---|
| CX-216 | `mutation` | `N21c-30:K-bare-pred`, the test-only kernel that decides the strict inequality by the sign of an `f32` residual (the plant of N21c-30's `spark.bare-f32-strict` and `spark.bare-f32-corpus`, so the seeding tool reuses that control and allocates no second one) | FACT: planning pilot; re-run by BD-030 at GATE-SPARK-08 | ADR-SPARK-03, FROB-REF-252, PO-SPARK-02, GATE-SPARK-08 |
| CX-217 | `mutation` | `N21b-30:transferless-model`, the cost model with its dispatch, transfer and synchronization terms removed, which places the 65,536-row host-resident predicate on the device | FACT: planning pilot timings; re-run by N21b-30's validation against the boundary screen measured on each machine | §21.8, §21.12, GATE-SPARK-09 |
| CX-218 | `mutation` | `N21c-30:spark.verbatim-strides`, the test-only converter that copies signed byte strides (8, 4) as unsigned element strides | DESIGN | FROB-REF-617, INT-FNP-01, INT-FT-01 |
| CX-219 | `mutation` | `N21a-10:blend-mutant`, a kernel twin that applies the mask as m·a + (1 − m)·b and returns NaN where the select returns 1 (a = 1, b = +∞) | DESIGN | §21.6 (dialect rule 8), §35.5, FROB-REF-251 |
| CX-220 | `mutation` | `N21c-30:spark.folded-eft`, the sources `two_sum_folded` and `two_prod_folded` of N21b-36 with the error term replaced by 0, which return (1, 0) and (1 + 2⁻¹¹, 0) | DESIGN | ADR-SPARK-10, FROB-REF-609, GATE-SPARK-01 |
| CX-221 | `planted-input` | `BD-013:spin-loop-source`, a kernel source containing `while (FROB_ATOMIC_MAX_U32(p, 0u) < n) {}`, which the dialect lint must reject under rule 6 | FACT: `frankensim/apps/instrument-lab/render-gpu-piano`, `metal.rs`; re-read at the pinned commit by BD-013 (the stall cannot be produced on demand and is not re-measured) | ADR-SPARK-09, FROB-REF-251, §21.6 (dialect rule 6) |
| CX-222 | `planted-input` | `N21-CX:claim-222`, the claim "Ω₃ ≤ 1.1 for degrees of freedom 18 and 35 mixed 90 to 10" offered to N21b-18's Ω function, which returns 4.5 | DESIGN | KILL-BUCKET, §21.11 |
| CX-223 | `planted-input` | `N21-CX:claim-223`, the claim "adding the Mac's GPU to training speeds a campaign by 1.5" offered to N21c-10's ceiling function at g = 0.204, which returns 1.256 and a training share of at most 0.30 | DESIGN | KILL-FLEET, BET-35, §21.17 |
| CX-224 | `planted-input` | `N21c-26:no_compile_after_warmup_on_variant_admission` with a planted per-design compile on a design admitted after warm-up: the compile counter must stay zero | FACT: an incumbent's published step trace and cold compile time; re-read against the source register by N21c-16 (`share_incumbent_trace`) | ADR-SPARK-08, KILL-SPECIALIZE, INV-SPARK-05 |
| CX-225 | `mutation` | `N21c-25:half-precision-factor`, the half-precision factorization of the rank-one matrix (κ = 4,801) refined in single precision, which does not contract (N18b-27's fixture); the same idea as source is rejected by `lint_rejects_half_type` | FACT: recorded shares over contact states of three robots; re-run by N21c-25 over seeds 0 to 199 | FROB-REF-251, §21.14, §35.10 |
| CX-226 | `planted-input` | `N21c-26:cow_indirection_not_expressible`, an IR program with a computed slab index (the indirection a copy-on-write fork needs), which must fail to parse | DESIGN | §21.20, §21.10 |
| CX-227 | `planted-input` | `N21-CX:claim-227`, the claim "a dense directory is too large to keep" offered to N21c-26's slot arithmetic for the 10 × 10 × 3 m interior at 2 cm, which returns 75,411 slots, 301,644 bytes, under 1.5% of the scene | DESIGN | §21.20, §19 |
| CX-228 | `mutation` | `N21c-25:two-centroid-sketch`, the closest-pair sketch in the test module fed {0, 3, 5, 9} in two orders, against the log-bin histogram, whose bin counts are identical in every order | DESIGN | INV-SPARK-04, §35.7 |
| CX-229 | `planted-input` | `N21c-26:host_read_outside_lease_refused`, a host read of the working state slab during a chunk, refused with `FROB-REF-614` | DESIGN (the quiescence requirement is the vendor's documented rule; the 6.9 MB/s figure is arithmetic) | FROB-REF-614, INV-SPARK-06 |
| CX-230 | `mutation` | `N21c-25:f32-sum-reordered`, the same three addends 16,777,216, 1, 1 summed in the two arrival orders (16,777,216 against 16,777,218); a float atomic in kernel source is rejected by `lint_rejects_float_atomic` | DESIGN | FROB-REF-251, §35.6, §35.7 |
| CX-231 | `planted-input` | `N21c-25:gemm_device_path_refused_under_d2`, a device matrix path requested outside the labelled D4 profile | FACT: one incumbent stack's wall-clock shares; re-read against the source register by N21c-25 (`policy_amdahl_bound`) | §35.2, §36.4 |
| CX-232 | `planted-input` | `N21-CX:claim-232`, the claim "a tile spans hundreds of steps" offered to N21c-26's `tile_bounds`, which returns 5 to 20 | DESIGN | INV-SPARK-05 |
| CX-233 | `planted-input` | `N22a-25:sampling-battery-for-promotion`, a sampling-controller battery offered as the scoring source of a promotion, refused with `FROB-REF-817` (from M3; until then the control is an obligation) | FACT: the smallest published setting; re-read against the source register by N21c-26 (`sampling_control_cost`) | FROB-REF-817, §22.8 |

**How an entry becomes a negative control.** The seeding tool of N38b-05 derives one record of `controls.toml`
from each entry: its `seed` is the CX number, its plant is the entry's `plant`, and the test that must fail with
the plant in place, and its tier, come from this bead's compiled-in table. Where a control with the same plant and
target already exists (the M0 controls of N21c-30 for CX-216, CX-218 and CX-220), the tool attaches the entry to
that control. A row whose test does not fail under its plant fails the registry check.

**Registry check** (`FROB_TEST_SEED=0 cargo test -p frob-conformance --test g0_laws -- registry::spark_cx`, the
form in which every conformance entry is run): each of the eighteen CX numbers is present with every field of the
schema; every entry's test in the compiled-in table resolves to at least one existing test; each test either
reproduces the breaking case's numbers exactly (for arithmetic rows) or plants the shortcut and shows the rule
catching it (for mechanism rows); a row whose test is missing after its owner's milestone fails the check, and
before it the row is printed `pending (Mn)`. Arithmetic rows are tier G0 or G1 on the host; mechanism rows run on
each backend against its class. The owners' milestones differ (M0 for the dialect, stride and canary rows; M2 for
the planner, bucket and rejected-mechanism rows; M3 for the 817 guard; M4 for the fleet pilot), so the entries are
entered at M2 and their controls become runnable row by row.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/counterexamples.toml` holds the eighteen entries CX-216 to CX-233, valid under N38b-05's loader (a duplicate identifier is rejected; the chapter count of 18 is met), each with shortcut, case, rule, `rule_ids`, `defined_in`, `label`, `control_kind` and `plant`; the seven FACT entries carry `rerun`; no file `registries/counterexamples.toml` and no field outside the schema exists.
- [ ] The registry check reports every row's test as resolved or `pending (Mn)` before its owner's milestone, and fails on a planted entry with no test in the compiled-in table.
- [ ] Each arithmetic value in the table is asserted somewhere in the named tests (exact or rational arithmetic where the value is exact), and the compiled-in literals equal the numbers in each entry's `case`.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control whose `seed` is its CX number (CX-216, CX-218 and CX-220 attached to N21c-30's existing controls); this bead writes no row of `controls.toml`; the tool's second run changes no byte.
- [ ] A FACT entry with `rerun = false` and the M3 entry CX-233 are `obligation` after seeding and are reported as such, never as caught; for every other entry the report gives the control's latest outcome from the control runner's history (`caught`, `missed`, `never run`).
- [ ] A printout generated from the catalogue by `cx_spark_printout` reproduces the first four columns of the first table, with each row's test status.
- [ ] `scripts/e2e/spark_counterexamples.sh` exits 0.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test counterexamples_spark`:
- `cx_spark_rows_load`: eighteen entries, identifiers 216 to 233 consecutive; every entry has `numeric = true`, `plan_case = "numbers"`, a `control_kind` from the closed list and a non-empty `plant`; exactly CX-216, -217, -221, -224, -225, -231 and -233 are FACT and carry `rerun`.
- `cx_spark_literals`: the compiled-in literals equal the numbers of each `case`: 16,777,216 and 16,777,217 (CX-216); 34 µs, 290 µs, 13 µs (CX-217, logged, not asserted as measurements); strides (8, 4) and (2, 1), elements 12 and 3 (CX-218); NaN against 1 (CX-219); (1, 2⁻³⁰), (1, 0), (1 + 2⁻¹¹, 2⁻²⁴) (CX-220); 1.3, 1.6, 1.8 and 1.8, 3.0, 4.5 (CX-222); 0.125, 0.30, g ≥ 1/3 (CX-223); 262, 53, 1.25, 1.11, 1.19 (CX-224); 4,801 and 2.3 (CX-225); 0.25 GB (CX-226); 75,411 (CX-227); (8/3, 3; 9, 1) and (3/2, 2; 7, 2) (CX-228); 6.9 MB/s (CX-229); 0.37 (CX-230); 1.15 (CX-231); 5 to 20 (CX-232); 2.0 M, 500, 4·10³, 86 min, 1.3 s (CX-233).
- `cx_spark_tests_exist`: every test of the compiled-in table resolves in the workspace test listing, or its row is `pending` before its owner's milestone.
- `cx_spark_claims_rejected`: for CX-222, CX-223, CX-227 and CX-232 the owner's computation returns the right figure and rejects the claimed one.
- `cx_spark_controls_recorded`: for each runnable entry the control runner's history `registry/control_history.ndjson` (N38b-04) is read: a latest line `missed` fails, `caught` passes, no line is reported `never run` with the owner and its milestone (running the plants is the control runner's, invoked by the owners' scripts; this test only reads its record).
- `cx_spark_printout`: the generated table equals the stored first four columns entry by entry.
- `check_fails_on_orphan_row`: a `synthetic` catalogue with a nineteenth entry and no table row fails the check.

### End-to-end script and logging

`scripts/e2e/spark_counterexamples.sh` (sources the STD-E2E helper; seed-explicit; idempotent; it calls only cargo
targets of its dependencies): runs `cargo test -p frob-conformance --test counterexamples_spark`; re-runs the
seeding tool of N38b-05, `cargo test -p frob-conformance --test seed_controls -- --ignored seed`, twice, asserting
that the second run leaves the digest of the committed `controls.toml` unchanged, then its join, `cargo test -p
frob-conformance --test seed_join`; runs the registry check in the form given above; and prints per CX the breaking
case's literals, the bound test's status (resolved or `pending (Mn)`) and the control's state and latest outcome.
Running the owners' tests on each backend, with and without `--features gauntlet-mutations` and the plant, is the
owners' scripts' and the control runner's (N38b-04), not this script's.
NDJSON at `target/e2e-logs/spark_counterexamples/<utc>.ndjson` with a transcript: every line has `ts`, `script`,
`bead`, `step`, `event`; `cmd` lines carry the exact command line, exit code and wall time; one `assert` event per
row with the CX number, the bound test's name and status, and the literals compared; one `metric` line per control, `control_caught: true|false|obligation|never-run`; the digest of `controls.toml`
before and after the second seeding; on failure the first failing row and its artifact paths; a final `summary`
with rows resolved, failed and pending by milestone.

### Known answers and negative controls

Related known answers: KA-529, KA-530 (CX-216), KA-532 (CX-218), KA-533 (CX-219), KA-527, KA-528 (CX-220), KA-547
(CX-222), KA-550 (CX-223), KA-549 (CX-224), KA-552 (CX-225), KA-551 (CX-227). The registry's own negative controls
are the orphan-row check and an entry whose plant does not make its test fail (a `synthetic` history line
`missed`, the control runner's record of a test that ignored its plant, must fail `cx_spark_controls_recorded`). Determinism lanes: the catalogue bytes and the seeded
controls identical twice and on Linux and macOS (class D0); device rows on each backend.

## Considerations

- CX-222's catalogue row and the chapter disagree on one bucket capacity (32 against 36); both sets' worst-member
  values are tested and the bucket decision records which set is frozen.
- CX-221 cannot be reproduced on demand (the stall is rare by nature); its test is the lint that makes the
  construct unwritable, not a stress run.
- Rows owned by other units are registered here with the test name and the plant this registry expects; an owner
  may rename either by updating the entry and the compiled-in table in one change. The plants `N21a-10:blend-mutant`,
  `N21b-30:transferless-model`, `N21c-25:half-precision-factor`, `N21c-25:two-centroid-sketch` and
  `N21c-25:f32-sum-reordered` name mutants or fixtures those beads already describe; the four `claim-` plants are
  this bead's own.
- Owed by the owners and the control runner, not by this registry: every row's test exists and passes by its owner's milestone, and for each runnable entry the owner test fails with the plant in place, recorded with the failing output.
- Plants run only in conformance builds, carry their own kernel identities where they are kernels, and can never
  reach a citable result.

## References

Plan Appendix N (section for Spark, CX-216 to CX-233) and the defining subsections §21.5, §21.6, §21.8, §21.11, §21.12, §21.14, §21.17, §21.18, §21.20.
