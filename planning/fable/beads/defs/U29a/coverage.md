# Coverage of drafting unit U29a

Chapter §29 Chronicle: telemetry, replay, evidence. This unit owns the chapter preamble and §29.1 to §29.8, the
registry `N29-KA`, and thirteen seed beads, several of which draw on subsections owned by unit U29b (§29.9,
§29.13 to §29.16) or on other chapters (§14, §17, §21, §32). Files: 13 seed beads, 24 new beads (`N29a-01` to
`N29a-24`), 1 registry bead.

## (a) Subsections and the beads that carry them

| Subsection | Title | Carried by (this unit) | Carried by other units |
|---|---|---|---|
| §29 preamble | (the chapter has no text before §29.1) | — | — |
| §29.1 | What Chronicle owns | N29a-01 (ownership map, the two refusals Chronicle owes, the no-decision rule, the mechanism inventory of all seventeen rows); every mechanism row is carried by the bead named in the rows below | BD-090 (U28b: crate, `CONTRACT.md`) |
| §29.2 | Decisions | ADR-CHRON-02: BD-091. ADR-CHRON-03: N29a-07, N29a-13, N29a-14. ADR-CHRON-04: N29a-12. ADR-CHRON-05: BD-203. ADR-CHRON-07: BD-095, N29a-01. ADR-CHRON-08: BD-092, BD-208, N29a-22. ADR-CHRON-09: BD-209 | ADR-CHRON-01: BD-090 (U28b). ADR-CHRON-06: BD-205 (U27a) |
| §29.3 | What is stored, and where | N29a-01 (tables and their single writers; identities on every row), N29a-02 (`run.export`), N29a-03 (`frob-verify` on `.frun` with the standard library alone), BD-097 (objects by digest), BD-024 (the M0 slice of `run` and `episode`) | BD-090 (U28b: ledger schema version 1, `.frun` format, reader, writer, `verify`); BD-141 (format registry row) |
| §29.4 | The chunk protocol | BD-091 (the publishing transaction, lease fence, receipt, three properties, FROB-REF-875, -878, -885), N29a-04 (pagination pinned to the committed frontier, FROB-REF-896), N29a-05 (stopping, budget stop, terminal receipts), N29a-06 (GATE-CHRON-04 on the CPU), BD-210 and N29a-24 (devices) | BD-096 (U33a: simulation testing, CPU part of GATE-CHRON-11); BD-211 (U32a: surface cursor, idempotency); N24a-36 (U24a: `telemetry.query`) |
| §29.5 | Telemetry levels, streams and channel kinds | N29a-07 (T0 envelopes and digests at intervals; the integer-aggregate rule), BD-207 (T0, T1, T2 on devices; channel declarations for the reference robots), N29a-11 (the per-world ring, failure flush, freeze at a mode change or requested tick), BD-203 and N29a-18 (T3) | BD-090 (U28b: levels T0 to T2 on the referee, nine streams, five channel kinds and their downsampling, jump rows, gaps, FROB-REF-888, the visible or privileged label) |
| §29.6 | Choosing which worlds to trace: a reservoir by address | N29a-12 (hash, fixed rate, fixed size, shard merge, the four properties, weights and the stratified estimator), BD-207 (the mask on devices), N29a-16 (gate) | BD-214 (U16b: the background referee's own address-hashed sample) |
| §29.7 | Deterministic integer histograms | N29a-13 (quantization, bins, defaults, merge, quantile as interval, the compactor counterexample), N29a-14 (`K-x-hist`, 32-bit atomic bins, per-world rows), N29a-15 (`telemetry.histogram`), N29a-07 (FROB-REF-898), N29a-16 (gate) | U21c beads for §21.26 (the no-wrap bound, the drain into 64-bit totals, GATE-SPARK-23) |
| §29.8 | Replay, teleport and forks (GOAL-10) | BD-203 (digests, digest chain, per-stage digests, snapshots at the fence, replay, `sim.replay`), N29a-18 (what a snapshot contains; relocatable full snapshots; sparse differences; profile binding and FROB-REF-894), BD-204 (teleport, GATE-CHRON-02), BD-206 (first divergence parts 1, 2, 4, 6, 7, 8 and `telemetry.divergence`), N29a-20 (parts 3 and 5: tolerance class, missing samples), N29a-19 (GATE-CHRON-01), N29a-21 (GATE-CHRON-06), BD-024 (referee `sim.run` at M0) | BD-205 and N27a-01 to N27a-08 (U27a: fork classification into the five classes, `run.fork`, the warm-start graft refusal FROB-REF-582, GATE-CHRON-05); N27a-09 (U27a: GATE-AUTOPSY-07); N21a-23 (U21a: the `K-x-digest` mix); N21b-07 (U21b: the commit fence) |

Subsections owned by U29b on which this unit's seed beads draw, and which those beads write completely:

| Subsection | Title | This unit's beads |
|---|---|---|
| §29.9 | The energy ledger and the audit for created energy | BD-092 (referee), BD-208 (`K-ledger-step`, batched tiers), N29a-22 (three monitors, `fidelity.energy`, energy half of GATE-CHRON-08) |
| §29.13 | The artifact graph | BD-095, N29a-10 |
| §29.14 | A structured log per bead | N29a-09, N29a-10 |
| §29.15 | Simulation testing of the daemon and the ledger | N29a-06 (chunk-protocol scenarios and oracles on the CPU), N29a-24 (device rows, await-point coverage, dual runs) |
| §29.16 | Retention | BD-209 |
| §29.19 | Known answers and negative controls | N29-KA |

Material from other chapters carried by this unit's seeds: §14.9 lowering 2, §17.4 and §21.11 shape class
(BD-148); §32.10 jobs (BD-210, N29a-23, N29a-24); §32.6 artifact and run clusters (BD-097); §43.3 and §46 the
M0 artifact (BD-024); §15.11 the ledger side of the port account (BD-092, BD-208).

## (b) Gates

| Gate | Milestone | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|---|
| GATE-CHRON-01 | M2 | BD-203 (mechanism), N29a-19 (measurement, script) | — |
| GATE-CHRON-02 | M2 | BD-204 (mechanism and battery) | — |
| GATE-CHRON-03 | M1 referee; M2 single precision | BD-092 (referee row), BD-208 (single-precision row) | — |
| GATE-CHRON-04 | M1 CPU; M2 device | BD-091, N29a-04, N29a-06 (CPU rows); BD-210, N29a-24 (device rows) | — |
| GATE-CHRON-05 | M2 | — (coverage only: §29.8's fork table) | BD-205, N27a-06, N27a-07 (U27a) |
| GATE-CHRON-06 | M2 | BD-206, N29a-20, N29a-21 | — |
| GATE-CHRON-07 | M2 | BD-207, N29a-12, N29a-13, N29a-14, N29a-16 | — |
| GATE-CHRON-08 | M2 | BD-208, N29a-22 (energy half) | BD-094 and its M2 extension (U24a: failure-taxonomy half) |
| GATE-CHRON-09 | M3 | — | BD-264 (U16b); the gate-measurement bead of U29b if BD-264 does not record it |
| GATE-CHRON-10 | M1 | BD-095, N29a-09, N29a-10 | — |
| GATE-CHRON-11 | M1 CPU; M2 device | BD-210, N29a-24 (device rows) | BD-096 (U33a: CPU part) |
| GATE-CHRON-12 | M2 | BD-209 | — |
| GATE-CHRON-13 | M2 | BD-207, N29a-17 | — |
| GATE-CHRON-14 | M1 referee; M2 batch | — (BD-092 and BD-208 reserve the `algorithmic_exchange` line) | U29b (§29.22) |
| KILL-DR | M3 | — | BD-264 (U16b) or U29b's decision bead |
| GATE-AUTOPSY-07 | M2 | BD-206 (mechanism) | N27a-09 (U27a: measurement) |
| GATE-TETHER-08 | M2 | BD-210, N29a-23, N29a-24 | — |
| GATE-SINEW-02 (single precision) | M2 | BD-208 | BD-076, N15b-05 (referee row) |
| GATE-FID-06 | M2 | BD-208, N29a-22 (mechanism) | N16b-14 and U16b's gate bead (record) |

## (c) Counts

- Known answers: 14 rows (KA-707 to KA-720) in `N29-KA`. Tested by this unit's beads: KA-707, -708 (BD-092,
  BD-208), -709, -710 (N29a-22), -711 (BD-091, N29a-06, N29a-24), -712 (N29a-04), -713 (N29a-12, N29a-16),
  -714 (N29a-13, N29a-14, N29a-16), -716 replay half (BD-203), -717 (BD-206, N29a-21), -718 (BD-204), -720
  (BD-095, N29a-10, BD-209). Tested by other units: KA-715 (BD-090), KA-716 fork half (BD-205), KA-719
  (BD-264). Known answers of other chapters cited with their values: KA-295, KA-354.
- Counterexamples: the registry `N29-CX` (CX-319 to CX-325) is written by U29b. Cited with their numbers in
  this unit's test plans: CX-319 (N29a-13, N29a-16), CX-320 (BD-206, N29a-21), CX-323 (N29a-04), CX-325
  (N29a-12, N29a-16): 4 of 7. The other three belong to beads of other units: CX-321 (BD-205), CX-322
  (BD-090), CX-324 (BD-264). From other chapters: CX-147, CX-148 (N29a-22), CX-228 (N29a-13).
- Refusal codes: the registry `N29-REF` is written by U29b. Implemented by this unit's beads, with meaning,
  trigger, repair and exit code: FROB-REF-875, -878, -885 (BD-091), -896 (N29a-04), -898 (N29a-07, N29a-15),
  -894 (N29a-18, BD-203, BD-204), -869 (BD-095, N29a-01), -815 (N29a-09), -813 (BD-209), -888 (N29a-05,
  N29a-13, N29a-15, BD-209; first defined in BD-090): 10 of the chapter's 11. FROB-REF-811 belongs to BD-264.
- Invariants: INV-CHRON-01, -02 (BD-091, N29a-06), -03 (N29a-06, N29a-24), -04 (N29a-04), -05 (N29a-12,
  N29a-16), -06 (N29a-07, N29a-13, N29a-16), -08 (BD-203, N29a-19), -09 (BD-092, BD-208), -10 (BD-095,
  N29a-10), -12 (BD-209). Other units: -07 (BD-205), -11 (the summary verb, N24a-35; the count identity is
  also enforced by N29a-05), -13 (BD-264).
- Proof obligations: PO-CHRON-01 (N29a-06, N29a-24), -03 (N29a-12), -04 (N29a-13), -05 (BD-206, N29a-21), -06
  (BD-203), -07 (BD-092, BD-208). Other unit: -02 (BD-264).
- Verbs implemented: `sim.run` at M0 (BD-024), `run.export` (N29a-02), `artifact.read`, `artifact.list`,
  `artifact.export`, `run.show`, `run.list` (BD-097), `evidence.trace` (BD-095), `sim.replay` (BD-203,
  BD-204), `telemetry.divergence` (BD-206), `telemetry.histogram` (N29a-15), `fidelity.energy` (N29a-22),
  `store.status`, `store.collect` (BD-209), `job.list`, `job.status`, `job.wait`, `job.find`, `job.cancel`
  (N29a-23): 19 verbs.
- End-to-end scripts: `sim_run_referee_g1.sh`, `chronicle_known_answers.sh`, `chronicle_record_surface.sh`,
  `chronicle_chunk_protocol.sh`, `evidence_trace.sh`, `energy_ledger_referee.sh`, `packed_lowering.sh`,
  `telemetry_sampling_histograms.sh`, `telemetry_cost.sh`, `sim_replay.sh`, `telemetry_divergence.sh`,
  `energy_ledger_batched.sh`, `energy_audit.sh`, `store_retention.sh`, `job_protocol_devices.sh`: 15.
- Pilots and kill gates: none. Every mechanism of §29.1 to §29.8 is tier Solid; the two Frontier rows of the
  mechanism table (the background referee sampler and segment fidelity) belong to §29.11 and §16.

## (d) Needs from other slices

Prerequisites named in `needs` for which this unit could not name a seed key (the lead resolves them):

1. **§21.8 device digest contract** (`K-x-digest` mix, stage-tagged variant, run digest): U21a's N21a-23.
   Needed by BD-203, BD-204.
2. **§21.9 commit fence and slab leases** (`K-commit`, FROB-REF-607, FROB-REF-614): U21b's N21b-07 and N21b-11.
   Needed by BD-203, BD-207, BD-210.
3. **§21.26 wide counts** (no-wrap bound, drain of 32-bit bins into 64-bit totals, GATE-SPARK-23): U21c's
   N21c-39, N21c-40. Needed by N29a-14, N29a-15, BD-207. Those beads list BD-207 and BD-210 as dependencies,
   so the edge direction is theirs onto ours; N29a-14 needs only their *rule*, not their code.
4. **§21.7 the `f32dd` double-float pair and its canary**: U21a's N21a-21. Needed by BD-208.
5. **§29.21 decisions**: OPEN-CHRON-2 (chunk length, ring depth) blocks GATE-CHRON-04; OPEN-CHRON-3
   (identifier types) blocks GATE-CHRON-10; OPEN-CHRON-4 (retention window, store budget) blocks
   GATE-CHRON-12; OPEN-CHRON-5 (lease granularity) blocks GATE-CHRON-04 on two devices. U29b owns the decision
   beads; each of this unit's beads states the fail-closed default it builds on.
6. **§29.17 the refusal registry bead `N29-REF`** and **§29.19 `N29-CX`** (U29b).
7. **§29.22 the algorithmic-exchange split** (U29b; owners `frob-contact` and `frob-dyn`): BD-092 and BD-208
   reserve the ledger line.
8. **§29.9 `telemetry.energy`, §29.12 `telemetry.summary`, §29.4/§29.5 `telemetry.query`**: U24a's N24a-37,
   N24a-35, N24a-36. This unit's scripts call `telemetry.query` and `telemetry.energy`.
9. **§29.10 failure monitors on batched tiers** (failing bit, T1 events): BD-094 and U24a's extension. Needed
   by N29a-11, N29a-12, BD-207.
10. **§16.10 the step-side created-energy accumulation and label consequences**: U16b's N16b-14. Needed by
    BD-208, N29a-22.
11. **§37.6 the path jail as a shared facility at M1** (BD-258 is M3; GATE-SAFE-10 is M1): needed by BD-097,
    N29a-02.
12. **§38.9 the conformance-only fault hook of the step** (one unit in the last place on a named stage
    output): U27a's N27a-09 defines one; needed by BD-204, N29a-21. One shared hook should exist. (Integration
    pass: the shared hook is now bead N29a-25, on which BD-204, BD-206, N29a-21 and N27a-09 depend.)
13. **§32.7 / §32.8 surface rules** (atomic idempotency binding, authenticated cursors, `u64str`): BD-211
    depends on BD-210; BD-210 defines the job row and the unique index that BD-211 completes.
14. **§20, §22, §23, §15, §19 snapshot sections**: each owner writes its section into the container of
    N29a-18 (BD-098, BD-189, BD-192 and others are related seeds).
15. **§26.7 the `.fmcw` reader** for `sim.replay <file.fmcw>` (U26).
16. **§34.9 format registry**: the auditor's independent `.frun` reader (N29a-03) must be listed as the
    sanctioned second implementation.

## (e) Contradictions and things that could not be turned into work as written

1. **Two refusal codes for one event, twice.** §29.17 gives FROB-REF-878 for "different bytes for a committed
   chunk key" and FROB-REF-896 for a cursor used with a different pin, filter or order; §32.18 gives
   FROB-REF-810 (exit 9) and FROB-REF-806 (exit 3) for the same two events, and FROB-REF-807 for an expired
   cursor. §29.17 has no exit-code column. Done: the store raises 878 and 896; the surface reports its own
   code with the store code as cause (BD-091, N29a-04, BD-210). The registry beads should settle one code per
   event.
2. **Who owns the publishing transaction.** §29.1 and §29.4 give the chunk protocol to Chronicle; §32.10 says
   "Tether owns the leases and the publishing transaction". Done: the transaction and lease check live in
   `frob-chronicle` (BD-091), the lease *manager* that decides reassignment lives in the job engine (BD-210).
3. **Order key of pagination.** §29.4: (tick, world, episode, stream, sample identifier). §32.8.1: (instant,
   world address, channel, sample index). Done: one key (tick, world, episode, stream, channel, phase, sample)
   that both descriptions read (N29a-04).
4. **Two descriptions of histogram bins.** §29.7 bins a quantized integer (quantum u; m and s; 3.1%); §35.7
   bins the sign, exponent and top five mantissa bits of an `f32` (3.2%). Done: §29.7 governs on the evidence
   path (N29a-13); the kernel bins whichever integer it is handed (N29a-14).
5. **What GATE-CHRON-03 bounds.** "Residual below 10⁻⁹ relative on the referee" can mean the bookkeeping
   closure defect or the size of the integrator-and-solver line; the second may be unreachable at the
   referee's default step. Done: BD-092 and BD-208 measure and record both and, fail-closed, require both for
   PASS. The owner should say which reading governs.
6. **§29.9's first identity and §15.11's stored-heat rule.** §29.9 lists copper, friction and gear loss beside
   "Δ stored"; §15.11 says losses flow into thermal nodes and must not be counted again. Done: the loss lines
   are attribution, with the thermal sub-balance `losses = Δ thermal stored + heat to ambient` (BD-092).
7. **The damped pendulum has no numbers** (Appendix M says truths are "named and not written"). Done: DESIGN
   fixture (1 kg, 1 m, 9.81 m/s², 0.5 N·m·s/rad, 60°, 60 s; 4.905 J), the same constants U24a's energy-verb
   bead uses.
8. **KA-718 has no negative control.** Done: one added (a localizer that always names the last stage).
9. **Stale section pointers.** Appendix M (KA-712) and Appendix N (CX-323) place pagination in §29.12; it is
   in §29.4. Appendix I.12's note says §29.8 lists three fork classes; it lists five. No work follows.
10. **Two chunk scales share one word.** §21.11's chunk is the device commit unit (bounded by a 250 ms cancel
    latency); §29.4's is the logical extent of L control ticks. Done: named "device chunk" and "logical
    chunk" throughout (BD-207, BD-210).
11. **Per-step digests.** §29.8 compares per-step digests inside the first unequal chunk, while T0 keeps
    digests "at intervals". Done: per-tick digests are recorded for ring-holding and traced worlds and
    otherwise obtained by same-backend replay with capture (BD-203, BD-204, BD-206).
12. **Unstated defaults.** The plan gives no default trace rate, T3 interval or ring snapshot period. Done:
    1/100, one stored snapshot per logical chunk, and 0.1 s in the ring, all labelled DESIGN.
13. **Interrupted trials.** The terminal receipt has four counts; a trial cut short by a stop fits none
    cleanly. Done: counted under `unstarted` with a visible sub-count `interrupted` (N29a-05).
14. **Migration across device families mid-episode** after a device loss is not addressed. Done: refused;
    resume only on the same family and fingerprint, else the trials are `refused` (BD-210).
15. **BD-148 is not Chronicle work.** The seed was mapped to §29 by keyword with no best subsection. It is the
    design subsystem's second lowering (§14.9, §17.4, §21.11) and is written in full on that basis, parented
    to WP-15. `ShapeClass` is sketched in §17.4, §21.11 and Appendix I.5, and U17a's fitting bead prints it
    too; BD-148 owns the plain-data type and `ShapeClassId`. Appendix I.5 lists a default size of 32 where
    §17.5 and §21.11 register 36; the registered set governs.
16. **GATE-CHRON-14** is absent from the main §40 table and present in its addendum row for §29; it belongs to
    U29b.
17. **Seed dependencies.** Every seed dependency is kept. Dependencies were added where the work needs them
    (for example BD-203 on N29a-18, BD-207 on BD-091 and the reservoir and histogram beads, BD-210 on
    N29a-05, BD-203 and BD-175).
