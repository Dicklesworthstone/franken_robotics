RESULT_STATUS: MATERIAL_INCREMENT

# P04-SCALE-hardware-envelopes (PQ4)

Labels: **F** fact with source, **A** assumption, **M** measured or computed in this pass. Nothing here is a
FrankenRobotics measurement. Every result is a range.

Shared inputs. F: RTX 4090 has 24 GB, 1,008 GB/s memory bandwidth, 72 MB L2, 82.6 TFLOPS FP32 (TechPowerUp,
PC Gamer, Tom's Hardware). F: M5 Ultra 1.2 TB/s (Apple support 128107). M (this Mac, `MTLDevice`, `sysctl`):
working-set limit 222.7 GiB, largest buffer 167.0 GiB, cores 12 + 24. F: plan defaults h = 2 ms, 4 Newton
iterations, control every tenth step (§14.4, §28.1). The plan states no row capacity.

## 1. Worlds

A (dominant): quadruped nv = 18, 13 bodies, 16–64 contacts, R = 60–280 rows; humanoid nv = 35, 30 bodies, 32–160
contacts, R = 125–700. Bytes (f32) = state + cache (61 words/body, 17/DoF) + three nv² matrices (M, H, L) + contacts
(24 words) + dense J (4·R·nv) + 12 words/row + 10 solver vectors.

| | J | M, H, L | cache | rows, contacts | total |
|---|---|---|---|---|---|
| Quadruped | 4–20 kB | 3.9 | 4.4 | 4–20 | 20–53 kB (mid 30) |
| Humanoid | 18–98 kB | 14.7 | 9.7 | 9–49 | 56–180 kB (mid 85) |

At 16–19 GB usable of 24 (A): quadruped 0.3–1.0 M worlds, humanoid 0.09–0.34 M. Mac at 167–223 GiB: 3–12 M and
1–4 M. PPO rollout storage (A: 23–100 kB/world) roughly halves these. The plan's batches (8,192 × 30 kB = 0.24 GB;
4,096 × 85 kB = 0.35 GB) use 1–2% of one card.

**Conclusion:** memory allows 10–100× more worlds than the plan uses; the real bound is throughput and iteration
latency, which cannot be set without a throughput-versus-N curve. Row capacity dominates.

## 2. Scene memory

A: volumes 300 m³ (home, plan), 19,200 m³ (warehouse, plan), 57,600 m³ (six storeys); surface areas 1,000 / 35,000 /
65,000 m² (×/÷2). Dense = 2V/s³ bytes. A narrow band occupies min(1, (A/V)·t·s), t = 8–20 voxels including brick
rounding.

| | 1 cm dense | 1 cm band | 2 cm dense | 2 cm band | 4 cm dense |
|---|---|---|---|---|---|
| Home | 0.6 GB | 0.16–0.4 GB | 75 MB | 40–75 MB | 9 MB |
| Warehouse | 38 GB | 6–14 GB | 4.8 GB | 1.4–3.5 GB | 0.6 GB |
| Building | 115 GB | 10–26 GB | 14.4 GB | 2.6–6.5 GB | 1.8 GB |

Dense variants in 12–16 GB of a 4090 / 190 GB of the Mac: home 2 cm 160–210 / 2,500; home 1 cm 20–27 / 320;
warehouse 4 cm 20–27 / 320; warehouse 2 cm 2–3 / 40; warehouse 1 cm 0 / 4; building 2 cm 0–1 / 13; building 1 cm
0 / 1. Sparsity gains V/(A·t·s): 1.5–11× at 1 cm, 1–5× at 2 cm, at most 3× at 4 cm; cluttered homes gain least.
Side check: 16 bits over ±10 m resolves 0.3 mm, which is 30 N at 10⁵ N/m, so the stored range must be truncated or
scaled per brick.

**Conclusion:** one scene per world is impossible (8,192 × 75 MB = 614 GB); shared scenes are mandatory; 1 cm
warehouses and buildings are Mac-only. Surface-to-volume ratio dominates.

## 3. Simulator-in-the-loop planning

ticks/s = T·h/(K·horizon). A 20 s episode at 50 Hz for one robot takes 1,000·K·horizon/(h·T).

| K, horizon, h | world-steps per tick | ticks/s at T = 0.5 / 2 / 5 M | 20 s episode |
|---|---|---|---|
| 256, 0.25 s, 5 ms | 12,800 | 39 / 156 / 391 | 26 / 6.4 / 2.6 s |
| 1,024, 0.5 s, 2 ms | 256,000 | 2.0 / 7.8 / 20 | 512 / 128 / 51 s |
| 4,096, 1.0 s, 2 ms | 2,048,000 | 0.24 / 0.98 / 2.4 | 4,096 / 1,024 / 410 s |

Real time needs at most T/50 = 40,000 world-steps per tick at T = 2 M. Against a policy rollout the cost factor is
K·horizon/(h·decimation) = 3×10³ to 2×10⁵; a 256-episode battery on the middle row is 9 GPU-hours against 1.3 s.
F: DIAL-MPC ran 2,048 samples × 20 nodes of 20 ms at 50 Hz on an RTX 4090; its 4,096-sample, 1 s, four-iteration
plan took 30 s per 2 s of motion (arXiv 2409.15610). The first setting at h = 2 ms needs 20 M world-steps/s.
A coarser inner step (10–20 ms) buys 5–10×; whether convex contact stays faithful there cannot be bounded without a
fidelity measurement. A reduced model (A: centroidal, 10⁸–10⁹ steps/s) gives over 500 ticks/s at K = 4,096 but is no
longer the simulator.

**Conclusion:** full-physics sampling control is a single-robot, coarse-step tool; as the default evaluator it
exceeds the envelope by 10³–10⁵×. K·horizon/h dominates.

## 4. CPU shadowing

GPU world-seconds per second = T·h: humanoid 0.5 M × 2 ms = 1,000 (plan target); 250 at a quarter of it. Referee:
24 × 0.2–1.0 = 4.8–24, so 0.5–2.4% (up to 10% for the slow GPU), or 0.24–1.2 twenty-second episodes per second.
Quadruped at 2 M: 0.1–0.6% (A: same referee speed).
F: MuJoCo C reports 650 k humanoid steps/s on an M3 Max and 1.8 M on a 64-core Threadripper 3995WX (MJX docs;
mapping by listed order): 28–46 k per core, 56–90× real time. A: an f64 batch tier at a quarter to all of that on
24 cores gives 0.4–1.7× the humanoid GPU target.

**Conclusion:** the referee shadows about 1%, so sampling is forced; an f64 CPU tier at incumbent speed could shadow
nearly every humanoid world. Referee speed dominates. The plan's `batch-cpu` target (150 k/s on 36 cores, 4.2 k per
core) is about 10× below that per-core rate.

## 5. Mixed precision

F: u = 4.9×10⁻⁴ half, 6.0×10⁻⁸ single, 1.1×10⁻¹⁶ double; half factorization with single working precision and
double residuals converges for κ₂ ≤ 10⁴ (strictly κ·u < 1, κ < 2×10³), GMRES-based refinement to 10⁸ with no speed
guarantee (Carson and Higham 2018). F: inertia-matrix condition grows O(N) to O(N⁴) (Featherstone 2004). F: SAP's
Hessian condition scales as 1/σ, default σ = 10⁻³ (arXiv 2110.10107).
M: G1 URDF, 35.1 kg, 29 joints, 201 poses; A: armature 0.0036–0.025 kg m², 8 sole points × 3 rows, all active.

| κ₂ | raw | unit-diagonal scaled |
|---|---|---|
| M without armature | 1.1–1.4×10⁵ | 280–580 |
| M with armature | 9.1–9.3×10³ | 160–300 |
| H, MuJoCo default impedance 0.9–0.95 | 3×10⁴–1×10⁵ | 2×10³–9×10³ |
| H, near-rigid (SAP β = 1, σ = 10⁻³) | 2–3×10⁶ | 2–5×10⁵ |

Half-precision Cholesky of scaled H: refinement contraction 0.19 median, 0.70 worst at 0.9; diverges in some poses at
0.95; breaks down in 40 of 40 near-rigid. Synthetic 15 kg quadruped (A): scaled κ(H) 100–360 soft, 1–2×10⁴
near-rigid. Payoff: halving H and L saves 4–6% of per-world bytes; Cholesky (n³/3 = 1,944 and 14,292 flops) is under
10% of the step.

**Conclusion:** reject half-precision factorization; humanoid contact sits at or past the threshold and nothing is
gained. Near-rigid, single precision has κ·u = 0.14–0.19 raw and 0.01–0.03 scaled, so the f32 solve needs diagonal
scaling. Contact stiffness dominates.

## 6. Heterogeneous batches

Work per robot ∝ L^p (p = 1 kinematics, 2 Jacobian, 3 Cholesky). Padding overhead Ω_p = L_max^p / E[L^p], with
1/r ≤ Ω_p ≤ r^−p for r = mean/max: r = 0.9 gives 1.1–1.4×, 0.75 gives 1.3–2.4×, 0.5 gives 2–8×. With nv 18 and 35:
50:50 → 1.3 / 1.6 / 1.8×; 90:10 → 1.8 / 3.0 / 4.5×; plus 1% of a 66-DoF hybrid → 3.3× to 23×.
Buckets of ratio 1.25 cap it at 1.25–1.95× (8 buckets over 12–66 DoF), but each bucket is its own static step plan.
A: 60 launches × 5–10 µs = 0.3–0.6 ms, equal to 600–1,200 world-steps at T = 2 M. Bucketing pays above roughly 10⁴
worlds per bucket with Ω > 1.3; below 10³ worlds per morphology, two or three padded buckets are cheaper.
F (suggestive, policy size confounded): 50-robot MJX training ran 5×10⁹ steps in 40 A100-hours = 35 k env-steps/s
against 106–417 k for single robots (E011).

**Conclusion:** the largest robot, not the mean, sets the cost; launch overhead per plan must be measured.

## 7. Pretraining

F: 1.6×10⁹ steps, 16 robots (arXiv 2409.06366); 5×10⁹, 50 robots (2509.02815); over 2×10¹² simulation steps for about
1,000 experts on 160 GPUs × 5 days = 19,200 GPU-hours (2505.05753).
Env-steps/s = T/10 = 50 k / 200 k / 500 k (quadruped, physics only; mixed batches lower by §6).

| Steps | GPU-hours at 50 k / 200 k / 500 k |
|---|---|
| 1.6×10⁹ | 9 / 2.2 / 0.9 |
| 5×10⁹ | 28 / 7 / 2.8 |
| 2×10¹² | 11,000 / 2,800 / 1,100 (26–257 days on two dedicated 4090s) |

The default envelope of one device-hour (OPEN-SPARK-2) is 0.2–1.8×10⁹ steps.

**Conclusion:** 10⁹–10¹⁰-step pretraining costs 1–30 GPU-hours and needs owner approval; 10¹²-step campaigns are out
of envelope. Inconsistency: 400 k env-steps/s at decimation 10 needs 4.0 M world-steps/s, twice the plan's physics
target.

## 8. Memory bandwidth

A (dominant): traffic per world-step ≈ 4 iterations × (3 passes over J + row scalars + H, L) + other stages =
100–500 kB quadruped (mid 240), 0.2–1.5 MB humanoid (mid 730). The batch exceeds the 72 MB L2 above 2,400 quadruped
or 850 humanoid worlds, so kernels stream from memory. Ceiling = η·BW/traffic, A: η = 0.5–0.8.
4090: quadruped 1–8 M (mid 2.1–3.4 M), humanoid 0.3–4 M (mid 0.7–1.1 M). M5 Ultra: 1.2× those, shared with the CPU.
Compute, A: 1–3×10⁵ and 0.6–2×10⁶ flops per step at 5–20% of peak: 14–165 M and 2–27 M steps/s.

**Conclusion:** on the 4090 the step is bandwidth-bound, by 2–80× (mid about 10×); the plan's 2.0 M and 0.5 M targets
sit within 1–4× of the ceiling, and T = 5 M needs at most 100–160 kB per step. On Metal, 25% of a 4090 needs
120 GB/s, a tenth of bandwidth; Metal throughput cannot be bounded without measuring command-buffer overhead and GPU
arithmetic rate.

## Candidate claims

| ID | Statement | Type | Basis | Assumptions | Replacing measurement |
|---|---|---|---|---|---|
| P04-1 | Memory admits 0.3–1.0 M quadruped or 0.09–0.34 M humanoid worlds per 4090; it never binds world count. | Derived estimate | §1 | Row capacities | Peak bytes at admitted capacities; throughput versus N |
| P04-2 | One scene per world is infeasible; a 4090 holds 160–210 homes or 2–3 warehouses at 2 cm and no 1 cm warehouse. | Derived estimate | §2 | Volumes, areas | Brick occupancy of compiled archetypes |
| P04-3 | A 16-bit field needs a truncated or per-brick range. | Derived | 20 m / 65,536 | Stiffness 10⁵ N/m | Force quantization on a resting block |
| P04-4 | Full-physics sampling control costs 3×10³–2×10⁵× a policy rollout; real time needs under 40,000 world-steps per tick at T = 2 M. | Arithmetic, published anchor | §3 | 50 Hz, decimation 4–10 | Tracking error versus (K, horizon, inner h) |
| P04-5 | A referee at 0.2–1× real time shadows 0.1–2.4% of GPU world-seconds on 24 cores. | Arithmetic | §4 | Plan throughput targets | Referee real-time factor per core |
| P04-6 | The plan's `batch-cpu` and humanoid GPU targets are at or below published CPU MuJoCo throughput on comparable hosts. | Comparison | §4 | Benchmark humanoid simpler than G1 | MuJoCo `testspeed`, G1 and Go2, both hosts |
| P04-7 | Half-precision factorization should be rejected for contact solves. | Computed, theorem | §5 | Simplified contact rows; one robot | κ(H) histogram over rollouts with the plan's law |
| P04-8 | The f32 Newton solve needs unit-diagonal scaling when contact is near-rigid. | Computed | §5 | Same | f32 against f64 Newton-step error |
| P04-9 | Padding overhead lies in [1/r, r⁻³], set by the largest robot; buckets pay above about 10⁴ worlds each. | Bound, estimate | §6 | Launch cost 5–10 µs | Mixed against bucketed throughput |
| P04-10 | Multi-embodiment pretraining at 10⁹–10¹⁰ steps is 1–30 GPU-hours; 10¹² steps is 1,100–11,000. | Arithmetic on published counts | §7 | T/10 env-steps/s | Env-steps/s with policy on a mixed batch |
| P04-11 | §28.1's 400 k env-steps/s and 2.0 M world-steps/s conflict at decimation 10. | Consistency check | 400 k × 10 | Env-step = control tick | None; define the unit |
| P04-12 | The GPU step is bandwidth-bound; plan targets are within 1–4× of the 4090 ceiling. | Derived estimate | §8 | Traffic per step, η | Bytes touched per launch; sweep across N = 2,400 |

Sources beyond the case file: arXiv 2409.15610, 2409.06366, 2509.02815, 2505.05753, 2110.10107;
eprints.maths.manchester.ac.uk/2562; Featherstone, IJRR 23(9) 2004 (abstract only);
mujoco.readthedocs.io/en/stable/mjx.html; support.apple.com/128107. G1 URDF: local vendored copy.
