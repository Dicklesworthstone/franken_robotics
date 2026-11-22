# COMPREHENSIVE PLAN FOR THE DESIGN OF FRANKEN_ROBOTICS (`frob`) — v2.0 (10× PARADIGM BREAKTHROUGH EDITION)

> **FrankenRobotics** (`franken_robotics`; 51 crates `frob-*`; binaries `frob`, `frobd`, `frob-mcp`, `frob-tui`, `frob-verify`)
> is a **campaign-quotienting, algebraic-geometric, dual-silicon physics gym, living multiphysics 3D world engine, and end-to-end AI-agent Robot Design Foundry** built in pure memory-safe Rust (`50/51` crates `#![forbid(unsafe_code)]`; single audited `<600`-LOC CUDA leaf) on the FrankenSuite constellation (`frankensim`, `franken_lean`, `frankensympy`, `franken_numpy`, `frankentorch`, `franken_threed`, `franken_textiles`, `franken_drone_geometry_reconstruction`, `asupersync`, `frankensqlite`, `fastmcp_rust`, `frankentui`, `franken_networkx`, `frankenscipy`, `franken_markdown`, `frankenmermaid`).
>
> **The 10× Paradigm Shift Beyond v1 and All Incumbents:** Rather than merely accelerating the legacy loop (*"freeze one XML robot → compile one kernel → burn 15–60 min of black-box RL → inspect scalar reward"*), `franken_robotics` v2.0 changes the **unit of computation, proof, world interaction, and verification**:
> 1. **Algebra Quotients the Campaign (Not Just the Step):** Decorated-cospan / operadic `.fmodule` substitution contracts (`frob-operad`), exact mission-observable quotients (`.fquot`), exact Burnside deduplication, H/K-theorem symmetry-reduced Floquet/Saltation gait atlases (`frob-gaitatlas`), exact `real_rur` + GPU polyhedral/parameter homotopy complete linkage synthesis (`frob-homotopy`), and formal observation-alias impossibility witnesses (`frob-preflight`) **delete 90%–99% of futile simulations before a single GPU rollout starts**.
> 2. **Morphology-as-Data Heterogeneous Dual-Silicon Execution:** **MorphoJIT** (`frob-morphvm`: 6-opcode screw-bytecode Tier-0 interpreter + Limb Port-ABI Tier 0.5 + ragged contact-island queues + `fs-race` race-annealed async Tier-1 Futamura specialization) steps **512 distinct robot topologies simultaneously** on one GPU grid across **Apple M5 Ultra (80 cores, 256 GiB unified memory, Metal 3.2→4)** and **Dual NVIDIA RTX 4090 (`SM_89`, 2× 24 GiB GDDR6X, PCIe Gen4)**, orchestrated as an asymmetric **Breadth/Depth Fleet** (*seed-not-voxels*, max-plus critical-circuit balanced).
> 3. **Stratified, Zero-Laundering Safety & Reflective Formal Proofs:** Direct proved **`fln-kernel` computational reflection** (zero dependence on unimplemented `fln-anvil` stubs!) powers three matched formal-meets-contact guarantees: **Observable Boundary Energy Budgets (`.fenergy`)**, **Hybrid Reachable Tubes (`.ftube`)**, and **Box-Constrained Compatibility-Certified Fused Shields** composed with runtime assumption monitors as a Rust-type-enforced **`ClaimProduct<VerifiedFormal, EvaluatedDesign>`**.
> 4. **Living Multiphysics Worlds, CEGIS Red-Teaming & Spec-to-Silicon Build Packets:** Conservative-flux 2.5D causal tribology films that robots spread and hydroplane on, 3-tier multiphysics zoom-lens (`ftx-drape`/`fs-lbm`), dual inner/outer $SE(2)$ clearance complexes (`.fclear`), CEGIS **Falsify → `ddmin` Shrink (`FMT-MCW-1`) → Pearl-Causal `2^10` Counterfactual Time-Travel Autopsy → Repair**, E–Engquist HMM 1,000-hour lifecycle compression, DPOR timing verification, an **Adaptive-Data-Analysis Alpha-Spending Qualification Firewall**, and one-transaction `.fbuild` hardware/BOM/harness/QEMU-verified `#![no_std]` firmware + offline `franken_threed` WebGPU dossier exports!

| Field | Value |
|---|---|
| **Document** | `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` |
| **Version** | **`2.0` — 10× Breakthrough Synthesis & Multi-Pass Adversarial Audit Fold-In** (supersedes v1.0; full audit trail in Appendix E.2) |
| **Date** | `2026-11-21` |
| **Status** | **PLAN (Pre-Implementation).** Repository `/Users/jemanuel/projects/franken_robotics` contains zero Rust code at snapshot date (`FACT`). Every capability, throughput, and LOC figure below is `DESIGN`, `HYPOTHESIS`, or `TARGET` unless explicitly tagged `FACT`. |
| **Sibling Plans Studied** | `frankensim/COMPREHENSIVE_PLAN_FOR_FRANKENSIM.md` (+ `EXTEND_TO_NEW_DOMAINS`, `BRIDGE_PLAN`), `franken_textiles/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENTEXTILES.md` (FTX), `franken_mining/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENMINING.md` (FM), **`franken_threed/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENTHREED.md` (F3D @ `bf7abe1`)**, `franken_drone_geometry_reconstruction` (FDGR), `franken_firmware`, `franken_lean`, `frankensympy`, `frankentorch`, `franken_numpy` |
| **Methodologies Applied** | `/planning-workflow` (3 independent v1 audits: Math/Proof, GPU/Systems, Robotics/Worlds/DAG) + `/idea-wizard` + `/modes-of-reasoning-project-analysis` (cross-model audited `MOR_CLAIMS.jsonl`, 16 claims: `9 PARTIALLY_SUPPORTED`, `6 HYPOTHESIS`, `1 REJECTED`) + `/dueling-idea-wizards` (3-model 90-idea adversarial duel across Claude Opus 5.5, OpenAI GPT-6.2-Sol, Gemini 4 Neon + 6 cross-scoring legs + Steelman + Phase 6.9 Blind-Spot Probes) + `/brennerbot-with-ntm` (3 Paradoxes, 8 Killed `†` Hypotheses, 4 Crucial Exclusion Tests `XT-P1`–`XT-P3` + `GATE-RIVAL-AUDIT`) |
| **Supporting Research Provenance** | [MOR_CASE.md](file:///Users/jemanuel/projects/franken_robotics/planning/mor/MOR_CASE.md) · [MODES_OF_REASONING_REPORT_AND_ANALYSIS_OF_PROJECT.md](file:///Users/jemanuel/projects/franken_robotics/planning/mor/MODES_OF_REASONING_REPORT_AND_ANALYSIS_OF_PROJECT.md) · [DUELING_WIZARDS_REPORT.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/DUELING_WIZARDS_REPORT.md) · [disagreement_register.md](file:///Users/jemanuel/projects/franken_robotics/planning/brenner/distillations/disagreement_register.md) · [brenner/ARTIFACT.md](file:///Users/jemanuel/projects/franken_robotics/planning/brenner/deliverables/ARTIFACT.md) |

---

## Table of Contents

- **PART 0 — DOCUMENT CONTROL & EPISTEMIC CHARTER** (§0.1–§0.7: Evidence Labels, Implemented Contract, ID Registry, Five Non-Conflated Claim Axes + `ClaimProduct`, 12-Subsystem / 51-Crate Atlas, Live-Code Grounding Audit `2026-11-21`, Hygiene Rules)
- **PART I — 10× THESIS, DECALOGUE, SIXTEEN BETS & NORTH-STAR SCENARIOS** (§1–§5: Incumbent Structural Reality & `GATE-RIVAL-AUDIT`, Robotics Decalogue, `BET-01`–`BET-16`, Goals/Non-Goals/Anti-Goals, `NS-01`–`NS-16`)
- **PART II — THE ALGEBRAIC, OPERADIC, HOMOTOPY & REFLECTIVE-PROOF SPINE** (§6–§13: Normative §6.0 `fs-ga` Conventions, PGA/CGA/$\mathrm{Sym}^+(4)$, Corrected Cartan Curvature, Corrected Trapezoidal-Potential LGVI, 4D Coulomb–Contensou + Joint Stiction/Backlash + Factor-Once Multi-RHS, GVS Cosserat, Extensive-Entropy pHS, $\Gamma_{\mathrm{mech}}/\Gamma_{\mathrm{obs}}$ + $\mathbb{R}/\mathbb{C}/\mathbb{H}$ + Wreath + H/K + Burnside, Decorated-Cospan Operads & `.fquot`, `real_rur` + GPU Homotopy Continuation, `FMT-KIR-1` + 4-Target Codegen, Proved `fln-kernel` Reflection, Stratified Safety `.fenergy`/`.ftube`/`ClaimProduct`)
- **PART III — MORPHOLOGIES, LIVING MULTIPHYSICS 3D WORLDS & SENSING** (§14–§16: Formal `FMT-FBOT-1` Grammar/Macro/Units, 18 Archetypes, 8-Stage Preflight + Observation-Alias Witness, 3-Tier GPU `WorldScene`, 2.5D Causal Tribology PDE + 3-Tier Multiphysics Zoom-Lens, 6 Realistic Interiors + `.fclear` Clearance Complexes, `fdgr` Uncertainty-Coupled Real-to-Sim, Multi-Modal Sensors + Cellular-Sheaf Calibration + Imperfect $SE_{2+m}(3)$ InEKF)
- **PART IV — DUAL-SILICON HETEROGENEOUS FORGE, CO-DESIGN CASCADE & ATELIER FOUNDRY** (§17–§20: M5 Ultra Metal 3.2→4 + `BD-029a` & Dual RTX 4090 CUDA Pools/Graphs/3-Tier SRAM, `DeviceArena` Spec, `MorphoJIT` + Limb Port-ABI + Ragged Queues + Race Annealing, `frob-gpu::fleet` Breadth/Depth, `FMT-FTASK-1` STL IR + `GpuTensorView` RL, Equivariant Gait Atlas + `.fskill` Capability Transfer + 10× Cascade, Blinded Alpha-Spend Qualification Ledger & DPOR Timing Checks, HMM Lifecycle Compression, CEGIS Red-Team + Pearl-Causal `2^10` Time-Travel Autopsy, `frob-mfg` Hardware Foundry & `#![no_std]` Firmware, Async `frob-mcp` + `franken_threed` WebGPU Dossiers)
- **PART V — FRANKEN CONSTELLATION INTEGRATION, 51-CRATE TOPOLOGY & GOVERNANCE** (§21–§24: Verified Consume-vs-Lift Dependency Ledger `FD-01`–`FD-17`, Downward Layering & `frob-cx`, 51-Crate LOC Budget Table, Complete `FMT-*` & `INV-*` Master Registries, Determinism Classes, Interop & Security)
- **PART VI — VERIFICATION, MASTER GATES, RISKS, MILESTONES & ZERO-ORPHAN BEADS DAG** (§25–§29: `KA-01`–`KA-14` Analytical Oracles, Crucial Brenner Exclusion Tests `XT-P1`–`XT-P3`, Master `INT-*`/`GATE-*`/`KILL-*` Tables, Pre-Mortem/Risks/Open Decisions, M0–M6 Roadmap & Acyclic Zero-Orphan Beads Blueprint `BD-001`–`BD-096` + Per-Bead Logging Contract)
- **APPENDICES A–E** (3-Model Dueling Idea Wizards Score Matrix & Blind Spots · Brenner Disagreement Register & Killed `†` Hypotheses · Verified Math EV Matrix & Python-Checked Galaxy-Brain Cards `GB-01`–`GB-16` · Corrected Worked `.fbot` + `.ftask` + `.fmodule` + Async MCP Session · Glossary & Multi-Pass Adversarial Audit Fold-In Trail)

---

# PART 0 — DOCUMENT CONTROL & EPISTEMIC CHARTER

## 0.1 Normative Language, Evidence Labels, and Ambition Tags

The keywords **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** follow RFC 2119. Every load-bearing factual, architectural, or quantitative claim carries an **evidence label**:

| Label | Meaning | Authority to Change |
|---|---|---|
| **`FACT`** | Directly verified in a cited sibling repository file, commit SHA, `sysctl`/`system_profiler` query, Python numerical verification script, or peer-reviewed theorem. Code facts cite `repo/path @ short-sha`. | Only new primary evidence |
| **`DESIGN`** | A normative architectural decision of this plan, carrying explicit rationale and tradeoffs. | Formal ADR (`docs/adr/`) |
| **`HYPOTHESIS`** | A testable scientific or engineering belief naming the exact discriminating experiment/gate (`GATE-*` or `XT-*`) that confirms or falsifies it. | Named discriminating gate |
| **`TARGET`** | A performance, accuracy, scale, or LOC budget goal — **never** a measured result until a gate records hardware-pinned measurements. | Recorded gate receipt |
| **`OPEN`** | An unresolved decision with a named owner, milestone deadline, deciding evidence, and fail-closed default (§27.3). | Owner decision |

Technical capabilities carry `frankensim` **ambition calibration tags**:
- **`[S]` Solid** — established geometric mechanics, control theory, GPU systems, or FrankenSuite patterns; execution risk is engineering craft.
- **`[F]` Frontier** — modern research rarely productized end-to-end (MorphoJIT screw-bytecode specialization, H/K-theorem saltation gait atlases, GPU homotopy synthesis, reflective Lean safety shields, causal thin-film tribology).
- **`[M]` Moonshot** — novel synthesis behind explicit feature flags, never on M0–M4 critical path, promoted only after passing a dedicated falsification gate.

**Plan-wide hygiene rule:** No bare performance number, unverified library claim, or decorative mathematical slogan is permitted. Every mathematical structure in Part II MUST compile to a named Rust crate, `frankensympy` derivation, `franken_lean` reflective proof obligation, or GPU kernel artifact. Every 10× headline comparison MUST follow the **Program Measurement Protocol** (adopted from the 3-model duel): paired campaigns (`n >= 10`) with a **95% paired lower confidence bound**, strongest warm/cached incumbent configuration frozen after reference-only tuning, and ALL compilation, admission, synchronization, transfer, fallback, refusal, and verification costs charged in the denominator!

## 0.2 Strict Definition of "Implemented"

A capability in `franken_robotics` is **implemented** iff **all seven** conditions hold simultaneously:
1. **Crate & Budget Compliance:** Lives in its §22.2 crate within `registries/loc_budget.toml` and passes `xtask check-layers` + `xtask check-unsafe` (`50/51` crates `#![forbid(unsafe_code)]`).
2. **Complete Contract:** Owning crate ships a 10-section `CONTRACT.md` (purpose, inputs, outputs, invariants, typed refusals, determinism class, evidence class, validity domain, pinned oracles, explicit no-claims).
3. **Pinned Oracle Verification:** Known-answer (`KA-01`–`KA-14`) and differential tests pass against pinned independent oracles (`frob-bench`) never regenerated from candidate output.
4. **Hardware-Twin Consistency:** Any GPU kernel (Metal or CUDA) passes margin-separated differential parity (`GATE-GPU-TWIN-01`) against CPU `f64` oracle and dual-number gradchecks.
5. **Cancel-Correct & Crash-Safe Execution:** Passes `asupersync::LabRuntime` cancellation, budget-exhaustion, DPOR timing (`frob-runtime::schedule_check`), and fault-injection tests with zero leaked GPU memory pools or torn Ledger commits.
6. **Studio Reachability:** Registered in `registries/verbs.toml` and reachable through `frob --json` (deterministic `FMT-ENV-1`) and `frob-mcp` (passing schema parity + async job contract).
7. **Recorded Gate Receipt:** Governing `GATE-*` / `XT-*` carries a digest-pinned PASS receipt recording exact commit SHA, hardware fingerprint, seed, and measured numbers.

Anything meeting fewer than all seven conditions is **`partial`** and MUST be labeled `partial` on every CLI, MCP, and report surface.

## 0.3 Stable Identifier Registry

Published IDs MUST NOT be renumbered; retired IDs remain tombstones pointing to their successor.

| Prefix | Object | Prefix | Object |
|---|---|---|---|
| `GOAL-` | Primary project goal (§4.1) | `NONGOAL-` | Explicit exclusion (§4.2) |
| `NS-` | North-Star end-to-end scenario (§5) | `BET-` | Architectural bet with falsifier (§3) |
| `INV-` | Constitutional invariant (§22.3) | `PO-` | Executable / reflective proof obligation (§13.2) |
| `G-` | *Retired v1 prefix* (superseded by `FROB-REF-nnn`, `PO-*`, `GATE-*`; see `† K-07`) | `GATE-` | Convergence or verification gate (§26.2) |
| `INT-` | Sibling foundation admission gate (§26.1) | `KILL-` | Kill gate with pre-declared fallback (§26.3) |
| `XT-` | Brenner crucial exclusion-test experiment (§25.2) | `FMT-` | Versioned durable binary/JSON format (§22.4) |
| `FROB-REF-nnn` | Typed refusal code with ranked repair hints (§22.5) | `RISK-` | Risk register item (§27.2) |
| `OPEN-` | Unresolved owner decision (§27.3) | `FD-` | Foundation dependency row (§21.1) |
| `WP-` | Work package (§28.2) | `M0`–`M6` | Delivery milestone (§28.1) |
| `BD-` | Dependency-aware execution bead (§29) | `GB-` | Galaxy-Brain transparency card (Appendix C.2) |
| `KA-` | Analytical known-answer physics benchmark (§25.1) | `FAIL-` | Automated causal autopsy class (§19.2) |

## 0.4 The Five Claims, Never Conflated — Plus Type-Enforced `ClaimProduct` (`INV-001`)

Every output emitted by `franken_robotics` carries an explicit claim axis (`frob-evidence`). No gate, report, or agent response may launder evidence across axes:

| Claim Axis | What It Establishes | What It Does **NOT** Establish | Required Evidence | Permitted Wording |
|---|---|---|---|---|
| **1. Formal / Algebraic Proof (`VerifiedFormal`)** | An algebraic, geometric, symmetry, inertia, passivity, operadic composition, boundary-energy, or rational-SOS proposition holds identically under declared model $M$ and assumptions $A$. | That model $M$ or assumptions $A$ match uncalibrated real hardware or exterior friction/perception. | `franken_lean` Crucible (`fln-kernel`) reflective receipt + independent `fln-checker` offline replay. | "Formally proved under model M & assumptions A (kernel digest H)." |
| **2. Numerical & Hardware Parity (`VerifiedNumeric`)** | Discrete integration, contact solve, saltation/adjoint gradient, homotopy root enclosure, Metal/CUDA kernel, or QEMU firmware trace matches analytical / CPU `f64` oracle to $\varepsilon$. | That constitutive laws (Coulomb friction, Bekker–Wong soil, Archard wear) match a specific physical floor or hardware lot. | Analytical benchmarks (`KA-01`–`KA-14`), `fs-ad` gradchecks, `GATE-GPU-TWIN-01`, `fs-ivl` enclosures. | "Matches analytical / CPU-f64 reference within $\varepsilon$ on host H." |
| **3. Physical Fidelity (`ValidatedPhysics`)** | Simulated trajectories, friction films, slosh loads, wear rates, or sensor traces agree with empirical real-world hardware measurements within stated uncertainty bands. | Generalization outside the calibrated terrain/material/load domain (synthetic blends stay `Estimated`!). | Held-out real-hardware measurement dataset + out-of-sample residual calibration receipt. | "Calibrated against dataset D on domain $\Omega$ with empirical coverage c%." |
| **4. Design & RL Improvement (`EvaluatedDesign`)** | Candidate B outperforms baseline A on declared mission/STL metrics under paired seeds & blinded confirmation with controlled error spend. | Global optimality, physical hardware safety signoff, or transfer to unmodeled regimes. | Paired CRN evaluation + `fs-eproc` e-value + blinded `frob-ledger::qualification` alpha-spend receipt. | "Outperforms A by $\Delta$ under blinded suite S (e-value $E \ge 1/\alpha_i$)." |
| **5. Hardware Performance (`BenchmarkedPerf`)** | Stated throughput (env-steps/s, kernel µs, compile ms, working-set GiB, paired campaign wall-clock) measured on named silicon under sustained thermals. | Correctness, physical accuracy, or design optimality. | Hardware-fingerprinted paired-block ablation (`franken_threed §16`, 20-min sustained thermal floor). | "Achieves X at batch B on device D (p50/p95/p99, R% roofline)." |

**Type-System No-Laundering Enforcement (`INV-001`, adopted from `TA-P3`):**
1. Combining a `VerifiedFormal` theorem with an `Estimated` constitutive card caps the composite physical prediction at `Estimated` while preserving the dependency subgraph.
2. When formal conditional theorems ($A_\sigma \Rightarrow G_\sigma$) combine with runtime assumption monitors and empirical statistical residual bounds ($\delta_1, \delta_2, \delta_3$), `frob-evidence` exposes **`ClaimProduct<VerifiedFormal, EvaluatedDesign>`** (upgrading the second factor to `ValidatedPhysics` *only* on real-hardware data!) with **ZERO `From`/`Into` conversion to `VerifiedFormal`**. Claim laundering is a Rust compile error!
3. All simulation, codegen, CAD, and `#![no_std]` firmware exports hard-wire `execution_authorized: false`, `hardware_safety_signoff: false`, and `sim_to_real_certified: false`.

## 0.5 Named Subsystems Atlas (12 Bounded Contexts, 51 Crates)

Twelve named bounded contexts partition `franken_robotics` across **51 crates** (`49` production/surface + `2` `T`-layer verification crates + `xtask`, §22.2):

| # | Subsystem | Layer(s) | Bounded Context | Primary Crates (Count = 51) |
|---|---|---|---|---|
| 1 | **AXIOM** | S0, S1 | Units/frames/spectral exponents, deterministic math/Philox, 5-axis `ClaimProduct`, `frob-cx` pure token, `asupersync` + generic DPOR engine + `FMT-RPC-1`, FrankenSQLite Merkle Ledger + Blinded $\alpha$-Spend Ledger | `frob-core`, `frob-units`, `frob-evidence`, `frob-cx`, `frob-runtime`, `frob-ledger` (6) |
| 2 | **LIE** | K0 | Coordinate-free $SE(3)/SO(3)/SE_k(3)$ & PGA/CGA (`fs-ga`), `AffineRows` (48 B), $\mathrm{Sym}^+(4)$, principal bundles & Cartan curvature $D\mathbf{A}_S$, $\Gamma_{\mathrm{mech}}/\Gamma_{\mathrm{obs}}$ + H/K + wreath + Burnside | `frob-lie`, `frob-bundle`, `frob-symmetry` (3) |
| 3 | **CAS** | K1 | `frankensympy` robotics compiler (PoE/RNEA/ABA, Echeandia–Wensing Coriolis, `real_rur` linkages), pure BKK/homotopy KIR emitter + Smale $\alpha$-cert, SSA `FMT-KIR-1`, pure 4-target codegen (`CPU/Metal/CUDA/no_std`) | `frob-symbolic`, `frob-homotopy`, `frob-kernel-ir`, `frob-codegen` (4) |
| 4 | **PROOF** | K1, E2 | Proved `FrobMath` direct `fln-kernel` computational reflection + `omega` + `PO-*` (`K1::frob-proof`), runtime stratified shields `.fenergy`/`.ftube`/box-constrained `ClaimProduct` (`E2::frob-shield`) | `frob-proof`, `frob-shield` (2) |
| 5 | **MORPH** | K2, V | Decorated-cospan operadic `.fmodule` algebra, `.fbot` IR (`FMT-FBOT-1`) & 18 archetypes, `.ftask` STL IR (`FMT-FTASK-1`), extensive-entropy pHS actuators, 8-stage preflight + Observation-Alias witness (`K2`), `frob-mfg` COTS/GD&T/harness/STEP foundry (`V`) | `frob-operad`, `frob-bot`, `frob-spec`, `frob-actuators`, `frob-preflight`, `frob-mfg` (6) |
| 6 | **TERRA** | K2 | Pure device-agnostic 3-Tier `WorldScene` IR, 6 realistic interiors + outdoor terrains + `.fclear` dual $SE(2)$ clearance complexes + static `amend` 40-item retrofit catalog, Bekker–Wong/Janosi–Hanamoto terramechanics | `frob-world`, `frob-worldgen`, `frob-terramech` (3) |
| 7 | **DYNAM** | E0 | Corrected Trapezoidal-Potential LGVI + discrete `energy_audit`, DLA wheels/mecanum/tracks, native GVS Cosserat ABA rods, 4D ellipsoidal Coulomb–Contensou + row-gated 3-tier SRAM Delassus + Multi-RHS `W10` + IPC, **MorphoJIT** + ragged queues, 3-tier multiphysics zoom-lens (2.5D causal tribology + slosh + `ftx-drape`/`fs-lbm`), symplectic/saltation adjoints | `frob-dyn`, `frob-contact`, `frob-morphvm`, `frob-multiphys`, `frob-adjoint` (5) |
| 8 | **FORGE** | E0 | Hardware-agnostic `DeviceArena`, `world_scene` GPU upload, `determinism` scatter trait & `StateHash` verifier (`B1`), CAS binary cache, `frob-gpu::fleet` Breadth/Depth scheduler, safe `frob-gpu-metal` (`ft-kernel-metal` v0.2 + `GpuTensorView` `simdgroup_matrix`), sole audited `unsafe` `frob-gpu-cuda` (`SM_89` pools/graphs/WMMA) | `frob-gpu`, `frob-gpu-metal`, `frob-gpu-cuda` (3) |
| 9 | **OPTIC** | E1 | GPU ray-traced LiDAR/RGB-D/stereo/event sensors (glass/mirror/steam/skew), tactile elastomer patch arrays, anchored $W$-orthogonal $\mathfrak{se}(3)$ cellular-sheaf calibration, Imperfect $SE_{2+m}(3)$ RI-EKF + observability Gramians | `frob-sensors`, `frob-estimate` (2) |
| 10 | **CORTEX** | E2 | $\mathbb{R}/\mathbb{C}/\mathbb{H}$ per-space + regular-rep + wreath + `FiberMorph` equivariant layers, IDA-PBC/DDP/WBC-QP + `.fskill` capability-map transfer + Farkas witnesses, H/K Equivariant Gait Atlas, zero-copy `GpuTensorView` `VectorEnv`, 10× Algebraic Co-Design Cascade + `model_form`/`rankcheck` (`B2`) + `confirm_runner` (`BS-3`), E–Engquist HMM Lifecycle Compression | `frob-equiv`, `frob-control`, `frob-gaitatlas`, `frob-gym`, `frob-codesign`, `frob-lifecycle` (6) |
| 11 | **ORACLE** | V, T | 6-axis telemetry (`.npz`), Pearl-causal `2^10` Time-Travel Counterfactual Autopsy + Wasserstein DRO, CEGIS Falsify→`ddmin` Shrink→Repair + `amend` `10,700`-kit retrofit solver (`B3`), fuzzed URDF/MJCF/glTF/USD/`fdgr` + `SubprocessSymbolicBackend` + shared client/verb dispatch, `KA-01`–`KA-14` + `GATE-RIVAL-AUDIT` + `GATE-DET-01`/`GATE-DPOR-01` harnesses (`T`), `std`-only offline `frob-verify` (`T`) | `frob-telemetry`, `frob-diagnose`, `frob-redteam`, `frob-io`, `frob-bench`, `frob-verify` (6) |
| 12 | **ATELIER** | U | Independent `U`-layer leaves driven by `registries/verbs.toml`: `frob` + `frobd` CLI/daemon, async job-based `frob-mcp` (42 enumerated tools / 7 clusters + `resources://`), `frob-tui`, `frob-view` (`NativePresenter` + `franken_threed` `WebStudioScenePacket` offline WebGPU dossier), `frob-report` | `frob-cli`, `frob-mcp`, `frob-tui`, `frob-view`, `frob-report` (5) |

## 0.6 Grounding & Live-Code Provenance Audit (`FACT` Snapshot `2026-11-21`, Independently Cross-Verified)

Every sibling crate claim below was verified directly against live source code and `Cargo.toml` manifests on `2026-11-21` (correcting every optimistic assumption caught during our 3 v1 audits and independent MOR audit):

| Fact ID | Subject | Verified Live Reality (`FACT`) | Mandatory Architectural Consequence (`DESIGN`) |
|---|---|---|---|
| **`F-01`** | Local `mac-studio` | `Darwin 27.0.0 arm64`, **Apple M5 Ultra**, 36 CPU cores (12 Super + 24 Perf), **80 GPU cores**, Metal 4 hardware capability, **256 GiB (`274,877,906,944` B) UMA**; live Metal probe confirms **`recommendedMaxWorkingSetSize = 222.72 GiB`** (`239.14 GB`), `maxBufferLength = 167.04 GiB`, `maxThreadgroupMemoryLength = 32 KiB`, ~800 GB/s planning assumption (`GATE-FORGE-00`). **Live Swift MSL probe (`2026-11-21`):** Metal MSL 3.2 AND 4.0 have **NO native `f64`** AND **NO 64-bit integer atomic add or 64-bit CAS** (`device atomic_ulong*`); `MathMode::Fast` folds away Dekker/Knuth `f32-dd` `TwoSum`. | Primary **Breadth / Studio / Counterfactual** pole: hosts permanent multi-building VDB banks (`StorageModePrivate`, admission-capped at `≤85%` of `recommendedMaxWorkingSetSize` ≈ `189 GiB` with LRU eviction to `≤500` worlds whenever Tier-B `fs-lbm` runs!), `StorageModeShared` checkpoint rings, CPU symbolic/Lean workers, and live `NativePresenter`. `B1` Metal deterministic scatter MUST use **stable-key sort + segmented reduction** or **body-centric CSR fixed-order gather** (never `int64` atomics!). `F32DD` audit kernels MUST compile under `MathMode::Safe` with a startup `TwoSum` canary (`FROB-REF-402`). |
| **`F-02`** | Remote `trj` | **Dual NVIDIA RTX 4090** (`Ada Lovelace SM_89`, 2× 24 GiB = 48 GiB GDDR6X, 1,008 GB/s/card, 128 SMs/card, 48 max warps/SM, 65,536 32-bit regs/SM, PCIe Gen4 — **no NVLink**; consumer Linux drivers typically disable PCIe P2P `cuDeviceCanAccessPeer == 0`; dense BF16-in/FP32-acc is **165.15 TFLOPS/card = 330.3 dual** [660 is sparse!]; FP64 is 1:64 ≈ 1.29 TFLOPS/card). | Primary **Depth / Foundry** pole: zero per-substep cross-GPU traffic; independent complete shards per card; pinned host-staged fixed-order `GPU0 + GPU1` gradient sync; `f64` CUDA restricted strictly to `B<=256` diagnostic twin tests; 1 dedicated OS thread per card for `CUcontext` safety. |
| **`F-03`** | `frankensim` (`@ 483d09c682`) | `173` workspace crates (`193` on disk) on `nightly-2026-07-06`. **CRITICAL `cargo metadata` FACT:** ONLY **9** relevant crates are truly `fs-qty`-free (`fs-ga`, `fs-phs` [`interconnect_mapped`, DAE Dirac deferred], `fs-symmetry`, `fs-eproc`, `fs-tropical` [`max_cycle_mean` + `critical_circuit`], `fs-conform`, `fs-race`, `fs-tilelang` [`kernel!`], `fs-ad`). **ALL 21 others** pull `fs-qty` (`generic_const_exprs`) either directly (`fs-mbd`, `fs-contact`, `fs-evidence`, `fs-lbm`, `fs-tribo`, `fs-solid`, `fs-ports`, `fs-system`, `fs-couple`, `fs-spectral`, `fs-asbuilt`, `fs-matdb`) or transitively via `fs-evidence`/`fs-constraint`/`fs-feec` (`fs-ivl`, `fs-sos`, `fs-topo`, `fs-voi`, `fs-fab`, `fs-toleralloc`, `fs-io`, `fs-geocon`, `fs-topopt`)! Also: `fs-solid::rod` is **statics only** (`"SE(3) dynamics remain successor scope"`), rainflow lives in `fs-material::fatigue`, and top-level `fs-mbd::Wrench` uses mixed `{force_world, torque_body}` (unlike `fs-mbd/src/articulated.rs`). | **Consume ONLY the 9 verified `fs-qty`-free leaves (`FD-01`)**; pre-classify **all 21 `fs-qty`-coupled crates as Lift with provenance (`FD-02`, `registries/lifts.toml`)** swapping `fs-qty` → stable `frob-units` (with an optional M0 upstream check to factor `fs-qty` out of `fs-evidence`, unblocking 7/12 crates immediately!). Author dynamic GVS Chebyshev/Legendre $O(N)$ ABA rods natively in `frob-dyn` (using `fs-solid::rod` strictly as static elastica `KA-05` oracle), and lift rainflow directly from `fs-material::fatigue`! |
| **`F-04`** | `franken_lean` (`@ 4426f8ea7`) | `nightly-2026-08-31`: `fln-kernel` (`tc.rs`, 516 KB, `11,101` prod LOC ≤12 KLOC TCB, `#![forbid(unsafe_code)]`) is a real CIC typechecker; `fln-checker` (`38.0 kLOC` prod crate / `53.9 kLOC` prod closure w/ `fln-hash` + `fln-core`, `80.4 kLOC` total `src/`) is real. **CRITICAL FACT:** `fln-anvil` (`3,675` total LOC / `2,089` prod code across 8 files: `DTree` + `omega` + `omega/{branch,certificate}`) has checkable `OmegaCertificate`, while `simp`, `egraph`, `norm_num`, `grind`, and `portfolio` are **data-type stubs only**! | `frob-proof` MUST NOT depend on `fln-anvil::{egraph, norm_num, simp}`! All M1–M4 obligations compile directly to `fln-core::Expr` proof terms verified via **proved `FrobMath` direct `fln-kernel` computational reflection checkers** + `omega` (`OmegaCertificate`) + `fs-sos/src/verify.rs` interval pre-screens! |
| **`F-05`** | `frankensympy` (`@ 29f847c1`) | `nightly-2026-08-20`: `fsym-core`, `fsym-calculus`, `fsym-matrices`, `fsym-polys`, `fsym-solvers::real_rur` (**hard caps:** `MAX_VARIABLES=8`, `MAX_EQUATIONS=64`, `MAX_DEGREE=256`, **`MAX_REAL_ROOT_DEGREE=64`** eliminant cap, `MAX_TERMS=4096`, `MAX_BITS=4096`, `MAX_COMPUTE=200M`, `MAX_MEMORY=512 MiB`, **0-D ideals only**; NO homotopy solver exists!). `fsym-proof-kernel` is `7.7 kLOC` prod crate / `33.2 kLOC` prod 9-crate closure (`54.5 kLOC` total `src/`), giving `frob-verify` a live transitive TCB of **`~95.1 kLOC` prod (`~143 kLOC` total `src/`)**. **CRITICAL FACT:** `fsym-formal` ONLY projects 2 univariate `ZZ` factor-product profiles (`i64`, deg ≤ 64, targeting external Lean 4.32.2 — NOT `franken_lean`!). | Scope `real_rur` strictly to bounded 0-D rational linkage roots (`W06`, M1, eliminant deg ≤ 64), build `frob-homotopy` (`CC-04`, M3) for degree > 64 / >8-var spatial & 9-point mechanisms, bypass `fsym-formal` via direct `fln-kernel` reflection, and isolate `SubprocessSymbolicBackend` in Layer `V::frob-io` (`KILL-FSYM-1`)! |
| **`F-06`** | `frankentorch` (`@ 8165d6a1`) | `nightly-2026-08-31`, `asupersync 0.6.2`. **CRITICAL FACT:** `frankentorch-core` (`lib ft_core`) `DenseTensor` ONLY stores CPU host-heap `Arc<Vec<T>>`; **BOTH `ft-nn` AND `frankentorch-autograd` (`lib ft_autograd`) run 100% on CPU and pull `rayon = "1.12"`**. `ft-kernel-metal` (`metal = "0.33.0"` + `[patch.crates-io] block = { git = "https://github.com/Dicklesworthstone/rust-block", rev = "b39ae859d1ee8e8cb5eef6a516471f1578d26b96" }`) encapsulates `91` audited `unsafe` Metal/ObjC FFI sites behind safe APIs (`fused.rs` `metal_simdgroup_matrix` 8×8 FP16→FP32 GEMM), but has **NO `MTL4*`**, sync `wait_until_completed`, `offset=0`, `MAX_ARGUMENTS=31`, `MTLDispatchType::Serial`, AND `metal 0.33.0` caps `MTLLanguageVersion` at `V3_1` with `set_fast_math_enabled` (no `MTLMathMode`). | **Option B (Decoupled GPU RL):** Execute hot GPU RL forward/backward/PPO directly on `DeviceArena` `GpuTensorView` kernels in `frob-gpu-metal` (`simdgroup_matrix`) and `frob-gpu-cuda` (WMMA) — zero CPU copies and zero `rayon`! Schedule **`BD-029a`** for upstream `ft-kernel-metal v0.2` (zero-copy slices, `PrivateBuffer`, **`BlitCommandEncoder`**, **`MTLBinaryArchive`**, **`set_threadgroup_memory_length`**, **`dispatch_threadgroups_indirect`**, offsets, concurrent encoders, async `SubmitHandle` + `MTLSharedEvent`) and target **Metal 3.1/3.2** honestly in M2 (`OPEN-METAL4-RT` for Metal 4/`objc2-metal`)! |
| **`F-07`** | `franken_threed` (`@ bf7abe1`) | **PRESENT ON DISK** (`nightly-2026-08-31`; 1,835-line plan, 11 crates ≤245 kLOC). Live 4 crates: `f3d-core`, `f3d-math`, `f3d-graph`, `f3d-runtime` (`f3d-gpu`/`scene`/`assets`/`compiler` planned). **CRITICAL `Cargo.toml` FACT:** `f3d-runtime` unconditionally pins **`asupersync = "=0.5.0"`** (incompatible with workspace `=0.6.2`!). Key engineering laws: §4.5 guarded specialization, §4.10 remove questions from frame loop, §6.7 queue order ≠ recording order, §7.2–7.5 `AffineRows` (48 B), §10.3–10.4 offline HTML/WebGPU, §16 paired-block thermal ablations. | Consume **ONLY `f3d-core`, `f3d-math`, `f3d-graph`** (zero `asupersync` dep!) until `f3d-runtime` bumps from `=0.5.0` → `=0.6.2`; integrate offline interactive WebGPU counterfactual & `.fbuild` dossiers behind a versioned `WebStudioScenePacket` adapter (`frob-view`)! |
| **`F-08`** | Remaining Suite Siblings | Verified live SHAs: `franken_numpy @ e8cb85f4`, `asupersync @ b32c5ba66` (`v0.6.2`), `frankensqlite @ 79b735cb2`, `frankentui @ 5979b92c` (NOTE: optional `ftui-runtime/asupersync-executor` pins `=0.5.0` — keep disabled!), `franken_networkx @ 02ea3206a`, `frankenscipy @ edb7ae619` (`08-31`), `franken_textiles @ ecb13277` (`ftx-drape`, `08-31`, `#![forbid(unsafe_code)]`), `franken_drone_geometry_reconstruction @ db45703` (`34` `fdgr-*` crates, `08-31`), `franken_firmware @ 151f9ca` (`1.99.0`, `unsafe_code = "forbid"`), `franken_markdown @ ac16e633` (`franken_markdown` / `fmd`, `08-31`), `frankenmermaid @ 431ee086` (`10` `fm-*` crates, `08-31`), `fastmcp_rust @ 117388bc` (`08-25`, verified on `08-31` in `INT-FMCP-001`). Crate prefix `frob-` has zero collisions. | Pin workspace to **`nightly-2026-08-31`** and **`asupersync = "=0.6.2"`** with single package-ID deduplication (`xtask check-deps`), forbidding `f3d-runtime` & `ftui-runtime/asupersync-executor` until bumped to `=0.6.2`, and carrying the full 40-char `[patch.crates-io] block` rev in root `Cargo.toml`! |

## 0.7 Plan Hygiene Rules (Inherited from FTX, FM, F3D, and FrankenSim)

1. **Delivery logs never live in this plan** (stored strictly in `docs/delivery/`).
2. **LOC ceilings from Commit 1:** Every crate in §22.2 carries planned LOC and a 1.15× CI merge block enforced by `xtask loc`.
3. **Convergence before breadth:** No new archetype, interior class, or `[M]` feature lands while any prior milestone's hardware-twin, known-answer, or layer gate is red.
4. **Refusals over silent clamps:** Invalid inertias, contact-buffer overflows, stale proof digests, `TwoSum` fast-math folding, grazing saltation singularities, and unattainable skill wrenches ALWAYS emit typed `FROB-REF-nnn` refusals with ranked repairs or separating witnesses.
5. **One Source of Truth per Boundary:** One `WorldScene` drives physics + sensors + visuals; one `frob-kernel-ir` lowers CPU + Metal + CUDA + `#![no_std]` firmware; one `registries/verbs.toml` drives CLI + MCP + TUI; one `.fbot`/`.fmodule` graph drives dynamics + proofs + equivariance + BOM/CAD.

---

# PART I — 10× THESIS, DECALOGUE, SIXTEEN BETS & NORTH-STAR SCENARIOS

## 1. Honest Incumbent Audit (`GATE-RIVAL-AUDIT`) & Why v2.0 Achieves a Genuine 10× Unlock

A credible 10× plan never argues against strawman caricatures of competitors. Direct verification of primary incumbent documentation (`2026-11-21`, pinned formally in **`GATE-RIVAL-AUDIT`** at M0) establishes both what modern engines **already do well** and the **exact structural walls** none of them can cross:

| Incumbent Engine | Real Capabilities Preserved in Fair Benchmarks (`GATE-RIVAL-AUDIT`) | Exact Structural Wall `franken_robotics` v2.0 Breaks |
|---|---|---|
| **MuJoCo / MJX / MuJoCo Warp** | Fast procedural CPU `mjSpec` edits + state-preserving recompilation; warm MJX JIT caches; **MJWarp per-world assets & continuous parameter fields** over common model dimensions; rock-solid soft convex contact. | Compiled `mjModel`/MJX/MJWarp layouts fix kinematic tree structure (`body_parentid`, DoF shapes); **zero** operadic substitution contracts with incremental proof survival, **zero** event-located saltation/Floquet H/K gait atlases, **zero** complete homotopy linkage synthesis, **zero** formal proof kernel, **zero** causal robot-advected tribology fields or STL-to-firmware equivalence. |
| **NVIDIA Isaac Lab (PhysX 5 / Newton)** | Massive CUDA RL throughput; multi-asset spawning, heterogeneous USD stage construction, compatible articulation views, and **Newton contact/deformable/VBD paths**. | **CUDA-only** (structurally locked out of Apple M5 Ultra's 256 GiB unified memory); articulation views require compatible kinematic layouts; **zero** analytic rigid-contact derivatives/saltation, **zero** symbolic algebra or formal proofs, **zero** mission-observable quotients, **zero** solvability-certified minimal counterexample grammars. |
| **MIT Drake** | Best-in-class CPU multibody accuracy, `Diagram` system composition, `AutoDiffXd`, `MathematicalProgram` direct trajectory optimization, floating-point SOS certificates, and rich hydroelastic contact. | **CPU-only** (zero GPU-batched multi-morphology RL or 1,024-way counterfactual replay); finalized `MultibodyPlant` fixes topology monolithically; time-stepping contact lacks batched GPU saltation/continuation; floating-point SDP solutions lack an independent ≤12 KLOC trusted formal kernel or kernel-fused GPU/firmware equivalence proof. |
| **Genesis World** | Multi-solver GPU backends (**including Apple Metal/Quadrants**), coupled IPC/MPM/SPH deformables, and state checkpoint/partial-reset APIs. | `scene.build(n_envs)` replicates one static solver layout across envs; **zero** morphology-as-data mixed-topology execution, **zero** algebraic preflight/homotopy/operad/equivariance/proof stack, **zero** causal 2.5D tribology fields at RL scale, **zero** blinded alpha-spending qualification or verified firmware codegen. |

**The Founding 10× Thesis (`DESIGN`):**
> **The 10× breakthrough in robotic engineering is NOT making one frozen XML step 2× faster — it is turning a robot design into an *incrementally executable, quotiented, formally stratified algebraic object* whose entire campaign (linkage synthesis → feasibility & observation-alias screening → operadic substitution → mixed-morphology GPU simulation → gait-atlas continuation → capability-map transfer → CEGIS red-teaming → Pearl-causal counterfactual repair → manufacturable CAD/BOM & `#![no_std]` firmware) operates natively on Lie groups, decorated cospans, representation theory, and reflective proof terms across asymmetric M5 Ultra + Dual RTX 4090 silicon.**

---

## 2. The Robotics Decalogue (Ten Non-Negotiable Principles)

1. **P1 — Pure Safe Rust Across 50/51 Workspace Crates; Quarantined GPU FFI Only.** **50 of 51 workspace crates** enforce `#![forbid(unsafe_code)]` (including `frob-gpu-metal`, which delegates audited Metal/ObjC FFI to sibling `frankentorch::ft-kernel-metal`, upgraded in `BD-029a` with explicit `SAFETY.md` review!). Within `franken_robotics`, `unsafe` is permitted ONLY in `frob-gpu-cuda` (`<600` LOC audited Driver/NVRTC leaf with `SAFETY.md`). Zero C++ physics engines anywhere in production closures.
2. **P2 — Geometry-First, Data-Layout-Respecting Truth.** Authoring and proofs live on $SE(3)$, PGA `Cl(3,0,1)`, $\mathrm{Sym}^+(4)$, principal bundles, and decorated cospans; runtime GPU loops consume **packed, flat, branch-free algebraic data layouts** (128-B `MorphoTable` rows, 48-B `AffineRows`, 8-float motors — zero runtime Clifford tags or categorical pointer chasing, honoring `franken_threed §7.2–7.5`!).
3. **P3 — Campaign-Time Quotienting Before Step-Time Brute Force.** Delete redundant simulations via `.fquot`, Burnside orbits, `.fmodule` substitution reuse, H/K gait atlases, homotopy synthesis, and Observation-Alias impossibility witnesses before burning GPU-hours on RL.
4. **P4 — Structure Preservation + Honest Contact Physics.** Strictly time-reversible Trapezoidal-Potential LGVI, Discrete Lagrange-d'Alembert nonholonomic Pfaffian rolling, 4D Coulomb–Contensou cones + joint stiction/backlash, extensive-entropy pHS actuators, and conservative-flux 2.5D causal tribology fields preserve physical invariants before hyperparameter tuning.
5. **P5 — Reflective Proofs & Stratified Safety — Zero Claim Laundering.** Direct proved `fln-kernel` computational reflection verifies algebraic invariants; impact safety uses `.fenergy` (zero statistical gap), low-D resets use `.ftube`, and high-DoF control uses box-constrained fused shields wrapped in non-promotable `ClaimProduct<VerifiedFormal, EvaluatedDesign>`.
6. **P6 — Asymmetric Dual-Silicon Roles With Symmetric Numerical Parity.** `GATE-GPU-TWIN-01` locks CPU `f64` ↔ Metal ↔ CUDA numerical parity, while `frob-gpu::fleet` exploits **role asymmetry**: M5 Ultra (256 GiB UMA) = World/Morphology Bank + Symbolic/Proof Host + Time-Travel Studio; Dual RTX 4090 (`SM_89`) = Specialist Training & Homotopy Foundries, communicating strictly *seeds-not-voxels*.
7. **P7 — Single-Authority Living World Truth.** One immutable 3-Tier `WorldScene` feeds contact, terramechanics, causal tribology, ray-traced LiDAR/RGB-D (glass/steam/skew), `.fclear` clearance audits, and rendering identically.
8. **P8 — Cancel-Correctness, DPOR Timing Rigor & Alpha-Spending Integrity.** `asupersync` budgets govern all GPU/CPU scopes; stream-ordered CUDA pools prevent cancellation stalls; DPOR checks sensor/watchdog/commit event orderings; a Blinded Qualification Ledger prevents adaptive AI p-hacking across campaigns.
9. **P9 — Honest Evidence & Actionable Refusals.** Every metric carries its Claim Axis (§0.4) and uncertainty envelope; every refusal (`FROB-REF-nnn`) emits ranked `.fbot`/`.fmodule` repairs or Farkas/observation-alias separating witnesses.
10. **P10 — Spec-to-Silicon Foundry Ergonomics.** One `registries/verbs.toml` drives `frob`/`frobd` CLI, async `frob-mcp`, `frob-tui`, `frob-view` (`franken_threed` WebGPU dossiers), and `.fbuild` manufacturable CAD/BOM/harness/`#![no_std]` firmware packages (`hardware_safety_signoff: false`).

---

## 3. The Sixteen Architectural Bets (Each With Its Crisp Falsifier)

| ID | Tag | Architectural Bet | Concrete Falsifier (`✂ Exclusion-Test` — What Proves Us Wrong) | Tested At |
|---|---|---|---|---|
| **`BET-01`** | `[F]` | **MorphoJIT + Limb Port-ABI + Ragged Contact Queues + Race Annealing (`frob-morphvm`, `CC-01`/`AGY #1`/`W02`/`TA-P2`).** 6-opcode screw-bytecode Tier-0 + Limb Port-ABI Tier-0.5 + ragged contact-island queues + async race-annealed Tier-1 specialization simulate 512 distinct topologies in one GPU grid with 0 ms topology-mutation stall. | In `XT-P2` (`H512`, 512 topologies on M5 Ultra & Dual 4090), Tier-0+Port-ABI fails to beat per-morphology graphs by `≥10×` at $n_m \le 128$, OR fails `≥0.33×` homogeneous Tier-1 DoF-normalized speed, OR 9 per-family templates (`H2b`) achieve `≥95%` mutation coverage and higher campaign throughput (`KILL-MORPHVM-1`). | M1–M2 (`GATE-MORPHVM-01`, `XT-P2`) |
| **`BET-02`** | `[F]` | **Campaign-Time Algebraic Quotienting & `.fquot` (`TA-P1` + `COD FTA-P1` + `frob-operad`).** Decorated-cospan `.fmodule` substitution contracts, Burnside dedup, exact mission quotients (`.fquot`), and H/K gait screening delete `≥80%–90%` of design-campaign GPU work vs per-design RL. | In `XT-P1` (128 designs + blinded stratified discard audit + ballast potency fixture `735,471 → 24,546`), `A2` fails `≥5×` speedup over `A0`, misses `≥6/8` top-8 overlap, drops planted dominant design from #1, or admits 1 false quotient equivalence (`KILL-OPERAD-1`). | M1–M4 (`GATE-OPERAD-01`, `XT-P1`) |
| **`BET-03`** | `[F]` | **H/K Equivariant Gait Atlas & Saltation Continuation (`frob-gaitatlas`, `CC-02`/`W07`).** H/K enumeration (`decide`), symmetry-reduced shooting, `fs-spectral::floquet` transient spectra, saltation adjoints, and Envelope Theorem gradients accelerate periodic morphology co-design `≥13×–100×` over scratch RL/FD. | On 100 periodic cases (incl. 20 grazing/unstable controls): fails to refuse all 20 controls, `<72/80` hit `cosine ≥ 0.95` vs central FD, continuation fails on `≥50%` of catalog morphologies, or RL warm-start speedup `<2×` (`KILL-ATLAS-1`). | M4–M5 (`GATE-ATLAS-01`) |
| **`BET-04`** | `[F]` | **Constructive Linkage & Homotopy Synthesis (`frob-symbolic` `W06` + `frob-homotopy` `CC-04`).** Bounded 3/5-pose rational + `real_rur` (M1, eliminant deg ≤ 64) plus GPU polyhedral/parameter homotopy + Smale $\alpha$-cert (M3) enumerate ALL generic complex isolated roots $N_{\mathrm{gen}}$ (`4,326` 9-point 4-bars = `1,442×3` cognates, `40` Stewart, `16` 6R in `≤2 s` re-query) and certify every real physical branch! | Misses generic complex counts (`4,326` / `40` / `16`) or known all-real fixtures (Dietmaier 40-real Stewart, Manseur–Doty 16-real 6R), `>0.1%` unrecovered paths, disagrees with trace test, or admits 1 false continuous branch (`KILL-HOMOTOPY-1`). | M1/M3 (`GATE-BURMESTER-01`, `GATE-HOMOTOPY-01`) |
| **`BET-05`** | `[F]` | **Proved `fln-kernel` Reflective Proofs & Stratified Safety (`.fenergy`, `.ftube`, `ClaimProduct` Shields).** Direct `fln-kernel` reflection checks inertia, universal Echeandia–Wensing skew, linear DAG codegen, `.fenergy` boundary budgets, `.ftube` resets, and box-constrained CBF shields (`∃u ∀w`) at `≤5%` GPU overhead. | Reflection soundness fails, any `.fenergy`/`.ftube` false admission occurs, shifted-world `XT-P3` violates composed `ClaimProduct` bound, shield cuts nominal return `<85%`, or Lean check exceeds `5 s` (`KILL-PROOF-1`, `KILL-SHIELD-1`). | M1/M4 (`GATE-PROOF-01`, `GATE-SHIELD-01`, `XT-P3`) |
| **`BET-06`** | `[F]` | **Physical Capability-Map Transfer (`.fskill` `W03`) + Contact-Consistent `FiberMorph` (`C004`).** Exact per-mode affine maps $w = w_0 + B_m(q)u$ + bounded WBC-QP + Farkas infeasibility witnesses + frozen task-space residual transfer skills zero-shot to unseen morphologies. | On 24 unseen morphologies (4 infeasible), `<18/20` feasible bodies hit `≥90%` success within 3 pp of tuned baselines zero-shot, 1 infeasible body is falsely admitted, or amortized transfer misses `≥10×` at declared denominator (`KILL-TRANSFER-1`). | M5 (`GATE-TRANSFER-01`) |
| **`BET-07`** | `[F]` | **Trapezoidal-Potential LGVI + Unified 4D Contact & Joint Stiction/Backlash (`frob-dyn`, `frob-contact`).** Symplectic time-reversible LGVI + Discrete Lagrange-d'Alembert wheels/mecanum/tracks + 4D Coulomb–Contensou cones + Factor-Once Multi-RHS `W10` pass `KA-01`–`KA-10` without drift or pyramidal bias. | Fails `KA-01`–`KA-10` tolerances, implicit LGVI costs `>2×` explicit Lie-midpoint on GPU (`KILL-LGVI-1`), unified Pfaffian+impact fails convergence (`KILL-CONTACT-1`), or `W10` multi-RHS fails `1e-6` residual / `1e-4` `f64` parity. | M1–M3 (`GATE-DYNAM-01`, `GATE-CONTACT-02`) |
| **`BET-08`** | `[S/F]` | **Breadth/Depth Dual-Silicon Fleet & 3-Tier GPU `WorldScene` (`frob-gpu::fleet`, `frob-world`, `CC-05`).** M5 Ultra hosts 500–1,000 resident worlds + CPU proof/symbolic co-residency; Dual 4090s run specialist foundries; *seed-not-voxels* keeps LAN traffic `≤1 GB/campaign-hr`; `fs-tropical` balances critical circuits. | 1,000-world bank fails to cut held-out generalization gap vs 100-world 4090 training AND dual-machine campaign speedup is `<1.3×` (`KILL-FLEET-1`), OR `GATE-FORGE-01` misses roofline/throughput floors (`KILL-GPU-1`). | M2–M4 (`GATE-FORGE-01`, `GATE-FLEET-01`) |
| **`BET-09`** | `[F]` | **Living Multiphysics Interiors: Conservative 2.5D Causal Tribology + 3-Tier Zoom-Lens (`frob-multiphys`, `CC-08`/`C011`).** Robot-advected thin-film PDE (mask inside flux + wheel mass balance) + 3-mode pHS slosh + Cosserat cords run at `≤15%` RL overhead, audited by `ftx-drape`/`fs-lbm` Tier-B. | Fails Huppert $t^{1/8}$ (`±2%`) or Lamb–Housner (`±3%`), causal fields change `<20%` of kitchen design rankings AND fail to cut causal-spill failures `≥3×`, or overhead exceeds `25%` (`KILL-TRIBO-1`). | M3–M5 (`GATE-TRIBO-01`, `GATE-MULTI-01`) |
| **`BET-10`** | `[F]` | **CEGIS Red-Teaming (`ddmin` `FMT-MCW-1`), `2^10` Time-Travel Autopsy & Robot–Home Retrofit Co-Design (`frob-redteam`, `frob-diagnose`, `frob-worldgen::amend`, `CC-03`/`AGY #10`/`B3`).** Certified-solvable adversarial search + minimal 3-edit shrinking + 1,024-way M5 UMA counterfactual forks (`do(...)` Shapley) + dual ≤3-item building-amendment kits (`10,700` kits) slash rare-failure discovery, repair cycles, and system cost per unlocked mission. | Finds planted $p \le 3\times 10^{-5}$ traps in `<3×` fewer episodes than uniform DR, `<70%` counterexamples are candidate-body solvable/reproducible (`KILL-REDTEAM-1`), counterfactual autopsy cuts agent repair iterations `<3×` (`KILL-TIMETRAVEL-1`), OR ≤3-item retrofit kits fix `<30%` of baseline SKU interior failures or fail bench test (`KILL-AMEND-1`). | M3–M5 (`GATE-REDTEAM-01`, `GATE-TIMETRAVEL-01`, `GATE-AMEND-01`) |
| **`BET-11`** | `[F]` | **$\mathbb{R}/\mathbb{C}/\mathbb{H}$ + Wreath-Product Equivariant RL on `GpuTensorView` (`frob-equiv`, `frob-gym`).** Per-space character projectors (handling $C_n$ complex irreps!) + regular-rep hidden layers + wreath products for heterogeneous limbs accelerate RL `≥1.8×`–`5×` with zero CPU `DenseTensor` copies. | 10-seed controlled sweeps show `<1.3×` sample-efficiency gain or `>3%` asymptotic regression on cluttered interiors (`KILL-EQUIV-1`), or wreath-product sharing shows `<1.5×` on heterogeneous-limb robots. | M4 (`GATE-EQUIV-01`) |
| **`BET-12`** | `[S/F]` | **Instant 8-Stage Preflight + Observation-Alias Impossibility Witness + Dual `.fclear` (`frob-preflight`, `frob-worldgen`, `W08` + `BS-1`).** Rejects unphysical inertias, zero curvature, doorway traps (`.fclear`), and sensor-blind missions (`observation_conflict`) in `<200 ms`–seconds before training. | Preflight misses `>20%` of 100 seeded defects (`GATE-PREFLIGHT-01`), `.fclear` emits 1 false route or false impossibility witness, or `observation_conflict` emits 1 false impossibility witness on negative controls. | M1–M3 (`GATE-PREFLIGHT-01`, `GATE-CLEAR-01`, `GATE-OBSALIAS-01`) |
| **`BET-13`** | `[S/F]` | **Extensive-Entropy pHS Actuators & E–Engquist HMM Lifecycle Compression (`frob-actuators`, `frob-lifecycle`, `CC-09`).** Thermal runaway, battery sag, and 2-timescale HMM wear/fatigue/SEI averaging (preserving rainflow residues) compress 1,000-hour age checks `≥100×` at `≤5%` error vs brute force. | `KA-09` fails, HMM compression `<50×` at `5%` error on 100-h reference, or misses planted backlash-induced Floquet gait bifurcation (`KILL-LIFE-1`). | M3/M5 (`GATE-ACT-01`, `GATE-LIFE-01`) |
| **`BET-14`** | `[S/F]` | **Epistemic Integrity Suite: Order-Independent GPU Replay (`B1`), Model-Form & Energy-Creation Audit (`B2`), DPOR Timing (`BS-2`) & Blinded $\alpha$-Spending Qualification (`BS-3`).** Order-independent GPU scatter + `StateHash` protect CRN/Shapley; cross-solver rankcheck + discrete $\Delta E_{\mathrm{mech}} - W_{\mathrm{act}} - W_{\mathrm{ext}} \le \varepsilon$ block solver-artifact exploits; DPOR cuts schedule checks up to `90×`; Discovery vs Blinded Confirmation ($\alpha_i = \alpha/[i(i+1)]$) prevents adaptive AI p-hacking. | Deterministic GPU path adds `>10%` step overhead (`KILL-DET-1`), model-form audit misses planted diagonal-pyramid or energy-pumping exploit (`GATE-RANKCHECK-01`), DPOR misses 1 planted race (`KILL-DPOR-1`), OR adaptive AI retries over a no-signal world family exceed nominal campaign false-promotion rate $\alpha$ (`GATE-QUAL-01`). | M2/M4/M5 (`GATE-DET-01`, `GATE-DPOR-01`, `GATE-RANKCHECK-01`, `GATE-QUAL-01`) |
| **`BET-15`** | `[S/F]` | **Spec-to-Silicon Foundry: Unit-Normalized STL + `frob-mfg` `.fbuild` + `#![no_std]` Firmware + `franken_threed` WebGPU Dossier (`frob-spec`, `frob-mfg`, `frob-codegen`, `frob-view`, `CC-10`/`W09`/`C013`–`C015`).** Compiles typed STL missions (unit-normalized past-time online reward + delayed future-time verdicts) into review-ready COTS/GD&T/STEP/harness packets, QEMU-verified `#![no_std]` controllers with real-board WCET receipts, and `≤15 MB` offline WebGPU dossiers. | Independent reviewer accepts `<11/12` packets, sampled STL robustness disagrees with offline reference on any audited trace, firmware trace diverges `>1e-5` from Tier-0 or allocates heap, or fresh agent solves `<5/10` missions unattended (`KILL-S2S-1`). | M4–M6 (`GATE-S2S-01`, `GATE-FW-TWIN-01`, `GATE-WEBSTUDIO-01`) |
| **`BET-16`** | `[S/F]` | **Uncertainty-Coupled `fdgr` Real-to-Sim-to-Real Loop (`frob-io`, `frob-redteam`, `O209`).** Mapping `fdgr` geometric/coverage deficits into red-team uncertainty bands & emitting active Re-Scan/Material-Probe requests designs uncertainty-hardened bespoke building robots. | Fails to localize 100% of seeded occluded structural bottlenecks across 5 real-scan fixtures or fails to tag unprobed surfaces `MaterialAuthority::UnprobedConservative`. | M5–M6 (`GATE-REAL2SIM-01`) |

---

## 4. Goals, Non-Goals, and Anti-Goals

### 4.1 Primary Goals (`GOAL-001`–`GOAL-016`)
- **`GOAL-001`** Coordinate-free Lie/PGA/$\mathrm{Sym}^+(4)$ & principal-bundle geometric kernel (`frob-lie`, `frob-bundle`).
- **`GOAL-002`** Structure-preserving LGVI + DLA nonholonomic + 4D Coulomb–Contensou/IPC contact dynamics (`frob-dyn`, `frob-contact`).
- **`GOAL-003`** `frankensympy` symbolic PoE/RNEA/ABA/Coriolis & bounded linkage synthesis (`frob-symbolic`, `frob-kernel-ir`, `frob-codegen`).
- **`GOAL-004`** Proved `franken_lean` (`fln-kernel`) reflective proofs & offline `frob-verify` auditing (`frob-proof`, `frob-verify`).
- **`GOAL-005`** First-class dual-silicon Metal (M5 Ultra) + CUDA (Dual RTX 4090) GPU Forge (`frob-gpu`, `frob-gpu-metal`, `frob-gpu-cuda`).
- **`GOAL-006`** Single-authority 3-Tier `WorldScene` across 6 realistic interior archetypes & Bekker–Wong outdoor terrains (`frob-world`, `frob-worldgen`, `frob-terramech`).
- **`GOAL-007`** Unified `FMT-FBOT-1` morphology IR covering all 18 legged, wheeled, tracked, hybrid, and continuum archetypes (`frob-bot`, `frob-actuators`).
- **`GOAL-008`** Realistic GPU ray-traced LiDAR/RGB-D/event/tactile sensors, anchored $\mathfrak{se}(3)$ sheaf calibration & Imperfect $SE_{2+m}(3)$ InEKF (`frob-sensors`, `frob-estimate`).
- **`GOAL-009`** $\mathbb{R}/\mathbb{C}/\mathbb{H}$ + wreath equivariant GPU RL & algebraic morphology co-design (`frob-equiv`, `frob-gym`, `frob-codesign`).
- **`GOAL-010`** Unified ATELIER AI-agent Robot Design Studio (`frob`/`frobd`, async `frob-mcp`, `frob-tui`, `frob-view`, `frob-report`).
- **`GOAL-011`** Five non-conflated Claim Axes + compile-time non-promotable `ClaimProduct` honesty (`frob-evidence`, `frob-ledger`).
- **`GOAL-012`** Fuzzed pure-Rust URDF/MJCF/glTF/USD/`fdgr` interop (`frob-io`).
- **`GOAL-013`** (**Campaign-Time Algebraic Quotienting & MorphoJIT**): Run 512 distinct robot topologies simultaneously via `MorphoJIT` + Limb Port-ABI + ragged contact queues + race-annealed specialization, while deleting redundant runs via `.fmodule`, `.fquot`, H/K gait atlases, homotopy synthesis, and Observation-Alias witnesses.
- **`GOAL-014`** (**Zero-Laundering Stratified Safety & Reflective Proofs**): Ship proved `fln-kernel` reflection powering `.fenergy`, `.ftube`, and box-constrained `ClaimProduct` shields.
- **`GOAL-015`** (**Living Multiphysics, CEGIS Red-Teaming, Time-Travel Autopsy & Retrofits**): Ship conservative 2.5D causal tribology fields, 3-tier zoom-lens (`ftx-drape`/`fs-lbm`), `.fclear` clearance complexes, `ddmin` MCW CEGIS, `B3` building-retrofit co-design, `2^10` Pearl-causal counterfactual forks, and E–Engquist HMM lifecycle compression.
- **`GOAL-016`** (**Spec-to-Silicon Foundry & Epistemic Firewall**): Ship typed `.ftask` STL contracts, `B1` deterministic GPU scatter, `B2` model-form/energy-creation audit, `BS-3` blinded $\alpha$-spending qualification, `BS-2` DPOR timing checks, COTS/GD&T/harness/STEP `.fbuild` packets, QEMU-verified `#![no_std]` firmware, and offline `franken_threed` WebGPU dossiers.

### 4.2 Explicit Non-Goals (`NONGOAL-001`–`NONGOAL-007`)
- `NONGOAL-001` **Direct physical motor actuation / safety-rated PLC/MCU BSP generation / hardware safety signoff** (`execution_authorized: false`, `hardware_safety_signoff: false`; generated firmware sits strictly behind caller-owned `MotorBusHal`!).
- `NONGOAL-002` **Wrapping C++ physics engines** (MuJoCo C, PhysX, Bullet, ODE, Dart) as runtime backends (dev-only pinned oracles in `frob-bench` only).
- `NONGOAL-003` **Hollywood VFX photorealism or interactive browser CAD sculpting**.
- `NONGOAL-004` **Full computational aerodynamics / high-Re Navier–Stokes aircraft design** (aerial-terrestrial hybrids use compact pHS rotor-thruster + Cheeseman–Bennett ground-effect + fuselage drag ports only).
- `NONGOAL-005` **Full-room brute-force 3D LBM / IPC cloth inside 32k-env RL loops** (`† K-04`: 63.7 TB impossibility — strictly 3-tier zoom-lens!).
- `NONGOAL-006` **Unbounded Lean proofs of unstructured real-world contact chaos or RL global convergence** (strictly stratified model-scoped proofs + `ClaimProduct`!).
- `NONGOAL-007` **Single-RHS FP16 Tensor-Core contact factorization claiming universal conditioning** (`† K-01`: mathematically refuted at $\kappa = 4{,}801$!).

### 4.3 Anti-Goals (Explicitly Rejected Even If Requested)
1. Silent clamping of non-SPD inertias, overflowed contacts, or uncertified GPU drift.
2. Separate disconnected physics, sensor, and visual world representations.
3. Mixing CPU-only `frankentorch::DenseTensor` copies or `rayon` pools into hot GPU RL loops.
4. Decorative math prose without a Rust crate, reflective Lean checker, or benchmark gate.
5. Adaptive AI tuning directly against the confirmation benchmark (blocked by `frob-ledger::qualification`).

---

## 5. North-Star Scenarios (`NS-01`–`NS-16`, Before → After; `TARGET`)

| ID | Scenario | Incumbent Pain ("Before") | `franken_robotics` v2.0 Experience ("After") | Target Milestone |
|---|---|---|---|---|
| `NS-01` | **Instant Algebraic & Observation-Alias Preflight** | Agent edits URDF blindly; burns 6 GPU-hours discovering hip singularity, triangle-inequality violation, or unobservable blind spot. | `frob verify` catches $\mathrm{Sym}^+(4)$, $D\mathbf{A}_S=0$, `.fclear` doorway pinch, AND proves an `observation_conflict` blind spot with the cheapest sensor fix in `<200 ms`–`2 s`. | M1 / M3 |
| `NS-02` | **512-Topology MorphoJIT Population Tournament** | Evolving 512 distinct robot structures forces 512 XLA/USD rebuilds and stalls GPUs at `<5%` utilization. | `frob-morphvm` steps all 512 distinct topologies in one GPU grid (`B ≥ 65,536` envs = `8–16` warps/SM); `fs-race` anneals `512×128 → 8×8,192` while promoting survivors to Tier-1 kernels asynchronously with zero stall. | M2 / M5 |
| `NS-03` | **Exact Closed-Loop Parallel Leg & Ankle Synthesis** | 99.9% of random 4-bar / parallel-ankle link mutations break loop closure; manual CAD takes days. | `W06` (`real_rur`, M1, eliminant deg ≤ 64) synthesizes exact 3/5-pose linkages in `<100 ms`; `frob-homotopy` (`CC-04`, M3) tracks all `4,326` (`1,442×3` cognates) generic complex 9-point 4-bars & `40` Stewart roots in `≤2 s` re-query and certifies every real branch! | M1 / M3 |
| `NS-04` | **Commercial Kitchen Wheel-Quad Under Causal Grease & Steam** | Static friction `0.8` sim passes; real robot hydroplanes on spread grease, blinds RGB-D in steam, and overheats knee motor in 33.5 s. | 2.5D causal tribology film spreads under tires, steam scatters RGB-D, pHS knee triggers `FAIL-THERMAL-CLIFF`, and CEGIS + counterfactual forks repair `N_knee` + `rgbd_mount_z` / `lidar_mount_z` in minutes! | M3 / M5 |
| `NS-05` | **Hexapod Spider $D_6$ Equivariant Rubble & Fault Recovery** | Unstructured MLP takes 50M steps and limps; losing 1 leg crashes policy. | `frob-symmetry` proves $D_6$ (`PO-EQUIV-1`), cuts weights `12×`, accelerates RL `≥5×`, and H/K gait atlas + `.fskill` transfer re-allocate wrenches when a leg disables. | M4 |
| `NS-06` | **Humanoid (`frob-g1-29dof`) Stratified Safety in Offices & Plants** | Penalty-shaped RL falls on glass doors or violates joint limits during exploration. | `.fenergy` bounds stop impacts with zero stats, `.ftube` certifies threshold resets, and box-constrained `ClaimProduct` shields keep exploration violation-free (`≤5%` overhead). | M4 |
| `NS-07` | **Warehouse & Retail Mobile Manipulator in Cluttered Aisles** | Vision policies fail on freezer glass, repetitive aisles (SLAM aliasing), and narrow 1.2 m shelf turns. | `.fclear` certifies $(x,y,\psi)$ shelf reach, `frob-sensors` models glass multi-path, `frob-estimate` optimizes LiDAR observability, and IPC handles fragile SKU grasps. | M3 / M4 / M6 |
| `NS-08` | **H/K Gait Atlas & Zero-RL Morphology Continuation** | Lengthening a femur +15% requires 15–45 min RL retraining just to check CoT and stability. | `frob-gaitatlas` continues the exact saltation-monodromy orbit in `<1 s`, flags period-doubling bifurcations via `fs-spectral::floquet`, and returns Envelope-Theorem gradients! | M4 / M5 |
| `NS-09` | **Zero-Shot `.fskill` Transfer & Infeasibility Witnesses** | Swapping a wheel for a foot or adding a 5th leg forces full retraining even when physically impossible. | `.fskill` solves constrained wrench allocation $w = w_0 + B_m(q)u$ zero-shot across unseen morphologies and emits a **Farkas separating witness** before training if infeasible! | M5 |
| `NS-10` | **Tracked Rover Outdoor Bog & Flipper Stair Ascent** | Rigid-ground sim misses wet-clay belly-out and grouser scrub torque. | Prescribed-belt bogies + Bekker–Wong/Janosi–Hanamoto shear (`KA-08`) catch 9.8 cm sinkage belly-out and co-optimize track width (`8 → 12 cm`) in seconds. | M3 |
| `NS-11` | **CEGIS Red-Team Minimal Counterexample & `2^10` Time-Travel Autopsy** | Agent stares at scalar reward `-42` and guesses random XML tweaks. | `frob-redteam` shrinks a rare trap to ≤3 edits (`FMT-MCW-1`); M5 Ultra UMA forks **1,024 Pearl `do(...)` branches in `<1 s`** returning exact Shapley causes + an offline `franken_threed` WebGPU multiverse `.html`! | M3 / M5 |
| `NS-12` | **Walk-Your-Building Uncertainty-Coupled `fdgr` Foundry** | Photogrammetry-to-USD pretends noisy scans are exact and misses occluded pinch points. | `frob-io` ingests `fdgr` mesh + coverage deficits directly into `frob-redteam`'s prior, designs conservatively around blind spots, and emits a targeted **`fdgr` Re-Scan Request** if occlusion bottlenecks BOM! | M5 / M6 |
| `NS-13` | **1,000-Hour Year-2 Reliability & DPOR Timing Hardening** | Minute-1 demo passes; field robot fails at Month 8 from gear backlash + battery sag or intermittent watchdog/commit timing race. | `frob-lifecycle` HMM compresses 1,000 h in `<45 s` exposing backlash-induced Floquet collapse; `frob-runtime::schedule_check` DPOR catches the exact watchdog/commit race in 8 replays (vs 720)! | M5 |
| `NS-14` | **Blinded Campaign Qualification & Offline `.frobcert` Audit** | Autonomous agent adaptively p-hacks the held-out suite across 50 retries; results aren't auditable offline. | `frob-ledger::qualification` enforces $\alpha_i = \alpha/[i(i+1)]$ on blinded world clusters; `frob-verify` replays `fln-checker` proofs + Merkle lineage offline with zero GPU/network. | M5 / M6 |
| `NS-15` | **Spec-to-Silicon: Prompt → `.fbuild` CAD/BOM/Harness + `#![no_std]` Firmware** | Co-design outputs unbuildable fantasy capsules; ME & firmware engineers spend 6 months rewriting C++. | `.fmission` STL compiles to exact Lemire rewards → `frob-mfg` outputs FreeCAD-valid faceted STEP + priced COTS BOM + adjoint `fs-toleralloc` GD&T + Steiner/Cosserat harness + QEMU-verified `#![no_std]` controller crate (`hardware_safety_signoff: false`)! | M4 / M6 |
| `NS-16` | **Breadth/Depth Dual-Silicon Asymmetric Studio Session** | Mac Studio 256 GiB sits unused while 4090s choke on 24 GiB VRAM scene swaps. | M5 Ultra hosts 500–1,000 hot worlds + interactive `NativePresenter` + time-travel; Dual 4090s execute specialist training & homotopy via `asupersync` QUIC (*seed-not-voxels*, `<1 GB/hr`). | M2 / M4 |

---


# PART II — THE ALGEBRAIC, OPERADIC, HOMOTOPY & REFLECTIVE-PROOF SPINE

> **Architectural Rule (P2 & P3):** Every algebraic object below is **executable software** — compiled either into a campaign-quotienting theorem/reduction before GPU rollout, a branch-free packed GPU data plane (`AffineRows`, `MorphoTable`, 8-float motors), or a proved `fln-kernel` reflective proof term. Zero unverified hand-waving and zero runtime Clifford pointer chasing.

---

## 6. Coordinate-Free Geometry: Normative Conventions, PGA/CGA & $\mathrm{Sym}^+(4)$ (`frob-lie`)

### 6.0 Normative Mathematical Conventions Table (Pinned to `frankensim::fs-ga`, `DESIGN`)

To prevent sign-flip, left/right-trivialization, and basis-ordering bugs across 51 crates and 4 codegen targets, every crate MUST obey this single normative table (checked by `KA-01` and `PO-LIE-1`):

| Symbol / Concept | Normative Definition in `franken_robotics` | Runtime Packing / Notes |
|---|---|---|
| **Pose $g \in SE(3)$** | $g = (R, p)$ with $R \in SO(3), p \in \mathbb{R}^3$, transforming body-frame points $x_b \mapsto R x_b + p$ in world frame. | Authoring/proofs: unit PGA motor $M \in \mathrm{Spin}^+(3,0,1)$ (`[8 × f32/f64]`). GPU hot-loop spatial transform: **`AffineRows`** (`[3 × 4]` row-major `f32` = **48 B**, per `franken_threed §7.2–7.5`). |
| **Body Twist $\xi \in \mathfrak{se}(3)$** | Left-trivialized body velocity $\hat{\xi} = g^{-1}\dot{g}$, ordered **angular-first**: $\xi = (\omega, v) \in \mathbb{R}^6$ (`[ω_x, ω_y, ω_z, v_x, v_y, v_z]`). | Matches Featherstone & `fs-mbd`. Spatial cross product $\mathrm{ad}_\xi = \begin{bmatrix} [\omega]_\times & 0 \\ [v]_\times & [\omega]_\times \end{bmatrix}$. |
| **Body Wrench $w \in \mathfrak{se}(3)^*$** | Dual pairing $\langle w, \xi \rangle = \tau^T \omega + f^T v$, ordered **torque-first**: $w = (\tau, f) \in \mathbb{R}^6$. | Coadjoint action $\mathrm{ad}_\xi^* w := \mathrm{ad}_\xi^T w = (\tau \times \omega + f \times v,\; f \times \omega)$ (note: **transpose, NOT inverse-transpose**, for $\mathfrak{se}(3)^*$ Euler–Poincaré $\dot{\mu} = \mathrm{ad}_\xi^* \mu + w_{\mathrm{ext}}$!). |
| **Adjoint & Coadjoint Maps** | $\mathrm{Ad}_g = \begin{bmatrix} R & 0 \\ [p]_\times R & R \end{bmatrix}$; wrench transform from body $b$ to world $w$: $w_w = \mathrm{Ad}_{g^{-1}}^T w_b = \mathrm{Ad}_g^{-T} w_b$, while pullback from $w$ to $b$ is $w_b = \mathrm{Ad}_g^T w_w$. | Explicit typed newtypes `AdjointMap` vs `WrenchPullback` (`Ad_g^T`) vs `WrenchPushforward` (`Ad_g^{-T}`) prevent silent transpose errors! |
| **PGA `Cl(3,0,1)` Basis** | Pinned to `fs-ga`: $e_0^2 = 0,\; e_1^2 = e_2^2 = e_3^2 = +1$, pseudoscalar $I = e_0 e_1 e_2 e_3$. Planes are grade-1 vectors $n_x e_1 + n_y e_2 + n_z e_3 + d e_0$; lines are grade-2 bivectors; points are grade-3 trivectors. | Sandwich product $x' = M x \widetilde{M}$ (with parity sign $(-1)^{\mathrm{gr}(x)}$ on odd versors/reflections). |
| **Improper $O(3)$ Reflections** | Affine reflection $g_v = \begin{bmatrix} R & p \\ 0 & 1 \end{bmatrix}$ ($\det R = -1$) acts on twists as **$\mathrm{Ad}_{g_v} = \begin{bmatrix} (\det R) R & 0 \\ (\det R)[p]_\times R & R \end{bmatrix}$** (reducing to $\mathrm{diag}((\det R) R,\, R)$ for origin-passing planes $p = 0$) and on pseudo-inertia $J \in \mathrm{Sym}^+(4)$ as $J' = g_v J g_v^T$. | Fixes v1's reflection-parity & affine-offset omissions for sagittal-mirror quadrupeds & humanoids! |
| **Small-Angle Lie Maps** | $\exp(\hat{\xi})$, $\log(g)$, $d\exp_\xi^{-1}$, and Cayley $\tau(\xi)$ MUST use branch-free degree-4 minimax/Taylor polynomials for $\mathrm{sinc}(\theta) = \frac{\sin\theta}{\theta}$, $\mathrm{cosc}(\theta) = \frac{1-\cos\theta}{\theta^2}$, and $\frac{\theta - \sin\theta}{\theta^3}$ when $\|\omega\| < 10^{-4}$ rad. | Eliminates catastrophic $0/0$ cancellation in `MorphoJIT` and `frob-adjoint` (`C12`)! |

### 6.1 PGA, CGA & Exact Geometric Primitives (`frob-lie`, `[S]`)
- **PGA (`Cl(3,0,1)`)** unifies screws, Plücker joint axes, planes, rays, and rigid motors in one grade-typed algebra consumed directly from `fs-ga` (`fs-qty`-free leaf!). Join ($\vee$) and meet ($\wedge$) compute exact line–plane intersections, wrench axes, and reciprocal screw subspaces without gimbal lock.
- **Conformal GA (`Cl(4,1)`)** powers exact workspace-sphere, capsule-sweep, and linkage-circle intersections at **compile/preflight time** (`frob-symbolic`, `frob-preflight`), then erases down to flat FP32/FP64 capsule/box/SDF arrays on the GPU (`P2`).
- **Dual-Number Automatic Differentiation** (`Dual<T>` + `Se3Dual`) lifts seamlessly through `exp`, `log`, and `Ad` to verify every analytic Jacobian and adjoint in `frob-adjoint` and `frob-bench`.

### 6.2 The $\mathrm{Sym}^+(4)$ Pseudo-Inertia Manifold (`frob-lie`, `[S]`)
Every rigid body's 10 inertial parameters $(m, m c_x, m c_y, m c_z, I_{xx}, I_{yy}, I_{zz}, I_{xy}, I_{xz}, I_{yz})$ embed bijectively into a $4\times 4$ symmetric positive-definite **pseudo-inertia matrix** (Traversaro–Wensing):
$$J = \begin{bmatrix} \Sigma & h \\ h^T & m \end{bmatrix} \in \mathrm{Sym}^+(4), \qquad \Sigma = \frac{1}{2}\mathrm{tr}(I_{\mathrm{com}})I_3 - I_{\mathrm{com}} + m c c^T, \qquad h = m c$$
- **Exact Physical Realizability Theorem:** A 10-tuple corresponds to a non-negative mass density distribution in $\mathbb{R}^3$ **if and only if** $J \succ 0$ (which automatically enforces $m > 0$, $I_{\mathrm{com}} \succ 0$, AND all three principal-moment triangle inequalities $\lambda_1 + \lambda_2 > \lambda_3$!).
- **Optimization Without Invalid Designs:** Every co-design optimizer (`frob-codesign`), CAD synthesis pass (`frob-mfg`), and system-ID step (`frob-estimate`) parameterizes body inertia strictly via its **log-Euclidean tangent** $S = \log(J) \in \mathrm{Sym}(4)$ (mapped back by $J = \exp(S) \in \mathrm{Sym}^+(4)$), making non-physical inertias **mathematically unrepresentable** during optimization and instantaneously refusable (`FROB-REF-101`) on URDF/MJCF import!

---

## 7. Principal Bundles, Mechanical Connection & Corrected Cartan Curvature (`frob-bundle`, `[F]`)

### 7.1 Principal Fiber Bundle Decomposition of Articulated Locomotion
The configuration space of any floating-base robot factors as a trivial principal $G$-bundle $Q = G \times S$, where $G = SE(3)$ (or $SE(2)$ on planar floors) is the base pose fiber and $S$ is the internal shape space (joint angles $r \in S$). Because kinetic energy $T(g, r, \xi, \dot{r}) = \frac{1}{2}\begin{bmatrix}\xi \\ \dot{r}\end{bmatrix}^T \begin{bmatrix}\mathbb{I}(r) & \mathbf{I}_{BS}(r) \\ \mathbf{I}_{BS}(r)^T & M_{SS}(r)\end{bmatrix}\begin{bmatrix}\xi \\ \dot{r}\end{bmatrix}$ is $G$-invariant, the zero-momentum or nonholonomic kinematic reconstruction equation defines the **local mechanical connection 1-form** $\mathbf{A}_S(r) : T_r S \to \mathfrak{g}$:
$$\xi = -\mathbf{A}_S(r)\,\dot{r} + \mathbb{I}(r)^{-1} p_b, \qquad \mathbf{A}_S(r) = \mathbb{I}(r)^{-1}\mathbf{I}_{BS}(r)$$
For nonholonomic wheeled/snake/wheeled-legged robots, the Pfaffian constraints $W_g(r)\xi + W_r(r)\dot{r} = 0$ augment $\mathbf{A}_S(r)$ into the **principal kinematic connection** (Ostrowski–Burdick) whenever $\mathrm{rank}(W_g) = \dim G$, or the mixed **nonholonomic momentum connection** otherwise.

### 7.2 Exact Cartan Curvature $D\mathbf{A}_S$ & Path-Ordered Holonomy (`FACT`-Verified Sign!)
For a closed periodic gait loop $\gamma : [0, T] \to S$ enclosing surface $\Sigma$ ($\partial\Sigma = \gamma$), net body displacement $g(T) \in G$ equals the **right-ordered path-ordered exponential** of the negative connection, approximated to second order by the surface integral of the **Cartan curvature 2-form**:
$$g(T) = \overrightarrow{\mathcal{P}}\exp\!\left(-\oint_{\partial\Sigma} \mathbf{A}_S(r)\,dr\right), \qquad D\mathbf{A}_S = d\mathbf{A}_S - \frac{1}{2}[\mathbf{A}_S \wedge \mathbf{A}_S]$$
In local shape coordinates $(r^1, \dots, r^n)$, the exact coordinate components are:
$$(D\mathbf{A}_S)_{ij} = \frac{\partial A_j}{\partial r^i} - \frac{\partial A_i}{\partial r^j} - [A_i, A_j]_{\mathfrak{g}}$$
*(Verified numerically in [planning/verify_plan_math.py](file:///Users/jemanuel/projects/franken_robotics/planning/verify_plan_math.py) against exact path-ordered integration on an $SO(3)$ non-abelian holonomy loop — minus sign gives `err = 9.7e-4` vs wrong plus sign `2.026` — AND on an $SE(2)$ coupled kinematic snake loop `KA-03`!)*

**Executable Uses in `franken_robotics`:**
1. **Preflight Locomotion Controllability (`Stage 4`, `<5 ms`):** Compute $\mathrm{rank}\big(\mathrm{span}\{D\mathbf{A}_S(\partial_i, \partial_j),\, \nabla_k D\mathbf{A}_S\}\big)$ on $S$. If the curvature vanishes identically in yaw or forward translation (e.g., an inverted-symmetry swimmer/snake or misaligned mecanum roller angle), refuse immediately with `FROB-REF-104` before burning RL compute!
2. **Stratified Legged Bundles:** For legged robots, each stance contact stratum $\sigma \in \mathcal{S}$ defines its own stance-pinned fiber connection $\mathbf{A}_\sigma(r)$ with switching maps across lift-off/touchdown transitions, feeding directly into the H/K Gait Atlas (`frob-gaitatlas`) and gait seed generator.

---

## 8. Structure-Preserving Dynamics, Unified 4D Contact, GVS Cosserat & Extensive-Entropy pHS

### 8.1 Strictly Time-Reversible Trapezoidal-Potential LGVI (`frob-dyn`, `[F]`)
To prevent secular energy drift during 100-second trajectories, high-spin aerial rotors, and long-horizon HMM lifecycle rolls without paying implicit-midpoint Jacobian inversion penalties on every step, `frob-dyn` implements the **Trapezoidal-Potential Lie-Group Variational Integrator (LGVI)** on $SE(3) \times S$ (fixed from v1's non-reversible sign & scaling errors and verified in [planning/verify_plan_math.py](file:///Users/jemanuel/projects/franken_robotics/planning/verify_plan_math.py)):

Define the discrete step displacement via the Cayley map $\tau : \mathfrak{se}(3) \to SE(3)$ such that $g_{k+1} = g_k\,\tau(+h\xi_k)$ (`+h\xi_k`!). Crucially, $d\tau_u$ MUST be the **left-trivialized** differential $\tau(u)^{-1}\delta\tau(u) = d\tau_u^{\mathrm{left}}(\delta u)$ (using the opposite right-trivialized convention flips Step 2's sign and blows up energy by `23.6%` in 1 s!), whose exact $6\times 6$ $SE(3)$ inverse block formula (verified to `1e-16`!) is:
$$(d\tau_u^{\mathrm{left}})^{-1} = \begin{bmatrix} I + \tfrac{1}{2}[\omega]_\times + \tfrac{1}{4}\omega\omega^T & 0 \\ \tfrac{1}{2}[v]_\times + \tfrac{1}{4}[\omega]_\times[v]_\times & I + \tfrac{1}{2}[\omega]_\times \end{bmatrix}, \qquad u = (\omega, v) \in \mathfrak{se}(3)$$
Using the trapezoidal discrete Lagrangian
$$L_d(g_k, g_{k+1}) = \frac{h}{2}\xi_k^T \mathbb{M}\xi_k - \frac{h}{2}\Big(V(g_k) + V(g_{k+1})\Big), \qquad \mu_k^+ := \big(d\tau_{-h\xi_k}^{\mathrm{left}}\big)^{-T}(\mathbb{M}\xi_k)$$
the Discrete Euler–Poincaré (DEL) equations yield the **symplectic, momentum-preserving, strictly time-reversible** step $(g_k, \mu_k^-) \mapsto (g_{k+1}, \mu_{k+1}^-)$:
1. **Half-kick potential/actuation impulse:** $\mu_k^+ = \mu_k^- + \frac{h}{2}\,f_k^{\mathrm{pot}}(g_k) + \frac{h}{2}\,u_k$, where $f_k^{\mathrm{pot}}(g) = -T_e^* L_g\!\left(\frac{\partial V}{\partial g}\right)$ is the left-trivialized potential wrench.
2. **Exact Cayley-momentum solve for $\xi_k$:** Solve $(d\tau_{-h\xi_k}^{\mathrm{left}})^{-T}(\mathbb{M}\xi_k) = \mu_k^+$ (converges to `1e-13` in **2–3 Newton steps** using the closed-form $6\times 6$ Jacobian of $(d\tau^{\mathrm{left}})^{-T}$!).
3. **Exact manifold drift & transport:** $g_{k+1} = g_k\,\tau(h\xi_k)$ and $\mu_{k+1}^{\mathrm{free}} = \big(\mathrm{Ad}_{\tau(h\xi_k)}\big)^T \mu_k^+$ (exact coadjoint transport!).
4. **Second half-kick:** $\mu_{k+1}^- = \mu_{k+1}^{\mathrm{free}} + \frac{h}{2}\,f_{k+1}^{\mathrm{pot}}(g_{k+1}) + \frac{h}{2}\,u_{k+1}$.

- **Verified Accuracy (`FACT`, [planning/verify_plan_math.py](file:///Users/jemanuel/projects/franken_robotics/planning/verify_plan_math.py)):** On free asymmetric $SO(3)$ rotation (5k/40k steps), relative energy error is `5.62e-5` at `h = 2 ms` and **`8.79e-7 < 1e-6` at `h = 0.25 ms`**; on full-$SE(3)$ gravity + spring heavy-top integration (`KA-02`), exact $O(h^2)$ energy oscillation and backward time-reversibility to `1e-14` (`f64`) are confirmed!
- **GPU Fast Path & Kill Fallback (`KILL-LGVI-1`):** For articulated multibody trees, `frob-dyn` runs Articulated Body Algorithm (ABA) / CRBA inside a symplectic Lie-Verlet / RATTLE splitting (`frob-morphvm` Tier-0/1). If the implicit Cayley Newton solve exceeds `2×` explicit Lie-midpoint GPU wall time at matched accuracy on `GATE-DYNAM-01`, `KILL-LGVI-1` automatically locks explicit symplectic Lie-midpoint for RL training while retaining implicit Trapezoidal LGVI for CPU `f64` oracle & adjoint/evaluation passes!

### 8.2 Discrete Lagrange–d'Alembert Nonholonomic Constraints (`frob-dyn`, `[S]`)
Wheeled, mecanum, omni, swerve, and tracked robots MUST NOT be faked with spongy isotropic lateral penalty springs. `frob-dyn` enforces exact **Discrete Lagrange–d'Alembert (DLA)** linear velocity constraints $A_c(q)\,\dot{q} = b_c(q, t)$ directly inside the unified velocity-impulse solver:
- **Differential-drive / Ackermann / Swerve:** Zero lateral slip Pfaffian row $e_{\mathrm{lat}}^T v_{\mathrm{wheel}} = 0$ bounded by the Coulomb traction cone $\|f_t\| \le \mu f_n$ (naturally transitioning from pure rolling into controlled drift/skid when traction saturates!).
- **Omni / Mecanum Rollers:** Roller-axis free projection at roller angle $\alpha_r \in \{\pm 45^\circ, 90^\circ\}$, zero perpendicular roller slip bounded by roller-tire $\mu$.
- **Prescribed-Belt Tracked Bogies & Flippers:** Exact kinematic belt tangential speed $b_c = v_{\mathrm{belt}}(u_{\mathrm{sprocket}})$ along every active grouser contact patch + Bekker–Wong/Janosi–Hanamoto soil shear (`frob-terramech`), reproducing grouser scrub torque during skid-steering and stair-nosing climbs accurately (`KA-08`).

### 8.3 Unified 4D Coulomb–Contensou Contact, Joint Stiction/Backlash, 3-Tier SRAM Delassus & `W10` Multi-RHS (`frob-contact`, `[S/F]`)
Contact mechanics is the #1 accuracy and speed bottleneck in robotics. `frob-contact` repairs EVERY v1/duel contact defect:
1. **Honest Non-Associated NCP Formulation (4D Ellipsoidal Coulomb–Contensou De Saxcé Bipotential):**
   With 4D **ellipsoidal** Coulomb–Contensou contact wrench $\lambda_i = (\lambda_{n,i}, \lambda_{t1,i}, \lambda_{t2,i}, \lambda_{\mathrm{spin},i}) \in \mathcal{K}_4(\mu_i, r_{\mathrm{eff},i})$ where $\sqrt{\lambda_{t1}^2 + \lambda_{t2}^2 + (\lambda_{\mathrm{spin}}/r_{\mathrm{eff}})^2} \le \mu_i \lambda_{n,i}$ (standard ellipsoidal approximation capturing torsional twist resistance of compliant rubber feet and swerve/omni tires; `KA-04` verifies against the exact 4D ellipsoidal cone ODE rather than the elliptic-integral flat-disk quadrature!), the De Saxcé velocity modification is:
   $$v_i^{\mathrm{mod}} = v_i^+ + \begin{bmatrix} \mu_i \sqrt{(v_{t1}^+)^2 + (v_{t2}^+)^2 + (r_{\mathrm{eff}} \omega_n^+)^2} \\ 0 \\ 0 \\ 0 \end{bmatrix}$$
   Because $v^{\mathrm{mod}}$ depends on $v^+$, this is a **non-associated Nonlinear Complementarity Problem (NCP)** — NOT a single convex optimization problem (`DESIGN`, correcting v1 prose!). `frob-contact` solves it via **iterated fixed-point convex SOCPs / Projected Gauss–Seidel (PGS) / Proximal ADMM / Semismooth Newton (SSN)** with exact Fischer–Burmeister / natural-map residual $\|r_{\mathrm{NCP}}\|_\infty \le 10^{-6}$ (eval) / $10^{-4}$ (RL), zero pyramidal $\sqrt{2}\mu$ diagonal bias (`B2`!).
2. **Unified Joint-Space Nonsmooth Rows (Box-Projected Decoupled Limits, Stiction & Gear Backlash):**
   Joint hard stops ($q_j \in [q_{\min}, q_{\max}]$), dry Coulomb/Stribeck stiction ($\tau_{\mathrm{fric}} \in [-\tau_s, +\tau_s]$), and **gearbox dead-zone backlash** ($q_{\mathrm{out}} - q_{\mathrm{motor}}/N \in [-\beta_j/2, +\beta_j/2]$) couple into the same velocity-impulse step via **box-projected ABA/ADMM operator splitting** (so diagonal joint rows do NOT inflate dense contact Delassus dimensions and spill 4-contact humanoids out of SRAM!).
3. **Tactile Elastomer Patch Arrays (Dexterous Hands & Grippers):**
   Fingertip pads optionally subdivide into $2\times 2$ or $4\times 4$ compliant shear+torsion Kelvin–Voigt/Contensou micro-patches feeding `frob-sensors` tactile arrays for slip-onset detection (`NS-07`).
4. **Row-Gated 3-Tier Delassus SRAM Architecture (Respecting 4D Contensou & M5 Ultra 32 KiB Threadgroup Limit!):**
   Tiers partition strictly by **total active dense contact/Pfaffian constraint rows** $n_{\mathrm{row}} = 3 m_{3\mathrm{D}} + 4 m_{4\mathrm{D}} + n_{\mathrm{pfaff}}$ (never assuming $n_{\mathrm{row}} = 3 m_c$ under 4D Contensou!):
   - **Tier S (`n_row ≤ 30`, e.g., ≤10 3D or ≤7 4D contacts):** Packed symmetric lower-triangle FP32 Delassus matrix $W = J M^{-1} J^T + R$ takes $\frac{30\times 31}{2}\times 4 = \mathbf{1{,}860\text{ B}}$ (`1.86 kB` / `1.82 KiB`) → fits entirely in GPU registers / threadgroup SRAM!
   - **Tier M (`31 ≤ n_row ≤ 48`, e.g., ≤16 3D or ≤12 4D contacts):** Packed symmetric lower-triangle FP32 takes $\frac{48\times 49}{2}\times 4 = \mathbf{4{,}704\text{ B}}$ (`4.70 kB` / `4.59 KiB`) → comfortably inside Ada shared memory AND M5 Ultra's `32 KiB` `maxThreadgroupMemoryLength` (`W02` ragged island compaction)!
   - **Tier L (`n_row > 48`):** **Matrix-Free Delassus Operator** (`ABA`-backed $v \mapsto J M^{-1} (J^T v)$ inside preconditioned ADMM / Krylov-SSN) routed to a dedicated `W02` overflow queue — NEVER materializing dense `36 KiB` 32-contact matrices!
5. **`COD W10` Factor-Once Multi-RHS Response Batching (Replacing Killed `† K-01`):**
   Single-RHS FP16 Delassus Cholesky is **dead (`† K-01`, $\kappa = 4{,}801$)**. Instead, `frob-contact::response_batch` uses Tensor Cores (`SM_89` WMMA) / Metal `simdgroup_matrix` strictly where arithmetic intensity is genuine: **factoring one active *sticking* (bilateral equality) contact matrix $A = J_{\mathcal{A}} M^{-1} J_{\mathcal{A}}^T + D$ once (FP32) and solving $A X = B$ for $K_{\mathrm{rhs}} \in [24, 64]$ controlled impulse/wrench interventions or adjoint sensitivity columns simultaneously!** Every column is guarded by:
   - Exact unrounded residual certificate $\|\hat{x}_j - x_j^*\|_2 \le \|b_j - A\hat{x}_j\|_2 / \underline{\lambda}$ (using certified lower spectral bound $\underline{\lambda} \le \lambda_{\min}(A)$);
   - Active-mode sticking-cone slack guard **AND inactive-contact non-penetration verification ($v_{n,i}^+ \ge 0,\ \forall i \notin \mathcal{A}$)** — any column violating cone slack or penetrating an inactive contact is automatically routed to the full nonlinear contact solver!
6. **Incremental Potential Contact (IPC) Lane (`[S/F]`):**
   For thin-cable snagging, multi-finger fragile grasping, and zero-penetration Tier-B verification, `frob-contact::ipc` provides a $C^2$ log-barrier + Conservative Continuous Collision Detection (CCD) lane lifted from `fs-contact` (`GATE-CONTACT-03`; `ftx-drape` stays cleanly feature-gated to Tier-B `frob-multiphys`!).

### 8.4 Geometrically Exact Variable-Strain (GVS) Cosserat Continuum Rods (`frob-dyn`, `[F]`)
Because live `frankensim::fs-solid::rod` is a **static nodal Cosserat solver** (`"SE(3) dynamics remain successor scope"`, `F-03`), `frob-dyn` authors dynamic GVS Cosserat rods **natively in Rust** — discretizing body strain fields $\xi(s) = \Phi(s) q_{\mathrm{rod}} + \xi_0(s)$ on $SE(3)$ (Chebyshev/Legendre/piecewise-constant bases) inside the **exact same $O(N)$ Lie-group ABA/RNEA tree** as rigid limbs (`KA-05`, `GATE-COSSERAT-01`), using lifted `fs-solid::rod` strictly as the static large-deflection elastica reference oracle (`KA-05`) and static harness equilibrium geometry (`frob-mfg`)!

### 8.5 Extensive-Entropy Port-Hamiltonian Actuators & Aerial Rotor Port (`frob-actuators`, `[S]`)
Every motor, gearbox, battery pack, thermal heatsink, and aerial rotor is a typed **port-Hamiltonian subsystem** (`fs-phs`) with total storage energy $H(q, p, \phi_m, Q_{\mathrm{bat}}, S_T)$ and power-conjugated ports $(e, f)$:
- **PMSM + SVPWM Inverter + Gearbox:** Electromagnetic torque $\tau_m = K_t(T_{\mathrm{mag}}) i_q$, copper loss $P_{\mathrm{cu}} = I_{\mathrm{rms}}^2 R_0 [1 + \alpha_{\mathrm{cu}}(T_w - T_0)]$, core/iron loss $P_{\mathrm{fe}}(\omega)$, amplitude-invariant $dq$ ($i_d = 0$) **SVPWM voltage ceiling** $\frac{V_{\mathrm{bus}}(Q_{\mathrm{bat}}, I_{\mathrm{bus}})}{\sqrt{3}} \ge \sqrt{(R_s i_q + K_e \omega)^2 + (p\,\omega L_q i_q)^2}$ (with pole-pair count $p$ and electrical speed $\omega_e = p\omega$, naturally enforcing field-weakening / back-EMF torque roll-off!), and Stribeck + load-dependent gearbox efficiency $\eta(\omega, \tau)$.
- **Extensive-Entropy Thermal Node (First- & Second-Law Exact):** Tracking extensive winding entropy $S_T$ ($dU = T_w\,dS_T = C_{\mathrm{th}}\,dT_w$) gives the exact energy-conserving balance:
  $$\dot{S}_T = \frac{1}{T_w}\big(P_{\mathrm{cu}} + P_{\mathrm{fe}} + P_{\mathrm{gear}}\big) - \frac{G_{\mathrm{th}}}{T_w}(T_w - T_{\mathrm{amb}}), \qquad \sigma_{\mathrm{irr,tot}} = \frac{P_{\mathrm{cu}} + P_{\mathrm{fe}} + P_{\mathrm{gear}}}{T_w} + \frac{G_{\mathrm{th}}(T_w - T_{\mathrm{amb}})^2}{T_w T_{\mathrm{amb}}} \ge 0$$
  (total system+ambient Clausius–Duhem production $\sigma_{\mathrm{irr,tot}} \ge 0$ verified without adding spurious heat into $\dot{S}_T$!), with positive thermal feedback $a = \frac{I_{\mathrm{rms}}^2 R_0 \alpha_{\mathrm{cu}} - G_{\mathrm{th}}}{C_{\mathrm{th}}} > 0$ triggering realistic **thermal runaway cliffs** (`GB-08`, `KA-09`, `NS-04`!).
- **2-RC Thevenin Battery + SEI Aging State:** Open-circuit voltage $V_{\mathrm{oc}}(\mathrm{SoC})$, fast/slow polarization voltages $(V_1, V_2)$, temperature-dependent internal resistance $R_0(T_{\mathrm{bat}}, \mathrm{SoH})$, and low-voltage brownout cutoffs.
- **Compact Aerial-Terrestrial Rotor Port (`NONGOAL-004` Compliant):** Blade-element momentum thrust $T = c_T \rho A (\Omega r)^2 \cdot k_{\mathrm{IGE}}(z/r)$ (Cheeseman–Bennett ground effect $k_{\mathrm{IGE}} = [1 - (r/4z)^2]^{-1}$, **valid for $z \ge 0.5 r$ and capped at $4/3$ for $z < 0.5 r$**!), reaction drag torque $\tau_z = c_Q \rho A r (\Omega r)^2$, induced-power electrical load, and quadratic fuselage drag — enabling hybrid wheeled/legged-aerial archetypes without Navier–Stokes CFD!

---

## 9. Morphological Symmetry, $\mathbb{R}/\mathbb{C}/\mathbb{H}$ Irreps, Wreath Products, H/K & Burnside (`frob-symmetry`, `[F]`)

### 9.1 Mechanical Symmetry $\Gamma_{\mathrm{mech}}$ vs Closed-Loop Observation/Task Symmetry $\Gamma_{\mathrm{obs}}$
A critical lesson from our audits (`C4`, v1 Appendix D fix): a quadruped with symmetric legs ($V_4$ mechanical symmetry) carrying a **front-only LiDAR and RGB-D camera** does **NOT** have $V_4$ closed-loop symmetry — front-back reflection changes observations! `frob-symmetry` therefore computes and stores **two explicitly nested finite groups** on every `.fbot` + `.ftask` pair:
1. **$\Gamma_{\mathrm{mech}} = \mathrm{Aut}_{\mathrm{mech}}(\mathcal{G})$:** Spatial graph automorphisms preserving link lengths, $\mathrm{Sym}^+(4)$ pseudo-inertias under $J' = g_v J g_v^T$, joint screws under the full affine reflection adjoint $\mathrm{Ad}_{g_v}$, actuator specs, and gear ratios. Used for dynamics caching, inertial reductions, and static equilibrium analysis.
2. **$\Gamma_{\mathrm{obs}} \le \Gamma_{\mathrm{mech}}$:** The maximal subgroup additionally preserving **every sensor pose/FOV, asymmetric payload, task goal orientation, and guard/terrain symmetry**. **ONLY $\Gamma_{\mathrm{obs}}$ is admitted** for equivariant RL weight sharing (`frob-equiv`), Burnside closed-loop deduplication, and H/K gait orbits! Violating $\Gamma_{\mathrm{obs}} \le \Gamma_{\mathrm{mech}}$ is rejected by `PO-EQUIV-1`!

### 9.2 Frobenius–Schur ($\mathbb{R}/\mathbb{C}/\mathbb{H}$) Exact Decomposition & Regular-Rep Layers
Real representations of finite groups (especially cyclic $C_n$ spiders with $n \ge 3$ and rotational turret groups!) split into **three Frobenius–Schur indicator types** ($\nu_\chi \in \{+1\ (\mathbb{R}),\; 0\ (\mathbb{C}),\; -1\ (\mathbb{H})\}$). Naive real Reynolds averaging $P = \frac{1}{|\Gamma|}\sum_g \rho_{\mathrm{out}}(g) \otimes \rho_{\mathrm{in}}(g)^{-T}$ blows up as $O(d_{\mathrm{in}}^3 d_{\mathrm{out}}^3)$ AND mishandles 2D real realizations of complex-conjugate pairs!
- `frob-symmetry` computes **real per-space $O(d^3)$ isotypic projectors** independently on input and output spaces (using `fs-symmetry` + exact cyclotomic arithmetic in $\mathbb{Q}(\cos(2\pi/n), \sin(2\pi/n)) \subseteq \mathbb{Q}(\zeta_n)$ for $n \le 12$, explicitly covering $C_5/D_5$ $\mathbb{Q}(\sqrt{5})$ as well as $\mathbb{Q}(\sqrt{2}), \mathbb{Q}(\sqrt{3})$!):
  - **Real ($\mathbb{R}$, $\nu=+1$):** $P_\chi = \frac{d_\chi}{|\Gamma|}\sum_{g\in\Gamma} \chi(g)\rho(g)$ ($1\times 1$ scalar intertwining weights per multiplicity pair);
  - **Complex ($\mathbb{C}$, $\nu=0$):** Conjugate-paired real projector **$P_{\chi\oplus\overline{\chi}} = \frac{2d_\chi}{|\Gamma|}\sum_{g\in\Gamma}\mathrm{Re}(\chi(g))\rho(g)$** plus skew-symmetric imaginary operator **$Q_\chi = \frac{2d_\chi}{|\Gamma|}\sum_{g\in\Gamma}\mathrm{Im}(\chi(g))\rho(g)$** yielding exact $2\times 2$ real rotation-scaling blocks $\begin{bmatrix}a & -b \\ b & a\end{bmatrix}$ (2 free params per irrep pair);
  - **Quaternionic ($\mathbb{H}$, $\nu=-1$):** $4\times 4$ real quaternion blocks (4 free params per irrep pair).
- **Hidden-Layer Regular Representations $\mathbb{R}[\Gamma]^k$:** Intermediate RL policy layers use the **regular representation** (`frob-equiv`), where pointwise nonlinearities (`SiLU`, `LayerNorm` per orbit) commute *identically* with $\Gamma_{\mathrm{obs}}$ without restricting expressivity or requiring expensive Clebsch–Gordan tensor products (`GATE-EQUIV-01`)!

### 9.3 Wreath Products ($\mathrm{Aut}(\text{comp}) \wr_X H$) & Exact Burnside Deduplication (`CC-07`, Fixed `C4/C5`)
- **Wreath-Product Equivariance for Heterogeneous-Limb Robots:** A quadruped carrying a symmetric dual-arm torso, or a 6-legged spider where each leg carries symmetric dual fingers, has symmetry group $\Gamma = K \wr_X H = K^{|X|} \rtimes H$ **strictly when** (a) every component automorphism in $K$ preserves its exposed attachment port invariants, and (b) the hub group $H$ preserves the decorated port interfaces (`C5`). `frob-symmetry::wreath` checks both preconditions symbolically before admitting nested parameter tying.
- **Exact Burnside / Pólya Campaign Deduplication:** Before spawning a combinatorial morphology sweep over $n$ slots with $k$ achiral (self-mirror-symmetric) module types under closed-loop group $\Gamma_{\mathrm{obs}}$, `frob-symmetry::burnside` computes the exact cycle index $Z(\Gamma_{\mathrm{obs}})$ and generates **exact canonical orbit representatives** (one simulation per equivalence class, zero duplicates!). For example, placing $k=10$ achiral leg modules onto $6$ dihedral ($D_6$) slots yields:
  $$|X / D_6| = \frac{10^6 + 2\cdot 10 + 2\cdot 10^2 + 10^3 + 3\cdot 10^4 + 3\cdot 10^3}{12} = \mathbf{86{,}185} \quad (\mathbf{11.60\times}\text{ exact dedup!})$$
  whereas if asymmetric front sensors restrict the closed loop to sagittal mirror symmetry $\mathbb{Z}_2$ (3 transpositions), the exact canonical count is $\frac{10^6 + 10^3}{2} = \mathbf{500{,}500}$ (**`2.00×`**) — honestly reported upfront (`C4`, verified in [planning/verify_plan_math.py](file:///Users/jemanuel/projects/franken_robotics/planning/verify_plan_math.py))!

---

## 10. Decorated-Cospan Operadic Algebra (`.fmodule`) & Mission Quotients (`.fquot`, `frob-operad`, `[F]`)

### 10.1 Decorated-Cospan `.fmodule` Substitution Contracts (`FMT-FMOD-1`, `COD W01` + `CC-07` + `AGY #1`)
Every reusable robot subassembly (chassis hub, 3-DoF leg, wheel-leg, parallel ankle, dexterous arm, battery/bus bay, sensor head, rotor pod) is an **open port-Hamiltonian module** serialized as **`FMT-FMOD-1`** (worked example in Appendix D.1b) and formalized as a **decorated cospan** $X \xrightarrow{i} N \xleftarrow{o} Y$ over finite typed port sets:

```ebnf
ModuleDoc  ::= "(module" String ":schema \"frob.module/v1\""
                 "(ports" PortDecl+ ")"
                 "(params" ParamDecl* ")"
                 "(body" (LinkForm | JointForm | LoopConstraint)+ ")"
                 "(contract :power-sign \"into-module\" :port-abi \"SymPsd6\""
                 ":digest" String ":proofs" "(" Ident+ ")" ")" ")"
PortDecl   ::= "(port" Ident ":kind" ("MechanicalSe3"|"ElectricalDc"|"ThermalEntropic"|"LogicalBus")
                 ":frame" Ident? ":dir" ("in"|"out"|"bidir") ")"
ParamDecl  ::= "(param" Ident ":type" ("ActuatorId"|"MaterialId"|"ContactLaw"|"Qty") ":default" Value ")"
```

- **Typed Ports:** `MechanicalSe3` (effort $w \in \mathfrak{se}(3)^*$, flow $\xi \in \mathfrak{se}(3)$, strict **into-module positive power** sign $\langle w, \xi \rangle > 0$), `ElectricalDc` $(V, I)$, `ThermalEntropic` $(T, \dot{S})$, `LogicalBus` (CAN/EtherCAT frame rate & latency bound).
- **Composition via Pushout & Dirac Interconnection:** Gluing module $M_1$ to $M_2$ along compatible shared ports $P$ computes the cospan pushout (`fs-phs::interconnect_mapped`), enforcing exact effort/flow power cancellation $\sum_{p \in P} (e_p f_p) = 0$ and **rejecting algebraic loops** (`FROB-REF-108`) statically.
- **Incremental Proof & Invalidation Contracts (Honest Coupling Semantics, `C5`):**
  Each `.fmodule` carries a content-addressed **Summary Digest** hashing its port signature, local $\mathrm{Sym}^+(4)$ proofs, local passivity certificate, and Featherstone Schur-complement boundary descriptor.
  - **Local Leaf Substitution:** Swapping a limb module preserves all untouched sibling modules' proofs, symbolic derivations, and local kernels (`≥10×` edit-derivation speed on hot leaves!).
  - **Honest Global Invalidation (`C5`):** Edits touching a shared `ElectricalDc` battery bus, shared thermal chassis sink, cross-branch closed kinematic loop (sub-PROP escalation), or **base-coupled Delassus contact leaves** explicitly invalidate dependent global certificates — verified continuously by COD's mandatory mirror-swap vs shared-bus invalidation controls (`GATE-OPERAD-01`)!
- **Featherstone Schur-Complement Limb Port-ABI ("Tier 0.5", `AGY #1`, Corrected PSD Fact):**
  During unconstrained forward dynamics, an arbitrary tree limb attached to a base port contracts exactly to its articulated spatial inertia & bias wrench $(I^A \in \mathrm{Sym}_{\succeq 0}(6),\, p^A \in \mathbb{R}^6)$ (`27` floats = `108 B` FP32, replacing MOR's early 30-float `120 B` sketch). Crucially (per Codex's proof in `DR-01`), after eliminating a free joint axis $s$, the ABA Schur update $I^A - I^A s (s^T I^A s)^{-1} s^T I^A$ is **positive semidefinite (PSD)** — not strictly SPD (e.g., $\mathrm{diag}(0,1,1,1,1,1)$ along $e_1$) — so `frob-operad` types the port inertia accurately as `SymPsd6` and explicitly routes multi-limb ground contacts through base-coupled constrained ABA / Delassus solves rather than pretending the 27-float packet decouples contacts!

### 10.2 Mission-Observable Design Quotients (`FMT-FQUOT-1`, `COD FTA-P1` + `CC TA-P1`)
Before running a design sweep over candidates $\mathcal{D}$, `frob-operad::quotient` compiles the **mission-observable projection map** $\pi_{\mathcal{M}} : \mathcal{D} \to \mathcal{O}_{\mathcal{M}}$ extracting ONLY the exact invariants that enter the mission's equations of motion, collision geometry, sensor visibility, thermal/electrical bus, and STL objective (e.g., total rigid-body pseudo-inertia $J_{\mathrm{tot}} \in \mathrm{Sym}^+(4)$, external collision hull, joint/actuator parameters, exposed port coordinates).
- **Soundness Rule:** Two designs $d_1, d_2 \in \mathcal{D}$ collapse into one equivalence class $[d]$ **if and only if** $\pi_{\mathcal{M}}(d_1) = \pi_{\mathcal{M}}(d_2)$ is proved algebraically across ALL active observables of mission $\mathcal{M}$! Adding a placement-dependent thermal hotspot or cable-harness routing length automatically enlarges $\pi_{\mathcal{M}}$ and splits previously equivalent classes (`COD FTA-P1` potency control!).
- **Pinned Potency Fixture (`XT-P1`, Python-verified):** Placing 8 equal ballast slugs into 24 collinear internal chassis slots yields $\binom{24}{8} = \mathbf{735{,}471}$ configurations, which depend on internal rigid-body dynamics strictly through $(N, \sum x_i, \sum x_i^2)$ — collapsing **provably** to exactly **`24,546` distinct quotient classes (`29.96×` exact experiment deletion!)**, while two configurations sharing $(\text{mass}, \sum x_i)$ but differing in $\sum x_i^2$ MUST split!

---

## 11. Exact Linkage Synthesis (`W06`) & GPU Polyhedral/Parameter Homotopy (`CC-04`)

Parallel legs (4-bar/5-bar cycloidal legs, 2-DoF differential parallel humanoid ankles, Stewart-platform wrists/waists) break naive random URDF mutation because >99.9% of random parameter tuples violate algebraic loop closure. `frob-symbolic` and `frob-homotopy` solve inverse mechanism synthesis in **two cleanly phased, live-code-grounded lanes**:

### 11.1 M1 Exact Rational & Bounded `real_rur` Lane (`frob-symbolic::linkage_synth`, `COD W06`, `[S/F]`)
Executes directly on live `frankensympy @ 29f847c1` (`F-05`):
1. **Closed-Form Rational 3-Pose Planar Dyads:** Solves exact $2\times 2$ linear circumcenter systems over $\mathbb{Q}$, partitioning degenerate configurations strictly by determinant rank (zero silent divide-by-zero!).
2. **Bounded 0-D `real_rur` Templates (5-Pose Burmester & Planar 4/5-Bar):** Formulates Study-quadric / loop-closure polynomials strictly inside `fsym-solvers::real_rur`'s verified live limits (`MAX_VARIABLES ≤ 8`, `MAX_EQUATIONS ≤ 64`, `MAX_DEGREE ≤ 256`, **`MAX_REAL_ROOT_DEGREE ≤ 64` eliminant cap**, `MAX_TERMS ≤ 4096`, `MAX_BITS ≤ 4096`, `MAX_COMPUTE ≤ 200M`, `MAX_MEMORY ≤ 512 MiB`), applying Rabinowitsch denominator saturation ($s \cdot d(x) - 1 = 0$) to exclude degenerate zero-length / collinear roots.
3. **Continuous Branch, Singularity & Clearance Certificate:** Isolates real roots via exact interval contraction (`fs-ivl`) and certifies that a single assembly circuit traverses ALL target poses without crossing a toggle singularity ($\det J_{\mathrm{loop}}(q) \neq 0$) or link self-collision (`GATE-BURMESTER-01`).

### 11.2 M3 Polyhedral & Parameter Homotopy Continuation Lane (`K1::frob-homotopy` + `E0::frob-gpu`, `CC-04`, `[F]`)
To transcend `real_rur`'s 8-variable / eliminant-degree-64 ceiling (e.g., **9-point four-bar path synthesis with generic complex count $N_{\mathrm{gen}} = 4{,}326 = 1{,}442\text{ coupler curves} \times 3$ Roberts–Chebyshev cognates**, **general Stewart–Gough $N_{\mathrm{gen}} = 40$ FK roots**, **spatial 6R $N_{\mathrm{gen}} = 16$ IK roots**, and parallel humanoid ankle-knee couplings):
1. **Pure Algebraic Core (`K1::frob-homotopy`, Zero GPU Import!):** Computes polyhedral/BKK mixed-volume start systems $F(z; p_0) = 0$ once per template, emits a specialized `FMT-KIR-1` parameter-homotopy predictor–corrector kernel, and runs CPU exact Smale $\alpha$-certification + trace-test verification.
2. **GPU Parameter Homotopy Tracking (`E0::frob-gpu`, `≤2 s` Re-Query Target):** Executes the lowered `FMT-KIR-1` tracker in parallel across Metal/CUDA warps for $H(z, t) = F(z; (1-t)p_0 + t p_1) = 0$ using adaptive RK4/Hermite predictor–corrector + Cauchy endgames for singular endpoints (with checked `f32-dd` / CPU arbitrary-precision recovery on ill-conditioned paths!).
3. **Smale $\alpha$-Certification (`alphaCertified` Port, `R1` — Corrected Reality Theorem!):** Runs exact rational/interval Smale $\alpha$-theory ($\alpha(f, z) < \alpha_0 = \frac{13 - 3\sqrt{17}}{4} \approx 0.15767$) in Rust + `fln-kernel` reflection (`PO-ALPHA-1`) to certify:
   - **Existence & Uniqueness** of a unique nonsingular complex zero $\zeta \in \mathbb{C}^n$ inside $B(z, 2\beta(f, z))$;
   - **Distinctness** of every pair of certified roots via disjoint certified neighborhoods $B(z_i, 2\beta_i) \cap B(z_j, 2\beta_j) = \emptyset$;
   - **Reality vs. Non-Reality (Exact `alphaCertified` Test):** **Reality** ($\zeta \in \mathbb{R}^n$) is certified by $\alpha(f, \mathrm{Re}(z)) < \alpha_0$ (since Newton iterates from $\mathrm{Re}(z) \in \mathbb{R}^n$ remain real and converge to the unique associated zero $\zeta \in \mathbb{R}^n$!), while **complex-conjugate ball disjointness** $B(z, 2\beta) \cap B(\overline{z}, 2\beta) = \emptyset$ certifies **non-reality** (soundly pruning complex conjugate pairs from the real linkage set)!
4. **Honest Completeness Classification (`C04` Concession):** Emits **`Complete`** *if and only if* the certified isolated **complex** root count (real + certified complex-conjugate pairs) equals the independent trace-test / characteristic-zero generic count $N_{\mathrm{gen}}$ with zero unexplained lost paths or positive-dimensional components; otherwise emits **`Partial(certified_n, lost_n)`** (`GATE-HOMOTOPY-01`, `KILL-HOMOTOPY-1`; all-real saturation tested specifically on Dietmaier's 40-real Stewart and Manseur–Doty's 16-real 6R fixtures)!

---

## 12. Symbolic Compiler, Formal `FMT-KIR-1` Grammar & 4-Target Lowering (`frob-symbolic`, `frob-kernel-ir`, `frob-codegen`)

### 12.1 Symbolic Robotics Algebra (`frob-symbolic`, `[S]`)
Consuming `fsym-core`, `fsym-calculus`, and `fsym-matrices` (with `SubprocessSymbolicBackend` isolated strictly in Layer `V::frob-io` to preserve pure-kernel layering under `KILL-FSYM-1`!), `frob-symbolic` derives:
- **Product-of-Exponentials (PoE) FK, Geometric Jacobians, RNEA, CRBA, and ABA** in body coordinates;
- **Echeandia–Wensing Universally Skew-Symmetric Coriolis Matrix $C(q, \dot{q})$** guaranteeing $\dot{M}(q) - 2C(q, \dot{q}) = -(\dot{M}(q) - 2C(q, \dot{q}))^T$ identically as a polynomial matrix identity (`PO-SKEW-1`, crucial for passivity-based IDA-PBC control!);
- **Exact Symbolic Pullback & Capability Maps** $w = w_0(q, \dot{q}) + B_m(q) u$ for `.fskill` transfer (`W03`).

### 12.2 Formal `FMT-KIR-1` Typed SSA Kernel IR (`frob-kernel-ir`, `[S/F]`)
All compiled dynamics, actuator pHS steps, STL robustness monitors, safety shields, and `#![no_std]` controllers lower through **one canonical, statically bounded, zero-allocation SSA IR (`FMT-KIR-1`)**:

```ebnf
KernelModule ::= "kir_v1" Ident "(" ParamList ")" "->" "(" OutputList ")" Metadata Block
ParamList    ::= (Ident ":" Type ("," Ident ":" Type)*)?
OutputList   ::= (Ident ":" Type ("," Ident ":" Type)*)?
Block        ::= "{" Stmt* ReturnStmt "}"
ReturnStmt   ::= "return" "(" (ValId ("," ValId)*)? ")" ";"
Scalar       ::= "f32" | "f64" | "f32dd" | "q16_16" | "q32_32" | "i32" | "u32" | "bool"
Type         ::= Scalar | "vec3<" Scalar ">" | "mat3<" Scalar ">" | "se3<" Scalar ">"
               | "sym6<" Scalar ">" | "array<" Type "," ConstUsize ">"
Stmt         ::= ValId ":" Type "=" Expr ";"
               | "store_soa" ArenaSlot "[" LaneExpr "," ConstOffset "]" ValId ";"
               | "for" Ident "in" ConstUsize ".." ConstUsize "{" Stmt* "}"   (* statically bounded only! *)
               | "if_pred" ValId "{" Stmt* "}" "else" "{" Stmt* "}"         (* pure branchless select *)
Expr         ::= ("add"|"sub"|"mul"|"div"|"min"|"max") "(" ValId "," ValId ")"
               | "fma" "(" ValId "," ValId "," ValId ")"
               | ("neg"|"sqrt"|"rsqrt"|"sinc"|"cosc"|"sin"|"cos"|"abs") "(" ValId ")"
               | "load_soa" ArenaSlot "[" LaneExpr "," ConstOffset "]"
               | "clip" "(" ValId "," ValId "," ValId ")" | ValId | Literal
Metadata     ::= "{" "hash:" Blake3Hex "," "math_mode:" ("IEEE754Safe"|"FastApprox"|"FixedPoint")
                     "," "max_regs:" U32 "," "max_stack_bytes:" U32 "," "proof_digest:" ("none"|Blake3Hex) "}"
(* Terminals: Ident = [a-zA-Z_][a-zA-Z0-9_]*, ValId = "%" Ident, ArenaSlot = Ident,
   LaneExpr = ValId | Ident | ConstUsize, ConstUsize/ConstOffset/U32 = [0-9]+, Blake3Hex = '"' [0-9a-f]{64} '"' *)
```

- **Optimizations:** Bounded algebraic CSE + **Horner/FMA fusion** + live-range register-pressure scheduling (capping Ada registers to `--maxrregcount=128` for occupancy!).
- **4-Target Pure Codegen (`frob-codegen`, `#![forbid(unsafe_code)]`, strictly emits congruence traces without importing `frob-proof`!):**
  1. **Safe Rust CPU (`f64` oracle & SIMD `f32`)** — reference twin for `GATE-GPU-TWIN-01`.
  2. **Metal Shading Language (MSL 3.1/3.2)** — `MathMode::Safe` mandatory for `f32dd` (`TwoSum` preserved, `F-01`); dense `simdgroup_matrix` GEMM kernels live alongside in hand-tuned `frob-gpu-metal` modules.
  3. **CUDA PTX / CUDA C (`SM_89`)** — `--fmad=true` (fast) vs IEEE-strict (`--fmad=false` for `f32dd` audit), branch-free `select`; dense Ada WMMA GEMM kernels live in hand-tuned `frob-gpu-cuda` modules.
  4. **Zero-Alloc `#![no_std]` Embedded Rust Firmware** — exact fixed-point (`Q16.16`/`Q32.32`) or IEEE-754 `f32` controller + `.fenergy`/shield kernel compiling cleanly for `thumbv7em-none-eabihf` (Cortex-M4/M7) and `riscv32imac`, with static stack-depth annotation and `MotorBusHal` trait isolation (`hardware_safety_signoff: false`)!

---

## 13. Proved `fln-kernel` Computational Reflection (`K1::frob-proof`) & Stratified Safety (`E2::frob-shield`)

### 13.1 Direct `fln-kernel` Computational Reflection Architecture (Honoring Live Fact `F-04`!)
Because live `franken_lean @ 4426f8ea7` ships a real CIC typechecker (`fln-kernel`, `11,101` prod LOC ≤12 KLOC TCB) and `fln-checker` (`38.0 kLOC` prod crate / `53.9 kLOC` prod closure), while `fln-anvil` (`3,675` LOC / `2,089` prod across 8 files) implements **ONLY `DTree` and `omega`** (`OmegaCertificate`; `egraph`/`norm_num`/`simp`/`grind` are stubs!) and `fsym-formal` supports **ONLY 2 univariate `ZZ` profiles** (`F-05`):
- `frob-proof` NEVER calls unimplemented `fln-anvil` tactics!
- Instead, `frob-proof` (`K1`) ships a small, self-contained, formally proved **`FrobMath` reflection library** inside `franken_lean`'s environment (proved once at M1 and checked offline by `fln-checker`) providing sound kernel-reducible boolean decision procedures (`check_*: Input -> Bool` with soundness theorems `∀ x, check_*(x) = true → Spec(x)`):
  1. **Pivoted Integer-Scaled Exact Rational $P^T Q P = L D L^T$ ($D \ge 0$) Reflector:** Certifies $J \succ 0$ (`Sym^+(4)`), strictly PD or boundary PSD ($Q \succeq 0$, pivoted with zero-column check!) sparse rational SOS Gram matrices $P = z^T Q z$, and interval-certified positive definiteness!
  2. **Universal Algebraic Identity & Quotient-Ring Polynomial Reflector:** Verifies Echeandia–Wensing $\dot{M} - 2C$ skew-symmetry (`PO-SKEW-1`, reducing trigonometric polynomials modulo the Pythagorean ideal $\langle c_i^2 + s_i^2 - 1 \rangle$ or via rational half-angle substitution $t_i = \tan(q_i/2)$!), operadic power-port conservation (`PO-OPERAD-1`), and cyclotomic $\mathbb{Q}(\zeta_n)$ symmetry intertwining (`PO-EQUIV-1`) by normalized sparse multivariate polynomial subtraction!
  3. **Linear-Size SSA DAG Congruence Trace Checker (`PO-CODEGEN-1` & `PO-CODEGEN-FW-1`):** `frob-codegen` purely emits a linear-size rewrite trace (associativity, commutativity, distributivity, CSE let-binding steps) over exact real/rational `FMT-KIR-1` ASTs, verified one-way inside `frob-proof` via `fln-kernel` in $O(|G|)$ kernel reductions (explicitly distinguishing algebraic equivalence over $\mathbb{R}$ from bounded FP32/Q32.32 rounding error tracked via `fs-ivl`!).
  4. **Finite-Group `decide` & Fixed-Point Interval Reflectors:** Verifies H/K subgroup gait lattices (`PO-GAITENUM-1`), `.fenergy` fixed-point energy inequalities (`PO-ENERGY-1`), `.ftube` Minkowski-eroded cell-transition inclusion (`PO-TUBE-1`), Smale $\alpha$-certification (`PO-ALPHA-1`), and Observation-Alias disjoint outer action boxes (`PO-OBSALIAS-1`) + `fln-anvil::omega` (`OmegaCertificate`)!

### 13.2 Complete Proof Obligation Registry (`PO-*`, `DESIGN`)

| PO ID | Formal Property Certified (`VerifiedFormal` Under Declared Model $M$ & Assumptions $A$) | Reflection / Checker Engine | Budget Target | Milestone |
|---|---|---|---|---|
| **`PO-LIE-1`** | Normative §6.0 Lie/PGA identities ($\mathrm{Ad}$, $\mathrm{ad}^*$, $D\mathbf{A}_S$ minus sign, affine $O(3)$ reflection parity) | `FrobMath` poly reflector | `<0.2 s` | M1 |
| **`PO-INERTIA-1`** | Every body pseudo-inertia $J \in \mathrm{Sym}^+(4)$ (implies $m>0$, $I\succ 0$, triangle inequalities) | Pivoted exact $LDL^T$ reflector | `<0.05 s` | M1 |
| **`PO-SKEW-1`** | Echeandia–Wensing Coriolis satisfies $\dot{M} - 2C + (\dot{M} - 2C)^T = 0 \pmod{\langle c_i^2+s_i^2-1\rangle}$ | Quotient-ring $\mathbb{Z}$-poly reflector | `<0.8 s` | M1 |
| **`PO-EQUIV-1`** | $\Gamma_{\mathrm{obs}} \le \Gamma_{\mathrm{mech}}$ commutes with dynamics, sensors, task & wreath port invariants | Cyclotomic $\mathbb{Q}(\zeta_n)$ ($\sqrt{2},\sqrt{3},\sqrt{5}$) reflector | `<0.5 s` | M1 / M4 |
| **`PO-OPERAD-1`** | Decorated-cospan `.fmodule` wiring preserves port power conservation $\sum e_p f_p = 0$ & DAG acyclicity | `FrobMath` cospan + `omega` | `<0.2 s` | M1 |
| **`PO-QUOT-1`** | `.fquot` mission-observable projection $\pi_{\mathcal{M}}(d_1)=\pi_{\mathcal{M}}(d_2)$ implies identical simulated trajectories | Algebraic moment/invariant reflector | `<0.2 s` | M1 |
| **`PO-CODEGEN-1`** | `FMT-KIR-1` CSE/lowering is algebraically equivalent over $\mathbb{R}$ (paired with `fs-ivl` FP rounding bound) | Linear SSA congruence reflector | `<1.5 s` | M1 / M2 |
| **`PO-CODEGEN-FW-1`** | `#![no_std]` fixed-point `Q32.32`/IEEE firmware lowering matches `FMT-KIR-1` + no-overflow bounds | SSA reflector + `omega` | `<2.0 s` | M4 |
| **`PO-ALPHA-1`** | Smale $\alpha(f, z) < \alpha_0$, disjoint balls, real certification $\alpha(f, \mathrm{Re}(z)) < \alpha_0$ & conjugate non-reality | Exact rational interval reflector | `<0.5 s` | M3 |
| **`PO-GAITENUM-1`** | Complete H/K spatiotemporal symmetry lattice of $\Gamma_{\mathrm{obs}}$ for catalog morphology | Finite-group `decide` in `fln-kernel` | `<0.3 s` | M4 |
| **`PO-ENERGY-1`** | **`.fenergy` Boundary Energy Budget (`FMT-FENERGY-1`):** $\frac{1}{2}mv^2 + \int P_{\mathrm{elec}}^+ dt + E_{\mathrm{ext}} + E_{\mathrm{stored}} + E_{\mathrm{res}} \le W_{\mathrm{abs,min}}$ | Fixed-point `omega` + interval reflector | `<0.05 s` | M2 / M4 |
| **`PO-TUBE-1`** | **`.ftube` Hybrid Reachable-Tube Inclusion (`FMT-FTUBE-1`):** $R_{mn}((K_m \oplus E)\cap G_{mn}, D) \oplus (-E) \subseteq K_n$ (Minkowski-eroded!) | Bitset cell-transition reflector | `<1.5 s` | M4 |
| **`PO-SHIELD-1`** | **Reduced Centroidal Box-Constrained CBF Compatibility:** $\exists u \in U_{\mathrm{act}},\ \forall(w,e)\in\mathcal{U},\ \dot{h} + \alpha(h) \ge L_h\Delta t$ (per stratum) | Pivoted sparse $\mathbb{Z}$-Gram + Farkas reflector | `<4.0 s`/stratum | M4 |
| **`PO-OBSALIAS-1`** | **Observation-Alias Impossibility Witness:** Two admitted hidden worlds share transcript with disjoint outer safe action sets | `fs-ivl` outer box + `omega` disjointness | `<0.5 s` | M3 |

### 13.3 Three-Tier Stratified Safety Architecture (`E2::frob-shield`, `[F]`, Zero Claim Laundering!)
Pure certificate predicates (`PO-ENERGY-1`, `PO-TUBE-1`, `PO-SHIELD-1`) live in `K1::frob-proof`, while the runtime shields (`frob-shield`) reside in Layer **`E2`** (peer of `frob-control`, cleanly consuming `E1::frob-estimate` innovations & `E2::frob-control` passive braking backups with zero upward layer violations!). `frob-shield` composes **three mathematically distinct safety tiers** (`OC-03`):

#### Tier 1 — Observable Boundary Energy Budgets (`FMT-FENERGY-1`, `COD FTA-P3`, `PO-ENERGY-1`)
For every guided stroke, stop, bumper collision, drawer/door actuation, lift platform, or ISO/TS 15066 transient human-robot contact boundary:
$$E_{\mathrm{deliv}}(t) := \frac{1}{2} m v(t)^2 + \int_{t}^{t + \Delta t_{\mathrm{cut}}} \max\big(P_{\mathrm{elec}}(\tau),\, 0\big)\,d\tau + E_{\mathrm{ext}}(t) + E_{\mathrm{stored}}(t) + E_{\mathrm{reserve}} \;\le\; W_{\mathrm{abs,min}}$$
where $E_{\mathrm{ext}}(t)$ upper-bounds external/gravitational work over the braking stroke, $E_{\mathrm{stored}}(t)$ bounds elastic/potential energy, and $E_{\mathrm{reserve}}$ adds an explicit sensor/numeric safety buffer!
- **Why It Is Immune to Contact Chaos:** Passive impacts, bounces, and arbitrary Coulomb/Contensou friction coefficients $\mu \ge 0$ can ONLY dissipate or redistribute mechanical energy — NEVER create it! Contact dynamics drops out by conservation of energy!
- **Hardware-Auditable Calibration:** One drop-weight bench test of the compliant absorber (yielding certified lower envelope $W_{\mathrm{abs,min}}$) + one drive power-meter test earns **`ValidatedPhysics`** across **every** subsequent policy revision (`100` retrainings = zero re-certification cost!), with a closed-form cutoff time (e.g., `50.5 ms` nominal equality cutoff at `1.49 J ≤ 2.00 J`, or `45.5 ms` with an additional `0.10 J` numeric reserve — verified in `GB-07`!) executable in `<5 µs` on an MCU or GPU!

#### Tier 2 — Hybrid Reachable-Tube Composition (`FMT-FTUBE-1`, `COD W04`, `PO-TUBE-1`)
For **low-dimensional ($\le 4\text{D}$ reduced state + bounded sensing error $E$) nonsmooth guard-ambiguous subsystems** (wheeled threshold traversal, tray anti-spill tipping, emergency braking on mixed tiles, door-pinch zones):
- Backward viability pruning over cell complexes (`≤8 modes × 16,384 cells = 16 MiB`) certifies a common safe action $u \in U_{\mathrm{act}}$ satisfying the **inductively closed Minkowski-eroded inclusion** **$R_{mn}\big((K_m \oplus E) \cap G_{mn},\, D\big) \oplus (-E) \subseteq K_n$** (i.e., $\subseteq K_n \ominus E$ for measured-state cells, or $R_{mn}(K_m \cap G_{mn}, D) \subseteq K_n \ominus (E \ominus E)$ for true-state belief-checked cells!) across ALL simultaneously enabled guard successors, explicitly accounting for inter-sample sweep, sensing/actuation latency, chatter exclusion, and a verified backup domain!

#### Tier 3 — Box-Constrained Compatibility-Certified Fused Shields + `ClaimProduct` (`CC-06`/`TA-P3` Repaired!)
For **high-DoF articulated locomotion & manipulation** (quadrupeds, hexapods, 29-DoF humanoids) during GPU RL exploration and runtime monitoring:
1. **Corrected Quantifiers & Split Friction Cones (`DR-05`, `C1`):**
   Every barrier constraint enforces the **non-clairvoyant** quantifier order **`∃u ∈ U_act, ∀(w, e) ∈ U`**. Contact forces split cleanly into $\lambda = (\lambda_{\mathrm{ctrl}}, \lambda_{\mathrm{env}})$:
   - Robot-commanded reaction forces $\lambda_{\mathrm{ctrl}}$ are constrained to the **conservative inner cone $\mathcal{K}(\mu_{\min})$** (always physically attainable!);
   - Adversarial environment disturbances $\lambda_{\mathrm{env}}$ inside the worst-case infimum range over the **outer cone $\mathcal{K}(\mu_{\max}) = \bigcup_{\mu \le \mu_{\max}} \mathcal{K}(\mu)$** (explicitly added to the monitored assumption set $A_\sigma$!).
2. **Exact Box-Constrained GPU Projection (`C2` Fix — Zero Box Violations!):**
   Given nominal RL action $u_\pi$, actuator box $u \in [\ell, \bar{u}]$, and $m$ active linear-in-$u$ barrier constraints $A u \ge b + L_h\Delta t$ (where $L_h := \sup\big|\frac{d}{dt}(\dot{h}+\alpha(h))\big|$ is the time-derivative Lipschitz inter-sample margin, plus actuator-tracking error margin!):
   - **$m = 1$, diagonal metric $R$:** Exact piecewise-linear sort-and-clip solve of $u(\lambda) = \mathrm{clip}(u_\pi + \lambda R^{-1} a,\, \ell, \bar{u})$ over its $\le 2n$ sorted breakpoints (on Codex's counterexample $R=I, u_\pi=(1,0), [0,1]^2, u_1+u_2 \ge 1.5$, yields exact $\lambda=0.5 \implies (1, 0.5)$ — zero box violation!); if the box is infeasible ($\max_{u\in[\ell,\bar{u}]} a^T u < b + L_h\Delta t$), immediately invokes the certified backup controller (`FROB-REF-207`)!
   - **$m \ge 2$ (capped at $m \le 4$ coupled centroidal/contact rows + orthogonal per-joint boxes on GPU):** Fixed-iteration box-constrained projected Newton solver + **mandatory post-hoc KKT certificate check** (primal feasibility, dual non-negativity, complementarity, stationarity $\le \varepsilon_{\mathrm{fp}}$); if ANY KKT check fails or constraints conflict, the step atomically invokes the **certified passive/braking backup controller** from `frob-control`!
3. **Exact Contact Strata (`27` $V_4$ / `45` $\mathbb{Z}_2$) & Properly Typed `ClaimProduct` Bound (`C15`, `C16`):**
   - For a 4-legged quadruped with per-foot states $\{\text{open}, \text{stick}, \text{slip}\}$: under $V_4$-invariant mechanical barriers, Burnside gives **exactly $\frac{3^4 + 3\cdot 3^2}{4} = \mathbf{27}$ canonical contact strata and `162` directed one-foot transition lemmas** (~10–13 min offline reduced-centroidal SOS synthesis on M5 Ultra; coarse binary open/stance model has `7` strata and `16` transitions); for front-sensor $\Gamma_{\mathrm{obs}} = \mathbb{Z}_2$ observation-coupled barriers, Burnside gives **`45` strata and `324` transitions** (`10`/`32` binary).
   - Runtime assumption monitors (`fs-eproc` / `fs-conform`) track assumption breaches (friction outside $[\mu_{\min}, \mu_{\max}]$, InEKF innovation spikes, unmodeled impact impulses) and compute the **properly typed conditional probability bound** calibrated strictly against **independent hidden-state ground-truth labels** (simulation oracle or hardware mocap — NEVER self-labeled by the monitor!):
     $$\delta_1 := \frac{\mathbb{E}[N_{\mathrm{silent\_breach}}]}{\mathbb{E}[N_{\mathrm{breach}}]}, \quad \delta_2 := P(\text{violation} \mid \text{alarm}), \quad \delta_3 := P(\text{violation} \land \text{no breach} \land \text{no alarm})$$
     $$P(\text{violation / episode}) \le \mathbb{E}[N_{\mathrm{breach}}]\,\delta_1 + \mathbb{E}[N_{\mathrm{alarm}}]\,\delta_2 + \delta_3$$
     (via Markov's inequality on event counts + union bound under an explicit coverage lemma; explicitly **NOT** claimed invariant under arbitrary out-of-distribution shift, which alters $\delta_1$ itself!).
   - Wrapped in `frob-evidence::ClaimProduct<VerifiedFormal, EvaluatedDesign>` (upgrading factor 2 to `ValidatedPhysics` *only* after real-hardware bench calibration), with **ZERO `From`/`Into` promotion to `VerifiedFormal` (`INV-001`)!**


---

# PART III — MORPHOLOGIES, LIVING MULTIPHYSICS 3D WORLDS & SENSING

## 14. Unified Morphology Grammar (`FMT-FBOT-1`), 18 Archetypes & 8-Stage Preflight

### 14.1 Formal `FMT-FBOT-1` S-Expression / JSON Morphology IR (`frob-bot`, `[S]`)
Every robot in `franken_robotics` — whether authored by a human, emitted by an AI agent over `frob-mcp`, synthesized by `frob-homotopy`, or imported from URDF/MJCF — lowers to one canonical, strictly typed, unit-checked AST (`FMT-FBOT-1`, dual lossless S-expression and JSON views):

```ebnf
BotDoc        ::= "(robot" String ":schema \"frob.bot/v1\"" TopForm+ ")"
TopForm       ::= ImportForm | SymmetryDecl | BaseForm | LinkForm | JointForm | LoopConstraint
                | MirrorMacro | ActuatorForm | PowerBusForm | SensorForm | LeverForm
                | BudgetForm | CapabilityForm | SafetyForm
SymmetryDecl  ::= "(symmetry :mech" String ":obs" String ")"
ImportForm    ::= "(use-module" Ident ":from" String ":mount" Se3Expr
                    (":rename" KvMap)? (":override" KvMap)? ")"
BaseForm      ::= "(base" Ident ":geometry" GeomExpr (":shell-thickness" Qty)?
                    ":material" MaterialId (":pseudo-inertia" Sym4Expr)? ")"
LinkForm      ::= "(link" Ident ":geometry" GeomExpr (":shell-thickness" Qty)?
                    ":material" MaterialId (":gvs-rod" CosseratSpec)? ")"
JointForm     ::= "(joint" Ident ":type" ("revolute"|"prismatic"|"helical"|"spherical"|"fixed"|"free")
                    ":parent" Ident ":child" Ident ":mount" Se3Expr ":axis" Vec3
                    ":limits" "(" Qty Qty ")" ":actuator" Ident
                    (":stiction" Qty)? (":backlash" Qty)? (":contact-law" ContactLaw)? ")"
LoopConstraint::= "(loop-close" Ident ":link-a" Ident ":link-b" Ident ":pose" Se3Expr ")"
MirrorMacro   ::= "(mirror-limb" Ident ":from" Ident ":planes" "(" ("sagittal"|"coronal")+ ")"
                    ":mount" Se3Expr ":mirror-axes" Bool ")"
ActuatorForm  ::= "(actuator" Ident ":kt" Qty ":ke" Qty ":r0" Qty ":inductance" Qty ":poles" Int
                    ":alpha-cu" Qty ":cth" Qty ":gth" Qty ":t-max" Qty ":i-max" Qty
                    ":gear" Num ":backlash" Qty ":mass" Qty ")"
PowerBusForm  ::= "(power-bus" Ident ":battery" BatterySpec ":v-nom" Qty
                    ":i-max" Qty ":thermal-sink" Qty ")"
SensorForm    ::= "(sensor" Ident ":type" Ident ":parent" Ident ":mount" Se3Expr ":rate" Qty ")"
LeverForm     ::= "(levers" "(" Ident ":range" "(" ScalarOrQty ScalarOrQty ")"
                    (":step" ScalarOrQty)? ":target" PathExpr ")"+ ")"
BudgetForm    ::= "(budgets" (":mass-max" Qty)? (":bom-max" Qty)? (":power-max" Qty)?
                    (":width-max" Qty)? (":height-max" Qty)? ")"
CapabilityForm::= "(capabilities" Ident+ ")"
SafetyForm    ::= "(safety" (":fenergy" String)? (":ftube" String)? (":shield-strata" Int)? ")"
BatterySpec   ::= "(li-ion-2rc :cap" Qty ":r0" Qty ":mass" Qty ")"
GeomExpr      ::= "(box" Qty Qty Qty ")" | "(cylinder" Qty Qty ")" | "(capsule" Qty Qty ")" | "(sphere" Qty ")"
Se3Expr       ::= "(se3 :xyz (" DimExpr DimExpr DimExpr ") :rpy (" Qty Qty Qty "))"
DimExpr       ::= Qty | "(" ("+"|"-"|"*"|"/") DimExpr DimExpr ")" | PathExpr
ContactLaw    ::= "(WheelContact :pfaffian" Bool ":mu-nom" Num ":rubber-density" Qty ")"
                | "(ContensouFoot :mu-nom" Num ":r-eff" Qty ")"
                | "(PrescribedBelt :width" Qty ":length" Qty ")"
                | "(TactileElastomerPatch :grid" Int Int ":mu-nom" Num ")"
KvMap         ::= "(" (Keyword Value)+ ")"
ScalarOrQty   ::= Qty | Num
Qty           ::= Num UnitToken  (* Lexer admits: m mm cm kg g s ms rad deg N Nm W V A Ah Ohm mH H
                                    degC K Pa kg/m3 Hz sqrtHz N/m Ns/m J/K W/K Nm/A V/(rad/s) 1/K USD *)
(* Terminals: Ident = [a-zA-Z_][a-zA-Z0-9_-]*, Keyword = ":" Ident, PathExpr = "(" Ident ("." Ident)* ")",
   MaterialId = Ident, Bool = "true"|"false", Num = -?[0-9]+(\.[0-9]+)?, Int = [0-9]+,
   Vec3 = "(" Num Num Num ")", Sym4Expr = "(sym4" Num{10} ")", CosseratSpec = "(gvs :modes" Int ":len" Qty ")" *)
```

**Design Corrections Enforced Directly in `FMT-FBOT-1` (Eliminating All v1 Appendix D Bugs):**
1. **Hollow-Shell Mass Derivation:** Structural boxes/cylinders carry explicit `:shell-thickness` (e.g., `0.52 × 0.34 × 0.14 m` 6061-T6 aluminum [`2700 kg/m3`] at `0.0025m` = **`3.95 kg` shell instead of a `66.83 kg` solid metal block!**); solid-fill defaults emit a loud preflight density warning (`FROB-REF-102`).
2. **Exact Mirroring & Separate Mechanical vs Observation Symmetry:** `(mirror-limb ...)` expands to explicit parametric `:mount (se3 ...)` transforms and reflection-parity screw axes, automatically verifying $\Gamma_{\mathrm{mech}}$ while separately computing $\Gamma_{\mathrm{obs}} \le \Gamma_{\mathrm{mech}}$ when asymmetric sensors (e.g., front-only LiDAR/RGB-D) are attached!
3. **Typed Contact Laws per End-Effector:** Tires/wheels explicitly declare `:contact-law (WheelContact :pfaffian true :mu-nom 0.72 :rubber-density 1150kg/m3)` with dedicated hub actuators (`pmsm-wheel-hub`); legged soles declare `ContensouFoot`; tracks declare `PrescribedBelt`; dexterous fingertips declare `TactileElastomerPatch`.
4. **Declared Co-Design Levers Only:** Every tunable morphological or sensor-mount parameter (including `lidar_mount_z`, `rgbd_mount_z`, `W_stance`, gear ratio `N_knee`) MUST be explicitly declared in `(levers ...)` with hard physical bounds; out-of-range lever mutations fail fast with `FROB-REF-110`!

### 14.2 Complete 18-Archetype Universal Robot Catalog (`frob-bot`, `[S/F]`)

| Category | # | Archetype ID | Key Algebraic / Dynamical Signature | Primary Milestone |
|---|---:|---|---|---|
| **Legged (5)** | 1 | `quadruped-dog` | $V_4$ mechanical / $\mathbb{Z}_2$ obs; 27 contact strata; H/K trot/bound/pace/gallop atlas | M1 / M4 |
| | 2 | `hexapod-spider` | $D_6$ (or $C_6$ complex-irrep!) symmetry; 12× weight cut; fault-tolerant tripod/ripple gaits | M4 |
| | 3 | `octopod-spider` | $D_8$ symmetry; dense contact islands (Tier M/L Delassus + `W02` ragged queue) | M4 |
| | 4 | `biped-walker` | $\mathbb{Z}_2$ sagittal symmetry; underactuated capture-point; saltation impact sensitivity | M3 / M4 |
| | 5 | `humanoid-dexterous` | `frob-g1-29dof` + optional 16-DoF tactile hands; parallel 2-DoF ankles (`W06`/`CC-04`) | M4 / M5 |
| **Wheeled (4)** | 6 | `diff-drive-amr` | Nonholonomic DLA Pfaffian wheels; `.fenergy` bump stops + `.fclear` doorway complexes | M1 / M2 |
| | 7 | `ackermann-ugv` | Steering linkage closure (`W06`); nonholonomic curvature $D\mathbf{A}_S$; outdoor soil | M2 / M3 |
| | 8 | `swerve-omni` | 4-module independent steer+drive ($V_4$ wreath); 4D Contensou scrub torque | M3 |
| | 9 | `mecanum-holonomic` | $\pm 45^\circ$ roller Pfaffian projections; zero-curvature roller-angle bug caught in Stage 4! | M2 |
| **Tracked (2)** | 10 | `tracked-skid-rover` | Prescribed-belt grouser velocity + Bekker–Wong/Janosi–Hanamoto soil shear (`KA-08`) | M3 |
| | 11 | `tracked-flipper-crawler` | Articulated front/rear belt flippers; stair-nosing & rubble climb; thermal cliff testing | M3 / M5 |
| **Hybrids & Continuum (7)** | 12 | `wheeled-legged-quad` | Hybrid DLA Pfaffian rolling + legged step-over; mode-switching `.fskill` transfer (`NS-04`) | M3 / M5 |
| | 13 | `wheeled-humanoid-torso` | Mobile base + dual 7-DoF arms; wreath $\mathbb{Z}_2$ arm symmetry; tray/shelf manipulation | M4 / M5 |
| | 14 | `mobile-dual-arm` | Omnidirectional base + bimanual closed-loop payload grasp (sub-PROP loop escalation) | M5 |
| | 15 | `parallel-delta-arm` | Closed kinematic chains synthesized via `real_rur` / `frob-homotopy`; high-speed pick-place | M3 |
| | 16 | `aerial-terrestrial-quad` | Compact pHS rotor-thruster + Cheeseman–Bennett ground-effect + legged/wheeled chassis | M5 |
| | 17 | `gvs-cosserat-tentacle` | Native dynamic GVS Chebyshev/Legendre Cosserat rod (`frob-dyn`; `fs-solid::rod` static oracle) | M4 |
| | 18 | `modular-operadic-custom` | Arbitrary AI-composed `.fmodule` decorated-cospan topology executed zero-stall on MorphoJIT! | M2 / M5 |

### 14.3 Eight-Stage `<200 ms` Algebraic & Observation-Alias Preflight (`frob-preflight`, `[S/F]`)
Every `.fbot` + `.ftask` + `WorldScene` submission passes an 8-stage deterministic preflight BEFORE allocating a single RL GPU-minute (Stages 1–5 + static torque/mass Stage 6a land in M1; pHS thermal Stage 6b, `.fclear` Stage 7, and Stage 8 land in M3!). Any failure halts immediately with a structured `FROB-REF-nnn` refusal (§22.5):

| Stage | Check & Algebraic Mechanism | Latency Target | Refusal Code & Actionable Output |
|---:|---|---|---|
| **1** | **Schema, Unit Algebra & Macro Expansion** (`frob-units` spectral exponents + shell-mass check) | `<2 ms` | `FROB-REF-100`, `102`, `110` (invalid unit, solid-block density trap, out-of-range lever) |
| **2** | **$\mathrm{Sym}^+(4)$ Pseudo-Inertia Realizability** (`PO-INERTIA-1` exact pivoted $LDL^T$ reflection) | `<3 ms` | `FROB-REF-101` (exact negative eigenvalue + nearest $\mathrm{Sym}^+(4)$ projection) |
| **3** | **Closed-Loop Assembly & Singularity Audit** (`W06` `real_rur` / `CC-04` homotopy + `fs-ivl` branch) | `<25 ms` | `FROB-REF-103` (broken closure or toggle singularity + nearest valid linkage branch) |
| **4** | **Principal Bundle Cartan Curvature Controllability** ($D\mathbf{A}_S = d\mathbf{A}_S - \frac{1}{2}[\mathbf{A}_S \wedge \mathbf{A}_S]$) | `<8 ms` | `FROB-REF-104` (rank-deficient locomotion / zero yaw curvature + roller/joint fix) |
| **5** | **$\Gamma_{\mathrm{mech}} \ge \Gamma_{\mathrm{obs}}$ Symmetry & Operadic Power Conservation** (`PO-OPERAD-1`, `PO-EQUIV-1`) | `<10 ms` | `FROB-REF-108` (algebraic loop, port sign mismatch, or unstated symmetry-breaking sensor) |
| **6** | **Static Torque (`6a`), Extensive-Entropy Thermal Ceiling (`6b`) & Directional Outer Boxes** (replaces rejected `C007`!) | `<15 ms` | `FROB-REF-105` (torque saturation), `FROB-REF-106` (thermal runaway cliff), `FROB-REF-107` (directional box exclusion) |
| **7** | **Dual Inner/Outer $SE(2)$ `.fclear` Clearance & `MorphologyNavEnvelope`** (`FMT-FCLEAR-1`, `COD W08`) | `<40 ms` | `FROB-REF-109` (exact bottleneck doorway/aisle cut + static geometric `K2::frob-worldgen::amend` hint!) |
| **8** | **Observation-Alias Impossibility Witness** (`BS-1`, `frob-preflight::observation_conflict`, `PO-OBSALIAS-1`) | `<95 ms`–`2 s` | `FROB-REF-111` (two admitted hidden-world prefixes with identical quantized transcript & Lean-proved disjoint safe action sets + **cheapest separating sensor / sample-rate / probe fix**!) |

*(Note on Stage 6 — Honoring `DR-02`/`C007`: Log-posynomial transformations produce convex log-sum-exp problems, NOT linear/polynomial constraints for `omega`. Stage 6 therefore uses exact monomial/convex-GP bounds ONLY on verified posynomial thermal/mass subproblems paired with independently `fs-ivl`-verified directional outer exclusion boxes — zero invalid `omega` claims!)*

---

## 15. Living Multiphysics 3D Worlds, Six Interiors, `.fclear`, `fdgr` & Robot–Home Retrofits

### 15.1 Single-Authority 3-Tier `WorldScene` (`K2::frob-world` IR + `E0::frob-gpu::world_scene` Upload, `[S/F]`)
One content-addressed, immutable-per-epoch `WorldScene` (authored as pure device-agnostic IR in `K2::frob-world`, then uploaded to GPU via `E0::frob-gpu::world_scene` — zero `K2 → E0` cycle!) drives rigid/IPC contact, Bekker–Wong terramechanics, 2.5D causal tribology, ray-traced LiDAR/RGB-D/camera sensors, `.fclear` clearance complexes, and `frob-view` rendering identically (`P7`, zero physics-vs-visual drift!):
- **Tier 0 — Static Macro-Architecture (Read-Only):** Compact multi-resolution BVH + sparse 3D VDB SDF tiles + per-surface physical/optical material IDs (roughness, complex IOR for glass/chrome, retroreflectance, baseline $\mu_0$, thermal emissivity). On M5 Ultra, populated via `BlitCommandEncoder::copy_from_buffer` (`BD-029a`) into **`StorageModePrivate`** (`F-01`) so Apple Silicon lossless compression stays engaged (`~140 MB`/building → 500–1,000 resident worlds governed by a `frob-runtime` `≤85%` admission cap on live `recommendedMaxWorkingSetSize = 222.72 GiB` ≈ `189 GiB`, automatically LRU-evicting to `≤500` worlds [`70 GB`] whenever Tier-B `fs-lbm` [`78 GB`] activates!).
- **Tier 1 — Articulated Interactive Fixtures:** Parameterized hinged doors (spring-closer torque, latch force, swing-stop angle), sliding glass doors, rolling drawers, elevators, variable-speed factory conveyors, swinging kitchen saloon doors, and scripted/social moving humans & push-carts — all represented as compact kinematic/pHS fixture descriptors updated inside the same GPU tick.
- **Tier 2 — Dynamic Props & Living Multiphysics Fields:** Manipulable objects (trays, boxes, pots, groceries, fragile glassware), native GVS Cosserat snagging appliance cords (`frob-dyn`), and 2.5D causal tribology film tiles (communicating with `frob-contact` via shared `FrictionField`/`ContactPatchFlux` SoA slabs in `E0::frob-gpu` — zero `frob-contact ↔ frob-multiphys` crate cycle!).

### 15.2 Conservative 2.5D Causal Tribology PDE & 3-Tier Multiphysics Zoom-Lens (`frob-multiphys`, `[F]`)
Static friction randomization cannot represent a robot tracking cooking grease out of a fry station across a tile floor and slipping on its own second pass (`CC-08`). Equally, brute-force 50 Hz 3D LBM (`AGY #9`, `† K-04`) is numerically unstable (`Ma = 0.35–5.2`, `τ - 1/2 = 6e-6`) and consumes 63.7 TB across 32k envs. `frob-multiphys` implements a **mathematically conservative 3-Tier Zoom-Lens**:

| Fidelity Tier | Active Physical Models | Execution Domain & Budget | Verified Analytical Gate |
|---|---|---|---|
| **Tier A (Hot GPU RL, `≤15%` overhead)** | 1. **Conservative 2.5D Thin-Film Causal Tribology PDE** (1 subcycle/step): cell thickness $h(x,y,t)$ & viscosity $\eta$, updated via finite-volume conservative face fluxes combining contact-advected transport AND thin-film gravity spreading across 2D cell **edge length $L_f$ $[\mathrm{m}]$**: $F_f = \big[\mathbb{1}_f\, h_{\mathrm{up}}\,(\bar{v}\cdot n)_f - \frac{\rho g}{3\eta} h_f^3 (\nabla h\cdot n)_f\big]\,L_f$ ($[\mathrm{m}^3/\mathrm{s}]$!), coupled to an explicit **wheel/sole-borne film reservoir** $V_{\mathrm{tire}}$ whose pickup and redeposition fluxes cancel identically (`C6` fix — zero boundary mass creation, audited by compensated global mass residual!). Local traction uses dimensionless Hersey–Stribeck number **$\Pi = \frac{\eta v}{p L_c}$** with explicit patch length $L_c$ (`C7` fix!) feeding $\mu(h, \Pi)$ into 4D De Saxcé cones via shared `E0::frob-gpu` `FrictionField` slabs!<br>2. **3-Mode Lamb–Housner pHS Slosh** for carried pots/trays.<br>3. **Native GVS Cosserat Snagging Cords** (2–4 active cords/room). | All 16k–65k RL envs simultaneously on M5 Ultra & Dual 4090s | `KA-05` (Cosserat cantilever), `KA-11` (Lamb–Housner slosh `±3%`), `KA-12` (**Huppert $t^{1/8}$** constant-volume axisymmetric gravity-current spreading `±2%`), zero global mass drift |
| **Tier B (Local Verification / Qualification Zoom-Lens)** | Full **`fs-lbm` (Mach-bounded subcycled `Ma < 0.20`, `τ > 0.55`, `≤64` envs on M5 Ultra ≈ `78 GB` `C8`)** + **`ftx-drape` implicit IPC cloth/rugs/garments** + **`frob-contact::ipc`** zero-penetration fragile grasp! Triggered automatically ONLY on finalist designs, counterexample verification, and Tier-A surrogate calibration (`W05` far-field causal tile caching where dirty work `≤4.21%`). | M5 Ultra (`≤64` envs, `≤500` LRU worlds) & focused Dual 4090 qualification passes | `GATE-MULTI-01` (Tier-A vs Tier-B rank agreement + discrete port work-energy residual!) |
| **Tier C (Hardware Bench Calibration)** | Real-world instrumented spill-pad traction, drop-weight absorber (`FTA-P3`), and torque-sensor wear logs lifting constitutive material cards from `Estimated` → **`ValidatedPhysics`**. | Offline calibration import (`frob-io`) | Held-out empirical coverage receipt (`INV-001`) |

### 15.3 Dual Inner/Outer $SE(2)$ Clearance Complexes (`FMT-FCLEAR-1`, `COD W08`) & `MorphologyNavEnvelope`
Replacing v1's simplistic scalar-ball `G-WORLD-1` gate (`† K-07`, which fails on rectangular AMRs, trailers, and narrow doorway turns), `frob-worldgen::clearance_complex` constructs **two-sided inner and outer cubical complexes** in $SE(2) = (x, y, \psi)$ (**`240 × 160 × 32`** = `1,228,800` cells on `12 m × 8 m @ 5 cm` with `32` yaw bins = `4.69 MiB` per fixture mode; exact **`74.6 mm`** [`75 mm` conservative] refinement band = `39.3 mm` heading half-bin [`0.4 m × 5.625°`] + `35.4 mm` translation half-diagonal [`25√2 mm`]):
- **Inner Complex $\mathcal{C}_{\mathrm{in}}(w)$:** A connected path from start to goal inside $\mathcal{C}_{\mathrm{in}}(w)$ is a **certified continuous geometric feasibility witness** for every robot whose swept footprint fits inside envelope $w$.
- **Outer Complex $\mathcal{C}_{\mathrm{out}}(w)$:** Disconnection of start and goal in $\mathcal{C}_{\mathrm{out}}(w)$ emits a **certified mathematical impossibility cut** via `fs-topo` persistent homology naming the exact critical doorway/corridor width (reused across 160 width queries at **`14.69×`** speedup!).
- **`MorphologyNavEnvelope` Extension:** Layers vertical overhead clearance (pallet-rack crossbeams, kitchen exhaust hoods) and maximum step-over/stair-riser capability onto `.fclear` so wheeled, legged, and wheeled-legged robots receive exact archetype-accurate clearance verdicts!

### 15.4 Six Realistic Interior Archetypes + Outdoor Terramechanics (`frob-worldgen`, `frob-terramech`)
Every interior generator ships an **M3 Lightweight Structural Proxy** (exact functional clearances, doors, ramps, spills, stairs, lighting, glass — ready in M3 so RL never stalls on asset art!) plus rich M5 procedural/scanned variations:

| # | Scene Archetype | Signature Physical, Geometric & Perception Hazards Modeled | Flagship North-Stars |
|---:|---|---|---|
| **1** | **Homes & Apartments** | Tight 76–82 cm doorways, 12–22 mm thresholds, curling area rugs (`ftx-drape` Tier-B), Cosserat power cords, hardwood-to-carpet $\mu$ transitions, stairs, clutter, dim hallways | `NS-01`, `NS-11`, `NS-12` |
| **2** | **Offices** | Full-height glass conference walls (LiDAR/RGB-D multipath!), rolling swivel chairs, heavy spring-closer doors (`≤22 N·m`), elevators, glossy lobby tile | `NS-06`, `NS-12` |
| **3** | **Warehouses** | Repetitive narrow pallet aisles (SLAM observability aliasing!), overhead rack beams, dock-plate ramps, shrink-wrap debris, moving forklifts/carts, concrete dust | `NS-07`, `NS-16` |
| **4** | **Manufacturing Plants** | Oil/coolant causal films, cable trays, metal grate floors (Contensou twist + foot snag), variable-speed conveyors, metallic glare, vibration, strict ISO/TS 15066 cobot zones (`.fenergy`) | `NS-06`, `NS-13`, `NS-15` |
| **5** | **Retail Stores** | Glass freezer aisles (condensation + specular drop), tight endcaps, uneven spilled liquids, fragile shelf SKUs (IPC grasp), dynamic shopper crossings | `NS-07`, `NS-09` |
| **6** | **Commercial Kitchens** | Causal grease/water thin-film spread from fryers/dishwashers, steam plumes scattering RGB-D, swinging saloon doors, hot-pot 3-mode Housner slosh, narrow 90 cm cooklines, drain grates | `NS-04`, `NS-11`, `NS-15` |
| **7** | **Outdoor 3D Terrains** (`frob-terramech`) | Multi-layer SDF overhangs/culverts, stairs/rubble piles, soft sand/wet-clay/snow Bekker–Wong sinkage $p = (k_c/b + k_\phi) z^n$ & Janosi–Hanamoto shear $\tau = (c + p\tan\phi)(1 - e^{-j/K})$ (`KA-08`, `GB-09`!) | `NS-05`, `NS-10` |

### 15.5 `fdgr` Uncertainty-Coupled Real-to-Sim & Joint Robot–Home Retrofit Co-Design (`B3`!)
1. **Uncertainty-Coupled `fdgr` Ingest (`frob-io`, `BET-16`):** Importing a real building scanned via `franken_drone_geometry_reconstruction` preserves per-voxel geometric variance and occlusion/coverage masks directly inside `WorldScene`. Unobserved surfaces default strictly to `MaterialAuthority::UnprobedConservative` (wide friction/compliance priors fed to `frob-redteam`). When occlusion uncertainty drives a `>15%` robot BOM penalty, the Studio emits an actionable **`fdgr` Re-Scan / Material-Probe Request** pointing at the exact 3D bounding box to capture!
2. **Joint Robot–Environment Retrofit Co-Design (`frob-worldgen::amend`, `frob-redteam::amend` — Blind Spot `B3`!):**
   Real-world building deployments frequently fail on a handful of $12–$80 physical building defects (a 20 mm door threshold, a 22 N·m door closer, an exposed extension cord, a dark corner). Treating the building as immutable forces absurd +$2,400 robot morphology upgrades!
   - `frob-worldgen::amend` maintains a **typed catalog of ~40 physical building amendments**: code-legal ADA threshold ramps, door-closer torque adjustments, magnetic door holders, cord raceways, rug grippers, AprilTag optical fiducials, motion-activated lights, compliant grab handles, and **`COD FTA-P3` `.fenergy` bump-stop absorbers** — each tagged with USD cost, install class (`renter-safe` / `landlord` / `contractor`), code legality constraints, and exact `WorldScene` geometry patch.
   - **Dual of `ddmin`:** Whenever `frob-redteam` emits a Minimal Counterexample World (`FMT-MCW-1`) or `.fclear` emits an impossibility bottleneck (`FROB-REF-109`), `frob-redteam::amend` searches the exact space of **$\le 3$-item amendment kits**:
     $$\binom{40}{1} + \binom{40}{2} + \binom{40}{3} = 40 + 780 + 9{,}880 = \mathbf{10{,}700}\text{ kits}$$
     reusing the candidate robot's **existing frozen policy** (zero retraining for geometry/perception retrofits!) and outputs a concrete installer remedy packet: *"Candidate SKU fails Kitchen Doorway D (`FROB-REF-109`); installing 1× $12 renter-safe 1:12 rubber threshold ramp flips mission solvability without upgrading chassis BOM!"* (`GATE-AMEND-01`, `KILL-AMEND-1`).

---

## 16. Multi-Modal GPU Sensors, Cellular-Sheaf Calibration & Imperfect $SE_{2+m}(3)$ InEKF (`frob-sensors`, `frob-estimate`)

### 16.1 Single-Authority GPU Ray-Traced Sensor Suite (`frob-sensors`, `[S]`)
All sensors execute on the exact same GPU `WorldScene` BVH/VDB as physics, capturing real hardware failure modes that cause sim-to-real collapse:
- **3D/2D Spinning & Solid-State LiDAR:** Per-ray **rolling-shutter motion skew** (interpolated along body twist $\xi$ over scan period!), specular **glass/mirror multipath reflections**, retroreflector bloom, and volumetric **steam/dust Mie scattering** attenuation.
- **RGB-D & Stereo / Event Cameras:** Structured-light IR speckle dropout on glass/chrome/sunlit floors, lens distortion, rolling shutter, motion blur, and asynchronous logarithmic brightness-change event streams.
- **Proprioception & Tactile:** 6-axis IMU (Allan-variance white noise, bias random walk, scale/cross-axis error, accelerometer saturation on hard impacts), joint encoders (quantization + index offset), bus voltage/phase-current/winding-temp sensors, and $4\times 4$ tactile elastomer shear/torsion patch readouts.

### 16.2 Anchored Linearized $\mathfrak{se}(3)$ Cellular-Sheaf Multi-Sensor Extrinsic Calibration (`frob-sensors::sheaf`, `[F]`)
Multi-sensor calibration drift (e.g., a bumped LiDAR mast after an impact) corrupts SLAM silently. `frob-sensors::sheaf` represents the robot's rigid-sensor network as a **cellular sheaf** $\mathcal{F}$ over the sensor graph $(V, E)$ with stalks $\mathcal{F}(v) = \mathfrak{se}(3)$ (extrinsic perturbation twists), edge restriction maps $\rho_{v \triangleleft e} = \mathrm{Ad}_{T_{ve}}$ induced by observed pairwise hand-eye/IMU-camera/LiDAR-odometry residuals $r_e \in \mathfrak{se}(3)$, and an explicit **base-vertex gauge anchor ($\delta_{\mathrm{base}} = 0$ on chassis/IMU)** eliminating the 6D global rigid-body nullspace of $L_0 = \delta_0^T W \delta_0$!
- Solving the anchored weighted **0-th Hodge / Sheaf Laplacian** $L_0 \delta = \delta_0^T W r$ yields the unique Gauss–Newton extrinsic correction on $SE(3)^{|V|-1}$, while the **$W$-orthogonal 1-st cohomology residual** **$\|P_{\mathrm{im}(\delta_0)^{\perp_W}}\, r\|_W$** where $P_{\mathrm{im}(\delta_0)^{\perp_W}} = I - \delta_0 L_0^+ \delta_0^T W$ (NOT Euclidean projection, which errs by `48%` under anisotropic rotation/translation weights $W$!) immediately isolates inconsistent cycle loops (`KA-10`) and localizes the exact bumped sensor bracket!

### 16.3 Imperfect Right-Invariant $SE_{2+m}(3)$ EKF & Observability Gramians (`frob-estimate`, `[S/F]`)
- **Lie-Group State Estimation:** Tracks pose $R \in SO(3)$, velocity $v \in \mathbb{R}^3$, position $p \in \mathbb{R}^3$, and $m$ active stance-foot / landmark positions $d_1, \dots, d_m \in \mathbb{R}^3$ on the matrix Lie group **$SE_{2+m}(3)$**, augmented with IMU gyro/accel biases $(b_g, b_a)$. Explicitly classified as an **Imperfect Right-Invariant EKF** (`DESIGN` — never falsely claiming exact state-independence across bias coupling!), achieving superior convergence over Euler-angle EKFs during aggressive slips and blind-step recovery (`KA-06`).
- **Observability Gramian & Alias-Detection Feed:** Computes the empirical local observability Gramian $\mathcal{O}_k$ along trajectories; its minimum singular value $\sigma_{\min}(\mathcal{O}_k)$ flags feature-starved corridor / repetitive warehouse-aisle SLAM aliasing (`NS-07`), triggers `frob-shield` assumption-monitor warnings, and feeds directly into Stage 8 `observation_conflict` sensor placement co-design!


---

# PART IV — DUAL-SILICON HETEROGENEOUS FORGE, CO-DESIGN CASCADE & ATELIER FOUNDRY

## 17. Dual-Silicon Hardware Forge, `DeviceArena`, Deterministic Scatter & MorphoJIT

### 17.1 Verified Hardware Reality & Asymmetric Role Design (`F-01`, `F-02`, `F-06`)
Rather than treating Apple M5 Ultra and Dual RTX 4090s as identical black boxes, `franken_robotics` enforces **symmetric numerical parity** (`GATE-GPU-TWIN-01`) alongside **asymmetric systems specialization** (`P6`):

| Hardware Property | Local `mac-studio` (Apple M5 Ultra) | Remote `trj` (Dual NVIDIA RTX 4090 `SM_89`) | Architectural Consequence |
|---|---|---|---|
| **Verified Specs (`FACT`)** | `Darwin 27.0.0 arm64`, **80 GPU cores**, **256 GiB (`274,877,906,944` B) UMA**, Metal 4 hardware capability; ~800 GB/s planning assumption (`GATE-FORGE-00`). | **2× 24 GiB GDDR6X**, 1,008 GB/s/card, 128 SMs/card, **48 max warps/SM**, **165 TFLOPS/card = 330.4 dual dense BF16-in/FP32-acc** (660 is sparse!), FP64 1:64 (~1.29 TFLOPS/card), PCIe Gen4 (**no NVLink**, consumer `cuDeviceCanAccessPeer == 0`). | M5 Ultra = **Breadth / World-Bank / Proof / Time-Travel Studio** (`StorageModePrivate` read-only worlds, probed `recommendedMaxWorkingSetSize`). Dual 4090s = **Depth / Specialist RL / Homotopy Foundry** (independent shards, pinned host-staged fixed-order `GPU0+GPU1` sync!). |
| **API & Precision Honesty** | **M2 targets Metal 3.2 honestly** (`OPEN-METAL4-RT` tracks Metal 4/`objc2-metal`). NO native `f64`; `MathMode::Fast` folds away Dekker/Knuth `TwoSum`! `frob-gpu-metal` stays **`#![forbid(unsafe_code)]`** via **`BD-029a` (`ft-kernel-metal v0.2`)** adding zero-copy slices, `PrivateBuffer`, byte offsets, concurrent encoders, and async `SubmitHandle` + `MTLSharedEvent`. | Single audited `unsafe` leaf **`frob-gpu-cuda` (`<600` LOC + `SAFETY.md`)**: Driver API + NVRTC, stream-ordered async pools (`cuMemAllocAsync`) for zero-leak cancellation, **1 dedicated OS thread per GPU** for `CUcontext` safety, persistent `TickControlBlock`, CUDA Graphs for warmed specialist loops, `f64` strictly for `B ≤ 256` twin diagnostics. | `F32DD` audit kernels on Metal MUST compile under `MathMode::Safe` and pass a startup `TwoSum` canary (`FROB-REF-402`). Binary kernels (CUBIN + `MTLBinaryArchive`) live in a content-addressed disk cache in `frob-gpu` (`E0`). |

### 17.2 `DeviceArena` 256-Byte Aligned Slab Layout & `GpuTensorView` (`frob-gpu`, `[S]`)
- **Zero Per-Tick Allocations:** All environment state (`q`, `qd`, `tau`, `AffineRows`, actuator entropy/battery states, sensor rays, contact pairs, `MorphoTable` bytecode, and RL rollout transitions) lives in **256-byte aligned Structure-of-Arrays (SoA)** slabs inside a pre-allocated `DeviceArena`.
- **Honest CPU–GPU Synchronization (`F-06` & `AGY #5` Fix):** Because Metal `StorageModeShared` writes become CPU-visible strictly upon command-buffer completion or `MTLSharedEvent` signal (never claiming unsupported `<0.8 µs` live polling!), every slab carries an explicit epoch-fenced `SlabLease` managed by `asupersync` scopes so CPU diagnostics (`frob-diagnose`) and GPU stepping overlap cleanly across double-buffered ring slots without races or silent stalls.
- **Zero-Copy `GpuTensorView` RL Boundary:** Because `frankentorch::DenseTensor` is 100% CPU `Arc<Vec<T>>` (`F-06`), `DeviceArena` exposes typed, strided **`GpuTensorView`** slices directly into `frob-gpu-metal` (`simdgroup_matrix` 8×8 FP16→FP32 GEMM) and `frob-gpu-cuda` (Ada WMMA BF16/FP16→FP32) — running policy forward, backward, GAE, and PPO updates **100% resident on GPU with zero host copies and zero `rayon` threads**!

### 17.3 Order-Independent GPU Scatter & `StateHash` Replay Determinism (`frob-gpu::determinism`, Blind Spot `B1`!)
A hidden 10× breakpoint caught in Phase 6.9 (`B1`): seeding Philox RNG fixes *random draws*, NOT *floating-point reduction order*! Unordered `f32` GPU atomics during contact-impulse accumulation, BVH refits, and data-dependent warp reductions inject 1-ulp ($2^{-24}$) perturbations that grow as $t \approx \ln(2^{24})/\lambda \approx 16.6/\lambda$ s — collapsing paired CRN correlation from $\rho = 0.95 \to 0.50$ (**10× more episodes per e-race!**) and turning exact `2^10` Shapley into noisy estimation (`1,024 → 13,300` rollouts!).
`frob-gpu::determinism` engineers replay determinism directly into the hot loop:
1. **Order-Independent Contact Scatter:** Stable-key sort + segmented reduction across both Metal and CUDA (or Q32.32 `int64` fixed-point accumulation atomics on CUDA + probed 64-bit atomic coverage on Metal — never assuming Metal 64-bit atomics blindly!).
2. **Fused Per-$K$-Step `StateHash`:** Computes a hardware-accelerated `xxh3` hash over canonical SoA state buffers at every checkpoint boundary.
3. **Receipt-Enforced `DeterminismClass`:** Every receipt using replay, CRN pairing, or Shapley carries `DeterminismClass::{BitwisePerDevice, Shadowed { eps, horizon }, None}`. Attempting exact-Shapley or paired-CRN claims under `None` fails fast with **`FROB-REF-405`**.
4. **Cross-Vendor Honesty:** Metal (`mac-studio`) and CUDA (`trj`) lack common `f64`/FMA contraction rules (`F-01`); cross-vendor pairs are explicitly declared **unpaired** (parity checked strictly via `GATE-GPU-TWIN-01` tolerances, never bit identity!).
- **Gate & Kill (`GATE-DET-01`, `KILL-DET-1`):** Replay 1,000 contact-rich 2 s windows twice per device comparing float atomics vs deterministic scatter. Deterministic path MUST add **`≤10%` env-step overhead**; if raw float atomics are already bitwise on `≥99.9%` of windows with unchanged CRN $\rho$, drop deterministic scatter overhead and keep the `StateHash` verifier gate!

### 17.4 `frob-morphvm` — MorphoJIT + Limb Port-ABI + Ragged Contact Queues + Race Annealing (`[F]`)
Combining the best elements of `CC-01`, `AGY #1`, `COD W02`, and repaired `TA-P2` (`OC-01`):
1. **Tier 0 — 6-Opcode Screw-Bytecode Rigid-Tree Interpreter:** Any newly mutated tree `.fbot` compiles in `<1 ms` into compact 128-B `MorphoTable` rows executing 6 universal Lie-group ABA opcodes (`FK_SCREW`, `BIAS_TWIST`, `INWARD_ABA`, `OUTWARD_ACC`, `ACT_PHS`, `SENSE_TAP`) with branch-free polynomial `sinc`/`cosc` small-angle guards (`C12`). Closed-loop `PFAFF`/`LOOP` constraints route cleanly to the coupled constraint lane.
2. **Tier 0.5 — Precompiled Limb Port-ABI (`AGY #1`):** Unchanged library limbs execute precompiled specialized inward/outward blocks passing exact Featherstone articulated inertia `(SymPsd6, bias)` (`27` floats = `108 B`) at the base port; multi-limb ground contacts couple honestly through the base-coupled constrained ABA/Delassus solve (`C5`).
3. **Contact Phase — Ragged Homogeneous Island Queues (`COD W02`):** Because identical kinematic trees still diverge on active contact counts and solver iterations (`C002`), active contact islands compact dynamically into 3 homogeneous queues matching §8.3's 3-Tier Delassus SRAM (`Tier S ≤10`, `Tier M 11..16`, `Tier L >16` matrix-free)!
4. **Race-Annealed Async Tier-1 Promotion (`TA-P2`, Repaired `C9/C10/C11/C12`):**
   - **Ada Occupancy Sizing (`C10`):** Round-1 batches size at **$B \ge 65{,}536\text{–}131{,}072$** (`2,048–4,096` warps = `8–16` warps/SM across Dual 4090s, fixing the `2 warps/SM` occupancy bug!).
   - **Async Dispatch-Boundary Promotion (`C12`):** As `fs-race` halves the tournament (`512 → 128 → 32 → 8`), background CPU workers compile fused Tier-1 `FMT-KIR-1` PTX/MSL shaders for top survivors and atomically promote cohorts strictly at **acknowledged tick/dispatch boundaries with versioned state maps** (never mid-warp hash switching!).
   - **Honest CRN Pairing (`C9`):** Latin-square world rotation balances *training load* across worlds (rebalanced after each halving); ALL checkpoint and `fs-race` elimination comparisons execute strictly on **paired `(world, replicate_id)` seeds at matched training stages**!
   - **Honest Baseline (`C11`):** Naive 8–20× warm CUDA Graph host-launch multiplication is withdrawn; `XT-P2` benchmarks **edit-to-first-accepted-result** and campaign throughput head-to-head against warm CUDA Graph buckets, `W02` ragged queues, and 9 per-family templates (`H2b`), with contacts matched (`GATE-MORPHVM-01`, `KILL-MORPHVM-1`)!

### 17.5 `frob-gpu::fleet` Breadth/Depth Dual-Silicon Scheduler (`CC-05`, Fixed `C19`, `[S/F]`)
- **Asymmetric Pools:** M5 Ultra holds **500–1,000 resident worlds** in `StorageModePrivate` (**honest `5×`** diversity vs both 4090s sharded `C19`!) + proof/symbolic/time-travel studio; Dual RTX 4090s execute specialist training and homotopy sweeps.
- ***Seed-Not-Voxels* Law:** Cross-machine `asupersync` QUIC messages transmit strictly seeds, recipes, `.fbot`/`.fmodule` diffs, receipts, and policy weights (`≤1 GB/campaign-hr`; cold `fdgr` scan cache misses explicitly metered!).
- **Max-Plus Pipeline Balancing (`C19` Fix):** Calls **`fs-tropical::max_cycle_mean`** (Karp's exact $O(V E)$ algorithm — NOT `critical_circuit`!) combined with measured EWMA queue/compile telemetry to prevent either pole from starving (`GATE-FLEET-01`, `KILL-FLEET-1`).

---

## 18. Declarative STL Tasks (`FMT-FTASK-1`), `GpuTensorView` RL, Gait Atlas, `.fskill` Transfer & 10× Co-Design

### 18.1 Formal `FMT-FTASK-1` Declarative `.ftask`/`.fmission` STL IR (`frob-spec`, `CC-10`, Fixed `C18`)
Every task specification compiles from one declarative S-expression/JSON contract (`FMT-FTASK-1`) declaring:
1. **Explicitly Unit-Normalized STL Predicates (`C18` Fix):** Every safety/mission predicate $\psi_k(x) \ge 0$ declares a physical normalization scale $s_k$ (e.g., `0.10 m` clearance, `10 degC` thermal margin, `5 N·m` torque margin) so $\tilde{\rho}_k = \rho_k / s_k$ is dimensionless before `min`/`max` temporal composition!
2. **Online Past-Time Reward vs Offline Delayed Future-Time Verdicts (`C18` Fix):** Past-time STL operators (`H_[a,b]`, `O_[a,b]`, `S_[a,b]`) compile via Lemire's $O(1)$ streaming monotonic deque directly into `FMT-KIR-1` GPU reward kernels; future-time operators (`G_[a,b]`, `F_[a,b]`, `U_[a,b]`) emit explicit delayed verdicts over completed traces (never silently truncating a 600 s window!).
3. **Canonical `ObsLayout`, `ActLayout`, Multi-Rate Clock & Settled-Pose Reset Cache:** Specifies exact per-sensor rates (e.g., physics 500 Hz, IMU 500 Hz, policy 50 Hz, LiDAR 10 Hz, thermal 10 Hz) and initializes episodes from a pre-computed **quasi-static settled-pose reset cache** (eliminating unreal spawn-penetration impulse spikes!).

### 18.2 Zero-Copy `GpuTensorView` Equivariant RL (`frob-gym`, `frob-equiv`, `[F]`)
- **Vectorized GPU RL Engine (`frob-gym`):** Implements PPO, SAC, TD3, and Evolution Strategies directly over `DeviceArena` `GpuTensorView` buffers with zero CPU `DenseTensor` copies and zero `rayon` dependencies (`F-06`), supporting asymmetric privileged-critic / realistic-sensor-actor training, adaptive domain randomization, and `frob-shield` safe exploration.
- **Equivariant Neural Layers (`frob-equiv`):** Compiles `frob-symmetry`'s exact $\mathbb{R}/\mathbb{C}/\mathbb{H}$ per-space character projectors, regular-rep $\mathbb{R}[\Gamma_{\mathrm{obs}}]^k$ hidden blocks, and wreath-product component ties directly into compact matrix weights consumed by `frob-gpu-metal` (`simdgroup_matrix`) and `frob-gpu-cuda` (WMMA) (`GATE-EQUIV-01`, `KILL-EQUIV-1`).

### 18.3 Equivariant H/K Gait Atlas & Saltation Continuation (`frob-gaitatlas`, `CC-02` + `W07`, Fixed `C13`, `[F]`)
1. **H/K Spatiotemporal Gait Discovery:** Enumerates all admissible symmetry subgroups $(H, K)$ of $\Gamma_{\mathrm{obs}}$ via `PO-GAITENUM-1` (`decide` in `fln-kernel`) — automatically proposing trot, bound, pace, gallop, pronk, tripod, and wave gait seeds for *any* arbitrary 2/4/5/6/8-legged topology!
2. **Symmetry-Reduced Twisted Shooting + Full-Space Stability (`C13` Fix):** Converges periodic orbits on the reduced horizon $[0, T/m]$ in $\mathrm{Fix}(K)$, then **MANDATORILY computes the FULL-SPACE monodromy matrix $D_x P_\theta$ and transient growth spectrum via `frankensim::fs-spectral::floquet`** so hidden transverse symmetry-breaking instabilities are NEVER admitted!
3. **Exact Autonomous Saltation & Envelope-Theorem Co-Design Gradients (`COD W07`):** Augments the state with constant morphology parameters $\theta$ so event-time and impact-reset derivatives propagate automatically through $S = D_x R + \frac{(f^+ - D_x R f^-)\nabla g^T}{\nabla g^T f^-}$, solves the compact settled-cycle adjoint $(I - D_x P_\theta)^T \lambda = \nabla_x J$, and extracts first-order morphology design gradients via the Envelope Theorem (subject to local regularity & residual-to-optimizer distance bounds!).
4. **Honest Refusal Protocol (`W07` Gate):** Grazing impacts ($|\nabla g^T f^-| < \varepsilon_{\mathrm{trans}}$), simultaneous multi-foot impacts, and mode-order switches emit typed `FROB-REF-204` refusals routing automatically to branch/DFO search (`GATE-ATLAS-01`: 100-case suite including 20 grazing/unstable controls ALL refused, `≥72/80` regular cases `cos ≥ 0.95` vs central FD, `13.61×` verified W07 speedup; `KILL-ATLAS-1`).

### 18.4 Physical Capability-Map Transfer (`.fskill` `W03`) + Contact-Consistent `FiberMorph` (`frob-control`, `[F]`)
- **Layer 1 (Always-On Exact Physics — `.fskill` `COD W03`):** Factors skills into a morphology-independent task-space wrench/trajectory spec + exact per-mode affine actuation map $w = w_0(q,\dot{q}) + B_m(q)u$ + bounded contact-consistent Whole-Body Control QP (`WBC-QP`) enforcing joint torque/speed boxes, 4D friction cones, and support limits + electrical/thermal secondary costs. If the target task wrench lies outside the new robot's attainable polytope, `.fskill` immediately emits a **Farkas separating hyperplane witness (`FROB-REF-206`)** BEFORE training!
- **Layer 2 (Zero-Shot Task-Space Residual — `FiberMorph` `AGY #2` Repaired):** A frozen dimensionless Buckingham-$\Pi$ (using corrected linear momentum scale $m_0\sqrt{g\ell_0}$ and angular momentum scale $m_0\ell_0\sqrt{g\ell_0}$!) equivariant graph residual outputs task-space deltas strictly ABOVE `.fskill`'s contact-consistent WBC-QP (`GATE-TRANSFER-01`: `≥18/20` feasible unseen bodies within 3 pp of tuned baselines zero-shot, `4/4` infeasible bodies refused, reporting BOTH incremental and amortized pretraining denominators; `KILL-TRANSFER-1`).
- **Additional Classical & Geometric Controllers (`frob-control`):** Energy-shaping **IDA-PBC** (powered by `PO-SKEW-1` Echeandia–Wensing Coriolis!), Lie-group **iLQR / DDP** (`frob-adjoint`), and certified passive braking backups for `frob-shield`.

### 18.5 The 10× Algebraic Co-Design Cascade, Model-Form Audit (`B2`), Blinded $\alpha$-Spending (`BS-3`) & DPOR (`BS-2`)
`frob-codesign` orchestrates a **6-stage campaign pipeline** eliminating wasted GPU-hours while mathematically blocking both simulator-artifact exploits and adaptive AI benchmark p-hacking:
1. **Stage 1 — Algebraic Quotienting & Preflight:** Collapse mission-equivalent duplicates via `.fquot` (`COD FTA-P1`) and exact closed-loop Burnside orbits (`frob-symmetry`); screen all survivors through the 8-stage `<200 ms` Preflight (including `.fclear` and `BS-1` Observation-Alias witnesses).
2. **Stage 2 — Operadic Reuse & Linkage Synthesis:** Reuse cached `.fmodule` proof/kernel digests on unchanged subassemblies (`W01`); synthesize exact parallel linkages via `W06` `real_rur` / `CC-04` `frob-homotopy`.
3. **Stage 3 — Zero-RL Algebraic & Capability Screening:** Evaluate candidate morphologies via H/K Gait Atlas continuation (`frob-gaitatlas`) and `.fskill` zero-shot wrench allocation (`W03`). To prevent screening bias (`C20`), every campaign runs **COD's blinded stratified audit of discarded designs** (sampling rejected strata directly into Stage 4) + planted poorly-charted winner verification!
4. **Stage 4 — MorphoJIT + `fs-race` Paired Tournament:** Step up to 512 heterogeneous survivors simultaneously via `frob-morphvm` + `fs-race` halving with strictly paired `(world, replicate_id)` CRN checkpoints (`C9`) backed by `B1` `BitwisePerDevice` deterministic scatter!
5. **Stage 5 — Model-Form Robustness & Simulator Discrete Energy-Creation Audit (`frob-codesign::rankcheck` + `frob-bench::model_form` — Blind Spot `B2`!):**
   Before ANY finalist can claim victory or spend prototype budget, Stage 5 re-evaluates top-$k$ finalists & runners-up across **admissible physics variants**: (a) compliant Hunt–Crossley penalty vs De Saxcé complementarity vs IPC barrier; (b) circular 4D vs pyramidal friction cone; (c) contact stiffness $\times\{\frac{1}{2}, 2\}$; (d) $\Delta t / 2$; (e) solver tolerance $\div 10$ — AND runs **`COD FTA-P3`'s energy inequality ON THE SIMULATOR ITSELF** (per-step discrete work–energy audit $\Delta E_{\mathrm{mech}} - W_{\mathrm{act}} \le \varepsilon$) to flag solver-created energy (penetration trampolining or $\sqrt{2}\mu$ diagonal-pyramid exploits!).
   - Passing designs earn **`EvaluatedDesign<ModelRobust { variants, top_k_overlap, energy_audit }>`**; any ranking that flips or exploits solver energy drops to **`ModelConditional`** and is **hard-blocked from spending hardware-prototype budget** (`GATE-RANKCHECK-01`, `≤5–10%` campaign compute!).
6. **Stage 6 — Blinded Qualification Firewall (`frob-ledger::qualification`, `BS-3`) & DPOR Timing Verification (`frob-runtime::schedule_check`, `BS-2`):**
   - **Blinded $\alpha$-Spending Firewall (`BS-3`):** Physically separates the open **Discovery Lane** from a sealed **Blinded Confirmation Runner** consuming fresh independent **world clusters**. Every confirmation request $i$ pre-commits candidate, baseline, mission, and metrics to the FrankenSQLite Ledger, executes a valid paired `fs-eproc` e-process at significance budget **$\alpha_i = \alpha / [i(i+1)]$** ($\sum_{i=1}^\infty \alpha_i = \alpha$, controlling campaign-wide false promotions even under arbitrary adaptive AI retries!), and returns ONLY the authorized decision + aggregate receipt until campaign close (`GATE-QUAL-01`).
   - **DPOR Event-Ordering Check (`BS-2`):** Verifies sensor capture, packet delivery, InEKF read, watchdog expiry, and actuator-commit permutations over `asupersync::LabRuntime` using Dynamic Partial-Order Reduction (exploring only non-commuting conflict orientations — e.g., `720 → 8 = 90×` reduction on 3 conflicting pairs!) to emit minimal timing race counterexamples (`GATE-DPOR-01`, `KILL-DPOR-1`).

### 18.6 E–Engquist HMM 1,000-Hour Lifecycle Compression (`frob-lifecycle`, `CC-09`, Fixed `C17`, `[F]`)
Simulating 1,000 hours at 500 Hz requires $1.8\times 10^9$ steps per robot. `frob-lifecycle` applies **E–Engquist Heterogeneous Multiscale Method (HMM)** two-timescale averaging over settled `frob-gaitatlas` limit cycles:
- Runs short micro-windows to sample steady-state joint loads, sliding velocities, motor currents, and temperatures, then steps macro-aging ODEs (Archard gear tooth wear → growing backlash $\beta_j(t)$ fed right back into `frob-contact`, Miner cyclic fatigue **explicitly carrying `fs-solid::fatigue`'s rainflow residue stack between macro-windows so open half-cycles are never dropped — `C17`!**, Arrhenius insulation aging, and Li-ion SEI capacity fade) using Richardson step-doubling error control!
- **Automatic Bifurcation & Overload Safeguards:** Whenever `fs-spectral::floquet` detects $|\lambda_{\max}| \to 1$ (e.g., backlash triggering gait instability!) or a rare overload event occurs, HMM automatically drops back to micro-resolution integration. **Claim Discipline:** Always tagged `Estimated` until calibrated against real-world accelerated wear trials (`GATE-LIFE-01`, `KILL-LIFE-1`).

---

## 19. CEGIS Red-Teaming, `2^10` Pearl-Causal Time-Travel & Spec-to-Silicon Foundry

### 19.1 CEGIS Red-Teaming, `ddmin` Minimal Counterexamples & `B3` Dual Retrofit Check (`frob-redteam`, `[F]`)
- **Legality-Guarded Adversarial Search (`CC-03` + `AGY #8`):** Searches parameterized interior/terrain perturbations strictly inside physical legality priors ($\mu \ge 0.09$, OSHA hallway $\ge 75$ cm, door closer $\le 22$ N·m, scan uncertainty bands from `fdgr`) via adaptive cross-entropy + `fs-voi` rank-flipping experiment selection (step ratio `6.41×` honest `C14` + `≥1.5×` agent repair-iteration gate).
- **Candidate-Body Solvability Oracle (`C14`):** Rejects impossible traps by requiring a witness feasible for the **candidate robot's actual body, actuators, and mission constraints** (any witness using privileged state is explicitly tagged *plant-model feasibility*, testing information feasibility separately via `BS-1`!).
- **`ddmin` Minimal Counterexample World (`FMT-MCW-1`):** Shrinks every discovered failure to a 1-minimal world edit (`≤3` atomic deltas) that reproduces deterministically (`StateHash`-verified!) AND immediately invokes `frob-redteam::amend` (**`B3`**) to check whether a `$12–$80` ≤3-item home retrofit kit resolves the failure cheaper than a robot chassis redesign (`GATE-REDTEAM-01`, `GATE-AMEND-01`)!

### 19.2 6-Axis Telemetry & `2^10` Pearl-Causal Counterfactual Time-Travel (`frob-telemetry`, `frob-diagnose`, `[F]`)
- **6-Axis Structured Telemetry (`.npz` via `fnp-io`):** Mechanical kinematics/contacts, Actuator thermal/electrical/entropy, Sensor/InEKF observability, Safety shield/energy margins, STL robustness traces, and Determinism `StateHash`es.
- **12 Automated Failure Classes (`FAIL-*`):** `FAIL-SLIP-GREASE`, `FAIL-THERMAL-CLIFF`, `FAIL-BROWNOUT`, `FAIL-GLASS-BLIND`, `FAIL-SLAM-ALIAS`, `FAIL-DOOR-PINCH`, `FAIL-CORD-SNAG`, `FAIL-SLOSH-TIP`, `FAIL-GAIT-BIFURC`, `FAIL-OBS-ALIAS`, `FAIL-RACE-TIMING`, `FAIL-MODEL-DRIFT`.
- **Exact `2^10` Pearl-Causal Counterfactual Forks (`AGY #10` + `B1`):** Rewinds any failure to $t_{\mathrm{fail}} - \Delta t$ in M5 Ultra UMA ring memory (full deterministic state + `W05` far-field tile reuse where dirty fraction `≤4.21%`) and executes all $2^{10} = 1{,}024$ exact `do(...)` causal intervention coalitions (e.g., `do(grease=0)`, `do(N_knee=16)`, `do(lidar_z=+0.08m)`, `do(ramp_D=true)`) under **`B1` `BitwisePerDevice` scatter** in `<1.5 s`, returning **exact Shapley factor attributions** + Wasserstein DRO sensitivity bounds (`GATE-TIMETRAVEL-01`: `≥90%` top-1 planted-cause accuracy, `≥3×` fewer agent repair iterations vs observational logs, `KILL-TIMETRAVEL-1`)!

### 19.3 Spec-to-Silicon Hardware Foundry (`.fbuild`) & `#![no_std]` Embedded Firmware (`frob-mfg`, `frob-codegen`, `[S/F]`)
Consuming/lifting `frankensim`'s real engineering crates (`fs-geocon`, `fs-fab`, `fs-toleralloc`, `fs-topopt`, `fs-io`, `fs-matdb` — `F-03`), `frob-mfg` turns a qualified `.fbot` + `.ftask` into a **single atomic, reviewer-ready `.fbuild` fabrication packet** (`COD W09` + `CC-10`):
1. **Counterexample-Guided COTS Catalog Join (Generalizable-Cut Rule):** Matches motors, gearboxes, bearings, batteries, LiDARs, IMUs, and fasteners against pinned supplier catalogs (`fs-io::supplier_corpus`); failed simulations prune candidate subfamilies **ONLY** when an algebraic lower-vs-upper bound proves the failure generalizes across the entire excluded family!
2. **Parametric CAD, Adjoint GD&T & Wiring Harness:** Emits FreeCAD-validated faceted STEP/STL (`fs-io::step_faceted_export`, C013 post-snap revalidation!), priced COTS BOM, adjoint tolerance stackup (`fs-toleralloc`), and a **Steiner-tree + GVS Cosserat routed wiring/connector harness** checked for bend-radius, joint-sweep collision, and ampacity/voltage-drop limits!
3. **Zero-Alloc `#![no_std]` Firmware Crate (`franken_firmware` Pattern):** Lowers the verified controller, InEKF, `.fenergy` monitor, and box-constrained shield directly from `FMT-KIR-1` into a zero-heap `#![no_std]` Rust crate sitting strictly behind a caller-owned **`MotorBusHal`** trait (`execution_authorized: false`, `hardware_safety_signoff: false`). Verified by: (a) `PO-CODEGEN-FW-1` in `fln-kernel`, (b) QEMU Cortex-M4/M7 differential parity (`≤1e-5` vs Tier-0 over 10k steps, zero heap allocs), and (c) linked binary stack-depth & real-board WCET receipts (`GATE-FW-TWIN-01`, `GATE-S2S-01` requiring `≥11/12` independent reviewer packet acceptance!).

---

## 20. ATELIER AI-Agent Robot Design Studio (`frob-cli`, `frob-mcp`, `frob-tui`, `frob-view`, `frob-report`)

- **Single Verb Authority (`registries/verbs.toml`):** One declarative verb manifest generates CLI commands (`frob`), daemon RPCs (`frobd`), MCP tools (`frob-mcp`), and TUI keybindings (`frob-tui`) with zero drift (`xtask check-verbs`).
- **CLI & Resident Daemon (`frob` + `frobd` in `frob-cli`, `FMT-RPC-1`):** `frob --json` emits deterministic `FMT-ENV-1` envelopes for scripting/CI; `frobd` (second `[[bin]]` in `frob-cli`) runs over Unix domain socket / `asupersync` QUIC keeping `DeviceArena`, M5 Ultra world banks, and compiled kernel caches hot across sub-second agent edit loops!
- **Async Job-Based `frob-mcp` (`fastmcp_rust @ 117388bc`, `INT-FMCP-001`):** Exposes **42 curated tools across 7 clusters** (`design`, `preflight`, `simulate`, `synthesize`, `redteam`, `foundry`, `ledger`) plus MCP discovery `resources://` (`archetypes`, `materials`, `cots_catalog`, `amendments`, `worlds`, `receipts`). **Every long-running tool** (`simulate`, `train`, `gait_atlas`, `homotopy_synth`, `redteam`, `time_travel`, `qualify`, `export_build`) obeys the **strict async job contract**: accepts `sync_wait_ms` (default `5000`, hard cap `30000`), returns immediately with `{ "status": "running", "job_id": "...", "progress_pct": ... }` if work continues, and pairs with `job_poll`, `job_stream`, and `job_cancel` (backed by `asupersync` cancellation + hard compute/walltime budget caps so no runaway agent can hang an MCP socket or burn unbounded GPU-hours!).
- **`frob-tui` (`frankentui`):** Live multi-pane terminal cockpit displaying `fs-race` survival brackets, dual-silicon GPU/memory telemetry, contact-island queue utilization, thermal runaway bars, `alpha_i` qualification budget meter, and failure autopsies.
- **`frob-view` (`NativePresenter` + `franken_threed` WebGPU Dossier Adapter):**
  - Native 60+ FPS Metal/wgpu viewport on `mac-studio` visualizing live `WorldScene`, 4D contact cones, 2.5D causal tribology heatmaps, `.fclear` doorways, and force/energy overlays.
  - **Versioned `WebStudioScenePacket` Adapter (`F-07` Fix):** Because live `franken_threed @ bf7abe1` ships `f3d-core`, `f3d-math`, `f3d-graph`, and `f3d-runtime` while `f3d-gpu`/`f3d-scene`/`f3d-compiler` are planned, `frob-view` emits a self-contained `WebStudioScenePacket` lowered into a **`≤15 MB` single-file offline HTML/WebGPU viewer** (explicitly documented for secure `https://` or `http://localhost` WebGPU context — `GATE-WEBSTUDIO-01`) showcasing interactive ghost-overlay `2^10` counterfactual multiverse replays and `.fbuild` CAD/harness inspection without blocking M4 on planned upstream crates!
- **`frob-report` (`franken_markdown` + `frankenmermaid`):** Generates deterministic Markdown/HTML/PDF engineering dossiers strictly stamped with the Five Claim Axes (`§0.4`), `ClaimProduct` boundaries, blinded qualification receipts, and Galaxy-Brain diagnostic cards.


---

# PART V — FRANKEN CONSTELLATION INTEGRATION, 51-CRATE TOPOLOGY & GOVERNANCE

## 21. Verified Foundation Dependency Ledger (`FD-01`–`FD-17`, Snapshot `2026-11-21`)

All sibling dependencies pin to workspace toolchain **`nightly-2026-08-31`** and **`asupersync = "=0.6.2"`** (deduplicated via canonical local path overrides checked by `xtask check-deps`) with root `[patch.crates-io] block = { git = "https://github.com/Dicklesworthstone/rust-block", rev = "b39ae859..." }` (`F-06`, `F-08`).

| FD ID | Sibling Repo & Verified SHA (`FACT`) | Mode (`Consume` vs `Lift`) | Exact Crates / Capabilities Used & Hard Integration Rules | Admission Gate |
|---|---|---|---|---|
| **`FD-01`** | `frankensim` (`@ 483d09c682`, `nightly-2026-07-06`) | **Consume** (`fs-qty`-free leaves ONLY!) | `fs-ga`, `fs-phs` (`interconnect_mapped`), `fs-symmetry`, `fs-sos` (`verify.rs` interval Cholesky), `fs-eproc`, `fs-spectral::floquet`, `fs-tropical::max_cycle_mean`, `fs-topo`, `fs-ivl`, `fs-conform`, `fs-voi`, `fs-race`, `fs-geocon`, `fs-fab`, `fs-toleralloc`, `fs-topopt`, `fs-asbuilt`, `fs-matdb`, `fs-io`, `fs-tilelang` | `INT-FSIM-001` |
| **`FD-02`** | `frankensim` (`@ 483d09c682`, `fs-qty`-coupled crates!) | **Lift w/ Provenance** (`registries/lifts.toml`) | `fs-mbd`, `fs-contact`, `fs-evidence`, `fs-lbm`, `fs-tribo`, `fs-solid` (`rod`, `fatigue`), `fs-ports`, `fs-system`, `fs-couple` — **pre-classified as Lift** swapping `fs-qty` (`generic_const_exprs`) → stable `frob-units` (`F-03`!) | `INT-FSIM-001` |
| **`FD-03`** | `franken_lean` (`@ 4426f8ea7`, `nightly-2026-08-31`) | **Consume** | `fln-core`, `fln-kernel` (`tc.rs` ≤12 KLOC TCB), `fln-checker` (`38.2 kLOC`), `fln-anvil` (`DTree` + `omega` ONLY — zero use of stubbed `egraph`/`norm_num`/`simp`! `F-04`) | `INT-FLEAN-001` |
| **`FD-04`** | `frankensympy` (`@ 29f847c1`, `nightly-2026-08-20`) | **Consume** (guarded by `KILL-FSYM-1`) | `fsym-core`, `fsym-calculus`, `fsym-matrices`, `fsym-polys`, `fsym-solvers::real_rur` (strictly `≤8` vars, `≤64` eqs, `deg≤256`, `≤4096` bits, 0-D ONLY!), `fsym-proof-kernel` (`49.4 kLOC`, verifier only). Zero reliance on univariate `fsym-formal` (`F-05`)! | `INT-FSYM-001` |
| **`FD-05`** | `frankentorch` (`@ 8165d6a1`, `nightly-2026-08-31`) | **Consume (Scoped) + Upstream `BD-029a`** | `ft-kernel-metal` (**upgraded to v0.2 in `BD-029a`** for slices, `PrivateBuffer`, offsets, concurrent encoders, async `SubmitHandle` + patched `block`!). `ft-core`/`ft-nn` used ONLY for CPU reference & weight I/O — NEVER in GPU RL hot loops (`F-06`)! | `INT-FTORCH-001` |
| **`FD-06`** | `franken_numpy` (`@ e8cb85f4`) | **Consume** | `fnp-ndarray`, `fnp-linalg`, `fnp-io` (`.npz` 6-axis telemetry & CPU reference matrices) | `INT-FNP-001` |
| **`FD-07`** | `franken_threed` (`@ bf7abe1`, `nightly-2026-08-31`) | **Consume (Adapter)** | Live 4 crates (`f3d-core`, `f3d-math`, `f3d-graph`, `f3d-runtime`) + versioned `WebStudioScenePacket` adapter for offline HTML/WebGPU dossiers (`F-07`) | `INT-F3D-001` |
| **`FD-08`** | `asupersync` (`@ b32c5ba66`, `v0.6.2`) | **Consume** (strict `=0.6.2`) | Structured concurrency, cancel-correct GPU/CPU scopes, `LabRuntime` (DPOR host!), QUIC fleet transport | `INT-ASUP-001` |
| **`FD-09`** | `frankensqlite` (`@ 79b735cb2`) | **Consume** | Content-addressed Merkle Design Ledger (`frob-ledger`) + Blinded Qualification Spend Ledger | `INT-FSQL-001` |
| **`FD-10`** | `fastmcp_rust` (`@ 117388bc`, `nightly-2026-08-25`) | **Consume** (verified on `08-31`) | Async `frob-mcp` server (`42` tools, discovery `resources://`, `job_id` streaming/polling) | `INT-FMCP-001` |
| **`FD-11`** | `frankentui` (`@ 5979b92c`) | **Consume** | `frob-tui` live multi-pane design-studio cockpit | `INT-FTUI-001` |
| **`FD-12`** | `franken_textiles` (`ftx-drape`) | **Consume / Lift** | `#![forbid(unsafe_code)]` implicit IPC cloth/rugs + `liquid_mass`/`thermal`/`vapor` (strictly Tier-B evaluation!) | `INT-FTX-001` |
| **`FD-13`** | `franken_drone_geometry_reconstruction` (`fdgr`) | **Consume (Schema Adapter)** | Real-to-sim building meshes + per-voxel coverage/uncertainty masks + active Re-Scan Request protocol | `INT-FDGR-001` |
| **`FD-14`** | `franken_networkx` (`@ 02ea3206a`) | **Consume** | Kinematic tree/cycle/Steiner-tree wiring graph & sensor-sheaf topology algorithms | `INT-FSIM-001` |
| **`FD-15`** | `frankenscipy` | **Consume** | CPU reference sparse/dense linear algebra, quadrature, and special functions (`frob-bench`) | `INT-FNP-001` |
| **`FD-16`** | `franken_markdown` & `frankenmermaid` | **Consume** | Deterministic Markdown/HTML/PDF engineering dossiers & embedded DAG/kinematic diagrams (`frob-report`) | `INT-FTUI-001` |
| **`FD-17`** | `franken_firmware` (Reference Architecture Pattern) | **Pattern Adoption** | `#![forbid(unsafe_code)]` HAL-isolated `#![no_std]` controller lowering + QEMU Cortex-M parity pattern (`frob-codegen`) | `GATE-FW-TWIN-01` |

---

## 22. Complete 51-Crate Topology, Downward Layering & Exact LOC Budgets

### 22.1 Strict Downward Layering (`xtask check-layers`, `C-08` Clean Architecture)
Dependencies flow strictly downward: `U (ATELIER) → V (ORACLE-IO/DIAG) → E2 (CORTEX) → E1 (OPTIC) → E0 (DYNAM/FORGE) → K2 (MORPH/TERRA) → K1 (CAS/PROOF) → K0 (LIE) → S1 (RUNTIME/LEDGER) → S0 (CORE/UNITS/EVIDENCE/CX)`. Verification layer `T` (`frob-bench`, `frob-verify`) depends on lower layers without being imported by any production crate.
- **Layer Cleanliness Fixes (`C-08`):** (1) Pure token crate `frob-cx` (`S0`) passes cancellation/budget handles into `K` crates without dragging `asupersync` QUIC/IO into pure algebra! (2) `SubprocessSymbolicBackend` (`KILL-FSYM-1` fallback) lives strictly in Layer `V::frob-io`, keeping `K1::frob-symbolic` 100% pure! (3) Content-addressed GPU binary kernel cache (CUBIN + `MTLBinaryArchive`) lives in `E0::frob-gpu`. (4) `frobd` is explicitly the second `[[bin]]` inside `U::frob-cli` (`FMT-RPC-1`).

### 22.2 Complete 51-Crate Budget Table (`registries/loc_budget.toml`, `TARGET`, 1.15× CI Ceiling)

*(Phase A ★ marks the `31` M0–M2 critical-path foundation crates totaling `219.2` planned kLOC; workspace helper `xtask` is tracked separately at `~2.5 kLOC`.)*

| # | Crate | Layer | Phase | Unsafe Policy | Planned (kLOC) | Ceiling 1.15× (kLOC) | Bounded Responsibility |
|---:|---|---|---|---|---:|---:|---|
| 1 | `frob-core` | `S0` | ★ | `forbid(unsafe)` | 4.0 | 4.6 | Deterministic Philox RNG, Blake3/xxh3 hashes, `FROB-REF-nnn` error algebra, IDs |
| 2 | `frob-units` | `S0` | ★ | `forbid(unsafe)` | 3.0 | 3.5 | Stable dimensional-exponent algebra (`degC`, `sqrtHz`), Buckingham-$\Pi$, replaces `fs-qty` |
| 3 | `frob-evidence` | `S0` | ★ | `forbid(unsafe)` | 4.5 | 5.2 | Five Claim Axes, non-promotable `ClaimProduct<VerifiedFormal, EvaluatedDesign>`, receipts |
| 4 | `frob-cx` | `S0` | ★ | `forbid(unsafe)` | 1.2 | 1.4 | Pure cancel/budget token isolating `K`-layer algebra from `asupersync` IO/QUIC |
| 5 | `frob-runtime` | `S1` | ★ | `forbid(unsafe)` | 6.5 | 7.5 | `asupersync` scope orchestration, GPU lease budgets, **`schedule_check` DPOR (`BS-2`)** |
| 6 | `frob-ledger` | `S1` | ★ | `forbid(unsafe)` | 7.0 | 8.1 | FrankenSQLite Merkle Design Ledger + **Blinded $\alpha$-Spending Qualification Firewall (`BS-3`)** |
| 7 | `frob-lie` | `K0` | ★ | `forbid(unsafe)` | 6.0 | 6.9 | Normative §6.0 $SE(3)/SO(3)/SE_k(3)$, PGA/CGA (`fs-ga`), `AffineRows` (48 B), $\mathrm{Sym}^+(4)$, `Dual` |
| 8 | `frob-bundle` | `K0` | ★ | `forbid(unsafe)` | 4.0 | 4.6 | Principal bundles, mechanical connection $\mathbf{A}_S$, corrected Cartan curvature $D\mathbf{A}_S$ |
| 9 | `frob-symmetry` | `K0` | ★ | `forbid(unsafe)` | 6.5 | 7.5 | $\Gamma_{\mathrm{mech}}/\Gamma_{\mathrm{obs}}$, $\mathbb{R}/\mathbb{C}/\mathbb{H}$ projectors, wreath, H/K, exact Burnside (`86,185`) |
| 10 | `frob-symbolic` | `K1` | ★ | `forbid(unsafe)` | 9.0 | 10.4 | `frankensympy` PoE/RNEA/ABA, Echeandia–Wensing Coriolis, `W06` exact rational + `real_rur` linkages |
| 11 | `frob-homotopy` | `K1` | — | `forbid(unsafe)` | 10.0 | 11.5 | GPU polyhedral & parameter homotopy (`CC-04`) + Smale $\alpha$-cert (`alphaCertified`) + trace test |
| 12 | `frob-kernel-ir` | `K1` | ★ | `forbid(unsafe)` | 5.5 | 6.3 | Formal `FMT-KIR-1` typed SSA IR, CSE, register-pressure scheduler, congruence trace emitter |
| 13 | `frob-codegen` | `K1` | ★ | `forbid(unsafe)` | 9.5 | 10.9 | Pure 4-target lowering: CPU Rust, Metal MSL (`MathMode::Safe`), CUDA PTX, `#![no_std]` firmware |
| 14 | `frob-proof` | `K1` | ★ | `forbid(unsafe)` | 8.5 | 9.8 | Proved `FrobMath` direct `fln-kernel` computational reflection + `omega` + `PO-*` catalog |
| 15 | `frob-shield` | `K1` | — | `forbid(unsafe)` | 7.5 | 8.6 | Stratified safety: `.fenergy` (`FTA-P3`), `.ftube` (`W04`), box-constrained `ClaimProduct` CBF shields |
| 16 | `frob-operad` | `K2` | ★ | `forbid(unsafe)` | 6.5 | 7.5 | Decorated-cospan `.fmodule` contracts (`W01`), Limb Port-ABI (`SymPsd6`), `.fquot` (`FTA-P1`) |
| 17 | `frob-bot` | `K2` | ★ | `forbid(unsafe)` | 9.5 | 10.9 | `FMT-FBOT-1` parser/macro expander, 18 robot archetypes, semantic structural `diff` |
| 18 | `frob-spec` | `K2` | ★ | `forbid(unsafe)` | 5.5 | 6.3 | `FMT-FTASK-1` unit-normalized STL IR, Lemire streaming kernels, `ObsLayout`/`ActLayout`, reset cache |
| 19 | `frob-actuators` | `K2` | ★ | `forbid(unsafe)` | 6.0 | 6.9 | Extensive-entropy pHS PMSM/gearbox, 2-RC Thevenin battery, aerial-terrestrial rotor port |
| 20 | `frob-preflight` | `K2` | ★ | `forbid(unsafe)` | 8.0 | 9.2 | 8-stage `<200 ms` preflight incl. Stage 8 **`observation_conflict` (`BS-1`)** & directional boxes |
| 21 | `frob-mfg` | `K2` | — | `forbid(unsafe)` | 9.0 | 10.4 | Counterexample-guided `.fbuild` foundry (`fs-geocon`/`fs-fab`/`fs-toleralloc`), STEP, COTS BOM, Steiner harness |
| 22 | `frob-world` | `K2` | ★ | `forbid(unsafe)` | 9.0 | 10.4 | Single-authority 3-Tier GPU `WorldScene`, BVH/VDB (`StorageModePrivate`), fixtures/doors/conveyors |
| 23 | `frob-worldgen` | `K2` | — | `forbid(unsafe)` | 11.5 | 13.2 | 6 realistic interiors (M3 proxies + M5 rich), `.fclear` (`W08`), **`amend` 40-item retrofit catalog (`B3`)** |
| 24 | `frob-terramech` | `K2` | — | `forbid(unsafe)` | 5.5 | 6.3 | Outdoor multi-layer SDF terrains + Bekker–Wong sinkage & Janosi–Hanamoto soil shear (`KA-08`) |
| 25 | `frob-dyn` | `E0` | ★ | `forbid(unsafe)` | 10.5 | 12.1 | Corrected Trapezoidal LGVI + Lie-Midpoint, DLA Pfaffian wheels/tracks, GVS Cosserat rods |
| 26 | `frob-contact` | `E0` | ★ | `forbid(unsafe)` | 12.0 | 13.8 | 4D Coulomb–Contensou NCP, joint stiction/backlash, 3-Tier SRAM Delassus, `W10` multi-RHS, IPC |
| 27 | `frob-morphvm` | `E0` | ★ | `forbid(unsafe)` | 9.5 | 10.9 | MorphoJIT Tier-0 screw bytecode, Tier-0.5 Port-ABI, `W02` ragged islands, `TA-P2` race annealing |
| 28 | `frob-multiphys` | `E0` | — | `forbid(unsafe)` | 8.5 | 9.8 | Conservative 2.5D causal tribology (`CC-08`), 3-mode Housner slosh, Tier-B `fs-lbm`/`ftx-drape` + `W05` tiles |
| 29 | `frob-adjoint` | `E0` | — | `forbid(unsafe)` | 8.0 | 9.2 | Symplectic discrete adjoints, autonomous saltation matrices, multi-RHS `W10` sensitivity probes |
| 30 | `frob-gpu` | `E0` | ★ | `forbid(unsafe)` | 8.5 | 9.8 | `DeviceArena`, `GpuTensorView`, **`determinism` (`B1`)**, `fleet` Breadth/Depth (`CC-05`), CAS binary cache |
| 31 | `frob-gpu-metal` | `E0` | ★ | `forbid(unsafe)` | 7.5 | 8.6 | Safe Metal 3.2 backend via `ft-kernel-metal v0.2`, `simdgroup_matrix` RL, `MathMode::Safe` `TwoSum` canary |
| 32 | `frob-gpu-cuda` | `E0` | ★ | **audited `<600` LOC `unsafe`** | 7.0 | 8.1 | Sole `unsafe` leaf (`SAFETY.md`): `SM_89` Driver/NVRTC, `cuMemAllocAsync`, 1-thread/GPU, WMMA RL |
| 33 | `frob-sensors` | `E1` | — | `forbid(unsafe)` | 9.0 | 10.4 | Ray-traced LiDAR/RGB-D/event (glass/steam/skew), tactile patches, linearized $\mathfrak{se}(3)$ **`sheaf`** calibration |
| 34 | `frob-estimate` | `E1` | — | `forbid(unsafe)` | 6.5 | 7.5 | Imperfect Right-Invariant $SE_{2+m}(3)$ EKF + local observability Gramian ($\sigma_{\min}(\mathcal{O})$) |
| 35 | `frob-equiv` | `E2` | — | `forbid(unsafe)` | 6.0 | 6.9 | $\mathbb{R}/\mathbb{C}/\mathbb{H}$ + regular-rep + wreath + `FiberMorph` equivariant layer compilation |
| 36 | `frob-control` | `E2` | — | `forbid(unsafe)` | 9.5 | 10.9 | `.fskill` capability maps + Farkas witnesses (`W03`), WBC-QP, IDA-PBC, Lie-DDP, backup brakes |
| 37 | `frob-gaitatlas` | `E2` | — | `forbid(unsafe)` | 8.0 | 9.2 | H/K gait atlas (`CC-02`), twisted shooting, `fs-spectral::floquet` full-space check, Envelope gradients |
| 38 | `frob-gym` | `E2` | — | `forbid(unsafe)` | 8.5 | 9.8 | Zero-copy `GpuTensorView` `VectorEnv`, PPO/SAC/ES, asymmetric actor-critic, safe exploration |
| 39 | `frob-codesign` | `E2` | — | `forbid(unsafe)` | 8.5 | 9.8 | 6-stage 10× co-design cascade, blinded discard audit, **`rankcheck` model-form gate (`B2`)** |
| 40 | `frob-lifecycle` | `E2` | — | `forbid(unsafe)` | 6.0 | 6.9 | E–Engquist HMM 1,000-h wear/fatigue (rainflow residue stack `C17`)/Arrhenius/SEI compression |
| 41 | `frob-telemetry` | `V` | ★ | `forbid(unsafe)` | 5.0 | 5.8 | 6-axis `.npz` telemetry recorder (`fnp-io`), ring snapshots, `StateHash` audit stream |
| 42 | `frob-diagnose` | `V` | — | `forbid(unsafe)` | 7.5 | 8.6 | 12 `FAIL-*` autopsies, `2^10` Pearl-causal counterfactual time-travel + exact Shapley + Wasserstein DRO |
| 43 | `frob-redteam` | `V` | — | `forbid(unsafe)` | 8.0 | 9.2 | Legality-guarded CEGIS + `fs-voi` + `ddmin` `FMT-MCW-1` + **`amend` `10,700`-kit dual solver (`B3`)** |
| 44 | `frob-io` | `V` | ★ | `forbid(unsafe)` | 9.5 | 10.9 | Fuzzed URDF/MJCF/glTF/USD/`fdgr` ingest, `SubprocessSymbolicBackend` (`KILL-FSYM-1`), `.fbuild` exporter |
| 45 | `frob-bench` | `T` | ★ | `forbid(unsafe)` | 8.0 | 9.2 | `KA-01`–`KA-14` oracles, `GATE-RIVAL-AUDIT`, F3D §16 thermal ablations, **`model_form` energy audit (`B2`)** |
| 46 | `frob-verify` | `T` | ★ | `forbid(unsafe)` | 8.0 | 9.2 | `std`-only offline `.frobcert` auditor (`~8 kLOC` crate; `~88 kLOC` transitive TCB w/ `fln-checker` + `fsym-proof-kernel`) |
| 47 | `frob-cli` | `U` | ★ | `forbid(unsafe)` | 6.5 | 7.5 | Unified `registries/verbs.toml` `frob` CLI (`FMT-ENV-1`) + **`frobd` resident daemon (`FMT-RPC-1`)** |
| 48 | `frob-mcp` | `U` | ★ | `forbid(unsafe)` | 7.5 | 8.6 | Async `fastmcp_rust` server (42 tools / 7 clusters, `resources://`, `sync_wait_ms`/`job_id`, caps) |
| 49 | `frob-tui` | `U` | — | `forbid(unsafe)` | 6.5 | 7.5 | `frankentui` live cockpit (race brackets, dual-silicon utilization, thermal cliffs, $\alpha_i$ meter) |
| 50 | `frob-view` | `U` | — | `forbid(unsafe)` | 9.5 | 10.9 | Native M5 Metal viewport + versioned `WebStudioScenePacket` `franken_threed` offline WebGPU dossier |
| 51 | `frob-report` | `U` | — | `forbid(unsafe)` | 5.5 | 6.3 | `franken_markdown` + `frankenmermaid` 5-axis dossiers & Galaxy-Brain cards |
| | **TOTAL** | **51 Crates** | **31 ★ (219.2 kLOC)** | **50/51 forbid** | **378.2** | **435.3** | *CI blocks any PR exceeding crate 1.15× ceiling (`xtask loc`)* |

### 22.3 Master Constitutional Invariants (`INV-001`–`INV-012`)

| INV ID | Constitutional Invariant | Automated Enforcement |
|---|---|---|
| **`INV-001`** | **Zero Claim Laundering:** `ClaimProduct<VerifiedFormal, EvaluatedDesign>` has NO `From`/`Into` conversion to `VerifiedFormal`; all artifacts carry explicit axis + `execution_authorized: false`, `hardware_safety_signoff: false`. | Rust type system + `frob-verify` |
| **`INV-002`** | **Memory-Safety Boundary:** `50/51` crates carry `#![forbid(unsafe_code)]`; `frob-gpu-cuda` `<600` LOC audited `unsafe` with `SAFETY.md`. | `xtask check-unsafe` |
| **`INV-003`** | **Strict Downward Layering:** Zero circular crate dependencies; `K`-layer algebra never imports `asupersync` IO or subprocesses (`frob-cx`, `V::frob-io`). | `xtask check-layers` |
| **`INV-004`** | **Physical Inertial Realizability:** Every rigid body inertia satisfies $J \in \mathrm{Sym}^+(4)$ (`PO-INERTIA-1`); non-SPD inertias refuse with `FROB-REF-101`. | `frob-preflight` Stage 2 |
| **`INV-005`** | **Single-Authority World Truth:** One immutable `WorldScene` feeds physics, terramechanics, tribology, sensors, `.fclear`, and visuals. | `frob-world` handle typing |
| **`INV-006`** | **Conservative Contact & Tribology Physics:** Zero mass creation in 2.5D tribology flux; zero silent solver-created energy ($\Delta E_{\mathrm{mech}} - W_{\mathrm{act}} \le \varepsilon$ audited in `B2`). | `GATE-TRIBO-01`, `GATE-RANKCHECK-01` |
| **`INV-007`** | **Replay & Pairing Determinism Honesty:** Exact Shapley & CRN e-race receipts require `DeterminismClass::BitwisePerDevice` + `StateHash` (`B1`); Metal/CUDA pairs declared unpaired. | `frob-gpu::determinism`, `FROB-REF-405` |
| **`INV-008`** | **Closed-Loop Symmetry Soundness:** Only $\Gamma_{\mathrm{obs}} \le \Gamma_{\mathrm{mech}}$ (preserving sensors, payloads, and tasks) is admitted for equivariant RL, Burnside, or H/K gaits. | `PO-EQUIV-1` |
| **`INV-009`** | **Cancel-Correct Resource Leases:** Every GPU/CPU scope cancels cleanly under `asupersync` with zero leaked `cuMemAllocAsync` pools or torn Ledger writes. | `asupersync::LabRuntime` CI |
| **`INV-010`** | **Blinded Qualification Firewall:** Discovery lane cannot read confirmation world clusters; confirmation spend obeys $\sum \alpha_i \le \alpha$ (`BS-3`). | `frob-ledger::qualification` |
| **`INV-011`** | **No Stubbed Proof Tooling:** Every `PO-*` runs end-to-end inside `fln-kernel` reflection + `omega` + offline `fln-checker` with zero `sorry` / axioms / stubbed `fln-anvil` calls. | `frob-verify` (`GATE-PROOF-01`) |
| **`INV-012`** | **Single Verb & Format Authority:** One `registries/verbs.toml` generates CLI/daemon/MCP/TUI; every durable artifact carries an explicit versioned `FMT-*` schema header. | `xtask check-verbs` |

### 22.4 Master Durable Formats Registry (`FMT-*`)

| Format ID | File / Wire Artifact | Schema / Encoding | Owner Crate |
|---|---|---|---|
| **`FMT-FBOT-1`** | `.fbot` / `.fbot.json` | Typed S-expression / JSON robot morphology AST (hollow shells, units, levers, symmetry) | `frob-bot` |
| **`FMT-FMOD-1`** | `.fmodule` | Content-addressed decorated-cospan open port-Hamiltonian module + proof digest | `frob-operad` |
| **`FMT-FTASK-1`** | `.ftask` / `.fmission` | Unit-normalized STL contract + `ObsLayout`/`ActLayout` + multi-rate clock + reset cache | `frob-spec` |
| **`FMT-KIR-1`** | `.kir` | Statically bounded zero-alloc SSA kernel IR lowering to CPU/Metal/CUDA/`no_std` | `frob-kernel-ir` |
| **`FMT-FQUOT-1`** | `.fquot` | Mission-observable quotient table + equivalence proof digest (`COD FTA-P1`) | `frob-operad` |
| **`FMT-FCLEAR-1`** | `.fclear` | Dual inner/outer $SE(2)$ cubical clearance complex + `MorphologyNavEnvelope` (`COD W08`) | `frob-worldgen` |
| **`FMT-FENERGY-1`** | `.fenergy` | Observable boundary energy budget receipt + absorber calibration digest (`COD FTA-P3`) | `frob-shield` |
| **`FMT-FTUBE-1`** | `.ftube` | ≤4D hybrid reachable-tube viability bitset + backup domain certificate (`COD W04`) | `frob-shield` |
| **`FMT-FSKILL-1`** | `.fskill` | Task-space skill + capability-map pullback contract + Farkas witness schema (`COD W03`) | `frob-control` |
| **`FMT-MCW-1`** | `.fmcw` | `ddmin` 1-minimal counterexample world delta (`≤3` edits) + solvability & `StateHash` witness | `frob-redteam` |
| **`FMT-FBUILD-1`** | `.fbuild` | Atomic fabrication review packet (STEP CAD, COTS BOM, GD&T, Steiner harness, `#![no_std]` FW) | `frob-mfg` |
| **`FMT-CERT-1`** | `.frobcert` | Offline-verifiable Merkle bundle of `fln-checker` proofs, `ClaimProduct` & qualification receipts | `frob-verify` |
| **`FMT-ENV-1`** | stdout JSON | Deterministic `frob --json` CLI envelope (schema version, claim axis, refusal/result) | `frob-cli` |
| **`FMT-RPC-1`** | Unix / QUIC JSON-RPC | Framed `frobd` resident daemon & `frob-gpu::fleet` control protocol | `frob-cli` |
| **`FMT-WSP-1`** | `WebStudioScenePacket` | Versioned binary/compressed packet lowered to offline `franken_threed` HTML/WebGPU dossier | `frob-view` |
| **`FMT-TELEM-1`** | `.npz` | 6-axis structured telemetry archive (`fnp-io`) with per-checkpoint `StateHash` | `frob-telemetry` |

---

## 23. Determinism Classes

### 23.1 Explicit Determinism Classes (`INV-007`, `B1`)
1. **`BitwisePortable` (CPU `f64` Oracle & `frob-verify`):** IEEE-754 strict (no FMA contraction variance), exact rational/integer `fln-kernel` reflection, deterministic Philox RNG; bit-identical across ARM64 macOS and x86_64 Linux.
2. **`BitwisePerDevice` (GPU Deterministic Replay & CRN Mode):** Order-independent GPU scatter (`frob-gpu::determinism`: sort + segmented reduce or Q32.32 fixed-point atomics) + fused `xxh3` `StateHash` — bit-identical across repeated replays on the same GPU architecture and driver (`required` for exact `2^10` Shapley & paired `fs-race` CRN!).
3. **`Shadowed { eps, horizon }` (Fast GPU Training Mode):** Permitted for throughput-only discovery sweeps with bounded divergence verified by periodic `StateHash` shadowing; cross-vendor Metal↔CUDA runs are ALWAYS `Shadowed` (bounded by `GATE-GPU-TWIN-01`, explicitly unpaired!).
4. **`None`:** Uncontrolled float atomic drift — explicitly refused (`FROB-REF-405`) by any CRN, Shapley, or qualification workflow!

---

## 24. Interop & Security Boundaries

### 24.1 Interop & Security Posture (`frob-io`, `frob-mcp`, `NONGOAL-001`)
- **Fuzzed Untrusted Importers:** All URDF, MJCF, glTF, USD, and `fdgr` parsers run in pure `#![forbid(unsafe_code)]` Rust with strict memory/depth/polygon limits and `cargo-fuzz` CI targets (`>10M` executions, zero panics).
- **Hard Hardware Safety Air-Gap (`NONGOAL-001`):** Every `.fbuild` packet, `#![no_std]` crate, MCP response, and dossier hard-codes `execution_authorized: false`, `hardware_safety_signoff: false`, and `sim_to_real_certified: false`. Generated firmware compiles strictly against caller-supplied `MotorBusHal` traits and NEVER touches raw physical registers, CAN sockets, or E-stop relays directly.
- **MCP & Daemon Sandboxing:** `frob-mcp` enforces path-jailed project roots, strict wall-clock / GPU-minute budgets per tool call, `sync_wait_ms` async handoffs, and capability-token authorization before invoking remote `trj` fleet jobs.

---

# PART VI — VERIFICATION, MASTER GATES, RISKS, MILESTONES & ZERO-ORPHAN BEADS DAG

## 25. Analytical Known-Answer Suite (`KA-01`–`KA-14`) & Brenner Exclusion Tests (`XT-P1`–`XT-P3`)

### 25.1 Pinned Analytical Known-Answer Benchmarks (`KA-01`–`KA-14`, `frob-bench`)

| KA ID | Analytical / Reference Benchmark | Quantitative Acceptance Criterion (`VerifiedNumeric`) | Milestone |
|---|---|---|---|
| **`KA-01`** | §6.0 Lie/PGA/$\mathrm{Sym}^+(4)$ & $O(3)$ Reflection Identities | Exact to `1e-13` (`f64`); small-angle `sinc`/`cosc` smooth down to `1e-15` rad | M1 |
| **`KA-02`** | Asymmetric Free Rigid Body + Heavy Top (Trapezoidal LGVI) | Relative energy error `<1e-6` at `h=0.25 ms`, `O(h^2)` convergence, backward time-reversibility `<1e-13` | M1 |
| **`KA-03`** | Coupled Kinematic Snake Cartan Curvature Holonomy ($D\mathbf{A}_S$) | Minus-sign $d\mathbf{A}_S - \frac{1}{2}[\mathbf{A}_S\wedge\mathbf{A}_S]$ matches right-ordered $\overrightarrow{\mathcal{P}}\exp$ within `1e-3` on small loops (`2.026` wrong sign rejected!) | M1 |
| **`KA-04`** | Painlevé Paradox, 4D Contensou Spinning Disk & Rolling Sphere (DLA) | Exact slip-stick transition time & spin-translation coupling within `1e-4`; zero pyramidal $\sqrt{2}\mu$ diagonal bias! | M2 |
| **`KA-05`** | Large-Deflection Elastica Cantilever Beam (GVS Cosserat Rod) | Matches exact elliptic-integral tip deflection & curvature within `0.5%` | M3 |
| **`KA-06`** | Imperfect $SE_{2+m}(3)$ Right-Invariant EKF Slip & Blind-Step Recovery | Faster basin convergence & `≤2%` NEES calibration error vs quaternion EKF under `45°` initial yaw error | M4 |
| **`KA-07`** | Rimless Wheel & Compass-Gait Saltation Monodromy Matrix | Analytical Poincaré return-map eigenvalues & saltation derivatives match `1e-6` (`fs-spectral::floquet`) | M4 |
| **`KA-08`** | Bekker–Wong Plate Sinkage & Janosi–Hanamoto Tracked Drawbar Pull | Matches analytical sinkage (`3.52 cm` / `9.79 cm` / `4.96 cm`, `GB-09`) & shear curves within `1e-5` | M3 |
| **`KA-09`** | Extensive-Entropy pHS PMSM Thermal Runaway Cliff | Exact analytical ODE $T(t)$ & cliff time `33.5 s` (`GB-08`, `I_rms=25.4 A`) matched within `0.1 s`; Clausius–Duhem $\sigma_{\mathrm{irr}} \ge 0$ | M3 |
| **`KA-10`** | Linearized $\mathfrak{se}(3)$ Cellular-Sheaf Calibration Localization | Recovers planted `2° / 15 mm` bumped LiDAR bracket with `100%` localization via $H^1$ residual | M4 |
| **`KA-11`** | 3-Mode Lamb–Housner Fluid Slosh Fundamental Frequency & Base Shear | Matches analytical Housner tank natural frequency & impulsive/convective forces within `±3%` | M3 |
| **`KA-12`** | **Huppert $t^{1/8}$ Axisymmetric Thin-Film Spreading & Mass Conservation** | Conservative 2.5D causal tribology matches Huppert similarity radius $R(t) \propto t^{1/8}$ within `±2%` & global mass drift `<1e-6` | M3 |
| **`KA-13`** | Exact Linkage Counts: 3/5-Pose Burmester, 9-Point 4-Bar (`4,326`), Stewart (`40`), 6R (`16`) | Exact isolated real root counts certified by `W06` (`real_rur`) & `CC-04` (`frob-homotopy` Smale $\alpha$-cert) | M1 / M3 |
| **`KA-14`** | `.fenergy` Drop-Stop Energy Conservation (`1.49 J ≤ 2.00 J`, `50.5/45.5 ms`) | Fixed-point `fln-kernel` proof + simulated multi-bounce trajectories NEVER exceed $W_{\mathrm{abs,min}}$ for any $\mu \in [0, 5]$ | M2 / M4 |

### 25.2 Brenner Crucial Exclusion Tests (`XT-P1`–`XT-P3` + `GATE-RIVAL-AUDIT`)

| XT ID | Paradox Resolved | Experimental Arms & Potency Controls | Decisive Pass / Kill Verdict | Milestone |
|---|---|---|---|---|
| **`XT-P1`** | **P1:** Campaign-Time Algebra vs Per-Step Algebra | 4 arms on 128 morphologies (`A0` generic, `A1` Tier-1 CSE only, `A2` quotient+atlas only, `A3` both) + **COD ballast potency fixture (`735,471 → 24,546 = 29.96×`, second-moment & harness split controls!)** + **COD blinded stratified audit of discarded designs** + planted poorly-charted winner | **Pass:** `A2 ≥ 5×` & `A3 ≥ 10×` (95% paired LCB) over `A0`, `≥6/8` top-8 overlap, planted winner kept #1, 0 false quotients. **Fail:** demote algebraic screen (`KILL-OPERAD-1`). | M4 |
| **`XT-P2`** | **P2:** Heterogeneous Batching vs Homogeneous Specialization | 4 arms on `H512` (512 topologies, realistic contact tails, $K \in \{1, 8, 32, 128\}$) on BOTH M5 Ultra & Dual 4090: (1) MorphoJIT Tier-0+0.5+Race; (2) `W02` ragged operator queues; (3) warm CUDA Graph buckets; (4) **`H2b` 9 per-family padded templates** | **Pass:** winning hybrid beats warm graphs on edit-to-first-accepted-result at $n_m \le 128$ and sustains `≥0.33×` DoF-normalized Tier-1 speed. **If `H2b` wins on ≥95% coverage & throughput:** adopt `H2b` cleanly (`KILL-MORPHVM-1`)! | M2 |
| **`XT-P3`** | **P3:** Formal Guarantees vs Real-World Contact Uncertainty | 5,000 in-envelope + 5,000 shifted-world episodes testing `.fenergy` (impact/stop), `.ftube` (≤4D reset), and `ClaimProduct` box-constrained shield against **independent hidden-state breach labels** & identically sampled shifted baseline | **Pass:** 0 `.fenergy`/`.ftube` false admissions; empirical violation satisfies Markov count bound $\mathbb{E}[N_b]\delta_1 + \mathbb{E}[N_a]\delta_2 + \delta_3$ on calibrated distribution; shield keeps `≥85%` nominal return (`KILL-SHIELD-1`). | M4 |

---

## 26. Master Gate Registries (`INT-*`, `GATE-*`, `KILL-*`)

### 26.1 Sibling Foundation Admission Gates (`INT-*`, All Cleared by M0/M1!)

| INT ID | Sibling / Target | Exact Verification Check | Milestone |
|---|---|---|---|
| **`INT-FSIM-001`** | `frankensim @ 483d09c682` | Verify `fs-qty`-free leaves compile clean on `nightly-2026-08-31`; verify `registries/lifts.toml` provenance lifts of `fs-mbd`/`fs-contact`/`fs-lbm`/`fs-tribo`/`fs-solid`/`fs-ports` pass unit tests on `frob-units` | M0 |
| **`INT-FLEAN-001`** | `franken_lean @ 4426f8ea7` | `fln-kernel` + `fln-checker` + `fln-anvil::omega` compile & typecheck sample `FrobMath` reflective theorem offline with zero `sorry` | M0 |
| **`INT-FSYM-001`** | `frankensympy @ 29f847c1` | `fsym-core`/`calculus`/`matrices`/`real_rur` compile on `08-31` and pass 3-pose rational + 5-pose Burmester fixture within `MAX_VARIABLES=8` cap (else trigger `KILL-FSYM-1`) | M0 |
| **`INT-FTORCH-001`** | `frankentorch @ 8165d6a1` | `[patch.crates-io] block` builds `ft-kernel-metal` clean on macOS; `BD-029a` v0.2 extensions pass slice/offset/async/concurrent-encoder tests; `xtask check-deps` proves zero `rayon` in GPU crates | M0 / M2 |
| **`INT-FNP-001`** | `franken_numpy` + `frankenscipy` | `.npz` round-trip (`fnp-io`) + reference linear algebra parity pass on ARM64 & x86_64 | M0 |
| **`INT-F3D-001`** | `franken_threed @ bf7abe1` | Live 4 crates (`f3d-core`/`math`/`graph`/`runtime`) compile clean on `08-31` behind `WebStudioScenePacket` adapter | M0 / M4 |
| **`INT-FMCP-001`** | `fastmcp_rust @ 117388bc` | Compiles cleanly on workspace `nightly-2026-08-31` (upgraded from `08-25`) and passes async `job_id` + cancel test | M0 |
| **`INT-ASUP-001`** | `asupersync = "=0.6.2"` | Single deduplicated package ID in `Cargo.lock` (`xtask check-deps`); `LabRuntime` deterministic replay & QUIC transport pass | M0 |
| **`INT-FSQL-001`** | `frankensqlite @ 79b735cb2` | WAL-mode crash-recovery & Merkle content-addressed receipt insertion pass | M0 |
| **`INT-FTUI-001`** | `frankentui` + `franken_markdown`/`mermaid` | Non-interactive TUI snapshot render & deterministic Markdown/Mermaid dossier generation pass | M0 |
| **`INT-FTX-001`** | `franken_textiles::ftx-drape` | Implicit IPC cloth rug/garment fixture passes Tier-B evaluation import with zero `unsafe` | M3 |
| **`INT-FDGR-001`** | `fdgr` | Real building scan + per-voxel coverage uncertainty ingest passes schema & `MaterialAuthority::UnprobedConservative` tagging | M5 |

### 26.2 Master Milestone & Verification Gates (`GATE-*`)

| GATE ID | Milestone | Exact Pass Criterion (All Performance Gates Use Paired `n >= 10`, 95% LCB, 20-Min Thermal Floor!) |
|---|---|---|
| **`GATE-RIVAL-AUDIT`** | M0 | Pin exact versions & live capabilities of MuJoCo/MJWarp, Isaac Lab (PhysX/Newton), Drake, and Genesis (incl. Metal); freeze fair reference baselines |
| **`GATE-FORGE-00`** | M0 | Hardware fingerprint receipt on `mac-studio` (M5 Ultra measured STREAM GB/s, `recommendedMaxWorkingSetSize`, Metal 64-bit atomic probe) & `trj` (Dual 4090 measured GB/s, dense BF16 TFLOPS, `cuDeviceCanAccessPeer`) |
| **`GATE-DYNAM-01`** | M1 | `KA-01`, `KA-02`, `KA-03` pass; Trapezoidal LGVI energy error `<1e-6` (`h=0.25 ms`); GPU cost vs Lie-Midpoint measured (`KILL-LGVI-1`) |
| **`GATE-BURMESTER-01`** | M1 | `W06` 3/5-pose rational + `real_rur` linkage synthesis certifies exact real branches & zero toggle singularities (`KA-13` M1 slice) |
| **`GATE-OPERAD-01`** | M1 | `.fmodule` mirror-swap reuses leaf proofs (`≥10×` derivation speed); shared-bus & base-coupled Delassus edits properly invalidate; adversarial reversed port sign refused (`PO-OPERAD-1`) |
| **`GATE-PROOF-01`** | M1 | Proved `FrobMath` direct `fln-kernel` reflection passes `PO-LIE-1`, `PO-INERTIA-1`, `PO-SKEW-1`, `PO-OPERAD-1`, `PO-CODEGEN-1` in `<5 s` + offline `frob-verify` (`fln-checker`) zero-`sorry` replay |
| **`GATE-PREFLIGHT-01`** | M1 | 8-stage preflight (Stages 1–7 in M1; Stage 8 in M3) catches `≥90%` of 100 seeded physical/geometric/symmetry defects in `<200 ms` with zero false admissions |
| **`GATE-GPU-TWIN-01`** | M2 | Margin-separated CPU `f64` ↔ Metal 3.2 ↔ CUDA `SM_89` parity (`<1e-4` rigid, `<1e-3` contact impulse over 500 steps); Metal `MathMode::Safe` `TwoSum` canary passes (`FROB-REF-402`) |
| **`GATE-FORGE-01`** | M2 | Sustained 20-min thermal throughput on M5 Ultra (`≥60%` roofline on `DeviceArena`) & Dual 4090 (`≥1.5M` rigid / `≥250k` dense-contact env-steps/s homogeneous `TARGET`) |
| **`GATE-DET-01`** | M2 | Order-independent GPU scatter (`B1`) achieves 100% `BitwisePerDevice` `StateHash` repeatability across 1,000 contact-rich 2 s windows at `≤10%` overhead (`KILL-DET-1`) |
| **`GATE-MORPHVM-01`** | M2 | Executes `XT-P2` on `H512` (M5 Ultra + Dual 4090, $B \ge 65\text{k}$); measures crossover $n_m^*$ & edit-to-first-accepted-result (`KILL-MORPHVM-1`) |
| **`GATE-CONTACT-02`** | M2 / M3 | `KA-04` passes (4D Contensou + DLA wheels/tracks + joint stiction/backlash); 3-tier Delassus SRAM fits 1.86/4.70 KB; `W10` multi-RHS meets `1e-6` residual & `1e-4` `f64` parity |
| **`GATE-CONTACT-03`** | M3 | IPC barrier lane (`frob-contact::ipc`) achieves zero tunneling/penetration on thin-cord & fragile-SKU manipulation fixtures |
| **`GATE-COSSERAT-01`** | M3 | `KA-05` GVS Cosserat elastica matches analytical deflection within `0.5%` inside unified ABA tree |
| **`GATE-ACT-01`** | M3 | `KA-09` extensive-entropy pHS PMSM thermal cliff (`33.5 s`) & 2-RC battery sag match analytical ODE within `0.1 s` |
| **`GATE-CLEAR-01`** | M3 | `.fclear` (`W08`, `74 mm` margin) + `MorphologyNavEnvelope` emit 0 false route & 0 false impossibility witnesses across 6 interior proxies |
| **`GATE-OBSALIAS-01`** | M3 | `observation_conflict` (`BS-1`, `PO-OBSALIAS-1`) catches planted blind spots & emits 0 false impossibility witnesses on blinded feasible/probe negative controls |
| **`GATE-HOMOTOPY-01`** | M3 | `frob-homotopy` (`CC-04`) recovers exact `4,326` / `40` / `16` roots (`KA-13`), `≤2 s` parameter re-query, `PO-ALPHA-1` verified, honest `Complete`/`Partial` label (`KILL-HOMOTOPY-1`) |
| **`GATE-TRIBO-01`** | M3 | Conservative 2.5D causal tribology passes `KA-12` (Huppert `±2%`, mass drift `<1e-6`) & `KA-11` (Housner `±3%`) at `≤15%` GPU RL overhead (`KILL-TRIBO-1`) |
| **`GATE-MULTI-01`** | M4 / M5 | Tier-B zoom-lens (`fs-lbm` `Ma<0.20`, `≤64` envs M5 + `ftx-drape` IPC) matches Tier-A kitchen/home design rankings & reports discrete port work-energy residuals |
| **`GATE-FLEET-01`** | M4 | M5 Ultra (500–1,000 `StorageModePrivate` worlds, honest `5×` vs dual 4090s) + Dual 4090s via `asupersync` QUIC (`≤1 GB/campaign-hr` *seed-not-voxels*) achieves `≥1.5×` campaign wall-clock speedup (`KILL-FLEET-1`) |
| **`GATE-EQUIV-01`** | M4 | $\mathbb{R}/\mathbb{C}/\mathbb{H}$ + regular-rep + wreath equivariant RL on `GpuTensorView` achieves `≥1.8×` sample efficiency (10-seed paired) on symmetric quadruped/hexapod with zero asymptotic regression (`KILL-EQUIV-1`) |
| **`GATE-ATLAS-01`** | M4 | `frob-gaitatlas` passes `KA-07` + W07 100-case gate (20/20 grazing/unstable refused, `≥72/80` `cos≥0.95`, full-space `fs-spectral::floquet` verified, `≥10×` vs FD, `KILL-ATLAS-1`) |
| **`GATE-SHIELD-01`** | M4 | Stratified safety passes `KA-14` (`.fenergy`), `.ftube` (≤4D ambiguous reset), box-constrained `ClaimProduct` shield (`XT-P3`, `≤5%` overhead, 0 box violations, `KILL-SHIELD-1`) |
| **`GATE-DPOR-01`** | M4 | `frob-runtime::schedule_check` (`BS-2`) catches 100% of planted stale-read & watchdog/commit timing races while reducing 720-permutation fixture to 8 classes (`90×`, `KILL-DPOR-1`) |
| **`GATE-TRANSFER-01`** | M5 | `.fskill` (`W03`) + `FiberMorph` achieves `≥18/20` unseen feasible bodies within 3 pp zero-shot, refuses `4/4` infeasible bodies via Farkas witness, reports both denominators (`KILL-TRANSFER-1`) |
| **`GATE-RANKCHECK-01`** | M5 | Model-form & simulator energy-creation audit (`B2`) catches 100% of planted diagonal-pyramid (`√2μ`) & penetration-trampoline exploits while passing genuine winners (`≤10%` campaign compute) |
| **`GATE-QUAL-01`** | M5 | Blinded Qualification Firewall (`BS-3`) keeps empirical campaign false-promotion rate `≤ α` under 500 adaptive AI retries on planted no-signal worlds while preserving power on planted real gains |
| **`GATE-LIFE-01`** | M5 | `frob-lifecycle` HMM (with rainflow residue stack `C17`) compresses 100-h reference `≥50×` at `≤5%` error & catches planted backlash-induced Floquet gait collapse (`KILL-LIFE-1`) |
| **`GATE-REDTEAM-01`** | M5 | CEGIS + `ddmin` (`FMT-MCW-1`) discovers planted $p \le 3\times 10^{-5}$ traps `≥3×` faster than uniform DR with `≥70%` candidate-body solvable, deterministic counterexamples (`KILL-REDTEAM-1`) |
| **`GATE-TIMETRAVEL-01`** | M5 | `2^10` Pearl-causal counterfactual replay on M5 Ultra UMA achieves `≥90%` top-1 planted-cause accuracy & cuts agent repair iterations `≥3×` (`KILL-TIMETRAVEL-1`) |
| **`GATE-AMEND-01`** | M5 | `frob-worldgen::amend` + `frob-redteam::amend` (`B3`, `10,700` kits) fixes `≥30%` of baseline SKU failures via ≤3-item kits reusing frozen policy & passes physical bench validation (`KILL-AMEND-1`) |
| **`GATE-REAL2SIM-01`** | M5 / M6 | Uncertainty-coupled `fdgr` pipeline localizes 100% of seeded occluded bottlenecks across 5 real-scan fixtures & emits actionable Re-Scan Requests |
| **`GATE-S2S-01`** | M6 | Spec-to-Silicon `.fbuild` packets pass `≥11/12` independent reviewer acceptance; unit-normalized STL robustness matches offline reference on 100% of traces; fresh AI agent solves `≥5/10` missions unattended (`KILL-S2S-1`) |
| **`GATE-FW-TWIN-01`** | M6 | `#![no_std]` firmware (`MotorBusHal`) passes `PO-CODEGEN-FW-1`, QEMU Cortex-M4/M7 parity (`≤1e-5` over 10k steps, 0 heap allocs), linked stack bound & real-board WCET receipt |
| **`GATE-WEBSTUDIO-01`** | M6 | `WebStudioScenePacket` lowers to `≤15 MB` offline HTML/WebGPU dossier verified on secure localhost context with interactive `2^10` multiverse ghost playback |
| **`GATE-RELEASE-01`** | M6 | End-to-end release gate: all 16 North-Stars (`NS-01`–`NS-16`), all `XT-P1`–`XT-P3`, all `KA-01`–`KA-14`, `frob-verify` offline audit, `50/51` `#![forbid(unsafe_code)]`, zero LOC-ceiling violations! |

### 26.3 Master Kill Gates & Pre-Declared Fallbacks (`KILL-*`)

| KILL ID | Trigger Condition | Pre-Declared Engineering Fallback (Zero Sunk-Cost Drift!) |
|---|---|---|
| **`KILL-FSYM-1`** | `frankensympy` fails `INT-FSYM-001` or leaks OOM/hangs on 29-DoF RNEA | Activate `SubprocessSymbolicBackend` in Layer `V::frob-io` (isolated timeout/RSS worker) + Rust-native PoE/RNEA generator |
| **`KILL-LGVI-1`** | Implicit Trapezoidal LGVI exceeds `2×` explicit Lie-Midpoint GPU step cost | Lock explicit symplectic Lie-Midpoint for GPU RL; keep Trapezoidal LGVI for CPU `f64` oracle & adjoint/eval passes |
| **`KILL-CONTACT-1`** | Unified Pfaffian+Contensou+backlash SSN fails convergence on `>1%` steps | Fall back per-island to Proximal ADMM / PGS with warm-started fixed iteration cap + explicit residual flag |
| **`KILL-GPU-1`** | `GATE-FORGE-01` misses roofline/throughput floors after profiling | Pivot hot loop to specialized 1-thread-per-env static ABI (`H2b`) & cut dynamic indirections |
| **`KILL-DET-1`** | Deterministic GPU scatter adds `>10%` step overhead OR float atomics already bitwise `≥99.9%` | If `>10%`: use deterministic scatter strictly for CRN/Shapley eval lanes; if already `≥99.9%`: drop scatter & keep `StateHash` gate |
| **`KILL-MORPHVM-1`** | `H2b` 9 per-family templates beat MorphoJIT on `XT-P2` (`≥95%` coverage & faster) | Promote `H2b` as primary RL engine; demote Tier-0 bytecode to instant Studio pre-compile preview |
| **`KILL-OPERAD-1`** | `XT-P1` `A2 < 5×` or misses `≥6/8` top-8 overlap or emits 1 false quotient | Restrict `.fquot` to exact symmetry/ballast transforms; keep `.fmodule` purely for derivation/proof caching |
| **`KILL-HOMOTOPY-1`** | `frob-homotopy` loses `>0.1%` paths or misses `4,326/40/16` counts | Fall back to `W06` (`real_rur` 3/5-pose) + multi-start CPU `frob-homotopy` recovery |
| **`KILL-PROOF-1`** | `FrobMath` reflection fails soundness or exceeds `5 s` on catalog bodies | Narrow reflection to `PO-INERTIA-1`, `PO-OPERAD-1`, `PO-ENERGY-1`, `PO-OBSALIAS-1`; emit `VerifiedNumeric` interval certificates for heavy polynomials |
| **`KILL-SHIELD-1`** | Box-constrained shield cuts return `<85%` or fails `XT-P3` bound | Retain `.fenergy` (`FTA-P3`) & `.ftube` (`W04`) hard certificates; demote high-DoF shield to soft Lagrangian penalty |
| **`KILL-TRIBO-1`** | 2.5D causal tribology exceeds `25%` RL overhead or changes `<20%` kitchen rankings | Move 2.5D film to Tier-B evaluation alongside `fs-lbm`/`ftx-drape`; keep 3-mode Housner slosh & Cosserat cords in Tier-A |
| **`KILL-FLEET-1`** | Dual-machine fleet speedup `<1.3×` AND 1,000-world bank gives zero generalization gain | Run `mac-studio` as standalone Studio/Diagnostics host and `trj` as standalone batch RL server (drop automated cross-LAN racing) |
| **`KILL-EQUIV-1`** | Equivariant RL shows `<1.3×` sample efficiency or `>3%` regression on cluttered interiors | Apply exact symmetry strictly to data augmentation & Burnside campaign deduplication; train unconstrained `GpuTensorView` MLPs |
| **`KILL-ATLAS-1`** | `frob-gaitatlas` fails W07 100-case gate or `<2×` RL warm-start speedup | Use `frob-gaitatlas` strictly for analytical stability diagnostics (`fs-spectral::floquet`) & warm-start shooting |
| **`KILL-TRANSFER-1`** | `.fskill` + `FiberMorph` fails `18/20` zero-shot or falsely admits 1 infeasible body | Retain exact `.fskill` WBC-QP + Farkas refusal witness; drop `FiberMorph` learned residual in favor of fast RL warm-starts |
| **`KILL-DPOR-1`** | DPOR misses 1 planted stale-read or watchdog/commit timing race | Fall back to bounded randomized schedule fuzzing + exhaustive permutation checks on critical ≤5-event control loops |
| **`KILL-LIFE-1`** | HMM compression `<50×` at `5%` error or misses planted backlash bifurcation | Use calibrated duty-cycle rainflow post-processing over 10-min representative rollouts |
| **`KILL-REDTEAM-1`** | CEGIS fails `≥3×` over uniform DR or `<70%` candidate-body solvable counterexamples | Keep `ddmin` shrinking (`FMT-MCW-1`) + legality priors on top of tuned cross-entropy DR |
| **`KILL-TIMETRAVEL-1`** | `2^10` counterfactual autopsy cuts agent repair iterations `<3×` | Ship single-factor paired `do(...)` rewind debugger (skip exhaustive 1,024-coalition Shapley sweep) |
| **`KILL-AMEND-1`** | ≤3-item home retrofit kits fix `<30%` of baseline SKU failures or fail bench test | Demote `amend` to advisory human checklist in `frob-report` |
| **`KILL-S2S-1`** | `<11/12` reviewer packet acceptance or fresh agent solves `<5/10` missions | Scope M6 export to BOM + STEP + STL specs; defer automated harness routing & firmware generation to v2.1 |

---

## 27. Pre-Mortem, Risk Register & Open Decisions

### 27.1 Pre-Mortem (If `franken_robotics` Failed 12 Months From Now, Why?)
1. **Nightly Toolchain & `fs-qty` Quagmire:** Trying to consume `fs-mbd`/`fs-contact` directly instead of lifting them off `fs-qty` (`F-03`) stalls M0 for 6 weeks → *Mitigated upfront by pre-classifying all `fs-qty` crates as Lift in `FD-02`!*
2. **Prover Scope Creep:** Trying to build general Lean 4 tactics (`fln-anvil::egraph/norm_num`) instead of small proved `FrobMath` reflection checkers (`F-04`) turns a robotics engine into an interactive theorem-prover rewrite → *Mitigated by `PO-*` scoping & `KILL-PROOF-1`!*
3. **Metal 3.2/4 Gateway Bottleneck:** Leaving `ft-kernel-metal` synchronous with `offset=0` (`F-06`) caps M5 Ultra throughput at 10% → *Mitigated by scheduling `BD-029a` explicitly in M1–M2 before M2 GPU convergence!*
4. **Silent Simulator-Artifact Exploits & AI P-Hacking:** Autonomous agents optimize $\sqrt{2}\mu$ friction corners and p-hack held-out worlds → *Mitigated structurally by `B1` (`determinism`), `B2` (`rankcheck` + discrete energy audit), and `BS-3` (Blinded $\alpha$-Spending Firewall)!*

### 27.2 Risk Register (`RISK-01`–`RISK-08`)

| RISK ID | Risk Description | Likelihood / Impact | Concrete Mitigation & Governing Gate |
|---|---|---|---|
| **`RISK-01`** | MorphoJIT Tier-0 register spill or branch divergence degrades throughput vs static templates | Med / High | Benchmark `XT-P2` directly against `H2b` (9 templates) & `W02` ragged queues; `KILL-MORPHVM-1` |
| **`RISK-02`** | `frankensympy` expression swell on 29-DoF humanoid symbolic Coriolis/ABA | Med / High | Hierarchical `.fmodule` subassembly CSE + `SubprocessSymbolicBackend` (`KILL-FSYM-1`) |
| **`RISK-03`** | `ft-kernel-metal v0.2` (`BD-029a`) or patched `block` crate breaks on macOS Darwin 27 SDK update | Med / High | Gate M0 via `INT-FTORCH-001`; track `objc2-metal` migration under `OPEN-METAL4-RT` |
| **`RISK-04`** | Metal lacks 64-bit integer atomics on target MSL revision for `B1` deterministic scatter | Med / Med | Probe in `GATE-FORGE-00`; default to portable stable-key sort + segmented reduction (`GATE-DET-01`) |
| **`RISK-05`** | `fastmcp_rust` (`08-25`) toolchain bump to `08-31` hits breaking nightly lint/feature | Low / Med | Verified Day 1 in `INT-FMCP-001`; pin patched local revision if needed |
| **`RISK-06`** | `franken_threed` planned GPU/compiler crates slip past M5 | High / Low | `frob-view` depends ONLY on live 4 crates + self-contained `WebStudioScenePacket` adapter (`F-07`) |
| **`RISK-07`** | Dual 4090 PCIe Gen4 P2P disabled (`cuDeviceCanAccessPeer == 0`) bottlenecks cross-card sync | High / Low | Zero per-substep cross-card traffic; independent shards + pinned host-staged gradient reduction (`F-02`) |
| **`RISK-08`** | 51-crate workspace compile times slow AI-agent inner edit loops | Med / Med | Strict downward layering (`frob-cx`), `sbh`/cargo target hygiene, per-crate `registries/loc_budget.toml` |

### 27.3 Open Decisions (`OPEN-*`, Every Entry Has Owner, Deadline & Fail-Closed Default)

| OPEN ID | Decision Question | Owner | Deadline | Fail-Closed Default |
|---|---|---|---|---|
| **`OPEN-METAL4-RT`** | Adopt Metal 4 residency sets / `MTL4*` via `objc2-metal` in M4+, or remain on hardened Metal 3.2 (`ft-kernel-metal v0.2`)? | FORGE Lead | M3 Close | Stay on hardened Metal 3.2 (`ft-kernel-metal v0.2`) |
| **`OPEN-LIC-01`** | Dual MIT/Apache-2.0 workspace license & third-party COTS supplier catalog license terms (`fs-io::supplier_corpus`) | Program Lead | M0 Close | Apache-2.0 / MIT; strip unverified proprietary CAD blobs |
| **`OPEN-PY-BRIDGE`** | Expose optional thin PyO3 / Gymnasium shim in M6, or keep pure Rust + `frob-mcp` + `.npz` only? | ATELIER Lead | M5 Close | Pure Rust + `frob-mcp` + `.npz` (zero Python runtime dep) |
| **`OPEN-IPC-GPU`** | Port `frob-contact::ipc` barrier solver to GPU in M5, or retain strictly as CPU/M5 Tier-B evaluator? | DYNAM Lead | M4 Close | Tier-B evaluator only (`GATE-CONTACT-03`) |

---

## 28. Milestone Roadmap (M0–M6) & Work-Package Architecture

### 28.1 Delivery Milestones (`M0`–`M6`, Convergence Before Breadth!)

| Milestone | Theme | Core Deliverables & Exit Gates |
|---|---|---|
| **M0** | **Constellation Admission, Hardware Fingerprint & Rival Baseline** | Workspace skeleton (`xtask check-layers/unsafe/loc/verbs`), all `INT-*` admission gates (`fs-qty` lifts verified!), `GATE-FORGE-00` silicon fingerprint, `GATE-RIVAL-AUDIT` pinned incumbent baselines |
| **M1** | **Algebraic Kernel, Reflective Proofs & `<200 ms` Preflight** | `frob-lie`/`bundle`/`symmetry`, `frob-symbolic` (`W06` linkages), `frob-kernel-ir`/`codegen`, `frob-proof` (`FrobMath` direct `fln-kernel` reflection), `frob-operad`, `frob-bot`/`spec`/`preflight` (Stages 1–7), `frob-dyn` LGVI + CPU `f64` oracle (`KA-01`–`KA-03`, `GATE-DYNAM-01`, `GATE-PROOF-01`, `GATE-OPERAD-01`, `GATE-BURMESTER-01`, `GATE-PREFLIGHT-01`) |
| **M2** | **Dual-Silicon Forge, `B1` Determinism, MorphoJIT & Studio Skeleton** | `BD-029a` (`ft-kernel-metal v0.2`), `DeviceArena` + `GpuTensorView`, `frob-gpu-metal` + `frob-gpu-cuda`, `B1` `determinism` (`GATE-DET-01`), `frob-contact` 3-Tier Delassus + `W10`, `frob-morphvm` + `XT-P2` (`GATE-MORPHVM-01`), `GATE-GPU-TWIN-01`, `GATE-FORGE-01`, `frob`/`frobd` CLI + async `frob-mcp` skeleton |
| **M3** | **Living Worlds, 6 Interior Proxies, Causal Tribology, Homotopy & Observation-Alias** | `frob-world`/`worldgen` (6 proxies + `.fclear` `GATE-CLEAR-01`), `frob-terramech` (`KA-08`), `frob-actuators` (`KA-09`), GVS Cosserat (`KA-05`), `frob-contact::ipc` (`GATE-CONTACT-03`), `frob-multiphys` conservative 2.5D tribology + Housner slosh (`KA-11`/`KA-12`, `GATE-TRIBO-01`), `frob-homotopy` (`GATE-HOMOTOPY-01`), Stage 8 `observation_conflict` (`GATE-OBSALIAS-01`) |
| **M4** | **Sensors/Sheaf/InEKF, Equivariant RL, Gait Atlas, Stratified Safety, DPOR & Breadth/Depth Fleet** | `frob-sensors` + $\mathfrak{se}(3)$ `sheaf` (`KA-10`), `frob-estimate` InEKF (`KA-06`), `frob-equiv` + `frob-gym` `GpuTensorView` RL (`GATE-EQUIV-01`), `frob-gaitatlas` (`KA-07`, `GATE-ATLAS-01`), `frob-shield` (`.fenergy` `KA-14`, `.ftube`, box-constrained `ClaimProduct`, `XT-P3`, `GATE-SHIELD-01`), `BS-2` DPOR (`GATE-DPOR-01`), `frob-gpu::fleet` (`GATE-FLEET-01`), `XT-P1`, `frob-tui` + `NativePresenter` |
| **M5** | **10× Co-Design Cascade, `.fskill` Transfer, CEGIS + `2^10` Time-Travel, `B2`/`B3`/`BS-3` & Lifecycle** | `frob-control` `.fskill` + `FiberMorph` (`GATE-TRANSFER-01`), `frob-codesign` + `B2` `rankcheck` (`GATE-RANKCHECK-01`), `BS-3` Blinded Qualification (`GATE-QUAL-01`), `frob-lifecycle` HMM (`GATE-LIFE-01`), `frob-redteam` CEGIS + `B3` `amend` (`GATE-REDTEAM-01`, `GATE-AMEND-01`), `frob-diagnose` `2^10` Time-Travel (`GATE-TIMETRAVEL-01`), Tier-B `fs-lbm`/`ftx-drape` (`GATE-MULTI-01`), full 18 archetypes & rich 6 interiors |
| **M6** | **Spec-to-Silicon `.fbuild`, `#![no_std]` Firmware, `fdgr` Real-to-Sim, WebGPU Dossier & v2.0 Release** | `frob-mfg` `.fbuild` CAD/BOM/GD&T/Steiner harness (`GATE-S2S-01`), `#![no_std]` QEMU+board parity (`GATE-FW-TWIN-01`), `fdgr` uncertainty loop (`GATE-REAL2SIM-01`), `franken_threed` offline WebGPU dossier (`GATE-WEBSTUDIO-01`), `frob-report`, `frob-verify` full offline audit, **`GATE-RELEASE-01`**! |

### 28.2 Work-Package Architecture (`WP-01`–`WP-12`)
- `WP-01` Admission, Workspace & Rival Baselines (M0) · `WP-02` AXIOM Runtime, Ledger, DPOR & Qualification Firewall (M0–M5) · `WP-03` LIE, Bundles & Symmetry (M1) · `WP-04` CAS, Homotopy, KIR & 4-Target Codegen (M1–M3) · `WP-05` Reflective `fln-kernel` Proofs & Stratified Safety (M1–M4) · `WP-06` Operads, `.fbot`/`.ftask`, Actuators & 8-Stage Preflight (M1–M3) · `WP-07` 3-Tier `WorldScene`, 6 Interiors, Terramech, `.fclear` & `B3` Retrofits (M2–M5) · `WP-08` LGVI, 4D Contact/IPC, MorphoJIT, Causal Tribology & Adjoints (M1–M5) · `WP-09` Dual-Silicon Forge, `BD-029a`, `B1` Determinism & Breadth/Depth Fleet (M1–M4) · `WP-10` GPU Sensors, Sheaf Calibration & InEKF (M4) · `WP-11` Equivariant RL, Gait Atlas, `.fskill`, Co-Design + `B2` Rankcheck & HMM Lifecycle (M4–M5) · `WP-12` CEGIS, `2^10` Time-Travel, `frob-mfg`/Firmware, `fdgr`, ATELIER & Release (M2–M6).


---

## 29. Complete Acyclic Zero-Orphan Execution Beads DAG (`BD-001`–`BD-096` + `BD-029a`)

### 29.1 Per-Bead Verification & Structured NDJSON Logging Contract
Every bead below is **incomplete** until:
1. All unit, property, differential, and named `KA-*`/`GATE-*`/`XT-*` tests pass under `cargo test` with zero warnings and zero `xtask check-{layers,unsafe,loc,verbs}` regressions.
2. Every run emits a deterministic **structured NDJSON execution log** (`docs/delivery/logs/<BD-ID>.ndjson`) containing `{ "ts", "bead_id", "commit_sha", "host_fingerprint", "determinism_class", "state_hash", "claim_axis", "metrics", "refusals", "verdict" }`.
3. Any bead closing a `GATE-*` or `XT-*` writes its signed receipt into `frob-ledger` verifiable offline by `frob-verify`.

### 29.2 Complete Beads Table (97 Beads: `BD-001`–`BD-096` + `BD-029a`, Sole Terminal Sink = `BD-096`)

| Bead | Milestone / WP | Crates / Scope | Depends On | Exit Criterion / Governing Gate |
|---|---|---|---|---|
| **`BD-001`** | M0 / `WP-01` | Root `Cargo.toml` (`08-31`, `asupersync =0.6.2`, `[patch] block`), `xtask`, `registries/` | — | `xtask check-{layers,unsafe,loc,verbs}` pass on empty workspace |
| **`BD-002`** | M0 / `WP-01` | `frankensim` `fs-qty`-free consumes + provenance lifts (`FD-01`/`FD-02`) | `BD-001` | `INT-FSIM-001` |
| **`BD-003`** | M0 / `WP-01` | `franken_lean` + `frankensympy` admission (`FD-03`/`FD-04`) | `BD-001` | `INT-FLEAN-001`, `INT-FSYM-001` |
| **`BD-004`** | M0 / `WP-01` | `frankentorch` (`ft-kernel-metal`) + `franken_numpy` + `franken_threed` (`FD-05`–`FD-07`, `FD-15`) | `BD-001` | `INT-FTORCH-001` (M0), `INT-FNP-001`, `INT-F3D-001` |
| **`BD-005`** | M0 / `WP-01` | `asupersync`, `frankensqlite`, `fastmcp_rust` (`08-31`), `frankentui`, `fmd` (`FD-08`–`FD-11`, `FD-16`) | `BD-001` | `INT-ASUP-001`, `INT-FSQL-001`, `INT-FMCP-001`, `INT-FTUI-001` |
| **`BD-006`** | M0 / `WP-01` | Silicon fingerprint (`mac-studio` M5 Ultra GB/s, working set, 64-bit atomic probe; `trj` Dual 4090) | `BD-001` | `GATE-FORGE-00` |
| **`BD-007`** | M0 / `WP-01` | Incumbent documentation & baseline freeze (MuJoCo/MJWarp, Isaac Lab/Newton, Drake, Genesis Metal) | `BD-001` | `GATE-RIVAL-AUDIT` |
| **`BD-008`** | M0 / `WP-02` | `frob-core`, `frob-units`, `frob-evidence` (`ClaimProduct`), `frob-cx` | `BD-002`, `BD-005` | Compile-fail test proves `ClaimProduct` cannot convert to `VerifiedFormal` (`INV-001`) |
| **`BD-009`** | M0 / `WP-02` | `frob-runtime` (`asupersync` scopes) & `frob-ledger` (Merkle SQLite ledger) | `BD-008`, `BD-005` | `LabRuntime` cancel + WAL crash-recovery tests pass (`INV-009`) |
| **`BD-010`** | M0 / `WP-12` | `frob-telemetry` (6-axis `.npz`) & `frob-bench` oracle/thermal harness | `BD-008`, `BD-004` | `.npz` round-trip & F3D §16 paired-block harness green |
| **`BD-011`** | M0 / `WP-12` | `frob-io` core (fuzzed URDF/MJCF/glTF/USD parsers + `SubprocessSymbolicBackend`) | `BD-008`, `BD-003` | Round-trip + fuzz smoke tests pass (`KILL-FSYM-1` ready) |
| **`BD-012`** | M0 / `WP-01` | **M0 Milestone Convergence Gate** | `BD-002`, `BD-003`, `BD-004`, `BD-005`, `BD-006`, `BD-007`, `BD-008`, `BD-009`, `BD-010`, `BD-011` | All M0 `INT-*`, `GATE-FORGE-00`, `GATE-RIVAL-AUDIT` recorded |
| **`BD-013`** | M1 / `WP-03` | `frob-lie` (§6.0 conventions, PGA/CGA `fs-ga`, `AffineRows` 48 B, $\mathrm{Sym}^+(4)$, `Dual`) | `BD-012` | `KA-01` passes to `1e-13`; small-angle `sinc`/`cosc` verified |
| **`BD-014`** | M1 / `WP-03` | `frob-bundle` (principal bundles, $\mathbf{A}_S$, corrected minus-sign Cartan curvature $D\mathbf{A}_S$) | `BD-013` | `KA-03` passes (`err < 1e-3`; wrong plus sign rejected!) |
| **`BD-015`** | M1 / `WP-03` | `frob-symmetry` ($\Gamma_{\mathrm{mech}}/\Gamma_{\mathrm{obs}}$, $\mathbb{R}/\mathbb{C}/\mathbb{H}$, wreath, H/K, exact Burnside `86,185`) | `BD-013` | Exact `86,185` ($D_6$) & `500,500` ($\mathbb{Z}_2$) + $C_n$ complex-irrep tests pass |
| **`BD-016`** | M1 / `WP-04` | `frob-symbolic` core (PoE/RNEA/ABA, Echeandia–Wensing universally skew Coriolis) | `BD-013`, `BD-011` | Symbolic $\dot{M} - 2C$ skew identity matches `Dual` gradcheck |
| **`BD-017`** | M1 / `WP-04` | `frob-symbolic::linkage_synth` (`W06` 3-pose rational + `real_rur` 5-pose Burmester) | `BD-016` | `GATE-BURMESTER-01` (`KA-13` M1 slice) |
| **`BD-018`** | M1 / `WP-04` | `frob-kernel-ir` (`FMT-KIR-1` typed SSA IR, CSE, register scheduler, congruence trace) | `BD-016` | Schema + register-bound + zero-alloc checks pass |
| **`BD-019`** | M1 / `WP-04` | `frob-codegen` CPU Rust, Metal MSL (`MathMode::Safe`), CUDA PTX lowering | `BD-018` | Generated CPU/MSL/PTX compile clean + emit `PO-CODEGEN-1` trace |
| **`BD-020`** | M1 / `WP-05` | `frob-proof` (`FrobMath` direct `fln-kernel` reflection library + `omega` + `PO-LIE-1`, `PO-INERTIA-1`, `PO-SKEW-1`, `PO-CODEGEN-1`) | `BD-013`, `BD-015`, `BD-016`, `BD-019` | Zero stubbed `fln-anvil` calls (`INV-011`); `<5 s` checks |
| **`BD-021`** | M1 / `WP-05` | `frob-verify` (`std`-only offline `.frobcert` auditor + `fln-checker` replay) | `BD-020`, `BD-009` | `GATE-PROOF-01` (offline zero-`sorry` replay green) |
| **`BD-022`** | M1 / `WP-06` | `frob-operad` (`.fmodule` decorated cospans `W01`, Limb Port-ABI `SymPsd6`, `.fquot` ballast `FTA-P1`) | `BD-015`, `BD-020` | `GATE-OPERAD-01` (`735,471 → 24,546` verified; shared-bus invalidation green) |
| **`BD-023`** | M1 / `WP-06` | `frob-bot` (`FMT-FBOT-1` grammar/macros/hollow shells, 5 Phase-A archetypes, semantic `diff`) | `BD-022`, `BD-014` | All 8 v1 Appendix D defects caught & fixed by parser tests |
| **`BD-024`** | M1 / `WP-06` | `frob-spec` (`FMT-FTASK-1` unit-normalized STL IR, Lemire kernels, `ObsLayout`/`ActLayout`, reset cache) | `BD-023` | Past-time online & delayed future-time STL match reference 100% |
| **`BD-025`** | M1 / `WP-06` | `frob-preflight` Stages 1–7 (`<200 ms`, convex-GP/directional boxes, no invalid `C007`) | `BD-014`, `BD-017`, `BD-023`, `BD-024` | `GATE-PREFLIGHT-01` (`≥90%` of 100 seeded defects caught `<200 ms`) |
| **`BD-026`** | M1 / `WP-08` | `frob-dyn` CPU `f64` oracle (Trapezoidal-Potential LGVI + Lie-Midpoint + DLA Pfaffian wheels) | `BD-013`, `BD-019`, `BD-023` | `KA-02`, `GATE-DYNAM-01` (`<1e-6` energy error at `0.25 ms`, exact reversibility) |
| **`BD-027`** | M1 / `WP-08` | `frob-contact` CPU `f64` oracle (4D Coulomb–Contensou NCP, joint limits/stiction/backlash, 3-Tier Delassus) | `BD-026`, `BD-022` | `KA-04` CPU oracle green; zero pyramidal $\sqrt{2}\mu$ bias |
| **`BD-028`** | M1 / `WP-01` | **M1 Milestone Convergence Gate** | `BD-021`, `BD-025`, `BD-027` | All M1 gates green (`NS-01`/`NS-03` M1 slices green) |
| **`BD-029a`** | M2 / `WP-09` | **Upstream `ft-kernel-metal v0.2`** (zero-copy slices, `PrivateBuffer`, offsets, concurrent encoders, async `SubmitHandle`) | `BD-012` | `INT-FTORCH-001` (M2); zero `rayon` in GPU closure |
| **`BD-029`** | M2 / `WP-09` | `frob-gpu` core (`DeviceArena` 256-B slabs, `SlabLease`, `GpuTensorView`, CAS CUBIN/`MTLBinaryArchive` cache) | `BD-028`, `BD-029a` | Zero per-tick allocations; epoch-fenced CPU/GPU lease tests pass |
| **`BD-030`** | M2 / `WP-09` | `frob-gpu-metal` (`#![forbid(unsafe_code)]`, Metal 3.2 compute + `simdgroup_matrix` + `MathMode::Safe` `TwoSum` canary) | `BD-029`, `BD-029a`, `BD-019` | `FROB-REF-402` canary passes on M5 Ultra; zero `unsafe` |
| **`BD-031`** | M2 / `WP-09` | `frob-gpu-cuda` (sole `<600` LOC `unsafe` leaf + `SAFETY.md`, `SM_89` pools, 1-thread/GPU, WMMA, host-staged dual sync) | `BD-029`, `BD-019` | Zero leaked `cuMemAllocAsync` pools on cancel; dual-4090 sync green |
| **`BD-032`** | M2 / `WP-09` | `frob-gpu::determinism` (**`B1` Order-Independent GPU Scatter & `StateHash`**, `DeterminismClass` gate) | `BD-030`, `BD-031` | `GATE-DET-01` (100% `BitwisePerDevice` across 1k windows, `≤10%` overhead) |
| **`BD-033`** | M2 / `WP-08` | `frob-contact` GPU 3-Tier Delassus (`1.86/4.70 KB`/matrix-free) + **`W10` Factor-Once Multi-RHS** | `BD-030`, `BD-031`, `BD-027`, `BD-032` | `GATE-CONTACT-02`, `GATE-GPU-TWIN-01` (`f64` ↔ Metal ↔ CUDA parity) |
| **`BD-034`** | M2 / `WP-08` | `frob-morphvm` (Tier-0 screw bytecode, Tier-0.5 Port-ABI, `W02` ragged islands, `TA-P2` race annealing $B\ge 65\text{k}$) | `BD-033`, `BD-022`, `BD-032` | Dispatch-boundary promotion + `(world, replicate_id)` CRN tests pass |
| **`BD-035`** | M2 / `WP-08` | `XT-P2` 4-arm hardware benchmark (`H512` M5 Ultra & Dual 4090: MorphoJIT vs `W02` vs warm Graphs vs `H2b`) | `BD-034`, `BD-007` | `GATE-MORPHVM-01`, `GATE-FORGE-01` (20-min sustained thermal floor) |
| **`BD-036`** | M2 / `WP-07` | `frob-world` core (single-authority 3-Tier GPU `WorldScene`, `StorageModePrivate` VDB/BVH, fixtures/doors/conveyors) | `BD-029`, `BD-024` | Single-authority physics/ray parity verified (`INV-005`) |
| **`BD-037`** | M2 / `WP-05` | `frob-shield` M2 slice (**`COD FTA-P3` `.fenergy`** observable boundary energy budget + `PO-ENERGY-1`) | `BD-020`, `BD-034`, `BD-022` | `KA-14` (`1.49 J ≤ 2.00 J`, `50.5/45.5 ms`, 0 violations for any $\mu\in[0,5]$) |
| **`BD-038`** | M2 / `WP-12` | `frob-cli` (`frob --json` `FMT-ENV-1` + **`frobd` resident daemon `FMT-RPC-1`** from `verbs.toml`) | `BD-023`, `BD-024`, `BD-035`, `BD-036`, `BD-037`, `BD-009` | `xtask check-verbs` green; warm `frobd` round-trip `<5 ms` |
| **`BD-039`** | M2 / `WP-12` | `frob-mcp` async server (`fastmcp_rust`, `sync_wait_ms`/`job_id`/`job_poll`/`job_cancel`, Phase-A tools & `resources://`) | `BD-038`, `BD-005` | Async job cancel + budget cap + schema parity tests pass |
| **`BD-040`** | M2 / `WP-01` | **M2 Milestone Convergence Gate** | `BD-035`, `BD-038`, `BD-039`, `BD-032`, `BD-033` | `GATE-GPU-TWIN-01`, `GATE-FORGE-01`, `GATE-DET-01`, `GATE-MORPHVM-01` green |
| **`BD-041`** | M3 / `WP-07` | `frob-worldgen` M3 lightweight structural proxies for all **6 interior archetypes** | `BD-040`, `BD-036` | All 6 interior proxies spawn cleanly on Metal & CUDA |
| **`BD-042`** | M3 / `WP-07` | `frob-worldgen::clearance_complex` (**`COD W08` `.fclear`** dual $SE(2)$, `74 mm` margin + `MorphologyNavEnvelope`) | `BD-041` | `GATE-CLEAR-01` (0 false routes, 0 false impossibility cuts) |
| **`BD-043`** | M3 / `WP-07` | `frob-terramech` (outdoor multi-layer SDF overhangs + Bekker–Wong/Janosi–Hanamoto + tracked belts) | `BD-040`, `BD-036` | `KA-08` (`3.52/9.79/4.96 cm` sinkage matched `1e-5`, `NS-10`) |
| **`BD-044`** | M3 / `WP-06` | `frob-actuators` (extensive-entropy pHS PMSM/gearbox, 2-RC battery, aerial rotor port) | `BD-040`, `BD-022` | `KA-09`, `GATE-ACT-01` (`33.5 s` cliff within `0.1 s`, $\sigma_{\mathrm{irr}}\ge 0$) |
| **`BD-045`** | M3 / `WP-08` | `frob-dyn` GVS Cosserat rods (`fs-solid::rod`) + 6 M3 archetypes (`swerve`, `tracked-*`, `wheeled-legged`, `biped`, `delta`) | `BD-040`, `BD-026` | `KA-05`, `GATE-COSSERAT-01` (`≤0.5%` elastica match) |
| **`BD-046`** | M3 / `WP-08` | `frob-contact::ipc` barrier + CCD lane + `INT-FTX-001` `ftx-drape` admission | `BD-045`, `BD-033`, `BD-002` | `GATE-CONTACT-03`, `INT-FTX-001` (0 tunneling on cords/fragile SKUs) |
| **`BD-047`** | M3 / `WP-08` | `frob-multiphys` Tier-A conservative 2.5D causal tribology (`CC-08` fixed) + 3-mode Housner slosh | `BD-041`, `BD-045`, `BD-046` | `KA-11`, `KA-12`, `GATE-TRIBO-01` (`±2%` Huppert, mass drift `<1e-6`, `≤15%` RL overhead) |
| **`BD-048`** | M3 / `WP-04` | `frob-homotopy` (**`CC-04`** GPU polyhedral/parameter homotopy + Smale $\alpha$-cert `PO-ALPHA-1` + trace test) | `BD-040`, `BD-017`, `BD-020` | `KA-13`, `GATE-HOMOTOPY-01` (`4,326`/`40`/`16`, `≤2 s` re-query, `NS-03`) |
| **`BD-049`** | M3 / `WP-06` | `frob-preflight::observation_conflict` (**Stage 8 `BS-1` Observation-Alias Witness**, `PO-OBSALIAS-1`) | `BD-042`, `BD-044`, `BD-020`, `BD-025` | `GATE-OBSALIAS-01` (0 false witnesses on negative controls, `NS-01`) |
| **`BD-050`** | M3 / `WP-08` | `frob-adjoint` (symplectic discrete adjoints, autonomous saltation matrices, `W10` multi-RHS sweeps) | `BD-040`, `BD-033` | Matches `Dual` & central FD within `1e-5` on smooth/impact transitions |
| **`BD-051`** | M3 / `WP-12` | `frob-redteam` M3 core (OSHA/friction legality priors, candidate-body solvability, `ddmin` `FMT-MCW-1`) | `BD-041`, `BD-047`, `BD-042`, `BD-049` | Deterministic `≤3`-edit `FMT-MCW-1` reproducer verified via `StateHash` |
| **`BD-052`** | M3 / `WP-01` | **M3 Milestone Convergence Gate** | `BD-042`, `BD-043`, `BD-044`, `BD-046`, `BD-047`, `BD-048`, `BD-049`, `BD-050`, `BD-051` | All M3 gates green (`NS-01`, `NS-03`, `NS-10` green) |
| **`BD-053`** | M4 / `WP-10` | `frob-sensors` GPU ray-traced LiDAR/RGB-D/stereo/event (glass, steam, skew) + tactile patches | `BD-052` | Glass multipath, steam dropout, and rolling-shutter fixtures verified |
| **`BD-054`** | M4 / `WP-10` | `frob-sensors::sheaf` (linearized $\mathfrak{se}(3)$ cellular-sheaf extrinsic calibration + $H^1$ localization) | `BD-053`, `BD-013` | `KA-10` (100% localization of planted `2° / 15 mm` bumped LiDAR bracket) |
| **`BD-055`** | M4 / `WP-10` | `frob-estimate` (Imperfect Right-Invariant $SE_{2+m}(3)$ EKF + observability Gramian $\sigma_{\min}(\mathcal{O})$) | `BD-053`, `BD-054`, `BD-013` | `KA-06` (`≤2%` NEES error under `45°` yaw error; catches corridor aliasing) |
| **`BD-056`** | M4 / `WP-11` | `frob-equiv` ($\mathbb{R}/\mathbb{C}/\mathbb{H}$, regular-rep, wreath, `PO-EQUIV-1`) + 5 M4 archetypes (`hexapod`, `octopod`, `humanoid-g1`, `wheeled-humanoid`, `cosserat-tentacle`) | `BD-052`, `BD-015` | Equivariance error `<1e-6` across $V_4, D_6, C_6, D_8$ & wreath architectures |
| **`BD-057`** | M4 / `WP-11` | `frob-gym` zero-copy `GpuTensorView` `VectorEnv` + PPO/SAC/ES on Metal `simdgroup_matrix` & CUDA WMMA | `BD-056`, `BD-055`, `BD-029` | `GATE-EQUIV-01` (`≥1.8×` 10-seed sample efficiency, 0 CPU `DenseTensor` copies, `NS-05`) |
| **`BD-058`** | M4 / `WP-11` | `frob-gaitatlas` (H/K `PO-GAITENUM-1`, twisted shooting, full-space `fs-spectral::floquet`, W07 saltation + Envelope Thm) | `BD-052`, `BD-050`, `BD-015`, `BD-020` | `KA-07`, `GATE-ATLAS-01` (20/20 grazing refused, `≥72/80` `cos≥0.95`, `NS-08`) |
| **`BD-059`** | M4 / `WP-05` | `frob-shield` full 3-tier stratified safety (`.fenergy` + `.ftube` `PO-TUBE-1` + box-constrained `ClaimProduct` `27` strata `PO-SHIELD-1`) | `BD-052`, `BD-037`, `BD-055`, `BD-020` | `XT-P3`, `GATE-SHIELD-01` (0 box violations, `≤5%` overhead, `NS-06`) |
| **`BD-060`** | M4 / `WP-09` | `frob-gpu::fleet` Breadth/Depth dual-silicon orchestrator (M5 500–1,000 worlds + Dual 4090 + *seed-not-voxels* + `fs-tropical::max_cycle_mean`) | `BD-052`, `BD-036`, `BD-009` | `GATE-FLEET-01` (`≤1 GB/campaign-hr`, `≥1.5×` speedup, `NS-16`) |
| **`BD-061`** | M4 / `WP-11` | `XT-P1` 4-arm campaign-quotienting exclusion test (128 morphologies + ballast fixture + blinded discard audit) | `BD-057`, `BD-058`, `BD-059`, `BD-060` | `XT-P1` (`A2≥5×`, `A3≥10×` 95% LCB, `≥6/8` overlap, planted winner #1) |
| **`BD-062`** | M4 / `WP-04` | `frob-codegen` `#![no_std]` embedded Rust lowering (`MotorBusHal`, `Q32.32`/IEEE, `PO-CODEGEN-FW-1`, QEMU harness) | `BD-052`, `BD-019`, `BD-020` | Compiles clean for `thumbv7em-none-eabihf` with 0 heap allocs |
| **`BD-063`** | M4 / `WP-12` | `frob-tui` (`frankentui` live cockpit: race brackets, silicon utilization, thermal bars, $\alpha_i$ meter) | `BD-057`, `BD-060`, `BD-038`, `BD-005` | Non-interactive VHS/snapshot regression suite passes |
| **`BD-064`** | M4 / `WP-12` | `frob-view` `NativePresenter` (M5 Ultra Metal 60+ FPS viewport + contact/tribology/`.fclear` overlays) | `BD-052`, `BD-036`, `BD-004` | Sustained 60+ FPS on 6 interior scenes with zero sim stalls |
| **`BD-065`** | M4 / `WP-02` | `frob-runtime::schedule_check` (**`BS-2` DPOR Timing Verification** over `asupersync::LabRuntime`) | `BD-052`, `BD-009`, `BD-055`, `BD-062` | `GATE-DPOR-01` (`720 → 8 = 90×`, 100% planted stale-read/watchdog races caught) |
| **`BD-066`** | M4 / `WP-01` | **M4 Milestone Convergence Gate** | `BD-054`, `BD-055`, `BD-057`, `BD-058`, `BD-059`, `BD-060`, `BD-061`, `BD-062`, `BD-063`, `BD-064`, `BD-065` | All M4 gates + `XT-P1`/`XT-P3` green (`NS-05`/`06`/`08`/`16` green) |
| **`BD-067`** | M5 / `WP-11` | `frob-control` (`.fskill` `W03` + WBC-QP + Farkas witness + `FiberMorph` residual + IDA-PBC + Lie-DDP) | `BD-066`, `BD-056`, `BD-057` | `GATE-TRANSFER-01` (`≥18/20` feasible zero-shot, `4/4` infeasible refused, `NS-09`) |
| **`BD-068`** | M5 / `WP-06` | Final 2 hybrid archetypes (`mobile-dual-arm`, `aerial-terrestrial-quad`) — **18/18 catalog complete!** | `BD-066`, `BD-045` | All 18 archetypes pass preflight, LGVI, and GPU twin parity |
| **`BD-069`** | M5 / `WP-07` | Rich procedural/asset variations across all **6 interior archetypes** (homes, offices, warehouses, plants, retail, kitchens) | `BD-066`, `BD-041` | 500–1,000 resident worlds verified inside M5 `recommendedMaxWorkingSetSize` |
| **`BD-070`** | M5 / `WP-08` | `frob-multiphys` Tier-B zoom-lens (`fs-lbm` `Ma<0.20`, `≤64` envs M5 + `ftx-drape` IPC + `W05` far-field tiles) | `BD-066`, `BD-046`, `BD-047`, `BD-069` | `GATE-MULTI-01` (Tier-A/B rank agreement + discrete port work-energy residual) |
| **`BD-071`** | M5 / `WP-11` | `frob-bench::model_form` (**`B2` Model-Form Robustness & Simulator Discrete Energy-Creation Audit**) | `BD-066`, `BD-033`, `BD-046`, `BD-010` | Catches 100% of planted $\sqrt{2}\mu$ pyramid & penetration-trampoline exploits |
| **`BD-072`** | M5 / `WP-02` | `frob-ledger::qualification` (**`BS-3` Blinded $\alpha$-Spending Qualification Firewall**, $\alpha_i = \alpha/[i(i+1)]$) | `BD-066`, `BD-009` | `GATE-QUAL-01` (false-promotion rate `≤ α` under 500 adaptive retries) |
| **`BD-073`** | M5 / `WP-11` | `frob-codesign` 6-stage 10× cascade + `rankcheck` (`ModelRobust` vs `ModelConditional`) | `BD-067`, `BD-068`, `BD-071`, `BD-072`, `BD-061` | `GATE-RANKCHECK-01` (`≤10%` overhead, blocks `ModelConditional` from BOM spend) |
| **`BD-074`** | M5 / `WP-11` | `frob-lifecycle` E–Engquist HMM 1,000-h compression (rainflow residue stack `C17`, backlash bifurcation fallback) | `BD-066`, `BD-058`, `BD-044`, `BD-027` | `GATE-LIFE-01` (`≥50×` at `≤5%` error, catches planted backlash collapse, `NS-13`) |
| **`BD-075`** | M5 / `WP-12` | `frob-redteam` full CEGIS + `fs-voi` + `ddmin` `FMT-MCW-1` | `BD-066`, `BD-051`, `BD-069`, `BD-070` | `GATE-REDTEAM-01` (`≥3×` faster than DR, `≥70%` candidate-body solvable) |
| **`BD-076`** | M5 / `WP-07` | `frob-worldgen::amend` + `frob-redteam::amend` (**`B3` Robot–Home Retrofit Co-Design**, 40-item catalog, `10,700` kits) | `BD-075`, `BD-042`, `BD-069` | `GATE-AMEND-01` (`≥30%` baseline failures fixed via ≤3-item kits + bench check) |
| **`BD-077`** | M5 / `WP-12` | `frob-diagnose` (12 `FAIL-*` + **`2^10` Pearl-Causal Counterfactual Time-Travel** + exact Shapley + DRO) | `BD-066`, `BD-032`, `BD-075`, `BD-076` | `GATE-TIMETRAVEL-01` (`≥90%` top-1 accuracy, `≥3×` fewer agent iterations, `NS-11`) |
| **`BD-078`** | M5 / `WP-12` | Full 42-tool `frob-mcp` studio surface + live `frob-tui` integration (co-design, time-travel, retrofits, qualification) | `BD-073`, `BD-074`, `BD-075`, `BD-076`, `BD-077`, `BD-039` | All 42 tools & `resources://` pass async contract & schema parity |
| **`BD-079`** | M5 / `WP-01` | **M5 Milestone Convergence Gate** | `BD-067`, `BD-068`, `BD-069`, `BD-070`, `BD-073`, `BD-074`, `BD-076`, `BD-077`, `BD-078` | All M5 gates green (`NS-02`, `04`, `07`, `09`, `11`, `13` green) |
| **`BD-080`** | M6 / `WP-12` | `INT-FDGR-001` & `frob-io` uncertainty-coupled `fdgr` real-scan ingest + Re-Scan Request emitter | `BD-079`, `BD-011` | `INT-FDGR-001`; unprobed surfaces tagged `UnprobedConservative` |
| **`BD-081`** | M6 / `WP-12` | End-to-end `fdgr` real-to-sim-to-retrofit-to-design validation across 5 real-scan fixtures | `BD-080`, `BD-075`, `BD-076` | `GATE-REAL2SIM-01` (100% seeded occluded bottlenecks localized, `NS-12`) |
| **`BD-082`** | M6 / `WP-06` | `frob-mfg` COTS catalog join (generalizable-cut rule) + FreeCAD-validated faceted STEP CAD (`fs-geocon`/`fs-fab`) | `BD-079`, `BD-002` | Post-snap revalidation & STEP geometry import checks pass |
| **`BD-083`** | M6 / `WP-06` | `frob-mfg` adjoint GD&T stackup (`fs-toleralloc`) + Steiner/GVS Cosserat wiring harness + thermal/electrical budgets | `BD-082` | Zero harness bend-radius, joint-sweep collision, or ampacity violations |
| **`BD-084`** | M6 / `WP-04` | `#![no_std]` firmware crate packaging + QEMU Cortex-M4/M7 parity (`≤1e-5`, 0 allocs) + linked stack/board WCET | `BD-062`, `BD-083`, `BD-065` | `GATE-FW-TWIN-01` (`hardware_safety_signoff: false` enforced!) |
| **`BD-085`** | M6 / `WP-12` | Atomic `.fbuild` transaction + 12-brief independent reviewer acceptance + unattended fresh-agent mission suite | `BD-083`, `BD-084`, `BD-078` | `GATE-S2S-01` (`≥11/12` packets accepted, `≥5/10` fresh-agent missions, `NS-15`) |
| **`BD-086`** | M6 / `WP-12` | `frob-view` `WebStudioScenePacket` (`FMT-WSP-1`) adapter on `franken_threed @ bf7abe1` → `≤15 MB` offline HTML/WebGPU dossier | `BD-079`, `BD-064`, `BD-077`, `BD-083`, `BD-004` | `GATE-WEBSTUDIO-01` (verified on secure localhost WebGPU context) |
| **`BD-087`** | M6 / `WP-12` | `frob-report` (`franken_markdown` + `frankenmermaid` 5-axis evidence dossiers & Galaxy-Brain cards) | `BD-079`, `BD-085`, `BD-086`, `BD-005` | Deterministic dossier build + zero claim-axis violations |
| **`BD-088`** | M6 / `WP-05` | `frob-verify` full offline `.frobcert` (`FMT-CERT-1`) audit suite (zero GPU/network, `fln-checker` + Merkle + $\alpha$-spend) | `BD-079`, `BD-021`, `BD-072`, `BD-085` | `NS-14` offline audit passes on fresh air-gapped container |
| **`BD-089`** | M6 / `WP-12` | 10M+ `cargo-fuzz` security/interop hardening (`frob-io`/`bot`/`spec`/`mcp`/`verify`) + final `xtask` compliance audit | `BD-081`, `BD-085`, `BD-086`, `BD-087`, `BD-088` | Zero fuzz panics; `50/51` `forbid(unsafe)`; all 51 crates ≤ 1.15× LOC ceiling |
| **`BD-090`** | M6 / `WP-12` | North-Star End-to-End Pack A (`NS-01`–`NS-04`) | `BD-089` | `NS-01`, `NS-02`, `NS-03`, `NS-04` PASS receipts in Ledger |
| **`BD-091`** | M6 / `WP-12` | North-Star End-to-End Pack B (`NS-05`–`NS-08`) | `BD-089` | `NS-05`, `NS-06`, `NS-07`, `NS-08` PASS receipts in Ledger |
| **`BD-092`** | M6 / `WP-12` | North-Star End-to-End Pack C (`NS-09`–`NS-12`) | `BD-089` | `NS-09`, `NS-10`, `NS-11`, `NS-12` PASS receipts in Ledger |
| **`BD-093`** | M6 / `WP-12` | North-Star End-to-End Pack D (`NS-13`–`NS-16`) | `BD-089` | `NS-13`, `NS-14`, `NS-15`, `NS-16` PASS receipts in Ledger |
| **`BD-094`** | M6 / `WP-12` | Final Paired Incumbent Benchmark Publication (`n>=10`, 95% LCB, 20-min thermal floor vs `GATE-RIVAL-AUDIT`) | `BD-090`, `BD-091`, `BD-092`, `BD-093`, `BD-007` | All published speedups carry hardware-pinned paired receipts |
| **`BD-095`** | M6 / `WP-12` | Complete 51-Crate `CONTRACT.md` Compliance & Reproducibility Artifact Archive (`FMT-CERT-1`, `FMT-WSP-1`, `frob-report`) | `BD-094`, `BD-088` | All 7 conditions of §0.2 "Implemented" met across all 51 crates |
| **`BD-096`** | M6 / `WP-01` | **`GATE-RELEASE-01` — `franken_robotics` v2.0 Ship Signoff (SOLE TERMINAL SINK)** | `BD-095` | **`GATE-RELEASE-01` PASS — v2.0 Ready!** |

---

# APPENDIX A — 3-MODEL DUELING IDEA WIZARDS SYNTHESIS & SIX BLIND SPOTS

*(Full per-item critiques, reviewer calibration meta-analysis, and all 78 independent cross-scores live in [DUELING_WIZARDS_REPORT.md](file:///Users/jemanuel/projects/franken_robotics/planning/duel/DUELING_WIZARDS_REPORT.md).)*

### A.1 Headline Cross-Model Convergence & Top-Scoring Mechanisms
Across **90+ ideas → 39 formal proposals** cross-scored by **`CC` (Claude Opus 5.5)**, **`COD` (OpenAI GPT-6.2-Sol)**, and **`AGY` (Gemini 4 Neon)**:
- **Top Safety Dissolution:** `COD FTA-P3` (`.fenergy` Observable Boundary Energy Budget — Rank #1 in `CC`'s scoring at `780/1000`, `870/1000` `AGY`, Steelman'd by `CC`).
- **Top Architecture Bet:** `CC-01` MorphoJIT (`864` `COD`, `945` `AGY`, Steelman'd by `COD`) fused with `AGY #1` Limb Port-ABI ("Tier 0.5") + `COD W02` ragged contact-island compaction + repaired `TA-P2` `(world, replicate_id)` race annealing.
- **Top Algebraic Campaign Levers:** `CC TA-P1` (`804` `COD`) + `COD FTA-P1` `.fquot` (`735,471 → 24,546 = 29.96×`) + `COD W01`/`CC-07` decorated cospans + wreath + exact Burnside (`86,185`) + `CC-02`/`COD W07` H/K Gait Atlas & saltation adjoint (`842`/`725`) + `COD W03` `.fskill` capability maps (`715`/`875`) + `COD W08` `.fclear` (`650`/`820`) + `COD W06`/`CC-04` exact linkages & GPU homotopy (`713`).
- **Top Studio / World Loops:** `CC-03` CEGIS `ddmin` (`820` `COD`) + `AGY #10` `2^10` Pearl-Causal Time-Travel (`818` `COD`) + repaired `CC-08` conservative 2.5D causal tribology (`735` `COD`) + `COD W09`/`CC-10` Spec-to-Silicon `.fbuild`.

### A.2 All Six Phase 6.9 Blind-Spot Breakthroughs Folded Into v2.0
1. **`BS-1` (`COD`) — Observation-Alias Impossibility Witness** (`frob-preflight::observation_conflict`, Stage 8, `PO-OBSALIAS-1`, `GATE-OBSALIAS-01`).
2. **`BS-2` (`COD`) — Dynamic Partial-Order Reduction (DPOR) Timing Verification** (`frob-runtime::schedule_check`, `720 → 8 = 90×`, `GATE-DPOR-01`).
3. **`BS-3` (`COD`) — Adaptive-Data-Analysis Blinded Qualification Firewall & $\alpha$-Spending Ledger** (`frob-ledger::qualification`, $\alpha_i = \alpha/[i(i+1)]$, `GATE-QUAL-01`).
4. **`B1` (`CC`) — Order-Independent GPU Scatter & `StateHash` Replay Determinism Gate** (`frob-gpu::determinism`, `GATE-DET-01`, protecting CRN $\rho$ and `2^10` Shapley!).
5. **`B2` (`CC`) — Model-Form Robustness & Simulator Discrete Work–Energy Audit** (`frob-bench::model_form` + `frob-codesign::rankcheck`, blocking $\sqrt{2}\mu$ & penetration-trampoline exploits, `GATE-RANKCHECK-01`).
6. **`B3` (`CC`) — Joint Robot–Home Retrofit Co-Design** (`frob-worldgen::amend` + `frob-redteam::amend`, 40-item catalog / `10,700` ≤3-item kits dual of `ddmin`, `GATE-AMEND-01`).

---

# APPENDIX B — BRENNER DISAGREEMENT REGISTER & KILLED (`†`) HYPOTHESES

*(Full register in [disagreement_register.md](file:///Users/jemanuel/projects/franken_robotics/planning/brenner/distillations/disagreement_register.md) and [brenner/ARTIFACT.md](file:///Users/jemanuel/projects/franken_robotics/planning/brenner/deliverables/ARTIFACT.md).)*

### B.1 Permanently Killed (`†`) Hypotheses (Never Re-Introduced!)
| Kill Tag | Killed Proposal / Assumption | Exact Mathematical or Live-Code Refutation (`FACT`) | Replacement in v2.0 |
|---|---|---|---|
| **`† K-01`** | Single-RHS FP16 Tensor-Core Delassus (`AGY #4`, `C009`) | Rank-1 $W$ gives $\kappa(W+\rho I) = \mathbf{4{,}801}$ (48× over claimed `≤100`); refinement solves shifted system; 4090 dense BF16/FP32-acc is `165` TFLOPS/card (`330.4` dual), not `660`! | `COD W10` Factor-Once Multi-RHS (`frob-contact::response_batch`) |
| **`† K-02`** | Log-Posynomial → `fln-anvil::omega` (`C007` `REJECTED`) | $\log(\sum c_k \prod \theta_i^{a_{ik}})$ is convex log-sum-exp, neither linear nor polynomial (`omega` is integer-linear only!). | Narrow convex-GP monomial bounds + `fs-ivl` directional boxes |
| **`† K-03`** | Assuming `fln-anvil::{egraph,norm_num,simp}` & multivariate `fsym-formal` exist (`E023`/`E024`) | Live `fln-anvil` (`1,423` LOC) implements ONLY `DTree` & `omega`; `fsym-formal` supports ONLY 2 univariate `ZZ` profiles! | Proved `FrobMath` direct `fln-kernel` computational reflection |
| **`† K-04`** | 50 Hz 3D LBM inside 1,024–32k-env RL loops (`AGY #9`) | At `dx=10 cm`, `dt=20 ms`: `Ma = 0.35–5.2`, `τ-½ = 6e-6` (BGK-unstable!); full-room 32k envs = `63.7 TB`! | Conservative 2.5D causal tribology + 3-mode Housner in Tier-A; Mach-bounded `fs-lbm` (`≤64` envs) in Tier-B |
| **`† K-05`** | Clairvoyant `∀w ∃u` CBF (`AGY #3`), inverted friction infimum & unboxed projection (`CC-06`) | `∀w ∃u` allows clairvoyance; $\inf_{\mathcal{K}(\mu_{\min})}$ is non-conservative for env forces; unboxed formula gives `(1.25, 0.25) ∉ [0,1]²`! | `∃u ∀(w,e)`, split $\mu_{\min}/\mu_{\max}$ cones, sort-and-clip / projected Newton + KKT cert + `.fenergy`/`.ftube` |
| **`† K-06`** | Running GPU RL directly on `frankentorch::DenseTensor` (`v1 §17.3`) | `DenseTensor` is 100% CPU `Arc<Vec<T>>`; `ft-nn` pulls `rayon`; `ft-kernel-metal` is sync `offset=0` (`F-06`)! | Device-owned `GpuTensorView` kernels + `BD-029a` `ft-kernel-metal v0.2` |
| **`† K-07`** | Inner-ellipsoid-only feasible-region sampling (`AGY #7`) & scalar-ball `G-WORLD-1` (`v1`) | Inner ellipsoids slice off active boundary optima; scalar balls fail rectangular doorway turns! | Exact `W06`/`CC-04` boundary-preserving synthesis + `COD W08` dual $SE(2)$ `.fclear` |
| **`† K-08`** | Unconditional $\delta_1$ double-counting (`TA-P3`), wrong `14` vs `27` orbits, `(world, epoch)` CRN pairing (`TA-P2`), wrong `83,333` Burnside (`CC-07`), retracted `≤0.1%` reward hacking (`CC-10`) | All recomputed, refuted & conceded in `WIZARD_REACTIONS_CC.md` (`C4`, `C9`, `C15`, `C16`, `C18`)! | Conditional $\delta_1 = P(\text{silent}\mid\text{breach})$, `27` $V_4$ strata, `(world, replicate_id)` CRN, `86,185` Burnside, unit-normalized STL |

---

# APPENDIX C — VERIFIED MATH EV MATRIX & GALAXY-BRAIN CARDS (`GB-01`–`GB-16`)

### C.1 Mathematical Machinery Expected-Value (EV) Filter
Every algebraic structure in Part II survived a strict EV test (payoff $\times$ probability of clean Rust execution $-$ complexity cost): PGA/CGA (`fs-ga`), $\mathrm{Sym}^+(4)$, Cartan curvature $D\mathbf{A}_S$, Trapezoidal LGVI, DLA Pfaffian, 4D Coulomb–Contensou + 3-Tier Delassus + `W10`, GVS Cosserat, extensive-entropy pHS, $\mathbb{R}/\mathbb{C}/\mathbb{H}$ + wreath + H/K + exact Burnside, decorated cospans + `.fquot`, `real_rur` + GPU homotopy + Smale $\alpha$-cert, proved `fln-kernel` reflection, `.fenergy`/`.ftube`/box-constrained `ClaimProduct`, conservative 2.5D tribology, $\mathfrak{se}(3)$ cellular sheaves, Imperfect $SE_{2+m}(3)$ InEKF, E–Engquist HMM, and `fs-tropical::max_cycle_mean`.

### C.2 Python-Verified Galaxy-Brain Transparency Cards (`GB-01`–`GB-16`)

*(Every single numerical value below was independently verified by `scratch/verify_math.py` and `scratch/verify_neon_v2.py`!)*

| GB ID | Transparency Card Title | Exact Verified Equation & Numerical Check (`FACT` / `DESIGN`) |
|---|---|---|
| **`GB-01`** | **Cartan Curvature Minus Sign** | $(D\mathbf{A}_S)_{ij} = \partial_i A_j - \partial_j A_i - [A_i, A_j]$ vs right-ordered $\overrightarrow{\mathcal{P}}\exp(-\oint \mathbf{A}_S dr)$: minus sign gives `err = 9.7e-4` on kinematic snake loop; wrong plus sign gives `err = 2.026`! |
| **`GB-02`** | **Trapezoidal-Potential LGVI Energy Stability** | Exact $+h\xi_k$ Cayley + $\mu_{k+1}^{\mathrm{free}} = (\mathrm{Ad}_{\tau(h\xi_k)})^T \mu_k^+$: rel energy err `5.62e-5` (`h=2 ms`) → **`8.79e-7 < 1e-6`** (`h=0.25 ms`), exact `O(h^2)` & backward time-reversibility `<1e-13`! |
| **`GB-03`** | **3-Tier Packed Symmetric Delassus SRAM** | `m_c=10` (`30×30`): $\frac{30\cdot 31}{2}\times 4 = \mathbf{1.86\text{ KB}}$ (Tier S); `m_c=16` (`48×48`): $\frac{48\cdot 49}{2}\times 4 = \mathbf{4.70\text{ KB}}$ (Tier M); `m_c>16`: matrix-free Tier L (never `36 KB` 32-contact dense!). |
| **`GB-04`** | **`COD W10` Multi-RHS Factor Reuse** | For $n=96$ active contact rows, 1 Cholesky = $96^3/3 = 294{,}912$ flops, 1 triangular solve = $2\cdot 96^2 = 18{,}432$ flops → 64 RHS = `1.475M` vs `20.05M` flops = **`13.6×`** linear-solve speedup (Amdahl needs $p \ge 97.1\%$ for 10× end-to-end). |
| **`GB-05`** | **Exact Burnside Closed-Loop Orbit Count** | 10 leg types on 6 $D_6$ slots: $(10^6 + 2\cdot 10 + 2\cdot 10^2 + 10^3 + 3\cdot 10^4 + 3\cdot 10^3)/12 = \mathbf{86{,}185}$ (`11.60×` dedup); sagittal mirror $\mathbb{Z}_2$ only: $(10^6 + 10^3)/2 = \mathbf{500{,}500}$ (`2.00×`). |
| **`GB-06`** | **`COD FTA-P1` Ballast Quotient Potency** | $\binom{24}{8} = \mathbf{735{,}471}$ internal ballast placements collapse via $(\sum x_i, \sum x_i^2)$ to exactly **`24,546` distinct classes (`29.96×` exact experiment deletion!)**. |
| **`GB-07`** | **`.fenergy` Observable Boundary Stop Budget (`FTA-P3`)** | $m=8\text{ kg}, v=0.4\text{ m/s}$: $E_{\mathrm{kin}} = 0.64\text{ J}$, drive $20\text{ W}\times 25\text{ ms} = 0.50\text{ J}$, stored `0.25 J`, reserve `0.10 J` → total **`1.49 J`** $\le W_{\mathrm{abs,min}} = 50\text{ N}\times 40\text{ mm} = \mathbf{2.00\text{ J}}$; exact cutoff = **`50.5 ms`** (**`45.5 ms`** w/ reserve)! |
| **`GB-08`** | **Extensive-Entropy PMSM Thermal Runaway Cliff** | $I_{\mathrm{rms}} = 25.40\text{ A}, R_0 = 0.54\,\Omega, \alpha_{\mathrm{cu}} = 0.00393/\text{K}, G_{\mathrm{th}} = 0.85\text{ W/K}, C_{\mathrm{th}} = 79\text{ J/K}$: $P_{20} = \mathbf{348.4\text{ W}}$, $P_{120} = \mathbf{485.3\text{ W}}$, runaway exponent $a = \mathbf{+0.00653/\text{s}} > 0$ → hits `120°C` cliff in **`33.5 s`** (`43.8%` torque derate!). |
| **`GB-09`** | **Bekker–Wong Tracked Wet-Clay Belly-Out** | Wet clay $k_c = 0.41, k_\phi = 21.6, n = 0.7$: narrow `8 cm` track sinks **`9.79 cm`** (belly-outs `8 cm` clearance!); widening to `12 cm` drops sinkage to **`4.96 cm`** (firm soil: `3.52 cm`), rescuing mission without motor change! |
| **`GB-10`** | **Dual-Silicon `fs-race` Early Elimination Savings** | Surviving compute fraction `0.382` over halving tournament cuts 42.67 GPU-h brute sweep to **`16.30 GPU-h` (`2.62×`)** on top of algebraic quotienting! |
| **`GB-11`** | **Exact $V_4$ Per-Foot Contact Strata (`TA-P3`)** | 4 feet $\times$ `{open, stick, slip}` under free $V_4$: $(3^4 + 3\cdot 3^2)/4 = \mathbf{27}$ strata & `162` one-foot transition lemmas (`189` SOS problems ≈ `10–13 min` on M5; coarse binary model = `7` strata / `16` lemmas). |
| **`GB-12`** | **Dual Inner/Outer $SE(2)$ `.fclear` Budget (`W08`)** | `160 × 160 × 48` = **`1,228,800` cells** (`4.69 MiB`/mode); heading half-bin `0.4 m × 5.625° = 39 mm` + translation half-diagonal `35 mm` = **`74 mm`** certified refinement band (`14.69×` width-family reuse!). |
| **`GB-13`** | **Box-Constrained Shield Exact Sort-and-Clip (`C2`)** | $R=I, u_\pi=(1,0), u\in[0,1]^2, u_1+u_2 \ge 1.5$: broken unconstrained formula gives `(1.25, 0.25) ∉ [0,1]²`; exact sort-and-clip over breakpoints finds $\lambda=0.5 \implies \mathbf{(1.0, 0.5)}$! |
| **`GB-14`** | **DPOR Conflict-Orientation Compression (`BS-2`)** | 6 equal-time events (sensor/packet/InEKF/watchdog/commit) with 3 independent conflicting pairs: `6! = 720` permutations collapse to **$2^3 = \mathbf{8}$ conflict orientations (`90×` exact reduction!)**. |
| **`GB-15`** | **Replay Determinism Lyapunov & CRN Breakpoint (`B1`)** | 1-ulp `f32` ($2^{-24}$) atomic drift grows as $t \approx \ln(2^{24})/\lambda \approx \mathbf{16.6/\lambda}$ s; dropping CRN $\rho$ from `0.95 → 0.50` inflates paired variance $(1-\rho)$ by **`10×`** and exact `2^10` Shapley from `1,024 → ~13,300` rollouts! |
| **`GB-16`** | **Robot–Home Retrofit Co-Design Combinatorics (`B3`)** | 40-item typed amendment catalog searched up to 3 items: $\binom{40}{1} + \binom{40}{2} + \binom{40}{3} = 40 + 780 + 9{,}880 = \mathbf{10{,}700}$ kits evaluated reusing the frozen robot policy! |

---

# APPENDIX D — CORRECTED WORKED `.fbot`, `.ftask`, `.fmodule` & ASYNC `frob-mcp` SESSION

*(All 8 v1 Appendix D bugs are fixed below: hollow shell `:shell-thickness 0.0025m`, explicit limb `(mount (se3 ...))` + sagittal/coronal mirror directives yielding true $V_4$ mechanical symmetry vs $\mathbb{Z}_2$ front-sensor observation symmetry, dedicated `pmsm-wheel-hub` + `1150 kg/m3` rubber tire + `WheelContact(Pfaffian)`, explicit `(power-bus ...)`, `(budgets ...)`, `(levers ...)` declaring `lidar_mount_z` and in-bounds `W_stance`, `mission_profile` passed to `bot_preflight`, and full `sync_wait_ms`/`job_id` async MCP handshake!)*

### D.1 Corrected `.fbot` (`kitchen_wheelquad_v2.fbot`, `FMT-FBOT-1`)
```lisp
(robot "kitchen_wheelquad_v2" :schema "frob.bot/v1"
  (symmetry :mech "V4" :obs "Z2_sagittal")  ;; front LiDAR/RGB-D breaks coronal obs symmetry!
  (power-bus main-bus :battery (li-ion-2rc :v-nom 48V :cap 20Ah :r0 0.04Ohm)
                      :i-max 80A :thermal-sink 1.2W/K)
  (budgets :mass-max 28kg :bom-max-usd 4800 :power-max 1800W :width-max 0.58m :height-max 0.72m)
  (actuator pmsm-knee :kt 0.14Nm/A :r0 0.54Ohm :alpha-cu 0.00393/K :cth 79J/K :gth 0.85W/K :gear 12.0 :backlash 0.003rad)
  (actuator pmsm-wheel-hub :kt 0.18Nm/A :r0 0.38Ohm :alpha-cu 0.00393/K :cth 95J/K :gth 1.1W/K :gear 6.0 :backlash 0.001rad)
  (base chassis :geometry (box 0.52m 0.34m 0.14m) :shell-thickness 0.0025m :material al-6061-t6)
  (use-module leg-fl :from "std/wheel_leg_3dof.fmodule@sha256:9a4e..."
                     :mount (se3 :xyz (0.22m 0.17m -0.04m) :rpy (0deg 0deg 0deg))
                     :override (:knee-actuator pmsm-knee :wheel-actuator pmsm-wheel-hub
                                :tire-material rubber-nitrile-1150kg/m3
                                :contact-law (WheelContact :pfaffian true :mu-nom 0.72)))
  (mirror-limb leg-fr :from leg-fl :planes (sagittal) :mount (se3 :xyz (0.22m -0.17m -0.04m) :rpy (0deg 0deg 0deg)) :mirror-axes true)
  (mirror-limb leg-rl :from leg-fl :planes (coronal)  :mount (se3 :xyz (-0.22m 0.17m -0.04m) :rpy (0deg 0deg 0deg)) :mirror-axes true)
  (mirror-limb leg-rr :from leg-fl :planes (sagittal coronal) :mount (se3 :xyz (-0.22m -0.17m -0.04m) :rpy (0deg 0deg 0deg)) :mirror-axes true)
  (sensor front-lidar :type lidar-3d :parent chassis :mount (se3 :xyz (0.26m 0m 0.12m) :rpy (0deg 0deg 0deg)) :rate 10Hz)
  (sensor front-rgbd  :type rgbd-ir  :parent chassis :mount (se3 :xyz (0.27m 0m 0.08m) :rpy (0deg 12deg 0deg)) :rate 30Hz)
  (sensor body-imu    :type imu-6dof :parent chassis :mount (se3 :xyz (0m 0m 0.02m) :rpy (0deg 0deg 0deg)) :rate 500Hz)
  (levers
    (W_stance      :range (0.30m 0.46m) :target (chassis.width))
    (N_knee        :range (8.0 20.0)    :target (pmsm-knee.gear))
    (lidar_mount_z :range (0.08m 0.28m) :target (front-lidar.mount.z))))
```

### D.2 Unit-Normalized STL Mission (`kitchen_tray_delivery.ftask`, `FMT-FTASK-1`)
```lisp
(mission "kitchen_tray_delivery" :schema "frob.task/v1"
  (world "commercial_kitchen_proxy_v3" :tribology-tier A :slosh-modes 3)
  (clocks :physics 500Hz :imu 500Hz :policy 50Hz :lidar 10Hz :thermal 10Hz)
  (reset-cache :settled-poses 64 :max-penetration 0.0005m)
  (stl-contract
    (pred clear-margin  :expr (- min_clearance 0.04m)    :scale 0.10m)
    (pred thermal-ok    :expr (- 115degC max_winding_t)  :scale 10degC)
    (pred spill-ok      :expr (- 0.015m tray_slosh_h)    :scale 0.015m)
    (online-past-reward (H_[0s,2s] (and clear-margin thermal-ok spill-ok)))
    (offline-verdict    (and (G_[0s,45s] (and clear-margin thermal-ok spill-ok))
                             (F_[0s,45s] (< dist_to_pass_counter 0.15m))))))
```

### D.3 Corrected Async `frob-mcp` Agent Session Transcript
```json
// 1) Agent invokes 8-stage preflight (passes mission_profile explicitly!)
{"tool": "bot_preflight", "args": {"bot": "kitchen_wheelquad_v2.fbot", "mission_profile": "kitchen_tray_delivery.ftask", "sync_wait_ms": 5000}}
// -> Returns in 142 ms: Stages 1–6 PASS (J in Sym+(4), Γ_mech=V4, Γ_obs=Z2_sagittal), Stage 7 .fclear WARNING (Doorway D threshold 22mm),
//    Stage 8 observation_conflict PASS. Emits B3 retrofit hint: "$12 renter-safe 1:12 threshold ramp clears Doorway D without chassis redesign!"

// 2) Agent launches paired co-design + 2^10 time-travel autopsy with strict async job_id contract
{"tool": "codesign_run", "args": {"bot": "kitchen_wheelquad_v2.fbot", "task": "kitchen_tray_delivery.ftask", "budget_gpu_min": 15, "sync_wait_ms": 5000}}
// -> Immediate async handoff (never blocks socket!):
{"status": "running", "job_id": "job_01J9X7K2", "progress_pct": 12.0, "determinism_class": "BitwisePerDevice", "execution_authorized": false}

// 3) Agent polls job_id -> completes Stage 5 B2 rankcheck (ModelRobust) & Stage 6 BS-3 Blinded Qualification (alpha_1 = 0.025)
{"tool": "job_poll", "args": {"job_id": "job_01J9X7K2"}}
// -> Result: FAIL-THERMAL-CLIFF at t=33.5s resolved by N_knee: 12->16 + lidar_mount_z: 0.12m->0.20m (above steam plume);
//    claim_axis: "EvaluatedDesign<ModelRobust>", qualification_receipt: "sha256:4f81...", hardware_safety_signoff: false.
```

---

# APPENDIX E — GLOSSARY & MULTI-PASS ADVERSARIAL AUDIT FOLD-IN TRAIL

### E.1 Concise Glossary
- **`AffineRows`:** 48-byte (`3×4` row-major FP32) GPU runtime rigid transform (`franken_threed §7.2–7.5`).
- **`ClaimProduct<VerifiedFormal, EvaluatedDesign>`:** Non-promotable Rust product type preventing formal/statistical claim laundering (`INV-001`).
- **Decorated Cospan (`.fmodule`):** Open port-Hamiltonian robot module with machine-checkable boundary substitution and incremental proof digests (`W01`).
- **DPOR (`schedule_check`):** Dynamic Partial-Order Reduction verifying sensor/estimator/watchdog/commit event orderings (`BS-2`).
- **`.fclear`:** Dual inner/outer $SE(2)$ cubical clearance complex certifying both feasible paths and impossibility cuts (`W08`).
- **`.fenergy`:** Contact-sequence-invariant observable boundary energy budget (`COD FTA-P3`, ISO/TS 15066 aligned).
- **`.fquot`:** Mission-observable design quotient deleting provably equivalent simulation runs (`COD FTA-P1`).
- **`.fskill`:** Task-space skill + exact per-mode affine actuation map + Farkas infeasibility witness (`COD W03`).
- **`MorphoJIT`:** 6-opcode screw-bytecode Tier-0 + Limb Port-ABI Tier-0.5 + ragged contact queues + race-annealed async Tier-1 GPU engine (`CC-01`/`AGY #1`/`W02`/`TA-P2`).
- **`Sym^+(4)`:** 4×4 SPD pseudo-inertia manifold guaranteeing physically realizable mass & triangle inequalities (`PO-INERTIA-1`).

### E.2 Complete Multi-Pass Adversarial Audit Fold-In Trail (`2026-11-21`)
Every single finding from **five independent adversarial verification passes** has been resolved and verified programmatically in v2.0:
1. **v1 Independent Math/Proof Audit (`68dd21f7`):** Fixed Cartan curvature sign (`dA - ½[A∧A]`), Trapezoidal LGVI (`+hξ_k`, `1/h`, `Ad^T`), non-associated De Saxcé NCP honesty, Frobenius–Schur $\mathbb{R}/\mathbb{C}/\mathbb{H}$ per-space character projectors, $\Gamma_{\mathrm{mech}}$ vs $\Gamma_{\mathrm{obs}}$, $O(3)$ reflection parity, and direct `FrobMath` `fln-kernel` reflection (replacing stubbed `fln-anvil` & univariate `fsym-formal`).
2. **v1 Independent GPU/Systems Audit (`b6453fa8`):** Fixed 3-Tier Delassus SRAM (`1.86/4.70 KB`/matrix-free vs 36 KB), Metal 3.2 honest M2 target (`OPEN-METAL4-RT`), `BD-029a` `ft-kernel-metal v0.2` + `[patch] block`, `GpuTensorView` decoupled from CPU-only `frankentorch::DenseTensor`, `MathMode::Safe` `TwoSum` canary, Dual 4090 `330.4` dense BF16 / `48` warps/SM / no-P2P host staging, and `fs-qty` Lift pre-classification (`FD-02`).
3. **v1 Independent Robotics/Worlds/DAG Audit (`74de55f8`):** Added `franken_threed @ bf7abe1` live integration (`F-07`), fixed all 8 Appendix D `.fbot`/MCP bugs, added `FMT-FTASK-1`, `frobd` (`FMT-RPC-1`), async `frob-mcp` `job_id`, `frob-cx` (`C-08`), `SubprocessSymbolicBackend` in Layer `V`, tactile patches, joint stiction/backlash, aerial rotor port, `frob-sensors::sheaf`, M3 lightweight interior proxies, and a 100% acyclic zero-orphan Beads DAG (`BD-001`–`BD-096`).
4. **Cross-Model MOR Audit (`AUD01`, OpenAI GPT-6.2-Sol):** Rejected `C007` log-posynomial→`omega`; killed `† K-01` single-RHS FP16 Delassus ($\kappa = 4{,}801$); verified `E021`–`E026` live sibling facts; enforced paired `n >= 10` 95% LCB Program Measurement Protocol.
5. **3-Model Dueling Idea Wizards & BrennerBot (`CC` + `COD` + `AGY`):** Adopted `.fenergy`, `.fquot`, `.fmodule`, `.fclear`, `.fskill`, `.ftube`, `W06`+`CC-04` homotopy, `W07`+`CC-02` H/K gait atlas, `W10` multi-RHS, MorphoJIT+Port-ABI+`W02`+`TA-P2` (`B >= 65k`, `(world, replicate_id)` CRN), conservative 2.5D causal tribology (`C6/C7/C8`), rainflow residue stack (`C17`), unit-normalized STL (`C18`), exact Burnside `86,185` (`C4`), `27` $V_4$ strata (`C16`), conditional $\delta_1$ (`C15`), box-constrained shields (`C1/C2`), AND all **6 Phase 6.9 Blind-Spot Breakthroughs (`BS-1`, `BS-2`, `BS-3`, `B1`, `B2`, `B3`)**!
