# Changelog

This is a synthesized, agent-facing history of `franken_robotics`. The project has no code yet. Its history is
its planning record: three plan editions and the multi-model research rounds that produced or reviewed them,
all written on a single day.

Scope window: project inception (the owner prompt recorded at 2026-11-21 18:01 EST) through the import
snapshot committed at about 19:55 EST the same day ([`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520)),
plus the README that followed it.

**Evidence and method.** There are no git tags, no GitHub Releases and no issue tracker yet (the base plan
seeds beads BD-001 to BD-125 but none have been created). Git history begins at the import commit, so the
timeline below was reconstructed from:

1. filesystem modification times of every plan and planning artifact (EST, read with `stat`);
2. explicit citations and header fields inside the files (which plan a round targets, which reports a plan cites);
3. the files' own tallies, recounted where they are machine-readable (`MOR_CLAIMS.jsonl`).

A modification time is the *last* write, not the creation time: several files were rewritten in place, which
also resets their birth time. Where a statement rests on inference rather than a timestamp or a citation, it
says so. All links below point at the import commit, so they keep showing the files as they were when this
history was written.

## Unreleased

State at the snapshot, and work visibly in flight:

- **Round 2 ("fable") is still running against base v0.2.** Its idea duel finished scoring between 19:47 and
  19:50. Its modes-of-reasoning run had normalized 94 claims, 93 of them `AUDIT_PENDING`, with empty
  `audit/` and `final/` directories. Its Brenner `deliverables/` and `debates/` directories are empty. An empty
  `planning/fable/blend/` directory appeared at 19:42, which suggests the base plan's planned "competing-plan
  blend" (base §0.6) is next. That is an inference: nothing in the record says what `blend/` is for.
- **Neon v2.0 was rewritten in place at 19:50:43** (1,391 → 1,452 lines). Its version field still reads 2.0,
  and the earlier text is not preserved.
- **No edition has been chosen as canonical.** The README uses base-plan names where the editions disagree.
- **Next steps** stated by the plans: reconcile the editions into one plan, fold in round 2, convert to beads,
  start M0 (base §34, §36; Neon §28–§29; Astra §15).

---

## Version Timeline

There are no tagged versions. The spine is the sequence of plan editions and research rounds, with the commits
that captured them. Times are EST on 2026-11-21.

| Version | Kind | Date | Summary |
|---------|------|------|---------|
| Neon v1.0 | Plan edition (not preserved) | before 17:46 | 1,726 lines (cited by round 1). Target of the round-1 audit, duel and Brenner session |
| Astra study | Research bundle | from about 18:00 | Astra's own reasoning audit, 3 × 3 idea duel and native GPU pilots. The run ID in its archive converts to 18:00:34 EST (inference) |
| Round 1 | Research round | 18:01 – 19:08 | Modes of reasoning, three-model dueling idea wizards and a Brenner session on Neon v1, run as one NTM swarm |
| [Base plan v0.2](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md) | Plan edition | last written 18:36 | "First complete draft plus one internal three-reviewer round". 3,666 lines |
| Round 2 ("fable") | Research round (in progress) | 18:44 – after 19:50 | Reasoning audit, Brenner slate with evidence packs, idea duel, graveyard and artifact passes, all on base v0.2 |
| [Astra edition](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) | Plan edition | 19:24, rewritten 19:37 | Inverse mission design and a sealed 10× acceptance test. 3,222 lines; links the reproduction bundle 8 times |
| [Neon v2.0](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) | Plan edition | 19:24, rewritten 19:50 | "10× Breakthrough Synthesis & Multi-Pass Adversarial Audit Fold-In", superseding Neon v1.0. 1,452 lines |
| [Import snapshot](https://github.com/Dicklesworthstone/franken_robotics/tree/7041f51df6303c26be72ff0b67f83027ee6ab520) | Git commit (no tag, no release) | about 19:55 | First commit: all three editions, `planning/`, the reproduction bundle, LICENSE and `.gitignore` |
| [README](https://github.com/Dicklesworthstone/franken_robotics/tree/cb8d3e27badc32a30cec80c0a616fd7c0d110cd7) | Git commit (no tag, no release) | about 19:55 | Honest status, the three editions compared, the planned design |

**Three parallel tracks, not a version chain.** No plan cites another plan. Neon's "supersedes v1.0" refers
to Neon's own earlier draft, not to base v0.2. The only cross-track awareness in the record is round 2's case
file (evidence row E017), which notes that the Neon and Astra editions exist and declines to treat them as
evidence. Do not infer who wrote an edition from its name: "Neon" is also the name of one of the duel's models
(Gemini 4 Neon), and nothing in the files states authorship.

---

## 1) Base plan v0.2: one mathematical instrument

The base plan defines FrankenRobotics as "a robot proving ground and a robot design studio built as one
mathematical instrument". It is the most complete edition at the level of crates, verbs, gates and the first
working session, and it is the only one round 2 reviews.

### Delivered capability

- Evidence labels (FACT, DESIGN, HYPOTHESIS, TARGET, OPEN), ambition tags ([S], [F], [M]) and an eight-part
  definition of "implemented" (§0.1–§0.2).
- Six claims that are never averaged together: Proof, Known answer, Parity, Fidelity, Reality, Performance (§6).
- Fifteen mathematical constructions, C-01 to C-15, each with a type, a verb, a test and a payoff (§8), and
  a proof-class system (`Proved`, `Certified`, `Tested`, `Estimated`) with theorem cards T-01 to T-14 (§10).
- Seventeen named subsystems (Marrow through Tether, plus Lemma, Gauntlet and Airlock) mapped to 28 `frob-*`
  crates plus `xtask`, with layering rules and a 361 kLoC cap (§11–§26).
- Three tiers (`referee`, `batch-cpu`, `gpu-cuda`/`gpu-metal`), a trust ladder from `provisional` to
  `established`, and replay of a GPU world on the CPU from its address (§14, §23).
- A verb registry generating the `frob` CLI and the MCP tools, with a fixed JSON envelope and exit codes
  (§24, Appendix J).
- Throughput targets with a mandatory disclosure block (§28), 12 bets, 10 admission gates, about 50
  convergence gates and 9 kill gates with written fallbacks (§2, §31).
- Milestones M0–M6, 33 work packages, 125 seed beads and a first implementation session (§34–§37).

### Closed workstreams

- Study of six sibling projects' plans (FrankenSim and its companions, FrankenTextiles, FrankenMining,
  FrankenThreeD, FrankenLean, FrankenSymPy), seven code audits of siblings, and a public research pass (§0.6,
  Appendix H).
- Idea-wizard pass: 30 ideas → best 5 → next 10 (Appendix B).
- One internal round by three fresh-context reviewers (robotics-physics and GPU, mathematics, plan audit). Its
  findings are said to be in Appendix K, which is **absent** from the file.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) imports the plan as last written at 18:36:46.

---

## 2) Round 1: reasoning audit, three-model duel and Brenner session on Neon v1

From 18:01 to 19:08 one NTM swarm reviewed Neon v1 with three model lanes: CC (Claude Opus 5.5), COD (OpenAI
GPT-6.2-Sol, run as Codex) and AGY (Gemini 4 Neon). Its findings fed Neon v2.0 and nothing else.

### Delivered capability

- **Modes-of-reasoning audit.** A neutral case file with evidence rows and six pivotal questions; two raw
  passes (mathematics and synthesis; worlds, silicon and studio); an independent audit by the COD lane.
  Result: 16 claims, **9 partially supported, 6 hypothesis, 1 rejected, 0 verified**. Rejected: C007, which
  treated a log-posynomial (log-sum-exp) as a linear integer problem. Refuted: AGY's claimed contact-matrix
  conditioning bound (κ ≤ 100), with a counterexample at κ = 4,801.
- **Dueling idea wizards.** "90+ raw ideas → 39 formal proposals … 78 independent adversarial scores", two
  steelmen and six cross-slate blind spots (BS-1 to BS-3 from COD, B1 to B3 from CC). Top mean: CC-01
  MorphoJIT, 904.5. Top safety idea: COD's `.fenergy` boundary energy budget, mean 825.0. Mutually killed:
  AGY's FP16 Delassus factorization, mean 332.0. Reviewer means were COD 690, CC 549 and AGY 861; the report
  treats AGY as over-generous and ranks on COD and CC. CC's reactions conceded 22 of about 25 COD catches.
- **Brenner session.** A question of record with three paradoxes (algebra vs speed, the heterogeneous-morphology
  GPU wall, verification vs contact), eight killed hypotheses († K-01 to † K-08, among them full-room 3D
  lattice-Boltzmann at 63.7 TB of VRAM and clairvoyant ∀w ∃u safety filters), crux-first exclusion tests
  XT-P1 to XT-P3 plus GATE-RIVAL-AUDIT, and a disagreement register (resolved DR-01 to DR-11, open OC-01 to
  OC-05).

### Closed workstreams

- [Reasoning case file](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/mor/MOR_CASE.md), [claims](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/mor/MOR_CLAIMS.jsonl) and [report](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/mor/MODES_OF_REASONING_REPORT_AND_ANALYSIS_OF_PROJECT.md) (18:03–18:36).
- [Dueling wizards report](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/duel/DUELING_WIZARDS_REPORT.md) (19:08) with its idea slates, six score files and [CC's reactions](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/duel/WIZARD_REACTIONS_CC.md). Only CC wrote a reactions file.
- [Brenner question of record](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/brenner/intake/question_of_record.md), [synthesis artifact](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/brenner/deliverables/ARTIFACT.md) (18:58) and [disagreement register](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/brenner/distillations/disagreement_register.md) (19:08).

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) imports every round-1 artifact, including the raw and audit passes under `planning/mor/.mor/`.

---

## 3) Neon v2.0: campaign-time quotienting and a 51-crate foundry

Neon v2.0 folds round 1 into its v1 draft. It reframes the project as changing "the unit of computation,
proof, world interaction, and verification", from one frozen robot per training run to a whole design
campaign.

### Delivered capability

- Five claim axes and a non-promotable `ClaimProduct<VerifiedFormal, EvaluatedDesign>`; exports hard-wire
  `execution_authorized`, `hardware_safety_signoff` and `sim_to_real_certified` to `false` (§0.4).
- Twelve bounded contexts, 51 crates (50 of them `forbid(unsafe_code)`) and five binaries: `frob`, `frobd`,
  `frob-mcp`, `frob-tui`, `frob-verify` (§0.5, §22).
- New file formats from the duel: `.fmodule`, `.fquot`, `.fclear`, `.fenergy`, `.ftube`, `.fskill`, `.fmcw`,
  `.fbuild`, `.frobcert`, alongside `.fbot` and `.ftask` (§22.4).
- Sixteen bets, sixteen North-Star scenarios, oracles KA-01 to KA-14, 37 gates, 21 kill gates and a 97-bead
  dependency graph to M6 (§3, §5, §25–§29).
- Appendix E.2 lists five fold-in passes: three v1 audits (mathematics and proofs; GPU and systems; robotics,
  worlds and the bead graph), the cross-model reasoning audit, and the duel plus Brenner session.

### Closed workstreams

- Round 1 (section 2 above), cited from Neon's header and Appendices A, B and E.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) imports Neon as rewritten at 19:50:43.

**Caveats for readers of Neon.** The three v1 audit passes it lists have no artifacts in `planning/`. Its
"Python-verified" cards cite `scratch/verify_math.py` and `scratch/verify_neon_v2.py`, which are not in the
repository. Its headline "delete 90%–99% of futile simulations" is stronger than its own pass bars (XT-P1
passes at 5× and 10×, that is 80% and 90%). The quotient count it cites (735,471 → 24,546) reproduces, but on
a constructed fixture rather than a design campaign. The 512-topology figure is unmeasured. An independent
recheck during this history found the `.fenergy` card's 45.5 ms figure appears to count a 0.10 J reserve
twice (50.5 ms with the reserve), and could not reproduce the 33.5 s thermal cliff from the inputs printed.

---

## 4) Astra edition: inverse mission design and a sealed tenfold test

Astra asks for the set of designs that can do a job, with checkable exclusions and the cheapest deciding
experiment. It defines success as 10× less elapsed time to an independently accepted engineering answer,
measured against the strongest composed rivals on sealed missions.

### Delivered capability

- Users, product boundaries and a release vocabulary of design, world, task and study (§1). Proposed
  `fr-robot-*` packages and a `frobot` CLI and MCP service with six inverse queries (`mission.synthesize`,
  `mission.refute`, `mission.distinguish`, `world.separate`, `skill.transport`, `result.revalidate`) (§3,
  §10, §18).
- Analytic results: the three-tick docking boundary (sensor-error tolerance 1/8 → 5/16; 561 of 561 cells match
  an independent LP comparator; a third gain of 33/32 reverses all 34,590 feasible labels per grid), the
  narrowest stance (3/14 m half-width), and a promotion rule that cannot declare a tie below 86 pairs
  (§13.2, §18.7, §18.10).
- Measured results: a docking-facet screen on the M5 Ultra and both RTX 4090s in which exact host integer
  arithmetic was fastest end to end on both machines (§11.6), and a pusher pilot whose speed gate failed at
  1.345× warm and 1.088× cold (§19.4).
- Probes of sibling code: three errors in FrankenSim's `fs_g1_train::ppo_update` and a planar-drive case
  that loses translation (§5.1, §9.3).
- Work packages W01–W43, milestones M0–M8, risks, and a sealed acceptance design with four strata and
  one-sided 95% lower bounds (§15–§19).

### Closed workstreams

- Astra's own reasoning audit (seven independently audited claims, five verified in scope), a 3 × 3 duel with
  18 external scores (top mean: compositional subsystem contracts, 665 of 1000; "No candidate reached 700 from
  both outside reviewers"), and follow-up checks (§20.4–§20.5).
- [`planning/astra-10x-reproduction.zip`](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/astra-10x-reproduction.zip): 155 entries, last rebuilt at 19:36:48 with a new `followups/` folder. `python3 -B verify.py --all` and `python3 -B verify_negative.py` re-check it.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) imports the edition as rewritten at 19:37:52 and the matching bundle.

---

## 5) Round 2 ("fable"): an isolated review of base v0.2, in progress

Round 2 reviews the base plan and is deliberately isolated from the other tracks: its study prompt says "Do
NOT read anything under planning/duel/, planning/mor/, planning/brenner/, or the files ending in __NEON.md or
__ASTRA.md". None of its findings has been folded into any plan, and most of them are unaudited.

### Delivered capability

- **Reasoning audit passes** (18:53–19:01). P01: policy training is about 93% of wall-clock, so a faster
  simulator alone "is capped below 8× on training and at 2–5× on a whole iteration". P02: "of the sixteen
  capabilities, none is cleanly out of reach for the incumbent set as a whole". P03: three transferable
  structures from other domains. P04: one scene per world is impossible (8,192 × 75 MB = 614 GB); reject
  half-precision factorization. P05: a backcast pre-mortem proposing a rank-fidelity gate. Claims were being
  normalized at the snapshot (94, 93 pending audit).
- **Brenner slate and evidence.** Hypotheses H-01 to H-24 and 23 evidence packs (19:15–19:23). Recommended
  states: 17 narrowed, 4 refuted (supercompiled kernels, incremental evaluation, tensor-core mixed precision,
  living multiphysics), 1 deferred and 1 active. The active one, H-19, is "a thin layer over an incumbent gets
  most of it", marked "ACTIVE, and the strongest rival". H-18 has no pack.
- **Idea duel.** Three slates of 30 ideas winnowed to 5 (19:09–19:20) and three cross-scoring files
  (19:47–19:50). The Codex scorer ranks CC #4, paired multilevel estimation across tiers, highest (891). The
  Claude scorer reports that no idea reached 850.
- **Graveyard harvest** (19:25): thirteen hotspots and three top cards (memoize everything downstream of
  `ModelId`; slot-level episode dispatch in `frobd`; paired multi-fidelity estimates with exact importance
  weights). "No gain here is measured."
- **Artifact compilation** (19:29): eleven decision cores, eight method families, and two recommendation
  contracts for the trust ladder. The top expected value (25.0) goes to a promote/reject e-process pair.

### Closed workstreams

- [Round-2 case file](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/mor/MOR_CASE.md), [hypothesis slate](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/brenner/hypothesis_slate.md), [graveyard harvest](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/graveyard/GRAVEYARD_HARVEST.md) and [artifact compilation](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/artifact/ARTIFACT_COMPILATION.md).
- Still open: the claim audit and final report, the Brenner deliverables (its question of record cites a
  `HANDBACK.md` that does not exist yet), the duel's reactions and report, and the blend.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) captures round 2 mid-flight, at about 19:55.

---

## 6) Repository bootstrap

The project became a public git repository. The base plan's OPEN-PROG-1 ("public or private repo and license
file", default private) blocked the first push. The owner chose public, under the suite's license.

### Delivered capability

- `LICENSE`: MIT with the OpenAI/Anthropic rider, identical to the sibling Franken repositories.
- `.gitignore` covering the planned layout (Rust workspaces and annexes, compiled GPU code and kernel caches,
  Lean build output, the viewer), run and telemetry output, private scans and walkthrough video (base §29.3),
  Python caches from the reproduction bundle, and agent session state (`.ntm/`, `.ee/` at any depth).
- `README.md` in the style of FrankenTextiles: an honest-status table, the three editions compared, and the
  planned design with every throughput figure labelled as a target.
- This changelog.

### Closed workstreams

- Base OPEN-PROG-1, resolved: public repository, suite license.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) Import the planning record, LICENSE and `.gitignore`.
- [`cb8d3e2`](https://github.com/Dicklesworthstone/franken_robotics/commit/cb8d3e27badc32a30cec80c0a616fd7c0d110cd7) Add the README.

---

## Notes for Agents

- **Start with the README's status table**, then the edition that matches your task (see its Reading paths).
  Do not treat any one edition as canonical until a reconciled plan says so.
- **What has been folded where.** Round 1 is folded into Neon v2.0 only. The Astra study is folded into Astra
  only. Round 2 is folded into nothing. The base plan is cited by round 2 but by no other edition.
- **Missing artifacts that the plans cite:** base Appendix K (review log), Neon's three v1 audit records and its
  `scratch/verify_*.py` scripts, and round 2's `HANDBACK.md`.
- **Internal inconsistencies already found**, so you do not rediscover them: base has several cross-references
  one section off around §22 and an Appendix K that is cited but absent; Neon's preflight latency is "<200 ms"
  in one place and up to 2 s in another, and its counterfactual replay is "<1 s" against "<1.5 s"; Astra's
  §13.2 mentions "150 paired cases" next to 156 rows. Astra's workload IDs P1–P6 collide with its planning
  experiments P01–P05.
- **Absolute local paths.** Several plans and planning files link to `file:///Users/...` paths, which do not
  resolve on GitHub. Use the relative paths in this repository instead.
- **Times** in this file are EST modification times. Treat them as "last written at", never "first created at".
- **Maintaining this file.** Add a timeline row for each new plan edition, reconciliation or research round, with
  the commit that lands it. Once the first git tag exists, link it from the timeline and keep the planning rows
  as prehistory.
