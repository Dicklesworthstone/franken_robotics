---
key: "BET-16"
title: "Decide BET-16: inheritance (a child policy tuned from its parent needs a small share of from-scratch steps)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "frob-learn", "frontier", "bet", "gate"]
parent: "WP-61"
deps: ["BD-322", "N00-28", "N23a-30", "N38a-09", "STD-E2E"]
needs: []
related: ["BET-17", "N00-12", "N40-31", "NS-19"]
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
training runs, the audit twin; the kill-gate decision KILL-INHERIT (N23a-30, a dependency: it applies the
trigger, writes `docs/adr/ADR-KILL-INHERIT-M4.md`, and switches the default initialization of children
and the behaviour of `train.inherit`). This bead reads that record, evaluates the bet's three falsifier
clauses and writes the bet's verdict in `docs/adr/bet-16-m4.md`, which cites the kill record and does not
repeat or replace it.

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
   printed; topology edits start from the family policy or from scratch. The terms on which it is offered in
   that state are the ones N23a-30 sets for "not fired, gate not passed": `train.inherit` runs with the label
   `experimental`, a fresh arm is trained beside it, and no campaign uses it by default.
5. **Precondition: the pilot.** The Frontier pilot (N23b-20; reached through N23a-30) must be on record with its
   falsifier not met before the 12-pair experiment is funded: three base robots, 10 parameter edits of 10 to 30%
   and 5 single-limb edits, three arms (scratch, from parent, from prior) with three seeds, one chain of 10
   edits, about 70 GPU-hours; falsifier: median parameter-edit cost above 60% of steps, or the inherited arm
   more than 5% worse in over 20% of edits (the same thresholds as KILL-INHERIT). If the 12-pair experiment is
   not run (the pilot's falsifier holds, or its compute is not approved), N23a-30 decides on the pilot's record
   and labels the decision provisional; the bet's verdict then rests on that record and carries the same
   `provisional` label: `Falsified` if the pilot's falsifier held, `Untested` otherwise.
6. **Bound on the gain.** Whatever the result, the gain is bounded by the share of designs that are trained at
   all; it is counted once, under "train few", and never again as "reuse".
7. **Downstream if falsified.** Shortlisted designs are trained from the class prior or from scratch; the
   ceilings are recomputed with the measured cost; the second half of the scenario "a family policy walks an
   unseen design" (inheritance reaches the target in 12% of the steps, an illustrative figure) is not
   demonstrated.
8. **Procedure.** Register the 12 pairs (the 4 blind ones sealed), the return targets, the control's stopping
   rule and the chain protocol before any run; read at M4 exit; `docs/adr/bet-16-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with pair list, blind subset sealed, and the early-stopping rule for controls.
- [ ] Per pair: steps for the inherited arm, the raw-checkpoint arm and the early-stopped control; the median
      step fraction; the share of edits where the child is more than 5% worse.
- [ ] The ten-edit chain result with the audit twin's return, under both reset controls.
- [ ] The verdict recorded as an ADR that cites the KILL-INHERIT decision, which is on record as the ADR of its
      own bead (N23a-30); topology-edit figures reported.

## Test plan

### Unit tests
`bets.rs`: `bet16_median_0_14_holds`; `bet16_median_0_27_falsifies_a_without_kill`;
`bet16_reads_kill_inherit_fired_at_0_64` (N23a-30's record: fired; the bet carries its hash and names the
fallback; the trigger itself is tested in N23a-30's truth table); `bet16_no_better_than_raw_checkpoint_falsifies_b`;
`bet16_chain_minus_7_percent_falsifies_c`; `bet16_worse_in_3_of_12_with_kill_record_falsifies` (25% > 20%; read
from the kill record); `bet16_pilot_only_verdict_is_provisional`.

### End-to-end script and logging
`scripts/e2e/bet_16_decide.sh` recomputes the per-pair fractions from re-evaluation records (never from training
curves), the median and the chain comparison, reads the verdict of BET-16 from the bet register's report
(`cargo run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule)
and checks the ADR against both; under a fired kill gate it checks that `frob train inherit --child <child>
--donor <policy> --dry-run --key b16-probe --json` returns exit 5 with FROB-REF-724 naming
`docs/adr/ADR-KILL-INHERIT-M4.md` and the fallback (the call carries a key: `train.inherit` is a run verb, and
without one it is `FROB-REF-805`, exit 3). Logged per STD-E2E, newline-delimited JSON at
`target/e2e-logs/bet_16_decide/<utc>.ndjson` with a transcript: pair, edit class, steps per arm, fraction,
child return against control, chain returns per generation; `summary` naming GATE-CORTEX-13 and KILL-INHERIT.

### Known answers and negative controls
Thresholds 20%, 5%, 60%. Negative controls: a control run to completion instead of stopped at 95% (rejected);
a blind pair unsealed before its run; zero-weight and random policies and the wrong-donor control of BD-322 failing the return target.

## Considerations

- Steps and returns come from re-evaluation under a replayable profile; numbers seen during nondeterministic
  training are not evidence.
- Inheritance can bias a campaign toward the parent's neighbourhood; the campaign's diversity measure is
  reported beside the saving.

## References

§4 (BET-16), §40.5, §0.8 (H-21), §2.4, §23.23; GATE-CORTEX-13; KILL-INHERIT.
