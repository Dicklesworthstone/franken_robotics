# GAPFILL 3: delta pass against the two revised source plans

Scope: the hybrid parts `parts/P00.md`–`P12.md` (§1–§37) and `parts2/` (§38–§47, appendices) as they stand after the
reconciliation pass, checked against what changed in the two source plans after the last audit:
`ASTRA_DELTA_AFTER_5afef55.diff` (two commits, 246 inserted lines) and `NEON_DELTA_AFTER_d18b5a1.diff` (one commit,
read as a word-level diff because of its line lengths). Section (b) of `GAPFILL_1.md` was read first; its 27 changes
are not repeated here, and rows that touch them say so.

Method. Every changed passage was read in the diff and, where needed, in the plan. Each change was looked up in the
parts by grep with several synonyms, and the hits were read. Every reported fact about a sibling library was checked
in the sibling's source under `/Users/jemanuel/projects/` (frankensim at `483d09c682f2`, franken_threed at
`bf7abe16367e`, frankentorch at `8165d6a18028`, fastmcp_rust at `117388bcc169`); the table says what the reading
confirmed. Every number that enters an insertion block or a correction was recomputed for this file, in exact
rationals where the claim is exact; the few figures taken on report are marked. Each ORIGINAL span in section (b) was
checked by script to occur exactly once in its file, on one physical line.

Status tokens: COVERED (the mechanism and its specifics are in the parts), WEAKER (the mechanism is there and a named
element is missing or wrong), MISSING, NOT TO TAKE (with the reason).

**Headline findings.**

- Three statements in the parts are now contradicted by counterexamples the source plans added, and each is repeated
  in several places: the stance certificate says its allocation "is affine in (W, P)" when no affine law attains that
  width (an affine controller needs 7/3 times the half-width); the damping-injection backup is claimed never to
  increase stored energy, which is false for a sampled and held command (a hand case adds 1.5 J); and the per-column
  residual bound of the response batches is not a bound when the residual is formed in working precision (a
  single-precision case reports zero error for an error of 9.9 × 10⁻⁹).
- Six new sibling facts were confirmed in source: two cases in which the `fs-sos` bound verifier returns a value that
  is not a bound; the suite's G1 catalog builder omits welded masses; the differential of the SE(3) exponential
  refuses beyond 32 m of translation; the swept-sphere helper covers straight motion only; the legacy transformer in
  `fs-phs` has the opposite sign to the mapped interconnection; and `f3d-math` renormalizes a ray direction under a
  scaled instance.
- One known answer already adopted from NEON (§38.17, the second thermal card) repeats a lever the source has since
  changed: regearing from 12 to 16 leaves the runaway in place. The source's replacement claim, that a ratio of 22
  eliminates it, is also not taken as stated: the rate changes sign and the equilibrium rise is 2,240 K.

---

## (a) Table of changes

Corrections are numbered C01–C34 in section (b); insertion blocks are B1–B18 in section (c).

| # | Plan | Change | Status | What the hybrid should do |
|---|---|---|---|---|
| 1 | ASTRA §0, §18.10 | Docking with a finite position tolerance over a continuous gain interval: least guaranteed tolerance (√1745 − 41)/32 m ≈ 24.1624 mm at exact rest; 20 mm infeasible; 25 mm with the rational probe 5/8 | MISSING | §25.15 says the amended game "requires solving". Recomputed here: derivation, witness policy run on the raw dynamics over 133,185 exact cases, and the exclusion at 1/50. B2; C04 |
| 2 | ASTRA §18.7, §18.12 | The controller class changes the stance optimum: an allocator restricted to one affine law of the observed load moments needs b ≥ 1/2 m against 3/14 m; the checker validates coverage "by an admissible control law"; the four witnesses relabelled as one affine law must be rejected | WEAKER, and the parts state the opposite | `P07` §25.5.3 writes `"allocation is affine in (W, P)"`. The four vertex allocations violate the parallelogram identity of an affine map; the implementable law is their bilinear interpolant. The affine optimum is the equal-spring load sharing of §25.13. B1; C01, C02, C03 |
| 3 | ASTRA §18.5 | Pairwise independence is not joint independence: seven reads at 3/4, pairwise tables equal to the independent ones, best success 55/64 (0.859), below the 0.9 that 3807/4096 suggested | WEAKER | A-17 and §25.10 distinguish independent from fully shared errors only. Enumeration recomputed. B3 |
| 4 | ASTRA §18.14 | The physical label message is priced: three labels need 2 clean bits, 3 bits with one known erasure, 5 bits with one unknown flip; a four-bit deadline excludes the last | MISSING | §25.18 counts labels and never their transmission. B3 |
| 5 | ASTRA §18.8 | Four-law probe with the delayed law admitted; the two pads share q and τ: independent extremes give a margin of −0.0776 N where the shared box gives at least 4.07 N; earliest separating dwell 0.5 s at radius 3/2; gap 0.01287; 900 of 900; time ratio 5/3; the 225 earlier reversals stand | MISSING | Recomputed: −0.0776 N; shared minimum 4.72 N; gaps 0.0129, 0.0662 and 0.1090 at radii 3/2, 2 and 3; 900 of 900 at each. A tie-break by gap would choose radius 3. B4 |
| 6 | ASTRA §18.15 | A small Kleene increment is not a fixed point: F(m) = 1 + 0.99·m moves by 5 g at 99.5 kg and the least feasible mass is 100 kg; an upper sizing witness needs F(U) ≤ U, or residual/(1 − q) with a proved contraction | WEAKER | §25.21.1 stops on "a fixed antichain", which is exact on finite chains and undefined on continuous ones. B5 |
| 7 | ASTRA §19 | A complete Pareto claim needs a coverage test on discrete builds: (0, 6), (4, 4), (6, 0) are all nondominated and no weighting selects (4, 4) | MISSING | §25.3.2 offers the claim `pareto` with no disposition rule for omitted options. B5 |
| 8 | ASTRA §18.4 | `fs-sos`: `Poly::new` drops a trailing coefficient of 2⁻⁵⁰; `certified_bound_on` rounds its last subtraction to nearest | MISSING. Confirmed by reading | `fs-sos/src/lib.rs:80–82` trims trailing coefficients below 10⁻¹⁵; `:289–308` returns `lower_bound − envelope.hi()` in round-to-nearest. The parts cite the verifier as sound (§25.4.9). Their own `sos-gram` kind is checked in rationals, so no certificate is affected. B17 row 13; C05 |
| 9 | ASTRA §2, §4.3 | Folding a fixed joint must preserve mass, first moment, inertia and named attachments; the suite's G1 builder keeps 30 links and 31.96314202 kg and omits 1.377 kg of welded links; a 320 N static support passes on the reduction and fails on the source assembly | MISSING. Confirmed by reading and by summing | `fs-mbd/src/robot_models.rs:398–403` states the omission; the 30 masses sum to 31.96314202 kg (summed here). §14.12 has no folding rule and uses the builder as the humanoid reference. B6; B17 row 9 |
| 10 | ASTRA §5.1 | `Se3::exp` accepts a pure translation of 33 m while its left and right Jacobians return `IllConditioned` at a series limit of 32; for pure translation the Jacobians are exactly I ± ad/2 | MISSING. Confirmed by reading | `fs-ga/src/lie.rs:46` and `:1044–1049`. A derivative refusal must name an owner's numerical limit, not a singular pose. B17 row 10 |
| 11 | ASTRA §5.4 | Two transformer conventions in `fs-phs` give opposite driven directions and both conserve power; bind the authored shaft direction and require a signed motion witness | MISSING. Confirmed by reading | `fs-phs/src/interconnection.rs:91`: "The legacy transformer has the opposite sign to the canonical map". A power balance cannot see the reversal. B7; B17 row 11 |
| 12 | ASTRA §5.4 | Coupled transmissions: efforts map back by Cᵀ, so one rotor on θ = 10·(q₁ + q₂) delivers only (t, t) and marginal limits admit the impossible (1, −1); rotor inertia is Cᵀ·J_r·C once (0 J and 2 J against a diagonal 1 J and 1 J); share the map across dynamics, allocation and feasibility | WEAKER | §15.9 has square constant couplings and their reflected inertia. Missing: non-square couplings, the torque set as an image, and the rotor-elimination paragraph that assumes one rotor per joint. B7; C34 |
| 13 | ASTRA §5.6 | Admitted sequences of stride modes: A and Aᵀ each have multipliers 1/2 and the two-stride product has 3/4 + 1/√2 > 1; test products or exhibit a common contraction | MISSING | §22.9.4 uses a common Lyapunov matrix for impact orderings inside one stride only. B13 |
| 14 | ASTRA §6, §13.1 | Swept bounds must cover the link's path: a chord padded by the sphere radius misses an obstacle on the arc; expand by the sagitta R·(1 − cos(Δθ/2)); the pinned helper encloses straight centre motion | WEAKER. Sibling fact confirmed | `fs-contact/src/broad_phase.rs:146–156` documents the limit. §18.4 says "bounds swept over the sub-interval" and gives no construction. B9; B17 row 12 |
| 15 | ASTRA §7.1 | A signed distance pulled back through a non-uniform scale is not a world distance: unit sphere scaled (2, 0.5, 1) reads 0.20 for a clearance of 0.10 m; bounds [σ_min·d, σ_max·d]; normals by A⁻ᵀ | WEAKER | §19.4 bakes scales before fields are computed, which avoids the error; there is no rule for a scale that survives to query time and no negative control. B10 |
| 16 | ASTRA §2, §7.1 | Instanced ray queries: a renormalized direction changes the ray parameter; convert by s_W = s_L/‖A⁻¹·d‖; the pinned FrankenThreeD primitive normalizes | MISSING. Confirmed by reading | `f3d-math/src/ray.rs:487–490` calls `Vector3::transform_direction`, which normalizes (`vector3.rs:708–719`). B10; B17 row 14 |
| 17 | ASTRA §8.1 | An integrating accelerometer responds to an impact's velocity jump: add R_WSᵀ·(v_P⁺ − v_P⁻) once per impact in the exposure; a rebound from −1 to +1 m/s reads +200 m/s² over 10 ms while every smooth sample is zero | MISSING | §20.5 defines the IMU from point acceleration only; §20.23 covers force sensors. B11 |
| 18 | ASTRA §8.1 | Encoder sampling: a modulo-one-turn position at 100 Hz is identical at +80 and −20 turns/s; unwrapping needs Ω·Δt + 2ε < 1/2; ambiguity is kept after a packet loss | MISSING | §13.11 covers the static turn count, not sampling. B11 |
| 19 | ASTRA §8.1 | A six-axis load cell is a cut in the load path with side, sign, origin, frame and tare: +80.38 N for a stationary 2 kg foot under a 100 N reaction, −23.62 N in swing at 2 m/s² | WEAKER | §20.5 calls it "a port reading". B11 |
| 20 | ASTRA §9.1, §11 | Timestep choice reverses a sampled loop's stability: exact held-input 64 of 64, one kick–drift step 1 of 64, two half steps 15 of 64 (stable and late); a rational referee and native single-precision kernels on three devices agreed | WEAKER | A-22 models the hold exactly for actuator loops and CX-262 has one ordering case. Missing: the tier's own integrator at the task's decimation, the fixed-clock rule for refinement, and counts for a deadline. Recomputed, with the decimation sweep. B8 |
| 21 | ASTRA §9.2 | A policy can make the solver fail preferentially: 2 of 2 valid with 98 unresolved is a rate in [2%, 100%]; report unresolved counts by policy; end the advantage trace at the gap | WEAKER | The denominator rules exist (§24.10, §28, §29). Missing: per-arm counts and the trace cut. C29, C30 |
| 22 | ASTRA §11.2 | Writable views are checked for independence separately from bounds: strides (1, 1) alias two outputs | WEAKER | §33.11 row 3 converts strides; §21.9 checks buffers, not views. C31; B18 |
| 23 | ASTRA §11.3 | The moment merge in single precision: shards at 4,095 and 4,097 give variance 0 by raw moments and 1 by the centred merge, on all three devices | COVERED | §23.9 already merges (n, μ, S) by the centred formula. The known answer is offered. B18 |
| 24 | ASTRA §13 | Trigger delay and mount translation are confounded on a straight approach: 20 ms at 2 m/s equals 40 mm; reversing without rotating separates them by 80 mm | MISSING | §30.8 lists eight structural aliases; this is a ninth. Added under its own gate so that "8 of 8" is not renumbered. B16 |
| 25 | ASTRA §15 | An energy tank must reserve the work a held command can supply: a sampled damper of 300 N·s/m on 1 kg at 10 ms turns +1 m/s into −2 m/s and does +1.5 J where initial power times duration predicts −3 J | WEAKER, and the parts state the opposite | §22.12.3 and PO-SHIELD-04 claim that damping injection never increases stored energy. True in continuous time; false for a held command above the gain bound 2·M̄ − h·K_d ⪰ 0. B14; C15–C19 |
| 26 | ASTRA §13.1, §16 | Gate A04 and eight work-package rows reworded to name the new fixtures | NOT TO TAKE | The hybrid's packages and gates are fixed by the Phase 2 brief; the fixtures arrive through B6–B11 |
| 27 | ASTRA §20 | Bundle statistics: 178 payload digests, 31 deliberate regressions, a verifier that rejects wrong decision counts | NOT TO TAKE | Process provenance of another document. The parts repeat none of the old counts. The three new recorded pilots are listed for Appendix K at the end of this file |
| 28 | NEON §10 | Reality of a root is never inferred from α(f, Re z) < α₀ alone; counterexample z³ + z at 0.001 + 1.001·i | COVERED | C-34 and §25.21.4 already require α(f, z) < 0.03 with ‖z − Re z‖ < 1/(20·γ). The counterexample was recomputed (0.0021 and 0.0010; 1.001 against 0.033) and is offered. B18 |
| 29 | NEON §12.2 | Tube inclusion: erosion by −E, and K_n ⊖ (E ⊕ (−E)) for true-state cells (the earlier E ⊖ E was wrong) | WEAKER | §22.12.5 erodes by E, right only for a symmetric error set; the Appendix D row has no erosion. C13, C14; B18 |
| 30 | NEON §15 | I − M is singular on every autonomous orbit (the phase direction has multiplier 1); the adjoint is solved on a section or with a bordered, phase-anchored system | WEAKER | §22.9.1 defines only the clock-forced stroboscopic map, while GATE-GAIT-01 tests two autonomous walkers; §22.9.5 would report a fold on each. B12 |
| 31 | NEON §6.5 | Power-invariant dq with K_t = K_e, voltage ceiling V_bus/√2, pole pairs, absolute temperature | COVERED | ADR-SINEW-05, §15.8, §15.5 |
| 32 | NEON §6.2 | The SE(3) Cayley map is the matrix map with translation (I − ω̂/2)⁻¹·v, not the direct product | COVERED | C-05 states exactly this. A negative control is offered. B18 |
| 33 | NEON §9.2 | Film update: face mobility as the arithmetic mean of cubes, so a front advances onto a dry cell; sub-cycling for thin fluids | WEAKER | §19.10.3 has the step bound and the refusal; h_f is undefined. C24; B18 |
| 34 | NEON §9.5 | Pose-graph residual orientation: r₁₂ ≈ δ₂ − Ad(T₂₁)·δ₁ | WEAKER | §20.14 says "adjoint blocks on the tail side" and gives no formula. Derived here from the right-perturbation convention of §11. C25; B18 |
| 35 | NEON §12.3 | Shield bound at episode level: E[N_b]·δ₁ + P(N_a ≥ 1)·δ₂ + δ₃, δ₁ defined as a ratio of expectations, recalibrated when the policy changes | WEAKER | Four places carry E[N_alarm]·δ₂ with a per-alarm δ₂, which can exceed 1 and leaves a late backup failure unassigned. C06–C12 |
| 36 | NEON §10.3 | Strictly positive pivots for pseudo-inertia | COVERED | T-19 |
| 37 | NEON §6.3 | A residual certificate needs higher precision or a rounding margin γ_n·(‖A‖·‖x̂‖ + ‖b‖) | WEAKER | §18.19 forms the residual "in the precision of the step" with no margin. C20–C23; B18 |
| 38 | NEON §6.3 | Backlash tracked on a rotor coordinate distinct from the output encoder | COVERED | §15.6; KA-511 |
| 39 | NEON §10.1 | Kernel IR: loop-carried values as loop arguments with a yield | WEAKER | §21.7 says single assignment with bounded loops and gives no way to carry a value across iterations. C26 |
| 40 | NEON §10.1 | Kernel IR: value select in place of predicated blocks, comparison and cast operators, composite constructors | NOT TO TAKE | §21.7 already has `select` and `compare`, rule 8 of §21.6 makes a mask a select, and control structure is hand-written in the shared dialect, not in the IR |
| 41 | NEON §10.1 | An explicit, correctly rounded fused multiply-add for the double-float product | NOT TO TAKE | Binding decision G4 and ADR-SPARK-10: the split product is used and a fused operation is banned in batched kernels |
| 42 | NEON §6.1, §6.2, KA-02, KA-03 | Corrected verification numbers (holonomy sign test 9.68 × 10⁻⁴ against 1.66; energy oscillation figures) | NOT TO TAKE | The parts repeat none of them; their sign and order tests are exact identities (T-20) and step-halving ratios |
| 43 | NEON GB-08, KA-09 | Thermal lever changed from regearing 12 → 16 to 12 → 22, "flipping" the runaway rate | WEAKER as to the hybrid's row; NOT TO TAKE as stated | §38.17 row 1 repeats the 12 → 16 lever. Recomputed: at 16 the rate is still +0.002254 s⁻¹; at 22 it is −0.000337 s⁻¹ with an equilibrium rise of 2,240 K and the limit reached at 128.6 s. C33 |
| 44 | NEON §8.2 | Preflight continues past marginal findings with advisories; its result is a claim product, never a bare proof | COVERED | §14.11: `--all`, advisories, an evidence class per stage |
| 45 | NEON App. D | A reward that pays for safety alone is collected by standing still; a progress predicate closes it | WEAKER | §24.20 has foil bodies and no idle arm. B15 |
| 46 | NEON App. D | Repairs to the worked wheel-leg example (a steering joint so the base is controllable, a continuous spin joint, hip clearance, sensor masses, 46 tools) | NOT TO TAKE | Another document's example. The checks that caught the defects exist: stage 4 growth vector, stage 3 interpenetration, §13.11 |
| 47 | NEON §2.3, §22.1 | Dependency ledger details beyond the 27: `sha2 = "=0.11.1"` in fastmcp_rust; optional features of `fs-ad`; capsule crates behind four leaves; crate counts; the spelling of the indirect-dispatch binding | COVERED | §33.12 already has 22 coupled crates, 12 direct and 10 transitive, and "8 of 22". The `sha2` pin was confirmed (`fastmcp_rust/Cargo.toml:84`) and is isolated by R3, since `frob-mcp` is its own workspace |
| 48 | NEON §6.3, §8.3, §9.3, §17 | Layout arithmetic (112-byte port record, 1,872-byte stride, 4 bytes per clearance cell, fluid and world-bank memory) | NOT TO TAKE | As GAPFILL_1 (b) row 26: unmeasured figures for a design the hybrid does not use |
| 49 | NEON §6.5, §10.2, §17 | Ground-effect ratio relabelled constant-power; firmware lowering by floating point on parts with a hardware unit; scope of the CUDA unsafe leaf | NOT TO TAKE | The hybrid's rotor port has no ground-effect factor, its firmware export is [M] with T-14 as its proof, and its crate map differs |
| 50 | NEON App. D | A temperature margin's scale is written in kelvin | WEAKER | §23.5 writes "12 °C" and "10 °C" for a margin and a scale where C-30 writes kelvin. C27, C28 |
| 51 | NEON §7.3 | Construction of quaternionic blocks from a matrix-unit basis | NOT TO TAKE | Do-not-inherit list (G16). §13 refuses a constituent of quaternionic type |
| 52 | NEON §15 | An ε-suboptimal policy carries an O(√ε) error in an envelope-theorem gradient | NOT TO TAKE | §22.9.6 uses the implicit-function form with a fixed controller, as GAPFILL_1 (b) row 13 found |
| 53 | NEON §2.3 | Matrix types of the Metal thread-group library compile in half, single and brain-float precision | NOT TO TAKE | G5 rejects tensor-core half precision; §18.12 and §23.8 do not adopt matrix accelerators |
| 54 | NEON §12, §22.2, §29 | Backup-controller trait placement, the auditor's link set, milestone markers on beads | NOT TO TAKE | As GAPFILL_1 (b) rows 15 and 27: not the hybrid's crate map or milestones |

Counts over the 54 rows: COVERED 8, WEAKER 21, MISSING 13, NOT TO TAKE 12 (row 43 is counted as WEAKER, since it
carries a correction). Of the 27 ASTRA rows: 1 COVERED, 11 WEAKER, 13 MISSING, 2 NOT TO TAKE. Of the 27 NEON rows: 7
COVERED, 10 WEAKER, none MISSING, 10 NOT TO TAKE.

---

## (b) Exact corrections to existing text

Each ORIGINAL is a verbatim span that occurs exactly once in the named file and lies on one physical line (checked by
script against the files as they stood when this was written). Where a sentence continues on the next line of the
file, the replacement reads correctly with that unchanged continuation. Replacements are single lines; rewrap after
applying.

**C01** The stance specimen: the law is not affine.

```text
FILE: parts/P07.md
ORIGINAL:      "hull": "convex combinations of the listed vertices; allocation is affine in (W, P)",
REPLACEMENT:   "law_class": "adjustable", "law": { "kind": "multilinear-vertex", "arguments": ["P_x", "P_y"], "note": "bilinear interpolation of the four vertex allocations over the load rectangle; no single affine law attains this width" },
```

**C02** The checker's coverage rule (the next line of the file begins "have is in the observation contract").

```text
FILE: parts/P07.md
ORIGINAL:    validates convex coverage of the payload envelope, and checks that the information the allocator is assumed to
REPLACEMENT: validates coverage of the payload envelope by a law of the declared class (here the bilinear interpolant of the four vertex allocations; the same vertices labelled as one affine law are rejected, by the controller-class rule of this chapter), and checks that the information the allocator is assumed to
```

**C03** Theorem card T-22.

```text
FILE: parts/P01.md
ORIGINAL:    feasibility is convex in the load, and every smaller b is infeasible by the dual witness.
REPLACEMENT: feasibility is convex in the load for an allocation chosen after the load is observed (the bilinear interpolant of the four witnesses is such a law; no single affine law attains this b, and the affine class needs b = 1/2 m), and every smaller b is infeasible by the dual witness.
```

**C04** §25.15: the amended game is now solved.

```text
FILE: parts/P07.md
ORIGINAL:    fails; a finite position tolerance requires solving the amended game. The rule: **gain support, delivery
REPLACEMENT: fails; the amended game with a finite position tolerance is solved for a continuous gain interval in this chapter's subsection on docking to a finite tolerance (least guaranteed tolerance (√1745 − 41)/32 m, about 24.2 mm, at U = 3/4 and ε = 1/64). The rule: **gain support, delivery
```

**C05** §25.4.9: the `fs-sos` verifier is not sound at the pin.

```text
FILE: parts/P07.md
ORIGINAL:    solve as untrusted; `fs-ivl` provides outward-rounded intervals, affine forms and multivariate Taylor models
REPLACEMENT: solve as untrusted (two cases in which that verifier's returned value is not a bound at the pin, a trimmed coefficient and a final subtraction rounded to nearest, are recorded with the foundation facts of §33, and no `fs-sos` value enters a certificate unchecked); `fs-ivl` provides outward-rounded intervals, affine forms and multivariate Taylor models
```

**C06, C07** C-31: the residual-risk bound at episode level (two consecutive lines of the file).

```text
FILE: parts/P01.md
ORIGINAL:         E[N_breach]·δ₁ + E[N_alarm]·δ₂ + δ₃, where δ₁ is the probability that a monitor stays silent given a
REPLACEMENT:      E[N_breach]·δ₁ + P(N_alarm ≥ 1)·δ₂ + δ₃, where δ₁ is the expected share of breaches on which every monitor stays
```

```text
FILE: parts/P01.md
ORIGINAL:         breach, δ₂ that the fallback fails given an alarm, and δ₃ the gap between template and body.
REPLACEMENT:      silent, δ₂ the probability that an episode with at least one alarm ends in a violation, and δ₃ the probability of a violation with neither a breach nor an alarm (the gap between template and body).
```

**C08–C10** §22.12.3: the bound, its three numbers, and their scope.

```text
FILE: parts/P06.md
ORIGINAL:        P(violation) ≤ E[N_breach]·δ₁ + E[N_alarm]·δ₂ + δ₃
REPLACEMENT:     P(violation) ≤ E[N_breach]·δ₁ + P(N_alarm ≥ 1)·δ₂ + δ₃
```

```text
FILE: parts/P06.md
ORIGINAL:    with δ₁ = P(monitor silent | breach), δ₂ = P(violation | alarm) and δ₃ = P(violation with no breach and no
REPLACEMENT: with δ₁ = E[N_silent]/E[N_breach], the expected number of breaches with no alarm over the expected number of breaches (zero when no breach is expected), δ₂ = P(violation in the episode | N_alarm ≥ 1), which covers several alarms in one episode and a backup that fails late, and δ₃ = P(violation with no breach and no
```

```text
FILE: parts/P06.md
ORIGINAL:    see reports δ₁ = 0 from its own log. The bound is not claimed under a shift of the world distribution, because a
REPLACEMENT: see reports δ₁ = 0 from its own log. The first term is Markov's inequality on the number of silent breaches and the second is a conditional probability, so no term grows with the number of alarms in an episode. The three numbers are estimated for one deployed policy and are estimated again when the policy changes. The bound is not claimed under a shift of the world distribution, because a
```

**C11** §22.21 (the previous line of the file ends "the bound of §22.12.3 is at least").

```text
FILE: parts/P06.md
ORIGINAL:    E[N_alarm], which says nothing. The alarm is correct and too late.
REPLACEMENT: P(N_alarm ≥ 1) = 1, which says nothing. The alarm is correct and too late.
```

**C12** Appendix D, the shield row.

```text
FILE: parts2/P16a.md
ORIGINAL:    | Residual risk of a shield | Per episode at most E[N_breach]·δ₁ + E[N_alarm]·δ₂ + δ₃ |
REPLACEMENT: | Residual risk of a shield | Per episode at most E[N_breach]·δ₁ + P(N_alarm ≥ 1)·δ₂ + δ₃, with δ₂ the probability that an episode with at least one alarm ends in a violation |
```

**C13, C14** Tube inclusion: the erosion is by −E.

```text
FILE: parts/P06.md
ORIGINAL:    action only if R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n ⊖ E for every guard G_mn that may be enabled, where R_mn is the
REPLACEMENT: action only if R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n ⊖ (−E) for every guard G_mn that may be enabled (cells hold measured states, the true state is the measured state plus an error in E, and A ⊖ B = {x : x + B ⊆ A}; the erosion is by −E, which equals E only for a centrally symmetric error set; where cells hold true states and the whole belief set must stay in one cell, the condition is R_mn(K_m ∩ G_mn, D) ⊆ K_n ⊖ (E ⊕ (−E))), where R_mn is the
```

```text
FILE: parts2/P16a.md
ORIGINAL:    | Tube inclusion | R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n for every simultaneously enabled guard G_mn |
REPLACEMENT: | Tube inclusion | R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n ⊖ (−E) for every simultaneously enabled guard G_mn, for cells of measured states with true = measured + e, e ∈ E |
```

**C15–C19** Damping injection is passive in continuous time, and under a gain bound when sampled.

```text
FILE: parts/P06.md
ORIGINAL:    With collocated outputs y and K_d ⪰ 0, the command u = −K_d·y gives dH/dt ≤ −yᵀ·K_d·y ≤ 0 in the port model of C-12: stored energy never increases (PO-SHIELD-04).
REPLACEMENT: With collocated outputs y and K_d ⪰ 0, the continuous-time command u = −K_d·y gives dH/dt ≤ −yᵀ·K_d·y ≤ 0 in the port model of C-12. A command sampled and held for a period h keeps the stored energy from increasing at the ticks only under the gain bound 2·M̄ − h·K_d ⪰ 0 on the reflected inertia M̄ (PO-SHIELD-04, and the sampled-damping subsection of this chapter): 300 N·s/m held for 10 ms on a free 1 kg slider turns +1 m/s into −2 m/s and adds 1.5 J.
```

```text
FILE: parts/P06.md
ORIGINAL:    | PO-SHIELD-04 | Under damping injection the port model's stored energy is non-increasing. | A polynomial identity for Lemma (a theorem card in §12); the ledger's monotonicity on replay |
REPLACEMENT: | PO-SHIELD-04 | Under damping injection the port model's stored energy is non-increasing: in continuous time for every K_d ⪰ 0, and at the ticks of a sampled, held command when 2·M̄ − h·K_d ⪰ 0 (with actuation latency, when the exact one-tick map of A-22 contracts in the energy norm). | A polynomial identity for Lemma (a theorem card in §12) for the continuous statement; an exact rational check of the gain bound per fitted backup; the ledger's monotonicity on replay |
```

```text
FILE: parts/P06.md
ORIGINAL:    | Damping-injection backup | Stored energy non-increasing in the ledger | A negative damping gain breaks monotonicity and is caught |
REPLACEMENT: | Damping-injection backup | Stored energy non-increasing in the ledger at every tick, for gains inside the sampled bound | A negative damping gain breaks monotonicity and is caught; so does a gain above the bound (300 N·s/m held 10 ms on a free 1 kg slider: +1.5 J) |
```

```text
FILE: parts/P06.md
ORIGINAL:    PO-SHIELD-04 (damping injection never increases stored energy in the port model) is offered to §12 as a theorem
REPLACEMENT: PO-SHIELD-04 (damping injection never increases stored energy in the port model, in continuous time and, under the sampled gain bound, at the ticks of a held command) is offered to §12 as a theorem
```

```text
FILE: parts2/P13a.md
ORIGINAL:    | Damping-injection backup | Stored energy is non-increasing in the ledger | ledger residual bound (§29) | G1 · referee | A negative damping gain breaks monotonicity and is caught |
REPLACEMENT: | Damping-injection backup, gain inside the sampled bound 2·M̄ − h·K_d ⪰ 0 | Stored energy is non-increasing in the ledger at every tick | ledger residual bound (§29) | G1 · referee | A negative damping gain breaks monotonicity and is caught; a gain above the bound adds energy and is caught (300 N·s/m held 10 ms on a free 1 kg slider: +1.5 J) |
```

**C20–C23** The per-column bound of the response batches needs the rounding margin of the residual.

```text
FILE: parts/P03.md
ORIGINAL:    1. the residual r = b − H·x̂ is formed explicitly, not estimated from the solve;
REPLACEMENT: 1. the residual r̂ = b − H·x̂ is formed explicitly in the working precision, not estimated from the solve, together with a bound ρ on the rounding error of forming it: ρ = γ·‖ |H|·|x̂| + |b| ‖₂ with γ = (n + 1)·u/(1 − (n + 1)·u), n the row length and u the unit roundoff (2⁻²⁴ in single precision); a residual formed in a wider format uses that format's u;
```

```text
FILE: parts/P03.md
ORIGINAL:    4. the bound ‖x̂ − x*‖₂ ≤ ‖r‖₂/λ_lo is attached to the column.
REPLACEMENT: 4. the bound ‖x̂ − x*‖₂ ≤ (‖r̂‖₂ + ρ)/λ_lo is attached to the column. Without ρ the expression is not a bound: in single precision H = 3 and b = 1 give x̂ = 0.33333334, a computed residual of exactly 0 and a true error of 9.9 × 10⁻⁹; with ρ = 2.4 × 10⁻⁷ the bound is 7.9 × 10⁻⁸.
```

```text
FILE: parts/P03.md
ORIGINAL:    carries the bound ‖x̂ − x*‖ ≤ ‖b − H·x̂‖/λ_lo for a certified lower bound λ_lo on the smallest eigenvalue, and a
REPLACEMENT: carries the bound ‖x̂ − x*‖ ≤ (‖b − H·x̂‖ + ρ)/λ_lo, with ρ the rounding margin of the computed residual (§18.19), for a certified lower bound λ_lo on the smallest eigenvalue, and a
```

```text
FILE: parts/P01.md
ORIGINAL:      counterfactual right-hand sides [F]; each column carries the bound ‖x̂ − x*‖ ≤ ‖b − A·x̂‖/λ, with λ a lower
REPLACEMENT:   counterfactual right-hand sides [F]; each column carries the bound ‖x̂ − x*‖ ≤ (‖b − A·x̂‖ + ρ)/λ, with ρ the rounding margin of the computed residual (§18.19) and λ a lower
```

**C24** §19.10.3: the face mobility of the film update.

```text
FILE: parts/P04.md
ORIGINAL:    thickness, v̄ the mean tangential velocity of the patch) and the second is gravity spreading in the lubrication limit.
REPLACEMENT: thickness, v̄ the mean tangential velocity of the patch) and the second is gravity spreading in the lubrication limit, with the face mobility taken as the arithmetic mean of the cubes, h_f³ = ½·(h_L³ + h_R³), so that a wet cell beside a dry one has a nonzero flux and the front advances (a harmonic or geometric mean gives zero and the spill never spreads). Under the step bound below the gravity term keeps h ≥ 0.
```

**C25** §20.14: the residual of one edge, written out.

```text
FILE: parts/P04.md
ORIGINAL:    adjoint blocks on the tail side. The Gauss–Newton step solves the normal equations
REPLACEMENT: adjoint blocks on the tail side: with corrections applied on the right of each mount transform (§11), an edge from sensor 1 to sensor 2 has r₁₂ = log(T̂₁₂⁻¹·T₁₂) ≈ δ₂ − Ad(T̂₂₁)·δ₁, where T̂₁₂ is the predicted and T₁₂ the measured pose of sensor 2 in the frame of sensor 1, and T̂₂₁ = T̂₁₂⁻¹. Using Ad(T̂₁₂) on the tail is the planted frame mutant. The Gauss–Newton step solves the normal equations
```

**C26** §21.7: values carried across loop iterations.

```text
FILE: parts/P05.md
ORIGINAL:               for i in 0..N { ... }                                       N a literal or a shape-class capacity
REPLACEMENT:            for i in 0..N carry (a = a₀, …) { …; yield (a′, …) }       N a literal or a shape-class capacity; a value that crosses iterations is a loop argument, so every name is still assigned once
```

**C27, C28** §23.5: a temperature margin is a difference.

```text
FILE: parts/P06.md
ORIGINAL:       thermal margin of 12 °C is 0.05 in metres and 12 if the clearance is written as 50 mm. With scales of 0.10 m
REPLACEMENT:    thermal margin of 12 K is 0.05 in metres and 12 if the clearance is written as 50 mm. With scales of 0.10 m
```

```text
FILE: parts/P06.md
ORIGINAL:       and 10 °C the margins are 0.5 and 1.2, and the clearance binds in every unit system.
REPLACEMENT:    and 10 K the margins are 0.5 and 1.2, and the clearance binds in every unit system. A temperature margin is a difference, so its scale is written in kelvin.
```

**C29** §23.6, rule P-9: the trace ends at an invalid step.

```text
FILE: parts/P06.md
ORIGINAL:    | P-9 Valid samples only | Invalid worlds and padded elements have weight zero in every loss and statistic. |
REPLACEMENT: | P-9 Valid samples only | Invalid worlds and padded elements have weight zero in every loss and statistic, and the advantage trace of the valid samples before an invalid one ends there, bootstrapping from the last valid value as at a truncation. |
```

**C30** §24.10: unresolved outcomes are reported per arm.

```text
FILE: parts/P08.md
ORIGINAL:    Both-succeed and neither-succeed counts, invalid pairs and the worst-case range for missing pairs are reported. A
REPLACEMENT: Both-succeed and neither-succeed counts, invalid pairs and the worst-case range for missing pairs are reported, with unresolved counts given per arm and per action regime, because a policy can enter states where the solver fails preferentially: a candidate with 2 successes in 2 valid trials and 98 unresolved has a complete-case rate of 100% and a success rate anywhere in [2%, 100%], against a baseline at 60 of 100, and its unresolved worlds are re-run on the referee before a ranking that depends on them is promoted. A
```

**C31** §33.11, row 3: writable views.

```text
FILE: parts/P12.md
ORIGINAL:    zero-copy contract. Neither library is wrong by itself; the trap is at our boundary.
REPLACEMENT: zero-copy contract. A writable view is also checked for independence, separately from bounds: a 2×2 view with element strides (1, 1) maps index [0, 1] and index [1, 0] to one element, so stores of 7 and 11 leave a result that depends on their order. Shared elements are admitted for reads; a write needs an injective index map, a declared reduction or separate storage, and is refused otherwise. Neither library is wrong by itself; the trap is at our boundary.
```

**C32** §1.7: the later foundation facts.

```text
FILE: parts/P00.md
ORIGINAL:    airlock only through a typed conversion (§33.17).
REPLACEMENT: airlock only through a typed conversion (§33.17). Six further facts found later are counted separately and recorded with their admission tests in the last subsection of §33: two cases in which the bound verifier of `fs-sos` returns a value that is not a bound; welded masses omitted by the suite's G1 catalog builder; a series limit that refuses the differential of the SE(3) exponential beyond 32 m of translation; a swept-sphere helper that covers straight motion only; and two more convention hazards, the sign of the legacy transformer in `fs-phs` and a ray direction renormalized under a scaled instance in `f3d-math`.
```

**C33** §38.17, row 1: the regearing lever.

```text
FILE: parts2/P13a.md
ORIGINAL:    Regearing from 12 to 16 at equal output torque cuts copper loss by 1 − (12/16)² = 43.75%
REPLACEMENT: Regearing from 12 to 16 at equal output torque cuts copper loss by 1 − (12/16)² = 43.75% and does not remove the runaway: s falls to 0.770 W/K, still above G, and the rate is 0.002254 s⁻¹. A ratio of 22 gives s = 0.407 W/K, below G, and a rate of −0.000337 s⁻¹, yet the equilibrium rise is P_amb/(G − s) = 2,240 K: the winding is at 58.7 °C after 45 s and at the limit after 128.6 s, so the sign of the rate decides nothing without the duty time
```

**C34** §15.6: rotor elimination behind a coupling transmission.

```text
FILE: parts/P02.md
ORIGINAL:    Schur complement changes only diagonal entries of the link block, at a cost linear in the number of rotors. The
REPLACEMENT: Schur complement changes only diagonal entries of the link block, at a cost linear in the number of rotors (a rotor behind a coupling transmission of §15.9 is the exception: its complement is the block Cᵀ·J_r·C over the joints it drives, which has off-diagonal entries and is added once). The
```

---

## (c) Insertion blocks

Eighteen blocks. Gates are numbered 701–718 in one sequence across this file. Refusals are named by description. Two
new kill gates, KILL-FREEPERIOD and KILL-STRIDESEQ, were checked against the parts; one block adds a trigger to the
existing KILL-AUGUR-PROBE and says so.

### B1

<<<INSERT chapter=25 title="The controller class is part of a family answer">>>
**What it is.** [S] A typing rule for inclusion over a set of loads or disturbances. A cell is certified for a
declared class of control laws, and the certificate carries the law. One feasible command per vertex of a load set
shows that a feasible command exists for every load in the hull, because the constraints are convex. It does not show
that a controller of the declared class can produce those commands. The class is the set of functions from the
observed quantities to the command, and it is a field of `policy_class` beside the authority model of §25.13. Owner:
`frob-mission` for the lowering, `frob-cert` for the checking rule. Verbs: `mission.synthesize`, `mission.refute`.

**The stance family under three classes.** The mission of §25.13: 400 N on feet at (±a, ±b) with a = 1/3 m, ratings of
160 N, and load moments P = (P_x, P_y) with abs(P_x) ≤ 60 N·m and abs(P_y) ≤ 30 N·m. The feet are ordered [++, +−, −+,
−−], and foot i has signs (s_x, s_y).

| Class | Smallest half-width b | A law that attains it |
|---|---|---|
| `adjustable`: any function of the observed P | 3/14 m | f(P) = Σ_σ w_σ(P)·f_σ over the four vertex allocations f_σ of §25.13, with w_σ(P) = ¼·(1 + σ_x·P_x/60)·(1 + σ_y·P_y/30) |
| `affine`: f(P) = f₀ + A·P | 1/2 m | fᵢ(P) = 100 + s_x·P_x/(4a) + s_y·P_y/(4b) newtons |
| `constant`: P is not observed before the command | none | No fixed f satisfies Σ xᵢ·fᵢ = P_x for two values of P_x |

- *Adjustable.* The weights w_σ are nonnegative, sum to one and satisfy Σ w_σ·P_σ = P. The interpolant therefore meets
  the three balance equations at every P and stays inside the ratings by convexity. It is not affine: at b = 3/14 m
  its coefficient of P_x·P_y is (−20, 20, 20, −20)/1800 N per (N·m)². At P = 0 it gives 100 N on every foot.
- *The four witnesses are not values of one affine law.* An affine function on a rectangle satisfies f(60, 30) +
  f(−60, −30) = f(60, −30) + f(−60, 30). The vertex allocations give (160, 240, 240, 160) N on the left and (240, 160,
  160, 240) N on the right.
- *Affine.* If an affine law is feasible, so is its image under each reflection of the load rectangle with the
  matching exchange of feet, and so is the average of the four images, because the constraints are convex and the
  mission is symmetric. The average is invariant under the reflections, which forces the form in the table, and the
  balance equations fix its coefficients. Its largest value is 100 + 60/(4a) + 30/(4b) = 145 + 15/(2b) N, which is at
  most 160 N exactly when b ≥ 1/2 m; its smallest value, 55 − 15/(2b) N, is positive there. The affine class needs 7/3
  times the half-width of the adjustable class.
- *The passive suspension is an affine controller.* The symmetric affine law is the load sharing of four equal
  springs. At b = 1/4 m it gives (175, 115, 85, 25) N at the vertex (60, 30), which is the passive allocation of
  §25.13. The authority counterexample and the class counterexample are one fact seen twice.

**Mechanism.** DESIGN.

- `policy_class` gains the field `law`, one of `adjustable`, `affine`, `piecewise-affine` and `constant`. The
  arguments of the law are the fields listed as observed before the decision. A query that asks for inclusion over a
  load or disturbance set and omits `law` is refused (refusal: inclusion over a disturbance set without a declared law
  class).
- Lowering. For `adjustable`: one command per vertex of the load polytope, which is the lifted program of §25.13; the
  returned law is the multilinear interpolant on a box, and the barycentric interpolant on a declared triangulation
  otherwise. For `affine`: the unknowns are (f₀, A); equalities are imposed as identities in P by matching
  coefficients; inequalities are imposed at the vertices, where an affine function on a polytope takes its extremes.
  For `constant`: one command for every vertex.
- Certificate. The `lp-primal` kind carries a `law` object with its kind (`multilinear-vertex`, `barycentric`,
  `affine` or `constant`) and its data. The checker recomputes the balance equations and the ratings at every vertex.
  For `affine` it checks that the listed vertex commands equal f₀ + A·P exactly. For `multilinear-vertex` it checks
  that the load set is a box whose vertices are the listed ones. In every case it checks that each argument of the law
  arrives before the decision instant (§20.4).
- `mission.refute` under `affine`, for b in [3/14, 1/2) m, returns the dual of the affine program and names the class
  as a repair beside the geometric one: widen to 1/2 m, or admit a law that is not affine.

**Known answers** (exact rationals; every value is derived on file from the stated inputs).

| Case | Value | Mutation that must fail |
|---|---|---|
| `adjustable`, b = 3/14 m | Inner, with the bilinear law; 100 N per foot at P = 0 | The four vertex commands labelled `affine` |
| `affine`, b = 1/2 m | Inner; (160, 130, 70, 40) N at the vertex (60, 30) | — |
| `affine`, b = 1/4 m | Excluded; the symmetric law needs 175 N | A hull certificate with no law object |
| `constant`, any b | Excluded | A constant command accepted because each vertex has some feasible command |

| | |
|---|---|
| GATE-AUGUR-701 | Known answer. Through the public verb the stance family returns 3/14 m under `adjustable` with the bilinear law, 1/2 m under `affine` with the symmetric law and a matching dual, and an exclusion for every b under `constant`. The bilinear law evaluated on a 121 × 61 grid of loads meets every balance equation and rating in exact rationals. The four vertex witnesses relabelled `affine` are rejected by `frob-verify`, and so is a certificate with no law object. M1. |
<<<END>>>

### B2

<<<INSERT chapter=25 title="Docking to a finite tolerance over a continuous gain interval">>>
**What it is.** [S] The amended docking game that §25.15 leaves open, solved in closed form. It is a second known
answer of the docking family: a least guaranteed tolerance over a continuum of hidden gains, with an irrational
algebraic optimum, a rational witness above it and a rational exclusion below it. Owner: `frob-mission`, with
`frob-conformance` for the fixture. Verbs: `mission.synthesize` with the objective "least guaranteed tolerance", and
`mission.refute`.

**The game.** The plant, the three ticks and the packet timing are those of §25.15. The gain g is any constant in [1,
2]. The command bound is U = 3/4. The first velocity reading r = g·p + e arrives before the second command, with e in
[−ε, +ε] and ε = 1/64. The robot must be exactly at rest after the third tick, and the guarantee sought is abs(x₃ − 1)
≤ δ for every admitted g and e.

**The answer.** The least guaranteed tolerance is δ* = (√1745 − 41)/32 m, about 24.1624 mm, the positive root of 16·δ²
+ 41·δ − 1 = 0. It is attained with the first command p* = (39 + √1745)/128, about 0.631041. A 20 mm requirement is
infeasible. A 25 mm requirement is met with the rational probe p = 5/8.

**Derivation.** Write p = u₀ and c = 2p + u₁. Rest forces u₂ = −p − u₁, and then x₃ = g·c. For p > 0 the limits on u₁
and u₂ confine c to [2p − U, p + U]. A reading r admits the gains [a, b] with a = max(1, (r − ε)/p) and b = min(2, (r
+ ε)/p), and the tolerance requires (1 − δ)/a ≤ c ≤ (1 + δ)/b. A common c exists at every reading exactly when three
conditions hold.

1. *Ambiguity.* The ratio b/a is largest at the reading p + ε, where a = 1 and b = 1 + 2ε/p. The interval for c is
   nonempty there exactly when ε/(p + ε) ≤ δ, that is p ≥ ε·(1 − δ)/δ.
2. *Reach at the high gain.* Where the reading admits g = 2, c is at most (1 + δ)/2, so 2p − U ≤ (1 + δ)/2, that is p
   ≤ (2U + 1 + δ)/4.
3. *Reach at the low gain.* Where the reading admits g = 1, c is at least 1 − δ, so p + U ≥ 1 − δ.

A larger probe eases the first condition and tightens the second. Equating them gives 4ε·(1 − δ) = δ·(2U + 1 + δ),
which is 16·δ² + 41·δ − 1 = 0 at U = 3/4 and ε = 1/64, with p* = (2U + 1 + δ*)/4. The third condition is slack. A
probe p ≤ 0 gives c ≤ U = 3/4, which is short of 1 − δ in the world g = 1 for every δ < 1/4. The error interval and
the tolerance are closed, so the optimum is attained and the answer carries `attained: true`, unlike the strict family
of §25.3.6. DESIGN arithmetic: for other values with U ≥ (1 + δ)/2 the same argument gives the positive root of δ² +
(2U + 1 + 4ε)·δ − 4ε = 0.

**The witness policy.** After the reading, compute [a, b], choose any c in [max((1 − δ)/a, 2p − U), min((1 + δ)/b, p +
U)], and command u₁ = c − 2p and u₂ = −p − u₁.

**Certificates.** The optimum is irrational. It is returned as the root of its polynomial with a rational enclosure,
bracketed by two rational certificates.

- *Inclusion at δ = 1/40 with p = 5/8.* 1/41 ≤ 1/40; 2p − U = 1/2 ≤ 41/80; p + U = 11/8 ≥ 39/40. The reading interval
  [p − ε, 2p + ε] is partitioned at p + ε and 2p − ε, and the interval for c is checked at the ends of each piece,
  where its bounds are monotone in r. Kind: `partition` over `interval-box` pieces.
- *Exclusion at δ = 1/50.* The polynomial is negative there: 16/2500 + 41/50 − 1 = −0.1736. Every p in (0, 49/64)
  fails condition 1 through an alias pair: the worlds g = 1 with e = +ε and g = min(2, 1 + 2ε/p) with the error that
  gives the same reading share p + ε and need disjoint values of c. Every p above 63/100 fails condition 2. Since
  63/100 < 49/64, the two cases cover (0, 3/4], and p ≤ 0 fails as above. Kind: `alias-pair`, with a `partition` of
  the probe axis.

**What the fixture is for.** It checks that Augur returns a minimum that is not rational, that a closed tolerance is
reported as attained, and that a continuum of hidden parameters is settled by endpoint reasoning with a certificate
and not by sampling gains.

FACT (a recorded pilot, Appendix K; to be re-run before a gate cites it): a solver on the raw three-tick dynamics
excluded 20 mm, and independent interval checks confirmed inclusion at the threshold. Recomputed for this section in
exact rationals: the witness policy at δ = 1/40 and p = 5/8, run on the raw dynamics for 2,049 gains and 65 errors,
ends at rest inside the tolerance in all 133,185 cases; at δ = 1/50 no probe on a grid of 95 values succeeds on all
cases.

| | |
|---|---|
| GATE-AUGUR-702 | Known answer. Through the public verbs: δ = 1/50 is excluded with the alias pair and the partition of the probe axis; δ = 1/40 is included with p = 5/8; the minimum is returned as the root of 16·δ² + 41·δ − 1 with an enclosure no wider than 10⁻⁹ m and `attained: true`. The witness policy passes the 133,185 exact cases on the raw dynamics. A screen compiled for the gain set {1, 2} is refused on the interval [1, 2]. M1. |
<<<END>>>

### B3

<<<INSERT chapter=25 title="Channel laws: the joint error law of repeated reads, and the code that carries a label">>>
**What it is.** [S] Two declarations that a guarantee about information must carry, each with the smallest case in
which omitting it gives a wrong answer. A sensor budget states the joint law of its read errors, not their pairwise
tables. A label count states the channel that carries the label and the length of the message. Owner: `frob-sense` for
the error law, `frob-mission` for the label query, `frob-analyze` for analysis A-17. Verbs: `sense.alias`,
`mission.synthesize`.

**Pairwise independence is not joint independence.** A hidden binary class is read seven times, and each read is right
with probability 3/4.

| Declared law of the errors | Best success, equal priors |
|---|---|
| Jointly independent | 3807/4096, about 0.9294 |
| One fault bit shared by all reads | 3/4 |
| The pairwise-independent law below | 55/64, about 0.8594 |

In each class draw the number K of wrong reads as 1, 2, 3 or 4 with probabilities (43, 3, 9, 9)/64, and place them
uniformly among the seven slots. Each read is wrong with probability E[K]/7 = (112/64)/7 = 1/4. Each pair is wrong
together with probability E[K·(K − 1)]/42 = (168/64)/42 = 1/16, so every pair has the table of two independent reads.
A history with one or two errors relative to one class has six or five relative to the other, which has probability
zero there, and is decoded with certainty. A history with three errors relative to one class has four relative to the
other, and its likelihood is (9/64)/35 under both. The best decoder succeeds with probability 46/64 + ½·(18/64) =
55/64. A requirement of 0.9 is met by the independent law and missed by this one, with the same accuracy and the same
pair tables.

DESIGN. A sensor card states `error_dependence`: `joint-independent`, `shared-fault`, `declared-law` (a finite joint
law by reference), `pairwise` or `unknown`. Analysis A-17 and `sense.alias` credit redundancy, meaning a success
probability above the single-read accuracy, only under `joint-independent`, under a `declared-law` whose best decoder
is found by enumeration (128 histories here), or under a proved lower bound on the histories that decide. Under
`pairwise` or `unknown` the guaranteed value is the single-read accuracy, and a request for more is refused (refusal:
redundancy credited without a joint error law).

**A label is a message.** The label query of §25.18 counts labels. The factory must also deliver the label by the
commitment deadline over a real channel. DESIGN: the query gains `channel`, one of `clean`, `erasures(e)` (up to e
known positions lost) and `flips(t)` (up to t unknown bit errors), for a one-shot binary message of fixed length with
no feedback, and a bit time. For L labels the least length n is the least n that admits a binary code of L words with
minimum distance 1, e + 1 or 2t + 1. The answer returns the label count, the length, a code, and the check n × bit
time ≤ time to commitment. For L ≤ 16 and n ≤ 12 (DESIGN caps) the least length is found by exhaustive search: the
code is the upper-bound witness, and the exhausted search, or a counting argument where one exists, is the lower
bound.

Known answers for the three labels of the one-label-map fixture of §25.18.

| Channel | Least length | Code | Why no shorter |
|---|---|---|---|
| `clean` | 2 bits | 00, 01, 10 | One bit has two words |
| `erasures(1)` | 3 bits | 000, 011, 101 (pairwise distance 2) | Any three words of length 2 include a pair at distance 1 |
| `flips(1)` | 5 bits | 00000, 00111, 11001 (distances 3, 3, 4) | Distance at least 3 for each pair needs the three distances to sum to 9; each bit position adds at most 2 to that sum, since three binary symbols cannot differ pairwise; four bits give at most 8 |

A transmission deadline of four bit times therefore admits guaranteed operation under one erasure and excludes it
under one flip. The checker enumerates the received-word sets under the declared channel and verifies that they are
disjoint.

| | |
|---|---|
| GATE-AUGUR-703 | Known answer. Enumeration of the 128 histories returns 55/64 for the pairwise law, 3807/4096 for the independent law and 3/4 for the shared fault, exactly; a redundancy claim on a card marked `pairwise` is refused. The three label rows return 2, 3 and 5 bits with codes whose received-word sets are disjoint; the four-bit request under `flips(1)` returns an exclusion with the counting argument. On 20 generated instances with L ≤ 8 and e, t ≤ 2 the least length equals brute force. The error-law rows at M3, the label rows at M4. |
<<<END>>>

### B4

<<<INSERT chapter=25 title="Shared nuisance parameters, and the probe for four admitted laws">>>
**What it is.** [F] Two additions to §25.14. The first is a rule: nuisance parameters that two quantities share in the
world stay shared in every bound that compares them. The second is a known answer: once the delayed law D is admitted
to the mechanism set, a probe exists that separates all four laws, at a longer dwell than the three-law probe. Owner:
`frob-mission`. Verbs: `mission.distinguish`, `experiment.suggest`.

**Shared parameters.** In the fixture both pads of one trial have the same residual fraction q and the same time
constant τ. The margin of the compact pad over the broad pad at the handover is m(q, τ) = 60·R(1, T; q, τ) − 50·R(2,
T; q, τ) newtons. Minimizing the two terms separately lets the compact pad take (q, τ) = (0.18, 0.9 s) while the broad
pad takes (0.22, 1.1 s). No such trial exists.

| Law | Least m with q and τ shared | Least m with independent extremes |
|---|---|---|
| V and C | 4.50 N, at (0.18, 0.9 s) | 0.28 N |
| D | 4.72 N, at (0.18, 1.1 s) | −0.078 N |
| P | −13.06 N; the broad pad wins everywhere, by at least 10.02 N | −15.08 N |

The values are derived on file on a 201 × 201 grid of the box. FACT (a recorded pilot, Appendix K; to be re-run): an
enclosure of the D row reported at least 4.07 N shared and −0.0776 N independent. Under D the independent bound
changes sign. It reports that the broad pad might win where the compact pad wins by more than 4.7 N over the whole
box.

DESIGN. Every nuisance parameter of a mission or a measurement contract has a `scope`: the set of quantities that read
one realization (a trial, a pad pair, a batch of material, a unit). Interval and affine evaluation treats a scoped
parameter as one variable across every expression in its scope, by a shared noise symbol or by subdividing the shared
box, never as an independent interval per expression. A parameter with no declared scope is evaluated independently
and the result is marked `scope: undeclared`. That direction is safe for inclusion, since it only widens the
unresolved set. It is not safe for refutation: an extreme point of an independent box is not a world, and it cannot
serve as a counterexample witness (refusal: witness assembled from unshared values of a shared parameter).

**The four-law probe.** With D admitted the decisions are compact for V, C and D and broad for P. On the candidate
grid of §25.14 no probe at a dwell of 1/4 s or less separates P from D, because D follows P until t/τ = 1/4. At 1/2 s
three radii separate them after both read errors of ±0.02 are included.

| Probe | Least reading under P | Greatest reading under V, C, D | Gap after both errors | Cost |
|---|---|---|---|---|
| r = 3/2, t = 1/2 s | 0.8206 | 0.7677 | 0.0129 | 0.75 s |
| r = 2, t = 1/2 s | 0.8937 | 0.7874 | 0.0662 | 0.75 s |
| r = 3, t = 1/2 s | 0.9509 | 0.8019 | 0.1090 | 0.75 s |

Each of the three decides all 900 enumerated cases correctly (four laws, 225 cases each). Against the direct
measurement at mission dwell, 1.25 s with the archived compact reading, the modeled time ratio is 1.25/0.75 = 5/3.
DESIGN: among probes of equal cost `mission.distinguish` chooses the largest gap, here r = 3. A tie-break by smallest
radius returns a gap eight times smaller at the same cost.

**What this does not change.** The 225 reversals of the three-law probe on the withheld law stand, and so does the
normative lesson of §25.14. This is a result on a known mechanism set. A fifth law outside it can reverse it again,
and the direct measurement at mission conditions stays in the candidate set.

**Ceiling, stated before any speed test.** 5/3 in modeled exposure time on this fixture, and 1 against a programmed
expert who is given the same four laws.

| | |
|---|---|
| GATE-AUGUR-704 | Known answer. The margin table reproduces to 10⁻³ N, and a refutation built from the independent extremes under law D is refused. With four laws admitted `mission.distinguish` returns a probe at t = 1/2 s with r = 3 and a gap of at least 0.10, decides 900 of 900 cases, and returns no probe at any dwell of 1/4 s or less. M4. |
| ✖ KILL-AUGUR-PROBE | The existing kill gate of §25.27 gains one trigger: a certificate or a counterexample that depends on unshared values of a parameter declared shared passes the checker. Its fallback is unchanged. |
<<<END>>>

### B5

<<<INSERT chapter=25 title="Sizing witnesses for fixed points, and Pareto coverage of a catalog">>>
**What it is.** [S] Two acceptance rules for the co-design lane of §25.21.1. A sizing answer on a continuous resource
is accepted on a witness, never on a small increment. A complete Pareto claim over discrete builds is accepted on a
disposition for every omitted option, never on a sweep of weights. Owner: `frob-synth`, with `frob-algebra` for the
order checks. Verb: `mission.synthesize`.

**A small increment is not a fixed point.** A pack of mass m needs a pack of mass F(m) = 1 + 0.99·m kilograms to carry
itself and the payload. Kleene iteration from zero gives x_k = 100·(1 − 0.99ᵏ). After 527 iterations the iterate is
about 99.5 kg and the next step adds 5 g. The least feasible mass is 100 kg. A stop on "increment below 10 g" returns
a pack half a kilogram too light, and F(99.5) = 99.505 > 99.5 shows that it does not carry itself.

DESIGN. On a finite catalog chain the criterion of §25.21.1 stands: the iteration stops at exact equality. On a
continuous resource the lane returns a bracket.

- *Upper witness.* A value U with F(U) ≤ U in the product order. Because F is monotone, every Kleene iterate from the
  bottom stays below U, so the least fixed point is at most U and U is feasible. The check is one evaluation of F.
- *Lower bound.* Any Kleene iterate from the bottom is at most the least fixed point.
- *Width.* The answer states U − x_k. Where a contraction constant q < 1 has been proved in a stated norm, the
  distance from x_{k+1} to the fixed point is at most q·‖x_{k+1} − x_k‖/(1 − q), which proposes U; the proposal is
  still checked by F(U) ≤ U.

In the fixture q = 0.99: from 99.5 kg the bound is 0.99 × 0.005/0.01 = 0.495 kg above 99.505 kg, which gives 100 kg,
and F(100) = 100. A sizing answer without an upper witness is labeled `lower-bound-only` and cannot be returned as a
design (refusal: sizing returned without a feasibility witness).

**A weighted sum does not find every nondominated build.** Three feasible builds have (body cost, facility cost) equal
to (0, 6), (4, 4) and (6, 0). None dominates another. For weights (w₁, w₂) ≥ 0, not both zero, the build (4, 4) is
best only if 4w₁ ≤ 2w₂ and 4w₂ ≤ 2w₁, which together force both weights to zero: the two extremes average to (3, 3),
which is not a build and lies below (4, 4). A facility budget of four makes (4, 4) the unique build of least body
cost.

DESIGN. An answer with the claim `pareto` over a discrete catalog lists, for every option it omits, one disposition:
`dominated-by` with the dominating option, `infeasible` with its exclusion, or `unresolved`. If any option is
unresolved the claim is downgraded to `pareto-candidates`, and the answer carries the remaining coverage obligation.
Candidates are generated by dominance filtering on a finite catalog and by constrained queries (least body cost
subject to a bound on each other objective at its attained values) otherwise; weighted sums may propose points and
never close the set.

| | |
|---|---|
| GATE-AUGUR-705 | Known answer. On the scalar pack the lane returns a bracket whose upper end passes F(U) ≤ U and lies within 1 g of 100 kg; the increment-stopping mutant returns 99.5 kg and is refused. On the three-build catalog all three builds are returned with dispositions; a weights-only mutant omits (4, 4) and fails; the budget query returns (4, 4). On 20 generated catalogs of at most 200 options the returned set equals brute-force dominance filtering. M4. |
<<<END>>>

### B6

<<<INSERT chapter=14 title="Folding fixed joints, and reduced models">>>
**What it is.** [S] A rule for lumping a link behind a fixed joint into its parent, in import (§14.12) and in the
lowering of §14.9, and a rule for a model that leaves parts out. A folded model is the same rigid body. A model that
omits mass is a different robot and is named as one. Owner: `frob-io` (import), `frob-model` (lowering). Verbs:
`design.import`, `design.check` stage 2.

**The fold.** For a child with pseudo-inertia 𝖯_c (§11.5) attached by the fixed transform G = parent_from_child, the
parent's pseudo-inertia becomes 𝖯_p + G·𝖯_c·Gᵀ (§11.4). That one sum carries the mass, the first moment and the second
moment about the parent's origin, and it is exact in rationals. Every named attachment of the child (sensor frames,
collision shapes, contact sites, port frames) is re-parented with the composed transform and keeps its identifier. The
loss report lists each folded link.

Hand case. A parent of 2 kg with rotational inertia 0.02·I kg·m² about its origin carries a welded child of 1 kg with
inertia 0.01·I about its own centre, placed at (0.3, 0, 0) m. The fold has mass 3 kg, centre of mass at (0.1, 0, 0) m,
inertia diag(0.03, 0.12, 0.12) kg·m² about the parent origin and diag(0.03, 0.09, 0.09) about the combined centre.
Adding the two inertias without the offset term gives 0.03·I about the origin, wrong by 0.09 kg·m² on two axes.
Dropping the child gives 2 kg.

**Reduced models.** DESIGN. A design that leaves out bodies of the source carries `reduced: { omitted: [...],
omitted_mass }`, and every result on it cites the reduction. An import that drops a body of nonzero mass without
declaring a reduction is refused (refusal: mass dropped at import without a declared reduction). Import parity
(§14.10) is judged against the full source: a round trip compares total mass, first moment and pseudo-inertia of every
folded group, and the map of named attachments.

FACT (`frankensim/crates/fs-mbd/src/robot_models.rs:398–403` at `483d09c682f2`, read 2026-11-21): the suite's G1
catalog builder states that fixed rubber hands, head, pelvis contour, logo, waist supports and sensor attachments are
omitted and that "their nonzero masses are not lumped into retained links". Its 30 retained links sum to 31.96314202
kg (summed for this section from the source table). FACT (the pinned upstream description of the same robot; reported,
to be re-read from the primary source before a gate cites it): four welded links, the head, the logo and the two
hands, add 1.377 kg, for 33.34014202 kg over 38 links.

Known answer. At 9.81 m/s² the reduced model weighs 313.5584 N and the source assembly 327.0668 N. A static support
rated at 320 N passes on the reduction and fails on the robot. The builder is therefore used as the profile
`g1-reduced-30`, and the `humanoid` archetype's import parity is against the 38-link source.

| | |
|---|---|
| GATE-SKEL-706 | Known answer. On 10³ random trees with fixed joints the folded model's mass, first moment and pseudo-inertia equal the sums over the unfolded links exactly in rationals and to 10⁻¹² relative in double precision; forward dynamics of the folded model and of the unfolded model with the joints held as constraints on the referee agree to 10⁻⁹; every named attachment resolves to the same world pose at 100 random configurations. The hand case is exact, and the mutant without the offset term fails it. The `g1-reduced-30` profile reports its omissions, sums to 31.96314202 kg, and the 320 N fixture passes on it and fails on the full import. M1. |
<<<END>>>

### B7

<<<INSERT chapter=15 title="Coupled transmissions: the torque set, the rotor energy and the direction witness">>>
**What it is.** [S] Three rules that extend the coupling transmissions of §15.9 to couplings that are not square, and
one rule for the sign of any transformer. Owner: `frob-actuate`; consumers `frob-dyn`, `frob-control`, `frob-analyze`,
`frob-mission`. Verbs: `actuator.check`, `actuator.envelope`.

**One map, written once.** A coupling is a matrix C(q) with motor rates θ̇ = C·q̇. Virtual work gives joint efforts
τ_joint = Cᵀ·τ_motor. Where the square matrix T of §15.9 is invertible, C = T⁻¹. C need not be square: one motor may
drive several joints through a differential or a tendon.

1. *The feasible joint-torque set is an image.* With motor limits abs(τ_motor,k) ≤ τ_max,k it is the zonotope Cᵀ·B,
   where B is the box of motor torques. It is not the box of per-joint limits. `actuator.envelope` returns its
   generators, the cone solver of §22.5 carries τ_motor as variables with τ_joint = Cᵀ·τ_motor, and the static
   analyses and mission queries (A-08, A-24, §25) read the same generators.
2. *Rotor inertia is pulled back once.* The rotors add Cᵀ·diag(J_r)·C to the joint-space mass matrix, with its
   off-diagonal entries. It is added in one place, the lowering, and never again in a controller's model.
3. *A coupling that varies with configuration has an inertial bias.* The rotor kinetic energy ½·q̇ᵀ·Cᵀ·J_r·C·q̇ then
   contributes velocity-product terms, which are derived with the mass matrix from the same expression. A linkage
   transmission is a closure in the design term (§14.3) and receives them by construction; a constant-ratio shortcut
   for it omits them.

Known answer. One base-mounted rotor with θ = 10·(q₁ + q₂), torque limit 0.1 N·m and inertia 0.01 kg·m².

| Quantity | Coupled map | Shortcut | Shortcut's error |
|---|---|---|---|
| Deliverable joint torques | (t, t) with abs(t) ≤ 1 N·m | Per-joint limits of 1 N·m | Admits (1, −1) N·m, which no motor torque produces |
| Reflected inertia | [[1, 1], [1, 1]] kg·m² | diag(1, 1) kg·m² | — |
| Rotor energy at joint rates (1, −1) rad/s | 0 J | 1 J | The rotor does not turn |
| Rotor energy at joint rates (1, 1) rad/s | 2 J | 1 J | Half the energy is missing |

**The direction witness.** A lossless transformer conserves power under either sign of its modulus, so a power balance
cannot detect a reversed gear. FACT (`frankensim/crates/fs-phs/src/interconnection.rs:91` at `483d09c682f2`, read
2026-11-21): the suite's legacy `transformer` carries the comment "The legacy transformer has the opposite sign to the
canonical map", the canonical map being `interconnect_mapped`. FACT (reported, not re-run): with two unit scalar
stores, efforts (2, 3) and coefficient 4, one gives rates (12, −8) and the other (−12, 8); both satisfy 2·12 + 3·(−8)
= 0, and a coefficient of −4 in one convention reproduces the other.

DESIGN. Every transformer, gear and coupling joins two labelled ports with an authored direction: positive rotation of
the driving port corresponds to a stated sign of rotation at the driven port. `actuator.check` runs a signed motion
witness when a card is bound to a joint: a unit positive effort on the driving port, with the driven port free and at
rest, must produce a rate of the authored sign at the driven port. A reversed witness is refused under the port-sign
refusal of §14.6. `frob-fsim` reaches only `interconnect_mapped` and passes the authored sign explicitly.

| | |
|---|---|
| GATE-SINEW-707 | Known answer. The one-rotor table is exact: the envelope has one generator (1, 1) N·m, the command (1, −1) N·m is refused as outside the image, the reflected inertia is [[1, 1], [1, 1]] and the two energies are 0 J and 2 J; the diagonal mutant reports 1 J twice and fails. For 10³ random couplings with up to four motors and six joints the power τ_jointᵀ·q̇ equals τ_motorᵀ·θ̇ to 10⁻¹², and the mass matrix with pulled-back rotors equals the mass matrix of the model with explicit rotor coordinates to 10⁻¹⁰. The signed motion witness passes on every card of the catalog and fails on each card with its ratio negated. M1. |
<<<END>>>

### B8

<<<INSERT chapter=16 title="A held command and the step that integrates it">>>
**What it is.** [S] A known answer, and three rules about the step that integrates a command held between control
ticks. A loop that is stable for the exact held-input plant can be unstable under the default integrator at one step
per tick. A loop that is asymptotically stable under a finer step can still miss a deadline. The controller, its rate,
the observation and the actuation are the same in every case; only the numerical step differs. Owner: `frob-dyn`
(integrators, §16.7), `frob-analyze` (analysis A-22), `frob-conformance` (the fixture). Verbs: `sim.run`,
`eval.compare`, and the verb of A-22.

**The fixture.** An ideal slider with commanded acceleration and a control period h = 10 ms. In the variables x
(position error, mm), w = h·v (mm) and u = h²·a (mm), the controller reads x and w at each tick and holds u =
−(3/2)·(x + w) until the next. The test takes the 64 initial pairs (x, w) in {−3, …, 4}² mm and passes a case when
max(abs(x), abs(w)) ≤ 1/32 mm at tick 16.

| Integration between ticks | One-tick map | Spectral radius | Cases passed |
|---|---|---|---|
| Exact for the held input | x′ = x + w + u/2, w′ = w + u | 1/2 | 64 of 64 |
| One semi-implicit step: velocity, then position | x′ = x + w + u, w′ = w + u | (1 + √3)/2, about 1.366 | 1 of 64, the origin |
| Two such steps of h/2 | x′ = x + w + 3u/4, w′ = w + u | (5 + √57)/16, about 0.784 | 15 of 64 |
| d such steps of h/d, for d = 3, 4, 8, 16, 32 | x′ = x + w + (d + 1)·u/(2d), w′ = w + u | 1/2, 1/4, 0.395, 0.451, 0.476 | 64 of 64 |

The second row is the default integrator of the lockstep tiers (§16.7) at a decimation of one. The third row is
asymptotically stable and fails 49 of 64 cases, because its contraction of 0.784 per tick is too slow for the
deadline.

**Rules.** DESIGN.

1. Analysis A-22 evaluates each sampled loop twice: with the hold modelled exactly, as it does now, and with the
   tier's own integrator at the task's decimation. It reports both spectral radii and does not label a loop stable on
   a tier where the second exceeds one.
2. A step-refinement study holds every clock of the clock contract fixed in seconds (control period, sensor periods,
   latencies) and changes only h. Clocks are physical parameters in the sense of ADR-KIN-05. A study that halves the
   control period together with the step compares two controllers and is refused as a refinement (refusal: refinement
   study that changes a clock).
3. A requirement with a deadline is compared across decimations d and 2d by its verdicts, case by case, and not by a
   stability label. A verdict that differs is `fidelity-dependent` (§24.10) and is not promoted from the coarser run.

FACT (a recorded pilot, Appendix K; to be re-run before a gate cites it): a rational referee and a native
single-precision kernel on the Metal device and on each RTX 4090 agreed on all 192 decisions of the first three rows,
and the device read-backs matched independently reconstructed single-precision terminal states byte for byte. The
counts and radii in the table were recomputed in exact rational arithmetic for this section.

**What it does not show.** An ideal slider has no actuator limit, no contact and no sensor path. A robot's loop needs
its own check. The fixture tests the sampled plant and its numerical lowering.

| | |
|---|---|
| GATE-KIN-708 | Known answer. The four rows reproduce exactly in rational arithmetic on the host. In single precision on `batch-cpu`, CUDA and Metal the pass or fail decision of each of the 64 cases equals the rational one for the exact map and for d = 1, 2 and 4. A-22 reports a tier radius of 1.366 at d = 1 and withholds the stability label. The mutant study that halves the control period with the step is refused. M2. |
<<<END>>>

### B9

<<<INSERT chapter=18 title="Swept bounds enclose arcs">>>
**What it is.** [S] The construction of the swept bounds that §18.4 and step T4.2 of §16.5 name. A bound swept over a
sub-interval encloses the path of the geometry, and a link that rotates does not move along the chord between its end
positions. A pair discarded in the broad phase never reaches the narrow phase, so an undersized bound is a missed
contact with no later chance of detection. Owner: `frob-contact`. Verbs: `contact.audit`, `contact.clearance`.

**Counterexample.** A 1 m arm turns from −60° to +60° about a fixed pivot at the origin. Its tip is a sphere of radius
5 cm, and an obstacle sits at (1, 0) m. The tip's end positions are (0.5, ∓0.866) m, their chord lies on x = 0.5 m,
and the chord padded by the radius reaches x = 0.55 m. The tip passes through (1, 0).

**Construction.** DESIGN. For a sample of radius r with end positions A and B over the sub-interval:

- *One revolute joint.* For a rotation by an angle Δθ of magnitude at most π at orbital radius R, the bound is the
  segment AB dilated by the sagitta R·(1 − cos(Δθ/2)) plus r. In the counterexample the sagitta is 0.5 m and the bound
  reaches x = 1.05 m.
- *Any admitted motion.* Let L be an upper bound on the length of the sample's path: the sum over the moving ancestors
  of the joint-speed limit times the largest lever arm of the sample about that joint, plus the base speed bound,
  times the duration. Every point X of the path satisfies ‖X − A‖ + ‖X − B‖ ≤ L, so the path lies in the segment AB
  dilated by ½·√(L² − ‖B − A‖²), plus r. For the counterexample L = 2π/3 m and the dilation is 0.589 m, which contains
  the sagitta bound.
- *Where a speed limit is missing.* The bound uses the speed at the start of the sub-interval plus the largest change
  that the actuator and load limits allow over it. A model that supplies neither is refused at admission (refusal:
  swept bound requested without a speed bound).

FACT (`frankensim/crates/fs-contact/src/broad_phase.rs:146–156` at `483d09c682f2`, read 2026-11-21): the suite's
`swept_sphere_aabb` documents that it "encloses straight centre motion only" and leaves curved trajectories to the
caller. It is used for prismatic and ballistic segments and not for a sample on a rotating link.

| | |
|---|---|
| GATE-TRAC-709 | Known answer. The arc fixture is detected on 1,000 of 1,000 seeded variants (radius 0.2 to 2 m, angle up to π, sphere radius 1 to 10 cm, an obstacle on the arc and outside the padded chord); the chord-only mutant fails the gate. On 10⁴ random two- and three-joint sweeps the bound of the second item contains 1,000 sampled path points per sweep with no exception. In the tunnelling audit (`contact.audit`) no missed contact is attributed to a broad-phase rejection. M1 on the referee, M2 on the transactional batch profile. |
<<<END>>>

### B10

<<<INSERT chapter=19 title="Scaled instances: distances, normals and ray parameters in world metres">>>
**What it is.** [S] The conversion rules for any query that reaches geometry through an instance transform with a
scale. The compiler of §19.4 bakes scales before it computes a field, so a compiled pack is in world metres. The rules
bind wherever a scaled transform survives to query time: an imported pack inspected before it is compiled, a distance
brick shared by instances of different size, the viewer's picking rays, and any adapter over a sibling's ray
primitive. Owner: `frob-scene` (compiler and bricks), `frob-sense` (ray kernels), the viewer. Verbs: `scene.compile`,
`scene.accept`.

**Rules.** DESIGN. Let A be the linear part of the instance transform, local to world.

1. *Distance.* A local signed distance d outside the shape bounds the world distance to [σ_min(A)·d, σ_max(A)·d]. It
   equals the world distance only for a uniform scale, where both ends coincide. An unresolved query is refined, or
   the field is baked in world metres; a local distance is never returned in metres.
2. *Normals.* A surface normal transforms as A⁻ᵀ·n and is then renormalized, before any contact frame or optical
   response is built from it.
3. *Rays.* For a unit world direction d the local direction is A⁻¹·d. If the primitive renormalizes it, the hit
   parameter is in local units and s_W = s_L/‖A⁻¹·d‖. The conversion happens before hits are sorted, clipped to a
   range gate, or used as optical depth.

**Counterexamples.**

- A unit sphere scaled by (2, 0.5, 1). At the world point (0, 0.6, 0) the local distance is 0.20 and the clearance is
  0.10 m: read as metres, the local value admits a 0.15 m probe that intersects. At (2.2, 0, 0) the local distance is
  0.10 and the clearance is 0.20 m. Both lie inside the bounds [0.5·d, 2·d].
- Two planes facing one ray. An opaque foreground plane at 2 m belongs to an instance of scale 1/2 and reports a local
  parameter of 4. A background plane at 3 m belongs to an instance of scale 3 and reports 1. Sorting by the local
  parameter returns the occluded plane, and a range gate of 2.5 m applied to the local values discards the foreground
  and keeps the background.

FACT (`franken_threed/crates/f3d-math/src/ray.rs:487–490` and `vector3.rs:708–719` at `bf7abe16367e`, read
2026-11-21): `Ray::apply_matrix4` transforms its direction with `Vector3::transform_direction`, which normalizes the
result, as the scene-graph method it mirrors does. An adapter over it converts by rule 3.

| | |
|---|---|
| GATE-HAB-710 | Known answer. The two sphere points return clearances of 0.10 m and 0.20 m after compilation, and the pre-compilation query returns the interval of rule 1 and never the local value. The two-plane fixture returns the foreground at 2 m through the compiled pack, through the viewer's pick and through the adapter; the mutant that sorts local parameters returns the background and fails. On 10³ random instances with anisotropic scales in [0.25, 4], depth against an analytic plane is within 1 mm plus the brick bound. M2. |
<<<END>>>

### B11

<<<INSERT chapter=20 title="Sampled readings: impacts in an exposure, encoder unwrapping, the load-cell cut">>>
**What it is.** [S] Three additions to the proprioceptive models of §20.5. Each fixes what a sampled reading means in
a case where the instantaneous formula gives a wrong or ambiguous number. Owner: `frob-sense`. Verbs: `sense.profile`,
`sense.alias`.

**An integrating accelerometer and an impact.** The reading over an exposure window of length T is the mean specific
force, and on a step that resolves contact as an impulse the velocity of the mounted point jumps. DESIGN: the window's
reading is

    f̄ = (1/T)·[ ∫ R_WSᵀ·(a_P − g) dt + Σ R_WSᵀ·(v_P⁺ − v_P⁻) ],

with the sum over the impacts inside the window. With continuous pose, a rigid offset r from the body point O
contributes (ω⁺ − ω⁻) × r to the jump of the point's velocity. Each impact belongs to exactly one window by the
half-open convention of §20.4.3. On the compliant tiers the force history is resolved and the integral already
contains it; the jump term applies to impulse-level steps (the referee's event step and the hard-contact batch law).
Bandwidth and clipping follow the ordered chain of §20.23, with the same three grades of force history.

Known answer: a body in ballistic flight, not rotating, rebounds from −1 to +1 m/s inside a 10 ms window. The window
reads +200 m/s² upward. Every smooth specific-force sample in the window is zero. The mutant without the jump term
reads zero through a bounce.

**Unwrapping a sampled encoder.** A modulo-one-turn position sampled at 100 Hz delivers identical samples at +80 and
at −20 turns per second, with no noise at all: 0.8 turn and −0.2 turn per sample are the same position. A controller
cannot recover the direction from those samples. DESIGN: nearest-increment unwrapping is valid when

    Ω·Δt + 2·ε < 1/2   (in turns),

with Ω a bound on the absolute speed, Δt the actual gap between delivered samples, and ε the error of one reading. At
equality with ε = 0 the two half-turns coincide. `sense.profile` reports each encoder's largest unambiguous speed,
(1/2 − 2·ε)/Δt, against the joint's speed limit seen through the transmission, and `sense.alias` returns the two-speed
pair as a witness when the limit exceeds it. After a lost packet Δt is the gap that occurred, and a velocity estimate
whose gap violates the condition carries the flag `ambiguous` until a hardware turn counter or an independent velocity
channel resolves it. The estimator never picks the nearer branch in silence. This is the sampling counterpart of the
static turn count of §13.11.

**The load-cell cut.** A six-axis force-torque sensor measures the wrench across a cut in the load path. Its card
states the cut, the side (distal on proximal, or the reverse), the sign convention, the moment origin, the frame and
the tare state. DESIGN: the reading is computed from the momentum balance of everything distal to the cut: gravity on
the distal bodies, every external load on them, and their rate of change of momentum, with moments about the stated
origin. A sum of the sole's contact forces is a different quantity.

Known answer: a 2 kg foot below the cut, upward positive, g = 9.81 m/s², untared, reporting distal on proximal.
Stationary under a 100 N ground reaction it reads +80.38 N. In swing with no contact and an upward acceleration of 2
m/s² it reads −23.62 N. The sole's contact sum reads 100 N and 0 N in the two cases.

| | |
|---|---|
| GATE-SENS-711 | Known answer. The rebound window reads 200 m/s² on the referee to 10⁻⁹ relative, and the adjacent windows read zero; the mutant without the jump term and the closed-closed window mutant both fail. The encoder fixture: +80 and −20 turns per second deliver bit-equal samples, `sense.alias` returns the pair, and the largest unambiguous speed is reported as 50 turns per second at ε = 0; after a planted packet loss the estimate is flagged `ambiguous` in 100 of 100 cases. The load-cell rows read +80.38 N and −23.62 N to 10⁻⁹, and each of the four card mutants (side, sign, origin, tare) changes the reading and is caught. M2. |
<<<END>>>

### B12

<<<INSERT chapter=22 title="Orbits of a loop with no clock: the free period and the trivial multiplier">>>
**What it is.** [F] The treatment of gaits whose closed loop is autonomous. §22.9.1 strobes a clock-forced loop over
one stride of a known period. A passive walker, a controller whose phase is reset by touchdown, and any loop that
reads no clock have no such period: the stride time is an unknown, and the equations of §22.9.5 and §22.9.6 are
singular on every such orbit. Owner: `frob-gait`. Verbs: `gait.orbit`, `gait.continue`, `gait.gradient`.

**Why the clock-forced equations fail.** On an orbit of an autonomous loop with period T*, a state displaced along the
flow returns displaced by the same amount. The vector field f(x*) is therefore an eigenvector of the time-T* monodromy
M with multiplier exactly 1, for every design and every controller. I − M is singular, the fold test of §22.9.5 fires
on every orbit, and the solve of §22.9.6 has no solution or infinitely many. The multiplier carries no information
about stability.

**Mechanism.** DESIGN.

- *Classification.* A `ControllerId` declares its phase source: `clock`, `event-reset` or `none`. A loop is
  `clock-forced` only under `clock`, and `autonomous` otherwise. `gait.orbit` refuses a stroboscopic solve on an
  autonomous loop (refusal: fixed-period shooting on a loop with a free period).
- *Section form.* Where the stride has a repeatable transverse event with guard g(x) = 0, the orbit is a fixed point
  of the return map P_Σ on the section Σ = {g = 0} (C-16), of dimension one less than the state. Its Jacobian is the
  saltation product projected along the flow, D P_Σ = (I − f⁺·∇gᵀ/(∇gᵀ·f⁺))·M restricted to the tangent space of Σ.
  Its multipliers are those of M without the trivial one. Stability, folds and the gradient equation (I − D P_Σ)ᵀ·λ =
  ∇_Σ J are stated on Σ.
- *Bordered form.* Where no event serves as a section, the unknowns are the state and the period, with a phase
  condition s(x) = 0, for example ⟨x − x_ref, f(x_ref)⟩ = 0. Newton solves

      [ M − I     f(x_T) ] [ δx ]       [ φ_T(x) − x ]
      [ ∇s(x)ᵀ      0    ] [ δT ]  = −  [ s(x)       ]

  The bordered matrix is nonsingular exactly when 1 is a simple multiplier and the phase condition is transverse to
  the flow. A fold is a singular bordered matrix. The same matrix, with the parameter derivatives on the right, gives
  dx*/dθ and the derivative of the stride time dT*/dθ; its transpose gives the adjoint for a metric.
- *What stays.* The symmetry reduction of §22.9.2, the rule that stability is computed over the full reduced state,
  the derivative labels of §22.9.3 and the typed outcomes of §22.9.5 apply unchanged. The removed coordinates listed
  in a `GaitOrbit` record gain the phase.

**Known answers.**

- *A planar loop, by hand.* ṙ = r·(μ − r), φ̇ = 1 in polar coordinates. The orbit is r = μ with period 2π. The time-2π
  monodromy has multipliers 1 and e^(−2πμ), so det(I − M) = 0. On the section φ = 0 the return map has the single
  multiplier e^(−2πμ), and the implicit-function solve gives dr*/dμ = 1. The bordered solve gives the same, with
  dT*/dμ = 0.
- *The rimless wheel of §22.16.* The state (θ, θ̇) has monodromy multipliers 1 and cos²(2α); on the post-impact
  section the multiplier is cos²(2α), which is 1/2 for eight spokes.

**Pilot, ceiling, kill.** Pilot: the planar loop, the rimless wheel and the compass gait through both forms.
Falsifier: the two forms disagree on a nontrivial multiplier by more than 10⁻⁶. Ceiling: none on speed; the benefit is
that orbit analysis applies to event-driven controllers at all, and that the stride time becomes a differentiable
metric.

| | |
|---|---|
| GATE-GAIT-712 | Known answer. On the planar loop the section and bordered forms agree with the closed forms to 10⁻⁹ on the multiplier, on dr*/dμ and on dT*/dμ; a stroboscopic solve of the same loop is refused, and is not reported as a fold. The rimless wheel and the compass gait of GATE-GAIT-01 pass through the autonomous path with the trivial multiplier removed and listed. M4. |
| ✖ KILL-FREEPERIOD | Trigger: on the gait battery the section and bordered forms disagree beyond tolerance on more than 5% of autonomous orbits after one repair cycle, or the bordered iteration fails to converge on more than a quarter. Fallback: autonomous loops are analysed on an event section only; a loop with no transverse event is reported `unsupported` for orbit analysis and goes to finite-disturbance recovery tests, class `Tested`. |
<<<END>>>

### B13

<<<INSERT chapter=22 title="Stability over admitted sequences of strides">>>
**What it is.** [F] The stability object for a gait that can alternate between stride modes: a step up and a step
down, a left and a right asymmetric stride, a slipping and a gripping stride. Each mode has its own return-map
Jacobian. A multiplier test on each mode alone does not decide the alternation. Owner: `frob-gait`, with the shared
cone solver of §22.5. Verb: `gait.orbit` with a mode set.

**Counterexample, by hand.** On a two-dimensional section let A = [[1/2, 1], [0, 1/2]] and B = Aᵀ. Repeating either
stride alone has both multipliers at 1/2. The two-stride product B·A = [[1/4, 1/2], [1/2, 5/4]] has an eigenvalue 3/4
+ 1/√2, about 1.457. The alternating pattern amplifies a disturbance by (1 + √2)/2, about 1.207, per stride while both
isolated tests pass.

**Mechanism.** DESIGN.

- *The admitted language.* The task's terrain grammar and the controller's mode guards define which mode may follow
  which: a directed graph on modes, registered with the orbit. A gait with one mode has the trivial graph and nothing
  here applies.
- *Lower bound.* Enumerate the admitted cycles up to a length L (DESIGN default 4, with a count cap) and take the
  largest ρ(product)^(1/length). A value above one is an instability with a witness word.
- *Upper bound.* Search for one matrix P ≻ 0 and a rate γ with A_σᵀ·P·A_σ ⪯ γ²·P for every mode σ, a semidefinite
  program of the size of the section, by bisection on γ. This is the construction §22.9.4 already uses for the
  orderings of a simultaneous impact. A common quadratic norm is sufficient and not necessary.
- *Verdict.* `stable-under-switching` only if the upper bound is below one; `unstable-sequence` with its word if the
  lower bound is above one; `undecided` otherwise, with both numbers. The verdict is local, like every multiplier: it
  says nothing about the recovery basin, which is tested with finite disturbances.

For the counterexample the lower bound at length two is 1.207, no common contraction exists, and the verdict is
`unstable-sequence` with the word (A, B).

**Pilot, ceiling, kill.** Pilot: the counterexample, then a quadruped trot over alternating risers on the compliant
tier, with the bracket compared against 200 sampled admitted sequences of 50 strides. Falsifier: a sampled sequence
grows under a `stable-under-switching` verdict, or the bracket is undecided on more than half of the multi-mode gaits
of the battery. Ceiling: none on speed; it removes one class of false stability labels.

| | |
|---|---|
| GATE-GAIT-713 | Known answer. The counterexample returns the eigenvalue 3/4 + 1/√2 to 10⁻¹², the per-stride bound 1.2071 and the word (A, B); a pair of commuting contractions returns `stable-under-switching` with a common matrix that the checker verifies in exact rationals; a mutant that tests each mode alone labels the counterexample stable and fails. M4. |
| ✖ KILL-STRIDESEQ | Trigger: the pilot's falsifier holds after one repair cycle, or cycle enumeration exceeds its budget on the reference gaits. Fallback: no switching verdict is issued; multi-mode gaits are tested by finite-disturbance recovery over sampled admitted sequences, class `Tested`, and the per-mode multipliers carry the flag `single-mode-only`. |
<<<END>>>

### B14

<<<INSERT chapter=22 title="Sampled damping, and the work of a held command">>>
**What it is.** [S] The sampled-data condition under which the damping-injection backup of §22.12.3 is passive, and
the rule for accounting the work of any command held over a control period. In continuous time u = −K_d·y never adds
energy. A command computed at a tick and held for a period h can. Owner: `frob-shield` (fitting), `frob-control` (the
backups), `frob-chronicle` (the ledger). Verbs: `shield.fit`, `shield.check`.

**Counterexample, by hand.** A free 1 kg slider moves at +1 m/s. A sampled damper with gain 300 N·s/m reads the speed
and holds −300 N for 10 ms. The speed ends at −2 m/s. The kinetic energy rises from 0.5 J to 2 J: the damper has done
+1.5 J of work. The power at the start of the interval times its duration predicts −3 J.

**The condition.** DESIGN. For the port model of C-12 with reflected inertia M̄ at the collocated outputs and no other
force during the hold, one tick maps the output rate y by I − h·M̄⁻¹·K_d, and the stored energy changes by
−(h/2)·yᵀ·(2·K_d − h·K_d·M̄⁻¹·K_d)·y. Therefore:

- stored energy does not increase at the ticks exactly when 2·M̄ − h·K_d ⪰ 0;
- the rate does not reverse in any direction exactly when M̄ − h·K_d ⪰ 0;
- with one tick of actuation latency the scalar loop v′ = v − κ·v_prev, κ = K_d·h/M̄, is stable only for κ < 1, half
  the bound without latency.

`shield.fit` computes the bound from the least reflected inertia over the design's mass ranges and the control period
of the task, checks it in exact rationals, and fits the backup gain inside the no-reversal bound by default. With
latency or with other stored energy in the loop it uses the exact one-tick map of analysis A-22 and requires a
contraction in the energy norm. A gain outside the bound is refused (refusal: sampled damping gain above the passivity
bound). A brake or a shorted winding is a physical damper and carries no such bound; the backup's card says which it
is.

**The work of a held command.** DESIGN. For a force F held over h on an inertia m moving at v, the work is F·v·h +
F²·h²/(2m). The second term is never negative, so the power at the tick times the period always understates the work
done on the plant. Any budget that debits predicted work ahead of the motion (for example an allowance on the work of
the learned residual of §22.7, or a monitor that predicts the drive term of §22.11) reserves this bound with the
inertia at its least admitted value, or locates the instant at which the allowance is exhausted and changes the
command there. The ledger of §29.9 is unaffected: it integrates port power with the velocities of each physics step.

**Known answers.**

| Case | Value | Mutation that must fail |
|---|---|---|
| The slider, κ = 3 | −2 m/s; +1.5 J | Work taken as initial power times duration: −3 J |
| κ = 2 | The speed reverses to −1 m/s; energy unchanged | — |
| κ = 1 | Stops in one tick; −0.5 J | — |
| κ = 3/2 with one tick of latency | Diverges: the map has spectral radius √(3/2) | A bound of κ < 2 applied to the delayed loop |

| | |
|---|---|
| GATE-SHIELD-714 | Known answer. The four rows are exact. `shield.fit` refuses 300 N·s/m at 10 ms on an inertia of 1 kg and admits 100 N·s/m. Over 10⁴ random triples (M̄, K_d, h) with 2·M̄ − h·K_d ⪰ 0 the ledger's stored energy at ticks never increases, and each of 10³ planted violations shows an increase. The continuous-time identity remains a theorem card. M4. |
<<<END>>>

### B15

<<<INSERT chapter=24 title="The idle arm of the task audit">>>
**What it is.** [S] One more arm for `task.audit` (§24.20). The foil bodies probe ways of satisfying a task without
the intended mechanism. The idle arm probes the cheapest of them: doing nothing. A requirement or a reward built from
safety atoms alone is satisfied in full by a robot that never leaves its start. Owner: `frob-arena`. Verb:
`task.audit`.

**Hand case.** A delivery task uses the past-time requirement `(H (0 s 2 s) (and clear-margin thermal-ok spill-ok))`
as its online reward over 60 s. A parked robot keeps all three margins positive for the whole episode and collects the
full reward. A robot that carries the tray through the pass and loses part of its spill margin for one second collects
less. Search prefers the parked robot.

**Mechanism.** DESIGN.

- The audit runs the candidate design, and each foil, under three idle laws: zero command with the brakes as carded; a
  hold-posture command from the baseline controller of §22.4; and the backup controller of §22.12.3.
- A task is flagged `rewards-idleness` when an idle run's requirement verdict is `satisfied`, or when its return is at
  least a declared fraction of the best audited return (DESIGN default 0.5).
- The finding carries a proposed clause, like every finding of §24.20: a progress atom with its scale (for the hand
  case, approach speed above 0.15 m/s on a scale of 0.50 m/s, which gives the parked robot a margin of −0.3), or a
  completion requirement with a deadline. The author accepts it, which yields a new `TaskId`, or marks idleness
  intended, as a station-keeping task would.
- `eval.qualify` on a task with an unresolved idleness finding is refused, under the existing rule for unresolved
  findings.

| | |
|---|---|
| GATE-ARENA-715 | Known answer. On ten authored tasks with a planted safety-only reward and ten clean tasks, the idle arm flags ten of ten and none of ten (TARGET: at least nine, at most one). The hand case is flagged, and after the progress clause is accepted the parked robot's verdict is `violated`. M3. |
<<<END>>>

### B16

<<<INSERT chapter=30 title="Exposure delay and mount translation on a straight pass">>>
**What it is.** [S] A ninth structural alias beside the eight of §30.8, kept as its own fixture so that the count of
GATE-GROUND-03 is unchanged. A synchronized clock does not separate a sensor's exposure delay from a translation of
its mount along the direction of travel. Owner: `frob-ground`. Verb: `ground.excite`.

**The alias.** A range sensor with fixed orientation approaches a wall at coordinate L along a straight line at speed
v. With an exposure delay δ and a forward mount offset b its reading at time t is

    L − v·(t + δ) − b.

Only v·δ + b appears. At +2 m/s a delay of 20 ms and an offset of 40 mm give the same 101 samples of a one-second pass
at 100 Hz.

**What separates them.** DESIGN. `ground.excite` flags the pair when every planned pass has the same velocity along
the sensor's axis, and proposes, in order of the separation they give:

- the same pass reversed without rotating the robot: at −2 m/s the two explanations differ by 80 mm;
- a second speed: at +1 m/s they differ by 20 mm;
- an independent exposure mark on the shared hardware line of §30.7, which measures δ directly.

A fit requested on data with one axial velocity returns the identified combination v·δ + b and updates neither the
card's delay nor the mount transform. This is the spatial counterpart of the delay-against-offset alias of §30.7, and
the same alias binds the extrinsic graph of §20.14 when a faulty edge is a time offset on one pair.

| | |
|---|---|
| GATE-GROUND-716 | Known answer. In the synthetic rehearsal of §30.21, the two explanations produce bit-equal samples on the +2 m/s pass and the fit reports "delay times speed plus offset = 40 mm" with no card update. With the reversed pass added, both the 20 ms delay and a planted 40 mm offset are recovered within their intervals, and the predicted separation is 80 mm to 10⁻⁹. A card delay update from the single pass is refused. M5. |
<<<END>>>

### B17

<<<INSERT chapter=33 title="Six later foundation facts and their admission tests">>>
**What it is.** [S] Six rows that continue the tables of §33.11 and §33.17, each with what was read, the admission
test and the interim posture. They are counted separately from the five defects or limits and the one convention
hazard of §33.11. Rows 9, 10, 12 and 13 are limits or defects; rows 11 and 14 are convention hazards. Nothing was
executed for this section: each mechanism was read in source on 2026-11-21, the sum in row 9 was computed from the
source table, and reported values are marked.

| # | Component | Fact | Read in source | Admission test | Interim posture |
|---|---|---|---|---|---|
| 9 | `fs-mbd::robot_models`, the G1 catalog (`frankensim/crates/fs-mbd/src/robot_models.rs:398–403`) | The builder keeps 30 links and omits fixed attachments with nonzero mass. Reported: 1.377 kg over four welded links, against a 38-link source of 33.34014202 kg | The omission is stated in the builder's own list. The 30 masses sum to 31.96314202 kg. The upstream figures were not re-read | The mass roll-up through the airlock equals 31.96314202 kg, and the reduction lists its omissions | Used as the reduced profile of §14, never as the import-parity reference |
| 10 | `fs_ga::Se3::left_jacobian`, `right_jacobian` (`fs-ga/src/lie.rs:46`, `:1044–1049`) | The series refuses when the infinity norm of ad exceeds 32, so a pure translation of 33 m returns `IllConditioned`, a pose that `Se3::exp` accepts and that is regular | Confirmed: `SE3_SERIES_MAX_AD_NORM = 32.0`, and the refusal precedes the series. `exp` has no such limit (`:766–776`) | The public paths at translations of 1 m and 33 m, against the closed form below | `frob-lie` owns the differentials in closed form (§13); a limit of an owner is reported as `unsupported`, never as a singular pose |
| 11 | `fs_phs::transformer` and `interconnect_mapped` (`fs-phs/src/interconnection.rs:91`) | Opposite signs of the driven direction; both conserve power | Confirmed by the source comment. The rates (12, −8) and (−12, 8) are reported | The signed motion witness of §15 through the airlock, for both entry points | Only `interconnect_mapped` is reached, with the authored sign passed explicitly |
| 12 | `fs_contact::broad_phase::swept_sphere_aabb` (`fs-contact/src/broad_phase.rs:146–156`) | Encloses straight centre motion only | Confirmed as a documented scope limit | The arc fixture of §18: the helper's bound misses the obstacle, and the arc-aware bound contains it | `frob-contact` builds its own bounds for rotating links |
| 13 | `fs-sos` (`fs-sos/src/lib.rs:80–84`, `:289–308`) | Two cases in which a returned value is not a bound. `Poly::new` trims trailing coefficients below 10⁻¹⁵ in magnitude, so 1 − 2⁻⁵⁰·x becomes the constant 1 and is certified as 1 on abs(x) ≤ 2²⁸, where the minimum is 1 − 2⁻²². `certified_bound_on` forms its result by one subtraction rounded to nearest, and returns 1 for 1 − x/2 on abs(x) ≤ 2⁻⁵⁵, where the minimum is 1 − 2⁻⁵⁶ | Both mechanisms confirmed by reading. The two returned values are reported and were not re-run | Both probes through the airlock, each compared with exact rational evaluation of the authored polynomial at the interval's ends | No `fs-sos` value enters a certificate. The `sos-gram` kind is checked in exact rationals (§12.4); a bound that is used as evidence is recomputed with the authored coefficients and an outward-rounded last step |
| 14 | `f3d_math::Ray::apply_matrix4` (`franken_threed/crates/f3d-math/src/ray.rs:487–490`, `vector3.rs:708–719`) | The transformed direction is renormalized, so hit parameters under a scaled instance are in local units | Confirmed by reading | The two-plane fixture of §19 through the adapter | The adapter converts by s_W = s_L/‖A⁻¹·d‖ |

**Row 10, the closed form and the hand case.** For a pure translation the twist has zero angular part, ad² = 0, and
the left and right differentials of the exponential are exactly I + ad/2 and I − ad/2, both with determinant one. For
a translation of 33 m along +x and a unit rotation rate about +z, the linear y component is −16.5 m in one and +16.5 m
in the other. A warehouse aisle is longer than 32 m, so a refusal there would exclude ordinary poses. The anchored
coordinates of §11 keep translations inside a cell small, but a relative pose between two bodies in one aisle is not
anchored.

**Row 13, why it matters although no certificate depends on it.** §25.4.9 and C-32 name `fs-sos` as an owner worth
probing, and §22.12.4 cites its region-of-attraction routine. A value from it is a proposal to check. Two rules follow
for any bound this project accepts from a floating-point routine: the bound is evaluated on the authored coefficients,
or the error of translating them is bounded on the actual domain; and the last operation is rounded outward.

**Upstream asks**, added to Appendix F by description: fold or declare the omitted masses of the G1 catalog; a
closed-form differential for small rotation and large translation; a documented or removed legacy transformer; an
arc-aware swept bound; an authored-coefficient check and an outward-rounded result in `fs-sos`; a ray transform that
returns the scale of its parameter. None is scheduled here.

| | |
|---|---|
| GATE-WS-717 | Known answer. Each of the six admission tests is recorded at the pin with its result: the roll-up of 31.96314202 kg; the refusal at 33 m and the agreement at 1 m to 10⁻¹², with the closed form returning ∓16.5 m; the signed motion witness for both transformer entry points; the arc fixture; the two `fs-sos` probes compared with exact evaluation; the two-plane ray fixture. A probe that now passes upstream is recorded as fixed and its interim posture is reviewed. M0, with rows 12 and 14 at M2. |
<<<END>>>

### B18

<<<INSERT chapter=38 title="Known answers adopted from late fixtures">>>
**What it is.** [S] Nine rows for the known-answer suite, each with a negative control. The mechanism that each row
tests is already in its owner section; the rows are the smallest cases that distinguish it from the shortcut. Every
value is derived on file from the stated inputs. The rows are owned by `frob-conformance` and run under the verbs of
their subsystems.

| Row | Inputs | Value | Negative control | Owner section |
|---|---|---|---|---|
| Moment merge in single precision | Two equal-size shards of constants: 4,095 and 4,097; −1 and +1; 8,191 and 8,193 | Population variance 1 in each case by the centred merge of §23.9 | The raw second moment minus the squared mean returns 0 for the first and third pairs, because 4,096² + 1 and 8,192² + 1 are not representable, and 1 for the second | §23.9 |
| Unresolved outcomes in a comparison | Baseline 60 successes in 100 valid trials; candidate 2 successes in 2 valid trials and 98 unresolved | Complete-case rate 100%; success rate in [2%, 100%]; no preference | A ranking on complete cases prefers the candidate | §24.10, §28 |
| Independence of a writable view | A 2×2 view with element strides (1, 1) | Index [0, 1] and index [1, 0] address one element; stores of 7 and 11 leave 7 or 11 by order; the view is refused for writing | A check of address bounds alone admits it | §33.11 row 3, §21.9 |
| Reality of a certified root | f(z) = z³ + z at z = 0.001 + 1.001·i | α(f, z) = 0.0021 certifies a root near z, which is +i. α(f, Re z) = 0.0010 certifies a root near Re z, which is 0, a different root. ‖z − Re z‖ = 1.001 exceeds 1/(20·γ(f, z)) = 0.033, so the robust test of C-34 does not call the root real | A test that infers reality from α(f, Re z) alone calls +i real | §25.21.4 |
| The Cayley map on SE(3) | ω = (0, 0, 2), v = (1, 0, 0) | Rotation: a quarter turn about z. Translation: (I − ω̂/2)⁻¹·v = (1/2, 1/2, 0) | The map that pairs cay(ω̂) with v unchanged returns (1, 0, 0) and fails the differential test of C-36 | C-05, §13 |
| Tube erosion with a one-sided error | One dimension; a cell [0, 1] of measured states; true state = measured + e with e in [0, 0.1] | Admissible true successors are [0.1, 1.0], the erosion by −E | Erosion by E gives [0, 0.9]: it admits 0.05, whose measurement can be −0.05, and rejects 0.95, whose measurements lie in [0.85, 0.95] | §22.12.5 |
| Residual bound in single precision | H = 3, b = 1 | x̂ = 0.33333334; the computed residual is exactly 0; the true error is 9.93 × 10⁻⁹; with the margin ρ = 2.4 × 10⁻⁷ the bound is 7.9 × 10⁻⁸ | The bound without ρ claims zero error | §18.19 |
| Film front at a dry cell | Two cells with h = (1 mm, 0) | Face mobility ½·(h_L³ + h_R³) = 0.5 mm³, so volume crosses the face and the planar front follows t^(1/5) | A harmonic or geometric mean of the cubes gives zero flux, and the spill never reaches the dry cell | §19.10.3 |
| One edge of the extrinsic graph | Predicted pose of sensor 2 in sensor 1: a translation of (1, 0, 0) m. Sensor 1's mount is off by a rotation of 0.01 rad about z; sensor 2's is exact | r₁₂ = −Ad(T̂₂₁)·δ₁ has angular part (0, 0, −0.01) rad and linear part (0, −0.01, 0) m: sensor 2 appears at (1, −0.01, 0) m | Using Ad(T̂₁₂) on the tail gives a linear part of (0, +0.01, 0) m | §20.14 |

FACT (a recorded pilot, Appendix K; to be re-run before a gate cites it): in the first row a native single-precision
kernel on the Metal device and on each RTX 4090 returned 0 by the raw form and 1 by the centred merge for the pair at
4,095 and 4,097, 1 by both for the pair at ±1, and 0 by the raw form at 8,192. Each device merged locally; nothing
about communication between devices or training throughput follows.

| | |
|---|---|
| GATE-GAUNT-718 | Known answer. Every row is reproduced at its tolerance (single-precision rows bit-exact on every backend; rational rows exact; the root row to four significant figures in exact rational evaluation of α and γ) and every negative control fails. Each row lands at the milestone of its owner section. |
<<<END>>>

---

## Notes for the lead

- **Appendix K (recorded pilots).** Four blocks cite recorded pilots that the pilot register does not yet list: the
  sampled slider on three devices (B8), the single-precision moment merge on three devices (B18), the raw-dynamics and
  interval checks of the continuous-gain docking game (B2), and the four-law probe follow-up (B4). Each is written
  "FACT (a recorded pilot, Appendix K; to be re-run before a gate cites it)". Every count and closed form in those
  blocks was also recomputed for this file, so the blocks do not depend on the register entry.
- **Appendix F (upstream asks).** B17 names six asks by description.
- **Fields, flags and labels introduced.** `policy_class.law` and the certificate's `law` object (B1); the sensor-card
  field `error_dependence` and the label query's `channel` (B3); a `scope` on nuisance parameters (B4); the answer
  labels `lower-bound-only` and `pareto-candidates` (B5); the design field `reduced` and the profile `g1-reduced-30`
  (B6); a phase source on `ControllerId` (B12); a mode set on `gait.orbit` with the verdicts `stable-under-switching`,
  `unstable-sequence` and `undecided` (B13); the task flag `rewards-idleness` (B15); the flag `ambiguous` on a
  velocity estimate (B11). No new verb is introduced.
- **Refusals, by description.** Inclusion over a disturbance set without a declared law class; redundancy credited
  without a joint error law; witness assembled from unshared values of a shared parameter; sizing returned without a
  feasibility witness; mass dropped at import without a declared reduction; refinement study that changes a clock;
  fixed-period shooting on a loop with a free period; sampled damping gain above the passivity bound; swept bound
  requested without a speed bound.
- **Counts quoted elsewhere.** B16 adds a ninth structural alias under its own gate so that "eight" and "8 of 8" in
  §30 and §40 stay true. B17 keeps "five defects or limits" and "one convention hazard" true by counting its six rows
  separately; C32 says so in §1.7.
