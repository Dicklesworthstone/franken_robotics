# EV-pack H-07 — rare-event engine

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool and
are short for that reason; the Fremont quote was extracted from the PDF text directly.

## Claim under test

Multilevel splitting or adaptive importance sampling estimates a failure probability of 1e-5 per episode with at
most 1/100 of the episodes naive sampling needs, with valid confidence bounds.

## Evidence against

1. **Splitting has no fixed-budget error guarantee and was not shown at 1e-5.** Innes and Ramamoorthy, adaptive
   splitting with temporal-logic monitors, https://arxiv.org/html/2405.15771v2 (FETCHED). Rarest property: ground
   truth 4.8e-05 from 100,000 runs; their estimate 3.5e-04 (±1.8e-03), seven times high with a spread five times
   the mean. "both splitting and sampling lack guarantees on estimation error for fixed N." The method "was sensitive
   to discard rate K" and suffered "extinctions". Sample efficiency was not reported: "these advantages complicate
   comparisons of our method to others in terms of sample efficiency".
2. **Cross-entropy importance sampling can be wrong by many orders.** Same paper: "This learning step has no
   convergence guarantees"; its estimate was 7.6e-22 against a truth of 2.0e-03.
3. **Plain splitting matched naive sampling on policy-in-the-loop tasks.** Sinha et al., NeurIPS 2020,
   https://ar5iv.labs.arxiv.org/html/2008.10581 (FETCHED). Relative mean-square error at 50,000 to 100,000 queries,
   CarRacing agent (p ≈ 2.5e-5): naive 1.0866, splitting 1.0211, their neural bridge 0.2285. "poor mixing adversely
   affects the performance of AMS and B".
4. **Measured gain in a driving study was 2 to 20 times.** O'Kelly et al., https://arxiv.org/abs/1811.00145
   (FETCHED): "accelerating system evaluation by 2-20 times over naive Monte Carlo sampling methods".
5. **Simulated failures do not all transfer.** Fremont et al., https://arxiv.org/pdf/2003.07739 (FETCHED): "62.5%
   of unsafe simulated test cases resulted in unsafe behavior on the track, including a collision; 93.3% of safe
   simulated test cases resulted in safe behavior on the track". So 37.5% did not reproduce, on a small sample.
6. **Snapshots only help with per-step noise (DERIVED).** A fork from an exact snapshot replays identically unless
   noise enters after the fork. Static scene and design parameters need whole-episode Markov-chain moves, which
   reuse no snapshot.

## Evidence for

- Bréhier et al., https://arxiv.org/abs/1505.02674 (FETCHED): the splitting estimator "is unbiased, whatever the
  choice of the importance function and the number of replicas."
- Sinha et al. (FETCHED): "the efficiency gains over naive Monte Carlo increase as pγ decreases."
- Feng et al., Nature 2023 (SEARCH; page paywalled): 1e3 to 1e5 times for driving with learned traffic models.
- DERIVED from the Au and Beck variance formula (MEMORY): at p = 1e-5, level probability 0.1, 1,000 samples per
  level, about 4,600 episodes; gain over naive sampling is about 480× with independent chains, 140× at chain
  correlation factor 3, 54× at factor 10. The 100× target sits at the ideal-case middle with no margin.

## Kill tests

1. Artifact: partial. `fs-uq/src/product_importance.rs` has defensive Gaussian-mixture importance sampling
   ("likelihood ratio is bounded by 1/epsilon") feeding a confidence sequence, and a cross-entropy mean update.
   No splitting or subset simulation exists (grep empty).
2. Lever: conditional. DERIVED: a zero-failure 95% bound at 1e-5 needs 3e5 episodes, about 3e8 control steps at
   1,000 steps each, roughly one to two training runs (training size from MEMORY). A perfect engine then saves at
   most about 3× on a finalist and nothing on other candidates. At 1e-6 the leverage is larger.
3. Scale: passes.
4. Foundation: `frankensim/README.md` line 1612: "GPU/accelerator execution | Not implemented". Fallback: naive
   sampling with `fs-eproc` confidence sequences.
5. Conflation: at risk. The probability is under the simulator's scene distribution.
6. Rival: fires. Incumbent state is arrays, so a fork is a copy; Isaac Lab's Newton backend offers
   `deterministic_mode` "run_to_run" (https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism,
   FETCHED). A thin layer does this.

## Cheapest discriminating test

Planted failures of known probability 1e-4 and 1e-5 on one quadruped in CPU MuJoCo, one with two separate failure
modes. Compare naive, splitting, subset simulation and defensive importance sampling at equal episode budget, 30
repetitions. Kill: gain below 10× at 1e-5, or 95% interval coverage below 85%, or a planted mode missed in more
than 20% of repetitions. Cost: two engineer-days, tens of CPU core-hours.

## Recommended state and the claim that survives

**NARROWED.** 10 to 100 times fewer episodes at 1e-4 to 1e-5, only for failures driven by per-step disturbances
with a good score. Valid bounds come only from bounded-weight importance sampling with confidence sequences, not
from splitting. Portable to incumbents, so it is evidence for H-19, not for a new engine.

## Open points

- No robot result at 1e-6 or 1e-7 was found.
- The share of discovered failures that are simulator artifacts is unmeasured for legged robots.
