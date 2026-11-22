# EV-pack H-20 — Proof-guided controller synthesis

Session `RS-20261121-frob-fable-10x`. Marks: FETCHED = read this session; MEMORY = not re-read.

## Claim under test

Sum-of-squares funnels, hybrid-zero-dynamics gaits and barrier shields with checked certificates yield deployable safety filters for real designs, through reduced-order templates and a bound relating template and full-order behaviour. Falsifier: certificates exist only for models of about ten states or fewer and the template-to-full bound exceeds the safety margin.

## Evidence against (excerpts)

1. Sum-of-squares scale. Methods in the literature "typically involve systems of very modest dimension (approximately 5-10 states)"; for an N-link pendulum "The SOS programs obtained for 2N > 12 are too large to run (due to memory" limits) (FETCHED, Majumdar, Ahmadi, Tedrake, https://groups.csail.mit.edu/robotics-center/public_papers/Majumdar14a.pdf). Verified regions for walking limit cycles cover the rimless wheel and the compass gait, four states (FETCHED, Manchester et al., https://arxiv.org/pdf/1010.2247). Funnels: a twelve-dimensional quadrotor at "approximately 20-25 minutes" per funnel; grid-based reachability stalls "beyond approximately 5-6 dimensional state spaces" (FETCHED, Majumdar and Tedrake, https://arxiv.org/pdf/1601.04037). Verified neural Lyapunov control: a planar quadrotor, 1.1 to 8.9 hours (FETCHED, Yang et al., https://arxiv.org/pdf/2404.07956).
2. The one 30-state legged result is not locomotion. ATLAS balancing by the cheaper diagonally-dominant relaxation assumes "the interface of the right foot and the ground is mediated by a pin joint at the toe", ignores link collisions, and "we do not take into account input saturations"; dynamics are a cubic Taylor expansion with a quadratic Lyapunov function (FETCHED, Majumdar, Ahmadi, Tedrake). No contact switching, no impacts.
3. The template-to-full bound is not computable in advance. "the precision ε depends on (a supremum over) the input to the reduced-order model, u2, external disturbances τext, and the state of the full-order model x1. This makes it difficult to compute ε a-priori." (FETCHED, Kurtz, Wensing, Lin, https://arxiv.org/pdf/2006.09921). The earlier version assumes "all joints are actuated and that the system is mounted to a single foot in contact" (FETCHED, https://arxiv.org/pdf/1909.09693).
4. Hardware practice adds uncertified heuristics. Cassie: "the closed-loop system is unstable … Leg angle adjustment is added to stabilize" (FETCHED, Gong et al., https://arxiv.org/pdf/1809.07279). ANYmal stepping stones: barrier constraints sit in the whole-body tracking layer and in "an MPC layer that operates on a purely kinematic model of the system", and on hardware "Both constraints are rarely violated" (FETCHED, Grandia et al., https://arxiv.org/pdf/2011.00032). Rarely is not never.

## Evidence for (excerpts)

- The relaxation does reach 30 states: "Each iteration takes approximately 4.5 minutes and convergence occurs in 4-5 iterations" (FETCHED, Majumdar, Ahmadi, Tedrake).
- Barrier shields run on real legged hardware: ANYmal trotting on stepping stones (Grandia et al.); a velocity-level barrier on a reduced-order model with a tracking controller, "with a Drone and a Quadruped in hardware experiments" (FETCHED, Molnar et al., https://arxiv.org/pdf/2109.09047). Their guarantee is conditional on a tracking assumption.
- Hybrid-zero-dynamics gaits walk on Cassie and Vision 60 (see EV-pack H-05).

## Kill tests

1. Artifact: nameable. 2. Lever: FAILS; no number, baseline or gate, and no stated link to decision time. 3. Scale: fails for full-order legged models; passes for templates of 4 to 12 states. 4. Foundation: partial. `fs-sos` has interval-checked certificates and `certify_roa` (`src/roa.rs`), but with a fixed quadratic Lyapunov function, smooth polynomial fields, a dense solver costing "O(N⁴) for an N × N Gram block" (`src/sdp.rs`), and a largest region-of-attraction example of two states (Van der Pol). Missing: hybrid and time-varying funnels, controller synthesis, diagonally-dominant relaxations, barrier synthesis. 5. Conflation: FAILS as worded. "Deployable … for real designs" presents a template proof as a robot fact. 6. Rival: barrier shields are a quadratic program over any simulator; FROST and Drake exist. Not engine-internal.

## Cheapest discriminating test

On a 12-DoF quadruped in an existing simulator, wrap a trained policy in a velocity-level barrier filter (keep-out zone and speed limit on a single-integrator base model). Over 10⁴ randomized episodes record the worst and 99.9th-percentile tracking error between commanded and realized base velocity, and barrier violations. Kill if the margin implied by that error exceeds the mission tolerance (0.3 m for an indoor robot) or violations exceed 10⁻³ per episode. Separately, run `certify_roa` on polynomial models of 4, 6 and 8 states; kill the funnel branch if 6 states fails or takes over 10 minutes. Cost: two engineer-days, under 5 GPU-hours.

## Recommended state and the claim that survives

NARROWED, with the funnel branch DEFERRED. Survives: reduced-order barrier shields whose template-to-full margin is measured statistically in simulation and labelled as such, strongest for wheeled and tracked robots and slow legged motion; hybrid-zero-dynamics gaits as nominal gait generators. Refuted: checked certificates for full-order legged locomotion, and the word "deployable". The mechanism improves evidence quality; no source shows it shortens the path to a decision.

## Open points

Measured tracking-error distributions for learned locomotion policies; whether a shield reduces the rare-event sampling needed by H-07; certified treatment of contact switching.
