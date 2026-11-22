# WIZARD_REACTIONS_CC.md: Claude's reaction to the reviews of its ideas

| | |
|---|---|
| Author | Claude Opus 5.5 (`claude-opus-5-5`) in Claude Code |
| Date | 2026-11-21 |
| Read in full | `WIZARD_SCORES_COD_ON_OTHERS.md` (Codex on CC and AGY) and `WIZARD_SCORES_AGY_ON_OTHERS.md` (AGY on CC and COD) |
| Re-read | My `WIZARD_IDEAS_CC.md` and `WIZARD_SCORES_CC_ON_OTHERS.md` |
| New checks | `frankensim/crates/fs-assimilate/src/nonlinear/` (read only), and plan §8 (line 592), §14.5 and §19.6 |

## 0. How they scored my ideas

| My idea | COD | AGY | Their main objection |
|---|---:|---:|---|
| #1 Ragged cohorts | 817 | 845 | Padding and bucket fragmentation. COD adds that my "25 designs/s" is the homogeneous ceiling, not the ragged figure. |
| #2 Sampling MPC nested in the batch | 739 | **875** | COD: the planner can rank easy-to-plan designs above better bodies. AGY: a 5-ms planner risks tunnelling and the Cayley refusal bound. |
| #3 Family policy taught by MPC | 724 | 670 | COD: global Kendall τ hides wrong rankings at the top. AGY: the network kernels are costly to build, and the policy biases search against unusual designs. |
| #4 Paired multilevel estimation | **891** | 815 | COD: my unit-coefficient correction is not the (1 − ρ²) estimator I quoted. AGY: chaotic divergence between tiers lowers ρ. |
| #5 Counterfactual probes | 841 | 790 | Both: carrying state across a geometry edit mid-episode is invalid. |
| 25 Design-decision benchmark | **936** | 780 | COD: an easy corpus or a weak incumbent arm manufactures a 10× result. |
| 16 Fused kernel per world | 664 | **835** | AGY calls it my "biggest missed gem". |
| 8 Interval carving | **827** | 655 | COD: a sound cut removes a whole family of rollouts. |

My top five average **802** from COD and **799** from AGY.

**Where the reviewers agree.**

- Both rate #1 and #4 highly.
- Both rate #3 lowest of my five.
- Both find the same flaw in #5.

**Where they split.** On #2 they are 136 points apart, and on #16, 171 points.

**A pattern in all three of our reviews, mine included.** Each reviewer rates the others' ideas higher when they
look like its own:

| Reviewer | Rates highly | Which resembles its own |
|---|---|---|
| AGY | CC-16 at 835 | AGY #1 |
| COD | AGY #10 at 854 | COD #1 |
| COD | CC #4 at 891 | COD #3 |
| Me | COD #10 at 660 | CC #1 |

The most informative signal is therefore an idea all three of us proposed *independently*. There are two:

- replay experiments from a failure snapshot (CC #5, COD #5, AGY #5);
- a benchmark that makes 10× falsifiable (CC-25, COD §0, AGY's TTVD).

---

## 1. Where I agree with them

1. **#4 needs a fitted coefficient, and ρ must be measured on contrasts (COD).** COD is right, and this is the most
   important correction I received.
   - My estimator, as written, is the unit-coefficient telescoping sum. Its variance is
     σ_H² + σ_L² − 2ρσ_Hσ_L.
   - I quoted the lever of the optimal-coefficient control variate, (1 − ρ²)σ_H². The two agree only when the tiers
     have equal variance. At ρ = 0.95 they are then 0.100 against 0.0975.
   - If the GPU tier's spread is 1.3× the referee's, the unit coefficient leaves 0.22σ_H², which is 2.3× worse.
   - The sharper point is that ρ must be measured on the **paired design contrast** B − A, not on absolute
     episode metrics. Pairing already removes the common variance, so the contrast correlation can be much lower
     than the metric correlation.
   - My falsifier measured the wrong ρ.
2. **#5 must not carry state across geometry, inertia or gear edits mid-episode (both).**
   - A wider foot under load starts out interpenetrating the ground.
   - Regearing changes reflected inertia and the motor-side coordinates, so even "identity on joint coordinates" is
     wrong.
   - And COD's deeper point stands: a mid-run rescue is conditional on a state the edited robot might never have
     reached, so it can rank permanent edits in the wrong order.
3. **#3's falsifier is too weak (COD).**
   - A Kendall τ over the whole corpus can be 0.7 while the top three are wrong. The test has to be top-k recall and
     regret near the selection boundary.
   - COD is also right that a raw parent index as a positional feature breaks the relabelling symmetry I claimed.
4. **#3 biases search against inventive designs (AGY).** A shared policy controls designs near the centre of the
   grammar better than outliers. In a search for breakthroughs, that is a systematic bias against exactly the
   designs worth finding.
5. **#2 can rank easy-to-plan designs above better bodies (COD).** A fixed-budget sampling planner measures
   "body × planner competence", not the body. This is the strongest objection to the core of my slate, and §5.1
   proposes the missing mechanism for it.
6. **#2's coarse planner collides with a refusal the plan already has (AGY).**
   - Plan §8 bounds the Cayley update at |ω|h ≤ 0.2 per substep and refuses beyond it (FROB-REF-212, line 592).
   - At h = 5 ms that caps |ω| at 40 rad/s, which a wheel of 5 cm radius reaches at 2 m/s.
   - I proposed a planner step that the plan would refuse for fast wheeled bases. The planner's step must be chosen
     per design from its expected maximum |ω|.
7. **#1 is overstated, and its spherical joints are wrong (COD).**
   - At my own 0.6× ragged target, a GPU-second covers about 15 designs, not 25.
   - COD's remark about coordinate maps for product joints exposes a real flaw. Writing a spherical joint as three
     successive screw exponentials is an Euler-angle chart, and it is singular at gimbal lock. See §3 for the fix.
8. **The benchmark is the most important item in all three slates (COD, 936).** I adopted it as the yardstick
   instead of ranking it, and COD's version of it is better than mine (§4.1).
9. **I made a factual error in idea 17 (COD).**
   - I wrote that `fs-assimilate` does linear-Gaussian Kalman fusion only.
   - It also has `nonlinear/ensemble.rs`, a deterministic serial ensemble square-root filter (Whitaker–Hamill), plus
     `nonlinear/iterated.rs` and a `robust` lane.
   - Idea 17 stays cut because no hardware or logs are in scope, but the stated reason was wrong.

## 2. Where I think they are wrong

1. **AGY: idea 16 (fused kernels) is a "10–15× gem", and my 5-ms planner was "forced by v0.2's slow pipeline".**
   - The 10–15× rests on AGY's 200 KB of memory traffic per world per substep. That is about 5–10× the plan's own
     budget (§28.4), as I showed in my scores file.
   - COD's occupancy arithmetic on Ada points the same way: two such blocks per SM, so at most 16 of 48 warps
     resident before counting registers.
   - The coarse planner was also not forced. Planning on a cheaper model than you execute is standard MPC practice.
     I paired it with a recorded planner–execution gap precisely because it is a choice.
   - That said, AGY has a point I missed; see §3, item 6.
2. **AGY: COD #3 (randomized audits) is "mathematically dominated" by my #4. This is wrong.**
   - Multilevel estimation with a pre-declared allocation is unbiased. Once you *choose which addresses to promote
     after seeing GPU results* (near ties, novel contact regimes, anything flagged by Explain), a deterministic
     selection rule biases it.
   - COD's logged propensities keep the estimate unbiased under exactly that adaptive targeting, and they allow any
     predictor m_i, learned or not, where multilevel uses a fixed coefficient.
   - The two are complements: pair at the same address to shrink Y − m, and audit at random to stay valid. AGY's
     "zero range explosion" point is true only for the non-adaptive case.
3. **AGY: COD #9 (sharing test cases across symmetry orbits) is "a genuine 4–8× speedup with zero approximation
   error" at 745. Wrong on both counts.**
   - The saving is only work you chose to create. If a battery contains mirrored copies of a scene, you would simply
     not build them.
   - Floating-point non-associativity and noise streams that are not mirrored also break exact invariance.
   - Orbit evaluation is useful as an invariance *test* (the plan's §19.6 already runs a mirrored check), not as a
     saving.
4. **AGY: COD #6 (gradients per contact regime) is a "strong 10×+ lever" at 775.** AGY's own objection on the same
   line, that `hybrid_sensitivity` refuses simultaneous events, is decisive for gaits. In a trot, two feet touch
   down at once by design. The score contradicts the stated objection.
5. **AGY: idea 12's restriction to limit cycles is an "unnecessary handicap".**
   - The restriction is what makes the gradient well-posed. The once-per-cycle return map averages over every
     contact switch inside the cycle and has a defined fixed-point sensitivity.
   - A short-horizon IFT gradient has neither property. It is biased at each contact switch, and it ignores how the
     controller responds to the edit.
   - COD's objection, that the method is ill-conditioned near an orbit bifurcation, is the right one, and I accept it.
6. **AGY: #3's network kernels are "a multi-month ML-compiler project". Overstated.**
   - The operations needed are few: gather and scatter by parent index, a per-token MLP, masked attention pooling,
     and layer norm, plus their backward passes.
   - FrankenTorch's CPU autograd is the oracle that GATE-CORTEX-02 already requires.
   - The real cost lies elsewhere: teacher coverage, pretraining compute, and the bias in item 4 of §1. I accept
     AGY's lower score for those reasons, not for this one.
7. **AGY: I "completely missed FrankenSim's ~290k-line structural stack".** I did not miss it: `fs-topopt`,
   `fs-solid` and the rest are in the evidence ledger (E006), and I excluded structural synthesis because it does
   not change the cost of a design decision. AGY is nevertheless right that the exclusion was a mistake (§3,
   item 8).
8. **COD: fund checked search over discrete part catalogs "first".**
   - I agree it is a cheap, reversible M1 lane, and I now rate it higher (§3).
   - But "first" overstates it. Every reviewer, COD included, notes that dynamic requirements break monotonicity, so
     the sound cuts reach only quasi-static infeasibility.
   - The benchmark and paired multilevel estimation come before it.
9. **COD: idea 15 (passability maps) at 766.** This is a little high. A static SE(2) map decides only geometric
   infeasibility for wheeled and tracked bases. AGY's "<2% of design time" point is closer to right for the overall
   loop, though I value it more for the interior archetypes.

## 3. Points that change my own evaluation

**Revised view of my own slate.**

| My idea | Was | Now | What changes |
|---|---|---|---|
| #1 Ragged cohorts | 1st | **1st** | Two mechanism fixes, and report 15 designs/s rather than 25. |
| #2 Sampling MPC | 2nd | **2nd** | Planner step chosen per design under FROB-REF-212. Check that rankings are stable as planner fidelity changes. Add the budget-curve estimator (§5.1). |
| #4 Multilevel estimation | 4th | **3rd** | Frozen fitted coefficients; ρ measured on contrasts; the same planner on every tier; COD's audits as the validity layer. |
| #5 Counterfactual probes | 5th | **4th** | Narrowed to diagnostic interventions. Design edits replay from reset. |
| #3 Family policy | 3rd | **out of the five** | An option for M5. Screening only, with top-k regret as the falsifier and a flag for designs far from the training set. |
| *New*: rank by achievable performance (§5.1) | — | **5th** | It is what makes #2's cheap rankings trustworthy. |

**The fixes in detail.**

1. **#1, joint configuration.**
   - Store each joint's configuration as a group element: a joint-local SE(3) transform, or a unit quaternion
     for spherical joints.
   - Update it as X_J ← X_J · cay(h·S·v), where S is still the masked 6×k screw basis.
   - This keeps the update uniform and free of branches, with no coordinate singularity, at a cost of about 8 floats
     per joint instead of 1, plus renormalization.
2. **#1, factorization.**
   - Replace padded dense Cholesky with a tree-sparse LTDL factorization scheduled by level from the parent tables.
   - This is AGY's arrowhead observation from AGY #1 applied to ragged trees. It removes AGY's O(n_v³) padding
     objection: cost becomes O(n·depth).
   - Foot contacts keep the arrowhead pattern. Rare self-contact between limbs falls back to dense, per world.
3. **#4, the estimator.**
   - Switch to frozen fitted coefficients, β = ρ̂σ̂_H/σ̂_L from a pilot. `fs-uq/src/product_control_variate.rs`
     already provides frozen linear control variates.
   - Measure ρ on paired contrasts.
   - Keep the controller identical across tiers. The *planner* stays on the GPU and only *execution* changes tier, so
     the referee world queries the same GPU planner every tick. Otherwise, re-planning on each tier makes the
     controller itself a source of decorrelation. This is COD's point about frozen control against re-planning.
   - Use COD's randomized audits whenever promotion targets are chosen adaptively.
4. **#5, two classes of intervention.**
   - **Diagnostic interventions** are forked from the snapshot: torque and speed caps, delay, friction cards,
     privileged state instead of the estimator, scene cards.
   - **Design edits** go through reset, or through a snapshot taken in a flight phase with clearance projection and an
     admissibility check.
   - The output stays `Estimated`.
   - Under MPC, counterfactual continuations cost on the order of 10³ GPU-seconds, as I recorded in my scores file.
     The default probe therefore runs under the gait-table controller and says so.
5. **#3, demoted.** It is not dead, but it is now an M5 option for screening only, with two conditions:
   - top-k recall and regret near the cutoff must pass;
   - any design whose tokens fall outside the training distribution goes to MPC.
6. **Idea 16 (fusion), raised to an M3 option. AGY's point that I missed:**
   - Under my own design, sampling MPC is the default controller, and its planning rollouts are physics steps. Steps
     per second therefore multiply directly into the cost of every decision.
   - "Raw stepping speed is not the lever" was true against PPO-per-design pipelines. It is not true inside mine.
   - Fusion is also not the same thing as specialization. A persistent kernel can read the ragged model tables
     instead of being compiled per topology.
   - And a design's K-sample planning fan-out is homogeneous by construction: one design, one topology.
   - So fusing the planner's fan-out does not conflict with ragged execution across designs. Profile first; I still
     expect 2–4×, not 10–15×.
7. **Catalog co-design (idea 8, COD #1, AGY #10), raised.** COD's argument corrects mine.
   - I said point checks cost 2 s, so carving saves little. That holds for an agent proposing one design at a time.
   - For catalog sizing (motors × gearboxes × batteries × frames, 10⁴–10⁶ combinations), a sound cut on a partial
     assignment saves point checks multiplicatively.
   - It is a good M1 lane for the quasi-static part of the problem.
8. **Structural link synthesis (AGY #4, COD #11), raised. Both reviewers scored it near 800, and their arguments
   moved me.** The user's brief asked for ideas that "change what is possible", and this one does. Two corrections
   to my own objections:
   - **Mass misframed the stake.** Total-mass savings are modest, as I said. But the inertia of distal links
     (shanks, feet) shapes swing dynamics and reflected inertia, so a sized link changes the *dynamic ranking* of
     designs, not just their mass.
   - **The load-fidelity objection is answerable.** Take the loads from the referee's exact-Coulomb tier, with an
     envelope over contact stiffness, not from GPU penalty contact.
   - My scores move from 460 to **620** (AGY #4) and from 420 to **540** (COD #11). COD's framing, a bounded lane for
     finalists, is the right shape. It is still not on the decision-cost path.

**My scores of their ideas: what moved and why.**

| Idea | Old | New | Why |
|---|---:|---:|---|
| COD framing (benchmark) | 740 | **800** | My objection was too few briefs. The arithmetic in §4.1 shows 20 paired briefs separate 10× from 3×. |
| COD #3 randomized audits | 610 | **660** | It is the validity layer for adaptive promotion (§2, item 2), and COD exposed a weakness in my own estimator. |
| AGY #5 intervention burst | 670 | **600** | Plan §19.6 already re-runs every reported policy result at half the time step, on the next tier, with card-perturbed parameters, and under the other contact law. AGY's contribution is the *placement* (at the failure snapshot, for any controller), plus two extra slots: twice the Newton iterations, and stiction speed. I over-credited it before. |
| AGY #4 link synthesis | 460 | **620** | §3, item 8. |
| COD #11 link from measured loads | 420 | **540** | §3, item 8. |
| AGY #2 IFT adjoints | 380 | **470** | COD's point holds: local margin sensitivities are the right first consumer. I still reject closed-loop design gradients. |
| AGY #1 fused kernel | 440 | **500** | Stepping speed matters under MPC (§3, item 6). The 20–30 M figure is still unsupported. |
| COD #1 / AGY #10 catalog search | 540 / 460 | **580 / 560** | §3, item 7. |

---

## 4. The strongest idea from each of the other models, steelmanned

Both of my picks overlap ideas in my own slate. That is either the convergence signal from §0 or the self-preference
I warned about. To guard against the second, §4.3 also gives the strongest idea from each model that I did *not*
have.

### 4.1 From COD: the benchmark from brief to accepted design, with censoring (COD §0)

**Why it is strongest.** Every other idea in all three slates states its 10× as a TARGET. This is the only one that
can make the owner's goal true or false, and it changes engineering choices as well as reporting them.

**The steelman.**

- **It answers the question none of us measured.**
  - MOR's first pivotal question (PQ1) asks where the time goes, and MOR's first unknown is the measured phase times
    of an incumbent loop.
  - All three slates reason from *guessed* phase times: my 50-minute B0, AGY's "15–60 GPU-hours", COD's
    illustrative unit costs.
  - Instrumenting the incumbent arm phase by phase replaces all those guesses with data before any GPU work is
    committed.
  - COD's rule, that *"a phase that accounts for less than 90% of baseline elapsed time cannot deliver 10× alone"*,
    then becomes a filter that kills work before it is built.
- **Censoring makes robustness part of speed.** An agent loop that resolves 70% of briefs quickly and silently drops
  the rest is not ten times better. Restricted mean time with unresolved briefs held at the cap makes that
  impossible to hide.
- **It defends against all three of our characteristic biases:**
  - AGY's inflated baselines, because the incumbent arm is measured, not assumed;
  - my choice of baselines, because the strongest pipeline per brief is selected on a separate tuning set;
  - COD's under-ambition, because the arms are frozen and the result is whatever it is.
- **My cost-of-power objection was wrong.**
  - Suppose the per-brief log-speedup has a standard deviation of 1.0, which is pessimistic for agent-driven runs.
    With 20 paired briefs the standard error is 0.22, and the 95% interval is ×/÷ 1.55.
  - 10× (log 2.30) and 3× (log 1.10) differ by 1.2 log units, or 5.4 standard errors.
  - So 20 held-out brief variants are enough, not "too few to confirm 10×". The arms themselves can reuse published
    reference code (Playground, the DIAL-MPC repository, RoboGrammar) as development-time oracles out of process,
    which plan §30.2 rule 3 permits.
- **It becomes the project's loss function.**
  - Each milestone gate can be phrased as a ratio of restricted mean times on a family of briefs, with an interval.
  - Briefs, accepted designs and failure logs become the training corpus for everything that learns: the family
    policy, and two of the new ideas below (§5.2 and §5.3).
- **Best case.**
  - M1 measures the incumbent's phases on four briefs.
  - M2 runs the first head-to-head.
  - By M3, about 20 held-out variants give an interval narrow enough to confirm or refute 10×.
  - If the claim is real, this is the evidence an outside reader accepts. If it is not, the benchmark names the phase
    that blocks it.

### 4.2 From AGY: the universal zero-training controller, in AGY's full-fidelity form (AGY #3)

**Why it is strongest.**

- It is the only proposal that retires the plan's weakest load-bearing bet (BET-06) with published evidence:
  full-order sampling MPC, real-time at 50 Hz on a quadruped in DIAL-MPC.
- COD scored this version 429 for its cost. The steelman is for the part I argued against: **planning on the
  execution model, not a coarser one.**

**The steelman.**

- **A coarse planner is a second model, and its errors correlate with design features.**
  - Foot radius against stair nosings.
  - Wheel speed against the Cayley bound (FROB-REF-212).
  - Thin swing clearances against a 5-ms step.
  - The plan's §19.6 detectors protect *policy results* against solver artifacts. A coarse planner reintroduces an
    artifact amplifier *inside the controller*, where it can bias *design rankings* without failing any detector.
  - Planning on the execution model makes the planner gap zero by construction, removing one confounder from every
    comparison.
- **Full fidelity is affordable without AGY's mega-kernel numbers.**
  - A pairwise decision with 24 pairs of 5-s episodes at about 5.1 M planning steps per world-second is about
    1.2 × 10⁹ steps. At the plan's own 2.0 M steps/s target, that is about 10 minutes on one 4090.
  - Combined with paired multilevel estimation and the importance-sampled battery of §5.3 (each 4–10× fewer
    episodes), it is about 1–3 minutes.
  - Either figure is 5–50× below B0's 50 minutes, with no training, and the "established" verdict still comes from
    the referee.
- **It makes stepping speed the single lever on evaluation cost.** That gives kernel work (AGY #1, my idea 16) a
  measurable target instead of a slogan.
- **The rest of AGY's version helps too:**
  - seeding only 25% of samples from the gait atlas keeps the prior from locking out asymmetric and hybrid bodies;
  - Philox-keyed sampling makes the controller as replayable as the physics;
  - the controller doubles as the teacher for any later learned policy.
- **Best case.**
  - At M1, on `batch-cpu`, five templates walk 5 m.
  - At M2, every design in the 200-design corpus has a competent controller with zero training.
  - BET-06 passes, or is replaced outright.
  - Easy-to-plan bias, its strongest remaining objection, is handled by §5.1.

### 4.3 The strongest idea from each that I did not have

- **COD #3, randomized referee audits.**
  - Promotion in a real system will be adaptive: agents, value-of-information rankings and Explain all decide which
    episodes deserve the referee after seeing GPU results.
  - COD's logged propensities are the only proposal in any slate whose estimate stays unbiased under that targeting.
    It works with any predictor and turns "which runs did we check?" into an auditable column.
  - Its weakness, wide confidence sequences at low audit rates, is exactly what same-address pairing fixes. Together
    they beat either alone.
- **AGY #4, structural link synthesis.**
  - It closes a loop no robot simulator closes. An agent that can return a sized, fatigue-checked link with its true
    inertia has delivered a design a human can act on.
  - The parts that matter for the 10× metric (distal inertia, modal stiffness against control bandwidth) feed back
    into dynamic rankings.
  - The sibling solvers exist. What is missing is bounded work: a path from density or soup to a sealed B-rep, and
    load envelopes taken from the referee.

---

## 5. Three ideas none of us thought of

**How I checked novelty.** Each idea was checked against all 90 ideas in the three slates and against the plan. Each
answers a failure the reviews exposed: our 10× claims rest on rankings made under imperfect controllers, imperfect
task specifications, and wasteful test batteries.

### 5.1 Rank designs by *achievable* performance, estimated from curves of score against controller budget

**The gap.**

- Every fast evaluation path in the three slates scores a design under a controller of fixed competence: sampling MPC
  at K samples, a family policy, or gait tables.
- COD's objection to my #2 and AGY's objection to my #3 are the same objection: the score measures body ×
  controller, and the ranking can favour the bodies that are easiest to control.
- The nearest thing in any slate is one slot in AGY #5, `do(2× Dial-MPC rollout budget)`, used as a single
  diagnostic on a failure.
- Nobody proposed estimating each design's performance *as controller effort grows*, and ranking on that.

**Mechanism.**

- **Where.** `frob-control` gains a budget coordinate in `ControllerId`, b = (K, N_iter, H), on a fixed ladder such as
  K ∈ {64, 256, 1,024}. `frob-opt` gains the estimator, and `frob-arena` the allocation.
- **Data.** Rows are keyed by (design, world address, budget). Every budget runs on the *same* addresses, so
  differences across budgets are paired and have low variance.
- **Model.** Fit J_d(b) = J_d∞ − a_d·(b/b₀)^(−c) per design.
  - The exponent c is shared across a morphology family and fitted hierarchically across its designs.
  - a_d ≥ 0, and J must be monotone in b.
  - That leaves two free parameters per design, identifiable from three budget points.
  - The output is J∞ with an interval, plus a *saturation index*: the share of the predicted remaining gain already
    realized.
- **Allocation: escalate the contenders and the unsaturated.**
  - Every candidate runs at b₀.
  - A design whose upper bound on J∞ crosses the current top-k boundary moves up to the next budget, then the next.
  - The expected cost is about 1 + 0.3×4 + 0.1×16 ≈ 4× a single budget per design, against 16× for running
    everyone at the top budget.
- **Labels.**
  - The measured contrast at the highest budget reached goes through the normal e-process and promotion path. It is
    the only thing that can become `established`.
  - J∞ is labelled `Estimated` and steers allocation.
  - A design is flagged `controller-limited` when J∞ − J(b_max) exceeds the decision margin.
  - The same machinery applies to learned policies, with fine-tuning steps as the budget axis.

**What an agent sees.**

```
$ frob eval compare quad-a quad-b --task stairs-0.15 --controller mpc-sample --budget-curve --json
  # J(b): a 0.61 → 0.70 → 0.72 (saturated 0.91);  b 0.55 → 0.68 → 0.76 (saturated 0.62, controller-limited)
  # fixed-budget verdict at K=64: A better;  estimated J∞: B ≥ A with probability 0.83 → escalating B to K=4096
```

**Lever.**

- TARGET: top-3 regret against an oracle at most half that of ranking at a fixed budget, at equal total compute.
  Equivalently, match the regret of running everyone at the top budget for no more than 0.3× its cost.
- Baselines: fixed-budget MPC ranking (my #2, AGY #3), uniform high budget, and per-design PPO.
- **Why it matters for 10×.** Every claimed 50–200× gain over per-design PPO holds only if cheap-controller rankings
  agree with what a well-trained controller achieves. This mechanism makes the disagreement visible and fixes it
  where it matters, near the cutoff.
- **Uniqueness is partial.** A wrapper could run DIAL-MPC at several sample counts. It would lack:
  - common random numbers across budgets on logical addresses, without which differences between budgets are too
    noisy to fit;
  - mixed-design batches, without which each (design, budget) pair is a separate compile and launch.

**Falsifier and fallback.**

- **Cheapest, at M1 on `batch-cpu`.** Run 20 variants from two templates at K ∈ {32, 128, 512} on shared addresses.
  - If no two designs' curves cross, rankings do not depend on budget. The idea is unnecessary, which is good news
    for #2 and AGY #3.
  - If more than 20% of curves are not monotone, the curve model is wrong.
- **At M3.** Take 30 designs in three families with an oracle: PPO per design over five seeds, plus MPC at K = 8,192.
  Kill if either holds:
  - regret at equal compute does not fall by at least 30%;
  - the extrapolated J∞ is further from the oracle than J(b_max) for more than half the designs.
- **Fallback.** Report fixed-budget scores with the saturation flag only.

**Sibling needs.** `fs-rand` addresses, `fs-eproc` for the measured contrast, and a small curve fitter written
in-house. None of it blocks.

### 5.2 Attack the task specification before attacking the design

**The gap.**

- The plan's exploitation detectors (§19.6) ask whether a good score is a *solver artifact*. They re-run under half
  the time step, the next tier, perturbed cards, the other contact law, and the mirrored policy.
- A behaviour that is physically real but *unintended* passes all of them:
  - sliding on the belly where walking was meant;
  - tumbling down a stair flight to reach the goal region;
  - riding a wall;
  - succeeding only at the final instant the predicate checks.
- AGY #5's exploit quarantine and COD #12's scene mutations both attack the design or the solver. Nothing in any slate
  attacks the *specification*.
- Faster search makes this worse. An agent that searches ten times faster finds the cheapest loophole ten times
  sooner.

**Mechanism.**

- **Where.** A verb `frob task audit <task>` in `frob-verbs`, with search in `frob-arena` and `frob-control`, and
  results in Chronicle keyed by the hash of the specification.
- **Ghost bodies.** A library of deliberately capable or degenerate bodies, run in the task's own scene and scored by
  the task's own cost and success predicate:
  1. a floating centroidal body that applies bounded wrenches at contact sites inside friction cones;
  2. the current design family with actuator limits relaxed 3× and collision enabled on every link;
  3. degenerate shapes: a sled, a ball, a tumbling box, a pogo.
- **Search.**
  - Sampling MPC (my #2) and CMA-ES over open-loop parameters (`fs-dfo`, which has CMA-ES with BIPOP).
  - MAP-Elites novelty search (`fs-archive`) over behaviour descriptors: a histogram of which body sites touched the
    ground, the range of base orientation, the airborne fraction, and the sliding fraction.
  - The goal is *many different* high-scoring behaviours, not one optimum.
- **Intent invariants.** Typed predicates evaluated from logs the plan already keeps:
  - contact at sites not designated as feet, wheels or tracks;
  - excursions of log(R) beyond a bound;
  - airborne and sliding fractions, the latter from tangential speed at stance contacts;
  - positive work by contacts, which is handed to §19.6;
  - use of scene features off the route graph;
  - success reached only inside the last 5% of the horizon.
- **Output.**
  - Each high-scoring ghost behaviour that violates an invariant comes back with a replay link and a proposed
    specification clause.
  - The agent either accepts the clause, which creates a new specification version, or marks the behaviour as
    intended, which is recorded.
  - Campaigns cite the specification version they ran under.
  - During a campaign, the winning episodes of the top-k designs are checked against the invariants. That flags
    designs "winning for the wrong reason".

**What an agent sees.**

```
$ frob task audit galley-delivery@v3 --budget gpu-s=120 --json
  # 3 loopholes: ghost:sled reaches goal (score 0.91) with belly contact 78% of the time → add: contact ⊆ {feet}
  #              ghost:relaxed-quad succeeds only at t ≥ 0.96·T → add: hold goal region for 2 s
  #              ghost:ball rides the counter edge off the route graph → add: stay within route corridor
```

**Lever.**

- TARGET: on a corpus of 30 specifications with planted loopholes and 30 clean ones, detect at least 80% of loopholes
  before any design campaign, at no more than 5% of a campaign's compute, with no more than 20% false alarms.
- **Payoff.** One undetected loophole invalidates a whole campaign: hundreds to thousands of evaluations, plus every
  agent turn spent on them.
- Under COD's benchmark, a brief "resolved" with a degenerate design counts as wrong, so this protects the resolution
  rate the benchmark measures.
- **Why incumbents lack it.**
  - Any optimizer *could* attack a specification.
  - The invariants need contact provenance per site, an energy ledger and a route graph. FrankenRobotics has all
    three by design; incumbents expose only some of them.

**Falsifier and fallback.**

- Kill if any of these holds:
  - detection on the planted corpus is below 60%;
  - false alarms exceed 30%;
  - the plain sampling MPC on the *actual* design finds the same loopholes at the same cost, which would mean ghost
    bodies add nothing.
- **Fallback.** Keep only the in-campaign checks of intent invariants on winning episodes.

**Sibling needs.** `fs-dfo` and `fs-archive` (both exist). Everything else is the plan's own: MPC, ledger, Chronicle.

### 5.3 A calibrated item bank for the test battery, sampled by importance

**The gap.**

- In a population decision, cost scales with designs × battery items × seeds.
- Every slate spends effort on the designs axis (racing, value of information, carving) or the physics axis (tiers,
  kernels).
- None models the *items*, even though most battery items say nothing about close designs:
  - easy scenes that every candidate passes;
  - impossible scenes that every candidate fails.
- The nearest neighbours are AGY #7 (value of information over design × rung × scene slices) and COD #8 (a fixed
  audit distribution for rare events). Neither keeps a persistent model of items across campaigns.

**Mechanism.**

- **Where.** `frob-arena` holds the battery and the bank, `frob-opt` the comparisons, and Chronicle the archive.
- **Item and bank.**
  - An item is (scene instance, task, disturbance law, family of initial conditions).
  - The bank stores calibrated parameters per morphology family, versioned with the battery.
- **Model.**
  - For success, a multidimensional two-parameter logistic model: P(success | d, i) = σ(a_iᵀθ_d − b_i), with
    abilities θ_d ∈ ℝ^k for k = 3–5 (traction, clearance, balance, power).
  - For continuous metrics, a heteroscedastic Gaussian model.
  - Both are fitted by regularized maximum likelihood on every (design, item, outcome) row in the archive, across
    campaigns and agents.
- **Sampling for a contrast between A and B.**
  - Use the proposal q(i) = (1 − ε)·q*(i) + ε·p(i), where p is the frozen battery law and
    q*(i) ∝ p(i)·√E_model[D_i²], with D_i = Y_B,i − Y_A,i on paired seeds. q* is the variance-optimal proposal under
    the model.
  - The defensive mixture ε (around 0.1) bounds the weights at 1/ε.
- **Estimator.**
  - Ĉ is the mean of (p(i)/q(i))·D_i over the items sampled. It is **unbiased for the frozen battery-law contrast
    whether or not the item model is right**. The model buys efficiency only.
  - Bounded increments feed the `fs-eproc` confidence sequences, so the verdict stays anytime-valid.
  - If promotion targets are adaptive, this composes with COD #3's audits.
- **Bank maintenance.**
  - Items that do not discriminate across the active population are down-weighted in q and never removed from p, so
    the estimand never drifts.
  - Item parameters refresh as campaigns run.
- **For agents.** "These six items separate A from B", each with its difficulty, ranked by discrimination. This is
  also a diagnostic of *which capability* differs.

**What an agent sees.**

```
$ frob eval compare quad-a quad-b --battery interiors@4 --sampling item-bank --json
  # 212 episodes (uniform-equivalent: 1,140); B better on success by 6.1% [2.4, 9.8] (anytime-valid, α = 0.05)
  # separating items: galley-wet-turn@3 (a=2.1), stair-0.18-nosing@1 (a=1.8), threshold-2cm-cart@5 (a=1.6), …
```

**Lever.**

- The idealized arithmetic:
  - Suppose the paired difference D is ±1 on a fraction f of items and 0 elsewhere, with net mean μ.
  - Uniform sampling has variance f − μ². The optimal proposal has variance f² − μ².
  - At f = 0.1 and μ = 0.05 that is 0.0975 against 0.0075, about 13×.
- With model error, the defensive mixture, and variance between seeds within an item (which item selection cannot
  remove), I expect 4–8×.
- TARGET: at least 4× fewer episodes at equal interval width for close pairwise and top-k decisions, and about 10×
  when no more than 10% of items discriminate.
- This multiplies with paired multilevel estimation, which reduces the cost of each episode, rather than overlapping
  with it.
- **Uniqueness is partial.** A wrapper could importance-sample a battery. What it lacks is:
  - an archive keyed by content identity across all agents and campaigns, which is what makes calibration amortize;
  - paired logical addresses;
  - e-process machinery that keeps the verdict valid.

**Falsifier and fallback.**

- Run a replay study at M3 on a recorded full matrix: about 40 designs × 150 items × 4 seeds, about 24,000 episodes,
  which is affordable on `trj` with the gait-table controller.
- Kill if any of these holds:
  - the reduction in episodes at equal width is under 2× for the 20 closest pairs;
  - coverage falls more than 2 points below nominal;
  - item parameters calibrated on one family predict held-out designs no better than per-item means.
- **Fallback.** Stratified sampling by interior archetype, which captures part of the gain with no model.

**Sibling needs.**

- `fs-eproc` (exists).
- `fs-uq::product_importance` does defensive importance sampling over *Gaussian germs*, so the discrete version over
  items is new code, though small.
- The response-model fit is in-house, a few hundred lines.

### 5.4 Considered and not included

| Idea | Why not |
|---|---|
| Dimensional-analysis (scaling-group) transport of evaluations | The most elegant use of Lie theory I found. Discrete actuator catalogs and absolute scene geometry break similarity in most real design questions, so it collapses to a dimension reduction for surrogate models. |
| Speculative evaluation of an agent's likely next edits during its think time | Bounded by how much compute overlaps think time, at most about 2×. |
| Symbolic regression of design laws from the archive | Good explanations for agents, but a surrogate with an unclear decision lever. |

---

## 6. What I would now tell the plan to do

1. **Adopt the benchmark first.** Use COD's protocol (§4.1), measure the incumbent's phase times at M1, and phrase
   every gate as a ratio of restricted mean times.
2. **At M2:**
   - ragged cohorts, with group-valued joint configuration and tree-sparse factorization by level;
   - sampling MPC as the default controller, with the planner step per design under FROB-REF-212 and a check of
     ranking stability across planner fidelity;
   - paired multilevel estimation with frozen coefficients, ρ measured on contrasts, and the same planner on every
     tier, with COD's audits for adaptive promotion.
3. **At M1–M3: the three new mechanisms (§5).** Each has a cheap falsifier first:
   - budget curves on `batch-cpu` at M1;
   - specification red-teaming as soon as tasks exist;
   - the item bank by replay at M3.
4. **Counterfactual probes at M3**, as diagnostic interventions only (COD #5's rules plus AGY #5's slot placement),
   handing off to full comparisons.
5. **Options, not commitments:**
   - planner-fan-out fusion after profiling (M3);
   - catalog carving (M1 lane);
   - bounded structural synthesis for finalists, with loads from the referee (M4);
   - the family policy, for screening only (M5).
