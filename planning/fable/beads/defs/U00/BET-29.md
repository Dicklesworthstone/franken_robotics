---
key: "BET-29"
title: "Decide BET-29: generative structure (optimized links at least 7% lighter than engineered parts at equal margin)"
type: "task"
priority: 2
milestone: "M5"
labels: ["m5", "foundry", "frob-foundry", "frontier", "bet", "gate"]
parent: "WP-71"
deps: ["BD-403", "BD-406", "BD-407", "N00-28", "N31b-13", "N31b-15", "N31b-18", "N31b-19", "N38a-09", "STD-E2E"]
needs: []
related: ["N00-11", "N40-31", "N40-32", "NS-20"]
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

In: pre-registration, the M5 reading, verdict and decision record. Out, with the beads that have them: load
extraction and the packet (their Foundry beads); topology optimization and GATE-FOUNDRY-05 (BD-403, with its
end-to-end script N31b-13); tolerance allocation and GATE-FOUNDRY-08 (BD-406); geometry export and
GATE-FOUNDRY-09 (BD-407); the decisions KILL-TOPOPT (N31b-15), KILL-TOLERANCE (N31b-18) and KILL-PACKET (N31b-19,
which also reads the packet gate GATE-FOUNDRY-10), which this bead reads and cites and never re-decides.

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
   KILL-PACKET (M5): the geometry gate GATE-FOUNDRY-09 or the packet gate GATE-FOUNDRY-10 fails after one
   repair cycle; the packet is reduced to a parts
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
- [ ] Verdict recorded in `docs/adr/bet-29-m5.md`, citing the three kill decisions on record (N31b-15, N31b-18,
      N31b-19) by identifier and hash; with any missing the verdict is not filed.

## Test plan

### Unit tests
`bets.rs`: `bet29_saving_11_percent_holds`; `bet29_saving_5_percent_falsifies_a_with_kill_record` (clause (a)
true; N31b-15's record: KILL-TOPOPT fired, its hash carried; the trigger is applied there);
`bet29_round_trip_mismatch_falsifies_b`; `bet29_tolerance_stack_open_falsifies_b`;
`bet29_equal_margin_violated_is_not_a_saving` (an optimized part 12% lighter with 20% less fatigue margin is
not counted).

### End-to-end script and logging
`scripts/e2e/bet_29_decide.sh` runs `frob foundry optimize <reference design> --links lower-leg --loop --key
"$(foundry_key bet29-opt)" --json` at coarse resolution (a `run` verb; the key comes from the cluster preamble,
N31a-51), `frob foundry tolerance --fixture worked-case --json` (a `read` verb, no key) on the worked
allocation, and `frob foundry export --fixture tube --geometry-only --key "$(foundry_key bet29-geo)" --json`,
whose read-back report (BD-407 §8) recomputes mass, centre of mass and inertia from each of the four exported
files; the script asserts `mass_rel_diff_max` ≤ 10⁻⁶ from it, recomputes the tube's mass from the STL itself
as an independent check, and then reads the verdict of BET-29 from the bet register's report (`cargo run -p
frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule) over the
three gate receipts and the three kill records, and checks `docs/adr/bet-29-m5.md` against it. Logged per
STD-E2E: link class, masses with units, saving,
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
