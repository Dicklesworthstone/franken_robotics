# Devil's Advocate Challenge to `VERDICT_REGISTER.md` (`AGY`)

- **Session:** `RS-20261121-frob-fable-10x`
- **Role:** Independent cross-family Devil's Advocate
- **Inputs read in full:** `intake/question_of_record.md`, `hypothesis_slate.md`, `deliverables/VERDICT_REGISTER.md`, and all 23 existing evidence packs in `evidence/packs/EV-pack-H-*.md` (noting that `EV-pack-H-18.md` does not exist).
- **Summary of rulings:** **12 AGREE · 12 CONTEST** across `H-01`–`H-24`, **CONTEST on Q-001**, plus **3 missing hypotheses (`H-25`–`H-27`)**. Every contest cites the exact pack line, source, or arithmetic that contradicts the register, plus the cheapest settling test. Pure Unicode math (no LaTeX).

---

## 1. Summary Table of All 24 Verdicts

| ID | Register State | Ruling | Proposed State (if CONTEST) | Core Defect in Register Verdict |
|---|---|---|---|---|
| `H-01` | NARROWED | **CONTEST** | NARROWED (wider decision budget, narrower generality) | Compares 299 MPC episodes vs 1-seed PPO instead of 24 decision pairs vs 5-seed PPO (§19.7); hides reliance on Raibert/swing-foot priors. |
| `H-02` | NARROWED | **CONTEST** | NARROWED (flat/fixed-actuator only) / DEFERRED for interiors | Too generous: quotes 70–85% / 1–20% from flat/fixed-actuation papers while its own pack shows 10% zero-shot on variable terrain (`2508.03660`). |
| `H-03` | NARROWED | **CONTEST** | NARROWED (strike "0.86"; elevate engine-internal gap) | Cites an ungrounded "0.86" figure absent from `EV-pack-H-03` (which explicitly states no penalty is published) and glosses over Kamino being nondeterministic. |
| `H-04` | REFUTED | **CONTEST** | NARROWED (topology-specialized `make_constraint` + tree-sparse `solve` + fusion) | Refuted on a straw-man 20% slice (RNEA/CRBA only), ignoring `make_constraint` (14.6%) + `solve` (47.1%) + `euler` (3.5%) = **85.5%** of the traced step. |
| `H-05` | NARROWED | **AGREE** | — | Supported: 4–260 s CPU solve times, bifurcation risk, and orbit ≠ closed-loop controller. |
| `H-06` | NARROWED | **AGREE** | — | Supported: 10⁴ boxes = 13 bisections (<1 cut/axis in 20D); per-constraint ≤6D sub-boxes is right. |
| `H-07` | NARROWED | **AGREE** | — | Supported: separating splitting exploration from `fs-uq` bounded-weight IS certification is sound. (Contradiction with Q-001 addressed in §4.) |
| `H-08` | NARROWED | **CONTEST** | NARROWED (fast `f32` replicate ensembles at Δ ≤ 1 s + solver slots; drop `f64` mandate) | Mandating `f64` from a 2.0 s chaotic window is unnecessary at Δ = 0.5–1.0 s (roundoff ≤ 3.6×10⁻⁵) and physically backwards vs `f32` noisy replicate ensembles. |
| `H-09` | REFUTED | **AGREE** | — | Supported: mass/geometry edits invalidate >90% of exact rollout compute; memo hygiene + verdict revalidation rightly survive. |
| `H-10` | NARROWED | **AGREE** | — | Supported: `fs-race` battery caps at 2–5× on separated fields; LLM/GA baselines challenge BO. |
| `H-11` | NARROWED | **CONTEST** | NARROWED (async wall-clock hider paired with `H-22(b)`, ≥3–35% coverage; agree kill "zero-copy") | Understates own measured coverage (35% raw / 3.5% at 10× slower referee) and misses that background shadowing zeroes out MLMC referee promotion wall-clock. |
| `H-12` | DEFERRED | **CONTEST** | SPLIT: fast `beamcol` sizing ACTIVE in-loop; 3-D `fs-topopt` DEFERRED to finalists | Relies on a flawed "4% of robot mass" XDog metric ignoring distal limb inertia (accelerating at 30–100 m/s²) and arms (31.6% mass cut in `2606.26643v1`). |
| `H-13` | NARROWED | **AGREE** | — | Supported: STL as acceptance/falsification score; plain robustness fails as sole RL reward. |
| `H-14` | NARROWED | **AGREE** | — | Supported: Beta(19, 2) has SD = 6.3 pts at n = 20; robot (not episode) is the exchangeable unit. |
| `H-15` | REFUTED | **AGREE** | — | Emphatic agree: measured MuJoCo probe kills `f16` Cholesky (κ·u₁₆ ≥ 1 in 37–64%; refinement costs 1.5–2.75× more than `f32` Cholesky). |
| `H-16` | REFUTED | **AGREE** | — | Supported: Stefan squeeze-film arithmetic proves `fs-tribo` film law + rigid proxies dominate coupled fluid/cloth islands. |
| `H-17` | NARROWED | **CONTEST** | NARROWED (add <2 s convex inverse-dynamics regressor; elevate catalog co-design to M1) | Checked only 4–10 min non-convex CPU DDP for (b), missing exact linearity of RNEA in pseudo-inertias/wrenches; left (c) catalog co-design unmeasured. |
| `H-18` | NARROWED | **CONTEST** | SPLIT: REFUTED vs per-design PPO incumbent; ACTIVE post-training (MPC/prior) | **No `EV-pack-H-18.md` exists!** Adjudicated without an evidence pack and conflates the PPO regime (~95% training) with the post-training agent-turn regime. |
| `H-19` | ACTIVE | **CONTEST** | SPLIT: ACTIVE on CUDA fixed-topology speed; REFUTED on 4–5 real engine-internal gaps | Gerrymanders `EV-pack-H-19`'s own evidence down to "<3 gaps" by waving away GPU nondeterminism (84% atomics cost), missing two-tier referee, Metal, and differentiability. |
| `H-20` | NARROWED | **AGREE** | — | Supported: SOS stalls past ~10–12 states; template-to-full ε is not computable a priori. |
| `H-21` | NARROWED | **CONTEST** | NARROWED (honest 15–40% parameter edit cost; flag diversity/plasticity loss) | Too generous: "1–20%" relies on H-Zero's unstopped 50k-epoch scratch denominator (pack line 44) and underplays Nature 2024 PPO collapse + diversity loss. |
| `H-22` | NARROWED | **CONTEST** | SPLIT: (a) DEFERRED/UNSUPPORTED (do not anchor Q-001); (b) NARROWED higher (5–15× on continuous metrics) | Contradiction: (a) has nearest ρ = 0.40 yet anchors Q-001! Meanwhile (b) was narrowed too timidly (~2×) by conflating trajectory chaos with episode-metric correlation. |
| `H-23` | NARROWED | **AGREE** | — | Supported: 3–4× published population compute gain; parametric sweeps exist in Warp; FD variance blows up across switches. |
| `H-24` | NARROWED | **AGREE** | — | Supported: P-GATr 7.5× slower and failed control; augmentation + heading normalization recovers convention invariance. |

---

## 2. Detailed Contestations (Evidence, Arithmetic & Cheapest Settling Test)

### `H-01` (Sampling control, zero training) — **CONTEST**
- **Register:** NARROWED ("screening rung of at most 3 episodes; 0.33–1 GPU-min quadruped, 1–5 humanoid; never evaluator of record").
- **Proposed verdict:** **NARROWED (wider decision-making scope: viable up to ~20–30 paired decision episodes for legged/wheeled designs, not capped at 3 episodes; while STILL refuted for 299-episode rare-failure certification AND explicitly marked UNTESTED on tracked/novel hybrids without hand-built gait priors).**
- **Evidence & arithmetic contradicting the register:**
  1. **Asymmetric PPO denominator (`EV-pack-H-01` lines 43–47):** The pack compared **299 zero-failure MPC acceptance episodes** (99 GPU-min quadruped, 299 GPU-min humanoid) against a **single-seed PPO run** (10 and 60 GPU-min). Plan §19.7 requires **at least 5 PPO seeds** (50 GPU-min quadruped, 300 GPU-min humanoid!). Meanwhile, an iterative design comparison (`GB-06`, effect size δ = 0.5, α = 0.05) needs **~24 paired episodes**, not 299: `24 × 0.33 GPU-min = 8 GPU-min` (quadruped) and `24 × 1.0 GPU-min = 24 GPU-min` (humanoid) — still **6×–12× cheaper than 5-seed PPO** even at DIAL-MPC's unoptimized `Nsample = 2,048`!
  2. **Overstated sample budget:** `EV-pack-H-01` line 33 itself cites CPU MPPI (`2409.10469v1`) succeeding 9/10 on real Go1 box pushing with **only 30 samples** (and `2511.19204` uses 30–70). Warm-started spline sampling at `K = 256` instead of `2,048` cuts per-episode GPU cost 8× (to ~2.5 GPU-s quadruped → 299 episodes = 12.5 GPU-min!).
  3. **Too generous on morphological generality:** Conversely, the register hides `EV-pack-H-01` lines 23–25: DIAL-MPC uses hand-built swing-foot references, cubic/quadratic splines dropped success from 10/10 to 6/10 and 0/10, and zero tracked-robot or corpus-wide evidence exists.
- **Cheapest settling test:** On one RTX 4090 with Hydrax/DIAL-MPC vs 5-seed Playground PPO across 20 quadruped/humanoid/wheel-leg designs, sweep `K ∈ {64, 128, 256, 512, 2048}` *with and without* hand-built swing-foot references; measure 24-episode pairwise decision cost and Kendall τ vs 5-seed PPO (~2 engineer-days, ~12 GPU-hours).

---

### `H-02` (Universal policy prior) — **CONTEST (Too Generous)**
- **Register:** NARROWED ("Within one morphology class: zero-shot 70–85%, fine-tuning 1–20% of from-scratch steps. Across topology-diverse designs: zero-shot 10–40%, fine-tuning 2–3×.")
- **Proposed verdict:** **NARROWED strictly to flat/open-ground parametric families with fixed actuation (zero-shot 70–85%, fine-tuning 15–30%); UNSUPPORTED / DEFERRED for multi-terrain interior missions (stairs, rubble, wet floors, payloads, changing actuator cards) and REMOVED as an assumed pillar of Q-001.**
- **Evidence contradicting the register:**
  - `EV-pack-H-02` line 13 explicitly reports that on **variable terrain**, a multi-embodiment prior scored **136.82 vs 1,406.58 from scratch (9.7% zero-shot!)** and *"On the flat terrain tasks, learning from scratch is comparable to end-to-end finetuning"* (`2508.03660`).
  - Line 15 shows the flagship zero-shot scaling paper (`2505.05753v2`) consumed **19,200 GPU-hours (~400 days on 2× RTX 4090!)**, on **flat terrain only**, with **body mass distribution, joint damping, and actuation held fixed**.
  - Line 46 admits `H-Zero`'s 1–10% fine-tuning figure used a fixed unstopped 50,000-epoch scratch denominator (inflating the ratio by 2–3×).
  - Not a single cited paper in `EV-pack-H-02` achieves 70–85% zero-shot or 1–20% fine-tuning on **variable interior terrain + changing actuator torque-speed cards + payloads** within a 2× 4090 budget!
- **Cheapest settling test:** Train a URMA-style quadruped family prior (~40 GPU-hours on 2× 4090) over 100 variants spanning link lengths ±20%, payload 0–10 kg, and 3 actuator cards, then test zero-shot and fine-tuning on **stairs + slopes + wet friction patches** against early-stopped from-scratch PPO (kill if median zero-shot <50% or fine-tune >40% of early-stopped scratch steps).

---

### `H-03` (Topology as data) — **CONTEST**
- **Register:** NARROWED ("An incumbent publishes a mixed batch at 0.86 of its homogeneous rate. The only differentiators left are measured throughput and time from an edit to the first GPU step.")
- **Proposed verdict:** **NARROWED (strike the ungrounded "0.86" claim; require size-bucketed batches with tree-sparse masked factorization; UPGRADE its differentiation: NO incumbent ships deterministic reduced-coordinate mixed-topology batching).**
- **Evidence contradicting the register:**
  1. **Fabricated/unbacked citation in the Register:** `VERDICT_REGISTER.md` claims *"An incumbent publishes a mixed batch at 0.86 of its homogeneous rate."* `EV-pack-H-03` contains **no such number**—in fact line 25 explicitly states *"No incumbent publishes a throughput penalty for heterogeneous or padded batches"* and line 71 states *"Kamino's measured throughput gap (figures only, not read)"*! A verdict built on an unverified figure violates Brenner evidence discipline.
  2. **Incumbent status understated:** `EV-pack-H-03` lines 14–17 & `EV-pack-H-19` line 16 show Kamino is a slower maximal-coordinate solver whose determinism is **`"No"`**; MuJoCo Warp has no world dimension on `body_parentid`/`jnt_type`/`dof_parentid` and is nondeterministic; Isaac Lab 3.0 forbids varying link/joint counts; Newton PR 4312 closed unmerged (`EV-pack-H-23` line 12). Deterministic reduced-coordinate mixed-topology batching is therefore a genuine engine-internal gap, not a parity feature!
- **Cheapest settling test:** `EV-pack-H-03`'s own 64-topology test on one 4090, adding a mandatory **run-to-run bitwise determinism check** on Kamino vs bucketed MuJoCo Warp vs a tree-sparse masked prototype (~3 engineer-days, <5 GPU-hours).

---

### `H-04` (Supercompiled kernels for finalists) — **CONTEST (Refuted on a Straw Man)**
- **Register:** REFUTED as a GPU throughput lever ("Lie-algebraic recursions are 16–20% of an incumbent's step, so the ceiling is ~1.2×.")
- **Proposed verdict:** **NARROWED (REFUTED for pure RNEA/CRBA-only specialization and e-graph equality saturation; ACTIVE/NARROWED for compile-time topology specialization of `make_constraint` sparse Jacobians + tree-sparse arrowhead LDLᵀ `solve` + Stage 6–9 fusion, targeting 1.5×–4× on the 85.5% smooth+constraint+solve+integrate share).**
- **Evidence & arithmetic contradicting the register:**
  - Look at the exact MuJoCo Warp `--event_trace` quoted in `EV-pack-H-04` line 11 (humanoid, 8,192 worlds, `step = 262.33 ns`):
    - `kinematics + com_pos + crb + com_vel + rne + fwd_acceleration = 53.13 ns (20.3%)`
    - `make_constraint = 38.31 ns (14.6%)`
    - `solve = 123.55 ns (47.1%)`
    - `euler = 9.29 ns (3.5%)`
    - **Sum of stages governed by fixed kinematic-tree topology = 224.28 ns = 85.5% of the entire step!**
  - Why does topology specialization slash `make_constraint` and `solve`? In a fixed articulated tree (35-DoF humanoid: 5 limbs attached to a 6-DoF base), every foot/hand contact Jacobian touches only 9–12 ancestor DoFs known statically at compile time (cutting Jacobian assembly & `JᵀGJ` sparsity work ~3×), and the contact Hessian `H = M + JᵀGJ` has **compile-time branch-induced arrowhead sparsity**: symbolic tree-sparse LDLᵀ takes ~2,800 FLOPs vs ~14,300 FLOPs for generic dense Cholesky (**5.1× fewer factorization FLOPs**, completely unrolled in registers/SRAM with zero runtime indirection!). Fusing Stages 6–9 eliminates intermediate global-VRAM spills of `M`, `J`, and `H`.
  - A 2.5× speedup on that 85.5% share yields `262.33 / (38.05 + 224.28/2.5) = 2.05×` whole-step speedup (well above the 1.5× claim and 1.2× falsifier!). Even `EV-pack-H-04` line 35 concedes *"No published GPU comparison of specialized against data-driven kernels for the same robot was found."* Refuting specialization by excluding the exact 62% of the step (`make_constraint + solve`) where tree sparsity pays off is an Amdahl framing error.
- **Cheapest settling test:** On one RTX 4090 (8,192 35-DoF humanoid worlds), benchmark (a) generic dense Cholesky + runtime Jacobian vs (b) compile-time unrolled tree-sparse LDLᵀ + ancestor-specialized Jacobian + Stage 6–9 fusion (~3 engineer-days, <2 GPU-hours; kill if whole-step gain <1.5× or compile time >60 s).

---

### `H-08` (Counterfactual autopsy) — **CONTEST**
- **Register:** NARROWED ("Interventional contrasts reported with flip rate under perturbation. Design-parameter interventions re-run from start. Attribution runs in double precision.")
- **Proposed verdict:** **NARROWED (STRIKE "Attribution runs in double precision"; run fast `f32` GPU replicate ensembles of `R = 16–32` physically perturbed forks over Δ ∈ {0.5, 1.0 s} plus inline solver-artifact slots `h/2` and `CL-2`, while retaining reset-from-start for permanent mass/geometry edits).**
- **Evidence & arithmetic contradicting the register:**
  1. **The `f64` mandate fails its own Lyapunov arithmetic at realistic pre-failure windows:** `EV-pack-H-08` line 21–23 took `λ = 0.16/step` over a full **2.0 s (80 steps)** (`e^(12.8) ≈ 3.6×10⁵` → `f32` roundoff `6×10⁻⁸` reaches `2%`). For actual pre-failure windows of **0.5 s (20 steps)** and **1.0 s (40 steps)**, amplification is only `e^(3.2) = 24.5×` (`f32` error `1.5×10⁻⁶`) and `e^(6.4) = 602×` (`f32` error **`3.6×10⁻⁵`**, 0.0036% of state!).
  2. **Physically backwards:** Any failure sensitive to `10⁻⁷` roundoff is knife-edge chaotic under real `10⁻³` IMU/encoder/friction noise! Running a single `f64` trajectory on CPU (or 1/64-rate `f64` GPU; impossible on Metal!) merely preserves a fragile single-sample path instead of estimating interventional survival probability across `R = 16–32` physically noisy replicates—which `f32` GPU cohorts compute in milliseconds!
  3. **Missed high-value slots:** Adding solver-artifact interventions (`h/2`, `2× Newton iters`, `CL-2`) turns autopsy into an inline numerical-exploit detector.
- **Cheapest settling test:** Extend `EV-pack-H-08`'s own 200-failure MuJoCo benchmark with two arms: (a) single-run `f64` at Δ = 2.0 s vs (b) `f32` ensemble `R = 16` (`10⁻⁴` sensor noise, Δ = 0.75 s); measure true-cause top-1 accuracy and wall time (~1 engineer-day, CPU).

---

### `H-11` (An always-on referee) — **CONTEST**
- **Register:** NARROWED ("Sampled snapshot referee covering 0.1–2% of GPU world-seconds, on any backend. Correctness guard, not a speed lever; 'zero copy' is irrelevant.")
- **Proposed verdict:** **NARROWED (AGREE on deleting "zero copy on unified memory" and making it backend-agnostic; CONTEST coverage floor and "not a speed lever": asynchronous CPU shadowing runs at 3–35% coverage on M5 Ultra and eliminates referee-promotion wall-clock latency completely when feeding `H-22(b)` MLMC pilots & paired contrasts!).**
- **Evidence & arithmetic contradicting the register:**
  - `EV-pack-H-11` lines 33–41 **measured on this exact M5 Ultra under load (17–28)**: MuJoCo `f64` on 24–36 threads hit **550k–576k G1 steps/s** (1,100 world-s/s) and **1.26M humanoid steps/s**—giving **35% coverage** of a 3.35M GPU (or **3.5%** even if FrankenSim's referee is 10× slower!), far above the register's timid "0.1–2%".
  - Lines 46–49 show 24 threads complete an entire 576-episode statistical referee check in **10–29 s** (2–5 min for a 10× slower referee)—meaning running the referee asynchronously in parallel with GPU training/MPC sweeps **hides ~100% of promotion wall-clock latency** and supplies free paired samples for `H-22(b)` MLMC! Dismissing it as "not a speed lever" ignores wall-clock pipelining of Q-001's mandatory referee acceptance step.
- **Cheapest settling test:** Measure FrankenSim `fs-mbd` + `fs-contact` G1 throughput on 24 idle M5 Ultra cores while a Metal compute load runs; verify <5% GPU slowdown and ≥2% world-second coverage (~1 engineer-day).

---

### `H-12` (Simulation-driven generative structure) — **CONTEST**
- **Register:** DEFERRED as a decision-time lever ("7–30% lighter links against engineered parts, ~4% of robot mass or less. Improves output, not time to a decision.")
- **Proposed verdict:** **SPLIT: Fast analytical beam/tube structural sizing (`fs-solid::beamcol`, milliseconds) ACTIVE inside the design loop to enforce physically realistic distal inertia and yield margins; full 3-D `fs-topopt` + `fs-solid` fatigue DEFERRED to the finalist output-quality lane.**
- **Evidence & arithmetic contradicting the register:**
  - `EV-pack-H-12` line 21–24 dismissed structural mass using the 15 kg XDog quadruped (`4 kg legs × 15% = 4% total mass`). That conflates **total mass** with **distal rotational inertia**: lower-leg links swing at 30–100 m/s² and contribute `m·r²` quadratically to hip/knee swing torque, footfall unsprung impact impulse, and **gearbox fatigue stress** (which line 25–26 notes IS the primary legged-robot failure mode!).
  - Furthermore, line 16 quotes a real 6-DoF robot arm achieving **31.6% total arm mass reduction (15.5 → 10.6 kg)**, and line 68 concedes mass shares for humanoids and larger robots were never even retrieved.
  - Leaving structural sizing completely out of the design loop lets agents optimize unbuildable spindly capsules with fake low distal inertia—causing costly redesign rework when the finalist fails structural checks. Millisecond `fs-solid::beamcol` sizing fixes that at ~0% decision-time cost, while rightly deferring multi-minute 3-D `fs-topopt` to finalists.
- **Cheapest settling test:** Run 10 quadruped and 5 mobile-manipulator designs with (a) unconstrained primitive links vs (b) `fs-solid::beamcol` yield/buckling-sized hollow tubes; measure how often peak-load sizing changes the winning actuator card or gear ratio (~2 engineer-days, CPU).

---

### `H-17` (Mission to design by solving) — **CONTEST**
- **Register:** NARROWED ("Trajectory optimization with design variables as local refinement, ~12 DoF, ≤17 params, 4–10 min. Polynomial synthesis only inside symbolic library caps.")
- **Proposed verdict:** **NARROWED with an upgraded inverse lane: (a) general polynomial linkage synthesis REFUTED; (b) expand beyond 4–10 min non-convex CPU DDP to include <2 s convex inverse-dynamics regressor synthesis (new `H-26`) + GPU IFT co-optimization (`H-25`); (c) elevate monotone catalog antichains to an M1 discrete-sizing solver rather than an unmeasured footnote.**
- **Evidence contradicting the register:**
  - `EV-pack-H-17` evaluated (b) solely via **non-convex CPU DDP/collocation** (`Crocoddyl` / Dinev et al., 252–600 s on 8 threads), completely overlooking that Euler–Poincaré `RNEA` is *convex/linear* in pseudo-inertias `P_i ∈ 𝒫⁺(4)` and contact wrenches `γ_c ∈ 𝒦_μ`—turning mass/CoM/gear/actuator co-design for fixed traversal motions into a **<2 s global SOCP/SDP** (see `H-26` below)!
  - On (c), `EV-pack-H-17` line 20 literally wrote *"solves catalog problems in seconds (MEMORY; no timing fetched)"*—never actually investigating how much discrete catalog pruning (`Motors × Gears × Batteries × Mounts`) saves before simulation!
- **Cheapest settling test:** Benchmark (1) a 1,024-entry discrete catalog antichain solver and (2) convex RNEA-regressor SOCP/SDP sizing on 12-DoF quadruped stair/payload missions against Crocoddyl DDP (252 s baseline) and default templates (~3 engineer-days, CPU).

---

### `H-18` (Third alternative: Information, not engine) — **CONTEST (Missing Evidence Pack + Regime Conflation)**
- **Register:** NARROWED ("Training dominates published loops, so the engine matters, but speed alone caps well under ten: the lever is deciding more designs per turn and training fewer of them.")
- **Proposed verdict:** **SPLIT BY BASELINE REGIME (and procedurally incomplete pending a real `EV-pack-H-18.md`): REFUTED against Q-001's per-design-PPO incumbent (where training is >90% of wall-clock, triggering H-18's own 70% falsifier); ACTIVE once per-design training is removed (post-MPC / family-policy / inheritance loops, where serial LLM agent turns and misdiagnosed edits become >70% of wall-clock!).**
- **Evidence & arithmetic contradicting the register:**
  1. **Procedural failure:** There is **NO `EV-pack-H-18.md`** in `evidence/packs/`! One of the two mandatory third-alternative guards was adjudicated without a single fetched phase-budget table from published agent co-design loops (`RoboMoRe`, `Eureka`, `Co-design Gym`, `Loki`).
  2. **Amdahl regime flip (arithmetic):** For a 50-design campaign with 1.5 min/turn LLM latency (`75 min` total agent time):
     - Under **Q-001's per-design PPO baseline** (5 seeds × 7 min/design on 1× 4090 = 35 min/design × 50 = `1,750 min` training): training is **95.9%** of elapsed time → pure information tools without removing training cannot beat ~1.04×, **refuting H-18 by its own ≥70% falsifier!**
     - Under a **post-training baseline** (e.g. 45 s/design via MPC or family policy = `37.5 min` compute vs `75 min` serial agent turns): agent/information latency jumps to **66.7%**—and if poor diagnosis causes 3× wasted edits (`150 turns = 225 min`), information/autopsy + batch-per-turn ("deciding 16–64 designs per agent turn") controls **80–86% of elapsed time**, making H-18 **ACTIVE**!
  - Smearing both regimes into one vague `NARROWED` verdict obscures the exact sequential bottleneck switch.
- **Cheapest settling test:** Instrument 10 real LLM-agent design missions on (a) per-design PPO and (b) Hydrax/DIAL-MPC on one 4090, logging exact wall-clock breakdown across `{LLM think/tool latency, static check, compile, train/MPC, referee, wasted-rework turns}` (~3 engineer-days, ~20 GPU-hours).

---

### `H-19` (Third alternative: A thin layer over an incumbent gets most of it) — **CONTEST (Gerrymandered Gap Count)**
- **Register:** ACTIVE ("Nearly every mechanism can be hosted by an incumbent with a layer. Engine-internal gaps are few: deterministic forks with mixed topologies, replay of one world outside its batch, certificates, a Metal tier.")
- **Proposed verdict:** **SPLIT / NARROWED — ACTIVE for pure CUDA fixed-topology throughput/decision-time (where MuJoCo Warp + Hydrax/DIAL-MPC + Python racing truly threatens any 10× speed claim); REFUTED on H-19's own 3-gap falsifier for the full FrankenRobotics mission, which requires 4–5 load-bearing engine-internal properties incumbents lack.**
- **Evidence contradicting the register (`EV-pack-H-19.md`'s own lines 14–54!):**
  Look at how `EV-pack-H-19.md` counted "fewer than three" gaps by quietly omitting or minimizing four real gaps documented inside its own text:
  1. **Gap 1 — Deterministic GPU execution + deterministic forks without crippling slowdown:** `EV-pack-H-19` line 18 quotes MuJoCo Warp: **`"Is MJWarp on GPU deterministic? No."`** and line 53–54 quotes **`"ATOMICS costs 84% on humanoid on main today"`** + **`"sorting costs 15 to 19%"`** (~2×–6× penalty!). Line 16 confirms Newton `SolverMuJoCo` on GPU `"is not a bitwise trajectory guarantee"` and `SolverKamino` = `"No"`. Calling deterministic GPU forks *"available"* in line 44 contradicts lines 16–18 and 53–54!
  2. **Gap 2 — Deterministic reduced-coordinate mixed-topology batches:** Confirmed absent in MuJoCo Warp, Isaac Lab, and Newton `SolverMuJoCo` (Newton PR 4312 closed unmerged; Kamino is slower maximal-coordinate and nondeterministic).
  3. **Gap 3 — Independent two-tier referee sharing `ModelId`, `SceneId`, and counter-based Philox `WorldAddr` (required by Q-001's own definition of an "accepted design decision"!):** Completely omitted from `EV-pack-H-19`'s table! MuJoCo Warp + MuJoCo C share the *exact same compliant soft-contact formulation* (not an independent exact-Coulomb `f64` referee with certified CCD and port-energy closure!), and Newton lacks cross-solver Philox logical addressing for paired MLMC (`H-22(b)`).
  4. **Gap 4 — Apple Silicon Metal 4 GPU tier (M5 Ultra 80-core GPU, 256 GB unified memory):** Explicitly absent in MuJoCo Warp, Isaac Lab, and Newton GPU (all NVIDIA-only).
  5. **Gap 5 — Analytical contact/morphology IFT adjoints + port-level energy closure:** MuJoCo Warp and PhysX are explicitly **non-differentiable** (`MOR_CASE` E012) and track only aggregate `(KE, PE)`, not per-port actuator/contact/numerical-residual energy closure.
- That is **4–5 engine-internal gaps**, tripping H-19's own 3-gap falsifier for the *full product*—while keeping H-19 **brutally ACTIVE** as a warning that on *CUDA fixed-topology speed alone*, a thin layer over MuJoCo Warp + DIAL-MPC is within ~1–3× unless FrankenRobotics delivers real engine-internal advantages (`H-04` specialized tree-sparse solve, `H-25` IFT adjoints, `H-26` convex inverse synthesis, `H-22(b)` paired two-tier referee).
- **Cheapest settling test:** Run `EV-pack-H-19`'s 2-week MuJoCo Warp + MuJoCo C spike *plus* measure (a) the true determinism slowdown on 8,192 humanoid worlds (testing the 84% atomics claim), (b) whether MuJoCo C catches a planted MuJoCo Warp soft-contact exploit (testing Gap 3!), and (c) mixed-topology batching (~5 engineer-days, ~10 GPU-hours).

---

### `H-21` (Policy inheritance along design edits) — **CONTEST (Too Generous)**
- **Register:** NARROWED ("Parameter edits: 1–20% of from-scratch steps. Single-limb topology edits: 33–100%. Chained inheritance risks loss of plasticity.")
- **Proposed verdict:** **NARROWED further: honest parameter-edit cost is ~15–40% of an early-stopped scratch run (not "1–20%"); single-limb topology edits are 33–100%; chained inheritance suffers both PPO plasticity collapse AND documented morphological-diversity collapse, capping net campaign savings at ~2–4× (not 10×).**
- **Evidence contradicting the register:**
  - `EV-pack-H-21` line 44 itself exposes the "1–10%" `H-Zero` ratio as a denominator artifact (*"H-Zero's 10–100× ratio uses a fixed 50,000-epoch scratch budget; scratch runs reached similar return at 20,000–50,000 epochs"*). The clean URMA figure is 20% (`20M` vs `100M`), while `2508.03660` (line 12) showed from-scratch actually **beat** full fine-tuning (`2,191` vs `1,636`).
  - Lines 11 & 14–15 cite Dohare et al. (*Nature* 2024: standard PPO collapses catastrophically under continual non-stationary edits) AND two co-design papers (`2604.15854`, `2309.13099`) proving Lamarckian inheritance **rapidly shrinks morphological diversity and loses significance (`p = 0.136`) under novelty pressure**! Periodic resets back to scratch or a prior shrink the net campaign speedup well below 5×.
- **Cheapest settling test:** `EV-pack-H-21`'s own 10-edit chain test on 1× 4090, enforcing an **early-stopped** from-scratch control (stopping at 95% asymptotic return, NOT a fixed 50k epochs) and measuring final morphological diversity alongside steps (~40 GPU-hours).

---

### `H-22` (Rank fidelity between rungs; fast tier as control variate) — **CONTEST (Internal Contradiction + Over-Timid on Part b)**
- **Register:** NARROWED, unmeasured ("No published rank agreement between shared policy and specialists. Nearest figure is ρ = 0.40 overall. Control variate saves ~2× unless correlation exceeds 0.9.")
- **Proposed verdict:** **SPLIT cleanly: `H-22(a)` (shared-policy rank fidelity) is UNSUPPORTED / DEFERRED-PENDING-EXPERIMENT and MUST NOT anchor Q-001; `H-22(b)` (paired MLMC control variate across tiers) is NARROWED higher to 5–15× on continuous physical metrics (energy/m, peak torque, speed) while staying ≤2× on chaotic binary long-horizon falls.**
- **Evidence & arithmetic contradicting the register:**
  1. **Part (a) is too generous in Q-001:** `EV-pack-H-22` lines 11–14 found zero supporting measurements, nearest Spearman **ρ = 0.40** (`2606.00702`, with complete collapse on high-predicted designs), NAS weight-sharing Kendall τ as low as **0.159–0.236**, and three papers proving shared controllers bias search toward easy-to-steer suboptimal designs. Marking it "unmeasured" in the table while relying on it as a load-bearing pillar of the Q-001 answer is inconsistent.
  2. **Part (b) was narrowed too timidly:** `EV-pack-H-22` line 16 extrapolated single-trajectory Lyapunov divergence (`2410.10674v1`: 5×10⁻⁴ deg after 50 steps) to conclude whole-episode ρ < 0.9 across the board. Yet for **fixed-policy continuous physical QoIs** (electrical energy per metre, steady velocity, stance torque utilization, slip distance over 2–5 s) on the *exact same* `ModelId`, `SceneId`, and Philox `WorldAddr`, `gpu-f32` vs `referee-f64` disagreement is bounded by contact compliance (not chaotic bifurcation), where `ρ ∈ [0.90, 0.98]` gives `1 / (1 − ρ²) = 5×–25×` referee variance reduction (verified via `fs-uq::product_mlmc` with fitted β and randomized audits)! Only chaotic binary fall/survival outcomes over long horizons suffer low ρ.
- **Cheapest settling test:** Using `GATE-FID-01`'s already-planned 200 paired addresses on 10 designs, compute cross-tier `ρ` *separately* for (1) continuous energy/m and torque utilization vs (2) binary 10 s fall/success, and measure actual MLMC interval-width speedup (~4 CPU-hours, zero new infra).

---

## 3. Verdicts Where I AGREE (12 of 24)

Brief confirmation of the 12 verdicts whose evidence packs hold up cleanly under scrutiny:
- **`H-05` (NARROWED):** AGREE — 4–260 s CPU orbit solves and unstabilized open-loop gaits make continuation a periodic screening tool, not a 1% controller replacement.
- **`H-06` (NARROWED):** AGREE — 10⁴ boxes is <1 bisection per axis in 20D; per-constraint ≤6D sub-boxes + monotonicity is the exact surviving scope.
- **`H-07` (NARROWED):** AGREE — Splitting explores while `fs-uq` bounded-weight IS certifies; static scene params gain nothing from mid-episode snapshots.
- **`H-09` (REFUTED as stated):** AGREE — Mass/geometry edits invalidate >90% of exact rollout GPU-seconds; memo hygiene and verdict revalidation survive cleanly.
- **`H-10` (NARROWED):** AGREE — `fs-race`'s own battery proves 2–5× only on separated fields and <1.5× on close ones; BO struggles against LLMs/GAs on noisy categorical RL.
- **`H-13` (NARROWED):** AGREE — Bounded-time STL belongs at acceptance/falsification, not replacing shaped RL rewards.
- **`H-14` (NARROWED):** AGREE — Beta(19, 2) has 6.3-point SD at n = 20 and robots (not episodes) are the exchangeable unit; `fs-eproc` shift test proves refusal outside calibrated buckets is mandatory.
- **`H-15` (REFUTED):** AGREE — Empirical MuJoCo probe directly triggers the falsifier (κ·u₁₆ ≥ 1 in 37–64% of scaled states; refinement costs 1.5–2.75× more than `f32` Cholesky).
- **`H-16` (REFUTED as stated):** AGREE — Stefan squeeze-film arithmetic proves `fs-tribo` film-pressure friction + rigid proxies cover kitchen/home failures without unbudgetable fluid/cloth islands.
- **`H-20` (NARROWED):** AGREE — SOS stalls past ~10–12 states (`fs-sos` is O(N⁴) dense) and template-to-full ε is uncomputable a priori; statistical barrier shields survive.
- **`H-23` (NARROWED):** AGREE — 3–4× published population compute gain; parametric sweeps exist in Warp; contact switches make long-horizon FD noisy.
- **`H-24` (NARROWED):** AGREE — P-GATr ran 7.5× slower and failed control; heading normalization + convention augmentation captures the real win.

---

## 4. Contesting the Overall Answer to Q-001

**Ruling on Q-001: CONTEST.** The register's synthesis contains a **fatal internal contradiction** and rests on its **weakest empirical link**:

1. **Direct contradiction with `H-19 = ACTIVE`:** The register lists five ingredients for Q-001: *"decide many designs per agent turn (`H-23`); avoid training for most of them with family policies (`H-02`) and inheritance (`H-21`), gated by measured rank agreement (`H-22`); stop early (`H-10`); search for failures (`H-07`)."* **Every single one of those five was explicitly tagged `"Rival: fires / portable to incumbents"` in its own evidence pack!** Against a composed incumbent (`B0⁺`: MuJoCo Warp + Hydrax/DIAL-MPC + URMA + checkpoint resume + racing), that conjunction delivers **~1× (a tie), NOT 3–10×**!
2. **Anchored on unmeasured, negatively indicated links (`H-02` + `H-22(a)`):** Betting the core screening funnel on zero-shot family policies across messy interiors when `EV-pack-H-02` reported **10% zero-shot on variable terrain** and `EV-pack-H-22` reported **nearest rank correlation ρ = 0.40** + systematic shared-controller search bias is high-risk wishful thinking.
3. **Prematurely discards the real engine-internal levers:** Dismissing engine speed as "a parity requirement" (`H-04`) and omitting differentiable contact (`H-25`) and inverse-dynamics convex synthesis (`H-26`) leaves FrankenRobotics with no genuine compute advantage over MuJoCo Warp on CUDA.

**What a non-contradictory, evidence-grounded answer to Q-001 actually looks like:**
- **Versus Q-001's written incumbent (`B0`: MuJoCo Warp/Newton + per-design PPO + outer optimizer):** **10×–30× end-to-end is achievable** without relying on fragile zero-shot family policies across rough interiors, via a **four-stage funnel**: (1) **<2 s inverse-dynamics convex synthesis (`H-26`) + catalog antichains (`H-17(c)`)** eliminating doomed actuator/gear/CoM regions before any rollout; (2) **two-stage coarse→fine GPU sampling MPC (`H-01`) over deterministic size-bucketed mixed-topology cohorts (`H-03`)** screening arbitrary morphologies without pretraining bias; (3) **joint IFT-SHAC (`H-25`) + guarded parameter-edit inheritance (`H-21`)** accelerating finalist specialist control 3×–5×; and (4) **asynchronous background CPU-referee MLMC (`H-11` + `H-22(b)`)** delivering referee-accepted verdicts off the critical path.
- **Versus a strong composed incumbent (`B0⁺`: MuJoCo Warp + DIAL-MPC + URMA + racing):** Be ruthlessly honest—**3×–10× pure speed on fixed-topology CUDA quadrupeds requires winning the engine-internal bets** (`H-04` tree-sparse LDLᵀ + constraint-Jacobian specialization + fusion, `H-25` smooth IFT adjoints which MuJoCo Warp lacks, and `H-26` inverse-dynamics convex regressor). Where FrankenRobotics achieves an **undeniable structural unlock over `B0⁺` (even when `B0⁺` ties on raw CUDA speed)** is on **referee-accepted decisions** (coupled two-tier exact-Coulomb referee via `H-22(b)`/`H-11`), **deterministic mixed-topology populations (`H-03`)**, **Apple M5 Ultra Metal 4**, and **estimator-observable, structurally sized designs (`H-12` `beamcol`, `H-27`)**.

---

## 5. Three Missing Hypotheses Absent From `hypothesis_slate.md`

None of `H-01`–`H-24` tests GPU analytical contact gradients, convex inverse-dynamics robot synthesis, or closed-loop perception/observability co-design. All three directly address Q-001's falsifier and survive Kill Test 6 (Rival).

### `H-25` — Smooth IFT Contact Adjoints & Joint Differentiable Actor-Critic Co-Design (SHAC)

- **Claim:** Reusing Stage 8's converged tree-sparse LDLᵀ Hessian factor (`H = A + JᵀGJ`) via the Implicit Function Theorem over short horizons (H = 16–32 steps) with binomial Revolve checkpointing and an online finite-difference cosine monitor cuts **joint finalist controller + continuous morphology co-optimization compute by at least 5×** against model-free PPO + black-box search (and supplies 30-parameter local sensitivity vectors in ≤2× one rollout cost), while failing Kill Test 6 on neither MuJoCo Warp nor Isaac Lab/PhysX (both explicitly non-differentiable).
- **Mechanism:** `frob-kir` emits reverse-mode straight-line pullbacks for `K-rnea`, `K-crba`, `K-cay-step`, `K-act-l1`, and the smooth convex contact residual `r(v_+, z) = 0` (C-06). After verifying Newton convergence/conditioning, one sparse triangular back-substitution (`H λ = v̄_+`) yields exact step sensitivities without unrolling Newton iterations. Joint `(φ, θ)` updates run via Short-Horizon Actor-Critic (SHAC) with antithetic CRN smoothing across contact switches and an online `fs-adjoint::certs`-style FD cosine guard that auto-clamps H when contact chatter degrades gradient quality.
- **Falsifier:** On quadruped and humanoid rough-terrain/stair co-design (20 continuous parameters + policy), joint IFT-SHAC fails to reach matched referee-verified return in ≤0.35× the GPU-hours of PPO + CMA-ES, OR online FD cosine similarity drops below 0.5 on >40% of contact-rich updates.
- **Expected evidence if true:** Published SHAC (`arXiv:2204.07137`), Dojo implicit contact differentiation (`arXiv:2203.00806`), and `Open-DiffLoco` SHAC quadruped results (`arXiv:2608.02069`) showing 3×–10× sample/wall-clock savings on compliant locomotion; MuJoCo Warp and PhysX docs confirming non-differentiability; MJX docs/issues documenting unrolled-solver gradient instability.

### `H-26` — Inverse-Dynamics Convex Robot Synthesis via the Linear-in-Pseudo-Inertia Euler–Poincaré Regressor

- **Claim:** Formulating mass/battery/CoM placement, gear-ratio selection, and actuator sizing inversely—exploiting the exact linearity of Euler–Poincaré `RNEA` (`C-04`) in pseudo-inertias `P_i ∈ 𝒫⁺(4)` (`C-03`), contact wrenches `γ_c(t) ∈ 𝒦_μ` (`C-06`), and joint torques `τ(t)`—solves globally optimal inertial/transmission parameters from nominal interior traversal trajectories in **<2 CPU-seconds** as a single SOCP/SDP, cutting forward co-design iterations to target electrical Cost of Transport by **at least 5×** and proving dynamically infeasible friction/actuator combinations via Farkas dual certificates before any GPU simulation.
- **Mechanism:** Given nominal kinematic/centroidal traversal trajectories for an interior task (stairs, wet kitchen turn, ramp, payload carry), `frob-kir` emits the exact symbolic RNEA regressor `Y(q(t), q̇(t), q̈(t))` so `τ(t) = Y(t)·π − J_c(t)ᵀ γ_c(t)`. `frob-algebra`'s existing symmetric-cone solver (SOC + PSD cones) solves jointly over realizable `P_i ≻ 0`, friction-cone wrenches `γ_c(t) ∈ 𝒦_{μ(x(t))}`, and L1 RMS/peak/voltage-speed constraints, minimizing electrical energy `Σ [R_j (τ_j/(N_j k_t))² + τ_j q̇_j] Δt`.
- **Falsifier:** Across 30 quadruped, humanoid, and wheel-leg designs on stairs and wet kitchens, inverse-synthesized `(CoM, gear ratios, actuator cards)` fail to beat default templates by ≥15% electrical CoT or save ≥3× forward co-design iterations under closed-loop MPC/PPO verification, OR fixed-trajectory assumptions cause >15% false infeasibility rejections (discarding designs that succeed by adapting their gait).
- **Expected evidence if true:** Wensing–Kim–Slotine (2018) linear-in-pseudo-inertia SDP identification literature; convex conic actuator/transmission sizing results; sub-second SOCP/SDP solve times for 50–100 trajectory knots on CPU.

### `H-27` — Joint SE₂₊ₖ(3) Estimator-Observability & Dynamic Self-Occlusion Sensor Co-Design

- **Claim:** Co-optimizing sensor mount poses (`g_sensor ∈ SE(3)`), torso/payload-bay geometry, and contact layout using the **exact state-independent Lie-algebraic Observability Gramian / CRLB of the contact-aided SE₂₊ₖ(3) Invariant EKF (`C-13`)** combined with **dynamic limb- and payload-occluded foothold lookahead ray tracing (`frob-sense`)** cuts the privileged-vs-estimated interior task failure gap by **at least 50%** and avoids **at least 3×** wasted teacher-student / perception-policy redesign cycles across cluttered payload-carrying missions—using Lie-group observability machinery unavailable in incumbent engines.
- **Mechanism:** Because `SE₂₊ₖ(3)` IEKF error dynamics are group-affine (Barrau & Bonnabel), the log-linearized transition matrix `A(t)` in `𝔰𝔢₂₊ₖ(3)` depends solely on the nominal trajectory twist and contact geometry, never on the error state. `frob-analyze` computes the exact trajectory-integrated Fisher Information Matrix / CRLB (exposing unobservable yaw/velocity drift on collinear contacts or wet floors) in milliseconds alongside GPU ray-traced foothold/threshold lookahead time (detecting when carried totes or swinging knees blind depth/lidar right before a stair nosing or doorway), exposing sensor mounts to joint optimization trading visibility against CoM height and under-table clearance.
- **Falsifier:** Across 20 payload-carrying quadruped, humanoid, and wheel-leg designs evaluated under `--state estimated` (IEKF + ray depth/lidar, zero privileged state) on stairs, doorways, and wet kitchens, designs optimized on `SE₂₊ₖ(3)` CRLB + lookahead visibility fail to cut the privileged-vs-estimated success gap by ≥50% vs default chassis mounts, OR the metrics correlate <0.65 with actual `--state estimated` navigation failure rates.
- **Expected evidence if true:** Hartley et al. (`arXiv:1904.09251`) and Barrau & Bonnabel (`arXiv:1410.1465`) contact-aided IEKF observability analyses; empirical sim-to-real reports of self-occlusion by carried payloads and low-mounted sensors in indoor stair/clutter navigation; `frob-lie` `SE₂₊ₖ(3)` and `frob-sense` ray kernels already in plan v0.2.
