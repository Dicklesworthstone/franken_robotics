---
key: "EPIC-M4"
title: "Milestone M4, amortized control, co-design, mission queries at scale: decide many designs, train few"
type: "epic"
priority: 2
milestone: "M4"
labels: ["m4", "delivery", "solid", "frontier", "gate", "review-notes-open"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["EPIC-M5", "EPIC-M7", "EPIC-PROGRAM", "N40-02", "N40-04", "N40-05", "N40-06", "N40-25", "N40-26", "N40-34"]
gates: ["GATE-REFLEX-04", "GATE-CORTEX-12", "GATE-ATEL-07", "GATE-CORTEX-11", "GATE-CORTEX-13", "GATE-CORTEX-03", "GATE-ATEL-03", "GATE-AUGUR-15", "GATE-SHIELD-01", "GATE-HAB-11", "GATE-GAUNT-01", "KILL-FAMILY", "KILL-INHERIT", "KILL-RANK", "KILL-LADDER", "KILL-EQUIV", "KILL-DISTILL", "KILL-GAIT", "KILL-MPC", "KILL-SHIELD", "KILL-ENERGY", "KILL-RETROFIT"]
plan_refs: ["§43.7", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§43.12"]
---
## Background

**Purpose.** M4 builds the two levers that carry the thesis, *deciding many designs per turn* and *training few of
them*, and extends mission queries from CPU fixtures to families on the GPU.

**Why here.** A design loop is slow mainly because every candidate design waits for its own controller to be
trained. Two mechanisms attack that: a **family policy** (one policy conditioned on the design, walking unseen
designs of its morphology class with no training) with **inheritance** along design edits, and a **ladder** of
cheap evaluation rungs that may select or discard designs only where their measured **rank agreement** with
trained specialists permits. Neither is funded before the partial ten-fold gate of M3 has shown what share of the
loop training actually holds, and each has a pre-funding pilot and a kill gate. The exit line is that rank
agreement is *measured* and the family-policy bets are *decided*, not that they succeeded: M4 has a complete
variant with no family policy. Planning estimate: 8 weeks (DESIGN). Critical path: yes; M7 needs M4.

**Demonstrates.** NS-05 *Symmetry for free*: the studio reports a hexapod's symmetry group and the subgroup its
sensors, payload and task preserve, lists the gaits that subgroup permits, and trains a policy equivariant under it
that reaches the target return in at most 0.7 times the steps of an unconstrained one. NS-06 *Thirty iterations in
an hour, three hundred designs decided*: thirty check-simulate-explain-edit cycles on a wheel-leg hybrid in one
hour on `trj`, each dispatching a family of at least ten variants; `frob design bisect` finds the edit that broke
stair climbing. NS-08 *Tracks on rubble*, with the sinkage model labelled Frontier. NS-18 *A twelve-dollar ramp
against a chassis redesign*: the clearance complex names a 19 mm threshold as the bottleneck, the retrofit
catalogue offers a 1:12 ramp at USD 12 against a larger drive wheel at USD 140 in parts, both in one budget.
NS-19 *A family policy walks an unseen design*: return 0.78 of the specialist reference with no training;
inheritance reaches the target in 12% of the steps of an early-stopped from-scratch control; a six-legged variant
is flagged out-of-distribution. NS-23 *An energy-budget safety claim*: an `.fenergy` budget for a 12 cm step-down
with every term from the energy ledger and no contact law in the claim; claim product Tested. NS-13 at scale.

## Scope

In: family policies, inheritance, distillation; the rank-agreement record and the ladder with its audit lane;
equivariant policies; gait orbits with continuation; optimization, racing, the quality-diversity archive, bisect,
registered predictions, the co-design cascade, decision cores DC-01 to DC-11; mission queries over families on the
GPU with region certificates, monotone transport and product-system dominance; synthesis lanes; tyre laws, tracks,
hybrids, moving surfaces, soil cards; the priced retrofit catalogue and its search; the energy-budget tier and
runtime shields; experiment suggestion by value of information; the agent-surface A/B; `solve.run`; the fleet
scheduler.

Explicitly not in M4: certificate coverage of derived kernels, Lean capsules, Ground, Foundry (M5); frontier
physics epics (M6); the tournament (M7).

What M7 needs from M4: the released surface with whatever levers survived; the measured trained fraction; the
strata re-stratified if KILL-RANK and KILL-FAMILY both fired. M7 needs M4 and neither M5 nor M6.

## Technical approach

**1. The artifact.**

- Family policies per morphology class (`frob train family`, `frob policy apply`), inheritance along design edits
  (`frob train inherit`) and distillation (`frob train distill`).
- The rank-agreement record per class and task family (`frob eval rank-agreement`) and the ladder that allocates
  by it (`frob opt cascade`), with the audit lane and the planted winner.
- Equivariant policies under the observation-preserving subgroup; gait orbits with continuation
  (`frob gait orbit`, `frob gait continue`).
- Optimization, racing and the quality-diversity archive (`frob opt start`, `frob opt archive`);
  `frob design bisect`; registered predictions; the co-design cascade and the decision cores DC-01 to DC-11.
- Mission queries over design families on the GPU, with region certificates on small sub-boxes, monotone
  transport and product-system dominance; `frob result revalidate`.
- Synthesis: `frob mission synthesize --strategy codesign`, `refine` and `linkage`.
- Tyre laws, tracks, wheel-leg hybrids, moving surfaces and soil cards; the priced retrofit catalogue and its
  search (`frob scene retrofit`).
- The energy-budget tier and runtime shields (`frob shield budget`, `frob shield fit`, `frob shield check`).
- `frob experiment suggest` ranked by value of information; the agent-surface A/B; `frob solve run`; the fleet
  scheduler (`frob fleet plan`).

**2. Gates with a part due at this exit (compiled register tables: no admission, 49 convergence).** The register
places GATE-HAB-11, GATE-SHIELD-01 and GATE-AUGUR-11 here, with their mechanisms. GATE-REFLEX-01 closes here for
the tracked and hybrid archetypes.

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| GATE-ATEL-02 | Known answer | M1 (the analyses that exist); M4 (the rest) | Each analysis A-01 to A-14 passes its known-answer tests; `design check` finishes in under 2 s at the 95th percentile on templates. | §28.23 |
| GATE-ATEL-03 | Design comparison | M4 | BET-10: the pre-registered A/B of agent surfaces on the design benchmark of §24, with an anytime-valid verdict and an effect size: restricted mean time to an accepted answer lower by a factor of at least 1.5 on at least ten briefs. The incumbent arm belongs to §39. | §28.23 |
| GATE-ATEL-04 | Known answer | M3 (comparison); M4 (bisect) | Comparison type-I error at or below α under optional stopping over 10⁴ null comparisons; `design bisect` finds the planted edit in at least 95 of 100 trials. | §28.23 |
| GATE-ATEL-05 | Known answer | M1 (A-20 to A-23); M4 (the rest) | Each analysis A-15 to A-26 passes its known answers (§28.22 and the owning chapters) and its negative control fails; the analyses marked ✓ keep `design check` under 2 s. | §28.23 |
| GATE-ATEL-07 | Design comparison | M3 (pilot); M4 | The rank-agreement experiment per morphology class: Kendall τ on the top band at least 0.7 for `selective` and at least 0.5 for `discard-only`, and ρ_ep at least 0.7, all at the lower bootstrap bound, on 40 designs. A class below 0.5 is `uninformative`. | §28.23 |
| GATE-ATEL-08 | Design comparison | M3 (pilot); M4 | Layer 2 against the fixed table: held-out coverage at least 1 − α − 0.03 per stratum; audited top-k recall no lower than halving's; a paired 95% lower bound above 1 on GPU-seconds to the same accepted decision; a planted over-scored design caught at the next rung; synthetic drift trips the alarm. | §28.23 |
| GATE-ATEL-09 | Known answer | M4 | Atelier does not trust a rung whose screening audit (§26.10, GATE-NEMESIS-07) is missing or failed: in 100 of 100 planted campaigns a stage with no audit record, a missed planted winner or an audited miss rate above its tolerance has its rung set to `uninformative` in the affected strata and its results labelled `screen-biased`; a planted unsound prune is caught in 100 of 100; no audit episode is ever cancelled. | §28.23 |
| GATE-ATEL-10 | Known answer | M4 | A-19: every planted cone-excess and energy-pumping exploit is typed `model-conditional`; honest winners are typed `model-robust`; the stage costs at most 10% of campaign compute. | §28.23 |
| GATE-ATEL-11 | Known answer | M1 (dependency grades on mission cells); M3 (dependency grades); M4 | Revalidation on the frozen 30-amendment campaign: every decision equals full recomputation; zero unsafe reuse; the stale-key mutant is caught; unnecessary invalidations are at most 20% of unaffected verdicts, counted and reported. | §28.23 |
| GATE-ATEL-13 | Known answer | M3 (DC-01, DC-09, DC-10); M4 (the rest) | Each decision core: its table regenerates bit-identically; its null calibration holds (PO-ATEL-01 to -05, -10); its fallback is demonstrated by forcing each trigger; its shadow run records both actions. | §28.23 |
| GATE-ATEL-14 | Known answer | M4 | The nine interference tests of §28.19.3 pass on a replayed campaign. | §28.23 |
| GATE-ATEL-15 | Design comparison | M4 | The offline replay of §28.9: the knowledge-gradient chooser at least 1.5 times better than the agent chooser in evaluations to a true top-3; the value is exactly zero on a fixture where no outcome can change the choice. | §28.23 |
| GATE-ATEL-16 | Known answer | M4 | The cascade end to end on a seeded population: stage controls all run; the report prints every mode, every fallback and the phase ledger; merge of two agents' archives in both orders gives identical bytes. | §28.23 |
| GATE-AUGUR-09 | Known answer | M3 (pilot); M4 | Region certificates on the 12-parameter box: the per-constraint arm decides at least 50% of each constraint's sub-box volume within 10⁴ boxes; no single monotone bound gives more than 95% of decided volume; zero certified boxes contradicted by 10⁶ point evaluations. | §25.27 |
| GATE-AUGUR-11 | Design comparison | M4 | Probe design: 675 of 675 in class; zero unconditional verdicts issued on the withheld law; the direct measurement is offered in every proposal list and is right in 900 of 900. | §25.27 |
| GATE-AUGUR-14 | Performance | M3, M4 | Ceiling before speed: every registered speed comparison in this subsystem records measured f, r and h on the strong baseline and is refused when 1/(1 − f + f/r + h) is below the claimed ratio. | §25.27 |
| GATE-AUGUR-15 | Parity | M4 | Family execution: inner, excluded and unresolved labels for the 65,536-row screen and for one 1,024-design family batch are identical on `batch-cpu`, CUDA and Metal under the exact-integer or guarded class, with zero final errors against exact CPU arithmetic. | §25.27 |
| GATE-AUGUR-16 | Design comparison | M4 | Product dominance: the z fixture exact; on the contact-transition pilot zero mislabelled joint outcomes against per-case referee replay over 10⁴ draws. | §25.27 |
| GATE-AUGUR-17 | Known answer | M4 | Constructive lanes: the co-design fixed point equals the GB-03 closed form and rejects the additive-peak rule on the bus fixture; the three-pose dyad and both circuit fixtures exact; zero certified linkages change circuit on the referee across 240 instances. | §25.27 |
| GATE-AUGUR-18 | Design comparison | M4 | Refinement: of 20 trajectory-optimized candidates per mission, at least 10 pass the referee route and beat the nominal design on held-out terrain, each solved within 10 minutes on 8 threads. | §25.27 |
| GATE-AUGUR-19 | Known answer | M4 | Labels: the three-variant, three-reading and station-record fixtures exact; with inner and outer sets the returned label interval contains the exact count on 20 generated instances. | §25.27 |
| GATE-AUTOPSY-08 | Design comparison | M4 | On the planted-defect designs of the corpus (§24.9), agents given Autopsy reach an accepted repair in at most one third of the serial turns of agents given logs and `explain` alone, pre-registered, with an anytime-valid verdict. | §27.17 |
| GATE-CORTEX-03 | Design comparison | M4 | BET-07 as the three-arm experiment of §23.13, ten paired seeds: the equivariant arm reaches the return in at most 0.7 times the steps of the unaugmented arm with no loss of final return; the one-sided patch is passed; the comparison with the augmented arm is reported, whichever wins. | §23.23 |
| GATE-CORTEX-11 | Design comparison | M4 | Family policy in one class on held-out parametric designs: median return with no adaptation at least 70% of a specialist's; fine-tuning reaches specialist return in at most 20% of the steps of a from-scratch run stopped at 95% of its final return, on the task family's terrain set including stairs, slopes and wet patches; out-of-class designs declined; re-described robots keep at least 80% of return. | §23.23 |
| GATE-CORTEX-12 | Design comparison | M3 as a pilot on incumbent tools; M4 on ours | The rank-agreement experiment of §23.11 is run and its consequence applied: τ, top-10 recall and intervals are recorded per class and task family, and a rung without a record is refused. The gate passes when the record exists and the rule is enforced, whatever τ is. | §23.23 |
| GATE-CORTEX-13 | Design comparison | M4 | Inheritance on 12 parent-child edit pairs (4 blind): median adaptation samples for parameter edits at most 20% of a from-scratch run stopped at 95% of its final return, and fewer than a raw-checkpoint warm start needs; the inherited child more than 5% worse in at most 20% of edits; a chain of 10 edits shows no decline under scheduled resets (audit twin within 5%). Topology edits reported, not required. | §23.23 |
| GATE-CORTEX-14 | Design comparison | M4 | The shuffled-label control fails; a student of a baseline or specialist teacher reaches at least 90% of the teacher's requirement pass rate; the figure for a sampling-controller teacher is recorded with its cost. | §23.23 |
| GATE-GAIT-01 | Known answer | M4 | Rimless-wheel fixed point and multiplier within 10⁻⁶; compass-gait multipliers within 10⁻⁶ of an independent high-precision computation; the planted transverse instability is found and the restricted computation is marked incomplete; PO-GAIT-01. | §22.18 |
| GATE-GAIT-02 | Known answer | M4 | 100-case suite: in at least 72 of 80 regular cases the gradient's cosine against central differences is at least 0.95; all 20 grazing and fold controls labeled or refused; no case refused for simultaneity alone; the bar fixture returns rest, v/4 and the sign of the coupling term. | §22.18 |
| GATE-GAIT-03 | Design comparison | M4 | Continuation succeeds on at least 50% of 10% design perturbations; median solve at most 10 minutes of CPU; Kendall τ of at least 0.5 between orbit cost of transport and specialists on the calibration set; the measured solve cost is recorded beside the published range. | §22.18 |
| GATE-HAB-11 | Design comparison | M4 | Retrofit [F]: three planted bottlenecks (a 20 mm threshold, a closer above push capability, an unmarked glass door) are repaired by the expected kits with the policy identity unchanged; the wheel-and-ramp closed form is reproduced by both arms. | §19.19 |
| GATE-LEMMA-04 | Proof | M1, repeated at M4 | The first mission slice of §25 ships its inner cells and its exclusions as bundles; a re-check on a second machine accepts every one; the seeded-defect suite yields zero accepted false certificates. One accepted false certificate fails the gate. | §12.6 |
| GATE-MARROW-04 | Known answer | M4 | PO-MARROW-05 holds exactly for every group of GATE-MARROW-02 on 100 random pairs of channel spaces each; all eight mutations of §13.8 are caught by at least three fixtures each. | §13.10 |
| GATE-NEMESIS-07 | Known answer | M4 | Screening audit: a planted winner removed by a biased screen is reported in 100 of 100 runs; the stratified bound covers the planted false-discard rate in at least 95% of 1,000 synthetic campaigns. | §26.15 |
| GATE-PHYLUM-06 | Design comparison | M4 | Paired gradient: the ballistic row within 1%; binary metrics refused; on the pilot task a variance ratio of at least 2 at the admitted horizon and sign agreement of at least 70% over 20 replicates. | §17.19 |
| GATE-PHYLUM-08 | Design comparison | M4 | Maps per dispatch: on the benchmark of §39, campaigns reach the acceptance target in at most half the serial rounds of one-design-per-turn search at equal compute, with compute reported beside rounds. | §17.19 |
| GATE-REFLEX-01 | Known answer | M1 (referee); M2 (`batch-cpu`); M3 (all but tracked and hybrid); M4 | Each archetype stands for 10 s and traverses 5 m of flat ground under its baseline controller: at M1 the first archetypes, on `referee` only, restricted to those whose contact law exists at M1 and whose contact count fits the referee's 21-contact cap (§16.6); at M2 the same set on `batch-cpu`; at M3 all eighteen on `batch-cpu`, except tracked and hybrid archetypes, which follow at M4 with their contact laws. | §22.18 |
| GATE-REFLEX-04 | Design comparison | M3 (legged, wheeled); M4 (tracked, hybrid) | Rank agreement of the baseline rung with trained specialists on 30 to 40 designs per morphology class and task family, thresholds fixed before data: a Kendall τ lower 95% bound of at least 0.7 lets the rung select; at least 0.5 lets it discard the bottom half; below 0.5 it removes outright failures only (`baseline-only: unranked`). | §22.18 |
| GATE-REFLEX-06 | Design comparison | M4 | Sampling controller on the pilot design set: success at least 60% within three episodes; median at most 1 GPU-minute per 20 s quadruped episode and 6 per humanoid episode (the thresholds of BET-27); Kendall τ against specialists at least 0.5; FROB-REF-817 fires on a planted acceptance request that has no frozen planner record; the digest does not change with the fork count. | §22.18 |
| GATE-REFLEX-07 | Known answer | M4 | Planted asymmetries (side slope, off-centre payload, front-only sensor, one-sided patch) remove exactly the expected generators; the two-latch fixture passes in law and fails tied; the event identity holds on regular landings and is flagged on grazing ones. | §22.18 |
| GATE-REFLEX-08 | Known answer | M4 | Skill exact layer: both hand fixtures reproduce; 4 of 4 infeasible bodies are refused with certificates that check exactly; feasible witnesses replay; the {stable, fall} refinement is refused. | §22.18 |
| GATE-SHIELD-01 | Known answer | M4 | Energy tier: the worked card reproduces; no exceedance over 10⁴ randomized contact sequences with μ in [0, 5], restitution in [0, 1] and every contact law; a budget with a planted unlisted spring is refused; a payload-only budget shows an exceedance. | §22.18 |
| GATE-SHIELD-02 | Known answer | M4 | PO-SHIELD-02 with zero box violations; the worked example gives (1, 0.5); a planted non-convergent multi-row instance invokes the backup; PO-SHIELD-03. | §22.18 |
| GATE-SHIELD-03 | Design comparison | M4 | 5,000 in-envelope and 5,000 shifted-world episodes with independent breach labels: in-envelope violations within the stated bound; shifted results reported separately; at least 85% of unshielded return; at most 5% added step cost. | §22.18 |
| GATE-SHIELD-04 | Known answer | M4 | Measured margin: empirical coverage at least 1 − α − 0.01 on 10⁴ held-out episodes of the calibrated family; a planted change of policy trips FROB-REF-457; a planted drift trips the alarm. | §22.18 |
| GATE-SPARK-13 | Performance | M4 | [F] On the standard campaign, the paired lower bound of the speedup of two machines over the better single machine is at least 1.3; at most 1 GB crosses per campaign-hour; no forbidden transfer class crosses. | §21.24 |
| GATE-SPARK-14 | Performance | M4 | [F] A specialized kernel is bit-identical to its generic kernel and at least 1.2 times as fast on the whole step, with compile time at most 60 s. | §21.24 |
| GATE-TETHER-13 | Design comparison | M4 | A fresh agent with only `robot-docs` completes 10 scripted tasks of the mission session with at least 9 correct, no budget violation and no instruction taken from a poisoned import. Second arm: with `solve.run`, serial turns per accepted design fall by the pre-registered factor or KILL-SOLVE fires. | §32.21 |
| GATE-TRAC-08 | Known answer | M4 | Soil, track and tyre: law evaluations match the closed forms within 10⁻⁶ in `f64`; simulated plate sinkage and drawbar pull within 2% of them; rut leak zero over 10³ resets. | §18.17 |

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-FID-09 | row 2 | Known answer. Every row is reproduced at its tolerance (times to 0.1 s; sinkage to 10⁻⁵ m in double precision; counts exact; admissions exact) and every negative control fails. Rows 1, 3 and 5 at M1; row 2 at M4; row 4 with the continuation lane. | §38.17 |
| GATE-AUGUR-33 | the label rows | Known answer. Enumeration of the 128 histories returns 55/64 for the pairwise law, 3807/4096 for the independent law and 3/4 for the shared fault, exactly; a redundancy claim on a card marked `pairwise` is refused. The three label rows return 2, 3 and 5 bits with codes whose received-word sets are disjoint; the four-bit request under `flips(1)` returns an exclusion with the counting argument. On 20 generated instances with L ≤ 8 and e, t ≤ 2 the least length equals brute force. The error-law rows at M3, the label rows at M4. | §25.37 |
| GATE-SKEL-10 | whole | Design comparison ¦ On 200 assemblies across the wheeled, tracked, legged and hybrid families, on carrying, ramp and threshold tasks inside the validity regime: discarding the bottom 90% by the composed screen keeps at least 9 of the true top 10 found by batched evaluation; Spearman ρ at least 0.75 on energy per metre and on payload capacity; every audited `outer` exclusion fails its quasi-static requirement on the referee ¦ M4 | §14.16 |
| GATE-TRAC-11 | whole; the gate of KILL-RESPONSE | Known answer ¦ The three fixtures reproduce and the λ = 3 mutation is caught. On 10⁴ battery contact states with 64 random columns each, solved in `f32` and compared with a full `f64` re-solve of every column: zero accepted columns whose true error exceeds the attached bound, and zero accepted columns whose re-solve has a different active set or stick label. The rerouted share is reported per robot class. A batch requested against a changed anchor is refused. ¦ M4 | §18.19 |
| GATE-SENS-17 | whole | Design comparison ¦ On 20 designs carrying payloads across stairs, a doorway and wet tiles under estimated state: mounts recommended by the optimizer cut the gap between privileged-state and estimated-state success by at least half; lead and blind interval correlate with the estimated-state failure rate at 0.65 or more ¦ M4 | §20.24 |
| GATE-SHIELD-06 | whole | GATE-SHIELD-06 (Known answer, M4). On the referee and on the `strict` profile, over 1,000 paired episodes with v₀ drawn from [1, 3] m/s and friction pairs from [0.3, 0.8]: the two transcripts are bit-equal before the first braking tick in every pair. Arm 1 violates in the low-friction world in every episode whose stopping distance exceeds its trigger range, with the overshoot within 1 mm of the closed form. Arm 2 has no violation in 10⁴ episodes. The mutant without the inter-sample term produces at least one violation, none larger than v₀·Δt. `shield.fit` refuses Arm 1's monitored-assumption listing and prints both admitting changes. | §22.21 |
| GATE-SHIELD-07 | whole | Known answer. The four rows are exact. `shield.fit` refuses 300 N·s/m at 10 ms on an inertia of 1 kg and admits 100 N·s/m. Over 10⁴ random triples (M̄, K_d, h) with 2·M̄ − h·K_d ⪰ 0 the ledger's stored energy at ticks never increases, and each of 10³ planted violations shows an increase. The continuous-time identity remains a theorem card. M4. | §22.25 |
| GATE-GAIT-04 | whole | Known answer. On the planar loop the section and bordered forms agree with the closed forms to 10⁻⁹ on the multiplier, on dr*/dμ and on dT*/dμ; a stroboscopic solve of the same loop is refused, and is not reported as a fold. The rimless wheel and the compass gait of GATE-GAIT-01 pass through the autonomous path with the trivial multiplier removed and listed. M4. | §22.23 |
| GATE-GAIT-05 | whole | Known answer. The counterexample returns the eigenvalue 3/4 + 1/√2 to 10⁻¹², the per-stride bound 1.2071 and the word (A, B); a pair of commuting contractions returns `stable-under-switching` with a common matrix that the checker verifies in exact rationals; a mutant that tests each mode alone labels the counterexample stable and fails. M4. | §22.24 |
| GATE-AUGUR-25 | the finite part | Known answer. Zero violated bounds over the 200 finite fixtures; the two-outcome reversal is refused; on the tray fixture the 95% upper bound of the measured budget is below the claim's slack for at least one skill, or the kill gate fires. Finite part M4; tray M8. | §25.29 |
| GATE-AUGUR-28 | whole | Known answer; Design comparison ¦ On an exhaustively solvable planar ramp-and-carrier family (states and controls discretized so the optimum of each of at least 500 design and scenario pairs is known by dynamic programming): every Ū at or above the optimum, every L at or below it, zero false dominance stops; the run-up candidate is not stopped and the relaxation without approach energy is rejected. On a held-out brief limited by energy or torque: controller effort per accepted decision at least 3 times lower than static filters followed by the ladder, at matched decisions ¦ M4 | §25.32 |
| GATE-AUGUR-29 | whole | Known answer; Design comparison ¦ The cart: boundary 35/654 m in exact rationals, the dual at 4 m/s² accepted by `frob-verify`. The regressor identity Y·π equals the derived inverse dynamics within 10⁻¹² relative on 10⁴ random states per reference robot. A request for jointly optimal energy and ratio is routed to enumeration. On 30 designs and two missions, candidates initialized by the lane beat their class templates by at least 15% in electrical cost of transport in closed-loop evaluation at a 95% paired lower bound, or start within 10% of the optimum that forward co-design reaches; the share of designs infeasible on every library motion that still pass the mission is reported ¦ M4 | §25.33 |
| GATE-AUGUR-30 | whole | Known answer; Design comparison ¦ The 20-parameter case returns five slabs with shares 0.58, 0.40 and 0.02, and `frob-verify` accepts every certificate. The negative control returns `unresolved` for the whole box with one enclosure evaluation and no bisection. On the 12-parameter static quadruped family of §25.6, against per-constraint decomposition restricted to constraints that read at most six parameters and at the same box budget: at least 10 percentage points more decided volume, or at least 10% more candidate evaluations avoided, with no certificate rejected ¦ M4 | §25.34 |
| GATE-AUGUR-34 | whole | Known answer. The margin table reproduces to 10⁻³ N, and a refutation built from the independent extremes under law D is refused. With four laws admitted `mission.distinguish` returns a probe at t = 1/2 s with r = 3 and a gap of at least 0.10, decides 900 of 900 cases, and returns no probe at any dwell of 1/4 s or less. M4. | §25.38 |
| GATE-AUGUR-35 | whole | Known answer. On the scalar pack the lane returns a bracket whose upper end passes F(U) ≤ U and lies within 1 g of 100 kg; the increment-stopping mutant returns 99.5 kg and is refused. On the three-build catalog all three builds are returned with dispositions; a weights-only mutant omits (4, 4) and fails; the budget query returns (4, 4). On 20 generated catalogs of at most 200 options the returned set equals brute-force dominance filtering. M4. | §25.39 |
| GATE-ATEL-18 | whole | Design comparison ¦ On 30 designs in three classes with an oracle (a specialist per design over five seeds, and the sampling controller at 8,192 samples): top-3 regret at equal total compute at most half that of ranking at the base budget; J∞ closer to the oracle than J(b_max) for at least half the designs ¦ M4 | §28.25 |

**4. Recurring gates.** GATE-TENFOLD-09, GATE-GROUND-01, GATE-SAFE-01, GATE-GAUNT-01, -02, -03, -05, -10;
GATE-GAUNT-08 at each funding decision (no Frontier part of WP-28 to WP-30, WP-61, WP-62 or WP-64 to WP-67 is funded
without a pilot record that has a falsifier, a computed ceiling, a fallback and a result).

**5. Kill gates decided here.** In brief: KILL-FAMILY → no family policy; specialists with inheritance from the
nearest parent, and the baseline screens. KILL-INHERIT and KILL-DISTILL → children train from the class prior or
from scratch; distillation only from a privileged teacher. KILL-RANK and KILL-LADDER → R1 is `uninformative` and
every candidate that survives static analysis is scored on the higher rung; the fixed halving table allocates
among trained candidates. KILL-EQUIV → symmetry is used for augmentation and metamorphic testing only. KILL-GAIT,
KILL-MPC, KILL-RESIDUAL → orbits as diagnostics; the sampling controller as a debugging aid; no residual rung.
KILL-SHIELD and KILL-ENERGY → barrier rows become penalties and alarms, and nothing is called a shield; an energy
claim that shows an exceedance is withdrawn and its monitor stays as an alarm. KILL-RETROFIT → the catalogue stays
as Nemesis's edit vocabulary and the search verb is withdrawn.

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-MARROW-BLOCKS | GATE-MARROW-04 fails after one repair cycle, or exact field arithmetic for n ≤ 12 exceeds its bit budget | Equivariant layers by Reynolds projection in `f64` with an idempotence test, classed `Tested`; the free-parameter count is read from the projection's rank | M4 | §13.10 |
| KILL-FAMILYLANE | Measured whole-step gain below 1.15 on the mixed workload, or overhead h ≥ f·(1 − 1/r) | The general masked lane only, with shared structure tables kept | M4 | §17.6 |
| KILL-FDGRAD | The pilot's falsifier fires (variance ratio below 2 at every horizon of 50 steps or more, or sign agreement below 70%), or no end point is confirmed by a specialist | Rank-based search (CMA-ES, racing); the stencil is kept as a sensitivity report for A-16 | M4; the pilot runs before funding | §17.13 (§28.8) |
| KILL-RESPONSE | Mode guards reroute more than half the columns on battery states | Full solves per query | M4 | §18.5 |
| KILL-SOIL | GATE-TRAC-08 fails after one repair cycle, or no soil card can be sourced with provenance | A rigid floor with rolling resistance and a drag card labeled `assumed`; soft-ground results labeled exploratory | M4 | §18.17 |
| KILL-RETROFIT | On the home battery kits flip fewer than 30% of the failures attributed to a building feature, or more than 10% of kit verdicts reverse on the referee | The catalog remains the edit vocabulary of Nemesis; the search verb is withdrawn | M4 | §19.12 |
| KILL-FLEET | The pilot shows g < 1/3, or after one repair cycle the paired lower bound of campaign speedup at GATE-SPARK-13 is below 1.3 against the better single machine | The machines run standalone; jobs are assigned by the role table; the seeds-only rule stays | M4 | §21.17 |
| KILL-SPECIALIZE | In the pilot the specialized kernel is below 1.2 times the data-driven one on the whole step, or takes more than 60 s to compile; or the M2 profile shows a specializable share under one third | Data-driven kernels only; derivation stays for correctness, certificates and the derivative kernels of CPU solvers | M4 | §21.18 (§17.9) |
| KILL-ENERGY | An exceedance under the premises survives one repair cycle at GATE-SHIELD-01 | The claim is withdrawn; the monitor remains as an alarm of class `Estimated` | M4 | §22.18 |
| KILL-GAIT | GATE-GAIT-03 fails | Orbits serve stability diagnostics and warm starts only; the atlas keeps supplying clocks and reward references; no design is ranked by an orbit | M4 | §22.18 |
| KILL-MPC | GATE-REFLEX-06 fails after one repair cycle | The sampling controller is kept as a single-world CPU debugging aid; demonstrations come from the baseline and from specialists | M4 | §22.18 |
| KILL-RESIDUAL | The pilot's falsifier holds, or at M4 the residual rung needs more than 50% of a specialist's steps on the corpus | The residual rung is removed; the baseline remains a behaviour-cloning warm start | M4 | §22.18 |
| KILL-SHIELD | GATE-SHIELD-03 fails after one repair cycle | The energy tier stays; barrier rows become reward penalties and alarms; nothing is called a shield | M4 | §22.18 |
| KILL-ALTALGO | For any one of the other algorithms of §23.22, its pilot's falsifier holds | That algorithm is not built; PPO under the rules of §23.6 remains the trainer | M4 at the earliest, per algorithm, before funding | §23.23 |
| KILL-DISTILL | Students stay below 70% of their teachers' requirement pass rate after one repair cycle | Distillation is limited to the privileged-teacher case; warm starts come from behaviour cloning only | M4 | §23.23 |
| KILL-EQUIV | GATE-CORTEX-03 fails (BET-07) | Symmetry is used for augmentation and metamorphic testing only | M4 | §23.23 |
| KILL-FAMILY | At GATE-CORTEX-11, return with no adaptation below 70% of specialists on more than half of the held-out set and fine-tuning above 20% of specialist steps; or the pilot's falsifier holds | No family policy: specialists with inheritance from the nearest parent, and the baseline screens | M4 | §23.23 |
| KILL-INHERIT | Median parameter-edit cost above 60% of from-scratch steps, or the inherited arm more than 5% worse in over 20% of edits | Every child trains from the class prior or from scratch; the mapping table is kept for loading policies | M4 | §23.23 |
| KILL-AUGUR-CUTS | One false cut on the frozen catalog, or less than 70% of evaluation cost saved; or one quotient merge with different replayed traces | Cuts off: exhaustive evaluation with the static screen as an ordering heuristic. Quotient off: memoize on full model identity only | M4 | §25.27 |
| KILL-AUGUR-DOMINANCE | One mislabelled joint outcome, or a conventional reachability tool closes the same cells at similar cost | Paired comparison by replay under the studies of §24; the product lowering is kept only as the rule for space-indexed shared inputs | M4 | §25.27 |
| KILL-AUGUR-LABELS | Deriving command sets costs more than labelling every part, or an inner set contains a failing command | The query is kept for supplied sets through a conventional solver; extraction from contact is parked | M4 | §25.27 |
| KILL-SYNTH-CODESIGN | The lane mis-ranks more than 20% of top designs against the referee, or prunes one independently feasible design | Restrict to the closure analysis A-10 for actuator, battery and compute selection | M4 | §25.27 |
| KILL-SYNTH-LINKAGE | A certified linkage binds or changes circuit on the referee, or more than 25% of template instances exceed the caps | Multi-start numerical optimization of linkage dimensions with the same circuit and clearance check, classed `Tested` | M4 | §25.27 |
| KILL-SYNTH-TRAJOPT | GATE-AUGUR-18 fails, or solves exceed the budget at 12 degrees of freedom | Continuous optimization of §28 with the baseline controller in the inner loop | M4 | §25.27 |
| KILL-ATLAS | The A-26 pilot of §28.4 fails | A-26 is a descriptive report with no validity label | M4 | §28.23 |
| KILL-LADDER | GATE-ATEL-08 fails after one repair cycle | The fixed halving table is the allocation rule; the audit lane is kept as measurement | M4 | §28.23 |
| KILL-RANK | GATE-ATEL-07 fails for every morphology class tested | R1 is `uninformative` everywhere and every candidate that survives static analysis is scored on the higher rung; the ladder's claim shrinks to stopping and elimination among trained candidates; §39 records the lost factor | M4 | §28.23 (§16.17) |
| KILL-REUSE | Any unsafe reuse in GATE-ATEL-11, or grade 3 saves less than a fifth of the recheck cost grade 2 leaves | Grades 1 and 2 only | M4 | §28.23 |
| KILL-VOI | GATE-ATEL-15 fails | Value of information is a stop rule only; `experiment.suggest` lists proposals unranked by value | M4 | §28.23 |
| KILL-SOLVE | The ceiling of §32.6.2 is under 1.4-fold, or agents with `solve.run` do no better than agents driving the verbs | The prompts | M4 | §32.21 |

| Kill gate | Trigger and fallback (from the defining subsection) | Decided | Gate beside it | Defined in |
|---|---|---|---|---|
| KILL-SKEL-CELL | Trigger: Top-10 recall below 90% or ρ below 0.75 at GATE-SKEL-10: no automatic discard from the screen. One unsound `outer` exclusion: the exclusion rule is suspended under the prune-soundness audit of §28.12. Fallback: Tables remain as a sizing report for agents; `design.check` runs its whole-robot analyses as before. | M4 | GATE-SKEL-10 | §14.16 |
| KILL-VISIBILITY | Trigger: GATE-SENS-17 fails on both counts, or the pilot attributes under a fifth of estimated-state failures to visibility or geometry. Fallback: `sense.visibility` stays as a report and a stage-7 check; mounts are not optimizer variables. | M4 | GATE-SENS-17 | §20.24 |
| KILL-AUGUR-BRACKET | Trigger: One false stop on the solvable family or in an audited campaign (the audit of §28.12 samples dominated designs and trains them); or less than 3 times at matched decisions; or the label changes the agent's next action in under a tenth of comparisons. Fallback: Brackets are printed as advisory labels, no stop is automatic, and designs leave the ladder only by the ladder's own rules. | M4 | GATE-AUGUR-28 | §25.32 |
| KILL-SYNTH-REGRESSOR | Trigger: The design-comparison part of GATE-AUGUR-29 fails; or more than 15% of designs infeasible on every library motion pass the mission in closed loop, in which case the motion-count screening signal is withdrawn first. Fallback: The mass-placement program of §25.21 and A-24 as they stand; candidates start from class templates. | M4 | GATE-AUGUR-29 | §25.33 |
| KILL-WEAKVAR | Trigger: The comparison of the gate fails; or more than 90% of the volume of constraints that read more than six parameters stays unresolved at budget on the reference families. Fallback: ADR-AUGUR-06 as first written: a constraint that reads more than six parameters receives no region certificate and is checked pointwise. | M4 | GATE-AUGUR-30 | §25.34 |
| KILL-BUDGETCURVE | Trigger: The pilot shows no crossing; or at GATE-ATEL-18 regret at equal compute falls by less than 30%; or J∞ is further from the oracle than J(b_max) for more than half the designs. Fallback: Fixed-budget scores, with the saturation index printed as a description and no escalation rule. | M4 | GATE-ATEL-18 | §28.25 |
| KILL-FREEPERIOD | Trigger: on the gait battery the section and bordered forms disagree beyond tolerance on more than 5% of autonomous orbits after one repair cycle, or the bordered iteration fails to converge on more than a quarter. Fallback: autonomous loops are analysed on an event section only; a loop with no transverse event is reported `unsupported` for orbit analysis and goes to finite-disturbance recovery tests, class `Tested`. | M4 | GATE-GAIT-04 | §22.23 |
| KILL-STRIDESEQ | Trigger: the pilot's falsifier holds after one repair cycle, or cycle enumeration exceeds its budget on the reference gaits. Fallback: no switching verdict is issued; multi-mode gaits are tested by finite-disturbance recovery over sampled admitted sequences, class `Tested`, and the per-mode multipliers carry the flag `single-mode-only`. | M4 | GATE-GAIT-05 | §22.24 |
| KILL-COUPLING | Trigger: one violated transported bound that survives a repair cycle, or no skill on the tray fixture with a budget below its slack. Fallback: `mission.transport` carries universal claims only, and every chance or expected-value claim is re-measured on the target in the qualification lane of §24.13. | M4 for the finite part (a violated transported bound); M8 for the tray clause | GATE-AUGUR-25 | §25.29 |

**6. Exit rule.** Recorded by a fresh-context session (seed BD-378, procedure N40-05). The exit line: *rank
agreement measured (GATE-REFLEX-04, GATE-CORTEX-12, GATE-ATEL-07); the family-policy bets decided (KILL-FAMILY,
KILL-INHERIT).*

**7. Dependency firewall.** Needs FrankenSim (optimizers, archive, surrogates), FrankenTorch (policy interchange),
FrankenSymPy (exact synthesis lane), fastmcp_rust (mission tools). If missing: the numerical lane of
KILL-SYNTH-LINKAGE; policies are exchanged in the native checkpoint format only.

**8. Owner input wanted.** OPEN-CORTEX-1 if still open (default: no pretraining). OPEN-SINEW-1 and the other
catalogue questions (which real actuator parts, sensor cards and film or soil tables enter the catalogues, under
what licences; default: generic classes and ranges labelled `assumed`, so every dependent result stays `Estimated`).
OPEN-SAFE-3 (sandboxed custom reward terms; default: not admitted). OPEN-TETHER-5 (whether the peer link between
the two machines earns its upkeep; default: standalone machines). OPEN-NEMESIS-3 (compute for a certified failure
rate of 10⁻⁶; default: certify to 10⁻⁵ and report 10⁻⁶ as `Estimated`). OPEN-PROG-5 before M5 closes (the order in
which admitted Moonshot items become active under the cap of four; default: none before its pilot passes).

**9. Stop or re-plan.** If KILL-RANK fires for every class and KILL-FAMILY fires as well, the lever "train few" is
gone. The ceilings are then recomputed with the measured trained fraction, every class whose ceiling is under 3
leaves the speed claim, and the sealed suite of M7 is re-stratified before it is sealed. M7 still runs, because its
result is published whatever it is. Specialized GPU work stops in any cell where the timing of the fourth mission
slice shows that transfers, authoring, training, checking or the rival's reuse rule out the target.

**10. The variant with no family policy (DESIGN; bead N40-34).** M4 can exit without one. It applies if
OPEN-CORTEX-1 stays at its default, if the pre-funding pilot meets its falsifier, or if KILL-FAMILY fires. The
rungs are then the static check, the baseline controller, the sampling controller, gait orbits, and per-design
specialists that inherit from the nearest parent; GATE-CORTEX-11 is recorded as failed or `not run`, BET-15 as
falsified or untested, NS-19 is not demonstrated, and the lost factor is recorded.

**11. No single failed bet blocks the exit.** BET-15, or no compute envelope → the variant with no family policy.
BET-17 (rank agreement is low) → cheap rungs that discard only; the fixed halving table. BET-20 (region
certificates decide too little) → dual witnesses, monotone bounds and sampled coverage labelled `Estimated`.

**12. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-28 | Optimization, lineage, experiment suggestion and the surface A/B [S], [F] | `frob-opt` | M4 | NS-06; GATE-ATEL-03, -15; GATE-TETHER-13 |
| WP-29 | Symmetry: equivariant policies and symmetric controllers [S], [F] | `frob-algebra`, `frob-analyze`, `frob-learn` | M4 | GATE-CORTEX-03; GATE-MARROW-04; GATE-REFLEX-07 |
| WP-30 | Wheels, tracks, hybrids, moving surfaces, soil [S], [F] | `frob-contact`, `frob-analyze` | M4 | GATE-TRAC-08; known answers of §18.16 |
| WP-61 | Family policies, inheritance and distillation [F] | `frob-learn` | M4 | GATE-CORTEX-11, -12, -13, -14 |
| WP-62 | The ladder, the cascade and the decision cores [S], [F] | `frob-opt`, `frob-verbs` | M3, M4 | GATE-ATEL-07 to -10, -13, -14, -16; GATE-PHYLUM-08 |
| WP-63 | Revalidation, the memo graph and quotienting | `frob-verbs`, `frob-chronicle`, `frob-phylum` | M3, M4 | GATE-ATEL-11, -12; GATE-PHYLUM-01, -05, -06, -07; INT-FSIM-16; INT-FSQL-02 |
| WP-64 | Gait orbits with continuation [F] | `frob-gait` | M4 | GATE-GAIT-01, -02, -03; INT-FSIM-13 |
| WP-65 | Energy-budget tier and runtime shields [F] | `frob-shield`, `frob-control` | M4 | GATE-SHIELD-01 to -04 |
| WP-66 | Mission queries at scale and region certificates [F] | `frob-mission`, `frob-cert`, `frob-lemma` | M4 | GATE-AUGUR-14, -15, -16, -19; GATE-LEMMA-04 |
| WP-67 | Synthesis lanes: co-design, trajectory optimization, bounded exact linkages [F] | `frob-synth`, `derive/frob-derive` | M4 | GATE-AUGUR-17, -18 |

## Acceptance criteria

- [ ] Every part in the gate tables has a recorded value and a caught negative control, or is recorded under the
      no-family-policy variant with its reason.
- [ ] τ, top-10 recall and intervals are recorded per class and task family, and a rung without a record is refused.
- [ ] The 30 kill gates of the compiled table and the 9 late ones each have an ADR with an outcome; KILL-FAMILY
      and KILL-INHERIT are among them.
- [ ] NS-05, NS-06, NS-08, NS-18, NS-19, NS-23 and NS-13 at scale run from a clean checkout, or are recorded as
      not demonstrated with the variant or kill decision that removed them.
- [ ] No more than four Moonshot items are active.
- [ ] The exit report for M4 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by member beads. Epic check: due counts for M4 are 0 admission, 49 convergence, 30 kill decisions, plus 16
late parts and 9 late kill decisions.

### End-to-end script and logging

`scripts/exit/review.sh M4`: the seven demonstrations as scripted sessions through CLI and MCP on `trj`; logs with
evidence blocks, τ with its bootstrap bounds beside 0.7 and 0.5, designs decided per hour, GPU-seconds per
accepted decision, the retrofit prices with their provenance labels, and the gates touched.

### Known answers and negative controls

τ thresholds 0.7 (select) and 0.5 (discard); 65,536-row screen labels identical on three backends; the co-design
fixed point equal to its closed form; rimless-wheel multiplier within 10⁻⁶. Controls: the shuffled-label student; a
planted over-scored design caught at the next rung; a planted winner removed by a biased screen; a payload-only
energy budget that shows an exceedance; four of four infeasible bodies refused with certificates.

## Considerations

- Family-policy pretraining is the largest single compute request of the program (class E3, approved per run with
  its ceiling and kill rule written first). Its default is "no pretraining", and the milestone still exits.
- 30 + 9 kill decisions: the heaviest decision load of any exit. Pilots before funding spread them over the
  milestone; the exit review still needs every ADR.
- M5 may overlap M4; nothing in M5 is needed for M4's exit or for M7.

## References

Plan §43.7, §43.1, §43.2, §40.3 to §40.6, §43.12. Bets BET-07, BET-10, BET-15, BET-16, BET-17, BET-19, BET-20,
BET-25, BET-26, BET-27, BET-30, BET-33, BET-35, BET-37. Scenarios NS-05, NS-06, NS-08, NS-13, NS-18, NS-19, NS-23.
Work packages WP-28 to WP-30, WP-61 to WP-67.
