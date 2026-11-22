RESULT_STATUS: MATERIAL_INCREMENT

# MOR-A1 Raw Creative & Hypothesis Pass — Algebraic Paradigm Shift, 10× Heterogeneous Co-Design & Constructive Formal Synthesis

| Field | Value |
|---|---|
| **Assignment ID** | `MOR-A1-math-synthesis` |
| **Run ID** | `r1-20261121` |
| **Date** | `2026-11-21` |
| **Operators Applied** | Constraint Reclassification · Analogical Transfer (`B6`) · Conceptual Blending (`B8`) · Brenner `⊘ Level-Split` · Brenner `𝓛 Recode` · Brenner Phase 3/4 Competing Hypotheses & Forced Third Alternatives · Brenner `✂ Exclusion-Test` (`†` kills) |
| **Target Pivotal Questions** | `PQ-10X-1` (Algebraic/Categorical Paradigm Shift), `PQ-10X-2` (10× Heterogeneous Co-Design & Zero-Shot Morphological Transfer), `PQ-10X-5` (Flipping `franken_lean` & `frankensympy` into Constructive Synthesis Engines) |
| **Paradoxes Attacked** | Paradox 1 (Algebra-vs-Speed Inversion), Paradox 2 (Heterogeneous-Morphology GPU Wall), Paradox 3 (Verification-vs-Contact Chasm) |
| **Epistemic Status** | All design options and claims below are author-level **`PROPOSED`** outputs awaiting normalization (`AUDIT_PENDING`) and independent audit per `CLAIM_SCHEMA.md`. No claim self-certifies. |

---

## 0. Grounded Sibling-Repository Fact Addendum (`E021`–`E026`)

Before generating options, direct source inspection of live sibling crates (`2026-11-21`) established six load-bearing `FACT` anchors that both **enable** concrete synthesis engines and **bound** what would otherwise be hand-waving:

| ID | Verified Fact (`FACT`) | Source Anchor | Consequence for `MOR-A1` |
|---|---|---|---|
| **`E021`** | `frankensim::fs-spectral::floquet` already implements complete nonsymmetric dense Floquet spectra (`FloquetSpectrum`, `FloquetControls`, `effective_spectral_radius`, `dominant_growth_rate_per_s`, `phase_multiplier_index`, explicit `M*g = g` phase neutrality check) plus `floquet::transient` nonnormal transient growth analysis — explicitly delegating trajectory integration and monodromy construction to the caller. | `~/projects/frankensim/crates/fs-spectral/src/floquet.rs:1-100` | `O003` (Poincaré/Floquet differentiable limit-cycle optimization) consumes a verified spectral backend directly; `frob-dyn`/`frob-adjoint` only supply the monodromy + Saltation operator. |
| **`E022`** | `frankensim::fs-sos` already ships multivariate polynomials (`MPoly`, outward-rounded interval `IPoly`), an in-house primal-dual interior-point block-SDP solver (`sdp`), centered SOS program lowering (`program`), Alefeld–Mayer interval Cholesky verification (`verify`), Lasserre/Putinar global optimization (`global`), and SOS Lyapunov regions of attraction (`roa`). | `~/projects/frankensim/crates/fs-sos/src/lib.rs:1-58` | `O007` (compiled GPU CBF shields) and `O008` (certified inner/outer feasible design polytopes) build directly on an existing pure-Rust SOS+interval-Cholesky stack. |
| **`E023`** | `frankensim::fs-phs` already implements `interconnect_mapped` (power-preserving port composition), `reduce_galerkin`, `multirate` (work-conjugate clocks), and `ConstitutiveDiscreteGradient` — and explicitly documents `DEFERRED: constrained/DAE Dirac structures wait for their first consumer`. | `~/projects/frankensim/crates/fs-phs/src/lib.rs:1-65` | `O001` (operadic open-pHS + kinematic module algebra) directly reuses `fs-phs` interconnection laws and supplies the exact DAE/rigid-port Dirac consumer `fs-phs` deferred. |
| **`E024`** | `frankensympy::fsym-solvers::real_rur` already implements provably complete bounded Rational Univariate Representation (`VerifiedRealRurSystem`) via checked grevlex Gröbner basis + checked FGLM conversion + invertible linear shears + zero-dimensional radical — **with hard verifier bounds**: `MAX_VARIABLES = 8`, `MAX_EQUATIONS = 64`, `MAX_DEGREE = 256`, `MAX_BITS = 4096`, `MAX_COMPUTE = 200_000_000`, and **explicitly zero-dimensional ideals only** (positive-dimensional manifolds unsupported). | `~/projects/frankensympy/crates/fsym-solvers/src/real_rur.rs:1-60` | **Kills** arbitrary high-DoF continuous-manifold Gröbner claims (`†`), while **enabling** exact Burmester 5-pose planar ($n=4$) and pinned spatial Study-quadric ($n \le 6$) zero-dimensional linkage synthesis (`O006`) inside verified caps! |
| **`E025`** | `frankensympy::fsym-formal` currently freezes only two Lean 4 projection profiles: `lean_core_zz_product_v1` and `lean_core_zz_nfactor_v1` (integer polynomial factor-product identities, `MAX_CAPSULE_BYTES = 16 KB`). | `~/projects/frankensympy/crates/fsym-formal/src/lib.rs:48-63` | Projecting SOS Gram certificates, CBF shield inequalities, and Burmester receipts into `franken_lean` requires minting new frozen profiles (`lean_core_qq_sos_gram_v1`, `lean_core_cbf_shield_v1`) rather than assuming arbitrary Lean projection already exists. |
| **`E026`** | `franken_lean::fln-anvil` already ships `simp`, `dtree`, **`egraph` (equality saturation with kernel-checked proof extraction)**, `norm_num`, **`omega` (linear-arithmetic decision procedure)**, `grind`, and speculative `portfolio`, all strictly gated by `fln-kernel` (Crucible, `<=12 KLOC`, `#![forbid(unsafe_code)]`). | `~/projects/franken_lean/crates/fln-anvil/src/lib.rs:1-38` | `fln-anvil::egraph` powers constructive verified kernel rewriting (`O007`); `fln-anvil::omega` powers linear polyhedral cutting-plane certification for feasible design envelopes (`O008`). |

---

## 1. Constraint Reclassification Matrix (`Operator: Constraint Reclassification`)

Before inventing mechanisms, separate **immutable physical/hardware laws** from **inherited simulator conventions** (which v1 of `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` partially inherited from MuJoCo/Isaac/Drake):

| # | Constraint / Assumption | Classification | Evidence | Unlock If Relaxed |
|---|---|---|---|---|
| **CR-1** | "Every time a robot topology changes, re-derive whole-robot symbolic dynamics, re-prove whole-robot Lean invariants, and re-compile one monolithic GPU kernel." | **Stale tooling convention** (inherited from flat URDF/MJCF trees) | `E010`, `E011` (§12.2, §14.1) | Rigid-body ABA Schur complements ($I^A, p^A$) and pHS Dirac ports (`E023`) compose **operadically** at 6-DoF hub ports — enabling `<25 ms` sub-kernel stitching and `<5 ms` functorial Lean proof composition (`O001`). |
| **CR-2** | "GPU physics throughput requires $B = 16,384$ identical copies of ONE robot topology." | **Generic GPU convention** (conflating grid-wide homogeneity with 32-thread warp homogeneity!) | `E001`, `E002`, `E010` | SIMT divergence occurs **within** a 32-thread warp/simdgroup, never **between** warps! Giving each of 512 distinct morphologies 2–4 warps (64–128 CRN envs) + SoA per-morphology parameter banks runs **512 different robots in ONE GPU launch** at 0% warp divergence (`O002`). |
| **CR-3** | "Evaluating a mutated robot morphology requires running 5M–50M RL steps (15+ minutes) from scratch." | **Inherited black-box RL convention** (conflating cyclic locomotion optimization, zero-shot morphological transfer, and specialist fine-tuning) | `E010`, `E012`, `E021` | Split into 3 stages: (1) **1–4 s** differentiable Poincaré/Floquet limit-cycle morphology optimization (`O003`), (2) **3 s** zero-shot `FiberMorph` gauge-equivariant sheaf policy rollout (`O004`), and (3) specialist RL *only* for top 3 Pareto finalists! |
| **CR-4** | "Closed-loop linkages and feasible design parameters must be discovered by trial-and-error sampling + post-hoc refusal." | **Legacy numerical convention** (treating CAS/Lean as passive checkers) | `E011` (§13, §14.3), `E022`, `E024`, `E026` | Invert the arrow: solve exact Burmester/Study linkage geometries algebraically in 50 ms (`O006`) and compile two-sided certified feasible polytopes $(\mathcal{P}_{\mathrm{in}}, \mathcal{P}_{\mathrm{out}})$ before sampling (`O008`). |
| **CR-5** | "Formal safety certificates (`Lean 4` / SOS) live offline because online safety filters require slow iterative QPs and contact chaos invalidates proofs." | **Level-conflated assumption** (mixing internal kinodynamic/thermal/self-collision invariants with unmodeled exterior friction) | `E011` (§13.2), `E022`, `E025` | Split Stratum 1 (internal pHS/joint/thermal/self-collision + bounded-disturbance capture-point) from Stratum 2 (exterior world uncertainty $d_{\max}$); compile exact closed-form `franken_lean`-verified CBF filters directly into CUDA/Metal kernels at `<0.25 µs`/step (`O007`). |
| **CR-6** | **ACTUAL HARD CONSTRAINTS (Never Relaxed):** `#![forbid(unsafe_code)]` Rust core; M5 Ultra 80c / 256 GiB UMA (~800 GB/s) & Dual RTX 4090 `SM_89` 2×24 GiB GDDR6X (PCIe Gen4, no NVLink, 72 MB L2/GPU, 128 SMs/GPU); `fsym-solvers::real_rur` caps (`n <= 8`, 0-D); 5 non-conflated claim axes (`INV-001`). | **Immutable physical / architectural law** | `E001`, `E002`, `E024`, `MOR_CASE.md` §1 | Every option below MUST pass `⊞ Scale-Check` and `INV-001` honesty bounds under these exact ceilings. |

---

## 2. Brenner Phase 3/4 Hypothesis Competition, Forced Third Alternatives & Explicit `†` Kills

Before enshrining options, we pit competing mechanisms against each other on every pivotal question and explicitly **KILL (`†`)** attractive-sounding ideas that fail `✂ Exclusion-Test` or `⊞ Scale-Check`.

### 2.1 `PQ-10X-1` — Compositional Robot Algebra & Hybrid Contact Topology

| ID | Hypothesis | Brenner `✂ Exclusion-Test` / `⊞ Scale-Check` Verdict |
|---|---|---|
| `H-101a` | **Status Quo (v1):** Flat `.fbot` AST + whole-robot symbolic CSE + whole-robot Lean proof replay on every discrete edit. | **SUPERSEDED (2× ceiling):** Forces 1.5–5 s re-derivation and full proof replay per topology mutation; exposes no reusable limb sub-kernels or compositional transfer primitives. |
| `H-101b` † | **Pure Abstract Category Theory:** Encode robots as morphisms in a general enriched $\infty$-operad / double category of arbitrary nonlinear hybrid DAE systems with runtime categorical dispatch. | **KILLED (`†` — Buzzword Theater & Physics Break):** (1) Simultaneous unilateral multi-foot impacts couple limbs globally through the Delassus Schur complement $W = J_c M^{-1} J_c^T$, breaking naive nonlinear monoidal independence; (2) runtime categorical wrappers violate `franken_threed` §7.2 (Paradox 1). |
| **`H-101c`** | **FORCED THIRD ALTERNATIVE (`O001`):** **Finite Typed Colored Operad $\mathcal{O}_{\mathrm{Rob}}$ of Open Rigid-pHS-Articulated Modules** compiling strictly at *design/compile time* to (a) Featherstone 6-DoF Schur-complement **Port-ABI sub-kernels** (`(I^A[21], p^A[6], elec_therm[3])`), (b) `fs-phs` Dirac composition proofs (`<5 ms` in `fln-kernel`), and (c) explicit sub-PROP escalation for cross-branch closed loops + global hub Delassus solve. | **SURVIVES → `O001`.** Exact mathematical isomorphism between operadic composition and Featherstone ABA inward/outward passes + `fs-phs` Dirac wiring; zero runtime overhead! |
| `H-102a` † | **Full Stratified Morse-Conley Algebraic Topology:** Compute Morse-Conley indices across the full $3^k$ stick/slip/open contact-mode stratification of a 12–30 DoF robot for global hybrid trajectory synthesis. | **KILLED (`†` — Exponential Blowup):** For a 6-foot spider, $3^6 = 729$ strata in $\mathbb{R}^{36}$; for a 20-contact dexterous hand, $3^{20} = 3.5 \times 10^9$ strata. Computing homology/Conley index pairs in dimension $>6$ is intractable by $>10^{12}\times$ on any hardware. |
| **`H-102c`** | **FORCED THIRD ALTERNATIVE (Folded into `O003` + v1 §11.3):** **$\Gamma$-Quotiented Contact-Mode Cayley Automaton** — quotient the $2^k$ foot-contact boolean lattice by morphological symmetry $\Gamma$ (collapsing 64 hexapod modes by $|D_6|=12$ to just **13 equivalence classes**; 16 quadruped modes by $|V_4|=4$ to **6 classes**), feeding directly into `fs-tropical` gait cycles and `O003`'s Poincaré guard sequences! | **SURVIVES (Clean, $O(2^k/|\Gamma|)$ tiny finite graph).** Captures 100% of practical gait-mode topology in microseconds without exponential blowup. |

### 2.2 `PQ-10X-2` — Breaking Paradox 2 (Heterogeneous GPU Batching) & Zero-Shot Transfer

| ID | Hypothesis | Brenner `✂ Exclusion-Test` / `⊞ Scale-Check` Verdict |
|---|---|---|
| `H-201a` † | **Single-Thread Generic Tree Interpreter:** Run 1 thread/env interpreting arbitrary CSR robot topology arrays on GPU. | **KILLED (`†` — Warp Divergence & Memory Starvation):** Adjacent threads in a 32-lane warp execute divergent loop bounds and uncoalesced indirect tree loads → 4×–10× throughput collapse. |
| `H-201b` † | **Monolithic Universal 60-DoF Dense Super-Tree Masking:** Embed dogs, spiders, humanoids, and tracked rovers inside one 60-DoF tree masked by `0/1` joint flags. | **KILLED (`†` — Register Spill & 5× Flop Waste):** Stepping a 12-DoF quadruped inside a 60-DoF masked tree wastes 80% of flops/memory and spills past `SM_89`'s 128-reg/thread sweet spot. |
| **`H-201c`** | **FORCED THIRD ALTERNATIVE (`O002`):** **Level-Split Two-Tier Warp-Homogeneous Operadic Megakernel** — (Level A) promote continuous $\theta_{\mathrm{morph}}$ to SoA per-env/per-warp buffers so 16,384 continuously varied robots execute ONE zero-divergence CSE kernel; (Level B) schedule 512 *topologically* distinct morphologies at **warp granularity** (2–4 warps = 64–128 CRN envs per morphology → 1,024–2,048 warps saturating all 128 SMs/4090 and 80 cores/M5 Ultra) with uniform warp-level dispatch to pre-compiled operadic limb sub-kernels + bounded family super-trees ($\text{DoF}_{\max}/\text{DoF}_{\min} \le 1.5$). | **SURVIVES → `O002`.** 0.0% intra-warp divergence, 512 KB parameter bank 100% L2-resident, full SM/simdgroup occupancy, **0 ms XLA recompilation**! |
| `H-202a` | **Pure Zero-Shot Universal Policy Only:** Replace specialist RL entirely with one graph transformer across all morphologies. | **PARTIALLY REJECTED:** Zero-shot policies empirically sacrifice 5%–15% asymptotic optimality on final tuned hardware and struggle on exotic asymmetric outlier topologies. |
| `H-202b` | **Pure Limit-Cycle / Trajectory Optimization Only:** Drop RL and rely solely on Poincaré/Floquet optimization. | **PARTIALLY REJECTED:** Limit cycles govern periodic gaits only; they fail on non-periodic cluttered kitchen/warehouse navigation, door opening, and occluded perception recovery. |
| **`H-202c`** | **FORCED THIRD ALTERNATIVE (`O003` + `O004` + `O009` Cascade):** **Three-Speed Co-Design Hierarchy** — (1) **1–4 s** warp-parallel Poincaré-Map + Saltation + `fs-spectral::floquet` implicit limit-cycle optimizer (`O003`) for dynamic locomotion & morphology pre-optimization; (2) **3 s** zero-shot Buckingham-$\Pi$-normalized $SE(3)$-gauge-equivariant sheaf policy `FiberMorph` (`O004`) evaluated across 64 cluttered interiors; (3) specialist $\Gamma$-equivariant PPO/SHAC *only* on the top 3 Pareto winners! | **SURVIVES → `O003`, `O004`, `O009`.** Combines millisecond algebraic dynamics, second-scale zero-shot cluttered-world screening, and zero asymptotic sacrifice on finalists. |
| `H-203a` † | **Global Cross-Contact Bilinear Koopman / EDMD MPC + Zero-Shot Morphology Extrapolation:** Lift full 36–60 state hybrid multi-contact dynamics into one globally linear/bilinear Koopman space on $SE(3)$ for MPC and instant morphology generalization. | **KILLED (`†` — Hybrid Continuity Breakdown + Dimensionality Explosion + No Morphology Map):** (1) Impact velocity jumps make the Koopman semigroup discontinuous across contact surfaces, causing Gibbs-like ringing and contact penetration; (2) degree-2 observables on $n=36$ states have $\binom{38}{2}=703$ dims → $K \in \mathbb{R}^{703\times 703}$ + 12 bilinear $B_i$ matrices = **6M params**, making dense lifted QP MPC **~100× slower** than 36-state Lie-group DDP; (3) data-driven eigenfunctions have zero analytic extrapolation across mutated link inertias! |
| **`H-203c`** | **FORCED THIRD ALTERNATIVE (Bounded Tail Option `O010`):** Restrict $SE(3)$-equivariant bilinear Koopman lifting strictly to a **low-dimensional ($n \le 12 \to 48$ lifted dims) local smooth residual surrogate** for non-analytic deformable/mud/Cosserat-tail interactions inside a single contact regime — explicitly kept as a low-cost tail `HYPOTHESIS`, NEVER on the critical path. | **SURVIVES AS TAIL OPTION (`O010`) ONLY.** Eliminates a seductive 6-month mathematical dead end while preserving the narrow regime where EDMD actually shines. |

### 2.3 `PQ-10X-5` — Constructive Formal & Symbolic Synthesis (Solving Paradox 3)

| ID | Hypothesis | Brenner `✂ Exclusion-Test` / `⊞ Scale-Check` Verdict |
|---|---|---|
| `H-501a` † | **Unbounded Whole-Robot Spatial Gröbner Inverse Design:** Ask `frankensympy` to solve full 12–29 DoF spatial kinematics + dynamics + obstacle workspace symbolically via Gröbner bases over $\mathbb{Q}$. | **KILLED (`†` — Violates `E024` Hard Caps & Doubly-Exponential Complexity):** `fsym-solvers::real_rur` enforces `MAX_VARIABLES = 8`, `MAX_BITS = 4096`, and 0-D ideals only (`E024`). Full-robot spatial ideals blow up doubly exponentially in $n$. |
| **`H-501c`** | **FORCED THIRD ALTERNATIVE (`O006`):** **Operadic Primitive Exact Inverse Synthesis** — restrict Gröbner/RUR Burmester & Study-quadric synthesis strictly to **atomic closed-loop limb/transmission/gripper primitives** ($n \le 6$ vars, 0-D precision-pose slices, e.g. 5-pose planar 4-bar leg/ankle $n=4$ or pinned spatial SS/RRS dyads $n \le 5$), solving in **5–80 ms** with completeness receipts and composing them into full robots via `O001`! | **SURVIVES → `O006`.** 100% inside verified `fsym-solvers::real_rur` limits; deletes 10,000+ blind linkage guesses. |
| `H-502a` † | **End-to-End Formal Safety Proofs of Unstructured Real-World Contact Chaos:** Attempt to prove collision-free real-world kitchen navigation in `franken_lean` directly. | **KILLED (`†` — Undecidable & Conflates Claim Axes `INV-001`):** Violates `NONGOAL-006` and conflates `VerifiedFormal` with unmodeled real-world friction/perception. |
| **`H-502c`** | **FORCED THIRD ALTERNATIVE (`O007`):** **Two-Stratum Compiled Verified GPU CBF Safety Shields** — prove Stratum-1 internal electro-thermal / joint-limit / capsule self-collision / bounded-disturbance capture-point CBFs via `fs-sos` + `franken_lean` (with outward-rounded `IPoly` `f32` rounding slack $\delta_{\mathrm{fp32}}$), symbolically lower the closed-form Sontag/KKT projection into **fused CUDA PTX / Metal 4 MSL kernel tails** (`<0.25 µs`/step), and expose a live runtime `DisturbanceBoundExceeded` monitor for Stratum-2 world shocks! | **SURVIVES → `O007`.** Resolves Paradox 3 honestly: machine-checked formal certificates actively accelerate GPU RL *and* ship as deployable zero-overhead safety shields with zero claim laundering. |
| `H-503a` | **Pure Inner Inscribed Certified Polytope Only:** Restrict all morphology search strictly inside a single SOS-certified inscribed ellipsoid $\mathcal{P}_{\mathrm{in}} \subseteq \mathcal{F}$. | **PARTIALLY REJECTED (Conservatism Trap):** Optimal robotic designs frequently sit on active thermal/geometric boundary corners where an inscribed ellipsoid leaves 20%–40% of valid design volume unreachable. |
| **`H-503c`** | **FORCED THIRD ALTERNATIVE (`O008`):** **Two-Sided Certified Design Sandwich $(\mathcal{P}_{\mathrm{in}} \subseteq \mathcal{F} \subseteq \mathcal{P}_{\mathrm{out}})$** — `fs-sos` + `fln-anvil::omega` certify BOTH (1) a 100%-feasible inner polytope/ellipsoid $\mathcal{P}_{\mathrm{in}}$ for zero-rejection batch warm-starts AND (2) a certified outer superset $\mathcal{P}_{\mathrm{out}}$ proving everything outside is mathematically impossible (pruning 70%–95% of dead space with **zero false exclusions of boundary optima**)! | **SURVIVES → `O008`.** Zero dead-space waste AND zero sacrifice of boundary optima. |

---

## 3. Canonical MOR Creative Records (`O001`–`O008`)

---

### `O001` — Typed Colored Operad of Open Rigid-pHS-Articulated Modules (`frob-operad`) Compiling to Featherstone Port-ABI Sub-Kernels & Modular Lean Proof Functors

- **Canonical claim ID:** `MOR-A1-C001` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `STRUCTURALLY_MAPPED`
- **Target PQ / Paradox:** `PQ-10X-1` · Resolves **Paradox 1 (Algebra-vs-Speed Inversion)**
- **Need claims:** `E010` (monolithic per-morphology re-derivation/recompilation bottleneck), `E011` (v1 flat `.fbot` re-compiles & re-proves whole robot on discrete edits), `E012` (AI agents need algebraic morphology rewrites, not fragile AST surgery)

#### Mechanism
Elevate `FrankenBot IR` (`.fbot`) from a flat kinematic tree into an **executable colored operad $\mathcal{O}_{\mathrm{Rob}}$** whose objects (colors) are typed physical attachment ports:
$$c = \big(\text{FrameType},\; \text{MechPort} \in T^*SE(3),\; \text{ElecBus} \in \mathbb{R}^2_{\ge 0},\; \text{ThermSink} \in \mathbb{R}^2,\; \text{ContactSlots} \in \mathbb{N}\big)$$
and whose $k$-ary operations $M \in \mathcal{O}_{\mathrm{Rob}}(c_1, \dots, c_k; c_{\mathrm{out}})$ are self-contained robotic modules (`QuadLeg3DoF`, `WheelPod2DoF`, `ParallelAnkle4Bar`, `DualTrackBogie`, `TorsoHub<K>`, `PMSMActuatorStage`, `SensorMast`). Every module compiles through **three synchronized operad-algebra functors** (strictly at compile/design time — **zero runtime categorical dispatch**):

1. **Dynamics Schur-Complement Functor $\mathcal{F}_{\mathrm{ABA}} : \mathcal{O}_{\mathrm{Rob}} \to \mathbf{KernelABI}$:**
   Exploiting the exact algebraic identity that Featherstone's inward ABA pass maps any articulated subtree to its root **articulated-body inertia and bias wrench** $(I^A_{\mathrm{port}} \in \mathrm{Sym}^+(6),\; p^A_{\mathrm{port}} \in \mathfrak{se}(3)^*)$, every leaf/limb module lowers ONCE via `frankensympy` CSE into two branch-free inlineable device functions:
   ```text
   mod_inward(q_loc, dq_loc, tau_loc, f_ext_loc, theta_mod) -> PortSummary { I_A[21], p_A[6], i_bus, p_diss, t_max }
   mod_outward(a_port, q_loc, dq_loc, theta_mod)            -> (ddq_loc, next_elec_therm_state)
   ```
   A hub operation $\gamma(\mathrm{Torso}; M_1, \dots, M_k)$ simply sums the transformed port summaries $I^A_{\mathrm{hub}} = I_{\mathrm{torso}} + \sum_{j=1}^k \mathrm{Ad}^*_{g_j} I^A_j \mathrm{Ad}_{g_j}$ and solves one $6 \times 6$ base acceleration — **never re-running symbolic CSE on the limbs!**
2. **Modular Formal Proof Functor $\mathcal{F}_{\mathrm{Lean}} : \mathcal{O}_{\mathrm{Rob}} \to \mathbf{FrobMath}$:**
   Each library module carries pre-checked `franken_lean` (`fln-kernel`) certificates (`PO-MOD-INERTIA`: $J_i \in \mathrm{Sym}^+(4)$, `PO-MOD-PASSIVITY`: $J_{\mathrm{mod}} + J_{\mathrm{mod}}^T = 0,\; R_{\mathrm{mod}} \succeq 0$, `PO-MOD-SKEW`). Because Dirac port interconnection (`fs-phs::interconnect_mapped`, `E023`) is strictly associative and passivity-preserving, composing $k$ verified modules into a brand-new robot invokes **one single functorial composition lemma** in `fln-kernel` checking port color match and $J_{\mathrm{wire}} + J_{\mathrm{wire}}^T = 0$ — certifying the entire composite robot in **`<5 ms`**!
3. **Algebraic Rewrite Calculus for AI Agents (`frob-mcp`):**
   Agents mutate robots via typed operadic morphisms (`swap_limb(FL, QuadLeg -> WheelPod)`, `apply_dihedral_fan(D6, HexLeg)`, `splice_transmission(knee, FourBarStage)`), preserving frame typing, SI units, $\mathrm{Sym}^+(4)$, and passivity **by syntactic construction**.

#### Project Fit
Directly unifies `frob-bot`, `frob-symbolic`, `frob-codegen`, `frob-proof`, and `fs-phs` (`E023`, providing the exact rigid-port DAE Dirac consumer `fs-phs` left deferred). Uses `frob-kernel-ir` as the single SSA target so Metal 4 MSL and CUDA PTX inline the exact same verified port-ABI sub-kernels.

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (Hardware RTL / Open pHS Operads — Baez–Fong–Pollard / Featherstone Substructuring) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Colored operad wiring diagram $\gamma(f; g_1,\dots,g_k)$ | Type-safe composition of open systems with interface preservation | `.fbot` operadic morphology expression + `frob-operad` AST | Compositional robot blueprint & AI-agent rewrite algebra | **Invariant:** Associative port wiring preserves Frame Groupoid + Dirac passivity. **Mismatch:** Cross-branch kinematic loops require PROP trace/feedback (handled by explicit escalation). |
| Static Schur-complement / Guyan substructuring | Exact elimination of internal DoFs onto boundary stiffness/mass | Featherstone inward articulated-body port summary $(I^A, p^A)$ | Exact $O(n)$ zero-approximation boundary summary per limb | **Invariant:** Exact for unconstrained tree dynamics. **Mismatch:** Simultaneous unilateral multi-foot impacts couple across hub via $W = J_c M^{-1} J_c^T$ — solved globally at hub Delassus stage using the exact same $I^A$ propagators! |
| Separate compilation + ABI linking (LLVM modules) | O(1) incremental rebuild when wiring pre-compiled modules | Pre-compiled `frankensympy` limb SSA sub-kernels in CAS | `<25 ms` whole-robot kernel stitching on topology swaps | **Invariant:** Pure register-in/register-out ABI. **Mismatch:** Cross-boundary CSE across hip joint is forfeited (~3–6% flop cost vs monolithic CSE — bounded and measurable). |

#### Breakpoints
1. **Cross-branch closed kinematic loops** (e.g., dual arms holding one rigid crate, or Stewart platforms with 6 struts meeting one platform): tree-operad composition alone cannot eliminate internal loop wrenches; must detect cycle edges and escalate the minimal containing sub-PROP to `frob-symbolic` GGL/Gröbner reduction.
2. **Simultaneous multi-limb contact impulses:** Impact impulses on Foot 1 instantaneously couple into Foot 2 through the floating base; the operad decomposes **free-flight/bias dynamics and passivity**, while the **Delassus matrix $W$** is assembled via one hub-level Schur sweep over active contact ports.
3. **Cross-port CSE loss:** Compiling limbs separately forfeits algebraic cancellations *across* the hip attachment frame (~3–8% extra flops vs monolithic CSE). Must stay `<10%` or offer async monolithic re-CSE for final Pareto winners.

#### Dependencies
- `frob-lie`, `frob-bot`, `frob-symbolic`, `frob-kernel-ir`, `frob-proof` (`fln-kernel`), `fs-phs::interconnect_mapped` (`E023`).

#### Second-Order Effects
- **Reusable Verified Module Library:** Every limb/transmission/gripper ever verified enters CAS with its Lean certificate + compiled PTX/MSL sub-kernel + Burmester synthesis spec. An AI agent designing its 50th robot starts from a library of 100 pre-certified physical building blocks.
- **Unlocks `O002` directly:** Operadic limb sub-kernels ARE the warp-dispatched building blocks of the 512-morphology heterogeneous GPU megakernel!

#### Asymmetric Value
- **Topology-edit compile latency:** `1.5–5.0 s` (v1) / `15–90 s` (MJX XLA) → **`<25 ms`** (**60×–3,600× faster**).
- **Topology-edit Lean proof latency:** `0.5–2.0 s` (v1) → **`<5 ms`** (**100×–400× faster**).
- **Structurally impossible in MuJoCo/Isaac/Drake/Genesis:** None combine symbolic port-ABI sub-kernel compilation with functorial formal passivity proofs.

#### Cost / Reversibility
- **Cost:** ~1,800 LOC in new `frob-operad` crate (M1/M2) + 250 LOC Lean functorial composition theorems in `FrobMath`.
- **Reversibility:** High — an operad expression flattens deterministically into v1's `.fbot` tree AST, so monolithic CSE remains an automatic fallback.

#### `⊞ Scale-Check`
- **Port-ABI Register Footprint:** $I^A \in \mathrm{Sym}^+(6)$ (21 `f32` = 84 B) + $p^A \in \mathbb{R}^6$ (24 B) + `(i_bus, p_diss, t_max)` (12 B) = **120 B = 30 scalar registers** (fits cleanly inside the 128-reg/thread envelope on `SM_89` and Apple M5 Ultra with zero local-memory spill!).
- **Sub-Kernel I-Cache Size:** 3-DoF leg inward+outward sub-kernel $\approx 730$ FMA ops $\approx 4\text{ KB}$ PTX/MSL; a 12-archetype library = **48 KB** (100% resident in GPU instruction cache).
- **Lean Composition Proof:** Checking port-color equality + $J_{\mathrm{wire}} + J_{\mathrm{wire}}^T = 0$ on a 6-limb robot = $<250$ kernel reductions $\implies$ **`<5 ms`** in `fln-kernel`.

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
Build a 3-archetype operad prototype (`QuadLeg3DoF`, `WheelPod2DoF`, `TorsoHub<K>`) in M1/M2; compose a 4-leg dog, 6-leg spider, and 2-leg+2-wheel hybrid; compare (1) port-ABI stitched kernel output against monolithic CPU `f64` ABA (`<= 1e-12` diff), (2) GPU step latency vs monolithic CSE kernel, and (3) `fln-kernel` composition verification wall time.

#### Promotion & Retirement Thresholds
- **Promote:** Exact `f64` parity `<= 1e-12`, stitched GPU throughput within `>= 0.90×` of monolithic CSE, topology-swap stitch+verify latency `< 50 ms`.
- **Retire / Fallback:** If port-ABI call overhead causes `> 25%` GPU slowdown vs monolithic CSE or register spilling exceeds 128 regs/thread on `SM_89`, retain `frob-operad` strictly as a **front-end algebraic rewrite & Lean proof composition layer** while lowering through monolithic inline expansion.

---

### `O002` — Two-Level Operadic Warp-Homogeneous Megakernel for 512-Morphology Simultaneous GPU Simulation (Breaking Paradox 2)

- **Canonical claim ID:** `MOR-A1-C002` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `STRUCTURALLY_MAPPED`
- **Target PQ / Paradox:** `PQ-10X-2` · Directly solves **Paradox 2 (The Heterogeneous-Morphology GPU Wall)**
- **Need claims:** `E001`, `E002`, `E010` (MJX/Isaac require 16,384 identical copies of ONE robot; evolving 512 designs collapses throughput 10×–100×)

#### Mechanism
Apply **Brenner `⊘ Level-Split`** to separate *continuous parametric heterogeneity* from *discrete topological heterogeneity*, and schedule at **true SIMT warp/simdgroup granularity (32 lanes)** rather than grid granularity:

1. **Level A — Continuous Parametric Heterogeneity (Zero-Divergence SoA Parameter Banks):**
   In v1 §12.2, numeric morphology parameters $\theta_{\mathrm{morph}}$ (link lengths, $\mathrm{Sym}^+(4)$ log-Cholesky inertias, joint limits, gear ratios, wheel radii, motor $K_t/R_w/C_{\mathrm{th}}$) are broadcast constants per launch. Promote them to a **warp-indexed / env-indexed SoA parameter bank `ParamBank[param_p * K + morph_id]`**! Every thread in a 16,384-env launch executes the **identical branch-free CSE instruction sequence** while reading distinct continuous link geometries and inertias per morphology — achieving **0.0% warp divergence** across up to 16,384 continuously distinct robots!
2. **Level B — Discrete Topological Heterogeneity (Warp-Homogeneous Persistent Operadic Megakernel + Bounded Family Super-Trees):**
   - **Key GPU Invariant:** On both NVIDIA Ada Lovelace (`SM_89`, 32-thread warps) and Apple M5 Ultra (32-thread simdgroups), **SIMD/SIMT divergence ONLY occurs inside a 32-thread warp — NEVER between warps!**
   - In any AI Design Studio evolution or e-process race (`O009`), each candidate morphology $k \in \{0, \dots, K-1\}$ requires $R \ge 64$ parallel CRN rollout environments (across 64 world/terrain seeds for low-variance evaluation).
   - Assign **$m = R / 32 \in \{2, 4\}$ full warps to each morphology $k$**. Inside each warp, all 32 lanes share the exact same `morph_id`, `topology_id`, and `limb_descriptor_table`!
   - A **single persistent GPU megakernel** (captured in ONE `CUgraphExec` on Dual 4090 / ONE Metal 4 unified compute encoder on M5 Ultra) executes:
     1. Uniform warp load of `topo = topo_table[morph_id]` (zero intra-warp branch divergence!);
     2. Loop over attached operadic ports $j \in 0..k_{\mathrm{limbs}}$ (identical count for all 32 lanes in the warp!), dispatching uniformly to the pre-compiled inlineable operadic sub-kernel (`O001`) for `port_kind[j]`;
     3. Hub $6 \times 6$ composite solve + outward limb sweep;
     4. Unified warp-cooperative **De Saxcé circular-cone Delassus PGS/ADMM** contact pass parameterized by `max_contacts_in_warp` (padded to multiples of 4 contacts);
     5. Optional **Bounded Family Super-Tree** for continuous topology-pruning homotopies within one family (strictly bounded by $\text{DoF}_{\max}/\text{DoF}_{\min} \le 1.5$ via null-joint Lie generators $\xi_j = 0 \implies \exp(0) = I_{SE(3)}$!).
   - **Zero-Recompile Live Population Churn (e-Process Cancellation Integration):** When `fs-eproc` eliminates a dominated morphology $k$ mid-tournament, the host writes the newly spawned child morphology's parameters + `limb_descriptor_table` into slot $k$ in **unified memory (`StorageModeShared` on M5 Ultra, 0 µs!)** or via one 2 KB async H2D memcpy on 4090 — **WITHOUT ever tearing down or rebuilding the CUDA Graph / Metal 4 pipeline!**

#### Project Fit
Exploits BOTH silicon poles asymmetrically:
- **Dual RTX 4090 (`E002`):** Shards 256 morphologies per GPU ($B = 32,768\text{–}65,536$ envs total); the entire 512-morphology parameter bank (`512 KB`) sits 100% inside each 4090's **72 MB L2 cache**; CUDA Graph replay eliminates host launch overhead across all 512 morphologies simultaneously!
- **Apple M5 Ultra (`E001`):** 80 GPU cores run 1,024–2,048 simdgroups concurrently while **256 GiB unified memory** holds 64 distinct multi-room VDB interiors + 512 morphologies + live `FiberMorph` training replay buffers with zero PCIe transfers!

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (Wavefront GPU Path-Tracing Megakernels & `franken_threed` §4.10) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Material-sorted wavefront shading queues (32-lane coherent warps) | Eliminates intra-warp material shader divergence in heterogeneous scenes | Warp-grouped morphology assignment (`morph_id = warp_id / m`) | Eliminates intra-warp kinematic/topology divergence across 512 distinct robots | **Invariant:** All 32 lanes follow one control-flow path. **Mismatch:** Different warps finish at different cycle counts (12-DoF dog finishes faster than 29-DoF humanoid) → **tail latency / load imbalance**, mitigated by dynamic persistent work-stealing queue or grouping similar-DoF archetypes into SM waves! |
| Bindless descriptor tables (Metal 4 argument buffers / CUDA device pointers) | Swapping scene assets without pipeline recompilation | `ParamBank` + `LimbDescriptorTable` slot hot-swap | Zero-recompile live morphology replacement during e-process racing | **Invariant:** Immutable kernel binary, dynamic typed data. **Mismatch:** Introducing a brand-new unseen limb *archetype* still requires compiling that 4 KB sub-kernel once (`<250 ms`). |

#### Breakpoints
1. **Inter-warp load imbalance (straggler effect):** Mixing 6-DoF wheeled carts and 37-DoF humanoids in the exact same grid causes fast warps to finish early while long warps hold SMs. **Mitigation:** Persistent warp worklist queue (each warp pulls next `(morph_id, env_chunk_32)` atomically) OR two-bucket dispatch (`<=18 DoF` vs `19–48 DoF`).
2. **Occupancy trap if $R$ is too small:** Running only 32 envs/morph across 512 morphologies = 512 warps = only 2 warps/SM across Dual 4090 (256 SMs total) → memory-latency starved! **Hard rule:** enforce $K \cdot (R/32) \ge 8 \times N_{\mathrm{SM}}$ (e.g., $R \ge 64\text{–}128$ envs/morph, or sub-warp 8-thread cooperation on large morphologies).
3. **Register pressure of unified switch-case megakernel:** Putting 12 limb sub-kernels inside one monolithic switch can inflate the kernel's *maximum* register count to the heaviest branch (e.g., 160 regs/thread, throttling occupancy of light warps). **Mitigation:** Measure register ceiling per target; if `regs > 96`, dispatch 2–3 **DoF-tiered megakernels** (Tier-S: wheeled/quad/spider `<=24 DoF`; Tier-M: wheel-leg/humanoid `25–48 DoF`) inside the same CUDA Graph!

#### Dependencies
- `O001` (operadic limb port-ABI sub-kernels), `frob-gpu-cuda` (CUDA Graphs + `SM_89`), `frob-gpu-metal` (`ft-kernel-metal` unified compute encoder).

#### Second-Order Effects
- **Enables `O004` (`FiberMorph`) directly:** A morphological foundation policy can be trained across 512 diverse topologies *in one single RL loop* without environment-switching overhead!
- **Enables zero-downtime live evolutionary tournaments:** Dead morphologies are hot-swapped in `<0.1 ms` into freed warp slots while surviving warps keep stepping!

#### Asymmetric Value
- **Multi-morphology throughput (512 distinct morphologies × 64 envs = 32,768 envs):**
  - MJX/JAX: 512 separate XLA compiles (~15–40 min compile alone!) + sequential/fragmented launches.
  - Isaac Lab: ~10×–50× throughput collapse when mixing distinct articulation topologies.
  - `frob` `O002`: **0 ms compile on param/slot swaps**, **>=75% of homogeneous single-robot GPU throughput** across 512 simultaneous morphologies (**20×–100× wall-clock speedup** for population co-design & generalist policy training!).

#### Cost / Reversibility
- **Cost:** ~2,200 LOC across `frob-gpu`, `frob-gpu-cuda`, `frob-gpu-metal` (M2/M5).
- **Reversibility:** High — setting `K = 1` reduces identically to v1's homogeneous `VectorEnv` kernel (zero architectural divergence!).

#### `⊞ Scale-Check`
- **Population:** $K = 512$ distinct morphologies × $R = 128$ CRN envs/morph = **$B = 65,536$ total envs** (32,768 envs / GPU on Dual 4090 = 1,024 warps/GPU = **8 warps/SM across all 128 SMs**; on M5 Ultra 80 cores with $B = 32,768$ envs = 1,024 simdgroups = **12.8 simdgroups/core**!).
- **Parameter Bank Memory:** $512 \times 256 \times 4\text{ B} = \mathbf{512\text{ KB}}$ (**0.7% of RTX 4090 72 MB L2 cache**; 100% L2-resident!).
- **State Buffer Memory (SoA, 24-DoF avg, 640 B/env):** $65,536 \times 640\text{ B} = \mathbf{41.9\text{ MB}}$ (**0.17% of 24 GiB VRAM**, **0.015% of 256 GiB M5 Ultra UMA**).
- **Contact Delassus Scratch (in shared/threadgroup SRAM):** $m_c \le 16$ contacts/robot × 3D $\implies$ symmetric $W$ or PGS diagonal precond = $1.2\text{ KB/warp}$ × 8 warps/SM = **9.6 KB/SM** (well below `SM_89`'s 100 KB shared memory/SM!).
- **Zero-Copy Slot Hot-Swap Bandwidth:** Updating 32 eliminated morphologies/sec = $32 \times 2\text{ KB} = 64\text{ KB/s}$ (negligible).

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
In M2, launch ONE CUDA Graph on 1× RTX 4090 and ONE Metal 4 encoder on M5 Ultra running **128 distinct morphologies** (mixing 64 parametrically varied quadrupeds + 32 hexapods + 32 wheel-quads, $R = 64$ envs each, $B = 8,192$ envs) vs 1 homogeneous quadruped launch at $B = 8,192$. Measure aggregate env-steps/s, SM/simdgroup occupancy, register count, and CPU `f64` parity on all 128 designs.

#### Promotion & Retirement Thresholds
- **Promote:** Aggregate heterogeneous throughput $\ge 0.70\times$ matched homogeneous throughput, `0` kernel recompilations on live slot swap, differential parity $\le 10^{-5}$ (`MathMode::Safe`).
- **Retire / Fallback:** If unified megakernel register pressure drops throughput below `0.45×` homogeneous, split into DoF-tiered homogeneous-topology batch launches inside one CUDA Graph (still keeping Level-A zero-divergence SoA parameter banks across 512 continuous morphologies!).

---

### `O003` — Warp-Parallel Differentiable Poincaré-Map & Floquet-Multiplier Limit-Cycle Optimizer (Powered by `fs-spectral::floquet` + Exact Saltation Adjoints)

- **Canonical claim ID:** `MOR-A1-C003` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `STRUCTURALLY_MAPPED`
- **Target PQ / Paradox:** `PQ-10X-2` & `PQ-10X-1` · Resolves **Paradox 1 (Algebra-vs-Speed Inversion)**
- **Need claims:** `E010` (15–45 min noisy RL needed just to check if a mutated walking/trotting morphology is dynamically stable and efficient), `E021` (`fs-spectral::floquet` already exists in `frankensim`!)

#### Mechanism
For every periodic locomotion or cyclic task (trotting, galloping, bipedal walking, hexapod tripod/ripple gaits, wheel-leg carving/skating), **bypass 50M stochastic RL steps completely during inner morphology optimization** by solving and differentiating directly through the **fixed point of the hybrid Poincaré return map**!

1. **Hybrid Limit-Cycle Fixed-Point Formulation:**
   Define a Poincaré cross-section $\Sigma = \{x : s(x) = 0,\; \nabla s \cdot f(x) > 0\}$ (e.g., apex height or stance touchdown) in the **base-horizontal-$SE(2)$-quotiented reduced state** $x_\Sigma \in \mathbb{R}^{n_\Sigma}$ (eliminating ignorable world $(x, y, \psi)$ coordinates via $SE(2)$ symmetry!). For gait/controller parameters $u$ (e.g., Fourier/spline CPG or IDA-PBC trajectory parameters) and morphology $\theta \in \mathcal{M}_{\mathrm{morph}}$, one stride defines the Poincaré return map $\Phi(x_\Sigma, u, \theta) \in \Sigma$, with periodic gait condition:
   $$R(x_\Sigma^*, u, \theta) \equiv \Phi(x_\Sigma^*, u, \theta) - x_\Sigma^* = 0$$
2. **Exact Saltation-Aware Monodromy Matrix in ONE GPU Warp Pass (`𝓛 Recode` Hardware-Algebra Match!):**
   Across smooth stance/flight phases $k$, the tangent variational flow integrates $D\psi_k$ via forward-mode differentiation of the LGVI kernel (`frob-codegen`). Across each unilateral foot-touchdown guard $g_k(x) = 0$ with impact reset map $\Delta_k(x^-) = x^+$, apply the exact **Saltation Matrix** (without which hybrid Floquet multipliers are mathematically wrong!):
   $$S_k = D\Delta_k(x^-) + \frac{\big(f^+(x^+) - D\Delta_k(x^-) f^-(x^-)\big)\,\nabla g_k(x^-)^T}{\nabla g_k(x^-)^T f^-(x^-)}$$
   **The GPU Warp Miracle:** Because $n_\Sigma \le 32$ for quadrupeds (32 dims), bipeds (24–32 dims), and wheel-quads (32–40 dims, 2 warps), **assigning lane $j \in \{0,\dots,31\}$ of a 32-thread warp to tangent column $j$** propagates the **ENTIRE $32 \times 32$ monodromy matrix $M(\theta) = \partial\Phi/\partial x_\Sigma$ in parallel alongside ONE nominal stride** (~200 substeps ≈ `0.4 ms` per morphology)!
3. **Certified Floquet Spectrum & Transient Amplification via `fs-spectral::floquet` (`E021`):**
   Feed $M(\theta)$ and phase-flow vector $g = \dot{x}^*(0)$ directly into `frankensim::fs-spectral::floquet` (plus `floquet::transient` for non-normal transient-growth sensitivity!). Extract:
   - Exact transverse spectral radius $\rho_\perp(\theta) = \max_{i \ne \text{phase}} |\lambda_i(M)|$ (orbitally stable iff $\rho_\perp < 1$);
   - Exact unstable eigenvector $v_{\max}$ (causal failure diagnosis: *"roll mode unstable at $\lambda = 1.18$, widen hip stance by +22 mm"*!).
4. **Implicit-Function-Theorem Morphology Gradients (Zero Backprop Through Long RL Horizons!):**
   At any converged fixed point $x_\Sigma^*(\theta)$, differentiate both performance $J(x_\Sigma^*, u, \theta)$ (CoT, speed, peak winding temperature) AND orbital stability $\rho_\perp$ analytically:
   $$\frac{dx_\Sigma^*}{d\theta} = \big(I - M(\theta)\big)^{-1} \frac{\partial \Phi}{\partial \theta}, \qquad \frac{\partial \lambda_i}{\partial \theta} = \frac{w_i^T \big(\frac{dM}{d\theta}\big) v_i}{w_i^T v_i}$$
   Optimizing $(\theta_{\mathrm{morph}}, u_{\mathrm{gait}})$ via damped Newton / SQP minimizing $\mathrm{CoT} + \gamma \max(0, \rho_\perp - 0.75)^2$ (plus transient-norm guard from `fs-spectral::floquet::transient`) finds **dynamically stable, minimum-energy morphologies and gaits in 1–4 seconds!**

#### Project Fit
Directly activates `frankensim::fs-spectral::floquet` (`E021`), `frob-adjoint`, `frob-bundle` (using §7.2 height-function curvature gaits as the initial shooting guess $u_0$!), and `O002` (512 morphologies running simultaneous warp-parallel monodromy evaluations in `<2 ms` per shooting iteration!).

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (Aerospace Periodic-Orbit / Passive-Dynamic Walker Floquet Theory) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Shooting method + monodromy Floquet multipliers ($\rho_\perp < 1$) | Certifies & optimizes orbital stability of spacecraft halos / biped walkers | `frob-codesign::poincare` + `fs-spectral::floquet` | 1–4 s Stage-1 morphology + gait stability optimizer | **Invariant:** Exact local exponential orbital stability of smooth/transverse-impact limit cycles. **Mismatch:** Fails at **grazing bifurcations** ($\nabla g \cdot f^- = 0$, denominator vanishes!) and **codimension-2 simultaneous multi-foot impacts** (handled by guard-transversality refusal + smooth-contact continuation). |
| Nonnormal pseudospectral transient-growth bound | Catches linearly stable orbits that blow up under tiny real perturbations | `fs-spectral::floquet::transient` | Guard against fragile high-condition-number monodromy matrices | **Invariant:** Direct API match (`E021`). |

#### Breakpoints
1. **Grazing bifurcations ($\nabla g \cdot f^- \to 0$) & simultaneous multi-foot touchdown corners:** Saltation denominator vanishes at grazing scuffing, and exact simultaneous 2-foot impacts produce Bouligand directional derivatives rather than one linear matrix. **Mitigation:** Detect $|\nabla g \cdot f^-| < \epsilon_{\mathrm{graze}}$ (emitting typed `FROB-REF-211 (GrazingImpactSingularity)`) and use either small $\Delta t = 1\text{ ms}$ staggered guard ordering or IPC smooth-contact continuation $\hat{d} \to 0$ during shooting!
2. **Periodic-only validity domain:** Does NOT apply to non-periodic cluttered obstacle avoidance, door opening, or sudden fall recovery from unseen grease patches. **Strictly scoped as Stage-1 cyclic locomotion & morphology pre-optimizer + warm-start generator for Stage-2 RL (`O004`/`O009`)!**
3. **Double-derivative cost of $\partial M / \partial \theta$:** Exact analytic $\partial M / \partial \theta$ needs 2nd-order dynamics derivatives; cheaply avoided via **forward-over-reverse or central complex-step/dual-number directional derivatives along top-$k$ levers** ($2p \approx 12$ extra warp evals = `<5 ms`!).

#### Dependencies
- `fs-spectral::floquet` + `floquet::transient` (`E021`), `frob-dyn` (LGVI + impact reset maps), `frob-adjoint`, `frob-bundle` (initial seed gaits), `O002` (warp-parallel tangent sweeps).

#### Second-Order Effects
- **Instantaneous Gait + Morphology Sensitivity Attribution:** Returns exact physical diagnoses (`d(rho_perp)/d(W_stance) = -1.4 /m`, `d(CoT)/d(L_tibia) = -0.28 /m`) in seconds instead of noisy RL finite differences.
- **Warm-Starts RL Policies:** Exporting the converged limit-cycle trajectory as a reference/imitation prior slashes Stage-2 RL training time by an additional **2×–4×**.

#### Asymmetric Value
- **Inner locomotion morphology evaluation time:** `15–45 min` (from-scratch PPO RL) → **`1–4 seconds`** (**200×–1,000× speedup** for cyclic locomotion morphology discovery!).
- **Structurally impossible in MuJoCo/Isaac/Genesis:** None provide Saltation-aware LGVI monodromy warps coupled to a verified Floquet spectral engine (`fs-spectral::floquet`) with implicit morphology differentiation.

#### Cost / Reversibility
- **Cost:** ~1,600 LOC in `frob-codesign::poincare` + tangent-warp lowering pass in `frob-codegen` (M4/M5).
- **Reversibility:** Fully modular Stage-1 accelerator; if a task is non-periodic (manipulation, cluttered navigation), Studio simply skips directly to `O004` (`FiberMorph`).

#### `⊞ Scale-Check`
- **State dimension on $\Sigma$:** 12-DoF quadruped = 36 full states − 3 ($SE(2)$ horizontal) − 1 (Poincaré section) = **32 dims = EXACTLY 1 GPU warp (32 lanes)!** 18-DoF hexapod = 44 dims = 2 warps.
- **Per-iteration GPU time across 512 morphologies:** 1 stride (200 substeps) × 1 warp/morph (512 warps = 4 warps/SM on 1× 4090, or 8 warps/SM across 2 shooting restarts) ≈ **`0.8–2.0 ms`**!
- **Floquet eigensolve (`fs-spectral::floquet`):** $32 \times 32$ dense nonsymmetric Schur/QR $\approx 3.3 \times 10^4$ flops × 512 morphologies = $1.7 \times 10^7$ flops ≈ **`3 ms`** on M5 Ultra / host CPU!
- **Full 25-iteration morphology-gait optimization across 512 designs:** $25 \times (2\text{ ms} + 3\text{ ms} + 12\text{ ms finite-diff } \partial M/\partial\theta) \approx$ **`0.45 seconds` total (targeting `<4 s` with safety margin)!**

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
Benchmark on a 12-DoF quadruped (`frob-quad-go2`) and 10-DoF biped (`frob-biped-cassie`): run `O003` to optimize $(L_{\mathrm{femur}}, L_{\mathrm{tibia}}, N_{\mathrm{knee}}, u_{\mathrm{trot}})$ for minimum CoT at $1.5\text{ m/s}$ with $\rho_\perp \le 0.75$. Verify (1) monodromy matches finite-difference perturbation to `<1e-5` on non-grazing strides, (2) predicted $\rho_\perp < 0.75$ survives a 50-stride perturbed rollout in full `frob-dyn`, and (3) wall time `<10 s` vs `>=15 min` PPO baseline.

#### Promotion & Retirement Thresholds
- **Promote:** Monodromy parity `<1e-4`, $\ge 90\%$ of `O003`-certified limit cycles survive 50-stride `0.15 m/s` push perturbations in full physics, and wall-time speedup $\ge 50\times$ vs RL on cyclic morphology optimization.
- **Retire / Fallback:** If grazing/simultaneous-impact noise breaks Newton shooting convergence on $>30\%$ of quadruped/biped morphologies even with smooth-contact continuation, downgrade `O003` to **analytic Floquet stability auditing of RL/MPC gaits** rather than direct morphology shooting.

---

### `O004` — `FiberMorph`: Buckingham-$\Pi$ Normalized $SE(3)$-Gauge-Equivariant Sheaf Foundation Policy for Zero-Shot Morphological Transfer

- **Canonical claim ID:** `MOR-A1-C004` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `STRUCTURALLY_MAPPED`
- **Target PQ / Paradox:** `PQ-10X-2` & `PQ-10X-1` · Breaks the **15-Minute Per-Edit RL Retrain Wall**
- **Need claims:** `E010`, `E012` (changing leg length, mass, or 4→6 legs forces full RL retraining in every existing simulator and even in v1's fixed-$\Gamma$ `EquivariantLinear` MLP!)

#### Mechanism
Replace fixed-dimension MLPs during rapid design exploration with a **Morphological Foundation Policy (`FiberMorph`)** formulated as a **Bidirectional Gauge-Equivariant Message-Passing Sheaf over the Operadic Kinematic Tree $G = (V, E)$** (mirroring Featherstone's exact inward/outward algebraic topology!):

1. **Buckingham-$\Pi$ Dimensionless Vertex Fibers (Scale & Mass Equivariance):**
   Every vertex $v \in V$ (joint, foot, wheel, sensor, hub) normalizes its state and physical properties by robot-wide dimensional invariants $(L_0 = \text{standing height},\; m_0 = m_{\mathrm{tot}},\; t_0 = \sqrt{L_0 / g},\; \tau_0 = m_0 g L_0)$:
   - **Scalar gauge-invariant features $s_v$:** normalized joint position $(q_v - q_{\min})/(q_{\max}-q_{\min})$, normalized speed $\dot{q}_v / \omega_{\mathrm{nl}}$, torque utilization $\tau_v / \tau_{\max}$, thermal margin $(T_{\max} - T_{w,v})/T_{\max}$, dimensionless link mass $m_v/m_0$, principal $\mathrm{Sym}^+(4)$ eigenvalues $\lambda_{1..4}(J_v)/(m_0 L_0^2)$, port type one-hot (`Revolute`, `Wheel`, `Foot`, `Arm`, `Hub`), local friction-margin $\mu \lambda_n - \|\lambda_t\|$.
   - **Covariant geometric fiber features $w_v \in \mathfrak{se}(3)^{C}$ & $f_v \in (\mathfrak{se}(3)^*)^{C}$:** local body-frame twists, accelerations, contact wrenches, and local ego-centric terrain height/normal patches expressed in vertex $v$'s own canonical `FrameGroupoid` frame!
2. **Exact Parallel-Transport Sheaf Restriction Maps (Mirroring Featherstone RNEA/ABA!):**
   For every directed edge $e = (u \to v)$ with current relative PGA motor $g_{vu}(q) \in \mathrm{Spin}^+(3,0,1)$:
   - **Inward Pass (Leaves $\to$ Hub, mirroring ABA inward wrench sweep):** Transforms child covariant wrench channels by the **exact coadjoint action $\mathrm{Ad}^*_{g_{vu}}$** and mixes scalar channels via gated MLP weights conditioned on invariant screw distance/angle of $g_{vu}$.
   - **Hub $O(3)$-Equivariant Port Self-Attention (Arbitrary Leg/Wheel Count Polymorphism!):** Aggregates across *any number* of attached operadic limb ports ($k = 2, 4, 6, 8$, homogeneous or mixed leg+wheel+arm!) using attention logits computed strictly from $O(3)$-invariant inner products of transported blades and scalar features.
   - **Outward Pass (Hub $\to$ Leaves, mirroring ABA outward acceleration sweep):** Broadcasts hub intent down every limb via the **exact adjoint action $\mathrm{Ad}_{g_{uv}}$**; each actuated vertex outputs a **dimensionless normalized action** $\hat{u}_v \in [-1, 1]$ (normalized torque fraction, impedance stiffness, or desired wheel slip velocity), rescaled by that actuator's physical limits!
3. **Automatic Universal Symmetry Inheritance (Theorem):**
   Because every vertex function is frame-canonical, every edge transport is exact $\mathrm{Ad}_g/\mathrm{Ad}^*_g$ (preserving polar vs axial $\det(R)$ signs automatically via PGA grades!), and hub attention is permutation-equivariant over ports, **`FiberMorph` is identically $\Gamma$-equivariant for EVERY finite morphological symmetry group $\Gamma \le O(3) \times \mathrm{Aut}(T_{\mathrm{kin}})$** ($\mathbb{Z}_2, V_4, D_4, D_6, D_8, C_n$) — **without ever recomputing Schur intertwining bases when topology changes!**
4. **Trained Once via `O002` (512-Morphology Megakernel) + Fast Zero-Shot / 15-Second Residual Adaptation:**
   Pre-train `FiberMorph` once on 512 diverse morphologies simultaneously (`O002`) across all 6 interior archetypes + outdoor terrains. On any brand-new agent-mutated morphology (lengthened legs, added 5th/6th leg, swapped wheels for feet), `FiberMorph` runs **ZERO-SHOT in 3 seconds** (or 15 s LoRA adapter fine-tune) to produce a full cluttered-world telemetry + causal autopsy report!

#### Project Fit
Lives in `frob-equiv::fibermorph` on `frankentorch` (`ft-nn`, `ft-kernel-metal`, `frob-gpu-cuda`). Directly pairs with `O002` (training data generator), `frob-lie` (exact PGA `Ad`/`Ad*` transport), and `O007` (compiled GPU CBF safety shield protecting zero-shot rollouts from out-of-distribution joint/thermal abuse!).

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (Gauge-Equivariant Mesh/Molecular GNNs — Cohen/Weiler/EGNN + Featherstone Tree Sweep) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Parallel transport $\rho(g_{vu})$ between local tangent frames on a manifold | Guarantees coordinate-free $SE(3)$ gauge equivariance across arbitrary meshes | Exact $\mathrm{Ad}_{g_{vu}}$ / $\mathrm{Ad}^*_{g_{vu}}$ PGA transport along kinematic tree edges | Transfers locomotion/manipulation physics across arbitrary link lengths, orientations, and leg counts | **Invariant:** Exact Lie-algebraic covariance (automatically preserves polar/axial $\det(R)$ signs). **Mismatch:** Tree depth varies across morphologies (3 hops for dog, 6 hops for humanoid arm) → requires $L = \text{depth}(T)$ dynamic message rounds (bounded $L \le 8$). |
| Dimensionless Buckingham-$\Pi$ similarity scaling (fluid/aerospace scaling laws) | Collapses geometrically similar machines of different sizes onto one universal curve | Robot-level $(L_0, m_0, t_0, \tau_0)$ feature normalization | Zero-shot scale transfer from 5 kg rover to 80 kg humanoid | **Invariant:** Exact for rigid-body gravity/inertia dynamics. **Mismatch:** **Actuator electrical/thermal timescales ($L/R, C_{\mathrm{th}} R_{\mathrm{th}}$) and tire friction do NOT scale with Froude number $\sqrt{L_0/g}$** — mitigated by explicitly feeding dimensionless thermal/electrical time-constant ratios $\tau_{\mathrm{th}}/t_0$ as scalar vertex invariants! |

#### Breakpoints
1. **Asymptotic specialist gap (5%–15%):** A generalist morphological policy cannot match the peak tuned performance of a specialist $\Gamma$-equivariant policy trained for 30 minutes on one frozen robot. **Explicit scoping:** `FiberMorph` is the **3-second evaluation engine for rapid design-space screening**; top 3 Pareto finalists always undergo specialist distillation!
2. **Severe topological OOD (e.g., first-ever 5-legged asymmetric spider or Cosserat tentacle):** Zero-shot score can exhibit high uncertainty. **Mitigation:** Attach an ensemble epistemic-uncertainty head (`fs-eproc` / variance across 4 heads); if zero-shot epistemic variance exceeds threshold, Studio automatically triggers a 30-second on-policy LoRA fine-tune before scoring!
3. **Non-Froude multiphysics scaling:** Thermal and battery dynamics break pure geometric similarity; must include explicit dimensionless thermal/electrical ratios in vertex embeddings (disclosed above).

#### Dependencies
- `O001` (typed operadic graph), `O002` (512-morphology GPU training megakernel), `frob-lie` ($\mathrm{Ad}_g, \mathrm{Ad}^*_g$), `frankentorch` (`ft-nn`, `ft-autograd`).

#### Second-Order Effects
- **Compounding Studio Data Flywheel:** Every morphology ever designed and specialist-trained in `frob-ledger` becomes an automatic distillation target for periodic `FiberMorph` updates — making zero-shot transfer monotonically sharper over the life of the Studio!

#### Asymmetric Value
- **Cluttered-world design evaluation latency per morphology mutation:** `15–45 min` (from-scratch PPO) → **`3 seconds` zero-shot** (`15–30 s` with LoRA polish) = **30×–300× iteration-speed jump**!
- **Rank-order fidelity:** Enables screening 100+ diverse topologies (4-leg vs 6-leg vs wheel-leg vs tracked) in **5 minutes** instead of **30 GPU-hours**!

#### Cost / Reversibility
- **Cost:** ~1,400 LOC in `frob-equiv::fibermorph` (M4/M5) + 1–2 GPU-hours pre-training budget on Dual RTX 4090 via `O002`.
- **Reversibility:** Fully additive; v1's specialist Schur-lemma `EquivariantLinear` (`§10.2`) remains untouched for final specialist policies and serves as the exact teacher/oracle benchmark!

#### `⊞ Scale-Check`
- **Network Size:** $N_v \le 48$ vertices, $C_s = 48$ scalar channels + $C_v = 8$ geometric $\mathfrak{se}(3)$ channels (96 total dims/vertex), 2 bidirectional tree sweeps + 2 hub attention layers $\implies$ **~185K parameters** (**370 KB in BF16** — 100% L2-resident; PCIe Gen4 dual-4090 gradient sync `< 0.05 ms`!).
- **Inference Compute:** ~0.48 MFLOPs/env × $B = 16,384$ envs = $7.86\text{ GFLOPs}$. On RTX 4090 BF16 Tensor Cores (330 TFLOPS/GPU, 15% utilization on small graph GEMMs ≈ 50 TFLOPS) $\implies$ **`~0.16 ms` per batched RL tick**; on M5 Ultra 80 cores (`simdgroup_matrix`) $\implies$ **`~0.45 ms` per tick**!

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
Train `FiberMorph` across 64 parametrically varied quadrupeds + 32 hexapods + 32 wheel-quads via `O002` (45 min on Dual 4090). Hold out 20 **unseen** morphologies (including +35% femur stretch, 2× mass scaling, and unseen 8-leg octopod + 2-wheel/2-leg hybrid). Measure: (1) zero-shot task success rate vs scratch PPO, and (2) **Spearman rank correlation $\rho_S$** between 3-second `FiberMorph` Pareto ranking and 20-minute ground-truth specialist PPO ranking!

#### Promotion & Retirement Thresholds
- **Promote:** Zero-shot unseen-morphology Spearman rank correlation $\rho_S \ge 0.85$ with specialist PPO rankings, and zero-shot success rate $\ge 75\%$ of specialist PPO on in-family mutations ($\ge 55\%$ on cross-leg-count topology jumps before 15 s LoRA).
- **Retire / Fallback:** If $\rho_S < 0.65$ (ranking uninformative for co-design screening), retire `FiberMorph` as a zero-shot evaluator and retain it strictly as a **warm-start weight initializer** for 3-minute specialist fine-tuning.

---

### `O006` — Exact Algebraic Inverse Mechanism Synthesis (Burmester & Study-Quadric Precision-Point Synthesis via `fsym-solvers::real_rur`)

- **Canonical claim ID:** `MOR-A1-C006` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `READY_FOR_TEST` (all core solver primitives verified live in `E024`!)
- **Target PQ / Paradox:** `PQ-10X-5` & `PQ-10X-1` · Flips `frankensympy` from passive lowering into **Constructive Geometric Synthesis**
- **Need claims:** `E010`, `E012` (blind trial-and-error guessing of 4-bar parallel leg/ankle/gripper linkages), `E024` (`fsym-solvers::real_rur` already verifies complete 0-D real root sets up to $n \le 8$!)

#### Mechanism
Instead of sampling random link lengths for closed-loop leg mechanisms (Cassie 4-bar legs, Unitree G1 parallel ankles, rocker-bogie rover suspensions, Ackermann steering bellcranks, parallel-jaw dexterous grippers), **solve algebraically for the EXACT complete set of all real linkages passing through target workspace poses**:

1. **Planar 5-Pose Burmester Synthesis (`n = 4 <= 8` — Exact Fit Inside `E024`!):**
   Given 5 target end-effector/foot poses $(p_k, \phi_k) \in SE(2)$ ($k = 0,\dots,4$ — e.g., touchdown, mid-stance, push-off, stair-riser clearance, high-step apex), the constant-length constraint $\| (R(\phi_k) x + p_k) - X \|^2 = \| (R(\phi_0) x + p_0) - X \|^2$ ($k = 1..4$) for fixed pivot $X=(X_1,X_2)$ and moving pivot $x=(x_1,x_2)$ yields **4 polynomial equations in 4 rational unknowns** over $\mathbb{Q}$!
   - Invoke `fsym-solvers::real_rur` (`VerifiedRealRurSystem`, `E024`): computes a checked grevlex Gröbner basis + checked FGLM conversion + Sturm real-root isolation returning **ALL real Burmester dyads** (guaranteed $\le 4$ real center-point pairs by Burmester's theorem, with a machine-verifiable completeness certificate proving ZERO valid linkages were missed!).
   - Pair any two real dyads ($\le \binom{4}{2} = 6$ exact 4-bar linkages) and filter algebraically in `<1 ms` by: (a) Grashof crank-rocker/double-rocker classification, (b) exact minimum transmission angle $\mu_{\min} \ge 35^\circ$, and (c) exact circuit/branch-defect absence (verifying all 5 poses lie on one connected real branch via discriminant sign parity!).
2. **Spatial Dyad Synthesis on the Study Quadric $\mathcal{Q}_S \subset \mathbb{P}^7$ (Direct `fs-ga::Motor` Isomorphism!):**
   **Conceptual Blend (`B8`):** A unit PGA motor $M = (q_0..q_3,\; e_0 q'_0..q'_3) \in \mathrm{Spin}^+(3,0,1)$ (`fs-ga::Motor`) **IS literally a homogeneous point on Study's quadric** $\mathcal{Q}_S: \sum_{i=0}^3 q_i q'_i = 0$!
   - For spatial parallel ankles (2-DoF U-2SPS / 2-RRU mechanisms on humanoids) or spatial RRS/SS leg dyads with 1–2 chassis-pinned pivot coordinates ($n \le 5$ unknowns), the sphere/plane kinematic constraints become linear/quadratic equations in the Study coordinates of target PGA poses $M_k$, directly solvable inside `fsym-solvers::real_rur` (`n <= 5 <= 8`, degree $\le 8$)!
3. **Direct Operadic Emission:** Each surviving exact linkage emits directly as a typed closed-loop module in `O001` (`(loop-module "burmester-ankle-b2" ...)`), pre-annotated with its exact symbolic transmission Jacobian and `fsym-proof-kernel` Gröbner completeness receipt!

#### Project Fit
Directly activates `frankensympy::fsym-solvers::real_rur` (`E024`), `fsym-proof-kernel::groebner`, and `fs-ga::Motor` Study coordinates. Exposes a first-class MCP tool `bot_synthesize_linkage(target_poses, min_transmission_deg, grashof_class)` so an AI agent *synthesizes* exact linkages from mission geometry rather than guessing!

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (Classical Algebraic Kinematics — Burmester 1888 / Study 1903 / Sommese–Wampler) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Burmester circle-point / center-point polynomial ideal | Enumerates all exact 4-bar linkages hitting 5 planar poses | `frob-symbolic::burmester` over `fsym-solvers::real_rur` | 50 ms exact closed-loop leg/ankle/gripper synthesis with completeness certificate | **Invariant:** Exact bijection with real roots of the 4-var ideal. **Mismatch:** Real robotic legs also have link collision and torque constraints — so exact Burmester solutions serve as **algebraically exact seeds** passed directly into `O008` feasibility & `O003` dynamic refinement! |
| Study quadric $\mathcal{Q}_S \subset \mathbb{P}^7$ representation of $SE(3)$ | Converts spatial rigid constraints into algebraic hypersurfaces | `fs-ga::Motor` 8-component PGA coordinates | Zero-conversion algebraic formulation of spatial parallel wrists/ankles | **Invariant:** Exact isomorphism $\mathrm{Spin}^+(3,0,1)/\{\pm 1\} \cong \mathcal{Q}_S \setminus E_\infty$. **Mismatch:** Unpinned 7-pose spatial SS ($n=6$, deg 20) can trigger rational coefficient bit-swell past `real_rur`'s `MAX_BITS = 4096` (`E024`) → cap exact RUR at $n \le 5$, deg $\le 8$, and use certified numeric homotopy continuation for deg $> 8$! |

#### Breakpoints
1. **Bit-length swell on unpinned spatial 6-variable systems (`E024` `MAX_BITS = 4096`):** Exact rational Gröbner bases on generic rational floats with 10-digit denominators swell rapidly! **Mitigation:** Quantize input target poses to bounded low-bit rationals (e.g., `1 mm` = $k/1000$ meters, `0.5 deg` rational Cayley half-tangents $t = \tan(\phi/2) \in \mathbb{Q}$ with denominator $\le 256$!) — slashing bit-growth by 10× while keeping geometric accuracy `<0.5 mm`!
2. **Branch / order / circuit defects:** A mathematically exact 4-bar hitting 5 poses may require disassembling the linkage between pose 2 and pose 3! **Mitigation:** Mandatory algebraic circuit-defect check (evaluating coupler-angle discriminant signs along the continuous interval) before emitting any `.fbot` module.
3. **4-pose / 3-pose underdetermined curves (positive-dimensional ideals):** `real_rur` refuses positive-dimensional systems (`E024`). **Mitigation:** For 4-pose synthesis (1-parameter Burmester cubic curve), sample $K = 16$ exact rational slices of the free angle parameter (turning 1-D curve into 16 instant 0-D `real_rur` solves in `<80 ms` total!).

#### Dependencies
- `fsym-solvers::real_rur` (`E024`), `fsym-proof-kernel::groebner`, `fs-ga::Motor`, `O001` (`frob-operad`).

#### Second-Order Effects
- **Unlocks Parallel-Actuated High-Performance Robots Automatically:** Humanoids (Unitree G1, Digit, Cassie) use closed-loop 4-bar and parallel-ankle mechanisms specifically to move heavy motor mass proximally (cutting leg inertia 3×!). Existing AI design loops almost NEVER discover parallel closed-loop legs because random mutations break loop closure 99.9% of the time. `O006` makes parallel-linkage discovery effortless!

#### Asymmetric Value
- **Closed-loop linkage synthesis:** `10,000+` failed blind optimization trials (hours) → **`5–80 ms` exact complete enumeration** of every valid real mechanism (**>1,000× speedup** + **previously impossible completeness guarantee**!).

#### Cost / Reversibility
- **Cost:** ~900 LOC in `frob-symbolic::inverse_kinematics` (M1/M3) — purely calling existing `fsym-solvers::real_rur`!
- **Reversibility:** Zero risk; purely additive constructive MCP/CLI synthesis tool.

#### `⊞ Scale-Check`
- **Planar 5-pose Burmester:** $n = 4$ vars ($\le 8$), $m = 4$ eqs ($\le 64$), max degree $3$ ($\le 256$), eliminant degree $\le 4$ (or 6 before factoring trivial solutions), rational poses quantized to 10-bit integers $\implies$ peak coefficient size $< 400\text{ bits}$ ($\ll 4,096$ cap), solve time **`5–40 ms` on single CPU core**!
- **Pinned Spatial 4-pose Parallel Ankle ($n=4$–$5$, deg $\le 8$):** peak bits `< 1,500`, solve time **`20–120 ms`**, memory `< 8 MB` ($\ll 512\text{ MB}$ `real_rur` cap).

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
Immediately in M1, run `fsym-solvers::real_rur` on 10 classic Burmester 5-pose planar leg/ankle benchmarks (including a Cassie-class 4-bar leg stride & a 2-DoF parallel humanoid ankle) using 10-bit rationalized poses; verify (1) `<100 ms` solve time, (2) 100% recovery of all known real Burmester dyads vs analytical ground truth (`<= 1e-9`), and (3) zero `MAX_BITS` / `MAX_COMPUTE` refusals.

#### Promotion & Retirement Thresholds
- **Promote:** $\ge 9/10$ benchmarks solve in `<150 ms` with verified `VerifiedRealRurSystem` receipts and zero undetected branch defects.
- **Retire / Fallback:** If rational bit-swell trips `MAX_BITS = 4096` on $>20\%$ of spatial 5-var ankle cases even with 10-bit rational poses, cap exact RUR strictly at planar 4-bar / 6-bar Burmester ($n \le 4$) and run certified interval-Newton homotopy continuation for spatial parallel mechanisms.

---

### `O007` — Proof-Carrying Zero-Overhead Runtime Safety Shields Compiled Directly Into CUDA PTX / Metal 4 MSL (`fs-sos` → `franken_lean` → `frob-kernel-ir`)

- **Canonical claim ID:** `MOR-A1-C007` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `STRUCTURALLY_MAPPED`
- **Target PQ / Paradox:** `PQ-10X-5` · Directly solves **Paradox 3 (The Verification-vs-Contact Chasm)**
- **Need claims:** `E010` (formal verification lives offline, disconnected from GPU RL and deployable controllers), `E022` (`fs-sos` already computes centered SOS + interval Cholesky), `E025` (`fsym-formal` needs new frozen profile), `E026` (`fln-anvil::egraph` + `fln-kernel`)

#### Mechanism
Bridge formal Lean proofs and chaotic multi-contact GPU simulation honestly using a **Two-Stratum Compiled CBF Safety Shield Pipeline**:

1. **Honest Two-Stratum Decomposition (Resolving Paradox 3 Without Claim Laundering `INV-001`):**
   - **Stratum 1 (Provably Safe Internal & Reduced-Order Kinodynamic Invariants):**
     - Per-joint electro-thermal & limit barrier: $h_i(q_i, \dot{q}_i, T_{w,i}) \ge 0$ (preventing hard-stop impacts, over-speed back-EMF faults, and winding thermal runaway $T_w > T_{\max}$);
     - Pairwise link-capsule self-collision barrier: $h_{ij}(q) = d_{\mathrm{cap}}(q)^2 - d_{\min}^2 \ge 0$;
     - Reduced-order 6-state centroidal capture-point / angular-momentum balance barrier $h_{\mathrm{bal}}(p, v, L) \ge 0$ under explicit bounded disturbance $\|d\| \le d_{\max}$.
   - **Stratum 2 (Unmodeled World Contact/Perception Shocks):** Grease patches, steam dropout, shifting payloads enter Stratum 1 strictly via the **explicit disturbance radius $d_{\max}$** (calibrated empirically by `frob-diagnose` — `Claim 3: ValidatedPhysics`), accompanied by a **live runtime guard** emitting `DisturbanceBoundExceeded` whenever observed residual acceleration exceeds $d_{\max}$!
2. **Constructive SOS Barrier Synthesis (`fs-sos`, `E022`) + Exact Closed-Form KKT Filter Lowering (`frankensympy`):**
   - `fs-sos` synthesizes polynomial CBFs $h_k(x)$ with centered positive-definite rational Gram witnesses $Q_k \succ 0$ proving that on $\{h_k \ge 0\}$, a control $u \in \mathcal{U}$ exists satisfying $\dot{h}_k(x, u, d) + \alpha_k(h_k(x)) \ge 0$ for all $\|d\| \le d_{\max}$.
   - **Why zero-overhead on GPU:** Rather than calling an iterative QP solver per environment at runtime, `frankensympy` symbolically derives the **exact closed-form minimal-intervention Sontag / analytic active-set projection** (exact closed form for decoupled joint barriers and $m \le 3$ coupled centroidal half-spaces!):
     $$u_{\mathrm{safe}}(x, u_{\mathrm{RL}}) = u_{\mathrm{RL}} + \Pi_{\mathrm{CBF}}(x, u_{\mathrm{RL}})$$
     which is **piecewise-smooth and differentiable** (allowing `frob-adjoint` / SHAC gradients to flow straight through the safety shield!).
3. **End-to-End Formal Kernel Certification (`franken_lean` + new `fsym-formal` profile `lean_core_qq_cbf_shield_v1`):**
   - Mint frozen `fsym-formal` profile `lean_core_qq_cbf_shield_v1` (`E025`) projecting the rational Gram matrix $Q_k = L_k D_k L_k^T$ AND the algebraic identity of $\Pi_{\mathrm{CBF}}$ into `franken_lean` (`fln-kernel`).
   - **Float32 Hardware-Rounding Soundness Bridge:** Compute an outward-rounded interval upper bound $\delta_{\mathrm{fp32}}$ on the `f32` evaluation error of $\Pi_{\mathrm{CBF}}$ over the compact box $\Omega$ using `fs-sos::IPoly` (`E022`), and prove in `fln-kernel` that the barrier holds with explicit strict margin $\ge \delta_{\mathrm{fp32}} + \|L_d h\|_{\max} d_{\max}$!
   - Use `fln-anvil::egraph` (`E026`) to extract a **kernel-checked rewrite proof** (`PO-CODEGEN-SHIELD-1`) showing the CSE-simplified `frob-kernel-ir` SSA shield equals the verified rational formula!
4. **Fused GPU Emission (`frob-codegen`):**
   Inline `Pi_CBF` directly into the **tail of the CUDA PTX / Metal 4 MSL control-to-physics kernel**! Every RL rollout step executes inside the formally verified safety shield at **<0.25 µs overhead** — AND the exact same verified Rust/C/PTX shield exports directly for hardware deployment!

#### Project Fit
Closes the entire FrankenSuite triangle: `fs-sos` (`E022`) → `frankensympy` (`fsym-formal`, `E025`) → `franken_lean` (`fln-anvil::egraph` + `fln-kernel`, `E026`) → `frob-codegen` → Metal 4 + Dual RTX 4090!

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (CompCert / Verified Runtime Monitors & Simplex Safety Architecture) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Proof-carrying compiled C/assembly monitor with explicit float rounding bounds (Flocq / CompCert) | Ensures compiled binary obeys the proved mathematical safety invariant | `frob-proof` + `fs-sos::IPoly` $\delta_{\mathrm{fp32}}$ margin + `fln-anvil::egraph` SSA equivalence | Certifies the actual lowered GPU `f32` safety filter, not just a paper ODE | **Invariant:** Strict downward preservation of barrier inequality modulo declared `MathMode::Safe` IEEE-754 semantics (`-fmad=false` / exact rounding bounds). **Mismatch:** `MathMode::Fast` breaks strict IEEE-754 bounds → **Rule:** Compiled shield sub-block ALWAYS emits IEEE-faithful instructions even inside `Fast` physics! |
| Control Barrier Function (CBF) minimal-intervention filter | Projects unsafe RL action onto safe set with minimum $\|u - u_{\mathrm{RL}}\|^2$ | Closed-form Sontag / active-set $m \le 3$ fused GPU tail | `<0.25 µs` per-step safe exploration across 32,768 envs | **Invariant:** Exact minimal projection for decoupled/small-active-set barriers. **Mismatch:** If $>4$ coupled nonlinear barriers activate simultaneously (conflicting constraints!), closed-form active-set enumerates $2^m$ faces and explodes → **Rule:** Decompose into orthogonal per-joint barriers + at most 3 coupled centroidal constraints, refusing conflicting simultaneous barrier sets cleanly (`FROB-REF-215`)! |

#### Breakpoints
1. **Simultaneous conflicting coupled CBFs ($m > 4$):** Closed-form KKT enumeration scales as $O(2^m)$ and conflicting constraints can have an empty safe action intersection ($\mathcal{U}_{\mathrm{safe}}(x) = \emptyset$). **Mitigation:** Prove **joint feasibility** (non-empty safe control set on $\{h \ge 0\}$) at synthesis time via `fs-sos`, and restrict coupled simultaneous barriers to $m \le 3$ (decomposing per-joint thermal/limit barriers orthogonally!).
2. **`MathMode::Fast` rounding drift:** Fast-math FMA contractions can invalidate tight `f32` rounding bounds. **Mitigation:** Emit the 130-op shield tail strictly with IEEE-754 no-contraction semantics (or conservative $4\times \delta_{\mathrm{fp32}}$ slack verified by `GATE-GPU-TWIN-01`).
3. **Epistemic misuse (Claim Laundering):** Users might falsely claim "Lean proved my robot can never fall in a kitchen." **Hard Architectural Guard (`INV-001`):** Every shield receipt embeds its exact `(model_hash, d_max, omega_box)` and increments a visible `disturbance_bound_exceeded_count` counter in `TelemetryBundle`.

#### Dependencies
- `fs-sos` (`E022`), new `fsym-formal` profile `lean_core_qq_cbf_shield_v1` (`E025`), `fln-anvil::egraph` + `fln-kernel` (`E026`), `frob-codegen`.

#### Second-Order Effects
- **2×–5× Faster Safe RL Convergence:** Eliminating catastrophic joint-limit / self-collision / thermal-shutdown episode terminations keeps RL trajectories in informative high-value regions from Epoch 1!
- **Deployable Safety Artifact:** The exact compiled shield (Rust `no_std` + Lean receipt) ships alongside the RL policy as a verifiable runtime safety wrapper!

#### Asymmetric Value
- **Online safety filter latency across 32,768 GPU envs:** `~100–500 µs` (batched iterative QP) → **`<0.25 µs` fused kernel tail** (**400×–2,000× faster**).
- **Capability impossible in MuJoCo/Isaac/Drake/Genesis:** End-to-end machine-checked `Lean 4` → `f32` GPU PTX/MSL compiled safety shield with zero C++ dependencies and zero claim laundering!

#### Cost / Reversibility
- **Cost:** ~1,500 LOC (`frob-proof::shield` + `fsym-formal` profile + `frob-codegen` tail fusion, M4).
- **Reversibility:** Feature-flagged shield wrapper around action input (`act_safe = shield(state, act_raw)`); disabling it restores raw actions cleanly.

#### `⊞ Scale-Check`
- **Synthesis + Proof Time (Compile Time, Once Per Morphology):**
  - 12 per-joint 3-state degree-4 CBFs ($\binom{5}{2}=10$ monomials, $10\times 10$ Gram): $12 \times 15\text{ ms} = 180\text{ ms}$ SDP + $12 \times 30\text{ ms} = 360\text{ ms}$ `fln-kernel` (parallelized across cores: **`<150 ms` wall time**!).
  - 1 centroidal 6-state degree-4 CBF ($\binom{8}{2}=28$ monomials, $28\times 28$ Gram): `~350 ms` SDP + `~750 ms` `fln-kernel` = **`~1.1 s`**!
- **GPU Runtime Cost (Per Env-Step):** ~110 FMA + 17 `max`/`div` ops = **~130 ops/env**; at `B = 32,768` envs on 1× RTX 4090 (82 TFLOPS FP32) = $4.26\text{ MFLOPs}$ = **`<0.25 µs`** (register-resident, zero extra global memory reads!).

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
Synthesize a 3-state winding-thermal + joint-limit CBF shield for `frob-quad-go2`'s knee actuator and a 6-state capture-point CBF shield for `frob-g1-29dof`. Verify: (1) `fln-kernel` accepts the `lean_core_qq_cbf_shield_v1` certificate in `<2 s`, (2) fused CUDA/Metal kernel incurs `<5%` step-time overhead at `B = 8,192`, (3) **0.0% thermal/joint-limit violations** under 100,000 adversarial random torque episodes whenever $\|d\| \le d_{\max}$, and (4) `DisturbanceBoundExceeded` fires deterministically on injected shocks $\|d\| = 1.5\, d_{\max}$!

#### Promotion & Retirement Thresholds
- **Promote:** `<2 s` synthesis+proof time, `<5%` GPU overhead, `0` Stratum-1 violations within $d_{\max}$ across 100k adversarial rollouts, and $\ge 1.5\times$ faster safe-RL convergence on thermal/stair tasks.
- **Retire / Fallback:** If `fs-sos` centered SDP fails to find a strictly feasible rational margin above $\delta_{\mathrm{fp32}}$ on $>25\%$ of benchmark robots, restrict compiled GPU shields to **exact analytical quadratic/barrier joint-thermal-limit shields** (always solvable in closed form!) while keeping 6-state centroidal SOS certificates offline.

---

### `O008` — Two-Sided Certified Feasible Design Polytopes & Semi-Algebraic Envelopes (`frob-envelope` via `fs-sos` + `fln-anvil::omega`)

- **Canonical claim ID:** `MOR-A1-C008` (provisional)
- **Claim type:** `design_option`
- **Canonical status:** `PROPOSED`
- **Option stage:** `STRUCTURALLY_MAPPED`
- **Target PQ / Paradox:** `PQ-10X-5` & `PQ-10X-2` · Flips `franken_lean` & `fs-sos` from post-hoc refusal checkers into **Pre-Search Feasible-Manifold Compilers**
- **Need claims:** `E010`, `E011` (§14.3 preflight still rejects infeasible mutations *after* proposing them), `E022` (`fs-sos`), `E026` (`fln-anvil::omega` linear arithmetic prover!)

#### Mechanism
Stop letting AI agents, CMA-ES, or Bayesian Optimization waste 60%–90% of morphology proposals bouncing off coupled nonlinear thermal/electrical/geometric feasibility walls! Compile the **Certified Two-Sided Feasibility Sandwich** $\mathcal{P}_{\mathrm{in}} \subseteq \mathcal{F} \subseteq \mathcal{P}_{\mathrm{out}}$ *once per parameterized operad template*:

1. **Symbolic Semi-Algebraic Feasibility System (`frankensympy`):**
   Given a parameterized `.fbot` template with $p \le 8$ continuous levers $\theta = (L_f, L_t, N_k, W_s, R_w, m_{\mathrm{pay}}, \dots)$ and mission specs (payload $M_{\mathrm{load}}$, ramp slope $\alpha_{\mathrm{ramp}}$, doorway bottleneck $W_{\mathrm{door}}$ from `fs-topo`, continuous speed $v_{\mathrm{req}}$), `frankensympy` emits the exact rational/polynomial constraint set $\mathcal{F} = \{\theta \in \mathcal{B} : g_1(\theta) \ge 0, \dots, g_m(\theta) \ge 0\}$ (cleared of positive denominators!):
   - Winding thermal steady-state inequality ($P_{\mathrm{Cu}}(\theta) \le \Delta T_{\max}/R_{\mathrm{th}}$);
   - Back-EMF speed-torque bus voltage inequality ($V_{\mathrm{bus}} \ge K_t N \omega + R_w \tau / (N K_t)$);
   - Geometric doorway clearance, belly-out ground clearance (`GB-09` Bekker–Wong sinkage bound!), and Grashof non-singularity inequalities;
   - Static tip-over ZMP margin inequality.
2. **Certified Inner Inscribed Polytope/Ellipsoid $\mathcal{P}_{\mathrm{in}} \subseteq \mathcal{F}$ (100% Zero-Rejection Sampling Lane):**
   `fs-sos` (`E022`) solves a centered SOS Putinar containment SDP maximizing $\log\det(\mathbf{E})$ for ellipsoid $\mathcal{P}_{\mathrm{in}} = \{\theta_0 + \mathbf{E} z : \|z\|_2 \le 1\}$ such that:
   $$g_i(\theta) = s_{i,0}(\theta) + s_{i,1}(\theta)\big(1 - \|\mathbf{E}^{-1}(\theta - \theta_0)\|_2^2\big), \qquad s_{i,0}, s_{i,1} \in \Sigma[\theta]$$
   Verified by `fs-sos::verify` interval Cholesky + `franken_lean` (`fln-kernel`)!
3. **Certified Outer Bounding Polytope $\mathcal{P}_{\mathrm{out}} \supseteq \mathcal{F}$ (Zero-False-Exclusion Pruning Lane!):**
   To avoid the conservatism trap where optimal designs sit on non-convex corners outside $\mathcal{P}_{\mathrm{in}}$, `fs-sos::global` (Lasserre/Putinar lower-bound relaxation, `E022`) + `fln-anvil::omega` (`E026`, exact linear-arithmetic polyhedral tightening!) compute certified supporting half-spaces $a_r^T \theta \le b_r$ valid for **EVERY** $\theta \in \mathcal{F}$ — mathematically proving that **NO feasible robot exists outside $\mathcal{P}_{\mathrm{out}}$!**
4. **Compiled Sampling Bijection for Studio & GPU Sweeps:**
   Export $(\mathcal{P}_{\mathrm{in}}, \mathcal{P}_{\mathrm{out}})$ to `frob-codesign` and `frob-mcp` (`bot_feasible_envelope`):
   - Initial 512-morphology GPU populations (`O002`) sample directly from $\mathcal{P}_{\mathrm{in}}$ via $T_{\mathrm{cert}}(z) = \theta_0 + \mathbf{E} z$ (**100% valid on Sample 1, 0 preflight rejections!**);
   - Boundary refinement explores ONLY the thin shell $\mathcal{P}_{\mathrm{out}} \setminus \mathcal{P}_{\mathrm{in}}$ (pruning 70%–95% of dead hypercube volume with **0.0% risk of excluding true Pareto optima**!).

#### Project Fit
Unites `frankensympy` (clearing denominators into exact polynomials), `fs-sos::program` + `fs-sos::global` (`E022`), `fln-anvil::omega` + `fln-kernel` (`E026`), and `frob-codesign` / `frob-mcp`.

#### Structural Transfer (`B6` Audit — Classification: `STRUCTURALLY_MAPPED`)

| Source Element (IRIS / Putinar Semi-Algebraic Region Certification & Branch-and-Bound Tightening) | Source Role | Target Element in `franken_robotics` | Target Role | Invariant / Mismatch |
|---|---|---|---|---|
| Maximal certified inscribed ellipsoid in C-free space (IRIS-NP / SOS containment) | Guarantees collision-free convex region for fast sampling/planning | Certified inner design ellipsoid $\mathcal{P}_{\mathrm{in}} \subseteq \mathcal{F}$ | Guarantees 100% thermally/electrically/geometrically valid initial design batches | **Invariant:** Exact Putinar Positivstellensatz containment. **Mismatch:** Single ellipsoid under-covers non-convex banana-shaped feasibility sets (e.g., hyperbolic $L_f / N \le c$) → **Resolved by Pairing with Outer $\mathcal{P}_{\mathrm{out}}$ AND Log-Coordinate Recoding** ($\tilde{\theta} = \ln\theta$ turns monomial/posynomial thermal & gear ratios into **exact convex linear/exponential inequalities** — Geometric Programming `𝓛 Recode`!!). |

**WAIT — LOOK AT THAT `𝓛 Recode` BONUS EMBEDDED RIGHT THERE:**
In log-lever coordinates $y_i = \ln \theta_i$ (positive link lengths, gear ratios, wheel radii, masses), static torque $\tau \propto L_f$, copper loss $P_{\mathrm{Cu}} \propto L_f^2 N^{-2}$, and back-EMF speed $\omega \propto N^{-1} R_w^{-1}$ are **posynomials**! Under `y = ln(theta)`, EVERY posynomial feasibility constraint becomes **EXACTLY CONVEX** — meaning the inner/outer certified sandwich becomes **near-tight (<5% volume gap!)** in log-lever space!

#### Breakpoints
1. **Curse of lever dimensionality ($p > 8$):** Degree-4 SOS monomial basis grows as $\binom{p+2}{2}$ ($p=8 \implies 45$; $p=16 \implies 153$, slow SDP!). **Mitigation:** Restrict full SOS sandwiching to the **top $p \le 8$ coupled mission-critical levers** (fixing minor parameters at nominal or bounding them with interval arithmetic `IPoly`), or exploit **log-coordinate posynomial convexity** which scales cleanly to $p = 32$ via linear/exponential-cone cutting planes checked by `fln-anvil::omega`!
2. **Dynamic vs static feasibility gap:** Static/nominal-ramp thermal and torque inequalities are *necessary* conditions (and sufficient for static standing/steady climb), NOT sufficient for violent dynamic impacts/sprinting! **Honesty rule (`INV-001`):** Certificate explicitly labels $\mathcal{P}_{\mathrm{in}}$ as **"Statically & Steady-Mission Certified"** — never claiming dynamic-impact feasibility before `O003`/`O002` simulation!

#### Dependencies
- `frankensympy`, `fs-sos` (`E022`), `fln-anvil::omega` + `fln-kernel` (`E026`), `frob-preflight`, `frob-codesign`.

#### Second-Order Effects
- **Instant Infeasibility Proofs for Impossible Mission Briefs:** If $\mathcal{P}_{\mathrm{out}} = \emptyset$ (certified by `fln-anvil::omega` / Farkas/Positivstellensatz refutation!), `frob-mcp` tells the AI agent in **`<500 ms`**: *"Mathematically IMPOSSIBLE for any choice of $(L_f, N_k, W_s)$ in this motor class to climb a $25^\circ$ ramp carrying 15 kg under 120°C — minimum feasible motor $K_t/\sqrt{R_w}$ must increase by $+38\%$!"* — saving hours of hopeless search!

#### Asymmetric Value
- **Feasible-sample yield in coupled multi-physics lever spaces:** `10%–35%` (naive hypercube sampling) → **`100%` inside $\mathcal{P}_{\mathrm{in}}$** and **3×–10× smaller search volume** inside $\mathcal{P}_{\mathrm{out}}$ (**5×–15× higher effective morphology-search sample efficiency** + exact impossibility proofs!).

#### Cost / Reversibility
- **Cost:** ~1,100 LOC in `frob-preflight::envelope` (M3/M5).
- **Reversibility:** Fully additive; falls back to standard hypercube + `frob-preflight` rejection if $p > 8$ non-posynomial constraints exceed SDP budget.

#### `⊞ Scale-Check`
- **Problem size:** $p = 6$ levers, $m = 12$ degree-4 constraints (or $p = 16$ in log-posynomial linear-cone form).
- **SOS SDP + Lean check time:** 12 blocks of $7\times 7$ to $28\times 28$ in `fs-sos` = **`0.4–2.0 s`** once per template + **`<0.5 s`** `fln-anvil::omega` + `fln-kernel` certification!
- **Runtime sampling throughput:** `T_cert(z)` is one $6\times 6$ matvec = 36 FMAs → **>10M guaranteed-feasible samples/s** on CPU/GPU!

#### Cheapest Falsifying Test (`✂ Exclusion-Test`)
Test on `galley-runner-v3` (Appendix D.1: 5 levers $L_f, L_t, R_w, N_k, W_s$ + CommercialKitchen Tier-3 doorway + $20^\circ$ stair climb thermal limit + 48 V bus): verify (1) `O008` compiles $(\mathcal{P}_{\mathrm{in}}, \mathcal{P}_{\mathrm{out}})$ in `<3 s`, (2) **100.0%** of 10,000 samples from $\mathcal{P}_{\mathrm{in}}$ pass full `frob-preflight` (`0` refusals vs `>60%` refusal rate in raw box!), (3) **0.0%** of 10,000 samples outside $\mathcal{P}_{\mathrm{out}}$ pass `frob-preflight` (zero false exclusions!), and (4) raising payload to an impossible 40 kg emits a certified empty-set impossibility proof in `<1 s`.

#### Promotion & Retirement Thresholds
- **Promote:** `100%` inner-set preflight validity, `0%` outer-set false exclusions, $\ge 70\%$ dead-box volume pruned by $\mathcal{P}_{\mathrm{out}}$, and `<3 s` compile time.
- **Retire / Fallback:** If non-convex SOS inner ellipsoid captures `<15%` of feasible volume even in log-lever coordinates, ship **outer pruning $\mathcal{P}_{\mathrm{out}}$ + impossibility proofs** only and use hit-and-run rejection sampling inside $\mathcal{P}_{\mathrm{out}}$.

---

## 4. Mechanism Recombination (`O009`) & Capability Flywheel (`B8 Conceptual Blend`)

### Combined Option `O009` — The 10× Algebraic Co-Design Cascade ("Synthesize → Envelope → Limit-Cycle → Zero-Shot → Certify")

- **Canonical claim ID:** `MOR-A1-C009` (provisional) · **Claim type:** `design_option` · **Status:** `PROPOSED` · **Stage:** `STRUCTURALLY_MAPPED`
- **Inputs Combined:** `O001` (Operadic Module Algebra) + `O002` (512-Morphology Warp-Homogeneous Megakernel) + `O003` (Warp-Parallel Poincaré/Floquet Optimizer) + `O004` (`FiberMorph` Zero-Shot Sheaf Policy) + `O006` (Burmester/Study Exact Linkage Synthesis) + `O007` (Compiled Verified GPU CBF Shields) + `O008` (Certified Feasible Design Envelopes)
- **New Capability:** An autonomous AI agent transforms a high-level mission brief (*"design a stair-climbing kitchen-delivery robot under 18 kg, 75 cm aisle, 120 W avg power, surviving wet $\mu=0.11$ grease"*) into a **Pareto frontier of 500+ dynamically evaluated, thermally/electrically feasible, safety-shielded multi-topology robots in <10 minutes** — instead of evaluating 4 designs/hour!
- **Why Neither Alone Provides It (True Non-Additive Synergy):**
  - `O006` + `O008` without `O002`/`O003` synthesize statically feasible linkages that still take hours of RL to dynamically evaluate;
  - `O002` without `O008` wastes 70% of its 512 GPU morphology slots simulating thermally/electrically dead robots;
  - `O004` (`FiberMorph`) without `O002` cannot be trained across 512 diverse topologies efficiently, and without `O007` (compiled CBF shields) suffers destructive out-of-distribution joint/thermal crashes on unseen morphologies;
  - `O003` without `O004` cannot evaluate non-periodic cluttered kitchen navigation, whereas `O004` without `O003` lacks millisecond-scale analytic orbital stability gradients!
- **Interface & Invariant Compatibility Audit:**
  - Single typed currency across all 7 stages: the **Colored Operad `.fbot` Graph (`O001`)** with `Sym^+(4)` inertias, `FrameGroupoid` tags, and explicit `Claim 1..5` provenance (`INV-001`).
  - Zero claim laundering: `O006`/`O007`/`O008` emit `VerifiedFormal` (model-scoped), `O002`/`O003` emit `VerifiedNumeric` (hardware-twin gated), `O004` emits `EvaluatedDesign` (CRN + e-process gated), and real-world friction calibration stays `ValidatedPhysics` (`Claim 3`)!
- **New Failure Modes Unique to the Cascade:**
  1. **Cascaded Filter Bias (Goodhart's Law across stages):** If `O008` (static envelope) or `O003` (cyclic flat-floor limit cycle) or `O004` (zero-shot policy) systematically penalizes an unconventional morphology (e.g., a tracked flipper-bot that is slow on flat floors but dominant on stairs), the cascade could prematurely prune the true global optimum!
     - **Hard Architectural Mitigation:** Enforce a **15% Diversity Exploration Quota** (passing top-diversity archetypes from every operadic family directly to Stage-2 `FiberMorph` + short specialist polish) and log cross-stage rank-inversion diagnostics (`frob-diagnose`).
  2. **Stage-handoff friction:** If each stage uses incompatible state/parameter encodings, glue complexity kills iteration speed. **Mitigation:** One shared `ParamBank` + `OperadModuleIR` layout in `frob-bot`/`frob-operad`.
- **End-to-End `⊞ Scale-Check` (10-Minute Autonomous Design Campaign on Dual RTX 4090 or M5 Ultra):**
  - `t = 0.0–2.5 s`: `O006` synthesizes exact parallel leg/ankle linkages (`40 ms`) + `O008` compiles $(\mathcal{P}_{\mathrm{in}}, \mathcal{P}_{\mathrm{out}})$ across 4 candidate operadic families (`2.0 s`) + `O001` stitches pre-compiled limb sub-kernels & modular Lean proofs (`25 ms`).
  - `t = 2.5–30 s`: `O002` + `O003` evolve **512 guaranteed-feasible morphologies simultaneously** through 25 warp-parallel Poincaré/Floquet stability + CoT iterations (`<10 s` compute + overhead).
  - `t = 30–90 s`: `O007` compiles per-family verified GPU CBF shields (`1.5 s`); `O004` (`FiberMorph`) evaluates top **64 Pareto + diverse morphologies** zero-shot across 64 cluttered `CommercialKitchen` interiors (`B = 4,096` envs, 60-second simulated missions → **~30 s wall time!**) with full causal failure autopsies (`frob-diagnose`).
  - `t = 1.5–10 min`: Top **4 Pareto finalists** run e-raced specialist $\Gamma$-equivariant PPO/SHAC fine-tuning on Dual 4090 (warm-started from `O003` gaits + `O004` policies, shielded by `O007`!).
  - **Net Result:** **512 dynamically screened + 64 cluttered-world evaluated + 4 specialist-perfected certified robots in 10 minutes** vs **0.6 robots in 10 minutes** in v1 / **0.1–0.3 robots** in Isaac Lab/MJX (**>25×–100× real design-discovery throughput**!).

### Capability Flywheel (Per `CREATIVE_APPLICATIONS.md` §9)
1. Running `O009` design campaigns in `ATELIER`
2. produces verified operadic modules (`O001`), exact Burmester primitives (`O006`), specialist policies, and `CrashPack` causal autopsies persisted in `frob-ledger`
3. improves the pre-certified module library AND periodically distills new specialist policies back into `FiberMorph` (`O004`)
4. lowers compile latency, zero-shot epistemic error, and specialist fine-tuning steps per new robot class
5. increases autonomous design throughput and morphological diversity explored per GPU-hour!
- **Weakest link:** Zero-shot rank correlation $\rho_S$ of `FiberMorph` (`O004`) on radically novel asymmetric topologies; if $\rho_S$ degrades under distribution shift, screening misranks finalists.
- **Delay:** `FiberMorph` retraining/distillation refresh occurs asynchronously every ~20–50 completed specialist campaigns (~1–2 GPU-hours background run on `trj`).
- **Saturation / negative feedback:** Diminishing returns once library covers ~20 core limb archetypes and zero-shot reaches ~90% of specialist asymptotic return (irreducible ~5–10% specialist tailoring gap remains).
- **Pilot instrumentation:** Track (a) `rank_correlation_stage1_vs_specialist` ($\rho_S$), (b) `preflight_rejection_rate` (target `<5%` with `O008`), (c) `verified_pareto_hypervolume_per_gpu_hour`, and (d) `stage_filter_false_elimination_rate` (measured via held-out audit of 10% randomly promoted Stage-1 rejects!).

---

## 5. Bounded Tail Hypothesis Screen (`O010` — Scoped Local Bilinear Lie-Koopman Residual)

After explicitly **KILLING (`†`)** global cross-contact Koopman/EDMD MPC (`H-203a`), we preserve **one** strictly scoped asymmetric tail hypothesis (`MOR_CASE` / `CREATIVE_APPLICATIONS.md` §5):

- **Option ID / Provisional Claim ID:** `O010` / `MOR-A1-C010` (`Canonical status: PROPOSED`, `Option stage: GENERATED`, **`Tail Hypothesis — max 1 slot`**)
- **Hypothesis:** Within a **single smooth contact mode** on non-analytic deformable media (wet clay Bekker–Wong rutting, granular gravel, soft Cosserat manipulator contact), a low-dimensional ($n \le 12 \to d \le 48$) $SE(3)$-equivariant bilinear Koopman residual operator $\dot{\Psi} = K \Psi + \sum u_j B_j \Psi$ trained online from 200 GPU rollouts can accelerate centroidal MPC over deformable terrain by $\ge 5\times$ vs sampling-based MPPI while preserving $<5\%$ trajectory error.
- **Mechanism:** Lift only the 12-dim centroidal body-frame twist + slip + sinkage state $(v_b, \omega_b, z_{\mathrm{sink}}, \text{slip}) \in \mathbb{R}^{12}$ into $d = 48$ $SE(2)$-invariant / $SO(3)$-covariant observables via `fnp-linalg` SVD/EDMD; solve a tiny 48-state linear-time-varying QP inside `frob-control`!
- **Verified facts:** `fnp-linalg` SVD (`E007`), `frob-terramech` Bekker–Wong non-smooth soil (`E011` §15.2, `GB-09`), RTX 4090 / M5 Ultra fast batched rollouts (`E001`, `E002`).
- **Speculative parts:** Whether 48 bilinear observables maintain accuracy across variable moisture/sinkage depth without re-fitting every 2 seconds.
- **Upside / avoided loss:** Unlocks fast real-time convex MPC on deformable outdoor mud/sand where exact analytical contact gradients do not exist cleanly.
- **Testing cost / downside:** **Bounded (`~350 LOC`, 2 days work in M6)**; completely off M0–M5 critical path.
- **Disconfirmers:** Fails if 48-dim bilinear prediction error exceeds `15%` over a 0.5 s horizon on wet clay (`GB-09`), OR if GPU-batched MPPI already beats it in both tracking error and wall time.
- **Cheapest test:** Benchmark on `frob-tracked-rover` climbing variable wet-clay slopes (`GB-09`) comparing 48-dim bilinear Koopman MPC vs 1,024-rollout GPU MPPI.
- **Promotion threshold:** `<5%` 0.5 s prediction error AND $\ge 3\times$ compute speedup over equal-accuracy GPU MPPI.
- **Retirement threshold:** `>15%` prediction error OR slower/worse tracking than GPU MPPI → **retire cleanly** (zero architectural damage).

---

## 6. Architectural Integration Map (Concrete Deltas to v1 Plan)

Every surviving mechanism compiles into named Rust crates, proof obligations, and falsifiable gates while preserving M0–M4 critical-path discipline (`§0.7` convergence-before-breadth):

| Option | New / Extended Crate(s) | New Lean Proof Obligation / Formal Profile | New Gate & Kill Gate | Milestone Placement |
|---|---|---|---|---|
| **`O001`** Operadic Robot Algebra & Port-ABI | New `frob-operad` (`K1`, ~1,800 LOC); extends `frob-bot`, `frob-codegen` | `PO-OPERAD-COMPOSE-1` (functorial passivity + $\mathrm{Sym}^+(4)$ + frame closure in `<5 ms`) | `GATE-OPERAD-01` / `KILL-OPERAD-1` (`>=0.90×` monolithic GPU speed, `<50 ms` stitch) | **M1** (AST+Lean) → **M2** (Port-ABI GPU) |
| **`O002`** 512-Morphology Warp Megakernel | Extends `frob-gpu`, `frob-gpu-cuda`, `frob-gpu-metal` (~2,200 LOC) | `GATE-GPU-TWIN-01` extended to heterogeneous 128-morph parity matrix | `GATE-HETERO-GPU-01` / `KILL-HETERO-1` (`>=0.70×` homogeneous speed across 128+ morphs) | **M2** (Level-A SoA params) → **M4/M5** (Level-B multi-topo megakernel) |
| **`O003`** Poincaré/Floquet Limit-Cycle Opt | New `frob-codesign::poincare` (~1,600 LOC); consumes `fs-spectral::floquet` (`E021`) | `VerifiedNumeric` monodromy + `fs-spectral::floquet` phase-neutrality check | `GATE-FLOQUET-01` / `KILL-FLOQUET-1` (`<10 s` cyclic opt, `>=90%` 50-stride perturbation survival) | **M4/M5** (Stage-1 co-design accelerator) |
| **`O004`** `FiberMorph` Sheaf Foundation Policy | New `frob-equiv::fibermorph` (~1,400 LOC) on `frankentorch` | `PO-FIBER-EQUIV-1` (exact $\mathrm{Ad}_g/\mathrm{Ad}^*_g$ gauge covariance test) | `GATE-FIBERMORPH-01` / `KILL-FIBER-1` (unseen-morph Spearman $\rho_S \ge 0.85$) | **M5** (trained on `O002`) |
| **`O006`** Burmester/Study Exact Synthesis | New `frob-symbolic::inverse_kinematics` (~900 LOC); consumes `fsym-solvers::real_rur` (`E024`) | `fsym-proof-kernel::groebner` completeness receipt | `GATE-BURMESTER-01` / `KILL-BURMESTER-1` (`>=9/10` benchmarks `<150 ms`, `0` branch defects) | **M1** (planar 4-bar) → **M3** (pinned spatial ankles) |
| **`O007`** Compiled Verified GPU CBF Shields | New `frob-proof::shield` (~1,500 LOC) + `fsym-formal` profile `lean_core_qq_cbf_shield_v1` | `PO-SHIELD-CBF-1` + `PO-CODEGEN-SHIELD-1` (`fln-anvil::egraph` + `IPoly` $\delta_{\mathrm{fp32}}$) | `GATE-SHIELD-GPU-01` / `KILL-SHIELD-1` (`<5%` GPU overhead, `0` Stratum-1 violations within $d_{\max}$) | **M4** (joint/thermal + 6-state centroidal) |
| **`O008`** Certified Design Envelopes $(\mathcal{P}_{\mathrm{in}}, \mathcal{P}_{\mathrm{out}})$ | New `frob-preflight::envelope` (~1,100 LOC); consumes `fs-sos` (`E022`) + `fln-anvil::omega` (`E026`) | `PO-ENVELOPE-1` (inner containment + outer impossibility certificates) | `GATE-ENVELOPE-01` / `KILL-ENVELOPE-1` (`100%` inner valid, `0%` outer false exclusions, `>=70%` pruned) | **M3/M5** |
| **`O010`** Local Bilinear Lie-Koopman Tail | Optional `frob-control::koopman_residual` (~350 LOC, feature-flagged) | `EvaluatedDesign` only | Tail gate: `<5%` 0.5 s wet-clay error & `>=3×` faster than equal-accuracy MPPI | **M6** (strictly off critical path) |

**Explicit Kill Ledger (Saved Engineering Months):**
- `† H-101b`: Runtime $\infty$-operad / nonlinear hybrid double-category wrappers — **KILLED** (Delassus impact coupling + Paradox 1 overhead).
- `† H-102a`: Full high-dimensional Morse-Conley contact stratification — **KILLED** ($3^k$ exponential blowup; replaced by tiny $\Gamma$-quotiented Cayley automaton).
- `† H-201a/b`: Single-thread CSR interpreter & monolithic 60-DoF dense super-tree masking — **KILLED** (warp divergence / 80% register & flop waste; replaced by warp-homogeneous `O002`).
- `† H-203a`: Global cross-contact Koopman/EDMD MPC & morphology extrapolation — **KILLED** (discontinuous impact Gibbs ringing + 6M-param quadratic blowup + zero morphology map; scoped down to tiny tail `O010`).
- `† H-501a`: Unbounded high-DoF whole-robot spatial Gröbner inverse design — **KILLED** (exceeds `E024` `MAX_VARIABLES = 8`; scoped to exact operadic primitives `O006`).
- `† H-502a`: End-to-end formal Lean proofs of unstructured real-world contact chaos — **KILLED** (undecidable & violates `INV-001`; resolved honestly via Two-Stratum `O007`).
