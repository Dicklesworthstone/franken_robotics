# Coverage of drafting unit U16b

Chapter §16 Kinesis, subsections §16.8 to §16.15. Seven seed beads (BD-212, BD-214, BD-263, BD-264, BD-265,
BD-266, BD-268) and thirty-five new beads (N16b-01 to N16b-35). The chapter's registry beads are written by other
units: `N16-KA` (U16a), `N16-CX` and `N16-REF` (U16c); the beads here cite KA and CX numbers with their values
inline and name the registries under `related`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Content | Beads |
|---|---|---|---|
| §16.8 | The trust ladder, the fidelity ledger and the promotion rule | The standard battery the ledger is measured on | N16b-01 |
| §16.8 | | The fidelity ledger: key, quantities, task decisions as disagreement | N16b-02, BD-212 |
| §16.8 | | Frozen envelopes, the one re-baseline, `fidelity.ledger`, GATE-FID-01 | BD-212, N16b-30 |
| §16.8 | | The promotion rule and its labels, `provisional` and `established` | BD-263, N16b-04 |
| §16.8 | | The label: the promote and reject e-process pair, the integer table, budgeted mode, fixed fallback | N16b-05, BD-263 |
| §16.8 | | The number: the doubly robust tier correction, ρ and the saving, PO-KIN-05 | BD-264, N16b-07, N16b-29 |
| §16.8 | | Variants: three-level telescoping estimate, segment mode | N16b-08, N16b-09 |
| §16.8 | | Budget, fallback triggers, KILL-DR | BD-264, N16b-10 |
| §16.8 | | Labels a result can carry, FROB-REF-820, the weakest-label rule | N16b-04 |
| §16.8 | | Planted tier-sensitive policy (NS-09), planted reversal | N16b-06, N16b-31 |
| §16.9 | Rank agreement between tiers, and model-form rank checks | Rank agreement between tiers: record, statistics, permitted use per stratum, pilot, KILL-RANK reading | BD-265, N16b-11, N16b-12, N16b-32 |
| §16.9 | | Model-form rank checks: variant families, the check, consequences, cost, demotion rule | N16b-13, BD-266, N16b-15, N16b-32 |
| §16.10 | The audit for energy created by the simulator | Signed residual, positive part by contact mode, storage rule, the flag's consequences | N16b-14 (with seed BD-208, U29a) |
| §16.10 | | The two planted exploits and GATE-FID-06 | N16b-15, N16b-33 |
| §16.11 | The background referee and its coverage figures | Sampling, snapshots, the coverage figure, arithmetic of coverage | BD-214, N16b-16 (pilot) |
| §16.11 | | One-step defects binned by stage | N16b-17 |
| §16.11 | | Outcome distributions, detection arithmetic | N16b-18 |
| §16.11 | | The gate at M3, KILL-SHADOW, the fallback | BD-268, N16b-19, N16b-20, N16b-34 |
| §16.11 | | The referee's own coverage (report per class, GATE-FID-03) | BD-213 (seed bead of U16a); consequence rules in N16b-04, N16b-26 |
| §16.12 | Events, loops and limits of the claim | Events: lockstep tiers do not locate events; the missed-contact audit | N16b-21 |
| §16.12 | | Kinematic loops: compliant rows on lockstep, bilateral rows when transactional, referee waits on upstream | N16b-22 (pilot), N16b-23 |
| §16.12 | | Derivatives offered only where they exist; refusal with the offending state; labelled alternatives | N16b-24 |
| §16.12 | | No real-time claim; interactive viewing with physics unchanged | N16b-24, N16b-25 |
| §16.13 | Verbs | `sim.run` | N16b-25 |
| §16.13 | | `sim.estimate` | N16b-26 |
| §16.13 | | `eval.promote` | BD-263, BD-264, N16b-08 |
| §16.13 | | `fidelity.ledger` | BD-212 |
| §16.13 | | `fidelity.rank` (tier-pair rank agreement) | BD-265, N16b-27 (naming decision) |
| §16.13 | | `fidelity.modelform` | BD-266 |
| §16.13 | | `fidelity.energy` | BD-208 (seed bead of U29a); data contract in N16b-14; e2e N16b-33 |
| §16.13 | | `fidelity.coverage` | BD-213 (U16a) and BD-214; e2e N16b-34 |
| §16.13 | | `fidelity.shadow` | BD-214 |
| §16.13 | | Registry entries, the cluster rule (six verbs; seven per task cluster) | N16b-27 |
| §16.14 | Refusals | All ten rows with trigger, repairs and exit code, exercised end to end | N16b-35 (table and script); N16-REF (U16c, registry) |
| §16.14 | | FROB-REF-820 | N16b-04 |
| §16.14 | | FROB-REF-821 | BD-212, BD-263 |
| §16.14 | | FROB-REF-222, FROB-REF-722 on the verb | N16b-25 |
| §16.14 | | FROB-REF-212, -220, -221, -223, -224, -721 (mechanisms in §16.5 and §16.6) | N16b-35 (surface test); mechanisms in U16a and U16c beads |
| §16.15 | Invariants and proof obligations | INV-KIN-01 to -08 | N16b-28 (register); -05 also N16b-03; -06 also N16b-14; -07 also N16b-13; -08 also N16b-04 |
| §16.15 | | PO-KIN-01 to -05 | N16b-29 (register); -05 also BD-264 |

## (b) Gates, kill gates and admission gates of the chapter

| Gate | Milestone | Beads that measure or decide it |
|---|---|---|
| GATE-FID-01 | M2 | BD-212 (record), N16b-30 (script) |
| GATE-FID-02 | M3 | BD-263 (record), N16b-31 (script) |
| GATE-FID-03 | M2 | Expected from U16a: seed bead BD-213. Consumed here by N16b-04 and N16b-26 |
| GATE-FID-04 | M3 | BD-265 (record), N16b-32 (script) |
| GATE-FID-05 | M3 | BD-266 (record), N16b-32 (script) |
| GATE-FID-06 | M2 | N16b-33 (record and script), N16b-14 (unit part); the seed bead BD-208 (U29a) also lists it |
| GATE-FID-07 | M2, repeated M3 | BD-214 (M2 clauses), BD-268 (full gate at M3), N16b-34 (script) |
| GATE-FID-08 | M3 | BD-264 (record), N16b-31 (script) |
| GATE-KIN-07 | M2 | N16b-03 (record), BD-212 (done-condition) |
| GATE-KIN-01 to -06 | M1, M2 | Expected from U16a (mechanisms of §16.5 to §16.7). N16b-28 and N16b-29 exercise the invariants and obligations behind -04 and -05 without recording the gates |
| GATE-KIN-08 to -13 | M2 to M8 | Expected from U16c (§16.19 to §16.24) |
| KILL-DR | M3 | N16b-10 (decision), BD-264 (measurements) |
| KILL-RANK | M3 reading; M4 gate | N16b-12 (tier-pair reading), BD-265 (measurements). The kill gate proper belongs to §28 |
| KILL-SHADOW | M3 | N16b-19 (decision), BD-268 (measurements), N16b-20 (fallback branch) |
| KILL-TREEOP, KILL-VARIATIONAL | M5 | Expected from U16a (§16.6, §16.7) |
| KILL-ANCESTOR, KILL-LOOPELIM, KILL-STEPADJOINT (and KILL-ROWQUEUE, KILL-INTERACT) | M2 to M8 | Expected from U16c |
| Gates of other chapters named by the seed beads | | GATE-ARENA-04 batch rows: BD-212. KILL-CONTACT: measurements in BD-212, decision bead owned by §18. GATE-CHRON-09: BD-264. GATE-CORTEX-15: BD-266. GATE-CHRON-08: controls registered by N16b-15 |

The chapter defines no admission gate.

## (c) Counts

- Known answers of §16 (Appendix M, KA-338 to KA-364, 27 rows): 21 carried with their values in these beads
  (KA-338 to KA-354 and KA-361 to KA-364). The nine that belong to these subsections are KA-350 to KA-354 and
  KA-361 to KA-364. Not carried here: KA-355 to KA-360 (integrators and parity, U16a).
- Known answers of other chapters cited with values: 16 (KA-016, -055, -088, -089, -132, -133, -134, -135, -401,
  -591, -692, -695, -709, -710, -719, -821).
- Counterexamples of §16 (Appendix N, CX-144 to CX-152): all 9 carried. Others cited: 6 (CX-026, -062, -258,
  -318, -324, -383).
- Refusal codes of the §16.14 table: all 10 carried (212, 220, 221, 222, 223, 224, 721, 722, 820, 821); the nine
  that Appendix O attributes to §16 are among them. Codes of other chapters used: 263, 537, 724, 811, 822, 830.
- Invariants: INV-KIN-01 to -08 (8), plus INV-CHRON-13. Proof obligations: PO-KIN-01 to -05 (5), plus
  PO-CHRON-02 and PO-ATEL-01.
- Architecture decisions stated in Backgrounds: ADR-KIN-01, -02, -04, -05, -06.
- Pilots: 5 (N16b-07, N16b-09, N16b-11, N16b-16, N16b-22). Kill-gate decisions: 3 (N16b-10, N16b-12, N16b-19).
  End-to-end scripts: 6 (N16b-30 to N16b-35).

## (d) Needs from other slices

- §24.9: the battery selector of `eval.run` (`--battery standard`) and the refusal on a battery-identity
  mismatch (GATE-ARENA-07). §38.16: OPEN-GAUNT-7 (battery lists for wheeled, tracked, wheel-legged, manipulating
  classes).
- §35.4: the per-world decision comparator and the exact binomial bound on discordant decisions (OPEN-NUM-2).
- §18.17: the KILL-CONTACT decision bead. §24.10: the batch-tier rows of fixture 1.
- §16.6 (U16a): the `Degraded` flag with its reason and the three caller policies; a forced-overflow hook.
  §36.3: OPEN-SPARK-3. §16.18 (U16c): OPEN-KIN-3, OPEN-KIN-5, OPEN-KIN-6.
- §16.5 (U16a, U16c): fault-injection hooks between check and commit; a forced-retry hook; fixtures that provoke
  refusals 220, 221, 223, 224; §16.4: a scenario over the referee's 21-contact cap; the referee's refusal cause
  `kinematic loop`; the per-class referee coverage figure.
- §23.19: FROB-REF-830 and FROB-REF-537. §18.11: `capacity-biased` and FROB-REF-822. §12.7: FROB-REF-263.
  §29.17: FROB-REF-811 (carried in BD-264, registry row owned by §29). §32.18: FROB-REF-724 and capability
  scopes.
- §28.19.1: the decision-core record `registries/cores/dc-01.toml` and `frob.core-table/1`. §28.6: the mixture
  confidence sequence from `fs-eproc` through the airlock. §28.10: the `rank_agreement` table shared with the
  rung-pair measurement. §28.9: the split-conformal margin. §28.23: the KILL-RANK decision bead for rungs.
- §18.5: CL-2 selectable on `batch-cpu`, CL-H as a batch law; the per-law convexity certificate. §18.6: the
  four-sided pyramid as a labelled, non-production cone kind. §18.13: the tunnelling-audit primitive.
- §29.9: the window and denominator of `energy-created`, the verb `fidelity.energy`. §29.22: the algorithmic
  exchange line. §29.16: retention pins for snapshots.
- §32: a low-priority job class with a CPU-core budget in the daemon; §32.12.4: task-cluster declarations and
  lints; §32.15: viewer pacing. §36.6: the paired lower-bound method. §36.10: the memory arithmetic.
- §33: airlock entries for `fs-uq` allocation, `fs-time::rk45_until_event`, `fs-contact::certified_ccd`.
- §14: the closure list of the compiled model and mobility analysis A-01. §17: per-world designs through
  `--designs`. §21: the kernel manifest's derivative-program field; §21.15: the divergence ledger.
- §42: OPEN-PROG-4 and OPEN-SPARK-2 (compute for the rank-agreement experiment). §31: the build-packet and
  prototype-budget verbs that must refuse a `model-conditional` ranking.

## (e) Contradictions and things that could not be turned into work as written

1. **One verb, two names.** §16.13 and one row of Appendix J name the tier-pair rank measurement `fidelity.rank`
   and count six `fidelity` verbs; Appendix J's one-name table, its task-cluster list ("five verbs") and the
   seed titles of BD-265 and BD-328 register it as `eval.rank-agreement` for rungs and for tier pairs, with no
   alias allowed. Done: one implementation (BD-265); default name `eval.rank-agreement` with a tier pair; the
   question goes to the owner in N16b-27.
2. **`fidelity.shadow` milestone.** The verb table says M3; the seed bead BD-214 and the M2 exit list deliver it
   at M2. Done: delivered at M2 behind its feature; noted in N16b-27.
3. **Coverage threshold.** GATE-FID-03, BET-34 and §16.11 say 90%; CX-151 and RISK-31 print 80%. Done: 90%
   (the gate register governs); noted in N16b-04.
4. **Created-energy bound.** OPEN-KIN-6 prints the default as "1% of actuator work per episode"; §16.10 and
   §29.9 state 1% of the absolute work exchanged in any 0.5 s window (10⁻⁶ relative on the referee). Done: the
   windowed rule; the open question closes with the single re-baseline (N16b-14, BD-212).
5. **Kill gates stated twice.** KILL-DR and KILL-RANK appear in §16.17 and in §29.20 and §28.23 with different
   triggers. Done: followed the register's resolution (§40.7): §29 governs KILL-DR and the effective-sample
   clause is a diagnostic (N16b-10); §28 governs KILL-RANK and the tier-pair row is a recorded reading (N16b-12).
6. **Seed done-conditions that say "KILL-… decided".** A decision needs the seed's own measurements, so the
   decision beads (N16b-10, N16b-19, N16b-12, and §18's bead for KILL-CONTACT) are successors of BD-264, BD-268,
   BD-265 and BD-212. The seed beads record the measurements and name the decision bead in their acceptance.
7. **Loss constants of the promotion decision.** §28.19 illustrates c = 2 and C_fp = 2,000; §29.11 lists 100, 1
   and 10. Done: both recorded as illustrative; the generated table uses (0.05, 0.30, 0.01, 0.05); OPEN-ATEL-1
   decides (N16b-05).
8. **Bytes per world.** KA-348 computes 27 kB and 72 kB (C = 24, R = 128; C = 40, R = 224); the capacity table
   of §16.6 gives 31 kB and 92 kB (C = 32, R = 160; C = 64, R = 320), and §16.21 quotes 27 kB. Done: the table's
   capacities, with the capacities printed beside every figure (N16b-03, N16b-26).
9. **Canonical zero.** CX-146 prints the canonical zero as +0; PO-KIN-03, §35.5 and KA-346 say −0.0, with +0.0
   as the failing case. Done: −0.0 (N16b-29).
10. **Battery freeze reference.** §38.8 says the battery is frozen at M2 "(BD-104)"; BD-104 is the
    self-collision analysis. Done: the freeze is BD-212 with N16b-01.
11. **"Each variant is a different `StepPlanId`."** Stiffness is a physical parameter of the interface card
    (§18.5), so the stiffness variant changes the parameter digest, not the step plan. Done: the variant record
    carries both identities (N16b-13).
12. **FROB-REF-820 and -830 overlap** on a `model-conditional` ranking used to authorize spend. Done: 830 for
    promotion, build and prototype budget (the model-form gate requires it), 820 for every other citation
    (N16b-04).
13. **Three wordings of `model-robust`.** §16.9: top-k overlap at least k − 1 and a clean energy audit; §23.16:
    the set and its order within noise; §28.11: the set and the order of the top two. Done: §16.9's criterion in
    BD-266; the cascade's stricter condition is left to BD-346.
14. **Gait rows of the M2 battery.** The battery asks for "each atlas gait at three speeds" at M2; the gait
    atlas is an M4 item. Done: the gaits of the baseline controller at M2, extended by a recorded battery
    revision later (N16b-01).
15. **A Frontier mechanism with no named pilot.** §16.12 labels the compliant closure row [F] and names no
    pilot or falsifier for it. Done: a pilot with a DESIGN falsifier, to be ratified by the Kinesis lead
    (N16b-22).
16. **Envelope values for ungated quantities.** The plan gives thresholds for three quantities only. Done: a
    DESIGN rule (the measured 95th percentile, rounded up, recorded at freeze) in BD-212.
17. **"Missing ledger stratum" as a fallback trigger and FROB-REF-821.** Done: no frozen envelope is always 821;
    the fixed 30-pair fallback applies when the envelope exists and the stratum's table or prior does not
    (N16b-05, BD-263).
18. **GATE-FID-06 on the pyramid exploit.** A traction exploit need not create net energy. Done: any flag of the
    created-energy audit's three monitors counts for that plant, and which one fired is logged (N16b-33).
19. **Exit code of the 72x refusals.** The envelope's table maps 72x to exit 5 (capability), although 721 and
    722 are capacity refusals. Done: the table as written, asserted in N16b-35.
20. **Promotion wall-clock at M2.** GATE-FID-03 asks for promotion wall-clock percentiles at the M2 exit, while
    `eval.promote` is an M3 verb. Left to BD-213 (U16a); BD-263 records the percentiles once promotions exist.
21. **Placement under the layer rule.** No crate of layers 1 to 3 may hold an estimator or a scheduler, while
    `frob-dyn` is described as owning the trust ladder. Done: pure pair, defect and sampling functions in
    `frob-dyn`; estimators and decisions in `frob-opt`; scheduling in the daemon; rows in `frob-chronicle`.
