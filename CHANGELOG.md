# Changelog

This is a synthesized, agent-facing history of `franken_robotics`. The project has no code yet. Its history is
its planning record: three plan editions and the multi-model research rounds that produced or reviewed them,
all written on a single day.

Scope window: project inception (the owner prompt recorded at 2026-11-21 18:01 EST) through the README revision
committed at 22:05 EST the same day ([`6c8e158`](https://github.com/Dicklesworthstone/franken_robotics/commit/6c8e158b4cadf6a47e2de9c98b250d0d531ba40a)),
plus the changelog update that followed it.

**Evidence and method.** There are no git tags, no GitHub Releases and no issue tracker yet (the base plan
seeds beads BD-001 to BD-125 but none have been created). Git history begins at the import commit, so the
timeline below was reconstructed from:

1. filesystem modification times of every plan and planning artifact (EST, read with `stat`);
2. explicit citations and header fields inside the files (which plan a round targets, which reports a plan cites);
3. the files' own tallies, recounted where they are machine-readable (`MOR_CLAIMS.jsonl`).

A modification time is the *last* write, not the creation time: several files were rewritten in place, which
also resets their birth time. Where a statement rests on inference rather than a timestamp or a citation, it
says so. Links in a section point at the commit that captured its subject, so they keep showing the files as they
were. The record is still being written: several agents work in one checkout, revise files in place and commit under
the same git identity, so anything written after 22:05 is outside this history.

## Unreleased

State at the snapshot, and work visibly in flight:

- **A hybrid v1.0 is being assembled** ([section 6](#6-the-v10-hybrid-integration-charter-and-phase-1-drafts)). Its
  charter says it will replace base v0.2 at the same path. Phase 1 (§0–§37) is drafted, reconciled once and reviewed;
  neither review is applied. Of Phase 2, §39 to §47 and Appendices F to J, L and O are drafted; §38 and Appendices A to
  E, K, M and N are not. Nothing is assembled. The file at the base path is still v0.2.
- **The Astra and Neon tracks are active again.** Astra was rewritten at 21:26 and 21:43
  ([section 4](#4-astra-edition-inverse-mission-design-and-a-sealed-tenfold-test)) and Neon at 21:51
  ([section 3](#3-neon-v20-campaign-time-quotienting-and-a-51-crate-foundry)).
- **16 of the 24 round-2 verdicts are contested** by at least one cross-family challenger and none is adjudicated
  ([section 5](#5-round-2-fable-an-isolated-review-of-base-v02)). The Phase 1 drafts bind to the register as written
  and do not mention the challenges.
- **Next steps**: apply the two reviews and the completeness audit, write the rest of Phase 2, assemble v1.0,
  adjudicate the contested verdicts, convert to beads, start M0 (charter §5, §11; base §34, §36).

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
| Round 2 ("fable") | Research round | 18:44 – 20:52 | Reasoning audit, Brenner slate with evidence packs and cross-family challenges, idea duel, graveyard and artifact passes, all on base v0.2 |
| [Astra edition](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) | Plan edition | 19:24, rewritten 19:37, 21:26 and 21:43 | Inverse mission design and a sealed 10× acceptance test. The link shows the 3,222-line text; the 3,424-line text is at [`4d99e2a`](https://github.com/Dicklesworthstone/franken_robotics/blob/4d99e2a23f6bdd1d92f4f1497000e9245178343f/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md) |
| [Neon v2.0](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) | Plan edition | 19:24, rewritten 19:50, 20:08 and 21:51 | "10× Breakthrough Synthesis & Multi-Pass Adversarial Audit Fold-In", superseding Neon v1.0. The link shows the 1,452-line text; later texts are at [`d18b5a1`](https://github.com/Dicklesworthstone/franken_robotics/blob/d18b5a1f083fa76a4c035a26840ad2b0177a44f7/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) (1,654 lines) and [`1f5703d`](https://github.com/Dicklesworthstone/franken_robotics/blob/1f5703d861eb042580e22cb45b9ee60558711597/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md) (1,694 lines) |
| [Import snapshot](https://github.com/Dicklesworthstone/franken_robotics/tree/7041f51df6303c26be72ff0b67f83027ee6ab520) | Git commit (no tag, no release) | about 19:55 | First commit: all three editions, `planning/`, the reproduction bundle, LICENSE and `.gitignore` |
| [README](https://github.com/Dicklesworthstone/franken_robotics/tree/cb8d3e27badc32a30cec80c0a616fd7c0d110cd7) | Git commit (no tag, no release) | about 19:55 | Honest status, the three editions compared, the planned design |
| [Changelog](https://github.com/Dicklesworthstone/franken_robotics/tree/acb809ce4ab3c437c0d0b2b9f99caea64071ed8b) | Git commit (no tag, no release) | 19:57 | First version of this file |
| [Neon v2.0, 20:08 text](https://github.com/Dicklesworthstone/franken_robotics/tree/d18b5a1f083fa76a4c035a26840ad2b0177a44f7) | Git commit (no tag, no release) | about 21:20 | Neon's 20:08 rewrite, the round-1 record synced to it, and its consistency verifier |
| [Round 2 complete](https://github.com/Dicklesworthstone/franken_robotics/tree/e88398a5caef7b539712d41b97e792809decdb09) | Git commit (no tag, no release) | about 21:20 | Audited claims, 24 verdicts, two challenges, handback, duel report and reactions |
| [v1.0 drafts](https://github.com/Dicklesworthstone/franken_robotics/tree/3996e58d7d7f6238661caf05043f939080decc4e) | Git commit (no tag, no release) | 21:20 | Charter, digests, idea catalog and Phase 1 parts P00 to P12, as written before reconciliation |
| [v1.0 parts reconciled](https://github.com/Dicklesworthstone/franken_robotics/tree/db95b163d47cc23d98dfd1017ea2d4c26e04e796) | Git commit (no tag, no release) | 21:25 | Eight parts edited, Phase 2 brief, inventories, refusal registry |
| [Astra bundle](https://github.com/Dicklesworthstone/franken_robotics/tree/a635b911505c555b02494b232a9d76985db152f7) and [Astra rewrite](https://github.com/Dicklesworthstone/franken_robotics/tree/5afef55e5a9404d945441142c27df71c1f55cae0) | Git commits (no tag, no release) | 21:26 – 21:27 | 161-file bundle; 3,317-line edition, with matching README corrections |
| [README revision](https://github.com/Dicklesworthstone/franken_robotics/tree/5c86ec1b4f9b74f4279682568155f39b4cd127ae) | Git commit (no tag, no release) | 21:28 | Audited claims corrected; see [section 8](#8-documentation-corrections) |
| [Astra rewrite](https://github.com/Dicklesworthstone/franken_robotics/tree/4d99e2a23f6bdd1d92f4f1497000e9245178343f) | Git commit (no tag, no release) | 21:44 | 3,424-line edition, 170-file bundle, README corrections |
| [Neon rewrite](https://github.com/Dicklesworthstone/franken_robotics/tree/1f5703d861eb042580e22cb45b9ee60558711597) | Git commit (no tag, no release) | 21:55 | 1,694-line edition, rewritten verifier, round-1 record synced |
| [v1.0 reviews and Phase 2 start](https://github.com/Dicklesworthstone/franken_robotics/tree/ff5e524900b2ca6f1a9853336d1650c73f17cc5b) | Git commits (no tag, no release) | 21:56 – 21:57 | Math review (26 findings), coherence review (44), partial Appendices F to J |
| [README update](https://github.com/Dicklesworthstone/franken_robotics/tree/361ad6c022a85a46b0be6238b1915222130e09be) | Git commits (no tag, no release) | 21:57 – 21:58 | New counts, both reviews, conflicting thresholds |
| [v1.0 Phase 2 drafts](https://github.com/Dicklesworthstone/franken_robotics/tree/ec917bbc4799b1ca5630837431ed127a25666644) | Git commits (no tag, no release) | 22:02 – 22:05 | §39 to §47, Appendices F to J and L, completeness audit |
| [README corrections](https://github.com/Dicklesworthstone/franken_robotics/tree/6c8e158b4cadf6a47e2de9c98b250d0d531ba40a) | Git commits (no tag, no release) | 22:03 – 22:05 | Review wording corrected, gate register and Phase 2 parts added |

**Three parallel tracks, not a version chain.** No plan cites another plan. Neon's "supersedes v1.0" refers
to Neon's own earlier draft, not to base v0.2. The only cross-track awareness in the record is round 2's case
file (evidence row E017), which notes that the Neon and Astra editions exist and declines to treat them as
evidence. The v1.0 charter ends that separation: it absorbs all three. Do not infer who wrote an edition from its
name: "Neon" is also the name of one of the duel's models (Gemini 4 Neon), and nothing in the files states authorship.

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
- Sixteen bets, sixteen North-Star scenarios, oracles KA-01 to KA-14, 37 gates, 21 kill gates and a 100-bead
  dependency graph to M6 (§3, §5, §25–§29; 97 beads in the text captured by the import commit).
- A last rewrite at 20:08 (1,452 → 1,654 lines): killed hypotheses renumbered to K-01 to K-13 (adding K-09 to K-13,
  among them "`fln-anvil` egraph, norm_num and simp exist", "GPU RL on `DenseTensor`" and "Metal 64-bit atomics"), a
  sixth pass in Appendix E.2 (a three-auditor sweep of math, live code and grammars), and three early evidence rows
  in the round-1 case file marked superseded.
- A further rewrite at 21:51 (1,654 → 1,694 lines), folding in a second three-auditor sweep: Appendix E.2 now lists
  seven passes, and `frob-mcp` plans 46 tools (was 42). It lists fixes to the test that an α-theory root is real,
  the `.ftube` erosion, the return-map adjoint (now nonsingular), the motor convention (power-invariant, absolute
  kelvin), the SE(3) Cayley map (explicit matrix form) and an episode-level probability bound. The round-1 case,
  report, duel report and disagreement register were synced to it.
- [`planning/verify_plan_math.py`](https://github.com/Dicklesworthstone/franken_robotics/blob/1f5703d861eb042580e22cb45b9ee60558711597/planning/verify_plan_math.py): a standard-library checker, largely rewritten at 21:50, of Neon's 16 numerical cards, crate table and layering, bead graph,
  cross-references, 46 MCP tools and worked examples (with a recursive-descent parser). Run for this update, it reports
  zero errors on 1,694 lines; it did so on the 1,654-line text as well.

### Closed workstreams

- Round 1 (section 2 above), cited from Neon's header and Appendices A, B and E.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) imports Neon as rewritten at 19:50:43.
- [`d18b5a1`](https://github.com/Dicklesworthstone/franken_robotics/commit/d18b5a1f083fa76a4c035a26840ad2b0177a44f7) the 20:08 text; [`1f5703d`](https://github.com/Dicklesworthstone/franken_robotics/commit/1f5703d861eb042580e22cb45b9ee60558711597) the 21:51 text and verifier.

**Caveats for readers of Neon.** The audit passes listed in its Appendix E.2 have no artifacts in `planning/`. Its
headline "delete 90%–99% of futile simulations" is stronger than its own pass bars (XT-P1 passes at 5× and 10×,
that is 80% and 90%). The quotient count it cites (735,471 → 24,546) reproduces, but on a constructed fixture rather
than a design campaign. The 512-topology figure is unmeasured. `verify_plan_math.py` hard-codes an absolute `ROOT`
path, so it runs unmodified only on the author's machine.

Two earlier caveats in this file are withdrawn because the text changed. At the import commit, the thermal card
(33.5 s from C_th = 79 J/K, G_th = 0.85 W/K) did not follow from its inputs; the current card (C_th = 140 J/K, G_th =
0.4545 W/K) does, and recomputing it gives 33.49 s. And the `.fenergy` figures are now printed as 50.5 ms nominal and
45.5 ms with an additional 0.10 J reserve, both of which `verify_plan_math.py` recomputes.

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
- Probes of sibling code: three errors in FrankenSim's PPO update (clipping for negative advantages, stale critic
  values, attention carried across episode resets) and a planar-drive case that loses translation (§5.1, §9.3). The
  21:26 rewrite adds a joint-sampling defect: signs of two nominally independent Gaussian actions agreed in about
  66.55% of draws, where independence predicts 50%.
- The same rewrite qualifies several claims with small counterexamples: a friction sweep that is not monotone (success
  at 0.500, timeout at 0.525, success at 0.550, with other outcomes under finer steps), a score crossing that is not
  population attainment (0.78548 chance over 50 looks at a true 0.89), a bootstrap lower bound that can be false, and
  accepted-decision rate as a separate measure from restricted time. It also corrects the pusher accounting: the 575.7 µs
  is a residual after subtracting a solve-only median, so the 1.46× ceiling is conditional on it.
- A third rewrite at 21:43 binds design guarantees to assumptions: the stance result needs a freely adjustable force
  allocation (a globally affine controller needs half-width 1/2 m, 7/3 times as much), a G1 model that drops four
  welded links loses 1.377 kg, encoder samples can alias, pairwise sensor independence does not support a seven-read
  guarantee (55/64 = 85.94%, against 3807/4096 under joint independence), and two univariate polynomial cases that the
  pinned owner cannot certify soundly. A native check of binary32 variance merging ran on the M5 Ultra and both RTX
  4090s: subtracting raw second moments returned zero, the centred merge returned the exact value.
- Work packages W01–W43, milestones M0–M8, risks, and a sealed acceptance design with four strata and
  one-sided 95% lower bounds (§15–§19).

### Closed workstreams

- Astra's own reasoning audit (seven independently audited claims, five verified in scope), a 3 × 3 duel with
  18 external scores (top mean: compositional subsystem contracts, 665 of 1000; "No candidate reached 700 from
  both outside reviewers"), and follow-up checks (§20.4–§20.5).
- [`planning/astra-10x-reproduction.zip`](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/astra-10x-reproduction.zip): 155 entries at 19:36:48, with a new `followups/` folder. It was rebuilt at 21:25:48 with 161 entries ([`a635b91`](https://github.com/Dicklesworthstone/franken_robotics/commit/a635b911505c555b02494b232a9d76985db152f7)): contact work, policy sampling, studio contracts, benchmark and friction-refinement checks, and the 150-condition friction table. All 160 checksums in its manifest match. At 21:40 it became 170 files (nine added, none removed), and all 169 manifest checksums match at [`4d99e2a`](https://github.com/Dicklesworthstone/franken_robotics/commit/4d99e2a23f6bdd1d92f4f1497000e9245178343f). `python3 -B verify.py --all` and `python3 -B verify_negative.py` re-check it; neither was run for these updates.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) imports the edition as rewritten at 19:37:52 and the matching bundle.
- [`a635b91`](https://github.com/Dicklesworthstone/franken_robotics/commit/a635b911505c555b02494b232a9d76985db152f7) rebuilds the bundle; [`5afef55`](https://github.com/Dicklesworthstone/franken_robotics/commit/5afef55e5a9404d945441142c27df71c1f55cae0) rewrites the edition to cite it.
- [`4d99e2a`](https://github.com/Dicklesworthstone/franken_robotics/commit/4d99e2a23f6bdd1d92f4f1497000e9245178343f) the 21:43 edition, its bundle and README corrections.

---

## 5) Round 2 ("fable"): an isolated review of base v0.2

Round 2 reviews the base plan and is deliberately isolated from the other tracks: its study prompt says "Do
NOT read anything under planning/duel/, planning/mor/, planning/brenner/, or the files ending in __NEON.md or
__ASTRA.md". It finished at 20:52. Its findings are folded into the v1.0 Phase 1 drafts (section 6), and base v0.2
itself was not edited.

### Delivered capability

- **Reasoning audit passes** (18:53–19:01). P01: policy training is about 93% of wall-clock, so a faster
  simulator alone "is capped below 8× on training and at 2–5× on a whole iteration". P02: "of the sixteen
  capabilities, none is cleanly out of reach for the incumbent set as a whole". P03: three transferable
  structures from other domains. P04: one scene per world is impossible (8,192 × 75 MB = 614 GB); reject
  half-precision factorization. P05: a backcast pre-mortem proposing a rank-fidelity gate.
- **Claim audit** (20:46–20:49). 94 claims, 76 audited by two auditors who wrote none of them: 31 verified, 39
  partially supported, 2 hypothesis, 2 rejected, 2 non-material. 18 were not audited (17 pending and one earlier
  non-material). Rejected: "no incumbent combines mixed topologies, determinism and articulated contact" (C021;
  Newton's SolverSteel documents all three) and "de-scope the Lean capsules" (C084). Recommended posture:
  EXPERIMENT_FIRST.
- **Brenner slate, evidence and verdicts.** Hypotheses H-01 to H-24, 23 evidence packs (19:15–19:23; H-18 has none)
  and a [verdict register](https://github.com/Dicklesworthstone/franken_robotics/blob/e88398a5caef7b539712d41b97e792809decdb09/planning/fable/brenner/deliverables/VERDICT_REGISTER.md) (20:31): 18 narrowed, 4 refuted (H-04 supercompiled kernels, H-09 incremental evaluation, H-15 half precision, H-16 living multiphysics), 1 deferred (H-12) and 1 active (H-19, "a thin layer over an incumbent").
- **Cross-family challenges** (20:39:55 and 20:39:57). Codex contests 13 verdicts and Gemini 12; 16 of 24 are contested
  by at least one and 9 by both. Both contest the question of record. They propose new hypotheses and reuse the IDs
  H-25 and H-26 between them. No verdict has been adjudicated since.
- **Idea duel.** Three slates of 30 ideas winnowed to 5 (19:09–19:20), three cross-scoring files (19:47–19:50),
  reactions (20:31–20:38) and a [report](https://github.com/Dicklesworthstone/franken_robotics/blob/e88398a5caef7b539712d41b97e792809decdb09/planning/fable/duel/DUELING_WIZARDS_REPORT.md) (20:52). Seven consensus winners, all from the Claude slate, led by the design-decision benchmark (858), paired multilevel estimation (853) and ragged cohorts (831). 13 killed on score, 26 contested. The compiler is the same model family as one duellist, which the report says.
- **Graveyard harvest** (19:25): thirteen hotspots and three top cards (memoize everything downstream of
  `ModelId`; slot-level episode dispatch in `frobd`; paired multi-fidelity estimates with exact importance
  weights). "No gain here is measured."
- **Artifact compilation** (19:29): eleven decision cores, eight method families, and two recommendation
  contracts for the trust ladder. The top expected value (25.0) goes to a promote/reject e-process pair.

### Closed workstreams

- [Round-2 case file](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/mor/MOR_CASE.md), [hypothesis slate](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/brenner/hypothesis_slate.md), [graveyard harvest](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/graveyard/GRAVEYARD_HARVEST.md) and [artifact compilation](https://github.com/Dicklesworthstone/franken_robotics/blob/7041f51df6303c26be72ff0b67f83027ee6ab520/planning/fable/artifact/ARTIFACT_COMPILATION.md).
- [Reasoning report](https://github.com/Dicklesworthstone/franken_robotics/blob/e88398a5caef7b539712d41b97e792809decdb09/planning/fable/mor/MODES_OF_REASONING_REPORT_AND_ANALYSIS_OF_PROJECT.md), [handback](https://github.com/Dicklesworthstone/franken_robotics/blob/e88398a5caef7b539712d41b97e792809decdb09/planning/fable/brenner/deliverables/HANDBACK.md) and challenges from [Codex](https://github.com/Dicklesworthstone/franken_robotics/blob/e88398a5caef7b539712d41b97e792809decdb09/planning/fable/brenner/debates/CHALLENGE_COD.md) and [Gemini](https://github.com/Dicklesworthstone/franken_robotics/blob/e88398a5caef7b539712d41b97e792809decdb09/planning/fable/brenner/debates/CHALLENGE_AGY.md).
- Still open: adjudication of the contested verdicts.

### Representative commits

- [`7041f51`](https://github.com/Dicklesworthstone/franken_robotics/commit/7041f51df6303c26be72ff0b67f83027ee6ab520) captures round 2 mid-flight, at about 19:55.
- [`e88398a`](https://github.com/Dicklesworthstone/franken_robotics/commit/e88398a5caef7b539712d41b97e792809decdb09) captures it complete.

---

## 6) The v1.0 hybrid: integration charter and Phase 1 drafts

A hybrid plan that is meant to replace base v0.2 at its current path is being written under `planning/fable/blend/`.
Its charter sets the outline, the names and 16 binding decisions (G1 to G16), and states a "maximalist" brief: absorb
every mechanism from the three editions and both rounds, with a falsifier and a kill gate, and write narrowed or
refuted ones as the evidence leaves them.

### Delivered capability

- **Inputs compiled** (19:59–20:33): digests of [Astra](https://github.com/Dicklesworthstone/franken_robotics/blob/3996e58d7d7f6238661caf05043f939080decc4e/planning/fable/blend/ASTRA_DIGEST.md) and [Neon](https://github.com/Dicklesworthstone/franken_robotics/blob/3996e58d7d7f6238661caf05043f939080decc4e/planning/fable/blend/NEON_DIGEST.md), a [catalog](https://github.com/Dicklesworthstone/franken_robotics/blob/3996e58d7d7f6238661caf05043f939080decc4e/planning/fable/blend/IDEA_CATALOG.md) of 112 distinct ideas merged from about 225, and the [charter](https://github.com/Dicklesworthstone/franken_robotics/blob/3996e58d7d7f6238661caf05043f939080decc4e/planning/fable/blend/INTEGRATION_CHARTER.md).
- **Phase 1 drafts** (finished 21:14): 13 part files P00 to P12, about 331,600 words, §0 to §37 of 47. They set 43
  crates with a 561 kLoC cap, 37 bets, 43 goals, 24 North-Star scenarios and 40 constructions, and restate the
  performance targets as ratios to an incumbent measured on the project's own machines.
- **Reconciliation** (21:20–21:24): an identifier-collision report, renumbered refusal codes, ten binding decisions
  (R1 to R10) and inventories of 421 gate, kill-gate and admission-gate identifiers, 141 open questions, 169 verbs and
  296 refusal codes. One gate tightened: training time to a threshold is at most 1.25× a re-measured incumbent (was
  1.5×).
- **Mathematical review** (21:55): an independent, read-only [review](https://github.com/Dicklesworthstone/franken_robotics/blob/54c86463965334246251b09626e67d01bc57fa2c/planning/fable/blend/REVIEW_MATH.md) of §9–§13, §15, §16, §18 and §35. 26 findings: 2
  critical (the §16.7 variational step uses the wrong differential for its sign; §16.6 pads sums with the wrong zero),
  7 major and 17 minor, with replacement text.
- **Coherence review** (21:57): a [review](https://github.com/Dicklesworthstone/franken_robotics/blob/361ad6c022a85a46b0be6238b1915222130e09be/planning/fable/blend/REVIEW_COHERENCE.md) of the parts against the charter, the brief and each other. 44 findings: 9 where two
  binding texts cannot both be implemented (among them rank-agreement thresholds, proof-kind registries, verb names,
  reference capacities, mixed-batch thresholds and the same §16.7 differential), 23 single disagreements and 12 minor.
  Its checks also found the incumbent figures, crate counts and withdrawn claims consistent across parts.
- **Phase 2** (verification, the ten-fold test, the gate register, risks, milestones, beads, the first session,
  appendices) is set out in a [brief](https://github.com/Dicklesworthstone/franken_robotics/blob/db95b163d47cc23d98dfd1017ea2d4c26e04e796/planning/fable/blend/PHASE2_BRIEF.md). Drafted between 21:53 and 22:05:
  [§39 and §40](https://github.com/Dicklesworthstone/franken_robotics/blob/ec917bbc4799b1ca5630837431ed127a25666644/planning/fable/blend/parts2/P13b.md) (the ten-fold test and the gate register),
  [§41 to §44, §46 and §47](https://github.com/Dicklesworthstone/franken_robotics/blob/ec917bbc4799b1ca5630837431ed127a25666644/planning/fable/blend/parts2/P14.md) (risks, open questions, milestones, 80 work packages, first
  session, release rubric), [§45](https://github.com/Dicklesworthstone/franken_robotics/blob/ec917bbc4799b1ca5630837431ed127a25666644/planning/fable/blend/parts2/P15.md) (442 beads) and
  [Appendices F to J and L](https://github.com/Dicklesworthstone/franken_robotics/blob/ec917bbc4799b1ca5630837431ed127a25666644/planning/fable/blend/parts2/P16b.md). §38 and Appendices A to E, K, M and N are not written. The
  brief calls the refusal registry final text for Appendix O; the coherence review asks for changes to it (F28).
- **Checked for this update**: the bead tables hold 442 beads, 1,091 dependencies, none pointing to an equal or higher
  number, one root and one sink. The gate register lists 37 admission gates (32 plus 5 sibling), 279 convergence gates
  (278 plus GATE-GAUNT-01) and 111 kill gates, which matches the 421 identifiers of the inventory plus those six. The
  register says it governs where a chapter differs. It reads the mixed-batch gate numbers as different workloads, sets
  rank agreement at 0.7 and 0.5, and does not reconcile the two capacity tables.
- **Completeness audit** (22:04): [GAPFILL_1](https://github.com/Dicklesworthstone/franken_robotics/blob/ec917bbc4799b1ca5630837431ed127a25666644/planning/fable/blend/GAPFILL_1.md) checks the parts against Astra and Neon and offers 18 insertion
  blocks, none applied. Of 63 items from the two digests it marks 57 covered, 3 weaker and 3 deferred. Of 19 items added
  to those plans after the digests, 11 are missing and 7 weaker.

### Closed workstreams

- Phase 1 of the integration charter (section drafts); reconciliation of identifiers; two read-only reviews; a
  completeness audit.
- Not closed: applying the reviews and the audit, the rest of Phase 2, assembly, adjudication. The drafts bind to the
  verdict register of section 5 as written, list three open disagreements in §0.8, and do not mention the cross-family
  challenges.

### Representative commits

- [`3996e58`](https://github.com/Dicklesworthstone/franken_robotics/commit/3996e58d7d7f6238661caf05043f939080decc4e) The charter, inputs and Phase 1 text before reconciliation (byte-identical to the swarm's own backup).
- [`db95b16`](https://github.com/Dicklesworthstone/franken_robotics/commit/db95b163d47cc23d98dfd1017ea2d4c26e04e796) The reconciled parts, inventories and Phase 2 brief.
- [`54c8646`](https://github.com/Dicklesworthstone/franken_robotics/commit/54c86463965334246251b09626e67d01bc57fa2c) The math review and the first Phase 2 appendices; [`ff5e524`](https://github.com/Dicklesworthstone/franken_robotics/commit/ff5e524900b2ca6f1a9853336d1650c73f17cc5b) the coherence review.
- [`8221cff`](https://github.com/Dicklesworthstone/franken_robotics/commit/8221cff56de5d9320e2c39277ace4834df93d5ca) §41 to §47, Appendices F to J and L and the completeness audit; [`ec917bb`](https://github.com/Dicklesworthstone/franken_robotics/commit/ec917bbc4799b1ca5630837431ed127a25666644) §39 and §40.

---

## 7) Repository bootstrap

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

## 8) Documentation corrections

The round-2 audit overturned claims that the first README repeated from base v0.2. The second README revision withdraws
them in the text and replaces the base throughput table. This section records what changed and why.

### Delivered capability

- **Claims withdrawn**: the base plan's fossils about CPU replay of GPU worlds and fixed morphology, the statement that
  training a policy per design takes "hours" (published runs take about 7 minutes to an hour), and the base §28.1
  throughput table, whose units were inconsistent (400 k env-steps/s of training at decimation 10 implies twice its
  own 2.0 M physics target) and whose targets fell below published incumbent figures.
- **Replaced by** the v1.0 draft's targets: ratios to an incumbent measured on the project's own hardware, with a
  tolerance and an ambition for each workload, stated in units. They are targets, and Phase 2 may change them.
- **Added**: the v1.0 hybrid section, round 2's verdicts and their contest, Neon's verifier, the CPU MuJoCo timings behind
  round 2, the seventh claim (Design comparison), the charter's milestones and scope, exit code 8, and the target
  layout.
- **`.gitignore`**: run records (`*.frun`), Python build output for the planned bindings, and the planning agents'
  backup copies.
- **Second update** (21:57–21:58): Astra and Neon counts, the two reviews and the thresholds the coherence review found
  conflicting. The Astra-specific README lines in [`5afef55`](https://github.com/Dicklesworthstone/franken_robotics/commit/5afef55e5a9404d945441142c27df71c1f55cae0) and [`4d99e2a`](https://github.com/Dicklesworthstone/franken_robotics/commit/4d99e2a23f6bdd1d92f4f1497000e9245178343f) were written by the Astra track's own agent.
- **Corrections to that update** (22:03–22:05): it had called the v1.0 drafts unreviewed beside a row listing their
  reviews, described the draft's real-root test as too weak when the review says it is reversed, and called Appendix O
  final although the coherence review asks for changes. The README also now covers the Phase 2 parts.

### Closed workstreams

- The documentation debt left by the first README (section 7), found by reading the round-2 reasoning report and the
  draft's §1.6 permitted wordings.

### Representative commits

- [`5c86ec1`](https://github.com/Dicklesworthstone/franken_robotics/commit/5c86ec1b4f9b74f4279682568155f39b4cd127ae) README and `.gitignore`.
- [`521bb7b`](https://github.com/Dicklesworthstone/franken_robotics/commit/521bb7bade8b7bbd95766dd63564a5556ba82f5c) and [`361ad6c`](https://github.com/Dicklesworthstone/franken_robotics/commit/361ad6c022a85a46b0be6238b1915222130e09be) README updates; [`da1b334`](https://github.com/Dicklesworthstone/franken_robotics/commit/da1b3341d924d2aa9158401b8211fb4a69eca5ba) and [`6c8e158`](https://github.com/Dicklesworthstone/franken_robotics/commit/6c8e158b4cadf6a47e2de9c98b250d0d531ba40a) corrections.

---

## Notes for Agents

- **Start with the README's status table**, then the edition that matches your task (see its Reading paths).
  Do not treat any one edition as canonical until a reconciled plan says so.
- **What has been folded where.** Round 1 is folded into Neon v2.0 only. The Astra study is folded into Astra
  only. All three editions and both rounds feed the v1.0 drafts. Those drafts have had two read-only reviews (math and
  coherence), and neither has been applied. Base v0.2 itself has not been edited since 18:36:46.
- **Before implementing from a v1.0 part**, read `REVIEW_MATH.md` and `REVIEW_COHERENCE.md`. The math review's two
  critical items and the coherence review's nine S1 items name places where the parts contradict each other.
- **Verdict contest.** The v1.0 drafts treat the verdict register as settled. The challenges in
  `planning/fable/brenner/debates/` contest 16 of its 24 rows. Read both before relying on a verdict.
- **Stale inputs of the v1.0 drafts.** `NEON_DIGEST.md` (20:04) predates Neon's 20:08 and 21:51 rewrites;
  `NEON_DELTA_SINCE_DIGEST.diff` (21:19) covers only the first. `ASTRA_DIGEST.md` (19:59) predates Astra's 21:26 and
  21:43 rewrites; `GAPFILL_1.md` covers the first and Neon edits up to 21:38. `IDEA_CATALOG.md` (20:27) marks the round-2 ideas "unscored"
  although the duel's score files date from 19:47 to 19:50. The 0.86 mixed-batch figure in the H-03 verdict row comes
  from MOR claim C022; the H-03 evidence pack does not contain it.
- **Numbers that disagree.** The reasoning report says nothing was measured on an incumbent, while Brenner evidence
  packs H-01, H-11, H-15 and H-19 timed CPU MuJoCo on the M5 Ultra under load. The performance table now in the README
  follows the v1.0 draft §36.3 as reconciled.
- **Missing artifacts that the plans cite:** base Appendix K (review log), the first three audit records listed in Neon's
  Appendix E.2, and §38 and Appendices A to E, K, M and N of v1.0. `scratch/verify_*.py`, cited by earlier Neon text, is replaced by
  `planning/verify_plan_math.py`, whose `ROOT` path is hard-coded.
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
