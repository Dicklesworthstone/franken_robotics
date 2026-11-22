---
key: "BET-03"
title: "Decide BET-03: convex compliant contact, audited by a referee, is good enough for locomotion decisions"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "traction", "kinesis", "frob-contact", "solid", "bet", "gate"]
parent: "WP-50"
deps: ["N00-28", "BD-212", "BD-266", "BD-208", "STD-E2E"]
needs: ["§18.4: the parameter mapping between the fast-tier contact law CL-1 and the referee's law, and the alternative law CL-2 (Hunt–Crossley)", "§38.8: the frozen standard battery on which fidelity is measured (OPEN-GAUNT-7 for wheeled, tracked and hybrid classes)"]
related: ["BD-386", "BD-263", "BD-265", "BET-34", "NS-09", "N00-25"]
gates: ["GATE-FID-01", "GATE-FID-05", "KILL-CONTACT"]
plan_refs: ["§4 (BET-03)", "§40.5", "§16.17", "§18.17", "§1.6 (N3)"]
---
## Background

**The bet.** The fast tiers use a convex, compliant contact formulation (the kind that scales to thousands of
GPU worlds), and a slower, more trusted referee tier audits them. The bet is that this pairing is good enough
for *locomotion decisions*: that the fast tier's slip, stance impulse and cost of transport track the referee
closely enough, and that which design wins does not depend on the contact model's form.

**Why it matters.** Every fast-tier result in the product is provisional until promoted on the referee. If the
fast tier disagreed with the referee on the quantities locomotion decisions rest on, promotion would overturn
most results and the batch would be an expensive suggestion engine. And if a ranking of finalists flips under
another admissible contact law, the "winner" is an artifact of model form. Tier: Solid.

## Scope

In: pre-registration, the M2 reading (fidelity) and the M3 reading (model-form rank check), the verdict and
decision record. Out: the contact laws, the fidelity ledger, the model-form check and the energy audit.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. On the standard battery the fast tier disagrees with the referee beyond the fidelity thresholds on slip
      distance, stance impulse or cost of transport, and no parameter mapping fixes it.
   b. The ranking of finalists changes under the model-form rank check: an optional hard-contact law, another
      cone shape, stiffness halved and doubled, and a half step.
2. **Competing arm.** The optional hard-contact complementarity batch law (Frontier). **Potency control.** A
   planted energy-creating contact exploit, which the work–energy audit of the simulator must catch; planted
   pyramid-cone and penetration-recovery "winners", which the model-form check must flag.
3. **Deciding gates (TARGETs).**
   - GATE-FID-01 (Fidelity, M2): `batch-cpu` against the referee on the M2 battery: median and 95th-percentile
     disagreement at most 2% and 5% for base displacement over 2 s, 3% and 8% for energy per metre, 5% and 12%
     for stance impulse. Re-baselined once at M2 with a written rationale, then frozen.
   - GATE-FID-05 (Known answer, M3): the planted pyramid-cone and penetration-recovery winners are returned as
     `model-conditional`; a genuine winner is `model-robust`; the check costs at most 10% of the campaign's
     compute.
4. **Kill gate and fallback.** KILL-CONTACT (M2): GATE-FID-01 fails on slip or creep after parameter mapping and
   one repair cycle. Fallback, in order: the Hunt–Crossley law CL-2 becomes the default at a published lower
   throughput; if that also fails, locomotion claims are restricted to the referee tier and the fast tiers are
   labelled exploratory.
5. **Downstream if falsified.** Throughput parity rows are re-measured under CL-2 and the thesis ceilings are
   recomputed; if both fallbacks are exhausted the legged strata of the partial ten-fold gate are re-planned.
   Under clause (b) alone the affected rankings carry `model-conditional` and may not spend prototype budget.
6. **Evidence so far.** None on this system. A plan-time planar pilot found 94 of 156 static-against-dynamic
   agreements on a sill fixture and that a change of winding law flips 24 of 156 labels: model form changes
   rankings in small cases, which is why clause (b) exists.
7. **Procedure.** Register before the battery is first run; read GATE-FID-01 and KILL-CONTACT at M2 exit, and
   GATE-FID-05 at M3 exit; write `docs/adr/bet-03-m2.md` and `bet-03-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with the six fidelity thresholds before the first battery run; the one permitted
      re-baseline at M2 has its written rationale and date.
- [ ] Measured medians and 95th percentiles for the three quantities are recorded against their thresholds.
- [ ] The energy-creating exploit and both planted model-form winners were caught.
- [ ] The verdict and the KILL-CONTACT decision are recorded as ADRs at M2 and M3.

## Test plan

### Unit tests
`bets.rs`: `bet03_within_thresholds_holds` (medians 1.6%, 2.4%, 4.1% and 95th percentiles 4.2%, 7.0%, 10.5%);
`bet03_stance_impulse_p95_13_percent_falsifies_a`; `bet03_planted_pyramid_winner_not_flagged_fails_fid05`;
`bet03_second_rebaseline_rejected`; `bet03_kill_contact_installs_cl2_then_referee_only`.

### End-to-end script and logging
`scripts/e2e/bet_03_decide.sh` recomputes the verdict from the gate rows, checks the ADRs, and checks that under
a fired kill gate reports carry the fallback's label. Logged per STD-E2E: the six measured values with
thresholds, model-form labels of the three planted cases, cost share of the check, controls caught; `summary`
naming GATE-FID-01, GATE-FID-05 and KILL-CONTACT.

### Known answers and negative controls
Negative controls: the planted energy-creating exploit; the two planted model-form winners; a battery re-frozen
a second time; a fidelity report that states the referee "is right" (refused: Fidelity is directional).

## Considerations

- Fidelity says the fast tier tracks the referee on a battery; it never says either is right, and nothing here
  is a claim about reality.
- Classes without a frozen battery (wheeled, tracked, hybrid, manipulation) have no fidelity claim until theirs
  exists.
- The referee refuses more than 21 contacts on some scenes; where it cannot complete a scenario the comparison
  falls to the strict CPU profile and the result says so (see BET-34).

## References

§4 (BET-03), §40.5, §16.17, §18.17; GATE-FID-01, GATE-FID-05; KILL-CONTACT.
