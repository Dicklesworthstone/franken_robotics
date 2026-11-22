# EV-pack H-22 — rank fidelity between rungs; fast tier as control variate

Session `RS-20261121-frob-fable-10x`. Sources read 2026-11-21. FETCHED = read this session; MEMORY = not re-read.

## Claim under test

(a) Ranking designs by a shared policy on the fast tier agrees with trained specialists on the referee at Kendall τ ≥ 0.7 near the top, so fewer than 1 design in 20 needs specialist training. (b) Paired cheap and expensive evaluations give unbiased expensive-rung estimates with at most 1/5 of the expensive samples.

## Evidence against (excerpts)

- No direct measurement exists. I found no paper reporting rank agreement between shared-policy scores and specialist scores; N-LIMB, Loki and the morphological-pretraining paper do not state one.
- Nearest measurement is weak. "the value predictions are positively correlated with true rollout return, with Spearman ρ=0.40 overall" and "ρ=0.50, 0.60, and 0.83 for MIT Humanoid, Unitree Go2, and Golem." This is 17 designs, critic against rollouts of the same shared policy, an easier comparison than (a). Also: "the critic predicted high returns but the real performance collapsed." https://arxiv.org/html/2606.00702 [FETCHED]
- Cheap rungs are biased, not just noisy. "premature fitness evaluation systematically underestimates true potential and biases selection towards fast learners". https://arxiv.org/html/2608.23100 [FETCHED]. Shared control: the population "converges to similar designs that are easier to steer with a shared universal controller." https://arxiv.org/abs/2502.10862 [FETCHED]. "the design distribution may incorrectly focus the search around suboptimal designs". N-LIMB, https://ar5iv.labs.arxiv.org/html/2207.11773 [FETCHED]
- Analogue from weight sharing in architecture search: "the super-net accuracy has a very low correlation with the final performance"; sparse Kendall τ 0.236, 0.740 and 0.159 on three benchmarks. https://ar5iv.labs.arxiv.org/html/2003.04276 [FETCHED]
- Control variate arithmetic. Variance scales by (1 − ρ²) (stated in the multi-fidelity paper below). A 5× saving needs ρ ≥ 0.894 even with free cheap samples; ρ = 0.7 gives 1.96× (my arithmetic).
- Correlation is fragile. "cross-fidelity correlation drops sharply and MFPG performance becomes effectively neutral" at large dynamics gaps. https://arxiv.org/html/2503.05696v4 [FETCHED]. Paired worlds decouple quickly: a 5×10⁻⁴ degree perturbation "causes the systems to significantly diverge after 50 steps." https://arxiv.org/html/2410.10674v1 [FETCHED]

## Evidence for (excerpts)

- "the universal controller impressively obtains up to 80% of the performance of individually trained controllers." N-LIMB [FETCHED]
- "the meta policy achieves close-to-optimal performance for each design instance after adaptation." https://arxiv.org/abs/2210.02750 [FETCHED, abstract]
- Top-1 agreement at modest ρ: "The final design f⋆ has both the highest value prediction and the highest rollout return for all three robots." [FETCHED, 2606.00702]
- A two-rung ladder in practice: Loki "explores ∼780× more designs" with "approximately 78% fewer simulation steps", then retrains the top 100 from scratch. https://arxiv.org/html/2505.21665 [FETCHED]
- The control variate is unbiased and helped in "8 out of 8 mild-to-moderate scenarios" [FETCHED, 2503.05696]. τ 0.74 on one architecture benchmark shows the target is reachable in some spaces.

## Kill tests

- Artifact, lever, scale: pass.
- Foundation: needs the referee tier and pairing by address (in plan).
- Conflation: a referee ranking is a model ranking, not a real-robot ranking.
- Rival: partial kill. Successive halving and multi-fidelity optimizers wrap any incumbent. Pairing by address is engine-specific, but chaos limits its value.

## Cheapest discriminating test

Calibration set of 40 designs in one class. Score each with the shared policy on the fast tier, with a 5%-budget fine-tune, and with a specialist on the referee. Report Kendall τ, top-10 recall and bootstrap intervals; report per-episode ρ across tiers on 256 paired episodes per design. Kill: top-of-ranking τ below 0.5, or ρ below 0.7. Cost: about 45 GPU-hours, or about 5 if the specialists from the H-02 test are reused.

## Recommended state and the claim that survives

NARROWED.
- (a) is unmeasured. Published neighbours give Spearman 0.4–0.8, which does not support τ ≥ 0.7. Use the cheap rung to discard, not to select. I infer that roughly the top quarter, not the top 5%, must go to a fine-tune rung; this is an inference, not a measured value.
- (b) survives at about 2× unless measured ρ exceeds 0.9.

## Open points

- A fine-tune rung at 1–10% of specialist cost (H-21 evidence) probably ranks better than zero-shot; untested.
- Bias toward easy-to-control designs is systematic, so rank agreement must be checked per region of design space.
- Excerpts were pulled through a fetch tool; table values need a hand check.
