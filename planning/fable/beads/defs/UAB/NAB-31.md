---
key: "NAB-31"
title: "M8 exit: final sweep of the decision records: Moonshot outcomes, final states, the closing printout"
type: "task"
priority: 4
milestone: "M8"
labels: ["m8", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M8"
deps: ["NAB-22", "NAB-30", "NAB-21", "BD-442"]
needs: ["§40.6: the M8 exit report with each Moonshot epic's plan, pilot and stop rule (N40-04, N40-30)", "§0.7, §38.5: the pilot ledger's final state (N00-07)"]
related: ["NAB-17", "NAB-32", "NAB-42", "NAB-49", "N00-07", "BD-435", "BD-436", "BD-437", "BD-438", "BD-439", "BD-440", "BD-441"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.1.4", "Appendix C.2", "Appendix C.3", "§43 (M8)", "§0.7", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the last
sweep, at the exit of **M8**: frontier delivery, the Moonshot epics behind flags, each with a pilot and a stop
rule (facility co-design and multi-robot handoff; skill transport through a physical interface and by a
learned residual; continuation-based linkage synthesis; firmware export of fixed-point controllers;
geometric-algebra tree policies; reachable-set tables for shields; the fused per-world step kernel).

A Moonshot is a synthesis this project proposed itself, admitted with a score that is often below the level at
which anything else would have been parked, on the condition that it stays off the critical path and is
funded only after a paper-scale pilot with a falsifier. M8 is where those conditions are cashed. The final
sweep also closes the records: after it, every row of every decision registry has a final state with evidence,
or an explicit statement that no evidence was ever gathered.

## Scope

In: the M8 sweep record and report; a final state for every family, queue item, killed idea, idea record and
unverified item; the closing printout. Out: deciding gates; the exit review (BD-442); the procedure (NAB-22).

## Technical approach

`decision-sweep skeleton --exit M8`, fill, `check --exit M8`, by a session that implemented none of the cited
beads.

**1. Families due by a kill gate decided at M8.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-F45 Skill transport: through a physical interface, and by a learned residual | 1.5 | KILL-AUGUR-TRANSPORT | M8 | The interface arm needs as many target trials as the conditioned policy, or reports coverage where the referee shows a fall | Family policies and inheritance of §23; the wrench-coverage test is kept as a static analysis |
| EV-F45 Skill transport: through a physical interface, and by a learned residual | 1.5 | KILL-SKILL | M8 | GATE-REFLEX-09 fails | Keep the exact layer and its witnesses; drop the learned residual |
| EV-F47 Reachable-set tables for small subsystems | 1.3 | KILL-TUBE | M8 | GATE-SHIELD-05 fails, or the pilot's falsifier holds | No tables are built; reduced-order barriers cover those subsystems |
| EV-F49 Geometric-algebra policy on the kinematic tree | 1.0 | KILL-GATREE | M8 | GATE-CORTEX-17 fails | The plain morphology transformer remains; `fs-ga` features are not used in policies |
| EV-F50 Numerical continuation for linkage synthesis, on the CPU | 1.0 | KILL-SYNTH-CONT | M8 | More than 0.1% lost paths, an unflagged lost path, or a missed known count | The exact lane plus multi-start CPU recovery; every result labelled `Partial` |
| EV-F53 Firmware export of fixed-point controllers | 1.0 | KILL-FIRMWARE | M8 | GATE-FOUNDRY-11 fails | Controller parameters, Q formats and the test-vector manifest only; no code |
| EV-F57 Fused per-world step kernel | 1.0 | KILL-MEGA | M8, before the work package | In the one-week fusion pilot (8,192 worlds, digests unchanged) the paired lower bound on the whole step is below 2.0 | The staged step plan, which is the design of record |
| EV-F59 Facility co-design and multi-robot handoff | 0.6 | KILL-AUGUR-FACILITY | M8 | The acceptance fixture fails to discriminate two strategies, or misses a seeded failure after one repair cycle | Single-robot missions with priced world edits from §19; multi-robot composition is parked |

Family and kill-gate pairs due at this exit: 8.

**2. Killed-idea records whose reversal condition names an M8 gate.**

| Record | Condition read at M8 | If met |
|---|---|---|
| KI-07 a megakernel at high occupancy | The one-week fusion pilot (kinematics, dynamics terms and one Newton iteration fused for the reference quadruped at 8,192 worlds, digests unchanged): paired lower bound of at least 2.0 on the whole step | The fused kernel is built behind its flag; otherwise the staged step plan stays the design of record. The occupancy arithmetic stands either way: 2 warps of 48 is 4.2%, and 12 of 48 is 25%, not 75% |
| KI-29 a geometric-algebra tree policy | Three arms with three seeds each at equal compute (about 90 GPU-hours in an owner-approved envelope); the ceiling stated first was 1.3 | Multivector features may be used in policies; otherwise the plain morphology transformer remains |
| KI-47 continuation with certified roots | On the CPU: 40 certified roots on ten generic parameter sets of the Stewart–Gough forward kinematics, at most 0.1% lost paths and none unflagged, each root passing α(f, z) < (13 − 3√17)/4 ≈ 0.157671 in the exact rational checker | The continuation lane ships labelled `Complete` only where a trace test agrees, otherwise `Partial`; the GPU and in-kernel forms stay refuted |
| KI-31, KI-65 certificates per stratum; reachable-set tables | Braking across two floor materials, two states and a mode: the table's inclusions check | Tables for subsystems of at most four dimensions; reduced-order barriers elsewhere |

Also decided here, with no killed-idea record of their own: facility co-design and multi-robot handoff (its
acceptance fixture: two strategies discriminated and four seeded failures recovered; on failure single-robot
missions with priced world edits remain and multi-robot composition is parked); skill transport in both forms (through a physical interface: the tray interface with three
pre-registered treatments; by a learned residual: 24 target bodies of one class, three treatments at equal
compute, about 40 GPU-hours), and firmware export (the fixed-point cases, 10⁶ vectors bit-equal; on failure
only parameters, Q formats and the test-vector manifest are exported). The chance-claim coupling rule, which
is read at M4 and again at M8, is re-read.

**3. The Moonshot queue, final.** Every item `MQ-01` to `MQ-21` ends in one of: `passed` (its gate recorded),
`killed` (its kill gate fired; fallback named), `falsified` (its pilot's falsifier held before funding),
`never-funded` (no free slot or no approval of its compute class; the reason recorded), `deferred` (general
Lean theorems on Lie groups, while the proof library's modules are blocked). The sweep prints the queue's
whole history: for each item the exits at which it was eligible, entered, and left.

**4. A final state for every record.**

- *Matrix:* each of the 143 families has a final history entry: `delivered` (with its last gate record),
  `fell-back` (with the fallback's name), `parked` or `rejected` (with whether its reopening evidence ever
  arrived), `never-evaluated` (no gate ever recorded a value; the reason is stated).
- *Killed-idea register:* each of the 67 entries records whether its revival condition was ever evaluated,
  with the measured values, and whether a revival was proposed, accepted or refused.
- *Idea-pass register:* each placed idea is `delivered`, `fell-back` or `not-delivered`; each not-yet-placed
  idea has the owner's disposition or the default (not built).
- *Source register:* the count of readings never re-read, and the gates that would have needed them and were
  therefore recorded `not run`; the final status of the 13 not-verified items.
- *The eight discrepancies* found when the refuting calculations were recomputed (NAB-41): confirmation that
  none of the withdrawn figures reappeared in any contract, report or registry.

**5. The closing printout.** `docs/decisions/` is regenerated: the matrix with plan-time and final scores side
by side; the queue history; the killed-idea register with evaluation results; the idea-pass register with
final carriers. One paragraph at the head states, with counts, how many plan-time judgments were confirmed by
measurement, how many were changed and in which direction, and how many were never tested.

**6. The reader rule, last time.** Any decision registry that no verb or check read during M4 to M8 is reduced
to its generated printout, and its file is marked frozen.

## Acceptance criteria

- [ ] `docs/delivery/exits/M8/decision-records.json` exists and `decision-sweep check --exit M8` passes.
- [ ] Each of the eight family-and-kill-gate pairs of step 1 has an outcome citing its decision ADR.
- [ ] All 21 queue items have a final state from the five allowed, and `active_after` is 0.
- [ ] All 143 families, 67 killed-idea entries, every idea record and all 13 not-verified items have a final
      state; the count of `never-evaluated` families is printed.
- [ ] The closing printout exists under `docs/decisions/`, is current under `check-generated`, and its head
      paragraph's counts equal the counts recomputed from the registries.
- [ ] No withdrawn figure of the discrepancy list occurs in the tree.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m8_families` (seven ids: `EV-F45`, `EV-F47`, `EV-F49`,
`EV-F50`, `EV-F53`, `EV-F57`, `EV-F59`; eight pairs); `final_states_are_total` (a registry row without a final
state fails `sweep.row-not-swept`); `queue_final_states_are_from_the_allowed_set`;
`head_paragraph_counts_recompute`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M8` (NAB-50): due list, `check`, eight pairs, the totals (143, 67, 21,
13) each asserted as fully stated, the closing printout regenerated and compared. Logged per STD-E2E.

### Known answers and negative controls

Known answers: seven families, eight pairs; 21 queue items; totals 143, 67 and 13; (13 − 3√17)/4 = 0.157671;
2/48 = 4.2% and 12/48 = 25%; 3 × 1,442 = 4,326 solutions of the nine-point four-bar problem, found among
286,720 paths. Negative controls: a queue item left `queued`; a family with no final entry; a head paragraph
whose "confirmed" count was typed by hand and differs from the recomputed count; a Moonshot described as
delivered with no gate record.

## Considerations

- "Never evaluated" is an honest final state and is expected for several Moonshots: the cap of four and the
  compute envelope mean some pilots never run. The record says so instead of implying a negative result.
- The closing printout is the document a sceptical reviewer reads first. It is generated, so it cannot be
  improved by editing.

## References

Appendix C.1, C.1.4, C.2, C.3; §0.7; §0.8; the M8 milestone and its exit review.
