# EV-pack H-13 — temporal-logic requirements replacing rewards

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool.

## Claim under test

Missions written as signal-temporal-logic requirements with quantitative robustness serve as the evaluation metric
and, smoothed, as a training signal, without loss of final performance.

## Evidence against

1. **Robustness as a reward is the weak baseline in the field's own papers.** TGPO,
   https://arxiv.org/abs/2510.00225 (FETCHED): "Prior RL approaches focus only on limited STL fragments or use STL
   robustness scores as sparse terminal rewards." Their fix decomposes the formula "into timed subgoals and
   invariant constraints" and gains "an average of 31.6% improvement in task success rate compared to the best
   baseline." So a formula alone was not enough.
2. **Robustness semantics hurt optimizers.** Study of robustness metrics for robot tasks,
   https://ar5iv.labs.arxiv.org/html/2110.00339 (FETCHED): "They are non-smooth and non-differentiable, making it
   more difficult for any optimizer to find a good solution." Success rates: 29.9% for the smooth variant with
   Bayesian optimization, 12.71% for classical space robustness, and at most 6.67% with an evolution strategy.
3. **Unbounded-time formulas are not learnable with guarantees.** Yang, Littman and Carbin, IJCAI 2022,
   https://ar5iv.labs.arxiv.org/html/2111.12679 (FETCHED): "Practically, our result implies that it is impossible
   for a reinforcement-learning algorithm to obtain a PAC-MDP guarantee", except for "formulas that are decidable
   within a finite horizon". This restricts the language to bounded-time operators; it does not hit bounded-time
   requirements.
4. **Agents write wrong formulas about one time in five.** NL2TL, https://ar5iv.labs.arxiv.org/html/2305.07766
   (FETCHED): "GPT-4 achieves an accuracy of 77.7% over 300 samples", so 22.3% wrong, above this hypothesis's own
   20% falsifier. A fine-tuned model reached ">95%". "the model will fail once the NL and LTL structures are too
   complex." These are 2023 models.
5. **Nothing exists in the sibling.** A search of all frankensim crates for signal temporal logic, temporal logic,
   metric temporal logic or LTL returned no crate.

## Evidence for

- One direct quadruped result. Human-STL, https://arxiv.org/abs/2607.00442 (FETCHED abstract): Barkour quadruped
  "in MuJoCo XLA (MJX)", rewards from smooth robustness; "Compared with hand-crafted rewards, an expert-switching
  oracle, and Text2Reward", it "maintains high command-tracking success across the evaluated speed range". The
  abstract gives no numbers; one robot; whether the formulas replaced or supplemented the reward is unverified.
- Robustness as a falsification objective is mature: a yearly competition category has run from 2018 to 2026 with
  several tools (SEARCH).
- Robustness worked as a splitting score for rare-event estimation, https://arxiv.org/html/2405.15771v2 (FETCHED),
  which links this hypothesis to H-07.
- As an evaluation metric the claim is close to definitional: a pass or fail with a margin.

## Kill tests

1. Artifact: nameable (a requirement language, monitors as kernels); nothing built.
2. Lever: none stated on decision time. "Without loss" is a non-inferiority claim.
3. Scale: passes; monitor cost is linear in episode length times formula size.
4. Foundation: fails without a fallback. Fallback: shaped rewards for training, requirements for acceptance.
5. Conflation: satisfying a formula in simulation is a fact about the model.
6. Rival: fires. The quadruped result above was built on an incumbent engine.

## Cheapest discriminating test

(a) Specification writing: 50 scripted mission statements, each with a reference formula and 20 labelled
trajectories. The agent writes the formula; score agreement on the labels. Kill: more than 20% wrong. Cost: one
day, no GPU. (b) Training: one quadruped velocity-tracking task on an incumbent engine; shaped reward, smoothed
robustness only, and both; five seeds each. Kill: robustness-only below 80% of shaped success. Cost: about ten
GPU-hours.

## Recommended state and the claim that survives

**NARROWED.** Bounded-time requirements as the acceptance metric and as the score for falsification and splitting.
As a training signal, only as an added term or after decomposition into subgoals. Agent-written formulas need a
semantic check against labelled trajectories. Portable to incumbents.

## Open points

- Numbers from the quadruped paper's full text were not read.
- Formula accuracy for current agent models is unmeasured here.
