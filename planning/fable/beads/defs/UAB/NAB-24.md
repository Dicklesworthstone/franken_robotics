---
key: "NAB-24"
title: "M1 exit: sweep of the decision records after the referee studio and the first mission slice on the CPU"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "delivery", "gauntlet", "solid", "exit-review", "decision-record"]
parent: "EPIC-M1"
deps: ["BD-087", "BD-131", "BD-132", "BD-145", "N14a-16", "N14b-11", "N15a-06", "N16a-07", "N18-REF", "N22-REF", "N24b-06", "N25-REF", "N25b-04", "N40-04", "N40-11", "N40-13", "N40-30", "NAB-22", "NAB-23", "NAB-50", "STD-E2E"]
needs: []
related: ["BD-147", "NAB-32", "NAB-42"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2", "§43 (M1)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register), and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the
sweep at the exit of **M1**: the referee studio and the first mission slice on the CPU, a useful and honest
product with exact answers and no GPU.

M1 is where the Solid spine first runs: the design language and its compiler, actuators with thermal and
voltage limits, the transactional referee step, the energy ledger, the static analyses of `design check`, the
baseline controllers, the requirement language, the two study types, and the first mission queries with
certificates checked offline. It is also where the incumbent's *design loop* (one agent-driven round, timed by
phase) is measured, which gives the first measured shares behind the ten-fold thesis.

## Scope

In: the M1 sweep record `docs/delivery/exits/M1/decision-records.json` and report; history entries on due rows;
proposals. Out: deciding gates (the decision beads this bead depends on: N14a-16 for KILL-SKEL-SUBST, N25b-04 for
the standing KILL-AUGUR-FALSECERT; the M1 re-read of KILL-MCP is N32a-05's record `ADR-KILL-MCP-M1`); the exit
review (BD-147), which comes after this sweep, runs its `check` as an attached check and copies its section into
the exit record; the procedure (NAB-22).

## Technical approach

The sweep is made at the M1 exit candidate commit, before the exit record (NAB-22 design decision 1). It reads
the M1 gate records through the gate report of N40-04 at that commit, the kill-gate ADRs of N14a-16 and N25b-04,
the schedule gate (N40-11) and the incumbent design-loop ledger (N40-13), and the beads behind the killed-idea
evaluations of step 3 (N24b-06 the pre-analysis verb, BD-131 the pusher battery, N16a-07 coupled impulses,
BD-087 the free-body variational oracle, N14b-11 the growth vector at preflight, N15a-06 the thermal node's
second-law test, BD-132 the certificate checkers) and the rank-agreement pre-registration (BD-145); it needs no
exit verdict. `decision-sweep skeleton --exit M1`, fill, `check --exit M1`, by a session that implemented none
of the cited beads; the exit runner of BD-147 runs the same `check` as the attached check `decision-sweep`.

**1. Families due by a kill gate decided at or standing from M1.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-F04 Composition over typed ports (the wiring diagram of version 0.2) as a thin data structure | 6.0 | KILL-SKEL-SUBST | M1 | A reused product differs from its recompiled value after one repair cycle, or leaf-local reuse saves less than 20% of compile time on the archetypes | Every substitution is global; the memo graph keeps subterm reuse by content hash only |
| EV-F19 Family certificates in the `.frobcert` bundle | 4.0 | KILL-AUGUR-FALSECERT | Standing, from M1 | One certificate accepted by `frob-verify` is shown false within its scope | The procedure of §25.5.5; if the cause is the rule and not an implementation defect, the proof kind is removed and its cells become `Tested` |

Family and kill-gate pairs due at this exit: 2.

KILL-MCP is re-read at M1 with the M1 verbs: the MCP-surface family (EV-S29) is swept again, and its entry
cites the M1 record. KILL-AUGUR-FALSECERT is a standing gate from M1 onward: one certificate accepted by the
offline auditor and shown false within its scope withdraws family certificates until the checker is repaired;
every later sweep re-reads it.

**2. Families due because their gates were first recorded at M1** (computed by the tool; expected at plan
time): inertia on the positive-definite cone; the Euler–Poincaré framing; group-preserving Cayley updates; the
symmetric-cone abstraction and the one cone solver; invariant filtering; static analyses in `design check`
(the latency target of under 2 s at the 95th percentile is measured here; the kill gate that could downgrade
verdicts to advisory lints is decided at M3); the clock contract on the referee; the transactional,
event-located referee step; the energy ledger on the referee; port-Hamiltonian actuators with a thermal node;
the requirement language core; the two declared study types; e-processes on the CPU; mission queries with inner
and outer design sets (first slice); the crash-consistent job protocol; the growth vector; the canonical form.

**3. Killed-idea records whose condition can be evaluated at M1.**

| Record | What M1 measures | What the sweep records |
|---|---|---|
| KI-44 a fixed-n rule with an unreachable verdict | The pre-analysis verb on the CPU: reachable verdicts at the declared n | The enumeration at n = 64 (no practical tie) and n = 86 (first tie) reproduced by the shipped code |
| KI-56 a new family query as a capability | The pusher battery on one plant, per-build and shared queries, against the generic comparators | Whether the specialized method ties the composed one (the tie rule then ships the generic method behind the same verbs); the decision itself is at M3 |
| KI-20 refusing simultaneous impacts; KI-19 symplectic claims | Coupled impulses per island in the transactional step; the free-body variational oracle | Known answers passed; no symplectic claim made for a tree |
| KI-21 curvature as a roller-misalignment test | The growth-vector analysis at preflight: the parallel-roller mecanum base has growth vector (2, 3), not (3) | The negative control is caught by the growth vector and passed by the curvature test, as the record says |
| KI-25 entropy as the state of a thermal node | The per-step second-law test of the thermal node | Stored heat is a store, not a loss; no double counting |
| KI-51 "proved once at M1" | Instance certificates checked in exact arithmetic; no general theorem claimed | The four evidence classes on outputs; the word "verified" absent from results |
| KI-59 engine alone, information alone | The incumbent design-loop phase ledger: measured shares of training, agent turns, authoring, diagnosis | The measured shares beside the illustrative ones (training about 96% where every candidate is trained; agent turns about two thirds once acquisition is amortized), and which regime the measured round is in |
| KI-63 posture and process | The schedule gate: M0 plus M1 against its planning estimate, with the cause of any slip attributed | The cause attribution, and whether upstream breakage contributed |

**4. Moonshot queue.** No item is eligible. `active_after = 0`.

**5. Idea register.** Three of the five best first-pass ideas ship at M1 and are marked `in-progress` with
their M1 gate records cited (`delivered` only once every named gate has a recorded pass, NAB-42
`idea.state-ahead-of-gate`): check before you simulate; a baseline controller for every design (first archetypes); the energy
ledger (on the referee). The rank-agreement experiment is pre-registered at M1 (design set, strata, thresholds
0.7 and 0.5 fixed before data); the sweep records that the pre-registration exists and predates any data.

**6. Source register.** Reading tasks for gates recorded at M1 are closed (the incumbent loop figures: about
an hour of reinforcement learning per round of 16 candidates on eight GPUs; 7.5 to 30 GPU-minutes per
candidate). Standard references used by M1 known answers (rigid-body dynamics, invariant filtering, the
Clopper–Pearson interval, Sterbenz's lemma) are confirmed against their primary sources and gain `reread`
entries.

**7. Glossary.** Unused-term and allow-file reports read; the M1 contracts' new terms added.

## Acceptance criteria

- [ ] `docs/delivery/exits/M1/decision-records.json` exists and `decision-sweep check --exit M1` passes.
- [ ] EV-F04 (composition over typed ports) has an entry citing the KILL-SKEL-SUBST decision; EV-F19 (family
      certificates) has an entry recording the state of the standing gate; EV-S29 is re-read.
- [ ] Each row of the table of step 3 has a recorded evaluation with measured values.
- [ ] The ceilings of the ten-fold thesis are recomputed from the measured shares and stored with the sweep
      (for a measured step share s the ceiling of a faster step is 1/(1 − s)).
- [ ] No reading task is open for a gate recorded at M1.
- [ ] No cross-reference check of the decision records still reports a recorded `skip` for a row that lands at
      M1: every kill gate named by the matrix, the Moonshot queue, the killed-idea register and the idea
      register resolves in the gate register with the deciding milestone of the frozen list, the six quoted
      triggers of the contested records equal the register's, and `FROB-REF-245`, `FROB-REF-817`, `FROB-REF-856`
      and `FROB-REF-877` exist in the refusal registry.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m1_families` (`EV-F04`, `EV-F19` by kill gate, and
`EV-S29` by the re-read rule); `standing_gate_is_swept_at_every_later_exit` (KILL-AUGUR-FALSECERT appears in the
due lists of M1 through M8).

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M1` (NAB-50): prints the due list, runs `decision-sweep check --exit M1`,
and asserts the two kill-gate pairs, the EV-S29 re-read, the evaluations of step 3, the recomputed ceilings and
zero open reading tasks; then replays the two negative controls below (fixtures under
`tests/fixtures/sweep/M1/controls/` written by this bead, the hook of NAB-50) on scratch copies of the sweep record and
asserts that `check` fails on each with `sweep.change-without-evidence`. Logged through the shared helper of
STD-E2E to `target/e2e-logs/decision_records/<utc>.ndjson` with a transcript, every line with `ts`, `script`,
`bead` (`NAB-24`), `step`, `event`: `cmd` lines with the exact command line, exit code and wall time; one
`assert` per pair with the kill gate, the ADR identifier and the outcome; one `metric` per measured phase share
with its ceiling 1/(1 − s); open reading tasks as a `metric` (threshold `== 0`); the digests of the gate report
and of the sweep record; `assert` lines of kind `negative-control` with `caught`; on failure the first due item
without an outcome, with the path of the sweep record; a `summary` with assertions passed and failed and the
kill gates read.

### Known answers and negative controls

Known answers: two kill-gate pairs; 8 × 60 ÷ 16 = 30 GPU-minutes per candidate (the top of the published
range); with one design per agent turn and 300 s of training the loop ratio is at most (300 + t)/t, which is
3.5 at t = 120 s and 6 at t = 60 s. Negative controls: a sweep that records KI-56's condition as evaluated at
M1 (`condition_met` set, citing the partial-gate record, which does not exist until M3: the evidence does not
resolve and the check fails `sweep.change-without-evidence`; an early entry must not silence the M3 due item);
a history entry that cites the schedule gate as evidence for raising an Impact (fails
`sweep.change-without-evidence` by the evidence-kind rule of NAB-22 rule 1.2).

## Considerations

- If the schedule gate shows that M0 plus M1 overran because of the foundations, Effort on the families that
  consume them is the factor with evidence; the sweep changes Effort, not Confidence.
- The measured phase ledger of the incumbent loop is the first evidence about Impact for every family whose
  claimed effect is on agent turns or on avoided training. The sweep records the shares; it does not yet
  re-score Impact, because our own arm has not run (that evidence arrives at M3 and M7).

## References

Appendix C.1, C.2; §0.8; the M1 milestone and its exit review; the schedule gate.
