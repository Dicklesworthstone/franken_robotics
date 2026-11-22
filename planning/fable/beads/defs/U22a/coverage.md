# Coverage of drafting unit U22a (§22 preamble and §22.1 to §22.9 of Reflex)

Files: 54 bead files. Thirteen seed beads (BD-049, BD-050, BD-051, BD-052, BD-114, BD-116, BD-249, BD-251, BD-252,
BD-330, BD-331, BD-334, BD-335). Forty-one new beads: the registry `N22-KA` and `N22a-01` to `N22a-40`.

The unit was drafted in two sittings. The first wrote 34 files (the nine seed beads up to BD-252 and `N22a-01` to
`N22a-25`) and was cut off before this file existed. The second kept those files unchanged and wrote the twenty that
were missing: BD-330, BD-331, BD-334, BD-335, `N22-KA`, and `N22a-26` to `N22a-40` (the key `N22a-26` was already
named by five of the earlier files as the script bead for `control.mpc`, `N22a-27` as a dependency of `N22a-16`, and
`N22a-29` as the first gait bead; the new beads fill exactly those slots).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §22 (preamble) | Reflex: estimation, control, gaits, skills, shields | No text before §22.1 beyond the heading; nothing to carry |
| §22.1 | What it owns | N22a-01 (crate table, placement of the traits `Controller`, `ActionFilter`, `StateSource` in `frob-dyn`, rule L-1, why Reflex exists with the published training times, what it is worth and what is untested, what Reflex does not claim); BD-249 (what GATE-REFLEX-02 does and does not ask); N22a-17 (GATE-REFLEX-04 asks the ranking question); N22a-10 (phase tables read from `frob-algebra`); N22a-29 (`frob-gait` owns the orbits; it receives the controller only through the trait) |
| §22.2 | Decisions | N22a-01 (all eight records ADR-REFLEX-01 to -08 with reasons); governed beads restate their own: ADR-REFLEX-01 in BD-116, N22a-16, BD-330; -02 in N22a-08, N22a-35; -03 in BD-252, N22a-24, N22a-25; -04 in N22a-29, N22a-33; -05 in N22a-34; -08 in N22a-15. ADR-REFLEX-06 and -07 govern beads of unit U22b |
| §22.3 | The estimator [S] | BD-114 (the contact-aided invariant filter on SE₂₊ₖ(3) with biases, wheel odometry, declared error convention and covariance transport, measured log-linearity, the four-dimensional gauge null space PO-REFLEX-01, contact add and marginalize, slip down-weighting); N22a-02 (scoring by the gauge rule, refusal of a fitted scale, consistency statistics, the quaternion-filter comparator, the `overconfident` flag); N22a-03 (`--state estimated` against `--state privileged` with the reported difference, the four namespaces, out-of-sequence measurements); N22a-04 (verb `control.estimate`); N22a-05 (its end-to-end script, GATE-REFLEX-03 recorded); N22a-13 (filter in lockstep on batched tiers) |
| §22.4 | Baseline controller synthesis by morphology class | BD-116 (the legged row assembled; identity; outcomes); N22a-06 (errors on the group, the declared re-chart near π, gains from the model); N22a-07 (`ControllerId`, INV-REFLEX-02); N22a-09 (single-step stance force distribution over friction cones and actuator boxes); N22a-10 (gait clock, choice by Froude number, capture-point footsteps, swing through leg IK); N22a-11 (end-to-end, legged row of GATE-REFLEX-01); N22a-13 (the row on `batch-cpu`); N22a-14 (the whole-body program, prioritized tasks, schedules re-checked under the committed contact law); N22a-15 (actuator bounds are not negotiable: `actuator-infeasible`); N22a-08, N22a-12 (infeasibility is an answer; prescribed fallback); N22a-18 (the last two columns of the class table as registry data); N22a-19 (rung R1); N22a-20 (pilot of the four Frontier rows); N22a-21 (the legged predictive row). The wheeled, steered, omnidirectional, tracked, multi-legged, biped, hybrid and arm rows are seed beads BD-117 and BD-247 of unit U14b; the verb `control.synth` is BD-118 of unit U22c |
| §22.5 | One cone solver [S] | N22a-08 (control programs on the shared solver, the three end states the caller must inspect, the Farkas argument, the certificate rounded to rationals and re-checked exactly, PO-REFLEX-03, the 50 N hand case); N22a-09 (the stance program and its witness); N22a-13 (programs as kernels in lockstep across worlds); BD-251 (auditor-checked certificates, GATE-REFLEX-05); N22a-14, N22a-21 (larger programs on the same solver); N22a-35 (the semidefinite use). The solver itself is BD-115 (unit U41); the cones are BD-050, BD-051 here |
| §22.6 | Reporting: three outcomes, never a bare failure [S] | N22a-12 (`not-synthesizable` with witness, `synthesized-failed` with fallback log, first failed layer and next rung, `ok` with saturation report, layer fractions and standing); N22a-17 (the standing `screening` until GATE-REFLEX-04 passes for the class and task family); N22a-09 (the witness when the reason is a program); BD-116, N22a-11 (outcomes on the legged row); BD-249 (outcome tally on the corpus) |
| §22.7 | An inspectable baseline plus a learned residual [F] | N22a-16 (the wrapper u = limit(u_base + Δ), bounded authority ρ, zero start, PO-REFLEX-02, `residual-saturated`, shared against retuned comparisons); N22a-15 (the five layer values on every tick; fallback events; INV-REFLEX-01); N22a-18 (verb `control.ladder`, rungs R0 to R4 and the escalation rules); N22a-19 (R1); N22a-14 (R3's base); BD-330 (R2 and R3 run and measured on the corpus; KILL-RESIDUAL's M4 clause); N22a-27 (the pilot; KILL-RESIDUAL's pilot clause); BD-251 (GATE-REFLEX-05) |
| §22.8 | The sampling-based predictive controller [F] | BD-252 (the controller, the six algorithm steps, class-default costs, planning model and state options, modelled latency, the two roles, verb `control.mpc` with its options); N22a-23 (kernels `K-fork`, `K-cost`, `K-weight`; forks leave their source untouched); N22a-24 (the cost arithmetic in exact form: the three published rows, GPU-minute figures, the step ratios, break-even counts, 299 episodes, the demonstration arithmetic; the rules: three-episode cap, comparison exception, plant equals model and `model-privileged`, declared latency); N22a-25 (never a stand-in evaluator: FROB-REF-817, INV-REFLEX-03); N22a-26 (end-to-end script of the verb); N22a-22 (pilot); BD-331 (GATE-REFLEX-06 and KILL-MPC measured); N22a-07 (planning configuration and latency model in the `ControllerId`) |
| §22.9 | Gaits as orbits (`frob-gait`) [F]: opening and the screening label | N22a-29, N22a-36 (the published cost, the label `screening: gait-orbit`, the limits printed with every result); N22a-28 (pilot); BD-335 (what screening may do) |
| §22.9.1 | The map | N22a-29 (state x = (q, v, a, c), the control-period map F_θ, the stroboscopic map P_θ = F_θ^N with a rational stride, equivariance under translation and yaw, the reduced state x̄ with removed coordinates listed, the relative periodic orbit and its drift) |
| §22.9.2 | Shooting with symmetry, stability without it | N22a-31 (the twisted condition, shooting in Fix(K) by Newton with a line search, seeds from the atlas and a settled stance); N22a-32 (stability over the full reduced state, M = (ρ(σ)⁻¹·D F̄^(N/m))^m, PO-GAIT-01, isotypic blocks, symmetry-breaking instabilities named by block, the spectrum and transient growth through the airlock, the smallest counterexample) |
| §22.9.3 | Derivatives, by tier | N22a-30 (compliant tiers: tangent worlds, central differences in `f64` at two step sizes with a Richardson check, the second implementation by derivative programs with its differential test; the label mapping; grazing and FROB-REF-431); N22a-33 (the saltation matrix, per-event records, what exists in the sibling repository, the referee route waiting on FD-FSIM-20) |
| §22.9.4 | Simultaneous impacts are handled | N22a-34 (the split check on compliant tiers; all orderings up to four feet and the coupled solve; order sensitivity in the mass norm; the coupling term; the bar fixture by hand); N22a-35 (the verdict: agreement, a common Lyapunov matrix from a semidefinite program, or `order-sensitive` with the range of spectral radii and a recovery test) |
| §22.9.5 | Continuation in design parameters | N22a-37 (pseudo-arclength continuation, the seven typed outcomes with detection and consequence, warm start of a child); BD-335 (verb `gait.continue`, screening, GATE-GAIT-03, KILL-GAIT); N22a-40 (end-to-end script) |
| §22.9.6 | Design gradients | BD-334 (d x̄*/dθ, the adjoint for stride metrics, PO-GAIT-02, refusal at a fold, multiplier gradients along the path, the route to derivative-free search, verb `gait.gradient`, the 100-case suite); N22a-39 (end-to-end script, GATE-GAIT-02 recorded) |
| §22.9.7 | What a result contains | N22a-36 (the `GaitOrbit` record, INV-GAIT-01, the standing); N22a-38 (end-to-end script of `gait.orbit`, GATE-GAIT-01 recorded). Verbs: `gait.atlas` is BD-332 (unit U28b); `gait.orbit` is BD-333 (unit U33b); `gait.continue` BD-335; `gait.gradient` BD-334 |

Seed beads assigned here that belong to another chapter's text. BD-049, BD-050, BD-051 and BD-052 are `frob-algebra`
beads (orbit counts and subgroup lattices; the `Cone` trait; the pseudo-inertia cone and mass support; posets and
Kleene iteration). Their content comes from §13 and the constructions C-03, C-06, C-12 and C-22; they are written
completely here, and what they need from the Marrow units is in their `needs`.

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone (register) | Measured or decided in this unit | Expected from another unit |
|---|---|---|---|
| GATE-REFLEX-01 | M1; M2; M3; M4 | BD-116 and N22a-11 (M1, the legged row on the referee); N22a-13 (M2, `batch-cpu`); N22a-21 (the legged predictive row); N22a-20 (pilot of the Frontier rows) | BD-118 and N22c-01 (M1, all first archetypes); N22c-47 (M2); BD-247 (M3); N22c-48 (M4, tracked and hybrid) |
| GATE-REFLEX-02 | M3 | BD-249 | — |
| GATE-REFLEX-03 | M1 | BD-114, N22a-02, N22a-04; recorded by N22a-05 | N22c-02 (PO-REFLEX-01 conformance case) |
| GATE-REFLEX-04 | M3; M4 | N22a-17 (what the recorded value lets a baseline score do) | BD-328 (the measurement, U23b); N22c-51 (thresholds) |
| GATE-REFLEX-05 | M3 | BD-251 (gate); N22a-14, N22a-15, N22a-16 (its clauses) | N22c-52 (OPEN-REFLEX-3) |
| GATE-REFLEX-06 | M4 | BD-331 (all five clauses on the pilot design set); N22a-26 (fork-count and refusal clauses at M3, partial); BD-252, N22a-23, N22a-25 (mechanisms) | N22c-53 (OPEN-REFLEX-4) |
| GATE-REFLEX-07 | M4 | — | BD-327, N22b-41 (U22b) |
| GATE-REFLEX-08 | M4 | — | BD-336, N22b-05 (U22b) |
| GATE-REFLEX-09 | M8 | — | N22b-07; BD-436 (U25a) |
| GATE-REFLEX-10 | M6 | — | N22c-25 |
| GATE-REFLEX-11 | M3 | — | N22c-33 |
| GATE-GAIT-01 | M4 | N22a-38 (recorded on a running system); N22a-32 (PO-GAIT-01 and the planted instability at unit level); N22a-33 (rimless wheel at unit level) | BD-333 (the verb and INT-FSIM-13, U33b) |
| GATE-GAIT-02 | M4 | BD-334 (suite and unit-level measurement); N22a-39 (recorded); N22a-34, N22a-35 (the bar and simultaneity clauses) | N22c-54 (OPEN-GAIT-2) |
| GATE-GAIT-03 | M4 | BD-335; N22a-37 (mechanism); N22a-28 (pilot, same statistics) | BD-328 (specialists and the ledger) |
| GATE-GAIT-04 | M4 | — | N22c-38 |
| GATE-GAIT-05 | M4 | — | N22c-42 |
| GATE-SHIELD-01 to -05 | M4; M8 | — | BD-337, BD-338, BD-339, BD-340, N22b-35 (U22b) |
| GATE-SHIELD-06 | M4 | — | N22c-29 |
| GATE-SHIELD-07 | M4 | — | N22c-46 |
| KILL-BASELINE | M3 | BD-249 (measurement and decision record) | N22c-14 (decision bead; merge) |
| KILL-RESIDUAL | M4 | N22a-27 (pilot clause); BD-330 (M4 clause, draft record, registry switch) | N22c-15 (decision bead) |
| KILL-MPC | M4 | BD-331 (measurement, draft record, registry switch); N22a-22 (pilot) | N22c-16 (decision bead) |
| KILL-GAIT | M4 | BD-335 (measurement, draft record, registry switch); N22a-28 (pilot) | N22c-17 (decision bead) |
| KILL-ENERGY, KILL-SHIELD, KILL-SKILL, KILL-TUBE | M4; M8 | — | U22b and U22c decision beads |
| KILL-MPCRECORD, KILL-FREEPERIOD, KILL-STRIDESEQ | M3; M4 | — | N22c-34, N22c-39, N22c-43 |
| GATE-MARROW-01 (not of this chapter) | M1 | BD-049, BD-050 (their law rows) | the Marrow units |

Every gate of the brief's list whose mechanism lies in §22.1 to §22.9 is named in the `gates` field of at least one
bead here: GATE-REFLEX-01 to -06, GATE-GAIT-01 to -03, KILL-BASELINE, KILL-RESIDUAL, KILL-MPC, KILL-GAIT. The
others belong to §22.10 to §22.25 and are listed with the bead that the owning unit's coverage file names.

## (c) Counts carried

- Subsections: 9 (§22.1 to §22.9), with the seven numbered parts of §22.9 each mapped separately.
- Gates: 9 convergence gates and 4 kill gates measured, recorded or decided in part here (above); 1 gate of
  another chapter touched by seed beads.
- Known answers: all 23 rows of Appendix M.21 (KA-553 to KA-575) in the registry `N22-KA`; 12 of them (KA-553 to
  KA-564) carried by value in mechanism and test beads of this unit; the other 11 belong to mechanisms of unit
  U22b. Rows of other registries cited with values: KA-043 to KA-045, KA-055, KA-056, KA-072, KA-073, KA-077,
  KA-081 to KA-084, KA-092, KA-093, KA-095 to KA-097, KA-123 to KA-126, KA-233 to KA-235, KA-238, KA-252, KA-255,
  KA-262, KA-277, KA-768.
- Counterexamples of §22 carried with their numbers: 5 of 12 (CX-234, CX-235, CX-237, CX-238, CX-245), the five
  defined in §22.3, §22.5, §22.8 and §22.9. Also carried: CX-009, CX-021, CX-034, CX-041, CX-043, CX-127, CX-233.
  The registry `N22-CX` is unit U22c's.
- Refusal codes of §22 carried with meaning, trigger, payload and ranked repairs: 5 of 11 (FROB-REF-421 in
  N22a-12; -424 in N22a-08, N22a-12; -431 in N22a-30, N22a-33, BD-334; -464 in N22a-02, N22a-03, N22a-04; -817 in
  N22a-25, N22a-26), plus the worded refusal `fixed-period-shooting-on-free-period` at its first raising site
  (N22a-29). Cited from other chapters: FROB-REF-104, -106, -213, -214, -215, -263, -443, -461, -537. The registry
  `N22-REF` is unit U22c's.
- Invariants: INV-REFLEX-01 (N22a-15, BD-251), INV-REFLEX-02 (N22a-07), INV-REFLEX-03 (N22a-25, BD-252),
  INV-GAIT-01 (N22a-36). Proof obligations: PO-REFLEX-01 (BD-114, N22a-05), PO-REFLEX-02 (N22a-16, BD-251),
  PO-REFLEX-03 (N22a-08, BD-251), PO-GAIT-01 (N22a-32, N22a-38), PO-GAIT-02 (BD-334, N22a-39).
- Pilots: 4 (N22a-20, N22a-22, N22a-27, N22a-28). End-to-end script beads: 6 (N22a-05, N22a-11, N22a-26, N22a-38,
  N22a-39, N22a-40); the corpus and gate experiments BD-249, BD-330, BD-331 and BD-335 and the verb bead N22a-18
  carry their own scripts.
- Verbs implemented: `algebra.orbits` (BD-049), `control.estimate` (N22a-04), `control.ladder` (N22a-18),
  `control.mpc` (BD-252), `gait.continue` (BD-335), `gait.gradient` (BD-334).
- Bets, risks, constructions named: BET-06, BET-26, BET-27; RISK-17, RISK-27, RISK-47; C-03, C-06, C-09, C-10,
  C-12, C-13, C-14, C-16, C-20, C-22, C-37.

## (d) Needs from other slices

| Need | Asked by |
|---|---|
| §33.9: admission INT-FSIM-13 of the airlock's `spectral` group with the adapter for `floquet::analyze_monodromy` and `stroboscopic_growth` (the admission clause of BD-333, U33b) | N22a-32, N22a-38 |
| §33.9 and Appendix J: the verb `gait.orbit`, its registry entry and option names (BD-333) | N22a-38, N22a-39, N22a-40 |
| §22.23: the event-section and bordered forms for loops without a clock (N22c-36, N22c-37), through which the rimless wheel and the compass gait are solved | BD-334, N22a-38, N22a-39 |
| §22.19: decisions on OPEN-REFLEX-1, -3, -4 and OPEN-GAIT-2 (N22c-50, N22c-52, N22c-53, N22c-54); OPEN-REFLEX-2 (N22c-51) | N22a-13, N22a-21; BD-251, N22a-16, BD-330; BD-252, N22a-24, BD-331; BD-334, N22a-39; N22a-17 |
| §22.14: registry rows of FROB-REF-421, -424, -464, -817 (`N22-REF`) | N22a-02, N22a-03, N22a-12, N22a-25 |
| §22.22: the frozen planner record and the delivered-controller lane (N22c-31, N22c-32) | BD-252, N22a-25, N22a-26, BD-331 |
| §22.20: the lifted planner behind `control.mpc --planner lifted:<id>` (N22c-24) | BD-252 |
| §22.12.3: the legged backup controller with its domain (U22b) | BD-116 |
| §22.4: the verb `control.synth` and its `--rung` option (BD-118, U22c) | N22a-12, N22a-19, N22a-21 |
| §10 C-16 and §9 D-15: the derivative-validity label type, the derived saltation kernel (T-17), refusal FROB-REF-263 | N22a-30, N22a-33 |
| §16.5: event records and fixed-mode advance from the `strict` step; the coupled impact solve and an ordered-resolution entry for analyses | N22a-33, N22a-34 |
| §16.6 and §21: a step plan whose step is a function of the declared state; kernel manifests for controller stages and the planner kernels; tangent programs of the step kernels (C-14) | N22a-29, N22a-13, N22a-23, N22a-30 |
| §13.1, §13.4, §13.5, §13.8: the `frob-algebra` foundation, Γ_obs with its representations, characters and projectors, the law harness (Marrow units N13-08, N13-13, N13-16; BD-048; BD-102) | BD-049 to BD-052, N22a-31, N22a-32, N22a-35 |
| §12.4: proof kinds `conic-farkas` and `rational-ldlt` in the auditor; the bundle writer for a control-program claim | N22a-08, BD-251, N22a-35 |
| §23.7, §23.6, §23.27: the residual head and its training option; the learner's action law for a limiting stage; `crossing` and `attained` in `train.status` | BD-330, N22a-16 |
| §23.10, §23.11: the distillation job and GATE-CORTEX-14's figure for a sampling teacher (BD-323); the family policy's out-of-distribution report | BD-252, N22a-26, BD-331 |
| §28.8 to §28.11: derivative-free search (BD-341); the rank-agreement ledger with FROB-REF-537 (BD-328); the cascade's screening and acceptance steps (BD-352) | BD-334, N22a-17, N22a-40, BD-335, N22a-25 |
| §24.6, §24.8, §24.9: registered tasks `stand-10s`, `traverse-5m-flat`, the `--state` option of the run verbs, a requirement class for engineering constraints, corpus label fields | N22a-11, N22a-03, N22a-15, BD-249 |
| §38.3, §38.5, §38.8, §38.11: the oracle tool (compass gait), the pilot register, the `smoke` battery list, the pre-registration record and gate ledger; BD-377 for the M4 suites | N22a-38, N22a-20, N22a-22, N22a-27, N22a-28, BD-251, BD-249, N22a-05, N22a-11, BD-334, N22a-39, BD-335 |
| §36 and §36.7: the disclosure block; measured throughput and training cost per class | N22a-22, N22a-27, N22a-28, N22a-24, BD-331 |
| §14.7, §14.12: typed edits from the command line; import of the pilot design set | N22a-37, N22a-40, BD-331 |
| §15, §18.6, §18.8, §20.4, §20.8, §29.9, §29.17, §32.6, §32.9, §32.10: actuator envelope tables and duty ratings, patch radius, friction ranges, rational clocks, a sensor-card bias plant, the electrical-energy channel, a missing-stream refusal, verb-set conventions (BD-137, BD-312, BD-376), budgets and the job protocol | N22a-14, N22a-15, BD-116, N22a-09, N22a-29, N22a-05, N22a-04, N22a-18, BD-334, BD-335, N22a-36, N22a-37 |
| Appendix F: upstream item FD-FSIM-20 (referee orbit route); Appendix G: reference-robot fixtures (BD-067) | N22a-33; N22a-20 |

Unit U22c's coverage asks this unit for "the labelled-derivative machinery of the orbit solver; the gait battery and
its autonomous and multi-mode members". The machinery is N22a-30 (labels on compliant tiers), N22a-33 (saltation
and its refusals) and BD-334 (gradients). The battery is the registered hybrid fixtures of N22a-33 and N22a-38, the
100-case suite of BD-334 and the catalogue of BD-335; autonomous and multi-mode members are added to those
registries by N22c-36 and N22c-41. Unit U22b's coverage asks for the `ControllerId` with the group used (N22a-07)
and for saltation matrices, labels and the grazing threshold (N22a-33, N22a-30).

## (e) Contradictions, ambiguities and what was done about them

1. **BD-333 (`gait.orbit`) belongs to another unit, and its mechanisms are §22.9's.** N22a-29 to N22a-36 carry the
   stride map, Jacobians, shooting, monodromy, saltation, simultaneous impacts and the record. None of them depends
   on BD-333; BD-333 should depend on them (the lead adds the edges) and keep the verb, the admission and the gate's
   unit-level clauses. N22a-38 records GATE-GAIT-01 on a running system and depends on BD-333. One hazard:
   N22a-32 needs the admitted `spectral` adapter (INT-FSIM-13, the other half of BD-333's done-condition). If that
   need is resolved to an edge onto BD-333 the graph has a cycle. The register puts INT-FSIM-13 at M3 and
   GATE-GAIT-01 at M4, so splitting BD-333 into an admission bead and a gate bead is the clean resolution.
2. **Cross-references kept out of `needs` to avoid cycles.** U22c's beads for §22.23, §22.24 and OPEN-GAIT-2 depend
   on BD-333, and its kill-gate decision beads depend on BD-330, BD-331 and BD-335. Mechanism beads here therefore
   name those beads in `related` and in their text, and list them in `needs` only where the work truly waits
   (BD-334, N22a-38, N22a-39 on the event-section path and on the recorded decision of OPEN-GAIT-2).
3. **Four pilot beads duplicate pilot beads of unit U22c**, which drafted §22.17: N22a-20 with N22c-05 and N22c-06;
   N22a-22 with N22c-08; N22a-27 with N22c-07; N22a-28 with N22c-09. Each pair is one experiment. Mechanism beads
   here depend on the `N22a` key; U22c's decision beads depend on the `N22c` key. Merge at load time.
4. **Kill-gate decisions are split between units.** BD-249 measures KILL-BASELINE and also writes its decision
   record; N22c-14 is a decision bead for the same gate. BD-330, BD-331 and BD-335 write the measurement, a draft
   record and the registry switch, and leave the decision to N22c-15, N22c-16 and N22c-17. BD-249 was left as
   written; the lead should give it the same split or merge N22c-14 into it.
5. **GATE-GAIT-01's fixtures are loops without a clock.** The rimless wheel and the compass gait are autonomous;
   §22.23, added later, forbids solving such loops stroboscopically and solves them on an event section. The
   gate's first two clauses and 24 regular cases of GATE-GAIT-02's suite therefore wait on N22c-36, which itself
   depends on BD-333. Beads say so; without that path those clauses are `not run`, never rescaled.
6. **Known-answer tiers against the referee route.** Appendix M places KA-560 and KA-563 on the referee, while
   §22.9.3 and OPEN-GAIT-1 say the referee orbit route waits on upstream item FD-FSIM-20 and the default is
   compliant tiers only. N22a-33 serves these rows on host hybrid fixtures and on the `strict` profile, answers a
   referee request by name, and gate rows record the adjudicating tier.
7. **Two refusal codes for grazing.** Construction C-16 refuses a derivative near grazing with FROB-REF-263; §22.14
   gives FROB-REF-431 for a gait derivative at grazing or at a fold. Beads treat 263 as the kernel-level refusal and
   re-raise it at the gait level as 431 with the one-sided estimates, as `N22-REF` also records.
8. **Point thresholds on τ against lower-bound modes.** GATE-GAIT-03 and GATE-REFLEX-06 say "Kendall τ of at least
   0.5"; §28.10 and §40.7 judge every rank rule on the lower bootstrap bound. BD-335 and BD-331 judge the gate
   clause as written and the ledger's mode on the lower bound, and say in the report when the two differ.
9. **GATE-REFLEX-06's humanoid cost clause against the throughput target.** The published humanoid setting is
   2.05 × 10⁸ model steps per 20 s episode; at 0.5 million steps per second that is 6.8 GPU-minutes, above the
   threshold of 6. BD-331 adds a throughput probe before the humanoid setting is registered and states the
   expectation in the registration.
10. **The common-norm program is larger than the shared solver's stated range for full robots.** A quadruped's
    reduced state of dimension 57 gives 1,653 unknowns; §22.5 describes programs of tens to a few hundred variables.
    N22a-35 poses the program within the admitted size and otherwise reports `common-norm: not-attempted (size)`,
    so full robots are judged by agreement or by the recovery test.
11. **"The filter state when the loop runs on estimated state" cannot be a fixed point as written**: the filter's
    covariance along its four unobservable directions grows without bound. N22a-29 computes orbits on privileged
    state by default, carries belief relative to truth with those directions removed as a HYPOTHESIS, and returns
    `unsupported: belief-not-reducible` when the reduction check fails.
12. **KILL-GAIT has no repair cycle in its trigger**, unlike KILL-MPC. BD-335 follows the text: a failed gate fires it.
13. **KA-568 rounds a bound down** (0.2511 J printed as 0.25 J). Recorded in the row's notes of `N22-KA`; the owning
    bead (BD-337, U22b) decides the outward rounding.
14. **Readings labelled DESIGN in the beads, where the plan gives one line or none**: the population, eligibility
    and statistic of the residual measurement, including the exclusion of designs whose baseline already suffices
    (BD-330); the single repair cycle's contents and the `debug-only` switch (BD-331); the reduced chart, the
    `cold-start` step plan and the equivariance precondition (N22a-29); step sizes, tolerances, the per-column
    classification and the cap on selections (N22a-30); the leak check and the fallback to a full-stride problem
    (N22a-31); attribution of a multiplier to a temporal index of H/K and the unit-circle guard (N22a-32); the
    continued-flow device and validation rule (N22a-33); the split perturbation and δ (N22a-34); the bisection,
    the size cap and the recovery test's sample (N22a-35); the two-valued standing and its registry switch
    (N22a-36); stopping at the first stability-changing point unless asked to continue (N22a-37); the composition
    of the 100-case suite (BD-334); the catalogue, controls and comparison arm of GATE-GAIT-03 and the rule that an
    orbit-less child is unranked (BD-335); the compass gait's parameters, whose truth comes from the oracle and is
    not quoted (N22a-33, N22a-38).
15. **Fourteen titles of the earlier files run to 131 to 153 characters**, above the charter's "about 110". They
    were left as written under the instruction not to rewrite complete files; the new files are at 99 to 128.
16. **Bead count.** 54 is above the typical range. The unit holds thirteen seed beads (four of them algebra beads
    of another chapter), three Frontier mechanisms that each need a pilot, an experiment and an end-to-end script,
    and §22.9, whose seven parts are distinct mechanisms with no seed bead of their own in this unit.
