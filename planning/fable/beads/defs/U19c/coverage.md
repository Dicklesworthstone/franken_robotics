# Coverage of drafting unit U19c

Chapter §19 Habitat: scenes, living worlds, clearance, retrofits. This unit owns §19.13 to §19.22, four seed beads
(BD-163, BD-164, BD-235, BD-236) and the registries N19-CX and N19-REF. It wrote 47 bead files: 4 seed beads, 41 new
beads (`N19c-01` to `N19c-41`) and 2 registry beads. The known-answer registry `N19-KA` is U19a's.

Seed beads written elsewhere whose mechanism lies in these subsections: BD-398 (real-place ingestion, `scene.import`,
INT-FDGR-01; U33a) and BD-291 (probe and instrument design, `mission.distinguish`, `scene.probe-request`; U25c). The
new beads of this unit carry every statement of §19.13 in full and treat those seeds as related or as prerequisites.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §19.13 | Ingesting real places [F] | N19c-01 (pilot and falsifier); N19c-02 (scale labels, `FROB-REF-304`, scale term of u_reg); N19c-03 (three-valued occupancy, unknown mask, treatment per consumer, INV-HAB-04, `FROB-REF-305`, ADR-HAB-10); N19c-04 (sources, adapters, what import adds, assumed cards, materials not inferred from appearance, loss report, privacy); N19c-05 (`scene.probe-request`); N19c-06 (surveyed proxy: interim posture of FD-FSS-01 and fallback of KILL-INGEST); N19c-07 (GATE-HAB-03 script); N19c-08 (KILL-INGEST); N19c-09 (OPEN-HAB-5); N19c-10 (OPEN-HAB-6). Seed BD-398 (verb `scene.import`, INT-FDGR-01) is another unit's |
| §19.14 | Train and test hygiene | BD-235 (`SceneSplit`, `scene.split`, INV-HAB-09, `FROB-REF-311`, building-level holdouts, feasibility counts); N19c-11 (groups: lineage, mirror canonical form, pool families, buildings, layout fingerprint); N19c-12 (scoring-time check, training scene manifests, `FROB-REF-502`, GATE-ARENA-02); N19c-13 (resolved values stored, audit after the fact, counts and recount, INV-HAB-05); N19c-14 (sealed families: a handle that returns scores, never scenes); N19c-15 (GATE-HAB-14 and GATE-ARENA-02 script) |
| §19.15 | Memory arithmetic, brick pools and the world bank | N19c-16 (dense, band and directory arithmetic; the three tables; one scene per world; ADR-HAB-05); BD-164 (brick pool, content addressing, directory per variant, three kinds of sharing, both branches of KILL-POOL); N19c-17 (GATE-HAB-13 script); N19c-18 (KILL-POOL at M2); N19c-19 (GATE-HAB-13 and KILL-POOL on the seven archetypes at M3; variant cost); BD-236 (world bank: where, admission and cap, `FROB-REF-314`, ledger, `scene.bank`, ceiling); N19c-20 (segments, largest buffer, 31-buffer limit, launch groups); N19c-21 (eviction by recompute cost and recency; the sibling policy as pattern); N19c-22 (scene change at reset through the 16-bit variant index); N19c-23 (what crosses machines, INV-HAB-10, no zero-copy claim); N19c-24 (GATE-HAB-12 script); N19c-25 (breadth experiment); N19c-26 (KILL-BANK-BREADTH and OPEN-HAB-7) |
| §19.16 | Refusals | N19-REF (all seventeen codes with trigger, repairs, exit code and raising bead; the note on `FROB-REF-502`; three unnumbered refusals). Raised in this unit: `FROB-REF-304` (N19c-02), `-305` (N19c-03), `-311` (BD-235), `-314` (BD-236), and `-502` enforced in N19c-12 |
| §19.17 | Invariants and proof obligations | N19c-27 (ledger of INV-HAB-01 to -10 and PO-HAB-01 to -07, binding audit, crate-wide tests for INV-HAB-01 and -02). Carried with tests in this unit: INV-HAB-03 (N19c-22), INV-HAB-04 (N19c-03), INV-HAB-05 (N19c-13), INV-HAB-09 (BD-235, N19c-11, N19c-15), INV-HAB-10 (N19c-23), PO-HAB-01 (BD-163, emission) |
| §19.18 | Known answers | N19c-28 (the eighteen cases as one runnable battery with their negative controls, three designed controls for the rows that state none, eight extension rows for §19.21 and §19.22). The numbered registry is N19-KA (U19a) |
| §19.19 | Gates and kill gates | N19c-29 (register rows for GATE-HAB-01 to -16 and the eleven kill gates, measuring beads, consistency checks); measured or decided in this unit: see (b) |
| §19.20 | Open questions | N19c-30 (OPEN-HAB-1, owner decision); N19c-31 (OPEN-HAB-2, apron experiment and decision); N19c-32 (OPEN-HAB-3, spacings); N19c-09 (OPEN-HAB-5); N19c-10 (OPEN-HAB-6); N19c-26 (OPEN-HAB-7). OPEN-HAB-4 and OPEN-HAB-8 are U19b's (N19b-43, N19b-22) |
| §19.21 | Far-field field reuse across forks (added late) | N19c-33 (pilot, bound table, ceiling, dirty-cone arithmetic, falsifier); N19c-34 (tile contract, content and event digests, TileKey, store, algorithm, preconditions); N19c-35 (retention over the fork window, store budget, `sim.estimate`, the a ≥ 0.8 rule, `field_reuse` receipt, `--no-field-reuse`); N19c-36 (GATE-HAB-15 script); N19c-37 (KILL-FARFIELD) |
| §19.22 | Scaled instances: distances, normals and ray parameters in world metres (added late) | N19c-38 (rules 1 and 2, singular-value interval, typed local distance, refinement, scaled brick sharing); N19c-39 (rule 3, ray parameters, viewer pick contract, sibling ray adapter); N19c-40 (GATE-HAB-16 script); BD-163 (the compiler bakes scales, so a compiled pack is in world metres) |
| §19.4, §19.1 (read for BD-163) | The scene compiler; verbs | BD-163 (driver, eleven stages, stage 5 emission, `scene.compile`, `scene.show`, `scene.list`); N19c-41 (end-to-end script of `scene.show` and `scene.list`). Stage beads are U19a's (N19a-10 to N19a-20, N19a-42) |

## (b) Gates and kill gates of §19, and the beads that measure or decide them

| Gate | Measured or decided by this unit | Expected from another unit |
|---|---|---|
| GATE-HAB-01 | BD-163 (field rows on its own fixtures) | N19a-42, N19a-14, N19a-16, N19a-17, N19a-25 (ripple clause) (U19a) |
| GATE-HAB-02 | N19c-30 names it (OPEN-HAB-1 blocks its realism reading only) | N19a-45, BD-234, N19a-26 (U19a); BD-225, BD-226 (U14b) |
| GATE-HAB-03 | N19c-07 (script and record); N19c-02 to N19c-05 (mechanisms) | BD-398 (U33a) for the verb and INT-FDGR-01 |
| GATE-HAB-04 | none | N19a-43, BD-228 (U19a) |
| GATE-HAB-05 | none | N19a-44, BD-229 (U19a) |
| GATE-HAB-06 | none | N19b-06 (U19b); BD-230 (U18a) |
| GATE-HAB-07 | none | N19b-10 (U19b); BD-413 (U33b) |
| GATE-HAB-08 | none | N19b-14, N19b-18 (U19b) |
| GATE-HAB-09 | none | N19b-16 (U19b) |
| GATE-HAB-10 | none | N19b-35, BD-233 (U19b) |
| GATE-HAB-11 | none | N19b-41, BD-369 (U19b) |
| GATE-HAB-12 | N19c-24 (script, both machines); BD-236, N19c-20 to N19c-23 (mechanisms) | none |
| GATE-HAB-13 | N19c-17 (M2 battery), N19c-19 (seven archetypes at M3); BD-164 (mechanism) | none |
| GATE-HAB-14 | N19c-15 (script); BD-235, N19c-11, N19c-12, N19c-13, N19c-14 | counts supplied by N19a-26 (U19a) |
| GATE-HAB-15 (not in the main tables of §40; §40.8) | N19c-36 (script); N19c-34, N19c-35 | none |
| GATE-HAB-16 (§40.9) | N19c-40 (script); N19c-38, N19c-39; BD-163 | the viewer's pick and the renderer route (needs) |
| GATE-ARENA-02 (a §24 gate named by seed BD-235) | N19c-12, N19c-15; BD-235 | the Arena unit may also carry it |
| KILL-INGEST | N19c-08; N19c-01 (pilot), N19c-06 (fallback) | BD-398 names it in its done-condition (U33a) |
| KILL-POOL | N19c-18 (M2), N19c-19 (M3 re-read); BD-164 (both branches) | none |
| KILL-BANK-BREADTH | N19c-26; N19c-25 (experiment); BD-236 | none |
| KILL-FARFIELD (§40.8) | N19c-37; N19c-33 (pilot) | none |
| KILL-ADVECT, KILL-LIVING, KILL-PROXY, KILL-SLOSH, KILL-FCLEAR, KILL-RETROFIT | none | N19b-11, N19b-20, N19b-19, N19b-15, N19b-36, N19b-42 (U19b) |
| KILL-DRESSING | none | N19a-41 (U19a) |

All sixteen gates and eleven kill gates of the chapter are carried by some unit; none needed a stand-in
gate-measurement bead. N19c-29 holds the register rows for all twenty-seven and the check that each has a measurer.

## (c) Counts

- Subsections: 10 (§19.13 to §19.22), two of them added late.
- Gates measured here: 6 (GATE-HAB-03, -12, -13, -14, -15, -16), with contributions to GATE-HAB-01 and GATE-ARENA-02.
  Kill gates decided here: 4 (KILL-INGEST, KILL-POOL, KILL-BANK-BREADTH, KILL-FARFIELD).
- Known answers: 27 of the chapter's 39 are cited with their values (KA-426 to -431, -436, -438, -440, -441, -443,
  -444, -446 to -449, -451, -453, -456 to -464); 7 belong to this unit's mechanisms (KA-456, -457, -459 to -463), with
  contributions to KA-426 to -428, -458 and -464. Eight unnumbered known answers of §19.21 and §19.22 are registered
  as extension rows HKA-A to HKA-H in N19c-28.
- Counterexamples: 16 of 16 in the registry N19-CX (CX-184 to CX-199), 3 of them owned by this unit's mechanisms
  (CX-196, CX-197, CX-198), plus 4 unnumbered extension rows (CXH-A to CXH-D) from §19.21 and §19.22. Also cited:
  CX-227, CX-229 (§21.20).
- Refusal codes: 17 of 17 in the registry N19-REF; 4 raised by this unit's beads (`FROB-REF-304`, `-305`, `-311`,
  `-314`), plus `FROB-REF-502` enforced in N19c-12. Three refusals the chapter implies and does not number are listed.
- Invariants and proof obligations: all 17 in the ledger N19c-27.
- Open questions: OPEN-HAB-1, -2, -3, -5, -6, -7 each have a bead here.
- Verbs: `scene.compile`, `scene.show`, `scene.list` (BD-163, N19c-41), `scene.split` (BD-235), `scene.bank`
  (BD-236), `scene.probe-request` (N19c-05).
- Pilots: N19c-01 (ingestion), N19c-33 (far-field reuse).
- Sizes: the largest bead is BD-163 (192 lines of body), the smallest N19c-10 (81 lines).

## (d) Needs from other slices

Cross-unit prerequisites are written in each bead's `needs` field, with the key where one exists. The charter's
self-check allows only seed, reserved and own keys in `deps`, so prerequisites that are new beads of U19a and U19b
are named in `needs` text with their key ("bead N19a-11, drafted by unit U19a; a blocking prerequisite") and repeated
in `related`. The lead can turn each into a dependency. They are: N19b-01 (BD-163, BD-164, N19c-16, N19c-38);
N19a-02 (BD-163, BD-235, BD-236, N19c-05, N19c-41); N19a-10, -11, -12, -13, -16, -17 (BD-163); N19a-05 (BD-235,
N19c-11); N19a-20 (N19c-11, N19c-23); N19a-13 and N19b-30 (N19c-03); N19a-10 and N19a-21 (N19c-04); N19b-32
(N19c-05); N19a-39 (N19c-06); N19a-26 (N19c-13); N19a-09 (N19c-22); N19-KA (N19c-28); N19b-08, N19b-09 (N19c-34);
N19b-10 (N19c-36); N19a-11 (N19c-38).

Prerequisites with no key found:

| Need | Asked by | Owner |
|---|---|---|
| An admission test for `fs-rep-sdf`, `fs-rep-mesh`, `fs-rep-frep`, `fs-topo` through the airlock's geometry group | BD-163 | Suite consumption (U33) |
| A capability scope for bank administration (admit, evict, pin) | BD-236 | Tether, safety |
| Compute-envelope approval: the bank resident far above 16 GiB; fifteen training runs of the breadth experiment | N19c-24, N19c-25 | Owner |
| The placement planner's launch groups honouring the segment constraint; residency map | N19c-20 | Spark (BD-178) |
| Masked-reset rewrite of a world row and the between-chunk host-write lease; addressed randomness | N19c-22 | Spark, Tether (BD-183, BD-295, BD-180) |
| The transfer-class meter and `FROB-REF-618`; epoch-fenced leases and `FROB-REF-614` | N19c-23 | Spark (BD-173, BD-370) |
| Policy manifest `trained_on.scenes` and training lineage through inheritance and distillation | N19c-12 | Cortex |
| The qualification service (sealed key, commit, receipts) and scopes `qual:admin`, `qual:run` | N19c-14, N19c-12 | Arena (BD-259, BD-261), safety (BD-258) |
| Study plan and pre-registration record for a paired, anytime-valid comparison with clusters as units | N19c-25 | Arena |
| Generators declare which parameters and seed streams are structural, mirror, clutter or condition | N19c-11 | U14b (BD-225, BD-226), U18a (BD-165) |
| Scorecard accounting for episodes excluded as `Degraded: contact-with-unknown` | N19c-03, N19c-10 | Arena (BD-254) |
| Hooks: a mission answer whose inner and outer answers differ because of an unknown region; an assumed-card reversal found by the adversarial search | N19c-05 | Augur, Nemesis |
| Schema fixtures of the drone reconstruction project and the subprocess posture (INT-FDGR-01) | N19c-02, N19c-04 | Suite consumption (BD-398) |
| Private scope, export flag and mark inheritance (`FROB-REF-731`) | N19c-07, N19c-41 | Safety (BD-305) |
| A reference robot, baseline controller and short navigation task for evaluation in an ingested place | N19c-07 | Arena |
| The phase ledger that attributes step time to stages (for the share a) | N19c-35 | Performance program |
| An admitted two-room spill fixture on a tier where film tiles run | N19c-36 | U19b (advected film), Arena |
| The viewer's picking path and a headless test entry for it; the admitted renderer route (INT-F3D-01) | N19c-39, N19c-40 | Tether (viewer), suite consumption |
| OPEN-TRAC-1 (dense bricks or a sparse tree at 1 cm), decided from the same measured memory | N19c-32 | Traction |
| Theorem-card numbers for PO-HAB-02 and PO-HAB-07 | N19c-27 | Lemma |
| Refusal codes for: a pack with incomplete acceptance; reuse under D3 or D4 or with no stencil; a reset to a variant not resident in the bound segment | N19-REF, N19c-34 | Registry owner (318 and 319 are free) |
| Appendix M and N numbers for the known answers and counterexamples of §19.21 and §19.22 | N19c-28, N19-CX | Registry owner |
| Regenerated §40 tables with rows for GATE-HAB-15, GATE-HAB-16, KILL-FARFIELD | N19c-29 | Gate-register owner |

Requests to other units' beads: BD-398 (U33a) should depend on N19c-02 to N19c-05 and should not depend on N19c-07 or
N19c-08, which depend on it; BD-226 and BD-225 (U14b) expose parameter roles for groups; the stage beads of U19a that
depend on BD-163 plug into the driver's `Stage` trait defined there.

## (e) Contradictions, gaps and what was done

1. **Seed done-conditions that name gates and kill decisions.** BD-163 ("GATE-HAB-01"), BD-164 ("GATE-HAB-13;
   KILL-POOL decided"), BD-235 and BD-236 are depended on by many beads (BD-226 depends on BD-164; the U19a stage
   beads on BD-163), so they cannot be capstones without delaying or cycling the graph. They are written as mechanism
   beads; each acceptance list states which bead records the gate and which decides the kill gate, and that the
   capability is reported as implemented only when those are closed. U19b chose capstones for its seeds; the lead may
   want one convention.
2. **GATE-HAB-13 and KILL-POOL are due at M2 on an "archetype battery" that exists at M3.** The M2 run and decision
   use the terrain families and authored stand-ins (N19c-17, N19c-18); the gate's per-archetype rows and the kill
   gate are re-read on the seven proxies at M3 (N19c-19), and that verdict is the decision of record.
3. **KILL-POOL's statistic.** "Measured deduplication on the archetype battery" is read as band bricks over unique
   bricks of the battery's union in one pool, with terrain and the mesh import reported beside it (N19c-18). A repair
   cycle is allowed when the cause is region labels in the brick record.
4. **Region palette inside the brick record.** Content addressing over the full record makes sharing depend on equal
   region identifiers; BD-164 adds a rule (identifiers per module type) and measures deduplication both ways.
5. **"Dense per-variant fields" as the fallback** is read as private per-variant storage behind the same directory
   trait, in band or fully dense form; the dense table is the planning figure.
6. **KA-460.** Appendix M's large-memory column divides 189 by sizes in GB; the chapter uses 203 GB (189 GiB). The
   chapter's column is asserted and the other recorded (BD-236, N19c-16).
7. **Building surface area** is not stated; the printed band figures imply about 65,000 m² (N19c-16).
8. **"About five times what both cards hold"**: 203 GB against 48 GB of card memory is 4.2, against 32 to 38 GB
   usable 5.3 to 6.3. Kept as planning arithmetic with the numbers shown.
9. **Variant index width.** Appendix I's world row has a 32-bit scene field; §19.15 and §21.11 say 16 bits. 16 bits
   is used.
10. **The cap on a CUDA card.** The cap is defined on "the device's measured working-set limit" (a Metal quantity);
    for cards the plan gives only "12–16 GB assumed". The same 85% fraction of card memory is used (N19c-24).
11. **`FROB-REF-502` has no repairs in the plan.** N19c-12 proposes three (DESIGN) and adds the group rule to the
    check, since identifiers alone do not prevent leakage.
12. **How group membership is established is not specified.** N19c-11 derives groups from provenance (parameter
    roles, lineage, pool families, building identity) and cross-checks with a mirror-invariant fingerprint; the
    fingerprint, its eight symmetries and the thresholds 0.98 and 50% are DESIGN and labelled.
13. **Retrofit variant at M3.** GATE-HAB-14 plants a retrofit variant at M3, while the catalog is M4. A typed scene
    edit with recorded lineage stands in (N19c-15), to be re-run with a catalog kit.
14. **Split rules over per-episode quantities** (held-out material conditions) are a DESIGN extension (N19c-13).
15. **Sealed families** cross the Habitat and Arena boundary; the handle, derivation and wrapper are in `frob-scene`
    (N19c-14), the service elsewhere. A colliding sealed cluster is skipped and logged, the one recorded exception to
    "no scene is replaced".
16. **`scene.probe-request`.** The verb table of U19a attributes it to BD-291 (M3, mission probes); Appendix J and
    §19.13 define it at M5 as a scene verb. N19c-05 implements it and depends on BD-291. The accuracy rule
    (a = half the nominal slack less the irreducible error) and the 1/20 default are DESIGN.
17. **Scale uncertainty in u_reg.** The plan says only "with the witness's uncertainty in u_reg"; N19c-02 derives
    u_scale = ε_c·(ρ + c). The ±5% default for an unstated prior is DESIGN.
18. **Unknown regions.** Mask granularity (one bit per coarse cell), "adjacent" (one coarse cell), the hole-fill
    neighbourhood (dilation by T) and the realization of "solid keep-out" with "free for cuts" (a reserved region
    class, c_hi = +∞) are DESIGN (N19c-03).
19. **Real-place gate and privacy.** A Reality gate cannot run in shared CI; N19c-07 has a rehearsal lane that can
    never record a pass and a private lane whose log is scrubbed.
20. **Eviction** is specified only as "by recompute cost and recency"; N19c-21 proposes a cost-aware greedy policy
    with ageing and compares it with the three-queue pattern on a recorded trace.
21. **A scene change to a variant in another pool segment** is not addressed by the plan; N19c-22 holds the world
    masked until the chunk boundary (results unchanged) and asks for a refusal code for the unservable case.
22. **Far-field pilot scale.** "A 256 × 256 tile grid" with 16 × 16-cell tiles and 100 forks in two arms is about
    2.5 × 10¹² cell updates, more than "two engineer-days" suggests; N19c-33 allows smaller tiles or a registered
    subsample without changing the falsifier, and records that the pilot's a is an upper bound.
23. **"Reuse is refused" under D3, D4 or without a stencil** has no code; N19c-34 degrades to full recomputation in
    `auto` mode and refuses only in `require` mode.
24. **KILL-FARFIELD and the graph.** The mechanism beads depend on the pilot, whose verdict gates them; the kill
    decision bead (N19c-37) records that verdict as stage A and decides stage B after the gate. The cost clause of
    GATE-HAB-15 is treated as a gate criterion, not a kill trigger, as the trigger text says.
25. **GATE-HAB-16 at M2** has a clause through an adapter over a sibling renderer admitted at M3, and one through
    the viewer's pick; N19c-40 records the adapter clause in two modes and marks rows `not run` where a component is
    absent.
26. **Decision thresholds proposed here** (OPEN-HAB-2: 5% step improvement; OPEN-HAB-3: 5% field-bound unresolved
    fits; OPEN-HAB-6: 5% and 20% excluded share; breadth experiment: five seeds, 200 held-out groups, α = 0.05) are
    proposals for the owners, labelled DESIGN or TARGET in their beads.
27. **Appendix O prints `FROB-REF-311`** with its repair folded into the meaning; N19-REF separates them.
28. **Bead count.** 47 files is slightly above the typical range; the unit covers ten subsections, two registries,
    six gates, four kill gates and six open questions.

## (f) Added by the integration pass

- N19c-42 (new, M2): the decision and registry rows for the four Habitat refusals the chapter states without a code
  (a run on an unaccepted pack; a scheduled variant resident in no bound segment; field reuse required under D3, D4
  or without a stencil; a pack whose recomputed identity differs from the stated one). Proposed numbers 318, 319, 322
  and 323; BD-228, BD-162, N19a-20, N19c-22, N19c-23 and N19c-34 raise them. N19-REF's interim wording and BD-228's
  were different and both now point here.
- N19c-11 takes the role tables of the shipped generators into its Scope (no generator bead declares parameter
  roles) and depends on N19b-44 for lineage along typed edits; N19c-34 depends on the KILL-ADVECT decision N19b-11.
- N19c-04 extends the loss report schema of the asset lane N19a-46.
