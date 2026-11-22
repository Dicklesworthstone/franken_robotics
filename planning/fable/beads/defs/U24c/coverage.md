# Coverage of drafting unit U24c

Chapter §24 Arena, subsections §24.14 to §24.23, and seed beads BD-064, BD-122, BD-125.
46 bead files: 3 seed beads, 41 new beads (`N24c-01` to `N24c-41`), 2 registries (`N24-CX`, `N24-REF`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §24.14 | Verbs and refusals | BD-122 (`task.new`, `task.show`, `task.list`, static `task.check`), N24c-01 (`task.check` example stage), N24c-02 (`task.verdict`), N24c-03 (`task.settle`), N24c-04 (`eval.run`, `eval.scorecard`), N24c-05 (FROB-REF-502 at scoring time), N24c-06 and N24c-07 (end-to-end scripts), BD-125 (the M1 task families the verbs run), N24-REF (the 21 codes). Verbs owned by other units' seeds: `task.fixture` (BD-123), `eval.preanalyze` and the study verbs (BD-124), `eval.compare` (BD-262), `eval.promote` (BD-263), `eval.qualify` and `eval.ledger` (BD-259) |
| §24.15 | Invariants and proof obligations | N24c-08 (INV-ARENA-01 to -05, PO-ARENA-01 to -04), N24c-09 (INV-QUAL-01 to -04); also named in N24c-02 (INV-ARENA-01, -02), N24c-04 (INV-ARENA-03) |
| §24.16 | Known answers | N24c-10 (the eleven rows as battery rows with controls), N24-CX (their counterexamples); values restated in N24c-02, N24c-04, N24c-08, N24c-09 |
| §24.17 | Gates, kill gate and pilots | Gate table in section (b). N24c-11 (requirement-writing pilot on paper), N24c-12 (KILL-SPEC-REWARD decision); the training pilot is seed BD-257 (another unit); "the first fixture needs no pilot; its analytic layer is the pilot" is carried in N24c-10 |
| §24.18 | Open questions | N24c-13 (OPEN-ARENA-3), N24c-14 (OPEN-ARENA-4), N24c-15 (OPEN-ARENA-5), N24c-16 (OPEN-ARENA-1, OPEN-ARENA-2) |
| §24.19 | Traction capacity is a slice of the wrench set at the commanded wrench | N24c-17 (cone program), N24c-18 (scorecard metric, `task.check` rule, A-08 hook), N24c-19 (fixture rows and gate script) |
| §24.20 | Auditing a task before any design runs on it: foil bodies and intent invariants | N24c-20 (pilot), N24c-21 (intent invariants, in-campaign check), N24c-22 (verb `task.audit`, findings, resolution, qualification refusal), N24c-23 (foil library, foil-as-candidate refusal), N24c-24 (search and archive), N24c-25 (gate run), N24c-26 (KILL-FOIL decision), N24c-28 (end-to-end script) |
| §24.21 | The item bank: importance sampling of battery items for close comparisons | N24c-29 (pilot), N24c-30 (items, law, bank, fit, `eval.items`), N24c-31 (proposal, estimator, `eval.compare --sampling item-bank`, rules), N24c-32 (end-to-end script and GATE-ARENA-12), N24c-33 (GATE-ARENA-13 replay), N24c-34 (KILL-ITEMBANK decision), N24c-35 (fallback: stratified sampling) |
| §24.22 | Exposure domains and the coupling test | N24c-36 (domains, compiler, exposure table), N24c-37 (`task.check --coupling`, study refusal), N24c-38 (fixture and M2 gate script), N24c-39 (pilot for coupling choice), N24c-40 (choice, lock, battery half of the gate), N24c-41 (KILL-EXPODOMAIN decision) |
| §24.23 | The idle arm of the task audit | N24c-27 (mechanism and gate fixture), N24c-28 (end-to-end hand case), N24c-22 (finding machinery and the qualification rule it reuses) |

Seed beads: BD-064 (design lints; a Skeleton bead, written from §14.4, §14.5, §14.7, §14.11, §14.13), BD-122,
BD-125.

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone | Beads of this unit | Expected from another unit |
|---|---|---|---|
| GATE-ARENA-01 | M2 | BD-125, N24c-04, N24c-07 (referee rows at M1) | BD-201 closes it on the batched tiers |
| GATE-ARENA-02 | M3 | N24c-05 (Arena's scoring-time check and its script) | BD-235 (the split and its hygiene gate) |
| GATE-ARENA-03 | M1 | BD-122, N24c-06, N24-REF (refusal rows); N24c-01, N24c-02, N24c-08 (examples, verdict, property tests); N24c-15 (device half) | BD-121 measures the monitor rows (10⁶ traces per operator) |
| GATE-ARENA-04 | M1, M2 | N24c-10 (rows registered and run in the battery) | BD-123 closes it |
| GATE-ARENA-05 | M1 | N24c-08 (PO-ARENA-03), N24c-10 | BD-124 closes it |
| GATE-ARENA-06 | M2 | N24c-14 (the blocking decision) | BD-202 closes it |
| GATE-ARENA-07 | M3 | N24c-04, N24c-08 (M1 rows: column partition, battery identity) | BD-254 closes it |
| GATE-ARENA-08 | M2 | N24c-03 (verb and referee rows at M1) | BD-195 closes it |
| GATE-ARENA-09 | M3 | N24c-11 (paper pilot), N24c-01 (the `task.check` clause) | BD-256 closes it |
| GATE-ARENA-10 | M2 | N24c-17, N24c-18, N24c-19 (measured by N24c-19) | none |
| GATE-ARENA-11 | M3 | N24c-21, N24c-22, N24c-23, N24c-24, N24c-28; measured by N24c-25 | none |
| GATE-ARENA-12 | M3 | N24c-31; measured by N24c-32 | none |
| GATE-ARENA-13 | M3 | N24c-30; measured by N24c-33 | none |
| GATE-ARENA-14 | M2 (fixture), M3 (battery) | N24c-36, N24c-37; fixture half measured by N24c-38; battery half by N24c-40 | none |
| GATE-ARENA-15 | M3 | N24c-27 (mechanism and measurement), N24c-22, N24c-28 | none |
| GATE-QUAL-01 | M3 | N24c-09 (invariant tests), N24c-13 (blocking decision) | BD-260 closes it; BD-259 builds the lane |
| GATE-QUAL-02 | M3 | N24c-09 (INV-QUAL-01 test) | BD-261 closes it |
| GATE-QUAL-03 | M5 | none | BD-383 closes it |
| KILL-SPEC-REWARD | M3 | N24c-12 (decision) | BD-257 runs the pilot |
| KILL-FOIL | M3 | N24c-26 (decision); inputs N24c-20, N24c-25 | none |
| KILL-ITEMBANK | M3 | N24c-34 (decision); inputs N24c-29, N24c-33; fallback N24c-35 | none |
| KILL-EXPODOMAIN | M3 | N24c-41 (decision); inputs N24c-39, N24c-40 | none |
| GATE-SKEL-04 (stage 1 share, through BD-064) | M1 | BD-064 | BD-110 closes it |

## (c) Counts

- Known answers: the 30 rows of Appendix M for §24 (KA-594 to KA-623) are each cited with values in at least one
  bead (the registry bead `N24-KA` is written by U24a). The eleven rows of §24.16 are carried as executable rows
  by N24c-10. Eight further known-answer groups of the late subsections have no Appendix number and are stated
  in full: two-point rig (52.5 N, 0.55 N·m, 25 N, 40 N); strip model (0.525, 0.3486); sled dwell hand case; idle
  hand case (margin −0.3); item-bank variances (0.0975, 0.0075, 0.0085); wrong-model case (0.9975, weight 10);
  tick-keyed timing (3.0 s and 1.5 s); footfall-keyed timing (5.0 s and 4.0 s).
- Counterexamples: 11 numbered (CX-259 to CX-269) and 8 unnumbered local ones (ARENA-W1, -W2, -A1, -A2, -I1,
  -I2, -E1, -E2), all in N24-CX.
- Refusal codes: 21 numbered (502, 520, 523, 525, 526, 528, 529, 530, 532, 533, 535, 536, 538, 539, 552, 840 to
  845) in N24-REF, plus 15 refusals of the late subsections registered under symbolic names because the plan
  assigns them no number.
- Invariants and obligations: INV-ARENA-01 to -05, INV-QUAL-01 to -04, PO-ARENA-01 to -04 (13).
- Open questions: OPEN-ARENA-1 to -5 (5). Kill gates: 4. Pilots: 4 (N24c-11, N24c-20, N24c-29, N24c-39).

## (d) Needs from other slices

Prerequisites with no known key (also in the beads' `needs` fields):

- §23: the training manifest of a policy or tuned controller, recorded with its `ControllerId` (N24c-05).
- §19: the scene's route graph with a corridor query exposed as a telemetry channel (N24c-21).
- §28.8: `fs-archive` MAP-Elites and an `fs-dfo` rank-based optimiser admitted through the airlock by M3; the
  product archive bead BD-342 is M4 (N24c-24).
- §22.12.3: backup controllers per morphology class callable as plain controllers at M3; the shield bead BD-338
  is M4 (N24c-27).
- §26.9: the variance-adaptive betting confidence sequence as a library the paired comparison can select
  (N24c-31).
- §19: a per-world cache of `field` samples in `frob-scene`; §14 and §19: typed mission roles and crossing events
  as telemetry events (N24c-36).

Dependencies that beads of other units should take on beads of this unit (the lead may add them):

- BD-256 (requirement-writing test) on N24c-11 (its paper pilot).
- BD-259 and BD-260 (qualification lane and its gate) on N24c-13 (OPEN-ARENA-3).
- BD-202 (fixture 2) on N24c-14 (OPEN-ARENA-4).
- BD-255 (requirement verdicts on batched tiers) on N24c-15 (OPEN-ARENA-5, the guard band).
- BD-123 (fixture 1) on N24c-03: the cart starts "settled under gravity from the reset cache", and `task.settle`
  is an M1 verb, while the cache's seed bead BD-195 is M2.
- BD-262, BD-124, BD-259, BD-264, BD-276 need the refusal hooks of N24c-31 and N24c-37 (item-bank and
  exposure-table refusals raised inside their admission paths).
- BD-222 and BD-312 (the M2 and M3 verb registry beads) should carry `task.check --coupling`, `task.audit`,
  `eval.items` and the `--sampling` modes of `eval.compare`.
- The registry owners should assign: Appendix M numbers to the eight late known-answer groups; Appendix N numbers
  to the eight local counterexamples; refusal code numbers to the 15 symbolic refusals.

## (e) Contradictions, gaps, and what was done

1. **BD-064 is not an Arena bead.** The seed "Design lints with ranked repairs as typed edits" was mapped to
   §24.22 by keyword. It is the lint layer of `frob-design` (stage 1 of the preflight). It was written from the
   design chapter (field reference, fill rule, typed edits, stage table, refusal table) with parent WP-07.
2. **"Refusals 520 to 528" in the seed of BD-122.** The gate lists nine codes, four of which now carry numbers
   outside that range (535, 536, 538, 539). The gate's list was used.
3. **"On the referee" for timed docking (BD-125).** The docking mission is an exact equality that floating point
   cannot decide; it runs as the exact scalar plant on the host. The physics families run on the referee.
4. **`task.settle` is M1 in the verb table; the reset-cache seed bead is M2, and §24.7 settles on `batch-cpu`,
   which does not exist at M1.** N24c-03 delivers the verb and a referee settle path at M1; BD-195 keeps the
   batched cache and GATE-ARENA-08.
5. **`eval.run` (M2) and `eval.scorecard` (M3) are named by an M1 seed bead.** N24c-04 delivers a reduced referee
   form at M1 with the same schemas.
6. **GATE-ARENA-12's "variance ratio at or below 1/ε".** The subsection's own counterexample gives 0.9975/0.0975
   = 10.23, above 1/ε = 10. The provable bound is on the second moment (exactly 10 there). The gate is
   implemented on the second-moment bound and the variance ratio is reported beside it (N24c-31, N24c-32,
   N24c-29).
7. **§45.12 names KILL-COUPLING for §24.22.** The subsection and §40.8 name KILL-EXPODOMAIN (KILL-COUPLING is
   §25.29's). KILL-EXPODOMAIN was used (N24c-41 records the confusion).
8. **Gates without register rows.** GATE-ARENA-10, -11, -12, -14 and the three late kill gates appear only in
   §40.8's convergence table; -13 and -15 are in §40.9. The defining subsection's thresholds and milestones were
   used.
9. **Refusals without code numbers** in §24.19 to §24.23 (15 rules). Registered under symbolic names; no number
   was invented.
10. **Late known answers and counterexamples have no Appendix M or N rows.** Carried in full in the beads and in
    N24-CX under local labels.
11. **No paper-scale pilot is named for the Frontier parts of §24.21 and §24.22.** Pilots N24c-29 and N24c-39
    were written in the form §0.7 requires (their protocols are DESIGN of this unit).
12. **Bands between a gate's pass threshold and its kill threshold are undefined** (task audit: detection 60% to
    80%, clean-task findings 20% to 30%; item bank: factor 2 to 4). N24c-26 defines a `not yet` state with one
    repair cycle on a fresh fixture; N24c-34 defines `kept-unproven`. Both are marked DESIGN.
13. **"Does not improve success ... across five seeds" (KILL-SPEC-REWARD)** has no decision rule. N24c-12 fixes
    a paired-sign rule (at least four of five seeds and a positive mean) before the pilot is read; DESIGN.
14. **One verb for the audit, but resolution mutates a task.** The registry lists only `task.audit`; N24c-22
    puts resolution under a `--resolve` mode of that verb; DESIGN.
15. **Schema extensions implied by the late subsections:** the `:intent` tag on online signals (N24c-21), the
    `foil: true` marker and the foil-only `wrench-site` component in the design language (N24c-23), `:domain`
    and its keys on randomisation draws and `:alternatives` (N24c-36, N24c-40), `wrench_command` in a task
    (N24c-18). Each is an optional addition; the owners of the design and task schemas should confirm.
16. **Numbers the plan leaves open were fixed as DESIGN and labelled in each bead:** thresholds of the standing
    tasks (BD-125); repair ranking and shell-thickness proposals (BD-064); the near-boundary threshold 0.1
    (N24c-01); rig constants and the slip-onset definition (N24c-19); foil bounds (N24c-23); archive bins
    (N24c-24); response-model regularisers (N24c-30); replay protocol constants (N24c-33); fixture ranges
    (N24c-38). All become part of a file's identity when it is first registered.
17. **Duplicate refusal codes across chapters** for one situation (552 and 553; 532 and 555; 533 and 542).
    N24-REF keeps both and states which verb raises which.
18. **FROB-REF-842** is a budget-shaped refusal with an `8xx` number; by the envelope's family rule its exit code
    is 3 (N24-REF).
19. **INV-ARENA-04 and FROB-REF-530** belong to the fixture mechanism (BD-123, another unit); they are tested
    here by N24c-08 and registered in N24-REF.
20. **Line ranges.** All ten subsection ranges in the brief began at their headings; none needed adjustment.
