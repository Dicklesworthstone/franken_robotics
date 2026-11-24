---
key: "EPIC-PROGRAM"
title: "FrankenRobotics: proving ground, agent design studio, mission-query engine, measured against a composed rival"
type: "epic"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "tenfold", "solid"]
deps: []
needs: []
related: ["N00-04", "N00-21", "N00-22", "N00-30", "N41-26", "STD-E2E", "STD-TEST"]
gates: ["GATE-TENFOLD-06", "GATE-TENFOLD-07", "GATE-GAUNT-01"]
plan_refs: ["header", "§0.1", "§0.2", "§0.5", "§0.7", "§0.8", "§1.4", "§1.5", "§1.8", "§2", "§3", "§8", "§35.1", "§38.2", "§38.11", "§43.1", "§43.2", "§47.1", "AGENTS.md", "REVISION_2.md"]
---
## Background

**Read this bead first.** It is the root of the bead graph and states what the program is, what it claims, what
it does not claim, and the rules every other bead inherits.

**What it is.** FrankenRobotics is one mathematical instrument that serves six products sharing one model, one
evidence ledger and one verb registry:

1. A proving ground (subsystems Arena, Cortex): a vectorized, device-resident environment API over batched
   physics for legged, wheeled, tracked and hybrid robots on generated terrain and in generated or ingested
   interiors (homes, apartments, offices, warehouses, manufacturing plants, retail stores, commercial kitchens).
   Many different robots share one batch. Missions are stated as requirements, and a result is qualified in a
   lane the discovery process cannot read.
2. A design studio for AI agents (Atelier, Tether): a command line and an MCP server whose verbs create and edit
   a robot design, analyze it without simulation, simulate it, compare it with statistical honesty, read
   structured telemetry about why it behaved as it did, optimize it, and say which earlier verdicts survive an
   edit.
3. A mission-query engine (Augur): given a mission, a parts catalogue, a controller class and a description of
   what the robot observes and when, it returns the set of designs that can do the job (an inner set with
   witnesses, an outer set with exclusions, the unresolved cells between them) and the cheapest experiment that
   would shrink the gap. The body, a sensor and the building are all admissible answers.
4. An adversary and a counterfactual debugger (Nemesis, Autopsy): search for the worlds that break a design
   inside physical-legality bounds, reduction of each failure to a minimal counterexample world, valid bounds on
   rare failure rates, and explanation of a failure by intervention on forked replays with a stability figure.
5. A path to reality and to the workshop (Ground, Foundry): calibration against imported measurements with
   predictions frozen in advance, reality bands only inside a calibrated family, and build packets whose
   machine-readable flags say they authorize nothing.
6. A mathematical instrument (Marrow, Lemma, Spark): configurations are Lie-group elements, joints are
   subgroups, contact is a cone program in the dual Lie algebra, symmetry is a finite group acting on the design,
   and hot kernels are derived from that algebra, which also supplies test oracles and offline-checkable
   certificates.

Five nouns recur. A *design* is a robot's intrinsic properties and connections. A *world* is geometry, material
fields, lighting, moving objects and their behaviour. A *task* is initial states, observations, actions, goals,
terminal events and metrics. A *mission* is a task family with requirements and the quantifiers relating designs,
controllers and worlds. A *study* binds versions of these to controllers, random streams, numerical settings,
hardware profiles and one declared analysis rule.

**For whom.** Primary user: a design agent calling verbs. Also: mission authors, training agents, controls
engineers, environment authors, implementing agents, the owner, maintainers of sibling projects, the adjudicator
and the rival-stack engineer of the ten-fold test, an auditor holding only a certificate bundle, bench and
manufacturing engineers, and sceptical reviewers (reader map: N00-04).

**Why it exists.** Each component technique already exists somewhere (articulated-body dynamics, convex compliant
contact on GPUs, Lie-group integrators, invariant filtering, morphological symmetry, monotone co-design,
anytime-valid inference, sampling control, cross-body policies, configuration-space certificates, adaptive stress
testing, conformal prediction, topology optimization). What was not found is their composition: one design
identity and one evidence ledger carried across static analysis, simulation tiers, statistics, family queries,
adversarial search, calibration and solid mechanics, so that a statement proved or measured at one layer is
usable at the next and is withdrawn when an edit invalidates it. That the composition is worth more than its
parts is the central HYPOTHESIS; its only test is the end-to-end comparison against a composed incumbent
(BET-13, BET-14).

**Status (FACT).** Planning stage. No FrankenRobotics code exists; nothing was compiled, run or measured as
FrankenRobotics; no incumbent GPU simulator was run by this program; the paper-scale pilots used no FrankenSim
dynamics, contact or sensor pipeline. The repository rule (`AGENTS.md`): work on plans, their evidence and their
review; building the product or executing implementation beads requires an explicit change of scope from the
owner. Beads describe work; they are created and changed only with the `br` tool.

## Scope

In: the program as a whole; this bead is the container of the milestone epics EPIC-M0 to EPIC-M8 and the
statement of the rules they inherit. Out: every mechanism (each has its own bead). Hierarchy: EPIC-PROGRAM →
EPIC-Mn → work-package epics WP-01 to WP-80 → task and feature beads → their test and pilot beads. Key families:
`BD-nnn` seed beads; `N<unit>-nn` beads added during conversion; `BET-nn` one decision per thesis bet; `NS-nn` one
runnable North-Star scenario each; `SPEC-C-nn` construction specifications; `CARD-T-nn` theorem cards; `FD-…`
upstream work items in sibling repositories; `STD-E2E` and `STD-TEST` the shared test standards.

## Technical approach

**The ten-fold thesis on one page.** The owner's ambition is comparative: an order of magnitude better than every
existing system at getting from a robot-design question to an answer that can be trusted.

- *Metric (DESIGN).* Time to an independently accepted answer. An answer is a design and controller passing a
  frozen acceptance battery, a family answer with inner and outer sets and witnesses, a correct withdrawal or
  confirmation of earlier verdicts after an amendment, or a truthful `unknown` with its witness. Independently
  accepted means a checker that is not the proposer accepted it (a battery on worlds neither arm trained in; the
  offline auditor `frob-verify` for every certificate; a neutral adjudicator). The clock T runs from receipt of
  the mission to acceptance and includes failures, training, tuning, compilation, transfers, waiting, checking,
  repair and review. Restricted mean time at deadline H: RMT = E[min(T, H)]. Ratio per stratum:
  R = E[min(T_rival, H)] ÷ E[min(T_frob, H)] against the strongest composed rival (leading simulators, an
  optimizer, analytic bounds, a verifier and the same AI agent, tuned by a competent engineer, then frozen).
- *Ceilings (arithmetic on published figures; N00-17).* A faster step alone gives at most 1/(1 − s), about 7.7×
  at a step share of 87%; one design per agent turn gives at most (300 + t)/t, 3.5–6× for agent latency t of
  60–120 s; compute per decision C ÷ (c + c_T·f) reaches ten only if under 0.7% of designs are trained; the
  general bound is 1/(1 − f + f/r + h); a missed-deadline fraction q caps R at 1/q.
- *Levers (HYPOTHESIS; N00-20).* No single mechanism delivers ten-fold and no conjunction has been shown to. The
  candidate conjunction: decide many designs per turn; train few; spend where it matters; find failures instead
  of waiting for them; answer over families; do not recompute or re-decide; explain by intervention; short setup.
- *What the planning evidence refuted (N00-10 to N00-12).* Recursion-only kernel specialization (ceiling about
  1.2×); exact reuse of rollouts after an edit (at most about 10% of compute); half precision for contact;
  full-room multiphysics in every world. Narrowed: sampling control (a screen and a fallback, never an
  evaluator), a universal policy (family policies per morphology class), region certificates (per constraint,
  about six split parameters), rare-event estimation (finding and refuting, never cheap absence).
- *What is open and when it is measured.* Whether a thin layer over an incumbent recovers the gain (recovery
  ratio; M3, M7). Whether design missions are dominated by training or by agent turns (phase ledger; M0, M1).
  Whether cheap rungs rank designs as specialists do (rank agreement; pilot by M3, on our tiers at M4). The
  partial ten-fold gate is at M3 and the tournament at M7; the result is published whatever it is. Expected
  ranges against per-design training are 3–10 for quadruped briefs and 5–20 for humanoid briefs; against a
  composed rival that hosts every portable lever the expected ratio on fixed-topology CUDA missions is near one
  unless an engine-internal property binds.

**Seven claims and the claim product (N00-25 to N00-27; the type is BD-005).** Every result makes exactly one
kind of claim: Proof, Known answer, Parity, Fidelity, Design comparison, Reality, Performance. Each has one of
four evidence classes, ordered Estimated < Tested < Certified < Proved; only a Proof can be Certified or Proved.
The claim product is a type in `frob-core`: class, physics tier and determinism class take the weakest input,
scope takes the intersection, conditions take the union, and three constants `execution_authorized`,
`hardware_safety_signoff`, `sim_to_real_certified` are always false. The word "verified" is used for no result.

**Three physics tiers, one model.** `referee` (FrankenSim's CPU dynamics and contact under a transactional,
event-located step, double precision); `batch-cpu` (our own kernels on CPU, profiles `lockstep` and `strict`);
the GPU tier (CUDA on two RTX 4090 on host `trj`; Metal on the M5 Ultra Mac Studio). GPUs propose and the referee
disposes: a fast-tier result is provisional until promoted, and any world can be re-run alone on the CPU from its
address (seed, world, episode). Determinism classes: D0 exact; D1 bitwise across backends; D2 bitwise per
backend and independent of batch composition; D3 tolerance with a decision-level test; D4 unspecified, never
evidence. Verification tiers: G0 laws, G1 known answers, G2 parity, G3 metamorphic, G4 chaos, G5 determinism.

**Subsystems (handles for agents; crates in parentheses).** Marrow: Lie groups, finite groups, cones, order
theory (`frob-lie`, `frob-algebra`). Skeleton: design language, modules, archetypes, preflight, model compiler
(`frob-design`, `frob-model`, `frob-io`). Sinew: actuators, transmissions, batteries, thermal limits
(`frob-actuate`). Kinesis: dynamics, integrators, constraint solve, the trust ladder (`frob-dyn`). Phylum: many
bodies in one batch (`frob-phylum`). Traction: collision, contact, wheels, tracks, terrain mechanics
(`frob-contact`). Habitat: scenes, living worlds, clearance complexes, retrofits (`frob-scene`). Sensorium:
sensors, the clock contract, observability (`frob-sense`). Spark: GPU backends and kernel derivation (`frob-kir`,
`frob-kernels`, `spark/frob-spark-cuda`, `spark/frob-spark-metal`, `derive/frob-derive`). Reflex: estimation,
control, gaits, skills, shields (`frob-control`, `frob-gait`, `frob-shield`). Cortex: the gym, learning, family
policies, inheritance (`frob-learn`, `bindings/frob-gymnasium`). Arena: tasks, requirements, fixtures, metrics,
blinded qualification (`frob-arena`, `frob-spec`). Augur: mission queries and synthesis (`frob-mission`,
`frob-synth`). Nemesis: adversarial worlds and rare failures (`frob-nemesis`). Autopsy: counterfactual debugging
(`frob-autopsy`). Atelier: static analysis, comparison, lineage, cascade, decision cores (`frob-analyze`,
`frob-opt`). Chronicle: telemetry, replay, energy ledger, evidence (`frob-chronicle`). Ground: calibration
against reality (`frob-ground`). Foundry: structure, tolerance, fatigue, build packets (`frob-foundry`,
`frob-lifecycle`). Tether: CLI, daemon, MCP, dashboard, viewer (`frob-verbs`, `frob-cli`, `frob-tui`,
`surface/frob-mcp`, `viewer/`). Lemma: proofs, certificates, the offline auditor (`frob-lemma`, `frob-cert`,
`lean/`). Gauntlet: verification and the ten-fold harness (`frob-conformance`, `xtask`). Airlock: the only code
that names volatile FrankenSim types (`frob-fsim`). `frob-core` sits under all of them.

**Binaries.** `frob` (CLI), `frobd` (job daemon; owns devices, kernel caches and the job queue), `frob-mcp` (MCP
server, in the core build graph, runs verbs in process), `frob-verify` (offline certificate auditor, standard
library only), `frob-tui` (terminal dashboard).

**Milestones (a milestone is a set of demonstrations with recorded values, not a date).** M0 spine and airlock:
admission records, one kernel on three backends, the incumbent phase ledger on `trj`. M1 referee studio and first
mission slice on the CPU. M2 batch, backends, families: kill decisions for the kernel toolchain, contact, Metal
and mixed batches. M3 places, tasks, adversaries, qualification: the partial ten-fold gate recorded, whatever the
ratio. M4 amortized control, co-design, mission queries at scale: rank agreement measured, family-policy bets
decided. M5 proofs, reality, manufacture (off the critical path). M6 frontier physics (off the critical path).
M7 the ten-fold tournament: the measured ratio published with its phase ledger. M8 frontier delivery (Moonshot
epics). Critical path: M0 to M4, then M7.

**Rules every bead inherits.**

1. *Implemented means eight conditions* (N00-02): code in its mapped crate; a `CONTRACT.md` with ten sections;
   known answers against a pinned oracle never regenerated from the candidate; outputs carry the claim product;
   a CPU reference path, with a retained differential check for any GPU path; reachable through `frob --json`
   and, where exposed, `frob-mcp`; cancellation tested; its gate has a recorded PASS with measured values.
   Otherwise the capability is described as partial with the missing items named.
2. *Evidence labels.* FACT (observed, with path or source), DESIGN (a decision with its reason), HYPOTHESIS
   (with the test that settles it), TARGET (a number intended, not measured), OPEN (owner, deadline, fail-closed
   default). A number without a label is a defect. No number about this system is written as a result.
3. *Tiers.* [S] Solid, [F] Frontier, [M] Moonshot. Every [F] and [M] item has a paper-scale pilot with a
   falsifier before it is funded, a ceiling of its benefit computed before any speed test, and a kill gate with
   a lower-tier fallback named in advance (N00-07, N00-09).
4. *The owner's ruling on siblings.* Every sibling project is the owner's. A defect or mismatch in a sibling is
   fixed in that sibling's repository, under its own `AGENTS.md`, gates and conventions, as an upstream work
   item (label `upstream`, mirrored in the sibling's tracker), with an interim posture and a switch-over rule so
   no milestone waits on it. One Rust toolchain T for the whole build graph, chosen at INT-TOOLCHAIN-01 and
   moved only by a coordinated toolchain train. `frob-mcp` is an ordinary member of the core graph and calls
   verbs in process; the process boundary is only KILL-MCP's fallback.
5. *Against false progress.* A bead closes with a typed reason citing its own test and that test's output; every
   gate has a negative control it catches; a gate's PASS is recorded by a fresh-context session that did not
   implement the work; synthetic output is labelled `synthetic`; a fallback is described by the fallback's name.
6. *Refuse by name.* A request outside a validity domain gets a typed refusal with ranked repairs; `unknown` is a
   verdict; nothing is clamped or approximated in silence.

## Acceptance criteria

- [ ] EPIC-M0 to EPIC-M8 each closed, or closed with its kill decisions recorded as ADRs and fallbacks in force.
- [ ] GATE-TENFOLD-06 (M3) and GATE-TENFOLD-07 (M7) each have a published record, whatever the ratio; the word
      "ten-fold" appears only where the decision rule confirms it.
- [ ] GATE-GAUNT-01 holds at every exit: every gate that passed has a registered negative control it catches.
- [ ] Every result in every release carries its claim product with the three authorization flags false, and
      every report prints its `reality:` line.
- [ ] No sibling was forked or patched locally without a recorded kill decision; the build graph is on one
      toolchain T.
- [ ] Not required: any [M] item; any particular ratio; any calibration against hardware (a recorded `no-claim`
      satisfies the ground clause).

## Test plan

### Unit tests
None of its own; an epic is accepted through its children. The honest-status table in the README is generated
from `registries/gates.toml` and nothing else (checked by `xtask check-generated`).

### End-to-end script and logging
The 24 North-Star scripts `scripts/e2e/ns_01.sh` … `ns_24.sh` (beads NS-01 to NS-24), each logging per the
shared helper of STD-E2E, are the program's end-to-end evidence; `scripts/e2e/northstar_index.sh` (N00-29) runs
those due at a milestone and prints the summary.

### Known answers and negative controls
The thesis arithmetic KA-009 to KA-016 (N00-17), the claim-product known answers KA-027 to KA-032 (N00-27) and
the counterexamples CX-001 to CX-015 (N00-CX) guard the statements made in this bead.

## Considerations

- The three bets on which the thesis turns are BET-13 (ten-fold on the sealed suite), BET-14 (the mechanisms
  need this engine) and BET-17 (rank agreement between rungs). If the composed incumbent ties on a stratum, the
  layer ships over that engine for that stratum and the report says so.
- No milestone is blocked by one failed bet: each has an exit under the fallback's name.
- At most four [M] items are active at once in any milestone.
- Delivery progress is recorded in `docs/delivery/` and in beads, never in the plan.

## References

Plan header, Part 0, §1–§8, §35.1, §38.2, §38.11, §43, §47.1; the repository `AGENTS.md`; the owner's ruling of
2026-11-22 (REV-1 to REV-3); BET-13, BET-14, BET-17; GATE-TENFOLD-06, -07; GATE-GAUNT-01.
