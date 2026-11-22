# Coverage of drafting unit U21b

Chapter §21 Spark (GPU backends and the kernel toolchain), subsections §21.9 to §21.15, and the nine seed beads
BD-014, BD-017, BD-020, BD-030, BD-171, BD-174, BD-176, BD-179, BD-183. Units U21a (§21.1 to §21.8) and U21c
(§21.16 to §21.26) own the rest of the chapter and its three registry beads (`N21-KA` is U21a's; `N21-CX` and
`N21-REF` are U21c's).

Files: 51 beads. Nine seed beads; 42 new beads (N21b-01 to N21b-42); no registry bead (none is assigned to this
unit). By milestone: M0 10, M2 39, M3 1, M4 1. By kind: 3 pilots (N21b-04, N21b-20, N21b-41), 3 kill-gate
decisions (N21b-08, N21b-19, N21b-34), 3 open-question decisions (N21b-03, N21b-09, N21b-18), 8 end-to-end script
beads (N21b-14, -25, -26, -27, -31, -35, -39, -40) beside the four seed beads that own their own scripts (BD-020,
BD-174, BD-179, BD-183).

This unit was drafted in two sittings. The first wrote 49 files and was interrupted; the second checked them,
wrote N21b-39 (which N21b-08 and BD-174 already cited) and N21b-42, made three small additions to existing files
(N21b-34 now depends on N21b-39; BD-183 carries the end-to-end bound on family batching; BD-179 names
construction C-05), and wrote this file.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §21.9 | Backends | **Trait and CPU backend:** BD-014 (`KernelBackend`, the device receipt with AE-01 to AE-12, `batch-cpu` as a backend, the harness; the strict profile as a second step plan; the `wgpu` route not admitted), N21b-01 (the manifest `frob.kernel/1` and stage signature that the seed's title names; footprint and alias validation). **CUDA:** BD-172 (U21a's seed: buffers, step plan, graph capture, pools, receipts); N21b-02 (one context and one host thread per device, the two-card shard rule, the replay-invariant control block, capture and in-chunk synchronization counters); N21b-03 (OPEN-SPARK-4: graph capture and pool bindings in the pinned `cudarc`, or the declared fallback). **Metal:** BD-174 (the backend as a whole: gateway facts, the full step on this Mac), N21b-04 (pilot: is Metal a rollout tier), N21b-05 (packed slabs, offset table, `#define` constants, 16-bit packing, the 31-argument and offset-zero limits), N21b-06 (one batch per chunk, one worker, device-timed receipts, profiling mode, rules R1, R3, R4, R5, R6), N21b-07 (rule R2: the commit fence, `K-commit`, four attempts, `FROB-REF-607`), N21b-24 (the uninterruptible wait: watchdog and supervisor), N21b-09 (OPEN-SPARK-5: no second binding without an owner decision), N21b-08 (KILL-METAL), N21b-39 (the run-to-run clause of KILL-METAL) |
| §21.10 | The device arena, leases and the binary kernel cache | BD-173 (U21a's seed, integrating); N21b-10 (slab catalogue, 256-byte alignment, packing under argument and largest-buffer limits, no allocation after creation, INV-SPARK-03); N21b-11 (`SlabLease`, host-read and host-write rules, Metal borrows and CUDA epoch checks, `FROB-REF-614`, INV-SPARK-06); N21b-12 (BLAKE3 key, CUBIN or expanded source, digest-checked hits, 4 GiB cap and eviction, `FROB-REF-612`, key change marks the check stale); N21b-13 (verbs `spark.cache` and `spark.precompile`); N21b-14 (end-to-end, GATE-SPARK-11); N21b-29 (the residency map printed by `spark.place`) |
| §21.11 | Execution model: shape classes, variants, buckets and the step plan | **Step plan and rollout:** BD-172 (U21a: the record `frob.stepplan/1`), BD-014, N21b-02, N21b-06, N21b-22. **Shape class, bucket, world row, layout rules:** N21b-15 (`ShapeClass` device view, `Bucket`, `WorldRow`, contiguous runs in multiples of the execution width, neutral padding, masks everywhere, INV-SPARK-04). **Parameter variants [S]:** N21b-15, BD-183 (clause 1 of GATE-SPARK-10). **Topology variants [F]:** N21b-16 (variant tables of about 192 bytes per link, admission into a free slot between chunks, `FROB-REF-611`), pilot BD-144 (another unit's seed), BD-183. **Design-family buckets [F]:** N21b-41 (pilot), N21b-17 (constants shared by content hash). **How many buckets:** BD-171 (Ω_p, the bounds, worked values, break-even, the exact planner, `phylum.plan`), N21b-27 (end-to-end for `phylum.plan`), N21b-18 (OPEN-SPARK-3: freeze the set), N21b-19 (KILL-BUCKET). **Slot dispatch [F]:** N21b-20 (pilot on `batch-cpu`), N21b-21 (persistent environments, refill at masked resets, 64 jobs and 5% reserve, per-job fallback; Spark side of KILL-SLOT). **Chunks and cancellation:** N21b-22 (one synchronization per chunk, the 250 ms target, request, drain, finalize, AE-10). **Faults:** N21b-23 (`Faulted`, `Degraded`, the non-finite sentinel, caller policies, refusals 601, 602, 603, 608, 610), BD-175 (U21a's seed, integrating). **Stalls:** N21b-24 (`Stalled`, `FROB-REF-615`, restart from the committed epoch). **Scripts:** N21b-25 (GATE-SPARK-06), N21b-26 (GATE-SPARK-12), BD-183 (GATE-SPARK-10) |
| §21.12 | The placement planner and its receipts | BD-178 (U21a's seed, integrating); N21b-28 (the record `frob.placement/1`, whole-denominator counters, the instrumented shim, rule 5 on device-timed receipts); N21b-29 (verb `spark.place`: strict, prefer, mixed-allowed; refusals 604, 605, 616; INV-SPARK-07); N21b-30 (the cost model for small jobs, calibrated from the fingerprint; the 13 µs inside 290 µs case); N21b-31 (end-to-end, GATE-SPARK-09) |
| §21.13 | The CPU-versus-device check | N21b-32 (verb `spark.check` v0, integer ulp histograms, class comparators, the divergence ledger row); BD-020 (GATE-SPARK-00 and INT-FSIM-11 on `K-cay-step`); BD-176 (every M2 kernel, the full step, rollouts, check currency, `FROB-REF-605` and `-864`, GATE-SPARK-02); N21b-33 (sweeps over batch size, slot, capacity, co-tenants and shards; first-divergence bisection as step, kernel and quantity); N21b-35 (end-to-end, GATE-SPARK-02); BD-177 (another unit's seed: the decision-level test for a D3 pair) |
| §21.14 | Precision | N21b-36 (`f32dd` with the split product, splitter 4097, 17 operations; the two canaries; `FROB-REF-609`; ADR-SPARK-10); N21b-37 (anchored positions: 8 m cell, offset in [−4, 4) m, PO-NUM-01, `FROB-REF-249`); N21b-38 (the conditioning figure of the scaled contact solve, PO-NUM-02, `FROB-REF-248`); BD-179 (the precision battery over 10⁶ steps out to 1 km, GATE-SPARK-05 and GATE-SPARK-18; the uncentred shift as a control; half precision not used) |
| §21.15 | Determinism on devices | BD-017 (verb `spark.probe`: the IEEE probe, the contraction detector, subnormal and NaN record, attainable class; GATE-SPARK-01); BD-030 (exact and guarded kernels on the boundary corpus; GATE-SPARK-08); N21b-40 (end-to-end for `spark.probe` on four devices); N21b-39 (run-to-run equality per device family over twelve lanes; the parity facts); N21b-42 (the lanes' negative-control mutants); N21b-34 (KILL-D1 per device family); N21b-33 and BD-181 (another unit's seed) for the batch-independence property |

Material outside the seven subsections that this unit's seed beads draw on, and where it landed:

| Source | What | Beads |
|---|---|---|
| §21.5 | The manifest and stage signature named in BD-014's title | N21b-01 (U21a's N21a-04, -05, -06, -18, -22 carry the kernel library around it) |
| §21.8 | Kernel classes; the recorded boundary pilot; representation before placement | BD-030, N21b-30 |
| §21.24 | Gate rows GATE-SPARK-00, -01, -02, -05, -06, -08 to -12 and KILL-METAL | section (b) |
| §21.25 | OPEN-SPARK-3, -4, -5 | N21b-18, N21b-03, N21b-09 |
| §17.5, §17.14, §17.19 | The bucket set, `phylum.plan`, the head-to-head that decides KILL-BUCKET | BD-171, N21b-27, N21b-19 |
| §32.21 | KILL-SLOT's ownership by the daemon chapter | N21b-20, N21b-21 (Spark side only) |
| §35.1 to §35.5, §35.7 | Determinism classes, licences, decision-level equivalence, batch independence, hazards | BD-017, BD-176, N21b-32 to N21b-35, N21b-39, N21b-42 |
| §35.8, §35.10 | Anchored positions; diagonal scaling and the conditioning figure | N21b-37, N21b-38, BD-179 |
| §36.5 | The bound on what family batching can deliver end to end | BD-183 (Considerations) |
| Appendix I.5, I.18 | Type sketches for `ShapeClass`, `Bucket`, `WorldRow`, `SlabLease`, `KernelBackend` | N21b-15, N21b-16, N21b-11, BD-014 |
| Appendix J | Verbs `spark.probe`, `spark.check`, `spark.place`, `spark.cache`, `spark.precompile`, `phylum.plan` | BD-017, N21b-32 with BD-176, N21b-29, N21b-13, BD-171 |

Identifiers carried:

| Kind | Identifier and bead |
|---|---|
| Invariants | INV-SPARK-01 (BD-014, BD-174), -02 (BD-014, N21b-28), -03 (N21b-10, N21b-14), -04 (N21b-01, N21b-15), -05 (N21b-02, N21b-16, N21b-26), -06 (N21b-07, N21b-11, N21b-26), -07 (N21b-29, N21b-31); INV-NUM-01 (N21b-15, N21b-39, N21b-42), INV-NUM-02 (N21b-15), INV-NUM-04 (N21b-42). INV-SPARK-08, -09, -10 belong to the other two units |
| Proof obligations | PO-SPARK-02 (BD-030), -03 (N21b-01, N21b-05), -04 (N21b-01); PO-NUM-01 (N21b-37), PO-NUM-02 (N21b-38) |
| Decisions | ADR-SPARK-01 (BD-014), -02 (BD-020), -03 (BD-030), -04 (BD-171, N21b-15), -05 (N21b-28, N21b-29), -06 (N21b-10, N21b-11, N21b-12), -09 (BD-174, N21b-05, N21b-06, N21b-09), -10 (N21b-36); ADR-NUM-04 (N21b-38), ADR-NUM-06 (N21b-37) |
| Open questions | OPEN-SPARK-3 (decision N21b-18), -4 (decision N21b-03), -5 (owner decision N21b-09); OPEN-SPARK-1, -2, -6 in Considerations with their fail-closed defaults (decision beads are U21c's); OPEN-NUM-1 to -4 in the beads they concern |
| Kill gates | KILL-METAL (N21b-08), KILL-BUCKET (N21b-19), KILL-D1 (N21b-34), KILL-SLOT (Spark side: N21b-21) |
| Bets and risks | BET-08 (BD-179 and the three precision beads, BD-030), BET-09 (N21b-04, BD-174, N21b-08); RISK-04, RISK-08, RISK-46, RISK-48 |
| Constructions | C-05 (BD-179, related `SPEC-C-05`) |

## (b) Gates, kill gates and admission gates

Measured or decided in this unit:

| Gate | Milestone | Measured or decided by | Mechanism and supporting beads |
|---|---|---|---|
| GATE-SPARK-00 | M0 | BD-020 (script and record) | N21b-32, BD-014 |
| GATE-SPARK-01 | M0 | BD-017 (probe and report), N21b-40 (script on four devices) | — |
| GATE-SPARK-02 | M2 | BD-176 (check and record), N21b-35 (script) | N21b-32, N21b-33, BD-177 |
| GATE-SPARK-05 | M2 | BD-179 | N21b-36, N21b-37, N21b-38 |
| GATE-SPARK-06 | M2 | N21b-25 (script); BD-175 (U21a's seed) | N21b-22, N21b-23, N21b-24, N21b-07 |
| GATE-SPARK-08 | M0 | BD-030 (kernels, corpus, record), N21b-40 (script) | N21b-36 |
| GATE-SPARK-09 | M2 | N21b-31 (script); BD-178 (U21a's seed) | N21b-28, N21b-29, N21b-30 |
| GATE-SPARK-10 | M2 | BD-183 | N21b-15, N21b-16, N21b-18 |
| GATE-SPARK-11 | M2 | N21b-14 (script); BD-173 (U21a's seed) | N21b-10, N21b-12, N21b-13 |
| GATE-SPARK-12 | M2 | N21b-26 (script); BD-172 (U21a's seed, CUDA rows) | N21b-02, N21b-06, N21b-07, N21b-11, N21b-22 |
| ✖ KILL-METAL | M2 | N21b-08 (decision); N21b-39 (its determinism clause); BD-217 (U21a: GATE-SPARK-04) | N21b-04 (pilot), N21b-09 |
| ✖ KILL-BUCKET | M2 | N21b-19 (decision); BD-171 (the bucket-count clause); BD-183 (the mixed-batch clause) | N21b-18, N21b-27; BD-186 (another unit) |
| ✖ KILL-D1 | M2 | N21b-34 (decision, per device family); BD-176 | BD-017, N21b-35, N21b-39 |
| ✖ KILL-SLOT | M3 | Owned by the daemon chapter (BD-295). N21b-21 supplies kernel-level co-tenancy evidence and the per-job mode | N21b-20 (pilot) |
| INT-FSIM-11 (admission) | M0 | BD-020 | — |
| GATE-SPARK-18 (a §35 gate named in seed BD-179) | M2 | BD-179 | N21b-38 |
| GATE-KIN-06, GATE-PHYLUM-02 (device rows named in seed BD-183) | M2 | BD-183 | — |
| GATE-GAUNT-07 (device rows: twice on one machine, worker counts, debug and release) | M2 | N21b-39 | N21b-42 |

Gates of the chapter that this unit expects another unit to carry:

| Gate | Expected carrier | This unit's supporting bead |
|---|---|---|
| GATE-SPARK-03 | BD-217 (U21a) | N21b-02, N21b-03 (the graph route is disclosed) |
| GATE-SPARK-04 | BD-217 (U21a) | BD-174, N21b-04 (prediction), N21b-08 (reads it) |
| GATE-SPARK-07 | BD-029 (U21a), N21c-06 | N21b-30 and N21b-18 read the fingerprint |
| GATE-SPARK-13 | BD-370 (U21c) | — |
| GATE-SPARK-14 | BD-371 (U21a) | BD-176 (the check it calls) |
| GATE-SPARK-23 | N21c-43 (U21c) | — |
| ✖ KILL-KIR | BD-221 (U21a) | — |
| ✖ KILL-FLEET | N21c-11, BD-370 (U21c) | — |
| ✖ KILL-MEGA | N21c-22, BD-441 (U21c) | — |
| GATE-SPARK-15, -16 (§35) | BD-181, BD-177 (the numerics unit) | N21b-33, N21b-15, N21b-21 |

## (c) Counts

- Subsections: 7 of 7 (§21.9 to §21.15).
- Gates of the chapter measured or decided here: 10 gates (GATE-SPARK-00, -01, -02, -05, -06, -08, -09, -10, -11,
  -12) and 4 kill gates (KILL-METAL, KILL-BUCKET, KILL-D1, and the Spark side of KILL-SLOT), of the 23 the brief
  lists; the other 9 are in the second table above. Five gates of other chapters are also touched (INT-FSIM-11,
  GATE-SPARK-18, GATE-KIN-06, GATE-PHYLUM-02, GATE-GAUNT-07).
- Known answers: 21 of the chapter's 28 are cited with their values (KA-525 to KA-537, KA-540 to KA-545, KA-547,
  KA-548), which is every one that falls in these subsections; six from other chapters are also cited (KA-052,
  KA-344 to KA-347, KA-656). KA-538, -539, -546, -549 to -552 belong to U21a's and U21c's subsections.
- Counterexamples: 10 of the chapter's 18 are stated and tested (CX-216, -217, -218, -219, -220, -221, -222, -229,
  -230, -232), and one from another chapter (CX-146).
- Refusal codes: 17 of the chapter's 18 appear with meaning, trigger and first repair (`FROB-REF-601` to `-617`;
  `-618` is U21c's). Raising sites in this unit: 601, 602, 603, 608, 610 (N21b-23); 604, 605, 616 (N21b-29);
  605 currency (BD-176); 607 (N21b-07); 609 (N21b-36); 611 (N21b-16, N21b-21); 612 (N21b-12); 614 (N21b-11); 615
  (N21b-24). Codes of other chapters used: `FROB-REF-160`, `-247`, `-248`, `-249`, `-250`, `-251`, `-252`, `-561`,
  `-562`, `-740`, `-741`, `-862`, `-863`, `-864`.
- Invariants and proof obligations: 10 invariants and 5 proof obligations as listed in (a). Open questions: 3
  decided here, 7 carried with defaults. Kill gates: 4.
- Bead size: smallest 114 lines (N21b-27 and N21b-41), largest 253 (N21b-39).

## (d) Needs from other slices

`needs` entries left for the lead to resolve:

| Bead | Need |
|---|---|
| BD-176, BD-179 | §16.8: the fidelity ledger's frozen margins and envelopes (seed BD-212), read by the statistical-equivalence clause and by the precision battery |
| N21b-13 | §32: capability scopes for `admin` verbs on both surfaces (the scope a caller needs to evict cache entries) |
| N21b-21 | §32.11: the daemon's dispatch policy and queue (seed BD-295 consumes the refill interface defined here) |

Things another unit should wire to this unit's keys:

| Other unit's bead | What it should depend on here |
|---|---|
| BD-172 (U21a) | N21b-02 (workers, control block, counters), N21b-03 (OPEN-SPARK-4), N21b-07 (commit fence), N21b-22 (chunks), N21b-10 and N21b-11 (arena, leases) |
| BD-173 (U21a) | N21b-10, N21b-11, N21b-12, N21b-13; script N21b-14 |
| BD-175 (U21a) | N21b-22, N21b-07, N21b-23, N21b-24; script N21b-25 |
| BD-178 (U21a) | N21b-28, N21b-29, N21b-30; script N21b-31 |
| BD-217 (U21a) | N21b-08 reads its GATE-SPARK-04 record; BD-217 should not also close KILL-METAL |
| BD-181, BD-177 (numerics unit) | N21b-33 calls their tests from inside `spark.check`; N21b-39 supplies the run-to-run half of class D2 |
| BD-295 (daemon unit) | N21b-21 (refill interface, per-job mode); N21b-20 (pilot result) |
| BD-186, BD-185 (population unit) | N21b-19 reads the head-to-head; KILL-PHYLUM governs where it and KILL-BUCKET differ |
| BD-224 (M2 exit review) | The ADRs of N21b-08, N21b-19, N21b-34 and the records of N21b-39 |
| The verb beads for `sim.run` (BD-222 and its owners) | The conformance-only options this unit's scripts use (see (e) item 8) |
| FD-FT-01 (upstream, FrankenTorch) | Named in `related` by BD-174, N21b-05, N21b-06, N21b-09, N21b-12, N21b-24; the switch-over rule is in BD-174 |

## (e) Contradictions, overlaps and what was done

1. **Four seeds of U21a sit in this unit's subsections** (BD-172 in §21.9 and §21.11, BD-173 in §21.10, BD-175 in
   §21.11, BD-178 in §21.12). U21a wrote them as integrating, gate-closing beads that name sub-beads in `needs`;
   this unit wrote those sub-beads and the four gate scripts. The two sets agree on script names. When loading,
   the `needs` of those seeds resolve to the keys in the second table of (d).
2. **Duplicate decision beads across units.** KILL-METAL has N21b-08 here and N21c-33 in U21c; OPEN-SPARK-3 has
   N21b-18 and N21c-36; OPEN-SPARK-4 has N21b-03 and N21c-37; OPEN-SPARK-5 has N21b-09 and N21c-38. One of each
   pair should be kept. This unit's are wired into its graph (N21b-02 depends on N21b-03, BD-183 on N21b-18,
   N21b-08 on N21b-09, and nothing here depends on N21b-08 itself), so if U21c's are kept those three
   dependencies must be rewritten.
3. **GATE-SPARK-08 has two end-to-end beads**: N21b-40 here (`spark_probe.sh`, which also carries GATE-SPARK-01)
   and N21a-24 in U21a. BD-030 keeps the kernels, corpus and gate record. One script should survive.
4. **Negative controls.** U21c's N21c-30 and N21c-31 register the mutants of the chapter's known-answer table;
   this unit's mechanism beads each carry their own planted defects, and N21b-42 adds thirteen for the
   determinism lanes. The disabled commit fence appears in N21b-07, N21b-26, N21b-42 and N21c-31: it is one seam
   and should be one registry row.
5. **Run-to-run determinism on a device family has no gate of its own.** It is the second trigger of KILL-METAL,
   the first half of class D2, and one clause of the conformance gate GATE-GAUNT-07. N21b-39 measures it per
   family with a stated detection bound and writes a record (DESIGN name `frob.determinism.lanes/1`). For CUDA a
   failure has no named kill gate; N21b-39 applies the M2 stop decision ("no device family holds a controlled
   class") and says so.
6. **Seed BD-174's title names "snapshot blits".** The gateway has shared storage only, so no blit exists today
   (rule R3). BD-174 and N21b-06 implement the snapshot read as a host copy after completion behind one read-path
   type, so that the blit variant is an addition when private storage arrives through FD-FT-01.
7. **The default bucket set is printed two ways**: {12, 18, 24, 36, 48, 64} in §21.11 and OPEN-SPARK-3 (worst
   member 25 in the 36 bucket: Ω₁ ≤ 1.44, Ω₃ ≤ 3.0) and a set with 32 in the known-answer and counterexample rows
   (worst member 33 in the 48 bucket: 1.45 and 3.1). BD-171 reproduces both worked cases; N21b-18 freezes one set
   and makes the generated registries agree. OPEN-SPARK-3 is "decided by the M2 profile" yet blocks GATE-SPARK-10,
   which the profile bead depends on; N21b-18 therefore decides from the pilot, the fingerprint's launch cost and
   `batch-cpu` overflow rates, and BD-183 depends on it.
8. **Options on verbs.** `spark.check` belongs to this unit: its options `--kernel`, `--all`, `--step`,
   `--rollout`, `--sweeps`, `--localize` are DESIGN names here, and U21a's beads call the sweep option `--vary`;
   one spelling should be chosen. The scripts also use conformance-build options on `sim.run`, which another unit
   owns: `--inject-faults`, `--inject-stall`, `--withhold-kernel`, `--chunk-intervals`, `--group-multiple`,
   `--cancel-at`, `--mutant`. They exist only in a conformance build and are labelled DESIGN.
9. **KILL-SLOT and KILL-BUCKET are stated in §21.11 but governed elsewhere** (§32.21 and §17.19). This unit wrote
   the Spark-side decision record for KILL-BUCKET (N21b-19, which defers to KILL-PHYLUM where the two differ) and
   no decision bead for KILL-SLOT (N21b-21 names it in `gates` only as the supplier of kernel-level evidence).
10. **Where the trait lives.** §21.9 places `KernelBackend` in `frob-kernels`; the layer rules elsewhere name the
    device backend among inversions defined in `frob-dyn`. BD-014 follows §21.9.
11. **Tolerances "to set"** for KA-547 and KA-548 in the known-answer registry: BD-171 and N21b-27 check them to
    the printed precision of the worked values and treat the inputs of the break-even (launch cost, time per
    world-step) as a HYPOTHESIS until the fingerprint measures them.
12. **Record names not given by the plan** are DESIGN names in the beads that introduce them: the divergence
    ledger row `frob.divergence/1` (N21b-32), the lanes record (N21b-39), and the input and output schemas of the
    verbs.
