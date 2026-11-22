---
key: "BET-23"
title: "Decide BET-23: interventional autopsy is stable enough to act on (planted causes, flip rates, repair turns)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "autopsy", "frob-autopsy", "solid", "bet", "gate"]
parent: "WP-56"
deps: ["N00-28", "BD-279", "BD-280", "BD-375", "STD-E2E"]
needs: ["§27.19: attribution in single precision on the originating tier and its admission rule (GATE-AUTOPSY-09)"]
related: ["BD-205", "BD-281", "BD-282", "BD-283", "BD-284", "BET-10", "BET-02", "NS-16", "N00-11"]
gates: ["GATE-AUTOPSY-03", "GATE-AUTOPSY-08", "KILL-AUTOPSY"]
plan_refs: ["§4 (BET-23)", "§40.5", "§2.4 (lever 7)", "§0.8 (H-08)", "§27.17", "§43.12"]
---
## Background

**The bet.** When an episode fails, an agent given logs orders the anomalies by time and guesses. The debugger
does an experiment instead: it forks the snapshot taken before the failure under typed interventions (lift a
torque limit, restore nominal friction, remove a packet delay), singly and in pairs, on identical random
streams, and reports which change flips the outcome. A *placebo fork* (no intervention) must replay the failure
bit for bit; perturbed forks at round-off scale measure how often the outcome flips by chance, which is the
*flip rate*. A factor whose intervention prevents the failure far more often than the placebo flips is a
but-for cause. The bet is that such attributions are right on planted causes, stable under tiny perturbations,
and useful: an agent with them repairs faster. Tier: Solid.

**What the evidence supports.** No published figure for agent repair turns exists. Contact-rich dynamics
amplify round-off: at a published exponent, single-precision round-off stays below 4 × 10⁻⁵ of state scale over
fork windows up to about 1 s and reaches 2% at 2 s, which is why double precision is the default for
attributing one failure and why an episode with a high placebo flip rate is labelled `chaotic: unattributable`
and given no cause.

## Scope

In: pre-registration, readings at M3 (planted causes) and M4 (agent repair turns), verdict and decision record.
Out: forks, interventions, contrasts, the agent study.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. Top-1 accuracy on planted causes is below 90%.
   b. The placebo flip rate exceeds 0.2 on more than half of the reference failures, so that most are
      unattributable.
   c. An agent with autopsy needs more than one third of the repair turns of an agent with logs.
2. **Competing arms.** First-anomaly ordering; an agent with logs only. **Potency controls.** The placebo fork,
   which must replay bit for bit; planted causes; a planted chaotic failure and a planted no-cause failure, which
   must name no factor.
3. **Deciding gates (TARGETs).**
   - GATE-AUTOPSY-03 (Known answer, M3): on planted single causes, the planted factor ranks first in at least
     90% of 200 failures in double precision, and in the single-precision lane for windows of at most 1 s; the
     top factor changes in at most 30% of cases under a 10⁻⁷ perturbation; the agreement between ten
     one-at-a-time forks and the full coalition table is recorded.
   - GATE-AUTOPSY-08 (Design comparison, M4): on the planted-defect designs of the corpus, agents given the
     debugger reach an accepted repair in at most one third of the serial turns of agents given logs and
     `explain` alone, pre-registered, with an anytime-valid verdict.
4. **Kill gate and fallback.** KILL-AUTOPSY (M3): GATE-AUTOPSY-03 fails after one repair cycle (planted-cause
   accuracy below 90%, or the top factor changes in more than 30% of cases at 10⁻⁷). Fallback: single-factor
   paired re-runs from the episode start, with first-anomaly ordering labelled Estimated; coalitions and
   Shapley summaries are removed.
5. **Downstream if falsified.** Under (a) or (b): attribution output is labelled Estimated and repairs are
   suggestions; the "explain by intervention" lever is removed from the thesis arithmetic. Under (c) alone: the
   mechanism ships as a diagnostic with no speed claim, and the reduction of the agent surface (BET-10) decides
   whether its tools stay visible by default.
6. **Procedure.** Register the 200 planted failures, the perturbation scale, the reference failure set for the
   flip-rate clause and the agent-study design before any fork; read at M3 and M4 exits;
   `docs/adr/bet-23-m3.md`, `bet-23-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the planted set and the reference failure set.
- [ ] Top-1 accuracy out of 200; the share of cases whose top factor changes under 10⁻⁷; the share of reference
      failures with placebo flip rate above 0.2.
- [ ] Placebo forks replayed bit for bit in every case used; the chaotic and no-cause plants named no factor.
- [ ] The agent study's turn ratio with its bound against one third; verdict and KILL-AUTOPSY decision
      recorded.

## Test plan

### Unit tests
`bets.rs`: `bet23_accuracy_187_of_200_holds_a` (93.5%); `bet23_accuracy_176_of_200_falsifies_a` (88%);
`bet23_placebo_flip_over_0_2_on_60_percent_falsifies_b`; `bet23_turn_ratio_0_41_falsifies_c`;
`bet23_top_factor_changes_in_34_percent_fires_kill`; `bet23_placebo_not_bitwise_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_23_decide.sh` re-runs five planted failures through `frob run fork` and `frob autopsy
attribute`, checks the placebo digests equal the original, recomputes accuracy and stability from the ledger
and the turn ratio from the agent study, and recomputes the verdict. Logged per STD-E2E: failure, planted
factor, ranked factors with prevention rates and placebo flip rates, digests, turn counts per arm; `summary`
naming the gates and KILL-AUTOPSY.

### Known answers and negative controls
Illustrative reading of a report: an intervention that prevents the fall in 94% of perturbed forks against a
placebo flip rate of 3% is a but-for cause; one that does so in 8% is not. Negative controls: the planted
chaotic failure (the doubling map) must yield `chaotic: unattributable`; a design-parameter intervention
requested mid-episode must be refused and re-run from the episode start (winding temperature depends on the
whole history); a checkpoint with its packet queue stripped must be refused.

## Considerations

- Attribution requires a controlled determinism class with batch-composition invariance current; under an
  uncontrolled class it is refused.
- A stable attribution on a planted cause does not guarantee correctness on natural failures; reports carry
  the stability figure, not a certainty.

## References

§4 (BET-23), §40.5, §2.4, §0.8 (H-08), §27.17, §43.12; GATE-AUTOPSY-03, -08; KILL-AUTOPSY.
