---
key: "BET-29"
title: "Decide BET-29: generative structure (optimized links at least 7% lighter than engineered parts at equal margin)"
type: "task"
priority: 2
milestone: "M5"
labels: ["m5", "foundry", "frob-foundry", "frontier", "bet", "gate"]
parent: "WP-71"
deps: ["N00-28", "N00-07", "BD-403", "BD-406", "BD-407", "STD-E2E"]
needs: ["§31.5: the reference links and their engineered reference parts, with the equal-stiffness and equal-fatigue-margin comparison rule"]
related: ["BD-400", "BD-401", "BD-402", "BD-404", "BD-408", "NS-20", "N00-11"]
gates: ["GATE-FOUNDRY-05", "GATE-FOUNDRY-08", "GATE-FOUNDRY-09", "KILL-TOPOPT", "KILL-TOLERANCE", "KILL-PACKET"]
plan_refs: ["§4 (BET-29)", "§40.5", "§0.8 (H-12)", "§31.19", "§2.5"]
---
## Background

**The bet.** The simulation knows the load history each link of a design actually sees. Topology optimization
under those loads can remove material an engineer would have left, giving links that are lighter at the same
stiffness and fatigue margin; the lighter link feeds back into the model (mass and inertia change), and the
result leaves the system as a build packet: a bill of materials, faceted solid geometry, a tolerance stack that
closes, and fatigue margins. The bet is that the saving is real and that the packet's geometry and tolerances
hold together. Tier: Frontier.

**What it is not.** It is not part of the ten-fold thesis: it improves the built object, not the time to a
decision (the hypothesis is DEFERRED as a decision-time lever). Published savings are links 7–30% lighter than
engineered parts, which is 2–8% of robot mass on the one quadruped with a published mass breakdown.

## Scope

In: pre-registration, the M5 reading, verdict and decision record. Out: load extraction, optimization,
tolerance allocation, geometry export, the packet.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. On the reference links the optimized part is less than 7% lighter than the engineered part at equal
      stiffness and fatigue margin.
   b. The build packet fails its geometry round trip or its tolerance closure.
2. **Competing arm.** The engineered reference part. **Potency controls.** A planted overstress the structural
   check must locate; an open shell and an over-bound part the exporter must refuse; a mutated flag the auditor
   must reject.
3. **Deciding gates (TARGETs).**
   - GATE-FOUNDRY-05 (Design comparison, M5): the stage-2 pilot: saving at least 7% (the kill threshold) with
     15% the target; the loads-and-mass convergence test passes within five iterations; coarse solve at most
     120 s per link class.
   - GATE-FOUNDRY-08 (Known answer, M5): the worked tolerance allocation to 10⁻⁹; the zero-sensitivity and
     quadratic cases handled as specified; a correlated stack equals its hand value.
   - GATE-FOUNDRY-09 (Known answer, M5): tetrahedron round trip bit-exact; open shell and over-bound parts
     refused; mass properties from exported geometry within 10⁻⁶ of the model.
4. **Kill gates and fallbacks.** KILL-TOPOPT (M5): saving under 7%, no convergence, or over budget; beam-rung
   sizing supplies mass and inertia feedback, three-dimensional optimization becomes an offline option or is
   dropped, and the structural margin analysis is unaffected. KILL-TOLERANCE (M5): the allocation gate fails,
   or band-extreme checks fail on more than a fifth of features in the pilot; worst-case boxes per interface.
   KILL-PACKET (M5): the geometry or packet gate fails after one repair cycle; the packet is reduced to a parts
   list, mass properties and STL, with STEP and harness deferred. The bet's own fallback: the structural margin
   analysis alone.
5. **Downstream if falsified.** The scenario "a build packet" is demonstrated with the reduced packet or
   without the re-optimized link; nothing on the critical path changes.
6. **Procedure.** Register the reference links, the equal-margin rule and the 7% threshold before the pilot;
   read at M5 exit; `docs/adr/bet-29-m5.md`.

## Acceptance criteria

- [ ] Pre-registered with the reference parts' masses, stiffness and fatigue margins.
- [ ] Per link class: optimized mass, engineered mass, the saving against 7% (and the 15% target), iterations
      to convergence, solve time against 120 s.
- [ ] Tolerance and geometry gates recorded with their measured residuals.
- [ ] Every packet file carries the three no-authorization flags and `fabrication_released`, all false.
- [ ] Verdict and the three kill decisions recorded as ADRs.

## Test plan

### Unit tests
`bets.rs`: `bet29_saving_11_percent_holds`; `bet29_saving_5_percent_falsifies_a_and_fires_kill_topopt`;
`bet29_round_trip_mismatch_falsifies_b`; `bet29_tolerance_stack_open_falsifies_b`;
`bet29_equal_margin_violated_is_not_a_saving` (an optimized part 12% lighter with 20% less fatigue margin is
not counted).

### End-to-end script and logging
`scripts/e2e/bet_29_decide.sh` runs `frob foundry optimize` on one reference link at coarse resolution,
`frob foundry tolerance` on the worked allocation, `frob foundry export` and a re-import, recomputes masses from
exported geometry, and recomputes the verdict. Logged per STD-E2E: link class, masses with units, saving,
stiffness and fatigue margins of both parts, convergence iterations, tolerance residual, round-trip digest,
flags; `summary` naming the three gates and three kill gates.

### Known answers and negative controls
A hand-checkable saving: an engineered link of 0.500 kg against an optimized 0.455 kg at equal margins is a 9.0%
saving (above 7%, below the 15% target). Share of robot mass: links 7–30% lighter is 2–8% of the robot.
Negative controls: the planted overstress (located with utilization 1.25 ± 5%); the open shell; a packet whose
flag was mutated.

## Considerations

- The packet's claim product is Estimated wherever its material cards are; it authorizes nothing.
- A load case that enters a packet is re-extracted on the referee before it is trusted.

## References

§4 (BET-29), §40.5, §0.8 (H-12), §31.19, §2.5; GATE-FOUNDRY-05, -08, -09; KILL-TOPOPT; KILL-TOLERANCE;
KILL-PACKET.
