# Coverage of drafting unit U28c

Chapter §28 (Atelier), subsections §28.16 to §28.26. Output: 49 bead files: 2 seed beads (BD-309, BD-350), 45 new
beads (N28c-01 to N28c-45) and 2 registry beads (N28-CX, N28-REF). The known-answer registry `N28-KA` is written
by unit U28a; this unit's battery N28c-24 wires its rows.

Reading note for the lead. Several subsections of this unit have a seed bead that belongs to another unit
(§28.16 → BD-351, U28a; §28.17 → BD-344, U41; §28.18 → BD-302, U25e, and BD-352, U28a; the eight decision-core
tables DC-02 to DC-08 and DC-11 of §28.19.2 → BD-349, U28a). This unit did not duplicate those beads. It wrote the
parts of each subsection that the seed's one-line done-condition does not carry, the studio-side contracts those
seeds consume, and the conformance and end-to-end beads. Where a row below names a bead of another unit, it is
marked with the unit.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §28.16 | Experiment suggestion | N28c-13 (measurement proposals: partition by decision, witness mechanisms, the A-25 check, the direct-measurement comparator, the model-class statement); N28c-14 (every suggestion registers its prediction); N28c-12 (advisory quantities cannot enter a verdict; FROB-REF-872); N28c-16 (end-to-end script: ranking, STOP, zero value); N28c-17 (KILL-VOI); N28c-09 (the DC-05 table's conformance). Ranking by decision value itself: BD-351 (U28a). Partition machinery and experiment grammar: BD-291 (U25c). |
| §28.17 | Registered predictions | N28c-14 (the studio's own predictions; prediction versions on refit); N28c-15 (calibration put to work: the hint in `next_actions`; the DC-05 trigger); N28c-18 (end-to-end script). Verbs `predict.register` and `predict.report`, the ledger, hash-and-freeze and the scoring rule: BD-344 (U41). |
| §28.18 | Reports | N28c-19 (campaign decision sections: modes, fallbacks fired, stage controls, labels, wording rules, losing-campaign completeness); N28c-20 (end-to-end script on a winning and a losing campaign). Verbs, package layout, statement model, resolution rule, byte determinism: BD-302 (U25e). M4 campaign content inside the cascade: BD-352 (U28a). |
| §28.19 | Decision cores (preamble) | N28c-01 |
| §28.19.1 | The core contract | N28c-01 (record `frob.core/1`, artifact `frob.core-table/1`, taxonomy, three conventions, validation); N28c-02 (compiler, regeneration and diff, FROB-REF-871); N28c-03 (decision evidence records, pinned frontier, shadow → canary → default, forced triggers, replay scenario); N28c-04 (verb `cores.show`) |
| §28.19.2 | The eleven cores | BD-309 (DC-01, DC-09, DC-10: records, known answers, M3 rows of GATE-ATEL-13); N28c-05 (end-to-end script for the M3 rows); N28c-08 (DC-02, DC-03); N28c-09 (DC-05, DC-07); N28c-10 (DC-04, DC-06); N28c-11 (DC-08, DC-11): each states the core's artifacts, loss model, obligations, fallback, triggers, budgeted mode and worked values in full and tests the four clauses of GATE-ATEL-13. Tables for DC-02 to DC-08 and DC-11: BD-349 (U28a). Mechanism-side records already drafted by owners: N16b-05 (DC-01 table generator, U16b), N26a-26 (DC-04 policy, U26a). |
| §28.19.3 | Composition: timescales and interference | N28c-06 (timescale order, single-writer rule, one-way pipelines, static check, period snapshot, the replayed-campaign fixture and replay driver); BD-350 (the nine interference tests; GATE-ATEL-14; PO-ATEL-04 and PO-ATEL-09) |
| §28.19.4 | Methods rejected as ornamental or premature | N28c-07 (all sixteen rejected methods and the four not scored, with reopen conditions); also in the Background of BD-309 and N28c-08 to N28c-11 for the cores concerned |
| §28.20 | Refusals | N28-REF (the fifteen codes with trigger, ranked repairs, exit code, raising mechanism, first milestone); N28c-21 (end-to-end battery: each code 100 of 100 on a planted violation, clean twins accepted) |
| §28.21 | Invariants and proof obligations | N28c-22 (INV-ATEL-01 to -12 as property tests with planted violations); N28c-23 (PO-ATEL-01 to -10: written derivations, acceptance bounds, discharge ledger). Individually also carried by: N28c-03 (INV-ATEL-06), N28c-12 (INV-ATEL-12), BD-350 (INV-ATEL-02, -03, -04, -05, -08; PO-ATEL-04, -09), BD-309 (PO-ATEL-01, -05), N28c-08 (PO-ATEL-03), N28c-11 (PO-ATEL-10) |
| §28.22 | Known answers and negative controls | N28c-24 (the chapter table's twenty-five rows, plus KA-701 and KA-706, as fixtures with caught controls); N28-CX (CX-302 to CX-318 and two late counterexamples). Registry `N28-KA`: U28a. |
| §28.23 | Gates and kill gates | N28c-25 (the nineteen gate rows and eight kill-gate rows in the gate registry, parts by milestone, the method-gate form check, the milestone schedule check); N28c-17 (KILL-VOI); N28c-41 (KILL-BUDGETCURVE); gate measurements in the beads of table (b) |
| §28.24 | Open questions | N28c-26 (OPEN-ATEL-1), N28c-27 (OPEN-ATEL-2), N28c-28 (OPEN-ATEL-3), N28c-29 (OPEN-ATEL-4), N28c-30 (OPEN-ATEL-5), N28c-31 (OPEN-ATEL-6) |
| §28.25 | Controller-budget curves and the controller-limited label (late) | N28c-32 (pilot with falsifiers); N28c-33 (budget coordinate on `ControllerId`, ladders, nested samples); N28c-34 (rows keyed by design, world address, budget; paired queries); N28c-35 (the fit: law, three-point form, pooled γ, saturation index, `curve-unfit`); N28c-36 (calibrated interval; declared-span fallback); N28c-37 (escalation rule; `controller-limited` flag; cascade stage 3; shadow); N28c-38 (`eval.compare --budget-curve`; `explain.compare`); N28c-39 (end-to-end script; GATE-ATEL-17); N28c-40 (GATE-ATEL-18); N28c-41 (KILL-BUDGETCURVE); N28c-12 (J∞ is advisory) |
| §28.26 | The reuse ledger: three columns, never summed (late) | N28c-42 (the ledger: estimand, attribution, cold and warm, identity, never summed); N28c-43 (from-scratch controls: matched-return training control; no-reuse decision control); N28c-44 (`design.diff --reuse`; `report.campaign` reuse section; refusal of an unnamed figure); N28c-45 (frozen lineage of 30 edits; end-to-end script; GATE-ATEL-19) |

## (b) Gates, kill gates and who measures or decides them

Gates carried by this unit (named in a `gates` field here):

| Gate | Beads of this unit | Note |
|---|---|---|
| GATE-ATEL-13 | BD-309, N28c-05 (M3: DC-01, DC-09, DC-10); N28c-02 (regeneration clause); N28c-03 (fallback and shadow clauses); N28c-08, N28c-09, N28c-10, N28c-11 (M4: the other eight); N28c-23 (calibration clause: PO-ATEL-01 to -05, -10) | BD-349 (U28a) owns the M4 tables and its done-condition is this gate |
| GATE-ATEL-14 | BD-350 | |
| GATE-ATEL-15 | N28c-16 (zero-value clause, through the verb); N28c-17 (reads both clauses for the kill decision) | The replay comparison (the 1.5× clause) is measured by BD-351 (U28a) on the log of BD-310 (U28a) |
| GATE-ATEL-16 | N28c-19, N28c-20 (the report clause: every mode, every fallback, the phase ledger); N28c-18 (prediction rows) | The gate as a whole is measured by BD-352 (U28a) |
| GATE-ATEL-17 | N28c-39 | M3 |
| GATE-ATEL-18 | N28c-40 | M4 |
| GATE-ATEL-19 | N28c-45 | M3 |
| KILL-VOI | N28c-17 | BD-351 (U28a) lists "KILL-VOI decided" in its done-condition; N28c-17 is the decision bead |
| KILL-BUDGETCURVE | N28c-32 (first trigger: the pilot), N28c-41 (decision) | |

Gates of the chapter that this unit expects another unit to carry (the mechanism is in their subsections or seed
beads; this unit registers the rows in N28c-25 and measures none of them):

| Gate | Expected carrier |
|---|---|
| GATE-ATEL-01 | BD-250 (U28a) |
| GATE-ATEL-02 | BD-110 (U28a), BD-332 (U28b), and the analysis seeds BD-100 to BD-107 |
| GATE-ATEL-03 | BD-374 (U28b); second arm BD-372 (U32a) |
| GATE-ATEL-04 | BD-262 (U28a, comparison); BD-343 (U28a, bisect) |
| GATE-ATEL-05 | BD-108 (U28a, A-20 to A-23); BD-348 (U28a, the rest) |
| GATE-ATEL-06 | BD-124 (U33b) |
| GATE-ATEL-07 | BD-310 (U28a, pilot); BD-328 (U23b) |
| GATE-ATEL-08 | BD-310 (U28a, pilot); BD-329 (U28a) |
| GATE-ATEL-09 | BD-345 (U28b) |
| GATE-ATEL-10 | BD-346 (U28b) |
| GATE-ATEL-11 | BD-292 (U25a); BD-347 (U28a) |
| GATE-ATEL-12 | BD-111 (U33b); BD-220 (U33a) |
| KILL-STATIC | BD-250 (U28a) |
| KILL-ATLAS | BD-348 (U28a) |
| KILL-LADDER | BD-329 (U28a) |
| KILL-REUSE | BD-347 (U28a) |
| KILL-RANK (in the §28.23 table; not in this unit's brief list) | BD-328 (U23b), BD-265 (U16b) |
| KILL-FDGRAD (consumer row in §28.23) | BD-353 (U17b) |

If a decision bead with label `kill-gate` is wanted for KILL-STATIC, KILL-ATLAS, KILL-LADDER and KILL-REUSE and
neither U28a nor U28b wrote one, N28c-17 and N28c-41 are the templates; N28c-25 already holds each one's trigger,
fallback, milestone and owner.

## (c) Counts

- Known answers carried: 27 (KA-680 to KA-706): the 25 rows of the §28.22 table plus KA-701 and KA-706, all in
  N28c-24; the decision-core and comparison rows also inline in BD-309, N28c-05, N28c-08 to N28c-11, N28c-16,
  N28c-23, N28c-39.
- Counterexamples carried: 17 (CX-302 to CX-318) in N28-CX, plus 2 late counterexamples without catalogue
  numbers (the threshold budget curve; the summed or cold-only reuse figure).
- Refusal codes carried: 15 (FROB-REF-542, -543, -545, -546, -548, -549, -550, -551, -553, -555, -556, -557,
  -870, -871, -872) in N28-REF, each fired by N28c-21.
- Invariants: 12 (INV-ATEL-01 to -12). Proof obligations: 10 (PO-ATEL-01 to -10). Open questions: 6. Decision
  cores: 11. Rejected methods: 16 plus 4 not scored. Gates registered: 19; kill gates registered: 8.
- Architecture decisions restated where they govern: ADR-ATEL-03, -04, -06, -07, -09, -10, -11.

## (d) Needs from other slices

- §16.8: the DC-01 table generator and null self-test (drafted as N16b-05 by U16b), which registers with the
  compiler of N28c-02 and is recorded by BD-309.
- §32.11.3: the daemon's dispatch guard table with hysteresis and its place in the configuration digest
  (BD-296, U32a); §26.11: the incidence matrix, greedy trace and certificate (BD-278, U26b).
- §28.9 and §28.10: the ladder's fixed table, audit lane, conformal survivor rule, exchangeability martingale and
  rank-agreement modes as running code (BD-329, BD-328); the audit-lane specialists are the only admissible
  calibration set for the J∞ interval (N28c-36) and for the escalation predictors (N28c-37).
- §28.13: the polarity table and its rule registry, one derivation and two fixtures per rule (BD-292, BD-347),
  for PO-ATEL-06 in N28c-23; the rerun cost estimates that the reuse ledger's second column reads.
- §28.14: the recompute cost recorded per memo node (BD-111, BD-220), read by the reuse ledger's first column.
- §28.3: a refusal code and an override flag for "a loop stage asked to run on a design the previous stage
  refused" (INV-ATEL-01); the chapter states the invariant and assigns no code (U28a owns §28.3).
- Appendix O: a refusal code for "a reuse figure printed without its column name" (§28.26 states the refusal
  and assigns none).
- Appendix N: catalogue numbers for the two late counterexamples in N28-CX.
- Appendix M: `N28-KA` (U28a); N28c-24 must agree with it number for number.
- §34.9: format-registry rows for `frob.core/1`, `frob.core-table/1`, `frob.core-decision/1`,
  `frob.budget-row/1`, `frob.reuse-ledger/1`, `frob.proposal/1` (DESIGN schema names introduced here).
- §32.12: verb-registry updates at M3 and M4 for `cores.show`, the `--budget-curve` option of `eval.compare`, the
  curves section of `explain.compare`, the `--reuse` option of `design.diff` and its `--run-control` sub-mode
  (BD-312, BD-376).
- §38.4 and §38.5: registration of method experiments and of the budget-curve pilot in the conformance
  registries (U38b); §38.11: the seeded cascade population (BD-377) that N28c-06 and N28c-20 should share.
- §40.8: a decision milestone for KILL-BUDGETCURVE in the register (this unit used M4, with a pilot reading at
  M3).
- §36.14 and §44: an approved compute envelope for GATE-ATEL-18 (about 150 specialist trainings).
- §25.27: the decision of KILL-AUGUR-PROBE at M3, on which the partition path of N28c-13 depends.
- §31: the build-packet verb, for the second trigger of FROB-REF-557 in the refusal battery (M5).
- §30: the reality band on admitted data (BD-393, M5), which re-measures the DC-07 row that N28c-09 measures on
  synthetic data at M4.

## (e) Contradictions, gaps and what was done

1. **Learned-policy budget ladder has zero base effort (§28.25).** The ladder is given as fractions
   {0, 0.05, 0.2}, while the law is written in c(b)/c(b₀) and the closed form needs a constant effort ratio.
   With c(b₀) = 0 the law is undefined, and the three levels are not in a constant ratio. Done: N28c-33 registers
   the ladder (so rows are keyed and paired) and marks it `fit: unavailable`; N28c-35 refuses such fits
   (`effort-undefined`); N28c-37 treats them as unsaturated, so the rule reduces to the fixed table. An owner
   decision on a positive effort scale for the zero-shot level is listed as a need.
2. **A class-shared γ and a per-design closed form (§28.25).** The law says γ is shared by the class; the
   three-point form and the worked case (0.726 and 0.888) imply a per-design rate (γ_A = 1.085, γ_B = 0.350 at
   an effort ratio of 4). Done: N28c-35 implements both estimators, reports the three-point value on
   constant-ratio ladders (the gate's worked case requires it) and the pooled-γ value beside it, and names the
   estimator in every output.
3. **The escalation rule needs an interval before three budgets exist (§28.25).** "Every candidate runs at b₀"
   and "a design moves to the next budget when the upper end of its interval for J∞ reaches the k-th best", with
   a cost illustration in which 70% of designs stop at b₀, but J∞ is only defined from three budgets. Done:
   N28c-37 specifies prefix predictors (a calibrated gap margin at b₀; a two-point estimate with the class γ̂ at
   b₁), each with its own conformal calibration, labelled DESIGN.
4. **"Reduces to the fixed table" (§28.25).** With the declared span as interval every upper end reaches the
   k-th best, which read literally escalates everything. Done: read as "the ladder's fixed halving table decides
   who moves up" (N28c-36, N28c-37), which is the stated fallback of every adaptive rule in the chapter.
5. **GATE-ATEL-18 and KILL-BUDGETCURVE leave a band.** The gate needs regret at most half; the kill gate fires
   below a 30% reduction. Between 30% and 50% the gate fails and the kill gate does not fire. Done: N28c-41
   states the outcome for the band (rule stays in shadow, gate `fail`, one repair cycle, re-read at the next
   exit).
6. **Late gates are outside the main register tables.** GATE-ATEL-17, -18, -19 and KILL-BUDGETCURVE appear only
   in the late index of §40; KILL-BUDGETCURVE has no decision milestone. Done: M3, M4, M3 as indexed; M4 for the
   kill gate with a pilot reading at M3 (N28c-25, N28c-41).
7. **Two claims on one gate.** The chapter gives GATE-ATEL-12 "Known answer, Performance" (the register says
   Performance) and GATE-ATEL-19 "Known answer; Performance". The register allows one claim per gate. Done:
   N28c-25 follows the register for -12 and takes Known answer as the governing part for -19, with the timing
   columns under the performance disclosure rules.
8. **OPEN-ATEL-1 blocks a gate part due at M3.** The DC-01 thresholds derive from undecided constants. Done:
   BD-309 depends on the decision bead N28c-26; with the question open the DC-01 row is
   `blocked: OPEN-ATEL-1`, the fixed 30-pair fallback acts, and the DC-09 and DC-10 rows proceed.
9. **Gate at M4, mechanism at M5 (DC-07).** GATE-ATEL-13 puts DC-07's row at M4; the reality band lands at M5.
   Done: N28c-09 measures the DC-07 automaton on `synthetic` calibration data at M4 and lists the M5
   re-measurement as a need.
10. **Small numeric inconsistencies in the plan.** (a) The 5th percentile of conformal coverage at n = 39 is
    printed as 0.885 (§28.9, PO-ATEL-03) and 0.884 (worked card); the exact value is 0.8840; tests use
    0.884 ± 0.003. (b) The preempt-resume wait of the DC-09 worked queue is printed as 0.2 s (§28.19, KA-703) and
    0.17 s (daemon section); exact 0.1724 s; tests use 0.17 with a 0.5% tolerance on the exact rational. (c) The
    DC-04 table formula ⌈ln(1/δ)/p*⌉ gives 29,958 at p* = 10⁻⁴ while the exact binomial count printed elsewhere
    is 29,956; both are stated in N28c-10. (d) The §28.22 table has 25 rows, while Appendix M has 27 for the
    chapter (KA-701, KA-706 extra); N28c-24 carries all 27.
11. **Refusal without a code (§28.26).** "A reuse figure printed without its column name is refused in a
    report": no code in Appendix O. Done: behaviour fully specified in N28c-44 with exit 3; code listed as a
    need in N28-REF and N28c-44.
12. **Invariant without a refusal (INV-ATEL-01).** No code is assigned for running a stage on a design the
    previous stage refused. Done: N28c-22 tests the behaviour and accepts whichever registered code the loop's
    owner raises; listed as a need.
13. **Where FROB-REF-871 is an outright refusal.** The chapter says the fallback is in force meanwhile, so a
    governed verb normally completes. Done (DESIGN): a normal call completes on the fallback with the code in
    `warnings` and in the decision record; a request that pins the rule (`--require-rule table`) is refused with
    exit 3; `cores.show` reports a finding with exit 2 (N28c-02, N28c-04, N28c-21).
14. **Design completions labelled DESIGN in the beads**, for the lead to confirm: the canary share (one in
    eight by address hash) and the standing shadow sample (one in 64) in N28c-03; the band rule that reproduces
    "65 to 92% over 30" as hits ≤ 19 or ≥ 28 and extends it to other confidence levels in N28c-15; buying a
    proposal with `--from-proposal` as the moment a suggestion's prediction is frozen in N28c-14; the paired
    lower-bound reading of GATE-ATEL-15's "1.5 times better" in N28c-17; the oracle as the larger of the
    specialist mean and the 8,192-sample planner in N28c-40; single attribution with precedence exact, carried,
    reacquisition in N28c-42; the edit-class tagging rule in N28c-42; the two-standard-error definition of
    "paired noise" in N28c-34 and N28c-35.
15. **Not done on purpose.** No bead was written for the DC-02 to DC-08 and DC-11 tables themselves (BD-349), for
    the `experiment.suggest` ranking (BD-351), for the prediction verbs (BD-344) or for the report verbs
    (BD-302): each belongs to another unit's seed bead. If any of those was drafted thinly, the full rule text
    is available in N28c-08 to N28c-11, N28c-13 to N28c-16 and N28c-19.

## Self-check

49 bead files; every front matter parses (fields present, lists valid JSON); keys unique and equal to file names;
every `deps` and `parent` key is a seed key, a reserved key or a key of this unit; no dependency cycle among this
unit's beads; no bead depends on a bead of this unit at a later milestone; every bead with label `e2e` depends on
`STD-E2E`; both seed beads have files and keep their seed dependencies (BD-309: BD-263, BD-278, BD-296; BD-350:
BD-349); every subsection of the brief appears in table (a); no body uses a section pointer in place of content
(section numbers appear only in the front-matter fields `plan_refs` and `needs` and in References lines). By milestone: M1: 5; M3: 21; M4: 23.
Largest bead: N28c-01 (263 lines); smallest: N28c-29 (115 lines).
