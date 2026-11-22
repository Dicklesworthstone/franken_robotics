# EV-pack H-23 — a family of designs per dispatch; paired finite-difference design gradients

Session `RS-20261121-frob-fable-10x`. Sources read 2026-11-21. FETCHED = read this session; MEMORY = not re-read.

## Claim under test

Evaluating a neighbourhood of designs (sweep, finite-difference stencil with common random numbers, or evolutionary generation) in one heterogeneous batch under a shared policy cuts sequential agent rounds in a design campaign by at least 5×. Falsifier: half as many rounds or more, or gradients too noisy to ascend.

## Evidence against (excerpts)

- Rival, for parametric families. A stencil or sweep changes parameters, not topology, and incumbents batch that today. "To enable batched simulation with different model parameter values, many" Model fields have a leading batch dimension. MuJoCo Warp docs, https://mujoco.readthedocs.io/en/latest/mjwarp/ [FETCHED]
- Only mixed topologies are missing from incumbents. "Articulation variants must have the same links, joints, collision-body count, and names." and "Newton views currently require a uniform body layout across worlds." Isaac Lab docs, https://isaac-sim.github.io/IsaacLab/main/source/how-to/multi_asset_spawning.html [FETCHED]. "This does not enable arbitrary heterogeneous articulations or different actual object counts." Newton PR 4312, closed unmerged 2026-11-02, https://github.com/newton-physics/newton/pull/4312 [FETCHED]
- Common random numbers decouple under contact. "all deep RL methods produce chaotic dynamics in the complex high-dimensional environments"; a 5×10⁻⁴ degree perturbation "causes the systems to significantly diverge after 50 steps." https://arxiv.org/html/2410.10674v1 [FETCHED]. The landscape can be "extremely non-smooth or fractal", "such that there does not exist gradient to be estimated in the first place." https://arxiv.org/abs/2310.15418 [FETCHED, abstract]
- For discontinuous outcomes (a fall, a contact-mode switch) the variance of a paired finite difference grows as 1/h in step size h. [MEMORY: textbook result on finite-difference sensitivity estimators; seen only in search snippets this session.]
- A gradient under a shared policy points toward designs that policy handles: the population "converges to similar designs that are easier to steer with a shared universal controller." https://arxiv.org/abs/2502.10862 [FETCHED]
- Surrogate ascent overshoots. "its predictions can be unreliable when the search moves too far from the experienced distribution." and "More search iterations can sometimes even hurt when over-optimizing the critic." https://arxiv.org/html/2606.00702 [FETCHED]
- Rounds are not compute. Published end-to-end gains from shared-policy co-design are 3–4×: a wall-clock cut of "67.4%" for 200 evaluations (https://arxiv.org/abs/2606.07424 [FETCHED, abstract]) and "approximately 78% fewer simulation steps" (Loki, https://arxiv.org/html/2505.21665 [FETCHED]).

## Evidence for (excerpts)

- Loki "explores ∼780× more designs than prior evolution-based approaches" through shared policies. [FETCHED]
- "designing each additional design takes only about 11–22 minutes of search time", after "the initial training run of about 77–99 hours". [FETCHED, 2606.00702]
- Batching cuts rounds almost linearly in black-box search: with batch q "it only needs 512/q iterations", and performance "decreases a little, but is still comparable to the sequential" method up to q = 128. https://arxiv.org/html/2411.16206v2 [FETCHED]
- Finite-difference search does ascend in contact-rich simulation for policy parameters: "matching state-of-the-art sample efficiency on the benchmark MuJoCo locomotion tasks", with "a high variability in performance". https://arxiv.org/abs/1803.07055 [FETCHED, abstract]

## Kill tests

- Artifact: pass. Lever: weak; "rounds" is neither GPU-hours nor wall-clock, the terms of the thesis falsifier.
- Scale: a 20-parameter central stencil × 64 seeds is 2,624 worlds; fits in 24 GB.
- Foundation: mixed-topology generations need H-03; the shared policy needs H-02. Fallback: parametric families only.
- Conflation: none.
- Rival: kills the engine-specific part for sweeps and stencils.

## Cheapest discriminating test

On MuJoCo Warp, one quadruped, 12 design parameters, fixed policy. Measure the variance ratio of paired against independent central differences at horizons of 25, 50, 100, 250 and 500 steps and perturbations of 1%, 3% and 10%; measure gradient sign agreement over 20 replicate batches. Run 20 ascent steps against a same-budget CMA-ES generation, and verify five end points with specialists. Kill: variance ratio below 2 at 250 steps or more, sign agreement below 70%, or no specialist-verified gain. Cost: about 4 GPU-hours of rollouts plus about 5 for specialists.

## Recommended state and the claim that survives

NARROWED. A 5× cut in rounds is plausible for batched population search. It is not engine-specific for parametric families, and the measured compute gain in the literature is 3–4×, not 10×. The paired finite-difference gradient is unproven for full episodes; expect it to hold only over short horizons and smooth metrics.

## Open points

- No source measures paired finite-difference variance for design parameters in legged simulation.
- The batch result quoted is for a generic optimizer, not robot design.
- Excerpts were pulled through a fetch tool; values need a hand check.
