# EV-pack-H-01 — zero-training evaluation by sampling MPC

Provenance: FETCHED-raw = source text downloaded and quoted exactly; FETCHED-sum = page read through the fetch
tool, short quotes as returned; MEASURED = run this session on this M5 Ultra (under background load); MEMORY = unverified.

## Claim under test

Sampling MPC with the batched full-order simulator as its model completes tasks for at least 80% of well-formed
legged, wheeled and tracked designs with no training, at no more than 3 GPU-minutes per 20-second episode
(falsifier: under 60%, or over 10 GPU-minutes on one RTX 4090).

## Evidence against

- Cost per tick, DIAL-MPC example configs (FETCHED-raw, `raw.githubusercontent.com/LeCAR-Lab/dial-mpc/main/dial_mpc/examples/`):
  Go2 trot `Nsample: 2048, Hsample: 16, Ndiffuse: 2, dt: 0.02, timestep: 0.02`; H1 jog `Nsample: 2048, Hsample: 25,
  Ndiffuse: 4`. The planning model steps at 20 ms, 4 to 10 times coarser than a training or referee step.
- Humanoids: "requires an increase from 30 to 70 parallel sample trajectories", which "prevents real-time execution";
  "these humanoid results are currently limited to simulation" (FETCHED-sum, https://arxiv.org/html/2511.19204v3).
  MJPC's humanoid "is tasked with standing", not walking (FETCHED-raw, https://ar5iv.labs.arxiv.org/html/2212.00541).
- Harder terrain is not real time: DIAL-MPC crate climbing "in total takes 30 seconds to generate a 2-second
  full-state motion plan" (FETCHED-sum, https://arxiv.org/html/2409.15610v1), which is 15× slower than real time,
  or 5 GPU-min per 20 s episode.
- Fragility: in 2511.19204, walking success over 10 seeds is 10/10 with Hermite splines, 6/10 cubic, 0/10 quadratic.
- Hand-built priors: DIAL-MPC uses a swing-foot reference, CMU's MPPI Raibert heuristics (Table I of 2511.19204).
  No corpus-level success rate over varied morphologies was found. No simulator-in-the-loop sampling MPC for tracked
  robots was found (one search; absence, not proof).
- "one can only control what can be simulated much faster than real-time" (FETCHED-raw, MJPC paper, section 5.3).

## Evidence for

- "achieve real-time 50Hz control" on a Go2 with "an RTX 4090 GPU"; also "a Unitree H1 humanoid in simulation"
  (FETCHED-sum, https://arxiv.org/html/2409.15610v1).
- CPU only: "roll out 30 samples", 0.4 s horizon, 100 Hz, real Go1, box pushing 9/10 (FETCHED-sum, https://arxiv.org/html/2409.10469v1).
- Training baseline: Go1 flat "within 5 minutes (2x RTX 4090)"; "under 30 minutes for the Unitree G1"
  (FETCHED-sum, https://arxiv.org/html/2502.08844).

## Kill tests (arithmetic)

- Steps per tick: Go2 2048×16×2 = 65,536; H1 jog 2048×25×4 = 204,800. A 20 s episode at 50 Hz is 1,000 ticks:
  6.6e7 and 2.0e8 model steps.
- GPU time: real time on Go2 means 20 GPU-s = 0.33 GPU-min per episode, with one design using the whole 4090
  (implied 3.3 M steps/s). H1 jog at that rate: 62 s ≈ 1.0 GPU-min (inference). At a 4 ms model step: 1.7 and 5.2 GPU-min.
- Train once: under 10 GPU-min (quadruped), under 60 GPU-min (G1). A trained-policy episode is 1,000 steps,
  about 0.01 GPU-s in a batch (from 106,093 PPO steps/s for G1 on an A100, Playground Table VII).
- Break-even episodes per design = training / MPC episode: 30 (quadruped), 60 (humanoid); 6 and 12 at 4 ms.
- Acceptance: bounding failure at 1% with zero failures at 95% confidence needs 299 episodes. MPC: 99 and 299
  GPU-min per design, against 10 and 60 trained. MPC is 10× and 5× more expensive. **The claim dies as an evaluator.**
- Gate: the slate's 10 GPU-min falsifier equals a whole quadruped training run, and 3 GPU-min is 30% of one. **No lever as written.**
- Screening: one episode is 30× to 60× cheaper than training; three episodes, 10× to 20×.
- Conflation: plant equals model, so success says nothing about the deployable policy or the real robot.
- Rival: MJPC, Hydrax ("sampling-based MPC algorithms on GPU", MJX, includes DIAL-MPC), Judo, BAMPC ("runs on
  MuJoCo Warp", https://arxiv.org/abs/2610.14064) are thin layers. **Fails kill test 6 as a reason for a new engine.**
- CPU (MEASURED): Go2 at 2 ms, f64, 36 threads: 410,000 steps/s, about 3 real-time 30-sample planners.

## Cheapest discriminating test

Hydrax or DIAL-MPC plus Playground PPO on one 4090. Thirty designs (Go2, H1 and G1 with limb lengths ±20%, masses
±30%, plus 5 wheeled), one fixed cost per class, 3 episodes each; PPO for 10 of them. Kills: success under 60%;
median above 1 GPU-min (quadruped) or 6 (humanoid); Kendall τ between MPC and PPO scores below 0.5.
Cost: 2 engineer-days, about 15 GPU-hours.

## Recommended state and the claim that survives

**NARROWED.** Sampling MPC is a screening rung of at most 3 episodes per legged design, at 0.33 to 1 GPU-min
(quadruped) and 1 to 5 (humanoid) per episode. It never produces acceptance evidence, and it needs no new engine.

## Open points

Rank agreement with trained policies (H-22); wheeled and tracked classes; whether a 4 ms model is required;
cost-function effort per design class.
