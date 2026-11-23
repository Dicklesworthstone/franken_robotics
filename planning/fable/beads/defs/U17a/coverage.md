# Coverage: drafting unit U17a (§17 Phylum, subsections §17.1 to §17.11 and the chapter preamble)

33 bead files: 3 seed beads (BD-285, BD-287, BD-356), 29 new beads (N17a-01 to N17a-29) and the chapter's
known-answer registry (N17-KA). The counterexample registry N17-CX and the refusal registry N17-REF are written by
unit U17b.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads of this unit | Seed beads of other units that carry part of it |
|---|---|---|---|
| §17 preamble | Crate, layer, what Phylum calls | N17a-01 | — |
| §17.1 | What Phylum owns | N17a-01 (ownership boundary, the narrowed claim, the three differentiators that can be lost) | — |
| §17.2 | Decisions | N17a-01 (all six ADRs with reasons, rejected alternatives and overturning gates); ADR-PHYLUM-01, -03 also N17a-03; -02 also N17a-02, N17a-05; -04 also N17a-23, N17a-19, BD-285; -06 also BD-356, N17a-12, N17a-26 | ADR-PHYLUM-05's experiment: BD-186 (U17b), BD-185 (U14b) |
| §17.3 | Three levels of variation | N17a-01 (table), N17a-02 (level decision, `single_structure`) | P-A: BD-169 (U16a); P-B: BD-170 (U17b); P-C: BD-356 (this unit) |
| §17.4 | Shape classes and model tables | N17a-02 (shape class, requirement, fitting, FROB-REF-160), N17a-03 (cohorts, model slots, WorldSlots, runs, padding contract, results keyed by address), N17a-10 (the batch-independence test and GATE-PHYLUM-02) | BD-148 (U29a: packed lowering, ShapeClassId), BD-170 (U17b: model table, masks, K-variant-gather), BD-183 (U21b: variants on devices) |
| §17.5 | Size buckets, and when bucketing recovers the padding overhead | N17a-04 (Ω_p, bounds, ratio caps, pay-off inequality, worked card, mixing ratio), N17a-05 (the dynamic program, cap of eight, default set, two-dimensional case, FROB-REF-741), N17a-06 (measuring c_L, stage shares, run length, contact padding), N17a-08 (end-to-end script of `phylum.plan`) | BD-171 (U21b: size buckets and `phylum.plan`), BD-144 (U14b: the M1 padding pilot) |
| §17.6 | Design-family lanes and the speedup bound | BD-356 (lanes, shared constants, static layouts, f, r, h), N17a-11 (pilot), N17a-12 (segmentation and regrouping), N17a-13 (measurement and end-to-end script), N17a-14 (KILL-FAMILYLANE decision) | BD-354 (U25a: ceiling-before-speed check) |
| §17.7 | Memory: which population shapes fit which machine | N17a-07 (byte model, the eleven-row table, verdicts, the three readings, FROB-REF-740), N17a-08 (asserted through `phylum.plan`) | BD-171 (U21b), BD-219 (U36: `sim.estimate`) |
| §17.8 | Admission, and edit-to-first-step latency as a target | N17a-09 (slot table, reset-time admission, the three paths and their targets, budgeted mode, fallback, latency ledger), N17a-10 (digests under 100 co-tenancies) | BD-184 (U17b: the latency gate over 1,000 edits), BD-183 (U21b: device-side admission), BD-295 (U32a: the dispatcher, KILL-SLOT), BD-040 (U36: incumbent edit-to-first-step) |
| §17.9 | Specialization of survivors, a gated optimization [F] | N17a-15 (gate to fund and three-arm pilot), N17a-16 (promotion rule, compile cap, boundary swap, INV-PHYLUM-05) | BD-371 (U21a: the specialized kernels, GATE-SPARK-14, KILL-SPECIALIZE decided) |
| §17.10 | The morphology generator [F] | BD-287 (grammar, GeneratorId, addressed sampling, exact probabilities, mutation operators, verb), N17a-17 (pilot), N17a-18 (validity, rejection report, shape awareness, stratification), N17a-19 (DesignSplit), N17a-20 (support, `--locate`, FROB-REF-541), N17a-21 (end-to-end script and GATE-PHYLUM-07) | BD-320 (U23b: family policies trained on a generator and split) |
| §17.11 | Campaign quotienting | BD-285 (pipeline, verb, sweep and cascade hooks), N17a-22 (similarity canonical form), N17a-23 (orbit representatives, PO-PHYLUM-01, FROB-REF-161, wreath option), N17a-24 (result transport across an orbit), N17a-25 (digest pilot), N17a-26 (mission-observable digest, exact lowering, memo key, split report), N17a-27 (end-to-end script), N17a-28 (replay audit, savings, unquotiented arm), N17a-29 (KILL-QUOTIENT decision) | The invalidation contract paragraph ("what survives", "what does not", positive and negative controls, calibration by shadow recompute, the stale-promotion counterexample) is seed BD-286 (U17b, `phylum.reuse`, GATE-PHYLUM-05) |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Beads of this unit | Expected from another unit |
|---|---|---|
| GATE-PHYLUM-01 (M3) | BD-285 (carries the seed's done-condition), N17a-27 (orbit and ballast rows, lidar and equal-Σx controls), N17a-28 (zero false quotients in 10⁴ replayed pairs) | — |
| GATE-PHYLUM-02 (M2) | N17a-10 (the full measurement: 512 designs, three capacities, 100 co-tenancies, per back end) | BD-170 (U17b, `batch-cpu` rows), BD-183 (U21b, device rows), BD-181 (U35, the shared fixture) |
| GATE-PHYLUM-03 (M2) | none (N17a-05 supplies the bucket count) | BD-186 (U17b) |
| GATE-PHYLUM-04 (M2) | none in `gates`; N17a-09 supplies the admission mechanism, the path classification and the latency ledger | BD-184 (U17b), whose done-condition is this gate |
| GATE-PHYLUM-05 (M3) | none in `gates`; N17a-24 and N17a-26 supply transport and the digest the contract uses | BD-286 (U17b), whose done-condition is this gate |
| GATE-PHYLUM-06 (M4) | none | BD-353 (U17b) |
| GATE-PHYLUM-07 (M3) | BD-287, N17a-18 (rejection rates), N17a-19 (no orbit straddles a split), N17a-21 (end-to-end measurement on two platforms) | — |
| GATE-PHYLUM-08 (M4) | none | BD-342 (U17b) |
| GATE-PHYLUM-09 (M2, §17.21) | none | U17b |
| KILL-FAMILYLANE (M4) | N17a-14 (decides), N17a-11 (pilot falsifier), N17a-13 (measures both triggers), BD-356 | — |
| KILL-QUOTIENT (M3) | N17a-29 (decides), N17a-25 (pilot falsifier), N17a-28 (measures both triggers and the bet's falsifiers), BD-285 | — |
| KILL-SPECIALIZE (M4; the device chapter governs) | N17a-15 (measures the specializable share s and the three-arm pilot) | The decision of record: BD-371 (U21a) and the device chapter's decision bead |
| KILL-PHYLUM (M2) | none (N17a-01 records the pre-commitment of ADR-PHYLUM-05; N17a-05 supplies the bucket count) | BD-186 (U17b) |
| KILL-FDGRAD (M4) | none | BD-353 (U17b) |
| KILL-BUCKET (device-side statement of KILL-PHYLUM) | inputs from N17a-05 | BD-171, BD-186 |
| KILL-SLOT (daemon chapter) | N17a-09 exposes the two measurements and honours the switch | BD-295 (U32a) |

## (c) Counts

- Known answers: 22 rows in the registry N17-KA (KA-365 to KA-386), each with case, truth, tolerance, tier,
  negative control and carrying bead. 16 of them are also restated with their values in the test plans of this
  unit's mechanism beads (KA-365 to KA-378, KA-380 and KA-385); the other six (KA-379, KA-381 to KA-384, KA-386)
  belong to U17b's seed beads and are in the registry only. Nine rows that had no stated negative
  control received a DESIGN control in N17-KA. Four known answers of the similarity construction (KA-119 to KA-122)
  are restated in N17a-22.
- Counterexamples: 13 of the chapter's 14 (CX-153 to CX-164 and CX-166) are stated with their cases in this unit's
  beads; CX-165 (finite-difference gradients) belongs to U17b. Three counterexamples of other chapters are used
  (CX-146, CX-056, CX-057). The registry bead N17-CX is U17b's.
- Refusal codes of the chapter: 7. Specified in full (meaning, trigger, envelope fields, ranked repairs, exit code)
  in a mechanism bead of this unit: 5 (160 in N17a-02; 161 in N17a-23 and BD-285; 541 in N17a-20; 740 in N17a-07;
  741 in N17a-05). Used with meaning and repairs: 162 (N17a-26; owned by BD-286). Listed only: 540 (U17b). All
  seven are tabulated for the crate contract in N17a-01. The registry bead N17-REF is U17b's.
- Invariants and obligations carried by this unit: INV-PHYLUM-01, -02, -03, -05, -06; PO-PHYLUM-01, -02 (INV-PHYLUM-04
  and -07 and PO-PHYLUM-03 are tested in U17b's seeds and stated in N17a-01's table).
- Pilots: 4 (N17a-11 family lanes, N17a-15 specialization, N17a-17 generator, N17a-25 mission digest).
- End-to-end script beads: N17a-08 (`phylum.plan`), N17a-10 (cohort invariance), N17a-13 (family lanes), N17a-21
  (`phylum.generate`), N17a-27 (`phylum.quotient`), N17a-28 (quotient audit); N17a-06, N17a-16 and the two
  decision beads carry their own scripts.

## (d) Needs from other slices

| Need | Where it is used | Probable owner |
|---|---|---|
| Registry rows of FROB-REF-160, -161, -162, -541, -740, -741 with their exit codes | N17a-02, -05, -07, -08, -20, -23, -26, BD-285 | N17-REF (U17b) |
| Registry entries of the verbs `phylum.plan`, `phylum.generate`, `phylum.quotient` in `registries/verbs.toml` (cluster `families`) | N17a-08, BD-287, BD-285, N17a-21, N17a-27 | BD-171, BD-222, BD-312 |
| `phylum.sweep` with option slots for quotiented dispatch, family lanes and survivor specialization | BD-285, N17a-10, N17a-13, N17a-16, N17a-24, N17a-27, N17a-28 | BD-184 (U17b) |
| The frozen corpus of structural edits (mutation corpus) and the mixed workload of 512 topologies | N17a-10, N17a-13 | BD-186 (U17b) / the design corpus BD-248 |
| The campaign corpus on which dispatch savings are measured (its definition and identity) | N17a-28, N17a-29 | Arena / Atelier (§24.9, BD-248) |
| Decision beads for OPEN-PHYLUM-1 (bucket set and waste limit), OPEN-PHYLUM-2 (run length), OPEN-PHYLUM-5 (contact-capacity axis); this unit's beads state the fail-closed defaults and supply the data | N17a-03, -05, -06, -17 | U17b (§17.20) |
| The slot-dispatch pilot (queue trace replayed on `batch-cpu`) and KILL-SLOT | N17a-09 | §21.11 / §32 (BD-295) |
| The per-workload capacity table (C, R, samples) frozen at M2 | N17a-02, N17a-07 | OPEN-SPARK-3, OPEN-KIN-3 (Kinesis, Spark) |
| One byte model shared with `sim.estimate` | N17a-07 | BD-219 (U36) |
| Kernel stage signatures listing the table columns each kernel reads; the task file's declared read set | N17a-26 | BD-014 (U21b), BD-122 (U24c) |
| A scene-generator operation mapping a world address through an isometry | N17a-24 | Habitat (§19) |
| The refusal code Cortex raises for a score requested on a training-manifest design; the family-policy scoring path that calls the support test | N17a-19, N17a-20 | BD-320, BD-321 (§23) |
| Row-class solve queues admitted on a back end (GATE-KIN-08), for the alone-and-together comparison with lanes | BD-356, N17a-13 | Kinesis (§16.19) |
| KILL-SPECIALIZE decision bead; the fourth pilot arm for limb-archetype kernels in the same harness | N17a-15, N17a-16 | U21 (§21.18), U17b (§17.21) |
| Airlock admission of `fs-regime` (dimensionless groups, scaling maps); the runner of the similarity metamorphic relation | N17a-22 | §33, §38 |
| BD-171 should depend on N17a-04, N17a-05 and N17a-07 (it wires them into `phylum.plan`); BD-144 consumes the generated set of N17a-17; BD-184 consumes the latency ledger of N17a-09; BD-286 consumes N17a-24 and N17a-26 | — | The lead, when resolving edges |

## (e) Contradictions, gaps and what was done about them

1. **Default bucket set.** The type sketch in the appendix of type sketches prints {12, 18, 24, 32, 48, 64}; §17.5,
   §21.11 and both open-question rows print {12, 18, 24, 36, 48, 64}. The beads use 36 and say so (N17a-02, N17-KA).
2. **"Padding waste" has no formula.** It is used in three thresholds (25% limit, GATE-PHYLUM-03, the generator
   pilot's falsifier). The beads define it as Ω₁ − 1 per bucket, the reading under which a 25% limit, a ratio of
   1.25 and eight buckets for 12 to 66 degrees of freedom are one statement, and print the alternative (1 − r)
   beside it (N17a-04, N17a-05, N17a-17).
3. **Exit code of FROB-REF-740 and -741.** The exit table names 70x (exit 4) and 72x/73x (exit 5) and is silent on
   74x. The beads propose exit 4 (admission class) and make the end-to-end script read the exit from the registry.
4. **KILL-SPECIALIZE has two owners.** §17.9 and §21.18 state the same pilot and nearly the same trigger; the gate
   register says the device chapter governs and adds the profile clause. This unit wrote the Phylum-side pilot
   (N17a-15) and promotion mechanism (N17a-16) and left the decision to BD-371. If the device unit also emits a
   three-arm pilot bead, the two should be merged. The bit-identity rule also differs (§21.18: must be
   bit-identical; §17.9: or a new kernel version owing the fidelity battery); N17a-16 swaps in only
   digest-equivalent kernels, which satisfies both.
5. **Seeds for this unit's subsections are owned elsewhere.** §17.4, §17.5, §17.7, §17.8 and §17.9 are carried
   by seeds of other units (BD-148, BD-170, BD-171, BD-183, BD-184, BD-371). To keep every mechanism of those
   subsections in a bead that states it fully, this unit wrote library and subtask beads (N17a-02 to N17a-10,
   N17a-15, N17a-16). Overlap with BD-171 (planner and `phylum.plan`) and BD-184 (admission) is possible and is
   for the lead to reconcile; the intended split is: this unit's beads hold the Phylum-side mathematics and host
   logic, the seeds hold device-side buckets, verb wiring and the latency gate.
6. **The 36-bucket capacities.** §17.7 prices the 36 bucket at C = 48, R = 256, but the reference humanoid (35
   degrees of freedom) needs C = 64, R = 320 and would not fit that row. The beads follow the stated default
   "capacities taken from the largest member" (N17a-02) and treat §17.7's rows as the reference byte figures.
7. **Two printed byte formulas** differ by 40 bytes per degree of freedom; N17a-07 uses the form of the memory
   arithmetic section, which the estimator uses.
8. **Family lanes: unstated details.** The plan names no command-line option, no lane count and no hysteresis, and
   does not say in §17.6 that lanes must reproduce the masked lane's digests. The beads label the option name, a
   lane count of 8 and a stability of two control intervals as DESIGN, and make digest equality a requirement
   derived from INV-KIN-04 and INV-PHYLUM-02 (BD-356, N17a-12).
9. **Generator exactness.** "Exact log-probability" with continuous parameters is realised by finite rational
   grids (DESIGN, BD-287). Probabilities are exact under the stated distribution; after rejection by
   `design check`, and under stratification, weights need an acceptance mass or class probability that is exact
   only on enumerable sub-grammars and `Estimated` otherwise (N17a-18). The 50% flag follows the plan's wording
   (rate above 50%) with a confirmed/provisional qualifier.
10. **DesignSplit rule.** The plan says "by seed and by structural stratum" with orbits on one side; N17a-19
    assigns sides by a keyed hash of the orbit key and keeps seed ranges as a second barrier (DESIGN). The
    counterexample on file (CX-166) has no numeric case; a mirror-pair fixture is registered as its smallest case.
11. **Determinism class of transported per-address results** is not stated; N17a-24 serves them as bit-identical
    only after a replay check for the group element and model class, and as tolerance-class otherwise (DESIGN).
12. **Radius of gyration** in the similarity form is not tied to an axis; N17a-22 uses the polar radius (DESIGN).
13. **Milestone tension, generator.** BD-144 (M1) measures "1,000 generated designs" two milestones before the
    generator (M3), and only three family classes exist at M1. N17a-17 supplies a prototype grammar at M1 and
    N17a-18 repeats the reports at M3 over all classes.
14. **Milestone tension, GATE-PHYLUM-02.** 512 designs are required at M2 when four archetypes exist; N17a-10 draws
    them from the archetypes and the frozen structural-edit corpus and lists that corpus under needs.
15. **BD-287 depends on BD-285** in the seed graph although the generator needs only orbit keys (N17a-23); the
    seed dependency is kept.
16. **GATE-PHYLUM-01's 10⁴ class pairs** may not exist in real corpora (the digest's factor is 1 in general);
    N17a-28 samples real classes first and fills from generated and fixture classes, reporting the shares.
17. **Wreath products** are marked Frontier in the construction but have no named pilot; N17a-23 treats them as
    an optional exact-counting path with two checked preconditions and their known answer, relying on BD-049.
18. **Parent work packages.** `frob-phylum` has only WP-44 (M2) and WP-63 (M3, M4). M3 and M4 beads, including
    family lanes, are parented to WP-63 by the crate-and-milestone rule although the lanes' seed dependencies sit
    in the mission-query package; the specialization beads are parented to WP-46, which names survivor
    specialization. Pilots that run before their mechanism's milestone keep the mechanism's work package.
19. **Bead size.** Bodies run 121 to 190 lines, above the typical 60 to 130, because each carries formulas,
    worked numbers, a refusal with repairs and a full test plan; none was compressed to fit.

## (f) Integration pass (group G17): what changed in this unit

- **New bead N17a-30**, the campaign corpus: the plan measures KILL-QUOTIENT's 5% clause and BET-31 "on the
  campaign corpus" and never defines it. N17a-30 defines a campaign design set, the composition of version 1, its
  identity and its freeze before any measurement; N17a-28 depends on it. Carries part of §17.11 and §17.19.
- **N17a-10 is the single GATE-PHYLUM-02 measurement.** The other unit's N17b-06 specified the same run and is
  superseded by it; its content is folded into N17a-10.
- **Seed beads of the other unit are capstones of this unit's components.** BD-170 (level P-B) depends on N17a-02
  and N17a-03, and N17a-03 no longer depends on BD-170. BD-184 (latency gate) depends on N17a-09. BD-286
  (invalidation contract) depends on N17a-24 and N17a-26.
- **Admission is split in two beads with one record.** N17a-09 is the slot table and reset planner; N17b-05 is the
  admission controller and owns the admission record, into which N17a-09 stamps two instants.
- **N17a-15 moved from M3 to M2**, because the limb-archetype arm (M2) is by definition a fourth arm of this pilot;
  N17b-24 depends on it and adds only its arm.
- **N17a-06 executes the runs whose criteria N17b-11 fixes**; N17b-11 depends on N17a-06 and N17a-17.
- N17a-23's dependency on theorem card T-24 (M4) is now `related`. N17a-26 depends on the specification bead of the
  projection quotient (N10b-12). The verb `phylum.sweep` is N17b-04, not BD-184: beads that need the verb depend
  on N17b-04. Registry entries of the cluster are N17b-12 (duplicated outside the group by VERBS-phylum).
- Cross-group duplicates and gaps reported to the lead are in `integration/out/G17.json`.
