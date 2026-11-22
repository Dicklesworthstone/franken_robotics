# Coverage of drafting unit U28a

Chapter §28 Atelier, subsections §28.1 to §28.6 and the chapter preamble; 17 seed beads; the known-answer
registry of the chapter. 51 bead files: 17 seed beads, 33 new beads (`N28a-01` to `N28a-33`) and the registry
`N28-KA`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §28 preamble | Chapter title: the studio loop, analyses, comparison, lineage, cascade, decision cores | N28a-01 (the two crates, what the studio is) |
| §28.1 | What Atelier owns: the two crates; the ownership table; the rule for cores that govern another chapter's mechanism; the mechanism map | N28a-01 (crates, ownership table, mechanism map, the shared-mechanism rule); N28a-05 (`analyze.<name>`); the mechanisms themselves in the beads of the rows below and of the other two units |
| §28.2 | Decisions ADR-ATEL-01 to -11 | N28a-01 (all eleven as records with their enforcing checks; ADR-ATEL-10 enforced there); ADR-ATEL-01 N28a-02; -02 N28a-18, BD-262; -03, -04 N28a-33; -05 BD-347; -06, -09, -11 BD-349, N28a-27, N28a-28; -07 BD-351; -08 BD-352 |
| §28.3 | The loop: order as a rule; each stage can refuse the next; the invalidation set of an edit; populations climb the ladder; phases recorded; the nine-tenths rule | N28a-02 (order, refusal of the next stage, recorded override, INV-ATEL-01); N28a-03 (phase attribution and the nine-tenths rule); BD-113 (an edit returns its invalidation set, recorded on the lineage edge); N28a-33, BD-329, BD-352 (populations on the ladder and the cascade) |
| §28.4 | Analyses: the table A-01 to A-26; the four rules; frontier analyses and their pilots; shortcuts that are wrong | N28a-04 (contract: findings, repairs, rule 1 bound sides and authority, rule 2 enclosure and FROB-REF-551, rule 3 memo key, rule 4 screening label); N28a-05 (the full table and one verb per analysis); BD-101 (A-01); BD-100 (A-02, A-08); N28a-06 (A-08 stance battery); BD-106 (A-07); BD-107 (A-12); N28a-07 (A-20); N28a-08 (A-21); N28a-09 (A-22); N28a-10 (A-23); BD-108 (A-20 to A-23 in `check`; gate rows); BD-348 (A-24; gate completion); N28a-13 (A-25); N28a-14, N28a-15, N28a-16 (A-26 pilot, KILL-ATLAS, mechanism); BD-110 and N28a-12 (`design.check` stages 1 to 6 and its seeded-defect corpus); N28a-11 (the nine shortcuts as negative controls); BD-250 and N28a-22 (validation on the corpus, KILL-STATIC). Analyses implemented by other units' seeds: A-03 BD-102; A-04 BD-332; A-05, A-06 BD-105; A-09, A-14, A-16 BD-243; A-10, A-11 BD-103; A-13 BD-104; A-15 BD-401; A-17 BD-120, BD-239, BD-240; A-18 BD-233; A-19 BD-346 |
| §28.5 | Explain: what `explain.run` assembles; description against cause; explaining a comparison; gait by symmetry mode; the first physical consequence of an edit | BD-112 (another unit's seed: `explain.run`, `explain.design`, the description rule, the symmetry-mode line, the edit lead at M1); N28a-17 (`explain.compare`; the M3 completions: sensitivity-ranked edits, first-divergence lead, the intervention offer) |
| §28.6 | Comparing designs honestly: study types `anytime` and `fixed-n`; reachability pre-analysis; exact curtailment; six pairing rules; several metrics; several designs; what a comparison returns; what it does not control; a relation, not two scores | BD-262 (anytime comparison on devices, address order, cancellation, `eval.compare`); N28a-18 (six pairing rules, refusals 542, 543, 553, 555, 556, accounting identity; the relation form in Considerations); N28a-19 (fixed-n rule, reachability table, exact curtailment, PO-ATEL-02); N28a-20 (several metrics; several designs; PO-ATEL-04); N28a-21 (the comparison record, five dispositions, absolute verdicts, `provisional`, development result, INV-ATEL-11, `study.compare`); N28a-30 (end-to-end). The CPU study types, `study.plan` and `eval.preanalyze` are another unit's seed BD-124 |

Seed beads of this unit that draw on subsections owned by the other two units, each written completely:

| Seed | Draws on | Companion beads written here |
|---|---|---|
| BD-113 | §28.7 (lineage graph, diff, blame, migration) | N28a-31 (end-to-end) |
| BD-343 | §28.7 (bisect, merge) | N28a-31 |
| BD-310 | §28.9 (pilot and falsifier for layers 2 and 3), §28.10 (the calibration experiment) | — |
| BD-329 | §28.9 (layers 2 and 3), §28.10 (modes), §28.19.2 (DC-02, DC-03) | N28a-33 (rungs and layer 1), N28a-25 (KILL-LADDER) |
| BD-347 | §28.13 (grades 3 and 4, the lattice, the five rules, the pilot) | N28a-23 (pilot), N28a-24 (KILL-REUSE) |
| BD-349 | §28.19.1 to §28.19.4 (DC-02 to DC-08, DC-11) | N28a-27 (DC-04, DC-06), N28a-28 (DC-07, DC-08, DC-11) |
| BD-351 | §28.16 (experiment suggestion), §28.9 layer 3, DC-05 | N28a-26 (KILL-VOI) |
| BD-352 | §28.11, §28.12, §28.8 (one verb), §28.18 (`report.campaign`) | N28a-32 (end-to-end) |

End-to-end beads: N28a-29 (`design.check` and the analyze verbs), N28a-30 (comparison), N28a-31 (lineage),
N28a-32 (ladder, cascade, campaign report). Pilot beads: BD-310 (rank agreement; ladder layers 2 and 3),
N28a-14 (A-26), N28a-23 (grade-3 reuse).

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Carried here | Expected from another unit |
|---|---|---|
| GATE-ATEL-01 | BD-250 | — |
| GATE-ATEL-02 | BD-110 (M1 measurement), BD-100, BD-101, BD-106, BD-107, N28a-06, N28a-11, N28a-29 | Completion at M4 with A-04: seed BD-332 (U28b); rows of A-03, A-05, A-06, A-09 to A-11, A-13, A-14 by their seeds |
| GATE-ATEL-03 | — | Seed BD-374 (U28b); second arm BD-372 |
| GATE-ATEL-04 | BD-262 (comparison row), BD-343 (bisect row), N28a-30, N28a-31 | — |
| GATE-ATEL-05 | BD-108 (A-20 to A-23, M1), BD-348 (M4 completion), N28a-07 to N28a-10, N28a-13, N28a-16, N28a-11, N28a-29 | Rows of A-15 (BD-401), A-16 (BD-243), A-17 (BD-120, BD-239, BD-240), A-18 (BD-233), A-19 (BD-346) |
| GATE-ATEL-06 | N28a-18 (refusal rows), N28a-19 (enumeration rows), N28a-30 | Seed BD-124 (U33b) names the gate in its done-condition |
| GATE-ATEL-07 | BD-310 (pilot rows) | Seed BD-328 (M4, on this system's engine) |
| GATE-ATEL-08 | BD-329, BD-310 (pilot rows), N28a-32 | — |
| GATE-ATEL-09 | Applied by BD-352 | Seed BD-345 (U28b) |
| GATE-ATEL-10 | Applied by BD-352 | Seed BD-346 (U28b) |
| GATE-ATEL-11 | BD-347 (M4) | Dependency-grade rows: seeds BD-292, BD-135, BD-136 |
| GATE-ATEL-12 | — | Seeds BD-111, BD-220; U28b for §28.14 |
| GATE-ATEL-13 | BD-349, N28a-27, N28a-28 (DC-02 to DC-08, DC-11) | DC-01, DC-09, DC-10: seed BD-309 (U28c) |
| GATE-ATEL-14 | — | Seed BD-350 (U28c) |
| GATE-ATEL-15 | BD-351 | — |
| GATE-ATEL-16 | BD-352, N28a-32 | Archive-merge row also in seed BD-342 |
| GATE-ATEL-17, -18, -19 | — | U28c (§28.25, §28.26) |
| GATE-SKEL-04 (stages 1 to 6; defined in §14) | BD-110, N28a-12, N28a-29 | Stages 7 and 8: seed BD-244 |
| GATE-CORTEX-12 (pilot record; defined in §23) | BD-310 | Seed BD-328 |
| GATE-AUGUR-11 (defined in §25) | BD-351 | Pilot: seed BD-291 |
| KILL-STATIC | N28a-22 (decision), measured by BD-250 | — |
| KILL-ATLAS | N28a-15 (decision), N28a-14 (pilot) | — |
| KILL-LADDER | N28a-25 (decision), measured by BD-329, pilot BD-310 | — |
| KILL-REUSE | N28a-24 (decision), measured by BD-347, pilot N28a-23 | — |
| KILL-VOI | N28a-26 (decision), measured by BD-351, pilot BD-310 | — |
| KILL-RANK | — | Seed BD-328 (rungs), BD-265 (tier pairs); U28b for §28.10 |
| KILL-BUDGETCURVE | — | U28c (§28.25) |
| KILL-FDGRAD | — | Owner §17.13 |

## (c) Counts

- Known answers: 27 (KA-680 to KA-706), all in `N28-KA`; cited with values in the mechanism beads. Known answers
  of other chapters cited with values: KA-037, KA-038, KA-058, KA-080, KA-088, KA-113, KA-154, KA-201, KA-224,
  KA-228, KA-266, KA-268, KA-291, KA-610, KA-612, KA-625.
- Counterexamples: 17 of the chapter's 17 (CX-302 to CX-318) are cited as negative controls in mechanism beads;
  the registry bead `N28-CX` belongs to U28c. Others cited: CX-019, CX-131.
- Refusal codes of §28: 15 of 15 are carried by a mechanism bead with trigger and repairs (551 N28a-04; 542,
  543, 553, 555, 556 N28a-18; 550 BD-343; 545, 548 N28a-33; 546 BD-329; 870 BD-347; 871 BD-349; 872 BD-351; 557
  BD-352 and N28a-21; 549 N28a-27); the registry bead `N28-REF` belongs to U28c.
- Invariants: INV-ATEL-01 to -07 and -09 to -12 are carried here (INV-ATEL-08, memo hits, belongs with §28.14).
  Proof obligations: PO-ATEL-02, -03, -04, -06, -08, -10 carried here; PO-ATEL-09 is tested locally in BD-262
  and measured by BD-350; PO-ATEL-01, -05, -07 belong to the beads of DC-01, DC-10 and the memo graph.

## (d) Needs from other slices

- §28.20 / Appendix O (U28c, `N28-REF`): a refusal code for "the previous stage of the loop refused" (N28a-02
  re-issues the upstream code); codes for an A-21 deadline that no schedule serves and an A-22 unstable loop
  (BD-108, N28a-08, N28a-09 report High findings and do not refuse); a note that 553/552, 555/532 and
  542/533/561 are pairs for the same defect seen at different layers (N28a-18).
- §28.24 (U28c): decision beads for OPEN-ATEL-1 (blocks GATE-ATEL-13: BD-349, BD-351), OPEN-ATEL-2 (BD-310,
  N28a-33), OPEN-ATEL-3 (BD-262, N28a-20, BD-349, BD-351), OPEN-ATEL-4 (BD-347), OPEN-ATEL-5 (BD-352),
  OPEN-ATEL-6 (BD-310). The fail-closed defaults are stated in the beads named.
- §28.10 (U28b): the rank-agreement ledger table, the mode automaton, FROB-REF-537, per-bucket calibration
  (N28a-33, BD-329, BD-310).
- §28.14 (U28b; seeds BD-111, BD-220): the `Memo` trait and store consuming the derived analysis keys of
  N28a-04; shadow recomputation.
- §28.19.1 (U28c; seed BD-309): the core contract check, record schema, `frob.core-table/1`, `cores.show`
  (BD-349, N28a-27, N28a-28).
- §28.25, §28.26 (U28c): the controller-budget escalation rule at cascade stage 3 and the three reuse columns in
  `report.campaign` and `design.diff --reuse` (BD-352, BD-113).
- §13 / C-01 (Marrow unit): the Hervé subgroup lattice with lattice mobility and its pilot (BD-101).
- §10 C-07: the pilot of the maneuverability index (BD-106).
- §22.9.3 (seed BD-334): derivative validity labels (N28a-14, N28a-16).
- §24.4 / §25.3.2: declaration of which observation channel must inform which decision (N28a-08).
- §39.6: the phase-ledger tables (N28a-03, BD-352).
- §40: the gate register row format and ADR location for kill decisions (all gate and kill beads).
- §42 OPEN-PROG-4: owner approval of compute for the rank-agreement pilot (BD-310).
- §38: the conformance crate's ownership of the frozen 30-amendment campaign and the seeded cascade population
  (seed BD-377 depends on BD-352, so both fixtures are first defined with BD-292 and N28a-32).

## (e) Contradictions and things hard to turn into work, with what was done

1. **A-20 to A-23 have no preflight stage.** §28.4 marks A-21, A-22 and A-23 as in `check` (A-20 when bound);
   the stage table of §14.11 assigns stages only to A-01 to A-16. BD-108 places them in stage 6 as a DESIGN
   default and lists the need.
2. **No refusal code for A-21 and A-22 failures, nor for "previous stage refused".** The findings are High and
   non-refusing (BD-108, N28a-08, N28a-09); N28a-02 re-issues the upstream stage's code. Raised under (d).
3. **Three refusal codes for one defect.** A paired statistic under an uncontrolled determinism class is
   FROB-REF-542 (§28.20), 533 (§24.14) and 561 (§35.11); mixing study types is 553 and 552; an unreachable
   verdict is 555 and 532. N28a-18 issues the §28 codes from `eval.compare` and records the overlap.
4. **GATE-ATEL-05 at M4 includes A-15, whose seed (BD-401) and gate are at M5.** BD-348 closes the gate for the
   analyses that exist and records the A-15 row `not-yet-due (M5)`.
5. **GATE-ATEL-13 at M4 includes DC-07, whose mechanism (seed BD-393) is at M5.** N28a-28 compiles and
   calibrates the table on synthetic residuals at M4; the shadow row is `not-yet-due (M5)`.
6. **Seed done-conditions that say "KILL-… decided".** The measurement stays in the seed (BD-250, BD-329,
   BD-347, BD-348, BD-351) and the decision is a separate bead that depends on it (N28a-22, N28a-25, N28a-24,
   N28a-15, N28a-26), following the charter's rule that each kill gate is a decision bead. For KILL-ATLAS the
   decision precedes the mechanism (N28a-14 → N28a-15 → N28a-16 → BD-348).
7. **Seed BD-343 says "bisect on a non-monotone predicate is refused".** §28.7 does not refuse such a bisect: it
   returns an `a-change` and refuses only a `first-change` answer without a prefix scan or a declared monotone
   predicate (FROB-REF-550). BD-343 follows §28.7 and records the reading.
8. **GATE-ATEL-01's "at a 95% paired lower bound".** Applied to the difference from the heuristic; the 0.8
   threshold is applied to the point estimate with its lower bound recorded beside it (BD-250), so the stricter
   reading can be applied from the same row.
9. **Seed BD-124 (another unit) carries `eval.preanalyze`, `eval.compare` on the CPU and GATE-ATEL-06**, while
   §28.6 is this unit's. N28a-18 and N28a-19 carry the pairing rules, the fixed-n rule, curtailment and
   PO-ATEL-02 and depend on BD-124; both name the gate. The lead may fold overlapping parts.
10. **Seed BD-349 covers eight cores whose mechanisms sit in five crates.** Split into BD-349 (the compiler,
    DC-02, DC-03, DC-05, the gate), N28a-27 (DC-04, DC-06) and N28a-28 (DC-07, DC-08, DC-11); the runtime of
    DC-02 and DC-03 is BD-329 and of DC-05 is BD-351.
11. **The frozen 30-amendment campaign and the seeded cascade population** are assigned to seed BD-377, which
    depends on BD-352; BD-347 and BD-352 need them earlier. They are first frozen with BD-292 and built by
    N28a-32, and BD-377 takes them over.
12. **`design.bisect`'s lineage merge has no verb in the registry.** BD-343 implements merge as a library
    operation tested through the conformance harness; no verb was invented.
13. **Verb names for analyses.** The registry gives `analyze.<name>` and fixes only `analyze.symmetry`. N28a-05
    proposes the other 25 names as DESIGN defaults for the registry freeze.
14. **The DC-05 worked example** reproduces (0.0013 and 0.0055) only under the reading that fast-tier pairs
    shrink the sampling component alone while referee pairs observe the difference itself; BD-349 and BD-351
    state that reading explicitly.
15. **Parent packages.** No work package of §44 holds `frob-opt` at M1 or `frob-analyze` at M4: BD-113 is under
    WP-07, N28a-18 and N28a-19 under WP-41, and BD-348, N28a-13 to N28a-16 under `EPIC-M4`.
