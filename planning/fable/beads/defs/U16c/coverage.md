# Coverage of drafting unit U16c

Chapter §16 Kinesis, subsections §16.16 to §16.24, the seed bead BD-083, and the chapter registries N16-CX and N16-REF.

Files: 48 beads. One seed bead (BD-083), two registry beads (N16-CX, N16-REF), 45 new beads (N16c-01 to N16c-45).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §16.5 (owned by U16a; source of the seed bead) | The transactional step: retry ladder and refinement law | BD-083 (ladder, numerical-fault, GATE-KIN-04 rows), N16c-01 (dyadic refinement law, FROB-REF-223), N16c-02 (end-to-end script) |
| §16.16 | Known answers | N16c-03 (rows 1 to 4 and 10 with their mutants, M1), N16c-04 (rows 5 to 9 with their mutants, M2 and M3); values also cited in BD-083, N16c-01, N16c-08, N16c-10, N16c-11. The KA registry bead N16-KA is U16a's |
| §16.17 | Gates and kill gates | N16c-05 (all 21 gates and 10 kill gates as registry rows with thresholds, controls and carriers); measuring and deciding beads listed in (b) |
| §16.18 | Open questions | N16c-06 (OPEN-KIN-1), N16c-07 (OPEN-KIN-2), N16c-08 (OPEN-KIN-3), N16c-09 (OPEN-KIN-4), N16c-10 (OPEN-KIN-5), N16c-11 (OPEN-KIN-6) |
| §16.19 | Row-class solve queues [F] | N16c-12 (pilot), N16c-13 (kernels), N16c-14 (step-plan options and generations), N16c-15 (verb option, phylum.plan bound, receipt, label and refusal), N16c-16 (GATE-KIN-08 script, arm E), N16c-17 (KILL-ROWQUEUE) |
| §16.20 | Decoupled stepping of separated robots in one world [M] | N16c-18 (paper pilot), N16c-19 (tile planner and separation certificate), N16c-20 (stepping, merge, streams, tile ledger, `--decouple`), N16c-21 (GATE-KIN-09 script), N16c-22 (KILL-INTERACT) |
| §16.21 | Step adjoints through the converged contact solve [M] | N16c-23 (pilot), N16c-24 (derived reverse kernels with certificates), N16c-25 (reverse sweep and labels), N16c-26 (`phylum.gradient --method adjoint`, guards, estimate, consumers), N16c-27 (GATE-KIN-10 script), N16c-28 (KILL-STEPADJOINT) |
| §16.22 | Closed-form elimination of single-loop planar closures [F] | N16c-29 (pilot and census), N16c-30 (recognition, branch, toggle, `design.check`), N16c-31 (four-bar kernels), N16c-32 (reduced embedding in the step), N16c-33 (GATE-KIN-11 script), N16c-34 (KILL-LOOPELIM) |
| §16.23 | Ancestor-sparse assembly and the solve-stage ledger [F] | N16c-35 (ancestor tables and counts), N16c-36 (ledger pilot), N16c-37 (sparse solve path and dense fallback), N16c-38 (`--solve-path`, identity, mixing refusal), N16c-39 (GATE-KIN-12 script), N16c-40 (KILL-ANCESTOR) |
| §16.24 | A held command and the step that integrates it [S] | N16c-41 (fixture), N16c-42 (rule 1, A-22 twice), N16c-43 (rule 2, refinement studies hold clocks), N16c-44 (rule 3, deadline verdicts across decimations), N16c-45 (GATE-KIN-13 script and pilot re-run) |
| Appendix N.13 | Counterexamples of §16 | N16-CX |
| §16.14 / Appendix O | Refusal codes of §16 | N16-REF |

## (b) Gates, kill gates and the beads that measure or decide them

Gates whose mechanism is in this unit's subsections:

| Gate | Measured or decided by | Mechanism beads naming it |
|---|---|---|
| GATE-KIN-04 (rows: forced-retry digests, refinement statistics, redraw mutation; referee, M1) | BD-083, N16c-02 | N16c-01, N16c-03 |
| GATE-KIN-08 | N16c-16 | N16c-13, N16c-14, N16c-15 |
| GATE-KIN-09 | N16c-21 | N16c-19, N16c-20 |
| GATE-KIN-10 | N16c-27 | N16c-24, N16c-25, N16c-26 |
| GATE-KIN-11 | N16c-33 | N16c-30, N16c-31, N16c-32 |
| GATE-KIN-12 | N16c-39 | N16c-35, N16c-36, N16c-37, N16c-38 |
| GATE-KIN-13 | N16c-45 | N16c-41, N16c-42, N16c-43 |
| ✖ KILL-ROWQUEUE | N16c-17 (pilot input N16c-12) | |
| ✖ KILL-INTERACT | N16c-22 (pilot input N16c-18) | |
| ✖ KILL-STEPADJOINT | N16c-28 (pilot input N16c-23) | |
| ✖ KILL-LOOPELIM | N16c-34 (pilot input N16c-29) | |
| ✖ KILL-ANCESTOR | N16c-40 (pilot input N16c-36) | |

Gates of the chapter that this unit expects another unit to carry (each has a seed bead; none is left to nobody, so no extra measurement bead was written). N16c-05 records every threshold, and N16c-03 and N16c-04 supply registered mutants for the known-answer rows.

| Gate | Expected carrier (seed; unit) | This unit's supporting bead |
|---|---|---|
| GATE-KIN-01 | BD-088 (U16a) | N16c-03 (free-body row and its mutant) |
| GATE-KIN-02 | BD-089 (U16a) | |
| GATE-KIN-03 | BD-150, BD-159 (U16a) | |
| GATE-KIN-04 (impact time, tied sliders, uncommitted reads; `strict` rows at M2) | BD-081, BD-082 (U16a); BD-161 (U18a) | N16c-03 |
| GATE-KIN-05 | BD-181 (U35) | N16c-04 |
| GATE-KIN-06 | BD-169 (U16a), BD-183 (U21b) | N16c-04 |
| GATE-KIN-07 | BD-212 (U16b) | N16c-08 (capacity decision that the gate needs) |
| GATE-FID-01 | BD-212 (U16b) | N16c-06, N16c-11 (decisions made before its re-baseline) |
| GATE-FID-02 | BD-263 (U16b) | N16c-04 |
| GATE-FID-03 | BD-213 (U16a) | |
| GATE-FID-04 | BD-265 (U16b) | |
| GATE-FID-05 | BD-266 (U16b) | |
| GATE-FID-06 | BD-208 (U29a) | N16c-04, N16c-11 |
| GATE-FID-07 | BD-214, BD-268 (U16b) | |
| GATE-FID-08 | BD-264 (U16b) | N16c-04 |
| ✖ KILL-DR | BD-264 (U16b) | N16c-05 records both readings |
| ✖ KILL-RANK | BD-265 (U16b) for tier pairs; BD-328 (U23b) for controller rungs | N16c-05 records both readings |
| ✖ KILL-SHADOW | BD-268 (U16b) | |
| ✖ KILL-VARIATIONAL | BD-384 (U16a) | |
| ✖ KILL-TREEOP | BD-385 (U16a) | N16c-40 states its relation to KILL-ANCESTOR |

If U16a or U16b do not write separate kill-gate decision beads for the last five, the seeds above still carry "KILL-… decided" in their done-conditions, and N16c-05 holds the full trigger and fallback text of each.

## (c) Counts

- Known answers carried from Appendix M with their values: 13 (KA-338, 339, 340, 341, 343, 344, 347, 348, 349, 350, 351, 354, 355). Known answers stated in §16.19 to §16.24 that Appendix M does not number yet, carried with values in the mechanism and gate beads: 21 (three for solve queues; the two hand cases and three planted controls of decoupled stepping; the scalar and block fixtures of step adjoints; four four-bar answers; the quadruped, humanoid and chain counts; the four rows of the held-command table). The N16-KA owner should append rows for them.
- Counterexamples: 9 numbered (CX-144 to CX-152) and 15 appended from the late subsections under provisional labels K1 to K15, all in N16-CX.
- Refusal codes: the 9 codes of the chapter (220, 221, 222, 223, 224, 721, 722, 820, 821) in N16-REF, with 7 refusals that the late subsections name without a code, and cross-references to 212, 263, 540, 561, 602 and 865.
- Invariants and proof obligations carried in test plans: INV-KIN-02, -03, -07 (BD-083); INV-KIN-03 and PO-KIN-04 (N16c-01); INV-KIN-04 (N16c-13, N16c-14, N16c-04); INV-KIN-05 (N16c-08, N16-REF); INV-KIN-06 (N16c-11, N16c-04); PO-KIN-02, PO-KIN-03 (N16c-04, N16c-37); PO-KIN-05 (N16c-04). INV-KIN-01, INV-KIN-08 and PO-KIN-01 belong to mechanisms of U16a and U16b.
- Decisions: ADR-KIN-05 (BD-083, N16c-43), ADR-KIN-03 (N16c-13), ADR-KIN-07 (N16c-36, N16c-40). Open questions OPEN-KIN-1 to -6: one decision bead each. Risks RISK-31 (BD-083) and RISK-47 (N16c-23, N16c-25). Bet BET-34 (N16c-05). NONGOAL-15 (N16c-23, N16c-28). Hypotheses H-04b (N16c-36, N16c-40) and H-27 (N16c-23). Constructions: C-14 (N16c-24, N16c-31), C-16 (N16c-25).
- Pilots (one bead each, every Frontier or Moonshot mechanism depends on its pilot): N16c-12, N16c-18, N16c-23, N16c-29, N16c-36.
- End-to-end script beads: N16c-02, N16c-16, N16c-21, N16c-27, N16c-33, N16c-39, N16c-45, plus the batteries N16c-03 and N16c-04.

## (d) Needs from other slices

Prerequisites for which no key could be named, or whose seed lies at a later milestone. Each also appears in the `needs` field of the bead that has it.

| Bead | Need |
|---|---|
| BD-083 | §18.5: the fixed-point cone iteration on s = μ·‖u_T‖ as an island solver available to the referee transaction at M1 (attempt 3 of the ladder) |
| N16c-01 | §35.9: addressed counter-based streams and normal variates on the CPU at M1 with a stream registry (the seed for addressed draws, BD-180, is the M2 device kernel) |
| N16c-02 | §38.2: M1 wiring of the conformance test targets and the receipt format |
| N16c-03, N16c-04 | §16.16: the registry bead N16-KA (U16a) for KA numbers |
| N16c-05 | §40.1: the schema of a gate row in `registries/gates.toml` |
| N16-REF | §32.4: the exit-code table, the refusal block and the one-code-one-exit test; registry-owner assignment of codes for seven uncoded refusals |
| N16c-06 | §36.3: OPEN-PERF-4 restatement of performance rows if the default step moves; §38: the frozen standard battery (part of BD-212, kept out of `deps` because the frontier must precede that bead's GATE-FID-01 re-baseline) |
| N16c-07 | §19.3: articulated interior fixtures as the idle-body test case |
| N16c-08 | §36.3: OPEN-SPARK-3 (capacities of Q and H); §38: the frozen standard battery (part of BD-212, which also carries the gate this decision feeds) |
| N16c-09 | §18.17: the record of GATE-TRAC-09 and KILL-HARDCONTACT |
| N16c-11 | §29.9: the `energy-created` monitor's window and denominator; §38: the frozen standard battery and the GATE-FID-01 re-baseline (BD-212, kept out of `deps` for the same ordering reason) |
| N16c-12 | §28: the batch sizes design campaigns use |
| N16c-13 | §21.8/§35.5: INV-SPARK-04 as an enforced manifest property; anchored reductions |
| N16c-14 | §21.11: the static step plan and its header |
| N16c-15 | §16.8: the promotion rule's admission check (BD-263, M3) refusing `experimental-execution`; §36.7: disclosure-block fields |
| N16c-16 | §17.14: the head-to-head harness accepting arm E |
| N16c-18 | §19.3: route-graph actors in the warehouse archetype (BD-421 is an M6 epic) |
| N16c-19 | §18.4 swept bounds and bounding radii; §15.3 acceleration and speed caps from cards; §20.4 message channels with minimum latency and sensor instants with range and field of view; §25.17 a multi-robot scene description |
| N16c-20 | §35.9 streams keyed by island as perturbed entity; §29.12 a tile-ledger table in `telemetry.summary`; §25.17 admission of multi-robot scenes and any multi-robot shape class |
| N16c-21 | §25.17: the two-carrier handoff fixture (BD-435 is an M8 epic); §19.7: a 16-robot sparse warehouse scene |
| N16c-24 | §21.7: reverse-mode transformation over the kernel IR; §12: instance certificates through the derivation subprocess |
| N16c-25 | §18.5 and §18.7: per-row region and normalized margin for contact and joint rows |
| N16c-26 | §13.4/§20.13: the observation-preserving subgroup; §14.5: chart and retraction of the inertia cone |
| N16c-29 | §14.10: the list of closures in the archetype library |
| N16c-30 | §14.9: compile memoization for the eliminability verdict |
| N16c-31 | §35.6: deterministic two-argument arctangent and trigonometric functions in the dialect |
| N16c-32 | §15.9: the linkage transmission reading its modulus from the eliminated-loop record |
| N16c-33 | §14.10: the remaining archetypes (BD-246, M3) for the matched share |
| N16c-35 | §17.4: the model table as the home of ancestor lists |
| N16c-36 | §38: the standard battery's scenario list for contact counts (frozen by BD-212 late in M2; the pilot runs earlier and is re-read) |
| N16c-37 | §18.12/§35.10: diagonal scaling shared by both solve paths |
| N16c-38 | §35.3: the pairing admission rule gaining a solve-path check; §16.8: promotion across solve paths |
| N16c-41 | §24.5: batched deadline verdicts (BD-255, M3) |
| N16c-42 | §16.7: each tier's one-step update as a published linear map; §24.4: a task's decimation readable by analyses |
| N16c-43 | §16.2: the physical and numerical parameter sets with a digest; §16.9: the model-form check's step arm built through the study type (BD-266, M3) |
| N16c-44 | §24.10: fidelity-dependent reporting with both runs kept |
| N16c-45 | §16.24: the reproduction bundle of the recorded held-command pilot |

## (e) Contradictions and things that could not be turned into work as written

1. **The ledger instrument (§16.23) and the six-check rule (§34.6, §38.1).** §16.23 says "`xtask` instruments the `batch-cpu` solve"; `xtask` is limited to six checks, runs no physics and records no gate. N16c-36 makes the ledger a `frob-conformance` test target with counters behind a cargo feature of `frob-dyn`.
2. **"Agree inside determinism class D2 and not bit for bit" (§16.23).** D2 is bitwise. N16c-37 and N16c-38 read it as: each solve path is D2 alone; across paths agreement is at class D3 with the decision-level test, which is also what GATE-KIN-12 asks for.
3. **Bytes per quadruped world.** §16.21 computes the adjoint store with 27 kB per world (capacities C = 24, R = 128, also printed in KA-348), while the table of §16.6 and §36.3 give C = 32, R = 160 and 31 kB. N16c-25 and N16c-26 compute the store from the shape class and state both figures (4.32 MB and 8.85 GB; 5.00 MB and 10.25 GB). KA-348 also prints 37 kB for the tracked workload where the formula gives 36,464 B (36 kB, as the chapter's table says); N16c-08 notes it.
4. **CX-146** prints "canonical zero +0"; §16.6 and §35.5 require −0.0. N16-CX registers −0.0.
5. **CX-151 and RISK-31** say 80% for the coverage below which the adjudicator is named; §16.11, GATE-FID-03 and §40 say 90%. N16-CX and N16c-05 use 90% (the register governs).
6. **OPEN-KIN-6** states its default as "1% of actuator work per episode"; §16.10 and §29.9 define the only flag rule as 1% of the absolute work exchanged in any 0.5 s window (10⁻⁶ relative on the referee). N16c-11 treats the windowed rule as governing and retires the per-episode wording through its ADR.
7. **KILL-DR and KILL-RANK** have different triggers, fallbacks and (for KILL-RANK) milestones in §16.17 and in §40. N16c-05 records both and marks §40 as governing; the deciding seeds are U16b's.
8. **KILL-ROWQUEUE and KILL-INTERACT** are defined in §16.19 and §16.20 and listed in §40.8, but are missing from the brief's gate list. They are carried by N16c-17 and N16c-22.
9. **Refusals without codes.** §16.19, §16.21, §16.22, §16.23 and §16.24 name refusals in words only. N16-REF lists seven with their block, an interim existing code where one fits (820, 602, 540, 561, 721/722) and a request for assignment; two have no fitting code and their mechanism beads may not close until one is assigned.
10. **Appendix M and N have no rows for §16.19 to §16.24.** Their counterexamples are appended in N16-CX under provisional labels; their known answers are carried with values in the beads and listed in (c) for the N16-KA owner.
11. **The retry ladder (§16.5)** does not say whether the changes of successive attempts accumulate, nor whether FROB-REF-220 and -224 consume attempts. BD-083 fixes: changes accumulate; 220 and 224 are terminal for the transaction. Both are recorded as DESIGN choices in the bead.
12. **Launch overhead of solve queues (§16.19).** The hypothesis counts 60 added launches for three classes; the mechanism also launches the general masked lane and the assignment kernel. N16c-12 and N16c-14 state the full count and use the measured one.
13. **The pilot of §16.22 has no stated falsifier.** N16c-29 defines three, matching GATE-KIN-11's parity tolerance and KILL-LOOPELIM's two triggers.
14. **Labels of step adjoints (§16.21).** The plan fixes the outcome on the block fixture ("a fixed-mode gradient in stick is reported as uninformative") without a general rule. N16c-25 defines one (the stick share of the adjoint's energy norm above one half), labelled DESIGN and open to tuning by the gate suite.
15. **"The crossed-branch mutant … refused" (GATE-KIN-11)** names no mechanism that refuses it. N16c-32 adds a reference-pose consistency check at environment creation, which is what refuses a kernel that returns the other root.
16. **GATE-KIN-11's parity clause "on referee"** depends on bilateral closure rows in the transactional step; exact closures in FrankenSim wait on an upstream item. N16c-33 runs the clause against the transaction's own bilateral rows and, if the referee wrapper lacks them at M2, against `strict` in `f64`, naming the adjudicator.
17. **BD-083's "best-matching subsection §16.24"** in the brief is a mapping artefact; its content is §16.5 (U16a's subsection). The bead was written in full from §16.5, split into BD-083 (ladder and gate rows) and N16c-01 (refinement law); BD-083 gained the dependencies N16c-01, BD-115, BD-094 and BD-098 in addition to the seed's BD-082.
18. **Milestones of the held-command work.** GATE-KIN-13 is M2 but rule 3 needs `eval.compare` on devices and the promotion rule (M3) and is not tested by the gate; N16c-44 is therefore M3. Rule 1 extends an M1 analysis in work package WP-11; N16c-42 is M2 with that parent.
19. **Proposed ADR identifiers.** The decision and kill-gate beads name ADR-KIN-08 to ADR-KIN-18 as proposed identifiers for the records they produce; the plan defines only ADR-KIN-01 to -07.
20. **Verb options and names introduced as DESIGN** where the plan names only the capability: `--closures rows|eliminate`, `--numerical-plan-floor`, `--chaos-plan`, `--substeps`, `--integration exact-hold`, `study plan --kind step-refinement`, `analyze sampled-loop`, `spark check --pair solve-path:…`, `eval compare --across decimation`. The verb-registry owner may rename them; behaviour is what the beads fix.
