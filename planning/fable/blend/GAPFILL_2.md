# GAPFILL_2: completeness audit of the Phase 1 parts

Audit date 2026-11-21. Scope: the thirteen part files `parts/P00.md` to `P12.md` (chapters §1 to §37). Chapters §38 to
§47 and the appendices are not yet written and were not counted as coverage. This file is a working document for
the lead: sections (a), (b) and (d) name sources and exercises; the insertion blocks of section (c) are final plan
text and name none.

**Totals.** 233 checklist rows: 202 COVERED, 23 WEAKER, 8 MISSING. Rows overlap where
two sources list one idea; the non-covered rows reduce to 21 distinct gaps, each filled by one block. The
adjudication of the verdict-register challenges (section (b)) requires nine further blocks. Section (c) holds
30 blocks: §14 (1), §16 (5), §17 (1), §18 (1), §19 (1), §20 (1), §22 (3), §23 (2), §24 (3), §25 (4), §27 (1), §28 (2), §31 (3), §32 (1), §39 (1).

**Method.** Each checklist item was read in full at its source, then looked up in the inventories
(`GATES_INVENTORY.md`, `VERBS_INVENTORY.md`, `LEDGERS.md`, `OPENS_INVENTORY.md`) and by grep over the parts with
several synonyms, and the cited subsections were read to confirm that the mechanism and the specifics the source
establishes (corrected numbers, conceded objections, executed counterexamples, fixtures) are present. COVERED means
both are present. WEAKER means the mechanism is present and a named element is missing. For a killed idea, COVERED
means the strong form is absent or rejected by name and the surviving narrow form is present.

**Identifiers.** New gates are `GATE-<SUBSYS>-801` to `-834`, numbered once across this file in block order. New
kill gates were checked against `GATES_INVENTORY.md`; two blocks add a trigger to an existing kill gate
(KILL-RESPONSE) or reuse an existing one (KILL-PACKET, KILL-TREEOP) and say so. Refusals are described without
numbers. New verbs introduced by the blocks: `control.lift`, `eval.items`, `module.characterize`,
`sense.visibility`, `task.audit`. New flags on existing verbs: `autopsy.attribute --lane`; `control.mpc --freeze`,
`--compare` and `--planner`; `eval.compare --bracket`, `--budget-curve` and `--sampling`; `mission.synthesize
--strategy placement`; `mission.refute --split-cap`; `phylum.gradient --method adjoint`; `report.campaign --arms`
and `--ablate`; `sim.run --solve-path`; `task.check --coupling`; `train.start --algo short-horizon-ac`;
`design.diff --reuse`. The list was extracted mechanically and should be checked when Appendix J is assembled.

## (a) Coverage table

### a.1 IDEA_CATALOG §1.1–§1.8 (rows K-001 to K-068): 26 covered, 5 weaker, 0 missing

| Item | Source | Status | Where covered, or what is missing |
|---|---|---|---|
| K-001 Module substitution contracts (`.fmodule`) | IDEA_CATALOG §1.1 | COVERED | P02.md §14.6 (port contract, guarantees and assumptions, four leaf-local conditions, reuse table, base-coupled contact invalidates, the two-fingers-on-a-bus counterexample and the time-resolved `bus_demand` profile); §14.9 (influence records; ceiling "small once a rollout takes seconds"); P03.md §17.11 invalidation contract and coupled-bus control; GATE-SKEL-06, KILL-SKEL-SUBST; P01.md §10 notes that constrained Dirac structures are deferred upstream and no passivity proof generator exists. Run-time composition is absent, as it should be. |
| K-002 Design canonicalization and symmetry dedup | IDEA_CATALOG §1.1 | COVERED | P03.md §17.11 (orbit representatives under Γ_obs only; exact 86,185 and 500,500; "honest range two to twelve"; the g·d in g⁻¹·w rule for joint robot–world orbits); P01.md C-22; P02.md §13.6; ADR-PHYLUM-04; verb `phylum.quotient`; GATE-PHYLUM-01; KILL-QUOTIENT. |
| K-003 Mission-observable quotient | IDEA_CATALOG §1.1 | COVERED | P03.md §17.11 "mission-observable digest" [F]: ballast fixture 735,471 → 24,546 (30.0), equal-Σx/different-Σx² control, "the factor is a property of this fixture", singleton warning; structural-not-proved rule; §17.18 known answers. No `.fquot` format (folded into the memo graph), which is consistent with the format list. |
| K-004 Temporal-logic requirement as reward and monitor | IDEA_CATALOG §1.1 | COVERED | P01.md C-30 (unit-normalized margins ρ = ψ/s; past-time operators online with at most three comparisons per sample per node; future-time gives verdicts on completed traces, `inconclusive` otherwise); P08.md §24 ADR-ARENA-04, §24.5; P06.md §23.5; GATE-CORTEX-10 (mixed-unit fixture, future-time online reward refused); FROB-REF-523, -554. The retracted "≤ 0.1%" claim does not appear. |
| K-008 Family (morphology-conditioned) policy | IDEA_CATALOG §1.2 | COVERED | P06.md §23.11 (token features as in the row, per-class weights, canonical inputs, pretraining distribution and held-out sets, compute-envelope question OPEN-CORTEX-1/OPEN-SPARK-2, supported numbers table, break-even N > P/(s·(1 − f)), rank-agreement rule); GATE-CORTEX-11, -12; KILL-FAMILY with the row's 70% / 20% trigger; exact layer beneath it in §22.10; Π-normalization caveat in §22.10 layer 2. |
| K-009 Capability-map skill transport (`.fskill`) | IDEA_CATALOG §1.2 | COVERED | P06.md §22.10: w = w₀ + B_m(q)·u from a constrained mass solve, allocation cone program, feasible-allocation witness or dual infeasibility witness before training, two-link hand fixture, limits (instantaneous only; scope; floating base needs the contact-consistent solve), refinement counterexample; learned residual [M] frozen and separately killed (KILL-SKILL); GATE-REFLEX-08, -09. |
| K-010 Controller transfer across typed edits | IDEA_CATALOG §1.2 | COVERED | P06.md §23.12 (declared correspondence of frames, actuator roles, sensors, contact roles, phase clocks; adjoint and dual transport; unmatched recurrent state reset; donor in manifest; scheduled resets; matched arm); §23.7 mapping table matched by role with signs, units, rates (INV-CORTEX-04, FROB-REF-531); GATE-CORTEX-13 (12 pairs, 4 blind); KILL-INHERIT; topology edits reported, not required. |
| K-011 Equivariant gait atlas | IDEA_CATALOG §1.2 | COVERED | P06.md §22.9 (H/K twisted shooting on Fix(K); stability over the full reduced state with the ẋ = −x, ẏ = +y counterexample; monodromy to `fs-spectral::floquet::analyze_monodromy`; pseudo-arclength continuation with fold, period-doubling, torus, symmetry-breaking outcomes; published solve cost stated; "screening" label); P01.md C-10; GATE-GAIT-01 to -03; KILL-GAIT. |
| K-017 Morphology as table data with gated specialization | IDEA_CATALOG §1.3 | COVERED | P03.md §17.4 (joint classes as screw-basis data, runs of 16 or 32 per model, no interpreter and no compile per design), §17.5 (launch-cost hypothesis stated as such; gain vanishes near 1,000 worlds per design), §17.8 (edit-to-first-step as the surviving lever, GATE-PHYLUM-04), §17.9 (promotion rule T_remaining·(1 − 1/g) > compile cost; ceiling 1.25; KILL-SPECIALIZE), §17.14 (head-to-head against nine per-family templates and incumbents, commitment to adopt the templates); P05.md §35.9 (draws keyed by world address, never by epoch or slot). The withdrawn 8–20× launch-cost claim is not made. |
| K-018 Limb port interface as an intermediate tier | IDEA_CATALOG §1.3 | WEAKER | Present: P02.md §14.6 "limb port interface" [F] (27 scalars, 108 bytes; semidefinite counterexample diag(0, 1, 1, 1, 1, 1); not valid under contact; not a way to decouple contact). The megakernel throughput claims are absent, correctly. Missing: the role the row keeps for it, an execution level between the table-driven kernel and per-design specialization (precompiled limb-archetype kernels, zero compile per design) and its place as an arm of the specialization pilot and the head-to-head. Block B07. |
| K-019 Work queues by row class and solver state | IDEA_CATALOG §1.3 | WEAKER | Present: per-world contact compaction (P03.md §16.6 stage 5), masks, the `done` mask copied forward (P05.md §35.5 rule 5), family lanes segmented at ambiguous guards (§17.6, KILL-FAMILYLANE). Missing: regrouping of worlds across unrelated robots by active row count and by live or converged solver state, with its price in launches; the 0.70 weighted-throughput floor; a head-to-head arm. No text prices the lane-time lost to idle lanes in a lockstep solve. Block B02. |
| K-025 Factor-once response batches | IDEA_CATALOG §1.4 | WEAKER | Present: P03.md §18.5 "Response batches on a fixed active set" [F] (per-column bound ‖b − H·x̂‖/λ_lo, mode guard, "a diagnostic query, not a stepping path", ratio about 14 at 96 rows and 64 columns, pilot, KILL-RESPONSE), verb `contact.response` (§18.13), P01.md C-06. Missing: the anchor digest, the per-column acceptance predicates, how λ_lo is certified, any known answer, and any convergence gate with a threshold (§18.17 has none for this mechanism). Block B08. |
| K-028 Settled-gait return-map adjoint | IDEA_CATALOG §1.4 | COVERED | P06.md §22.9.3 (saltation matrix with recorded guard margin and denominator; derivative labels; refusal near grazing), §22.9.4 (simultaneous impacts handled, bar fixture), §22.9.6 (d x̄*/dθ = (I − M)⁻¹·∂P̄/∂θ and the adjoint solve; refused at a fold), §22.9.7 record; GATE-GAIT-02 is the row's protocol of record (100 cases, 72 of 80 at cosine ≥ 0.95, 20 controls labeled or refused); PO-GAIT-02. |
| K-037 Division of labour between the machines | IDEA_CATALOG §1.5 | COVERED | P05.md §21.17 (role table, seeds-only crossing with pinned generator and `SceneId` equality, TARGET 1 GB per campaign-hour, FROB-REF-618, fleet ceiling 1/(1 − g), KILL-FLEET), §21.9 (31 buffer arguments, packed slabs); P04.md §19.15 world bank and GATE-HAB-12; INV-SPARK-10; GATE-SPARK-13. No generalization benefit is claimed. The max-plus campaign model is dropped on purpose ("nothing more elaborate for two nodes"). |
| K-041 Film field with tiered multiphysics | IDEA_CATALOG §1.6 | COVERED | P03.md §18.8 (film law, Hersey number Π = η·v/(p·a), dimensionless); P04.md §19.10 three tiers, §19.10.2 squeeze-film patches, §19.10.3 advected film [F] with conservative face fluxes and a reservoir per contact body, Huppert exponents 1/8 and 1/5, INV-HAB-07, GATE-HAB-07 (exponent within 2%, non-conservative control fails); optical volumes with extinction in §19.3 and §20.6; declared persistence across episodes (INV-HAB-03). |
| K-045 Clearance complexes (`.fclear`) | IDEA_CATALOG §1.6 | COVERED | P04.md §19.11: inner and outer complexes, witness, impossibility cut, `unresolved`; one union-find sweep over every margin with the named bottleneck (zero-dimensional persistence, `fs-topo` as parity oracle); 1,228,800 cells and 4.69 MiB; the 74.6 mm error term and local refinement; slabs and transition predicates for thresholds, stairs, ramps; `kinematic` against `geometric-only` labels; unknown regions; certificates; corridor closed form; "not claimed as unique"; KILL-FCLEAR; GATE-HAB-10; verb `scene.fit`. |
| K-058 Energy budget at a physical boundary (`.fenergy`) | IDEA_CATALOG §1.8 | COVERED | P06.md §22.11 (budget terms, exact-integer monitor, four premises, what it does not support, worked card 1.49 J against 2.00 J, 50.5 ms and 45.5 ms, reflected-inertia counterexample, format sketch); P01.md C-31 (exclusion reading: 0.94176 J against 0.90 J, removed by 0.05 J of approach energy); P07.md §25.4.5 rule 1 and P09.md §28 analysis A-20; GATE-SHIELD-01; KILL-ENERGY. One labeling inconsistency is listed under ERRORS. |
| K-059 Reachable-set tables for small subsystems | IDEA_CATALOG §1.8 | COVERED | P01.md C-31 tier 2 (at most four reduced states; R_mn((K_m ⊕ E) ∩ G_mn, D) inclusion; latency, sweep, backup domain; covered or uncovered); P06.md §22.12.5 [M] (8 modes × 16,384 cells, backward viability pruning, the four-state reduction is a monitored assumption); proof kind `tube-inclusion` (§12); GATE-SHIELD-05 (5,000 episodes); KILL-TUBE. No `.ftube` format, consistent with the format list. |
| K-062 Family-level parametric certificates | IDEA_CATALOG §1.8 | COVERED | P01.md C-17, C-31, C-32 and D-17 (generic comparator; "a satisfiability solver derived the family answer"); P07.md §25.3.2 and §25.4.2 (quantifier prefixes; `clairvoyant` may exclude, never include; FROB-REF-881), §25.4.5 (four proof rules: sound exclusions with approach energy, bounded control, necessary conditions), §25.6 (region evidence within its real scope), §25.22 (pusher: 20 of 20 intervals in each arm, measured ties 1.345 and 1.088, ceiling 1.46), GATE-AUGUR-04, KILL-FAMILYCERT, KILL-AUGUR-TIE. No novelty is claimed for the query. |
| K-065 Model-form rank check and energy-creation audit | IDEA_CATALOG §1.8 | COVERED | P03.md §16.9 (five variant families, `ModelRobust` / `ModelConditional`, cannot authorize a build packet or prototype budget, cost at most 10%, the row's demotion rule), §16.10 (signed work–energy residual tagged by contact mode, `energy-suspect`, penetration-recovery and pyramid-cone planted exploits); GATE-FID-05, -06; P08.md §26.6 (artifact guard: numerical variants before a failure is counted); P01.md §9 cites the feed-forward evidence (1.48 → 0.92). |
| K-013 Lifted-operator (Koopman) control — killed | IDEA_CATALOG §1.2 | WEAKER | Strong form absent, correctly. `fs-surrogate` (POD, DMD, polynomial EDMD inside validity bands) is listed as a dependency in P12.md §33. Missing: the surviving narrow form, one bounded off-critical-path experiment on a single soft-soil regime judged against the sampling controller at equal accuracy, and the size calculation that scopes it (1,891 functions, 3.58 × 10⁶ coefficients for 60 states). Block B11. |
| K-022 Earliest-interaction calendar — killed | IDEA_CATALOG §1.3 | WEAKER | Strong form explicitly rejected with its reason in P05.md §21.20 ("Per-world adaptive time tiles between interactions"). Missing: the surviving narrow form, an optional late lane for sparse multi-robot worlds (the scenes of P07.md §25.17). Block B03. |
| K-023 Time-parallel rollouts — killed | IDEA_CATALOG §1.3 | COVERED | Absent from every part; no narrow form survives. |
| K-024 Half-precision tensor-core contact solve — killed | IDEA_CATALOG §1.4 | COVERED | Rejected with the row's calculation (κ = 1 + 48/0.01 = 4,801, κ·u₁₆ = 2.34) plus cost and hardware arguments in P03.md §18.12 and P05.md §21.20; FROB-REF-245; replaced by the response batch of §18.5. A conflict about policy networks is listed under ERRORS. |
| K-029 Differentiation through all contact changes — killed | IDEA_CATALOG §1.4 | COVERED | NONGOAL-15 (P00.md §6); narrow forms present: labeled regime-local derivatives and refusal near grazing (P06.md §22.9.3), return-map gradients (§22.9.6), paired finite-difference gradients with binary metrics refused (P03.md §17.13). |
| K-038 Zero-copy live CPU reads of GPU state — killed | IDEA_CATALOG §1.5 | COVERED | Rejected in P05.md §21.20 (quiescence required; data are tiny). Narrow forms present: fenced snapshots (§21.9 rule R1), the residency map printed by `spark.place` (§21.10), the background referee and `batch-cpu` shadowing (P03.md §16.11). |
| K-042 In-loop lattice fluid, cloth and steam islands — killed | IDEA_CATALOG §1.6 | COVERED | P04.md §19.10.1 gives the removing calculations (152 bytes per cell, Mach number at control rate, relaxation time, the c ≤ 181 throughput bound). Narrow forms present: slosh as sprung masses with a modal known answer (GATE-HAB-08), rigid and articulated cord and rug proxies, coupled islands on the CPU referee for finalists only [M]. |
| K-043 Full deformable or fluid physics in every world — killed | IDEA_CATALOG §1.6 | COVERED | Same sections; reduced laws (film patches, soil laws of P03.md §18.10) plus Frontier and Moonshot tiers behind flags; KILL-LIVING. |
| K-051 Simulated human co-inhabitants — killed | IDEA_CATALOG §1.6 | COVERED | NONGOAL-16 (P00.md §6); kinematic capsule actors on route graphs as scenario dressing (P04.md §19.7). |
| K-066 Conley/Morse mode stratification — killed | IDEA_CATALOG §1.8 | COVERED | Absent. The surviving idea (symmetry-orbit mode strata, 27 strata and 162 transitions) is in P01.md C-16 and feeds the assumption-monitored bound of C-31. |
| K-068 Tropical and semi-algebraic reachability — killed | IDEA_CATALOG §1.8 | COVERED | Cylindrical algebraic decomposition rejected as doubly exponential (P07.md §25.6). Max-plus timing survives in P01.md C-10 (gait schedules, `fs-tropical::max_cycle_mean`). |

### a.2 IDEA_CATALOG §1.9–§1.15, §2 convergent ideas, §3 killed ideas, §4 blind spots: 81 covered, 12 weaker, 4 missing

"(see a.1)" marks an item whose master-table row is audited in a.1.

| Item | Source | Status | Where covered, or what is missing |
|---|---|---|---|
| K-070 Design-space carving (interval and monotone pre-filter; ellipsoid sampling and log-posynomial route killed) | IDEA_CATALOG §1.9 | WEAKER | P07.md §25.6 (per-constraint pavings on sub-boxes of at most six parameters, monotone bounds first, dual witnesses, "Killed forms, not built" naming both killed routes), §25.7. Missing: the surviving "narrow geometric-programming monomial screen" (exact exclusion of a conjunction of monomial constraints in exponent coordinates). Block B19. |
| K-072 Exact rational linkage synthesis within solver caps (unbounded Gröbner killed) | IDEA_CATALOG §1.9 | COVERED | P07.md §25.21.3 (three-pose dyads, bounded zero-dimensional templates, circuit/branch/clearance certificate, caps as FACT, "no speed claim" ceiling, KILL-SYNTH-LINKAGE); P01.md C-34 and §12 caps table. Unbounded synthesis stated as not built. |
| K-073 Homotopy continuation with α-certified roots | IDEA_CATALOG §1.9 | COVERED | P07.md §25.21.4 (parameter homotopy, monodromy with trace test, α-test with distinctness and reality checks, `Complete` only when the certified count equals the trace-test count, else `Partial`; 4,326 against 286,720; GATE-AUGUR-22, KILL-SYNTH-CONT); P01.md C-34, T-34. Tracking is placed on the CPU in `f64` by decision, with the reason stated. |
| K-082 Campaign-level quotienting, with the blinded audit of discards and the planted winner | IDEA_CATALOG §1.10 | COVERED | P09.md §28.12 (three audits; planted winner "in a form the screen represents poorly"), §28.15; P03.md §17.11; P08.md §26.10; P01.md C-22; P00.md BET-31. Honest counts 86,185 and 500,500 printed. |
| K-083 Rare-failure search over certified-solvable worlds | IDEA_CATALOG §1.10 | COVERED | P08.md §26.2 to §26.9 (legality set, solvability witness from the candidate's own body, margin score, recorded proposal law, exploration apart from certification; ceiling "near 6" after the recomputation); P00.md §2.4 lever 4; P01.md C-27. |
| K-086 Value-of-information ladder (killed as a decision procedure; survives as a scheduling heuristic) | IDEA_CATALOG §1.10 | COVERED | P09.md §28.16 (advisory, never decides validity, ADR-ATEL-07, KILL-VOI); P08.md §24.12 (two study types never mixed), §26.4 ("the ranking orders work and never decides a verdict"). |
| K-090 Counterfactual intervention branching | IDEA_CATALOG §1.11 | COVERED | P08.md §27.3 to §27.13 (forks, classification, placebo, contrasts, budgeted Shapley with the negative-value case, re-run from episode start for design parameters, "not a minimal repair"); P09.md §29.8; P01.md C-29. |
| K-091 Minimal counterexample world by delta debugging | IDEA_CATALOG §1.11 | COVERED | P08.md §26.7 ("1-minimal under removal of atomic edits, and nothing stronger", order-dependence counterexample, `.fmcw`); P01.md C-28; dual in P04.md §19.12. |
| K-095 Offline single-file replay dossier | IDEA_CATALOG §1.11 | WEAKER | P04.md §20.10 and P11.md §32.15 (self-contained HTML export, ghost overlay), verb `telemetry.export-html`. Missing: packet schema and contents, a size cap with the Base64 arithmetic, the rule that it is an adapter built after the native fork harness, the secure-context caveat, a gate. Block B29. |
| K-096 Counterexample-guided build packet | IDEA_CATALOG §1.12 | WEAKER | P10.md §31.12 (packet layout, four false flags with no setter, snap revalidation, family-prune rule with the two-finger counterexample, cross-export identity 10⁻⁶), GATE-FOUNDRY-09, -10, KILL-PACKET. Missing: the preferred gate's hidden-brief protocol and the reviewer record (12 hidden briefs, at least 11 accepted); ADR-FOUNDRY-11 makes review an import lane but gives it no format. Block B26. |
| K-097 Closed-loop link topology optimisation with tolerance allocation | IDEA_CATALOG §1.12 | COVERED | P10.md §31.5 to §31.11 (load extraction, symmetric limbs unioned not assumed equal, sizing and topology optimization, fatigue, inertia fed back with a convergence test, manufacturability, t ∝ (c/s²)^(1/3)), GATE-FOUNDRY-05 (120 s per link class; 5% kill, 15% target), KILL-TOPOPT with beam-rung fallback. |
| K-098 `no_std` fixed-point controller export (board-support generation killed) | IDEA_CATALOG §1.12 | COVERED | P10.md §31.13 (joint-level scope, caller-owned bus trait, range analysis, bit-vector overflow certificates, 10⁶ vectors "not called equivalence", `timing: no-claim` until a linked-binary receipt), GATE-FOUNDRY-11, KILL-FIRMWARE. |
| K-100 Lifecycle compression by two-timescale averaging | IDEA_CATALOG §1.12 | COVERED | P10.md §31.14 (rainflow residues carried across windows, every output Estimated, "a count of steps, not of wall time", validity domains, return to full resolution near a unit Floquet multiplier), GATE-FOUNDRY-12, KILL-LIFECYCLE. |
| K-106 Multi-agent guilds and design markets (killed as a product lever) | IDEA_CATALOG §1.14 | COVERED | Absent as a product feature, as intended. Swarm use stays external: P11.md §32 (deficit round-robin across agents, shared HTTP server, mail thread identifiers). |
| K-109 Design-decision benchmark and the ten-fold metric | IDEA_CATALOG §1.15 | COVERED | P00.md §2.1 to §2.7 (restricted mean time to an independently accepted answer, strongest composed rival, cold and warm, pooled ratio over at least ten paired campaigns, compute per established decision, ceilings), BET-10, BET-13; P08.md §24.9 and GATE-AUTOPSY-08 (planted-defect corpus, repair turns). The protocol and decision rule belong to §39, not yet written. |
| K-110 Rival audit gate | IDEA_CATALOG §1.15 | COVERED | P05.md §36.16 GATE-SPARK-21 (parity baseline measured before anything is built against it), §36.8 item 7; P00.md §2.7 item 1. Freezing of comparator revisions is deferred to §39, not yet written. |
| Rank 1: K-090 | §2 | COVERED | As the K-090 row. |
| Rank 2: K-017 to K-021 many different robots in one launch | §2 | COVERED | P03.md §17.3 to §17.9 (model tables, size buckets, family lanes, admission latency, survivors-only specialization) and §17.14 (four-arm head-to-head with the commitment to its outcome, GATE-PHYLUM-03, KILL-PHYLUM). |
| Rank 3: K-083 | §2 | COVERED | As the K-083 row. |
| Rank 4: K-109 | §2 | COVERED | As the K-109 row. |
| Rank 5: K-028 settled-gait return-map sensitivities | §2 | COVERED | P01.md C-16; P06.md §22.9.3 to §22.9.6 (labels, grazing refused, simultaneous impacts handled by enumeration), GATE-GAIT-02. (see a.1) |
| Rank 6: K-037 Mac as scene bank, RTX 4090 as trainers | §2 | COVERED | P00.md §1.8, BET-35; P04.md §19.15 (world bank, KILL-BANK-BREADTH for the untested generalisation benefit); P05.md §21.17. (see a.1) |
| Rank 7: K-002 canonicalise and dedup by symmetry | §2 | COVERED | P01.md C-21, C-22; P02.md §13; P03.md §17.11 (86,185 and 500,500). (see a.1) |
| Rank 8: K-001 module substitution contracts | §2 | COVERED | P02.md §14.6 (`.fmodule`, invalidation table, the shared-bus time-correlation counterexample); P01.md C-12 (base-coupled contact invalidates every global result). (see a.1) |
| Rank 9: K-008 family policy | §2 | COVERED | P06.md §23.11 (per class, compute envelope, rank-agreement rule, fail-closed default), KILL-FAMILY. (see a.1) |
| Rank 10: K-045 clearance and route-feasibility certificates | §2 | COVERED | P04.md §19.11 (witness, cut or `unresolved`), GATE-HAB-10, KILL-FCLEAR. (see a.1) |
| Rank 11: K-097 | §2 | COVERED | As the K-097 row. |
| Rank 12: K-105 evaluation and decision cache | §2 | COVERED | P09.md §28.13, §28.14 (memo graph, shadow recompute, stop on one mismatch; sensitivity-margin reuse withdrawn); P07.md KILL-AUGUR-REUSE; e-value averaging rule in P09.md §28.6. |
| Rank 13: K-086 | §2 | COVERED | As the K-086 row. |
| Rank 14: K-013 Koopman lifting (cut; one bounded soil-regime experiment survives) | §2 | MISSING | No lifted-operator experiment appears in any part; the strong form is correctly absent. The surviving narrow form (one off-critical-path soil-regime experiment against an equal-accuracy sampling controller) is absent. (see a.1) Block B11. |
| Rank 14: K-101 actuator identification | §2 | COVERED | P10.md §30.8 to §30.10 (identifiability, actuators first, card updates from logs). |
| Rank 14: K-103 scan or video to simulation twin | §2 | COVERED | P04.md §19.13 (ingestion with three-valued occupancy, probe requests instead of friction from appearance, KILL-INGEST); P10.md §30.14. |
| Rank 14: K-106 | §2 | COVERED | As the K-106 row. |
| Rank 15: K-010 controller transfer across typed edits | §2 | COVERED | P06.md §23.12 (inheritance, `train.inherit`, KILL-INHERIT); P01.md C-23. (see a.1) |
| Rank 15: K-027 implicit-function contact and morphology adjoints | §2 | WEAKER | Narrow forms present: return-map adjoint (P06.md §22.9.6), adjoint columns on a fixed active set (P03.md §18.5 response batches), paired finite differences (P03.md §17.13). P00.md NONGOAL-15 and the M6 milestone refer to gradients through contact, but no chapter specifies the mechanism, its guard or its kill gate. Block B04. |
| Rank 15: K-035 fused persistent step kernel | §2 | COVERED | P05.md §21.18 (fused step kernel [M], one-week pilot, KILL-MEGA). |
| Rank 15: K-065 model-form and energy audit | §2 | COVERED | P03.md §16.9, §16.10, GATE-FID-05; P06.md §23.16, GATE-CORTEX-15. (see a.1) |
| Rank 15: K-096 | §2 | WEAKER | As the K-096 row. Block B26. |
| Rank 15: K-098 | §2 | COVERED | As the K-098 row. |
| Rank 15: K-100 | §2 | COVERED | As the K-100 row. |
| Rank 15: K-095 | §2 | WEAKER | As the K-095 row. Block B29. |
| Rank 15: K-059 reachable tubes | §2 | COVERED | P06.md §22.12.5 (reachable-set tables for at most four reduced states [M]), GATE-SHIELD-05, KILL-TUBE; P01.md §12 kind `tube-inclusion`. (see a.1) |
| Rank 15: K-062 family certificates | §2 | COVERED | P01.md §12.2, KILL-FAMILYCERT; P07.md §25.5. (see a.1) |
| Rank 15: K-070 | §2 | WEAKER | As the K-070 row. Block B19. |
| Convergence on a problem: cheap referee promotion (K-084, K-085, K-111) | §2 | COVERED | P03.md §16.8 (doubly robust tier correction with logged audit probabilities, `--method paired|dr|multilevel|segment`), §16.11 (sampled background referee, KILL-SHADOW); P01.md C-25; P09.md §29.11 (switch-off below ρ = 0.7 on 200 addresses). |
| Convergence on a problem: contact safety by matching the certificate to the regime (K-058 to K-061) | §2 | COVERED | P06.md §22.11 (energy budget), §22.12 (quantifier order, split cones, exact one-row projection, bound with independent labels, no claim under shift); P00.md §8.2 claim product. |
| Highest cross-scores list (K-017, K-011, K-090/K-083/K-091, K-082, K-001/K-002, K-058, K-028, K-041) | §2 | COVERED | Rows above, plus `gait.atlas` in P06.md §22.9 and the film-friction law in P03.md §18.8 and P04.md §19.10.2, §19.10.3. (see a.1 for the K-0xx rows) |
| K-024 half-precision Delassus (survivors: factor-once multi-RHS batching; scalar path at four or fewer contacts) | §3 | COVERED | Rejection with the calculation: P03.md §18.12. Multi-RHS survivor: P03.md §18.5 response batches, KILL-RESPONSE (strengthened by Block B08). The "scalar path at four or fewer contacts" was the crossover against the tensor-core tile path in the dual (Delassus) formulation; the plan solves the primal system H = A + JᵀGJ and builds no tile path, so there is no crossover to state. No block. (see a.1) |
| K-042 lattice-Boltzmann tiles inside training (survivors: slosh ports, film, fluid solver as evaluation oracle) | §3 | COVERED | P04.md §19.10.1 (Mach arithmetic), §19.10.3, §19.10.4 (equivalent mechanical slosh model), §19.10.6; P12.md §33.9 (fluid solver on the evaluation tier only). (see a.1) |
| K-043 full multiphysics in every world (survivors: tiered zoom, reduced terramechanics, offline challenges) | §3 | COVERED | P04.md §19.10 (three tiers); P03.md §18.10 (pressure-sinkage and shear laws). (see a.1) |
| K-060 clairvoyant barrier condition (survivor: one command for every admitted disturbance and error) | §3 | COVERED | P06.md §22.12.1 "Quantifier order"; P01.md C-31. |
| K-060 friction quantifier and closed-form projection (survivors: split cones; sort-and-clip; KKT check otherwise) | §3 | COVERED | P06.md §22.12.1 "Split cones", §22.12.2 (exact one-row breakpoint solution, the (1.25, 0.25) counterexample, mandatory optimality check, more than four rows refused). |
| K-070 log-posynomial feasibility (survivor: narrow monomial screens plus proved interval exclusion boxes) | §3 | WEAKER | Killed form named in P07.md §25.6; interval boxes present. Monomial screens absent. Block B19. |
| K-070 sample only inside a certified inner ellipsoid (survivor: certificate-backed local boxes) | §3 | COVERED | P07.md §25.6 (killed form named; per-constraint boxes). |
| K-072 unbounded Gröbner synthesis | §3 | COVERED | P07.md §25.21.3 and §25.21.4. |
| K-001 run-time composition wrappers (survivor: compile-time lowering) | §3 | COVERED | P02.md §14.6, §14.9 (modules compiled and memoized); P01.md C-12. (see a.1) |
| K-013 global Koopman contact control (survivor: one bounded soil-regime experiment) | §3 | MISSING | As the rank 14 row. (see a.1) Block B11. |
| K-066 Conley index; K-068 exact reachability (survivors: symmetry-orbit mode strata; max-plus scheduling) | §3 | COVERED | P01.md C-16 (27 strata, 162 transitions), C-10 (max-plus timing); P07.md §25.6 (cylindrical decomposition named as killed). (see a.1) |
| K-022 interaction calendar; K-023 time-parallel rollouts (survivor: optional sparse-warehouse lane) | §3 | MISSING | Neither strong form appears. The optional late lane for sparse multi-robot warehouses (a conservative next-interaction bound between isolated islands) is absent. (see a.1) Block B03. |
| K-036 staged erasure (survivor: a compiler rule) | §3 | COVERED | P01.md C-02 (static frames are phantom types; model-indexed frames checked at model compile time, "batched kernels pay nothing"), C-14; P05.md §21 (arithmetic derived once at build time; certificates are build artifacts). |
| K-038 zero-copy CPU/GPU handoff (survivors: fenced snapshots; CPU shadow auditor) | §3 | COVERED | P03.md §16.11 ("it needs a snapshot, not shared memory", GATE-FID-07, KILL-SHADOW). (see a.1) |
| K-017 derived launch-cost advantage (survivor: novel-topology admission time, measured) | §3 | COVERED | P03.md §17.8; P05.md §36.15 row 20 "Edit to first step". (see a.1) |
| K-018 megakernel occupancy and lossless limb summary (survivor: limb port kernels as an intermediate tier and race arm) | §3 | WEAKER | Closest forms: family lanes (P03.md §17.6) and the per-family-template control arm B (§17.14). Per-limb precompiled kernels joined at a port summary are not a tier and not an arm of the head-to-head. (see a.1) Block B07. |
| K-011 batched continuation cost (survivor: the mathematics; the return-map gate) | §3 | COVERED | P06.md §22.9.5, GATE-GAIT-02, -03, KILL-GAIT; no per-slice timing claimed. (see a.1) |
| K-082, K-083, K-002, K-037 headline ratios (survivor: mechanisms with honest numbers) | §3 | COVERED | No headline multiplier is printed; P03.md §17.11 (11.6 and 2.0), P08.md §26.4 (ratio "near 6"), P04.md §19.15. |
| K-061 probability bound (survivors: typed evidence product; conditional bound with independent labels) | §3 | COVERED | P06.md §22.12.3 (bound, independent hidden-state labels, self-labeling counterexample, not claimed under shift); P00.md §8.2. |
| K-004 reward-hacking figure; K-100 "1,800×" (survivors: unit-normalised margins; lifecycle as Estimated) | §3 | COVERED | P08.md §24.5, §24.6 (normalized margins, past-time operators online, future-time offline); P01.md C-30; P10.md §31.14. (see a.1 for K-004) |
| K-044 causal tile reuse (survivor: far-field fields only) | §3 | MISSING | Forks are full replays (P08.md §27.3, P09.md §29.8); scene variants reuse bricks at compile time (P04.md §19.15). No reuse of far-field living-world tiles across forks, with its dirty-fraction bound, exists. Block B09. |
| K-029 gradients through everything (survivors: regime-local and return-map gradients) | §3 | COVERED | P01.md C-16 derivative labels; P06.md §22.9.3, §22.9.6; P03.md §17.13 (binary outcomes refused). (see a.1) |
| Unconstrained zero-sum adversarial worlds; automated board-support generation | §3 | COVERED | P08.md §26.2 ADR-NEMESIS-01, -02; P10.md §31.13 ("no register access, no bus driver and no stop-circuit code"). |
| K-086 adaptive-fidelity decision | §3 | COVERED | As the K-086 row. |
| K-105 decision reuse beyond caching | §3 | COVERED | P09.md §28.13 (withdrawn for want of a certificate); P07.md §25.22 decision-reuse pilot row, KILL-AUGUR-REUSE. |
| K-062, K-052 "new family query" | §3 | COVERED | P00.md §2.4 lever 5 and §2.6 ("deriving the abstraction from a rich plant is cheaper than an expert doing it by hand"); P07.md §25.22 ("Every pilot's strong baseline tied or won"). |
| Hypothesis: faster numerics give a ten-fold decision | §3 | COVERED | P00.md §2.5 (1.345× warm, 1.088× cold, ceiling 1.46×), §2.7. |
| K-051 simulated humans; K-106; learned tier correction; mixed precision | §3 | COVERED | P00.md NONGOAL-16; P04.md §19.7 (scripted actors as scenario dressing, KILL-DRESSING). The tier correction of P03.md §16.8 stays unbiased under a wrong prediction model, so it is not the killed learned correction. |
| BS-1 observation-alias impossibility witness (K-052) | §4.1 | COVERED | P04.md §20.12 (`sense.alias`, priced separating changes, KILL-ALIAS on one false witness), GATE-SENS-09; P01.md C-19; P07.md §25.10, §25.18. |
| BS-2 schedule and timing-race check (K-064) | §4.1 | COVERED | P04.md §20.4.7 (one representative per commutation class; 720 orderings in 8 classes; planted stale read and watchdog race), KILL-SCHED; P01.md C-19. |
| BS-3 blinded qualification and α-spending (K-087) | §4.1 | COVERED | P08.md §24.13 (α_i = α/[i·(i + 1)], fresh sealed clusters), GATE-QUAL-01 to -03. |
| B1 replay determinism engineered, not asserted (K-030) | §4.1 | COVERED | P05.md §35.1 to §35.7 (classes, bans, count-scan-write, fixed-order trees), §35.3 (pairing refused under an uncontrolled class; the 0.95 to 0.50 arithmetic); P03.md §16.6. |
| B2 model-form robustness of rankings (K-065) | §4.1 | COVERED | P03.md §16.9, §16.10; P06.md §23.16 (√2·μ box-cone exploit, energy audit, `model-conditional`). |
| B3 the building as a design variable (K-046) | §4.1 | COVERED | P04.md §19.12 (catalog of about forty amendments, install classes, 10,700 kits, policy identity unchanged), GATE-HAB-11, KILL-RETROFIT. |
| A speed test can be powerless (K-112) | §4.2 | COVERED | P00.md §0.7 ("The ceiling is computed before any speed test"); P05.md §36.5 "Ceiling first" with a refusal. |
| A frozen decision rule may be unable to return some verdicts (K-112) | §4.2 | COVERED | P08.md §24.10, §24.12 (`eval.preanalyze`; 2,145 count pairs at n = 64), GATE-ARENA-05; P01.md T-26. |
| Local contracts hide time correlation (K-001) | §4.2 | COVERED | P02.md §14.6 (two fingers on a unit-current bus); P09.md §28.4 A-23; P10.md §31.12. |
| In-family checks cannot expose structural error (K-053) | §4.2 | COVERED | P09.md §28.16 (675 of 675, 0 of 225, 900 of 900; direct measurement always listed); P07.md §25.14. |
| The actuator authority model changes the hardware decision (K-074) | §4.2 | COVERED | P07.md §25.13 (155 N against 175 N on a 160 N rating); P01.md D-19, C-32. |
| Widening a hypothesis set invalidates certificates wholesale | §4.2 | COVERED | P07.md §25.15 (gain 33/32; 34,590 labels reverse); P01.md C-18; P04.md §20.16; P09.md §28.13. |
| Controller clock and rotor inertia flip benchmark outcomes | §4.2 | COVERED | P06.md §23.16 (`rate-sensitive` flag; 0.093 and 0.780 to 1.000 and 1.000); P09.md §28.4 A-22. |
| Contact counts diverge inside a same-morphology batch | §4.2 | COVERED | P03.md §17.4 ("homogeneity by morphology is not homogeneity by work"), §16.6 stages 5 to 8. |
| A screen can throw away the winner | §4.2 | COVERED | P09.md §28.12; P08.md §26.10. |
| Monitors cannot see a breach until it is too late | §4.2 | WEAKER | P00.md BET-25 names the control (two friction regimes with identical histories before first slip) and asks Appendix N for "the pre-slip aliasing fixture"; P06.md §22.12.3 states the bound. No chapter defines the fixture, its numbers, a rule for when a monitored assumption may count, or a gate. Block B12. |
| After the engine is fast, agent think time dominates | §4.2 | COVERED | P00.md §2.3 item 3 and §2.4 lever 1; P11.md §32 `solve.run`, KILL-SOLVE; P07.md §25.3 (`mission.synthesize` as the one-verb route). |
| The plans measure themselves where they trail | §4.2 | COVERED | P05.md §36.3 (rows stated as ratios to the incumbent re-measured on `trj`), §36.16; P00.md §1.1. |
| Scorer addition: a constrained allocator with refusal reporting under any shared policy | §4.2 | COVERED | P06.md §22.10 layer 1 (allocation program with a feasible witness or an infeasibility witness, residual acting above it), §22.5, §22.6. |
| Scorer addition: a two-mode sampled-data shield fixture with a verified backup domain | §4.2 | WEAKER | P06.md §22.12.3 (backups with declared domains), §22.17 (braking pilot for reachable-set tables). No fixture with the inter-sample term worked by hand and no stated backup-domain invariance. Block B12. |
| Scorer addition: a native paired-intervention harness before any browser bundle | §4.2 | WEAKER | The native harness exists (P08.md §27) and so does the HTML export (P04.md §20.10); the ordering rule is not stated. Block B29. |
| Scorer addition: "edit to first accepted result" as the unit for batching claims | §4.2 | COVERED | P03.md §17.14 measure (1); §17.8. |

### a.3 Duel report, the three reaction files, graveyard harvest, artifact compilation: 95 covered, 6 weaker, 4 missing

| Item | Source | Status | Where covered, or what is missing |
|---|---|---|---|
| Design-decision benchmark against the incumbent and the composed incumbent, as a censored restricted-mean-time protocol with frozen utility, thresholds, tuning budgets, recipes, hidden briefs and adjudication | DUEL report: consensus | COVERED | P00.md §2.1 (answer, clock, restricted mean time, missed deadlines cost H, strongest composed rival, cold and warm, crossover reuse count, one false certificate), §2.6, §4 (BET-13, BET-14 tie rule); P09.md §28.23 GATE-ATEL-03. The final decision rule belongs to §39, not yet written |
| Paired multilevel estimation across tiers, with a fitted frozen coefficient and ρ measured | DUEL report: consensus | COVERED | P01.md C-25 (control variate with β, multilevel, doubly robust; limits 1/(1 − ρ²)); P03.md §16.8; P09.md §29.11 (mᵢ = c + β·Y_fast frozen from earlier ledger rows; ρ_ep on 200 addresses; applied to the paired contrast); KILL-DR |
| Ragged cohorts: per-world model identity, bounded execution shapes, tree-ordered factor | DUEL report: consensus | COVERED | P03.md §17.1 to §17.5, §17.8; P03.md §16.6 (leaves-first factor with no fill, pilot); P05.md §21.11; GATE-PHYLUM-02 |
| Counterfactual probes narrowed to diagnostic interventions; design edits replay from reset | DUEL report: consensus | COVERED | P08.md §27.3 to §27.8; P09.md §29.8 (fork classification, structural substitution restarts from the authored initial condition, FROB-REF-882); P09.md §28.19.2 DC-06 |
| Sampling controller nested in the batch on each design's own model | DUEL report: consensus | COVERED (narrowed to a screening rung and a fallback, never the evaluator of record) | P06.md §22.8 (algorithm, cost arithmetic, plant-equals-model flag, FROB-REF-817); P09.md §28.9; GATE-REFLEX-06, KILL-MPC. The objection that a fixed-budget planner ranks easy-to-plan bodies first is answered only by rank agreement; see item 23 |
| Rare-failure search over a scene grammar | DUEL report: consensus | COVERED | P08.md §26.3, §26.4, §26.9 (defensive mixture for discrete scene edits, weights bounded by 1/ε, Good–Turing monitor, exploration never issues a bound); P09.md §28.19.2 DC-04 |
| Gait-cycle adjoints for design gradients | DUEL report: consensus | COVERED | P01.md C-16; P06.md §22.9 (`gait.gradient`, implicit-function form at the fixed point, refused at a fold, screening label); KILL-GAIT |
| Universal zero-training sampling controller, with coarse-then-exact annealing and planning on the execution model | DUEL report: contested | COVERED (narrowed) | P06.md §22.8 (planning configuration in `ControllerId`, annealing iterations, declared latency); the unmeasured throughput figures are absent |
| Closed-loop structural link synthesis with manufacturability and tolerances, as a bounded finalist lane with referee-tier loads; fabrication readiness a separate claim | DUEL report: contested | WEAKER | P10.md §31.2 (finalists only), §31.5 to §31.12, flags of §31.12. Missing: §31.6 names no tier for the load campaign, so an envelope case from a fast tier can enter a build packet without re-extraction on the referee. Block B27 |
| Implicit-function contact and morphology adjoints, narrowed to a fixed-regime pilot and local margin sensitivities | DUEL report: contested | COVERED (narrowed) | P03.md §16.12 (fixed-mode derivatives, refusal near grazing or ambiguous simultaneity); P06.md §22.9; P09.md §28.4 (A-16, A-26); P00.md §6 NONGOAL-15 |
| Interventional causal debugging burst: intervention battery and solver-artifact slots | DUEL report: contested | COVERED | P08.md §27.5, §27.6, §27.11; P06.md §23.16 (perturbation table). The 50 ms figure and "exact causal proof" are absent |
| Pareto antichains over discrete part catalogs, frontier verified by enumeration | DUEL report: contested | COVERED | P07.md §25.21.1; §25.7 (typed holes, checked cuts, pilot exhausting a frozen catalog of 1,024 leaves); KILL-SYNTH-CODESIGN |
| Sharing test cases across symmetry orbits | DUEL report: contested | COVERED (narrowed to the observation-preserving subgroup) | P03.md §17.11; P02.md §13.4 (reduction needs the same world; augmentation may pair two); P09.md §28.11 stage 1 |
| Manufacturing variation into part decisions | DUEL report: contested | COVERED | P10.md §31.11 (correlated stacks, band-extreme check, FROB-REF-165), §30.14 as-built audit |
| Nonholonomic Lie-bracket primitives and a swept passability prover | DUEL report: contested | COVERED | P01.md C-07; P09.md §28.4 (A-07, A-18); P04.md §19.11 |
| Contact wrench cone and mode-transition planner on a small explicit mode graph | DUEL report: contested | COVERED (narrowed) | P09.md §28.4 (A-08, A-24 with a separating dual); P06.md §22.4 (explicit drive, stand and step mode graph), §22.10 |
| Symbolic kinematic-loop closure | DUEL report: contested | WEAKER | P02.md §15.9 (one sentence: a position-dependent ratio is a closure whose Jacobian is the transformer modulus); P03.md §16.12 (closures are compliant or bilateral rows); P01.md C-11 (closed forms for inverse kinematics only). Missing: compile-time closed-form elimination of single-loop planar closures with branch selection and refusal near the toggle. Block B05 |
| Gradients per contact regime, as a fixed-support CPU experiment | DUEL report: contested | COVERED (narrowed) | P03.md §16.12, §17.13; P09.md §28.4 A-26 with KILL-ATLAS |
| A link optimized from measured loads, preserving ports | DUEL report: contested | COVERED | P10.md §31.6 (time-coherent cases, unit loads), §31.7 (port bosses frozen) |
| Fused stepping kernel held as an option | DUEL report: contested (dispute) | COVERED | P05.md §21.18, §36.11 (profile-gated fused assemble, factor and solve; tested at 18, 30 and 60) |
| Persistent whole-robot kernel narrowed to a tree-ordered factor, local fusion and a direct Metal path | DUEL report: contested (dispute) | COVERED (narrowed) | P03.md §16.6; P05.md §21.9, §21.18; KILL-MEGA |
| Randomized referee audits with logged propensities as the validity layer beside paired estimation | DUEL report: contested (dispute) | COVERED | P03.md §16.8 (audit probability fixed and logged, assignment persisted before dispatch); P09.md §29.11; P01.md C-25 |
| Rank designs by achievable performance from curves of score against controller budget | REACTIONS CC; DUEL report: blind spot | MISSING | The bias is acknowledged (P09.md §28.9, §28.10 per-region margins) but there is no budget coordinate, curve fit, saturation index, `controller-limited` flag or escalation rule. Block B24 |
| Bracket what the embodiment can achieve before paying to train controllers | REACTIONS COD; DUEL report: blind spot | WEAKER | Two-sided feasibility exists: P07.md §25.1 (I ⊆ F ⊆ O), §25.4.5 Rules 1 and 2; P09.md §28.4 (A-20, A-24). Missing: a bracket on a bounded task utility, the stop that ends controller effort when a checked upper bound is below a competitor's achieved lower bound, and the body-limited and controller-limited labels. Block B20 |
| Attack the task specification before the design (deliberately capable or degenerate bodies, intent invariants) | REACTIONS CC; DUEL report: blind spot | MISSING | Nearest: the vacuous-atom check of P08.md §24.5 and the detectors of P06.md §23.16, which test solver artifacts only. Block B16 |
| A calibrated item bank for the battery, sampled by importance | REACTIONS CC; DUEL report: blind spot | MISSING | Nearest: battery selection by coverage (P08.md §26.11, P09.md DC-10), which chooses scenes and does not weight them per comparison. Block B17 |
| Common randomness compiled from the task, with a test of what "the same world" means | REACTIONS COD; DUEL report: blind spot | WEAKER | P05.md §35.9 (streams keyed by the physical entity), P09.md §28.6 pairing rule 2, P03.md §16.5 (dyadic refinement law), P04.md §20.17. Missing: the event-triggered exposure domain keyed by mission role and occurrence, the marginal-invariance and retiming test, and the lock of a coupling chosen on pilot seeds. Block B18 |
| Inverse-dynamics convex synthesis through the linearity of inverse dynamics in the inertial parameters | REACTIONS AGY; DUEL report: blind spot | MISSING | Nearest: the mass-placement program and trajectory optimization of P07.md §25.21, A-24 of P09.md §28.4. Block B21 |
| Pre-characterized sub-assembly macromodels at the ports, composed to screen assemblies | REACTIONS AGY; DUEL report: blind spot | WEAKER | P02.md §14.6 (module contract with guarantees and assumptions, limb port packet). Missing: capability tables characterized once over an operating box, the composed quasi-static screen, the validity regime and a recall gate. Block B01 |
| Estimator observability and self-occlusion co-designed with morphology | REACTIONS AGY; DUEL report: blind spot | WEAKER | P01.md C-20; P04.md §20.11 (Gramian along a route, placement score [F]); P09.md §28.4 (A-14, A-17). Missing: occlusion by the robot's own limbs and payload, lead and blind-interval metrics, sensor mounts as optimizer variables against the stability trade, and the privileged-against-estimated gap as falsifier. Block B10 |
| Family policy demoted to a screening role with a regret falsifier | DUEL report: killed (conceded) | COVERED | P06.md §23.11; P09.md §28.9 (R1 discards under §28.10); KILL-FAMILY, GATE-CORTEX-12 |
| Controller transfer across typed edits, keeping the role and frame correspondence check | DUEL report: killed (conceded) | COVERED | P06.md §23.7 (mapping table matched by role, FROB-REF-531), §23.12; KILL-INHERIT |
| Route-feasibility evidence narrowed to a wheeled and tracked geometry lane | DUEL report: killed (conceded) | COVERED | P04.md §19.11; KILL-FCLEAR |
| Learned morphology-conditioned dynamics; learned fast-to-referee correction | DUEL report: killed | COVERED (rejected by name) | P09.md §28.19.4 |
| Multi-agent language-model design debate | DUEL report: killed | COVERED (absent) | No occurrence in P00 to P12 |
| Population-scale or universal deformable soil in every world | DUEL report: killed | COVERED (absent) | No occurrence in P00 to P12; soft soil appears only as a later frontier epic in the milestone skeleton |
| Video-to-splat scenes for RGB training | DUEL report: killed | COVERED (absent) | No occurrence in P00 to P12 |
| Stator-winding and gearbox co-design | DUEL report: killed | COVERED (absent) | Gear and actuator selection only from catalogs, P07.md §25.21.1 |
| Gradients through all contact changes | DUEL report: killed | COVERED | P00.md §6 NONGOAL-15; P03.md §16.12 |
| Live editing in the viewer | DUEL report: killed | COVERED (absent) | No occurrence in P00 to P12 |
| Symmetry matched to a broken task context | DUEL report: killed (one score) | COVERED (narrowed) | P06.md §22.13: the stabilizer intersection licenses reductions inside a calculation; the full model stays executable. The stated objection (discontinuous actions when the group changes at a mode switch) is not discussed |
| Actuator-identification experiments | DUEL report: killed (one score) | COVERED (narrowed) | P10.md §30.6 (`ground.excite`, identifiability A-25) |
| Incremental contact-geometry builds; index of contact-compatible modules | DUEL report: killed (one score) | COVERED (absent as mechanisms) | Scene compile is a memo node, P09.md §28.14 |
| Geometric phase as an experiment generator | DUEL report: killed (one score) | COVERED (narrowed) | P07.md §25.19: [F] on kinematic fixtures, [M] as an atlas |
| Conceded headline claims (multiplied time table, 20 to 30 M steps/s, 50 ms) | DUEL report: killed (conceded) | COVERED (absent) | P00.md §2.2 to §2.5 use an Amdahl ledger and ceilings |
| Benchmark power: a modest number of paired briefs separates a ratio of 10 from 3 | DUEL report: post-reveal revision | COVERED | P00.md §2.1 (pooled ratio, at least ten paired campaigns per arm, power computed before the suite is opened) |
| Catalog wedge: a sound cut on a partial assignment; discrete selection tested early | DUEL report: post-reveal revision | COVERED | P07.md §25.7, §25.21.1 pilot |
| Sparse brick-tree distance fields with sharp-edge overlays | DUEL report: contested (matrix) | COVERED | P04.md §19.3, §19.4, §19.15 |
| Adversarial interior and disturbance generator | DUEL report: contested (matrix) | COVERED | P08.md §26.3, §26.4 |
| Certified envelope prover and minimal unsatisfiable specification diagnosis | DUEL report: contested (matrix) | COVERED | P07.md §25.6, §25.7 (inconsistent missions, FROB-REF-877) |
| Local energy balance per port for solver triage | DUEL report: contested (matrix) | COVERED | P09.md §29.9 (every term a port quantity; residual as its own line; contact passivity per contact episode) |
| Paired-episode failure clustering | DUEL report: contested (matrix) | COVERED | P09.md §29.8 (first divergence between arms), §29.10; P08.md §26.9 (mode catalogue) |
| Mac for scene diversity, CUDA for training | DUEL report: contested (matrix) | COVERED | P05.md §21.17; P04.md §19.15 |
| Cross-family mission compiler | DUEL report: contested (matrix) | COVERED | P07.md §25.7 (starter designs across families, with a falsifier) |
| Real-place twin augmentation | DUEL report: contested (matrix) | COVERED | P04.md §19.13; P10.md §30 (`ground.ingest`) |
| Bounded reduced model per locomotion regime | DUEL report: contested (matrix) | COVERED | P09.md §28.4 A-26 |
| Broken-symmetry perturbation calculus | DUEL report: contested (matrix) | COVERED (narrowed) | P02.md §13.4 (near-symmetry report with the restoring edit); P06.md §22.13 |
| Port-Hamiltonian passive actuators and contact | DUEL report: contested (matrix) | COVERED | P02.md §15.5; P01.md C-38 |
| Koopman or Floquet limit-cycle stability oracle | DUEL report: contested (matrix) | COVERED (narrowed to Floquet multipliers) | P06.md §22.9 |
| Sustained-duty thermal and airflow cards | DUEL report: contested (matrix) | COVERED | P02.md §15.7, §14.11 stage 6 |
| Asymmetric pipeline on two cards without a link | DUEL report: contested (matrix) | COVERED | P06.md §23.9; P05.md §21.17, §36.13 |
| Batching topology churn by execution shape | DUEL report: near miss | COVERED | P03.md §17.3 to §17.5 |
| Interval and monotone carving of design space | DUEL report: near miss | COVERED (narrowed) | P07.md §25.4.5 Rule 3, §25.6 (small sub-boxes) |
| Content-addressed evaluation cache; canonical designs | DUEL report: near miss | COVERED | P09.md §28.14, §28.15; P03.md §17.11 |
| CPU shadow auditor on idle cores | DUEL report: no consensus | COVERED | P03.md §16.11; P09.md §29.11 (background sampler) |
| Per-building passability map | DUEL report: single score | COVERED | P04.md §19.11; P09.md §28.4 A-18 |
| Memoize everything downstream of the model identity | GRAVEYARD card 1 | COVERED | P09.md §28.14 (derived keys with property tests, one hit in 64 recomputed, loss model, 64 GB, eviction never blocks, rollout order, `--no-memo`, percentile targets); PO-ATEL-07; GATE-ATEL-12 |
| Slot-level episode dispatch in the daemon | GRAVEYARD card 2 | COVERED | P05.md §21.11 (persistent environments, KILL-SLOT); P03.md §17.8; P11.md §32.11.1 to §32.11.3 (budgeted mode, proof obligation, shadow rollout, guard table); GATE-TETHER-09 |
| Paired multi-fidelity estimates with exact importance weights | GRAVEYARD card 3 | COVERED | P03.md §16.8; P09.md §29.11 (loss model 100, 1, 10; 256 episodes; 30% effective-sample trigger); P08.md §26.9. The median-of-means countermeasure is replaced by a floor on audit probabilities |
| Conformal interval on `sim.estimate` | GRAVEYARD candidate | COVERED | P05.md §36.8 (every estimate an interval); P09.md §28.19.2 DC-09, §28.17 |
| S3-FIFO eviction for the memo store and the scene cache | GRAVEYARD candidate | COVERED | P09.md §28.14 (admission weighted by recompute cost); P04.md §19.15; P12.md FD-FSQL-01 |
| Deterministic simulation testing of the daemon and ledger with lineage-driven faults | GRAVEYARD candidate | COVERED | P09.md §29.15 (dual-run scenarios keep the fake honest); GATE-CHRON-11 |
| Shortest estimated work first, with ageing | GRAVEYARD candidate | COVERED | P11.md §32.11.2 |
| Shared brick pool with one directory per scene variant | GRAVEYARD candidate | COVERED | P04.md §19.15; GATE-HAB-13 |
| Warm template environments; fork by indexed copy | GRAVEYARD candidate | COVERED | P05.md §21.11; P06.md §22.8 (`K-fork`); P08.md §26.9 (child addresses, correlated continuations stated) |
| Atomics only for integer maximum, minimum, or and add | GRAVEYARD candidate | COVERED | P05.md §21.6, §35.7 |
| Telemetry sample by address hash with weight 1/p | GRAVEYARD candidate | COVERED | P09.md §29.6 |
| Lineage and archive merge as grow-only sets with a total order | GRAVEYARD candidate | COVERED | P09.md §28.7, INV-ATEL-09; P03.md §17.12 |
| One stable integer-key sort | GRAVEYARD candidate | COVERED | P05.md §21.5 (`K-sort`, `K-compact`), §35.7 |
| Segmented arrays and size classes instead of padding to the largest robot | GRAVEYARD candidate | COVERED | P03.md §17.4, §17.5 |
| Fused assemble, factor and solve | GRAVEYARD candidate | COVERED | P05.md §21.18, §36.11 |
| Integer log-bin histograms | GRAVEYARD candidate | COVERED | P09.md §29.7; P05.md §35.7 |
| Factorization staged per topology | GRAVEYARD candidate | COVERED | P03.md §16.6, §17.9 (KILL-SPECIALIZE) |
| E-graph extraction beyond common-subexpression elimination | GRAVEYARD candidate | COVERED (profile-gated) | P05.md §21.7; P12.md §33 (frankenjax as a pattern); P09.md §28.19.4 |
| Rejected set (superoptimization, polyhedral scheduling, perfect hashing, copy-on-write device state, replication protocols, memory tiering, bandits) | GRAVEYARD rejected | COVERED (absent or rejected by name) | P05.md §21.20; P09.md §28.19.4 |
| Artifact-graph identifiers | GRAVEYARD convention | COVERED | P12.md §34.12; P09.md §29.13 |
| Contracts as code | GRAVEYARD convention | COVERED | P12.md §34.5, §34.12; P09.md §28.19.1 |
| Shadow run as the adoption wedge | GRAVEYARD convention | COVERED | P12.md §34.12; P09.md §28.19.1 |
| Demo gate and budgets with a safe mode | GRAVEYARD convention | COVERED | P12.md §34.12 (`demo.yaml`, safe mode in `contract.toml`) |
| Two planes | GRAVEYARD convention | COVERED | P09.md §28.2 ADR-ATEL-10; P12.md §34 rule L-11 |
| Controller gate | GRAVEYARD convention | COVERED | P09.md §28.2 ADR-ATEL-09; P12.md §34.12 |
| F1 sequential evidence | ARTIFACT F1 | COVERED | P09.md §28.19.2 DC-01; P03.md §16.8 |
| F2 conformal prediction and risk control (margins, audit lane, reality band, effort profiles) | ARTIFACT F2 | COVERED | P09.md §28.9 layer 2; §28.19.2 DC-02, DC-03, DC-07, DC-08 |
| F3 value of information | ARTIFACT F3 | COVERED | P09.md §28.19.2 DC-05, §28.16; KILL-VOI |
| F4 rare-event simulation | ARTIFACT F4 | COVERED | P09.md §28.19.2 DC-04; P08.md §26.9 |
| F5 counterfactual causation | ARTIFACT F5 | COVERED | P09.md §28.19.2 DC-06; P08.md §27.5 to §27.7 |
| F6 submodular coverage | ARTIFACT F6 | COVERED | P09.md §28.19.2 DC-10; P08.md §26.11 |
| F7 queueing | ARTIFACT F7 | COVERED | P09.md §28.19.2 DC-09; P11.md §32.11 |
| F8 certified rewrites | ARTIFACT F8 | COVERED | P09.md §28.19.2 DC-11; P05.md §21.7 |
| Failure modes, fallbacks, triggers and budgeted modes per family | ARTIFACT table | COVERED | P09.md §28.19.2 (every core) |
| Worked cards A1 to A8 | ARTIFACT cards | COVERED | P09.md §28.19.2 worked examples; §28.9 (ladder cost case); P11.md §32.11.2 |
| Composition: timescales, shared quantities, interference tests | ARTIFACT composition | COVERED | P09.md §28.19.3; GATE-ATEL-14 |
| Methods rejected as ornamental or premature | ARTIFACT rejected | COVERED | P09.md §28.19.4 |
| RC-1 promotion guard table | ARTIFACT contract | COVERED | P03.md §16.8; P09.md DC-01, PO-ATEL-01 |
| RC-2 ladder allocation by conformal survivors | ARTIFACT contract | COVERED | P09.md §28.9; GATE-ATEL-08; KILL-LADDER |

## (b) Changes required by the adjudication

Source: `planning/fable/brenner/deliverables/CHALLENGE_ADJUDICATION.md`. All 16 contested register rows change
(11 rulings upheld, 16 partly upheld). Each item below names every place in the parts that states the old verdict
(found by grep over `P00.md` to `P12.md`) and gives the corrected wording as final plan text. "Block Bnn" (B01 to B30) refers to
an insertion block of section (c). Line numbers are those of the part files as read on 2026-11-21.

### b.1 The register printed in §0.8 (`P00.md` lines 303–345)

**Replace these rows** (same five columns as the printed register). Rows not listed are unchanged.

| ID | Hypothesis | State | What survives | Where it binds |
|---|---|---|---|---|
| H-01 | Sampling control with zero training | NARROWED | A screening rung and an out-of-distribution fallback, at about 0.3–1 GPU-minute per quadruped episode and 1–5 per humanoid episode: cheaper than one training run up to about 30 episodes per quadruped design and 60 per humanoid design at a 20 ms planning step (6 and 12 at 4 ms). The published successes are legged robots with hand-built swing or stepping references; there is no success rate over a varied corpus and nothing for tracked robots. It never stands in for a trained policy as acceptance evidence. Where the sampling controller is itself the delivered controller, its own episodes are the evidence; the break-even against training once is 2.0 GPU-seconds per 20 s episode for a quadruped and 12 for a humanoid, which published settings miss by 10× and 5×. | Reflex has a sampling controller as a screening rung, a fallback and a demonstration source, and a delivered-controller lane with a frozen planner record (§22.8; BET-27). |
| H-02 | A universal policy prior | NARROWED | Within one morphology class, on flat or mildly varied ground with fixed actuation: zero-shot at 70–85% of specialist return; fine-tuning at about 2–25% of a from-scratch run stopped at matched return (1–10% against the fixed budget one source used). Across topology-diverse designs at affordable pretraining: zero-shot at 10–40%, a fine-tuning gain of 2–3× at best and none on flat ground. Within a class on interior terrain, with payload or actuator-card changes: no published figure. | Family policies per morphology class, not one universal policy. A gated experiment on interior terrain, against an early-stopped from-scratch control, that needs an owner-approved compute envelope (§23; BET-15). |
| H-03 | Topology as data | NARROWED | Mixed-topology populations with few worlds per design, in size buckets. One incumbent solver publishes a mixed batch of three body plans at 0.86 of the harmonic mean of its homogeneous rates. That solver is maximal-coordinate, is documented as not deterministic, and is reported as slower than the reduced-coordinate solvers of its framework. Three differentiators are left: measured mixed-batch throughput, the time from an edit to the first GPU step, and run-to-run replay and forks inside a mixed batch, which no incumbent document read claims. | Per-world model variants inside padded capacities; a head-to-head experiment with a per-family-template control arm and a digest check inside the mixed batch (§17; BET-18). |
| H-04 | Supercompiled kernels | REFUTED for recursion-only specialization | The Lie-algebraic recursions are 16–20% of two incumbent step traces, so the ceiling is about 1.2× and reaching it needs the recursions more than five times faster. Derived derivative kernels for CPU solvers survive. Reversal condition: a specializable share of at least one third on our own profile. | Specialization only for survivors, and only past the reversal condition. Derivation stays, for correctness, certificates and CPU solvers (§21; BET-01). |
| H-04b | Structure-exploiting constraint assembly and solve | ACTIVE | Constraint assembly and the solve are 62% of the same incumbent trace: a ceiling of 2.6× if both vanished and 1.7× at a three-fold reduction. Ancestor-sparse rows and a tree-ordered factor cut operation counts 2.6× to 4.1× for the factor and 3.8× to 8.1× per foot row. The shares of those operations inside the solve stage are unmeasured, fewer operations do not imply a faster device step, and the sparsity is carried as data, not compiled per design. | An operation ledger on the CPU tier and a device measurement at M2, with a kill gate. Not counted in the thesis of §2 (§16). |
| H-06 | Region certificates | NARROWED | Certificates per constraint. Bisection is confined to a split set of about six sensitive parameters; the others are enclosed at full range, and a box of any dimension is admitted when that enclosure is narrower than the constraint's margin. Isotropic paving of a 10–20 dimensional box is out of reach. Whether robot constraints have narrow weak-variable enclosures is unmeasured. | Family answers are built from per-constraint certificates, monotonicity and witnesses, not from brute paving (§12, §25; BET-20). |
| H-07 | A rare-event engine | NARROWED | Search finds failures; a bound comes from a proposal frozen before the confirming draws, with bounded weights, or from crude sampling. Published measured gains are 2–20× and about 5× in mean-square error; 10–100× at 10⁻⁴–10⁻⁵ is a TARGET. Proposals may act on static scene parameters as well as per-step disturbances; only snapshot reuse needs per-step noise. A failure mode the proposal misses costs up to 1/ε times crude sampling, so absence is never certified more cheaply. | Nemesis separates exploration from certification, and its bound claims refutation and tightening (§26; BET-32). |
| H-08 | Counterfactual autopsy | NARROWED | Interventional contrasts reported with a flip rate under perturbations at or above round-off scale. Temporary interventions fork from the snapshot; design-parameter interventions re-run from the episode start. Precision follows measured sensitivity: at the published exponent, single-precision round-off stays below 4 × 10⁻⁵ of state scale over fork windows up to about 1 s and reaches 2% at 2 s. Double precision is the default for one failure and the escalation for marginal cases. | Autopsy reports the stability of each attribution, uses a placebo fork, and may attribute on the originating tier under an admission rule (§27; BET-23). |
| H-09 | Incremental evaluation | REFUTED for exact reuse | Exact reuse of rollouts after a mass or geometry edit is at most about 10% of compute. Training avoided by warm starts is a different quantity; it belongs to H-21 and is counted once. A memo graph for analyses, step plans, kernels and scene compiles survives as hygiene. Dependency-directed revalidation of verdicts survives as an evidence rule. | The memo graph extends past model identity; "which verdicts survive an edit" is a verb; a reuse ledger keeps exact reuse, carried evidence and avoided reacquisition apart (§28; BET-33). |
| H-11 | An always-on referee | NARROWED | A sampled snapshot referee on any backend. A correctness guard that saves no compute. Its coverage is unmeasured and scales with referee speed: about 0.1–2.4% of GPU world-seconds if the referee runs at 0.2–1.0× real time per core on 24 cores, and more if it is faster. Its samples, drawn with recorded inclusion probabilities, also serve the paired tier correction. | A background referee sampler that reports its coverage and detection delay per hardware and backend pair (§16; BET-34). |
| H-12 | Generative structure | DEFERRED as a decision-time lever | Links 7–30% lighter than engineered parts: 2–8% of robot mass on the one quadruped with a published mass breakdown, and no figure for humanoids. It improves the output, not the time to a decision. | Foundry is an output-quality subsystem (§31; BET-29). |
| H-12b | Member sizing inside screening | ACTIVE | A closed-form check that a declared link wall survives the design's own loads keeps screening from rewarding links that cannot be built. How often it changes the winning actuator or gear ratio is unmeasured. | The beam rung runs at preflight and blocks promotion of under-built designs (§31, §14). |
| H-16 | Living multiphysics in every world | REFUTED as stated | Full-room fluid is out of memory by orders of magnitude, and coupled cloth and cable at batch speed are out of reach on present foundations. A contact-level film-friction law and rigid proxies are the [S] substitutes; whether they reproduce the failures of coupled models is untested. | The film law and the proxies are [S]. The proxy-adequacy pilot decides the proxies' claim and whether a coupled island is ever funded; islands are [M] (§19; BET-36). |
| H-17 | From mission to design by solving | NARROWED | Trajectory optimization with design variables as local refinement (about 12 degrees of freedom, up to 17 parameters, 4–10 minutes). Polynomial synthesis only inside the symbolic library's caps. Monotone catalog selection of actuators, batteries and compute, with no measured timing. The pass rate of synthesized designs under independent closed-loop evaluation is unknown. | A bounded exact synthesis lane, a refinement lane and catalog selection; continuation is [F] on the CPU; every synthesized candidate passes the ordinary ladder (§25). |
| H-18 | Information, not the engine, is the lever | ACTIVE | No instrumented design session exists. Where every candidate is trained, training is about 96% of an illustrative campaign and information features that do not remove training cap near 1.04×. Once controller acquisition is amortized, agent turns are about two thirds of elapsed time. Which regime a mission is in is a measurement. | The ten-fold test measures decisions, with a phase ledger and three single-change ablations (§2, §39). |
| H-19 | A thin layer over an incumbent would do | ACTIVE | Decided by a measured recovery ratio, not by a count of gaps. Documented engine-internal gaps: a mixed-topology batch with run-to-run replay; a Metal tier. Unverified: replay of one world outside its batch; solver-internal certificates. In the incumbent documents read, run-to-run determinism, heterogeneous worlds and the fastest solver sit on different solver paths, so each is available and their conjunction is not shown. | The ten-fold test has three arms and reports how much of the gain a composed incumbent recovers; the plan says what it does if that arm ties (§39; BET-14). |
| H-21 | Policy inheritance | NARROWED | No published figure measures a small typed parameter edit from parent to child. Transfer from a multi-embodiment prior to a similar body costs about 2–25% of a from-scratch run stopped at matched return; the one parent-to-child figure is about 50%. Single-limb topology edits: 33–100%. Chained inheritance risks loss of plasticity and narrows morphological diversity. | Inheritance for parameter edits must beat an early-stopped from-scratch control per edit class before it is the default. Resets follow a measured regression of an audit twin, with a depth schedule as fallback (§23; BET-16). |
| H-22a | Rank fidelity between rungs | ACTIVE | No published rank agreement between a shared policy and specialists was found. The nearest figure, 0.40, compares a critic with its own policy. A high rank correlation does not protect the winner: with 40 designs and the true best at cheap rank 3, τ is 0.91 on the top ten and a cut to two discards it. | The rank-agreement experiment is the decisive early experiment of the program, and it reports containment and regret beside τ (§23, §28; BET-17). |
| H-22b | The fast tier as a paired control | ACTIVE | A paired correction gains 1/(1 − ρ²) before cost: 2.0× at 0.7, 2.8× at 0.8, 5.3× at 0.9. The correlation is unmeasured and is reported separately for continuous metrics and for binary outcomes. | The tier-correction ledger of §16 and §29. |
| H-25 | Brackets on what a body can achieve | ACTIVE | A controller-independent upper bound and an achieved lower bound on task utility; controller effort stops only when a checked upper bound falls below a competitor's lower bound. The bound is often vacuous for contact-rich full-order tasks. | §25, §28 (Block B20). |
| H-26 | Common randomness compiled from the task | ACTIVE | Separate address domains for world fields, absolute time and task-triggered events keep paired draws paired across retiming and topology changes. A paired contrast's variance scales with 1 − ρ. Any change in a marginal distribution is a failure of the mechanism. | §24, §35 (Block B18). |
| H-27 | Step adjoints with short-horizon joint training | ACTIVE | Implicit-function adjoints through the converged contact solve are exact for a fixed active set and cheap. Several incumbents differentiate their step, so this is no differentiator. Gradients say nothing about outcomes decided by a change of contact sequence. | [M], off the critical path (§16, §23; Blocks B04 and B15). |
| H-28 | Placement and sizing along a prescribed motion | ACTIVE | Inverse dynamics is linear in inertial parameters and contact forces for a fixed motion, so placement and sizing is one cone program. Its certificate excludes a motion, not a design; gear ratios are enumerated; link lengths are not variables. | §25 (Block B21). |
| H-29 | Visibility and observability in sensor placement | ACTIVE | Visibility along a motion with the robot's own limbs and payload as occluders is a cheap, missing analysis. The observability index is exact only without bias states. The share of estimated-state failures it explains is unknown. | §20 (Block B10). |

**Replace the closing paragraph and list of §0.8** (lines 330–339, "The register's answer … BET-20).") with:

> The question the program was asked is open: no matched mission has been run on any pipeline. No single mechanism
> delivers ten-fold, and no conjunction has been shown to. §2 states the candidate conjunction and the two
> comparators it must be measured against. The register above was challenged after it was first adjudicated, and
> sixteen of its rows were corrected; the corrections are in the rows as printed. These disagreements remain open,
> and each has a test that settles it:
>
> - **Whether a thin layer over an incumbent recovers the gain.** Settled by the recovery ratio of §39 (BET-14).
> - **Whether design missions are dominated by training or by agent turns and diagnosis.** Settled by the phase
>   ledger and the three ablations of §39.
> - **Whether cheap rungs rank designs as specialists do, and how well tiers correlate.** Settled by the
>   rank-agreement experiment (§23, §28; BET-17).
> - **Whether a family policy holds within a class on interior terrain, and what an inherited edit costs.** Settled
>   by BET-15 and BET-16 against early-stopped controls (§23).
> - **Whether a sampling controller is more than a screen.** Settled by BET-06 and BET-27 on the design corpus, and
>   by the delivered-controller lane (§22).
> - **Whether a structure-exploiting solve beats parity, and whether per-design code adds to it.** Settled by the
>   operation ledger and our own M2 profile (§16, §21, §36).
> - **Whether rigid proxies reproduce the failures of coupled models.** Settled by the pilot of §19.10.7.
> - **Whether formal certificates change any design decision.** There is no evidence either way. Lemma records, for
>   every certificate consumed by a decision, whether the decision would have differed without it (§12; BET-11,
>   BET-20).

In the state table of §0.8 (line 294), the ACTIVE row needs no change. The sentence at line 289 ("Each hypothesis
received one of four verdicts") stays true: split hypotheses are printed as two rows.

### b.2 §2, the ten-fold thesis (`P00.md`)

| Where | Now reads | Corrected wording |
|---|---|---|
| §2.3, last paragraph (line 752) | "per-design kernel specialization is capped near 1.2× on an incumbent's step trace (H-04)" | "per-design specialization of the recursions is capped near 1.2× on an incumbent's step trace (H-04); whether structure-exploiting constraint assembly and solve give more than parity is open and is measured at M2 (H-04b, §16)" |
| §2.4, first sentence (line 758) | "No single mechanism delivers ten-fold (§0.8). The thesis is a conjunction of eight levers, each of which attacks a different row of the ledger." | "No single mechanism delivers ten-fold, and no conjunction has been shown to (§0.8). The candidate thesis is a conjunction of eight levers, each of which attacks a different row of the ledger. Seven of the eight are methods a composed rival can also host; §2.6 says what that implies." |
| §2.4, lever 1, surviving number (line 765) | "An incumbent's mixed batch runs at 0.86 of its homogeneous rate; published population methods report compute gains of about 3–4× (H-03, H-23)." | "One incumbent solver's mixed batch runs at 0.86 of the harmonic mean of its homogeneous rates; that solver is not deterministic and is slower than its framework's reduced-coordinate solvers. Published population methods report compute gains of about 3–4× (H-03, H-23)." |
| §2.4, lever 2, surviving number (line 766) | "Zero-shot at 70–85% of specialist return within a class; fine-tuning at 1–20% of from-scratch steps for parameter edits and 33–100% for single-limb topology edits (H-02, H-21). Rank agreement is unmeasured; the nearest published figure is 0.40 (H-22)." | "Zero-shot at 70–85% of specialist return within a class on flat or mildly varied ground; fine-tuning from a class prior at about 2–25% of a from-scratch run stopped at matched return; the one published parent-to-child figure is about 50%; single-limb topology edits 33–100% (H-02, H-21). Nothing is published for interior terrain within a class. Rank agreement is unmeasured; the nearest published figure, 0.40, is for an easier comparison (H-22a)." |
| §2.4, lever 4, surviving number (line 768) | "10–100× fewer episodes at failure probabilities of 10⁻⁴–10⁻⁵ for failures driven by per-step disturbances, given a good score (H-07)." | "Published measured gains in finding failures are 2–20×; 10–100× at failure probabilities of 10⁻⁴–10⁻⁵ is a TARGET. The gain is in finding failures and in refuting a failure-rate target; certifying absence is never cheaper than crude sampling (H-07)." |
| §2.4, lever 5, surviving number (line 769) | "Certificates per constraint on sub-boxes of about six parameters or fewer (H-06)" | "Certificates per constraint, with bisection confined to a split set of about six parameters and the rest enclosed at full range (H-06)" |
| §2.4, lever 6, surviving number (line 770) | "Exact rollout reuse is at most about 10% of compute (H-09)." | "Exact rollout reuse is at most about 10% of compute (H-09). Training avoided by inheritance is lever 2 and is counted there, once." |
| §2.5, "A faster step" (line 782) | "Engine throughput is a parity requirement (§36). The plan must match incumbents closely enough that the levers above are not cancelled; it does not look for the result there." | Append: "One engine-internal throughput question is open: structure-exploiting constraint assembly and solve (§16). Its ceiling on an incumbent's trace is 2.6×. It is measured at M2 and is not counted in the thesis until then." |
| §2.5, "Per-design kernel specialization" (line 784) | "Capped near 1.2×." | "Specialization of the recursions is capped near 1.2×." |
| §2.5, "Sampling control as the evaluator" (line 787) | "**Sampling control as the evaluator.** … It is a fallback and a source of demonstrations (H-01)." | "**Sampling control as a stand-in evaluator.** Even the most sample-efficient published controller costs about 4 × 10³ times a policy rollout per simulated second, so a battery of 256 twenty-second episodes takes about 86 minutes of a device against about 1.3 s for a policy. It is a screening rung, a fallback and a source of demonstrations. Its episodes are acceptance evidence only for itself, where it is the delivered controller (§22.8, H-01)." |
| §2.5, "Reusing rollouts across edits" (line 793) | "At most about 10% (H-09)." | "Exact reuse is at most about 10% (H-09)." |
| §2.6, sentence before the table (line 803) | "The ranges are what the ceilings of §2.3 and the surviving numbers of §2.4 allow." | Append: "They are stated against the incumbent as it is commonly run, with per-design training and a standard outer loop. The metric of §2.1 is against the strongest composed rival, which may host every portable lever; against that rival the expected ratio on fixed-topology CUDA missions is near one unless an engine-internal property binds, and no range is offered. §39 reports both, with the recovery ratio between them." |
| §2.6, table header, column 4 | "Expected ratio" | "Expected ratio against per-design training" |
| §2.6, row "Failure discovery and repair" (line 812) | "Failures driven by per-step disturbances only \| 3 to 100 in episodes to a planted failure; about 3 in repair turns" | "The proposal must cover the failure mode; certifying absence gains nothing \| 2 to 20 in episodes to a planted failure (published range), up to 100 as a TARGET; about 3 in repair turns" |
| §2.6, closing paragraph (line 815) | "On quadruped missions the realistic range tops out at ten, and reaching the top needs every lever to work at once." | "On quadruped missions the range against per-design training tops out at ten, and reaching the top needs every lever to work at once, including the two that are least measured (family policies and rank agreement). The denominator is an accepted decision and not a training run: a 1% failure bound at 95% confidence costs 299 failure-free episodes per accepted design in every arm." |
| §1.8 (line 635) | "a CPU referee can shadow on the order of 0.1–2% of GPU world-seconds, which is why referee work is sampled" | "a CPU referee at 0.2–1.0× real time per core can shadow on the order of 0.1–2% of GPU world-seconds, which is why referee work is sampled; the figure scales with referee speed and is measured at M2" |
| §1.6, N11 (line 551) | "Only as two measured numbers against Newton with Kamino or Steel on `trj`: distinct designs times steps per second, and time from an edit to the first GPU step." | Append: "A third candidate is a property, not a number: run-to-run replay and forks inside a mixed batch. It is claimed only after the incumbent's deterministic solvers have been run on a heterogeneous batch and found unable to do it." |

### b.3 §4 and §6, bets and non-goals (`P00.md`)

| Where | Now reads | Corrected wording |
|---|---|---|
| BET-14, falsifier (line 915) | "On a stratum, the composed-incumbent arm reaches the same accepted scope with a restricted mean time within a factor of 1.5 of ours." | Append: "Or the composed incumbent's recovery ratio (§39) is at least 0.8 on elapsed time and on compute." |
| BET-14, fallback | "Keep engine work only for the gaps internal to an engine: deterministic forks with mixed topologies, replay of one world outside its batch, certificates, the Metal tier." | "Keep engine work only for the gaps internal to an engine. Two are documented: a mixed-topology batch with run-to-run replay, and the Metal tier. Two are unverified and are checked before they are relied on: replay of one world outside its batch, and solver-internal certificates." |
| BET-16, falsifier (line 917) | "A parameter edit needs more than 20% of from-scratch steps; or after a chain of ten inherited generations the final return is more than 5% below a from-scratch control (loss of plasticity)." | "A parameter edit needs more than 20% of the steps of a from-scratch control stopped at 95% of its final return; or inheritance does not beat a raw-checkpoint warm start for the edit class; or after a chain of ten inherited generations the final return is more than 5% below a from-scratch control (loss of plasticity)." |
| BET-16, control and fallback | "Control: an arm with scheduled resets." … "Inheritance for parameter edits only, with scheduled resets" | "Control: an arm that resets on a measured regression of its audit twin, and an arm with a depth schedule." … "Inheritance for parameter edits only, with resets on a measured regression and a depth schedule where no audit twin exists" |
| BET-17, falsifier (line 918) | "Or top-k recall, measured by the audit lane, is below its declared floor." | "Or top-k recall, measured by the audit lane, is below its declared floor; or a cut sized from τ alone discards the higher rung's best design in the calibration set (the winner check of §23)." |
| BET-20, statement (line 921) | "Per-constraint certificates on sub-boxes of about six parameters or fewer, composed by monotonicity, symmetry and witnesses, resolve a useful share of a family." | "Per-constraint certificates with a split set of about six parameters, composed by monotonicity, symmetry and witnesses, resolve a useful share of a family." |
| BET-20, fallback | "Joint paving of a high-dimensional box is not attempted in any case (H-06)." | "Isotropic paving of a high-dimensional box is not attempted in any case (H-06)." |
| BET-27, fallback (line 928) | "Those designs route to training. The sampling controller is never the evaluator of record in any case." | "Those designs route to training. A sampling controller never stands in for a trained policy as acceptance evidence; its own episodes are accepted only under the delivered-controller lane of §22.8." |
| BET-32, statement and falsifier (line 933) | "Bounded-weight importance sampling gives valid bounds on failure rates with far fewer episodes than crude sampling, for failures driven by per-step disturbances (H-07). [S] \| … or the saving over crude sampling is below 3× at a rate of 10⁻⁴." | "Bounded-weight importance sampling gives valid bounds on failure rates. Once failures are found it refutes a failure-rate target, and tightens an estimate, with fewer episodes than crude sampling; it does not certify absence more cheaply (H-07). [S] \| On fixtures with a planted failure rate, the bound's coverage is below nominal; or the effective sample size falls below 30% on the reference family; or, on a fixture whose planted rate of 10⁻⁴ is above the target, the episodes needed for a lower confidence bound to exceed the target are not at least 3× fewer than crude sampling needs (published range 2–20×); or a planted second failure mode outside the proposal is reported as absent." |
| BET-34, falsifier (line 935) | "or sampled coverage is below 0.1% of GPU world-seconds" | "or sampled coverage is below 0.1% of GPU world-seconds, with coverage computed from the referee's measured world-seconds per second and no band assumed" |
| BET-36, statement and falsifier (line 937) | "A contact-level film-friction law and rigid proxies capture the failures that matter [S]" … "The film law misses its known answers; or advected films change fewer than 20% of kitchen rankings; or they add more than 25% to training time." | "A contact-level film-friction law and rigid proxies stand in for coupled matter [S]; whether they reproduce the failures of coupled models is the pilot of §19.10.7" … "The film law misses its known answers; or, in the pilot of §19.10.7, the proxies reproduce fewer than 90% of the coupled model's failures in the same class (the proxy claim is then withdrawn for that class and the island stays as an escalation); or advected films change fewer than 20% of kitchen rankings; or they add more than 25% to training time." |
| NONGOAL-11 (line 1039) | "Certified paving of a high-dimensional design box. \| New. Out of reach beyond about six parameters (H-06)." | "Certified isotropic paving of a high-dimensional design box. \| New. Out of reach beyond a split set of about six parameters (H-06)." |
| NONGOAL-13 (line 1041) | "Capped near 1.2× (H-04)." | "Recursion-only specialization is capped near 1.2× (H-04)." |
| NONGOAL-14 (line 1042) | "Sampling-based control as the evaluator of record." | "Sampling-based control as a stand-in evaluator for a trained policy. The delivered-controller lane of §22.8 is the one exception." |

### b.4 Subsystem chapters

| Hypothesis | Where | Now reads | Corrected wording |
|---|---|---|---|
| H-01 | `P06.md` §22.2, ADR-REFLEX-03 (line 41) | "It is never the evaluator of record." | "It never stands in for a trained policy as the evaluator of record. Where it is the delivered controller, its own episodes are admitted under a frozen planner record (§22.8)." |
| H-01 | `P06.md` §22.8, "Rules that follow" (lines 244–246) | "**A screening rung of at most three episodes per design.**" and "**Never the evaluator of record.** Episodes whose controller kind is `mpc-sample` are refused as the scoring source of acceptance, promotion or qualification (FROB-REF-817, INV-REFLEX-03)." | "**A screening rung of at most three episodes per design by default.** A paired screening comparison may use up to the break-even count of its class (30 for the quadruped setting and 60 for the humanoid at a 20 ms planning step; 6 and 12 at 4 ms), and stays labeled `screening: sampling-control`." and "**Never a stand-in evaluator.** Episodes whose controller kind is `mpc-sample` are refused as the scoring source of acceptance, promotion or qualification (FROB-REF-817, INV-REFLEX-03), unless the task declares the sampling controller as the delivered controller and the episodes meet the four conditions of the delivered-controller lane." Then insert Block B13. |
| H-01 | `P06.md` §22.8, verb line (line 265) | "`control.mpc` (run, with `--episodes` capped at three unless `--demonstrations` names a distillation job)" | "`control.mpc` (run, with `--episodes` capped at three unless `--demonstrations` names a distillation job, `--compare` names a paired screening comparison, or `--freeze` writes a planner record)" |
| H-01 | `P06.md` GATE-REFLEX-06 (line 804) | "FROB-REF-817 fires on a planted acceptance request" | "FROB-REF-817 fires on a planted acceptance request that has no frozen planner record" |
| H-01 | `P05.md` §21.20 (line 718) and ledger (line 1796); `P09.md` §28.9 (line 371) | "never the evaluator of record" / "It is never the evaluator of record: …" | "never a stand-in evaluator for a trained policy (§22.8)" / "It never stands in for a trained policy as the evaluator of record: …" |
| H-02, H-21 | `P01.md` §10 (lines 1075–1078) | "matches it after 1 to 20% of from-scratch steps" … "Inheritance costs 1 to 20% of from-scratch steps for parameter edits" | "matches it after about 2 to 25% of a from-scratch run stopped at matched return" … "Inheritance from a class prior costs about 2 to 25% of such a run; the one published parent-to-child figure is about 50%; no source measures a small typed parameter edit" |
| H-02 | `P06.md` §23.11, numbers table (line 1213) | "1 to 20% of from-scratch steps" | "About 2 to 25% of a from-scratch run stopped at matched return (1 to 10% against the fixed budget one source used; 20% in a second)". Add under the table: "Every figure in the first row is for flat or mildly varied ground with fixed actuation. No source measures a class prior on stairs, slopes or wet floors, with payload or with actuator-card changes; the held-out evaluation of GATE-CORTEX-11 therefore runs on the task family's interior terrain set." |
| H-02 | `P06.md` GATE-CORTEX-11 (line 1521) | "fine-tuning reaches specialist return in at most 20% of from-scratch steps" | "fine-tuning reaches specialist return in at most 20% of the steps of a from-scratch run stopped at 95% of its final return, on the task family's terrain set including stairs, slopes and wet patches" |
| H-21 | `P06.md` §23.2, ADR-CORTEX-06 (line 881) | "Inheritance is the default for parameter edits and an experiment for topology edits, with scheduled resets. \| Parameter edits cost 1 to 20% of from-scratch steps; …" | "Inheritance is the default for parameter edits once GATE-CORTEX-13 has passed for the edit class, and an experiment for topology edits; resets follow a measured regression, with a depth schedule as fallback. \| Transfers from a class prior cost about 2 to 25% of an early-stopped from-scratch run, and the one parent-to-child figure is about 50%; …" |
| H-21 | `P06.md` §23.12, table row (line 1251) | "The default. FACT (Appendix H): 1 to 20% of from-scratch steps" | "The default once its gate has passed for the edit class. FACT (Appendix H): about 2 to 25% of an early-stopped from-scratch run for transfer from a prior to a similar body; about 50% in the one parent-to-child study; no figure for a small typed edit" |
| H-21 | `P06.md` §23.12, "Scheduled resets" rule | "A lineage therefore resets on a schedule. A child at depth k in a chain (DESIGN default k = 5, OPEN-CORTEX-3) is trained from the class prior, or from scratch where there is none." | "A lineage therefore resets when its audit twin falls more than 5% below the fresh control, and in any case at depth k (DESIGN default k = 5, OPEN-CORTEX-3), when the child is trained from the class prior, or from scratch where there is none." |
| H-21 | `P06.md` GATE-CORTEX-13 (line 1523) | "median adaptation samples for parameter edits at most 20% of from-scratch" | "median adaptation samples for parameter edits at most 20% of a from-scratch run stopped at 95% of its final return, and fewer than a raw-checkpoint warm start needs" |
| H-02, H-21 | `P09.md` §28.9 (lines 376–380) and ledger (line 1967) | "fine-tuning needs 1 to 20% of from-scratch steps" … "an inherited policy needs 1 to 20% of from-scratch steps" | As the two `P01.md` replacements above |
| H-03 | `P03.md` §17.1 (lines 679–682) | "one incumbent publishes a mixed batch at 0.86 of its homogeneous rate (86 k samples/s)" … "The differentiators that remain are two measured numbers, throughput and the time from an edit to the first GPU step, and both can be lost (§17.14)." | "one incumbent solver publishes a mixed batch of three body plans at 86 k samples/s, 0.86 of the harmonic mean of its homogeneous rates; that solver is maximal-coordinate, is documented as not deterministic, and is reported as slower than the reduced-coordinate solvers of its framework" … "The differentiators that remain are two measured numbers, throughput and the time from an edit to the first GPU step, and one property, run-to-run replay and forks inside the mixed batch. All three can be lost (§17.14)." |
| H-03 | `P03.md` §17.14, "Measures" (after "(4) Digest invariance (GATE-PHYLUM-02).") | — | Add: "(5) Run-to-run digests of the mixed batch for arm C under each of its solver's deterministic modes that accepts a heterogeneous batch; if none does, that is recorded as the finding." |
| H-04 | `P03.md` §16.6, tree-operator bullet (lines 227–229) | "Ceiling: the factor is under a tenth of the step at 18 to 35 degrees of freedom, so the gain there is below 1.1; the route is justified only by padded buckets and by mechanisms above 60." | "Ceiling: HYPOTHESIS, the factor is under a tenth of our step at 18 to 35 degrees of freedom. An incumbent trace puts constraint assembly at 15% and the whole solve at 47% of its step without separating the factor, so the share is measured, not assumed: the ledger of the ancestor-sparse route below decides whether the route applies at 18 to 35 degrees of freedom or only to padded buckets and mechanisms above 60." Then insert Block B06. |
| H-04 | `P05.md` §21.2, ADR-SPARK-08 (line 66) | "The incumbent step trace caps the gain near 1.2× (§21.18)" | "The incumbent step trace caps recursion-only specialization near 1.2× (§21.18); sparsity in constraint assembly and solve is carried as data and is measured separately (§16)" |
| H-06 | `P07.md` §25.2, ADR-AUGUR-06 (lines 109–111) | "**ADR-AUGUR-06: region evidence is per constraint and low-dimensional.** Interval and Taylor-model certificates are issued for one constraint on a sub-box of about six parameters. Joint paving of a design box of ten to twenty dimensions is not attempted (§25.6)." | "**ADR-AUGUR-06: region evidence is per constraint, with a bounded split set.** Interval and Taylor-model certificates are issued for one constraint; bisection is confined to about six parameters and the others are enclosed at full range. Isotropic paving of a design box of ten to twenty dimensions is not attempted (§25.6)." |
| H-06 | `P07.md` §25.5, kinds table (line 637) | "One constraint holds, or fails, over a sub-box of at most six parameters" | "One constraint holds, or fails, over a box whose split set has at most six parameters" |
| H-06 | `P07.md` §25.6 (lines 743–744) | "Certificates for one constraint over a sub-box of the parameters that constraint depends on, with at most about six parameters, combined by products, intersections and monotonicity." | "Certificates for one constraint over a box of the parameters that constraint depends on, bisected along at most about six of them, combined by products, intersections and monotonicity." Then insert Block B22. |
| H-06 | `P01.md` §10 (lines 801 and 1408) | "Region certificates reach about six parameters per constraint (C-32)." / "Certificates are issued per constraint, on sub-boxes of about six parameters or fewer" | "Region certificates bisect about six parameters per constraint and enclose the rest (C-32)." / "Certificates are issued per constraint, with bisection confined to a split set of about six parameters" |
| H-07 | `P08.md` §26.9 (lines 1062–1063) | "HYPOTHESIS, in its surviving form: for failures driven by per-step disturbances with an informative score, exploration reaches failures of nominal probability 10⁻⁴ to 10⁻⁵ with 10 to 100 times fewer episodes than crude sampling." | "TARGET, in its surviving form: with an informative score, exploration reaches failures of nominal probability 10⁻⁴ to 10⁻⁵ with 10 to 100 times fewer episodes than crude sampling. FACT (Appendix H): the measured gains in the sources read are 2 to 20 times in one driving study and about 5 times in mean-square error on one policy-in-the-loop task. Splitting needs per-step disturbances; tilting and the defensive mixture also act on static scene parameters." |
| H-07 | `P09.md` §28.19 (lines 900–902) | "published adaptive stress testing reports 10 to 100 times fewer episodes at rates of 10⁻⁴ to 10⁻⁵ for failures driven by per-step disturbances with a good score (FACT, source register)" | "published adaptive stress testing reports 2 to 20 times fewer episodes (FACT, source register); 10 to 100 times at rates of 10⁻⁴ to 10⁻⁵ is a TARGET" |
| H-07 | `P01.md` §10 (lines 1215–1217) | "with one to two orders of magnitude fewer episodes when they are driven by per-step disturbances and the score is informative (FACT, source register, to be re-read)" | "with 2 to 20 times fewer episodes in the published record (FACT, source register, to be re-read) and one to two orders of magnitude as a TARGET, when the score is informative" |
| H-08 | `P08.md` §27.2, ADR-AUTOPSY-04 (lines 1310–1311) | "**ADR-AUTOPSY-04. Attribution runs in double precision.** Closed-loop legged control amplifies round-off enough to flip marginal outcomes in single precision (§27.9)." | "**ADR-AUTOPSY-04. Attribution reports a flip rate at round-off scale; double precision is the default for one failure.** Closed-loop legged control amplifies round-off enough to flip marginal outcomes in single precision over long windows (§27.9). Single precision on the originating tier is admitted by a measured rule (the single-precision lane)." |
| H-08 | `P08.md` §27.9, title and DESIGN paragraph (lines 1477, 1486, 1492–1493) | "### 27.9 Double precision for attribution [S]" … "DESIGN: attribution runs on `batch-cpu` in double precision, or on the referee." … "3. If the two agree, attribute in double precision. If they do not, label the failure `precision-sensitive`, refuse attribution (FROB-REF-584) and pass the world to the artifact guard of §26.6." | "### 27.9 Precision for attribution [S]" … "DESIGN: by default attribution runs on `batch-cpu` in double precision, or on the referee." … "3. If the two agree, attribute in double precision. If they do not, the failure goes to the single-precision lane with the numerical-variant interventions first; only if that lane does not admit it is the failure labeled `precision-sensitive`, attribution refused (FROB-REF-584) and the world passed to the artifact guard of §26.6." Then insert Block B23. |
| H-08 | `P08.md` label table (line 1403); refusal table (line 1656); GATE-AUTOPSY-03 (line 1701) | "The failure is not reproduced in double precision (§27.9)" / "Attribution requested for a failure that is not reproduced in double precision, or under an uncontrolled determinism class" / "ranks first in at least 90% of 200 failures in double precision" | "The failure is not reproduced in double precision and is not admitted to the single-precision lane (§27.9)" / "Attribution requested for a failure that neither lane admits, or under an uncontrolled determinism class" / "ranks first in at least 90% of 200 failures in double precision, and in the single-precision lane for windows of at most 1 s" |
| H-09 | `P09.md` §28.13, the ceiling paragraph (lines 648–652) | "Revalidation is an evidence rule and a hygiene measure. It is not a source of the ten-fold." | Append: "Training avoided by a warm start is not reuse in this sense; it is reported in its own column of the reuse ledger and credited to inheritance once." Then insert Block B25. |
| H-11 | `P03.md` §16.11 (lines 494–495 and 517–518) | "of order 0.1 to 2%. Referee speed dominates the figure." … "Ceiling: no speed benefit by construction; the benefit is fewer false accepts." | "of order 0.1 to 2% at that assumed referee speed. Referee speed dominates the figure and is unmeasured: a published CPU engine runs a humanoid at about 48 times real time per core, and a referee within a factor of ten of it would cover several percent." … "Ceiling: no compute benefit by construction; the benefit is fewer false accepts. Because sampled worlds are drawn with recorded inclusion probabilities, they also serve as the referee half of the paired tier correction (§16.8), which takes confirmation off the wall-clock critical path without reducing its cost." |
| H-11 | `P05.md` §36.3 row 17 (line 1326) and §36.9 (line 1497); `P09.md` §29 (line 1633) | "on the order of 0.1 to 2%" | "on the order of 0.1 to 2% at a referee speed of 0.2 to 1.0 times real time per core (HYPOTHESIS; measured at M2)" |
| H-12 | `P01.md` §10 (lines 1539–1540 and 1556) | "which is about 4% of robot mass or less" / "Ceiling: about 4% of robot mass." | "which is about 2 to 8% of robot mass on the one quadruped with a published mass breakdown" / "Ceiling: f_L·s ≤ 0.27 × 0.30 ≈ 8% of robot mass on that quadruped (§31.2)." |
| H-12 | `P10.md` §31.2 (line 699) and §31.3, ADR-FOUNDRY-01 (line 705) | "Foundry runs on finalists, after qualification (§24), never inside screening." / "Foundry runs after the design decision, on qualified finalists. An exploratory run is allowed, is labelled, and cannot enter a build packet." | "Foundry's topology rung, finite-element checks and build packet run on finalists, after qualification (§24). Its beam rung also runs inside screening, as a validity check on declared link walls." / "Foundry's optimization and build output run after the design decision, on qualified finalists. The beam rung also runs at preflight and after the first rollouts as a validity check that can block promotion. An exploratory run is allowed, is labelled, and cannot enter a build packet." Then insert Block B28. |
| H-12 | `P02.md` §14.11, analysis table (line 949) | "A-15 Structural margin \| Separate verb" | "A-15 Structural margin \| 6, beam rung under the bounded load set; full form a separate verb" |
| H-16 | `P04.md` §19.2, ADR-HAB-07 (line 75) | "contact-level film law and rigid proxies capture the failures at negligible cost." | "contact-level film law and rigid proxies stand in for coupled matter at negligible cost; whether they reproduce the failures of coupled models is decided by the pilot of §19.10.7." |
| H-19, H-18 | §39 (not yet written) | — | Insert Block B30 (three arms, the recovery ratio, the phase ablations). |
| H-22 | `P06.md` §23.11, consequences (line 1231) | "A lower 95% bound on τ of at least 0.7 near the top of the ranking lets the rung select finalists." | "A lower 95% bound on τ of at least 0.7 near the top of the ranking, together with the winner check below, lets the rung select finalists." Then insert Block B14. |
| H-22 | `P09.md` §28.10, mode table (lines 489–491) | `selective` at τ ≥ 0.5; `discard-only` at 0.2 ≤ τ < 0.5; `uninformative` below 0.2 | These thresholds contradict §23.11 and reconciliation decision R6 (section (d)). Corrected: `selective` at a lower bound on τ of at least 0.7 with the winner check passed; `discard-only` at 0.5 ≤ τ < 0.7, or unmeasured; `uninformative` (no discard) below 0.5. |
| New hypotheses | §0.8 rows H-25 to H-29 | — | Supplied by Blocks B20, B18, B04, B15, B21 and B10. |

## (c) Insertion blocks

Blocks are ordered by target chapter. The comment line above each block is for the lead and is not plan text.
Blocks B-numbers are local to this file. Where a block amends an existing subsection, its first sentence says
which, and section (b) gives the matching edits to the existing text.

| Block | Chapter | Title | Fills | Tier (S, F, M) | New gates | Kill gates (new unless noted) |
|---|---|---|---|---|---|---|
| B01 | 14 | Module capability tables and the composed quasi-static screen | pre-characterized sub-assembly port macromodels and composed screening | F | GATE-SKEL-801, GATE-SKEL-802 | KILL-SKEL-CELL |
| B02 | 16 | Row-class solve queues | K-019 | F | GATE-KIN-803 | KILL-ROWQUEUE |
| B03 | 16 | Decoupled stepping of separated robots in one world | K-022 | M | GATE-KIN-804 | KILL-INTERACT |
| B04 | 16 | Step adjoints through the converged contact solve | K-027 implicit-function contact and morphology adjoints (the M6 "gradients through contact" item) | M | GATE-KIN-805 | KILL-STEPADJOINT |
| B05 | 16 | Closed-form elimination of single-loop planar closures | symbolic kinematic-loop closure | F | GATE-KIN-806 | KILL-LOOPELIM |
| B06 | 16 | Ancestor-sparse assembly and the solve-stage ledger | adjudication H-04 (structure-exploiting constraint assembly and solve; the unlabelled ceiling of §16.6) | F | GATE-KIN-807 | KILL-ANCESTOR |
| B07 | 17 | Limb-archetype kernels | K-018 | F | GATE-PHYLUM-808 | KILL-LIMBKERNEL |
| B08 | 18 | Response batches: anchor, acceptance, known answers, gate | K-025 | F | GATE-TRAC-809 | KILL-RESPONSE (existing; second trigger) |
| B09 | 19 | Far-field field reuse across forks | K-044 causal tile recomputation, surviving narrow form "far-field fields only" | M | GATE-HAB-810 | KILL-FARFIELD |
| B10 | 20 | Visibility under self-occlusion, and sensor mounts as design variables | estimator-observability and self-occlusion sensor co-design | S, F | GATE-SENS-811, GATE-SENS-812 | KILL-VISIBILITY |
| B11 | 22 | A lifted linear planning model for one soft-soil regime | K-013 | M | GATE-REFLEX-813 | KILL-SOILLIFT |
| B12 | 22 | The pre-slip aliasing fixture and the sampled-data backup domain | §4.2 "Monitors cannot see a breach until it is too late"; §4.2 scorer addition "two-mode sampled-data shield fixture with a verified backup domain"; the control named in BET-25 | S | GATE-SHIELD-814 | — |
| B13 | 22 | The sampling controller as the delivered controller | adjudication H-01 (the sampling controller as the delivered controller; the categorical refusal of §22.8) | F | GATE-REFLEX-815 | KILL-MPCRECORD |
| B14 | 23 | The winner check for a selecting rung | adjudication H-22 (a high rank correlation does not protect the winner) | S | GATE-CORTEX-816 | — |
| B15 | 23 | Short-horizon gradient training with design parameters | assessment of the proposed hypothesis on contact adjoints with joint actor-critic training (learning arm) | M | GATE-CORTEX-817 | KILL-SHORTGRAD |
| B16 | 24 | Auditing a task before any design runs on it: foil bodies and intent invariants | attack the task specification before the design | F, S | GATE-ARENA-818 | KILL-FOIL |
| B17 | 24 | The item bank: importance sampling of battery items for close comparisons | calibrated item bank for the test battery, sampled by importance | F | GATE-ARENA-819, GATE-ARENA-820 | KILL-ITEMBANK |
| B18 | 24 | Exposure domains and the coupling test | common randomness compiled from the task; test of what "the same world" means | S, F | GATE-ARENA-821 | KILL-COUPLING |
| B19 | 25 | Monomial screens in exponent coordinates | K-070 surviving narrow form "geometric-programming monomial screens" | S | GATE-AUGUR-822 | — |
| B20 | 25 | Utility brackets and the dominance stop | bracket what the embodiment can achieve; dominance stop for controller effort | F | GATE-AUGUR-823 | KILL-AUGUR-BRACKET |
| B21 | 25 | Placement and sizing along a prescribed motion | inverse-dynamics convex synthesis via the linear dependence on inertial parameters | F | GATE-AUGUR-824 | KILL-SYNTH-REGRESSOR |
| B22 | 25 | Weak-variable enclosure and the split set | adjudication H-06 (coupled constraints can admit large boxes; the six-parameter cap is on the split set) | F | GATE-AUGUR-825 | KILL-WEAKVAR |
| B23 | 27 | Attribution in single precision on the originating tier | adjudication H-08 (the double-precision mandate; attribution on the originating tier) | F | GATE-AUTOPSY-826 | KILL-ENSEMBLE |
| B24 | 28 | Controller-budget curves and the controller-limited label | achievable-performance ranking from controller-budget curves | F | GATE-ATEL-827, GATE-ATEL-828 | KILL-BUDGETCURVE |
| B25 | 28 | The reuse ledger: three columns, never summed | adjudication H-09 (exact reuse, carried evidence and avoided reacquisition are different quantities) | S | GATE-ATEL-829 | — |
| B26 | 31 | Packet review as an imported record | K-096 build packet, the hidden-brief review gate preferred in the catalog | S | GATE-FOUNDRY-830 | — |
| B27 | 31 | Referee re-extraction of load cases that enter a build packet | referee-tier loads for the structural finalist lane | S | GATE-FOUNDRY-831 | — |
| B28 | 31 | Member sizing as a validity check in screening | adjudication H-12 (closed-form member sizing as a validity check inside screening) | S | GATE-FOUNDRY-832 | — |
| B29 | 32 | The offline replay packet | K-095 offline replay dossier; §4.2 "native paired-intervention harness before any browser bundle" | S | GATE-TETHER-833 | KILL-REPLAYPACK |
| B30 | 39 | Three arms, the recovery ratio and the phase ablations | adjudication H-19 and H-18 (the composed-incumbent recovery ratio; phase ablations) | S | GATE-TENFOLD-834 | — |

<!-- B01 · chapter 14 · tier F · fills: pre-characterized sub-assembly port macromodels and composed screening -->
<<<INSERT chapter=14 title="Module capability tables and the composed quasi-static screen">>>
**Capability tables [F].** This extends the substitution contract of §14.6. The contract there carries what the rest
of the robot may rely on as scalars and ranges. A capability table carries what a module can *do* at its ports over a
declared operating box, characterized once and cached by content, so that an assembly of modules can be screened by
composing tables before any whole-robot computation runs. It adds nothing to what a contract may claim about contact
dynamics: the three consequences listed for the limb port packet stand.

```rust
/// What a module can do at its ports over a declared operating box. DESIGN sketch: intent, not signatures.
pub struct CapabilityTable {
    module: ModuleHash, cards: Vec<CardVersion>, version: CharacterizationVersion,
    operating_box: OperatingBox,      // end-effector pose cells in the port frame, speed bound, duty bins, bus range
    wrench_outer: Vec<SupportRows>,   // per pose cell: h(u) ≥ max uᵀw over attainable w, outward-rounded
    wrench_inner: Vec<VertexSet>,     // per pose cell: attained wrenches; their hull lies inside the true set
    power: Vec<PowerBounds>,          // electrical power over (wrench cell, speed, duty)
    heat: Vec<HeatBounds>,            // steady heat to the sink over the same cells
    port_inertia: SymPsd6Range, first_mode: FrequencyInterval,
    validity: Validity,               // quasi-static; Froude number ≤ 0.5; bound on the neglected inertial wrench
}
```

*Characterization.* For each pose cell and each direction u of a fixed direction set, one cone program over the
module's own kinematics and cards (the program of A-24) gives the support value h(u) and an attaining wrench. Outer
rows are rounded outward and inner vertices are attained points, so the true attainable set lies between the two.
Power and heat rows come from the card laws of §15 at the attaining operating points. The table is a memo node
(§28.14) keyed by the module hash, card versions, operating box and characterization version; `set-param`, `regear`
or `swap-actuator` inside one module re-characterizes that module only. Verb: `module.characterize` (run).

*The composed screen.* For an assembly, a stance set σ (the modules bearing load, taken from the gait atlas or the
task) and a quasi-static requirement with external wrench w_ext at the base,

    find  w_i ∈ W_i  (i ∈ σ)   with   Σ_{i ∈ σ} Ad*(g_i)·w_i + w_ext = 0.

- With the outer sets, each enlarged by the validity record's bound on the neglected inertial wrench, infeasibility
  excludes the assembly for that requirement and stance, with an `lp-farkas` certificate. The result is typed `outer`
  and names its load-sharing model (rule 1 of §28.4).
- With the inner sets, each shrunk by the same bound, feasibility is a typed `inner` witness.
- Otherwise the assembly is `unresolved` and goes to the next rung.
- The electrical bus is checked by the joint time-indexed schedule of A-23, never by a sum of peaks, and heat by the
  sink balance of the contract.

The screen runs at stage 6 of `design.check` and at stages 1 and 2 of the cascade (§28.11). Its margins (payload
margin, estimated energy per metre) are a screening signal under §28.10: they discard only in strata where rank
agreement has been measured.

*Known answers.* Four identical one-joint legs, each a horizontal link of 0.25 m with a torque limit of 10 N·m, can
each press with at most 40 N. Under a 16 kg body (156.96 N) on a square stance with feet at (±0.2, ±0.2) m:

- centre of mass centred: 39.24 N per foot, admitted;
- centre of mass 0.05 m forward: the front pair must carry 0.625 × 156.96 = 98.1 N > 2 × 40 N, excluded for every
  force distribution. The certificate is the vertical force balance plus the pitch moment balance.

*Counterexample: a table is valid for its stance set only.* The centred body above is admitted with four feet down.
In a trot two diagonal feet carry the weight, 78.48 N each against 40 N. A screen that reuses the four-foot result
for a trotting requirement admits a robot that cannot trot. The stance set is part of the query and of the result.

*What the screen may not conclude.* An `outer` exclusion is an exclusion of the quasi-static requirement in the
declared stance. A dynamic strategy that uses momentum is outside it, as the approach-energy case of §28.4 shows.
Above the validity regime the screen returns `unresolved`, never a verdict.

*Cost and ceiling.* TARGET, unmeasured: characterization of one module in seconds on the CPU (64 pose cells × 26
directions = 1,664 small programs); a composed screen under 1 ms, so on the order of 10³ assemblies per core-second.
Ceiling, stated first: the screen decides only what static composition can decide. It removes whole-robot work for
the candidates it excludes or admits and none for the rest, and limbs coupled through a light moving base are not
isolated at their ports, which is why no dynamic verdict is composed.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-SKEL-801 | Known answer | The two stance cases and the trot counterexample exactly; inner ⊆ outer on 10⁴ random directions per archetype module; an edit inside one module re-characterizes one table and no other | M3 |
| GATE-SKEL-802 | Performance of the method | On 200 assemblies across the wheeled, tracked, legged and hybrid families, on carrying, ramp and threshold tasks inside the validity regime: discarding the bottom 90% by the composed screen keeps at least 9 of the true top 10 found by batched evaluation; Spearman ρ at least 0.75 on energy per metre and on payload capacity; every audited `outer` exclusion fails its quasi-static requirement on the referee | M4 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-SKEL-CELL | Top-10 recall below 90% or ρ below 0.75 at GATE-SKEL-802: no automatic discard from the screen. One unsound `outer` exclusion: the exclusion rule is suspended under the prune-soundness audit of §28.12 | Tables remain as a sizing report for agents; `design.check` runs its whole-robot analyses as before |
<<<END>>>

<!-- B02 · chapter 16 · tier F · fills: K-019 -->
<<<INSERT chapter=16 title="Row-class solve queues">>>
**What it is.** [F]. This subsection extends §16.6 and adds an arm to the head-to-head of §17.14. The lockstep
solve of §16.6 launches stages 7 and 8 over every world at the row capacity R for a fixed number of Newton
iterations, under masks. Padding by morphology is bounded by the buckets of §17.5. Padding by *work* is not:
inside one model the active row count changes within a stride, and worlds converge after different numbers of
iterations. A row-class solve queue regroups worlds by the work they need, so that one execution group (32 lanes
on CUDA) holds worlds with similar active row counts that are still iterating, whichever robot they belong to.
Owner: `frob-kernels` for the kernels, `frob-dyn` for the step plan. Exposure: the option `--solve-queues` of
`sim.run` and `phylum.sweep`; `phylum.plan` prints the bound below for the population in hand; the placement
receipt of §21.12 records whether queues ran.

**Mechanism (DESIGN).**

1. *Classes.* After stage 5 each world has an active contact count c_w ≤ C. Three row classes are fixed
   fractions of capacity: c_w ≤ C/4, c_w ≤ C/2 and c_w ≤ C. Class k has a queue of world indices with a fixed
   capacity Q_k, a constant of the shape class, so no buffer changes shape.
2. *Assignment.* Kernel `K-queue-assign` counts worlds per class, takes an exclusive scan and writes world
   indices in ascending world order: count, scan, write, with no floating-point atomics. A world that does not
   fit its class queue goes to the next larger class, and from the largest to the general masked lane, which
   holds every world and therefore always fits.
3. *Execution.* The kernels of stages 7 and 8 are launched once per class with that class's loop bound, and
   they read and write each world's rows in place through the index list. No state moves.
4. *Regrouping live worlds.* After every second Newton iteration (DESIGN default) the assignment is repeated on
   the `done` masks, so later iterations run over the live prefix of each queue. The number of launches stays a
   constant of the step plan; a launch over an empty prefix returns at once.
5. *Generations.* Queue capacities change only at a control-interval boundary, as a new generation of the step
   plan's execution options.

**Why no digest can change.** A world's arithmetic reads its own rows only (INV-SPARK-04). The loop bound of its
class is at least its active count, and inactive rows are identity rows that follow every active row (§35.5).
The per-world `done` test is unchanged. Reductions inside a world are anchored trees whose shape does not depend
on a bound. Queue membership, order inside a queue and co-tenants therefore enter no result: this is INV-KIN-04
applied to work classes. Until GATE-KIN-803 has passed on a backend, a run with queues carries the label
`experimental-execution` and is refused as promotion evidence (refusal: execution option not yet qualified on
this backend).

**Counterexample: classing once per episode.** A trotting quadruped with four samples per foot has 8 active
contacts in diagonal support and 16 in double support (illustrative). With C = 24 the class bounds are 6, 12 and
24, so the world crosses a class boundary twice per stride. A sort made at reset leaves it in the wrong class
for part of every stride: either the loop bound is below its active count, which drops contacts and is forbidden
by §18.11, or it sits in the largest class and nothing is saved. Assignment is therefore per step.

**Ceiling, stated before any speed test.** Let f be the share of step time in the solve, ℓ the share of the
lane-time launched there that does arithmetic on an active row of a world still iterating, and h the added
relative overhead (assignment, indirection, extra launches). Under the serial model of §17.6,

  speedup ≤ 1 / (1 − f + f·ℓ + h).

- FACT (Appendix H): the solve is 47% of an incumbent's humanoid step. With every idle lane removed and no
  overhead the bound is 1/(1 − 0.47) = 1.89. The mechanism cannot double the step.
- Idle lanes are the rule in a lockstep solve. If a fraction ℓ of worlds is live and liveness is independent
  across the 32 lanes of a group, the group is busy with probability 1 − (1 − ℓ)³². At ℓ = 1/4 that is
  1 − 1.0 × 10⁻⁴: every group stays busy while three quarters of its lanes wait. After regrouping, 8,192 worlds
  at ℓ = 1/4 fill 64 groups instead of 256.
- HYPOTHESIS, from the launch figures of §17.5: the solve accounts for about 30 of a step plan's 60 launches at
  5 to 10 µs each, so three classes in place of one add 60 launches, 0.3 to 0.6 ms per step. At 0.5 µs per
  world-step that is h = 0.07 to 0.15 at 8,192 worlds and 0.009 to 0.018 at 65,536. The test is the measured
  launch cost in the fingerprint (§21.16).

| Worlds | ℓ | h | Bound at f = 0.47 |
|---|---|---|---|
| 8,192 | 0.50 | 0.07 to 0.15 | 1.20 to 1.09 |
| 8,192 | 0.25 | 0.07 to 0.15 | 1.39 to 1.25 |
| 65,536 | 0.50 | 0.009 to 0.018 | 1.29 to 1.28 |
| 65,536 | 0.25 | 0.009 to 0.018 | 1.52 to 1.50 |

The mechanism is a modest optimization that pays at large batches and low live fractions, and it loses at 8,192
worlds if launches cost what the hypothesis says. On Metal every dispatch of a chunk is encoded serially into
one command buffer (§21.9), and the cost of an added dispatch there is unmeasured; admission is per backend.

**Relation to family lanes.** The lanes of §17.6 group worlds by tree and contact mode in order to share
constants. Queues group by amount of work across unrelated robots. Both remove masked work from the same
stages, so their gains do not multiply. The profile measures each alone and both together, and the simpler
configuration that reaches the larger measured gain is kept.

**Pilot before funding.** No new kernel. On the M2 profile of the masked path, counters record ℓ per Newton
iteration index and the active-contact histogram per reference class (`telemetry.histogram`), and the
fingerprint supplies the launch cost. Falsifier: a predicted bound below 1.15 at every batch size the campaigns
of §28 use.

**Known answers and controls.** (1) A world's digests with queues on equal its digests on the masked path, bit
for bit. (2) The trot fixture: per-step assignment follows the 8 and 16 alternation, and the reset-time sort is
caught, by a truncation flag or by zero saving. (3) A planted kernel that sizes a reduction tree by the class
bound, in place of anchoring it, changes a digest and is caught.

**Gate and kill gate.**

| Gate | Claim | Threshold (TARGET) | Earliest |
|---|---|---|---|
| GATE-KIN-803 | Performance; known answer (determinism) | Digests identical to the masked path on 1,000 episodes per backend under 100 random co-tenancies and queue capacities; overflow to the general lane at most 1% of world-steps on the standard battery; a mixed cohort with queues reaches at least 0.70 of its weighted homogeneous throughput; the 95% lower bound of the paired whole-step ratio against the masked path is at least 1.15 at the batch size named in the receipt. | M2 on CUDA; Metal when its dispatch cost is measured |

✖ KILL-ROWQUEUE. Trigger: the pilot's falsifier holds; or the measured lower bound is below 1.15; or
h ≥ f·(1 − ℓ) on the measured profile; or a digest mismatch survives one repair cycle. Fallback: the masked
lockstep solve of §16.6 alone, with the ℓ counters kept in the disclosure block of §36.7.

In the head-to-head of §17.14 this is arm E, arm A with queues on, reported on measure (2). The pre-declared
outcomes of that experiment are unchanged.
<<<END>>>

<!-- B03 · chapter 16 · tier M · fills: K-022 -->
<<<INSERT chapter=16 title="Decoupled stepping of separated robots in one world">>>
**What it is.** [M]. Per-world time tiles between interactions are rejected as a general mechanism (§21.20):
separate environments are already independent, and policy and sensor frames bound a tile to 5 to 20 steps. With
a fraction p of step time inside tiles of L steps and overhead o the bound is 1/(1 − p + p/L + o); ten needs
p ≥ 0.924 at L = 64 and o = 0.01, and is out of reach for any p once L ≤ 10, where the bound at p = 1 is 9.1.
One narrow case remains, for the multi-robot scenes of §25.17: several robots in one world that are far apart
for most of a shift. While a certificate shows that two robots cannot touch, exchange a message or sense each
other before the next boundary, each is stepped as its own single-robot world in its own cohort and shape
class, and the multi-robot world exists as a ledger over them. Owner: `frob-dyn` for the tile planner, with the
swept bounds of §18.4 from `frob-contact` and the clock contract of §20. Exposure: `sim.run` on a multi-robot
scene with `--decouple`; the tile ledger is a table of `telemetry.summary`.

**Mechanism (DESIGN).**

1. *Islands.* The bodies of the world are partitioned into islands: a robot with everything it carries or
   touches; each free prop on its own; the static scene in none.
2. *Tile boundary.* All islands of a world are synchronized at every tile boundary. A tile ends at the earliest
   of: (a) the next control tick of any robot in the world; (b) the next exposure or sample instant of an
   exteroceptive sensor whose range and field of view could contain another island; (c) the earliest arrival
   of a message on a declared channel between islands, which is its send instant plus the channel's minimum
   latency; (d) the earliest possible contact between two islands. Clause (a) is the fence that removed the
   general mechanism, and nothing here exceeds it.
3. *Separation bound.* For islands a and b with bounding radii r_a, r_b about reference points at distance d,
   current speeds v_a, v_b of those points, and caps ā_a, ā_b on their accelerations (from actuator cards and
   friction limits; a free prop takes g), the gap after time t is at least

     γ(t) = d − r_a − r_b − (v_a + v_b)·t − ½·(ā_a + ā_b)·t².

   A tile of length T is certified for the pair when γ(T) > 0. Clipping speeds at the cards' speed caps can
   only enlarge γ and is allowed.
4. *Decoupled stepping.* Inside a certified tile each island steps in a slot of a single-robot cohort that
   shares the world's scene slot. Its random streams keep the multi-robot world's address, with the island as
   the perturbed entity (§35.9), so a robot's draws do not depend on whether it was stepped coupled or apart.
5. *Merge.* Islands whose pair is not certified for the next tile are stepped together for that tile as one
   coupled group, under the `strict` profile with an island-wise solve, or in a multi-robot shape class where
   one is admitted. The switch happens only at a tile boundary and is recorded with the pair, the gap and the
   clause that ended certification.

**Smallest case, by hand.** Two robots are at rest with a 0.10 m gap between their bounding spheres, and each
can accelerate at 5 m/s². The gap closes when 2·(½·5·t²) = 0.10 m, at t = √0.02 s = 0.141 s. A 20 ms tile is
certified, with γ = 0.10 − 0.002 = 0.098 m. A 0.2 s tile is not: γ = 0.10 − 0.20 < 0. *Counterexample:* a bound
built from current velocities alone sees a closing speed of zero and certifies any tile length, and the robots
then collide at 0.141 s inside a tile, unseen. The acceleration term is not optional. Second case: head-on at
speed caps of 1.5 m/s each from a 3.0 m gap, the earliest contact is at 1.0 s, so fifty 20 ms tiles are
certified and the pair merges when the gap falls below 3.0 × 0.02 = 0.06 m.

**Ceiling, stated before any speed test.** For N similar robots let φ be the share of a single-robot step spent
in the dense factor. A coupled world factored densely costs (1 − φ) + φ·N² per robot-step, in units of a
single-robot step, and the decoupled worlds cost 1 + o. At φ = 0.1, the upper end at 18 to 35 degrees of
freedom (§16.6), and o = 0.02 the ratio is 2.45 for N = 4 and 7.16 for N = 8. That is the ratio against the
weakest comparator. A comparator that factors island by island already removes the N² term, and against it the
saving is the cross-island broad phase alone. FACT (Appendix H): collision is about 4% of an incumbent's
humanoid step, so that ratio is below 1.05. The mechanism is therefore not a throughput lever. What it gives is
admission: a fleet scene runs on the single-robot shape classes, kernels and launch graphs that already exist,
each robot in its own size bucket, with no multi-robot bucket padded to N times the largest robot. Its value is
bounded by the share of a shift the robots spend certified apart.

**Pilot before funding.** Paper only. On the warehouse archetype of §19.7, with 4, 8 and 16 kinematic
route-graph actors standing in for robots, the share of robot-time certified at 20 ms tiles is computed from
traces. Falsifier: a share below 0.5 at 8 robots.

**Known answers and controls.** The two hand cases; a planted message whose latency is shorter than the tile,
which must end the tile at its arrival; a lidar that can see the other robot, which must end the tile at its
sample instant; a thrown prop, whose island must merge before impact.

**Gate and kill gate.**

| Gate | Claim | Threshold (TARGET) | Earliest |
|---|---|---|---|
| GATE-KIN-804 | Known answer | On the two-carrier handoff fixture of §25.17 and a 16-robot sparse warehouse: zero contacts, message arrivals or sensor returns from another island inside a certified tile over 10⁶ robot-steps; every robot's digests equal those of the reference that steps the same islands in one world with island-wise solves on `batch-cpu`; the four controls behave as stated; the certified share is reported. | M8, with §25.17 |

✖ KILL-INTERACT. Trigger: one missed interaction survives a repair cycle; or the certified share is below 0.5
of robot-time on the warehouse fixture. Fallback: a multi-robot world is one coupled world under `strict` with
island-wise solves and fixed decimation, and a fleet scene beyond its capacity is refused under §18.11.
<<<END>>>

<!-- B04 · chapter 16 · tier M · fills: K-027 implicit-function contact and morphology adjoints (the M6 "gradients through contact" item) -->
<<<INSERT chapter=16 title="Step adjoints through the converged contact solve">>>
This subsection specifies the mechanism behind the M6 item "gradients through contact" and the revisit clause of
NONGOAL-15. Tier [M]. It is off the critical path and is funded only after its pilot. Owner: `frob-dyn` for the
reverse sweep, with derivative kernels produced by `derive/frob-derive` into `frob-kernels`. Verb:
`phylum.gradient --method adjoint`; the paired finite-difference method of §17.13 stays the reference.

**What it is.** On lockstep tiers stage 8 returns the velocity v⁺ that minimizes a strictly convex objective Φ(v; z),
where z collects the state at the start of the step, the action and the design parameters of the parameter row
(§16.6). At convergence the residual r(v, z) = ∇ᵥΦ is zero and the Hessian H = A + JᵀGJ has already been factored.
The implicit function theorem gives dv⁺/dz = −H⁻¹ · ∂r/∂z. In reverse mode, for the cotangent v̄ of a scalar loss
with respect to v⁺:

  solve H · λ = v̄   (two triangular solves with the factor the step already holds),   then   z̄ ← z̄ − (∂r/∂z)ᵀ · λ.

Nothing is differentiated through Newton iterations or the line search. The two solves cost about n_v² multiply-adds:
324 for the reference quadruped (n_v = 18) and 1,225 for the humanoid (n_v = 35). The products (∂r/∂z)ᵀ·λ and the
transposed Jacobians of stages 1, 6, 7 and 9 are straight-line reverse programs derived at build time from the kernel
IR that produces the forward kernels, each with an instance certificate that it equals the symbolic derivative (§12,
§21).

**The sweep.** A horizon of K control intervals with K ≤ 32 (DESIGN; §17.13 caps paired differences for the same
reason). Step buffers are stored for every world-step of the horizon, so no recomputation schedule is needed. FACT
(arithmetic on §16.6): at 27 kB per quadruped world, K = 16 and 10 world-steps per interval, the store is
160 · 27 kB = 4.32 MB per world and 8.85 GB for 2,048 worlds. `sim.estimate` prints the figure and the job is refused
when it does not fit. HYPOTHESIS: the reverse sweep costs about one forward sweep; the pilot measures it.

**Labels.** Each step's derivative carries a label of C-16. `smooth`: every contact and joint row is strictly inside
one region of its law (open, stick, slip; limit inactive or active), with normalized margin above a guard.
`fixed-mode`: the derivative is taken with active sets frozen. `one-sided`: a row sits on a boundary; both one-sided
values are returned on request. A step whose solve did not converge has no derivative, because the premise r = 0
fails, and its world is dropped from the batch gradient and counted. The label of a K-step gradient is the weakest
label on its path, reported with the number of steps under each label.

**Guards.**

- *Agreement.* In every batch, 8 random unit directions in design space are also evaluated by paired central
  differences (§17.13) on 64 seeds, and the cosine between the two sets of directional derivatives is recorded. Below
  0.65 the horizon is halved and the job repeats. Below 0.5 at K = 8 the verb returns the finite-difference result
  with the label `adjoint-rejected`.
- *Metrics.* Smooth metrics only; a binary outcome is refused as in §17.13.
- *Determinism.* Class D2 or better, since the agreement check pairs worlds (§35).
- *Manifold.* A gradient with respect to an inertia is a tangent vector of the cone in the chart of §14, and a step
  along it uses the retraction. Under a symmetry of the closed loop the gradient is averaged over the
  observation-preserving subgroup (§13), so a symmetric design stays symmetric.

**Known answers.**

- *Scalar implicit function.* Φ(v; z) = ½·m·(v − z)² + ½·g_c·v², a free term plus a contact term of curvature g_c.
  Then r = m·(v − z) + g_c·v, H = m + g_c and ∂r/∂z = −m, so v⁺ = m·z/(m + g_c) and dv⁺/dz = m/(m + g_c). With
  m = 2 and g_c = 6 the derivative is 1/4; with the contact open it is 1. At the boundary the one-sided values are 1
  and 1/4 and the label must be `one-sided`.
- *A zero gradient is not a stationary point.* A 1 kg block rests on a floor with μ = 0.5 under ideal Coulomb
  friction and is pushed with force u for one step of 0.01 s. Breakaway is at μ·m·g = 4.905 N. At u = 4 N, v⁺ = 0 and
  dv⁺/du = 0. At u = 6 N, v⁺ = 0.01 · (6 − 4.905) = 0.01095 m/s and dv⁺/du = 0.01 s/kg. For the objective "reach
  0.1 m/s" the adjoint at u = 4 N is exactly zero although raising u past 4.905 N makes progress. Under a regularized
  law whose friction ramps linearly up to a stiction speed of 1 mm/s, the stick derivative is 0.01/(1 + 49.05) ≈
  2.0 × 10⁻⁴ s/kg, 50 times smaller than in slip, and its value is set by the stiction speed, which is a numerical
  parameter (§18.5). A `fixed-mode` gradient in stick is therefore reported as uninformative, and a consumer that
  receives one takes a derivative-free step.

**Consumers.** `opt.start` with gradient steps on the design manifold, the refinement lane of §25.21.2 and the
sensitivity analysis A-16. A first-order policy learner over short horizons is not part of this item; it would be a
separate experiment under §23 with its own pilot.

**Pilot before funding.** Paper-scale and outside the workspace, on an incumbent engine that already ships
derivatives: one quadruped, 30 design parameters, a smooth tracking-and-energy objective over 16 control intervals,
256 worlds, 20 replicate batches. Arms at equal wall-clock: adjoint ascent, the stencil of §17.13, and one
derivative-free generation. Falsifier: median cosine against central differences below 0.5, or adjoint ascent needs
more than a quarter of the derivative-free arm's wall-clock to reach the same objective value. Cost: about three
engineer-days and 6 GPU-hours (TARGET).

**Ceiling, stated before any speed test.** One gradient costs about two sweeps against 2P + 1 = 61 for the stencil at
P = 30: at most about 30 times fewer world-steps per gradient, on smooth objectives over horizons where most steps are
`smooth`, and nothing where the objective is decided by a binary outcome or by a change of contact sequence. FACT
(Appendix H): several incumbents already differentiate their step, so this is not a differentiator.

**Gate.** GATE-KIN-805 (Known answer and Performance of the method, M6). The scalar and block fixtures are exact,
with their labels. On a 100-case suite of short horizons, 80 with no change of active set and 20 controls with a
planted change: cosine against central differences at least 0.95 in at least 72 of the 80, and all 20 controls
labeled `one-sided` or `fixed-mode`, or refused. The reverse sweep takes at most twice the forward sweep's wall time
on the reference quadruped at 2,048 worlds.

**Kill gate.** ✖ KILL-STEPADJOINT. Trigger: the pilot's falsifier fires; or GATE-KIN-805 fails after one repair
cycle; or the median cosine on the design corpus is below 0.5 at K = 8. Fallback: the paired stencil of §17.13 where
its own gate holds, otherwise rank-based search; the response batches of §18.5 keep supplying adjoint columns at a
single state.
<<<END>>>

<!-- B05 · chapter 16 · tier F · fills: symbolic kinematic-loop closure -->
<<<INSERT chapter=16 title="Closed-form elimination of single-loop planar closures">>>
**Eliminated closures [F].** This extends the kinematic-loops rule of §16.12 and the linkage transmissions of §15.9.
On the lockstep tiers a closure is a compliant equality row, and exact closures on the referee wait on an upstream
constraint lane. For the loops that robots most often contain (a parallel knee, a steering linkage, a parallelogram
that keeps a foot level) neither is needed: a planar four-bar has one degree of freedom and a closed-form position
solution, so the loop can be removed from the constraint system when the model is compiled.

*Recognition.* At compile time `frob-model` marks a `closure` as eliminable when it closes exactly one independent
loop of four revolute joints with parallel axes, the mobility analysis A-01 reports one degree of freedom for the
loop, and no other closure shares a link with it. Everything else stays a row.

*The closed form.* With ground link d, input crank a at angle θ, coupler b and output link c at angle φ, both angles
measured from the ground link, the loop equation |O₂ + c·(cos φ, sin φ) − a·(cos θ, sin θ)| = b has the two solutions

    φ = atan2(k_y, k_x) ± arccos( (b² − c² − R²) / (2·c·R) ),    (k_x, k_y) = (d − a·cos θ, −a·sin θ),  R = ‖k‖.

The two signs are the two assembly branches. The branch is fixed by the design's reference pose and recorded in the
model; a compile that cannot tell the branches apart at the reference pose refuses. `frob-kir` emits kernels for
φ(θ), the transmission ratio n(θ) = dφ/dθ and n′(θ), and the same for the coupler angle. The dependent joints of the
spanning tree then follow θ: q̇_dep = n(θ)·θ̇ and q̈_dep = n(θ)·θ̈ + n′(θ)·θ̇², and the step uses the reduced
coordinates through the embedding Γ = ∂q/∂θ, with mass matrix ΓᵀMΓ and bias Γᵀ(M·Γ̇·θ̇ + h). No closure row, no
compliance and no drift remain for that loop on any tier, and the transformer modulus of §15.9 is n(θ) exactly.

*Refusal near the toggle.* Where coupler and output link line up, the argument of the arccosine reaches ±1, the
branches meet and n(θ) is unbounded. Elimination is refused when the sine of the transmission angle falls below 0.17
(about 10°, DESIGN) anywhere in the declared joint range, and the loop stays a row with that reason in the report.

*Known answers.*

- Parallelogram, a = c = 1 and b = d = 2: on the open branch φ = θ and n = 1 for every θ.
- The same linkage at θ = 90° on the crossed branch: the coupler end is at (1.2, −0.6) and φ = −143.13°. A solver
  that returns this root for a knee assembled open has folded the leg the other way.
- A crank-rocker with (a, b, c, d) = (1, 3, 2.5, 3) at θ = 90°: φ = 98.857° and n = 0.3722 on the open branch;
  φ = −135.727° and n = −0.1722 on the crossed one.
- The parallelogram at θ = 0 is a change point: all four links are collinear and elimination is refused.

*Owners and verb.* `frob-model` owns recognition and the branch record; `frob-kir` the emitted kernels; `frob-dyn` the
reduced embedding. `design.check` reports each closure as `eliminated` or `row` with the reason.

*Ceiling and pilot.* Ceiling: one constraint block fewer per such loop, and no closure error; no throughput claim is
made, and a spatial or multi-loop mechanism gains nothing. Pilot before funding: the parallel-knee archetype on
`batch-cpu` both ways, comparing knee torque and foot trajectory over a 2 s stride against the referee's bilateral
rows.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-KIN-806 | Known answer; Parity | The four known answers to 10⁻¹⁰ rad in double precision; reduced-coordinate forward dynamics within 10⁻⁹ relative of the bilateral-row solution on `referee` over 2 s; on the lockstep tiers in single precision the loop residual stays below 10⁻⁶ m for 10⁶ steps; the crossed-branch mutant and a loop driven through its toggle are both refused | M2 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-LOOPELIM | The parity part of GATE-KIN-806 fails on any archetype loop after one repair cycle; or the recognizer matches fewer than half of the closures in the archetype library | Compliant equality rows on the lockstep tiers and bilateral rows under the transactional profiles, as in §16.12 |
<<<END>>>

<!-- B06 · chapter 16 · tier F · fills: adjudication H-04 (structure-exploiting constraint assembly and solve; the unlabelled ceiling of §16.6) -->
<<<INSERT chapter=16 title="Ancestor-sparse assembly and the solve-stage ledger">>>
**Ancestor-sparse assembly and the solve-stage ledger [F].** This extends the tree-operator route of §16.6. The
pilot named there counts factor operations only, and the ceiling stated there ("the factor is under a tenth of the
step at 18 to 35 degrees of freedom") is a HYPOTHESIS with no measurement behind it. On an incumbent's step trace
the two stages this route touches, constraint assembly and the solve, take 38.31 and 123.55 of 262.33 ns per
world-step, which is 61.7% of the step (FACT, Appendix H). If both vanished the step would be 2.61 times faster; a
three-fold reduction of both gives 1/(0.383 + 0.617/3) = 1.70. Those are ceilings on that incumbent's step and say
nothing yet about ours. They are large enough that the question is settled by a ledger and a pilot, not by an
assumption, and small enough that the answer cannot be a source of the ten-fold (§2.5).

*What is exploited.* For a contact or limit row r attached to one link, the Jacobian row is nonzero only on that
link's ancestor coordinates. Let n be the number of coordinates, d_k the depth of coordinate k (the number of its
ancestors, itself included) and d_r the depth of the link that row r is attached to. Three operations inside every
Newton iteration then shrink.

| Operation, per iteration | Dense count (multiply-adds) | Ancestor-sparse count |
|---|---|---|
| One row's contribution to JᵀGJ | n·(n + 1)/2 | d_r·(d_r + 1)/2 |
| Factor of H = A + JᵀGJ, ordered leaves first, no fill | (n³ − n)/6 | Σ_k d_k·(d_k − 1)/2 |
| Two triangular solves | n² | Σ_k (2·d_k − 1) |
| One row's residual and line-search evaluation | n | d_r |

*Known answers, checkable by hand.*

- Quadruped, n = 18: a six-coordinate base and four three-coordinate legs, so the depths are 1 to 6 once and 7, 8, 9
  four times. Factor: 35 + 4·(21 + 28 + 36) = 375, against (18³ − 18)/6 = 969: a ratio of 2.58. A foot row has
  d_r = 9: 45 against 171, a ratio of 3.8.
- Humanoid, n = 35: a six-coordinate base, two six-coordinate legs (depths 7 to 12), a three-coordinate waist (7 to
  9) and two seven-coordinate arms beyond the waist (10 to 16). Factor: 35 + 2·251 + 85 + 2·560 = 1,742, against
  (35³ − 35)/6 = 7,140: a ratio of 4.10. A foot row has d_r = 12: 78 against 630, a ratio of 8.1.
- A single chain (d_k = k) gives a ratio of exactly 1 in every row of the table. There is nothing to exploit, and
  the route reports that instead of running.

*The counterexample that bounds the route.* A row between two limbs (a hand on a thigh, one foot on the other) is
nonzero on both ancestor chains, and its product couples the two limbs: the leaves-first order then has fill. The
smallest case is a one-coordinate base with two one-coordinate limbs (n = 3). Rows on single limbs give H the arrow
pattern with a zero at the (limb 1, limb 2) entry; one row touching both limbs fills that entry, and a factor that
skips it is wrong. Such a world takes the dense path for that step under a flag, as §16.6 already requires, and the
share of world-steps that do so is a reported number.

*Fewer operations are not a faster step.* On a device the step is bound by memory traffic and launch structure as
much as by arithmetic (§36). The ancestor lists are data in the model table (§17.4) and are fixed per shape class,
so every lane of a bucket still runs one static launch graph; the price is gathers through an index and shorter,
irregular inner loops. Nothing here needs a compile per design. The sparsity is carried as data, and whether
unrolled per-design code adds anything to it is the separate question of §21.18.

*Pilot before funding: the ledger.* `xtask` instruments the `batch-cpu` solve and prints, per Newton iteration and
per reference robot, the operation counts of the four rows above and their measured share of the step, for the
dense path and the ancestor-sparse path on identical worlds, at contact counts drawn from the standard battery.
Two engineer-days, no device (TARGET). Falsifier: the four operations together are under one fifth of the measured
`batch-cpu` step for both reference robots. In that case the route keeps its original scope: mechanisms above 60
coordinates and padded buckets.

*Owner and verb.* `frob-dyn` owns the solve and `frob-model` the ancestor lists. `sim.run --solve-path {dense,
ancestor}` selects the path, and the `StepPlanId` includes it. The two paths order their arithmetic differently, so
they agree inside determinism class D2 and not bit for bit; a study never mixes them (§35), and a request that
would is refused with the two step-plan identities.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-KIN-807 | Known answer; Performance | The ledger reproduces 375, 969, 1,742 and 7,140 and the row counts exactly, and reports a ratio of 1 for a chain. The three-coordinate fill case takes the dense path and is flagged. The two paths pass the decision-level equivalence test of §35 on the standard battery. On the M2 device profile the whole-step gain is at least 1.15 for the 35-coordinate humanoid at 8,192 worlds | M2 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-ANCESTOR | The pilot's falsifier holds; or the device gain is below 1.15 on the humanoid and below 1.10 on the quadruped; or more than 20% of battery world-steps take the dense path | The dense path for 18 to about 60 coordinates as in §16.6; the tree route only where KILL-TREEOP leaves it |
<<<END>>>

<!-- B07 · chapter 17 · tier F · fills: K-018 -->
<<<INSERT chapter=17 title="Limb-archetype kernels">>>
**What it is.** [F]. This subsection extends §17.9 and uses the limb port interface of §14.6. Between the
table-driven kernel, which serves every design of a shape class, and a kernel specialized to one design, which
§17.9 reserves for survivors, there is a third level: a kernel specialized to a *limb archetype*. An archetype
fixes a limb's joint-class sequence and depth (DESIGN examples: a serial three-revolute leg, a four-joint
wheel-leg, a seven-revolute arm, a track flipper) and leaves every value in the model table: placements, screw
components, inertias, limits, card slots. Archetype kernels are compiled once per kernel version and never per
design, so a new design assembled from registered archetypes runs on them with no compile. Owner:
`frob-kernels` for the kernels; `frob-phylum` for decomposing a model table into a base and archetype
instances. Exposure: the option `--limb-kernels` of `phylum.sweep`; `phylum.plan` reports the archetype
coverage of a population.

**Mechanism (DESIGN).**

- *Registry.* At most 16 archetypes, drawn from the limbs of the archetypes of §14.10. A limb is an instance
  when its joint-class sequence and depth match after the canonical ordering of §14. A design with any limb
  outside the registry stays whole on the table-driven kernel.
- *Per-limb stages.* Stages 1 and 6 of §16.6 run per limb instance, with loops unrolled over the archetype's
  fixed depth and the joint-class column masks resolved at compile time. Worlds are grouped so that one
  execution group holds instances of one archetype.
- *The port is the only interface.* On the dense route a limb with k joints returns its k × k block of the mass
  matrix, the 6 × k coupling block expressed at its mechanical port, its composite inertia at the port (10
  scalars) and its bias terms, and the base kernel assembles the full matrix. On the tree-operator route it
  returns the `LimbPortPacket` of §14.6 (27 scalars).
- *Contact is untouched.* Stages 5, 7 and 8 run for the whole robot. Contact rows couple every limb through the
  base. An archetype kernel is a faster way to compute terms that the coupled solve needs, and nothing more.
- *Same bits.* The operation order inside an archetype kernel equals the table-driven kernel's order for the
  same limb, so results are bit-identical (INV-PHYLUM-05) and a design moves between the two paths without a
  new `StepPlanId`.

**Counterexample: a port packet is not the limb when its foot is down.** A point mass m on a massless leg stands
on a floor under gravity g. Treated as a free tree limb, its packet reports the body's inertia and a bias equal
to the weight m·g, and the base equation closed from the packet gives a downward acceleration g. With the
contact row the normal force is m·g and the acceleration is zero. Any path that closes the base equation from
port packets without the contact rows is wrong by exactly g here. The second control is the packet of §14.6
with value diag(0, 1, 1, 1, 1, 1): it is accepted as a packet and refused as an invertible matrix.

**Ceiling, stated first.** An archetype kernel touches the stages that per-design specialization touches, and
cannot beat it. FACT (Appendix H): those stages are 53 of 262 ns of an incumbent's humanoid step, so an
infinitely fast recursion gives 262/209 = 1.25 and one twice as fast gives 1.11. HYPOTHESIS: the gain is 1.05
to 1.15. What this level adds over §17.9 is reach: with zero compile per design it applies inside the edit loop
and to every design, where per-design kernels apply to survivors only. Two things are not claimed: a fused
kernel for all topologies at three quarters of homogeneous throughput, which rested on an occupancy miscount,
and a lossless limb summary under contact, which the counterexample above refutes.

**Pilot before funding.** A fourth arm in the pilot of §17.9, at no added calendar time: the 12-joint quadruped
as a base and four instances of the serial three-revolute archetype, measured on the whole step beside the
padded table-driven, unpadded table-driven and fully unrolled arms. Falsifier: the archetype arm gains less
than 1.10 on the whole step, or less than half of what the fully unrolled arm gains. A paper measurement at M1
accompanies it: the share of limbs in the design corpus (§24) and in 1,000 generated designs (§17.10) that
match a registry of 16.

**Gate and kill gate.**

| Gate | Claim | Threshold (TARGET) | Earliest |
|---|---|---|---|
| GATE-PHYLUM-808 | Performance; known answer (determinism) | Every registered archetype reproduces table-driven digests bit for bit on the pinned corpus; at most 16 archetypes cover at least 90% of the limbs of the design corpus; zero kernel compiles while 1,000 corpus edits are admitted; the 95% lower bound of the paired whole-step ratio against the table-driven kernel is at least 1.15 at 8,192 worlds; both hand controls reproduce. | M2 |

✖ KILL-LIMBKERNEL. Trigger: the pilot's falsifier holds; or coverage is below 80% with 16 archetypes; or the
gate's ratio fails after one repair cycle. Fallback: the table-driven kernel for every design, per-design
specialization for survivors under §17.9's own gate, and the limb port interface kept for the tree-operator
route and the per-limb caches of §14.6.

In the head-to-head of §17.14 the level is reported as a variant of arm A on measures (1) and (2).
<<<END>>>

<!-- B08 · chapter 18 · tier F · fills: K-025 -->
<<<INSERT chapter=18 title="Response batches: anchor, acceptance, known answers, gate">>>
**What it adds.** [F]. This subsection extends the paragraph "Response batches on a fixed active set" of §18.5,
which states the mechanism, its ceiling, its pilot and KILL-RESPONSE, and leaves four things unspecified: the
identity a batch is bound to, the tests a column must pass, the source of λ_lo, and a gate. Owner:
`frob-contact`. Verb: `contact.response`.

**Anchor (DESIGN).** A batch is keyed by an anchor digest over everything its factor depends on, at one
converged step of one world: the configuration, the mass matrix, the constraint Jacobian, the regularization,
the cone parameters, and the active set with a stick or slip label per row. A batch is served only against the
anchor it was factored for. Any other request refactors (refusal otherwise: response batch requested against a
stale anchor).

**Per-column acceptance.** A column b is accepted only when all four hold, each evaluated in the precision of
the step that produced the anchor:

1. the residual r = b − H·x̂ is formed explicitly, not estimated from the solve;
2. every sticking row remains strictly inside its cone, and the slack is reported;
3. no open row closes and no active normal force changes sign;
4. the bound ‖x̂ − x*‖₂ ≤ ‖r‖₂/λ_lo is attached to the column.

A column that fails test 2 or test 3 is rerouted to the full solve of §16.6 and counted.

**The lower bound λ_lo.** H = A + JᵀGJ with G positive semidefinite (§16.6), so a lower bound on the smallest
eigenvalue of A is one for H. λ_lo is a shift s for which A − s·I is shown positive definite by the
`rational-ldlt` proof kind of §12, computed once per anchor on the host; A depends on the configuration and
not on the active set, so the certificate serves every column of the batch. Where a tighter value is wanted the
same proof kind is applied to H − s·I. A batch whose λ_lo has no certificate reports its bounds as class
Estimated.

**Known answers, by hand.**

- *The bound, at equality.* H = [[2, 1], [1, 2]] has eigenvalues 1 and 3, so λ_lo = 1. For b = (3, 3) the
  solution is (1, 1). The perturbed x̂ = (1.01, 0.99) has residual (−0.01, 0.01) of norm 0.01414, and its true
  error also has norm 0.01414: the error lies along the eigenvector of the smallest eigenvalue. A bound formed
  with λ = 3 would claim 0.00471 and is violated; that mutation must be caught.
- *The mode guard.* A block rests on a floor with μ·N = 5 N and the anchor is in stick. A counterfactual
  tangential load of 4 N is accepted: friction answers 4 N and the block stays. A load of 6 N must be rerouted:
  on the frozen sticking set the linear response reports no motion, and the true response slides under a net
  force of 1 N.
- *The flop count.* At n = 96 unknowns and 64 columns, a full solve per column costs n³/3 + 2n² = 294,912 +
  18,432 = 313,344 flops, 20,054,016 in all. One factor and 64 solves cost 294,912 + 64 × 18,432 = 1,474,560.
  The ratio is 13.6, and it reaches 10 from 23 columns. Against a baseline that also factors once it is 1,
  which is the ceiling §18.5 already states.

**Gate.**

| Gate | Claim | Threshold (TARGET) | Earliest |
|---|---|---|---|
| GATE-TRAC-809 | Known answer | The three fixtures reproduce and the λ = 3 mutation is caught. On 10⁴ battery contact states with 64 random columns each, solved in `f32` and compared with a full `f64` re-solve of every column: zero accepted columns whose true error exceeds the attached bound, and zero accepted columns whose re-solve has a different active set or stick label. The rerouted share is reported per robot class. A batch requested against a changed anchor is refused. | M4 |

✖ KILL-RESPONSE gains a second trigger: a bound violation or a false mode acceptance that survives one repair
cycle. Its fallback is unchanged: full solves per query.
<<<END>>>

<!-- B09 · chapter 19 · tier M · fills: K-044 causal tile recomputation, surviving narrow form "far-field fields only" -->
<<<INSERT chapter=19 title="Far-field field reuse across forks">>>
This subsection adds one mechanism to the living worlds of §19.10: a fork (§27.3, §29.8) may share the parent's field
tiles and recompute only those whose inputs differ. Tier [M]. Owners: `frob-scene` for tile keys and the tile store,
`frob-chronicle` for retention over the fork window. It is reported by `autopsy.fork` and `run.fork`; it adds no verb.

**Scope.** Fields only: film tiles, and where they are admitted the thermal, optical-volume and fluid-island fields of
the evaluation tier. Rigid bodies, contacts, sensors and controllers of a fork are always recomputed in full. In a
robot-centred scene they dominate the step and always differ from the parent, which is why the general form of this
idea, a dependency graph over the whole world, is not built.

**Tile versions.** A tile's content after a substep is identified by

  TileKey = BLAKE3(field kind, math contract and kernel versions, substep index,
                   content digests of the tile and of every tile in its stencil before the substep,
                   digest of the events applied to the tile in the substep).

Events are sources and sinks, the motion of a fixture, door or cart that overlaps the tile, and robot contacts that
deposit, remove or drag film. A store maps keys to tile contents for the parent's fork window.

**Algorithm, per substep of a child.** Form each tile's key from the child's inputs. If the store holds the key, the
child refers to the stored content and no kernel runs for that tile. Otherwise the tile is dirty: it is recomputed,
its content digest taken and the result stored. Keys are built from content digests, not from the fact of
recomputation. A recomputed tile whose content equals the parent's therefore leaves its neighbours' next keys
unchanged, and the dirty set shrinks when a disturbance dies out.

**Preconditions.** The field kernels return bit-identical results for identical inputs whatever the batch (class D1
or D2 with the rule of §35.5). The update's stencil covers its domain of dependence for one substep; this is the
scheme's stability bound and is part of the tile's math contract. Every write to a field goes through the event list.
Reuse is refused under classes D3 and D4, and for a field kind whose contract states no stencil.

**The bound.** Let a be the share of a fork's compute spent on tile updates, d the dirty fraction of those updates and
b the bookkeeping (hashing and lookups) as a fraction of full cost. A fork costs (1 − a) + a·d + b of a full
recomputation.

| a | d | b | Cost | Factor |
|---|---|---|---|---|
| 0.95 | 0.02 | 0.01 | 0.079 | 12.7 |
| 0.95 | 0.0421 | 0.01 | 0.100 | 10.0 |
| 0.90 | 0.02 | 0.01 | 0.128 | 7.8 |
| 0.20 | 0 | 0.01 | 0.810 | 1.23 |

A robot in one room with a small film patch has a small a, and no dirty fraction helps it. DESIGN: reuse is enabled
only for worlds whose phase ledger shows a ≥ 0.8 (TARGET): spills across several rooms, steam, fluid islands on the
evaluation tier.

**How fast the dirty set can grow (FACT, arithmetic).** A disturbance travels at most one stencil radius per substep.
In a row of N tiles disturbed at one end, at most K·(K + 1)/2 tile-updates are dirty over K substeps: 210 for K = 20,
which is 10.5% of the work at N = 100 and 1.05% at N = 1,000. In a plane with a square stencil at most (2k + 1)² tiles
are dirty after k substeps, and Σ_{k=0}^{K−1} (2k + 1)² = K·(2K − 1)·(2K + 1)/3 = 10,660 for K = 20, which is 5.3% of a
100 × 100 tile domain over 20 substeps and already above the 4.21% line. The cone is the worst case. Whether real
disturbances die out sooner is what the pilot measures; it is not assumed.

**Known answers.** A null fork has an empty dirty set at every substep. A fork that changes the robot's path in one
room recomputes no tile of a dry room it never enters. A door swung through a dry room is an event with no field
effect: the tiles it overlaps are recomputed while it overlaps them, their content equals the parent's, and no
neighbour becomes dirty. In every case, tile digests with reuse equal tile digests without it.

**Memory.** The store holds the parent's tile versions for the fork window only (a lead of at most 2 s and 1 s after
the failure, §27.3): one base version per tile plus one version per tile-update that changed content. `sim.estimate`
reports the bytes before a fork batch is admitted. A store over its budget disables reuse for that batch, which then
recomputes in full and stays exact.

**Receipt.** Every fork receipt carries `field_reuse` with the measured a, d and b and the counts of tiles referenced
and recomputed. `--no-field-reuse` forces full recomputation.

**Pilot before funding.** Paper-scale, on the CPU: an explicit thin-film update on a 256 × 256 tile grid divided into
two rooms, a scripted robot path in one room, and 100 forks that change the path at the fork tick; measure a, d and b
and compare digests. Falsifier: d above 4.21% on the two-room case, b above 5%, or one digest mismatch. Cost: about
two engineer-days (TARGET).

**Ceiling, stated before any speed test.** With every far tile clean the factor is 1/((1 − a) + b): 1.23 at a = 0.2,
4.8 at a = 0.8 and 16.7 at a = 0.95, for b = 0.01. It applies to forks of worlds with living fields and to nothing
else: not to first runs, and not to worlds whose fields are a small share of the step.

**Gate.** GATE-HAB-810 (Known answer and Performance of the method, M6). On the two-room spill fixture, 1,000 forks
with reuse equal 1,000 forks without, tile digest for tile digest at every substep; the three known answers hold; the
dirty set never exceeds the stencil cone; and where measured a is at least 0.9, fork cost is at most 0.15 of full
recomputation at the paired 95% bound.

**Kill gate.** ✖ KILL-FARFIELD. Trigger: one digest mismatch that survives a repair cycle; or the pilot's falsifier;
or no admitted living-world fixture reaches a ≥ 0.8. Fallback: the full-replay forks of §27.3, which are exact and
need no dependency tracking.
<<<END>>>

<!-- B10 · chapter 20 · tier S, F · fills: estimator-observability and self-occlusion sensor co-design -->
<<<INSERT chapter=20 title="Visibility under self-occlusion, and sensor mounts as design variables">>>
**Visibility along a motion [S]; mounts in the optimizer [F].** This extends the sensor-coverage analysis A-14 and the
placement use of the Gramian in §20.11. Coverage by ray geometry against the scene misses the occluder that changes
most between designs: the robot's own limbs and whatever it carries. A torso camera is blinded by a tote just before
a step; a low body cannot see over a lip; a swinging leg crosses the field of view once per stride.

*The query.* `sense.visibility` takes a design, a payload envelope, a scene and a nominal motion (a route with a gait
or drive profile). At every sample instant of every ranging sensor it casts the sensor's pattern against three
occluders: the scene field, the robot's own collision primitives at the pose of that instant, and the declared
payload. A target is visible at an instant when at least the card's minimum number of valid returns land on it after
the optical masks of §20.6. Targets are the contact sites the motion will use (footholds, wheel paths), the
thresholds and steps of the route, and the hazard regions of the scene's hazard list (§19).

*Two numbers per target.* With t_c the instant the body first touches or crosses the target:

- **lead** = t_c − (first instant the target is visible);
- **blind interval** = t_c − (last instant the target is visible).

The lead must cover the sense-decide-actuate chain of A-21 plus the controller's planning horizon; the blind interval
must stay under the bound the task declares. A third figure, the **blind dwell**, is the floor area inside the
stopping distance ahead of the body that no sensor has seen within a declared age. Each is reported as a profile
along the route with the occluder that caused each gap (scene, named link, or payload).

*Hand case.* A depth camera at the body's front edge at height H looks down and forward. A carried box has its top
front corner 0.4 m ahead of the camera and 0.3 m above the floor. The ray grazing that corner meets the floor at
x = 0.4·H/(H − 0.3), and floor nearer than that is hidden. At H = 0.5 m, x = 1.0 m; with the next foothold 0.2 m
ahead of the camera at 1 m/s the blind interval is 0.8 s. At H = 0.9 m, x = 0.6 m and the blind interval is 0.4 s.
The price is in another analysis: a 1 kg mast head raised by 0.4 m on a 20 kg robot lifts the centre of mass by
0.02 m, and with a half-track of 0.2 m the static tip-over angle falls from atan(0.2/0.30) = 33.7° to
atan(0.2/0.32) = 32.0°. With the box removed the hidden region ends at the body's own front edge.

*Mounts as design variables [F].* A sensor mount is already a model field with a typed edit (`move-sensor`, §14.7).
The optimizer of §28.8 may hold mount transforms and payload-bay geometry as continuous parameters, with lead, blind
interval and the Gramian index of §20.11 as objectives or constraints beside the static margins of A-08 and the
clearance of A-18. Every move of a mount recomputes 𝔾_obs (§20.13). Nothing here claims an estimator bound: the
Gramian remains a local index, the alias witness of §20.12 remains the test for finite ambiguity, and bias states
remove the state-independence that the invariant error enjoys without them.

*Owners and verb.* `frob-sense` owns the query and reuses the ray kernels of §20.6 with the model's collision
primitives as moving occluders; Atelier lists the result under A-14; `frob-opt` consumes it. Verb: `sense.visibility`
(run). At stage 7 of `design.check` a target with a lead shorter than the declared latency chain is a refusal with
ranked repairs as typed edits (raise or tilt the mount, move the payload, shorten the latency).

*Ceiling, pilot and gates.* Ceiling: the share of estimated-state failures caused by visibility and not by
calibration, filter dynamics or control, which is unknown; it is the same unknown as in §20.11 and the same pilot
settles both. Pilot before funding the optimizer use: on the M3 battery classify failures under estimated state by
cause; under a fifth from visibility or geometry and the optimizer use is not built.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-SENS-811 | Known answer | The box-and-mast case: hidden floor boundary at 1.000 m and 0.600 m within 1 cm; no hidden region beyond the body's front edge with the payload removed; a planted occlusion by a swinging limb is found and attributed to the link; lead and blind interval identical on every tier inside their determinism class | M3 |
| GATE-SENS-812 | Performance of the method | On 20 designs carrying payloads across stairs, a doorway and wet tiles under estimated state: mounts recommended by the optimizer cut the gap between privileged-state and estimated-state success by at least half; lead and blind interval correlate with the estimated-state failure rate at 0.65 or more | M4 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-VISIBILITY | GATE-SENS-812 fails on both counts, or the pilot attributes under a fifth of estimated-state failures to visibility or geometry | `sense.visibility` stays as a report and a stage-7 check; mounts are not optimizer variables |
<<<END>>>

<!-- B11 · chapter 22 · tier M · fills: K-013 -->
<<<INSERT chapter=22 title="A lifted linear planning model for one soft-soil regime">>>
**What it is.** [M]. A lifted linear predictor is a finite matrix acting on a dictionary of functions of the
state. As a global model for control through contact it is not built. A contact-rich hybrid flow has no
finite-dimensional invariant subspace of such functions, and size alone settles it: a quadratic dictionary on
60 states has 1 + 60 + 1,830 = 1,891 functions, so one dense map has 1,891² ≈ 3.58 × 10⁶ coefficients before
any mode jump is represented. One bounded experiment is kept, off the critical path. It covers a single regime
with no contact-mode change, a tracked or wheeled base driving on soft soil (§18.10), where the dynamics are
smooth and a rollout of the compiled model is comparatively expensive. There a lifted model is tried as the
*planning model* of the sampling controller of §22.8 and judged at equal tracking accuracy. Owner:
`frob-control`; bands come through the airlock from `fs-surrogate`. Verbs: `control.lift` (fit, calibrate,
report) and `control.mpc --planner lifted:<id>`.

FACT: `fs-surrogate` provides exact DMD, EDMD over a polynomial dictionary with ridge regularization, and
split-conformal forecast bands per horizon with an optional simultaneous adjustment; it states that the band is
its only accuracy statement and holds only inside the calibration distribution
(`frankensim/crates/fs-surrogate/src/koopman.rs`). Its EDMD is autonomous and has no input term. The
input-affine fit below is therefore ours, and a request for it goes upstream (Appendix F).

**Model (DESIGN).** The reduced state x has 7 entries: forward speed, yaw rate, the slip of each track, the
sinkage of each track, and pitch. The inputs are u = (u_L, u_R), the two track torque commands. The dictionary
ψ(x) holds every monomial of total degree at most 2: C(9, 2) = 36 functions. The predictor is bilinear in the
lifted state and the input,

  z⁺ = A·z + u_L·B_L·z + u_R·B_R·z,  z = ψ(x),

fitted by ridge least squares on triples (x_k, u_k, x_{k+1}) from `batch-cpu` rollouts in `f64`. The state is
read back from the linear monomials and lifted again at every step. The three 36 × 36 maps hold 3,888
coefficients, and one planning step is three matrix–vector products, 3,888 multiply–adds.

**Admission.** The model carries its regime as data: the soil card with its declared ranges of pressure, slip
and sinkage (§18.10), a box of speed and yaw rate, a slope range, and the condition that every track sample
stays in contact. A simultaneous 95% forecast band over the planning horizon is calibrated on trajectories held
out from the fit. A query outside the regime, or a planner tick whose measured state leaves the box, is refused
and that tick is served by `mpc-sample` on the compiled model (refusal: lifted planner outside its calibrated
regime). Results carry `screening: sampling-control` and `surrogate`, are class Estimated, and inherit every
rule of §22.8, including that the controller is never the evaluator of record.

**Known answer and counterexample.**

- *Linear recovery.* Data from x⁺ = 0.9·x with the dictionary {1, x} must return the map diag(1, 0.9) and the
  eigenvalue 0.9 to 10⁻⁹.
- *A mode switch defeats a polynomial lift even when the fit is exact on its data.* Take the reset map
  x⁺ = |x| and the dictionary {1, x, x²}. On the samples x ∈ {−1, 0, 1} the targets are {1, 0, 1}, which x²
  reproduces exactly, so the fitted predictor is x⁺ = x² with zero training residual. At x = 0.5 it predicts
  0.25 where the truth is 0.5. A zero residual on the fitting set is no evidence. Only the held-out band is,
  and only inside one mode.

**Ceiling, stated first.** The lifted model can remove at most the planner's rollouts, N_s·H·N_it model steps
per tick: 4 × 10⁴ to 2 × 10⁵ in the configurations of §22.8, against one plant step. Inside the controller the
ceiling is that ratio. On a design cycle it is the share of wall time that the screening rung of §22.8 spends
on soft-soil tasks in the phase ledger of §39, which is unmeasured and expected to be small. Nothing in M0–M7
depends on this item.

**Pilot before funding.** On `batch-cpu` only: fit and calibrate on the tracked skid-steer of §16.6 over one
soil card, and report band width against horizon. Falsifier: the simultaneous band at a 0.9 s horizon is wider
than the task's tracking tolerance (DESIGN default: 5% of commanded speed and 0.05 m of path offset). Equal
accuracy is then unreachable and the experiment stops.

**Gate and kill gate.**

| Gate | Claim | Threshold (TARGET) | Earliest |
|---|---|---|---|
| GATE-REFLEX-813 | Performance of the method | Linear recovery to 10⁻⁹; the absolute-value control returns 0.25 against 0.5 and that fit is denied admission; empirical coverage of the simultaneous 95% band at least 0.93 on 1,000 held-out trajectories; closed-loop tracking error with the lifted planner within 10% of the compiled-model planner's on 100 episodes paired by world address; planner cost per tick at least ten times lower at that accuracy; every out-of-regime query refused. | M6 |

✖ KILL-SOILLIFT. Trigger: the pilot's falsifier holds; or more than 1% of calibration trajectories contain a
contact-mode change inside the declared regime; or the accuracy or cost clause of the gate fails after one
repair cycle. Fallback: `mpc-sample` on the compiled model, as §22.8 specifies; no lifted model is offered for
any other regime.
<<<END>>>

<!-- B12 · chapter 22 · tier S · fills: §4.2 "Monitors cannot see a breach until it is too late"; §4.2 scorer addition "two-mode sampled-data shield fixture with a verified backup domain"; the control named in BET-25 -->
<<<INSERT chapter=22 title="The pre-slip aliasing fixture and the sampled-data backup domain">>>
This subsection extends §22.12.3 and §22.16. It defines the control named in BET-25, states when a monitored
assumption may count toward the bound of §22.12.3, and gives a two-mode sampled-data fixture whose backup domain is
checked by hand. Tier [S]: a known answer with negative controls. Owner: `frob-shield`, with the fixture registered in
`frob-conformance`. Verbs: `shield.fit` (the rule and its refusal) and `shield.check` (replay of the fixture).

**The fixture.** A cart on a level floor approaches a keep-out line at v₀ = 2.0 m/s with its wheels rolling and no
drag. A brake command asks for a deceleration of 5.0 m/s²; the floor delivers min(5.0, μ·g). There are two worlds:
A with μ = 0.8 (μ·g = 7.848 m/s², so the wheels hold and the cart decelerates at 5.0) and B with μ = 0.3
(μ·g = 2.943 m/s², so the wheels slide). The modes are `stick` and `slip`. The control period is Δt = 0.02 s. The
sensors are wheel speed, body acceleration and range s to the line.

| Quantity | World A | World B |
|---|---|---|
| Stopping distance v₀²/(2·decel) | 4/10 = 0.400 m | 4/5.886 = 0.6796 m |
| Observations before the first braking tick | Identical in both worlds, sample for sample | |

**Why the histories are identical.** Before braking the traction demand is zero, so no contact law reads μ, and the
delivered transcripts of the two worlds are equal. A monitor is a function of the delivered transcript. It therefore
returns the same output in both worlds up to the first braking tick, whatever its design. The fixture asserts
transcript equality as a precondition and is void on any tier where it fails.

**Arm 1: a nominal controller with a slip monitor.** The controller assumes μ ≥ 0.5 and brakes when s ≤ 0.45 m (the
0.40 m of world A plus 0.05 m). A monitor compares wheel speed with body speed and raises an alarm on slip; the
backup is full braking. In world A the cart stops 0.05 m short. In world B the alarm is raised at the first braking
tick and the backup is already running: the cart crosses the line at √(4 − 2·2.943·0.45) = 1.162 m/s and stops
0.6796 − 0.45 = 0.2296 m beyond it. For this stratum P(violation | alarm) = 1, and the bound of §22.12.3 is at least
E[N_alarm], which says nothing. The alarm is correct and too late.

**The rule this fixture enforces.** For an assumption with a monitor and a backup, let t_d be the first tick at which
the delivered transcript of a violating world differs from that of every admitted non-violating world, and let ℓ be
the monitor's latency in ticks. The pair is *timely* if the state at t_d + ℓ lies in the backup's domain for every
violating world of the belief set. An assumption whose monitor is not timely may not be listed as monitored: its
whole range goes inside the quantifier of §22.12.1, so the rows hold for every value in the belief set. `shield.fit`
refuses a shield that lists such an assumption as monitored, and returns the first distinguishing tick, the state
there and the two changes that would admit it. Here they are: brake by s ≤ 0.7196 m, or limit the approach speed to
1.57 m/s, the largest v with v²/5.886 + 0.02·v ≤ 0.45.

**Arm 2: the shield with the belief set inside the quantifier.** Belief set μ ∈ [0.3, 0.8], so μ_min·g = 2.943 m/s².
The backup is full braking, with domain

  D = {(s, v) : s ≥ v²/(2·μ_min·g) + v·Δt}.

*Invariance, by hand.* If (s, v) ∈ D at a tick, coasting for one period leaves s − v·Δt ≥ v²/(2·μ_min·g). The filter
permits coasting only inside D and otherwise brakes. Braking therefore begins at a tick whose range is at least
v²/(2·μ_min·g), and the deceleration from then on is at least μ_min·g in either mode (5.0 in `stick`, μ·g ≥ μ_min·g
in `slip`), so the cart stops at or before the line for every μ in the belief set. At v₀ = 2.0 m/s the threshold is
0.6796 + 0.04 = 0.7196 m. The shield has no violation in either world. Its cost is visible: in world A the cart stops
between 0.28 m and 0.32 m short of the line, depending on where the sampling instant falls, where 0.05 m would have
sufficed.

**Negative control: the continuous-time barrier in a sampled loop.** Drop the term v·Δt and test
h = s − v²/(2·μ_min·g) ≥ 0 at each tick. At a tick with s = 0.70 m, h = 0.0204 and the filter permits coasting. At the
next tick s = 0.66 m, braking begins, and in world B the cart stops 0.6796 − 0.66 = 0.0196 m beyond the line. The
overshoot of this mutant is at most v·Δt = 0.04 m. This is the inter-sample term L·Δt of §22.12.1 at its smallest
size.

**What the fixture does not show.** It has one degree of freedom and an exact stopping law. It says nothing about the
tracking margin of §22.12.4 or about legged backups, whose domains are `Tested` only.

**Gate.** GATE-SHIELD-814 (Known answer, M4). On the referee and on the `strict` profile, over 1,000 paired episodes
with v₀ drawn from [1, 3] m/s and friction pairs from [0.3, 0.8]: the two transcripts are bit-equal before the first
braking tick in every pair. Arm 1 violates in the low-friction world in every episode whose stopping distance exceeds
its trigger range, with the overshoot within 1 mm of the closed form. Arm 2 has no violation in 10⁴ episodes. The
mutant without the inter-sample term produces at least one violation, none larger than v₀·Δt. `shield.fit` refuses
Arm 1's monitored-assumption listing and prints both admitting changes.
<<<END>>>

<!-- B13 · chapter 22 · tier F · fills: adjudication H-01 (the sampling controller as the delivered controller; the categorical refusal of §22.8) -->
<<<INSERT chapter=22 title="The sampling controller as the delivered controller">>>
**The delivered-controller lane [F].** This amends the rules of §22.8 and ADR-REFLEX-03. The rule "never the
evaluator of record" guards one substitution: scoring a design under a sampling controller and reporting the score
as if a trained policy had earned it. It does not cover the case where the sampling controller is itself what the
mission delivers. An accepted answer is a design, a controller and an evidence package (§2.1). When the controller
in that package is of kind `mpc-sample`, episodes run under it are the only evidence about it, and a refusal keyed
on the controller kind would leave the package with no evidence at all.

DESIGN: a task or mission may declare `controller.delivered = mpc-sample`. The declaration freezes a planner
record, whose digest is part of the `ControllerId`.

```
planner_record  (frob.control.planner/1)                                      DESIGN, fixes intent
  kind            mpc-sample
  budget          samples N_s · horizon steps H · iterations N_it      model steps per tick = N_s·H·N_it
  planning_step   h_p · control rate
  cost_terms      content hash of the class-default or task-declared cost
  planning_model  ModelId rolled out by the planner · source of its parameters (nominal | estimated | plant)
  state_source    estimated (the filter of §22.3) | privileged
  compute_target  named computer · measured planning time per tick: median, 99th percentile · deadline
```

Episodes under a frozen planner record are admitted as the scoring source of acceptance, promotion and
qualification **for that record only**, when four conditions hold. Otherwise the refusal of §22.8 stands.

1. *Not model-privileged.* The planning model's parameters do not come from the plant's randomization draw, and
   `state_source = estimated`. An episode flagged `model-privileged` is refused as acceptance evidence.
2. *A plant the planner does not own.* The plant runs on the tier the battery names and is promoted to the referee
   by the rule of §16.8, as for any controller. The planner keeps its own model and its own planning step.
3. *Measured latency.* `compute_target` carries a measurement. The planner's output is delayed by the measured
   99th-percentile planning time, rounded up to whole control periods, and a tick that misses its deadline holds
   the previous plan. A record with a modeled or zero delay is refused for acceptance.
4. *The same bar.* The battery, the requirement verdicts and the failure bound are those of any other controller,
   and the cost is charged to the phase ledger of §39 in real GPU-seconds and CPU-seconds.

The verdict names what was accepted: `accepted with delivered controller mpc-sample@<record digest>`. It says
nothing about a trained policy for the same design, and a later change of controller voids it under §28.13.

*Why condition 1 is not a formality.* A cart at 2 m/s must stop before a wall on a floor whose friction is drawn
from {0.2, 0.8}; stopping distance is v²/(2·μ·g). A planner that reads the plant's draw brakes at 1.019 m on the
slippery floor and 0.255 m on the grippy one and never fails. The same planner holding the nominal μ = 0.5 begins
braking 0.408 m from the wall and overruns by 0.612 m (1.0194 − 0.4077) on every slippery world, half of the draws,
unless its estimator detects the floor in time. The privileged run reports zero failures in 299 episodes for a
controller that fails every second episode.

*Break-even, stated before any test.* FACT arithmetic on the figures of §22.8. A 1% failure bound at 95% confidence
needs 299 failure-free episodes; training once costs about 600 GPU-seconds for the quadruped and 3,600 for the
humanoid. The lane is cheaper than training only below 600/299 = 2.0 GPU-seconds per 20 s episode for a quadruped
and 3,600/299 = 12.0 for a humanoid. The published settings cost about 20 and 62 GPU-seconds per episode: 10 and 5
times over. At the throughput targets of §36 (2 M and 0.5 M world-steps/s) the break-even budgets are 4.0 × 10⁶ and
6.0 × 10⁶ model steps per episode, which at 50 Hz is 4,000 and 6,000 model steps per tick. One setting inside the
quadruped budget is N_s = 64, H = 16 at a 20 ms planning step and N_it = 3 (3,072 steps per tick), against 65,536
for the published quadruped setting. HYPOTHESIS: a planner that small completes the reference tasks for some
classes. FACT (Appendix H): controllers with about 30 samples run a real quadruped from a CPU; the published
successes use hand-built swing-foot or stepping references; success in one study fell from 10 of 10 to 6 of 10 and
0 of 10 when the spline family changed; nothing was found for tracked robots or for a varied design corpus.
Ceiling: no speed benefit at any published setting. The benefit is a capability: a design whose deliverable
controller is a planner can be accepted at all, with its price on the ledger.

*Owner, verbs and refusal.* `frob-control` owns the planner record and the latency model; `frob-arena` admits the
episodes. `control.mpc --freeze` writes the record and `eval.qualify` consumes it. An acceptance request whose
scoring episodes are `mpc-sample` without a frozen, non-privileged, latency-measured planner record is refused,
under the existing refusal of §22.8, with the missing condition named.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-REFLEX-815 | Known answer; Fidelity | The cart: the privileged run is refused as evidence, and the nominal-model planner overruns by 0.612 m within 1% on the slippery draws. On the M3 corpus, every delivered-controller verdict on the fast tier survives promotion to the referee plant or is labelled `tier-sensitive`; the measured 99th-percentile planning time fits the control period on the named computer in at least 99% of ticks; the phase ledger prints the lane's cost beside the cost of training once | M3 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-MPCRECORD | More than 10% of delivered-controller verdicts reverse at the referee; or no named computer meets the deadline in 99% of ticks for any class on the pilot set | The rule of §22.8 as first written: screening and fallback only. A mission that requires a sampling controller returns `provisional: controller-not-qualifiable` with its screening episodes |
<<<END>>>

<!-- B14 · chapter 23 · tier S · fills: adjudication H-22 (a high rank correlation does not protect the winner) -->
<<<INSERT chapter=23 title="The winner check for a selecting rung">>>
**A selecting rung must pass a winner check [S].** This extends the consequences fixed in §23.11 and the
rank-agreement row of §28.10. A lower bound of 0.7 on the top-band Kendall τ admits a rung to selection. But τ
counts inverted pairs and weighs every pair equally; it does not say that the best design survives a cut.

*Counterexample.* Forty designs. The cheap rung reproduces the true order exactly, except that the true best is
ranked third. Among the true top ten, 2 of 45 pairs are inverted, so τ = (43 − 2)/45 = 41/45 ≈ 0.911, far above
0.7. A cut that keeps the top 5%, two designs, discards the winner.

*Rule.* For a rung admitted to selection with m finalists wanted, the number of designs advanced is the largest of
three numbers: m; the survivor count of the conformal margin rule (§28.9, layer 2); and the **containment rank**
k_c(m), the smallest k such that, in the calibration experiment and in every audit-lane draw to date for the class
and task family, the higher rung's top m all lie inside the lower rung's top k. The ledger row of §28.10 gains two
fields: `containment_rank(m)`, and the **regret** of a cut, which is the higher-rung score of the best design the
cut discards minus that of the best design it keeps. Positive regret means the winner was lost. A selection whose
retention is below k_c(m) is refused, with k_c(m) as the repair. Where a class has no calibration, the rung
discards by the fixed table and does not select.

*Owner and verbs.* `frob-opt`. `eval.rank-agreement` reports τ, top-k recall, the containment rank and the regret
of the requested cut; `opt.cascade` reads k_c(m) when it sizes a rung transition.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-CORTEX-816 | Known answer | On the forty-design fixture `eval.rank-agreement` returns τ = 41/45 on the true top ten, top-1 recall 0 at k = 2, containment rank 3 for m = 1 and positive regret at k = 2. A request to select with k = 2 is refused and the same request with k = 3 is accepted | M3 |
<<<END>>>

<!-- B15 · chapter 23 · tier M · fills: assessment of the proposed hypothesis on contact adjoints with joint actor-critic training (learning arm) -->
<<<INSERT chapter=23 title="Short-horizon gradient training with design parameters">>>
**Short-horizon gradient training with design parameters [M].** This is the learning arm that would consume the
step adjoints of §16; the gradient mechanism, its step labels and its kill gate are specified there. It is off the
critical path and is not funded before the step-adjoint gate has passed.

*What it is.* Policy parameters θ and a continuous design vector φ (up to about 20 entries of the parameter row of
§16.6) are updated together from analytic gradients of a short rollout. For a window of K control steps, K ∈ {16,
32}, the loss is the negated sum of rewards in the window plus a learned value V_ψ at its end. Its gradient in θ
and φ is one reverse sweep through the window: policy network, step adjoints, reward terms. The critic V_ψ is
fitted to temporal-difference targets as in on-policy actor-critic training. Three guards make it usable where
contact changes.

1. *A cosine monitor.* On a fixed share of windows (DESIGN default 1 in 16) the analytic design gradient is
   compared with the paired central difference of §17.13 on the same world addresses. The running median cosine is
   logged. Below 0.5 the window halves; at K = 8 and still below 0.5 the design update switches to the paired
   stencil and the run is labelled `gradient: stencil`.
2. *Step labels.* A window that contains a step the adjoint labels anything other than `smooth` contributes to the
   policy update and not to the design update, because its design gradient is one-sided or fixed-mode.
3. *Design updates are proposals.* φ moves inside the family's box. Every design the run reports is a typed edit
   sequence, recompiled and scored by the ordinary ladder (§28.9). No verdict cites a gradient.

Training with analytic gradients is a labelled D4 training profile (§35); every score that enters the ladder is an
evaluation on a D2 profile.

*Why the window is short.* At the closed-loop exponent quoted in §27.9, 0.16 per control step, a perturbation
grows by e^(0.16·K): 13 at K = 16, 167 at K = 32 and 7.8 × 10⁸ at K = 128. A gradient through 128 steps is
dominated by its variance, and its mean tells the optimizer nothing. The exponent is a published figure for other
systems; the monitor measures ours.

*Rival check.* FACT (Appendix H): several incumbents differentiate their step, and short-horizon actor-critic
training on a differentiable simulator is published. This is not a differentiator, and a composed incumbent can
host it. It is here because joint refinement of a selected design and its controller is the one stage where
population methods are known to be slow.

*Ceiling, before any speed test.* The finalist refinement stage only. Its share of the phase ledger (§39) bounds
the end-to-end gain, and nothing here helps screening.

*Pilot before funding.* One quadruped on rough ground and stairs with 20 design parameters, three seeds. Arms at
equal GPU-hours: this method; PPO with a derivative-free outer loop on φ; PPO with the paired stencil. Falsifier:
the method does not reach the matched, referee-promoted return in at most 0.35 of the derivative-free arm's
GPU-hours, or the median cosine is below 0.5 on more than 40% of monitored windows.

*Owner and verb.* `frob-learn`; `train.start --algo short-horizon-ac --design-params <list>`.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-CORTEX-817 | Performance of the method | The pilot's comparison on both tasks: matched referee-promoted return in at most 0.35 of the derivative-free arm's GPU-hours; cosine at least 0.5 on at least 60% of monitored windows; every reported design re-scored by the ladder, none promoted on a gradient | M6 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-SHORTGRAD | The pilot's falsifier fires, or the gate fails after one repair cycle | PPO with the paired stencil of §17.13 where that passes its own gate; otherwise derivative-free search over φ |
<<<END>>>

<!-- B16 · chapter 24 · tier F, S · fills: attack the task specification before the design -->
<<<INSERT chapter=24 title="Auditing a task before any design runs on it: foil bodies and intent invariants">>>
**Task audit [F]; intent invariants [S].** The detectors of §23.16 ask whether a good score is an artifact of the
solver. A behaviour that is physically real and unintended passes every one of them: sliding on the belly where
walking was meant, tumbling down a flight to reach the goal region, riding a wall, satisfying the predicate only at
the instant it is checked. Faster search finds the cheapest such loophole sooner, and one loophole invalidates every
evaluation of the campaign that ran under it. `task.audit` attacks the task itself before any candidate is scored.

*Foil bodies.* A library `assets/foils/` of bodies that are deliberately over-capable or degenerate, written in the
design language and marked `foil: true`. Each runs in the task's own scene and is scored by the task's own
requirement and cost. A foil is never a candidate: offering one to `opt.*`, `eval.compare` or `eval.qualify` is
refused.

| Foil | What it is | What it probes |
|---|---|---|
| Free centroidal body | One rigid body with the mass and inertia of the design class, applying bounded wrenches at the contact sites the task designates, inside friction cones; no limbs, no actuator limits | Whether the predicate can be met by forces alone, without the intended mechanism |
| Relaxed family member | The current design with card limits multiplied by 3 (DESIGN) and collision enabled on every link | Behaviours the real design is only barely kept from |
| Degenerate shapes | A sled, a ball, a tumbling box and a single-leg hopper of the class's mass | Sliding, rolling, tumbling and bouncing routes to a satisfied verdict |

*Search.* The sampling controller of §22.8 and rank-based search over open-loop parameters, filling a
quality-diversity archive (§28.8) over behaviour descriptors: the fraction of time each site class touches the
ground, the range of base attitude ‖log R‖, the airborne fraction, and the sliding fraction (tangential speed at
stance contacts above a card threshold). The objective is many distinct high-scoring behaviours, not one optimum.

*Intent invariants.* Past-time predicates in the requirement language of §24.5 over channels Chronicle already
records. A task family declares which it adopts.

| Invariant | Signal |
|---|---|
| Contact only at designated sites (feet, wheels, tracks) | Contact provenance per site |
| Attitude excursion inside a bound | ‖log R‖ of the base |
| Airborne and sliding fractions inside their bounds | Contact state; tangential speed at stance contacts |
| No net positive work by a contact | The `contact-active` monitor of §29.9 |
| The body stays inside the route corridor | The scene's route graph (§19) |
| Dwell: success is held for a declared time, and is not first reached in the last 5% of the horizon (DESIGN) | The verdict's own online signal |

*Findings and their resolution.* A finding is (foil, behaviour cell, score, violated invariant, replay address,
proposed clause), the clause being a typed edit of the `requirements` section. The author accepts the clause, which
yields a new `TaskId` because a task is content-addressed, and the foil's trace is added as `(example … violated)`,
so the amended requirement is tested against the loophole from then on. Or the author marks the behaviour intended,
and the trace is added as `(example … satisfied)`. A high-scoring degenerate behaviour that violates no adopted
invariant is reported `unclassified`. Campaign results cite the `TaskId` they ran under. `eval.qualify` on a task
with an unresolved finding, or with no audit record for its `TaskId`, is refused. The audit runs on discovery worlds
only and reads no confirmation world.

*In-campaign check [S].* The winning episodes of the top-k designs are evaluated against the adopted invariants. A
design whose wins violate one is flagged `wins-outside-intent` and cannot be promoted until the author resolves it.

*Hand case.* Goal region x ∈ [4.9, 5.1] m and requirement `(F (0 s 10 s) in_goal)`. A sled foil moving at a constant
1 m/s is inside the region for t ∈ [4.9, 5.1] s and never stops: the verdict is satisfied with a dwell of 0.2 s. The
proposed clause is `(F (0 s 10 s) (H (0 s 2 s) in_goal))`. The sled's longest dwell is 0.2 s < 2 s, so the amended
requirement is violated by the foil; a body at rest at x = 5 m from t = 6 s satisfies it from t = 8 s.

*Owners and verb.* `frob-arena` owns the audit, the foil library and the invariants; `frob-control` and `frob-opt`
supply the search; findings are stored by `frob-chronicle` keyed by `TaskId`. Verb: `task.audit` (run; budgeted; a
long call returns a job handle).

*Ceiling, pilot and gate.* Ceiling: the compute of campaigns that a loophole would have invalidated. How often
authored tasks contain one is unmeasured. FACT (§2.2, source register): in one published agent design loop 15.8% of
trained candidates were degenerate after a full training run. HYPOTHESIS: at least one accepted finding in a fifth of
authored tasks; test: the count over the task families of §24.3 at M3. Pilot before funding: `task.audit` on ten
authored tasks with a budget of 5% of a reference campaign each. If no finding is accepted by an author, only the
in-campaign check is built.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-ARENA-818 | Known answer; Performance of the method | The hand case exactly. On 30 tasks with one planted loophole each (off-site contact, tumbling, off-route travel, last-instant success, energy-creating contact) and 30 clean tasks: at least 80% of planted loopholes found before any design campaign, at no more than 5% of a reference campaign's compute per task, with findings on at most 20% of the clean tasks | M3 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-FOIL | Detection below 60%; or findings on more than 30% of clean tasks; or the sampling controller on the actual design finds the same loopholes at the same cost, so foil bodies add nothing | The in-campaign check on winning episodes only; `task.audit` runs the actual design alone |
<<<END>>>

<!-- B17 · chapter 24 · tier F · fills: calibrated item bank for the test battery, sampled by importance -->
<<<INSERT chapter=24 title="The item bank: importance sampling of battery items for close comparisons">>>
**Item bank [F].** The cost of a population decision scales with designs × battery items × replicates. Racing and the
ladder work on the first factor and the tiers on the cost of an episode. Nothing so far models the items, although
most items say nothing about two close designs: scenes both pass and scenes both fail. The item bank keeps a
persistent model of items and uses it only to choose which items a comparison runs. The estimand never changes.

*Item, law and bank.* An item is (scene instance, task, disturbance law, family of initial conditions), identified by
content hash. A battery version fixes a law p(i) over its items; p is the estimand's weight and is never edited by the
bank. The bank stores, per morphology class and battery version, a response model fitted by regularized maximum
likelihood on every (design, item, outcome) row of discovery data in the archive, in canonical row order:

    success:      P(Y = 1 | d, i) = σ(a_iᵀ·θ_d − b_i),   θ_d ∈ ℝᵏ, k ≤ 5 (DESIGN)
    continuous:   Y | d, i  ~  N(μ_i + a_iᵀ·θ_d,  s_i²)

The fit has a version, and the version is part of the identity of every comparison that used it.

*Proposal and estimator.* For a contrast of designs A and B with paired differences D_i = Y_B,i − Y_A,i,

    q*(i) ∝ p(i)·√(E_model[D_i²]),      q(i) = (1 − ε)·q*(i) + ε·p(i),      ε = 0.1 (DESIGN),
    Ĉ = (1/n)·Σ_j w(i_j)·D_{i_j},        i_j drawn from q by an addressed stream,   w(i) = p(i)/q(i) ≤ 1/ε.

E_q[w·D] = Σ_i q(i)·(p(i)/q(i))·E[D_i] = Σ_i p(i)·E[D_i]: the estimate is unbiased for the contrast under the frozen
law whether or not the response model is right. The model buys efficiency and nothing else. Items that do not
discriminate are down-weighted in q and never removed from p.

*Arithmetic, exact.* Let D be ±1 on a fraction f of items and 0 elsewhere, with mean μ. Uniform sampling has variance
f − μ². Under q* the weight is f on the discriminating items and the variance is f² − μ². At f = 0.1 and μ = 0.05
these are 0.0975 and 0.0075, a ratio of 13. With the defensive mixture at ε = 0.1 the weight on a discriminating item
is 1/(0.9/f + 0.1) = 0.1099 and the variance is 0.1 × 0.1099 − 0.0025 = 0.0085, a ratio of 11.5.

*Counterexample: a wrong model costs up to 1/ε.* Let the model place q* on a set of items disjoint from the items
that discriminate. Each discriminating item then has q = ε·p and weight 10, the second moment is f × 10 = 1.0, and
the variance is 0.9975 against 0.0975 under uniform sampling: 10.2 times worse, with the estimate still unbiased. The
mixture bounds the damage at 1/ε. It does not remove it, and the gate therefore measures realized interval width
and not the model's own forecast.

*Rules.*

- **Anytime studies on discovery worlds only.** The weighted increment w·D lies in a span 1/ε times the span declared
  for D, so a range-driven sequence would lose by (1/ε)² in sample size. Item-bank sampling is admitted only with the
  variance-adaptive betting sequence, and the span it declares is the widened one.
- **Refused for `fixed-n` studies and inside the blinded qualification lane.** The exact rule of §28.6 counts
  discordant pairs under the frozen law, and confirmation draws from p on fresh sealed clusters. The fit never reads
  a confirmation world.
- **Not composed with the tier-correction estimator.** Promotion pairs are drawn from p; multiplying the two weights
  would allow 10 × 20 = 200.
- **A rare-failure bound refuses item-bank episodes**, as it refuses any episode not drawn nominally (§26).
- **Replicate noise is untouched.** Variance between seeds inside an item is not reduced by choosing items.

*What an agent reads.* Beside the verdict: the episodes used and the uniform-equivalent count, and the separating
items ranked by discrimination ‖a_i‖ with their difficulty b_i, which says in which capability the two designs
differ.

*Owners and verbs.* `frob-arena` owns the battery law and the bank; `frob-opt` owns the proposal and the estimator;
rows come from `frob-chronicle`. `eval.compare --sampling item-bank` runs a comparison; `eval.items` (read) returns
the bank rows for a battery and class with their sample counts and fit version.

*Ceiling and expectation.* The idealized ratio is 1/f when a fraction f of items discriminates. With model error, the
mixture and replicate noise the expectation is 4 to 8 (HYPOTHESIS; test: the replay study below). The saving
multiplies with the tier correction of §29.11, which lowers the cost of an episode instead of their number.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-ARENA-819 | Known answer | The two-value fixture reproduces 0.0975, 0.0075 and 0.0085 in exact arithmetic; under the disjoint wrong model the mean of Ĉ over 10⁴ replays equals the true contrast within its Monte Carlo error and the variance ratio stays at or below 1/ε; a `fixed-n` request with item-bank sampling is refused | M3 |
| GATE-ARENA-820 | Performance of the method | Replay on a recorded full matrix of about 40 designs × 150 items × 4 replicates (24,000 episodes): for the 20 closest pairs, episodes to equal interval width lower by a factor of at least 4; coverage within 2 points of nominal; item parameters fitted on one family predict held-out designs better than per-item means | M3 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-ITEMBANK | At GATE-ARENA-820 the factor is under 2 for the 20 closest pairs; or coverage falls more than 2 points below nominal; or the held-out prediction is no better than per-item means | Stratified sampling by interior archetype with proportional allocation, which needs no model |
<<<END>>>

<!-- B18 · chapter 24 · tier S, F · fills: common randomness compiled from the task; test of what "the same world" means -->
<<<INSERT chapter=24 title="Exposure domains and the coupling test">>>
**Exposure domains [S]; chosen couplings [F].** This extends the `randomization` section of the task file (§24.4),
pairing rule 2 of §28.6 and the keyed streams of §35.9. Those rules say that a draw is keyed by the physical entity
it perturbs. What is still unstated is which kind of entity each draw belongs to, what happens to a disturbance that
a mission event triggers, and how anyone checks that a coupling left each arm's own distribution alone. Every draw
in `randomization` now declares one of four exposure domains, and the task compiler in `frob-arena` lowers the
section to a versioned exposure table that is part of the task's identity.

| Domain | Key, beside `WorldAddr` and `stream_id` | Meaning | Example |
|---|---|---|---|
| `field` | An anchored coordinate cell or a fixture identifier | Sampled once per world at stable coordinates; a robot reads it where it is | Friction of a floor tile; the mass of a cart in the scene |
| `process` | A node of the dyadic tree over physical time (§16.5) | An exogenous process in physical time; a change of step or control rate reads the same path | A gust; a push scheduled at 3 s |
| `event` | A mission role and an occurrence index | Exists only where the task defines the exposure by a declared mission event; its physical time may differ between arms by definition | A push when the base first crosses the docking gate |
| `private` | The hash of a port name | Noise with no counterpart in the other arm | Noise of a sensor that only one design carries |

Roles are the typed ports and task regions of §14 and §19, so they survive edits. An `event` draw is legal only when
the exposure is defined by a mission event in the task; keying by the order of contacts, footfalls or draws remains
refused (§28.6).

*The coupling test.* `task.check --coupling` runs four checks on a task and a pair of controllers or designs. A
coupling that fails one is refused.

1. *Marginal invariance.* For each arm, outcomes under the coupled protocol and under independent sampling agree in
   distribution within a tolerance fixed in advance. A coupling may change the joint law of the two arms and nothing
   about either arm alone.
2. *Insertion invariance.* Adding unused draws, a sensor or a limb changes no exposure of another domain entry (the
   sensor-set property of §20.17, applied to exposures).
3. *Rate invariance.* Halving the physics step changes no `field` value, and a `process` increment over a coarse
   interval equals the sum of its fine increments.
4. *Retiming.* Under a deliberately retimed controller, a `field` stays at its coordinates, a `process` push occurs at
   the same physical time, and an `event` push occurs at each arm's own crossing.

*Counterexamples, hand-checkable.* A push keyed by control tick 1,500 occurs at 3.0 s with a 2 ms step and at 1.5 s
with a 1 ms step. A push keyed by "the tenth footfall" occurs at 5.0 s under a gait period of 0.5 s and at 4.0 s
under 0.4 s: a disturbance meant to be timed now follows gait phase, and the two arms were pushed in different
states. The first keying fails check 3 and the second fails check 4.

*Choosing among legal couplings [F].* Where more than one legal keying exists (friction by tile or by fixture), the
choice is made on pilot addresses by the measured variance of the contrast, the table version is then locked, and
confirmation addresses are drawn only afterwards. Both marginals and the contrast variance are recorded. For equal
marginal variances σ², Var(D) = 2σ²·(1 − ρ): moving the paired correlation from 0.50 to 0.95 lowers the variance of
a contrast ten times at a fixed count. That is an illustration of what is at stake, not a prediction.

*Owners and verb.* `frob-arena` owns the compiler and the test; `frob-scene` caches `field` samples per world; the
stream registry of §35.9 holds the domains. Verb: `task.check --coupling`. A paired study on a task whose exposure
table has not passed the test is refused.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-ARENA-821 | Known answer; Performance of the method | Fixture: a slick strip over x ∈ [2.0, 2.5] m (`field`, per 0.25 m tile), a push at t = 3 s (`process`), a push at the first crossing of a gate at x = 5 m (`event`), a baseline controller and one retimed from a 0.5 s to a 0.4 s gait period. With 1,000 unused draws inserted, the step halved and a limb added: tile values bit-identical; the timed push at 3 s in every arm; the gate push at each arm's crossing; per-arm outcomes indistinguishable from independent sampling in a two-sided test at level 0.01 on 4,096 worlds. The tick-keyed and footfall-keyed controls are refused. On a held-out slip-and-dock battery the variance of the contrast is at most half that of named streams keyed per stream and tick, at equal count | M2 for the fixture; M3 for the battery |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-COUPLING | Under a factor of 2 in contrast variance on the held-out battery; or the automatic choice among couplings picks one that fails the test in any pilot | The keyed streams of §35.9 as they stand, with explicit per-stream adapters; the four checks are kept as a test |
<<<END>>>

<!-- B19 · chapter 25 · tier S · fills: K-070 surviving narrow form "geometric-programming monomial screens" -->
<<<INSERT chapter=25 title="Monomial screens in exponent coordinates">>>
This subsection extends §25.6 ("Monotone bounds first") and the static-exclusion step of the worklist in §25.7. It
excludes a whole design family when a conjunction of sizing relations is contradictory, with a certificate that is
checked in integer arithmetic. Tier [S]. Owners: `frob-mission` for the screen, `frob-cert` for the proof kind and its
checking rule. Verb: `mission.refute` for a family, and the worklist of `mission.synthesize`.

**The object.** Design parameters θ₁, …, θₙ are positive: lengths, masses, ratios, ratings. A monomial constraint is
c · θ₁^a₁ ⋯ θₙ^aₙ ≤ 1 with c a positive rational and integer exponents (rational exponents are cleared by raising the
inequality to a common power). Parameter bounds are monomial constraints: lo/θᵢ ≤ 1 and θᵢ/hi ≤ 1. Many static sizing
relations have this form: holding torque m·g·ℓ ≤ N·τ, no-load speed v·N ≤ ω·ℓ, bus current, pressure against sinkage.

**The certificate.** For constraints k = 1, …, K with coefficients c_k and exponent rows a_k, nonnegative integers
λ_k, not all zero, with

  Σ_k λ_k · a_k = 0   and   Π_k c_k^λ_k > 1

prove that no positive θ satisfies all K constraints: multiplying the constraints raised to the powers λ_k cancels
every parameter and leaves Π_k c_k^λ_k ≤ 1, a contradiction. In the coordinates y = ln θ the constraints are the
linear system a_k · y ≤ −ln c_k, and by the theorem of the alternative such λ exists whenever the system is
infeasible; because the exponent matrix is rational, λ can be taken integer. The search is a linear program in
floating point. Only the certificate is trusted: the checker verifies n integer sums, one per parameter, and
compares one product of integer powers of rationals with 1. It never evaluates a logarithm. Multipliers and bit
sizes have declared caps, and a certificate beyond them is refused, not approximated.

DESIGN: the proof kind `monomial-product` is added to the closed registry of §25.5.2 with class `Certified`, the
checking rule above and an invalid fixture per field. A cell's box bounds enter as constraints, so the same rule
excludes one cell or the whole positive orthant.

**Sums of monomials.** A constraint P(θ) = Σ_j m_j(θ) ≤ 1 with several monomial terms is not linear in y: its
logarithm is a convex log-sum-exp. Two necessary conditions are monomial and may join the system:

- each term alone: m_j ≤ 1;
- a weighted mean inequality: for positive rational weights w_j with Σ w_j = 1 and common denominator D,
  Σ_j m_j ≥ Π_j (m_j/w_j)^w_j, so P ≤ 1 implies Π_j (m_j/w_j)^(D·w_j) ≤ 1.

Any positive weights are sound. Default weights are equal, plus the term shares m_j/P at the centre of the cell,
rounded to denominators of at most 16. The screen only excludes. Inclusion, and cells the screen does not decide, go
to the per-constraint lane of §25.6.

**Known answer: no gear ratio and no leg length helps.** A leg must hold m·g = 12 kg · 9.81 m/s² = 117.72 N at lever
arm ℓ and move the foot at v = 1.5 m/s, through a reduction N, with a motor of torque τ = 2 N·m and speed
ω = 80 rad/s. The two constraints are (117.72/2) · ℓ · N⁻¹ ≤ 1 and (1.5/80) · ℓ⁻¹ · N ≤ 1, with coefficients 2943/50
and 3/160 and exponent rows (1, −1) and (−1, 1). The multipliers λ = (1, 1) cancel both parameters and give
8829/8000 = 1.103625 > 1. No (ℓ, N) exists. The certificate reads as physics: the mission needs
m·g·v = 176.58 W and the motor's corner power τ·ω is 160 W. Control: with τ = 2.5 N·m the product is
17,658/20,000 = 0.8829, no certificate exists, and ℓ = 0.2 m with N = 10 satisfies both constraints (0.94176 and
0.9375).

A per-constraint paving does not see this cheaply. Over ℓ ∈ [0.1, 0.4] m and N ∈ [5, 50] each constraint alone is
satisfiable (0.118 at ℓ = 0.1, N = 50 for the first; 0.234 at ℓ = 0.4, N = 5 for the second). The two feasible sets,
ℓ/N ≤ 0.01699 and ℓ/N ≥ 0.01875, are separated by a diagonal band that boxes approximate only by refinement.

**Counterexample: the term-wise screen is not a decision.** With two terms m₁ = m₂ = 0.6 each term passes and the sum
is 1.2. Equal weights give 4·m₁·m₂ = 1.44 > 1 and exclude it. With m₁ = 0.9 and m₂ = 0.15 the sum is 1.05, equal
weights give 4 · 0.135 = 0.54 and do not exclude, and the weights (6/7, 1/7) give 1.05⁷ ≈ 1.407 > 1 and do. Over a
cell with fixed weights the relaxation is an outer approximation, so an undecided cell says nothing.

**Scope and ceiling.** An exclusion covers the static rigid model named in the bundle and says nothing about dynamic
performance. No speed claim is made: for a single proposed design a pointwise check gives the same answer. The value
is a family exclusion with a one-line certificate and a named binding pair of constraints, which `mission.refute`
reports with the smallest change of a constant that removes the contradiction (here, motor corner power above
176.58 W).

**Gate.** GATE-AUGUR-822 (Known answer, M3). On 1,000 generated monomial systems of at most 12 parameters and 24
constraints with planted status: no feasible system is excluded; every infeasible system whose contradiction has a
product of at least 1.01 and fits the size caps is excluded; every certificate is accepted by `frob-verify`, and each
of 200 mutated certificates (one multiplier or one coefficient changed) is rejected. The leg fixture returns
λ = (1, 1) and 8829/8000; its control returns the witness; the two-term case (0.9, 0.15) is excluded at weights
(6/7, 1/7) and not at equal weights.
<<<END>>>

<!-- B20 · chapter 25 · tier F · fills: bracket what the embodiment can achieve; dominance stop for controller effort -->
<<<INSERT chapter=25 title="Utility brackets and the dominance stop">>>
**Utility brackets [F].** This extends Rules 1 and 2 of §25.4.5 from feasibility to a bounded task utility, and gives
the ladder of §28.9 a sound reason to stop spending controller effort on a design. For a mission whose task declares a
utility u ∈ [0, 1] (a success probability, a normalized payload, a normalized energy margin), a design d and a world
law, a **bracket** is a pair L_d ≤ U*_d ≤ Ū_d, where U*_d is the best value any admitted causal controller achieves.

- *The upper end Ū_d* uses the relaxations of Rule 1: every contact force inside an outer wrench cone, every torque
  under the card envelope, the energy budget of A-20 with every admitted energy source, reach and deadline limits, and
  any observation a controller could wish for. Per scenario the relaxation is optimized for u instead of tested for
  feasibility, and Ū_d(w) is the outward-rounded optimum with its dual as certificate. Over a world law, the mean of
  Ū_d(w) on the sampled scenarios carries an upper confidence bound.
- *The lower end L_d* is achieved performance: rollouts of one frozen controller on confirmation addresses that its
  development never read, with a lower confidence bound. A stronger controller raises L_d and leaves Ū_d valid.

With m the comparison's margin and u_req the requirement threshold, four labels follow.

| Condition | Label | Consequence |
|---|---|---|
| Ū_d < u_req | `body-limited` | An exclusion under Rule 1, with its certificate |
| Ū_d − L_d ≤ m | `saturated` | More controller effort cannot change a decision at this margin |
| Ū_d − L_d > m, and Ū_d at or above the k-th best lower end | `controller-limited` | Effort belongs on the controller; the design escalates (the budget curves of §28) |
| Ū_d < L_c for a competitor c | `dominated` | **The dominance stop**: d receives no further training, fine-tuning or planner budget in this comparison |

Only the dominance stop removes work, and it is sound only when both bounds hold at once. With M designs compared at a
study level α, each one-sided bound is taken at level α/(2M) (DESIGN: a union bound; a design's bounds across
scenarios share one world sample). The stop ends effort inside one comparison. It is not an exclusion from O: a
dominated design may still satisfy the mission.

*Worked case.* A carrier of body mass 10 kg lifts a payload p through h = 2 m on a ramp with 300 J of usable stored
energy at the foot, and the mission admits arrival at the foot at up to 1.5 m/s. Utility is p divided by 10 kg. Every
controller obeys (10 + p)·(g·h − v²/2) ≤ 300 J, so 10 + p ≤ 300/(19.62 − 1.125) = 16.22 kg and Ū = 0.622. A
relaxation that leaves out the approach energy gives 10 + p ≤ 300/19.62 = 15.29 kg and Ū = 0.529. Against a
competitor with L_c = 0.60 the incomplete relaxation issues a dominance stop, and it is false: p = 6 kg is feasible
with a run-up, because 16 × 9.81 × 2 = 313.92 J ≤ 300 + ½·16·1.5² = 318 J. The sound bound 0.622 does not stop the
design. This is the utility form of the sill case of Rule 1, and the incomplete relaxation is the seeded defect the
gate must catch.

*Honest limits.* A bound over all controllers is only as tight as its relaxation. For contact-rich full-order tasks
it is often vacuous (Ū = 1); the bracket then says nothing and has cost one small program. Whole-body certifying
relaxations are not built here. The value concentrates in briefs limited by energy, torque, reach or a deadline.

*Ceiling.* The stop removes controller-stage work only. If a fraction x of candidates is dominated before training
and computing the bounds costs a fraction y of baseline controller work, the stage shrinks by 1/((1 − x) + y): 10 at
x = 0.93 and y = 0.03, and 1.4 at x = 0.3 (illustrative). The end-to-end gain is capped by the controller stage's
share of the phase ledger (§2.2).

*Owners and verbs.* `frob-mission` produces both ends as Rule 1 certificates over the relaxations of `frob-analyze`;
`frob-opt` applies the labels and the stop in the ladder. `mission.refute` returns Ū_d with its binding constraint.
`eval.compare --bracket` returns both ends, the label and, for a dominated design, the competitor and both
certificates. A request to stop on a bound whose relaxation does not enclose a declared state or energy source is
refused, under the existing rule for exclusions (§28.4).

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-AUGUR-823 | Known answer; Performance of the method | On an exhaustively solvable planar ramp-and-carrier family (states and controls discretized so the optimum of each of at least 500 design and scenario pairs is known by dynamic programming): every Ū at or above the optimum, every L at or below it, zero false dominance stops; the run-up candidate is not stopped and the relaxation without approach energy is rejected. On a held-out brief limited by energy or torque: controller effort per accepted decision at least 3 times lower than static filters followed by the ladder, at matched decisions | M4 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-AUGUR-BRACKET | One false stop on the solvable family or in an audited campaign (the audit of §28.12 samples dominated designs and trains them); or less than 3 times at matched decisions; or the label changes the agent's next action in under a tenth of comparisons | Brackets are printed as advisory labels, no stop is automatic, and designs leave the ladder only by the ladder's own rules |
<<<END>>>

<!-- B21 · chapter 25 · tier F · fills: inverse-dynamics convex synthesis via the linear dependence on inertial parameters -->
<<<INSERT chapter=25 title="Placement and sizing along a prescribed motion">>>
**A fifth constructive lane [F].** For a fixed topology with fixed joint axes and link lengths, the inverse dynamics
of a prescribed motion is linear in the inertial parameters (C-03, C-04). At knot k of a motion (q_k, q̇_k, q̈_k) with
active contacts C_k,

    Y(q_k, q̇_k, q̈_k)·π = Sᵀ·τ_k + Σ_{c ∈ C_k} J_c(q_k)ᵀ·λ_{c,k},

where π stacks the ten inertial parameters of every body, S selects the actuated coordinates, and Y is the regressor
that `frob-kir` emits from the derived inverse-dynamics kernel. The lane (`mission.synthesize --strategy placement`,
owned by `frob-synth`) takes a small library of nominal motions for the archetype (from the gait atlas, the skill
artifacts and centroidal planning) and solves one cone program per motion and per catalog choice, over:

- **inertia**: π = π_fixed + Σ_s m_s·φ_s with m_s ≥ 0, where φ_s is the parameter vector of a unit mass at an
  admissible site s (battery cells, compute, ballast), and, for bodies whose distribution is free, the pseudo-inertia
  cone and the mass-support bound of C-03;
- **contact forces** λ_{c,k} inside the friction cone of the scene's interface card at that contact;
- **joint torques** τ_k, tied to the others by the equation above, under the cards of §15 at fixed gear ratios N:
  peak |τ_{j,k}| ≤ η·N·τ_peak; thermal ‖τ_j‖₂ ≤ √K·η·N·τ_cont over the K knots; voltage
  R·|τ_{j,k}|/(η·N·k_t) + k_e·N·|q̇_{j,k}| ≤ V_bus, linear because q̇ is data;
- **objective**: electrical energy Σ_j Σ_k [R·(τ_{j,k}/(η·N·k_t))² + τ_{j,k}·q̇_{j,k}]·Δt, a convex quadratic.

The solver is the one cone solver of §22.5, on the CPU; the certificate kinds are `lp-primal`, `lp-farkas` and
`conic-farkas` of §25.5. Three results come back: a placement and sizing that realizes the motion at least energy; a
lower bound on electrical energy *for that motion*; or a dual certificate that no placement and no admissible contact
forces realize it.

*What the certificate says, and what it does not.* The statement excluded is the pair (design cell, motion). A design
that completes the mission by a different motion is untouched. The lane therefore proposes candidates and prunes
motions; it removes a cell from O only when the mission itself prescribes the motion (a declared lift profile, a
commanded acceleration). A count of library motions a design can realize is a screening signal under rule 4 of §28.4.

*Three limits, each with its smallest case.*

1. *A continuous site enters quadratically.* A mass m at position x contributes m·x to the first moments and m·x² to
   the second. The program is linear in masses at fixed sites, not in a position. A splittable mass (cells along a
   rail) is exact; for one unsplittable module the program is a relaxation whose infeasibility still excludes every
   single-site placement, and a fractional optimum is rounded and re-solved.
2. *Energy is not jointly convex in torque and gear ratio.* With N a variable the peak, thermal and voltage limits
   stay convex (the voltage limit times N reads c₁·|τ| + k_e·|q̇|·N² ≤ V_bus·N), so feasibility over a continuous
   ratio is one cone program. The copper-loss term g(τ, N) = τ²/N² is not convex: g(0, 1) = 0, g(2, 3) = 4/9 and at
   the midpoint g(1, 2) = 1/4 > 2/9. Minimum energy therefore enumerates catalog ratios, one program each.
3. *Link lengths are not variables.* Y depends on them nonlinearly. They are enumerated on the cells of the query.

*Known answer.* A rear-drive cart with wheelbase 1 m and centre of mass at floor height: 15 kg fixed at 0.5 m from
the rear axle and a 5 kg battery at x_b on a rail. The rear normal force is g·[7.5 + 5·(1 − x_b)] and traction needs
μ·N_r ≥ 20·a. With μ = 1/2 and g = 981/100 m/s²: at a = 3 m/s² the motion is feasible exactly when x_b ≤ 35/654 m
(53.5 mm); the largest acceleration over all placements is 0.5 × 9.81 × 12.5/20 = 3.066 m/s²; at a = 4 m/s² the
program is infeasible for every placement, with the traction inequality as its dual. The same 4 m/s² motion is
feasible with all wheels driven (μ·g = 4.905 m/s²), and the original cart reaches the same goal by a slower motion:
the certificate excludes a motion, not the cart.

*Pilot, ceiling and gate.* Ceiling: the lane decides only the share of a design decision that is placement and
sizing for known motions, and it starts a search; it does not replace one. A conventional optimizer over a dynamics
library can pose the same program, so this is a convenience of the typed core and not a differentiator. Pilot before
funding: the cart and one quadruped trot with a movable battery, two engineer-days on the CPU.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-AUGUR-824 | Known answer; Design comparison | The cart: boundary 35/654 m in exact rationals, the dual at 4 m/s² accepted by `frob-verify`. The regressor identity Y·π equals the derived inverse dynamics within 10⁻¹² relative on 10⁴ random states per reference robot. A request for jointly optimal energy and ratio is routed to enumeration. On 30 designs and two missions, candidates initialized by the lane beat their class templates by at least 15% in electrical cost of transport in closed-loop evaluation at a 95% paired lower bound, or start within 10% of the optimum that forward co-design reaches; the share of designs infeasible on every library motion that still pass the mission is reported | M4 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-SYNTH-REGRESSOR | The design-comparison part of GATE-AUGUR-824 fails; or more than 15% of designs infeasible on every library motion pass the mission in closed loop, in which case the motion-count screening signal is withdrawn first | The mass-placement program of §25.21 and A-24 as they stand; candidates start from class templates |
<<<END>>>

<!-- B22 · chapter 25 · tier F · fills: adjudication H-06 (coupled constraints can admit large boxes; the six-parameter cap is on the split set) -->
<<<INSERT chapter=25 title="Weak-variable enclosure and the split set">>>
**Boxes of any dimension with a bounded split set [F].** This extends §25.6 and corrects the reading of
ADR-AUGUR-06 under which a certificate's box has at most about six parameters. What is bounded is the number of
parameters that are bisected, not the number the constraint reads. A constraint may depend on every parameter of
a family and still be decided over most of the box, when most of that dependence is weak.

*Definitions.* For a lowered constraint g(x) ≤ 0 on a box B ⊂ ℝⁿ, an affine-form enclosure gives
g(x) ∈ g₀ + Σᵢ aᵢ·εᵢ + r·[−1, 1] with εᵢ ∈ [−1, 1]. Parameter i contributes width 2·|aᵢ| and the nonlinear
remainder contributes 2·r. Order the parameters by |aᵢ|. The **split set** S is the leading part of that order,
capped at six (DESIGN: the cap of ADR-AUGUR-06). The other parameters are **weak**, and the **weak width** is
ω = 2·(Σ_{i ∉ S} |aᵢ| + r) with every weak parameter at its full range.

*Algorithm.* A queue holds sub-boxes that differ from B only in the coordinates of S.

1. Enclose g over the sub-box with every weak parameter at full range. Upper end at most 0: inner. Lower end above
   0: excluded. Either result is an `interval-box` or `taylor-box` certificate whose box is the full n-dimensional
   sub-box. The checker's work is linear in n per box.
2. Otherwise bisect along the parameter of S with the largest current contribution. The enclosure is linearized
   again on each sub-box, so the aᵢ are local and the order may change.
3. Stop splitting a sub-box when the width contributed by S falls below ω. No further split inside S can decide
   it, because the weak parameters alone straddle the threshold. The cell is `unresolved` with the reason
   `weak-variable width`, and the report names the weak parameter with the largest |aᵢ| as the next candidate.
4. The budget is the box count of §25.6, per constraint.

Combination across constraints is unchanged: a design is inner when it lies in every constraint's inner set. Each
set is now stored over the whole box with its split set recorded.

*Known answer.* On [0, 1]²⁰ let m = (1/19)·Σ_{i=2..20} (2·xᵢ − 1) and δ = 0.01·m², so 0 ≤ δ ≤ 0.01. Require
g₁ = x₁ + δ − 0.8 ≤ 0 and g₂ = 0.2 + δ − x₁ ≤ 0. Both constraints read all 20 parameters and neither is monotone
in x₂ to x₂₀. With S = {x₁} the weak width is 0.01. The slab x₁ < 0.2 is excluded by g₂; 0.21 ≤ x₁ ≤ 0.79 is
inner for both; x₁ > 0.8 is excluded by g₁; the slabs [0.2, 0.21) and (0.79, 0.8] are unresolved. Five slabs
decide 0.98 of the volume: 0.58 inner and 0.40 excluded. An isotropic paving of the same box with the budget of
§25.6 (10⁴ boxes, 13 bisections, less than one cut per axis) decides almost none of it.

*Negative control.* g = Σ_{i=1..20} xᵢ − 10 ≤ 0 on the same box. Every parameter contributes the same width, 1.
With any six parameters in S the weak width is 14 and the sum over S ranges over [0, 6]. An inner cell would need
that sum to be at most −4 and an excluded cell would need it above 10; neither can occur. Stopping rule 3 fires at
once, since 6 < 14. The mechanism must return `unresolved` for the whole box after one enclosure and no
bisection, although half the volume is feasible by symmetry. A run that bisects here burns budget, and a run that
returns an inner or excluded cell is unsound. The exact answer to this constraint belongs to the monotone rule of
§25.6. That is the point of the control: the split-set rule is for dependence that is weak, not for strong
dependence spread evenly.

*Scope and ceiling.* The mechanism decides volume only where the weak width is small against the constraint's
margin. HYPOTHESIS: for the static constraints of legged and wheeled classes, the parameters of other limbs and
of distal bodies enter one leg's torque, reach and clearance constraints weakly, through posture and load share.
OPEN-AUGUR-3 measures the constraint-to-parameter graph and now records |aᵢ| per parameter, not only incidence.
Ceiling: no speed claim. Decided volume is evidence quality and tolerance robustness (§25.6); the number that
matters is candidate evaluations avoided.

*Owner and verbs.* `frob-mission` owns the driver and the choice of S; enclosures come from the interval and
Taylor-model arithmetic behind the airlock; certificates are of the existing kinds. `mission.refute` and
`mission.synthesize` take `--split-cap k` (default 6) and report, per constraint, S, ω and the three volume shares.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-AUGUR-825 | Known answer; Performance of the method | The 20-parameter case returns five slabs with shares 0.58, 0.40 and 0.02, and `frob-verify` accepts every certificate. The negative control returns `unresolved` for the whole box with one enclosure evaluation and no bisection. On the 12-parameter static quadruped family of §25.6, against per-constraint decomposition restricted to constraints that read at most six parameters and at the same box budget: at least 10 percentage points more decided volume, or at least 10% more candidate evaluations avoided, with no certificate rejected | M4 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-WEAKVAR | The comparison of the gate fails; or more than 90% of the volume of constraints that read more than six parameters stays unresolved at budget on the reference families | ADR-AUGUR-06 as first written: a constraint that reads more than six parameters receives no region certificate and is checked pointwise |
<<<END>>>

<!-- B23 · chapter 27 · tier F · fills: adjudication H-08 (the double-precision mandate; attribution on the originating tier) -->
<<<INSERT chapter=27 title="Attribution in single precision on the originating tier">>>
**The single-precision lane [F].** This amends ADR-AUTOPSY-04 and §27.9. Double precision on the CPU remains the
default for one failure, where it costs little. It is not a requirement of the evidence. What the evidence
supports is a flip rate measured under perturbations at or above round-off scale (§27.5), and that measurement can
be made on the tier where the failure happened.

*Why it is admissible.* FACT arithmetic on the exponent quoted in §27.9, 0.16 per control step: the growth factor
e^(0.16·n) is 24.5 at n = 20, 602 at n = 40 and 3.6 × 10⁵ at n = 80. Single-precision round-off of 6 × 10⁻⁸
therefore reaches 1.5 × 10⁻⁶ of state scale after 20 steps, 3.6 × 10⁻⁵ after 40 and 2% after 80; at that
benchmark's 25 ms period these are fork windows of 0.5 s, 1 s and 2 s. A verdict that moves under a relative
change of 10⁻⁵ in state also moves under sensor noise and friction scatter two orders larger. It is marginal in
any arithmetic, and the right output is the label `marginal`, not more digits. HYPOTHESIS: the exponent of our
closed loops is of the same order. It was measured on walker and cheetah benchmarks under learned policies, and
the gate below measures ours.

*Procedure* for a failure recorded on a single-precision tier.

1. The null fork on the originating profile must give identical digests (unchanged, INV-AUTOPSY-01).
2. R₀ placebo forks at a relative 10⁻⁷ on the originating tier over the fork window W (DESIGN default R₀ = 32).
   Record the placebo flip rate ρ₀ and the measured growth G(W) of the separation between placebo forks and parent.
3. *Admission.* ρ₀ ≤ 0.1 and G(W)·u ≤ 10⁻⁴ of state scale, with u = 6 × 10⁻⁸. Attribution then runs on the
   originating tier in single precision with at least 16 replicates per intervention at 10⁻⁷, contrasts tested
   against the placebo as in §27.6, and the record carries `tier: <backend>, precision: f32`.
4. *Not admitted.* Shorten W to the longest window that passes, within the bound the first-divergence algorithm of
   §27.10 gives on how late a fork may start, or escalate to double precision as in §27.9.
5. *A failure that double precision does not reproduce* is no longer refused outright. It is attributed in this
   lane with the numerical-variant interventions first: half step, doubled solver iterations, the alternative
   contact law, the stricter step profile. A rescue by one of them gives the label `tier-specific: <variant>` and
   passes the world to the artifact guard of §26.6. Only when the lane is also not admitted is the failure
   labelled `precision-sensitive` and attribution refused.

*A cross-check that can fail.* One attribution in k (DESIGN default k = 8), and every attribution that enters a
report, is repeated in double precision on `batch-cpu`; the top factor, its label and the sign of each contrast
are compared, and the agreement rate is a ledger figure.

*Known answers.* (i) The linear map x ← 1.1735·x has exponent ln 1.1735 = 0.16 per step: after 40 steps a
perturbation has grown by 602, and the measured G must match within 1%. (ii) Negative control: a pendulum released
5 × 10⁻⁸ rad from upright with no control, with the verdict "falls to the positive side". A placebo of 10⁻⁷ rad
flips the side whenever its sign is negative, so ρ₀ is about one half; the lane must not admit the case, and the
label is `marginal`. At a placebo of 10⁻¹² the side is stable, which is why the 10⁻¹² series alone would have
hidden the problem.

*Cost and ceiling.* One failure with ten factors and 16 replicates is 160 forks of 2 s: 320 world-seconds,
seconds on either tier, and the lane saves nothing. A cluster of 200 failures with pair interactions (55
interventions × 32 replicates × 2 s) is 7.0 × 10⁵ world-seconds: about 3 minutes on a device producing 4,000
world-seconds per second, against 12 minutes to 2 hours on `batch-cpu` at 1,000 to 100. Ceiling: the wall time of
bulk attribution, by the ratio of device to CPU world-seconds; and attribution of tier-specific failures, which
the double-precision rule could not attribute at all.

*Owner and verb.* `frob-autopsy`. `autopsy.attribute --lane {f64, tier-f32, auto}`; `auto` applies the admission
rule and records which lane ran and why.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-AUTOPSY-826 | Known answer; Performance of the method | The linear map and the pendulum control behave as stated. G(W) is recorded for the reference quadruped and humanoid policies at 0.5 s, 1 s and 2 s. On the planted-cause suite of GATE-AUTOPSY-03 with windows of at most 1 s, the lane names the same top factor as double precision in at least 95% of admitted cases. A planted solver artifact that double precision does not reproduce is labelled `tier-specific` with the right variant. Bulk attribution of 200 failures is at least 3 times faster in wall time than on `batch-cpu` | M3 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-ENSEMBLE | Agreement with double precision below 90%; or fewer than half of the reference failures admitted at any window of 0.5 s or more; or a bulk speedup below 3 | ADR-AUTOPSY-04 as first written: attribution in double precision only, and a failure not reproduced there goes to the artifact guard unattributed |
<<<END>>>

<!-- B24 · chapter 28 · tier F · fills: achievable-performance ranking from controller-budget curves -->
<<<INSERT chapter=28 title="Controller-budget curves and the controller-limited label">>>
**Controller-budget curves [F].** Every cheap rung of §28.9 scores a body together with a controller of fixed
competence: the sampling controller at one sample count, a family policy with no adaptation, a fine-tune at one
fraction of specialist steps. The score ranks body × controller, and a design that needs more controller effort to
show what it can do is under-scored. §28.10 corrects this by region after the fact. The mechanism here measures it per
design before the design is discarded.

*The budget coordinate.* `ControllerId` (§22) gains a budget coordinate b on a registered ladder b₀ < b₁ < b₂ with a
scalar effort c(b). For `mpc-sample`, c = N_s·N_it·H model steps per tick, with the default ladder N_s ∈ {64, 256,
1,024} (DESIGN). For a learned policy, c is the fine-tuning fraction of specialist steps, default {0, 0.05, 0.2},
which are the R1 and R2 rungs already defined. A row is keyed by (design, world address, budget). Every budget runs on
the same addresses, so differences across budgets are paired.

*The curve.* For design d in a morphology class,

    J_d(b) = J_d∞ − a_d·(c(b)/c(b₀))^(−γ),     a_d ≥ 0,  γ > 0 shared by the class,  J_d non-decreasing in b.

For fixed γ the law is linear in (J_d∞, a_d): a two-parameter least-squares fit per design, with one residual degree
of freedom at three budgets, and γ the minimizer of the pooled residual over the class. On a ladder with a constant
effort ratio the three-point form is closed. With Δ₁ = J(b₁) − J(b₀), Δ₂ = J(b₂) − J(b₁) and ρ = Δ₂/Δ₁,

    J∞ = J(b₀) + Δ₁/(1 − ρ),      saturation index  s_d = (J(b_max) − J(b₀)) / (J_d∞ − J(b₀)).

The saturation index is the share of the predicted gain already realized at the highest budget reached.

*Worked case, illustrative.* Design A scores 0.61, 0.70, 0.72 at the three budgets and design B scores 0.55, 0.68,
0.76. Then ρ_A = 0.02/0.09 = 0.222, J_A∞ = 0.61 + 0.09/0.778 = 0.726 and s_A = 0.11/0.116 = 0.95; ρ_B = 0.08/0.13 =
0.615, J_B∞ = 0.55 + 0.13/0.385 = 0.888 and s_B = 0.21/0.338 = 0.62. The fixed-budget verdict at b₀ prefers A by 0.06
and at b₂ prefers B by 0.04: the curves cross between b₁ and b₂. A is saturated; B is limited by its controller.

*What the estimate may do.* J_d∞ is `Estimated`. It steers allocation and never enters a verdict field, under the
same invariant that binds value-of-information quantities (§28.21). Its interval is not the standard error of the
fit: it is the conformal margin of §28.9 for the predictor ĝ = J_d∞, calibrated on audit-lane designs that were
scored at the specialist rung. Where no calibration exists the interval is the metric's declared span, and the rule
below reduces to the fixed table. The measured contrast at the highest budget reached goes through §28.6 and DC-01
unchanged and is the only quantity that can become `established`.

*Escalation rule.* Every candidate runs at b₀. A design moves to the next budget when the upper end of its interval
for J_d∞ reaches the k-th best measured score of its stratum. A design whose fit is refused is treated as unsaturated
and escalates. A design is flagged `controller-limited` when J_d∞ − J_d(b_max) exceeds the decision margin of the
study; campaign reports print the flag beside the rank, and a `controller-limited` design is never described as
inferior. Cost, illustrative: with an effort ratio of 4 and escalation of 30% of designs to b₁ and 10% to b₂, the
mean cost per design is 1 + 0.3 × 4 + 0.1 × 16 = 3.8 units of b₀, against 16 for running every design at b₂. On the
quadruped row of §22.8 (0.33 GPU-minutes per episode at 2,048 samples; cost proportional to the sample count) a
design escalated through all three budgets on three addresses costs 3 × (64 + 256 + 1,024)/2,048 × 0.33 = 0.65
GPU-minutes (DESIGN arithmetic on that FACT), inside the three-episode screening allowance of §22.8.

*Refusals of the fit.* The fit is refused and the design labelled `curve-unfit` when its scores decrease by more than
paired noise, when Δ₂ ≥ Δ₁ (ρ ≥ 1: the curve is still accelerating), or when Δ₁ is within noise of zero.
Counterexample: a body that needs a threshold budget to find any gait scores 0.20, 0.20, 0.80. The two lower budgets
extrapolate to J∞ = 0.20, wrong by at least 0.60, and the three-point form divides by zero. A saturating law does not
describe a threshold. Such a design is escalated and is never discarded on its curve.

*Owners and verbs.* `frob-control` owns the budget coordinate; `frob-opt` owns the fit, the index and the escalation
rule; rows are stored by `frob-chronicle`. `eval.compare --budget-curve` returns, per design, the scores by budget,
J∞ with its interval, the saturation index and the flag. `opt.cascade` applies the escalation rule at stage 3, and
`explain.compare` prints both curves.

*Ceiling, pilot and gates.* Ceiling, stated before any test: the rule cannot rank better than running every design at
the top budget. Its benefit is the compute between that and the escalation rule, at most c(b₂) divided by the mean
cost (16/3.8 = 4.2 in the illustration), and it is nothing when no two curves cross. Pilot before the work package is
funded, on `batch-cpu`: 20 variants from two templates at N_s ∈ {32, 128, 512} on shared addresses. If no two curves
cross, rankings do not depend on budget, the mechanism is not built, and §22.8 is the better for it. If more than a
fifth of the curves are refused, the law is wrong for this controller.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-ATEL-827 | Known answer | On 10³ synthetic designs drawn from the law with planted (J∞, a, γ) and paired noise, the calibrated interval covers the planted J∞ at a rate of at least 1 − α − 0.03; the worked case returns 0.726, 0.888, 0.95 and 0.62; the threshold case is refused | M3 |
| GATE-ATEL-828 | Performance of the method | On 30 designs in three classes with an oracle (a specialist per design over five seeds, and the sampling controller at 8,192 samples): top-3 regret at equal total compute at most half that of ranking at the base budget; J∞ closer to the oracle than J(b_max) for at least half the designs | M4 |

| Kill gate | Trigger | Fallback |
|---|---|---|
| ✖ KILL-BUDGETCURVE | The pilot shows no crossing; or at GATE-ATEL-828 regret at equal compute falls by less than 30%; or J∞ is further from the oracle than J(b_max) for more than half the designs | Fixed-budget scores, with the saturation index printed as a description and no escalation rule |
<<<END>>>

<!-- B25 · chapter 28 · tier S · fills: adjudication H-09 (exact reuse, carried evidence and avoided reacquisition are different quantities) -->
<<<INSERT chapter=28 title="The reuse ledger: three columns, never summed">>>
**Three kinds of reuse, reported apart [S].** This extends §28.13 and §28.14. "Reuse after an edit" has meant
three different things. Adding them, or reporting one under another's name, overstates what a cache does.

| Column | Definition | What it is evidence of |
|---|---|---|
| Exact reuse | Compute whose result is taken unchanged from the memo graph because no dependency changed: analyses, step plans, kernels, scene compiles, rollouts of untouched designs | The memo graph (§28.14) |
| Evidence carried forward | Verdicts that `result.revalidate` keeps without recomputation, each with the dependency receipt that justifies it | The revalidation rule (§28.13) |
| Reacquisition avoided | Training or optimization steps not spent because a child started from a parent's policy, orbit or solution, measured against a from-scratch control stopped at the same return | Inheritance and warm starts (§23.12, §22.9) |

Each column is a share of one declared estimand: **total work per accepted child decision**, in GPU-seconds and
CPU-seconds, priced cold and warm separately. The third column is counted here and nowhere else; the phase ledger
of §39 does not credit it again to inheritance. A reuse figure printed without its column name is refused in a
report.

*Arithmetic the ledger must reproduce.* With rollouts and training at nine tenths of per-design compute, exact
reuse after a dynamics-affecting edit is at most about one tenth. A kernel compile of 95.91 s beside 240 s of
training is 28.6% of a cold run, and the cached 0.89 s is 0.37% of a warm one (FACT, Appendix H), so a cold figure
flatters a cache. If a warm start avoids a fraction a of training, the third column is 0.9·a: 72% at a = 0.8. An
edit therefore "reuses 70% of prior work" only through the third column, and only if inheritance holds for that
class of edit (§23.12).

*Owner and verbs.* `frob-chronicle` keeps the ledger and `frob-opt` runs the from-scratch control. `design.diff
--reuse` and `report.campaign` print the three columns per edit class: mass, mount, controller-only, mission-only,
topology.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-ATEL-829 | Known answer; Performance | On one frozen lineage of 30 edits spanning the five classes, the three columns are reported cold and warm, with the from-scratch control run for at least one edit per class. The columns plus recomputed work sum to the measured total within 2%. A controller-only edit shows exact reuse of every model analysis, and a mass edit shows exact reuse of none of its rollouts | M3 |
<<<END>>>

<!-- B26 · chapter 31 · tier S · fills: K-096 build packet, the hidden-brief review gate preferred in the catalog -->
<<<INSERT chapter=31 title="Packet review as an imported record">>>
This subsection extends §31.12 ("Review is an import lane") and keeps ADR-FOUNDRY-11: no convergence gate depends on a
reviewer. It gives the import lane a protocol and a record format, so that an imported review means the same thing for
every packet, and it separates what a program can check from what only a person can. Tier [S]. Owner: `frob-foundry`.
Verb: `foundry.check`, which runs the mechanical pre-review and, with `--review-record <file>`, imports a record.

**Hidden briefs.** DESIGN: a set of 12 written briefs (TARGET), authored by someone who has not seen the development
fixtures of the packet pipeline. Each states a mission, an approved parts catalogue, a budget, the interfaces and the
deliverables. The digest of the set is committed before the pipeline version under test is frozen, and the briefs are
opened only to the frozen pipeline. A brief is used once and then retired to the development corpus, by the rule that
governs confirmation clusters in §24.13.

**Unattended production.** Each brief runs from mission query to design, qualification verdict and packet with no
human edit in between; attempts and refusals stay in the record. A brief the pipeline cannot satisfy must end in a
typed refusal that names the binding constraint. A correct refusal is a valid outcome and is reviewed as one.

**Mechanical pre-review** (`foundry.check <packet>`), each item pass or fail with the offending file:

| Item | Check |
|---|---|
| M1 | Schema, every digest, the four flags present and false (§31.12) |
| M2 | Cross-export identity: mass, centre of mass and inertia agree within 10⁻⁶ relative (TARGET) among the model the evidence used, `mass.json`, the values recomputed from each of the four geometry exports, and the URDF and MJCF exports of §14.12, with units checked |
| M3 | Referential integrity: every joint has an actuator card, every net has two ends, every made part has its geometry files, every purchased part resolves to a card |
| M4 | Every number carries a claim class and an evidence reference that resolves in the artifact graph (§29.13) |
| M5 | The cited qualification verdict is for this `DesignId` and `ModelId` |
| M6 | Every snapped part was revalidated after the snap |

**The review record** (schema `frob.review/1`), a sidecar addressed by the packet's manifest digest:

```json
{ "schema": "frob.review/1", "packet": "manifest digest", "brief": "brief digest or null",
  "reviewer": { "independent": true, "role": "mechanical designer" },
  "rubric": { "R1_producibility": "pass", "R2_assembly_and_tool_access": "fail",
              "R3_harness_and_service": "pass", "R4_quotable_from_packet_alone": "pass",
              "R5_evidence_legible_and_not_overstated": "pass" },
  "notes": [{ "item": "R2", "part": "bracket-b", "text": "second fastener is enclosed after step 3" }],
  "outcome": "accept-with-changes", "minutes": 55, "date": "YYYY-MM-DD" }
```

`independent` means the reviewer took no part in developing the pipeline or the design. Importing a record changes no
byte of the packet; the manifest view then shows `review` as the record's outcome with the record digest in place of
`no-claim`. A record whose packet digest does not match is refused.

**What is gated and what is recorded.** The gate below covers the mechanical half only. The human half is a recorded
count with a TARGET: at least 11 of the 12 packets receive `accept` or `accept-with-changes`. The count, the reviewer
minutes and the list of rubric failures are printed in the release rubric (§47). While no record exists the count
reads `no-claim`, and no text may describe a packet as reviewed.

**Counterexample: a mechanical pass is not a review.** A two-part bracket whose second fastener sits behind the first
part once it is mounted. Each part passes the tool-access check of §31.10 as a part, the packet passes M1 to M6, and
the assembly cannot be completed in the listed order. Rubric item R2 exists for this case, which is why a mechanical
pass is never rendered as `review: accept`.

**Known answers.** The tube of §31.18 (0.0950 kg) is reproduced from each geometry export within 10⁻⁶. A surface file
written in millimetres and read as metres has 10⁹ times the volume; M2 reports the ratio and names the file. A packet
that cites a verdict for the pre-snap design fails M5.

**Ceiling, stated before any time comparison.** No time claim is made. FACT: no reference packet has been timed by
this project, so a figure such as hours for a scripted reference against minutes for the pipeline would be
illustrative. Time from brief to accepted packet is measured only inside the protocol of §39, in a stratum that
includes packets, with reviewer minutes inside the clock.

**Gate.** GATE-FOUNDRY-830 (Known answer, M5). On the 12 hidden briefs every outcome is a packet that passes M1 to M6
or a typed refusal naming a binding constraint (12 of 12); no cross-export difference exceeds 10⁻⁶; the planted unit
error and the planted stale verdict are each caught in 100 of 100 runs; importing a record leaves the packet digest
unchanged; a record with a wrong packet digest is refused. The fail-closed default without a reviewer is
`review: no-claim`, and KILL-PACKET (§31.19) is unchanged.
<<<END>>>

<!-- B27 · chapter 31 · tier S · fills: referee-tier loads for the structural finalist lane -->
<<<INSERT chapter=31 title="Referee re-extraction of load cases that enter a build packet">>>
**Load cases are promoted like any other result [S].** This adds a seventh rule to §31.6. A load campaign runs on a
fast tier, because it needs every world traced and many episodes. Peak contact and joint wrenches are exactly the
quantities a compliant fast-tier contact gets wrong, and rule 3 of §31.6 bounds only the part of the error that moves
with contact stiffness. A load case is therefore a fast-tier result under the trust ladder of §16.8, and it is
`provisional` until promoted.

7. **Envelope cases are re-extracted on the referee before they enter a packet.** For each envelope case, the world
   and instant that produced it are replayed on the referee from the nearest snapshot (the teleport of §29.8) over a
   window of 0.5 s on either side (DESIGN), and the port wrenches and body field are read again at the instant of
   the referee's own maximum projection inside the window. The case then carries both values, the tier pair and one
   of three labels: `established` when the two agree inside the fidelity envelope for wrenches, with the referee's
   value used; `tier-sensitive` when they do not, in which case the referee's value replaces the fast one and the
   direction is searched again among the neighbouring fast-tier samples; `provisional: referee-unavailable` when the
   referee cannot complete the scenario (§29.11), in which case the `strict` profile of `batch-cpu` adjudicates and
   the label names it.

A build-packet request that contains a `provisional` envelope case is refused; an exploratory structure run may use
provisional cases and is labelled, as §31.3 already requires. Fatigue spectra are checked the same way on a keyed
sample of windows: the ratio of referee to fast-tier stress range at each gauge is recorded, and a spectrum whose
ratio leaves the band declared in the packet is rescaled to the referee's range before damage is summed.

*Why a small tier bias matters here.* Cumulative damage scales as stress range to the power m of the material card's
S-N law. A load that is 10% high gives 1.1⁵ = 1.61 times the damage at m = 5 and 1.1¹⁰ = 2.59 times at m = 10. An
error that would be negligible in a task score changes a predicted life by a factor of two.

*Cost.* One referee window per envelope case: a two-port link has 18 load coordinates (§31.6), so a direction set of
2 × 18 axis directions gives at most 36 windows of 1 s per link class, and three to five classes on a symmetric
quadruped give 108 to 180 referee-seconds of simulated time per iteration of the structure loop (DESIGN arithmetic).

*Owner and verb.* `frob-foundry`; `foundry.loads` returns each case with both values, its label and the replay
address. No new kill gate: the rule is a use of promotion, and the fallback when the referee and `strict` are both
unavailable is the existing one, no packet.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-FOUNDRY-831 | Known answer | Every envelope case in a packet has a re-extraction record. A planted fast-tier artifact (a stiffness spike that doubles one peak on the fast tier only) is relabelled `tier-sensitive` and replaced in 100 of 100 runs; a packet request holding a provisional case is refused in 100 of 100; the referee cost per packet is reported | M5 |
<<<END>>>

<!-- B28 · chapter 31 · tier S · fills: adjudication H-12 (closed-form member sizing as a validity check inside screening) -->
<<<INSERT chapter=31 title="Member sizing as a validity check in screening">>>
**Member sizing inside screening [S].** This amends ADR-FOUNDRY-01. The topology rung, the finite-element checks
and the build packet stay with qualified finalists. The beam rung of §31.7 costs milliseconds and also runs
earlier, as a validity check on what a design declares. Without it a screening campaign rewards links that could
not be built: a limb segment whose shell is thinner than any section that survives its own loads has less distal
inertia, lower swing torque and a better score than its buildable version would have.

*The check.* For each link whose fill is a shell or a lattice (§14.5), `foundry.size` computes the lightest stock
section between the link's ports that passes the criteria of §31.7 under a load set. It compares that section's
mass m_min and its inertia about the proximal joint with the values the design's fill produces.

- *Before any rollout*, at stage 6 of the preflight beside the actuator checks: a bounded load set from statics.
  Each foot, wheel or end-effector port carries the robot's weight times a class load factor (DESIGN defaults on
  the archetype card: 3 for legged feet, 2 for wheels, 1.5 for manipulator ports with the rated payload added),
  in the worst direction the port admits. The finding is `under-built`, with the ratio m_min over declared mass.
- *After the first rollouts*: the same check under the peak port wrenches of T1 telemetry, taken time-coherently
  as in §31.6. A design that is `under-built` under measured loads keeps its scores with the label
  `structure-unchecked`. It cannot be promoted (§16.8) until it is re-run with the sized section, or until a typed
  edit changes the declared one.

A link may ask to be sized instead of declaring a wall: the fill `{shell: sized}` takes the thinnest stock wall
that passes, and the model's mass properties are set from it before the first rollout (DESIGN; the default for
archetype templates if the gate below supports it).

*Known answer.* A cantilevered tube 0.3 m long carries 600 N across its tip, about three times the weight of a
20 kg robot. The root moment is 180 N·m. In an aluminium alloy with S_y = 276 MPa and a safety factor of 2 the
allowable stress is 138 MPa. At a mean radius of 15 mm the thin-wall section modulus π·r²·t requires
t ≥ 180/(138 × 10⁶ × π × 0.015²) = 1.85 mm. At 2,700 kg/m³ the mass 2π·r·t·L·ρ is 0.141 kg, and the inertia about
the root, as a uniform rod, is m·L²/3 = 4.2 × 10⁻³ kg·m². A design that declares this segment at 0.05 kg is
under-built by a factor of 2.8, and its swing inertia is understated by the same factor.

*What it is not.* It is a floor on mass, not a design: no fatigue, no joints, no local buckling beyond the card's
knock-down, and impact enters only through the load factor until telemetry exists. It does not make structure
part of the decision-time thesis (§31.2). It removes one way for screening to mislead.

*The number that decides whether it matters.* HYPOTHESIS: enforcing sized members changes the winning actuator
card or gear ratio in a non-trivial share of campaigns. The test: 10 quadruped and 5 mobile-manipulator campaigns
from the design corpus, each run with declared walls and with sized members, reporting how often the winner or its
actuator selection changes.

*Owner, verb and refusal.* `frob-foundry` sizes; `frob-analyze` calls it at stage 6; `frob-arena` applies the
`structure-unchecked` label. The verb is the existing `foundry.size`, called by `design.check`. Promotion of a
design with a link that is `under-built` under measured loads is refused, with the link, the ratio and the sized
section as a typed repair.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-FOUNDRY-832 | Known answer; Design comparison | The tube case returns t = 1.85 mm and 0.141 kg within 1%. A planted under-built link is flagged before the first rollout and blocks promotion. The check adds at most 50 ms to `design.check` on the reference robots. The corpus comparison is run and its share recorded; if the winner or its actuator selection changes in fewer than 1 campaign in 15, the check stays as a promotion guard and `{shell: sized}` is not made the template default | M3 |
<<<END>>>

<!-- B29 · chapter 32 · tier S · fills: K-095 offline replay dossier; §4.2 "native paired-intervention harness before any browser bundle" -->
<<<INSERT chapter=32 title="The offline replay packet">>>
This subsection extends §32.15 and the verb `telemetry.export-html` (§20.10). It fixes what the self-contained HTML
replay contains, how large it may be and when it may be built. Tier [S]. Owners: `viewer/` for the page and
`frob-chronicle` for the packet writer.

**Order of construction (DESIGN).** The packet is an adapter over records that already exist. It draws a parent world
and its forks from Chronicle (§29) and the contrast table from Autopsy (§27). It computes no fork, no attribution and
no verdict, and it contains no stepping code. The fork overlay is enabled only after the native harness has passed its
null-fork and fork-classification gates (§27.17). Reason: a browser bundle built first becomes a second, unreviewed
implementation of replay. A page that only draws recorded rows cannot disagree with the harness.

**Contents (schema `frob.replay-packet/1`).** One `.html` file that makes no network request. In file order:

1. A header in canonical JSON: the run, the parent world address and the fork paths; the model, scene, task and policy
   identities; tier, determinism class and the claim class of the source run; the tick range and the frame period as
   a rational; the channel table (name, kind, unit, frame, quantization); a BLAKE3 digest per embedded block; the
   chain digests of the source `.frun` chunks; the exporter version; the no-authorization flags of §12.
2. Timelines for the parent and for at most 16 forks. A frame holds one affine row per body: a 3 × 4 matrix in `f32`,
   48 bytes, in the anchored convention of §35.8. A fork is stored from its fork tick and refers to the parent block
   for its prefix.
3. Overlay channels as quantized columns in the layout of `.frun` (§29.3): contact cones, energy flow, actuator state,
   the seven instants of a sample, as selected.
4. Tables: the fork classification of each fork, the contrast table with flip rates and the placebo flip rate, and
   the labels of §27. Text is the text the verbs returned.
5. The page: script and styles inline.

**Size (FACT, arithmetic).** Timeline bytes are 48 · bodies · frames · timelines. A quadruped with 13 bodies over a
3 s window at 50 frames per second, with a parent and 16 forks, is 48 · 13 · 150 · 17 = 1,591,200 bytes, and
2,121,600 bytes after Base64 encoding, which multiplies by 4/3. A humanoid with 30 bodies is 3,672,000 bytes and
4,896,000 encoded. DESIGN: the whole file is capped at 8 MiB = 8,388,608 bytes by default (TARGET), with 600,000 bytes
budgeted for the page, so the binary payload may not exceed (8,388,608 − 600,000) · 3/4 = 5,841,456 bytes.

Counterexample to budgeting by payload size: 6,200,000 bytes of rows is under the cap as binary, 8,266,667 bytes once
encoded and 8,866,667 bytes with the page, which is over. The writer therefore computes the encoded size before it
writes anything. A request over the cap is refused with the size table and the three reductions that would admit it
(half the frame rate, fewer forks, bodies restricted to a named subtree). Nothing is truncated silently. Decimation in
time always keeps the fork tick, the first-divergence tick and the failure tick.

**Rendering.** WebGL 2 through the Three.js page of §32.15. WebGPU is used only when the page reports a secure context
and an adapter. FACT (to be re-read from the primary source before a gate cites it): WebGPU is exposed only in secure
contexts, and whether a file opened from disk is one is left to the browser, so a packet that required WebGPU would not
open everywhere. No frame-rate figure is claimed.

**Determinism and read-back.** The same run, options and exporter version produce the same bytes.
`frob telemetry export-html --check <file>` recomputes every block digest and compares it with the source chunks; when
the source has been evicted it reports `source: evicted` and checks internal digests only. The in-page probe is the
equivalent of `telemetry.inspect-pixel`: it returns the channel, unit, world address and tick of the quantity under
the pointer, so every number a viewer reads is a stored value.

**What a packet may say.** The claim class of its source and nothing higher. Fork overlays carry the fork's label
(§27.4), attributions are `Estimated`, and the header repeats the flags. A packet is a view: importing one creates no
evidence.

**Refusals.** A packet over the cap; frames requested from a chunk that is neither stored nor pinned; a fork overlay
requested for a run whose determinism class does not license replay (§35.3); a fork overlay requested before the
harness gates have passed for the build.

**Known answers.** The four sizes above. A null fork drawn as a ghost coincides with its parent: the rows are
bit-equal, so the deviation is exactly zero in every frame. The 6,200,000-byte request is refused with 8,866,667
printed. A probe value equals the telemetry channel within its quantization.

**Gate.** GATE-TETHER-833 (Known answer, M3). For the docking fixture of §24.10 and the split-grip fixture of §24.11,
each with a parent and 16 forks: the file is at most 8,388,608 bytes; it opens and plays from a local file with the
network disabled in two pinned browsers under a headless driver; two exports are byte-identical; every block digest
equals the source's; 100 probe points per packet equal `telemetry.query` within channel quantization; the null-fork
deviation is zero; the over-cap request is refused with its size table.

**Kill gate.** ✖ KILL-REPLAYPACK. Trigger: GATE-TETHER-833 fails after one repair cycle, or the page cannot be held to
its byte budget. Fallback: `run.export` to `.frun`, stills with overlays (`telemetry.stills`) and the live viewer; no
single-file packet. The build packet of §31.12 may include a replay packet as an optional file and never depends on
one.
<<<END>>>

<!-- B30 · chapter 39 · tier S · fills: adjudication H-19 and H-18 (the composed-incumbent recovery ratio; phase ablations) -->
<<<INSERT chapter=39 title="Three arms, the recovery ratio and the phase ablations">>>
**Three arms [S].** One ratio cannot answer the two questions the test is asked: how far this system is ahead of
the incumbent pipeline as it is commonly run, and how much of that lead an incumbent engine recovers once every
portable method is layered over it. Every stratum is therefore run with three arms on the same sealed missions,
acceptance protocol, hardware and resource caps.

| Arm | What it is | Frozen by |
|---|---|---|
| B, the declared incumbent | A leading incumbent simulator with per-design reinforcement learning and a standard outer-loop optimizer | Its own advocates, on development fixtures |
| W, the composed incumbent | The same engine or engines with every portable method layered on: native state restore and forks, per-topology buckets or the engine's own heterogeneous solver, a sampling controller or a multi-embodiment prior with checkpoint resume, racing and stopping, importance-weighted failure search, a second engine as referee | A competent baseline engineer (§2.1), on development fixtures |
| F, this system | The release under test | The release |

With T_X the restricted mean time of arm X on a stratum and C_X its GPU-hours, the **recovery ratio** of the
composed incumbent is

    r_T = (T_B − T_W) / (T_B − T_F)        r_C = (C_B − C_W) / (C_B − C_F)

defined when F beats B on that measure. A value of 1 means the layer recovers the whole saving and the engine adds
nothing on that stratum; 0 means the layer recovers nothing. If T_F = 0.10·T_B, then r_T ≥ 0.8 exactly when
T_W ≤ 0.28·T_B. Three cases are fixed before any measurement. If T_F ≥ T_B the stratum is reported as "no saving to
recover" and r is undefined. If T_W < T_F then r exceeds 1 and the stratum is a loss to the composed incumbent.
A mission unresolved at the deadline H costs H in every arm.

Decision rules (TARGET thresholds, registered before the sealed suite opens):

- r_T ≥ 0.8 and r_C ≥ 0.8 on a stratum: a thin layer over an incumbent is enough there. The report says so and the
  fallback of BET-14 applies to that stratum.
- Both below 0.5: the engine is a source of the gain on that stratum.
- Otherwise the stratum is reported as mixed, with the phase the layer could not recover named from the ledger.
- The word "ten-fold" is governed by F against the stronger of B and W (§2.1), never by F against B alone.

*One receipt first.* FACT (Appendix H): in the incumbent documents read, run-to-run determinism, heterogeneous
worlds and the fastest solver sit on different solver paths, so each is available and their conjunction is not
shown. Arm W is therefore built as one pipeline, on one fixed-topology brief and one mixed-topology contact-rich
brief, before F exists, and the property that binds when all are demanded at once is recorded. One expensive
incompatibility can sink the layer; several rarely exercised gaps may leave it competitive. Feature counts decide
neither.

**Phase ablations [S].** The phase ledger separates three accounts of where a gain comes from. On two briefs (one
narrow wheeled, one contact-rich legged), on arm B's engine, three single-change replays run against the traced
baseline.

1. *Information only.* Batch proposals and a sound cheap diagnostic filter; the same controller acquisition and
   the same solver.
2. *Controller acquisition only.* The same candidates, with per-design training replaced by a qualified cheaper
   controller: a prior with fine-tuning, inheritance, or a delivered sampling controller (§22).
3. *Stepping only.* The same candidates and controllers on a faster solver at matched fidelity.

Each returns its own ratio and the share of the baseline's critical path that it touched. The ablations must
respect two bounds computed from the baseline trace. If training that an intervention does not remove is a share
s of elapsed time, that intervention's ratio is at most 1/s (1.43 at s = 0.7). If an intervention removes that
share entirely its ratio is at most 1/(1 − s) (3.33). Illustration: 50 trained designs at 35 minutes each with 1.5
minutes of agent time per design make training 1,750 of 1,825 minutes, 95.9%, and cap information alone at 1.04.
With controller acquisition at 45 s per design, agent time is 75 of 112.5 minutes, two thirds, and information
becomes the larger lever. Which regime a stratum is in is a measurement, not a premise.

*Owner and verbs.* The protocol is this chapter's; the phase ledger is `frob-chronicle`'s. `report.campaign --arms
B,W,F` prints T, C, r_T and r_C with intervals, and `report.campaign --ablate {information, controller, stepping}`
prints an ablation with its bound.

| Gate | Claim | Threshold (TARGET) | First milestone |
|---|---|---|---|
| GATE-TENFOLD-834 | Design comparison; Performance | r_T and r_C are published with 95% intervals for every stratum that has a ratio, whatever their values. The three ablations are run on both briefs, and a ratio above its 1/s or 1/(1 − s) bound voids the stratum as an accounting error. The one-receipt pipeline for arm W is built and its binding property recorded before any engine-internal work package beyond M2 is funded | M3; M7 for the sealed suite |
<<<END>>>

## (d) Contradictions and errors noticed in the parts

None of these was changed. Items that an insertion block or a corrected wording of section (b) resolves say so.

1. **Energy-budget card, term labels.** P01.md §10 (C-31, "Test") says "with 0.25 J stored and 0.10 J reserve
   the total is 1.49 J". P06.md §22.11 (worked card) assigns the same numbers differently: 0.25 J is external
   work (gravity over the stroke), 0.10 J is the series spring (stored), and reserve is a *further* 0.10 J that
   gives the 45.5 ms figure. The §22.11 reading matches the source arithmetic; C-31's labels should follow it.
   In the same section the `.fenergy` sketch lists external 0.25, stored 0.10 and `reserve_j` 0.10 together,
   which totals 1.59 J, not the card's 1.49 J.
2. **Tube inclusion, statement and tier.** P01.md §10 (C-31, tier 2) states the inclusion as
   R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n; P06.md §22.12.5 states it as ⊆ K_n ⊖ E. The constructions table lists C-31
   as [F] as a whole, while §22.12.5, §22.17 and GATE-SHIELD-05 make the tables [M] (M8).
3. **Model-table size.** P03.md §17.4 (struct comment) and §17.7 give 20 to 50 kB per design ("0.2 GB for 4,096
   designs"). P05.md §21.11 gives about 192 bytes per link, 12 kB for a 64-link variant and 3 MB for 256
   variants. The fields of the §17.4 struct sum to roughly 165 bytes per link, which agrees with §21.11 and not
   with 20 to 50 kB unless sample lists are counted; neither section says so.
4. **Two gates and two kill gates for one mixed-batch experiment.** GATE-SPARK-10 (P05.md §21.24) requires at
   least 0.70 of the harmonic mean of homogeneous rates on three body plans; GATE-PHYLUM-03 (P03.md §17.19)
   requires at least 0.6 of homogeneous world-steps/s on cohorts of at least 256 designs. KILL-BUCKET (P05.md
   §21.11) and KILL-PHYLUM (P03.md §17.19) both retire the table-driven mixed batch to per-family templates,
   with different triggers (Ω₃ ≤ 2 within eight buckets and 0.5 of the harmonic mean; 0.5 of homogeneous on 512
   designs, eight buckets, or arm B covering 95% and faster). The pair is not in reconciliation decision R1.
5. **Matrix accelerators for policies.** P03.md §18.12 ends "Matrix accelerators remain available to policy
   networks (§23)". P06.md §23.8 says half-precision and tensor-core paths "are not adopted for policies of a
   few hundred units either", and P05.md §21.20 rejects tensor-core products for a 256-wide policy outside the
   labeled D4 profile.
6. **Response batch without a gate.** P03.md §18.5 defines an [F] mechanism with a verb and KILL-RESPONSE, but
   §18.16 and §18.17 hold no known answer and no gate for it, against style rule 5 (filled by Block B08).
7. **Rank-agreement thresholds disagree across chapters and with reconciliation decision R6.** P06.md §23.11: a lower
   bound on τ of at least 0.7 lets a rung select finalists, 0.5 to 0.7 discards only, and below 0.5 "it is not a
   rung". P09.md §28.9 (mode table): `selective` at τ ≥ 0.5, `discard-only` at 0.2 ≤ τ < 0.5 (the bottom two thirds
   are eliminated), `uninformative` below 0.2. So §28.9 lets a rung discard where §23.11 and R6 ("0.5: below this the
   cheap rung may not discard") forbid it, and it awards selection at 0.5 where §23.11 requires 0.7.
8. **Two different "closed" registries of proof kinds.** P01.md §12 lists `poly-identity`, `rational-ldlt`,
   `interval-sign`, `interval-root`, `farkas-dual`, `conic-dual-strict`, `witness-replay`, `partition-cover`,
   `monotone-transport`, `complex-connectivity`, `tube-inclusion`, `rewrite-trace`, `orbit-count`,
   `group-enumeration`, `energy-budget`, `alias-witness`, `product-unreachability`, `alpha-root`, `lean-capsule`,
   `statistical-receipt`. P07.md §25.5.2 (registry `frob.cert.kinds/1`, "closed") lists `lp-primal`, `lp-farkas`,
   `conic-farkas`, `interval-box`, `taylor-box`, `sos-gram`, `finite-game`, `alias-pair`, `monotone-transport`,
   `symmetry-transport`, `partition`, `composition`, `product-unreachable`, `trace-witness`, `sample-coverage`. Only
   `monotone-transport` has the same name in both. §25 text uses the §25 names throughout; T-34 and the tube tables
   use §12 names. One registry must govern.
9. **Fork classification differs between Autopsy and Chronicle.** P08.md §27.4 has five classes and lets a change of
   surface friction, compliance or the contact law start at the fork tick (contact memory discarded, cold first
   solve), and re-initializes packet queues and filter state for an information intervention. P09.md §29.8 has three
   classes and puts "a contact law; global friction" under structural substitution, restarting from the authored
   initial condition, with an intervention allowed only when "no state held in the snapshot depends on a changed
   field". GATE-CHRON-05 tests "the three classes"; GATE-AUTOPSY-02 tests 40 pairs over five. The warm-start-graft
   refusal is FROB-REF-582 in §27.4 and FROB-REF-882 in §29.8.
10. **`telemetry.export-html` has no single home.** P11.md §32.6 lists the telemetry cluster, `export-html` included,
   as specified in §29; P09.md (§29) never mentions the verb; P04.md §20.18 lists it "as in §20.9 and §20.10"; the
   inventory lists it under §20 only. The viewer paragraph is duplicated almost verbatim in P04.md §20.10 and P11.md
   §32.15.
11. **The screening audit is specified twice with different arithmetic and different owners.** P08.md §26.1 says
   Nemesis owns the audit of screening (§26.10, `nemesis.audit`, GATE-NEMESIS-07) and bounds the miss rate with zero
   misses in 30 audited discards at 9.5% (the exact 1 − δ^(1/m)). P09.md §28.12 defines the same audit for Atelier
   (`eval.audit`, GATE-ATEL-09) with the bound ln(1/δ)/m, which is 10.0% at m = 30. Two verbs, two gates and two
   formulas for one mechanism.
12. **Plan-time pilots are cited to different appendices and with different rounding.** P00.md §0.7 and §2.5 cite the
   solver pilot as "FACT, Appendix K" with 1.345× warm and 1.088× cold; P05.md §36.5 cites the same pilot as "a
   recorded pilot, Appendix H" with 1.35 and 1.09; P07.md §25.22 points to a reproduction bundle located through
   Appendix H.
13. **Dangling references.** (a) P00.md BET-25 names as its control "two friction regimes with identical histories
   before first slip", and the P00 ledger asks Appendix N to hold "the pre-slip aliasing fixture"; no chapter defines
   the fixture or a gate for it (Block B12 supplies it). (b) P00.md NONGOAL-15 and the milestone skeleton place
   "gradients through contact" at M6; no chapter specifies the mechanism (Block B04 supplies it). (c) P04.md §20.4.7
   states its kill rule inline without the identifier KILL-SCHED, which appears only in the §20 gate table.
14. **A preferred gate was dropped without a replacement.** P10.md ADR-FOUNDRY-11 removes every reviewer-dependent
   gate, which is defensible, but §31.12 then leaves the review import lane with no record format or protocol, so
   `review` can never leave `no-claim` in a comparable way (Block B26 supplies the format without adding a
   reviewer-dependent gate).
15. **P00.md §4 BET-32 against P08.md §26.9 and P09.md §28.19.4.** BET-32 says bounded-weight importance sampling
   "gives valid bounds on failure rates with far fewer episodes than crude sampling" and sets a falsifier of a saving
   below 3 times at a rate of 10⁻⁴. §26.9 shows that with weights bounded by 1/ε the upper bound after n clean runs
   cannot fall below about ln(1/δ)/(ε·n), "so importance sampling is never the cheaper way to certify absence", and
   §28.19.4 keeps it "only as a lower bound for refutation". As written the bet's own falsifier fires by arithmetic
   for an upper bound. The bet should be restated for refutation and for estimates once failures are observed.
16. **P03.md §16.8 against P09.md §29.11 (weights of the tier correction).** §16.8: "weights clipped at 1/p_min ≤ 20
   with the clipped mass reported". §29.11: "Bounded weights without clipping … nothing is clipped and no clipping
   bias exists". The floor on the audit probability makes clipping unnecessary; §16.8 should say bounded, not clipped.
   The two sections also give different defaults for the prediction (identity in §16.8; c + β·Y_fast from earlier
   ledger rows in §29.11).
17. **KILL-FDGRAD has two different triggers.** P03.md §17.13: variance ratio below 2 at every horizon of 50 or more,
   or sign agreement below 70%, or no end point confirmed by a specialist; fallback: rank-based search with the
   stencil kept as a sensitivity report. P09.md §28.8: a variance ratio below 2 at 250 steps, sign agreement below
   70%, or no specialist-checked gain over a same-budget CMA-ES generation; P09.md §28.23 calls it "the pilot of
   §28.8" although the pilot is specified in §17.13. The brief's R1 makes §17 the owner; §28 should cite it.
18. **P03.md §16.12 cross-reference.** "Design gradients by paired differences are §17.12." Paired finite-difference
   design gradients are §17.13; §17.12 is the quality-diversity maps.
19. **P09.md §28.23 GATE-ATEL-06 and the P09 ledger cite a stale refusal range.** "FROB-REF-553 to -544" (gate) and
   "FROB-REF-553 to FROB-REF-551" (ledger) no longer denote a range: the codes of §28.20 are now 553, 555, 542, 543,
   556, 545, 546, 557, 548, 549, 550, 551. The gate should list the codes or say "every refusal of §28.20".
20. **Sampling-controller cost figures differ between chapters.** P06.md §22.8 derives about 0.33 and 1 GPU-minutes
   per 20 s episode (quadruped, humanoid) at a 20 ms planning step and about 1.7 and 5.2 at 4 ms. P09.md §28.9 quotes
   "0.3 to 1 GPU-minute per quadruped episode and 1 to 5 per humanoid episode", which drops the 1.7 for the
   quadruped. GATE-REFLEX-06 then sets medians of 1 and 6.
21. **Two orchestration verbs for the same job.** P09.md §28.8 defines `opt.cascade --spec` as the "one verb for the
   whole campaign", judged in the A/B of GATE-ATEL-03; P11.md §32 defines `solve.run` with its own kill gate
   KILL-SOLVE and the same criterion (agents do no better than agents driving the verbs). The relation between the
   two is not stated in either chapter.
22. **Slot-invariance gate named twice.** P05.md §21.11 KILL-SLOT triggers on GATE-SPARK-15; P11.md §32 states the same
   test as GATE-TETHER-09 (1,000 episodes solo and in 100 co-tenancies; at most 5% loss at 8,192 identical worlds).
   R1 names §32 the owner of KILL-SLOT; the kill gate should cite one gate.
23. **The word "ghost" already has two meanings** (the viewer's counterfactual overlay in P08.md §27.12 and P11.md §32;
   the metadata queue of the S3-FIFO policy in P12.md §33). Block B16 therefore uses "foil body".
24. **P06.md §22.13 leaves a stated objection unanswered.** Γ_use includes Stab(current contact mode) and is "computed
    when it is used", so tied gains untie at a mode switch. The section does not say whether a controller's output
    must be continuous across the change of group; the equivariance-in-law fixture covers a different case.
25. **The mixed-batch figure is worded two ways.** P00.md §0.8 (row H-03), §2.4 (lever 1) and P03.md §17.1 say an
   incumbent's mixed batch runs "at 0.86 of its homogeneous rate"; P00.md §1.1 and P05.md §21.11 and §36.3 say
   "0.86 of the harmonic mean of its homogeneous rates". The second is right (86,246 ÷ 100,318, the harmonic mean
   of 286,510, 130,308 and 53,360). P03.md §17.1 also prints "(86 k samples/s)" as if it were the ratio. Corrected
   in section (b).
26. **The structural mass ceiling is stated as 4% and as 8%.** P00.md §0.8 (row H-12) and P01.md §10 ("about 4% of
   robot mass or less"; "Ceiling: about 4% of robot mass") against P10.md §31.2, which computes
   f_L·s ≤ 0.27 × 0.30 ≈ 0.08 and says "thirty percent about 8%". Corrected in section (b).
27. **One figure is a HYPOTHESIS in one chapter and a FACT in two.** P08.md §26.9 labels "10 to 100 times fewer
   episodes" a HYPOTHESIS; P09.md §28.19 and P01.md §10 write it as "published adaptive stress testing reports"
   and "FACT, source register". The sources behind it report 2 to 20 times and about 5 times in mean-square error;
   the larger figure is derived from a variance formula. Corrected in section (b).
28. **The scope of bounded-weight sampling differs between the thesis and its chapter.** P00.md §2.6 ("Failures
   driven by per-step disturbances only") and BET-32 restrict it to per-step disturbances. P08.md §26.9 restricts
   only splitting, applies tilting to static parameters and applies the defensive mixture to discrete scene
   edits. Corrected in section (b), together with the item above on BET-32 against P08.md §26.9.
29. **An unlabelled ceiling.** P03.md §16.6: "the factor is under a tenth of the step at 18 to 35 degrees of
   freedom, so the gain there is below 1.1" carries no label and no source (style rule 3). The incumbent trace
   the plan cites elsewhere (P03.md ledger: "53 of 262 ns; 38 ns constraint assembly") puts the solve stage at 47%
   and does not separate the factor. Block B06 and section (b) relabel it and measure it.
30. **Two thresholds for one cost.** BET-27 (P00.md §4) is falsified when a humanoid episode under the sampling
   controller costs more than 5 GPU-minutes; GATE-REFLEX-06 (P06.md §22.18) passes at a median of 6.
31. **The screening cap is not derived from the section's own arithmetic.** P06.md §22.8 computes a break-even of
   30 and 60 episodes per design and then caps screening at three with no reason given; GATE-REFLEX-06 asks for a
   Kendall τ against specialists, which three episodes per design estimate poorly. Section (b) keeps three as the
   default and admits paired comparisons up to the break-even.
32. **A stale ledger line.** P05.md ledger (lines 1794–1795) restates GATE-CORTEX-05 as "tolerance 1.5 times the
   incumbent … planning value 615 s". P05.md §36.3 row 4 and P06.md GATE-CORTEX-05 say 1.25 times and 515 s, as
   reconciliation decision R5 requires.
33. **The expected-range table does not name its comparator.** P00.md §2.1 defines the ratio against the strongest
   composed rival. P00.md §2.6 derives its ranges from denominators of per-design training (410 s; 30 minutes) and
   does not say that a composed rival hosting the same portable levers would shrink them. Corrected in section (b).
34. **Four engine-internal gaps are stated as established; two are documented.** P00.md §0.8 (row H-19) and BET-14
    list deterministic forks with mixed topologies, replay of one world outside its batch, certificates and a
    Metal tier. The evidence behind the row documents the first and the last and does not examine the other two.
    Corrected in section (b).
35. **A narrowed state with no measurement.** P00.md §0.8 gives H-18 the state NARROWED on "Training dominates
    published loops", while §2.2 of the same file says no end-to-end phase ledger of a design loop was found, and
    the state table of §0.8 reserves NARROWED for a strong form that failed on evidence. The hypothesis's own
    falsifier is that training dominates, so the surviving sentence, if measured, would refute it, not narrow it.
    Corrected in section (b): ACTIVE.
36. **The referee coverage band travels without its assumption.** P03.md §16.11 derives "of order 0.1 to 2%" from a
    referee at 0.2 to 1.0 times real time per core, and in the same paragraph cites a published CPU engine at
    28,000 to 46,000 steps/s per core, which would give a far larger share. P00.md §0.8 and §1.8, P05.md §36 and
    P09.md §29 repeat the band with no referee speed attached. Corrected in section (b).
