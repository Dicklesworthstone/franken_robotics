---
key: "BET-32"
title: "Decide BET-32: rare-failure bounds (bounded-weight importance sampling refutes, tightens; never cheap absence)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "solid", "bet", "gate"]
parent: "WP-55"
deps: ["BD-276", "BD-277", "N00-28", "N26a-30", "N26a-32", "STD-E2E"]
needs: []
related: ["GB-32", "N00-11", "N00-20", "N40-31"]
gates: ["GATE-NEMESIS-04", "GATE-NEMESIS-05", "KILL-RARE"]
plan_refs: ["§4 (BET-32)", "§40.5", "§0.8 (H-07)", "§2.4 (lever 4)", "§26.15"]
---
## Background

**The bet.** A finalist design must often be shown to fail rarely: say with probability below 10⁻⁴ per episode.
Crude sampling needs tens of thousands of failure-free episodes for that. Importance sampling draws worlds from
a *proposal* tilted toward failures and reweights; if the proposal is frozen before the confirming draws and
mixed with the nominal distribution (a *defensive mixture*, nominal weight ε), every weight is at most 1/ε and
the resulting bound is valid. The bet is specific about what this buys: once failures have been found it
refutes a failure-rate target, and tightens an estimate, with fewer episodes than crude sampling. It does not
certify absence more cheaply: a failure mode the proposal misses costs up to 1/ε times crude sampling. Tier:
Solid for the bounded-weight estimator.

**Two lanes, never mixed.** *Exploration* (splitting, adaptive search) finds failures and yields Estimated rates
only. *Certification* uses a frozen proposal with bounded weights, or crude sampling with an exact binomial
bound.

## Scope

In: pre-registration; the measurement of clauses (b) and (c), which no gate records (this bead's script runs the
crude and the bounded-weight arm on a 10⁻⁴ trap); the M3 reading, verdict and decision record. Out, with the
beads that have them: the certification lane `nemesis.bound` with the defensive mixture and GATE-NEMESIS-04,
whose record supplies clause (a) (row 1: coverage over 10⁴ synthetic studies) and clause (d) (row 2: the
mixture's confidence sequence covers a two-sided tail under a proposal shifted to one side only, so the mode
outside the proposal is not reported absent) (BD-276, with its end-to-end script N26a-30); the exploration lane (multilevel splitting) and
GATE-NEMESIS-05 (BD-277); the decision KILL-RARE, its ADR and its fallback (N26a-32), which this bead reads and
cites and never re-decides.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. On fixtures with a planted failure rate, the bound's coverage is below nominal.
   b. The effective sample size falls below 30% on the reference family.
   c. On a fixture whose planted rate of 10⁻⁴ is above the target, the episodes needed for a lower confidence
      bound to exceed the target are not at least 3× fewer than crude sampling needs (published range 2–20×).
   d. A planted second failure mode outside the proposal is reported as absent.
2. **Competing arm.** Crude Monte Carlo with an exact binomial bound. **Potency control.** A planted rate; a
   mis-centred proposal under which the defensive mixture must still cover; non-nominal episodes offered to the
   certification lane, which must be refused.
3. **Deciding gates (TARGETs).**
   - GATE-NEMESIS-04 (Known answer, M3): over 10⁴ synthetic studies with known p the 95% upper bound covers p
     in at least 95%; the defensive mixture covers under a mis-centred proposal; non-nominal episodes are
     refused.
   - GATE-NEMESIS-05 (Design comparison, M3): exploration on planted failures of probability 10⁻⁴ and 10⁻⁵ (one
     with two separate modes), equal episode budget, 30 repetitions: at least 10 times fewer episodes than
     crude sampling at 10⁻⁵; both modes found in at least 80% of repetitions.
4. **Kill gate and fallback.** KILL-RARE (M3): in the exploration pilot the gain is below 10 times at 10⁻⁵, or
   the coverage of any interval offered as a bound is below 85%, or a planted mode is missed in more than 20%
   of repetitions. Fallback: crude nominal sampling with the binomial bound alone; splitting is removed;
   tilting is kept only inside the adversarial search. The bet's own fallback: crude sampling alone; splitting
   remains exploration and yields Estimated rates only.
5. **Downstream if falsified.** Failure-rate claims cost what crude sampling costs (299 failure-free episodes
   for a 1% bound at 95% confidence; about 30,000 for 10⁻⁴); a certified rate of 10⁻⁶ is out of reach and is
   reported as Estimated; qualification campaigns budget accordingly.
6. **Procedure.** Register the fixtures with their planted rates and modes, ε, the nominal coverage and the 3×
   factor before any draw; proposals are frozen (digest) before confirming draws; read at M3 exit;
   `docs/adr/bet-32-m3.md`.

## Acceptance criteria

- [ ] Pre-registered; each certification run's proposal digest predates its confirming draws.
- [ ] Coverage over 10⁴ synthetic studies (at least 95%); effective sample size share on the reference family
      (at least 30%); every weight at most 1/ε.
- [ ] The episode ratio against crude sampling on the 10⁻⁴ refutation fixture (at least 3×).
- [ ] The second mode outside the proposal was not reported absent (the result says the proposal does not cover
      it); the verdict recorded in `docs/adr/bet-32-m3.md`, citing the KILL-RARE decision on record by N26a-32
      (by identifier and hash; with it missing the verdict is not filed).

## Test plan

### Unit tests
`bets.rs`: `bet32_coverage_0_957_ess_0_41_ratio_4_2_holds`; `bet32_coverage_0_93_falsifies_a`;
`bet32_ess_0_22_falsifies_b`; `bet32_ratio_2_1_falsifies_c`; `bet32_second_mode_reported_absent_falsifies_d`;
`bet32_weight_above_one_over_epsilon_rejected`; `bet32_proposal_changed_after_draws_rejected`.

### End-to-end script and logging
`scripts/e2e/bet_32_decide.sh` reads clauses (a) and (d) and the refusal row from the GATE-NEMESIS-04 record
(`cargo run -p frob-conformance --example registry_report -- gates --chapter 26 --milestone M3 --json`) and
measures clauses (b) and (c) on a T4 trap of the planted registry (N26a-04: probability exactly 10⁻⁴ on the
reference quadruped's task) against the registered target p⋆ below 10⁻⁴, in the registered number of paired
repetitions. The crude arm is `frob nemesis bound <reference quadruped> --task <T4 task> --tier batch-cpu
--target <p⋆> --mode certify --finalist --key b32-crude-<i> --json` (nominal sampling only; it ends `refuted` when
its lower bound passes p⋆; its budget is registered large enough that N26a-22's `choose_design` picks
`sequential`, so that it stops at refutation); the bounded-weight arm is the same call with `--mode refute
--shift <the registered shift> --epsilon 1/10 --key b32-bw-<i>`. Every call carries `--tier`, which has no
default, `--finalist`, without which N26a-26's finalists-only rule refuses a fixture candidate, and its own key,
so that the two arms are two runs and not one stored receipt. The script asserts every recorded weight at most
1/ε = 10, computes the ratio of the arms' mean episodes `n` to `refuted` (clause (c), against 3) and the
bounded-weight runs' effective sample size as a share of n from their `weights` blocks (clause (b), against
30%), reads the verdict of BET-32 from the bet register's report (`cargo run -p frob-conformance --example bets --
status --json`, example `bets` of N00-28, the one verdict rule) and checks `docs/adr/bet-32-m3.md` against it.
Logged per STD-E2E: fixture, planted rate, proposal digest and its
timestamp against the first confirming draw, maximum weight against 1/ε, effective sample size, bounds,
episodes per arm; `summary` naming GATE-NEMESIS-04, GATE-NEMESIS-05 and KILL-RARE.

### Known answers and negative controls
The Gaussian tail P(Z > 4) = 3.167 × 10⁻⁵; a random-walk level-crossing probability of 1.504 × 10⁻⁴; the
zero-failure rule: n failure-free episodes give a 95% upper bound of 1 − 0.05^(1/n) (n = 299 gives 0.00997;
⌈ln 0.05 ÷ ln(1 − 10⁻⁴)⌉ = 29,956 episodes for a 10⁻⁴ bound, the "about 30,000" above).
Negative controls: the mis-centred proposal; a proposal adapted during the confirming draws; the planted second
mode; a fork with no fresh randomness.

## Considerations

- A valid bound from importance sampling is a statement about the nominal distribution; the world family must
  be the mission's declared one.
- Exploration results are never offered as bounds, whatever their apparent precision.

## References

§4 (BET-32), §40.5, §0.8 (H-07), §2.4, §26.15; GATE-NEMESIS-04, -05; KILL-RARE.
