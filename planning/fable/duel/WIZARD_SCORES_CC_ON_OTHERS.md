# WIZARD_SCORES_CC_ON_OTHERS.md: Claude's scores for the Codex (COD) and AGY idea slates

| | |
|---|---|
| Scorer | Claude Opus 5.5 (`claude-opus-5-5`) in Claude Code |
| Date | 2026-11-21 |
| Scored | `WIZARD_IDEAS_COD.md` (126 lines) and `WIZARD_IDEAS_AGY.md` (309 lines), both read in full |
| Also used | My own slate `WIZARD_IDEAS_CC.md`, the plan v0.2, `MOR_CASE.md`, read-only checks of sibling source (§2), and two web searches (§8) |
| Scale | 0–1000, as a judgment of quality, practical usefulness to agents and humans, buildability, and whether the gain pays for the complexity it adds |

## 0. How I scored, and my conflicts

**Anchors.**

| Band | Meaning |
|---|---|
| 850+ | Adopt now. A proven mechanism that changes what a decision costs, and incumbents cannot copy it cheaply. |
| 700–849 | Adopt with a kill gate. A strong lever whose main risk can be tested cheaply. |
| 500–699 | Worth building, but secondary, or strong in part with an overclaimed remainder. |
| 300–499 | Narrow, a polish item, wrappable by incumbents, or the cost outweighs the gain at this stage. |
| Below 300 | Reject for this plan. Out of scope, a slogan, or the objection is decisive. |

No idea reached 850. Every strong idea here still has an untested load-bearing assumption.

**What a score covers.** It scores the idea as written, with its own numbers. Overclaims cost points even when the
core idea is good, because the plan will inherit the numbers.

**Conflicts of interest.** Several ideas overlap with my own top five, so my scores on them may carry
self-preference. I flag each one, and where my own version shares a flaw, I say so.

| Their idea | Overlaps my |
|---|---|
| AGY #3, sampling MPC | CC #2 |
| AGY #5 and COD #5, replay interventions | CC #5 |
| COD #10, heterogeneous batching | CC #1 |
| AGY #9, a policy shared across robot bodies | CC #3 |
| COD #3, randomized referee audits | CC #4, an alternative estimator |

## 1. Headline ranking, both slates together

| Rank | Idea | Score | Verdict |
|---|---|---|---|
| 1 | AGY #3: sampling MPC as the universal zero-training controller | **790** | Adopt; replace the speed figures (§3.1) |
| 2 | COD framing: time from brief to an accepted design, with censoring and paired briefs | **740** | Adopt as the benchmark protocol |
| 3 | AGY #5: one-burst battery of interventions with solver-artifact slots | **670** | Adopt its intervention battery, with COD #5's validity rules |
| 4 | COD #10: batching different topologies by execution shape *(overlaps my #1)* | **660** | Adopt at M2; COD's 3× target undersells it |
| 5 | COD #5: replay interventions with prefix-validity rules and negative controls | **630** | Merge with AGY #5 |
| 6 | COD #3: randomized referee audits with inverse-propensity correction | **610** | Adopt as the validity layer over same-address pairing |
| 7 | AGY #9: a policy conditioned on morphology *(overlaps my #3; AGY cut it)* | **560** | M4 option, after MPC exists |
| 7 | COD #8: rare-failure importance sampling with a fixed audit distribution | **560** | M3–M4 |
| 9 | COD #1: requirements compiled into a sound branch-and-bound morphology search | **540** | Cheap CPU-first lane at M1 |
| 10 | AGY #8: sparse brick distance fields with exact edges for stair nosings | **500** | Engineering to do in M2/M3 |
| 11 | AGY #20, COD #20, AGY #7 | 480, 480, 470 | Secondary |
| … | AGY #4 link synthesis 460; COD #4 route evidence 460; AGY #1 fused kernel 440; AGY #2 contact adjoints 380 | | See §3 |

**What the ranking says.**

- The three slates converge on the same core: a controller that needs no training, a batch that holds different
  robot topologies, and controlled replay experiments. Three different models reaching the same three ideas
  independently is evidence they are right.
- The disagreement is about substrate.
  - AGY bets on raw stepping speed (AGY #1) and on gradients (AGY #2).
  - COD bets on statistics and sound pruning.
  - The checks in §2 favour COD's bet over AGY's.

---

## 2. Fact checks that moved scores

All of these were read-only checks of source on 2026-11-21, plus two web searches. Paths are under
`/Users/jemanuel/projects/frankensim/`.

| # | Claim | What I found | Affects |
|---|---|---|---|
| F1 | AGY: `render-gpu-piano/src/kernel.h` proves the fused SRAM pattern | **Partly true.** It is one persistent kernel shared by CUDA and Metal, using threadgroup memory, `shfl`, `warp_sum` and `block_sum`. Line 1 calls it *"The contact-free grand_piano step"*. It proves the toolchain and the single-source dialect. It says nothing about tree-structured articulated dynamics with variable contacts. | AGY #1 |
| F2 | AGY: `fs-adjoint` already supplies IFT contact and morphology adjoints | **The doctrine exists; the code is the wrong kind.** `fs-adjoint/src/ift.rs` is a matrix-free IFT over FEEC meshes (`TetComplex`, `fs_feec`, `fs_solver`). Its canonical fixture is a density-parameterized Poisson problem. Nothing articulated or contact-specific is reusable beyond the pattern and the checks `certs::fd_spot_checks` and `verify::verify_gradient`, both of which exist. | AGY #2 |
| F3 | COD #6, and AGY #2 by implication: `fs-time::hybrid_sensitivity` covers event derivatives | **True, and it refuses the common locomotion case.** Its docs say *"Initial/same-time events and competing brackets are refused."* A trot touches down two feet at the same time by design. | COD #6, AGY #2 |
| F4 | AGY #4: `fs-topopt::sdf3` already does 3-D CutFEM SIMP with stress, modal, thermal and body loads | **True, but gated.** It is compiled only with the `cutfem-marquee` feature, labelled *"[F]-gated per the Ambition-Tag rule"*. Its docs say cut volumes and sensitivities *"are not continuum certificates or proofs of optimality."* | AGY #4 |
| F5 | AGY #4: `fs-io::step_faceted_export` emits sealed STEP for synthesized links | **Not as stated.** The module *"accepts only an already decoded bare-`FACE` `DecodedFacetedBrep`"*, which is a re-emitter for B-reps already parsed from STEP. I found no public path from a triangle soup or a density field to a `DecodedFacetedBrep`. That path is new work. | AGY #4 |
| F6 | AGY #4: `fs-fab` supplies manufacturing constraints such as `overhang_angle` and `cnc_tool_radius` | **True, but they are scalar checks.** `FabConstraint::margin(&self, value: f64)` tests a number the caller supplies (471 lines in all). Nothing measures overhangs or tool access on a 3-D solid. | AGY #4 |
| F7 | AGY: `fs-toleralloc` provides `allocate`, `propagate_correlated_stack`, `robustness_check` and `gdt_report` | **True.** All four are in `fs-toleralloc/src/lib.rs`. | AGY #4 |
| F8 | AGY #11: `fs-constraint` diagnoses infeasibility | **True.** `fs-constraint/src/diagnose.rs` has `diagnose_infeasibility`. | AGY #11 |
| F9 | AGY #7: `fs-ladder`, `fs-recompute` and `fs-surrogate::koopman` exist | **True.** | AGY #7, COD #14 |
| F10 | COD: Isaac Lab's multi-asset spawning weakens the case for heterogeneous batches | **Weaker than COD implies.** Isaac Lab's docs say articulations spawned this way *"must have the same number of links and joints, the same number of collision bodies and the same names"* (web). | COD #10 |
| F11 | AGY #3: incumbents take 75–250 s per 10-s MPC rollout | **Contradicted.** DIAL-MPC reports real-time 50 Hz control on MJX. A later paper lists it at 2,048–4,096 samples on a GPU (web; a secondhand figure). Real time means a 10-s evaluation in about 10 s, not 75–250 s. | AGY #3, AGY #1 |

One check was internal arithmetic. AGY #1 says the plan's pipeline moves about 200 KB per world per substep through
memory. The plan's own budget (§28.4) is a few KB of state plus tens of KB of constraint rows per world. The
argument from a bandwidth ceiling therefore starts from a baseline about 5–10 times too high.

---

## 3. AGY's top five, in detail

### 3.1 AGY #3: sampling MPC as the universal zero-training controller: **790**

**What it is.** Annealed spline MPC in the style of DIAL-MPC, running on the design's own full-order model inside the
GPU batch. It replaces the plan's per-class baseline controllers (§18.3). The gait atlas seeds 25% of the samples,
an optional IFT polish follows, and the result can teach a learned policy.

**What is good.**

- It is the single best answer to MOR's open unknown: does a zero-training controller exist that works across body
  classes? It has published evidence behind it.
- It removes the plan's weakest load-bearing bet (BET-06) and gives every design a competent controller in the time
  of one rollout.
- Its falsifier is well chosen. Spearman ρ of at least 0.80 between MPC scores and converged PPO scores over
  30 designs tests exactly the risk that matters: that MPC flatters designs a deployable policy would struggle with.
- Seeding only 25% of samples from the gait atlas, so asymmetric and hybrid bodies are not handicapped, is a good
  detail.

**What is overclaimed.**

- **The speed figures assume AGY #1.** The "~2.1 s per 10-s evaluation" assumes 25 M world-steps/s. At the plan's
  2.0 M target, the same configuration costs about 26 s of GPU per world per 10 s.
- **The planner runs at full fidelity.** That means 2-ms substeps, a 0.4–0.6-s horizon and 512 rollouts, about
  5.1 M planning steps per world-second. A coarser planning configuration, with the planner-versus-execution gap
  recorded in the fidelity ledger, would cut this 5–10 times. That is what my own version (CC #2) does.
- **A decision needs many episodes.** About 24 paired episodes per design (GB-06), so per-design figures understate
  decision cost by roughly 50×.
- **The incumbent baseline is wrong (F11).** DIAL-MPC already runs in real time on MJX. Against B0⁺ (MJX plus
  DIAL-MPC), the per-design lever is near parity. The unique part is batching *different* designs and running the
  planner as its own tier. AGY credits the speed to AGY #1; that credit belongs to heterogeneous batching.
- **"Zero confounding bias" is false.** MPC is a different class of controller from the deployable policies the
  design will ship with. That is exactly why the Spearman falsifier exists.

**10×?** Yes. Against the common pipeline that trains PPO per design, this is the clearest real 10× in either slate.
Against B0⁺ it is near parity per design, and becomes 10× only together with heterogeneous batching.

**Strongest technical objection.** Sampling cost grows with action dimension and horizon. A 29-DoF humanoid or a
wheel-leg hybrid on stairs may need far more than 512 samples, and nothing in the write-up bounds that. The M1
falsifier on five templates has to cover a humanoid and a wheel-leg robot, not only quadrupeds.

**Keep.** All of it, with planning fidelity decoupled from execution fidelity and AGY's latency numbers struck.

### 3.2 AGY #5: one-burst battery of causal interventions with an exploit check: **670**

**What it is.**

- Rewind to the snapshot 1 s before failure and fork 64 worlds on the same random streams.
- Each fork applies one intervention: actuators, morphology, scene, sensing, or solver artifacts.
- Score each factor by "necessity", then test pairs among the top six.

**What is good.** Two details here are better than anything in my CC #5 or in COD #5:

- **Slot 0 is a determinism control.** The unperturbed replay must match the failing run bit for bit, or the burst
  is invalid. That is a cheap self-check on every probe.
- **Slots 57–63 test solver artifacts:** half step size, twice the Newton iterations, another contact law, a tighter
  stiction speed. If halving the step changes the outcome, the failure, or the success, is a numerical artifact.
  That makes the plan's trust ladder fire *inline*, at the moment an agent looks at a result. It is the best use of
  the three-tier design in either slate.

The fixed, typed battery is also easy for agents to use, and the follow-up over pairs handles coupled failures.

**What is overclaimed.**

- **"50 ms" holds only for a cheap controller.**
  - Under AGY's own universal controller (AGY #3), each forked world must re-plan. That costs 512 forks × 1.2 s ×
    5.1 M planning steps per world-second ≈ 3 × 10⁹ steps.
  - That is about 2 minutes at AGY's claimed 25 M steps/s, and about 26 minutes at the plan's 2 M.
  - Under a fixed learned policy it is cheap. But a policy trained on the original body does not adapt to a
    morphology intervention, so the attribution is confounded by controller mismatch.
  - My own CC #5 made the same error: its "a few seconds" holds only under a family policy, not under MPC. I correct
    that here.
- **"Exact causal proof" and "do-calculus" overstate it.**
  - A single-factor intervention from t_f − 1 s shows that a local fix was sufficient at that moment.
  - The root cause can lie before the window, for example in a gait chosen earlier.
- **Geometry interventions need state transport.** Something like `+1 cm foot radius` applied mid-episode can start
  the robot interpenetrating the ground. COD #5 handles this properly by replaying permanent edits from reset.

**10×?** Partly. Fewer agent turns per fix: 3–5× is believable. The inline exploit check is a qualitative unlock that
no incumbent offers.

**Strongest technical objection.** The burst forces a choice. With a re-planning controller it is expensive, by four
to six orders of magnitude over the 50-ms claim. With a fixed controller, its morphology attributions are confounded.

**Keep.** The determinism slot, the solver-artifact slots, and the pairwise follow-up. Take reset and prefix rules
from COD #5. Label the output `Estimated` and hand it to a full comparison, as my CC #5 does.

### 3.3 AGY #4: closed-loop link synthesis, fatigue checks, tolerance allocation and STEP export: **460**

**What it is.** Extract wrench envelopes for each link from rollouts, then:

- topology-optimize each link with `fs-topopt::sdf3`;
- check fatigue with `fs-solid`;
- feed exact mass and inertia back into the robot model;
- allocate tolerances from sensitivities;
- export STEP.

**What is good.**

- It reuses a very large sibling asset that the plan ignores.
- Merging load cases across symmetric limbs is clean.
- No robot simulator offers this, so it is genuinely a category of one.
- The fallback, beam-column sizing in the fast loop and full 3-D only for finalists, is the right shape.

**What is overclaimed.**

- **The sibling claims are weaker than stated:**
  - `sdf3` is feature-gated (F4);
  - the STEP exporter cannot take a synthesized solid (F5);
  - `fs-fab` checks scalars, not geometry (F6).
- **The loads are untrustworthy.** Peak link loads come from compliant contact, whose penalty stiffness *sets* the
  impact force.
- **Fatigue is quasistatic.** AGY says so honestly.
- **Mass savings are overstated.** In legged robots, motors, gearboxes and batteries dominate mass, so a 20–40% cut
  in *link* mass is a much smaller cut in robot mass. The mass-spiral amplification (κ = 0.6) is asserted, not
  derived.
- **SIMP output is not a part.** A bearing seat, a fastener pattern and assembly access are still human CAD work.
  "Days to minutes" is not credible.

**10×?** No for decision cost. It adds scope rather than speed. It is a real differentiator for a late milestone.

**Strongest technical objection.** Structural loads taken from a compliant-contact simulator are artifacts of the
contact model's stiffness. Optimizing structure against them optimizes against a solver parameter.

**Keep.** As an M4+ option. Beam-column sizing feeding mass and inertia back into the design loop is cheap and
useful earlier.

### 3.4 AGY #1: whole-robot persistent mega-kernel in on-chip memory, with tree-sparse LDLᵀ and per-world parameter rows: **440**

**What it is.**

- One persistent kernel per topology runs ten substeps with all state in shared or threadgroup memory, one warp per
  world.
- Factorization is tree-sparse (arrowhead) LDLᵀ.
- A per-world `DesignParamRow` lets continuous design parameters differ between worlds.
- Metal goes directly through `objc2-metal`.

**What is good.**

- The arrowhead observation is correct. A foot-contact Jacobian touches only the base and that leg's coordinates,
  so H = M + JᵀGJ keeps the tree's sparsity until limbs contact each other. The FLOP counts are right.
- Fusion is a known source of 2–4× speedups, and the persistent-kernel toolchain does exist (F1).
- The byte budgets per world (5.6 KB for a quadruped, 13.6 KB for a humanoid) fit both devices.

**What is overclaimed.**

- **The bandwidth premise is inflated about 5–10× (§2).**
- **20–30 M quadruped steps/s on one 4090 is unsupported.** The Genesis episode (E012) shows how such numbers fail
  under scrutiny.
- **The precedent is a contact-free string model (F1).**
- **Per-world parameter rows already exist elsewhere.** MuJoCo Warp has batched model fields that vary per world.
  Topology still compiles per `TopologyId`, so the "0 s recompile across 256 designs" holds only for parametric
  edits, which incumbents already handle.
- **The falsifier will take longer than a week.** Building a fused ten-substep kernel with an SDF narrow phase and
  variable contacts takes months, not "<1 week".
- **It undercuts verification.** A monolithic kernel works against the plan's certificates per stage and its CPU
  emulation oracle per stage.

**10×?** No. A 2–4× gain on world-steps/s is plausible, but world-steps/s is not the binding term in design-decision
cost, and the plan's 2.0 M target already exceeds what the decision loop needs.

**Strongest technical objection.** It optimizes the wrong term. AGY's slate composes 10–15× physics × 50–100× MPC ×
10–30× gradients into "20–100×". End-to-end time does not multiply that way. Physics speed counts only within the
physics-bound share of a decision.

**Keep.** Tree-sparse LDLᵀ now. Direct `objc2-metal` as the planned fallback. Fusion of stages 6–9 as a profile-driven
optimization after M2, my CC idea 16.

### 3.5 AGY #2: IFT adjoints of the converged contact solve, on a symmetry-reduced design manifold: **380**

**What it is.**

- Reuse the factored contact Hessian from the last Newton iteration for one back-substitution per step.
- Checkpoint with revolve.
- Smooth with antithetic common random numbers.
- Monitor the gradient against finite differences by cosine similarity.
- Take Riemannian gradients on pseudo-inertias.
- Feed short-horizon policy training (SHAC), design gradients and sensitivity reports.

**What is good.**

- The step-level IFT on a strictly convex compliant solve is correct and cheap.
- The automatic cosine monitor that downgrades a gradient's evidence class is exactly the right guard.
- Short-horizon sensitivity reports ("+1 cm thigh → …") are useful to agents even if everything else fails.

**What is overclaimed.**

- **The reusable code is not there.** `fs-adjoint`'s IFT is mesh-based (F2), so only the pattern and the FD checks
  carry over.
- **Simultaneous contacts are refused (F3).** The existing hybrid sensitivity code refuses simultaneous events,
  which locomotion produces by design.
- **"15–40 gradient steps against 300–1,000 for CMA-ES" ignores the loop.** It assumes a smooth closed-loop
  objective. Design quality under a sampling MPC or a learned policy depends on how the controller *responds* to θ.
  A step-level adjoint does not capture that, and returns over horizons of seconds are rugged in contact-mode
  switches.
- **Pseudo-inertias are not the design variables.** Agents edit lengths, gear ratios and mounts. The Riemannian
  structure on 𝒫⁺(4) is elegant but sits downstream of the variables that matter.
- **It loads M2.** Pulling differentiability into M2 puts a research risk on the milestone that must ship the GPU
  tier.

**10×?** Not for design decisions. Possibly 2–5× for short-horizon policy training on compliant contact.

**Strongest technical objection.** Closed-loop design gradients need the controller's sensitivity to the design,
and neither sampling MPC nor PPO provides it. Step-level adjoints cannot stand in for it, and over task horizons the
objective is not smooth.

**Keep.** Short-horizon sensitivities with the cosine monitor, at M3. Defer design gradients and SHAC.

### 3.6 AGY's framing, time to verified design (TTVD): **430**

- It is good that the definition includes buildability.
- But the formula multiplies the gains of separate phases as if they were independent.
- Several baselines are inflated: 15–60 GPU-hours for incumbents, 75–250 s for MPC, 200 KB of spill.
- Because the metric includes fabrication readiness that no incumbent attempts, part of any measured "speedup" would
  measure scope rather than speed.

### 3.7 AGY #9, cut by AGY: a policy conditioned on morphology: **560** *(overlaps my CC #3; discount for self-preference)*

**Why AGY cut it.** Its reasons are fair: a pretraining budget the owner has not approved (OPEN-SPARK-2), and the
risk of failing on hybrids outside the training distribution.

**What its version lacks:**

- a teacher (my version uses MPC teachers with DAgger);
- a ranking falsifier, beyond a return threshold.

Its "<90% of PPO after 30 s of fine-tuning" falsifier is stricter than mine, which is good.

**10×?** Yes on breadth, once trained. It is the only route to thousands of designs per GPU-hour under a competent
controller.

**Strongest objection.** Its rankings of close designs may reflect the policy's blind spots rather than the bodies.

---

## 4. COD's top five and framing, in detail

### 4.1 COD framing: time from task brief to an accepted design, with censoring: **740**

**What it is.** Measure the time from a task brief to the first robot plus controller that clears a frozen
acceptance battery. The protocol:

- restricted mean time at a shared cap;
- unresolved briefs counted as censored, not dropped;
- the strongest incumbent pipeline per brief, chosen on a separate tuning set;
- cold-start and amortized cost reported separately;
- four briefs to start: legged on rubble, wheeled docking, tracked on stairs, and a wheel-leg robot on thresholds.

**What is good.**

- This is the best-specified metric in any of the three slates, mine included. Censoring stops failed briefs from
  vanishing from the denominator.
- Selecting the incumbent on a tuning set stops cherry-picking.
- The Amdahl sentence is exactly right: *"A phase that accounts for less than 90% of baseline elapsed time cannot
  deliver 10× alone even if its cost falls to zero."*

**10×?** It is the instrument that makes 10× falsifiable, not a lever.

**Strongest technical objection.** Agent-driven runs vary widely. With four to seven briefs, paired uncertainty on
restricted mean time will be too wide to confirm 10×. It needs perhaps 20 or more held-out variants. Building the
incumbent arms (RoboGrammar, Co-design Gym, RoboMoRe, Drake) is itself months of work.

**Keep.** Adopt it as the protocol for my CC idea 25. The cost-per-decision and breadth metrics become component
metrics inside it.

### 4.2 COD #5: ranking repairs by replay interventions: **630**

**What it is.** Snapshots with copy-on-write branch descriptors. The rules:

- temporary interventions branch from a checkpoint;
- permanent edits replay from reset;
- an unchanged prefix *must* reproduce, as a negative control;
- the result is checked on held-out seeds.

**What is good.**

- It is the most careful treatment of validity among the three versions.
- Its incumbent arm is honest: native checkpoints plus hand-written patch adapters, with the time spent writing
  adapters charged against the incumbent.
- Its cost arithmetic is explicit: a branch costs 0.07 of a full run.

**What is weak.** Its own validity rules shrink the gain:

- Any permanent edit that touches mass, inertia, controller memory or contacts starts from reset, and that covers
  most design edits.
- The prefix saving therefore applies mainly to *diagnostic* interventions.
- The 10× is claimed only for late failures.

**10×?** Partly. It shortens diagnosis by 2–5×. A repair is still confirmed at full cost.

**Strongest technical objection.** The edits worth making are exactly those the validity rules send back to reset, so
the speedup lands on diagnosis, not on confirmation.

**Keep.** Merge with AGY #5. Take AGY's typed battery and solver-artifact slots, and COD's prefix rules, negative
controls and held-out confirmation.

### 4.3 COD #3: randomized referee audits with inverse-propensity correction: **610** *(an alternative to my CC #4)*

**What it is.**

- Z_i = m_i + A_i(Y_i − m_i)/p_i, where an audit is a Bernoulli draw with a logged, floored probability.
- A confidence sequence with bounded increments drives the stopping rule.
- Retries keep their audit assignment, and missing outcomes are handled under a declared policy.

**What is good.**

- It stays unbiased however wrong the cheap tier is. My same-address multilevel estimator assumes the tiers
  correlate; this one only needs randomization.
- It handles adaptive selection of challengers by demanding fresh seeds and a separate error budget.
- The cost arithmetic is unusually honest: rows cost 0.08 of a referee pair, so a 10× win leaves room for only
  1.25× as many rows.

**What is weak.**

- Referee calls fall by about 1/p only while p ≳ E[(Y − m)²]/Var(Y), which is roughly 1 − ρ². That is the same
  ceiling as control variates. The randomization buys validity, not efficiency.
- Increments bounded by range/p_min widen the confidence sequence when p is small. COD rates its own confidence in
  10× as medium-low, which is correct.

**10×?** Only in strata where the tiers agree closely (ρ ≥ 0.95). Its value is robustness, not more speed than the
alternatives.

**Strongest technical objection.** The confidence sequence needs a bound on its increments, and inverse-propensity
weights inflate that bound by 1/p_min. The design therefore trades away most of the saving it targets unless the
audit rate stays high.

**Keep.** Use it as the *validity layer*. Pair at the same world addresses (CC #4) to shrink Y − m, then audit
randomly to guarantee unbiasedness. The two together beat either alone.

### 4.4 COD #10: batching different topologies by execution shape: **660** *(overlaps my CC #1; discount for self-preference)*

**What it is.** Per-depth packed arrays of parents and edges, with batches bucketed by DoF, contact and sensor
capacity.

**Assessment.**

- COD lists it tenth, targets only 3× over Newton's heterogeneous worlds, and says it is worth doing only "when model
  churn dominates matched wall time".
- That undersells it. Under a zero-training controller or a shared policy, model churn *is* the workload.
- Isaac Lab's multi-asset spawning, which COD cites as competition, requires articulations with identical link and
  joint counts (F10). MuJoCo Warp does not support heterogeneous worlds, and Newton's support is experimental.
- The one-line write-up has a good kill test (packing, masks or kernel losses erase the gain) but no data layout for
  joint classes.

**10×?** Yes, as the enabler. On its own it is about 3×.

**Strongest technical objection.** Padding waste and bucket fragmentation. A population with wheels beside point feet
needs two-dimensional buckets (DoF × contact capacity), and those can fragment into many small batches.

### 4.5 COD #1: requirements compiled into a sound morphology search: **540**

**What it is.** Typed holes in the design term. Branch-and-bound prunes a node only with a checked bound over *every*
legal completion, and nondominated summaries are kept for resuming. The first gate is a 1,024-leaf exhaustive catalog
on the CPU.

**What is good.**

- It is sound by construction.
- It shows agents why a family was excluded and lets them resume after widening a requirement.
- Its first falsifier needs no GPU.

**What is weak.**

- Static necessary conditions prune only gross infeasibility: mass closure, torque to stand, geometry.
- The expensive question, dynamic performance under a controller, stays open.
- After the plan's `design check`, agents seldom propose grossly infeasible designs, so the share of evaluations
  saved may be small.

**10×?** Only against a grammar search that proposes many infeasible designs, such as RoboGrammar, not against an
agent working with checks.

**Strongest technical objection.** Monotonicity rarely holds once dynamics enter. Sound cuts then shrink to quasi-static
bounds, which are cheap to evaluate by point checks anyway.

### 4.6 COD #4: reusable route-feasibility evidence: **460**

**What is good.** A three-valued answer (clear, blocked, unknown) with explicit uncertainty from scan registration and
occlusion is exactly what an agent needs for doorways and galley kitchens.

**What is weak.**

- The baseline it beats, C-space checks per footprint, already costs seconds, so 10× of a small cost is small.
- Coverage of legged robots is weak.
- Sound interval enclosures over scene bricks is substantial new work.

**10×?** No for decisions. It is a useful pre-filter for wheeled and tracked bases.

**Strongest objection.** It reaches a definite answer only where geometry decides. Dynamic feasibility still needs
rollouts.

### 4.7 COD #2: transferring controllers across typed edits: **440** *(overlaps the transport part of my CC #3)*

**What is good.**

- Correspondence through typed edits is a real asset.
- The planted sign-or-ordering error test is good.
- Its confidence levels are honest.

**What is weak.**

- For parameter edits, where COD is most confident, inheriting the checkpoint already works because action dimensions
  do not change.
- The novel part, role changes and topology edits, is where COD rates confidence low.
- With a zero-training MPC available, the value of transfer falls further.

**10×?** No. About 2× fewer adaptation samples on local edits.

**Strongest objection.** It wins clearly only where plain warm-starting already works.

---

## 5. COD ideas 6–30

| # | Idea | Score | Toward 10×? | Strongest technical objection |
|---|---|---|---|---|
| 6 | Gradients per contact regime | 380 | No. Local tuning at best. | `fs-time::hybrid_sensitivity` refuses simultaneous events (F3), and gaits switch mode every 0.1–0.3 s, so regimes are too short to carry a design gradient. |
| 7 | Proof-driven time stepping in quiet windows | 290 | No (3× is its own target). | Lockstep batches step at the slowest world's pace. Legged contact changes leave few quiet windows, and certifying them costs extra. |
| 8 | Rare-failure importance sampling with a fixed audit distribution | 560 | Yes, for estimating rare-event probabilities. | Gaussian proposals collapse in mixed discrete and continuous scene-grammar spaces, and `fs-uq`'s admitted lane is Gaussian only. |
| 9 | Sharing test cases across symmetry orbits | 250 | No (≤ the group order). | Interiors are asymmetric, and float non-associativity plus noise streams that are not mirrored break exact score invariance. |
| 11 | Link optimization from measured loads | 420 | No. | Loads come from compliant contact (see §3.3), and link mass is a small share of robot mass. |
| 12 | Scene mutations that preserve task intent | 460 | Partly. Finds failures faster. | "Task intent" is hard to formalize, and coverage-guided search needs a behavioral coverage metric. It can be wrapped around incumbents. |
| 13 | Correlated manufacturing tolerances | 240 | No. | Hardware is out of scope, and failed fits are not a simulation decision. |
| 14 | Bounded reduced model per locomotion regime | 310 | No. | Design edits break the exchangeability conformal bands need, so it abstains constantly where it would be useful. |
| 15 | Skill sequences over a contact-mode graph | 420 | Partly, for hybrids. | Guard tables per morphology contradict generality, and sampling MPC with gait priors already switches modes. |
| 16 | Incremental contact-geometry builds | 240 | No. | Typed robot edits never touch the scene, which is lowered once. Rebuilding robot geometry is tiny. |
| 17 | Designing actuator-identification experiments | 250 | No. | No hardware is in scope, and identifying a simulator's parameters from that simulator is circular. |
| 18 | Policy interchange by semantic actuator role | 410 | No, but good hygiene. | The gain is modest, though joint-order mismatches between model formats are a real and common bug. |
| 19 | Index of contact-compatible modules | 210 | No. | Part catalogs hold hundreds of entries, and a linear scan costs microseconds. |
| 20 | Decision cache across versions with a dependency graph | 480 | No (2–4×). | A missed invalidation serves a stale verdict, which is worse than recomputing. |
| 21 | Contact material grids from interface evidence | 300 | No. | No measured interface data exists to calibrate the correlations. |
| 22 | Symmetry matched to broken task context | 290 | No (2×). | Switching subgroups gives discontinuous actions, as COD itself notes. |
| 23 | Geometric phase as an experiment generator | 180 | No. | It applies to principally kinematic systems such as swimmers and snakes, not to legged contact or wheeled bases. |
| 24 | Acquiring the measurement that changes a decision | 400 | No. | In pure simulation the "measurement" is just another run, and real card fields need hardware. |
| 25 | Local energy balance to triage solver failures | 400 | No. | Compliant contact dissipates legitimately, so per-port residuals alias with real physics. The plan's ledger covers the aggregate. |
| 26 | Certified recovery region for one maneuver | 290 | No. | It works on reduced models only, degree grows quickly, and it does not inform comparisons between designs. |
| 27 | Mac for scene diversity, CUDA for training | 360 | No (2×). | The Metal tier is itself unproven (KILL-METAL), and scheduler gains are bounded. |
| 28 | Forecast feedback on agent decisions | 320 | No. | Calibrating agents needs many briefs, and the signal is noisy. |
| 29 | Differentiating through all contact changes | 160 | No. | Gradients through contact are biased and memory-heavy, and MJX and Genesis are already differentiable. COD parked it correctly. |
| 30 | Full MPM or FEM soil in every world | 100 | No. | It exceeds the per-world memory budget, and Newton already has MPM. Parked correctly. |

(COD #10 is in §4.4.)

## 6. AGY ideas 6–30

| # | Idea | Score | Toward 10×? | Strongest technical objection |
|---|---|---|---|---|
| 6 | Live shadow referee on the M5's unified memory, with no copies | 400 | No. | The referee runs at about 0.2× real time per core, so 36 cores sample a sliver of millions of steps per second. "<1 s lag" cannot hold except for a handful of worlds. |
| 7 | Scheduler combining value of information, conformal surrogates and recomputation | 470 | Partly (2–5×). | Adaptive search breaks conformal exchangeability, and wiring four crates (F9) is a large integration. |
| 8 | Sparse brick distance fields with exact edges | 500 | No for decisions, yes for large interiors at 5 mm. | The plan already has a two-level field, and blending exact overlays into it creates discontinuities in the field's gradient. |
| 9 | Policy conditioned on morphology | 560 | Yes on breadth. | See §3.7. |
| 10 | Monotone co-design antichains over part catalogs | 460 | No alone. | Monotonicity breaks under dynamics, and catalogs are not totally ordered. It is a good quasi-static pre-filter. |
| 11 | Certified envelope prover plus minimal unsatisfiable specification | 450 | No. | Intervals and SOS scale only to reduced models. The minimal conflicting set of requirements (F8) is the cheap, valuable part. |
| 12 | Adversarial interior and disturbance generator ("red-team habitat") | 440 | Partly. | It can be wrapped around incumbents, building-code invariants must be encoded, and MAP-Elites needs behavior descriptors. |
| 13 | Real-to-sim identification against hardware | 320 | No. | Hardware is out of scope (MOR §1). |
| 14 | Calculus for broken symmetry | 280 | No (2–4×). | It refines policy learning and does not change decision cost. |
| 15 | Exact contact-wrench cones and a mode-transition planner | 290 | No. | It grows combinatorially past 6–8 contacts, and sampling MPC already handles mode switches. |
| 16 | Canonicalizing morphologies with e-graphs | 270 | No (≤2×). | Canonical hashing of labelled trees is linear time, so e-graphs are overkill for the job. |
| 17 | Nonholonomic Lie-bracket primitives as a passability prover | 290 | No. | Ball–box estimates are local with unspecified constants, not proofs in cluttered rooms. Reeds–Shepp plus C-space search is already exact for car-like bases. |
| 18 | Koopman and Floquet stability oracle | 310 | No. | Push recovery is a large-perturbation question, Floquet multipliers describe infinitesimal stability, and DMD on hybrid gaits is unreliable. |
| 19 | Port-Hamiltonian actuators and contact | 290 | No. | It improves the referee's fidelity, not loop speed, and IDA-PBC is impractical for underactuated legs. |
| 20 | Coupled interior fixtures switched on near the robot (doors, carts) | 480 | No for speed, yes for capability. | Fixtures add DoF and contacts per world, which breaks fixed capacities. "<5% idle overhead" is unsupported. |
| 21 | Thermal and fluid surrogate cards | 270 | No. | Ten-minute duty cycles are expensive to simulate, and a two-node thermal model already exposes throttling. |
| 22 | Perturbations generated from a scanned twin | 370 | No. | It needs semantic segmentation of the scan, and the twin pipeline is M5. |
| 23 | Clustering paired episodes by failure | 430 | No (3×). | It is a thin layer an incumbent wrapper could build, and it depends on good signature features. |
| 24 | Mission compiler across morphology families | 390 | No alone. | It is a facade over #10, #11 and #17, each weak once dynamics enter. |
| 25 | Shared swarm knowledge graph and sub-assembly grafting | 390 | No (2–3×). | Cache poisoning across agents, and the value of grafting is unproven. |
| 26 | Roofline autotuner carrying proofs | 220 | No (1.3–2×). | Re-associated floating point is not exactly equivalent, so "proven equivalence" needs error bounds, not identities. |
| 27 | Asymmetric pipeline across the two 4090s | 270 | No. | Sharding independent worlds already scales about 2× with no traffic between the cards. |
| 28 | Cone-traced sensors | 320 | No. | Proprioceptive design loops do not need depth, and it matters only at M5. |
| 29 | Co-designing stator windings with `fs-em` | 210 | No. | Teams rarely choose custom motors, and analytic scaling laws cover the trade. |
| 30 | Symbolic closure of kinematic loops | 260 | No. | Common loops (four-bar knees, parallel ankles) have closed forms, and the Gröbner cap (E008) limits the rest. |

(AGY #1–5 and #9 are in §3.)

---

## 7. What each slate does best, and what the plan should take

**COD.**

- **Strengths:**
  - the strongest falsifiers and kill tests;
  - honest incumbent arms;
  - an explicit Amdahl argument;
  - care over statistical validity.
- **Weaknesses:**
  - it is under-ambitious in places: #10 at 3×, MPC entirely absent;
  - its ranking scatters attention across 30 narrow lanes;
  - few of its ideas would change *what* an agent can do rather than how rigorously it is checked.
- **Take:**
  - the benchmark protocol (§4.1);
  - randomized audits as the validity layer (§4.3);
  - the prefix and reset rules and negative controls for replay (§4.2);
  - requirement compilation as a cheap lane at M1 (§4.5).

**AGY.**

- **Strengths:**
  - it puts the zero-training controller at the centre;
  - it uses the trust ladder inline, through solver-artifact intervention slots;
  - it notices the large unused sibling assets for structure and fabrication.
- **Weaknesses:**
  - its arithmetic is inflated in its baselines (§2, F11) and in its multiplication across phases;
  - several sibling capabilities are overstated (F2, F4, F5, F6);
  - wording such as "exact causal proof" and "zero confounding bias" claims more than the mechanisms deliver.
- **Take:**
  - sampling MPC as the default controller, with planning fidelity decoupled from execution;
  - the determinism and solver-artifact slots;
  - tree-sparse LDLᵀ;
  - direct `objc2-metal`;
  - the minimal unsatisfiable specification core from `fs-constraint`;
  - exact edge overlays for stair nosings;
  - link synthesis as an option at M4 or later, starting from beam-column sizing.

**Correction to my own slate.** Checking AGY #5 showed that my CC #5's cost figure ("a few seconds" for about
10⁴ counterfactual worlds) holds only under a policy shared across bodies. Under sampling MPC the same probe costs
on the order of 10³ GPU-seconds. Until the family policy exists, the probe should run under MPC with fewer
interventions and shorter continuations, or under the gait-table controller, labelled as such.

## 8. Sources

**Web.** Two searches on 2026-11-21. These are search summaries; the pages were not opened.

- Isaac Lab multi-asset spawning, which requires the same link and joint counts and names for articulations:
  [v3.0.0-beta how-to](https://isaac-sim.github.io/IsaacLab/v3.0.0-beta/_sources/source/how-to/multi_asset_spawning.rst),
  [current docs](https://docs.nvidia.com/isaaclab/isaac-lab/spawning-multiple-assets)
- DIAL-MPC, real-time at 50 Hz on MJX: [arXiv 2409.15610](https://arxiv.org/abs/2409.15610). The
  2,048–4,096-sample figure is secondhand, from a comparison table in [arXiv 2511.19204](https://arxiv.org/pdf/2511.19204v2).

**Local.** Read-only, 2026-11-21. Paths are under `/Users/jemanuel/projects/frankensim/`.

- `apps/instrument-lab/render-gpu-piano/src/kernel.h`
- `crates/fs-adjoint/src/ift.rs`, `certs.rs`, `verify.rs`
- `crates/fs-time/src/hybrid_sensitivity.rs`
- `crates/fs-topopt/src/lib.rs`, `sdf3.rs`, `Cargo.toml`
- `crates/fs-io/src/step_faceted_export.rs`, `step_faceted.rs`
- `crates/fs-fab/src/lib.rs`
- `crates/fs-toleralloc/src/lib.rs`
- `crates/fs-solid/src/fatigue.rs`
- `crates/fs-constraint/src/diagnose.rs`
- `crates/fs-surrogate/src/`
- Directory listings and line counts for `fs-motion`, `fs-ladder` and `fs-recompute`.
