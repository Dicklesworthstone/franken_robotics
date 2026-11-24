---
key: "BET-23"
title: "Decide BET-23: interventional autopsy is stable enough to act on (planted causes, flip rates, repair turns)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "autopsy", "frob-autopsy", "solid", "bet", "gate"]
parent: "WP-56"
deps: ["BD-279", "BD-280", "BD-319", "BD-375", "N00-28", "N27b-04", "N27b-11", "N27b-24", "N27b-28", "STD-E2E"]
needs: []
related: ["N00-11", "N40-31", "N40-32", "NS-16"]
gates: ["GATE-AUTOPSY-03", "GATE-AUTOPSY-08", "KILL-AUTOPSY"]
plan_refs: ["§4 (BET-23)", "§40.5", "§2.4 (lever 7)", "§0.8 (H-08)", "§27.17", "§43.12"]
---
## Background

**The bet.** When an episode fails, an agent given logs orders the anomalies by time and guesses. The debugger
does an experiment instead: it forks the snapshot taken before the failure under typed interventions (lift a
torque limit, restore nominal friction, remove a packet delay), singly and in pairs, on identical random
streams, and reports which change flips the outcome. The *null fork* (the empty intervention) must replay the
failure bit for bit; *placebo forks* (no intervention, one state coordinate perturbed at round-off scale, a
relative 10⁻¹²) measure how often the outcome flips by chance, which is the *placebo flip rate*. (The bet's row
in the bets table says "the placebo fork, which must replay bit for bit"; the Autopsy beads keep the two forks
apart under the names used here, because a placebo fork is perturbed on purpose and cannot replay bit for
bit.) A factor whose intervention prevents the failure far more often than the placebo flips is a
but-for cause. The bet is that such attributions are right on planted causes, stable under tiny perturbations,
and useful: an agent with them repairs faster. Tier: Solid.

**What the evidence supports.** No published figure for agent repair turns exists. Contact-rich dynamics
amplify round-off: at a published exponent, single-precision round-off stays below 4 × 10⁻⁵ of state scale over
fork windows up to about 1 s and reaches 2% at 2 s, which is why double precision is the default for
attributing one failure and why an episode with a high placebo flip rate is labelled `chaotic: unattributable`
and given no cause.

## Scope

In: pre-registration, readings at M3 (planted causes) and M4 (agent repair turns), verdict and decision record.
Out, with the beads that have them: forks and interventions (`autopsy.intervene`, BD-279); contrasts, flip rates
and the GATE-AUTOPSY-03 record (`autopsy.attribute`, BD-280); the single-precision lane and its end-to-end script
(N27b-24, N27b-28); the agent study and the GATE-AUTOPSY-08 record (BD-375); the KILL-AUTOPSY decision (N27b-11),
which this bead reads and cites and never re-decides.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. Top-1 accuracy on planted causes is below 90%.
   b. The placebo flip rate exceeds 0.2 on more than half of the reference failures, so that most are
      unattributable.
   c. An agent with autopsy needs more than one third of the repair turns of an agent with logs.
2. **Competing arms.** First-anomaly ordering; an agent with logs only. **Potency controls.** The null fork,
   which must replay bit for bit (the row's "placebo fork"); planted causes; a planted chaotic failure and a
   planted no-cause failure, which must name no factor.
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
   `docs/adr/bet-23-m3.md`, `bet-23-m4.md`. The M3 reading is taken at the M3 exit by the exit review (BD-319),
   from the gate rows through the bet register's verdict function (N00-28) and the exit state of N40-31; this
   bead closes at M4 and writes `bet-23-m3.md` from that recorded reading (it does not recompute it from later
   data) and `bet-23-m4.md` from the M4 reading. The reference failure set of clause (b) is the committed
   population of the null-fork gate: 1,000 recorded failing worlds of the reference quadruped, drawn by address
   and not selected by margin (manifest of N27a-06). Its figure is measured at M3 by the script of N27b-04,
   which runs the null fork and 64 placebo forks on every one of them: `share_rho0_above_0.2` = the worlds with
   more than 12 placebo flips of 64, over the worlds read in double precision. Clause (b) holds when that share
   is above one half. This bead reads the figure from that script's log by receipt hash and measures nothing
   of its own for it.

## Acceptance criteria

- [ ] Pre-registered with the planted set and the reference failure set.
- [ ] Top-1 accuracy out of 200; the share of cases whose top factor changes under 10⁻⁷; the share of reference
      failures with placebo flip rate above 0.2.
- [ ] Null forks replayed bit for bit in every case used; the chaotic and no-cause plants named no factor.
- [ ] The agent study's turn ratio with its bound against one third; verdict recorded in `bet-23-m3.md` and
      `bet-23-m4.md`, citing the KILL-AUTOPSY decision on record by N27b-11 (by identifier and hash).

## Test plan

### Unit tests
`bets.rs`: `bet23_accuracy_187_of_200_holds_a` (93.5%); `bet23_accuracy_176_of_200_falsifies_a` (88%);
`bet23_placebo_flip_over_0_2_on_60_percent_falsifies_b`; `bet23_turn_ratio_0_41_falsifies_c`;
`bet23_reads_kill_autopsy_top_factor_34_percent` (N27b-11's record: fired after the repair cycle because the top
factor changed in 34% > 30% of cases at 10⁻⁷ → `Falsified`, the record's hash carried; the trigger is applied
there); `bet23_null_fork_not_bitwise_is_untested`; `bet23_reference_share_read_from_n27b04` (the share 0.31
of the recorded log → clause (b) does not hold; 0.52 → it holds).

### End-to-end script and logging
`scripts/e2e/bet_23_decide.sh` re-runs five planted failures through `frob run fork`, `frob autopsy intervene`
(each `run` call with `--key bet23-<failure>-<step>`; the children are read by address where the gate's run
left them) and `frob autopsy attribute`, checks the null fork's digests equal the original, recomputes accuracy
and stability from the gate's records, reads the reference share of clause (b) from the log of N27b-04 and the
turn ratio from the agent study, reads the verdict of BET-23 from the bet register's report (`cargo run -p
frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule) and checks
both ADRs against it. Logged per STD-E2E: failure, planted
factor, ranked factors with prevention rates and placebo flip rates, digests, the reference share against 0.5,
turn counts per arm; `summary` naming the gates and KILL-AUTOPSY.

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
