# EV-pack-H-11 — an always-on CPU referee on unified memory

Provenance: FETCHED-raw = source text downloaded and quoted exactly; FETCHED-sum = page read through the fetch
tool, short quotes as returned; MEASURED = run this session on this M5 Ultra (load average 17 to 28 from other
jobs, so rates are lower bounds); MEMORY = unverified.

## Claim under test

On unified memory, idle CPU cores shadow a sample of GPU worlds with the referee tier at zero copy cost and catch
tier-sensitive behaviour during training at no more than 5% throughput cost (falsifier: reads stall the GPU, or
coverage is below 0.05% of world-seconds).

## Evidence against

- Shared storage is not free concurrent access. "When either the CPU or GPU changes the contents of the resource,
  you're responsible for synchronizing access to the texture from the other participant. Ensure that all changes you
  schedule on either the CPU or GPU for a resource that uses shared memory complete before accessing that resource
  on the other processor." (FETCHED-raw, https://developer.apple.com/documentation/metal/mtlstoragemode/shared.md).
  Live state buffers cannot be read mid-flight; a snapshot buffer and a completion handler are required.
- Trajectory shadowing is meaningless beyond a few steps. "Any numerical difference between states, no matter how
  small, will become significant upon integration, especially for systems with contact. Contact events have high
  Lyapunov exponents" (FETCHED-raw, MuJoCo docs, `doc/computation/index.rst`, Reproducibility).
- Bitwise comparison against a Metal GPU is not available. Apple DTS, September 2026: "No API selects or queries
  the FP32 arithmetic rounding mode, and none controls or queries denormal handling." (FETCHED-sum, https://developer.apple.com/forums/thread/844598).
- A thin layer does it. Isaac Lab's Newton backend exposes `use_mujoco_cpu`, "already reproducible"
  (FETCHED-raw, https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism.md); MuJoCo Warp has
  `mjw.get_data_into` to copy device data into `mjData`.

## Evidence for

- Tier sensitivity is documented: "Solver settings, including iterations, collision detection, and small friction
  values may be sensitive to differences in floating point representation." (FETCHED-raw, https://mujoco.readthedocs.io/en/latest/_sources/mjwarp/index.rst.txt).
- CPU rates, MuJoCo 4.0.1, f64 (MEASURED). DeepMind humanoid (27 DoF, 5 ms): 50,612 steps/s on 1 thread,
  1,186,904 on 24, 1,259,397 on 36. Unitree G1 (35 DoF, 2 ms): 24,153; 549,621; 575,651.
- GPU reference: "Pure Warp (No JAX FFI) | Humanoid 3.35M" steps/s (FETCHED-raw, `doc/mjx.rst`; GPU model not stated;
  no Metal figure exists because MuJoCo Warp is "designed for NVIDIA hardware").

## Kill tests (arithmetic)

- Coverage = CPU rate / GPU rate. One thread: 50,612 / 3.35 M = 1.5%. Twenty-four threads: 35%. With a referee
  10× costlier than default MuJoCo: 0.15% and 3.5%. The 0.05% falsifier fails only if the referee is about 700×
  slower than MuJoCo on 24 threads. **The gate is too weak to kill anything.**
- Copy cost. G1 integration state is about 135 floats = 540 B. Shadowing 256 worlds at 50 Hz moves 6.9 MB/s.
  A PCIe link carries gigabytes per second (MEMORY). **Zero copy saves nothing measurable; unified memory is not
  the lever**, and the mechanism works on the two-4090 machine by explicit readback.
- What can be detected. A 5-point gap in success rate between tiers needs n ≈ 16·p(1−p)/δ² = 576 (p = 0.9) to
  1,600 (p = 0.5) episodes per tier, that is 11,520 to 32,000 world-seconds. At 24 threads the G1 rate is
  549,621 × 0.002 = 1,099 world-seconds per second: one valid check every 10 to 29 s, or 2 to 5 minutes for a
  10× costlier referee.
- Lever: it adds cost and removes no training or search. Its value is fewer false accepts. No ten-fold.
- Rival: MuJoCo Warp plus MuJoCo C is this mechanism on CUDA. Fails kill test 6 as a differentiator.
- Conflation: referee agreement is parity between two simulators, not agreement with a robot.
- Scale: passes.

## Cheapest discriminating test

(a) Metal microbenchmark: a compute kernel rewriting a 100 MB shared buffer per command buffer, a blit of 256 rows
to a snapshot buffer, CPU threads reading it after the completion handler while 24 cores run MuJoCo. Kill:
GPU kernel time rises more than 5% against idle CPU. (b) On a 4090: MuJoCo Warp G1, 4,096 worlds, with and
without CPU shadow threads. Kill: over 5% loss or under 1% coverage. Cost: 1 engineer-day each.

## Recommended state and the claim that survives

**NARROWED.** A sampled snapshot referee, on any backend, covering at least 1% of world-seconds and reporting
one-step defects and outcome-distribution gaps, at a cost of at most 5% (unmeasured). It is a correctness guard.
It is not a source of the ten-fold, and "zero copy on unified memory" should be deleted from the claim.

## Open points

Speed of the plan's own referee (FrankenSim `fs-mbd`, G1 at 480 Hz) is unknown, and coverage scales with it.
Power and memory-bandwidth contention between CPU and GPU on the M5 is unmeasured. Policy inference for
closed-loop shadow worlds needs a CPU-to-GPU round trip per control tick.
