---
key: "NAB-28"
title: "M5 exit: sweep of the decision records after proofs, reality and manufacture report (off the critical path)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M5"
deps: ["NAB-22", "NAB-27", "BD-412"]
needs: ["§40.6: the M5 exit report with its kill decisions (N40-04, N40-30)", "§30.2: the Reality lane's first record or its recorded `no-claim` (BD-397)", "§12.4: the reflection pilot's measurements and the kill decision on kernel reflection (BD-382, N12-32)"]
related: ["NAB-17", "NAB-14", "NAB-21", "NAB-32", "NAB-29", "N12-32", "BD-380", "BD-382", "BD-397", "BD-399"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2", "Appendix H.6", "§43 (M5)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the sweep
at the exit of **M5**: proofs, reality and manufacture, which is everything about what can be trusted and what
can be built, and all of it off the critical path.

Three kinds of claim are tested at M5 that no earlier milestone could test. *Proof*: certificate coverage of
derived kernels, the proof checker admitted on a Linux host, Lean theorem cards as sealed capsules, and the
reflection pilot (the one route from an instance certificate to evidence class Proved). *Reality*: the import
lane for measured data, which rests at `no-claim` until real data is admitted, and whose procedure is rehearsed
on synthetic data that is never promoted. *Manufacture*: load cases from telemetry, sizing, topology
optimization, tolerance allocation and the build packet, adopted for output quality and explicitly not counted
in the ten-fold thesis.

## Scope

In: the M5 sweep record and report; history entries; proposals; the move of kernel reflection if its kill gate
fired. Out: deciding gates; the exit review (BD-412); the procedure (NAB-22).

## Technical approach

`decision-sweep skeleton --exit M5`, fill, `check --exit M5`, by a session that implemented none of the cited
beads.

**1. Families due by a kill gate decided at M5.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-F27 Instance certificates | 3.0 | KILL-LEMMA | M5 | GATE-LEMMA-01 below 50% after one repair cycle | Certificates become exact-rational randomized identity tests recorded as `Tested`; the word certificate leaves kernel manifests |
| EV-F32 Calibration against reality, and conformal bands per calibrated cell | 3.0 | KILL-BAND | M5 | GATE-GROUND-06 or -07 fails | Bands are withdrawn; reports print `reality: no-claim` with the raw residual table |
| EV-F32 Calibration against reality, and conformal bands per calibrated cell | 3.0 | KILL-GROUND | M5 | GATE-GROUND-04 fails after one repair cycle | No identification; Ground keeps admission, clocks, frozen predictions and bands on direct measurements with prior cards |
| EV-F37 Trapezoidal variational step (the variational articulated integrator of version 0.2) | 2.3 | KILL-VARIATIONAL | M5 | The implicit solve costs more than twice a semi-implicit step on `batch-cpu`, or the articulated form fails its momentum row after one repair cycle | The free-body step is kept as a known-answer oracle; every rollout uses the default integrator |
| EV-F38 Topology optimization, fatigue, tolerance allocation and the build packet | 2.3 | KILL-LOADS | M5 | GATE-FOUNDRY-02 fails after one repair cycle | Load cases are declared by hand from statics with a stated dynamic factor; spectra and fatigue read `no-claim` |
| EV-F38 Topology optimization, fatigue, tolerance allocation and the build packet | 2.3 | KILL-PACKET | M5 | GATE-FOUNDRY-09 or -10 fails after one repair cycle | Parts list, mass properties and STL only; STEP and harness deferred |
| EV-F38 Topology optimization, fatigue, tolerance allocation and the build packet | 2.3 | KILL-TOLERANCE | M5 | GATE-FOUNDRY-08 fails, or band-extreme checks fail on more than a fifth of features in the pilot | Worst-case boxes per interface |
| EV-F38 Topology optimization, fatigue, tolerance allocation and the build packet | 2.3 | KILL-TOPOPT | M5 | GATE-FOUNDRY-05 fails: saving under 7%, no convergence, or over budget | Beam-rung sizing supplies mass and inertia feedback; three-dimensional optimization becomes an offline option or is dropped; A-15 is unaffected |
| EV-F39 Hard-contact batch law CL-H | 2.3 | KILL-HARDCONTACT | M5 | GATE-TRAC-09 fails after one repair cycle | Fixed-iteration island solvers on `batch-cpu` with a residual flag; CL-H remains the referee's specification and a model-form arm under `strict` |
| EV-F44 Tree-operator route for mechanisms above 64 degrees of freedom | 1.5 | KILL-TREEOP | M5 | Whole-step gain below 1.15 at 64 degrees of freedom on the M2 profile | The dense factorization everywhere and a cap of 64 degrees of freedom |
| EV-F48 Kernel reflection | 1.0 | KILL-REFLECT | M5 | GATE-LEMMA-05 fails after one repair cycle, or no reflector's closure avoids the blocked modules | Instance certificates stay `Certified`; reflection is parked in Appendix C |

Family and kill-gate pairs due at this exit: 11.

The rule of the matrix for kernel reflection applies here: it is tiered Frontier, is not counted against the
Moonshot cap, and **if KILL-REFLECT fires it moves to the parked table**. The sweep performs the move as a
history entry `move = "frontier-moonshot → parked"` citing the decision ADR; the row keeps its id and gains
`reopen_on` ("a reflector whose closure avoids the blocked library modules").

**2. Killed-idea records whose reversal condition names an M5 gate.**

| Record | Condition read at M5 | If met |
|---|---|---|
| KI-51 "proved once at M1" | The proof tool has the tactics and the unblocked library modules the obligations need; the reflection pilot passes | General theorems may be scheduled; otherwise four evidence classes with instance certificates remain, and the word "verified" stays unused for results |
| KI-19 symplectic claims beyond a free rigid body | The articulated variational step: the implicit solve costs at most twice a semi-implicit step on `batch-cpu`, and the articulated form passes its momentum row | The step stays in the referee and the `strict` profile, with no symplectic claim for a tree |
| KI-07 a megakernel with a lossless limb summary | The limb port interface's proof obligation (the port inertia is positive semidefinite, exact only without base-coupled contact) and the tree-operator pilot (whole-step gain of at least 1.15 at 64 degrees of freedom on the M2 profile) | Limb port packets stay as an intermediate tier |
| KI-57 conformal reality bands for designs never built | Bands per calibrated cell reach their coverage gates on held-out data of the same cell; on the synthetic rehearsal only, unless real data was admitted | Bands per cell; a refusal elsewhere; nothing for an unbuilt design |
| KI-58 generative structure as a decision-time lever | Topology optimization saves at least 7% of link mass, converges and stays in budget; the phase ledger shows whether structure shortened any decision | Foundry stays an output-quality subsystem outside the thesis |
| KI-63 posture and process (gates that need hardware a simulator lacks; scripted humans as dressing) | The Reality lane's record: `no-claim`, or a first calibrated cell from admitted data; the scene-dressing kill gate (a dressing item that breaks the fixed launch shape or the determinism class, or adds more than 25% to step cost, is removed) | Import lanes stay import lanes |

**3. Source register and the not-verified list.** This exit settles, or explicitly carries forward: NV-12 (any
Lean result: after the checker's admission and the first sealed capsules), NV-13 (any statement about a
physical robot: stays `unverified` unless real data was admitted), NV-07 (asset licences: asset-populated
interiors may not ship an asset with an unverified licence), NV-08 (glTF physics extensions), NV-09 (hazard
dimensions and retrofit prices stay `assumed` with provenance). Reading tasks for M5 gates are closed: the
published topology-optimization savings (7 to 30% lighter links; 2 to 8% of robot mass on the one quadruped
with a published mass breakdown) and the conformal-coverage references.

**4. Moonshot queue.** No item's earliest milestone is M5. Active items (if any pilot was funded early) are
re-read; `next_entries` is recorded.

**5. Idea register.** Rows whose carriers' gates are recorded at M5: build packet; link topology optimization
closed into the loop; actuator identification from logs; scan or video to a simulation twin; kernel
reflection; correlated tolerance stacks; calibration against reality.

## Acceptance criteria

- [ ] `docs/delivery/exits/M5/decision-records.json` exists and `decision-sweep check --exit M5` passes.
- [ ] Each of the 11 family-and-kill-gate pairs of step 1 has an outcome citing its decision ADR.
- [ ] If KILL-REFLECT fired, kernel reflection's row is in the parked table with a move entry and `reopen_on`.
- [ ] Each row of step 2 has measured values and `condition_met`; any entry that rests on synthetic data says
      `synthetic` and promotes nothing.
- [ ] NV-07, NV-08, NV-09, NV-12 and NV-13 each have a status and a record or a carry-forward reason.
- [ ] No output, report template or registry describes a result as "verified" or a design as "deployable"
      (the wording lint's findings are attached).

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m5_families` (seven ids: `EV-F27`, `EV-F32`, `EV-F37`,
`EV-F38`, `EV-F39`, `EV-F44`, `EV-F48`; 11 pairs, four of them on the structure row);
`reflection_kill_moves_row_to_parked`; `synthetic_evidence_cannot_promote` (a history entry whose only evidence
is a `synthetic` dataset may not raise Confidence of the reality family).

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M5` (NAB-50): due list, `check`, 11 pairs, the six conditions of step
2. Logged per STD-E2E.

### Known answers and negative controls

Known answers: seven families, 11 pairs; split-conformal coverage at n = 20 and 90% nominal is distributed as
Beta(19, 2), with a standard deviation of 6.3 points, and at n = 200 as Beta(181, 20), with 2.1 points, so a
coverage gate read at n = 20 has that much sampling spread. Negative controls: a reality family whose
Confidence was raised on the synthetic rehearsal; kernel reflection left in the Frontier table after its kill
gate fired; an NV item marked `measured` with no record.

## Considerations

- Nothing swept here is on the critical path, so the temptation is to skip it. The M7 tournament's honesty
  depends on these records being right: an arm may not cite a certificate class or a reality band that the
  sweep shows was never earned.
- A `no-claim` Reality lane is a correct outcome. The sweep records it as an outcome, not as a gap.

## References

Appendix C.1, C.2 (entries on proofs, reality and structure); Appendix H.6; the M5 milestone and its exit
review.
