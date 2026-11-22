# Handback — `RS-20261121-frob-fable-10x` (FrankenRobotics ten-fold session)

Written 2026-11-21 from `intake/`, `hypothesis_slate.md`, `deliverables/VERDICT_REGISTER.md`, the 23 packs in `evidence/packs/`, the two files in `debates/` and `deliverables/CHALLENGE_ADJUDICATION.md`. The register has been revised for the challenges; this file follows the revised register.

## Question (Q-001)
Which mechanisms, if any, let FrankenRobotics reach an accepted robot-design decision with at least ten times less elapsed time and compute than the strongest incumbent pipeline at equal or better decision quality, and which proposed mechanisms should be killed?

## Answer (as revised by the adjudication)
Q-001 is open. No matched mission has been run on any pipeline, and "three to ten on quadrupeds" is withdrawn as an answer. No single mechanism delivers ten-fold, and no conjunction has been shown to. The denominator is an accepted decision, which costs 299 failure-free episodes per design in every arm, not a training run. Against per-design training, paper arithmetic gives about 3–3.5× on quadruped cycles and about 15× on humanoid ones, if training can be replaced when designs are ranked (H-02, H-22a, the two least measured links). Against a composed incumbent that hosts the same portable methods, the expected ratio on fixed-topology CUDA missions is near one unless an engine-internal property binds. Engine speed is a parity requirement, with one open question (H-04b). Nothing was tested by running FrankenRobotics code; none exists.

## Hypotheses killed, narrowed or left active
"Contested" names the challenger: A = AGY (Gemini family), C = COD (Codex). Every contested row changed.

| ID | State | Decisive evidence, and what survives | Contested |
|---|---|---|---|
| H-04 Supercompiled kernels | REFUTED for recursion-only specialization | Recursions are 16–20% of two incumbent step traces; ceiling about 1.2×. Reversal condition: a specializable share of one third on our own profile. | A, C |
| H-09 Incremental evaluation | REFUTED for exact reuse | Exact rollout reuse is at most about 10% of compute. Training avoided by warm starts belongs to H-21 and is counted once. | C |
| H-15 Half precision | REFUTED | Condition number times half-precision round-off is at or above one in 37–64% of sampled contact states. | — |
| H-16 Living multiphysics | REFUTED as stated | Full-room fluid exceeds memory by orders of magnitude. Film law and rigid proxies are kept; whether they reproduce coupled-model failures is untested. | C |
| H-01 Sampling control | NARROWED | A screening rung and fallback, up to about 30 (quadruped) and 60 (humanoid) episodes per design at a 20 ms planning step; never a stand-in for a trained policy; acceptance evidence only as the delivered controller. | A, C |
| H-02 Universal policy prior | NARROWED | Within a class on flat ground: zero-shot 70–85%; fine-tuning about 2–25% of an early-stopped run. Interior terrain: no figure. | A |
| H-03 Topology as data | NARROWED | One incumbent solver's mixed batch runs at 0.86 of the harmonic mean of its homogeneous rates, and is not deterministic. Three differentiators remain. | A |
| H-05 Gait continuation | NARROWED | Orbit solves take 4–260 s of CPU, about 1% to 63% of a 410 s training run. A screening signal, not a controller. | — |
| H-06 Region certificates | NARROWED | Per-constraint certificates; the six-parameter cap applies to the split set, not to the box. | C |
| H-07 Rare-event engine | NARROWED | Measured gains 2–20× and about 5× in mean-square error; 10–100× is a target. Absence is never certified more cheaply than crude sampling. | C |
| H-08 Counterfactual autopsy | NARROWED | Contrasts carry a flip rate; precision follows measured sensitivity; design edits re-run from the start. | A, C |
| H-10 Value of information | NARROWED | 2–5× from stopping and elimination when candidates are separated, about 1× otherwise. | — |
| H-11 Always-on referee | NARROWED | Coverage is unmeasured: about 0.1–2.4% if the referee runs at 0.2–1.0× real time per core on 24 cores. A correctness guard. | A, C |
| H-13 Requirements replace rewards | NARROWED | Robustness alone fails as a training reward. Kept as acceptance metric and falsification score. | — |
| H-14 Conformal reality bands | NARROWED | Coverage varies about six points at n = 20 and holds only inside a calibrated family. | — |
| H-17 Mission to design by solving | NARROWED | Local refinement at about 12 degrees of freedom; catalog selection with no measured timing; pass rate under closed-loop evaluation unknown. | A |
| H-20 Proof-guided control | NARROWED | Published sum-of-squares certificates cover roughly 5–10 states. Reduced-order shields with a measured margin survive. | — |
| H-21 Policy inheritance | NARROWED | No figure for a small typed edit; prior-to-similar-body about 2–25%; the one parent-to-child figure about 50%. | A, C |
| H-23 A family per dispatch | NARROWED | Published population methods gain about 3–4× in compute. | — |
| H-24 Geometric-algebra policy | NARROWED | Augmentation recovers most of the invariance; expect 0–1.3×. | — |
| H-12 Generative structure | DEFERRED | Links 7–30% lighter, 2–8% of robot mass on one quadruped. Member sizing inside screening is ACTIVE as H-12b. | A, C |
| H-18 Information, not engine | ACTIVE | No pack and no instrumented session; which regime a mission is in is a measurement. | A, C |
| H-19 Thin layer over an incumbent | ACTIVE | Decided by a measured recovery ratio; two engine-internal gaps documented, two unverified. | A, C |
| H-22a, H-22b Rank fidelity; paired control | ACTIVE | No published rank agreement; a high τ does not protect the winner; pairing gains 1/(1 − ρ²) with ρ unmeasured. | A, C |
| H-04b, H-12b, H-25 to H-29 | ACTIVE | Admitted by the adjudication, untested: structure-exploiting solve; member sizing; utility brackets; compiled common randomness; step adjoints; placement along a motion; sensor visibility. | new |

## Still open, with the next action
| Item | Next action |
|---|---|
| Q-001 is unmeasured | The three-arm benchmark on hidden missions with one acceptance protocol; failed and exhausted briefs stay in the denominator. |
| H-19 thin layer | Build one composed-incumbent pipeline on a fixed-topology and a mixed-topology brief; report the recovery ratio at 0.8. |
| H-18 regime | Instrument about ten agent design missions on the incumbent by phase; write the missing pack. |
| H-22a, H-22b | 40 designs in one class: τ, top-k recall, containment, regret; cross-tier ρ for continuous and binary outcomes. |
| H-02, H-21 | A class prior on stairs, slopes and wet patches, and three- and ten-edit chains, against early-stopped scratch. |
| H-01 | Sample-count sweep with and without hand-built references on three robot classes. |
| H-04b, H-04 | An operation ledger of the solve stage, then dense, sparse-as-data and per-design arms on one device. |
| H-16, H-12b, H-06, H-07, H-08, H-11 | The pilots of `CHALLENGE_ADJUDICATION.md` §6, rows 10 to 15. |
| Whether certificates change a decision | Record, per certificate a decision consumed, whether the decision would have differed. |

## Challenge round and adjudication
- A contested 12 verdicts and C 13: 25 contests on 16 hypotheses, 9 contested by both, plus both contests of the answer. Eight rows stood uncontested: H-05, 10, 13, 14, 15, 20, 23, 24.
- 27 rulings: 11 upheld, 16 partly upheld, none rejected whole; ten specific claims inside partly upheld contests were rejected. All 16 contested rows changed; four changed state (H-04 and H-22 split, H-09 narrowed to exact reuse, H-18 to ACTIVE).
- The "0.86" figure is published (second version of the solver's paper) and audited; the H-03 pack had read the first version.
- Five proposed hypotheses were admitted as ACTIVE and renumbered H-25 to H-29.

## Phases of the ten-phase method compressed or skipped
- **No beads ledger.** Hypotheses and verdicts live in markdown tables.
- **Investigators were sub-agents of one model family** (four, refuter first), with a cross-family challenge round and one adjudicator of the investigators' family. No swarm panes or agent mail are recorded.
- Hypothesis generation: 19 seeds from the plan author, 3 from the modes-of-reasoning passes, 2 third alternatives and 5 from the challengers; three further rows come from splits made by the adjudication.
- **No distillation per model family**, no meta-synthesis, no fresh-eyes audit rounds, no drift check, no freeze or `RESUME.md`.

## Metrics
- Register rows 32 (24 original hypotheses, three of them split in two, and 5 new): narrowed 16; refuted 4; deferred 1; active 11.
- Refute attempts: 23 refuter-first packs, one per original hypothesis except H-18; four include a measurement run this session (H-01, H-11, H-15, H-19).
- Challenge round: 48 verdict reviews, 25 contests and 2 contests of the answer, 27 adjudicated; 16 rows revised; 19 disagreements registered with settling tests.

## Residual risk
- The answer rests on literature and arithmetic. No incumbent pipeline was instrumented end to end, and several packs quote sources through a summarizing fetch.
- Its load-bearing links are its least measured: family policies (H-02) and rank agreement (H-22a).
- The adjudicator and the packs come from one model family. If H-19 holds, a new engine is not the source of the gain.
