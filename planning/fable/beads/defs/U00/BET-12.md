---
key: "BET-12"
title: "Decide BET-12: narrow purity is affordable (the schedule gate at the end of M1)"
type: "task"
priority: 0
milestone: "M1"
labels: ["m1", "delivery", "suite", "solid", "bet", "gate", "owner-decision"]
parent: "EPIC-M1"
deps: ["BD-037", "BD-044", "BD-147", "N00-09", "N00-28", "N40-06", "N40-11", "STD-E2E"]
needs: []
related: ["N00-08", "N40-31", "N40-32"]
gates: ["GATE-PROG-SCHEDULE"]
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

In: pre-registration of the bet's falsifier with the estimates, the start event and the rules it rests on; the
reading of the schedule gate's record at the M1 exit; the bet verdict and its ADR; and, if the bet is falsified,
the bet-side consequence: funding records of M2 work packages are refused until the re-plan ADR exists.

Out, and all in N40-11 (a dependency; this bead reads that gate's record and holds no second computation): the
elapsed time T, the cause tags and the attribution of blocked time (B_q and B_n), the three-branch decision
rule, the register row `GATE-PROG-SCHEDULE`, the pause mechanism (the M1 exit report `not-ready`, the pre-check
that refuses to start an M2 bead) and the content required of a re-plan. Also out: the re-plan itself (an owner
decision); the admission and upstream work.

## Technical approach

1. **Falsifier.** M0 plus M1 slip by more than 50% because a sibling lacked a primitive, its upstream work item
   had not landed and no interim posture applied. With planning estimates of 2 and 5 elapsed weeks, the gate
   asks its question if the two milestones took more than 10.5 weeks for that cause.
2. **Measurement: read from the gate.** The schedule gate is measured by N40-11, and this bead takes its record
   as it stands: (a) T, the elapsed time of M0 plus M1; (b) B_q, the weeks blocked for the qualifying cause, and
   B_n, the weeks blocked for other causes, attributed from the blocked intervals of the per-bead logs (for each
   interval: the bead that was blocked, the sibling primitive it waited for, the upstream work item, and whether
   an interim posture existed and was used); (c) the branch of the decision rule that was taken. Only intervals
   with all three conditions (primitive missing, upstream item not landed, no applicable interim posture) count
   toward the falsifier; that is the qualifying rule of N40-11. The start and end events are one definition
   (DESIGN, the same in N40-11; the plan says "the elapsed time of M0 plus M1" and names no event): T runs from
   `started` of M0 in `registries/milestones.toml`, the first M0 bead moving to in-progress, to the candidate
   exit of M1 at which the exit review evaluates the gate. The first M0 bead is the first bead of the first
   implementation session, so this is the start this bet registers; reason for the registry's event: it is
   written by the exit runner and cannot be typed by hand.
3. **No competing arm and no potency control are defined for this bet;** its check is an accounting one. To
   keep the accounting honest the gate's row, with its attribution, is recorded by the fresh-context exit-review
   session, not by the implementers (the cause tags in the logs are raw data written when a bead was blocked;
   the attribution is the computation over them and its record), and every counted interval cites log lines.
   A gate record whose recording session implemented M0 or M1 work is rejected as input to the verdict.
4. **Deciding gate and verdict.** The schedule gate (a program gate, end of M1; register row
   `GATE-PROG-SCHEDULE`, the DESIGN identifier N40-11 gives it): not met if the two milestones took more than 1.5
   times their estimate for the named cause. The bet verdict follows the branch N40-11 recorded: branch 1
   (T ≤ 10.5 weeks, gate met) → the bet holds at M1; branch 2 (T > 10.5 and T − B_q ≤ 10.5, gate not met) → the
   bet is falsified; branch 3 (T > 10.5 and T − B_q > 10.5, met with a recorded slip) → the bet is not falsified
   by this evidence, and the verdict records the slip and its causes.
5. **Consequence if not met.** The program pauses for a written re-plan before M2 starts. The pause mechanism
   and the list of what a re-plan must state are N40-11's (each missing primitive with its chosen route, among
   them more upstream capacity, the worker-process and lift fallbacks of the suite-consumption bet, a change of
   scope, or an exception to the foreign-engine anti-goal, which would be an owner decision recorded as an ADR,
   never a quiet dependency). This bead adds the bet-side refusal to N00-09's `admit` (run as
   `cargo run -p frob-conformance --example funding -- admit --record <path> --json`, the example N00-09
   declares): a funding record for an M2 work package is refused until the same re-plan ADR exists.
6. **Downstream.** A met gate changes nothing. An unmet gate blocks the funding of M2 work packages until the
   re-plan is accepted.
7. **Evidence so far.** The planning estimates exist to make this bet testable; they are not acceptance evidence
   and no other gate cites them. The critical path M0 to M4 plus M7 sums to 33 weeks.
8. **Procedure.** Register the estimates, the start-event definition and the attribution rule before the first
   implementation session; read at M1 exit; `docs/adr/bet-12-m1.md`.

## Acceptance criteria

- [ ] Pre-registered, by digest: estimates (2 and 5 weeks), the start event (`started` of M0 in the milestone
      registry), the three-condition attribution rule and the three-branch decision rule of N40-11.
- [ ] The M1 exit record states elapsed weeks, total slip, and slip attributed to the named cause, each counted
      interval citing log lines; these values are those of the row `GATE-PROG-SCHEDULE` (T, B_q, B_n, branch)
      and are not computed a second time here.
- [ ] Verdict recorded by the exit-review session; if not met, the pause is recorded and M2 funding records are
      refused until the re-plan ADR exists.

## Test plan

### Unit tests
`bets.rs` (each on a gate record of N40-11, labelled `synthetic`): `bet12_nine_weeks_holds` (T = 9 → branch 1
→ holds); `bet12_twelve_weeks_other_causes_holds` (T = 12, B_q = 0.5 → T − B_q = 11.5 > 10.5 → branch 3 → not
falsified, slip recorded); `bet12_twelve_weeks_named_cause_not_met` (T = 12, B_q = 5 with all three conditions
→ T − B_q = 7 ≤ 10.5 → branch 2 → falsified); `bet12_interval_with_interim_posture_not_counted` (the gate
record of a history in which an interim posture applied has B_q = 0, and the verdict uses that value: the bet
recounts nothing); `bet12_attribution_by_implementer_rejected` (a gate record whose recording session
implemented M0 or M1 work is refused as input); `bet12_verdict_follows_branch` (the three branches map to
holds, falsified, not falsified with slip); `bet12_falsified_refuses_m2_funding` (on a branch-2 record with no
re-plan ADR, `admit` of a funding record for an M2 work package is refused with the reason "schedule gate: re-plan
pending" and the gate record's path; once the ADR "re-plan after the schedule gate" of N40-11 exists, the same
record is admitted; an M1 package is never refused by this rule).

### End-to-end script and logging
`scripts/e2e/bet_12_decide.sh` reads the row `GATE-PROG-SCHEDULE` and the exit record, re-runs the gate's own
`attribute` and `decide` of N40-11 on the same logs as a re-check (it contains no second implementation of
either), checks that every interval counted in B_q cites log lines that resolve, and maps the branch to the bet
verdict; if the bet is falsified it checks that `cargo run -p frob-conformance --example funding -- admit
--record <an M2 package's record> --json` exits 3 with the reason "schedule gate: re-plan pending" and the gate
record's path, and that the same call on an M1 package's record exits 0. Logged through the STD-E2E
helper to `target/e2e-logs/bet_12_decide/<utc>.ndjson`: per line `ts`, `script`, `bead`, `step`, `event`, the
command, exit code and wall time, the evidence block `{claim: known-answer, class: Tested, tier: G0:host,
determinism: D0, seed: 0}`; the start and end events, T, B_q and B_n in weeks beside the 10.5-week threshold, each
attributed interval with its three conditions and its log-line citations, the branch and the verdict, the digests
of the gate record and the registration; on failure the first interval or value that differs and the paths of the
logs read; a `summary` line with assertion counts naming GATE-PROG-SCHEDULE.

### Known answers and negative controls
Arithmetic: 1.5 × (2 + 5) = 10.5 weeks. Negative controls: an interval counted without a delivery-log citation;
an interval for which an interim posture existed; a start event moved after the fact (rejected by the
registration digest); an M2 funding record admitted while the bet is falsified and no re-plan ADR exists.

## Considerations

- A slip for other reasons (scope, defects of this project's own) is recorded and is not this bet.
- The gate measures elapsed time for a swarm of agents; it is not a statement about effort.
- The owner may re-plan earlier; the gate is a floor, not the only moment a re-plan can happen.

## References

§4 (BET-12), §40.5 (the schedule gate), §43.1, §43.4, §6.3.
