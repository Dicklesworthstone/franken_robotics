# Coverage of drafting unit U38b

Chapter §38 "Gauntlet: the verification program", §38.10 to §38.18; Appendix M's closing sections M.37
"Coverage and gaps" and M.38 "Values that could not be reconciled with their own formulas"; Appendix N's
closing section N.35 "Seeding the negative-control registry"; seed beads BD-025, BD-146, BD-223, BD-318,
BD-377, BD-411. 70 bead files: 6 seed, 3 registry (`N38-KA`, `N38-CX`, `N38-REF`), 61 new (`N38b-01` to
`N38b-61`). The other half of the chapter (§38.1 to §38.9, `STD-E2E`, `STD-TEST`) is unit U38a's; its beads are
depended on by key (`N38a-nn`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §38.10 | The Reality lane | N38b-01 (the four duties as checks: the line, the Parity and synthetic labels with the M3 schedule of published anchors, the rehearsal, the share of guessed card fields; the flag with no setter); N38b-02 (end-to-end script); N38b-26 (OPEN-GAUNT-1); BD-411 (the synthetic rehearsal rows) |
| §38.11 | Guarding against false progress | Typed close reasons: N38b-03. Negative controls and the registry record: BD-025 (crate v0, record schema, loader, the no-control check). Five rules of the registry: N38b-04 (control runner by kind, same-run rule, defect not boundary, suspect review, permanent controls). Seeding from the catalogue and from stated control cells: N38b-05. Live-dependency proofs: N38b-06. Synthetic is labelled: N38b-07. Behaviour, not plumbing: N38b-08. No self-certification: N38b-19. Measurement: N38b-09; per milestone BD-146, BD-223, BD-318, BD-377, BD-411 |
| §38.12 | Certifying the certifiers | N38b-10 (register, the broken-certifier rule, four carried self-tests); N38b-11 (certificate checker corpus; checker before an answer); N38b-12 (qualification lane; solvability oracle); N38b-13 (rank-agreement ledger; screening audits); N38b-14 (requirement monitors; fixed-n pre-analysis; rare-event bounds); N38b-15 (attribution; conformal bands); N38b-16 (claim product; determinism classifier; energy audit; the Gauntlet's own harness); N38b-17 (end-to-end measurement of GATE-GAUNT-04); BD-318 and BD-411 (planted registries and corpora) |
| §38.13 | Status accounting | N38b-18 (base gate record, claim kinds, statuses, the four refusals of `pass`, plan-time count); N38b-19 (recorder: receipt to row, same-run rule, session identities); N38b-20 (generated views: README table, release rubric section, dashboard feed, printed catalogue and control registry); N38b-21 (end-to-end measurement of GATE-GAUNT-10); N38b-25 (run schedule and recurring gates) |
| §38.14 | Invariants and proof obligations | N38b-22 (all ten in one suite with planted violations; the registry diff for INV-GAUNT-07). Mechanisms: INV-GAUNT-01 N38a-04; -02 BD-025, N38b-19; -03 N38a-05; -04 N38b-07, N38a-06; -05 N38b-19; -06 N38b-20; -07 N38b-22; PO-GAUNT-01 N38a-08; -02 N38a-14; -03 N38b-05 |
| §38.15 | Gates and kill gate | N38b-23 (the thirteen rows entered control-first); N38b-24 (KILL-DEVORACLE decision); measuring beads per gate in table (b) |
| §38.16 | Open questions | OPEN-GAUNT-1 N38b-26; -2 N38b-27; -3 N38b-28 to N38b-34; -4 N38b-35; -5 N38b-36 (default applied by N38b-25); -6 N38b-61 with N38b-50 to N38b-60; -7 N38b-37; -8 N38b-38 |
| §38.17 | Known answers adopted from corrected fixtures (added late) | N38-KA (rows KA-836 to KA-840); N38-CX (CX-385 to CX-389); N38b-40 (rows 1, 3, 5 at M1); N38b-43 (row 2 at M4); N38b-45 (row 4 with the continuation lane); N38b-46 (end-to-end measurement of GATE-FID-09) |
| §38.18 | Known answers adopted from late fixtures (added late) | N38-KA (rows KA-841 to KA-849); N38-CX (CX-390 to CX-398); N38b-39 (M0), N38b-40 (M1), N38b-41 (M2), N38b-42 (M3), N38b-43 (M4), N38b-44 (M6), N38b-45 (M8); N38b-46 (end-to-end measurement of GATE-GAUNT-11) |
| Appendix M.37 | Coverage and gaps | N38b-49 (per-chapter table as data, chapters without rows, burn-down); controls gap N38b-28 to N38b-34 (325 rows, every number listed by chapter, a proposed control per row) and the three boundary-behaviour rows in N38b-28; tolerances gap N38b-27 (133 rows listed); truths named and not written N38b-47 (six derivation files with closed forms); illustrative cards N38b-48 |
| Appendix M.38 | Values that could not be reconciled with their own formulas | One bead per value: N38b-50 (1; also the reconciliation register), N38b-51 (2), N38b-52 (3), N38b-53 (4), N38b-54 (5), N38b-55 (6), N38b-56 (7), N38b-57 (8), N38b-58 (9), N38b-59 (10), N38b-60 (11); closure N38b-61 |
| Appendix N.35 | Seeding the negative-control registry | N38b-05 (counts, kind table, gate resolution, the seventeen word-only entries, the five thesis fixtures, the pre-slip aliasing obligation, entries whose rule has no gate, PO-GAUNT-03); N38-CX (entries stated by this chapter) |

Seed beads:

| Seed | Milestone | Content | Depends on (beyond the seed's own list) |
|---|---|---|---|
| BD-025 | M0 | The crate, six tier targets, the control record and loader, the no-control registry check | STD-E2E, STD-TEST |
| BD-146 | M1 | Known answers against pinned oracles, catalogue seeds runnable, `smoke` and `fixtures`, GATE-GAUNT-01 for M1 | N38b-04, -05, -09, -19; N38a-04, -12, -13 |
| BD-223 | M2 | Standard battery wiring to its four readers, relations and chaos on devices, GATE-GAUNT-01 for M2 | N38a-19, -20, -24, -35; N38b-09, -19 |
| BD-318 | M3 | `hard` and `covering`, planted registries of the adversarial and debugging subsystems, hostile-input corpora, GATE-GAUNT-01 for M3 | N38a-21, -25, -26, -27, -36; N38b-09, -19 |
| BD-377 | M4 | Calibration sets, the frozen 30-amendment campaign, the seeded cascade population, GATE-GAUNT-01 for M4 | N38a-36; N38b-09, -13, -19 |
| BD-411 | M5 | Certificate mutation corpora, the synthetic rehearsal, the structure fixtures, GATE-GAUNT-01 for M5 | N38b-01, -09, -11, -19 |

## (b) Gates, kill gates and admission gates

| Gate | Defined in | Registered by | Measured or decided by |
|---|---|---|---|
| GATE-GAUNT-01 | §38.15 | N38b-23 | N38b-09 (all-controls run); per milestone BD-025, BD-146, BD-223, BD-318, BD-377, BD-411; mechanisms BD-025, N38b-04, N38b-05; recurring schedule N38b-25 |
| GATE-GAUNT-02 | §38.15 | N38b-23 | N38a-13 (U38a); obligations closed by N38b-27 to N38b-34, N38b-47, N38b-49, N38b-50 to N38b-61 |
| GATE-GAUNT-03 | §38.15 | N38b-23 | N38a-19 (U38a); count decision N38b-38; wiring BD-223 |
| GATE-GAUNT-04 | §38.15 | N38b-23 | N38b-17; M1 part N38b-11; rows N38b-10 to N38b-16 |
| GATE-GAUNT-05 | §38.15 | N38b-23 | N38a-04, N38a-05 (U38a); decision N38b-35; BD-146 |
| GATE-GAUNT-06 | §38.15 | N38b-23 | N38a-27 (U38a); wiring BD-223, BD-318 |
| GATE-GAUNT-07 | §38.15 | N38b-23 | N38a-35 (U38a); wiring BD-223 |
| GATE-GAUNT-08 | §38.15 | N38b-23 | N38a-09, N38a-10 (U38a); schedule N38b-25 |
| GATE-GAUNT-09 | §38.15 | N38b-23 | N38a-07, N38a-08 (U38a); certifier row in N38b-16 |
| GATE-GAUNT-10 | §38.15 | N38b-23 | N38b-21; mechanisms N38b-18, N38b-19, N38b-20 |
| GATE-GAUNT-11 | §38.18 | N38b-23 | N38b-46; rows N38b-39 to N38b-45 |
| GATE-FID-09 | §38.17 | N38b-23 | N38b-46; rows N38b-40, N38b-43, N38b-45 |
| KILL-DEVORACLE | §38.15 | N38b-23 | N38b-24 (decision at M1; count produced by N38a-06) |
| GATE-GROUND-01 (another chapter's; the Gauntlet's duty 1) | §30.22 | — | N38b-01, N38b-02 (conformance side); owner bead BD-139 |
| GATE-GROUND-10 (import lane) | §30.22 | — | N38b-26 (the decision that makes it eligible) |
| GATE-FID-01, GATE-ARENA-07 | §16, §24 | — | N38b-37 (scenario lists per class), BD-223 (binding) |

## (c) Counts

- **Known answers.** 14 registry rows entered (reserved KA-836 to KA-849: five corrected fixtures, nine late
  fixtures), each with derivation, tolerance, tier, site, control and due milestone (N38-KA). 325 rows without
  a stated control carried by number, by chapter, each with a proposed control (N38b-28: 15; N38b-29: 72;
  N38b-30: 25; N38b-31: 69; N38b-32: 70; N38b-33: 29; N38b-34: 45) plus 3 boundary-behaviour rows replaced
  (KA-444, KA-489, KA-496). 133 rows with tolerance to set carried by number, by chapter (N38b-27). 6 rows with
  truths named and not written given closed forms (KA-356, -357, -269, -336, -061, -282; N38b-47). 4 rows in
  the two illustrative-card pairs (KA-302, -403, -308, -185; N38b-48). The per-chapter table of all 835 rows
  (N38b-49).
- **Counterexamples.** 16 catalogue entries entered (reserved CX-385 to CX-400; N38-CX). The 384 existing
  entries carried as counts per chapter, with the 17 word-only entries listed by number and given proposed
  planted defects, the 13 shared-fixture entries counted, and the five thesis fixtures identified (CX-010,
  -011, -012, -013, -015; N38b-05). Existing entries cited with their cases: CX-001, -002, -014, -080, -257,
  -262, -282, -290, -381, -384.
- **Refusal codes.** The chapter defines no public `FROB-REF` code (Appendix O has none for §38; the Gauntlet
  has no verb). N38-REF carries the Gauntlet's harness reasons (40 typed reason strings in 24 table rows,
  with trigger, repairs and exit) and 24 existing public codes the chapter's rules rely on, each with
  meaning, place of use and exit code.
- **Certifiers.** 19 rows (4 carried, 15 added), each with threshold and a broken-certifier control.
- **Invariants and proof obligations.** 7 + 3, all in N38b-22.
- **Open questions.** 8 of 8 (5 decision beads, 2 experiment beads, 1 closure with 11 value beads).
- **Reconciled values.** 11 of 11, one bead each.

## (d) Needs from other slices

- §40.1 to §40.9: the register layer over the base gate record (kinds, parts per milestone, `blocked`, kill
  fields, validator), the rows, the exit query and exit protocol, the fallback ledger and kill-decision
  records, the owner-input calendar (drafted by the gate-register unit as N40-01, -02, -03, -04, -05, -10,
  -29, -33, -34, -35).
- §29.13, §29.14: the artifact graph and the per-bead log (BD-095).
- §30.2, §30.21: the Reality statuses and line grammar, the share of guessed card fields, the
  published-anchor comparison, the rehearsal dataset (the calibration unit: N30a-02, N30a-03, N30a-04,
  BD-311, BD-388).
- §24.9, §26.11: battery manifests `smoke`, `fixtures`, `hard`, `covering`.
- §24.13, §26.5, §26.9, §26.10, §27, §28.10, §28.12, §28.13: the lanes, oracles, estimators, audits and
  engines whose self-tests and planted registries are registered here.
- §12.4: the proof-kind registry and specimens; the mission mutation corpus (BD-136).
- Appendix M and N: every chapter's registry beads (`N<chapter>-KA`, `-CX`), whose owners confirm proposed
  controls, tolerances and closed forms.
- §39: the rival audit that freezes comparators (OPEN-GAUNT-4, OPEN-PERF-2, OPEN-PHYLUM-4).
- §36.14, §42.2: compute-envelope classes and their ratification (OPEN-SPARK-2).
- §22.11: the numeric instance of the pre-slip aliasing fixture.
- §18.16: a row for track scrub work (the "belt-law known answer" that §14.13 cites).
- §34.6: `check-generated` and `check-contracts` learning the generators and the release manifest.

## (e) Contradictions and things that could not be turned into work as written

1. **Appendix M has no section for §38**, and the fourteen rows of §38.17 and §38.18 have no KA numbers.
   Numbers KA-836 to KA-849 are reserved in table order (N38-KA); likewise CX-385 to CX-400 (N38-CX). The
   registry confirms them at first commit.
2. **§38.12 against §28.10.** The certifier table says "an unmeasured stratum is `discard-only`"; the ledger's
   own section says an unmeasured stratum is `uninformative` and the default fails closed. N38b-13 tests the
   stricter rule and records the difference.
3. **Status vocabulary.** §38.13 lists six statuses; §40.1 lists five with `blocked`. N38b-18 defines the base
   six and names `blocked` as the register layer's, matching the gate-register unit's union of seven.
4. **Count of kill gates.** §38.13 and §45.10 give 421 identifiers (278 + 111 + 32); the introduction of §40
   says 112 kill gates. N38b-18 asserts the counts through an expectation file that names the identifiers.
5. **Claim-identifier prefix.** `clm:` in the gate record, `c:` in the bead-log example; left to the identity
   module (N38b-18, Considerations).
6. **Inputs of the status table.** §38.13: generated from `gates.toml` "and from nothing else"; §29.14 and
   §45.11: from the logs and `gates.toml`. N38b-20: one content source, the logs as a cross-check.
7. **The dashboard's "view of gates (§32.14)".** §32.14 lists no gate pane. N38b-20 feeds gates through the
   read verb `capabilities` into the Evidence pane.
8. **Example control numbers.** NC-0412, NC-0398 and NC-0399 in the examples follow no stated numbering;
   they are reserved for the records they illustrate (BD-025, N38b-05).
9. **§38.11 says each catalogue entry attaches "to the gates of the rule it protects"**, but only 117 of 384
   rule cells name a gate; 267 name an invariant, decision, refusal code, construction or section. N38b-05
   resolves through a rule-to-gate map and files the remainder as findings.
10. **M.38 row 4 "stands as a remark", yet the body of §19.10.3 now prints 361 tiles and about 370 kB** while
    KA-445 prints 352 and about 360 kB. N38b-53 settles on whole tiles.
11. **M.38 row 8**: the stated control cannot fail its row. N38b-57 replaces it and adds the loader rule
    `cannot-fail`.
12. **§38.17 names the verbs `design.check`, `gait.atlas`, `task.fixture` and puts rows 1, 3 and 5 at M1**, but
    `gait.atlas` is an M4 verb. The strata row runs through the orbit-counting verb at M1 and again under the
    gait atlas at M4 (N38b-40).
13. **"Each row lands at the milestone of its owner section" (GATE-GAUNT-11)** names no milestones. Assigned
    as DESIGN: Cayley M0; unresolved outcomes M1; moment merge and writable view M2; calibration edge M3;
    residual bound M4; film front M6; certified root and tube erosion M8. GATE-FID-09 row 4 "with the
    continuation lane": M8.
14. **Session identity (INV-GAUNT-05)** cannot be authenticated inside a repository; N38b-19 states it as a
    discipline check with a planted test and the exit review as second line.
15. **Path names.** The brief says `registries/negative_controls.toml`; the plan says
    `frob-conformance/registry/controls.toml`. The plan's path is used; the brief's is a logical alias.
16. **Overlaps with other units, and the split adopted.** The gate-register unit drafted the gate record's
    extension, validator, exit query and an end-to-end script; U38b keeps the base record, the recorder's
    receipt and session mechanics, the views and GATE-GAUNT-10's own script. The calibration unit drafted
    Reality statuses, the share and the anchor comparison; U38b keeps the cross-cutting conformance checks. The
    thesis unit drafted controls for its fourteen rows and hygiene checks; U38b lists those rows with proposals
    and defers to that unit's controls where they differ. The sensing unit's calibration beads also cite
    GATE-GAUNT-11; N38b-42 supplies the registry-bound row and its control.
17. **Per-row control proposals and closed forms are DESIGN proposals**, not plan content: the plan leaves them
    to "each owning chapter". They are written so that the work is concrete; owners confirm or replace.
18. **Dependencies on `N38a-nn` keys** (the other half of the chapter) are used as instructed; all other
    non-seed prerequisites are in `needs`.
19. **Parent.** `WP-14` is an M1 package and is the only Gauntlet-mechanism package of `frob-conformance`; it is
    used as parent for beads of every milestone here (as U38a did), except N38b-35, whose parent is WP-34
    (incumbent ledger and parity baseline).
20. **No type sketch and no verb.** The appendix of type sketches has none for `frob-conformance`, and the
    verb registry has no Gauntlet verb (by decision ADR-GAUNT-01). Record formats and Rust sketches in these
    beads are DESIGN; the recorder is a Cargo example target and every check a test target.
21. **All 70 beads have type `task`.** No capability of these subsections is a verb a user or agent observes.
22. **N38b-61 closes OPEN-GAUNT-6 at M1** although two of its value beads concern mechanisms of later
    milestones (films at M6, region certificates at M3): the settlements are registry and document work and
    do not wait for those mechanisms.
