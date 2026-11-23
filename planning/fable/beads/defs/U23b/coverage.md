# Coverage: drafting unit U23b (§23.14 to §23.29 of Cortex)

50 bead files: 7 seed beads (BD-218, BD-267, BD-301, BD-320, BD-324, BD-328, BD-439), 2 registries (N23-CX,
N23-REF) and 41 new beads (N23b-01 to N23b-41). The other unit of the chapter (U23a) owns §23.1 to §23.13 and the
known-answer registry N23-KA.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §23.14 | A geometric-algebra tree policy [M] | BD-439 (epic, GATE-CORTEX-17), N23b-01 (pilot), N23b-02 (KILL-GATREE, first reading) |
| §23.15 | Randomization from cards [S] | BD-200 (seed of another unit: the card-interval rule and draws by address), N23b-03 (resolved values, physical against numerical, mean and CVaR, held-out, `beyond-card`), N23b-04 (adaptive randomization [F]) |
| §23.16 | Detecting exploitation (GOAL-16), and rank robustness of finalists [S] | BD-267 (`eval.detect`, flags, GATE-CORTEX-04), N23b-05 (harness; dt, tier, law, tolerance, stiffness), N23b-06 (parameter, rate and rotor inertia, mirror, leak, energy), N23b-07 (planted exploits and honest policy), N23b-08 (rank robustness on trained finalists, FROB-REF-830, cost, demotion rule, GATE-CORTEX-15); the verb `fidelity.modelform` itself is BD-266 (seed of another unit) |
| §23.17 | Evaluation protocol [S] | N23b-09 (held-out scenes, five seeds, interquartile mean with stratified bootstrap, estimated-state twin, matched compute), N23b-10 (one study type, pairing and determinism, fixed evaluation episodes, holdout retirement, acceptance elsewhere, method-gate form) |
| §23.18 | The Gymnasium-compatible adapter [F] | BD-301 (adapter, clauses 1 and 2 of GATE-CORTEX-16), N23b-11 (pilot), N23b-12 (shared-device views), N23b-13 (external PPO, clause 3, final kill reading), N23b-14 (KILL-GYMADAPT first reading and the whole-rollout fallback), N23b-27 (transport and binding decision) |
| §23.19 | Refusals | N23-REF (12 codes and 3 late refusals); raising sites named per row |
| §23.20 | Invariants and proof obligations | N23b-17 (all 13 as a battery); INV-CORTEX-05 also N23b-15; INV-CORTEX-06 also BD-328; INV-CORTEX-04 also BD-320; PO-CORTEX-04 also N23b-31, N23b-32; PO-CORTEX-05 also N23b-12 |
| §23.21 | Known answers and negative controls | N23b-18 (the 15 fixtures, KA-586, KA-592, KA-593 and 5 late fixtures as one battery), N23-CX (controls) |
| §23.22 | Pilots and ceilings for the Frontier and Moonshot items | N23b-19 (family policy), N23b-20 (inheritance), N23b-21 (distillation and behaviour cloning), BD-324 (other algorithms), N23b-22 (equivariant networks), N23b-11 (Gymnasium adapter), N23b-01 (geometric-algebra tree policy); the rank-agreement pilot row is BD-310 with BD-145 (seeds of other units); BD-320 depends on N23b-19 |
| §23.23 | Gates and kill gates | N23b-15 (the behaviour-not-plumbing rule, FROB-REF-544), N23b-16 (all 22 gate rows and 9 kill-gate rows with thresholds and carriers); decisions N23b-02, N23b-14, N23b-23, N23b-37 |
| §23.24 | Open questions | N23b-24 (OPEN-CORTEX-1), N23b-25 (OPEN-CORTEX-2 and OPEN-CORTEX-5), N23b-26 (OPEN-CORTEX-3), N23b-27 (OPEN-CORTEX-4, with OPEN-WS-5 and the program question on the adapter's status), N23b-28 (OPEN-CORTEX-6) |
| §23.25 | The joint law of sampled actions (late) | N23b-29 (sampler, manifest, refusal), N23b-30 (battery, planted sampler, GATE-CORTEX-18) |
| §23.26 | The actor's information contract covers every field it can read (late) | N23b-31 (contract, `ActorView`, digest), N23b-32 (four planted leaks, docking known answer, GATE-CORTEX-19); adapter part in BD-301 |
| §23.27 | A threshold crossing is not attainment (late) | N23b-33 (procedure, `train.status` fields), N23b-34 (`eval.preanalyze` cadence mode, GATE-CORTEX-20), BD-218 (GATE-CORTEX-05 under the rule) |
| §23.28 | The winner check for a selecting rung (late) | N23b-35 (containment rank, regret, retention refusal, GATE-CORTEX-21), BD-328 (the record that uses it) |
| §23.29 | Short-horizon gradient training with design parameters (late) | N23b-36 (pilot), N23b-37 (KILL-SHORTGRAD first reading), N23b-38 (learner, verb argument), N23b-39 (three guards, GATE-CORTEX-22, final kill reading) |

Seed beads that draw on subsections of the other unit and were written completely here: BD-320 (§23.11
architecture, pretraining distribution, envelope, membership; with N23b-40 tokens and N23b-41 network) and BD-328
(§23.11's rule for using a family policy as an evaluation rung, with §28.10's ledger and modes).

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|
| GATE-CORTEX-01 | N23b-15 supplies the record type and checker | BD-198 measures it |
| GATE-CORTEX-02 | — | BD-196 |
| GATE-CORTEX-03 | pilot input N23b-22 | BD-326 |
| GATE-CORTEX-04 | BD-267, with N23b-05, N23b-06, N23b-07 | — |
| GATE-CORTEX-05 | BD-218 (state of the comparator: N23b-28; scoring rule: N23b-33) | — |
| GATE-CORTEX-06 | — | BD-192 |
| GATE-CORTEX-07 | rule P-12 added by N23b-29, N23b-30 | BD-197 |
| GATE-CORTEX-08 | — | BD-199 |
| GATE-CORTEX-09 | — | BD-193 |
| GATE-CORTEX-10 | — | BD-255 |
| GATE-CORTEX-11 | pilot input N23b-19; mechanism BD-320 | BD-321 |
| GATE-CORTEX-12 | BD-328 (M4, on our engine) | BD-310 (M3 pilot record) |
| GATE-CORTEX-13 | pilot input N23b-20; schedule decision N23b-26 | BD-322 |
| GATE-CORTEX-14 | N23b-15 (shuffled-label control in the record); pilot input N23b-21 | BD-323 |
| GATE-CORTEX-15 | N23b-08, N23b-07 | BD-266 (the verb's own gate row) |
| GATE-CORTEX-16 | BD-301 (clauses 1 and 2), N23b-13 (clause 3) | — |
| GATE-CORTEX-17 | BD-439 | — |
| GATE-CORTEX-18 | N23b-30 (sampler N23b-29) | — |
| GATE-CORTEX-19 | N23b-32 (types N23b-31) | — |
| GATE-CORTEX-20 | N23b-34 (procedure N23b-33) | — |
| GATE-CORTEX-21 | N23b-35 | — |
| GATE-CORTEX-22 | N23b-39 (learner N23b-38) | — |
| GATE-ATEL-07, GATE-REFLEX-04 (named in the done-condition of BD-328) | BD-328 | their defining chapters' beads |
| KILL-ALTALGO | N23b-23 (pilots BD-324) | — |
| KILL-DISTILL | pilot reading N23b-21 | BD-323 decides |
| KILL-EQUIV | pilot reading N23b-22 | BD-326 decides |
| KILL-FAMILY | pilot reading N23b-19; BD-320 gated on it | BD-321 decides |
| KILL-GATREE | N23b-02 (first reading), BD-439 (final) | — |
| KILL-GYMADAPT | N23b-14 (first reading and fallback), N23b-13 (final) | — |
| KILL-INHERIT | pilot reading N23b-20 | BD-322 decides |
| KILL-LEARNKERN | BD-218 supplies the measurement for its second clause | BD-196 decides |
| KILL-SHORTGRAD | N23b-37 (first reading), N23b-39 (final) | — |
| KILL-RANK (for rungs; named in the done-condition of BD-328) | BD-328 | BD-265 for tier pairs |

Every one of the 22 gates and 9 kill gates has its full threshold, milestone and carrier in N23b-16.

## (c) Counts

- Known answers: the 18 numbered rows of the chapter (KA-576 to KA-593) are cited with their values in this unit's test plans and all 18 are executed by N23b-18; the registry bead N23-KA belongs to the other unit. Plus 5 late known answers that have no number (joint law 0.6655; docking exclusion flip; 0.90497, 0.0303, 0.78548 and the counts 36, 46, 74, 103; τ = 41/45 with containment rank 3; growth 13, 167, 7.8 × 10⁸), carried in N23b-30, N23b-32, N23b-34, N23b-35, N23b-38 and N23b-39.
- Counterexamples: 13 numbered (CX-246 to CX-258) and 5 late, unnumbered (provisional CX-23-L1 to L5), all in N23-CX.
- Refusal codes: 12 (FROB-REF-521, 522, 524, 527, 531, 534, 537, 544, 547, 554, 830, 867) and 3 late refusals stated without a code (candidates 558, 559, 570), all in N23-REF.
- Invariants and proof obligations: 7 and 6, all in N23b-17. Open questions: 6 (OPEN-CORTEX-1 to -6) in 5 decision beads.
- Pilots: 7 of the 8 rows of §23.22 (the eighth, rank agreement, is a seed of another unit) and the pilot of §23.29.

## (d) Needs from other slices

- **U23a (§23.1 to §23.13).** BD-198 must build its GATE-CORTEX-01 record with the type of N23b-15 and extend `train.status` with the block of N23b-33. BD-197 must read the default action law from N23b-25 and add rule P-12 (N23b-29, N23b-30) to its battery. BD-322 should depend on the inheritance pilot N23b-20 and the schedule decision N23b-26, and `policy.apply` must call the membership check of BD-320. BD-323 should depend on N23b-21; BD-326 (and BD-325) on N23b-22. Build beads for SAC, TD3, evolution strategies, the privileged-teacher path and asynchronous sharding must depend on N23b-23. Decision records for KILL-EQUIV, KILL-LEARNKERN, KILL-FAMILY, KILL-INHERIT and KILL-DISTILL are expected from the seed beads that carry "decided" in their done-conditions; this unit wrote none for them. N23-KA should add late slots for the five unnumbered known answers above.
- **§23.15 core.** BD-200 (another unit): the card-interval rule and draws by address, on which N23b-03 and N23b-04 build.
- **Fidelity (§16).** BD-266: `fidelity.modelform`, the variant step plans and the pyramid cone; BD-263: `eval.promote` must call the FROB-REF-830 check; BD-212: frozen envelopes; BD-265: the tier-pair form of `eval.rank-agreement` at M3; BD-208: the energy audit and the penetration-recovery variant.
- **Atelier (§28).** BD-329, BD-345, BD-352, BD-346: read k_c(m) and the rank-agreement modes, raise FROB-REF-537 and the retention refusal at allocation, call the finalist check each final round.
- **Arena and studies (§24).** Task-file declarations of access classes and delivery instants and the contract digest in `TaskId` (BD-122, BD-194); the cadence mode in the registration of `eval.preanalyze` (BD-124, BD-222); a refusal for randomizing a numerical parameter; scene splits (BD-235); the qualification lane (BD-259).
- **Sensing (§20).** `sense.causal-test` on every tier hosting the four planted leaks (BD-119, BD-215); the exact docking fixture (BD-190).
- **Mission and memo (§25, §28).** The docking family's verdict (BD-129, BD-134); memo keys and certificate assumption sets that include the contract digest (BD-111, BD-128, BD-133).
- **Dynamics and Phylum.** Step adjoints behind their gate (BD-422) and the paired stencil (BD-353) for §23.29; the generator and split (BD-287), variants on devices (BD-183), the canonical design form (BD-054) for family policies.
- **Spark.** Attention and layer-normalization kernels with backward passes for the morphology transformer; export of device buffers to a second process for the adapter's shared views (BD-172, BD-173).
- **Tether and workspace.** Verb registration for M2, M3, M4 and M6 (BD-222, BD-312, BD-376 and the later set); the job interface for the adapter's fallback (BD-210, BD-295); `check-deps` proving no interpreter in core binaries (BD-003); assignment of three refusal codes; schemas of `gates.toml`, `pilots.toml` and `controls.toml` with stable control identifiers.
- **Performance and ten-fold.** The incumbent harness and its time-to-threshold measurement under the same attainment rule (BD-039, BD-040, BD-142), the phase ledger with the learner's share (BD-143), the rival freeze that pins the external learner (BD-314, BD-315); the D4 training profile and the raising site of FROB-REF-867 (BD-269).
- **Owner.** OPEN-CORTEX-1 (pretraining envelope), the general compute envelope, approval of compute for the rank-agreement experiment, and the adapter's support status.

## (e) Contradictions and gaps found, and what was done

1. **GATE-CORTEX-20 and the word "crossing".** The mechanism defines `crossing` on the point score; the gate's 78.5% belongs to a rule that applies an unadjusted 95% lower bound at every look (30 of 30). With the point-score rule the same policy crosses with probability 0.58 per look and almost surely in 50 looks. Done: both are defined and reported (N23b-33, N23b-34); the gate's "first-crossing rule" is the unadjusted-bound rule.
2. **GATE-CORTEX-20's second clause is vacuous on its own cadence.** With 30 episodes per look the spending rule can never attain against 0.90 (look 1 needs 36). Done: the clause is measured as written and two potency cadences (50 × 120 at true rates 0.90 and 0.95) are added so that the gate can fail (N23b-34).
3. **GATE-CORTEX-18's family-wise rate.** 559 sign statistics at ± 3.16 standard deviations give a correct sampler a 58% chance of leaving a band in a fresh random run. Done: registered seed plans and one confirmation at N = 10⁶ per out-of-band statistic, with the family-wise rate recorded (N23b-30); flagged for the Cortex lead.
4. **Containment rank on populations of another size (§23.28).** k_c(m) is defined on calibration and audit sets and applied to campaigns of other sizes without a scaling rule. Done: rank-quantile scaling as a stated reading (N23b-35); flagged for the ladder's owner.
5. **Three refusals without codes** (§23.25, §23.26, §23.28). Done: candidates 558, 559, 570 in N23-REF, to be assigned by the registry owner.
6. **Overlapping codes.** 830 against 820 and 557 (model-conditional results); 867 against 861 and 825 (D4); 534 against 541; 524 against 442; 554 against 523. Done: a one-condition-one-code reading in N23-REF, flagged.
7. **Two ranges for the ceiling 1/f**: 4 to 50 times (§23.11, f from 0.25 to 0.02) and 5 to 100 times (§23.22). Done: each carried with its f range (BD-320, N23b-19).
8. **Joint placement feature**: log(base_from_joint) in the chapter, log(parent_from_joint) in the construction. Done: both features in the token (N23b-40).
9. **"Missed by more than two times" (KILL-LEARNKERN)** has no definition. Done: read as above twice the gate's limit (2.5 times the incumbent; 1,025 s at the planning value) in BD-218 and N23b-16; flagged.
10. **Units in the mixed-unit known answer**: the suite row writes 12 °C with scale 10 °C, the chapter writes kelvin. Done: kelvin (a margin is a temperature difference).
11. **Claim kinds**: GATE-CORTEX-08 and -14 have two kinds in their chapter rows and one in the register. Done: the register's kind in N23b-16.
12. **`eval.rank-agreement` is an M4 verb, and GATE-CORTEX-21 is M3.** Done: N23b-35 at M3 builds on the tier-pair form (BD-265) and a fixture mode.
13. **The detectors' decision rule** ("a collapse outside the envelope") is not specified. Done: the promotion rule's e-process pair on exceedance bits (N23b-05), labelled DESIGN.
14. **Seven of ten planted exploits** are not given by the plan. Done: designed as synthetic fixtures with numbers (N23b-07), labelled DESIGN.
15. **The host library's autoreset conventions** are not named in the plan. Done: stated as an external fact to be re-read at the pin by the pilot (N23b-11, BD-301).
16. **No count in "an external PPO reaches the threshold"** (GATE-CORTEX-16). Done: at least two of three seeds, with per-seed outcomes always reported (N23b-13).
17. **"Periodic audit" after demotion** has no period. Done: one campaign in five as a default (N23b-08), flagged.
18. **No refusal code for fewer than five seeds.** Done: a label `under-seeded` that gate records and comparisons refuse (N23b-09).
19. **The window loss of §23.29** is written without a discount. Done: the physical discount of the task is applied; β = 0 gives the plan's sum (N23b-38).
20. **Flag defaults in the information contract.** The plan states the default class only for reward and `info` fields. Done: termination and truncation flags and undeclared masks default to trainer-only (N23b-31), flagged.

## (f) Integration pass (group G23: units U23a and U23b)

Changes made when the two units of the chapter were connected; the tables above are the drafting record and are
read with these corrections.

- **Superseded duplicates.** N23b-26 (OPEN-CORTEX-3) is superseded by N23a-29 of the other unit, whose reset
  convention matches the mechanism N23a-28 (one reset in k + 1 children); read N23a-29 for N23b-26 in the rows
  above. N23b-24 (OPEN-CORTEX-1) and N23b-25 (OPEN-CORTEX-2 with OPEN-CORTEX-5) are the kept beads of their
  questions; the other unit's N23a-22 and N23a-09 were folded into them. The Cortex defaults live in
  `registries/defaults.toml`.
- **Kill-gate decisions.** The five kill gates of §23.1 to §23.13 are decided by separate beads of the other
  unit: KILL-LEARNKERN N23a-16 (fallback trainer N23a-17), KILL-DISTILL N23a-21, KILL-FAMILY N23a-26,
  KILL-INHERIT N23a-30, KILL-EQUIV N23a-33. The seed beads measure; N23b-16's carrier map says so. The parity
  clauses of GATE-CORTEX-08 are N23a-19; BD-199 measures the scaling clause.
- **Verbs.** `train.start`, `train.status`, `train.stop` are N23a-14 and `train.export` is N23a-13 (not BD-198);
  `policy.apply` is N23a-25 (not BD-322). N23b-33 and N23b-09 depend on those beads and BD-198 depends on them.
- **Build beads behind KILL-ALTALGO** (asked for in section (d)): N23a-37 (SAC), N23a-38 (TD3), N23a-39
  (evolution strategies) were written; asynchronous sharding is N36-12; adaptive randomization is N23b-04; the
  privileged-teacher path is inside BD-323 and checks its row.
- **Late known answers.** The five unnumbered known answers are provisional rows KA-23-L1 to L5 of N23-KA.
- **Refusal registry.** The raising-site column of N23-REF names the beads that raise each code (N23a-03, -05,
  -08, -13, -18, -24, -25, -27 beside the seeds).
- **Reported, not fixed here.** No bead assigns Appendix O codes to refusals that late subsections state in
  words (three in this chapter, one in §24.14). The planted linear-phase sampler is drafted twice (N23b-30 and
  N33c-37); the geometric-algebra pilot twice (N23b-01 and N10b-16); the inheritance pilot overlaps N10b-15.
