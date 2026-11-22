RESULT_STATUS: MATERIAL_INCREMENT

# P01-AMDAHL-where-time-goes (PQ1)

Published numbers support three things. Policy training is about 93% of wall-clock in the one agent co-design
loop that publishes a breakdown. A faster simulator alone is capped below 8× on training and at 2–5× on a whole
iteration. Nobody has published a phase measurement of a design loop on the late-2026 stack, so the denominator
of a "tenfold" claim does not exist yet and the project has to measure it.

## 1. Phase budget

**Table A. Published figures per phase.** Sources S1–S10 are listed at the end.

| # | Phase | Figure | Source or label |
|---|---|---|---|
| 1 | Propose and diagnose (LLM agents) | ≈4 min and ≈$0.50 per round of 16 candidates (GPT-5.2) | S1 §3.1 |
| 2 | Realize the design | Text to mesh 1–2 min; mesh to URDF ≈30 s | S4 |
| 2 | Same, parametric edit | 0.5 min | ASSUMPTION |
| 3 | Compile, JAX stack | "JIT compilation can be slow (1-3 minutes on Playground's tasks)" | S2 §VI |
| 3 | Compile, Warp stack | Cold kernels "can take a minute"; no measured figure | S7 |
| 3 | Start-up, Isaac Sim | 4.4–6.3 s with cached shaders, 263 s without; warehouse scene load 27 s (RTX 5080) | S8 |
| 3 | Scene build, Isaac Lab, 4,096 worlds | NOT PUBLISHED | — |
| 4 | Train, quadruped | Go1 flat joystick within 5 min, 2× RTX 4090; ANYmal under 4 min flat, 20 min rough, one GPU | S2 §IV-B1 (E011); S6 |
| 4 | Train, humanoid | Berkeley Humanoid under 15 min; G1 and T1 under 30 min; 2× RTX 4090 | S2 §IV-B2 |
| 4 | Train, manipulation | LEAP hand ≈2,080 s on one RTX 4090, ≈670 s on 8× H100 | S2 Fig. 5 |
| 4a | Environment step as a share of training | G1 rough, 4,096 worlds, RTX 4090: 94k FPS step only, 82k with inference and training, so 87%. Shadow hand 85%. Cartpole 46% | S5, ratio derived |
| 4a | Same, from pixels | Physics, rendering and inference together are 9% (Cartpole) and 43% (Franka) of training time | S2 §IV-D1 |
| 5 | Evaluate | 128 rollouts of 1,000 steps per design; seconds at 10^5 steps/s | S3; time DERIVED |
| 5 | Final confirmation | 5 retrainings of the winner | S1; S9 |
| 6 | Human diagnosis | NOT PUBLISHED | — |
| 7 | Trained candidates per campaign | 80 (D2C; Eureka per run); 125 (RoboMoRe as run in S1); 1,600 (RoboMorph); 100 new per generation for 20–50 generations (Text2Robot) | S1; S9; S3; S4 |
| 8 | Campaign compute | ≈40 GPU-h, 8 Titan V, ≈1 h per round (D2C); ≈200 GPU-h per seed, 8 A100 (RoboMorph); under one day on 8 A100 (Eureka run) | S1; S3; S9 |
| 8 | Per trained candidate | 7.5 GPU-min (RoboMorph); ≤30 GPU-min (D2C) | DERIVED from row 8 |

**Table B. Composite iteration on two RTX 4090 and the Amdahl bounds.** This composes phases from different
stacks. It is not a measurement. Propose 0.3 min (row 1, amortized over 16), realize 0.5 (ASSUMPTION), compile 2
(midpoint of 1–3), evaluate 0.1 (DERIVED).

| | Quadruped (train 5 min) | Humanoid (train 30 min) |
|---|---|---|
| Total per iteration | 7.9 min | 32.9 min |
| Training share | 63% | 91% |
| Compile share | 25% | 6% |
| Bound: environment step infinitely fast (87% of training) | 2.2× | 4.8× |
| Bound: training deleted | 2.7× | 11× |
| Bound: training and compile deleted | 8.8× | 37× |

## 2. Candidate claims

**P01-1.** In the D2C loop, RL evaluation takes ≈1 h per round and LLM inference ≈4 min, so training is about
93% of round wall-clock.
Type: observation. Evidence: S1 §3.1, §5. Assumptions: none. Scope: Brax, toy morphologies, 8 Titan V, 16
candidates per round. Reversed by: a loop on a current stack where agent time equals or exceeds training time.

**P01-2.** At fixed sample count, an infinitely fast environment step speeds training by at most 7.8× (G1), 6.7×
(Shadow hand) and 1.9× (Cartpole). A simulator-speed advantage alone therefore cannot deliver tenfold end to end.
Type: inference. Evidence: S5; E016. Assumptions: the incumbent's learner share is like Isaac Lab's; "environment
step" includes observation and reward code, so the physics-only bound is lower. Scope: state-based PPO on Isaac
Lab 2.3 with PhysX. Reversed by: an incumbent measurement where the environment step is at least 90% of whole
iteration wall-clock.

**P01-3.** For quadruped-class tasks, incumbent training is already about 5 min, so no single phase yields
tenfold: deleting training gives about 2.7×, and training plus compile about 9×. For humanoid-class tasks training
is about 91% and deleting it gives about 11×.
Type: inference. Evidence: Table B. Assumptions: the composite; that the 5 min excludes JIT (S2 Fig. 11 suggests
its curves include it, which would raise the compile share). Scope: per iteration, simulation only. Reversed by: a
measured ledger with different shares.

**P01-4.** Campaign cost is trained candidates × cost per candidate × seeds. Published campaigns train 80 to
several thousand candidates for 40–200 GPU-hours. Tenfold is most reachable as a product across these terms (for
example 3× fewer trainings and 3× cheaper training).
Type: inference. Evidence: rows 7–8. Assumptions: the terms are independent. Scope: says nothing about whether
BET-05 or BET-06 deliver those factors. Reversed by: a campaign where proposal quality does not change the number
of trainings needed.

**P01-5.** The plan's targets put the dominant phase at parity or worse: at most 15 min on one RTX 4090 against 5
min on two. The learner, 13% of incumbent training time on GPU, has no GPU implementation in FrankenTorch.
Type: inference. Evidence: E016, E007, S2, S5. Assumptions: targets are met and not exceeded. Reversed by:
measured training at or under incumbent wall-clock on the same host.

**P01-6.** Build the incumbent baseline loop and its phase ledger at M0–M1, before committing to mechanisms, and
state tenfold only as the ratio defined in section 3.
Type: recommendation. Evidence: P01-1 to P01-5; case file §4, first unknown. Reversed by: an independent published
phase budget for the same stack.

## 3. Metric and benchmark protocol

**Metric: time to qualified design (TTQD).** Wall-clock from a written brief to the first design and controller
that pass a frozen acceptance test.

- **Briefs.** At least six, covering legged, wheeled, tracked and hybrid robots on terrain and interiors. Two are
  held out until the final run.
- **Acceptance test.** Success rate over held-out randomized scenarios in a referee simulator that neither
  pipeline trained in. Thresholds are fixed before any run.
- **Arms.** Incumbent: Playground or mjlab on MuJoCo Warp, or Isaac Lab, plus a thin tool layer. Both arms use
  host `trj` (E002), the same agent model and the same token budget. Incumbent configurations are published.
- **Reporting.** Median TTQD over at least five campaign seeds with a bootstrap interval; runs that fail within
  budget are censored, not dropped. Cold and warm caches are reported separately. Also GPU-hours, tokens, and best
  acceptance score at 1 h and 8 h.
- **Phase ledger.** Propose, realize, compile, train, evaluate, diagnose and idle must sum to wall-clock within 2%.
- **Claim.** Tenfold holds if the ratio of median TTQD is at least 10, with the lower 95% bound above 5, on at
  least four of six briefs including both held-out ones.
- **Kill gate.** A ratio under 3 at M4 withdraws the claim.

## 4. Checked and not established

- **No phase measurement on the late-2026 stack.** The mjlab report (S10) gives no timings. I found no training
  wall-clock for MuJoCo Warp on an RTX 4090.
- **Isaac Lab scene build at thousands of worlds.** Not published. One issue title reports "10+ minute load time"
  (github.com/isaac-sim/IsaacLab/issues/4151); I did not read the thread.
- **Per-morphology recompilation on Warp.** I could not establish whether MuJoCo Warp recompiles kernels when the
  robot topology changes. If it does not, the compile row is near zero for the incumbent.
- **Human diagnosis time.** Not found. One survey reports 92% of 24 practitioners design rewards by trial and
  error (ojs.aaai.org/index.php/AAAI/article/view/25733), without durations.
- **Whether Playground's training minutes include JIT.** Ambiguous in S2.
- **D2C's 40 GPU-hours.** Stated "per environment"; the arithmetic fits per seed.
- **Share of candidates that fail trivially.** Not searched; BET-05 depends on it.
- **Reading method.** Figures were extracted through a page-summarizing fetch. The rows from S1, S2 and S5 were
  read twice. One misread was caught: Eureka "55 iterations", where the repository default is 5. Re-read the PDFs
  before citing these in the plan.

## Sources

- S1 Debate2Create: https://arxiv.org/abs/2510.25850
- S2 MuJoCo Playground: https://arxiv.org/abs/2502.08844
- S3 RoboMorph: https://arxiv.org/abs/2407.08626
- S4 Text2Robot: https://arxiv.org/abs/2406.19963
- S5 Isaac Lab performance benchmarks: https://isaac-sim.github.io/IsaacLab/v2.3.2/source/overview/reinforcement-learning/performance_benchmarks.html
- S6 Rudin et al., Learning to Walk in Minutes: https://arxiv.org/abs/2109.11978
- S7 Newton tutorial: https://docs.nvidia.com/learning/physical-ai/getting-started-with-newton/latest/newton-fundamentals/core-concepts.html
- S8 Isaac Sim benchmarks: https://docs.isaacsim.omniverse.nvidia.com/6.0.1/reference_material/benchmarks.html
- S9 Eureka: https://arxiv.org/abs/2310.12931 and https://github.com/eureka-research/Eureka
- S10 mjlab: https://arxiv.org/abs/2601.22074
