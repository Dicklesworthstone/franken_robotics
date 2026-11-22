# Coverage of drafting unit U30a

Chapter §30 Ground: calibration against reality; the chapter preamble and subsections §30.1 to §30.14; the seed
beads BD-311 and BD-390; the registry bead `N30-KA`.
51 bead files: 2 seed beads, 48 new beads (`N30a-01` to `N30a-48`) and the registry bead `N30-KA`. `N30-CX` and
`N30-REF` are written by U30b.

How this unit relates to the seed beads of the same subsections. The seed beads whose titles are the mechanisms of
§30.7 and §30.9 to §30.14 (BD-389, BD-391, BD-392, BD-393, BD-394, BD-395) are drafted by U30b, and BD-387 (§30.5) by
U25a. This unit therefore wrote the parts of each mechanism as separate beads (the estimator, its rules and
refusals, its record, its gate script), each complete on its own, and expects each of those seed beads to be the
integrating bead that closes the gate and decides the kill gate. The wiring the lead should add is in part (d).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §30 preamble | Crate, layer, cluster, tiers, "no gate needs hardware" | N30a-01 |
| §30.1 | What Ground owns | N30a-01 (ownership and what it does not own; INV-GROUND-07 firewall); each owned item has its beads below |
| §30.2 | The Reality claim, and why it is mostly `no-claim` | N30a-02 (four reasons, six statuses, transitions, line grammar); N30a-03 (the share of `generic` and `assumed` card fields); BD-311 (`published-anchor`); N30a-04 (end-to-end script); the line on every report and its 50-misuse battery: BD-139 (U30b) |
| §30.3 | Decisions | The register of ADR-GROUND-01 to -10: N30a-01. Governing beads: -01 N30a-01; -02 N30a-07; -03 N30a-20; -04 BD-390, N30a-15; -05 N30a-31, N30a-34 (and BD-387); -06 N30a-27; -07 N30a-28; -08 N30a-01, N30a-05, N30a-06; -09 N30a-41, N30a-42; -10 N30a-01, N30a-16 |
| §30.4 | What the siblings provide today | N30a-05 (`fs-assimilate`, `fs-asbuilt`; the three consequences; FD-FSIM-23, -24, -25 interim postures); N30a-06 (`fs-uq`, `fs-eproc`, `fs-voi`; local conformal reference) |
| §30.5 | Data admitted, and the cell | BD-387 and N25a-29 (U25a: manifest, admission rules, sealing, `ground.ingest`); N30a-07 (the cell key as a type, envelope containment, diff, pool guard 814); N30a-08 (instruments, shared calibration error, the covariance R) |
| §30.6 | The procedure | N30a-09 (ten steps as per-cell state, preconditions, fail-closed outcomes, step 10 feedback); N30a-10 (end-to-end script) |
| §30.7 | Clock fitting [F] | N30a-11 (fit, two-mark formulas, piecewise, 876); N30a-12 (rules 1, 4, 5); N30a-13 (rules 2, 3); N30a-14 (GATE-GROUND-02 script); integrating seed BD-389 (U30b) |
| §30.8 | Identifiability and excitation design [F] | BD-390 (Fisher analysis, the checks, `ground.excite`); N30a-15 (eight structural aliases, fit guard 879); N30a-16 (experiment grammar); N30a-17 (greedy selection; decision relevance); N30a-18 (profile likelihood; model-class caution); N30a-19 (GATE-GROUND-03 script) |
| §30.9 | Identification: actuators first [F] | N30a-20 (why actuators first, order, L1 model, gating, L2 quantities, direct measurement); N30a-21 (pass A); N30a-22 (pass B); N30a-23 (pass C); N30a-24 (receipt, autocorrelation, energy cross-check `wrong-reason`); N30a-25 (stages 2 to 4); N30a-26 (GATE-GROUND-04 script); integrating seed BD-391 (U30b) |
| §30.10 | Card updates from logs [F] | N30a-27 (new card version, covariance, correlated draws); N30a-28 (candidate then admitted; INV-GROUND-05); N30a-29 (discrepancy floor); N30a-30 (unit and class cards) |
| §30.11 | Frozen predictions, sealed trials, holdout retirement [F] | N30a-31 (record, freeze, tolerances); N30a-32 (interval score, bias, coverage, structure; `ranking-falsified`); N30a-33 (study type; reachability); N30a-34 (versions, spent holdout, α_i); N30a-35 (GATE-GROUND-05 script); integrating seed BD-392 (U30b) |
| §30.12 | Conformal bands per calibrated cell [F] | N30a-36 (definition, quantile, coverage law, PO-GROUND-01); N30a-37 (exchangeable unit, rule 1, pooling counterexample, shift bound); N30a-38 (rules 2 to 4, correlation, ceiling); N30a-39 (GATE-GROUND-06 script); integrating seed BD-393 (U30b) |
| §30.13 | Drift detection that retires a calibration [F] | N30a-40 (scope change, expiry); N30a-41 (rank drift); N30a-42 (miss counting); N30a-43 (retirement effects, printed power facts, 897); N30a-44 (GATE-GROUND-07 script); integrating seed BD-394 (U30b) |
| §30.14 | As-built ingestion [F] | N30a-45 (steps 1 to 3, noise screen, provenance, 812); N30a-46 (steps 4, 5); N30a-47 (steps 6 to 8); N30a-48 (GATE-GROUND-08 script); integrating seed BD-395 (U30b) |
| Appendix M.29 | Known answers of §30 | N30-KA |
| Appendix I.2, I.16 | `RealityStatus`; `CalibrationRecord` | N30a-02; N30a-24 |

Identifiers carried. ADR-GROUND-01 to -10 as in the §30.3 row. Invariants (defined in §30.20, tested here):
INV-GROUND-01 (N30a-02 at type level; BD-139 on every report), -02 (N30a-27), -03 (N30a-31, with BD-387), -04
(N30a-34), -05 (N30a-28), -06 (N30a-02, N30a-37), -07 (N30a-01). Proof obligations: PO-GROUND-01 (N30a-36),
PO-GROUND-02 (N30a-11), PO-GROUND-03 (N30a-41, N30a-42). Open questions, in Considerations with their defaults:
OPEN-GROUND-1 (N30a-37), -2 (N30a-16), -3 (N30a-40), -4 (N30a-06, N30a-17), -5 (N30a-01, N30a-05); OPEN-SINEW-1
(N30a-03). Kill gates, named with trigger and fallback where they change the bead: KILL-CLOCK (N30a-11 to -14,
N30a-35), KILL-GROUND (N30a-20, N30a-25, N30a-26, BD-390), KILL-BAND (N30a-36, N30a-39, N30a-41 to -44), KILL-ASBUILT
(N30a-45 to -48). Constructions and analyses: C-39 (N30a-17 with `SPEC-C-39` in deps; N30a-18), A-25 (BD-390,
N30a-15), A-16 (N30a-17). Bets, hypotheses, goals, risks: BET-28 and H-14 (N30a-36, N30a-39), GOAL-37 (N30a-01),
RISK-32 (N30a-03, BD-311), RISK-42 (BD-390, N30a-24), RISK-43 (BD-311). Upstream items FD-FSIM-23, -24, -25
(N30a-05, N30a-22, N30a-23, N30a-45, N30a-48; the items themselves are UF's beads).

## (b) Gates and kill gates of §30, and the beads that measure or decide them

| Gate | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|
| GATE-GROUND-01 | N30a-02 (the status type, the default, the grammar; unit level) | BD-139 (U30b): the line on every report and the battery of 50 planted misuses; re-run at every exit |
| GATE-GROUND-02 | N30a-14 (script and record); N30a-11, N30a-12, N30a-13 (mechanisms and unit-level rows) | BD-389 (U30b) closes it |
| GATE-GROUND-03 | N30a-19 (script and record); BD-390, N30a-15, N30a-16, N30a-17, N30a-18 | — |
| GATE-GROUND-04 | N30a-26 (script and record); N30a-20 to N30a-25, N30a-27, N30a-28 | BD-391 (U30b) closes it |
| GATE-GROUND-05 | N30a-35 (script and record); N30a-31 to N30a-34 | BD-392 (U30b) closes it; N25a-29 (U25a) covers sealed-read routes at ingest |
| GATE-GROUND-06 | N30a-39 (script and record); N30a-36, N30a-37, N30a-38 | BD-393 (U30b) closes it |
| GATE-GROUND-07 | N30a-44 (script and record); N30a-40 to N30a-43 | BD-394 (U30b) closes it |
| GATE-GROUND-08 | N30a-48 (script and record); N30a-45, N30a-46, N30a-47 | BD-395 (U30b) closes it |
| GATE-GROUND-09 | — | U30b (§30.18; BD-396) |
| GATE-GROUND-10 | — | U30b (BD-397) |
| GATE-GROUND-11 | — | U30b (§30.24) |
| KILL-CLOCK | N30a-14 and N30a-35 write the two records it reads | The decision bead: U30b (§30.22); BD-389 |
| KILL-GROUND | N30a-26 writes the record; N30a-20 builds the fallback path (direct measurement) | The decision bead: U30b; BD-391 |
| KILL-BAND | N30a-39 and N30a-44 write the two records; N30a-43 is the withdrawal path | The decision bead: U30b; BD-394 |
| KILL-ASBUILT | N30a-48 writes the record | The decision bead: U30b; BD-395 |
| KILL-KIT | — | U30b (BD-396) |

## (c) Counts

- Known answers: 16 of 16 (KA-721 to KA-736) in `N30-KA`, each with case, truth, tolerance, tier, site, negative
  control and carrying beads; four rows had "none stated" as control and received a DESIGN control. Eleven
  supporting values that the chapter states without a KA number are listed there as proposed auxiliary rows.
  Known answers of other chapters cited with their values: KA-187, KA-188 (C-39).
- Counterexamples: all ten of §30 are cited with their cases in mechanism beads (CX-326, -327: N30a-13; CX-328,
  -329: BD-390, N30a-15; CX-330: N30a-07, N30a-37; CX-331: N30a-18, N30a-28, N30a-29; CX-332: N30a-24; CX-333:
  N30a-33; CX-334: N30a-02; CX-335: N30a-03, BD-311). The registry bead `N30-CX` is U30b's. Also cited: CX-143,
  CX-092, CX-093.
- Refusal codes: ten of the chapter's eleven are specified with meaning, trigger, repair and exit code 3 in the
  bead that raises them (812: N30a-45; 814: N30a-07, N30a-37; 873: BD-387, N30a-12; 876: N30a-11, N30a-12,
  N30a-13; 879: N30a-15, BD-390; 883: N30a-31, N30a-34; 886: N30a-34; 889: N30a-36, N30a-37; 895: N30a-37;
  897: N30a-43). 899 (the kit) is U30b's. The registry bead `N30-REF` is U30b's. Two codes of §24 are used where
  the study library raises them: 532 and 552 (N30a-33).
- Subsections: 14 (§30.1 to §30.14) and the preamble. Gates with a measuring bead in this unit: 7
  (GATE-GROUND-02 to -08), plus the type-level half of GATE-GROUND-01.

## (d) Needs from other slices

Wiring the lead should add (these seed beads are drafted by other units; the beads on the right are their parts):

| Seed bead | Should depend on |
|---|---|
| BD-389 Clock fitting | N30a-11, N30a-12, N30a-13 (N30a-14 depends on BD-389) |
| BD-391 Identification and card updates | N30a-20 to N30a-25, N30a-27 to N30a-30 (N30a-26 depends on BD-391) |
| BD-392 Frozen predictions | N30a-31 to N30a-34 (N30a-35 depends on BD-392) |
| BD-393 Bands | N30a-36, N30a-37, N30a-38 (N30a-39 depends on BD-393) |
| BD-394 Drift | N30a-40 to N30a-43 (N30a-44 depends on BD-394) |
| BD-395 As-built ingestion | N30a-45, N30a-46, N30a-47 (N30a-48 depends on BD-395) |
| BD-139 The `reality:` line | N30a-02 |
| BD-387 Admission (U25a, already drafted) | none added; N30a-07 and N30a-08 extend it and depend on it |

From U30b (named in `needs`, no key known): the three pilot beads of §30.22 (bands and drift; identification;
as-built), on which BD-390, N30a-20 to N30a-23, N30a-36, N30a-37, N30a-41, N30a-42 and N30a-45 to N30a-47 depend;
the five kill-gate decision beads; the decision beads for OPEN-GROUND-3 and the other open questions; the
decision model of §30.15 (the guard table, α_d, the experiment table, the budgeted mode of 30 episodes); the
ninth alias of §30.24 and GATE-GROUND-11; the verbs of §30.16; the rehearsal generator's fixtures used by every
gate script (BD-388); `N30-CX` and `N30-REF`. If U30b also writes end-to-end beads for `ground.identify`,
`ground.predict`/`ground.score` or `ground.band`, they overlap with N30a-26, N30a-35 and N30a-39 and one of
each pair should be kept.

From other units:
- U25a (BD-387): the dataset manifest needs fields this unit's beads rely on: the kind of each clock mark
  (N30a-13), a maintenance log and per-unit operating hours (N30a-40), fiducials, length unit and an optional
  external-registration block for scans (N30a-45), and an optional `resolution` and `bias_sigma` per instrument
  (N30a-08).
- U38a (N38a-05): the typed Parity record with status `published-anchor` (BD-311, N30a-02).
- U28a (N28a-13, BD-348): analysis A-25 and its regime enumeration (BD-390, N30a-15); BD-351 for the value of an
  experiment (N30a-17); BD-292 and BD-347 for marking results stale (N30a-27, N30a-43).
- U41 (BD-200): randomization must read a Ground-touched field only through `ground::effective` and must use the
  card's covariance block for correlated draws (N30a-27, N30a-28).
- U15a (BD-069): the card schema must admit a `[covariance]` block, `delay_set`, `parent`, and the hierarchical
  class fields (N30a-27, N30a-30); `actuator.show` prints them.
- U25e or the report owner (BD-302): reports print the `cards:` line (N30a-03) and re-render a retired band as
  `no-claim` (N30a-43).
- U31b (BD-406, BD-408): fiducials and the tolerance list with sensitivities in the build packet (N30a-45,
  N30a-47).
- U32 (§32.13): the session capability `ground:retire` (N30a-43).
- U33 (§33.7): feature groups of the airlock for `fs-assimilate`, `fs-asbuilt`, `fs-uq`, `fs-eproc` (with
  `conformal-hardening`), `fs-voi` and `fs-mbd-material` listed for `frob-ground` (N30a-05, N30a-06, N30a-47).

## (e) Contradictions, gaps and what was done

1. **Refusal numbers.** The verb table of §30.16 lists refusals 840 to 850 for the Ground verbs, and rule 1 of
   §30.12 cites "-846, -850"; the refusal registry (Appendix O) assigns 840 to 850 to §24.14 and §25.24. The
   chapter's own refusal table (§30.19) and the registry agree on 812, 814, 873, 876, 879, 883, 886, 889, 895,
   897, 899. Those are used throughout (as U25a also did in BD-387).
2. **GATE-GROUND-03's milestone.** §30.22 says M4; the gate register says M5 and notes it may be recorded at M4.
   BD-390 and its parts are at M5; N30a-19 records the gate at M4 through the guard function if
   `ground.identify` does not yet exist, and again at M5 through the verb.
3. **Missing refusal codes.** No registry code exists for: an invalid prediction at registration (a unitless
   tolerance, N30a-31); a split or denominator refusal and a missing instrument instant at admission (873 is
   used, as in BD-387); a scan whose scale indicates a unit error (812 is used with a `unit-suspicion`
   diagnostic, N30a-45). Each bead says which code it uses and why.
4. **Two separators in the alias table.** For alias 1 (armature against link inertia) a payload of known inertia
   adds a known constant to the same sum and fixes the torque scale (alias 4), not the split; for alias 8
   (contact stiffness against damping) a second drop height separates the pair only if restitution depends on
   impact speed in the contact law. The table is carried verbatim; N30a-15 and N30a-19 assert separation for
   aliases 2, 4, 5, 6 and 7 and, for 1, 3 and 8, assert only that the report agrees with the rank test and
   record what it finds.
5. **The record's mutable fields.** The type sketch of the calibration record contains `status` and
   `discrepancy_floor`, which change after the record is written, while cards and receipts are immutable.
   N30a-24 reads them as ledger events over an immutable record.
6. **Pilots.** §30.22 names pilots for three mechanism groups (bands and drift; identification; as-built).
   Clock fitting, card updates and frozen predictions are tagged [F] and have no named pilot; no pilot bead was
   invented for them, and their gates are decided on the synthetic rehearsal.
7. **GATE-GROUND-02's margin.** With Gaussian jitter and the 3σ criteria, an honest implementation recovers in
   about 99.3 to 99.5% of resamples; at 1,000 resamples the standard error is 0.24 points, so the 99% threshold
   can be missed by chance on about one seed in twenty. N30a-14 pins the seed, logs three more, and escalates to
   10,000 resamples before concluding (DESIGN; for the Ground lead to confirm).
8. **GATE-GROUND-07's power.** At n = 20 the miss monitor gains evidence only when the true miss rate exceeds
   about 0.35 and the rank monitor only for gross shifts. Whether "friction raised by 30%" is detected within 20
   episodes depends on its effect on the banded metric. N30a-44 logs the effect size so that a failure can be
   told apart from a defect.
9. **p₀(n).** The plan prints 0.22 and uses it in the miss-monitor arithmetic (26.7 and 11.7); the exact 95th
   percentile of Beta(2, 19) is 0.2161, which would give 28.7 and 12.4. N30a-42 rounds the quantile up to two
   decimals (conservative; reproduces both of the plan's figures and KA-731).
10. **Where the plan is silent and a DESIGN choice was made** (each flagged in its bead): the estimator of the
    reality-gap variance per cell (N30a-09); the rule that finds where a reversed ranking "held" (N30a-32); what
    happens when the energy cross-check cannot run for lack of a voltage channel (N30a-24); whether an admitted
    card's width reverts when its cell is retired (N30a-28); the per-unit statistic of an across-units band
    (N30a-37); the prior weight on unit-to-unit spread (N30a-30); the window features of pass B (N30a-22); the
    delay grid step and ensemble size of pass C (N30a-23); the tolerance bounds at registration (N30a-31); the
    coverage allowance for admission (N30a-33); knot placement for a piecewise clock (N30a-11); the experiments,
    warm starts and guards of identification stages 2 to 4, which the plan gives one line each (N30a-25).
11. **Conformal source.** §33's sibling table says conformal bands "come from `fs-surrogate` or are written
    here", and that `fs-conform` is not conformal prediction; §30 names `fs-eproc::hardening::MondrianConformal`.
    §30 is followed, with a local reference implementation as differential check and fallback (N30a-06).
12. **`ground.ingest` at M1.** The verb table gives M1 "for admission and the `reality:` line" while admission is
    an M5 seed bead; only the crate, the status type and the line exist at M1 (N30a-01, N30a-02, BD-139).
