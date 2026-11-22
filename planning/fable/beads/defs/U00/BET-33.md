---
key: "BET-33"
title: "Decide BET-33: revalidation (\"which verdicts survive this edit\" is answered without ever returning a stale verdict)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "atelier", "chronicle", "frob-verbs", "solid", "bet", "gate"]
parent: "WP-63"
deps: ["N00-28", "BD-286", "BD-292", "BD-347", "STD-E2E"]
needs: ["§28.13: the frozen 30-amendment campaign, the declared share of unnecessary invalidations (at most 20% of unaffected verdicts) and the dependency grades 1 to 4", "§28.26: the reuse ledger with three columns never summed (exact reuse, carried evidence, avoided reacquisition)"]
related: ["BD-111", "BD-063", "BD-220", "BD-288", "BET-31", "NS-13", "N00-11", "N00-27"]
gates: ["GATE-ATEL-11", "GATE-AUGUR-12", "GATE-PHYLUM-05", "KILL-REUSE", "KILL-AUGUR-REUSE"]
plan_refs: ["§4 (BET-33)", "§40.5", "§0.8 (H-09)", "§3 (principle 8)", "§2.4 (lever 6)", "§28.23", "§25.27"]
---
## Background

**The bet.** In a design studio every result depends on inputs that an edit may change: the design, a card, a
scene, a task, a kernel version. After an edit the system must say which earlier verdicts still stand, which
are withdrawn, and what it would cost to restore them. The governing principle: a hash identifies an artifact;
only an implication licenses skipping a recheck. The bet is that this question can be answered usefully (not by
invalidating everything) and safely (never returning a verdict that the edit has made false). Tier: Solid.

**What the evidence supports.** Exact reuse of rollouts after a mass or geometry edit is at most about 10% of
compute, so reuse is not a speed lever. What survives is hygiene: a content-addressed memo graph for analyses,
step plans, kernels and scene compiles, and dependency-directed revalidation as an evidence rule. In a
plan-time pilot a hermetic dependency graph with full recomputation of dependents tied a more elaborate
semantic-receipt rule exactly (3 pair evaluations and 2 reductions each; every verdict matched fresh replay),
and omitting one declared dependency made both inherit a stale promotion.

## Scope

In: pre-registration, the readings at M1 (mission cells), M3 (dependency grades 1 and 2) and M4 (grades 3 and
4), verdict and decision record. Out: the revalidation engine, the memo graph, the invalidation contract.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. One false reused verdict on the sealed amendment suite, which kills the reuse rule that produced it.
   b. Unnecessary invalidations above the declared share.
2. **Competing arm.** A hermetic dependency graph with full recomputation of dependents. **Potency control.**
   Omitting one declared dependency must produce a stale verdict that the shadow recompute catches (the
   stale-key mutant).
3. **Deciding gates (TARGETs).**
   - GATE-ATEL-11 (Known answer; M1 for dependency grades on mission cells, M3, M4): on the frozen
     30-amendment campaign every decision equals full recomputation; zero unsafe reuse; the stale-key mutant is
     caught; unnecessary invalidations are at most 20% of unaffected verdicts, counted and reported.
   - GATE-AUGUR-12 (Known answer, M3): on the same campaign: zero incorrectly inherited verdicts; unnecessary
     invalidations at most 20%; a fixture with a cost-9 witness and a cost-5 minimum keeps the witness and
     withdraws the minimum (a witness survives an edit that leaves it valid; a minimality claim does not).
   - GATE-PHYLUM-05 (Known answer, M3): the invalidation contract across designs: the positive control reuses,
     each negative control invalidates or refuses, and 10⁴ random edit sequences give results equal to full
     recomputation.
4. **Kill gates and fallbacks.** KILL-REUSE (M4): any unsafe reuse in GATE-ATEL-11, or grade 3 saves less than a
   fifth of the recheck cost grade 2 leaves; grades 1 and 2 only. KILL-AUGUR-REUSE (standing from M3): one
   incorrectly inherited verdict from an implication rule; that rule is removed and reuse falls back to the
   dependency-keyed memo graph. The bet's own fallback: full recomputation per verb, with compile-only
   memoization.
5. **Downstream if falsified.** Amendments cost full recomputation, so the warm-campaign results of the
   ten-fold test (the crossover reuse count) worsen and are reported as measured; one missed withdrawal for a
   scoped claim makes a tournament stratum descriptive.
6. **Procedure.** Register the amendment campaign (sealed), the 20% share and the shadow-recompute sampling
   rate before the first amendment; read at M1, M3 and M4 exits; `docs/adr/bet-33-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the campaign's digest.
- [ ] Zero false reused verdicts across the 30 amendments (every reused verdict equals its fresh
      recomputation); the unnecessary-invalidation share against 20%.
- [ ] The stale-key mutant was caught by the shadow recompute.
- [ ] The cost-9 and cost-5 fixture behaved as stated; verdict and the two kill decisions recorded as ADRs.

## Test plan

### Unit tests
`bets.rs`: `bet33_zero_stale_14_percent_unnecessary_holds`; `bet33_one_stale_verdict_falsifies_a_and_kills_rule`
(the rule's identifier is listed for removal); `bet33_unnecessary_27_percent_falsifies_b`;
`bet33_grade3_saves_15_percent_fires_kill_reuse` (less than a fifth);
`bet33_stale_key_mutant_not_caught_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_33_decide.sh` replays three amendments of the campaign through `frob result revalidate` (or the
mission form at M1), recomputes every carried verdict from scratch and compares, runs the stale-key mutant
build, and recomputes the verdict. Logged per STD-E2E: amendment, verdicts carried, withdrawn and recomputed,
each carried verdict's fresh value and equality, the unnecessary share, the mutant's outcome; `summary` naming
the three gates and two kill gates.

### Known answers and negative controls
The stance amendment: at b = 1/4 m the commanded allocation (155, 135, 105, 5) N is feasible and the passive
springs give (175, 115, 85, 25) N against a 160 N rating, so the inner set is withdrawn (KA-022). Negative
controls: the stale-key mutant; an implication rule that carries a minimality claim across an edit that adds a
cheaper option.

## Considerations

- "Unnecessary" invalidation is a cost, never a correctness failure; the asymmetry is deliberate.
- Avoided reacquisition (training not redone) is counted under the "train few" lever and never added to exact
  reuse.

## References

§4 (BET-33), §40.5, §0.8 (H-09), §3, §2.4, §28.23, §25.27; GATE-ATEL-11; GATE-AUGUR-12; GATE-PHYLUM-05;
KILL-REUSE; KILL-AUGUR-REUSE; KA-022.
