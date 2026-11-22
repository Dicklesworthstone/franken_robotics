# EV-pack-H-19 — a thin layer over an incumbent gets most of it (third alternative)

Provenance: FETCHED-raw = source text downloaded and quoted exactly; FETCHED-sum = page read through the fetch
tool, short quotes as returned; MEASURED = run this session on this M5 Ultra; MEMORY = unverified.

## Claim under test

An agent studio wrapped around MuJoCo Warp or Newton gets at least 80% of the benefit, so a new engine is not the
source of the ten-fold (falsifier: at least three load-bearing mechanisms need engine-internal properties that
incumbents lack).

## Evidence against (properties incumbents lack)

- Replay across CPU and GPU. Newton: "By default, a Newton simulation on the GPU is not bit-for-bit repeatable";
  run-to-run results "usually differ across GPU models and between the CPU and CUDA backends"; "Newton only tests
  RUN_TO_RUN"; SolverMuJoCo on GPU "is not a bitwise trajectory guarantee"; SolverKamino: "No" (FETCHED-raw,
  https://newton-physics.github.io/newton/latest/concepts/determinism.html).
  MuJoCo Warp: "Is MJWarp on GPU deterministic? No." (FETCHED-raw, MuJoCo Warp docs; issue 562 open since July 2025).
  MuJoCo: "Exact reproducibility is only guaranteed within a single version, on the same architecture." (FETCHED-raw).
- Mixed-topology batches: `body_parentid (nbody,)` and `jnt_type (njnt,)` are not per-world in MuJoCo Warp
  (FETCHED-raw, `types.py`). Isaac Lab: "Articulation variants must have the same links, joints, collision-body
  count, and names." (FETCHED-raw).
- No Metal tier: MuJoCo Warp is "designed for NVIDIA hardware" (FETCHED-sum, repository README).

## Evidence for (already present or layerable)

- Same-backend determinism: Newton run-to-run mode gives "bitwise identical results" on one GPU model, driver and
  build; SolverSteel "costs about as much as the default" (same page). MuJoCo CPU batch API: "Results are
  bit-identical to a serial loop over one mjData per simulation, at any thread count." (FETCHED-raw, `doc/programming/simulation.rst`).
- Forking: after `mj_copyState(model, src, dst, mjSTATE_INTEGRATION)`, "stepping src or dst will produce identical
  results" (same file).
- Mixed topology exists: "Kamino supports heterogeneous worlds" (FETCHED-sum, https://arxiv.org/html/2603.16536v1).
  Parameter sweeps are per-world data: `body_pos (*, nbody, 3)`, `body_mass (*, nbody,)`, `jnt_axis (*, njnt, 3)`.
- Energy: MuJoCo Warp `energy: potential, kinetic energy (nworld, 2)` (FETCHED-raw, `types.py`).
- Sampling controllers: MJPC, Hydrax, Judo, BAMPC (see EV-pack-H-01).
- The new engine cannot promise cross-backend bitwise replay on Metal either: "No API selects or queries the FP32
  arithmetic rounding mode, and none controls or queries denormal handling." (FETCHED-sum, Apple DTS, https://developer.apple.com/forums/thread/844598).

## Kill tests (audit and arithmetic)

| Property | Engine-internal? | Incumbent status | Load-bearing for the ten-fold? |
|---|---|---|---|
| Bitwise replay across CPU and GPU | Yes | Absent everywhere | No. Fork, autopsy and splitting need same-backend repeatability |
| Run-to-run replay, fork by address | No, state arrays plus a mode switch | CPU: yes. Newton GPU: yes, at a cost. MuJoCo Warp GPU: partial | Yes, and available |
| Mixed-topology batch | Yes for MuJoCo Warp | Kamino and ManiSkill3 have it; slower, not deterministic | Only for mixed-topology populations |
| Port-level energy ledger | Totals and actuator power layerable; closure to round-off needs integrator internals | Totals only | No, diagnostic |
| Typed evidence, refusals | No | Layerable | Yes, and layerable |
| Metal GPU tier | Yes | Absent | Only on the M5 |

- Count of load-bearing mechanisms needing engine-internal properties that incumbents lack: one clear
  (mixed-topology batches with deterministic forks), one conditional (Metal tier). **Fewer than three: the falsifier
  does not fire.**
- Determinism cost on the incumbent, community-reported: "the sorting costs 15 to 19% at 8192 worlds";
  "ATOMICS costs 84% on humanoid on main today" (FETCHED-sum, https://github.com/google-deepmind/mujoco_warp/issues/562).
- Scale: on the M5 the incumbent is already fast without a GPU. MuJoCo 4.0.1, f64, 36 threads (MEASURED, machine
  under load): 1,259,397 humanoid steps/s, against a documented 3.35 M for MuJoCo Warp on a GPU.
- Conflation: none. Rival: this hypothesis is the rival.

## Cheapest discriminating test

A two-week spike on MuJoCo Warp plus MuJoCo C: (1) snapshot and restore 1,024 worlds with every determinism flag
set, assert bitwise run-to-run equality over 1,000 steps, record the throughput penalty; (2) 64 designs in
per-topology buckets against a Kamino heterogeneous batch; (3) energy-ledger closure residual. H-19 dies if
deterministic forking costs more than 2× and bucketed dispatch is more than 4× slower per generation on the
benchmark's design distribution. Cost: about 5 engineer-days, under 10 GPU-hours.

## Recommended state and the claim that survives

**ACTIVE, and the strongest rival.** On CUDA the documented evidence supports a thin layer for sampling control,
forking, run-to-run replay, parameter batches, energy totals and typed evidence. A new engine is justified only by
(a) a Metal GPU tier and (b) mixed-topology batches with deterministic forks. The 80% figure is unmeasured.

## Open points

Whether benchmark missions change topology; the M5's share of the workload; Newton deterministic-mode overhead for
legged scenes (documented qualitatively only); the plan's own cross-ISA determinism ask (FD-FSIM-07) is open.
