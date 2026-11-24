---
key: "NAB-25"
title: "M2 exit: sweep of the decision records against the first device profile and the engine's kill decisions"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "delivery", "gauntlet", "solid", "exit-review", "decision-record"]
parent: "EPIC-M2"
deps: ["BD-179", "BD-181", "BD-186", "BD-205", "BD-214", "BD-218", "BD-221", "N13-32", "N15b-24", "N16c-17", "N16c-34", "N17b-10", "N17b-28", "N18b-33", "N19c-18", "N20a-11", "N21a-27", "N21b-19", "N21b-34", "N21c-16", "N21c-43", "N36-27", "N40-04", "N40-33", "NAB-22", "NAB-24", "STD-E2E"]
needs: []
related: ["BD-224", "N41-09", "NAB-32", "NAB-33", "NAB-34", "NAB-49", "NAB-50"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2", "Appendix C.1.4", "Appendix B.8", "§43 (M2)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the sweep
at the exit of **M2**: the batch under the studio, on CUDA and Metal, measured against the incumbent.

M2 produces the single most cited piece of evidence in the decision records: **our own profile of the batched
step**. A large share of the killed-idea register was refuted with arithmetic on an *incumbent's* profile
(of 262 ns per world-step for a humanoid at 8,192 worlds, 53 ns in stages a design-specialized recursion could
touch, 124 ns in the solve, 38 ns in constraint construction). Those refutations name our own M2 profile as
the evidence that could reverse them. This sweep is where the reversal conditions are actually evaluated.

## Scope

In: the M2 sweep record and report; history entries on due rows; proposals; the first Moonshot-queue decision.
Out: deciding gates; the exit review (BD-224); the procedure (NAB-22).

Order of work (DESIGN). The sweep is made at the exit candidate commit, before the exit record is written, and
the exit review depends on it: the review's runner executes `decision-sweep check --exit M2` as the attached
check of NAB-22 and copies the sweep's section into the exit record, so the record must exist first. What the
sweep reads is produced before the review, not by it: the gate report at that commit (N40-04), the kill-decision
ADRs written by their decision beads, the M2 profile, and the variant record if one is in force. The dependency
list names those beads for that reason. This is the order of NAB-22's design decision 1, which every sweep
follows; what the sweep shares with the review is its independence rule (a session that implemented none of the
cited beads).

## Technical approach

`decision-sweep skeleton --exit M2`, fill, `check --exit M2`, by a session that implemented none of the cited
beads.

**1. Families due by a kill gate decided at M2.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-S15 Clock contract: rational periods, phase accumulators, seven instants, packet queues inside every checkpoint | 10.0 | KILL-SCHED | M2 | The race check misses a planted race | The fixed total order, with no enumeration |
| EV-S17 Exact-integer and guarded single-precision kernel classes | 10.0 | KILL-MARROW-GUARD | M2 | Fallback rate above 10⁻² on the contact battery on either GPU backend, or the exact path cannot be made bit-identical across backends | Strict predicates are evaluated exactly only on `strict` and the referee; GPU tiers mark worlds that came within E of a boundary and the sampled referee re-runs them |
| EV-S24 Four-dimensional contact cone, and joint-level stiction, backlash and stop rows | 8.0 | KILL-SINEW-ROWS | M2 | With stiction rows on, the fast-tier solve needs more than 1.5 times the Newton iterations of the smooth-friction model on the contact battery, or throughput falls by more than 20%, after one repair cycle | Throughput tiers apply regularized friction as a force; the rows stay on the referee and `strict`; smooth-model results carry a flag and the fidelity ledger measures the gap |
| EV-S27 Batch-composition invariance and decision-level equivalence under class D3 | 8.0 | KILL-D1 | M2 | GATE-SPARK-01 or GATE-SPARK-02 shows that a device family cannot hold bit equality with `batch-cpu` for the floating kernels of the step | That family is class D2 for itself and D3 against others, with the decision-level test of §35.4; a teleported world is a tolerance replay; cross-family comparisons are unpaired; the replay claim of §6 and §29 is reworded |
| EV-S32 Convex compliant contact with a referee audit | 6.7 | KILL-CONTACT | M2 | GATE-FID-01 fails on slip or creep after parameter mapping and one repair cycle | CL-2 becomes the default at a published lower throughput; if that also fails, locomotion claims are restricted to the referee tier and the fast tiers are labeled exploratory |
| EV-F23 Mixed-topology model variants in padded size buckets | 3.8 | KILL-BUCKET | M2 | More than eight buckets are needed to hold Ω₃ ≤ 2 on the reference family corpus, or the mixed batch of GATE-SPARK-10 runs below 0.5 of the harmonic mean of its homogeneous rates after one optimization pass | At most nine padded per-family templates, each a homogeneous environment, scheduled in turn; parameter variants are unaffected |
| EV-F23 Mixed-topology model variants in padded size buckets | 3.8 | KILL-PHYLUM | M2 | GATE-PHYLUM-03 fails: below 0.5 of homogeneous throughput on 512 designs, more than eight buckets, or arm B covers at least 95% of the mutation corpus and is faster | Per-family templates become the engine; cohorts are formed by exact tree with per-design parameter tables; the table-driven path remains for admission and preview on `batch-cpu` |
| EV-F26 Derived kernels with dimension typing | 3.0 | KILL-KIR | M2 | At M2 the derivation toolchain cannot express or emit more than 20% of the kernel arithmetic, or emitted kernels miss BET-01's speed bound after one optimization pass | Arithmetic is hand-written in Rust and the dialect and bound by differential tests; certificates fall back per KILL-LEMMA |

Family and kill-gate pairs due at this exit: 8.

The parked family "specialized kernels for tournament survivors" (EV-P18) is not decided at M2, but its profile
clause is *measured* at M2: the sweep records the specializable share and whether the reopening condition (at
least one third) is met. Throughput parity has its own kill gate, KILL-PARITY (at M2 exit, after one
profile-guided optimization pass, the paired lower bound of a physics row is below 0.33 of the incumbent on the
same card); it has no family row of its own and is recorded against KI-59 and KI-62.

**2. Killed-idea records whose reversal condition names the M2 profile or an M2 gate.**

| Record | Condition, as a number to read from the M2 evidence | If met |
|---|---|---|
| KI-03 supercompiled per-design kernels | Specializable share of the step ≥ 1/3 (the ceiling is 1/(1 − share): 1.25 at 20%, 1.5 at one third) | A proposal to fund the survivor-specialization pilot (decided at M4) |
| KI-04 equality saturation of kernel arithmetic | More than about one fifth of step time in derived arithmetic | A proposal to reopen under the rewrite-certificate core, one class-R candidate per release |
| KI-08 staged erasure as a source of speed | The step is bound by arithmetic and not by memory traffic | Reopen the lever; otherwise it stays a compiler rule |
| KI-10 copy-on-write forks | The fork copy costs more than a stated share of one batch step (recomputed estimate: about 0.5 ms for 0.25 GB, an eighth of a 4.1 ms step) | A proposal; otherwise forks stay indexed copies |
| KI-12 perfect hashing for brick directories | The dense directory threatens the device memory budget (estimate: 0.3 MB, 2.3 MB and 18.8 MB for the three sized scenes, about 0.4% of the field) | A proposal; the pool's deduplication gate (KILL-POOL: below 1.5) is read beside it |
| KI-14 an always-on referee | Measured referee rate, hence coverage of GPU world-seconds (estimate 0.1 to 2.4%) | The surviving sampler keeps its gate at M3; the strong form stays refuted unless coverage reaches the episode counts n ≈ 16·p·(1 − p)/δ² |
| KI-06, KI-11 unordered scatter; order-dependent sketches | Batch-composition invariance holds bit for bit with the ordered reduction and integer histograms | The refutations stand; any alternative must pass the same test |
| KI-42 topology as data, as a capability claim | The four-arm head-to-head: mixed batch against nine per-family templates, an incumbent's mixed batch, and the incumbent design by design (thresholds: 0.5 of homogeneous throughput on 512 designs; at most eight buckets) | KILL-PHYLUM decides whether templates become the engine |
| KI-59, KI-62 engine speed; corrected targets | Parity rows: the paired lower bound against the incumbent re-measured on the same card (target 0.67 or better; kill below 0.33); training time at no more than 1.25 times the incumbent | Targets move only with the re-measured incumbent |
| KI-67 64-bit sums with device atomics | The counts-beyond-2³² gate: 32-bit counts with a no-wrap bound checked at plan time (a 32-bit count wraps after 2³² increments, 18 minutes at 4 M world-steps/s into one bin) | The refutation stands unless every backend reports 64-bit atomics |
| KI-01 half precision | The conditioning certificate on our own contact battery | Stays refused unless κ·u₁₆ < 1 everywhere and 9k/n < 1 |

Also read at M2: the structure-exploiting constraint assembly and solve (an ACTIVE hypothesis with no killed-idea
entry: assembly and solve are 62% of the incumbent trace; the operation ledger and the device measurement
arrive here), and the kill gates of three later-added mechanisms measured on the same profile (row-class solve
queues, closed-form elimination of single-loop planar closures, limb-archetype kernels). Their families gain
rows in the matrix by ADR if they are adopted; until then the sweep lists them as `carried_forward`.

**3. Moonshot queue.** The first item, the fused per-world step kernel (MQ-01), becomes eligible "after the M2
profile". The sweep evaluates its precondition: if the profile shows the step is not bound by what fusion
removes, the sweep writes the ADR that proposes MQ-01's `state` as `falsified` (a word of the queue's closed
vocabulary, NAB-21; the record is never deleted) and lists the item under the sweep record's queue `freed` with
the reason `falsified-before-pilot`, so that its slot is never taken. The queue record's `state` field is
written only by the `release` transition of N41-09, whose command lands at M5: until that transition is applied
with this ADR the record still reads `queued`, and the sweep record is what the exits of M2 to M4 read for
MQ-01 (an item in that state is never activated, because every activation is itself a transition). Otherwise it stays
queued for its one-week pilot at M8 (paired lower bound of 2.0 on the whole step at 8,192
worlds with digests unchanged). The proxy-adequacy pilot of MQ-02 needs no new solver and may be scheduled from
M3; the sweep notes it.

**4. Not-yet-placed ideas with a trigger read at M2** (NAB-49): *coherent sensor ray bundles* (trigger: the ray
stage exceeds its share in the M2 profile); *scene-side incremental geometry rebuilds* (trigger: the pooled
variant path misses its target). Each is recorded `trigger-not-met` or as a proposal.

**5. Idea register.** Rows whose carriers' gates are recorded at M2: many different robots in one launch;
determinism engineered instead of asserted; teleport on single-source kernels; sparse brick fields; the two
cards as one pipeline.

**6. Source register.** Reading tasks for M2 gates closed; `[[unverified]]` items NV-03 (Metal rounding and
subnormals) and NV-05 (an incumbent on Apple silicon) settled or carried forward.

**7. The no-GPU variant.** If the variant record of the M2 no-GPU exit is in force (N40-33), every due item
whose evidence is a device measurement is `carried_forward` to M3 with the reason "gate recorded not run", and a
family whose kill gate that record defers to M3 is `carried_forward` with the deferral's ADR; nothing is
re-scored on absent evidence.

## Acceptance criteria

- [ ] `docs/delivery/exits/M2/decision-records.json` exists and `decision-sweep check --exit M2` passes.
- [ ] Each of the eight family-and-kill-gate pairs of step 1 has an outcome citing the decision ADR; a fired
      gate's family names its fallback.
- [ ] Each row of step 2 has the measured number beside the condition, and `condition_met` true or false.
- [ ] MQ-01 has an outcome: `queued` with the profile evidence, or an ADR proposing `falsified` with the
      reason `falsified-before-pilot` in the sweep record's queue `freed` list (the state change itself waits
      for the transition of N41-09).
- [ ] The sweep record exists at the exit candidate commit before the M2 exit record is written, and the exit
      runner's attached check reads it; the sweep's session implemented none of the beads it cites.
- [ ] The two triggers of step 4 are evaluated.
- [ ] In the no-GPU variant every device-dependent item is carried forward with its reason and none is
      marked unchanged.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m2_families` (`EV-S15`, `EV-S17`, `EV-S24`, `EV-S27`,
`EV-S32`, `EV-F23`, `EV-F26`; eight pairs because EV-F23 has two kill gates); `m2_profile_conditions` (on a
fixture profile with a specializable share of 0.20 the KI-03 condition is false and its ceiling prints 1.25;
with 0.34 it is true); `no_gpu_variant_carries_forward`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M2` (NAB-50): prints the due list, runs `decision-sweep check --exit
M2`, and asserts the eight pairs, the eleven conditions of step 2, the MQ-01 outcome and the two triggers of
step 4; then replays the three negative controls below (fixtures under
`tests/fixtures/sweep/M2/controls/` written by this bead, the hook of NAB-50) on scratch copies of the sweep record and asserts that
`check` fails on each with its reason. Logged through the shared helper of STD-E2E to
`target/e2e-logs/decision_records/<utc>.ndjson` with a transcript, every line with `ts`, `script`, `bead`
(`NAB-25`), `step`, `event`: `cmd` lines with the exact command line, exit code and wall time; one `assert`
per pair with the kill gate, the ADR identifier and the outcome; one `metric` per condition of step 2 with the
measured number, its unit, the threshold and `condition_met`; the profile's receipt identifier and the digests
of the gate report and of the sweep record; `assert` lines of kind `negative-control` with `caught`; on failure
the first due item without an outcome, or the first condition whose number does not resolve to a receipt, with
the path of the sweep record; a `summary` with assertions passed and failed and the kill gates read.

### Known answers and negative controls

Known answers: seven families, eight pairs; arithmetic of the conditions: 262/209 = 1.25; 1/(1 − 1/3) = 1.5;
53/262 = 20%; 124/262 = 47%; 2³² ÷ (4 × 10⁶ per second) = 1,074 s ≈ 18 minutes; 0.49 GB ÷ 1,008 GB/s ≈ 0.49 ms.
Negative controls: a sweep that reopens KI-03 on a share of 0.25 (condition not met; a proposal there fails the
check); a sweep that leaves MQ-01 without an outcome; a no-GPU sweep that marks a device-dependent family
`unchanged`.

## Considerations

- The profile is one measurement on two machines. A condition met by a narrow margin opens a proposal with the
  margin stated; it does not move a row.
- Whatever KILL-PHYLUM decides changes the vocabulary of half the idea register (cohorts or per-family
  templates as the engine); the idea records are updated to the name of whatever shipped.

## References

Appendix C.1, C.2 (entries on arithmetic, precision, kernels, memory and forks), C.1.4, B.8; the M2 milestone
and its exit review.
