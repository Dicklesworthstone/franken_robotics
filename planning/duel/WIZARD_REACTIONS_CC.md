# WIZARD_REACTIONS_CC.md — Phase 6: Reveal, Steelman, Blind-Spot Probe (lane `CC`)

| Field | Value |
|---|---|
| Run | MOR `r1-20261121` · Brenner `RS-20261121-franken-robotics-10x` · `Q-001` · `2026-11-21` |
| Inputs | `WIZARD_SCORES_COD_ON_CC.md` (Codex), `WIZARD_SCORES_AGY_ON_CC.md` (Gemini), `WIZARD_IDEAS_{CC,COD,AGY}.md`, `MOR_CASE.md`, audit `AUD01-C001-C016` |
| Method | Every numerical catch was recomputed before I conceded it. Each `CONCEDE` names the fix to `CC`'s design. Each `REBUT` names what still stands and why. |

## 1. Reveal and Rebuttal

### 1.1 The Scores

| `CC` item | COD | AGY | My response |
|---|---:|---:|---|
| `CC-01` MorphoJIT | 864 | 945 | Concede the engineering catches (§1.3). Keep the thesis. Adopt Codex's edit-to-first-accepted-result gate. |
| `CC-02` Equivariant Gait Atlas | 842 | 925 | Concede the full-space multiplier check. Rebut the claim that H/K must prove a pattern is realized. |
| `CC-03` Falsify → shrink → attribute → repair | 820 | 910 | Concede the 6.4× step ratio. Rebut that the unit is repairs per hour, but that composite is still unproven. |
| `TA-P1` Algebra quotients the workload | 804 | 940 | Concede 9.8× < 10×. Adopt the audit of discarded designs. |
| `CC-07` Operadic robot algebra | 782 | 905 | **Concede: Burnside gives 86,185, not 83,333.** Dedup is 2–11.6×, depending on which symmetry group is admitted. |
| `TA-P2` Warp diversity + race annealing | 763 | 935 | **Concede: CRN pairing is broken, and occupancy is 2 warps/SM, not 4.** |
| `CC-08` Causal tribology | 735 | 855 | **Concede: the mask sits outside the divergence, and the Hersey term has units.** |
| `CC-09` Lifecycle compression | 715 | 845 | Concede. Reuse the rainflow residue that `fs-solid` already carries. |
| `CC-04` Complete-solution synthesis | 713 | 885 | Concede on completeness. Rebut on distinctness and reality, which α-theory already tests. |
| `CC-10` Spec-to-silicon | 686 | 870 | Concede. Retract "reward hacking ≤ 0.1%". |
| `CC-05` Breadth/depth dual silicon | 683 | 895 | Concede. The honest diversity ratio is 5× against both 4090s, as my own table says. |
| `TA-P3` Contract-stratified certificates | 621 | 930 | **Concede: δ₁ is ill-typed, and there are 27 strata, not 14.** The typed product survives. |
| `CC-06` Proof-carrying shields | 515 | 835 | **Concede on both counts: the friction quantifier points the wrong way, and the box is ignored.** I accept being ranked last. |
| **Mean** | **734** | **898** | Range: COD 515–864 (349 points); AGY 835–945 (110 points). |

### 1.2 Calibration of the Reviewers

Codex's review is **the most accurate document in the duel about my slate**. Of its roughly 25 substantive catches, I concede 22 outright and rebut 3 in part. I adopt its re-ranking: `CC-07` rises above `CC-04`/`CC-05`, and `CC-06` drops to last.

AGY's review carries little discriminating information:

- **It endorsed three of the errors Codex found:**
  - it praised "friction-cone monotonicity" as a "major improvement" (§1.3, C1);
  - it called the Latin-square pairing a "masterstroke" (C12);
  - it said the δ-bound "solves Paradox 3 cleanly" (C15).
- **It repeated my unsupported "≤ 0.1% reward-hacking" number as fact.**
- **It misread a ceiling.** It said `CC-04` breaks a "degree-40 Stewart ceiling", but `real_rur` allows `MAX_DEGREE = 256`. The binding limits are variable count and dimension, not degree 40.

AGY's real catches were the E022–E024 sibling facts — `fs-qty` lifts, the `fln-anvil` stubs, and univariate-only `fsym-formal` — which I had already conceded in `WIZARD_SCORES_CC_ON_COD_AGY.md` §4. It also made three useful engineering notes, which I accept, subject to measurement:

- benchmark size class L against thread-per-env ABA;
- use `StorageModePrivate` for read-only texture banks;
- cap fused active sets on the GPU.

**I decline AGY's 945/940/935.**

### 1.3 Concessions and the Fix Each One Forces

| # | Item | Catch, re-derived | Fix to the `CC` design |
|---|---|---|---|
| C1 | `CC-06` | **The friction quantifier runs the wrong way.** I wrote inf over λ ∈ 𝒦(μ_min) and claimed "the smallest cone suffices". For forces the environment chooses, an infimum over a *smaller* set is *larger*, so 𝒦(μ_min) is optimistic and the proof is unsound. | Split λ = (λ_ctrl, λ_env). Forces the robot commands must be feasible in 𝒦(μ_min), which is conservative. Environment reactions inside the infimum range over 𝒦(μ_max) = ⋃_{μ ≤ μ_max} 𝒦(μ). This adds a declared, monitored μ_max to the `TA-P3` assumption set. |
| C2 | `CC-06` | **The closed form ignores the box.** With R = I, u_π = (1, 0), box [0,1]², and u₁ + u₂ ≥ 1.5, my formula gives (1.25, 0.25), which is outside the box. The true optimum is (1, 0.5), confirmed by brute force. | For m = 1 and diagonal R: u(λ) = clip(u_π + λR⁻¹a, lo, hi). Then aᵀu(λ) is piecewise-linear and nondecreasing, so sort its ≤ 2n breakpoints and solve exactly. In the counterexample this gives λ = 0.5 → (1, 0.5). For m ≥ 2, replace 64-set enumeration with fixed-iteration projected Newton plus a **post-hoc KKT certificate** (feasibility, dual sign, complementarity, stationarity ≤ ε_fp). If the certificate fails, use the verified backup. |
| C3 | `CC-06` | **4k FLOPs is 20% of a 20k-FLOP step, not "≤ 10%".** A margin on h alone also misses inter-sample, actuation and reset error. | Retract the FLOP claim and gate on measured wall time. Add Lipschitz·Δt inter-sample and actuator-tracking terms to the margin. The proof path goes through `fln-kernel` reflection (E023/E024). |
| C4 | `CC-07` | **I wrote "≈10⁶/12" and called it "exact Burnside".** The D₆ cycle index gives (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = **86,185**. | Dedup is **11.6×**. If the plant, controller and task admit only a sagittal mirror (three transpositions), the count is (10⁶ + 10³)/2 = 500,500, which is only **2.0×** dedup. Count only automorphisms of the whole closed loop. |
| C5 | `CC-07` | **A shared moving base makes the Delassus operator dense across leaves.** | Apply the C001 standard I used on `AGY` #1 to myself. Certificate reuse holds for hot leaves only; base-coupled contact must invalidate. Use Codex's mirror-swap / shared-bus gate. |
| C6 | `CC-08` | **The mask outside the divergence creates mass.** 𝟙·∇·(h v̄) ≠ ∇·(𝟙 h v̄). The difference, h v̄·∇𝟙, is a spurious line source at the edge of the contact patch. | Use conservative finite volumes with face flux F_f = 𝟙_f h_up (v̄·n)_f \|f\|, plus a tracked wheel-borne film reservoir whose pickup and deposit fluxes cancel exactly. Gate on the global mass residual (compensated sums on Metal). |
| C7 | `CC-08` | **ηv/p has units of length.** | Use the dimensionless Π = ηv/(p ℓ_c), with ℓ_c the contact-patch length, or a calibrated law. |
| C8 | `CC-08` | **Two water subcycles need 354 GB/s.** The 5 mm LBM cube needs 1.22 GB per env, so 128 envs come to 156 GB. Continuous Dirac losslessness says nothing about discrete energy. | The training lane runs one subcycle. The LBM lane is capped at ≤ 64 envs on the M5 (≈ 78 GB). Use discrete port splitting and report the energy residual. |
| C9 | `TA-P2` | **CRN pairing is broken** (verified). With w = (m + e·g) mod 64, a world is shared within an epoch only by morphologies with m ≡ m′ (mod 64), e.g. {0, 64, …, 448}. Every cross-residue comparison is unpaired under (world, epoch) seeds. | Keep the Latin rotation for *training-load balance* only. Do **paired evaluation** at checkpoints: every surviving morphology runs the same (world, replicate) seeds at the same training stage. This also fixes the confound where an epoch-3 policy is compared with an epoch-40 one. Rebalance the rotation after each `fs-race` halving. |
| C10 | `TA-P2` / `CC-01` | **Occupancy: 512 warps over 256 SMs is 2/SM, not 4** (I counted one card). B = 16,384 lanes fills just **4.2%** of 12,288 warp slots. | Raise B to ≥ 65,536–131,072 (2,048–4,096 warps, 8–16 per SM), or pack several small envs per warp. The 55–80 µs round-1 estimate is withdrawn. |
| C11 | `CC-01` / `TA-P2` | **Warm CUDA Graph replays have near-constant host launch cost.** My "512 nodes × 2–4 µs" baseline overcharged the incumbent. | **Withdraw the derived 8–20×.** The surviving lever is novel-topology admission and concurrency, measured as edit-to-first-accepted-result against warm graph buckets and the W02 ragged scheduler, with contacts matched. |
| C12 | `CC-01` | **A warp cannot switch pipelines by hash.** `PFAFF`/`LOOP` rows need a coupled solver. Twist exponentials need a safe small-angle limit. | Promote only at acknowledged dispatch boundaries, with versioned state maps. Tier 0 handles rigid trees only; loops go to the constrained lane. Use branch-free polynomial sinc/cosc. |
| C13 | `CC-02` | **A restricted Fix(K) orbit can hide unstable transverse multipliers.** The twisted action must be a symmetry of the controller, guard/reset and terrain, not just the body. | After converging in Fix(K), compute the **full** monodromy spectrum with `fs-spectral::floquet` (including nonnormal growth). Admit a pattern only if it is stable in full space. Adopt COD W07's gate: refuse grazing cases, cosine ≥ 0.95. |
| C14 | `CC-03` | **Steps: 100M/(6.0 + 9.2 + 0.41)M = 6.41×.** Even if the baseline shrinks too, (100 + 9.61)/15.61 = 7.02×. | The step ratio is < 10×. Any 10× in repairs per hour must come from fewer agent repair iterations, so add an AGY #10-style iteration gate (≥ 1.5×). Say "1-minimal under the tested edit relation". Label privileged-sensing witnesses as *plant-model* feasibility. |
| C15 | `TA-P3` | **δ₁ is ill-typed.** I defined it as P(breach ∧ silent) and then multiplied by r_breach, which double-counts. Also, a monitor cannot label its own silent breaches. | Define δ₁ := P(silent \| breach) under a declared breach-sampling law. Then P(viol/episode) ≤ E[N_breach]·δ₁ + E[N_alarm]·δ₂ + δ₃ (Markov on counts plus a union bound), **if** a coverage lemma shows every violation falls in one of those events. Estimate against independent hidden-state labels (simulation ground truth, mocap). **Retract "valid under shift"**: shift moves δ₁ itself. |
| C16 | `TA-P3` | **Per-foot open/stick/slip gives 27 orbits, not 14.** (3⁴ + 3·3²)/4 = 27. My 14 assumed one global slip flag. | Recomputed for the per-foot model: **27 strata and 162 directed one-foot transition lemmas** (vs. 7 and 16 for the binary model), about 189 SOS problems × 3–4 s ≈ **10–13 min** on the M5. The architecture survives, and the "14" is labeled as the coarser model. InEKF covariance becomes a monitored assumption, not a bound. |
| C17 | `CC-09` | **Passing only a histogram between windows drops open half-cycles.** "1,800×" counts substeps, not wall time. | `fs-solid::fatigue` already "retains rainflow residues across calls" (line 7), so pass the residue stack between windows. Add the planted overload and regime-switch controls. |
| C18 | `CC-10` | **Mixed-unit min/max robustness is wrong.** Future-time STL operators need delayed verdicts. Instruction counts are not WCET. | Normalize predicate margins. Use only the past-time STL fragment for online reward. Require linked-binary stack and real-board timing receipts. "≤ 0.1% reward hacking" is unsupported and retracted. |
| C19 | `CC-05` | My table already said "≈ 10× per device (≈ 5× vs both 4090s sharded)", so the 10× headline used the weaker baseline. | Headline **5×**. Measure the generalization effect separately. Use `fs-tropical::max_cycle_mean` (line 457) for the cycle rate, not `critical_circuit` (line 552). Gate on the probed working set. |
| C20 | `TA-P1` | **A2 = 32/(3 + 0.27) = 9.79×** before confirmation. A screening step can discard the true winner. | Adopt Codex's blinded, stratified audit of discarded designs and its planted poorly-charted winner. A3's 24× stays a hypothesis until each stage is timed independently. Add COD FTA-P1's ballast fixture as the positive control. |

### 1.4 Rebuttals (Partial, Bounded)

- **R1 · `CC-04`: distinctness and reality.** Codex says these "require additional witnesses". They do, but the witnesses already exist. alphaCertified (Hauenstein–Sottile) certifies distinct solutions (disjoint certified neighborhoods) and real solutions (the conjugate test) in the same pass. This is a port, not open research. **Completeness is conceded:** the output is "N certified roots; generic count N_gen from a trace test; Complete iff equal", and otherwise `Partial`.
- **R2 · `CC-02`: realization.** `CC-02` never claimed H/K proves a pattern is realized. Enumeration *proposes* seeds, and shooting plus continuation realizes or refuses them. That is why the budget counts 8 patterns × 50 points, failures included. Wording fix only.
- **R3 · `CC-01`: the all-small case.** Low occupancy comes from the batch-size parameter (C10), not the architecture. Codex's own steelman reaches the same thesis.

## 2. Steelman (Phase 6.75) — `COD` FTA-P3, the Observable Energy Budget (`.fenergy`)

Every other safety proposal in the duel — my `CC-06`/`TA-P3`, AGY #3/FTA-P3, COD W04 — tries to certify the robot's *contact dynamics*, the one thing no simulator models faithfully. `.fenergy` certifies something else: a conservation inequality across a physical interface. The energy delivered toward a guided stop is ½mv² + ∫max(P_elec, 0)dt + E_stored + E_reserve, and it must stay ≤ W_abs,min. Each robot-side term is measured: encoder speed times a known mass, plus motor electrical input, which upper-bounds the mechanical work the drive can add. The environment-side term is a bench-measured lower envelope of the absorber's work. Passive impacts and friction can only dissipate or redistribute energy, so the inequality holds for **every** contact sequence, friction coefficient and unmodeled bounce. The contact model drops out by construction, which makes the certificate immune to the sim-to-real gap that undermines the others. It needs no SOS proofs, none of the reflection bridges that E023/E024 show are missing, and no statistical monitor. Its runtime check is one integrator and one comparison per tick, cheap enough for a 1 kHz MCU loop or a GPU env-step. Its cutoff (50.5 ms, or 45.5 ms with reserve) is a closed-form controller parameter an agent can read and act on. It earns the hardest claim axis cheaply: one drop-weight test of the absorber and one power-meter test of the drive give `ValidatedPhysics` for the whole class. The receipt also survives every policy retraining while the runtime check holds, so a campaign that ships 100 policy revisions pays for the evidence once — a genuine evidence-amortization 10× with no simulator in the loop. It is in the regulator's own language, since ISO/TS 15066 already expresses transient-contact limits as transferred energy derived from force and pressure thresholds. And it composes: in a port-Hamiltonian stack (`fs-phs`, W01), energy is exactly what crosses ports, so `.fenergy` is a port's supply-rate certificate, ready to become a proof-obligation family next to barrier proofs. Calling it "one tray cart" misreads it. The same pattern covers every guided stroke that ends at a stop: linear-axis end stops, grippers closing on fragile items, drawer and door actuation, bumper-limited bases, and lift platforms.

## 3. Blind-Spot Probe (Phase 6.9)

Codex's three blind spots — the observation-alias witness, the partial-order timing check (DPOR), and the α-spend ledger — are all strong. The ledger is the most important for agent-driven campaigns. Below are three **other** gaps. I grepped all six idea and score files for each one, and the "evidence" line cites what that check found.

### B1 · Replay determinism is *asserted* everywhere and *engineered* nowhere

**Evidence.** Four results rest on replay or pairing, and none builds it:
- `CC-03` forks "4,096 exact branches (`DeterministicPerDevice`)";
- AGY #10 claims "`BitwisePortable` … keyed Philox";
- COD W05 replays content-addressed tiles;
- `TA-P2`, COD's paired 95% protocol, and every e-race assume CRN pairing.

**What is missing.** The words "atomic" and "reduction order" appear in no mechanism. CPU `fs-contact` has `ExecMode::Deterministic`, but GPU scatter does not. Contact-impulse accumulation, BVH refits, warp reductions with data-dependent active masks, and two-stream / two-GPU timing are all order-nondeterministic in f32. Seeding Philox fixes the *random numbers*, not the *arithmetic*.

**Why it matters (10× breakpoint).**
- **Divergence time.** An f32 1-ulp perturbation (2⁻²⁴) reaches O(1) after t ≈ ln(2²⁴)/λ ≈ 16.6/λ seconds. With a finite-time Lyapunov rate λ > 8.3 s⁻¹ — plausible near impacts and stick-slip, and measured here, not assumed — a "replayed" 2-s branch is a different trajectory.
- **CRN pairing.** The variance of a paired difference scales with 1 − ρ. If nondeterminism drops the pair correlation from 0.95 to 0.5, every paired race needs **10× more episodes**.
- **Exact Shapley.** It turns into a noisy estimate. At noise σ equal to the effect size d, resolving each coalition at z = 2.5 needs 2(zσ/d)² ≈ 13 replicates, so **1,024 → ≈ 13,300 rollouts**.

**Artifact.** `frob-determinism`:
1. Order-independent scatter: stable-key sort plus segmented reduction on both backends, or Q32.32 int64 fixed-point atomics where the backend supports 64-bit add (CUDA). Probe Metal's 64-bit atomic coverage; do not assume it.
2. A fused per-K-step `StateHash` (xxh3 over SoA state).
3. A `DeterminismClass {BitwisePerDevice, Shadowed(ε, T), None}` field on every receipt that uses replay or pairing.
4. A typed refusal of exact-Shapley and CRN claims when the class is `None`.

Cross-backend Metal/CUDA pairs are declared unpaired: no f64, different FMA contraction, and `MathMode::Fast` hazards (E025).

**Gate and kill.**
- **Measure:** replay 1,000 contact-rich 2-s windows twice per device, with float atomics vs. the deterministic path. Record the first divergence step, the paired ρ, and top-1 attribution stability on the AGY #10 / `CC-03` planted-cause corpus.
- **Kill:** if the float path is already bitwise in ≥ 99.9% of windows *and* ρ is unchanged, drop the engineering and keep only the hash gate.
- **Cost bound:** at most 10% env-step overhead.

### B2 · Model-form robustness: no lane asks whether a ranking survives a change of admissible physics

**Evidence.** All three lanes issue `EvaluatedDesign` under **one** contact formulation, with `ValidatedPhysics` deferred. Nothing in between catches the classic failure: optimizers (RL and co-design alike) exploit solver artifacts. Grep hits for "model-form" or "rank stability" across all six files: **0**.

**Planted artifacts that a single-model ranking happily promotes:**
- A box-linearized friction cone grants **√2·μ** of tangential force along diagonals, 41% free traction for diagonal gaits.
- Penetration recovery at high stiffness injects energy, so a hopper can "gain" height.
- Tolerance-limited PGS under-resolves stacked contacts.

**Artifact.** `frob-rankcheck::model_form`. Re-evaluate each final round's top-k and runners-up under admissible variants: compliant-penalty vs. complementarity vs. barrier contact; circular vs. pyramidal cone; stiffness ×{½, 2}; Δt ÷ 2; tolerance ÷ 10. Add **COD FTA-P3's inequality turned on the simulator itself**: a per-step discrete work–energy audit, ΔE_mech − W_act ≤ ε, which flags solver-created energy. Rollouts whose return depends on flagged steps are re-run with those contacts damped. The output is `EvaluatedDesign<ModelRobust{variants, top-k overlap, energy-audit}>`. A ranking that flips under any variant becomes `ModelConditional` and **cannot spend hardware-prototype budget**.

**Why it is structural.** It is not a 10× by itself. It keeps every *other* claimed 10× from being a simulator-artifact 10×, and it is the cheapest proxy for the sim-to-real gap available before hardware. **Cost:** variants re-evaluate fixed policies, with retraining only for the top 2, so ≤ 5–10% of campaign compute in TA-P1 terms.

**Gate and kill.**
- **Pass:** the check flags both planted artifacts (diagonal-pyramid gait, energy-pumping hopper) and passes a planted genuine winner.
- **Demote:** if over 20 campaigns the top-8 overlap across variants is ≥ 7/8 in ≥ 95% of them with zero energy flags, demote to a periodic audit.

### B3 · Robot–home co-design: the dual of the minimal counterexample is a **minimal amendment**

**Evidence.** All three lanes treat the home as fixed (W08, W09) or adversarial (`CC-03`, AGY #8). Every repair they search lives in the robot: morphology, policy, controller. Grep hits for an environment edit as a design variable: **0**.

Yet home-robot failures cluster on a few cheap features: thresholds, cords, rugs, door closers, dark corners. A $15 threshold ramp can stand in for a morphology campaign.

**Artifact.** `frob-amend`:
1. A typed catalog of about 40 amendments: threshold ramps (code-legal slopes as typed constraints), door-closer torque settings, door holders, cord raceways, rug grippers, fiducials, motion lights, grab handles, and COD FTA-P3 bump-stop absorbers. Each carries a cost, an install class (renter-safe, landlord, contractor), and geometry.
2. When `CC-03` emits a minimal counterexample world, or W08 emits a "cannot fit" certificate naming the bottleneck, run **ddmin's dual**: the smallest kit of ≤ 3 amendments that flips the verdict, checked by the same solvability oracle and W08 certificates. There are C(40,1) + C(40,2) + C(40,3) = **10,700** kits, and each check reuses the robot's existing policy. Geometry and perception amendments need only re-evaluation, not retraining.
3. The output is a W09-style installer packet. COD W03's refusal witness gains a **remedy**: "this SKU fails doorway D; a $12 ramp makes the mission feasible."

**10× unit.** (Home, mission) pairs unlocked per dollar of *system* cost (robot BOM + kit) versus morphology upgrades, on a held-out corpus of 200 procedural and `fdgr`-scanned homes × 5 missions. The claim survives only if kits fix a large share of failures without retraining.

**Gate and kill.**
- **Kill** if ≤ 3-item kits fix < 30% of a cheap SKU's failures.
- **Kill** if a single kit verdict fails its physical bench test (ramp traction and slope, closer torque).

**Claim axes.** The solvability flip is `EvaluatedDesign`. An installed-amendment trial is `ValidatedPhysics`.

---

**Net:** Codex's review made the `CC` slate more correct. I concede 20 design fixes (C1–C20, 22 catches in all), and the two hardest hits are `CC-06`'s quantifier and box and `TA-P3`'s δ₁ typing. The merged plan should take Codex's measurement protocol, COD FTA-P3 as a new proof-obligation family, and B1 as a precondition of every replay or pairing claim in all three lanes.
