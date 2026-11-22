# Candid Evaluation & Scoring of Opponent Wizard Proposals (`AGY` on `COD` and `CC`)

- **Date:** 2026-11-21
- **Evaluated files:** `WIZARD_IDEAS_COD.md` (Codex) and `WIZARD_IDEAS_CC.md` (Claude Opus 5.5)
- **Verification basis:** Full read of both proposals, `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` v0.2, `MOR_CASE.md`, and direct read-only inspection of cited sibling code (`fs-uq/src/{product_mlmc,product_importance}.rs`, `fs-time/src/hybrid_sensitivity.rs`, `fs-motion/src/clearance.rs`, `fs-ivl/src/multi_taylor.rs`, `fs-topopt/src/sdf3.rs`, `fs-solid/src/fatigue.rs`, `fs-adjoint/src/*`, `fs-mbd/src/articulated.rs`, `ft-kernel-metal/src/compute.rs`, `render-gpu-piano/src/kernel.h`).
- **Scoring rubric (0–1000):** Composite judgment of (a) real-world usefulness for AI coding/design agents and humans, (b) genuine movement toward a 10× unlock over the strongest late-2026 incumbents (MuJoCo Warp, MJX + DIAL-MPC, Newton, Isaac Lab, Genesis, Drake), (c) engineering practicality and correctness, (d) ROI of utility versus complexity/tech debt, and (e) grounding in verified hardware (2× RTX 4090, M5 Ultra 80-core GPU / 256 GB unified memory) and real sibling code.
  - **850–1000:** Transformative, structurally un-wrappable pillar with high ROI and solid grounding.
  - **700–849:** Strong, practical, genuinely accretive mechanism with clear payoff and manageable debt.
  - **500–699:** Real engineering merit, but bounded lever (<10×), high complexity/debt, narrow scope, or dominated by a cleaner alternative.
  - **300–499:** Fragile, hardware-hostile, over-scoped, or weak ROI.
  - **0–299:** Slogan, thin wrapper, scope creep, or actively harmful to plan invariants.

---

## 1. Executive Comparison & Calibration Table

### 1.1 Detailed Top-5 Ideas Head-to-Head (Sorted by Score)

| Rank | ID | Author | Idea | Score | 10× Unlock? (1-line) | Strongest Technical Objection (1-line) |
|---|---|---|---|---:|---|---|
| 1 | `CC-2` | Claude | Nested batched sampling MPC (DIAL-MPC + planner-gap ledger) | **875** | Yes vs B0 (50–200×) and retires `BET-06`, though only parity per design vs B0⁺ until paired with cohorts/fast kernels. | Coarsening the planner to h = 5 ms and 2 Newton iters (forced by keeping v0.2's slow pipeline) risks tunneling/compliance bias on stairs and wet floors, and pure sampling struggles on 29-DoF humanoids. |
| 2 | `CC-1` | Claude | Ragged cohorts: one launch steps thousands of different robots | **845** | Yes for design breadth in composition with MPC/probes and removes JIT-recompile walls; partially overlapped by experimental Newton. | Padding heterogeneous robots into dense Cholesky buckets wastes O(n_v³) FLOPs (2–3×) and fragments across DoF × contact-capacity while keeping v0.2's global-VRAM multi-kernel bottleneck. |
| 3 | `CC-4` | Claude | Paired multilevel estimation across tiers (`fs-uq` MLMC + `fs-eproc`) | **815** | ~10× *on referee promotion* (uniquely FrankenRobotics, tiny cost), not standalone 10× end-to-end. | Chaotic contact bifurcations over 10 s trajectories decorrelate GPU and referee outcomes, dropping ρ below 0.7 and collapsing the (1 − ρ²) gain from 10× to <2×. |
| 4 | `CC-5` | Claude | Counterfactual probes: edit the robot at the moment it fails | **790** | Moves agent turns-to-fix 3–10× vs heuristic/scalar baselines by isolating physical vs scene faults. | Transporting state mid-contact across geometry/mount edits (`foot.radius`, `move-mount`) teleports primitives into floor interpenetration, creating fake impulse rescues or crashes. |
| 5 | `COD-5` | Codex | Rank repairs by actual replay interventions (checked prefix reuse + reset validation) | **740** | Real 3–5× reduction in agent debug thrashing, but falls well short of 10× wall-clock speed without GPU cohort batching or fast MPC. | Requiring reset-from-t=0 for every mass/inertia/contact edit while lacking heterogeneous GPU batching erases most of the prefix-reuse speed gain on real hardware design edits. |
| 6 | `CC-3` | Claude | One policy per robot family (MPC-taught GNN + typed-edit carry) | **670** | High ceiling (>10× breadth vs B0⁺ after ~5,000 designs), but heavy upfront cost and unproven generalization. | Rebuilding GNN + masked-attention + LayerNorm autograd in custom CUDA/MSL dialect kernels plus 48–200 GPU-h pretraining risks massive tech debt and OOD ranking bias against inventive outlier morphologies. |
| 7 | `COD-1` | Codex | Compile engineering requirements into morphology search (typed holes + B&B) | **640** | Cuts static catalog search 10× on simple wheeled/monotone sizing, not on dynamic legged/hybrid design. | To guarantee zero false cuts, completion bounds over dynamic legged/hybrid partial trees must be so conservative they go vacuous and prune almost nothing beyond cheap leaf checks. |
| 8 | `COD-2` | Codex | Transfer controllers across typed edits (SE(3) adjoint + role blocks) | **595** | No 10× unlock: largely matched by standard warm-start fine-tuning on parameter edits and brittle on topology edits. | Fails to solve cold-start control for new families and loses transfer fidelity on discontinuous support/topology changes—the exact regime where universal sampling MPC wins immediately. |
| 9 | `COD-3` | Codex | Target referee work at the decision with randomized AIPW audits | **545** | No: only touches referee promotion and is mathematically dominated by `CC-4`'s coupled MLMC. | Inverse-propensity weighting (1/p_i) blows up the bounded-increment range in anytime confidence sequences on rare tier reversals, destroying sample efficiency (which Codex itself flags). |
| 10 | `COD-4` | Codex | Compile reusable route feasibility evidence (interval bricks + optimistic graphs) | **510** | No: speeds up a tiny fraction (<2%) of total design time and mostly serves static 2D wheeled bases. | Rigid swept-hull interval enclosures yield massive `unresolved` abstention rates for articulated/legged robots climbing stairs, stepping over clutter, or pushing doors. |

### 1.2 Overall Portfolio Assessment

- **Claude (`WIZARD_IDEAS_CC.md`)** assembled a far stronger, tighter **Top 5** (average **799**) that directly attacks the real GPU/control/agent bottlenecks (`CC-1` + `CC-2` + `CC-4` + `CC-5`), defined an admirably honest dual baseline (`B0` and `B0⁺`), and found `fs-uq`'s existing MLMC machinery. Its three biggest strategic misses: (1) surrendering on raw GPU kernel architecture by cutting `CC-16` (leaving v0.2's slow global-VRAM multi-kernel pipeline intact and forcing unsafe `h = 5 ms` coarsening in `CC-2`), (2) completely missing FrankenSim's ~290k-line structural/topopt/fab/STEP stack, and (3) dismissing smooth short-horizon IFT contact gradients.
- **Codex (`WIZARD_IDEAS_COD.md`)** wrote the **stronger scientific benchmark protocol** (§0 censored restricted mean time, cross-engine adjudication, amortized cost accounting) and did deeper, more precise source inspection (`fs-time::hybrid_sensitivity`, `fs-motion::clearance`, `fs-uq::product_importance`, `fs-topopt`). However, its **Top 5** (average **606**) suffers from GPU aversion and excessive formalism: it skipped GPU execution speed, skipped universal zero-training control, buried three of its best ideas (`COD-11` topopt, `COD-6` regime gradients, `COD-10` heterogeneous batching) in the cut list, and selected CPU-bound formal/statistical wrappers (`COD-1`, `COD-3`, `COD-4`) that either abstain on dynamic robots or are dominated by simpler, stronger mechanisms.

---

## 2. Detailed Evaluations: Claude's Top 5 (`WIZARD_IDEAS_CC.md`)

### `CC-1` — Ragged cohorts: one launch steps thousands of different robots · **Score: 845 / 1000**

- **10× Unlock (1-line):** Yes for design-population breadth when composed with MPC/family policies/probes (and eliminates MJX's 24 s+ per-structure JIT wall), though partial overlap with Newton's experimental heterogeneous worlds means it is not a 10× unlock in isolation.
- **Strongest Technical Objection (1-line):** Bucketing heterogeneous morphologies into padded dense Cholesky sizes n_v ∈ {12, 18, 24, 32, 48, 64} wastes O(n_v³) compute (up to 2–3× FLOPs on boundary sizes), fragments across 2-D `(DoF × contact_capacity)` buckets, and still sits atop v0.2's memory-bandwidth-starved 10-stage global-VRAM pipeline.

**Candid Evaluation.**
This is an exceptionally smart architectural refactor of v0.2's batching boundary. The cleanest stroke is turning C-01's screw basis `S ∈ ℝ^{6×3}` with a column mask `k ∈ {1, 2, 3}` into pure per-model table data so revolute, universal, planar, and spherical joints execute one branchless generated kernel—this is exactly what "algebra as working machinery" looks like. Grouping 16–32 worlds of the same model contiguously so every warp executes the same tree-level schedule cleanly avoids intra-warp divergence, and the 200 MB memory footprint for 4,096 distinct models fits easily on both RTX 4090 (24 GB) and M5 Ultra (256 GB). Implementing it upfront at BD-074/BD-082/BD-099 avoids an expensive post-M2 rewrite.

Two flaws keep it from 900+: First, because Claude chose a generic multi-stage global-memory cohort loop (explicitly cutting kernel fusion in `CC-16`), it accepts **0.6×** of v0.2's already-modest 2.0M quadruped target (~1.2M world-steps/s)—whereas compiling fixed-topology `DesignParamRow` batches inside an SRAM-resident mega-kernel runs an order of magnitude faster for continuous/card sweeps, with ragged cohorts reserved specifically for mixed-DoF topology generations. Second, contact-sample disparity (rim-sampled wheels/tracks vs point-foot quadrupeds) creates a 2-D bucketing problem that Claude candidly flags in its confidence note and will likely fragment small diverse cohorts. Even so, designing model tables as per-world indexed data from Day 1 is an unmistakable upgrade over v0.2.

---

### `CC-2` — Sampling MPC that plans on each design's own model, nested inside the batch · **Score: 875 / 1000**

- **10× Unlock (1-line):** Yes against B0 (50–200× faster than per-design PPO and retires v0.2's brittle `BET-06` heuristics), though only parity per design against B0⁺ (MJX + DIAL-MPC) until combined with heterogeneous cohorts and faster kernels.
- **Strongest Technical Objection (1-line):** Running the inner planner at h = 5 ms with only 2 Newton iterations and half contact samples (forced by v0.2's slow 2M step/s pipeline) risks contact tunneling, Cayley refusal bounds (`|ω|h > 0.2`), and severe planner-vs-execution divergence on 1–2 cm stair nosings and wet floors, while pure zeroth-order sampling degrades sharply on 29-DoF humanoids.

**Candid Evaluation.**
This is the single highest-impact, most pragmatic idea in either opponent slate. v0.2's biggest hidden trap is `BET-06`—assuming hand-crafted classical controllers (capture-point + gait table + QP) will make 80%+ of novel/hybrid robot designs walk. Replacing that fragile per-class synthesizer with GPU-nested annealed MPPI (DIAL-MPC) running directly on each design's own compiled model immediately unlocks zero-training dynamic evaluation for quadrupeds, hexapods, wheeled bases, tracked robots, and wheel-leg hybrids with zero pretraining compute and smaller code complexity than the gait-table synthesizer it demotes.

Claude added three genuinely sharp FrankenRobotics-native refinements: (1) deterministic counter-based Philox sampling so the controller itself is replayable and teleportable, (2) recording the **planner-vs-execution gap** in the fidelity ledger so an agent sees if MPC is exploiting its own coarse planner, and (3) a dead-cheap M1 CPU falsifier (`K = 64` on 5 templates) plus a Kendall τ ≥ 0.6 ranking correlation check against PPO. The main engineering flaw is self-inflicted: because Claude abandoned raw GPU kernel fusion (`CC-16`), it had to coarsen the planner aggressively (`h = 5 ms`, 2 Newton iters, half contact samples) to hit ~5M planning steps/s. On narrow stair nosings, wet kitchen tiles, and high-speed swing legs, `h = 5 ms` with 2 Newton iterations will either violate Cayley's `|ω|h ≤ 0.2` bound or soften contacts enough to distort design rankings. Pair `CC-2` with an SRAM-resident mega-kernel (running exact `h = 2 ms` physics at 20M+ steps/s) and optional short-horizon gradient polishing for 29-DoF humanoids, and it becomes a 930+ cornerstone.

---

### `CC-3` — One policy per family of robots, taught by MPC and carried across typed edits · **Score: 670 / 1000**

- **10× Unlock (1-line):** Potentially >10× breadth over B0⁺ *only* after amortizing 48+ GPU-hours across >5,000 designs per family, with high risk of failing the ranking-fidelity falsifier on inventive out-of-distribution morphologies.
- **Strongest Technical Objection (1-line):** Because FrankenTorch is CPU-only (E007), building forward+backward kernels for GNN message passing, masked attention pooling, and LayerNorm in a raw CUDA/MSL C-dialect adds enormous DL-framework tech debt (~2× Cortex I) for a model prone to OOD ranking bias that penalizes novel designs.

**Candid Evaluation.**
The tokenization scheme here is genuinely tasteful: feeding each joint token its Lie screw basis, `log(base_from_joint) ∈ 𝔰𝔢(3)`, normalized subtree inertia, actuator-card limits, and C-09 symmetry orbit label turns FrankenRobotics's typed model into an architecture-native embodiment representation, and substituting MPC teachers (DAgger) for GENBOT-1K's 1,000 per-body RL expert runs is clever.

However, the complexity, compute, and epistemic risks are severely underestimated relative to the payoff. First, **implementation tech debt is massive**: writing and debugging numerically deterministic CUDA + Metal backward passes for gather/scatter GNN message passing, masked attention pooling, and LayerNorm in a custom C-dialect (without PyTorch/JAX) is a multi-month ML-compiler project on top of a physics engine! Second, **compute amortization is harsh**: 48 GPU-hours *per family* across 4–5 families (legged, wheeled, tracked, wheel-leg, mobile-arm) means ~200+ GPU-hours on a heavily loaded `trj`, breaking even against B0⁺ only after **5,000 designs per family** (Claude's own calculation!). Third, **design-search Goodharting**: a generalist policy inevitably controls canonical grammar-center designs better than weird inventive outliers, biasing topology search *against* true design breakthroughs. Right verdict: treat as an M5 frontier research bet (only after `CC-1`, `CC-2`, and `FD-FT-02` real FrankenTorch GPU autograd exist), never as an M4 load-bearing pillar.

---

### `CC-4` — Paired multilevel estimation across tiers (`fs-uq` MLMC + `fs-eproc`) · **Score: 815 / 1000**

- **10× Unlock (1-line):** Delivers a ~5–15× speedup specifically on *referee-established promotion* (uniquely impossible for incumbents lacking a second trusted shared-address tier), though not a standalone 10× end-to-end design-loop speedup.
- **Strongest Technical Objection (1-line):** Chaotic contact timing over 10 s dynamic episodes (falls, slips, stair catches) can rapidly bifurcate GPU and CPU-referee trajectories, driving whole-episode correlation ρ below 0.7 and collapsing the (1 − ρ²) speedup from 10× down to <2×.

**Candid Evaluation.**
This is a textbook high-IQ, ultra-low-cost architectural find. Direct inspection of `frankensim/crates/fs-uq/src/product_mlmc.rs` confirms Claude's claim: FrankenSim *already* implements coupled coarse/fine Multilevel Monte Carlo with disjoint Philox sample ordinals, Giles allocation `N_ℓ ∝ √(V_ℓ / C_ℓ)`, and pilot estimation—and v0.2 completely omitted `fs-uq` from §25.4 (`FD-FSIM`)! Combining three levels (`gpu-f32`, `batch-cpu-f64`, `referee-f64`) with `fs-eproc` mixture confidence sequences at `α/3` (union bound) costs only a few hundred lines of adapter code (bridging `fs-uq`'s scalar closure to batched GPU execution) and piggybacks its falsifier for free on `GATE-FID-01` at M2.

Why 815 rather than 900+? Two honest limits: First, **chaotic contact divergence is real**—on binary pass/fail stair or wet-kitchen runs over 10 seconds, small contact-compliance differences frequently flip one tier from stumble-recover to fall, dropping ρ well below 0.90 (and Claude's optional 1-s segment-mode fallback measures local step bias, *not* full-episode task success). Second, speeding up referee promotion from 19 min to 2 min only compresses the final confirmation stage—it doesn't speed up physics, control, or morphology search. Still, given its tiny footprint, unique non-wrappability, and direct elevation of v0.2's signature Trust Ladder, this is an instant adopt.

---

### `CC-5` — Counterfactual probes: edit the robot at the moment it fails · **Score: 790 / 1000**

- **10× Unlock (1-line):** Moves agent serial turns-to-fix by 3–10× over scalar/heuristic debugging and separates design faults from scene faults in seconds, multiplying strongly with #1/#2.
- **Strongest Technical Objection (1-line):** Applying geometric or structural edits (`foot.radius`, `move-mount`, `add-limb`) via simple state transport at `t_f − 0.5 s` mid-stance teleports bodies into floor/stair interpenetration, generating artificial contact-impulse spikes that can spuriously rescue or crash the continuation.

**Candid Evaluation.**
Claude correctly diagnosed a crucial bottleneck that pure-physics engineers miss: once GPU simulation takes seconds, the AI agent's own 30–120 s serial turn latency dominates wall time. Running 16–32 interventional edits in parallel from Chronicle's T3 failure ring buffer, computing exact 5-factor Shapley coalitions (32 combinations in one cohort batch), and explicitly including scene interventions (`dry → rubber` nosing to separate scene faults from robot faults) gives an AI agent the exact debugging instrument it needs. Requiring the `Estimated` label and hand-off to `eval compare` keeps statistical honesty intact.

Where `CC-5` loses points (and where `COD-5` actually beats it on rigor!): **mid-episode state transport for geometry/topology edits is physically treacherous.** Widening a foot radius by 8 mm or moving a hip mount while the foot is loaded against a stair nosing instantaneously creates an 8 mm penetration, triggering a huge compliant contact kick that has nothing to do with how the edited robot would actually approach that stair! `CC-5` also refuses per-design PPO policies outright (requiring MPC or a family policy to adapt). Restricting mid-episode counterfactual forks cleanly to **non-penetrating physical/card/sensor/scene/solver interventions** (torque/speed limits, actuator delay, friction cards, estimator vs privileged state, `h/2` exploit checks) while validating geometry/mass changes from either pre-contact flight/reset or with explicit clearance projection makes the mechanism airtight.

---

## 3. Detailed Evaluations: Codex's Top 5 (`WIZARD_IDEAS_COD.md`)

### `COD-1` — Compile engineering requirements into morphology search (typed holes + branch-and-bound) · **Score: 640 / 1000**

- **10× Unlock (1-line):** Can cut static discrete-catalog search 10× on simple wheeled/monotone sizing tasks, but unlikely to achieve a general 10× unlock on dynamic legged/hybrid robots where conservative completion bounds go loose.
- **Strongest Technical Objection (1-line):** Enforcing zero false cuts over arbitrary completions of a partial articulated robot tree forces conservative interval/monotonicity bounds so loose on coupled dynamic tasks (stairs, impact torques, balance) that branch-and-bound prunes little beyond cheap leaf filters while adding heavy compiler/verifier complexity.

**Candid Evaluation.**
Intellectually, `COD-1` is the cleanest CS idea in Codex's slate: introducing typed mount holes into `frob-design`, interning partial terms in a canonical arena, tracking outward-rounded interval capability/resource antichains, and letting an agent widen a requirement and resume from surviving summaries is genuinely slick. Codex's M1 falsifier (exhaustive 1,024-leaf catalog with planted non-monotone actuator interactions and zero-false-cut tolerance) is also exemplary.

In real-world robot design, however, the ROI vs complexity balance is poor for a #1 pillar. Building a sound partial-term elaboration semantics, monotonicity proof checker, and checked interval completion algebra across all constructors (`mirror`, `radial`, `closure`, `attach`) is substantial subtle compiler tech debt. Yet the hard bottlenecks for quadrupeds, humanoids, and wheel-legs are *dynamic* (impact torque spikes, swing inertia, slip margins, contact stability)—properties that are famously non-monotone in leg length, mass distribution, and gear ratio (e.g., adding a heavier transmission can increase reflected inertia enough to stabilize or destabilize impact; longer legs reduce stepping frequency while increasing torque). Under Codex's own strict zero-false-cut rule, non-monotone dynamic branches must be left alive—leaving `COD-1` mostly pruning elementary mass-spiral and static-drawbar failures that v0.2's fast 2-second `design check` (A-10/A-11) or cheap static screening already eliminates in seconds. Useful as an optional catalog-sizing filter for wheeled/logistics platforms, not a flagship #1 unlock.

---

### `COD-2` — Transfer controllers across typed edits (SE(3) adjoint transport + role-local residual blocks) · **Score: 595 / 1000**

- **10× Unlock (1-line):** No 10× unlock: on continuous parameter edits standard checkpoint warm-starting already captures most of the gain, while on discontinuous topology edits transfer breaks down and cold-start training remains unsolved.
- **Strongest Technical Objection (1-line):** Explicit frame/role correspondence plus SE(3) adjoint transport adds nontrivial controller-architecture coupling and CSR bookkeeping to accelerate local parameter edits where ordinary warm-starting works, yet Codex itself rates confidence "Low" on the topology/support edits where agents actually suffer.

**Candid Evaluation.**
Using typed-edit lineage (`§12.2`) to carry explicit parent→child frame and semantic-role correspondences—transporting twists by `Ad_g` and wrenches by `Ad*_g` and catching planted sign/ordering errors automatically—is mathematically tidy and avoids silent channel-mismatch bugs during fine-tuning.

As a Top-5 10× lever, though, it falls flat on three counts. First, **no cold-start solution**: before you can transfer a controller to a child edit, you still have to pay 5–30 minutes of RL training to build a donor for every topology family. Second, **thin delta over incumbent warm-starting**: for stationary edits (link length, gear ratio, mount shift), fine-tuning an existing PPO checkpoint in MJX/Warp/Isaac Lab already runs 5–10× faster than cold training without building a custom role-indexed CSR policy transport engine. Third, **breaks on structural invention**: swapping wheels for feet, adding a limb, or changing gait support polygons changes the contact dynamics discontinuously—exactly where `CC-2` (zero-training sampling MPC) works out of the box with zero donor required. Worth keeping only as a small semantic-role manifest check (`COD-18`) during PPO fine-tuning.

---

### `COD-3` — Target referee work at the decision with randomized audits (AIPW confidence sequences) · **Score: 545 / 1000**

- **10× Unlock (1-line):** No: it only accelerates referee promotion (leaving physics, control, and search untouched) and is mathematically dominated by `CC-4`'s coupled MLMC due to inverse-propensity range blowup.
- **Strongest Technical Objection (1-line):** In bounded anytime-valid confidence sequences (`fs-eproc`), dividing residuals by small audit propensities `p_i` (e.g., `p_i = 0.05`) inflates the worst-case observation range `B` by `20×`, wrecking interval width on rare tier reversals and breaking the 10× sample budget.

**Candid Evaluation.**
Codex deserves credit for statistical honesty—it literally ran its own back-of-the-envelope budget (`0.08` referee-pair cost/row allows only `1.25×` more rows for 10× gain) and explicitly warned *"The inverse weights can easily break that budget... Medium-low confidence in 10×."* That honesty exposes the core flaw: **AIPW (`Z_i = m_i + A_i (Y_i − m_i) / p_i`) is the wrong estimator when you control both simulation tiers!** AIPW was invented for observational/bandit settings where you cannot directly allocate independent strata cleanly. Here, we own the Philox counter-based random streams across GPU (`ℓ0`), `batch-cpu` (`ℓ1`), and `referee` (`ℓ2`)! Plain coupled Multilevel Monte Carlo (`CC-4`, already implemented in `fs-uq/src/product_mlmc.rs`) achieves exact unbiased estimation of the referee contrast with `Y_2 − Y_0 ∈ [-1, 1]`—**zero `1/p_i` range explosion**—and requires no delicate propensity-logging/retry-preservation state machine. Between `CC-4` and `COD-3`, `CC-4` wins decisively on math, variance, simplicity, and sibling reuse.

---

### `COD-4` — Compile reusable route feasibility evidence (interval bricks + optimistic/robust route graphs) · **Score: 510 / 1000**

- **10× Unlock (1-line):** No: static geometric passability is <2% of end-to-end robot design time, Drake GCS already competes directly on planar/rigid route regions, and articulated/legged robots force massive abstention.
- **Strongest Technical Objection (1-line):** Conservative rigid swept-hull interval enclosures cannot tightly bound articulated 18–35 DoF legged/hybrid postures (stepping over clutter, ducking, turning limbs, opening doors), causing either huge `unresolved` abstention rates or intractable posture-chart index explosion.

**Candid Evaluation.**
Direct inspection of `frankensim/crates/fs-motion/src/clearance.rs` confirms FrankenSim has rigorous two-sided clearance bounds with explicit additive error budgets (`chart_conversion_m`, `spatial_discretization_m`), and Codex's three-way distinction (`certified clear`, `certified impossible` via optimistic graph disconnect, `unresolved` for occluded scan patches/doors) is mathematically disciplined. Caching clearance via footprint envelope inclusion across near-neighbor wheeled-base edits is neat.

Yet as a Top-5 choice for FrankenRobotics, this is a classic misallocation of complexity. First, **tiny leverage on TTVD**: even without an interval-brick route-certificate compiler, a 2-D GPU SDF dilation (`CC-15`) or fast static check filters impossible wheeled footprints in milliseconds. Second, **narrow morphology coverage**: across the 7 interior archetypes, wheeled bases in static corridors are only one corner—quadrupeds climbing stairs, humanoids pushing commercial-kitchen swing doors, and wheel-leg hybrids ducking around movable office chairs cannot be decided by static rigid swept hulls without either constant `unresolved` abstentions or an expensive high-dimensional C-space atlas. Third, **incumbent overlap**: Drake's IRIS/GCS already solves reusable geometric corridor certificates cleanly. Demote to an optional M3–M4 wheeled-base analysis verb (`A-07` extension), not a Top-5 pillar.

---

### `COD-5` — Rank repairs by actual replay interventions (checked prefix reuse + reset validation) · **Score: 740 / 1000**

- **10× Unlock (1-line):** High-value 3–5× reduction in agent debugging turns, but cannot achieve a 10× end-to-end speedup inside Codex's slate because Codex omitted both heterogeneous GPU batching and fast universal MPC.
- **Strongest Technical Objection (1-line):** Properly refusing prefix reuse for any permanent edit touching mass, inertia, contacts, or controller memory means most real hardware design edits still pay full reset-to-failure replay plus controller resynthesis—erasing the 0.07× branch cost unless backed by fast multi-design GPU batching.

**Candid Evaluation.**
`COD-5` is clearly Codex's best Top-5 idea and outperforms `CC-5` on **physical and causal rigor**. Where `CC-5` glibly transported state across `foot.radius` and `move-mount` edits mid-contact, Codex explicitly catches that defect: it strictly separates **temporary diagnostic checkpoint perturbations** (driver limits, friction cards, command inputs—reusing the exact verified prefix!) from **permanent model edits touching mass/inertia/contacts**, forcing the latter to validate from reset with explicit controller qualification! Its seeded negative-control gate (early symptom distinct from planted cause, plus a mass edit that *must* invalidate prefix reuse and an unchanged prefix that *must* match bit-for-bit) is rock-solid.

Why 740 instead of 840+? Two reasons: First, Codex cut heterogeneous GPU batching (`COD-10`) and omitted fast universal GPU MPC, so `COD-5` has to pay sequential replays and slow controller resynthesis on its reset-validated branches—gutting its wall-clock leverage. Second, without `COD-10`/`CC-1` and typed port cards, single-topology checkpoint replay is partially replicable via a thin wrapper over MuJoCo's restorable `mjData`. Combine `COD-5`'s strict prefix-invalidation soundness rules with `CC-1`/Design-SIMD's 20 ms parallel GPU fork and `CC-2`'s zero-training MPC, and you get the ideal causal debugger.

---

## 4. Scored Non-Top-5 Ideas Worth Highlighting (Hidden Gems & Instructive Cuts)

Evaluating the 6–30 lists exposed **several genuinely high-value ideas that both wizards cut prematurely**, alongside weak/harmful ideas that illustrate the bottom end of the 0–1000 scale.

### 4.1 Notable Non-Top-5 Ideas from Claude (`WIZARD_IDEAS_CC.md` 6–30)

| ID | Idea | Score | 10× Unlock? (1-line) | Strongest Technical Objection (1-line) |
|---|---|---:|---|---|
| `CC-16` | One fused kernel per world for homogeneous training (warp/SIMD-group SRAM) | **835** | **YES — Claude's biggest missed gem!** Keeping 10 substeps + tree-sparse LDLᵀ in SRAM delivers 10–15× physics throughput and makes `CC-2` (MPC) fast *without* unsafe `h = 5 ms` coarsening. | Specializing to one topology trades against arbitrary mixed-DoF ragged cohorts (solvable by Design-SIMD per-world params inside each topology family) and risks register spills if naively unrolled on 29-DoF humanoids. |
| `CC-25` | Design-decision benchmark against B0 and B0⁺ on `trj` | **780** | Not an algorithmic lever itself, but the indispensable falsification instrument that prevents 10× self-deception. | Keeping external Python incumbents (Playground, DIAL-MPC, Newton) pinned and fairly tuned on a heavily loaded `trj` requires disciplined maintenance to avoid straw-man drift. |
| `CC-6` | Searching scene grammars for rare failures (STL robustness + subset sim / `fs-uq`) | **760** | Real 50–100× speedup *for tail-failure estimation* (P ≈ 10⁻⁴) and surfaces worst-case interior fixtures automatically. | `fs-uq/src/product_importance.rs` requires independent normal germs with defensive mixture floor `ε`; mapping discrete/combinatorial interior grammars cleanly onto normal germs without weight degeneracy is non-trivial. |
| `CC-12` | Gait-cycle adjoints for design gradients (IFT on C-06 + Revolve) | **745** | ~25× faster continuous morphology gradients, but Claude unnecessarily handicapped it by restricting to periodic limit cycles. | Fails whenever there is no stable periodic gait (stairs, clutter, door-opening, novel hybrids) and when non-smooth contact switches bias unsmoothed gradients—fixable via short-horizon (H = 16–32) smoothed IFT. |
| `CC-14` | CPU shadow auditor on idle cores (36 Mac / `trj` CPU) | **735** | Zero-wall-clock continuous fidelity data and MLMC pilots; massive unlock on M5 Ultra's 256 GB unified memory. | At 0.2× real time per core for humanoids, a heavily loaded `trj` CPU may stream too few shadows/hour to keep pace with rapid agent design churn. |
| `CC-11`/`24` | Symmetry-canonical global evaluation cache in Chronicle | **720** | ~2–4× swarm compute savings (not 10×), near-zero risk, immediate adopt. | Full canonicalization modulo graph automorphism + port relabeling must be exact or subtle cache collisions silently poison comparisons. |
| `CC-9` | Mac 256 GB as the scene-breadth tier (100+ resident interior packs) | **710** | Real 10× simultaneous scene-breadth unlock vs 24 GB 4090s, directly exploiting M5 Ultra hardware. | Throttled if Metal stays behind `ft-kernel-metal`'s synchronous 31-arg wrapper (`KILL-METAL` risk) instead of native async Metal 4 (`objc2-metal`). |
| `CC-8` | Carving design space with `fs-ivl` intervals / `multi_taylor.rs` & monotone boxes | **655** | ~10–100× faster static grid pruning (cleaner than `COD-1`), but value is capped by static-check predictiveness (`BET-05`). | `multi_taylor.rs` caps at 32 variables and suffers interval wrapping explosion past shallow kinematic/quasi-static compositions. |
| `CC-28` | Granular/deformable MPM terrain at scale | **240** | No: exceeds v1 hardware for 8,192-world batches and adds no design-decision lever (rightly cut). | MPM per-world particle state blows past 24 GB VRAM and slashes throughput by 100×+ when Bekker–Wong suffices for v1 wheels/tracks. |
| `CC-22` | Learned morphology-conditioned dynamics surrogate for screening | **230** | No: unreliable on contact-rich novel morphologies and fully wrappable over incumbents (rightly cut). | Learned contact models hallucinate penetration/slip dynamics precisely on out-of-distribution designs where physics accuracy matters most. |
| `CC-18` | Learned residual correction from fast tier to referee inside conformal bands | **210** | No: actively undermines FrankenRobotics's trust ladder (rightly cut). | Applying an ML band-aid to hide GPU-vs-referee discrepancy masks real solver/contact exploits that the promotion gate exists to expose. |
| `CC-26` | Multi-agent LLM design-debate layer | **190** | No: 18–35% polish completely deliverable by a thin prompt wrapper over any incumbent (rightly cut). | Zero structural connection to physics, algebra, or GPU architecture; belongs in agent orchestration, not the simulator core. |
| `CC-30` | Video-to-splat scenes for RGB policy training | **175** | No: orthogonal rendering sprawl with zero v1 design-decision leverage (rightly cut). | Requires building an entire Gaussian-splat rasterizer while RGB policies are explicitly out of scope until M5. |

### 4.2 Notable Non-Top-5 Ideas from Codex (`WIZARD_IDEAS_COD.md` 6–30)

| ID | Idea | Score | 10× Unlock? (1-line) | Strongest Technical Objection (1-line) |
|---|---|---:|---|---|
| `COD-11` | Optimize a link from measured loads while preserving ports (`fs-topopt`/`fs-topols`/`fs-solid`) | **795** | **YES — Codex's biggest missed gem!** Activating FrankenSim's 290k-line 3-D CutFEM `sdf3` + fatigue + fab + STEP stack eliminates weeks of external CAD/FEA rework; Codex under-scoped it to a single-link tweak and buried it at #11! | Running full 3-D octree SIMP (`fs-topopt::sdf3`) inside a tight inner loop can stall at 30–120 s/link unless scheduled asynchronously on M5 Ultra's 36 cores and paired with fast `beamcol` screening. |
| `COD-6` | Compile gradients per contact regime (`fs-adjoint::ift` + `fs-time::hybrid_sensitivity`) | **775** | Strong 10×+ continuous optimization lever that Codex should have promoted ahead of `COD-3`/`COD-4`. | Direct inspection of `fs-time/src/hybrid_sensitivity.rs` shows it explicitly refuses simultaneous/competing contact events—making smooth convex IFT on C-06 far more robust than transverse guard differentiation for multi-foot locomotion. |
| `COD-10` | Batch topology churn by execution shape | **765** | Essential multi-design GPU substrate that Codex wrongly cut at #10, crippling its own `COD-5` and `COD-2`. | Bucket padding and mask overhead can eat gains unless paired with warp-contiguous model grouping (`CC-1`) or per-world parameter structs. |
| `COD-9` | Use algebraic symmetry to share expensive test cases (joint robot×task×scene orbits) | **745** | Genuine 4–8× evaluation speedup using exact finite-group orbit quotienting—uniquely algebraic and zero approximation error! | Only pays on symmetric scenes/tasks (open terrain, symmetric aisles); cluttered asymmetric interiors collapse the joint orbit to trivial. |
| `COD-8` | Mine rare failures with fixed audit distribution (`fs-uq::product_importance`) | **730** | ~10× narrower rare-event intervals on verified sibling code (`product_importance.rs`), clean M3 reliability upgrade. | Fixed Gaussian importance proposals degrade sharply in high-dimensional scene parameter spaces when failure modes are multimodal. |
| `COD-25` | Local discrete energy balance per port to triage solver failures (`fs-ports` + `fs-phs`) | **715** | Not 10× end-to-end, but a brilliantly cheap, mathematically sharp debugger that localizes numerical/contact faults to the exact port. | Single-precision GPU noise can alias tiny numerical energy residuals unless accumulated in double-float per port. |
| `COD-13` | Turn manufacturing variation into part decisions (`fs-toleralloc` + `fs-asbuilt`) | **710** | Strong physical-realism differentiator using real FrankenSim crates; highest payoff when fed by IFT gradients (`COD-6`) alongside `COD-11`. | First-order tolerance propagation breaks on nonlinear backlash/contact thresholds unless guarded by `fs-toleralloc::robustness_check` at extremes. |
| `COD-27` | Schedule Mac for scene diversity and CUDA for repeated training | **690** | Smart pragmatic ~2× throughput/coverage gain tailored to the exact 256 GB Mac + dual-4090 Linux hardware split. | Cross-machine job coordination over `frobd` adds operational complexity and depends on Metal hitting acceptable throughput (`KILL-METAL`). |
| `COD-14` | Bounded reduced model per locomotion regime (`fs-surrogate` POD/DMD + conformal) | **635** | ~10× cheap screening *inside* verified quasi-periodic regimes, cleanly backed by existing `fs-surrogate`. | High abstention rate whenever an agent edits topology or enters aperiodic interior maneuvers (doors, clutter, stairs). |
| `COD-26` | Certify a small maneuver's recovery region (`fs-sos` + `fs-ivl`) | **470** | Neat formal proof badge on toy reduced-order models, zero leverage on full-body robot design decisions. | SOS polynomial degree and interval enclosure error explode past ~4–6 state variables, excluding real multi-contact articulated robots. |
| `COD-15` | Synthesize skill sequences over contact modes with guard/dwell tables | **450** | No: brittle hybrid-automaton engineering that loses to GPU sampling MPC (`CC-2`) on novel hybrids and messy stairs. | Manually authoring or maintaining sound guard/dwell tables across arbitrary morphologies reintroduces the exact brittle `BET-06` trap. |
| `COD-7` | Proof-driven time stepping in quiet windows (`fs-ivl` + `fs-motion`) | **380** | No: theoretical 3× stage cut that actively backfires on GPU throughput. | Per-world variable time-stepping / stage skipping shatters GPU warp lockstep and static CUDA launch graphs (which cost 3–4× throughput when broken, §20.5). |
| `COD-29` | Differentiate rigid locomotion end-to-end across all contact changes | **310** | No: well-known gradient-explosion/bias trap over long contact horizons (rightly parked by Codex). | Branch reversals across multiple simultaneous contact mode changes produce misleading or exploding long-horizon gradients. |
| `COD-28` | Improve agent decisions through forecast feedback | **260** | No: tiny ~20% tweak on v0.2's already-planned prediction ledger (§22.7) and purely wrappable. | Calibration adjustments over noisy small-sample agent briefs rarely transfer across novel tasks. |
| `COD-30` | Full deformable soil MPM/FEM in every design world | **220** | No: hardware-busting scope creep (rightly parked by Codex). | Coupled GPU MPM/FEM + articulated robots exceeds 24 GB VRAM and destroys fast-lane rollout budgets; Newton already owns MPM/VBD. |

---

## 5. Bottom-Line Synthesis: Best Ideas to Adopt From Both Opponents

If synthesizing the strongest, non-overlapping upgrades from `COD` and `CC` into FrankenRobotics v0.2:
1. **Adopt immediately (≥790):** `CC-2` (nested batched sampling MPC with planner-gap ledger, upgraded with fast kernels so it doesn't need risky `h = 5 ms` coarsening), `CC-1` (ragged cohorts turning C-01 screw bases into per-model data), `CC-16` (Claude's cut SRAM-resident fused kernel—actually a top-tier multiplier!), `CC-4` (paired MLMC referee promotion via existing `fs-uq`, decisively beating `COD-3`), `COD-11` (Codex's cut 3-D CutFEM link synthesis via existing `fs-topopt`/`fs-solid`, expanded into a full manufacturable hardware loop with `COD-13`), and **`CC-5` + `COD-5` fused** (parallel GPU counterfactual probes with Shapley coalitions *plus* Codex's strict prefix-invalidation soundness and reset validation for mass/geometry edits).
2. **Adopt as high-ROI Phase-2/M3–M4 instruments (720–780):** Codex's §0 censored benchmark protocol paired with `CC-25` (the B0/B0⁺ bake-off), `COD-6` (reframed around smooth short-horizon IFT on C-06 rather than brittle guard isolation), `CC-6`/`COD-8` (rare-failure importance sampling via `fs-uq::product_importance`), `COD-9` (joint robot×task×scene symmetry orbit sharing), `CC-14` (idle-CPU shadow auditor), `CC-11`/`24` (symmetry-canonical eval cache), and `COD-25` (per-port discrete energy-balance fault triage).
3. **Reject or defer past v1 (<680):** `CC-3` (defer until GPU autograd lands in FrankenTorch; too much custom DL-kernel debt + OOD bias), `COD-1` (too complex for weak dynamic pruning), `COD-2` (thin over warm-start RL), `COD-3` (dominated by `CC-4` MLMC), `COD-4` (too narrow / high abstention on legged interiors), `COD-7` (breaks GPU lockstep/graphs), and `CC-18`/`22`/`26`/`28`/`30` / `COD-28`/`29`/`30`.
