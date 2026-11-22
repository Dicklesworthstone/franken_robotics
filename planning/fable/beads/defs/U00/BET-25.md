---
key: "BET-25"
title: "Decide BET-25: an energy-budget safety tier (no contact model in the claim; measured shield margin; class Tested)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "reflex", "safety", "frob-shield", "frontier", "bet", "gate"]
parent: "WP-65"
deps: ["N00-28", "N00-07", "BD-337", "BD-338", "BD-339", "STD-E2E"]
needs: ["§22.21: the pre-slip aliasing fixture with its numeric instance (two friction regimes with identical histories before first slip) and the sampled-data backup domain (GATE-SHIELD-06)", "§22.11: the `.fenergy` mutation corpus"]
related: ["BD-340", "BD-092", "NS-23", "N00-12", "N00-CX", "N00-26"]
gates: ["GATE-SHIELD-01", "GATE-SHIELD-03", "KILL-ENERGY", "KILL-SHIELD"]
plan_refs: ["§4 (BET-25)", "§40.5", "§0.8 (H-20)", "§22.18", "§8.2"]
---
## Background

**The bet.** A safety statement about a robot at impact usually depends on a contact model nobody trusts at
impact. The energy-budget tier states the claim differently: the kinetic and potential energy that *can arrive*
at a joint or structure in a given situation is below what it is rated to absorb, with every term taken from
the energy ledger. No contact law enters such a claim, so it survives contact uncertainty. Beside it, reduced-
order runtime *shields* filter commands to keep the system inside the budget, with a margin that is measured
statistically. The bet is that this tier is sound on its mutation corpus and cheap enough to use. Tier:
Frontier.

**What is not claimed.** Full-order safety certificates are not attempted. The claim product of a shielded
result is Tested, and the type offers no conversion to Proved: the planning evidence narrowed "proof-guided
control" to exactly this.

## Scope

In: pre-registration, the M4 reading, verdict and decision record. Out: the budget format and monitor, shields,
the margin measurement.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. Any false admission on the energy-budget mutation corpus (a budget that should be refused is admitted).
   b. A shielded policy keeps less than 85% of unshielded return.
   c. The shield costs more than 5% of step time.
   d. The measured margin is violated on shifted worlds beyond its stated rate.
2. **Competing arm.** An unshielded policy trained with a penalty. **Potency control.** The pre-slip aliasing
   fixture: two friction regimes with identical histories before first slip, where a monitor alone cannot see
   the breach in time; the control shows that enforcement (a shield), not observation (a monitor), is what the
   claim needs. Its numeric instance is owed by the energy-budget mechanism before the control can be
   registered.
3. **Deciding gates (TARGETs).**
   - GATE-SHIELD-01 (Known answer, M4): the worked card reproduces; no exceedance over 10⁴ randomized contact
     sequences with friction coefficient in [0, 5], restitution in [0, 1] and every contact law; a budget with a
     planted unlisted spring is refused; a payload-only budget shows an exceedance.
   - GATE-SHIELD-03 (Design comparison, M4): 5,000 in-envelope and 5,000 shifted-world episodes with
     independent breach labels: in-envelope violations within the stated bound; shifted results reported
     separately; at least 85% of unshielded return; at most 5% added step cost.
4. **Kill gates and fallbacks.** KILL-ENERGY (M4): an exceedance under the premises survives one repair cycle at
   GATE-SHIELD-01; the claim is withdrawn and the monitor remains as an alarm of class Estimated. KILL-SHIELD
   (M4): GATE-SHIELD-03 fails after one repair cycle; the energy tier stays, barrier rows become reward
   penalties and alarms, and nothing is called a shield. The bet's own fallback: monitors and reports only,
   with no shield claim.
5. **Downstream if falsified.** The scenario "an energy-budget safety claim" is demonstrated up to what
   survives (the budget without the shield, or the monitor as an alarm); every report that printed a shielded
   margin prints the alarm class instead; the safety-tier goal is recorded as met by fallback.
6. **Procedure.** Register the mutation corpus, the return and cost thresholds, the stated violation rate and
   the shifted-world family before any run; read at M4 exit; `docs/adr/bet-25-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the corpus digest and the stated violation bound.
- [ ] Zero false admissions on the mutation corpus; zero exceedances over 10⁴ randomized contact sequences
      under the premises.
- [ ] Return kept (at least 85%), added step cost (at most 5%), in-envelope violation rate against its bound,
      shifted-world rate reported separately.
- [ ] The pre-slip fixture run once its numbers exist; verdict and both kill decisions recorded as ADRs; every
      output's claim class is Tested or weaker.

## Test plan

### Unit tests
`bets.rs`: `bet25_all_within_holds`; `bet25_one_false_admission_falsifies_a`;
`bet25_return_kept_0_81_falsifies_b`; `bet25_step_cost_6_percent_falsifies_c`;
`bet25_shifted_violations_above_stated_rate_falsifies_d`; `bet25_exceedance_fires_kill_energy`;
`bet25_class_never_above_tested` (a synthetic shielded result offered as Proved is refused, FROB-REF-834).

### End-to-end script and logging
`scripts/e2e/bet_25_decide.sh` re-runs the worked card with `frob shield budget` and the planted unlisted-spring
budget (expects a refusal), replays a shielded run with `frob shield check`, recomputes return kept, step cost
and violation rates from the ledger, and the verdict. Logged per STD-E2E: budget terms from the energy ledger
with units, exceedance counts, return ratio against 0.85, cost against 5%, violation rates against the stated
bound, the evidence class; `summary` naming GATE-SHIELD-01, GATE-SHIELD-03, KILL-ENERGY and KILL-SHIELD.

### Known answers and negative controls
A hand-checkable budget: a 40 kg body stepping down 0.12 m at 0.5 m/s can deliver at most m·g·h + ½·m·v² =
40 × 9.81 × 0.12 + 0.5 × 40 × 0.25 = 47.1 + 5.0 = 52.1 J to the landing leg (illustrative numbers); a rating of
60 J admits the budget and a rating of 50 J refuses it. CX-012. Negative controls: the planted unlisted spring;
the payload-only budget; the monitor-only arm on the pre-slip fixture.

## Considerations

- The budget's soundness rests on listing every energy store; an unlisted spring is the canonical way to be
  wrong, hence the planted control.
- A measured margin holds for the calibrated policy and family; a change of policy must trip the refusal that
  invalidates it.

## References

§4 (BET-25), §40.5, §0.8 (H-20), §22.18, §8.2; GATE-SHIELD-01, -03; KILL-ENERGY; KILL-SHIELD; CX-012.
