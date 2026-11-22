# Coverage of drafting unit U29b

Chapter §29 (Chronicle: telemetry, replay, evidence), subsections §29.9 to §29.22, plus the chapter's
counterexample registry `N29-CX` and refusal registry `N29-REF`. The unit owns no seed bead. 26 bead files: 24 new
beads (`N29b-01` to `N29b-24`) and 2 registry beads.

How to read this file. Most of the unit's subsections are also the subject of seed beads that the brief assigns
to other units, which were told to write those beads completely (BD-092, BD-095, BD-208, BD-209, BD-210 in U29a;
BD-094 in U24a; BD-096 in U33a; BD-263, BD-264 in U16b). Those units were drafting at the same time. Once their
files existed, this unit's beads were re-scoped to what the seed beads leave open: the late subsection §29.22
(which no seed covers), the five open questions, the invariant and proof-obligation suites, the two registries,
the batched-tier failure monitors, the Chronicle side of the tier-correction ledger, and the edge cases of the
energy ledger, the citation rule and retention. Five drafts that duplicated beads of other units were removed
(see section (e), item 1), and the surviving beads use the other units' names for scripts, tables and fields.

## (a) Subsections and the beads that carry them

| Subsection | Title | This unit's beads | Beads of other units that carry the rest |
|---|---|---|---|
| §29.9 | The energy ledger (GOAL-11) and the audit for created energy | N29b-01 (phase attribution at events and for bodies without legs; the partition law; negative cells; cost-of-transport corner cases; per-actuator signed parts and bus circulation; channel kinds of energy quantities; per-task figures with no completion and with numerical outcomes; the residual of an aggregate). N29b-02 (the created-energy rule as a host evaluator on the referee at M1: residual, window, thresholds, integer predicate, what a flag does). N29b-03 (the two contact-level monitors: work in relative velocity for moving surfaces, host oracles, kernel differential, pooled and per-contact shares). N29b-04 (flags on the episode record, run-level counts, the flag join for comparisons, the audit-off control). N29b-06 (the assembled gate record) | BD-092 (U29a: lines, identities, accumulation on the referee, breakdowns, cost of transport, the three refinements, first offending event, GATE-CHRON-03 referee). BD-208 (U29a: `K-ledger-step`, single precision). N29a-22 (U29a: the three monitors on batched tiers, `fidelity.energy`, energy half of GATE-CHRON-08). N24a-37 (U24a: `telemetry.energy`). N24a-18 (U24a: cost-of-transport and per-task metrics). N16b-14, N16b-15, N16b-33 (U16b: step-side accumulation, planted exploits, GATE-FID-06). BD-346, BD-266 (A-19 typing), BD-267 (detectors) |
| §29.10 | Failure taxonomy | N29b-05 (monitors on batched tiers: first-anomaly latches by integer minimum, level-T1 events, terminal kinds, the ring-flush trigger, receipts, differential against host monitors). N29b-06 (failure half of GATE-CHRON-08 on batched tiers) | BD-094 (U24a: classes, kinds, first cause, `telemetry.failures`, referee monitors at M1). N24a-38 (U24a: the ten later world and perception classes). N24a-39 (U24a: referee rows, script). BD-191 (stills at −0.5 s, 0 s, +0.2 s) |
| §29.11 | The tier-correction ledger | N29b-07 (the store: stratum key, pair records, exact integer moments, derived row with ρ_ep, β, c and the ceiling 1/(1 − ρ_ep²), referee reach and promotion wall-clock at the M2 exit, unconditional recording, the rule that segment results never enter). N29b-08 (audit rounds: recomputed addressed audit bits, the floor 1/20, write-order enforcement of INV-CHRON-13, the FROB-REF-811 guard, the derivation for PO-CHRON-02) | BD-264 (U16b: the estimator, confidence sequence, budget of 256, switch-off rule, GATE-CHRON-09, GATE-FID-08). BD-263 (U16b: `eval.promote`). N16b-07 (pilot: ρ on 200 addresses). N16b-08 and N16b-09 (U16b: multilevel and segment modes; the segment pilot and its falsifier). BD-214, N16b-16, BD-268 (U16b: the background sampler [F], its pilot, its gate). N16b-10 (U16b: the KILL-DR decision record). N16b-31 (U16b: `promotion.sh`) |
| §29.12 | Summaries an agent can afford | N29b-15 (INV-CHRON-11 as a schema-driven test over every summarizing verb). N29b-04 (flag counts printed as "k of n"). N29b-10 (a number that supports a claim carries its `claim_id`) | N24a-35 (U24a: `telemetry.summary`, budget, hierarchy, handles, counts identity, frontier, compact encoding). N24a-41 (U24a: `telemetry.export-html`, the replay packet writer). BD-191 |
| §29.13 | The artifact graph: every claim traces to its evidence | N29b-09 (decision OPEN-CHRON-3 and the text forms of the four identifiers). N29b-10 (the citation guard on every surface: claim states, the citing operations, read-time class check, unscoped callers, re-affirmation of recorded PASS rows) | BD-095 (U29a: identifiers, recording, closure, resolution rule, build-time registry check, `evidence.trace`). N29a-10 (U29a: GATE-CHRON-10, `evidence_trace.sh`). N38b-18, N38b-19, N38b-20 (U38b: the gate registry record, the recorder, status views). N34-09, N34-12 (U34: demonstration files and `demos.sh`). FD-FKER-01 (upstream identifier types) |
| §29.14 | A structured log per bead | N29b-10 (the closing line's claims go through `cite`; re-affirmation appends the demotion line). N29-REF (FROB-REF-815) | N29a-09 (U29a: schema `frob.beadlog/1`, append, chain, closing-rule evaluator, FROB-REF-815, status inputs). N38b-03 (U38b: close audit, shared close reasons). N38b-20 (status views) |
| §29.15 | Simulation testing of the daemon and the ledger | N29b-11 (second wave: pagination over the surface cursor, retention against verdicts, the memo store, shard takeover under both lease designs; await-point coverage on M2 code; dual runs against `batch-cpu`; per-commit, nightly and release lanes; regression rules; the discharge record of PO-CHRON-01) | BD-096 (U33a: harness, fake device, fault menu, oracles, lineage-driven selection, minimization, dual runs, CPU part of GATE-CHRON-11). N29a-06 (U29a: GATE-CHRON-04 CPU rows). BD-210, N29a-24 (U29a: device rows of GATE-CHRON-04 and -11). BD-223 (kernel-batch chaos) |
| §29.16 | Retention | N29b-12 (decision OPEN-CHRON-4: windows, budgets, eviction order, over-budget behaviour, `store.toml`). N29b-13 (sweep re-checks pins after the mark; every claim writer pins and the coverage audit; withdrawal as a surface with dependents and gate rows; holder life cycles and orphan pins). N29b-14 (privacy class of telemetry through placement, reads, exports, views, tombstones, logs and purge) | BD-209 (U29a: classes, pins with verdicts, `withdraw`, mark, sweep, tombstones, gaps, `store.status`, `store.collect`, GATE-CHRON-12, `store_retention.sh`). N29a-04 (U29a: pinned cursors). BD-305 (private places, `scene.purge`) |
| §29.17 | Refusals | N29-REF (eleven codes with trigger, ranked repairs, retry class, exit code, raiser and fixture; the cross-reference to FROB-REF-582 and the tombstone of 882; three overlaps with surface codes; the uncoded refusal of §29.22) | Each code's mechanism bead, named in the registry's last column |
| §29.18 | Invariants and proof obligations | N29b-15 (INV-CHRON-01 to -13: one property test and one planted violation each, staged by milestone). N29b-16 (PO-CHRON-01 to -07: discharge records; derivations for PO-03 and PO-04 written out; the tests specific to PO-03 to PO-06). N29b-08 (PO-CHRON-02's derivation; INV-CHRON-13). N29b-11 (PO-CHRON-01's record) | Mechanism beads of U29a, U27a, U16b that test their own invariant (listed in U29a's coverage file) |
| §29.19 | Known answers and negative controls | N29-CX (the fourteen negative controls of the section, each with gate and carrier; CX-319 to CX-325; six appended cases). Known answers are cited with their values in N29b-01 (KA-708), N29b-02 and N29b-03 (KA-709, KA-710), N29b-08 (KA-719), N29b-10 (KA-720), N29b-11 (KA-711, KA-712), N29b-16 (KA-713, -714, -717, -718) | N29-KA (U29a: the registry of KA-707 to KA-720). N38b-34 (U38b: controls owed by Chronicle rows) |
| §29.20 | Gates and kill gates | See table (b). Measured or supported here: GATE-CHRON-08 (N29b-05, N29b-06), -09 (N29b-08, store rows), -11 (N29b-11), -14 (N29b-23, N29b-24), KILL-DR (N29b-07, the measurement) | See table (b) |
| §29.21 | Open questions | N29b-17 (OPEN-CHRON-1, experiment and decision). N29b-18 (OPEN-CHRON-2). N29b-09 (OPEN-CHRON-3). N29b-12 (OPEN-CHRON-4). N29b-19 (OPEN-CHRON-5) | The beads blocked by each decision state the fail-closed default they build on (BD-091, BD-095, BD-209, BD-210, N29a-06, N29a-10, N29a-11, N29a-24) |
| §29.22 | The solver's kinetic-energy correction is not heat (added late; no seed) | N29b-20 (`frob-contact`: row classes, stage energy S, whole-interval work W, algorithmic exchange X, constitutive dissipation by law component, remainder). N29b-21 (`frob-dyn`: the three row velocities, the predictor side, the cancellation identity with proof, events, joint-limit, damping, stiction and backlash rows, the restore set). N29b-22 (`frob-chronicle`: line kinds, the line with both sides, the identity check, the binding rule for thermal and wear states, the refusal, the `ConstitutiveHeat` type). N29b-23 (GATE-CHRON-14 on the referee: three binary-exact fixtures at 32, 64 and 2,000 Hz, the thermal test node, the mutant, the record). N29b-24 (batch tiers: the two row quantities in `K-ledger-step`, the `strict` rows, lockstep rows, device differential) | BD-092 and BD-208 (U29a) reserve the ledger line; N24a-37 (U24a) prints the two lines |

End-to-end beads of this unit: N29b-06 (`gate_chron_08.sh`), N29b-23 with N29b-24 (`gate_chron_14.sh`). Beads that
add named step groups to another unit's script instead of creating a second script for the same capability:
N29b-01 and N29b-02 (`energy_ledger_referee.sh` of BD-092), N29b-03 (`energy_audit.sh` of N29a-22), N29b-08
(`promotion.sh` of N16b-31), N29b-09 and N29b-10 (`evidence_trace.sh` of N29a-10), N29b-12 and N29b-13
(`store_retention.sh` of BD-209). Scripts owned by test and registry beads of this unit:
`chronicle_tier_ledger.sh` (N29b-07), `chronicle_simtest.sh` (N29b-11), `chronicle_privacy.sh` (N29b-14),
`chronicle_invariants.sh` (N29b-15), `chronicle_proof_obligations.sh` (N29b-16), `chronicle_column_codecs.sh`
(N29b-17), `chronicle_chunk_len.sh` (N29b-18), `chronicle_lease_two_device.sh` (N29b-19),
`chronicle_refusals.sh` (N29-REF), `chronicle_controls.sh` (N29-CX).

Pilot beads: none in this unit. The two Frontier mechanisms of §29.11 have their pilots in U16b: N16b-16 (the
background referee) and N16b-09 (segment disagreement against episode-level exceedance, with the falsifier of
§29.11). Every mechanism this unit drafts is tier Solid.

## (b) Gates, kill gates and admission gates of §29.20 and §29.22

| Gate | Milestone (§40 governs) | Measured or decided by this unit | Expected from, or already written by, another unit |
|---|---|---|---|
| GATE-CHRON-01 | M2 | — (INV-CHRON-08's module in N29b-15) | BD-203, N29a-19 (U29a) |
| GATE-CHRON-02 | M2 | — (N29-CX proposes the missing negative control) | BD-204 (U29a) |
| GATE-CHRON-03 | M1 referee; M2 single precision | — (N29b-02 supplies the certifier row "stiff penalty bounce raises `energy-created`"; N29b-01 the per-task recorded check) | BD-092, BD-208 (U29a) |
| GATE-CHRON-04 | M1 CPU; M2 device | — (N29b-18 and N29b-19 are the decisions that block it; N29b-11 re-runs its oracles on M2 subjects) | BD-091, N29a-04, N29a-06, BD-210, N29a-24 (U29a) |
| GATE-CHRON-05 | M2 | — (INV-CHRON-07's module in N29b-15) | BD-205 and U27a's gate beads |
| GATE-CHRON-06 | M2 | — (PO-CHRON-05's trials in N29b-16) | BD-206, N29a-20, N29a-21 (U29a) |
| GATE-CHRON-07 | M2 | — (PO-CHRON-03 and -04 in N29b-16; INV-CHRON-05 and -06 in N29b-15) | BD-207, N29a-12 to N29a-16 (U29a) |
| GATE-CHRON-08 | M2 (referee failure rows at M1) | N29b-05 (failure half on batched tiers), N29b-06 (the comparison control, the control sweep, the one gate row) | N29a-22 (U29a: energy half); BD-094, N24a-39, N24a-38 (U24a: referee failure rows and later classes) |
| GATE-CHRON-09 | M3 | N29b-08 (store-level rows: exact equality at n = N read back from a stored round; the after-the-fact audit plant refused) | BD-264 (U16b: the statistical rows and the record) |
| GATE-CHRON-10 | M1 | — (N29b-09 is the decision that blocks it; N29b-10 adds surface and re-affirmation steps to its script) | BD-095, N29a-09, N29a-10 (U29a) |
| GATE-CHRON-11 | M1 CPU; M2 device | N29b-11 (simulation rows for the M2 subjects; the release lane) | BD-096 (U33a: CPU part); BD-210, N29a-24 (U29a: device rows) |
| GATE-CHRON-12 | M2 | — (N29b-12 is the decision that blocks it; N29b-13 adds race, withdrawal and orphan steps to its script) | BD-209 (U29a) |
| GATE-CHRON-13 | M2 | — (N29b-05 and N29b-24 state that their accumulators must fit its budget) | BD-207, N29a-17 (U29a) |
| GATE-CHRON-14 | M1 referee; M2 batch tiers (not in the §40 main table; listed in §40.8) | N29b-23 (referee rows, the gate row itself), N29b-24 (`strict` and lockstep rows) | — |
| KILL-DR | M3 | N29b-07 (records ρ_ep per stratum on at least 200 paired addresses, and keeps recording after the fallback) | N16b-10 (U16b: the decision record); BD-264 (the coverage measurement) |

No kill-gate decision bead was written here: KILL-DR's decision record already exists as N16b-10, with the
trigger and fallback of §29.20 as the governing statement (the register's resolution in §40.7). No admission gate
is defined in these subsections.

## (c) Counts

- Known answers: the chapter's 14 rows (KA-707 to KA-720) are registered by `N29-KA` (U29a). Cited with their
  values in this unit's test plans: 13 of 14 (KA-707 to -714 and -716 to -720); KA-715 (impulse downsampling)
  belongs to BD-090 and is carried here as CX-322 with a numeric case. Known answers of §29.22 that have no row
  in Appendix M: 3, stated in N29b-20, N29b-22, N29b-23 and filed for the registry through N29-CX. Known answers
  of other chapters cited: KA-354, KA-765.
- Counterexamples: all 7 of the chapter (CX-319 to CX-325) in `N29-CX`; 6 appended cases (C1 to C6) from §29.9,
  §29.10, §29.16 and §29.22; the 14 negative controls of §29.19 each with a gate and a carrier. Cited from other
  chapters: CX-147, CX-148, CX-150, CX-346.
- Refusal codes: all 11 of the chapter (811, 813, 815, 869, 875, 878, 885, 888, 894, 896, 898) in `N29-REF`;
  FROB-REF-582 cross-referenced and 882 tombstoned; 1 uncoded refusal (§29.22) recorded with a proposal. Raised
  by this unit's mechanism beads: 811 (N29b-08), 813 and 869 (N29b-13, N29b-10), 888 (N29b-01, N29b-04,
  N29b-12). Codes of other chapters used: 720, 731, 734, 806, 807, 810, 820, 865.
- Invariants: INV-CHRON-01 to -13, all in N29b-15; also carried in mechanism beads: -06 (N29b-01, N29b-03,
  N29b-04, N29b-07), -09 (N29b-01, N29b-22), -10 (N29b-09, N29b-10), -12 (N29b-13), -13 (N29b-08). One invariant
  stated without a number in §29.22 (thermal and wear states take input only from constitutive dissipation and
  electrical loss) is carried by N29b-22 under the proposed name INV-CHRON-14.
- Proof obligations: PO-CHRON-01 to -07, all in N29b-16; -01 also N29b-11; -02 also N29b-08.
- Architecture decisions of §29.2 restated where they govern: ADR-CHRON-02 (N29b-18), -03 (N29b-01, N29b-02,
  N29b-03, N29b-05), -04 (N29-CX), -07 (N29b-10), -08 (N29b-01, N29b-02, N29b-04, N29b-22), -09 (N29b-12,
  N29b-13). ADR-CHRON-01, -05, -06 belong to §29.2's beads in U29a, U28b and U27a.
- Open questions: 5 of 5, each with its own decision or experiment bead and its fail-closed default.
- Risks named: RISK-03, RISK-05, RISK-12, RISK-15, RISK-39, RISK-54; premortem stories 5 and 8 of §41.1.
- Verbs: this unit implements no verb of Appendix J from scratch (`telemetry.energy`, `telemetry.failures`,
  `telemetry.summary`, `telemetry.export-html`, `evidence.trace`, `store.status`, `store.collect`,
  `eval.promote`, `fidelity.energy` are all in beads of U24a, U29a and U16b). It proposes one option on an
  existing verb: `store.collect --withdraw-claim` (N29b-13).

## (d) Needs from other slices

Prerequisites named in `needs` for which no seed key could be used (the keys in brackets are beads of other units
that exist in the definitions tree and could not be listed under `deps` by the charter's rule):

| Bead | Need |
|---|---|
| N29b-01 | §14: named limbs and designated contact sites in the compiled model; §24.8: the distance and mass of the cost-of-transport metric [N24a-18, N24a-22]; §15.11: braking-mode lines of the actuator account |
| N29b-02 | §16.10: the per-step residual, its positive part by contact mode and the storage rule [N16b-14]; the M2 monitors [N29a-22] |
| N29b-03 | §29.9 at M2: in-kernel counters, flags and `fidelity.energy` [N29a-22]; §18.6: the labelled pyramid cone [N16b-15]; §18.4: persistent feature-pair identity |
| N29b-04 | §16.8: the label algebra and FROB-REF-820 [N16b-04]; consumers A-19 (BD-346, M4) and the detectors (BD-267, M3) |
| N29b-05 | §21: registration of one monitor kernel in the last stage of the batched step, and a name for it in the kernel registry (`K-x-monitor` is a proposal); §29.5: the device ring [N29a-11] |
| N29b-06 | The energy half's script [N29a-22]; the planted solver mutations [N16b-15]; the control registry's second-milestone wiring (BD-223) |
| N29b-07, N29b-08 | §16.8: the promotion pipeline and estimator (BD-263, BD-264 are M3 seeds that consume these stores; they could not be made to depend on N29b keys from here); §16.11: background pairs (BD-214); §28.6: DC-01's exceedance bits reading the same frozen round |
| N29b-09 | §34: layer and dependency rules for `frob-core` and the auditor crate; §33: the stack lock's pin and the single-version rule |
| N29b-10 | §37.10: the claim product at the boundary; §38.13: the gate recorder [N38b-18, N38b-19]; §29.14: the closing-rule evaluator [N29a-09] |
| N29b-11 | §32.11: the dispatcher (BD-296, M3), whose scenarios join later; §38.9: kernel-batch chaos (BD-223) |
| N29b-12 | §42.2: the owner's decision; §37.15: the companion defaults of OPEN-SAFE-4 |
| N29b-13 | §28: verdict, prediction and qualification writers that must issue claims through the pinning transaction; §27, §28.7: open comparisons, bisects and autopsies as holders; Appendix J: a registered surface for claim withdrawal |
| N29b-14 | §37.6: the private mark, scopes, export flag and `scene.purge` (BD-305); §32.24: the replay packet [N24a-41] |
| N29b-15 | §29.11: INV-CHRON-13's subject exists at M3; §29.12: the summary verbs [N24a-35] |
| N29b-16 | §29.11: the statistical half of PO-CHRON-02 at M3 (BD-264) |
| N29b-17 | §12: the auditor's standard-library check must survive any encoding (BD-026); §34.9: the format-version rule |
| N29b-18, N29b-19 | The beads these decisions block (BD-091, BD-207, BD-210, N29a-06, N29a-11, N29a-24) carry `needs` for the decisions; the lead should make them depend on N29b-18 and N29b-19 |
| N29b-20, N29b-21 | §18.5, §18.6: each contact law's dissipation and stored-energy functions; §15.6: the row descriptors of stiction, backlash and hard stops |
| N29b-22 | §15: thermal networks and the declaration of their heat inputs (BD-071, BD-072; later tyre and film nodes); §31.14: wear states; Appendix O: a code for the uncoded refusal |
| N29b-23 | §40: a row for GATE-CHRON-14 in `registries/gates.toml`; Appendix M: rows for the three known answers of §29.22 [N29-KA] |
| N29b-24 | §18.5: the hard-contact batch law (BD-386, M5); §21: the kernel differential harness of the device beads |
| N29-REF | §32.4, §32.18: exit codes and the overlapping surface codes [N38-REF and the surface chapter's registry bead] |
| N29-CX | Appendix M: the three rows of §29.22 [N29-KA] |

Dependencies the lead should add in the other direction (beads of other units that need a bead of this unit and
could not name it): BD-095 and N29a-10 on N29b-09; BD-091 and N29a-06 on N29b-18; BD-210 and N29a-24 on N29b-19;
BD-209 on N29b-12; BD-264 on N29b-07 and N29b-08; N29a-11 and BD-207 on N29b-05 (the failing bit); N24a-37 and
BD-208 on N29b-22 (the two lines); N29a-22 on N29b-02 and N29b-03.

## (e) Contradictions, gaps and what was done

1. **Overlap with units that write seed beads on these subsections.** Three units (U29a, U24a, U16b) were told to
   write completely the seed beads that draw on §29.9 to §29.16, and U38b and U34 own the gate recorder, the
   close audit and demonstration files. After their files appeared, five drafts of this unit were removed as
   duplicates: a second energy-ledger breakdown bead (merged into N29b-01 as edge cases of BD-092); a build-time
   check of gate rows and demonstration files (carried by BD-095, N29a-10, N34-09, N34-12, N38b-19); an
   end-to-end script for GATE-CHRON-10 (N29a-10); a bead-log closing-rule bead (N29a-09, N38b-03, N38b-20); an
   end-to-end script for GATE-CHRON-12 (BD-209). Four beads still overlap a bead of another unit on purpose,
   each holding the cases the other leaves open, and each says in its Considerations what to do if the lead
   merges the pair: N29b-01 with BD-092; N29b-02 and N29b-03 with N29a-22; N29b-10 with BD-095; N29b-13 with
   BD-209. Nothing in them contradicts the other bead as far as could be checked; where a number or name
   differed it was aligned to the seed bead's (the 1 nJ ledger quantum, `EnergyFigure`, the pooled 1% rule, the
   stiff-penalty fixture, pin reason kinds, tombstone reasons, script names).
2. **A script name is used twice by other units.** `scripts/e2e/energy_audit.sh` is the script of N16b-33
   (GATE-FID-06) and of N29a-22 (energy half of GATE-CHRON-08). N29b-03 and N29b-06 refer to the second. One of
   the two needs another name.
3. **§29.22 gives no per-row formula.** It states the concept, the laws and three known answers. N29b-20 and
   N29b-21 propose definitions (DESIGN): for an impulse-level row, stage energy S = λ·½·(u* + u⁺), whole-interval
   work W = λ·½·(u⁻ + u⁺), algorithmic exchange X = S − W = λ·½·(u* − u⁻); the predictor side
   X_pred = h·fᵀ·½·(v* − v⁺); and the identity X_pred + Σ X = 0 in every interval, proved. They reproduce the
   plan's numbers exactly (1 J/s and 0.5 J/s; 4 W; 2 J). Rows whose force is a function of state are given zero
   exchange by definition, which is the plan's statement for a resting compliant contact; whether a compliant row
   may have a non-zero exchange at touchdown is recorded as an open reading in N29b-20.
4. **GATE-CHRON-14's tolerances need constructed fixtures.** "Exactly 0" and "within 10⁻⁹ J of 2 J" hold only
   with binary-exact parameters (which is presumably why the text uses g = 8 m/s² and rates of 32 and 64 Hz) and
   with the impact as a located event at a prescribed approach speed; a drop integrated by a first-order step
   does not arrive at exactly 2 m/s. N29b-23 defines the fixtures that way and reports the free-drop variant as a
   companion row. On the lockstep tiers impacts are not located, so N29b-24 reads "on `referee` and `strict`"
   literally and reports the lockstep impact as a fidelity row.
5. **The gate's "film temperature" does not exist at M1 or M2.** The film-friction law is a third-milestone seed.
   N29b-23 uses a lumped thermal test node at the contact interface, labelled `synthetic`, and re-points the
   mutant at the real node when it exists.
6. **A refusal without a code, an invariant without a number.** §29.22 names the refusal "non-physical energy
   bound to a physical state" and states an invariant, and neither appears in §29.17, §29.18 or Appendix O.
   N29-REF records the refusal with a proposal for the `8xx` block and no interim code; N29b-22 carries the
   invariant under the proposed name INV-CHRON-14. GATE-CHRON-14 itself is in §40.8 only, and Appendix M has no
   rows for the subsection's three known answers.
7. **Two codes for one event, twice; no exit codes.** FROB-REF-878 against FROB-REF-810 (different bytes for a
   committed chunk) and FROB-REF-896 against FROB-REF-806 (cursor mismatch), with FROB-REF-807 for an expired
   cursor; §29.17 has no exit-code column. N29-REF assigns exits by the surface's rule (875 → 7 and 878 → 9 are
   judgements, the rest 3) and proposes `surfaces_as` for the first pair and a role split for the second, in
   agreement with what U29a's mechanism beads do. The registry's owner must rule.
8. **Two printed forms of a claim identifier.** `clm:…` in the gate registry's example (§38.13) and `c:…` in the
   bead-log example (§29.14); `c:` is also the prefix of a controller identity in envelopes. BD-095 and N29a-09
   follow the bead-log example. N29b-09 makes the text form part of the identifier decision and proposes `clm:`.
9. **"More than 1% of stance time" has three readings** for `cone-excess`: pooled over (contact, tick) pairs,
   per contact, or per world-tick. N29a-22 uses the pooled reading; N29b-03 follows it and records the
   per-contact share beside it, with the smallest case in which they differ (0.75% against 3%), so the
   re-baseline at M2 can decide.
10. **"10⁻⁶ relative on the referee"** is read as the same window and denominator as the fast-tier rule with the
    threshold 10⁻⁶ (N29b-02); the plan says no more than the number.
11. **Claim withdrawal has no verb.** §29.16 and FROB-REF-813 make "withdraw the claim first" the repair, and
    Appendix J has no verb for it. N29b-13 proposes an option on `store.collect` and states the transaction so
    that another surface can replace it.
12. **Purge meets pins.** §37.6's `scene.purge` removes a lineage closure; §29.16 forbids removing a pinned
    object. N29b-14 resolves it fail closed: purge lists the claims it would break and is refused with 813
    unless asked to withdraw them in the same operation.
13. **Failure monitors on batched tiers are not specified.** §29.10 defines monitors as readers of telemetry, and
    control-rate traces exist only for sampled and failing worlds on devices, so the first anomaly of an
    unsampled world would be lost. N29b-05 proposes in-kernel first-anomaly latches updated by integer minimum
    (licensed by ADR-CHRON-03) and a monitor kernel whose name needs the kernel registry's confirmation.
14. **The promotion wall-clock "reported at the M2 exit"** precedes the promotion verb (M3). N29b-07 times a dry
    promotion (the referee half of 64 addresses per stratum) at M2 and labels the projection Estimated.
15. **Per-task energy with numerical outcomes.** §29.9 defines the two figures for successes and failures only.
    N29b-01 excludes numerical, budget and protocol outcomes from both numerators and prints them in an exclusion
    block (FROB-REF-865 otherwise); the alternative is stated in the bead.
16. **§29.21 gives OPEN-CHRON-4 one pair of numbers** (30 days, 64 GB) while §29.16 gives the memo store its own
    64 GB. N29b-12 treats them as two pools by default and asks the owner to say so explicitly.
17. **The plan states no negative control for teleport** (KA-718). A gate without a control cannot record PASS,
    so N29-CX proposes one (comparison of whole-step digests only, which names the step and not the stage) for
    GATE-CHRON-02's owner.
18. **INV-CHRON-11 has no gate.** Its control is attached to GATE-CHRON-10's run as a recorded check (N29b-15).
