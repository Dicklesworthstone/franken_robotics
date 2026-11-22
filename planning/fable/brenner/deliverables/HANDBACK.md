# Handback — `RS-20261121-frob-fable-10x` (FrankenRobotics ten-fold session)

Written 2026-11-21 from `intake/`, `hypothesis_slate.md`, `deliverables/VERDICT_REGISTER.md`, the 23 packs in `evidence/packs/` and the two files in `debates/`. The register predates the challenges and has not been revised for them.

## Question (Q-001)
Which mechanisms, if any, let FrankenRobotics reach an accepted robot-design decision with at least ten times less elapsed time and compute than the strongest incumbent pipeline at equal or better decision quality, and which proposed mechanisms should be killed?

## Answer (as registered; contested by both challengers)
No single mechanism delivers ten-fold. What survives is a conjunction: decide many designs per agent turn; avoid training most of them through family policies and inheritance, gated by measured rank agreement; stop early; search for failures; and measure time to an independently accepted answer against a composed incumbent. On quadruped missions an incumbent already trains in about seven minutes on one RTX 4090, so the registered range is three to ten; it is wider for humanoid and multi-robot missions. Engine speed is a parity requirement. Nothing was tested by running FrankenRobotics code; none exists.

## Hypotheses killed or narrowed
"Contested" names the challenger: A = AGY (Gemini family), C = COD (Codex).

| ID | State | Decisive evidence, and what survives | Contested |
|---|---|---|---|
| H-04 Supercompiled kernels | REFUTED as a GPU lever | Lie-algebraic recursions are 16–20% of an incumbent step trace, so the ceiling is about 1.2×. Derived derivative kernels for CPU solvers survive. | A, C |
| H-09 Incremental evaluation | REFUTED as stated | Exact rollout reuse after a mass or geometry edit is at most about 10% of compute. Memo graph and verdict revalidation survive. | C |
| H-15 Half precision | REFUTED | Condition number times half-precision round-off is at or above one in 37–64% of sampled contact states (emulated measurement). | — |
| H-16 Living multiphysics | REFUTED as stated | Full-room fluid exceeds memory by orders of magnitude; a film-friction law and rigid proxies are kept. | C |
| H-01 Sampling control | NARROWED | 299 acceptance episodes cost 5–10× one training run. Survives as a screening rung and fallback, never evaluator of record. | A, C |
| H-02 Universal policy prior | NARROWED | Authors' own limits: zero-shot 70–85% within a class, 10–40% across topologies. Family policies per class. | A |
| H-03 Topology as data | NARROWED | An incumbent publishes a mixed batch at 0.86 of its homogeneous rate. Only measured throughput and edit-to-first-step latency differentiate. | A |
| H-05 Gait continuation | NARROWED | Orbit solves take 4–260 s of CPU. A screening signal for periodic gaits, not a controller. | — |
| H-06 Region certificates | NARROWED | 10⁴ boxes is under one bisection per axis in 20 dimensions. Per-constraint certificates on about six parameters. | C |
| H-07 Rare-event engine | NARROWED | 10–100× fewer episodes only for per-step-disturbance failures; valid bounds need bounded-weight importance sampling. | C |
| H-08 Counterfactual autopsy | NARROWED | Contact dynamics are sensitive to round-off. Contrasts carry a flip rate; design edits re-run from the start. | A, C |
| H-10 Value of information | NARROWED | 2–5× from stopping and elimination when candidates are separated, about 1× otherwise. | — |
| H-11 Always-on referee | NARROWED | Throughput arithmetic gives 0.1–2% coverage of GPU world-seconds. A correctness guard, not a speed lever. | A, C |
| H-13 Requirements replace rewards | NARROWED | Robustness alone fails as a training reward. Kept as acceptance metric and falsification score. | — |
| H-14 Conformal reality bands | NARROWED | Coverage varies about six points at n = 20 and holds only inside a calibrated family. | — |
| H-17 Mission to design by solving | NARROWED | Published co-optimization: about 12 degrees of freedom, 17 parameters, 4–10 minutes. Local refinement only. | A |
| H-18 Information, not engine | NARROWED | Training dominates published loops; speed alone caps well under ten. No evidence pack exists for this verdict. | A, C |
| H-20 Proof-guided control | NARROWED | Published sum-of-squares certificates cover roughly 5–10 states. Reduced-order shields with a measured margin survive. | — |
| H-21 Policy inheritance | NARROWED | Parameter edits cost 1–20% of from-scratch steps; single-limb edits 33–100%. | A, C |
| H-22 Rank fidelity | NARROWED, unmeasured | No published rank agreement; nearest figure is 0.40. A control variate saves about 2× unless correlation exceeds 0.9. | A, C |
| H-23 A family per dispatch | NARROWED | Published population methods gain about 3–4× in compute. | — |
| H-24 Geometric-algebra policy | NARROWED | Augmentation recovers most of the invariance; expect 0–1.3×. | — |

## Still open, with the next action
| Item | Next action |
|---|---|
| Q-001 is unmeasured | Run the pre-registered benchmark of the question of record with a composed-incumbent arm and a phase ledger. |
| H-19 thin layer over an incumbent (ACTIVE, strongest rival) | The pack's two-week spike on MuJoCo Warp plus MuJoCo C: bitwise snapshot and restore of 1,024 worlds, 64 bucketed designs against a Kamino mixed batch, energy-ledger residual. |
| H-22 rank agreement (the register's "decisive early experiment") | 40 designs in one class scored by shared policy, a 5%-budget fine-tune and a referee specialist; report Kendall τ, top-10 recall and cross-tier correlation on 256 paired episodes. |
| H-18 has no evidence pack | Instrument incumbent design missions by phase (both challengers' cheapest test) and write the pack. |
| H-12 generative structure (DEFERRED) | One lower-leg link against a hand-sized tube; kill below 5% saving. A wants fast beam sizing inside the loop. |
| Sampling controller as default baseline | The H-01 test: Hydrax or DIAL-MPC against PPO on 30 designs, about 15 GPU-hours. |
| Whether specialization pays | The H-04 three-arm kernel test on our own profile. |
| Whether certificates change a decision | No evidence either way and no test named in the artifacts; one must be designed. |
| Challenge round | Adjudicate the 16 contested verdicts and rule on five proposed hypotheses (A: contact adjoints with joint actor-critic, convex inverse-dynamics synthesis, observability and sensor co-design; C: embodiment-potential bounds, compiled common random numbers). Both files use the IDs H-25 and H-26. |

## Cross-family challenges (`debates/CHALLENGE_AGY.md`, `debates/CHALLENGE_COD.md`)
- A contests 12 verdicts, C contests 13; 16 of 24 are contested by at least one and 9 by both (H-01, 04, 08, 11, 12, 18, 19, 21, 22). Eight stand uncontested: H-05, 10, 13, 14, 15, 20, 23, 24. No Claude-family challenge file was present.
- Both contest the answer. C: the thesis stays open, the 3–10× quadruped range has no matched mission evidence, and seven minutes of training is not the denominator for a decision that needs 299 episodes. A: every ingredient of the conjunction is marked portable to incumbents in its own pack, so against a composed incumbent it may tie.
- Evidence cited: H-04, the same step trace (A counts 85.5% of the step as topology-dependent; C says 20% holds for that trace only). H-01, 24 paired decision episodes or a 2 GPU-second episode changes the crossover. H-11, the H-11 pack's own measurement, which A reads as 3.5–35% coverage. H-21, the 1–20% rests on a scratch baseline that was not stopped early. H-22, the 0.40 is a critic predicting its own policy.
- A notes that the 0.86 figure under H-03 is not in the H-03 pack. It appears in the audited modes-of-reasoning ledger (claim C022).

## Phases of the ten-phase method compressed or skipped
- **No beads ledger.** Hypotheses and verdicts live in two markdown tables.
- **Investigators were sub-agents of one model family** (four, refuter first), **with a cross-family challenge round** in place of pairwise debates and an adjudicator. No swarm panes or agent mail are recorded in the workspace.
- Hypothesis generation: 19 seeds from the plan author before any swarm input, 3 from the modes-of-reasoning passes, 2 third alternatives; no multi-agent proposal round is recorded.
- **No distillation per model family** and no meta-synthesis. **No fresh-eyes audit rounds. No drift check.** No freeze or `RESUME.md`.

## Metrics
- Hypotheses proposed 24; narrowed 18; refuted 4; deferred 1 (H-12); active 1 (H-19); third alternatives 2 (H-18, H-19).
- Refute attempts: 23 refuter-first packs, one per hypothesis except H-18; four include a measurement run this session (H-01, H-11, H-15, H-19).
- Challenge round: 48 verdict reviews, 25 contests, 0 adjudicated; 5 new hypotheses proposed.

## Residual risk
- The answer rests on literature and arithmetic. No incumbent pipeline was instrumented end to end, and several packs quote sources through a summarizing fetch.
- Its load-bearing links are its least measured: family policies (H-02) and rank agreement (H-22).
- Two-thirds of the verdicts and the headline answer are contested by other model families and unadjudicated; if H-19 holds, a new engine is not the source of the gain.
