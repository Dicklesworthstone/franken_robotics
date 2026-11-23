# Coverage of drafting unit U10c (chapter 10, constructions C-28 to C-40)

Files in this directory: 13 specification beads `SPEC-C-28` to `SPEC-C-40`, 10 new beads `N10c-01` to `N10c-10`,
the registry bead `N10-CX`, and this file. No seed bead is assigned to this unit.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads | What each bead carries |
|---|---|---|---|
| §C-28 | A failure has a minimal counterexample world | SPEC-C-28 | The order-ideal lattice, the test relation, 1-minimality, delta debugging with the final descent, the pair check, the solvability witness, the pilot |
| §C-29 | A deterministic forkable simulator is a causal model | SPEC-C-29, N10c-01 | SPEC-C-29: the causal-model reading, abduction, action, prediction, fork laws, the snapshot-versus-episode-start rule. N10c-01: placebo flip rate, probability of necessity, the sequential naming rule, labels, coalitions and the Shapley summary |
| §C-30 | Requirements are temporal logic over unit-normalized predicates | SPEC-C-30 | The margin table, soundness, unit invariance, streaming and block algorithms, `undetermined`, the bounded-future rewrite |
| §C-31 | Safety is an energy budget first, a filter second | SPEC-C-31, N10c-02, N10c-03 | SPEC-C-31: tier separation and tier 1 (the budget, premises, exclusion reading, monitor rule). N10c-02: tier 3 (rows, quantifier order, split cones, exact one-row projection, KKT check, residual risk). N10c-03: tier 2 (tube inclusion, erosion by −E, viability pruning) |
| §C-32 | Region certificates and dual witnesses of infeasibility | SPEC-C-32, N10c-04 | SPEC-C-32: Farkas and conic witnesses, the stance family in inverse geometry, the full specimen. N10c-04: enclosures, per-constraint assembly, paving limits, the 12-parameter pilot |
| §C-33 | Clearance is certified from both sides | SPEC-C-33 | The cell bound, inner and outer complexes, witness and cut, the width filtration, checker rules, the ladder and doorway cases |
| §C-34 | Linkage synthesis: an exact bounded lane and a continuation lane | SPEC-C-34, N10c-05 | SPEC-C-34: lane 1 (dyads, capped templates with saturation, circuit and branch certificate). N10c-05: lane 2 (parameter homotopy, the α-test, reality and distinctness, `Complete` or `Partial`, known counts) |
| §C-35 | Structure and life: topology, fatigue, tolerance | SPEC-C-35, N10c-06, N10c-07 | SPEC-C-35: the frame and topology optimization. N10c-06: rainflow, linear damage, Basquin, carried residues. N10c-07: stacks, cost-optimal allocation, the zero-sensitivity rule |
| §C-36 | The trapezoidal variational step and the discrete nonholonomic step | SPEC-C-36, N10c-08 | SPEC-C-36: the Cayley differential, the four-step map and its laws, the trivialization control. N10c-08: rolling rows, the kinetic-metric projection, cone-bounded traction, the sleigh and skid-steer cases |
| §C-37 | The four-dimensional contact cone and joint-level rows | SPEC-C-37 | The cone and its dual, maximal dissipation, patch radius, projection identities, stop, stiction and backlash rows |
| §C-38 | Actuators are port-Hamiltonian with a thermal node | SPEC-C-38 | The port-Hamiltonian form, the motor axis, three electrical modes, the thermal node, entropy sign check, runaway |
| §C-39 | The value of an experiment, and the design of a probe | SPEC-C-39, N10c-09 | SPEC-C-39: the closed-form value, ranking, the stop rule, the calibration trigger. N10c-09: the seven-step probe procedure, disjoint reading sets, conditionality, both fixtures |
| §C-40 | Dominance is unreachability in a product system | SPEC-C-40 | The product system, unreachability, witness and `unknown`, joint coverage, position-indexed shared inputs, paired and absolute verdicts |
| Chapter 10, counterexamples (Appendix N.7) | CX-019 to CX-097 | N10-CX | All 79 entries with control kind, planting bead and gates |
| Chapter 10, "the spine is real" for C-28 to C-40 | Definition-of-done clause | N10c-10 | The end-to-end audit script and its map |

## (b) Gates, kill gates and admission gates

Chapter 10 defines no gate, kill gate or admission gate of its own: a construction fixes mathematics and its test,
and each gate lives in the owning chapter. The table lists the gates that measure the constructions of this unit,
the specification beads that supply their known-answer rows and controls, and the seed bead that records or
decides each (those beads are owned by other units).

| Gate or kill gate | Milestone | Rows and controls supplied by | Recorded or decided by |
|---|---|---|---|
| GATE-NEMESIS-03, GATE-NEMESIS-01 (oracle rows), KILL-MCW | M3 | SPEC-C-28 | BD-274, BD-271 |
| GATE-AUTOPSY-01, GATE-AUTOPSY-02, GATE-AUTOPSY-06 | M2, M2, M3 | SPEC-C-29 | BD-205, BD-283 |
| GATE-AUTOPSY-03, -04, -05, KILL-AUTOPSY | M3 | N10c-01 | BD-280, BD-281, BD-282 |
| GATE-ARENA-03, GATE-CORTEX-10, KILL-SPEC-REWARD | M1, M3, M3 | SPEC-C-30 | BD-121, BD-255, BD-257 |
| GATE-SHIELD-01, KILL-ENERGY | M4 | SPEC-C-31 | BD-337 |
| GATE-SHIELD-02, -03, -04, KILL-SHIELD | M4 | N10c-02 | BD-338, BD-339, BD-340 |
| GATE-SHIELD-05, KILL-TUBE | M8 | N10c-03 | BD-440 |
| GATE-AUGUR-02, GATE-AUGUR-05, GATE-LEMMA-03, GATE-LEMMA-04 | M1 | SPEC-C-32 | BD-130, BD-132, BD-133 |
| GATE-AUGUR-09, KILL-AUGUR-REGION | pilot M3, gate M4 | N10c-04 | BD-289, BD-358 |
| GATE-HAB-10, KILL-FCLEAR | M3 | SPEC-C-33 | BD-233 |
| GATE-AUGUR-17 (linkage rows), KILL-SYNTH-LINKAGE | M4 | SPEC-C-34 | BD-361 |
| GATE-AUGUR-22, KILL-SYNTH-CONT | M8 | N10c-05 | BD-437 |
| GATE-FOUNDRY-04, GATE-FOUNDRY-05, KILL-TOPOPT | M5 | SPEC-C-35 | BD-403 |
| GATE-FOUNDRY-03 | M5 | N10c-06 | BD-402 |
| GATE-FOUNDRY-08, KILL-TOLERANCE | M5 | N10c-07 | BD-406 |
| GATE-KIN-01 (variational rows), KILL-VARIATIONAL | M1, M5 | SPEC-C-36 | BD-088, BD-384 |
| GATE-TRAC-01 (rolling rows) | M1 | N10c-08 | BD-085 |
| GATE-SINEW-04, GATE-TRAC-04, GATE-SINEW-09, KILL-SINEW-ROWS | M1 and M2 | SPEC-C-37 | BD-084, BD-154, BD-155 |
| GATE-SINEW-01, -02, -05, -08 | M1 (M2 for single precision) | SPEC-C-38 | BD-070, BD-072, BD-076 |
| GATE-ATEL-15, KILL-VOI | M4 | SPEC-C-39 | BD-351 |
| GATE-AUGUR-11, KILL-AUGUR-PROBE, GATE-GROUND-03 | pilot M3, gate M4; M5 | N10c-09 | BD-291, BD-351, BD-390 |
| GATE-AUGUR-16, KILL-AUGUR-DOMINANCE | M4 | SPEC-C-40 | BD-359 |
| GATE-GAUNT-01 (controls exist and are caught) | every exit | N10-CX | BD-025, BD-146 |
| GATE-GAUNT-02 (no row counted with `to set` or `none stated`) | every exit; zero by M3 for rows due by M3 | N10c-10 and every specification bead | the Gauntlet harness |

## (c) Counts

- Known answers carried: 49 rows of Appendix M.9, KA-142 to KA-190, each stated with its value in the test plan of
  its specification bead (the chapter's registry bead `N10-KA` is written by unit U10a). Also cited with values:
  KA-033 (lifted against projected stance programs) and KA-213 (budget without its gravity term). Three rows that
  the verification chapter adopted late and that have no printed KA number are carried and appended: tube erosion
  with a one-sided error (N10c-03), reality of a certified root (N10c-05), the Cayley map on SE(3) (SPEC-C-36).
  New appended rows proposed by these beads: 2 in N10c-04, 2 in N10c-07, 2 in N10c-09.
- Of the 49 rows, those printed with tolerance `to set` now have a stated tolerance (KA-168, KA-171, KA-172 to
  KA-185, KA-187; for KA-180 the floating-point residual), and those printed with no negative control now have one (KA-143, KA-144,
  KA-145, KA-147, KA-148, KA-150, KA-152, KA-164, KA-167, KA-168, KA-169, KA-171, KA-174, KA-176, KA-177, KA-179 to
  KA-183, KA-185, KA-187). Every such value is labelled a DESIGN proposal for the owning lead to confirm.
- Counterexamples carried: 79 in `N10-CX` (CX-019 to CX-097), of which the 31 of this unit's constructions
  (CX-067 to CX-097) are also planted as named mutants in the law suites. Also cited: CX-012, CX-013, CX-016.
- Refusal codes: chapter 10 defines none. Codes of other chapters that the contracts rely on are named with their
  meaning in the beads (for example the unlisted-source refusal of the energy budget, the more-than-four-rows
  refusal of a shield, the borrowed-margin refusal, the null-fork and two-mechanism refusals of Autopsy, the
  requirement-language refusals, the caps refusal of exact synthesis, the fatigue no-claim and zero-sensitivity
  refusals of Foundry).
- Invariants, proof obligations, decisions, open questions and kill gates: chapter 10 defines none in this range.
  Those of the owning chapters are named where a contract relies on them (INV-AUTOPSY-01, -02, -05; INV-ARENA-01,
  -02; PO-ARENA-04; PO-SHIELD-01 to -04; INV-SHIELD-01, -02; INV-NEMESIS-04; ADR-AUGUR-03; ADR-ATEL-07;
  OPEN-SHIELD-1, -2; OPEN-AUGUR-2, -3; OPEN-AUTOPSY-1 to -4; OPEN-SINEW-2; OPEN-TRAC-2; OPEN-FOUNDRY-4).
- Theorem cards named with their statements: T-06, T-07, T-13, T-21, T-22, T-28, T-31, T-32, T-34.
- Metamorphic relations carried: MR-08, MR-19, MR-20, MR-22, MR-29, MR-34.

## (d) Needs from other slices

All blocking prerequisites were resolved to keys, so every `needs` list is empty. What these beads expect from
other units, without a blocking edge:

1. **Registry rows.** Unit U10a writes `N10-KA` (possibly split). The specification beads here create rows KA-142
   to KA-190 if they are absent and otherwise bind to them; either order works because numbers are append-only.
2. **Pilots and kill decisions.** No separate pilot bead was written here: each construction's pilot is stated in
   full in its specification bead and is run by the seed bead of the mechanism, which also decides the kill gate:
   C-28 by BD-274; C-31 by BD-337 to BD-339; C-32 by BD-289; C-33 by BD-233; C-34 by BD-361 and BD-437; C-35 by
   BD-403 with BD-402 and BD-406; C-36 by BD-384 and BD-087; C-39 by BD-351, BD-344 and BD-291; C-40 by BD-359.
   If a subsystem unit did not write a pilot bead for one of these, the pilot text in the specification bead is
   complete enough to lift into one.
3. **Auditor checkers** for the proof kinds the contracts name: `energy-budget`, `tube-inclusion`, `lp-farkas`,
   `conic-farkas`, `lp-primal`, `interval-box`, `taylor-box`, `partition`, `complex-connectivity`,
   `product-unreachable`, `alpha-root`, `trace-witness` (BD-132 and the mechanism beads).
4. **Theorem-card beads** `CARD-T-06`, `-07`, `-13`, `-21`, `-22`, `-28`, `-31`, `-32`, `-34` (named in `related`).
5. **Production bindings.** Each law suite is generic over a subject trait; a mechanism bead that implements a
   construction after its specification bead has closed must bind its implementation to that suite. The beads
   name the implementers: BD-274, BD-205, BD-279 to BD-283, BD-121, BD-255, BD-337 to BD-340, BD-440, BD-130,
   BD-132, BD-358, BD-233, BD-361, BD-437, BD-402, BD-403, BD-406, BD-087, BD-384, BD-080, BD-084, BD-154,
   BD-155, BD-070 to BD-072, BD-076, BD-351, BD-291, BD-359.

## (e) Contradictions, gaps and what was done

1. **Dependency direction.** The brief makes a specification bead depend on the seed bead that creates the crate
   its law tests live in, while mechanism drafters were told to add `SPEC-C-nn` to the dependencies of beads that
   implement a construction. Where the crate-creating seed is also the implementer, both edges may appear and
   form a cycle. Edges of this unit to watch: SPEC-C-30 → BD-121; SPEC-C-36 → BD-087; SPEC-C-31 → BD-337;
   SPEC-C-32 → BD-128; SPEC-C-40 → BD-128; SPEC-C-38 → BD-069; SPEC-C-39 → BD-262; lower risk: SPEC-C-28 → BD-270,
   SPEC-C-33 → BD-162, SPEC-C-34 → BD-360, SPEC-C-35 → BD-400, SPEC-C-29 → BD-090. Each law suite is generic over a
   subject trait and passes first on a reference model, so either order is workable: if the lead keeps the
   mechanism → specification edge, drop the specification → seed edge and let the mechanism bead bind the suite.
2. **Crate-creating seeds are not always identifiable.** No seed title names `frob-dyn`, `frob-opt`,
   `frob-shield`, `frob-autopsy`, `frob-nemesis`, `frob-actuate` or `frob-foundry`. The earliest seed working in
   each was used (BD-087 with BD-011; BD-262; BD-337; BD-279; BD-270; BD-069; BD-400).
3. **Milestones where a construction spans several.** C-29: core at M2 (first gate GATE-AUTOPSY-01), attribution
   at M3. C-32: duals and stance at M1, regions at M3. C-37: one bead at M1 (joint rows are gated at M1) with the
   cone bound at M2. C-39: M3, because its first consumer is the M3 probe pilot, although both its gates are at M4.
   C-31 and C-34: Moonshot parts at M8.
4. **Energy budget, gravity term (C-31).** The plan prints 8 × 9.81 × 0.040 × 0.08 = 0.25 J; the product is
   0.251136 J. KA-153 keeps the printed values; SPEC-C-31 records the discrepancy and a fail-closed reading.
5. **Pilot sizes and wording differ between chapter 10 and the owning chapters.** C-34 lane 1: 24 templates here,
   240 instances in the Augur chapter and its gate. C-40: "a sole exchange over a split-grip strip" here, "a sole
   replacement on a single step edge" there. C-36: the cost pilot here, an energy-order pilot there. Both versions
   are carried; nothing was reconciled.
6. **Two fixtures for one formula.** C-34 circumcentre and four-bar, C-30 unit normalization, C-38 thermal runaway
   and C-37 backlash closure each appear with different numbers in chapter 10 and in the owning chapter. Both are
   carried as separate fixture identities.
7. **Naming of the variational-step control (C-36).** The plan calls the defect "the differential at +h·ξ_k",
   "the right-trivialized form" and "a right-trivialized differential"; by the identity proved in the bead the
   right-trivialized differential at +h·ξ_k is the correct operator. The contract names the operator by matrix
   and argument.
8. **Labels of attribution (C-29).** The construction names three labels; the Autopsy chapter's table has seven.
   The contract adopts the seven. The "exact one-sided test" is unnamed in the plan; Fisher's exact test was
   chosen as a DESIGN proposal.
9. **Sign of the backlash coordinate (C-37).** Chapter 10 writes q_out − q_motor/N and the owning chapters the
   negative; the rows are identical; both names are recorded.
10. **Incomplete fixtures completed by DESIGN.** C-38's runaway fixture has no heat capacity or temperatures;
    C-36's sleigh and skid-steer cases have no numbers; C-31's pre-slip aliasing control has no numbers anywhere.
    Each bead supplies numbers, marks them as derived or proposed, and requires an independent check before they
    are registered.
11. **Tolerance allocation, "k = 3, budget 10 µm" (C-35).** Read as a stack limit of 30 µm, which reproduces every
    printed digit.
12. **Rounding (C-32).** "0.64 of each side" is 0.637 for 13 bisections and 0.631 for exactly 10⁴ boxes; both are
    stated.
13. **Verb for clearance.** The verb registry gives `scene.fit` for the clearance complex; one seed title pairs
    the complex with `contact.clearance`, which the registry gives to the enclosure test. SPEC-C-33 uses
    `scene.fit`.
14. **Known answers with FACT status.** The stance grid counts, the probe pilot counts and the literature root
    counts are re-run or flagged for re-reading in the law suites before any gate may cite them.

## Integration pass (group G10)

- Item 1 of part (e) is settled by the order-of-work rule in section 6 of `N10a-01`: every specification
  bead and part of this unit depends on `N10a-01` and no longer on the seed that creates its crate or
  implements the construction (now `related`); the implementers named in item 5 of part (d) depend on the
  specification, and those edges are recorded in `integration/out/G10.json`.
- `SPEC-C-28` to `SPEC-C-40` fill their rows of the construction register and follow the ten-section contract
  template, as the other twenty-seven specification beads do.
- New bead `N10c-11`: the pilot of C-39 (thirty registered experiments), which item 2 of part (d) had left to
  mechanism beads that carry only the run-time calibration trigger.
- The known-answer rows KA-142 to KA-190 are in `N10-KA3`; the audit for C-01 to C-15 is `N10a-22`.
