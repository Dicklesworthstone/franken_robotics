# Coverage of drafting unit U39b (§39.9 to §39.22, the ten-fold test, second half)

53 bead files: 3 seed beads (BD-143, BD-317, BD-429), 47 new beads (N39b-01 to N39b-47) and 3 registry beads
(N39-KA, N39-CX, N39-REF).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §39.9 | Disqualification | N39b-01 (grounds, scope, effects, integrity detectors, cap-breach censoring), N39b-02 (canaries, access-log and firewall audit), N39b-03 (KILL-TENFOLD-CERT), N39b-04 (KILL-TENFOLD-LEAK); applied in BD-317, N39b-13 |
| §39.10 | The outcome rules, written in advance | N39b-05 (frozen parameter block, outcome table, sentences, actions, ADR drafts, across-strata rules, what remains if withdrawn, no re-test), N39b-06 (KILL-TENFOLD-SPEED), N39b-07 (KILL-TENFOLD-TIE, engine advantage not established, the four engine-internal gaps), N39b-43 (the tie test), N39b-33 (ratification), N39b-15 (wording) |
| §39.11 | The incumbent parity baseline at M0 and M1 | N39b-08 (GATE-TENFOLD-01), BD-143 (GATE-TENFOLD-02; what the baseline may and may not change), N39b-09 (lever shares read, KILL-TENFOLD-LEVER at M1), N39b-10 (re-read at M2; the KILL-PARITY redo), N39b-32 (licences), N39b-31 (the agent) |
| §39.12 | The partial gate at M3 | BD-317 (scope, what arm A has, procedure, record, passing), N39b-11 (what else is decided on its record), N39b-29 (cost and envelope), N39b-12 (rehearsal script) |
| §39.13 | The tournament at M7 | N39b-13 (order of work, pin freeze, breach rules, GATE-TENFOLD-07), N39b-14 (the report: one table per cell, one ledger per arm), BD-429 (the decision rule), N39b-16 (its pilot), N39b-17 (dress rehearsal) |
| §39.14 | The replication kit | N39b-18 (contents, manifest, limits), N39b-19 (the scorer, level 1), N39b-20 (level 2), N39b-21 (GATE-TENFOLD-08 script) |
| §39.15 | What the test does not measure | N39b-15 (the limits block, verbatim, and the wording rules), N39b-14 (prints it) |
| §39.16 | Worked card: from a phase ledger to a ratio | N39-KA (fixtures A, B, C; rows TF-01 to TF-10), N39b-19 and N39b-22 (reproduced exactly), N39b-12 (scaled rehearsal), N39b-09 (lever reading on the card) |
| §39.17 | Invariants and known answers of the harness | N39b-23 (INV-TENFOLD-01 to -08 as an auditor with property tests), N39-KA, N39-CX (the seven negative controls and the planted wrong scorers), N39b-22 (GATE-TENFOLD-03) |
| §39.18 | Gates and kill gates | Table (b); kill-gate decision beads N39b-03, -04, -06, -07, -09, -10, -24, -25 |
| §39.19 | Open questions | N39b-26 (OPEN-TENFOLD-1), N39b-27 (-2), N39b-28 (-3), N39b-29 (-4), N39b-30 (-5), N39b-31 (-6), N39b-32 (-7), N39b-33 (-8); OPEN-TENFOLD-9 (blocks nothing) in N39b-07 |
| §39.20 | Estimating the ratio, and the smallest suite that can certify | N39b-34 (`eval.preanalyze` for ten-fold plans, the population refusal), N39b-35 (two estimands, bounded paired contrast, result types, zero-count intervals, cluster unit), N39b-36 (GATE-TENFOLD-10), BD-429, N39b-16 |
| §39.21 | Rival arms, protocol controls and the sealed suite | N39b-37 (rival classes, rival audit, experiment pack, GATE-TENFOLD-11), N39b-38 (controls: run plan, isolation, blinding, publication set), N39b-39 (the second claim: a new engineering operation), N39b-40 and N39b-41 (the six sealed mission types, planted mutations, warm-campaign rules), N39b-42 (GATE-TENFOLD-12) |
| §39.22 | Three arms, the recovery ratio and the phase ablations | N39b-43 (r_T, r_C, cases, reading, governing rival), N39b-44 (one receipt first), N39b-45 (three ablations and bounds), N39b-46 (`report.campaign --arms`, `--ablate`), N39b-47 (GATE-TENFOLD-13) |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|
| GATE-TENFOLD-01 | N39b-08 (measurement and record); N39b-32 (its licence precondition) | BD-039, BD-040 (harness and raw rows); N00-13, N00-14 |
| GATE-TENFOLD-02 | BD-143; N39b-31 (agent pin) | BD-147 records it at the M1 exit; N00-18 |
| GATE-TENFOLD-03 | N39b-22 (battery and script); N39-KA, N39-CX, N39-REF; N39b-23 | BD-314 cites it at M3 |
| GATE-TENFOLD-04 | Touched: BD-429, N39b-16 (pre-analysis), N39b-25, N39b-28, N39b-30, N39b-33, N39b-38, N39b-40 | Measurement of the freeze-and-seal order belongs to U39a (§39.4, §39.5, §39.7; BD-314, BD-428, BD-430) |
| GATE-TENFOLD-05 | Decided inputs: N39b-26, N39b-27; fallback N39b-24 | Measurement belongs to U39a and U38a (§39.3; BD-314, BD-432, N38a-34) |
| GATE-TENFOLD-06 | BD-317 (record); N39b-05, N39b-11, N39b-12, N39b-14, N39b-15, N39b-23 | BD-314, BD-315, BD-316 (protocol and arms) |
| GATE-TENFOLD-07 | N39b-13 (conduct and record); N39b-14, N39b-15, N39b-17, N39b-23 | BD-428, BD-430 to BD-434 |
| GATE-TENFOLD-08 | N39b-21 (script), N39b-18, N39b-19, N39b-20 | — |
| GATE-TENFOLD-09 | N39b-09 (the funding half at M1) | The program-wide refusal of speed comparisons belongs to U39a (§39.8); N00-17; BD-147 records it; BD-354 |
| GATE-TENFOLD-10 | N39b-36 (script), N39b-34, N39b-35, N39-KA, BD-429 | BD-314 cites it |
| GATE-TENFOLD-11 | N39b-37; N39b-28 | BD-314, BD-315, BD-430 |
| GATE-TENFOLD-12 | N39b-42 (script), N39b-40, N39b-41, N39b-39 | BD-428 |
| GATE-TENFOLD-13 | N39b-47 (script), N39b-43, N39b-44, N39b-45, N39b-46, BD-317 (M3 row), N39b-13 (M7 row) | — |
| KILL-TENFOLD-SPEED | N39b-06 (M3; the M7 reading under N39b-13); reading function in N39b-05 | — |
| KILL-TENFOLD-TIE | N39b-07; statistics N39b-43 | N25a-18 (KILL-AUGUR-TIE), N12-30 (KILL-FAMILYCERT) read the same record |
| KILL-TENFOLD-CERT | N39b-03; N39b-01 | KILL-AUGUR-FALSECERT (mission subsystem) |
| KILL-TENFOLD-LEAK | N39b-04; N39b-02, N39b-01 | — |
| KILL-TENFOLD-LEVER | N39b-09 (M1), N39b-10 (M2) | — |
| KILL-TENFOLD-ADJ | N39b-24 | Mechanism in §39.3 (U39a, U38a) |
| KILL-TENFOLD-REACH | N39b-25; BD-429, N39b-16 | Mechanism in §39.7 (U39a) |

Also touched: GATE-GAUNT-08 (the pilot rule) by N39b-16.

## (c) Counts

- Known answers: 22 new rows (local labels TF-01 to TF-22 in N39-KA) plus two existing rows cited (KA-009,
  KA-014). Every value of §39.16, §39.17, §39.20 and §39.22 was recomputed while drafting; none disagreed.
- Counterexamples and negative controls: 24 new entries (TF-CX-01 to TF-CX-24 in N39-CX) plus two existing
  entries cited (CX-003, CX-008).
- Refusals: 24 rows in N39-REF: 11 product codes (10 existing: FROB-REF-532, -552, -761, -840, -860, -862, -863,
  -865, -866, -899; 1 new, unnumbered: `suite-too-small-for-population-claim`) and 13 harness refusals or voids
  with slugs `tenfold.*`.
- Invariants: INV-TENFOLD-01 to -08 (N39b-23). Open questions: OPEN-TENFOLD-1 to -9. Bets served: BET-10,
  BET-13, BET-14, BET-19. Risks named: RISK-25, RISK-26, RISK-32, RISK-51, RISK-52.

## (d) Needs from other slices (no key known)

- §39.1: the harness core in `frob-conformance/tenfold/` and its driver binary (called `frob-tenfold` in these
  beads; subcommand names attach to whatever the core provides); the gate directory and its hash-chained event
  log with actor, role, time and payload hash.
- §39.3: the acceptance service, receipts `frob.tenfold.receipt/1`, the submission index by proof kind, direct
  evaluation at sealed probe points, the neutral exchange description, blind submission tokens.
- §39.4: mission cards `frob.tenfold.mission/1`, the development pool (at least six `legged-quad` and six
  `wheeled` missions by M1), the sealed store and key, the six sealing steps as log entries, cluster structure
  in the registration, family retirement, standing assets and R_warm.
- §39.5: freeze manifests `frob.tenfold.freeze/1`, the advocates' tagging wrapper and tag map, arm C and arm W,
  the learner-swap runs.
- §39.6: `frob.tenfold.ledger/1`, the exclusive and resource tables, the 2% closure check.
- §39.7: the estimator, censoring rule, bias-corrected family bootstrap, exact guard with Holm, the exact
  non-inferiority bound for completion and coverage (its exact method is not stated in my subsections), the
  verdict rule, the monitor, the binding pre-analysis record.
- §39.8: the six ceiling calculators and the registration record of a speed comparison.
- §24.12: the study-plan record that `eval.preanalyze` reads. §28.18: the base `report.campaign` schema.
- Appendix O: allocation of the new refusal code. §38.13: the gate runner's receipt. §44: a `funding_reason`
  field on work-package records. §1.2, §1.4: the incumbent capability and absence registers (N00-13, N00-14).

## (e) Contradictions, gaps and what was done

1. **Registries were assigned to nobody.** The brief's registry section is empty and the printed appendices
   file no known answer, counterexample or refusal under §39, although §39.17 says its known answers are
   "numbered in §38". I wrote N39-KA, N39-CX and N39-REF because §39.17 is in this unit, with local labels
   (TF-nn, TF-CX-nn) to be replaced by registry numbers on append. If U39a also wrote `N39-*` registries the
   lead must merge them.
2. **A refusal without a code.** "Suite too small for a population claim" (§39.20) has no code in the refusal
   registry. N39-REF and N39b-34 request a new code in the study block and test on the slug until it exists.
3. **GATE-TENFOLD-02 needs acceptances before a neutral simulator is accepted.** The M1 baseline must stop
   clocks at accepted answers, but adjudicator acceptance (GATE-TENFOLD-05) and the deadlines (OPEN-TENFOLD-5)
   are not due until M3. BD-143 uses a provisional H of 14,400 s and, if no simulator is named, acceptance in
   the rival's CPU reference tier by a separate session, labelled `adjudication: provisional-home`.
4. **Seed BD-317 says two kill gates are "decided", but their decision beads depend on BD-317** (N12-30,
   N25a-18). BD-317 delivers the decision inputs and evaluates the triggers on measured values; N39b-11 hands
   the tables over; the ADRs close in the owners' beads.
5. **The one-receipt pipeline is placed at M2** (N39b-44) although GATE-TENFOLD-13's first milestone is M3,
   because the gate requires it "before any engine-internal work package beyond M2 is funded".
6. **The kit at M3.** §39.14 says every gate's result is published with a kit, but GATE-TENFOLD-08 is an M7
   gate. The scorer (level 1) is at M3 and BD-317's record follows the kit layout; the builder, level 2 and
   the gate script are at M7.
7. **Formulas the text leaves open, fixed as DESIGN in the bead that needs them and labelled so:** the meaning
   of "same accepted scope" in the tie test (N39b-43); the portfolio ratio (N39b-05); the "adjudication
   condition" of the verdict rule under cross-home adjudication (N39b-24); the interval method for r_T and r_C
   and its treatment of an unstable denominator (N39b-43); level-2 sampling and hardware comparability
   (N39b-20); canary construction (N39b-02); agent caps (N39b-31); run-plan balance (N39b-38); the deadline
   rule's handling of censored development runs and its stop (N39b-30); the kit manifest's schema name
   `frob.tenfold.kit/1` (N39b-18; the chapter names five record schemas and none for the manifest); a
   platform-independent normal quantile inside the scorer so that bootstrap indices are bit-stable (N39b-19).
8. **Two arm vocabularies.** §39.5 uses A, B, C (and D); §39.22 uses F, B, W. The beads state both and treat arm
   C as the first form of W, as §39.5 says.
9. **Possible overlap with U39a.** KILL-TENFOLD-ADJ, -REACH and -LEVER are defined in §39.18 (this unit) but
   their mechanisms are in §39.3, §39.7 and §39.8; the report generator and the scorer package statistics that
   U39a's beads define. I wrote the decision beads and the packaging beads and put the mechanisms in `needs`.
10. **Keys of other units used only in `related`** (never in `deps`): BET-10, BET-13, BET-14, BET-19, NS-24,
    N00-13 to N00-20, N00-34, N00-KA, N00-CX, N12-30, N25a-18, N25a-27, N28a-03, N38a-34.
11. **Bead count (53) is above the typical range** because §39.18 and §39.19 alone require 15 decision beads
    (seven kill gates, with LEVER read at two milestones, and eight blocking open questions).
