# WIZARD_SCORES_CC_ON_COD_AGY.md — Phase 5 Adversarial Cross-Scoring (Lane `CC` scores `COD` and `AGY`)

| Field | Value |
|---|---|
| Run / session | MOR `r1-20261121` · Brenner `RS-20261121-franken-robotics-10x` · `Q-001` · `2026-11-21` |
| Scorer | Lane `CC` (Claude Code, Opus 5.5) |
| Slates scored | `WIZARD_IDEAS_COD.md`: W01–W10 + 3 FTAs · `WIZARD_IDEAS_AGY.md`: #1–#10 + 3 FTAs (**26 items**) |
| Grounding | `MOR_CASE.md` E001–E026 (especially E021–E026); `.mor/run-r1/audit/AUD01-C001-C016.jsonl`; direct repo spot-checks and recomputed arithmetic (§6) |
| Conflict of interest | `CC` is a competing lane. Where an opponent idea overlaps a `CC` idea, this file says so and applies the same standard. §4 lists the places where repo facts or opponents beat `CC`. |

**Scale (0–1000).**

| Code | Criterion | Max |
|---|---|---:|
| **U** | Genuine 10× unlock over rivals, after Amdahl and against honest baselines | 300 |
| **S** | Mathematical and hardware soundness | 200 |
| **R** | Real-world utility for humans and AI agents | 200 |
| **X** | Executability against *today's* repo state, and tech-debt ratio | 150 |
| **H** | Honesty across the five claim axes, and calibration | 150 |

**Verdict codes:**
- **ADOPT** — take as is.
- **ADOPT-NARROWED** — take with a smaller scope or corrected claim.
- **SALVAGE** — keep the core, rewrite the claims.
- **REJECT AS STATED** — do not take in its current form.

---

## Summary Table A — `COD` (OpenAI Codex GPT-6.2-Sol)

| Rank | Item | Score | U | S | R | X | H | Verdict | One-line reason |
|---:|---|---:|---:|---:|---:|---:|---:|---|---|
| 1 | **FTA-P3** Observable energy budget at a physical boundary | **780** | 190 | 185 | 160 | 105 | 140 | ADOPT | **Best item in the duel.** The energy inequality holds for any contact sequence, and every number checks. |
| 2 | **W07** Settled-gait return-map adjoint | **725** | 170 | 170 | 130 | 110 | 145 | ADOPT | Best-executed saltation/Floquet version in any lane. Honest 13.6×, with mandatory refusal controls. |
| 3 | **W03** Skill transport via capability maps | **715** | 170 | 150 | 160 | 100 | 135 | ADOPT | Exact per-mode actuation maps, plus Farkas refusal witnesses for agents. |
| 4 | **W02** Operator-queue batching across robots | **650** | 150 | 130 | 150 | 85 | 135 | ADOPT (as an ablation arm) | Only proposal that tackles contact-count divergence. Dispatches per tick are not priced. |
| 5 | **W08** Configuration-space bottleneck certificates | **650** | 120 | 165 | 145 | 90 | 130 | ADOPT | Certifies both "fits" and "cannot fit". Exact discretization margins. |
| 6 | **W01** Substitution contracts for modules | **645** | 130 | 150 | 130 | 100 | 135 | ADOPT-NARROWED | Most buildable operad idea, but the 10× covers derivation only (8.9 ms/edit slack). |
| 7 | **FTA-P1** Quotient of the design space | **635** | 120 | 170 | 80 | 120 | 145 | ADOPT (as potency fixture) | 735,471 → 24,546 classes verified. Fixture is chosen to make the quotient large. |
| 8 | **W09** Counterexample-guided build packet | **630** | 120 | 130 | 160 | 85 | 135 | ADOPT-NARROWED | Right product gate (reviewer-accepted packet). The 5 h → 26 min figure is not measured. |
| 9 | **W04** Hybrid reachable-tube composition | **620** | 110 | 175 | 110 | 80 | 145 | ADOPT-NARROWED | Most rigorous reset semantics in the duel. Lever is lookup amortization; 4-D state only. |
| 10 | **W06** Closed-loop linkage synthesis | **590** | 80 | 160 | 95 | 125 | 130 | ADOPT-NARROWED | Executable within `real_rur` caps today, but 3-pose synthesis is textbook, so the speed claim fails. |
| 11 | **W10** Factor once, solve many contact RHS | **570** | 70 | 175 | 70 | 110 | 145 | ADOPT-NARROWED | Correct form of tensor-core contact. Workload (one tick, frozen contact state) is rare. |
| 12 | **W05** Causal recomputation of world branches | **540** | 100 | 125 | 125 | 60 | 130 | SALVAGE | Honest Amdahl, but robot-centric replays are mostly dirty after an intervention. |
| 13 | **FTA-P2** Interaction-calendar scheduling | **480** | 110 | 90 | 90 | 70 | 120 | SALVAGE | Separate envs are already independent, and policy ticks couple every step. |

**COD lane: mean 633 · median 635.**

## Summary Table B — `AGY` (Gemini 4 Neon)

| Rank | Item | Score | U | S | R | X | H | Verdict | One-line reason |
|---:|---|---:|---:|---:|---:|---:|---:|---|---|
| 1 | **#1** Operadic pHS + warp-locked heterogeneous megakernel | **600** | 220 | 90 | 150 | 75 | 65 | SALVAGE (high ceiling) | Limb Port-ABI is a real insight, but occupancy is 25% (not ≥75%) and contact coupling is not priced. |
| 2 | **#2** Buckingham-Π universal policy + symbolic pullback | **570** | 190 | 95 | 145 | 65 | 75 | SALVAGE | Right architecture, but the pullback formula is fixed-base and ignores contact wrenches. |
| 3 | **FTA-P3** Assume-guarantee quotient + KKT shield + conformal-e | **570** | 170 | 115 | 150 | 55 | 80 | ADOPT-NARROWED | Converges with `CC`'s TA-P3. Mislabels a simulated monitor as `ValidatedPhysics`. |
| 4 | **#10** Pearl-causal time travel + `franken_threed` dossier | **565** | 140 | 120 | 170 | 60 | 75 | ADOPT-NARROWED | Exact Shapley at 2¹⁰ and an agent-level gate. Assumes f3d crates that are not present. |
| 5 | **#3** Contact-stratified CBF tubes + KKT GPU shield | **560** | 170 | 100 | 150 | 55 | 85 | ADOPT-NARROWED | Includes guard-jump invariance. Proof path relies on stubbed tooling; "safe for any μ" overclaims. |
| 6 | **#6** Floquet–Poincaré implicit differentiation atlas | **550** | 200 | 85 | 130 | 80 | 55 | ADOPT-NARROWED | Core math correct. Scale check omits per-step Jacobians (≈65× optimistic). |
| 7 | **FTA-P2** Warp-cohomogeneous operadic super-batching | **500** | 190 | 80 | 130 | 50 | 50 | SALVAGE | Premise "CRN needs K ≥ 32" is false; inherits #1's errors. |
| 8 | **#8** Differentiable adversarial scene grammar | **455** | 150 | 65 | 130 | 50 | 60 | SALVAGE | Gradient of a binary failure is zero almost everywhere; β₀ connectivity ≠ task solvability. |
| 9 | **#5** M5 Ultra UMA co-processor symbiosis | **410** | 110 | 70 | 120 | 50 | 60 | SALVAGE | Gate never tests the 10× (generalization). Sub-µs handoff is not a Metal guarantee. |
| 10 | **#7** Study-quadric/Gröbner synthesis + feasible polytopes | **380** | 110 | 60 | 100 | 70 | 40 | SALVAGE / REJECT polytope rule | "100% of 4/5/6-bar" exceeds `real_rur` caps; inner-ellipsoid sampling excludes optima. |
| 11 | **#9** Sparse-tile LBM + IPC living multiphysics | **360** | 100 | 35 | 110 | 45 | 70 | SALVAGE (Housner) / REJECT (50 Hz LBM) | 50 Hz LBM on 10 cm cells is unstable (Ma ≈ 0.35, τ−½ ≈ 6×10⁻⁶). About 2 steps/s per env. |
| 12 | **FTA-P1** Staged algebraic erasure | **310** | 50 | 90 | 60 | 70 | 40 | REJECT AS A THIRD ALTERNATIVE | It restates the QoR's own resolution hypothesis; the falsifier is confounded. |
| 13 | **#4** Tensor-core mixed-precision Delassus | **220** | 60 | 30 | 60 | 50 | 20 | REJECT AS STATED | "κ ≤ 100" is really ≤ 4,801. Refinement solves the shifted system, not the contact problem. |

**AGY lane: mean 465 · median 500.**

## Headline Findings

1. **The best single item in the duel is `COD` FTA-P3, and it beats `CC`'s own TA-P3 for its class.** An energy budget at a guided physical boundary does not depend on the contact sequence, by construction. Every number in it recomputes exactly. It extends directly to ISO/TS 15066-style transient contact-energy limits for collaborative robots.
2. **All three lanes converged on the same five ideas:**
   - heterogeneous-morphology batching;
   - saltation/Floquet gait sensitivities;
   - mode-stratified shields with assumption monitors;
   - solvability-guarded red-team worlds;
   - counterfactual branching.

   Convergence is evidence that these ideas *matter*, not that any version is correct. §3 names the best-executed version of each, and none of them comes entirely from one lane.
3. **Four broken bounds change `AGY` verdicts:**
   - κ(W + ρI) ≤ 100 is actually ≤ **4,801** (C009, recomputed);
   - "12 warps/SM ⇒ ≥ 75% occupancy" is **25%** (Ada allows 48 warps/SM);
   - LBM at 50 Hz on 10 cm cells gives **Ma ≈ 0.35** per 1 m/s of flow and **τ − ½ ≈ 6×10⁻⁶**, which is unstable;
   - the Floquet scale check prices tangent GEMMs but not per-step Jacobians, so it is **≈65× optimistic**.
4. **`AGY` asserts sibling capabilities that do not exist today:**
   - `f3d-scene`/`f3d-compiler`/`f3d-gpu`/`f3d-assets` (the `franken_threed` workspace has 4 crates; verified);
   - `fln-anvil` egraph/norm_num (type stubs only; E024, verified);
   - multivariate Gram projection in `fsym-formal` (it has two univariate ZZ profiles; E023);
   - a GPU-resident `frankentorch` GNN (`ft-nn` is CPU-only; E025).

   `CC` shares the `fln-anvil` error (§4).
5. **`COD`'s weakness mirrors `AGY`'s.** Its arithmetic is flawless (all 30+ figures recomputed), but most of its 10× ratios are amortization ratios on narrow fixtures: a tray cart, a ballast line, a 3-pose four-bar. No `COD` item makes the *whole program* 10×. `AGY` aims at the program-level 10× and breaks bounds getting there.

---

## 1. Per-Item Critiques — `COD`

### COD FTA-P3 · Observable energy budget at a physical boundary — **780 · ADOPT**

- **What it gets right.** It replaces "verify the contact dynamics" with a measurable interface inequality: energy delivered toward the end of a guided stroke ≤ verified absorber work. That holds for *any* set of passive impacts and frictional interactions, so contact-sequence ambiguity drops out by construction. That is a genuine dissolution of Paradox 3 for impact and stop-type safety, not a compromise.
- **Arithmetic re-derived, all exact:**
  - kinetic energy ½·8·0.4² = 0.64 J;
  - drive work 20 W × 25 ms = 0.50 J;
  - total budget 0.64 + 0.50 + 0.25 + 0.10 = 1.49 J;
  - absorber 50 N × 40 mm = 2 J, leaving a 0.51 J reserve;
  - equality cutoff (2 − 0.99)/20 W = 50.5 ms, or 45.5 ms with the 0.10 J reserve.
- **Kill conditions are adversarial and physical:** an active energy source, an absorber work deficit, a guide bypass, and a late cutoff.
- **Limits.**
  1. It needs a physical guide that puts the absorber on every forward path, plus a measured lower work envelope. COD keeps both as separate physical receipts, which is correct.
  2. It does not cover tip-over or loss of balance, where energy is conserved but redistributed.
  3. The 10.5–20× monitor-decision gain is illustrative (100 µs vs 5 µs).
- **Beat `CC`.** `CC` TA-P3 needs statistical residuals δ₁–δ₃. This certificate needs none within its class. Recommend adding it as a new proof-obligation family ("energy-budget POs") next to barrier POs.

### COD W07 · Differentiate the settled gait through event time and impacts — **725 · ADOPT**

- **What it gets right.** It is the best-executed version of an idea all three lanes proposed (`CC-02`, `AGY` #6):
  - a correct saltation map;
  - a phase-removed Poincaré section, with quotients only by admitted symmetries;
  - the settled-cycle adjoint (I − DₓP)ᵀλ = ∇ₓJ;
  - a stored transversality margin.
- **Honest ratio.** 240 cycle-equivalents for 24 central finite differences vs 13 (orbit plus adjoint) gives 18.46×, and 245/18 = **13.61×** after validation. All recomputed.
- **Strict gate.** 20 grazing or unstable controls must be refused, at least 72/80 regular cases need cosine ≥ 0.95, and one silently admitted wrong event sequence kills it.
- **Limits.**
  1. It leaves the bigger lever unclaimed: warm-started continuation across morphology instead of RL retraining, plus symmetry-based gait seeding (`CC-02`).
  2. It does not reuse the existing `fs-spectral::floquet` (E021).
  3. GPU event-located integration does not exist yet (C003).
- **Recommendation.** Use W07's gate as the protocol of record for any gait-sensitivity claim.

### COD W03 · Transport skills through physical capability maps — **715 · ADOPT**

- **What it gets right.**
  - Exact per-mode affine actuation maps w = w₀ + B_m(q)u, from a constrained mass solve.
  - A bounded allocation program over actuator, friction, and support limits.
  - A **separating witness when a requested wrench is unattainable** — the agent gets a refusal *with a reason* before any training.
  - Honest amortization: 24(900/10 − 60) = 720 s of allowed incremental pretraining, and 10× needs 120 targets if pretraining costs 3,600 s. Both denominators are reported.
  - A hard gate: 18/20 feasible bodies within 3 pp of tuned baselines with **zero** target training, plus correct refusal of all 4 infeasible bodies.
- **Limits.** The scope is wheeled and wheel-legged task-space skills (braking, doorway alignment, tray stabilization). Legged locomotion needs *phase-sequence* transfer, which capability maps do not supply (`CC-02`/W07 complement it). The frozen learned residual is the real risk.
- **Beat `AGY` #2.** This is the rigorous form of `AGY`'s "pullback" lower layer.

### COD W02 · Execute mixed morphologies by batching operations across robots — **650 · ADOPT as the ragged arm**

- **What it gets right.** Pooling at the operator level — queues keyed by operator, row count, and solver state — is the only proposal in the three lanes that addresses divergence from **contact counts and solver iterations**, not just topology. `CC-01` claimed "no divergence" and missed this (§4).
- **Honest thresholds.** The reference needs ≥ 112 s of admission time for 10×, and the 70%-of-homogeneous kill is stated. Memory arithmetic checks: 192 MiB of links, 2 GiB of workspace, 36 KiB per dense W.
- **Unpriced cost: dispatches per tick.** Depth-ordered forward kinematics, leaf-to-root elimination, and contact passes come to roughly levels × operators × sweeps ≈ 50–100 dispatches per tick for 48-link trees. Metal has no grid-wide sync, and today's `ft-kernel-metal` waits synchronously on serial encoders (E025). So the launch fragmentation W02 set out to remove returns *per tick*. State also round-trips through global memory between levels.
- **Lever at risk.** The "admission without recompile" lever is plausibly beaten by parallel, cached MJX/MuJoCo compilation, which COD itself concedes.
- **Recommendation.** Run W02 against warp-per-morphology bins, exactly as audit C002 prescribes.

### COD W08 · Certify morphologies against configuration-space bottlenecks — **650 · ADOPT**

- **What it gets right.**
  - Dual inner/outer cubical complexes in (x, y, ψ) give witnesses of **both** feasibility and impossibility, reused across nested chassis envelopes.
  - Persistence pulls out the critical doorway.
  - Every discretization number is exact: 1,228,800 cells; 4.69 MiB per fixture mode; heading half-bin 0.4 m × 5.625° = 39 mm, plus 35 mm translation half-diagonal = **74 mm** refinement band; 14.69× with 0.92 s left over.
  - It answers the most common question for interior robots — *will this chassis get through that kitchen?* — with a checkable artifact.
- **Limits.** Wheeled or rigidly folded bodies only. Nonholonomic steering witnesses are deferred, and legged step-over is out of scope. The 180 ms-per-query planner baseline is illustrative.

### COD W01 · Compose robot modules with machine-checkable substitution contracts — **645 · ADOPT-NARROWED**

- **What it gets right.** It is the most implementation-ready operad idea in any lane, including `CC-07`:
  - exact port power cancellation, with the gear example's signs correct;
  - dependency digests that invalidate only the affected summaries;
  - a deliberate high-invalidation control (a global bus edit);
  - adversarial injections (stale bounds digest, reversed port sign) where one accepted invalid receipt kills it.

  It also builds on `fs-phs::interconnect_mapped`, which exists (E021).
- **The 10× is narrower than the headline.** COD's own arithmetic (81.92/7.68 = 10.67×) leaves **8.9 ms per edit** for all other work. The headline unit, "qualified edits/hour including … a replay sample," therefore cannot reach 10× once a replay takes seconds.
- **Missing pieces.** Constrained/DAE Dirac structures are deferred in `fs-phs` (E021), and no passivity proof generator exists (C001, E024).
- **Fix.** Gate derivation-plus-verification latency per edit (where 10× is plausible) and report end-to-end edit latency separately.

### COD FTA-P1 · Compile the quotient of the design space — **635 · ADOPT as the potency fixture**

- **Verified exactly.** C(24, 8) = 735,471 subsets map to **24,546** distinct (Σx, Σx²) classes: **29.96×**.
- **The equivalence is provable.** Equal collinear point masses inside an unchanged chassis contribute inertia that depends only on (count, Σx, Σx²).
- **Good potency tests.** The same-mass/same-COM, different-second-moment control must split, and the placement-dependent harness output must block reuse.
- **Converges with `CC` TA-P1** ("algebra deletes experiments") on a much cheaper and sharper fixture. `CC`'s `XT-P1` should borrow it.
- **Limit.** The fixture is chosen to have a large quotient. Realistic design spaces collapse to singletons once any unprojected observable differs, which COD says itself. As a general 10× it is unproven.

### COD W09 · Turn a mission into a counterexample-guided build packet — **630 · ADOPT-NARROWED**

- **What it gets right.**
  - The product gate is correct: an *independent reviewer accepts* a fabrication-review packet for 11 of 12 hidden briefs.
  - Generalizable cuts are separated from point failures ("provable cut only when it generalizes").
  - Board firmware, compliance, and physical signoff are separate receipts.
  - It builds on `fs-geocon`/`fs-fab`/`fs-io`, which exist (E022) but need admission.
- **Limits.**
  1. The 5 h vs 26 min (11.54×) is illustrative; no reference packet has been timed.
  2. Post-snap revalidation and faceted STEP output need explicit receipts (C013).
- **Overlap.** `CC-10` makes the same move. W09's reviewer gate is better.

### COD W04 · Compile hybrid safety as reachable-tube composition — **620 · ADOPT-NARROWED**

- **What it gets right.** The most rigorous hybrid semantics in the duel:
  - every possibly enabled guard successor under a common action, R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n;
  - inter-sample sweep, latency, chatter handling, and a verified backup domain;
  - runtime checks against the whole belief set.

  The numbers are exact: 16 MiB table; 10× at Q = 60,000 queries; 0/300 failures stated correctly as an ≈ 1% one-sided upper bound.
- **Limits.**
  1. The 10× is a lookup-amortization ratio (explicit-MPC style), so uniqueness against a tuned online reachability filter is modest.
  2. The state is 4-D plus belief. Cell counts explode beyond about 6-D.
  3. Reflective checking in `fln-kernel` is unbuilt (C006).

### COD W06 · Synthesize closed-loop linkages from task geometry — **590 · ADOPT-NARROWED**

- **What it gets right.** Executable today:
  - a 3-pose circumcenter solve;
  - degeneracy partitioned by determinant rank;
  - denominator saturation (sd − 1 = 0);
  - interval branch and clearance certification;
  - `real_rur` only for admitted zero-dimensional templates, which matches the verified caps (≤ 8 variables, ≤ 64 equations, degree ≤ 256; E023, re-verified).
- **Limits.** Three-position four-bar synthesis has textbook closed-form circumcenters, so a 16-start × 20 ms multistart baseline is a strawman. Against direct geometric construction (which COD names) the speed ratio collapses. The value is the **branch, clearance, and tolerance certificate**, not speed, and the score reflects that.
- **Relation to `CC-04`.** `CC-04` (homotopy, complete solution sets) is the bigger step beyond `real_rur`'s caps. W06 is the right first slice.

### COD W10 · Factor a contact system once for dozens of controlled interventions — **570 · ADOPT-NARROWED**

- **What it gets right.** Impeccable numerics:
  - 96³/3 = 294,912; 2·96² = 18,432;
  - 20.05M vs 1.475M flops = 13.6×;
  - the Amdahl requirement for 10× is p ≥ 97.1% (recomputed);
  - ‖b − Ax̂‖₂/λ_min is a valid error bound for SPD systems;
  - failed guards go to the full solve.

  This is the *correct* way to put contact work on tensor cores — reuse one factor across many right-hand sides — and it contrasts sharply with `AGY` #4.
- **Limit.** The query type is rare: one-tick responses at a frozen contact state. That suits sensitivity probes and sampling MPC, not stepping. Use it as a diagnostic query, not a pillar.

### COD W05 · Branch a living world at the intervention, with causal recomputation — **540 · SALVAGE**

- **What it gets right.**
  - A correct Amdahl ceiling: 1/(0.05 + 0.95·0.02 + 0.01) = 12.66×, which misses 10× above 4.21% dirty work.
  - The right spill model: explicit finite-volume shallow water under its CFL bound.
  - Dense cascades are counted.
- **The premise breaks.** It assumes 95% of replay work is dependency-addressable. In robot-centric scenes, the robot and its contact neighborhood dominate compute and are *always* dirty after an intervention, while the far field is static and nearly free anyway.
- **Correctness surface.** Tracking dependencies across contacts, rays, and policy inputs is large, and "one omitted dependency kills."
- **Comparison.** `AGY` #10's full-replay branching is simpler and exact for robot-centric failures.
- **Salvage.** Restrict W05 to far-field multiphysics fields (multi-room spill and steam), where the dirty fraction really is small.

### COD FTA-P2 · Schedule by the earliest possible interaction — **480 · SALVAGE**

- **What it gets right.** Time-tiling L = 64 steps per launch attacks the part of the wall that is real: launch fragmentation in the per-topology baseline. `CC`'s §16.5 shows that launch overhead dominates there. The lookahead kinematics (1.5·0.256 + 0.256² = 0.4495 m vs 0.45 m available) and the ceiling 1/(1 − p + p/L + o) = 11.81× are exact.
- **Breaks.**
  1. Separate RL environments are already causally independent, so cross-env interaction calendars solve a non-problem.
  2. Within an environment, closed-loop policy inference couples every control tick, so L = 64 needs the policy fused into the tile.
  3. p = 0.94 "amortizable" is asserted without support.
- **Salvage.** Recast it as "per-topology graphs + in-kernel policy + time-tiled launches" and race it against warp packing.

---

## 2. Per-Item Critiques — `AGY`

### AGY #1 · Operadic pHS composition + warp-locked heterogeneous megakernel — **600 · SALVAGE (high ceiling)**

- **What it gets right.** A **limb-level Port-ABI.** Featherstone's inward pass contracts any subtree into (I^A ∈ Sym⁺(6), p^A) — 27 floats at its attachment port — so precompiled per-limb kernels compose additively at the base. That compile unit is sharper than `CC-01`'s joint bytecode: close to Tier-1 speed, with module-level reuse. It is also the most ambitious batching claim in the duel, and `CC` should adopt it as a "Tier 0.5".
- **Breaks:**
  1. **Occupancy arithmetic is wrong.** "12 concurrent warps/SM … maintaining ≥ 75% warp occupancy": Ada (`SM_89`) allows 48 warps/SM, so 12 is **25%**.
  2. **The 27-float summary is not lossless for contact.** It is exact for *unconstrained* forward dynamics. Legged robots have contacts on the limbs, and the Delassus operator J M⁻¹ Jᵀ couples all limbs through the base, so every contact iteration needs extra propagation (constrained ABA or inverse articulated inertia). That cost is unpriced (C001: "coupled contacts … can invalidate locality").
  3. **"0% intra-warp divergence" is false in practice.** Contact counts and solver iterations vary per env (C002).
  4. **"e-process CRN racing naturally requires K ≥ 32"** is false: CRN has no minimum K (C002).
  5. **Throughput comes from DRAM bandwidth alone** (22.9 MB/substep), applied to a latency-bound serial ABA chain.
  6. **"0 ms" holds only for re-wiring** among ≤ 16 compiled archetypes. A new archetype is a compile.
  7. **Rival claims are inflated.** MJX's "40–170 min of compile stalls" ignores caching and parallel compilation. Isaac's "per-actor CPU scatter-gather fallback" is unsupported. Isaac Lab's multi-asset views exist (C002 cites them).
  8. **Grounding gaps.** `fs-mbd` must be lifted, not consumed, because of `fs-qty` (E022). Proofs need `fln-kernel` reflection, not `fln-anvil` (E024).
- **To score higher.** Correct the occupancy, price the contact coupling, and keep the 70% kill gate, which is good.

### AGY #2 · Dimensionless Buckingham-Π universal policy + exact symbolic pullback — **570 · SALVAGE**

- **What it gets right.** The right architecture for zero-shot transfer: a morphology-agnostic task-space policy on top, with an exact per-body model below, so morphology dependence lives where the model knows it. The gate is tough and honest — Spearman ≥ 0.80 against specialist PPO, ≥ 75% return, ≥ 10× fine-tune — with a sensible fallback.
- **Breaks:**
  1. **The pullback is the wrong formula for legged robots.** τ = Jᵀ Λ(ẍ_des + K e + D ė) + C q̇ + g is *fixed-base* operational-space control. A floating-base legged robot is underactuated, and task forces must arrive through contact wrenches inside friction cones. That requires a contact-consistent WBC-QP, not a closed-form pullback.
  2. **Normalization ≠ dynamic similarity.** Normalizing by ℓ₀, m₀, t₀ does not give dynamic similarity unless *all* Π groups match (torque/weight, inertia distribution, μ) (C004).
  3. **The GPU policy does not exist yet.** `ft-nn`/`ft-autograd` are CPU-only, with `Arc<Vec<T>>` host tensors (E025, C004), so a "GPU-resident GNN with zero CPU round-trip" is a build item.
  4. **">22,000× screening" compares unlike things:** zero-shot *evaluation* against RL *training*.
  5. **The dual-4090 throughput is overstated.** "660 TFLOPS BF16" for the pair overstates the FP32-accumulate rate by about 2× (≈ 165 dense TFLOPS per card per NVIDIA's Ada whitepaper).
- **Better lower layer.** `COD` W03 is the rigorous version.

### AGY FTA-P3 · Assume-guarantee quotient verification + KKT shield + conformal-e guard — **570 · ADOPT-NARROWED**

- **What it gets right.** A three-layer split — proof on a quotient, KKT shield, anytime-valid assumption monitor — that revokes the formal badge when an assumption is breached. The falsifier is concrete: 5,000 in-envelope + 5,000 out-of-envelope episodes with four kill clauses. This is **the same resolution `CC` reached independently in TA-P3**, so neither lane deserves novelty credit over the other.
- **Breaks:**
  1. **Claim-axis slip.** Layer 3 labels a *simulated* monitor's detection rate `ValidatedPhysics`. Only real-hardware data earns axis 3; in simulation it is `EvaluatedDesign`.
  2. **A statistical monitor cannot enforce a hard bound.** A 60 ms window cannot enforce a hard disturbance bound (C006).
  3. **"Provably passive IDA-PBC … safe for every μ ≥ 0" overclaims.** Ḣ ≤ 0 is not safety: a robot can tip over while dissipating energy.
  4. **The zero-violation result needs its bound stated.** 0/5,000 bounds the rate at ≈ 6×10⁻⁴ (rule of three).

### AGY #10 · GPU Pearl-causal counterfactual time travel + `franken_threed` dossier — **565 · ADOPT-NARROWED**

- **What it gets right.**
  - Exact Shapley values over 10 binary interventions using 2¹⁰ = 1,024 deterministic branches. C008 confirms exact enumeration is valid at this size.
  - A 90% top-1 accuracy gate on planted causes.
  - **An agent-level gate:** ≥ 3× fewer repair iterations than observational autopsy. It is honestly 3×, not a fake 10×.
  - The highest human and agent utility in the `AGY` slate.
- **Breaks:**
  1. **A false FACT in the header.** It says `f3d-scene`, `f3d-compiler`, `f3d-gpu`, and `f3d-assets` are present. The `franken_threed` workspace members are `f3d-core`, `f3d-graph`, `f3d-math`, and `f3d-runtime` only (verified; C015). The single-file dossier is an adapter that must be built.
  2. **Snapshot size contradicts AGY #9.** "< 5 KB per-env snapshots" conflicts with #9's 1.41 MB/env multiphysics tiles.
  3. **Morphology `do()` interventions depend on #1** working.
  4. **"120 FPS" is unsupported.**

### AGY #3 · Contact-stratified hybrid barrier tubes + closed-form KKT GPU shield — **560 · ADOPT-NARROWED**

- **What it gets right.** It puts guard-jump invariance h_{S′}(Δ(x, μ)) ≥ 0 into the synthesis, and its four-part kill gate is well built. It overlaps `CC-06` and `COD` W04 and should be merged with them.
- **Breaks:**
  1. **The proof path relies on tooling that is not built.** It routes rational Gram projection through `fsym-formal`, which supports two univariate ZZ factor-product profiles only (E023). `fln-anvil` norm_num is a stub (E024). A reflective SOS checker in `fln-kernel` must be built (C006). *`CC-06` has the same gap* (§4).
  2. **The arithmetic is undefined.** There is no floating-point rounding margin and no sampled-data (inter-sample) invariance.
  3. **"~110 FMAs" undercounts.** For a 29-DoF humanoid, aⱼᵀu and AAᵀ alone cost ≈ 4·29 + 10·29 ≈ 400 FMAs before 16 active-set solves, and the barrier gradients L_g h need the dynamics model.
  4. **"Safe for any μ ≥ 0" overclaims**, for the same passivity ≠ safety reason as FTA-P3.

### AGY #6 · Exact Floquet–Poincaré implicit differentiation + bifurcation atlas — **550 · ADOPT-NARROWED**

- **What it gets right.** The core mathematics is correct: the saltation matrix, the implicit function theorem on the cycle, multiplier sensitivities w_iᴴ (∂M/∂θ) v_i, grazing detected as a typed event, and a cosine ≥ 0.95 gradient gate.
- **Breaks:**
  1. **The scale check is about 65× optimistic.** It prices only 48³ tangent GEMMs (44.2 MFLOPs per morphology). The Jacobian ∂x_{k+1}/∂x_k of each step through ABA and the contact solve costs about 36 JVPs of the full step, and that is omitted. `CC`'s estimate for the same workload — about 45k lane-steps per orbit solve, at the dual-4090 substep rate — gives about **2.4 s for 256 orbits, not 37 ms**. AGY's own 500 ms gate would fail.
  2. **Axis slip.** A floating-point Floquet spectrum is labeled `VerifiedNumeric` without enclosures.
  3. **">5,000× vs RL" compares unlike things:** a local orbit solve against training a policy.
  4. **It misses an existing crate.** `fs-spectral::floquet` already exists (E021, verified), and AGY does not use it.

### AGY FTA-P2 · Warp-cohomogeneous operadic port-graph super-batching — **500 · SALVAGE**

- **Same mechanism as #1**, with the same occupancy and contact-coupling errors.
- **Its central "hidden assumption" rests on a false premise** — that statistics require K ≥ 32 CRN episodes per morphology (C002). Warp-per-morphology is justified by SIMT width, not by statistics.
- **What it gets right.** The falsifier compares against *both* a homogeneous baseline and a dynamic-tree interpreter (≥ 4×, ≥ 70%). That is a good design, and `CC`'s `XT-P2` uses it too.

### AGY #8 · Adversarial differentiable scene grammar + `fs-topo` red-team architect — **455 · SALVAGE**

- **What it gets right.**
  - Anti-cheat legality priors: μ ≥ 0.09, OSHA hallway ≥ 75 cm, door closer ≤ 22 N·m.
  - A β₀ solvability gate.
  - A "≤ 2% unsolvable" kill condition.
  - A gradient-free fallback.
- **Breaks:**
  1. **No gradient signal.** The gradient of a binary failure event is zero almost everywhere. Without a declared smooth failure margin, ascent through contact plus ray-traced steam and glass has nothing to follow. "A 1-in-10,000 trap in < 2 s via 30 steps" assumes a smooth basin leading to the trap.
  2. **β₀ connectivity is not task feasibility** (C010).
  3. **Differentiable optics through steam and glass** is a large unbuilt piece of `frob-adjoint`/`frob-sensors`.
- **Recommendation.** `CC-03`'s cross-entropy search + `ddmin` shrinking + capability-envelope solvability is the more executable version. Merge AGY's legality priors into it.

### AGY #5 · M5 Ultra 256 GiB unified-memory zero-copy co-processor symbiosis — **410 · SALVAGE**

- **What it gets right.**
  - A concrete residency map: 96 GiB of buildings + 64 GiB of replay + 16 GiB of simulation + 16 GiB of CPU heap.
  - The 60 ms PCIe reload, compared with a pointer switch, is a real cost of discrete VRAM.
  - CPU bandwidth is rate-limited to ≈ 0.125% of unified-memory bandwidth.
- **Breaks:**
  1. **The gate never tests the 10×.** It checks only latency, stall, and throughput degradation; the claimed lever, generalization across 64 buildings, goes unmeasured.
  2. **The handoff latency is not a Metal guarantee.** "< 0.8 µs lock-free handoff" on live `StorageModeShared` buffers fails because GPU writes become visible to the CPU only at command-buffer completion or a shared-event signal (tens of µs). Today's gateway also waits synchronously, binds at offset 0, and has no `MTL4*` residency sets (E025). As written, the design would likely trip its own KILL clause 1 (> 10 µs).
  3. **Working-set fit is unchecked.** 176 GiB of GPU-resident data must fit under the probed `recommendedMaxWorkingSetSize`.
  4. **">60,000× faster" is a pointer-vs-copy ratio**, not a robotics lever.
- **Overlap.** `CC-05` is the same thesis with a diversity gate. Both need that gate run.

### AGY #7 · Study-quadric / Gröbner inverse synthesis + certified feasible polytopes — **380 · SALVAGE synthesis / REJECT polytope rule**

- **What it gets right.** Per-loop Gröbner/RUR synthesis capped at ≤ 6 variables and degree ≤ 4 fits inside `real_rur`'s caps, so Burmester-class problems are executable now.
- **Breaks:**
  1. **"Covers 100% of planar/spatial 4-bar, 5-bar, 6-bar … synthesis" is false.** The 9-point four-bar has 4,326 isolated solutions, whose univariate degree far exceeds `MAX_DEGREE = 256`. Six-bar path synthesis is larger still. That is exactly why `CC-04` needs homotopy continuation.
  2. **The sampling rule excludes the best designs.** Sampling only inside a certified *inner* ellipsoid cuts off the feasibility boundary, where optimal designs usually sit (active torque and clearance limits). "100% preflight pass" buys conservatism, not good designs.
  3. **The 1.38% figure rests on an invented assumption.** 0.70¹² = 1.38% assumes twelve independent 70% constraints, with no basis.
  4. **Unsupported sibling claims.** `fsym-modular` is asserted without evidence, and `fln-anvil` is stubbed.
  5. **A variant was already rejected.** Audit C007 rejected the two-sided posynomial version of this design sandwich.

### AGY #9 · Sparse active-tile Dirac-coupled living multiphysics — **360 · SALVAGE (Housner) / REJECT (50 Hz LBM)**

- **What it gets right.**
  - It reuses crates that exist: `fs-lbm`, `fs-solid::rod`, `fs-convection`, `fs-conform` (verified).
  - Uncalibrated materials are tagged `Estimated`.
  - It has Lamb–Housner slosh and cord-tension known answers.
  - It has a smart multi-pendulum Housner fallback.
- **Breaks:**
  1. **The LBM is unstable at the stated parameters** (recomputed). A 32×32×16 tile over a ≤ 1.6 m radius gives dx ≈ 10 cm. At dt = 20 ms, the lattice sound speed is c_s ≈ 2.9 m/s, so a 1 m/s flow already runs at **Ma ≈ 0.35** — above its own 0.25 refusal threshold. Water's relaxation gap is **τ − ½ ≈ 6×10⁻⁶**, which is BGK-unstable. A 1 mm grease film cannot exist on 10 cm cells, and a pot-scale 1 cm grid at the same dt runs at Ma ≈ 3.5.
  2. **Too slow for RL.** Its own target, 2,000 env-steps/s across 1,024 envs, is about 2 steps/s per env: fine for evaluation, not for training.
  3. **Lifts needed.** `fs-lbm`, `fs-solid`, and `fs-tribo` need `fs-qty` lifts (C011).
  4. **A rival claim is wrong.** "Genesis is CUDA-only" contradicts Genesis's documented Apple Metal backend (confirm in `GATE-RIVAL-AUDIT`).
- **Recommendation.** Use `fs-lbm::film` or a 2.5-D field (`CC-08`) for training, and full LBM for evaluation only.

### AGY FTA-P1 · Two-stage staged algebraic erasure — **310 · REJECT AS A THIRD ALTERNATIVE**

- **It is not a third alternative.** "Erase all algebra before runtime" *is* the QoR's own stated resolution hypothesis (algebra only at compile and synthesis time), so nothing is forced.
- **The falsifier is confounded.** One arm bundles CSE, equivariance, shields, and proofs, so neither a pass nor a fail can be attributed to any of them.
- **"Algebra makes the runtime strictly faster than hand-written loops" is unsupported.** It leans on GB-05 flop counts (18k → 4.8k). The env-step is latency-bound, so a flop reduction does not imply ≥ 1.8×.
- **What it gets right.** Proof erasure (`Prop` → 0 bytes) is a clean framing, and franken_threed §16 paired-block ablations are the right protocol.

### AGY #4 · Tensor-core & simdgroup-matrix mixed-precision Delassus solver — **220 · REJECT AS STATED**

- **Breaks:**
  1. **The precision guarantee is false.** The "provably bounded κ(W̃) ≤ 100" fails. With ρ = 10⁻²·tr(W)/(3m_c) and λ_max ≤ tr(W), κ ≤ 1 + 3m_c/10⁻² = **4,801** at m_c = 16, attained by a rank-one W (C009, recomputed). The Carson–Higham contraction argument κ·u ≤ 0.049 therefore fails by about 48×.
  2. **It solves the wrong problem.** Refinement uses r = b − (W + ρI)λ, so it converges to the *regularized* problem's solution, not the contact solution, unless outer proximal iterations run. "8 ADMM matvecs" are far too few for 10⁻⁵ complementarity.
  3. **Amdahl caps the speedup.** A 48×48 tiled Cholesky is three dependent tile steps whose diagonal factorizations and TRSMs are scalar, which caps the tensor-core gain well below 8×. With FP32 accumulation, GeForce Ada runs FP16 tensor ops at half rate (≈ 165 dense TFLOPS per card), not "660" for the pair.
  4. **The baseline is invented.** The 68 µs PGS baseline has no measurement behind it.
- **What survives.** The m_c ≤ 4 scalar crossover rule is sensible. `COD` W10 (multi-RHS factor reuse) is the correct form of this idea. `CC` killed the per-step version (`CC-16`) because the step is latency-bound.

---

## 3. Cross-Lane Convergence — the Best-Executed Version of Each Shared Idea

| Converged idea | `CC` | `COD` | `AGY` | Best version, and what the merged plan should take |
|---|---|---|---|---|
| Heterogeneous-morphology batching | `CC-01`, TA-P2 | W02, FTA-P2 | #1, FTA-P2 | **No lane alone.** Combine: `AGY` #1's limb Port-ABI kernels as "Tier 0.5"; `CC`'s warp = morphology packing, race annealing, and an `XT-P2` that lets H2b (per-family templates) win; `COD` W02's operator/row-count compaction for the *contact* phase. |
| Saltation / Floquet gait sensitivities | `CC-02` | **W07** | #6 | **W07's** gate and honest ratios, extended with `CC-02`'s H/K gait enumeration and continuation. Reuse `fs-spectral::floquet`. |
| Shields + assumption monitors | `CC-06`, TA-P3 | W04, **FTA-P3** | #3, FTA-P3 | **`COD` FTA-P3** for impact and stop safety (no statistics needed). `CC` TA-P3 ≈ `AGY` FTA-P3 for general contact safety, with the type-level `ClaimProduct` from `CC`. W04's reset semantics as the reference. |
| Solvability-guarded red-team worlds | **`CC-03`** | W18/W24 (cut) | #8 | **`CC-03`** (cross-entropy search + `ddmin` + capability-envelope solvability) with `AGY`'s legality priors. Gradients only as an optional proposal source. |
| Counterfactual branching | `CC-03` autopsy | W05 | **#10** | **`AGY` #10**: full-replay exact Shapley at 2¹⁰ plus an agent-iteration gate. W05 only for far-field fields. |
| Linkage synthesis | `CC-04` | **W06** | #7 | **W06** first (within `real_rur` caps today); `CC-04` homotopy beyond the caps. Drop `AGY` #7's inner-ellipsoid rule. |
| Mission → build packet | `CC-10` | **W09** | #11 (cut) | **W09's** reviewer-accepted gate with `CC-10`'s STL spec-as-reward. |
| Lifecycle compression | `CC-09` | W17 (cut) | #12 (cut) | All three agree it stays `Estimated` until calibrated. Reuse `fs-solid::fatigue` and `fs-contact` wear (E022). |

---

## 4. Self-Calibration — Where Repo Facts and Opponents Beat `CC`

Applying the same standard to `CC`'s slate:

1. **The `fln-anvil` egraph and norm_num are stubs** (E024; verified — `lib.rs` has only `EGraphConfig`/`EGraphOutcome` types, and the crate is 1,423 LOC total). `CC` labeled them `FACT` and built four witnesses on them: `CC-01`'s Tier-1 `PO-CODEGEN-1`, `CC-06`'s `PO-SHIELD-EQ-1`, `CC-04`'s `PO-ALPHA-1`, and `CC-10`'s `PO-CODEGEN-FW-1`. All four must move to `fln-kernel` computational reflection plus `omega`, and e-graph witnesses become a build item. Under this rubric, each of those four `CC` ideas would lose about 40–80 points on X and H.
2. **`fsym-formal` cannot carry multivariate Gram certificates** (E023). `CC-06` has the same gap as `AGY` #3.
3. **`COD` W02 caught a real `CC-01` overclaim.** Same-morphology warps still diverge on contact counts and solver iterations; only `CC-01`'s opcode branches are uniform.
4. **`COD`'s rival table is more accurate than `CC`'s.** It covers MJWarp per-world assets, Isaac Lab multi-asset views and Newton heterogeneous worlds, and Genesis checkpoints. `CC-01` (b) understated how much partial heterogeneity the incumbents already have.
5. **`CC` under-used existing sibling capabilities:**
   - `fs-spectral::floquet` for `CC-02`;
   - `fs-solid::{rod, fatigue}` and `fs-contact` wear for `CC-09`. `CC`'s note G-4, "no `fs-fatigue`/`fs-cosserat`," was true of crate names but misleading about capability;
   - `fs-lbm::film` and `fs-tribo::resistive_film` for `CC-08`;
   - `fs-conform` for the `TA-P3` monitor.
6. **`fs-mbd` needs an `fs-qty` lift** (E022), so `CC-01`'s CPU oracle cannot be consumed directly.
7. **`franken_threed` single-file packaging is planned, not built** (C015), so the `CC-03` and `CC-10` dossiers are adapters to build.
8. **The Metal gateway is synchronous and binds at offset 0** (E025). `CC-05`'s zero-copy CPU-while-GPU work and `CC-01`'s two-dispatch Metal claim both need gateway work first.
9. **`COD` FTA-P3 beats `CC` TA-P3** for impact and stop-type safety (see above). **`COD` FTA-P1's ballast fixture** is a cheaper, sharper potency test than `CC`'s `XT-P1`.

## 5. What the Merged Plan Should Take From Each Opponent

**From `COD`:**
- FTA-P3 energy-budget proof obligations as a new PO family.
- W07 as the gait-sensitivity gate protocol.
- W03 capability maps with infeasibility witnesses as the Studio's refusal semantics.
- W08 configuration-space certificates for interior fit.
- W02 operator compaction for the contact phase.
- FTA-P1's ballast fixture.
- **COD's measurement protocol, program-wide:** a paired 95% lower confidence bound ≥ 10×, refusals and failures counted in the denominator, and competent incremental baselines.

**From `AGY`:**
- #1's limb Port-ABI.
- #2's task-space universal layer, with the pullback replaced by a contact-consistent WBC-QP.
- #10's exact-Shapley branching and its agent-iteration gate.
- #5's residency map, once its gate tests generalization.
- #9's Housner slosh fallback.
- #8's legality priors.

**Reject:** `AGY` #4 as stated; `AGY` FTA-P1 as a "third alternative"; `AGY` #7's inner-ellipsoid sampling rule; `AGY` #9's 50 Hz LBM.

## 6. Verification Receipts for This Scoring Pass (`2026-11-21`)

| Check | Result |
|---|---|
| `COD` FTA-P1: enumerate C(24, 8) subsets and count distinct (Σx, Σx²) | 735,471 → **24,546** classes; ratio **29.96×** ✓ |
| `AGY` #4 κ bound: rank-one W, ρ = 0.01·tr(W)/48 | κ = **4,801**, not ≤ 100 ✗ |
| `AGY` #1 occupancy: 12 warps out of 48 max per Ada SM | **25%**, not ≥ 75% ✗ |
| `AGY` #9 LBM at dx = 0.10 m, dt = 0.02 s | c_s = 2.887 m/s, Ma = 0.35 at 1 m/s, τ − ½ = 6×10⁻⁶ ✗. At dx = 0.01 m, Ma = 3.46 ✗ |
| `COD` W01, W04–W10, FTA-P2, FTA-P3 arithmetic | All re-derived ✓, including 10.67×, Q = 60,000, 12.66× / 4.21%, 15.39×, 13.61×, 14.69×, 11.54×, 13.6× / 97.1%, 0.4495 m / 11.81× / 92.44%, and 1.49 J / 50.5 ms / 45.5 ms. One wording nit in W06: its "51.2 s" threshold is the reference total including the 2 s of acceptance checks, so the bare multistart baseline would need to be ≥ 49.2 s. |
| `franken_threed/Cargo.toml` workspace members | `f3d-core`, `f3d-graph`, `f3d-math`, `f3d-runtime` only; the other crates are commented as "Planned" ✓ (C015) |
| `fln-anvil` source | 698 + 725 LOC. `egraph` and `norm_num` appear only as config/outcome types and doc comments ✓ (E024) |
| `fsym-solvers/src/real_rur.rs` caps | `MAX_VARIABLES = 8`, `MAX_EQUATIONS = 64`, `MAX_DEGREE = 256`, `MAX_BITS = 4096`, `MAX_COMPUTE = 200_000_000` ✓ (E023) |
| `frankensim` modules | `fs-spectral/src/floquet.rs` exists; `fs-lbm/src/{film, freesurface, d3q19, d3q27}.rs` exist; `fs-solid/src/{rod, fatigue}.rs` exist; `fs-conform` and `fs-convection` exist ✓ (E021/E022) |
| Not verified here (to `GATE-RIVAL-AUDIT`) | Genesis Apple Metal backend; GeForce Ada FP32-accumulate tensor rate (≈ 165 dense TFLOPS per 4090 per NVIDIA's Ada whitepaper); MJWarp and Isaac Lab Newton heterogeneity details |

*End of `CC` Phase-5 cross-scoring.*
