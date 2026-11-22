---
key: "BET-15"
title: "Decide BET-15: family policies (one policy per morphology class decides most designs without per-design training)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "frob-learn", "frontier", "bet", "gate"]
parent: "WP-61"
deps: ["N00-28", "N00-07", "BD-320", "BD-321", "BD-328", "STD-E2E"]
needs: ["§23.11: the pre-funding pilot of family policies and the break-even count N = P/(s·(1 − f))", "§42 (OPEN-CORTEX-1): the owner-approved compute envelope for pretraining one family policy (class E3, approved per run with its ceiling and kill rule written first)"]
related: ["BET-16", "BET-17", "BD-322", "NS-19", "N00-11", "N00-20"]
gates: ["GATE-CORTEX-11", "KILL-FAMILY"]
plan_refs: ["§4 (BET-15)", "§40.5", "§0.8 (H-02)", "§2.4 (lever 2)", "§23.23", "§43.7"]
---
## Background

**The bet.** Training a policy per design is the dominant cost of today's design loops (about 85% of a composite
quadruped iteration, over 90% for humanoids). The "train few" lever proposes one policy per morphology class,
conditioned on the design's parameters, that walks most designs of the class with no per-design training, so
that only shortlisted designs are ever trained. Tier: Frontier.

**What the evidence supports.** A universal policy across all morphologies is out of reach at an affordable
pretraining budget and is not attempted. Within one class, on flat or mildly varied ground with fixed
actuation, published work shows zero-shot return at 70–85% of a specialist's and fine-tuning at about 2–25% of a
from-scratch run stopped at matched return. Across topology-diverse designs at affordable pretraining: 10–40%
zero-shot. Within a class on interior terrain, with payload or actuator-card changes, there is no published
figure. One published policy trained on about 1,000 embodiments used over 2 × 10¹² steps on 160 GPUs for five
days: a scale two cards cannot reach.

## Scope

In: pre-registration, the pilot's result as a precondition, the M4 reading, the break-even computation, verdict
and decision record. Out: the policy architecture, pretraining, the experiment itself.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. Zero-shot return on held-out in-class designs is below 70% of the per-design specialist's.
   b. Rank agreement with specialists is below the threshold of the rank-agreement bet (BET-17).
   c. Pretraining, counted in compute, is not repaid within the break-even count N = P/(s·(1 − f)) at the
      measured values, where P is the pretraining cost, s the cost of one specialist training and f the fraction
      of a specialist's cost that using the family policy still spends per design (fine-tuning and evaluation).
      Illustrative: 60 GPU-hours at f = 0.1 repay after about 590 quadruped designs at s = 410 s
      (216,000 s ÷ (410 s × 0.9) = 585), or after 70 to 130 humanoid designs.
2. **Competing arm.** Per-design PPO from the baseline warm start. **Potency control.** Designs outside the
   class must be flagged out-of-distribution and declined, not walked badly in silence.
3. **Deciding gate (TARGET).** GATE-CORTEX-11 (Design comparison, M4): in one class on held-out parametric
   designs: median return with no adaptation at least 70% of a specialist's; fine-tuning reaches specialist
   return in at most 20% of the steps of a from-scratch run stopped at 95% of its final return, on the task
   family's terrain set including stairs, slopes and wet patches; out-of-class designs declined; re-described
   robots (the same body under a different description convention) keep at least 80% of return.
4. **Preconditions.** The pre-funding pilot is on record with its falsifier not met; the compute envelope is
   approved by the owner. If the envelope stays at its default, the bet is `Untested` and M4 exits in its
   variant with no family policy.
5. **Kill gate and fallback.** KILL-FAMILY (M4): at GATE-CORTEX-11, return with no adaptation below 70% of
   specialists on more than half of the held-out set and fine-tuning above 20% of specialist steps; or the
   pilot's falsifier holds. Fallback: no family policy; per-design specialists with inheritance from the nearest
   parent, a per-design behaviour-cloning warm start from the baseline, the sampling fallback for
   out-of-distribution designs, and the baseline as the screen.
6. **Downstream if falsified.** The rungs become the static check, the baseline controller, the sampling
   controller, gait orbits and specialists; the ceilings of the thesis are recomputed with the measured trained
   fraction; classes whose ceiling falls under 3 leave the speed claim; the scenario "a family policy walks an
   unseen design" is not demonstrated; the ten-fold report records the factor that was lost.
7. **Procedure.** Register thresholds, the held-out design split, the terrain set and the measured P, s, f
   definitions before pretraining; read at M4 exit; `docs/adr/bet-15-m4.md`.

## Acceptance criteria

- [ ] Pilot record and envelope approval exist before any pretraining run, or the bet is recorded `Untested`
      with the reason.
- [ ] Held-out median zero-shot return ratio, fine-tuning step fraction against the early-stopped control,
      out-of-class decline rate, re-description retention, and rank agreement are recorded per class.
- [ ] The break-even count is computed from measured P, s and f, and compared with the number of designs the
      program expects to evaluate in the class.
- [ ] Verdict and the KILL-FAMILY decision recorded as an ADR.

## Test plan

### Unit tests
`bets.rs`: `bet15_breakeven_585` (P = 216,000 GPU-s, s = 410, f = 1/10 → N = 585.4, printed "about 590");
`bet15_zero_shot_0_74_and_ft_0_15_holds`; `bet15_zero_shot_0_66_falsifies_a`;
`bet15_not_repaid_falsifies_c` (N = 585 against 200 expected designs); `bet15_no_envelope_is_untested`;
`bet15_out_of_class_walked_silently_fails_gate`.

### End-to-end script and logging
`scripts/e2e/bet_15_decide.sh` recomputes the verdict from the experiment's records, recomputes N, and under a
fired kill gate checks that `frob policy apply` returns a refusal naming the fallback route. Logged per
STD-E2E: class, held-out count, return ratios, step fractions, P, s, f, N, the expected design count;
`summary` naming GATE-CORTEX-11 and KILL-FAMILY.

### Known answers and negative controls
N = P/(s·(1 − f)) with the illustrative values above. Negative controls: a six-legged design offered to the
quadruped policy (must be declined); held-out designs that leaked into pretraining (the split digest is checked
against the generator's record); a from-scratch control run to completion instead of stopped at 95% of final
return (inflates the saving; rejected).

## Considerations

- The comparison control is an early-stopped from-scratch run; comparing against a fixed budget overstates the
  gain.
- Avoided training is counted once, here, and never again under reuse or inheritance.
- A family policy's verdicts are screening evidence; acceptance of a finalist still needs its own qualified
  result.

## References

§4 (BET-15), §40.5, §0.8 (H-02), §2.4, §23.23, §43.7; GATE-CORTEX-11; KILL-FAMILY.
