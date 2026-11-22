# EV-pack H-17 — Mission to design by solving

Session `RS-20261121-frob-fable-10x`. Marks: FETCHED = read this session; MEMORY = not re-read.

## Claim under test

Three solvers produce candidate designs for a mission in minutes: (a) kinematic synthesis by polynomial methods, (b) trajectory optimization with design variables, (c) monotone co-design over catalogs. Falsifier: more than half of synthesized designs fail referee evaluation, or the problems do not solve within budget at 12 or more degrees of freedom.

## Evidence against (excerpts)

1. (a) Synthesis systems are far beyond the sibling solver. Four-bar nine-point path synthesis: "there were 8,214 unique finite roots found by tracking 25,922 homotopy paths, It is expected that 83,430 homotopy paths must be tracked in order to obtain all roots. This is compared to 286,720 for the best known multihomogeneous homotopy"; a six-bar case needed "a 1.5 million solution set" (FETCHED, Plecnik and Fearing, https://ptolemy.berkeley.edu/projects/robotics/~ronf/PAPERS/mplecnik-idetc16.pdf). The four-bar system is one planar loop with eight unknowns. frankensympy caps real solving at `MAX_VARIABLES: usize = 8` and `MAX_DEGREE: u64 = 256` (`crates/fsym-solvers/src/real_rur.rs`), Gröbner bases at 1,024 polynomials and 10⁶ pending pairs (`crates/fsym-polys/src/groebner.rs`), and contains no homotopy continuation (grep for homotopy, path tracking, Bertini: no hits). The benchmark exceeds the degree cap by a factor of 32.
2. (b) Results are local refinements of one robot for one motion. Small cases are slow: a two-joint monoped, "for 10³ trajectory optimization problems, the overall time was about 9 hours, taking on average 33.72[s] for each Crocoddyl solver call" (FETCHED, Fadini et al., https://iris.unitn.it/retrieve/1e741b92-4e6b-482f-9908-e6d34a717224/ICRA21_fadini.pdf). The contact schedule, topology and task are fixed in advance; the plan is checked by tracking it with a PD controller whose gains come from a 20 × 20 grid search (FETCHED, Dinev et al.), not by a referee on held-out scenes.
3. No source read reports what fraction of optimized designs pass an independent closed-loop evaluation. The first half of the falsifier is untested in the literature.
4. (c) Monotone co-design needs each component expressed as a monotone map between functionality and resources (FETCHED, Censi, https://arxiv.org/pdf/1512.08055); published robotics use is component selection for an autonomous vehicle (FETCHED, Zardini et al., https://arxiv.org/pdf/2011.10756). Gait performance is not monotone in link lengths, so locomotion enters only through a surrogate that must come from simulation.

## Evidence for (excerpts)

- (b) at 12 actuated degrees of freedom: Solo quadruped, design vector of 4 to 17 parameters (leg lengths, attachment points, trunk shape, motor mass, gear ratio), "the worst-case time we observed is 600 seconds for CMA versus 252 seconds for our approach (both for the 17-DOF jumping co-design task)" on 8 threads; "the cost improvements remain in simulation within 10% of the ones found during optimization" (FETCHED, Dinev et al., https://arxiv.org/pdf/2103.04660).
- Hardware-validated quadrupeds from design-and-motion optimization by the implicit function theorem (FETCHED in part, Ha et al., https://la.disneyresearch.com/wp-content/uploads/Joint-Optimization-of-Robot-Design-and-Motion-Parameters-using-the-Implicit-Function-Theorem-Paper.pdf; no timings found).
- (c) solves catalog problems in seconds (MEMORY; no timing fetched).

## Kill tests

1. Artifact: nameable for (b) and (c). 2. Lever: "in minutes" has no baseline; the pass rate is the real gate. 3. Scale: passes (CPU). 4. Foundation: (a) FAILS, no continuation solver and caps far too low; fallback is numerical optimization of linkage dimensions. (b) partial: `fs-ascent` has SQP, augmented Lagrangian and continuation tests, `fs-adjoint` has an `src/ift.rs` module (listed, not read), but no collocation, multiple-shooting or DDP trajectory optimizer was found in `fs-opt`, `fs-ascent`, `fs-control`, `fs-adjoint`. 5. Conflation: at risk; plans assume rigid contact and ideal actuators. 6. Rival: FAILS as a new-engine advantage. Crocoddyl with Pinocchio does (b) today; an open monotone co-design solver exists. A thin layer over them delivers this, which supports H-19.

## Cheapest discriminating test

With Crocoddyl, generate 20 candidate designs for three missions (trot at 1 m/s, 0.3 m step, 5 kg payload) on a 12-DoF quadruped, each within 10 minutes on 8 threads. Referee each: train a policy for 20 minutes on one RTX 4090 and score on held-out terrain against the nominal design. Kill if more than 10 of 20 fail or do not beat nominal, or if solves exceed budget. Cost: three engineer-days, about 10 GPU-hours.

## Recommended state and the claim that survives

NARROWED. Survives: (b) as local refinement, 12 degrees of freedom, up to 17 continuous design parameters, fixed contact schedule, 4 to 10 minutes on 8 CPU threads; (c) for actuator, battery and compute selection. (a) is REFUTED on foundation beyond toy sizes. The pass rate at the referee is unknown and is the deciding number.

## Open points

Pass rate of optimized designs under closed-loop evaluation; sensitivity to the chosen contact schedule; wheeled and tracked classes have no co-design results in the sources read.
