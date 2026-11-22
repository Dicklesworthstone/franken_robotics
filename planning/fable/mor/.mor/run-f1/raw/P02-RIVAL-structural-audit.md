RESULT_STATUS: MATERIAL_INCREMENT

# P02-RIVAL-structural-audit (PQ2)

Pass date 2026-11-21. Method: documentation audit only; nothing was run. Versions pinned from PyPI and project
changelogs today: MuJoCo 4.0.1 and MuJoCo Warp (MJWarp) 4.0.1 (2026-11-11), Newton 1.8.0 (2026-11-12), Warp 1.19.0,
Isaac Lab docs "Release 3.0" with Isaac Sim 6.2.0, Genesis World 1.4.7, Drake 1.58.0, ManiSkill 3.0.1, Brax 0.14.2.

Headline: of the sixteen capabilities, none is cleanly out of reach for the incumbent set as a whole. Ten are
documented in at least one incumbent in whole or in part (a, b, c, d, e, f, h, l, m, n), six need only a layer
that touches no engine core (g, i, j, k, o, p), and two narrow sub-capabilities have a plausible structural
barrier (inside a and l). Ledger row E012 is out of date on determinism and on heterogeneous worlds.

A quote marked † was relayed by a summarizing fetch tool; the others were read from the raw source text.

## 1. Checklist

| # | Capability | Best incumbent evidence | Classification | Confidence |
|---|---|---|---|---|
| a | Bit-reproducible replay of a GPU world; re-run on a CPU reference | Newton 1.8.0 [determinism guide](https://github.com/newton-physics/newton/blob/main/docs/concepts/determinism.rst): "repeated runs on the same GPU model, driver, and Warp build produce bitwise identical results" (SolverSteel, XPBD, Featherstone: Yes; Kamino: No; MuJoCo solver on GPU: "not a bitwise trajectory guarantee"). Same guide: results "usually differ across GPU models and between the CPU and CUDA backends". [Isaac Lab 3.0](https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism): "identical results for scenes with rigid bodies and articulations" on the same hardware and version. [MJWarp](https://mujoco.readthedocs.io/en/latest/mjwarp/index.html) default: "No. There may be ordering or *small* numerical differences". | Same-GPU bitwise replay: ALREADY-AVAILABLE. CPU re-run within a tolerance: AVAILABLE-WITH-THIN-LAYER (record seed, actions, per-world fields; re-run on Warp's CPU device). Bitwise CPU-equals-GPU: STRUCTURALLY-HARD, at the compiler-toolchain level (two compilers, fused multiply-add, different elementary functions), not the representation level. | High; high; medium-low (documented as absent, not shown to be impossible; Warp already has a `GPU_TO_GPU` mode) |
| b | Fork a snapshot into counterfactual branches | MuJoCo 4.0.1 [simulation docs](https://mujoco.readthedocs.io/en/latest/programming/simulation.html): after `mj_copyState(..., mjSTATE_INTEGRATION)`, "stepping ``src`` or ``dst`` will produce identical results"; the 3.17.0 batch API keeps an "``nsim x nstate`` array" of state rows with "results ... bit-identical to a plain loop at any thread count". No GPU fork verb found in MJWarp or Newton docs (absence of evidence). | ALREADY-AVAILABLE on CPU; AVAILABLE-WITH-THIN-LAYER on GPU (copy state arrays plus solver warm starts) | High; medium |
| c | Different topologies in one GPU batch | [ManiSkill3](https://maniskill.readthedocs.io/en/latest/user_guide/tutorials/custom_tasks/advanced.html) on PhysX: "we allow merging articulations that may have completely different link/joint structures and DOFs" (padded to `max_dof`). Newton 1.8.0 ships [`example_kamino_basic_heterogeneous`](https://github.com/newton-physics/newton/blob/main/newton/examples/kamino/example_kamino_basic_heterogeneous.py): "all basic models as a single heterogeneous multi-world model"; its [worlds page](https://github.com/newton-physics/newton/blob/main/docs/concepts/worlds.rst) warns "not all solvers are able to simulate them". Not available in MJWarp (one model per batch; per-world assets only), in one Isaac Lab binding ("Articulation variants must have the same links, joints, collision-body count, and names"), or in Genesis (variants "for the same entity"). | ALREADY-AVAILABLE, experimental and partial. Structural only for MJWarp and MJX. | High that it exists; low on speed and maturity at legged-robot scale |
| d | Edit structure without rebuilding | MuJoCo 4.0.1 [model editing](https://mujoco.readthedocs.io/en/latest/programming/modeledit.html): "`mj_recompile` updates an existing mjModel and mjData pair in-place, while preserving the simulation state. This allows model editing to occur **during simulation**, for example adding or removing bodies." On GPU, MJWarp: "compilation currently requires a host call: 1 call per new field instance", and a new graph capture. | ALREADY-AVAILABLE on CPU (a state-preserving recompile). On GPU: AVAILABLE-WITH-THIN-LAYER at a latency cost nobody has published. | High; medium |
| e | Zero-training control by sampling MPC | [MJPC](https://github.com/google-deepmind/mujoco_mpc): "a simple yet very competitive planner called Predictive Sampling". [hydrax](https://github.com/vincekurtz/hydrax): "Sampling-based model predictive control on GPU with MuJoCo MJX". [arXiv 2511.19204](https://arxiv.org/abs/2511.19204): "all without requiring reference tracking or offline pre-training"†, on a real Go2 and a simulated humanoid. | Mechanism: ALREADY-AVAILABLE. For arbitrary morphologies: AVAILABLE-WITH-THIN-LAYER (automatic cost and contact-schedule synthesis); whether that layer works across classes is unproven for everyone. | High; low |
| f | Differentiability through contact | [MJX docs](https://mujoco.readthedocs.io/en/latest/mjx.html): "mostly supported in MJX-JAX but is **not** currently available in MJX-Warp". [Brax](https://github.com/google/brax): "fast and fully differentiable physics engine". Newton and Genesis advertise autodiff. | ALREADY-AVAILABLE (gradient quality not audited) | High |
| g | Energy accounting per actuator and contact | MuJoCo 4.0.1 [energy flag](https://mujoco.readthedocs.io/en/latest/XMLreference.html): "potential and kinetic energy in constraints is not accounted for"; sensors `actuatorfrc`, `actuatorvel`, `contact`, `e_potential`, `e_kinetic` exist. No built-in ledger found. | AVAILABLE-WITH-THIN-LAYER (integrate force times velocity from existing fields; book the rest as residual) | Medium |
| h | Electrical and thermal actuator dynamics | MuJoCo 3.7.0 [changelog](https://mujoco.readthedocs.io/en/latest/changelog.html): "Added the dcmotor actuator ... Supports optional electrical dynamics (inductance), cogging torque, thermal resistance variation, and LuGre friction"; the thermal model "Adds one activation variable for winding temperature". Newton 1.8.0 imports it; Isaac Lab has DC-motor, delayed and LSTM actuator models. | ALREADY-AVAILABLE | High |
| i | Static pre-simulation design analysis | None found built into any incumbent engine. MorphoSymm supplies symmetry groups from per-robot configuration. Language-model co-design tools ([RoboMoRe](https://arxiv.org/abs/2506.00276), [Debate2Create](https://arxiv.org/abs/2510.25850)) evaluate by simulation only. | AVAILABLE-WITH-THIN-LAYER (analyses read only the model description). Unshipped, which is a product gap, not a barrier. | Medium |
| j | Valid sequential comparison | STEP, [arXiv 2503.10966](https://arxiv.org/abs/2503.10966): "finite-sample, near-optimal sequential testing procedure"†; [arXiv 2603.13616](https://arxiv.org/abs/2603.13616) extends it to non-binary metrics. | AVAILABLE-WITH-THIN-LAYER (works on episode outcomes) | High |
| k | Rare-event and adversarial failure search | Research methods only, for example [RADIUM](https://arxiv.org/abs/2404.03412); no integrated GPU tool found. | AVAILABLE-WITH-THIN-LAYER (black-box samplers plus the forking of row b) | Medium |
| l | Formal or interval certificates | Drake 1.58.0 [`CspaceFreePolytope`](https://drake.mit.edu/doxygen_cxx/classdrake_1_1geometry_1_1optimization_1_1_cspace_free_polytope.html): "all configurations in the convex polytopes is collision free"†. Nothing found for simulated quantities in any GPU engine. | Kinematic and polynomial certificates: ALREADY-AVAILABLE (Drake). Certified kernels or enclosures of simulated quantities: STRUCTURALLY-HARD for hand-written single-precision kernels. | High; medium |
| m | Native Apple-silicon GPU | [Genesis](https://github.com/Genesis-Embodied-AI/genesis-world): its compiler "lowers Python kernel code to CUDA, AMD ROCm, Apple Metal, Vulkan, x86, and ARM64". Newton: "macOS (CPU only)". [JAX](https://github.com/jax-ml/jax/blob/main/docs/installation.md): "JAX is not supported on Mac/OSX GPU". | ALREADY-AVAILABLE in Genesis; STRUCTURALLY-HARD for the Warp and JAX stacks | High |
| n | Interior scenes with articulated fixtures | [RoboCasa365](https://github.com/robocasa/robocasa) v1.0: "2,500+ kitchen scenes". [SceneSmith](https://arxiv.org/abs/2602.09153): "generates simulation-ready indoor environments from natural language prompts"†. [Infinigen](https://github.com/princeton-vl/infinigen) Indoors and Articulated. | ALREADY-AVAILABLE for homes and kitchens. Warehouses, plants and retail: no evidence found. | High; low |
| o | Agent interface with budgets and typed refusals | [Isaac Sim 6.2.0](https://docs.isaacsim.omniverse.nvidia.com/6.2.0/development_tools/isaac_sim_mcp.html) ships an MCP server that "gives AI coding assistants deep knowledge of NVIDIA Isaac Sim"† (search, not execution); a community server drives the simulator. Budgets and typed refusals: not found. | AVAILABLE-WITH-THIN-LAYER (it is the layer) | High |
| p | Structural analysis, topology optimization, fatigue | None found. MuJoCo 4.0.0 adds "Continuum elasto-plasticity" for deformables, which is simulation, not design-side stress analysis. | AVAILABLE-WITH-THIN-LAYER in kind (export load histories to an external solver), but the layer is a second solver stack. Absence of evidence. | Medium-low |

## 2. Candidate claims

**P02-1.** No checklist capability, taken alone, is a durable structural differentiator against the incumbent set.
*Type:* audit finding. *Evidence:* table. *Assumptions:* documentation matches shipped behaviour. *Scope:*
existence of a capability, not its speed or maturity. *Reversal:* a hands-on test shows a documented capability
unusable at locomotion scale.

**P02-2.** E012 needs correcting: opt-in bitwise run-to-run GPU determinism exists (Newton 1.8.0, Isaac Lab 3.0
on PhysX), and so do heterogeneous worlds (Newton's Kamino solver, ManiSkill3). *Type:* factual correction.
*Evidence:* rows a, c. *Assumptions:* none beyond the documents. *Scope:* late 2026. *Reversal:* the cited
documents are wrong.

**P02-3.** The only part of replay that incumbents document as missing is bitwise agreement between CPU and GPU.
*Type:* conditional differentiator. *Evidence:* row a; the plan's own §20.8 calls this a hypothesis and notes
Metal gives no control over rounding. *Assumptions:* FrankenRobotics controls code generation down to the
elementary functions, and CPU replay is worth more than same-GPU replay. *Scope:* CPU and CUDA; Metal is
weaker. *Reversal:* GATE-SPARK-01 fails, or Warp extends `GPU_TO_GPU` to the CPU.

**P02-4.** No single incumbent solver is documented to combine heterogeneous topologies, bitwise determinism and
implicit articulated contact in one GPU batch. Kamino has the first and not the second; SolverSteel has the second
and the first is undocumented. *Type:* gap observation, an absence of evidence. *Assumptions:* I read the right
pages. *Scope:* Newton 1.8.0. *Reversal:* a Newton release closes it; the guide says missing solver support "is
tracked", so treat the gap as temporary.

**P02-5.** "Topology as data" differentiates only as a measured number: distinct designs times steps per second,
and time from an edit to the first GPU step. *Type:* performance claim awaiting measurement. *Evidence:* rows c,
d. *Assumptions:* design search needs many distinct morphologies with few worlds each. *Scope:* GPU tiers.
*Reversal:* ManiSkill3 or Newton heterogeneous throughput is within about three times homogeneous throughput.

**P02-6.** Certified kernels and enclosures of simulated quantities are the one item with no incumbent evidence
and an authoring-level barrier. *Type:* structural candidate of unproven value. *Evidence:* row l.
*Assumptions:* E008 and E009 leave enough reach. *Scope:* kernels and reduced models, not contact-rich rollouts.
*Reversal:* BET-11 fails, or the certificates never change a design decision.

**P02-7.** Metal differentiates against NVIDIA-lineage engines only; Genesis already runs there. *Type:*
comparative. *Evidence:* row m. *Assumptions:* Genesis rigid-body simulation works on Metal as advertised.
*Scope:* Apple silicon. *Reversal:* Genesis on this M5 Ultra meets the BET-09 target.

**P02-8.** What no incumbent ships is the composition: one design identity and one evidence ledger across
static analysis, simulation tiers, statistics and in-family solid mechanics (E006). *Type:* inference, weaker
than the claims above. *Evidence:* rows i, j, k, o, p are all layers that exist separately. *Assumptions:*
integration cost is large for incumbent users. *Scope:* the agent design loop. *Reversal:* a baseline built from
MuJoCo 4, mjSpec, Newton, hydrax and STEP matches the end-to-end metric.

## 3. Plan differentiators that fail this audit

- §1.1 GPU fossil ("The CPU version is a different code base"; a failed GPU world "cannot be replayed anywhere
  else"): false for Warp, which compiles one kernel source for CPU and CUDA, and for same-GPU replay.
- §1.1 Design fossil ("Morphology is fixed before the simulator starts"): contradicted by `mj_recompile`.
- N1, one source for three backends: Warp and Genesis's compiler already do this. Only the stored per-kernel
  disagreement record is new, and it is a practice.
- N3, trust ladder: Newton runs several solvers on one model and Isaac Lab 3.0 switches PhysX and Newton.
- N4 and GOAL-15, anytime-valid comparison: STEP and its successor.
- N2 and BET-05, static analyses: unshipped but a thin layer.
- GOAL-10: reduced to the hypothesis in P02-3.
- GOAL-11, energy ledger: a thin layer.
- GOAL-12, actuators: MuJoCo's `dcmotor`.
- GOAL-13 and BET-06, baseline controller: sampling MPC exists.
- GOAL-06, interiors: residential scenes exist.
- GOAL-18 and BET-10, agent surface: a layer by definition.
- BET-09, Metal: see P02-7.

## 4. Not verified

- Nothing was executed. Speed, maturity and usability of every incumbent feature are unmeasured.
- Whether SolverSteel accepts heterogeneous worlds, and whether Kamino handles hundreds of distinct legged robots.
- Whether one world's trajectory is independent of batch composition under run-to-run determinism.
- MJWarp's `DeterminismType` details, and whether the `dcmotor` thermal state runs on GPU.
- MJX recompilation cost per new model (no source read).
- Drake differentiation through its contact solvers, and its region-of-attraction documentation.
- Genesis rigid-body gradients and Metal throughput.
- Whether Isaac Lab 3.0.0 is final: the release-notes page tops out at v3.0.0-beta2.patch1.
- Generators for warehouses, plants, retail stores and commercial kitchens.
- Whether MorphoSymm detects symmetry automatically (secondary source only).
- Any agent design tool with structural analysis in the loop; one search found none.
