# EV-pack H-04 — Supercompiled per-design kernels

Session `RS-20261121-frob-fable-10x`. Marks: FETCHED = read this session; SUMMARY = search-engine summary only; MEMORY = not re-read.

## Claim under test

Partial evaluation of the Lie-algebraic recursions on one fixed design, plus equality saturation, gives at least 1.5× the throughput of the universal kernel, parity with the best incumbent, and at most 60 s compile per design. Falsifier: below 1.2×, or over 60 s.

## Evidence against (excerpts)

1. The recursions are a minority of the incumbent's step. MuJoCo Warp `--event_trace`, humanoid, 8,192 worlds (FETCHED, https://github.com/google-deepmind/mujoco_warp/pull/1119, 2026-02-05), nanoseconds per world-step: `step: 262.33 … kinematics: 9.94, com_pos: 5.67, crb: 13.30, collision: 9.84, make_constraint: 38.31 … com_vel: 5.93, rne: 7.08 … fwd_acceleration: 11.21, solve: 123.55 … euler: 9.29`. Everything a design-specialized recursion could touch (kinematics, com_pos, crb, com_vel, rne, fwd_acceleration) sums to 53.1 ns, 20% of the step. An infinitely fast specialized recursion gives 262.33/209.2 = 1.25×; a 2× faster one gives 1.11×. The manipulation scene `aloha_pot` (FETCHED, https://github.com/google-deepmind/mujoco_warp/pull/1014): step 513.39, collision 162.63, solve 226.10; recursion share 16%, ceiling 1.19×. The stated 1.5× is arithmetically out of reach on an incumbent-like step; the 1.2× falsifier is met unless the recursions become more than five times faster.
2. Generated code does not reliably beat a good generic library. "While we do not rely on code-generation software, our algorithm is 30% faster for the inverse dynamics (5 µs versus 7 µs) and 60% faster for the direct dynamics (8 µs versus 20 µs)." (FETCHED, Carpentier and Mansard, RSS 2018, https://www.roboticsproceedings.org/rss14/p38.pdf). "for the floating base robots, RobCoGen was the slowest library by far, and Pinocchio gave the fastest runtimes, at 1.5µs and 3.5µs for HyQ and Atlas" (FETCHED, Neuman et al., IROS 2019, https://people.csail.mit.edu/devadas/pubs/iros2019.pdf).
3. Compile time. Cold MuJoCo Warp JIT: "Total JIT time: 95.91 s", of which `mujoco_warp._src.smooth … took 4301.18 ms (compiled)`; the dense solver kernels take 13 to 17 s each (FETCHED, https://github.com/google-deepmind/mujoco_warp/pull/989, RTX 6000 Ada). Symbolic expansion swells at humanoid size: "FROST crashes after more than an hour of computing the mass matrix of the Atlas model" (FETCHED, Rosa and Lynch, https://arxiv.org/pdf/2006.03785). Code generation "becomes time and memory-consuming" at n = 36 (SUMMARY, arXiv 2302.06001).

## Evidence for (excerpts)

- "RobCoGen gave the fastest runtimes for forward dynamics for all robot models, at 1.1µs, 2.4µs, and 5.9µs for iiwa, HyQ, and Atlas", attributed to loop unrolling (FETCHED, Neuman et al.).
- GRiD, generated CUDA from a URDF: "as much as a 7.2x speedup over a state-of-the-art, multi-threaded CPU implementation, and maintains as much as a 2.5x speedup when accounting for I/O overhead"; this is GPU against an 8-core CPU for batched gradients, not specialized against generic on one device (FETCHED, https://arxiv.org/pdf/2109.06976).
- Generated derivative code speeds trajectory optimization "by up to 500%" against numerical differentiation (FETCHED, Giftthaler et al., https://arxiv.org/pdf/1709.03799).

## Kill tests

1. Artifact: nameable, nothing built. 2. Lever: stated. 3. Scale: passes. 4. Foundation: FAILS without fallback. No e-graph or equality-saturation code in frankensim (grep of `crates/`); no GPU physics backend (only an out-of-workspace CUDA audio renderer, `apps/instrument-lab/render-gpu`); `fs-mbd/src/articulated.rs` has CPU Featherstone dynamics only. 5. Conflation: none. 6. Rival: FAILS. MJX-JAX already specializes on structure: "Numpy arrays in `mjx.Model` and `mjx.Data` are structural fields that control the output of JIT compilation" (FETCHED, https://raw.githubusercontent.com/google-deepmind/mujoco/main/doc/mjx.rst); MuJoCo Warp treats the model as data with no per-design compile.

## Cheapest discriminating test

Three arms on one 12-DoF quadruped, one CPU core then one RTX 4090, 8,192 worlds: (a) padded universal kernel, (b) unpadded data-driven kernel, (c) fully unrolled constant-folded kernel written by script, no e-graph. Measure whole-step time and compile time. Kill if (c)/(b) is below 1.2× on the whole step or (c) compiles in more than 60 s. A zero-cost pre-test is the Amdahl sum above on the engine's own trace. Cost: two engineer-days, under one GPU-hour.

## Recommended state and the claim that survives

REFUTED as a throughput lever for the GPU gym: ceiling 1.19× to 1.25×, expected 1.05× to 1.15×, below its own falsifier. Any gain over the padded kernel belongs to H-03 (not padding). Residue worth keeping: specialized derivative kernels for the CPU solvers of H-05 and H-17, where rigid-body derivatives dominate.

## Open points

The share is the incumbent's; an engine with a much cheaper contact solver raises it. No published GPU comparison of specialized against data-driven kernels for the same robot was found.
