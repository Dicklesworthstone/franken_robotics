# Coverage of drafting unit U17b (§17.12 to §17.21 of chapter 17, Phylum)

37 bead files: 6 seed beads (BD-170, BD-184, BD-186, BD-286, BD-342, BD-353), 29 new beads (N17b-01 to N17b-29) and
2 registries (N17-CX, N17-REF). The known-answer registry N17-KA is written by unit U17a.

## (a) Subsections and the beads that carry them

| Subsection | Title | Bead keys |
|---|---|---|
| §17.12 | Quality-diversity maps per dispatch [F] | BD-342 (campaign loop, two-agent merge, the serial-rounds gate); N17b-16 (pilot); N17b-17 (map type, total order, merge laws, bias and strata); N17b-29 (per-dispatch update and envelope delta); N17b-18 (`phylum.map`, `opt.archive`, storage) |
| §17.13 | Paired finite-difference design gradients [F] | BD-353 (verb, gate, end-point confirmation); N17b-19 (pilot); N17b-20 (KILL-FDGRAD, first reading); N17b-21 (stencil, retractions, shared draws); N17b-22 (estimator, divergence, truncation, limits, FROB-REF-540) |
| §17.14 | The head-to-head experiment | BD-186 (the experiment, measures, statistics, pre-declared outcomes); N17b-03 (512-topology workload and mutation corpus); N17b-09 (arms C and D); N17b-08 (freeze of incumbents); N17b-07 (compute envelope); N17b-01 (the pilot on existing engines); N17b-10 (KILL-PHYLUM decision); N17b-06 (measure 4, digest invariance). Arm B is seed BD-185 and the M1 paper pilot is seed BD-144 (both unit U14b) |
| §17.15 | Verbs | N17b-12 (the cluster of seven, options, registry entries); N17b-04 (`phylum.sweep`); N17b-14 (`phylum.reuse`); BD-353 (`phylum.gradient`); N17b-18 (`phylum.map`). `phylum.plan` is seed BD-171 (U21b); `phylum.generate` is BD-287 and `phylum.quotient` is BD-285 (U17a) |
| §17.16 | Refusals | N17-REF (all seven codes with triggers, repairs, exits); raising sites in BD-170 and N17b-04 (160, 740, 741), N17b-13, BD-286, N17b-14 (162), N17b-22 and BD-353 (540); 161 and 541 are raised by BD-285, BD-287 and two sites listed under (d) |
| §17.17 | Invariants and proof obligations | N17b-15 (index of all ten; tests of INV-PHYLUM-03, -06, PO-PHYLUM-01, -02); INV-PHYLUM-01: N17b-04, N17b-06; INV-PHYLUM-02: BD-170, N17b-02, N17b-06; INV-PHYLUM-04: BD-286, N17b-13; INV-PHYLUM-05: N17b-26, N17b-27; INV-PHYLUM-07: N17b-21; PO-PHYLUM-03: N17b-17 |
| §17.18 | Known answers | Hexapod and mirror rows: N17b-15, N17-CX (and BD-285); ballast: N17b-15, N17-CX; padding table: N17b-11, N17b-03, N17b-15; mixed against homogeneous: BD-170, N17b-06; mirror swap under Γ_obs and coupled bus: BD-286, N17b-14; ballistic gradient and grazing: N17b-22, BD-353; generator: N17b-15 (and BD-287). The registry bead N17-KA is U17a's |
| §17.19 | Gates and kill gates | See table (b) |
| §17.20 | Open questions | N17b-11 (OPEN-PHYLUM-1, -2, -5); N17b-07 (OPEN-PHYLUM-3); N17b-08 (OPEN-PHYLUM-4) |
| §17.21 | Limb-archetype kernels (late subsection, no seed) | N17b-23 (paper pilot: coverage); N17b-24 (pilot: fourth kernel arm); N17b-28 (KILL-LIMBKERNEL, first reading); N17b-25 (registry and decomposition); N17b-26 (kernels, bit identity); N17b-27 (GATE-PHYLUM-09, hand controls, final reading) |

Seed beads of this unit that draw on subsections owned by U17a, written completely here: BD-170 with N17b-02
(§17.3, §17.4: model variants, model tables, masks, `K-variant-gather`); BD-184 with N17b-05 (§17.8: admission paths
and edit-to-first-step latency); BD-286 with N17b-13 and N17b-14 (§17.11: the invalidation contract).

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone | Bead keys in this unit | Carried elsewhere |
|---|---|---|---|
| GATE-PHYLUM-01 | M3 | none (N17b-15 tests its known answers and the lidar and equal-Σx controls) | Expected from U17a: seed BD-285 |
| GATE-PHYLUM-02 | M2 | N17b-06 (the full gate: 512 designs, every back end, three capacities, 100 co-tenancies); BD-170 (`batch-cpu` rows); N17b-02 (kernel level) | Device mechanisms: seed BD-183 (U21b) |
| GATE-PHYLUM-03 | M2 | BD-186 | — |
| GATE-PHYLUM-04 | M2 | BD-184 | — |
| GATE-PHYLUM-05 | M3 | BD-286 | — |
| GATE-PHYLUM-06 | M4 | BD-353 | — |
| GATE-PHYLUM-07 | M3 | none (N17b-15 tests generator reproducibility and exact probabilities) | Expected from U17a: seed BD-287 |
| GATE-PHYLUM-08 | M4 | BD-342 | — |
| GATE-PHYLUM-09 | M2 (defined in §17.21, not in the §40 tables) | N17b-27 | — |
| KILL-PHYLUM | M2 | N17b-10 (decision); BD-186 (verdict by the pre-declared rules) | KILL-BUCKET, its device-side statement, is listed on BD-186 and belongs to the device chapter |
| KILL-FDGRAD | M4 | N17b-20 (first reading, from the pilot); BD-353 (final reading, end-point confirmation) | — |
| KILL-LIMBKERNEL | M2 | N17b-28 (first reading, from the pilots); N17b-27 (final reading, the ratio after one repair cycle) | — |
| KILL-QUOTIENT | M3 | none | Expected from U17a: seed BD-285 ("KILL-QUOTIENT decided") |
| KILL-FAMILYLANE | M4 | none | Expected from U17a: seed BD-356 ("KILL-FAMILYLANE decided") |
| KILL-SPECIALIZE | M4 | none (N17b-24 produces arms (b) and (c) of its pilot data; N17b-28 cites its ceiling) | Expected from the device chapter's seed BD-371 ("KILL-SPECIALIZE decided"), with §17.9 owned by U17a |

## (c) Counts

- Known answers of the chapter cited with their values in test plans: 22 (KA-365 to KA-386). The registry bead is U17a's. Known answers of other chapters cited: KA-345, KA-346, KA-538, KA-264, KA-051.
- Counterexamples carried in N17-CX: 14 numbered (CX-153 to CX-166) and 2 stated in §17.21 without a catalogue number. Cross-referenced from other chapters: CX-024, CX-058, CX-059, CX-129, CX-146, CX-222, CX-224.
- Refusal codes carried in N17-REF: 7 (FROB-REF-160, 161, 162, 540, 541, 740, 741).
- Invariants and proof obligations: 7 + 3, all indexed by N17b-15.
- Open questions: 5 (three decision beads). Kill gates decided in this unit: 3. Pilots: 5 (N17b-01, N17b-16, N17b-19, N17b-23, N17b-24).

## (d) Needs from other slices

| Bead | Need |
|---|---|
| BD-170 | §14.9: lowering of a compiled model to a variant record in a size bucket, with exact rational roll-up of welded sub-assemblies rounded once |
| BD-184, N17b-01, N17b-08 | §39.5, §39.21: pinned incumbent revisions and the rival audit's freeze |
| BD-186 | §16.19: arm E (row-class solve queues); §36.5: the ceiling-before-speed check (seed BD-354 is one milestone later) |
| BD-286 | §17.11: mapping of a world address through an element of Γ_obs by the scene generator; §15.10: the full bus analysis with time-resolved profiles |
| BD-342 | §39 and §24.9: the design benchmark and its acceptance rule; §23: a family policy as the shared controller of a generation |
| BD-353 | §23, §28.9: a specialist or fine-tuned policy for end-point confirmation; §28.4: analysis A-16 as the consumer of the sensitivity report |
| N17-CX | §38.11: catalogue numbers for the two §17.21 counterexamples |
| N17-REF | §17.11: the raising site of FROB-REF-161 for a weight tie; §23: the raising site of FROB-REF-541 at family-policy scoring |
| N17b-02 | §16.6, §21.5: level kernels in the form that reads structure through a per-world view and masks screw columns by count |
| N17b-03, N17b-23 | §17.10: a generator able to emit 512 topologies at M2; §14.10: kinematic definitions of the fourteen later archetypes and of wheel, wheel-leg, track and arm limbs |
| N17b-04 | §16.11: the `referee_coverage` block; §32.8: pagination at a committed frontier |
| N17b-05 | §32.11: the dispatcher's free-slot policy and KILL-SLOT; §21.10: counters of kernel compiles and graph captures |
| N17b-06 | §32.11: the daemon's dispatcher re-running the fixture |
| N17b-07 | §36.14: ratification of the envelope classes (OPEN-SPARK-2) |
| N17b-10, N17b-20, N17b-28 | §43: the M2 and M4 exit reviews listing the kill decisions; §21.11: KILL-BUCKET's record; §28.8, §28.23: the optimization chapter's consumer of KILL-FDGRAD |
| N17b-11 | §21.25: OPEN-SPARK-3, the same decision from the device side |
| N17b-12 | §32.12: MCP tool generation and task clusters |
| N17b-13 | §28.14: the memo graph's node record and shadow-recompute hook |
| N17b-14 | §28.26: the reuse ledger's columns |
| N17b-17, N17b-29 | §33: admission of the suite's archive crate; §28.10: strata and rank-agreement rows; §24.8: registered metrics for speed and energy per metre |
| N17b-18 | §28.7: lineage merge calling the same merge function; §32.13: scopes for reading another agent's map |
| N17b-19 | §28.8: a CMA-ES generation runner before the optimizer bead exists |
| N17b-21 | §14.4: the retraction of each lever manifold as an operation of the design language |
| N17b-22 | §18, §29: a per-step digest of each world's active contact set; §24.8: a smoothness attribute on metrics |
| N17b-24 | §17.9, §21.18: the three-arm specialization pilot and its harness |
| N17b-25, N17b-26 | §14, §14.6: canonical limb ordering and the limb port interface; §21.7: the exact-rewrite rule of the derivation toolchain; §16.6: the tree-operator route |
| N17b-27 | §24.9: the design corpus; §38.8: the pinned corpus of episodes |

## (e) Contradictions, gaps and what was done

1. **Default bucket set, 32 against 36.** §17.5, §21.11 and OPEN-SPARK-3 give {12, 18, 24, 36, 48, 64}; the type sketch of Appendix I and CX-222 print 32. The chapter's set is used throughout; N17b-11 freezes the set and corrects the sketch.
2. **`K-variant-gather` is named and never specified** (§21.5 lists it; the title of BD-170 names it). N17b-02 defines it as the one kernel that turns a world's model slot into a per-world view, labelled as a bead-level design with a contract and tests.
3. **The head-to-head is at M2 but draws on M3 and M4 material**: the generators of §17.10 (BD-287, M3), fourteen archetypes (M3) and track and tyre contact laws (M4). N17b-03 builds an M2 fixture from the M1 pilot sampler and kinematic limb definitions, prints the limitation, and regenerates under a new identity later.
4. **"The design corpus" is cited by M1 and M2 measurements but is built at M3** (§17.14 pilot, §17.21 paper measurement, GATE-PHYLUM-09's coverage clause). BD-144 had already narrowed the M1 pilot to the first archetypes; N17b-23 follows it, and N17b-27 records GATE-PHYLUM-09 as `partial` with an interim coverage figure until the corpus exists.
5. **Exit code of FROB-REF-740 and -741.** The exit table of §32.4 names `70x` (exit 4) and `72x`, `73x` (exit 5) and not `74x`. N17-REF maps both to exit 4 as refusals at admission.
6. **"Padding waste" is not defined.** Read as Ω₁ − 1 inside a bucket (consistent with "1.25 at q = 1.25" and the 25% limit), with a population-level form in N17b-11.
7. **GATE-PHYLUM-03's throughput statistic** is worded two ways ("harmonic mean of the homogeneous rates"; "relative to a homogeneous batch of each bucket's largest design") and "cohorts of at least 256 designs" does not fit a 512-design population split over buckets. BD-186 fixes one statistic (the harmonic mean over buckets of the largest member's homogeneous rate, measured at populations of 256 and 512 designs) and says so.
8. **Between the pass at 0.70 and the kill below 0.5 the plan is silent**, and "wins at K ≤ 128" does not say whether every K is meant. BD-186 reads "every tested K" and routes the in-between and split cases to a recorded FAIL and a discretionary decision in N17b-10. KILL-PHYLUM has no "after one optimization pass" clause while KILL-BUCKET does; at most one pass is allowed.
9. **§17.12 is Frontier and names neither a pilot nor a kill gate.** N17b-16 defines a pilot from the stated hypothesis and hazard; BD-342 names the fallback without minting a kill identifier.
10. **"Variance ratio of paired to independent differences at least 2"** is read as the reduction factor Var(independent)/Var(paired).
11. **Kill gates whose trigger has clauses decidable before and after the mechanism exists** (KILL-FDGRAD: pilot falsifier, then end-point confirmation; KILL-LIMBKERNEL: pilot and coverage, then the ratio after a repair cycle) are split into a first reading (a decision bead the work depends on) and a final reading inside the gate bead, to keep dependencies acyclic.
12. **KILL-LIMBKERNEL and GATE-PHYLUM-09** exist only in §17.21 and §40's late-subsection table; the kill gate was not in the brief's list. Both are carried (N17b-27, N17b-28).
13. **The two counterexamples of §17.21 have no Appendix N number.** They are in N17-CX under working names.
14. **GATE-PHYLUM-08 names "the benchmark of §39"**, while the related bet and gate name the design benchmark of §24. One benchmark is assumed (need recorded on BD-342).
15. **GATE-PHYLUM-06's claim type** is "known answer and method" in §17.19 and "design comparison" in the §40 table; §40's own note (the variance-ratio and sign-agreement comparison governs) is followed.
16. **GATE-PHYLUM-02 requires slot dispatch at M2**, while the daemon's dispatcher is an M3 seed. N17b-06 uses environment-level slot placement available at M2.
17. **GATE-PHYLUM-09's ratio "at 8,192 worlds" names no workload.** N17b-27 gates on the reference quadruped and reports the humanoid and the mixed workload beside it.
18. **`phylum.map` is a read verb that merges maps.** N17b-18 has the merge return a content-addressed artifact and move no campaign head.
19. **"Joint class" in §17.21** is read as the column count of the screw basis, so an archetype key is a depth and a column-count sequence; N17b-23 and N17b-25 state it.
20. **The seed title of BD-184 names `phylum.sweep`.** The verb has its own bead (N17b-04) and the admission mechanism another (N17b-05); BD-184 keeps the latency campaign and the gate.
21. **Fixture numbers the chapter does not give** (the grazing body, the standing point mass, the corpus strata, the decision criteria of the three measurement-decided open questions) are chosen in the beads that use them and marked as bead-level design.
