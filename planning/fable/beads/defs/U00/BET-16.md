---
key: "BET-16"
title: "Decide BET-16: inheritance (a child's policy fine-tuned from its parent needs a small fraction of from-scratch steps)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "frob-learn", "frontier", "bet", "gate"]
parent: "WP-61"
deps: ["N00-28", "N00-07", "BD-322", "STD-E2E"]
needs: ["§23.12: the edit classes of typed design edits (parameter edits; single-limb topology edits), the audit twin and the depth schedule for resets"]
related: ["BET-15", "BET-17", "BD-058", "BD-113", "BD-320", "NS-19", "N00-12"]
gates: ["GATE-CORTEX-13", "KILL-INHERIT"]
plan_refs: ["§4 (BET-16)", "§40.5", "§0.8 (H-21)", "§2.4 (lever 2)", "§23.23"]
---
## Background

**The bet.** Designs in a studio are not independent: each is a typed edit of a parent. When a child differs
from its parent by a small parameter edit (a femur 8% longer, a wider stance), the parent's policy should be a
far better starting point than nothing. The bet is that fine-tuning from the parent, through a mapping that the
typed edit defines, reaches the child's return target in a small fraction of the steps a from-scratch run
needs. With family policies it forms the "train few" lever. Tier: Frontier.

**What the evidence supports.** No published figure measures a small typed parameter edit from parent to child.
Transfer from a multi-embodiment prior to a similar body costs about 2–25% of a from-scratch run stopped at
matched return; the one parent-to-child figure is about 50%; single-limb topology edits cost 33–100%. Chained
inheritance risks loss of plasticity (a network that has been fine-tuned many times learns worse) and narrows
morphological diversity.

## Scope

In: pre-registration, the M4 reading per edit class, verdict and decision record. Out: the mapping table, the
training runs, the audit twin.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. A parameter edit needs more than 20% of the steps of a from-scratch control stopped at 95% of its final
      return.
   b. Inheritance does not beat a raw-checkpoint warm start (loading the parent's weights with no mapping) for
      the edit class.
   c. After a chain of ten inherited generations the final return is more than 5% below a from-scratch control
      (loss of plasticity).
2. **Competing arm.** From-scratch training with the baseline warm start. **Controls.** An arm that resets on a
   measured regression of its *audit twin* (a from-scratch copy trained alongside to detect when the inherited
   policy has fallen behind), and an arm with a fixed depth schedule of resets.
3. **Deciding gate (TARGET).** GATE-CORTEX-13 (Design comparison, M4): on 12 parent-child edit pairs, 4 of them
   blind: median adaptation samples for parameter edits at most 20% of a from-scratch run stopped at 95% of its
   final return, and fewer than a raw-checkpoint warm start needs; the inherited child more than 5% worse in at
   most 20% of edits; a chain of 10 edits shows no decline under scheduled resets (audit twin within 5%).
   Topology edits are reported, not required.
4. **Kill gate and fallback.** KILL-INHERIT (M4): median parameter-edit cost above 60% of from-scratch steps, or
   the inherited arm more than 5% worse in over 20% of edits. Fallback: every child trains from the class prior
   or from scratch; the mapping table is kept for loading policies. Between the bet's 20% and the kill gate's
   60% the bet is falsified without the kill gate firing: inheritance is offered for parameter edits only, with
   resets on a measured regression and a depth schedule where no audit twin exists, and its measured cost is
   printed; topology edits start from the family policy or from scratch.
5. **Bound on the gain.** Whatever the result, the gain is bounded by the share of designs that are trained at
   all; it is counted once, under "train few", and never again as "reuse".
6. **Downstream if falsified.** Shortlisted designs are trained from the class prior or from scratch; the
   ceilings are recomputed with the measured cost; the second half of the scenario "a family policy walks an
   unseen design" (inheritance reaches the target in 12% of the steps, an illustrative figure) is not
   demonstrated.
7. **Procedure.** Register the 12 pairs (the 4 blind ones sealed), the return targets, the control's stopping
   rule and the chain protocol before any run; read at M4 exit; `docs/adr/bet-16-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with pair list, blind subset sealed, and the early-stopping rule for controls.
- [ ] Per pair: steps for the inherited arm, the raw-checkpoint arm and the early-stopped control; the median
      step fraction; the share of edits where the child is more than 5% worse.
- [ ] The ten-edit chain result with the audit twin's return, under both reset controls.
- [ ] Verdict and the KILL-INHERIT decision recorded as an ADR; topology-edit figures reported.

## Test plan

### Unit tests
`bets.rs`: `bet16_median_0_14_holds`; `bet16_median_0_27_falsifies_a_without_kill`;
`bet16_median_0_64_fires_kill_inherit`; `bet16_no_better_than_raw_checkpoint_falsifies_b`;
`bet16_chain_minus_7_percent_falsifies_c`; `bet16_worse_in_3_of_12_fires_kill` (25% > 20%).

### End-to-end script and logging
`scripts/e2e/bet_16_decide.sh` recomputes the per-pair fractions from re-evaluation records (never from training
curves), the median, the chain comparison and the verdict; under a fired kill gate it checks that `frob train
inherit` returns a refusal naming the fallback. Logged per STD-E2E: pair, edit class, steps per arm, fraction,
child return against control, chain returns per generation; `summary` naming GATE-CORTEX-13 and KILL-INHERIT.

### Known answers and negative controls
Thresholds 20%, 5%, 60%. Negative controls: a control run to completion instead of stopped at 95% (rejected);
a blind pair unsealed before its run; shuffled-label and zero-weight policies failing the return target.

## Considerations

- Steps and returns come from re-evaluation under a replayable profile; numbers seen during nondeterministic
  training are not evidence.
- Inheritance can bias a campaign toward the parent's neighbourhood; the campaign's diversity measure is
  reported beside the saving.

## References

§4 (BET-16), §40.5, §0.8 (H-21), §2.4, §23.23; GATE-CORTEX-13; KILL-INHERIT.
