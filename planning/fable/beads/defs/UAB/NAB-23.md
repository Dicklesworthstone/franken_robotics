---
key: "NAB-23"
title: "M0 exit: sweep of the decision records (EV matrix re-score, killed-idea conditions, queue, ideas, sources)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-22", "BD-044"]
needs: ["§40.6: the M0 exit report with its kill decisions for KILL-FSIM and KILL-MCP (N40-04, N40-30)", "§1.4: the M0 re-audit of incumbents with execution on host `trj` (N00-14)", "Appendix K.5: the re-read of every published figure the thesis, the audit and the bets quote (N00-34)", "§0.6, §38.5: the plan-time check ledger after the M0 re-runs (N00-06)"]
related: ["NAB-17", "NAB-32", "NAB-42", "NAB-09", "NAB-24", "N00-06", "N00-14", "N00-34", "N00-35", "N40-12", "BD-029", "BD-039", "BD-040"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2", "Appendix H", "§43 (M0)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data: the expected-value matrix (`registries/ev_matrix.toml`, 143
scored families of mechanism), the Moonshot queue, the killed-idea register (67 refuted or deflated ideas, each
with the condition that would revive it), the idea-pass register and the source register. They were written on
paper evidence. At each milestone exit they are re-read against the evidence that exit produced, under one
rule: a verdict changes only on new evidence of the kind that produced it (a pilot, a measurement at a gate, or
a primary source re-read at a pinned version). NAB-22 defines the procedure, the due list and the sweep record;
this bead is the sweep at the exit of **M0** (the spine and the airlock: foundations admitted, one kernel on
three backends, the rival's loop measured).

M0 is the first time any statement of the planning record meets an execution on this project's own machines.
Three kinds of evidence arrive: the foundations are admitted or not (the two kill gates of M0 decide whether
FrankenSim is consumed by path and whether the MCP server joins the core workspace); the incumbents are
installed and measured on host `trj`, so documented figures become reproduced or contradicted; and the
plan-time pilots are re-run from the reproduction archive. Little of the matrix moves at M0, but much of the
source register does, and that is the point of sweeping here.

## Scope

In: the M0 sweep record `docs/delivery/exits/M0/decision-records.json` and its generated report; history
entries on the rows that were due; proposals where a revival or reopening condition was met. Out: deciding any
gate or kill gate; the exit review (BD-044); the procedure and tool (NAB-22).

## Technical approach

Run `cargo run -p frob-conformance --bin decision-sweep -- skeleton --exit M0`, fill each due item, then
`… check --exit M0`. The session that performs the sweep did not implement the M0 beads whose gates it cites.

**1. Families due by a kill gate decided at M0.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-S29 MCP surface on `fastmcp_rust`, generated from the verb registry, in the core graph with verbs run in process | 8.0 | KILL-MCP | M0; re-read at M1 with the M1 verbs | INT-FMCP-01 cannot pass within the M0 budget after FD-FMCP-01 and FD-ASUP-01 have been attempted | Fallback 1: `frob-mcp` as its own workspace, talking to `frobd` over a local socket in newline-delimited JSON. Fallback 2: agents use the CLI, which is complete on its own (§33.13, §40.7) |

Family and kill-gate pairs due at this exit: 1.

KILL-FSIM (trigger: the path-dependency admission test INT-FSIM-01 cannot pass within two weeks of M0 effort,
or its cold check exceeds the machine envelope, or upstream breakage exceeds 20% of working days) has no single
family row; it governs the *posture* toward the foundations. Its outcome is recorded against the posture record
of the killed-idea register (`KI-63`, the row on copying the foundations wholesale): consume by path behind the
airlock; a worker process as the first fallback; a minimal lifted referee set last.

**2. Families due because their first gates were recorded at M0** (the tool lists them from the gate
register; expected at plan time): the Lie-typed core; exact-integer and guarded single-precision kernel
classes (the boundary screen of 65,536 rows on three GPUs is re-run here); derived kernels with dimension
typing (one kernel source on `batch-cpu`, CUDA and Metal); instance certificates; the offline auditor and the
claim product. Each is recorded `unchanged` with the gate records looked at, or changed with its reason.
Confidence in derived kernels is **not** raised on the M0 spike alone: its kill gate (KILL-KIR) is decided at
M2.

**3. Killed-idea records whose condition can be evaluated at M0.**

| Record | What M0 measures | What the sweep records |
|---|---|---|
| KI-01 half precision | The conditioning check re-run on this machine (κ·u₁₆ on sampled contact states of three models) | Whether κ·u₁₆ ≥ 1 on a material share of states is reproduced; the idea stays refused either way unless κ·u₁₆ < 1 everywhere and 9k/n < 1 |
| KI-13 zero-copy reads; KI-67 64-bit device atomics | The hardware fingerprint: storage-mode behaviour; presence of 64-bit integer atomics per backend | Confirmed or contradicted, with the fingerprint id |
| KI-44, KI-56, KI-34 | The plan-time pilots re-run: the fixed-n enumeration (2,145 pairs; 521, 521, 1,103, 0), the solver pilot (1.345 warm, 1.088 cold, residual 575.7 µs, ceiling 842.8 ÷ 575.7 ≈ 1.46), the decision-reuse tie | `reproduced` or `differs` per check; a `differs` reopens the record's refutation for review |
| KI-59, KI-62 | The parity baseline: incumbent phases on `trj` for cold start, physics and full step; time to threshold on one and two cards | The measured step share s and its ceiling 1/(1 − s) beside the published 87% and 7.7; the measured training time beside 410 s |
| KI-61 withdrawn differentiators | The M0 re-audit with execution | Per withdrawn claim: documentation reproduced, or contradicted (then the claims registry, which is append-only, records the change) |
| KI-63 posture and process | KILL-FSIM; the state of the two upstream work items that align the async runtime and bring the MCP library onto the one toolchain | The posture in force after M0 |

**4. Moonshot queue.** No item is eligible (the first, the fused per-world step kernel, waits for the M2
profile). The sweep records `active_after = 0`.

**5. Idea register.** Carriers whose first gate is at M0 are marked `in progress`: single-source kernels on
three backends, the one verb registry, the negative-control registry.

**6. Source register.** (a) Every reading task attached to a gate recorded at M0 must be closed: the incumbent
figures behind the parity baseline are re-read from primary sources (39 of 163 public readings were primary at
plan time). (b) Local sources are re-read at the stack-lock commits by the fact-sheet script; paths that moved
are listed. (c) `[[unverified]]` items settled at M0: NV-01 (incumbent throughput on an RTX 4090, by
GATE-SPARK-21), NV-02 to NV-04 (hardware properties, by the fingerprint), NV-10 (the unexecuted parts of the
reported foundation defects, by the admission tests). (d) The reproduction archive's revision is matched and
recorded.

**7. Glossary.** The first `CONTRACT.md` files exist; the lint's unused-term and allow-file reports are read,
and terms the contracts introduced are added.

## Acceptance criteria

- [ ] `docs/delivery/exits/M0/decision-records.json` exists, validates against `frob.decision-sweep/1`, and
      `decision-sweep check --exit M0` passes.
- [ ] The MCP-surface family (EV-S29) has a history entry for M0 citing the KILL-MCP decision ADR; if the kill
      gate fired, its decision text names the fallback (a separate workspace over a local socket, then the
      CLI).
- [ ] Each row of the table of step 3 has a recorded evaluation with measured values, and no revival is applied
      without an accepted ADR.
- [ ] No reading task is open for a gate recorded at M0; NV-01 to NV-04 and NV-10 are `measured` or carried
      forward with a reason.
- [ ] The sweep's session is not an implementer session of a bead whose gate it cites.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m0_families` (exactly `EV-S29` by kill gate);
`m0_requires_kill_fsim_posture_entry` (a sweep record without an evaluation of KI-63 fails
`sweep.row-not-swept`); `m0_reading_tasks_closed`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M0` (NAB-50) prints the due list, runs `check`, and asserts the counts
of step 1 (one family-and-kill-gate pair) and step 6. Logged per STD-E2E: due items, outcomes by kind, open
reading tasks (threshold `== 0`), the archive revision.

### Known answers and negative controls

Known answers: one kill-gate pair due; arithmetic the evaluations must reproduce: 1/(1 − 0.87) = 7.69;
842.8 ÷ 575.7 = 1.464; 521 + 521 + 1,103 = 2,145. Negative controls: a sweep that raises the confidence of
derived kernels citing only the M0 spike gate when its kill gate is still open (allowed only with the reason
stated; the planted case has an empty reason and must fail `sweep.change-without-evidence`); a sweep record
that marks NV-01 `measured` with no gate record.

## Considerations

- If KILL-FSIM fires, a large part of the matrix changes character (the referee is reached through a worker
  process). The sweep does not re-score every family that touches the referee; it records the posture and lets
  the M1 sweep re-score with M1 evidence.
- Incumbent figures that are contradicted on `trj` are the most consequential outcome possible here: parity
  targets and ten-fold denominators move with them. The sweep lists every decision record that quotes a
  contradicted figure.

## References

Appendix C.1, C.2; Appendix H; §0.8; the M0 milestone and its exit review.
