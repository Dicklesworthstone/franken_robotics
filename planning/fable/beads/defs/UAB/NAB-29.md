---
key: "NAB-29"
title: "M6 exit: sweep of the decision records after the frontier-physics epics; first Moonshot-queue turnover"
type: "task"
priority: 3
milestone: "M6"
labels: ["m6", "delivery", "gauntlet", "solid", "exit-review", "decision-record"]
parent: "EPIC-M6"
deps: ["N15b-16", "N15b-23", "N16c-28", "N19b-11", "N19b-15", "N19b-19", "N19b-20", "N19c-37", "N22c-26", "N23b-37", "N25c-46", "N25d-05", "N31b-23", "N38a-09", "N38a-38", "N38a-39", "N40-04", "NAB-21", "NAB-22", "NAB-28", "NAB-50", "STD-E2E"]
needs: []
related: ["BD-427", "NAB-32", "NAB-35", "NAB-49"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.1.4", "Appendix C.2", "Appendix B.8", "§43 (M6)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the sweep
at the exit of **M6**: frontier physics, a set of epics behind feature flags, each with its plan, its pilot and
its kill gate (robot-advected films, payload slosh, articulated cord and rug proxies, coupled multiphysics
islands, soft soil at scale, tactile sensing, the rotor port, RGB and splat scenes, moving people, gradients
through contact, lifecycle compression, the hard-contact law on GPU tiers, geometric phase for locomotion,
symmetry at the instant of contact).

Two things make this sweep different. First, it is the Moonshot queue's first real turnover: seven of the 21
queued items have M6 as their earliest milestone and two more are decided or land with it (coupled islands,
after M6 on its proxy pilot; lifecycle compression, whose work lands with M6), and the cap is four active at
once, so the order of entry matters. Second, M6 is where the deferred question of *gradients through contact* is revisited, which several
parked and killed ideas wait on.

## Scope

In: the M6 sweep record and report; history entries; queue turnover; proposals. Out: deciding gates; the exit
review (BD-427), which comes after this sweep, runs its `check` as an attached check and copies its section into
the exit record; the procedure (NAB-22).

Order of work (DESIGN, as at M2 to M5 in NAB-25 to NAB-28). The sweep is made at the exit candidate commit,
before the exit record is written (NAB-22 design decision 1). What it reads is produced before the review, not
by it: the gate report at that commit (N40-04), which carries GATE-HAB-07 to -09, GATE-AUGUR-20 and -21 and
GATE-FOUNDRY-12; the kill-decision ADRs written by their decision beads: N19b-15 (KILL-SLOSH), N19b-19
(KILL-PROXY), N19b-11 (KILL-ADVECT), N15b-23 (KILL-SINEW-ROTOR), N25d-05 (KILL-AUGUR-SYMEVENT), N31b-23
(KILL-LIFECYCLE), N25c-46 (KILL-AUGUR-PHASE) and N19b-20 (KILL-LIVING, with the proxy-adequacy pilot KI-17 and
MQ-02 read); the decisions of the row-less gates of step 1's last paragraph (N15b-16, N16c-28, N19c-37,
N22c-26, N23b-37, N38a-38, N38a-39); the pilot register (N38a-09) and the queue register (NAB-21). The
dependency list names those beads for that reason. The session that performs the sweep implemented none of the
cited beads.

## Technical approach

`cargo run -p frob-conformance --bin decision-sweep -- skeleton --exit M6` (the tool of NAB-22; below shortened
to `decision-sweep …`), fill, `decision-sweep check --exit M6`; the exit runner of BD-427 runs the same `check`
as the attached check `decision-sweep`.

**1. Families due by a kill gate decided at M6.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-F35 Payload slosh by an equivalent mechanical model | 3.0 | KILL-SLOSH | M6 | The frequency gate fails after one repair cycle, or sprung masses change no ranking on the carried-container battery | Rigid ballast, with a spill metric computed from payload acceleration |
| EV-F36 Articulated cord and rug proxies | 3.0 | KILL-PROXY | M6 | The proxy pilot's falsifier fires (Solid proxies already reproduce the failures of the articulated ones), or the cost exceeds its target with no route to less | Solid proxies in training batches; articulated proxies in evaluation batches only, or not at all |
| EV-F42 Robot-advected film fields | 2.0 | KILL-ADVECT | M6, with GATE-HAB-07 | The advected film fails its mass gate after one repair cycle, or costs more than 15% of the step, or design rankings on the spill battery agree with static patches at a rank correlation of 0.95 or above | Static film-friction patches under the law of §18; advection is not built |
| EV-F43 Rotor port for flight | 2.0 | KILL-SINEW-ROTOR | M6 | The hover known answer or the port balance fails, or the hybrid variant has no consumer when frontier physics is funded | The rotor port and the variant are withdrawn |
| EV-F46 Symmetry at the instant of contact, amortized per family | 1.3 | KILL-AUGUR-SYMEVENT | M6 | The identity cannot separate the symmetric fixture from its mutations, or amortization shows no reduction in decision time on either GPU | Per-event finite differences; symmetry is used only on smooth kernels |
| EV-F54 Lifecycle compression | 1.0 | KILL-LIFECYCLE | M6 | GATE-FOUNDRY-12 fails | Aged model variants at hand-specified wear states, with no time axis and no compression claim |
| EV-F55 Shape-space atlas for locomotion by geometric phase | 1.0 | KILL-AUGUR-PHASE | M6 | No proposed gait is admitted by a full rollout, or trajectory optimization finds every proposed gait at lower total cost | A-09 remains a forward diagnostic; the inverse query and the atlas are parked |
| EV-F58 Coupled multiphysics islands: free-surface fluid, implicit cloth, Cosserat rod, on the CPU referee for finalists | 0.8 | KILL-LIVING | M6, before the island work package | The proxies reproduce at least 90% of the full-model failures (GATE-HAB-09), or an island costs more than 181 rigid worlds at 5% activity with no route to less | The Solid tier and the Frontier proxies are the product; coupled islands are not built; reports say deformable matter is represented by proxy |

Family and kill-gate pairs due at this exit: 8.

Kill gates decided at M6 that have no family row in the matrix are outside the pair count and are read where a
record names them: KILL-STEPADJOINT (N16c-28) by KI-23, KILL-FARFIELD (N19c-37, stage A N19cp1-02) by KI-16,
KILL-SOILLIFT (N22c-26) by KI-22 and KI-65, KILL-TACTILE and KILL-RUT (N38a-39, N38a-38, entered by dated
amendment) by the queue outcomes of MQ-04 and MQ-05. KILL-SINEW-AGE (battery ageing, N15b-16; fallback: ageing
factors are user-set constants) and KILL-SHORTGRAD (first reading N23b-37; the final reading comes with
GATE-CORTEX-22 in the gate report) are named by no record: a fired gate changes nothing in the decision records,
and a mechanism that passes gets a proposal for a row by ADR, scored with this exit's evidence.

**2. Killed-idea records whose reversal condition names an M6 gate.**

| Record | Condition read at M6 | If met |
|---|---|---|
| KI-17 full-room fluid, cloth or granular matter in every training world | The proxy-adequacy pilot: proxies reproduce fewer than 90% of the full-model failures on three scenes of 200 episodes each, and an island costs at most 181 rigid worlds at 5% activity (from 0.95 + 0.05·c ≤ 10) | A coupled island on the CPU referee is funded for finalists only; matter in every world stays refuted (152 bytes per lattice cell; 45.6 GB for one 10 × 10 × 3 m room at 1 cm) |
| KI-18 lattice fluid at control rate | Payload slosh by one to three sprung masses per axis passes its frequency gate and changes a ranking on the carried-container battery; the advected film passes its mass gate at no more than 15% of the step | The reduced models stay; a lattice fluid appears only inside an island as an offline oracle |
| KI-23 gradients through everything | The M6 review of the non-goal on gradients through contact, with the step-adjoint pilot (implicit-function adjoints through the converged solve, exact for a fixed active set) on record | Labelled, regime-local derivatives may enter the rigid core behind a flag; a gradient of a binary outcome stays refuted |
| KI-16 causal tile reuse across replays | The far-field pilot: a two-room thin-film grid, 100 forks, the measured reusable share and its recomputation cost | Far-field field reuse across forks for multi-room film and steam fields only |
| KI-22, KI-65 a lifted linear model | The one bounded experiment: a lifted linear planning model for a tracked base on one soil card, with its band width against horizon | A lifted planner for that one regime, with a per-tick regime check; nothing wider |

**3. The Moonshot queue.** The sweep records every state change since M5 and calls `next_entries(queue, M6,
active)`.

- Earliest milestone M6: MQ-03 (rotor port; pilot: hover by hand, thrust equals weight, ideal induced power
  T^(3/2)/√(2·ρ·A), and the energy ledger closes), MQ-04 (tactile micro-patch pads; 2 × 2 and 4 × 4 contacts on
  the `strict` profile against one patch), MQ-05 (persistent ruts and footprints; the leak fixture: a rut made
  in one episode has no effect on another), MQ-06 (link-chain tracks; a chain of 40 links on the referee),
  MQ-07 (hydraulic actuation; a single cylinder through an orifice against the closed form), MQ-09 (shape-space
  atlas; funded only after one proposed gait is admitted by a full referee rollout), MQ-10 (symmetry at the
  instant of contact; the mirrored landing and its four asymmetric mutations).
- MQ-02 (coupled islands) is decided by the proxy-adequacy pilot before its work package; MQ-08 (the continuum
  arm) waits for MQ-02 and is not built without it; MQ-12 (lifecycle compression) lands with M6: a 100-hour
  reference of 1.8 × 10⁸ steps compressed at least 50-fold with at most 5% error on every slow state.
- For MQ-09 and MQ-10 the Moonshot part is funded only after the Frontier part of the same section passed its
  own gate.
- With more than four eligible, entry follows the order rule: earlier consumers first; within a milestone the
  cheaper pilot first; a dependent item after its dependency. The sweep records which four entered, which wait,
  and which were removed (`passed`, `killed`, `falsified`, `expired`: NAB-21's four events that free a slot).

**4. Parked and not-yet-placed ideas that wait on M6.** *Saddle-point robust co-design* (parked until the
non-goal on gradients through contact is revisited): the sweep records the outcome of that review and whether
the idea stays parked. *Micro-slip precursors for tactile sensing* (parked behind the tactile pilot and the
import lane): recorded with the tactile pilot's outcome. *Pressure-field contact* (deferred): recorded as still
unimplemented by any contact law, or proposed.

**5. Sources.** Reading tasks for M6 gates closed: the sloshing frequencies, the squeeze-film formula and the
viscous-spreading exponents are standard references cited from memory and are confirmed against primary
sources before a known answer pins them; the low-Mach limit of the sibling lattice solver (0.3) is re-read at
the pinned commit.

## Acceptance criteria

- [ ] `docs/delivery/exits/M6/decision-records.json` exists and `decision-sweep check --exit M6` passes.
- [ ] Each of the eight family-and-kill-gate pairs of step 1 has an outcome citing its decision ADR.
- [ ] Every Moonshot-queue item with earliest milestone M6, and MQ-02, MQ-08 and MQ-12, has a recorded state;
      `active_after` is at most 4; no item entered ahead of a cheaper pilot of the same milestone without an
      ADR.
- [ ] MQ-08 is `killed` if MQ-02 was killed, and never `active` while MQ-02 is not `passed`.
- [ ] Each row of step 2 has measured values and `condition_met`.
- [ ] The three ideas of step 4 have a recorded state.
- [ ] The entries of KI-16, KI-22, KI-23 and KI-65 and the queue outcomes of MQ-04 and MQ-05 cite the decision
      ADRs of the row-less gates of step 1's last paragraph that they name.
- [ ] The sweep record exists at the exit candidate commit before the M6 exit record is written, and the exit
      runner's attached check reads it; the sweep's session implemented none of the beads it cites.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m6_families` (eight ids: `EV-F35`, `EV-F36`, `EV-F42`,
`EV-F43`, `EV-F46`, `EV-F54`, `EV-F55`, `EV-F58`); `m6_queue_turnover_respects_cap_and_order`;
`continuum_arm_follows_islands`; `frontier_part_first_for_mq09_mq10`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M6` (NAB-50): prints the due list, runs `decision-sweep check --exit M6`,
and asserts the eight pairs, the conditions of step 2, the queue before and after and the three ideas of step 4;
then replays the three negative controls below (fixtures under
`tests/fixtures/sweep/M6/controls/` written by this bead, the hook of NAB-50) on scratch copies of the sweep record and asserts that `check`
fails on each with its reason. Logged through the shared helper of STD-E2E to
`target/e2e-logs/decision_records/<utc>.ndjson` with a transcript, every line with `ts`, `script`, `bead`
(`NAB-29`), `step`, `event`: `cmd` lines with the exact command line, exit code and wall time; one `assert` per
pair with the kill gate, the ADR identifier and the outcome; one `metric` per condition of step 2 with the
measured number, its unit, the threshold and `condition_met`; the queue's `active_after` as a `metric`
(threshold `<= 4`) with the items entered, waiting and freed; the digests of the gate report and of the sweep
record; `assert` lines of kind `negative-control` with `caught`; on failure the first due item without an
outcome, or the first queue item out of order, with the path of the sweep record; a `summary` with assertions
passed and failed and the kill gates read.

### Known answers and negative controls

Known answers: eight families; nine queue items with earliest milestone M6 or decided here (MQ-02 to MQ-07,
MQ-09, MQ-10, MQ-12) and one dependent (MQ-08); 0.95 + 0.05·c ≤ 10 gives c ≤ 181; 19 × 4 × 2 = 152 bytes per
cell; 100 h × 3,600 s ÷ 2 ms = 1.8 × 10⁸ steps; the lattice Mach number √3·u·Δt/Δx at 1 m/s and 20 ms is 0.35,
3.5 and 6.9 at cells of 10 cm, 1 cm and 5 mm. Negative controls: a fifth active Moonshot; MQ-08 active with
MQ-02 queued; an island funded when the proxies reproduced 90% or more of the failures (KILL-LIVING's trigger).

## Considerations

- The kill gate on coupled islands fires when the proxies are *good enough*: that is a success of the Solid
  tier, and the sweep words it so.
- Lifecycle compression's gate was recorded at two different milestones in the planning record; the gate
  register places it at M6 and governs.

## References

Appendix C.1, C.1.4, C.2 (entries on worlds and physics); Appendix B.8; the M6 milestone and its exit review.
