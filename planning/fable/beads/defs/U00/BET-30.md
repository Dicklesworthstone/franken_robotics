---
key: "BET-30"
title: "Decide BET-30: equivariance under the observation-preserving subgroup (Γ_obs ≤ Γ_mech is the right group)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "marrow", "sensorium", "frob-learn", "frontier", "bet", "gate"]
parent: "WP-29"
deps: ["N00-28", "N00-07", "BD-241", "BD-325", "BD-326", "BET-07", "STD-E2E"]
needs: ["§23.13: the third arm of the equivariance experiment on a robot with asymmetric sensing (a Γ_mech-constrained policy against a Γ_obs-constrained one)"]
related: ["BD-102", "BD-246", "BD-439", "NS-05", "N00-12"]
gates: ["GATE-MARROW-03", "GATE-MARROW-04", "GATE-CORTEX-03", "KILL-EQUIV", "KILL-MARROW-BLOCKS"]
plan_refs: ["§4 (BET-30)", "§40.5", "§0.8 (H-24)", "§13.10", "§23.23"]
---
## Background

**The bet.** A robot's mechanical symmetry group Γ_mech (for example C₂ × C₂ for a quadruped that is symmetric
left to right and front to back) is not always the group a policy may be constrained to. A front-only depth
camera, a one-sided payload or a task with a preferred direction breaks part of it. The subgroup that also
preserves sensors, payload and task is Γ_obs ≤ Γ_mech (the observation-preserving subgroup). The bet is that
Γ_obs, not Γ_mech, is the right group for equivariant policies: constraining to the larger group on a robot with
asymmetric sensing costs return. Tier: Frontier.

**The Moonshot rider.** The same bet row carries the falsifier of a geometric-algebra tree policy [M]:
invariance to heading and to description conventions is necessary, augmentation recovers most of it, and there
is no locomotion evidence for the geometric-algebra specifics (expected gain 0–1.3×). That half is read at M8
by its own gate and kill gate and does not block this bead; its result is appended to the decision record when
its epic closes.

## Scope

In: pre-registration; the M4 reading of the one question this bet adds to BET-07, namely whether Γ_obs and not
the larger Γ_mech is the group a policy must be tied under. It is read on the diagnostic arm of the three-arm
experiment (a robot with asymmetric sensing, clause (a)) and on the gates of the symmetry analysis and of the
exact block construction (GATE-MARROW-03, GATE-MARROW-04). Also in: the M8 addendum for the tree policy, this
bet's verdict and its decision record `docs/adr/bet-30-m4.md`.

Out, and where it is:

- Whether an equivariant policy is worth having at all on a symmetric robot (more than 0.7× the steps of the
  unconstrained arm, or no better than augmentation): BET-07. Both bets read one record of GATE-CORTEX-03 and
  one decision KILL-EQUIV. Clause (b) here takes BET-07's verdict as its input and does not decide it again,
  which is why this bead depends on BET-07.
- The symmetry analysis (BD-102, BD-241, BD-246); the equivariant maps (BD-325); the experiment with its
  diagnostic arm (BD-326); the tree policy at M8 (BD-439).
- Applying the triggers of KILL-EQUIV (N23a-33) and KILL-MARROW-BLOCKS (N13-24) and writing their ADRs. This
  bead reads those decisions.

## Technical approach

1. **Falsifier clauses** (either falsifies the Frontier half):
   a. On a robot with asymmetric sensing, a policy constrained to Γ_mech does not lose return against one
      constrained to Γ_obs (then the distinction is unnecessary).
   b. The Γ_obs policy fails the threshold of the symmetry bet (it does not reach the return in at most 0.7× the
      steps of the unconstrained arm).
   Moonshot half: the tree policy is falsified if it does not reach the threshold in 1/1.3 of the steps of a
   plain morphology transformer.
2. **Competing arms.** A Γ_mech-equivariant policy; augmentation only; a plain morphology transformer (for the
   Moonshot half). **Potency control.** Planted asymmetries (a moved sensor, a one-sided payload, a changed
   sole material, a reversed gear) for which the analysis must name the correct breaker.
3. **Deciding gates (TARGETs).**
   - GATE-MARROW-03 (Known answer, M3): Γ_mech and Γ_obs equal the groups on file for all eighteen archetypes
     and for 24 planted asymmetries, with the correct breaker named in every case; zero disagreements.
   - GATE-MARROW-04 (Known answer, M4): the equivariant-map construction holds exactly for every group of the
     character-table gate on 100 random pairs of channel spaces each; all eight registered mutations are caught
     by at least three fixtures each.
   - GATE-CORTEX-03 (Design comparison, M4): the three-arm experiment, ten paired seeds, 0.7× step ratio.
   - GATE-CORTEX-17 (M8, Moonshot): one tree-policy arm is at least 1.3 times faster than the plain arm to the
     return threshold at equal wall-clock, or better with no adaptation beyond seed spread.
4. **Kill gates and fallbacks.** KILL-EQUIV (M4): the equivariance gate fails; symmetry is used for augmentation
   and metamorphic testing only. KILL-MARROW-BLOCKS (M4): the exact block construction fails after one repair
   cycle or exceeds its bit budget for n ≤ 12; equivariant layers by Reynolds projection in double precision
   with an idempotence test, classed Tested. KILL-GATREE (M8): the plain morphology transformer remains. The
   bet's own fallback: augmentation and metamorphic tests under Γ_obs.
5. **Downstream if falsified.** Under (a): policies may be constrained to Γ_mech where it is larger, and the
   Γ_obs computation remains an analysis (it is still needed for campaign quotienting and for sensing
   feasibility). Under (b): no equivariant layers ship; Γ_obs is used for augmentation and tests.
6. **Procedure.** Register the asymmetric-sensing robot, the arms, seeds and return target before any run; read
   at M4 exit; append the M8 result; `docs/adr/bet-30-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the robot, its Γ_mech and Γ_obs on file, and the planted asymmetries.
- [ ] The symmetry analysis names the correct breaker in 24 of 24 planted cases.
- [ ] Final return of the Γ_mech-constrained and Γ_obs-constrained arms with a paired comparison; the Γ_obs
      arm's step ratio against 0.7.
- [ ] Verdict and the kill decisions recorded as ADRs; the Moonshot half marked `pending M8` until its epic
      closes.

## Test plan

### Unit tests
`bets.rs`: `bet30_gmech_loses_return_and_gobs_ratio_0_64_holds`; `bet30_gmech_no_loss_falsifies_a`;
`bet30_gobs_ratio_0_78_falsifies_b`; `bet30_wrong_breaker_named_fails_marrow03`;
`bet30_moonshot_half_pending_does_not_block`; `bet30_tree_policy_1_1x_falsifies_moonshot_half`.

### End-to-end script and logging
`scripts/e2e/bet_30_decide.sh` runs `frob analyze symmetry` and `frob sense symmetry` on the archetype with a
front-only depth camera (expects Γ_mech = C₂ × C₂ and Γ_obs = C₂ with the camera named as breaker), recomputes
the arm comparison from the ledger and the verdict. Logged per STD-E2E: design, groups as generator lists,
breakers, per-seed returns and steps per arm, ratios; `summary` naming the gates and kill gates.

### Known answers and negative controls
The quadruped with a front-only depth camera: Γ_mech = C₂ × C₂ (order 4), Γ_obs = C₂ (order 2), breaker = the
camera's mount. Negative controls: the 24 planted asymmetries; a declared symmetry that differs from the
computed one (refused, FROB-REF-109); a policy constrained to Γ_mech on a symmetry-breaking task without
refusal.

## Considerations

- Γ_obs depends on the task as well as the body; it is computed per robot and task, never cached per robot
  alone.
- At most four Moonshot items are active at once; the tree-policy half waits its turn.

## References

§4 (BET-30), §40.5, §0.8 (H-24), §13.10, §23.23; GATE-MARROW-03, -04; GATE-CORTEX-03, -17; KILL-EQUIV;
KILL-MARROW-BLOCKS; KILL-GATREE.
