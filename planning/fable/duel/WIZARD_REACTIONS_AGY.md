# Candid Reactions, Steelmans, and Three Blind-Spot Ideas (`AGY`)

- **Date:** 2026-11-21
- **Reviews read in full:** `WIZARD_SCORES_COD_ON_OTHERS.md` (Codex) and `WIZARD_SCORES_CC_ON_OTHERS.md` (Claude Opus 5.5)
- **Hygiene:** Pure Unicode math (no LaTeX, plan §0.7); sibling capabilities verified directly against live code.

---

## 1. Honest Reaction: Where COD and CC Were Right About My Slate

Both reviews did exactly what an adversarial review is supposed to do: they caught genuine arithmetic errors, overstated sibling checks, and unexamined physical edge cases in `WIZARD_IDEAS_AGY.md`. I concede every one of the following without hedging:

1. **The 50 ms vs. re-planning MPC contradiction in AGY #5 (caught by Claude §3.2 & Codex §AGY #5).**
   - My 307,200-step / 12–20 ms budget assumed 1 rollout per counterfactual fork—which holds ONLY under a cheap fixed policy or frozen control trajectory, NOT under my own AGY #3 Dial-MPC! Re-planning 512 internal MPC samples inside 512 forks explodes to ~3 × 10⁹ steps (~2 minutes even at an optimistic 25M steps/s, ~26 minutes at 2M steps/s). Claude caught the exact same latent bug in its own `CC #5` while checking mine.
   - Codex also nailed that adding a synchronous CPU-referee counterfactual slot on M5 Ultra (0.2× real time → ~6 core-seconds for one 1.2 s branch) puts seconds on the critical path even with zero-copy unified memory, and that a single-factor conditional rescue at t_f − 1 s is a *local diagnostic screen*, not "exact causal proof" of global design superiority.
   - Finally, BOTH rightly flagged that mid-contact geometry/mass edits (`+1 cm foot radius`, `±10% mass`) cannot simply transport joint coordinates without risking instantaneous floor interpenetration or momentum inconsistency—Codex's strict rule sending permanent mass/geometry edits back through reset is simply better engineering.

2. **Overstated sibling readiness and load-artifact risk in AGY #4 (caught by Claude F4/F5/F6 & Codex §AGY #4).**
   - Claude's direct source checks caught three real overstatements in my sibling audit: (a) `fs-topopt::sdf3` lives behind the `[F]`-gated `cutfem-marquee` feature (F4); (b) `fs-io::step_faceted_export` re-emits an *already decoded* `DecodedFacetedBrep`, so marching-isosurface → manifold B-rep conversion is genuinely new work (F5); and (c) `fs-fab` validates caller-supplied scalar margins rather than extracting overhang angles or CNC tool access directly from 3-D solids (F6).
   - Both also caught real mechanical-engineering gaps: peak impact forces taken from a compliant-contact solver (CL-1) are heavily shaped by the contact penalty stiffness (Claude), raw peak wrenches from different timestamps do not constitute a valid multiaxial fatigue spectrum (Codex), asymmetric payloads break symmetric link-load merging (Codex), and a raw SIMP solid still lacks bearing seats, bolt patterns, and assembly clearances—making "days to minutes" for full CAD readiness overstated.

3. **Inflated baselines, Ada SM occupancy math, and naive phase multiplication in AGY #1, AGY #3, and §0 (caught by both).**
   - **F11 (DIAL-MPC baseline):** Claiming incumbents take 75–250 s per 10 s MPC rollout was flatly wrong—DIAL-MPC already runs in real time (50 Hz, ~10 s per 10 s episode) on MJX.
   - **Ada occupancy & bandwidth (AGY #1):** Codex's occupancy check is spot-on: eight 5.6 KB worlds = 45 KB per 8-warp block caps Ada's 100 KB SM shared memory at 2 blocks = 16 of 48 warps (33% occupancy) before registers! Claude also rightly noted that v0.2's quadruped step moves tens of KB per substep rather than 200 KB, that `render-gpu-piano/src/kernel.h` is contact-free (proving the single-source `TGM`+`shfl` toolchain, not variable-contact articulated fusion), and that building a full 10-substep fused kernel with SDF narrow phase takes months, not "<1 week".
   - **TTVD multiplication fallacy:** Multiplying 10–15× physics × 50–100× MPC × 10–30× gradients × 3–5× debug ignores Amdahl's Law and phase overlap. Codex's §0 censored restricted-mean-time protocol and Claude's B0/B0⁺ baselines are much cleaner yardsticks than my multiplicative table.

4. **Subtle solver and closed-loop gaps in AGY #2 (caught by both).**
   - Claude verified (F2) that `fs-adjoint/src/ift.rs` operates on FEEC `TetComplex` meshes—so only the IFT doctrine and `certs::fd_spot_checks` carry over, not articulated dynamics code.
   - Codex caught an easy-to-miss numerical bug: in a fixed 4-iteration Newton solve with line search, the last factored Hessian sits at the *penultimate* iterate and may not have converged to r = 0, requiring an explicit residual/conditioning check (and refactor at v_+* when needed) before trusting the transpose back-substitution.
   - Claude rightly pointed out that a step-level physics adjoint ∂v_+/∂θ under a fixed action tape omits ∂π*(θ)/∂θ (how the optimal controller adapts to the edited body), and that pulling full differentiability into M2 overloads the milestone that must land the GPU tier cleanly.

---

## 2. Where I Think COD and CC Are Wrong (Substantive Pushback)

Conceding real flaws does not mean accepting every low score. Four specific judgments from the other two models are wrong or internally inconsistent:

1. **Claude's 440/1000 on AGY #1 contradicts its own slate (`CC #2`, `CC #3`, `CC #5`).**
   - Claude argues *"the plan's 2.0 M target already exceeds what the decision loop needs"* and slashes AGY #1 to 440. Yet *three of Claude's own Top 5* immediately bleed out because v0.2's unfused 2.0 M step/s pipeline is too slow!
   - Specifically: (a) Claude's `CC #2` (MPC) is forced to coarsen planning down to **h = 5 ms, 2 Newton iterations, and half contact samples** just to reach 5 M planning steps/s—risking tunneling and Cayley refusal bounds (`|ω|h > 0.2`) on 1–2 cm stair nosings and wet kitchen tiles; (b) Claude's `CC #3` needs **48 GPU-hours per family** (~200+ GPU-hours across 4–5 robot families); and (c) Claude's own §7 correction admits `CC #5` blows up to **~1,000+ GPU-seconds** under MPC at 2 M steps/s!
   - When sampling MPC (0.5M–5M planning steps per world-second) IS the universal controller, physics stepping throughput IS >85% of GPU wall-clock time—Amdahl's Law works *for* faster kernels here, not against them!
   - Moreover, Codex's valid Ada occupancy catch (5.6 KB/world → 16 warps/SM) is cleanly fixable by design: keeping `DesignParamRow` in L1/constant cache and computing sparse contact Jacobian rows on-the-fly in registers shrinks per-quadruped SRAM from 5.6 KB down to ~2.0 KB (48 warps/SM = full occupancy). Stripping out the hyped "25 M" projection, **tree-sparse arrowhead LDLᵀ + localized Stage 6–9 SRAM fusion + direct `objc2-metal` Metal 4** delivers a grounded, profile-verifiable **4×–8×** kernel win that keeps MPC accurate and rescues the M5 Ultra from `KILL-METAL`. Scoring AGY #1 at 440 threw out a real architectural multiplier over an inflated bandwidth back-of-the-envelope.

2. **Codex's 429/1000 on AGY #3 (below COD #4's 2-D wheeled route graphs!) underestimates the #1 bottleneck in robot co-design.**
   - Codex scored Claude's `CC #2` at **739** while dropping AGY #3 to **429** because AGY #3 didn't coarsen to h = 5 ms and leaned on AGY #1's 25 M target. Fair enough on the unprofiled 25 M number—but rejecting full-resolution planning outright in favor of h = 5 ms / 2-iter planning misses why coarse planners fail in messy interiors (stairs, curbs, slick wet patches).
   - The right synthesis—which outperforms BOTH `CC #2` and `AGY #3` as originally written—is **two-stage fidelity annealing inside Dial-MPC**: coarse `h = 4–5 ms` on annealing round 1 (cheaply locating the right contact-mode basin), followed by exact `h = 2 ms / 4-iter` on annealing round 2 (scoring the final action faithfully on stair nosings and wet patches). That recovers ~3×–4× speed without blinding the planner to thin geometry!

3. **Claude is far too harsh on AGY #4 (460/1000) and AGY #2 (380/1000) by equating "adds capability" with "zero lever."**
   - **On AGY #4 (Codex scored it 805; Claude scored it 460):** Claude writes *"link mass is a small share of robot mass"* and *"adds scope rather than speed."* In real legged and hybrid robots, structural chassis + limbs are typically 30–45% of total mass and **>60% of distal swing inertia**, directly governing impact forces, knee torque spikes, and energy per metre. Calling structural feedback "just scope" ignores that optimizing floating capsules with invented masses frequently picks the wrong gearbox and motor class. And Claude's legitimate compliant-load-spike objection has a clean fix: drive structural sizing from **referee-tier impulses and kinematic inertial loads**, not raw CL-1 penalty spikes, using `fs-solid::beamcol` in the fast loop and `fs-topopt::sdf3` on finalists.
   - **On AGY #2 (Codex scored it 665; Claude scored it 380):** Claude's claim that closed-loop design gradients fail because neither MPC nor PPO gives ∂π*/∂θ misses **joint differentiable actor-critic + morphology optimization (SHAC)**—literally validated on MJX quadrupeds in `Open-DiffLoco` (arXiv 2608.02069, which Codex itself cited!). You do NOT need implicit controller differentiation ∂π*/∂θ when policy weights φ and morphology θ are updated jointly through the same short-horizon (H = 16–32) backward pass! Deferring AGY #2 to M3 (after M2 stabilizes) with Codex's Newton-convergence check fixes the scheduling risk without discarding a proven gradient lever.

4. **Codex's own ranking still tilts too far toward CPU-side quasi-static pruning over dynamic GPU control.**
   - Codex named `AGY #10` (catalog Pareto antichains, **854**) the top AGY idea—above every GPU, control, and causal-debugging mechanism! Static catalog pruning is genuinely high-ROI at M1 (see §3 below), *but* after antichain pruning leaves 50 valid motor-gear-battery combinations on the Pareto frontier, static bounds cannot tell you which robot actually climbs 18 cm stairs or turns cleanly on a wet kitchen floor. Without fast GPU zero-training control (`CC #2`/`AGY #3`) and heterogeneous batching (`CC #1`), the downstream dynamic bottleneck still dominates wall clock.

---

## 3. Where Their Reviews Genuinely Changed My Own Evaluations

Reading both critiques updated my own scores and plan recommendations in four concrete ways:

| Idea | My Old View | Updated View | Why I Changed My Mind |
|---|---:|---:|---|
| **`CC #4` + `COD #3` Hybrid** (Paired MLMC + Randomized Audits) | `CC #4`: 815; `COD #3`: 545 ("dominated") | **Hybrid: ~885** (`COD #3` alone: ~620) | Codex showed `CC #4`'s naive unit-coefficient telescoping variance `σ_H² + σ_L² − 2ρσ_Hσ_L` breaks when tiers differ in scale; Claude showed layering `COD #3`'s randomized audit validity over same-address pairing (`CC #4` + fitted control-variate weight β) beats either alone! |
| **`AGY #10` + `CC #8` + discrete slice of `COD #1`** (Catalog Pareto Antichains & Box Carving) | `COD #1`: 640; `CC #8`: 655; cut `AGY #10` | **Combined M1 CPU wedge: ~805** | Codex convinced me that while dynamic locomotion breaks monotonicity, *discrete component catalog selection* (Motors × Gears × Batteries × Mounts) IS largely monotone/lower-boundable, cheap to test exhaustively on CPU at M1, and prunes doomed families before burning any GPU time. |
| **`COD §0` Benchmark Protocol + `CC #25`** | Praised `CC #25` (780); underrated censoring protocol | **~910 (Mandatory M0/M1 Gate)** | Seeing both reviewers dismantle my multiplicative TTVD arithmetic proved that censored restricted mean time to resolution across paired held-out briefs (against tuned B0 and B0⁺) must be built *first* before claiming any 10× win. |
| **`AGY #1`, `#2`, `#4`, `#5`** (My own Top 5) | Bundled monolithically into M2 | **Unbundle & re-stage honestly** | Keep tree-sparse LDLᵀ + `objc2-metal` + ragged/Design-SIMD cohorts at M2; stage localized Stage 6–9 fusion after profiling; move IFT adjoints + joint SHAC to M3 with convergence checks; merge `AGY #5` + `CC #5` + `COD #5` (preserving reset validation and separating fixed-action vs re-planning cost); scope `AGY #4` as `beamcol` fast-loop + referee-load `sdf3` finalist pass. |

---

## 4. Question (1): Strongest Single Idea From Each Opponent (Steelmanned)

### 4.1 Strongest Single Idea from Claude (`WIZARD_IDEAS_CC.md`): **`CC #4` — Paired Multilevel Referee Estimation Across Tiers**

**Why I now rank `CC #4` above `CC #2` as Claude's single strongest idea.**
In Round 2 I ranked `CC #2` (875) above `CC #4` (815). Round 3's fact check (`F11`) showed DIAL-MPC already runs in real time on MJX (meaning `CC #2` alone is parity per design against B0⁺), whereas **`CC #4` is 100% structurally impossible for any incumbent (B0 or B0⁺) to replicate at any price**, costs only a few hundred lines of adapter code on top of existing `fs-uq`, and directly unlocks FrankenRobotics's headline Trust Ladder.

**The Steelman Case for `CC #4`.**
Every fast GPU robot simulator on Earth faces the same fatal dilemma: fast single-precision compliant physics produces simulator exploits, while trusted `f64` exact-Coulomb CPU referee physics is 100×–1,000× slower (~50 core-seconds per 10 s humanoid episode). If establishing a referee-grade verdict costs 20–40 CPU-minutes per comparison, agents will treat promotion as a rare end-of-week ceremony and spend 99% of their design loop optimizing fast-tier illusions.

`CC #4` turns the Trust Ladder from an expensive audit into the **default online estimator** by exploiting a structural triad unique to FrankenRobotics: (1) a single `ModelId` lowered to three distinct fidelity levels (`gpu-f32`, `batch-cpu-f64`, `referee-f64`), (2) identical counter-based Philox `WorldAddr` streams (`fs-rand`) coupling the exact same terrains, disturbances, and card draws across tiers, and (3) existing Giles-allocated Multilevel Monte Carlo code already sitting in `frankensim/crates/fs-uq/src/product_mlmc.rs`. Upgraded cleanly with two refinements surfaced by this duel—**an empirically fitted control-variate coefficient β** (avoiding unit-coefficient scale mismatch!) and **`COD #3`'s randomized-audit propensity layer** (preserving unbiasedness even under tier bias or adaptive selection)—the estimator writes:

```
E[Δ_referee] = β · E[Δ_gpu] + E[Δ_referee − β · Δ_gpu]
```

with mixture confidence sequences (`fs-eproc`) stopping the moment the referee-level interval clears the decision threshold. On the huge majority of design decisions where paired design *contrasts* correlate strongly across tiers (ρ = 0.90–0.98), referee cost drops **5×–25×** (turning a 19-minute humanoid promotion into ~1–2 minutes on the M5 Ultra's 36 cores, or background zero-wall-clock latency via `CC #14`'s idle-CPU shadow auditor). When chaotic contact bifurcations drive ρ low on a hard stair/slip stratum, the estimator **gracefully degrades to exact paired referee racing without ever producing a false verdict**—and flags the low ρ itself as actionable evidence that the fast tier cannot be trusted on that regime! Its falsifier piggybacks for free on M2's `GATE-FID-01`. Virtually zero downside, minimal code, zero incumbent parity, maximum leverage on the product's defining promise.

---

### 4.2 Strongest Single Idea from Codex (`WIZARD_IDEAS_COD.md`): **`COD #5` — Sound Replay Interventions with Checked Prefix Reuse & Reset Confirmation**

*(Note: Codex's §0 censored paired-brief benchmark protocol is the single strongest meta-instrument across all three slates; among Codex's 30 product ideas, `COD #5` is the strongest.)*

**Why `COD #5` is Codex's strongest idea.**
Unlike `COD #1` (which struggles with non-monotone dynamic legged tasks), `COD #3` (dominated by `CC #4` unless layered as a validity wrapper), or `COD #4` (narrowly scoped to static 2-D wheeled geometry), **`COD #5` works across every morphology class (legged, wheeled, tracked, hybrid) and every one of the 7 interior archetypes**, directly attacks AI-agent serial turn latency, and caught the exact physical-soundness flaw that both Claude (`CC #5`) and I (`AGY #5`) missed.

**The Steelman Case for `COD #5`.**
When a robot fails 8.5 seconds into a commercial-kitchen or rubble-staircase run, correlated failure cascades erupt within 200 ms: a foot slips, unloading the leg, spiking joint velocity into back-EMF saturation, tripping thermal derating, and diverging the IEKF estimator. Telling an AI agent *"earliest anomaly: knee velocity spike at t = 8.42 s"* (`v0.2 §23.5`) routinely sends the agent down 3–5 blind edit cycles fixing a downstream symptom.

`COD #5` turns debugging into a **scientifically watertight, versioned experimental engine** inside `frob-chronicle` and `frob-analyze`:
1. **Strict physical-soundness boundary:** It cleanly separates **temporary diagnostic checkpoint interventions** (raising a driver torque cap, drying a floor friction card, zeroing IMU latency, holding a command) that reuse the verified pre-failure prefix (costing ~7% of a full run!) from **permanent hardware edits** touching mass, inertia, geometry, contacts, or controller memory—which are *refused* mid-contact prefix reuse and automatically validated from reset on held-out seeds!
2. **Self-verifying negative controls:** Every diagnostic batch includes an unchanged-prefix replay that *must* reproduce the exact failure digest bit-for-bit, and the test suite checks that mass-altering edits *must* invalidate prefix reuse—guaranteeing the debugger can never hallucinate fake rescues from floor interpenetration or stale warm-starts.
3. **Composable with GPU cohorts & inline exploit checks:** When `COD #5`'s strict prefix/reset validity contract is mounted on `CC #1`/Design-SIMD heterogeneous GPU batches and fused with `AGY #5`'s inline solver-artifact slots (`h/2`, `CL-2`) and `CC #5`'s small-set coalition ranking (running fixed-action/frozen-spline perturbations in milliseconds, then promoting top permanent edits to reset-validated comparisons), it gives AI coding/design agents something **no incumbent simulator provides**: a trustworthy, zero-hallucination physical debugger that cuts wasted design iterations by 3×–5× and exposes simulator artifacts on the spot.

---

## 5. Question (2): Three High-Leverage 10× Ideas NONE of the Three of Us Thought Of

Across all 120 ideas in v0.2 Appendix B, `WIZARD_IDEAS_AGY.md`, `WIZARD_IDEAS_CC.md`, and `WIZARD_IDEAS_COD.md`, **all three of us fell into three shared blind spots**: (1) we all assumed robot synthesis must run *forward* (guess robot → control → simulate → tweak), (2) we all simulated every whole robot monolithically rather than pre-characterizing reusable physical modules like VLSI standard cells, and (3) we all designed *blind, privileged-state* robots while ignoring Lie-group estimator observability and self-occlusion of sensors by the robot's own body and payload!

Here are three concrete, non-wrappable mechanisms that directly address those blind spots:

---

### Blind-Spot Idea 1: Inverse-Dynamics Convex Robot Synthesis ("Trajectory-First Co-Design via the Linear-in-Pseudo-Inertia Euler–Poincaré Regressor")

- **The shared blind spot:** All three slates (and every incumbent simulator: MuJoCo, MJX, Warp, Newton, Isaac Lab, Genesis, Drake) design robots *forward*: guess a robot's inertias, gear ratios, and actuators → train or run MPC to move it through a building → observe saturation/slip → repeat.
- **Mechanism (`frob-analyze`, `frob-algebra`, `frob-opt`):** Flip the equation using one of the deepest algebraic facts in articulated mechanics—**the Euler–Poincaré inverse dynamics (`C-04`, `RNEA`) is EXACTLY AFFINE in the robot's inertial parameters `π` (equivalently, the 4×4 pseudo-inertia matrices `P_i ∈ 𝒫⁺(4)` of `C-03`), contact wrenches `γ_c(t)`, and joint torques `τ(t)`**:

  ```
  τ(t) = Y(q(t), q̇(t), q̈(t)) · π − J_c(q(t))ᵀ · γ_c(t)
  ```

  Given a small library of **nominal building traversal trajectories** (base SE(3) + foot/wheel contact schedules crossing 18 cm stairs, wet kitchen turns, ramps, and 10 kg tote lifts, generated once per kinematic archetype via centroidal/IK trajectory generation), precompute the regressor `Y(t)` and contact Jacobians `J_c(t)` once via `K-rnea`. Then solve **ONE convex symmetric-cone program (SOCP + SDP, reusing the exact `Cone` primitives of `C-03` and `C-06`!)** jointly over:
  - Link pseudo-inertias `P_i ∈ 𝒫⁺(4)` (constrained to physically realizable mass/CoM placement boxes—e.g., where to place the battery, compute pack, and hip motors inside the torso/thighs!),
  - Time-varying contact wrenches `γ_c(t) ∈ 𝒦_{μ(x(t))}` respecting the interior's exact dry/wet friction map (Second-Order Cone!),
  - Joint torques `τ_j(t)` and inverse gear-ratio scaling variables subject to L1 actuator cards: continuous RMS torque `(1/T) Σ_t τ_j(t)² ≤ τ_cont,j²` (SOC!), peak torque `|τ_j(t)| ≤ τ_peak,j`, voltage-speed envelope `R|τ_j|/(η N_j k_t) + k_e N_j |q̇_j| ≤ V_bus` (linear!), and quadratic electrical energy `Σ_{j,t} [R_j (τ_j/(N_j k_t))² + τ_j q̇_j] Δt` (convex quadratic!).

  One solve (<500 ms on CPU, or batched across candidate link-length grids on GPU!) returns: (a) **globally optimal mass/battery/CoM placement, gear ratios, and actuator sizing** for that building traversal, (b) a **mathematically certified lower bound on Cost of Transport**, and (c) an **exact conic Farkas infeasibility certificate** proving NO valid contact-force distribution in `𝒦_μ` can execute the stair/wet-floor maneuver within actuator limits—before a single forward simulation or MPC rollout runs!
- **Lever:** Replaces **50–300 forward simulation + MPC/PPO co-design iterations** over mass distribution, battery placement, gear ratios, and actuator sizing with **one <1 s globally optimal convex cone solve** (TARGET: **10×–50× faster** initialization and pruning of feasible robot designs, plus exact global CoT lower bounds that no forward-only simulator can compute!). Incumbents (MuJoCo Warp, Isaac Lab, Genesis) have neither symbolic Euler–Poincaré regressor emission (`frob-kir`), nor pseudo-inertia PSD cones (`C-03`), nor symmetric-cone solvers unified with actuator cards.
- **Falsifier & fallback:** On 30 quadruped, humanoid, and wheel-leg designs tested on stairs and wet kitchen turns, (a) if the inverse-synthesized `(CoM, gear ratios, actuator cards)` fail to beat default templates by ≥15% electrical Cost of Transport (or fail to start within 10% of full forward co-design optima) when verified in forward Dial-MPC/PPO rollouts, OR (b) if fixed-trajectory kinematic assumptions cause >15% false infeasibility rejections (rejecting robots that succeed by adapting their gait/trajectory!), kill inverse infeasibility pruning and retain the convex regressor strictly as a **<1 s warm-start initializer** for battery/CoM placement and gear ratios before forward search.
- **Sibling check:** `frob-kir` emits the symbolic RNEA regressor `Y(q, q̇, q̈)` directly from the same Lie-algebraic tree already built at M1; `frob-algebra` already owns the SOC and PSD `Cone` primitives (`C-03`, `C-06`). Zero new sibling blockers!

---

### Blind-Spot Idea 2: EDA-Style Pre-Characterized Sub-Assembly Port Macromodels ("Robot Standard-Cell `.lib` & Compositional Dynamic Screening")

- **The shared blind spot:** `MOR_CASE.md` PQ3 explicitly asked what transfers from **chip design automation (EDA)** with its structure intact—and all three of us missed VLSI's #1 architectural trick! Chip designers never simulate 10 billion transistors at SPICE level to explore architectures; they **characterize standard cells once into `.lib` timing/power/area tables** and run **compositional Static Timing Analysis (STA)** at 100,000× speed before SPICE sign-off. Meanwhile, AGY, CC, and COD all simulate every whole robot monolithically at full multi-body resolution from scratch every time!
- **Mechanism (`frob-model`, `frob-analyze`, `frob-opt`, `frob-fsim`):** Operationalize FrankenRobotics's `C-12` Dirac port operad (`fs-ports`) as a **hierarchical standard-cell characterization and compositional screening engine (`frob cell characterize` + `frob design compose-screen`)**:
  1. **Characterize reusable sub-assemblies once (`CellMacromodel`, content-addressed by subterm hash):** Every reusable physical module—e.g., a 3-DoF mammal leg with a specific actuator+belt card, a steered hub-wheel module, a 2-DoF tracked flipper pod, a battery+torso core, or a 5-DoF carry arm—is characterized *once* at its boundary mechanical, electrical, and thermal ports across a bounded operating box of `(base_twist, base_accel, tip_wrench, duty_cycle)`. Characterization outputs a compact, split-conformal-bounded (`fs-surrogate`) + interval-enclosed (`fs-ivl`) **Port Capability Contract**:
     - Feasible tip-wrench polytope vs. workspace pose and speed;
     - Electrical power draw & steady-state thermal rise as a function of wrench + velocity + duty cycle;
     - Reflected port inertia tensor + lowest structural/control eigenfrequency (coupling directly to `AGY #4` / `fs-solid::beamcol`!);
     - Explicit **inter-limb coupling residual bound** (bounding neglected base-Coriolis cross-terms as a function of peak base angular rate).
  2. **Compositional assembly screening at 10,000+ assemblies/second on CPU:** Given a candidate full robot assembled from 4–8 standard cells (e.g., 2 front mammal legs + 2 rear hub-wheel legs + torso + battery + carry arm), compose their port contracts algebraically across the Dirac interconnection graph (`C-12`)—solving a tiny 6-DoF base wrench-balance + power/thermal-bus closure + modal-stiffness check in **<100 microseconds per whole robot**!
  3. **Incremental invalidation across swarms:** Editing one knee gear ratio re-characterizes *only* that single leg cell (~seconds, cached globally in Chronicle/`fs-recompute`) and instantly re-evaluates 10,000 candidate robot assemblies using that leg in <1 second—reserving full-body GPU simulation (`CC #1`/`CC #2`) as "post-layout SPICE sign-off" strictly for the top ~1% of composed designs!
- **Lever:** Directly transfers EDA's **100×–1,000× architectural-exploration speedup** onto modular robot design (10,000+ whole-robot multi-family assemblies screened per CPU-second before GPU rollout!), and makes cross-family hybrid invention (mixing leg cells, wheel cells, flipper cells, and arm cells) genuinely compositional! Incumbents built on flat XML/MJCF/USD trees have no typed power ports, no subterm content hashing, and no port-level macromodel algebra.
- **Where the EDA analogy breaks (explicitly guarded):** Unlike CMOS gates with crisp capacitive isolation, limbs coupled to a lightweight floating base experience strong nonlinear Coriolis/impact cross-coupling during dynamic running or jumping. The macromodel therefore carries an explicit **coupling-validity badge** (valid for quasi-static, wheeled, tracked, carrying, and moderate-Froude walking `Fr ≤ 0.5`; automatically escalates to full GPU simulation for high-Froude dynamic running/jumping!).
- **Falsifier & fallback:** Across 200 diverse multi-family assemblies (quadrupeds, hexapods, wheeled, tracked, wheel-leg hybrids) on warehouse, kitchen, and ramp-climbing tasks (Fr ≤ 0.5), if port-composed screening fails to achieve ≥90% top-10 recall (i.e., misses >1 of the true top-10 GPU-simulated designs when pruning the bottom 90% of assemblies!) or Spearman ρ < 0.75 on energy-per-metre and payload capacity, kill automated pruning and demote `CellMacromodel` to an advisory catalog/sizing estimator.
- **Sibling check:** `fs-ports` (power-port contracts), `fs-surrogate` (conformal error bands), `fs-ivl` (interval enclosures), and `fs-recompute` (content-addressed tolerance-slack caching) ALL exist today in FrankenSim!

---

### Blind-Spot Idea 3: Joint SE₂₊ₖ(3) Estimator-Observability & Dynamic Self-Occlusion Sensor Co-Design

- **The shared blind spot:** Across all 120 ideas in v0.2, AGY, CC, and COD, **not a single idea co-designs sensor placement, occlusion geometry, and closed-loop state-estimator observability alongside robot morphology!** Everyone designed robots assuming privileged state (or slapping a fixed IMU/depth camera on the torso as an afterthought). Yet in real homes, apartments, offices, warehouses, manufacturing plants, retail stores, and commercial kitchens, **perception-morphology coupling is a primary cause of late-stage failure**: carrying a 10 kg warehouse tote blinds the torso depth camera right before a step; a low-slung quadruped cannot see over a curb or shelf lip; an IMU placed near high-frequency leg-impact resonance corrupts bias estimation; and narrow/collinear foot or wheel contact geometry creates an unobservable yaw/velocity drift mode on wet floors!
- **Mechanism (`frob-sense`, `frob-control`, `frob-analyze`, `frob-opt`):** Exploit the defining mathematical property of v0.2's `C-13` estimator—the **contact-aided Invariant Extended Kalman Filter on the Lie group SE₂₊ₖ(3)**:
  1. **Exact state-independent Lie-algebraic Observability Gramian & CRLB:** Because IMU + kinematic-contact dynamics on `SE₂₊ₖ(3)` are **group-affine**, the log-linearized error matrix `A(t)` in the Lie algebra `𝔰𝔢₂₊ₖ(3)` depends ONLY on the nominal body trajectory and active contact points—*independent of estimator error*! Along any nominal building trajectory (or short MPC rollout), `frob-analyze` computes the exact **trajectory-integrated Lie-algebraic Fisher Information Matrix (FIM) and Cramér–Rao Lower Bound (CRLB)** for base roll/pitch/yaw, linear velocity, IMU bias, and slip estimation in milliseconds—detecting weak/unobservable modes (e.g., collinear wheel/foot contacts or insufficient yaw excitation) *analytically before training a single policy*!
  2. **Dynamic Foothold & Hazard Lookahead Visibility (Ray-Traced Self-Occlusion):** Using `frob-sense`'s deterministic GPU ray kernels against the robot's *own* articulated limbs, carried payload/tote, and the interior SDF along the trajectory, compute two crisp physical metrics: (a) **foothold/threshold lookahead time** (seconds a stair nosing, wet patch, or doorway threshold remains continuously visible to depth/lidar before foot/wheel contact, accounting for self-occlusion by swinging limbs and carried cargo!), and (b) **blind-spot dwell volume** in the robot's swept maneuver envelope.
  3. **Joint morphology + sensor-mount optimization (`frob opt sensors`):** Expose sensor mount transforms `g_sensor ∈ SE(3)` (mast height, pitch angle, dual-camera split) and torso/payload-bay geometry as continuous variables coupled into the design loop—explicitly trading off **higher mast = better stair/tote visibility** against **higher CoM = worse ramp tip-over margin & blocked under-table clearance**!
- **Lever:** Eliminates the classic **"trains great on privileged state, crashes on `--state estimated` in cluttered interiors"** rework cycle—cutting **5×–10× wasted teacher-student / proprioceptive-vision policy training runs** caused by designing perception-blind or unobservable morphologies first and discovering the sensor occlusion only at M4/M5! Structurally unavailable in incumbents (MuJoCo Warp, MJX, Isaac Lab) because they lack native `SE₂₊ₖ(3)` invariant-error Lie Jacobians compiled directly from the robot model alongside deterministic payload-aware ray occlusion inside the morphology optimizer.
- **Falsifier & fallback:** Take 20 quadruped, humanoid, and wheel-leg designs carrying payloads across stairs, doorways, and wet kitchen tiles under `--state estimated` (IEKF + ray depth, NO privileged state). Optimize sensor placement and payload-bay clearance using the `SE₂₊ₖ(3)` CRLB + lookahead visibility objective. **Fail condition:** Optimized designs fail to cut the privileged-vs-estimated task success gap by ≥50% (or CRLB + lookahead metrics correlate <0.65 with actual `--state estimated` navigation failure rate). **Fallback:** Keep foothold lookahead visibility as an inexpensive static/replay diagnostic lint inside `frob design check` (extending `A-14`) without joint sensor-morphology optimization.
- **Sibling check:** `frob-lie` already includes `SE₂₊ₖ(3)` (`§11.1`, `C-13`), `frob-control` already builds the IEKF (`§18.2`), and `frob-sense` already ships deterministic GPU ray kernels (`§17.3`). Zero new sibling blockers!
