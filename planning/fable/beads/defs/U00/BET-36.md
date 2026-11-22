---
key: "BET-36"
title: "Decide BET-36: living worlds change decisions (film-friction law, rigid proxies, advected films)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "habitat", "traction", "frob-scene", "frob-contact", "frontier", "bet", "gate"]
parent: "WP-53"
deps: ["N00-28", "N00-07", "BD-230", "BD-231", "BD-253", "STD-E2E"]
needs: ["§19.10.7: the proxy-adequacy pilot (do rigid proxies reproduce the failures of coupled models in the same class) and the spill battery for advected films", "§18.8: the wet-floor battery and the constant-coefficient patch it is compared with"]
related: ["BD-229", "BD-413", "BD-415", "BD-416", "NS-04", "N00-12", "N00-23"]
gates: ["GATE-TRAC-05", "GATE-HAB-06", "KILL-FILM"]
plan_refs: ["§4 (BET-36)", "§40.5", "§0.8 (H-16)", "§18.8", "§18.17", "§19.10", "§19.19", "§43.9"]
---
## Background

**The bet.** Robots fail in worlds that change: a wet patch on a kitchen floor, a rug that slides, a cord
across a doorway, a spill the robot's own wheels spread. Simulating fluid, cloth and cables in every training
world is out of reach (full-room fluid is out of memory by orders of magnitude; coupled cloth and cable at batch
speed are out of reach on present foundations). The design substitutes: a contact-level **film-friction law**
(friction depends on a film thickness state at the contact, with squeeze-out and shear in closed form) and
**rigid proxies** (a rug as a free slab, a cord as a fixed ridge or a free rod, clutter as rigid bodies). Both
are Solid and in the core. The bet has three parts: the film law is right and matters; the proxies reproduce
the failures that coupled models would show; and (Frontier) films advected by the robot change design rankings.

**What is untested.** Whether proxies reproduce the failures of coupled models. A pilot decides that claim and
whether a coupled multiphysics island is ever funded; islands are Moonshot.

## Scope

In: pre-registration, the M3 reading (the Solid half), the M6 addendum (the proxy pilot and advected films),
verdict and decision records. Out: the film law, proxies, advected films, islands.

## Technical approach

1. **Falsifier clauses.**
   a. (Solid, M3) The film law misses its known answers, changes fewer than 10% of episode outcomes on the
      wet-floor battery, or adds more than 5% to step time.
   b. (Pilot, before the island work package) The proxies reproduce fewer than 90% of the coupled model's
      failures in the same class; the proxy claim is then withdrawn for that class and the island stays as an
      escalation.
   c. (Frontier, M6) Advected films leave design rankings on the spill battery at a rank correlation of 0.95 or
      above with static patches, or cost more than 15% of the step.
2. **Competing arm.** Static friction maps. **Potency controls.** The dry limit of the film law must be
   bit-identical to the dry contact law; a non-conservative advection control must fail the mass gate.
3. **Deciding gates (TARGETs).**
   - GATE-TRAC-05 (Known answer, M3): the film law: squeeze-out (Stefan) times within 2%; shear (Couette) force
     within 10⁻⁶ relative in double precision; the dry limit bit-identical; the limits at film thickness
     h ≤ σ and h ≥ 3σ (σ the combined surface roughness) and continuity between them on 10⁴ random states.
   - GATE-HAB-06 (Known answer, M3): the law as scenes consume it: the three squeeze times within 2%; the R⁴
     scaling of squeeze time with contact radius; dry and thick-film limits recover the card coefficients.
   - Addenda: the proxy-adequacy gate (reports the pilot's number; before the island work package) and the
     advected-film gate at M6 (relative volume residual at most 10⁻⁶ over 10⁵ steps with pickup and
     redeposition; spreading exponent within 2% of 1/8; no cross-world leak in 10⁶ world-episodes).
4. **Kill gates and fallbacks.** KILL-FILM (M3): on the wet-floor battery the film law changes fewer than 10% of
   episode outcomes against a constant-coefficient patch, or adds more than 5% to step time; static condition
   cards on the lockstep tiers, the law kept on the transactional profiles for audits. KILL-ADVECT (M6): the
   advected film fails its mass gate after one repair cycle, or costs more than 15% of the step, or rankings
   agree with static patches at 0.95 or above; static film-friction patches, advection not built. KILL-LIVING
   (M6, before the island work package): the proxies reproduce at least 90% of full-model failures, or an island
   costs more than 181 rigid worlds at 5% activity with no route to less; then coupled islands are not built
   and reports say deformable matter is represented by proxy. The bet's own fallback: static friction maps with
   wet patches.
5. **Downstream if falsified.** Under (a): wet floors are constant-coefficient patches and the scenario "a
   kitchen with a wet floor" reports a patch, not a film. Under (b) for a class: no proxy claim for that class.
   Under (c): no advection. None of this is on the critical path of the ten-fold test, and none is counted in
   the thesis.
6. **Procedure.** Register the batteries, the 10% outcome share, the 5% and 15% cost limits and the 0.95
   correlation before any run; read the Solid half at M3 exit; append the pilot and M6 results when their beads
   close; `docs/adr/bet-36-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with the wet-floor battery's digest.
- [ ] Film law known answers recorded; share of episode outcomes changed against a constant-coefficient patch
      (at least 10%); added step time (at most 5%).
- [ ] The dry limit is bit-identical to the dry law on every tier.
- [ ] Verdict for the Solid half and the KILL-FILM decision recorded at M3; clauses (b) and (c) marked `pending`
      with the beads that will supply them.

## Test plan

### Unit tests
`bets.rs`: `bet36_outcomes_changed_17_percent_cost_3_percent_holds_a`;
`bet36_outcomes_changed_6_percent_fires_kill_film`; `bet36_cost_7_percent_fires_kill_film`;
`bet36_proxy_reproduction_84_percent_withdraws_class_claim`; `bet36_rank_correlation_0_97_fires_kill_advect`;
`bet36_pending_halves_do_not_block_m3_verdict`.

### End-to-end script and logging
`scripts/e2e/bet_36_decide.sh` evaluates the film law at fixture states with `frob contact probe` (dry limit,
thick-film limit, one squeeze time), recomputes the changed-outcome share and step cost from the battery's
ledger records, and the verdict. Logged per STD-E2E: state, film thickness, friction coefficient, squeeze time
against its closed form, outcome counts under film and under patch, step-time ratio; `summary` naming
GATE-TRAC-05, GATE-HAB-06 and KILL-FILM.

### Known answers and negative controls
Squeeze-out of a film under a disc of radius R scales with R⁴: doubling the contact radius multiplies the
squeeze time by 16; the spreading of an advected film follows an exponent of 1/8. Negative controls: a film law
whose dry limit differs by one unit in the last place from the dry law; the non-conservative advection control;
a film thicker than its admitted range (refused).

## Considerations

- "Changes outcomes" is measured against a constant-coefficient patch, the cheapest alternative; a law that is
  right but changes nothing is not worth its step time.
- A proxy is a model, and reports say so: deformable matter represented by proxy.

## References

§4 (BET-36), §40.5, §0.8 (H-16), §18.8, §18.17, §19.10, §19.19; GATE-TRAC-05; GATE-HAB-06, -07, -09; KILL-FILM;
KILL-ADVECT; KILL-LIVING.
