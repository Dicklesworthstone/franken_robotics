# FIXLOG_A: reconciliation pass over parts/P00.md to P06.md

One line per finding: source and item | file and subsection | status. "applied" means the part file was edited in
this pass. "already fixed" means the text was already correct when the pass started. Arithmetic that was touched
was recomputed; the recomputed values are given where they changed.

## Specific decisions of FIX_DECISIONS.md

- Decision 1 (verb names), `world.separate` | P01 C-18 | already fixed
- Decision 1, `world.separate` | P01 LEDGER verb list | applied (stale line named by GAPFILL_1 d1)
- Decision 1, `skill.transport` against `mission.transport` | P06 §22.10 | applied
- Decision 1, `telemetry.export-html` | P04 §20.10 (now a pointer to §32.15, §29.12), §20.18 verb row | applied
- Decision 2 (proof kinds) | P01 §12.4: registry named `frob.cert.kinds/1`; rows `taylor-box`, `sos-gram`, `symmetry-transport`, `composition` added; `monomial-product` (used by §25.31) added on the same rule; LEDGER kind list updated | applied
- Decision 3 (forks) | P04 §19.21: `autopsy.fork` replaced by `run.fork` (§29.8) | applied
- Decision 4 (screening audit) | P00–P06 | skipped: no text in these files states the audit
- Decision 5 (energy audit, one flag `energy-created`) | P03 §16.8 labels, §16.10, LEDGER label list | applied
- Decision 6 (derivative labels) | P06 §22.9.3, §22.9.4, LEDGER label list | applied (smooth→`smooth`, one-sided→`one-sided`, set-valued→`generalized`, surrogate→`smoothed`)
- Decision 7 (requirement verdicts) | P01 C-30, two places | applied
- Decision 8 (pilots, residual of 575.7 µs) | P00 §0.7, §2.3 item 6, §2.5, §2.6 family-query row; P05 §36.5, §21.8 | applied
- Decision 9, 7.7 | P00 §2.3 item 1; P05 §36.4 item 1; P06 §23.1 | applied
- Decision 9, 513 s | P05 §36.3 row 4, LEDGER line | applied
- Decision 9, H-05 at 1% to 63% of 410 s | P00 §0.8, BET-26; P06 §22.9 | applied (P01 C-10 already fixed)
- Decision 9, §16.11 | P03 §16.11 | applied (0.14 to 0.71% and 0.12 to 0.6%; batch-cpu share recomputed against 1.7 M: 0.10 to 0.16 and 0.40 to 0.65)
- Decision 9, §20.4.6 | P04 §20.4.6, §20.20 | applied (616 decisions; overpayment 2.7%)
- Decision 9, §21.20 | P05 §21.20 fork-copy row | applied (about 0.5 ms; an eighth of a batch step)
- Decision 10 (device memory budget 85%) | P04 §19.15 (203 GB, 189 GiB; scene counts recomputed: 2,700, 340, 340, 42, 5, 14, 1), §19.16 admission; P05 §36.10 (5.8 M and 2.1 M worlds; scene counts) | applied
- Decision 11 (size buckets) | P05 §21.11 sketch, §21.11 worst case (25 in the 36 bucket, Ω₁ ≤ 1.44, Ω₃ ≤ 3.0), OPEN-SPARK-3 | applied
- Decision 12 (notation) | P02 §13, §14, §15 and LEDGER; P04 §20.13 and LEDGER | applied (bare group symbol also replaced: Γ in P02 §13.5, Γ_mech in P04)
- Decision 13 (model-table size) | P03 §17.4 struct comment, §17.7 | applied (12 kB, 3 MB, at most about 50 MB for 4,096 designs)
- Decision 14 (matrix accelerators) | P03 §18.12 last sentence | applied
- Decision 15 (mixed-batch figure) | P00 §0.8 H-03, §2.4 lever 1; P03 §17.1 | applied (P00 §1.1, P05 §21.11 already fixed)
- Decision 16 (structural mass ceiling) | P00 §0.8 H-12; P01 C-35 limits and ceiling | applied
- Decision 17 (rare-event figures; BET-32) | P00 §0.8 H-07, §2.4 lever 4, §2.6 row, BET-32; P01 C-27 payoff | applied
- Decision 18 (sampling-controller cost; thresholds 1 and 6) | P00 BET-27; P06 §22.8 rules and verb line, §22.17 pilot row, GATE-REFLEX-06 | applied (GATE-REFLEX-06 and the pilot row set back from 5 to 6)
- Decision 19 (tier-correction weights) | P03 §16.8 step 4 and step 1 default prediction | applied
- Decision 20 (foundation defects) | P00 §1.7 (sampler bullet, count of five, wrench hazard), BET-04, LEDGER line; P06 ADR-CORTEX-03, §23.6 | applied
- Decision 20 (rainflow in `fs-material`) | P01 C-35 foundations; P00 §1.7 | applied
- Decision 20 (64-bit atomics) | P05 §21.16, §21.6 rule 7 pointer to §21.26 | applied
- Decision 20 (cloth oracle posture) | P00–P06 | skipped: stated only in P12
- Decision 21 (ten-fold decision rule) | P00 §2.1, LEDGER line | applied
- Decision 22 (energy-budget card) | P01 C-31 test and term definitions; P06 §22.11 `.fenergy` sketch (reserve set to 0.00, totals stated) | applied
- Decision 23 (tube tables [M], inclusion stated once) | P01 C-31 tier 2, tier line, constructions table, `tube-inclusion` row | applied
- Decision 24 (referee coverage band with its assumption) | P00 §0.8 H-11, §1.8; P03 §16.11; P05 §36.3 row 17, §36.9 | applied
- Decision 25 (verdict register) | P00 §0.8 rows and closing paragraph | applied; "Block Bnn" cell references replaced by §25.32, §24.22, §16.21, §23.29, §25.33, §20.24

## GAPFILL_1.md

- (b) 1, 2 Metal 64-bit atomics | P05 §21.16 | applied (see decision 20)
- (b) 3 working-set cap | P04 §19.15, P05 §36.10 | applied (see decision 10)
- (b) 8 energy budget labels | P01 C-31 | applied
- (b) 14 fatigue crate | P01 C-35 | applied
- (b) 4–7, 9–13, 15–27 | P00–P06 | skipped: marked "already there", "no", or addressed to Appendix C and P12
- (b2) fifth foundation defect | P00 §1.7, P06 §23 | applied
- (b2) ten-fold statistics | P00 §2.1 | applied
- (b2) withdrawn 94% attribution | P00, P05 | applied
- (b2) bundle manifest 160 and 25 | P00–P06 | skipped: the counts 154 and 20 do not occur in these files
- (d) 1 | P01 | already fixed in C-18; LEDGER applied
- (d) 2 | P06 §22.10 | applied
- (d) 3 | P01 C-31 | applied
- (d) 4 | P03 §16.10 | applied
- (d) 5 | P06 §22.9.3 | applied
- (d) 6 | P01 C-30 | applied
- (d) 7 | P00 §0.7, §2.3; P05 §36.5 | applied
- (d) 8 | P00 §2.1 | applied
- (d) 9 | P05 §21.6 rule 7 | applied (pointer to §21.26; P09 side not mine)
- (d) 10 | P00 §1.7; P06 ADR-CORTEX-03 | applied
- (d) 11 | P01 C-35 | applied
- (d) 12, 14, 15 | P12, parts2 | skipped: not my files
- (d) 13 | P05 §21.16 | applied

## GAPFILL_2.md section (b)

- b.1 register rows H-01 to H-04, H-04b, H-06 to H-09, H-11, H-12, H-12b, H-16 to H-19, H-21, H-22a, H-22b, H-25 to H-29 | P00 §0.8 | applied; H-17 keeps "continuation is [M]" (owner §25 and REVIEW_COHERENCE F29) against b.1's "[F]"
- b.1 closing paragraph | P00 §0.8 | applied
- b.2 §2.3 last paragraph | P00 | applied
- b.2 §2.4 first sentence, levers 1, 2, 4, 5, 6 | P00 | applied
- b.2 §2.5 four items | P00 | applied
- b.2 §2.6 sentence, column header, failure row, closing paragraph | P00 | applied
- b.2 §1.8 | P00 | applied
- b.2 §1.6 N11 | P00 | applied
- b.3 BET-14, BET-16, BET-17, BET-20, BET-27, BET-32, BET-34 | P00 §4 | applied
- b.3 BET-36 | P00 §4 | applied, merged with the gate-aligned thresholds already in place (KILL-FILM, KILL-ADVECT)
- b.3 NONGOAL-11, -13, -14 | P00 §6 | applied
- b.4 H-01 ADR-REFLEX-03, §22.8 rules, verb line, GATE-REFLEX-06 | P06 | applied
- b.4 H-01 §21.20 and ledger | P05 | applied
- b.4 H-02, H-21 C-23 limits | P01 §10 | applied
- b.4 H-02 §23.11 table and note, GATE-CORTEX-11 | P06 | applied (1/f range recomputed to 4 to 50)
- b.4 H-21 ADR-CORTEX-06, §23.12 row and reset rule, GATE-CORTEX-13 | P06 | applied
- b.4 H-03 §17.1, §17.14 measure (5) | P03 | applied
- b.4 H-04 §16.6 tree-operator ceiling | P03 | applied; ADR-KIN-07 labelled to match
- b.4 H-04 ADR-SPARK-08 | P05 §21.2 | applied
- b.4 H-06 C-17 and C-32 | P01 §10 | applied
- b.4 H-07 C-27 | P01 §10 | applied
- b.4 H-11 §16.11 two places | P03 | applied
- b.4 H-11 §36.3 row 17, §36.9 | P05 | applied
- b.4 H-12 C-35 | P01 §10 | applied
- b.4 H-12 A-15 row | P02 §14.11 table and sentence above it | applied
- b.4 H-16 ADR-HAB-07 | P04 §19.2 | applied
- b.4 H-22 §23.11 consequences | P06 | applied (cites §23.28)
- b.4 rows naming P07–P10 | — | skipped: not my files

## GAPFILL_2.md section (d)

- (d) 1 | P01 C-31; P06 §22.11 sketch | applied
- (d) 2 | P01 C-31 | applied
- (d) 3 | P03 §17.4, §17.7 | applied
- (d) 4 | P03 §17.14, KILL-PHYLUM (unit: harmonic mean; owner statement); P05 KILL-BUCKET defers to §17 | applied
- (d) 5 | P03 §18.12 | applied
- (d) 6 | P03 §18.5 | applied (pointer to §18.19 and GATE-TRAC-11)
- (d) 7, 9, 11, 14, 19, 21 | P07–P11 | skipped: not my files
- (d) 8 | P01 §12.4 | applied
- (d) 10 | P04 §20.10, §20.18 | applied
- (d) 12 | P05 §36.5, §21.8 | applied ("FACT, Appendix K"; 1.345 and 1.088)
- (d) 13a | P00 BET-25 | applied (cites §22.21)
- (d) 13b | P00 NONGOAL-15 | applied (cites §16.21)
- (d) 13c | P04 §20.4.7 | applied (KILL-SCHED named inline)
- (d) 15 | P00 BET-32 | applied
- (d) 16 | P03 §16.8 | applied
- (d) 17 | P03 §17.13 | skipped: owner text stays as written
- (d) 18 | P03 §16.12 | applied
- (d) 20 | P06 §22.8 | already fixed (owner figures 0.33, 1, 1.7, 5.2 stand)
- (d) 22 | P05 §21.11 KILL-SLOT | applied (cites §32.21 and GATE-TETHER-09)
- (d) 23 | — | skipped: no change asked
- (d) 24 | P06 §22.13 | applied: one rule added ("A change of group at a mode switch"), answered from the section's own definition of exact reduction
- (d) 25 | P00, P03 | applied
- (d) 26 | P00, P01 | applied
- (d) 27 | P01 C-27 | applied
- (d) 28 | P00 §2.6, BET-32 | applied
- (d) 29 | P03 §16.6 | applied
- (d) 30 | P00 BET-27; P06 GATE-REFLEX-06 | applied (1 and 6)
- (d) 31 | P06 §22.8 | applied
- (d) 32 | P05 LEDGER | applied
- (d) 33 | P00 §2.6 | applied
- (d) 34 | P00 §0.8 H-19, BET-14 | applied
- (d) 35 | P00 §0.8 H-18 | applied
- (d) 36 | P00, P03, P05 | applied

## CHALLENGE_ADJUDICATION.md

- §3 rows and §5 answer | P00 §0.8, §2 | applied through GAPFILL_2 (b)
- H-22b curve (2.8 at 0.8; ρ per outcome class) | P03 §16.8 | applied (the sentence "about 2 unless ρ exceeds 0.9" removed)
- H-21 denominator | P00 NS-19 | applied ("12% of the steps of an early-stopped from-scratch control")
- H-01 stand-in wording | P06 §22.20 | applied

## LEDGER blocks of parts2

- P13a M.38 item 1 (1.46 against 1.43) | P00, P05 | applied (decision 8)
- P13a M.38 item 2 (7.8) | P00, P05, P06 | applied
- P13a M.38 item 3 (515 s) | P05 | applied
- P13a M.38 item 4 (film tile counts) | P04 §19.10.3 | applied (whole tiles: 361 tiles, about 370 kB; 16 tiles, 16 kB)
- P13a M.38 item 5 (190 GB) | P04 §19.15; P05 §36.10 | applied
- P13a M.38 item 6 (2.6%) | P04 §20.4.6 | applied
- P13a M.38 items 7, 8 | P08 | skipped: not my files
- P13a M.38 item 9 (memory share of §1.8) | P00 §1.8 | already fixed (1 to 3%)
- P13a M.38 item 10 (C-15 level) | P01 C-15 | applied (two-sided 97.5%, the level of §24.10)
- P13a M.38 item 11 (C-32 0.64) | P01 C-32 | skipped: 0.64 is the rounding of 2^(−13/20) = 0.637 for the 13 bisections the sentence names
- P13a device budget 85% against 80% | P04, P05 | applied
- P13a directory entry size | P04 §19.15 | applied (4-byte directory entry plus 2-byte coarse sample, 6 bytes in all)
- P13a §36.3 row 1 (0.67 × 4.0 M = 2.7 M against a floor of 2.0 M) | P05 §36.3 row 1 | applied as a clarification: the floor is an interim value, half the planning value, and is not the 0.67 tolerance; no number changed (owner rule; choice recorded)
- P13a quadruped cycle 360–420 s, 354 s, 325 s, 300 s | P00 §2.3 items 2, 3; P05 §36.4 items 4, 6, 7 | applied as clarifications: 360–420 s is 300 s plus an agent turn of 60–120 s, 354 s is the composite iteration of §2.2, 300 s is a planning value under the published 325 s; no number changed (choice recorded)
- P13a notation | P02, P04 | applied
- P13a cross-reference §16.12 | P03 | applied
- P13a nine template families | P03 §17.14 arm B | applied: "at most nine padded templates, one per family of the eighteen archetypes that share a maximal tree (the grouping is frozen with the arm)"; the grouping itself is not defined anywhere
- P13a §16.6 row rule | P03 §16.6 | applied: Q and H take §36.3's capacities (Q's 152 rows raised to 160); bytes recomputed from the section's own formula (31 kB and 92 kB; 35 and 97 kB with the state of §36.10); Jacobian share 24 to 49%
- P13a backlash sign convention | P03 §18.7 | applied (§15.6's δ = θ/N − q)
- P13a P01 ledger refusal codes | P01 LEDGER | applied (826, 829, 833, 834, 835)
- P13a flags (fourth packet flag) | P00 §8.2 sketch, NS-20; P01 §12.2 | applied
- P13a tier names share `referee` | — | skipped: no change asked
- P13a boundary corpus 2 × 65,536 | P04 §20.16 | applied (the dense grid is one of the two)
- P13a belt-law known answer expected by §14.13 | P02 §14.13 row | applied (cites the skid-steer no-slip row of §18.16)
- P13b model-form label spelling | P03 §16.9 | applied (the type variants carry the labels `model-robust` and `model-conditional`, the spelling of §8.2 and §23)
- P13b §2.1 "ten paired campaigns" | P00 | applied
- P13b GATE-SAFE-03, GATE-ATEL-06 | P08, P09, P11 | skipped: not my files
- P14 bucket sets | P05 | applied
- P14 GATE-SPARK-21 at M0 | P05 | already fixed
- P14 GATE-TRAC-09, GATE-HAB-11, GATE-SHIELD-01 milestones | P03, P04, P06 | already fixed
- P14 GATE-REFLEX-01, GATE-SKEL-03 | P06, P02 | already fixed; first archetypes named (see REVIEW_COHERENCE §5 item 2)
- P14 `strict` rows of GATE-KIN-04 and GATE-TRAC-06 | P03 §16.17, §18.17 | applied ("M1 (referee); M2 (`strict`)", as the register has them)
- P14 OPEN-SPARK-2 defined twice | P05 §21.25 | applied (the longer list; §36.16 named as the restatement)
- P14 kernel reflection tier | P00 §4 | already fixed
- P15 first archetypes at M1 | P02 §14.10; P06 GATE-REFLEX-01 | applied
- P16a discrepancies 1–6 | P00, P03, P04, P05 | applied (see decisions 8, 9)
- P16a discrepancies 7–10 | P08, P09 | skipped: not my files
- P16a default capacities | P03 §16.6 | already fixed (C and R); bytes recomputed, see above
- P16a squeeze-film tolerance | P04 GATE-HAB-06 already fixed; known-answer row §19.18 | applied (2%)
- P16a energy-budget labels | P01, P06 | applied
- P16b 1 `world.separate` in §12's ledger | P01 LEDGER | applied
- P16b 3 proof-kind registries | P01 §12.4 | applied
- P16b 4 one claim per bundle | P01 §12.4 | applied: one bundle per claim, several bundles under one Merkle root, and the answer cites the root (owner rule; choice recorded)
- P16b 5 flags object key | P01 §12.4 already fixed; P06 `.fskill` and `.fenergy` sketches | applied (`authorization`)
- P16b 6 schema names | P04 | already fixed
- P16b 7 `ShapeClass` sketched twice | P05 §21.11 sketch comment | applied: §21.11 declared the device-side view of §17.4's type, Appendix I holds the one definition
- P16b 9 `NoAuthorization` | P00 §8.2 | applied (comment on the packet's fourth constant)
- P16b 11 sensor clocks | P02 §14.4 example | applied
- P16b 13 cluster limit | P03 §16.13 | applied (the limit of seven applies to task clusters)
- P16b 2, 8, 10, 12, 14, 15 | P07–P11, parts2 | skipped: not my files

## REVIEW_MATH.md

- F-17 | P01 C-09 item 3 | applied
- F-21 | P01 §12.4 reflectors (`farkas_sound`, `ldltPos_sound` over every linearly ordered field containing ℚ) | applied
- F-13 (R-13) | P03 §16.11 | applied: R-13 was in place but computed against the withdrawn 0.5 M target; recomputed against 1.7 M
- F-14 | P00 §0.8, BET-26; P06 §22.9 | applied (P01 already fixed)
- R-01 to R-12b, R-14 to R-26 | P01, P02, P03, P05 | already fixed
- Wrap of lines lengthened by the earlier substitutions | P01, P02, P03 | applied (paragraphs re-wrapped near 118 characters, words unchanged)

## REVIEW_COHERENCE.md

- F02 prose (name the table, add four rows) | P01 §12.4 | applied
- F32 prose (known-answer row follows the gate) | P04 §19.18 | applied
- F03 (`fidelity.rank`) | P03 §16.13 | applied as a pointer: the verb stays registered for tier pairs (Appendix J lists it) and names `eval.rank-agreement` for rungs
- F03 (`eval.rankcheck`) | P06 body already fixed; LEDGER | applied
- F04 (one open question owns the capacities) | P03 §16.6, OPEN-KIN-3 | applied (OPEN-SPARK-3 owns Q and H)
- F10 KILL-ADVECT | P04 LEDGER kill list | applied
- F13 | P06 | superseded by decision 18 (thresholds 1 and 6)
- F24 claim name "Performance of the method" | P02 (1), P03 (2), P04 (2), P06 (11) | applied (replaced by "Design comparison"); the two-claim rows were not split, since rows are not renumbered
- F30 | P04 §19.12 | applied (ownership sentence)
- F33 | P00 LEDGER (`K-cay-step`) | applied; NS-11 already fixed
- F36 | P05 §36.10 | applied (band columns exclude labels, which add 12.5%)
- F44 | P05 LEDGER | applied
- F05, F06, F07, F08, F12, F15 to F23, F25, F27, F29, F31, F34, F35, F40, F41 | P00–P06 | already fixed
- §5 item 1 (which threshold governs a bet) | P00 §4 | applied for BET-27; BET-15, -18, -26, -29, -31, -34, -35, -36 already agree with their gates
- §5 item 2 (first archetypes) | P02 §14.10; P06 GATE-REFLEX-01 | applied
- §5 item 3 (`strict` at M1) | P03 | applied (M2)
- §5 item 7 (proof-kind names, authorization key) | P01, P06 | applied
- §5 item 8 (reference capacities and their owner) | P03, P05 | applied
- §5 item 9 (owner of `frob-kernels`) | P00 §0.5; P03 §16 head | applied (Spark, as §21 and §34 have it)
- §5 item 10 (retrofit search) | P04 §19.12 | applied; `experiment.suggest` and A-19 are not in my files
- §5 items 4, 5, 6, 11, 12 | §39, §28, Appendix J, §42 | skipped: not my files
- §5 item 13 (§0.6 placeholder) | P00 §0.6 | skipped: the lead fills it

## Not resolved here

- GATE-FID-07 is at M2 in P03 §16.17 (the milestone skeleton and BET-34) and at M3 in the register of §40 (P13b). The register is not my file.
- KILL-INHERIT (P06 §23) still states its 60% against "from-scratch steps"; no finding gives a corrected denominator.
- The grouping of eighteen archetypes into at most nine template families (P03 §17.14, P05 KILL-BUCKET, BET-18) is named and not defined.
- The interim floor of 2.0 M world-steps/s (P05 §36.3 row 1) and the 300 s planning value (P00 §2.3, P05 §36.4) are kept and annotated, not changed.
