---
key: "BET-22"
title: "Decide BET-22: minimal counterexample worlds (adversarial search beats randomization; shrink to ≤ 3 edits)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "frontier", "bet", "gate"]
parent: "WP-55"
deps: ["BD-271", "BD-273", "BD-274", "N00-28", "N26a-07", "N26a-11", "N26a-12", "N26a-35", "N38a-09", "STD-E2E"]
needs: []
related: ["BET-24", "BET-32", "BET-37", "N00-11", "N00-20", "N40-31", "N40-32", "NS-15"]
gates: ["GATE-NEMESIS-02", "GATE-NEMESIS-03", "KILL-NEMESIS", "KILL-MCW"]
plan_refs: ["§4 (BET-22)", "§40.5", "§2.4 (lever 4)", "§0.8 (H-07)", "§26.15", "§43.12"]
---
## Background

**The bet.** Domain randomization reports a failure tail and leaves the agent reading logs to guess why. The
adversary searches instead: inside a declared *legality set* (worlds that are physically possible and within
the mission's stated envelope) it looks for worlds that break a design; it confirms with a *solvability
witness* that the candidate's own body and actuators could have completed the mission there (so the failure is
the design's or the controller's, not an impossible world); and it shrinks the failing world to a *minimal
counterexample world*: the fewest edits against the nominal scene that still produce the failure, removing any
one of which removes it. The bet is that this finds failures faster than randomization and yields small,
reproducible, legitimate counterexamples. Tier: Frontier.

**What the evidence supports.** Published measured gains in finding failures are 2–20×. The gain is in finding
failures and refuting a failure-rate target; certifying absence is never cheaper than crude sampling, and a
failure mode the search's proposal misses can cost far more.

## Scope

In: pre-registration, the M3 reading, verdict and decision record. Out: the legality set, the oracle, the
search, the shrinker, the artifact guard. Also out: the decisions of the two kill gates. KILL-NEMESIS is decided
by N26a-12 and KILL-MCW by N26a-35 (both dependencies): each applies its trigger literally and writes its own
record (`docs/adr/ADR-KILL-NEMESIS-M3.md`, `docs/adr/ADR-KILL-MCW-M3.md`: a kill-gate decision is
`docs/adr/ADR-<gate-name>-<milestone>.md`). This bead reads those two records, evaluates
the bet's falsifier clauses and writes the bet verdict (`docs/adr/bet-22-m3.md`), which cites them. Where a line
below says a kill decision is recorded, it means on record by those beads; where a test of the bet's evaluator
says a kill gate fires, the evaluator is reproducing their rule and must agree with their record.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. Planted traps of probability at most 3 × 10⁻⁵ are found less than 3× faster than by uniform randomization.
   b. Fewer than 70% of reported counterexamples carry a solvability witness.
   c. A shrunk world fails to reproduce bit for bit.
2. **Competing arms.** Uniform randomization at equal compute; a scenario-language generator given the same
   monitors. **Potency controls.** Planted traps; an unsolvable world, which must be rejected and not counted as
   a counterexample; planted illegal edits, which must be refused.
3. **Deciding gates (TARGETs).**
   - GATE-NEMESIS-02 (Design comparison, M3): on planted traps of nominal probability at most 3 × 10⁻⁵ with 30
     paired repetitions: a first promoted failure with at least 3 times fewer simulated steps than card-uniform
     randomization at a 95% lower bound; at least 70% of reported counterexamples `solvable-causal` or
     `plant-feasible`.
   - GATE-NEMESIS-03 (Known answer, M3): shrinking recovers planted causes exactly in at least 95 of 100
     trials; every minimal-counterexample file replays with identical digests in 100 of 100; on unplanted
     failures the share of worlds with at most three edits is recorded.
4. **Kill gates and fallbacks.** KILL-NEMESIS (M3): GATE-NEMESIS-02 fails after one repair cycle; the search
   methods are removed and the adversary keeps the legality profile, the oracle, the guard and shrinking,
   applied to failures that card randomization finds. KILL-MCW (M3): on unplanted failures fewer than half of
   shrunk worlds have at most three edits, or fewer than 90% replay; full failing worlds ship with replay and
   single-edit contrasts ranked by margin, and the file format stays. The bet's own stated fallback:
   importance-weighted tail sampling, with failing worlds reported unshrunk.
5. **Downstream if falsified.** The "find failures instead of waiting for them" lever shrinks to randomization
   plus shrinking and the guard; the retrofit search still works on whatever failing worlds exist; the expected
   range for failure discovery (2 to 20 in episodes to a planted failure) is replaced by the measured figure.
6. **Procedure.** Register the trap fixtures, their nominal probabilities, the 3× factor, the 70% share and the
   30 paired seeds before the search runs; read at M3 exit; `docs/adr/bet-22-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with trap fixtures and seeds; the randomization arm runs at equal simulated-step budget.
- [ ] The paired ratio of steps to first promoted failure with its 95% lower bound against 3; the share of
      counterexamples with a witness against 70%.
- [ ] Shrinking: exact recovery count out of 100; replay count out of 100; the share of unplanted failures with
      at most three edits.
- [ ] The unsolvable world was rejected and illegal edits refused.
- [ ] The verdict is recorded as `docs/adr/bet-22-m3.md` by a session that did not implement the adversary, citing
      by identifier and hash the KILL-NEMESIS record of N26a-12 and the KILL-MCW record of N26a-35, whatever their
      outcomes; this bead writes no kill ADR, and with either record missing the verdict is not filed.

## Test plan

### Unit tests
`bets.rs`: `bet22_ratio_lb_3_6_and_witness_78_percent_holds`; `bet22_ratio_lb_2_4_falsifies_a`;
`bet22_witness_share_0_64_falsifies_b`; `bet22_one_non_reproducing_world_falsifies_c`;
`bet22_unplanted_three_edit_share_0_42_fires_kill_mcw`; `bet22_unsolvable_world_counted_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_22_decide.sh` recomputes the paired ratio from the ledger, replays every recorded
minimal-counterexample file with `frob sim replay <file>.fmcw --key b22-replay-<n> --json` and compares digests,
replays for three sample worlds each one-edit-removed neighbour the file lists in `neighbours_passing` (`frob sim
replay w:<hash> --key b22-nb-<n> --json`) and checks the failure is absent (1-minimality), reads the verdict of
BET-22 from the bet register's report (`cargo run -p frob-conformance --example bets -- status --json`, example
`bets` of N00-28, the one verdict rule) and checks the ADR against it. Logged per STD-E2E: trap,
repetition, steps per arm, ratio and bound, witness labels, replay digests, edit counts; `summary` naming the
gates and kill gates.

### Known answers and negative controls
Expected search effort under uniform randomization for a trap of probability p is 1/p episodes: about 33,000 at
3 × 10⁻⁵. Negative controls: the unsolvable world; planted illegal edits; a fork with no fresh randomness
(refused); a shrinker that returns a non-minimal world (removing an edit leaves the failure: fails the
1-minimality check).

## Considerations

- A counterexample must be legal and solvable to count; otherwise the search optimizes for impossible worlds.
- Every counterexample joins the regression battery and the hard-world set; that side effect holds under every
  fallback.

## References

§4 (BET-22), §40.5, §2.4, §0.8 (H-07), §26.15, §43.12; GATE-NEMESIS-02, -03; KILL-NEMESIS; KILL-MCW.
