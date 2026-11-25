# FrankenRobotics

<div align="center">

[![Status](https://img.shields.io/badge/status-building%20M0-blue)](#honest-status)
[![Plan](https://img.shields.io/badge/plan-v1.0%20assembled-blue)](#the-plan-v10)
[![Physics](https://img.shields.io/badge/physics-FrankenSim%20referee-2ea44f)](#foundations)
[![Core](https://img.shields.io/badge/core-Rust%20%C2%B7%20CUDA%20%C2%B7%20Metal-b7410e)](#architecture)
[![Agents](https://img.shields.io/badge/agents-CLI%20%2B%20MCP-8a2be2)](#command-surface-planned)
[![License](https://img.shields.io/badge/license-MIT%20%2B%20AI%20rider-yellow)](LICENSE)

</div>

**FrankenRobotics is a robot proving ground, a robot design studio for AI agents and a mission-query engine, planned
as one mathematical instrument.** An agent writes a robot as a typed design: bodies, joints, actuators, wheels,
tracks, feet, batteries and sensors. It checks the design in seconds without simulating it, gets a baseline
controller synthesized from the model, and runs thousands of worlds at once on CUDA or Metal. It then reads a
structured account of what happened (the first cause of failure, where the energy went, stills of the moment
it slipped), edits the design, and compares the two versions on paired episodes with a test that stays valid
at whatever point it stops. Or it states a mission and asks which designs can do it, which cannot and why, and which
experiment would settle the rest. A result is promoted only after a sample of it re-runs on a CPU referee built on
[FrankenSim](https://github.com/Dicklesworthstone/frankensim).

The robots are legged, wheeled, tracked and hybrid. The worlds are generated terrain (slopes, stairs, gaps,
stepping stones, rubble, soft ground) and generated or scanned interiors: homes, apartments, offices,
warehouses, manufacturing plants, retail stores and commercial kitchens. The same model serves the gym, the
design studio and the mathematics. Configurations are group elements, joints are subgroups, contact is a cone
program in the dual Lie algebra, a design's symmetry is a finite group acting on it, and the GPU kernels are
derived from that algebra instead of being typed in by hand. A construction stays only if it owns a Rust type,
an agent verb, a test that fails when the mathematics is wrong, and a measured payoff or a date to deliver one
(D-2, "No ornamental mathematics").

The first forward-physics question the plans pose is deliberately one where either answer can be right. A
small indoor delivery cart must cross a raised threshold. Should its drive wheels be larger? The motor,
battery, chassis envelope, payload and current limit stay fixed. For an 8 kg cart with 1.2 N·m on each front
shaft, a 60 mm wheel can push 40 N at the tread and a 75 mm wheel only 32 N. Against a 12 mm lip the larger
wheel's lever arm rises from 36.0 to 40.7 mm, and the push needed per unit of load falls from 0.750 to 0.646
(Astra §4.4, analytic). "Either design can lose." Which effect wins depends on weight distribution, motor
lag, compliance and clearance, so it is a question for the simulator and a paired comparison, not for
intuition.

**This project is building its first milestone.** This repository holds the M0 foundation code, [plan v1.0](#the-plan-v10),
the editions it was made from, the multi-model research rounds and reviews behind it, and a reproduction bundle for the
planning experiments. No robot has been simulated yet. Read [Honest status](#honest-status) first.

---

## Honest status

**Status.** Building M0, the bootstrap milestone (§43). Since 2026-11-24 there is a Cargo workspace on one toolchain
with nine crates: `frob-core` (identities, canonical JSON, claims, refusals, units), `frob-cert` (the independent
certificate auditor and its `frob-verify` binary), `frob-conformance` (the end-to-end harness `frob-e2e`, the gate,
pilot and hypothesis registers, and the ten-fold harness `frob-tenfold`), `frob-verbs` (the verb registry),
`frob-kir` and `frob-kernels` (the kernel dialect lint and prelude), `frob-fsim` (the airlock that takes FrankenSim
by path), `frob-lie` (SO(3) and SE(3) over FrankenSim's `fs-ga`, in progress) and `xtask` (six workspace checks:
layers, deps, unsafe, stack, contracts and generated files). There is no `frob` command yet, no robot design type,
no physics step, no GPU kernel and no training run. Work is tracked as 3,658 Beads converted from the plan; a Bead
closes only after a central verifier re-runs its tests at an exact commit, and on 2026-11-25 six had closed. Every
command, file format and throughput number below that is not in this paragraph is a proposal unless it is marked
as a computed planning result. Every throughput figure is a target. The plans and their reviews were written on
2026-11-21 and in the first minutes of 2026-11-22
(EST). The audits found errors in claims that earlier revisions of this README repeated; they are corrected below and
listed in [CHANGELOG.md](CHANGELOG.md). A bare "§" below cites plan v1.0.

| Question | Answer |
|---|---|
| Is there a design? | **Yes.** [Plan v1.0](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md) has 47 chapters and Appendices A to O in 36,990 lines, about 638,800 words. It was assembled at 00:17 EST on 2026-11-22 and replaces v0.2. At 3.9 MB it is too large for GitHub to display; [read it by part](#read-it-by-part) |
| Is there a studio implementation? | **Not yet.** The studio, gym and agent interfaces are proposals. The code so far is the M0 foundation listed above. |
| Can I run anything? | **The workspace's checks and tests, and the planning checks. Nothing simulates a robot yet.** In the workspace, `cargo xtask <check>` runs the six workspace checks, `cargo test` runs the crates' tests and `scripts/e2e/run-all.sh` runs the end-to-end scripts. [`planning/astra-10x-reproduction.zip`](planning/astra-10x-reproduction.zip) re-checks Astra's analytic and pilot results with Python, NumPy, SciPy and Z3. [`planning/verify_plan_math.py`](planning/verify_plan_math.py) re-checks Neon's arithmetic, crate table, bead graph and cross-references. [`planning/fable/blend/tools/`](planning/fable/blend/tools/) rebuilds v1.0 from its parts and checks its identifiers. The Astra archive includes an exploratory planar cart model. See [Planning experiments](#planning-experiments-already-computed) |
| Has anything been measured? | **Planning experiments have run on the target hardware.** Astra §11.6 classified points on a docking boundary on the M5 Ultra and on both RTX 4090s; exact integer arithmetic on the CPU had the lowest end-to-end time on both hosts. Astra also ran normalization arithmetic (§11.2) and a sampled slider with three physics updates (§9.1) on those GPUs. Round-2 evidence packs also timed CPU MuJoCo on the M5 Ultra, under background load, to size the referee. No FrankenRobotics physics step, environment step or training run has been measured, and neither round ran an incumbent training loop |
| Has v1.0 been reviewed? | **Its chapters have; its last insertions have not.** A mathematical review (26 findings) and a coherence review (44) were applied, and three completeness audits checked it against its sources. The 66 subsections those audits inserted, about 3,200 lines, "have had no independent review" (Appendix K.5). Round 2 tested the hypotheses behind it on paper, and two other model families challenged the verdicts. No review has come from outside the program |
| Which plan is canonical? | **v1.0.** It absorbs the [Astra edition](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) as of `47e8e89` and [Neon v2.0](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) as of `1f5703d`, the current text of both, and keeps v0.2's names. v0.2 is kept as [a backup](planning/fable/blend/PLAN_v0.2_BACKUP.md). See [Where v1.0 came from](#where-v10-came-from) |
| Public or private, and which license? | Public, under the suite's MIT license with the OpenAI/Anthropic rider. v1.0 still prints this as an open question whose default is "Private" (OPEN-PROG-1) |
| Anything decided since v1.0 was assembled? | **Yes, and the plan text does not say it yet.** An owner ruling of 2026-11-22, recorded as [Revision 2](planning/fable/blend/REVISION_2.md), says every sibling project is the owner's: a mismatch is fixed in the sibling instead of worked around, one Rust toolchain covers the whole build, and `frob-mcp` is built in the core graph. Where v1.0 says siblings are read-only or puts the MCP server behind a process boundary, Revision 2 governs |
| What happens next? | The rest of M0 (§43). The first runnable result is `sim.run --tier referee` on the built-in G1 model (BD-024), which needs the referee dynamics over FrankenSim (BD-007), the ledger (BD-009) and the `frob` command (BD-022). M0 also holds the CUDA and Metal kernel backends, the incumbent audit and phase ledger (§39) and the paper-scale pilots that fund Frontier and Moonshot work (§0.7). The conversion to Beads (§45) is done |

The plan is written to be falsifiable. Each bet names the measurement that would kill it, and each kill
gate names its fallback before any code exists ([Gates and kill gates](#gates-and-kill-gates)). The table below
is generated from `registries/gates.toml` by `cargo xtask check-generated --write` (GATE-GAUNT-10), so it can only
say what a gate has recorded.

<!-- status:begin -->
| Gate             | Claim             | Milestone | Status     | Measured     | Recorded     |
| ---------------- | ----------------- | --------- | ---------- | ------------ | ------------ |
| GATE-ARENA-04    | known-answer      | M1        | open       | —            | —            |
| GATE-ARENA-05    | known-answer      | M1        | open       | —            | —            |
| GATE-GAUNT-01    | known-answer      | M0        | open       | —            | —            |
| GATE-GAUNT-05    | known-answer      | M0        | open       | —            | —            |
| GATE-GAUNT-08    | known-answer      | M0        | open       | —            | —            |
| GATE-GAUNT-10    | known-answer      | M0        | open       | —            | —            |

Summary: 6 gates (open: 6, pass: 0, fail: 0, killed: 0, blocked: 0, no-claim: 0, superseded: 0)
<!-- status:end -->

---

## TL;DR

**The problem.** Robot simulators are fast and widely used, but the pieces an autonomous design agent needs are
scattered or missing. Round 2 audited what incumbents document, and v1.0 keeps only the gaps that survived (§1.2).
Each is a statement about documentation read at pinned versions, and each is listed beside the incumbent it describes:

- **Replay.** No incumbent documents a bitwise re-run of one GPU world alone on a CPU. Newton's guide says results
  "usually differ ... between the CPU and CUDA backends".
- **Batch independence.** None documents that a world's trajectory does not depend on the batch it ran in.
- **GPU determinism.** Opt-in bitwise run-to-run modes exist (Newton 1.8, Isaac Lab 3.0). MuJoCo Warp's
  documentation answers "Is MJWarp on GPU deterministic? No."
- **Topology inside a running batch.** Changing it is unsupported or experimental in the engines audited.
- **Energy.** MuJoCo's reference says "potential and kinetic energy in constraints is not accounted for".
- **Agents.** MCP access to a simulator exists, but nothing audited documents compute budgets, provenance or typed
  refusals on it.
- **Comparison and transfer.** Published sequential tests compare policies, not robot designs, and transfer to bodies
  far outside a training set is, in its authors' words, "still problematic".

**Withdrawn claims.** Earlier revisions of this README repeated "fossils" from v0.2 that the audit overturned; v1.0
lists sixteen statements it does not make (§1.3). Incumbents do run one kernel source on CPU and CUDA, do offer bitwise
replay on the same GPU, do support mixed topologies in one batch (ManiSkill3, two Newton solvers), and do let CPU MuJoCo
edit a model without losing state. Training a quadruped policy takes about seven minutes on one RTX 4090, not "hours".
Electrical and thermal actuator models, sampling control without training, interior generators for homes, kitchens,
retail and warehouses, and anytime-valid tests for policies are all prior art. Where this README says "replaces" below,
it names what an earlier tool did, not a gap in today's incumbents.

**What already exists.** Each ingredient has been published: Featherstone's spatial algebra and its Lie-group
form, convex compliant contact at GPU scale (MuJoCo Warp, Newton), Drake's SAP on the CPU, Lie-group and
variational integrators, invariant filtering, morphological symmetry, monotone co-design, and anytime-valid
inference. Much of it already exists in Rust in FrankenSim: SE(3) and SO(3) maps and Jacobians (`fs-ga`),
floating-base Featherstone dynamics (`fs-mbd`), a 29-DoF Unitree G1 model, semismooth-Newton
Signorini–Coulomb contact with certified continuous collision (`fs-contact`), and anytime-valid racing. "What was not
found is their composition" (§1.4).

**What FrankenRobotics adds.** The composition: "one design identity and one evidence ledger across static analysis,
simulation tiers, statistics and solid mechanics". That is claim N13, "a hypothesis whose only test is the end-to-end
benchmark against a composed incumbent baseline". Six products share one model, one ledger and one verb registry (§1.5):

1. **A proving ground** (Arena, Cortex): a vectorized, device-resident environment API over batched physics, with many
   different robots in one batch.
2. **A design studio for agents** (Atelier, Tether): CLI and MCP verbs to create, check, simulate, explain, compare
   and optimize robot designs.
3. **A mission-query engine** (Augur): the set of designs that can do a job, as an inner set with witnesses, an outer
   set with exclusions, and the cheapest experiment that would shrink the gap.
4. **An adversary and a counterfactual debugger** (Nemesis, Autopsy): worlds that break a design, reduced to minimal
   counterexamples, and failures explained by intervening on forked replays.
5. **A path to reality and to the workshop** (Ground, Foundry): calibration against imported measurements, and build
   packets that carry machine-readable flags saying they authorize nothing.
6. **A mathematical instrument** (Marrow, Lemma, Spark): the algebra that generates the kernels, supplies the test
   oracles, prunes designs before simulation and, where a checker accepts it, carries proofs.

FrankenSim stays "the referee and the bedrock". FrankenRobotics owns the batched Lie-algebraic rollout engine, the
scenes, the sensors, the learning loop, the mission layer, the adversary, the record and the agent-facing studio.

**The ten-fold thesis.** The unit of success is time to an independently accepted answer, not steps per second (§2.1).
"No single mechanism delivers ten-fold, and no conjunction has been shown to" (§2.4). The candidate is eight levers,
seven of which a composed rival can also host; against that rival the expected ratio on fixed-topology CUDA missions is
near one. Nothing has been measured.

---

## The plan, v1.0

v1.0 was written against an [integration charter](planning/fable/blend/INTEGRATION_CHARTER.md) whose brief is
"maximalist": absorb every mechanism from the three editions, the two research rounds and the idea exercises, at the
tier its evidence supports, with a falsifier and a kill gate. Narrowed mechanisms are written with their surviving
claims, refuted ones are not built in their strong form, and no number is written as a result.

### Read it by part

GitHub does not display the assembled file. The drafts it was assembled from hold the same chapters, and all of them
render except P13a and P16b, which GitHub shows as plain text. (The Astra, Neon and v0.2 files are also shown as plain
text.) `tools/assemble.py` rebuilds the plan from them byte for byte.

| Part | Chapters |
|---|---|
| [P00](planning/fable/blend/parts/P00.md) | §0.1 to §0.8 document control and the evidence register; §1 what exists and what this is; §2 the ten-fold thesis; §3 to §8 principles, bets, goals, scenarios, claims |
| [P01](planning/fable/blend/parts/P01.md) | §9 doctrine; §10 the forty constructions; §11 conventions; §12 Lemma |
| [P02](planning/fable/blend/parts/P02.md) | §13 Marrow; §14 Skeleton; §15 Sinew |
| [P03](planning/fable/blend/parts/P03.md) | §16 Kinesis; §17 Phylum; §18 Traction |
| [P04](planning/fable/blend/parts/P04.md) | §19 Habitat; §20 Sensorium |
| [P05](planning/fable/blend/parts/P05.md) | §21 Spark; §35 numerics and determinism; §36 performance |
| [P06](planning/fable/blend/parts/P06.md) | §22 Reflex; §23 Cortex |
| [P07](planning/fable/blend/parts/P07.md) | §25 Augur |
| [P08](planning/fable/blend/parts/P08.md) | §24 Arena; §26 Nemesis; §27 Autopsy |
| [P09](planning/fable/blend/parts/P09.md) | §28 Atelier; §29 Chronicle |
| [P10](planning/fable/blend/parts/P10.md) | §30 Ground; §31 Foundry |
| [P11](planning/fable/blend/parts/P11.md) | §32 Tether; §37 safety, security, privacy |
| [P12](planning/fable/blend/parts/P12.md) | §33 suite consumption; §34 workspace, crates, formats |
| [P13a](planning/fable/blend/parts2/P13a.md) | §38 Gauntlet; Appendix M known answers; Appendix N counterexamples |
| [P13b](planning/fable/blend/parts2/P13b.md) | §39 the ten-fold test; §40 the gate register |
| [P14](planning/fable/blend/parts2/P14.md) | §41 risks; §42 open questions; §43 milestones; §44 work packages; §46 first session; §47 release rubric |
| [P15](planning/fable/blend/parts2/P15.md) | §45 the beads |
| [P16a](planning/fable/blend/parts2/P16a.md) | Appendices A to E: glossary, idea passes, EV matrix and killed ideas, mathematics, worked cards |
| [P16b](planning/fable/blend/parts2/P16b.md) | Appendices F to J and L: upstream asks, reference robots, sources, type sketches, verbs, worked artifacts |
| [K](planning/fable/blend/APPENDIX_K.md), [O](planning/fable/blend/APPENDIX_O_REFUSALS.md), [§0.6](planning/fable/blend/LEAD_PROVENANCE.md) | Appendix K review log; Appendix O refusal registry; how the plan was produced |

The assembler adds two sections that no part holds: §40.9 (33 gates missing from the register's tables) and §45.12 (66
subsections to place in the bead graph).

**What it fixes.** 43 Rust crates (38 core, 5 annex) with a 576 kLoC cap and a closed list ("a new crate is created only
for a distinct dependency, a distinct device target, or a proof boundary"); five binaries, adding `frob-verify` (an
offline certificate auditor that depends only on the standard library) and `frob-tui`; thirteen versioned artifact
formats (`.fbot`, `.fmodule`, `.fcard`, `.fscene`, `.fclear`, `.ftask`, `.fmission`, `.fenergy`, `.fskill`, `.fmcw`,
`.frun`, `.fbuild`, `.frobcert`); the MCP server on `fastmcp_rust`; seven claims; milestones M0 to M8. Six subsystems
join v0.2's seventeen:

| New subsystem | What it adds |
|---|---|
| **Phylum** | Families of bodies in one batch: per-world model variants, size buckets, mixed topologies [F] |
| **Augur** | Mission queries: the set of designs that can do a job, with witnesses, exclusions and the next experiment [F] |
| **Nemesis** | Worlds that break a design, minimal counterexample worlds, bounds on rare failure rates |
| **Autopsy** | Counterfactual debugging by intervening on forked replays, with a stability figure on every attribution |
| **Ground** | Calibration against imported measurements; reality bands inside a calibrated family only [F] |
| **Foundry** | Structure, fatigue, tolerances and build packets; every export says it authorizes nothing [F] |

**What it counts** (identifiers counted here in the assembled file). 37 bets, 43 goals and 16 non-goals, 24 North-Star
scenarios, 40 constructions, 35 theorem cards, 11 decision cores, 54 risks, 163 open questions, 80 work packages and 442
beads (1,091 dependencies, all pointing to a lower number; one root and one sink). 835 known answers, of which 133 print
no full tolerance and 325 no negative control, and 384 counterexamples. 243 registered verbs by Appendix J's own count;
294 refusal codes in Appendix O. Bets are meant to name a competing arm or a potency control, since a test that could
not have failed does not count. The ten-fold claim is BET-13: a ratio of restricted mean times to an independently
accepted answer, against the strongest composed rival, on a sealed suite, with a partial gate at M3 and the tournament
at M7. BET-14 is the tie rule: if a short robust program over an incumbent engine ties, the plan's words are "say so and
ship the layer".

**How it was reviewed** (Appendix K lists twelve rounds).

| Step | Result |
|---|---|
| Inputs | A [digest of Astra](planning/fable/blend/ASTRA_DIGEST.md), a [digest of Neon](planning/fable/blend/NEON_DIGEST.md) with a 24-item "doubtful or wrong" list, and a [catalog of 112 distinct ideas](planning/fable/blend/IDEA_CATALOG.md) merged from about 225 entries |
| [Mathematical review](planning/fable/blend/REVIEW_MATH.md) | 26 findings: 2 critical, 7 major, 17 minor |
| [Coherence review](planning/fable/blend/REVIEW_COHERENCE.md) | 44 findings, 9 where two binding texts could not both be implemented. Replayed here, the two reviews give 133 exact replacements; 131 applied, and the other two met text an earlier one had changed. 119 still stand word for word; later decisions changed the rest |
| Completeness audits [1](planning/fable/blend/GAPFILL_1.md), [2](planning/fable/blend/GAPFILL_2.md), [3](planning/fable/blend/GAPFILL_3.md) | Against Astra and Neon; against the idea catalog, the duel and the adjudication; and against the two editions' latest revisions. 18, 30 and 18 inserted blocks, all [present](planning/fable/blend/LATE_INSERTIONS.md). The third found 13 changes missing, 21 weaker, 8 covered and 12 not to take, and its 34 corrections all stand |
| Reconciliation | Sixteen binding decisions of the [charter](planning/fable/blend/INTEGRATION_CHARTER.md) (G1 to G16), ten more in the [Phase 2 brief](planning/fable/blend/PHASE2_BRIEF.md) and 25 [fix decisions](planning/fable/blend/FIX_DECISIONS.md), with a line per finding in two logs ([A](planning/fable/blend/FIXLOG_A.md), [B](planning/fable/blend/FIXLOG_B.md)) |
| Verdicts | §0.8 prints the [32-row register](planning/fable/brenner/deliverables/VERDICT_REGISTER.md) of round 2, with the same state in every row |

The third audit corrected three statements that the drafts repeated and that counterexamples refute: a stance
certificate whose force allocation was called affine (no affine law attains its width; one needs 7/3 times as much), a
damping backup said never to increase stored energy (a sampled and held command can, by 1.5 J in a hand case), and an
error bound that was not one when its residual is formed in working precision.

**Still open on 2026-11-22 at 00:17 EST**, found by checks here unless the text says so itself:

- **Revision 2 is not folded in.** It changes §0.7, §1, §3, §32 to §34, §37, §40 to §47 and Appendix F, and the plan
  still prints the older rules there (read-only siblings, separate MCP and annex toolchains, KILL-MCP as a default).
- **No independent review of the insertions**, and none from outside (Appendix K.5).
- **The gate register.** The gates that later subsections define are listed in §40.8 and §40.9 and not yet merged into
  its tables. §40.9 calls its 33 entries additions of the final audit, but ten are gates and one a kill gate of §38, and
  three (GATE-ARENA-13, GATE-TENFOLD-04 and -05) already have entries in shorthand. Two rows of §40.7 sit below §40.8.
- **The beads predate the insertions.** §45.12 lists the 66 subsections to place; §45.10 checks gate coverage against an
  inventory of 421 identifiers, and the regenerated one has 501.
- **Numbers.** §36.14 gives 5 to 7.5 device-hours for the rank-agreement experiment with reused specialists where §2.7,
  §28.10 and §42.2 give about 5. The fix log says KILL-INHERIT's 60% has no corrected denominator.
- **Mobility of the Bennett linkage.** C-01 now leaves it `undecided` in the subgroup lattice and decides it by the rank
  test; counterexample CX-019 and Appendix D still credit the lattice.
- **Sources.** Appendix H cites an earlier revision of the reproduction bundle (241,568 bytes; the committed one has
  275,178).

---

## Where v1.0 came from

Three plans were written in parallel on 2026-11-21 and none cites another. v1.0 replaces the base plan and absorbs the
other two, which remain in the repository as their authors' own texts.

| | [Plan v0.2](planning/fable/blend/PLAN_v0.2_BACKUP.md) (replaced) | [Astra edition](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) | [Neon v2.0](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) |
|---|---|---|---|
| **Organizing idea** | "A robot proving ground and a robot design studio built as one mathematical instrument." Algebra first, numbers last | Inverse mission design: the feasible set of designs, exclusions and the deciding experiment. Success means 10× less time to an independently accepted engineering answer | Changes "the unit of computation, proof, world interaction, and verification": campaign-time quotienting, heterogeneous-morphology GPU execution, stratified certificates, spec-to-silicon packets |
| **Shape** | 3,666 lines. Parts 0–V, §1–§38, Appendices A–J. 17 named subsystems, 28 crates plus `xtask` (361 kLoC cap) | 3,531 lines. 20 sections. Work packages W01–W43, milestones M0–M8 | 1,694 long lines. 12 bounded contexts, 51 crates, 5 binaries. 16 bets, 16 North-Star scenarios, M0–M6 |
| **Delivery seed** | 125 beads, BD-001 to BD-125 (M0–M2 detailed, M3+ as epics); a first-session script (§37) | Dependency graph of 43 work packages; a first session around a 2R arm and a wheel-on-lip fixture (§15.2) | 100 beads to one terminal sink (BD-096), checked acyclic by `verify_plan_math.py` |
| **Evidence it carries** | Study of six sibling projects' plans, seven code audits, an idea-wizard pass (30 → 5 → 15), one internal review round | Analytic results, native pilots on the M5 Ultra and both RTX 4090s, its own audit and duel, and a reproduction bundle with executable physical and numerical checks | Round 1 in [`planning/`](planning/): modes-of-reasoning audit, three-model duel (90+ ideas → 39 proposals → 78 scores), Brenner session (13 killed hypotheses, K-01 to K-13), seven audit passes listed in its Appendix E.2 |

**Where they agree.**

- FrankenSim supplies the mechanics. A CPU reference path is permanent. CUDA on two RTX 4090s and Metal on
  an M5 Ultra are both first-class.
- Agents get one CLI and MCP surface, generated from one registry, with explicit units, seeds, budgets,
  versions and capabilities. Inputs that cannot be honoured are refused by name.
- Different kinds of claim are never averaged together (v1.0: seven claims and a `ClaimProduct` type; Neon: five axes
  and a non-promotable `ClaimProduct`; Astra: separate evidence classes).
- Every bet names a falsifier and every gate a fallback, before any code exists.
- The product ends at simulation, export, replay and analysis. Nothing certifies a physical robot as safe.

**Where they disagree, and what v1.0 does.**

| Question | v1.0 | Astra | Neon v2.0 |
|---|---|---|---|
| Crates and binaries | 43 `frob-*` crates; `frob`, `frobd`, `frob-mcp`, `frob-verify`, `frob-tui` | 8 `fr-robot-*` packages, starting as modules; `frobot` | 51 `frob-*` crates; adds `frob-tui` and `frob-verify` |
| Kernels | One shared C-dialect source for CUDA and Metal; arithmetic derived through FrankenSymPy; an exact-integer class for strict decisions | Native CUDA and Metal plans compiled from the typed description; choose the representation first, the device second | Typed SSA IR (`FMT-KIR-1`) lowered to CPU Rust, Metal, CUDA PTX and `no_std` |
| FrankenSim | Consumed by path behind one airlock crate; then a worker process; then a lift | One pinned closure as physics authority, with owner code re-tested before admission | Consume the `fs-qty`-free leaves; lift `fs-mbd` and `fs-contact` with provenance |
| Firmware and hardware | Export admitted at [M] in M8, as an artifact that authorizes nothing (NONGOAL-04) | Ends at export | `.fbuild` packets with QEMU-checked `no_std` firmware; `hardware_safety_signoff: false` |
| Headline number | A ratio of restricted mean times, pooled over a sealed suite, reported whatever it is | A 10× time ratio whose one-sided 95% lower bound must reach 10 in each of four sealed strata | "Delete 90%–99% of futile simulations"; 512 topologies on one GPU grid (both targets) |

**What round 2 says.** Round 2 ("fable") reviewed v0.2 without reading the other editions. Two independent auditors
checked 76 of its 94 claims (31 verified, 39 partially supported, 2 hypothesis, 2 rejected, 2 not material; the other 18
are unaudited). Its answer to "which mechanisms give a tenfold faster accepted decision?" is that no single one does. The
thesis that survives is a conjunction: decide many designs per agent turn, train few of them, stop early, search for
failures, and measure time to an independently accepted answer against the strongest composed incumbent. An adjudication
at 22:19 revised several of the numbers:

- An infinitely fast environment step speeds training by at most about 7.7× on one incumbent stack (earlier revisions
  here said 7.8×, from an unrounded share) and 1.4× on another.
  An incumbent already measures 2.55 M humanoid steps/s. Engine speed is a parity requirement.
- **The ten-fold question is open and unmeasured.** The register's range of 3 to 10× on quadrupeds is withdrawn as an
  answer. Against per-design training, paper arithmetic gives about 3 to 3.5× on quadruped design cycles and about 15× on
  humanoid ones, only if family policies can replace per-design training when designs are ranked. Nobody has measured
  that rank agreement; v1.0 calls the experiment that would (forty designs; about 45 GPU-hours, or about 5 if another
  experiment's specialists are reused) the decisive early one. v1.0's own table (§2.6) states wider ranges as a
  hypothesis: 3 to 10× and 5 to 20×. Against a composed incumbent that hosts the same portable methods, the expected
  ratio on fixed-topology CUDA missions is near one.
- **The [register](planning/fable/brenner/deliverables/VERDICT_REGISTER.md) has 32 rows**: the 24 hypotheses, three of
  them split in two, and five admitted from the challenges. 16 are narrowed, 4 refuted (recursion-only kernel
  specialization, exact rollout reuse, half precision, living multiphysics in every world), 1 deferred and 11 active.
  The active ones include H-18 (information, not the engine, sets the pace) and H-19, "a thin layer over an incumbent".
- **The challenges are adjudicated.** Codex and Gemini contested 16 of the 24 verdicts, 9 of them both. The
  [adjudication](planning/fable/brenner/deliverables/CHALLENGE_ADJUDICATION.md) ruled on 25 contests and on both
  contests of the main question: 11 upheld, 16 partly upheld, none rejected whole, ten specific claims rejected. All 16
  rows change and four change state. Five challenger hypotheses are admitted untested as H-25 to H-29. The adjudicator
  took no part in the packs or the challenges but comes from the same model family as the packs, which the document
  says.

The idea duel on the same plan gave seven ideas 700 or more from both other models. All seven come from the Claude
slate, and the report says so ("the same family as one of the three duellists" compiled it). The top three are a
design-decision benchmark against incumbents (858), paired estimation across fidelity tiers (853) and ragged cohorts
that step different robots in one launch (831). See [`planning/fable/`](planning/fable/) and
[CHANGELOG.md](CHANGELOG.md).

---

## The design loop

The studio enforces an order (§28.3): "check before you simulate, simulate before you train, train
before you claim." This is a target session from §32.16. None of these commands exist yet, and the
numbers in the comments illustrate the envelope rather than report output.

```text
$ frob design new quad-a --template quadruped-12dof --json            # exit 0, design d:41aa…
$ frob design check quad-a --json                                     # exit 2: knee undersized (A-11)
$ frob design edit quad-a swap-actuator --joints '*.knee' --to qdd-l  # new version d:7c03…, symmetry preserved
$ frob design check quad-a --json                                     # exit 0: closes at 15.1 kg
$ frob control synth quad-a --json                                    # ok: trot from the atlas
$ frob sim estimate quad-a --scene terrain:stairs@0.4 --worlds 2048 --seconds 20 --tier gpu-cuda
                                                                      # 38 GPU-seconds, 90% interval 31 to 47; 3.1 GB
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

- **Check before you simulate.** A design is a term, not a file of numbers. `frob design check` runs eight stages that
  need no batch simulation, from form and mass to statics, sizing, clearance and observation aliases. The target for
  stages 1 to 6 is under 2 s at the 95th percentile. In NS-01 "the mass spiral closes at 14.2 kg, peak knee torque
  demand exceeds the selected actuator's continuous rating by 31%", a front-only depth camera keeps only C₂ of the
  body's C₂ × C₂ symmetry, and three ranked repairs come back as typed edits. Twenty-six analyses in all (A-01 to A-26)
  are planned, each also callable as its own verb.
- **Every design can stand on day one.** `frob control synth` derives an estimator and baseline controller
  from the model itself: gaits from an atlas indexed by leg count and Froude number, capture-point footsteps
  and force distribution over friction cones for legs, and kinematic connections and flat outputs for wheels.
  It reports one of `not-synthesizable`, `synthesized-failed` or `ok`. The baseline exists to give feedback,
  not peak performance. "A design that fails under its baseline is not thereby a bad design." Whether its ranking of
  designs matches that of trained policies is untested; that is what BET-17 is for.
- **Explain, don't just score.** `frob explain` returns a failure taxonomy (`fall`, `stuck`, `slip`,
  `torque-saturation`, `thermal-limit`, `numerical-fault` and others), stills before, at and after the event,
  and an energy ledger in which the integrator and solver residual is its own line. Autopsy goes further: it forks the
  failed episode under one changed cause, with a placebo fork, and reports how often the outcome flips.
- **Compare honestly.** `frob eval compare` pairs episodes by world address, so both designs see the same
  random numbers, and runs a betting e-process per metric. The comparison can stop the moment the evidence is
  decisive without inflating its error rate.
- **Promote, or be labelled.** Fast-tier results start `provisional`. They become `established` only after a
  sample re-runs on a more trusted tier inside the promotion envelope. Otherwise they stay
  `provisional: tier-sensitive`.
- **Say what an edit invalidates.** `frob result revalidate` reports which earlier verdicts survive a change and which
  need a rerun. A hash identifies an artifact; only a recorded implication carries a verdict forward (D-11).

The second session of §32.16 asks a mission instead of scoring one design: `mission synthesize` returns an inner set of
cells, each with a witness, an outer set with exclusions, and unresolved cells; `nemesis search` and `nemesis shrink`
reduce a failure to a two-edit world; `autopsy intervene` finds the cause; and `eval qualify` spends one submission of a
sealed lane that discovery cannot read. `frob-verify` re-checks the resulting bundle offline.

---

## Mathematics that earns its place

The spine is forty constructions (§10). Each one replaces something specific, names its payoff and its honest limits,
and has a test that fails when the mathematics is wrong. Tags are the plan's own: **[S]** solid, **[F]**
frontier (published, not productized by mainstream robot simulators), **[M]** moonshot (off the critical path). The
first fifteen carry over from v0.2, corrected by the mathematical review:

| | Construction | Replaces | Payoff |
|---|---|---|---|
| C-01 | Configuration is a Lie group; joints are its subgroups (twelve Hervé classes) [S/F] | `enum JointKind` | One kernel generator per screw basis; mobility where the Grübler–Kutzbach count is wrong (Sarrus: 1, not 0, by the subgroup lattice; Bennett: 1, not −2, by the rank test, with the lattice `undecided`) |
| C-02 | Frame groupoid and typed spatial algebra [S] | Poses as bags of floats | `a_from_b · b_from_c` has type `a_from_c`. Frame errors become type errors, at zero cost in batched kernels |
| C-03 | Inertia lives on a cone [S/F] | Trusting imported numbers | Physically consistent iff the 4 × 4 pseudo-inertia is positive definite; import validation is an eigenvalue test |
| C-04 | Dynamics is Euler–Poincaré on a tree [S] | An opaque `step()` | RNEA, CRBA and ABA; centroidal momentum as the momentum map; ballistic invariants as exact tests |
| C-05 | Time stepping stays on the group [S/F] | Re-normalizing after every step | The Cayley map needs only +, −, ×, ÷ and √, so it is a candidate for bit-identical CPU, CUDA and Metal results (hypothesis, GATE-SPARK-01). Exact momentum holds for the symmetries a potential keeps |
| C-06 | Contact is a cone program in the dual algebra [S/F] | Softness tuned for throughput | Strictly convex fast tiers with the solver error recorded per step; exact Coulomb complementarity on the referee; one `Cone` abstraction for contact, limits, control and certificates |
| C-07 | Wheels are a distribution; maneuvering is a Lie bracket [S/F] | Kinematic wheel rules | The growth vector decides controllability and the cost of moving sideways |
| C-08 | Locomotion is holonomy [F] | No model of gait displacement | Net displacement per cycle ≈ a curvature integral. For dynamic walking "it is a diagnostic, not a predictor" |
| C-09 | A morphology has two symmetry groups [S/F] | Symmetry rediscovered by hand | The mechanical group and the subgroup that also keeps sensors, payload, task and terrain. Data augmentation, equivariant policies and one metamorphic test per group element, only where they hold |
| C-10 | Gaits are subgroups; timing is max-plus linear [F] | Hand-tuned gait tables | An (H, K) gait atlas, separated by a declared leg-clock group where the body's own symmetry cannot tell trot from pace; the stride period is a max-plus eigenvalue |
| C-11 | Kinematics is algebraic geometry [S/F/M] | Numeric IK only | Closed-form three-joint legs, certified IK by interval Newton, singular loci. General 6R IK is not promised |
| C-12 | A robot is a composition over typed ports [F] | XML trees | Mechanical, shaft, electrical, thermal and signal ports; the mass spiral (heavier robot → bigger motors → heavier robot) as a least fixed point |
| C-13 | Estimation and control live on groups [S/F] | Euclidean filters | Invariant EKF on SE₂₊ₖ(3); control errors as g_d⁻¹·g; centroidal MPC over contact cones |
| C-14 | Kernels are derived, dimension-typed and certified [F] | Hand-written kernels and derivatives in three languages | Straight-line programs with dimension checking and automatic differentiation; identities checked by FrankenSymPy and bound to a content hash. Not claimed as a throughput lever |
| C-15 | Evidence is a test martingale [S] | p-values at a fixed sample size | Stop at any time with the error rate intact (Ville's inequality) |

C-16 to C-40 are new. They cover gaits as hybrid orbits (C-16), missions as quantified sets of designs (C-17), the
assumption lattice (C-18), information with a clock (C-19), observability (C-20), canonical designs and symmetry
quotients of campaigns (C-21, C-22), family policies (C-23, C-24), coupled fidelity tiers and rank agreement (C-25,
C-26), rare failures (C-27), minimal counterexample worlds (C-28), the simulator as a causal model (C-29), requirements
(C-30), safety as an energy budget (C-31), region and clearance certificates (C-32, C-33), linkage synthesis (C-34),
structure (C-35), variational and nonholonomic steps (C-36), the four-dimensional contact cone (C-37), actuators (C-38),
experiments (C-39) and dominance (C-40).

Proofs are typed and the weakest link wins. A result is `Proved`, `Certified`, `Tested` or `Estimated`;
"'Verified' is not a class and the word is not used for results" (§12.2). Lemma does not claim that the simulator is
verified, that floating-point results satisfy identities exactly, that any Mathlib Lie group result has been re-checked,
or that certificates change a design decision (§12.5). Lean is "a second opinion, not a prover we drive" (§12.1).

---

## Three tiers and a trust ladder

| Tier | Precision | Engine | Scale | Role |
|---|---|---|---|---|
| `referee` | `f64` | FrankenSim through the airlock crate: exact Coulomb friction, certified collision | One to a few dozen worlds | The most trusted tier. "The referee is not called ground truth" |
| `batch-cpu` | `f64` or `f32` | The same derived kernels and stage order as the GPUs; a `strict` profile that locates events | Hundreds to thousands | Permanent reference for the GPU tiers |
| `gpu-cuda`, `gpu-metal` | `f32` | Derived kernels from one source | Thousands to tens of thousands per device | Rollouts, training, broad search |

**Teleport.** Every world has an address (seed, world, episode), and every random stream is keyed by it. In NS-02,
world 1,337 of 4,096 produces NaNs on the GPU; `frob sim replay --world 1337 --tier batch-cpu` replays it alone on the
CPU, stops at the first step whose digest differs and names the contact that started it.

**Exploit detectors** (§23.16). A policy is re-evaluated at half the time step, on the next tier up, with parameters
moved within their stated uncertainty, under an alternative contact law, with a tighter solver, with the stiffness and
the control rate changed, mirrored, and against a work-energy audit. The flags are `dt-sensitive`, `tier-sensitive`,
`parameter-sensitive`, `law-sensitive`, `tolerance-sensitive`, `stiffness-sensitive`, `rate-sensitive`,
`mirror-asymmetric`, `leak-sensitive` and `energy-created`.

**Determinism classes** (§35.1). D0 exact (integer, rational, symbolic). D1 bitwise across backends (hypothesis until
GATE-SPARK-01). D2 bitwise per backend, run to run and across worker counts, batch size, slot and co-tenant worlds,
which every kernel must meet. D3 within a recorded tolerance. D4 unspecified, never cited as evidence. Float atomics,
fused multiply-add and fast-math are banned in kernels; sums use fixed-shape pairwise trees; positions are anchored as
an integer cell plus an `f32` offset of at most 4 m (worst case 0.24 µm against 7.6 µm for a raw `f32` at 100 m). Half
precision is not built (refuted: the condition number times half-precision round-off is at or above one in 37–64% of
sampled contact states).

---

## The claims

| Claim | Meaning | What establishes it |
|---|---|---|
| **Proof** | A stated proposition holds in the declared model | A checker accepted it: a Lean kernel, or for an instance FrankenSymPy's proof kernel, an interval or exact-rational certificate, or `frob-verify` on a bundle |
| **Known answer** | Recovers an analytic or synthetic truth | A known-answer test with its derivation on file |
| **Parity** | Reproduces a reference tool or published number | A fixture from a pinned oracle, never regenerated from the candidate |
| **Fidelity** | A faster tier agrees with a slower, more trusted one | The fidelity ledger: disagreement distributions on the standard battery, and agreement of the decisions the tiers lead to |
| **Design comparison** | One design is better than another, or meets a mission, on a stated family of worlds | A declared study on common world addresses; for a qualified result, the blinded lane |
| **Reality** | Simulation agrees with physical measurement | Imported measurements with provenance and predictions frozen before comparison. Everywhere else `no-claim` |
| **Performance** | Speed or scale on named hardware | A recorded run with the full disclosure block; for a ratio against a rival, the protocol of §39 |

They are never averaged into one number (§8). A gate names exactly one claim. A failure is a failed cell, never an
excluded cell. Agreeing with MuJoCo "establishes that two programs agree", not that either matches reality. The
weakest-input rule is a type: `frob-core` owns a `ClaimProduct` whose class is the minimum over its inputs, with no
setter, so "a proved kernel fed an assumed friction coefficient produces an estimate, and the type leaves no other
outcome". `unknown` is a verdict. Every export carries `execution_authorized: false`, `hardware_safety_signoff: false`
and `sim_to_real_certified: false`, and no verb produces any other value.

---

## Architecture

```text
 agent ─► Tether: frob · frob-mcp · frobd · frob-tui   one verb registry, budgets, refusals, JSON envelope
             │
             ▼
 Skeleton ── design term ─► typed edits ─► RobotModel (content-addressed ModelId)
   │  Marrow: Lie groups, frames, cones, finite groups     Sinew: actuator, battery and thermal cards
   ▼
 Atelier ── check (eight stages) ─► Reflex: invariant filter + synthesized baseline controller
   │        Augur: mission ─► inner set · outer set · unresolved cells · next experiment
   ▼
 Phylum ── families of bodies in size buckets
   ▼
 Kinesis ── referee ─────────── FrankenSim through the Airlock (frob-fsim), f64
   │        batch-cpu ──┐
   │        gpu-cuda ───┼─────── Spark: one derived kernel source, three backends
   │        gpu-metal ──┘
   │   Traction: contact, wheels, tracks, terrain      Habitat: scene packs ─► Sensorium: sensors, stills
   ▼
 Cortex: VecEnv, on-device PPO          Arena: tasks, requirements, batteries, blinded qualification
   │                                    Nemesis: worlds that break it ─► minimal counterexamples
   ▼
 Chronicle: ledger, telemetry, energy ledger, replay and forks ─► Autopsy ─► Atelier: explain, compare, promote
 Ground: calibration, reality bands     Foundry: structure, tolerances, build packets
 Lemma: certificates, theorem cards, frob-verify       Gauntlet: conformance tiers, oracles, the ten-fold harness
```

| Subsystem | Crates (§34.2) | What it owns |
|---|---|---|
| Marrow | `frob-lie`, `frob-algebra` | Lie groups and algebras, retractions, frame typing; finite groups and representations, cones and a cone solver, the inertia cone |
| Skeleton | `frob-design`, `frob-model`, `frob-io` | Design terms (`.fbot`, schema `frob.design/1`), modules with port contracts, eighteen archetypes, typed edits, the model compiler, URDF/MJCF import and export with a loss report |
| Sinew | `frob-actuate` | Content-addressed actuator cards; motors, coupled transmissions, batteries, buses and thermal models |
| Kinesis | `frob-dyn` | The three tiers, the ten-stage batched step, integrators and the trust ladder |
| Phylum | `frob-phylum` | Per-world model variants inside padded capacities, size buckets, the batch-independence test |
| Traction | `frob-contact` | Collision, contact laws, the four-dimensional contact cone, wheels, tracks, rigid and soft terrain |
| Habitat | `frob-scene` | Scene packs, terrain and interior generators, clearance complexes, living worlds, the retrofit catalogue |
| Sensorium | `frob-sense` | Sensors as deterministic queries, the clock contract, the observation service, observability |
| Spark | `frob-kir`, `frob-kernels`, `spark/frob-spark-cuda`, `spark/frob-spark-metal`, `derive/frob-derive` | Straight-line program IR, the kernel library, the CUDA and Metal annexes, symbolic derivation |
| Reflex | `frob-control`, `frob-gait`, `frob-shield` | Contact-aided invariant EKF, baseline controller synthesis, gait orbits, skills, energy budgets and runtime shields |
| Cortex | `frob-learn`, `bindings/frob-gymnasium` | `VecEnv` over device-resident buffers, on-device PPO, family policies, inheritance; an optional Gymnasium adapter |
| Arena | `frob-arena`, `frob-spec` | The requirement and mission languages, tasks, fixtures, batteries, the blinded qualification lane |
| Augur | `frob-mission`, `frob-synth` | Mission queries over design families, region certificates, probes, inverse design |
| Nemesis | `frob-nemesis` | Adversarial worlds, minimal counterexample worlds, rare-failure estimation |
| Autopsy | `frob-autopsy` | Interventions on forked replays, attribution with a stability report |
| Atelier | `frob-analyze`, `frob-opt` | Static analyses, comparison, lineage (`diff`, `blame`, `bisect`), the cascade, decision cores, registered predictions |
| Chronicle | `frob-chronicle` | Ledger, telemetry, replay and forks, digests, the energy ledger, the job and memo stores |
| Ground | `frob-ground` | Clock fitting, identifiability, frozen predictions, reality bands |
| Foundry | `frob-foundry`, `frob-lifecycle` | Structure, fatigue, tolerance allocation, build packets |
| Tether | `frob-verbs`, `frob-cli`, `frob-tui`, `surface/frob-mcp` | The verb registry, envelope and schemas; `frob` and `frobd`; the dashboard; the MCP server |
| Lemma | `frob-lemma`, `frob-cert` | Claims, certificate production, theorem cards T-01 to T-35; the bundle reader and `frob-verify`, standard library only |
| Gauntlet | `frob-conformance`, `xtask` | Test tiers, oracle fixtures, known answers, counterexamples, the ten-fold harness; six workspace checks |
| Airlock | `frob-fsim` | The only crate that names FrankenSim's fast-changing types |

`frob-core` (identities, hashing, evidence classes, the claim product, refusals, units glue) sits under all of them.
Crate caps sum to 576 kLoC; reaching 115% of a cap triggers a scope conversation. Eight core crates hold only Frontier
or Moonshot content.

Layers depend only downward. The core crates set `unsafe_code = "deny"` and never depend on a GPU crate; CUDA
and Metal live in annex workspaces under `spark/` with registered unsafe capsules. Forbidden everywhere: tokio
and its ecosystem, a direct Rayon pool, BLAS and LAPACK, C or C++ numerical libraries, and a Python interpreter outside
`bindings/frob-gymnasium` (§34.3). FrankenSim's own dependencies bring `nalgebra` and `rayon` in transitively; the plan
says so: "'Franken-only' describes direct edges."

---

## Command surface (planned)

One registry, `registries/verbs.toml`, generates the CLI, the MCP tools, the schemas and the docs, and a
parity gate keeps them identical. "Agents ask questions and spend budgets." They do not drive solvers.

```bash
frob doctor | capabilities | schema | robot-docs        # operator verbs (M0)
frob session open | show | focus | close                # scopes, budgets, namespaces
frob spark devices | probe | check | precompile         # device evidence; IEEE probe; CPU-vs-device divergence ledger
frob design new | show | templates | edit | check       # typed edits only; check runs the eight preflight stages (M1)
frob design diff | blame | bisect | import | export     # lineage; URDF/MJCF, always with a loss report
frob analyze <A-01..A-26>                               # one verb per analysis
frob control synth | show                               # baseline estimator + controller
frob scene gen | import | compile | list | show | purge # terrain, interiors, private scans
frob sim estimate | run | replay                        # estimate never runs physics; replay --tier teleports
frob run show | list | export | fork                    # bundles and counterfactual branches
frob train start | status | stop | export               # on-device PPO
frob eval run | compare | preanalyze | promote | scorecard | qualify | ledger | rank-agreement
frob study plan | submit | status | cancel | compare    # two-phase studies with a pre-analysis
frob mission new | check | synthesize | refute | show   # design sets with witnesses and exclusions
frob result show | revalidate                           # what an edit invalidates
frob nemesis search | shrink | oracle | bound           # failing worlds, minimal counterexamples, bounds
frob autopsy intervene | attribute | repair | rerun     # interventions with placebo forks
frob telemetry summary | query | energy | failures | divergence | stills | inspect-pixel | export-html
frob explain run | design | compare                     # ends with next_actions
frob opt start | status | archive | cascade             # search and the co-design cascade (M4)
frob predict register | report                          # registered predictions, scored later
frob prove kernel | verify | list | bundle              # certificates, re-checked offline by frob-verify
frob ground ingest | identify | predict | score | band  # calibration (M5)
frob foundry loads | size | check | tolerance | export  # build packets (M5)
frob report design | run | mission | campaign           # evidence packages
frob job list | status | wait | find | cancel           # long work through frobd
```

With `--json`, every verb writes exactly one `frob.envelope/1` envelope: `verb`, `ok`, `exit`, `result`,
`evidence` (claim class, tier, model identity, determinism, reality), `bound` (the immutable identifiers every name
resolved to), `cost`, `budget`, `warnings` and `next_actions`. Long jobs stream NDJSON progress first. Exit codes are
fixed (§32.4):

| Exit | Meaning |
|---|---|
| 0 | Completed, nothing to flag |
| 2 | Completed with findings or flags |
| 3 | Refused input or question (`FROB-REF-1xx` to `5xx`, and `8xx` other than conflicts) |
| 4 | Budget: refused at admission or stopped; nothing partial is published as a result |
| 5 | Capability not implemented, not admitted, or outside the session's scopes |
| 6 | Backend or device unavailable (`FROB-REF-6xx`) |
| 7 | Conflict: idempotency key reuse, stale parent, or lineage conflict |
| 8 | Accepted and still running; the envelope carries a job handle |
| 9 | Indeterminate. "Never retry blindly" |

Exit code 1 "is never emitted on purpose". Mutating and job-submitting verbs require idempotency keys. Capabilities are
scoped, and the discovery scopes give no way to read a sealed lane. Agents supply data, never code: designs, task specs,
controller configurations and policy weights, but no kernels or programs. Appendix J registers 243 verbs and groups them
for agents into task clusters of at most seven tools. The MCP server is built on fastmcp_rust and treats imported text
as data that "is never interpreted as instructions".

Astra's interface has the same shape under different names (`frobot design patch`, `study plan|submit|compare`,
`run replay|fork`, `experiment suggest`) and adds six inverse queries, among them `mission.synthesize`,
`mission.refute` and `result.revalidate`, which v1.0 takes over. Neon's `frob-mcp` plans 46 tools in seven clusters with
`job_poll`, `job_stream` and `job_cancel`.

---

## Hardware and performance targets

Two machines are named in every edition: `trj`, a Linux x86-64 host with two RTX 4090s (24 GB each, no link between
them), and a Mac Studio with an M5 Ultra (36 CPU cores, an 80-core GPU and 256 GB of unified memory), which holds the
world bank and the large scenes. Only seeds, policy weights and receipts cross between them (§1.8). Metal has no double
precision and the RTX 4090's doubles run at about 1/64 rate, so the devices work in single precision. Anchored
positions, group-valued updates, fixed-order sums and double-float accumulators are meant to make that enough, and
BET-08 tests whether it is.

**v0.2's throughput table is withdrawn.** Round 2 found that it mixed world-steps and env-steps (400 k env-steps/s of
training at a decimation of 10 implies 4.0 M world-steps/s, twice its own 2.0 M physics-only target), set a 15-minute
training target 2.2× slower than a published run on the same card, put its humanoid target about five times below a
published figure, and put its CPU target 7 to 11× below published CPU MuJoCo per core. v1.0 states targets as ratios to
an incumbent measured on our own hardware, in stated units, each with a **tolerance** (a parity band the plan's argument
needs) and an **ambition** (a better number nothing depends on). Planning values below come from published figures. None
is a measurement.

| Workload (§36.3, at h = 2 ms, d = 10) | Hardware | Tolerance | Ambition | Incumbent reference |
|---|---|---|---|---|
| Quadruped, physics only | One RTX 4090, 8,192 worlds | ≥ 0.67 of the incumbent; until measured, a floor of 2.0 M world-steps/s | ≥ 1.0; 4.0 M | mjlab on MuJoCo Warp: 4.8 M world-steps/s at d = 4 (RTX 5090) |
| Quadruped, full environment step | Same | ≥ 0.75 of the physics row | ≥ 0.85 | The same benchmark: 347 k env-steps/s against 1.20 M for physics |
| Quadruped, PPO training throughput | Same | ≥ 0.60 of the physics row | ≥ 0.70 | Playground reports 417 k on an A100 with MJX, at d = 5 |
| Quadruped, time to a return threshold (cold) | One RTX 4090 | ≤ 1.25× the incumbent (re-measured on the same card); planning value 513 s | ≤ the incumbent; 410 s | Playground: about 410 s, read by eye from a figure |
| Humanoid, physics only | One RTX 4090, 8,192 worlds | ≥ 0.67; planning value 1.7 M | ≥ 1.0; 2.55 M | MuJoCo Warp nightly: 2.55 M (RTX 6000 Ada) |
| Mixed batch of three body plans | One RTX 4090, 8,192 worlds | ≥ 0.70 of the harmonic mean | ≥ 0.86 | Newton's Kamino solver: 0.86 at 2,048 environments (maximal-coordinate, documented as not deterministic, GPU not stated) |
| Quadruped physics, Metal | M5 Ultra | ≥ 0.25 of one RTX 4090 (BET-09) | ≥ 0.60 | None published on this chip; Genesis is measured beside it |
| Humanoid physics, `batch-cpu`, `f32` | M5 Ultra, 24 threads | ≥ 0.5 of CPU MuJoCo on the same machine | ≥ 1.0 | CPU MuJoCo (`f64`): 550 k world-steps/s on 24 threads, measured here under load |
| `design check` / `control synth` / generate and compile an interior | CPU | < 2 s p95 / < 5 s / < 60 s | `design check` after one edit: 0.2 / 0.5 / 1.0 s | |

The drafts once gave five thresholds for the mixed batch. v1.0 keeps one tolerance, 0.70, and one kill threshold, 0.5,
on two workloads (§40.7). The reference capacities are fixed in §36.3 (32 contacts and 160 rows for the quadruped, 64 and
320 for the humanoid).

A published figure is a fact about its source, to be re-read from the primary source before a gate cites it. Every
performance claim must carry a disclosure block (§36.7): hardware and driver, backend and compiler
versions, world count, time step and substeps, solver iterations and tolerance, contact capacity and mean active
contacts, sensors, whether policy and learning are included, scene, determinism class, warm-up, launch-graph
capture, median and p99 step time, and peak memory. The plan adds the block because a published simulator
headline was "later re-measured two orders of magnitude lower".

---

## Planning experiments already computed

Astra combines exact calculations, numerical simulations and measured performance tests. Its reduced cart model integrates wheel, chassis and winding dynamics with penalty contact. The integrated Franken Robotics gym and design studio are planned.

- **Neon's consistency checker** (passing at `9b00e13`). `python3 -I -B planning/verify_plan_math.py` recomputes Neon's 16
  numerical cards, its 51-crate budget table, its 100-bead graph and crate layering, its 13 families of cross-references,
  its 46 MCP tools and its worked `.fbot`, `.fmodule` and `.ftask` examples with a real recursive-descent parser. It
  reports zero errors on the 1,694-line Neon plan. It checks that the plan is internally consistent, which is not the
  same as checking that the design works, and its file path is hard-coded to the author's machine (change `ROOT` to run
  it elsewhere).
- **Incumbent timings behind round 2** (measured, under background load). Evidence packs H-01, H-11, H-15 and H-19 ran
  CPU MuJoCo 4.0.1 on the M5 Ultra to size the referee and the precision risk: for example 50,612 steps/s on one
  thread for a 27-DoF humanoid, and 410,000 steps/s on 36 threads for a Go2. They are the incumbent's numbers, and they
  feed the referee-coverage arithmetic. v1.0 gives about 0.1 to 2.4% of GPU world-seconds if the referee runs
  at 0.2 to 1.0 times real time per core on 24 cores; the adjudication leaves coverage unmeasured (H-11).

- **A three-tick docking robot** (analytic, §18.10). A robot can move to learn which of two actuator gains,
  1 or 2, it has, before committing to its final approach. With a command limit of 3/4, a fixed probe
  tolerates sensor error below 1/8. Designing the probe and the feedback together raises that threshold to
  5/16. At the boundary, two physically different robots can report the same velocity, 15/16, and need
  incompatible next commands, −1/4 and −3/4. The continuous boundary was checked against an independently
  formulated LP comparator on 561 actuator/error cells: 561 of 561 match (344 admissible, 217 excluded, 9
  exactly on the boundary). Adding a third plausible gain, 33/32, makes exact docking impossible, and all
  34,590 previously feasible labels on each native test grid reverse. An agent must return that changed
  verdict when the tolerance changes.
- **The narrowest quadruped stance** (analytic, §18.7). A 400 N load on four point feet, each able to push
  0 to 160 N, with a half-length of 1/3 m and a centre of mass that can move anywhere in a rectangle of
  half-dimensions 150 and 75 mm, needs a stance half-width of at least 3/14 m. At 0.20 m one foot would need
  165 N. All 11,520 checked cases agree. That boundary assumes adjustable force allocation. Restricting
  the controller to a single affine law of the measured load moments raises the minimum half-width to 1/2 m.
- **Cart over a sill** (reduced numerical model, §13.2). On the 300-row friction sweep, the 75 mm wheel with a sharp sill
  and a rearward payload succeeded at μ = 0.500, timed out at 0.525, and succeeded at 0.550. Refining the step and
  contact model changed those outcomes. The bundle replays both the original grid and a fixed-clock refinement.
- **When a comparison can declare a tie** (analytic, §13.2). Under the registered promotion rule, a 64-pair
  batch cannot show equivalence whatever the outcome. The first sample size that can is 86 pairs.
- **A speed gate that failed** (measured, §19.4). On a pusher family where each design needs its own
  command, the specialized arm ran only 1.345× faster than the composed baseline (ratio of medians, warm) and
  1.088× faster cold. Subtracting the 34.4 µs solve-only median from the 610.1 µs full median leaves 575.7 µs for parsing,
  checking, serialization and other work. Keeping that residual fixed would cap the gain at about 1.46× (an 842.8 µs composed median over the 575.7 µs; both come from a later timing run, so they do not combine with 1.345×). "The inclusive speed gate failed."
- **Native docking-facet screen** (measured, §11.6). The same scalar boundary test ran on the target
  hardware. Exact integer arithmetic on the host was fastest end to end on both machines:

  | Host and path | Warm median, near / spaced grid (µs) |
  |---|---|
  | M5 Ultra CPU, exact integer | 33.750 / 35.333 |
  | M5 Ultra Metal, guarded `f32` | 288.500 / 308.750 (GPU-active 6.875 / 12.708) |
  | `trj` CPU, exact integer | 780.560 / 423.382 |
  | `trj` dual RTX 4090, guarded `f32` | 2,554.780 / 4,392.090 |

  Comparisons are within a host only. The lesson Astra draws is "first choose a representation that preserves
  strict inequalities, then place it according to where the data and consumer already reside."
- **GPU training arithmetic** (§11.2). The M5 Ultra and each RTX 4090 returned zero variance for equal
  groups at 4,095 and 4,097 using raw binary32 second moments. A centered merge returned the exact
  population variance of one on each device. This was a numerical check of the merge operation.
- **Controller stability under simulation** (§9.1). For a fixed 10 ms servo and 64 starting states,
  exact held-input dynamics met the terminal tolerance in all 64 cases. One kick–drift step per tick
  met it in one case, and two half steps met it in 15. The M5 Ultra and both RTX 4090s reproduced
  all 192 terminal state pairs in the same order.
- **Owner code needs admission.** On the pinned FrankenSim revision, executable tests found three PPO update
  errors: wrong clipping for negative advantages, stale critic values and attention carried across episode
  resets. A separate sampler test drew two nominally independent Gaussian actions per call. Their signs
  agreed in about 66.55% of draws across three seeds, while independence predicts 50%. Astra also reproduces
  a planar-drive case that loses translation (§§5.1, 9.3).

The archive contains 178 checksum-verified payloads plus its manifest. Full replay checks the sharp continuous-gain docking tolerance and the distinction between label count and error-correcting bit budget. Its follow-ups include the
controller-dependent stance result, a sensor fault law that passes every pairwise independence check,
and two univariate polynomial cases that the pinned owner cannot certify soundly.

To re-check them, unzip the bundle into an empty directory outside this repository:

```bash
mkdir -p ~/scratch/astra-repro && cd ~/scratch/astra-repro
unzip /path/to/franken_robotics/planning/astra-10x-reproduction.zip
python3 -B verify.py             # standard-library checks
python3 -B verify.py --all       # needs NumPy, SciPy and the z3 executable on PATH
python3 -B verify_negative.py    # deliberate regressions that must each be caught
```

The recorded portable-study environment is CPython 3.15.0, NumPy 2.5.4, SciPy 1.18.1 and Z3 4.15.4 on
arm64 macOS. In that environment both verifiers pass on the bundle at `47e8e89`: `verify.py --all` in about two minutes,
and `verify_negative.py` rejects all 31 planted faults. The replay runs on the CPU and checks the GPU results against
their bundled receipts. The bundle README includes commands to repeat the native and pinned-source checks.
Measurements of complete robot dynamics, sensor pipelines and the integrated design workflow remain open.

---

## Gates and kill gates

The [gate register](planning/fable/blend/parts2/P13b.md) (§40) lists 37 admission gates (`INT-*`), 279 convergence gates
and 112 kill gates, with thirteen gates and seven kill gates of the ten-fold test beside them. Two indexes add gates that
later subsections define: 49 gates and 22 kill gates (§40.8) and 33 more (§40.9). It says that it governs where a chapter
differs. Every gate has a registered negative control that it must catch, and "No self-certification": a PASS is recorded
by a fresh-context session that did not build the capability. Ten that matter most to a reader:

| Kill gate | Trigger | Fallback |
|---|---|---|
| KILL-FSIM | FrankenSim cannot be consumed by path within two weeks of M0 effort, its cold check exceeds the machine envelope, or upstream breakage exceeds 20% of working days in any four weeks | A pinned FrankenSim worker process behind the same traits; only then a lift with provenance |
| KILL-KIR | At M2 derivation cannot emit more than 20% of the kernel arithmetic, or emitted kernels miss BET-01's speed bound after one pass | Hand-written arithmetic in Rust and the shared dialect, bound by differential tests |
| KILL-CONTACT | Fast-tier contact disagrees with the referee on slip or creep after parameter mapping and one repair cycle | CL-2 becomes the default at a published lower throughput; then referee-only locomotion claims |
| KILL-D1 | A device family cannot hold bit equality with `batch-cpu` for the floating kernels of the step | That family drops to D2 for itself and D3 against others; a teleported world is labelled a tolerance replay |
| KILL-METAL | Metal below 25% of one RTX 4090 after one pass, or not deterministic run to run | Development and replay tier; CUDA trains; the Mac trains on `batch-cpu` |
| KILL-PHYLUM | The mixed batch runs below 0.5 of the homogeneous rate on 512 designs, needs more than eight buckets, or padded per-family templates cover at least 95% of the mutation corpus and run faster | Per-family templates become the engine |
| KILL-STATIC | Static checks do not predict which designs fail (area under the ROC curve below 0.8, or below 0.05 above the heuristic) | `design check` becomes advisory lints and the pruning claim is withdrawn |
| KILL-BASELINE | Fewer than 80% of check-passing designs traverse 5 m under their baseline, as a paired 95% lower bound | Synthesis limited to the classes that pass; others route to training |
| KILL-RANK | Cheap and expensive evaluation disagree on the ranking for every morphology class tested | Surviving candidates are all scored on the higher rung; the lost factor is recorded |
| KILL-PARITY | At M2 exit a physics row is below 0.33 of the incumbent on the same card after one profile-guided pass | Feature work on the device tier stops; whether to ship a layer over an incumbent is decided at the partial ten-fold gate |

**The ten-fold test** (§39) compares the time to an independently accepted answer on a sealed suite of missions, as a
ratio of restricted mean times with the strongest composed rival as denominator. Three parties hold separate roles:
builders, rival advocates and an adjudicator. The outcome rules are written in advance (§39.10): a measured ratio below
3 withdraws the speed claim for that class; 3 to 10 is published as measured and "the word 'ten-fold' is not used"; a
tie, where the ablation arm comes within a factor of 1.5 or a layer recovers 0.8 of the saving in time and compute, means
"say so and ship the layer". One false certificate disqualifies an arm. A schedule gate (BET-12) pauses for a written
re-plan if M0 and M1 together take more than 1.5× their estimate because a sibling lacked a primitive.

Neon adds three Brenner exclusion tests (XT-P1, campaign algebra against per-step algebra; XT-P2, heterogeneous
morphologies against specialized templates; XT-P3, formal certificates against real contact) and 21 kill gates, and its
killed-hypothesis list runs to K-13. Astra's sealed test (§19) uses four strata of held-out missions, one-sided 95% lower
bounds of at least 10 in each stratum, and "One false certificate disqualifies that arm from the scoped claim."

---

## Milestones

From §43.1. M1 is "a design studio on the CPU referee tier, with the first mission slice", and GPUs arrive at M2 "on top
of something that already works". The estimates are elapsed weeks for a swarm of agents, exist to make BET-12 testable,
and "are not acceptance evidence". The critical path, M0 to M4 and M7, sums to 33 weeks.

| Milestone | Delivers | Weeks | Scenarios |
|---|---|---|---|
| **M0** spine and airlock | `frob doctor` lists admitted foundations; `frob sim run g1 --tier referee` reproduces FrankenSim's own digest; `frob spark probe` on both device families; one `K-cay-step` source on `batch-cpu`, CUDA and Metal; three certificates that replay offline; M0 verbs over MCP; the incumbent's phase ledger measured on `trj` | 2 | |
| **M1** referee studio and first mission slice, CPU only | Design language and compiler; first archetypes; URDF and MJCF import with loss reports; `design check`; invariant filter and baseline controllers; energy ledger and `explain`; `mission synthesize` and `refute` with bundles that `frob-verify` accepts | 5 | NS-01, NS-07, NS-22, NS-13 on the CPU |
| **M2** batch, backends, families | The batched step on three backends; terrain and scene packs; many designs in one batch; on-device PPO; divergence ledgers; replay and teleport; measured throughput with disclosure | 6 | NS-02, NS-10, NS-14 |
| **M3** places, tasks, adversaries, qualification | Seven interior archetypes; requirement verdicts; counterexample worlds; the blinded lane; the partial ten-fold gate, recorded whatever the ratio | 6 | NS-03, NS-04, NS-09, NS-15 to -17 |
| **M4** amortized control, co-design, mission queries at scale | Family policies and inheritance; the rank-agreement ladder; the cascade; mission queries over batches | 8 | NS-05, NS-06, NS-08, NS-18, NS-19, NS-23, NS-13 at scale |
| **M5** proofs, reality, manufacture (off the critical path) | Certificate coverage, Lean capsules; Ground and its Reality lane, or a recorded `no-claim`; build packets | 6 | NS-11, NS-12, NS-20, NS-21 |
| **M6** frontier physics (off the critical path) | Robot-advected films, slosh, cable and rug proxies; then rods, cloth, coupled islands, soft soil at scale, tactile sensing, the rotor port | per epic | |
| **M7** ten-fold tournament | The sealed suite, every pin frozen, the rival arms, neutral adjudication, publication whatever the ratio | 6 | NS-24 |
| **M8** frontier delivery (off the critical path) | Facility co-design, multi-robot handoff, skill transport, continuation-based synthesis, firmware export | per epic | |

Behind the milestones are 80 work packages ([§44](planning/fable/blend/parts2/P14.md)) and 442 beads
([§45](planning/fable/blend/parts2/P15.md)), tasks for a future implementation phase. They have not been created in the
issue tracker, and §46 describes the first session they would begin with.

---

## Foundations

FrankenRobotics is built on the FrankenSuite. Each foundation has a posture, an admission gate and a written
fallback (§33.4). An admission record counts only if it cites a commit whose tracked `Cargo.lock`
contains the foundation.

| Foundation | Posture | Used for | Fallback |
|---|---|---|---|
| [frankensim](https://github.com/Dicklesworthstone/frankensim) | Consume by path, behind `frob-fsim` | Referee dynamics and contact, geometry, optimizers, racing and e-processes, evidence ledger | KILL-FSIM: a worker process, then a lift |
| asupersync | Consume | Structured concurrency and cancellation; deterministic simulation testing; one package per build | Process boundary |
| frankensqlite | Consume | The Chronicle ledger; the qualification ledger | Append-only JSON-lines ledger |
| frankentorch | Consume | `ft-kernel-metal` gateway in the Metal annex; CPU trainer, gradient oracle, policy interchange | Bind Metal directly |
| `cudarc` (third party) | Annex only | CUDA driver and NVRTC bindings | No CUDA tier |
| franken_numpy, frankenscipy | Narrow consume and oracle | Array files at the export boundary; reference integrators | Plain binary columns; drop the cross-checks |
| frankensympy | Build time and second checker | Kernel derivation; certificates; linkage synthesis as a subprocess | KILL-KIR, KILL-LEMMA |
| franken_lean | Subprocess | Independent re-check and sealing of Lean proofs | Stock Lean, recorded as a single-kernel check |
| fastmcp_rust | Annex only in v1.0's text; in the core graph under Revision 2 | The MCP server | KILL-MCP: the CLI, complete on its own |
| frankentui | Consume [F] | The terminal dashboard | Line-delimited JSON events |
| franken_threed | Pattern; subprocess [F] | Admission-record discipline; an optional viewer route | Plain Three.js |

Seven more serve as patterns, oracles or optional subprocesses: drone geometry reconstruction, frankenjax,
franken-kernel and franken-evidence, franken_textiles, franken_firmware and franken_surveillance_system.

The firewall (§33.7): "M1 depends only on FrankenSim, asupersync and FrankenSQLite. If the GPU, symbolic and proof
foundations all failed admission, M1 would still ship a referee-tier design studio and the first mission slice on the
CPU." Outside engines (MuJoCo in double precision, run out of process) are comparison oracles and the rival arm of the
ten-fold test only. Reference robots come from MuJoCo Menagerie under their own licences, and scene assets are
procedural by default.

A reading of the sources found five reported defects or limits, one convention hazard and six later facts, each with an
admission test and an upstream ask (§33.11, §33.18). Four nightly compilers and one stable release are in play across the
suite (§33.13). `INT-TOOLCHAIN-01` records which of the four build the one crate that uses an unfinished language
feature (`fs-qty`); the escape hatch is a small, stable `frob-units` crate.

Revision 2 (after assembly) replaces the parts of this that assume siblings cannot be changed: a defect becomes an
upstream work item done in the sibling's repository, every one keeping an interim posture so no milestone waits for it,
the whole build uses one toolchain, and the MCP server links the verb implementations in process. KILL-MCP is then a true
fallback.

---

## Scope boundaries

Not a photorealistic renderer (NONGOAL-01). Not a general Python API: the Rust `VecEnv` is primary, and one optional
Gymnasium-compatible adapter exists because the ten-fold test needs rival learning stacks to drive the environment
(NONGOAL-05). No motor actuation, ROS or hardware-in-the-loop claim (NONGOAL-04). No Windows. No full-room fluid, cloth
or granular simulation in every training world, and no soft robots as primary bodies. No half-precision contact, no
universal policy across morphologies, no general formal verification of controllers. Under v1.0's brief, five areas that
v0.2 excluded are admitted in bounded form, at [F] or [M] behind flags and off the critical path: rods and cables,
cloth, films, slosh and coupled islands; a rotor port for flight; firmware export; tactile hands; and the adapter (§6.2).

The anti-goals (§6.3) are firmer: never lower fidelity silently, never wrap MuJoCo, PhysX or another foreign engine as
the production engine, never publish throughput without the disclosure block, never accept an inertia on faith, never
put a language model inside the physics or control loop, and never use the word "verified" for a result.

**No safety certification** (§37.9). "Simulation results are not a safety case for a physical robot." Every exported
artifact says so in machine-readable form, with `execution_authorized`, `hardware_safety_signoff` and
`sim_to_real_certified` fixed at `false`.

## Privacy

A scan of a real place (NS-12, "Your own kitchen") becomes a scene pack marked `private`, stored under a private root
outside the repository. The mark is computed: anything derived from a private scene, including a still of a replay or a
policy trained in it, inherits the mark, and it is not exported without an explicit flag (§37.6). "A private scan stays on
the machine that holds it." Scale is labelled `Estimated` unless it is anchored. `scene.purge` removes a pack and
everything derived from it, with a receipt. Licences are enforced when a pack is built, so research-only assets cannot
reach an exported report or a redistributed pack.

---

## Repository layout

Today:

```text
COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md          plan v1.0 (assembled; GitHub does not display it)
COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md   Astra edition
COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md    Neon v2.0
CHANGELOG.md                                        the planning history, with evidence links
planning/fable/blend/parts/ parts2/                 the parts v1.0 is assembled from, plus APPENDIX_K.md,
                                                    APPENDIX_O_REFUSALS.md and LEAD_PROVENANCE.md beside them
planning/fable/blend/tools/                         scripts that assemble the plan and check it
planning/fable/blend/PLAN_v0.2_BACKUP.md            plan v0.2, the text v1.0 replaced
planning/fable/blend/*.md                           charter, digests, idea catalog, reviews, audits, fix logs
planning/mor/                                       round 1: modes-of-reasoning audit of Neon v1 (case, 16 claims, report)
planning/duel/                                      round 1: three-model idea duel (ideas, cross-scores, reactions, report)
planning/brenner/                                   round 1: question of record, disagreement register, synthesis
planning/verify_plan_math.py                        consistency checker for the Neon edition
planning/fable/mor/ brenner/ duel/                  round 2 on plan v0.2: audited claims, the verdict register, its
                                                    challenges and adjudication, three-model duel
planning/fable/graveyard/ artifact/                 graveyard harvest and decision-core compilation
planning/astra-10x-reproduction.zip                 Astra's planning experiments, receipts and verifiers (reproduction bundle)
AGENTS.md                                           agent contract and swarm rules
.beads/                                             issue tracker (`br`): the plan as 3,658 Beads
Cargo.toml  rust-toolchain.toml  franken-stack.lock core workspace (M0); one toolchain; sibling pins
.cargo/config.toml  DEPENDENCY_ADMISSION.md         build identity; admission evidence per foundation
crates/frob-*/  crates/xtask/                       the nine crates listed under Honest status
registries/  schemas/                               registries as data; their JSON schemas
scripts/e2e/  scripts/lib/  scripts/precommit.sh    end-to-end scripts and harness; shared shell; pre-commit
receipts/  tests/admission/  tools/oracle/          stack receipts; admission records; oracle capture tools
docs/delivery/                                      the delivery log (never in the plan)
```

Target (§34.1; seven workspaces, an eighth that re-targets one crate):

```text
Cargo.toml  rust-toolchain.toml  franken-stack.lock    core workspace; pinned nightly; sibling pins
AGENTS.md  DEPENDENCY_ADMISSION.md                     agent contract; admission evidence per foundation
crates/frob-*/                                         the core workspace (38 crates, unsafe denied)
spark/frob-spark-cuda/  spark/frob-spark-metal/        GPU annexes
surface/frob-mcp/                                      MCP server on fastmcp_rust, separate toolchain
apps/  worker/                                         binaries; FrankenSim's out-of-process fallback
derive/frob-derive/  bindings/frob-gymnasium/          symbolic derivation; optional adapter
kernels/  registries/  schemas/                        derived kernels; registries; generated schemas
lean/  viewer/  demos/  receipts/  assets/             Lean project; viewer; release demos; templates
tests/fixtures/oracle/  tools/oracle/                  pinned oracle captures and their capture scripts
docs/delivery/                                         the delivery log (never in the plan)
```

## Reading paths

- **What and why**: this README, then §1 to §4 ([P00](planning/fable/blend/parts/P00.md)).
- **The mathematics**: §9 to §12 ([P01](planning/fable/blend/parts/P01.md)) and Appendix D
  ([P16a](planning/fable/blend/parts2/P16a.md)); Neon Part II.
- **Mission queries**: §25 ([P07](planning/fable/blend/parts/P07.md)); Astra's front matter.
- **The ten-fold test**: §2 and §39 ([P13b](planning/fable/blend/parts2/P13b.md)); Astra §18 to §20.
- **GPUs and determinism**: §21, §35 and §36 ([P05](planning/fable/blend/parts/P05.md)); Astra §11; Neon §17.
- **The agent surface**: §32 ([P11](planning/fable/blend/parts/P11.md)) and Appendix J
  ([P16b](planning/fable/blend/parts2/P16b.md)); Astra §10.
- **Gates, risks and delivery**: §38 to §47 ([P13a](planning/fable/blend/parts2/P13a.md),
  [P13b](planning/fable/blend/parts2/P13b.md), [P14](planning/fable/blend/parts2/P14.md)); Neon §25 to §29; Astra §13 to §16.
- **How the plan was written and reviewed**: Appendix K ([K](planning/fable/blend/APPENDIX_K.md)), §0.6
  ([provenance](planning/fable/blend/LEAD_PROVENANCE.md)) and [CHANGELOG.md](CHANGELOG.md).

---

## FAQ

**Why not wrap MuJoCo, MuJoCo Warp, Newton or Genesis?** An anti-goal bars a foreign engine as the production engine
(§6.3). Round 2 withdrew several of v0.2's "fossils" and left a shorter list of documented gaps ([TL;DR](#tldr)), and it
is also the program's strongest open rival: H-19 asks whether a thin layer over an incumbent gets most of the value, and
the plan keeps it as a bet with a tie rule (BET-14) and a composed-incumbent arm in the ten-fold test. MuJoCo
runs out of process as a parity oracle. Parity with a simulator is not Reality.

**Why Lie theory and abstract algebra?** Because each construction pays for itself or is cut. Frame errors
become type errors. Group-valued updates stay on the group. The Cayley map's arithmetic is a candidate for
bit-identical results across CPU, CUDA and Metal, and it can carry a certificate. One cone abstraction serves
contact, control and certificates. A design's symmetry group yields augmentation, equivariant policies and
free metamorphic tests where the closed loop has it. Static analyses answer design questions in seconds.

**Why both CUDA and Metal?** The owner's hardware is two RTX 4090s and an M5 Ultra with 256 GB of unified
memory, which can hold scenes no 24 GB card can. BET-09 tests whether Metal is "a real rollout tier, not a
demo"; if not, KILL-METAL demotes it to development and replay.

**Why keep a CPU referee if the GPUs are faster?** "GPUs propose and the referee disposes": "a fast-tier result is
provisional until it is promoted, and any world can be re-run alone on the CPU from its address" (§3). A world that fails
on a GPU must be replayable somewhere it can be inspected.

**Why synthesize a baseline controller?** Training a policy per candidate is the dominant phase of a design loop:
over 90% of a humanoid iteration on published figures (§2.6). A synthesized baseline gives feedback on day one. It is not
meant to be the best controller, and whether its design ranking can be trusted is one of the program's first experiments.

**Why anytime-valid comparisons?** An agent can stop the moment the evidence is decisive, with the error rate
intact. Pairing by world address "does more for sample size than the test itself".

**Why were there three plans?** Three were written in parallel on 2026-11-21 and none cites another. v1.0 was assembled
from them and the research rounds and is the canonical plan (see [Where v1.0 came from](#where-v10-came-from)).
[CHANGELOG.md](CHANGELOG.md) records which research fed which edition.

**Does a good simulated result mean the robot will work?** No. The plan's Reality claim is `no-claim` except inside a
calibrated family with imported measurements, and nothing here is a safety case. Build it and measure it.

---

## License

MIT with the OpenAI/Anthropic rider. See [LICENSE](LICENSE).
