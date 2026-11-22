---
key: "BET-37"
title: "Decide BET-37: the building is a design variable (a priced retrofit catalogue against robot-only repair)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "habitat", "frob-scene", "frontier", "bet", "gate"]
parent: "WP-53"
deps: ["N00-28", "N00-07", "BD-233", "BD-369", "STD-E2E"]
needs: ["§19.12: the retrofit corpus of minimized failures and the cited sources for prices and dimensional norms", "§26.7: minimal counterexample worlds, the form in which a failure is handed to the retrofit search"]
related: ["BD-274", "BD-227", "BD-234", "BET-22", "NS-18", "N00-22"]
gates: ["GATE-HAB-11", "KILL-RETROFIT"]
plan_refs: ["§4 (BET-37)", "§40.5", "§2.4 (lever 5)", "§19.12", "§19.19"]
---
## Background

**The bet.** Every repair an agent usually considers lives in the robot. But a robot is deployed into a place,
and sometimes the cheapest fix is in the place: a threshold ramp, a door closer adjusted, a marker on a glass
door. The design makes the world a design variable with two mechanisms. A **clearance complex** answers
"can this footprint get through here" with a route witness or an *impossibility cut* that names the
bottleneck. A **priced retrofit catalogue** lists building amendments with prices and dimensional norms from
cited sources; a search finds the cheapest kit that flips a failing verdict, with the policy frozen, and sets
it beside the cheapest redesign in one budget. The bet is that for a meaningful share of failures a small kit
exists and is the cheaper answer. Tier: Frontier.

## Scope

In: pre-registration, the M4 reading, verdict and decision record. Out: clearance complexes, the catalogue,
the search verb.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. Kits of at most three items fix fewer than 30% of minimized failures on the retrofit corpus.
   b. An accepted kit violates a dimensional norm from its cited source.
2. **Competing arm.** Robot-only repair. **Potency control.** A failure that no listed retrofit can fix must
   return the bottleneck and no kit.
3. **Deciding gate (TARGET).** GATE-HAB-11 (Design comparison, M4): three planted bottlenecks (a 20 mm
   threshold, a door closer above the robot's push capability, an unmarked glass door) are repaired by the
   expected kits with the policy identity unchanged; the wheel-and-ramp closed form is reproduced by both arms.
4. **Kill gate and fallback.** KILL-RETROFIT (M4): on the home battery kits flip fewer than 30% of the failures
   attributed to a building feature, or more than 10% of kit verdicts reverse on the referee. Fallback: the
   catalogue remains the edit vocabulary of the adversarial search (which needs a list of legal world edits),
   and the search verb is withdrawn. The bet's own fallback: the clearance complex reports the bottleneck only.
5. **Downstream if falsified.** Mission answers list robot edits only; the scenario "a twelve-dollar ramp
   against a chassis redesign" is demonstrated as a bottleneck report; the "world as a design variable" goal is
   met by the clearance complex alone.
6. **Evidence so far.** None measured. The wheel-and-ramp case has a closed form (held in the retrofit
   mechanism's derivation file), which is why it serves as the known answer for both arms. Two facts usable
   without it: a ramp of rise h at slope 1:12 needs a run of 12·h; a rigid wheel of radius r cannot roll up a
   square step of height h ≥ r.
7. **Procedure.** Register the corpus, the three-item limit, the 30% share, and the catalogue version with its
   sources before the search; every price and norm without a cited source is labelled `assumed` and its kits
   cannot be accepted under the norm clause; read at M4 exit; `docs/adr/bet-37-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the corpus digest and the catalogue version.
- [ ] The share of minimized failures fixed by kits of at most three items (at least 30%), with the policy
      identity unchanged in every counted case; the share of kit verdicts that reverse on the referee (at most
      10%).
- [ ] The three planted bottlenecks were repaired by the expected kits; the unfixable failure returned its
      bottleneck and no kit.
- [ ] No accepted kit violates a cited dimensional norm; verdict and the KILL-RETROFIT decision recorded.

## Test plan

### Unit tests
`bets.rs`: `bet37_fix_share_0_38_holds`; `bet37_fix_share_0_24_falsifies_a`;
`bet37_kit_violating_norm_falsifies_b` (a ramp steeper than its cited norm accepted);
`bet37_policy_identity_changed_not_counted`; `bet37_referee_reverses_14_percent_fires_kill`;
`bet37_assumed_price_cannot_support_acceptance_under_norm_clause`.

### End-to-end script and logging
`scripts/e2e/bet_37_decide.sh` runs `frob scene fit` on the planted 20 mm threshold (expects an impossibility
cut naming the threshold), then `frob scene retrofit` on the corresponding minimal counterexample world (expects
a one-item kit: a threshold ramp), re-evaluates with the frozen policy and compares policy identities,
recomputes the corpus statistics and the verdict. Logged per STD-E2E: failure, bottleneck, kit items with price
and source or `assumed`, norm checks, policy digest before and after, verdict flip, referee confirmation;
`summary` naming GATE-HAB-11 and KILL-RETROFIT.

### Known answers and negative controls
A 19 mm threshold with a 1:12 ramp needs a run of 228 mm; a 20 mm threshold needs 240 mm. Negative controls: the
unfixable failure; a kit of four items offered where the limit is three; a ramp whose slope exceeds its cited
norm; a "fix" that works only because the policy was retrained.

## Considerations

- Prices and norms date quickly and vary by place; the catalogue is versioned and every accepted kit names its
  source and date.
- A retrofit and a redesign are competing, testable decisions in one budget; the answer lists both.

## References

§4 (BET-37), §40.5, §2.4, §19.12, §19.19; GATE-HAB-11; KILL-RETROFIT.
