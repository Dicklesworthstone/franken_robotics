# Coverage of drafting unit U24b (§24.9 to §24.13, and nine seed beads)

36 bead files: 9 seed beads (BD-067, BD-123, BD-126, BD-194, BD-202, BD-254, BD-259, BD-261, BD-383) and 27 new
beads (N24b-01 to N24b-27). No registry bead is owned by this unit (N24-KA is U24a's; N24-CX and N24-REF are
U24c's). No Frontier or Moonshot mechanism, open question or kill gate is defined in §24.9 to §24.13, so the
unit has no pilot, owner-decision or kill-gate bead.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §24.9 | Batteries and the design corpus | N24b-01 (manifest, identity, events; `smoke`, `fixtures`), BD-254 (`standard`, `full`, `hard`, `covering` wiring and accounting), N24b-02 (design-corpus registry, label freeze, held-out third), N24b-23 (end-to-end). Content of the corpus: BD-248 (U28b). Content of `standard`: BD-212 (U16b). Hard-world set: BD-275. Covering selection: BD-278 |
| §24.10 | Fixture 1: a loaded cart crosses a sill and docks | N24b-03 (fixture registry, `task.fixture`, INV-ARENA-04, FROB-REF-530), N24b-04 (analytic layer on the host, derivation files), N24b-05 (bodies, drive, world and rules, requirement, energy), BD-123 (limiting and creep rigs on the referee, both negative controls, GATE-ARENA-04 referee record), N24b-06 (the rule and the pre-analysis, `eval.preanalyze`), N24b-07 (development batch), N24b-08 (mechanism controls; fidelity-dependent results), N24b-09 (batch tiers), N24b-10 (sealed holdout; equivalence registration), N24b-20 (end-to-end) |
| §24.11 | Fixture 2: a quadruped on a split-grip landing | N24b-12 (limiting model, plate-drag rig, controls), N24b-11 (slip metric and its invariance), BD-202 (body, controller, soles, course, pass rule, cases, development set, reflection, GATE-ARENA-06), N24b-13 (holdout, two contact laws, decision rule), N24b-22 (end-to-end) |
| §24.12 | Two study types | N24b-14 (types, admission, four common rules, refusals 552, 532, 533, peeking), N24b-06 (pre-analysis and unreachable verdict), N24b-21 (end-to-end). Engines and `study.*` verbs: BD-124 (U33b) |
| §24.13 | The blinded qualification lane | N24b-15 (campaigns, index, level ledger, `eval.ledger`), N24b-16 (commitment, dry run, 841 to 843), N24b-17 (sealed clusters, 844, close and reveal, retirement), BD-259 (executor, receipt, `eval.qualify`), BD-261 (firewall, 840, audit), N24b-18 (campaign simulator), N24b-19 (standing, 845, campaign section of the bundle), BD-383 (replay by `frob-verify`), N24b-10 (first use on fixture 1), N24b-24 and N24b-25 (end-to-end). Full 500-retry run: BD-260 (U24a) |

Subsections of other units that this unit's seed beads draw on, written completely as the brief requires:

| Subsection | Bead |
|---|---|
| §24.8 Metrics and scorecards (U24a) | BD-254 (metric registry, outcome columns, two denominators), N24b-11 (slip distance) |
| §25.3.2, §25.4.2, §25.24 (`.fmission` schema, quantifier order, admission refusals) | BD-126, N24b-26 |
| §23.3, §23.5, §23.6 rules P-3 and P-7 (time-aware rewards, physical discounting, action laws, requirement terms) | BD-194, N24b-27 |
| §38.3, §38.14, Appendix G.1 (oracles and pins, reference robots) | BD-067 |
| §37.5 (capabilities; discovery cannot read confirmation worlds) | BD-261 |
| §12.4 (`sample-coverage` proof kind; the auditor) | N24b-19, BD-383 |

## (b) Gates and the beads that measure or decide them

| Gate | Milestone | Beads in this unit | Carried elsewhere |
|---|---|---|---|
| GATE-ARENA-04 | M1 referee; M2 batch tiers | BD-123 (referee record), N24b-04 (host rows), N24b-09 (batch-tier rows), N24b-20 (script) | BD-212 counts the batch-tier half in its done-condition |
| GATE-ARENA-05 | M1 | N24b-06 (measures), N24b-21 (script) | BD-124 lists it in its done-condition |
| GATE-ARENA-06 | M2 | BD-202 (record; reflection), N24b-12 (limiting-model rows), N24b-11 (slip invariance), N24b-22 (script) | — |
| GATE-ARENA-07 | M3 | BD-254 (measures and records), N24b-23 (script) | — |
| GATE-QUAL-01 | M3 | N24b-18 (instrument; power half and negative control at host scale), BD-259 (same through the verb), N24b-24 (script) | BD-260 (U24a): the 2,000-campaign, 500-retry record |
| GATE-QUAL-02 | M3 | BD-261, N24b-24 | — |
| GATE-QUAL-03 | M5 | BD-383, N24b-25 | — |
| GATE-SAFE-03 (§37) | M3 | BD-261, N24b-24 | — |
| GATE-AUGUR-01 (§25; admission half) | M1 | BD-126, N24b-26 | BD-136 and the checker beads for the certificate half |
| GATE-CORTEX-06 (reward row), GATE-CORTEX-07 (rows P-3, P-7) | M2 | BD-194, N24b-27 | BD-192, BD-197 for the other rows |
| GATE-GAUNT-05 (§38; these fixtures) | M0, then every exit | BD-067 | The Gauntlet beads for the whole gate |

Gates of chapter 24 that this unit expects other units to carry: GATE-ARENA-01 (BD-201, BD-253; U24a),
GATE-ARENA-02 (BD-235; U19c), GATE-ARENA-03 (BD-121, BD-122; U24a, U24c), GATE-ARENA-08 (BD-195; U24a),
GATE-ARENA-09 (BD-256; U24a), GATE-ARENA-10 to GATE-ARENA-15 and KILL-FOIL, KILL-ITEMBANK, KILL-EXPODOMAIN
(§24.19 to §24.23; U24c), KILL-SPEC-REWARD (BD-257; U24a).

## (c) Counts carried

- Known answers: 27. From chapter 24: KA-598, KA-599, KA-600 and KA-602 to KA-623 (25 rows), each cited with its values in at least one test plan. From other chapters, through seed beads: KA-579 (BD-194), KA-032 (BD-126).
- Counterexamples: 12. From chapter 24: CX-261 to CX-268 (8). From other chapters: CX-384 (BD-259, BD-261), CX-248 (BD-194), CX-045 and CX-075 (BD-126).
- Refusal codes with meaning, trigger, repairs and exit code: 19. Chapter 24: FROB-REF-530, -532, -533, -552, -840, -841, -842, -843, -844, -845 (10). Through seed beads: -868, -874, -877, -884, -887, -847, -846 (BD-126); -527, -554 (BD-194). Cited as used, defined elsewhere: -526, -529, -539, -567, -825.
- Invariants and proof obligations carried by tests: INV-ARENA-03, -04, -05; INV-QUAL-01, -02, -03, -04; PO-ARENA-02, PO-ARENA-03; and through seed beads INV-AUGUR-03, INV-CORTEX-03, INV-GAUNT-01, INV-GAUNT-03, INV-SAFE-03, PO-SAFE-02.
- Decisions stated with reasons and rejected alternatives: ADR-ARENA-07, -08, -09, -10; ADR-SAFE-02; ADR-AUGUR-02, -11, -12; ADR-GAUNT-04; ADR-LEMMA-06; ADR-CORTEX-03.

## (d) Needs from other slices (no seed key could be named)

| Need | Used by |
|---|---|
| §24.18: decision beads for OPEN-ARENA-3 (who holds `qual:admin`; sealed families) and OPEN-ARENA-4 (fixture 2's body) | BD-259, BD-202, N24b-10, N24b-15, N24b-17 |
| §24.20: the task-audit record per `TaskId` that `eval.qualify` requires | BD-259, N24b-16, N24b-10 |
| §24.19: the free-to-yaw run of the plate-drag rig and the wrench-slice rows (GATE-ARENA-10) | N24b-12 |
| §24.7: a settled start state at M1 (the reset cache, BD-195, is M2) | N24b-05, BD-202 |
| §35.9: addressed counter-based draws on the host at M1 (the device kernel, BD-180, is M2); a namespace field in the world address | N24b-07, N24b-17 |
| §29: channel kinds interval extremum and accumulator; chunk-store root handles for a separate confirmation root | N24b-05, N24b-11, BD-261 |
| §18: contact quadrature with weights, pressures and tangential velocities exposed to telemetry; a material boundary inside one footprint; a finer compliant sole law; an alternative contact law selectable per run | N24b-11, N24b-12, N24b-13, N24b-08 |
| §16.8: the frozen fidelity envelope per quantity and tier | N24b-09 |
| §19.14: sealed scene families exposed only through a handle that returns scores | N24b-17 |
| §19: a scene with per-region friction for the low-grip strip | BD-202 |
| §22: a crawl controller for the reference quadruped, frozen by identity; the frozen planner record for sampling controllers | BD-202, N24b-16 |
| §26.5, §26.8, §26.11: the solvability oracle's witness; the hard-world set; the covering selection | BD-254, N24b-01, N24b-23 |
| §26.10, §27.16: planted winners and planted single causes for the corpus | N24b-02 |
| §28.6, §35.3: pairing rules in the comparison engine; what each determinism class licenses and the D3 equivalence record | N24b-14 |
| §28.18, §12.4: report verbs that call the standing check; the `sample-coverage` registry row and `lineage.alpha_spend` | N24b-19 |
| §32.12, §32.13, §33: the MCP tools of the qualify cluster; capability tokens; the qualification ledger on FrankenSQLite | BD-259, BD-261, N24b-15, N24b-24 |
| §23.27, §39: further rules registered under `eval.preanalyze` (cadence of a threshold crossing; the ten-fold study plan) | N24b-06 |
| §25.28, §20, §14: OPEN-AUGUR-1; the seven instants; design families with charts | BD-126 |
| §23.7, §23.15, §23.6: the mapping table; ranges from cards; `train.start` with a registered action law | BD-194, N24b-27 |
| §38.16, §33.7: OPEN-GAUNT-4 (the development oracle); the dependency firewall | BD-067 |

Suggested dependency edits in beads owned by other units (for the lead): BD-124 should depend on N24b-06 (it
calls the enumerator); BD-260 should depend on N24b-18 (it runs the simulator at full scale); BD-248 should
depend on N24b-02 (it populates the corpus registry); BD-212 should depend on N24b-09 (the batch-tier rows of
GATE-ARENA-04).

## (e) Contradictions, gaps and what was done

1. **Overlap with BD-124 (U33b).** Its title covers "the fixed-n exact study with its reachability pre-analysis … `eval.preanalyze`" and its done-condition names GATE-ARENA-05, while the mechanism is in §24.10 and §24.12. This unit wrote the rule and enumerator (N24b-06, carrying the gate) and the admission rules (N24b-14, depending on BD-124) and left the engines and `study.*` verbs to BD-124.
2. **Overlap with BD-248 (U28b).** §24.9's corpus paragraph and BD-248 describe the same corpus. N24b-02 carries the registry, freeze and split mechanics; BD-248 authors the content.
3. **GATE-QUAL-01 is split across units.** The seed for the full run is BD-260 (U24a) although §24.13 is this unit's. N24b-18 specifies the simulator, the adversary and all thresholds so nothing depends on how BD-260 is written.
4. **Milestone mismatches.** Fixture 1 runs at M1 but the reset cache (BD-195) and addressed draws on devices (BD-180) are M2: N24b-05 settles its own state by the cache's criterion and N24b-07 records the host-draw need. The verb table lists `task.settle` at M1 while its seed (BD-255) is M3.
5. **The 1.13% creep-feasible share** is the value of the sharp-lip expression over the whole (μ, δ) distribution, although half of the development batch has filleted lips; the same integral for the filleted stratum gives 1.83% (derived). N24b-04 registers the printed value and records both in the derivation file.
6. **Referee tolerances for several analytic rows** are printed as "GATE-ARENA-04" with no number. BD-123 and N24b-09 propose DESIGN tolerances that are frozen in the fixture manifest before the first run, per ADR-ARENA-10.
7. **Fixture 2 leaves constants open:** what "standing" means numerically, whether "within 12 s and held for 1 s" ends at 12 s or 13 s, the sign convention of strip offsets, and the clocks. BD-202 fixes each as a DESIGN constant recorded in the manifest and says so.
8. **FROB-REF-532 wording differs** between §24.12 ("refused") and §28.6 ("refused unless the request acknowledges it"). N24b-06 implements acknowledgement as removing the verdict from the declared set, which satisfies both.
9. **The worked session uses an absolute claim** ("failure probability at most 0.05", a one-sided exact binomial bound) that §24.10 and §24.12 do not define. N24b-06 adds it as the rule `one-sided-binomial-bound/1` with the session's numbers (0.0432 and 0.0504 at 3 and 4 failures of 200 at level 0.05/2) as known answers.
10. **GATE-SAFE-03's negative control** says a build that leaks per-world outcomes "is detected by the adaptive-retry gate". On fresh clusters a leak does not inflate the error rate, so detection cannot be statistical; N24b-18 implements it as the simulator's receipt-shape assertion, and BD-261 as the sealed-field audit.
11. **Two homes for oracle fixtures** (`frob-conformance/fixtures/` in §38.1 and `tests/fixtures/oracle/` in §34.1). BD-067 uses the repository-layout path for oracle captures and states the choice.
12. **BD-126's "empty or inconsistent domain"** has refusal codes for an empty world family (874) and for predicates in conflict with it (877) and none for an empty design domain; BD-126 reports the last as an invalid file (868) with a pointer.
13. **Three of this unit's seed beads were mapped to chapter 24 by keyword** and belong in substance to other chapters: BD-067 (Gauntlet oracles, §38.3), BD-126 (Augur, §25.3.2), BD-194 (Cortex, §23.3 to §23.5). Each was written completely from its own chapter's text; units U38, U25 and U23 may hold adjacent beads that the lead should check for duplication (in particular a bead for rules P-3 and P-7, and one for `.fmission` admission).
14. **The slip-distance metric** is defined in §24.8 (U24a's subsection) with the rest of the metric registry at M3, but GATE-ARENA-06 needs it at M2; N24b-11 carries it at M2 and BD-254 registers it with the others.
