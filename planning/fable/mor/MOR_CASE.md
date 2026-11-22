# MOR_CASE.md — neutral case file, FrankenRobotics ten-fold pass (run `f1`)

Run: `f1` · Date: 2026-11-21 · Target document: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md`
(3,666 lines, version 0.2). This file states the decision and the evidence. It contains no proposed answer.

## 1. Decision contract

- **Primary question.** Which changes to the plan would let FrankenRobotics deliver a tenfold improvement over the
  strongest robot simulation and design pipelines available in late 2026, and which tempting changes should be
  rejected? "Tenfold" must be made measurable by the analysis; it is not yet defined in the plan.
- **Decision owner.** The project owner (Jeffrey Emanuel), who asked on 2026-11-21 for a plan that is "much more
  radically innovative and ambitious" and seeks "a 10x improvement and unlock compared to all existing systems".
- **Horizon.** The build program of the plan (milestones M0 to M6), by agent swarms.
- **Scope.** In: architecture, mathematics, algorithms, GPU use, scenes, agent studio, verification. Out: staffing,
  business model, hardware robots.
- **Success criterion.** A ranked set of audited claims that either justify adding a mechanism to the plan with a
  falsifier and a kill gate, or justify rejecting it.
- **Loss function.** Two errors are equally costly: (a) missing a real order-of-magnitude lever and shipping an
  incremental simulator; (b) adopting impressive-sounding mechanisms that do not work, cannot be built on the
  foundations that exist, or do not move the end-to-end metric. A claim of speed or novelty that is not grounded in
  evidence is harmful.
- **Constraints (owner facts).** Rust; reuse of the sibling projects as far as they really go; CUDA on a Linux host
  with two RTX 4090 and low-level Metal on Apple silicon; legged, wheeled, tracked and hybrid robots; terrain and
  interior scenes (homes, apartments, offices, warehouses, manufacturing plants, retail stores, commercial kitchens);
  a robot design studio for AI agents through a CLI and MCP; formulation in Lie theory and abstract algebra, with
  franken_lean and frankensympy.
- **Changeability.** Nothing is built. Every choice in the plan is reversible at the cost of editing a document.

## 2. Evidence ledger

Each row is a fact with its source. Paths are under `/Users/jemanuel/projects/` unless stated.

| ID | Kind | Fact | Source |
|---|---|---|---|
| E001 | Machine | This Mac: Apple M5 Ultra, 36 CPU cores, 80-core GPU, Metal 4, 256 GB unified memory. | `sysctl`, `system_profiler` on 2026-11-21 |
| E002 | Owner fact | Linux host `trj`: Threadripper PRO 5995WX, two RTX 4090 (24 GB each, no NVLink). Often heavily loaded. | Owner brief; `frankensim/docs/CROSS_ISA_VERIFICATION.md` |
| E003 | Code | FrankenSim has SE(3)/SO(3) with exp, log, adjoint, coadjoint, Jacobians (`fs-ga/src/lie.rs`), Featherstone forward and inverse dynamics for floating-base trees with one-axis joints only (`fs-mbd/src/articulated.rs`), a 29-DoF Unitree G1 and a KUKA iiwa 7 model, and a G1 rollout at 480 Hz with compliant feet (`fs-cmaes-viz-wasm/src/g1_walking.rs`). | Files named |
| E004 | Code | FrankenSim contact: simultaneous Signorini–Coulomb solve by semismooth Newton, default cap 21 contacts, an event solve with no persistent time stepper; convex-primitive narrow phase only; no heightfields, no mesh manifolds. | `frankensim/crates/fs-contact`, `fs-query`; contracts |
| E005 | Code | FrankenSim has no batched environments and no GPU code in its workspace. Its accelerator doctrine: "FrankenSim does not ship an accelerator backend"; "The CPU reference is permanent". GPU apps exist outside the workspace (`apps/instrument-lab/render-gpu*`) using `cudarc` + NVRTC and Metal with one shared kernel source and a CPU emulation oracle. | `frankensim/docs/ACCELERATOR_DOCTRINE.md`; `apps/instrument-lab/render-gpu-piano/src/` |
| E006 | Code | FrankenSim crates that exist with line counts of source: `fs-topopt` 40k (SIMP topology optimization), `fs-topols` 15.7k (level-set), `fs-solid` 212k (elasticity, fatigue), `fs-toleralloc` 6.5k, `fs-asbuilt` 10.8k, `fs-geocon` 1.3k, `fs-fab` 0.4k, `fs-io` 22k (bounded STEP), `fs-voi` 1.0k (value of information), `fs-race` 0.6k (e-racing), `fs-eproc` 1.4k (e-processes), `fs-uq` 15.9k, `fs-assimilate` 13k (data assimilation), `fs-surrogate` 6.7k (reduced-order models inside validity bands), `fs-sos` 4.3k (SOS with interval-verified certificates), `fs-spectral` 14.3k, `fs-bo` 8.3k, `fs-dfo` 37k, `fs-archive` 0.4k (MAP-Elites), `fs-adjoint` 10k, `fs-phs` 66k (port-Hamiltonian), `fs-couple` 1.04M, `fs-lbm` 348k, `fs-tropical`, `fs-ivl` (intervals, Taylor models), `fs-tilelang` 0.3k. Absent: any reachability, conformal, Koopman or rare-event crate. | `ls` and `wc` on `frankensim/crates` on 2026-11-21 |
| E007 | Code | FrankenTorch executes tensors on the CPU only; `Device::Cuda`/`Metal` are metadata. It has reverse-mode autograd, Linear/GRU/LSTM/attention, Adam, safetensors. `ft-kernel-metal` is a real Metal compute gateway (synchronous, 31 buffer arguments, `f32`/`u32` buffers, `MathMode::Safe`). No PPO. | `frankentorch/crates/ft-api`, `ft-dispatch`, `ft-kernel-metal/src/compute.rs` |
| E008 | Code | FrankenSymPy: exact polynomial algebra, Buchberger Gröbner bases (cap 1,024 basis polynomials), real RUR for zero-dimensional systems (caps of 8 variables, degree 256), interval certificates up to 16 coordinates, a proof kernel for polynomial identities, single-expression Rust/C printers, no multi-output code generation, no Lie algebras or group theory in Rust, no homotopy continuation. | `frankensympy/crates/fsym-*` |
| E009 | Code | FrankenLean re-checks compiled `.olean` files with an independent kernel (Mathlib: 9,131 of 11,083 modules accepted; Lie groups, matrix exponential and representation theory blocked). It cannot elaborate Lean source that needs Mathlib tactics. | `franken_lean/IMPLEMENTATION_STATUS.md`, S26 receipt |
| E010 | Code | franken_numpy is an `f64` heap-array library without GPU or autodiff. frankenjax's GPU backend is a stub. fastmcp-rust 0.13 is a working MCP server library. | READMEs and manifests |
| E011 | Public | MuJoCo Playground (MJX) reports PPO training throughput on one A100 of 417k env-steps/s for a quadruped and 106k for a 29-DoF humanoid; a quadruped joystick policy trains in about 5 minutes on two RTX 4090; humanoid policies in under 30 minutes. | https://arxiv.org/abs/2502.08844 |
| E012 | Public | MuJoCo Warp: single precision, not differentiable, per-world capacity limits whose overflow is undefined behaviour, not deterministic by default; NVIDIA reports it far faster than MJX without absolute RTX 4090 numbers. Newton 1.8 offers several solvers (MuJoCo Warp, a reduced-coordinate PGS solver, a maximal-coordinate ADMM solver, XPBD, VBD, cloth, MPM) and is CPU-only on macOS. Genesis has a Metal path through its own compiler; one headline throughput number was re-measured about 150 times lower under corrected settings. | https://mujoco.readthedocs.io/en/latest/mjwarp/index.html ; https://github.com/newton-physics/newton ; https://stoneztao.substack.com/p/the-new-hyped-genesis-simulator-is |
| E013 | Public | Sim-to-real studies in 2025–2026 attribute most of the gap to actuator dynamics (armature, friction, delay); one reports 46% more travel distance after identifying them. | https://arxiv.org/abs/2604.10351 ; https://arxiv.org/abs/2503.01255 |
| E014 | Public | Published work on equivariant policies for symmetric legged robots reports better sample efficiency; work on co-design by language-model agents (several 2025–2026 papers) reports gains of roughly 18–35% from multi-agent design debate. | https://arxiv.org/abs/2402.15552 ; https://arxiv.org/abs/2510.25850 |
| E015 | Plan | The current plan (v0.2) proposes: Lie-typed state and Cayley updates; three physics tiers (CPU referee on FrankenSim, CPU batch, GPU) with a fidelity ledger and a promotion rule; convex compliant contact; derived kernels from one source for CUDA and Metal; scene packs with distance fields and seven interior grammars; ray-kernel sensors; an invariant filter and baseline controllers synthesized per morphology class (gait table plus a force-distribution program); PPO on device with optional equivariance; sixteen static analyses including symmetry, mobility, co-design closure and maneuverability; anytime-valid design comparison; lineage with bisect; an energy ledger; replay of any GPU world on the CPU; certificates for kernels; a verb registry generating CLI and MCP. | The target document |
| E016 | Plan | The current plan's stated performance targets are parity-class with incumbents (for example at least 2.0 M quadruped world-steps/s on one RTX 4090 and at least 400k env-steps/s in training). It defines no end-to-end metric for design-iteration speed and no benchmark against an incumbent pipeline. Its bets BET-05, BET-06, BET-07 and BET-10 concern pruning by static analysis, baseline controllers, symmetry and agent surfaces. | Target document §2, §28 |
| E017 | Workspace | Two other plans for the same project exist beside the target (`…__NEON.md`, `…__ASTRA.md`), written by other agents, and a separate ten-fold pass on the NEON plan has left idea slates from three model families under `planning/duel/`. These are other agents' proposals, not evidence of feasibility. | `ls` of the project folder |

## 3. Pivotal questions

A question is pivotal if a plausible answer would change what the plan commits to.

- **PQ1. Where does the time go?** In a design-iteration loop built on the strongest incumbent tools, what are the
  phases, how long does each take, and which phases bound any end-to-end improvement? What end-to-end metric makes
  "tenfold" falsifiable?
- **PQ2. What is structurally out of reach for incumbents?** Which capabilities could FrankenRobotics have that an
  incumbent engine plus a thin agent layer could not obtain without changing its core representation? Which claimed
  differentiators in the current plan are not differentiators at all?
- **PQ3. What transfers from other fields?** Chip design automation, compilers and build systems, computational
  chemistry, aerospace multidisciplinary design, formal verification of cyber-physical systems, and game tooling
  each solved a "design against an expensive simulator" problem. Which of their mechanisms map onto robot design
  with their structure intact, and where does each mapping break?
- **PQ4. What does the hardware really permit?** Given E001 and E002, what are the order-of-magnitude envelopes for
  worlds, scene memory, simulator-in-the-loop planning, CPU shadowing of GPU worlds, and mixed precision?
- **PQ5. What must be true, and what would make it fail?** Reasoning backwards from a 2029 in which the tenfold
  claim was independently confirmed, and from one in which everything was built and the claim still failed.

## 4. Unknowns that could reverse the answer

- Measured phase times of an incumbent design loop (no measurement exists in this workspace).
- Whether a zero-training controller exists that works across morphology classes.
- Whether a kernel that treats robot topology as data is competitive with one specialized per robot.
- Whether any formal or interval method scales past reduced-order models in contact-rich settings.
- The compute envelope the owner will approve for pretraining campaigns.
