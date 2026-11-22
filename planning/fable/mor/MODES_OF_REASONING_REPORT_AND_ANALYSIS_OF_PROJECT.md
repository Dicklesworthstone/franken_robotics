# FrankenRobotics — Audited Multi-Mode Analysis

**Primary question:** Which changes to plan v0.2 would let FrankenRobotics deliver a tenfold improvement over the strongest robot simulation and design pipelines of late 2026, and which tempting changes should be rejected?
**Scope:** architecture, mathematics, algorithms, GPU use, scenes, agent studio, verification; not staffing, business model or hardware robots.
**Decision owner and horizon:** the project owner; milestones M0 to M6. Nothing is built yet.
**Evidence cutoff:** 2026-11-21. **Run ID:** `f1`. **Ledger:** `MOR_CLAIMS_AUDITED.jsonl`.

## Bottom Line

The plan cannot yet show a tenfold gain, and the audited evidence says simulator speed will not supply one.

- **No test exists.** The plan has no end-to-end metric and no incumbent arm. Its only agent-level gate compares the product with a cut-down version of itself (C072, C077).
- **Speed alone is capped.** An infinitely fast environment step speeds training by at most 7.8× on the measured humanoid task, and on a late-2026 stack physics is 29% of the environment step (C003). An incumbent already measures 2.55 M humanoid steps/s (C064).
- **Quadrupeds fall short on the plan's own numbers.** Its north-star cycle is 3.0–3.5× an incumbent train-and-evaluate cycle, and one design per agent turn caps at 3.5–6× (C014, C078).
- **Humanoids reach 11–16× on one condition:** that evaluation under the synthesized baseline controller can replace evaluation under a trained policy. No gate tests this (C006, C014, C073).
- **Most claimed novelties are prior art.** Ten of sixteen claimed capabilities are documented, in whole or in part, in an incumbent (C017).

**Recommended posture: EXPERIMENT_FIRST.** Define the metric with an incumbent arm, measure the incumbent's phases at M0–M1, and test rank agreement between the baseline-controller and trained-policy rungs before committing to mechanisms. For quadrupeds the plan must name the extra factor it is betting on: designs decided per turn, fewer turns, or cheaper training (C078).

**Reversal conditions.** The owner defines tenfold as capability, not comparison (C077); or the environment step is measured above 90% of training time on contact-rich tasks (C003).

**Confidence: Moderate.** Sources were re-read at pinned versions and the arithmetic redone. Phase times are composites of published figures from different stacks, and nothing was run on an incumbent.

## Verified Material Findings

Status is V (VERIFIED) or P (PARTIALLY_SUPPORTED); each statement is the audited, corrected one.

1. **No incumbent arm** (C072 V, C077 P). The owner's wording is comparative; the plan's rules already permit an out-of-process incumbent harness. *Effect:* add a bet, a benchmark row and the harness.
2. **Amdahl bound on step rate** (C003 V). At most 7.8×, 6.7× and 1.9× on three Isaac Lab tasks; at most 1.4× from physics alone on mjlab. *Effect:* do not argue tenfold from step rate.
3. **No single phase gives tenfold for quadrupeds** (C006 P). A composite iteration is 5.9 min, training 85% of it; deleting training and compile gives 6.6×. For humanoids deleting training gives about 11×.
4. **Compute ratio** (C015 P). Tenfold on compute needs at most one simulated design in 300 to go on to training, against a one-card incumbent at about 410 GPU-s.
5. **Targets and denominators are off** (C068, C069, C071 V; C070 P). Units are undefined and inconsistent; the training target is about 2.2× slower than the published incumbent on the same card; the CPU batch target is 7–11× below published CPU MuJoCo per core; "six hours" overstates the incumbent by 6–70×. *Effect:* re-base §28.1 and the narrative.
6. **Novelties to withdraw** (C018, C019, C031, C032, C034 V). Incumbents document opt-in bitwise GPU determinism, mixed topologies in one batch, electrical and thermal actuators, zero-training sampling control, and agent interfaces.
7. **Still claimable, each behind a measurement** (C020, C022, C023, C027, C030 P). Bitwise CPU-to-GPU agreement with batch-composition invariance; mixed-batch throughput and edit-to-first-step latency against Newton; kernel certificates, value unproven; a promotion rule; a closed, attributed energy balance.
8. **Two unguarded rungs** (C073 V, C074 P). No gate tests whether the baseline-controller rung ranks designs as the trained rung does, and the plan has no policy reuse between designs.
9. **Missing statements** (C075, C076 V). No sentence lets worlds in a batch carry different design parameters; no contact or row capacity is given.
10. **Hardware envelopes** (C052, C057, C058 V). Memory does not bind: 8,192 worlds use about 1–1.5% of a 24 GB card. Full-physics sampling control costs 3e3–2e5 policy rollouts per simulated second. The referee covers 0.1–2.4% of GPU world-seconds.
11. **No published tenfold** (C041 P). None in time to a qualified legged design; order-of-magnitude gains are published only for designs evaluated per unit of simulation.
12. **Tier agreement has a blind spot** (C082 P). All tiers share one actuator card, so agreement cannot expose a wrong actuator parameter.

## Conditions and Pivotal Unknowns

| Claim | Unknown | Cheapest resolving step | Why it matters |
|---|---|---|---|
| C010 | Incumbent phase times on the current stack | Time the incumbent on `trj` by phase | Sets which phase to attack |
| C042 | Do baseline-controller rankings match trained-policy rankings? | Thirty per-design trainings, 5–7.5 GPU-hours, owner approval | If not, the cheap rung cannot carry the speed claim |
| C005 | Incumbent edit-to-first-step latency (none published) | Time a topology edit in MuJoCo Warp and Newton | Decides whether edit latency counts |
| C016 | Share of candidates failing for statically detectable reasons | Best figure is 16%, so at most 1.19× | Pruning must rank viable designs |
| C043 | Correlation between tiers on paired differences | Compute from promotion pairs | 2× at 0.7, 30× at 0.98; maybe near zero for falls |
| C011 | A neutral adjudicating simulator | Owner names one in which neither arm trained | Otherwise the comparison is contestable |

## Recommended Actions and Experiments

1. **End-to-end metric with an incumbent arm** (C011 P). Time to qualified design, wall-clock primary; freeze and publish the incumbent tool layer before any run.
2. **Measure the incumbent first** (C010 P), seeded with published component figures.
3. **Re-specify the decision rule and withdrawal gate** (C012, C013 P). As drafted a true 10× passes in at most 18% of campaigns, and a threshold of 3 sits on the expected quadruped ratio. Use a pooled ratio with a lower bound above 5 and at least ten seeds per arm; partial gate at M3, full at M4.
4. **Rank-agreement gate between controller rungs** (C042 V), thresholds fixed before data.
5. **Per-world design parameters, a mixed-topology batch and an edit-latency target** (C079, C048 P), recorded as parity with the incumbent.
6. **Referee-fidelity estimate with a confidence sequence** (C043 P); `fs-uq` already has the estimators (C044 V).
7. **Referee coverage gate at M2 exit** (C080 V): scenarios completed under the 21-contact cap, and promotion latency.
8. **Policy inheritance along lineage, as a gated experiment** (C036 P); published reuse cut co-design time 67.4% in small design spaces (C038 V).
9. **Restate physics targets as ratios to MuJoCo Warp measured on `trj`** (C092 P), and add a published-figure anchor at M3 under the Parity label (C081 P).

## Conflicts After Audit

No factual conflict between audited claims survives. Two were narrowed:

- **Throughput targets** (C092 against C069, C070). "Do not seek the tenfold in step rate" stands; "leave the targets" does not.
- **Sampling control** (C032 against C086). Real-time control of one world is published; as the evaluator for many worlds it is rejected.

Three choices remain open as conventions or owner decisions: the adjudicator (C011), the rank-agreement thresholds (C042), and what a measured ratio between 3 and 10 means (C013).

## Rejected Tempting Claims

- **"No incumbent combines mixed topologies, determinism and articulated contact"** (C021, REJECTED). Newton 1.8.0's SolverSteel documents all three.
- **"De-scope the Lean capsules"** (C084, REJECTED). Only T-13 depends on blocked modules and it is already deferred; the owner constraint names franken_lean.
- **A bigger steps-per-second headline** (C092, C064 P). No order-of-magnitude headroom per step.
- **Sampling control as default evaluator** (C086 P). At least about 4e3 policy rollouts per simulated second.
- **Contact gradients before M6** (C093 V) and **half precision** (C091 P).

## Tail Hypotheses

- **The composition** (C025, HYPOTHESIS). One design identity and one evidence ledger across static analysis, tiers, statistics and solid mechanics. Test: the end-to-end benchmark against a composed incumbent baseline.
- **Morphology-conditioned policy prior** (C037, HYPOTHESIS). Could remove per-design training only inside its training distribution, and novel designs often fall outside. Needs an owner-approved compute envelope.

## Provenance and Limits

- **Coverage.** 94 claims; 76 audited by two independent auditors who wrote none of them: 31 VERIFIED, 39 PARTIALLY_SUPPORTED, 2 HYPOTHESIS, 2 REJECTED, 2 NON_MATERIAL.
- **18 claims were not audited and are excluded:** C035, C040, C045–C047, C049–C051, C053–C055, C060, C061, C087–C090, C094. They cover the hard-world set, splitting, gait-fixed-point gradients, the rollout cache, scene-memory envelopes, single-precision conditioning and five pre-emptive rejections. C091 leans on the unaudited C061.
- **Nothing was measured on an incumbent simulator.** The only executions were a Metal device query, arithmetic and a small Monte Carlo of the C012 rule. "Documented" means documented, not demonstrated; "not found" means a bounded search.
- **Artifacts.** `MOR_CLAIMS.jsonl` is unmodified; `.mor/run-f1/final/` is empty; audit downloads sit in a non-durable scratchpad.
