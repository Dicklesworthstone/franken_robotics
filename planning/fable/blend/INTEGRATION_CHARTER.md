# Integration charter for the hybrid FrankenRobotics plan (version 1.0)

Every drafter reads this file first and follows it exactly. It fixes the outline, the names, the identifiers, the
binding decisions and the style, so that sections written in parallel fit together.

## 1. What we are producing

One document: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md`, version 1.0.
It replaces version 0.2 (the BASE plan, currently at that path) by absorbing everything good from two other plans
of the same project and from several idea and audit exercises. The owner asked for a **maximalist** hybrid: more
mechanisms, more ambitious features, more functionality, a much longer and more detailed plan, "the most
comprehensive, useful, compelling, accretive and radically innovative project of its kind". The owner also asked
that the MCP surface use his `fastmcp_rust` project, and that the alien-graveyard and alien-artifact-coding skills
be applied.

Maximalist does not mean credulous. The rule is: **include every mechanism that is real, at the tier its evidence
supports, with a falsifier and a kill gate; include narrowed versions of mechanisms whose strong form was
refuted; never include a number as a result.** A reader must be able to tell the spine from the frontier.

Drafters write final plan text into part files. The lead assembles them.

## 2. Inputs (read what your sections need)

| Input | Path |
|---|---|
| BASE plan v0.2 (carry its text forward unless a decision below changes it) | `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` |
| Second plan, full text | `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md` |
| Digest of the second plan (26k words; §12 inverse mission design, §13 ten-fold test, A–E assessments) | `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/ASTRA_DIGEST.md` |
| Third plan, full text (very long lines) | `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` |
| Digest of the third plan (with "(D) Doubtful or wrong: do not inherit") | `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/NEON_DIGEST.md` |
| Idea catalog: 112 deduplicated ideas from three multi-model exercises, with cross-scores, kills, executed checks, corrections of fact | `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/IDEA_CATALOG.md` |
| Three-model idea duel on the BASE plan (ideas, cross-scores, reactions if present) | `/Users/jemanuel/projects/franken_robotics/planning/fable/duel/WIZARD_*.md` |
| Audited reasoning passes: findings and plan defects | `/Users/jemanuel/projects/franken_robotics/planning/fable/mor/.mor/run-f1/audit/AUD01-findings.md`, `AUD02-findings.md`; raw passes in `../raw/` |
| Hypothesis verdicts (24 hypotheses, states and surviving numbers) | `/Users/jemanuel/projects/franken_robotics/planning/fable/brenner/deliverables/VERDICT_REGISTER.md`; evidence packs in `../evidence/packs/` |
| Graveyard harvest (18 primitive matches, three recommendation cards) | `/Users/jemanuel/projects/franken_robotics/planning/fable/graveyard/GRAVEYARD_HARVEST.md` |
| Decision-core compilation (eight mathematical families, artifacts, obligations, cards) | `/Users/jemanuel/projects/franken_robotics/planning/fable/artifact/ARTIFACT_COMPILATION.md` |
| Sibling code, to check a fact before stating it | `/Users/jemanuel/projects/{frankensim,frankentorch,franken_numpy,frankensympy,franken_lean,fastmcp_rust,franken_threed,asupersync,frankensqlite,frankenscipy,frankentui}` |

Do not modify any input. Do not read or write anything else under `planning/`.

## 3. Style rules (binding)

1. **No attribution.** Never write ASTRA, NEON, BASE, Fable, Claude, Codex, Gemini, GPT, "another plan", "the
   duel", or a duel score, in plan text. Ideas are simply part of the plan. Process provenance lives only in §0.6
   and Appendix K, written by the lead.
2. **Register.** A specification, not a pitch. Plain declarative sentences. No exclamation marks, no capitals for
   emphasis, no "10×" slogans, no praise of the plan. Write "ten-fold" only where the ten-fold test is meant.
3. **Labels.** Every factual or numeric statement is FACT (with a path or URL), DESIGN (with its reason),
   HYPOTHESIS (with the test that would settle it), TARGET (a number we intend, not a measurement) or OPEN (owner,
   blocked gate, fail-closed default). A number without a label is a defect. External numbers taken from the
   digests or audits are written as FACT with the source and the phrase "to be re-read from the primary source
   before a gate cites it" in the source register, not repeated in every sentence.
4. **Tiers.** [S] Solid, [F] Frontier, [M] Moonshot on every mechanism. Nothing [M] is on the critical path. Every
   [F] and [M] item names a paper-scale pilot with a falsifier that must run before its work package is funded,
   and states the ceiling of the benefit before any speed test.
5. **Every mechanism has**: what it is (types, data layout, algorithm), which crate owns it, the verb that exposes
   it, a known-answer or negative-control test, a gate with a numeric threshold, and for [F]/[M] a kill gate with
   a fallback named in advance.
6. **Counterexamples are first-class.** Where a plausible shortcut is wrong, give the smallest hand-checkable case
   that breaks it. These seed the negative-control registry.
7. **Formatting.** Markdown. Wrap prose at about 118 characters. Tables are fine and may have long rows. Mathematics
   in Unicode, never LaTeX (no `$`, no backslash commands). Code blocks for schemas, type sketches and transcripts.
   Rust sketches are labeled DESIGN and fix intent, not signatures.
8. **Chapter template** for subsystem chapters: what it owns; decisions (ADR-…) with reasons; design; refusals;
   invariants and proof obligations; known answers; gates and kill gates; open questions. Keep the BASE chapter's
   content and add to it.
9. **Carry-over.** Where a section exists in the BASE plan, start from its text and keep its identifiers. Change it
   only as the decisions below require, to fix a listed defect, or to add material. Do not drop BASE content.
10. **Cross-references** use the final section numbers in §5 below (for example "§16.5"), subsystem names and
    stable identifiers. If you need to refer to something another drafter defines, refer to the section and the
    concept, not to an identifier you invent on their behalf.
11. **Length.** Write as much as the material needs. Targets per part are in §6; exceeding them is fine if nothing
    is padding.
12. **End of file.** Finish your part file with a section titled `<!-- LEDGER -->` followed by plain lists of: new
    identifiers you defined (gates, kill gates, bets if any, invariants, proof obligations, open questions,
    refusal codes, formats, verbs, crates touched), and "needs from other parts". The lead strips this before
    assembly.

## 4. Binding decisions for the hybrid

**G1 Names.** Crate prefix `frob-`. Binaries `frob` (CLI), `frobd` (daemon), `frob-mcp` (MCP server),
`frob-verify` (offline certificate auditor, standard-library only), `frob-tui` (terminal dashboard). Subsystem
names in §5. No other naming scheme appears.

**G2 FrankenSim posture.** Consume by path dependency behind the airlock crate `frob-fsim`, pinned by commit, core
toolchain follows FrankenSim's. Fallbacks in order: (a) a pinned FrankenSim worker process per device exchanging
whole jobs; (b) lifting a minimal referee set with provenance. Sibling repos are read-only; changes are filed as
asks upstream and never scheduled as local beads. Record as FACT-to-recheck the reported defects in foundations
(three errors in `fs_g1_train::ppo_update`, a near-straight integration defect in `fs-mbd::planar_drive`, a stride
convention trap between franken_numpy and frankentorch, a fixed-witness assumption in the sustained contact step):
each becomes an admission test and an upstream ask. Record the `fs-qty` nightly coupling as a named risk tested at
INT-TOOLCHAIN-01, with a small stable `frob-units` crate as the documented escape hatch.

**G3 Physics by tier.** Three tiers as in BASE. The referee and a strict `batch-cpu` profile use a transactional,
event-located step (commit fence, retries that restore all state, a refinement-consistent noise law, witness
migration). Throughput tiers keep the fixed-step lockstep convex compliant solve. Contact gains a four-dimensional
cone (normal, two tangential, spin) and joint-level stiction and backlash rows. A hard-contact complementarity law
is an optional batch law [F] and the written specification of the referee. Model-form rank checks compare laws.
Dense per-world mass matrix for 18–60 degrees of freedom; a tree-operator route for larger mechanisms [F].

**G4 Determinism.** Classes D0–D4 as in BASE. Add: a decision-level equivalence test under D3 (do task decisions
agree, not only state errors); an exact-integer kernel class for strict predicates and digests; guarded single
precision with an exact fallback; a test that a world's trajectory does not depend on the batch it ran in; paired
statistics are refused under an uncontrolled determinism class; a nondeterministic training profile is allowed
only as labeled D4 whose results must be re-evaluated on a stricter profile. No fast-math, no fused multiply-add,
no built-in transcendentals in batched kernels, floating-point atomics banned.

**G5 GPU.** One shared C-dialect kernel source with per-backend preludes plus derived arithmetic; `cudarc` with
NVRTC on CUDA; FrankenTorch's `ft-kernel-metal` gateway on Metal; annex workspaces. Add: per-world **model
variants** inside padded capacities and size buckets (many different robots in one batch); design-family buckets
with the speedup bound 1/(1 − f + f/r + h); a placement planner with requested-versus-actual receipts per stage;
strict-backend errors before scoring; an arena lease and a binary kernel cache; a hardware fingerprint gate; a
division of labour between the machines (the M5 Ultra holds the world bank and large scenes, proof and symbolic
work, replay, the viewer and large-memory jobs; the two RTX 4090 train; only seeds, weights and receipts cross).
Per-design kernel specialization is for survivors only and only if our own profile shows the recursion share
justifies it (the incumbent trace caps it near 1.2×). Tensor-core half precision is rejected.

**G6 Statistics.** Two declared study types, never mixed inside one study: anytime-valid e-process studies, and
fixed-n exact studies with a pre-analysis of which verdicts are reachable at that n. A **blinded qualification
lane**: discovery cannot read confirmation worlds; confirmation runs on fresh sealed world clusters with
pre-committed requests and error budget α_i = α / [i·(i+1)] for the i-th submission, so error control holds across
an agent's adaptive retries. Rank-agreement between rungs is measured, with consequences. Screening is audited by
sampling discarded strata and by planting a known winner.

**G7 Gym semantics.** Explicit reset by default with the final observation preserved; termination and truncation
distinguished for bootstrap targets; the likelihood of the applied action; physical discounting; time-aware
rewards. Masked automatic reset exists as a labeled mode that still returns final observations. The Rust `VecEnv`
is primary; an optional Gymnasium-compatible adapter lives under `bindings/` so rival learning stacks can drive
this environment in the ten-fold test.

**G8 Time.** Rational periods with phase accumulators; seven distinct instants (sample, exposure, timestamp,
arrival, decision, command, actuation) with a boundary ordering rule; packet queues are part of every checkpoint.
The BASE rule "integer ratios of the physics tick" is withdrawn.

**G9 Proof classes.** Proved, Certified, Tested, Estimated as in BASE. Add: family certificates for mission
answers in a portable bundle format checked by the independent `frob-verify`; a claim-product type so the weakest
input wins by construction and nothing can be converted into a proof; machine-readable no-authorization flags on
every exported artifact; kernel reflection as a Frontier route to promote selected instance certificates. The
word "verified" is not used for results.

**G10 Claims.** Seven, never conflated: Proof, Known answer, Parity, Fidelity, Design comparison, Reality,
Performance.

**G11 Scope.** Under the maximalist brief the following are admitted as [F] or [M], behind flags and off the
critical path, each with known answers and a kill gate: Cosserat rods and cables; cloth; fluid films advected by
the robot, slosh; coupled multiphysics islands; a rotor port for flight; tactile hands; firmware export; lifecycle
compression; facility co-design and multi-robot handoff. Their Solid counterparts (film-friction law, rigid
proxies) are in the core.

**G12 The ten-fold test.** Success is defined as a ratio of restricted mean times to an independently accepted
answer against the strongest composed rival on a sealed suite, with cold and warm runs separated, a crossover
reuse count, a phase ledger, a neutral adjudicator, one-false-certificate disqualification, and written kill
rules, including: if a short robust program over an incumbent engine ties, say so and ship the layer. A partial
gate at M3 and the tournament at M7. Engine throughput is a parity requirement, not the source of the ten-fold.

**G13 Evidence.** The verdict register binds: narrowed mechanisms are written with their surviving claims and
numbers; refuted ones are not built in their strong form. Withdrawn differentiators (AUD02) are not claimed.
Corrected numbers (AUD01, AUD02) replace BASE numbers.

**G14 MCP.** `frob-mcp` is built on `fastmcp_rust` (inspect it for the real API). Tools are generated from the
verb registry in clusters of at most seven per task; long tools take a bounded synchronous wait
(`sync_wait_ms`) and otherwise return a job handle; catalogs, stills, reports and schemas are MCP resources; three
failure classes; atomic idempotency binding; pagination pinned to a committed frontier; integers above 2⁵³ travel
as strings.

**G15 Milestones.** M0 spine and airlock; M1 CPU referee studio plus the first mission slice on the CPU; M2 batch,
GPU backends, mixed batches; M3 places, tasks, adversaries, qualification, partial ten-fold gate; M4 amortized
control, co-design, mission queries at scale; M5 proofs, reality, manufacture; M6 frontier physics; M7 the
ten-fold tournament; M8 frontier delivery.

**G16 Do not inherit.** Everything in NEON_DIGEST "(D) Doubtful or wrong": the lift posture; unverifiable
"verified" numbers; "proved once at M1"; headline multipliers; tensor cores for contact; GPU continuation with
kernel-checked certificates; symplectic claims beyond a free body; curvature as a mecanum test; quaternionic
irreducibles; refusing simultaneous impacts; ornamental labels (decorated cospan, sub-PROP, Futamura,
Pearl-causal, Wasserstein, cellular sheaf, conformal geometric algebra) on ordinary mechanisms; gates that need
hardware a simulator lacks; fast-math switches; latency slogans; unadmitted third-party tools. Keep each
underlying mechanism where it is sound and name it plainly.

## 5. Final outline, subsystem names and owners

| § | Title | Subsystem | Carries BASE § | Part file |
|---|---|---|---|---|
| Header, Contents, Part 0 (§0.1–0.8) | Document control | — | Part 0 | `P00` |
| 1 | What exists, what is missing, what this is | — | 1 | `P00` |
| 2 | The ten-fold thesis | — | new | `P00` |
| 3 | Twelve principles | — | 0.4 list | `P00` |
| 4 | Bets, each with its falsifier | — | 2 | `P00` |
| 5 | Readers | — | 3 | `P00` |
| 6 | Goals, non-goals, anti-goals | — | 4 | `P00` |
| 7 | North-Star scenarios | — | 5 | `P00` |
| 8 | The seven claims and the claim product | — | 6 | `P00` |
| 9 | Doctrine | — | 7 | `P01` |
| 10 | The constructions (C-01 onward) | — | 8 | `P01` |
| 11 | Conventions | — | 9 | `P01` |
| 12 | Lemma: proofs, certificates, the offline auditor | Lemma | 10 | `P01` |
| 13 | Marrow: the algebraic core | Marrow | 11 | `P02` |
| 14 | Skeleton: design language, modules, archetypes, preflight | Skeleton | 12 | `P02` |
| 15 | Sinew: actuators, transmissions, power | Sinew | 13 | `P02` |
| 16 | Kinesis: dynamics on three tiers | Kinesis | 14 | `P03` |
| 17 | Phylum: families of bodies in one batch | Phylum | new | `P03` |
| 18 | Traction: collision, contact, wheels, tracks, terrain | Traction | 15 | `P03` |
| 19 | Habitat: scenes, living worlds, clearance, retrofits | Habitat | 16 | `P04` |
| 20 | Sensorium: sensors, clocks, observability | Sensorium | 17 | `P04` |
| 21 | Spark: GPU backends and the kernel toolchain | Spark | 20 | `P05` |
| 22 | Reflex: estimation, control, gaits, skills, shields | Reflex | 18 | `P06` |
| 23 | Cortex: the gym, learning, family policies, inheritance | Cortex | 19 | `P06` |
| 24 | Arena: tasks, requirements, fixtures, metrics, qualification | Arena | 21 | `P08` |
| 25 | Augur: mission queries and inverse design | Augur | new | `P07` |
| 26 | Nemesis: adversarial worlds and rare failures | Nemesis | new | `P08` |
| 27 | Autopsy: counterfactual debugging | Autopsy | new | `P08` |
| 28 | Atelier: the studio loop, analyses, comparison, lineage, cascade, decision cores | Atelier | 22 | `P09` |
| 29 | Chronicle: telemetry, replay, evidence | Chronicle | 23 | `P09` |
| 30 | Ground: calibration against reality | Ground | 30.7 | `P10` |
| 31 | Foundry: structure, manufacture, lifecycle | Foundry | new | `P10` |
| 32 | Tether: CLI, daemon, MCP, dashboard, viewer | Tether | 24 | `P11` |
| 33 | Suite consumption | — | 25 | `P12` |
| 34 | Workspace, crates, formats | — | 26 | `P12` |
| 35 | Numerics and determinism | — | 27 | `P05` |
| 36 | The performance program | — | 28 | `P05` |
| 37 | Safety, security, privacy | — | 29 | `P11` |
| 38 | Gauntlet: the verification program | Gauntlet | 30 | `P13` |
| 39 | The ten-fold test | — | new | `P13` |
| 40 | The gate register | — | 31 | `P13` |
| 41 | Risks and the pre-mortem | — | 32 | `P14` |
| 42 | Open questions | — | 33 | `P14` |
| 43 | Milestones M0–M8 | — | 34 | `P14` |
| 44 | Work packages | — | 35 | `P14` |
| 45 | The beads | — | 36 | `P15` |
| 46 | The first implementation session | — | 37 | `P14` |
| 47 | Definition of done and the release rubric | — | 38 | `P14` |
| App. A–N | Glossary; idea passes; EV matrix and killed-idea register; mathematics compendium; worked cards; upstream asks; robots, scenes, licences, hazard lists; source register; type sketches; verb registry; review log; worked artifacts; known-answer suite; counterexample catalogue | — | A–J | `P16a`, `P16b` |

Part titles: Part 0 Document control; Part I Thesis (§1–8); Part II The mathematical spine (§9–12); Part III The
body (§13–21); Part IV The mind (§22–28); Part V The record and the world (§29–32); Part VI Substrate and
engineering (§33–37); Part VII Verification, the ten-fold test, gates and risks (§38–42); Part VIII Delivery
(§43–47).

Part files are written to `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/parts/<name>.md`
(for example `P03.md`). Start each chapter with its `## N. Title` heading exactly as in the table (Part headings
`# PART …` are added by the lead).

## 6. Crates (final names)

Layer 0: `frob-core`, `frob-fsim`. Layer 1: `frob-lie`, `frob-algebra`, `frob-kir`. Layer 2: `frob-design`,
`frob-model`, `frob-io`, `frob-actuate`, `frob-scene`. Layer 3: `frob-kernels`, `frob-contact`, `frob-dyn`,
`frob-phylum`, `frob-sense`. Layer 4: `frob-control`, `frob-gait`, `frob-learn`, `frob-analyze`, `frob-spec`,
`frob-arena`, `frob-opt`. Layer 5: `frob-mission`, `frob-synth`, `frob-nemesis`, `frob-autopsy`, `frob-ground`,
`frob-foundry`, `frob-lifecycle`, `frob-shield`. Layer 6: `frob-chronicle`, `frob-lemma`, `frob-cert`,
`frob-verbs`. Layer 7: `frob-cli`, `frob-tui`, `frob-conformance`, `xtask`. Annexes: `spark/frob-spark-cuda`,
`spark/frob-spark-metal`, `surface/frob-mcp`, `derive/frob-derive`, `bindings/frob-gymnasium`, `viewer/`,
`lean/`. `frob-units` exists only if INT-TOOLCHAIN-01 forces it.

## 7. Formats (extensions are fixed; each has a versioned schema name `frob.<name>/1`)

`.fbot` design (canonical JSON; an S-expression view is lossless and optional); `.fmodule` reusable module with a
port contract; `.fcard` actuator, interface and material cards; `.fscene` scene pack manifest; `.fclear` clearance
complex; `.ftask` task (terms, requirements, randomization); `.fmission` mission query; `.fenergy` energy budget;
`.fskill` skill artifact; `.fmcw` minimal counterexample world; `.frun` episode and telemetry chunks; `.fbuild`
build packet; `.frobcert` certificate bundle.

## 8. Identifier rules

Existing BASE identifiers keep their meaning (BET-01–12, GOAL-01–20, NS-01–12, C-01–15, T-01–14, A-01–16,
GATE-*, KILL-*, INT-*, OPEN-*, RISK-01–20, WP-*, BD-001–125, GB-01–07, ADR-*, INV-*, PO-*, L-1–8, D-1–8). New
identifiers continue the series: gates as `GATE-<SUBSYS>-NN` with the next free number in that subsystem (new
subsystems: AUGUR, PHYLUM, NEMESIS, AUTOPSY, GROUND, FOUNDRY, SHIELD, GAIT, TENFOLD, QUAL); kill gates
`KILL-<NAME>`; open questions `OPEN-<SUBSYS>-N`; refusal codes `FROB-REF-nnn` in the family ranges of BASE §27.5;
formats `FMT-<NAME>`; verbs as `cluster.verb`. Constructions C-16 onward are defined only in `P01`; bets, goals
and North-Star scenarios only in `P00`; known answers get `KA-nn` numbers only in `P13` (other parts list their
known answers in a table without numbers); beads only in `P15`; work packages, milestones, risks and open
questions are compiled in `P14` (other parts list their open questions locally with provisional names
`OPEN-<SUBSYS>-N`, which `P14` adopts).

## 9. Numbers to use (FACT, each to be re-read from its primary source before a gate cites it)

- Incumbent training: a quadruped joystick policy trains in about 410 s on one RTX 4090 (MuJoCo Playground,
  figure read by eye) and about 5 minutes on two; humanoid policies in under 30 minutes. Published agent-driven
  design loops spend about an hour of reinforcement learning per round.
- Incumbent throughput: about 2.55 M humanoid (G1) world-steps/s at 8,192 worlds on an RTX 6000 Ada (MuJoCo Warp
  nightly benchmark). A mixed-topology batch in an incumbent runs at 0.86 of its homogeneous rate (86 k samples/s).
- Incumbent features that exist: opt-in bitwise run-to-run determinism on the same GPU (Newton 1.8, Isaac Lab
  3.0); mixed topologies in one batch (ManiSkill3, two Newton solvers); structural model editing; sampling-based
  control with no training (about 30 samples on a CPU controls a real quadruped); electrical and thermal actuator
  models (MuJoCo, MuJoCo Warp 4.0.1); a Metal path (Genesis); interior generators for homes, kitchens, retail and
  warehouses; MCP access to a simulator.
- Amdahl: an infinitely fast environment step speeds training by at most about 7.8× on one incumbent stack and
  about 1.4× on another; the BASE design cycle gives about 3.0–3.5× on quadruped briefs and about 15× on humanoid
  briefs if a baseline controller can replace training, which is untested.
- Hardware: Apple M5 Ultra with 36 CPU cores, an 80-core GPU, 256 GB unified memory, a measured Metal working-set
  limit of about 222 GiB and a largest single buffer of about 167 GiB; two RTX 4090 with 24 GB each and no link;
  an RTX 4090 step is memory-bandwidth-bound at these state sizes.
- Scale: per-world state about 20–53 kB (quadruped) and 56–180 kB (humanoid); a 2 cm home scene is tens of
  megabytes and a 2 cm warehouse several gigabytes; a CPU referee can shadow on the order of 0.1–2% of GPU
  world-seconds; half precision fails on contact Hessians; pretraining at 10⁹–10¹⁰ steps costs about 1–30
  GPU-hours and at 10¹² steps over a thousand.
- BASE defects to fix: §28.1 mixed world-steps and env-steps (400 k env-steps/s at decimation 10 implies 4.0 M
  world-steps/s against a 2.0 M target); training target 15 minutes was slower than the published 410 s on the
  same card; humanoid and `batch-cpu` targets were below published incumbent figures; "six hours" and "hours" in
  the narrative overstate incumbent loops; §9 and Appendix D disagreed on the coadjoint sign convention; several
  "fossil" rows and novelty claims describe incumbents wrongly (see AUD02's withdraw list).

## 10. What each mechanism's tier is (summary; details in the inputs)

Solid: Lie-typed core; Cayley updates; inertia cone; three tiers and the trust ladder; convex compliant contact;
static analyses; baseline estimator and controller; energy ledger; anytime-valid and fixed-n studies; blinded
qualification; memo graph; replay and forks; interventional autopsy with a stability report; bounded-weight
rare-event estimation; requirement language as acceptance metric; film-friction law and rigid proxies; per-world
parameter variants; clock contract; correct gym semantics; crash-consistent job protocol; scene acceptance.
Frontier: mixed-topology batches; family policies and inheritance; rank-agreement ladder allocation; mission
queries with inner and outer design sets; region certificates on small sub-boxes; clearance complexes; minimal
counterexample worlds; gait orbits with continuation; trajectory optimization with design variables; bounded
exact linkage synthesis; equivariant policies under the observation-preserving subgroup; energy-budget safety
tier and runtime shields; derived kernels and instance certificates; variational articulated integrator;
calibration and conformal reality bands; topology optimization, fatigue, tolerance allocation and build packets;
retrofit catalog; sampled always-on referee; hard-contact batch law.
Moonshot: geometric-algebra tree policies; kernel reflection; continuation-based synthesis; coupled multiphysics
islands; Cosserat rods, cloth, slosh; rotor port; tactile hands; firmware export; lifecycle compression; facility
co-design and multi-robot handoff; skill transport with learned residuals; study-quadric synthesis.

## 11. Parts and what each must contain (see your prompt for your assignment)

Each drafter's prompt names its sections and the inputs most relevant to them. Phase 1 parts (P00–P12) are
written in parallel and must not depend on each other's wording. Phase 2 parts (P13–P16) read the Phase 1 part
files.
