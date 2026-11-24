# Coverage of drafting unit U36: §36 The performance program

45 bead files: 6 seed beads (BD-040, BD-142, BD-149, BD-199, BD-216, BD-219) and 39 new beads (N36-01 to N36-34,
N36-KA, N36-CX, N36-REF, and N36p1-01 and N36p1-02 from polish round 1). N36-33 and N36-34 were written in the integration pass
(section (f)).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §36.1 | Position | N36-01 (the three jobs of the program; ADR-PERF-01 and -03 and the decision record of all five ADRs); N36-03 (ADR-PERF-02); N36-06 (ADR-PERF-04); N36-05 (ADR-PERF-05); BD-216 (the measurement rules as one instrument); the Frontier item "two-device strong scaling beyond the incumbent's own": N36-09 (pilot), N36-10 (KILL-SHARD), N36-12 (mechanism) |
| §36.2 | Units | N36-03 (world-step, env-step, decimation, world-second, device-second, rate; conversion; INV-PERF-01; matched and native comparability); N36-02 (matched and native setting records per comparator); BD-216 (the ordering check on every report); BD-040 (matched settings measured), BD-142 (native settings measured) |
| §36.3 | Targets | N36-01 (rows 1 to 19 and 20 to 22 as data; tolerance against ambition; the 0.67 reason; the three notes); N36-02 (reference workloads Q and H, row-capacity rule, the 10⁻⁴ `Degraded` target); N36-24 (the tolerance 0.67 and its band). Row carriers: rows 5 and 6 BD-199; rows 9, 16, 18 N36-22; row 19 N36-23; rows 20 to 22 N36-21; incumbent references reproduced by BD-040 and BD-142. Rows carried by other units' seed beads and named per row in N36-01: rows 1, 2, 3, 7, 8, 12 to 15 BD-217; row 4 BD-218; rows 10 and 11 BD-183; row 17 BD-214; row 20's own latency BD-184. The `--workload` selector that runs a reference workload by name: N36-33. The humanoid first-stage flat-ground task of row 9 (task file, target registration, recipe, equal-quality check): N36-34 |
| §36.4 | Amdahl limits | N36-04 (the seven limits as functions, recomputed from measured ledgers; the conclusion); N36-24 (the slowdown penalty table) |
| §36.5 | The speedup bound for family batches, and the ceiling-first rule | N36-05 (the bound, the family-batch application, the three usage rules, the ceiling record, `FROB-REF-866`, the recorded pilot); N36-07 (ceiling beforehand for every optimization); N36-09 (the bound applied to two-card scaling); BD-040 (the incumbent's switch cost measured) |
| §36.6 | Admission by paired lower bound; weak and strong scaling | N36-06 (the rule, protocol, A/A calibration, `FROB-REF-863`); N36-07 (optimizations: ceiling, behaviour proof, bound at least 1.03); N36-08 (two devices: shards, per-shard admission from the fingerprint); BD-199 (weak and strong scaling measured); N36-31 (script); N36-09, N36-10, N36-11, N36-12 (KILL-SHARD: pilot, decision, fallback, the work on the other branch) |
| §36.7 | The disclosure block | N36-13 (every base and added field, completeness check, `FROB-REF-862`, the reason it is mandatory; OPEN-PERF-3); N36-18 (the bytes-touched field); N36-28 (the parity-defect field) |
| §36.8 | Method | Item 1 (profile before optimizing) N36-07 with the Spark unit's profile bead; item 2 (whole workflow) BD-216; item 3 (one lever, behaviour proof) N36-07; item 4 (paired, interleaved, soak, host load) N36-06, BD-216, BD-142; item 5 (regressions are test failures) N36-14; item 6 (ceiling first and the doctrine screen) N36-05, N36-07; item 7 (measure the incumbent first) BD-040, BD-142, N36-29, N36-25; item 8 (calibrate across sizes) BD-216; item 9 (timers cover completion, INV-PERF-02; what a run reports) BD-216, N36-21, N36-23; item 10 (every estimate is an interval) N36-17, BD-219 |
| §36.9 | Where time goes | N36p1-02 (the incumbent trace, the stage-share and coverage hypotheses registered before the M2 profile with the test that settles each, the referee and `batch-cpu` coverage arithmetic); N36-15 (the outcomes scored from the profile and the referee measurement) |
| §36.10 | Memory arithmetic per world and per scene | BD-219 (per-world byte model, consequences, the Mac's limits and device budget, estimation by live intervals, refusal before allocation); N36-16 (per-scene model, shared scenes, capacity tables, the role of the Mac); N36-17 (the conformal time interval per fingerprint); N36-32 (script and gate assembly); N36p1-01 (the gate measured on every device) |
| §36.11 | The bandwidth-bound observation | N36-18 (the observation as a tested hypothesis; bytes touched per launch; the Jacobian remedies at 18, 30 and 60; the Metal bound) |
| §36.12 | Camera memory arithmetic | N36-19 (the arithmetic; frame buffers as slabs; resolution and cadence as task identity; camera-heavy evaluation on the Mac) |
| §36.13 | The distributed PPO normalization rule | N36-08 (rules 1 to 5 on two physical cards; the known answer and the shortcut that fails); BD-199 (gate records); N36-12 (rule 5's asynchronous variant); N36-31 (script). The reducer and statistics merge themselves are the learning unit's bead N23a-19 |
| §36.14 | The compute envelope question | N36-20 (what the default cannot run; classes E0 to E3; the device-hour meter; `FROB-REF-761`; the class a job needs). The owner decision OPEN-SPARK-2 and its registry are the Spark unit's bead N21c-35 |
| §36.15 | End-to-end rows | N36-21 (rows 20 to 22: the phase split, the derived figure, the per-device phase ledger; the FACT for row 20); N36-01 (the rows as data) |
| §36.16 | Known answers, gates, kill gate and open questions | N36-KA (21 known answers); N36-CX (10 counterexamples); N36-REF (5 refusal codes); gates as in table (b); N36-27 and N36-28 (KILL-PARITY and its fallback); N36-24 (OPEN-PERF-1), N36-25 (OPEN-PERF-2), N36-13 (OPEN-PERF-3, Considerations), N36-26 (OPEN-PERF-4), N36-20 (OPEN-SPARK-2, enforcement) |
| (seed assigned by keyword) | `K-fk-level` and `K-twist-level` | BD-149. Its content is not in §36: it comes from §16.6 (stage 1 of the batched step), §17.4 (model tables, joints as data, padding), §21.5 to §21.7 (manifests, skeletons, derived arithmetic), §11 and §13 (conventions) |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone | Measured or decided by |
|---|---|---|
| GATE-SPARK-19 | M2 | BD-216 (gate test and record); N36-30 (end-to-end script); clauses implemented by N36-03 (ordering, omitted decimation), N36-06 (paired-bound known answer, A/A count), N36-13 (disclosure refusal) |
| GATE-SPARK-20 | M2 | BD-199 (campaigns and record); N36-31 (script); N36-08 (the gradient known answer on real cards) |
| GATE-SPARK-21 | M0 (`trj` rows that need no agent); M1 (complete) | BD-040 (M0 rows); BD-142 (completion); N36-29 (script); N36-25 (the decision that unblocks it); N36-13 (the block every row is recorded under) |
| GATE-SPARK-22 | M2 | N36p1-01 (the capstone: the campaign on every device, the controls, the record as measured; it carries the seed BD-219's done-condition and its dependency BD-217); BD-219 (the verb: memory model, budgets, fit and refusal, the record that embeds the components' groups); N36-17 (time-interval clause); N36-18 (bytes-touched clause); N36-16 and N36-19 (scene and camera rows); N36-32 (script and the gate test that assembles the four clauses) |
| KILL-PARITY | M2 | N36-27 (decision and ADR); N36-28 (fallback and the defect band). The deciding measurement is the other unit's seed bead BD-217, which also lists the gate |
| KILL-SHARD | M2 | N36-10 (decision and ADR); BD-199 (deciding measurement); N36-09 (pilot and ceiling); N36-11 (fallback); N36-12 (the work on the not-fired branch) |
| GATE-CORTEX-08 (scaling clause only; defined in §23) | M2 | BD-199, N36-08, N36-31. The parity clauses are N23a-19's |
| GATE-KIN-03 (kinematics row only; defined in §16) | M2 | BD-149 |
| GATE-SPARK-03 (defined in §21) | M2 | Unblocked by N36-24 (OPEN-PERF-1); measured by BD-217 |

Identifiers of the chapter and where they are carried: INV-PERF-01 (N36-03, BD-216); INV-PERF-02 (BD-216, N36-21,
N36-23); ADR-PERF-01 to -05 (N36-01, N36-03, N36-06, N36-05); OPEN-PERF-1 (N36-24), OPEN-PERF-2 (N36-25),
OPEN-PERF-3 (N36-13), OPEN-PERF-4 (N36-26), OPEN-SPARK-2 (N36-20; decision N21c-35), OPEN-SPARK-3 (N36-02,
Considerations; decision by the Spark units); RISK-09 (N36-06, BD-216, N36-14, N36-29), RISK-10 (N36-20), RISK-25
(BD-040, BD-142, N36-04, N36-25), RISK-30 (N36-24, N36-27, N36-28), RISK-45 (N36-07, N36-13, N36-14), RISK-46
(N36-08, BD-199, N36-10); BET-09 (N36-01 row 12, BD-142); the pre-mortem entries on throughput under the parity
band, on quadruped missions and on "nothing confirmable existed" (N36-14, N36-27, BD-040, N36-04).

## (c) Counts

- Known answers carried: 21 (KA-806 to KA-826), all in N36-KA with carriers; tolerances left "to set" are set there.
- Counterexamples carried: 10 (CX-374 to CX-383), all in N36-CX with executable tests and owners.
- Refusal codes carried: 5 (`FROB-REF-862`, `-863`, `-866`, `-761`, `-602`) in N36-REF with triggers, repairs, exit codes and raising sites; plus one rejection the plan requires without a code (the unit-ordering violation), for which a code is proposed.
- Gates: 4 convergence gates and 2 kill gates of the chapter; 3 gates of other chapters touched.
- Subsections: 16.

## (d) Needs from other slices (no seed key could be named)

- **OPEN-SPARK-2 decision** (the Spark unit's N21c-35): the registry `registries/compute_envelope.toml`, the class in force and the needed-class function. N36-20 reads it. Both beads describe the needed-class computation; the lead should give the function one home (suggested: N21c-35 owns the registry and the pure function, N36-20 the meter and the admission path).
- **The M2 step profile and funding screen** (the Spark unit's N21a-27): per-stage and per-kernel shares, the doctrine thresholds 50% and 15%. It also implements a ceiling calculator and raises `FROB-REF-866`; it should call the bound and ceiling record of N36-05 (M0) instead of defining its own.
- **The incumbent harness interface** (seed BD-039, unit U39a, not yet drafted when this unit was written): it must emit `frob.perf.disclosure/1` (N36-13) and typed rates (N36-03) for every incumbent figure, expose one command per arm, workload, mode and settings, and take its pins from `registries/comparators.toml` (N36-25).
- **The two-shard reducer and statistics merge** (the learning unit's N23a-19) and the attainment rule for time to threshold (§23.27); a humanoid first-stage flat-ground training task for target row 9.
- **Arena layout planner in plan-only mode** with slab live intervals (the Spark unit's N21b-10), the allocated-bytes counter and peak figure in run receipts, kernel manifests with footprints, and the `batch-cpu` access recorder (N21c-29, since superseded by N21a-06).
- **The four Kinesis output groups of `sim.estimate`** (the dynamics unit's N16b-26, which depends on BD-219) and the shared GATE-KIN-03 parity harness (N16a-17) for BD-149.
- **Fingerprint fields** `peer`, measured bandwidth, launch and commit costs, compile times (N21c-01, N21c-03).
- **The ten-fold ledger** `frob.tenfold.ledger/1` with its phases and closure rule (§39.6), the pilot register (§38.5), the gate register and ADR format (§40.1), the known-answer, counterexample and negative-control registry formats (§38.1, §38.6).
- **The refusal registry rows** of `761` and `862` to `866` (§35.11, the numerics unit) and of `602` (N21-REF); a code for the unit-ordering violation.
- **The `smoke` battery** (§24.9), the design corpus, the standard battery and fidelity ledger (seed BD-212), per-world digests (seed BD-203).
- **The daemon's per-device queues, budgets and conformal calibration for all verbs** (seeds BD-295 to BD-297) for the independent-lanes fallback and the M3 generalization of the time interval.
- **OPEN-KIN-1** (the default physics step; the dynamics unit) as the trigger of N36-26; **OPEN-SPARK-3** (capacities of Q and H) for N36-02.
- **The `fast-training` profile and its licences** (seed BD-269) for N36-12.

## (e) Things found contradictory, ambiguous or hard to turn into work, and what was done

1. **A seed bead that is not this chapter's.** BD-149 (`K-fk-level`, `K-twist-level`) was mapped to §36 by keyword. It was written as a kernel bead under WP-15 from §16.6, §17.4 and §21. The plan does not say how a helical joint's axial travel is stored on the batched tiers; BD-149 computes it from the lifted angle and records the alternative.
2. **Row capacity rounding.** §36.3 says "rounded up" and raises 152 to 160 for Q; §16.6 says "rounded up to a multiple of 8", but 152 is already a multiple of 8. N36-02 stores 160 and 320 as data and uses a granule of 32 (the execution width), which reproduces both, and flags the reading.
3. **Two plan-time CPU readings disagree.** §36.3 row 13 gives CPU MuJoCo on the Mac at 576 k world-steps/s for a 35-degree-of-freedom humanoid on 36 threads; Appendix H.5 gives about 410 k steps/s for a reference quadruped on 36 threads; row 14 says a quadruped step is cheaper. BD-142 re-measures both and requires a written reconciliation.
4. **No refusal code for an ordering violation.** GATE-SPARK-19 requires the harness to reject a report that violates the unit ordering; no code is assigned. It is reported under `FROB-REF-862` with pointer `rates.ordering` until the registry assigns one (N36-03, N36-REF).
5. **What a "rate" times.** §36.2 defines a rate over wall-clock from submission to completion; §36.7 discloses warm-up and duration, and incumbents publish steady-state figures. BD-216 records both a primary rate (submission to completion) and a steady-window rate; gates and ratios read the primary rate.
6. **GATE-SPARK-22's time clause against a 90% interval.** "At least 90% of held-out runs inside their intervals" is failed about half the time by an interval whose coverage is exactly 90% (§32.9 says the 90% interval is reported). N36-17 reports two levels and scores the gate on the 95% interval, declared in advance; the reading is flagged for the gate's owner.
7. **Envelope classes E2 and E3** are given only as device-hours (no per-run time, cores or memory). N36-20 lets them inherit E1's machine limits until the owner states otherwise.
8. **OPEN-SPARK-2 is stated twice** (§21.25 and §36.16). One decision bead exists (N21c-35); this unit wrote only the enforcement (N36-20) to avoid two records for one question.
9. **KILL-PARITY is listed by three beads** (the measuring seed BD-217, the delivery unit's kill-gate ledger, and N36-27). N36-27 is written as the single place the ADR is produced; BD-217's text says its kill decisions are written as ADRs and names this decision bead in its needs.
10. **The disclosure block and unit types are needed at M0**, but the seed that names them (BD-216) is at M2. They were split out as M0 beads (N36-03, N36-13) so that the M0 rows of GATE-SPARK-21 can be recorded "under the disclosure block".
11. **Where the code may live.** The harness has no verb and nothing may depend on `frob-conformance`, yet run verbs must emit rates and disclosure blocks and `sim.estimate` needs the model. Types and pure checks were placed in `frob-core` (module `perf`), the estimator model in `frob-kernels`, the harness in `frob-conformance`; `frob-core`'s scope cap (8 thousand lines) should be checked against this addition.
12. **Work package of the scaling beads.** The work-package table lists GATE-SPARK-20 under WP-47 and the two-device update and scaling rows under WP-48; the WP-47 epic itself puts the two-device update and KILL-SHARD out of scope. BD-199 and its children are under WP-48.
13. **Target row 9** (humanoid time to threshold on two cards) has a tolerance and no gate, needs a humanoid first-stage flat-ground task that is not among the M2 task families, and ten pairs of it exceed the 12 device-hours of one gate measurement. N36-22 measures what the envelope allows and records `not judged` or `not run` otherwise.
14. **Target row 19** prints "under 5 s" and "under 60 s" with no percentile; N36-23 reads both at the 95th percentile and reports the maximum beside it. Rows 16 and 18 likewise had no carrier and are in N36-22.
15. **Small rounding differences** kept as printed and tested exactly: 17.5 printed as 17 (row 21); "20 to 27" homes at 1 cm from 26.7; "about 3,200" two-camera worlds from 3,255; "about 5.8 M" quadruped worlds from 5.88 M.
16. **The card's peak arithmetic rate** used in §36.11 (it implies about 8.3·10¹³ operations per second) is not in the source register; N36-18 marks it to be re-read.
17. **Published references are on other hardware** (an RTX 5090, an RTX 6000 Ada, an A100), so "reproduced within 20%" will usually require the written explanation; BD-040 and BD-142 make the explanation file a checked artifact and replace planning values by measurements.
18. **An addition, not in the plan's text:** N36-09 notes that strong scaling at constant global workload gains on collection only by 2·T(W/2)/T(W), which is 1 where a card's throughput is still linear in its world count. This follows from the size curve the harness measures and sharpens the ceiling the plan already asks for.

## (f) Changes made in the integration pass

- **Two beads added.** N36-33: more than forty beads of the device, numerics, telemetry and performance units write `frob sim run --workload Q`, an option the verb registry does not list and no bead carried; the selector, its refusals and the workload stamp on receipts are now one bead. N36-34: target row 9 needs a humanoid first-stage flat-ground training task that no unit defined (item 13 of section (e)); the task file, the registration of its target, the recipe and the equal-quality check are now one bead, and N36-22 depends on it.
- **BD-219 is the capstone of GATE-SPARK-22.** As drafted, N36-16, N36-17 and N36-19 depended on BD-219 while BD-219's done-condition needed their clauses. The components now export slabs, group values, repairs and the time interval, and depend on the slab catalogue and the harness; BD-219 depends on all four, embeds their groups and writes the record.
- **Polish round 1: the gate split from the verb.** More than forty beads depend on BD-219 for the verb and its refusal, and BD-219 waited on the M2 parity rows (BD-217) and on a gate record that only its own dependent script (N36-32) could write. The gate's campaign and record, with the seed's dependency on BD-217, are now N36p1-01, which depends on BD-219, the four components and N36-32; N36-32 owns the gate test that assembles the record; BD-219 is the verb and keeps every mechanism of the seed.
- **Polish round 1: where-time-goes predictions registered first.** N36-15 registered the stage-share predictions and depended on the profile bead N21a-27, which records the profile at its closing, so the predictions could not precede the measurement. Registration, its lint and the coverage function are now N36p1-02 (M1), requested as a dependency of N21a-27 and BD-214; N36-15 scores the outcomes.
- **N36-REF depends on no mechanism.** The raising beads are related; the rows are owned by the numerics and device registries.
- **BD-199 no longer depends on the higher-numbered seed BD-216**; it depends on the harness parts it uses.
- **N36-02's record type is in `frob-core`**, because run verbs resolve workloads and nothing may depend on the conformance crate (item 11 of section (e)).
- **N36-11's default routing has an M2 rule**; the daemon's shortest-estimated-work assignment is a later milestone.
- **The ordering rejection** is stated with the same code and pointer in N36-03, BD-216, N36-30 and N36-REF (item 4 of section (e)).
