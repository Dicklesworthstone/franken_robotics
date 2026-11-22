---
key: "BET-07"
title: "Decide BET-07: symmetry buys sample efficiency on symmetric robots and symmetry-preserving tasks"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "marrow", "frob-learn", "solid", "bet", "gate"]
parent: "WP-29"
deps: ["N00-28", "BD-325", "BD-326", "STD-E2E"]
needs: []
related: ["BD-102", "BD-241", "BET-30", "NS-05", "N00-12"]
gates: ["GATE-CORTEX-03", "KILL-EQUIV"]
plan_refs: ["§4 (BET-07)", "§40.5", "§23.13", "§23.23", "§1.2"]
---
## Background

**The bet.** A robot's morphological symmetry group, computed from the design instead of being read from a
hand-written configuration file, can be built into the policy as an equivariance constraint, and on symmetric
robots doing tasks that preserve the symmetry this reduces the environment steps needed to reach a return
target. Equivariant policies and the symmetry classification of gaits are published; what the project adds is
computing the group from the design and checking a declared symmetry. The bet is that the constraint is worth
having at all, compared with the much cheaper alternative of symmetry augmentation.

**Why it could fail.** Augmentation recovers most of the benefit of invariance in many settings; an equivariant
network costs structure and can hurt when sensors, payload or task break the symmetry. Tier: Solid.

## Scope

In: pre-registration, the M4 reading of the three-arm experiment, verdict and decision record. Out: the
equivariant layers, the training runs, the symmetry analysis.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. Equivariant policies need more than 0.7× the environment steps of unconstrained policies to reach the same
      return, across ten paired seeds.
   b. They do no better than symmetry augmentation at equal compute.
2. **Competing arm.** An unconstrained policy with symmetry augmentation. **Potency control.** A task that
   breaks the symmetry (a one-sided patch of different friction, for example), on which the equivariant
   constraint must be refused before training starts.
3. **Deciding gate (TARGET).** GATE-CORTEX-03 (Design comparison, M4): the three-arm experiment with ten paired
   seeds: the equivariant arm reaches the return in at most 0.7 times the steps of the unaugmented arm with no
   loss of final return; the one-sided patch is passed (the constraint is refused there); the comparison with
   the augmented arm is reported, whichever wins.
4. **Kill gate and fallback.** KILL-EQUIV (M4): GATE-CORTEX-03 fails. Fallback: symmetry is used for
   augmentation and for metamorphic testing only; no equivariant layers ship.
5. **Downstream if falsified.** The scenario "symmetry for free" is demonstrated only up to the symmetry report
   and the gait list; the block-diagonal analysis of modes and the metamorphic relations keep using the group;
   the observation-preserving subgroup bet (BET-30) loses its learning half.
6. **Evidence so far.** Published sample-efficiency gains for equivariant policies on symmetric robots (FACT
   about sources); none on this system.
7. **Procedure.** Register the step ratio 0.7, the seed list and the return target before any run; read at M4
   exit; `docs/adr/bet-07-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the ten seeds, the return target and the step budget per arm.
- [ ] Steps-to-return for the three arms per seed, the paired ratio with its lower and upper bounds, and final
      returns.
- [ ] The symmetry-breaking task refused the constraint.
- [ ] Verdict and the KILL-EQUIV decision recorded as an ADR.

## Test plan

### Unit tests
`bets.rs`: `bet07_ratio_0_62_holds` (upper bound of the paired step ratio 0.68 ≤ 0.7, final return not lower);
`bet07_ratio_0_78_falsifies_a`; `bet07_ties_augmentation_falsifies_b` (equivariant and augmented arms within
seed spread at equal compute); `bet07_final_return_loss_fails_gate`; `bet07_patch_not_refused_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_07_decide.sh` recomputes the verdict from the experiment's ledger records and checks the ADR;
under a fired kill gate it checks that `frob train start --equivariant` returns a refusal naming the fallback.
Logged per STD-E2E: per seed and arm the steps to return and final return, the paired ratio and bounds against
0.7, the patch outcome; `summary` naming GATE-CORTEX-03 and KILL-EQUIV.

### Known answers and negative controls
Negative controls: the symmetry-breaking patch; a zero-weight and a shuffled-label policy, which must fail the
return threshold in every arm (a learning gate needs failing controls); seeds chosen after seeing curves
(rejected by the registration digest).

## Considerations

- Training curves produced under the nondeterministic training profile are not evidence; steps-to-return are
  taken from re-evaluation under a replayable profile.
- The right group may be smaller than the mechanical one (sensors, payload and task can break it); that
  refinement is BET-30.

## References

§4 (BET-07), §40.5, §23.13, §23.23; GATE-CORTEX-03; KILL-EQUIV.
