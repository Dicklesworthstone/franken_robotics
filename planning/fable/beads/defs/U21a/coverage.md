# Coverage of drafting unit U21a

Chapter §21 Spark (GPU backends and the kernel toolchain), subsections §21.1 to §21.8 and the chapter preamble;
twelve seed beads; the chapter's known-answer registry `N21-KA`. Units U21b (§21.9 to §21.15) and U21c (§21.16 to
§21.26) draft the rest of the chapter; their keys are named below where this unit's beads lean on them through
`needs`.

Files written: 40 bead files (12 seed beads, 27 new beads `N21a-01` to `N21a-27`, 1 registry bead `N21-KA`) and
this file.

## (a) Subsections and the beads that carry them

| Subsection | Title | Carried by |
|---|---|---|
| §21 preamble | Chapter heading only (no text before §21.1) | — |
| §21.1 | What Spark owns | N21a-03 (ownership statement and the table of crates, verbs, records, admission tests); verbs and records by: BD-029 (`spark.fingerprint`, `frob.fingerprint/1`), BD-172 (`frob.stepplan/1`), BD-173 (`spark.cache`, `spark.precompile`), BD-178 (`spark.place`, `frob.placement/1`), BD-018 and N21a-16 (`frob.kir/1`); by other units: `spark.devices`, `spark.probe` (BD-017), `spark.check` (BD-176), `fleet.status`, `fleet.plan` (BD-370), `frob.kernel/1` (the manifest sub-bead of BD-014) |
| §21.2 | Position relative to suite doctrine | N21a-01 (ADR-SPARK-01: annex workspaces, capsules, `apps/`, INV-SPARK-01; what is claimed), N21a-02 (the twelve evidence fields at the agent boundary, INV-SPARK-02, the exploratory label), N21a-27 (the doctrine's 50% and 15% falsifier applied to Spark's own optimization funding), N21a-03 (the ADR text and the claim wording), BD-175 (AE-10: executor-minted drain evidence) |
| §21.3 | Decisions | N21a-03 (all ten ADRs with reason, rejected alternatives, enforcement, governed beads). Each ADR is also stated in the Background of the bead it governs: ADR-SPARK-01 → N21a-01, N21a-02; -02 → BD-013, BD-018, N21a-16; -03 → BD-053, N21a-22; -04 → BD-172; -05 → BD-178; -06 → BD-173; -07 → N21a-03 only (its mechanism is another unit's); -08 → BD-371; -09 → BD-013 (barrier part), N21a-03; -10 → N21a-21 |
| §21.4 | Mechanisms and their tiers | N21a-03 (the tier register, nineteen rows with kill gates and beads); pilots owed by Frontier and Moonshot rows of this unit's subsections: N21a-13 (derived arithmetic through the IR), N21a-26 (fixed-point types, reserved); seed beads mapped here: BD-217, BD-371 |
| §21.5 | The kernel library | N21a-04 (families at M2, naming, completeness, one sort and one compaction), N21a-05 (stride conventions, `FROB-REF-617`), N21a-06 (footprints, PO-SPARK-04, the static half of INV-SPARK-04), BD-152 with N21a-07, N21a-08, N21a-09 (utility kernels), N21a-18 (resource bounds of the stage signature), N21a-22 (the class field); the manifest type itself is the sub-bead of BD-014 written by U21b |
| §21.6 | Two layers of source | BD-013 (preludes; lint rules 1 to 4 and 6; the two renderer habits not inherited), N21a-10 (rules 5, 7, 8, 9; the Rust kernel lint; wiring), N21a-11 (`det_*` functions), N21a-12 (derivation once: generated sources with digests, per-body budgets, the isolated worker, INV-SPARK-08), N21a-13 (pilot), BD-221 (KILL-KIR decision), N21a-14 (KILL-KIR fallback), N21a-25 (end-to-end pipeline script) |
| §21.7 | The kernel IR | BD-018 (v0: straight-line programs, CSE, emitters, dimension checker T-09 and PO-SPARK-01), N21a-16 (v1: composite types, bounded loops, loads and stores, canonical hash, reserved words), N21a-17 (unsigned-integer operators, `ExactInteger`), N21a-18 (resource metadata, PO-SPARK-03, live-range scheduling), N21a-19 (exact rewrite table, rule traces), N21a-20 (derivative programs), N21a-21 (`f32dd`), N21a-11 (elementary functions as subprograms), N21a-15 (e-graph extraction pilot INT-FJAX-01), N21a-26 (fixed-point types) |
| §21.8 | Kernel classes: floating, exact-integer and guarded | N21a-22 (the four classes, registration rules, `stat` never feeds state, rules 1 and 3), BD-053 (predicate typing, select lint, error-bound derivation, PO-SPARK-02; rule 2), N21a-17 (the exact class's arithmetic; the uniform-conversion law of rule 5), N21a-23 (rule 4: the digest as an integer mix), N21a-24 (end-to-end script and GATE-SPARK-08; reproduction of the recorded pilot) |

Seed beads whose content lies mostly in other units' subsections, written completely here as the brief requires:
BD-029 (§21.16), BD-172 (§21.9, §21.11), BD-173 (§21.10), BD-175 (§21.11, §21.21), BD-178 (§21.12), BD-217 (§36),
BD-221 (§21.6, §33.15), BD-371 (§21.18, §17.9).

## (b) Gates, kill gates and admission gates

| Gate | Milestone | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|---|
| GATE-SPARK-00 | M0 | — (N21a-25 runs the CPU half of the same known answer) | BD-020 (U21b) |
| GATE-SPARK-01 | M0 | — | BD-017 (U21b) |
| GATE-SPARK-02 | M2 | — | BD-176 and its end-to-end bead (U21b) |
| GATE-SPARK-03 | M2 | BD-217 | — |
| GATE-SPARK-04 | M2 | BD-217 | — |
| GATE-SPARK-05 | M2 | — | BD-179 (U21b) |
| GATE-SPARK-06 | M2 | BD-175 | end-to-end script bead N21b-25 |
| GATE-SPARK-07 | M0 | BD-029 | end-to-end bead N21c-06; probes N21c-01, N21c-02 |
| GATE-SPARK-08 | M0 | N21a-24 (end-to-end and gate record) | kernels and corpus: BD-030 (U21b) |
| GATE-SPARK-09 | M2 | BD-178 | end-to-end script bead N21b-31 |
| GATE-SPARK-10 | M2 | — | BD-183 (U21b) |
| GATE-SPARK-11 | M2 | BD-173 | end-to-end script bead N21b-14 |
| GATE-SPARK-12 | M2 | BD-172 (CUDA rows) | end-to-end script bead N21b-26; Metal rows BD-174 |
| GATE-SPARK-13 | M4 | — | BD-370 (U21c) |
| GATE-SPARK-14 | M4 | BD-371 | — |
| GATE-SPARK-23 | M2 (not in the §40 register) | — (N21a-07 carries the two-limb test; N21a-10 the no-64-bit-atomic lint) | N21c-43 (U21c) |
| KILL-KIR | M2 | BD-221 (decision), N21a-13 (paper-scale reading), N21a-14 (fallback branch) | — |
| KILL-METAL | M2 | BD-217 (measurement and decision record) | decision and fallback beads N21b-08, N21c-33 |
| KILL-PARITY (a §36 gate that seed BD-217 decides) | M2 | BD-217 | its fallback: the §36 unit |
| KILL-SPECIALIZE (registered under §21.18 and §17.9) | M4 | BD-371 (final reading) | pilot N21c-17, share measurement N21c-16, first reading N21c-18 |
| KILL-BUCKET | M2 | — | N21b-19 (U21b) |
| KILL-D1 | M2 | — | N21b-34, BD-176 (U21b) |
| KILL-FLEET | M4 | — | N21c-11, BD-370 (U21c) |
| KILL-MEGA | M8 | — | N21c-22, BD-441 (U21c) |
| KILL-SLOT | M3 | — | the daemon unit (§32.21 governs); Spark side N21b-20, N21b-21 |
| INT-FJAX-01 (admission gate, conditional) | M2 | BD-221 (go or no-go), N21a-15 (the pilot) | — |

## (c) Counts

- Known answers: 28 (KA-525 to KA-552), all in `N21-KA`; each is cited with its values in at least one carrier bead of this unit or named with its carrier in another unit.
- Counterexamples: the chapter's registry `N21-CX` is U21c's. This unit's beads state and test 10 of the chapter's 18 (CX-216, -217, -218, -219, -220, -221, -224, -226, -229, -230) and two from other chapters (CX-123, CX-164).
- Refusal codes: the chapter's registry `N21-REF` is U21c's. This unit's beads state meaning, trigger, first repair and exit code for 17 of the chapter's 18 codes (601 to 617) and name 618; they also carry the build-time codes that the toolchain raises: `FROB-REF-215`, `-250`, `-251`, `-252`, and the performance codes `-862`, `-863`, `-866`.
- Invariants and proof obligations carried with tests: INV-SPARK-01 (N21a-01), -02 (N21a-02), -03 and -06 (BD-173), -04 (N21a-06, N21a-22), -05 (BD-172), -07 (BD-178), -08 (N21a-12), -09 (BD-371); PO-SPARK-01 (BD-018, N21a-16), -02 (BD-053), -03 (N21a-18), -04 (N21a-06). INV-SPARK-10 is U21c's.
- Open questions: OPEN-SPARK-1 (default implemented in N21a-01 and N21a-02; decision bead N21c-34), -2 (BD-217; decision N21c-35), -4 (BD-172; decision N21b-03 and N21c-37), -6 (BD-029; experiment N21c-03); OPEN-NUM-1 (BD-013, N21a-10, N21a-21); OPEN-MARROW-3 (BD-053, N21a-17); OPEN-ARENA-5 (BD-053); OPEN-PERF-1, -2 (BD-217).

## (d) Needs from other slices (no seed key known when drafted)

| Need | Used by | Key seen in a sibling directory |
|---|---|---|
| The kernel manifest `frob.kernel/1` with its stage signature | N21a-04, N21a-05, N21a-06, N21a-18, N21a-22 | N21b-01 |
| The device receipt type and its own validation | N21a-02 | BD-014 (U21b) |
| OPEN-SPARK-1 owner decision | N21a-01, N21a-02, N21a-03, BD-217 | N21c-34 |
| OPEN-SPARK-2 owner decision (compute envelope) | BD-217 | N21c-35 |
| OPEN-SPARK-4 decision (graph capture and pools in `cudarc`) | BD-172 | N21b-03, N21c-37 (two beads for one question) |
| Fingerprint rate probes; arithmetic-feature probes; currency and `FROB-REF-606`; `spark.devices`; the OPEN-SPARK-6 experiment | BD-029, N21a-02, N21a-10 | N21c-01, N21c-02, N21c-05, N21c-04, N21c-03 |
| Device workers, control block, capture and synchronization counters | BD-172 | N21b-02 |
| Commit fence (rule R2) | BD-172, BD-175 | N21b-07 |
| Chunks and cancellation | BD-172, BD-175 | N21b-22 |
| Arena layout planner; leases; kernel cache; cache verbs; GATE-SPARK-11 script | BD-173 | N21b-10, N21b-11, N21b-12, N21b-13, N21b-14 |
| World fault states; stall watchdog; GATE-SPARK-06 script | BD-175 | N21b-23, N21b-24, N21b-25 |
| GATE-SPARK-12 script | BD-172 | N21b-26 |
| Placement record; `spark.place` verb; cost model; GATE-SPARK-09 script | BD-178 | N21b-28, N21b-29, N21b-30, N21b-31 |
| `K-canary` start-up canaries and `FROB-REF-609` | N21a-21, N21-KA | N21b-36 |
| Specialization pilot, share measurement, first-reading decision, bit-identity fence, cohort switch | BD-371 | N21c-17, N21c-16, N21c-18, N21c-19, N21c-20 |
| KILL-METAL decision and fallback | BD-217 | N21b-08, N21c-33 |
| KILL-PARITY decision and fallback; OPEN-PERF-1, OPEN-PERF-2 | BD-217 | the §36 unit (not seen) |
| The no-wrap bound in the step plan; the histogram drain; GATE-SPARK-23 | BD-172, N21a-07, N21a-10 | N21c-39, N21c-40, N21c-43 |
| The `rewrite-trace` proof kind and its independent checker (T-27) | N21a-19, N21a-15 | the §12 unit (not seen) |
| The certificate fallback of KILL-LEMMA | N21a-14, BD-221 | the §12 unit (not seen) |
| `fs-ad` dual numbers and `fs-math` deterministic functions through the airlock | N21a-11, N21a-13, N21a-20 | the §33 units (not seen) |
| The Guarded predicate list of Marrow with input ranges | BD-053 | the §13 unit (not seen) |
| The normal-variate construction of `fs-rand` for addressed draws | N21a-11 | BD-180 |

## (e) Contradictions, ambiguities and overlaps, and what was done

1. **Overlap with sibling units, to merge when loading.** The three Spark units worked concurrently. These pairs
   cover the same mechanism and should be merged or one of each pair dropped:
   - N21a-02 (device results at the agent boundary, INV-SPARK-02) and N21c-28.
   - N21a-06 (footprint containment, PO-SPARK-04) and N21c-29.
   - N21a-21 (`f32dd` in the IR, error-free transformations) and N21b-36 (double-float accumulators and canaries);
     N21a-21 is the IR lowering and CPU proof, N21b-36 adds the device canaries.
   - N21a-07 (two-limb sums inside `K-reduce` and `K-seg-reduce`) and N21c-41.
   - N21a-10 (dialect rule 7: 64-bit integers and atomics) and N21c-42.
   - N21a-03 (ADR-SPARK-01 to -10 and the tier register) and N21c-24 (routes not taken), N21c-27 (invariant
     register), N21c-32 (gate register): adjacent registers, not duplicates.
   - This unit's seed beads BD-172, BD-173, BD-175, BD-178 and BD-371 state their mechanisms in full, as the brief
     requires, and U21b and U21c also drafted those mechanisms as sub-beads. The seed beads were therefore written
     as the integration and gate-closing beads; they name the sub-beads in `needs` and use the sibling units'
     end-to-end script names (`spark_gate06_faults_cancel.sh`, `spark_gate09_placement.sh`,
     `spark_gate11_arena_cache.sh`, `spark_gate12_launch_fence.sh`) instead of owning second scripts.
   - Because other units already drafted them, this unit did **not** write: the OPEN-SPARK-1 decision bead, the
     `spark.devices` verb, the fingerprint probes, the KILL-METAL fallback bead, or the specialization pilot.
     BD-371 (a Frontier mechanism) therefore names its pilot in `needs` (N21c-17) instead of `deps`; the lead should
     turn that into a blocking dependency when keys are resolved.
2. **Seed BD-152's done-condition** reads "padding a reduction with −0.0 as the neutral element is caught". Taken
   literally it contradicts the padding rule (−0.0 is the neutral element of a sum; +0.0 is not, because
   (−0.0) + (+0.0) = +0.0). It is implemented as: the −0.0 case is the test, and padding with +0.0 is the planted
   defect that must be caught.
3. **The default capacity set** is stated two ways: {12, 18, 24, 36, 48, 64} (§21.11, OPEN-SPARK-3) and
   {12, 18, 24, 32, 48, 64} (Appendix I.5, the counterexample row CX-222). KA-547 in Appendix M quotes the 48
   bucket (worst member 33: Ω₁ ≤ 1.45, Ω₃ ≤ 3.1) while §21.11 quotes the 36 bucket (worst member 25: 1.44, 3.0).
   `N21-KA` stores the formula and both worked cases and leaves the set to OPEN-SPARK-3.
4. **The fingerprint digest.** The record is hashed whole, but its rates differ by a few percent between runs, so
   a whole-record digest cannot mean "the same machine". BD-029 stores `digest` (one measurement) and an
   `identity` over the discrete subset, following the definition U21c gave in N21c-05.
5. **Where the lints live.** The plan asks for a dialect lint and a Rust kernel lint, and separately fixes `xtask`
   at exactly six subcommands. The lints are therefore library modules of `frob-kir`, called by tests, by
   `check-generated` and by every compile path (BD-013, N21a-10), not a seventh `xtask` check.
6. **How a gate is recorded.** The conformance harness has no agent verb and no `xtask` subcommand by decision. The
   end-to-end scripts in this unit drive `frob`, and the gate receipt is written by a
   `cargo test -p frob-conformance --test <tier> -- <gate>` target from the script's collated output.
7. **The trait's home.** §21.9 places `KernelBackend` in `frob-kernels`; the layer rules of §34.3 name the device
   backend among the inversions "defined in `frob-dyn`". This unit follows §21.9 (and U21b's BD-014). The lead may
   want one sentence in the workspace beads to settle it.
8. **The 20% clause of KILL-KIR has no stated metric.** BD-221 defines two (executed share, the deciding one, and
   static share), records both with the population, and reads the smaller across backends and workloads.
9. **Which rows GATE-SPARK-03 reads.** The gate says "the CUDA rows of the targets table". BD-217 measures rows 1,
   2, 3, 7, 8 (and 12 for GATE-SPARK-04, and 13 to 15 for the no-GPU variant); the other CUDA rows are gated by
   their own beads (time to threshold, scaling, mixed batches, variants) and cross-referenced in the record.
10. **The digest's mixing function is not specified by the plan** beyond "four `u32` words, raw bit patterns, slab
    order, keyed by address, wrapping sum across worlds". N21a-23 proposes a concrete mix and makes freezing it
    (with test vectors and an avalanche criterion) part of the bead. It also fixes that padding lanes are not
    folded, which the capacity-independence of per-world digests requires.
11. **Milestone and work package of BD-053.** The seed is M1 in `frob-kir`; no M1 work package lists that crate.
    Parent chosen: WP-35 (hardware fingerprint; exact and guarded kernels), by subject. N21a-17 follows it.
12. **Fixed-point types [M], reserved.** The plan names no pilot for this Moonshot row (it reserves the words and
    stops). N21a-26 writes the semantics and defines a CPU-only pilot with a falsifier so that the pilot rule is
    met; it is priority 4 and may never run.
13. **Verb flags.** Several end-to-end steps use options on verbs owned by other units (`spark.check --vary`,
    `--list`, `--footprint-audit`; `spark.probe --suite`; `--emit-receipts` at M0, before `artifact.read` exists).
    They are written as DESIGN in the beads that use them; the owning verb beads should adopt or rename them.
