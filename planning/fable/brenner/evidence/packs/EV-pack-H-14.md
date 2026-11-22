# EV-pack H-14 — conformal reality bands

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool.

## Claim under test

With at least 20 paired real episodes, split-conformal residual bands on simulator predictions achieve nominal
coverage on held-out real episodes for a robot class.

## Evidence against

1. **Twenty points is too few for the stated falsifier (DERIVED).** Angelopoulos and Bates,
   https://arxiv.org/html/2107.07511v6 (FETCHED): coverage given the calibration data follows Beta(n+1−l, l) with
   l = ⌊(n+1)α⌋, and "choosing n=1000 calibration points leads to coverage that is typically between .88 and .92".
   For n = 20 and a 90% target this is Beta(19, 2): mean 0.905, standard deviation 0.063, and probability 0.069
   that realized coverage is below 0.80. So with perfect exchangeability the "misses nominal by more than 10
   points" falsifier fires about one time in 14. For a standard deviation of 2 points, n must be about 200. With
   n = 20 a 95% band is the largest residual seen, and any level above 95.2% gives an unbounded band.
2. **The guarantee needs the test point to come from the calibration distribution.** Same source: the methods
   "rely on Theorem 1, which assumes that the incoming test points come from the same distribution", and "in the
   most general case, conditional coverage is impossible to achieve." Barber et al., arXiv 2202.13415 (SEARCH):
   coverage gap ≤ Σ wᵢ·d_TV(Z, Zᵢ) / (1 + Σ wᵢ). For an unbuilt design that distance cannot be measured.
3. **The exchangeable unit is the robot, not the episode (DERIVED; theory from MEMORY).** Twenty episodes on one
   machine are one cluster. A band for a new design has an effective sample size equal to the number of calibrated
   robots.
4. **The sibling already holds a counterexample.** `frankensim/crates/fs-eproc/tests/conformal_shift_receipt.rs`
   (FETCHED): pooled calibration at a 90% target covers 20 of 20 in one regime and 16 of 20 (80%) in the shifted
   one; "pooling still has nominal marginal coverage and silently undercovers the shifted bucket."
5. **Paired simulation buys little.** SureSim, https://arxiv.org/abs/2510.04354 (FETCHED): "our approach saves over
   20-25% of hardware evaluation effort to achieve similar bounds on policy performance." Project page,
   https://suresim-robot-eval.github.io (FETCHED): "In low correlation settings, real-world-only evaluation gives
   the tightest bounds on policy performance."
6. No paper reporting achieved coverage for a band calibrated in simulation and tested on a real robot was found
   (null result).

## Evidence for

- Marginal coverage is a theorem under exchangeability for any simulator quality; a poor simulator gives wide bands,
  not wrong ones.
- The sibling has the right machinery. `fs-eproc/src/hardening.rs` (FETCHED): per-bucket calibration where "a
  bucket that lacks calibration mass REFUSES rather than extrapolating another bucket's quantile", drift tests on
  the probability integral transform, and false-coverage budgets.
- SureSim shows finite-sample intervals on real performance from few real trials plus many simulated ones.

## Kill tests

1. Artifact: passes (exists behind the `conformal-hardening` feature).
2. Lever: none on time or compute. This is an honesty mechanism.
3. Scale: passes.
4. Foundation: passes.
5. Conflation: it is the remedy for conflation, and commits it if "robot class" exchangeability is assumed rather
   than tested.
6. Rival: fires. A conformal quantile is a few lines on any pipeline.

## Cheapest discriminating test

No hardware. Use a public paired simulation and real dataset, or as a stand-in treat a second simulator with a
perturbed contact model as reality. Calibrate on 20 episodes from task family A; test on held-out A and on shifted
family B; 1,000 resamples. Report mean coverage and the fraction of calibration draws with realized coverage below
80%. Kill as stated: mean coverage on B below 80%. Cost: hours of CPU, one to two engineer-days of data handling.

## Recommended state and the claim that survives

**NARROWED.** Marginal 90% coverage inside one calibrated bucket (same robot, same task family), with realized
coverage varying by about 6 points at n = 20 and about 2 points at n = 200. Refuse outside calibrated buckets. No
claim for designs that have not been built. Off the ten-fold path.

## Open points

- A public paired dataset with enough robots per class to test robot-level exchangeability was not identified.
- The total-variation bound was taken from a search summary, not the paper.
