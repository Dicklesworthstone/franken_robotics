# FrankenRobotics

<div align="center">

[![Status](https://img.shields.io/badge/status-planning%20only-blue)](#honest-status)
[![Plans](https://img.shields.io/badge/plans-3%20editions%20%C2%B7%20v1.0%20being%20assembled-blue)](#three-plan-editions)
[![Physics](https://img.shields.io/badge/physics-FrankenSim%20referee-2ea44f)](#foundations)
[![Core](https://img.shields.io/badge/core-Rust%20%C2%B7%20CUDA%20%C2%B7%20Metal-b7410e)](#architecture)
[![Agents](https://img.shields.io/badge/agents-CLI%20%2B%20MCP-8a2be2)](#command-surface-planned)
[![License](https://img.shields.io/badge/license-MIT%20%2B%20AI%20rider-yellow)](LICENSE)

</div>

**FrankenRobotics is a robot proving ground and a robot design studio for AI agents, planned as one
mathematical instrument.** An agent writes a robot as a typed design: bodies, joints, actuators, wheels,
tracks, feet, batteries and sensors. It checks the design in seconds without simulating it, gets a baseline
controller synthesized from the model, and runs thousands of worlds at once on CUDA or Metal. It then reads a
structured account of what happened (the first cause of failure, where the energy went, stills of the moment
it slipped), edits the design, and compares the two versions on paired episodes with a test that stays valid
at whatever point it stops. A result is promoted only after a sample of it re-runs on a CPU referee built on
[FrankenSim](https://github.com/Dicklesworthstone/frankensim).

The robots are legged, wheeled, tracked and hybrid. The worlds are generated terrain (slopes, stairs, gaps,
stepping stones, rubble, soft ground) and generated or scanned interiors: homes, apartments, offices,
warehouses, manufacturing plants, retail stores and commercial kitchens. The same model serves the gym, the
design studio and the mathematics. Configurations are group elements, joints are subgroups, contact is a cone
program in the dual Lie algebra, a design's symmetry is a finite group acting on it, and the GPU kernels are
derived from that algebra instead of being typed in by hand. A construction stays only if it owns a Rust type,
an agent verb, a test that fails when the mathematics is wrong, and a measured payoff or a date to deliver one
(base plan D-2, "No ornamental mathematics").

The first forward-physics question the plans pose is deliberately one where either answer can be right. A
small indoor delivery cart must cross a raised threshold. Should its drive wheels be larger? The motor,
battery, chassis envelope, payload and current limit stay fixed. For an 8 kg cart with 1.2 N·m on each front
shaft, a 60 mm wheel can push 40 N at the tread and a 75 mm wheel only 32 N. Against a 12 mm lip the larger
wheel's lever arm rises from 36.0 to 40.7 mm, and the push needed per unit of load falls from 0.750 to 0.646
(Astra §4.4, analytic). "Either design can lose." Which effect wins depends on weight distribution, motor
lag, compliance and clearance, so it is a question for the simulator and a paired comparison, not for
intuition.

**This project is in planning only.** This repository holds three plan editions, the multi-model research rounds that
reviewed them, the drafts of a hybrid v1.0 that will replace the base plan, and a reproduction bundle for the
planning experiments. Read [Honest status](#honest-status) first.

---

## Honest status

**Status.** Plan stage. There is no Cargo workspace, no crate and no binary. Every command, crate, file format
and throughput number below is a proposal unless it is marked as a computed planning result. Every throughput
figure is a target. Everything here was written on 2026-11-21. Both research rounds have reported. The last one
found errors in claims that earlier revisions of this README repeated; they are corrected below and listed in
[CHANGELOG.md](CHANGELOG.md).

| Question | Answer |
|---|---|
| Is there a design? | **Yes, three, and a fourth in assembly.** The [base plan v0.2](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md) (3,666 lines), the [Astra edition](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) (3,531 lines) and [Neon v2.0](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) (1,694 long lines) were written in parallel and none cites another. A [hybrid v1.0](#the-v10-hybrid-being-assembled) is drafted except for one appendix and not yet assembled |
| Is there a studio implementation? | The studio, gym and agent interfaces are proposals. Current work covers the plans and their supporting research. |
| Can I run anything? | **Only the planning checks.** [`planning/astra-10x-reproduction.zip`](planning/astra-10x-reproduction.zip) re-checks Astra's analytic and pilot results with Python, NumPy, SciPy and Z3. [`planning/verify_plan_math.py`](planning/verify_plan_math.py) re-checks Neon's arithmetic, crate table, bead graph and cross-references. The Astra archive includes an exploratory planar cart model. See [Planning experiments](#planning-experiments-already-computed) |
| Has anything been measured? | **Planning experiments have run on the target hardware.** Astra §11.6 classified points on a docking boundary on the M5 Ultra and on both RTX 4090s; exact integer arithmetic on the CPU had the lowest end-to-end time on both hosts. Astra also ran normalization arithmetic (§11.2) and a sampled slider with three physics updates (§9.1) on those GPUs. Round-2 evidence packs also timed CPU MuJoCo on the M5 Ultra, under background load, to size the referee. No FrankenRobotics physics step, environment step or training run has been measured, and neither round ran an incumbent training loop |
| Have the plans been reviewed? | **Each by a different process.** Base: one internal three-reviewer round (§0.6; its review log, Appendix K, is cited but absent) and round 2 ("fable"), which audited its claims against incumbents, tested 24 hypotheses on paper, had the verdicts challenged by two other model families and then adjudicated the challenges. Neon: round 1 (16 claims: 9 partially supported, 6 hypothesis, 1 rejected, 0 verified; a three-model idea duel; a Brenner session) plus two later three-auditor sweeps. Astra: its own reasoning audit and a 3 × 3 duel, with five of seven audited claims verified in scope (§20.4) |
| Which plan is canonical? | **A v1.0 hybrid is meant to be, and it is not assembled.** An [integration charter](planning/fable/blend/INTEGRATION_CHARTER.md) says v1.0 will replace base v0.2 at its current path and absorb the other two editions. All of it is drafted except Appendix K, and a second reconciliation has applied the two reviews, two completeness audits and the adjudicated verdicts. Until it is assembled, the file at that path is still v0.2, and this README uses base-plan names. The charter keeps them (`frob-` crates; `frob`, `frobd`, `frob-mcp`) and adds `frob-verify` and `frob-tui`. Neon keeps the prefix on its own 51-crate map. Astra proposes `fr-robot-*` packages and a `frobot` CLI |
| Public or private, and which license? | Public, under the suite's MIT license with the OpenAI/Anthropic rider. This settles base OPEN-PROG-1, which blocked the first push |
| What happens next? | Fold the latest Astra and Neon edits into the v1.0 drafts, settle what the fix logs leave open, write Appendix K, assemble v1.0 and convert the plan to beads. Building starts only if the owner changes the scope in [AGENTS.md](AGENTS.md) |

The plans are written to be falsifiable. Each bet names the measurement that would kill it, and each kill
gate names its fallback before any code exists ([Gates and kill gates](#gates-and-kill-gates)). Once the
workspace exists, the base plan generates this status table from `registries/gates.toml` (§30.10), so it can
only say what a gate has recorded.

---

## TL;DR

**The problem.** Robot simulators are fast and widely used, but the pieces an autonomous design agent needs are
scattered or missing. Round 2 audited what incumbents document, and the v1.0 draft keeps only the gaps that
survived (draft §1.2). Each is a statement about documentation read at pinned versions, and each is listed beside
the incumbent it describes:

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

**Withdrawn claims.** Earlier revisions of this README repeated "fossils" from base v0.2 that the audit overturned
(draft §1.3). Incumbents do run one kernel source on CPU and CUDA, do offer bitwise replay on the same GPU, do
support mixed topologies in one batch (ManiSkill3, two Newton solvers), and do let CPU MuJoCo edit a model without
losing state. Training a quadruped policy takes about seven minutes on one RTX 4090, not "hours". Electrical and
thermal actuator models, sampling control without training, interior generators for homes, kitchens, retail and
warehouses, and anytime-valid tests for policies are all prior art. Where this README says "replaces" below, it
names what an earlier tool did, not a gap in today's incumbents.

**What already exists.** Each ingredient has been published: Featherstone's spatial algebra and its Lie-group
form, convex compliant contact at GPU scale (MuJoCo Warp, Newton), Drake's SAP on the CPU, Lie-group and
variational integrators, invariant filtering, morphological symmetry, monotone co-design, and anytime-valid
inference. Much of it already exists in Rust in FrankenSim: SE(3) and SO(3) maps and Jacobians (`fs-ga`),
floating-base Featherstone dynamics (`fs-mbd`), a 29-DoF Unitree G1 model, semismooth-Newton
Signorini–Coulomb contact with certified continuous collision (`fs-contact`), and anytime-valid racing. "What
does not exist is their composition."

**What FrankenRobotics adds.** The composition: one design identity and one evidence ledger across static analysis,
simulation tiers, statistics and solid mechanics. That is the v1.0 draft's claim N13, "a hypothesis whose only test
is the end-to-end benchmark against a composed incumbent baseline". Base v0.2 frames it as three products that share
one model and one ledger (§1.3):

1. **Arena**, a gym: a vectorized, device-resident environment API over batched physics.
2. **Atelier**, a design studio for agents: CLI and MCP verbs to create, check, simulate, explain, compare
   and optimize robot designs.
3. **A mathematical instrument**: the algebra that generates the kernels, supplies the test oracles, prunes
   designs before simulation and, where a checker accepts it, carries proofs.

The v1.0 draft widens that to six (draft §1.5), adding a **mission-query engine** (Augur), an **adversary and
counterfactual debugger** (Nemesis, Autopsy) and a **path to reality and the workshop** (Ground, Foundry). FrankenSim
stays "the referee and the bedrock". FrankenRobotics owns the batched Lie-algebraic rollout engine, the scenes, the
sensors, the learning loop and the agent-facing studio.

**What the other editions add.** Astra turns the question around: give the studio a mission and ask for the
*set* of designs that can do the job, with checkable reasons why the alternatives cannot, and the cheapest
experiment that would decide what remains open. Neon aims to "change the unit of computation" from one robot
to a whole design campaign, removing equivalent designs before any GPU rollout and stepping many different
morphologies on one GPU grid.

---

## Three plan editions

| | [Base plan v0.2](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md) | [Astra edition](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) | [Neon v2.0](COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) |
|---|---|---|---|
| **Organizing idea** | "A robot proving ground and a robot design studio built as one mathematical instrument." Algebra first, numbers last | Inverse mission design: the feasible set of designs, exclusions and the deciding experiment. Success means 10× less time to an independently accepted engineering answer | "Change the unit of computation, proof, world interaction, and verification": campaign-time quotienting, heterogeneous-morphology GPU execution, stratified certificates, spec-to-silicon packets |
| **Shape** | Parts 0–V, §1–§38, Appendices A–J. 17 named subsystems, 28 crates plus `xtask` (361 kLoC cap) | 20 sections. Work packages W01–W43, milestones M0–M8 | 12 bounded contexts, 51 crates, 5 binaries. 16 bets, 16 North-Star scenarios, M0–M6 |
| **Delivery seed** | 125 beads, BD-001 to BD-125 (M0–M2 detailed, M3+ as epics); a first-session script (§37) | Dependency graph of 43 work packages; a first session around a 2R arm and a wheel-on-lip fixture (§15.2) | 100 beads to one terminal sink (BD-096), checked acyclic by `verify_plan_math.py` |
| **Evidence it carries** | Study of six sibling projects' plans, seven code audits, an idea-wizard pass (30 → 5 → 15), one internal review round | Analytic results, native pilots on the M5 Ultra and both RTX 4090s, its own audit and duel, and a reproduction bundle with executable physical and numerical checks | Round 1 in [`planning/`](planning/): modes-of-reasoning audit, three-model duel (90+ ideas → 39 proposals → 78 scores), Brenner session (13 killed hypotheses, K-01 to K-13), seven audit passes listed in its Appendix E.2 |

**Where they agree.**

- FrankenSim supplies the mechanics. A CPU reference path is permanent. CUDA on two RTX 4090s and Metal on
  an M5 Ultra are both first-class.
- Agents get one CLI and MCP surface, generated from one registry, with explicit units, seeds, budgets,
  versions and capabilities. Inputs that cannot be honoured are refused by name.
- Different kinds of claim are never averaged together (base: six claims; Neon: five axes and a
  non-promotable `ClaimProduct`; Astra: separate evidence classes).
- Every bet names a falsifier and every gate a fallback, before any code exists.
- The product ends at simulation, export, replay and analysis. Nothing certifies a physical robot as safe.

**Where they disagree.**

| Question | Base v0.2 | Astra | Neon v2.0 |
|---|---|---|---|
| Crates and binaries | 28 `frob-*` crates plus `xtask`; `frob`, `frobd`, `frob-mcp` | 8 `fr-robot-*` packages, starting as modules; `frobot` | 51 `frob-*` crates; adds `frob-tui` and `frob-verify` |
| Kernels | One shared C-dialect source for CUDA and Metal; arithmetic derived through FrankenSymPy | Native CUDA and Metal plans compiled from the typed description; choose the representation first, the device second | Typed SSA IR (`FMT-KIR-1`) lowered to CPU Rust, Metal, CUDA PTX and `no_std` |
| Metal | Metal 4 | Metal 4, via `ft-kernel-metal` first | Metal 3.2 at M2 |
| FrankenSim | Consumed by path behind one airlock crate | One pinned closure as physics authority, with owner code re-tested before admission | Consume the `fs-qty`-free leaves; lift `fs-mbd` and `fs-contact` with provenance |
| Firmware and hardware | Out of scope (NONGOAL-04) | Ends at export | `.fbuild` packets with QEMU-checked `no_std` firmware; `hardware_safety_signoff: false` |
| Headline number | Per-tier throughput targets | A 10× time ratio whose one-sided 95% lower bound must reach 10 in each of four sealed strata | "Delete 90%–99% of futile simulations"; 512 topologies on one GPU grid (both targets) |

**What round 2 says.** Round 2 ("fable") reviewed base v0.2 without reading the other editions and has finished.
Two independent auditors checked 76 of its 94 claims (31 verified, 39 partially supported, 2 hypothesis, 2 rejected;
the other 18 are unaudited). Its answer to "which mechanisms give a tenfold faster accepted decision?" is that no
single one does. The thesis that survives is a conjunction: decide many designs per agent turn, train few of them,
stop early, search for failures, and measure time to an independently accepted answer against the strongest composed
incumbent. The audited numbers bound every edition's headline. An adjudication at 22:19 has since revised several of
them:

- An infinitely fast environment step speeds training by at most about 7.7× on one incumbent stack (earlier revisions
  here said 7.8×, from an unrounded share) and 1.4× on another.
  An incumbent already measures 2.55 M humanoid steps/s. Engine speed is a parity requirement.
- **The ten-fold question is open and unmeasured.** The register's range of 3 to 10× on quadrupeds is withdrawn as an
  answer. Against per-design training, paper arithmetic gives about 3 to 3.5× on quadruped design cycles and about 15× on
  humanoid ones, only if family policies can replace per-design training when designs are ranked. Nobody has measured
  that rank agreement; the v1.0 draft calls the experiment that would (forty designs; about 45 GPU-hours, or about 5 if
  another experiment's specialists are reused) the decisive early one. Against a composed incumbent that hosts the same
  portable methods, the expected ratio on fixed-topology CUDA missions is near one.
- **The [register](planning/fable/brenner/deliverables/VERDICT_REGISTER.md) now has 32 rows**: the 24 hypotheses, three of
  them split in two, and five admitted from the challenges. 16 are narrowed, 4 refuted (recursion-only kernel
  specialization, exact rollout reuse, half precision, living multiphysics in every world), 1 deferred and 11 active.
  The active ones include H-18 (information, not the engine, sets the pace) and H-19, "a thin layer over an incumbent".
  The v1.0 draft states a rule for what happens if H-19 ties (BET-14).
- **The challenges are adjudicated.** Codex and Gemini contested 16 of the 24 verdicts, 9 of them both. The
  [adjudication](planning/fable/brenner/deliverables/CHALLENGE_ADJUDICATION.md) ruled on 25 contests and on both
  contests of the main question: 11 upheld, 16 partly upheld, none rejected whole, ten specific claims rejected. All 16
  rows change and four change state. Five challenger hypotheses are admitted untested as H-25 to H-29. The adjudicator
  took no part in the packs or the challenges but comes from the same model family as the packs, which the document
  says. The v1.0 drafts now print the revised register (§0.8), and every row there has the same state as the register's.

The idea duel on the same plan gave seven ideas 700 or more from both other models. All seven come from the Claude
slate, and the report says so ("the same family as one of the three duellists" compiled it). The top three are a
design-decision benchmark against incumbents (858), paired estimation across fidelity tiers (853) and ragged cohorts
that step different robots in one launch (831). See [`planning/fable/`](planning/fable/) and
[CHANGELOG.md](CHANGELOG.md).


---

## The v1.0 hybrid, being assembled

The [integration charter](planning/fable/blend/INTEGRATION_CHARTER.md) describes one document that will replace base
v0.2 at its current path. Its brief is "maximalist": absorb every mechanism from the three editions, the two research
rounds and the idea exercises, at the tier its evidence supports, with a falsifier and a kill gate. It "does not mean
credulous": narrowed mechanisms are written with their surviving claims, refuted ones are not built in their strong
form, and no number is written as a result.

| | Status, evening of 2026-11-21 (EST) |
|---|---|
| Inputs compiled | A [digest of Astra](planning/fable/blend/ASTRA_DIGEST.md), a [digest of Neon](planning/fable/blend/NEON_DIGEST.md) with a 24-item "doubtful or wrong" list, and a [catalog of 112 distinct ideas](planning/fable/blend/IDEA_CATALOG.md) merged from about 225 entries |
| Phase 1 drafts | **Complete** at 21:14, revised through 23:22: [13 part files](planning/fable/blend/parts/), about 372,400 words, §0 to §37 of 47 |
| Reconciliation | **Two passes done.** The first (21:20 to 21:24) renumbered colliding refusal codes, recorded ten binding decisions and generated inventories. The second (22:30 to 23:25) inserted [48 subsections](planning/fable/blend/LATE_INSERTIONS.md) and applied both reviews, both audits and the adjudicated verdicts under [25 further decisions](planning/fable/blend/FIX_DECISIONS.md), with a line per finding in two logs ([P00 to P06](planning/fable/blend/FIXLOG_A.md), [P07 to P12 and Phase 2](planning/fable/blend/FIXLOG_B.md)) |
| Phase 2 drafts | **Drafted except Appendix K** (the lead's review log), about 261,200 words. [§38 and Appendices M and N](planning/fable/blend/parts2/P13a.md) (verification; 835 known answers, of which 133 lack a full tolerance and 325 a negative control; 384 counterexamples), [§39 and §40](planning/fable/blend/parts2/P13b.md) (the ten-fold test and the gate register), [§41 to §44, §46 and §47](planning/fable/blend/parts2/P14.md) (risks, open questions, milestones, work packages, first session, release rubric), [§45](planning/fable/blend/parts2/P15.md) (442 beads; a mechanical check here found 1,091 dependencies, all pointing to a lower number, one root and one sink), [Appendices A to E](planning/fable/blend/parts2/P16a.md) (glossary, idea passes, EV matrix with 67 killed ideas, mathematics, 41 worked cards) and [Appendices F to J and L](planning/fable/blend/parts2/P16b.md). The [refusal registry](planning/fable/blend/APPENDIX_O_REFUSALS.md) for Appendix O has not been edited since 21:23. [Phase 2 brief](planning/fable/blend/PHASE2_BRIEF.md) |
| Reviews of the drafts | **Done and applied.** An independent [mathematical review](planning/fable/blend/REVIEW_MATH.md) found 26 problems (2 critical, 7 major, 17 minor). A [coherence review](planning/fable/blend/REVIEW_COHERENCE.md) found 44 (9 where two binding texts cannot both be implemented), among them five different thresholds for one mixed-batch experiment, two closed registries of proof kinds, and different verb names for the same actions. Of the 70 exact replacements the two reviews give, 61 stand word for word; the other nine were changed again by later decisions. A first [completeness audit](planning/fable/blend/GAPFILL_1.md), against Astra and Neon, supplied 18 insertion blocks; a [second](planning/fable/blend/GAPFILL_2.md), against the idea catalog, the duel, the graveyard and artifact passes and the adjudication, checked 233 items (202 covered, 23 weaker, 8 missing) and supplied 30. The appendices list the figures that did not follow from their formulas (C.3, E.7 and M.38) and say which have been corrected; none changes a conclusion |
| Assembly | **Not started.** The file at the base path is still v0.2 |

**What the charter and drafts fix.** 43 Rust crates (38 core, 5 annex) with a 561 kLoC cap and a closed list ("a new crate is
created only for a distinct dependency, a distinct device target, or a proof boundary"); five binaries, adding
`frob-verify` (an offline certificate auditor that depends only on the standard library) and `frob-tui`; thirteen
versioned artifact formats (`.fbot`, `.fmodule`, `.fcard`, `.fscene`, `.fclear`, `.ftask`, `.fmission`, `.fenergy`,
`.fskill`, `.fmcw`, `.frun`, `.fbuild`, `.frobcert`); the MCP server on `fastmcp_rust`; a seventh claim, Design
comparison; and milestones M0 to M8. Six new subsystems join the base plan's seventeen:

| New subsystem | What it adds |
|---|---|
| **Phylum** | Families of bodies in one batch: per-world model variants, size buckets, mixed topologies [F] |
| **Augur** | Mission queries: the set of designs that can do a job, with witnesses, exclusions and the next experiment [F] |
| **Nemesis** | Worlds that break a design, minimal counterexample worlds, bounds on rare failure rates |
| **Autopsy** | Counterfactual debugging by intervening on forked replays, with a stability figure on every attribution |
| **Ground** | Calibration against imported measurements; reality bands inside a calibrated family only [F] |
| **Foundry** | Structure, fatigue, tolerances and build packets; every export says it authorizes nothing [F] |

The draft defines 37 bets (25 new), 43 goals, 24 North-Star scenarios and 40 constructions (C-16 onward are new). Bets
are meant to name a competing arm or a potency control, since a test that could not have failed does not count. The ten-fold
claim is BET-13: a ratio of restricted mean times to an independently accepted answer, against the strongest composed
rival, on a sealed suite, with a partial gate at M3 and the tournament at M7. If a measured ratio is between 3 and 10
the report gives the number and does not use the word "ten-fold". BET-14 is the tie rule: if a short robust program
over an incumbent engine ties, the charter's words are "say so and ship the layer".

**Read the drafts as drafts.** They are corrected against two reviews, but they are unassembled and cite an Appendix K
that does not exist. The mathematical review's two critical findings (how padded sums are signed, and which differential
the variational step uses) are fixed in §16.6 and §16.7. Still open on 2026-11-21 at 23:28 EST:

- **Late plan edits.** The digests predate Neon's rewrites at 20:08 and 21:51 and Astra's at 21:26, 21:43 and 22:07. The
  first audit covered part of them. The rest was extracted at 23:26 ([Neon](planning/fable/blend/NEON_DELTA_AFTER_d18b5a1.diff),
  [Astra](planning/fable/blend/ASTRA_DELTA_AFTER_5afef55.diff)), and no log records folding them in.
- **The gate register.** It indexes the gates of the inserted subsections in §40.8 without merging them into its tables,
  omits GATE-GAUNT-02 to -10 and KILL-DEVORACLE of §38, and has two rows of §40.7 stranded below §40.8.
- **Appendix O** still lists two refusal codes that the second pass retired (FROB-REF-730 and -882).
- **Numbers.** §36.14 gives 5 to 7.5 device-hours for the rank-agreement experiment with reused specialists where §2.7,
  §28.10 and §42.2 give about 5. The fix log adds a kill threshold stated against a denominator nobody has corrected
  (KILL-INHERIT) and a grouping of eighteen archetypes into nine template families that no chapter defines.

---

## The design loop

The studio enforces an order (base §22.1): "check before you simulate, simulate before you train, train
before you claim." This is a target session from base §24.8. None of these commands exist yet, and the
numbers in the comments illustrate the envelope rather than report output.

```text
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

- **Check before you simulate.** A design is a term, not a file of numbers. `frob design check` runs the
  static analyses that need no simulation: mobility, mass roll-up, symmetry, static stability, co-design
  closure, actuator envelope and self-collision. The target is under 2 s at p95. In NS-01 it reports that "the
  mass spiral closes at 14.2 kg, peak knee torque demand exceeds the selected actuator's continuous rating by
  31%, the design has symmetry group C₂ × C₂, and three ranked repairs are offered as typed edits". Sixteen
  analyses in all (A-01 to A-16) are planned, each also callable as its own verb.
- **Every design can stand on day one.** `frob control synth` derives an estimator and baseline controller
  from the model itself: gaits from an atlas indexed by leg count and Froude number, capture-point footsteps
  and force distribution over friction cones for legs, and kinematic connections and flat outputs for wheels.
  It reports one of `not-synthesizable`, `synthesized-failed` or `ok`. The baseline exists to give feedback,
  not peak performance. "A design that fails under its baseline is not thereby a bad design." Whether its ranking of
  designs matches that of trained policies is untested; that is what BET-17 in the v1.0 draft is for.
- **Explain, don't just score.** `frob explain` returns a failure taxonomy (`fall`, `stuck`, `slip`,
  `torque-saturation`, `thermal-limit`, `numerical-fault` and others), stills before, at and after the event,
  and an energy ledger in which the integrator and solver residual is its own line. "An agent can see that,
  say, 38% of the energy went to copper loss in the knees during stance." The ranked edits come back as
  `next_actions`.
- **Compare honestly.** `frob eval compare` pairs episodes by world address, so both designs see the same
  random numbers, and runs a betting e-process per metric. The comparison can stop the moment the evidence is
  decisive without inflating its error rate.
- **Promote, or be labelled.** Fast-tier results start `provisional`. They become `established` only after a
  sample re-runs on a more trusted tier inside the promotion envelope. Otherwise they stay
  `provisional: tier-sensitive`.

---

## Mathematics that earns its place

The base plan's spine is fifteen constructions (§8); the v1.0 draft has forty (C-16 to C-40 are new). Each one
replaces something specific, names its payoff
and has a test that fails when the mathematics is wrong. Tags are the plan's own: **[S]** solid, **[F]**
frontier (published, not productized by mainstream robot simulators).

| | Construction | Replaces | Payoff |
|---|---|---|---|
| C-01 | Configuration is a Lie group; joints are its subgroups (twelve Hervé classes) [S/F] | `enum JointKind` | One kernel generator per screw basis; correct mobility where the Grübler–Kutzbach count is wrong (Sarrus: 1, not 0; Bennett: 1, not −2) |
| C-02 | Frame groupoid and typed spatial algebra [S] | Poses as bags of floats | `a_from_b · b_from_c` has type `a_from_c`. Frame errors become type errors, at zero cost in batched kernels |
| C-03 | Inertia lives on a cone [S/F] | Trusting imported numbers | Physically consistent iff the 4 × 4 pseudo-inertia is positive definite; import validation is an eigenvalue test |
| C-04 | Dynamics is Euler–Poincaré on a tree [S] | An opaque `step()` | RNEA, CRBA and ABA; centroidal momentum as the momentum map; ballistic invariants as exact tests |
| C-05 | Time stepping stays on the group [S/F] | Re-normalizing after every step | The Cayley map needs only +, −, ×, ÷ and √, so it is a candidate for bit-identical CPU, CUDA and Metal results (hypothesis, GATE-SPARK-01) |
| C-06 | Contact is a cone program in the dual algebra [S/F] | Softness tuned for throughput | Strictly convex fast tiers with the solver error recorded per step; exact Coulomb complementarity on the referee; one `Cone` abstraction for contact, limits, control and certificates |
| C-07 | Wheels are a distribution; maneuvering is a Lie bracket [S/F] | Kinematic wheel rules | The growth vector decides controllability and the cost of moving sideways |
| C-08 | Locomotion is holonomy [F] | No model of gait displacement | Net displacement per cycle ≈ a curvature integral. Exact for kinematic systems; "for dynamic walking it is a diagnostic, not a predictor" |
| C-09 | A morphology has a symmetry group [S/F] | Symmetry rediscovered by hand | C₂ for a biped, C₂ × C₂ for a quadruped, dihedral for a hexapod. Data augmentation, equivariant policies, block-diagonal linearizations and one metamorphic test per group element |
| C-10 | Gaits are subgroups; timing is max-plus linear [F] | Hand-tuned gait tables | An (H, K) gait atlas; the stride period is a max-plus eigenvalue |
| C-11 | Kinematics is algebraic geometry [S/F] | Numeric IK only | Closed-form three-joint legs, certified IK by interval Newton, singular loci. General 6R IK is not promised |
| C-12 | A robot is a composition over typed ports [F] | XML trees | Mechanical, shaft, electrical, thermal and signal ports; the mass spiral (heavier robot → bigger motors → heavier robot) as a least fixed point |
| C-13 | Estimation and control live on groups [S/F] | Euclidean filters | Invariant EKF on SE₂₊ₖ(3); control errors as g_d⁻¹·g; centroidal MPC over contact cones |
| C-14 | Kernels are derived, dimension-typed and certified [F] | Hand-written kernels and derivatives in three languages | Straight-line programs with dimension checking and automatic differentiation; identities checked by FrankenSymPy and bound to a content hash |
| C-15 | Evidence is a test martingale [S] | p-values at a fixed sample size | Stop at any time with the error rate intact (Ville's inequality) |

**Reviewed, with corrections pending.** The [mathematical review](planning/fable/blend/REVIEW_MATH.md) of the v1.0 drafts
confirms the Sarrus and Bennett mobilities and narrows some of the claims around them. A subgroup lattice explains
Sarrus but not Bennett, whose mobility follows from metric conditions (F-04). The (H, K) gait atlas cannot separate trot, pace and walk with
the ambient group the draft states (F-03). The draft's test for a real root is reversed: a ball disjoint from its
conjugate proves that the root is not real (F-05). Exact momentum conservation holds only for the symmetries a
potential keeps (F-15). These concern the drafts, not base v0.2, which has not been reviewed this way.

Proofs are typed and the weakest link wins. A result is `Proved`, `Certified`, `Tested` or `Estimated`;
"'Verified' is not a class and the word is not used for results" (§10.2). The base plan does not claim that the
simulator is verified, that floating-point arithmetic satisfies identities exactly, or that any Mathlib Lie
group result has been re-checked (§10.5). Lean is "a second opinion, not a prover we drive".

---

## Three tiers and a trust ladder

| Tier | Precision | Engine | Scale | Role |
|---|---|---|---|---|
| `referee` | `f64` | FrankenSim through the airlock crate: exact Coulomb friction, certified collision | One to a few dozen worlds | The most trusted tier. "The referee is not called ground truth" |
| `batch-cpu` | `f64` or `f32` | The same derived kernels and stage order as the GPUs | Hundreds to thousands | Permanent reference for the GPU tiers |
| `gpu-cuda`, `gpu-metal` | `f32` | Derived kernels from one source | Thousands to tens of thousands per device | Rollouts, training, broad search |

**Teleport.** Every world has an address, `(run_seed, world, episode)`, and every random stream is keyed by
`(WorldAddr, step, stream_id)`. In NS-02, "world 1,337 of 4,096 produces NaNs on the GPU"; `frob sim replay
--world 1337 --tier batch-cpu` replays it on the CPU, stops at the first differing stage digest and names the
contact that started it.

**Exploit detectors.** A policy is re-evaluated at half the time step, on the next tier up, with parameters
moved within their stated uncertainty, and under an alternative contact law. The flags are `dt-sensitive`,
`tier-sensitive`, `parameter-sensitive` and `law-sensitive`. A mirrored policy must match its mirror image.

**Determinism classes** (base §27.1). D0 exact (integer, rational, symbolic). D1 bitwise across backends
(hypothesis until GATE-SPARK-01). D2 bitwise per backend, run to run and across worker counts, which every
kernel must meet. D3 within a recorded tolerance. D4 unspecified, "never cited as evidence". Float atomics are
banned in kernels; sums use fixed-shape pairwise trees; positions are anchored as an integer cell plus an `f32`
offset (worst case 0.48 µm against 7.6 µm for a raw `f32` at 100 m). The v1.0 draft adds a test that a world's
trajectory does not depend on the batch it ran in, and drops half precision (refuted: the condition number times
half-precision round-off is at or above one in 37–64% of sampled contact states).

---

## The claims

| Claim | Meaning | What establishes it |
|---|---|---|
| **Proof** | A stated proposition holds | A checker accepted it: FrankenSymPy's proof kernel or a Lean kernel |
| **Known answer** | Recovers an analytic or synthetic truth | A known-answer test with its derivation on file |
| **Parity** | Reproduces a reference tool or published number | A fixture from a pinned oracle, never regenerated from the candidate |
| **Fidelity** | A faster tier agrees with a slower, more trusted one | The fidelity ledger: per-quantity disagreement distributions on the standard battery |
| **Reality** | Simulation agrees with physical measurement | A real-robot dataset with provenance. "In v1 this claim is almost always `no-claim`" |
| **Performance** | Speed or scale on named hardware | A recorded run with the full disclosure block |

Base v0.2 has these six. The v1.0 draft adds a seventh, **Design comparison** ("one design is better than another, or
a design meets a mission, on a stated family of worlds"), and turns the rule into a type: `frob-core` owns a
`ClaimProduct` whose class is the minimum over its inputs, with no setter.

They are never averaged into one number (base §6). A gate names exactly one claim. A failure is a failed
cell, never an excluded cell. Agreeing with MuJoCo "establishes that two programs agree", not that either
matches reality. "A result inherits the weakest claim among its inputs. A proved kernel fed an assumed friction
coefficient produces an estimate." Neon makes the same point in the type system: `ClaimProduct<VerifiedFormal,
EvaluatedDesign>` has no conversion to `VerifiedFormal`, and every export hard-wires
`sim_to_real_certified: false`.

---

## Architecture

```text
 agent ─► Tether: frob · frob-mcp · frobd        one verb registry, budgets, refusals, JSON envelope
             │
             ▼
 Skeleton ── design term ─► typed edits ─► RobotModel (content-addressed ModelId)
   │  Marrow: Lie groups, frames, cones, finite groups     Sinew: actuator, battery and thermal cards
   ▼
 Atelier ── check (A-01..A-16) ─► Reflex: invariant filter + synthesized baseline controller
   │
   ▼
 Kinesis ── referee ─────────── FrankenSim through the Airlock (frob-fsim), f64
   │        batch-cpu ──┐
   │        gpu-cuda ───┼─────── Spark: one derived kernel source, three backends
   │        gpu-metal ──┘
   │   Traction: contact, wheels, tracks, terrain      Habitat: scene packs ─► Sensorium: sensors, stills
   ▼
 Cortex: VecEnv, on-device PPO          Arena: task families, metrics, scorecards, batteries
   │
   ▼
 Chronicle: ledger, telemetry, energy ledger, replay and teleport ─► Atelier: explain, compare, promote

 Lemma: certificates and theorem cards        Gauntlet: conformance tiers, oracles and gates
```

| Subsystem | Crates (base §26.2) | What it owns |
|---|---|---|
| Marrow | `frob-lie`, `frob-algebra` | Lie groups and algebras, retractions, frame typing; finite groups and representations, cones and a cone solver, the inertia cone |
| Skeleton | `frob-design`, `frob-model`, `frob-io` | Design terms (`frob.design/1`), constructors (`attach`, `serial`, `mirror`, `radial`, `closure`…), typed edits, the model compiler, URDF/MJCF import and export with a loss report |
| Sinew | `frob-actuate` | Content-addressed actuator cards; models from ideal torque (L0) through torque-speed envelope, reflected inertia and delay (L1, the training default) to winding current, two-node thermal and battery sag (L2) |
| Kinesis | `frob-dyn`, `frob-kernels` | The three tiers, the ten-stage batched step, integrators and the trust ladder |
| Traction | `frob-contact` | Collision against distance fields, contact laws, wheels and tires, tracks, rigid and soft terrain |
| Habitat | `frob-scene` | Scene packs (quantized distance-field bricks, materials, semantic labels, articulated fixtures, moving surfaces), terrain and interior generators |
| Sensorium | `frob-sense` | Proprioception, IMU, force and contact, depth, lidar and height scans as deterministic ray queries; stills with a per-pixel link identifier |
| Reflex | `frob-control` | Contact-aided invariant EKF; baseline controller synthesis for legged, wheeled and tracked designs |
| Cortex | `frob-learn` | `VecEnv` (`reset`, `step`, `snapshot`, `restore`) over device-resident buffers, declarative tasks, on-device PPO, the FrankenTorch bridge |
| Spark | `frob-kir`, `spark/frob-spark-cuda`, `spark/frob-spark-metal`, `derive/frob-derive` | Straight-line program IR, emitters, the CUDA and Metal annexes, symbolic derivation |
| Arena | `frob-arena` | Task families, metrics, scorecards, test batteries, a corpus of about 200 designs with planted defects |
| Atelier | `frob-analyze`, `frob-opt` | Static analyses, comparison, lineage (`diff`, `blame`, `bisect`), optimization, registered predictions |
| Chronicle | `frob-chronicle` | FrankenSQLite ledger, telemetry levels T0–T3, replay digests, the energy ledger, token-budgeted summaries |
| Lemma | `frob-lemma` | Claims, certificate production and offline verification, theorem cards T-01 to T-14 |
| Tether | `frob-verbs`, `frob-cli`, `surface/frob-mcp` | The verb registry, envelope and schemas; the `frob` and `frobd` binaries; the MCP server |
| Gauntlet | `frob-conformance` | Six test tiers (laws, known answers, parity, metamorphic, chaos, determinism), oracle fixtures, batteries |
| Airlock | `frob-fsim` | The only crate that names FrankenSim's fast-changing types |

Base v0.2 also has `frob-core` (identities, hashing, evidence classes, refusals, units glue) and an `xtask` limited to six
checks (`check-layers`, `check-deps`, `check-unsafe`, `check-generated`, `check-contracts`, `check-stack`).
Crate size caps sum to 361 kLoC, and exceeding a cap by 15% triggers a conversation rather than a trim.

The v1.0 draft keeps these, splits out crates such as `frob-cert`, `frob-spec`, `frob-phylum` and `frob-mission`, and
raises the cap to 561 kLoC over 43 crates. Its rule for adding one: "a new crate is created only for a distinct
dependency, a distinct device target, or a proof boundary".

Layers depend only downward. The core crates set `unsafe_code = "deny"` and never depend on a GPU crate; CUDA
and Metal live in annex workspaces under `spark/` with registered unsafe capsules. Forbidden everywhere: tokio
and its ecosystem, a direct Rayon pool, BLAS and LAPACK, C or C++ numerical libraries and a Python interpreter
(base §26.3). FrankenSim's own dependencies bring `nalgebra` and `rayon` in transitively; the plan says so: "'Franken-only'
describes direct edges."

---

## Command surface (planned)

One registry, `registries/verbs.toml`, generates the CLI, the MCP tools, the schemas and the docs, and a
parity gate keeps them identical. "Agents ask questions and spend budgets. They do not drive solvers."

```bash
frob doctor | capabilities | schema | robot-docs        # operator verbs (M0)
frob spark devices | probe | check | precompile sm_89   # device evidence; IEEE probe; CPU-vs-device divergence ledger
frob design new | show | templates | edit | check       # typed edits only; check aggregates the static analyses (M1)
frob design diff | blame | bisect                       # lineage; bisect finds the edit that broke a task (M4)
frob design import | export                             # URDF/MJCF, always with a loss report
frob analyze <A-01..A-16>                               # one verb per static analysis
frob control synth | show                               # baseline estimator + controller
frob scene gen | compile | import | list | show         # terrain, interiors, private scans
frob sim estimate | run | replay                        # estimate never runs physics; replay --tier teleports
frob train start | status | stop | export               # on-device PPO
frob eval run | scorecard | compare | promote           # anytime-valid comparison; promotion to the referee
frob telemetry summary | query | energy | failures | stills | inspect-pixel | export-html
frob explain run | design                               # ends with next_actions
frob opt start | status | archive                       # CMA-ES, BO, racing, MAP-Elites archive (M4)
frob predict register | report                          # registered predictions, scored later
frob prove kernel | verify | list                       # certificates, re-checked offline
frob report design | run                                # evidence packages
frob job list | status | wait | cancel                  # long work through frobd
```

With `--json`, every verb writes exactly one `frob.envelope/1` envelope: `verb`, `ok`, `exit`, `result`,
`evidence` (claim class, tier, model identity, determinism), `cost` (wall time, CPU core-seconds, GPU-seconds),
`warnings` and `next_actions`. Long jobs stream NDJSON progress first. Exit codes are fixed:

| Exit | Meaning |
|---|---|
| 0 | Completed, nothing to flag |
| 2 | Completed with findings or flags |
| 3 | Refused input (`FROB-REF-1xx` to `3xx`) |
| 4 | Budget stop; nothing partial is published |
| 5 | Capability not implemented or not admitted |
| 6 | Backend or device unavailable (`FROB-REF-6xx`) |
| 7 | Conflict: idempotency key reused or lineage conflict |
| 9 | Indeterminate. "Never retry blindly" |

The v1.0 draft keeps these and adds exit 8, "accepted and still running", because long tools take a bounded wait
(`sync_wait_ms`) and then return a job handle. Appendix J of the draft registers 243 verbs, adding `mission`,
`nemesis`, `autopsy`, `ground`, `foundry`, `cert` and `study`. The coherence review had found several chapters using
other names for the same actions (F03); the second reconciliation keeps the owner chapter's name in each case.

"Exit code 1 is never emitted on purpose." Every verb that can cost more than a second accepts `--estimate`.
Mutating verbs take idempotency keys. Capabilities are scoped (`design:write`, `sim:run`, `gpu:seconds=600`,
`scene:import`, `train:run`). Agents supply data, never code: designs, task specs, controller configurations
and policy weights, but no kernels or programs. The MCP server is built on fastmcp-rust, groups tools "into
clusters of at most seven per task", and treats imported text as data, "never interpreted as instructions".

Astra's interface has the same shape under different names (`frobot design patch`, `study plan|submit|compare`,
`run replay|fork`, `experiment suggest`) and adds six inverse queries, among them `mission.synthesize`,
`mission.refute` and `result.revalidate`. Neon's `frob-mcp` plans 46 tools in seven clusters with
`job_poll`, `job_stream` and `job_cancel`.

---

## Hardware and performance targets

Two machines are named in every edition: `trj`, a Linux x86-64 host with two RTX 4090s (24 GiB each, no
NVLink, peer-to-peer unverified), and a Mac Studio with an M5 Ultra (36 CPU cores, an 80-core GPU, Metal 4 and
256 GB of unified memory, "the large-scene machine"). Metal has no double precision and the RTX 4090's doubles
run at about 1/64 rate, so the devices work in single precision. Anchored positions, group-valued updates,
fixed-order sums and double-float accumulators are meant to make that enough, and BET-08 tests whether it is.

**Base v0.2's throughput table (§28.1) is withdrawn.** Round 2 found that it mixed world-steps and env-steps (400 k
env-steps/s of training at a decimation of 10 implies 4.0 M world-steps/s, twice its own 2.0 M physics-only target),
set a 15-minute training target 2.2× slower than a published run on the same card, put its humanoid target about five
times below a published figure, and put its CPU target 7 to 11× below published CPU MuJoCo per core. The v1.0 draft
restates the targets as ratios to an incumbent measured on our own hardware, in stated units, each with a **tolerance**
(a parity band the plan's argument needs) and an **ambition** (a better number nothing depends on). Planning values
below come from published figures. None is a measurement, and Phase 2 can still change them.

| Workload (v1.0 draft §36.3, at h = 2 ms, d = 10) | Hardware | Tolerance | Ambition | Incumbent reference |
|---|---|---|---|---|
| Quadruped, physics only | One RTX 4090, 8,192 worlds | ≥ 0.67 of the incumbent, floor 2.0 M world-steps/s | ≥ 1.0; 4.0 M | mjlab on MuJoCo Warp: 4.8 M world-steps/s at d = 4 (RTX 5090) |
| Quadruped, full environment step | Same | ≥ 0.75 of the physics row | ≥ 0.85 | The same benchmark: 347 k env-steps/s against 1.20 M for physics |
| Quadruped, PPO training throughput | Same | ≥ 0.60 of the physics row | ≥ 0.70 | Playground reports 417 k on an A100 with MJX, at d = 5 |
| Quadruped, time to a return threshold (cold) | One RTX 4090 | ≤ 1.25× the incumbent (re-measured on the same card); planning value 513 s | ≤ the incumbent; 410 s | Playground: about 410 s, read by eye from a figure |
| Humanoid, physics only | One RTX 4090, 8,192 worlds | ≥ 0.67; planning value 1.7 M | ≥ 1.0; 2.55 M | MuJoCo Warp nightly: 2.55 M (RTX 6000 Ada) |
| Mixed batch of three body plans | One RTX 4090, 8,192 worlds | ≥ 0.70 of the harmonic mean | ≥ 0.86 | Newton's Kamino solver: 0.86 at 2,048 environments (maximal-coordinate, documented as not deterministic, GPU not stated) |
| Quadruped physics, Metal | M5 Ultra | ≥ 0.25 of one RTX 4090 (BET-09) | ≥ 0.60 | None published on this chip; Genesis is measured beside it |
| Humanoid physics, `batch-cpu`, `f32` | M5 Ultra, 24 threads | ≥ 0.5 of CPU MuJoCo on the same machine | ≥ 1.0 | CPU MuJoCo (`f64`): 550 k world-steps/s on 24 threads, measured here under load |
| `design check` / `control synth` / generate and compile an interior | CPU | < 2 s p95 / < 5 s / < 60 s | `design check` after one edit: 0.2 / 0.5 / 1.0 s | |

The mixed-batch row is one of five thresholds the drafts give for that experiment (coherence review F07). The gate
register, drafted afterwards, reads the gate and kill-gate numbers as different workloads; it does not take up the 0.33
in the bet itself (BET-18). The reference capacities behind the physics rows are stated two ways (F04), and the register
does not reconcile them.

A published figure is a fact about its source, to be re-read from the primary source before a gate cites it. Every
performance claim must carry a disclosure block (base §28.2, draft §36.7): hardware and driver, backend and compiler
versions, world count, time step and substeps, solver iterations and tolerance, contact capacity and mean active
contacts, sensors, whether policy and learning are included, scene, determinism class, warm-up, launch-graph
capture, median and p99 step time, and peak memory. The base plan adds the block because a published simulator
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
  feed the referee-coverage arithmetic. The drafts give about 0.1 to 2.4% of GPU world-seconds if the referee runs
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

The base plan registers ten admission gates (`INT-*`, one per foundation), about fifty convergence gates and
nine kill gates. The drafted v1.0 [gate register](planning/fable/blend/parts2/P13b.md) lists 37 admission gates, 279
convergence gates and 112 kill gates, with thirteen gates and seven kill gates of the ten-fold test beside them and an
index of 49 gates and 22 kill gates that later subsections define. It says that it governs where a chapter differs. It
does not list the nine gates (GATE-GAUNT-02 to -10) and one kill gate (KILL-DEVORACLE) of §38, so treat it as provisional. "No
self-certification": a gate's PASS is recorded by a fresh-context session, and every gate has a registered negative
control that it must catch.

| Kill gate | Trigger | Fallback |
|---|---|---|
| KILL-FSIM | FrankenSim cannot be consumed by path within two weeks of M0 effort, or upstream breakage exceeds 20% of working days | Lift the needed FrankenSim modules with provenance; the README says "lift-only" |
| KILL-KIR | At M2 the derivation toolchain cannot cover the kernel arithmetic, or derived kernels stay more than 1.3× slower than hand-tuned ones | Hand-written arithmetic in Rust and the shared dialect, under differential tests |
| KILL-CONTACT | Fast-tier contact disagrees with the referee on slip and creep | Hunt–Crossley contact as default at lower throughput; then referee-only locomotion claims |
| KILL-METAL | Metal below 25% of one RTX 4090, or not deterministic run to run | Metal becomes a development and replay tier; CUDA trains; the Mac trains on `batch-cpu` |
| KILL-MCP | The MCP library fails admission | A separate server talking NDJSON to `frobd` over a local socket; then CLI only |
| KILL-STATIC | Static checks fail to predict which designs fail (AUC < 0.8, or < 0.05 above a mass-and-torque heuristic) | `design check` becomes advisory lints and the pruning claim is withdrawn |
| KILL-BASELINE | Fewer than 80% of check-passing designs walk or drive 5 m under their baseline | Synthesis limited to the classes that pass |
| KILL-EQUIV | Equivariant policies need more than 0.7× the environment steps of unconstrained ones | Symmetry used for augmentation and metamorphic tests only |
| KILL-LEMMA | Fewer than half of derived kernels carry replayable certificates after one repair | Exact-rational randomized tests, recorded as `Tested`; "certificate" removed from manifests |

A program gate at the end of M1 pauses for a written re-plan if M0 and M1 take more than 1.5× their estimate
because a sibling lacked a primitive. Neon adds three Brenner exclusion tests (XT-P1, campaign algebra against
per-step algebra; XT-P2, heterogeneous morphologies against specialized templates; XT-P3, formal certificates
against real contact) and 21 kill gates, and its killed-hypothesis list now runs to K-13. Astra defines a sealed tenfold test (§19): four strata of held-out
missions, the strongest available rival pipelines as comparators, one-sided 95% lower bounds of at least 10 in
each stratum, and "One false certificate disqualifies that arm from the scoped claim."

---

## Milestones

From the base plan (§34). "M1 is a design studio on the CPU referee tier"; GPUs arrive at M2, "on top of
something that already works". The planning estimates are in elapsed swarm weeks (M0 one, M1 three, M2 to M5
four each) and are "not acceptance evidence".

| Milestone | Delivers | Scenarios |
|---|---|---|
| **M0** spine and airlock | `frob doctor` lists admitted foundations; `frob sim run g1 --tier referee --seconds 2` reproduces FrankenSim's own digest; `frob spark probe` on both device families; one `K-cay-step` kernel source running on `batch-cpu`, CUDA and Metal; a certificate that replays offline; M0 verbs over MCP | |
| **M1** referee and static studio | Design language and compiler; quadruped, humanoid, differential-drive and mecanum templates; URDF/MJCF import with loss reports; `design check`; actuator models L1 and L2; referee rollouts; invariant filter; baseline controllers; telemetry and energy ledger; `explain` | NS-01, NS-07 |
| **M2** batch and Spark | Batched step on three backends; terrain and scene packs; sensors and stills; on-device PPO; fidelity and divergence ledgers; replay and teleport; measured throughput with disclosure | NS-02, NS-10 |
| **M3** Habitat and Arena | Seven interior archetypes; interior navigation, payload and tight-space tasks; anytime-valid comparison, promotion and exploit detectors; `frobd` and full MCP | NS-03, NS-04, NS-09 |
| **M4** co-design and breadth | Optimization, racing and quality-diversity archive; lineage; predictions; equivariant networks; gait atlas; tires, tracks, hybrids and moving surfaces; reach-and-carry | NS-05, NS-06, NS-08 |
| **M5** Lemma and reality | Certificate coverage; Lean capsules; variational integrator; private real-place ingestion; a first Reality record or a recorded `no-claim` | NS-11, NS-12 |
| **M6** frontier | Unscheduled epics: gradients through contact, granular terrain at scale, flexible links, soft robots, moving people, OpenUSD, hydraulics | |

The v1.0 charter renames and extends the list to nine: **M0** spine and airlock; **M1** CPU referee studio plus the
first mission slice on the CPU; **M2** batch, GPU backends and mixed batches; **M3** places, tasks, adversaries,
qualification and the partial ten-fold gate; **M4** amortized control, co-design and mission queries at scale; **M5**
proofs, reality and manufacture; **M6** frontier physics; **M7** the ten-fold tournament; **M8** frontier delivery. Phase
2 drafts 80 work packages ([§44](planning/fable/blend/parts2/P14.md)) and 442 beads
([§45](planning/fable/blend/parts2/P15.md)) behind them. These are proposed tasks for a future implementation phase.

---

## Foundations

FrankenRobotics is built on the FrankenSuite. Each foundation has a posture, an admission gate and a written
fallback (base §25.4). An admission record counts only if it cites a commit whose tracked `Cargo.lock`
contains the foundation.

| Foundation | Posture | Used for | Fallback |
|---|---|---|---|
| [frankensim](https://github.com/Dicklesworthstone/frankensim) | Consume by path, behind `frob-fsim` | Referee dynamics and contact, geometry, optimizers, racing and e-processes, evidence ledger | KILL-FSIM: lift with provenance |
| asupersync | Consume | Structured concurrency and cancellation; one package per build | Process boundary |
| frankensqlite | Consume | The Chronicle ledger | Append-only JSON-lines ledger |
| frankentorch | Consume | `ft-kernel-metal` gateway in the Metal annex; CPU trainer, gradient oracle, safetensors | Bind Metal directly |
| `cudarc` (third party) | Annex only | CUDA driver and NVRTC at run time, no toolkit needed | No CUDA tier |
| frankensympy | Build time and checker | Kernel derivation; certificate checking | KILL-KIR, KILL-LEMMA |
| franken_lean | Subprocess | Re-check and seal Lean proofs | Stock Lean |
| fastmcp-rust | Consume | The MCP server | KILL-MCP |
| franken_numpy, frankenscipy | Narrow consume and oracle | Array I/O; `solve_ivp` as an ODE oracle | |
| franken_threed | Subprocess and pattern | Packaging replay viewers | Plain Three.js |

The firewall: "M1 depends only on FrankenSim, asupersync and FrankenSQLite. If the GPU, symbolic and proof
foundations all failed admission, M1 would still ship a referee-tier design studio on the CPU." Outside
engines (MuJoCo in double precision, run out of process) are comparison oracles only. Reference robots come
from MuJoCo Menagerie under their own licences, and scene assets are procedural by default.

The v1.0 draft keeps the postures and records what a reading of the sources found: four defects or limits reported in
FrankenSim and other foundations are "facts to recheck", each with an admission test and an upstream ask. Its
`INT-TOOLCHAIN-01` gate records which of four nightly compilers build the one crate that uses an unfinished language
feature (`fs-qty`); if a needed toolchain cannot, the fallback is a small, dependency-free `frob-units` crate.

---

## Scope boundaries

Not a photorealistic renderer. Not a Python library ("The users are agents with a CLI and MCP"). Not a
real-time or hardware-in-the-loop system, and not ROS. No Windows. No soft, continuum, flying or swimming robots
in v1, and no dexterous in-hand manipulation (mobile reach-and-carry is in scope at M4). No general formal
verification of controllers. The v1.0 charter narrows this list: rods and cables, cloth, fluid films, slosh, coupled
multiphysics, a rotor port for flight, tactile hands, firmware export and facility co-design are admitted as optional
or moonshot work behind flags, with known answers and a kill gate, and stay off the critical path (charter G11). The
anti-goals (base §4.3) are firmer: never lower fidelity silently for frame
rate, never wrap MuJoCo, PhysX or another foreign engine as the production engine, never publish throughput
without the disclosure block, never accept an inertia on faith, and never put "a language model inside the
physics or control loop".

**No safety certification** (base §29.6). "Simulation results are not a safety case for a physical robot."

## Privacy

A scan of a real place (NS-12, "Your own kitchen") becomes a scene pack marked `private`, stored under a
private root outside the repository. Stills, replays, reports and fixtures derived from it inherit the mark and
cannot be exported without an explicit flag. "The scan never leaves the machine." Scale stays `ESTIMATED`
unless it is anchored by a measurement. Licences are enforced when a pack is built, so research-only assets
cannot reach an exported report or a redistributed pack.

---

## Repository layout

Today:

```text
COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md          base plan v0.2
COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md   Astra edition
COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md    Neon v2.0
CHANGELOG.md                                        the planning history, with evidence links
planning/mor/                                       round 1: modes-of-reasoning audit of Neon v1 (case, 16 claims, report)
planning/duel/                                      round 1: three-model idea duel (ideas, cross-scores, reactions, report)
planning/brenner/                                   round 1: question of record, disagreement register, synthesis
planning/verify_plan_math.py                        consistency checker for the Neon edition
planning/fable/mor/ brenner/ duel/                  round 2 on base v0.2: audited claims, the verdict register, its
                                                    challenges and adjudication, three-model duel
planning/fable/graveyard/ artifact/                 graveyard harvest and decision-core compilation
planning/fable/blend/                               v1.0 in progress: charter, digests, idea catalog, 13 draft parts,
                                                    inventories, Phase 2 brief and parts, two reviews, two gap
                                                    audits, fix decisions and logs
AGENTS.md                                           project scope: planning only
.beads/                                             issue tracker (`br`) workspace, no issues
planning/astra-10x-reproduction.zip                 Astra's planning experiments, receipts and verifiers (reproduction bundle)
```

Target (base §26.1):

```text
Cargo.toml  rust-toolchain.toml  franken-stack.lock    workspace, pinned nightly, sibling pins
AGENTS.md  DEPENDENCY_ADMISSION.md                    agent contract; admission evidence per foundation
crates/frob-*/                                        the core workspace (unsafe denied)
spark/frob-spark-cuda/  spark/frob-spark-metal/       GPU annexes with their own lockfiles and SAFETY.md
surface/frob-mcp/                                     MCP server generated from the registry
derive/frob-derive/                                   symbolic derivation and certificate production
apps/                                                 links core and annexes into frob and frobd
kernels/                                              derived kernel sources, checked in with digests
registries/                                           verbs, refusals, kernels, gates, actuator cards
lean/  viewer/  assets/                               Lean project; Three.js replays; templates
tests/fixtures/oracle/  tools/oracle/                 pinned oracle fixtures and their capture scripts
docs/delivery/                                        the delivery log (never in the plan)
```

The v1.0 draft (§34.1) adds `bindings/` (an optional Gymnasium adapter), `worker/` (FrankenSim's out-of-process
fallback), `schemas/`, `demos/` and `receipts/`.

## Reading paths

- **What and why**: this README, then base §1–§6.
- **The mathematics**: base §7–§10 and Appendix D; Neon Part II.
- **Inverse design and the tenfold test**: Astra's front matter, then §18–§20.
- **GPUs and determinism**: base §20, §27 and §28; Astra §11; Neon §17.
- **The agent surface**: base §24 and Appendix J; Astra §10.
- **Gates, risks and delivery**: base §30–§37; Neon §25–§29; Astra §13–§16.
- **The v1.0 direction**: the [integration charter](planning/fable/blend/INTEGRATION_CHARTER.md), then draft part
  [P00](planning/fable/blend/parts/P00.md) (thesis, bets, goals, the corrected fossils) and the part named by your topic.
- **How the plans were reviewed**: [CHANGELOG.md](CHANGELOG.md), then [`planning/`](planning/).

---

## FAQ

**Why not wrap MuJoCo, MuJoCo Warp, Newton or Genesis?** The base plan makes it an anti-goal (§20.9), citing
its fossils plus practical reasons (Python ecosystems; Newton is CPU-only on macOS). Round 2 withdrew several of those
fossils and left a shorter list of documented gaps ([TL;DR](#tldr)). It is also the program's strongest open rival: H-19 says a thin layer over an incumbent gets most of the value, and
the v1.0 draft keeps it as a bet with a tie rule (BET-14) and a composed-incumbent arm in the ten-fold test. MuJoCo
runs out of process as a parity oracle. Parity with a simulator is not Reality.

**Why Lie theory and abstract algebra?** Because each construction pays for itself or is cut. Frame errors
become type errors. Group-valued updates stay on the group. The Cayley map's arithmetic is a candidate for
bit-identical results across CPU, CUDA and Metal, and it can carry a certificate. One cone abstraction serves
contact, control and certificates. A design's symmetry group yields augmentation, equivariant policies and
free metamorphic tests. Static analyses answer design questions in seconds.

**Why both CUDA and Metal?** The owner's hardware is two RTX 4090s and an M5 Ultra with 256 GB of unified
memory, which can hold scenes no 24 GB card can. BET-09 tests whether Metal is "a real rollout tier, not a
demo"; if not, KILL-METAL demotes it to development and replay.

**Why keep a CPU referee if the GPUs are faster?** "The CPU reference is permanent. GPUs propose, the referee
disposes." A world that fails on a GPU must be replayable somewhere it can be inspected, and a fast-tier result
must survive a sample on a trusted tier before anyone calls it established.

**Why synthesize a baseline controller?** Training a policy per candidate is the dominant phase of a design loop
(about 85% of a quadruped iteration and over 90% of a humanoid one in published figures). A synthesized baseline
gives feedback on day one. It is not meant to be the best controller, and whether its design ranking can be trusted
is the program's first experiment.

**Why anytime-valid comparisons?** An agent can stop the moment the evidence is decisive, with the error rate
intact. Pairing episodes by world address "does more for sample size than the test itself".

**Why are there three plans, and a fourth?** Three were written in parallel on 2026-11-21 and none cites another. A
hybrid v1.0 is being assembled from them and the research rounds, and is meant to become the canonical plan (see
[The v1.0 hybrid](#the-v10-hybrid-being-assembled)). [CHANGELOG.md](CHANGELOG.md) records which research fed which
edition.

**Does a good simulated result mean the robot will work?** No. In v1 the Reality claim is almost always
`no-claim`, and nothing here is a safety case. Build it and measure it.

---

## License

MIT with the OpenAI/Anthropic rider. See [LICENSE](LICENSE).
