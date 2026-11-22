# Dueling Idea Wizards Report: FrankenRobotics (ten-fold pass)

Date: 2026-11-21. Compiled from the nine `WIZARD_*.md` files in this directory. Scores are copied from the
score files as written; averages are arithmetic means of the scores an idea actually received. This synthesis
was compiled by a Claude-family agent, the same family as one of the three duellists (CC).

## Executive Summary

Three agents each generated 30 ideas for making the FrankenRobotics plan (v0.2) a ten-fold advance over
existing systems and winnowed them to five, giving 90 ideas and 15 finalists. Each agent then scored the other
two slates from 0 to 1000 and reacted to the scores it received. Of the 69 ideas that received two scores,
**7 are consensus winners** (700 or more from both other agents), 6 were killed (both scores below 300) and 26
are contested (the two scorers are 200 or more points apart). Seven more ideas were scored below 300 by the
only agent that scored them, giving 13 kills on score. All seven consensus
winners come from the Claude slate: four of its five finalists and three ideas it had cut. No Codex or AGY
finalist reached 700 from both scorers. The three highest averages are the **design-decision benchmark**
(CC #25, 858), **paired multilevel estimation across fidelity tiers** (CC #4, 853) and **ragged cohorts that
step many different robots in one launch** (CC #1, 831). The reactions add a point the scores alone do not
show: all three agents independently proposed a benchmark that makes "10×" falsifiable and a replay-based
failure debugger, and after the reveal all three put the benchmark first.

## Methodology

- **Agents.** CC = Claude (Claude Opus 5.5 in Claude Code, per its file headers). COD = Codex (GPT family; the
  files do not name the model version). AGY = Gemini family (the files do not name the model version).
- **Ideas.** 30 per agent, winnowed to 5 by the author. An idea's number is its author's own rank (#1 best).
- **Scoring.** Each agent scored the other two slates on 0–1000, with a one-line verdict on the 10× claim and
  the strongest technical objection. Every idea therefore has at most two scores, never three.
- **Phases actually completed.** Study, ideate, cross-score, reveal and reaction. The reveal prompt also asked
  each agent to steelman the strongest idea from each opponent and to name up to three ideas nobody proposed,
  so those sections sit inside the reaction files.
- **Phases not run.** No separate rebuttal, forced-steelman or blind-spot files exist (`WIZARD_REBUTTAL_*`,
  `WIZARD_STEELMAN_*`, `WIZARD_BLINDSPOTS_*`), and there was one round only.
- **Degradation.**
  - All three reaction files exist; none is missing.
  - AGY scored 39 of the 60 opposing ideas: all 10 opposing finalists, 14 of Claude's other 25 and 15 of
    Codex's other 25. CC and COD each scored all 60. As a result 21 non-finalist ideas have one score only.
  - AGY gave Claude's ideas 11 and 24 one joint score (720); it is entered against both.
  - The blind-spot ideas in the reactions were never cross-scored.
  - Codex's reaction states revised dispositions but no revised numbers. Claude and AGY gave some revised
    numbers; they are listed separately below and do not enter the matrix.
  - Two framings were scored by CC only: Codex's benchmark protocol (740) and AGY's time-to-verified-design
    arithmetic (430). AGY gave Codex's protocol no number in its score file but placed it, paired with CC #25,
    in its 720–780 adoption band.
  - Claude's idea file labels its web facts as taken from search-result summaries, not from the pages.

## Score Matrix

Verdict rules, applied in this order to ideas with two scores: **consensus winner** = both scores 700 or more;
**killed** = both scores below 300; **contested** = scores 200 or more apart; **near miss** = average 700 or
more with one score below 700; **weak** = one score below 300; otherwise **no consensus**. An idea with one
score is killed if that score is below 300. "(own)" marks the author's column.

### Finalists (each author's top five)

| Idea | Origin | CC score | COD score | AGY score | Average | Verdict |
|---|---|---:|---:|---:|---:|---|
| CC #4 Paired multilevel estimation across tiers | CC | (own) | 891 | 815 | 853 | CONSENSUS WINNER |
| CC #1 Ragged cohorts: one launch steps thousands of different robots | CC | (own) | 817 | 845 | 831 | CONSENSUS WINNER |
| CC #5 Counterfactual probes: edit the robot at the moment it fails | CC | (own) | 841 | 790 | 815.5 | CONSENSUS WINNER |
| CC #2 Sampling MPC on each design's own model, nested in the batch | CC | (own) | 739 | 875 | 807 | CONSENSUS WINNER |
| CC #3 One policy per robot family, taught by MPC, carried across typed edits | CC | (own) | 724 | 670 | 697 | NO CONSENSUS; demoted by its author |
| COD #5 Rank repairs by replay interventions | COD | 630 | (own) | 740 | 685 | NO CONSENSUS |
| AGY #4 Closed-loop structural link synthesis, manufacturability, GD&T | AGY | 460 | 805 | (own) | 632.5 | CONTESTED (gap 345) |
| AGY #3 Universal zero-training GPU sampling MPC (Dial-MPC + IFT shooting) | AGY | 790 | 429 | (own) | 609.5 | CONTESTED (gap 361) |
| COD #1 Compile engineering requirements into morphology search | COD | 540 | (own) | 640 | 590 | NO CONSENSUS; narrowed by its author |
| COD #3 Randomized referee audits (inverse-propensity correction) | COD | 610 | (own) | 545 | 577.5 | NO CONSENSUS; demoted by its author |
| AGY #5 50 ms interventional causal debugging and exploit quarantine | AGY | 670 | 442 | (own) | 556 | CONTESTED (gap 228) |
| AGY #2 IFT contact and morphology adjoints | AGY | 380 | 665 | (own) | 522.5 | CONTESTED (gap 285) |
| COD #2 Transfer controllers across typed edits | COD | 440 | (own) | 595 | 517.5 | NO CONSENSUS; demoted by its author |
| COD #4 Reusable route-feasibility evidence | COD | 460 | (own) | 510 | 485 | NO CONSENSUS; demoted by its author |
| AGY #1 Whole-robot-in-SRAM persistent mega-kernel, tree-sparse LDLᵀ, design-SIMD | AGY | 440 | 521 | (own) | 480.5 | NO CONSENSUS |

No finalist received a score below 300. The lowest finalist score is 380 (CC on AGY #2).

### Non-finalists with two scores

| Idea | Origin | CC score | COD score | AGY score | Average | Verdict |
|---|---|---:|---:|---:|---:|---|
| CC #25 Design-decision benchmark against B0 and B0⁺ | CC | (own) | 936 | 780 | 858 | CONSENSUS WINNER |
| CC #6 Rare-failure scene-grammar search | CC | (own) | 791 | 760 | 775.5 | CONSENSUS WINNER |
| CC #16 Homogeneous fused stepping (one fused kernel per world) | CC | (own) | 664 | 835 | 749.5 | NEAR MISS |
| CC #8 Interval and monotone carving of design space | CC | (own) | 827 | 655 | 741 | NEAR MISS |
| CC #12 Gait-cycle adjoints for design gradients | CC | (own) | 713 | 745 | 729 | CONSENSUS WINNER |
| COD #10 Batch topology churn by execution shape | COD | 660 | (own) | 765 | 712.5 | NEAR MISS |
| CC #11 Content-addressed evaluation cache | CC | (own) | 698 | 720 † | 709 | NEAR MISS |
| CC #24 Canonicalize equivalent designs | CC | (own) | 692 | 720 † | 706 | NEAR MISS |
| CC #14 CPU shadow auditor on idle cores | CC | (own) | 657 | 735 | 696 | NO CONSENSUS |
| AGY #10 Pareto antichain solver over discrete part catalogs | AGY | 460 | 854 | (own) | 657 | CONTESTED (gap 394) |
| COD #8 Rare-failure mining with a fixed audit distribution | COD | 560 | (own) | 730 | 645 | NO CONSENSUS |
| CC #9 Mac 256 GB as the scene-breadth tier | CC | (own) | 572 | 710 | 641 | NO CONSENSUS |
| AGY #8 Sparse brick-tree SDFs with sharp-edge overlays | AGY | 500 | 781 | (own) | 640.5 | CONTESTED (gap 281) |
| COD #11 Optimize a link from measured loads, preserving ports | COD | 420 | (own) | 795 | 607.5 | CONTESTED (gap 375) |
| AGY #12 Adversarial interior and disturbance generator | AGY | 440 | 754 | (own) | 597 | CONTESTED (gap 314) |
| AGY #11 Certified envelope prover and minimal UNSAT spec diagnosis | AGY | 450 | 714 | (own) | 582 | CONTESTED (gap 264) |
| COD #6 Gradients per contact regime | COD | 380 | (own) | 775 | 577.5 | CONTESTED (gap 395) |
| AGY #7 VOI + conformal surrogate + tolerance-slack scheduler | AGY | 470 | 647 | (own) | 558.5 | NO CONSENSUS |
| COD #25 Local energy balance per port for solver triage | COD | 400 | (own) | 715 | 557.5 | CONTESTED (gap 315) |
| AGY #23 Paired-episode failure clustering | AGY | 430 | 674 | (own) | 552 | CONTESTED (gap 244) |
| AGY #20 Activation-gated coupled interior fixtures | AGY | 480 | 620 | (own) | 550 | NO CONSENSUS |
| COD #27 Mac for scene diversity, CUDA for training | COD | 360 | (own) | 690 | 525 | CONTESTED (gap 330) |
| AGY #9 Morphology-conditioned graph-transformer policy | AGY | 560 | 488 | (own) | 524 | NO CONSENSUS |
| AGY #17 Nonholonomic Lie-bracket primitives / swept passability prover | AGY | 290 | 743 | (own) | 516.5 | CONTESTED (gap 453) |
| AGY #24 Cross-family mission compiler | AGY | 390 | 642 | (own) | 516 | CONTESTED (gap 252) |
| AGY #15 Exact contact wrench cone and mode-transition planner | AGY | 290 | 725 | (own) | 507.5 | CONTESTED (gap 435) |
| COD #9 Share test cases across symmetry orbits | COD | 250 | (own) | 745 | 497.5 | CONTESTED (gap 495) |
| AGY #22 Real-place twin augmentation | AGY | 370 | 596 | (own) | 483 | CONTESTED (gap 226) |
| AGY #6 Zero-copy unified-memory live shadow referee | AGY | 400 | 559 | (own) | 479.5 | NO CONSENSUS |
| AGY #25 Swarm knowledge graph and sub-assembly grafting | AGY | 390 | 568 | (own) | 479 | NO CONSENSUS |
| COD #13 Manufacturing variation into part decisions | COD | 240 | (own) | 710 | 475 | CONTESTED (gap 470) |
| COD #14 Bounded reduced model per locomotion regime | COD | 310 | (own) | 635 | 472.5 | CONTESTED (gap 325) |
| AGY #30 Symbolic kinematic-loop closure | AGY | 260 | 678 | (own) | 469 | CONTESTED (gap 418) |
| AGY #14 Broken-symmetry perturbation calculus | AGY | 280 | 601 | (own) | 440.5 | CONTESTED (gap 321) |
| AGY #19 Port-Hamiltonian passive actuators and contact | AGY | 290 | 588 | (own) | 439 | CONTESTED (gap 298) |
| COD #15 Skill sequences over contact modes | COD | 420 | (own) | 450 | 435 | NO CONSENSUS |
| AGY #18 Koopman/Floquet limit-cycle stability oracle | AGY | 310 | 541 | (own) | 425.5 | CONTESTED (gap 231) |
| AGY #21 Sustained-duty thermal/airflow/fluid cards | AGY | 270 | 539 | (own) | 404.5 | CONTESTED (gap 269) |
| AGY #27 Dual-4090 asymmetric pipeline | AGY | 270 | 533 | (own) | 401.5 | CONTESTED (gap 263) |
| AGY #28 Coherent cone-traced sensors | AGY | 320 | 476 | (own) | 398 | NO CONSENSUS |
| COD #26 Certified recovery region for one maneuver | COD | 290 | (own) | 470 | 380 | WEAK (one score below 300) |
| AGY #16 E-graph morphology canonicalization | AGY | 270 | 406 | (own) | 338 | WEAK (one score below 300) |
| COD #7 Proof-driven time stepping in quiet windows | COD | 290 | (own) | 380 | 335 | WEAK (one score below 300) |
| AGY #26 Proof-carrying roofline autotuner | AGY | 220 | 416 | (own) | 318 | WEAK (one score below 300) |
| AGY #13 Real-to-sim identification / living hardware posterior | AGY | 320 | 306 | (own) | 313 | NO CONSENSUS |
| COD #28 Forecast feedback on agent decisions | COD | 320 | (own) | 260 | 290 | WEAK (one score below 300) |
| CC #18 Learned fast-to-referee correction | CC | (own) | 309 | 210 | 259.5 | WEAK (one score below 300) |
| CC #22 Learned morphology-conditioned dynamics surrogate | CC | (own) | 284 | 230 | 257 | KILLED (both below 300) |
| COD #29 Differentiate through all contact changes | COD | 160 | (own) | 310 | 235 | WEAK (one score below 300) |
| CC #26 Multi-agent LLM design debate | CC | (own) | 238 | 190 | 214 | KILLED (both below 300) |
| CC #28 Population-scale deformable (MPM) terrain | CC | (own) | 185 | 240 | 212.5 | KILLED (both below 300) |
| AGY #29 Stator-winding and gearbox co-design | AGY | 210 | 194 | (own) | 202 | KILLED (both below 300) |
| CC #30 Video-to-splat scenes for RGB training | CC | (own) | 151 | 175 | 163 | KILLED (both below 300) |
| COD #30 Full MPM/FEM soil in every world | COD | 100 | (own) | 220 | 160 | KILLED (both below 300) |

† AGY scored ideas 11 and 24 together as "CC-11/24" at 720.

### Non-finalists with one score (AGY did not score them)

| Idea | Origin | CC score | COD score | AGY score | Average | Verdict |
|---|---|---:|---:|---:|---:|---|
| CC #15 Per-building passability map | CC | (own) | 766 | — | 766 | SINGLE SCORE |
| CC #7 `frob solve --spec`: specification to evidenced Pareto set | CC | (own) | 635 | — | 635 | SINGLE SCORE |
| CC #10 Choose ranking-flipping experiments | CC | (own) | 623 | — | 623 | SINGLE SCORE |
| CC #20 Controller-free trajectory potential | CC | (own) | 588 | — | 588 | SINGLE SCORE |
| CC #19 Gait symmetry-mode reporting | CC | (own) | 571 | — | 571 | SINGLE SCORE |
| CC #21 Pool e-values across agents | CC | (own) | 503 | — | 503 | SINGLE SCORE |
| CC #13 MAP-Elites topology evolution | CC | (own) | 487 | — | 487 | SINGLE SCORE |
| COD #20 Decision cache across versions | COD | 480 | (own) | — | 480 | SINGLE SCORE |
| CC #17 Identify actuators from logs | CC | (own) | 474 | — | 474 | SINGLE SCORE |
| COD #12 Intent-preserving scene mutations | COD | 460 | (own) | — | 460 | SINGLE SCORE |
| CC #23 Mixed precision | CC | (own) | 444 | — | 444 | SINGLE SCORE |
| COD #18 Policy interchange by semantic actuator role | COD | 410 | (own) | — | 410 | SINGLE SCORE |
| COD #24 Acquire the measurement that changes a decision | COD | 400 | (own) | — | 400 | SINGLE SCORE |
| COD #21 Contact material grids from interface evidence | COD | 300 | (own) | — | 300 | SINGLE SCORE |
| COD #22 Symmetry matched to broken task context | COD | 290 | (own) | — | 290 | KILLED (only score below 300) |
| COD #17 Actuator-envelope identification experiments | COD | 250 | (own) | — | 250 | KILLED (only score below 300) |
| COD #16 Incremental contact-geometry builds | COD | 240 | (own) | — | 240 | KILLED (only score below 300) |
| COD #19 Index of contact-compatible modules | COD | 210 | (own) | — | 210 | KILLED (only score below 300) |
| COD #23 Geometric phase as experiment generator | COD | 180 | (own) | — | 180 | KILLED (only score below 300) |
| CC #27 Gradients through all contact | CC | (own) | 174 | — | 174 | KILLED (only score below 300) |
| CC #29 Live editing in the viewer | CC | (own) | 123 | — | 123 | KILLED (only score below 300) |

### Post-reveal revisions stated in the reactions (not in the matrix)

| Idea | Reviser | Was | Now | Reason given |
|---|---|---:|---:|---|
| Codex benchmark protocol (§0) | CC | 740 | 800 | CC's own arithmetic shows 20 paired briefs separate 10× from 3×; its "too few briefs" objection was wrong. |
| COD #3 randomized audits | CC | 610 | 660 | It is the validity layer when promotion targets are chosen adaptively. |
| AGY #5 intervention burst | CC | 670 | 600 | Plan §19.6 already re-runs results at half the step, the next tier and perturbed cards; AGY adds placement and two slots. |
| AGY #4 link synthesis | CC | 460 | 620 | Distal link inertia changes dynamic rankings; loads can come from the referee tier. |
| COD #11 link from measured loads | CC | 420 | 540 | Same argument. |
| AGY #2 IFT adjoints | CC | 380 | 470 | Local margin sensitivities are the right first consumer; closed-loop design gradients still rejected. |
| AGY #1 fused kernel | CC | 440 | 500 | Stepping speed matters once sampling MPC is the default controller; the 20–30 M figure is still unsupported. |
| COD #1 / AGY #10 catalog search | CC | 540 / 460 | 580 / 560 | A sound cut on a partial assignment saves point checks multiplicatively in catalog sizing. |
| CC #4 + COD #3 as a hybrid | AGY | 815 and 545 | about 885 (COD #3 alone about 620) | Fitted coefficient plus audit layer beats either alone. |
| AGY #10 + CC #8 + discrete slice of COD #1 | AGY | COD #1 640, CC #8 655, AGY #10 self-cut | about 805 combined | Discrete catalog selection is largely monotone and cheap to test at M1. |
| Codex §0 protocol + CC #25 | AGY | 780 for CC #25 | about 910 | The benchmark must exist before any 10× claim. |

None of these revisions moves an idea across the 700 line for both scorers.

## Consensus Winners (700 or more from both other agents)

Ranked by average. All seven are Claude's.

1. **CC #25 Design-decision benchmark against B0 and B0⁺: 858** (COD 936, AGY 780). Cut by its author, who
   used it as a yardstick. COD: "Do first"; freeze utility, thresholds, tuning budgets, incumbent recipes,
   hidden briefs and adjudication. AGY: "the indispensable falsification instrument". Against: an easy planted
   corpus or an unmatched incumbent manufactures a 10× result (COD); keeping pinned incumbents fairly tuned
   needs maintenance (AGY). All three reactions favour Codex's censored restricted-mean-time protocol as the
   form it should take.
2. **CC #4 Paired multilevel estimation across tiers: 853** (COD 891, AGY 815). Reuses coupled multilevel
   Monte Carlo already in `fs-uq`. COD: "the best immediate leverage bet". Against: the unit-coefficient
   correction as written does not have the (1 − ρ²) variance the proposal quoted (COD); contact bifurcations
   over 10 s episodes can push ρ below 0.7 and cut the gain to under 2× (AGY). Claude accepted both and now
   specifies a fitted frozen coefficient with ρ measured on paired design contrasts.
3. **CC #1 Ragged cohorts: 831** (COD 817, AGY 845). Per-world model identity and bounded execution shapes as
   an early architectural contract. Against: padded dense solves and bucket fragmentation across DoF × contact
   capacity (both); the quoted 25 designs/s is the homogeneous ceiling, the ragged figure is 15 (COD). Claude
   accepted the 15 and replaced dense Cholesky with a tree-sparse factorization.
4. **CC #5 Counterfactual probes: 815.5** (COD 841, AGY 790). Short paired replays from a failure snapshot
   that tell an agent which experiment to try next. Against, from both: carrying state across a geometry,
   inertia or gearing edit mid-episode is invalid, and a mid-run rescue is conditional on a state the edited
   robot might never reach. Claude narrowed it to diagnostic interventions; design edits replay from reset.
5. **CC #2 Sampling MPC nested in the batch: 807** (COD 739, AGY 875). AGY: "the single highest-impact, most
   pragmatic idea in either opponent slate". Against: a fixed-budget planner can rank easy-to-plan designs
   above better bodies, and a strong incumbent can already run DIAL-MPC (COD); a 5 ms planner step risks
   tunnelling and the plan's Cayley refusal bound (AGY). The two scorers are 136 points apart.
6. **CC #6 Rare-failure scene-grammar search: 775.5** (COD 791, AGY 760). Cut by its author at sixth. Against:
   mapping discrete scene grammars onto the Gaussian germs `fs-uq` supports, and weight degeneracy if the
   proposal misses failure modes.
7. **CC #12 Gait-cycle adjoints: 729** (COD 713, AGY 745). Cut by its author. Against: ill-conditioned near an
   orbit bifurcation (COD, accepted by Claude); no stable periodic gait on stairs, clutter or doors (AGY).

**Outside the strict rule.** Codex's benchmark framing (§0) has one numeric score (CC 740, later 800) and an
AGY band placement of 720–780; after the reveal AGY rated it with CC #25 at about 910. Claude and AGY both
call it stronger than Claude's own version of the benchmark.

## Contested Ideas

Left for human judgment. Finalists first, then the widest gaps among the rest.

| Idea | High scorer and reasoning | Low scorer and reasoning | After the reveal |
|---|---|---|---|
| AGY #3 universal sampling MPC | CC 790: the clearest real 10× against per-design PPO, near parity per design against MJX + DIAL-MPC. CC flags that the idea overlaps its own #2. | COD 429: 102,400 planning steps per tick, 51.2 M per 10 s episode; at the unmeasured 25 M steps/s that is 2.05 s per episode, and it depends on AGY #1's unmeasured speed. Full-resolution planning discards a cost lever. | AGY concedes the 25 M figure and that its incumbent baseline (75–250 s per rollout) was wrong, and proposes coarse-then-exact annealing. Claude steelmans planning on the execution model. |
| AGY #4 structural link synthesis | COD 805: the most distinctive AGY feature; ship a narrow finalist workflow; fabrication readiness is a separate claim. | CC 460: adds scope, not decision speed; loads from compliant contact are artifacts of contact stiffness; source checks found the topology optimizer feature-gated, no path from a density field to the STEP exporter, and fabrication checks that test caller-supplied scalars. | AGY concedes the three source overstatements. CC raises to 620. Both accept a bounded finalist lane with referee-tier loads. |
| AGY #2 IFT contact adjoints | COD 665: keep a fixed-regime derivative pilot; local margin sensitivities are a good first consumer. | CC 380: the cited IFT code is for finite-element meshes; the event-sensitivity code refuses simultaneous events, which a trot produces by design; closed-loop design gradients need the controller's sensitivity. | AGY concedes the source point and Codex's Newton-convergence point, and disputes the controller argument by citing joint actor-critic and morphology optimization. CC raises to 470. |
| AGY #5 causal debugging burst | CC 670: adopt the intervention battery and solver-artifact slots under Codex's validity rules. | COD 442: possible for diagnostic latency, unsupported for causal certainty or exploit assurance. | AGY concedes the 50 ms figure fails under its own re-planning controller and that "exact causal proof" overstated a local screen. CC lowers to 600. |
| AGY #10 catalog Pareto antichains | COD 854: strong pragmatic differentiator; verify the finite frontier by enumeration. | CC 460: monotonicity breaks under dynamics; a quasi-static pre-filter. | CC raises to 560. AGY, who had cut it, now rates the combined catalog wedge about 805 and still objects that Codex ranks it above every dynamic mechanism. |
| COD #9 symmetry-orbit test sharing | AGY 745: a 4–8× evaluation saving with zero approximation error. | CC 250: interiors are asymmetric; floating-point and unmirrored noise break exact invariance; the saving is work one would not create. | Claude restates its objection. No concession from AGY. |
| COD #13 manufacturing variation | AGY 710: strong physical-realism differentiator on existing sibling crates. | CC 240: hardware is out of scope. | Not revisited. |
| AGY #17 nonholonomic passability prover | COD 743: combine with CC #15 for a first interior product. | CC 290: local estimates are not proofs in cluttered rooms; exact methods already exist for car-like bases. | Not revisited. |
| AGY #15 contact wrench cone planner | COD 725: useful for a small explicit mode graph. | CC 290: combinatorial growth past 6–8 contacts; sampling MPC already switches modes. | Not revisited. |
| AGY #30 symbolic loop closure | COD 678: solves a genuine numerical problem for a four-bar or parallel knee. | CC 260: common loops have closed forms. | Not revisited. |
| COD #6 gradients per contact regime | AGY 775: a strong continuous-optimization lever. | CC 380: the sibling code refuses simultaneous events and gait regimes are too short. | Claude notes AGY's own stated objection is the same refusal. Codex accepts a narrower fixed-support CPU experiment. |
| COD #11 link from measured loads | AGY 795: "Codex's biggest missed gem". | CC 420: loads come from compliant contact; link mass is a small share. | CC raises to 540. Codex agrees to move it up as a single-link loop. |

Three disputes that the gap rule does not capture:

- **CC #16 fused kernel (COD 664, AGY 835).** AGY calls it Claude's "biggest missed gem" worth 10–15×. Claude
  and Codex both reject that figure, citing memory-traffic and occupancy arithmetic. Claude then accepts
  AGY's narrower point: under sampling MPC, stepping speed multiplies into every decision. It now holds
  fusion as an M3 option and expects 2–4×.
- **AGY #1 mega-kernel (CC 440, COD 521).** Both scorers are low. AGY concedes the occupancy arithmetic, the
  20–30 M steps/s projection and the "under a week" estimate, restates the lever as 4–8× from tree-sparse
  factorization, localized fusion and direct Metal, and maintains that 440 was too low. CC moves to 500.
- **COD #3 versus CC #4.** AGY scored COD #3 at 545 as "mathematically dominated" by CC #4. Claude and Codex
  both reject the dominance claim: logged audit propensities keep the estimate unbiased when promotion
  targets are chosen after seeing GPU results. AGY then rated the hybrid about 885.

## Killed Ideas

**Scored below 300 by both scorers (6).** None was a finalist; each had been cut or parked by its author.

| Idea | Scores | Why it died |
|---|---|---|
| CC #22 learned morphology-conditioned dynamics | AGY 230, COD 284 | Learned contact models fail on the novel designs where accuracy matters; one-step errors compound under policy feedback. |
| CC #26 multi-agent LLM debate | AGY 190, COD 238 | A prompt wrapper over any incumbent; correlated reasoning and extra serial turns consume the gain. |
| CC #28 population-scale MPM terrain | AGY 240, COD 185 | Exceeds per-world memory and throughput; Newton already has MPM. |
| CC #30 video-to-splat scenes | AGY 175, COD 151 | Rendering work with no design-decision lever; RGB policies are out of scope until M5. |
| COD #30 full MPM/FEM soil in every world | AGY 220, CC 100 | Exceeds the per-world memory budget; Newton already has MPM. |
| AGY #29 stator-winding and gearbox co-design | CC 210, COD 194 | Custom motors are rarely chosen; supply, thermal and manufacturing constraints swamp the gain. |

**Scored below 300 by the only agent that scored them (7).** These rest on one opinion.

| Idea | Score | Why |
|---|---|---|
| CC #27 gradients through all contact | COD 174 | Switching and grazing make the derivative nonexistent or misleading. |
| CC #29 live editing in the viewer | COD 123 | Low value for an agent-first program; makes model identity ambiguous. |
| COD #22 symmetry matched to broken task context | CC 290 | Switching subgroups gives discontinuous actions. |
| COD #17 actuator-identification experiments | CC 250 | No hardware in scope; identifying a simulator's parameters from itself is circular. |
| COD #16 incremental contact-geometry builds | CC 240 | Typed robot edits never touch the scene; robot geometry rebuilds are tiny. |
| COD #19 index of contact-compatible modules | CC 210 | Catalogs hold hundreds of entries; a linear scan takes microseconds. |
| COD #23 geometric phase as experiment generator | CC 180 | Applies to principally kinematic systems, not legged contact or wheeled bases. |

**Conceded by the originator.** No finalist was withdrawn outright. Four were demoted by their authors, and
one framing and several headline claims were conceded.

| Item | What the author conceded |
|---|---|
| CC #3 family policy | Out of Claude's top five. "Not dead": an M5 option for screening only, with top-k regret as the falsifier. |
| COD #2 controller transfer | "Both objections land." Demoted as a standalone pillar; keep the role and frame correspondence check. |
| COD #3 randomized audits | No longer the default. "A second estimator experiment, rather than a product feature"; coupled estimation goes first. |
| COD #4 route-feasibility evidence | Demoted to a wheeled and tracked geometry lane. Codex still disputes the "under 2% of design time" figure as a guess. |
| AGY's multiplied time-to-verified-design table | Conceded as ignoring Amdahl's law and phase overlap. |
| AGY claims | The 50 ms figure and "exact causal proof" (#5); 20–30 M steps/s (#1); incumbents at 75–250 s per rollout (#3); three sibling-readiness statements (#4). The ideas are kept in narrower form. |

**One-sided scores below 300 (16, not counted as kills).** The other scorer was at 300 or above: CC #18;
COD #7, #9, #13, #26, #28, #29; AGY #14, #15, #16, #17, #19, #21, #26, #27, #30. Fourteen of the sixteen low
scores are Claude's (the other two are AGY's, on CC #18 and COD #28). Five of these ideas (COD #9, COD #13,
AGY #15, AGY #17, AGY #30) drew 678 or more from the other scorer and are listed as contested above.

## Blind Spots Named in the Reactions

Eight ideas that their proposers say none of the 90 contained. None has been cross-scored, so each rests on
its proposer alone.

| Proposer | Idea | Mechanism in one line | Stated lever | Cheapest falsifier |
|---|---|---|---|---|
| CC | Rank designs by achievable performance | Run each design at several controller budgets on the same world addresses, fit a saturation curve, escalate only contenders. | Top-3 regret at most half that of fixed-budget ranking at equal compute. | 20 variants at three budgets on CPU; if no curves cross, the idea is unnecessary. |
| COD | Bracket what the embodiment can achieve | A sound optimistic upper bound over all admissible controllers plus an achieved lower bound from a rollout. | 10× lower controller effort per accepted design on one energy- or torque-limited brief. | A small exhaustively solvable ramp model with a planted feasible candidate; kill on the first false rejection. |
| CC | Attack the task specification first | `frob task audit`: run deliberately capable or degenerate "ghost bodies" against the task's own predicate to find loopholes. | Detect at least 80% of planted loopholes at no more than 5% of a campaign's compute. | Kill if detection is below 60%, false alarms exceed 30%, or plain MPC on the real design finds the same loopholes. |
| CC | Calibrated item bank for the test battery | A response model over battery items, sampled by importance so uninformative scenes are skipped. | 4–8× fewer episodes expected (13× in the idealized case). | Replay on a recorded 40 × 150 × 4 matrix; kill under 2× for the 20 closest pairs. |
| COD | Compile common randomness from the task | Key disturbances by physical meaning (place, time, mission event) so pairing survives edits and rate changes. | 10× less referee work at equal uncertainty where mismatched exposure dominated variance (illustrative). | A fixture with a spatial slick strip and timed push; kill if marginals change or a timed push follows gait phase. |
| AGY | Inverse-dynamics convex robot synthesis | Inverse dynamics is affine in inertial parameters, so mass placement, gearing and actuator sizing along nominal trajectories become one cone program. | 10–50× faster initialization and pruning, with infeasibility certificates. | 30 designs; kill pruning if synthesized designs do not beat templates by 15% or false rejections exceed 15%. |
| AGY | Pre-characterized sub-assembly macromodels | Characterize limbs and modules once at their ports, then screen whole assemblies by composition, as chip design does with standard cells. | 100–1,000× faster architectural exploration below a stated dynamic regime. | 200 assemblies; kill pruning if top-10 recall is below 90%. |
| AGY | Estimator-observability and self-occlusion sensor co-design | Observability bounds from the invariant filter plus ray-traced self-occlusion, optimized with morphology. | 5–10× fewer wasted training runs on perception-blind designs. | 20 designs under estimated state; fail if the privileged-versus-estimated gap is not halved. |

Claude's and Codex's first entries address the same gap from two directions: fast evaluation scores body and
controller together, so a ranking can favour bodies that are easy to control.

## Meta-Analysis

**What each model favoured.**

- **Claude (CC)** built its slate on evaluation cost per design decision: batching different robots, a
  controller that needs no training, cheaper referee verdicts and failure probes. It was the harshest scorer
  (mean 369 over Codex's 30 and 389 over AGY's 30; "No idea reached 850"). It published a table of eleven
  fact checks that moved scores and listed the ideas where its scores might carry self-preference.
- **Codex (COD)** favoured statistical validity, sound pruning and measurement before building. Its two
  highest scores went to the benchmark (936) and paired estimation (891), and its top AGY pick was a CPU
  catalog solver (854) that AGY itself had cut. AGY and Claude both describe its slate as under-ambitious on
  GPU execution and control.
- **AGY** favoured raw kernel throughput, gradients and fabrication. It rewarded those themes in others
  (CC #16 at 835, COD #11 at 795, COD #6 at 775) and scored only a subset of the non-finalists. Both
  opponents found inflated baselines and overstated sibling readiness in its slate, which it conceded.

**Scorer means on the opposing finalists.** AGY on CC 799, COD on CC 802; AGY on COD 606, CC on COD 536;
COD on AGY 572, CC on AGY 548. Claude's finalists were rated about 200 points above either other slate by
both of its scorers.

**Similarity bias.** Claude's reaction observes that each reviewer scored highest the ideas that resemble its
own (AGY on CC #16, COD on AGY #10 and CC #4, CC on COD #10) and concludes that independent convergence is the
more informative signal. Two ideas were proposed independently by all three: replay experiments from a
failure snapshot (CC #5, COD #5, AGY #5) and a benchmark that makes 10× falsifiable (CC #25, Codex §0, AGY's
framing).

**Where adversarial pressure changed a position.**

- Claude: demoted its own #3; rewrote #4's estimator after Codex showed the quoted variance did not match the
  written estimator; narrowed #5; reversed "raw stepping speed is not the lever" within its own design;
  raised structural synthesis and catalog search; corrected a factual error about `fs-assimilate`.
- Codex: promoted heterogeneous batching from a late optimization to an early interface contract; made
  coupled multilevel estimation the default over its own audits; demoted #2 and #4; accepted that its four
  briefs are smoke fixtures, not a speedup estimate.
- AGY: withdrew its headline numbers and its multiplied speedup table; reversed its "dominated" verdict on
  COD #3; reinstated its own cut catalog solver; moved the benchmark to a mandatory first gate; ranked CC #4
  above CC #2 after the baseline fact check.

**Positions that did not move.** AGY holds that kernel speed is undervalued by both opponents and that Codex
over-weights quasi-static pruning. Claude holds that closed-loop design gradients do not work and that
symmetry-orbit sharing saves nothing real. Codex holds that randomized audits have a regime where they beat
stratified coupling, and that a cut over a family of unbuilt designs is different work from a leaf check.

## Recommended Next Steps

These are the items on which the score files and all three reactions agree, in the order the reactions give.

1. **Build the benchmark first** (CC #25 in Codex's §0 form). Measure an incumbent pipeline's phase times
   before committing GPU work, use censored restricted mean time on paired held-out briefs, and state each
   milestone gate against it. Claude's estimate is that about 20 held-out brief variants separate 10× from 3×.
2. **Paired multilevel estimation for referee verdicts** (CC #4), with a frozen fitted coefficient, ρ measured
   on paired design contrasts, the same controller on every tier, and Codex's randomized audits tested as the
   validity layer for adaptive promotion. First test: replay contrast residuals already collected for the
   fidelity battery.
3. **Per-world model identity and execution-shape separation as early contracts** (CC #1, COD #10). Start with
   exact-topology batches and measure padding and occupancy before committing to ragged buckets.
4. **Sampling MPC as a class-scoped controller candidate** (CC #2, AGY #3), tested against the plan's
   controller synthesis and native DIAL-MPC, with the planner step chosen per design and a check that rankings
   are stable as planner fidelity changes.
5. **One merged failure debugger** (CC #5, COD #5, AGY #5): diagnostic interventions from the snapshot,
   permanent edits validated from reset, unchanged-replay negative controls, output labelled `Estimated`.
6. **Cheap bounded lanes, each behind its own gate:** an exhaustive small-catalog co-design pilot on CPU at M1
   (COD #1, AGY #10, CC #8); a single-link structural loop for finalists with referee-tier loads (AGY #4,
   COD #11); kernel fusion only after profiling.
7. **Score the eight blind-spot ideas** before adopting any; the two that target the body-versus-controller
   confound answer the strongest standing objection to item 4.
