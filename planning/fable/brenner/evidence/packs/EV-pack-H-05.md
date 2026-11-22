# EV-pack H-05 — Gait continuation across design changes

Session `RS-20261121-frob-fable-10x`. Marks: FETCHED = read this session; MEMORY = not re-read.

## Claim under test

A stable periodic gait with a fixed contact sequence is transported across design changes by continuation of the hybrid periodic orbit, at no more than 1% of the cost of a training run per design point, giving cost of transport and Floquet stability as functions of design. Falsifier: failure on more than half of 10% perturbations at full order, or one orbit solve over 10 minutes.

## Evidence against (excerpts)

1. The 1% lever does not close against a fast incumbent. Training baseline: a quadruped policy "in under four minutes, and in twenty minutes for uneven terrain" on one GPU (FETCHED, Rudin et al., https://arxiv.org/pdf/2109.11978). One percent is 2.4 to 12 s. Published full-order orbit solves: Cassie, 20 DoF, "24.51 seconds" on a laptop (FETCHED, Hereid et al., https://arxiv.org/pdf/1807.06614); Vision 60, 36 states, "262.13 s for walking, 42.69 s for ambling and 116.05 s for trotting" with initial guesses supplied (FETCHED, Ma, Hamed, Ames, https://arxiv.org/pdf/1909.08124). That is 2% to over 100% of a training run, not 1%.
2. Continuation is not guaranteed to arrive. "turning points in σ … or bifurcation points, where the gait family branches into multiple solution paths, may be encountered. Consequently, reaching σend from an initial stationary point χstart is not guaranteed." The studies are a compass gait and RABBIT, "ten state variables" (FETCHED, Raff, Flaßkamp, Remy, https://arxiv.org/pdf/2410.09512).
3. An orbit is not an accepted controller. On Cassie, "With the gait library implemented, the closed-loop system is unstable … Leg angle adjustment is added to stabilize the closed-loop system." (FETCHED, Gong et al., https://arxiv.org/pdf/1809.07279). "A heuristic foot placement controller was also applied to stabilize the lateral motion" (FETCHED, Hereid et al.). "not every solution of the NLP can lead to experimental success"; stabilizing one trot by matrix inequalities took 6 iterations where "each iteration … takes approximately 30 minutes" (FETCHED, Ma, Hamed, Ames). Floquet multipliers therefore describe a chosen feedback law, not the design.

## Evidence for (excerpts)

- Continuation in physical parameters at humanoid size exists. Atlas, "a 71-dimensional state-time-control space S (68 states, 2 control parameters, and 1 switching time)": "Our Mathematica code took 1.5 hours to compute 250 gaits", then a model-parameter homotopy "took 1.8 hours … and computed 48 gaits" (FETCHED, Rosa and Lynch, https://arxiv.org/pdf/2006.03785). That is 22 s and 135 s per gait point.
- Gait libraries are cheap in bulk: 1,331 Cassie gaits, "an average-per-gait computation time of approximately 4.1 seconds" of wall-clock on 72 virtual CPUs, about 5 thread-minutes each (FETCHED, Hereid et al.).
- With structure, a quadruped orbit solves in "3.96 second" on average, "0.039 second" per iteration (FETCHED, Ma and Ames, https://arxiv.org/pdf/1909.08560).

All solve times pass the 10-minute falsifier. No source reports a success rate under 10% design perturbations; that number does not exist in the literature read.

## Kill tests

1. Artifact: nameable. 2. Lever: stated, but 1% fails against the 4-minute baseline. 3. Scale: passes (CPU). 4. Foundation: partial. Present: `fs-orbit` (shooting, pseudo-arclength continuation, monodromy; `shooting.rs` disclaims "hybrid saltations"), `fs-time/src/hybrid_sensitivity.rs` (saltation correction), `fs-spectral/src/floquet.rs` (interprets a supplied monodromy only), `fs-mbd/src/articulated.rs` and `articulated_impact.rs`. Missing: a hybrid multiple-shooting orbit solver on articulated models, and derivatives of the articulated dynamics with respect to state and design (no such public functions found in `fs-mbd/src/articulated*`). 5. Conflation: at risk; rigid impacts and fixed contact order are model facts. 6. Rival: FROST and C-FROST already build gait libraries, but regenerate symbolic models per design.

## Cheapest discriminating test

With existing tools (C-FROST or Crocoddyl with Pinocchio), take one 12-DoF quadruped trot. Apply 20 random 10% perturbations to link lengths and masses, one at a time and jointly; warm-start from the parent orbit. Record success rate and time. Then train a policy on 10 of the designs and compare the ranking by cost of transport. Kill if success is under 50%, median time exceeds 10 minutes, or rank agreement (Kendall τ) is under 0.5. Cost: three engineer-days, about 4 GPU-hours.

## Recommended state and the claim that survives

NARROWED. Surviving claim: warm-started continuation of a fixed-contact-sequence orbit costs roughly 4 to 260 s of CPU per design point at 20 to 68 states, and yields a nominal gait and cost of transport as a screening signal. It is 1% of training only against baselines of 20 minutes or more, and it does not produce a referee-ready controller.

## Open points

Success rate under design perturbation; whether orbit cost of transport ranks designs as trained policies do; wheeled and tracked robots are outside this method.
