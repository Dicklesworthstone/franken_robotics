# EV-pack H-10 — value-of-information experiment planning

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool.

## Claim under test

Choosing the next experiment by expected information gain on the decision cuts the number of simulations needed to
reach a decision by at least 3× against agent-chosen experiments.

## Evidence against

1. **The baseline is strong.** Zhang et al., https://arxiv.org/html/2312.04528v2 (FETCHED): "LLMs can match or
   outperform traditional HPO methods like Bayesian optimization". At 10 evaluations "GPT-4 Turbo beats random
   search 81.25% of the time" against 50.00% for Gaussian-process Bayesian optimization and 56.25% for the
   random-forest variant. At 30 evaluations: 90.62% against 84.38% and 71.88%. This is the opposite sign to a 3×
   gain over an agent. Caveats: hyperparameter tuning, 2023 models, "performance can be inconsistent between random
   seeds".
2. **Model-based search lost in a robot co-design benchmark.** Evolution Gym,
   https://ar5iv.arxiv.org/html/2201.09863 (FETCHED): "overall, GA outperforms the other two baseline algorithms";
   for Bayesian optimization "the high-dimensional categorical input parameter space and the noisy evaluation done
   by RL together pose a challenge".
3. **The estimator is hard and fragile.** Rainforth et al., Statistical Science 2024,
   https://ar5iv.labs.arxiv.org/html/2302.14545 (FETCHED): "the EIG is a doubly-intractable quantity that is
   difficult to estimate, let alone optimize."; adaptive design "can suffer from serious pathologies in practice if
   our model is misspecified"; "it can be better to instead resort to heuristic approaches when constructing an
   accurate model is not possible."
4. **The sibling's own measured ceiling is 2 to 5 times, and only on easy fields.**
   `frankensim/crates/fs-race/tests/battery.rs` (FETCHED) asserts savings of at least 2.0 on a separated field
   ("claimed 2-5x") and below 1.5 on an inseparable field ("no fake payoff").
5. **`fs-voi` has no measured saving.** Its comparison test, `voi_beats_the_uncertainty_proportional_baseline` in
   `fs-voi/tests/voi.rs` (FETCHED), is a three-design hand-built fixture against a naive rule.

## Evidence for

- `fs-voi` exists (1,715 lines): expected opportunity loss over all alternatives, knowledge gradient, a stop gate,
  and actions tied to unresolved requirements (FETCHED file).
- Hierarchical batch optimization for a six-legged microrobot, arXiv 1905.01334 (SEARCH): "360% reduction in
  production cycles over standard Bayesian optimization", in simulation.
- Fisher-information exploration for identification, ASID, arXiv 2404.12308 (SEARCH): a single real episode is
  reported to suffice. Active identification for legged robots, SPI-Active (SEARCH): 42 to 63% improvement.
- No study was found that compares information-gain planning against an agent in robot design.

## Kill tests

1. Artifact: passes (`fs-voi`, `fs-race`, `fs-eproc`).
2. Lever: stated, but the baseline "agent-chosen" is undefined and the evidence points below 1.5× for selection.
3. Scale: passes.
4. Foundation: passes for independent Gaussian design estimates; no posterior over contact-rich model parameters.
   Fallback: racing.
5. Conflation: information gain is computed under a model that may be wrong; say so in the output.
6. Rival: fires. Standard optimization libraries do this on any engine.

## Cheapest discriminating test

Offline replay. Log once: 64 designs × 3 fidelity rungs × 32 seeds on an incumbent engine (about one GPU-day).
Then replay four choosers with no further simulation: an agent reading the log so far, `fs-voi` knowledge gradient,
`fs-race` elimination, and uniform allocation. Metric: evaluations until a true top-3 design is identified at 95%
confidence, over 100 replays. Kill: `fs-voi` under 1.5× better than the agent. Cost: one GPU-day plus one
engineer-day.

## Recommended state and the claim that survives

**NARROWED.** Anytime-valid stopping and elimination save 2 to 5 times the evaluations when candidates are
separated and about nothing when they are not. Information-gain selection against a capable agent is unsupported
and plausibly below 1.5×. Use value of information as a stop rule and budget allocator. Portable to incumbents.

## Open points

- Results with current agent models, not 2023 ones.
- How separated real design populations are, which decides whether the 2 to 5 times applies.
