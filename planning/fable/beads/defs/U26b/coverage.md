# Coverage of drafting unit U26b

Chapter §26 Nemesis (adversarial worlds and rare failures): subsections §26.10 to §26.16. Three seed beads
(BD-271, BD-274, BD-278), twenty-four new beads (N26b-01 to N26b-24) and the chapter's counterexample and refusal
registries (N26-CX, N26-REF). The known-answer registry of the chapter (N26-KA) is U26a's; the battery that runs
its eleven rows is here (N26b-17).

Unit U26a had already written the component beads of two of this unit's seeds (the oracle's necessary
conditions and witness search; the shrinker's test relation, search and file), their end-to-end scripts, the two
pilots and the three kill-gate decisions. The seed beads here are therefore written as the integrating beads
(pipeline, verb, record, what the gate reads) and name those component beads as prerequisites in `needs`, with
their keys, since a sibling unit's key may not appear in `deps`. Section (d) lists the links the lead should
turn into dependencies.

## (a) Subsections and the beads that carry them

| Subsection | Title | Content | Beads |
|---|---|---|---|
| §26.10 | Auditing what screening discarded [S] | Why survivors say nothing about discards; ADR-NEMESIS-07 with reason and rejected alternatives | N26b-01 |
| §26.10 | | Sampling discarded strata: strata by reason, design family and score band; the keyed draw before any score; 10% of the stage's spend; fixed sample size at the next stage; never cancelled by a race | N26b-01 |
| §26.10 | | The false-discard rate per stratum with the exact binomial bound 1 − δ^(1/m) (9.5% at m = 30, 5.0% at m = 59); the approximation ln(1/δ)/m not used; the tolerance verdict; the stage-level bound | N26b-02 |
| §26.10 | | Planting a winner: rank known from a full evaluation, poorly represented by the screen, blinded identity, must survive every screen; FROB-REF-566 | N26b-03 |
| §26.10 | | The label `screen-biased`; bypass of the stage for the affected strata until re-qualified; the audit record | N26b-04 |
| §26.10 | | The audit lane as the only calibration source for rank agreement between rungs | N26b-04 (export), N26b-01 (pre-sample) |
| §26.10 | | The verb `nemesis.audit`; "the cascade applies it stage by stage" | N26b-05; the application is seed BD-345 (U28b) |
| §26.10 | | Planted screens and synthetic campaigns; GATE-NEMESIS-07 measured | N26b-06, N26b-07 |
| §26.11 | Covering scene sets by submodular selection [S] | The decision and its contract block (decision, loss, obligation, failure modes, fallback and trigger, budgeted mode, interference, verification), carried verbatim in substance | BD-278 |
| §26.11 | | Incidence: scenes against failure modes over a reference set, after the guard; declared weights frequency × severity × designs affected | N26b-08 |
| §26.11 | | Objective f(S), monotone and submodular; lazy greedy; the cost-benefit variant by gain per second | N26b-09 |
| §26.11 | | Certificate OPT_k ≤ f(S) + the k largest remaining gains (PO-NEMESIS-03); the (1 − 1/e) guarantee; the worked instance (86, 98, 88%, 63%) | N26b-10 |
| §26.11 | | Held-out slots, one in five; fallback to the standard battery and its trigger; "any greedy prefix is a valid battery"; a bound refuses battery-selected episodes; the DC-10 decision record | N26b-11 |
| §26.11 | | The verb `nemesis.cover`; FROB-REF-565; GATE-NEMESIS-08 measured | BD-278, N26b-12 |
| §26.12 | Verbs and refusals | The verb table (seven verbs: kinds, first milestones, notes) as registry rows, schemas, clusters | N26b-13 |
| §26.12 | | `nemesis.oracle` | BD-271 |
| §26.12 | | `nemesis.shrink` | BD-274 |
| §26.12 | | `nemesis.cover` | BD-278 |
| §26.12 | | `nemesis.audit` | N26b-05 |
| §26.12 | | `nemesis.search`, `nemesis.bound`, `nemesis.hardset` | Seeds BD-273 and BD-276 (U26a), BD-275 (U15a); rows in N26b-13 |
| §26.12 | | The refusal table (twelve codes) with triggers, envelope fields, ranked repairs, exit codes | N26-REF |
| §26.12 | | Every refusal exercised from the user's side with an admissible twin | N26b-14 (eleven codes), N26b-07 (FROB-REF-566) |
| §26.13 | Invariants and proof obligations | INV-NEMESIS-01 to -06: binding ledger, type gates, crate-wide tests, mutants | N26b-15; also BD-271 (INV-01, -05), BD-274 (INV-01, -04) |
| §26.13 | | PO-NEMESIS-01 to -04: test bindings | N26b-15; PO-03 in N26b-10; PO-01 named in BD-274 |
| §26.13 | | The arguments "on file in `docs/derivations/`"; the candidate theorem card for PO-NEMESIS-04 | N26b-16 |
| §26.14 | Known answers | The eleven rows with expected values and negative controls, as one battery and one runner | N26b-17 |
| §26.14 | | Rows of this unit's subsections: screening audit; coverage certificate | N26b-02, N26b-06, N26b-07 (KA-668); N26b-10, BD-278, N26b-12 (KA-669) |
| §26.14 | | The controls as registered counterexamples and mutants | N26-CX |
| §26.15 | Gates, kill gates and pilots | The gate table (eight gates) and the three kill gates as registry rows with full thresholds, triggers, fallbacks | N26b-18 |
| §26.15 | | "Pilots before funding": both pilots with design, falsifier, cost, ceiling | N26b-18 (register rows); the pilot beads are N26a-06 and N26a-28 (U26a) |
| §26.15 | | Gates measured by this unit | See (b) |
| §26.16 | Open questions | OPEN-NEMESIS-1 (owner decision, with OPEN-HAB-1) | N26b-19 |
| §26.16 | | OPEN-NEMESIS-2 (experiment; blocks GATE-NEMESIS-05) | N26b-20 |
| §26.16 | | OPEN-NEMESIS-3 (owner decision; the 10⁻⁶ envelope) | N26b-21 |
| §26.16 | | OPEN-NEMESIS-4 (airlock maintainer; sample caps, no upstream splitting; FD-FSIM-18, FD-FSIM-19) | N26b-22 (decision, admission tests, upstream rows), N26b-24 (switch-over) |
| §26.16 | | OPEN-NEMESIS-5 (Arena lead; carrying a hard-world set) | N26b-23; the default's first half is in BD-275 (U15a) |

Seed beads drawing on subsections of U26a: BD-271 (§26.5: classes, three rules, hand-offs, verb) and BD-274
(§26.7: input, relation, algorithm, minimality, the file, re-promotion of the shrunk world, verb) are written
completely, with the component mechanisms left to the U26a beads named in their `needs`.

## (b) Gates, kill gates and admission gates

| Gate | Milestone | Beads of this unit that measure or decide it | Expected from another unit |
|---|---|---|---|
| GATE-NEMESIS-01 | M3 | BD-271 (oracle rows: crate-level gate test; seed done-condition) | Legality row and the combined record: BD-270, N26a-05, N26a-33 (U26a) |
| GATE-NEMESIS-02 | M3 | Registry row only (N26b-18) | BD-273, N26a-11 (U26a) |
| GATE-NEMESIS-03 | M3 | BD-274 (what the gate reads; seed done-condition) | Script and record: N26a-34 (U26a) |
| GATE-NEMESIS-04 | M3 | Registry row only (N26b-18); re-run at an upstream switch-over (N26b-24) | BD-276, N26a-30 (U26a) |
| GATE-NEMESIS-05 | M3 | Registry row only (N26b-18); the open question that blocks it (N26b-20); re-run at an upstream switch-over (N26b-24) | BD-277, N26a-31 (U26a) |
| GATE-NEMESIS-06 | M3 | Registry row only (N26b-18) | BD-272, N26a-16 (U26a); BD-275 (U15a) |
| GATE-NEMESIS-07 | M4 | N26b-05 (crate-level gate test), N26b-07 (script and record), N26b-06 (fixtures) | BD-345 (U28b) lists it beside GATE-ATEL-09 for the cascade's application |
| GATE-NEMESIS-08 | M3 | BD-278 (crate-level gate test; seed done-condition), N26b-12 (script and record), N26b-10 (oracle and instances) | — |
| KILL-NEMESIS | M3 | Registry row only (N26b-18) | N26a-12 (decision), BD-273, N26a-11 (U26a) |
| KILL-RARE | M3 | Registry row only (N26b-18) | N26a-32 (decision), N26a-28, BD-277, N26a-31 (U26a) |
| KILL-MCW | M3 | BD-274 (the two measured shares it reads; the mode switch it sets) | N26a-35 (decision), N26a-34 (U26a) |

All eleven are rows of N26b-18. The chapter defines no admission gate. Gates of other chapters touched:
GATE-ATEL-09 (the cascade does not trust a rung whose audit is missing or failed; N26b-04 supplies the record and
the bypass query, BD-345 measures), GATE-ATEL-13 (decision-core tables; N26b-11 supplies DC-10's record, BD-309
registers), GATE-GAUNT-01 (every gate has a registered negative control; N26-CX, N26b-15, N26b-06, N26b-10),
GATE-GAUNT-08 (pilot records; N26b-18's two rows), GATE-TETHER-02 and -04 (surface parity; one exit code per
refusal; N26b-13, N26-REF, N26b-14).

## (c) Counts

- Bead files: 29. Seed beads: 3 (BD-271, BD-274, BD-278). New beads: 26 (N26b-01 to N26b-24, N26-CX, N26-REF).
- Subsections: 7 (§26.10 to §26.16), all in table (a).
- Gates: 11 of 11 registered (N26b-18). Measured or decided by a bead of this unit (named in its `gates`
  field): 5 (GATE-NEMESIS-01, -03, -07, -08; KILL-MCW). Carried by U26a: 6 (GATE-NEMESIS-02, -04, -05, -06;
  KILL-NEMESIS, KILL-RARE), plus the scripts and decisions for -01, -03 and KILL-MCW.
- Known answers of §26 (KA-659 to KA-669, 11 rows): all 11 in the battery N26b-17 with values and controls;
  KA-668 and KA-669 carried with tests in this unit's mechanism beads. Others cited with values: KA-704, KA-706
  (the cascade's twins), KA-136, KA-138, KA-139, KA-141, KA-142, KA-143, KA-144 (constructions).
- Counterexamples of §26 (CX-288 to CX-295, 8 rows): all 8 in N26-CX with shortcut, case, rule, mutant and
  catching test; 3 construction entries cross-referenced (CX-065, -066, -067); 6 chapter-local controls entered
  as provisional rows NC-26-01 to -06.
- Refusal codes of §26: 12 of 12 in N26-REF (360, 361, 362, 560, 563, 564, 565, 566, 567, 568, 569, 760), each
  with meaning, trigger, envelope fields, ranked repairs, retry class and exit code; 565 and 566 are raised by
  this unit's beads (BD-278; N26b-03 with N26b-05), 564 at the verb by BD-274. Codes of other chapters named:
  840, 549, 546, 545, 312, 724, 761.
- Invariants: INV-NEMESIS-01 to -06 (6), all in the ledger N26b-15. Proof obligations: PO-NEMESIS-01 to -04
  (4), all bound in N26b-15 with derivations in N26b-16.
- Architecture decisions stated with reason and rejected alternatives: ADR-NEMESIS-02 (BD-271), -04 (BD-274),
  -07 (N26b-01), -01 (N26b-19); -03, -05, -06 named where they bind (N26b-15, N26b-11).
- Open questions: OPEN-NEMESIS-1 to -5, one bead each (two owner decisions, one experiment, one maintainer
  decision with a separate switch-over bead, one Arena decision with its mechanism). Also placed: OPEN-ATEL-5
  (N26b-03).
- End-to-end script beads: 4 (N26b-07, N26b-12, N26b-14, N26b-17); further scripts specified inside decision
  beads (N26b-19 to N26b-24). Planted-fixture beads: 2 (N26b-06; the instance generator and oracle in N26b-10).
- Risks named: RISK-37 (N26b-01, N26b-04, N26b-05), RISK-15 (BD-271, N26b-11). Bets: BET-22, BET-32, BET-05,
  BET-17, BET-31. Constructions: C-26, C-27, C-28 (SPEC-C-26 and SPEC-C-28 are dependencies).
- Largest bead: BD-271, 226 lines. Smallest: N26-CX, 129 lines.

## (d) Needs from other slices

Links the lead should turn into dependencies (each is a `needs` entry that names the key):

| This unit's bead | Should depend on (other unit's bead) |
|---|---|
| BD-271 | N26a-13, N26a-14, N26a-01, N26a-02; N26a-04 for its gate test (U26a) |
| BD-274 | N26a-06 (its pilot), N26a-17, N26a-18, N26a-19, N26a-03 (U26a) |
| BD-278, N26b-08 | N26a-27, N26a-01 (U26a) |
| N26b-01 | N26a-01 (U26a) |
| N26b-11, N26b-12 | N24b-01 (U24b); N26a-01, N26a-23 (U26a) |
| N26b-14 | N26a-04, N26a-15 (U26a) |
| N26b-15 | N26a-01, N26a-19, N26a-23 (U26a) |
| N26b-17 | N26-KA, N26a-27, N26a-04, N26a-15, N26a-34 (U26a) |
| N26b-19 | N19c-30 (U19c), decided together |
| N26b-20 | N26a-29, N26a-28 (U26a) |
| N26b-21, N26b-22 | N26a-21, N26a-22 (U26a) |
| N26b-23 | N26a-07, N26a-19 (U26a) |
| N26b-24 | N26a-30, N26a-31 (U26a); the reserved upstream beads FD-FSIM-18 and FD-FSIM-19 (in `deps`) |

Links in the other direction: BD-345 (U28b) should depend on N26b-05 (the verb) and N26b-04 (record and bypass
query) and keep GATE-ATEL-09 and the application to the cascade; if U28b wrote its own lane, bound, plant or
GATE-NEMESIS-07 script they duplicate N26b-01 to N26b-07 and should be merged. BD-309 (U28c) should read the
DC-10 record of N26b-11. BD-275 (U15a) and N26a-33, N26a-34, N26a-35 (U26a) already depend on the seeds here.

Prerequisites with no known key:

- Tether (§32.12, §32.18, §32.4): the task-cluster table's ruling on `nemesis_cover`; registration of
  `nemesis_audit` in `ladder`; the exit code of the `76x` refusal codes (N26b-13, N26b-05, N26-REF).
- Atelier (§28.9 to §28.11, §28.18): a screening stage's decision record and the campaign manifest hash; the
  final-stage rank record of a design; dispatch of a next-stage evaluation at fixed n on the survivors'
  addresses; the rank-agreement ledger's intake of calibration rows; `report.campaign` printing the audit
  (N26b-01, N26b-03, N26b-04, N26b-05).
- Workspace (§35.9): registration of the streams `nemesis-audit`, `nemesis-cover-heldout`, and the fixture
  streams `nemesis-audit-fixtures`, `nemesis-cover-gate` (N26b-01, N26b-11, N26b-06, N26b-10).
- Gauntlet (§38.4, §38.5, §38.15, Appendix N): the method-experiment registration; the pilot register;
  counterexample numbers for the six provisional controls (N26b-20, N26b-18, N26-CX).
- Lemma (§12.3): a ruling on the candidate theorem card for the zero-failure bound, with a number and proof kind
  if admitted (N26b-16).
- Performance (§36.14): the envelope classes and the record of the class in force (N26b-21).
- Airlock (§33.3, §33.6, §33.8): the `stats` group's admission record; the compatibility train (N26b-22,
  N26b-24).
- Upstream beads FD-FSIM-18 and FD-FSIM-19 (reserved keys; not written here). The five fields each needs for
  `registries/upstream.toml`, including the reproducible failing fixture, are specified in N26b-22.

## (e) Contradictions and things that could not be turned into work as written

1. **"Before any score exists" against strata that need scores.** §26.10 stratifies discards by reason and score
   band and draws the lane before any score; §28.9 draws the lane from the whole population, §26.10 from the
   discards. Done: every design gets a keyed priority before scores; a pre-sample (priority below M/N) is the
   whole-population lane; strata are formed after the screen and filled with their smallest priorities, of
   which the pre-sampled discards are a prefix (N26b-01). Calibration rows record how each design entered
   (N26b-04).
2. **"An audited miss rate above the declared tolerance"**: the measured rate or its bound? Reading the bound
   labels every small stratum biased (30 clean audits bound the rate at 9.5%); reading the rate calls 30 clean
   audits a pass. Done: a three-valued verdict, `screen-biased` on the measured rate, `passed` only when the
   bound is also within tolerance, `bound-above-tolerance` otherwise (N26b-02).
3. **A tenth of a cheap stage's spend may buy no audit at all.** In the plan's own ladder example a tenth of the
   first rung is 81 s against a 3-minute evaluation. Done: the lane size is computed and reported; a stratum
   under a minimum is merged upward; a stage with a weak audit is reported as such, never as audited (N26b-01).
4. **Undefined terms in §26.10**: "top-k design" among audited discards, "re-qualified", "blinded identity" for
   a content-addressed design, "represents poorly". Done: a predicate on paired next-stage scores (N26b-02); a
   recorded re-qualification event with its conditions (N26b-04); blinded as unlabelled and sealed by a
   commitment; a required declaration of the weakness (N26b-03).
5. **One verb, two moments.** `nemesis.audit` must act before and after the stage's scores. Done: phases
   `prepare` and `run`, and an offline form on a stage record so that synthetic and replayed campaigns are
   audited by the real binary (N26b-05). A stage without a pre-score commitment has no refusal code: it is a
   completed `not-run` result.
6. **GATE-NEMESIS-07's coverage clause** ("covers in at least 95% of 1,000 campaigns") is a coin toss for a
   bound calibrated at exactly 95%, and "the stratified bound" is not defined. Done: both a per-stratum share
   and a stage-level stratified bound are measured; the synthetic study's exact expected coverage (0.9952 per
   stratum row) is computed by enumeration and frozen before measurement (N26b-06, N26b-07).
7. **Seed BD-345 (U28b) names GATE-NEMESIS-07**, while §26.10 says this chapter owns the audit, its verb and its
   gate. Done: mechanism, verb, fixtures and gate script are here; BD-345 is treated as the cascade's
   application. See (d).
8. **`nemesis.cover` is kind `read` but its product is a battery.** Done: the verb returns a selection document
   and writes nothing; adoption is a membership event of Arena's battery mechanism citing the selection
   (BD-278, N26b-11).
9. **`nemesis_cover` has no MCP task cluster**, and `attack` and `ladder` are full at seven. Done: proposed for
   the `audit` cluster, recorded as a proposal (N26b-13).
10. **Mode weights.** "Frequency × severity × number of designs affected": severity has no table anywhere;
    frequency of a mode found by search is not a nominal frequency. Done: integer weights F × V × D, a declared
    severity table defaulting to 1, an `exploration_only` mark (N26b-08).
11. **"One slot in five" and "held-out scenes"** are not defined. Done: slots 5, 10, 15, … (so every prefix keeps
    the rule; fewer than five slots has none and cannot be adopted); the pool is the held-out side of the scene
    split minus the incidence's rows; unfilled held-out slots stay empty (N26b-11).
12. **DC-10's owner** is the evaluation crate in the decision-core table, while the mechanism and verb are in
    `frob-nemesis`. Done: the decision record is written where the selection is made; BD-309 registers the
    table (N26b-11).
13. **The worked coverage instance has only aggregate numbers.** Done: a concrete instance of 40 modes and 18
    scenes was constructed that reproduces all of them (gains 31, 22, 14, 9, 6, 4; remaining top six 12; 86; 98)
    and has a true optimum of 90, so that a certificate that is too small can be caught; its further numbers
    (prefix certificates 53, 77, 90, 91, 94, 98; 36 against 93 gain evaluations; 99 from stale gains; 132 from
    double counting) are derived values of this unit (N26b-10). They need known-answer numbers if registered.
14. **KA-704's control, "a certificate computed from stale gains"**, describes a bound that is still valid (a
    stale gain is at least the true gain), only looser. Done: caught as a known-answer mismatch (99 for 98);
    two mutants that give an invalid bound were added (N26b-10).
15. **"The constant of the cost-benefit variant"** is not stated. Done: the better of ratio greedy and gain
    greedy, with ½·(1 − 1/e) labelled as a FACT from the literature to be re-read before a gate cites it; a
    budget form of the certificate by the fractional-knapsack relaxation, which the plan does not give (N26b-09,
    N26b-10).
16. **Exit code of FROB-REF-760**: `76x` is in neither row of the exit table. Done: exit 4, flagged for the
    registry owner (N26-REF). Same finding as U26a's.
17. **Refusal-like situations with no code** (a failed hard-set admission or a battery of hard worlds only; a
    bound for a non-finalist; an audit without a commitment; shrinking without a witness; a failed certificate
    check). Done: five rows with present behaviour; free numbers noted; none assigned (N26-REF).
18. **Two codes for one rule**: FROB-REF-560 (Nemesis, at the bound) and FROB-REF-549 (Atelier, at the cascade)
    both refuse battery-selected or search-selected episodes for a bound. Done: kept distinct and
    cross-referenced (N26-REF, N26b-11).
19. **PO-NEMESIS-04 is "a candidate theorem card"** with no card number (cards run T-01 to T-35). Done: the
    candidate is written with two checkable routes and handed to Lemma; class `Tested` meanwhile. The three
    formulas for the zero-failure count (29,956 exact; 29,958 sufficient; 29,957 rounded down) are tabulated
    (N26b-16).
20. **Appendix F's switch-over rule for the sample caps** names the exploration gate, though the caps also touch
    the certification sequence. Done: both gates are re-run at the switch-over (N26b-24).
21. **OPEN-NEMESIS-5 blocks nothing**, yet half of its default ("worlds are re-promoted before they enter
    another") is work no bead carried. Done: `nemesis.hardset carry` (N26b-23), a sub-command the plan's verb
    note does not list.
22. **The shrunk world's witness and promotion.** §26.7 puts a witness and a referee promotion in the file but
    the test relation does not ask for either on sub-worlds; a witness of the full world need not hold for a
    smaller one. Done: the shrunk world is promoted and witnessed again before the file is written, with a
    bounded resume when either is lost (BD-274). A failure whose world has no witness is `not-shrunk`.
23. **Seed done-conditions that name gates and a kill decision** whose scripts and decision bead depend on the
    seed (BD-271, BD-274). Done: the seeds own the crate-level gate tests and "what the gate reads"; the records
    are written by the scripts; the kill gate is decided by its decision bead.
24. **Surfaces added as DESIGN** (flagged in N26b-13 for the verb-registry owner): `nemesis.audit prepare|run`
    with `--stage-record`, `--outcomes`, `--reveal`; `nemesis.cover --incidence`, `--algorithm`, `--greedy-only`,
    `--prefixes`; `nemesis.search --distress <score>` (N26b-20); `nemesis.hardset carry [--refresh]`. Schemas
    named here: `frob.nemesis.oracle/1`, `frob.nemesis.shrink/1`, `frob.nemesis.cover/1`,
    `frob.nemesis.incidence/1`, `frob.nemesis.audit/1`, `frob.nemesis.stage/1`. Registries named here:
    `registries/nemesis/severity.toml`, `registries/nemesis/distress_scores.toml`.
