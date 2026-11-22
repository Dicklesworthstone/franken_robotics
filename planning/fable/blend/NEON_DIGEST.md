# NEON digest for the hybrid FrankenRobotics plan

Source digested: `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` (v2.0, 1,391 lines, read in full).
Compared against: `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (the BASE plan, v0.2; Parts 0 to III read in full, Parts IV to VI and the appendices read or skimmed).
Date: 2026-11-21.

## 0. How to read this digest

**Columns used throughout.**

- **BASE**: `Has` (with the BASE section), `Weaker` (with how NEON is stronger), or `Lacks`.
- **Verdict**: `Sound` (mathematically correct and implementable), `Plausible` (reasonable, but unproven or under-specified), `Doubtful` (reason given), `Ornament` (the mathematics is correct, but a plainer formulation does the same work).

**NEON's provenance codes.** NEON folds in a three-model idea duel, so its text is full of tags. `CC-nn`, `TA-Pn`, `B1`–`B3` are proposals attributed to Claude; `COD Wnn`, `FTA-Pn`, `BS-1`–`BS-3` to the OpenAI model; `AGY #n` to the Gemini model (NEON's author). `Cnn` (for example `C13`) marks a correction conceded during the duel; `† K-nn` a killed hypothesis. These tags carry no technical content and need not survive the blend.

**What I checked independently, so you know which NEON statements rest on more than its own word.**

| Check | Result |
|---|---|
| Sibling facts on disk (NEON §0.6) | Confirmed: frankensim at `483d09c682` on `nightly-2026-07-06`; `fs-qty` uses `#![feature(generic_const_exprs)]`; `fs-mbd`, `fs-contact`, `fs-evidence`, `fs-lbm`, `fs-tribo`, `fs-solid`, `fs-ports`, `fs-system`, `fs-couple` all depend on `fs-qty`; frankensympy at `29f847c1` with `MAX_VARIABLES = 8`, `MAX_EQUATIONS = 64`, `MAX_DEGREE = 256`, `MAX_BITS = 4096`; franken_lean at `4426f8ea7` with `fln-anvil` = 1,423 lines (`lib.rs` + `omega.rs`) and `fln-kernel/src/tc.rs` = 516,842 bytes; `ft-kernel-metal` `MAX_ARGUMENTS = 31`; franken_threed has four crates; fastmcp_rust on `nightly-2026-08-25`. The symbols `fs-tropical::max_cycle_mean`, `fs-phs::interconnect_mapped`, `fs-io` `step_faceted_export` and `supplier_corpus`, `fs-solid` `rod` and `fatigue`, and Floquet code in `fs-spectral` exist. |
| Sibling facts NEON got wrong | `fs-spectral` and `fs-matdb` also depend on `fs-qty`, yet NEON lists both as `fs-qty`-free crates to consume directly. `fs-symmetry` is 480 lines and `fs-fab` 471 lines, far thinner than NEON's use of them implies. |
| Size of NEON's "lift" set | Line counts of Rust source: `fs-mbd` 292k, `fs-contact` 147k, `fs-solid` 443k, `fs-lbm` 517k, `fs-tribo` 89k, `fs-evidence` 57k, `fs-system` 49k, `fs-ports` 14k, `fs-couple` 1.84M. NEON's whole budget is 378 kLOC and has no line for lifted code. |
| NEON's verification scripts | `scratch/verify_math.py` and `scratch/verify_neon_v2.py`, cited for every "Python-verified" number, are not present anywhere under the project directory. |
| Numbers I recomputed | Correct: Burnside 86,185 and 500,500; ballast classes 24,546 of 735,471; retrofit kits 10,700; 27 contact strata; `.fenergy` 1.49 J and 50.5 ms; Delassus sizes 1.86 KB and 4.70 KB; multi-RHS flop ratio 13.6; α-spending series sums to α; 720 → 8 schedule classes; sort-and-clip example. Not reproducible: the thermal cliff (the stated parameters give 21.1 s to reach 120 °C, not 33.5 s, and an exponent of 0.00657/s, not 0.00653/s); the Bekker sinkage ratio; the `.fclear` heading half-bin (48 heading bins give 3.75°, not 5.625°). |
| Formulas I checked by hand | Correct: coadjoint action; adjoint matrix; reflection parity on twists; pseudo-inertia; curvature sign (agrees with the Magnus expansion for ġ = g·ξ, ξ = −A·ṙ); saltation matrix; Smale constant (13 − 3√17)/4; Cheeseman–Bennett factor. |

---

## 1. Thesis, incumbent audit, decalogue, bets, goals, North-Star scenarios (NEON Part 0, §1–§5)

### 1.1 Document-control devices (NEON Part 0)

| Device | What it is | BASE | Verdict |
|---|---|---|---|
| Evidence labels and ambition tags (§0.1) | FACT, DESIGN, HYPOTHESIS, TARGET, OPEN; [S], [F], [M]. | Has (§0.1), identical. | Sound |
| **Program Measurement Protocol** (§0.1) | Every headline comparison: paired campaigns with n ≥ 10, a 95% paired lower confidence bound, the strongest warm or cached incumbent configuration frozen after reference-only tuning, and every compile, admission, sync, transfer, fallback, refusal and verification cost in the denominator. Performance gates add a 20-minute sustained thermal floor. | Weaker. §28.2 has a fuller disclosure block and §28.3 interleaved paired runs, but no rule for comparisons against incumbents and no thermal floor in minutes. | Sound. Adopt. |
| "Implemented" in seven conditions (§0.2) | Crate and budget compliance; ten-section `CONTRACT.md`; pinned oracles; hardware-twin parity for any GPU kernel; cancel-correct and crash-safe including a schedule check; reachable from both `frob --json` and `frob-mcp`; a digest-pinned gate receipt with commit, hardware fingerprint, seed and numbers. | Has (§0.2, eight conditions). NEON adds the hardware fingerprint in the receipt and MCP reachability; BASE adds evidence class at the agent boundary. | Sound |
| **Five claim axes plus `ClaimProduct`** (§0.4, `INV-001`) | Axes: `VerifiedFormal`, `VerifiedNumeric`, `ValidatedPhysics`, `EvaluatedDesign`, `BenchmarkedPerf`, each with what it does not establish and permitted wording. Rules: a formal theorem combined with an estimated material card is capped at `Estimated`; a formal conditional theorem combined with runtime monitors and statistical bounds is a `ClaimProduct<VerifiedFormal, EvaluatedDesign>` with no `From`/`Into` to `VerifiedFormal`, enforced by a compile-fail test; every export carries `execution_authorized: false`, `hardware_safety_signoff: false`, `sim_to_real_certified: false`. | Weaker. §6 has six claims and the weakest-input rule (D-5), but the rule is prose, not a type, and there is no "design improvement" axis. NEON folds BASE's Known answer, Parity and Fidelity into one numeric axis, which loses the distinction between agreeing with an oracle and agreeing with a slower tier. | Sound. Adopt the type and the three hard-wired flags; keep BASE's six claims and add "Design comparison" as a seventh. |
| Live-code grounding table (§0.6, `F-01`–`F-08`) | One row per sibling: verified reality, then the mandatory architectural consequence. Notable rows: Metal has no `f64` and fast-math folds away double-float `TwoSum` (`F-01`); two RTX 4090 have no NVLink and peer access is normally off (`F-02`); `fs-qty` needs `generic_const_exprs` (`F-03`); `fln-anvil` implements only `DTree` and `omega`, the rest are stubs (`F-04`); `real_rur` caps and zero-dimensional ideals only, `fsym-formal` covers two univariate profiles (`F-05`); frankentorch tensors are CPU-only and `ft-nn` pulls `rayon` (`F-06`); franken_threed has four crates (`F-07`). | Has, scattered (§1.5, §10.1, §19.4, §20.4, §25). NEON's single table with a consequence column is easier to audit. The `fs-qty` nightly fact is the one BASE under-weights: BASE lists `fs-qty` as bedrock-direct and requires its types in every public signature (L-6). | Sound, with the two errors noted in §0. |
| Hygiene rules (§0.7) | Delivery logs outside the plan; LOC ceilings at 1.15×; convergence before breadth; refusals over silent clamps; one source of truth per boundary. | Has (§0.7), with stronger anti-ceremony rules. | Sound |
| Twelve named subsystems (§0.5) | AXIOM, LIE, CAS, PROOF, MORPH, TERRA, DYNAM, FORGE, OPTIC, CORTEX, ORACLE, ATELIER. | Has its own seventeen names (§0.5). | Naming conflict; see (C). |

### 1.2 Thesis and incumbent audit (NEON §1)

**Thesis.** The gain is not a faster step. It is changing the unit of work from "one frozen robot, one kernel, one training run" to the whole design campaign: synthesis, feasibility screening, module substitution, mixed-morphology simulation, gait continuation, skill transfer, adversarial testing, counterfactual repair, manufacturing outputs. NEON's four headline shifts: (1) algebra removes simulations before they run; (2) morphology is data, so many topologies share one GPU batch; (3) safety claims are stratified and cannot be laundered; (4) worlds are living, failures are shrunk and explained, and the output is a build packet.

**Incumbent audit.** A table per engine with "real capabilities preserved in fair benchmarks" and "structural wall":

| Engine | Conceded strengths | Wall NEON claims |
|---|---|---|
| MuJoCo, MJX, MuJoCo Warp | Fast procedural `mjSpec` edits with state-preserving recompilation; warm JIT caches; per-world assets and continuous parameter fields; robust soft convex contact | Compiled layouts fix tree structure; no substitution contracts, gait atlases, linkage synthesis, proof kernel, robot-advected films |
| Isaac Lab (PhysX 5, Newton) | CUDA throughput; multi-asset spawning; articulation views; Newton deformables | CUDA-only; views need compatible layouts; no analytic contact derivatives, symbolic algebra or proofs |
| Drake | CPU accuracy; `Diagram` composition; autodiff; direct trajectory optimization; floating-point SOS; hydroelastic contact | CPU-only; finalized plant fixes topology; no independent small proof kernel |
| Genesis | Multi-solver GPU backends including Metal; coupled deformables; checkpoint and partial reset | `scene.build(n_envs)` replicates one layout; no mixed-topology execution |

`GATE-RIVAL-AUDIT` (M0) pins exact versions and capabilities of each incumbent and freezes the fair baselines before any comparison is made.

- **BASE:** Weaker. §1.1 lists "fossils" and §20.9 lists routes not taken, but there is no per-incumbent concession table and no gate that freezes baselines.
- **Verdict:** The device is Sound and worth adopting. The "zero X" walls are stated absolutely and several are overstated; do not inherit the wording. The claim that algebra deletes "90%–99% of futile simulations" has no basis beyond one contrived fixture.

### 1.3 The decalogue (NEON §2)

P1 safe Rust everywhere, one quarantined CUDA leaf (50 of 51 crates `forbid(unsafe_code)`). P2 geometry-first authoring, flat branch-free runtime layouts (128-byte `MorphoTable` rows, 48-byte `AffineRows`, 8-float motors). P3 campaign-time quotienting before step-time brute force. P4 structure preservation and honest contact. P5 reflective proofs, stratified safety, no claim laundering. P6 asymmetric roles for the two machines with symmetric numerical parity. P7 one world truth for physics, sensors, clearance and rendering. P8 cancel-correctness, schedule checking, α-spending integrity. P9 honest evidence and actionable refusals (ranked repairs or separating witnesses). P10 one verb registry driving CLI, daemon, MCP, TUI, viewer and build packets.

BASE has P1, P2 (as D-1, D-3), P4, P7, P9 and P10 in its own words. P3, P6 and the α-spending half of P8 are new to BASE.

### 1.4 The sixteen bets (NEON §3)

| ID | Bet and claimed lever | Falsifier | Gate | BASE | Verdict |
|---|---|---|---|---|---|
| BET-01 [F] | MorphoJIT: bytecode interpreter plus limb port interface plus ragged contact queues plus asynchronous specialization steps 512 distinct topologies in one GPU grid. | In `XT-P2`, fails ≥ 10× over per-morphology graphs at ≤ 128 morphologies, or fails ≥ 0.33× of specialized speed per degree of freedom, or nine per-family templates reach ≥ 95% mutation coverage and higher throughput. | `GATE-MORPHVM-01`, M2 | Lacks | Plausible; mechanism Sound |
| BET-02 [F] | Campaign quotienting (`.fmodule`, Burnside, `.fquot`, gait screening) deletes ≥ 80–90% of campaign GPU work. | In `XT-P1`, quotient arm < 5× over generic, or < 6 of 8 top-8 overlap, or planted winner loses rank 1, or one false equivalence. | `GATE-OPERAD-01`, M1–M4 | Lacks | Mechanisms Sound; the percentage is Doubtful |
| BET-03 [F] | Gait atlas with Floquet stability, saltation adjoints and continuation speeds periodic co-design ≥ 13×–100× over learning from scratch or finite differences. | On 100 periodic cases with 20 grazing or unstable controls: any control not refused, or < 72 of 80 with gradient cosine ≥ 0.95 against central differences, or continuation fails on ≥ 50% of catalog, or warm-start speedup < 2×. | `GATE-ATLAS-01`, M4–M5 | Weaker (C-10, A-04: atlas and timing only) | Plausible |
| BET-04 [F] | Exact bounded synthesis plus homotopy continuation enumerates all isolated real closed-loop mechanisms (4,326 nine-point four-bars in ≤ 2 s re-query). | Misses known counts (4,326; 40; 16; ≤ 4), > 0.1% lost paths, disagrees with a modular Gröbner count, or admits one false branch. | `GATE-BURMESTER-01`, `GATE-HOMOTOPY-01` | Lacks (parked in App. C) | Lane 1 Sound; lane 2 Plausible |
| BET-05 [F] | Kernel-reflective proofs and three safety tiers at ≤ 5% GPU overhead. | Reflection unsound, any `.fenergy`/`.ftube` false admission, shifted-world bound violated, shield keeps < 85% return, or a check exceeds 5 s. | `GATE-PROOF-01`, `GATE-SHIELD-01`, `XT-P3` | Weaker in ambition, stronger in honesty (§10) | Tier 1 Sound; tier 3 Doubtful |
| BET-06 [F] | `.fskill` capability maps plus a learned residual transfer skills zero-shot to unseen bodies. | On 24 unseen bodies (4 infeasible): < 18 of 20 feasible reach ≥ 90% success within 3 points of tuned baselines, or one infeasible body admitted, or amortized transfer < 10×. | `GATE-TRANSFER-01`, M5 | Lacks (§18 synthesizes per body) | Layer 1 Sound; residual Plausible |
| BET-07 [F] | Trapezoidal variational integrator, nonholonomic wheels, 4D contact, joint stiction and backlash pass the known-answer suite without drift. | Fails `KA-01`–`KA-10`; implicit step > 2× explicit; unified solve fails to converge; multi-RHS misses 1e-6 residual. | `GATE-DYNAM-01`, `GATE-CONTACT-02` | Has a different contact formulation (C-05, C-06) | See §4 |
| BET-08 [S/F] | Fleet: the Mac hosts 500–1,000 resident worlds and proof work; the 4090s train; ≤ 1 GB per campaign-hour between them. | 1,000-world bank does not cut the held-out gap and dual-machine speedup < 1.3×; or throughput floors missed. | `GATE-FORGE-01`, `GATE-FLEET-01` | Lacks | Plausible |
| BET-09 [F] | Robot-advected thin-film friction, slosh and cords at ≤ 15% training overhead change real rankings. | Fails Huppert t^(1/8) (±2%) or Housner (±3%); films change < 20% of kitchen rankings and cut spill failures < 3×; overhead > 25%. | `GATE-TRIBO-01`, `GATE-MULTI-01` | Lacks (static wet patches only) | Plausible |
| BET-10 [F] | Adversarial search, minimal counterexamples, 1,024-way counterfactual forks and ≤ 3-item building retrofits cut discovery and repair cycles. | Finds planted traps (p ≤ 3e-5) < 3× faster than uniform randomization; < 70% counterexamples solvable; autopsy cuts repair iterations < 3×; kits fix < 30% of failures. | `GATE-REDTEAM-01`, `GATE-TIMETRAVEL-01`, `GATE-AMEND-01` | Lacks | Sound |
| BET-11 [F] | Real, complex and quaternionic irreps plus wreath-product weight tying give ≥ 1.8×–5× sample efficiency with no host copies. | < 1.3× over ten seeds, or > 3% asymptotic regression in clutter, or wreath tying < 1.5×. | `GATE-EQUIV-01`, M4 | Has (BET-07, 0.7× steps) | Sound; see §5 |
| BET-12 [S/F] | Eight-stage preflight with observation-alias witness and clearance complexes in < 200 ms to seconds. | Misses > 20% of 100 seeded defects; one false route; one false impossibility witness. | `GATE-PREFLIGHT-01`, `GATE-CLEAR-01`, `GATE-OBSALIAS-01` | Weaker (§22.2 lacks stages 7 and 8) | Sound |
| BET-13 [S/F] | Thermodynamic actuators plus two-timescale lifecycle compression: 1,000 hours ≥ 100× faster at ≤ 5% error. | `KA-09` fails; compression < 50×; planted backlash-induced gait bifurcation missed. | `GATE-ACT-01`, `GATE-LIFE-01` | Lacks lifecycle; has actuators (§13) | Plausible |
| BET-14 [S/F] | Epistemic integrity suite: order-independent GPU scatter; model-form and energy-creation audit; schedule checking; blinded α-spending qualification. | Deterministic path > 10% overhead; audit misses a planted pyramid-cone or energy-pumping exploit; one planted race missed; false-promotion rate above α under adaptive retries. | `GATE-DET-01`, `GATE-RANKCHECK-01`, `GATE-DPOR-01`, `GATE-QUAL-01` | Has determinism (§27) and exploit detectors (§19.6); lacks blinded qualification and schedule checking | Sound |
| BET-15 [S/F] | Spec-to-silicon: temporal-logic missions compiled to build packets, QEMU-verified firmware and ≤ 15 MB offline dossiers. | < 11 of 12 packets accepted by an independent reviewer; robustness disagrees with reference on any trace; firmware diverges > 1e-5 or allocates; fresh agent solves < 5 of 10 missions. | `GATE-S2S-01`, `GATE-FW-TWIN-01`, `GATE-WEBSTUDIO-01` | Lacks (NONGOAL-04) | Plausible; partly outside what a simulator can gate |
| BET-16 [S/F] | Scan uncertainty and coverage deficits from the reconstruction pipeline feed adversarial priors and produce re-scan requests. | Fails to localize 100% of seeded occluded bottlenecks on five real-scan fixtures, or fails to tag unprobed surfaces conservative. | `GATE-REAL2SIM-01`, M5–M6 | Weaker (§16.6 unknown-region policy only) | Plausible |

Every NEON bet has a kill gate with a named fallback (21 kill gates in all; BASE has 9).

### 1.5 Goals, non-goals, anti-goals (NEON §4)

- **Goals.** `GOAL-001`–`012` restate v1 (Lie and PGA core, variational integrator with constraints, frankensympy, franken_lean, both GPU families, six interiors plus outdoor soil, 18 morphologies, sensors and invariant filter, equivariant learning and co-design, CLI and MCP studio, claim honesty, URDF/MJCF/glTF/USD/scan interop). `GOAL-013` campaign quotienting and MorphoJIT. `GOAL-014` stratified safety and reflective proofs. `GOAL-015` living multiphysics, adversarial testing, counterfactual autopsy, lifecycle. `GOAL-016` spec-to-silicon and the epistemic firewall.
- **Non-goals.** 001 motor actuation, safety-rated BSPs, hardware signoff (firmware sits behind a caller-owned `MotorBusHal`). 002 wrapping C++ engines (oracles only). 003 photorealism and browser CAD sculpting. 004 full aerodynamics (compact rotor ports only). 005 full-room 3D fluid or cloth inside training loops. 006 unbounded proofs about contact chaos or learning convergence. 007 single-right-hand-side half-precision contact factorization.
- **Anti-goals.** Silent clamping; separate physics, sensor and visual worlds; CPU tensors or `rayon` in hot loops; decorative mathematics; tuning against the confirmation benchmark.

Scope conflicts with BASE: NEON includes continuum rods, an aerial rotor port, tactile hands and firmware export, each a BASE non-goal (NONGOAL-02, -03, -07, -04).

### 1.6 North-Star scenarios (NEON §5)

| ID | "After" experience | Milestone | Nearest BASE scenario |
|---|---|---|---|
| NS-01 | `frob verify` catches invalid inertia, zero curvature, a doorway pinch, and proves a sensing blind spot with the cheapest sensor fix, in < 200 ms to 2 s. | M1, M3 | NS-01 (no world or sensing checks) |
| NS-02 | 512 topologies in one GPU grid; racing anneals 512×32 worlds to 8×2,048 while survivors are promoted to specialized kernels. | M2, M5 | None |
| NS-03 | Exact 3- and 5-pose linkages in < 100 ms; all 4,326 nine-point four-bars and 40 Stewart roots certified, ≤ 2 s re-query. | M1, M3 | None |
| NS-04 | Kitchen wheel-leg robot: grease spreads under tires, steam blinds the depth camera, the knee hits a thermal cliff; adversarial search plus counterfactual forks repair gear ratio and camera mount. | M3, M5 | NS-04 (static wet patch) |
| NS-05 | Hexapod D₆: 12× fewer weights, ≥ 5× faster learning, gait atlas plus skill transfer reallocate wrenches after a leg fails. | M4 | NS-05 |
| NS-06 | Humanoid with energy budgets on impacts, reachable tubes on threshold resets, shielded exploration at ≤ 5% overhead. | M4 | None |
| NS-07 | Warehouse and retail mobile manipulator: certified shelf reach, glass multipath, lidar observability, fragile grasps. | M3–M6 | Partly NS-03 |
| NS-08 | Lengthen a femur 15%: the periodic orbit is continued in < 1 s, period-doubling is flagged, design gradients returned. | M4, M5 | None |
| NS-09 | Swap a wheel for a foot: skill transfers zero-shot, or a Farkas witness says it cannot before training. | M5 | None |
| NS-10 | Tracked rover: soil model catches 9.8 cm sinkage belly-out; track width 8 → 12 cm fixes it. | M3 | NS-08 |
| NS-11 | Rare failure shrunk to ≤ 3 edits; 1,024 intervention branches in < 1 s return attributions and an offline replay page. | M3, M5 | Partly NS-02 (teleport) |
| NS-12 | A scanned building enters with coverage deficits; design is conservative around blind spots; a targeted re-scan request is issued when occlusion drives the bill of materials. | M5, M6 | NS-12 |
| NS-13 | 1,000 hours compressed to < 45 s exposes backlash-induced gait collapse; schedule check finds a watchdog/commit race in 8 replays instead of 720. | M5 | None |
| NS-14 | Blinded qualification with α_i = α/[i(i+1)]; proofs and lineage replay offline with no GPU or network. | M5, M6 | Partly NS-03, NS-11 |
| NS-15 | Prompt to build packet: STEP, priced bill of materials, tolerance stack, wiring harness, verified `no_std` controller. | M4, M6 | None |
| NS-16 | The Mac holds 500–1,000 hot worlds, the viewer and time travel; the 4090s train; < 1 GB per hour crosses. | M2, M4 | Partly NS-10 |

BASE scenarios with no NEON counterpart, to keep: NS-02 teleport, NS-03 honest A/B, NS-06 thirty iterations with bisect, NS-07 import loss report, NS-09 exploit caught by promotion.

---

## 2. Geometry and conventions (NEON §6, crate `frob-lie`)

BASE counterpart: §9, §11, C-01 to C-03.

**The normative conventions table (§6.0), pinned to `fs-ga`, checked by `KA-01` and `PO-LIE-1`.**

| Concept | NEON's definition | Runtime packing |
|---|---|---|
| Pose | g = (R, p), x_b ↦ R·x_b + p | Authoring and proofs: unit PGA motor, 8 floats. Hot loops: `AffineRows`, a 3×4 row-major `f32` block of 48 bytes |
| Body twist | ξ = g⁻¹ġ, angular first (ω, v); ad_ξ = [[ω×, 0], [v×, ω×]] | Matches Featherstone and `fs-mbd` |
| Body wrench | (τ, f), pairing τ·ω + f·v; **ad\*_ξ w := ad_ξᵀ w = (τ×ω + f×v, f×ω)**; Euler–Poincaré μ̇ = ad\*_ξ μ + w_ext; "transpose, not inverse transpose" | — |
| Adjoint | Ad_g = [[R, 0], [p×R, R]]; body-to-world wrench w_w = Ad_{g⁻¹}ᵀ w_b; pullback w_b = Ad_gᵀ w_w | Three newtypes: `AdjointMap`, `WrenchPullback`, `WrenchPushforward` |
| PGA basis | e₀² = 0, e₁² = e₂² = e₃² = 1; planes are vectors, lines bivectors, points trivectors; sandwich product with a parity sign on odd versors | From `fs-ga` |
| Reflections | For R with det R = −1: T_twist(R) = diag(det(R)·R, R); pseudo-inertia J′ = g_v·J·g_vᵀ | Needed for mirrored limbs |
| Small angles | exp, log, dexp⁻¹ and Cayley use branch-free degree-4 polynomials for sin θ/θ, (1 − cos θ)/θ², (θ − sin θ)/θ³ below 1e-4 rad | Removes 0/0 in kernels |

- **BASE:** Has the same orderings (§9) plus things NEON lacks: `a_from_b` naming, unit-complex revolute coordinates, integer ticks, anchored positions, world addresses.
- **What to take.** (1) The explicit coadjoint component formula. BASE §9 says ad\* is "the negative transpose action", while BASE Appendix D writes π̇ = π×ω + ℓ×v + τ, which is the plain transpose that NEON states. One of BASE's two sentences is wrong under its own convention; NEON's row removes the ambiguity. (2) The three typed wrench maps. (3) The reflection-parity formula for twists, which BASE's "signed-permutation representations" (C-09) implies but never writes. (4) The small-angle polynomial rule for the exponential lane (BASE's Cayley lane does not need it).
- **Verdict:** Sound.

**Other §6 items.**

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| PGA Cl(3,0,1) for screws, Plücker axes, planes, rays, motors; join and meet for line–plane intersection and reciprocal screws (§6.1) | Consumed from `fs-ga` | Has (ADR-MARROW-01) | Sound |
| Conformal GA Cl(4,1) for sphere, capsule-sweep and linkage-circle intersections at preflight time, erased to flat arrays on the GPU (§6.1) | Compile-time only | Lacks; App. C rejects a second geometric-algebra layer | Ornament. The same intersections are short closed forms. |
| Dual-number differentiation through exp, log, Ad (`Dual<T>`, `Se3Dual`) (§6.1) | Verifies analytic Jacobians and adjoints | Has (`fs-ad`) | Sound |
| Pseudo-inertia J = [[Σ, h], [hᵀ, m]], realizable iff J ≻ 0 (§6.2) | Refusal `FROB-REF-101` reports the negative eigenvalue and the nearest valid projection | Has (C-03) | Sound |
| Log-Euclidean parametrization S = log J for every optimizer, CAD pass and system-identification step (§6.2) | Invalid inertias are unrepresentable during optimization | Has an equivalent (affine-invariant geodesics, C-03). BASE goes further: inertia is derived from geometry and material by default | Sound; either metric works |

---

## 3. Principal bundles, mechanical connection, curvature (NEON §7, crate `frob-bundle`, 4 kLOC)

BASE counterpart: C-08, A-09 (connection curvature as a design oracle, Frontier), C-07 (wheels as a distribution).

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| Bundle decomposition (§7.1) | Q = G × S with G = SE(3) or SE(2). Kinetic energy in block form with locked inertia 𝕀(r) and coupling I_BS(r). Reconstruction ξ = −A(r)·ṙ + 𝕀(r)⁻¹·p_b with A = 𝕀⁻¹·I_BS. For wheels and snakes the Pfaffian constraints W_g·ξ + W_r·ṙ = 0 give the principal kinematic connection when rank W_g = dim G, and the nonholonomic momentum connection otherwise. | Has (C-04, C-08) | Sound |
| Curvature with verified sign (§7.2, `GB-01`, `KA-03`) | DA = dA − ½[A∧A]; components (DA)_ij = ∂_i A_j − ∂_j A_i − [A_i, A_j]; g(T) = right-ordered path exponential of −∮A·dr. NEON reports error 9.7e-4 with the minus sign and 2.026 with the plus sign on a kinematic snake loop. | Has the statement (App. D "net motion ≈ −∬ DA") without the component formula or a sign test | Sound. I confirmed the sign by a second-order expansion. Adopt the component formula and the wrong-sign negative control. |
| Preflight controllability, stage 4 (< 5–8 ms) | Rank of span{DA(∂_i, ∂_j), ∇_k DA} over shape space; if curvature vanishes in yaw or forward translation, refuse with `FROB-REF-104` and a joint or roller fix. | Weaker: A-09 exists but is not in `design check` and is a ranking oracle, not a refusal | Sound for shape-changing kinematic locomotors. For wheeled bases it is the wrong test: a mecanum base with misaligned rollers loses rank in the connection itself, which BASE's growth vector (C-07) detects correctly. Use both. |
| Stratified legged bundles | One stance-pinned connection A_σ per contact stratum, with switching maps at lift-off and touchdown, feeding gait seeds. | Lacks | Plausible. BASE's caution stands: for dynamic walking this is a diagnostic, not a predictor. |

NEON has no growth vector, ball-box cost or maneuverability index (BASE C-07, A-07, GB-05). Keep BASE's.

---

## 4. Dynamics, contact, rods, actuators (NEON §8)

BASE counterpart: C-04 to C-06, §13 to §15.

### 4.1 Trapezoidal-potential Lie-group variational integrator (§8.1, `frob-dyn`)

**Mechanism.** Step g_{k+1} = g_k·τ(h·ξ_k) with τ the Cayley map. Discrete Lagrangian L_d = (h/2)·ξ_kᵀ𝕄ξ_k − (h/2)(V(g_k) + V(g_{k+1})); momentum μ_k = dτ⁻ᵀ_{−hξ_k}(𝕄ξ_k). Four sub-steps:

1. Half kick: μ⁺ = μ⁻ + (h/2)·f_pot(g_k) + (h/2)·u_k.
2. Solve dτ⁻ᵀ_{−hξ}(𝕄ξ) = μ⁺ for ξ_k by Newton with the closed-form 6×6 Jacobian (2–3 iterations to 1e-13).
3. Drift and transport: g_{k+1} = g_k·τ(hξ_k); μ_free = (Ad_{τ(hξ_k)})ᵀ μ⁺.
4. Second half kick with f_pot(g_{k+1}) and u_{k+1}.

Claimed numbers for an asymmetric free body with gravity (`KA-02`, `GB-02`): relative energy error 5.62e-5 at h = 2 ms and 8.79e-7 at h = 0.25 ms, second-order convergence, time reversal to 1e-14. `KILL-LGVI-1`: if the implicit solve costs more than 2× an explicit Lie-midpoint step on the GPU, the explicit step is locked for training and the variational step is kept for the CPU oracle, adjoints and evaluation.

- **Lever claimed:** no secular energy drift over 100 s trajectories, spinning rotors and lifecycle runs.
- **BASE:** Weaker in specificity. C-05 and §14.4 name a variational lane as Frontier at M5 on the CPU tiers and give no algorithm; the default is semi-implicit Euler with a Cayley group update.
- **Verdict:** Sound for a single rigid body (it is a symmetric kick-drift-kick composition; the two energy figures are consistent with second order). For articulated trees NEON says only "ABA/CRBA inside a symplectic Lie-Verlet/RATTLE splitting", which is not a derivation: with a configuration-dependent mass matrix an explicit splitting is not symplectic, and the property is moot once dissipative contact impulses enter. Adopt the four-step algorithm and `KA-02` as the concrete content of BASE's variational lane; keep BASE's default and its "Frontier" label.

### 4.2 Nonholonomic constraints (§8.2)

Rolling is a set of linear velocity rows A_c(q)·q̇ = b_c(q, t) inside the impulse solve, each bounded by the traction cone, so rolling passes into skidding when traction saturates: differential, Ackermann and swerve (zero lateral slip); omni and mecanum (roller-axis free projection at ±45° or 90°); tracks (prescribed belt speed b_c = v_belt(u_sprocket) on every grouser patch plus soil shear).

- **BASE:** Has the same physics by another route (§15.5: wheel bodies with friction, anisotropic ellipse for rollers, prescribed-velocity belt). The difference is exact stick against regularized friction with creep.
- **Verdict:** Sound. "Discrete Lagrange–d'Alembert" is a label for constraint rows in a cone-bounded solve.

### 4.3 Unified 4D contact (§8.3, `frob-contact`, 12 kLOC)

| # | Mechanism | Specifics | BASE | Verdict |
|---|---|---|---|---|
| 1 | Non-associated complementarity with the De Saxcé bipotential | Impulse λ = (λ_n, λ_t1, λ_t2, λ_spin) in the cone √(λ_t1² + λ_t2² + (λ_spin/r_eff)²) ≤ μ·λ_n. Modified velocity adds μ·√(v_t1² + v_t2² + (r_eff·ω_n)²) to the normal component. Stated honestly as a nonlinear complementarity problem, not one convex program. Solvers: iterated convex cone programs, projected Gauss–Seidel, proximal ADMM, semismooth Newton. Residual ≤ 1e-6 (evaluation) or 1e-4 (training). No pyramid cone, so no √2·μ diagonal bias. | Conflict. BASE's fast tiers solve one strictly convex compliant problem in generalized velocity by Newton and Cholesky (C-06, §14.3); exact Coulomb is the referee only. BASE has torsional and rolling rows and elliptic cones (§15.4) but does not couple spin into one cone. | Sound physics. Convergence on a GPU at fixed iteration counts is not guaranteed, which NEON admits (`KILL-CONTACT-1`: fall back per island to ADMM or Gauss–Seidel with a residual flag). The 4D cone is a linear image of a second-order cone and drops into BASE's `Cone` trait. |
| 2 | Joint-space nonsmooth rows | Hard stops, Coulomb/Stribeck stiction τ ∈ [−τ_s, τ_s], and gear backlash q_out − q_motor/N ∈ [−β/2, β/2] in the same system as ground contact, so backlash can grow with wear. | Weaker: friction in the actuator card at L1, backlash at L3 as Frontier (§13.3) | Sound. Adopt. |
| 3 | Tactile elastomer patches | Fingertips split into 2×2 or 4×4 compliant shear-and-torsion micro-patches feeding tactile sensors. | Lacks (NONGOAL-07) | Plausible |
| 4 | Three-tier Delassus storage | ≤ 10 contacts: packed lower triangle of a 30×30 `f32` matrix, 1.86 KB, in shared memory. 11–16 contacts: 48×48, 4.70 KB. More: matrix-free operator v ↦ J·M⁻¹·(Jᵀv) inside ADMM or Krylov, in an overflow queue. | Not applicable to BASE's primal formulation, whose Hessian is n_v × n_v | Sound arithmetic; only matters if the dual formulation is used. |
| 5 | Factor-once multi-RHS response (`W10`) | On a fixed active set, factor A = J_A·M⁻¹·J_Aᵀ + D once and solve 24–64 right-hand sides (counterfactual impulses, adjoint columns). Each column carries the bound ‖x̂ − x\*‖ ≤ ‖b − A·x̂‖ / λ_min-lower-bound and a guard that reroutes any column crossing a stick, slip or open boundary to the full solve. | Lacks | The idea is Sound and transfers to BASE: reuse the converged Newton factor for sensitivities. The tensor-core framing is Ornament (see D). |
| 6 | Barrier contact lane | C² log barrier with conservative continuous collision detection, lifted from `ftx-drape` and `fs-contact`, for thin cables, fragile grasps and zero-penetration checks. | Parked (App. C leaves it to FrankenSim) | Plausible as an evaluation-only lane |

### 4.4 Variable-strain Cosserat rods (§8.4)

Strain field ξ(s) = Φ(s)·q_rod + ξ₀(s) with Chebyshev, Legendre or piecewise-constant bases, so a rod is a generalized joint in the same O(N) recursion as rigid links. Uses: tentacle arms, sagging harnesses for the build packet, snagging appliance cords. `KA-05` (elastica cantilever within 0.5%), `GATE-COSSERAT-01`. Lifted from `fs-solid::rod`.

- **BASE:** Lacks (NONGOAL-02, deferred to M6).
- **Verdict:** Plausible. The formulation is published and fits a tree recursion. Rod-to-environment contact (the snag) is not specified, and whether `fs-solid::rod` is variable-strain is unverified.

### 4.5 Port-Hamiltonian actuators with thermodynamics (§8.5, `frob-actuators`, 6 kLOC)

| Item | Specifics | BASE | Verdict |
|---|---|---|---|
| Motor, inverter, gearbox | τ_m = K_t(T)·i_q; copper loss I²R₀[1 + α_cu(T_w − T₀)]; iron loss P_fe(ω); voltage ceiling V_bus ≥ √((R·i_q + K_e·ω)² + (ω·L·i_q)²), which gives field-weakening roll-off; Stribeck and load-dependent gear efficiency η(ω, τ). | Has most (§13.2–13.3). Lacks the temperature-dependent resistance loop and the voltage ellipse. | Sound |
| Entropy as thermal state | Winding entropy S_T as the stored variable; a Clausius–Duhem check σ_irr ≥ 0; positive feedback exponent a = (I²R₀α_cu − G_th)/C_th > 0 produces a thermal runaway "cliff" (`GB-08`, `KA-09`, failure class `FAIL-THERMAL-CLIFF`). | Weaker: two-node network with derating; port energy balance; no second-law test, no runaway analysis | The runaway feedback and a second-law test are Sound and cheap. Entropy-as-state is Ornament for one lumped node. The entropy equation as printed mixes dissipation and an extra σ_irr term and should be re-derived. The pinned 33.5 s does not follow from the stated parameters (I get 21.1 s). |
| Battery | Two-RC Thevenin model, open-circuit voltage against state of charge, R₀(T, health), aging state, brownout cutoff. | Weaker (sag and state of charge at L2) | Sound |
| Aerial rotor port | T = c_T·ρ·A·(Ωr)²·k_IGE(z/r), k_IGE = [1 − (r/4z)²]⁻¹ capped for z > 0.5r; reaction torque; induced power; quadratic fuselage drag. | Lacks (NONGOAL-03) | Sound as a lumped port |

NEON has no actuator cards with per-field provenance and no model ladder L0–L4. Keep BASE's and add NEON's physics as fields and levels.

---

## 5. Symmetry (NEON §9, crate `frob-symmetry`, 6.5 kLOC; layers in `frob-equiv`)

BASE counterpart: C-09, C-10, A-03, A-04, GB-02.

| Item | What it is | Lever claimed | BASE | Verdict |
|---|---|---|---|---|
| **Two nested groups** (§9.1, `INV-008`, `PO-EQUIV-1`) | Γ_mech: graph automorphisms preserving link lengths, pseudo-inertias, joint screws (with reflection parity), actuators, gear ratios. Γ_obs ≤ Γ_mech: the subgroup that also preserves every sensor pose and field of view, asymmetric payload, task goal orientation, and guard or terrain symmetry. Only Γ_obs may be used for weight sharing, orbit deduplication and gait orbits. Example: a quadruped with a front-only lidar is V₄ mechanically and Z₂ in closed loop. Both are stored on each robot-plus-task pair. | Prevents unsound weight tying | Weaker. C-09's group preserves masses, inertias and actuator cards; sensors, payload and task are not in the definition, though each observation term declares its representation (§19.2). | Sound and important. Adopt. |
| Frobenius–Schur typed decomposition (§9.2) | Per-space isotypic projectors P_χ = (d_χ/\|Γ\|)·Σ conj(χ(g))·ρ(g) on input and output separately, O(d³) each. Intertwiner blocks by type: real 1×1; complex 2×2 of the form [[a, −b], [b, a]]; quaternionic 4×4. Exact arithmetic in ℚ(√2), ℚ(√3), ℚ(cos 2π/n) for n ≤ 12. | Correct handling of Cₙ, n ≥ 3; avoids an O(d_in³·d_out³) projection | Weaker. BASE uses the Reynolds operator and "isotypic projection" without saying how complex-type irreps of Cₙ are parameterized. | Sound. The complex case matters for radial robots without mirror symmetry. The quaternionic case does not arise for point groups or leg permutations: Ornament. NEON's claim that Reynolds averaging "mishandles" complex pairs is wrong; it is exact but expensive. |
| Regular-representation hidden layers (§9.2) | Hidden features carry ℝ[Γ]^k, so pointwise nonlinearities and per-orbit normalization commute with the group. | No Clebsch–Gordan products | Has (GB-02) | Sound |
| Wreath products (§9.3) | Γ = K ≀ H = K^\|X\| ⋊ H when each component automorphism preserves its attachment-port invariants and the hub group preserves the decorated ports; both checked before nested tying. | ≥ 1.5× on heterogeneous-limb robots | Lacks | Sound; narrow use (limbs that are themselves symmetric). |
| **Exact orbit deduplication** (§9.3, `GB-05`) | Before a combinatorial sweep of k module types over n slots, compute the cycle index of Γ_obs and generate one canonical representative per orbit. Worked: 10 leg types on 6 dihedral slots, (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = 86,185, an 11.60× reduction; with only a sagittal mirror, (10⁶ + 10³)/2 = 500,500, 2.00×. | Up to \|Γ_obs\|× fewer simulations | Lacks | Sound. Cheap, exact, and honest about the small factor when sensors break symmetry. Adopt. |
| H/K gait enumeration | All admissible (H, K) pairs of Γ_obs, checked by `decide` in the proof kernel (`PO-GAITENUM-1`). | Gait seeds for any leg count | Has (C-10, A-04) without the kernel check | Sound; the kernel check is optional |
| Contact strata count (§13.3, `GB-11`) | Four feet × {open, stick, slip} under V₄: (3⁴ + 3·3²)/4 = 27 strata and 162 one-foot transitions; binary model 7 and 16. | Sizes the safety lemmas | Lacks | Sound counting |

BASE items NEON lacks: the group is computed from the design term with `mirror` and `radial` constructors and typed edits that report their symmetry effect; max-plus gait timing; symmetry as a metamorphic test of the whole simulator.

---

## 6. Operadic composition and mission quotients (NEON §10, crate `frob-operad`, 6.5 kLOC)

BASE counterpart: C-12, §12.3 (five functors, per-subterm memoization, INV-SKEL-02).

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| `.fmodule` as a decorated cospan (§10.1, `FMT-FMOD-1`) | A reusable subassembly is an open port-Hamiltonian module X → N ← Y over typed port sets. Ports: `MechanicalSe3` (wrench, twist, power positive into the module), `ElectricalDc` (V, I), `ThermalEntropic` (T, Ṡ), `LogicalBus` (frame rate and latency bound). Composition is the cospan pushout with Dirac interconnection (`fs-phs::interconnect_mapped`), power sum Σ e_p·f_p = 0, algebraic loops rejected (`FROB-REF-108`). Modules are files referenced by content hash: `"std/wheel_leg_3dof.fmodule@sha256:9a4e…"`. | Has the idea (typed ports, Dirac structure, operad of wiring diagrams). Lacks a module file format and library, the bus port and loop rejection. | Sound. "Decorated cospan" and "operad" name the same structure; neither adds mechanism beyond the pushout rule. |
| **Proof and invalidation contract** | Each module carries a summary digest over its port signature, local inertia proofs, local passivity certificate and boundary descriptor. Swapping a leaf preserves siblings' proofs, derivations and kernels (target ≥ 10× edit-derivation speed). Edits touching a shared battery bus, a shared thermal sink, a cross-branch kinematic loop or base-coupled contact invalidate global certificates. `GATE-OPERAD-01` tests a mirror swap (must reuse) against a shared-bus edit (must invalidate) and refuses a reversed port sign. | Weaker: memoization is specified, invalidation semantics are not | Sound. Adopt the explicit rule and both control tests. |
| Limb port interface ("tier 0.5") | A tree limb attached at a port contracts to its articulated inertia and bias wrench: 21 + 6 = 27 floats, 108 bytes. After eliminating a joint axis s the update I − I·s·(sᵀI·s)⁻¹·sᵀI is positive **semi**definite (example diag(0,1,1,1,1,1)), so the type is `SymPsd6`, and multi-limb ground contact still goes through the coupled base solve. | Lacks | Sound. A precise statement of what can and cannot be cached per limb. |
| **Mission quotient `.fquot`** (§10.2, `FMT-FQUOT-1`, `PO-QUOT-1`) | Compile the projection π_M from designs to exactly the invariants that enter the mission's equations: total pseudo-inertia, collision hull, joint and actuator parameters, port coordinates, sensor visibility, thermal and electrical bus, task objective. Two designs share a class only when their projections are equal over all active observables; adding a placement-dependent hotspot or harness length enlarges the projection and splits classes. Fixture (`GB-06`): 8 ballast slugs in 24 collinear slots, C(24,8) = 735,471 placements, which enter rigid-body dynamics only through (N, Σx, Σx²), giving 24,546 classes (29.96×); two placements with equal Σx and different Σx² must split. | Lacks | The equivalence is Sound and I confirmed the count. Practically it is a digest of the compiled model restricted to what the mission reads, with results memoized on it. The 29.96× is a property of this fixture. The proof obligation "equal projections imply identical trajectories" is a statement about the simulator and cannot be discharged in a proof kernel; make it a structural rule with a bit-identical replay test. |

BASE items NEON lacks: the budget functor and monotone co-design with a Kleene fixed point (the mass spiral, GB-03, A-10). NEON's nearest substitute is a convex geometric-program bound in preflight stage 6.

---

## 7. Linkage synthesis and homotopy continuation (NEON §11)

BASE counterpart: C-11 (closed-form leg IK, certified IK by interval Newton); App. C parks Study-quadric synthesis and rejects general 6R by Gröbner bases.

**Lane 1, exact and bounded (M1, `frob-symbolic::linkage_synth`).**

1. Three-pose planar dyads: exact 2×2 circumcenter systems over ℚ, degenerate cases split by determinant rank.
2. Five-pose Burmester and planar four- and five-bar templates: Study-quadric or loop-closure polynomials inside `real_rur`'s limits (≤ 8 variables, ≤ 64 equations, degree ≤ 256, ≤ 4,096 bits), with Rabinowitsch saturation s·d(x) − 1 = 0 to exclude zero-length and collinear roots.
3. Branch, singularity and clearance certificate: real roots isolated by interval contraction (`fs-ivl`); one assembly circuit must traverse all target poses with det J_loop ≠ 0 and no self-collision. Gate `GATE-BURMESTER-01`; target < 100 ms.

**Lane 2, continuation (M3–M4, new crate `frob-homotopy`, 10 kLOC).**

1. A generic start system per mechanism family, solved once by polyhedral (mixed-volume) homotopy and cached in the ledger.
2. Parameter homotopy H(z, t) = F(z; (1−t)·p₀ + t·p₁) tracked in parallel on the GPU with an adaptive predictor–corrector, Cauchy endgames, and double-float or arbitrary-precision recovery of ill-conditioned paths. Target ≤ 2 s per new task.
3. Certification by Smale's α-theory, α(f, z) < (13 − 3√17)/4 ≈ 0.15767: existence of a unique root within 2β, distinctness by disjoint balls, reality by conjugate-ball disjointness; also replayed in the proof kernel (`PO-ALPHA-1`).
4. Labels: `Complete` only when the certified count equals an independent generic count (trace test) with no lost paths or positive-dimensional components; otherwise `Partial(certified_n, lost_n)`.

Known answers (`KA-13`): 4,326 nine-point four-bars, 40 Stewart–Gough roots, 16 for spatial 6R, ≤ 4 Burmester points. Kill: > 0.1% lost paths or a missed count falls back to lane 1 plus multi-start CPU recovery.

- **Lever claimed:** constructive design of parallel legs, ankles, steering linkages and delta arms instead of random mutation.
- **BASE:** Lacks synthesis entirely; has `closure` as a constructor and mobility analysis (A-01).
- **Verdict:** Lane 1 is Sound and fits the tool limits I confirmed. The circuit-and-branch certificate is the valuable part and is a real defect class in linkage design. Lane 2 is real mathematics with reference implementations elsewhere, and Plausible as a Frontier crate, with three reservations: a polyhedral start system for the nine-point problem is a very large one-time computation (monodromy with a trace test is the lighter route and NEON does not mention it); path tracking wants `f64`, so the GPU is not the natural home; replaying α-certificates for thousands of 8-variable roots inside a proof kernel in 0.5 s has no basis. The statement that 99.9% of random four-bar mutations break closure is unsupported.

---

## 8. Symbolic compiler, kernel IR, four targets (NEON §12)

BASE counterpart: C-14, §20.2–20.3 (`frob-kir`, `derive/frob-derive`, shared dialect).

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| Symbolic robotics algebra (`frob-symbolic`, 9 kLOC) | Product-of-exponentials kinematics, geometric Jacobians, RNEA, CRBA, ABA derived with frankensympy; the Echeandia–Wensing Coriolis factorization for which Ṁ − 2C is skew as a polynomial identity (`PO-SKEW-1`), used by energy-shaping control; symbolic capability maps w = w₀(q, q̇) + B_m(q)·u. | Has the derivation idea; tests skewness numerically (C-04). BASE derives per kernel body and keeps loops in hand-written skeletons, which avoids whole-robot expression swell (NEON `RISK-02`). | Sound. BASE's granularity is safer. |
| `SubprocessSymbolicBackend` (`KILL-FSYM-1`) | If frankensympy fails admission or hangs on a 29-joint derivation, run it in an isolated worker with timeout and memory cap, plus a Rust-native generator. | Has an equivalent (a build-time annex with its own toolchain) | Sound |
| **`FMT-KIR-1` grammar** (`frob-kernel-ir`, 5.5 kLOC) | Typed SSA. Types: `f32`, `f64`, `f32dd`, `q16_16`, `q32_32`, `i32`, `u32`, `bool`, `vec3<>`, `mat3<>`, `se3<>`, `sym6<>`, `array<T, N>`. Statements: single assignment; `for` with constant bounds only; `if_pred` as a branchless select. Expressions: Add, Sub, Mul, Div, Fma, Min, Max, Neg, Sqrt, Rsqrt, Sinc, Cosc, Sin, Cos, Abs, `load_soa`/`store_soa` on an arena slot with lane and constant offset, `clip`. Metadata: BLAKE3 hash, `math_mode` ∈ {`IEEE754Safe`, `FastApprox`, `FixedPoint`}, `max_regs`, `max_stack_bytes`, optional `proof_digest`. Optimizations: bounded CSE, Horner and FMA fusion, live-range scheduling to cap register pressure. | Weaker in two ways (no published grammar; no fixed-point types, bounded loops or resource metadata) and stronger in three (every value carries a physical dimension, T-09; derivative programs come from the IR; no transcendental built-ins and no contraction). | Sound as a format. Its Sin/Cos ops, FMA fusion and `FastApprox` mode conflict with BASE's cross-backend determinism rules; see (C). |
| Four-target lowering (`frob-codegen`, 9.5 kLOC) | (1) safe Rust CPU, the `f64` oracle and SIMD `f32` twin; (2) Metal Shading Language 3.2, safe math mandatory for double-float; (3) CUDA C or PTX for `sm_89`, with `--fmad=false` for double-float audits; (4) zero-allocation `#![no_std]` Rust in Q16.16, Q32.32 or `f32` for `thumbv7em-none-eabihf` and `riscv32imac`, with static stack annotation behind a `MotorBusHal` trait. | Has three targets by a different route (derived arithmetic into Rust and one shared C-dialect text for both GPUs). Lacks the embedded target (NONGOAL-04). | Plausible. BASE's shared-dialect route is already proven in the suite (`render-gpu-piano`). The fourth target is the new idea. |

---

## 9. Proof layer (NEON §13, crates `frob-proof` 8.5 kLOC, `frob-shield` 7.5 kLOC)

BASE counterpart: §10 (Lemma), proof classes, theorem cards T-01 to T-14.

### 9.1 Kernel reflection

**Mechanism.** Because the tactic layer is mostly stubs, NEON never calls it. It ships `FrobMath`, a library of boolean checkers `check_*: Input → Bool` each with a soundness theorem `check(x) = true → Spec(x)`, "proved once at M1" inside franken_lean's environment and replayed offline by `fln-checker`. An instance proof is then a kernel evaluation of the checker. Four reflectors:

1. Integer-scaled exact rational LDLᵀ: positive-definiteness of pseudo-inertias and of rational sum-of-squares Gram matrices.
2. Polynomial identity by normalized sparse multivariate subtraction over ℤ: Coriolis skewness, port power conservation, symmetry intertwining over ℚ(√2), ℚ(√3).
3. Linear-size SSA congruence trace: the code generator emits its rewrite trace (associativity, commutativity, distributivity, CSE), checked in O(|G|) reductions. It proves equivalence over ℝ; rounding is bounded separately by intervals.
4. Finite-group `decide` and fixed-point interval checks, plus `omega` for integer-linear bounds.

**Proof obligations (§13.2), with NEON's time budgets.**

| ID | Property | Budget | Milestone |
|---|---|---|---|
| PO-LIE-1 | Conventions of §6.0 (Ad, ad\*, curvature sign, reflection parity) | 0.2 s | M1 |
| PO-INERTIA-1 | Every pseudo-inertia is positive definite | 0.05 s | M1 |
| PO-SKEW-1 | Ṁ − 2C + (Ṁ − 2C)ᵀ = 0 identically | 0.8 s | M1 |
| PO-EQUIV-1 | Γ_obs ≤ Γ_mech commutes with dynamics, sensors, task; wreath port invariants | 0.5 s | M1, M4 |
| PO-OPERAD-1 | Wiring conserves port power; no algebraic loop | 0.2 s | M1 |
| PO-QUOT-1 | Equal mission projections imply identical trajectories | 0.2 s | M2 |
| PO-CODEGEN-1 | IR lowering is equivalent over ℝ | 1.5 s | M1, M2 |
| PO-CODEGEN-FW-1 | Fixed-point firmware lowering matches the IR, no overflow | 2.0 s | M4 |
| PO-ALPHA-1 | α-certificates, ball disjointness, reality | 0.5 s | M3 |
| PO-GAITENUM-1 | Complete H/K lattice of Γ_obs | 0.3 s | M4 |
| PO-ENERGY-1 | `.fenergy` inequality | 0.05 s | M2, M4 |
| PO-TUBE-1 | `.ftube` transition inclusion for all enabled guards | 1.5 s | M4 |
| PO-SHIELD-1 | ∃u ∈ U ∀(w, e): ḣ + α(h) ≥ L_h·Δt | 4.0 s | M4 |
| PO-OBSALIAS-1 | Two admitted worlds share a transcript and have disjoint safe-action boxes | 0.5 s | M3 |

- **Lever claimed:** proofs at interactive latency with a trusted base of about 12 kLOC and no dependence on unbuilt tactics. `INV-011` forbids `sorry`, axioms and stub calls.
- **BASE:** Different and more cautious. Instance identities are `Certified` by frankensympy's proof kernel; Lean is a second opinion on theorems written in stock Lean with Mathlib and re-checked by franken_lean; analytic statements wait (§10.1–10.5).
- **Verdict:** Reflection is a Sound and standard technique, and it is the one route by which instance certificates could be raised from `Certified` to `Proved`. As planned it is Doubtful on three points. NEON never says how the soundness theorems are authored when there is no tactic layer and (BASE §10.1) franken_lean cannot elaborate tactic proofs; the workable route is the one BASE already describes (write them in stock Lean, re-check the compiled result). Every budget is a guess about kernel-reduction speed on big rationals. And PO-QUOT-1, PO-CODEGEN-FW-1 and PO-SHIELD-1 state properties of a simulator, a compiler and a 29-joint closed loop that these checkers cannot express. `KILL-PROOF-1` narrows to PO-INERTIA, PO-OPERAD, PO-ENERGY and PO-OBSALIAS, which is the realistic core.

### 9.2 Three safety tiers (§13.3)

| Tier | Mechanism | Domain | BASE | Verdict |
|---|---|---|---|---|
| **1. `.fenergy`, observable boundary energy budget** (`FMT-FENERGY-1`, `KA-14`, `GB-07`) | E_deliv(t) = ½·m·v² + ∫ over the cutoff delay of max(P_elec, 0) + E_stored + E_reserve ≤ W_abs,min. Passive contact with any μ ≥ 0 can only dissipate or redistribute energy, so the contact sequence drops out. One drop-weight test of the absorber and one drive power test calibrate it for every later policy. Worked: 8 kg at 0.4 m/s is 0.64 J, plus 20 W × 25 ms = 0.50 J, plus 0.25 J stored and 0.10 J reserve: 1.49 J ≤ 50 N × 40 mm = 2.00 J; cutoff 50.5 ms (45.5 ms with reserve). The monitor runs in < 5 µs. | Stops, bumpers, doors, drawers, lifts, transient human contact (ISO/TS 15066) | Lacks. BASE's energy ledger (§23.4) already measures every term. | Sound, and the best safety idea in either plan. The bound must include every source on the load path (gravity over the stroke, springs, other actuators). The "validated" label needs bench data a simulator cannot produce. |
| 2. `.ftube`, hybrid reachable tubes (`FMT-FTUBE-1`) | Backward viability pruning on cell complexes of ≤ 8 modes × 16,384 cells (16 MiB) for subsystems of ≤ 4 reduced states with bounded sensing error E: a common safe action with R_mn((K_m ⊕ E) ∩ G_mn, D) ⊆ K_n for all simultaneously enabled guards, accounting for inter-sample sweep, latency, chatter exclusion and a backup domain. | Threshold traversal, tray tipping, braking on mixed tiles, door pinch | Lacks | Plausible. Sound as abstraction-based reachability; the reduction from the full robot to four states is an assumption to be monitored, not proved. |
| 3. Box-constrained fused shields | (a) Quantifier order ∃u ∀(w, e), never the clairvoyant reverse; commanded reaction forces in the inner cone K(μ_min), environment forces in the outer cone K(μ_max). (b) Projection of the policy action onto barrier rows A·u ≥ b + δ_Lip·Δt within actuator boxes: one row with a diagonal metric is solved exactly by sorting ≤ 2n breakpoints of u(λ) = clip(u_π + λ·R⁻¹a, lo, hi) (example: u_π = (1, 0), box [0,1]², u₁ + u₂ ≥ 1.5 gives (1, 0.5), where the unboxed formula gives the infeasible (1.25, 0.25)); two to four rows use a fixed-iteration projected Newton step with a mandatory KKT check, and any failure invokes a certified passive braking controller. (c) Offline lemmas per contact stratum (27 strata, 162 transitions, "10–13 minutes" of SOS synthesis). (d) Runtime assumption monitors give P(violation per episode) ≤ E[N_breach]·δ₁ + E[N_alarm]·δ₂ + δ₃ with δ₁ = P(silent \| breach) calibrated on independent ground-truth labels. Result typed `ClaimProduct`. | High-degree-of-freedom exploration | Lacks (C-13 mentions SOS and barrier certificates as Frontier; T-14) | The projection with its KKT check and backup (a, b) is Sound and useful as a runtime filter. The formal lemmas (c) are Doubtful: SOS certificates for full articulated dynamics are not tractable at that size and the time estimate has no basis. Adopt (a), (b), (d) with class `Tested`. |

---

## 10. Morphology grammar, archetypes, preflight (NEON §14)

BASE counterpart: §12 (design term, typed edits, compile pipeline, templates), §22.2 (`design check`).

### 10.1 `.fbot` grammar (`FMT-FBOT-1`, crate `frob-bot`, 9.5 kLOC)

One typed, unit-checked syntax tree with lossless S-expression and JSON views; schema string `frob.bot/v1`. Top-level forms:

| Form | Content |
|---|---|
| `(use-module id :from "path@sha256:…" :mount se3 :rename map)` | Import of an `.fmodule`, with `:override` of actuators, materials, contact law |
| `(symmetry :mech "V4" :obs "Z2_sagittal")` | Declared, then verified |
| `(base …)`, `(link …)` | `:geometry`, optional `:shell-thickness`, `:material`, optional `:pseudo-inertia`, optional `:gvs-rod` |
| `(joint …)` | `:type` revolute, prismatic, helical, spherical, fixed, free; parent, child, mount, axis, limits, actuator; optional `:stiction`, `:backlash`, `:contact-law` |
| loop constraint | Closed kinematic loops |
| `(mirror-limb new :from limb :planes (sagittal coronal) :mount se3 :mirror-axes true)` | Expands to explicit mounts and parity-corrected axes |
| `(actuator …)`, `(power-bus id :battery spec :v-nom :i-max :thermal-sink)` | Shared electrical and thermal resources are first-class |
| `(sensor id :type :parent :mount :rate)` | |
| `(levers (name :range (lo hi) :step s :target path) …)` | The only parameters co-design may touch; out-of-range mutation is `FROB-REF-110` |
| `(budgets :mass-max :bom-max-usd :power-max :width-max :height-max)` | |
| capability form, safety form | Named in the grammar, not elaborated |
| `Qty` | Number plus unit token: m, mm, cm, kg, g, s, ms, rad, deg, N, Nm, W, V, A, Ah, Ohm, H, degC, K, Pa, kg/m3, Hz, sqrtHz, N/m, Ns/m, J/K, USD |

Four corrections NEON builds into the grammar: (1) **hollow shells**: a box or cylinder with `:shell-thickness 0.0025m` in 6061 aluminium weighs about 3.1 kg where the solid default would be 56 kg; a solid fill raises a density warning (`FROB-REF-102`); (2) mirroring expands to explicit transforms and computes Γ_mech and Γ_obs separately; (3) a **typed contact law per end effector**: `WheelContact :pfaffian true :mu-nom 0.75`, `ContensouFoot`, `PrescribedBelt`, `TactileElastomerPatch(4x4)`; (4) declared levers only.

Worked example, condensed from NEON Appendix D.1:

```lisp
(robot "kitchen_wheelquad_v2" :schema "frob.bot/v1"
  (symmetry :mech "V4" :obs "Z2_sagittal")
  (power-bus main-bus :battery (li-ion-2rc :v-nom 48V :cap 20Ah :r0 0.04Ohm) :i-max 80A :thermal-sink 1.2W/K)
  (budgets :mass-max 28kg :bom-max-usd 4800 :power-max 1800W :width-max 0.58m :height-max 0.72m)
  (actuator pmsm-knee :kt 0.14Nm/A :r0 0.54Ohm :alpha-cu 0.00393/K :cth 79J/K :gth 0.85W/K :gear 12.0 :backlash 0.003rad)
  (base chassis :geometry (box 0.52m 0.34m 0.14m) :shell-thickness 0.0025m :material al-6061-t6)
  (use-module leg-fl :from "std/wheel_leg_3dof.fmodule@sha256:9a4e..." :mount (se3 :xyz (0.22m 0.17m -0.04m))
              :override (:knee-actuator pmsm-knee :contact-law (WheelContact :pfaffian true :mu-nom 0.72)))
  (mirror-limb leg-fr :from leg-fl :planes (sagittal) :mount (se3 :xyz (0.22m -0.17m -0.04m)) :mirror-axes true)
  (sensor front-lidar :type lidar-3d :parent chassis :mount (se3 :xyz (0.26m 0m 0.12m)) :rate 10Hz)
  (levers (W_stance :range (0.30m 0.46m) :target (chassis.width))
          (N_knee :range (8.0 20.0) :target (pmsm-knee.gear))
          (lidar_mount_z :range (0.08m 0.28m) :target (front-lidar.mount.z))))
```

- **BASE:** Has a design term in canonical JSON (`frob.design/1`) with constructors (`attach`, `serial`, `mirror`, `radial`, `scale`, `replace`, `closure`), parameters with unit, bounds, manifold kind and provenance, typed edits with a declared symmetry effect, incremental compilation, import loss reports. BASE lacks: shell thickness (its `Body` is a Region with a material, so a box is solid), the power bus, the budgets block with a cost in dollars, per-end-effector contact laws, module imports by hash, a text syntax (OPEN-SKEL-1 defaults to JSON only). NEON lacks typed edits, lineage, provenance per parameter, loss reports and the `radial` constructor; it offers only a "semantic structural diff".
- **Verdict:** Sound. The shell-thickness rule fixes a real mass error that BASE's geometry-derived inertia would otherwise make.

### 10.2 The eighteen archetypes (§14.2)

| # | ID | Signature NEON gives | Milestone |
|---|---|---|---|
| 1 | `quadruped-dog` | V₄ mechanical, Z₂ observed; 27 contact strata; trot, bound, pace, gallop atlas | M1, M4 |
| 2 | `hexapod-spider` | D₆ (or C₆, complex irreps); 12× weight cut; tripod and ripple gaits with fault tolerance | M4 |
| 3 | `octopod-spider` | D₈; dense contact islands | M4 |
| 4 | `biped-walker` | Z₂; capture point; impact sensitivity | M3, M4 |
| 5 | `humanoid-dexterous` | 29 joints plus optional 16-joint tactile hands; parallel 2-DoF ankles from linkage synthesis | M4, M5 |
| 6 | `diff-drive-amr` | Nonholonomic wheels; energy-budget bump stops; doorway complexes | M1, M2 |
| 7 | `ackermann-ugv` | Steering linkage closure; outdoor soil | M2, M3 |
| 8 | `swerve-omni` | Four steer-and-drive modules (wreath structure); scrub torque from the 4D cone | M3 |
| 9 | `mecanum-holonomic` | ±45° roller projections; roller-angle bug caught in preflight | M2 |
| 10 | `tracked-skid-rover` | Prescribed belt plus soil shear | M3 |
| 11 | `tracked-flipper-crawler` | Articulated flippers; stair nosing; thermal cliff tests | M3, M5 |
| 12 | `wheeled-legged-quad` | Rolling plus step-over; mode-switching skill transfer | M3, M5 |
| 13 | `wheeled-humanoid-torso` | Mobile base with two 7-joint arms; tray and shelf work | M4, M5 |
| 14 | `mobile-dual-arm` | Omnidirectional base with bimanual closed-loop grasp | M5 |
| 15 | `parallel-delta-arm` | Closed chains from synthesis; pick and place | M3 |
| 16 | `aerial-terrestrial-quad` | Rotor ports with ground effect on a legged or wheeled chassis | M5 |
| 17 | `gvs-cosserat-tentacle` | Variable-strain rod on a mobile base | M4 |
| 18 | `modular-operadic-custom` | Any agent-composed module topology | M2, M5 |

- **BASE:** Has 14 templates (§12.6) and, unlike NEON, a list of reference robots with licences (Go1, Go2, A1, ANYmal, Spot, G1, H1, Cassie, Stretch, TIAGo, iiwa). New in NEON: swerve, dexterous hands, the two dual-arm bases, the delta arm, the aerial hybrid, the tentacle and the open modular class. Absent in NEON: tri-wheel stair cluster, rocker-bogie, three-wheel omni.

### 10.3 Eight-stage preflight (§14.3, crate `frob-preflight`, 8 kLOC)

Input is the robot **plus the task plus the world**. Each failure is a refusal with ranked repairs or a separating witness.

| Stage | Check | Target | Refusal |
|---|---|---|---|
| 1 | Schema, unit algebra, macro expansion, shell-mass check | 2 ms | 100, 102, 110 |
| 2 | Pseudo-inertia realizability by exact LDLᵀ | 3 ms | 101, with the negative eigenvalue and nearest valid matrix |
| 3 | Loop assembly and singularity audit (exact solver, interval branch check) | 25 ms | 103, with the nearest valid linkage branch |
| 4 | Connection curvature controllability | 8 ms | 104, with a roller or joint fix |
| 5 | Γ_mech ≥ Γ_obs and port power conservation | 10 ms | 108: algebraic loop, port sign mismatch, or an undeclared symmetry-breaking sensor |
| 6 | Static torque, thermal ceiling, convex geometric-program bounds on verified posynomial sub-problems, plus interval outer exclusion boxes | 15 ms | 105, with a gear-ratio or mass repair vector |
| 7 | **Clearance complex** and navigation envelope against the world | 40 ms | 109: the exact bottleneck doorway or aisle, plus a retrofit remedy if one exists |
| 8 | **Observation-alias impossibility witness** | 95 ms to 2 s | 111: two admitted hidden-world prefixes with identical quantized transcripts and disjoint safe-action sets, plus the cheapest separating sensor, sample rate or probe |

Gate: ≥ 90% of 100 seeded defects caught in < 200 ms with no false admission (stages 1–7 at M1, stage 8 at M3).

- **Lever claimed:** no GPU minute is spent on a design that cannot work.
- **BASE:** `design check` in under 2 s runs A-01 mobility, A-02 mass, A-03 symmetry, A-07 maneuverability, A-08 static stability, A-10 co-design closure, A-11 actuator envelope, A-13 self-collision, with nine more analyses as separate verbs. BASE lacks stages 7 and 8 and the world-and-task-aware framing. NEON lacks static stability, the mass spiral, modes, workspace, structural margin and sensitivity.
- **Verdict:** Sound. **Stage 8 is a genuinely new kind of check**: if two worlds the mission admits produce the same observations yet require disjoint actions, no policy can succeed, and the witness names the missing sensor. The millisecond targets are arbitrary; BASE's two-second budget is the honest one.

---

## 11. Living multiphysics worlds (NEON §15)

BASE counterpart: §16 (scene packs, compiler, terrain and interior generators, ingestion, splits), §15.6 (terrain mechanics).

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| Three-tier `WorldScene` (§15.1, `frob-world`, 9 kLOC, `INV-005`) | Tier 0, static and read-only: multi-resolution BVH, sparse VDB distance tiles, per-surface physical and optical material identifiers (roughness, complex refractive index, retroreflectance, base friction, emissivity). Tier 1, fixtures: hinged doors with closer torque, latch force and stop angle; sliding glass doors; drawers; elevators; conveyors; saloon doors; scripted people and carts. Tier 2: props and living fields (film tiles, cords). One immutable-per-epoch scene serves contact, soil, film, ray sensors, clearance and rendering. | Has scene packs with distance bricks, interface cards, fixtures as articulated models, moving surfaces, route graphs. BASE lacks optical material properties and the "one truth" invariant in that form; it deliberately keeps a visual mesh that physics never reads. | Sound. Adopt the optical properties and fixture parameters. |
| Memory claim | About 140 MB per building in private storage with lossless compression, so 500–1,000 resident worlds on the Mac. | BASE's arithmetic (§16.8) is 75 MB to 600 MB per scene | Doubtful as stated: lossless GPU compression of general buffers is asserted, not measured. |
| **Causal thin-film friction** (§15.2, tier A, `frob-multiphys`, 8.5 kLOC) | A 2.5D finite-volume film: thickness h(x, y, t) and viscosity η per cell; conservative face fluxes F_f = 1_f·h_up·(v̄·n)_f·A_f with the contact-support mask inside the divergence; a wheel- or sole-borne reservoir V_tire whose pickup and redeposition cancel exactly; global mass residual audited with a compensated sum. Friction from a Hersey–Stribeck number Π = η·v/(p·L_c) through μ(h, Π) into the contact cone. One sub-cycle per step, target ≤ 15% overhead. `KA-12`: Huppert spreading R ∝ t^(1/8) within 2%, mass drift < 1e-6. | Lacks. BASE has static wet and greasy patches in the interface map. | Plausible. The numerics are Sound; the constitutive law is semi-empirical and must stay `Estimated`. A robot that spreads the hazard it then meets is a real failure mode no incumbent models. |
| Slosh | Three-mode Lamb–Housner equivalent mechanical model for carried pots and trays; `KA-11` within 3%. | Lacks | Sound, cheap. Adopt. |
| Snagging cords | 2–4 active variable-strain cords per room. | Lacks | Plausible (contact unspecified) |
| Tier B, evaluation zoom | Full `fs-lbm` with Mach < 0.20 and relaxation > 0.55 on ≤ 64 worlds (about 78 GB on the Mac); `ftx-drape` implicit cloth for rugs; barrier contact for fragile grasps; far-field tile caching where dirty work ≤ 4.21%. Triggered only for finalists, counterexample verification and calibration of tier A. `GATE-MULTI-01`: tier A and tier B agree on design rankings. | Lacks. The pattern (cheap tier audited by an expensive tier) is BASE's trust ladder. | Plausible for cloth; Doubtful value for lattice-Boltzmann, a heavy dependency for thin spills. |
| Tier C | Bench data (spill-pad traction, drop-weight absorber, wear logs) raises material cards from `Estimated` to validated. | Has the Reality lane (§30.7) | Sound |
| **Clearance complexes `.fclear`** (§15.3, `FMT-FCLEAR-1`, `GB-12`) | Two cubical complexes in SE(2) = (x, y, ψ) per fixture mode: 160 × 160 × 48 = 1,228,800 cells, 4.69 MiB. A connected start-to-goal path in the **inner** complex is a feasibility witness for every robot whose swept footprint fits envelope w; disconnection in the **outer** complex is an impossibility certificate, with the critical doorway or corridor width named by degree-zero persistence over the width filtration (reused across 160 width queries, claimed 14.69× saving). Stated refinement band 74 mm. `MorphologyNavEnvelope` adds overhead clearance and step-over or stair-riser capability. Gate: no false route, no false impossibility on six interiors. | Weaker: route graphs, a traversable-route check for a nominal footprint (GATE-HAB-02), door and aisle passability in A-07 | Sound. Two-sided answers with a named bottleneck. The 74 mm figure is internally inconsistent (see D). |
| Six interiors plus outdoor (§15.4, `frob-worldgen` 11.5 kLOC, `frob-terramech` 5.5 kLOC) | Hazards per archetype. Homes: 76–82 cm doors, 12–22 mm thresholds, curling rugs, cords, hardwood-to-carpet friction steps, stairs, dim halls. Offices: full-height glass (lidar and depth multipath), swivel chairs, closers ≤ 22 N·m, elevators, glossy tile. Warehouses: repetitive aisles (localization aliasing), rack beams, dock plates, shrink-wrap, forklifts, dust. Plants: oil and coolant films, cable trays, metal grates (twist and snag), conveyors, glare, vibration, collaborative zones. Retail: glass freezer aisles with condensation, tight endcaps, spills, fragile stock, shoppers. Kitchens: grease and water films from fryers and dishwashers, steam, saloon doors, hot-pot slosh, 90 cm cook lines, drain grates. Outdoor: layered distance fields with overhangs, rubble, soil by Bekker–Wong and Janosi–Hanamoto. | Has seven archetypes as attribute grammars with licensed assets (§16.4–16.5). NEON's lists add perception hazards and specific dimensions; BASE adds apartments and the asset and licence plan. | Sound |
| Proxy-first delivery | Each interior ships a lightweight structural proxy at M3 with exact functional clearances, doors, ramps, spills, stairs, lighting and glass; rich variations wait for M5. | Similar (box-and-fixture level is [S], asset-populated [F]) | Sound |
| Scan ingestion with uncertainty (§15.5, `BET-16`) | Per-voxel geometric variance and coverage masks are kept; unobserved surfaces get `MaterialAuthority::UnprobedConservative` with wide priors fed to adversarial search; when occlusion uncertainty costs more than 15% in bill of materials, the studio emits a **re-scan or material-probe request** with the 3D box to capture. | Weaker: unknown-region policy and `estimated` scale labels (§16.6); no active request | Sound in design; whether the reconstruction pipeline exports those masks is untested (`INT-FDGR-001`). |
| **Robot–building retrofit co-design** (`B3`, `GB-16`, `frob-worldgen::amend`, `frob-redteam::amend`) | A typed catalog of about 40 building amendments (threshold ramps, door-closer adjustment, magnetic door holders, cord raceways, rug grippers, optical fiducials, motion lights, grab handles, bump-stop absorbers), each with cost in dollars, install class (`renter-safe`, `landlord`, `contractor`), code-legality constraints and a scene patch. On a minimal counterexample or a clearance refusal, search all kits of ≤ 3 items (40 + 780 + 9,880 = 10,700) **with the robot's policy frozen**. Output: "candidate fails doorway D; one $12 renter-safe 1:12 ramp restores solvability". | Lacks | Sound and original: the building is a design variable. Cheap once worlds are editable. The gate's "physical bench validation" is outside a simulator. |

BASE items NEON lacks: scene splits enforced mechanically (§16.7), licence manifests per asset, privacy inheritance for scanned places (§29.3), manual trilinear sampling for determinism.

---

## 12. Sensors, calibration, estimation (NEON §16)

BASE counterpart: §17 (Sensorium), §18.2 (invariant filter).

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| Ray-traced sensors on the physics scene (§16.1, `frob-sensors`, 9 kLOC) | Lidar: per-ray rolling-shutter motion skew interpolated along the body twist over the scan period, glass and mirror multipath, retroreflector bloom, steam and dust attenuation. Depth and stereo: structured-light dropout on glass, chrome and sunlit floors, lens distortion, rolling shutter, motion blur. Event cameras: asynchronous log-brightness change streams. IMU: Allan-variance noise, bias walk, scale and cross-axis error, saturation on impacts. Encoders: quantization and index offset. Bus voltage, phase current, winding temperature. Tactile 4×4 shear and torsion patches. | Has depth, lidar patterns, height scans, labels, range noise, grazing dropout, IMU with bias and latency (§17.2–17.3), with sensors in the physics determinism class and a noninterference gate. Lacks optical failure modes, event cameras, tactile, saturation. | Sound |
| "Cellular-sheaf" extrinsic calibration (§16.2, `KA-10`) | The sensor graph (V, E) with stalks 𝔰𝔢(3) and restriction maps Ad_{T_ve} from pairwise hand-eye, IMU–camera and lidar–odometry residuals r_e. Solving L₀·δ = δ₀ᵀW·r gives the Gauss–Newton extrinsic correction; the residual ‖P_{im(δ₀)⊥}·r‖_W isolates inconsistent cycles and localizes a bumped bracket. Known answer: recover a planted 2°, 15 mm lidar bump with 100% localization. | Parked in App. C ("the invariant filter covers the need") | The mechanism is Sound and useful: weighted least squares on a pose graph plus a cycle-consistency test. The sheaf vocabulary is Ornament. Localization needs enough independent cycles through each edge, which NEON does not state. |
| "Imperfect" right-invariant filter on SE₂₊ₘ(3) (§16.3, `frob-estimate`, 6.5 kLOC, `KA-06`) | Rotation, velocity, position and m stance-foot or landmark positions, with gyro and accelerometer biases; labelled imperfect because bias coupling breaks exact state-independence. Known answer: ≤ 2% consistency error under 45° initial yaw error. | Has (C-13, §18.2) | Sound; the honest label is worth copying |
| Observability Gramian | Empirical local Gramian along trajectories; its smallest singular value flags feature-starved corridors and repetitive aisles, raises shield assumption warnings, and feeds stage 8 and sensor placement. | Lacks (A-14 sensor coverage is ray geometry only) | Sound. Adopt as an analysis. |

BASE items NEON lacks: evaluation under estimated against privileged state with the gap reported; stills and pixel inspection for agents.

---

## 13. Dual-silicon hardware design (NEON §17)

BASE counterpart: §20 (Spark), §27 (determinism), §28 (performance).

### 13.1 Roles and backends (§17.1)

| | Mac Studio, M5 Ultra | `trj`, two RTX 4090 |
|---|---|---|
| Facts NEON pins | 80 GPU cores, 256 GiB unified memory, no `f64` in Metal, fast-math folds `TwoSum`; about 800 GB/s is an assumption until fingerprinted | 2 × 24 GiB, 128 SMs per card, 48 warps per SM, no NVLink, peer access normally off, `f64` at 1/64 rate |
| Role | **Breadth**: world bank, morphology bank, symbolic and proof host, counterfactual studio, live viewer | **Depth**: specialist training and continuation foundries; independent shards per card; fixed-order host-staged gradient sync |
| Backend | `frob-gpu-metal`, `forbid(unsafe_code)`, on an upgraded `ft-kernel-metal` "v0.2" (zero-copy slices, private buffers, byte offsets, concurrent encoders, asynchronous submit with shared events); Metal 3.2 at M2, Metal 4 an open decision | `frob-gpu-cuda`, the only unsafe crate, "< 600 LOC" of audited unsafe: driver API and NVRTC, stream-ordered pools (`cuMemAllocAsync`) so cancellation cannot leak, one OS thread per context, a persistent tick control block, CUDA Graphs for warmed loops, `f64` only for twin tests at ≤ 256 worlds |
| Shared | Content-addressed cache of compiled binaries (CUBIN and `MTLBinaryArchive`); a startup `TwoSum` canary for double-float kernels (`FROB-REF-402`); `GATE-FORGE-00` fingerprints both machines (measured bandwidth, working-set limit, 64-bit atomic probe, peer access) | |

- **BASE:** Has both backends by a different route (annex workspaces, pinned `cudarc`, the existing Metal gateway with packed arguments, shared dialect). BASE treats the missing gateway features as upstream asks (FD-FT-01) with a fallback; NEON schedules the upstream change as its own bead `BD-029a`.
- **Verdict:** The role split is Sound and worth adopting as policy. The hardware fingerprint gate, the binary cache and the canary are Sound. See (C) for the backend conflict.

### 13.2 Device arena (§17.2, `frob-gpu`, 8.5 kLOC)

- **`DeviceArena`**: every per-world quantity (q, q̇, τ, transforms, actuator and battery state, sensor rays, contact pairs, morphology tables, rollout transitions) in pre-allocated, 256-byte-aligned structure-of-arrays slabs. No per-tick allocation.
- **`SlabLease`**: an epoch-fenced lease per slab, because shared-storage writes on Metal are CPU-visible only at command-buffer completion or a shared-event signal; CPU diagnostics and GPU stepping overlap on double-buffered ring slots.
- **`GpuTensorView`**: typed strided views into the arena handed to the learning kernels (8×8 half-to-single matrix products on Metal `simdgroup_matrix`; half or bfloat tensor-core products on CUDA). Policy forward, backward, advantage estimation and PPO updates stay on the device.

- **BASE:** Has fixed-capacity buffers, device-resident learning kernels and host views (§14.3, §19.1, §19.4). The lease is new and Sound. Tensor-core policy kernels are a micro-optimization for 256-wide networks.

### 13.3 Deterministic scatter and replay (§17.3, `B1`, `frob-gpu::determinism`)

**Argument.** Seeding the generator fixes draws, not reduction order. Unordered `f32` atomics in impulse accumulation, BVH refits and warp reductions inject one-ulp differences that grow like t ≈ ln(2²⁴)/λ ≈ 16.6/λ seconds. That destroys common-random-number pairing (NEON's illustration: correlation 0.95 → 0.50 inflates paired variance (1 − ρ) tenfold) and turns exact attribution into estimation.

**Mechanism.** (1) Order-independent contact scatter: stable-key sort and segmented reduction on both backends, or Q32.32 integer accumulation where 64-bit atomics are proven available. (2) A state hash (`xxh3`) over canonical buffers every K steps. (3) A `DeterminismClass` on every receipt: `BitwisePerDevice`, `Shadowed { eps, horizon }` or `None`; a paired comparison or exact attribution requested under `None` is refused with `FROB-REF-405`. (4) Metal and CUDA runs are always declared unpaired. Gate `GATE-DET-01`: 1,000 contact-rich 2 s windows replay bit-identically at ≤ 10% overhead.

- **BASE:** Has, and stronger. Floating-point atomics are banned outright; compaction is count-scan-write; reductions are fixed-order trees; digests are in-kernel integer mixes; warm starts are in snapshots; classes D0–D4 (§27).
- **What to take:** the refusal that ties statistical verbs to a determinism class, and the explicit argument for why pairing needs bitwise replay. Do not take NEON's kill clause that drops ordered scatter if atomics happen to agree on 99.9% of windows.

### 13.4 MorphoJIT (§17.4, crate `frob-morphvm`, 9.5 kLOC)

| Tier | Mechanism |
|---|---|
| 0 | Any new tree compiles in "< 1 ms" to 128-byte `MorphoTable` rows executed by six opcodes: `FK_SCREW`, `BIAS_TWIST`, `INWARD_ABA`, `OUTWARD_ACC`, `ACT_PHS`, `SENSE_TAP`. Loop and rolling constraints (`PFAFF`, `LOOP`) go to the coupled constraint lane. |
| 0.5 | Unchanged library limbs run precompiled inward and outward blocks and pass the 27-float articulated inertia and bias at the base port. |
| Contact | Active contact islands are compacted each step into three homogeneous queues by size (≤ 10, 11–16, > 16). |
| 1 | As racing halves a tournament (512 → 128 → 32 → 8 designs), CPU workers compile fused specialized kernels for survivors and promote cohorts only at acknowledged tick boundaries with versioned state maps. First-round batches of 65,536–131,072 worlds (8–16 warps per SM on two cards). Worlds are rotated by a Latin square to balance training load; every elimination compares paired `(world, replicate_id)` seeds at matched training stages. |

`XT-P2` compares four arms on 512 topologies on both machines: MorphoJIT; ragged operator queues alone; warm CUDA Graph buckets; and nine padded per-family templates. If the templates cover ≥ 95% of mutations and are faster, they become the engine and the interpreter is demoted to a preview (`KILL-MORPHVM-1`).

- **Lever claimed:** a population of different robots is one GPU workload, with no rebuild per design.
- **BASE:** Lacks. BASE's batched step is one `RobotModel` per environment, with shapes fixed at creation (§14.3, §20.5).
- **Verdict:** The mechanism is Sound; a table-driven tree recursion is ordinary, and the new part is letting the table vary per world inside padded capacities. That is compatible with BASE's static launch graphs: BASE already lets each world index one of K scene variants, and model variants can be handled the same way. The 10× and 0.33× thresholds are hypotheses, and the experiment with the per-family-template control arm is well designed. "0 ms stall" and "< 1 ms" are slogans.

### 13.5 Fleet scheduler (§17.5, `frob-gpu::fleet`)

The Mac holds 500–1,000 resident worlds; the 4090s train. **Seeds, not voxels**: only seeds, recipes, design and module diffs, receipts and policy weights cross the network (≤ 1 GB per campaign-hour over asupersync QUIC; cold scan-cache misses metered). Balancing by `fs-tropical::max_cycle_mean` on measured queue and compile telemetry. Gate: ≥ 1.5× campaign speedup; kill: run the two machines standalone.

- **BASE:** Lacks cross-machine orchestration; notes the Mac is the large-scene machine (§28.4). BASE also records a machine rule NEON ignores: jobs are capped at 16 GiB, 8 cores and one hour until the owner approves a campaign envelope (OPEN-SPARK-2).
- **Verdict:** Plausible. The "seeds, not voxels" law is Sound and free. Max-plus balancing of a two-node pipeline is Ornament.

---

## 14. Tasks, learning, gait atlas, skills, co-design cascade, lifecycle (NEON §18)

BASE counterpart: §19 (Cortex), §21 (Arena), §22 (Atelier), C-10, C-15.

### 14.1 `.ftask` and `.fmission` (`FMT-FTASK-1`, crate `frob-spec`, 5.5 kLOC)

1. **Unit-normalized temporal-logic predicates.** Each predicate ψ_k(x) ≥ 0 declares a scale s_k (0.10 m clearance, 10 °C thermal margin, 5 N·m torque margin) so robustness ρ_k/s_k is dimensionless before min and max compose them.
2. **Past-time operators online, future-time offline.** `H_[a,b]`, `O_[a,b]`, `S_[a,b]` compile through a streaming monotonic deque into reward kernels; `G_[a,b]`, `F_[a,b]`, `U_[a,b]` produce delayed verdicts over completed traces, never a silently truncated window.
3. **Layouts and clocks.** Canonical `ObsLayout` and `ActLayout`; per-sensor rates (physics 500 Hz, IMU 500 Hz, policy 50 Hz, lidar 10 Hz, thermal 10 Hz).
4. **Settled-pose reset cache.** Episodes start from precomputed quasi-static poses (64 poses, ≤ 0.5 mm penetration), removing spawn-impulse spikes.

```lisp
(mission "kitchen_tray_delivery" :schema "frob.task/v1"
  (world "commercial_kitchen_proxy_v3" :tribology-tier A :slosh-modes 3)
  (clocks :physics 500Hz :imu 500Hz :policy 50Hz :lidar 10Hz :thermal 10Hz)
  (reset-cache :settled-poses 64 :max-penetration 0.0005m)
  (stl-contract
    (pred clear-margin :expr (- min_clearance 0.04m)   :scale 0.10m)
    (pred thermal-ok   :expr (- 115degC max_winding_t) :scale 10degC)
    (pred spill-ok     :expr (- 0.015m tray_slosh_h)   :scale 0.015m)
    (online-past-reward (H_[0s,2s] (and clear-margin thermal-ok spill-ok)))
    (offline-verdict (and (G_[0s,45s] (and clear-margin thermal-ok spill-ok))
                          (F_[0s,45s] (< dist_to_pass_counter 0.15m))))))
```

- **BASE:** Weaker. Tasks are declarative data with unit-carrying reward terms, terminations, randomization and curriculum, and each term declares its representation (§19.2). There is no temporal logic, no mission verdict distinct from reward, and no reset cache.
- **Verdict:** Sound. Adopt as the mission layer above BASE's task terms.

### 14.2 Learning (`frob-gym` 8.5 kLOC, `frob-equiv` 6 kLOC)

PPO, SAC, TD3 and evolution strategies on arena views; privileged critic with realistic-sensor actor; adaptive domain randomization; shielded exploration; equivariant layers compiled from the symmetry crate. Gate: ≥ 1.8× sample efficiency over ten paired seeds with no asymptotic regression.

- **BASE:** Has PPO first, then SAC and teacher–student, behaviour cloning from the baseline controller, a gradient oracle against frankentorch, a learning gate with zero-weight and random-policy negative controls, and randomization derived from card uncertainty. BASE is stronger on verification; NEON adds algorithms.

### 14.3 Gait atlas with orbits and continuation (§18.3, crate `frob-gaitatlas`, 8 kLOC)

1. Enumerate (H, K) pairs of Γ_obs; propose trot, bound, pace, gallop, pronk, tripod and wave seeds for any leg count.
2. Symmetry-reduced shooting on [0, T/m] inside Fix(K), then a **mandatory full-space monodromy matrix D_x P_θ** and transient growth spectrum (`fs-spectral::floquet`), so symmetry-breaking instabilities are not hidden.
3. Design gradients: augment state with morphology parameters θ; propagate event-time and reset derivatives through the saltation matrix S = D_x R + (f⁺ − D_x R·f⁻)·∇gᵀ/(∇gᵀf⁻); solve the adjoint (I − D_x P_θ)ᵀλ = ∇_x J; read first-order design gradients (NEON calls this the envelope theorem; it is the implicit-function theorem at a fixed point).
4. Refusal `FROB-REF-204` on grazing impacts (|∇gᵀf⁻| below a threshold), simultaneous multi-foot impacts and mode-order switches, routing to derivative-free search.

Gate: 100 cases, all 20 grazing or unstable controls refused, ≥ 72 of 80 regular cases with gradient cosine ≥ 0.95 against central differences, ≥ 10× over finite differences. Kill: use the atlas for stability diagnostics and warm starts only.

- **BASE:** Weaker. A-04 lists gaits, phase lags and max-plus timing, and uses them as controller clocks; it computes no orbits, no stability and no design gradients.
- **Verdict:** Plausible. The mathematics is correct. Two gaps: NEON never says which controller closes the loop whose periodic orbit is being continued (BASE's synthesized baseline controller is the natural answer), and refusing simultaneous multi-foot impacts refuses the symmetric gaits the atlas exists to find, since trot, bound and pronk have simultaneous touchdowns by construction. With BASE's compliant contact the return map is continuous and differentiable almost everywhere, so a monodromy matrix comes from differentiating the discrete step and needs no saltation matrices; that is the better first implementation. The quoted 13.61× is an operation count, not a measurement.

### 14.4 `.fskill` transfer (§18.4, `FMT-FSKILL-1`, crate `frob-control`, 9.5 kLOC)

- **Layer 1, exact:** a morphology-independent task-space wrench and trajectory specification; a per-mode affine actuation map w = w₀(q, q̇) + B_m(q)·u; a bounded whole-body program with torque and speed boxes, 4D friction cones, support limits and electrical and thermal secondary costs. If the task wrench is outside the new body's attainable set, a **Farkas separating witness** (`FROB-REF-206`) is returned before any training.
- **Layer 2, learned:** `FiberMorph`, a frozen dimensionless residual (momentum scales m₀√(g·ℓ₀) and m₀·ℓ₀·√(g·ℓ₀)) acting above the whole-body program.
- Also: energy-shaping control (IDA-PBC), Lie-group DDP, certified passive braking.

- **BASE:** Has per-design controller synthesis with a three-outcome report (`ok`, `synthesized-failed`, `not-synthesizable` with a reason), a cone solver shared with contact, and an estimator. Lacks a transferable skill artifact and an infeasibility certificate. NEON lacks baseline synthesis for every design, which is BASE's answer to "every design can stand on day one".
- **Verdict:** Layer 1 is Sound; the witness is a dual certificate BASE's cone solver can already emit. Layer 2 is Plausible and a Moonshot.

### 14.5 Six-stage co-design cascade (§18.5, crate `frob-codesign`, 8.5 kLOC)

| Stage | Content | BASE |
|---|---|---|
| 1 | Mission quotient and orbit deduplication, then preflight | Lacks deduplication; has `design check` |
| 2 | Reuse cached module proofs and kernels; synthesize linkages | Has memoization |
| 3 | Screen by gait continuation and zero-shot skill allocation. **Blinded stratified audit of discarded designs**: sample rejected strata into stage 4, and plant a poorly charted winner that must survive. | Lacks the audit. BET-05 tests screening accuracy once, on a corpus. |
| 4 | Mixed-morphology racing tournament on paired seeds | Has racing (§22.6), one model at a time |
| 5 | **Model-form rank check** (`B2`, `frob-codesign::rankcheck`, `frob-bench::model_form`): re-evaluate top-k and runners-up under (a) penalty against complementarity against barrier contact, (b) circular against pyramidal cone, (c) contact stiffness ×½ and ×2, (d) Δt/2, (e) solver tolerance ÷10, and run a **per-step work–energy audit of the simulator**, ΔE_mech − W_act ≤ ε, to catch penetration trampolining and pyramid-corner exploits. Results are typed `EvaluatedDesign<ModelRobust { variants, top_k_overlap, energy_audit }>` or `ModelConditional`; the latter cannot spend prototype budget. Cost 5–10% of a campaign. | Has the policy-level version (§19.6: half step, next tier, parameters within card uncertainty, alternative law) and the energy residual as its own ledger line (§23.4). Lacks rank stability across designs and the spend block. |
| 6 | **Blinded qualification firewall** (`BS-3`, `frob-ledger::qualification`, `INV-010`): an open discovery lane and a sealed confirmation runner on fresh, independent world clusters. Request i pre-commits candidate, baseline, mission and metrics to the ledger, runs a paired e-process at level α_i = α/[i(i+1)] (the series sums to α, so the campaign-wide false-promotion rate holds under any number of adaptive retries), and returns only the decision and an aggregate receipt until the campaign closes. Gate: rate ≤ α under 500 adaptive retries on no-signal worlds, with power preserved on planted gains. Plus the **schedule check** (`BS-2`, `frob-runtime::schedule_check`): dynamic partial-order reduction over sensor capture, packet delivery, estimator read, watchdog expiry and actuator commit on asupersync's lab runtime; 6 events with 3 independent conflicting pairs reduce from 720 orders to 8. | Has anytime-valid single comparisons (C-15), scene splits, registered predictions. Lacks control of repeated adaptive confirmation and any schedule exploration. |

- **Verdict:** Stages 3 (audit), 5 and 6 (firewall) are Sound and are NEON's most important methodological contributions. The schedule check is Sound as a technique; its value depends on whether controller timing or firmware is in scope.

### 14.6 Lifecycle compression (§18.6, crate `frob-lifecycle`, 6 kLOC)

1,000 hours at 500 Hz is 1.8e9 steps. Two-timescale averaging (the heterogeneous multiscale method) over settled gait cycles: short micro-windows sample joint loads, sliding speeds, currents and temperatures; macro ODEs advance Archard tooth wear (which grows backlash β_j(t), fed back into contact), Miner fatigue **carrying the rainflow residue stack between windows so open half-cycles are not lost**, Arrhenius insulation aging and cell capacity fade, with step-doubling error control. When a Floquet multiplier approaches 1, or a rare overload occurs, it drops back to full resolution. Always `Estimated`. Gate: a 100-hour reference compressed ≥ 50× at ≤ 5% error and a planted backlash-induced gait collapse caught.

- **BASE:** Lacks.
- **Verdict:** Plausible. The method is Sound; every wear law is empirical. It depends on the gait atlas for its limit cycles.

---

## 15. Adversarial testing, counterfactual debugging, manufacturing outputs (NEON §19)

BASE counterpart: §19.6 (exploit detectors), §23 (telemetry, replay, teleport, failure taxonomy), A-15; firmware is NONGOAL-04.

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| Legality-guarded adversarial search (§19.1, `frob-redteam`, 8 kLOC) | Adaptive cross-entropy search over world perturbations inside physical legality priors (μ ≥ 0.09; hallway ≥ 75 cm; closer ≤ 22 N·m; scan uncertainty bands), with value-of-information selection of rank-flipping experiments (`fs-voi`). | Lacks; has card-derived randomization and tail metrics | Sound |
| Candidate-body solvability oracle | A counterexample is kept only with a witness that the candidate's own body, actuators and mission constraints could solve it; a witness that uses privileged state is tagged plant-feasible only. | Lacks | Sound. This is what separates a useful trap from an impossible one. |
| **Minimal counterexample world** (`FMT-MCW-1`, `.fmcw`) | Delta debugging (`ddmin`) shrinks each failure to a 1-minimal edit of ≤ 3 atomic deltas that reproduces bit-identically (state-hash verified), then asks the retrofit solver whether a cheap kit fixes it. Gate: planted traps of probability ≤ 3e-5 found ≥ 3× faster than uniform randomization, ≥ 70% solvable. | Lacks | Sound. The best agent-feedback artifact in NEON. |
| Six-axis telemetry (`FMT-TELEM-1`, `.npz` via `fnp-io`) | Mechanics and contacts; actuator thermal, electrical, entropy; sensor and estimator observability; shield and energy margins; robustness traces; state hashes. | Has levels T0–T3 with ring buffers and token-budgeted summaries (§23.2, §23.6), which is richer | Sound |
| Twelve failure classes | `FAIL-SLIP-GREASE`, `-THERMAL-CLIFF`, `-BROWNOUT`, `-GLASS-BLIND`, `-SLAM-ALIAS`, `-DOOR-PINCH`, `-CORD-SNAG`, `-SLOSH-TIP`, `-GAIT-BIFURC`, `-OBS-ALIAS`, `-RACE-TIMING`, `-MODEL-DRIFT`. | Has eleven mechanical classes (§23.5); NEON's are environment- and perception-specific | Sound; merge the lists |
| **Counterfactual time travel** (§19.2, `frob-diagnose`, 7.5 kLOC) | Rewind a failure to t_fail − Δt from ring memory and run all 2¹⁰ = 1,024 coalitions of ten interventions (`do(grease=0)`, `do(N_knee=16)`, `do(lidar_z=+0.08m)`, `do(ramp_D=true)`) under bitwise replay in < 1.5 s; return exact Shapley attributions, plus "Wasserstein DRO sensitivity bounds". Gate: ≥ 90% top-1 accuracy on planted causes; ≥ 3× fewer agent repair iterations than with logs. Kill: single-factor paired rewind only. | Weaker: snapshots, restore, replay and cross-tier teleport exist; first cause is an earliest-anomaly heuristic labelled `Estimated` | Sound for environment factors. For design factors (a gear ratio) an intervention applied at t_fail − Δt is not the counterfactual of interest, because thermal and policy history differ; those need a re-run from the episode start on the same seeds. The DRO bounds are named, not specified. |
| Build packet `.fbuild` (§19.3, `FMT-FBUILD-1`, crate `frob-mfg`, 9 kLOC) | One atomic reviewer-ready packet from a qualified robot and task: (1) parts joined against pinned supplier catalogs (`fs-io::supplier_corpus`) under a **generalizable-cut rule**: a failed simulation prunes a part family only when an algebraic bound proves the failure holds across the family; (2) faceted STEP and STL (`fs-io::step_faceted_export`) validated in FreeCAD, a priced bill of materials, an adjoint tolerance stack (`fs-toleralloc`), and a wiring harness routed as a Steiner tree with rod sag, checked for bend radius, joint-sweep collision, ampacity and voltage drop; (3) firmware. | Lacks (A-15 structural margin only) | Plausible. The FrankenSim crates exist, though `fs-fab` and `fs-geocon` are small. Pricing data and its licence are an open decision in NEON. |
| `#![no_std]` firmware | Controller, filter, energy monitor and shield lowered from the kernel IR into a zero-heap crate behind a caller-owned `MotorBusHal`; verified by a proof obligation, QEMU Cortex-M4/M7 parity (≤ 1e-5 over 10,000 steps, no heap), stack-depth analysis and a real-board timing receipt. All exports carry the three false flags. | Excluded (NONGOAL-04); T-14 is a related Moonshot | Plausible for small controllers; Doubtful for a whole-body program and a filter in fixed point on a Cortex-M4. The board receipt and the 11-of-12 reviewer gate need resources outside the project. |

---

## 16. The agent studio (NEON §20)

BASE counterpart: §24 (Tether), §17.4–17.5 (stills, viewer), §22.8 (reports).

| Item | What it is | BASE | Verdict |
|---|---|---|---|
| One verb registry | `registries/verbs.toml` generates CLI commands, daemon RPCs, MCP tools and TUI key bindings; `xtask check-verbs`. | Has, with a richer entry (kind, capabilities, schemas, refusals, idempotency, cost model, examples) | Sound |
| CLI and daemon | `frob --json` writes `FMT-ENV-1`; `frobd`, a second binary in `frob-cli`, speaks `FMT-RPC-1` over a Unix socket or QUIC and keeps the arena, world banks and kernel caches hot; warm round trip < 5 ms. | Has, with the envelope schema, exit codes, `--estimate`, budgets and idempotency keys specified | Sound |
| **Asynchronous MCP job contract** | 42 tools in 7 clusters (`design`, `preflight`, `simulate`, `synthesize`, `redteam`, `foundry`, `ledger`), not enumerated. Every long tool accepts `sync_wait_ms` (default 5,000, cap 30,000), returns `{status: "running", job_id, progress_pct}` if work continues, and pairs with `job_poll`, `job_stream`, `job_cancel`, with hard compute and wall-clock caps. Named tools: `bot_preflight`, `codesign_run`, `simulate`, `train`, `gait_atlas`, `homotopy_synth`, `redteam`, `time_travel`, `qualify`, `export_build`. | Has start, status, cancel and progress notifications, and clusters of at most seven tools. Lacks the bounded synchronous wait. | Sound. Adopt `sync_wait_ms`. |
| Discovery resources | `resources://` for `archetypes`, `materials`, `cots_catalog`, `amendments`, `worlds`, `receipts`. | Has stills, reports, schemas as resources | Sound; add the catalogs |
| TUI (`frob-tui`, 6.5 kLOC) | Panes: racing survival brackets, per-machine GPU and memory, contact-queue utilization, thermal bars, α-budget meter, failure autopsies; snapshot regression tests. | Optional dashboard (FD-FTUI, Frontier) | Sound |
| Viewer (`frob-view`, 9.5 kLOC) | Native 60 FPS viewport on the Mac with contact cones, film heat maps, clearance doorways, force and energy overlays; and a versioned `WebStudioScenePacket` (`FMT-WSP-1`) lowered to a single offline HTML/WebGPU file of ≤ 15 MB with ghost-overlay counterfactual replays and build-packet inspection, depending only on franken_threed's four existing crates. | Has a Three.js viewer with self-contained HTML export and franken_threed as an optional packaging route; plus stills and pixel inspection, which NEON lacks | Plausible. The overlays are the useful part. |
| Reports (`frob-report`, 5.5 kLOC) | Deterministic Markdown, HTML and PDF dossiers through franken_markdown and frankenmermaid, stamped with claim axes, claim-product boundaries and qualification receipts. | Has evidence packages (§22.8) | Sound |
| Offline auditor (`frob-verify`, binary, `FMT-CERT-1` `.frobcert`) | A `std`-only tool that replays proofs, Merkle lineage and α-spend receipts with no GPU or network; NEON states its transitive trusted base honestly as about 88 kLOC. | Has `frob prove verify` and capsules; not a separate binary or bundle format | Sound. Adopt the bundle. |

Session shape from NEON Appendix D.3: `bot_preflight` with the mission returns in 142 ms with stages 1–6 passed, a stage-7 warning on a 22 mm threshold and a retrofit hint; `codesign_run` with `budget_gpu_min: 15` returns a `job_id` at once with `determinism_class` and `execution_authorized: false`; `job_poll` returns the thermal-cliff failure resolved by gear ratio 12 → 16 and lidar height 0.12 → 0.20 m, with `claim_axis: "EvaluatedDesign<ModelRobust>"` and a qualification receipt.

BASE items NEON lacks: `next_actions` in every envelope, exit-code semantics, cost estimates before running, capability tokens and per-agent namespaces, token-budgeted summaries, `robot-docs`, `doctor`, registered predictions, lineage diff, blame and bisect, and the rule that imported text is data and never instructions.

---

## 17. Dependency ledger, crate topology, invariants, formats (NEON §21–§22)

BASE counterpart: §25 (postures, airlock, stack lock, admission gates, dependency firewall), §26 (crate map, layer rules).

### 17.1 Foundation ledger (`FD-01`–`FD-17`)

Workspace pinned to `nightly-2026-08-31` and `asupersync = "=0.6.2"`, with a root `[patch.crates-io] block` entry for the Metal binding.

| FD | Sibling | Mode | Use |
|---|---|---|---|
| 01 | frankensim, `fs-qty`-free crates | Consume | `fs-ga`, `fs-phs`, `fs-symmetry`, `fs-sos`, `fs-eproc`, `fs-spectral`, `fs-tropical`, `fs-topo`, `fs-ivl`, `fs-conform`, `fs-voi`, `fs-race`, `fs-geocon`, `fs-fab`, `fs-toleralloc`, `fs-topopt`, `fs-asbuilt`, `fs-matdb`, `fs-io`, `fs-tilelang` |
| 02 | frankensim, `fs-qty`-coupled crates | **Lift with provenance** (`registries/lifts.toml`), replacing `fs-qty` by a new stable `frob-units` | `fs-mbd`, `fs-contact`, `fs-evidence`, `fs-lbm`, `fs-tribo`, `fs-solid` (rod, fatigue), `fs-ports`, `fs-system`, `fs-couple` |
| 03 | franken_lean | Consume | `fln-core`, `fln-kernel`, `fln-checker`, `fln-anvil` (`DTree`, `omega` only) |
| 04 | frankensympy | Consume, guarded by `KILL-FSYM-1` | `fsym-core`, `-calculus`, `-matrices`, `-polys`, `-solvers::real_rur`, `-proof-kernel` (verifier) |
| 05 | frankentorch | Consume, scoped, plus an upstream change (`BD-029a`) | `ft-kernel-metal`; `ft-core`, `ft-nn` for CPU reference and weight I/O only |
| 06 | franken_numpy | Consume | `fnp-ndarray`, `fnp-linalg`, `fnp-io` |
| 07 | franken_threed | Consume through an adapter | Four existing crates |
| 08 | asupersync | Consume | Scopes, cancellation, lab runtime, QUIC |
| 09 | frankensqlite | Consume | Merkle design ledger and qualification ledger |
| 10 | fastmcp_rust | Consume | MCP server |
| 11 | frankentui | Consume | TUI |
| 12 | franken_textiles | Consume or lift | `ftx-drape` cloth, evaluation tier only |
| 13 | franken_drone_geometry_reconstruction | Schema adapter | Scans with coverage and uncertainty masks; re-scan request protocol |
| 14 | franken_networkx | Consume | Tree, cycle and Steiner-tree algorithms |
| 15 | frankenscipy | Consume | CPU reference linear algebra and quadrature |
| 16 | franken_markdown, frankenmermaid | Consume | Reports |
| 17 | franken_firmware | Pattern | HAL-isolated `no_std` lowering with QEMU parity |

- **BASE:** Different posture. FrankenSim is consumed by path behind one airlock crate, pinned by commit, with the core on FrankenSim's own toolchain, a lift fallback (KILL-FSIM), a stack lock, admission records that must cite a real lockfile, and a table showing which milestone needs which sibling. Sibling repos are read-only; needs are filed as asks.
- **Verdict on NEON's posture:** the diagnosis is Sound (I confirmed the `generic_const_exprs` coupling and the differing nightlies). The remedy is Doubtful: the lift set is over three million lines against a 378 kLOC budget with no line for it, and two of the "free" crates are not free. FrankenSim crates NEON uses that BASE's airlock list should add: `fs-phs`, `fs-spectral`, `fs-topo`, `fs-conform`, `fs-voi`, `fs-toleralloc`, `fs-topopt`, `fs-geocon`, `fs-io`, `fs-lbm`. Siblings NEON uses that BASE does not: franken_textiles, the drone reconstruction project, franken_networkx, franken_markdown, frankenmermaid, franken_firmware.

### 17.2 Layer rule

Dependencies flow strictly downward: `U` (studio) → `V` (I/O and diagnosis) → `E2` (learning, control, co-design) → `E1` (sensing) → `E0` (dynamics, GPU) → `K2` (morphology, worlds) → `K1` (symbolic, proof) → `K0` (Lie) → `S1` (runtime, ledger) → `S0` (core). Layer `T` (verification) depends downward and is imported by nothing. Four "cleanliness" devices: a pure token crate `frob-cx` passes cancel and budget handles into the algebra layers without importing the async runtime; the subprocess symbolic backend lives in `V`; the binary kernel cache lives in `frob-gpu`; `frobd` is a second binary in `frob-cli`.

The token crate is a Sound idea BASE can use. The layering itself does not survive NEON's own bead graph (see D): the world crate in `K2` needs GPU buffers from `E0`, the shield in `K1` depends on the morphology engine in `E0` and the estimator in `E1`, and GPU continuation in `K1` needs the GPU crates.

### 17.3 The 51 crates (★ = on the M0–M2 critical path, 31 crates, 219.2 kLOC)

| # | Crate | Layer | kLOC | Purpose |
|---|---|---|---|---|
| 1 | `frob-core` ★ | S0 | 4.0 | Philox generator, hashes, refusal algebra, identifiers |
| 2 | `frob-units` ★ | S0 | 3.0 | Stable dimension-exponent algebra, Buckingham Π; replaces `fs-qty` |
| 3 | `frob-evidence` ★ | S0 | 4.5 | Claim axes, `ClaimProduct`, receipts |
| 4 | `frob-cx` ★ | S0 | 1.2 | Pure cancel and budget token |
| 5 | `frob-runtime` ★ | S1 | 6.5 | Scope orchestration, GPU lease budgets, schedule check |
| 6 | `frob-ledger` ★ | S1 | 7.0 | Merkle design ledger, blinded qualification firewall |
| 7 | `frob-lie` ★ | K0 | 6.0 | Conventions, groups, PGA and CGA, `AffineRows`, pseudo-inertia, duals |
| 8 | `frob-bundle` ★ | K0 | 4.0 | Connection and curvature |
| 9 | `frob-symmetry` ★ | K0 | 6.5 | Two groups, typed projectors, wreath, H/K, orbit counting |
| 10 | `frob-symbolic` ★ | K1 | 9.0 | Symbolic kinematics and dynamics, skew Coriolis, exact linkage synthesis |
| 11 | `frob-homotopy` | K1 | 10.0 | Polyhedral and parameter continuation, α-certification, trace test |
| 12 | `frob-kernel-ir` ★ | K1 | 5.5 | `FMT-KIR-1`, CSE, register scheduling, congruence trace |
| 13 | `frob-codegen` ★ | K1 | 9.5 | Rust, Metal, CUDA and `no_std` lowering |
| 14 | `frob-proof` ★ | K1 | 8.5 | `FrobMath` reflection, `omega`, obligation catalog |
| 15 | `frob-shield` | K1 | 7.5 | `.fenergy`, `.ftube`, box-constrained shields |
| 16 | `frob-operad` ★ | K2 | 6.5 | `.fmodule`, limb port interface, `.fquot` |
| 17 | `frob-bot` ★ | K2 | 9.5 | `.fbot` parser and macros, 18 archetypes, structural diff |
| 18 | `frob-spec` ★ | K2 | 5.5 | `.ftask`, streaming kernels, layouts, reset cache |
| 19 | `frob-actuators` ★ | K2 | 6.0 | Motor, gearbox, battery, rotor port |
| 20 | `frob-preflight` ★ | K2 | 8.0 | Eight-stage preflight |
| 21 | `frob-mfg` | K2 | 9.0 | Build packets |
| 22 | `frob-world` ★ | K2 | 9.0 | Three-tier world scene, fixtures |
| 23 | `frob-worldgen` | K2 | 11.5 | Six interiors, clearance complexes, retrofit catalog |
| 24 | `frob-terramech` | K2 | 5.5 | Outdoor terrain and soil |
| 25 | `frob-dyn` ★ | E0 | 10.5 | Integrators, nonholonomic rows, rods |
| 26 | `frob-contact` ★ | E0 | 12.0 | 4D contact, joint rows, Delassus tiers, multi-RHS, barrier lane |
| 27 | `frob-morphvm` ★ | E0 | 9.5 | MorphoJIT |
| 28 | `frob-multiphys` | E0 | 8.5 | Thin film, slosh, evaluation-tier fluid and cloth |
| 29 | `frob-adjoint` | E0 | 8.0 | Discrete adjoints, saltation, sensitivity probes |
| 30 | `frob-gpu` ★ | E0 | 8.5 | Arena, views, determinism, fleet, binary cache |
| 31 | `frob-gpu-metal` ★ | E0 | 7.5 | Metal backend |
| 32 | `frob-gpu-cuda` ★ | E0 | 7.0 | CUDA backend, the one unsafe crate |
| 33 | `frob-sensors` | E1 | 9.0 | Ray-traced sensors, tactile, calibration |
| 34 | `frob-estimate` | E1 | 6.5 | Invariant filter, observability Gramian |
| 35 | `frob-equiv` | E2 | 6.0 | Equivariant layer compilation |
| 36 | `frob-control` | E2 | 9.5 | `.fskill`, whole-body program, IDA-PBC, DDP, backup brakes |
| 37 | `frob-gaitatlas` | E2 | 8.0 | Gait atlas |
| 38 | `frob-gym` | E2 | 8.5 | Vector environment and learning algorithms |
| 39 | `frob-codesign` | E2 | 8.5 | Cascade, discard audit, rank check |
| 40 | `frob-lifecycle` | E2 | 6.0 | Lifecycle compression |
| 41 | `frob-telemetry` ★ | V | 5.0 | Six-axis recorder, ring snapshots, hash stream |
| 42 | `frob-diagnose` | V | 7.5 | Failure classes, counterfactual forks, attribution |
| 43 | `frob-redteam` | V | 8.0 | Adversarial search, shrinking, retrofit solver |
| 44 | `frob-io` ★ | V | 9.5 | URDF, MJCF, glTF, USD and scan importers; subprocess symbolic backend; build-packet exporter |
| 45 | `frob-bench` ★ | T | 8.0 | Known answers, rival audit, thermal ablations, model-form audit |
| 46 | `frob-verify` ★ | T | 8.0 | Offline certificate auditor |
| 47 | `frob-cli` ★ | U | 6.5 | `frob` and `frobd` |
| 48 | `frob-mcp` ★ | U | 7.5 | MCP server |
| 49 | `frob-tui` | U | 6.5 | TUI |
| 50 | `frob-view` | U | 9.5 | Native viewport and web packet |
| 51 | `frob-report` | U | 5.5 | Dossiers |
| | **Total** | | **378.2** | Ceiling 435.3 (1.15×) |

- **BASE:** 24 core crates plus 5 annex crates, cap 361 kLOC, in a layout that separates the unsafe-denied core from GPU annexes, a derivation annex and a Lean project (§26).
- **Verdict:** NEON's list is a useful inventory of responsibilities. Its budgets are Doubtful: roughly three times BASE's scope in the same line count (10 kLOC for a continuation engine, 9 kLOC for sensors of five modalities plus calibration, 8.5 kLOC for four learning algorithms on two backends).

### 17.4 Constitutional invariants (`INV-001`–`012`)

001 no claim laundering. 002 unsafe confined to one crate. 003 strict downward layering. 004 every inertia realizable. 005 one world truth. 006 no mass creation in the film and no silent solver-created energy. 007 paired and attribution receipts require bitwise replay. 008 only Γ_obs for equivariance, deduplication and gaits. 009 cancel-correct leases. 010 the discovery lane cannot read confirmation worlds. 011 no stubbed proof tooling. 012 one verb and format authority.

### 17.5 Format registry (`FMT-*`)

`.fbot` (`FMT-FBOT-1`), `.fmodule` (`FMT-FMOD-1`), `.ftask`/`.fmission` (`FMT-FTASK-1`), `.kir` (`FMT-KIR-1`), `.fquot` (`FMT-FQUOT-1`), `.fclear` (`FMT-FCLEAR-1`), `.fenergy` (`FMT-FENERGY-1`), `.ftube` (`FMT-FTUBE-1`), `.fskill` (`FMT-FSKILL-1`), `.fmcw` (`FMT-MCW-1`), `.fbuild` (`FMT-FBUILD-1`), `.frobcert` (`FMT-CERT-1`), CLI envelope (`FMT-ENV-1`), daemon and fleet RPC (`FMT-RPC-1`), web scene packet (`FMT-WSP-1`), telemetry `.npz` (`FMT-TELEM-1`).

BASE has schema strings (`frob.design/1`, `frob.envelope/1`) and content-hashed identities but no registry of named artifact formats. A registry with an owner crate per format is Sound and worth adopting.

---

## 18. Determinism, interop, security (NEON §23–§24)

| Item | NEON | BASE | Verdict |
|---|---|---|---|
| Determinism classes | `BitwisePortable` (CPU `f64` oracle and offline verifier, across ARM64 macOS and x86-64 Linux); `BitwisePerDevice` (GPU replay and pairing); `Shadowed { eps, horizon }` (fast training; always the class of Metal-against-CUDA); `None` (refused by pairing, attribution and qualification) | D0 exact, D1 bitwise across backends (a hypothesis, measured by an exhaustive IEEE probe), D2 bitwise per backend, D3 tolerance, D4 unspecified (§27.1) | BASE's ladder is finer and better grounded. NEON's portable class assumes deterministic elementary functions that `fs-mbd` does not yet use (BASE ask FD-FSIM-07). |
| Importers | URDF, MJCF, glTF, USD and scan parsers in safe Rust with memory, depth and polygon limits and fuzz targets (> 10 million executions, zero panics) | URDF and MJCF with loss reports through a small in-house XML reader; USD is Frontier | NEON's fuzz budget is Sound. A USD parser inside its line budget is Doubtful. |
| Hardware air gap | Every packet, crate, MCP response and dossier carries `execution_authorized: false`, `hardware_safety_signoff: false`, `sim_to_real_certified: false`; firmware never touches registers, CAN sockets or E-stop relays | "No safety certification" line (§29.6) | Sound. Adopt the machine-readable flags. |
| MCP sandboxing | Path-jailed project roots; wall-clock and GPU-minute budgets per call; capability token before remote fleet jobs | Capability tokens with scopes, per-agent namespaces, budgets, agents supply data not code, imported text is never instructions, private-scene inheritance, licence enforcement | BASE is stronger. Add path jailing. |

---

## 19. Verification (NEON §25–§26)

BASE counterpart: §30 (six tiers G0–G5, oracles, metamorphic relations, chaos, false-progress guards), §31 (gate register).

### 19.1 Known-answer suite (`KA-01`–`KA-14`, crate `frob-bench`)

| KA | Benchmark | Criterion | M |
|---|---|---|---|
| 01 | Lie, PGA, pseudo-inertia and reflection identities | 1e-13 in `f64`; small-angle functions smooth to 1e-15 rad | M1 |
| 02 | Asymmetric free rigid body and heavy top on the variational step | Energy error < 1e-6 at h = 0.25 ms; order 2; reversibility < 1e-13 | M1 |
| 03 | Kinematic snake holonomy | Minus-sign curvature matches the path-ordered exponential within 1e-3; the plus sign (error 2.026) is rejected | M1 |
| 04 | Painlevé paradox, spinning disk with spin–slide coupling, rolling sphere | Slip–stick transition time and coupling within 1e-4; no pyramid bias | M2 |
| 05 | Large-deflection elastica cantilever | Tip deflection and curvature within 0.5% | M3 |
| 06 | Invariant filter slip and blind-step recovery | ≤ 2% consistency error against a quaternion filter under 45° yaw error | M4 |
| 07 | Rimless wheel and compass gait monodromy | Return-map eigenvalues and saltation derivatives within 1e-6 | M4 |
| 08 | Plate sinkage and tracked drawbar pull | Sinkage 3.52, 9.79, 4.96 cm and shear curves within 1e-5 | M3 |
| 09 | Motor thermal runaway | Cliff at 33.5 s (I = 25.4 A) within 0.1 s; σ_irr ≥ 0 | M3 |
| 10 | Calibration fault localization | Planted 2°, 15 mm bracket bump localized 100% | M4 |
| 11 | Housner slosh | Frequency and impulsive and convective forces within 3% | M3 |
| 12 | Huppert thin-film spreading | R(t) ∝ t^(1/8) within 2%; mass drift < 1e-6 | M3 |
| 13 | Linkage counts | 3- and 5-pose Burmester; 4,326; 40; 16 | M1, M3 |
| 14 | `.fenergy` drop stop | 1.49 J ≤ 2.00 J; 50.5 and 45.5 ms; no exceedance for any μ ∈ [0, 5] | M2, M4 |

Known answers new to BASE: 03 (with its wrong-sign control), 05, 07, 09 (after re-derivation), 11, 12, 13, 14. BASE's own battery (§15.7, §30.3: incline, rolling sphere, box stack, mecanum, Sarrus and Bennett mobility, growth vectors, Elroy's beanie, mass spiral) is absent from NEON and must be kept. KA-08 and KA-09 pin numbers I could not reproduce.

### 19.2 Exclusion tests

| Test | Question | Arms and controls | Pass, or kill |
|---|---|---|---|
| `XT-P1` (M4) | Does campaign-time algebra pay beyond per-step algebra? | 128 morphologies; A0 generic, A1 specialized kernels only, A2 quotient and atlas only, A3 both. Potency controls: the ballast fixture with second-moment and harness split cases; a blinded stratified audit of discards; a planted poorly charted winner. | A2 ≥ 5× and A3 ≥ 10× over A0 at a 95% paired lower bound; ≥ 6 of 8 top-8 overlap; planted winner stays first; zero false quotients. Else `KILL-OPERAD-1`. |
| `XT-P2` (M2) | Heterogeneous batching or homogeneous specialization? | 512 topologies with realistic contact tails, on both machines: MorphoJIT; ragged queues; warm CUDA Graph buckets; nine per-family templates. | Best hybrid beats warm graphs on edit-to-first-accepted-result at ≤ 128 morphologies and holds ≥ 0.33× specialized speed. If templates win on ≥ 95% coverage, adopt them. |
| `XT-P3` (M4) | Do formal guarantees survive contact uncertainty? | 5,000 in-envelope and 5,000 shifted-world episodes for each safety tier, with independent hidden-state breach labels and an identically sampled shifted baseline. | Zero false admissions for tiers 1 and 2; empirical violations within the stated count bound; shield keeps ≥ 85% of return. Else `KILL-SHIELD-1`. |

The three-part design (competing arms, potency controls, pre-declared kill) is stronger than BASE's single-threshold bets and worth adopting as the form for BASE's method gates.

### 19.3 Admission gates (`INT-*`)

FSIM (free crates compile on the common nightly; lifts pass tests on `frob-units`), FLEAN (kernel, checker and `omega` check a sample reflective theorem with no `sorry`), FSYM (three-pose and five-pose fixtures within the eight-variable cap), FTORCH (patched Metal binding builds; no `rayon` in GPU crates), FNP, F3D, FMCP (builds on the common nightly; async job and cancel test), ASUP (one package identity; lab replay; QUIC), FSQL (crash recovery), FTUI, FTX (M3), FDGR (M5).

### 19.4 Convergence gates (thresholds are TARGETs; all performance gates use paired n ≥ 10, 95% lower bound, 20-minute thermal floor)

| Gate | M | Criterion |
|---|---|---|
| RIVAL-AUDIT | M0 | Incumbent versions, capabilities and baselines frozen |
| FORGE-00 | M0 | Hardware fingerprints of both machines |
| DYNAM-01 | M1 | KA-01–03; energy < 1e-6 at 0.25 ms; implicit-step GPU cost measured |
| BURMESTER-01 | M1 | Exact branches certified, no toggle singularity |
| OPERAD-01 | M1 | Mirror swap reuses proofs (≥ 10×); shared-bus and base-coupled edits invalidate; reversed port sign refused |
| PROOF-01 | M1 | Five obligations in < 5 s; offline replay with no `sorry` |
| PREFLIGHT-01 | M1 | ≥ 90% of 100 seeded defects in < 200 ms, no false admission |
| GPU-TWIN-01 | M2 | CPU `f64` against Metal against CUDA: < 1e-4 rigid, < 1e-3 contact impulse over 500 steps; `TwoSum` canary |
| FORGE-01 | M2 | Mac ≥ 60% of roofline; two 4090s ≥ 1.5M rigid and ≥ 250k dense-contact environment-steps/s |
| DET-01 | M2 | 100% bitwise repeatability on 1,000 contact-rich 2 s windows at ≤ 10% overhead |
| MORPHVM-01 | M2 | `XT-P2` run; crossover and edit-to-result measured |
| CONTACT-02 | M2–M3 | KA-04; Delassus tiers fit; multi-RHS residual 1e-6 and 1e-4 parity |
| CONTACT-03 | M3 | Barrier lane: no tunneling on cords and fragile items |
| COSSERAT-01, ACT-01 | M3 | KA-05; KA-09 and battery sag |
| CLEAR-01 | M3 | No false route and no false impossibility on six interiors |
| OBSALIAS-01 | M3 | Planted blind spots caught; no false witness on negative controls |
| HOMOTOPY-01 | M3 | Counts recovered; ≤ 2 s re-query; honest completeness label |
| TRIBO-01 | M3 | KA-11, KA-12 at ≤ 15% overhead |
| MULTI-01 | M4–M5 | Cheap and expensive multiphysics tiers agree on rankings; port work–energy residuals reported |
| FLEET-01 | M4 | ≤ 1 GB per campaign-hour; ≥ 1.5× campaign speedup |
| EQUIV-01 | M4 | ≥ 1.8× sample efficiency, ten paired seeds, no regression |
| ATLAS-01 | M4 | KA-07; 20 of 20 controls refused; ≥ 72 of 80 gradients; ≥ 10× over finite differences |
| SHIELD-01 | M4 | KA-14; tubes; shield at ≤ 5% overhead with zero box violations |
| DPOR-01 | M4 | All planted stale-read and watchdog races caught; 720 → 8 |
| TRANSFER-01 | M5 | ≥ 18 of 20 feasible bodies within 3 points zero-shot; 4 of 4 infeasible refused; both denominators reported |
| RANKCHECK-01 | M5 | All planted pyramid and trampoline exploits caught; genuine winners pass; ≤ 10% compute |
| QUAL-01 | M5 | False-promotion rate ≤ α under 500 adaptive retries; power preserved |
| LIFE-01 | M5 | ≥ 50× at ≤ 5% error on a 100-hour reference; planted collapse caught |
| REDTEAM-01 | M5 | ≥ 3× faster than uniform randomization; ≥ 70% solvable counterexamples |
| TIMETRAVEL-01 | M5 | ≥ 90% top-1 planted-cause accuracy; ≥ 3× fewer repair iterations |
| AMEND-01 | M5 | ≥ 30% of failures fixed by ≤ 3-item kits with the policy frozen |
| REAL2SIM-01 | M5–M6 | All seeded occluded bottlenecks localized on five real scans |
| S2S-01 | M6 | ≥ 11 of 12 packets accepted by reviewers; robustness matches reference on all traces; fresh agent solves ≥ 5 of 10 missions |
| FW-TWIN-01 | M6 | QEMU parity ≤ 1e-5 over 10,000 steps; no heap; stack bound; board timing receipt |
| WEBSTUDIO-01 | M6 | ≤ 15 MB offline page with interactive counterfactual playback |
| RELEASE-01 | M6 | All scenarios, exclusion tests and known answers; offline audit; unsafe and line-count rules |

### 19.5 Kill gates and fallbacks

| Kill | Fallback |
|---|---|
| FSYM-1 | Subprocess symbolic backend plus a Rust-native generator |
| LGVI-1 | Explicit Lie midpoint for training; variational step for oracle and evaluation |
| CONTACT-1 | Per-island ADMM or Gauss–Seidel with fixed iterations and a residual flag |
| GPU-1 | One-thread-per-world static layout (per-family templates) |
| DET-1 | Ordered scatter only in pairing and attribution lanes |
| MORPHVM-1 | Per-family templates become the engine; bytecode becomes a preview |
| OPERAD-1 | Quotients restricted to exact symmetry and ballast transforms; modules only for caching |
| HOMOTOPY-1 | Exact bounded lane plus multi-start CPU recovery |
| PROOF-1 | Reflection narrowed to four obligations; interval certificates elsewhere |
| SHIELD-1 | Keep tiers 1 and 2; high-DoF shield becomes a soft penalty |
| TRIBO-1 | Film moves to the evaluation tier; slosh and cords stay |
| FLEET-1 | Two standalone machines |
| EQUIV-1 | Symmetry for augmentation and deduplication only |
| ATLAS-1 | Stability diagnostics and warm starts only |
| TRANSFER-1 | Keep the exact layer and the witness; drop the learned residual |
| DPOR-1 | Bounded randomized schedule fuzzing plus exhaustive checks on ≤ 5-event loops |
| LIFE-1 | Rainflow post-processing over 10-minute rollouts |
| REDTEAM-1 | Keep shrinking and legality priors over tuned randomization |
| TIMETRAVEL-1 | Single-factor paired rewind |
| AMEND-1 | Advisory checklist in reports |
| S2S-1 | Bill of materials, STEP and specifications only; harness and firmware deferred |

NEON has no negative-control registry, no rule against self-certification, no live-dependency proofs and no metamorphic tier; those are BASE's (§30.4, §30.8) and are the stronger guards against false progress. NEON's per-bead contract adds one device BASE lacks: a structured NDJSON log per bead (`docs/delivery/logs/<BD>.ndjson`) with timestamp, commit, host fingerprint, determinism class, state hash, claim axis, metrics, refusals and verdict.

---

## 20. Risks, milestones, beads (NEON §27–§29)

**Pre-mortem (four stories).** The `fs-qty` nightly quagmire stalls M0; proof scope creeps into rewriting a theorem prover; the synchronous Metal gateway caps the Mac at a tenth of its throughput; agents exploit solver artifacts and p-hack held-out worlds. BASE has fifteen stories, including "the mathematics became the product", which NEON does not consider.

**Risk register.** 01 interpreter register spill or branch divergence. 02 symbolic expression swell on a 29-joint humanoid. 03 the upgraded Metal gateway or patched binding breaks on an SDK update. 04 no 64-bit atomics in Metal. 05 MCP toolchain bump. 06 franken_threed's planned crates slip. 07 no peer-to-peer between the two 4090s. 08 51-crate compile times slow the agent loop.

**Open decisions.** Metal 4 through `objc2-metal` or stay on 3.2 (default stay); licence and supplier-catalog terms (default Apache-2.0/MIT, strip unverified CAD); a thin Python or Gymnasium shim at M6 (default none); barrier contact on the GPU (default evaluation only).

**Milestones.**

| M | Theme | Main content |
|---|---|---|
| M0 | Admission, hardware fingerprint, rival baseline | Workspace, all admission gates, lifts verified, both fingerprints, incumbents frozen |
| M1 | Algebraic kernel, reflective proofs, preflight | Lie, bundle, symmetry, symbolic with exact linkages, IR and code generation, proof library, modules, robot and task formats, preflight stages 1–7, CPU `f64` dynamics and contact oracle |
| M2 | GPU forge, determinism, MorphoJIT, studio skeleton | Upstream Metal gateway change, arena, both backends, ordered scatter, GPU contact, MorphoJIT and `XT-P2`, twin parity, CLI, daemon, MCP skeleton, first energy-budget slice |
| M3 | Living worlds, homotopy, observation alias | Six interior proxies with clearance, soil, actuators, rods, barrier lane, thin film and slosh, continuation, preflight stage 8, adjoints, first adversarial core |
| M4 | Sensors, equivariant learning, gait atlas, safety, fleet | Sensors and calibration, filter, learning, atlas, three safety tiers and `XT-P3`, schedule check, fleet, `XT-P1`, embedded lowering, TUI, viewer |
| M5 | Co-design cascade, transfer, adversarial testing, lifecycle | Skills, cascade with rank check, qualification firewall, lifecycle, full adversarial loop with retrofits, counterfactual forks, evaluation-tier multiphysics, all 18 archetypes, rich interiors, 42 tools |
| M6 | Build packets, firmware, real scans, release | Manufacturing packet, firmware parity, scan loop, web dossier, reports, offline audit, fuzzing, four North-Star packs, incumbent benchmark publication |

- **BASE:** M1 is a CPU design studio on the referee tier needing only three siblings; GPUs arrive at M2. NEON's M1 requires proofs, modules, a kernel IR with three back ends and seven preflight stages before any robot is simulated on a GPU, and has no baseline controller, so the first closed-loop behaviour arrives with learning at M4. BASE's sequencing is the safer one.

**Work packages and beads.** Twelve work packages. 97 beads (`BD-001`–`096` plus `029a`) in one table with dependencies and exit gates; one convergence bead per milestone (012, 028, 040, 052, 066, 079) that depends on that milestone's gates; four end-to-end North-Star packs (090–093); a paired incumbent benchmark publication bead (094); a single terminal sink (096). BASE has 125 beads with "done when" observables and negative controls, and a first-implementation-session script that NEON lacks. The per-milestone convergence bead and the single sink are worth copying.

---

## 21. Appendices (NEON A–E)

**A. Dueling-wizards synthesis.** Ninety ideas reduced to 39 proposals cross-scored by three models. The scores (for example 780/1000) are opinions, not evidence. What the appendix usefully records is which ideas all three converged on: the energy budget, MorphoJIT with the limb interface and ragged queues, quotients and modules, the gait atlas with saltation, skill capability maps, clearance complexes, linkage synthesis, minimal counterexamples, counterfactual forks, causal film, build packets.

**The six blind spots** (ideas no model proposed until a late probe; all six are in sections above):

| Tag | Idea | Section here |
|---|---|---|
| BS-1 | Observation-alias impossibility witness | §10.3 |
| BS-2 | Schedule checking by partial-order reduction | §14.5 |
| BS-3 | Blinded qualification with α-spending | §14.5 |
| B1 | Order-independent scatter and replay determinism as a precondition for pairing | §13.3 |
| B2 | Model-form robustness and a work–energy audit of the simulator | §14.5 |
| B3 | Robot–building retrofit co-design | §11 |

**B. Killed hypotheses** (a register of ideas refuted and never to return):

| Tag | Killed idea | Refutation | Replacement |
|---|---|---|---|
| K-01 | Half-precision tensor-core factorization of one contact system | A rank-one example gives condition number 4,801 against a claimed ≤ 100; the throughput figure used was the sparse one | Factor-once multi-RHS |
| K-02 | Sending log-posynomial constraints to an integer-linear decision procedure | Log-sum-exp is convex, not linear | Geometric-program bounds plus interval boxes |
| K-03 | Assuming the tactic layer and multivariate formal export exist | Only `DTree` and `omega` exist | Kernel reflection |
| K-04 | 50 Hz 3D lattice-Boltzmann in every training world | Mach 0.35–5.2, relaxation margin 6e-6, 63.7 TB at 32k worlds | Thin film and slosh in training; bounded fluid on ≤ 64 worlds |
| K-05 | Clairvoyant ∀w ∃u barriers; inverted friction infimum; unboxed projection | Counterexample (1.25, 0.25) outside the box | ∃u ∀w, split cones, exact projection with KKT check |
| K-06 | Learning on CPU-only tensors | Device tags are metadata | Arena views |
| K-07 | Inner-ellipsoid feasible-region sampling; scalar-ball clearance | Cuts off boundary optima; fails rectangular robots in doorways | Exact synthesis; clearance complexes |
| K-08 | Unconditional miss-rate double counting; 14 strata; `(world, epoch)` pairing; a wrong orbit count of 83,333; a retracted reward-hacking bound | Recomputed | Conditional miss rate; 27 strata; `(world, replicate_id)`; 86,185 |

A killed-ideas register is a Sound device. BASE's Appendix C (parked and rejected, with scores) serves the same purpose; add NEON's rows to it.

**C. EV matrix and galaxy-brain cards.** The "EV filter" is one sentence listing everything that survived, with no scores; BASE's Appendix C matrix is the real one. The sixteen cards and their subjects:

| Card | Subject | My check |
|---|---|---|
| GB-01 | Curvature minus sign | Formula correct |
| GB-02 | Variational integrator energy stability | Consistent with second order; script absent |
| GB-03 | Three-tier Delassus storage sizes | Correct |
| GB-04 | Multi-RHS factor reuse: 13.6× on 96 rows and 64 right-hand sides | Arithmetic correct; baseline is a strawman; 96 rows exceeds the 48-row dense tier |
| GB-05 | Orbit counts 86,185 and 500,500 | Correct |
| GB-06 | Ballast quotient 735,471 → 24,546 | Correct |
| GB-07 | Energy stop budget 1.49 J ≤ 2.00 J, 50.5 ms | Correct |
| GB-08 | Thermal runaway cliff at 33.5 s | Not reproducible (21.1 s) |
| GB-09 | Wet-clay sinkage 9.79 cm at 8 cm track, 4.96 cm at 12 cm | Not reproducible from the stated law |
| GB-10 | Racing saves 42.67 → 16.30 GPU-hours (2.62×) | Arithmetic only |
| GB-11 | 27 contact strata, 162 transitions | Correct |
| GB-12 | Clearance complex size and 74 mm band | Size correct; band inconsistent |
| GB-13 | Exact sort-and-clip projection | Correct |
| GB-14 | 720 schedules → 8 | Correct |
| GB-15 | Replay determinism: 16.6/λ seconds; variance inflation 10× | Arithmetic correct; inputs illustrative |
| GB-16 | 10,700 retrofit kits | Correct |

**D. Worked examples.** A corrected `.fbot` (§10.1 here), a `.ftask` (§14.1 here) and an MCP session (§16 here). The appendix title promises an `.fmodule` example; none is given.

**E. Glossary and audit trail.** Ten glossary entries. The audit trail lists five review passes and what each fixed; its most useful content is the list of v1 errors (curvature sign, integrator sign and scaling, Delassus memory by a factor of nine, GPU learning on CPU tensors, Burnside miscounts), which are all cases where a confident formula in a plan was wrong.

---

# Assessments

## (A) Where NEON is better than the BASE plan

Ranked, most important first. Each item says why, and what BASE should do.

1. **Error control across a whole campaign, not one comparison.** BASE's e-processes make a single A/B valid under optional stopping. They do nothing about an agent that submits fifty candidates to the same held-out scenes until one passes. NEON's sealed confirmation lane on fresh world clusters, pre-committed requests and α_i = α/[i(i+1)] closes that hole, and the hole is exactly the one an optimizing agent will find. **Adopt** into §22.4 and §16.7 as a new verb `eval qualify`, an invariant ("discovery cannot read confirmation worlds") and NEON's 500-retry gate.
2. **Many different robots in one GPU batch.** BASE's device tier runs one model per environment, so a design population is a sequence of rebuilds. That is the workload a design studio most needs to be fast. **Adapt**: extend §14.3 and §20.5 so each world indexes one of K model variants inside padded capacities (the way it already indexes one of K scenes), add specialized kernels for survivors as an optimization, and take `XT-P2` whole, including the per-family-template control arm and the pre-declared willingness to lose to it.
3. **Sensors are part of feasibility and of symmetry.** BASE's symmetry group ignores sensors, payload and task; an equivariant policy trained under the mechanical group of a robot with a front-only lidar is constrained to something false. NEON's Γ_obs ≤ Γ_mech rule fixes that. Its observation-alias witness goes further and proves a mission unsolvable for want of a sensor. **Adopt** both into C-09, A-03 and `design check`; add the observability Gramian as an analysis beside A-14.
4. **Failures are shrunk to minimal counterexample worlds.** BASE randomizes from card uncertainty and reports tails; it never searches for the failure or reduces it. Legality-bounded adversarial search, a solvability oracle for the candidate's own body, and delta-debugging to ≤ 3 edits give an agent the smallest world that breaks its robot. **Adopt** as a new subsystem and format beside Arena.
5. **Attribution by intervention.** BASE's `explain` orders anomalies by time and calls the result `Estimated`. BASE already has snapshots, restore and bitwise replay; forking a snapshot under interventions and attributing over coalitions is a short step that turns a heuristic into a measurement. **Adopt** into §22.3 and §23, with the correction that design-parameter interventions re-run from the episode start.
6. **A safety claim that survives contact uncertainty.** The energy-budget tier needs no contact model, and BASE's energy ledger already measures every term. NEON also turns "weakest input wins" into a type that cannot be converted to a proof, and makes the no-authorization flags machine-readable. **Adopt** the `.fenergy` tier, the claim-product type and the three flags; **adapt** the shield as a runtime filter with a KKT check and backup, classed `Tested`.
7. **Rank robustness for designs, with consequences.** BASE re-tests a policy under perturbations and sets flags. NEON re-tests the ranking of finalists under contact-law, cone, stiffness, step and tolerance variants, audits the simulator for created energy, and blocks a `ModelConditional` result from spending prototype budget. It also audits what screening threw away, by sampling discarded strata and planting a winner. **Adopt** into §19.6, §22.4 and BET-05's gate.
8. **A mission language above reward terms.** Unit-normalized temporal-logic predicates, online past-time reward with offline future-time verdicts, explicit multi-rate clocks and a settled-pose reset cache. BASE has no notion of a mission verdict separate from reward. **Adopt** into §19.2 and §21.
9. **Two-sided navigation certificates, and the building as a design variable.** Clearance complexes return either a feasibility witness or an impossibility cut with the bottleneck named. The retrofit catalog then asks whether a twelve-dollar ramp beats a chassis redesign, with the policy frozen. BASE has route graphs and a passability analysis. **Adopt** both into §16 and A-07.
10. **Worlds whose state the robot changes, and sensors that fail the way real ones do.** Robot-advected films, slosh, glass multipath, steam, rolling-shutter skew, speckle dropout, and per-archetype hazard lists with real dimensions. BASE's interiors are geometrically careful but physically static and optically ideal. **Adopt** film (as Frontier with NEON's known answers and kill gate), slosh, the optical material fields and the hazard lists into §16–§17.
11. **Campaign deduplication with stated limits.** Exact orbit representatives under Γ_obs, a mission-observable digest with memoized results, an explicit invalidation contract for module edits with a positive and a negative control, and the limb port interface with the semidefinite caveat. BASE memoizes compilation but says nothing about when a cached proof or result may be reused. **Adopt** into §12.3 and C-12; drop the "decorated cospan" vocabulary.
12. **Gaits as orbits, and time beyond one episode.** Shooting, full-space Floquet stability, continuation in morphology and design gradients make A-04 an analysis instead of a table; lifecycle compression asks what the robot does in month eight. **Adapt**: compute orbits of the design under BASE's synthesized baseline controller on the compliant tier first; add lifecycle as Frontier.
13. **Constructive linkage synthesis.** BASE can analyze a closure but not design one. The bounded exact lane with circuit and branch certificates fits the tools as they are; continuation with `Complete` or `Partial` labels is an honest Frontier lane. **Adopt** lane 1 at M3–M4, lane 2 as Frontier on the CPU.
14. **Joint-level and actuator physics that decide real failures.** Stiction and backlash as rows in the solve, a spin-coupled friction cone, resistance rising with temperature and the runaway it causes, the voltage ellipse, a two-RC battery with aging. BASE's card ladder is the better container; these are the missing contents. **Adopt** into §13.3 and §15.4.
15. **The form of its experiments.** Competing arms, potency controls, a paired lower bound, and a named fallback for all sixteen bets (21 kill gates to BASE's 9), plus a measurement protocol for incumbent comparisons and a gate that freezes the incumbents first. **Adopt** the form for BASE's method gates (GATE-ATEL-01, -03, GATE-REFLEX-02, GATE-CORTEX-03).
16. **Conventions written as formulas.** NEON's table exposes a real inconsistency in BASE (§9 says the coadjoint is the negative transpose; Appendix D uses the transpose). **Adopt** the coadjoint row, the typed wrench maps, the reflection-parity formula, the curvature component formula with its wrong-sign control, and the hollow-shell rule, which prevents a factor-of-eighteen mass error.
17. **Named artifacts and an offline auditor.** Sixteen versioned formats with owner crates, a `std`-only verifier with a single certificate bundle, and a structured log per bead. **Adopt** a format registry in `registries/`, the bundle, and the log.
18. **A division of labour between the two machines.** BASE treats the Mac as a slower GPU with more memory. NEON gives it the world bank, proof and symbolic work, replay and the viewer, and gives the 4090s training, with only seeds and receipts crossing. Also the hardware fingerprint gate, the slab lease and the double-float canary. **Adopt** as policy in §20 and §28; orchestration is Frontier.
19. **Agent surface details.** A bounded synchronous wait on every long MCP tool, catalogs as resources, TUI panes for racing and budgets, physical overlays in the viewer. **Adopt** into §24.
20. **Manufacturing and firmware as outputs.** BASE stops at a scored design. Under a maximalist brief the build packet (parts, STEP, tolerances, harness) is a natural M5–M6 Frontier addition and the embedded target a Moonshot, both behind the false flags. **Adapt**; lift NONGOAL-04 only for exported, unauthorized artifacts.
21. **It noticed the `fs-qty` nightly coupling.** BASE requires `fs-qty` types in every public signature above layer 1. NEON identifies that this welds the core to `generic_const_exprs`. **Learn**: decide this at INT-TOOLCHAIN-01, and consider NEON's pure token crate for the same decoupling purpose.
22. **A register of killed ideas with the refuting calculation**, and a grounding table with a consequence column. **Adopt** both forms into Appendix C and §25.
23. **Skill artifacts with infeasibility witnesses.** A dual certificate when a body cannot produce a task wrench is a better answer than `not-synthesizable`. **Adopt** the witness into §18.5; treat the learned residual as a Moonshot.

## (B) Where the BASE plan is better than NEON

1. **It can be built from where the siblings are.** Consuming FrankenSim behind an airlock, with a stack lock, real admission records and a milestone-by-sibling firewall, is a plan. Lifting more than three million lines is not.
2. **The trust ladder.** A referee tier, a fidelity ledger, a promotion rule and teleport. NEON has a CPU twin for kernel parity and an evaluation tier for multiphysics, but no rule that a fast-tier result is provisional until promoted.
3. **Every design moves on day one.** Baseline estimator and controller synthesis has no NEON counterpart; without it NEON's loop is preflight, then learning.
4. **Version control for robots.** Typed edits with inverses and symmetry effects, lineage, diff, blame, bisect, a quality-diversity archive, registered predictions, reference-robot scorecards.
5. **The energy ledger**, with the numerical residual on its own line. NEON audits energy only as an exploit detector.
6. **Determinism engineering.** Dialect rules, no transcendental built-ins, contraction off, an exhaustive IEEE probe, anchored positions, in-kernel integer digests, five classes. NEON's kernel IR admits Sin, Cos, FMA fusion and a fast mode.
7. **Honesty about the proof tools**, typed proof classes, and the rule that "verified" is not a word. NEON's "proved once at M1" has no authoring route.
8. **A convex fast-tier contact solve** with a unique solution, a per-step error bound and a static launch graph. NEON's complementarity solve on the GPU has neither a convergence guarantee nor a fixed shape.
9. **Guards against false progress.** A negative control for every gate, no self-certification, live-dependency proofs, typed close reasons, a metamorphic tier generated from the mathematics, `xtask` capped at six checks, a first-session script.
10. **The agent surface in detail.** Envelope, exit codes, estimates before running, budgets, idempotency, capability scopes, namespaces, `next_actions`, token-budgeted summaries, stills with pixel inspection, the data-not-instructions rule.
11. **Provenance and law.** Actuator and interface cards with per-field provenance and uncertainty, randomization derived from them, import loss reports, reference robots and assets with licences, privacy inheritance for scanned places, the machine's compute rules.
12. **Mathematics NEON does not have.** Joints as subgroups with lattice-based mobility (Sarrus, Bennett), growth vectors and the ball-box cost, monotone co-design with a Kleene fixed point, max-plus gait timing, dimension-typed kernels with derivative programs, unit-complex joint coordinates.
13. **Sequencing.** M1 is a CPU studio that needs three siblings. NEON's M1 needs the proof library, modules, the IR and three code generators before a robot is simulated.
14. **Register.** BASE reads as a specification. NEON reads as a pitch: capitals, exclamation marks, "10×" in the title, multipliers to four significant figures from scripts that are not in the repository.

## (C) Conflicts and recommendations for the hybrid

| Topic | BASE | NEON | Recommendation |
|---|---|---|---|
| FrankenSim posture | Consume by path behind `frob-fsim`; lift is the kill fallback | Consume free crates, lift everything coupled to `fs-qty` | BASE. Add NEON's extra crates to the airlock list. Record the `fs-qty` nightly coupling as a named risk with a test at INT-TOOLCHAIN-01. |
| Toolchain | Core follows FrankenSim (`2026-07-06`); annexes have their own files | One workspace on `2026-08-31` | BASE. FrankenSim, frankensympy and fastmcp_rust are on three different nightlies today; one workspace cannot assume a common one. |
| Units | `fs-qty` at every public API | New stable `frob-units` | BASE by default; keep a `frob-units` escape hatch as the documented fallback if `fs-qty` cannot build on the core toolchain. |
| Crate layout | 24 core crates plus annexes for GPU, MCP, derivation, Lean | 51 flat crates in ten layers | BASE's structure and airlock. Add crates only where NEON brings a new bounded context: mission spec, adversarial search and diagnosis, gait atlas, lifecycle, synthesis, manufacturing, shields. |
| Naming | Marrow, Skeleton, Sinew, … ; `frob-design`, `frob-scene`, `frob-sense`, `frob-learn`, `frob-chronicle`, `frob-lemma` | AXIOM, LIE, CAS, … ; `frob-bot`, `frob-world`, `frob-sensors`, `frob-gym`, `frob-telemetry`, `frob-proof` | BASE's names. Take NEON's file extensions for the new artifacts (`.fmodule`, `.ftask`, `.fclear`, `.fenergy`, `.fmcw`, `.fskill`, `.fbuild`, `.frobcert`). |
| Design format | Canonical JSON term with constructors and typed edits | S-expression and JSON views, macros, declared symmetry | BASE's term and edits as the canonical form; add NEON's fields (shell thickness, power bus, budgets, levers, contact law, module import); offer the S-expression as a second lossless view, which settles OPEN-SKEL-1. Symmetry is computed; a declaration is an assertion to check. |
| Contact | Convex compliant Newton solve on fast tiers; exact Coulomb on the referee | Non-associated complementarity everywhere, Delassus tiers | BASE's convex tier as default. Add the 4D cone and the joint rows to it. Offer NEON's complementarity solve as an optional hard-contact batch law and as the written specification of the referee. Model-form rank checks compare the two. |
| Integrator | Semi-implicit Euler with a Cayley update; variational lane is Frontier | Variational step as primary with an explicit fallback | BASE's default; NEON's four-step algorithm fills in the variational lane. |
| Determinism | D0–D4, floating-point atomics banned, bit equality across vendors a measured hypothesis | Four classes, cross-vendor always unpaired, ordered scatter droppable | BASE's ladder and bans. Add NEON's refusal of paired statistics under an uncontrolled class. |
| GPU approach | Hand-written skeletons in one shared C dialect, derived arithmetic, `cudarc`, existing Metal gateway, annex workspaces | IR with loops, four generated targets, own CUDA leaf, upgraded Metal gateway, in-workspace | BASE's route (already working in the suite). Take NEON's IR metadata, the fixed-point types for a later embedded target, the arena lease, the binary cache, stream-ordered pools and graph capture. Treat the Metal gateway upgrade as an upstream ask, never as our bead. |
| Kernel IR | Straight-line programs with dimension types and derivatives | SSA with bounded loops, selects, transcendental ops, resource metadata | Merge: BASE's dimension typing and operator set, plus NEON's bounded loops and metadata. No Sin, Cos or FMA in batched kernels. |
| Proofs | Frankensympy certificates; Lean as a re-checked second opinion | Reflection in the franken_lean kernel for every obligation | BASE's classes and routes. Add reflection as a Frontier route to promote a few instance certificates to `Proved`, with soundness theorems written in stock Lean. Take NEON's obligation list as theorem cards; drop the three that are not provable. |
| Claims | Six (Proof, Known answer, Parity, Fidelity, Reality, Performance) | Five, with a product type | BASE's six plus "Design comparison"; add the product type. |
| Scenes | Distance-field bricks, heightfields, visual mesh separate from physics | BVH and sparse VDB, one scene for physics and rendering | BASE's bricks and manual interpolation (determinism). Add optical properties and fixture parameters. Keep the visual mesh separate. |
| MCP | Generated tools in clusters of ≤ 7, start/status/cancel, capability scopes | 42 tools in 7 clusters, bounded synchronous wait | BASE's registry and scopes with NEON's `sync_wait_ms` and catalog resources. |
| Viewer | Three.js with self-contained HTML | Native viewport plus a WebGPU packet | BASE's viewer first; NEON's overlays; a native viewport is optional. |
| Scope | No soft bodies, flight, firmware, in-hand manipulation in v1 | Rods, rotor port, tactile hands, firmware in | Under the maximalist brief, admit each as Frontier or Moonshot behind a flag and off the critical path (D-6). |
| Milestones | CPU studio first | Algebra and proofs first | BASE's order. NEON's features land from M3 onward. |
| Siblings | Read-only; asks filed upstream | Schedules an upstream change as a local bead | BASE. |

## (D) Doubtful or wrong in NEON: do not inherit

1. **The lift posture.** About 3.4 million lines of Rust (1.6 million without `fs-couple`) are "pre-classified as lift" with no budget, against a 378 kLOC plan. Two crates listed as free of `fs-qty` (`fs-spectral`, `fs-matdb`) depend on it. The gait atlas and lifecycle both rest on `fs-spectral`.
2. **"Python-verified" numbers.** The scripts are not in the repository. The thermal-cliff time is wrong for the stated parameters (21.1 s, not 33.5 s; exponent 0.00657/s). The soil sinkage pair does not follow from the stated law at equal load. The 74 mm clearance band mixes a 48-bin grid with a 32-bin half-angle. Do not pin KA-08 or KA-09 to these values; re-derive.
3. **"Proved once at M1."** No route is given for writing soundness theorems without tactics. Every latency budget in the obligation table is a guess. `PO-QUOT-1` (identical trajectories), `PO-CODEGEN-FW-1` (fixed-point firmware equivalence) and `PO-SHIELD-1` (a quantified barrier condition for high-degree-of-freedom dynamics) are not statements those checkers can decide. `INV-011` as written would fail on day one.
4. **Headline multipliers.** "90%–99% of futile simulations deleted", "≥ 80–90% of campaign work", "≥ 13×–100×", "13.61×", "6.41×", "14.69×". The 29.96× quotient factor is a property of one fixture built to have it. None is a measurement; several are operation counts.
5. **Tensor cores for contact and for policies.** A 48-row triangular solve with 64 right-hand sides and a 256-wide policy have little arithmetic intensity, and half or bfloat precision is a risk in both. The 13.6× compares against refactoring per right-hand side, which nobody does, and uses 96 rows where the dense tier stops at 48.
6. **GPU continuation and kernel-checked α-certificates.** Path tracking wants double precision; Metal has none. A polyhedral start system for the nine-point problem is a very large computation that NEON prices at zero. Certifying thousands of roots in exact arithmetic inside a proof kernel in half a second has no basis.
7. **Symplectic claims beyond a free rigid body.** The energy figures are for one body. The articulated scheme is a phrase, and the property is moot once dissipative contact impulses enter.
8. **The curvature test for a mecanum roller bug.** A base with misaligned rollers still has spanning curvature, like a differential drive; the defect is a rank drop in the connection, which BASE's growth vector reports and NEON's stage 4 would pass.
9. **"Reynolds averaging mishandles complex irreps."** It does not; it is exact and merely expensive. The typed blocks are a parameterization, not a correction.
10. **Refusing simultaneous multi-foot impacts.** That refuses trot, bound and pronk, whose touchdowns are simultaneous by symmetry.
11. **Ornament.** Quaternionic irreps (no robot symmetry group needs them); conformal geometric algebra for sphere intersections; "cellular sheaf" for a least-squares pose graph; max-plus cycle means to balance two machines; "decorated cospan", "sub-PROP", "Futamura", "Pearl-causal", "Wasserstein DRO" as labels on ordinary mechanisms; entropy as the state of one lumped thermal node. Keep the mechanisms; drop the labels.
12. **The entropy equation** as printed adds a separate σ_irr to terms that are already the irreversible production. Re-derive before it becomes a test.
13. **Gates that need things a simulator does not have.** A real-board timing receipt, physical bench validation of retrofit kits, eleven of twelve independent human reviewers, five real building scans, bench data for "validated" physics. Each should be an import lane with a recorded no-claim, as BASE's Reality lane is.
14. **Layering that its own bead graph contradicts.** The world crate (K2) needs GPU buffers (E0); the shield (K1) depends on the morphology engine (E0) and the estimator (E1); GPU continuation sits in K1; a K1 crate cannot call its fallback in V.
15. **Budgets.** 378 kLOC for roughly three times BASE's scope; "< 600 lines" of unsafe for a CUDA driver binding with pools, graphs and runtime compilation; "50 of 51 crates forbid unsafe" achieved by pushing the Metal FFI into a sibling.
16. **Hardware and memory assertions.** 140 MB per building through lossless compression of private buffers; 500–1,000 resident worlds; 800 GB/s; 165 TFLOPS per card. All unmeasured, and NEON's own `GATE-FORGE-00` exists to measure them.
17. **The kill clause that drops ordered scatter** if unordered atomics agree on 99.9% of windows. A replay that is bitwise 99.9% of the time is not bitwise.
18. **Fast-math switches in the kernel pipeline** (`--fmad=true`, FMA fusion, `FastApprox`, built-in Sin and Cos) alongside a three-way parity gate.
19. **Counterfactual forks for design parameters** applied a moment before failure. History-dependent state (winding temperature, the policy's own training) makes that a different question from "would this design have failed".
20. **Scheduling a change to a sibling repository as a local bead** (`BD-029a`), and depending on it for M2.
21. **Absolute statements about incumbents** ("zero" analytic derivatives, "zero" composition) and the unsupported "99.9% of random linkage mutations break closure". Also the legality prior "OSHA hallway ≥ 75 cm", which matches no figure I know; take dimensional norms from a cited source, as BASE's OPEN-HAB-1 requires.
22. **Latency slogans.** "0 ms topology-mutation stall", "< 1 ms compile", a 198 ms preflight summed from eight invented stage budgets, "< 5 µs" monitors.
23. **Third-party tools slipped in.** A `wgpu` viewport and FreeCAD validation appear without admission, against NEON's own purity rule.
24. **Duel scores as evidence.** "780/1000", "Rank #1", and the model names in the provenance table are not arguments for any mechanism.

