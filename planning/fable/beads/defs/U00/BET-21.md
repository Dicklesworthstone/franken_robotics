---
key: "BET-21"
title: "Decide BET-21: blinded qualification (error control holds across an agent's adaptive retries)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "arena", "frob-arena", "solid", "bet", "gate"]
parent: "WP-54"
deps: ["BD-259", "BD-260", "BD-261", "N00-28", "STD-E2E"]
needs: []
related: ["BET-24", "N00-27", "N40-31", "N40-32", "NS-03", "NS-17"]
gates: ["GATE-QUAL-01", "GATE-QUAL-02"]
plan_refs: ["§4 (BET-21)", "§40.5", "§3 (principle 9)", "§24.17", "§43.6", "§43.12"]
---
## Background

**The bet.** An agent that submits fifty candidates against the same held-out scenes until one passes has made
fifty comparisons, each valid alone, and a campaign that is not: the chance that some candidate passes by luck
grows with every retry. The qualification lane is the design's answer. A request pre-commits the candidate, the
baseline, the mission and the metrics. Submission i runs on fresh sealed world clusters at level
α_i = α / [i·(i + 1)], a series whose partial sums are α·m/(m + 1) and whose total is α. The discovery lane
cannot read the sealed worlds, and only the decision and an aggregate receipt come back until the campaign
closes. The bet is that this holds the false-promotion rate at α however adaptively the agent retries, while
keeping enough power to promote real gains. Tier: Solid.

**Why it matters.** "Qualified" is the word the product uses for an accepted design, and the ten-fold test's
acceptance rests on it. If the lane leaked or lost its error control, nothing could be called qualified.

## Scope

In: pre-registration, the M3 reading, verdict and decision record. Out, with the beads that have them: the
blinded lane `eval.qualify` and its receipt (BD-259); the 2,000 simulated campaigns and the GATE-QUAL-01 record
(BD-260); the confirmation firewall and the GATE-QUAL-02 record (BD-261). BET-21 has no kill gate: its written
consequence (below) is applied by this bead's decision record.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. The false-promotion rate exceeds α under 500 adaptive retries on no-signal worlds.
   b. Power on planted gains falls below 0.8.
2. **Potency control.** The unblinded procedure (repeated comparison on the same held-out worlds at a constant
   level) must show an inflated error rate on the same no-signal worlds; the certifier's own test requires a
   false promotion in more than 99% of such campaigns. That shows the test can fail. The count and the control
   are read on the gate's absolute no-signal world (N24b-18's `absolute-above`, `synthetic:null-absolute/1`;
   control expected 0.9987), the one world where a test's size is near its level; the two paired no-signal
   worlds are reported beside it against their computed expectations, since the conservative paired rule
   cannot reach 99% there (BD-260).
3. **Deciding gates (TARGETs).**
   - GATE-QUAL-01 (Known answer, M3): over 2,000 simulated no-signal campaigns at α = 0.05, each with 500
     adaptive retries by a scripted adversary that sees every receipt and all discovery data: a false promotion
     in at most 123 campaigns (an exact one-sided binomial test of a rate at or below 0.05 over 2,000 campaigns
     rejects at the 1% level from 124; the expected count at exactly α is 100); a
     planted true gain of the declared size promoted within 10 requests in at least 80% of campaigns.
   - GATE-QUAL-02 (Known answer, M3): the firewall: 10⁴ fuzzed read attempts across every verb return no sealed
     field; a planted leak path is found by the audit; receipts contain only the listed fields.
4. **No kill gate; the fallback is written.** Fixed-n exact confirmation with a hard cap on submissions per
   campaign. If GATE-QUAL-01 fails, nothing is called qualified, and confirmation falls back to fixed-n exact
   studies with the cap before any tournament is scheduled.
5. **Downstream if falsified.** The word "qualified" leaves reports until the fallback is in force; the
   ten-fold test's acceptance uses fixed-n confirmation, with the cost of the cap in its phase ledger; a leak
   (clause of GATE-QUAL-02) voids the affected cells, retires their world families and, if unbounded, voids the
   partial gate.
6. **Evidence so far.** The level series is arithmetic: Σ_{i=1..m} α/[i·(i + 1)] = α·m/(m + 1) < α. The cost of
   late requests grows: for a failure-rate target of 10⁻⁴, a single test at level 0.05 needs 29,956
   failure-free episodes, and the tenth qualification request, at level α₁₀ = 0.05/110, needs 76,959.
7. **Procedure.** Register α, the adversary script, the planted gain size and the campaign counts before the
   simulation; read at M3 exit; `docs/adr/bet-21-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with the adversary's script digest and the planted gain size.
- [ ] Counts recorded: false promotions out of 2,000 (at most 123); planted gains promoted within 10 requests
      (at least 1,600 of 2,000); the unblinded control's false-promotion share (above 99%), each on the world
      BD-260 reads it on, with the paired worlds' counts reported beside them.
- [ ] Firewall results: 10⁴ fuzzed reads with zero sealed fields; the planted leak found.
- [ ] Verdict recorded; if falsified, the fallback and the withdrawal of "qualified" are in force.

## Test plan

### Unit tests
`bets.rs`: `bet21_level_series` (α = 1/20: α₁ = 1/40, α₂ = 1/120, α₁₀ = 1/2200; partial sum to m = 10 equals
α·10/11 = 1/22); `bet21_118_false_promotions_holds`; `bet21_131_false_promotions_falsifies_a`;
`bet21_power_0_76_falsifies_b`; `bet21_unblinded_control_not_inflated_is_untested`;
`bet21_one_sealed_field_leak_fails_qual02`.

### End-to-end script and logging
`scripts/e2e/bet_21_decide.sh` recomputes the counts from BD-260's gate record and the campaign ledgers read
through `frob eval ledger <campaign> --json` (levels, outcomes and receipt hashes; the offline replay of
commitments and cluster derivation by `frob-verify` is BD-383's, at M5, and is not cited at M3), reruns a
50-campaign smoke of the adversary against the real `frob eval qualify` on `synthetic:null-absolute/1` (each
campaign opened and closed by the owner session with `frob eval ledger --action open … --json` and
`frob eval ledger <campaign> --action close --json`, admin actions that take no key, each qualification request
with its own `--key`), reads the
verdict of BET-21 from the bet register's report (`cargo run -p frob-conformance --example bets -- status
--json`, example `bets` of N00-28, the one verdict rule) and checks `docs/adr/bet-21-m3.md` against it. Logged
per STD-E2E: campaigns, false promotions against 123, power against 0.8, control inflation
and the world it was read on, fuzz results, levels spent; `summary` naming GATE-QUAL-01 and GATE-QUAL-02.

### Known answers and negative controls
Σ α/[i·(i + 1)] = α·m/(m + 1). Failure-free episode counts by the exact rule ⌈ln(level) ÷ ln(1 − p)⌉ at
p = 10⁻⁴: 29,956 at level 0.05; 76,959 at level α₁₀ = 1/2200. Negative controls: resubmission to
reused clusters; peeking at a fixed-n test; a receipt with an extra per-world field.

## Considerations

- The lane's levels shrink quickly, so an agent that retries carelessly prices itself out; reports show the
  level spent and the cost of the next request.
- The decision of request i must not depend on requests after it; that metamorphic relation is part of the
  gate's battery.

## References

§4 (BET-21), §40.5, §3, §24.17, §43.6, §43.12; GATE-QUAL-01, GATE-QUAL-02.
