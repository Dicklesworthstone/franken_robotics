# Consolidated LEDGER blocks from the Phase 1 part files


# ===== P00.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

- Bets: BET-13 ten-fold on the sealed suite; BET-14 the mechanisms need this engine (composed-incumbent tie rule);
  BET-15 family policies; BET-16 inheritance; BET-17 rank agreement between rungs; BET-18 mixed-topology batches;
  BET-19 mission queries; BET-20 region certificates; BET-21 blinded qualification; BET-22 minimal counterexample
  worlds; BET-23 interventional autopsy stability; BET-24 requirement language; BET-25 energy-budget safety tier;
  BET-26 gait orbits as screening; BET-27 sampling controller as fallback; BET-28 calibration bands; BET-29 generative
  structure; BET-30 equivariance under the observation-preserving subgroup; BET-31 campaign quotienting; BET-32
  rare-failure bounds; BET-33 revalidation; BET-34 referee coverage; BET-35 two machines; BET-36 living worlds change
  decisions; BET-37 the building is a design variable. BET-01 to BET-12 carried with corrected wording and two added
  columns.
- Goals: GOAL-21 to GOAL-43. GOAL-01 to GOAL-20 carried; wording corrected on GOAL-02, -06, -08, -10, -11, -12, -13,
  -14, -15, -16, -17, -18, -19.
- Non-goals: NONGOAL-09 to NONGOAL-16 new. NONGOAL-02, -03, -04, -05, -07 narrowed (identifier kept). NONGOAL-01, -06,
  -08 unchanged in substance.
- North-Star scenarios: NS-13 to NS-24. NS-01 to NS-12 carried with corrected denominators; NS-06 retitled "Thirty
  iterations in an hour, three hundred designs decided".
- Differentiators: N1 to N13 (§1.6). N1 to N5 keep their version 0.2 subjects; N6 to N13 are new labels for the
  remaining audited wordings.
- Hypothesis verdicts: H-01 to H-24, with states, in the register of §0.8.
- Prefixes registered in §0.3: `N`, `H-`, `KA-`, `FMT-`, `cluster.verb`. Gate subsystem codes registered: AUGUR,
  PHYLUM, NEMESIS, AUTOPSY, GROUND, FOUNDRY, SHIELD, GAIT, QUAL, TENFOLD.
- Types, intent only, in `frob-core`: `ClaimProduct`, `ClaimKind`, `EvidenceClass`, `NoAuthorization`; flag names
  `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified`; condition flags `tier-sensitive`,
  `controller-sensitive`, `model-conditional`.
- Gates, kill gates, invariants, proof obligations, open questions, refusal codes, formats, verbs: none defined here.
- Existing identifiers cited: GATE-SPARK-01 to -05, GATE-FID-01, GATE-ATEL-01, GATE-ATEL-03, GATE-REFLEX-02,
  GATE-CORTEX-03, GATE-LEMMA-01, INT-FSIM-01, INT-TOOLCHAIN-01, INT-FMCP-01, KILL-KIR, KILL-CONTACT, KILL-FSIM,
  KILL-STATIC, KILL-BASELINE, KILL-EQUIV, KILL-METAL, KILL-LEMMA, A-15, D-2, CL-2, K-se3-cay-step.
- Verbs cited, not defined: `design.check`, `design.bisect`, `sim.run`, `sim.replay`, `eval.compare`, `eval.qualify`,
  `prove.kernel`, `mission.synthesize`, `result.revalidate`.
- Formats cited, not defined: `.fmcw`, `.fenergy`, `.fbuild`, `.frobcert`.
- Crates touched: `frob-core` (the claim-product type). The subsystem-to-crate table of §0.5 assigns every charter
  crate to a subsystem.

**Needs from other parts**

- P13 (§38 to §40): a gate identifier and threshold for each of BET-13 to BET-37; the decision rule of the ten-fold
  test on a pooled ratio with at least ten paired campaigns per arm and a power analysis against a true ratio of 10;
  the M3 partial-gate rule (under 3 withdraws the speed claim for the class; between 3 and 10 the ratio is reported
  and the word ten-fold is not used); the adjudicating evaluator; KA numbers for the stance family (NS-13), the
  docking fixture (NS-22) and the quantifier-order fixture; reclassification of the version 0.2 "method" gates as
  Design comparison.
- P01 (§12): evidence classes in the order Proved, Certified, Tested, Estimated; family certificates as Proof claims
  of class Certified; `frob-verify` refusing a bundle with missing assumptions; a record of whether each consumed
  certificate changed a decision.
- P03 (§16, §17): the referee coverage gate (90% of standard-battery scenarios at M2 exit, sampled coverage of at
  least 0.1% of GPU world-seconds); the head-to-head of §17 with per-family templates and the incumbent mixed batch,
  the two numbers of N11, the 1,024-variant digest test at no more than 1.3× the homogeneous step, and the
  edit-to-first-GPU-step measurement.
- P05 (§21, §35, §36): the batch-composition invariance test; guarded single precision with exact fallback; throughput
  targets restated as ratios to the incumbent measured on `trj`; the two-machine gate (1.3×, 1 GB per campaign-hour).
- P06 (§22, §23): the rank-agreement gate (Kendall τ 0.7 and 0.5, fixed before data), the family-policy gate (70%
  zero-shot, payback within 200 decisions), the inheritance gate (20% of steps, ten-generation plasticity control),
  the sampling fallback gate, the orbit screening gate, the shield gate (85% return, 5% overhead), the Γ_obs
  experiment.
- P08 (§24, §26, §27): the design benchmark that BET-10 runs on; the qualification gate (500 adaptive retries, power
  0.8 on planted gains); requirement-language conformance; the planted-trap gate (3 × 10⁻⁵, 3×, 70% with witness, at
  most three edits); the rare-failure bound gate; the autopsy gate (90% top-1, placebo flip rate 0.2, one third of
  repair turns).
- P07 (§25): spellings of the mission verbs; the family strata fixtures; the declared maximum unresolved band; the
  midpoint counterexample as a negative control.
- P04 (§19, §20): the retrofit corpus and its 30% gate; the film-law known answers and the 20% ranking gate; the
  observation-alias witness; cited sources for prices and dimensional norms.
- P09 (§28, §29): the memo graph and shadow recompute; the revalidation gate (no stale verdict, missed withdrawals,
  unnecessary invalidations); the quotienting gate (no false equivalence, planted winner, six of eight); the
  model-form rank check.
- P10 (§30, §31): the reality-band gate as an import lane; the build-packet checks.
- P11 (§32, §37): the claim product in every output envelope; the three flags on every export; `frob-mcp` on
  fastmcp_rust with a CLI-only fallback.
- P12 (§33, §34): admission tests and upstream asks for the four reported foundation defects of §1.7; INT-TOOLCHAIN-01
  covering the `fs-qty` nightly coupling; the conditional `frob-units` crate.
- P14 (§41 to §44): open questions for the compute envelope of the rank-agreement and family-policy experiments, for
  the pre-registration of the τ thresholds, for the neutral adjudicator, and for the price and norm catalogue; the
  milestones at which the [M] items of GOAL-43 land.
- P16 and the lead: Appendix K must hold the pilot results cited here as "FACT, Appendix K" (the 1.345× measurement
  with its 1.46× ceiling; the 64-pair rule with no reachable fourth verdict before 86; the 18% power of the draft
  decision rule; the guarded single-precision boundary screen; the 675 of 675 and 0 of 225 probe result; the
  dependency-graph tie; the exact family answers reproduced by standard solvers). Appendix H must register every
  external figure used in §1 and §2. Appendix C must hold the strong forms of the narrowed and refuted hypotheses with
  their refuting calculations. Appendix N must hold the quantifier-order fixture, the midpoint counterexample, the
  coupled-bus fixture, the pre-slip aliasing fixture and the omitted-law fixture.

**Decisions taken here that the charter did not settle**

- The verdict register is printed in compact form in §0.8, since no appendix was assigned to it.
- Family certificates are Proof claims of class Certified; the version 0.2 "method" gates name the Design comparison
  claim.
- The three no-authorization flags are named as above.
- Differentiators are labelled N1 to N13 and the prefix `N` is registered.
- BET-32 to BET-37 were added beyond the charter's list of new bets.
- The wording rule for a measured ratio between 3 and 10, and the positioning rule under 3, are stated in BET-13.
- Kendall τ thresholds of 0.7 and 0.5 are used as TARGETs for BET-17; the inputs also proposed 0.6 and 0.3.
- Two HTML comments mark where the lead inserts the PART 0 and PART I headings.


# ===== P01.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

- Doctrine rules: D-9, D-10, D-11, D-12, D-13, D-14, D-15, D-16, D-17, D-18, D-19. D-2, D-5 and D-6 carry one
  added sentence each.
- Constructions: C-16 hybrid orbits, saltation, return maps, labeled derivatives; C-17 quantified mission
  semantics (F, I ⊆ F ⊆ O, proof rules); C-18 assumption lattice, polarity, design–world duality; C-19 causal
  policies, indistinguishability, docking trade-off, label count, schedule classes; C-20 observability Gramian
  modulo gauge; C-21 similarity group and canonical form; C-22 campaign quotients (orbits, Burnside, wreath
  products, mission-observable projection); C-23 family policy as a section, link tokens, transport along edits;
  C-24 geometric-algebra tree policy; C-25 coupled fidelity estimators (control variate, multilevel, doubly
  robust correction); C-26 rank agreement and conformal survivor sets; C-27 rare events; C-28 minimal
  counterexample worlds; C-29 counterfactuals on a forkable simulator; C-30 temporal-logic requirements; C-31
  energy budgets, tube inclusion, runtime filters; C-32 region certificates and dual witnesses; C-33 two-sided
  clearance; C-34 linkage synthesis (exact lane, continuation lane); C-35 topology, fatigue, tolerance; C-36
  trapezoidal variational step and nonholonomic step; C-37 four-dimensional contact cone and joint rows; C-38
  port-Hamiltonian actuators with a thermal node; C-39 value of information and probe design; C-40
  product-system dominance. C-09 is retitled ("two symmetry groups") and rewritten; C-01 to C-08 and C-10 to
  C-15 carry corrections and added limits.
- Theorem cards: T-15 to T-35. T-04, T-07, T-09, T-10 and T-12 carry amended wording.
- Gates: GATE-LEMMA-03, GATE-LEMMA-04, GATE-LEMMA-05, GATE-LEMMA-06 (GATE-LEMMA-00 to -02 carried, with
  milestones M0, M5, M5).
- Kill gates: KILL-REFLECT, KILL-FAMILYCERT (KILL-LEMMA carried).
- Decisions: ADR-LEMMA-01 to ADR-LEMMA-06.
- Invariants: INV-LEMMA-01 to INV-LEMMA-06.
- Proof obligations: PO-LEMMA-01 to PO-LEMMA-04 (PO-MARROW-01 is extended in scope to §11.2–§11.6).
- Open questions: OPEN-LEMMA-2 to OPEN-LEMMA-6 (OPEN-LEMMA-1 carried).
- Refusal codes, provisional numbers chosen high in each family to avoid collisions with other parts:
  FROB-REF-168, FROB-REF-263, FROB-REF-823, -862, -863, -864, -865, -866.
- Formats: `.frobcert` (schema `frob.cert/1`, FMT-FROBCERT): field list, rules and the closed proof-kind
  registry are defined in §12.4. Content of `.fenergy` is named in C-31, of `.fmcw` in C-28, of `.fclear` in
  C-33, of `.fmodule` in C-12; their schemas belong to the owning chapters.
- Proof kinds: `poly-identity`, `rational-ldlt`, `interval-sign`, `interval-root`, `farkas-dual`,
  `conic-dual-strict`, `witness-replay`, `partition-cover`, `monotone-transport`, `complex-connectivity`,
  `tube-inclusion`, `rewrite-trace`, `orbit-count`, `group-enumeration`, `energy-budget`, `alias-witness`,
  `product-unreachability`, `alpha-root`, `lean-capsule`, `statistical-receipt`.
- Derivative-validity labels: `smooth`, `saltation`, `discrete-step`, `fixed-mode`, `one-sided`, `generalized`,
  `smoothed`, `finite-difference`, `refused`.
- Verbs: new `prove.bundle`, `prove.reflect`. Used and owned elsewhere: `prove.kernel`, `prove.verify`,
  `prove.list`, `mission.synthesize`, `mission.refute`, `mission.distinguish`, `world.separate`,
  `result.revalidate`.
- Type sketches (DESIGN): `Class`, `Evidence<K>`, `ClaimProduct<A, B>`, `NoAuthorization`; `TwistMap`,
  `WrenchPushforward`, `WrenchPullback`.
- Crates touched: `frob-lemma`, `frob-cert` (standard library only; holds the `frob-verify` binary),
  `frob-lie`, `frob-algebra`, `frob-kir`, `derive/frob-derive`, `lean/`.

**Pilots named here that need a gate in the owning chapter**

- §16: C-16 (100 cases, 20 controls refused, 72 of 80 at cosine 0.95); C-36 (variational step at most twice
  the cost at matched accuracy, else referee and strict profile only).
- §17, §28: C-21 exact power-of-two similarity test; C-22 ballast projection replay.
- §19: C-33 ladder, doorway and one interior at two resolutions.
- §22: C-10 orbit ranking against rollout ranking at Kendall τ 0.5; C-31 tray-cart budget and filter KKT check.
- §23: C-09 three-arm equivariance pilot; C-23 family policy (zero-shot 70%, fine-tune 0.2) and 12 edit pairs;
  C-24 three arms with kill at 1.3×.
- §24, §28: C-25 ρ on 200 addresses with kill at 0.7; C-26 the forty-design rank-agreement experiment with kill
  at τ 0.5 or ρ 0.7; C-39 thirty registered experiments with coverage between 65% and 92%.
- §25: C-17 first mission slice against generic comparators; C-18 twelve scripted amendments; C-19 docking grid
  against a scenario-tree linear program; C-32 twelve-parameter paving (kill at 90% undecided); C-40 one contact
  transition against a reachability tool.
- §26, §27: C-27 random-walk fixture and planted trap; C-28 planted traps with 70% solvability witnesses; C-29
  planted causes at 90% top-rank recovery.
- §28: C-08 funding rule for the shape-space atlas; C-34 lane 1 (24 templates) and lane 2 (Stewart–Gough, kill
  at 0.1% lost paths).
- §31: C-35 one link through topology, fatigue and tolerance (7% mass saving, 5% window stability).

**Known answers listed here without KA numbers (for P13)**

Sarrus and Bennett mobility; planar 2R fixture (J, torques −3 N·m, −0.3 W); mass-support and shell fixtures;
pyramid-cone √2 exploit; growth vectors; constant-twist chord (0, 2/π); Elroy's beanie; SO(3) curvature sign
loop; sagittal-mirror parity; two-latch fixture; bouncing-ball saltation; rimless-wheel multiplier cos²(2α);
two-ball commuting identity; 27 strata and 162 transitions; docking witness and ε < (U + ½)/4; majority votes
27/32, 459/512, 3807/4096; three-polytope label count; 720 orderings in 8 classes; two-anchor ambiguity;
pendulum group count; Burnside 86,185 and 500,500 and 92; wreath count (k² + k)/2; ballast 735,471 to 24,546;
control-variate and doubly-robust arithmetic; conformal index 38 of 39; zero-failure bound; splitting
coefficient of variation; ddmin {3, 6}; Shapley (½, ½) pair; unit-normalization example; energy budget 1.49 J
of 2.00 J and 50.5 ms; sill exclusion 0.94176 J; filter (1, 0.5); stance 3/14 m with (160, 130, 110, 0) N and
the dual witness at 1/5 m; passive springs (175, 115, 85, 25) N; torque sign (30, −60, 0); ladder 2√2·a;
circumcentre and Grashof fixtures; continuation root counts; two-bar truss 45°; damage 0.5 + 0.0625; rainflow
residue; tolerance 0.0447 and 0.0894 mm; Cayley differential and four-step free-body tests; Chaplygin sleigh
energy; spin coupling 1/√2 and r = 2R/3, 3πR/16; stiction and backlash; RL time constant 10 ms; critical
current 35.8 A; probe fixture with error 0.1; dominance interval [2/5, 3/5); e-process table 16 and 22;
Clopper–Pearson interval at n = 64; clock example L = 1,200; impulse 20 N.

**Counterexamples for the catalogue (Appendix N)**

Lower three rows of a space Jacobian; reset Jacobian used as event derivative; wrench moved by the twist map;
angular velocity treated as polar; +[𝔸ᵢ, 𝔸ⱼ] in the curvature; pyramid friction cone; ideal rolling for a
skid-steer base; bracket rank as reachability with one-sided input; Γ_mech used where Γ_obs is required;
deterministic tied policy on the two-latch task; orbit search in Fix(K) without the full monodromy; convex hull
of two feasible designs; per-world answers intersected; pairwise compatibility taken as joint; hash match taken
as verdict transfer; matching some dimensionless groups; file-order tokens; survivor-only calibration;
defensive mixture as a discount on certification; fork without fresh randomness; 1-minimal read as minimum;
Shapley value read as necessity; unnormalized minimum over mixed units; projection then clipping; ∀w ∃u in
place of ∃u ∀(w, e); omitted energy source; first moment passed as torque; independent-force inclusion applied
to a passive suspension; support polygon with total capacity; scalar clearance radius; target poses on
different circuits; rainflow without residues; momentum test on a nonholonomic system; right-trivialized
differential in the variational solve; heat counted as stored and lost; outcomes enclosed separately in a
dominance check; products coupled by tick instead of position; reassociation filed as a bit-exact rewrite.

**Needs from other parts**

- P00: the seven claims of §8 as the kinds K of `Evidence<K>`; BET-11 and GOAL-13 keep their meaning; the
  thesis of §2 names avoided trainings as its largest term (C-23 refers to it).
- P02 (§13–§15): PO-MARROW-01 covers the formulas of §11.2–§11.6; `frob-algebra` computes Γ_mech and Γ_obs,
  wreath products and subgroup lattices; the design term has a wall-thickness field, a mass-envelope field and
  the `.fmodule` invalidation contract of C-12; canonical scales m₀, ℓ₀, t₀ are computed at model compile;
  actuator cards carry the thermal node, the three braking modes, stiction and backlash parameters.
- P03 (§16–§18): derivative labels and FROB-REF-263 at kernel outputs; the variational step in the referee and
  strict profile; the power-of-two similarity test and orbit deduplication in family batches; the
  four-dimensional cone in the `Cone` trait; nonholonomic rows; the hard-contact law as specification.
- P04 (§19–§20): inner and outer complexes with a reported band; the retrofit search consuming `.fmcw`; the
  seven instants, the clock contract and checkpointed packet queues; the Gramian analysis and global ambiguity
  search.
- P05 (§21, §35, §36): the exact-integer kernel class and guarded `f32`; rewrite classes E and R; the
  even-exponent rule for square roots; the batch-independence replay; adoption of the refusal codes above.
- P06 (§22–§23): gait orbits and return maps on the baseline controller; the `.fenergy` monitor and runtime
  filters; family policies, link tokens, transport, the pilots and kills of C-23, C-24 and C-26.
- P07 (§25): the mission verbs, one bundle per cell, polarity in `result.revalidate`, and the link from
  KILL-FAMILYCERT to the partial ten-fold gate.
- P08 (§24, §26, §27): requirement language, the reachability pre-analysis of fixed-n studies, α spending, the
  two rare-event lanes, minimal counterexample worlds, autopsy with placebo forks.
- P09 (§28): analyses that expose C-16, C-20, C-22, C-26 and C-39; the rank-agreement gate.
- P10 (§30–§31): the Reality lane behind the `sim_to_real_certified` flag; the structure pipeline of C-35.
- P11 (§32): registration of `prove.bundle` and `prove.reflect`; the three flags on every tool response.
- P13: KA numbers; gate-register rows for GATE-LEMMA-03 to -06, KILL-REFLECT and KILL-FAMILYCERT; the
  negative-control registry seeded from the counterexample list above.
- P14: adoption of OPEN-LEMMA-2 to -6; the D-6 cap against the list of admitted Moonshots.
- P16: Appendix D rows for the formulas of §11 and C-16 to C-40; Appendix H entries for every FACT quoted from
  a publication or a planning pilot (family-policy transfer figures; inheritance costs; re-description loss;
  equivariance and augmentation results; geometric-algebra network timings; orbit solve times; specialization
  ceiling; rank-correlation figures; rare-event savings; value-of-information gains; topology-optimization
  savings; reuse share after an edit; continuation root counts; and the pilot results cited in D-17 to D-19,
  C-12, C-17, C-18, C-32 and C-39).

**Decisions the charter did not settle, as written here**

1. Kernel reflection is tiered [F], following G9; the summary list in the charter's §10 files it under
   Moonshot.
2. The `frob-verify` binary is placed in `frob-cert`, which is therefore standard-library only and carries its
   own BLAKE3.
3. Continuation-based synthesis is [M] on the CPU, following the charter; the verdict register reads Frontier.
4. The seven instants are named as in G8; their definitions and the half-open window (a, b] are fixed in §11.8.
5. `.ftube` and `.fquot` are not in the format list, so tube tables are a proof kind inside `.frobcert` and the
   mission-observable projection is a memo key, not a file format.
6. D-6 keeps the cap of four active Moonshots; with about a dozen admitted, the rest queue in Appendix C.
7. Refusal numbers and the square-root typing rule added to T-09 are provisional until P05 adopts them.


# ===== P02.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

Decisions: ADR-MARROW-05, -06, -07, -08, -09; ADR-SKEL-01 to ADR-SKEL-09; ADR-SINEW-01 to ADR-SINEW-08.

Gates: GATE-MARROW-03, GATE-MARROW-04, GATE-MARROW-05; GATE-SKEL-04, -05, -06, -07, -08; GATE-SINEW-04, -05, -06,
-07, -08, -09. Carried unchanged: GATE-MARROW-01, -02; GATE-SKEL-01, -02; GATE-SINEW-01, -02, -03. Carried with a
wording change: GATE-SKEL-03 ("each template" became "each of the eighteen archetypes", stages 1 to 6, symmetry
groups on file, "stands or rolls").

Kill gates: KILL-MARROW-BLOCKS, KILL-MARROW-GUARD, KILL-SKEL-SUBST, KILL-SKEL-PORT, KILL-SKEL-ALIAS,
KILL-SKEL-SEXPR, KILL-SINEW-ROWS, KILL-SINEW-AGE, KILL-SINEW-ROTOR.

Invariants: INV-SKEL-04 to INV-SKEL-09; INV-SINEW-01 to INV-SINEW-06. Carried: INV-SKEL-01, -02, -03.

Proof obligations: PO-MARROW-04, -05, -06 (PO-MARROW-01 extended to the bracket and the group coadjoint);
PO-SKEL-01 to PO-SKEL-04; PO-SINEW-01 to PO-SINEW-03.

Open questions: OPEN-MARROW-2, OPEN-MARROW-3; OPEN-SKEL-2, OPEN-SKEL-3, OPEN-SKEL-4; OPEN-SINEW-2, OPEN-SINEW-3,
OPEN-SINEW-4. Carried: OPEN-MARROW-1, OPEN-SINEW-1. Closed and tombstoned: OPEN-SKEL-1.

Refusal codes: FROB-REF-213, -214, -215 (Marrow, family 2xx; FROB-REF-212 carried). FROB-REF-102, -103, -105,
-106, -108, -109, -111, -112, -114, -115, -116, -118, -119, -120, -121, -122, -123, -124, -125, -126 (Skeleton,
family 1xx; 101, 104, 107, 110, 113 carried; 117 and 127 to 129 left free). FROB-REF-130 to -136 (Sinew, family
1xx). The lead should check 213 to 215 against whatever the Kinesis and numerics parts allocate in 2xx.

Formats: FMT-FBOT (`.fbot`, schema `frob.design/1`, with the S-expression view); FMT-FMODULE (`.fmodule`,
schema `frob.module/1`); FMT-FCARD as used here (`.fcard` actuator, battery and transmission cards); the patch
document schema `frob.design.patch/1` (no extension of its own).

Verbs: `design.patch`, `design.view` (new); `design.check` extended with `--task`, `--scene`, `--stages`,
`--all`; `design.templates` now lists the eighteen archetypes; `module.list`, `module.show`, `module.check`,
`module.substitute`; `algebra.irreps`, `algebra.orbits`; `analyze.symmetry` now returns both groups and the
breakers; `actuator.list`, `actuator.show`, `actuator.check`, `actuator.envelope`, `actuator.thermal`,
`actuator.bus`.

Crates touched: `frob-lie`, `frob-algebra`, `frob-design`, `frob-model`, `frob-io`, `frob-actuate`; consumers
named: `frob-kir` (predicate classes, the select lint, error-bound derivation), `frob-analyze` (preflight stages
2 to 8), `frob-dyn` and `frob-contact` (row assembly, rotor elimination).

Worked cards proposed for Appendix E (the lead assigns GB numbers): the fill rule (§14.5); thermal runaway
(§15.7); the voltage limit (§15.8); the bus ceiling (§15.10). Counterexamples proposed for Appendix N: reversed
ad* that conserves energy; forward lidar and 𝔾_obs; the two latches; C₃ treated as real type; single-precision
0.3, 0.4, 0.5; mecanum with parallel rollers; peak-adding contracts; the two beacons; the single-multiplier
gear; the semidefinite limb packet.

Executed checks behind the numbers: every worked number in this part (thermal card, shell masses and gyration
radii, orbit counts, voltage closed forms, bus ceiling, gear fixture, backlash impact, the single-precision
predicate) was recomputed by script for this part; none is a measurement of FrankenRobotics code.

**Needs from other parts**

- §11 (P01): the conventions table must carry the coadjoint rows of §13.3 unchanged: ad* is the transpose of ad;
  the group coadjoint is the inverse transpose of Ad; μ̇ = ad*_ξ μ + w. Also the rule that the time convention is
  rational periods, which §15.5's slower thermal clock assumes.
- §12 (P01): two candidate theorem cards, the rational gear law (PO-SINEW-03) and passivity of an
  interconnection of passive elements (PO-SINEW-01); T-11 is used to check joint class against screw basis.
- §10 (P01): C-09 should define the group as 𝔾_mech with 𝔾_obs as its observation-preserving subgroup, and C-03
  should mention mass support beside positivity.
- §16 (P03): assembly of joint rows with contact rows; rotor elimination inside the dense solve; the settled
  pose per `ModelId`; the tree-operator route that consumes limb port packets; the sampled referee that re-runs
  worlds flagged by KILL-MARROW-GUARD's fallback.
- §17 (P03): size buckets and the variant record that §14.9's fourth lowering fills.
- §18 (P03): the contact laws named in `.fbot` (`foot-patch`, `wheel-rolling`, `roller-wheel`, `belt`, `skid`,
  `pad`), the four-dimensional cone, interface cards, and the belt-law known answer for skid turning.
- §19 (P04): clearance complexes and the retrofit catalog that stage 7 calls.
- §20 (P04): sensor cards and the clock contract; the quantized transcript that stage 8 compares.
- §23 (P06): BET-07's gate for equivariant policies must use 𝔾_obs and the block parameterization of §13.5.
- §24 (P08): task terms must declare their transformation kind, the finite probe family and world family for
  stage 8 (OPEN-SKEL-3), and required static margins and heights for stages 6 and 7.
- §28 (P09): A-01 to A-16 keep their meanings; A-11 uses the temperature-dependent steady state; A-03 returns
  both groups; GATE-ATEL-02 reports per-stage medians and 95th percentiles; the card-conditional label of §15.12
  comes from A-16; the lineage graph stores patches.
- §29 (P09): the energy ledger's lines follow §15.11 (stored heat is stored; braking modes separate; first
  offending event).
- §32 (P11): atomic idempotency binding for `design.patch`; the new verbs in the registry and their clusters.
- §33 (P12): admission tests for `fs-ir`'s reader, `fs-qty` parsing, `fs-phs` battery and interconnection,
  `fs-tribo`'s set-valued friction, `fs-control`'s conventions; the `planar_drive` defect as an admission test.
- §34 (P12): the common artifact envelope; the format registry entries above; whether the schema name for
  `.fbot` stays `frob.design/1` (kept here as a carried identifier) or becomes `frob.fbot/1`.
- §35 (P05): the exact-integer kernel class and the decision-level equivalence test that §13.7 relies on;
  refusal family ranges.
- §38 (P13): KA numbers for the known-answer tables of §13.10, §14.14 and §15; the negative-control registry
  entries for the rejection fixtures of §14.13 and the mutation rule of §13.8.


# ===== P03.md =====
<!-- LEDGER -->

**Crates touched.** `frob-dyn`, `frob-kernels` (§16); `frob-phylum` (§17, new); `frob-contact` (§18). Used
through the airlock: `fs-mbd`, `fs-contact`, `fs-time`, `fs-query`, `fs-tribo`, `fs-eproc`, `fs-uq`, `fs-rand`,
`fs-archive`, `fs-race`, `fs-recompute`.

**Decisions.** ADR-KIN-01 to ADR-KIN-07; ADR-PHYLUM-01 to ADR-PHYLUM-06; ADR-TRAC-01 to ADR-TRAC-08.

**Gates, new.** GATE-KIN-04, -05, -06, -07; GATE-FID-03, -04, -05, -06, -07, -08; GATE-PHYLUM-01 to -08;
GATE-TRAC-04 to -10. **Gates, kept with their meaning.** GATE-KIN-01, -02, -03; GATE-FID-01, -02; GATE-TRAC-01,
-02, -03 (GATE-TRAC-01 now also runs on the `strict` profile).

**Kill gates, new.** KILL-DR, KILL-RANK, KILL-SHADOW, KILL-VARIATIONAL, KILL-TREEOP (§16); KILL-PHYLUM,
KILL-FAMILYLANE, KILL-SPECIALIZE, KILL-FDGRAD, KILL-QUOTIENT (§17); KILL-FILM, KILL-HARDCONTACT, KILL-MESHGPU,
KILL-RESPONSE, KILL-SOIL (§18). **Kept.** KILL-CONTACT.

**Bets.** None defined here. Candidates for P00: the cohort bet behind GATE-PHYLUM-03 and KILL-PHYLUM; the
rank-agreement bet behind GATE-FID-04.

**Invariants.** INV-KIN-01 to INV-KIN-08; INV-PHYLUM-01 to INV-PHYLUM-07; INV-TRAC-01 to INV-TRAC-07.

**Proof obligations.** PO-KIN-01 to PO-KIN-05; PO-PHYLUM-01 to PO-PHYLUM-03; PO-TRAC-01 to PO-TRAC-04.

**Open questions.** OPEN-KIN-1, -2 (kept); OPEN-KIN-3, -4, -5, -6; OPEN-PHYLUM-1 to -5; OPEN-TRAC-1 (kept);
OPEN-TRAC-2 to -5.

**Refusal codes (provisional numbers inside the family ranges; the lead may renumber).**
- 1xx: FROB-REF-160 no shape class; -161 group not observation-preserving; -162 reuse without a dependency
  digest.
- 2xx: FROB-REF-220 event refinement limit; -221 transaction exhausted; -222 profile not available on tier; -223
  no refinement law; -224 indeterminate contact; -240 invalid cone; -241 no rotor state; -242 film law out of
  domain; -243 soil or tyre law out of range; -244 stiffness beyond single precision; -245 half precision; -246
  law not admitted. FROB-REF-212 kept.
- 3xx: FROB-REF-320 thin feature; -321 no enclosure.
- 5xx: FROB-REF-540 gradient unreliable; -541 outside generator support.
- 7xx: FROB-REF-721 referee capacity; -722 capacity overflow refused; -740 population exceeds device; -741 bucket
  fragmentation.
- 8xx: FROB-REF-820 label misuse; -821 envelope not frozen; -822 capacity-biased verdict.

**Formats.** None new. The clearance certificate of §18.3 travels in the `.frobcert` bundle (P01 owns the
format). `GeneratorId`, `ShapeClassId` and `StepPlanId` are new content-hash identities.

**Verbs.** New: `fidelity.ledger`, `fidelity.rank`, `fidelity.modelform`, `fidelity.energy`,
`fidelity.coverage`, `fidelity.shadow`; `phylum.plan`, `phylum.sweep`, `phylum.generate`, `phylum.quotient`,
`phylum.reuse`, `phylum.gradient`, `phylum.map`; `contact.laws`, `contact.probe`, `contact.capacity`,
`contact.audit`, `contact.clearance`, `contact.soil`, `contact.response`. Extended: `sim.run` (`--profile`,
`--law`, `--designs`), `sim.estimate` (capacities, bytes, expected overflow and coverage), `eval.promote`
(`--method`).

**Kernels named (for the kernel library of P05).** `K-np-thin`, `K-joint-rows`, `K-film`, `K-scale-diag`,
`K-ledger-step`; the existing stage kernels are unchanged in name.

**Labels a result can carry (for P09 and P13).** `provisional`, `established` with adjudicator `referee` or
`batch-cpu:strict`, `provisional: tier-sensitive`, `model-conditional`, `energy-suspect`, `capacity-biased`,
`truncated-horizon`, `extrapolated`; world states `Degraded`, `Degraded: unconverged`, `Faulted` with cause
`numerical-fault`.

**Known answers for P13 to number.** The tables of §16.16, §17.18 and §18.16. **Worked cards for Appendix E.**
Padding and buckets (§17.5); the promotion table (§16.8); squeeze-film times (§18.8). **Counterexamples for the
catalogue.** Redraw bias (¾); tied sliders; capacity-shaped reduction tree; position-level correction creating
0.125 J; pyramid cone √2·μ; lidar and the mechanical group; equal Σx with different Σx²; coupled bus; 6 mm gap
with 4 mm errors; overlapping sweeps at different times; thin barrier between clear endpoints; independent
slide and spin rows; backlash as a torque dead zone; skid-steer under no-slip; scaling does not improve
Cholesky; the half-precision calculation.

**Needs from other parts.**
- P00: GOAL-02 to read "thousands of worlds of up to thousands of different designs per device"; bets for the
  cohort and for rank agreement; NS-06 restated as designs decided per hour if the lead adopts that unit.
- P01: §11 to list `ShapeClassId`, `StepPlanId`, `GeneratorId` and the profile names (`lockstep`, `strict`,
  `referee`); the coadjoint sign convention fixed once (§16.7 depends on it); T-07 extended to the
  four-dimensional cone; the clearance certificate as a record kind in `.frobcert`; the claim product applied to
  result labels.
- P02: Γ_obs (the observation-preserving subgroup) defined in §13 with its action on module slots; exact
  rational roll-up of welded sub-assemblies and canonical link order in the lowering (§17.11 depends on both);
  the path table for sparse Jacobians; actuator card levels that carry rotor state, stiction torque τ_s and
  backlash β as card fields; parameter retractions for §17.13.
- P04: the clock contract (seven instants, boundary ordering, packet queues in checkpoints); the film field and
  its Frontier advection writing the field §18.8 reads; thin-feature detection, the bounding-volume hierarchy
  and per-brick range scaling in scene packs; registration error bounds for enclosures; building-scale
  clearance complexes calling `contact.clearance`.
- P05: the kernel names above in the library; determinism classes for pairing refusals; the exact-integer and
  guarded-predicate kernel classes for enclosure tests; §36 workloads matching the capacity table of §16.6, with
  world-steps and env-steps defined there; the compute envelope (OPEN-SPARK-2) that the head-to-head and
  pretraining need; launch-overhead measurement per step plan at M2.
- P06: family policies trained on a `GeneratorId` with a `DesignSplit`; frozen policy weights for the background
  referee's episode replays; explicit-reset semantics for stage 10.
- P08: the standard battery including wet-floor, thin-barrier, sill and tracked scenarios; the mutation corpus
  and design corpus used by §17.14; the fixed-n study type for the head-to-head.
- P09: the memo graph as the store for mission-digest results and the shadow-recompute rule; the conformal
  quantile q̂ used in §16.9; the energy ledger's residual line and contact-mode tags; `witness-reset` and
  coverage fields in telemetry; `numerical-fault` kept apart in the failure taxonomy.
- P10: `model-conditional` and `capacity-biased` blocking build packets and prototype spend; wear models that
  grow backlash.
- P11: `frobd` slot-level dispatch policy and the `--dispatch per-job` fallback; cluster limits for the three
  verb clusters above.
- P12 and Appendix F: upstream asks for a contact cap above 21 or an island-wise solve, a batch entry point for
  `fs-uq` multilevel estimation, witness migration in the sustained step, and the `planar_drive` near-straight
  defect; admission tests at INT-FSIM-01 for every limit in §16.4.
- P13: gate register rows and milestones for all gates above; negative-control registry entries for every
  mutation named in the known-answer tables; the head-to-head arms in the rival audit; measured incumbent
  edit-to-first-step latency.
- P14: adoption of the OPEN items; work packages for the transaction, cohorts, quotienting, the cone and joint
  rows, the film law and the mesh path; M2 exit to include GATE-FID-03 and GATE-PHYLUM-03.
- P16: source-register entries, each to be re-read from its primary source before a gate cites it: the
  incumbent step trace (53 of 262 ns; 38 ns constraint assembly); the mixed-batch rate of 0.86; incumbent
  documentation on batched model fields and heterogeneous worlds; cold and cached kernel compile times;
  published CPU engine rates per core; the per-world byte ranges; card and scene capacity figures; the
  half-precision emulation; condition-number estimates; zero-shot transfer ranges; population co-design gains;
  the contact-divergence horizon; pretraining costs; rank-correlation neighbours.



# ===== P04.md =====
<!-- LEDGER -->

**New or amended identifiers defined in this part**

Decisions
- ADR-HAB-01 to ADR-HAB-12 (§19.2)
- ADR-SENS-01 to ADR-SENS-12 (§20.2)

Gates
- GATE-HAB-01 (amended: brick bound, overlays, quantization ripple), GATE-HAB-02 (amended: feasibility tags replace "a
  traversable route for every seed"), GATE-HAB-03 (amended: a probe request emitted; `no-claim` where no capture
  exists)
- GATE-HAB-04 to GATE-HAB-14 (new)
- GATE-SENS-01 (amended: lever-arm and free-fall cases), GATE-SENS-02 (amended: masks and delivered-observation
  digests), GATE-SENS-03 (amended: five noninterference properties)
- GATE-SENS-04 to GATE-SENS-14 (new)

Kill gates
- KILL-LIVING, KILL-FILM, KILL-SLOSH, KILL-PROXY, KILL-DRESSING, KILL-INGEST, KILL-FCLEAR, KILL-RETROFIT, KILL-POOL,
  KILL-BANK-BREADTH
- KILL-ALIAS, KILL-SCHED, KILL-EXPOSURE

Invariants
- INV-HAB-01 to INV-HAB-10
- INV-SENS-01 to INV-SENS-09

Proof obligations
- PO-HAB-01 to PO-HAB-07
- PO-SENS-01 to PO-SENS-09

Open questions
- OPEN-HAB-1 (carried), OPEN-HAB-2 to OPEN-HAB-8 (new)
- OPEN-SENS-1 to OPEN-SENS-5 (new)

Refusal codes
- FROB-REF-301 (carried), FROB-REF-302 to FROB-REF-317 (new, scene family)
- FROB-REF-440 to FROB-REF-443, FROB-REF-445 to FROB-REF-447 (new; placed in the 4xx family as a contiguous Sensorium
  block; 444 left unused)
- FROB-REF-502 (carried, unchanged)
- Codes whose meaning is closer to another family, should the lead prefer to renumber: 310 (licence and privacy, 7xx
  or 8xx), 312 and 447 (evidence, 8xx), 314 (budget, 7xx), 311 (protocol, 5xx)

Formats and registries
- FMT-FSCENE: `.fscene`, schema `frob.fscene/1` (manifest sketch in §19.3)
- FMT-FCLEAR: `.fclear`, schema `frob.fclear/1` (cells: two 16-bit one-sided clearances in millimetres; witness and
  cut payloads)
- `.fcard` kinds used here and not yet named in the charter: `optical`, `sensor`, `payload`
- `registries/hazards/<archetype>.toml`, `registries/retrofits.toml`

Verbs
- Carried: `scene.gen`, `scene.compile`, `scene.import`, `scene.list`, `scene.show`, `telemetry.stills`,
  `telemetry.inspect-pixel`, `telemetry.export-html`
- New: `scene.accept`, `scene.hazards`, `scene.fit`, `scene.retrofit`, `scene.probe-request`, `scene.split`,
  `scene.bank`
- New: `sense.profile`, `sense.schedule`, `sense.causal-test`, `sense.observability`, `sense.alias`, `sense.symmetry`,
  `sense.calibrate`

Crates touched
- Owned: `frob-scene`, `frob-sense`
- Kernels authored here and housed in `frob-kernels`: `K-np-sdf` (now with overlays and the pooled directory),
  `K-ray-*`, `K-sense-*`, a film-update kernel
- Consumers named: `frob-contact` (film coefficient into the cone), `frob-model` (fixtures, payload modules, proxies),
  `frob-algebra` (group operations for 𝔾_obs), `frob-analyze` (registration of the observability and symmetry
  analyses), `frob-nemesis`, `frob-mission`, `frob-arena`, `frob-ground`, `frob-chronicle` (checkpoint contents),
  `frob-conformance` (fixtures), `viewer/`

Known answers (unnumbered; for `P13` to number)
- §19.18: 18 rows. §20.20: 27 rows.

Counterexamples for the negative-control registry
- Three-point registration accepts a reflection (§19.5.1)
- Nominal 6 mm gap with 4 mm error per side is `unknown` (§19.5.2)
- Inscribed and circumscribed disc tests are both wrong for a rectangle at a corner (§19.11)
- A fixed step is not a rug: the castor pushes the slab (§19.10.2)
- Masked non-conservative film transport loses volume (§19.10.3)
- A mirrored layout leaks across a split keyed by `SceneId` (§19.14)
- Every-thirteenth-step scheduling runs a 30 Hz policy at 30.77 Hz (§20.4.6)
- Independent dropout coins overstate the value of repeated reads (§20.6)
- A full-rank local Gramian in each of two identical aisles (§20.11)
- The T-junction alias (§20.12)
- 𝔾_obs of order two inside 𝔾 of order four; a spinning lidar breaks a reflection (§20.13)
- A triangle of sensors cannot localize a bad edge (§20.14)
- Equality at the docking boundary; a third gain reverses every admitted cell (§20.16)

**Corrections of earlier text made here** (for the lead's change log, and for the parts that hold the matching rows)
- Scene memory: a 2 cm warehouse is 4.8 GB (the 600 MB figure is 4 cm); a narrow band saves a factor of one to five at
  2 cm, not an order of magnitude. The worked card on scene memory in the appendix (GB-07) needs the same correction.
- "Quantized to 16 bits" is now a truncated field with a stated range (§19.4).
- GATE-HAB-02 no longer requires a traversable route for every seed.
- The time convention "sub-schedules are integer ratios of the physics tick" (conventions table, §11) and "substeps
  are integer ratios of the control tick" (§16) are withdrawn in favour of §20.4.
- Cameras gain exposure and rolling shutter at M2; radiometric RGB stays [F] and is not needed before M5.

**Needs from other parts**
- `P01` (§10, §12): C-09 should state that closed-loop consumers use 𝔾_obs (§20.13); theorem cards for PO-HAB-02,
  PO-HAB-03, PO-HAB-07, PO-SENS-05, PO-SENS-07, PO-SENS-08; the certificate bundle should admit the two `.fclear`
  payload kinds (path witness, component cut) and the alias witness; the conventions table (§11) should carry the time
  row "integer quanta of 1/Q s; rational periods with phase accumulators".
- `P02` (§14, §15): sensors, payload modules with sprung masses, chain cords and hinged plates are ordinary components
  and modules; `.fcard` kinds `optical`, `sensor`, `payload`; actuation latency on the actuator card (instant 7).
- `P03` (§16, §17, §18): the per-contact film coefficient μ of §19.10.2 enters the cone as a lagged per-step value;
  padded capacities must admit cord and rug islands; event-located steps on the referee and strict `batch-cpu` accept
  clock events as boundaries (`split`); rut fields stay Traction's and use the per-world mutable-state rule of
  §19.3.3; OPEN-TRAC-1 is related to OPEN-HAB-3.
- `P05` (§21, §35, §36): the placement planner groups launches by pool segment; the deterministic exponential is
  needed by the steam rule; the exact-integer kernel class and guarded single precision are first exercised by the
  docking fixture; performance rows for the ray stage with rolling shutter (at most 5% added) and for the film field
  (at most 15% added); the batch-independence test covers delivered observations.
- `P06` (§22, §23): estimator error dynamics Φ for the Gramian; estimator and controller histories inside checkpoints;
  out-of-sequence packets; the gym keeps the final observation at reset; observation age as an optional input;
  equivariant policies use 𝔾_obs; the actor schema is frozen and tested with privileged channels absent.
- `P07` (§25): mission queries consume alias witnesses, clearance cuts and retrofit kits as priced alternatives, and
  emit probe requests through `scene.probe-request`.
- `P08` (§24, §26): tasks declare goal symmetry, deadlines and which doors are open, closed or openable; sealed scene
  families for the qualification lane; Nemesis uses the hazard registries and the retrofit catalog as its scene-edit
  vocabulary, searches the intervals of `assumed` cards, and supplies minimal counterexample worlds to
  `scene.retrofit`.
- `P09` (§28, §29): registration of the observability and 𝔾_obs analyses beside A-03, A-07 and A-14; A-07 consumes
  `.fclear`; battery selection uses the hazard registries as failure-mode rows; checkpoint contents of §20.4.5;
  boundary work of prescribed movers on its own ledger line.
- `P10` (§30): spill-pad traction data for the film law; real sensor logs for `sense.calibrate`; measured sensor and
  optical cards.
- `P11` (§32, §37): viewer overlays of §20.10; scene catalogs, stills and acceptance reports as MCP resources; privacy
  inheritance for real places and their stills; long verbs (`scene.gen`, `scene.fit`, `scene.retrofit`, `sense.alias`)
  take the bounded synchronous wait.
- `P12` (§33, §34): airlock entries for `fs-rep-sdf`, `fs-rep-mesh`, `fs-rep-frep`, `fs-topo`, `fs-tribo`, `fs-lbm`;
  upstream asks for dynamic rods and for geometry publication by the reconstruction project; scope caps for
  `frob-scene` and `frob-sense` need raising for clearance, retrofit, pools, clocks and the analyses (suggested caps:
  34 and 20 thousand lines).
- `P13` (§38–§40): KA numbers for the tables of §19.18 and §20.20; the gates above in the register; the proxy-adequacy
  pilot uses an incumbent pinned by the rival audit.
- `P14` (§41–§44): work packages for clearance, retrofit, living worlds (three tiers), the world bank, the clock
  contract, optical failure modes and the sensing analyses; milestones as given in the gate tables.
- `P16` (appendices): the full hazard registries with provenance columns; source-register rows for the squeeze-film
  formula, the viscous-spreading exponents, linear sloshing frequencies, the right-angle corridor result, small-time
  local controllability of car-like bases, the glTF camera convention, the published interior generators, the
  incumbent deformable solvers and determinism note, the executed sill study and boundary-screen experiment, and the
  measured memory limits of the M5 Ultra.

**Decisions the charter did not settle, taken here**
- Slosh: the charter's summary lists slosh under Moonshot and the assignment lists payload slosh under Frontier. Here
  the equivalent mechanical model is [F] and resolved free-surface fluid is inside the [M] islands.
- "Rigid proxies" [S] are read as bodies with no internal joints (free slab, fixed ridge, free rod); chain cords and
  hinged-plate rugs are [F].
- The film law's thickness state is Habitat's and the cone row is Traction's.
- Sensorium refusal codes sit in a 44x block of the 4xx family.
- Verbs for sensing analyses use a new `sense` cluster; Atelier may alias them under `analyze.*`.
- Glass reports the stronger return deterministically (OPEN-SENS-5).
- On throughput tiers the default for intra-step clock events is `hold` (OPEN-SENS-1).


# ===== P05.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

Gates
- GATE-SPARK-07 hardware fingerprint (M0)
- GATE-SPARK-08 exact and guarded kernels (M0)
- GATE-SPARK-09 strict placement and receipts (M2)
- GATE-SPARK-10 variants, buckets, mixed batch, variant admission (M2)
- GATE-SPARK-11 arena allocation, kernel cache (M2)
- GATE-SPARK-12 launch graphs, pools, snapshot reads, commit fence (M2)
- GATE-SPARK-13 fleet campaign speedup and seeds-only transfer (M4, [F])
- GATE-SPARK-14 specialized kernels (M4, [F])
- GATE-SPARK-15 batch-composition, slot and capacity invariance (M2)
- GATE-SPARK-16 decision-level equivalence under D3 (M2)
- GATE-SPARK-17 class-licence refusals and D4 re-evaluation (M3)
- GATE-SPARK-18 scaled solve and conditioning certificate (M2)
- GATE-SPARK-19 unit ordering, paired-bound harness, disclosure refusal (M2)
- GATE-SPARK-20 weak and strong scaling; PPO normalization (M2)
- GATE-SPARK-21 incumbent baselines on our hardware (M1)
- GATE-SPARK-22 memory estimator and bytes per world-step (M2)
- Changed wording, same identifier: GATE-SPARK-02 (adds the decision-level test), GATE-SPARK-03 (points to
  §36.3 tolerances and the paired lower bound), GATE-SPARK-04 (adds the incumbent Metal measurement)

Kill gates
- KILL-BUCKET, KILL-SLOT, KILL-D1, KILL-FLEET, KILL-SPECIALIZE, KILL-MEGA (§21); KILL-SHARD, KILL-PARITY (§36)
- Carried: KILL-KIR, KILL-METAL

Decisions
- ADR-SPARK-02 to ADR-SPARK-10; ADR-NUM-01 to ADR-NUM-06; ADR-PERF-01 to ADR-PERF-05

Invariants and proof obligations
- INV-SPARK-01 to INV-SPARK-10; PO-SPARK-01 to PO-SPARK-04
- INV-NUM-01 to INV-NUM-06; PO-NUM-01 to PO-NUM-03
- INV-PERF-01 (unit ordering), INV-PERF-02 (timers cover completion)

Open questions (provisional names for P14)
- OPEN-SPARK-3 bucket set and capacities; OPEN-SPARK-4 `cudarc` bindings for graph capture and async pools;
  OPEN-SPARK-5 posture if the Metal gateway asks never land; OPEN-SPARK-6 peer access between the two RTX 4090
- OPEN-NUM-1 explicit fused multiply-add; OPEN-NUM-2 decision-equivalence thresholds; OPEN-NUM-3 cell side;
  OPEN-NUM-4 conditioning limit and refinement count
- OPEN-PERF-1 tolerance ratio; OPEN-PERF-2 comparator revisions; OPEN-PERF-3 energy source; OPEN-PERF-4
  restating rows if the default step moves
- Carried and extended: OPEN-SPARK-1, OPEN-SPARK-2 (now also the envelope classes E0 to E3 of §36.14)

Refusal codes (numbers provisional; the lead reconciles against other parts)
- `6xx`: FROB-REF-601 to FROB-REF-618 (§21.21)
- `2xx`: FROB-REF-247 to FROB-REF-252
- `5xx`: FROB-REF-561, FROB-REF-562
- `7xx`: FROB-REF-761
- `8xx`: FROB-REF-861 to FROB-REF-866

Formats and records
- FMT-KERNEL `frob.kernel/1` (kernel manifest with stage signature); FMT-KIR `frob.kir/1`;
  FMT-STEPPLAN `frob.stepplan/1`; FMT-FINGERPRINT `frob.fingerprint/1`; FMT-PLACEMENT `frob.placement/1`

Verbs
- New: `spark.fingerprint`, `spark.place`, `spark.cache`, `fleet.status`, `fleet.plan`
- Carried: `spark.devices`, `spark.probe`, `spark.check`, `spark.precompile`
- New flags on existing verbs: `sim.run --backend strict:<b> | prefer:<b> | mixed-allowed`,
  `--require-current-check`, `--profile reference-replay | device-replay | cross-backend | fast-training`,
  `--precision default | stiff`; `frobd --dispatch=per-job`

Kernel names added
- `K-act-bus`, `K-scale`, `K-refine`, `K-anchor`, `K-seg-reduce`, `K-variant-gather`, `K-commit`, `K-snapshot`,
  `K-x-pred`, `K-x-digest`, `K-x-hist`, `K-x-philox`, `K-g-pred`, `K-canary`

Named profiles, classes and labels
- Kernel classes `float`, `exact`, `guarded`, `stat`; math modes `Ieee754Safe`, `ExactInteger`, `FixedPoint`
  (reserved); determinism profiles `reference-replay`, `device-replay`, `cross-backend`, `fast-training`;
  precision profiles `default`, `stiff`; pairing labels `paired-exact`, `paired-by-address`; decision label
  `boundary-sensitive`; world label `Degraded: ill-conditioned`; envelope classes E0 to E3; reference workloads
  Q and H

Crates touched
- `frob-kir`, `frob-kernels`, `frob-core` (class type for INV-NUM-06), `frob-conformance`,
  `derive/frob-derive`, `spark/frob-spark-cuda`, `spark/frob-spark-metal`; the daemon in `frob-cli` for slot
  dispatch and the fleet scheduler; `frob-phylum` for variant tables

**Needs from other parts**

- P00 (§2, §6): the ten-fold arithmetic must use §36.4's limits; the replay goal must be worded per KILL-D1
  (bit-exact only where class D1 is attained; batch-composition invariance is the property claimed after
  GATE-SPARK-15). The 120 s cycle and its 3.0 to 3.5 ratio are quoted in §36.4 item 4 from the North-Star
  scenario.
- P01 (§11): the conventions row for anchored positions should say the offset is centred, in [−4, 4) m
  (ADR-NUM-06, a correction: the uncentred shift is inexact); the time row follows charter G8 (§35.7 cites §20
  for rational clocks). Constructions C-05 and C-14 and theorem T-09 are cited by number.
- P03 (§16, §17, §18): the ten-stage step, the lockstep iteration count as a shape-class constant, the commit
  fence of the strict profile, the referee sampler (row 17), the head-to-head mixed-batch experiment with the
  per-family-template control arm (KILL-BUCKET defers to it; reconcile with any kill gate §17 defines), the
  optional hard-contact law's R × R operator, the four-row contact cone and joint rows used for row capacities.
- P04 (§19, §20): shared brick pool with one dense directory per scene variant; 16-bit narrow-band fields with a
  per-brick scale; `SceneId` equality across platforms (GATE-HAB-02); rational clocks.
- P06 (§22, §23): GATE-CORTEX-05 restated against §36.3 row 4 (tolerance 1.5 times the incumbent reproduced on
  `trj`, planning value 615 s; ambition 410 s); normalizers frozen within a collection segment; the distributed
  PPO rule of §36.13; the sampling controller is never the evaluator of record.
- P08, P09 (§26, §27, §29): exact importance weights from addressed draws; forks by indexed copy; attribution
  requires class D2 (`FROB-REF-562`); `EpisodeId` as BLAKE3 over committed snapshots; first-divergence search
  over the digest chain; integer log-bin histograms for telemetry quantiles.
- P09 (§28): the rank-agreement measure cited in §35.4; memoized episodes reusable only under D2 on one
  fingerprint.
- P11 (§32): `frobd` owns slot dispatch policy and the fleet scheduler; idempotent job records; the
  `spark` cluster has seven verbs and `fleet` two.
- P12 (§33, §34): INT-CUDA-01 must confirm graph capture and asynchronous pool bindings in the pinned `cudarc`;
  INT-FT-01 must confirm the gateway facts of §21.9 at the pinned commit; INT-FNP-01 and INT-FT-01 carry the
  stride-convention admission test; Appendix F ask FD-FT-01 should add an interruptible wait, byte offsets,
  function constants and a binary archive; the format registry should list the five records above.
- P13 (§38, §39, §40): the incumbent harness as an out-of-process oracle; comparator freeze; rows 20 to 22;
  gates GATE-SPARK-07 to -22 and the eight new kill gates for the register; known answers of §21.23, §35.14
  and §36.16 for `KA-nn` numbering; counterexamples for the catalogue (2²⁴ + 1; −0.0 padding; batch normalizer;
  the [0, 8) cell shift; sequential draws; two-centroid sketch; mean of shard means; pruning times speedup;
  unscaled tolerance; stride copy).
- P14: OPEN-SPARK-2 sits on the critical path of the M1 and M2 performance gates; work packages for the
  fingerprint (M0), exact kernels (M0), incumbent baselines (M1) and the bucketed step plan (M2).
- P16 (Appendix H): sources to register, each to be re-read before a gate cites it: MuJoCo Playground training
  curves; mjlab nightly benchmark; MuJoCo Warp nightly benchmark and event traces; Isaac Lab benchmark table;
  Newton determinism guide and tests; the mixed-batch paper; CPU MuJoCo rates measured on this Mac; RTX 4090 and
  M5 Ultra bandwidth figures; Carson and Higham on three-precision refinement; Demmel, van der Sluis and Higham
  on scaling; Sterbenz's lemma; Apple's shared-storage documentation and the September 2026 statement on
  rounding control; the recorded pilots (boundary classifier on three GPUs; specialized-solver speed gate;
  half-precision condition numbers; humanoid conditioning over 201 poses).


# ===== P06.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

Gates: GATE-REFLEX-04, -05, -06, -07, -08, -09; GATE-GAIT-01, -02, -03; GATE-SHIELD-01, -02, -03, -04, -05;
GATE-CORTEX-06, -07, -08, -09, -10, -11, -12, -13, -14, -15, -16, -17. Carried with changes: GATE-REFLEX-02
(paired lower bound and potency control added), GATE-REFLEX-03 (gauge rule, null-space test and comparator
added), GATE-CORTEX-01 (export replay and non-zero head added), GATE-CORTEX-03 (three arms, ten paired seeds),
GATE-CORTEX-05 (re-based from 15 minutes to 1.25 times a re-measured incumbent; published context 410 s).
Carried unchanged: GATE-REFLEX-01, GATE-CORTEX-02, GATE-CORTEX-04.

Kill gates: KILL-RESIDUAL, KILL-MPC, KILL-GAIT, KILL-SKILL, KILL-ENERGY, KILL-SHIELD, KILL-TUBE, KILL-LEARNKERN,
KILL-FAMILY, KILL-INHERIT, KILL-DISTILL, KILL-ALTALGO, KILL-GATREE, KILL-GYMADAPT. Carried: KILL-BASELINE,
KILL-EQUIV.

Bets: none defined. Referenced: BET-06, BET-07.

Decisions: ADR-REFLEX-01 to -08; ADR-CORTEX-01 to -09.

Invariants: INV-REFLEX-01 to -04; INV-GAIT-01; INV-SHIELD-01, -02; INV-CORTEX-01 to -07.

Proof obligations: PO-REFLEX-01, -02, -03; PO-GAIT-01, -02; PO-SHIELD-01, -02, -03, -04; PO-CORTEX-01 to -06.
PO-SHIELD-04 (damping injection never increases stored energy in the port model) is offered to §12 as a theorem
card.

Open questions: OPEN-REFLEX-2, -3, -4 (OPEN-REFLEX-1 carried); OPEN-GAIT-1, -2; OPEN-SHIELD-1, -2;
OPEN-CORTEX-1 to -6. Referenced and unchanged: OPEN-SPARK-2.

Refusal codes: FROB-REF-421, -424, -431, -441, -444, -451, -454, -457, -461, -464 (Reflex, `4xx`);
FROB-REF-521, -522, -524, -527, -531, -534, -537, -544, -547, -554 (Cortex, `5xx`); FROB-REF-817, -864, -867
(evidence, `8xx`).

Formats: FMT-FSKILL (`.fskill`, `frob.skill/1`); FMT-FENERGY (`.fenergy`, `frob.energy/1`). Schema without a
file extension: `frob.policy/1` (policy manifest and mapping table, carried in safetensors metadata).

Verbs: `control.synth`, `control.show` (carried); `control.estimate`, `control.ladder`, `control.mpc`,
`control.symmetry`; `gait.atlas`, `gait.orbit`, `gait.continue`, `gait.gradient`; `skill.compile`,
`skill.check`, `skill.transport`, `skill.show`; `shield.budget`, `shield.fit`, `shield.check`, `shield.report`;
`train.start`, `train.status`, `train.stop`, `train.export` (carried); `train.distill`, `train.family`,
`train.inherit`; `policy.apply`; `eval.detect`, `eval.rankcheck`.

Kernels required of the kernel library: `K-fork`, `K-cost`, `K-weight` (sampling controller);
`K-energy-monitor` (exact-integer class); streaming past-time requirement kernels; on-device forward, backward
and Adam kernels.

Result flags and labels: `screening`, `screening: sampling-control`, `screening: gait-orbit`,
`baseline-only: unranked`, `model-privileged`, `actuator-infeasible`, `residual-saturated`, `shield-dominated`,
`overconfident`, `order-sensitive`, `set-valued`, `one-sided`, `surrogate`, `uncovered`, `beyond-card`,
`published-context`, `model-robust`, `model-conditional`, and the detector flags `dt-`, `tier-`, `parameter-`,
`law-`, `tolerance-`, `stiffness-`, `rate-`, `leak-sensitive`, `mirror-asymmetric`, `energy-created`.

Crates touched: `frob-control`, `frob-gait`, `frob-shield`, `frob-learn`, `bindings/frob-gymnasium`; requirements
placed on `frob-algebra` (cone solver end states and dual certificates, H/K enumeration, isotypic projectors),
`frob-dyn` (the `Controller`, `ActionFilter`, `StateSource` and `TaskProgram` declarations), `frob-kernels`,
`frob-spec`, `frob-chronicle` (ledger residual bound, layer and fallback events), `frob-lemma` and `frob-cert`
(certificate re-checks).

Hand-checkable counterexamples for Appendix N: scale-fitted estimator scoring; the friction certificate; the
two-link arm witness; restricted-space Floquet (ẋ = −x, ẏ = +y); the bar on two supports (β = 1/3 gives v/4); the
payload-only energy budget (50 kg of reflected mass); the unboxed shield projection; the self-labeling monitor;
two latches; {stable, fall} refinement; the single `done` flag; per-step rewards at two control rates; per-step
discount at two control rates; the two action laws; the reversed clip branch; mean of shard means; per-shard
variance; verbatim stride copy; the mixed-unit minimum; the box cone.

**Needs from other parts**

- §12 (Lemma): the names of the no-authorization flags; the claim-product type; the offline auditor's intake for
  dual certificates and for the `.fenergy` predicate; a theorem card for PO-SHIELD-04.
- §14 (Skeleton): the canonical form of a design term; typed edits that declare role correspondences (frames,
  actuator roles, sensors, contact roles, phase clocks); the design grammar used as a pretraining distribution;
  Γ_mech and Γ_obs stored per robot-plus-task pair.
- §15 (Sinew): reflected rotor inertia, duty ratings, brake holding torque and cutoff delay on actuator cards;
  absorber envelopes on interface cards with provenance.
- §16 (Kinesis): `Degraded` and `Faulted` semantics; the coupled impact solve and event location on the referee;
  the trust ladder's promotion rule; full-state fork including warm starts and queues.
- §17 (Phylum): masks for padded links and worlds; per-world model variants for family pretraining.
- §18 (Traction): the four-dimensional contact cone; the alternative and hard-contact laws used by detectors.
- §19 (Habitat): scene splits with building-level holdouts; the clearance analysis that supports "absorber on
  every forward path".
- §20 (Sensorium): the clock contract (seven instants, boundary order, sample age, packet queues in
  checkpoints); namespaces for truth, measurement, belief and policy observation.
- §21 and §36 (Spark, performance): exact-integer kernel class; the batch-independence test; the training
  wall-clock target consistent with GATE-CORTEX-05; throughput figures that name environment steps or
  world-steps.
- §24 (Arena): `.ftask` ownership, requirement semantics and the blinded qualification lane; the design corpus
  and the calibration sets for rank agreement.
- §28 (Atelier): the rank-agreement procedure and the ladder allocation rule that consume GATE-REFLEX-04,
  GATE-GAIT-03 and GATE-CORTEX-12; the audit lane; whether `eval.rankcheck` is defined here or there (one
  definition must be removed if both exist).
- §29 (Chronicle): the energy ledger's residual bound; layer, fallback and shield events in telemetry.
- §30 and §31 (Ground, Foundry): import of absorber and drive measurements; the spend block on
  `model-conditional` results.
- §33 and Appendix F: admission tests and upstream asks for the three PPO errors, the stride convention, and
  articulated-dynamics derivatives for the referee orbit route; FD-FT-02.
- §35: refusal-code registry (collisions in `4xx`, `5xx`, `8xx` to be resolved by the lead); the refusal of
  paired statistics under an uncontrolled determinism class.
- §39: the rival arm's use of the adapter; the phase ledger entries for training and learner share.
- Layering: `frob-control`, `frob-gait`, `frob-learn` and `frob-spec` are all in layer 4. This part avoids
  edges between them by placing shared trait declarations in layer 3. If the lead prefers a declared DAG inside
  layer 4, rule L-1 needs that amendment.


# ===== P07.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

- Decisions: ADR-AUGUR-01 to ADR-AUGUR-12.
- Invariants: INV-AUGUR-01 to INV-AUGUR-11.
- Proof obligations: PO-AUGUR-01 to PO-AUGUR-07.
- Gates: GATE-AUGUR-01 to GATE-AUGUR-24.
- Kill gates: KILL-AUGUR-TIE, KILL-AUGUR-FALSECERT, KILL-AUGUR-REGION, KILL-AUGUR-CUTS, KILL-AUGUR-REUSE,
  KILL-AUGUR-SEPARATE, KILL-AUGUR-ALIAS, KILL-AUGUR-PROBE, KILL-AUGUR-DOMINANCE, KILL-AUGUR-LABELS,
  KILL-AUGUR-TRANSPORT, KILL-AUGUR-FACILITY, KILL-AUGUR-PHASE, KILL-AUGUR-SYMEVENT, KILL-SYNTH-CODESIGN,
  KILL-SYNTH-TRAJOPT, KILL-SYNTH-LINKAGE, KILL-SYNTH-CONT.
- Open questions: OPEN-AUGUR-1 to OPEN-AUGUR-11.
- Refusal codes: FROB-REF-868 to FROB-REF-859 (a provisional block in the 8xx family; renumber if it collides).
- Formats: FMT-FMISSION (`.fmission`, schema `frob.mission/1`); the family payload of `.frobcert` and the closed
  proof-kind registry `frob.cert.kinds/1` (kinds: `lp-primal`, `lp-farkas`, `conic-farkas`, `interval-box`,
  `taylor-box`, `sos-gram`, `finite-game`, `alias-pair`, `monotone-transport`, `symmetry-transport`,
  `partition`, `composition`, `product-unreachable`, `trace-witness`, `sample-coverage`). The `.frobcert`
  container itself is assumed to be defined in §12.
- Verbs: `mission.synthesize`, `mission.refute`, `mission.distinguish`, `mission.separate`,
  `mission.transport`, `mission.revalidate`, `experiment.suggest`. Strategy flag values of
  `mission.synthesize`: `codesign`, `refine`, `linkage`, `continuation`.
- Identity and address prefixes used in examples: `MissionId` (`q:`), mission answer (`a:`), bundle (`b3:`).
- Evidence profiles: `exact`, `enclosed`, `empirical`. World kinds: `bounded-set`, `population`. Persistence
  values: `fixed-per-execution`, `resampled`. Authority values used in fixtures: `independent-normal-force`,
  `passive-equal-spring`.
- Crates touched: `frob-mission`, `frob-synth`, `frob-cert` (owned); `frob-algebra` (cone solver, posets and
  antichains, used); `frob-fsim` (airlock to `fs-ivl`, `fs-sos`, `fs-voi`, `fs-regime`, `fs-ascent`,
  `fs-adjoint`); `frob-verbs` (registry entries); the `frob-verify` binary (links the `check` module).
- Existing identifiers referenced with unchanged meaning: C-08, C-09, C-12, A-08, A-09, A-10, GB-03.

**Needs from other parts**

- P00 (§8): the claim product type, and the seven claims by the names used in the gate table (Proof, Known
  answer, Parity, Design comparison, Performance). GATE-AUGUR-07 is an agent-level gate filed under Performance;
  remap it if §8 keeps a separate name for performance of the method.
- P01 (§10, §12): the hybrid-automaton and saltation constructions and the symmetry construction, referred to
  here by concept; the `.frobcert` container, hashing and signing; the general contract of `frob-verify`
  (standard library only, protocol version, size limits); candidate theorem cards for the soundness of the four
  proof rules on finite and polyhedral fragments (PO-AUGUR-06); the proof classes `Proved`, `Certified`,
  `Tested`, `Estimated`.
- P02 (§14, §15): design families with typed holes, free and bound parameters with charts, tolerances, admitted
  isomorphisms; `.fmodule` port contracts (the linkage lane emits one); actuator cards with authority models.
- P03 (§16, §17, §18): the transactional referee step and its retry semantics (for the unknown rule); Phylum
  family batches, segmentation at ambiguous guards and the speedup bound; outer wrench cones and contact laws.
- P04 (§19, §20): clearance complexes, the priced retrofit catalog and rigid proxies; the seven instants and
  the boundary ordering rule (whether a packet arriving at the decision instant is usable).
- P05 (§21, §35, §36): the exact-integer kernel class and guarded single precision with exact fallback; derived
  kernels for design-parameter gradients; the place where the ceiling formula is stated for the program.
- P06 (§22, §23): `.fskill` task interfaces and whole-body inverse dynamics; family policies and inheritance as
  the comparison arm for transport.
- P08 (§24, §26, §27): the requirement language and monitors; the two study types and the blinded
  qualification lane; Nemesis search, minimization and `.fmcw`; the referee route used to evaluate candidates.
- P09 (§28, §29): the memo graph and dependency keys; paired comparison by world address (contrasted in
  §25.16); A-08, A-09, A-10; replay addresses and first-cause classification.
- P11 (§32): registry entries for the seven verbs, the `mission` MCP cluster (six tools) and the placement of
  `experiment.suggest`; the budget-stop exit code (OPEN-AUGUR-7); `checker_s` as a budget line.
- P12 (§33, §34): admission of `fs-ivl`, `fs-sos`, `fs-voi`, `fs-regime`, `fs-ascent`, `fs-adjoint` and of
  `frankenscipy/crates/fsci-opt/src/linprog.rs` as a second linear-program solver; the crate split of
  OPEN-AUGUR-6; scope caps for the three crates.
- P13 (§38, §39, §40): KA numbers for the rows of §25.26; the negative-control registry entries for §25.4.7;
  the strata of the partial ten-fold gate that use these fixtures; the rule that one false certificate
  disqualifies an arm.
- P14 (§42–§44): adoption of OPEN-AUGUR-1 to 11; work packages for the five slices of §25.23 and for each
  pilot listed "to run" in §25.27; M1 exit to include GATE-AUGUR-01 to 07.
- P16 (App. H, K, N): a source-register row for the planning reproduction bundle (its file name contains a
  word the style rules exclude from plan text, so the path is left to the source register) with the inner paths
  cited in §25.22; rows for Bretl and Lall, Orsolino et al., Hatton and Choset, Censi, Zardini et al., Dinev et
  al., Fadini et al., Flow*, Scenic, decision-region determination, ASID, saltation matrices, contact-size
  dependence in gel indentation, the interval-design literature (Merlet; Chablat, Wenger and Merlet; Neumaier),
  the nine-point, Stewart–Gough and 6R solution counts, Burmester theory, orbit-library solve times and transfer
  results; the counterexample catalogue entries for §25.4.7.


# ===== P08.md =====
<!-- LEDGER -->

**Crates touched.** `frob-spec`, `frob-arena`, `frob-nemesis`, `frob-autopsy`. Consumed through the airlock:
`fs-eproc` (betting e-process, Bernoulli mixture sequence), `fs-uq` (defensive importance sampling, proposal fit),
`fs-dfo`, `fs-archive`, `fs-voi`, `fs-race`.

**Formats.** FMT-FTASK (`.ftask`, schema `frob.task/1`); FMT-FMCW (`.fmcw`, schema `frob.mcw/1`). Schemas without a
file extension of their own: `frob.verdict/1` (inside `.frun`), `frob.legality/1`, `frob.hardset/1`,
`frob.qualify.request/1`, `frob.qualify.receipt/1` (both travel in `.frobcert`), `frob.autopsy.attribution/1`,
`frob.autopsy.transcript/1`.

**Verbs.** `task.new`, `task.show`, `task.list`, `task.check`, `task.verdict`, `task.settle`, `task.fixture`;
`eval.preanalyze`, `eval.qualify`, `eval.ledger` (added to the existing `eval.run`, `eval.scorecard`, `eval.compare`,
`eval.promote`); `nemesis.search`, `nemesis.oracle`, `nemesis.shrink`, `nemesis.bound`, `nemesis.audit`,
`nemesis.cover`, `nemesis.hardset`; `autopsy.fork`, `autopsy.intervene`, `autopsy.attribute`, `autopsy.repair`,
`autopsy.rerun`, `autopsy.diverge`, `autopsy.show`.

**Capability scopes.** `qual:admin` (open and close campaigns, read sealed data), `qual:run` (submit requests).

**Decisions.** ADR-ARENA-01 to -10; ADR-NEMESIS-01 to -07; ADR-AUTOPSY-01 to -07.

**Gates.** GATE-ARENA-01, -02 (carried); GATE-ARENA-03 to -09 (new); GATE-QUAL-01, -02, -03; GATE-NEMESIS-01 to -08;
GATE-AUTOPSY-01 to -08.

**Kill gates.** KILL-SPEC-REWARD; KILL-NEMESIS; KILL-RARE; KILL-MCW; KILL-AUTOPSY.

**Invariants.** INV-ARENA-01 to -05; INV-QUAL-01 to -04; INV-NEMESIS-01 to -06; INV-AUTOPSY-01 to -06.

**Proof obligations.** PO-ARENA-01 to -04; PO-NEMESIS-01 to -04; PO-AUTOPSY-01 to -03. PO-ARENA-02 and PO-NEMESIS-04
are offered to §12 as candidate theorem cards.

**Refusal codes.** FROB-REF-502 (carried); 520 to 533 (requirement language, fixtures, studies); 840 to 845
(qualification); 360 to 362 (legality); 560 to 567 (Nemesis protocol); 760 (budget); 580 to 584, 586, 587 (Autopsy);
880 (citing an attribution as established). The numbers were chosen in unused parts of the family ranges; the lead may
renumber. FROB-REF-533 duplicates the determinism-class refusal of §35 if that section assigns its own code.

**Open questions.** OPEN-ARENA-1, -2 (carried); OPEN-ARENA-3, -4, -5; OPEN-NEMESIS-1 to -5; OPEN-AUTOPSY-1 to -4.

**Fixtures.** `fixture:cart-sill-dock` (fixture 1), `fixture:quad-split-grip` (fixture 2).

**Known answers offered to §38 and Appendix M (unnumbered).** The tables of §24.16, §26.14 and §27.16.

**Counterexamples offered to Appendix N.** Unit-dependent binding atom without scales; joint stage rates (18 or 19 of
20); peeking at a fixed-n sign test (13/256 after 8 looks); the unreachable tie at n = 64; fourteen retries at 5%;
delta debugging order dependence ({c, d, e} against {a, b}); the energy-per-completion reversal; explicit
current-then-wheel integration at 0.5 ms (1.497 against 0.921); Shapley values that cannot tell joint from alternative
causes; a negative Shapley value with a single-factor repair; the gear-ratio fork against the from-start re-run;
midpoint search on a non-monotone predicate (ticks 1 and 5); wide and narrow soles reversing with the sign of the
offset.

**Needs from other parts.**
- §8, §12: the exact names of the no-authorization flags (three are shown as examples), the claim-product type, and
  the `.frobcert` bundle carrying qualification receipts for `frob-verify`; theorem cards for PO-ARENA-02 and
  PO-NEMESIS-04.
- §14: the reference quadruped template and its pinned `DesignId` for fixture 2; typed design edits with model
  differences, used as interventions.
- §15: the actuator stage integrates the winding and wheel subsystem so that the wheel-loop known answer (1.4820 and
  0.9207) holds; limit crossings located inside a step.
- §16: the promotion rule and its envelope; the lockstep rule that online monitors obey; the fidelity ledger accepting
  artifact candidates.
- §17: mixed model variants in one batch, for forks that carry design edits.
- §19: `SceneSplit` and sealed scene families; cited sources for legality bounds (OPEN-HAB-1); the clearance complex
  as a necessary condition for the oracle; the retrofit query that takes a `.fmcw`.
- §20: rational clocks with phase accumulators and the boundary ordering rule; delivered-packet signals with
  timestamps.
- §22: a sampling controller and re-synthesis of the baseline controller, as oracle witnesses and for re-runs;
  hard-limit atoms shared with shields.
- §23: reward-term semantics, early termination on the decided-early flag, the optional added reward term, the
  perturbation list for exploitation detection.
- §28: `eval.compare` and racing as study executors; the rank-agreement measurement that uses the audit lane;
  `design bisect` on a monotone predicate; the memo graph holding reset caches.
- §29: checkpoint contents (packet queues, contact memory, actuator thermal state), channel kinds including interval
  extrema, ring buffers, chained per-tick digests at checkpoints, the failure taxonomy, the energy-ledger residual.
- §32: `sync_wait_ms` on every long verb listed here; capability scopes `qual:admin` and `qual:run`; ghost overlay of
  a fork in the viewer.
- §35: the decision-level equivalence test; the exact-integer kernel class and guarded single precision; batch
  independence of a world's trajectory; the refusal of paired statistics under an uncontrolled class.
- §36: per-world state sizes and episode throughput, used here only as illustrations.
- §38: admission of a development oracle outside the product for the pilots; registration of every negative control
  listed in the known-answer tables.
- §39: the phase ledger that bounds the benefit of Nemesis and Autopsy; a composed-incumbent arm that may host the
  same methods.
- Appendix F: upstream asks for the 10⁶ sample caps in `fs-eproc` and `fs-uq` and for a splitting estimator.
- Appendix H: source register entries for the external facts cited: temporal-logic reinforcement learning (subgoal
  decomposition, robustness metrics, the finite-horizon learnability result, formula translation accuracy); adaptive
  splitting with temporal-logic scores; neural bridge sampling against plain splitting; the driving stress-testing
  study; the simulated-to-track transfer study; unbiasedness of adaptive multilevel splitting; chaos in learned
  continuous control; causal models of robot failures; the planning archive's sill pilot, boundary-classifier runs and
  sampled wheel analysis.


# ===== P09.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

Decisions: ADR-ATEL-01 to ADR-ATEL-11; ADR-CHRON-01 to ADR-CHRON-09.

Analyses: A-17 Observability and aliasing; A-18 Clearance and route feasibility; A-19 Rank robustness; A-20
Energy-budget feasibility; A-21 Deadline and latency closure; A-22 Sampled-loop stability; A-23 Shared-bus schedule;
A-24 Task-wrench attainability; A-25 Identifiability; A-26 Regime atlas. Rows A-03, A-04, A-07, A-08 and A-15 were
extended in place.

Decision cores: DC-01 to DC-11 (the eleven decision problems; renamed from a bare "D" prefix to avoid collision with
determinism classes D0 to D4 and doctrine rules).

Ladder rungs: R0 to R5. Ladder modes: `selective`, `discard-only` (the fixed table), `uninformative`. Reuse grades
1 to 4. Result types: `model-robust`, `model-conditional`, `budget-truncated`, `screen-unsound`, `a-change`,
`first-change`, `provisional: referee-unavailable`. Telemetry flags: `energy-created`, `contact-active`,
`cone-excess`. Outcome kinds: `physical`, `numerical`, `budget`, `protocol`. Added failure classes: `film-slip`,
`thermal-runaway`, `brownout`, `perception-dropout`, `localization-alias`, `pinch`, `snag`, `slosh-tip`,
`gait-instability`, `observation-alias`, `timing-race`, `model-drift`.

Gates: GATE-ATEL-05 to GATE-ATEL-16 (GATE-ATEL-01 and -03 amended: paired lower bound and potency controls; effect
size and benchmark reference). GATE-CHRON-04 to GATE-CHRON-13 (GATE-CHRON-01 amended: batch independence).

Kill gates: KILL-RANK, KILL-LADDER, KILL-VOI, KILL-REUSE, KILL-FDGRAD, KILL-ATLAS, KILL-DR. KILL-STATIC carried
forward.

Invariants: INV-ATEL-01 to INV-ATEL-12; INV-CHRON-01 to INV-CHRON-13.

Proof obligations: PO-ATEL-01 to PO-ATEL-10; PO-CHRON-01 to PO-CHRON-07.

Open questions: OPEN-ATEL-1 to OPEN-ATEL-6; OPEN-CHRON-2 to OPEN-CHRON-5 (OPEN-CHRON-1 carried forward).

Refusal codes: FROB-REF-553 to FROB-REF-551 (evaluation protocol); FROB-REF-870 to FROB-REF-872 (evidence protocol,
Atelier); FROB-REF-869 to FROB-REF-815 (evidence protocol, Chronicle). Sub-ranges were chosen to stay clear of other
parts; the lead may renumber.

Formats and schemas: FMT-FRUN (`.frun`, `frob.run/1`); `frob.verdict/1`; `frob.core-table/1`; `frob.beadlog/1`.
Registry directories: `registries/cores/`. Log path: `docs/delivery/logs/<bead>.ndjson`.

Verbs: `result.revalidate`; `experiment.suggest` (mechanism here, also listed by §25); `opt.cascade`; `opt.start
--family`; `eval.compare --plan` (reachability pre-analysis); `eval.rank-agreement`; `eval.audit`; `explain.compare`;
`report.campaign`; `cores.show`; `run.fork`; `run.export`; `telemetry.divergence`; `telemetry.histogram`;
`evidence.trace`; `store.status`; `store.collect`. Existing verbs kept: `design.check`, `design.diff`, `design.blame`,
`design.bisect`, `design.edit`, `eval.compare`, `eval.promote`, `explain.run`, `opt.start`, `opt.status`,
`opt.archive`, `predict.register`, `predict.report`, `report.design`, `sim.replay`, `telemetry.summary`.

Identifier types: `ClaimId`, `EvidenceId`, `DecisionPolicyId`, `TraceId`, `ChunkKey`, `RunId`, `StreamId`.

Crates touched: `frob-analyze`, `frob-opt`, `frob-chronicle`, `frob-core` (the `Memo` trait, identifier types),
`frob-verbs` (memo wrapper around executors), `frob-cert` (licence and mask records read by `frob-verify`). Decision
rules for mechanisms in `frob-nemesis`, `frob-autopsy`, `frob-ground`, `frob-dyn`, `frob-arena`, `frob-kir`,
`derive/frob-derive` and `frobd` are specified here and owned there.

**Decisions the charter did not settle, as resolved here (the lead may overrule)**

- The eleven decision problems are identified as DC-01 to DC-11, not D1 to D11, because D0 to D4 are determinism
  classes and both appear in the same sentences (ADR-ATEL-11).
- Two revalidation verbs exist: `result.revalidate` here for any stored verdict, and the mission cluster's
  revalidation verb in §25. This part treats them as one engine behind two verbs (OPEN-ATEL-4).
- The artifact-graph "policy identifier" is named `DecisionPolicyId`, because `PolicyId` already names a control
  policy by content hash and the sibling type of the same name means a versioned decision rule.
- `.frun` has the schema name `frob.run/1` and the format identifier `FMT-FRUN`.
- Refusal sub-ranges 540 to 551, 840 to 851 and 870 to 872 were self-assigned.
- A-19 and A-26 read runs, so the heading "static analyses" became "analyses"; the two are marked as needing runs and
  never join `check`.
- New gates were given milestones in a line under each gate table, for §40 and §43 to adopt or change.
- The fixed table uses η = 3; the low mode threshold 0.2, the audit share of one tenth and the memo shadow rate of 1
  in 64 are DESIGN defaults with no evidence behind the exact values.

**Needs from other parts**

- P00 (§8): the claim-product type; a home for the result types `model-robust` and `model-conditional` inside the
  Design comparison claim. P00 (§4): BET-05 and BET-10 wording consistent with the amended GATE-ATEL-01 and -03.
- P01 (§10, §12): construction references C-01, C-03, C-06 to C-12, C-14, C-15 unchanged; the no-authorization flags;
  theorem cards, if wanted, for PO-ATEL-01 and PO-CHRON-02.
- P02 (§14, §15): `design.edit` with expected parent, idempotency key and a returned invalidation set; the
  stale-parent refusal code; the incremental-compilation invariant; card error-dependence fields read by A-17.
- P03 (§16, §17, §18): the trust ladder, the fidelity ledger and its frozen envelopes, with the promotion sample size
  set by the DC-01 table and no longer "chosen by the e-process"; effort profiles as model identities and the default
  profile (DC-08); the background referee sampler; the transactional referee and its contact capacity; mixed batches,
  the observation-preserving canonical representative and the mission-observable digest (§28.15); the hard-contact
  batch law and cone variants used by A-19.
- P04 (§19, §20): `.fclear` and its known answers (A-18); the clock contract with seven instants and the boundary
  ordering rule (A-21); observability and alias witnesses (A-17); privacy class of scenes.
- P05 (§21, §35, §36): rewrite classes E and R and the kernel binary cache (DC-11); determinism classes and the
  decision-level equivalence test; the batch-independence test; the disclosure block; the profile condition for class
  R.
- P06 (§22, §23): gait orbits with continuation (A-04); the cone program and its separating witness (A-24); family
  policies, fine-tuning fractions and inheritance (R1 to R3); the family-policy experiment whose specialists the
  rank-agreement experiment reuses; exploitation detectors reading the flags of §29.9.
- P07 (§25): the docking fixture and the finite-game fragment used by PO-ATEL-06; `mission.revalidate` as the same
  engine as `result.revalidate` (OPEN-ATEL-4); product-system dominance.
- P08 (§24, §26, §27): task-declared metric spans; the design benchmark for GATE-ATEL-03; the blinded qualification
  lane and `alpha_spend`; batteries and DC-10; rare-failure machinery and DC-04; fork-based attribution and DC-06.
- P10 (§30, §31): reality bands and DC-07; identifiability procedure (A-25); build packets refusing
  `model-conditional` inputs.
- P11 (§32, §37): the job state machine, idempotency binding, `sync_wait_ms`, the daemon's dispatcher and DC-09; the
  verb registry entries for the verbs above; privacy inheritance.
- P12 (§33, §34): admission of `fs-recompute`, `fs-voi`, `fs-race`, `fs-eproc`, `fs-archive`, `fs-bo`, `fs-dfo`,
  `fs-ascent`, `fs-package`, `fs-checker`, asupersync's lab runtime and `franken_kernel` identifier types; the
  generated-artifact and contract checks that validate `registries/cores/` and bead logs.
- P13 (§38, §39, §40): KA numbers for the tables of §28.22 and §29.19; the standard battery; the phase ledger;
  registration of the new gates and kill gates.
- P14 (§41 to §44): adoption of OPEN-ATEL-1 to -6 and OPEN-CHRON-2 to -5; risks "a screen discards the winner",
  "retention deletes the interval that explains a decision", "an incomplete memo key serves a stale verdict"; work
  packages for the ladder, the cascade, revalidation, the decision-core tables, the chunk protocol, the artifact graph
  and simulation testing.
- P16 (appendices): worked cards for DC-01 to DC-11 in Appendix E (the illustrative examples of §28.19.2 are the
  content); verb registry rows; type sketches for `ChunkRecord` and the verdict record; counterexample catalogue
  entries from §28.4, §28.6, §28.7, §28.13, §29.7 and §29.8.

**Source-register entries this part relies on** (each to be re-read from the primary source before a gate cites it)

- Sibling code: `frankensim/crates/fs-race` (`CONTRACT.md`, `tests/battery.rs`); `fs-eproc` (`CONTRACT.md`,
  `src/bernoulli.rs`, `src/hardening.rs`); `fs-voi` (`CONTRACT.md`); `fs-archive`; `fs-recompute` (`CONTRACT.md`);
  `fs-bo`, `fs-dfo`, `fs-ascent`; `fs-uq`; `fs-package`, `fs-checker`; `fs-contact` (the 21-contact limit);
  `frankensqlite/crates/fsqlite-pager/src/s3_fifo.rs`; `asupersync/src/lab/`; `asupersync/franken_kernel/src/lib.rs`.
- Published figures: incumbent training times (about 410 s quadruped on one RTX 4090; under 30 minutes humanoid);
  family-policy and inheritance figures (70 to 85% zero-shot within a class; 1 to 20% fine-tuning steps; 33 to 100%
  after a single-limb edit); rank-correlation neighbours (0.40 overall; 0.50, 0.60, 0.83; sparse Kendall τ 0.236,
  0.740, 0.159; up to 80% of specialist performance); biased early evaluation and shared-controller drift;
  cross-fidelity correlation collapse and the 5 × 10⁻⁴ degree divergence after 50 steps; population co-design compute
  gains of three to four times; sampling-control episode costs; adaptive stress testing savings of 10 to 100 times;
  conformal coverage spread at n = 20; the recursion share of 16 to 20% of an incumbent step; language-model tuner
  against Bayesian optimization; the co-design benchmark where model-based search lost.
- Executed pilots: the stance enumeration (11,520 cases; 1,596 false accepts; the equal-spring forces); the sampled
  wheel loop (1.4820, 0.9207, 2.7075); the promotion-rule enumeration (2,145; 521; 521; 1,103; first tie at n = 86);
  the docking gain 33/32 follow-up (34,590 decisions per grid); the camera-delivery reuse fixture; the gripper probe
  (675 of 675, 0 of 225, 900 of 900); the two-finger bus; the passive sill work cut; the sensor alias counts
  (3807/4096).


# ===== P10.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

- Decisions: ADR-GROUND-01 to -10; ADR-FOUNDRY-01 to -12.
- Gates: GATE-GROUND-01 to -10; GATE-FOUNDRY-01 to -12.
- Kill gates: KILL-GROUND, KILL-BAND, KILL-CLOCK, KILL-ASBUILT, KILL-KIT; KILL-TOPOPT, KILL-LOADS, KILL-FAB,
  KILL-TOLERANCE, KILL-PACKET, KILL-FIRMWARE, KILL-LIFECYCLE.
- Invariants: INV-GROUND-01 to -07; INV-FOUNDRY-01 to -07.
- Proof obligations: PO-GROUND-01 to -03; PO-FOUNDRY-01 to -04.
- Open questions (provisional, for §42): OPEN-GROUND-1 to -5; OPEN-FOUNDRY-1 to -6.
- Refusal codes (provisional numbers; the lead reconciles them with other parts): FROB-REF-873 to -850 (Ground,
  evidence and protocol family); FROB-REF-166 to -165 (Foundry, design and model family); FROB-REF-260 to -262
  (Foundry, numerics family); FROB-REF-860 to -864 (Foundry, evidence and protocol family).
- Formats: FMT-FBUILD (`.fbuild`, schema `frob.fbuild/1`). Schemas with no new extension:
  `frob.ground.dataset/1` (manifest of a `.frun` bundle of origin `real` or `synthetic`),
  `frob.ground.prediction/1` (an extension of the registered-prediction record).
- Verbs: `ground.ingest`, `ground.excite`, `ground.identify`, `ground.predict`, `ground.score`, `ground.band`,
  `ground.kit`; `foundry.loads`, `foundry.size`, `foundry.optimize`, `foundry.check`, `foundry.tolerance`,
  `foundry.export`, `foundry.lifecycle`.
- Reality statuses: `no-claim`, `synthetic`, `published-anchor`, `candidate`, `calibrated`, `retired`. Result
  labels: `ranking-falsified`, `wrong-reason`, `model-conditional`, `singular: fillet required`,
  `fatigue: no-claim`, `review: no-claim`, `timing: no-claim`, `check: same-family`.
- Feature flags: `firmware-export`, `lifecycle`.
- Packet flag added by this part: `fabrication_released` (beside the three artifact flags of §12).
- Crates touched: `frob-ground`, `frob-foundry`, `frob-lifecycle`; consumers of `frob-fsim`, `frob-chronicle`,
  `frob-actuate`, `frob-model`, `frob-kir`, `frob-cert`.
- Known answers listed without numbers in §30.21 (14 rows) and §31.18 (17 rows), for numbering in §38.
- Counterexamples for the catalogue: clock fitted to the scored event; delay against clock offset; the eight
  structural aliases; pooled conformal calibration (20 of 20 and 16 of 20); assembled peaks, including the
  unsafe opposite-sign case; equal loading of symmetric limbs; window-split rainflow; zero first-order
  sensitivity; Q15 (−1)·(−1); unclamped integrator; law of the average against average of the law; the
  bus-schedule prune.

**Needs from other parts**

- §8 and §12 (P00, P01): the canonical names of the no-authorization flags and the claim-product type; whether
  `frob-verify` checks `.fbuild` flags; a home for theorem card T-14 and for PO-GROUND-01 as a theorem candidate.
- §15 (P02): card schema support for a joint covariance block, per-field provenance values `measured` and
  `as-built`, validity domains for wear and ageing laws, and material cards with process-specific stress-life
  data; OPEN-SINEW-1 retained.
- §14 (P02): a typed edit that replaces a link's primitive Region by a geometry reference with mass properties.
- §16 to §18 (P03): joint-level stiction and backlash rows; the alternative contact law and stiffness variants
  used for the model-form label; the divergence ledger; per-world model variants for aged and as-built robots.
- §20 (P04): the seven instants and rational clock periods; sensor models used by the synthetic rehearsal.
- §22 (P06): the gait-orbit analysis with Floquet multipliers; the energy-budget monitor; baseline controller
  re-synthesis after an inertia edit.
- §24 (P08): the cart-and-sill fixture; qualification verdict identifiers; the α_i = α/[i·(i+1)] spending rule;
  declared study types.
- §26 (P08): rare-overload rates for lifecycle runs.
- §28 (P09): analyses A-12, A-13, A-15, A-16; registered predictions; dependency-directed revalidation; the
  reality-calibration decision core and the experiment table.
- §29 (P09): T2 traces of joint constraint wrenches and body twists on every world of a load campaign; the
  energy ledger evaluated on a real log.
- §32 (P11): the seven-verb cluster rule, capability tokens for `ground.band --retire`, job handles.
- §33 and Appendix F (P12, P16): airlock entries for `fs-assimilate`, `fs-asbuilt`, `fs-uq`, `fs-eproc`
  (feature `conformal-hardening`), `fs-voi`, `fs-topopt` (feature `cutfem-marquee`), `fs-topols`, `fs-solid`,
  `fs-material`, `fs-mesh`, `fs-mbd-material`, `fs-geocon`, `fs-fab`, `fs-toleralloc`, `fs-io`, `fs-orbit`, and
  `fln-verdict` in franken_lean; numbered upstream asks for the eight requests in §30.4 and §31.4.
- §37 (P11): the private mark and its inheritance, as used by the kit builder.
- §38 and Appendix M (P13, P16): KA numbers; the published-anchor comparison at M3 under the Parity label; the
  report line giving the share of `generic` and `assumed` card fields.
- §43 and §44 (P14): work-package identifiers for Ground, for the optional bench packages A to F, and for
  Foundry's structure loop, packet, firmware and lifecycle; milestone placement (Ground and Foundry at M5,
  Moonshots at M8).
- Appendix H (P16): source-register entries for arXiv 2604.10351, 2107.07511, 2202.13415, 2510.04354,
  2606.26643, the quadruped mass-share paper, the executed pilots cited in §30.8 and §30.9, and the sibling
  files read for §30.4 and §31.4 at FrankenSim commit `483d09c682` and franken_lean commit `4426f8ea7`.
- Appendix E (P16): worked cards for the reality-calibration decision (§30.15) and the loads–mass loop (§31.9).


# ===== P11.md =====
<!-- LEDGER -->

**New identifiers defined in this part**

- Gates: GATE-TETHER-04 to GATE-TETHER-14 (GATE-TETHER-01 to -03 carried; -02 extended with the served
  `tools/list` check). GATE-SAFE-01 to GATE-SAFE-10.
- Kill gates: KILL-MCP (carried), KILL-SLOT, KILL-PREEMPT, KILL-MCPTASKS, KILL-SOLVE, KILL-TUI.
- Decisions: ADR-TETHER-01 to -11; ADR-SAFE-01 to -05.
- Invariants: INV-TETHER-01 to -13; INV-SAFE-01 to -08. Proof obligations: PO-TETHER-01 to -05; PO-SAFE-01 to -03.
- Open questions (provisional): OPEN-TETHER-1 to -5; OPEN-SAFE-1 to -4.
- Refusal codes: FROB-REF-701 to -704, -720 to -723, -730, -731, -733, -734, -801 to -810, -860 to -864. The
  ranges 700–739, 801–810 and 860–864 are claimed provisionally; reconcile with 7xx and 8xx codes from other parts.
- Exit codes: 8 added (accepted and still running); exit 3 extended to families 4xx, 5xx and 8xx, which the
  carried table left unmapped; exit 9 also covers integrity faults.
- Formats and schema names: FMT-ENVELOPE (`frob.envelope/1`, extended with `claims`, `reality`, `bound`, `budget`,
  `refusal`, `job`, `cursor`, `frontier`, `idempotent_replay`, `authorization`), FMT-RPC (`frob.rpc/1`), FMT-CURSOR
  (`frob.cursor/1`), FMT-PLAN (`frob.plan/1`), FMT-JOB (`frob.job/1`). Schema types `int53`, `u64str`, `i64str`;
  schema annotation `x-frob-untrusted`; resource scheme `frob://`.
- Verbs (new; all earlier verbs carried): `session.open`, `.show`, `.focus`, `.close`; `job.find`; `artifact.read`,
  `.export`, `.list`; `run.show`, `.list`, `.fork`; `eval.qualify`; `study.plan`, `.submit`, `.status`, `.cancel`,
  `.compare`; `experiment.suggest`; `autopsy.interventions`, `.run`, `.show`, `.diff`; `nemesis.search`, `.shrink`,
  `.separate`, `.rare`, `.show`, `.adopt`; `mission.new`, `.check`, `.synthesize`, `.refute`, `.distinguish`,
  `.show`; `result.show`, `.revalidate`; `cert.bundle`, `.check`, `.show`, `.list`; `ground.import`, `.calibrate`,
  `.band`, `.compare`, `.show`; `foundry.structure`, `.fatigue`, `.tolerance`, `.packet`, `.show`, `.firmware`;
  `report.mission`; `scene.purge`; `solve.run`. MCP-only compatibility tool `verb_invoke`.
- Reserved cluster names, verbs left to their chapters: `family`, `gait`, `shield`, `skill`, `synth`, `lifecycle`.
- MCP task clusters (seventeen with `operate`) and three base tools, §32.12.4; capability scopes, §32.13.
- Crates touched: `frob-verbs` (new `registry-only` feature), `frob-cli` (`frob`, `frobd`), `surface/frob-mcp`,
  `frob-tui`, `viewer/`, `frob-io`, `frob-chronicle`, `frob-cert`, `xtask`.

**Needs from other parts**

- §33, §34 (P12): keep INT-FMCP-01 as worded; the `fastmcp_rust` row should record a commit pin at or after
  `117388bc` (not registry 0.13.0), `nightly-2026-08-25`, `rust-version` 1.100 against FrankenSim's rustc 1.99
  nightly, and the MIT-with-rider licence. Add admission rows for FrankenTUI (0.10.0, `nightly-2026-08-31`,
  `asupersync-executor` off), FrankenSQLite as the job ledger (single writer) and the asupersync lab runtime for
  fault tests. `frob-verbs` `registry-only` must have no path to `frob-fsim`; the annex has its own toolchain file.
- §35, §36, §21 (P05): the batch-independence test GATE-TETHER-09 relies on; the definition of the
  nondeterministic training profile labeled in §37.11; the refusal-family table as extended here; the cost model
  behind `sim.estimate`; the reference batch (8,192 worlds) and throughput target (R = 2 × 10⁶ world-steps/s is a
  placeholder, not a target defined here); snapshot boundaries cheap enough for a 5 s quantum; per-world buffers
  for model variant, scene, task and job; placement receipts for the Devices pane. §17 (P03): shape classes.
- §24 (P08): the blinded lane's request commitment, sealed clusters and adaptive-retry gate (GATE-SAFE-03's
  negative control); the fixed-n reachability pre-analysis returned by `study.plan`; the requirement language.
- §25 (P07), §26, §27 (P08), §30, §31 (P10), §12 (P01): the semantics behind the `mission`, `nemesis`, `autopsy`,
  `ground`, `foundry` and `cert` verbs named here; those chapters may add verbs to these clusters.
- §29 (P09): chunk format and store, committed frontiers, fork classification, `artifact.read` by byte range,
  token-budgeted summaries.
- §8 (P00), §12 (P01): the claim-product type; `frob-verify` checking the flags and recomputing the product.
- §20 (P04): the viewer text is carried here from the earlier Sensorium chapter; GATE-SENS-03 stays there.
- Appendices H and J (P16): source rows for the incumbent MCP servers and the sibling facts cited by path;
  the registry compiled from §32.6.
- §42, §43 (P14): adopt the OPEN items above. The tag SAFE (gates, invariants, open questions) is new and is not
  in the charter's list of subsystem tags. Kill-gate rows here omit the cross marker used in the earlier tables.


# ===== P12.md =====
<!-- LEDGER -->

New identifiers defined in this part

Architecture decision records
- ADR-SUITE-01 (carried; text extended)
- ADR-SUITE-02 worker-process fallback before any lift
- ADR-WS-01 rule for creating a new crate
- ADR-WS-02 independent auditor
- ADR-WS-03 format registry

Admission gates
- INT-TOOLCHAIN-01 (carried; now a matrix over four nightlies)
- INT-FSIM-01 (carried; extended with root settings, build times, drift classes)
- INT-FSIM-02 to INT-FSIM-11 first admission tests, file by file (INT-FSIM-12 reserved, unused)
- INT-FSIM-13 spectral group; INT-FSIM-14 statistics group; INT-FSIM-15 structure group; INT-FSIM-16 recompute
  store; INT-FSIM-17 multiphysics bridge; INT-FSIM-18 worker round trip
- INT-ASUP-01 (carried); INT-ASUP-02 lab runtime
- INT-FSQL-01 (carried); INT-FSQL-02 eviction policy differential
- INT-FT-01 (carried; patch repeated); INT-FT-02 resident buffers and completion receipt
- INT-CUDA-01 (carried)
- INT-FSYM-01 (carried); INT-FSYM-02 two-link Jacobian
- INT-FLN-01 (carried); INT-FLN-02 admit and reject
- INT-FMCP-01 (carried; restated for the annex toolchain and process boundary)
- INT-FNP-01 (carried); INT-FNP-02 stride conversion
- INT-FTUI-01; INT-F3D-01; INT-FDGR-01; INT-FJAX-01; INT-FTX-01

Convergence gates
- GATE-WS-01 six checks catch seven plants; GATE-WS-02 format round trips; GATE-WS-03 auditor independence;
  GATE-WS-04 checks finish in 60 s

Kill gates
- KILL-FSIM (carried; now two ordered fallbacks: (a) worker process, (b) lift)
- References only, defined elsewhere: KILL-KIR, KILL-LEMMA, KILL-MCP

Foundation rows
- Carried: FD-FSIM, FD-ASUP, FD-FSQL, FD-FT, FD-CUDA, FD-FNP, FD-FSCI, FD-FSYM, FD-FLN, FD-FMCP, FD-FTUI, FD-F3D,
  FD-FJAX, FD-FSS
- New: FD-FDGR (drone reconstruction), FD-FKER (franken-kernel, franken-evidence), FD-FTX (franken_textiles),
  FD-FFW (franken_firmware)

Upstream asks (for Appendix F)
- FD-FSIM-10 PPO corrections; FD-FSIM-11 planar_drive branch; FD-FSIM-12 witness migration; FD-FSIM-13 contact
  capacity above 21; FD-FSIM-14 build identity for out-of-tree consumers; FD-FSIM-15 stable fallback in fs-qty;
  FD-FSIM-16 refresh constellation.lock
- FD-FNP-01 stride conversion (franken_numpy and frankentorch); FD-FSQL-01 generic key in S3-FIFO; FD-FMCP-01
  common compiler for fastmcp_rust and FrankenSim; FD-FTUI-01 asupersync pin on the optional executor;
  FD-FKER-01 claim and evidence identifiers

Invariants
- INV-SUITE-01 to INV-SUITE-05; INV-WS-01 to INV-WS-05

Layer rules
- L-1 (clarified: layer and rank); L-2 (qty through frob_core::qty); L-5 (frob-spec added to boundary crates;
  Python only in bindings); L-6 (frob_core::qty); L-9 auditor edges; L-10 inversions; L-11 two planes; L-12
  tier features off by default

Refusal codes (numbers provisional; the lead reconciles ranges across parts)
- FROB-REF-790 foundation-group-absent; FROB-REF-791 referee-worker-mismatch; FROB-REF-890 stack-mismatch;
  FROB-REF-891 format-version-unsupported; FROB-REF-892 view-is-not-identity; FROB-REF-893 format-owner-bypass

Formats
- FMT-FBOT, FMT-FMODULE, FMT-FCARD, FMT-FSCENE, FMT-FCLEAR, FMT-FTASK, FMT-FMISSION, FMT-FENERGY, FMT-FSKILL,
  FMT-FMCW, FMT-FRUN, FMT-FBUILD, FMT-FROBCERT
- Schema names: frob.design/1 (carried), frob.module/1, frob.card/1, frob.scene/1, frob.clearance/1, frob.task/1,
  frob.mission/1, frob.energy/1, frob.skill/1, frob.mcw/1, frob.run/1, frob.build/1, frob.cert/1; build record
  frob.stack-receipt/1 (not an artifact format)

Open questions
- OPEN-SUITE-1 (carried); OPEN-SUITE-2 to OPEN-SUITE-6; OPEN-WS-1 to OPEN-WS-5

Verbs
- None defined here.

Crates touched
- All crates of charter §6 (crate map). Conditional: frob-units. Binary targets, not crates: frob-fsim-worker
  (of frob-fsim), frob-verify (of frob-cert). Directories added to the layout: worker/, schemas/, demos/,
  receipts/; registries added: formats.toml, lifts.toml, capsules.toml (asks.toml carried).

Sibling facts found that other parts should know
- fastmcp_rust declares rust-version 1.100; FrankenSim's pin is rustc 1.99.0-nightly. frob-mcp cannot share the
  core toolchain today; this part makes the process boundary to frobd the default wiring (§33.13). P11 (§32)
  should not assume frob-mcp links frob-verbs.
- frankensim/crates/fs-la/build.rs fails closed outside FrankenSim's tree unless a build-identity variable is
  set; the core sets its own in .cargo/config.toml (first session, §46, should include this step).
- The closure of fs-mbd is 42 crates and about 1.57 million lines of library source; the M1 set is 65 crates and
  about 2.06 million. P14 should carry a risk row for build time and for the size of the lift fallback.
- Four nightlies are in play (2026-07-06, 08-20, 08-25, 08-31), plus stable 1.99.0 in franken_firmware.
- frankentui (optional executor), frankenjax and franken_threed pin asupersync 0.5.0; none may enter a graph.
- franken-kernel has TraceId, DecisionId, PolicyId, SchemaVersion, Budget, Cx; no claim or evidence identifier.
- franken_threed has four crates and no single-file packaging; the viewer's HTML export is ours (P11).
- franken_drone_geometry_reconstruction does not yet publish geometry; schemas exist (P04, P10).
- fs-conform is a converter-conformance kit, not conformal prediction (P10 should not cite it for bands).
- fs-g1-train consumes no fs-* crate; it is not a precedent for path consumption, and its PPO has the defects of
  §33.11.
- FrankenSim's lockfile has 262 registry packages of 479 (the earlier figure of 244 is stale).
- FrankenSim's agent rules cap any unapproved run at 16 GiB, 8 cores and one hour; §34.7 adopts this as the
  default envelope (it is already the default of OPEN-SPARK-2).

Needs from other parts
- P00: BET-04's wording unchanged (the 20% breakage threshold is quoted in KILL-FSIM).
- P01 (§12): the certificate kinds `frob-cert` must check, and the behavioural gates of the auditor; the bundle
  must embed the canonical bytes of the statements it certifies (§34.9 rule 1); names of the no-authorization
  flags.
- P02 (§14): confirm `frob_core::qty` as the only import path for quantities; confirm the rank order
  actuate → design → model → io → scene; `frob-model` keeps compile memoization only.
- P09 (§28, §29): the memo graph for results is placed in `frob-verbs` (policy) and `frob-chronicle` (bytes) so
  that rule L-11 holds; confirm or move by ADR.
- P03 (§16, §18): witness migration and the over-capacity refusal referenced by §33.11 row 4; the film kill gate
  referenced by the multiphysics bridge.
- P05 (§21, §35, §36): refusal family ranges; the 0.1% shadowing floor cited in §33.14; the dialect line cap.
- P06 (§23): the Gymnasium adapter's binding mode (OPEN-WS-5); PPO's first three tests are the probes of §33.11.
- P07 (§25): owner of `.fmission` (OPEN-WS-4); whether `frob-synth` reaches FrankenSymPy only as a subprocess.
- P10 (§30, §31): which airlock traits Ground and Foundry call (`Assimilation`, `AsBuilt`, `StructuralOpt`,
  `ToleranceAllocation`, `FabricationRules`, `MeshExchange`).
- P11 (§32, §37): the local-socket protocol between `frob-mcp` and `frobd`; KILL-MCP's text; the runtime
  estimate's interval; privacy inheritance for imported scans.
- P13 (§38, §40): adopt GATE-WS-01 to -04 and the INT register; assign KA numbers to the known answers listed in
  §33.9 to §33.12 and §34.9 to §34.11.
- P14 (§41 to §46): risk rows for the `fs-qty` coupling, build time of the FrankenSim closure, the fastmcp
  compiler floor, and dirty sibling trees; adopt OPEN-SUITE-2 to -6 and OPEN-WS-1 to -5; the first session sets
  the build-identity variable and writes the first receipt.
- P16 (Appendix F): add the twelve asks above with their fixtures.
