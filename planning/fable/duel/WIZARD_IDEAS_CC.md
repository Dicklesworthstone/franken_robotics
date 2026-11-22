# WIZARD_IDEAS_CC.md: thirty ideas, five survivors (FrankenRobotics, ten-fold pass)

| | |
|---|---|
| Author | Claude Opus 5.5 (`claude-opus-5-5`) in Claude Code, working alone |
| Date | 2026-11-21 |
| Target | `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` v0.2 (3,666 lines), read in full |
| Other inputs | `planning/fable/mor/MOR_CASE.md`; sibling source in `frankensim` and `frankentorch` (files named where used); five public web searches (§6) |
| Not read | Other agents' plans and idea files (`__NEON`, `__ASTRA`, `planning/duel/`, `planning/mor/`, `planning/brenner/`) |
| Labels | FACT, HYPOTHESIS and TARGET as in plan §0.1. Web facts come from search-result summaries of the cited pages; I did not open the pages, so each MUST be confirmed before a gate cites it. |

---

## 0. The yardstick: what "tenfold" means here

### 0.1 Raw stepping speed is not the lever

- FACT (web): the MJX documentation reports about 2.67 M humanoid steps/s on its Warp path with launch-graph
  capture, and 0.80 M when the graph is re-captured every step (hardware as stated on that page).
- FACT (plan §28.1): FrankenRobotics targets at least 500 k humanoid world-steps/s on one RTX 4090.
- FACT (plan §19.8, §28.1): GATE-CORTEX-05 targets at most 15 minutes to train the reference quadruped on one
  RTX 4090. MuJoCo Playground reports about 5 minutes on two (MOR E011).

A tenfold gain on world-steps/s over these engines is not on offer, and the plan should stop measuring itself
there. The tenfold has to come from the **cost of a design decision**, which is what an agent actually buys.

### 0.2 Three metrics

| Metric | Definition |
|---|---|
| **CPD**, cost per established decision | Wall-clock and device-seconds on `trj` to decide, at α = 0.05, whether design B beats design A on task T. Both designs run under a competent controller, and the verdict is checked against the referee tier. |
| **Breadth** | Distinct designs evaluated per GPU-hour under a competent controller on a stated battery. |
| **Turns-to-fix** | Serial agent turns from a failing design to a passing one on the planted-defect corpus of plan §21.3. |

### 0.3 Two baselines, so the claim is not made against a straw man

- **B0, the common pipeline.** MuJoCo Playground on MJX or MuJoCo Warp, with one compiled model per design, PPO
  per design, and at least five seeds (the plan's own protocol, §19.7). From E011, a quadruped training run takes
  about 5 minutes on two RTX 4090s. One pairwise decision therefore costs about 2 designs × 5 seeds × 5 min =
  50 minutes of `trj`, with no check against any more trusted tier. Breadth is about 12 designs per hour on `trj`
  at one seed each, or about 6 per GPU-hour.
- **B0⁺, the strongest assembly an agent could build today with a thin layer.** B0 with PPO replaced by
  training-free sampling MPC, and Newton's heterogeneous worlds where its solvers allow them. The pieces:
  - DIAL-MPC runs on MJX and needs no training (FACT, web, arXiv 2409.15610).
  - MJX recompiles when structural model fields change (FACT, web). One user reports about 24 s of JIT time
    for a humanoid (an anecdote from a GitHub issue).
  - MuJoCo Warp "does not support heterogeneous worlds yet": only the values of batched fields may differ
    between worlds, never the degrees of freedom (FACT, web).
  - Newton allows different robots per world, but its documentation calls this "quite experimental", and not
    every solver supports it (FACT, web).
  - None of these has a second, more trusted physics tier sharing the model and the random draws.

### 0.4 What the five ideas claim, stated honestly

- Against **B0**: CPD at least 10× lower and breadth at least 100× higher (TARGET).
- Against **B0⁺**:
  - Breadth at least 10× higher under a learned controller (TARGET).
  - An *established* verdict, checked on the referee, which B0⁺ cannot produce at any price.
  - **No tenfold claim for a single provisional pairwise decision.** There I expect B0⁺ to land within about
    3× of FrankenRobotics (HYPOTHESIS). The plan should not claim more than that.
- Idea 25, a design-decision benchmark run on `trj` against both baselines, is the instrument that settles all
  of these numbers. I adopt it as the yardstick rather than ranking it.

**Where the time goes after the five.** Once compute per decision is seconds, the agent's own think time per
serial turn dominates (about 30–120 s, HYPOTHESIS). That is why idea 1 makes verbs act on whole populations and
idea 5 cuts the number of turns.

## 1. How I winnowed

An idea survived only if it met all five tests:

1. It is a mechanism with a named algorithm and data layout, not a slogan.
2. It fits two 24 GB RTX 4090s, 256 GB of unified memory, and the CPU cores that exist.
3. An incumbent engine with a thin wrapper does not already deliver it. I checked this against documentation
   where I could, and say where uniqueness is only partial.
4. It uses sibling code that exists today, or says what must be built.
5. It moves one metric of §0.2 by about 10×, or makes a decision possible that was not possible before.

The top five are ordered by value × confidence × uniqueness, with dependencies counted. Idea 1 comes first
because ideas 3 and 5 need it, it is cheap now, and it is expensive to retrofit.

---

## 2. All thirty

Each entry gives the mechanism (**Mech**), the lever, the falsifier with its fallback, what the sibling projects
must supply (**Needs**), and the verdict.

1. **Ragged cohorts: one launch steps thousands of different robots.** Top five; see §3.1.
2. **Sampling MPC that plans on each design's own model, nested inside the batch.** Top five; see §3.2.
3. **One policy per family of robots, taught by MPC and carried across typed edits.** Top five; see §3.3.
4. **Paired multilevel estimation across tiers.** Established verdicts at about a tenth of the referee cost.
   Top five; see §3.4.
5. **Counterfactual probes: edit the robot at the moment it fails.** Top five; see §3.5.
6. **Searching scene grammars for rare failures.**
   - Mech: score each run by a signal-temporal-logic robustness value, then search grammar and terrain parameters
     by subset simulation or cross-entropy importance sampling (`frob-arena`, `frob-scene`), on common random
     numbers keyed by world address.
   - Lever: estimating P(fail) ≈ 10⁻⁴ to 10% relative error takes about 10⁶ plain Monte Carlo episodes against
     about 10⁴ (HYPOTHESIS, the standard subset-simulation scaling), roughly 100×. It also returns the worst
     scenes as fixtures for Explain.
   - Falsifier: on planted rare failures (a 0.5% slick patch), its error exceeds plain Monte Carlo's at equal
     cost. Fallback: Monte Carlo plus CVaR (§19.5).
   - Needs: `fs-uq` already does defensive Gaussian importance sampling with confidence sequences (FACT,
     `fs-uq/src/product_importance.rs`), but on normal germs, so grammar parameters need a map to normal germs.
   - Verdict: strong, ranked sixth. Tail risk matters once the design loop is fast, not before.
7. **`frob solve --spec`: one verb from a specification to an evidenced Pareto set.**
   - Mech: compiles task, constraints and budget into carve (8) → cohort sweep (1) under the family policy or
     MPC (2, 3) → racing (10) → multilevel promotion (4). It streams progress and returns a Pareto set with
     verdicts.
   - Lever: about 5× fewer serial agent turns per accepted design (HYPOTHESIS).
   - Falsifier: in a BET-10-style A/B test, agents with `solve` do no better than agents driving the verbs by
     hand.
   - Needs: nothing new.
   - Verdict: the right surface for the top five, but orchestration rather than a lever of its own.
8. **Carving the design space with interval and monotone certificates.**
   - Mech: evaluate A-02, A-08, A-10 and A-11 over parameter boxes, not points, using `fs-ivl` intervals,
     affine forms and Taylor models. Monotone co-design (C-12) makes corner evaluation exact for monotone maps.
     Bisect into feasible, infeasible and undecided boxes (`frob-analyze`).
   - Lever: a 10⁴-point grid decided by about 10² box evaluations when the boundary is smooth (HYPOTHESIS).
   - Falsifier: under 50% of the box volume decided after eight bisection levels on the corpus. Fallback: point
     checks.
   - Needs: `fs-ivl` (FACT: intervals and affine forms, plus `taylor.rs` and `multi_taylor.rs`); FrankenSymPy
     interval certificates for up to 16 coordinates (FACT, E008).
   - Verdict: cut from the top five. A point `design check` already costs at most 2 s, so carving pays only
     inside population search, and its value rides on BET-05.
9. **The Mac as the scene-breadth tier.** This extends plan §28.4.
   - Mech: keep 100 or more full-resolution interior packs resident in 256 GB (75–600 MB each, GB-07), so every
     world in a batch can sit in a different building. A 4090 holds about 10–30.
   - Lever: about 10× more distinct scenes per batch (HYPOTHESIS), which matters for held-out evaluation and
     for idea 6.
   - Falsifier: KILL-METAL fires. Fallback: the Mac keeps only replay and referee roles.
   - Needs: `ft-kernel-metal` allocates shared-storage buffers on unified memory but is limited to 31 buffer
     arguments (FACT, E007), so scene tables must be packed.
   - Verdict: a good use of this hardware, but secondary.
10. **Choosing the next experiment by whether it could change the ranking.**
    - Mech: send the next episode wherever `fs-voi`'s ranking-flip probability is largest, and prune losers with
      `fs-race`.
    - Lever: 2–4× fewer episodes for best-of-N than uniform racing (HYPOTHESIS).
    - Falsifier: under 1.5× on the benchmark.
    - Needs: `fs-voi::ranking_flip_probability` and `fs-race` (FACT).
    - Verdict: cut. Successive halving around any engine gets most of this.
11. **A global evaluation cache keyed by content identity.**
    - Mech: every result is cached under (ModelId, SceneId, TaskId, ControllerId, WorldAddr, tier, kernel
      versions) in the Chronicle ledger. Designs are canonicalized modulo their symmetry group (24) before
      lookup.
    - Lever: removes repeated swarm work, perhaps 20–40% of it (HYPOTHESIS).
    - Falsifier: a hit rate under 10% over a week of swarm use.
    - Needs: frankensqlite, already FD-FSQL.
    - Verdict: adopt it because it is cheap. Not a tenfold.
12. **Gait-cycle adjoints for design gradients.**
    - Mech: an adjoint of the gait's once-per-cycle return map. It differentiates through the strictly convex
      compliant contact solve by the implicit-function theorem (C-06), under a smooth baseline controller, with
      revolve checkpointing.
    - Lever: the full gradient of cost of transport with respect to 40 parameters for about 3 cycle costs,
      against 80 for central differences (about 25×, HYPOTHESIS).
    - Falsifier: adjoint and finite differences disagree by more than 10% on the templates, or there is no stable
      periodic gait to differentiate. Fallback: CMA-ES (`fs-dfo`).
    - Needs: the `fs-adjoint` pattern with revolve checkpointing (FACT). `fs-mbd`'s articulated API is plain
      `f64` and not generic over `fs-ad`'s `Real` (FACT: `fs-mbd/src/articulated.rs`), so this works only on the
      batched kernels derived by `frob-kir`.
    - Verdict: high ceiling but fragile. Sampling MPC and learned policies are not smooth controllers. Defer to M5
      or later.
13. **Evolving robot topologies with MAP-Elites, using ragged cohorts.**
    - Mech: an `fs-archive` grid over mass, speed, energy per metre and footprint. Mutations are typed edits,
      including `add-limb` and `remove-limb`, and each generation is one cohort (1) under the family policy (3).
    - Lever: distinct topologies explored per GPU-hour; this follows from ideas 1 and 3.
    - Falsifier: no archive cell beats the templates.
    - Needs: `fs-archive` (FACT: MAP-Elites and CVT).
    - Verdict: the main application of ideas 1–3, not a separate lever.
14. **A CPU shadow auditor.**
    - Mech: idle CPU cores (36 on the Mac; `trj`'s when it is not loaded) keep re-running randomly sampled GPU
      worlds on the referee from their snapshots and stream fidelity-ledger rows.
    - Lever: fidelity data and the pilot variances idea 4 needs, at no added wall-clock time.
    - Falsifier: the referee runs below the 0.2× real-time target of §28.1, so too few shadows finish per hour to
      estimate correlations.
    - Needs: the `fs-mbd` referee and `fs-exec` tiles (FACT).
    - Verdict: folded into idea 4 as its source of pilots.
15. **Passability maps for a robot in a given building.**
    - Mech: compute SE(2) configuration-space obstacles by dilating the scene distance field with the robot's
      footprint, one heading slice at a time on the GPU, then check reachability over the route graph. The answer
      is which doors, aisles and galley turns the base can pass.
    - Lever: impossible wheelbases are pruned in seconds per building, before any dynamics.
    - Falsifier: more than 10% disagreement with simulated traversal.
    - Needs: `fs-rep-sdf` (plan FACT).
    - Verdict: useful for wheeled and tracked bases, but narrow.
16. **One fused kernel per world for homogeneous training.**
    - Mech: `frob-kir` emits one kernel per step, specialized to a single topology, with one warp (CUDA) or one
      SIMD-group (Metal) per world and the state held in registers and threadgroup memory.
    - Lever: 2–4× more world-steps/s than the ten-stage launch plan (HYPOTHESIS).
    - Falsifier: under 1.5× at the M2 profile.
    - Needs: nothing from the siblings.
    - Verdict: cut from the top five. Raw stepping speed is not the lever (§0.1), and specializing to one topology
      fights idea 1. Keep it as an option for long homogeneous training runs.
17. **Identifying actuators from logs.**
    - Mech: a posterior over armature, Coulomb and viscous friction and delay per joint, fitted from logged
      commands and encoder traces by batched likelihood over GPU rollouts. The posterior becomes the card
      uncertainty that drives randomization (§19.5).
    - Lever: actuator dynamics are the largest source of sim-to-real error (E013: 46% more travel distance after
      identification).
    - Falsifier: identified cards fail to cut held-out trajectory error by at least 30%.
    - Needs: `fs-assimilate` does linear-Gaussian Kalman fusion only (FACT), so nonlinear friction needs new
      code.
    - Verdict: cut for now. No hardware or logs are in scope (MOR §1).
18. **A learned correction from the fast tier to the referee, used only inside conformal bands.**
    - Mech: a small residual model predicts the per-step referee-minus-GPU difference from paired data, and is
      applied only inside a split-conformal band.
    - Lever: it would raise the tier correlation ρ that idea 4 depends on.
    - Falsifier: an out-of-band rate above 10% on the held-out battery.
    - Needs: `fs-surrogate::conformal_band` (FACT).
    - Verdict: cut. A learned patch on the fast tier hides exactly the artifacts the trust ladder exists to
      expose.
19. **Reporting gaits by symmetry mode.**
    - Mech: project joint trajectories onto the isotypic components of the design's symmetry group (C-09) and
      report energy per component.
    - Lever: in one line instead of a trace, the agent sees which gait emerged and how much a payload broke the
      symmetry.
    - Falsifier: agents in an A/B test ignore it.
    - Needs: the `frob-algebra` projectors the plan already includes.
    - Verdict: cheap; adopt it. Not a lever.
20. **A controller-free "potential" score from batched trajectory optimization.**
    - Mech: centroidal or whole-body trajectory optimization per design, with the contact schedule taken from the
      gait atlas, solved by the cone solver.
    - Lever: designs can be compared without any controller at all.
    - Falsifier: Kendall τ against the ranking by trained policies is below 0.6.
    - Needs: the plan's cone solver.
    - Verdict: cut. Idea 2 gives a controller-independent score with far less fragility.
21. **Pooling evidence across agents with e-values.**
    - Mech: averaging the e-values from different agents' comparisons of the same pair gives a valid swarm-level
      e-value. Multiplying them is valid only when the comparisons are independent.
    - Lever: swarms stop re-running comparisons that are already settled.
    - Falsifier: shared world addresses break independence, forcing the average and a smaller gain.
    - Needs: `fs-eproc` e-value arithmetic (FACT).
    - Verdict: small; folded into idea 11.
22. **A learned dynamics model conditioned on morphology, for screening.**
    - Mech: a graph network over the kinematic tree that predicts the next state across a design population.
    - Lever: screening perhaps 100× cheaper per design (HYPOTHESIS).
    - Falsifier: rank correlation with simulated outcomes below 0.7.
    - Needs: on-device network kernels.
    - Verdict: cut. Learned contact dynamics are unreliable, and an incumbent with a thin wrapper can train the
      same model.
23. **Mixed precision inside the step.**
    - Mech: half-precision Jacobian assembly, a single-precision solve, and double-float accumulators.
    - Lever: 1.3–1.6× (HYPOTHESIS).
    - Falsifier: the precision battery (GATE-SPARK-05) fails.
    - Needs: nothing new.
    - Verdict: cut. It is polish, and it complicates the determinism classes.
24. **Never evaluating the same design twice under different names.**
    - Mech: a canonical form of the design term, taken modulo its symmetry group and port relabeling.
    - Lever: up to |𝔾|, so 2–8× on symmetric searches.
    - Falsifier: the duplicate rate is under 5%.
    - Needs: nothing new.
    - Verdict: folded into idea 11.
25. **A design-decision benchmark.**
    - Mech: a frozen set of decision questions with planted answers: the §21.3 corpus plus pairwise questions
      with known winners. Each runs through B0, B0⁺ and FrankenRobotics on `trj`, reporting CPD, breadth and
      turns-to-fix with the disclosure block.
    - Lever: it makes "tenfold" falsifiable (MOR PQ1).
    - Falsifier: none; it is the instrument.
    - Needs: Playground and DIAL-MPC installed on `trj`, out of process, as development-time oracles (plan §30.2
      rule 3 allows this).
    - Verdict: adopted as the yardstick (§0), not ranked.
26. **A multi-agent design-debate layer.**
    - Mech: several language-model agents propose and critique edits before anything is simulated.
    - Lever: published co-design work reports 18–35% gains (E014).
    - Falsifier: no gain on the benchmark.
    - Needs: nothing.
    - Verdict: cut. An incumbent with a thin wrapper delivers it, and the swarm already exists outside the
      product.
27. **Gradients through contact everywhere.**
    - Mech: differentiate the whole step.
    - Lever: unclear. MJX is already differentiable, and gradients through contact are known to be biased (the
      plan defers them to M6).
    - Falsifier: not needed.
    - Needs: `fs-mbd` is `f64`-only and not generic over `fs-ad` (FACT).
    - Verdict: cut as a slogan. Idea 12 is the narrow version that might work.
28. **Granular and deformable terrain at scale (MPM).**
    - Mech: a material-point solver for soil and debris.
    - Lever: none on decision cost.
    - Falsifier: not needed.
    - Needs: a solver that does not exist.
    - Verdict: cut. It exceeds the hardware for population batches, and Bekker–Wong (§15.6) already covers wheels
      and tracks.
29. **Live interactive editing in the viewer.**
    - Mech: edit the design while it runs at 60 Hz.
    - Lever: for humans.
    - Falsifier: not needed.
    - Needs: the viewer.
    - Verdict: cut. The users are agents.
30. **Video-to-splat scenes for training policies.**
    - Mech: Gaussian-splat scenes rendered from walkthrough video.
    - Lever: none on decision cost.
    - Falsifier: not needed.
    - Needs: a renderer the plan does not have.
    - Verdict: cut. RGB policies are out of scope until M5.

---

## 3. The top five

### 3.1 Ragged cohorts: one launch steps thousands of different robots

**What it is.** The plan's batched step (§14.3) runs copies of one `RobotModel` across worlds, and GOAL-02 reads
"thousands of worlds per device". This idea changes the unit of batching from one model to a **cohort**: up to a
few thousand *distinct* designs that share one compiled step plan. A design population then runs as one launch,
and adding a new design means writing table rows, not compiling.

**Mechanism.**

- **Cohort signature.**
  - The signature is (DoF bucket n_v ∈ {12, 18, 24, 32, 48, 64}, link bucket, maximum tree depth, contact-sample
    capacity, contact-row capacity, actuator count).
  - Kernels compile once per signature and kernel version, never per design. That is fewer compiles than the plan
    has today, where the cache is keyed by `ModelId`.
- **Model tables** live on the device and stay read-only during a rollout. They are indexed by a model slot m.
  - Per link:
    - parent index λ and depth;
    - joint placement as a packed SE(3) motor;
    - a screw basis S ∈ ℝ^{6×3} with a column mask. The mask holds k = 1 for revolute, prismatic and helical
      joints, k = 2 for cylindrical and universal joints, and k = 3 for spherical and planar joints, so C-01's
      subgroup classes become *data*;
    - spatial inertia (10 floats), limits, the actuator-card slot, and the range of the link's collision samples.
  - Per model: `level_start[m][ℓ]` over links sorted by depth.
  - Padding links are identity transforms with zero inertia, masked out of every sum.
- **World tables.** Each world has a model slot m(w), a scene slot and its address. Worlds of one model are
  contiguous, 16 or 32 per model so they fill whole warps. That keeps the level loop free of divergence inside a
  warp.
- **Dynamics.**
  - RNEA and CRBA run over levels ℓ = 0 … d_max of the cohort, one thread per (world, link in level). Each
    thread reads its model's level range from the table.
  - The joint map τ(θ₁S₁)·τ(θ₂S₂)·τ(θ₃S₃), with masked columns as the identity, is one generated kernel for
    every joint class, with no branch on joint kind. This is exactly where the plan's Lie algebra pays: the joint
    *is* its screw basis.
- **Solve.**
  - A batched dense Cholesky at the bucket size n_v.
  - Padded degrees of freedom get identity rows and a zero right-hand side, so they stay exactly at v = 0.
- **Contact.**
  - Sample lists are per model.
  - Capacity is per world, at the bucket's cap, and overflow is flagged as `Degraded`, as §14.3 already says.
- **Admitting a design.**
  - Compile the model on the CPU, reusing memoized subterms (INV-SKEL-02), then copy its rows, tens of KB, into
    a free slot.
  - A running sweep can admit or retire designs between episodes.
- **Replay.**
  - `WorldAddr` gains the cohort id and model slot.
  - Teleport replays one world on `batch-cpu` with that world's own model; the CPU never needs the cohort.

**What an agent sees.**

```
$ frob sim sweep --designs @archive:gen-14 --task stairs-0.15 --worlds-per-design 16 --tier gpu-cuda --json
  # 512 designs in 3 cohorts (DoF buckets 18, 24, 32); padding waste 17%; 8,192 worlds; 41 GPU-s
  # one envelope; a provisional scorecard per design
```

Agents will experience this as population verbs (`sweep`, `opt archive`, `solve`) that cost about the same as
one design did before. The risk they will feel is bucket fragmentation: a population that is too varied splits
into many small cohorts. The envelope reports padding waste and cohort count so that the cost is visible.

**The lever.**

- **Number.**
  - TARGET: at least 0.6× homogeneous world-steps/s at padding waste of 25% or less, for cohorts of at least
    256 distinct designs.
  - Breadth is then homogeneous throughput divided by the steps each design needs. At the plan's TARGET of
    2.0 M quadruped world-steps/s, a design with 16 worlds × 10 s at 500 Hz needs 80,000 steps, so a GPU-second
    of physics covers about 25 designs.
- **Baseline.**
  - MJX recompiles on any structural change (FACT, web).
  - MuJoCo Warp cannot batch different DoF counts (FACT, web).
  - Both therefore need one batch per design, and a 16-world batch uses a sliver of a device that saturates
    only near thousands of worlds (HYPOTHESIS, consistent with the plan's own 8,192-world target).
  - Newton allows different robots per world experimentally, and not with every solver (FACT, web).
- **Why it cannot be bolted onto them cheaply.**
  - In MJX and MuJoCo Warp the kinematic tree is structure baked into compiled code or into arrays every world
    shares. Making it per-world data changes their core representation, which is MOR PQ2's test.
  - Here it is cheap, because nothing is built yet and C-01 already makes a joint into data.
- **Honest limit.**
  - Ragged cohorts are only partly unique against Newton, so their tenfold does not stand alone.
  - It appears when they are combined with the family policy (§3.3) and counterfactual probes (§3.5), where each
    design gets few worlds and occupancy decides cost.
  - With sampling MPC (§3.2) each design already brings K× more planning worlds, so occupancy matters less
    there.

**Falsifier and fallback.**

- **Cheapest, at M1 with no GPU code.** Compute the padding waste of the design corpus (§21.3) and of a
  1,000-design sample from the grammar under the bucket set. If holding waste at or below 25% needs more than
  eight buckets, cohorts will fragment.
- **At M2.** On `batch-cpu` and CUDA, run 512 distinct 12–18-DoF designs against a homogeneous batch of the
  bucket's largest design with the same world count. Ragged below 0.5× of homogeneous fails.
- **Fallback.** Cohorts by exact tree topology: the same tree with different parameters, still with no compile
  per design. That is the MuJoCo Warp regime plus per-design rows.
- **Proposed kill gate.** KILL-RAGGED at M2.

**Sibling needs, checked.**

- **FrankenSim:** nothing new. The referee stays one design and one world at a time on the CPU. FrankenSim has no
  batched environments at all (FACT, E005), so nothing upstream assumes homogeneity.
- **`ft-kernel-metal`:** synchronous, at most 31 buffer arguments, `f32` and `u32` buffers only (FACT, E007).
  The model tables pack into three or four buffers with offset tables, which is the plan's interim posture
  already.
- **`cudarc` and NVRTC:** as planned.
- **`frob-kir`:** must emit masked loops over screw-basis columns. The plan owns this.

**Hardware fit.**

- A design's tables take about 20–50 KB, so 4,096 designs need about 200 MB.
- Per-world state is as in plan §28.4: a few KB, plus tens of KB of constraint rows.
- That gives 8,000–30,000 worlds per 24 GB device.

**Cost and placement.**

- Rewrites three beads:
  - BD-074: packed lowering becomes model tables.
  - BD-082: the step runs over cohorts.
  - BD-099: `VecEnv` gets a per-world model slot.
- About 15–25% more M2 kernel work if it is done now (HYPOTHESIS). Retrofitting after M2 touches every kernel.
- GOAL-02 becomes "thousands of worlds of up to thousands of different designs per device", and §28.1 gains a
  ragged row.

**Confidence.**

- High for the mechanism: padded level schedules and masked dense factorizations are standard batched-GPU
  technique.
- Medium for holding 0.6× throughput, which the M2 benchmark settles.
- What would change my mind: if contact capacity rather than DoF dominates padding (rim-sampled wheels beside
  point feet), buckets must be two-dimensional and may fragment.

### 3.2 Sampling MPC that plans on each design's own model, nested inside the batch

**What it is.**

- Today the plan's day-one controller (§18.3) is a synthesizer chosen per morphology class: a gait table, a
  force-distribution program and capture-point stepping.
- Whether it generalizes is BET-06 and the MOR's open unknown: does a zero-training controller exist that works
  across morphology classes?
- This idea makes the default a **sampling MPC that plans on the design's own compiled model**, inside the same
  batch as worlds × samples.
- It needs no training and no class detection. The gait-table controller becomes the cheap fallback and the
  sampler's prior.

**Mechanism.**

- **Where.** `frob-control` gains a controller kind, `mpc-sample`. `frob-dyn` gains fan-out, and `frob-kernels`
  gains `K-fork`, `K-cost` and `K-weight`.
- **Algorithm:** annealed MPPI in the style of DIAL-MPC.
  1. At each control tick (25–50 Hz), for each executing world, keep an action sequence U of H spline knots.
     These are joint position targets passed through the L1 actuator model, so torque limits are the card's.
  2. Draw K perturbations from Philox streams keyed by (WorldAddr, tick, sample).
  3. Fork the world state K times with `K-fork`, copying q, v and warm starts at a few KB per copy.
  4. Roll each copy out H steps on the *planning configuration* and score it with the task's cost terms.
  5. Take softmax weights over the samples and update U.
  6. Repeat N_iter times with shrinking noise, then execute U's first knot on the *execution configuration*.
- **Planning configuration.**
  - Coarser than execution: h = 5 ms against 2 ms, two Newton iterations against four, and half the contact
    samples. It is recorded in the controller's identity.
  - The gap between planning and execution is a fidelity-ledger quantity. A controller that wins by exploiting
    its planner's artifacts therefore shows up, which is the plan's trust ladder applied to the controller
    itself.
- **Cost terms, the same for every morphology.**
  - Tracking of the commanded body-frame twist, and attitude error as log(R_dᵀR) (C-13).
  - A centre-of-mass height band derived from A-02's standing height.
  - Electrical power from the L1 model, using the energy-ledger terms.
  - Penalties for friction utilization above 0.8 (slip), and barrier terms for joint limits and self-collision.
  - Wheeled bases use the same terms and have no gait.
- **Prior.** The gait atlas (C-10) supplies the initial phase pattern for U and the shape of the noise, which
  shortens annealing. It is optional.
- **Data layout.**
  - Planning worlds P = E execution worlds × K samples. For example, E = 64 and K = 128 give P = 8,192 on one
    4090 (HYPOTHESIS that this saturates the device).
  - Inside a ragged cohort, a world's samples are contiguous and share its model slot.
- **Determinism.** The controller is kernels running on counter-based noise, so it can be replayed and
  teleported like the physics.

**What an agent sees.**

```
$ frob control synth hexa-b --kind mpc-sample --json      # ok in 0.3 s: no training, no class detection
$ frob sim run hexa-b --controller mpc-sample --scene kitchen:galley@7 --worlds 64 --tier gpu-cuda --json
  # walking in 64/64 worlds at 0.42 m/s; planner gap (Δ energy per metre) 3.1%, recorded; provisional
```

Agents will read an MPC result as "my design works". The risk is that MPC flatters designs that a deployable
policy would find hard to control. The planner-gap column and the ranking falsifier below exist for that reason.

**The lever.**

- **Number.**
  - Time to first competent behaviour for a new design falls from a training run to zero. In B0 that run takes
    about 5 minutes on two 4090s for a quadruped and up to 30 minutes for a humanoid (E011), multiplied by seeds
    and reward iterations.
  - Evaluation costs more instead. Planning steps per world-second ≈ ticks × K × H × N_iter = 25 × 128 × 80 × 2
    ≈ 5 × 10⁵.
  - At about 5 M planning steps/s per 4090 (HYPOTHESIS: 2.5× the plan's 2.0 M TARGET, from the coarser
    setting), that is about 0.1 GPU-s per world-second.
  - A pairwise decision with 24 pairs of 5-s episodes (GB-06, effect size 0.5) is about 240 world-seconds, so
    about 25 GPU-s, or about 15 s on `trj`.
  - B0 needs about 50 minutes for the same decision, so about 200× (HYPOTHESIS). The tenfold survives a 20×
    error in my throughput estimate.
- **Evidence the mechanism works.** DIAL-MPC reports, all FACT (web, arXiv 2409.15610 and its project page):
  - training-free torque-level control over full-order quadruped dynamics, in real time at 50 Hz;
  - 13.4× lower tracking error than plain MPPI;
  - 50% better than RL policies on challenging climbing tasks.
- **Why the incumbents cannot get it cheaply.**
  - For one design, they can: DIAL-MPC runs on MJX, so B0⁺ has it.
  - What they lack is the nesting (worlds × samples × *different* designs in one launch, which needs §3.1), a
    planner tier distinct from the execution tier with the gap recorded, and replayable controller internals.
  - So this idea's tenfold is against B0, not B0⁺. Against B0⁺ it is parity per design, and the advantage lives
    in the composition.
- **Why it still ranks second.** It retires the plan's weakest load-bearing bet (BET-06) with a mechanism that
  has published evidence, and it is the teacher that §3.3 needs.

**Falsifier and fallback.**

- **Cheapest, at M1 on `batch-cpu` with K = 64 and no GPU.** Does `mpc-sample` carry five templates 5 m across
  flat ground: a quadruped, a humanoid, a hexapod, a differential drive and a wheel-leg? If fewer than four of
  five make it, it is not a general day-one controller.
- **At M2–M3.**
  - On the reference quadruped's velocity task, `mpc-sample`'s success on the standard battery is below 60% of
    PPO's.
  - Or, across 20 design variants, Kendall τ between the ranking by MPC score and the ranking by per-design PPO
    return is below 0.6. That means MPC scores do not predict what a trained controller would achieve, and MPC
    stays only as a teacher.
- **Fallback.** The plan's gait-table synthesis becomes the default wherever it passes. MPC stays for the classes
  where it passes, and the rest route to training, as KILL-BASELINE already says.
- **Proposed kill gate.** KILL-MPC at M3, beside GATE-REFLEX-02.

**Sibling needs, checked.**

- **Upstream:** nothing. FrankenSim has no MPC (FACT: plan C-13, "model-predictive synthesis in FrankenSim is
  proposed, not built"), and `fs-control` covers motor loops only.
- **`fs-rand`:** counter-based Philox streams with random access by index (FACT,
  `fs-rand/src/lib.rs`) give the keyed noise.
- **Snapshot and restore:** these are the plan's own (`VecEnv`), not a sibling's.

**Hardware fit.** Planning worlds cost the same memory as physics worlds, so 8,192 of them take a few hundred MB.
A 29-DoF humanoid may need K = 512, which is about 4× the quadruped's planning cost per world-second.

**Cost.**

- What gets built: one controller module, `K-fork`, cost and weighting kernels with fixed-order reductions
  (§27.2), and a planner configuration inside `ModelId`.
- That is smaller than the gait atlas plus force-distribution synthesis it demotes from default.

**Confidence.**

- Medium-high for quadrupeds, from published evidence.
- High for wheeled bases, which need no gait at all.
- Medium for humanoids: sampling cost grows with action dimension, so 29 DoF may need K = 512 or spline knots
  over fewer joints.
- Unknown for hexapods and wheel-legs, which the cheapest falsifier settles at M1.

### 3.3 One policy per family of robots, taught by MPC and carried across typed edits

**What it is.**

- A policy that takes the robot's body as input: one set of weights for a whole family (legged robots, or wheeled
  hybrids).
- It is trained once on thousands of generated designs in ragged cohorts, with sampling-MPC teachers.
- It then evaluates a new design zero-shot in milliseconds, is fine-tuned briefly when needed, and carries across
  typed edits by construction.
- This is what turns breadth from hundreds of designs per GPU-hour into thousands.

**Mechanism.**

- **Where.** `frob-learn` gains an `embody` module. Training uses dialect kernels, and the design population
  comes from the `frob-design` grammar.
- **Tokens come from the typed design term, not from XML.** There is one token per joint and one per contact site
  (foot, wheel, track).
  - Static features, computed by the model compiler in the parent link's frame:
    - the joint's screw basis, 6 numbers per column;
    - the joint's placement relative to the base, as log(base_from_joint) ∈ 𝔰𝔢(3);
    - subtree mass and inertia, scaled to the robot;
    - a summary of the actuator card: peak and continuous torque, speed limit, gear ratio, reflected inertia;
    - tree depth and parent index, as a positional code;
    - the joint's orbit label under the design's symmetry group (C-09).
  - Dynamic features per step: the unit-complex angle (ADR-MARROW-03), velocity, the last action and a contact
    flag.
  - A base token carries projected gravity, the base twist and the command.
- **Network.**
  - A shared per-token MLP encoder, then two rounds of message passing along parent and child edges (gather and
    scatter by parent index).
  - Masked attention pooling into a body latent, then a per-token decoder that emits that joint's target.
  - It is equivariant to joint permutations by construction. Weights tied across orbit labels give equivariance
    to the body's symmetry, reusing the BET-07 machinery.
  - Size: 1–5 M parameters (HYPOTHESIS).
- **Training data.**
  - The grammar generates 2,000–5,000 designs per family (HYPOTHESIS): templates × parameter perturbations ×
    topology edits (two, four or six legs; two or three joints per leg; wheels on legs).
  - Training runs in ragged cohorts (§3.1).
  - The teachers are sampling MPC (§3.2), with no per-design RL experts.
  - Schedule: DAgger first (the student acts, MPC labels the states it visits), then PPO over the population
    with a curriculum uniform over designs.
- **Use.**
  - Zero-shot evaluation first.
  - Warm-start fine-tuning for a few minutes when zero-shot misses its threshold.
  - Fall back to MPC if neither works.
- **Carrying across edits.** Typed edits (§12.2) declare their effect on components, so with tokens:
  - `set-param`, `regear`, `reshape` and `swap-actuator` change features only;
  - `add-limb` and `remove-limb` add or remove tokens;
  - `mirror` and `radial` produce tokens whose orbit labels tie weights.
  - A design's fine-tuned head moves to its child through the edit's declared map.

**What an agent sees.**

```
$ frob train family legged-v1 --designs @grammar:legged --budget gpu-h=48          # once, in an approved envelope
$ frob eval run quad-a@v17 --controller family:legged-v1 --task stairs-0.15 --worlds 256 --json
  # zero-shot success 0.81 in 0.6 s; provisional; fine-tuning not needed
```

To an agent, the family policy feels like an oracle. The danger is that its zero-shot ranking of two close
designs reflects the policy's blind spots rather than the bodies. The ranking falsifier below is aimed at exactly
that.

**The lever.**

- **Number.**
  - Once the family exists, evaluating a design costs inference only: one forward pass per tick. MPC costs about
    2 × 10⁴ planning steps per tick (K × H × N_iter = 20,480).
  - That makes the policy about 100–1,000× cheaper per evaluated world-second than MPC (HYPOTHESIS), and it
    involves no training, unlike B0.
- **Breadth.**
  - At the plan's 2.0 M quadruped world-steps/s TARGET, with a policy forward pass every tenth step, 1,000 designs
    × 16 worlds × 10 s at 500 Hz = 8 × 10⁷ steps, or about 40–80 GPU-s.
  - That is over 10,000 designs per GPU-hour (HYPOTHESIS). TARGET: at least 2,500.
  - B0 manages about 6 designs per GPU-hour.
  - B0⁺ with MPC per design needs about 16 GPU-s of planning plus a compile per design, so about 100–225 designs
    per GPU-hour.
  - The TARGET is therefore at least 10× B0⁺ and at least 400× B0.
- **Amortized cost.**
  - Pretraining costs about 48 GPU-hours (HYPOTHESIS budget; it needs OPEN-SPARK-2).
  - It breaks even against B0 after about 290 quadruped designs at 10 GPU-minutes each.
  - Against B0⁺'s MPC evaluation (about 36 s per design including compile) it breaks even after about
    5,000 designs.
  - A swarm running topology MAP-Elites (idea 13) evaluates that many in a day.
- **Evidence (FACT, web, arXiv 2505.05753, CoRL 2025).**
  - The authors trained on about 1,000 procedurally generated humanoids, quadrupeds and hexapods (GENBOT-1K).
  - Generalization to unseen bodies improved as the number of training bodies grew.
  - The resulting policy transferred zero-shot to a real Unitree Go2 and H1.
  - They trained embodiment-specific RL experts first. That is the expensive step MPC teachers replace here,
    and it is a HYPOTHESIS that MPC teachers are good enough.
- **Why the incumbents cannot get it cheaply.**
  - Training on thousands of different bodies needs them in one batch. MuJoCo Warp cannot do this, MJX
    recompiles per structure, and Newton's support is experimental (FACT, web, ×3).
  - The scaling study paid for about 1,000 expert training runs. Here the population is a handful of cohorts,
    and the teachers need no training.
  - The token features come from typed design terms (screws, inertias, cards, orbits). An XML-based stack would
    have to reverse-engineer them.

**Falsifier and fallback.**

- **Cheapest, at M3 on the existing corpus.**
  - Train on 200 quadruped and hexapod variants.
  - Test on 50 variants held out by parameter range and 10 held out by topology.
  - Kill if zero-shot return is below 70% of per-design PPO on more than half the held-out set, *and* warm-start
    fine-tuning needs more than 0.2× PPO's from-scratch steps.
- **For design decisions specifically.** If Kendall τ between the zero-shot ranking and the per-design PPO
  ranking is below 0.6 on 20 variants, the family policy must not decide between designs. It stays as a warm
  start only.
- **Fallback.** MPC (§3.2) makes decisions, and the family policy becomes a warm start (§19.3's behaviour-cloning
  slot).
- **Proposed kill gate.** KILL-FAMILY at M4.

**Sibling needs, checked.**

- **FrankenTorch** runs tensors on the CPU only; `Device::Cuda` and `Device::Metal` are metadata (FACT, E007).
  Training kernels must therefore be the plan's dialect kernels, extended with masked attention pooling, gather
  and scatter by parent index, layer norm, and the backward pass of each.
- **FrankenTorch** has attention modules with autograd (FACT: `frankentorch/crates/ft-nn`; E007). It serves as
  the CPU gradient oracle for the new kernels, which extends GATE-CORTEX-02.
- **Compute envelope.** OPEN-SPARK-2's default (one device, one hour) blocks this. It needs a campaign the owner
  approves, and `trj` is often heavily loaded (E002).

**Hardware fit.** A 5 M-parameter policy with its Adam state takes about 60 MB, which is trivial on 24 GB.
Inference also runs on the Mac.

**Cost.** The largest of the five. It needs network kernels with their backward passes, a data pipeline and a
curriculum, roughly 1.5–2× the plan's Cortex I (HYPOTHESIS). It belongs at M4, once §3.1 and §3.2 exist.

**Confidence.**

- Medium.
- The published scaling evidence is strong for locomotion on fairly open ground.
- Transfer to tight interiors, payloads and wheel-legs is not established.
- MPC-teacher quality at this scale is unproven.

### 3.4 Paired multilevel estimation across tiers

**What it is.**

- The trust ladder (§14.5) is the plan's most distinctive promise (N3), and it is expensive.
- This idea makes it cheap. It estimates a metric's referee-tier value, or a design difference, from many paired
  GPU episodes plus a few referee episodes *at the same world addresses*.
- It uses the telescoping multilevel estimator that FrankenSim already implements.

**Mechanism.**

- **Where.**
  - `frob-opt` (`compare`, `promote`) and `frob-dyn` (the tiers).
  - `frobd` schedules the levels across devices and machines.
  - `fs-uq` is reached through the airlock.
- **Levels.**
  - ℓ0: GPU, `f32`, CL-1 compliant contact (or the cheaper MPC planning configuration).
  - ℓ1: `batch-cpu` in `f64`, with the same kernels.
  - ℓ2: the referee, FrankenSim's exact Coulomb contact in `f64`.
  - Y_ℓ(a) is the metric of world address a at level ℓ. For a comparison, Y is the paired difference B − A at
    the same address.
- **Estimator.**
  - E[Y₂] = E[Y₀] + E[Y₁ − Y₀] + E[Y₂ − Y₁].
  - Each correction is computed at the same address: the same terrain draw, the same disturbance, the same
    randomization. Philox streams keyed by address make this free (FACT, `fs-rand`).
  - Samples are allocated N_ℓ ∝ √(V_ℓ / C_ℓ) (Giles), from pilot variances.
  - FACT: `fs-uq` already implements this allocation, with coarse and fine runs coupled on disjoint Philox
    ordinals and with pilot estimation (`fs-uq/src/mlmc.rs`, `product_mlmc.rs`,
    `product_mlmc_allocation.rs`, `product_mlmc_pilot.rs`).
- **Anytime validity.**
  - Each level's mean gets a mixture confidence sequence at α/3 (FACT: `fs-eproc` provides them).
  - The sum of the three intervals is anytime-valid by a union bound.
  - Promotion stops when the referee-level interval clears the promotion envelope's boundary, or when the budget
    ends.
- **Segment mode, optional.**
  - Chaotic contact can make whole episodes decorrelate across tiers.
  - Then the referee re-simulates 1-s segments from GPU snapshots instead.
  - This estimates per-segment tier bias, which is a different quantity, and reports say so.
- **Pilots** come from the CPU shadow auditor (idea 14).

**What an agent sees.**

```
$ frob eval promote cmp:7c1e… --tier referee --method multilevel --json
  # established: B better on energy per metre by 7.9% [5.1, 10.6] (anytime-valid, α = 0.05)
  # 4,096 GPU pairs, 512 batch-cpu pairs, 38 referee pairs; ρ(gpu, referee) = 0.95
  # a referee-only estimate of the same width would need ≈ 410 referee pairs
```

Agents will perceive this as "established" becoming the default, not a ceremony. Its one confusing output is a
weak ρ, so the envelope prints ρ, and the referee-only count it replaced.

**The lever.**

- **Number.**
  - The referee episodes needed for a given interval width scale by (1 − ρ²), where ρ is the correlation between
    paired GPU and referee outcomes.
  - ρ = 0.90 gives 5.3× fewer; 0.95 gives 10×; 0.98 gives 25×. This is exact for the two-level
    control-variate case with many level-0 samples.
- **Why it matters.**
  - The referee is the costly tier: `f64` on the CPU, with a plan TARGET of at least 0.2× real time per core for a
    humanoid.
  - A 10-s humanoid episode is then about 50 core-seconds.
  - 400 pairs × 2 designs is about 4 × 10⁴ core-seconds, about 19 minutes on the Mac's 36 cores.
  - At ρ = 0.95 that falls to about 2 minutes.
- **The unlock.** Established verdicts become cheap enough to be the default for every decision, not an
  occasional promotion, and the plan's main differentiator becomes affordable.
- **Why the incumbents cannot get it cheaply.**
  - They have no second, more trusted physics that shares the model, the scene addresses and the random draws.
  - MJX and MuJoCo C are one physics, so pairing them adds no fidelity information.
  - Newton has several solvers (FACT, E012), but no shared logical addressing of randomness and no fidelity
    ledger (HYPOTHESIS, not checked).
  - A wrapper could pair seeds, but it would be pairing equally untrusted solvers.

**Falsifier and fallback.**

- **Cheapest, at M2.**
  - On 200 addresses of the standard battery, compute ρ between GPU and referee for the promoted quantities:
    energy per metre, displacement over 2 s, success.
  - That costs about 400 referee episodes, a few CPU-hours, and GATE-FID-01 already pays for the same paired
    data.
  - If ρ is below 0.7 for the main quantities, the gain is under 2×, so the estimator is not worth building.
- **Fallback.** The plan's plain e-process promotion (§14.5), and segment mode wherever whole-episode ρ is low.
- **Proposed kill gate.** KILL-MLMC at M2, beside GATE-FID-01.

**Sibling needs, checked.**

- **`fs-uq`**, all FACT, `frankensim/crates/fs-uq/src/`:
  - multilevel execution with Giles allocation, pilots, and coarse/fine coupling;
  - frozen linear control variates (`product_control_variate.rs`);
  - defensive importance sampling of failure probabilities (`product_importance.rs`).
  - None of it is in the plan's FD-FSIM list (FACT, §25.4), so `fs-uq` must be added there.
- **A mismatch to adapt around.**
  - `fs-uq`'s `MlmcExecution::advance` drives a per-call closure `FnMut(usize, &[f64]) -> Result<f64, E>` over a
    `UqPlan` parameter law (FACT, `product_mlmc.rs`).
  - That fits the referee tier directly. The GPU levels run in batches, so FrankenRobotics needs an adapter that
    turns batch results into calls.
  - It also needs an additive upstream ask for a batch entry point; call it FD-FSIM-10.
- **`fs-eproc`** mixture confidence sequences and **`fs-rand`** keyed streams (FACT).

**Hardware fit.** The referee runs on the Mac's 36 cores (FACT, E001) and on `trj`'s CPU when it is not loaded
(E002). GPU levels add almost nothing.

**Cost.** Small: the estimator plus the adapter, perhaps a few hundred lines. The expensive part, paired referee
runs, is already in the plan for GATE-FID-01.

**Confidence.** High on the statistics. Medium on ρ for long, contact-rich episodes, and the falsifier for that
is cheap and comes at M2.

### 3.5 Counterfactual probes: edit the robot at the moment it fails

**What it is.**

- When worlds fail, rewind each to just before its first cause.
- Apply a few dozen candidate edits, to the robot or to the scene, in place.
- Re-run short continuations, all in one batch.
- The agent sees which edits would have prevented the failure before it spends a full evaluation on any of them.

**Mechanism.**

- **Where.**
  - A new verb, `explain.counterfactual`, in `frob-analyze` and `frob-opt`.
  - Snapshots come from Chronicle's per-world ring buffer (T3, §23.2).
  - State transport lives in `frob-design`.
  - Cohorts come from §3.1, and the controller from §3.2 or §3.3.
- **Algorithm.**
  1. For F failing worlds, take the first-cause time t_f (§23.5) and the snapshot at t_f − Δ, with
     Δ ∈ {0.25, 0.5, 1.0 s}.
  2. Gather candidate interventions:
     - the top typed edits by A-16 sensitivity and by the Explain heuristics;
     - edits the agent names;
     - scene interventions, such as switching an interface card from wet to dry or lowering a step by 2 cm,
       which separate a design fault from a scene fault.
  3. Transport the state. Each typed edit declares a map from the parent's state to the child's:
     - the identity on joint coordinates for `set-param`, `regear`, `reshape` and `swap-actuator`;
     - a re-anchored base pose for `move-mount`;
     - the rest pose for added joints.
     - Edits that cannot be transported are refused, with the reason.
  4. Run M continuations for each (world, intervention), with M = 16–32 and keyed noise, to t_f + 1 s.
     - The controller must adapt to the edited body without retraining: MPC re-plans on the edited model, and the
       family policy reads the new tokens.
     - A per-design PPO policy cannot adapt, and the verb refuses it.
  5. Score each intervention: the fraction of failures avoided, metric deltas, and energy-ledger deltas. For up to
     five interventions, compute exact Shapley values over all 32 coalitions in the same batch, to expose
     interactions.
  6. Hand the top three to `eval compare` for a full paired verdict. The probe itself is labeled `Estimated`; it
     is a screen, not a verdict.
- **Batch size.** 20 failing worlds × 16 interventions × 32 continuations is about 10,240 short worlds: one
  cohort launch, a few seconds.

**What an agent sees.**

```
$ frob explain counterfactual r:19e2… --failures 20 --budget gpu-s=20 --json
  # first cause: slip of the front feet on the stair nosing, 17 of 20 failing worlds
  # set-param foot.radius 0.022→0.030     avoided 14/17   Δ energy per metre +2%   transportable
  # regear front.knee 1.55→1.8            avoided  9/17   Δ energy per metre +6%   transportable
  # scene: nosing interface dry→rubber    avoided 16/17   (a scene fault, not a design fault)
  # next_actions: frob eval compare quad-a quad-a+foot.radius=0.030 --task stairs-0.15
```

This is the closest thing to a debugger an agent designing a robot can have. The way agents could misread it is
to treat a local rescue ("it would have survived this fall") as a global verdict ("the edited design is better").
The label `Estimated` and the mandatory hand-off to `eval compare` guard against that.

**The lever.**

- **Number.**
  - Measured as turns-to-fix on the planted-defect corpus (§21.3).
  - TARGET: at least 3× fewer serial turns than the plan's heuristic Explain (§22.3), and at least 10× fewer
    than the scalar-reward control arm of BET-10.
  - Each turn costs agent think time (about a minute, HYPOTHESIS) plus a full evaluation. A probe tests about 16
    candidates in about 10 GPU-seconds.
- **Why the incumbents cannot get it cheaply, partly.**
  - MuJoCo Warp lets batched model *field values* vary per world (FACT, web). A wrapper could therefore probe
    scalar parameters for one topology from MJX or MuJoCo Warp snapshots.
  - What it cannot probe in one batch is geometry or topology, because heterogeneous worlds are unsupported (FACT,
    web).
  - It also has no state transport declared by edits, and no controller that adapts without retraining.
  - So uniqueness is partial: it lies in the geometric and topological probes, and in the adapting controller.

**Falsifier and fallback.**

- **Cheapest, at M3, once §3.1 and §3.2 exist.** Run 50 planted defects. The probe fails if either:
  - its top intervention is the planted fix (or one of equal effect in a full paired comparison) in under 50% of
    cases; or
  - its "avoided" fractions agree in sign with full paired verdicts in under 70% of cases, which makes it
    misleading.
- **Fallback.** Probes appear only as hints inside `explain run`, with no separate verb.
- **Proposed kill gate.** KILL-PROBE at M4, beside GATE-ATEL-03.

**Sibling needs, checked.** Nothing upstream. `fs-uq` has variance-based Shapley effects for continuous inputs
(FACT, `product_shapley.rs`), which is not the discrete, interventional Shapley needed here. Exact Shapley over at
most five interventions is a few lines in-house.

**Hardware fit.** About 10⁴ worlds of 1–2 s each, which is trivial.

**Cost.** Modest, given §3.1 and §3.2: a state-transport table per edit kind, the verb, and a scoring kernel.

**Confidence.** Medium. Local counterfactuals can disagree with global outcomes, which is why the probe feeds a
full comparison and carries the label `Estimated`.

---

## 4. How the five compose: two decisions, priced

All figures are HYPOTHESIS until idea 25's benchmark measures them on `trj`.

**Decision 1: one established pairwise decision.** "Is quadruped B better than A on `stairs-0.15`?", at α = 0.05
and effect size 0.5, which needs about 24 pairs (GB-06).

| Phase | B0 | B0⁺ | Plan v0.2, if BET-06 holds | With the five |
|---|---|---|---|---|
| Controller for each design | PPO, 5 seeds × about 5 min on two 4090s: about 50 min | None (DIAL-MPC); one JIT compile per design, tens of seconds | Gait-table synthesis in seconds; failing that, training | None: MPC, or the family policy |
| Evaluation of 24 pairs | Inside the training budget | About 25 GPU-s of MPC | Seconds | About 25 GPU-s of MPC, or under 1 s with the family policy |
| Check on a more trusted tier | None | None | E-process promotion on the referee, on the order of 100 pairs | Multilevel promotion, about 10× fewer referee pairs; about 400 core-s |
| **Total** | **About 50 min, not established** | **About 1–2 min, not established** | **Minutes, only if BET-06 holds** | **About 0.5–1 min, established** |

Against B0 this is 50–100×. Against B0⁺ it is about 2–4× faster *and* established, and no tenfold is claimed
there. The difference from plan v0.2 is that the plan's fast path depends on BET-06, whereas the five give the
same path a controller with published evidence behind it.

**Decision 2: a population decision.** "Which of 500 generated designs is best on `stairs-0.15`?"

| | B0 | B0⁺ | With the five |
|---|---|---|---|
| Screening | 500 × 10 GPU-min ≈ 83 GPU-hours, about 42 h on `trj` | 500 JIT compiles (minutes to hours, depending on how far compiles run in parallel) + 500 × 16 GPU-s of MPC ≈ 2.2 GPU-hours, so about 1.5–4 h | Family policy zero-shot in one or two cohorts: 4 × 10⁷ steps, about 40 GPU-s |
| Confirmation | None | None | MPC on the top 20 (about 5 GPU-min), then multilevel promotion of the top three (minutes of CPU) |
| **Total** | **About 42 h** | **About 1.5–4 h** | **About 10 min (TARGET)** |

That is at least 10× against B0⁺ and about 250× against B0. If KILL-FAMILY fires, screening falls back to MPC for
all 500 designs, and the gain over B0⁺ shrinks to about 1.5–4×, mostly from removing compiles. The plan should
state that dependency in plain words.

**After the five**, compute per decision is seconds to minutes, and the agent's serial turns dominate. Idea 7
(`solve`) and idea 5 (probes) are the next levers, which is why idea 5 is in the five.

---

## 5. Changes to the plan these imply, and defects noticed on the way

1. **Metrics.** Add CPD, breadth and turns-to-fix to §28, measured against B0 and B0⁺ on the benchmark (idea 25).
   Retire GATE-CORTEX-05's "at most 15 minutes on one RTX 4090" as a headline. It is slower than the published
   5 minutes on two (§28.1's own context column; E011).
2. **Batching.** GOAL-02 becomes "thousands of worlds of up to thousands of different designs per device".
   BD-074, BD-082 and BD-099 are designed for cohorts from the start.
3. **Controllers.**
   - In §18, `mpc-sample` becomes the default. Gait-table synthesis becomes the fallback and the sampler's prior.
   - KILL-BASELINE's fallback routes to MPC before it routes to training.
   - Add KILL-MPC and KILL-FAMILY.
4. **Foundations.** Add `fs-uq` (multilevel Monte Carlo, control variates, defensive importance sampling, Shapley
   effects), `fs-voi` and `fs-surrogate` to the FD-FSIM row of §25.4. None appears there today (FACT), although
   each exists with a public API (FACT, files cited above).
5. **Compute.** OPEN-SPARK-2 should be raised at M0 with a concrete campaign request for the family policy
   (about 48 GPU-hours, HYPOTHESIS).
6. **Missing appendix.** Appendix K, the review log, is cited in the Contents and in §0.6 but is not in the file
   (FACT: the document ends at Appendix J, line 3,666).

---

## 6. Sources

**Public**, from five web searches on 2026-11-21. These are search summaries, and the pages were not opened.

- MuJoCo Warp documentation (heterogeneous worlds unsupported; batched model fields):
  https://mujoco.readthedocs.io/en/latest/mjwarp/
- MJX documentation (recompilation on structural change; humanoid throughput with and without graph capture):
  https://mujoco.readthedocs.io/en/stable/mjx.html
- MJX JIT-time anecdote: https://www.github.com/google-deepmind/mujoco/issues/1616
- Newton worlds and heterogeneous support: https://newton-physics.github.io/newton/1.2.0/concepts/worlds.html
- DIAL-MPC: https://arxiv.org/abs/2409.15610 and https://lecar-lab.github.io/dial-mpc
- Embodiment scaling laws (GENBOT-1K): https://arxiv.org/abs/2505.05753
- MuJoCo Playground, through MOR E011: https://arxiv.org/abs/2502.08844

**Local**, read on 2026-11-21. Paths are under `/Users/jemanuel/projects/`.

- `franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md`, all of it.
- `franken_robotics/planning/fable/mor/MOR_CASE.md`.
- `frankensim/crates/`, module documentation and public signatures:
  - `fs-ad`, `fs-adjoint`, `fs-surrogate` (`conformal_band`), `fs-assimilate`, `fs-voi`
    (`ranking_flip_probability`);
  - `fs-archive`, `fs-race`, `fs-eproc`, `fs-rand`, `fs-ivl` (`taylor.rs`, `multi_taylor.rs`), `fs-exec`
    (`KillRegistry`), `fs-sos`, `fs-dfo`, `fs-phs`, `fs-symmetry`;
  - `fs-uq` (`mlmc.rs`, `product_mlmc*.rs`, `product_control_variate.rs`, `product_importance.rs`,
    `product_shapley.rs`);
  - `fs-mbd/src/articulated.rs` (plain `f64` signatures, no `Real` generics).
- `frankensim/apps/instrument-lab/render-gpu-piano/src/`: file sizes only.
- `frankentorch/crates/` listing, and `ft-nn`, which contains attention modules.
