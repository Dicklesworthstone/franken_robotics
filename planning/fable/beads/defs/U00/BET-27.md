---
key: "BET-27"
title: "Decide BET-27: a sampling controller as fallback and demonstration source, never as a stand-in evaluator"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "reflex", "cortex", "frob-control", "frontier", "bet", "gate"]
parent: "WP-27"
deps: ["N00-28", "N00-07", "BD-252", "BD-323", "BD-331", "STD-E2E"]
needs: ["§22.22: the delivered-controller lane with its frozen planner record (GATE-REFLEX-11; refusal FROB-REF-817 for an acceptance request with no frozen planner record)"]
related: ["BET-06", "BET-17", "BD-249", "N00-11", "N00-CX", "N00-23"]
gates: ["GATE-REFLEX-06", "GATE-CORTEX-14", "KILL-MPC", "KILL-DISTILL"]
plan_refs: ["§4 (BET-27)", "§40.5", "§0.8 (H-01)", "§2.5", "§22.18", "§23.23"]
---
## Background

**The bet.** A sampling-based predictive controller plans by simulating many candidate action sequences from
the current state and executing the best; it needs no training. The design gives it three limited roles: a
screening rung, a fallback for designs the synthesized baseline cannot control (those reported
`not-synthesizable`), and a source of demonstrations from which a policy can be distilled. The bet is that in
those roles, with costs set per morphology class and not per robot, it is good enough and cheap enough. Tier:
Frontier.

**What it must never be.** A stand-in evaluator for a trained policy. Even the most sample-efficient published
controller costs about 4 × 10³ times a policy rollout per simulated second: a battery of 256 twenty-second
episodes takes about 86 minutes of a device against about 1.3 s for a policy. Published settings cost about
0.3–1 GPU-minute per quadruped episode and 1–5 per humanoid episode; the published successes are legged robots
with hand-built swing or stepping references, with no success rate over a varied corpus and nothing for tracked
robots. Its own episodes are acceptance evidence only where it is itself the delivered controller, under a
lane with a frozen planner record.

## Scope

In: pre-registration, the M3 and M4 readings, verdict and decision record. Out: the controller, distillation,
the delivered-controller lane.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. On designs marked `not-synthesizable`, fewer than half traverse the reference task under it.
   b. A 20 s episode costs more than 1 GPU-minute for a quadruped or 6 for a humanoid at the median.
   c. Distillation from its demonstrations does not reduce training steps.
2. **Competing arm.** Training from scratch. **Potency controls.** A shuffled-label distillation control that
   must fail; a planted acceptance request citing sampling-controller episodes with no frozen planner record,
   which must be refused.
3. **Deciding gates (TARGETs).**
   - GATE-REFLEX-06 (Design comparison, M4): on the pilot design set: success at least 60% within three
     episodes; median at most 1 GPU-minute per 20 s quadruped episode and 6 per humanoid episode; Kendall τ
     against specialists at least 0.5; the refusal fires on the planted acceptance request; the digest does not
     change with the fork count.
   - GATE-CORTEX-14 (Design comparison, M4): the shuffled-label control fails; a student of a baseline or
     specialist teacher reaches at least 90% of the teacher's requirement pass rate; the figure for a
     sampling-controller teacher is recorded with its cost.
4. **Kill gates and fallbacks.** KILL-MPC (M4): GATE-REFLEX-06 fails after one repair cycle; the sampling
   controller is kept as a single-world CPU debugging aid, and demonstrations come from the baseline and from
   specialists. KILL-DISTILL (M4): students stay below 70% of their teachers' requirement pass rate after one
   repair cycle; distillation is limited to the privileged-teacher case and warm starts come from behaviour
   cloning only. The bet's own fallback: designs the baseline cannot control route to training.
5. **Downstream if falsified.** `not-synthesizable` designs get no closed-loop feedback before training, so
   "every design stands on day one" holds only for classes the baseline covers; the screening rung disappears
   from the ladder for those designs; nothing changes for acceptance, which never used it.
6. **Procedure.** Register the pilot design set, the cost accounting (GPU-seconds per simulated 20 s episode at
   a stated planning step and sample count), class-default costs frozen per class, and the distillation
   comparison before any run; read at M3 (fallback availability) and M4 exits; `docs/adr/bet-27-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with class-default costs frozen; no per-robot cost is set in any counted run (manifest
      check).
- [ ] The traverse share on `not-synthesizable` designs against one half; median GPU-minutes per episode
      against 1 and 6; τ against 0.5.
- [ ] Distillation's step reduction against the from-scratch arm; the shuffled-label control failed.
- [ ] The planted acceptance request was refused; verdict and both kill decisions recorded as ADRs.

## Test plan

### Unit tests
`bets.rs`: `bet27_traverse_0_58_cost_0_7_and_4_holds`; `bet27_traverse_0_44_falsifies_a`;
`bet27_quadruped_cost_1_3_gpu_min_falsifies_b`; `bet27_distillation_no_step_reduction_falsifies_c`;
`bet27_acceptance_without_planner_record_refused_817`; `bet27_student_at_0_66_fires_kill_distill`.

### End-to-end script and logging
`scripts/e2e/bet_27_decide.sh` runs three episodes of `frob control mpc` on one `not-synthesizable` fixture
design, records GPU-seconds per episode, submits the planted acceptance request (expects FROB-REF-817),
recomputes the statistics from the ledger and the verdict. Logged per STD-E2E: design, class, episodes,
traverse outcome, GPU-seconds per 20 s episode against the thresholds, fork count and digest, distillation step
counts; `summary` naming GATE-REFLEX-06, GATE-CORTEX-14, KILL-MPC and KILL-DISTILL.

### Known answers and negative controls
Break-even context (as recorded in the hypothesis register): a sampling controller is cheaper than one
training run only up to about 30 episodes per quadruped design and 60 per humanoid design at a 20 ms planning
step (6 and 12 at 4 ms); at exactly 1 GPU-minute per episode the quadruped figure is 410 s ÷ 60 s ≈ 6.8
episodes. CX-009. Negative controls: the shuffled-label student; the planted acceptance request; a run with a
per-robot cost weight.

## Considerations

- Cost depends strongly on planning step and sample count; both are in the disclosure of every figure.
- The fork count must not change the digest, or the controller's episodes cannot be replayed.

## References

§4 (BET-27), §40.5, §0.8 (H-01), §2.5, §22.18, §23.23; GATE-REFLEX-06; GATE-CORTEX-14; KILL-MPC;
KILL-DISTILL; CX-009.
