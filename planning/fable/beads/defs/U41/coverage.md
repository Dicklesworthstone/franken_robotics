# Coverage of drafting unit U41: §41 risks, §42 open questions, §45.10 and §45.11, §46 the first session, §47 definition of done

Files in this directory: 66 bead files and this file. 13 are the seed beads the brief assigns (BD-056, BD-058,
BD-087, BD-115, BD-153, BD-157, BD-160, BD-166, BD-180, BD-200, BD-231, BD-341, BD-344); 53 are new. The unit was
drafted in two passes. The first wrote 57 files and was interrupted before this file existed. The second pass
checked every one of those files (front matter, the ten sections, a non-empty References section, every key
named in a body), found all 57 complete, and added nine beads for what was still missing: the planted instances
of the 55 older risk rows (N41-32 to N41-37) and the three registries the charter requires per unit (N41-KA,
N41-CX, N41-REF). It also made small cross-reference edits to existing files, listed under (e).

Inventory of the new beads by subject:

| Subject | Beads |
|---|---|
| Risk register: schema, rules, review at every exit | N41-01 |
| Risk register rows (59, in four parts of at most 15) | N41-RISK1 (15), N41-RISK2 (15), N41-RISK3 (15), N41-RISK4 (14) |
| Pre-mortem tables (27 stories, 6 outcome reasons) | N41-PM |
| Planted instances of the four late rows (GATE-WS-07) | N41-02 (M0), N41-31 (M2 and M3) |
| Planted instances of the 55 older rows, by due milestone | N41-32 (layer, M0), N41-33 (M1), N41-34 (M2), N41-35 (M3), N41-36 (M4), N41-37 (M5 and the full sweep) |
| Open-question register: schema, the decision rule, defaults as ADRs | N41-03 |
| Open-question rows (161, in eight parts) | N41-OPEN1 to N41-OPEN8 |
| Owner-decision packets | N41-05 to N41-15 |
| End-to-end script for the registers at an exit | N41-04 |
| Tracker hygiene and bead lifecycle | N41-16, N41-17 |
| First session, steps 1 to 8 | N41-18 to N41-25 |
| Definition of done, release rules, rubric, evidence kinds, end-to-end release check | N41-26, N41-27, N41-28, N41-29, N41-30 |
| Registries of the unit | N41-KA (40 rows), N41-CX (51 rows), N41-REF |

## (a) Subsections and the beads that carry them

| Subsection | Title | Carried by |
|---|---|---|
| §41 (opening) | Three parts; labels: thresholds are TARGETs repeated from the gate named, the gate register governs, nothing in the chapter is a result | N41-01 (Background), N41-RISK1 (conventions for every part), N41-PM |
| §41.1 | It is late 2029 and FrankenRobotics failed. Why? (27 stories; the three clusters) | N41-PM; each story's rows of the register are in N41-RISK1 to N41-RISK4 |
| §41.2 | Every gate passed and the claim still failed (6 reasons, ranked) | N41-PM (the outcome reasons and their reading at every exit); N41-04 (reason 1 walked at the M0 exit); N41-CX row X41-48 |
| §41.3 | Risk register (RISK-01 to RISK-55; severity rule; owners are roles) | N41-RISK1, N41-RISK2, N41-RISK3, N41-RISK4 (the rows, complete); N41-01 (schema and loader) |
| §41.4 | How the register is used (review, escalation, growth, no hope) | N41-01 (the four rules as loader rules and the review step); N41-04 (end to end at three exits); N41-37 (the sweep at every exit from M5) |
| §41.5 | Four risks that follow from late foundation facts (late subsection; GATE-WS-07; the counterexample) | N41-RISK1 (RISK-56, RISK-57), N41-RISK3 (RISK-58), N41-RISK2 (RISK-59); N41-02 (the two build plants, M0); N41-31 (the sampler plant, M2, and the rare-tail arm, M3); N41-32 to N41-37 (the clause "no row of the register lacks a planted instance" for the other 55 rows); N41-CX row X41-01 (the counterexample) |
| §42 (opening) | What an open question is; labels | N41-03 |
| §42.1 | The decision rule (five clauses; 147 and 161) | N41-03 (clauses as loader rules, defaults as ADRs, counts); N41-04 (silence blocks an exit) |
| §42.2 | Questions for the owner (22 rows, 32 questions: default, cost of the default, needed by) | N41-OPEN1 to N41-OPEN8 (each owner row with its default and what the default costs); N41-05 to N41-15 (this unit's packets); the chapter units' owner-decision beads (map below) |
| §42.3 | The register by subsystem (147 rows) and the 14 adopted rows | N41-OPEN1 (Lemma, Marrow, Skeleton, Sinew), N41-OPEN2 (Kinesis, Phylum, Traction), N41-OPEN3 (Habitat, Sensorium, Spark), N41-OPEN4 (Reflex, gaits, shields, Cortex, Arena), N41-OPEN5 (Augur, Nemesis, Autopsy), N41-OPEN6 (Atelier, Chronicle, Ground, Foundry, Tether), N41-OPEN7 (suite, workspace, numerics, performance, safety), N41-OPEN8 (program; adopted from the Gauntlet and the ten-fold test) |
| §42.4 | What the defaults add up to (the all-defaults product; the six that matter most; four closed; four confirmed) | N41-03 (`open defaults-product`, the six in due order, tombstones, confirmed rows); N41-14 (the recorded decision OPEN-SUITE-2 and OPEN-TETHER-1); N41-04 |
| §45.10 | Graph facts (size, no cycles, exit chain, 44 exit-only beads, longest chains, widest beads, gate coverage; the 20 rows of gates that close in parts) | N41-16 (every fact as a known answer of the self-test; the parts table; the checks) |
| §45.11 | Conversion rules (inheritance, closing, kill gates, work packages, pilots and compute, splitting and adding, exit reviews) | N41-17 (ten rules as checks); N41-16 (cycles after conversion; discovered beads) |
| §46 | The first implementation session (working rules; steps 1 to 8; what each observation decides; how the session ends) | N41-18 (step 1), N41-19 (step 2), N41-20 (step 3, with fastmcp_rust built on T), N41-21 (step 4), N41-22 (step 5), N41-23 (step 6), N41-24 (step 7), N41-25 (step 8, the decision table, the session summary, silent retries) |
| §47 (opening) | Labels: clauses are DESIGN statements of acceptance; thresholds live in the gates | N41-26 |
| §47.1 | Definition of done (16 clauses; three things not required) | N41-26 |
| §47.2 | Rules for any release (nine) | N41-27 (the nine rules; the eight conditions of "implemented" re-evaluated per capability) |
| §47.3 | The rubric by milestone (nine rows; cumulative; withdrawn stays visible) | N41-28 |
| §47.4 | The order in which evidence is procured, and complete dispositions (late subsection; GATE-WS-08) | N41-29 (the four kinds, the per-stage device check, dispositions, the gate); N41-30 (end to end) |

The 13 seed beads belong to chapters other units own; each was drafted from its own chapter's text:
BD-056 and BD-058 (§14), BD-087 (§16.7), BD-115 (§22.5), BD-153, BD-157, BD-160, BD-166 (§16.6 with §18, §19,
§21, §35), BD-180 (§35.9), BD-200 (§23.15), BD-231 (§19.10), BD-341 and BD-344 (§28).

**Risk rows: where each is entered and which bead builds its planted instance.** "Due" is the earliest exit at
which the row's mechanical detector exists; five rows (RISK-11, -12, -25, -37, -47) are due later than they are
first readable by the reviewer, and ten rows have a further observation due later.

| Row | Severity | Entered in | First readable | Plant due, bead | Further observation |
|---|---|---|---|---|---|
| RISK-01 | High | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-02 | High | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-03 | High | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-04 | Medium | N41-RISK3 | M0 | M0 N41-32 | M2 N41-34 |
| RISK-05 | High | N41-RISK3 | M2 | M2 N41-34 | M3 N41-35 |
| RISK-06 | Medium | N41-RISK3 | M2 | M2 N41-34 | — |
| RISK-07 | Medium | N41-RISK3 | M0 | M0 N41-32 | — |
| RISK-08 | Medium | N41-RISK3 | M0 | M0 N41-32 | M2 N41-34 |
| RISK-09 | Medium | N41-RISK2 | M2 | M2 N41-34 | — |
| RISK-10 | High | N41-RISK2 | M0 | M0 N41-32 | M2 N41-34 |
| RISK-11 | Medium | N41-RISK4 | M2 | M3 N41-35 | — |
| RISK-12 | High | N41-RISK4 | M2 | M3 N41-35 | — |
| RISK-13 | Medium | N41-RISK3 | M1 | M1 N41-33 | — |
| RISK-14 | Medium | N41-RISK4 | M3 | M3 N41-35 | — |
| RISK-15 | Medium | N41-RISK4 | M3 | M3 N41-35 | — |
| RISK-16 | Low | N41-RISK1 | M5 | M5 N41-37 | — |
| RISK-17 | Medium | N41-RISK4 | M1 | M1 N41-33 | M3 N41-35 |
| RISK-18 | Medium | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-19 | Low | N41-RISK1 | M3 | M3 N41-35 | — |
| RISK-20 | Medium | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-21 | High | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-22 | High | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-23 | Medium | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-24 | High | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-25 | High | N41-RISK2 | M0 | M1 N41-33 | — |
| RISK-26 | High | N41-RISK2 | M3 | M3 N41-35 | — |
| RISK-27 | High | N41-RISK2 | M3 | M3 N41-35 | — |
| RISK-28 | High | N41-RISK2 | M4 | M4 N41-36 | — |
| RISK-29 | Medium | N41-RISK2 | M1 | M1 N41-33 | M2 N41-34 |
| RISK-30 | High | N41-RISK2 | M2 | M2 N41-34 | — |
| RISK-31 | High | N41-RISK3 | M1 | M1 N41-33 | M2 N41-34 |
| RISK-32 | High | N41-RISK3 | M1 | M1 N41-33 | M3 N41-35 |
| RISK-33 | High | N41-RISK2 | M1 | M1 N41-33 | — |
| RISK-34 | High | N41-RISK4 | M3 | M3 N41-35 | — |
| RISK-35 | High | N41-RISK4 | M1 | M1 N41-33 | — |
| RISK-36 | High | N41-RISK4 | M0 | M0 N41-32 | — |
| RISK-37 | High | N41-RISK4 | M3 | M4 N41-36 | — |
| RISK-38 | High | N41-RISK4 | M1 | M1 N41-33 | — |
| RISK-39 | Medium | N41-RISK4 | M2 | M2 N41-34 | — |
| RISK-40 | Medium | N41-RISK3 | M2 | M2 N41-34 | — |
| RISK-41 | Medium | N41-RISK3 | M3 | M3 N41-35 | — |
| RISK-42 | Medium | N41-RISK3 | M5 | M5 N41-37 | — |
| RISK-43 | Medium | N41-RISK2 | M1 | M1 N41-33 | — |
| RISK-44 | High | N41-RISK4 | M3 | M3 N41-35 | — |
| RISK-45 | Medium | N41-RISK3 | M2 | M2 N41-34 | — |
| RISK-46 | Medium | N41-RISK2 | M2 | M2 N41-34 | — |
| RISK-47 | Medium | N41-RISK3 | M3 | M4 N41-36 | — |
| RISK-48 | Medium | N41-RISK3 | M0 | M0 N41-32 | — |
| RISK-49 | Medium | N41-RISK4 | M3 | M3 N41-35 | M4 N41-36 |
| RISK-50 | Medium | N41-RISK4 | M3 | M3 N41-35 | — |
| RISK-51 | High | N41-RISK2 | M3 | M3 N41-35 | — |
| RISK-52 | Medium | N41-RISK2 | M3 | M3 N41-35 | — |
| RISK-53 | High | N41-RISK2 | M0 | M0 N41-32 | — |
| RISK-54 | Medium | N41-RISK1 | M0 | M0 N41-32 | M1 N41-33 |
| RISK-55 | Medium | N41-RISK1 | M0 | M0 N41-32 | — |
| RISK-56 | High | N41-RISK1 | M0 | M0 N41-02 | — |
| RISK-57 | Medium | N41-RISK1 | M0 | M0 N41-02 | — |
| RISK-58 | High | N41-RISK3 | M2 | M2 N41-31 | — |
| RISK-59 | High | N41-RISK2 | M3 | M3 N41-31 | — |

The mitigation of every row is carried by beads of other units, named with their keys in the "Beads that carry
the mitigation" column of the row. The only pieces of risk work that no other bead owned are the planted
instances, which is why they have beads here.

**The owner's questions (§42.2): the bead labelled `owner-decision` that decides each.** All 32 have one, and so
do the four adopted questions that the owner decides (last row); none needed a `needs` entry in place of a key.

| Question | Needed by | Decided in (this unit) | Decided in (chapter unit) |
|---|---|---|---|
| OPEN-SPARK-1 | M0; blocks the M2 exit | — | N21c-34 |
| OPEN-SPARK-2 | M0 | — | N21c-35 |
| OPEN-PROG-4 | M1 | N41-08 | — |
| OPEN-PHYLUM-3 | M2 | — | N17b-07 |
| OPEN-CORTEX-1 | M3 | — | N23a-22 (also N23b-24) |
| OPEN-PROG-2 (with OPEN-TENFOLD-1, -2) | M2 | N41-06 | N39b-26, N39b-27 for the two aliases |
| OPEN-PROG-3 | M3 | N41-07 | — |
| OPEN-AUGUR-4 | M3 | — | N25d-45 |
| OPEN-GAUNT-1 | M5 | N41-10 | N38b-26 |
| OPEN-GROUND-2 | sourcing may start at M0 | N41-10 | N30b-26 |
| OPEN-PROG-1 | first push; again before the M7 pin freeze | N41-05 | — |
| OPEN-PROG-5 | M5 | N41-09 | — |
| OPEN-ARENA-3 | M2 | — | N24c-13 |
| OPEN-SHIELD-2 | M3 | — | N22c-56 (also N22b-27) |
| OPEN-SPARK-5 | M2 | — | N21c-38 (also N21b-09) |
| OPEN-SUITE-2 (with OPEN-TETHER-1), decided | confirmed at M0 | N41-14 | — |
| OPEN-SUITE-3, OPEN-SUITE-5 | M0 | — | N33c-33, N33c-34 |
| OPEN-SINEW-1, OPEN-SENS-4, OPEN-TRAC-3 | M4 | — | N15b-25; N20b-26 (also N20a-32); N18b-36 |
| OPEN-HAB-1, OPEN-HAB-4 | M3 for norms; M5 for prices | — | N19c-30; N19b-43 |
| OPEN-SKEL-2, OPEN-FOUNDRY-2 | M5 | N41-13 | N31b-25 for OPEN-FOUNDRY-2 |
| OPEN-NEMESIS-3, OPEN-AUGUR-10 | M3; M6 | — | N26b-21; N25d-49 |
| OPEN-SAFE-2, OPEN-SAFE-3, OPEN-SAFE-4, OPEN-CHRON-4 | M2 to M4 | N41-11 | N37-20, N37-09, N37-30, N29b-12 |
| OPEN-TETHER-5 | M4 | N41-12 | N32b-36 (also N32a-34) |
| OPEN-TENFOLD-3, -4, -6, -8 (adopted; owner) | M2 | N41-15 | N39b-28, N39b-29, N39b-31, N39b-33 |

The map covers 36 questions (the 32 of §42.2 and the four adopted ones the owner decides). The other 125 are
decided by measurements or by subsystem leads; the bead that decides each is named in the last column of its row
in N41-OPEN1 to N41-OPEN8. Eight block nothing and are carried in the Considerations of the bead named there
(OPEN-MARROW-2, OPEN-SKEL-4, OPEN-AUGUR-11, OPEN-AUTOPSY-3, OPEN-AUTOPSY-4, OPEN-GROUND-4, OPEN-PERF-3,
OPEN-TENFOLD-9); OPEN-NUM-1, which also blocks nothing, is carried by the IEEE probe bead BD-017.

## (b) Gates, kill gates and admission gates

The chapters of this unit define two gates and no kill gate or admission gate.

| Gate | Defined in | Measured or decided by |
|---|---|---|
| GATE-WS-07, the two build rows (M0) | §41.5 | N41-02 (100 variants each; the all-feature-groups resolution; positive controls) |
| GATE-WS-07, the sampler row (M2) and the suite row (M3) | §41.5 | N41-31 |
| GATE-WS-07, "no row of the register lacks a planted instance" | §41.5 | N41-01 (loader rule 5: every row has a plant record); N41-02 (the structural half recorded at M0); N41-32 (binding, staging, 18 rows), N41-33 (10), N41-34 (8), N41-35 (14), N41-36 (3), N41-37 (2, and the sweep that records the clause complete: 59 of 59, 69 firings) |
| GATE-WS-08 (M1, and at every release) | §47.4 | N41-29 (the four planted cells; the corrected manifest); N41-30 (end to end, re-run at each release) |

Gates defined elsewhere that beads of this unit measure, support or decide:

| Gate | Bead here | Part |
|---|---|---|
| GATE-GAUNT-01 | N41-16, N41-17, N41-32, N41-37, N41-CX, N41-REF | Supported: every gate has a bead that can record it and a control that can fail |
| GATE-GAUNT-02 | N41-KA | Known-answer rows pinned by tests |
| GATE-GAUNT-08 | N41-24 | The first exact known answers reproduced with a control caught |
| GATE-SPARK-00, GATE-SPARK-01 | N41-21, N41-22 | One kernel bit for bit on three backends; the square-root half of the probe |
| GATE-SPARK-21 | N41-23 | The first row of the incumbent phase ledger |
| INT-FSIM-01 | N41-19 | The first admission record |
| INT-ASUP-01, INT-TOOLCHAIN-01, INT-FMCP-01 | N41-20, N41-14 | Package count, the first matrix cells, fastmcp_rust on T; the recorded decision confirmed |
| INT-FT-01, INT-CUDA-01 | N41-21, N41-22 | The first device runs |
| KILL-MCP | N41-14 | Its fallbacks, if the admission cannot pass |
| KILL-TENFOLD-ADJ | N41-06 | Read with the adjudicator decision |
| GATE-KIN-01 | BD-087 | Free-body momentum known answer |
| GATE-ATEL-13, GATE-ATEL-16 | BD-341, BD-344 | Racing; registered predictions |

Kill gates named by risk rows as the decision that a fired signal leads to (for example KILL-FSIM, KILL-PARITY,
KILL-KIR, KILL-SHARD, KILL-PHYLUM, KILL-FAMILY, KILL-AUGUR-TIE) are owned by their chapters; the planted
instances here offer synthetic records to their trigger evaluators and never write a decision.

## (c) Counts

| Kind | Count | Where |
|---|---|---|
| Risk rows | 59 (55 printed, 4 late) | N41-RISK1 to N41-RISK4 |
| Pre-mortem stories and outcome reasons | 27 and 6 | N41-PM |
| Planted instances of risk rows | 59 primary and 10 further observations | N41-02, N41-31, N41-32 to N41-37 |
| Open questions | 161 (147 + 14), with 2 aliases | N41-OPEN1 to N41-OPEN8 |
| Owner questions with a decision bead | 32 of 32 (15 with a packet in this unit, 17 in chapter units only) | The map above |
| Known answers of Appendix M whose section is one of this unit's chapters | 0 (the suite has no section for §41, §42, §45, §46 or §47) | — |
| Known-answer rows stated by these chapters and carried | 40 (K41-01 to K41-40), including GATE-WS-07, GATE-WS-08 and the first-session counts | N41-KA |
| Counterexamples of Appendix N whose section is one of this unit's chapters | 0 | — |
| Counterexample and planted-defect rows carried | 51 (X41-01 to X41-51); two are counterexamples the plan states in words (§41.5, §47.4) | N41-CX |
| Refusal codes of Appendix O defined by this unit's chapters | 0 | — |
| Harness reasons allocated for this unit's tools | 32 table rows naming 62 reasons (nine of them the release rules) | N41-REF |
| Existing public codes this unit's rules, plants and defaults rely on | 35 | N41-REF |
| Refusals stated by these chapters with no code | 1 (§47.4, "release cell lacks evidence of the kind it declares"), carried as a harness reason | N41-29, N41-REF |
| Known answers, counterexamples and refusal codes cited by the 13 seed beads | Stated inline in each seed bead with their KA and CX numbers; their registries belong to the chapter units | The seed beads |

## (d) Needs from other slices

Every `needs` entry of the unit names the key of the bead that supplies it, with four exceptions that are
requests rather than prerequisites: KA numbers for the rows of N41-KA and CX or NC numbers for the rows of N41-CX
(from the unit that owns the appendices' numbering), and public codes for the three provisional refusal names
used by BD-200 (`randomize-no-interval`) and BD-344 (`prediction-after-outcome`,
`prediction-changed-after-freeze`), from N23-REF and N28-REF.

Keys of other units' new beads that this unit's beads need, by owning unit (seed keys are in `deps`):

| Unit | Keys needed (and by which bead here) |
|---|---|
| U00 | N00-01 (N41-27, N41-28, N41-35); N00-02 (N41-26, N41-27, N41-30); N00-03 (N41-32); N00-07 (N41-09, N41-17, N41-33); N00-09, N00-17 (N41-33); N00-15 (N41-27, N41-28, N41-35); N00-18 (N41-23, N41-29); N00-29 (N41-30) |
| U10a | N10a-01 (N41-26) |
| U12 | N12-23 (N41-37) |
| U13, U14a | N13-02, N13-12, N14a-03 to N14a-06, N14a-11, N14a-17 (seed beads BD-056, BD-058, BD-087, BD-157, BD-341); N14a-10 (N41-13) |
| U15b | N15b-06 (BD-200, N41-13, N41-33); N15b-07 (N41-35) |
| U16a, U16b | N16a-11, N16a-13 (BD-157, BD-153); N16b-03, N16b-15 (N41-34) |
| U17b | N17b-20 (N41-36) |
| U18a, U18b | N18a-10, N18a-12, N18a-16, N18b-43, N18b-44 (BD-153, BD-166) |
| U19a, U19b, U19c | N19a-14, N19a-16, N19a-17, N19b-03 (BD-166, BD-231); N19a-43, N19c-15 (N41-35) |
| U21a, U21b, U21c | N21a-17, N21a-22 (BD-180); N21b-03 (N41-22); N21b-37 (BD-157, BD-166); N21c-10, N21c-11 (N41-12); N21c-35 (BD-341, N41-08, N41-12, N41-15, N41-17, N41-32) |
| U22a | N22a-17 (N41-35) |
| U23a, U23b | N23a-22, N23a-26, N23a-30 (N41-36); N23b-03 (BD-200); N23b-13, N23b-14, N23b-27 (N41-07); N23b-29 (BD-180); N23b-30 (BD-180, N41-31); N23-REF (BD-200, N41-REF) |
| U24a, U24b | N24a-17 (BD-344); N24a-31 (N41-35); N24b-17 (N41-11) |
| U26b, U27a | N26b-07 (N41-36); N27a-14 (N41-35) |
| U28b, U28c | N28b-23 (N41-08); N28c-28 (BD-341); N28-REF (BD-344, N41-REF) |
| U29a, U30b, U32a | N29a-23, N32a-09 (BD-341); N30b-38 to N30b-40 (N41-10); N32a-35 (N41-12) |
| U33a, U33b, U33c | N33a-04 (N41-18, N41-32); N33a-05 (N41-32); N33a-07 (N41-18, N41-19, N41-25, N41-26); N33a-10 (N41-19, N41-32); N33a-11 (N41-02); N33a-12 (BD-160, BD-180); N33a-14 (N41-33); N33a-28 (N41-19); N33b-04 (N41-32); N33c-01, N33c-26 (N41-20); N33c-03, N33c-30 (N41-32); N33c-08 (N41-14); N33c-37 (N41-31); N33c-38 (N41-02); N33c-39 (N41-21) |
| U34 | N34-03 (N41-02, N41-32); N34-04, N34-21 (N41-02); N34-08 (N41-29, N41-32); N34-09 (N41-27, N41-29); N34-12 (N41-27, N41-30); N34-13, N34-25 (N41-32) |
| U36 | N36-06, N36-10, N36-27 (N41-34) |
| U38a, U38b | N38a-04 (BD-115, N41-06, N41-23); N38a-09 (N41-09, N41-17, N41-33); N38a-34 (N41-06); N38a-36 (N41-35); N38b-03, N38-REF (N41-REF); N38b-04 (N41-32 to N41-37, N41-CX) |
| U40 | N40-01, N40-03, N40-04 (N41-01, N41-04, N41-17, N41-26, N41-28, N41-34); N40-02, N40-10 (N41-16); N40-05 (N41-01, N41-03, N41-04, N41-16, N41-25, N41-27, N41-30, N41-33, N41-37, N41-PM); N40-06 (N41-28); N40-07 (N41-32); N40-09, N40-18 (N41-33); N40-14 (N41-06, N41-15); N40-15, N40-22 (N41-35); N40-17 (N41-05); N40-19 (N41-15, N41-31, N41-35); N40-30, N40-37 (N41-17); N40-32 (N41-03); N40-35 (N41-03, N41-04, N41-32) |
| U44 | N44-01 (BD-341, N41-08, N41-09, N41-15, N41-17, N41-32); N44-02 (N41-34); N44-04 (N41-16) |
| UF | NF-02 (N41-14, N41-20); NF-05 (N41-14); NF-06 (N41-32) |

The exit runner of N40-05 is the largest single need: ten beads here add a step to it (the risk review, the
plant check and sweep, the pre-mortem readings, the open-question listing, the tracker check, the done report,
the release check). The lead may want those steps listed in N40-05's own acceptance criteria.

## (e) Contradictions and things that could not be turned into work as written, with what was done

1. **The risk series has 59 rows, not 55.** §41.3 prints RISK-01 to RISK-55 and §41.5 adds four rows with no
   numbers ("identifiers are assigned with the rest of the series"). They are assigned RISK-56 (a dependency
   feature changes canonical bytes), RISK-57 (an exact pin blocks an oracle), RISK-58 (a sampler with a wrong
   joint law) and RISK-59 (a rare slow mission absent from the sealed sample). Severity tally 28, 29, 2.
2. **The open-question register holds 161, not about 141.** §42.1: 141 raised by the subsystem chapters plus 6 at
   program level is 147, plus 14 adopted from §38 and §39; OPEN-TENFOLD-1 and -2 are aliases of OPEN-PROG-2.
3. **Owner questions decided in two units.** The brief asks this unit for a decision bead for every question
   that needs the owner. When the unit was drafted, 17 of the 32 owner questions already had an
   `owner-decision` bead in their chapter unit; those are named in the register rows and in the map above, and no
   second bead was written for them. For the questions of chapters that were being drafted at the same time, both
   a chapter bead and a packet of this unit now exist: OPEN-CHRON-4 (N41-11, N29b-12), OPEN-GROUND-2 (N41-10,
   N30b-26), OPEN-FOUNDRY-2 (N41-13, N31b-25), OPEN-TETHER-5 (N41-12, N32b-36, N32a-34), OPEN-SAFE-2, -3, -4
   (N41-11; N37-20, N37-09, N37-30), OPEN-GAUNT-1 (N41-10, N38b-26), OPEN-TENFOLD-1 to -4, -6, -8 (N41-06, N41-15;
   N39b-26 to N39b-29, N39b-31, N39b-33). The register rows name both keys. Suggested resolution: the chapter
   bead keeps the engineering (wiring, evidence, checks in the product), the packet here keeps the question as
   put to the owner (default, cost of the default, the ADR); or fold the packet into the chapter bead. If a packet
   of this unit is wanted for the other 17 as well, the pattern of N41-11 and N41-12 applies unchanged.
4. **"No row of the register lacks a planted instance" had no owner for 55 rows.** The four register beads state
   a planted instance for every row, and N41-02 and N41-31 build the four late ones; nothing built, bound or fired
   the others. N41-32 to N41-37 do. Two points had to be settled to make that work.
   (a) A plant can be fired only against the real detector, and detectors arrive from M0 to M5. Each plant
   therefore has a `due` milestone and may be `declared` until then; N41-01's loader rule 5 and N41-02's
   register-wide clause were reworded accordingly (they had required a registered control for every row at M0,
   which cannot hold for a detector built at M3). (b) The control registry's identifiers are `NC-` and four
   digits and it rejects a record that only restates a limit; N41-01's sketch used `ctl:risk-23-dirty-tree`,
   which is now the plant's slug, with the registry's identifier beside it and `removed_check` on refusal-type
   plants.
5. **First readable is not the same as plant due.** N41-01's table of rows first readable per exit (21, 9, 11,
   15, 1, 2) is unchanged; it says when the reviewer can read a signal. For RISK-11, RISK-12, RISK-25, RISK-37 and
   RISK-47 the mechanical detector is built one milestone later than that, so the plant is due later (20, 10, 9,
   15, 3, 2 by due milestone). At the exits in between, those rows are read from a listing and their plants are
   listed as declared.
6. **One bead for the risk review, not nine.** The review at each milestone exit is one mechanism run nine
   times; N41-01 carries it with the per-exit table of rows, and each exit-review seed bead (BD-044 to BD-442) is
   named in its `related` list. If a tracked item per exit is wanted, it belongs in the exit-review beads.
7. **Two codes exist for the envelope refusal.** FROB-REF-761 (a job exceeds the compute envelope in force) and
   FROB-REF-704 (a campaign-class job without an approved compute envelope). The beads of this unit cite 761,
   and N41-REF lists both; the job-admission bead decides which one a job above class E0 with no approval
   returns.
8. **The one refusal these chapters state has no code.** §47.4's "release cell lacks evidence of the kind it
   declares" is raised by a workspace check on a manifest, so it is carried as a harness reason
   (`release-evidence-kind`) and no public code was invented.
9. **Overlaps for the integration pass.** N41-17 (bead lifecycle rules as checks) and N38b-03 (the close audit)
   read the same close records; N41-REF reuses the close audit's reason names and adds only the reasons about
   kill gates, readiness, splitting and exit reviews. N41-16 (tracker hygiene) calls the work-package structure
   check of N44-04 and does not repeat it. N41-28 (the rubric's "must not be said") feeds patterns to the wording
   lint of N00-01 and N00-15. N41-03 (the register) and N40-35 (the owner-input calendar, a per-milestone view of
   it) are two beads by design.
10. **The seed-graph facts of §45.10 describe the 442 seed beads.** The converted graph is larger and its gate
    inventory has grown (the late subsections; GATE-WS-07 and GATE-WS-08). N41-16 keeps the facts as the
    self-test of the tracker check on the seed fixture and applies the same checks, without the counts, to the
    live export.
11. **GATE-WS-07 asks for "100 of 100" from deterministic checks.** A deterministic detector fires the same way
    every time, so the 100 runs are 100 seeded variants in which the hazard enters by different routes (N41-02,
    N41-31). The planted instances of the other 55 rows are fired once each per exit; the gate asks for
    repetition only of its four.
12. **Edits made to files of the first pass** (all small): N41-01 (the `planted` fields in the schema sketch,
    loader rule 5, one Scope line, `related`); N41-02 (the register-wide clause and one acceptance line,
    `related`); N41-04, N41-31, N41-RISK1 to N41-RISK4 (`related`); BD-166, BD-341, BD-344, N41-10, N41-12, N41-26
    (a key added to a `needs` entry that had named none).
