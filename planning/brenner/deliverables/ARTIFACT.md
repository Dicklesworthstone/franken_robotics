# BrennerBot Synthesis Artifact — `franken_robotics` 10× Breakthrough Verdict (`RS-20261121-franken-robotics-10x`)

**Session ID:** `RS-20261121-franken-robotics-10x`  
**Question of Record:** `Q-001` ([question_of_record.md](file:///Users/jemanuel/projects/franken_robotics/planning/brenner/intake/question_of_record.md))  
**Disagreement Register:** [disagreement_register.md](file:///Users/jemanuel/projects/franken_robotics/planning/brenner/distillations/disagreement_register.md)  
**Cross-Model Evidence Base:** 30×3 = 90 generated candidates across 3 independent model families (`CC` Claude Opus 5.5, `COD` OpenAI GPT-6.2-Sol, `AGY` Gemini 4 Neon), 16 independently audited MOR claims ([MOR_CLAIMS.jsonl](file:///Users/jemanuel/projects/franken_robotics/planning/mor/MOR_CLAIMS.jsonl)), and 6 full adversarial cross-scoring legs.

---

## 1. The Central Brenner Discovery: Why v1 Hit a 2–3× Ceiling and Where the Real 10× Lives

v1 of `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` was already a strong 2×–3× algebraic robotics simulator, yet it remained trapped inside the **old unit of work**: *"one frozen robot → compile one kernel → train RL from scratch → inspect scalar telemetry."*

Pitting three independent model families against the three Paradoxes of `Q-001` revealed a **unifying structural transformation**:

1. **Paradox 1 (Algebra vs. Speed) → Campaign-Time Workload Quotienting + Compositional Substitution + Mission Quotients (`CC TA-P1` + `COD FTA-P1` + `COD W01` + `CC-07`):**
   - Per-step symbolic CSE alone hits an Amdahl wall (~2–3×) because GPU physics stepping at realistic contact densities is latency- and occupancy-bound.
   - The genuine **10×–100×** algebraic lever occurs at **campaign time** — *deleting entire simulations and derivations* via: (i) **Decorated-cospan / Operadic `.fmodule` substitution contracts** (reusing proofs & sub-kernels on local edits), (ii) **Exact Mission-Observable Quotients (`.fquot`)** collapsing equivalent designs (verified `735,471 → 24,546 = 29.96×` on the ballast potency fixture), (iii) **Exact Burnside/Pólya deduplication** (`86,185` dihedral orbits), (iv) **H/K-theorem symmetry-reduced Floquet/Saltation Gait Atlases** replacing 15-min RL retrains for periodic morphology sweeps, (v) **Exact `real_rur` + GPU Homotopy Continuation** enumerating complete closed-loop linkage families directly instead of blind sampling, and (vi) **Observation-Alias Impossibility Witnesses** rejecting unsolvable sensor suites before Step 0!
2. **Paradox 2 (Heterogeneous-Morphology GPU Wall) → MorphoJIT + Limb Port-ABI + Ragged Contact Queues + Race-Annealed Specialization (`CC-01` + `AGY #1` + `COD W02` + `CC TA-P2`):**
   - Homogeneity is a **32-lane warp/simdgroup property**, NOT a batch property, and a co-design campaign is **not static**!
   - Tier-0 **MorphoJIT** (6-opcode screw bytecode, 128 B/body) makes *any* unseen topology immediately executable with 0 ms compile; **Limb Port-ABI (Tier 0.5)** gives module-level speed; **Ragged Operator/Row-Count Queues (`COD W02`)** eliminate intra-warp contact-count divergence; and **`fs-race` Race-Annealed Async Specialization** smoothly promotes surviving finalists (`512×32 → 8×2048`) to Tier-1 compiled kernels behind an acknowledged dispatch guard (with `(world, replicate_id)` CRN pairing and honest `XT-P2` allowing per-family templates `H2b` to win if faster!).
3. **Paradox 3 (Verification vs. Contact Chasm) → Stratified Multi-Artifact Guarantees: `.fenergy` + `.ftube` + `ClaimProduct<VerifiedFormal, EvaluatedDesign>` (`COD FTA-P3` + `COD W04` + `CC TA-P3` + `CC-06`):**
   - Dissolves the false binary between impossible 36-D global hybrid proofs and unverified statistical RL by matching the certificate to the physical regime — **zero claim laundering enforced by the Rust type system**:
     - **Impact/Stop/ISO-15066 Energy Safety (`COD FTA-P3`, `.fenergy`):** Contact-sequence-invariant observable energy inequality checked in fixed-point integer arithmetic inside `fln-kernel` (ZERO statistical gap!).
     - **Low-D ($\le 4\text{D}$) Guard-Ambiguous Reset Safety (`COD W04`, `.ftube`):** Backward-pruned viable cells certifying every possibly enabled guard successor under sensing error + backup domain.
     - **High-DoF Balance/Locomotion Safety (`CC TA-P3` + `CC-06`, `ClaimProduct`):** Mode-stratified (`27` $V_4$ `{open,stick,slip}` orbits) box-constrained compatibility-certified CBF shields (`∃u ∈ U_act, ∀(w,e) ∈ U`, `fs-ivl` FP32 margin, proved `fln-kernel` reflection) composed with online assumption monitors and conditional miss-rate e-process bounds as a non-promotable `ClaimProduct<VerifiedFormal, EvaluatedDesign>`!

---

## 2. Killed Hypotheses Ledger (`†` — Permanently Excluded From v2.0)

| Killed ID | Hypothesis | Exact Reason for Death (`✂ Exclusion-Test`) | Replacement in v2.0 |
|---|---|---|---|
| **`† K-01`** | Direct single-RHS FP16 Tensor-Core Delassus Cholesky claiming $\kappa(W+\rho I) \le 100$ (`AGY #4`) | Rank-1 counterexample yields $\kappa = 4{,}801$ (48× broken); refinement converges to shifted problem, not true contact; 660 TFLOPS was sparse peak. | `COD W10` Factor-Once Multi-RHS Response Batching (unrounded residuals, $\underline{\lambda}$ error bound) + separate M2 ablation. |
| **`† K-02`** | Log-posynomial feasibility → `fln-anvil::omega` (`MOR-A1 C007`) & inner-ellipsoid-only sampling (`AGY #7`) | Log-posynomial is convex log-sum-exp, NOT integer-linear `omega`; inner ellipsoid excludes boundary optima where real robots live. | Narrow convex-GP monomial screen + proved directional/interval outer exclusion boxes. |
| **`† K-03`** | Runtime $\infty$-operad / nonlinear hybrid double-category wrappers (`H-101b`) | Violates `franken_threed §7.2` data-layout law & breaks on simultaneous hub Delassus coupling. | AOT compile-time decorated-cospan / operad `.fmodule` (`W01` + `CC-07`). |
| **`† K-04`** | Full-room 3D LBM (`1.94 GB/env`) & 50 Hz coarse-cell LBM (`AGY #9`, `Ma = 0.35–5.2`, `τ - 1/2 = 6e-6`) inside RL | 63.7 TB VRAM impossibility / BGK numerical blowup / 2 steps/s per env. | Conservative-flux 2.5D causal tribology PDE (`CC-08`) + 3-mode Housner slosh (Tier-A) + Mach-bounded `fs-lbm`/`ftx-drape` Tier-B oracle. |
| **`† K-05`** | Full high-dim Morse–Conley stratification (`3^k` strata) & global 60-state hybrid Koopman MPC (`3.58M+` params) | Exponential/quadratic blowup & impact-jump discontinuity. | `V_4`/`D_6` Burnside mode strata + bounded M6 smooth-soil bilinear residual tail (`C016`). |
| **`† K-06`** | Unbounded whole-robot spatial Gröbner synthesis (`H-501a`) & `AGY FTA-P1` as a "third alternative" | Exceeds `real_rur` caps (`n<=8, MAX_BITS=4096`); `AGY FTA-P1` restated QoR's own compile-time horn. | Bounded `real_rur` (`W06`) + GPU homotopy continuation (`CC-04`) + Campaign-Time `TA-P1` + `.fquot`. |
| **`† K-07`** | Clairvoyant `∀w ∃u` CBFs (`AGY #3`), pure statistical conformal-e safety invariance (`AGY FTA-P3`), unconstrained zero-sum adversarial worlds, and automated unsafe MCU BSP generation | Fails basic control causality / indistinguishable pre-slip histories / adversary cheating / ISO safety boundaries (`NONGOAL-001`). | `∃u ∀w` box-constrained shields + `.fenergy` + `.ftube` + `ClaimProduct`, candidate-witness CEGIS red-teaming (`CC-03`), HAL-isolated `#![no_std]` codegen (`C014`). |
| **`† K-08`** | Parareal time-parallel rollouts (`CC-25`) & `COD FTA-P2` asynchronous interaction calendar as M2 core | Contact breaks coarse/fine parareal; 50–100 Hz policy & sensor fences cap calendar lookahead in tight interiors. | Warp-coherent `MorphoJIT` + ragged contact queues (`W02`); keep `interaction_calendar` strictly as optional M6 sparse-warehouse research lane. |

---

## 3. Mandatory Execution Order of Crucial Exclusion Tests (Crux-First Discipline)

1. **`GATE-RIVAL-AUDIT` (M0, 3 days CPU):** Pin live MuJoCo/MJWarp, Isaac Lab, Drake, and Genesis versions & strongest warm/cached baselines so all 10× denominators are honest.
2. **`GATE-FORGE-00` & `XT-P2` / `GATE-MORPHVM-01` (M0–M2):** Fingerprint real M5 Ultra & Dual RTX 4090 constants (including `nvidia-smi topo -p2p r`, `recommendedMaxWorkingSetSize`, `TwoSum` canary) and race MorphoJIT Tier-0 vs Limb Port-ABI Tier-0.5 vs Ragged Queues (`W02`) vs 9 Per-Family Templates (`H2b`).
3. **`XT-P1` (M3–M4):** 4-arm campaign-quotienting test (`A0/A1/A2/A3`) with blinded stratified discarded-design audit + `COD` ballast potency control (`735,471 → 24,546`).
4. **`XT-P3` (M4):** Planted breach + in-assumption scenario test across `.fenergy`, `.ftube`, and `ClaimProduct` on shifted interior distributions with independent hidden-state breach labels.
