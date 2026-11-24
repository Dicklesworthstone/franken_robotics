---
key: "BET-37"
title: "Decide BET-37: the building is a design variable (a priced retrofit catalogue against robot-only repair)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "habitat", "frob-scene", "frontier", "bet", "gate"]
parent: "WP-53"
deps: ["BD-233", "BD-274", "BD-369", "N00-28", "N19b-37", "N19b-42", "N19b-43", "N38a-09", "STD-E2E"]
needs: []
related: ["N00-22", "N28bx-01", "N40-31", "NS-15", "NS-18"]
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

In: pre-registration, the M4 reading, verdict and decision record. Out, with the beads that have them:
clearance complexes `.fclear` with route witnesses and impossibility cuts (BD-233, gated at M3 by
GATE-HAB-10); minimal counterexample worlds `.fmcw` (`nemesis.shrink`, BD-274); the priced catalogue
`registries/retrofits.toml`, the search verb `scene.retrofit` and GATE-HAB-11 (BD-369); the catalogue pilot
(N19b-37); the owner's decision on the sources of prices and norms, OPEN-HAB-4, with its fail-closed default
(N19b-43); the decision KILL-RETROFIT (N19b-42).

The register lists BET-37 at M3 and M4. No deciding-gate part is due at M3 (GATE-HAB-11 lands at M4 with the
catalogue), so the M3 exit state is `not-due`; the clearance complex the bet's fallback rests on is gated at M3
by GATE-HAB-10, which is not this bet's deciding gate.

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
6. **Evidence so far.** None measured. The wheel-and-ramp case has a closed form, which is why it serves as the
   known answer for both arms: a driven wheel of radius r that carries its own load and meets an edge of height
   h quasi-statically, with no help from another axle, needs a friction coefficient of at least
   tan α = √(h·(2r − h)) / (r − h) at the edge. For r = 50 mm and h = 20 mm that is 40/30 = 4/3, which no floor
   card offers, so the climb is statically impossible; a wedge with a run of twelve times its rise needs 1/12;
   doubling the wheel radius to 100 mm needs 60/80 = 0.75. The retrofit arm must return the ramp, the redesign
   arm the wheel, and the report both. The static figure is a screen, not the verdict: a body arriving at
   √(2·g·h) = 0.63 m/s has the energy to rise 20 mm before losses, which is why the search re-runs the full
   episode (FACT recorded in the plan's source register: in an executed planar sill study, 62 of 156 cases a
   sampled static test rejected succeeded dynamically). A ramp of rise h at slope 1:12 needs a run of 12·h; a
   rigid wheel of radius r cannot roll up a square step of height h ≥ r.
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
- [ ] No accepted kit violates a cited dimensional norm; the bet's verdict is recorded and cites the
      KILL-RETROFIT decision record. The kill gate is decided once, by the kill-gate bead N19b-42, which applies
      the trigger, writes the kill ADR and supplies the measured shares, the control count, the arm's price
      comparison and the norm-violation count; this bead reads them, applies the bet's own falsifier clauses and
      writes the bet verdict, and writes no second kill decision. The bet can be falsified with the kill gate not
      fired (a norm violation, or a fix share under 30% on the whole corpus where the building-attributed share
      is not).

## Test plan

### Unit tests
`bets.rs`: `bet37_fix_share_0_38_holds`; `bet37_fix_share_0_24_falsifies_a`;
`bet37_kit_violating_norm_falsifies_b` (a ramp steeper than its cited norm accepted);
`bet37_policy_identity_changed_not_counted`; `bet37_reads_kill_retrofit_reversals_14_percent` (N19b-42's
record fired because 14% > 10% of kit verdicts reversed on the referee → `Falsified` through its kill gate, the
record's hash carried; the trigger is applied there); `bet37_edge_friction_4_3_and_0_75` (tan α at r = 50 mm and
100 mm, h = 20 mm);
`bet37_assumed_price_cannot_support_acceptance_under_norm_clause`.

### End-to-end script and logging
`scripts/e2e/bet_37_decide.sh` runs `frob scene fit … --key b37-fit` on the planted 20 mm threshold (expects an
impossibility cut naming the threshold), then `frob scene retrofit … --key b37-retrofit` (both `run` verbs, each
call with its own key) on the corresponding minimal counterexample world (expects
a one-item kit: a threshold ramp), re-evaluates with the frozen policy and compares policy identities,
recomputes the corpus statistics, reads the verdict of BET-37 from the bet register's report (`cargo run -p
frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule) and checks
`docs/adr/bet-37-m4.md` against it. Logged per STD-E2E: failure, bottleneck, kit items with price
and source or `assumed`, norm checks, policy digest before and after, verdict flip, referee confirmation;
`summary` naming GATE-HAB-11 and KILL-RETROFIT.

### Known answers and negative controls
A 19 mm threshold with a 1:12 ramp needs a run of 228 mm; a 20 mm threshold needs 240 mm. The edge-friction
closed form above (4/3 at r = 50 mm, 0.75 at r = 100 mm, 1/12 on the wedge). Negative controls: the
unfixable failure; a kit of four items offered where the limit is three; a ramp whose slope exceeds its cited
norm; a "fix" that works only because the policy was retrained.

## Considerations

- Prices and norms date quickly and vary by place; the catalogue is versioned and every accepted kit names its
  source and date.
- A retrofit and a redesign are competing, testable decisions in one budget; the answer lists both.

## References

§4 (BET-37), §40.5, §2.4, §19.12, §19.19; GATE-HAB-11; KILL-RETROFIT.
