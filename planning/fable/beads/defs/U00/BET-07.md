---
key: "BET-07"
title: "Decide BET-07: symmetry buys sample efficiency on symmetric robots and symmetry-preserving tasks"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "marrow", "frob-learn", "solid", "bet", "gate"]
parent: "WP-29"
deps: ["BD-325", "BD-326", "N00-28", "N23a-33", "STD-E2E"]
needs: []
related: ["N00-12", "N40-31", "NS-05"]
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

In: pre-registration; the M4 reading of the three-arm experiment on a robot whose sensing keeps the symmetry
(the equivariant arm against the unaugmented arm and against the augmented arm); this bet's verdict and its
decision record `docs/adr/bet-07-m4.md`.

Out, and where it is:

- Which group the policy is tied under. Whether the observation-preserving subgroup Γ_obs, and not the larger
  mechanical group Γ_mech, is the right constraint; the diagnostic arm on a robot with asymmetric sensing; the
  gates of the symmetry analysis and of the exact block construction; the M8 tree-policy reading: all of these
  are BET-30. BET-30 reads the same record of GATE-CORTEX-03 and the same decision KILL-EQUIV for a different
  question, takes this bead's verdict as the input of its clause (b), and depends on this bead.
- The experiment and its gate record (BD-326); the equivariant maps (BD-325); the symmetry analysis (BD-102,
  BD-241).
- Applying the trigger of KILL-EQUIV and writing the kill-gate ADR (N23a-33). This bead reads that decision and
  records the bet verdict beside it.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. Equivariant policies need more than 0.7× the environment steps of unconstrained policies to reach the same
      return, across ten paired seeds.
   b. They do no better than symmetry augmentation at equal compute. Read by the same rule as the kill-gate
      bead N23a-33, so that the bet and the decision never judge the same record two ways: clause (b) holds
      when the paired 95% interval of the compute ratio (equivariant ÷ augmented, compute to the return target)
      includes or exceeds 1. When clause (a) passes and (b) holds, N23a-33 asks the owner whether equivariant
      networks stay as a non-default option; whatever the ruling, this bet is recorded `Falsified{b}` ("passed
      its gate, falsified by its second clause").
2. **Competing arm.** An unconstrained policy with symmetry augmentation. **Potency control** (the bet's, as the
   bet register names it; BD-326 also runs a second, different potency control, a planted advantage the paired
   comparison must detect, and both must be caught): a task that breaks the symmetry, on which the equivariant
   constraint must be refused before training starts.
   BD-326 uses an off-centre payload, which removes the mirror from Γ_obs: requesting the tie under the
   symmetric task's group is refused with FROB-REF-161 (group not observation-preserving, exit 3), naming the
   generator the payload breaks; where no non-trivial group remains, the request is refused as "nothing to tie"
   (N23a-31). A slippery patch whose side is drawn symmetrically in training does not break the group in law
   and is not this control: it is the gate's patch clause below.
3. **Deciding gate (TARGET).** GATE-CORTEX-03 (Design comparison, M4): the three-arm experiment with ten paired
   seeds: the equivariant arm reaches the return in at most 0.7 times the steps of the unaugmented arm with no
   loss of final return; the one-sided patch is passed (read as in BD-326: arm E's requirement pass rate on the
   one-sided-patch scenes of the out-of-distribution battery is not below arm N's by more than the registered
   tolerance, paired, and is above the registered floor; the policy is conditioned on the transformed terrain,
   so it can act asymmetrically there); the comparison with the augmented arm is reported, whichever wins.
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
- [ ] The symmetry-breaking task refused the constraint, and the patch clause is evaluated on the recorded
      paired pass rates against the registered tolerance and floor.
- [ ] The verdict is recorded in `docs/adr/bet-07-m4.md`, which cites the KILL-EQUIV decision on record in
      N23a-33's ADR (`docs/adr/ADR-KILL-EQUIV-M4.md`, the kill-gate record form of N40-03; by identifier and
      hash) together with the owner's
      ruling where clause (b) alone holds; with no decision on record the verdict is not filed.

## Test plan

### Unit tests
`bets.rs`: `bet07_ratio_0_62_holds` (upper bound of the paired step ratio 0.68 ≤ 0.7, final return not lower);
`bet07_ratio_0_78_falsifies_a`; `bet07_ties_augmentation_falsifies_b` (step ratio upper bound 0.66 passes
clause (a), but the paired 95% interval of the compute ratio equivariant ÷ augmented is [0.93, 1.08], which
includes 1 → `Falsified{b}`); `bet07_beats_augmentation_holds` (compute-ratio interval [0.71, 0.88] → clause
(b) does not hold); `bet07_final_return_loss_fails_gate`; `bet07_patch_clause_fails_gate` (step ratio upper
bound 0.66, but arm E's paired patch pass rate below arm N's by more than the tolerance → GATE-CORTEX-03 fails,
KILL-EQUIV fires, `Falsified`); `bet07_control_not_refused_is_untested` (a record in which the symmetry-breaking
request was admitted does not count as a pass: `Untested`, not `Held`).

### End-to-end script and logging
`scripts/e2e/bet_07_decide.sh` recomputes the verdict from the experiment's recorded gate rows with the bet
register's report (`cargo run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28; the
script reads the row of BET-07) and checks the ADR against it; under a fired kill gate it checks that
`frob train start <the symmetric fixture design> --task <the registered task> --tying equivariant --dry-run
--key b07-equiv --json` is refused as not admitted (process exit code 5, naming the kill record, as the
kill-gate bead N23a-33 specifies) and that the same call with `--tying augmentation --key b07-aug`, the
fallback, is admitted (exit 0). Each call carries its own key (a run verb without one is `FROB-REF-805`, exit 3,
which would mask the answer under test).
Logged per STD-E2E: per seed and arm the steps to return and final return, the paired ratio and bounds against
0.7, the patch outcome; `summary` naming GATE-CORTEX-03 and KILL-EQUIV.

### Known answers and negative controls
Negative controls: the symmetry-breaking task (refused); a zero-weight and a random policy, which must fail the
return threshold in every arm (the behaviour-not-plumbing rule; a shuffled-label student applies only where
distillation is involved); the potency control of BD-326 detected; seeds chosen after seeing curves (rejected by
the registration digest).

## Considerations

- Training curves produced under the nondeterministic training profile are not evidence; steps-to-return are
  taken from re-evaluation under a replayable profile.
- The right group may be smaller than the mechanical one (sensors, payload and task can break it); that
  refinement is BET-30.

## References

§4 (BET-07), §40.5, §23.13, §23.23; GATE-CORTEX-03; KILL-EQUIV.
