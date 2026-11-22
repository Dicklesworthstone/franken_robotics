# EV-pack-H-03 — topology as data

Provenance: FETCHED-raw = source text downloaded and quoted exactly; FETCHED-sum = page read through the fetch
tool, short quotes as returned; MEASURED = run this session; MEMORY = unverified.

## Claim under test

One compiled kernel simulates robots with different trees and joint classes in one batch at no more than 2× the
per-world cost of a specialized kernel, so at least 256 distinct designs run per dispatch (falsifier: above 4× at
up to 64 links, or slower than sequential specialized runs at populations of 64 or more).

## Evidence against

- A rival already ships it. "Kamino supports heterogeneous worlds, allowing for batched simulation of structurally
  diverse robots on a single GPU." Mechanism: "All GPU kernels operate on 2D thread grids"; "threads whose index
  exceeds the per-world count exit early via a mask check". Price: "relatively lower throughput compared to the
  reduced-coordinate solvers available within Newton" (FETCHED-sum, https://arxiv.org/html/2603.16536v1).
- ManiSkill3: "different articulations with different DOFs across parallel environments" (FETCHED-sum, https://arxiv.org/html/2410.00425v2).
- Parameters are already data in MuJoCo Warp (FETCHED-raw, `mujoco_warp/_src/types.py`, main):
  `body_pos (*, nbody, 3)`, `body_mass (*, nbody,)`, `body_inertia (*, nbody, 3)`, `jnt_axis (*, njnt, 3)`,
  `actuator_gear (*, nactuator, 6)`. A fixed-topology sweep can be batched today, at one host compile per variant
  ("1 call per new field instance", same docs).
- Large padded trees are the incumbents' weak case: "Improving performance for single connected mechanisms beyond
  ~60 DoFs remains an active priority" (FETCHED-raw, https://mujoco.readthedocs.io/en/latest/_sources/mjwarp/index.rst.txt).
- No incumbent publishes a throughput penalty for heterogeneous or padded batches. The 2× figure has no measurement behind it.

## Evidence for

- Structure is not data in the leading incumbent: `body_parentid (nbody,)`, `jnt_type (njnt,)`, `dof_parentid (nv,)`
  carry no world dimension (same file). Docs 3.4.0 and 3.5.0: "Heterogeneous worlds, for example: per-world meshes or
  number of degrees of freedom, are not currently available." (FETCHED-raw). The latest docs add per-world assets
  only, with "the maximum number of geoms" padded.
- Isaac Lab 3.0: "Articulation variants must have the same links, joints, collision-body count, and names."
  (FETCHED-raw, https://docs.nvidia.com/isaaclab/isaac-lab/spawning-multiple-assets.md).
- Newton: "Support for fully heterogeneous simulations is still under active development"; "not all solvers are able
  to simulate them"; "SolverMuJoCo compiles one geom layout for all worlds" (FETCHED-sum, https://newton-physics.github.io/newton/latest/concepts/worlds.html).
- Genesis: "steps many identical copies of a scene at once" (FETCHED-raw, parallel_simulation.md).

## Kill tests (arithmetic)

- Padding. Degrees of freedom measured from Menagerie models: Go2 18, G1 35. In one batch padded to 64: linear
  passes cost 64/18 = 3.6× and 64/35 = 1.8×; a dense quadratic stage (64/18)² = 12.6×; a dense cubic stage
  (64/18)³ = 45×. **A single padded batch breaks the 4× falsifier for small robots on any super-linear stage.**
  With three size buckets (24, 40, 64) the worst case is 1.7× linear and about 4× cubic, so 2× also needs
  tree-sparse factorization that skips masked slots.
- Memory (my estimate, about 60 KB per padded world: 128×64 dense Jacobian 33 KB, 64×64 Hessian 16 KB, state 10 KB).
  256 designs × 16 worlds = 0.25 GB: fits. 256 designs × 2,048 rollouts (H-01 inside H-03) = 524,288 worlds = 31 GB:
  **does not fit a 24 GB 4090**; fits the M5.
- Rival: Kamino and ManiSkill3 batch mixed topologies; MuJoCo Warp batches parameters. The increment over
  incumbents is mixed-topology populations with few worlds per design, in a reduced-coordinate solver.
- Lever: per-design switching cost in MuJoCo Warp (compile, `put_model`, graph capture) is unpublished. If it is
  about 1 s, 256 designs cost about 4 minutes per generation against seconds of simulation. Unmeasured.
- Conflation: none. Foundation: not assessed.

## Cheapest discriminating test

One 4090, existing engines. 64 distinct topologies, 64 worlds each, 1,000 steps: (a) MuJoCo Warp sequentially,
timing the switch overhead; (b) a 64-DoF max-template model with locked joints against the specialized model;
(c) Kamino heterogeneous batch. Kills: bucketed sequential (a) within 2× of the best heterogeneous wall-clock, or
(b) above 4×. Cost: 2 to 3 engineer-days, under 5 GPU-hours.

## Recommended state and the claim that survives

**NARROWED.** Parameter variation as data is incumbent practice and no source of the ten-fold. The surviving claim:
mixed-topology populations in size-bucketed batches (at most 2× with sparse masked factorization, unproven) save
per-design dispatch overhead when worlds per design are few (under about 100). "256 designs per dispatch" holds
for policy rollouts, not for per-design sampling MPC on a 4090.

## Open points

Switch overhead in MuJoCo Warp; Kamino's measured throughput gap (figures only, not read); how many benchmark
missions change topology at all.
