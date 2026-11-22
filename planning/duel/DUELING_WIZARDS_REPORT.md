# DUELING_WIZARDS_REPORT.md — Three-Model Adversarial Idea Duel Synthesis (`franken_robotics` v2.0)

| Field | Value |
|---|---|
| **Session** | MOR `r1-20261121` · Brenner `RS-20261121-franken-robotics-10x` · `Q-001` · `2026-11-21` |
| **Models (3 Distinct Families)** | **`CC`** (Claude Opus 5.5) · **`COD`** (OpenAI GPT-6.2-Sol) · **`AGY`** (Gemini 4 Neon) |
| **Total Candidate Space** | **90+ raw ideas generated** → **39 formal proposals** (10 proposals + 3 Brenner Forced Third Alternatives per model) cross-scored by both opponents (**78 independent adversarial scores**), plus **2 Phase 6.75 Steelman essays** and **6 Phase 6.9 Blind-Spot Breakthroughs** |
| **Source Artifacts** | [WIZARD_IDEAS_CC.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_IDEAS_CC.md) · [WIZARD_IDEAS_COD.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_IDEAS_COD.md) · [WIZARD_IDEAS_AGY.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_IDEAS_AGY.md) · [WIZARD_SCORES_COD_ON_CC.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_SCORES_COD_ON_CC.md) · [WIZARD_SCORES_COD_ON_AGY.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_SCORES_COD_ON_AGY.md) · [WIZARD_SCORES_CC_ON_COD_AGY.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_SCORES_CC_ON_COD_AGY.md) · [WIZARD_SCORES_AGY_ON_COD.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_SCORES_AGY_ON_COD.md) · [WIZARD_SCORES_AGY_ON_CC.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_SCORES_AGY_ON_CC.md) · [WIZARD_REACTIONS_CC.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/WIZARD_REACTIONS_CC.md) |

---

## 1. Executive Summary

Three distinct frontier model families independently attacked the same hard question (`Q-001`): *How do we turn `franken_robotics` from a strong algebraic simulator (v1) into a verified, genuinely 10× category-defining robotics foundry without succumbing to mathematical theater, false performance claims, or broken live-code assumptions?*

The adversarial cross-scoring pass produced **four decisive outcomes**:
1. **Unanimous Convergence on the True 10× Axis:** All three models independently discovered that per-step CSE alone cannot yield a 10× wall-clock jump on latency-bound GPU robotics; the genuine 10× unlock lives at the **campaign level** — *deleting futile simulations via algebra* (`CC TA-P1`, `COD FTA-P1` `.fquot`, `W01`/`CC-07` operadic `.fmodule`, `CC-02`/`W07` H/K gait atlas + saltation, `W06`/`CC-04` exact linkage/homotopy synthesis, `W08` `.fclear` clearance complexes) while making novel structural edits **immediately executable without compiler stalls** (`CC-01` MorphoJIT + `AGY #1` Limb Port-ABI + `COD W02` ragged contact queues + `TA-P2` race annealing).
2. **A Clear Winner for Formal-Meets-Contact Safety (`COD FTA-P3` `.fenergy`):** Both `CC` (`780/1000`, Rank #1 overall in `CC`'s scoring) and `AGY` (`870/1000`) crowned Codex's **Observable Boundary Energy Budget (`.fenergy`)** as the cleanest dissolution of Paradox 3 — eliminating contact-sequence chaos by conservation of energy across a guided physical interface, with zero statistical gap and direct ISO/TS 15066 alignment. Pairing `.fenergy` (impact/stop bounds) with `.ftube` (low-D ambiguous resets) and `ClaimProduct<VerifiedFormal, EvaluatedDesign>` + box-constrained fused shields (high-DoF locomotion) produces a zero-laundering stratified safety stack.
3. **Surgical Elimination of Hidden Math & Systems Bugs:** Cross-scoring exposed and killed **9 severe mathematical/hardware defects** before a single line of Rust was written: `AGY #4` FP16 Delassus conditioning ($\kappa = 4{,}801$, not $\le 100$); `AGY #9` 50 Hz LBM instability ($\mathrm{Ma} = 0.35\text{–}5.2$, $\tau-\frac{1}{2} = 6\times 10^{-6}$); `AGY #3` clairvoyant `∀w ∃u` CBF quantifier; `CC-06` inverted friction-cone infimum & unboxed projection (`(1.25, 0.25)` outside `[0,1]²`); `CC-08` non-conservative contact mask outside divergence & dimensional Hersey term; `TA-P2` broken `(world, epoch)` CRN pairing & `2` warps/SM (not `4`) Ada occupancy; `TA-P3` ill-typed unconditional $\delta_1$ & `27` (not `14`) $V_4$ `{open,stick,slip}` contact orbits; `CC-07` exact Burnside = **`86,185`** (not `83,333`); and `E022`–`E025` live-code sibling realities (`fs-qty` lifts, stubbed `fln-anvil`, univariate `fsym-formal`, CPU-only `frankentorch-core::DenseTensor`, 3 `asupersync`-free `franken_threed` crates).
4. **Six Extraordinary Phase 6.9 Blind-Spot Breakthroughs** that *none* of the 39 original proposals covered:
   - **`BS-1` (`COD`)** — Observation-Alias Impossibility Witness (`K2::frob-preflight::observation_conflict`)
   - **`BS-2` (`COD`)** — Dynamic Partial-Order Reduction (DPOR) across sensor/watchdog/commit timings (`S1::frob-runtime::schedule_check` + `T::frob-bench` robotics suite)
   - **`BS-3` (`COD`)** — Adaptive-Data-Analysis Blinded Qualification Firewall & $\alpha$-Spending Ledger (`S1::frob-ledger::qualification` + `E2::frob-codesign::confirm_runner`)
   - **`B1` (`CC`)** — Order-Independent GPU Scatter & `StateHash` Replay Determinism Gate (`E0::frob-gpu::determinism` + backend kernels + `T::frob-bench`)
   - **`B2` (`CC`)** — Model-Form Robustness & Simulator Discrete Energy-Creation Audit (`E0::frob-dyn::energy_audit` + `E2::frob-codesign::model_form`)
   - **`B3` (`CC`)** — Joint Robot–Home Retrofit Co-Design (`K2::frob-worldgen::amend` + `V::frob-redteam::amend` — dual of `ddmin`, `10,700` ≤3-item kits)

---

## 2. Reviewer Calibration Meta-Analysis (Honest Accounting of Model Biases)

Before reading raw scores, cross-model calibration revealed a striking methodological lesson (`WIZARD_REACTIONS_CC.md §1.2`):

| Reviewer | Mean Score Given | Range (Spread) | Analytical Signature & Bias |
|---|---:|---:|---|
| **`COD` (OpenAI GPT-6.2-Sol)** | **690** (`734` on CC, `645` on AGY) | `369`–`864` (**495 pts**) | **Highest-signal adversarial auditor.** Recomputed every single arithmetic/occupancy/combinatorial claim and caught ~25 substantive flaws in `CC` (22 conceded outright!) and ~30 in `AGY`. Tendency in its own *ideation* lane: conservative narrow fixtures (tray cart, ballast line) rather than program-wide 10× synthesis. |
| **`CC` (Claude Opus 5.5)** | **549** (`633` on COD, `465` on AGY) | `220`–`780` (**560 pts**) | **Strictest rubric & highest self-calibration.** Recomputed all 30+ COD numbers, caught 4 fatal AGY bounds (`κ=4801`, 25% occupancy, LBM Ma/τ, 65× optimistic Floquet), proactively ran an honest §4 self-audit docking its own `fln-anvil`/`fsym-formal` slips, and conceded 20 fixes cleanly in Phase 6. |
| **`AGY` (Gemini 4 Neon)** | **861** (`898` on CC, `824` on COD) | `685`–`945` (**260 pts**) | **Compressed/over-generous scoring scale.** Caught `E022`–`E025` live-code sibling gaps and strong systems synergies, **but** rubber-stamped 3 subtle mathematical bugs in `CC` (`CC-06` friction-cone direction, `TA-P2` Latin-square CRN pairing, `TA-P3` $\delta_1$ double-counting) and misread `real_rur`'s `MAX_DEGREE=256`. Accordingly, **rank ordering relies primarily on `COD` + `CC` strict scores and recomputed mathematics, NOT raw `AGY` points.** |

---

## 3. Complete 3-Model × 3-Model Cross-Score Matrix (All 39 Proposals)

### 3.1 `CC` Proposals (Claude Opus 5.5) — Scored by `COD` & `AGY`

| ID | Proposal | COD | AGY | Mean | Post-Phase-6 Status & v2.0 Disposition |
|---|---|---:|---:|---:|---|
| **`CC-01`** | MorphoJIT (6-opcode screw bytecode + async Tier-1) | **864** | 945 | **904.5** | **ADOPT (CORE PILLAR).** Withdraw 8–20× warm CUDA Graph launch projection (C11); raise batch $B \ge 65\text{k}$ for Ada occupancy (C10); dispatch-boundary promotion + polynomial sinc/cosc (C12); fuse with AGY #1 Port-ABI + W02 ragged contacts. |
| **`CC-02`** | Equivariant Gait Atlas (H/K + saltation + Envelope Thm) | **842** | 925 | **883.5** | **ADOPT (CORE PILLAR).** Add full-space `fs-spectral::floquet` transverse check (C13) + W07 grazing/cosine≥0.95 gate. |
| **`CC-03`** | Falsify → `ddmin` Shrink (`FMT-MCW-1`) → Attribute → Repair | **820** | 910 | **865.0** | **ADOPT (CORE PILLAR).** Rebase step ratio to 6.41× (C14); require candidate-body witness + AGY #10 agent-iteration gate + `B3` dual retrofit solver. |
| **`TA-P1`** | Algebra Quotients Campaign Work | **804** | 940 | **872.0** | **ADOPT (FOUNDING THESIS).** Rebase A2 to 9.79× pre-confirm (C20); add COD's blinded stratified discard audit + ballast fixture. |
| **`CC-07`** | Operadic Robot Algebra + Wreath + Burnside | **782** | 905 | **843.5** | **ADOPT (CORE PILLAR).** Fix exact Burnside to **`86,185`** (11.6×; sagittal mirror 500,500 = 2.0×, C4); base-coupled Delassus invalidates leaf certs (C5); fuse with COD W01 contracts. |
| **`TA-P2`** | Warp Diversity + Race Annealing | **763** | 935 | **849.0** | **ADOPT (FIXED).** Fix CRN pairing to `(world, replicate_id)` checkpoints (C9); fix 2 warps/SM occupancy (C10); include W02 ragged arm in `XT-P2`. |
| **`CC-08`** | Causal Tribology (2.5D film + Huppert $t^{1/8}$) | **735** | 855 | **795.0** | **ADOPT (FIXED).** Move mask inside conservative face-flux divergence + wheel film reservoir (C6); add $L_c$ to Hersey $\Pi = \eta v/(p L_c)$ (C7); 1 subcycle in RL, $\le 64$ envs LBM eval (C8). |
| **`CC-09`** | Lifecycle Compression (E–Engquist HMM) | **715** | 845 | **780.0** | **ADOPT (FIXED).** Retain `fs-material::fatigue` (`RainflowMiner`) rainflow residue stack across macro-windows (C17); add planted overload & regime-switch controls. |
| **`CC-04`** | Complete-Solution Homotopy Linkage Synthesis | **713** | 885 | **799.0** | **ADOPT (M3 AFTER W06).** Keep Smale $\alpha$/Hauenstein–Sottile same-zero reality/conjugate/distinctness cert (R1); classify completeness honestly (`Complete` iff certified complex count = trace test $N_{\mathrm{gen}}$, else `Partial`); check via `fln-kernel` reflection. |
| **`CC-10`** | Spec-to-Silicon (STL → `.fbuild` + `#![no_std]`) | **686** | 870 | **778.0** | **ADOPT (FIXED).** Unit-normalize STL margins, past-time online reward / delayed future verdicts, real-board WCET receipts, retract "≤0.1% reward hacking" (C18); fuse with W09 reviewer gate. |
| **`CC-05`** | Breadth/Depth Dual Silicon + *Seed-Not-Voxels* | **683** | 895 | **789.0** | **ADOPT (FIXED).** Headline honest **`2.5×–5×`** diversity vs both 4090s (`500–1,000` M5 worlds vs `~200` dual-4090 worlds, `C19`); use `fs-tropical::max_cycle_mean` (not `critical_circuit`); probe `recommendedMaxWorkingSetSize`. |
| **`TA-P3`** | Contract-Stratified `ClaimProduct` Certificates | **621** | 930 | **775.5** | **ADOPT (FIXED).** Fix conditional $\delta_1 = P(\text{silent} \mid \text{breach})$ + Markov count union bound over independent hidden-state labels + retract "valid under shift" (C15); fix to **`27`** $V_4$ strata / 162 lemmas (C16). |
| **`CC-06`** | Proof-Carrying Kernel-Fused Shields | **515** | 835 | **675.0** | **SALVAGE (HEAVILY REWRITTEN).** Split controlled ($\mu_{\min}$) vs env ($\mu_{\max}$) cones (C1); replace broken unconstrained formula with sort-and-clip ($m=1$) & projected Newton + post-hoc KKT cert ($m\ge 2$) (C2); add Lipschitz·$\Delta t$ inter-sample margin & `fln-kernel` reflection (C3). |

### 3.2 `COD` Proposals (OpenAI GPT-6.2-Sol) — Scored by `CC` & `AGY`

| ID | Proposal | CC | AGY | Mean | v2.0 Disposition |
|---|---|---:|---:|---:|---|
| **`FTA-P3`** | **Observable Energy Budget at Physical Boundary (`.fenergy`)** | **780** | **870** | **825.0** | **ADOPT (#1 SAFETY BREAKTHROUGH).** Top-ranked single proposal by `CC` & Steelman'd by `CC`! New `PO-ENERGY-1` proof obligation covering guided stops, grippers, drawers, doors, ISO/TS 15066. |
| **`W07`** | Settled-Gait Return-Map Saltation Adjoint (merged into `frob-gaitatlas`) | **725** | **890** | **807.5** | **ADOPT (CORE PROTOCOL).** Canonical saltation math + augmented state + 100-case gate (20 grazing controls refused, `≥72/80` `cos ≥ 0.95`), fused with `CC-02` H/K + `fs-spectral::floquet`. |
| **`W03`** | Physical Capability-Map Skill Transport (`.fskill`) | **715** | **875** | **795.0** | **ADOPT (CORE PILLAR).** Exact $w = w_0 + B_m(q)u$ + bounded WBC-QP + Farkas separating infeasibility witness BEFORE RL! |
| **`W01`** | Decorated-Cospan Substitution Contracts (`.fmodule`) | **645** | **915** | **780.0** | **ADOPT (CORE PILLAR).** Exact port power signs, dependency invalidation, mirror-swap & global-bus controls, fused with `CC-07` wreath/Burnside. |
| **`W02`** | Ragged Operation-Queue Batching | **650** | **860** | **755.0** | **ADOPT (HYBRID ARM).** Handles dynamic contact-count & solver-iteration tails (`n_row ≤ 30 / 31..48 / >48`) alongside MorphoJIT warp packing; competes directly in `XT-P2`. |
| **`W08`** | Dual Inner/Outer $SE(2)$ Clearance Complexes (`.fclear`) | **650** | **820** | **735.0** | **ADOPT (UPGRADES `G-WORLD-1`).** Exact inner/outer $(x,y,\psi)$ width-family complexes (`74.6 mm` / `75 mm` ceil margin, `240×160×32`) + `MorphologyNavEnvelope` height/riser filter + `B3` retrofit remedy hook. |
| **`FTA-P1`** | Mission-Observable Quotient (`.fquot`) | **635** | **845** | **740.0** | **ADOPT (CORE + POTENCY FIXTURE).** `735,471 → 24,546` (`29.96×`) ballast fixture embedded directly in `XT-P1` & `K2::frob-operad::quotient`. |
| **`W09`** | Counterexample-Guided Build Packet (`.fbuild`) | **630** | **850** | **740.0** | **ADOPT (CORE PRODUCT GATE).** Generalizable-cut rule + 11/12 independent reviewer-accepted packet gate, backed by `fs-geocon`/`fs-fab`/`fs-toleralloc`. |
| **`W04`** | Hybrid Reachable-Tube Composition (`.ftube`) | **620** | **765** | **692.5** | **ADOPT (SCOPED ≤4D).** Guard-ambiguous reset safety (`R_mn((K_m ⊕ E) ∩ G_mn, D) ⊕ (-E) ⊆ K_n`, Minkowski-eroded!) for low-D subsystems (thresholds, trays, braking). |
| **`W06`** | Exact 3-Pose Rational + Bounded `real_rur` Linkages | **590** | **830** | **710.0** | **ADOPT (M1 FOUNDATION).** Respects `E023` `real_rur` caps (`n≤8`, `deg≤256`, `MAX_REAL_ROOT_DEGREE=64`) today; precedes M3 `frob-homotopy`. |
| **`W10`** | Factor-Once Multi-RHS Contact Response Batching | **570** | **790** | **680.0** | **ADOPT (REPLACES `AGY #4`).** Exact $\|b-A\hat{x}\|_2/\underline{\lambda}$ + sticking & inactive non-penetration mode-guard fallback for adjoint/counterfactual multi-RHS queries. |
| **`W05`** | Causal Spacetime World-Tile Recomputation | **540** | **840** | **690.0** | **ADOPT (SCOPED FAR-FIELD).** Applied to far-field multiphysics tiles (spills/steam) where dirty fraction stays `≤4.21%`; robot-centric replays use full-state deterministic branching. |
| **`FTA-P2`** | Earliest-Possible Interaction Calendar | **480** | **685** | **582.5** | **DEFERRED (Post-v2.0 Research, `† K-08`).** Policy/sensor ticks couple every step in dense RL; deferred out of v2.0 critical path to post-v2.0 sparse multi-robot warehouse research. |

### 3.3 `AGY` Proposals (Gemini 4 Neon) — Scored by `COD` & `CC`

| ID | Proposal | COD | CC | Mean | v2.0 Disposition |
|---|---|---:|---:|---:|---|
| **`#1`** | Operadic pHS + Limb Port-ABI Megakernel | **803** | **600** | **701.5** | **ADOPT LIMB PORT-ABI ("TIER 0.5").** Fix Ada occupancy (`25%`, not `≥75%`), note ABA Schur update is PSD (not strictly SPD), price base-coupled contacts explicitly. |
| **`#10`** | Pearl-Causal `2^10` Time Travel + `franken_threed` Dossier | **818** | **565** | **691.5** | **ADOPT (FIXED).** Keep exact `2^10` Shapley + `≥3×` agent-iteration gate; build `WebStudioScenePacket` adapter on **3 `asupersync`-free crates (`f3d-core`/`math`/`graph` with `default-features = false, features = ["std"]`, banning `f3d-runtime` `=0.5.0` & `serde_json =1.0.151`!)** from `franken_threed @ bf7abe1`; pair with `B1` deterministic GPU scatter! |
| **`#6`** | Floquet–Poincaré Continuation Atlas | **835** | **550** | **692.5** | **MERGED INTO `CC-02` + `W07`.** Withdraw 65× optimistic 37 ms estimate; reuse `fs-spectral::floquet`. |
| **`#2`** | Buckingham-Π Universal Policy + Symbolic Pullback | **646** | **570** | **608.0** | **SALVAGE (STACKED ON `W03`).** Replace fixed-base pullback with W03 contact-consistent WBC-QP; fix `m₀√(gℓ₀)` linear momentum & **`330.3`** dense BF16 TFLOPS; execute on `GpuTensorView` (not CPU `DenseTensor`). |
| **`#8`** | Adversarial Scene Grammar + Legality Priors | **721** | **455** | **588.0** | **MERGED INTO `CC-03`.** Retain OSHA/friction/door-closer legality priors; drop zero-almost-everywhere binary failure gradient; replace `β₀` with `.fclear` + candidate-body witness. |
| **`#7`** | Study-Quadric / Gröbner Synthesis + Feasible Polytopes | **746** | **380** | **563.0** | **MERGED INTO `W06` + `CC-04`; REJECT INNER-ELLIPSOID SAMPLING RULE.** Keep bounded 5-pose Burmester; drop inner-ellipsoid sampling (cuts off boundary optima!). |
| **`#5`** | M5 Ultra 256 GiB UMA World Atlas | **685** | **410** | **547.5** | **MERGED INTO `CC-05`.** Keep concrete residency map; drop unsupported `<0.8 µs` live shared-buffer handoff claim; enforce `StorageModePrivate` + `recommendedMaxWorkingSetSize` + generalization gate. |
| **`FTA-P1`** | Two-Stage Staged Algebraic Erasure | **754** | **310** | **532.0** | **DEMOTED FROM FTA → COMPILER LAW.** Good `Prop → 0 B` erasure & F3D §16 ablation discipline (already in NEON), but restated QoR rather than adding a third alternative. |
| **`#3`** | Hybrid Barrier Tubes + Fused KKT Shield | **478** | **560** | **519.0** | **MERGED INTO STRATIFIED SAFETY (FIXED).** Fix clairvoyant `∀w ∃u` → `∃u ∀w`; route through `fln-kernel` reflection; retract "safe for any μ ≥ 0". |
| **`FTA-P2`** | Warp-Cohomogeneous Super-Batching | **551** | **500** | **525.5** | **MERGED INTO `TA-P2`.** Retract false "CRN requires K ≥ 32" premise; warp grouping is strictly a SIMT hardware alignment choice. |
| **`#9`** | Sparse-Tile 50 Hz LBM + IPC Multiphysics | **614** | **360** | **487.0** | **REJECT 50 Hz LBM IN RL (`†`); SALVAGE HOUSNER SLOSH & TIER-B LBM.** Unstable `Ma=0.35–5.2`, `τ-½=6e-6`; move `fs-lbm`/`ftx-drape` strictly to Tier-B eval, adopt `CC-08` 2.5D film + 3-mode Housner for Tier-A RL! |
| **`FTA-P3`** | Quotient Proof + KKT + Conformal-e | **369** | **570** | **469.5** | **MERGED INTO `TA-P3` (FIXED).** Fix axis slip (simulated monitor = `EvaluatedDesign`, NOT `ValidatedPhysics`), keep e-process as diagnostic alarm alongside `.fenergy`/`.ftube`. |
| **`#4`** | Tensor-Core Single-RHS FP16 Delassus | **444** | **220** | **332.0** | **MUTUALLY KILLED (`† K-01`).** Refuted at $\kappa = 4{,}801$; replaced wholesale by `COD W10`! |

---

## 4. Phase 6.75 Steelman Insights & Phase 6.9 Six Blind-Spot Breakthroughs

### 4.1 Phase 6.75 Cross-Model Steelman Essays
- **`COD` Steelman of `CC-01` (MorphoJIT):** Even before clever compilation or universal RL exists, turning unseen topology mutations into a validated data-table update on one resident GPU grid decouples evolutionary design search from compiler backlog and provides an always-on reversible reference behind every future specialization.
- **`CC` Steelman of `COD FTA-P3` (`.fenergy`):** Every other shield tries to certify contact dynamics — the exact thing simulators get wrong. `.fenergy` certifies conservation of energy across a physical interface ($\frac{1}{2}mv^2 + \int \max(P_{\mathrm{elec}}, 0)\,dt + E_{\mathrm{ext}}(t) + E_{\mathrm{stored}} + E_{\mathrm{reserve}} \le W_{\mathrm{abs,min}}$), dropping contact-sequence chaos out by construction, earning `ValidatedPhysics` from one drop-weight + power-meter bench test, surviving 100 policy retrainings for zero re-verification cost, and speaking ISO/TS 15066 natively!

### 4.2 All Six Phase 6.9 Blind-Spot Breakthroughs (Adopted in v2.0!)

| ID | Origin | Gap Exposed Across All 39 Original Proposals | v2.0 Architectural Artifact & Decisive Falsifier |
|---|---|---|---|
| **`BS-1`** | `COD` | **Observation-Alias Impossibility Witness** — Nobody proved when *every* policy fails due to sensor aliasing before burning RL compute. | `frob-preflight::observation_conflict` (`fln-kernel` disjoint outer action-set witness + cheapest separating sensor/rate fix). **Kill:** 1 false witness on negative controls (`GATE-OBSALIAS-01`). |
| **`BS-2`** | `COD` | **Event-Ordering / Timing Races (DPOR)** — Nobody checked sensor capture / packet / estimator / watchdog / actuator-commit permutations. | `frob-runtime::schedule_check` (DPOR over `asupersync::LabRuntime`, e.g., `720 → 8 = 90×`). **Kill:** misses 1 planted stale-read or watchdog/commit race (`GATE-DPOR-01`). |
| **`BS-3`** | `COD` | **Adaptive-Data-Analysis Qualification Firewall** — Autonomous AI agents retrying against held-out worlds p-hack benchmarks. | `frob-ledger::qualification` (Discovery vs Blinded Confirmation runner on fresh world clusters, $\alpha_i = \alpha/[i(i+1)]$). **Kill:** campaign false-promotion rate exceeds $\alpha$ on planted no-signal worlds (`GATE-QUAL-01`). |
| **`B1`** | `CC` | **Replay Determinism Asserted, Never Engineered** — `f32` GPU atomic scatter drift collapses CRN $\rho$ (`0.95 → 0.50` = 10× episodes) and corrupts exact `2^10` Shapley (`1,024 → 13,300` rollouts)! | `E0::frob-gpu::determinism` + backend kernels + `T::frob-bench` (Metal: sort+segmented-reduce / body-centric CSR gather [zero 64-bit int atomics, `† K-13`]; CUDA: sort+reduce or Q32.32 `int64` atomics; per-K-step xxh3 `StateHash`, `DeterminismClass` receipt gate, Metal/CUDA explicitly unpaired). **Kill:** `>10%` step overhead (`GATE-DET-01`). |
| **`B2`** | `CC` | **Model-Form Robustness & Simulator Energy-Creation Audit** — Single-solver rankings promote solver-artifact exploits (diagonal $\sqrt{2}\mu$ pyramid cones, energy-pumping penetration recovery). | `E0::frob-dyn::energy_audit` + `E2::frob-codesign::model_form` (re-rank finalists across compliant vs complementarity vs barrier, circular vs pyramid, stiffness ×{½,2}, $\Delta t/2$ + discrete $\Delta E_{\mathrm{mech}} - W_{\mathrm{act}} - W_{\mathrm{ext}} \le \varepsilon$ energy audit; `ModelConditional` blocked from hardware budget!). **Kill:** misses planted diagonal-pyramid or energy-pumping exploit (`GATE-RANKCHECK-01`). |
| **`B3`** | `CC` | **Joint Robot–Home Retrofit Co-Design** — All 39 ideas only mutated the *robot*, never cheap physical home fixes ($12 ramp vs +$2,400 chassis!). | `frob-worldgen::amend` + `frob-redteam::amend` (~40-item typed amendment catalog; dual of `ddmin` searching `C(40,1)+C(40,2)+C(40,3) = 10,700` ≤3-item kits reusing existing robot policy → installer remedy packet). **Kill:** fixes `<30%` of baseline SKU failures or fails bench test (`GATE-AMEND-01`). |

---

## 5. Best-of-All-Three v2.0 Convergence Blueprint

Every subsystem of `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` v2.0 directly executes the highest-scoring, mathematically repaired synthesis from this duel:
1. **Campaign Quotienting:** `CC TA-P1` + `COD FTA-P1` (`.fquot`, ballast potency fixture) + `COD W01` & `CC-07` (decorated cospans + wreath + exact Burnside `86,185`).
2. **Heterogeneous Dual-Silicon Execution:** `CC-01` MorphoJIT + `AGY #1` Limb Port-ABI (Tier 0.5) + `COD W02` ragged contact-island queues + `TA-P2` `(world, replicate_id)` race annealing + `CC-05` Breadth/Depth (*seed-not-voxels*) + `B1` order-independent GPU scatter & `StateHash`.
3. **Linkage & Gait Synthesis:** `COD W06` (M1 rational + `real_rur`) → `CC-04` (M3 GPU parameter homotopy + Smale $\alpha$-cert); `COD W07` + `CC-02` (H/K gait atlas + saltation + `fs-spectral::floquet` + Envelope Theorem).
4. **Stratified Safety & Reflective Proofs:** Direct proved `fln-kernel` computational reflection powering **`COD FTA-P3` `.fenergy`** (impact/stop) + **`COD W04` `.ftube`** (≤4D resets) + **fixed `CC-06`/`TA-P3` `ClaimProduct`** box-constrained shields.
5. **Skill Transfer & Living Worlds:** `COD W03` `.fskill` (exact $w = w_0 + B_m(q)u$ + Farkas witness) underneath `AGY #2` `FiberMorph` residual; `COD W08` `.fclear` dual $SE(2)$ clearance complexes; fixed `CC-08` conservative 2.5D causal tribology + `AGY #9` Housner slosh + Tier-B `fs-lbm`/`ftx-drape`; `COD W05` far-field causal tiles.
6. **Falsification, Lifecycle, Retrofits & Epistemic Hygiene:** `CC-03` CEGIS `ddmin` + `AGY #10` `2^10` Pearl-causal time travel + `B3` `amend` building-retrofit co-design + `CC-09` E–Engquist HMM lifecycle + `BS-1` Observation-Alias + `BS-2` DPOR + `BS-3` Blinded $\alpha$-Spending Qualification + `B2` Model-Form & Energy-Creation Audit + `COD W09`/`CC-10` Spec-to-Silicon `.fbuild` + `franken_threed` WebGPU dossiers.
