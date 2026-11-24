# Coverage of drafting unit U30b

Chapter §30 (Ground: calibration against reality), subsections §30.15 to §30.24, and the ten seed beads assigned
to this unit. Output: 52 bead files: 10 seed beads (BD-139, BD-388, BD-389, BD-391 to BD-397), 40 new beads
(N30b-01 to N30b-40) and 2 registry beads (N30-CX, N30-REF). The known-answer registry `N30-KA` is written by
unit U30a; every bead here cites the KA numbers it uses and states their values inline.

**Reading note for the lead.** All ten seed beads of this unit have their best-matching subsection in §30.22,
but their mechanisms are specified in subsections that U30a owns (§30.2, §30.7, §30.9 to §30.14). The brief says
to write such seed beads completely, so this unit read §30.1 to §30.14 in full and wrote the mechanisms. Where a
mechanism was too large for one bead it was split into the seed bead and N30b sub-beads. The table at the end of
section (a) lists those U30a-owned subsections and the beads here that carry them, so that any bead U30a writes
for the same text can be reconciled. Two seed beads of the chapter are not this unit's: BD-390 (`ground.excite`,
§30.8, U30a) and BD-311 (published anchor, §30.2, U30a); BD-387 (`ground.ingest`, §30.5) is written by U25a and
exists on disk.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §30.15 | The decision model for reality calibration [F] | N30b-08 (the core DC-07 as its owner specifies it: states, actions, loss, alarm level α_d = c_recal/C_stale, the four artifacts, the guard table (n, level, draw), the automaton, obligations, failure modes, fallback triggers, budgeted mode, timescale, evidence records, adoption); N30b-09 (buying real episodes: EVSI with the reality-gap variance, the substituted case 0.0251 / 0.0084 / 0.0035, the experiment-table rows, the 30-episode cap, "never which conditions", the interference test); N30b-34 (end-to-end walk of the automaton and the planner rows); OPEN-GROUND-4 in the Considerations of N30b-08 and N30b-09 |
| §30.16 | Verbs | N30b-13 (the seven registry entries: kinds, milestones, schemas, capabilities, corrected refusal lists, long-verb behaviour, the `--retire` parameter set, MCP names and task clusters, the limit of seven). Handlers: BD-391 (`ground.identify`), BD-392 (`ground.predict`), N30b-06 (`ground.score`), BD-393 (`ground.band`), BD-396 and N30b-12 (`ground.kit`); `ground.ingest` is BD-387 (U25a) with clock fitting in BD-389 and the scan form in BD-395; `ground.excite` is BD-390 (U30a), extended here by N30b-29 |
| §30.17 | Bench work packages (optional, outside the critical path) | N30b-38 (packages A and B, and the bench protocol format `frob.ground.protocol/1`); N30b-39 (packages C and D); N30b-40 (packages E and F); N30b-26 (the owner decision on which, if any, is funded). Parent of all four: WP-69 |
| §30.18 | The replication kit [F] | BD-396 (contents, layout, lineage selection, the privacy and licence screen with FROB-REF-899 and no override, dataset-or-protocol substitution, expected outputs, embedded rehearsal, losing results); N30b-12 (the self-test on a clean machine in classes D2 and D3); N30b-36 (end-to-end script); N30b-21 (KILL-KIT) |
| §30.19 | Refusals | N30-REF (the eleven codes with trigger, payload, ranked repairs, exit code, raising mechanism, per-verb lists, the map from the stale 840-block numbers); N30b-37 (end-to-end battery: each code on at least three planted violations with a clean twin, on both surfaces); N30b-01 (the fifty misuses, which exercise 873, 889, 895, 897, 814 at M1) |
| §30.20 | Invariants and proof obligations | N30b-23 (INV-GROUND-01 to -07 as property tests with planted violations; the model-based sequence test; the dependency firewall for INV-GROUND-07); N30b-24 (PO-GROUND-01 to -03: derivations on file, Monte Carlo discharge with acceptance bounds, discharge records). Individually also carried by: BD-139 (INV-01, -06), N30b-04 (INV-02, -05), BD-392 and N30b-06 (INV-03, -04), BD-388 (INV-06), BD-389 (PO-02), BD-393 (PO-01), BD-394 (PO-03) |
| §30.21 | Known answers: the synthetic rehearsal | BD-388 (truth simulator, fifteen fixtures R-01 to R-15 with planted values, truth isolation, exact rows, labelling); N30b-31 (the ten-step procedure over the rehearsal, one assertion per row, the `synthetic` sweep); each row's consumer: BD-389 (clock; delay against offset), BD-390 (U30a: aliases; friction against torque limit), BD-391 and N30b-02 (actuator recovery), N30b-06 (omitted law; protocol), N30b-03 (wrong reason), BD-393 and N30b-07 (band coverage; band refusal), BD-394 (drift; miss arithmetic), BD-392 (reachability), BD-395 (as-built). Registry `N30-KA`: U30a |
| §30.22 | Pilots, gates and kill gates | Pilots: N30b-14 (bands and drift), N30b-15 (identification), N30b-16 (as-built). Gate rows and the ceiling statement: N30b-22. Gate measurements: the beads of table (b). Kill-gate decisions: N30b-17 (KILL-GROUND), N30b-18 (KILL-BAND), N30b-19 (KILL-CLOCK), N30b-20 (KILL-ASBUILT), N30b-21 (KILL-KIT). GATE-GROUND-10: BD-397 |
| §30.23 | Open questions | N30b-25 (OPEN-GROUND-1), N30b-26 (OPEN-GROUND-2), N30b-27 (OPEN-GROUND-3), N30b-28 (OPEN-GROUND-5); OPEN-GROUND-4 (blocks nothing) in the Considerations of N30b-08 and N30b-09, with its fail-closed default enforced by N30b-09 (no rate, no ratio column) |
| §30.24 | Exposure delay and mount translation on a straight pass (late) | N30b-29 (the alias, detection in `ground.excite`, ranked separating experiments with predicted separations, the fit that returns only the combination, the refusal, the pose-graph link); N30b-30 (fixture R-15 exactly, GATE-GROUND-11 end-to-end); BD-388 (hosts the fixture); N30-CX (entry CX-G-01) |

**U30a-owned subsections whose mechanisms this unit's seed beads carry in full** (for reconciliation):

| Subsection | Carried here by |
|---|---|
| §30.1, §30.2 (the Reality claim, statuses, the line, the guess share) | BD-139, N30b-01 |
| §30.3 (ADR-GROUND-01 to -10) | Each stated with its reason in the bead it governs: -01 N30b-23; -02 BD-393, N30b-04; -03 BD-389, BD-391, N30b-05; -04 BD-391; -05 BD-392, N30b-06; -06 N30b-04, N30b-11; -07 N30b-04; -08 BD-391, N30b-28; -09 BD-394, N30b-08; -10 N30b-26, N30b-38 to N30b-40 |
| §30.4 (what the siblings provide) | The FACT rows are quoted in the bead that calls each sibling crate: BD-391 and N30b-02 (`fs-assimilate`), BD-395 and N30b-11 (`fs-asbuilt`, `fs-mbd-material`), BD-393, N30b-07 and BD-394 (`fs-eproc`), N30b-04 (`fs-uq` copula), N30b-09 (`fs-voi`); upstream items FD-FSIM-23, -24, -25 with interim posture and switch-over in BD-391, N30b-02, BD-395 |
| §30.6 (the ten-step procedure) | N30b-31 (the whole procedure), BD-397 (the procedure on the first real cell), N30b-09 (step 10) |
| §30.7 (clock fitting) | BD-389 |
| §30.9 (identification) | BD-391 (verb, passes A and B, the battery), N30b-02 (pass C), N30b-03 (the calibration record and the energy cross-check), N30b-05 (stages 2 to 4) |
| §30.10 (card updates) | N30b-04 |
| §30.11 (frozen predictions, sealed trials, holdout retirement) | BD-392 (`ground.predict`), N30b-06 (`ground.score`) |
| §30.12 (conformal bands) | BD-393, N30b-07 |
| §30.13 (drift) | BD-394 (the two e-processes), N30b-10 (scope change, expiry, retirement transaction, stale marking) |
| §30.14 (as-built ingestion) | BD-395 (steps 1 to 5 and the noise screen), N30b-11 (steps 6 to 8) |

Not carried here and expected from U30a: §30.5 beyond what BD-387 (U25a) holds; §30.8 (identifiability, the
eight aliases, the experiment grammar, the two cautions: BD-390); the published-anchor status (BD-311).

## (b) Gates, kill gates and who measures or decides them

| Gate | Milestone (register) | Beads of this unit | Note |
|---|---|---|---|
| GATE-GROUND-01 | M1, then every exit | BD-139 (mechanism and battery definition), N30b-01 (script, re-run at every exit) | No kill gate: a failure blocks the exit |
| GATE-GROUND-02 | M5 | BD-389 (mechanism and reading of the clauses), N30b-32 (script, 1,000 resamples), N30b-24 (PO-GROUND-02), N30b-19 (decision) | Feeds KILL-CLOCK |
| GATE-GROUND-03 | M5 (chapter tags M4) | N30b-31 (re-asserts 8 of 8 through the pipeline) | **Expected carrier: BD-390 (U30a)**, the bead whose done-condition is this gate. N30b-22 registers the row |
| GATE-GROUND-04 | M5 | BD-391, N30b-02, N30b-03, N30b-04 (mechanisms), N30b-32 (script, 100 rigs), N30b-17 (decision) | Feeds KILL-GROUND after one repair cycle |
| GATE-GROUND-05 | M5 | BD-392, N30b-06 (mechanisms), N30b-33 (script), N30b-19 (decision) | Feeds KILL-CLOCK |
| GATE-GROUND-06 | M5 | BD-393, N30b-07, N30b-08 (mechanisms), N30b-34 (script), N30b-24 (PO-GROUND-01), N30b-18 (decision) | Feeds KILL-BAND |
| GATE-GROUND-07 | M5 | BD-394, N30b-10, N30b-08 (mechanisms), N30b-34 (script), N30b-24 (PO-GROUND-03), N30b-18 (decision) | Feeds KILL-BAND |
| GATE-GROUND-08 | M5 | BD-395, N30b-11 (mechanisms), N30b-35 (script), N30b-20 (decision) | Feeds KILL-ASBUILT |
| GATE-GROUND-09 | M5 | BD-396, N30b-12 (mechanisms), N30b-36 (script, two platforms), N30b-21 (decision) | Feeds KILL-KIT |
| GATE-GROUND-10 | Not tied to a milestone | BD-397 (the record, both forms), N30b-40 (package E, the only route to a pass), N30b-25 and N30b-27 (the two open questions that block it) | Reality claim; resting state `no-claim` |
| GATE-GROUND-11 | M5 | N30b-29 (mechanism), N30b-30 (fixture and script) | Late gate (§40.8); no kill gate |
| KILL-GROUND | M5 | N30b-17 (decision); BD-391 (measurement) | Trigger: GATE-GROUND-04 fails after one repair cycle |
| KILL-BAND | M5 | N30b-18 (decision); BD-394 (measurement; BD-393 for -06) | Trigger: GATE-GROUND-06 or -07 fails; also the decision point of BET-28 |
| KILL-CLOCK | M5 | N30b-19 (decision); BD-389 (measurement; BD-392 for -05) | Trigger: GATE-GROUND-02 or -05 fails |
| KILL-ASBUILT | M5 | N30b-20 (decision); BD-395 (measurement) | Trigger: GATE-GROUND-08 fails |
| KILL-KIT | M5 | N30b-21 (decision); BD-396 (measurement) | Trigger: GATE-GROUND-09 fails |

All sixteen rows are entered in the gate registry by N30b-22, with fixtures, controls and kill linkage.

## (c) Counts

- Known answers: 16 cited with values (KA-721 to KA-736), each in the bead that asserts it and all in N30b-31;
  the registry bead `N30-KA` is U30a's. One fixture has no KA number (the exposure alias of §30.24; carried as
  CX-G-01 and by GATE-GROUND-11).
- Counterexamples: 10 numbered (CX-326 to CX-335) and 12 unnumbered cases the chapter states with numbers,
  under local identifiers CX-G-01 to CX-G-12, all in N30-CX with one control record each (22 records).
- Refusal codes: 11 (812, 814, 873, 876, 879, 883, 886, 889, 895, 897, 899), all in N30-REF.
- Invariants 7 and proof obligations 3 (N30b-23, N30b-24); decisions ADR-GROUND-01 to -10 (10, each in a
  Background); open questions 5 (four decision beads, one in Considerations); kill gates 5; gates 11; pilots 3;
  bench packages 6 (three beads).

## (d) Needs from other slices (no key could be named, or the key is another unit's)

- **Admission at the M5 pin of the sibling crates Ground calls** (§33.6: admission is never inherited):
  `fs-assimilate` (`nonlinear::iterated`, Huber fit, `groups`, `nonlinear::ensemble` behind its feature, the
  linear path and typed front door), `fs-eproc` with `conformal-hardening` (`MondrianConformal`, `DriftMonitor`,
  `admission_alpha`, `fcr_flag`), `fs-uq::product_copula`, `fs-asbuilt` (`rigid3`, `datum`, `field`,
  `propagate`, `uncertainty`, `well_posed`), `fs-voi`. The group gates are first recorded by BD-124
  (INT-FSIM-14) and BD-401 (INT-FSIM-15); no bead was found that re-records them for Ground's functions at M5.
- §32.12 / Appendix J.5: a task-cluster assignment for `ground_excite` and `ground_kit`, which are not in the
  `reality` cluster of seven; N30b-13 proposes `reality-plan`.
- §34.9: whether the kit manifest and the bench protocol are registered file formats (BD-410's owner).
- §38.16: OPEN-GAUNT-1, the owner's half of the dataset question, decided together with N30b-25.
- §28.13: stale marking of dependants on retirement (BD-292, BD-347) and §16: the divergence ledger entry for a
  `ranking-falsified` pair (BD-176).
- §31.12 and §31.11: fiducials and datums in the build packet (BD-408) and the tolerance allocation (BD-406),
  read by BD-395, N30b-11 and N30b-39.
- §23: the randomization compiler must ask Ground for a card field's width with a cell (BD-200), for
  INV-GROUND-05.
- A catalogue number for the twelve unnumbered counterexamples, from the owner of Appendix N.
- Refusal codes for three conditions the chapter refuses without a code (a unitless tolerance; a timestamp with
  no declared instant; a dataset with no declared cell) and for an id mismatch on a relabelled dataset, from the
  owner of Appendix O. Interim: FROB-REF-102, -873/-876, -873, -873.

## (e) Contradictions, gaps and what was done

1. **Stale refusal numbers.** §30.16 lists refusals 840 to 850 per verb and §30.12 rule 1 cites "-846, -850";
   Appendix O assigns 840 to 850 to §24 and §25. §30.19 and Appendix O agree and govern. The positional map
   (840→873, 841→876, 842→879, 843→883, 844→886, 845→889, 846→895, 847→897, 848→899, 849→812, 850→814) is in
   N30-REF and N30b-13, with a lint that fails if a `ground` verb lists the old block.
2. **GATE-GROUND-03 milestone.** The chapter says M4, §40 says M5 and notes it may be recorded at M4. §40
   governs; N30b-22 encodes both.
3. **`ground.ingest` at M1.** §30.16 gives M1 "for admission and the reality line"; the admission seed BD-387
   is M5. At M1 only the line exists (BD-139 creates the crate with the status type, the guard and empty record
   tables); the verb entry answers not-implemented until M5. The fifty misuses are designed to be runnable at
   M1.
4. **The fifty planted misuses** are fixed only as a count. BD-139 defines them (ten groups of five, with
   expected codes) as DESIGN, registered before the first run.
5. **GATE-GROUND-02 can fail by construction under one reading.** With Gaussian jitter a correct fit passes a
   resample with probability 0.9973 on skew alone, 0.9945 with coverage at one registered instant, and 0.990
   with simultaneous coverage over the bracket (Monte Carlo, 2 × 10⁵ draws). At 0.990 the count falls below
   990 of 1,000 in 38% of runs. BD-389 registers coverage at the scored instant (expected count 994.5; about
   2% chance of a false failure) and N30b-19 states the residual risk.
6. **GATE-GROUND-07's 5% against two monitors at α_d = 0.05 each.** PO-GROUND-03 bounds each monitor; the union
   is bounded by 0.10. The miss monitor contributes about 1% at a 200-episode horizon (computed). N30b-24
   measures the union before the gate, and a pre-declared half-level adjustment (threshold 40, new policy
   version) applies if it exceeds 5%.
7. **The drift gate needs a registered signal size.** At n = 20 the miss monitor retires within 20 episodes with
   probability 0.71, 0.92, 0.99 at post-drift miss probabilities 0.5, 0.6, 0.7 (computed). BD-388 registers at
   least 0.7 for the 30% friction fixture; without that the 90% clause is not reachable.
8. **KILL-CLOCK's trigger includes GATE-GROUND-05**, but "time-free metrics only" does not repair a broken
   freeze-before-read protocol. N30b-19 keeps the register's pairing and adds a fail-closed suspension of
   admission and band issue until INV-GROUND-03's own test passes again.
9. **§30.24 "bit-equal samples".** Evaluated naively in double precision, L − v·(t + δ) − b gives bit-equal
   samples for the two explanations on only 49 of 101 samples (checked). BD-388 and N30b-30 define the fixture
   in exact rationals (v·δ + b = 1/25 m) with one conversion.
10. **GATE-GROUND-11 wording** ("both the 20 ms delay and a planted 40 mm offset are recovered") is read as two
    truths, each recovered when planted, since a single truth holding both would make the forward combination
    80 mm, not the stated 40 mm (N30b-30).
11. **`ground.band` is kind `read` with a state-changing `--retire` form.** N30b-13 keeps the kind and registers
    `--retire` as a capability-gated parameter set with its own idempotency key.
12. **PO-GROUND-01 "a candidate for a theorem card"** against §12.3, which lists "a conformal margin delivers
    its error rate" among obligations no checker decides. N30b-24 records it as Tested, with an exhaustive
    rank-uniformity enumeration for n ≤ 8 as the exact part.
13. **DC-07 in §28.19.2** keys the band by (robot, firmware, floor, policy) and names a martingale on conformal
    p-values; §30.15 uses the full cell key and two named monitors. N30b-08 implements §30.15 and says so.
14. **Kit contents against the kit screen.** "The dataset if its licence permits, otherwise its hash and the
    acquisition protocol" and "refuses any asset whose licence forbids redistribution" are reconciled in BD-396:
    a restrictively licensed dataset becomes a stub; any other such asset, and anything private, is refused;
    prediction records (frozen before the data was read) ship, and derived scores under a restrictive join are
    withheld and listed.
15. **BD-388's done-condition** ("the known answers reproduce") depends on beads that depend on BD-388. BD-388
    closes on the fixtures and the closed-form rows; the full reproduction is N30b-31, which BD-388's close
    reason cites.
16. **Energy cross-check without a voltage channel.** The chapter does not say what happens. N30b-03: verdict
    `not-evaluable`, flag `energy-unchecked` printed on the card fields, admission still by sealed score; the
    stricter alternative is recorded.
17. **Calibration pairs.** The chapter does not say whether pairs used to identify a card may calibrate its
    band. BD-393 excludes them (split conformal), as DESIGN.
18. **Across-unit bands.** The chapter gives the count rule (nine units at 90%) and no construction. N30b-07:
    one keyed pick per unit, as DESIGN.
19. **Unreachable-verdict refusal** has two codes in the registry (532 from §24.14, 555 from §28.20).
    `ground.predict` raises 532, the study planner's.
20. **Pilots.** §30.22 names three pilots; clock fitting, frozen predictions and the kit are Frontier with no
    named pilot. None was invented: PO-GROUND-02's derivation and Monte Carlo (N30b-24) is the paper-scale check
    for clocks, and the other two are protocol and packaging with no falsifiable performance claim.
21. **Bench packages have no milestone.** Filed at M5, priority 4, under WP-69, depending on the owner decision
    N30b-26; a graph lint in N30b-26 keeps them from blocking any exit.
22. **Bead count** (52) is above the typical range because ten seed beads here carry eight mechanism
    subsections of the other unit in addition to this unit's ten subsections.

## (f) Integration note (group G30)

Added by the integration pass; the tables above are the drafter's and are left as drafted.

- **Capstones.** The seed beads BD-139, BD-389, BD-391, BD-392, BD-393, BD-394 and BD-395 depend on the part
  beads of U30a (N30a-01, N30a-02; N30a-11 to -13; N30a-20 to -25 and N30a-27 to -30; N30a-31 to -34; N30a-36 to
  -38; N30a-40 to -43; N30a-45 to -47), and their Scope sections say so. The verb `ground.score` (N30b-06)
  depends on N30a-28, -29, -32, -33 and -34.
- **Superseded beads of U30b** (each carries `superseded_by`; content folded into the kept bead or beads):
  N30b-02 → N30a-23; N30b-03 → N30a-24; N30b-04 → N30a-27 (and N30a-28, -29, -30); N30b-05 → N30a-25;
  N30b-07 → N30a-38 (and N30a-37); N30b-10 → N30a-43 (and N30a-40); N30b-11 → N30a-47; N30b-32 → N30a-26
  (clock half: N30a-14); N30b-33 → N30a-35; N30b-34 → N30a-39 (drift half, the DC-07 walk and the planner rows:
  N30a-44); N30b-35 → N30a-48.
- **Gate scripts.** GATE-GROUND-02 `ground_clock.sh` (N30a-14); -03 `ground_excite.sh` (N30a-19); -04
  `ground_identify.sh` (N30a-26); -05 `ground_protocol.sh` (N30a-35); -06 `ground_band.sh` (N30a-39); -07
  `ground_drift.sh` (N30a-44); -08 `ground_asbuilt.sh` (N30a-48). The kill-gate decisions N30b-17 to N30b-20
  depend on these.
- **Design choices settled between the two units.** Rehearsal fixtures are those of BD-388's catalogue (the
  delay grid is T_c/8). GATE-GROUND-02 asserts coverage at the registered scored instant. An envelope
  excursion retires the calibration. The across-units statistic defaults to a keyed pick per unit. A combined
  result with a synthetic part is `synthetic`. The reality line is printed in N30a-02's grammar, the `cards:`
  line in N30a-03's. A unitless tolerance is refused with FROB-REF-102. First-use admission records of the
  sibling crates at the M5 pin are written by N30a-05 and N30a-06.
