---
key: "EPIC-M3"
title: "Milestone M3, places, tasks, adversaries, qualification: comparable results and the partial ten-fold gate"
type: "epic"
priority: 2
milestone: "M3"
labels: ["m3", "delivery", "solid", "gate", "tenfold"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["BD-319", "N40-14", "N40-15", "N40-19", "N40-20", "N40-22", "N40-25", "N40-26", "N40-04", "N40-05", "EPIC-M2", "EPIC-M4"]
gates: ["GATE-TENFOLD-03", "GATE-TENFOLD-04", "GATE-TENFOLD-05", "GATE-TENFOLD-06", "GATE-TENFOLD-10", "GATE-TENFOLD-11", "GATE-TENFOLD-13", "GATE-QUAL-01", "GATE-QUAL-02", "GATE-NEMESIS-02", "GATE-AUTOPSY-03", "GATE-ATEL-01", "GATE-REFLEX-02", "GATE-FID-02", "GATE-TETHER-01", "GATE-GAUNT-01", "KILL-STATIC", "KILL-BASELINE", "KILL-NEMESIS", "KILL-AUTOPSY", "KILL-AUGUR-TIE", "KILL-FAMILYCERT", "KILL-TENFOLD-SPEED", "KILL-TENFOLD-TIE"]
plan_refs: ["§43.6", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§40.5", "§43.12", "§39.12"]
---
## Background

**Purpose.** M3 makes results trustworthy enough to be compared with a rival, and then makes the first comparison.
Places (interior scenes), tasks, adversarial worlds, autopsy and blinded qualification land together because the
**partial ten-fold gate** needs all of them: a mission to answer, a way to accept the answer, and a way to break
it.

**Why here.** The program's claim is a ratio of design-loop times against the strongest composed incumbent. A
ratio of times means nothing unless both arms' answers are accepted by the same rule, on worlds neither arm tuned
on, with failures searched for and explained. Those are the capabilities of this milestone. The partial gate is
held at M3, before the costly levers of M4 (family policies, inheritance) are built, so that their funding is
decided on a measured loop: a class whose measured ratio is under 3 leaves the speed claim, and where a short
program over an incumbent engine ties within a factor of 1.5 the layer ships over that engine. The exit line is
that the gate is *recorded, whatever the ratio*. Planning estimate: 6 weeks (DESIGN). Critical path: yes. M3
carries 77 convergence parts and 28 kill decisions of the compiled tables.

**Demonstrates.** NS-03 *An honest A/B*: `frob eval compare A B --task warehouse-tote` runs both designs on common
world addresses under a declared anytime-valid study and reports the evidence ratio, valid at any stopping time,
separately from whether each design meets the hard limits. NS-04 *A kitchen with a wet floor*: a humanoid crosses a
generated commercial kitchen whose wet patch obeys the film-friction law; telemetry reports slip per stance, the
friction utilization margin and stills at the slip. NS-09 *The exploit is caught*: a policy exploiting contact
compliance falls outside the promotion envelope on the referee and is labelled `provisional: tier-sensitive`; a
design whose rank changes under another admissible contact law is `model-conditional`. NS-15 *A minimal
counterexample world of three edits*: a `.fmcw` file (a sill raised 9 mm, friction 0.31 on one tile, a depth packet
delayed 40 ms), each edit necessary, replaying bit for bit. NS-16 *An autopsy that names the cause*: twelve typed
interventions on forks of the pre-fall snapshot; lifting the knee torque limit prevents the fall in 94% of forks
against a placebo flip rate of 3%. NS-17 *Qualification that survives 500 adaptive retries*: submission i runs on
fresh sealed clusters at level α_i = α / [i·(i + 1)].

## Scope

In: seven interior archetypes with hazard lists; film patches, rigid proxies, the world bank; clearance complexes
and the mesh path; task families through tight-space maneuvering, requirement verdicts, fixture 2, scorecards; the
blinded qualification lane; Nemesis; Autopsy; promotion with tier correction, exploitation detectors, the
model-form check; the daemon with slot dispatch, capabilities and budgets; the full MCP surface; the dashboard; the
design corpus and benchmark; baseline controllers for all eighteen archetypes; the sampling controller as
fallback; the Gymnasium-compatible adapter; mission queries under amendment; the rank-agreement pilot on incumbent
tools; the partial ten-fold gate with a phase ledger per arm.

Explicitly not in M3: family policies, inheritance, rank-agreement allocation on our engine, region certificates
at scale, retrofit search, shields (M4); tracked and hybrid briefs in the ten-fold test (first measured at M7).

What M4 needs from M3: the measured `train` share per class (decides how much of lever 2 is funded as a speed
lever); the corpus and benchmark; the pilot τ; OPEN-CORTEX-1 answered or defaulted, on M4's first day.

## Technical approach

**1. The artifact.**

- Seven interior archetypes with hazard lists: `frob scene gen`, `frob scene accept`, `frob scene hazards`,
  `frob scene split`; film-friction patches and rigid proxies; the world bank (`frob scene bank`).
- Clearance complexes (`.fclear`) that return a route witness or an impossibility cut; the mesh path with
  enclosures and thin-feature detection.
- Arena task families through tight-space maneuvering; requirement verdicts (`frob task verdict`); fixture 2;
  scorecards with full denominators.
- The blinded qualification lane: `frob eval qualify` on sealed world clusters with pre-committed requests.
- Nemesis: `frob nemesis search`, `frob nemesis shrink` to a `.fmcw` file, `frob nemesis bound`.
- Autopsy: `frob autopsy fork`, `frob autopsy attribute`, `frob autopsy repair`, with the placebo fork.
- Promotion with tier correction (`frob eval promote`), the exploitation detectors (`frob eval detect`) and the
  model-form rank check (`frob fidelity modelform`).
- The daemon with slot dispatch, capabilities and budgets; the full MCP surface in task clusters of at most seven
  tools; the dashboard `frob-tui`.
- The design corpus with frozen labels and the design benchmark; baseline controllers for all eighteen
  archetypes; the sampling controller as fallback (`frob control mpc`); the Gymnasium-compatible adapter.
- Mission queries under amendment: `frob mission revalidate`, `frob mission separate`, and the expert grid behind
  `frob experiment suggest`.
- The rank-agreement pilot on incumbent tools, and the partial ten-fold gate on legged and wheeled briefs, with a
  phase ledger for each arm.

**2. Gates with a part due at this exit (compiled register tables: 6 admission, 77 convergence).**

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| INT-FSIM-13 | Admission | M3 | Foundation: frankensim group `spectral`. The Floquet multipliers of the monodromy of ẍ + x = 0 over one period are both 1 to 10⁻¹²; a monodromy with a multiplier of modulus 1.01 is reported unstable | §33.9 |
| INT-FSIM-14 | Admission | M3 | Foundation: frankensim group `stats`. The ranking-flip probability Φ(−Δ/√(s₁² + s₂²)) to 10⁻¹²; a multilevel estimate of E[x²] under a standard normal is 1 within its stated interval; importance weights that miss the failure region trip the effective-sample-size floor and the estimate is refused | §33.9 |
| INT-FMCP-01 | Admission | M0 (admission); M1 (M1 verbs); M3 (full surface) | Foundation: fastmcp_rust. `frob-mcp` builds in the core graph on T with exactly one asupersync package (`cargo tree -d` shows no duplicate), lists the tools generated from the verb registry, and passes the MCP protocol smoke test over stdio and HTTP | §33.6 |
| INT-FTUI-01 | Admission | M3 | Foundation: frankentui [F]. One asupersync package or none in its graph; the job pane rendered from a recorded event stream equals its stored snapshot in the headless harness; idle processor use at most 2% of one core | §33.15 |
| INT-F3D-01 | Admission | M3 | Foundation: franken_threed (pattern; subprocess) [F]. Ten reference frames of the replay page built through the route match the plain bundle within 1/255 per channel on at least 99.9% of pixels; an unadmitted feature fails explicitly | §33.15 |
| INT-FDGR-01 | Admission | M3 (schema fixtures); M5 (scan import) | Foundation: drone geometry reconstruction (pattern; subprocess) [F]. The schema fixtures validate; a geometry generation whose scale status is not metric is refused; coverage masks become the scene's unknown-space label; privacy inheritance applies | §33.15 |
| GATE-ARENA-02 | Known answer | M3 | The scene-split check rejects an evaluation that reuses a training scene. | §24.17 |
| GATE-ARENA-07 | Known answer | M3 | Scorecard accounting: a planted infeasible scenario appears in its column and in the all-scenario denominator; columns sum to the battery size on 100 random batteries; a silently shortened battery changes identity. | §24.17 |
| GATE-ARENA-09 | Design comparison | M3 | Requirement writing: on 50 scripted mission statements, each with a reference formula and 20 labelled trajectories the agent does not see, at most 20% of first-draft requirements mislabel a trajectory, and `task.check` rejects every draft that does. | §24.17 |
| GATE-ATEL-01 | Design comparison | M3 | BET-05 on the design corpus: area under the ROC curve at least 0.8 and at least 0.05 above the heuristic baseline at a 95% paired lower bound; planted defective designs flagged and a planted sound design passed. | §28.23 |
| GATE-ATEL-04 | Known answer | M3 (comparison); M4 (bisect) | Comparison type-I error at or below α under optional stopping over 10⁴ null comparisons; `design bisect` finds the planted edit in at least 95 of 100 trials. | §28.23 |
| GATE-ATEL-07 | Design comparison | M3 (pilot); M4 | The rank-agreement experiment per morphology class: Kendall τ on the top band at least 0.7 for `selective` and at least 0.5 for `discard-only`, and ρ_ep at least 0.7, all at the lower bootstrap bound, on 40 designs. A class below 0.5 is `uninformative`. | §28.23 |
| GATE-ATEL-08 | Design comparison | M3 (pilot); M4 | Layer 2 against the fixed table: held-out coverage at least 1 − α − 0.03 per stratum; audited top-k recall no lower than halving's; a paired 95% lower bound above 1 on GPU-seconds to the same accepted decision; a planted over-scored design caught at the next rung; synthetic drift trips the alarm. | §28.23 |
| GATE-ATEL-11 | Known answer | M1 (dependency grades on mission cells); M3 (dependency grades); M4 | Revalidation on the frozen 30-amendment campaign: every decision equals full recomputation; zero unsafe reuse; the stale-key mutant is caught; unnecessary invalidations are at most 20% of unaffected verdicts, counted and reported. | §28.23 |
| GATE-ATEL-13 | Known answer | M3 (DC-01, DC-09, DC-10); M4 (the rest) | Each decision core: its table regenerates bit-identically; its null calibration holds (PO-ATEL-01 to -05, -10); its fallback is demonstrated by forcing each trigger; its shadow run records both actions. | §28.23 |
| GATE-AUGUR-06 | Proof | M1, repeated at M3 | Checker independence: dependency closure is `std`; zero shared source files with proposers; a seeded-fault campaign of at least 200 mutated witnesses per proof kind yields zero false accepts. | §25.27 |
| GATE-AUGUR-08 | Known answer | M3 | Monotone transport: zero false transports on a planted set of at least 20 non-monotone fixtures (including the widened sole); the falsification harness finds every planted reversal within 10⁴ ordered pairs. | §25.27 |
| GATE-AUGUR-09 | Known answer | M3 (pilot); M4 | Region certificates on the 12-parameter box: the per-constraint arm decides at least 50% of each constraint's sub-box volume within 10⁴ boxes; no single monotone bound gives more than 95% of decided volume; zero certified boxes contradicted by 10⁶ point evaluations. | §25.27 |
| GATE-AUGUR-10 | Known answer | M3 | Alias and jaw fixtures exact; `mission.refute` on the blind payload task returns all three engineering answers with their conditions. | §25.27 |
| GATE-AUGUR-12 | Known answer | M3 | Revalidation on the frozen 30-amendment campaign: zero incorrectly inherited verdicts; unnecessary invalidations at most 20% of unaffected verdicts; the cost-9 and cost-5 fixture keeps the witness and withdraws the minimum. | §25.27 |
| GATE-AUGUR-13 | Design comparison | M3 | Separation: for each of 10 planted design reversals, `mission.separate` returns an admissible world pair whose minimized edit list has at most 3 edits and whose first cause is unchanged by minimization, confirmed on the referee. | §25.27 |
| GATE-AUGUR-14 | Performance | M3, M4 | Ceiling before speed: every registered speed comparison in this subsystem records measured f, r and h on the strong baseline and is refused when 1/(1 − f + f/r + h) is below the claimed ratio. | §25.27 |
| GATE-AUTOPSY-03 | Known answer | M3 | Planted single causes: the planted factor ranks first in at least 90% of 200 failures in double precision, and in the single-precision lane for windows of at most 1 s, and the top factor changes in at most 30% of cases under 10⁻⁷ perturbation; the agreement between ten one-at-a-time forks and the full coalition table is recorded. | §27.17 |
| GATE-AUTOPSY-04 | Known answer | M3 | Coalitions: planted joint and alternative causes are labelled correctly in at least 95 of 100; the Shapley identities hold exactly; the negative-value case is reproduced. | §27.17 |
| GATE-AUTOPSY-05 | Known answer | M3 | Abstention: a planted chaotic failure and a planted no-cause failure name no factor in 100 of 100. | §27.17 |
| GATE-AUTOPSY-06 | Known answer | M3 | The thermal counterexample: all four numbers within 0.1 s and 0.1 K; the mid-episode request is refused and the re-run passes. | §27.17 |
| GATE-CHRON-09 | Known answer | M3 | Tier correction: equal to the plain mean at n = N; type-I error at most α over 10⁴ null comparisons under optional stopping; coverage at least nominal − 0.03; a planted between-tier reversal never established; referee episodes per established verdict 16 / 64 / 256 at the 50th / 95th / 99th percentile. | §29.20 |
| GATE-CORTEX-04 | Known answer | M3 | Each detector of §23.16 flags its planted exploit and passes an honest policy. | §23.23 |
| GATE-CORTEX-10 | Known answer | M3 | Streaming past-time robustness equals offline evaluation on 10⁴ random traces; the mixed-unit fixture binds on the clearance in both unit systems; a future-time online reward is refused. | §23.23 |
| GATE-CORTEX-12 | Design comparison | M3 as a pilot on incumbent tools; M4 on ours | The rank-agreement experiment of §23.11 is run and its consequence applied: τ, top-10 recall and intervals are recorded per class and task family, and a rung without a record is refused. The gate passes when the record exists and the rule is enforced, whatever τ is. | §23.23 |
| GATE-CORTEX-15 | Known answer | M3 | Planted box-cone and penetration-energy exploits flip a planted ranking and are typed `model-conditional`; a genuine winner stays `model-robust`; the cost is at most 10% of the campaign; FROB-REF-830 fires. | §23.23 |
| GATE-CORTEX-16 | Parity | M3 | 10³ episodes driven through the adapter and natively have equal digests (D2); the final observation is in `info` in every autoreset mode; an external PPO reaches the threshold of GATE-CORTEX-01 through it. | §23.23 |
| GATE-FID-02 | Fidelity | M3 | The promotion rule is exercised end to end: a planted tier-sensitive policy is caught (NS-09). | §16.17 |
| GATE-FID-04 | Fidelity | M3 | Rank agreement between `batch-cpu` and the referee on the 40-design set per reference class, with the permitted use per stratum written to the ledger; per-episode ρ on 256 paired episodes per design reported. | §16.17 |
| GATE-FID-05 | Known answer | M3 | Model-form check: the planted pyramid-cone and penetration-recovery winners are returned as `model-conditional`; a genuine winner is `model-robust`; cost at most 10% of the campaign's compute. | §16.17 |
| GATE-FID-08 | Known answer | M3 | Tier correction: type-I error at most α over 10⁴ null comparisons with optional stopping; equals the paired mean at pᵢ = 1; a planted between-tier reversal is not established; ρ and the replaced higher-tier count printed. | §16.17 |
| GATE-HAB-02 | Known answer | M3 | Per archetype, 1,000 seeds yield valid scenes: all constraints satisfied; each tagged `witnessed`, `impossible` or `unresolved` (at most 5% `unresolved`); `impossible: intended` within one percentage point of quota; none resampled; one seed gives one `SceneId` on Linux and macOS. | §19.19 |
| GATE-HAB-04 | Known answer | M3 | Scene acceptance: every fixture of §19.5.6 passes on 100 scenes per archetype; the reflected room is rejected and the three-point control accepts it; the 6 mm case returns `unknown`. | §19.19 |
| GATE-HAB-05 | Known answer | M3 | Separation of optics and friction: the four-state tile behaves as in §19.5.4 on every tier, bit for bit within the determinism class. | §19.19 |
| GATE-HAB-06 | Known answer | M3 | Film-friction law as scenes consume it (the law's own gate is GATE-TRAC-05): the three squeeze times within 2%; the R⁴ scaling; dry and thick-film limits recover the card coefficients. | §19.19 |
| GATE-HAB-10 | Known answer | M3 | Clearance complexes [F]: the corridor closed form bracketed by witness and cut on 100 (a, b) pairs; zero false witnesses and zero false cuts on the seven archetype fixtures against a brute-force grid at four times the resolution; the checker accepts every certificate and rejects six mutated ones. | §19.19 |
| GATE-HAB-12 | Performance | M3 | World bank: resident counts recorded against §19.15 (targets: at least 2,000 homes at 2 cm on the M5 Ultra; at least 150 on one RTX 4090 beside an 8,192-world quadruped batch); admission never exceeds the cap; a world changes scene at reset with no recompilation. | §19.19 |
| GATE-HAB-14 | Known answer | M3 | Hygiene: a mirrored layout and a retrofit variant planted across a split are refused; feasibility counts in manifests equal recounts. | §19.19 |
| GATE-MARROW-03 | Known answer | M3 | Γ_mech and Γ_obs equal the groups on file for all eighteen archetypes of §14.10 and for 24 planted asymmetries (a moved sensor, a one-sided payload, a changed sole material, a reversed gear), with the correct breaker named in every case; zero disagreements. | §13.10 |
| GATE-NEMESIS-01 | Known answer | M3 | Legality and oracle: 100 of 100 planted illegal edits refused; 0 of 100 planted impossible traps counted; at least 95 of 100 planted solvable traps receive a witness. | §26.15 |
| GATE-NEMESIS-02 | Design comparison | M3 | Planted traps of nominal probability at most 3 × 10⁻⁵, 30 paired repetitions: a first promoted failure with at least 3 times fewer simulated steps than card-uniform randomization at a 95% lower bound; at least 70% of reported counterexamples `solvable-causal` or `plant-feasible`. | §26.15 |
| GATE-NEMESIS-03 | Known answer | M3 | Shrinking: planted causes recovered exactly in at least 95 of 100 trials; every `.fmcw` replays with identical digests in 100 of 100; on unplanted failures the share of worlds with at most three edits is recorded. | §26.15 |
| GATE-NEMESIS-04 | Known answer | M3 | Bounds: over 10⁴ synthetic studies with known p the 95% upper bound covers p in at least 95%; the defensive mixture covers under a mis-centred proposal; non-nominal episodes are refused. | §26.15 |
| GATE-NEMESIS-05 | Design comparison | M3 | Exploration on planted failures of probability 10⁻⁴ and 10⁻⁵ (one with two separate modes), equal episode budget, 30 repetitions: at least 10 times fewer episodes than crude sampling at 10⁻⁵; both modes found in at least 80% of repetitions. | §26.15 |
| GATE-NEMESIS-06 | Known answer | M3 | Guard: 100 of 100 planted artifact failures labelled and none in the hard-world set; at least 95 of 100 genuine planted failures survive promotion. | §26.15 |
| GATE-NEMESIS-08 | Known answer | M3 | Coverage: lazy and plain greedy agree; on 1,000 random instances of at most 20 scenes the brute-force optimum is at or below the certificate and greedy is at or above (1 − 1/e) of it. | §26.15 |
| GATE-PHYLUM-01 | Known answer | M3 | Orbit and ballast rows of §17.18 exact; zero false quotients in 10⁴ sampled class pairs by replay; the lidar and equal-Σx controls caught. | §17.19 |
| GATE-PHYLUM-05 | Known answer | M3 | Invalidation contract: the positive control reuses, each negative control invalidates or refuses, and 10⁴ random edit sequences give results equal to full recomputation. | §17.19 |
| GATE-PHYLUM-07 | Known answer | M3 | Generator: reproducible across platforms; exact probabilities on enumerable sub-grammars; rejection rates reported; no orbit straddles a `DesignSplit`. | §17.19 |
| GATE-QUAL-01 | Known answer | M3 | Over 2,000 simulated no-signal campaigns at α = 0.05, each with 500 adaptive retries by a scripted adversary that sees every receipt and all discovery data: a false promotion in at most 123 campaigns; a planted true gain of the declared size promoted within 10 requests in at least 80% of campaigns. | §24.17 |
| GATE-QUAL-02 | Known answer | M3 | Firewall: 10⁴ fuzzed read attempts across every verb return no sealed field; a planted leak path is found by the audit; receipts contain only the listed fields. | §24.17 |
| GATE-REFLEX-01 | Known answer | M1 (referee); M2 (`batch-cpu`); M3 (all but tracked and hybrid); M4 | Each archetype stands for 10 s and traverses 5 m of flat ground under its baseline controller: at M1 the first archetypes, on `referee` only, restricted to those whose contact law exists at M1 and whose contact count fits the referee's 21-contact cap (§16.6); at M2 the same set on `batch-cpu`; at M3 all eighteen on `batch-cpu`, except tracked and hybrid archetypes, which follow at M4 with their contact laws. | §22.18 |
| GATE-REFLEX-02 | Design comparison | M3 | BET-06: at least 80% of corpus designs that pass `design check` traverse 5 m, as a paired 95% lower bound. Potency control: every corpus design with a planted control-relevant defect fails. | §22.18 |
| GATE-REFLEX-04 | Design comparison | M3 (legged, wheeled); M4 (tracked, hybrid) | Rank agreement of the baseline rung with trained specialists on 30 to 40 designs per morphology class and task family, thresholds fixed before data: a Kendall τ lower 95% bound of at least 0.7 lets the rung select; at least 0.5 lets it discard the bottom half; below 0.5 it removes outright failures only (`baseline-only: unranked`). | §22.18 |
| GATE-REFLEX-05 | Known answer | M3 | PO-REFLEX-02; every command has a layer record; a planted infeasible program yields a certificate that checks exactly and a fallback event; a run that needs commands beyond the envelope is marked `actuator-infeasible`. | §22.18 |
| GATE-SAFE-02 | Known answer | M3 | Injection: at least 200 hostile strings pass through every verb that echoes imported text; none appears outside an untrusted-marked field, and none appears in any tool description, prompt or `next_actions` command. | §37.14 |
| GATE-SAFE-03 | Known answer | M3 | Confirmation isolation: a session holding every discovery scope attempts at least 25 enumerated read paths against a confirmation run; all return `FROB-REF-840`. Negative control: a test build that leaks per-world outcomes is detected by the adaptive-retry gate of §24. | §37.14 |
| GATE-SAFE-04 | Known answer | M3 | Privacy: for a private scene, each derived artifact kind carries the mark; exports without the flag are refused; after `scene.purge` no digest of the closure remains in the store. | §37.14 |
| GATE-SAFE-05 | Known answer | M3 | Licences: over random derivation graphs the licence set equals the join of the inputs; a research-only asset cannot be made to enter an exported report or pack. | §37.14 |
| GATE-SAFE-09 | Known answer | M3 | D4: a result produced under the nondeterministic training profile is refused on each of the five paths of INV-SAFE-07; the same policy re-evaluated under D2 is accepted. | §37.14 |
| GATE-SENS-07 | Known answer | M3 | Optical failure modes: the mirror, glass, steam and pattern-dropout cases of §20.20; every masked sample carries a reason; repeated reads through one pane fail together. | §20.21 |
| GATE-SENS-08 | Known answer | M1 (docking rows); M3 | Observability: the corridor case and the four-dimensional gauge, by the linearized and the empirical form, agreeing to within 5% in the non-zero eigenvalues. | §20.21 |
| GATE-SENS-09 | Known answer | M1 (docking rows); M3 | Alias witness: the T-junction and the late-packet docking witnesses are found and pass the checker; on 64 planted layouts every planted alias is found and no false witness is reported. | §20.21 |
| GATE-SENS-10 | Known answer | M3 | Γ_obs: the two cases of §20.13 on every template; the equivariance test passes for members and fails for non-members. | §20.21 |
| GATE-SENS-11 | Known answer | M3 | Calibration: the triangle and four-sensor residual patterns; the planted bump recovered within 0.1° and 1 mm and attributed to the right node in 100 of 100 seeds. | §20.21 |
| GATE-SENS-14 | Known answer | M3 | Sensor cards: every failure mode of §20.8 marked [S] has its first check passing, and every card field reports its provenance. | §20.21 |
| GATE-SKEL-03 | Known answer | M1 (archetypes landed); M3 (all eighteen) | Each of the eighteen archetypes compiles, passes stages 1 to 6 of `design check` with no refusal, has the symmetry groups on file, and stands or rolls under its baseline controller at the milestone GATE-REFLEX-01 gives for it. | §14.15 |
| GATE-SKEL-04 | Known answer | M1 (stages 1 to 6); M3 (stages 7 and 8) | Stages 1 to 6: of 100 seeded defects (at least two per refusal code) all refused, at least 95 by the stage and code that own them; zero refusals on the eighteen archetypes and on 10³ random valid designs. Stages 7 and 8: of 40 planted cases at least 36 decided correctly within budget and none wrongly. | §14.15 |
| GATE-SKEL-07 | Known answer | M1; M3 (all eighteen) | PO-SKEL-02 holds exactly; the mass of each archetype lies within 15% of the mass on file for its reference robot where one exists. | §14.15 |
| GATE-SPARK-17 | Known answer | M3 | Each licence refusal of §35.3 fires on its planted case (a D4 arm in a paired comparison; attribution below D2; a cited training return; a cross-family pair demanded as paired), and a policy trained under `fast-training` re-evaluates to the same digest twice. | §35.15 |
| GATE-TETHER-01 | Design comparison | M3 | A fresh agent given only `robot-docs` completes 20 scripted studio tasks with at least 18 correct and no budget violation. | §32.21 |
| GATE-TETHER-09 | Performance | M3 | Slot invariance: 1,000 episodes solo and in 100 random co-tenancies have identical digests. At 8,192 identical worlds shared dispatch loses at most 5% throughput and 99th-percentile step time is at most 1.10 of solo. Admission to first step for jobs of up to 256 worlds: 0.1, 0.5 and 2 s at the 50th, 95th and 99th percentiles. | §32.21 |
| GATE-TETHER-10 | Performance | M3 | Scheduling on a replayed mix (90% interactive) at utilization 0.7: interactive wait at the 95th percentile at most 6 s; batch slowdown at most 10%; no job beyond the ageing bound; random preemption and reordering leave every digest and verdict identical. | §32.21 |
| GATE-TETHER-11 | Known answer | M3 | The 90% estimate interval covers the actual cost in 85% to 95% of at least 200 held-out jobs per stratum; a stratum with fewer than 19 points reports `uncalibrated`. | §32.21 |
| GATE-TETHER-12 | Known answer | M3 | MCP: every `run` tool returns within `sync_wait_ms` plus 500 ms with a result or a handle; a wire-cancelled wait leaves the job running; after `session_focus` at most seven tools plus the base three; each resource kind of §32.12.7 listed and readable; prompts match `robot-docs`. | §32.21 |
| GATE-TETHER-14 | Known answer | M3 | Dashboard: a snapshot test per pane on a recorded daemon trace, and every displayed number is reproduced by its source verb. Viewer: on analytic scenes the overlay value under a probe pixel equals the telemetry channel within its quantization. | §32.21 |
| GATE-TRAC-05 | Known answer | M3 | Film law: Stefan times within 2%; Couette force within 10⁻⁶ relative in `f64`; the dry limit bit-identical; the limits at h ≤ σ and h ≥ 3σ and continuity between them on 10⁴ random states. | §18.17 |
| GATE-TRAC-07 | Known answer | M3 | Enclosures: on 10⁴ generated pairs with known truth, no false `clear` and no false `penetrates`; the 6 mm case answers `unknown`; every certificate re-checked offline (§12). | §18.17 |

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-WS-07 | the suite row | Known answer. Each of the four planted instances raises its named signal in 100 of 100 runs; `xtask check-deps` resolves the graph with every feature group enabled and fails on the planted second pin; no row of the register lacks a planted instance. M0 for the two build rows, M2 for the sampler row, M3 for the suite row. | §41.5 |
| GATE-GAUNT-04 | the lanes | Known answer ¦ Certifiers: every row of §38.12 meets its threshold; the certificate mutation corpus holds at least 1,000 mutants per proof kind with zero accepted ¦ M1 for the checker; M3 for the lanes | §38.15 |
| GATE-ARENA-14 | second part | Known answer; Design comparison ¦ Fixture: a slick strip over x ∈ [2.0, 2.5] m (`field`, per 0.25 m tile), a push at t = 3 s (`process`), a push at the first crossing of a gate at x = 5 m (`event`), a baseline controller and one retimed from a 0.5 s to a 0.4 s gait period. With 1,000 unused draws inserted, the step halved and a limb added: tile values bit-identical; the timed push at 3 s in every arm; the gate push at each arm's crossing; per-arm outcomes indistinguishable from independent sampling in a two-sided test at level 0.01 on 4,096 worlds. The tick-keyed and footfall-keyed controls are refused. On a held-out slip-and-dock battery the variance of the contrast is at most half that of named streams keyed per stream and tick, at equal count ¦ M2 for the fixture; M3 for the battery | §24.22 |
| GATE-GAUNT-06 | the daemon | Known answer ¦ Chaos: every row of §38.9 ends in its typed refusal or an accepted state with no leaked resource; 10⁵ seeded schedules per release; every await point on the publish and submit paths hit ¦ M2 for kernels; M3 for the daemon | §38.15 |
| GATE-SKEL-09 | whole | Known answer ¦ The two stance cases and the trot counterexample exactly; inner ⊆ outer on 10⁴ random directions per archetype module; an edit inside one module re-characterizes one table and no other ¦ M3 | §14.16 |
| GATE-SENS-16 | whole | Known answer ¦ The box-and-mast case: hidden floor boundary at 1.000 m and 0.600 m within 1 cm; no hidden region beyond the body's front edge with the payload removed; a planted occlusion by a swinging limb is found and attributed to the link; lead and blind interval identical on every tier inside their determinism class ¦ M3 | §20.24 |
| GATE-REFLEX-11 | whole | Known answer; Fidelity ¦ The cart: the privileged run is refused as evidence, and the nominal-model planner overruns by 0.612 m within 1% on the slippery draws. On the M3 corpus, every delivered-controller verdict on the fast tier survives promotion to the referee plant or is labelled `tier-sensitive`; the measured 99th-percentile planning time fits the control period on the named computer in at least 99% of ticks; the phase ledger prints the lane's cost beside the cost of training once ¦ M3 | §22.22 |
| GATE-CORTEX-21 | whole | Known answer ¦ On the forty-design fixture `eval.rank-agreement` returns τ = 41/45 on the true top ten, top-1 recall 0 at k = 2, containment rank 3 for m = 1 and positive regret at k = 2. A request to select with k = 2 is refused and the same request with k = 3 is accepted ¦ M3 | §23.28 |
| GATE-ARENA-11 | whole | Known answer; Design comparison ¦ The hand case exactly. On 30 tasks with one planted loophole each (off-site contact, tumbling, off-route travel, last-instant success, energy-creating contact) and 30 clean tasks: at least 80% of planted loopholes found before any design campaign, at no more than 5% of a reference campaign's compute per task, with findings on at most 20% of the clean tasks ¦ M3 | §24.20 |
| GATE-ARENA-12 | whole | Known answer ¦ The two-value fixture reproduces 0.0975, 0.0075 and 0.0085 in exact arithmetic; under the disjoint wrong model the mean of Ĉ over 10⁴ replays equals the true contrast within its Monte Carlo error and the variance ratio stays at or below 1/ε; a `fixed-n` request with item-bank sampling is refused ¦ M3 | §24.21 |
| GATE-ARENA-13 | whole | Design comparison ¦ Replay on a recorded full matrix of about 40 designs × 150 items × 4 replicates (24,000 episodes): for the 20 closest pairs, episodes to equal interval width lower by a factor of at least 4; coverage within 2 points of nominal; item parameters fitted on one family predict held-out designs better than per-item means ¦ M3 | §24.21 |
| GATE-ARENA-15 | whole | Known answer. On ten authored tasks with a planted safety-only reward and ten clean tasks, the idle arm flags ten of ten and none of ten (TARGET: at least nine, at most one). The hand case is flagged, and after the progress clause is accepted the parked robot's verdict is `violated`. M3. | §24.23 |
| GATE-AUGUR-26 | whole | Known answer. On the banded fixture the detector flags the non-monotone outcome in 100 of 100 seeds, no verdict is filled, and the mission answer leaves the band `unresolved`; on the creep rig the licensed fill equals the exhaustive grid at every point; a mutant that fills above the first success is caught on both. M3. | §25.30 |
| GATE-AUGUR-27 | whole | GATE-AUGUR-27 (Known answer, M3). On 1,000 generated monomial systems of at most 12 parameters and 24 constraints with planted status: no feasible system is excluded; every infeasible system whose contradiction has a product of at least 1.01 and fits the size caps is excluded; every certificate is accepted by `frob-verify`, and each of 200 mutated certificates (one multiplier or one coefficient changed) is rejected. The leg fixture returns λ = (1, 1) and 8829/8000; its control returns the witness; the two-term case (0.9, 0.15) is excluded at weights (6/7, 1/7) and not at equal weights. | §25.31 |
| GATE-AUGUR-33 | the error-law rows | Known answer. Enumeration of the 128 histories returns 55/64 for the pairwise law, 3807/4096 for the independent law and 3/4 for the shared fault, exactly; a redundancy claim on a card marked `pairwise` is refused. The three label rows return 2, 3 and 5 bits with codes whose received-word sets are disjoint; the four-bit request under `flips(1)` returns an exclusion with the counting argument. On 20 generated instances with L ≤ 8 and e, t ≤ 2 the least length equals brute force. The error-law rows at M3, the label rows at M4. | §25.37 |
| GATE-AUTOPSY-09 | whole | Known answer; Design comparison ¦ The linear map and the pendulum control behave as stated. G(W) is recorded for the reference quadruped and humanoid policies at 0.5 s, 1 s and 2 s. On the planted-cause suite of GATE-AUTOPSY-03 with windows of at most 1 s, the lane names the same top factor as double precision in at least 95% of admitted cases. A planted solver artifact that double precision does not reproduce is labelled `tier-specific` with the right variant. Bulk attribution of 200 failures is at least 3 times faster in wall time than on `batch-cpu` ¦ M3 | §27.19 |
| GATE-ATEL-17 | whole | Known answer ¦ On 10³ synthetic designs drawn from the law with planted (J∞, a, γ) and paired noise, the calibrated interval covers the planted J∞ at a rate of at least 1 − α − 0.03; the worked case returns 0.726, 0.888, 0.95 and 0.62; the threshold case is refused ¦ M3 | §28.25 |
| GATE-ATEL-19 | whole | Known answer; Performance ¦ On one frozen lineage of 30 edits spanning the five classes, the three columns are reported cold and warm, with the from-scratch control run for at least one edit per class. The columns plus recomputed work sum to the measured total within 2%. A controller-only edit shows exact reuse of every model analysis, and a mass edit shows exact reuse of none of its rollouts ¦ M3 | §28.26 |
| GATE-FOUNDRY-15 | whole | Known answer; Design comparison ¦ The tube case returns t = 1.85 mm and 0.141 kg within 1%. A planted under-built link is flagged before the first rollout and blocks promotion. The check adds at most 50 ms to `design.check` on the reference robots. The corpus comparison is run and its share recorded; if the winner or its actuator selection changes in fewer than 1 campaign in 15, the check stays as a promotion guard and `{shell: sized}` is not made the template default ¦ M3 | §31.23 |
| GATE-TETHER-15 | whole | Design comparison. On the 30 tasks a fresh agent names the seeded cause first in at least 16 of 20 seeded tasks; converts no inconclusive or no-defect task into a cause; runs at most twice the minimal number of simulations in the median task; and every diagnosis is reproduced by a clean client. A second arm with `sim.run` and logs only is run as the comparison, and the difference is reported with its interval. M3. | §32.23 |
| GATE-TETHER-16 | whole | GATE-TETHER-16 (Known answer, M3). For the docking fixture of §24.10 and the split-grip fixture of §24.11, each with a parent and 16 forks: the file is at most 8,388,608 bytes; it opens and plays from a local file with the network disabled in two pinned browsers under a headless driver; two exports are byte-identical; every block digest equals the source's; 100 probe points per packet equal `telemetry.query` within channel quantization; the null-fork deviation is zero; the over-cap request is refused with its size table. | §32.24 |

**4. Program gates (beads N40-14, N40-15, N40-19, N40-20, N40-22).** Before the sealed suite is opened:
GATE-TENFOLD-03 (the harness reproduces its known answers and catches its controls), GATE-TENFOLD-04 (freeze and
seal in six ordered steps), GATE-TENFOLD-05 (adjudicator acceptance), GATE-TENFOLD-10 (the pre-analysis known
answers 10.081, 9.677, 1.8215, 0.78568 and the counts 32, 46, 49, 182) and GATE-TENFOLD-11 (the rival audit
complete before sealing). Then GATE-TENFOLD-06, the partial gate: on `legged-quad` and `wheeled`, and on
`envelope` and `sensing` where ready, the pooled ratio with its interval, verdict, completion, coverage, tie test
and phase ledgers for arms A, B and C, with the outcome rules applied; met when the record exists, whatever the
ratio; M3 does not exit without it. GATE-TENFOLD-13: the recovery ratios with 95% intervals and the three phase
ablations. GATE-TENFOLD-09, GATE-GROUND-01, GATE-SAFE-01, GATE-GAUNT-01 to -03 recur.

**5. Kill gates decided here.** In brief: KILL-STATIC → `design check` verdicts become advisory lints and the
pruning claim is withdrawn. KILL-BASELINE → synthesis is limited to the classes that pass; others route to the
sampling fallback or to training. KILL-NEMESIS, KILL-RARE, KILL-MCW → card randomization with the legality
profile, the oracle and the guard; crude sampling with a binomial bound; full failing worlds with replay.
KILL-AUTOPSY → single-factor paired re-runs from the episode start. KILL-DR and KILL-SHADOW → promotion from
referee episodes alone; the referee at promotion time only. KILL-FAMILYCERT and KILL-AUGUR-TIE, read at the
partial gate → mission answers come from generic solvers behind the same verbs, schema and checker.
KILL-AUGUR-FALSECERT applies to both arms of the partial gate. The rule of BET-13 is applied per mission class at
the partial gate (KILL-TENFOLD-SPEED, -TIE, -REACH for the classes run).

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-FAMILYCERT | At the partial ten-fold gate no family certificate has decided a cell that the generic comparator of D-17 leaves undecided at equal cost | Mission answers come from the generic solvers alone and still ship in the bundle format; the specialized projections are removed | M3 | §12.6 |
| KILL-SKEL-ALIAS | Stage 8 returns a witness that a hand-built policy defeats, or decides fewer than half of the planted cases within budget | Stage 8 becomes an advisory report built on A-14 coverage and refuses nothing | M3 | §14.15 |
| KILL-SHADOW | GATE-FID-07 fails on cost or on coverage after one repair cycle, or no planted defect is detected | The referee runs at promotion time only, plus an offline audit of sampled snapshots after each run | M3 | §16.17 |
| KILL-QUOTIENT | One false quotient survives a repair cycle, or digest classes save less than 5% of dispatches on the campaign corpus | Orbit representatives only; the digest is kept as a cache key with no claim | M3 | §17.19 |
| KILL-FILM | On the wet-floor battery the film law changes fewer than 10% of episode outcomes against a constant-coefficient patch, or adds more than 5% to step time | Static condition cards on the lockstep tiers; the law is kept on the transactional profiles for audits | M3 | §18.8 |
| KILL-MESHGPU | Mesh traversal on a GPU breaks fixed launch shapes or adds more than 15% to the step | GPUs see thin geometry as analytic primitives only; meshes stay on the CPU tiers | M3 at the earliest, before the work package | §18.3 |
| KILL-BANK-BREADTH | Held-out success does not rise with scene breadth beyond its confidence sequence | The bank serves evaluation, qualification, replay, clearance and retrofit search and large scenes; training keeps per-card resident sets | M3 | §19.15 |
| KILL-FCLEAR | One false witness or false cut survives a repair cycle, or more than 30% of doorway queries on the archetype battery stay `unresolved` after refinement | The route-graph traversability check for a nominal footprint, labeled heuristic | M3 | §19.11 |
| KILL-ALIAS | One false witness from the searched lane | Structural, enumerated and declared witnesses only | M3 | §20.21 |
| KILL-BASELINE | GATE-REFLEX-02 fails (BET-06) | Baseline synthesis is limited to the morphology classes that pass; others report `not-synthesizable` and route to training | M3 | §22.18 |
| KILL-GYMADAPT | GATE-CORTEX-16 fails | Rival stacks drive the environment through the job interface of §32 in whole rollouts, and the ten-fold test records the added latency in its phase ledger | M3 | §23.23 |
| KILL-SPEC-REWARD | In the training pilot the arm with the added requirement term does not improve success over the shaped reward across five seeds | Requirements stay acceptance metrics and search scores only; the added-term option leaves the task schema; margin-only training is offered only if it reaches 80% of the shaped arm's success | M3 | §24.17 |
| KILL-AUGUR-ALIAS | Generated alias witnesses are contradicted by a permitted probe in more than 5% of audited cases | Witnesses are labelled experiment-specific only; universal observability claims are refused | M3 | §25.27 |
| KILL-AUGUR-PROBE | The generator never beats the programmed expert plus direct measurement at matched correctness on the coupled-layout experiment, or issues one false lower bound | `experiment.suggest` ships the expert grid and the direct measurement; the experiment grammar is parked | M3 | §25.27 |
| KILL-AUGUR-REGION | The pilot of §25.6 meets its falsifier (more than 90% undecided in the best arm, or more than 95% of decided volume from one monotone bound), or GATE-AUGUR-09 fails after one repair cycle | Region certificates are removed; family answers use dual witnesses, monotone bounds and pointwise static analyses; untested regions are reported as sample coverage, `Estimated` | M3 | §25.27 |
| KILL-AUGUR-REUSE | One incorrectly inherited verdict from an implication rule | That rule is removed; reuse falls back to the dependency-keyed memo graph of §28 | Standing, from M3 | §25.27 |
| KILL-AUGUR-SEPARATE | Minimization changes the failure mechanism in more than 20% of planted cases, or a conventional scenario tool with the same generator matches the result at equal cost | Ship the Nemesis search and minimization without the design-pair query; keep counterexamples as regressions | M3 | §25.27 |
| KILL-AUGUR-TIE | On the strata of the M3 partial gate that use §25, a short robust program over a conventional solver reaches the same checked scope at comparable turnaround | Ship that program behind the same verbs, schema and checker; retire any claim of exclusive expressivity on those families; spend the research budget on the first mission the program cannot resolve | M3 | §25.27 |
| KILL-MCW | On unplanted failures fewer than half of shrunk worlds have at most three edits, or fewer than 90% replay | Nemesis ships full failing worlds with replay and single-edit contrasts ranked by margin; the format stays | M3 | §26.15 |
| KILL-NEMESIS | GATE-NEMESIS-02 fails after one repair cycle | The search methods are removed; Nemesis keeps the legality profile, the oracle, the guard and shrinking, applied to failures that card randomization finds | M3 | §26.15 |
| KILL-RARE | In the exploration pilot the gain is below 10 times at 10⁻⁵, or the coverage of any interval offered as a bound is below 85%, or a planted mode is missed in more than 20% of repetitions | Crude nominal sampling with the binomial bound alone; splitting is removed; tilting is kept only inside `nemesis.search` | M3 | §26.15 |
| KILL-AUTOPSY | GATE-AUTOPSY-03 fails after one repair cycle: planted-cause accuracy below 90%, or the top factor changes in more than 30% of cases at 10⁻⁷ | Single-factor paired re-runs from the episode start, with first-anomaly ordering labelled `Estimated`; coalitions and Shapley summaries are removed | M3 | §27.17 |
| KILL-STATIC | GATE-ATEL-01 fails (BET-05) | `design check` verdicts are downgraded to advisory lints and the pruning claim is withdrawn | M3 | §28.23 |
| KILL-DR | ρ_ep below 0.7 on 200 paired addresses in every stratum tested, or GATE-CHRON-09 coverage fails after one repair cycle | Promotion reports the label only, from referee episodes alone; the ledger keeps recording ρ_ep | M3 | §29.20 (§16.17) |
| KILL-MCPTASKS | The pilot of §32.12.6 fails, or the protocol extension changes incompatibly | Job handles only | M3 | §32.21 |
| KILL-PREEMPT | A resumed job's digest differs, or measured resume cost exceeds the quantum | Run to completion with the slot reserve only | M3 | §32.21 |
| KILL-SLOT | Any digest difference under co-tenancy, or GATE-TETHER-09's throughput or step-time limits fail after one optimization pass | `frobd --dispatch=per-job`: one environment per job, queued | M3 | §32.21 (§21.11) |
| KILL-TUI | FrankenTUI cannot be admitted with one asupersync package, or the snapshot tests cannot be made stable | No dashboard; `job.list` and `session.show` carry the same numbers | M3 | §32.21 |

| Kill gate | Trigger and fallback (from the defining subsection) | Decided | Gate beside it | Defined in |
|---|---|---|---|---|
| KILL-MPCRECORD | Trigger: More than 10% of delivered-controller verdicts reverse at the referee; or no named computer meets the deadline in 99% of ticks for any class on the pilot set. Fallback: The rule of §22.8 as first written: screening and fallback only. A mission that requires a sampling controller returns `provisional: controller-not-qualifiable` with its screening episodes. | M3 | GATE-REFLEX-11 | §22.22 |
| KILL-FOIL | Trigger: Detection below 60%; or findings on more than 30% of clean tasks; or the sampling controller on the actual design finds the same loopholes at the same cost, so foil bodies add nothing. Fallback: The in-campaign check on winning episodes only; `task.audit` runs the actual design alone. | M3 | GATE-ARENA-11 | §24.20 |
| KILL-ITEMBANK | Trigger: At GATE-ARENA-13 the factor is under 2 for the 20 closest pairs; or coverage falls more than 2 points below nominal; or the held-out prediction is no better than per-item means. Fallback: Stratified sampling by interior archetype with proportional allocation, which needs no model. | M3 | GATE-ARENA-12, -13 | §24.21 |
| KILL-EXPODOMAIN | Trigger: Under a factor of 2 in contrast variance on the held-out battery; or the automatic choice among couplings picks one that fails the test in any pilot. Fallback: The keyed streams of §35.9 as they stand, with explicit per-stream adapters; the four checks are kept as a test. | M3 (the exit at which the last part of GATE-ARENA-14 is due; the gate's first part is due at M2) | GATE-ARENA-14 | §24.22 |
| KILL-ENSEMBLE | Trigger: Agreement with double precision below 90%; or fewer than half of the reference failures admitted at any window of 0.5 s or more; or a bulk speedup below 3. Fallback: ADR-AUTOPSY-04 as first written: attribution in double precision only, and a failure not reproduced there goes to the artifact guard unattributed. | M3 | GATE-AUTOPSY-09 | §27.19 |
| KILL-REPLAYPACK | Trigger: GATE-TETHER-16 fails after one repair cycle, or the page cannot be held to its byte budget. Fallback: `run.export` to `.frun`, stills with overlays (`telemetry.stills`) and the live viewer; no single-file packet. The build packet of §31.12 may include a replay packet as an optional file and never depends on one. | M3 | GATE-TETHER-16 | §32.24 |

**6. Exit rule.** Recorded by a fresh-context session (seed BD-319, procedure N40-05). The exit line:
*GATE-TENFOLD-06 recorded, whatever the ratio.*

**7. Dependency firewall.** Needs FrankenSim (optimizers, racing, statistics, spectral), asupersync (daemon),
FrankenSQLite (qualification ledger), fastmcp_rust (full MCP). Uses if admitted: franken_numpy (the Gymnasium
adapter), frankentui, franken_threed, drone-reconstruction schemas. If missing: local implementations against the
same known answers; KILL-MCP; KILL-TUI; plain Three.js.

**8. Owner input wanted.** OPEN-CORTEX-1, the envelope for family-policy pretraining, which M4 needs on its first
day (proposal: one class, 200 designs, at most 60 GPU-hours of pretraining and 40 for specialists; default: no
pretraining, so M4 exits in its variant with no family policy). OPEN-PROG-3 (whether the Gymnasium-compatible
adapter is a supported binding; default: built and gated for the test and for conformance only). OPEN-AUGUR-4
(a quantifier-elimination or SMT engine as a production dependency; default: comparator only). OPEN-HAB-1 and
OPEN-HAB-4 (cited sources for dimensional norms and retrofit prices; default: generic ranges labelled `assumed`,
constraints marked unchecked). OPEN-SHIELD-2 (α = 10⁻³ and a budget of 10⁻³ per episode by default). OPEN-SAFE-4
(retention defaults as at M2).

**9. Stop or re-plan.** The partial gate decides, by the rule written before it runs. A measured ratio under 3 on a
mission class withdraws the speed claim for that class. Under 3 on every class, speed work is no longer funded, the
product is positioned on replay, qualification, certificates and the evidence ledger, and the family-policy spend
of M4 is decided again. Where the composed-incumbent arm is within a factor of 1.5, the layer ships over that
engine for the stratum (BET-14). Separately: if GATE-QUAL-01 fails, nothing is called qualified, and confirmation
falls back to fixed-n exact studies with a hard cap on submissions before any tournament is scheduled.

**10. No single failed bet blocks the exit.** BET-06 (baselines fail on many morphologies) → the sampling fallback
and training for the classes that fail. BET-21 (the blinded lane does not hold its error rate) → fixed-n exact
confirmation with a cap on submissions. BET-22, BET-23 (adversarial search or attribution underperforms) →
randomization with shrinking and the guard; single-factor re-runs. No neutral party named (OPEN-PROG-2) → a partial
gate labelled `self-adjudicated`.

**11. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-23 | Interiors: seven archetypes and hazard lists | `frob-scene` | M3 | GATE-HAB-02 |
| WP-24 | Arena: task families, scorecards, the design corpus and benchmark | `frob-arena` | M3 | GATE-ARENA-01, -02, -07, -09 |
| WP-25 | Atelier: compare, explain, promote, detect | `frob-opt`, `frob-learn` | M3 | GATE-ATEL-01, -04; GATE-FID-02; GATE-CORTEX-04 |
| WP-26 | Tether, full: slot dispatch, capabilities, the full MCP surface | `frob-cli`, `frob-verbs`, `surface/frob-mcp` | M3 | GATE-TETHER-01, -09 to -12 |
| WP-27 | Reflex II: baselines for every archetype, the sampling controller, skill artifacts [S], [F] | `frob-control` | M3, M4 | GATE-REFLEX-02, -04, -05, -06, -08 |
| WP-53 | Habitat II: clearance complexes, retrofit catalogue, world bank, film patches, rigid proxies [S], [F] | `frob-scene`, `frob-contact` | M3, M4 | GATE-HAB-04 to -06, -10, -11, -12, -14; GATE-TRAC-05, -07 |
| WP-54 | Blinded qualification lane | `frob-arena`, `frob-verbs` | M3 | GATE-QUAL-01, -02; GATE-SAFE-03; INT-FSIM-14 |
| WP-55 | Nemesis [S], [F] | `frob-nemesis` | M3, M4 | GATE-NEMESIS-01 to -08 |
| WP-56 | Autopsy | `frob-autopsy` | M3, M4 | GATE-AUTOPSY-03 to -06, -08 |
| WP-57 | Gymnasium-compatible adapter [F] | `bindings/frob-gymnasium`, `frob-learn` | M3 | GATE-CORTEX-16 |
| WP-58 | Safety, privacy and supply chain | `frob-verbs`, `frob-io`, `frob-chronicle` | M2, M3 | GATE-SAFE-02, -04, -05, -06, -07, -09, -10; GATE-SPARK-17 |
| WP-59 | Mission queries II: amendments, transport, separation, probes [F] | `frob-mission`, `frob-opt` | M3 | GATE-AUGUR-06, -08 to -13 |
| WP-60 | The rank-agreement pilot and the partial ten-fold gate | `frob-conformance` | M3 | GATE-CORTEX-12 (pilot); GATE-TENFOLD-03 to -06, -10, -11, -13 |

## Acceptance criteria

- [ ] Every part in the gate tables has a recorded value and a caught negative control.
- [ ] GATE-TENFOLD-03, -04, -05, -10, -11 are recorded before the sealed suite is opened; GATE-TENFOLD-06 and -13
      are recorded with a phase ledger per arm; the outcome rules are applied per class.
- [ ] The 28 kill gates of the compiled table and the 6 late ones each have an ADR with an outcome;
      KILL-AUGUR-REUSE, KILL-TENFOLD-CERT and KILL-TENFOLD-LEAK are registered as standing from here.
- [ ] NS-03, NS-04, NS-09, NS-15, NS-16 and NS-17 run from a clean checkout through `frob --json` and `frob-mcp`.
- [ ] GATE-CORTEX-12 is recorded as a pilot and GATE-AUGUR-09 as a pilot; neither is printed as the whole gate.
- [ ] The exit report for M3 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by member beads. Epic check: due counts for M3 are 6 admission, 77 convergence, 28 kill decisions, plus 21
late parts and 6 late kill decisions, plus seven ten-fold program parts.

### End-to-end script and logging

`scripts/exit/review.sh M3`: the six demonstrations as scripted sessions through CLI and MCP; the partial-gate
record regenerated by the scorer from ledgers and receipts alone; logs with evidence blocks, e-values and
stopping times, digests of `.fmcw` replays, flip rates, α_i spent per submission, and the gates touched.

### Known answers and negative controls

α_i = α / [i·(i + 1)] sums to α; false promotion in at most 123 of 2,000 no-signal campaigns; 10.081, 9.677,
1.8215, 0.78568; 32, 46, 49, 182. Controls: a planted tier-sensitive policy; planted illegal edits and impossible
traps; a planted chaotic failure that must name no cause; a canary value from a sealed world; the percentile
bootstrap certifying ten-fold in 78.6% ± 3 points of rare-tail campaigns.

## Considerations

- The partial gate exceeds the default compute envelope (HYPOTHESIS: about 70 device-hours per design cell; worst
  case 288). Without approval it is recorded `not run` and M3 does not exit.
- 28 + 6 kill decisions in one exit is the heaviest decision load but one; each needs its ADR, not a summary.
- A class withdrawn at M3 stays withdrawn until M7 measures it on fresh sealed families.

## References

Plan §43.6, §39.12, §43.1, §43.2, §40.3 to §40.6, §43.12. Bets BET-05, BET-06, BET-13, BET-14, BET-21, BET-22,
BET-23. Scenarios NS-03, NS-04, NS-09, NS-15, NS-16, NS-17. Work packages WP-23 to WP-27, WP-53 to WP-60.
