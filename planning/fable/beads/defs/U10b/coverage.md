# Coverage of drafting unit U10b: constructions C-16 to C-27 of §10

31 bead files: 12 specification beads (`SPEC-C-16` … `SPEC-C-27`), 8 specification parts, 10 pilot beads and
1 end-to-end audit bead (`N10b-01` … `N10b-19`). This unit owns no seed bead.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads | What each carries |
|---|---|---|---|
| §C-16 | A gait is a hybrid orbit: saltation, return maps, labeled derivatives | SPEC-C-16, N10b-01, N10b-02, N10b-03 | SPEC-C-16: automaton, events, simultaneity rule, saltation matrix and event-time shift with derivation, bouncing ball, return map, monodromy of a relative periodic orbit, Floquet multipliers, rimless wheel, compliant-tier statement, orbit adjoint, transient growth, retirement rule, foundations. N10b-01: the nine-label table, the four refusal causes of FROB-REF-263 with the normalization, the symmetric-tie exception, validation of `saltation` derivatives, grazing fixture. N10b-02: the commuting identity (T-18) with the two-ball derivation, its honest limits, stratum counts 27/162 and 45/324, simultaneous impacts. N10b-03: pilot (100 cases, 20 controls), falsifier, ceiling, fallback |
| §C-17 | A mission is a quantified set of designs | SPEC-C-17, N10b-04, N10b-05 | SPEC-C-17: the six ingredients of a query, the formula for F, the triple I ⊆ F ⊆ O and what each cell carries, the seven rules attached to the definition, quantifier order, strictness, chance constraints, honest limits, foundations, type sketch. N10b-04: the four proof rules with side conditions and proof kinds, the non-convexity fixture (841/800 N), the invalid-fixture rule, paving limits. N10b-05: pilot (static, timing, energy families against generic comparators), falsifier, ceiling, tie rule |
| §C-18 | Assumptions form a lattice, and verdicts have polarity | SPEC-C-18, N10b-06, N10b-07 | SPEC-C-18: monotonicity of F, the six-row polarity table, the record on a certificate, the twelve docking moves with fresh evaluations, the 33/32 gain, per-world against joint answers, the stale-verdict control, honest limits, PO-LEMMA-03. N10b-06: the Galois connection, closed pairs, the separating set and the order on worlds for `mission.separate`. N10b-07: pilot (twelve scripted amendments), falsifier, ceiling, fallback |
| §C-19 | Information has a clock: causal policies and indistinguishability | SPEC-C-19, N10b-08, N10b-09, N10b-10 | SPEC-C-19: the seven instants, ordering and same-instant order, causal policies, the witness and its argument, the total-variation cap, repeated reads, the three engineering answers, honest limits, foundations. N10b-08: the docking fixture in full (model, six derivations, grid 344/217/9, late packet, third gain 1/65, arithmetic FACT). N10b-09: label count as a hypergraph colouring; schedule classes (720 orderings, 8 classes). N10b-10: pilot (closed form against a scenario-tree linear program; a planted stale-read race) |
| §C-20 | Observability is a Gramian modulo a gauge | SPEC-C-20 | The Gramian, its empirical form, the four-dimensional gauge, the index, Fisher information, the locality rule with the global ambiguity search, range-anchor and corridor fixtures. Solid: no pilot |
| §C-21 | Designs are canonical up to the similarity group | SPEC-C-21, N10b-11 | SPEC-C-21: the group, the gravity-fixing subgroup, canonical scales and derived scales, the list of groups, the exact power-of-two test with the representation-constant rule, honest limits, foundations. N10b-11: pilot for canonical policy inputs (an arm of the family-policy pilot) |
| §C-22 | A campaign is quotiented by symmetry orbits | SPEC-C-22, N10b-12, N10b-13 | SPEC-C-22: acting group Γ_obs, Burnside count with cycle types, canonical representative with its mapping element, transport of results, wreath product with its two preconditions, hexapod and wreath fixtures. N10b-12: the mission-observable projection, its digest, exact lowering, the ballast fixture (735,471 placements, 24,546 classes). N10b-13: pilot (ballast end to end with replay) |
| §C-23 | A family policy is a section, and edits transport it | SPEC-C-23, N10b-14, N10b-15 | SPEC-C-23: bundle and section, tokens, dimensionless features, permutation equivariance and orbit tying, naturality, the evidence figures, design rules, economics, the statements of Pilot 1 and Pilot 2 with kill, ceiling and fallback. N10b-14: transport along an edit (correspondences, Ad and its dual, zero heads, reset, composition), type sketch. N10b-15: Pilot 2 (12 edit pairs, 4 blind). Pilot 1 is the forty-design experiment run by seeds BD-145 and BD-310 (other units) |
| §C-24 | A geometric-algebra policy on the kinematic tree | SPEC-C-24, N10b-16 | SPEC-C-24: multivector tokens, the gravity-preserving group and how features transform, layer requirements, message order, evidence figures, the three-arm pilot statement. N10b-16: the pilot itself (three arms, kill rule, cost, ceiling, fallback) |
| §C-25 | Fidelity tiers are coupled estimators | SPEC-C-25, N10b-17 | SPEC-C-25: control variate with derivation, multilevel estimator and allocation, doubly robust tier correction with unbiasedness and variance, persistence of the audit assignment, stopping rule, honest limits, foundations. N10b-17: pilot (ρ on 200 world addresses; correction against referee-only) |
| §C-26 | Rungs are trusted by rank agreement and conformal margins | SPEC-C-26 | Kendall τ, top band and thresholds, the conformal index and margin, the 2·q̂ survivor rule and the 2α bound, minimum audit size, coverage distribution, ladder-cost numbers, the pilot protocol in full (run by BD-145, BD-310, BD-328), kill, ceiling, fallback |
| §C-27 | Rare failures: bounded weights certify, splitting explores | SPEC-C-27, N10b-18 | SPEC-C-27: the two lanes, zero-failure bound, bounded-weight estimator, truncated lower bound, defensive mixture, splitting variance, Good–Turing unseen mass, scope of a bound, worked counts, foundations. N10b-18: pilot (random-walk fixture and one planted trap) |
| All twelve | cross-cutting | N10b-19 | End-to-end audit: contracts, register rows, law suites with controls, known-answer rows with no open tolerance or control, one fixture identity for shared cases, cross-references, theorem cards, pilot records |

## (b) Gates

Chapter §10 defines no gate, kill gate or admission gate of its own: a construction fixes mathematics and its
test, and each gate lives in the implementing chapter. The table lists the gates that measure each
construction, the beads of this unit that supply their known-answer rows and controls, and the falsifiers that
§10 itself states (the "Pilot and ceiling" lines), each carried by a pilot bead.

| Gate, kill gate or pilot falsifier | Construction | Supplied or carried by (this unit) | Recorded or decided by (other units) |
|---|---|---|---|
| GATE-GAIT-01, -02, -03; KILL-GAIT; GATE-AUGUR-21; KILL-AUGUR-SYMEVENT | C-16 | SPEC-C-16, N10b-01, N10b-02 | BD-333, BD-334, BD-335, BD-426 |
| Pilot falsifier C-16 (any control not refused, or fewer than 72 of 80 regular cases at cosine 0.95) | C-16 | N10b-03 | — |
| GATE-AUGUR-01 to -07; GATE-LEMMA-04; KILL-FAMILYCERT; KILL-AUGUR-TIE; KILL-AUGUR-FALSECERT; GATE-AUGUR-08, -09; KILL-AUGUR-REGION | C-17 | SPEC-C-17, N10b-04 | BD-128 to BD-136, BD-288, BD-289 |
| Pilot falsifier C-17 (any false certificate, or no cell decided that the comparator leaves undecided at equal cost) | C-17 | N10b-05 | — |
| GATE-ATEL-11; GATE-AUGUR-12; GATE-AUGUR-03 (three-gain amendment); GATE-PHYLUM-05; KILL-AUGUR-REUSE; KILL-REUSE; GATE-AUGUR-13; KILL-AUGUR-SEPARATE | C-18 | SPEC-C-18, N10b-06 | BD-128, BD-292, BD-293, BD-347 |
| Pilot falsifier C-18 (one survivor that fresh evaluation contradicts) | C-18 | N10b-07 | — |
| GATE-SENS-04, -05, -09, -12, -13; GATE-AUGUR-03, -10, -19; KILL-ALIAS; KILL-AUGUR-ALIAS; KILL-SKEL-ALIAS; KILL-SCHED; KILL-AUGUR-LABELS | C-19 | SPEC-C-19, N10b-08, N10b-09 | BD-078, BD-098, BD-119, BD-129, BD-134, BD-190, BD-240, BD-363 |
| Pilot falsifier C-19 (one grid cell where the two arms disagree, or the planted race missed) | C-19 | N10b-10 | BD-041 supplies the re-run of the plan-time pilot |
| GATE-SENS-08 | C-20 | SPEC-C-20 | BD-120, BD-239 |
| GATE-GAUNT-03 (relation MR-06); GATE-PHYLUM-01; GATE-CORTEX-11 (re-description clause) | C-21 | SPEC-C-21 | BD-223, BD-285, BD-321 |
| Pilot falsifier C-21 (no gain in held-out return across a size sweep) | C-21 | N10b-11 | — |
| GATE-PHYLUM-01; KILL-QUOTIENT; KILL-AUGUR-CUTS; the M1 counts of BD-049 | C-22 | SPEC-C-22, N10b-12 | BD-049, BD-285, BD-357 |
| Pilot falsifier C-22 (one merged pair whose replays differ in any bit) | C-22 | N10b-13 | — |
| GATE-CORTEX-11, -12, -13; KILL-FAMILY; KILL-INHERIT | C-23 | SPEC-C-23, N10b-14 | BD-320, BD-321, BD-322, BD-328 |
| Pilot 1 kill rule C-23 (zero-shot below 70% of specialist on more than half of the held-out set together with fine-tuning above 0.2 of specialist steps) | C-23 | stated in SPEC-C-23 | BD-145, BD-310, BD-320 run it |
| Pilot 2 falsifier C-23 (transport fails to halve the median adaptation samples) | C-23 | N10b-15 | — |
| GATE-CORTEX-17; KILL-GATREE | C-24 | SPEC-C-24 | BD-439 |
| Pilot kill rule C-24 (neither arm B nor C at least 1.3× faster, nor higher zero-shot return beyond seed spread) | C-24 | N10b-16 | — |
| GATE-FID-08; GATE-CHRON-09; KILL-DR | C-25 | SPEC-C-25 | BD-264 |
| Pilot kill rule C-25 (ρ below 0.7, or failure to beat referee-only by a factor of 2 on high-agreement strata) | C-25 | N10b-17 | — |
| GATE-ATEL-07, -08; GATE-CORTEX-12; GATE-FID-04; GATE-REFLEX-04; KILL-RANK; KILL-LADDER | C-26 | SPEC-C-26 | BD-145, BD-265, BD-310, BD-328, BD-329 |
| Pilot kill rule C-26 (top-of-ranking τ below 0.5 or ρ below 0.7) | C-26 | stated in SPEC-C-26 | BD-145 (pre-registration), BD-310 (execution) |
| GATE-NEMESIS-04, -05; KILL-RARE | C-27 | SPEC-C-27 | BD-276, BD-277 |
| Pilot falsifier C-27 (trap not found at least three times sooner than uniform sampling, or fewer than 1% of forks reach the next level) | C-27 | N10b-18 | — |
| GATE-GAUNT-01, -02, -03, -08 (controls registered; known-answer rows eligible; relations; pilot records) | all | every bead; N10b-19 audits them together | the Gauntlet beads record them |

## (c) Counts

- Known answers carried: **50** (KA-092 to KA-141: C-16 six, C-17 five, C-18 five, C-19 eight, C-20 three, C-21
  four, C-22 five, C-23 three, C-24 one, C-25 four, C-26 two, C-27 four), each with value, tolerance, tier and
  control stated inline. Also bound from other chapters' sections because they are the same fixtures: KA-024,
  KA-036, KA-205, KA-224 and the adopted row "contact strata under the observation subgroup" (45 strata, 324
  transitions).
- Counterexamples carried: **26** (CX-041 to CX-066), plus CX-035 (shared with C-10), CX-105 (§11.8) and
  CX-292 (§26) where they are the control of one of this unit's rows; CX-010 and CX-015 are named as the same
  cases as CX-044 and CX-045.
- Refusal codes: **1** carried in full with trigger, payload, ranked repairs and exit code (FROB-REF-263, whose
  conditions C-16 defines; its registry row is listed in §12.7). **8** cited with meaning and exit code where a
  construction's failure case maps to them: FROB-REF-161, -431, -531, -537, -560, -569, -826, -835.
- Metamorphic relations bound: 10 (MR-04, -05, -06, -08, -10, -11, -17, -23, -24, -33).
- Theorem cards named: 6 (T-09, T-17, T-18, T-23, T-24, T-30). Proof obligation PO-LEMMA-03 is a law test of
  SPEC-C-18.
- Pilot records: 11 (PILOT-C-16 to -19, -21 to -27); 10 have a bead in this unit; PILOT-C-26 is written by the
  seeds BD-145 and BD-310.
- Known-answer rows whose tolerance was `to set` and is set by a bead here (DESIGN, per the rule that such a
  row cannot pass until its derivation file states one): KA-094 (10⁻⁹ relative in `f64`; exact in ℚ), KA-116
  (principal angle at most 10⁻⁸ rad), KA-128 and KA-131 (64 units in the last place; exact for relabeling and
  power-of-two scaling), KA-129 and KA-130 (bit equality), KA-140 (empirical value within a factor of 2).
- Known-answer rows whose control was `none stated` and is registered by a bead here: KA-093, KA-096, KA-103,
  KA-107, KA-108, KA-109, KA-111, KA-112, KA-115, KA-116, KA-118, KA-119, KA-121, KA-125, KA-126, KA-129,
  KA-130, KA-132, KA-133, KA-134, KA-138, KA-139, KA-140 (23 rows of this chapter), and KA-224 of the Lemma chapter
  (bound in SPEC-C-18).

## (d) Needs from other slices

| Need | Owner | Used by |
|---|---|---|
| The construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the law-suite convention (bead N10a-01) | U10a | every SPEC bead and N10b-19 |
| Registry beads N10-KA (rows KA-092 to KA-141) and N10-CX (rows CX-041 to CX-066) | U10a, U10c | every SPEC bead and part |
| Registry rows for KA-224, FROB-REF-263, FROB-REF-826, FROB-REF-835 and the closed proof-kind registry | U12 (Lemma) | SPEC-C-18, N10b-01, SPEC-C-17, N10b-04 |
| Relation rows MR-04, -05, -06, -08, -10, -11, -17, -23, -24, -33 in `relations.toml`; the pilot register `pilots.toml`; the adopted row of §38.18 | Gauntlet units (U38b) | SPEC beads; all pilot beads |
| The stance reference model of construction C-32 (for the stance half of the twelve-move check) | U10c (SPEC-C-32) | SPEC-C-18, N10b-05, N10b-07 |
| Γ_mech and Γ_obs (SPEC-C-09), the tree recursions (SPEC-C-04), the invariant error (SPEC-C-13) | U10a | SPEC-C-22, SPEC-C-23, SPEC-C-24, SPEC-C-20 |
| The forty-design rank-agreement and family-policy pilot (pre-registration BD-145, execution BD-310, the Cortex pilot of BD-320) must carry the protocol stated in SPEC-C-26 §5 and SPEC-C-23 §4 | U34, U28a, U23b | SPEC-C-23, SPEC-C-26, N10b-11, N10b-15 |
| The energy exclusion of the sill fixture as a mission exclusion at M1 (analysis A-20, proof kind `energy-budget`) | U28a (BD-108) and the Augur units | N10b-05 |
| The schedule enumerator over the lab runtime and the race check with KILL-SCHED | U20a | N10b-09, N10b-10 |
| Owner-approved compute envelopes (OPEN-CORTEX-1, OPEN-SPARK-2, the E3 class of WP-79) | owner decision beads of other units | N10b-11, N10b-15, N10b-16 |
| **Edges the lead should add** (a mechanism depends on its pilot): BD-333, BD-334, BD-335 → N10b-03; the M3 and M4 mission beads of WP-59 and WP-66 → N10b-05; BD-292, BD-347 → N10b-07; BD-240, BD-363 and the race-check bead → N10b-10; BD-285, BD-357 → N10b-13; BD-322 → N10b-15; BD-439 → N10b-16; BD-264 → N10b-17; BD-277 → N10b-18; BD-321 → N10b-11 (the verdict selects the default feature map; N10b-11 itself depends on BD-320) | lead | — |

## (e) Things found contradictory or hard to turn into work, and what was done

1. **Dependency cycle hazard between a specification bead and the bead that creates its crate.** The brief
   asks a SPEC bead to depend on the seed that creates the crate its law tests live in; mechanism beads are
   asked to depend on SPEC beads. For C-17, C-18 and C-19 the crate-creating seeds (BD-128 for `frob-mission`,
   BD-078 for `frob-sense`, possibly BD-126) are themselves the implementations, so both rules together make a
   cycle. Done: for SPEC-C-17, SPEC-C-18, SPEC-C-19 and their parts (N10b-04, -06, -08, -09) the law suites live
   in `frob-conformance` (which exists from BD-025 and may depend on every crate), the SPEC beads do **not**
   depend on BD-128, BD-078 or BD-126, and the prerequisite is recorded in `needs`. For the other
   constructions the SPEC bead depends on a crate-creating seed that implements a *different* construction:
   SPEC-C-16 → BD-081, BD-332; SPEC-C-20 → BD-078; SPEC-C-21 → BD-061; SPEC-C-22 → BD-048; SPEC-C-23 and
   SPEC-C-24 → BD-192; SPEC-C-25 → BD-124; SPEC-C-26 → BD-262; SPEC-C-27 → BD-270; N10b-12 → BD-170; N10b-14 →
   BD-058. If the drafter of one of those seeds added the SPEC key to its deps, the lead should drop that edge
   on the seed's side.
2. **Milestones where the first named gate is later than the first consumer.** SPEC-C-22 is M1 because BD-049
   (M1) must produce the counts 86,185, 500,500 and 27, although the first named gate (GATE-PHYLUM-01) is M3.
   SPEC-C-21 is M2 (relation MR-06 under GATE-GAUNT-03). N10b-09 is M2 (KILL-SCHED is decided at M2; the label
   gate is M4). SPEC-C-26 is M3 while BD-145 (M1) pre-registers thresholds that use its definitions: BD-145
   should cite the contract as related, not block on it.
3. **Thresholds for Kendall τ.** Appendix D.8 words 0.7 as the level at which the cheap rung "is used to
   discard"; §40 and the type sketch read 0.7 as the level for selecting and 0.5 for discarding. §40 governs;
   SPEC-C-26 states the §40 reading and records the difference.
4. **Episode count for the zero-failure bound.** C-27 prints n = ⌈ln 20/p*⌉ ("about 3·10⁴"), which is 29,958
   for 10⁻⁴; §26.9 and the worked card give the exact 29,956 from ⌈ln δ/ln(1 − p*)⌉. Both are carried in
   SPEC-C-27: the exact count as the known answer, the looser one as the sufficient count.
5. **Numbers the plan leaves open.** The grazing threshold of FROB-REF-263 has a normalization and no value:
   N10b-01 introduces ν_min with a fail-closed default of 1/16 (DESIGN) until the pilot N10b-03 fixes it. The
   scalar definition of the radius of gyration ℓ₀ is implicit: SPEC-C-21 fixes ℓ₀² = tr(Σ_C)/m₀ (DESIGN). The
   order "nearest" for a separating world is made precise in N10b-06 (DESIGN: lexicographic on edit count, then
   the largest normalized distance). Tolerances and controls listed in (c) are set as DESIGN.
6. **Pilots that already exist as seed beads of other units.** The pilot of C-26 and Pilot 1 of C-23 are the
   forty-design experiment of BD-145 and BD-310; the grid half of the C-19 pilot is re-run by BD-041. No
   duplicate bead was written: the protocols are carried in full in SPEC-C-26, SPEC-C-23 and N10b-10, and (d)
   asks that the executing beads carry them.
7. **Pilots that need mechanisms to exist.** Several §10 pilots are stated on the product (the first mission
   slice; the referee; replay; forks; the fidelity battery). Those pilot beads depend on the seed beads that
   provide the host (BD-082; BD-129, BD-130, BD-136; BD-203, BD-170; BD-198; BD-212; BD-205; BD-320) and gate
   the *later* work of the package, not the host. Each states a development-oracle alternative where one is
   allowed.
8. **Kill gates and open questions.** §10 defines none; every kill gate a construction names (KILL-GAIT,
   KILL-FAMILYCERT, KILL-AUGUR-REUSE, KILL-ALIAS, KILL-SCHED, KILL-QUOTIENT, KILL-FAMILY, KILL-INHERIT,
   KILL-GATREE, KILL-DR, KILL-RANK, KILL-LADDER, KILL-RARE) is stated with trigger, milestone and fallback in
   the Considerations or the pilot bead of its construction; its decision bead belongs to the defining chapter.
   No row of §41 or §42 names C-16 to C-27; the open questions touched (OPEN-LEMMA-6, OPEN-GAUNT-2 and -3,
   OPEN-CORTEX-1, OPEN-SPARK-2, the compute class of WP-79, the rare-failure compute question) are in the
   Considerations with their fail-closed defaults.
9. **No user-visible capability is defined in §10.** Verbs and their end-to-end scripts belong to the
   mechanism beads. Each bead here ships a script that drives `frob --json doctor`, `frob --json prove list`
   and the law suites; pilot scripts drive the verbs they evaluate; N10b-19 is the unit's end-to-end bead.
10. **`docs/math/` is not in the repository layout of §34.1** (which lists `docs/delivery/` and uses
    `docs/derivations/`). The brief names `docs/math/C-nn.md`; the beads use it, with derivation files under
    `docs/derivations/` as §38.6 requires.
11. **Parent choices that are not the obvious crate owner.** SPEC-C-20 is under WP-49 (the package that names
    GATE-SENS-08) although its milestone is M1; SPEC-C-25 is under WP-50 (tier correction) although the
    construction table names `frob-arena`; N10b-19 is under EPIC-M4.
