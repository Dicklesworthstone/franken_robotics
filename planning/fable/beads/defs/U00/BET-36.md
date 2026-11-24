---
key: "BET-36"
title: "Decide BET-36: living worlds change decisions (film-friction law, rigid proxies, advected films)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "habitat", "traction", "frob-scene", "frob-contact", "frontier", "bet", "gate"]
parent: "WP-53"
deps: ["BD-230", "BD-231", "BD-253", "N00-28", "N18a-33", "N18a-34", "N18a-37", "N19b-06", "N38a-09", "STD-E2E"]
needs: []
related: ["GB-21", "N00-12", "N00-23", "N19b-11", "N19b-16", "N19b-20", "N40-31", "NS-04"]
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

In: pre-registration of all three clauses (their thresholds are fixed now, before any of them is measured), the
M3 reading of clause (a), the M3 verdict and its record `docs/adr/bet-36-m3.md` with clauses (b) and (c) marked
`pending`. Out, with the beads that have them:

- The film-friction law `K-film` and its gates GATE-TRAC-05 and GATE-HAB-06 (BD-230; end-to-end scripts N18a-37
  and N19b-06); the wet-floor battery that measures the changed-outcome share and step cost (N18a-33); the
  decision KILL-FILM, its ADR and its downstream changes (N18a-34), which this bead reads and never re-decides.
- The rigid proxies (BD-231) and the M3 interior tasks that host the scenes (BD-253).
- The M6 reading: the proxy-adequacy pilot and GATE-HAB-09 (N19b-16), the decision KILL-LIVING (N19b-20), the
  advected film and GATE-HAB-07 (BD-413, N19b-10), the decision KILL-ADVECT (N19b-11), and the bet's verdict on
  clauses (b) and (c) with `docs/adr/bet-36-m6.md`: N00p1-12, which depends on this bead (a bead closed at M3
  cannot carry a reading due at M6). Coupled islands are BD-416 (Moonshot).

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
   - Read at M6 by N00p1-12, not here: the proxy-adequacy gate GATE-HAB-09 (Fidelity; reports the pilot's
     number; before the island work package) and the advected-film gate GATE-HAB-07 (Known answer, M6: relative
     volume residual at most 10⁻⁶ over 10⁵ steps with pickup and redeposition; spreading exponent within 2% of
     1/8; the non-conservative control fails; no cross-world leak in 10⁶ world-episodes).
4. **Kill gates and fallbacks.** KILL-FILM (M3): on the wet-floor battery the film law changes fewer than 10% of
   episode outcomes against a constant-coefficient patch, or adds more than 5% to step time; static condition
   cards on the lockstep tiers, the law kept on the transactional profiles for audits (decided by N18a-34).
   KILL-ADVECT (M6; N19b-11): the
   advected film fails its mass gate after one repair cycle, or costs more than 15% of the step, or rankings
   agree with static patches at 0.95 or above; static film-friction patches, advection not built. KILL-LIVING
   (M6, before the island work package; N19b-20): the proxies reproduce at least 90% of full-model failures
   (GATE-HAB-09), or an island
   costs more than 181 rigid worlds at 5% activity with no route to less; then coupled islands are not built
   and reports say deformable matter is represented by proxy. The bet's own fallback: static friction maps with
   wet patches.
5. **Downstream if falsified.** Under (a): wet floors are constant-coefficient patches and the scenario "a
   kitchen with a wet floor" reports a patch, not a film. Under (b) for a class: no proxy claim for that class.
   Under (c): no advection. None of this is on the critical path of the ten-fold test, and none is counted in
   the thesis.
6. **Procedure.** Register the batteries, the 10% outcome share, the 5% and 15% cost limits and the 0.95
   correlation before any run; read the Solid half at M3 exit; `docs/adr/bet-36-m3.md`. The pilot and M6 results
   are a second record, `docs/adr/bet-36-m6.md`, written by N00p1-12; this record is not edited after M3.

## Acceptance criteria

- [ ] Pre-registered with the wet-floor battery's digest.
- [ ] Film law known answers recorded; share of episode outcomes changed against a constant-coefficient patch
      (at least 10%); added step time (at most 5%).
- [ ] The dry limit is bit-identical to the dry law on every tier.
- [ ] Verdict for the Solid half recorded in `docs/adr/bet-36-m3.md`, citing the KILL-FILM decision on record by
      N18a-34 (by identifier and hash; with it missing the verdict is not filed); clauses (b) and (c) marked
      `pending` with N00p1-12 named as their reader.

## Test plan

### Unit tests
`bets.rs`: `bet36_outcomes_changed_17_percent_cost_3_percent_holds_a`;
`bet36_outcomes_changed_6_percent_falsifies_a`; `bet36_cost_7_percent_falsifies_a`;
`bet36_squeeze_time_off_by_3_percent_falsifies_a` (a known answer missed: 0.0388 s against 0.0377 s, 2.9%);
`bet36_reads_kill_film_record` (N18a-34's record fired → `Falsified` on (a) through its kill gate, the record's
hash carried; the trigger is applied there); `bet36_pending_halves_do_not_block_m3_verdict` (clauses (b) and (c)
`pending`, the M3 verdict is `Held { through: M3 }` on (a) alone). The clause-(b) and clause-(c) tests are
N00p1-12's.

### End-to-end script and logging
`scripts/e2e/bet_36_decide.sh` evaluates the film law at fixture states with `frob contact probe` (dry limit,
thick-film limit, one squeeze time), recomputes the changed-outcome share and step cost from the battery's
ledger records, and the verdict. Logged per STD-E2E: state, film thickness, friction coefficient, squeeze time
against its closed form, outcome counts under film and under patch, step-time ratio; `summary` naming
GATE-TRAC-05, GATE-HAB-06 and KILL-FILM.

### Known answers and negative controls
KA-404, Stefan's law on a circular patch of radius a under load N in a film of viscosity η:
dh/dt = −2·N·h³/(3π·η·a⁴), integrated 1/h² − 1/h₀² = 4·N·t/(3π·η·a⁴); from a thick film to 10 µm: water
(1 mPa·s), a = 2 cm, 100 N: 0.038 s (0.0377); cooking oil (60 mPa·s), same patch: 2.3 s; water, a = 6 cm, 400 N:
0.76 s (0.763), each within 2% (CX-177: against a 0.3 s stance a small foot on water reaches boundary friction
almost at once while oil and large flat feet ride a film the whole stance, so a constant coefficient per floor
texel is wrong). Squeeze time scales with a⁴: doubling the contact radius at equal load multiplies it by 16
(MR-26). Friction blend μ_eff = ψ·μ_b + (1 − ψ)·η·v·π·a²/(h·N), ψ = 1 for h ≤ σ and ψ = 0 for h ≥ 3σ. The spreading
of an advected film follows an exponent of 1/8 (read at M6). Negative controls: a film law
whose dry limit differs by one unit in the last place from the dry law; the non-conservative advection control;
a film thicker than its admitted range (refused).

## Considerations

- "Changes outcomes" is measured against a constant-coefficient patch, the cheapest alternative; a law that is
  right but changes nothing is not worth its step time.
- A proxy is a model, and reports say so: deformable matter represented by proxy.

## References

§4 (BET-36), §40.5, §0.8 (H-16), §18.8, §18.17, §19.10, §19.19; GATE-TRAC-05; GATE-HAB-06, -07, -09; KILL-FILM;
KILL-ADVECT; KILL-LIVING.
