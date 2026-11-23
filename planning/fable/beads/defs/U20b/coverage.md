# Coverage of drafting unit U20b (§20 Sensorium, subsections §20.13 to §20.25)

Bead files in this directory: 7 seed beads (BD-099, BD-102, BD-119, BD-187, BD-215, BD-241, BD-242), 44 new beads (N20b-01 to N20b-44) and 2 registry beads (N20-CX, N20-REF). Total 53. The known-answer registry N20-KA is written by unit U20a.

By kind: 10 features (verbs and sensor capabilities) and 43 tasks, of which 10 are end-to-end script beads (N20b-03, -07, -11, -15, -20, -22, -31, -34, -37, -42), 5 are open-question decisions, 1 is a kill-gate decision, 1 is a pilot and 5 are registries or indexes (N20-CX, N20-REF, N20b-17, N20b-19, N20b-21). By milestone: M1 11, M2 19, M3 17, M4 4, M5 2.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §20.13 | The observation-preserving symmetry subgroup [S] | BD-102 (`analyze.symmetry`, A-03, breakers, near-symmetry repairs, the stage-5 assertion; M1), N20b-01 (the five predicate families with timing; PO-SENS-05 closure card), N20b-02 (INV-SENS-08 guard, FROB-REF-443), BD-241 (`sense.symmetry`, the stored tuple, the equivariance test), N20b-03 (end to end, GATE-SENS-10). The group algorithms of the Marrow chapter are another unit's (N13-15, N13-16) |
| §20.14 | Calibration as a least-squares pose graph [S] | N20b-04 (graph, residual, tail-adjoint rule, Gauss–Newton, covariance), N20b-05 (W-orthogonal consistency residual, bump against bad edge, graph support, FROB-REF-446, PO-SENS-08), N20b-06 (the extrinsic bump as a fault; the post-impact self-check), BD-242 (`sense.calibrate` in simulation, the gate fixture), N20b-07 (end to end, GATE-SENS-11), N20b-08 (imported logs at M5, the delay-offset alias) |
| §20.15 | The causal-policy test [S] | N20b-09 (four kinds of twin world), N20b-10 (five planted leaks; the frozen actor schema with privileged channels absent; FROB-REF-442, schema half), BD-119 (the test and `sense.causal-test` on the referee), N20b-11 (end to end, GATE-SENS-12), N20b-12 (batched tiers; no evaluation without a passing receipt) |
| §20.16 | The docking fixture: a known answer with a closed-form boundary | N20b-13 (model, derivation, boundary, 561-cell grid, worked commands, equality, late packet; PO-SENS-06), N20b-14 (third gain, verdict binding, the three arithmetics). Consumers written elsewhere: BD-134 and BD-190 (U20a), BD-129 (mission chapter), BD-030 (kernel classes) |
| §20.17 | Noninterference [S] | BD-215 (five properties, a control per property, FROB-REF-442 presentation half, the every-tier record of GATE-SENS-12), N20b-15 (end to end, GATE-SENS-03), N20b-12 (the batched causal test used by the truth property) |
| §20.18 | Verbs and refusals | N20b-17 (the cluster's eight registry rows and the two telemetry verbs' sensing side), N20b-16 (`sense.profile`, complete), BD-099 (`sense.profile` v0), BD-119 (`sense.causal-test`), BD-241 (`sense.symmetry`), BD-242 and N20b-08 (`sense.calibrate`), N20b-33 (`sense.visibility`), N20-REF (the seven codes and three code-less refusals), N20b-18 (FROB-REF-447). `sense.schedule`, `sense.observability`, `sense.alias`: U20a seeds BD-078, BD-120 and BD-239, BD-134 and BD-240. `telemetry.stills`, `telemetry.inspect-pixel`: BD-191 |
| §20.19 | Invariants and proof obligations | N20b-19 (index of all eighteen with evidence kind, test and control, and a completeness check). Carried in mechanism beads: INV-SENS-01, -06, -09 in BD-215; INV-SENS-07 in N20b-10; INV-SENS-08 in N20b-02; INV-SENS-02 in BD-119 (causal half); INV-SENS-05 in BD-099 and BD-187; PO-SENS-05 in N20b-01; PO-SENS-06 in N20b-13; PO-SENS-08 in N20b-05. Expected from U20a's mechanism beads: INV-SENS-02 (queue half), -03, -04; PO-SENS-01, -02, -03, -04, -07, -09 |
| §20.20 | Known answers | N20b-20 (the 27 chapter cases mapped to KA numbers, gates and owners, plus three late slots; one battery script). Numbered rows: N20-KA (U20a). Cited with values in every mechanism bead |
| §20.21 | Gates and kill gates | N20b-21 (twenty-two registry rows with thresholds, milestones and controls); measurement in the beads of table (b) |
| §20.22 | Open questions | N20b-23 (OPEN-SENS-1), N20b-24 (OPEN-SENS-2), N20b-25 (OPEN-SENS-3), N20b-26 (OPEN-SENS-4), N20b-27 (OPEN-SENS-5) |
| §20.23 | Force sensors act on a force history: filter, clip and integrate in a declared order (late) | N20b-28 (card chain, stage semantics, CPU reference, rectangular rows), N20b-29 (grades `resolved`, `modeled`, `none`; pulse models; `overload: unknown`; undetermined atoms; the code-less refusal), N20b-30 (`K-sense-ft` on batched tiers; the 2% row), N20b-31 (end to end, GATE-SENS-15), N20b-16 (profile listing) |
| §20.24 | Visibility under self-occlusion, and sensor mounts as design variables (late) | N20b-32 (query, predicate, lead, blind interval, blind dwell, attribution, hand case), N20b-33 (`sense.visibility`, A-14 listing, stage-7 refusal with ranked repairs), N20b-34 (end to end, GATE-SENS-16), N20b-35 (pilot), N20b-36 (mounts as optimizer variables, Frontier), N20b-37 (GATE-SENS-17), N20b-38 (KILL-VISIBILITY) |
| §20.25 | Sampled readings: impacts in an exposure, encoder unwrapping, the load-cell cut (late) | N20b-39 (integrating accelerometer with the jump term), N20b-40 (unwrapping condition, `ambiguous`, the two-speed alias pair), N20b-41 (load-cell cut by momentum balance), N20b-42 (end to end, GATE-SENS-18), N20b-16 (profile's largest unambiguous speed) |

Seed beads of this unit that draw on subsections owned by U20a, written completely here: BD-099 (§20.5: IMU with lever arm, encoders, cards, on the referee), BD-187 (§20.6: height scan, depth, lidar on batched tiers) with its children N20b-43 (the ray kernel family), N20b-44 (batched proprioception with addressed noise) and N20b-22 (end to end, GATE-SENS-01 and -02).

## (b) Gates, kill gates and admission gates

| Gate | Milestone (register governs) | Measured or decided by |
|---|---|---|
| GATE-SENS-01 | M2 (referee IMU rows at M1) | BD-099 (referee IMU and turntable rows), BD-187 (record), N20b-43 (depth against analytic scenes at kernel level), N20b-44 (IMU on every tier), N20b-22 (script) |
| GATE-SENS-02 | M2 | BD-187 (record), N20b-22 (script), N20b-43 and N20b-44 (differential checks) |
| GATE-SENS-03 | M2 | BD-215 (record), N20b-15 (script) |
| GATE-SENS-04 | M1 (CPU); M2 (every tier) | Expected from U20a: BD-078, BD-189. Decision input: N20b-23 (OPEN-SENS-1, the default it must name) |
| GATE-SENS-05 | M1 (CPU); M2 (every tier) | Expected from U20a: BD-098, BD-189 |
| GATE-SENS-06 | M2 | Expected from U20a: BD-188 |
| GATE-SENS-07 | M3 | Expected from U20a: BD-237 |
| GATE-SENS-08 | M1 (register: "docking rows"); M3 | Expected from U20a: BD-120, BD-239 |
| GATE-SENS-09 | M1 (docking rows); M3 | Expected from U20a: BD-134, BD-240. Fixture supplied here: N20b-13 |
| GATE-SENS-10 | M3 | BD-241 (record), N20b-01, N20b-02, N20b-03 (script) |
| GATE-SENS-11 | M3 | BD-242 (record), N20b-04, N20b-05, N20b-06, N20b-07 (script) |
| GATE-SENS-12 | M1 (CPU); M2 (every tier) | BD-119 (referee rows), N20b-09, N20b-10, N20b-11 (script, all tiers); N20b-12 and BD-215 (every-tier record) |
| GATE-SENS-13 | M2 | Expected from U20a: BD-190 (devices, byte identity). Host rows here: N20b-13, N20b-14 |
| GATE-SENS-14 | M3 | Expected from U20a: BD-238 |
| GATE-SENS-15 (late; not in the register's subsystem table) | M2 | N20b-28, N20b-29, N20b-30, N20b-31 (script and record) |
| GATE-SENS-16 (late) | M3 | N20b-32, N20b-33, N20b-34 (script and record) |
| GATE-SENS-17 (late) | M4 | N20b-37 (inputs from N20b-36) |
| GATE-SENS-18 (final-audit table) | M2 | N20b-39, N20b-40, N20b-41, N20b-42 (script and record) |
| KILL-ALIAS | M3 | Expected from U20a (its seed BD-240 decides it) |
| KILL-SCHED | M2 | Expected from U20a (BD-189) |
| KILL-EXPOSURE | M2 | Expected from U20a (BD-188) |
| KILL-VISIBILITY (late) | M4 (the milestone of the gate beside it) | N20b-38 (decision), N20b-35 (pilot trigger), N20b-37 (gate trigger) |

All twenty-two rows are also written into `registries/gates.toml` by N20b-21, which measures none of them (its `gates` field is left empty on purpose so that a gate nobody measures stays visible). Gates of other chapters that beads here serve: GATE-MARROW-03 and GATE-ATEL-02 (BD-102), GATE-GAUNT-11 (the single-edge row; N20b-04, N20b-07), GATE-TETHER-02 (the cluster's rows; N20b-17). No admission gate (`INT-…`) is defined in these subsections.

## (c) Counts carried

- Known answers: all 60 numbered rows of the chapter (KA-465 to KA-524) are cited by number in this unit's beads, 37 of them with their values and controls in mechanism and test beads of this unit (KA-465 to KA-471, KA-473 to KA-476, KA-483, KA-486, KA-493 to KA-511, KA-513, KA-519, KA-520, KA-522, KA-524); the other 23 belong to U20a's mechanisms and are mapped to their owners in N20b-20. Plus 3 late answer groups without numbers (the force-chain rows, the box-and-mast case, the three sampled-reading rows), 1 late row of the adopted-fixtures table (one edge of the extrinsic graph), 1 adopted contact-strata row (27 and 45 strata) and KA-680 (A-03).
- Counterexamples: 16 numbered (CX-200 to CX-215, all in N20-CX) plus 4 late shortcuts without numbers (pending slots L-a to L-d in N20-CX). Cross-references carried: CX-049, CX-052, CX-119, CX-121, CX-305.
- Refusal codes: 7 (FROB-REF-440, -441, -442, -443, -445, -446, -447, all in N20-REF) plus 3 refusals stated without a code (reserved slots). Codes of other chapters raised or surfaced here: FROB-REF-109, -126, -214, -215.
- Other identifiers: INV-SENS-01 to -09 (9) and PO-SENS-01 to -09 (9) in N20b-19; OPEN-SENS-1 to -5 (5 decision beads); ADR-SENS-01, -05, -06, -07, -08, -09, -10, -11 stated with reasons where they govern; KILL-VISIBILITY (1 decision bead, 1 pilot); verbs `sense.profile`, `sense.causal-test`, `sense.symmetry`, `sense.calibrate`, `sense.visibility`, `analyze.symmetry`; constructions C-09 and C-19; theorem cards T-23 and T-30; metamorphic relations MR-18 and MR-25; analyses A-03, A-14, A-21.

## (d) Needs from other slices

Prerequisites owned elsewhere for which no seed key could be named (also in each bead's `needs`):
- Marrow (U13): the symmetry computation `frob-algebra::symmetry` (N13-15, N13-16) under BD-102, BD-241, N20b-01, N20b-02; the turn-count rule's alias witness (N13-34 onward) beside N20b-40.
- U20a (same chapter): the other proprioceptive sensors of §20.5 (contact switches, force-torque port reading, wheel odometry, readouts, tactile) beside BD-099, N20b-28, N20b-30, N20b-41; decision beads for KILL-ALIAS, KILL-SCHED, KILL-EXPOSURE; the property tests for INV-SENS-02 to -04 and PO-SENS-01 to -04, -07, -09 that N20b-19 indexes; the Gramian placement pilot, which reads N20b-35's classification; the registry bead N20-KA.
- Contact chapter: an optional pulse-model field (shape, duration, provenance) on interface cards (N20b-29); effective mass and stiffness at a contact.
- Requirement language and tasks: the `undetermined` verdict with a reason on peak-load atoms (N20b-29); declared transformation kinds of task terms and scene-family invariance (N20b-01).
- Learning chapter: the evaluation protocol's first-evaluation record, where the causal receipt is enforced (N20b-12); the four additional planted leaks of the actor's information contract, to be registered in N20b-10's leak registry and measured by that chapter's gate; consumer entry points routed through the Γ_obs guard (N20b-02).
- Nemesis: the fault-injection interface for the extrinsic bump (N20b-06).
- Ground: dataset admission, the delay-offset alias fixture and card updates from logs (N20b-08, N20b-27).
- Surface and registries: codes for three refusals (below); numbered KA and CX rows for the late subsections; regeneration of the printed gate register with the late gates; MCP task-cluster assignment for eight `sense` verbs.
- Dependency directions the lead should check: BD-312, BD-222 and BD-137 (the per-milestone verb beads) depend on verb implementations (BD-312 lists BD-242), so no bead here depends on them; BD-223 depends on BD-215; BD-110 and BD-246 depend on BD-102.

## (e) Contradictions and things that could not be turned into work as written

1. **Third-gain amendment of the docking fixture.** The chapter says "every cell admitted under the two-gain mission is wrong for the three-gain mission" and GATE-SENS-13 says the amendment "reverses every admitted cell". Exact enumeration of the 561-cell grid gives 327 of 344 reversed; the 17 cells with ε = 0 stay admitted, because an exact reading with a non-zero probe identifies the gain and the limits for g = 33/32 hold on the same probe interval. N20b-14 asserts the enumerated counts (327 reversed, 17 unchanged) and N20b-21 carries the note; the gate text should read "every admitted cell with ε > 0".
2. **The 561-cell grid is not a single-precision control.** All its quantities are dyadic with at most seven fractional bits, so `f32` is exact on it. The control "unguarded single precision misclassifies near the boundary" can only be exercised on the dense boundary corpus of 65,536 rows; N20b-14 says so and uses that corpus.
3. **GATE-SENS-08 at M1 "(docking rows)".** The register prints this for the observability gate, which has no docking rows; the milestone chapter says the mission slice needs "the docking rows of GATE-SENS-08 and -09". N20b-21 records the M1 scope as the linearized form on the CPU and flags the wording.
4. **Verb milestones.** The chapter's verb table and the verb registry give M2 or M3 as first milestone for `sense.profile`, `sense.schedule`, `sense.causal-test`, `sense.observability` and `sense.alias`, while the seed beads deliver them on the CPU at M1. N20b-17 writes registry rows with first milestone M1 and a coverage note.
5. **Three refusals without codes.** A peak-load claim without a resolved or modeled force history (§20.23); a stage-7 target whose lead is shorter than the latency chain (§20.24); evaluation of a policy without a passing causal-policy receipt (§20.15, a rule with no table row). N20-REF registers reserved slots and the three beads implement the behaviour fail-closed.
6. **`sense.visibility` is absent from the chapter's verb table** (it is in the verb registry appendix only), so the cluster has eight verbs, one more than an MCP task cluster allows; N20b-17 proposes a split and leaves the assignment to the surface owner.
7. **GATE-SENS-11's rig.** "A four-sensor rig with all six pairwise measurements" has no edge to the base that the chapter anchors. BD-242's fixture declares one sensor coincident with the base as the anchor; a rig without any declared anchor is refused (FROB-REF-446).
8. **Edges of the pose graph in simulation.** The chapter specifies the solver and takes pairwise measurements as given. BD-242 defines a `synthetic` front end (true relative pose plus addressed noise) and labels results accordingly; real front ends arrive with imported logs.
9. **"Exactly up to the signed permutation" in the equivariance test.** Integer fields and masks are compared exactly; float payloads are required bit-exact on the referee and the strict CPU profile, and BD-241 records per-channel differences on throughput tiers as findings, since a reduction ordered by index is not carried along by a mirror.
10. **`retime` in the typed-edit table is listed as symmetry-preserving**, but retiming one sensor of a mirrored pair breaks the clock clause of Γ_obs. N20b-01's predicate is authoritative.
11. **GATE-SENS-17's correlation** names no coefficient; N20b-37 fixes Spearman rank correlation before data.
12. **A derived row not in the chapter:** a packet arriving after the second command and before the third is also excluded (v₃ = 0 forces the same third command in both worlds). N20b-13 tests it and labels it derived.
13. **The late subsections' answers and shortcuts have gates and no KA or CX numbers**; N20b-20 and N20-CX carry them as pending slots.
14. **Parent packages.** BD-102 is placed under WP-11 (static analyses, `frob-analyze`) while the symmetry computation it calls sits under WP-38; M1 Sensorium beads are under WP-40 because it is the package that owns `frob-sense` at M1.

## (f) Integration pass (group G20)

- New bead N20b-45: allocation of registry numbers for the three uncoded refusals, the four late counterexamples and the late known answers of (e) 5 and (e) 13.
- Superseded as duplicates of U20a beads: N20b-23 (by N20a-12), N20b-26 (by N20a-32), N20b-35 (by N20a-35). Their content was folded into the kept beads, and references in N20b-18, -24, -25, -27, -32, -34, -36, -37, -38 were repointed.
- Overlaps with U20a re-scoped: see section (f) of the U20a coverage file. BD-099's encoder card field names now follow N20a-13, its mask and stream names follow N20a-02 and N20a-04; BD-187's mask reason `miss` is `no-return` and it quantizes after converting to the output convention.
- Graph fixes: BD-187 no longer depends on the higher-numbered seed BD-189 (the every-tier clock arrives through N20b-44).
