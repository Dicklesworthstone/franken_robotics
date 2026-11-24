---
key: "BET-24"
title: "Decide BET-24: a requirement language as acceptance metric and falsification score, beside shaped rewards"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "arena", "frob-spec", "solid", "bet", "gate"]
parent: "WP-24"
deps: ["BD-121", "BD-147", "BD-255", "BD-256", "BD-257", "N00-28", "N00-37", "N23b-07", "N24a-15", "STD-E2E"]
needs: []
related: ["N00-12", "N40-31"]
gates: ["GATE-ARENA-03", "GATE-ARENA-09", "GATE-CORTEX-10", "KILL-SPEC-REWARD"]
plan_refs: ["§4 (BET-24)", "§40.5", "§0.8 (H-13)", "§24.17", "§23.23"]
---
## Background

**The bet.** A mission is stated as requirements: bounded-time statements over signals with units, such as
"within 20 s the base reaches the goal region and at no time does a foot slip more than 3 cm". A requirement
has a Boolean verdict and a signed *robustness* margin (how far the trace is from violating it). The bet is that
such requirements work as the acceptance metric (a design is accepted when its traces satisfy them) and as a
falsification score (the adversary minimizes robustness to find failures). Tier: Solid.

**What is not claimed.** Requirements do not replace shaped rewards: the planning evidence narrowed that idea.
Training keeps its reward terms; requirements judge the result. The reason to bet on requirements for
acceptance is that a reward threshold can be met by a policy that exploits the reward's shape while failing
the mission.

## Scope

In: pre-registration, readings at M1 (the monitors) and M3 (requirement writing; streaming monitors; acceptance
comparison), the bet's verdict and its decision record. Out: the language, the evaluators, the tasks, the
training pilot. Also out: the kill-gate decision KILL-SPEC-REWARD. N24a-15 reads the training pilot, applies the
pre-registered rule and writes that gate's ADR and register row. This bead depends on it, reads the record and
cites it in the bet verdict; it applies no kill trigger and writes no kill ADR.
Also out: running the acceptance comparison of clause (b). N00-37 registers it, runs both acceptance rules on
the design corpus, scores the accepted designs on the sealed variants and writes the sealed-variant ledger with
its summary record; this bead reads that summary. Its field `clause_b` decides the clause: `not lower` falsifies
it, `lower` does not, and `lower, not significant`, `not run: unreachable` or `void: control not caught` leave
the clause unmeasured, so the bet stays `Untested` on it.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. Requirement robustness disagrees with the reference monitor on any trace of the conformance corpus.
   b. Designs accepted by requirement verdicts fail the sealed variants at a rate no lower than designs accepted
      by a reward threshold.
2. **Competing arm.** Acceptance by reward threshold. **Potency control.** A policy known to exploit a shaped
   reward must pass the reward threshold and fail the requirement.
3. **Deciding gates (TARGETs).**
   - GATE-ARENA-03 (Known answer, M1): the three evaluators agree bit for bit on 10⁶ random traces per
     operator, including multi-rate signals and windows with rounding; every refusal of the requirement
     language (FROB-REF-520, -535, -536, -523, -538, -525, -526, -539 and -528) is triggered by its planted
     task file.
   - GATE-ARENA-09 (Design comparison, M3): requirement writing: on 50 scripted mission statements, each with a
     reference formula and 20 labelled trajectories the agent does not see, at most 20% of first-draft
     requirements mislabel a trajectory, and `task.check` rejects every draft that does.
   - GATE-CORTEX-10 (Known answer, M3): streaming past-time robustness equals offline evaluation on 10⁴ random
     traces; a mixed-unit fixture binds on the clearance in both unit systems; a future-time online reward is
     refused.
4. **Kill gate and fallback.** KILL-SPEC-REWARD (M3) concerns the optional use of a requirement term inside
   training: if, in the training pilot, the arm with the added requirement term does not improve success over
   the shaped reward across five seeds, requirements stay acceptance metrics and search scores only, the
   added-term option leaves the task schema, and margin-only training is offered only if it reaches 80% of the
   shaped arm's success. The bet's own fallback if clauses (a) or (b) hold: metric thresholds with declared
   comparators. Training keeps shaped reward terms in either case.
5. **Downstream if falsified.** Under (a): the language is not trusted for verdicts until the evaluators agree;
   acceptance uses metric thresholds. Under (b): the acceptance battery of the ten-fold test is defined on
   metric thresholds, and the adversary's falsification score falls back to metric margins.
6. **Procedure.** Register the conformance corpus, the 50 mission statements with sealed labelled trajectories,
   and the sealed variants before any agent drafts a requirement; read at M1 and M3 exits;
   `docs/adr/bet-24-m1.md` and `bet-24-m3.md`. The M1 reading (clause (a) on GATE-ARENA-03) is taken at the M1
   exit by the exit review (BD-147), from the gate rows through the bet register's verdict function (N00-28) and
   the exit state of N40-31; this bead closes at M3 and writes `bet-24-m1.md` from that recorded reading (it
   does not recompute it from later data) and `bet-24-m3.md` from the M3 reading.

## Acceptance criteria

- [ ] Pre-registered with corpus digests; labelled trajectories sealed from drafting agents.
- [ ] Zero disagreements among the three evaluators on 10⁶ traces per operator; streaming equals offline on
      10⁴ traces.
- [ ] The first-draft mislabel share (at most 20%) and the `task.check` rejection of every mislabelling draft.
- [ ] Failure rates on sealed variants for the two acceptance arms; the reward-exploiting policy passes the
      reward threshold and fails the requirement.
- [ ] The bet verdict is recorded as `docs/adr/bet-24-m1.md` and `bet-24-m3.md`; the M3 record cites the
      KILL-SPEC-REWARD decision as recorded by N24a-15 (by identifier and hash). If that decision is not on
      record, the M3 verdict is not filed.

## Test plan

### Unit tests
`bets.rs`: `bet24_zero_disagreements_and_lower_failure_rate_holds`; `bet24_one_trace_disagreement_falsifies_a`;
`bet24_failure_rates_equal_falsifies_b` (requirement-accepted 12% against reward-accepted 12%: "no lower");
`bet24_exploit_policy_passes_requirement_is_untested` (the control failed);
`bet24_reads_kill_spec_reward_record` (given a KILL-SPEC-REWARD record in which the added term gave no gain and
the outcome is `fired`, the bet's record carries that outcome and the record's hash; the rule itself is tested
where it is applied, in N24a-15).

### End-to-end script and logging
`scripts/e2e/bet_24_decide.sh` reads the recorded values of GATE-ARENA-03, GATE-ARENA-09 and GATE-CORTEX-10
from the gate register; re-runs the evaluator agreement in the Gauntlet form (`FROB_TEST_SEED=24 cargo test -p
frob-conformance --test g1_known_answers -- GATE-ARENA-03`, the three evaluators on the row's random traces),
asserting zero disagreements; evaluates the exploit policy's recorded run with `frob task verdict <run> --task
<TaskId> --json` (the requirement rule) and its recorded return against the reward threshold (the reward rule);
recomputes the two failure rates from the sealed-variant ledger summarized by N00-37; reads the verdict of
BET-24 from the bet register's report (`cargo run -p frob-conformance --example bets -- status --json`, example
`bets` of N00-28, the one verdict rule) and checks both ADRs against it; on a mismatch it logs the first
differing field with the paths of the gate rows and the ADR. Logged per STD-E2E: operator, traces, disagreements, the two
acceptance decisions for the exploit policy, failure rates with intervals, draft mislabel share; `summary`
naming the three gates and KILL-SPEC-REWARD.

### Known answers and negative controls
A hand-checkable robustness: for "always within [0, 2] s the clearance is at least 0.05 m", written with the
scale 0.05 m, on a trace with minimum clearance 0.02 m, the margin is (0.02 − 0.05)/0.05 = −0.6 and the verdict is
false; in millimetres the same trace gives (20 − 50)/50 = −0.6 and the same verdict. The raw differences −0.03 m
and −30 mm are the shortcut that the declared scale removes (ADR-ARENA-03): under raw composition the binding
atom can change with the unit system, which the mixed-unit fixture (KA-581) shows and the scale prevents. Negative controls: the reward-exploiting policy; a
requirement whose window reaches past the horizon (refused by `task.check`); a future-time reward requested
online.

## Considerations

- A requirement is only as good as its author's formalization; the writing gate measures that directly, and
  `task.check`'s labelled examples and mutation check are the guard.
- Robustness margins are dimensionless: every atom divides by its declared scale before any minimum or maximum
  (ADR-ARENA-03), so margins compose across units and the binding atom does not depend on the unit system; an
  atom with no scale that is not declared exact is refused (FROB-REF-520).

## References

§4 (BET-24), §40.5, §0.8 (H-13), §24.17, §23.23; GATE-ARENA-03, -09; GATE-CORTEX-10; KILL-SPEC-REWARD.
