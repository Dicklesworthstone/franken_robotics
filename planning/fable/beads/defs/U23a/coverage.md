# Coverage of drafting unit U23a

Chapter §23 Cortex (the gym, learning, family policies, inheritance), subsections §23.1 to §23.13 and the chapter preamble, plus eleven seed beads. 48 bead files: 11 seed beads, 36 new beads `N23a-01` to `N23a-36`, and the registry `N23-KA`. The other unit of the chapter (U23b) owns §23.14 to §23.29 and writes `N23-CX` and `N23-REF`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §23 preamble, §23.1 | What it owns (crates, layer, what learning is for, the two throughput ceilings) | N23a-01 (crate, module map, contract, refusal rows, the role of learning); the throughput-units rule also in BD-192 and N23a-14 |
| §23.2 | Decisions ADR-CORTEX-01 to -09 | All nine restated in N23a-01's contract; each governed in: -01 and -02 BD-192; -03 BD-197, N23a-10; -04 BD-196, N23a-15; -05 BD-321, N23a-24; -06 BD-322, N23a-28; -07 N23a-31, N23a-32, BD-326; -08 BD-198; -09 N23a-18 |
| §23.3 | The environment API and its semantics | BD-192 (trait, views, E-1 to E-5, masked autoreset, interval integration of rewards, `dt_sim`, the two throughput units, integration of E-6 to E-9, GATE-CORTEX-06); N23a-02 (E-6 typed spaces, normalization, clipping); N23a-03 (E-8 access classes, E-9 causality, PO-CORTEX-04); N23a-04 (E-7 identity, order, seeds, snapshot and restore, recurrent state); N23a-07 (bootstrap targets, physical discounting, the hand table); N23a-34 (end-to-end) |
| §23.4 | Device-resident tensor views | BD-193 (TensorView, one stride convention, the converter, unrepresentable views, epochs and leases, PO-CORTEX-05, GATE-CORTEX-09); N23a-06 (rollout storage on the device, masks everywhere, INV-CORTEX-07, diagnostic ring slots) |
| §23.5 | Declarative task specifications with requirement terms | N23a-05 (the term table: what Cortex requires of each term kind; the three requirement-term rules; FROB-REF-554; curriculum and reset rows); BD-255 (streaming past-time monitors on batched tiers, the boundary guard, `task.verdict`, `task.settle`, `frob.verdict/1`, GATE-CORTEX-10). The `.ftask` compilation itself is seed BD-194 (unit U24b) |
| §23.6 | Algorithms and the PPO rules | BD-197 (the eleven rules as a table, the assembled trainer, the admission battery, three planted defects plus the single-`done` control, GATE-CORTEX-07); N23a-07 (P-1, P-7, trace part of P-9); N23a-08 (P-3, P-11); N23a-10 (P-2, P-4, P-5, P-9, P-10, value normalization, adaptive step size, the sibling's defects as facts); N23a-11 (P-6, P-8, PO-CORTEX-02); N23a-06 (P-9 masks); N23a-19 (P-10 across shards); N23a-09 (the default action law); N23a-14 (verbs `train.start`, `train.status`, `train.stop`; the algorithm registry for SAC, TD3, evolution strategies, teacher-student, behaviour cloning); N23a-13 (`train.export`); BD-198 (the reference training); N23a-35 (end-to-end) |
| §23.7 | Networks, checkpoints and the mapping table | N23a-12 (NetworkSpec, MLP, GRU, asymmetric actor-critic); N23a-13 (manifest `frob.policy/1`, mapping table, role matching, FROB-REF-531, INV-CORTEX-04); N23a-20 (residual head [F]); N23a-31 (equivariant MLP [F]). The morphology transformer and the geometric-algebra tree policy are U23b's seed beads BD-320 and BD-439 |
| §23.8 | Where learning runs, and what FrankenTorch is for | BD-196 (on-device kernels, backward passes, GATE-CORTEX-02, PO-CORTEX-06, the FD-FT-02 switch-over rule, precision); N23a-15 (the three jobs of FrankenTorch: CPU learner, gradient oracle, interchange; admission; no-sibling fallback); N23a-16 (KILL-LEARNKERN decision); N23a-17 (its fallback trainer); N23a-18 (which machine, precision, the labeled D4 profile, FROB-REF-867, learner share, cross-backend evaluation); BD-198 (the behaviour-not-plumbing gate rule) |
| §23.9 | Multi-device PPO: the normalization rule | N23a-19 (shard sums, fixed-order reduction, the exact pairwise merge, PO-CORTEX-03, the 0.25 and 32/9 fixtures, the parity clauses of GATE-CORTEX-08). The two-card scaling measurement is seed BD-199 (unit U36) |
| §23.10 | Distillation [F] | BD-323 (one mechanism, four teachers, dataset aggregation, the rules, `train.distill`, GATE-CORTEX-14); N23a-21 (KILL-DISTILL decision); N23a-36 (end-to-end) |
| §23.11 | Family policies per morphology class [F] | BD-321 (the experiment, fine-tuning rungs, the evidence table, break-even, GATE-CORTEX-11); N23a-22 (OPEN-CORTEX-1, the compute envelope, break-even arithmetic); N23a-23 (pretraining distribution, three held-out sets, membership rule, terrain set, thresholds, pre-registered); N23a-24 (the rule for using it as an evaluation rung, INV-CORTEX-06, FROB-REF-537, GATE-CORTEX-12 on our engine); N23a-25 (`policy.apply`, FROB-REF-534); N23a-26 (KILL-FAMILY decision). Architecture, tokens, pretraining and `train.family` are U23b's seed bead BD-320 |
| §23.12 | Inheritance along design edits [F] | BD-322 (`train.inherit`, disclosure, matched arms, the 12-pair experiment and the 10-edit chain, GATE-CORTEX-13); N23a-27 (InheritanceMap, transport by adjoint, dual and group element, FROB-REF-531); N23a-28 (scheduled resets, audit twins, separate accounting of inherited and fresh evaluations); N23a-29 (OPEN-CORTEX-3: k and m); N23a-30 (KILL-INHERIT decision); N23a-36 (end-to-end) |
| §23.13 | Equivariance, only where the closed loop has it | N23a-31 (exact reduction under Γ_obs, equivariance in law); N23a-32 (augmentation with a symmetry loss); BD-326 (the three-arm experiment for BET-07, the one-sided patch, GATE-CORTEX-03); N23a-33 (KILL-EQUIV decision) |

Seed bead outside the chapter's text: BD-042 (the plan-time pilot for the static stance family) is assigned to this unit; its content comes from §25.13, §25.22, §25.5.3, construction C-32 and Appendix H.5, and the bead carries it in full.

Identifiers carried. Invariants: INV-CORTEX-01 BD-192; -02 N23a-10, N23a-02; -03 N23a-08; -04 N23a-13; -05 BD-198 (the record's controls; the generic validator is U23b's); -06 N23a-24; -07 N23a-06. Proof obligations: PO-CORTEX-01 N23a-07; -02 N23a-11; -03 N23a-19; -04 N23a-03; -05 BD-193; -06 BD-196. Open questions: OPEN-CORTEX-1 N23a-22 (owner decision); -2 N23a-09 (decision); -3 N23a-29 (decision); -5 in the Considerations of N23a-07 and the defaults of N23a-05; -6 in N23a-16; -4 belongs to the adapter (U23b). Bets: BET-07 and BET-30 BD-326; BET-15 BD-321; BET-16 BD-322; BET-17 N23a-24. Risks: RISK-24 BD-197; RISK-27 and RISK-37 N23a-24; RISK-28 N23a-22, N23a-26, N23a-28; RISK-30 N23a-16. Constructions: C-09 (N23a-31, N23a-32, BD-326), C-23 (N23a-27, BD-321, BD-322), C-30 (N23a-05), C-32 (BD-042). Theorem card T-22 (BD-042). Upstream work items named with interim posture and switch-over rule: FD-FT-02 (BD-196, N23a-15, N23a-17), FD-FSIM-10 (BD-197, N23a-10), FD-FNP-01 (BD-193).

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone | Measured or decided by (this unit) | Notes |
|---|---|---|---|
| GATE-CORTEX-01 | M2 | BD-198 (first record); N23a-13 (exported-checkpoint clause); N23a-35 (confirming receipt) | |
| GATE-CORTEX-02 | M2 | BD-196 (first record); N23a-35 (confirming receipt) | |
| GATE-CORTEX-03 | M4 | BD-326 (the experiment and record); N23a-31 and N23a-32 (arms and their controls) | |
| GATE-CORTEX-06 | M2 | BD-192 (first record, through the battery `cortex-env`); N23a-02, N23a-03, N23a-04, N23a-07 (rule fixtures); N23a-34 (confirming receipt) | |
| GATE-CORTEX-07 | M2 | BD-197 (first record, battery `cortex-ppo`); N23a-06, N23a-07, N23a-08, N23a-10, N23a-11 (rule fixtures); N23a-35 (confirming receipt) | Rule P-12 extends the battery (U23b) |
| GATE-CORTEX-08 | M2 | N23a-19 (parity clauses: D3 agreement, empty shard, 0.25 and 32/9); N23a-35 (confirming receipt) | The scaling clause (at least 1.25 times) is measured by seed BD-199 (U36) |
| GATE-CORTEX-09 | M2 | BD-193 (first record); N23a-34 (confirming receipt) | |
| GATE-CORTEX-10 | M3 | BD-255 (record); N23a-05 (the refusal and the mixed-unit fixture at admission) | |
| GATE-CORTEX-11 | M4 | BD-321 (record); N23a-23 (registration); N23a-25 (decline clause); N23a-36 (user-side receipt) | |
| GATE-CORTEX-12 | M3 pilot, M4 on ours | N23a-24 (scores, record on the family policy, enforcement) | The statistics and ledger are seed BD-328 (U23b); the pilot on incumbent tools is seed BD-310 (U28a) |
| GATE-CORTEX-13 | M4 | BD-322 (record); N23a-27, N23a-28 (laws and chain clause); N23a-36 (user-side receipt) | |
| GATE-CORTEX-14 | M4 | BD-323 (record); N23a-36 (user-side receipt) | |
| KILL-LEARNKERN | M2 | N23a-16 (decision); N23a-17 (fallback, on the fired branch) | |
| KILL-FAMILY | M4 | N23a-26 (decision) | |
| KILL-INHERIT | M4 | N23a-30 (decision) | |
| KILL-DISTILL | M4 | N23a-21 (decision) | |
| KILL-EQUIV | M4 | N23a-33 (decision) | |

Gates of the chapter that this unit expects another unit to carry (their mechanisms are in §23.14 to §23.29 or in another chapter's seed bead): GATE-CORTEX-04 (BD-267, U23b), GATE-CORTEX-05 (BD-218, U23b), GATE-CORTEX-15, -16 (BD-301), -17 (BD-439), -18, -19, -20, -21, -22 (U23b); KILL-ALTALGO (BD-324), KILL-GATREE, KILL-GYMADAPT, KILL-SHORTGRAD (U23b). Kill gates from other chapters that these beads name without deciding: KILL-RESIDUAL (N23a-20; control subsystem), KILL-SHARD (N23a-19; performance chapter), KILL-SPEC-REWARD (N23a-05, BD-255; task subsystem).

## (c) Counts

- Known answers: the registry `N23-KA` holds all 18 rows of the chapter (KA-576 to KA-593). Sixteen are tested by this unit's beads (KA-576 to KA-590 and KA-593); KA-591 and KA-592 belong to mechanisms in U23b's subsections. In addition this unit's beads carry, with values inline, 8 known answers from other chapters for BD-042 (KA-157, -158, -159, -160, -624, -625, -626, -658) and 2 from the suite chapter for the PPO probes (KA-773, KA-774).
- Counterexamples: the registry `N23-CX` is U23b's. This unit's beads state and test 12 of the chapter's 13 counterexamples (CX-246 to CX-257; CX-258 belongs to the model-form check), plus CX-357 and CX-379 (learning-related rows of other chapters) and CX-016, CX-077, CX-078, CX-079, CX-273, CX-274 in BD-042.
- Refusal codes: the registry `N23-REF` is U23b's. Of the chapter's 12 codes, this unit's beads implement 10 with meaning, trigger, ranked repairs and exit code 3: 521 and 522 (BD-192), 524 (N23a-03), 527 (N23a-08), 531 (N23a-13, N23a-27), 534 (N23a-25), 537 (N23a-24), 547 (BD-193), 554 (N23a-05, BD-255), 867 (N23a-18). All 12 are registered as data rows by N23a-01. Codes 544 and 830 belong to mechanisms in U23b's subsections (BD-198 requires the controls that 544 checks).

## (d) Needs from other slices

Prerequisites owned by another unit for which no key was known (each appears in a bead's `needs` field, starting with its plan subsection):

- **§23.22 pilots (U23b).** The pre-funding pilots of the Frontier mechanisms drafted here, each with its protocol and falsifier restated in the needing bead: family policy (BD-321, N23a-22, N23a-26); inheritance (BD-322, N23a-27, N23a-28, N23a-29, N23a-30); distillation and behaviour cloning (BD-323); equivariant networks (BD-326, N23a-31). This unit wrote no pilot bead for these rows because §23.22 is U23b's subsection; if U23b also deferred them, the lead should commission them.
- **§23.23 (U23b).** The training-gate record validator behind FROB-REF-544 and INV-CORTEX-05 (BD-198, BD-323).
- **§23.25 (U23b).** Rule P-12, the sampler qualification receipt required by `train.start`, GATE-CORTEX-18 (BD-197, N23a-08, N23a-14, N23a-35).
- **§23.26 (U23b).** Rule E-10: access classes for rewards, flags, masks and `info`; the four planted leaks; GATE-CORTEX-19 (N23a-03, N23a-34).
- **§23.27 (U23b).** The attainment rule and the `crossing` and `attained` fields of `train.status` (BD-198, BD-326, N23a-14, N23a-35).
- **§23.28 (U23b).** The winner check for a selecting rung (N23a-24).
- **§23.25 to §23.28 (U23b).** Known answers of the late subsections, which have no row in Appendix M (N23-KA).
- **§23.15 (U23b).** Randomization from cards (N23a-05). **§23.11 `train.family`** is part of seed BD-320 (N23a-36).
- **§24.4, §24.13.** A compiled `TaskProgram` for the fixtures (BD-192; seed BD-194 supplies it); blind edit pairs sealed by the qualification lane (BD-322).
- **§28.9, §28.10, §28.** The ladder's rungs and audit lane, the rank-agreement ledger and verb, the diversity-seeking search (BD-321, N23a-24, N23a-28).
- **§14.7, §17, §17.16.** Typed edits' correspondence declarations (N23a-27); the generator, split and support check (N23a-23, N23a-25).
- **§22.7, §22.8, §22.17.** The wrapped baseline-plus-residual controller and its pilot (N23a-20); the sampling-controller fallback (N23a-25).
- **§33.6.** An admission test for FrankenTorch's host crates as CPU learner and gradient oracle: the registered tests INT-FT-01 and INT-FT-02 cover only the Metal gateway (N23a-15 writes the test itself and flags the missing registry row).
- **§35.2, §36.6, §36.14, §39, §32.** The fast-training profile's definition (N23a-18); the two-card measurement (N23a-19); the compute classes (N23a-22); the phase ledger (N23a-18); registration of M2 verbs (N23a-14).

Dependencies the lead should consider adding to beads written by other units (this unit could not edit them): BD-199 → N23a-19 (the normalization rule precedes the two-card measurement); BD-320 → N23a-22 and N23a-23 (the envelope decision and the sealed held-out sets precede pretraining); BD-328 → N23a-24 (the three-way scores feed `eval.rank-agreement`); BD-330 → N23a-20 (the residual head); BD-301 → N23a-34 (the adapter's parity runs reuse the environment battery); BD-269 ↔ N23a-18 (the D4 profile's two halves).

## (e) Contradictions and items that could not be turned into work as written

1. **The reproduction archive's fingerprint.** Appendix H.5 prints 241,568 bytes and SHA-256 `0326e89d…b09a4f`. The only `.zip` in `planning/` at drafting time is `astra-10x-reproduction.zip`, 275,178 bytes, SHA-256 `9620da42…cd7980`. BD-042's first step is to fingerprint the file, record which pair it matches, and stop for the owner if neither does.
2. **Two refusal codes for one condition.** §23.4 and §23.19 give FROB-REF-547 for a tensor view the receiver cannot represent under a zero-copy contract; §21's kernel-manifest text gives FROB-REF-617 for the same rule. BD-193 raises 547 in `frob-learn` and notes 617 as the device-layer code.
3. **KA-581's unit.** Appendix M prints the thermal margin and its scale in degrees Celsius; §23.5 says a temperature margin is a difference and its scale is written in kelvin. `N23-KA`, N23a-05 and BD-255 use kelvin and record the discrepancy.
4. **KILL-LEARNKERN's trigger and fallback.** "Missed by more than two times" admits two readings (a ratio above 2.5, twice the gate's 1.25 limit; or above 2.0). N23a-16 adopts 2.5 as DESIGN, states the stricter reading, and asks the owner to confirm in the decision record. The §40 row omits the fallback's last clause (the trainer moves to FrankenTorch device execution if FD-FT-02 lands), which §23.23 has; the bead keeps it.
5. **Seed done-conditions that would create cycles.** BD-196 "KILL-LEARNKERN decided": the trigger needs the training-time gate, which is downstream of BD-196. BD-321, BD-322, BD-323, BD-326 "KILL-… decided": the decision needs the bead's own measurement. Each seed bead records the kill-gate inputs, and a separate decision bead (N23a-16, -21, -26, -30, -33) depends on it and records the decision.
6. **GATE-CORTEX-06 inside BD-192.** The gate includes the reward-rate fixture, which needs task terms, while seed BD-194 (task terms) depends on BD-192. BD-192 uses hand-built synthetic task programs for that fixture; N23a-34 re-measures through real `.ftask` files.
7. **OPEN-CORTEX-2 "blocks GATE-CORTEX-07".** The evidence for choosing the default action law would naturally come from the reference training, which is after the battery. N23a-09 runs a small paired study through the library interface before the verbs exist and ratifies the fail-closed default if the owner does not decide; BD-197 depends on it.
8. **The P-4 fixture.** The plan gives the finite differences "in log σ" (0, −0.5, −1 against −0.25, −0.5, 0) without the sample. N23a-10 reconstructs one that yields those numbers (squared standardized residual 3/2, advantage −1) and labels it DESIGN; the admission probe of seed BD-034 is authoritative.
9. **GATE-CORTEX-03 against BET-07.** The gate can pass (the equivariant arm beats the unaugmented arm) while the bet's second falsifier holds (no better than augmentation at equal compute). N23a-33 makes the fork explicit with a fail-closed default.
10. **KILL-DISTILL (70%) against GATE-CORTEX-14 (90%), KILL-INHERIT (60%) against GATE-CORTEX-13 (20%), KILL-FAMILY's conjunction.** The plan does not say what holds between the kill and the pass thresholds. The decision beads define the intermediate state as DESIGN (advisory, experimental, or a restricted role) and say so.
11. **BD-322's seed dependency on BD-320.** Specialist inheritance does not need a family policy and must work in the M4 variant with none; the dependency is kept as given and the bead notes that only the prior-based arm and reset target need it.
12. **Running conformance batteries from `frob`.** The plan names `frob-conformance` and `eval.run` but no command for unit-level batteries. The beads register batteries `cortex-env`, `cortex-ppo` and `cortex-verdict` and run them with `frob eval run --battery <name>` (DESIGN names).
13. **FrankenTorch's GRU gate form** is not stated in the plan; N23a-12 marks interchangeability as a HYPOTHESIS settled by a conformance test against the pinned source.
14. **GATE-CORTEX-08 has two owners** (§23 defines it; the seed bead that measures it is in the performance chapter). N23a-19 measures the parity clauses; the scaling clause stays with BD-199.
