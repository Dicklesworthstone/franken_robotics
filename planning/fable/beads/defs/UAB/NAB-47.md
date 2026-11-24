---
key: "NAB-47"
title: "registries/ideas.toml: the nineteen contested ideas: both arguments, resolution, deciding measurement"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["N40-27", "NAB-42"]
needs: []
related: ["NAB-48"]
gates: []
plan_refs: ["Appendix B.5", "Appendix B.2", "§2.3", "§2.4"]
---
## Background

Some ideas behind FrankenRobotics were not merely examined; they were fought over. An idea is **contested**
when independent examiners reached opposite conclusions about it, or when the passes that proposed it
disagreed about its form. Nineteen ideas are in that state, and they include most of the mechanisms the
system's thesis depends on: stepping many different robots in one batch, sampling-based control as the
default controller, one policy across bodies, derivatives through contact, runtime shields, verdict reuse,
counterfactual forks, and the question of where a ten-fold gain could come from at all.

The rule for resolving a contested idea is the most important thing this record carries: **the resolution is
never a vote. It is a tier, a pre-declared test and a fallback.** For each idea the record holds the strongest
argument for it, the strongest argument against it, and what was done: the tier it was placed at, the gate
that measures it, the kill gate whose trigger was written before any measurement, and what the system falls
back to if the trigger fires. An implementer who thinks one of these mechanisms is obviously right, or
obviously wrong, should read the other side here first.

This bead carries the nineteen as records `CT-01` to `CT-19` of the idea-pass register
(`registries/ideas.toml`, schema `frob.ideapass/1`, NAB-42). Thresholds are TARGETs owned by their gates; a
ratio quoted from an argument is a HYPOTHESIS as proposed unless marked FACT. Tier of the bead: Solid (a
decision record).

## Scope

In: records `CT-01` to `CT-19`; the four standing disagreements with their measurements; the hand-checkable
numbers the arguments quote. Out: the schema and lint (NAB-42); the placed rows the records point to (NAB-44
to NAB-46); the decisions themselves (the decision beads of the owning units).

## Technical approach

**Transcription rule.** One record per row, in table order. Fields beyond the common ones: `argument_for`,
`argument_against`, `resolution` (verbatim), `decided_by` (the last column: the gate, the kill gate and the
beads that take the measurement and record the decision), `placed_rows` (the `IP` records the idea
corresponds to), `status` (`open` until the deciding gate is recorded; then `settled` with the measured
values, or `fell-back`). `counts_toward_tally = false`. `disposition` is `placed-narrowed` where the record has
placed rows, whose carriers and gates it shares; CT-12 is `declined` with `killed_idea = ["KI-16"]` (the
robot-neighbourhood form; its far-field sibling is UP-04); CT-19 is a program rule (`tiers` empty,
`tier_text = "Program rule"`, gates GATE-TENFOLD-01 to -13, kill gate KILL-PARITY, carriers from its last
column).

Correspondence to placed rows: CT-01 ↔ IP-12; CT-02 ↔ IP-03; CT-03 ↔ IP-04; CT-04 ↔ IP-16; CT-05 ↔ IP-13;
CT-06 ↔ IP-60; CT-07 ↔ IP-25; CT-08 ↔ IP-26; CT-09 ↔ IP-21; CT-10 ↔ IP-70; CT-11 ↔ IP-45; CT-12 has no placed
row (it is not built); CT-13 ↔ IP-33; CT-14 ↔ IP-37; CT-15 ↔ IP-74 and IP-75; CT-16 ↔ IP-35; CT-17 ↔ IP-02;
CT-18 ↔ IP-18; CT-19 has no placed row (it is the thesis).

**The records.**

| Idea | Strongest argument for | Strongest argument against | How the plan resolved it | Decided by |
|---|---|---|---|---|
| The batch unit for many different robots (IP-12) | Stepping many topologies in one launch removes a compile per design, and with few worlds per design it is the only way to fill a device | Homogeneity by morphology is not homogeneity by work: contact counts diverge inside a cohort; padding and bucket fragmentation eat the gain; a warm incumbent with per-family templates may win the early rounds | A four-arm head-to-head with outcomes declared in advance (§17.14): cohorts, nine per-family templates, an incumbent's mixed batch, and the leading incumbent design by design. KILL-PHYLUM makes the templates the engine if they win. The unit of measure is edit to first accepted result (§17.8) | BD-186 (four arms, GATE-PHYLUM-03), BD-184 (edit to first accepted result), N17b-10 (KILL-PHYLUM); read at BD-224 |
| Sampling-based control as the default controller (IP-03) | It removes training from the design loop, and against per-design training it is the clearest large factor any pass found | A fixed-budget planner ranks bodies that are easy to plan for above better bodies; incumbents already run it; and it costs thousands of times as many model steps per simulated second as a policy rollout (DESIGN arithmetic on published FACTs, §22.8) | [F], as a screening rung of three episodes per design by default and as a demonstration source. It never stands in for a trained policy as the evaluator of record (FROB-REF-817, NONGOAL-14); the delivered-controller lane of §22.8 is the one exception. Its results carry `model-privileged` unless the planning model was drawn independently. GATE-REFLEX-06; KILL-MPC | BD-331 (GATE-REFLEX-06), N22c-16 (KILL-MPC); read at BD-378 |
| Family policy across bodies (IP-04) | One training is amortized over a class, which is where breadth per GPU-hour comes from | The pretraining budget is unowned; published transfer across morphologies is partial; novel hybrids are out of distribution; one proposer cut its own version and another demoted it after cross-examination | [F], per morphology class only (NONGOAL-10), above the exact skill layer of §22.10. GATE-CORTEX-11 and KILL-FAMILY decide it; a rung may discard designs only after rank agreement is measured (GATE-CORTEX-12; top-weighted Kendall τ thresholds 0.7 and 0.5) | BD-321 (GATE-CORTEX-11), BD-328 (GATE-CORTEX-12), N23a-26 (KILL-FAMILY); read at BD-378 |
| Derivatives through the contact solve (IP-16) | At a converged convex solve the adjoint is one back-substitution, and local margin sensitivities are a sound first consumer | The derivative is biased at make and break; the sibling's event-sensitivity code refuses simultaneous events, which a trot produces by design; a closed-loop design gradient also needs the controller's sensitivity | Not in the rigid core before M6 (NONGOAL-15). Fixed-mode derivatives on smooth kernels only, each with a validity label (D-8, C-16, §16.12). Return-map gradients under GATE-GAIT-02. Paired finite differences over short horizons are [F] under KILL-FDGRAD | BD-334 (GATE-GAIT-02), BD-353 with N17b-20 (KILL-FDGRAD), BD-422 at M6 |
| Fused per-world step kernel (IP-13) | Memory traffic is the lever, and once a sampling controller is in the loop stepping speed multiplies into every decision | The step is bound by latency and occupancy, not by operation count; one world's working set fills a block, so occupancy falls to a few percent of lanes; the proposed throughput was conceded | [M], behind a flag. KILL-MEGA is a one-week pilot with a paired lower bound of 2.0 on the whole step; the staged step plan is the design of record | N21c-21 (the one-week pilot), N21c-22 (KILL-MEGA), BD-441 |
| Staged algebraic erasure as a throughput lever (IP-60) | Doing the algebra at compile time makes the run-time step strictly cheaper | It restates a position the plan already held; the operation count falls by 18,000/9,600 = 1.875 on a step that is not bound by operation count; its falsifier bundled four treatments | Kept as a rule and not counted as a lever: checks at model compile time, no tags in kernels (C-02, C-14). Per-design specialization is for survivors only (KILL-SPECIALIZE) | No measurement of its own: it is a rule. Its only speed claim is decided by BD-371 and N21c-16 (KILL-SPECIALIZE) |
| Runtime shields from barrier conditions (IP-25) | No model-level violation during exploration, and the same filter lowers to firmware | Three demonstrated defects: a "for every disturbance there is a control" condition admits a clairvoyant controller; an infimum over the smaller friction cone is optimistic for forces the environment chooses; a closed-form projection that ignores the box returns (1.25, 0.25) where the optimum is (1, 0.5) (FACT, hand-checkable; the counterexample catalogue) | [F], rewritten: one control for all disturbances and estimation errors; separate cones for controlled and environment forces; sort-and-clip for one row, a projected solve with a checked KKT certificate and a verified backup otherwise. Evidence class Tested. GATE-SHIELD-02, -03; KILL-SHIELD. The energy budget (IP-23) comes first | BD-338 (GATE-SHIELD-02), BD-339 (GATE-SHIELD-03), N22b-28 (KILL-SHIELD) |
| A probability bound from assumption monitors (IP-26) | Splitting a guarantee at assume-and-guarantee interfaces keeps the proof part honest about what it assumes | The bound cannot validate itself: a monitor cannot label its own silent breaches; distribution shift moves the miss rate; four independent feet with three states each give 27 strata and 162 transitions under the symmetry group, not 14 (FACT by count, C-16) | The typed product is [S] and is in every output envelope. The bound is reported only with breach labels drawn from hidden state independently of the monitor, with shifted worlds reported separately (GATE-SHIELD-03) | BD-339 (independent breach labels, GATE-SHIELD-03), BD-308 (the typed product at the boundary) |
| Observation-alias witness as a new capability (IP-21) | It shows before any training that every policy on an interface must fail, and names the cheapest fix | Deriving the admissible action sets from a contact-rich plant is most of the work; given the same sets, an ordinary exact solver recovers every frontier (FACT, the planning review log) | Adopted as a checked witness, with no claim of a new algorithm. Structural and enumerated witnesses are [S]; searched witnesses are [F] and labelled experiment-specific. One false witness triggers KILL-ALIAS | BD-240 (searched lane), N20a-43 (KILL-ALIAS: one false witness) |
| Joint paired-outcome certification (IP-70) | It closes whole parameter boxes of a design comparison at once | A conventional reachability tool handles the same product model, and one mislabelled draw voids the certificate | [F], on one contact transition. KILL-AUGUR-DOMINANCE falls back to paired replay; the rule that shared inputs are indexed by position and not by tick is kept in either case | BD-359 (GATE-AUGUR-16), N25c-27 (KILL-AUGUR-DOMINANCE) |
| Verdict reuse beyond a keyed cache (IP-45) | Four passes proposed it; repeated work in a campaign is real | Executed check: a correctly keyed dependency graph skipped exactly the work the richer receipt skipped, and omitting one dependency inherited a false promotion (FACT, the planning review log) | The memo graph is [S] with a shadow recompute. Reuse by implication is [F] under KILL-REUSE and KILL-AUGUR-REUSE: one unsafe reuse removes the rule. Reuse by sensitivity margin is withdrawn (the killed-idea register) | BD-292, BD-347 (GATE-ATEL-11), N28a-23 (pilot) and N28b-36 (KILL-REUSE), N25a-19 (KILL-AUGUR-REUSE) |
| Recomputing only what an intervention touches | A fork that recomputes only the descendants of its dirty frontier would make interventions cheap: a factor of 1/(0.05 + 0.019 + 0.01) = 12.7 if 95% of replay is addressable and 2% is dirty (HYPOTHESIS as proposed) | In a robot-centric scene the robot and its contact neighbourhood dominate compute and are always dirty; the dependency surface is large; full replay is simpler and exact | Not built. Forks copy full state by index, and a prefix is reused only when identical by construction (§29.8). A far-field lane is listed in the not-yet-placed list (NAB-49) | Not built. The far-field lane was later given a pilot: N19c-33 with KILL-FARFIELD (GATE-HAB-15) |
| Counterfactual branching: what a fork may claim (IP-33) | Seven passes proposed it; it tells an agent which experiment to try next | Carrying state across a geometry, inertia or gearing edit is invalid; a mid-run rescue is conditional on a state the edited robot might never reach; coalition values can be negative and cannot establish necessity | [S], narrowed: diagnostic interventions fork from the snapshot; design-parameter edits re-run from the episode start (§27.8); forks are classified and the invalid ones refused (GATE-AUTOPSY-02); placebo forks and the flip rate bound instability (§27.5, §27.6); attributions are labelled and are never cited as established (FROB-REF-880) | BD-205 (GATE-AUTOPSY-02), BD-279, BD-280 (GATE-AUTOPSY-03), BD-283; N27b-11 (KILL-AUTOPSY) |
| Structure and manufacture inside the loop (IP-37) | Distal link inertia changes dynamic rankings, the sibling structural crates exist, and a design that cannot be built is not a result | It adds scope and not decision speed; loads taken from compliant contact are artifacts of contact stiffness; sibling readiness was overstated and conceded | [F], declared an output-quality subsystem and not a speed lever (§31.2). Load cases are time-coherent and come from referee-tier telemetry (§31.6). Each stage has a kill gate with a simpler fallback | BD-400 to BD-406 with N31b-16 (KILL-LOADS), N31b-15 (KILL-TOPOPT), N31b-17 (KILL-FAB) and N31b-18 (KILL-TOLERANCE), recorded at BD-412 |
| Catalog co-design by monotone bounds (IP-74, IP-75) | Discrete component selection is largely monotone, cheap to test on a CPU, and a sound cut on a partial assignment saves point checks multiplicatively | Monotonicity breaks under dynamics; a quasi-static filter can mis-rank; a shared resource is joint and time-dependent | [S] for catalog closure, [F] for antichains and cuts. KILL-SYNTH-CODESIGN fires on one falsely pruned design or on more than 20% of top designs mis-ranked; KILL-AUGUR-CUTS on one false cut | BD-360 with N25d-11 (KILL-SYNTH-CODESIGN); BD-357 with N25b-16 (pilot) and N25b-17 (KILL-AUGUR-CUTS) |
| The estimator for cheap referee promotion (IP-35) | Coupled estimation reuses an existing sibling estimator and gains (1 − ρ²); randomized audits stay unbiased when promotion targets are chosen after seeing fast-tier results | A unit-coefficient correction does not have the quoted variance; ρ falls on long contact episodes; inverse-propensity weights can exhaust the budget | Both, as one estimator: a fitted, frozen control-variate coefficient with a doubly robust audit correction on logged propensities (C-25). KILL-DR at ρ below 0.7 on 200 addresses falls back to the plain paired promotion | BD-264 (GATE-FID-08, GATE-CHRON-09), N16b-10 (KILL-DR) |
| Sharing test cases across symmetry orbits (IP-02) | An exact saving with no approximation | Interiors are asymmetric; floating point and unmirrored noise break exact invariance; the saving is on work one would not create | Orbits are taken only under the group that preserves robot, sensors, task and scene, and members have equal outcome distributions, not equal outcomes at equal addresses (§17.11). The known answers bound the lever: 86,185 orbits of 10⁶ labelings under the full dihedral group, 500,500 under a mirror alone. KILL-QUOTIENT below 5% of dispatches saved | BD-285 (GATE-PHYLUM-01), N17a-29 (KILL-QUOTIENT) |
| Clearance complexes (IP-18) | Fits, cannot fit or unresolved before any dynamics, reusable across a footprint family | It covers wheeled or rigidly folded bodies only; a conventional planning library could hold the same index; its share of design time is small and disputed | [F], two-sided with a reported band; a nonholonomic base without a steering primitive is labelled geometric only. KILL-FCLEAR falls back to a route-graph check labelled heuristic | BD-233 (GATE-HAB-10), N19b-36 (KILL-FCLEAR) |
| Where the ten-fold comes from | Kernel throughput multiplies into every other mechanism | An infinitely fast environment step speeds training by at most about 7.7 times (1/(1 − 0.87)) on one incumbent stack and about 1.4 on another (FACT, §2.3) | Engine throughput is a parity requirement (KILL-PARITY); the ten-fold is sought in avoided trainings, family answers and fewer agent rounds (§2.4), and is measured, not argued (§39) | BD-217 (the parity rows), N36-27 (KILL-PARITY), BD-314 to BD-317 (partial gate at M3), BD-428 to BD-433 (tournament at M7) |

**Four disagreements did not move under cross-examination.** The planning record settles none of them by
argument; each has a measurement:

| Standing disagreement | Record | Measurement that settles it |
|---|---|---|
| Whether kernel speed is undervalued | CT-19 | The parity rows at M2 (kill trigger: a physics row below 0.33 of the incumbent); the phase ledgers of the partial gate at M3 and of the tournament at M7 |
| Whether closed-loop design gradients can work | CT-04 | The paired finite-difference pilot (kill trigger: variance ratio below 2 at every horizon of 50 steps or more, or sign agreement below 70%, or no end point confirmed); the M6 review of gradients through contact |
| Whether randomized audits have a regime in which they beat coupled estimation | CT-16 | The tier-correction estimator at its gates (kill trigger: ρ below 0.7 on 200 paired addresses in every stratum tested) |
| Whether orbit sharing saves real work | CT-17 | The quotienting gate (kill trigger: one false quotient surviving a repair cycle, or digest classes saving less than 5% of dispatches on the campaign corpus) |

**Hand-checkable numbers quoted in the arguments** (stored as `[[idea.calculation]]`, evaluated by the same
harness as the killed-idea register):

| Record | Expression | Value |
|---|---|---|
| CT-06 | operation-count ratio of staged erasure: 18,000/9,600 | 1.875, on a step that is not bound by operation count |
| CT-07 | closed-form projection of (1, 0) onto u₁ + u₂ ≥ 1.5, ignoring the box [0, 1]²; the true optimum | (1.25, 0.25); (1, 0.5) |
| CT-08 | strata of four independent feet with three states each, up to the symmetry group C₂ × C₂: (3⁴ + 3·3²)/4; directed single-foot transitions: 81 × 4 × 2 ÷ 4 | 27 strata and 162 transitions, not 14 |
| CT-12 | the proposed saving of recomputing only what an intervention touches: 1/(0.05 + 0.019 + 0.01) if 95% of replay is addressable and 2% is dirty | 12.7 (HYPOTHESIS as proposed) |
| CT-17 | orbits of 10⁶ labelings (ten labels on a ring of six positions) under the full dihedral group of order 12; under one mirror alone | 86,185; 500,500 |
| CT-19 | the most that an infinitely fast environment step speeds training, at step shares of 87% and 29%: 1/(1 − s) | about 7.7; about 1.4 |

The dihedral count by orbit counting: the identity fixes 10⁶ labelings; rotations by one and five positions fix
10 each; by two and four, 10² each; by three, 10³; three reflections through opposite positions fix 10⁴ each;
three through opposite edges fix 10³ each; the sum 1,034,220 divided by 12 is 86,185. Under one mirror through
opposite edges, (10⁶ + 10³)/2 = 500,500.

## Acceptance criteria

- [ ] Records `CT-01` … `CT-19` exist with both arguments, the resolution and `decided_by` verbatim, and
      `placed_rows` as listed.
- [ ] Every bead and gate named in `decided_by` exists; every kill gate named in a resolution exists in the
      gate register with a trigger whose numbers equal those quoted (KILL-MEGA: a paired lower bound of 2.0;
      KILL-DR: ρ below 0.7 on 200 addresses; KILL-QUOTIENT: below 5%; KILL-SYNTH-CODESIGN: one falsely pruned
      design or more than 20% of top designs mis-ranked; KILL-AUGUR-CUTS: one false cut; the rank-agreement
      thresholds 0.7 and 0.5); so do the triggers quoted for the standing disagreements (KILL-PARITY: below
      0.33 of the incumbent; KILL-FDGRAD: variance ratio below 2 at every horizon of 50 steps or more, or sign
      agreement below 70%).
- [ ] The four standing disagreements each point to a record and a measurement, and their `status` is `open`
      at plan time.
- [ ] The harness recomputes the six calculation rows; the dihedral count is also checked by brute force.
- [ ] No record's `status` is `settled` without the measured values and the gate record id.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `contested`:

- `nineteen_records`; `placed_rows_resolve`; `two_records_have_no_placed_row` (CT-12, CT-19).
- `decided_by_resolves` (beads and gates).
- `kill_trigger_numbers_match_register`: the six quoted triggers, and the KILL-PARITY and KILL-FDGRAD
  triggers of the standing disagreements, against `registries/gates.toml`. KILL-DR is entered at M0; the
  other rows are entered at M1 and are compared under a recorded `skip` until then (the rule of NAB-17 step 6).
- `ct06_ratio`: 1.875. `ct12_factor`: 1/0.079 = 12.66. `ct19_ceilings`: 7.69 and 1.41.
- `ct07_projection`: (1.25, 0.25) violates the box; the exact optimum is (1, 0.5).
- `ct08_strata_and_transitions`: enumerate 3⁴ = 81 labelings and 648 directed single-foot transitions; orbits
  under the three double transpositions number 27 and 162.
- `ct17_orbits`: brute-force canonical forms of 10⁶ labelings of a six-ring with ten labels under the dihedral
  group give 86,185 classes; under one edge mirror 500,500.
- `standing_disagreements_are_four_and_open`.
- `rows_frozen` against `tests/fixtures/ideas/contested.tsv`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 19 records, recomputes the calculations (the orbit count
runs in under a second), and prints the four standing disagreements with their status. Logged per STD-E2E.

### Known answers and negative controls

Known answers: 19 records; 4 standing disagreements; the six calculation rows; 1,034,220/12 = 86,185.
Negative controls: a record marked `settled` by a sentence of argument with no gate record
(`idea.state-ahead-of-gate`); a resolution edited to remove its fallback (`idea.upper-tier-without-kill`); a
kill trigger restated more kindly than the register ("below 0.6" where the register says "below 0.7") caught
by `kill_trigger_numbers_match_register`; the orbit count reported as 10⁶/12 = 83,333 (the naive division that
orbit counting corrects).

## Considerations

- These records are the place to send anyone who wants to reopen an argument: the argument has been had, and
  what is missing is the measurement. The sweeps (NAB-25, NAB-26, NAB-27, NAB-30) record the measurements as
  they arrive.
- CT-12 is "not built" and has a later, narrower sibling (far-field field reuse across forks, with its own
  pilot and kill gate). The record says so in its last column so that the two are not confused.
- CT-19 is the thesis itself: engine throughput is a parity requirement, and the ten-fold is sought in avoided
  trainings, family answers and fewer agent rounds, and is measured, not argued.

## References

Appendix B.5; Appendix B.2; §2.3; §2.4.
