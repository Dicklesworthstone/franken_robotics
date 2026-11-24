---
key: "BET-33"
title: "Decide BET-33: revalidation (\"which verdicts survive this edit\" is answered, never with a stale verdict)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "atelier", "chronicle", "frob-verbs", "solid", "bet", "gate"]
parent: "WP-63"
deps: ["BD-286", "BD-292", "BD-347", "BD-377", "N00-28", "N25a-19", "N28b-35", "N28b-36", "N28c-42", "STD-E2E"]
needs: []
related: ["N00-11", "N00-27", "N40-31", "NS-13"]
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
4), the bet's verdict and its decision record. Out, with the beads that have them: the dependency grades on
mission cells at M1 (the mission slice, BD-135 and BD-136); the revalidation engine with grades 1 and 2
(BD-292); grades 3 and 4 of `result.revalidate` (BD-347) and the M4 measurement of GATE-ATEL-11 with the costs
KILL-REUSE reads (N28b-35); the frozen 30-amendment campaign (BD-377); the invalidation contract across designs
and GATE-PHYLUM-05 (BD-286); the reuse ledger (N28c-42). Also out: the two kill-gate decisions. KILL-AUGUR-REUSE is decided, and its ADR and gate-register row
written, by N25a-19 (standing from M3); KILL-REUSE by N28b-36 (M4). Each applies its own trigger to the gate
receipts. This bead depends on both, reads their records, evaluates the bet's two falsifier clauses and writes
the bet verdict; it applies no kill trigger and writes no kill ADR.

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
- [ ] The cost-9 and cost-5 fixture behaved as stated; the bet verdict is recorded as an ADR
      (`docs/adr/bet-33-m4.md`), and that ADR cites the two kill decisions as recorded by their decision beads
      (the ADR of KILL-AUGUR-REUSE from N25a-19 and of KILL-REUSE from N28b-36), by identifier and hash. If
      either is not on record, the verdict is not filed.

## Test plan

### Unit tests
`bets.rs`: `bet33_zero_stale_14_percent_unnecessary_holds`; `bet33_one_stale_verdict_falsifies_a` (the bet's
record names the rule that produced it, as N25a-19's KILL-AUGUR-REUSE record lists it for removal; the removal
is applied there); `bet33_unnecessary_27_percent_falsifies_b`;
`bet33_reads_kill_reuse_record` (given a KILL-REUSE record in which grade 3 saved 15%, less than a fifth, and
the outcome is `fired`, the bet's record carries that outcome and the record's hash; the trigger arithmetic is
tested where it is applied, in N28b-36);
`bet33_stale_key_mutant_not_caught_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_33_decide.sh` replays three amendments of the frozen campaign through `frob result revalidate
<design> --between <before> <after> --grades 3 --key b33-rev-<n> --json` (a run verb, one key
per call; grade 3 under the session flag `frontier-revalidate-implication`, and `--grades 2` when N28b-36's
KILL-REUSE record fired), recomputes every carried verdict from scratch and compares, runs the stale-key control
in the Gauntlet form (`FROB_TEST_SEED=33 cargo test -p frob-conformance --test g1_known_answers --features
gauntlet-mutations -- GATE-ATEL-11`) and asserts it caught, reads the verdict of BET-33 from the bet register's
report (`cargo run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one
verdict rule; no second verdict rule in the script) and checks `docs/adr/bet-33-m4.md` against it. Logged per STD-E2E: amendment, verdicts carried, withdrawn and recomputed,
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
