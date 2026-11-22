---
key: "BET-12"
title: "Decide BET-12: narrow purity is affordable (the schedule gate at the end of M1)"
type: "task"
priority: 0
milestone: "M1"
labels: ["m1", "delivery", "suite", "solid", "bet", "gate", "owner-decision"]
parent: "EPIC-M1"
deps: ["N00-28", "BD-044", "BD-147", "STD-E2E"]
needs: ["§43.1: the planning estimates per milestone (M0: 2 elapsed weeks for a swarm; M1: 5) against which the slip is measured", "Appendix F: the interim posture of each upstream work item, used to attribute a slip"]
related: ["BET-04", "BD-037", "BD-008", "BD-023", "N00-08"]
gates: []
plan_refs: ["§4 (BET-12)", "§40.5 (the schedule gate)", "§43.1", "§43.4", "§6.3"]
---
## Background

**The bet.** The product's core is built only on the owner's own suite of sibling projects, with GPU annexes of
this project's own, and with no foreign physics engine in any production dependency graph. Foreign engines
appear only as development-time comparison oracles and as the rival arm of the ten-fold test, run out of
process. The bet is that this "narrow purity" is affordable: that the siblings supply enough, or can be changed
quickly enough in their own repositories, that the first two milestones do not stall for want of a primitive.

**Why it is a bet.** Wrapping an established engine would be the fast route to a batch simulator. The design
refuses it because the thesis is a composition over one typed algebra and one evidence ledger, which a wrapped
engine would cut in half. The price is dependence on siblings whose APIs move daily. This is a program gate: it
decides whether the program continues as planned. Tier: Solid.

## Scope

In: pre-registration of the estimate and the attribution rule, the reading at M1 exit, the verdict and, if not
met, the trigger of the written re-plan. Out: the re-plan itself (an owner decision); the admission and
upstream work.

## Technical approach

1. **Falsifier.** M0 plus M1 slip by more than 50% because a sibling lacked a primitive, its upstream work item
   had not landed and no interim posture applied. With planning estimates of 2 and 5 elapsed weeks, the gate
   asks its question if the two milestones took more than 10.5 weeks for that cause.
2. **Measurement.** (a) Elapsed time of M0 plus M1, from the first implementation session to the M1 exit
   record. (b) The slip attributed by cause in the M1 exit review, using the delivery log: for each blocked
   interval, the bead that was blocked, the sibling primitive it waited for, the upstream work item, and whether
   an interim posture existed and was used. Only intervals with all three conditions (primitive missing,
   upstream item not landed, no applicable interim posture) count toward the falsifier.
3. **No competing arm and no potency control are defined for this bet;** its check is an accounting one. To
   keep the accounting honest the attribution is done by the fresh-context exit-review session, not by the
   implementers, and every counted interval cites delivery-log entries.
4. **Deciding gate.** The schedule gate (a program gate, end of M1): not met if the two milestones took more
   than 1.5 times their estimate for the named cause.
5. **Consequence if not met.** The program pauses for a written re-plan before M2 starts. The re-plan weighs,
   at least: more upstream capacity; the worker-process and lift fallbacks of the suite-consumption bet; and
   whether any exception to the foreign-engine anti-goal is proposed (which would be an owner decision recorded
   as an ADR, never a quiet dependency).
6. **Downstream.** A met gate changes nothing. An unmet gate blocks the funding of M2 work packages until the
   re-plan is accepted.
7. **Evidence so far.** The planning estimates exist to make this bet testable; they are not acceptance evidence
   and no other gate cites them. The critical path M0 to M4 plus M7 sums to 33 weeks.
8. **Procedure.** Register the estimates, the start-event definition and the attribution rule before the first
   implementation session; read at M1 exit; `docs/adr/bet-12-m1.md`.

## Acceptance criteria

- [ ] Pre-registered: estimates (2 and 5 weeks), the start event, the three-condition attribution rule.
- [ ] The M1 exit record states elapsed weeks, total slip, and slip attributed to the named cause, each counted
      interval citing delivery-log entries.
- [ ] Verdict recorded by the exit-review session; if not met, the pause is recorded and M2 funding records are
      refused until the re-plan ADR exists.

## Test plan

### Unit tests
`bets.rs`: `bet12_nine_weeks_holds`; `bet12_twelve_weeks_other_causes_holds` (elapsed 12 weeks, attributed 0.5
week to the named cause → met); `bet12_twelve_weeks_named_cause_not_met` (elapsed 12, of which 5 weeks blocked
with all three conditions → elapsed exceeds 10.5 for the cause → not met);
`bet12_interval_with_interim_posture_not_counted`; `bet12_attribution_by_implementer_rejected`.

### End-to-end script and logging
`scripts/e2e/bet_12_decide.sh` reads the delivery log and the exit record, recomputes elapsed and attributed
time, checks every counted interval's citations resolve, and recomputes the verdict; if not met it checks that
`funding admit` refuses an M2 package. Logged per STD-E2E: start and end events, elapsed weeks, each attributed
interval with its three conditions, the 10.5-week threshold; `summary`.

### Known answers and negative controls
Arithmetic: 1.5 × (2 + 5) = 10.5 weeks. Negative controls: an interval counted without a delivery-log citation;
an interval for which an interim posture existed; a start event moved after the fact (rejected by the
registration digest).

## Considerations

- A slip for other reasons (scope, defects of this project's own) is recorded and is not this bet.
- The gate measures elapsed time for a swarm of agents; it is not a statement about effort.
- The owner may re-plan earlier; the gate is a floor, not the only moment a re-plan can happen.

## References

§4 (BET-12), §40.5 (the schedule gate), §43.1, §43.4, §6.3.
