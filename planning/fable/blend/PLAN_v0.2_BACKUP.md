# COMPREHENSIVE PLAN FOR FRANKEN_ROBOTICS

> **FrankenRobotics** is a robot proving ground and a robot design studio built as one mathematical instrument.
> It simulates legged, wheeled, tracked and hybrid robots in terrain and interior scenes with physics supplied by
> FrankenSim, runs thousands of worlds at once on CUDA and Metal GPUs, and exposes a command line and an MCP server
> through which AI agents design a robot, test it, read what happened, and improve it. Every layer is formulated in
> Lie theory and abstract algebra: configurations are group elements, joints are subgroups, contact is a cone program
> in the dual Lie algebra, symmetry is a finite group acting on the design, and the hot kernels are derived from that
> algebra instead of being typed in by hand. Repo `franken_robotics`; crate prefix `frob-`; binaries `frob` (CLI),
> `frobd` (job daemon) and `frob-mcp` (MCP server).

| | |
|---|---|
| Document | `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` |
| Version | 0.2 (first complete draft plus one internal three-reviewer round; see §0.6) |
| Date | 2026-11-21 |
| Status | Plan stage. **Nothing in this document is implemented in the §0.2 sense.** The repo is empty. |
| Language | Rust 2024 on a pinned dated nightly; one shared C-dialect kernel source for CUDA and Metal |
| Foundations | frankensim, frankentorch, franken_numpy, frankensympy, franken_lean, frankenscipy, asupersync, frankensqlite, fastmcp-rust, franken_threed |
| Hardware targets | Linux x86-64 with two RTX 4090 (`trj`); Apple silicon with Metal 4 (this Mac Studio: M5 Ultra, 80-core GPU, 256 GB) |
| Sibling plans studied | frankensim (main, addendum, bridge, new domains, Wright Flyer), franken_textiles, franken_mining, franken_threed, franken_lean, frankensympy |
| Process | `planning-workflow` and `idea-wizard` skills; provenance in §0.6; idea pass in Appendix B |
| Next step | External multi-model review rounds, then conversion to beads (§36 is the seed) |

---

## Contents

- **Part 0. Document control**: labels, what "implemented" means, identifiers, reading guide, names, provenance, hygiene
- **Part I. Thesis**: §1 what is wrong and what this is · §2 bets · §3 readers · §4 goals · §5 North-Star scenarios ·
  §6 the six claims
- **Part II. The mathematical spine**: §7 doctrine · §8 the fifteen constructions · §9 conventions · §10 Lemma
  (proofs and certificates)
- **Part III. The machine, end to end**: §11 Marrow · §12 Skeleton · §13 Sinew · §14 Kinesis · §15 Traction ·
  §16 Habitat · §17 Sensorium · §18 Reflex · §19 Cortex · §20 Spark · §21 Arena · §22 Atelier · §23 Chronicle ·
  §24 Tether
- **Part IV. Substrate, suite and engineering**: §25 suite consumption · §26 workspace and crates · §27 numerics and
  determinism · §28 performance · §29 safety, security, privacy
- **Part V. Verification, gates, risks**: §30 Gauntlet · §31 gate register · §32 risks and pre-mortem · §33 open
  questions
- **Part VI. Delivery**: §34 milestones · §35 work packages · §36 first beads · §37 first implementation session ·
  §38 definition of done
- **Appendices**: A glossary · B idea-wizard · C EV matrix · D mathematics compendium · E worked cards ·
  F upstream asks · G robots, scenes, licenses · H source register · I type sketches · J verb registry · K review log

---

# PART 0. DOCUMENT CONTROL

## 0.1 Normative language and evidence labels

MUST, MUST NOT, SHOULD and MAY carry their RFC 2119 meanings. Five labels say what kind of statement a sentence is.
A number in this plan without one of these labels, or without an obvious label from its table header, is a defect in
the plan.

| Label | Meaning | Who may change it |
|---|---|---|
| **FACT** | Observed on 2026-11-21 in a sibling repo, on this machine, or in a cited public source. Code facts cite `repo/path`; web facts cite a URL in Appendix H. | New evidence only |
| **DESIGN** | A decision of this plan. It carries its reason. | An architecture decision record (ADR) |
| **HYPOTHESIS** | Believed, not established. It names the test that would confirm or kill it. | The named test |
| **TARGET** | A number we intend to hit. It is not a measurement until a gate records the measured value. | The gate that measures it |
| **OPEN** | Undecided. It has an owner, a deadline expressed as the gate it blocks, and a fail-closed default (§33). | The owner |

Ambition is calibrated with the three tags the suite already uses (FACT: `frankensim/COMPREHENSIVE_PLAN_FOR_FRANKENSIM.md` §0):

- **[S] Solid**: established mathematics and engineering. The risk is implementation quality.
- **[F] Frontier**: published research that mainstream robot simulators have not productized. Real engineering risk.
- **[M] Moonshot**: a synthesis proposed here. It lives behind a feature flag and never sits on the critical path.

The spine of this plan is [S]. The features that make it worth building are [F]. A small, capped number of [M]
items are what would make it unlike anything else, and the plan works without every one of them.

## 0.2 What "implemented" means

A capability is implemented only when all eight hold. Otherwise it MUST be described as partial, and the missing
items named.

1. Code exists in the crate this plan maps it to (§26).
2. The crate ships a `CONTRACT.md` with ten sections: purpose, inputs, outputs, invariants, refusals, determinism
   class, evidence class of outputs, validity domain, oracles, known gaps.
3. Known-answer tests pass against a pinned oracle that was never regenerated from the candidate (§30.2).
4. Outputs that cross the agent boundary carry an evidence class, a tier, a model identity and a seed. No naked
   numbers reach an agent (§23).
5. A CPU reference path exists. Where a GPU path is claimed, a retained differential check against the CPU path
   exists for the same kernels (§20.6).
6. The capability is reachable through `frob --json` with a documented schema and exit code (§24).
7. Cancellation is tested: request, drain, finalize, with no leaked device resources (§30.6).
8. Its gate has a recorded PASS carrying the measured values, not the targets (§31).

## 0.3 Stable identifier prefixes

Identifiers are never renumbered. A retired identifier is tombstoned, not reused.

| Prefix | Tags | Prefix | Tags |
|---|---|---|---|
| `GOAL-` / `NONGOAL-` | Goals and non-goals (§4) | `NS-` | North-Star scenario (§5) |
| `BET-` | Thesis bet with its falsifier (§2) | `D-` | Doctrine rule (§7) |
| `C-` | Mathematical construction (§8) | `T-` | Theorem card (§10) |
| `INV-` | Invariant a subsystem maintains | `PO-` | Proof obligation, with its verification |
| `A-` | Static analysis of the design studio (§22) | `K-` | Kernel in the kernel library (§20) |
| `GATE-` | Convergence gate with a numeric threshold | `INT-` | Admission gate for a foundation (§25) |
| `KILL-` | Kill gate with its fallback named in advance | `ADR-` | Architecture decision record |
| `FD-` | Foundation dependency, a sibling repo (§25, App. F) | `FROB-REF-nnn` | Refusal code (§27.5) |
| `RISK-` | Risk row (§32) | `OPEN-` | Open question (§33) |
| `WP-` / `M` | Work package and milestone (§34, §35) | `BD-` | Plan bead, one-to-one with a `br` issue (§36) |
| `GB-` | Worked card with illustrative numbers (App. E) | `L-` | Layering or dependency rule (§26) |

## 0.4 Reading guide

Read §6 (the six claims) and §31 (the gate register) before anything else. Then read the sentences below; each is
the short form of a section and the plan stands or falls with them.

1. **Algebra first, numbers last.** Every hot kernel is the image of an algebraic object under a derivation we can
   replay (§7, §20.3).
2. **No ornamental mathematics.** A construction stays in the plan only if it owns a type, a verb an agent can call,
   a test that fails when the mathematics is wrong, and a measured payoff or a kill date (§7 D-2).
3. **Group elements, not angles.** State is stored as elements of Lie groups and advanced by maps that stay on the
   group (§11).
4. **One model, three tiers, one ledger of disagreement.** The referee tier, the CPU batch tier and the GPU tier
   run the same model, and their differences are measured and stored, never assumed away (§14).
5. **The CPU reference is permanent. GPUs propose, the referee disposes.** (§14.5, §20.6)
6. **Derive, don't hand-write.** Kernel arithmetic comes from symbolic derivation with a content hash (§20.3).
7. **Compose, don't enumerate.** A robot is a composition of components over typed ports, which is also
   FrankenSim's governing rule (§12).
8. **Consume FrankenSim, never fork it, and confine it behind one airlock crate.** (§25)
9. **Check before you simulate, simulate before you train, train before you claim.** (§22)
10. **Every design can stand on day one.** A baseline controller is synthesized from the model, so no design waits
    for a learned policy before it gets feedback (§18).
11. **A proof is what a checker accepted.** Everything else is a certificate, a test or an estimate, and says so
    (§10).
12. **Refuse by name, never approximate in silence.** (§27.5)
13. **A performance number is a target until a gate records it, and a recorded number names its world count,
    time step, substeps, solver iterations and contact count.** (§28)

| Reader | Start with |
|---|---|
| An agent about to implement a bead | §0.2, §26, the bead's chapter in Part III, §30 |
| An agent that will use the finished tool to design robots | §5, §22, §24, Appendix J |
| The owner | §1, §2, §5, §31, §33, §34 |
| A maintainer of a sibling project | §25, Appendix F |
| A mathematician | Part II, Appendix D, Appendix C |
| A reviewer looking for what could fail | §2, §31, §32, Appendix C |

## 0.5 Subsystem names

FrankenRobotics assembles a creature from parts, so the subsystems are named for the parts. The names are handles
for agents to talk to each other; the crates are in §26.

| Subsystem | Bounded context | Primary crates |
|---|---|---|
| **Marrow** | Lie groups and algebras, finite groups, cones, order theory | `frob-lie`, `frob-algebra` |
| **Skeleton** | The robot design language and the model compiler | `frob-design`, `frob-model`, `frob-io` |
| **Sinew** | Actuators, transmissions, batteries, thermal limits | `frob-actuate` |
| **Kinesis** | Dynamics, integrators, constraint solve, the three tiers | `frob-dyn`, `frob-kernels` |
| **Traction** | Collision, contact, friction, wheels, tracks, terrain mechanics | `frob-contact` |
| **Habitat** | Scenes: terrain and interiors, generated or ingested | `frob-scene` |
| **Sensorium** | Sensor models and the observation service | `frob-sense` |
| **Reflex** | State estimation and model-based control | `frob-control` |
| **Cortex** | The vectorized environment API and learning | `frob-learn` |
| **Spark** | GPU backends and the kernel derivation toolchain | `frob-kir`, `spark/frob-spark-cuda`, `spark/frob-spark-metal`, `derive/frob-derive` |
| **Arena** | Tasks, metrics, benchmark batteries | `frob-arena` |
| **Atelier** | The design studio: static analysis, comparison, optimization | `frob-analyze`, `frob-opt` |
| **Chronicle** | Telemetry, replay, the energy ledger, evidence | `frob-chronicle` |
| **Lemma** | Proofs and certificates | `frob-lemma`, `lean/` |
| **Tether** | CLI, daemon, MCP, the agent model | `frob-verbs`, `frob-cli`, `surface/frob-mcp` |
| **Gauntlet** | The verification program | `frob-conformance` |
| **Airlock** | The only code that names volatile FrankenSim types | `frob-fsim` |

## 0.6 How this plan was produced

1. **Study of the exemplars.** The plan documents named by the owner were read: FrankenSim's main plan (876
   lines) and its four companion plans (6,877 lines), FrankenTextiles (6,125), FrankenMining (9,490), FrankenThreeD
   (1,835), FrankenLean (738) and FrankenSymPy (1,527). Their structural devices are reused here on purpose:
   evidence labels and the "implemented" definition (Mining §0.1–0.2), claims that are never conflated (Textiles §3,
   ThreeD §5), ambition tags and the layer discipline (FrankenSim §0, §4), kill gates with fallbacks named in
   advance (Textiles §19, Mining §34), the first implementation session and definition of done (ThreeD §21–22), and
   the idea-wizard appendix (Mining App. C).
2. **Seven code audits of the siblings**, each reporting what exists in code as opposed to in plans: FrankenSim
   (workspace, multibody, contact, GPU policy, consumption mechanism), its companion plans, FrankenTextiles,
   FrankenMining, FrankenThreeD and adjacent 3D projects, FrankenLean with FrankenSymPy, and the numeric siblings
   (franken_numpy, frankentorch, frankenjax, frankenscipy, asupersync, fastmcp-rust). The findings that shaped this
   plan are stated as FACT where they are used, and collected in §25 and Appendix H.
3. **Direct verification** of the load-bearing facts: the `fs-ga` Lie API, the `fs-mbd` articulated API, the
   `fs_ports::Component` trait, the `ft-kernel-metal` compute gateway, FrankenSim's accelerator doctrine, the
   shared-kernel-source pattern in `frankensim/apps/instrument-lab/render-gpu-piano`, toolchain pins across the
   suite, the asupersync version, and this machine's hardware.
4. **A public-source research pass** (2026-11-21) on GPU robot simulators, Rust-to-GPU routes, geometric robotics,
   scene assets and benchmarks. Sources are in Appendix H. Items the pass could not verify are listed there and
   are not relied on.
5. **An idea-wizard pass**: 30 ideas, winnowed to 5, expanded to 15 (Appendix B). Every survivor is designed into
   the body; the appendix says where.
6. **One internal review round** by three fresh-context reviewers who had not seen the drafting conversation: a
   robotics-physics and GPU reviewer, a mathematics reviewer, and a plan auditor who checked claims against the
   sibling repos. Their findings and what changed are in Appendix K.

What has **not** happened yet: the four or more external review rounds the planning workflow calls for, a
competing-plan blend, and conversion to beads. The plan is written to survive those rounds, not to skip them.

## 0.7 Plan hygiene rules

These come from what the sibling plans recorded about their own mistakes.

- **The delivery log lives in `docs/delivery/`, never in this plan.** FrankenTextiles' §25 grew to 82% of its
  plan (FACT: Mining §0.7). This document states decisions; crate contracts and beads state progress.
- **A closed bead is not acceptance.** FrankenMining found 87 beads closed within 12 seconds with one shared
  close reason, serving figures from a stand-in (FACT: Mining App. K.2). Here a bead closes with a typed reason
  that cites its own test, and gates require a live-dependency proof (§30.8).
- **Process is never the product.** FrankenMining's `xtask` reached about 27% of its code (FACT: audit of
  `franken_mining`). `xtask` here has a hard scope: six checks, listed in §26.6. A new check needs a named defect
  class it would have caught.
- **Line counts are scope caps, never schedules.** Per-crate figures in §26 are caps that trigger a scope
  conversation at 115%. FrankenTextiles set a ceiling and retired it at 1.9× (FACT: audit of `franken_textiles`).
- **Every number carries a label** (§0.1). Performance numbers also carry the disclosure block of §28.2.
- **The six claims are never conflated** (§6), and each gate names exactly one of them.
- **Mathematics is written in Unicode, without LaTeX**, and every symbol is defined in §9 or Appendix D.
- **Sibling repos are read-only to this program.** Changes to them are requested as beads in their own trackers
  (Appendix F).

---
# PART I. THESIS

## 1. What robot simulation gets wrong, and what FrankenRobotics is

### 1.1 The fossils

Today's robot simulators are fast and widely used. They also carry design decisions that made sense when a
simulator was a C library driven by a person in a notebook, and that fit badly when the user is an AI agent
designing a robot it has never seen.

| Stage | Fossil | Consequence |
|---|---|---|
| State | Poses are bags of floats: Euler angles, quaternions renormalized when someone remembers. | Frame mix-ups and sign errors are the most common robotics bug, and nothing in the types can catch them. |
| Model | URDF and MJCF are XML trees of free-form numbers with no units and no composition rules. | An inertia matrix that no physical body could have loads without complaint. Two sub-assemblies cannot be combined with a guarantee about the result. |
| Dynamics | One opaque `step()`. | Nobody can say what the integrator conserves. Energy drift contaminates cost of transport, which is the number a leg designer cares about most. |
| Contact | A solver tuned for throughput, with softness parameters that have no physical meaning. | A learned policy finds the artifact and exploits it. The failure shows up on hardware and is filed under "sim-to-real gap". |
| GPU | Kernels written per vendor in single precision, with floating-point atomics. The CPU version is a different code base. | A world that fails on the GPU cannot be replayed anywhere else, so it cannot be debugged. |
| Design | Morphology is fixed before the simulator starts. Co-design means black-box search over XML parameters. | The only feedback is a scalar reward, hours later. |
| Agents | A Python API for a human at a keyboard. | No budgets, no refusals, no provenance. Telemetry is a tensor dump the agent cannot afford to read. |
| Verification | "It looks right in the viewer." | Results cannot be audited, and a design that scored well may have scored well for a wrong reason. |

### 1.2 What is already solved, and what nobody has composed

Each of the following exists and works (sources in Appendix H):

- Linear-time articulated-body dynamics in spatial algebra (Featherstone), and its Lie-group formulation (Park,
  Bobrow, Ploen).
- Convex, compliant contact that scales to thousands of GPU worlds (MuJoCo's formulation in MuJoCo Warp and
  Newton; Drake's SAP and its successors on the CPU).
- Lie-group and variational integrators with discrete conservation laws (Iserles et al.; Marsden and West; Lee,
  Leok, McClamroch).
- Invariant filtering for legged state estimation (Barrau and Bonnabel; Hartley et al.).
- Morphological symmetry groups and equivariant policies (Ordoñez-Apraez et al.), and the symmetry classification
  of gaits (Golubitsky, Stewart, Buono, Collins).
- Monotone co-design with guaranteed fixed points (Censi).
- Anytime-valid sequential inference, already implemented in FrankenSim (`fs-race`, `fs-eproc`).

What does not exist is their composition: one typed algebra in which the model, the dynamics, the contact problem,
the symmetry, the controller, the learning problem and the evidence all speak the same language, so that a
statement proved or measured at one layer is usable at the next. FrankenSim's thesis is that such a typed algebra
collapses an archipelago of tools into a continuum (FACT: FrankenSim plan §1). FrankenRobotics applies that
thesis to robots and takes it one step further: the algebra is not only the type system, it is the source code of
the kernels.

### 1.3 What FrankenRobotics is

Three products that share one model and one ledger.

1. **Arena, a gym.** A vectorized, device-resident environment API over batched physics. It runs legged, wheeled,
   tracked and hybrid robots across generated terrain and generated or ingested interiors: homes, apartments,
   offices, warehouses, manufacturing plants, retail stores, commercial kitchens.
2. **Atelier, a design studio for agents.** A CLI and an MCP server with verbs to create and edit a robot design,
   analyze it without simulation, simulate it, compare it against another design with statistical honesty, read
   structured telemetry about why it behaved as it did, and optimize it.
3. **A mathematical instrument.** A Lie-algebraic formulation of design, simulation and control in which the
   mathematics is load-bearing: it generates kernels, supplies test oracles, prunes designs before simulation,
   and, where a checker can accept it, carries proofs.

### 1.4 Claims of novelty, kept honest

Each is a HYPOTHESIS about the state of the field, stated to the best of our knowledge, and each has a test in
this plan.

- **N1.** One derivation produces the kernel arithmetic for a CPU reference, CUDA and Metal, with the
  cross-backend disagreement measured and stored per kernel (BET-02, GATE-SPARK-02).
- **N2.** Lie-theoretic static analyses are agent-callable verbs that return in seconds: the morphology's symmetry
  group, its gait atlas, its mobility, the growth vector of its wheel constraints, and the curvature of its
  mechanical connection (§22.2).
- **N3.** A trust ladder with promotion gates: no design or policy result is reported as established until it has
  been re-evaluated on a higher tier, and the difference is recorded (§14.5).
- **N4.** Design comparisons are anytime-valid: an agent may stop a comparison the moment the evidence is decisive
  and the error rate still holds (§22.5).
- **N5.** Kernels carry instance certificates: algebraic identities checked by a proof kernel, attached to the
  kernel's content hash (§10).

### 1.5 Why the suite, and why now

FACT, from the audits:

- FrankenSim already contains the hard referee-grade pieces: SE(3) and SO(3) with exponential, logarithm, adjoint,
  coadjoint and Jacobians (`frankensim/crates/fs-ga/src/lie.rs`); Featherstone forward and inverse dynamics for
  floating-base trees (`fs-mbd/src/articulated.rs`); a 29-degree-of-freedom Unitree G1 model with provenance
  (`fs-mbd/src/robot_models.rs`); a semismooth-Newton Signorini–Coulomb contact solve and certified continuous
  collision detection (`fs-contact`); a discrete Euler–Poincaré step on SE(3) with its adjoint (`fs-time`);
  CMA-ES, Bayesian optimization and MAP-Elites (`fs-dfo`, `fs-bo`, `fs-archive`); max-plus algebra
  (`fs-tropical`); anytime-valid racing (`fs-race`); units, intervals, evidence and a ledger.
- FrankenSim does **not** contain what a gym needs: no batched environments, no GPU path in its workspace, no
  persistent contact time stepper, no terrain or interior scene representation, no robot sensors, no MCP server.
  Its accelerator doctrine says "FrankenSim does not ship an accelerator backend" and "The CPU reference is
  permanent" (`frankensim/docs/ACCELERATOR_DOCTRINE.md`).
- The suite has proven the GPU pattern outside FrankenSim's workspace: one kernel source compiled for CUDA through
  `cudarc` and NVRTC and for Metal through a prelude of macros, checked against a CPU emulation
  (`frankensim/apps/instrument-lab/render-gpu-piano/src/{kernel.h,cuda.rs,metal.rs,emulate.rs}`), and a generic
  Metal compute gateway whose documentation names "a physics integrator" as an intended consumer
  (`frankentorch/crates/ft-kernel-metal/src/compute.rs`).

FrankenRobotics is therefore the missing product on top of a mostly present foundation: FrankenSim is the referee
and the bedrock; FrankenRobotics owns the batched Lie-algebraic rollout engine, the scenes, the sensors, the
learning loop and the agent-facing studio.

## 2. Bets, each with its falsifier

A bet is a claim the project depends on and could lose. Each names what would falsify it and the gate at which
that is measured. Thresholds are TARGETs.

| ID | Bet | Falsifier | Tested at |
|---|---|---|---|
| BET-01 | **The algebra pays for itself.** A Lie-typed, derived formulation gives competitive speed and removes whole bug classes. | Derived kernels run more than 1.3× slower than hand-tuned equivalents on the same backend after one optimization pass, or frame and unit bugs still appear in Gauntlet triage at the rate of an untyped baseline. | GATE-SPARK-03, M2 |
| BET-02 | **One source, three backends.** Derived arithmetic plus a shared kernel dialect keeps CPU, CUDA and Metal inside their declared determinism classes. | Unexplained cross-backend disagreement above the class envelope survives two repair cycles. | GATE-SPARK-02, M2 |
| BET-03 | **Convex compliant contact, audited by a referee, is good enough for locomotion.** | On the standard battery the fast tier disagrees with the referee beyond the fidelity thresholds on slip distance, stance impulse or cost of transport, and no parameter mapping fixes it. | GATE-FID-01, M2 |
| BET-04 | **FrankenSim can be consumed by path from outside its tree, stably.** | INT-FSIM-01 fails, or upstream breakage costs more than 20% of working days in any four-week window. | INT-FSIM-01, M0; KILL-FSIM |
| BET-05 | **Static analysis prunes bad designs before simulation.** | On the design corpus, `design check` verdicts predict simulated failure no better than a mass-and-torque heuristic (area under the ROC curve below 0.8, or less than 0.05 above the heuristic). | GATE-ATEL-01, M3 |
| BET-06 | **A synthesized baseline controller makes most well-formed designs move.** | Fewer than 80% of corpus designs that pass `design check` traverse 5 m of flat ground under the baseline controller. | GATE-REFLEX-02, M3 |
| BET-07 | **Symmetry buys sample efficiency.** | Equivariant policies need more than 0.7× the environment steps of unconstrained policies to reach the same return on symmetric robots, across five seeds. | GATE-CORTEX-03, M4 |
| BET-08 | **Single precision is adequate on the GPU**, given anchored coordinates and group-preserving updates. | The precision battery fails at long horizons: constraint violation, group-membership error or energy error exceed their envelopes. | GATE-SPARK-05, M2 |
| BET-09 | **Metal on Apple silicon is a real rollout tier, not a demo.** | On this M5 Ultra, quadruped throughput is below 25% of one RTX 4090 at matched settings. | GATE-SPARK-04, M2; KILL-METAL |
| BET-10 | **Agents design better with structure than with reward alone.** | In an A/B on the design benchmark, agents given the full Atelier surface do not beat agents given only `sim run` and a scalar score, per unit of compute. | GATE-ATEL-03, M4 |
| BET-11 | **Proof-carrying kernels are affordable.** | Instance certificates cover fewer than 80% of derived kernels within FrankenSymPy's budgets. | GATE-LEMMA-01, M5 |
| BET-12 | **Narrow purity is affordable.** A Franken-only core with GPU annexes reaches M2 without a foreign physics engine. | M0 plus M1 slip by more than 50% because a sibling could not supply a primitive and no fallback in Appendix F applied. | Program gate, end of M1 |

## 3. Readers and their questions

| Reader | Their question | Where answered |
|---|---|---|
| A design agent (primary user) | What can I call, what does it cost, what will it refuse, and how do I know a result is real? | §22, §24, App. J |
| A training agent | How do I get device-resident observations for thousands of worlds, and how do I reproduce one of them? | §19, §20, §23 |
| An implementing agent | What exactly do I build, against which oracle, and when is it done? | §0.2, Part III, §30, §36 |
| The owner | What am I betting on, what could kill it, and what do I have to decide? | §2, §31, §33 |
| A sibling maintainer | What does this project need from mine, and what happens if I say no? | §25, App. F |
| A sceptical reviewer | Where is the mathematics decoration, and where is a number unearned? | §7, App. C, §28 |

## 4. Goals, non-goals and anti-goals

### 4.1 Goals

| ID | Goal | Primary section |
|---|---|---|
| GOAL-01 | Simulate articulated robots with contact at referee quality on the CPU, built on FrankenSim. | §14.2 |
| GOAL-02 | Simulate thousands of worlds per device on CUDA and on Metal from one kernel source. | §14.4, §20 |
| GOAL-03 | Support legged robots: quadrupeds, hexapods and other multi-legged layouts, bipeds and humanoids. | §12.6 |
| GOAL-04 | Support wheeled robots (differential, Ackermann, omnidirectional), tracked robots, and hybrids such as wheel-legs and stair-climbing wheel clusters. | §15.5 |
| GOAL-05 | Provide terrain: slopes, stairs, gaps, stepping stones, rubble, and soft ground. | §16.3 |
| GOAL-06 | Provide interiors: homes, apartments, offices, warehouses, manufacturing plants, retail stores, commercial kitchens, generated from grammars and ingested from scans. | §16.4, §16.6 |
| GOAL-07 | Formulate design, simulation and control in Lie theory and abstract algebra, with every construction load-bearing. | Part II |
| GOAL-08 | Use FrankenSim, FrankenTorch, FrankenNumpy, FrankenSymPy and FrankenLean for what each can really do today, and file precise asks for the rest. | §25, App. F |
| GOAL-09 | Give agents a design studio: typed designs, typed edits, static analysis, simulation, comparison, telemetry, optimization. | §22, §24 |
| GOAL-10 | Make any world reproducible: replay a GPU world on the CPU from its address alone. | §23.3 |
| GOAL-11 | Account for energy: every joule that enters a rollout is attributed to where it went. | §23.4 |
| GOAL-12 | Model actuators honestly: torque-speed limits, reflected inertia, friction, delay, heating. | §13 |
| GOAL-13 | Synthesize a baseline estimator and controller for any well-formed design. | §18 |
| GOAL-14 | Train policies on device with PPO first, with equivariance available where the robot has symmetry. | §19 |
| GOAL-15 | Compare designs with anytime-valid statistics and common random numbers. | §22.5 |
| GOAL-16 | Detect results that depend on simulator artifacts before they are reported. | §14.5, §19.7 |
| GOAL-17 | Attach checked certificates to derived kernels and to selected safety claims. | §10 |
| GOAL-18 | Expose everything through one verb registry that generates the CLI, the MCP tools and the documentation. | §24 |
| GOAL-19 | Simulate sensors a robot would carry: encoders, IMU, force and contact, depth, lidar, height scans. | §17 |
| GOAL-20 | State determinism as a class on every result, and test it. | §27 |

### 4.2 Non-goals

| ID | Not in scope | Reason |
|---|---|---|
| NONGOAL-01 | A photorealistic renderer. | Sensors need geometry-correct depth and labels first. Hero images come from FrankenSim's path tracer offline (§17.5). |
| NONGOAL-02 | Soft robots, continuum arms, cloth and deformable manipulation in v1. | FrankenSim has no coupled soft-body lane for articulated robots yet (FACT, audit). Deferred to M6. |
| NONGOAL-03 | Flying and swimming robots in v1. | The rigid core supports them later; the aerodynamic crates exist in FrankenSim but are a separate program. |
| NONGOAL-04 | A firmware or deployment stack, ROS integration, or hardware-in-the-loop. | No real-time or HIL claim is made anywhere in this plan (§14.7). |
| NONGOAL-05 | A Python API. | The users are agents with a CLI and MCP. A thin binding can come later if a real consumer appears. |
| NONGOAL-06 | Windows. | Targets are Linux x86-64 and macOS on Apple silicon. |
| NONGOAL-07 | Dexterous manipulation benchmarks. | Mobile-manipulation reach and carry tasks are in scope at M4; in-hand manipulation is not. |
| NONGOAL-08 | General-purpose formal verification of robot controllers. | Lemma covers kernels and a short list of theorem cards (§10). |

### 4.3 Anti-goals

Things that would make the product worse even if someone asked for them.

- Lowering physics fidelity silently to keep a frame rate or a throughput figure. Under load the system sheds
  presentation or refuses; it never changes the physics tier of a running world (FACT: the same rule in the Wright
  Flyer plan §4.3).
- Wrapping MuJoCo, PhysX or any foreign engine as the production engine. Foreign engines are development-time
  comparison oracles only (§30.2).
- Publishing a throughput number without the disclosure block (§28.2).
- Accepting inertial parameters on faith. An imported model that is not physically consistent is refused or
  repaired with a recorded repair (§12.5).
- Putting a language model inside the physics or control loop.
- Hiding nondeterminism. A result whose determinism class is `Unspecified` is never cited as evidence.
- Mathematical decoration. See D-2.

## 5. North-Star scenarios

Each is a before-and-after acceptance story, tagged with the first milestone that can demonstrate it. Numbers are
TARGETs.

- **NS-01 Check before you simulate [M1].** *Before:* an agent edits XML, launches training, and learns six hours
  later that the knees saturate. *After:* `frob design check` returns in under two seconds: the mass spiral closes
  at 14.2 kg, peak knee torque demand exceeds the selected actuator's continuous rating by 31%, the design has
  symmetry group C₂ × C₂, and three ranked repairs are offered as typed edits.
- **NS-02 Teleport a world [M2].** *Before:* world 1,337 of 4,096 produces NaNs on the GPU and nobody can look
  inside it. *After:* `frob sim replay --world 1337 --tier batch-cpu` reproduces that world on the CPU from its
  address (seed, world, episode), stops at the first step whose digest differs from the GPU's or at the first
  refusal, and reports the contact that started it.
- **NS-03 An honest A/B [M3].** *Before:* two designs are compared on mean reward over an arbitrary number of
  episodes. *After:* `frob eval compare A B --task warehouse-tote` runs both on common random numbers, stops after
  the e-process crosses its threshold, and reports "B beats A on energy per metre, evidence 240 to 1, valid at any
  stopping time".
- **NS-04 A kitchen with a wet floor [M3].** A humanoid crosses a generated commercial kitchen whose floor
  friction map includes a wet patch. Telemetry reports slip distance per stance, the friction utilization margin,
  and the stills at the moment of the slip. The agent can ask which link a pixel belongs to.
- **NS-05 Symmetry for free [M4].** The studio reports the symmetry group of a hexapod, lists the gaits that group
  permits, and trains an equivariant policy that reaches the target return in at most 0.7× the steps of an
  unconstrained one.
- **NS-06 Thirty iterations in an hour [M4].** An agent runs thirty design-check-simulate-explain-edit cycles on a
  wheel-leg hybrid in one hour on `trj`, every design version content-addressed, every comparison anytime-valid,
  and `frob design bisect` finds the edit that broke stair climbing.
- **NS-07 An import that tells the truth [M1].** Importing a public humanoid description yields a loss report:
  which fields were dropped, which inertias fail the physical-consistency test, and which actuator limits were
  absent and filled from a card marked `ASSUMED`.
- **NS-08 Tracks on rubble [M4].** A tracked robot climbs stairs and crosses rubble; on soft soil the sinkage
  model is active and labeled Frontier in the evidence.
- **NS-09 The exploit is caught [M3].** A policy that scores well by exploiting contact compliance is re-evaluated
  on the referee tier; the score falls outside the promotion envelope and the result is labeled
  `provisional: tier-sensitive`.
- **NS-10 The same task on a Mac [M2].** The quadruped task trains on this Mac through Metal. Results agree with
  the CUDA run inside the declared determinism class, and the report says which class.
- **NS-11 A kernel with a certificate [M5].** `frob prove kernel K-se3-cay-step` returns a certificate that the
  update stays on the group and conserves spatial momentum in free flight, checked by FrankenSymPy's proof kernel,
  with a Lean capsule re-checked by FrankenLean where the theorem's dependencies allow.
- **NS-12 Your own kitchen [M5].** A walkthrough video becomes a private scene pack through the owner's existing
  reconstruction pipeline, and a candidate home robot is evaluated in it. Scale is labeled `ESTIMATED` unless
  anchored, and the scan never leaves the machine.

## 6. The six claims, never conflated

Every result this system produces makes exactly one kind of claim, and each kind is established by a different
kind of evidence. Gates, reports and documentation keep them apart.

| Claim | Meaning | Evidence that establishes it |
|---|---|---|
| **Proof** | A stated proposition holds. | A checker accepted it: FrankenSymPy's proof kernel for an instance identity, or a Lean kernel for a theorem (§10). |
| **Known answer** | The implementation recovers an analytic or synthetic truth. | A known-answer test with a derivation on file (§30.3). |
| **Parity** | The implementation reproduces a reference tool or a published number. | A fixture captured from a pinned oracle that was never regenerated from the candidate (§30.2). |
| **Fidelity** | A faster tier agrees with a slower, more trusted tier. | The fidelity ledger: per-quantity disagreement distributions on the standard battery (§14.5). |
| **Reality** | The simulation agrees with physical measurement. | A dataset of real-robot measurements with its own provenance. In v1 this claim is almost always `no-claim`, and reports say so (§30.7). |
| **Performance** | A speed or scale figure on named hardware. | A recorded run with the disclosure block (§28.2). |

Five rules:

1. A gate names exactly one claim.
2. A failure is a failed cell, never an excluded cell. Partial coverage never rounds up.
3. Parity with a simulator is not Reality. Agreeing with MuJoCo establishes that two programs agree.
4. Fidelity is directional: it says the fast tier tracks the referee on a battery, not that either is right.
5. A result inherits the weakest claim among its inputs. A proved kernel fed an assumed friction coefficient
   produces an estimate.

---
# PART II. THE MATHEMATICAL SPINE

## 7. Doctrine: algebra first, numbers last

The owner's brief is that the whole robot design, simulation and control problem be formulated in the language of
abstract algebra and Lie theory, and that this machinery guide the system instead of decorating it. The eight
rules below are how that brief is kept honest.

- **D-1 Algebra first, numbers last.** The primary description of every object is algebraic: a group, an algebra,
  a cone, a representation, a poset. Floating-point arrays are a lowering of that description, produced as late as
  possible and by a replayable derivation.
- **D-2 No ornamental mathematics.** A construction earns its place with four things: a Rust type, a verb an
  agent can call, a test that fails when the mathematics is wrong, and a measured payoff or a date by which the
  payoff is measured. A construction missing any of the four is moved to Appendix C as parked or rejected.
- **D-3 Group elements, not coordinates.** State is stored as group elements. Coordinates (angles, Euler triples)
  appear only at import, export and display, each at one boundary.
- **D-4 Structure is a test oracle.** Every conserved quantity, every symmetry and every algebraic identity is a
  metamorphic test that costs nothing to state. The Gauntlet's property tier is generated from Part II (§30.4).
- **D-5 Proof classes are typed, and the weakest link wins.** `Proved`, `Certified`, `Tested`, `Estimated` are
  distinct evidence classes (§10.2). A result is no stronger than its weakest input.
- **D-6 The moonshot budget is capped.** At most four [M] items are active at once. Each has a falsifier and an
  expiry gate. Nothing tagged [M] is on the critical path of §34.
- **D-7 Derive, don't hand-write.** The arithmetic of a kernel comes from a symbolic derivation with a content
  hash (C-14). Hand-written arithmetic is allowed in the referee tier and in kernel skeletons (loops, gathers,
  sorts), not in the mathematical core of a batched kernel.
- **D-8 Every domain ships its adjoint or its excuse.** A kernel either has a derivative program or a recorded
  reason it does not, with the estimator that replaces it (inherited from FrankenSim's expansion charter, D8).

## 8. The fifteen constructions

Each construction is stated with the same schema: the **object**, what it **replaces**, the **payoff**, the
**foundations** as they exist today (FACT), the **test**, and the **tier**. Symbols are defined in §9 and
Appendix D.

### C-01 Configuration is a Lie group, and joints are subgroups

- **Object.** A floating-base robot's configuration space is Q = SE(3) × G₁ × … × Gₙ. Joint j contributes the map
  θ ↦ ∏ₖ exp(θₖ Sₖ) for screws Sₖ ∈ 𝔰𝔢(3). When span{Sₖ} is closed under the bracket, the joint is a connected
  subgroup of SE(3). Those subgroups fall into twelve conjugacy classes (Hervé): the identity; line translation;
  rotation about an axis; helical motion of pitch p; planar translation; cylindrical; spatial translation; planar
  gliding SE(2); spherical SO(3); translating screw; Schönflies; and SE(3) itself. The six lower pairs are the
  revolute, prismatic, helical, cylindrical, planar and spherical classes. A joint that is not a subgroup (a
  universal joint) is a product of one-parameter subgroups.
- **Replaces.** `enum JointKind` with one hand-written code path per case.
- **Payoff.** One kernel generator parameterized by a screw basis covers every joint. Mobility analysis (A-01) can
  use intersections and products in the subgroup lattice, which gives the right answer for over-constrained
  linkages where the Grübler–Kutzbach count is wrong.
- **Foundations.** `fs-mbd` has `JointKind = Fixed | Revolute | Prismatic | Helical` only, with no spherical,
  universal or planar joints; `fs-ga` has `Twist::bracket` and `Twist::ad`. The joint catalog crate
  `fs-kinematics` proposed by FrankenSim's charter does not exist.
- **Test.** Bracket-closure classification of a screw basis in exact rational arithmetic (T-11). Known answers:
  the Sarrus linkage has mobility 1 where the count gives 0, and the Bennett linkage has mobility 1 where the count
  gives −2.
- **Tier.** [S] for the generator and classification; [F] for lattice-based mobility.

### C-02 The frame groupoid and typed spatial algebra

- **Object.** Frames are objects and a rigid transform `a_from_b` is a morphism from b to a, so transforms form a
  groupoid. The adjoint Ad is a functor from that groupoid to invertible maps on 𝔰𝔢(3); the coadjoint is its
  contragredient on 𝔰𝔢(3)*. A twist is a vector typed by a frame, a wrench is a covector typed by a frame, and
  their pairing is power, which does not depend on the frame.
- **Replaces.** 4×4 matrices and 6-vectors whose frames live in comments.
- **Payoff.** `a_from_b · b_from_c` has type `a_from_c`, and nothing else composes. A twist in frame b cannot be
  added to a twist in frame c. Static frames are phantom types; model-indexed frames are checked at model compile
  time, so batched kernels pay nothing.
- **Foundations.** `fs-ga` provides `Se3::{compose, inverse, adjoint, coadjoint}`, `Twist::transform_by`,
  `Wrench::{pairing, transform_by}`; `fs-mbd` already names transforms `world_from_base`.
- **Test.** Ad(gh) = Ad(g)·Ad(h); invariance of the pairing under a change of frame (T-04, T-05).
- **Tier.** [S].

### C-03 Inertia lives on a cone

- **Object.** A rigid body's ten inertial parameters are equivalent to the 4×4 pseudo-inertia matrix
  P = [[Σ, h], [hᵀ, m]], with Σ the second moment of mass and h the first. The parameters are physically
  consistent exactly when P is positive definite (Wensing, Kim, Slotine). The cone of such matrices with its
  affine-invariant metric is a Riemannian symmetric space.
- **Replaces.** Ten free floats.
- **Payoff.** Import validation is an eigenvalue test with a certificate. Optimization and identification steps
  follow geodesics and cannot leave the feasible set. By default inertias are not parameters at all: they are
  computed from geometry and material density, so an agent designs shapes and materials.
- **Foundations.** `fs-mbd::SpatialInertia`; `fs-mbd-material` (material-derived inertia, about 60k lines);
  `fs-opt` manifold variables with retractions.
- **Test.** Round trip between P and spatial inertia; detection of triangle-inequality violations; an audit of
  every reference robot in Appendix G.
- **Tier.** [S] for validation and geometry-derived inertia; [F] for Riemannian identification.

### C-04 Dynamics is the Euler–Poincaré equation on a tree

- **Object.** With body velocity ξ = g⁻¹ġ of the base and shape s (the joints), the body momentum is
  μ = 𝕀(s)·(ξ + 𝔸(s)·ṡ). Here 𝕀(s) is the locked inertia, which is the composite rigid-body inertia at the base,
  and 𝔸 is the mechanical connection. The base obeys μ̇ = ad*_ξ μ plus applied wrenches. Featherstone's
  recursions (RNEA, CRBA, ABA) evaluate these equations on the kinematic tree using Ad, ad and ad*.
- **Replaces.** An opaque step function.
- **Payoff.** Centroidal momentum is the momentum map, so centroidal quantities and their dynamics are available
  without extra code and the centroidal controller of §18 is the reduced equation. In ballistic flight the centre
  of mass follows a parabola and angular momentum about it is constant, which is an exact test. The recursion is
  parallel across tree depth, which is the shape a GPU wants.
- **Foundations.** `fs-mbd::articulated::{forward_kinematics, inverse_dynamics, forward_dynamics,
  free_floating_forward_dynamics, free_floating_momentum}` and `generalized::free_floating_terms` (mass matrix by a
  composite-rigid-body pass).
- **Test.** Parity with `fs-mbd` (GATE-KIN-03); ballistic invariants; kinetic energy equals ½·q̇ᵀM q̇;
  Ṁ − 2C is skew.
- **Tier.** [S].

### C-05 Time stepping stays on the group

- **Object.** An update is g₊ = g · τ(h ξ) for a retraction τ from the algebra to the group. Two retractions are
  supported: the exponential, and the Cayley map cay(x) = (I − x/2)⁻¹(I + x/2), which is rational and lands
  exactly on SO(3) and SE(3). On SE(3), cay(ω, v) has rotation cay(ω̂) and translation (I − ω̂/2)⁻¹v. The
  structure-preserving lane uses a discrete Lagrangian on the group; its discrete Euler–Poincaré equations
  conserve the discrete momentum map exactly (discrete Noether) and are symplectic.
- **Replaces.** Adding a quaternion increment and renormalizing; Runge–Kutta on Euler angles.
- **Payoff.** In exact arithmetic the update is group-valued, so the only thing left to remove is round-off: one
  normalization per step by a correctly rounded square root, a correction of order 10⁻⁷ in single precision
  instead of the order-h² projection that an additive update needs. The Cayley update uses only +, −, ×, ÷ and √,
  which IEEE 754 requires to be correctly rounded, so it is a candidate for bit-identical execution on CPU, CUDA
  and Metal (HYPOTHESIS; GATE-SPARK-01). A rational update is an object a polynomial identity checker can certify
  (T-01, T-02). Bounded energy error makes cost of transport trustworthy over long rollouts.
- **Honest limits.** Cayley is second-order: it rotates by 2·atan(|ω|h/2) instead of |ω|h (GB-01), so its use is
  bounded by |ω|h ≤ 0.2 per substep and refused beyond (FROB-REF-212). The default rollout integrator is
  semi-implicit Euler with implicit joint damping and a group update; the articulated variational integrator is
  Frontier.
- **Foundations.** `fs-time` has a discrete Euler–Poincaré step on SE(3) with `dep_momentum_adjoint` and a RATTLE
  projection hook. `fs-mbd` stepping is a second-order Lie midpoint step that its contract calls "neither an
  energy-exact nor a variational/symplectic integrator".
- **Test.** Group-membership residual stays within four units in the last place over 10⁷ steps with the per-step
  normalization on, and grows no faster than a random walk with it off; momentum conservation in free flight;
  bounded energy error; measured order by step halving.
- **Tier.** [S] for retractions and the semi-implicit lane; [F] for the articulated variational lane.

### C-06 Contact is a cone program in the dual algebra

- **Object.** At a contact frame the impulse γ = (γₙ, γₜ) lies in the friction cone 𝒦_μ = {‖γₜ‖ ≤ μ·γₙ}, with
  Signorini complementarity along the normal and maximal dissipation in the tangent plane. 𝒦_μ is a linear image
  of the second-order cone, which is the cone of squares of a spin-factor Jordan algebra. The nonnegative orthant
  (joint limits) and the positive-semidefinite cone (C-03, SOS certificates) are the other symmetric cones that
  occur. The fast tiers solve a convex, compliant problem in generalized velocities:
  v = argmin ½·(v − v*)ᵀA(v − v*) + Σ_c ℓ_c(J_c v), where the gradient of the convex potential ℓ_c is the contact
  impulse. The referee tier solves non-associated Coulomb friction with exact complementarity.
- **Replaces.** Projected Gauss–Seidel with softness knobs that have no physical meaning.
- **Payoff.** One `Cone` abstraction (Jordan product, spectral decomposition, projection, barrier) serves contact,
  joint limits, the control programs of §18 and the certificates of §10. The fast solve is strictly convex, so it
  has one solution and its gradient norm bounds the distance to it: solver error is a recorded number per step.
  Parameters are physical (stiffness, dissipation time, friction coefficient). The known artifacts of the convex
  model (bounded creep of a stuck contact, energy stored in compliance) are named and measured against the
  referee in the fidelity ledger.
- **Foundations.** `fs-contact::frictional_multicontact::solve_frictional_contacts` (simultaneous
  Signorini–Coulomb by semismooth Newton–Krylov, default cap 21 contacts, an event solve without persistent time
  stepping); elliptic friction; Hertz, Hunt–Crossley and JKR normal laws; `fs-tribo` friction laws. FrankenSim's
  persistent stepper lanes (Moreau–Jean, IPC) are open beads without code.
- **Test.** The known answers of §15.8; Hessian positive semidefinite by construction (T-08); projection
  identities (T-07).
- **Tier.** [S] for the convex compliant model; [F] for pressure-field patches and barrier contact.

### C-07 Wheels are a distribution, and maneuvering is a Lie bracket

- **Object.** Rolling constraints are Pfaffian, A(q)·q̇ = 0, and define a distribution 𝒟 = ker A. The flag
  𝒟 ⊂ 𝒟 + [𝒟, 𝒟] ⊂ … has a growth vector that decides controllability (Chow–Rashevskii) and cost: a direction
  first reached at bracket depth w needs path length of order ε^(1/w) to move a distance ε (ball-box theorem).
- **Replaces.** Folklore about which bases can do what.
- **Payoff.** A static maneuverability index for a wheeled base in tight interiors (A-07, GB-05). Bracket motion
  primitives for planners. For kinematic bases, the base motion follows from wheel motion with no dynamics at
  all. Ideal rolling is also the known-answer oracle for the frictional wheel contact that the simulator runs by
  default.
- **Foundations.** `fs-mbd::planar_drive` is SE(2) differential-drive kinematics with "no force, traction, slip";
  FrankenSim's nonholonomic lane is proposed, not built. FrankenSymPy's `diff` on scalar expressions exists, so
  brackets of vector fields can be computed symbolically today.
- **Test.** Growth vectors: differential drive (2, 3); car-like with steering (2, 3, 4); car with one trailer
  (2, 3, 4, 5); ideal omnidirectional base (3). Rolling disk and Chaplygin sleigh trajectories.
- **Tier.** [S] for the growth vector; [F] for the index and primitives.

### C-08 Locomotion is holonomy

- **Object.** Q → Q/G is a principal bundle over shape space. A connection 𝔸 splits motion into shape change and
  group motion. For a closed loop in shape space (one gait cycle) the net displacement is the holonomy, which to
  leading order is the integral of the curvature of 𝔸 over the enclosed surface.
- **Replaces.** Trial-and-error gait search for kinematic locomotors.
- **Payoff.** A design oracle with no rollouts: curvature magnitude over reachable shape space bounds displacement
  per cycle, so two morphologies can be ranked before simulation (A-09). With zero angular momentum the same
  formula gives the reorientation a robot achieves in flight by cycling limbs or a tail.
- **Honest scope.** Exact for principally kinematic systems (kinematic wheeled bases, snake-like crawlers,
  zero-momentum free flight). For dynamic walking it is a diagnostic, not a predictor. Curvature computed in poor
  coordinates misleads; the minimum-perturbation coordinates of Hatton and Choset are required.
- **Foundations.** None in the siblings. Derivatives of 𝔸(s) come from `fs-ad` forward duals.
- **Test.** Elroy's beanie: two coaxial bodies with inertias I₁, I₂ and zero momentum; a relative rotation Δψ
  rotates body 1 by −I₂/(I₁ + I₂)·Δψ. A planar three-link floating body: net rotation per cycle against direct
  integration.
- **Tier.** [F].

### C-09 A morphology has a symmetry group

- **Object.** The symmetry group 𝔾 of a design is the set of automorphisms of its labeled kinematic tree that are
  realized by an isometry of the body and preserve masses, inertias and actuator cards. 𝔾 acts on configuration,
  velocity, action and observation spaces through signed-permutation representations ρ, and the dynamics are
  equivariant: f(ρ_X(g)·x, ρ_U(g)·u) = ρ_X(g)·f(x, u). Each representation splits into isotypic components, and
  the Reynolds operator W ↦ |𝔾|⁻¹·Σ_g ρ_out(g)·W·ρ_in(g)⁻¹ projects any linear map onto the equivariant ones.
  Typical groups: C₂ for a biped, C₂ × C₂ for a quadruped built symmetric fore-aft and left-right, a dihedral
  group for a radially symmetric hexapod.
- **Replaces.** Hand-written mirror functions with sign tables per robot.
- **Payoff.** The group is computed from the design (A-03). It yields data augmentation by a factor |𝔾|,
  equivariant policies and invariant critics (BET-07), block-diagonal linearizations, and one metamorphic test of
  the entire simulator per group element. Observations declared invariant under world yaw and horizontal
  translation define a quotient decision process, and a metamorphic test checks that declaration against the full
  state including terrain. Design constructors `mirror` and `radial` produce symmetric designs with known
  generators.
- **Foundations.** FrankenSymPy has no group theory in Rust; FrankenLean's re-check accepted none of Mathlib's 43
  representation-theory modules; `fs-symmetry` is about 480 lines covering cyclic groups. The groups here have at
  most a few dozen elements, so closure, conjugacy classes, characters and projectors are built in `frob-algebra`.
- **Test.** Step equivariance to round-off for every generator (T-10); character orthogonality; idempotent
  projectors; ρ(g)·M(q)·ρ(g)ᵀ = M(ρ(g)·q).
- **Tier.** [S] for detection, augmentation and metamorphic tests; [F] for equivariant networks.

### C-10 Gaits are subgroups, and their timing is max-plus linear

- **Object.** A periodic gait of a 𝔾-symmetric system has a group H of spatio-temporal symmetries (elements that
  map the orbit to itself up to a phase shift) and a normal subgroup K of purely spatial symmetries; H/K is
  cyclic (Buono and Golubitsky). Trot, pace, bound, walk, pronk and the jump are distinct (H, K) pairs with
  prescribed phase lags. A footfall schedule is a max-plus linear system x(k+1) = A ⊗ x(k) whose eigenvalue is the
  stride period and whose eigenvector gives the steady phase offsets (Lopes et al.).
- **Replaces.** Hand-tuned phase-offset tables per robot.
- **Payoff.** A gait atlas (A-04): every (H, K) pair of the design's leg-permutation group, with phase relations,
  each usable as a clock for the baseline controller and as a reference for reward shaping. Periodic-orbit search
  restricted to symmetric trajectories has lower dimension. Max-plus synthesis gives schedules with a guaranteed
  steady state and a bound on transition time between gaits.
- **Foundations.** `fs-tropical` exists in FrankenSim. Nothing else.
- **Test.** The quadruped atlas recovers the primary gaits reported by Golubitsky et al. (1999); the max-plus
  eigenvalue of a trot schedule equals the stride period computed by hand.
- **Tier.** [F].

### C-11 Kinematics is algebraic geometry

- **Object.** Forward kinematics is a product of exponentials, a polynomial map in (cᵢ, sᵢ) with cᵢ² + sᵢ² = 1.
  Inverse-kinematics solution sets, workspace boundaries and singular loci are therefore real algebraic
  varieties, and SE(3) embeds as the Study quadric through dual quaternions.
- **Replaces.** Numerical IK with no statement of how many solutions exist.
- **Payoff.** Closed-form IK for three-joint legs, derived once by elimination and emitted as a kernel. Certified
  IK: all solutions in a box with existence and uniqueness certificates by interval Newton. Singular loci
  det J = 0 for leg Jacobians, giving singularity-free workspace volume and distance-to-singularity along a
  nominal gait (A-06).
- **Honest limits.** FrankenSymPy has Buchberger with a cap of 1,024 basis polynomials, no F4, and univariate
  resultants only, so general 6R inverse kinematics is not promised.
- **Foundations.** FrankenSymPy `fsym-polys` (Gröbner over ℚ with certificates, elimination, real RUR),
  `CertifyRootBox` (at most 16 coordinates); `fs-ivl` Krawczyk; `fs-ga` PGA motors as dual quaternions.
- **Test.** A spatial 3R positioning chain has up to four IK solutions; the certified solver finds exactly the
  solutions the closed form gives.
- **Tier.** [S] for product-of-exponentials and leg IK; [F] for certified IK; [M] for Study-quadric synthesis.

### C-12 A robot is a composition over typed ports

- **Object.** Components expose typed ports: a mechanical port carries a twist and a wrench at a frame, a shaft
  port carries (ω, τ), an electrical port (V, I), a thermal port (T, Q̇), a signal port a typed message.
  Interconnection is power-conserving (a Dirac structure). Designs are terms of an operad of wiring diagrams, and
  five functors give them meaning: kinematic (to the frame groupoid), dynamic (to port-Hamiltonian components),
  geometric (to collision Regions), budget (to monotone maps between resource and functionality posets), and
  symmetry (to groups). A design with feedback, such as heavier actuators needing more torque, is a least fixed
  point found by Kleene iteration (Censi's monotone co-design).
- **Replaces.** A monolithic XML file plus a sizing spreadsheet.
- **Payoff.** Editing a subterm recomputes only that subterm's meanings, by content-addressed memoization. "Does
  the design close?" is answered before simulation with either the minimal mass and power or a proof of
  infeasibility (A-10, GB-03). The energy ledger of §23.4 is the port structure read at run time. Gears are
  transformers and motors are gyrators, which is standard bond-graph algebra.
- **Foundations.** `fs_ports::component::Component` (contract v0, not frozen) with `ports()`, `class()`,
  `stored_energy_j()`; `fs-system::SystemBuilder`; `fs-mbd` already wraps a rigid body as a Component;
  FrankenSim's operating rule since 2026-11-09 is "Compose, do not enumerate".
- **Test.** Each functor commutes with composition (property test). The energy balance closes every step:
  supplied = stored change + dissipated + exported, to tolerance. Kleene iteration matches the closed form of
  GB-03.
- **Tier.** [F], deliberately thin: one data structure and five functors, not a framework.

### C-13 Estimation and control live on groups

- **Object.** The floating-base state (rotation, velocity, position) is an element of SE₂(3), extended by contact
  points to SE₂₊ₖ(3). IMU dynamics are group-affine, so the invariant error obeys an exactly linear equation and
  the invariant extended Kalman filter applies (Barrau and Bonnabel; Hartley et al.). Control errors are group
  errors g_d⁻¹·g and their logarithms. Centroidal model-predictive control is a program over contact wrenches in
  the cones of C-06 subject to the reduced equation of C-04. Whole-body control is an inverse-dynamics program
  over the same cones. Flat wheeled bases are tracked through their flat outputs.
- **Replaces.** Euler-angle PD loops and estimators that become inconsistent.
- **Payoff.** Every design receives an estimator and a baseline controller synthesized from its model (GOAL-13).
  Policies can be evaluated on estimated state instead of privileged state. The control programs reuse the cone
  solver of C-06, so one kernel family serves physics and control.
- **Foundations.** `fs-control` is motor current and speed control only (about 2.6k lines); `fs-ga::lie_opt` has
  geodesic means and `se3_optimize`; model-predictive synthesis in FrankenSim is proposed, not built.
- **Test.** Filter consistency (normalized estimation error squared inside χ² bounds); geometric PD convergence
  from attitude errors near π; KKT residuals of every control program.
- **Tier.** [S] for geometric PD and the invariant filter; [F] for controller synthesis across morphologies and
  for SOS and barrier certificates.

### C-14 Kernels are derived, typed by dimension, and carry certificates

- **Object.** A kernel's arithmetic is a straight-line program (SLP) over +, −, ×, ÷, √ and selects, generated
  from symbolic expressions with common-subexpression elimination. Every value carries its physical dimension as
  an exponent vector over `fs-qty`'s base dimensions, and a program type-checks only if every sum is homogeneous:
  physical quantities form a graded algebra and the kernel respects the grading. Derivative programs come from
  automatic differentiation of the SLP. Identities the kernel must satisfy are polynomial identities, checked by
  FrankenSymPy's proof kernel and bound to the kernel's content hash.
- **Replaces.** Hand-written kernels with hand-written derivatives in three languages.
- **Payoff.** The same arithmetic on three backends; adjoints for smooth kernels without extra work; no dimension
  errors; certificates that travel with the code.
- **Foundations.** FrankenSymPy has `Expr`, `fsym_calculus::diff`, `Matrix::jacobian`, `simplify`, a proof kernel
  with `Claim::AlgebraicIdentity` and a `PolynomialNormalForm` rule, and an offline verifier
  (`fsym-capsule-consumer`). It has no multi-output code generation with CSE and no GPU emitters; its bytecode
  interpreter caps an expression at 8,192 operations. FrankenRobotics therefore owns the SLP representation and
  its emitters (`frob-kir`) and uses FrankenSymPy at build time as the derivation engine and certificate checker.
- **Test.** Emitted Rust against FrankenSymPy's own evaluation and against `fs-ad` dual numbers; emitted GPU
  source against emitted Rust on each backend (GATE-SPARK-02).
- **Tier.** [F].

### C-15 Evidence is a test martingale

- **Object.** Comparing designs A and B on paired episodes with common random numbers yields differences dᵢ. A
  betting e-process Eₙ = ∏ᵢ (1 + λᵢ·d̃ᵢ) is a nonnegative supermartingale under the null hypothesis that B is no
  better, so by Ville's inequality the chance it ever exceeds 1/α is at most α. The comparison can stop at any
  time. Predictions an agent registers before a run are scored with proper scoring rules.
- **Replaces.** "Mean reward over five seeds."
- **Payoff.** Fewer episodes when the answer is clear, a valid error rate when the agent peeks, and cancellation
  of the losing arm the moment it loses (§22.5, GB-06).
- **Foundations.** `fs-race` and `fs-eproc` exist in FrankenSim; counter-based random streams keyed by logical
  identity exist in `fs-rand`.
- **Test.** Empirical type-I error under the null at or below α over 10⁴ simulated comparisons with optional
  stopping.
- **Tier.** [S].

## 9. Conventions

One convention per concept, converted at exactly one boundary each.

| Concept | Convention |
|---|---|
| World frame | Right-handed, z up, metres. Gravity points along −z. |
| Transform names | `a_from_b` maps coordinates in b to coordinates in a. Composition reads left to right by cancelling inner names. |
| Twist | ξ = (ω, v), angular part first, as `fs_ga::Twist::new(angular, linear)` (FACT). Body-frame by default: ξ = g⁻¹ġ. |
| Wrench | (τ, f), torque first, as `fs_ga::Wrench::new(torque, force)` (FACT). Pairing ⟨w, ξ⟩ is power in watts. |
| Adjoint | Ad_g maps a twist in the source frame of g to its target frame. ad_ξ η = [ξ, η]. ad*_ξ is the negative transpose action on wrenches, with signs fixed by μ̇ = ad*_ξ μ for a free body. |
| Rotation storage | Unit quaternion; component order and sign rule are FrankenSim's, confirmed at M0 (PO-MARROW-01). The double cover is resolved by a deterministic sign rule. |
| Revolute coordinate | A unit complex number (c, s) plus an integer turn count. No raw angle is stored in state. |
| Link numbering | Topological: the parent index λ(i) is less than i. Depth levels are precomputed. |
| Units | SI. `fs-qty` types at every public API. GPU buffers are raw `f32` with the dimension recorded in the buffer schema. |
| Time | Integer ticks of a rational step h = p/q seconds. Sub-schedules are integer ratios of the physics tick, fixed at admission. |
| World address | `WorldAddr = (run_seed, world, episode)`. A random stream is keyed by `(WorldAddr, step, stream_id)`. |
| Anchored position | A position is an integer cell index plus an `f32` offset inside a cell of side 8 m (§27.3). |
| Tier names | `referee`, `batch-cpu`, `gpu-cuda`, `gpu-metal`. |
| Identities | `DesignId`, `ModelId`, `SceneId`, `KernelId`, `PolicyId`, `EpisodeId` are content hashes (BLAKE3, `fs-blake3`). |

## 10. Lemma: proofs and certificates

### 10.1 What the two proof tools can do today

FACT, from the audit of `franken_lean` and `frankensympy`:

| Capability | FrankenSymPy | FrankenLean |
|---|---|---|
| Exact polynomial identity checking with an offline verifier | Yes (`fsym-proof-kernel`, `fsym-capsule-consumer`) | Not natively |
| Interval certificates: roots in a box, enclosures, sign over a box | Yes, up to 16 coordinates and 1,024 parameter cells | No |
| Exact rational linear algebra with certificates (LU, QR, LDLᵀ) | Yes | No |
| Gröbner bases, ideal membership | Yes: Buchberger over ℚ and 𝔽ₚ, cap of 1,024 basis polynomials | No |
| Lie algebras, quaternions, noncommutative symbols, group theory | None in Rust | — |
| Elaborating Lean source with Mathlib tactics | — | No: `MetaM`/`TacticM` are not wired; no Mathlib tactic has run |
| Re-checking compiled `.olean` files with an independent kernel | — | Yes: Lean stdlib 2,433 of 2,433 modules; Mathlib 9,131 of 11,083 accepted, 0 failed |
| Mathlib modules a robotics proof would want | — | Accepted: `Algebra.Lie.Basic`, `Algebra.Lie.OfAssociative`, `Algebra.Quaternion`, special linear and unitary groups, permutation groups. Blocked: `Geometry.Manifold.Algebra.LieGroup`, `Analysis.Normed.Algebra.MatrixExponential`, `Analysis.Quaternion`, all representation theory |
| Sealed offline proof bundles | Proof capsules | `fln seal-capsule` / `verify-capsule --require-semantic` |
| Bit-vector decisions with checked proofs | — | Yes (`Engine::decide_bv`) |

Two consequences shape this chapter. First, **the algebraic route**: formulate what we want certified as
polynomial or rational identities (Cayley maps, quaternion norms, adjoint matrices), because that is what both
tools can check now; analytic statements about exp, log and manifolds wait. Second, **Lean is a second opinion,
not a prover we drive**: proofs are written in stock Lean with Mathlib as a development-time tool, and
FrankenLean re-checks the compiled result with its independent kernel when the proof's dependency closure avoids
the blocked modules.

### 10.2 Proof classes

| Class | What it means | Produced by |
|---|---|---|
| `Proved` | A general statement accepted by a Lean kernel, and re-checked by FrankenLean when its closure permits. | `lean/` plus `fln verify-capsule` |
| `Certified` | An instance-level statement (this kernel, this model, this box) accepted by a checker with an offline-verifiable certificate. | FrankenSymPy proof kernel, interval certificates, exact rational certificates, the KIR dimension checker |
| `Tested` | A property held on a stated set of cases. | Gauntlet tiers G0–G5 |
| `Estimated` | A number from a model or a statistical procedure, with its uncertainty. | Everything else |

A report never prints `Proved` or `Certified` without the certificate's content hash and the checker's identity.
"Verified" is not a class and the word is not used for results.

### 10.3 Theorem cards

Each card is a statement the project wants, its route, and its tier. Status for every card today is HYPOTHESIS:
none has been checked.

| Card | Statement | Route | Tier |
|---|---|---|---|
| T-01 | For skew-symmetric Ω, cay(Ω)ᵀ·cay(Ω) = I and det cay(Ω) = 1. | Polynomial identity in the entries of Ω (`Certified`); general statement in Lean (`Proved`) | [S] / [F] |
| T-02 | The SE(3) Cayley update maps the group to itself. | Polynomial identity | [S] |
| T-03 | The quaternion update preserves the norm: ‖q·c‖² = ‖q‖²·‖c‖². | Polynomial identity; Lean through `Mathlib.Algebra.Quaternion` | [S] / [F] |
| T-04 | Ad(gh) = Ad(g)·Ad(h), and the coadjoint is the inverse transpose. | Polynomial identity in group coordinates | [S] |
| T-05 | The power pairing is invariant under a change of frame. | Polynomial identity | [S] |
| T-06 | The Cayley discrete Euler–Poincaré step for a free rigid body conserves spatial momentum. | Identity modulo the update equations (ideal membership) | [F]; budget is a HYPOTHESIS |
| T-07 | The friction-cone projection P satisfies P(x) ∈ 𝒦, P(x) − x ∈ 𝒦*, and ⟨P(x), P(x) − x⟩ = 0. | Per-case polynomial identities and inequalities | [F] |
| T-08 | The fast-tier contact objective has a positive-semidefinite Hessian. | Structural: a sum of positive-semidefinite terms; Lean | [F] |
| T-09 | Every kernel is dimensionally homogeneous. | Decided exactly by the KIR type checker | [S] |
| T-10 | A model's derived step kernels are equivariant under each generator of its symmetry group. | Polynomial identity per generator, or exact-rational randomized identity testing with a Schwartz–Zippel bound | [F] |
| T-11 | A joint's screw basis is, or is not, closed under the bracket, and its Hervé class is as reported. | Exact rational linear algebra | [S] |
| T-12 | Monotone co-design on finite chains has a least fixed point and Kleene iteration reaches it. | Lean (order theory) | [F] |
| T-13 | Discrete Noether for left-invariant discrete Lagrangians on a matrix Lie group. | Lean, general | [M] |
| T-14 | An exported fixed-point controller has no overflow on its stated input range. | FrankenLean `decide_bv` | [M] |

### 10.4 Mechanics

- **Instance certificates** are produced at build time by `derive/frob-derive` and checked by FrankenSymPy. Each
  is stored beside the kernel as `K-<name>.cert` with the kernel's content hash inside. `frob prove verify`
  re-checks certificates offline.
- **Lean theorems** live in a Lake project under `lean/`, pinned to the Lean version and Mathlib commit of
  FrankenLean's own pin (FACT: Lean v4.32.0). Stock Lean compiles them; FrankenLean's `check-olean` and
  `seal-capsule` re-check and seal. Neither tool is in the production dependency graph, and build outputs are not
  committed.
- **Constraints that are facts today.** FrankenLean's suite lock certifies only `x86_64-unknown-linux-gnu`, and
  this Mac has no Lean 4.32.0 toolchain installed, so Lean work runs on a Linux host. A full Mathlib re-check
  peaked at 92.8 GiB; a single theorem's closure is far smaller, and its measured size is recorded before any
  claim (OPEN-LEMMA-1).

### 10.5 What Lemma does not claim

- It does not claim the simulator is verified. It claims that named kernels satisfy named identities.
- It does not claim floating-point results satisfy the identities exactly. A certificate is about the SLP over
  the rationals with square roots; the floating-point gap is measured by tests, not proved.
- It does not claim any Mathlib result about Lie groups, matrix exponentials or representation theory has been
  re-checked by FrankenLean, because those modules are blocked today.

### 10.6 Gates

| Gate | Threshold (TARGET) |
|---|---|
| GATE-LEMMA-00 | T-01, T-03 and T-04 certified by FrankenSymPy for the M0 kernels; certificates replay offline. |
| GATE-LEMMA-01 | At least 80% of derived kernels carry a replayable instance certificate for each identity their manifest declares (BET-11). |
| GATE-LEMMA-02 | At least three Lean theorem cards compiled by stock Lean, re-checked by `fln`, and sealed; the capsule verifies from an empty environment. |
| ✖ KILL-LEMMA | Trigger: GATE-LEMMA-01 below 50% after one repair cycle. Fallback: certificates become exact-rational randomized identity tests recorded as `Tested`, and the word "certificate" is removed from kernel manifests. |

---
# PART III. THE MACHINE, END TO END

Each chapter follows one template: what the subsystem owns, the decisions with their reasons, the design,
refusals, proof obligations, gates, and open questions. Type sketches are in Appendix I.

## 11. Marrow: the algebraic core (`frob-lie`, `frob-algebra`)

### 11.1 What it owns

`frob-lie` owns the Lie-theoretic vocabulary every other crate speaks: the `LieGroup` and `LieAlgebra` traits;
the groups SO(2), SO(3), SE(2), SE(3), SE₂(3) and SE₂₊ₖ(3); product groups; joint subgroups described by a screw
basis with their Hervé class (C-01); the two retractions and their derivatives (dexp, dcay and inverses); and
frame-typed transforms, twists and wrenches (C-02).

`frob-algebra` owns the non-Lie algebra: finite groups as signed permutations with closure, conjugacy classes,
character tables, isotypic projectors and the Reynolds operator (C-09); the `Cone` trait with the nonnegative
orthant, second-order and positive-semidefinite cones (C-06); posets and antichains for co-design (C-12); the
pseudo-inertia cone with its affine-invariant operations (C-03); and max-plus matrices, taken from
`fs-tropical` through the airlock (C-10).

### 11.2 Decisions

- **ADR-MARROW-01: reuse `fs-ga` as the host representation.** `So3`, `Se3`, `Twist`, `Wrench` and `Mat6` are
  FrankenSim's, with deterministic elementary functions already routed through `fs_math::det` (FACT). FrankenSim
  itself has two vector vocabularies today, because `fs-mbd` defines its own `Vec3`, `UnitQuaternion` and `Pose`
  beside `fs-ga`'s (FACT). FrankenRobotics does not add a third. `frob-lie` adds traits, the groups `fs-ga`
  lacks, and frame typing.
- **ADR-MARROW-02: Cayley on the batched tiers, exponential on the referee.** Both retractions exist on every
  tier and the choice is part of `ModelId`. Reason: C-05. The Cayley map is refused when |ω|h exceeds 0.2 per
  substep (FROB-REF-212), which the step planner avoids by substepping.
- **ADR-MARROW-03: a revolute coordinate is a unit complex number.** The update is
  z₊ = z · ((1 − a²/4) + i·a)/(1 + a²/4) with a = ωh, followed by one normalization. A turn counter is updated
  by a deterministic branch rule. Limits are tested on the half-angle tangent t = s/(1 + c), which is monotone in
  the angle, plus the turn count. Reason: no trigonometric call in the hot path, and observations in (cos, sin)
  form are what policies consume anyway.
- **ADR-MARROW-04: packed single-precision layouts are generated, not typed.** There is no `Se3f32` struct.
  `frob-kir` emits structure-of-arrays layouts from the same algebraic description, so the host type and the
  kernel layout cannot disagree.

### 11.3 Laws

Each law is a G0 property test over random inputs and a boundary set (identity, rotations by π, tiny angles,
large translations). Tolerances are in units in the last place (ulp).

| Law | Statement |
|---|---|
| Group axioms | Associativity, identity, inverse for every group. |
| Retraction | τ(0) = identity; dτ(0) = identity; τ(x)·τ(−x) = identity for cay and exp. |
| Round trip | log(exp(x)) = x inside the injectivity radius; cay⁻¹(cay(x)) = x. |
| Adjoint | Ad(gh) = Ad(g)·Ad(h); Ad(exp x) = exp(ad x) to tolerance; coAd is the inverse transpose. |
| Pairing | ⟨w, ξ⟩ is invariant under a change of frame. |
| Jacobians | dexp and dcay match `fs-ad` dual-number derivatives. |
| Subgroups | A classified screw basis is bracket-closed; the generated joint map composes like its subgroup. |
| Representations | ρ(gh) = ρ(g)·ρ(h); characters are orthonormal; projectors are idempotent and sum to the identity. |
| Cones | P(P(x)) = P(x); Moreau decomposition x = P_𝒦(x) + P_𝒦°(x) with the parts orthogonal. |

### 11.4 Proof obligations, gates, open questions

| ID | Statement | Verification |
|---|---|---|
| PO-MARROW-01 | `frob-lie` conventions equal `fs-ga` conventions (quaternion order, sign rule, twist order, adjoint direction). | A fixture of 1,000 transforms produced by `fs-ga` and consumed by `frob-lie`, bit-compared. |
| PO-MARROW-02 | Frame typing has zero run-time cost in batched kernels. | Emitted kernel source contains no frame identifiers. |
| PO-MARROW-03 | The turn counter of ADR-MARROW-03 never disagrees with an angle integrated in `f64`. | Property test over 10⁹ random steps. |

| Gate | Threshold (TARGET) |
|---|---|
| GATE-MARROW-01 | All laws pass on 10⁶ cases each: at most 64 ulp in `f64`; at most 16 ulp in `f32` for single operations. T-01, T-03, T-04 certified (GATE-LEMMA-00). |
| GATE-MARROW-02 | Character tables and isotypic projectors correct for Cₙ and Dₙ with n ≤ 12, their products with C₂, and S₄, against tables on file. |

- **OPEN-MARROW-1.** Whether to canonicalize quaternion sign in stored state. Default: no. State stays
  continuous; observations use sign-invariant features such as projected gravity. Decide at GATE-MARROW-01.

## 12. Skeleton: the design language and the model compiler (`frob-design`, `frob-model`, `frob-io`)

### 12.1 The design term

A design is a term, not a file of numbers. Its leaves are components and its internal nodes are constructors.

- **Components.** `Body` (a Region with a material), `Joint` (a screw basis with limits), `Actuator` (a card
  reference, §13), `Transmission`, `Battery`, `Sensor` (§17), `Wheel`, `Track`, `Foot`, `Payload`. Each exposes
  typed ports (C-12).
- **Constructors.** `attach(parent_port, child)`, `serial(k, link)`, `mirror(plane, limb)`, `radial(n, axis,
  limb)`, `scale(factor, sub)`, `replace(path, component)`, `closure(port_a, port_b)` for kinematic loops.
- **Parameters.** Every parameter has a unit, bounds, a manifold kind (ℝ, ℝ₊, S¹, S², SE(3), the inertia cone),
  and a provenance: `designed`, `datasheet`, `measured` or `assumed`.

A design is stored canonically as JSON with schema `frob.design/1`, and its `DesignId` is the hash of the
canonical form. A fragment:

```json
{ "schema": "frob.design/1", "name": "quad-a",
  "root": { "body": "torso", "region": {"box": [0.42, 0.20, 0.10]}, "material": "al-6061" },
  "limbs": { "mirror": "xz", "of": { "mirror": "yz", "of": {
      "attach": "torso.hip_mount",
      "chain": [
        { "joint": {"class": "R", "axis": [1,0,0], "limits_deg": [-45, 45]},  "actuator": "qdd-m" ,
          "body": {"capsule": [0.03, 0.06]} },
        { "joint": {"class": "R", "axis": [0,1,0], "limits_deg": [-90, 150]}, "actuator": "qdd-m",
          "body": {"capsule": [0.025, 0.21]} },
        { "joint": {"class": "R", "axis": [0,1,0], "limits_deg": [-160, -30]}, "actuator": "qdd-m",
          "transmission": {"belt": 1.55}, "body": {"capsule": [0.02, 0.21]} },
        { "foot": {"sphere": 0.022, "material": "rubber-60a"} } ] } } },
  "battery": "li-6s-5ah", "sensors": ["imu:torso", "encoders:*", "foot-contact:*"] }
```

The two nested `mirror` constructors make the symmetry C₂ × C₂ a property of the term, not something to be
rediscovered.

### 12.2 Typed edits

Edits are the morphisms of the design category. Each has preconditions, an inverse where one exists, and a
declared effect on the symmetry group.

| Edit | Example | Symmetry effect |
|---|---|---|
| `set-param` | Thigh length 0.21 → 0.24 m | Preserved when applied under a constructor |
| `regear` | Knee belt ratio 1.55 → 1.8 | Preserved |
| `swap-actuator` | `qdd-m` → `qdd-l` at all knees | Preserved; broken if applied to one leg |
| `add-limb` / `remove-limb` | Add a tail with one joint | Reported: new group computed |
| `move-mount` | Hip mount 2 cm forward | Reported |
| `reshape` | Shank capsule → tapered capsule | Preserved under a constructor |
| `mirror` / `radial` | Wrap a limb in a constructor | Group grows |
| `closure-add` | Add a parallel knee linkage | Reported |

A design's history is a path of edits in a content-addressed lineage graph (§22.6).

### 12.3 The compile pipeline

`Design` → **elaborate** (expand constructors, resolve parameters) → **validate** (ports, units, bounds) →
**five functors** (C-12): kinematic tree with loop closures; collision primitives and inertia from Regions and
materials; actuator and sensor binding; budget relations; symmetry generators → **`RobotModel`**, immutable, with
a `ModelId`. The model then lowers three ways:

1. to an `fs_mbd::articulated::ArticulatedModel` for the referee tier, through the airlock;
2. to packed structure-of-arrays buffers for the batched tiers, with depth levels and a path table for sparse
   Jacobians;
3. to analysis views for Atelier (symbolic kinematics for one limb, the leg-permutation group, the budget graph).

Compilation is memoized per subterm by content hash, so an edit recompiles only what it touched (INV-SKEL-02).

### 12.4 Refusals instead of repairs in silence

An invalid design is refused with a stable code, a message, and ranked repairs expressed as typed edits. Examples:
`FROB-REF-101` port type mismatch; `FROB-REF-104` inertia not physically consistent (the smallest eigenvalue of
the pseudo-inertia is reported); `FROB-REF-107` empty joint range; `FROB-REF-110` a closure that over-constrains
the mechanism (mobility below one); `FROB-REF-113` an actuator card with a required field of class `assumed`
when the caller asked for `--strict-data`.

### 12.5 Import and export (`frob-io`)

- **URDF and MJCF import**, through a small in-house XML reader (elements and attributes; no entities or DTDs),
  so no third-party parser enters the core (L-5). FrankenSim's own robot models are hand-transcribed from pinned
  URDF sources and do not parse at run time (FACT), so this reader is new work.
- **Every import produces a loss report**: fields dropped, defaults filled, contact parameters that have no exact
  equivalent, inertias that fail C-03 with the proposed repair, actuator limits absent and filled from a card
  marked `assumed`.
- **Export** to URDF and MJCF for use elsewhere, with the same loss report in reverse. glTF export for visuals
  goes through FrankenSim's GLB exporter (FACT: `fs-geom` exports GLB).
- **OpenUSD** import is Frontier. Only the core specification is settled and robotics schemas are still vendor
  extensions (FACT, Appendix H), so it waits for a real consumer.

### 12.6 Templates and reference robots

Templates are designs with parameters, shipped in `assets/templates/`. Reference robots are imported public
models used as parity fixtures and benchmark baselines (licenses in Appendix G).

| Class | Templates | Reference robots |
|---|---|---|
| Quadruped | 12-DoF mammal layout; sprawled layout | Unitree Go1, Go2, A1; ANYmal B, C; Spot (BSD-3 models) |
| Hexapod and other multi-legged | 18-DoF radial hexapod; 8-leg variant | None with a permissive model on file (OPEN-ARENA-2) |
| Biped and humanoid | Point-foot biped; 29-DoF humanoid | Unitree G1 (already in `fs-mbd::robot_models`), H1; Cassie (MIT model) |
| Wheeled | Differential drive; Ackermann; mecanum; three-wheel omni | Stretch 3, TIAGo bases (Apache-2.0 models) |
| Tracked | Skid-steer with flippers | None on file |
| Hybrid | Wheel-leg quadruped; stair-climbing tri-wheel cluster (`fs-mbd::tri_wheel_cluster` has the geometry); rocker-bogie | None on file |
| Arm on a base | 7-DoF arm on a mobile base | KUKA iiwa 7 (already in `fs-mbd::robot_models`) |

### 12.7 Invariants, gates, open questions

| ID | Invariant |
|---|---|
| INV-SKEL-01 | A `RobotModel` is immutable and fully determined by its `DesignId`, the compiler version and the card versions it references. |
| INV-SKEL-02 | Incremental compilation equals full compilation, bit for bit. |
| INV-SKEL-03 | Every inertia in a compiled model passes C-03, and every parameter has a unit and a provenance. |

| Gate | Threshold (TARGET) |
|---|---|
| GATE-SKEL-01 | Each reference robot imports with forward-kinematics parity against the oracle within 10⁻⁹ m and 10⁻⁹ rad over 10⁴ configurations, and its loss report lists every dropped field found by a field-coverage diff. |
| GATE-SKEL-02 | Functor laws and INV-SKEL-02 hold on 10⁴ random designs and edits. |
| GATE-SKEL-03 | Each template compiles, passes `design check` with no refusal, and stands under its baseline controller (with GATE-REFLEX-01). |

- **OPEN-SKEL-1.** Whether designs should also have a compact text syntax, or be FrankenScript modules
  (`fs-ir` has isomorphic s-expression and JSON syntaxes, FACT). Default: JSON only for v1. Owner: project lead.
  Decide before M3.

## 13. Sinew: actuators, transmissions and power (`frob-actuate`)

### 13.1 Why actuators are first-class

Recent sim-to-real studies agree on where the gap lives: actuator dynamics. Identifying armature, joint friction
and delay gave 46% more travel distance and 75% less rotational deviation in one 2026 study; static friction is
commonly missing from randomization; joint friction and actuator delay were called particularly critical for a
point-foot biped (FACT, Appendix H). A simulator with exact rigid-body dynamics and an ideal torque source is
precise about the wrong thing.

### 13.2 The actuator card

An `ActuatorCard` is immutable, content-addressed data: torque constant, back-EMF constant, winding resistance
and inductance, rotor inertia, gear ratio and efficiency map, Coulomb, viscous and Stribeck friction, peak and
continuous torque, speed limit, bus voltage and current limits, thermal resistances and capacitances, mass,
envelope geometry, encoder resolution, driver control rate and latency. **Every field carries a provenance**
(`datasheet` with a citation, `measured`, `generic`, `assumed`) and an uncertainty. The starter catalog contains
generic classes (small, medium and large quasi-direct-drive; hobby servo; strain-wave; hub motor) marked
`generic`. Real parts enter only with a cited datasheet (OPEN-SINEW-1).

### 13.3 The model ladder

Each level is a Component with shaft, electrical and thermal ports. The level is part of `ModelId`.

| Level | Contents | Tier |
|---|---|---|
| L0 | Ideal torque source. Used only by mathematical tests. | [S] |
| L1 | Torque-speed envelope from the bus: i ≤ min(I_max, (V_bus − k_e·N·ω)/R); output torque η·N·k_t·i minus friction; reflected inertia N²·J_rotor added to the joint; first-order delay; position-velocity loop at the driver rate. | [S] |
| L2 | Winding current dynamics, a two-node thermal network with torque derating, battery voltage sag and state of charge. | [S] |
| L3 | Backlash, series elasticity as an added degree of freedom, cable and belt compliance. | [F] |
| L4 | Hydraulic actuation. | [M] |

L1 is the default for training because the literature above says it is the minimum that transfers, and because
reflected inertia also conditions the contact solve (it adds to the diagonal of A in C-06).

### 13.4 Energy at the ports

Because an actuator is a port component, its balance is checked every step: electrical energy in equals
mechanical work out plus copper loss i²R plus friction loss plus gear loss plus the change in stored magnetic and
thermal energy. The balance residual is recorded, and it is what makes the energy ledger of §23.4 exact instead
of estimated.

### 13.5 Foundations, gates, open questions

FACT: FrankenSim has `fs-em` (about 255k lines), `fs-thermal`, `fs-tribo` (Stribeck, LuGre, rolling loss),
`fs-control` (motor current and speed loops) and a lumped `dc_motor` system example. `frob-actuate` reuses
their laws on the referee tier through the airlock and re-derives the L1 and L2 arithmetic as kernels for the
batched tiers.

| Gate | Threshold (TARGET) |
|---|---|
| GATE-SINEW-01 | Known answers within 1%: stall torque, no-load speed, electrical and mechanical time constants of a DC motor step, thermal steady-state rise ΔT = P·R_th. |
| GATE-SINEW-02 | Port energy balance closes to 10⁻⁹ relative on the referee and 10⁻⁴ relative per episode on single-precision tiers. |
| GATE-SINEW-03 | Batched L1 and L2 kernels agree with the referee components within 10⁻⁴ relative on a 10⁴-case battery. |

- **OPEN-SINEW-1.** Which real actuator parts ship in the catalog, and under what data license. Default: generic
  classes only. Owner: project lead. Blocks nothing before M4.

## 14. Kinesis: dynamics on three tiers (`frob-dyn`, `frob-kernels`)

### 14.1 One model, three tiers

| Tier | Purpose | Arithmetic | Foundation | Worlds |
|---|---|---|---|---|
| **Referee** (`referee`) | The most trusted answer we can compute. Adjudicates the others. | `f64`, deterministic elementary functions, exact Coulomb friction, certified collision queries | `fs-mbd`, `fs-contact`, `fs-time`, `fs-query` through the airlock | One to a few dozen |
| **Batch CPU** (`batch-cpu`) | The permanent reference for the batched algorithm. Runs anywhere, including where there is no GPU. Debugging target for a teleported world. | `f64` or `f32`; the same derived kernels and the same stage order as the GPU | `frob-kernels` in Rust on `fs-exec` tiles | Hundreds to thousands |
| **GPU** (`gpu-cuda`, `gpu-metal`) | Throughput for training and for design sweeps. | `f32` | The same kernels in the shared dialect (§20) | Thousands to tens of thousands per device |

The referee is not called ground truth. Its outputs carry the evidence class FrankenSim's contracts give them, and
Reality is a separate claim (§6).

### 14.2 The referee tier as it can exist at M1

FACT: `fs-mbd` provides floating-base forward dynamics and a contact step
(`articulated_contacts::step::step_articulated_frictional_contacts`) that is first-order kick-contact-drift, takes
the contact set from the caller, and states that "independent endpoint gap checks are not continuous collision
detection". A 29-DoF G1 rollout at 480 Hz with Hunt–Crossley feet and `fs-tribo` friction already runs
(`frankensim/crates/fs-cmaes-viz-wasm/src/g1_walking.rs`). The M1 referee is therefore: FrankenSim's articulated
dynamics and contact solve, plus contact-set generation that `frob-contact` supplies from `fs-query` convex
separation and signed-distance queries, plus an audit pass with certified continuous collision detection on
sampled windows. It improves as FrankenSim lands its persistent steppers (asks FD-FSIM-02 and FD-FSIM-04).

### 14.3 The batched step

Tiers `batch-cpu` and `gpu-*` execute the same ten stages in the same order. Every stage is a named kernel or a
short chain of kernels (§20.2), batched over worlds, with no data-dependent control flow across worlds.

| # | Stage | Kernels | Parallel shape |
|---|---|---|---|
| 1 | Kinematics: link poses and twists down the tree | `K-fk-level`, `K-twist-level` | Worlds × links at one depth |
| 2 | Collision geometry: world poses of primitives and sample points | `K-geom-pose` | Worlds × primitives |
| 3 | Broad phase for dynamic pairs | `K-bp-sort`, `K-bp-pairs` | Per world, deterministic sort |
| 4 | Narrow phase: primitive pairs; samples against the scene distance field | `K-np-prim`, `K-np-sdf` | Worlds × candidates |
| 5 | Contact compaction: count, exclusive scan, write | `K-compact` | Fixed capacity per world, overflow flagged |
| 6 | Dynamics terms: mass matrix, bias forces, actuators, passive forces | `K-crba-level`, `K-rnea-level`, `K-act-l1`, `K-passive` | Worlds × links |
| 7 | Constraint rows: contacts, limits, closures, cones | `K-jac-rows`, `K-cone-setup` | Worlds × rows |
| 8 | Solve: Newton on the convex objective with Cholesky and exact line search | `K-hess`, `K-chol`, `K-linesearch`, `K-cone-eval` | Worlds in lockstep under masks |
| 9 | Integrate on the group | `K-cay-step`, `K-normalize` | Worlds × joints |
| 10 | Sensors, observations, rewards, masked resets | `K-sense-*`, `K-obs`, `K-reset` | Worlds |

Design rules for the step:

- **Fixed shapes.** Every buffer has a capacity fixed at environment creation. A world that overflows its contact
  capacity is marked `Degraded` with a reason; it is never undefined behaviour and never a silent truncation.
  The caller chooses refuse, reset or continue-flagged.
- **Lockstep solve.** All worlds run the same number of Newton iterations under a per-world `done` mask. This
  keeps the launch graph static (§20.5) and makes iteration counts part of the model identity.
- **No floating-point atomics.** Compaction and reductions use count-scan-write and fixed-order trees (§27.2).
- **The mass matrix is small and dense per world.** For the robots in scope (18 to about 60 degrees of freedom)
  H = A + JᵀGJ is factored by a batched dense Cholesky. Tree-sparse factorization is Frontier for larger
  mechanisms.

### 14.4 Integrators

- **Default, [S]:** semi-implicit Euler. Stage 8 returns v₊; stage 9 sets q₊ = q·τ(h·v₊). Joint damping and
  reflected inertia are implicit in A. Default physics step h = 2 ms with control every tenth step
  (OPEN-KIN-1).
- **Structure-preserving, [F]:** a linear-time variational integrator for articulated bodies in reduced
  coordinates on `referee` and `batch-cpu`, for long-horizon energy studies and passive-dynamics work.
- **Substeps** are integer ratios of the control tick and are fixed at admission; they enter `ModelId`
  (the same rule as the Wright Flyer plan §4.3, FACT).

### 14.5 The trust ladder and the fidelity ledger

**The fidelity ledger** is a table, keyed by (model class, scene class, quantity, tier pair, kernel versions), of
the distribution of disagreement between a faster tier and a slower one on the standard battery (§30.5).
Quantities include base displacement, yaw, stance impulse, slip distance, peak joint torque, energy per metre and
time of first failure.

**Promotion rule.** A design score or a policy return computed on a fast tier is labeled `provisional`. It
becomes `established` only after re-evaluation of a sample of episodes on a higher tier, with the difference
inside the promotion envelope recorded in the ledger for that quantity. The sample size is chosen by the
e-process of C-15, so clear cases promote after a handful of referee episodes. A result that fails promotion is
labeled `provisional: tier-sensitive` and reported with both numbers (NS-09).

This is FrankenSim's "speculate low, verify high" pattern and its Goodhart guard (addendum proposals 9 and D,
FACT), applied to robots.

### 14.6 Events, loops and limits of the claim

- **Events.** Fast tiers are time-stepping schemes with a velocity-scaled contact margin; they do not locate
  events. The referee's audit uses event location and certified collision detection
  (`fs-time::rk45_until_event`, `fs-contact::certified_ccd`, FACT) on sampled windows to count missed contacts.
- **Kinematic loops.** On fast tiers a closure is a compliant equality row in the convex solve, with its
  compliance stated [F]. Exact closures wait for FrankenSim's constraint lane (FD-FSIM-04).
- **No real-time claim.** Nothing here claims real-time or hardware-in-the-loop behaviour. Interactive viewing
  runs one world at presentation rate with physics unchanged.

### 14.7 Gates and open questions

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-KIN-01 | Known answer | Pendulum period within 10⁻⁶ relative at h = 1 ms; symmetric-top precession within 10⁻⁴; free-flight momentum conserved to 10⁻¹² relative over 10⁶ steps on the variational lane; ballistic invariants of C-04. |
| GATE-KIN-02 | Parity | Contact-free forward dynamics of each reference robot against the pinned oracle: joint accelerations within 10⁻⁹ relative in `f64`. |
| GATE-KIN-03 | Parity | `batch-cpu` in `f64` against `fs-mbd`: kinematics, inverse dynamics, forward dynamics and mass matrix within 10⁻¹⁰ relative on 10⁴ states per reference robot. |
| GATE-FID-01 | Fidelity | On the M2 battery, `batch-cpu` against `referee`: median and 95th-percentile disagreement at or below 2% and 5% for base displacement over 2 s, 3% and 8% for energy per metre, 5% and 12% for stance impulse. Thresholds are re-baselined once at M2 with a written rationale, then frozen. |
| GATE-FID-02 | Fidelity | The promotion rule is exercised end to end: a planted tier-sensitive policy is caught (NS-09). |

- **OPEN-KIN-1.** Default physics step and Newton iteration budget. Default: 2 ms, 4 iterations with warm start.
  Decided by the fidelity-throughput frontier measured at M2.
- **OPEN-KIN-2.** Whether sleeping and islands are worth their complexity for single-robot worlds. Default: no.

## 15. Traction: collision, contact, wheels, tracks and terrain (`frob-contact`)

### 15.1 Two kinds of geometry

- **Everything static is a distance field.** A scene compiles to a signed-distance field (dense bricks, with a
  sparse brick tree as Frontier), an optional heightfield for open terrain, and maps of material and friction
  (§16.2). A distance field answers "how far, and which way" in constant time for any query point, which is what
  thousands of worlds need.
- **Everything that moves is a union of convex primitives.** Spheres, capsules, boxes, cylinders and rounded
  boxes, with convex hulls as Frontier. A robot's collision primitives are fitted from its link Regions by the
  model compiler with a recorded fitting error, or authored.

FACT: FrankenSim's narrow phase is convex-against-convex for primitives through a certified Frank–Wolfe
separation in `fs-query`, with signed-distance pairs, heightfields and mesh manifolds refused as
`MissingCapability`. The batched collision path is therefore new work in `frob-contact`, checked against
`fs-query` where both apply (asks FD-FSIM-03).

### 15.2 Detection

- **Moving against static.** Each primitive carries sample spheres (a foot has four to eight; a wheel has rim
  samples chosen by curvature). A sample at point c with radius r is in contact when φ(c) − r is below the margin,
  with normal ∇φ(c). Field values are trilinear in `f32` with a stated interpolation error bound per brick.
- **Moving against moving.** Per world: a deterministic sort-and-sweep along a fixed axis or a uniform grid, then
  analytic pair tests. Self-collision pairs are a precomputed filter list from adjacency and nominal clearance.
- **Speculative margin.** The margin grows with approach speed, so fast feet do not tunnel thin geometry between
  steps. The referee's audit (§14.6) measures what is still missed.

### 15.3 Contact laws

`ContactLaw` is a trait whose implementations are convex potentials ℓ_c (C-06). The law is part of `ModelId`.

| Law | Contents | Tier |
|---|---|---|
| CL-1 | Linear compliant normal with a dissipation time; regularized Coulomb friction with a stated stiction speed. | [S], default |
| CL-2 | Hunt–Crossley normal law and regularized friction in convex form. | [F] |
| CL-M | A mapping of MuJoCo's soft-contact parameters, for importing reference models. The mapping is lossy and the loss report says so. | [S], parity only |

Contact parameters come from a card for the pair of surface materials and their condition (dry, wet, greasy,
dusty), following FrankenSim's rule that friction belongs to an interface system and not to an unordered pair of
materials (FACT: expansion charter §5.3).

### 15.4 Friction

Isotropic cones by default. Anisotropic elliptic cones for mecanum rollers, treads and skates. Torsional and
rolling resistance for feet and wheels as extra cone rows. The stiction speed of the regularized law is a
reported parameter, and creep under it is one of the quantities in the fidelity ledger. Stribeck curves are
Frontier.

### 15.5 Wheels, tracks and hybrids

- **Wheels.** A wheel is a body, a revolute joint and a cylinder or torus collider. Contact and friction do the
  rest, so slip, traction limits and climbing a threshold come from the physics and not from a kinematic rule.
- **Omnidirectional wheels.** Mecanum and omni wheels are an anisotropic friction ellipse aligned with the roller
  axis in the wheel frame. Individual rollers are not simulated.
- **Tire laws [F].** Brush and Fiala models as constitutive laws from slip ratio and slip angle to contact
  force, for vehicles where carcass compliance matters.
- **Tracks [F].** A track is a set of road wheels plus a belt modeled as a contact surface with prescribed
  tangential velocity and an anisotropic friction ellipse. A full link-chain track is a Moonshot.
- **Hybrids.** A wheel-leg robot or a stair-climbing wheel cluster is a composition of the above and needs no new
  mechanism (C-12).
- **The nonholonomic model** of C-07 is not what the simulator integrates. It is the planning model, the static
  analysis model, and the known-answer oracle for rolling.

### 15.6 Terrain mechanics

| Ground | Model | Tier |
|---|---|---|
| Rigid floor with a friction map (tile, concrete, wood, wet patch, grease) | CL-1 with per-texel interface cards | [S] |
| Compliant coverings (carpet, mats) | CL-1 with covering stiffness and a rolling-resistance row | [S] |
| Soft soil under wheels and tracks | Bekker–Wong pressure-sinkage and Janosi–Hanamoto shear | [F] |
| Granular media under legs | Resistive force theory | [F] |
| Terrain that deforms and stays deformed | Rut and footprint fields | [M] |

### 15.7 Known-answer battery

| Case | Truth |
|---|---|
| Block on an incline | Stays if tan α < μ and slides if tan α > μ; the measured threshold is within 1% of μ, and creep below it is at most the stated stiction speed. |
| Sphere rolling down an incline | Acceleration (5/7)·g·sin α without slip. |
| Disk rolling on a plane | v = ω·r within 0.1%; the nonholonomic trajectory of C-07. |
| Box stack, ten high | At rest for 10 s with drift below 1 mm. |
| Dropped body, inelastic law | Total mechanical energy never increases. |
| Capsule against a spherical distance field | Gap error at most the interpolation bound. |
| Mecanum base commanded sideways | Moves sideways with yaw below 1°. |
| Ring on a cone | Slides if and only if tan α > μ (the garment analogue in FrankenTextiles §11.9, FACT). |
| Painlevé rod | The referee reports the indeterminate configuration instead of returning a number. |

### 15.8 Gates and open questions

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-TRAC-01 | Known answer | Every row of §15.7 passes on `referee` and `batch-cpu`; rows marked for the fast law pass on GPU tiers within the stated creep. |
| GATE-TRAC-02 | Parity | Primitive pair distances and normals agree with `fs-query` within 10⁻⁹ m (`f64`) on 10⁶ random pairs. |
| GATE-TRAC-03 | Fidelity | Tunneling audit: on the M2 battery the fast tiers miss fewer than 1 contact per 10⁶ world-steps that the certified detector finds. |
| ✖ KILL-CONTACT | — | Trigger: GATE-FID-01 fails on slip or creep after parameter mapping and one repair cycle. Fallback: CL-2 becomes the default with a published lower throughput; if that also fails, locomotion claims are restricted to the referee tier and the fast tiers are labeled exploratory. |

- **OPEN-TRAC-1.** Dense bricks or a sparse brick tree for interior scenes at 1 cm resolution. Default: dense
  bricks with a coarse outer level; revisit with measured memory at M3.

## 16. Habitat: scenes and terrain (`frob-scene`)

### 16.1 The scene pack

A `ScenePack` is immutable and content-addressed (`SceneId`). It contains:

- a signed-distance field in bricks, quantized to 16 bits, at a fine level near surfaces and a coarse level
  elsewhere, plus an optional heightfield for open terrain;
- an interface map: for each surface region, a material and a condition (dry, wet, greasy, dusty), which select
  contact cards (§15.3);
- semantic labels: room type, object class, and a traversability class;
- **articulated fixtures** (doors, drawers, cabinet doors, swing gates, carts) as articulated models simulated by
  the same engine as the robot, because a door is a one-joint mechanism and needs no special case;
- movable objects (boxes, totes, chairs, pallets) as rigid bodies;
- moving surfaces (conveyors, escalators) as contact surfaces with prescribed tangential velocity, the same
  mechanism as a track (§15.5);
- spawn regions, goals and route graphs;
- a visual mesh in GLB for the viewer, never read by physics;
- provenance: a license manifest per asset, and for scanned scenes a scale label (`measured-anchor` or
  `estimated`) and an **unknown-region policy**: unobserved space is unknown, not free.

### 16.2 The scene compiler

Source geometry (F-rep constructive solids from generators, or meshes from import) → watertightness and
orientation checks → signed distance by FrankenSim's distance-field machinery (`fs-rep-sdf` tiled and sparse
fields; `fs-rep-mesh` generalized winding numbers; `fs-rep-frep` constructive solids — FACT) → bricks with a
recorded interpolation error bound per brick → interface and semantic maps → pack. Scene compilation is a CPU job
with the evidence FrankenSim's geometry kernel provides; a scene that fails the watertight check is refused
(`FROB-REF-301`), not patched.

Field sampling in kernels is **manual trilinear interpolation in the kernel source**, not hardware texture
filtering, because hardware filtering uses fixed-point weights that differ between vendors and would break the
determinism classes of §27.

### 16.3 Terrain generators [S]

Deterministic functions of (family, parameters, seed) with a scalar difficulty: flat ground, slopes, stairs
(rise and run), single steps, gaps, stepping stones, rough ground (band-limited noise), rubble (random boxes),
ramps and curbs. Families tile into a curriculum grid in the manner of the legged-locomotion benchmarks.

### 16.4 Interior generators

Each archetype is a typed attribute grammar: Building → Floors → Rooms → Zones → Fixtures → Clutter, with
dimensional constraints (door widths, aisle widths, counter heights, stair geometry) and a seed. Geometry is
emitted as constructive solids and compiled by §16.2. The box-and-fixture level is [S]; scenes populated from
asset libraries are [F].

| Archetype | What it contains | What it tests in a robot |
|---|---|---|
| Home | Rooms off a hallway, thresholds, rugs, stairs, furniture, small floor clutter | Step and threshold crossing, narrow doors, clutter avoidance |
| Apartment | Compact plan, tight turns, a lift lobby | Turning radius, maneuvering index (C-07) |
| Office | Open plan with desks, castor chairs, carpet, glass doors, meeting rooms | Movable obstacles, carpet rolling resistance, door opening |
| Warehouse | Long aisles, racking, pallets, totes, ramps, dock plates | Speed and energy per metre with payload, ramp traction |
| Manufacturing plant | Machine cells, fences, conveyors, floor grates, oil patches | Moving surfaces, low-friction patches, keep-out zones |
| Retail store | Shelving aisles, displays, checkout lanes, carts | Narrow aisles with movable carts, shelf reach |
| Commercial kitchen | Galley aisles, wet and greasy floor patches, drains, mats, swing doors, low shelves | Slip margin, swing doors, tight galley turns |

Moving people and vehicles are kinematic capsule actors on route graphs and are Frontier.

### 16.5 Assets

Only permissively licensed assets enter the default pipeline: fully procedural generators here; Infinigen
(BSD-3) as an offline external generator; Google Scanned Objects (CC-BY 4.0); RoboCasa assets (CC BY 4.0, with
MIT code) for kitchen fixtures. Research-only collections (HM3D, HSSD, PartNet-Mobility, BEHAVIOR-1K) are
supported as user-supplied imports and are never redistributed (FACT and licenses in Appendix G).

### 16.6 Ingesting real places [F]

The owner already has a pipeline from walkthrough video to an agent-built Blender twin with stable feature
identifiers and a GLB export, and an accepted plan for a twin package with linked visual, geometric and
traversability representations (FACT: the reconstruction skills and
`franken_surveillance_system/BLENDER_TWIN_INTEGRATION_PLAN.md`). `frob scene import` consumes that package and
adds what it lacks: the collision compile, interface cards (marked `assumed` unless measured), fixture
articulation, and the unknown-region policy. Real-home data is governed by §29.3.

### 16.7 Train and test hygiene

A `SceneSplit` separates training scenes from held-out scenes by seed, by parameter range and by archetype
variant. Arena checks mechanically that a reported evaluation score comes from scenes whose identifiers are
absent from the policy's training manifest (`FROB-REF-502` otherwise).

### 16.8 Memory arithmetic

Illustrative, to size the design (GB-07): a 10 m × 10 m × 3 m interior at 2 cm is 37.5 million samples, or 75 MB
at 16 bits. A 60 m × 40 m × 8 m warehouse at 4 cm is 300 million samples, or 600 MB, which is why two levels are
the default. A device holds K scene variants and every world indexes one; K is chosen by the budget estimator.

### 16.9 Gates and open questions

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-HAB-01 | Known answer | Compiled field against source geometry: error at most half a fine voxel at 10⁶ surface samples; analytic shapes exact to the interpolation bound. |
| GATE-HAB-02 | Known answer | For each archetype, 1,000 seeds yield valid scenes: all constraints satisfied and a traversable route between spawn and goal for the nominal footprint. The same seed gives the same `SceneId` on Linux and macOS. |
| GATE-HAB-03 | Reality | One real place ingested end to end, privately, with its scale label and unknown regions respected. [F] |

- **OPEN-HAB-1.** Which published building and workplace norms supply the dimension distributions. Default:
  generic ranges labeled `assumed`. Owner: project lead. Blocks GATE-HAB-02 for claims of realism, not for
  validity.

## 17. Sensorium: sensors and the observation service (`frob-sense`)

### 17.1 The rule

Observations come from a deterministic perception service that is part of the model identity. They never come
from a renderer, a viewer, presentation timestamps or level-of-detail state (the same rule the Wright Flyer plan
sets for its modeled pilot, FACT). A sensor is a Component: it reads ports and fields and emits a typed signal
with its noise, delay and rate.

### 17.2 Proprioception [S]

- Joint encoders with the card's resolution and noise; velocity by the estimator the real driver uses.
- Motor current and estimated torque; winding and housing temperature; battery voltage and current.
- IMU: specific force and angular rate in the sensor frame with bias random walks, noise densities, scale and
  misalignment, sample rate and latency.
- Foot contact switches with threshold and hysteresis; force-torque sensors, which are a port reading.

### 17.3 Exteroception [S]

Depth cameras, spinning and solid-state lidar patterns, height scans around the base, and semantic and instance
labels are all ray queries: sphere tracing against the scene distance field plus analytic ray tests against the
moving primitives. Noise models cover range-dependent noise, grazing-angle dropout and range limits. Because the
ray kernels are ordinary kernels (§20), sensor outputs have the same determinism class as the physics.

### 17.4 Images

- **Stills for agents [S].** `frob telemetry stills` draws contact sheets (several views at key frames) with the
  same ray kernels, shaded simply, together with an identifier buffer. `frob telemetry inspect-pixel` maps a
  pixel to a link, a fixture or a contact. A vision-capable agent can look and then ask (the loop
  FrankenTextiles uses for garments, FACT).
- **RGB cameras for policies [F].** Rasterized textured meshes. Not needed by any task before M5.
- **Hero renders [F].** FrankenSim's spectral path tracer, offline only (its `tracer` feature is Frontier-gated
  and off by default, FACT).

### 17.5 The viewer

A Three.js r186 page that consumes telemetry snapshots. Replays are exported as self-contained HTML. Where the
page stays inside FrankenThreeD's admitted feature subset it is packaged through FrankenThreeD's build route
(`node tools/ingest/cli.mjs app.html --build-app out --pack-html replay.html`); otherwise it ships as plain
Three.js. FACT about FrankenThreeD today: phase 0/1, a general WebGPU route that matches pinned Three.js pixels
for ten scenes on a software Vulkan driver, numeric-island specialization only, and no measured speedup, so it is
a packaging route and a future accelerator, not a dependency. The viewer never feeds anything back to physics: the
simulator owns state and the viewer reads immutable snapshots.

### 17.6 Gates

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-SENS-01 | Known answer | Depth against analytic scenes within 1 mm plus the interpolation bound; an IMU at rest reads (0, 0, +g) and zero rate; on a turntable it reads the table rate; lidar beam geometry matches its pattern definition. |
| GATE-SENS-02 | Fidelity | Sensor outputs agree across backends inside their determinism class. |
| GATE-SENS-03 | Known answer | Noninterference: turning the viewer, stills or telemetry level on or off leaves every physics digest unchanged. |

## 18. Reflex: estimation and model-based control (`frob-control`)

### 18.1 Why it exists

Training a policy for every candidate design makes the design loop hours long. Reflex synthesizes an estimator
and a baseline controller from the model in seconds, so a new design can be simulated at once (GOAL-13). The
baseline is for feedback, not for peak performance.

### 18.2 The estimator [S]

A contact-aided invariant extended Kalman filter on SE₂₊ₖ(3) with IMU biases: inputs are the IMU, the encoders
and the contact switches; outputs are base pose and velocity with covariance (C-13). Wheeled bases use wheel
odometry with the IMU on the same filter structure. Every evaluation can run with `--state estimated` or
`--state privileged`, and the difference is reported, because a controller that only works on privileged state
is not a result.

### 18.3 Baseline controller synthesis

`frob control synth <design>` detects the morphology class from the model and assembles a controller.

| Class | Controller | Tier |
|---|---|---|
| Legged | A gait from the atlas (C-10) chosen by leg count and target Froude number; footstep placement by a capture-point heuristic; stance forces by a single-step force-distribution program over friction cones; swing by Cartesian PD through the leg IK kernel; joint PD at the driver | [S] |
| Legged, predictive | Centroidal single-rigid-body model-predictive control over the cones of C-06 | [F] |
| Biped balance | LQR on the linearization about a stand, then the legged controller | [F] |
| Wheeled | Wheel speeds from the commanded body twist through the kinematic connection (C-07), traction-limited acceleration, path tracking through flat outputs | [S] |
| Tracked | Skid-steer with a slip estimate | [S] |
| Hybrid | Roll where the route graph allows, step where it does not | [F] |
| Arm on a base | Operational-space control with the base as a floating frame | [F] |

Gains come from the model: joint-space natural frequencies from the linearization and the reflected inertia, not
from a table.

### 18.4 One cone solver

The force-distribution and predictive programs are small dense conic programs (tens to a few hundred variables).
They are solved by the symmetric-cone solver that `frob-algebra` provides to the contact solve, compiled as
kernels so the baseline controller runs batched on any tier.

### 18.5 Reporting

`control synth` returns one of three outcomes, never a bare failure: `not-synthesizable` with the reason (for
example no gait in the atlas satisfies static stability with these leg counts), `synthesized-failed` with the
failure taxonomy of §23.5, or `ok`. A design that fails under its baseline is not thereby a bad design, and the
report says that.

### 18.6 Gates and open questions

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-REFLEX-01 | Known answer | Every template stands for 10 s and traverses 5 m of flat ground under its baseline controller on `referee` and `batch-cpu`. |
| GATE-REFLEX-02 | Performance of the method | BET-06: at least 80% of corpus designs that pass `design check` traverse 5 m. |
| GATE-REFLEX-03 | Known answer | Filter consistency inside χ² bounds on the simulated IMU; position drift under 2% of distance on a flat trot. |

- **OPEN-REFLEX-1.** Whether the predictive controller runs on device. Default: the single-step program runs on
  every tier; the predictive controller runs on CPU tiers only until measured.

## 19. Cortex: the environment API and learning (`frob-learn`)

### 19.1 The environment API

`VecEnv` exposes `reset(mask)`, `step(actions)`, `snapshot(worlds)` and `restore(snapshot)` over device-resident
buffers. Semantics follow the common vectorized conventions: per-world `terminated` and `truncated` flags, masked
automatic reset, and observations for the post-reset state. Host code receives views; nothing is copied unless
asked. `snapshot` and `restore` are what make replay and teleport possible (§23.3).

### 19.2 Declarative task specifications

A task is data with a `TaskId`: observation terms (each with unit, noise, delay, and the representation under
which it transforms, C-09), action terms (joint position targets through the driver loop by default; torque or
velocity as alternatives), reward terms with units and explicit normalization, termination conditions,
randomization and curriculum. Declaring the representation of each term is what lets the framework build
equivariant networks, generate mirrored data, and test declared invariances mechanically.

### 19.3 Algorithms and networks

- PPO with generalized advantage estimation, value normalization and an adaptive step size [S].
- SAC, and teacher-student distillation from privileged to proprioceptive observations [F].
- Behaviour cloning from the baseline controller as a warm start [F]: Reflex supplies demonstrations for free.
- Networks: MLP and GRU [S]; equivariant MLP by isotypic projection, or augmentation with a symmetry loss as the
  solid alternative [F]/[S]; transformers [M].

### 19.4 Where learning runs, and what FrankenTorch is for

FACT about FrankenTorch today: tensors execute on the CPU only; `Device::Cuda` and `Device::Metal` are metadata;
it has a reverse-mode tape with higher-order gradients, Linear, GRU, LSTM and attention modules, Adam and
gradient clipping, a passing end-to-end MLP-with-Adam training test, safetensors and a limited ONNX export; it
has no PPO and no `vmap`. FACT about the suite's only PPO: `frankensim/crates/fs-g1-train` hand-rolls PPO with
manual backpropagation and depends only on `libm`; its exported checkpoint has an all-zero policy head, and its
bead says it "proves model/training/export plumbing, not learned humanoid locomotion".

DESIGN:

- **On device**, the policy forward pass, the backward pass and the optimizer are kernels in the shared dialect
  (matrix products, activations, Adam). Observations never leave the device during a rollout, which is the main
  source of GPU-simulator throughput.
- **FrankenTorch has three jobs**: the CPU trainer for small runs and tests; the gradient oracle against which the
  on-device backward kernels are differentially tested; and policy interchange through safetensors.
- **An upstream ask** (FD-FT-02) requests real device execution in FrankenTorch. If it lands, the on-device
  trainer switches to it under the rule in Appendix F.
- **The lesson from `fs-g1-train` is a gate rule**: a training gate tests learned behaviour against a negative
  control, never plumbing (GATE-CORTEX-01).

### 19.5 Randomization from cards

Every randomized parameter traces to an uncertainty on a card (actuator, interface, material, sensor). There are
no free-floating ranges. Objectives may be the mean or a tail measure (conditional value at risk) over the
randomization.

### 19.6 Detecting exploitation (GOAL-16)

Before a policy result is reported it is re-evaluated under four perturbations that should not matter to a
physically meaningful behaviour: half the time step; the next tier up; friction and stiffness moved within their
card uncertainty; the alternative contact law. A collapse outside the envelope sets `dt-sensitive`,
`tier-sensitive`, `parameter-sensitive` or `law-sensitive` on the result. For a symmetric robot the mirrored
policy must score the same within noise.

### 19.7 Evaluation protocol

Held-out scenes (§16.7); at least five seeds; interquartile mean with a stratified bootstrap interval; pairwise
comparisons through the e-process of C-15.

### 19.8 Gates

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-CORTEX-01 | Known answer | PPO on the reference quadruped's flat velocity task reaches the return threshold on `batch-cpu` and on each GPU backend; a zero-weight policy and a random policy both fail the same threshold. |
| GATE-CORTEX-02 | Parity | On-device backward kernels match FrankenTorch autograd within 10⁻⁵ relative on 10³ random batches. |
| GATE-CORTEX-03 | Performance of the method | BET-07, five seeds, with intervals. |
| GATE-CORTEX-04 | Known answer | Each detector of §19.6 flags its planted exploit and passes an honest policy. |
| GATE-CORTEX-05 | Performance | Wall-clock to the return threshold of GATE-CORTEX-01 on one RTX 4090 at most 15 minutes (§28). |

## 20. Spark: GPU backends and the kernel toolchain

### 20.1 Position relative to suite doctrine

FACT: FrankenSim's accelerator doctrine states "FrankenSim does not ship an accelerator backend", requires a
"Permanent CPU reference", and defines twelve evidence fields (AE-01 to AE-12) for any device run: device
identity, driver and runtime, compiler and backend, kernel source hash, kernel and workload identity, CPU axes,
reduction and determinism policy, CPU-versus-device comparison, the retained CPU reference, cancellation outcome,
phase attribution, and a go or no-go decision. In-process foreign kernels are forbidden in FrankenSim's
production paths. The suite's working GPU code lives outside FrankenSim's workspace in what its manifests call
the engine-kit pattern, and in FrankenTorch's `ft-kernel-metal`, which FrankenTorch calls its sanctioned unsafe
boundary.

**ADR-SPARK-01.** FrankenRobotics has an explicit owner mandate for CUDA and low-level Metal. It meets the
mandate without weakening the core:

- The core workspace is `unsafe_code = "deny"` with Franken-only runtime dependencies and never depends on a GPU
  crate.
- GPU backends live in **annex workspaces** under `spark/`, each with its own lockfile and a registry of unsafe
  capsules with a `SAFETY.md` each.
- Binaries that use a GPU are built from an `apps/` workspace that links the core and an annex.
- Every device run emits a receipt with FrankenSim's twelve evidence fields, unchanged.
- The CPU reference is permanent (§14.1).

Owner ratification is OPEN-SPARK-1; the fail-closed default is that GPU results are labeled exploratory until
ratified.

### 20.2 The kernel library

A kernel is a manifest, its arithmetic, and its sources. The manifest holds: `KernelId` (content hash), inputs
and outputs with shapes and physical dimensions, the determinism class it claims, the identities it declares
(the certificates of §10 attach here), whether a derivative program exists (D-8), and the symbol of its CPU
reference. Families at M2: kinematics (`K-fk-*`, `K-twist-*`), dynamics (`K-crba-*`, `K-rnea-*`), actuators
(`K-act-*`), collision (`K-geom-*`, `K-bp-*`, `K-np-*`, `K-compact`), constraints and solve (`K-jac-*`,
`K-cone-*`, `K-hess`, `K-chol`, `K-linesearch`), integration (`K-cay-step`, `K-normalize`), sensing
(`K-ray-*`, `K-sense-*`), learning (`K-gemm`, `K-act-fn`, `K-adam`), and utility (`K-scan`, `K-sort`,
`K-reduce`).

### 20.3 Two layers of source

**Skeletons are written by hand, twice.** The control structure of a kernel (loops, gathers, scans, sorts,
barriers) is written once in Rust for `batch-cpu` and once in the **shared dialect**: the common subset of CUDA
C++ and Metal Shading Language, made portable by a per-backend prelude of macros for address-space qualifiers,
kernel signatures and barriers. This is the pattern already working in
`frankensim/apps/instrument-lab/render-gpu-piano` (one `kernel.h`, a prelude in `cuda.rs`, a prelude in
`metal.rs`, and a CPU emulation that is the oracle — FACT). Dialect rules:

1. No backend intrinsic outside the prelude.
2. No floating-point atomics.
3. No built-in transcendental functions. Elementary functions are `det_*` functions written in the dialect
   itself, ported from `fs-math`'s deterministic set.
4. Floating-point contraction off; no fast-math relaxations.
5. Explicit loop bounds, unsigned 32-bit indices, no recursion, no pointer arithmetic beyond buffer plus index.

**Arithmetic is derived, once.** The mathematical body of a kernel (a Cayley step, an adjoint transport, a
cone projection, a Jacobian row) is produced at build time by `derive/frob-derive` using FrankenSymPy, lowered to
a straight-line program in `frob-kir`, and emitted as Rust and as dialect text, together with its derivative
programs and its certificates (C-14). Generated source is checked in with its digest; CI regenerates and
compares; generated code is never edited by hand.

| | |
|---|---|
| ✖ KILL-KIR | Trigger: at M2 the derivation toolchain cannot express or emit more than 20% of the kernel arithmetic, or emitted kernels miss BET-01's speed bound after one optimization pass. Fallback: arithmetic is hand-written in Rust and the dialect and bound by differential tests; certificates fall back per KILL-LEMMA. The skeleton pattern is unaffected. |

### 20.4 Backends

`KernelBackend` is a core trait: create buffers, compile a library, build a step plan, launch it, read back, and
return a receipt.

- **CPU.** `batch-cpu` is itself a backend whose kernels are Rust symbols, so one harness drives every tier.
- **CUDA (`spark/frob-spark-cuda`).** `cudarc` pinned to an exact version with the driver, NVRTC and dynamic
  loading features, as in FrankenSim's `render-gpu` (FACT: `cudarc = "=0.19.9"`; the current release is 0.19.10).
  The driver and NVRTC load at run time, so the build needs no CUDA toolkit. Dialect source compiles per device
  architecture to a cached binary; `frob spark precompile sm_89` fills the cache on a host without a GPU (the
  same recipe as `render-gpu --precompile`, FACT). Compile flags disable multiply-add fusion. Each device has one
  context and one host thread. With two RTX 4090s, worlds are sharded across devices and only small statistics
  cross through the host: the cards have no NVLink and peer-to-peer transfer is unverified (Appendix H).
- **Metal (`spark/frob-spark-metal`).** FrankenTorch's `ft_kernel_metal::compute::Gateway` (FACT): it compiles
  Metal Shading Language with `MathMode::Safe` by default, allocates shared-storage buffers on unified memory,
  chains several kernels in one command buffer through `batch()` with read and write alias checking, and returns
  a `Receipt`. Its limits today are also facts: synchronous completion only, at most 31 buffer arguments,
  helpers for `f32` and `u32` buffers, and the upstream-deprecated `metal` 0.33 binding. Until the asks in
  Appendix F land, kernels pack their arguments into a few buffers with offset tables to stay under the argument
  limit.
- **A third backend through `wgpu` [F]** is possible for other hardware because the dialect is small, but it is
  not planned.

### 20.5 Execution model

Environment creation allocates every buffer, compiles or loads every kernel, and builds a static **step plan**:
an ordered list of launches with fixed grid sizes. A rollout is policy forward, then the step plan repeated for
the control decimation, then observation, all on the device. The host receives a small statistics ring and
snapshots on request.

- **Static launch graphs.** On CUDA the step plan is captured once per shape as a launch graph. Published
  measurements of another simulator show throughput falling to roughly a quarter to a third when the graph is
  re-captured every step (FACT, Appendix H), so nothing in a rollout may change a shape.
- **Cancellation** is at the kernel-batch boundary: request, drain, finalize, with an executor-minted report
  (AE-10). A dropped handle is not a cancellation receipt.
- **Faults.** Device loss, out-of-memory, compile failure and a per-world non-finite sentinel are typed refusals
  (`FROB-REF-6xx`). A world that produces a non-finite value is frozen and marked `Faulted` for teleport. A
  watchdog aborts and reruns a stalled Metal command buffer, a failure mode the suite has already met
  (FACT: `render-gpu-piano/src/metal.rs`; `ft-kernel-metal` has a fault-injection test seam).

### 20.6 The CPU-versus-device check

`frob spark check` runs each kernel, and the full step, on `batch-cpu` in `f32` and on the device from identical
inputs, compares them under the kernel's determinism class, and writes the result to the **divergence ledger**.
A device result is not citable until its kernels have a current check on that device family, driver and compiler
(AE-08). The check runs again whenever any of those change.

### 20.7 Precision

Single precision on devices: Metal has no double type, and an RTX 4090 runs double precision at about 1/64 of its
single-precision rate (FACT: `render-gpu/README.md`). Three measures keep single precision adequate (BET-08):
anchored positions (§27.3), group-preserving updates with per-step normalization (C-05), and fixed-order pairwise
or compensated sums. Accumulators that need more (time, odometry, the energy ledger) use a double-float pair
type in the dialect, as `render-gpu-piano` does (FACT).

### 20.8 Determinism on devices

Run-to-run determinism per device family is a design requirement, achieved by the dialect rules, sort-based
compaction and fixed-order reductions. Bit equality across CPU, CUDA and Metal is a HYPOTHESIS: IEEE 754
requires correct rounding of +, −, ×, ÷ and √, CUDA documents compliance, but Apple confirmed in September 2026
that Metal offers no control over rounding mode or subnormal handling and the language specification permits
flushing subnormals (FACT, Appendix H). GATE-SPARK-01 measures it instead of assuming it.

### 20.9 Routes considered and not taken

FACT from the research pass, with the decision:

| Route | State in late 2026 | Decision |
|---|---|---|
| `wgpu` as the only layer | 30.x; portable; hides launch graphs, allocators and barriers; no doubles on Metal | Not the primary; possible third backend |
| Slang as the kernel language | CUDA target supported; Metal and WGSL targets experimental | Revisit when Metal is no longer experimental |
| NVIDIA's Rust-to-PTX compiler | Early alpha, Linux and NVIDIA only | Watch |
| `rust-gpu`, CubeCL and Burn | Alpha or a large dependency tree | Conflicts with the Franken-only core |
| Wrapping MuJoCo Warp, Newton or Genesis | Python ecosystems; Newton is CPU-only on macOS; Genesis has a Metal path through its own compiler | Anti-goal; used as oracles and as design references |

### 20.10 Gates and open questions

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-SPARK-00 | Known answer | One dialect source for `K-cay-step` compiles and runs on CUDA (`trj`) and on Metal (this Mac) and matches the Rust reference inside its class. M0. |
| GATE-SPARK-01 | Known answer | IEEE probe per device family: exhaustive bit comparison of `f32` square root and reciprocal over all 2³² inputs; 10⁹ sampled operand pairs for +, −, ×, ÷ including subnormal and overflow edges; a contraction detector; subnormal and NaN behaviour recorded. The outcome sets each device's best attainable determinism class. |
| GATE-SPARK-02 | Fidelity | Every M2 kernel and the full step agree between `batch-cpu` (`f32`) and each device inside the declared class: bit-equal for class D1, at most 4 ulp per kernel output and statistical equivalence of 1,000-step rollouts for class D3. |
| GATE-SPARK-03 | Performance | CUDA throughput targets of §28.1, with the disclosure block. |
| GATE-SPARK-04 | Performance | Metal throughput at least 25% of one RTX 4090 at matched settings on this M5 Ultra (BET-09). |
| GATE-SPARK-05 | Known answer | Precision battery: over 10⁶ steps, group-membership error within 4 ulp, closure and contact violations inside their envelopes, energy-ledger residual under 10⁻⁴ relative, at distances up to 1 km from the origin. |
| GATE-SPARK-06 | Known answer | Cancellation and fault injection: every injected fault yields its typed refusal, no leaked device memory after 10³ cancel cycles, and the drain report is executor-minted. |
| ✖ KILL-METAL | — | Trigger: GATE-SPARK-04 fails after one optimization pass, or the Metal backend cannot hold run-to-run determinism. Fallback: Metal becomes a development and replay tier (single-device debugging, small batches, the viewer); CUDA is the training tier; the Mac trains on `batch-cpu` using its 36 CPU cores. |

- **OPEN-SPARK-1.** Owner ratification of ADR-SPARK-01. Default: exploratory label on GPU results.
- **OPEN-SPARK-2.** A compute envelope for training campaigns. FrankenSim's machine rule caps any run at 16 GiB,
  8 cores and one hour without written approval (FACT: FrankenSim AGENTS.md rule 1.1). Default: FrankenRobotics
  inherits that rule for CPU work and limits a GPU job to one device and one hour until the owner approves a
  campaign envelope.

## 21. Arena: tasks, metrics and batteries (`frob-arena`)

### 21.1 Task families

| Family | Example tasks | First milestone |
|---|---|---|
| Stand and recover | Stand for 10 s; recover from a push of stated impulse | M1 |
| Velocity tracking | Commanded planar velocity and yaw rate on flat and rough ground | M2 |
| Terrain traversal | Stairs up and down, gaps, stepping stones, slopes, rubble | M2 |
| Interior navigation | Point goal through doors and thresholds in each archetype | M3 |
| Payload transport | Carry or tow a stated mass along a warehouse route, including a ramp | M3 |
| Low-friction crossing | Cross a kitchen galley with a wet or greasy patch | M3 |
| Tight-space maneuvering | Turn around, parallel shift and dock in an aisle of stated width | M3 |
| Moving surfaces | Board and leave a conveyor | M4 |
| Stair and curb climbing for wheels and tracks | A tracked base on stairs; a wheel cluster on a curb | M4 |
| Reach and carry | Mobile base with an arm: reach a shelf point, carry a tote | M4 |

### 21.2 Metrics

Every metric has a unit and a definition on file. The core set:

| Metric | Definition |
|---|---|
| Tracking error | Root-mean-square error between commanded and achieved planar velocity and yaw rate |
| Success, fall and collision rates | Per the task's termination rules, with the failure taxonomy of §23.5 |
| Cost of transport | Energy divided by (mass × g × distance), reported twice: mechanical work at the joints, and electrical energy from the battery |
| Froude number | v² / (g × leg length), so gaits are compared at matched dynamic similarity |
| Actuator utilization | Fractions of time at the torque, speed and thermal limits, per joint |
| Friction utilization | The distribution of ‖γₜ‖ / (μ·γₙ) over stance |
| Slip distance | Tangential travel of a contact while in stance |
| Clearance | Minimum distance between non-contact links and the scene |
| Robustness | The 10% conditional value at risk of the task score over the randomization |
| Maneuver cost | Path length and time for the tight-space tasks, against A-07's prediction |

A **scorecard** is a table of metrics by scene class with intervals, normalized against the reference robot of
the same class. No single scalar is reported unless the task defines one.

### 21.3 Batteries and the design corpus

- `smoke`: minutes; every template and reference robot on a handful of scenarios. Runs on every change.
- `standard`: the fidelity battery of §30.5, about 200 scenarios per robot class.
- `full`: every task family across every archetype with the full randomization.
- **The design corpus** for BET-05 and BET-06: about 200 designs made of templates, parameter perturbations of
  them, and designs with planted defects of known kind (an undersized knee actuator, an inertia that breaks
  static stability, a wheelbase too long for an aisle). Labels are fixed before any analysis is tuned.

### 21.4 Gates and open questions

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-ARENA-01 | Known answer | Every task is reproducible inside its determinism class from its `TaskId`, `SceneId`, `ModelId` and seed; reference-robot baseline scorecards are recorded. |
| GATE-ARENA-02 | Known answer | The scene-split check rejects an evaluation that reuses a training scene. |

- **OPEN-ARENA-1.** Which public task definitions to mirror for comparability. Default: mirror the reward terms
  of the public velocity-tracking tasks and make no claim of score parity.
- **OPEN-ARENA-2.** A permissively licensed hexapod reference model. Default: none; the template stands alone.

## 22. Atelier: the design studio (`frob-analyze`, `frob-opt`)

### 22.1 The loop

```
 design ──► check ──► synth controller ──► simulate ──► explain ──► edit ──► compare ──┐
   ▲        (static,   (seconds)           (batch or     (telemetry,  (typed)  (anytime-  │
   │         seconds)                       GPU)          energy,              valid)     │
   └───────────────────────────── lineage, archive, optimize, train ◄────────────────────┘
```

The order is a rule, not a suggestion: check before you simulate, simulate before you train, train before you
claim. Each stage is cheaper than the next by roughly an order of magnitude, and each can refuse the next.

### 22.2 Static analyses

These need no simulation. `frob design check` runs the ones marked ✓ and returns in under two seconds for a
template-sized design (TARGET).

| ID | Analysis | Question it answers | Mathematics | In `check` | Tier |
|---|---|---|---|---|---|
| A-01 | Mobility | How many degrees of freedom does it really have? Is any loop over-constrained? | Rank of the constraint Jacobian at exact rational sample points; the subgroup lattice (C-01) | ✓ | [S] / [F] |
| A-02 | Mass roll-up | Mass, centre of mass and inertia by subtree; physical consistency | C-03 | ✓ | [S] |
| A-03 | Symmetry | What is the symmetry group, how does it act, and what is the smallest edit that restores a near-symmetry? | C-09 | ✓ | [S] |
| A-04 | Gait atlas | Which gaits can this body have, with what phase lags and stability margins? | C-10 | | [F] |
| A-05 | Workspace | Can the feet or the hand reach what the tasks require (step height, shelf height)? | C-11 | | [S] |
| A-06 | Singularities | Where are they, and how close does a nominal gait come? | C-11 | | [S] |
| A-07 | Maneuverability | Growth vector, turning footprint, cost of a sideways shift, door and aisle passability | C-07 | ✓ for wheeled | [S] / [F] |
| A-08 | Static stability | Support margins, tip-over angles on slopes, contact-wrench-cone margin with payload | C-06 | ✓ | [S] |
| A-09 | Connection curvature | Displacement per shape cycle for kinematic locomotors; reorientation capacity in flight | C-08 | | [F] |
| A-10 | Co-design closure | Does the mass spiral close? What is the least mass and power? | C-12 | ✓ | [S] |
| A-11 | Actuator envelope | Do the torque-speed points of standing, crouching, stepping up and nominal gaits fit inside each card, with thermal steady state? | §13 | ✓ | [S] |
| A-12 | Modes | Natural frequencies and mode shapes at stance, block-diagonalized by symmetry; is actuator bandwidth sufficient? | C-09 | | [S] |
| A-13 | Self-collision and clearance | Do limbs sweep through each other or the ground over their ranges? | Swept charts (`fs-motion`, FACT) | ✓ | [S] |
| A-14 | Sensor coverage | Does the sensor suite see the footholds and the route ahead? | Ray geometry | | [S] |
| A-15 | Structural margin | Would a link yield under the peak loads seen or bounded? | Beam theory; finite elements through `fs-solid` | | [F] |
| A-16 | Sensitivity | Which parameters matter most for a chosen metric? | Automatic differentiation for static metrics; Morris screening and smoothed gradient estimates for simulated ones | | [S] / [F] |

Every analysis returns its result with an evidence class, the model identity, and, when it finds a problem,
ranked repairs as typed edits.

### 22.3 Explain

After a simulation, `frob explain <run>` assembles: the failure taxonomy with first-cause ordering (§23.5); the
energy ledger (§23.4); the saturation report by joint; friction utilization and slip; the gap between estimated
and privileged state; and a ranked list of typed edits predicted to help, computed from A-16. The list is the
envelope's `next_actions`.

### 22.4 Comparing designs honestly (C-15)

`frob eval compare A B --task T` pairs episodes by world address, so both designs meet the same scenes, the same
disturbances and the same randomization draws. A betting e-process runs per metric. With several metrics the
e-values are averaged, which is again an e-value, so the error rate holds without correction tables. The
comparison stops when a threshold is crossed or the budget ends, cancels the unfinished episodes, and returns a
verdict with the e-value, the confidence sequence for the effect, the number of pairs used, and the tier.
A verdict from a fast tier is `provisional` until promoted (§14.5).

### 22.5 Lineage: diff, blame, bisect

Design versions form a content-addressed graph whose edges are typed edits. `design diff` lowers the difference
of two terms to a list of edits. `design blame` names the edit that introduced a parameter value.
`design bisect --predicate "<task> succeeds"` walks a lineage path by binary search, deciding each step with the
anytime-valid comparison, and returns the edit that changed the outcome (NS-06).

### 22.6 Optimization

Agents are the discrete optimizer: they choose topology, leg count and actuator families. The studio supplies
fast continuous optimization and honest evaluation underneath.

- **Continuous parameters.** CMA-ES with restarts, Bayesian optimization with trust regions and multiple
  fidelities, and Riemannian quasi-Newton for static objectives on manifold-valued parameters, all from
  FrankenSim (`fs-dfo`, `fs-bo`, `fs-ascent`, FACT), reached through the airlock.
- **Racing.** Candidates in a generation are compared by e-processes and the dominated ones are cancelled
  mid-evaluation (FrankenSim's Bet 8), so compute follows the survivors.
- **Quality-diversity.** A MAP-Elites archive (`fs-archive`, FACT) over behaviour descriptors such as mass,
  speed, energy per metre and footprint. `frob opt archive show` gives an agent a map of different good designs
  instead of one optimum.
- **The fidelity ladder.** Static analyses, then the baseline controller on a fast tier, then a trained policy
  for finalists, then promotion on the referee. Each rung certifies or escalates.
- **Co-design with control.** The inner loop is the synthesized baseline controller, which costs seconds; policy
  fine-tuning is spent only on finalists.

### 22.7 Registered predictions

Before a run, an agent may register a prediction: `frob predict register --metric cot-elec --interval 0.42 0.50
--confidence 0.8 --for <planned-run>`. It is hashed and frozen before the run starts and scored afterwards with a
proper scoring rule. `frob predict report` shows each agent's calibration. The mechanism costs almost nothing
and answers a question nothing else does: whether an agent understands the robot it is designing or is searching
blindly (adapted from FrankenMining's forecast ledger, FACT).

### 22.8 Reports

`frob report design <id>` writes an evidence package: the design and its lineage, every analysis, scorecards,
comparisons with their verdicts, certificates, device receipts and replay addresses, as a directory with an HTML
index (FrankenSim's evidence-package pattern, FACT: `fs-package`).

### 22.9 Gates

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-ATEL-01 | Performance of the method | BET-05 on the design corpus: area under the ROC curve at least 0.8 and at least 0.05 above the heuristic baseline. |
| GATE-ATEL-02 | Known answer | Each analysis A-01 to A-14 passes its known-answer tests; `design check` finishes in under 2 s at the 95th percentile on templates. |
| GATE-ATEL-03 | Performance of the method | BET-10: the A/B of agent surfaces, pre-registered, with an anytime-valid verdict. |
| GATE-ATEL-04 | Known answer | Comparison type-I error at or below α under optional stopping over 10⁴ null comparisons; `design bisect` finds the planted edit in at least 95 of 100 trials. |

## 23. Chronicle: telemetry, replay and evidence (`frob-chronicle`)

### 23.1 What is stored, and where

Run and episode metadata, events, summaries, receipts, verdicts and predictions live in a ledger on FrankenSQLite
(the pattern of FrankenSim's `fs-ledger`, FACT). Bulk data lives in content-addressed column files. Everything
carries the identities of §9.

### 23.2 Telemetry levels

| Level | Contents | Default scope |
|---|---|---|
| T0 | Per-episode metrics, the energy ledger totals, and state digests at intervals | Every world, always |
| T1 | Events: contacts made and broken, limit and saturation episodes, faults, refusals | Every world |
| T2 | Traces at control rate: joint states, torques, contact wrenches, estimator state | A reservoir sample of worlds, and every failing world |
| T3 | Full state snapshots at intervals, sufficient for exact restart | Sampled and failing worlds |

A per-world ring buffer keeps the last seconds at T2 and T3 on the device and is flushed when a world fails, so
failures are always recorded in full without paying to record everything.

### 23.3 Replay and teleport (GOAL-10)

A world is determined by its address, its model, scene, task and policy identities, and its kernel versions.
`frob sim replay <episode>` reproduces it. Adding `--tier batch-cpu` **teleports** it: the run restarts from the
nearest snapshot on the CPU, compares digests step by step with the recorded ones, and at the first difference
re-runs that step with per-stage digests to name the stage and kernel where the tiers part (the per-subsystem
digest technique of the Wright Flyer plan, FACT). Digests are a 128-bit integer mix computed in-kernel, since
integer arithmetic is exact on every backend; BLAKE3 is used on the host for snapshots.

### 23.4 The energy ledger (GOAL-11)

For each world and episode:

```
battery energy out = Σ actuators (copper loss + friction loss + gear loss + Δ stored) + mechanical work at joints
mechanical work at joints = Δ kinetic + Δ potential + contact dissipation (normal damping + sliding friction)
                            + joint damping + work on fixtures and payloads + numerical residual
```

Every term is a port quantity (C-12), accumulated in double-float on the device. The numerical residual is
reported as its own line, "integrator and solver", and is never folded into a physical term. Breakdowns are
available by joint, by limb, and by stance and swing. Cost of transport decomposes along the same lines, so an
agent can see that, say, 38% of the energy went to copper loss in the knees during stance.

### 23.5 Failure taxonomy

`fall`, `stuck`, `collision`, `self-collision`, `slip`, `torque-saturation`, `speed-saturation`,
`thermal-limit`, `limit-impact`, `timeout`, `numerical-fault`. Monitors timestamp their first anomaly; the
earliest anomaly before the terminal event is reported as the first cause, with stills at −0.5 s, 0 s and +0.2 s.
The ordering is a heuristic and is labeled `Estimated`.

### 23.6 Summaries an agent can afford

`frob telemetry summary --budget 800` returns a hierarchical summary sized to a token budget, with handles for
drill-down. Numbers carry units and evidence classes. A compact encoding is available (the suite's MCP tools
already use TOON, FACT). Raw traces are never the default answer.

### 23.7 Gates

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-CHRON-01 | Known answer | 1,000 random worlds from a GPU run replay with identical digests on the same backend, and inside the class on `batch-cpu`. |
| GATE-CHRON-02 | Known answer | Teleport localizes a planted single-kernel divergence to the right step and stage in 100 of 100 trials. |
| GATE-CHRON-03 | Known answer | The energy ledger closes: residual below 10⁻⁹ relative on the referee and 10⁻³ on single-precision tiers; a damped pendulum's ledger matches the hand calculation. |

## 24. Tether: the CLI, the daemon, the MCP server and the agent model

### 24.1 Principles

- **One verb registry.** `registries/verbs.toml` defines each verb once: name, cluster, kind (`read`, `run`,
  `mutate`, `admin`), required capabilities, input and output schema identifiers, possible refusals, idempotency
  rule, cost model and examples. The CLI, the MCP tools, the schemas and the agent documentation are generated
  from it, and a parity gate diffs them (the design FrankenMining adopted, FACT).
- **Agents ask questions and spend budgets.** They do not drive solvers (FrankenSim addendum proposal 8, FACT).
  A verb takes a question, a confidence or tolerance where relevant, and a budget; it answers, or it refuses and
  says what would make the question answerable.
- **The Five Explicits.** Units, seeds, budgets, versions and capabilities are never implicit (FrankenSim P10,
  FACT).

### 24.2 The envelope

With `--json`, every verb writes exactly one envelope to standard output (long jobs write newline-delimited
progress events, then the envelope). Malformed input and unknown verbs also produce the envelope.

```json
{ "schema": "frob.envelope/1", "verb": "design.check", "ok": true, "exit": 2,
  "result": { "mass_kg": 14.2, "closure": "closed", "symmetry": "C2xC2",
              "findings": [ { "code": "A-11.knee.continuous", "severity": "high",
                              "message": "Knee continuous torque demand exceeds the card by 31% in crouch.",
                              "repairs": [ { "edit": "swap-actuator", "args": {"joints": "*.knee", "to": "qdd-l"},
                                             "predicted": {"mass_kg": "+0.9", "margin": "+46%"} },
                                           { "edit": "regear", "args": {"joints": "*.knee", "ratio": 1.9} } ] } ] },
  "evidence": { "class": "tested", "tier": "static", "model_id": "m:9f2c…", "determinism": "exact" },
  "cost": { "wall_ms": 412, "cpu_core_s": 0.4, "gpu_s": 0 },
  "warnings": [], "next_actions": [ { "command": "frob design edit quad-a swap-actuator --joints '*.knee' --to qdd-l",
                                      "why": "Largest predicted margin gain per kilogram." } ] }
```

| Exit | Meaning |
|---|---|
| 0 | Completed, nothing to flag |
| 2 | Completed with findings or flags |
| 3 | Refused input (`FROB-REF-1xx` to `3xx`) |
| 4 | Budget stop; nothing partial is published as a result |
| 5 | Capability not implemented or not admitted |
| 6 | Backend or device unavailable (`FROB-REF-6xx`) |
| 7 | Conflict: idempotency key reuse or lineage conflict |
| 9 | Indeterminate. Never retry blindly |

Exit code 1 is never emitted on purpose, so a 1 always means an unhandled failure.

### 24.3 Verb clusters

Full registry in Appendix J.

| Cluster | Verbs |
|---|---|
| `design` | `new`, `show`, `edit`, `diff`, `blame`, `bisect`, `check`, `import`, `export`, `templates` |
| `analyze` | One verb per analysis A-01 to A-16 |
| `scene` | `gen`, `import`, `compile`, `list`, `show` |
| `control` | `synth`, `show` |
| `sim` | `estimate`, `run`, `replay` |
| `train` | `start`, `status`, `stop`, `export` |
| `eval` | `run`, `compare`, `promote`, `scorecard` |
| `telemetry` | `summary`, `query`, `energy`, `failures`, `stills`, `inspect-pixel`, `export-html` |
| `explain` | `run`, `design` |
| `opt` | `start`, `status`, `archive` |
| `predict` | `register`, `report` |
| `prove` | `kernel`, `verify`, `list` |
| `spark` | `devices`, `probe`, `check`, `precompile` |
| `job` | `list`, `status`, `wait`, `cancel` |
| `report` | `design`, `run` |
| operator | `doctor`, `capabilities`, `robot-docs`, `schema` |

### 24.4 Estimates, budgets and jobs

Every verb that can cost more than a second accepts `--estimate` and returns CPU core-seconds, GPU seconds,
memory and disk with an interval, without running. A session carries budgets; a verb that would exceed one is
refused with the estimate attached. Mutating verbs take idempotency keys. Long work is a job with a handle,
progress events, and cancellation that drains (§20.5).

### 24.5 The daemon

`frobd` owns devices, kernel caches and the job queue, with fair sharing between agents. Without the daemon the
CLI still runs CPU-tier verbs in-process, and says in the envelope that it did.

### 24.6 The MCP server

`frob-mcp` is built on fastmcp-rust (FACT: version 0.13, transports for stdio, streamable HTTP, SSE and
WebSocket, tools declared with an attribute macro, and an exact pin on asupersync 0.6.2). Tools are generated
from the registry and grouped into clusters of at most seven per task so an agent's context stays small. Stills,
reports and schemas are MCP resources. Long verbs are exposed as start, status and cancel with progress
notifications. Text that arrives from imports (design names, asset metadata, scan annotations) is data and is
never interpreted as instructions.

### 24.7 Capabilities and many agents

A session holds a capability token with scopes such as `design:write`, `sim:run`, `gpu:seconds=600`,
`scene:import` and `train:run`. Each agent has its own design namespace and lineage; no agent can rewrite
another's history. A shared leaderboard and archive are read-only views. Lineage identifiers are usable as Agent
Mail thread identifiers, so a swarm can coordinate with the tools it already has.

### 24.8 A session, end to end

```
$ frob design new quad-a --template quadruped-12dof --json            # exit 0, design d:41aa…
$ frob design check quad-a --json                                     # exit 2: knee undersized (A-11)
$ frob design edit quad-a swap-actuator --joints '*.knee' --to qdd-l  # new version d:7c03…, symmetry preserved
$ frob design check quad-a --json                                     # exit 0: closes at 15.1 kg
$ frob control synth quad-a --json                                    # ok: trot from the atlas
$ frob sim estimate quad-a --scene terrain:stairs@0.4 --worlds 2048 --seconds 20 --tier gpu-cuda
                                                                      # 38 ± 6 GPU-seconds, 3.1 GB
$ frob predict register --metric success --interval 0.6 0.9 --confidence 0.8 --for next
$ frob sim run quad-a --scene terrain:stairs@0.4 --worlds 2048 --seconds 20 --tier gpu-cuda --json
                                                                      # exit 2: success 0.44, provisional
$ frob explain run r:19e2… --budget 600                               # first cause: slip on stair nosing;
                                                                      # friction utilization p95 = 1.0 at front feet
$ frob telemetry stills r:19e2… --world 311 --at failure              # contact sheet PNG
$ frob design edit quad-a set-param foot.radius 0.022 0.030
$ frob eval compare quad-a@prev quad-a --task stairs-0.4              # B better, e = 310, n = 96 pairs, provisional
$ frob eval promote r:… --tier referee --json                         # established, inside the envelope
```

### 24.9 Documentation for agents

`frob robot-docs` prints the guide, the verbs, the schemas and worked examples. `frob capabilities --json` lists
what is implemented in the §0.2 sense and what is not. `frob doctor` checks toolchains, foundation pins, devices,
kernel caches and probe status. A skill file documents the design loop for agents that load skills, in the way
FrankenTextiles ships its fitting loop (FACT).

### 24.10 Gates

| Gate | Claim | Threshold (TARGET) |
|---|---|---|
| GATE-TETHER-01 | Performance of the method | A fresh agent given only `robot-docs` completes 20 scripted studio tasks with at least 18 correct and no budget violation. |
| GATE-TETHER-02 | Known answer | CLI and MCP surfaces are generated from the registry and their parity diff is empty; every verb has a schema, an example and a refusal list. |
| GATE-TETHER-03 | Known answer | Envelope and exit-code conformance on malformed, unknown and over-budget inputs. |
| ✖ KILL-MCP | — | Trigger: INT-FMCP-01 fails (the MCP stack cannot share a build graph with the core). Fallback: `frob-mcp` is a separate workspace that talks to `frobd` over a local socket in newline-delimited JSON; if that also fails, agents use the CLI, which is complete on its own. |

---
# PART IV. SUBSTRATE, SUITE AND ENGINEERING

## 25. Suite consumption: what FrankenRobotics takes from its siblings, and how

### 25.1 Postures

Five postures, the vocabulary FrankenMining established (FACT: Mining §27):

| Posture | Meaning |
|---|---|
| **Consume** | A Cargo dependency at an exact pin, default features off |
| **Subprocess** | A binary or CLI we invoke; no crate in our build graph |
| **Lift** | Copied source with a provenance header, a registry row and a golden test; re-owned here |
| **Oracle** | Development and test only; never in a production graph |
| **Pattern** | We copy the design, not the code |

Each foundation has an `FD-` row with its posture, its pin, the crates allowed to touch it, an admission gate and
a fallback, because no milestone may be blocked by a sibling that has not converged.

### 25.2 The FrankenSim decision

**ADR-SUITE-01: consume FrankenSim by path dependency, confined behind the airlock crate `frob-fsim`, pinned by
commit in `franken-stack.lock`.**

The owner's brief is to use FrankenSim for the physics to the greatest extent possible. The referee tier needs
`fs-mbd` and `fs-contact` whole (about 440k lines between them, FACT), together with geometry, integrators,
optimizers and the ledger. Copying that would be a fork.

The two sibling products chose otherwise, and their reasons are facts this plan must answer. FrankenMining
rejected a Cargo dependency on FrankenSim because it would link two asupersync versions, needs a different
nightly for one crate's const-generic feature, failed `cargo check` on fifteen targets at the time, and uses
out-of-repo relative paths. FrankenTextiles lifts files one by one. Why the decision differs here:

1. **The need is different in kind.** Those products took numerics files. This one needs the multibody and
   contact engines.
2. **The pattern has precedents.** Inside FrankenSim's own repo, several crates sit outside its workspace with
   their own `[workspace]` and consume `fs-*` crates by path (`render-gpu`, `fs-cmaes-viz-wasm`, `fs-g1-train`,
   `fs-flyer-wasm`); one external project consumes by path (FACT).
3. **Each objection has a mitigation that is a gate.** The core workspace adopts FrankenSim's toolchain pin
   (INT-TOOLCHAIN-01). The workspace root repeats FrankenSim's `[patch.crates-io] asupersync` entry and its
   FrankenSQLite profile overrides, because Cargo ignores a path dependency's patches and profiles
   (INFERENCE from the audit; confirmed or refuted by INT-FSIM-01). A build may contain exactly one asupersync
   package (INT-ASUP-01). Volatile FrankenSim types are named in one crate only.
4. **The fallback is written down.** See KILL-FSIM.

Known hazards on the day of writing (FACT): FrankenSim's `fs-mbd` API changes daily; its `Component` trait is
contract v0 and not frozen; its `constellation.lock` pins asupersync 0.5.0 at one commit while the local
asupersync checkout is 0.6.2 at another, so its own verify-only preflight refuses today. A pinned commit and the
airlock are what make those survivable.

### 25.3 The airlock

`frob-fsim` is the only crate that may name types from FrankenSim's volatile crates. It exposes
FrankenRobotics-native traits (`RefereeDynamics`, `RefereeContact`, `GeometryKernel`, `Optimizer`, `Ledger`) and
adapts them. A short **bedrock-direct** list may be used anywhere, because those crates are small and stable and
their types are the vocabulary of the mathematics: `fs-ga`, `fs-math`, `fs-qty`, `fs-rand`, `fs-blake3`,
`fs-ivl`, `fs-simd`, `fs-exec`, `fs-evidence`, `fs-ad`. Everything else goes through the airlock. A FrankenSim
API change then costs one crate.

### 25.4 Foundation ledger

| FD | Foundation | Posture | Used for | Confined to | Admission | Fallback |
|---|---|---|---|---|---|---|
| FD-FSIM | frankensim | Consume by path | Bedrock-direct crates (above). Through the airlock: `fs-mbd`, `fs-mbd-material`, `fs-contact`, `fs-tribo`, `fs-time`, `fs-query`, `fs-geom`, `fs-rep-sdf`, `fs-rep-mesh`, `fs-rep-frep`, `fs-motion`, `fs-ports`, `fs-system`, `fs-matdb`, `fs-em`, `fs-thermal`, `fs-control`, `fs-solid`, `fs-dfo`, `fs-bo`, `fs-ascent`, `fs-archive`, `fs-opt`, `fs-sos`, `fs-race`, `fs-eproc`, `fs-tropical`, `fs-symmetry`, `fs-ledger`, `fs-session`, `fs-package`, `fs-render`, `fs-roofline`, `fs-detaudit`, `fs-propcheck` | `frob-fsim`, plus the bedrock-direct list | INT-FSIM-01 | KILL-FSIM |
| FD-ASUP | asupersync | Consume | Structured concurrency, cancellation, `Cx` | All async code; one package id per build | INT-ASUP-01 | Process boundary between surfaces and core |
| FD-FSQL | frankensqlite | Consume | The Chronicle ledger | `frob-chronicle` (through `fs-ledger` where it fits) | INT-FSQL-01 | Append-only JSON-lines ledger with the same schema |
| FD-FT | frankentorch | Consume | `ft-kernel-metal` (Metal gateway) in the Metal annex; `ft-api`, `ft-nn`, `ft-optim`, `ft-serialize` as CPU trainer, gradient oracle and policy interchange | `spark/frob-spark-metal`; `frob-learn` behind a feature | INT-FT-01 | Annex binds `objc2-metal` directly; CPU trainer uses the in-house kernels on `batch-cpu` |
| FD-CUDA | `cudarc` (third party) | Consume, annex only | CUDA driver and NVRTC bindings | `spark/frob-spark-cuda` | INT-CUDA-01 | None needed for CPU and Metal; CUDA tier absent |
| FD-FNP | franken_numpy | Consume narrowly; Oracle | Array files at the export boundary (`fnp-io`); `fnp-linalg` batch routines as an `f64` oracle for small-matrix kernels | `frob-chronicle`, `frob-conformance` | INT-FNP-01 | Plain binary columns; drop the oracle |
| FD-FSCI | frankenscipy | Oracle | Reference integrators (`solve_ivp`), rotation conversions (`fsci-spatial`), cross-check optimizers | `frob-conformance` | With INT-FSIM-01 (FrankenSim already pins it) | Drop the cross-checks |
| FD-FSYM | frankensympy | Build-time; certificate checker | Derivation of kernel arithmetic; polynomial, interval and rational certificates | `derive/frob-derive`, `frob-lemma` (offline verifier only) | INT-FSYM-01 | KILL-KIR, KILL-LEMMA |
| FD-FLN | franken_lean | Subprocess | Independent re-check and sealing of Lean proofs | `lean/` tooling | INT-FLN-01 | Stock Lean alone, recorded as a single-kernel check |
| FD-FMCP | fastmcp-rust | Consume | The MCP server | `surface/frob-mcp` | INT-FMCP-01 | KILL-MCP |
| FD-F3D | franken_threed | Subprocess; Pattern | Packaging the viewer as self-contained HTML; its admission-record discipline | `viewer/` build scripts | None (optional) | Plain Three.js bundle |
| FD-FJAX | frankenjax | Pattern | Reference semantics for batching and differentiation transforms | — | None | — |
| FD-FTUI | frankentui | Consume [F] | An optional live dashboard | `frob-cli` behind a feature | None | No dashboard |
| FD-FSS | franken_surveillance_system | Pattern | The twin package and the camera-model contract | — | None | — |

Honest notes on two of these:

- **franken_numpy** is an `f64` array library on heap vectors with no single-precision storage and no fixed-size
  matrix types, and it lists GPU arrays and automatic differentiation as out of scope (FACT). It is the right
  tool at the interchange boundary and as an oracle. It is not put in a hot path where it does not fit.
- **frankensympy**'s workspace refers to two sibling pin directories that do not exist on this machine, and it
  pins a different nightly from the rest of the suite (FACT). `derive/` therefore depends on individual `fsym-*`
  crates by path and has its own toolchain file.

### 25.5 The stack lock

`franken-stack.lock` is TOML in FrankenMining's format (FACT). One row per foundation, checked by
`xtask check-stack`:

```toml
[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "../frankensim"
git_rev = "<40-hex commit recorded at INT-FSIM-01>"
toolchain = "nightly-2026-07-06"
confined_to = ["frob-fsim", "bedrock-direct"]
admission_gate = "INT-FSIM-01"
fallback = "KILL-FSIM: lift fs-ga, fs-math det, fs-rand and the fs-mbd articulated module with provenance."

[[foundation]]
id = "FD-ASUP"
name = "asupersync"
posture = "consume"
version = "<the single version every workspace resolves>"
admission_gate = "INT-ASUP-01"
fallback = "Process boundary between surface and core workspaces."
```

### 25.6 Admission gates

An admission record is valid only if it cites a commit of this repo whose tracked `Cargo.lock` contains the
foundation, with the command and its output (the rule FrankenMining adopted after finding admission records for
crates that were not in the build, FACT). Records live in `DEPENDENCY_ADMISSION.md`, a convention taken from
FrankenThreeD.

| Gate | Pass criterion |
|---|---|
| INT-TOOLCHAIN-01 | The core workspace builds and tests on FrankenSim's pinned nightly. Whether the suite's common nightly also works is recorded. |
| INT-FSIM-01 | From this repo, a crate depending by path on `fs-ga`, `fs-mbd`, `fs-contact`, `fs-time` and `fs-query` passes `cargo check --all-targets`, and reproduces the digest of a G1 forward-dynamics state computed inside FrankenSim at the pinned commit. |
| INT-ASUP-01 | `cargo tree -d` shows one asupersync package in each workspace, and no binary links two. |
| INT-FSQL-01 | The ledger opens, writes, and recovers after a forced kill mid-write. |
| INT-FT-01 | `ft-kernel-metal`'s gateway builds in the Metal annex and runs a kernel on this Mac. |
| INT-CUDA-01 | The pinned `cudarc` builds without a CUDA toolkit, loads the driver on `trj`, and runs a kernel on both GPUs. |
| INT-FSYM-01 | `derive/` builds against the `fsym-*` crates by path, derives `K-cay-step`, and its certificate replays in the offline verifier. |
| INT-FLN-01 | `fln` builds on a Linux host, re-checks and seals one trivial theorem from `lean/`, and the capsule verifies from an empty environment. |
| INT-FMCP-01 | `frob-mcp` builds with a single asupersync package and passes a protocol smoke test. |
| INT-FNP-01 | An array file written through `fnp-io` reads back identically. |

### 25.7 The dependency firewall

Which milestone needs which foundation. A blank cell means the milestone does not depend on it.

| Foundation | M0 | M1 | M2 | M3 | M4 | M5 |
|---|---|---|---|---|---|---|
| frankensim | admit | referee, geometry, ledger | parity oracle | optimizers, racing | optimizers, archive | structural, variational |
| asupersync | admit | jobs | device jobs | daemon, MCP | | |
| frankensqlite | admit | ledger | | | | |
| frankentorch | admit (Metal) | | Metal backend, gradient oracle | | | |
| cudarc | admit | | CUDA backend | | | |
| frankensympy | admit | certificates for M0 kernels | derived kernels | | | coverage gate |
| franken_lean | | | | | | capsules |
| fastmcp-rust | admit | MCP for M1 verbs | | full MCP | | |

M1 depends only on FrankenSim, asupersync and FrankenSQLite. If the GPU, symbolic and proof foundations all
failed admission, M1 would still ship a referee-tier design studio on the CPU.

### 25.8 Upstream asks and drift

Sibling repos are read-only to this program. A need is filed as a bead in the sibling's own tracker with a
priority, an interim posture here, and a switch-over rule (Appendix F). Asks are additive: a new mode or type,
never a change to existing behaviour, with any lossiness self-described (the rule the Wright Flyer plan sets for
its upstream extensions, FACT). Pins advance through a compatibility train: move the pin, rerun the admission
and parity gates, relock. A sibling's newer clean head is never rolled back locally.

| | |
|---|---|
| ✖ KILL-FSIM | Trigger: INT-FSIM-01 cannot pass within two weeks of M0 effort, or BET-04's breakage threshold is crossed. Fallback: lift the minimal referee set with provenance headers and golden tests (`fs-ga`, `fs-math` deterministic functions, `fs-rand`, the `fs-mbd` articulated module, the `fs-contact` multicontact solve), state "lift-only" in the README, and keep asks open. |

## 26. Workspace, crate map and budgets

### 26.1 Repository layout

```
franken_robotics/
  Cargo.toml  rust-toolchain.toml  franken-stack.lock        # core workspace; toolchain follows FrankenSim
  AGENTS.md  README.md  DEPENDENCY_ADMISSION.md  COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md
  crates/frob-*/            # the core: unsafe denied, Franken-only runtime dependencies
  spark/frob-spark-cuda/    # annex workspace: cudarc, registered unsafe capsules
  spark/frob-spark-metal/   # annex workspace: ft-kernel-metal gateway
  apps/                     # workspace that links core + annexes into frob, frobd
  surface/frob-mcp/         # MCP server (joins the core workspace if INT-FMCP-01 allows)
  derive/frob-derive/       # build-time derivation with FrankenSymPy; own toolchain file
  lean/                     # Lake project; outputs not committed
  viewer/                   # Three.js viewer and HTML export
  kernels/                  # shared-dialect sources and generated arithmetic, with digests
  registries/               # verbs.toml, refusals.toml, kernels.toml, gates.toml, cards/
  assets/                   # templates, generic actuator and interface cards, license manifests
  tests/fixtures/oracle/    # pinned oracle captures, never regenerated from the candidate
  tools/oracle/             # capture scripts for development-time oracles
  docs/delivery/            # the delivery log (never in this plan)
```

### 26.2 Crate map

Scope caps are in thousands of lines of Rust including tests. They are caps that trigger a scope conversation at
115%, not estimates of effort or schedule.

| Crate | Layer | Responsibility | Cap |
|---|---|---|---|
| `frob-core` | 0 | Identities and hashing, evidence classes, refusals, schema versions, units glue | 6 |
| `frob-fsim` | 0 | The airlock: adapters to FrankenSim's volatile crates | 14 |
| `frob-lie` | 1 | Lie groups and algebras, subgroups, retractions, frame typing | 12 |
| `frob-algebra` | 1 | Finite groups and representations, cones and the cone solver, posets, the inertia cone | 14 |
| `frob-kir` | 1 | Straight-line programs, dimension checking, differentiation, emitters | 12 |
| `frob-design` | 2 | Design terms, constructors, typed edits, lints | 14 |
| `frob-model` | 2 | The model compiler, functors, lowerings, memoization | 14 |
| `frob-io` | 2 | URDF and MJCF import and export, loss reports, the XML reader | 10 |
| `frob-actuate` | 2 | Actuator, transmission, battery and thermal models; cards | 10 |
| `frob-scene` | 2 | Scene packs, the scene compiler, terrain and interior generators, import | 22 |
| `frob-kernels` | 3 | The kernel library: manifests, Rust skeletons, generated arithmetic | 24 |
| `frob-contact` | 3 | Collision geometry, detection, contact laws, wheels, tracks, terrain mechanics | 18 |
| `frob-dyn` | 3 | The batched step, integrators, tiers, the trust ladder | 16 |
| `frob-sense` | 3 | Sensor models and the observation service | 10 |
| `frob-control` | 4 | The invariant filter, controller synthesis, control programs | 16 |
| `frob-learn` | 4 | Environment API, task specifications, PPO, networks, FrankenTorch bridge | 16 |
| `frob-analyze` | 4 | Static analyses A-01 to A-16 | 18 |
| `frob-opt` | 4 | Comparison, lineage operations, optimization, the archive, predictions | 12 |
| `frob-arena` | 4 | Tasks, metrics, scorecards, batteries, the design corpus | 10 |
| `frob-chronicle` | 5 | Telemetry, replay, digests, the energy ledger, summaries, stills | 16 |
| `frob-lemma` | 5 | Claims, certificate production and offline verification, theorem cards | 6 |
| `frob-verbs` | 5 | The verb registry, envelope, schemas, verb implementations | 16 |
| `frob-cli` | 6 | The `frob` and `frobd` binaries | 8 |
| `frob-conformance` | 6 | The Gauntlet harness, oracle fixtures, batteries | 16 |
| `xtask` | — | Six checks (§26.6) | 3 |
| `spark/frob-spark-cuda` | annex | CUDA backend | 6 |
| `spark/frob-spark-metal` | annex | Metal backend | 5 |
| `surface/frob-mcp` | annex | MCP server generated from the registry | 5 |
| `derive/frob-derive` | annex | Symbolic derivations and certificate production | 12 |
| **Total** | | | **361** |

Kernel sources in the shared dialect (about 15k lines) and the viewer (about 8k lines of JavaScript) are
additional.

### 26.3 Layer and dependency rules

- **L-1** A crate depends only on lower layers. `xtask check-layers` fails a deliberate upward edge.
- **L-2** Only `frob-fsim` names types from FrankenSim's volatile crates. The bedrock-direct list is the only
  exception.
- **L-3** The core never depends on an annex. Backends implement core traits.
- **L-4** Each foundation is confined to the crates in its `FD-` row.
- **L-5** Core runtime dependencies are `std` and Franken foundations. `serde` and `serde_json` at exact pins are
  allowed in boundary crates only (`frob-io`, `frob-verbs`, `frob-chronicle`, `frob-design`), never in a hot
  path. Forbidden everywhere: tokio and its ecosystem, a direct Rayon pool, BLAS and LAPACK, C or C++ numerical
  libraries, a Python interpreter.
- **L-6** No bare `f64` physical quantity in a public signature above layer 1. Quantities are `fs-qty` types or
  group elements.
- **L-7** Library crates do not print. Hot loops emit no per-iteration JSON.
- **L-8** Generated files are never edited. `xtask check-generated` regenerates and compares.

Honesty about the closure: path dependencies on FrankenSim bring its transitive closure, which includes
third-party crates such as `nalgebra` and `rayon` through other siblings (FACT: 244 registry packages in
FrankenSim's lockfile). "Franken-only" describes direct edges.

### 26.4 Unsafe code

The core has `unsafe_code = "deny"` and plans zero capsules: SIMD comes from `fs-simd`, arenas from FrankenSim's
substrate. The annexes may contain capsules, each registered, under 300 lines, behind a safe API, with a
`SAFETY.md` (FrankenSim's capsule discipline, FACT).

### 26.5 Contracts

Every crate ships `CONTRACT.md` with the ten sections of §0.2 and a `tests/conformance.rs`. A crate becomes a
dependency target only after its contract lands.

### 26.6 The six checks

`xtask` contains exactly: `check-layers`, `check-deps`, `check-unsafe`, `check-generated`, `check-contracts`,
`check-stack`. A seventh check needs a named defect class it would have caught and the owner's approval. This is
the guard against the process growth FrankenMining recorded.

### 26.7 Working rules inherited from the machine and the suite

FACT, from the machine-level and FrankenSim agent instructions, adopted here:

- Cargo runs through the build-offload shim, which queues for a worker and never compiles locally; cargo is not
  invoked by absolute path. Projects stay under `~/projects`. Isolated target directories use
  `${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/<name>`, and large scratch uses `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/<name>`.
- A disk preflight precedes heavy lanes.
- No file is deleted without explicit permission. No branches and no worktrees: work lands on `main`, and file
  reservations through Agent Mail replace branches.
- `ubs` runs on changed files before a commit. Beads are managed with `br` and triaged with `bv --robot-*`.
- Source that was never compiled does not land.

### 26.8 What ships with the first commit

`AGENTS.md`, `README.md` with an honest-status table, this plan, `franken-stack.lock`,
`DEPENDENCY_ADMISSION.md`, the registries (empty but schema-valid), and the initial beads. FrankenTextiles
shipped its companions with its plan and recorded that as worth repeating (FACT).

## 27. Numerics posture and determinism

### 27.1 Determinism classes

Every result records its class. A class is claimed by a contract and tested by the Gauntlet's G5 tier.

| Class | Guarantee | Where it applies |
|---|---|---|
| **D0 Exact** | Integer, rational or symbolic results | Static analyses over ℚ, group computations, certificates, world addressing |
| **D1 Bitwise across backends** | The same bits on `batch-cpu` (`f32`), CUDA and Metal | Algebraic kernels, if GATE-SPARK-01 passes for the device family. A HYPOTHESIS until then |
| **D2 Bitwise per backend** | The same bits run to run and across worker counts on one device family, driver and compiler | The design requirement for every kernel |
| **D3 Tolerance** | Agreement inside a recorded envelope | Cross-backend comparison when D1 is unavailable; cross-tier fidelity |
| **D4 Unspecified** | None | Never cited as evidence |

The referee tier in `f64` aims at FrankenSim's cross-ISA class on CPUs by using its deterministic elementary
functions; FrankenSim's `fs-mbd` currently calls platform `sqrt`, `sin` and `cos` and makes no cross-ISA claim
(FACT), so that is ask FD-FSIM-07.

### 27.2 Hazards and rules

| Hazard | Fact | Rule |
|---|---|---|
| Platform elementary functions | They differ across libraries and GPUs | Only `det_*` functions written in the dialect or from `fs-math`; a lint bans the built-ins in kernel sources |
| Fused multiply-add | Compilers contract a·b + c by default on some backends | Contraction off on every backend; an explicit fused operation only where all backends are measured to round identically |
| Reductions | Order changes the sum | Fixed-shape pairwise trees; compensated sums for long accumulations |
| Floating-point atomics | The main source of GPU nondeterminism (FACT, Appendix H) | Banned. Count, exclusive scan, write |
| Sorting | Ties and unstable sorts reorder contacts | Stable sorts with total order keys (value, then index) |
| Hardware texture filtering | Fixed-point weights differ by vendor | Manual interpolation in kernel source |
| Subnormals on Metal | The specification permits flushing; no control (FACT) | Measured at GATE-SPARK-01; D1 claimed only where behaviour matches |
| Hash-map iteration | Order is not specified | Not allowed to affect outputs |
| Warm starts | Solver state carries across steps | Warm-start buffers are part of every snapshot |
| Thread scheduling | Completion order varies | Results are placed by index, never by arrival |
| Toolchain and driver changes | Code generation moves | The kernel cache is keyed by compiler and driver; a change re-runs `spark check` |

### 27.3 Anchored positions

Single precision resolves about 7.6 µm at 100 m from the origin and worse beyond, which is too coarse for contact
(GB-04). A world position is therefore an integer cell index plus an `f32` offset in a cell of side 8 m, where
resolution is at worst 0.48 µm. Moving between cells adds or subtracts 8.0, which is exact. Scene fields are
addressed cell-relative. Quantities that are differences (gaps, link-relative poses, velocities) never touch
absolute position.

### 27.4 Randomness

Counter-based streams keyed by logical identity: `(WorldAddr, step, stream_id)`, built on FrankenSim's Philox
streams (FACT: `fs-rand`). No device-side generator state exists. Common random numbers for a comparison are
therefore free: two designs given the same addresses see the same world.

### 27.5 Refusals

A refusal has a stable code, a message and ranked repairs. Families: `1xx` design and model; `2xx` numerics and
integrator admission; `3xx` scene; `4xx` control and estimation; `5xx` learning and evaluation protocol; `6xx`
backend and device; `7xx` budget and capability; `8xx` evidence and protocol (for example citing a provisional
result as established); `9xx` internal. Three standing rules: refuse, never truncate; a budget stop publishes
nothing partial as a result; a misspelt parameter is a refusal, not a default.

### 27.6 A number never claims more than its certificate

Every numerical result records what bounds it: solver gradient norm and iteration count, maximum constraint and
closure violation, group-membership error, the energy-ledger residual, the number of worlds marked `Degraded`
or `Faulted`. A summary statistic over worlds states how many worlds it excludes and why.

## 28. The performance program

### 28.1 Targets

All TARGETs. None is a measurement. References are FACTs from Appendix H and are context, not promises.

| Workload | Hardware | Target | Context |
|---|---|---|---|
| Quadruped, 12 actuated joints, 16 foot samples, flat and rough ground, h = 2 ms, 4 Newton iterations: physics only | One RTX 4090, 8,192 worlds | ≥ 2.0 M world-steps/s | No absolute RTX 4090 figure for the leading GPU simulators was found |
| Same task, PPO training including policy and learning | One RTX 4090 | ≥ 400 k env-steps/s | MuJoCo Playground reports 417 k for a quadruped on an A100 with MJX |
| Humanoid, 29 joints: physics only | One RTX 4090, 4,096 worlds | ≥ 500 k world-steps/s | Playground reports 106 k training throughput for G1 on an A100 |
| Two devices | Two RTX 4090 | ≥ 1.8× one device | Independent shards |
| Quadruped physics | M5 Ultra, Metal | ≥ 25% of one RTX 4090 (BET-09); stretch 60% | No published Metal physics benchmark for M5 was found |
| Quadruped physics, `batch-cpu`, `f32` | M5 Ultra, 36 cores | ≥ 150 k world-steps/s | HYPOTHESIS-grade |
| Referee, humanoid, one world | One CPU core | ≥ 0.2× real time | — |
| Height scan, 187 rays per world at 50 Hz | Any GPU | ≤ 15% added step cost | — |
| `design check` | CPU | < 2 s at the 95th percentile | — |
| `control synth` | CPU | < 5 s | — |
| Interior scene generate and compile | CPU | < 60 s | — |
| Wall-clock to train the reference quadruped task | One RTX 4090 | ≤ 15 min | Playground reports about 5 min on two RTX 4090s for a comparable task |

### 28.2 The disclosure block

Any Performance claim carries all of: hardware and driver; backend, compiler and kernel versions; world count;
time step, substeps and control decimation; solver iterations and tolerance; contact capacity and mean active
contacts per world; sensors enabled; whether the policy and learning are included; scene; determinism class;
warm-up and duration; whether launch graphs were captured; median and 99th-percentile step time; peak memory. A
public dispute over one simulator's headline number, later re-measured two orders of magnitude lower under
corrected settings (FACT, Appendix H), is the reason this block is mandatory.

### 28.3 Method

1. **Profile before optimizing.** Kernel microbenchmarks precede physics tuning. The first profile at M2 decides
   where effort goes.
2. **Account for the whole workflow.** Transfers, synchronization, compilation, resets and telemetry stay in the
   denominator (the rule in FrankenSim's accelerator doctrine, FACT).
3. **One lever at a time, with a behaviour proof.** An optimization lands only with identical digests (classes D1
   and D2) or an unchanged fidelity battery.
4. **Paired, interleaved runs** on a quiet host, with a thermal soak on the Mac. `trj` is often heavily loaded
   (FACT), so its measurements record the load.
5. **Regressions are test failures** against a tolerance band, on the smoke battery.

HYPOTHESIS about where time goes, to be replaced by the M2 profile: the constraint solve 40 to 60%, collision 15
to 25%, dynamics terms 10 to 20%.

### 28.4 Memory

Per-world state for a quadruped is a few kilobytes; constraint rows and Jacobians add tens of kilobytes at the
default capacities. Tens of thousands of worlds fit in 24 GB. Scenes dominate (§16.8). This Mac's 256 GB of
unified memory makes it the large-scene machine. `sim estimate` reports all of this before a run, from a cost
model fitted to recorded runs.

## 29. Safety, security and privacy

- **29.1 Agents supply data, not code.** In v1 an agent submits designs, task specifications, controller
  configurations and policy weights. It cannot submit kernels or arbitrary programs. Sandboxed custom reward
  terms are Frontier.
- **29.2 Resource governance.** Budgets, quotas and draining cancellation (§24.4); device-memory and disk
  preflight; the compute envelope of OPEN-SPARK-2.
- **29.3 Private places stay private.** A scene pack built from a scan of a real home or workplace is marked
  `private`. It is stored under a configured private root outside the repository. Stills, replays, reports and
  fixtures derived from it inherit the mark and are not exported without an explicit flag. The owner's
  reconstruction skills state the same rule (FACT).
- **29.4 Licenses are enforced at pack time.** Each asset has a manifest entry. Research-only assets cannot enter
  an exported report or a redistributed pack.
- **29.5 Supply chain.** Exact pins; admission records with real build evidence; kernel caches verified by
  content hash before load.
- **29.6 No safety certification.** Simulation results are not a safety case for a physical robot. Reports carry
  that no-claim line.

---
# PART V. VERIFICATION, GATES, RISKS AND OPEN QUESTIONS

## 30. Gauntlet: the verification program (`frob-conformance`)

### 30.1 Six tiers

The tiers are FrankenSim's (FACT), read for robots.

| Tier | Kind | Robotics content |
|---|---|---|
| G0 | Laws | The algebraic laws of §11.3, functor laws of C-12, cone laws, dimension checks on every kernel |
| G1 | Known answers | Analytic and synthetic truths: §14.7, §15.7, actuators, sensors, analyses |
| G2 | Parity | Agreement with pinned oracles (§30.2) and published benchmark problems |
| G3 | Metamorphic | Relations that must hold without knowing the answer (§30.4) |
| G4 | Chaos | Cancellation at every boundary, injected faults, capacity overflow, device loss, malformed input |
| G5 | Determinism | Twice on one machine; across worker counts; debug and release; across platforms; across backends, each against its class |

"A false certificate is worse than an ordinary wrong answer" (FACT: FrankenSim's agent instructions). The
Gauntlet's first duty is to the things that issue certificates and verdicts.

### 30.2 Oracles and pins

| Oracle | Pin | Use | Posture |
|---|---|---|---|
| `fs-mbd`, `fs-contact`, `fs-query` | The FrankenSim commit in `franken-stack.lock` | Parity for batched kinematics, dynamics and primitive distances; the referee itself | Consume |
| MuJoCo C, double precision | An exact release, recorded with the fixture | Parity for contact-free dynamics of reference robots; import fixtures | Development-time, out of process |
| A reference rigid-body library with analytical derivatives (optional) | Exact release | Parity for derivative kernels | Development-time, out of process |
| FrankenTorch autograd | The pinned commit | Gradient oracle for on-device learning kernels | Oracle |
| FrankenSciPy `solve_ivp` (DOP853) | The pinned version | Reference trajectories for smooth known answers | Oracle |
| FrankenSymPy evaluation | The pinned commit | Reference values for derived arithmetic | Oracle |
| Analytic derivations | `docs/derivations/`, one file per known answer | Known answers | — |

Rules:

1. A fixture is captured by a script under `tools/oracle/`, stored with its pin and date, and **never
   regenerated from the candidate**.
2. Moving a pin is a deliberate event with its own record.
3. Foreign tools run out of process and never enter a production dependency graph (FrankenSim's adapter rule,
   FACT).
4. Agreement with an oracle is Parity. It is never reported as Reality.

### 30.3 Known answers

Collected from the chapters: group and cone laws (§11.3); mobility of Sarrus, Bennett and four-bar linkages
(C-01); growth vectors (C-07); Elroy's beanie (C-08); pendulum, top and ballistic invariants (GATE-KIN-01);
the contact battery (§15.7); motor and thermal constants (GATE-SINEW-01); sensor readings (GATE-SENS-01); the
mass-spiral fixed point (GB-03); leg IK solution counts (C-11); energy-ledger closure for a damped pendulum
(GATE-CHRON-03). Each has a derivation on file and a negative control.

### 30.4 Metamorphic relations

Generated from Part II (D-4). Each relation is a test over random scenarios on every tier.

| Relation | Source |
|---|---|
| Applying a symmetry element to the initial state and the actions gives the symmetry element applied to the result | C-09 |
| Changing the world frame by a horizontal translation and a yaw changes nothing an invariant observation reports | C-09 |
| Re-expressing every quantity in another body frame leaves powers and energies unchanged | C-02 |
| Scaling the unit system leaves dimensionless outputs unchanged | C-14 |
| Halving the time step moves results by the integrator's order | C-05 |
| Permuting world indices permutes the results and changes nothing else | §14.3 |
| Adding inactive worlds or unused capacity changes no active world's digest | §14.3 |
| Refining the scene field moves contact results by no more than the interpolation bound | §16.2 |
| Splitting a rollout at a snapshot and resuming equals running it whole | §23.3 |
| An edit followed by its inverse returns the same `DesignId` | §12.2 |
| A design built with `mirror` equals its hand-expanded form | C-12 |

### 30.5 The standard battery

For each robot class, about 200 scenarios: stand, push recovery, each atlas gait at three speeds, turning,
stairs up and down, a slope, a low-friction patch, a payload, and a drop, crossed with randomized parameters
under fixed seeds. It is defined and frozen at M2 (BD-104). The fidelity ledger, the divergence ledger and the
exploitation detectors all read it, so a change to the battery is a recorded event.

### 30.6 Chaos

Cancellation is injected at every kernel-batch boundary under a deterministic schedule (asupersync's lab runtime
provides virtual time and replay, FACT). Faults are injected through the backends' test seams. The ledger is
killed mid-write. Importers and the envelope parser are fuzzed. Capacity overflow is forced. Every case must end
in its typed refusal with no leaked resource.

### 30.7 The Reality lane

Reality is the one claim v1 mostly cannot make. The lane exists so that the first admitted dataset has somewhere
to land: a dataset of real-robot measurements enters with its license, its provenance and its measurement
uncertainty, and a comparison is recorded as a Reality result with its scope. Until then every report prints
`reality: no-claim`. FrankenTextiles recorded the same limit about its own domain: "More numerical permutations
cannot supply those observations" (FACT).

### 30.8 Guarding against false progress

The sibling projects documented how agent swarms produce the appearance of progress. These rules are the
response.

- **Typed close reasons.** A bead closes with a reason that cites its own test and that test's output. A shared
  close reason across many beads is itself a failure signal.
- **Negative controls.** Every gate has at least one planted failure it must catch, registered in
  `frob-conformance`. "A gate no deficient model can fail validates nothing" (FACT: Wright Flyer plan).
- **Live-dependency proofs.** For a gate that claims a result depends on an input (a scene, a card, a policy),
  the test perturbs the input and requires the output to change.
- **Synthetic is labeled.** Output computed from a fixture or a stand-in prints `synthetic` in its evidence.
- **Behaviour, not plumbing.** A learning gate needs a learned behaviour and failing controls (the zero-weight
  checkpoint lesson, §19.4).
- **No self-certification.** A gate's PASS is recorded by a fresh-context session that did not implement the
  capability.

### 30.9 Certifying the certifiers

The certificate verifier must reject mutated certificates. The dimension checker must reject seeded violations.
The e-process must hold its error rate under the null with optional stopping. The physical-consistency test must
reject a matrix one eigenvalue below zero. These are G0 tests of the Gauntlet itself.

### 30.10 Status accounting

`registries/gates.toml` holds each gate's claim type, threshold, status, measured value, date and the recording
session. The README's honest-status table is generated from it.

## 31. The gate register

A gate passes only with a recorded measurement against its threshold. A kill gate (✖) names its fallback in
advance and is never quietly re-scoped. Thresholds are in the sections cited.

### 31.1 Admission gates

| Gate | Foundation | First needed | Section |
|---|---|---|---|
| INT-TOOLCHAIN-01 | Rust toolchain shared with FrankenSim | M0 | §25.6 |
| INT-FSIM-01 | frankensim | M0 | §25.6 |
| INT-ASUP-01 | asupersync | M0 | §25.6 |
| INT-FSQL-01 | frankensqlite | M0 | §25.6 |
| INT-FT-01 | frankentorch (`ft-kernel-metal`) | M0 | §25.6 |
| INT-CUDA-01 | `cudarc` | M0 | §25.6 |
| INT-FSYM-01 | frankensympy | M0 | §25.6 |
| INT-FMCP-01 | fastmcp-rust | M0 smoke, M1 full | §25.6 |
| INT-FNP-01 | franken_numpy | M2 | §25.6 |
| INT-FLN-01 | franken_lean | M5 | §25.6 |

### 31.2 Convergence gates

| Gate | Claim | Earliest | Section |
|---|---|---|---|
| GATE-MARROW-01, -02 | Known answer | M0, M1 | §11.4 |
| GATE-LEMMA-00, -01, -02 | Proof | M0, M5, M5 | §10.6 |
| GATE-SKEL-01, -02, -03 | Parity, known answer, known answer | M1 | §12.7 |
| GATE-SINEW-01, -02, -03 | Known answer, known answer, fidelity | M1, M1, M2 | §13.5 |
| GATE-KIN-01, -02, -03 | Known answer, parity, parity | M1, M1, M2 | §14.7 |
| GATE-FID-01, -02 | Fidelity | M2, M3 | §14.7 |
| GATE-TRAC-01, -02, -03 | Known answer, parity, fidelity | M1 and M2, M1, M2 | §15.8 |
| GATE-HAB-01, -02, -03 | Known answer, known answer, reality | M2, M3, M5 | §16.9 |
| GATE-SENS-01, -02, -03 | Known answer, fidelity, known answer | M2 | §17.6 |
| GATE-REFLEX-01, -02, -03 | Known answer, method, known answer | M1, M3, M1 | §18.6 |
| GATE-CORTEX-01 to -05 | Known answer, parity, method, known answer, performance | M2, M2, M4, M3, M2 | §19.8 |
| GATE-SPARK-00 to -06 | Known answer, known answer, fidelity, performance, performance, known answer, known answer | M0, M0, M2, M2, M2, M2, M2 | §20.10 |
| GATE-ARENA-01, -02 | Known answer | M2, M3 | §21.4 |
| GATE-ATEL-01 to -04 | Method, known answer, method, known answer | M3, M1, M4, M3 | §22.9 |
| GATE-CHRON-01, -02, -03 | Known answer | M2, M2, M1 and M2 | §23.7 |
| GATE-TETHER-01, -02, -03 | Method, known answer, known answer | M3, M1, M0 | §24.10 |
| GATE-GAUNT-01 | Known answer: every gate above has a registered negative control that it catches | Every milestone exit | §30.8 |

### 31.3 Kill gates

| Kill | Trigger | Fallback | Decided at |
|---|---|---|---|
| ✖ KILL-FSIM | INT-FSIM-01 cannot pass, or upstream breakage exceeds BET-04's threshold | Lift the minimal referee set with provenance; say "lift-only" | M0, then rolling |
| ✖ KILL-KIR | The derivation toolchain cannot cover the kernel arithmetic, or misses BET-01's speed bound | Hand-written arithmetic in Rust and the dialect under differential tests | M2 |
| ✖ KILL-CONTACT | GATE-FID-01 fails on slip or creep after one repair cycle | CL-2 as default at lower throughput; then referee-only locomotion claims | M2 |
| ✖ KILL-METAL | GATE-SPARK-04 fails, or run-to-run determinism cannot be held | Metal becomes the development and replay tier; the Mac trains on `batch-cpu` | M2 |
| ✖ KILL-MCP | INT-FMCP-01 fails | Separate-workspace server over a local socket; then CLI only | M0 to M1 |
| ✖ KILL-STATIC | GATE-ATEL-01 fails (BET-05) | `design check` verdicts are downgraded to advisory lints and the pruning claim is withdrawn | M3 |
| ✖ KILL-BASELINE | GATE-REFLEX-02 fails (BET-06) | Baseline synthesis is limited to the morphology classes that pass; others report `not-synthesizable` and route to training | M3 |
| ✖ KILL-EQUIV | GATE-CORTEX-03 fails (BET-07) | Symmetry is used for augmentation and metamorphic testing only | M4 |
| ✖ KILL-LEMMA | GATE-LEMMA-01 below 50% after one repair cycle | Certificates become exact-rational randomized tests recorded as `Tested` | M5 |

### 31.4 Program gate

At the end of M1: if M0 plus M1 took more than 1.5 times their planning estimate because a sibling could not
supply a primitive and no fallback applied (BET-12), the program pauses for a written re-plan before M2 starts.

## 32. Risks and the pre-mortem

### 32.1 It is late 2028 and FrankenRobotics failed. Why?

| # | Failure story | Leading indicator | Countermeasure in this plan | Residual |
|---|---|---|---|---|
| 1 | The mathematics became the product. Elegant algebra crates grew and no robot walked. | Layer-1 crates grow while M1 exits slip | D-2; milestone exits are robots moving; the moonshot cap D-6 | The temptation is constant |
| 2 | FrankenSim moved weekly and adapter churn consumed the effort. | BET-04's breakage metric | Pinned commit, the airlock, the compatibility train, KILL-FSIM | A lift fork would be large |
| 3 | GPU kernels passed tests and diverged in the field, so nobody trusted fast results. | Divergence-ledger entries without explanations | `spark check` on every driver or compiler change; teleport; the promotion rule | Class D3 comparisons are statistical |
| 4 | Throughput landed ten times under target and agents stopped waiting. | The M2 profile | Profile first; KILL-KIR; the fidelity ladder reduces how many steps a decision needs | Targets are unmeasured |
| 5 | Policies exploited contact compliance and promotions kept failing. | Promotion failure rate | The trust ladder, the detectors of §19.6, KILL-CONTACT | Convex models have real artifacts |
| 6 | Baseline controllers failed on new morphologies, so every design needed training after all. | GATE-REFLEX-02 trend | Three-outcome reporting; KILL-BASELINE; behaviour cloning warm starts | Novel bodies are hard |
| 7 | The agent surface was too large and agents got lost. | GATE-TETHER-01 failures | Clusters of at most seven tools; `next_actions`; `robot-docs`; the surface A/B (BET-10) | — |
| 8 | The swarm closed beads that proved plumbing. | Shared close reasons; gates without negative controls | §30.8 in full | Requires discipline at every exit review |
| 9 | Interiors were toy boxes and results did not transfer to real places. | Reviewer inspection of generated scenes | OPEN-HAB-1 norms; asset pipeline; real-place ingestion | Realism is unbounded work |
| 10 | Sim-to-real was never tested and claims overreached. | Any report without a Reality line | The six claims; `reality: no-claim`; card provenance | No hardware is in scope |
| 11 | "Certified" became a marketing word while the proof tools stayed immature. | Certificates without replay hashes | §10.2 wording rule; KILL-LEMMA | Depends on sibling progress |
| 12 | Single precision failed in large scenes and long runs. | GATE-SPARK-05 | Anchoring, normalization, double-float accumulators | — |
| 13 | The GPU mandate and the suite's no-FFI doctrine were never reconciled, and work stalled on policy. | OPEN-SPARK-1 still open at M2 | The annex design; asking the owner at M0 | Owner decision |
| 14 | The Metal gateway's limits throttled the Mac tier. | Step-plan launch counts on Metal | Packed arguments; asks FD-FT-01; direct `objc2-metal` fallback | — |
| 15 | The plan itself became the delivery log and stopped being readable. | This file growing past its outline | §0.7 | — |

### 32.2 Risk register

| ID | Risk | Severity | Mitigation | Owner |
|---|---|---|---|---|
| RISK-01 | FrankenSim API churn in `fs-mbd` and the `Component` trait | High | Airlock, pin, ask FD-FSIM-08 | Airlock lead |
| RISK-02 | Toolchain split between FrankenSim and the rest of the suite | High | INT-TOOLCHAIN-01; separate workspaces with their own toolchain files | Build lead |
| RISK-03 | Two asupersync versions in one build | High | INT-ASUP-01; process boundary fallback | Build lead |
| RISK-04 | Bit equality across backends is unattainable on Metal | Medium | Classes D2 and D3 are sufficient for every gate; D1 is a bonus | Spark lead |
| RISK-05 | The convex contact model's creep misleads wheel and stair results | High | Fidelity ledger quantities for creep; CL-2; KILL-CONTACT | Traction lead |
| RISK-06 | Derived kernels are slower than hand-written ones | Medium | BET-01 bound; KILL-KIR | Spark lead |
| RISK-07 | FrankenSymPy budgets block derivations or certificates | Medium | Derive per limb, not per robot; BET-11; KILL-LEMMA | Lemma lead |
| RISK-08 | The Metal gateway is synchronous and limited to 31 arguments | Medium | Packed buffers; asks; direct binding fallback | Spark lead |
| RISK-09 | `trj` is heavily loaded, so measurements are noisy and compiles slow | Medium | Precompiled kernel cache; recorded load; interleaved runs | Performance lead |
| RISK-10 | No compute envelope for training is approved | High | OPEN-SPARK-2 raised at M0 | Owner |
| RISK-11 | Asset licenses restrict what can ship | Medium | Procedural-first; license manifests enforced at pack time | Habitat lead |
| RISK-12 | A scan of a private place leaks through a still or a fixture | High | §29.3 inheritance of the private mark; export flag | Chronicle lead |
| RISK-13 | Reference robot imports hide lossy contact mappings | Medium | Loss reports; CL-M labeled parity-only | Skeleton lead |
| RISK-14 | The design corpus is tuned to the analyses | Medium | Labels frozen before tuning; a held-out third of the corpus | Arena lead |
| RISK-15 | Agents optimize the scorecard instead of the robot | Medium | Promotion, detectors, held-out scenes, tail metrics | Atelier lead |
| RISK-16 | Lean work needs a large-memory Linux host | Low | Measure closure size first (OPEN-LEMMA-1) | Lemma lead |
| RISK-17 | Baseline controllers mask design flaws or create false ones | Medium | Three-outcome reporting; estimated-versus-privileged comparison | Reflex lead |
| RISK-18 | Crate scope caps are exceeded early | Medium | The 115% conversation; no silent growth | Project lead |
| RISK-19 | The viewer drifts outside FrankenThreeD's admitted subset | Low | Plain Three.js is the fallback and is always built | Viewer lead |
| RISK-20 | Upstream asks are never staffed | Medium | Every ask has an interim posture that ships | Project lead |

## 33. Open questions and the decision rule

### 33.1 The rule

Every OPEN has an owner, a deadline expressed as the gate or milestone it blocks, a fail-closed default, and the
evidence that would decide it. Nothing is decided by silence: at its deadline an undecided question takes its
default, and the default is recorded as an ADR.

### 33.2 Questions for the owner

| ID | Question | Default if undecided | Blocks |
|---|---|---|---|
| OPEN-SPARK-1 | Ratify ADR-SPARK-01: GPU backends as annex workspaces with FFI capsules, outside the Franken-only core | GPU results are labeled exploratory | M2 exit |
| OPEN-SPARK-2 | A compute envelope for training campaigns on `trj` and this Mac | One device, one hour, 16 GiB, 8 cores per job | GATE-CORTEX-05, M3 onward |
| OPEN-SINEW-1 | Which real actuator parts enter the catalog, and their data licenses | Generic classes only | M4 |
| OPEN-HAB-1 | Sources of dimensional norms for the seven archetypes | Generic ranges labeled `assumed` | Realism claims at M3 |
| OPEN-GAUNT-1 | Which real-robot dataset, if any, is admitted to the Reality lane first | `reality: no-claim` everywhere | M5 |
| OPEN-PROG-1 | Public or private repository, and the license file | Private; the suite's license file | First push |

### 33.3 Engineering questions

| ID | Question | Default | Decided by |
|---|---|---|---|
| OPEN-MARROW-1 | Canonicalize quaternion sign in state? | No | GATE-MARROW-01 |
| OPEN-SKEL-1 | A text syntax or FrankenScript embedding for designs? | JSON only | Before M3 |
| OPEN-KIN-1 | Default time step and Newton iterations | 2 ms, 4 iterations | The M2 fidelity-throughput frontier |
| OPEN-KIN-2 | Sleeping and islands | No | M2 profile |
| OPEN-TRAC-1 | Dense or sparse bricks for interiors | Dense with a coarse outer level | M3 memory measurements |
| OPEN-REFLEX-1 | Predictive control on device | CPU tiers only | M3 |
| OPEN-ARENA-1 | Which public task definitions to mirror | Public velocity-tracking reward terms; no score parity claim | M2 |
| OPEN-ARENA-2 | A permissive hexapod reference model | None | M4 |
| OPEN-LEMMA-1 | Memory and time to re-check one theorem's closure with FrankenLean | Measure before any claim | INT-FLN-01 |
| OPEN-SUITE-1 | Which asupersync version the whole graph resolves to | Whatever FrankenSim's pinned commit builds with | INT-ASUP-01 |
| OPEN-CHRON-1 | Column compression without a third-party codec | Uncompressed quantized columns | M2 |

---
# PART VI. DELIVERY

## 34. Milestones M0 to M6

### 34.1 Shape of the program

The first wedge is chosen for the same reason FrankenMining chose its own: start where value is deliverable with
the fewest unproven dependencies. **M1 is a design studio on the CPU referee tier.** It needs only FrankenSim,
asupersync and FrankenSQLite (§25.7), and it already demonstrates the thesis: a typed design, static analyses
from Lie theory, a synthesized controller, a rollout, and structured telemetry, all through one verb registry.
GPUs arrive at M2 on top of something that already works.

Planning estimates, in elapsed weeks for a swarm. These are estimates to make BET-12 testable, not acceptance
evidence: M0 one, M1 three, M2 four, M3 four, M4 four, M5 four.

### 34.2 M0: spine and airlock

- **Purpose.** Retire the risks that could invalidate the architecture, with the smallest possible amount of
  code. No subsystem is built out.
- **Artifact.** `frob doctor` lists admitted foundations with their records. `frob sim run g1 --tier referee
  --seconds 2` produces a state digest equal to the one FrankenSim computes for itself. `frob spark probe` runs
  on both device families. `K-cay-step` runs from one source on `batch-cpu`, CUDA and Metal. T-01 has a
  certificate that replays offline. The M0 verbs are callable over MCP.
- **Acceptance.** INT-TOOLCHAIN-01, INT-FSIM-01, INT-ASUP-01, INT-FSQL-01, INT-FT-01, INT-CUDA-01, INT-FSYM-01,
  INT-FMCP-01 (smoke); GATE-MARROW-01 for SO(3) and SE(3); GATE-SPARK-00; GATE-SPARK-01; GATE-LEMMA-00;
  GATE-TETHER-03; GATE-GAUNT-01.
- **Kill.** KILL-FSIM and KILL-MCP are decided here. **Fallbacks:** the lift set; the socket server.
- **Owner input wanted.** OPEN-SPARK-1 and OPEN-SPARK-2.
- **Work packages.** WP-01 to WP-06.

### 34.3 M1: referee and static studio

- **Artifact.** The design language and model compiler; templates for a quadruped, a humanoid, a differential
  drive and a mecanum base; URDF and MJCF import with loss reports; `design check` with A-01, A-02, A-03, A-08,
  A-10, A-11 and A-13, plus A-05, A-06, A-07 and A-12 as separate verbs; actuator levels L1 and L2; referee
  rollouts on planes and primitive obstacles; the invariant filter; baseline controllers for legged and wheeled
  templates; telemetry levels T0 to T2 with the energy ledger; `explain`; every M1 verb on the CLI and over MCP.
- **Demonstrates.** NS-01, NS-07.
- **Acceptance.** GATE-MARROW-02; GATE-SKEL-01, -02, -03; GATE-SINEW-01, -02; GATE-KIN-01, -02; GATE-TRAC-01
  (referee rows), -02; GATE-REFLEX-01 (referee), -03; GATE-ATEL-02; GATE-CHRON-03 (referee); GATE-TETHER-02;
  GATE-GAUNT-01.
- **Program gate.** §31.4.
- **Work packages.** WP-07 to WP-14.

### 34.4 M2: batch and Spark

- **Artifact.** The batched step on `batch-cpu`, CUDA and Metal; scene packs and terrain generators; contact law
  CL-1 against distance fields; proprioception, height scans, depth and lidar; stills; the environment API; PPO
  on device; the fidelity and divergence ledgers; replay and teleport; measured throughput with disclosure
  blocks.
- **Demonstrates.** NS-02, NS-10.
- **Acceptance.** GATE-KIN-03; GATE-FID-01; GATE-TRAC-01 (batched), -03; GATE-SINEW-03; GATE-HAB-01;
  GATE-SENS-01, -02, -03; GATE-SPARK-02 to -06; GATE-CORTEX-01, -02, -05; GATE-CHRON-01, -02, -03;
  GATE-ARENA-01; GATE-GAUNT-01.
- **Kill.** KILL-KIR, KILL-CONTACT, KILL-METAL are decided here, each recorded as an ADR.
- **Work packages.** WP-15 to WP-22.

### 34.5 M3: Habitat and Arena

- **Artifact.** Interior generators for the seven archetypes with fixtures and movable objects; the task
  families through tight-space maneuvering; baseline controllers for every template; anytime-valid comparison;
  the promotion rule and the exploitation detectors; the daemon, capabilities and the full MCP surface; the
  design corpus with frozen labels.
- **Demonstrates.** NS-03, NS-04, NS-09.
- **Acceptance.** GATE-HAB-02; GATE-REFLEX-02; GATE-ATEL-01, -04; GATE-FID-02; GATE-CORTEX-04; GATE-ARENA-02;
  GATE-TETHER-01; GATE-GAUNT-01.
- **Kill.** KILL-STATIC, KILL-BASELINE.
- **Work packages.** WP-23 to WP-27.

### 34.6 M4: co-design and breadth

- **Artifact.** Optimization with racing and a quality-diversity archive; lineage diff, blame and bisect;
  registered predictions; full symmetry support with equivariant networks; the gait atlas with max-plus timing;
  tire laws, tracks, wheel-leg hybrids and moving surfaces; the maneuverability index and connection curvature;
  reach-and-carry tasks; the agent-surface A/B.
- **Demonstrates.** NS-05, NS-06, NS-08.
- **Acceptance.** GATE-CORTEX-03; GATE-ATEL-03; known answers for each new contact and analysis lane;
  GATE-GAUNT-01.
- **Kill.** KILL-EQUIV.
- **Work packages.** WP-28 to WP-30.

### 34.7 M5: Lemma and reality

- **Artifact.** Certificate coverage across derived kernels; Lean theorem capsules re-checked by FrankenLean;
  the articulated variational integrator; real-place ingestion; the first Reality-lane record, or a recorded
  no-claim; the structural margin loop.
- **Demonstrates.** NS-11, NS-12.
- **Acceptance.** INT-FLN-01; GATE-LEMMA-01, -02; GATE-HAB-03; GATE-GAUNT-01.
- **Kill.** KILL-LEMMA.
- **Work packages.** WP-31, WP-32.

### 34.8 M6: frontier

Unscheduled epics, each needing its own short plan before work starts: gradients through contact with honest
bias and variance diagnostics; soft soil and granular terrain at scale; flexible links; soft robots; moving
people and vehicles; RGB sensing and splat scenes; OpenUSD; hydraulic actuation.

## 35. Work packages

"Depends on" is the coarse view of the bead graph in §36. After conversion to beads this table is regenerated
from the graph by transitive reduction, so the two cannot drift.

| WP | Name | Crates | Milestone | Gates | Depends on |
|---|---|---|---|---|---|
| WP-01 | Bootstrap and stack | workspace, `xtask`, `frob-core` | M0 | INT-TOOLCHAIN-01 | — |
| WP-02 | Airlock and referee smoke | `frob-fsim` | M0 | INT-FSIM-01, INT-ASUP-01, INT-FSQL-01 | WP-01 |
| WP-03 | Marrow core | `frob-lie`, `frob-algebra` | M0, M1 | GATE-MARROW-01, -02 | WP-02 |
| WP-04 | Spark spikes | dialect, annexes, backend trait | M0 | INT-CUDA-01, INT-FT-01, GATE-SPARK-00, -01 | WP-01 |
| WP-05 | Derivation spike | `frob-kir`, `derive/`, `frob-lemma` | M0 | INT-FSYM-01, GATE-LEMMA-00 | WP-03 |
| WP-06 | Verb registry and surfaces | `frob-verbs`, `frob-cli`, `frob-mcp` | M0, M1 | INT-FMCP-01, GATE-TETHER-02, -03 | WP-01 |
| WP-07 | Design language and compiler | `frob-design`, `frob-model` | M1 | GATE-SKEL-02, -03 | WP-03 |
| WP-08 | Import and export | `frob-io` | M1 | GATE-SKEL-01 | WP-07 |
| WP-09 | Actuators | `frob-actuate` | M1 | GATE-SINEW-01, -02 | WP-07 |
| WP-10 | Referee tier | `frob-dyn`, `frob-contact` | M1 | GATE-KIN-01, -02, GATE-TRAC-01, -02 | WP-02, WP-09 |
| WP-11 | Static analyses I | `frob-analyze` | M1 | GATE-ATEL-02 | WP-07, WP-09 |
| WP-12 | Reflex I | `frob-control` | M1 | GATE-REFLEX-01, -03 | WP-10 |
| WP-13 | Chronicle I | `frob-chronicle` | M1 | GATE-CHRON-03 | WP-10 |
| WP-14 | Gauntlet harness and oracles | `frob-conformance`, `tools/oracle` | M1 | GATE-GAUNT-01 | WP-02 |
| WP-15 | Kernel library and batched step | `frob-kernels`, `frob-dyn` | M2 | GATE-KIN-03 | WP-05, WP-07, WP-09 |
| WP-16 | Traction, batched | `frob-contact` | M2 | GATE-TRAC-01, -03 | WP-15, WP-17 |
| WP-17 | Scene packs and terrain | `frob-scene` | M2 | GATE-HAB-01 | WP-02 |
| WP-18 | CUDA backend | `spark/frob-spark-cuda` | M2 | GATE-SPARK-02, -03, -06 | WP-04, WP-15 |
| WP-19 | Metal backend | `spark/frob-spark-metal` | M2 | GATE-SPARK-02, -04, -06 | WP-04, WP-15 |
| WP-20 | Sensors I | `frob-sense` | M2 | GATE-SENS-01, -02, -03 | WP-16 |
| WP-21 | Cortex I | `frob-learn` | M2 | GATE-CORTEX-01, -02, -05 | WP-18, WP-19, WP-20 |
| WP-22 | Trust ladder and replay | `frob-dyn`, `frob-chronicle` | M2 | GATE-FID-01, GATE-CHRON-01, -02, GATE-SPARK-05 | WP-13, WP-16 |
| WP-23 | Interiors | `frob-scene` | M3 | GATE-HAB-02 | WP-17 |
| WP-24 | Arena | `frob-arena` | M3 | GATE-ARENA-01, -02 | WP-21, WP-23 |
| WP-25 | Atelier: compare, explain, promote, detect | `frob-opt`, `frob-learn` | M3 | GATE-ATEL-01, -04, GATE-FID-02, GATE-CORTEX-04 | WP-22, WP-24 |
| WP-26 | Tether, full | `frob-cli`, `frob-mcp` | M3 | GATE-TETHER-01 | WP-06, WP-25 |
| WP-27 | Reflex II | `frob-control` | M3 | GATE-REFLEX-02 | WP-12, WP-15 |
| WP-28 | Optimization and lineage | `frob-opt` | M4 | NS-06 | WP-25 |
| WP-29 | Symmetry and gaits | `frob-algebra`, `frob-analyze`, `frob-learn` | M4 | GATE-CORTEX-03 | WP-11, WP-21 |
| WP-30 | Wheels, tracks, hybrids | `frob-contact`, `frob-analyze` | M4 | Known answers; GATE-ATEL-03 with WP-28 | WP-16, WP-11 |
| WP-31 | Lemma | `frob-lemma`, `lean/`, `derive/` | M5 | INT-FLN-01, GATE-LEMMA-01, -02 | WP-05, WP-15 |
| WP-32 | Reality and ingestion | `frob-scene`, `frob-conformance`, `frob-analyze` | M5 | GATE-HAB-03 | WP-23, WP-22 |
| WP-33 | Frontier epics | — | M6 | Per-epic plans | — |

## 36. The first beads

These are the seed for conversion with `br`. Titles name the artifact; "Done when" is one observable, usually
with a negative control. Beads through M2 are tasks; later ones are epics to be split when their milestone
opens. Every bead inherits §0.2 and §30.8.

### 36.1 M0

| Bead | Title | Depends on | Done when |
|---|---|---|---|
| BD-001 | Core workspace skeleton: `Cargo.toml`, toolchain file at FrankenSim's pin, `franken-stack.lock`, the asupersync patch entry, FrankenSQLite profile overrides | — | `cargo check` is green on the empty workspace through the offload shim |
| BD-002 | Companion documents: `AGENTS.md`, `README.md` with a generated honest-status table, `DEPENDENCY_ADMISSION.md`, schema-valid empty registries | BD-001 | The status table regenerates from `registries/gates.toml` |
| BD-003 | `xtask check-layers` and `check-deps` | BD-001 | A deliberate upward edge and a forbidden dependency each fail |
| BD-004 | `xtask check-stack`, `check-contracts`, `check-unsafe`, `check-generated` | BD-003 | A seeded violation fails each check |
| BD-005 | `frob-core`: identities and hashing, evidence classes, the refusal type and registry loader | BD-001 | Unit tests pass; an unknown refusal code is a compile-time or load-time error |
| BD-006 | INT-TOOLCHAIN-01 and INT-FSIM-01: path dependencies on `fs-ga`, `fs-mbd`, `fs-contact`, `fs-time`, `fs-query` | BD-001 | An admission record with the command, its output and the commit |
| BD-007 | `frob-fsim` v0: `RefereeDynamics` over `fs_mbd::articulated`; the G1 digest test | BD-006 | The digest equals the one computed inside FrankenSim at the pinned commit; a perturbed input changes it |
| BD-008 | INT-ASUP-01 wired into `check-stack` | BD-006 | `cargo tree -d` evidence recorded; a seeded second version fails |
| BD-009 | INT-FSQL-01: ledger open, write, forced kill, recover | BD-006 | The forced-kill test passes 100 times |
| BD-010 | `frob-lie`: traits; SO(3) and SE(3) over `fs-ga`; the PO-MARROW-01 fixture | BD-006 | Bit-equal on 1,000 transforms |
| BD-011 | `frob-lie`: Cayley and exponential retractions with derivatives; laws for SO(3) and SE(3) | BD-010 | GATE-MARROW-01 rows for these groups, with a planted sign error caught |
| BD-012 | `frob-lie`: SO(2) as a unit complex number with a turn counter | BD-011 | PO-MARROW-03 over 10⁹ steps |
| BD-013 | Shared dialect prelude v0 for CUDA and Metal; the dialect lint | BD-001 | The lint rejects a built-in transcendental and a floating-point atomic |
| BD-014 | `KernelBackend` trait and the `batch-cpu` harness | BD-005 | A trivial kernel runs through the harness with a receipt |
| BD-015 | INT-CUDA-01 and `spark/frob-spark-cuda` v0: driver load, compile, cache, launch | BD-013, BD-014 | A kernel runs on both GPUs of `trj`; admission record |
| BD-016 | INT-FT-01 and `spark/frob-spark-metal` v0 over the `ft-kernel-metal` gateway | BD-013, BD-014 | A kernel runs on this Mac; admission record |
| BD-017 | `frob spark probe`: the IEEE probe kernels and report | BD-015, BD-016 | GATE-SPARK-01 results stored as device evidence for both families |
| BD-018 | `frob-kir` v0: straight-line programs, CSE, Rust and dialect emitters, the dimension checker | BD-005 | Emits `K-cay-step`; a seeded dimension violation is rejected |
| BD-019 | INT-FSYM-01 and `derive/frob-derive` v0: derive `K-cay-step` and `K-normalize` | BD-018 | Regeneration reproduces the checked-in sources bit for bit |
| BD-020 | GATE-SPARK-00: one source for `K-cay-step` on three backends | BD-015, BD-016, BD-019 | Agreement inside the class, recorded in the divergence ledger |
| BD-021 | `frob-lemma` v0: certificate envelope and offline verification; T-01, T-03, T-04 for the M0 kernels | BD-019 | GATE-LEMMA-00; a mutated coefficient is rejected |
| BD-022 | `frob-verbs` v0 and the `frob` skeleton: registry loader, envelope, exit codes, `doctor`, `capabilities`, `schema`, `robot-docs` | BD-005 | GATE-TETHER-03 on these verbs |
| BD-023 | INT-FMCP-01: `frob-mcp` generated from the registry for the M0 verbs | BD-022 | Protocol smoke passes; one asupersync package |
| BD-024 | `frob sim run --tier referee` for the built-in G1; digest and T0 telemetry to the ledger | BD-007, BD-009, BD-022 | A fresh session reproduces the M0 artifact |
| BD-025 | M0 exit review by a fresh-context session | BD-017, BD-020, BD-021, BD-023, BD-024 | Every M0 gate has a recorded value and a caught negative control |

### 36.2 M1

| Bead | Title | Depends on | Done when |
|---|---|---|---|
| BD-026 | `frob-lie`: SE(2), SE₂(3), SE₂₊ₖ(3), product groups | BD-011 | Laws pass |
| BD-027 | `frob-lie`: joint subgroups, Hervé classification (T-11), generated joint maps | BD-011 | Each of the twelve classes is recognized from a basis; a non-closed basis is reported as a product |
| BD-028 | Frame-typed transforms, twists and wrenches | BD-010 | Compile-fail tests for mismatched frames |
| BD-029 | `frob-algebra`: finite groups from generators, classes, characters, projectors | BD-005 | GATE-MARROW-02 |
| BD-030 | `frob-algebra`: the `Cone` trait; orthant, second-order and semidefinite cones | BD-005 | Cone laws; a planted non-idempotent projection is caught |
| BD-031 | `frob-algebra`: the inertia cone | BD-030 | Round trip; a matrix one eigenvalue below zero is rejected |
| BD-032 | `frob-algebra`: posets, antichains, Kleene iteration | BD-005 | GB-03's closed form reproduced; a divergent loop is reported infeasible |
| BD-033 | `frob-design`: term model, schema `frob.design/1`, canonical form, `DesignId` | BD-005 | Canonicalization is idempotent; the hash is equal on Linux and macOS |
| BD-034 | Constructors: `attach`, `serial`, `mirror`, `radial`, `scale`, `replace`, `closure` | BD-033 | `mirror` equals its hand-expanded form |
| BD-035 | Typed edits with preconditions, inverses and declared symmetry effects | BD-034 | An edit and its inverse return the same `DesignId` |
| BD-036 | `frob-model`: the kinematic functor, `RobotModel`, referee lowering | BD-034, BD-007, BD-027 | A template quadruped's forward kinematics runs on the referee |
| BD-037 | Geometry and inertia functor: Regions, materials, primitive fitting with recorded error | BD-036, BD-031 | INV-SKEL-03 on every template |
| BD-038 | Memoized incremental compilation | BD-036 | INV-SKEL-02 on 10⁴ random edits |
| BD-039 | Design lints with ranked repairs | BD-035 | Each lint has a fixture that triggers it and one that does not |
| BD-040 | Templates: quadruped, humanoid, differential drive, mecanum | BD-037 | Each compiles and passes its lints |
| BD-041 | `frob-io`: the XML reader; URDF import with a loss report | BD-036 | Reference robots import; a planted dropped field appears in the report |
| BD-042 | MJCF import with a loss report; URDF and MJCF export | BD-041 | GATE-SKEL-01 |
| BD-043 | Actuator cards: schema, provenance, generic catalog | BD-005 | A card without provenance is refused |
| BD-044 | Actuator levels L0 and L1 on the referee | BD-043, BD-036 | L1 rows of GATE-SINEW-01 |
| BD-045 | Actuator level L2: current, thermal network, battery | BD-044 | Remaining rows of GATE-SINEW-01 |
| BD-046 | Port energy-balance check on the referee | BD-044 | GATE-SINEW-02; a planted unaccounted loss is caught |
| BD-047 | `frob-contact`: collision primitives; contact sets from `fs-query` | BD-036 | GATE-TRAC-02 |
| BD-048 | The referee step: FrankenSim's contact step on generated contact sets; compliant feet | BD-047, BD-044 | Stand and drop tests |
| BD-049 | The contact known-answer battery on the referee | BD-048 | Referee rows of GATE-TRAC-01 |
| BD-050 | The dynamics known-answer battery | BD-048 | GATE-KIN-01 |
| BD-051 | Oracle capture scripts; fixtures for reference robots with pin and date | BD-042 | Fixtures on disk; the regeneration-from-candidate path does not exist |
| BD-052 | Contact-free dynamics parity | BD-051, BD-048 | GATE-KIN-02 |
| BD-053 | A-02 mass roll-up and A-08 static stability | BD-037, BD-030 | Known answers; a planted top-heavy design is flagged |
| BD-054 | A-01 mobility | BD-027, BD-036 | Sarrus 1, Bennett 1, planar four-bar 1 |
| BD-055 | A-03 symmetry detection with near-symmetry repairs | BD-029, BD-036 | Templates report their groups; a planted 0.4% asymmetry is found with its repair |
| BD-056 | A-10 co-design closure and A-11 actuator envelope | BD-032, BD-044 | A planted undersized actuator is found |
| BD-057 | A-13 self-collision and clearance | BD-047 | A planted limb interference is found |
| BD-058 | A-05 workspace, A-06 singularities, and the derived leg IK kernel | BD-019, BD-036 | A 3R leg has four solutions; the singular locus matches the closed form |
| BD-059 | A-07 growth vector and maneuverability | BD-036 | The growth vectors of C-07 |
| BD-060 | A-12 modes with symmetry block-diagonalization | BD-055 | Modes of a symmetric two-mass model |
| BD-061 | `design check`: aggregation, repairs, timing | BD-053, BD-054, BD-055, BD-056, BD-057 | GATE-ATEL-02 for the included analyses |
| BD-062 | The invariant filter | BD-026 | GATE-REFLEX-03 on a synthetic IMU |
| BD-063 | The small dense cone-program solver | BD-030 | KKT residuals; agreement with a reference optimizer on 10³ programs |
| BD-064 | Legged baseline controller | BD-063, BD-058, BD-048 | A quadruped stands and walks 5 m on the referee |
| BD-065 | Wheeled baseline controller | BD-048 | Differential and mecanum templates traverse 5 m |
| BD-066 | `control synth` with three-outcome reporting | BD-064, BD-065 | GATE-REFLEX-01 on the referee |
| BD-067 | `frob-chronicle`: ledger schema, episodes, telemetry levels T0 to T2 | BD-009, BD-048 | Round-trip tests |
| BD-068 | The energy ledger on the referee | BD-067, BD-046 | GATE-CHRON-03 on the referee |
| BD-069 | Failure taxonomy, first-cause ordering, budgeted summaries | BD-067 | A fixture per failure class |
| BD-070 | The M1 verbs with schemas, examples and refusal lists | BD-022, BD-061, BD-066, BD-069 | GATE-TETHER-02 for the M1 verbs |
| BD-071 | MCP tools generated for the M1 verbs | BD-023, BD-070 | The parity diff is empty |
| BD-072 | `frob-conformance`: tiers G0 to G5 and the negative-control registry | BD-005 | GATE-GAUNT-01 for M1 |
| BD-073 | M1 exit review by a fresh-context session; the program gate | BD-052, BD-070, BD-071, BD-072 | Gate values recorded; NS-01 and NS-07 demonstrated |

### 36.3 M2

| Bead | Title | Depends on | Done when |
|---|---|---|---|
| BD-074 | Packed lowering of `RobotModel`: depth levels, path tables, capacities | BD-036, BD-018 | The layout digest is stable across platforms |
| BD-075 | Derive and emit `K-fk-level` and `K-twist-level` | BD-019, BD-074 | Parity with `fs-mbd` forward kinematics |
| BD-076 | Derive and emit `K-crba-level` and `K-rnea-level` | BD-075 | Rows of GATE-KIN-03 |
| BD-077 | `K-act-l1`, `K-act-l2`, `K-passive` | BD-074, BD-045 | GATE-SINEW-03 |
| BD-078 | Utility kernels: scan, stable sort, fixed-tree reduce, compaction | BD-014 | Bitwise equal across worker counts |
| BD-079 | `K-jac-rows`, `K-cone-setup`, `K-cone-eval` for CL-1 | BD-076, BD-030 | KKT and projection tests |
| BD-080 | `K-hess`, batched dense `K-chol`, exact `K-linesearch` | BD-079 | Convergence on the battery; the gradient-norm certificate is recorded |
| BD-081 | Articulated `K-cay-step` and `K-normalize` | BD-075 | Group-membership test |
| BD-082 | The batched step on `batch-cpu` in `f64` and `f32`, with lockstep masks and capacity flags | BD-077, BD-078, BD-080, BD-081 | A quadruped drops, stands and walks under the baseline controller |
| BD-083 | The tile program on `fs-exec` with fixed-slot reductions | BD-082 | Bitwise equal across worker counts |
| BD-084 | `frob-scene`: the scene-pack format, bricks, interpolation bounds | BD-005 | Schema and round trip |
| BD-085 | The scene compiler through the airlock | BD-084, BD-006 | GATE-HAB-01 |
| BD-086 | Terrain generators with a difficulty parameter | BD-085 | The same seed gives the same `SceneId` on both platforms |
| BD-087 | `K-geom-pose`, `K-np-sdf`, `K-np-prim`, `K-bp-*` | BD-078, BD-084 | The sphere-against-field known answer |
| BD-088 | Friction cones, rolling and torsional rows, interface cards | BD-079 | Incline, rolling and mecanum known answers |
| BD-089 | The contact battery on batched tiers; the tunneling audit | BD-087, BD-088, BD-049 | GATE-TRAC-01 (batched), GATE-TRAC-03 |
| BD-090 | CUDA backend: buffers, step plan, launch-graph capture, receipts | BD-015, BD-082 | The full step runs on one device |
| BD-091 | Sharding across both GPUs | BD-090 | The two-device figure is measured and recorded |
| BD-092 | Metal backend: packed arguments, batch chains, receipts | BD-016, BD-082 | The full step runs on this Mac |
| BD-093 | Device faults and cancellation | BD-090, BD-092 | GATE-SPARK-06 |
| BD-094 | `frob spark check` and the divergence ledger | BD-090, BD-092 | GATE-SPARK-02 |
| BD-095 | Anchored positions and the precision battery | BD-082 | GATE-SPARK-05 |
| BD-096 | File the upstream asks of Appendix F as beads in the sibling trackers | BD-006, BD-016 | Bead identifiers recorded in `registries/asks.toml` |
| BD-097 | Ray kernels; height scan, depth, lidar; proprioception with noise | BD-087 | GATE-SENS-01, -02 |
| BD-098 | Stills and `inspect-pixel` | BD-097 | A pixel maps to the right link in fixtures |
| BD-099 | `VecEnv`, task specifications, masked reset, snapshots | BD-082 | Reset and snapshot round trips |
| BD-100 | On-device networks and optimizer; gradient parity with FrankenTorch | BD-090, BD-092 | GATE-CORTEX-02 |
| BD-101 | PPO with negative controls | BD-099, BD-100 | GATE-CORTEX-01 on all three tiers |
| BD-102 | In-kernel digests, per-stage digests, snapshots, replay | BD-082, BD-067 | GATE-CHRON-01 |
| BD-103 | Teleport with step and stage localization | BD-102, BD-094 | GATE-CHRON-02 |
| BD-104 | The standard battery, frozen; the fidelity ledger | BD-089, BD-049 | GATE-FID-01, with the one permitted re-baseline written down |
| BD-105 | The noninterference test | BD-098, BD-102 | GATE-SENS-03 |
| BD-106 | Profile, optimize one lever at a time, measure, disclose | BD-101 | GATE-SPARK-03, -04 and GATE-CORTEX-05 recorded as measured, pass or fail |
| BD-107 | The `sim estimate` cost model | BD-106 | Estimates inside their stated interval on held-out runs |
| BD-108 | The M2 verbs | BD-070, BD-101, BD-103 | GATE-TETHER-02 for the M2 verbs |
| BD-109 | M2 exit review by a fresh-context session; kill-gate decisions as ADRs | BD-104, BD-105, BD-106, BD-108 | Decisions recorded for KILL-KIR, KILL-CONTACT, KILL-METAL |

### 36.4 M3 and later (epics)

| Bead | Title | Depends on | Done when |
|---|---|---|---|
| BD-110 | Interior grammars for the seven archetypes; fixtures; movable objects | BD-086 | GATE-HAB-02 |
| BD-111 | Arena task families, scorecards, the design corpus with frozen labels | BD-101, BD-110 | GATE-ARENA-01, -02 |
| BD-112 | Anytime-valid comparison, the promotion rule, exploitation detectors | BD-104, BD-111 | GATE-ATEL-04, GATE-FID-02, GATE-CORTEX-04 |
| BD-113 | Baseline controllers for every template; predictive control | BD-066, BD-082 | GATE-REFLEX-02 |
| BD-114 | Daemon, capabilities, the full MCP surface, `robot-docs`, the skill file | BD-071, BD-112 | GATE-TETHER-01 |
| BD-115 | Validation of static analysis on the corpus | BD-111, BD-061 | GATE-ATEL-01 |
| BD-116 | Optimization, racing, the archive, lineage bisect, predictions | BD-112 | NS-06 demonstrated |
| BD-117 | Equivariant networks; the gait atlas; max-plus timing | BD-055, BD-101 | GATE-CORTEX-03 |
| BD-118 | Tire laws, tracks, hybrids, moving surfaces | BD-088 | Known answers; NS-08 demonstrated |
| BD-119 | The agent-surface A/B | BD-114, BD-116 | GATE-ATEL-03 |
| BD-120 | Certificate coverage; Lean capsules | BD-021 | GATE-LEMMA-01, -02 |
| BD-121 | The articulated variational integrator | BD-082 | Momentum and energy known answers |
| BD-122 | Real-place ingestion | BD-110 | GATE-HAB-03 |
| BD-123 | The Reality lane's first record | BD-104 | A recorded Reality comparison, or a recorded no-claim with its reason |
| BD-124 | The structural margin loop | BD-068 | A planted overstressed link is found |
| BD-125 | Frontier backlog, one short plan per epic | BD-109 | Plans exist; nothing is built without one |

Graph facts for this seed: 125 beads; every dependency points to a lower-numbered bead, so the graph has no
cycles; the longest chain runs BD-001 → 005 → 018 → 019 → 075 → 076 → 079 → 080 → 082 → 090 → 094 → 103 → 108
→ 109 → 125.

## 37. The first implementation session

Before anything else, in this order. Each step produces an observation, not a scaffold.

1. Create the core workspace with FrankenSim's toolchain pin and `franken-stack.lock` (BD-001). Run `cargo check`
   through the offload shim. Record the output.
2. Add one crate with path dependencies on `fs-ga` and `fs-mbd`. Build the built-in G1 model, run forward
   dynamics at one state, and print a digest. Compute the same digest from a test inside FrankenSim at the same
   commit. Record whether they match (BD-006, BD-007). This is the measurement BET-04 stands on.
3. Run `cargo tree -d` and record how many asupersync packages the build contains (BD-008).
4. On this Mac, open `ft_kernel_metal::compute::Gateway`, compile a twenty-line Cayley step in Metal Shading
   Language with `MathMode::Safe`, run it on a million quaternions, and compare with a Rust loop bit for bit
   (BD-016). Then run the exhaustive square-root comparison over all 2³² inputs (the first half of BD-017).
5. On `trj`, compile the same kernel text with the CUDA prelude through `cudarc` and NVRTC, run it on both
   GPUs, and compare with the same Rust loop (BD-015).
6. Write every result, passing or failing, into `DEPENDENCY_ADMISSION.md` with commands and outputs.

The session ends with two honest measurements (does FrankenSim consume cleanly; do the three backends agree on
one algebraic kernel) and a two-second humanoid rollout on the referee. It does not end with a forest of empty
crates.

## 38. Definition of done

The program has delivered when each clause is true and its gate has a recorded value.

- **The spine is real.** Every construction C-01 to C-15 has its type, its verb, its failing-when-wrong test and
  its measured payoff, or has been moved to Appendix C with the reason.
- **The referee is real.** Reference robots simulate on the referee tier with known answers and parity recorded.
- **The batch is real.** One kernel source runs on the CPU, CUDA and Metal; the divergence ledger explains every
  difference; throughput is measured and disclosed, whatever it is.
- **The ladder is real.** A planted exploit is caught by promotion, and no report cites a provisional result as
  established.
- **The places are real.** Seven interior archetypes and the terrain families generate valid, reproducible
  scenes, and one real place has been ingested privately.
- **The studio is real.** A fresh agent with only `robot-docs` designs, checks, simulates, explains, edits and
  compares a robot inside its budget.
- **The honesty is real.** Every result carries its claim type, tier, determinism class and seed; every gate has
  a caught negative control; `reality: no-claim` is printed wherever it is true.
- **The foundations are respected.** No sibling was forked without a recorded kill decision, and every ask has
  a bead in its home tracker.

---
# APPENDICES

## Appendix A. Glossary

| Term | Meaning here |
|---|---|
| Adjoint, coadjoint | The actions of a group element on twists and on wrenches (§9) |
| Airlock | `frob-fsim`, the only crate that names FrankenSim's volatile types |
| Anchored position | An integer cell index plus a single-precision offset (§27.3) |
| Annex | A separate Cargo workspace for GPU or surface code, outside the core's rules (§20.1) |
| Card | Immutable, content-addressed data about a real or generic part, with provenance per field |
| Cayley map | The rational retraction cay(x) = (I − x/2)⁻¹(I + x/2) |
| Divergence ledger | Recorded CPU-versus-device differences per kernel (§20.6) |
| Fidelity ledger | Recorded differences between a faster tier and a slower one (§14.5) |
| Hervé class | One of the twelve conjugacy classes of connected subgroups of SE(3) (C-01) |
| Kernel | A manifest, its arithmetic and its sources in Rust and the shared dialect (§20.2) |
| Promotion | Re-evaluating a fast-tier result on a higher tier before it is called established |
| Referee | The most trusted tier, built on FrankenSim (§14.1). Not ground truth |
| Retraction | A map from the Lie algebra to the group used to advance state |
| Shared dialect | The common subset of CUDA C++ and Metal Shading Language, with per-backend preludes |
| Straight-line program | Branch-free arithmetic with shared subexpressions; the form of derived kernel bodies |
| Teleport | Replaying a GPU world on the CPU from its address to find where tiers part (§23.3) |
| World address | `(run_seed, world, episode)`; determines every random draw a world sees |

## Appendix B. Idea-wizard: 30 ideas → 5 → 15

The pass generated 30 ideas against the brief, thought each through (how it works, how an agent would
experience it, how it is built, how it fails), and winnowed them. The ranking criterion was *obviously accretive
and pragmatic for an agent designing a robot*, not mathematical interest. Every survivor is already designed into
the body; this appendix says where and adds no design.

**The 30.** (1) a trust ladder with promotion and exploitation detectors; (2) a check-before-simulate static
analysis suite; (3) a synthesized baseline controller for every design; (4) single-source derived kernels on
three backends; (5) teleporting a GPU world to the CPU by address; (6) an energy ledger read from the port
structure; (7) anytime-valid design comparison on common random numbers; (8) morphology symmetry detection with
equivariant learning; (9) typed edits with lineage diff, blame and bisect; (10) scene splits enforced
mechanically; (11) actuator cards with provenance and a co-design closure check; (12) registered predictions with
per-agent calibration; (13) a quality-diversity archive of designs; (14) deterministic ray-kernel sensors and
stills with pixel inspection; (15) scorecards normalized against a reference robot per class; (16) a structural
margin loop from telemetry loads; (17) a gait atlas from symmetry with max-plus timing; (18) a maneuverability
index from Lie brackets; (19) a connection-curvature design oracle; (20) import loss reports with
physical-consistency repair; (21) a budget estimate before every expensive verb; (22) one verb registry that
generates the CLI, the MCP tools and the documentation; (23) a failure taxonomy with first-cause ordering;
(24) token-budgeted telemetry summaries; (25) self-contained HTML replays; (26) randomization derived from card
uncertainty; (27) certificates bound to kernel hashes; (28) racing optimizers that cancel losers; (29) a
behaviour-cloning warm start from the baseline controller; (30) a negative-control registry for every gate.

**The best five, best first.**

1. **The trust ladder (§14.5, §19.6; BD-104, BD-112).** Fast-tier results are provisional until a sample is
   re-run on a higher tier, and policies are re-tested under perturbations that should not matter. *Why it wins:*
   the most damaging failure of a learned-control simulator is a result that is an artifact of the simulator. It
   is also the cheapest to guard against, because the referee already exists in FrankenSim and the sample size is
   set by a sequential test. *Confidence:* high. The mechanism is simple, and FrankenSim already states the same
   pattern for its surrogates.
2. **Check before you simulate (§22.2; BD-053 to BD-061).** Sixteen static analyses, seven of them in a
   two-second `design check` with repairs as typed edits. *Why it wins:* it moves the first feedback from hours
   to seconds, it runs on a CPU with exact arithmetic where possible, it ships at M1 before any GPU work, and it
   is where the Lie theory visibly earns its place. *Confidence:* high for sizing, stability, symmetry and
   collision; medium for how well verdicts predict simulated failure, which BET-05 tests.
3. **A baseline controller for every design (§18; BD-062 to BD-066, BD-113).** An estimator and a controller
   synthesized from the model. *Why it wins:* without it every design iteration contains a training run, and the
   loop of NS-06 is impossible. It also supplies demonstrations for warm starts. *Confidence:* medium-high for
   standard layouts, medium for novel ones; BET-06 and KILL-BASELINE handle the miss.
4. **Teleport, on single-source kernels (§20.3, §23.3; BD-018 to BD-020, BD-102, BD-103).** Any GPU world can be
   re-run on the CPU from its address, and the first differing stage is named. *Why it wins:* a batched GPU
   simulator that cannot be debugged cannot be trusted, and counter-based addressing plus a CPU twin make this
   nearly free once designed in. *Confidence:* high for teleport; medium for the speed of derived arithmetic,
   which BET-01 tests and KILL-KIR covers.
5. **The energy ledger (§23.4; BD-046, BD-068).** Every joule attributed to a loss or a store, with the numerical
   residual on its own line. *Why it wins:* energy per metre is the figure of merit for a mobile robot, and
   knowing where the energy went tells an agent what to change. It falls out of the port structure instead of
   being bolted on. *Confidence:* high.

**The next ten.**

6. **Anytime-valid comparison on common random numbers** (§22.4; GB-06; BD-112): an agent can stop as soon as
   the answer is clear, and pairing by world address does more for sample size than the test itself.
7. **Symmetry as a computed property** (C-09; A-03; BD-055, BD-117): the group is found, not declared; it gives
   augmentation, equivariant policies and a metamorphic test of the whole simulator per group element.
8. **Lineage with diff, blame and bisect** (§22.5; BD-116): version control for robots, so "which edit broke
   stairs" has an answer.
9. **Scene splits enforced mechanically** (§16.7; GATE-ARENA-02): a policy cannot be scored on a scene it
   trained on, by construction.
10. **Actuator cards and the closure check** (§13, A-10, A-11; GB-03; BD-043, BD-056): the mass spiral is
    decided before simulation, and no actuator constant is invented.
11. **Registered predictions** (§22.7; BD-116): a calibration record per agent, at almost no cost.
12. **A quality-diversity archive** (§22.6; BD-116): a map of different good designs instead of one optimum.
13. **Deterministic ray-kernel sensing and stills** (§17.3, §17.4; BD-097, BD-098): depth, lidar and height
    scans share the physics determinism class, and a vision-capable agent can look and then ask about a pixel.
14. **Reference-robot scorecards** (§21.2; BD-111): every number has a familiar baseline beside it.
15. **A negative-control registry** (§30.8; GATE-GAUNT-01; BD-072): a gate that nothing can fail is not a gate.

Ideas 16 to 30 are also in the plan, at lower priority or as Frontier items; (19) and (16) are the two most
likely to be cut first.

## Appendix C. EV matrix and the parked or rejected ideas

Score = (Impact × Confidence) ÷ Effort, each from 1 to 5. A family scoring below 2.0 is rejected, parked or
deferred unless something else needs it. Scores are judgments, recorded so they can be argued with.

| Family | Where | Impact | Confidence | Effort | Score | Decision |
|---|---|---|---|---|---|---|
| Inertia on the positive-definite cone | C-03 | 4 | 5 | 1 | 20.0 | Adopted |
| Euler–Poincaré framing and the momentum map | C-04 | 4 | 5 | 1 | 20.0 | Adopted |
| E-processes for comparison | C-15 | 4 | 5 | 1 | 20.0 | Adopted |
| Group-preserving Cayley updates | C-05 | 4 | 4 | 1 | 16.0 | Adopted |
| Lie-typed core on `fs-ga` | C-01, C-02 | 5 | 5 | 2 | 12.5 | Adopted |
| Max-plus gait timing | C-10 | 3 | 4 | 1 | 12.0 | Adopted, Frontier |
| Growth vector and maneuverability | C-07 | 3 | 4 | 1 | 12.0 | Adopted |
| Invariant filtering | C-13 | 4 | 5 | 2 | 10.0 | Adopted |
| Finite symmetry groups | C-09 | 4 | 4 | 2 | 8.0 | Adopted |
| Monotone co-design | C-12 | 4 | 4 | 2 | 8.0 | Adopted |
| Symmetric-cone abstraction | C-06 | 3 | 5 | 2 | 7.5 | Adopted |
| Convex compliant contact with a referee audit | C-06 | 5 | 4 | 3 | 6.7 | Adopted |
| Wiring-diagram operad as a thin data structure | C-12 | 3 | 4 | 2 | 6.0 | Adopted, kept thin |
| Gait atlas from symmetry | C-10 | 3 | 3 | 2 | 4.5 | Adopted, Frontier |
| Equivariant networks | C-09 | 3 | 3 | 3 | 3.0 | Adopted, Frontier, KILL-EQUIV |
| Connection-curvature oracle | C-08 | 3 | 3 | 3 | 3.0 | Adopted, Frontier, kinematic locomotors only |
| Derived kernels with dimension typing | C-14 | 4 | 3 | 4 | 3.0 | Adopted, Frontier, KILL-KIR |
| Instance certificates | C-14, §10 | 3 | 3 | 3 | 3.0 | Adopted, Frontier, KILL-LEMMA |
| Variational articulated integrator | C-05 | 3 | 3 | 4 | 2.3 | Deferred to M5 |
| Pressure-field (hydroelastic) contact | C-06 | 3 | 3 | 4 | 2.3 | Deferred, Frontier |
| Contact-mode enumeration by hyperplane arrangements | — | 2 | 3 | 3 | 2.0 | Parked; useful for planning later |
| Gradients through contact | D-8 | 4 | 2 | 4 | 2.0 | Deferred to M6 |
| Barrier contact for articulated bodies | C-06 | 3 | 3 | 5 | 1.8 | Left to FrankenSim's own lane |
| Operator-theoretic (Koopman) locomotion models | — | 2 | 2 | 3 | 1.3 | Parked |
| Path signatures as telemetry features | — | 2 | 2 | 3 | 1.3 | Parked |
| Study-quadric mechanism synthesis | C-11 | 2 | 2 | 4 | 1.0 | Parked, Moonshot |
| Sheaf-theoretic sensor fusion | — | 2 | 2 | 4 | 1.0 | Parked; the invariant filter covers the need |
| Topological complexity of configuration spaces | — | 1 | 3 | 3 | 1.0 | Rejected: it changes no decision an agent makes |
| General Lean theorems on Lie groups | T-13 | 2 | 2 | 5 | 0.8 | Deferred, Moonshot; the tools are blocked today |
| Persistent homology of configuration spaces | — | 1 | 2 | 4 | 0.5 | Rejected |
| General 6R inverse kinematics by Gröbner bases | C-11 | 2 | 1 | 4 | 0.5 | Rejected at current tool limits |
| A category-theoretic framework beyond five functors | C-12 | 2 | 3 | 4 | 1.5 | Rejected under D-2 |
| A second in-house geometric-algebra layer | — | 2 | 4 | 3 | 2.7 | Rejected as duplication: `fs-ga` already provides it |

## Appendix D. Mathematics compendium

Conventions as in §9. g = (R, p) is a rigid transform; ξ = (ω, v) a twist; w = (τ, f) a wrench.

| Name | Statement | Used in | Assumptions and validity |
|---|---|---|---|
| Bracket on 𝔰𝔢(3) | [ξ₁, ξ₂] = (ω₁ × ω₂, ω₁ × v₂ − ω₂ × v₁) | C-01, C-07 | Angular part first |
| Adjoint | Ad_g(ω, v) = (Rω, Rv + p × Rω) | C-02, C-04 | g maps source frame to target frame |
| Coadjoint | Wrenches transform by the inverse transpose of Ad: (τ, f) ↦ (Rτ + p × Rf, Rf) | C-02 | Same g |
| Power pairing | ⟨w, ξ⟩ = τ·ω + f·v, invariant under change of frame | C-02, §23.4 | Twist and wrench in the same frame |
| Euler–Poincaré, free body | μ̇ = ad*_ξ μ + w. With μ = (π, ℓ): π̇ = π × ω + ℓ × v + τ, ℓ̇ = ℓ × ω + f | C-04 | Body frame, left-invariant kinetic energy |
| Momentum map | μ = 𝕀(s)·(ξ + 𝔸(s)·ṡ) | C-04, C-08 | 𝕀 is the composite rigid-body inertia at the base |
| Reconstruction | ξ = −𝔸(s)·ṡ; net motion over a cycle ≈ −∬ D𝔸 | C-07, C-08 | Principally kinematic systems; leading order in loop size |
| Cayley map | cay(x) = (I − x/2)⁻¹(I + x/2). On SO(3): R = I + 4/(4 + ‖ω‖²)·(ω̂ + ω̂²/2). On SE(3): p = (I − ω̂/2)⁻¹v | C-05 | Second-order agreement with exp |
| Cayley angle | A Cayley step rotates by 2·atan(‖ω‖h/2) | GB-01 | — |
| Planar Cayley | z₊ = z·((1 − a²/4) + i·a)/(1 + a²/4), a = ωh | ADR-MARROW-03 | Unit complex z |
| Pseudo-inertia | P = [[Σ, h], [hᵀ, m]], Σ = ½·tr(Ī)·I − Ī; consistent exactly when P ≻ 0 | C-03 | Ī is rotational inertia about the frame origin |
| Mobility count | M = 6(n − 1 − j) + Σ fᵢ | C-01 | Wrong for over-constrained linkages, which is the point of A-01 |
| Friction cone | 𝒦_μ = {‖γₜ‖ ≤ μγₙ}; its dual is 𝒦_{1/μ} | C-06 | Isotropic; elliptic cones by a linear map |
| Spin-factor Jordan product | (x₀, x̄) ∘ (y₀, ȳ) = (x₀y₀ + x̄·ȳ, x₀ȳ + y₀x̄); eigenvalues x₀ ± ‖x̄‖ | C-06 | Second-order cone is its cone of squares |
| Fast-tier contact problem | v = argmin ½(v − v*)ᵀA(v − v*) + Σ_c ℓ_c(J_c v), with γ_c = −∇ℓ_c | C-06 | ℓ_c convex; A positive definite |
| Growth vector and cost | A direction first reached at bracket depth w costs path length of order ε^(1/w) | C-07 | Regular points of a bracket-generating distribution |
| Reynolds projection | W ↦ \|𝔾\|⁻¹·Σ_g ρ_out(g)·W·ρ_in(g)⁻¹ | C-09 | Finite group, linear representations |
| Isotypic projector | P_χ = (d_χ/\|𝔾\|)·Σ_g χ(g)*·ρ(g) | C-09 | χ irreducible of degree d_χ |
| H/K theorem | For a periodic orbit of an equivariant system, H/K is cyclic, with K the pointwise and H the setwise symmetries | C-10 | Statement used as an enumeration rule |
| Max-plus eigenproblem | A ⊗ x = λ ⊗ x; λ is the maximum cycle mean of A | C-10 | Irreducible A |
| Product of exponentials | g(θ) = exp(θ₁Ŝ₁)…exp(θₙŜₙ)·g₀ | C-11 | Screws in the base frame |
| Kleene iteration | x₀ = ⊥, x_{k+1} = f(x_k) converges to the least fixed point | C-12 | f monotone and continuous on a complete partial order |
| Invariant error | η = X̂⁻¹X; for group-affine dynamics log η obeys a linear equation | C-13 | Noise enters to first order |
| Attitude error | e_R = ½·(R_dᵀR − RᵀR_d)^∨ | C-13 | Almost-global; excludes rotations by π from the target |
| Ville's inequality | For a nonnegative supermartingale with E₀ = 1, P(sup Eₙ ≥ 1/α) ≤ α | C-15 | Holds under optional stopping |
| Motor envelope | i ≤ min(I_max, (V_bus − k_e·N·ω)/R); τ = η·N·k_t·i − τ_friction | §13.3 | Steady-state electrical model at L1 |
| Cost of transport | E/(m·g·d) | §21.2 | Stated separately for mechanical and electrical energy |
| Froude number | v²/(g·L) | §21.2 | L is leg length |
| Pressure-sinkage and shear | p = (k_c/b + k_φ)·zⁿ; τ = (c + σ·tan φ)·(1 − e^(−j/K)) | §15.6 | Semi-empirical; parameters from soil cards |

## Appendix E. Worked cards

All values are illustrative unless tagged FACT. Each card gives the equation, a substitution, the intuition, the
assumptions and what would flip the conclusion.

**GB-01 · How wrong is a Cayley step?**
- **Equation.** A Cayley step rotates by 2·atan(ωh/2) instead of ωh; the relative error is about (ωh)²/12.
- **Substituted.** ω = 20 rad/s, h = 2 ms gives ωh = 0.04 and a relative error of 1.3 × 10⁻⁴, a lag of
  0.0027 rad/s. At the refusal bound ωh = 0.2 the error is 0.33%.
- **Intuition.** The Cayley map is a second-order integrator's worth of error, taken in exchange for rational
  arithmetic.
- **Flip.** A requirement below 10⁻⁵ relative needs ωh < 0.011: substep, or use the exponential.

**GB-02 · What a symmetry group is worth to a policy.**
- **Equation.** Between n and m copies of the regular representation of 𝔾, equivariant linear maps have
  n·m·|𝔾| free parameters against n·m·|𝔾|² unconstrained.
- **Substituted.** A quadruped with C₂ × C₂ (order 4), layers of width 256 (64 copies): 16,384 weights instead
  of 65,536, and every trajectory yields four.
- **Assumptions.** Hidden features carry the regular representation.
- **Flip.** A payload mounted on one side breaks the group to a subgroup, and the dividend shrinks with it.

**GB-03 · Does the mass spiral close?**
- **Equation.** Actuator mass m_a = τ_req/ρ_τ with torque density ρ_τ; τ_req = c·m·g·L; m = m_f + n·m_a. So
  m = m_f/(1 − κ) with loop gain κ = n·c·g·L/ρ_τ, and the design closes only if κ < 1.
- **Substituted.** n = 12, c = 0.25, L = 0.30 m, ρ_τ = 15 N·m/kg gives κ = 0.59 and m = 2.4·m_f: a 6 kg
  structure becomes a 14.6 kg robot. With L = 0.50 m, κ = 0.98 and m = 53·m_f.
- **Intuition.** Kleene iteration on this loop is a geometric series. Leg length is not free.
- **Flip.** κ reaches 1 at L = 0.51 m. Longer legs need a denser actuator or a different load path.

**GB-04 · Why positions are anchored.**
- **Equation.** Single-precision spacing is 2^(e−23) for a value in [2^e, 2^(e+1)).
- **Substituted.** Near 100 m the spacing is 7.6 µm; near 1 km it is 61 µm; inside an 8 m cell it is at most
  0.48 µm. A foot carrying 75 N on a contact stiffness of 10⁵ N/m penetrates 0.75 mm, so 61 µm of position noise
  is 8% force noise.
- **Flip.** Scenes under about 30 m with soft contacts would work without anchoring. Anchoring costs nothing, so
  it is unconditional.

**GB-05 · What it costs a differential drive to move sideways.**
- **Equation.** Rotate by α, drive s, rotate back, reverse: lateral shift d = s·sin α. For small angles the
  wheel travel is about 2d/α + 2αb with half-track b, minimized at α = √(d/b) for a cost of 4·√(d·b).
- **Substituted.** b = 0.20 m. A shift of 0.10 m costs about 0.57 m of wheel travel, 5.7 times the shift. A
  shift of 0.01 m costs about 0.18 m, 18 times the shift.
- **Intuition.** This is the ball-box theorem with growth vector (2, 3): cost scales as the square root of the
  displacement, so small corrections are relatively the most expensive.
- **Flip.** An omnidirectional base has growth vector (3) and pays d for d.

**GB-06 · How many paired episodes does a comparison need?**
- **Equation.** For a standardized paired effect δ, a betting e-process crosses 1/α after about 2·ln(1/α)/δ²
  pairs.
- **Substituted.** α = 0.05: δ = 1 needs about 6 pairs, δ = 0.5 about 24, δ = 0.2 about 150.
- **Intuition.** Clear wins stop early. Pairing by world address raises δ by removing scene and disturbance
  variance, and that matters more than the choice of test.
- **Flip.** Unpaired comparison with a third of the effect size needs nine times the episodes.

**GB-07 · How much memory is a scene?**
- **Equation.** Samples = volume / spacing³; bytes = 2 × samples at 16 bits.
- **Substituted.** 10 × 10 × 3 m at 2 cm: 37.5 million samples, 75 MB. 60 × 40 × 8 m at 4 cm: 300 million
  samples, 600 MB.
- **Flip.** A narrow band around surfaces cuts this by roughly an order of magnitude, at the cost of an indexed
  lookup in the kernel.

## Appendix F. Upstream asks

Each ask is filed as a bead in the sibling's own tracker (BD-096). The interim posture is what FrankenRobotics
does until the ask lands; the switch-over rule is what must be re-run when it does.

| Ask | Repo | Request | Priority | Interim posture | Switch-over rule |
|---|---|---|---|---|---|
| FD-FSIM-01 | frankensim | Spherical, universal, planar and cylindrical joints in `fs-mbd` or the proposed `fs-kinematics` | P1 | The referee lowers them to products of one-axis joints | Re-run GATE-KIN-02, -03 |
| FD-FSIM-02 | frankensim | A persistent contact time stepper (its open Moreau–Jean and barrier lanes) | P1 | The first-order contact step with generated contact sets | Re-baseline the fidelity ledger once |
| FD-FSIM-03 | frankensim | Support maps for capsules and convex hulls; heightfield and distance-field pair gaps | P1 | `frob-contact` implements them; parity only where both apply | Re-run GATE-TRAC-02 |
| FD-FSIM-04 | frankensim | Holonomic loop constraints and nonholonomic rolling in `fs-mbd` | P2 | Compliant equality rows on fast tiers; no exact loops on the referee | Add exact-closure known answers |
| FD-FSIM-05 | frankensim | Adjoints through the articulated and contact steps | P3 | Derivative programs from `frob-kir` on smooth kernels; estimators through contact | Compare gradients |
| FD-FSIM-06 | frankensim | One vector vocabulary across `fs-mbd` and `fs-ga` | P2 | The airlock converts | Delete conversions |
| FD-FSIM-07 | frankensim | Deterministic elementary functions and cancellation checkpoints inside `fs-mbd` | P2 | The referee claims same-platform determinism only | Add cross-ISA lanes to G5 |
| FD-FSIM-08 | frankensim | Freeze the `Component` trait as contract v1 | P1 | The airlock adapts v0 | Re-run functor laws |
| FD-FSIM-09 | frankensim | A home for device receipts (fields AE-01 to AE-12) in its evidence types | P3 | Local types with the doctrine's field names | Replace local types |
| FD-FT-01 | frankentorch | Metal gateway: asynchronous completion with several command buffers in flight; typed buffers beyond `f32` and `u32`; relief from the 31-argument limit; Metal 4 command allocators and ahead-of-time compilation; migration to `objc2-metal` | P1 | Packed argument buffers and synchronous batches | Re-run GATE-SPARK-02, -04 |
| FD-FT-02 | frankentorch | Real device execution for tensors and autograd | P2 | On-device learning kernels in the dialect | Gradient parity, then switch the trainer |
| FD-FT-03 | frankentorch | A CUDA gateway mirroring the Metal gateway | P2 | `spark/frob-spark-cuda` on `cudarc` | Re-run GATE-SPARK-02, -03 |
| FD-FSYM-01 | frankensympy | Multi-output straight-line code generation with CSE; lift the 8,192-operation cap | P2 | `frob-kir` owns it | Regenerate and compare digests |
| FD-FSYM-02 | frankensympy | A Lie-algebra module: structure constants from a matrix basis, ad matrices, BCH coefficients | P3 | Explicit matrices in `frob-derive` | — |
| FD-FSYM-03 | frankensympy | Permutation groups and character tables in Rust | P3 | Small-group code in `frob-algebra` | Cross-check tables |
| FD-FSYM-04 | frankensympy | Certificate families for matrix identities and for sums of squares with rational Gram matrices | P2 | Entry-wise polynomial identities | Re-issue certificates |
| FD-FSYM-05 | frankensympy | Faster Gröbner bases and multivariate resultants | P3 | Inverse kinematics limited to leg-sized systems | — |
| FD-FLN-01 | franken_lean | Unblock the Mathlib roots for matrix exponentials, Lie groups, analytic quaternions and representation theory | P3 | The algebraic route | Promote selected cards from `Certified` to `Proved` |
| FD-FLN-02 | franken_lean | Certify its suite lock on macOS aarch64 | P3 | Lean work on Linux | — |
| FD-ASUP-01 | asupersync and the suite | One published version that FrankenSim, FrankenTorch and fastmcp-rust all pin | P1 | INT-ASUP-01; process boundary if it fails | Merge the surface workspace into the core |

## Appendix G. Reference robots, scene assets and licenses

FACT, from the research pass on 2026-11-21 (URLs in Appendix H). Licenses must be re-checked at admission.

**Robot models (MuJoCo Menagerie).** BSD-3: Unitree Go1, Go2, A1, H1, G1; ANYmal B and C; Spot. MIT: Cassie.
Apache-2.0: Booster T1, Apptronik Apollo, Fourier N1, Barkour, OP3, TALOS, Stretch 3, TIAGo. Already in the
suite with provenance: Unitree G1 (29 DoF) and KUKA iiwa 7 in `fs-mbd::robot_models`.

**Scene assets.**

| Asset | License | Use here |
|---|---|---|
| Procedural generators in `frob-scene` | This project's | Default |
| Infinigen Indoors | BSD-3 | Offline external generator for populated interiors |
| Google Scanned Objects | CC-BY 4.0 | Household clutter |
| RoboCasa | Code MIT, assets CC BY 4.0 | Kitchen fixtures and layouts |
| ProcTHOR | Code Apache-2.0; dataset license not verified | Generator reference only |
| HSSD | CC BY-NC 4.0 | User-supplied import only |
| HM3D | Academic, non-commercial | User-supplied import only |
| PartNet-Mobility | Non-commercial research terms | User-supplied import only |
| BEHAVIOR-1K | Its own agreement | Not used |
| Objaverse-XL | Per-object | Not used by default |

## Appendix H. Source register

Nothing in this plan was compiled, run or measured as FrankenRobotics. Facts come from reading.

**Local sources read on 2026-11-21** (paths under `/Users/jemanuel/projects/` unless noted):

- `frankensim/COMPREHENSIVE_PLAN_FOR_FRANKENSIM.md`, `COMPREHENSIVE_ADDENDUM_TO_FRANKENSIM_PLAN.md`,
  `COMPREHENSIVE_BRIDGE_PLAN_FOR_FRANKENSIM.md`, `COMPREHENSIVE_PLAN_TO_EXTEND_FRANKENSIM_TO_NEW_DOMAINS.md`,
  `COMPREHENSIVE_PLAN_FOR_REAL_TIME_WRIGHT_FLYER_SIM_WITH_FRANKENSIM.md`, `AGENTS.md`,
  `docs/ACCELERATOR_DOCTRINE.md`, `Cargo.toml`, `constellation.lock`, `capability-maturity.json`.
- `frankensim/crates/`: `fs-ga/src/lie.rs`, `fs-mbd/src/articulated.rs`, `fs-mbd/src/robot_models.rs`,
  `fs-mbd/CONTRACT.md`, `fs-contact`, `fs-time`, `fs-query`, `fs-ports/src/component.rs`,
  `fs-cmaes-viz-wasm/src/g1_walking.rs`, `fs-g1-train`.
- `frankensim/apps/instrument-lab/render-gpu/README.md`, `render-gpu-piano/src/{kernel.h,cuda.rs,metal.rs,emulate.rs}`.
- `franken_textiles/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENTEXTILES.md`, `AGENTS.md`, `README.md`.
- `franken_mining/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENMINING.md`, `AGENTS.md`, `franken-stack.lock`.
- `franken_threed/COMPREHENSIVE_PLAN_FOR_THE_DESIGN_OF_FRANKENTHREED.md`, `README.md`,
  `DEPENDENCY_ADMISSION.md`, `Cargo.toml`.
- `franken_lean/README.md`, `IMPLEMENTATION_STATUS.md`, `CERTIFICATE_FORMAT.md`, `CARTRIDGE_FORMAT.md`,
  `SUITE.lock`, the S26 conformance receipt.
- `frankensympy/README.md`, `COMPREHENSIVE_PLAN_FOR_FRANKENSYMPY.md`, `crates/fsym-*`.
- `frankentorch/crates/ft-kernel-metal/src/compute.rs` and `Cargo.toml`, `ft-api`, `ft-dispatch`,
  `docs/planning/FEATURE_PARITY.md`.
- `franken_numpy`, `frankenscipy`, `frankenjax`, `asupersync`, `fastmcp_rust`, `mcp_agent_mail_rust`: README,
  manifests and selected sources.
- `/Users/jemanuel/AGENTS.md`, `/Users/jemanuel/projects/AGENTS.md`.
- This machine: `sysctl` and `system_profiler` report an Apple M5 Ultra, 36 CPU cores, an 80-core GPU with
  Metal 4, and 256 GB of memory.

**Public sources** (fetched 2026-11-21):

- MuJoCo changelog and MJX documentation: https://mujoco.readthedocs.io/en/latest/changelog.html ,
  https://mujoco.readthedocs.io/en/latest/mjx.html
- MuJoCo Warp: https://github.com/google-deepmind/mujoco_warp ,
  https://mujoco.readthedocs.io/en/latest/mjwarp/index.html
- MuJoCo Playground: https://arxiv.org/abs/2502.08844
- Newton: https://github.com/newton-physics/newton ; its solver overview:
  https://newton-physics.github.io/newton/stable/solvers/index.html
- Isaac Lab: https://arxiv.org/abs/2511.04831
- Genesis: https://github.com/Genesis-Embodied-AI/genesis-world ; the throughput re-test:
  https://stoneztao.substack.com/p/the-new-hyped-genesis-simulator-is
- Drake releases and SAP: https://drake.mit.edu/release_notes/release_notes.html ,
  https://arxiv.org/abs/2110.10107
- `cudarc`: https://docs.rs/crate/cudarc/latest ; NVIDIA's Rust kernel tracks:
  https://developer.nvidia.com/blog/introducing-cuda-rust-two-tracks-for-writing-gpu-kernels/
- Metal: https://developer.apple.com/videos/play/wwdc2025/205/ ; rounding and subnormals:
  https://developer.apple.com/forums/thread/844598 ; `objc2-metal`: https://docs.rs/objc2-metal
- Apple M5 family: https://www.apple.com/newsroom/2025/10/apple-unleashes-m5-the-next-big-leap-in-ai-performance-for-apple-silicon/
- Deterministic execution in Warp:
  https://nvidia.github.io/warp/latest/user_guide/execution_and_performance/deterministic_execution.html
- `wgpu` releases: https://github.com/gfx-rs/wgpu/releases ; Slang: https://github.com/shader-slang/slang
- Morphological symmetries: https://arxiv.org/abs/2402.15552 ; gait symmetry:
  https://www.doi.org/10.1038/44416
- Invariant and equivariant filtering: https://arxiv.org/abs/1410.1465 , https://arxiv.org/abs/2010.14666
- Monotone co-design: https://arxiv.org/abs/1512.08055
- Linkage universality: https://arxiv.org/abs/math/9803150
- Gradients through contact: https://arxiv.org/abs/2202.00817 , https://arxiv.org/abs/2109.05143
- Sim-to-real actuator studies: https://arxiv.org/abs/2604.10351 , https://arxiv.org/abs/2503.01255 ,
  https://arxiv.org/abs/2508.02194 , https://arxiv.org/abs/2509.06342
- Assets: https://github.com/princeton-vl/infinigen , https://github.com/robocasa/robocasa ,
  https://huggingface.co/datasets/hssd/hssd-hab , https://aihabitat.org/datasets/hm3d/ ,
  https://github.com/google-deepmind/mujoco_menagerie

**Standard references cited from background knowledge and not re-fetched.** Featherstone, *Rigid Body Dynamics
Algorithms*; Park, Bobrow and Ploen on Lie-group robot dynamics; Murray, Li and Sastry; Lynch and Park; Hervé on
displacement subgroups; Iserles, Munthe-Kaas, Nørsett and Zanna on Lie-group methods; Marsden and West on
variational integrators; Lee, Leok and McClamroch; Wensing, Kim and Slotine on physically consistent inertia;
Todorov on convex contact; Bloch, Krishnaprasad, Marsden and Murray on nonholonomic systems; Hatton and Choset;
Buono and Golubitsky; Lopes et al. on max-plus gait models; Hartley et al. on contact-aided invariant filtering;
Bullo and Lewis; Lee, Liu, Park and Srinivasa on linear-time variational integrators; Kobilarov, Crane and
Desbrun on Cayley-based Lie-group integrators. Each MUST be confirmed against its primary source before a gate
cites it.

**Not verified, and not relied on.** Absolute MuJoCo Warp or Newton throughput on an RTX 4090. The RTX 4090's
double-precision ratio from a primary source and its peer-to-peer support. Whether Metal's single-precision
division and square root are correctly rounded with fast math off (GATE-SPARK-01 measures it). Licenses of
ProcTHOR-10K data, Replica and YCB. Ratification status of glTF physics extensions.

## Appendix I. Type sketches

DESIGN. These fix intent, not signatures.

```rust
/// A Lie group with two retractions. Its laws are G0 property tests (§11.3), not comments.
pub trait LieGroup: Copy {
    type Algebra: LieAlgebra;
    const DIM: usize;
    fn identity() -> Self;
    fn compose(self, rhs: Self) -> Self;
    fn inverse(self) -> Self;
    fn exp(x: Self::Algebra) -> Self;              // deterministic elementary functions
    fn log(self) -> Self::Algebra;
    fn cay(x: Self::Algebra) -> Self;              // +, −, ×, ÷, √ only
    fn cay_inv(self) -> Self::Algebra;
    fn adjoint(self) -> <Self::Algebra as LieAlgebra>::Operator;
}

pub trait LieAlgebra: Copy {
    type Dual: Copy;                               // 𝔤*: momenta, wrenches
    type Operator;                                 // linear maps on 𝔤
    fn bracket(self, rhs: Self) -> Self;
    fn ad(self) -> Self::Operator;
    fn coad(self, mu: Self::Dual) -> Self::Dual;   // ad*_x μ
    fn pair(self, mu: Self::Dual) -> Power;        // ⟨μ, x⟩, in watts
}

/// A joint: a product of one-parameter subgroups of SE(3) given by a screw basis.
pub struct JointSubgroup { class: HerveClass, screws: ScrewBasis, closed_under_bracket: bool }

/// `a_from_b`. Composition type-checks: Transform<A,B> * Transform<B,C> = Transform<A,C>.
pub struct Transform<A: Frame, B: Frame> { motor: fs_ga::Se3, _frames: PhantomData<(A, B)> }
pub struct TwistIn<F: Frame>  { xi: fs_ga::Twist,  _f: PhantomData<F> }
pub struct WrenchIn<F: Frame> { w:  fs_ga::Wrench, _f: PhantomData<F> }

/// A symmetric cone, by its Jordan algebra.
pub trait Cone {
    type Point;
    fn jordan(a: &Self::Point, b: &Self::Point) -> Self::Point;
    fn spectral(a: &Self::Point) -> Spectrum;
    fn project(a: &Self::Point) -> Self::Point;
    fn barrier(a: &Self::Point) -> f64;
}

/// A convex contact potential whose negative gradient is the impulse (C-06).
pub trait ContactLaw { fn potential(&self, vc: ContactVelocity, p: &InterfaceCard) -> ConvexTerm; }

/// Where kernels run. `batch-cpu` implements it with Rust symbols.
pub trait KernelBackend {
    fn compile(&mut self, lib: &KernelLibrary) -> Result<CompiledLibrary, Refusal>;
    fn allocate(&mut self, layout: &BufferLayout) -> Result<BufferSet, Refusal>;
    fn plan(&mut self, stages: &[Stage], shapes: &Shapes) -> Result<StepPlan, Refusal>;
    fn launch(&mut self, plan: &StepPlan, bufs: &mut BufferSet, cx: &Cx<'_>) -> Result<Receipt, Refusal>;
}

/// The vectorized environment. Buffers stay on the device.
pub trait VecEnv {
    fn spec(&self) -> &TaskSpec;
    fn reset(&mut self, mask: &WorldMask, cx: &Cx<'_>) -> Result<StepView<'_>, Refusal>;
    fn step(&mut self, actions: ActionBuf<'_>, cx: &Cx<'_>) -> Result<StepView<'_>, Refusal>;
    fn snapshot(&self, worlds: &[WorldId]) -> Result<Snapshot, Refusal>;
    fn restore(&mut self, snapshot: &Snapshot) -> Result<(), Refusal>;
}

/// What every result carries across the agent boundary.
pub struct Evidence {
    claim: ClaimKind,            // Proof | KnownAnswer | Parity | Fidelity | Reality | Performance
    class: ProofClass,           // Proved | Certified | Tested | Estimated
    tier: Tier, determinism: DeterminismClass,
    model: ModelId, scene: Option<SceneId>, task: Option<TaskId>, seed: Option<u64>,
    status: Standing,            // Provisional { flags } | Established { promoted_on: Tier }
    certificates: Vec<ContentHash>, receipts: Vec<ContentHash>,
}
```

## Appendix J. Verb registry v0

One entry, as it appears in `registries/verbs.toml`:

```toml
[[verb]]
name = "design.check"
cluster = "design"
kind = "read"
capabilities = ["design:read"]
input_schema = "frob.design.check.in/1"
output_schema = "frob.design.check.out/1"
refusals = ["FROB-REF-101", "FROB-REF-104", "FROB-REF-107", "FROB-REF-110"]
idempotent = true
cost_model = "static: under 2 s for template-sized designs"
first_milestone = "M1"
examples = ["frob design check quad-a --json"]
```

| Verb | Kind | First milestone | Notes |
|---|---|---|---|
| `doctor`, `capabilities`, `schema`, `robot-docs` | read | M0 | Operator verbs |
| `spark.devices`, `spark.probe`, `spark.check`, `spark.precompile` | read, run | M0, M2 | Device evidence |
| `design.new`, `design.show`, `design.templates` | mutate, read | M1 | |
| `design.edit` | mutate | M1 | Typed edits only; returns the new `DesignId` and the symmetry effect |
| `design.check` | read | M1 | Aggregates the analyses marked in §22.2 |
| `design.diff`, `design.blame` | read | M1 | |
| `design.bisect` | run | M4 | Uses the anytime-valid comparison at each step |
| `design.import`, `design.export` | mutate, read | M1 | Always returns a loss report |
| `analyze.<name>` | read | M1 to M4 | One per analysis A-01 to A-16 |
| `control.synth`, `control.show` | run, read | M1 | Three outcomes |
| `scene.gen`, `scene.compile`, `scene.import`, `scene.list`, `scene.show` | run, read | M2, M3 | |
| `sim.estimate` | read | M2 | Never runs physics |
| `sim.run` | run | M0 (referee), M2 (all tiers) | Returns provisional evidence on fast tiers |
| `sim.replay` | run | M2 | `--tier` teleports |
| `train.start`, `train.status`, `train.stop`, `train.export` | run, read | M2 | |
| `eval.run`, `eval.scorecard` | run, read | M2, M3 | |
| `eval.compare` | run | M3 | Anytime-valid |
| `eval.promote` | run | M3 | Applies the promotion rule |
| `telemetry.summary`, `telemetry.query`, `telemetry.energy`, `telemetry.failures` | read | M1 | Token-budgeted |
| `telemetry.stills`, `telemetry.inspect-pixel`, `telemetry.export-html` | read | M2 | |
| `explain.run`, `explain.design` | read | M1 | Ends with `next_actions` |
| `opt.start`, `opt.status`, `opt.archive` | run, read | M4 | |
| `predict.register`, `predict.report` | mutate, read | M4 | |
| `prove.kernel`, `prove.verify`, `prove.list` | read | M0, M5 | |
| `report.design`, `report.run` | read | M3 | Evidence packages |
| `job.list`, `job.status`, `job.wait`, `job.cancel` | read, admin | M2 | |
