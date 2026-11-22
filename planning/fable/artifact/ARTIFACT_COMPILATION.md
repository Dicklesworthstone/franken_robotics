# Artifact compilation for decision cores D1–D11

An alien-artifact-coding pass over the eleven decision cores, read against plan §14.5, §19, §22–24, §27, §28 and
Appendices C and E.

**Nothing here is measured.** The plan has no baseline, so failure signatures are predicted (pre-mortem items 4
and 5, NS-09) and numbers are illustrative unless marked TARGET. Each family ships in shadow behind its baseline
and re-clears the matrix at M2. One asymmetry runs throughout: reporting an artifact-dependent result as
established, or eliminating the best design, costs far more than a wasted higher-rung episode.

## 1. EV opportunity matrix

Score = Impact × Confidence ÷ Effort.

| Candidate | Core | I | C | E | Score | Decision |
|---|---|---|---|---|---|---|
| Promote/reject e-process pair, loss-set thresholds | D1; stops in D2, D4, D7 | 5 | 5 | 1 | 25.0 | **F1** |
| Successive halving, fixed schedule | D2 | 5 | 4 | 1 | 20.0 | Baseline and fallback table, not a family |
| Crude Monte Carlo, exact binomial bound | D4 | 3 | 5 | 1 | 15.0 | **F4** |
| Lazy-greedy weighted coverage | D10 | 3 | 5 | 1 | 15.0 | **F6** |
| Conformal margin with audit lane | D3, D2 | 5 | 4 | 2 | 10.0 | **F2** |
| Counterfactual forks, placebo control | D6 | 4 | 5 | 2 | 10.0 | **F5** |
| Conformal band, exchangeability alarm | D7 | 4 | 4 | 2 | 8.0 | **F2** |
| Risk-controlled effort profiles | D8 | 4 | 4 | 2 | 8.0 | **F2** |
| Size classes with preempt-resume (M/G/1) | D9 | 4 | 4 | 2 | 8.0 | **F7** |
| Value of information per cost | D5 | 4 | 3 | 2 | 6.0 | **F3** |
| Rule-trace rewrite certificates | D11 | 3 | 4 | 2 | 6.0 | **F8** |
| Multilevel splitting on forks | D4 | 4 | 4 | 3 | 5.3 | **F4** |
| UCB or Thompson bandits | D2 | 3 | 3 | 3 | 3.0 | Rejected |
| Expected information gain | D5 | 2 | 3 | 2 | 3.0 | Rejected |
| Exact Shapley attribution | D6 | 2 | 4 | 3 | 2.7 | Budgeted summary only |
| Defensive-mixture importance sampling | D4 | 2 | 4 | 3 | 2.7 | Rejected |
| Extreme-value extrapolation | D4 | 3 | 2 | 3 | 2.0 | Rejected |
| PID or MPC on solver effort | D8 | 2 | 3 | 3 | 2.0 | Rejected |
| MMD or optimal-transport drift tests | D7 | 2 | 3 | 3 | 2.0 | Rejected |
| E-graph and SMT over real rewrites | D11 | 4 | 2 | 5 | 1.6 | Rejected |
| Gaussian-process surrogate over designs | D2, D5 | 3 | 2 | 4 | 1.5 | Rejected |
| Mechanism design | D9 | 2 | 2 | 4 | 1.0 | Rejected |

Relevance gate (symptom 0.35, architecture 0.25, proof readiness 0.20, operability 0.20): F1 4.8, F2 4.35,
F6 4.3, F4 4.25, F5 4.25, F7 3.8, F8 3.65, F3 3.6.

The eight families sit in three planes of at most three: evidence (F1–F3), failure (F4–F6), platform (F7, F8).
Successive halving is a fixed table; a reviewer who counts it as a ninth family should drop F8 back to the
plan's C-14 gate.

## 2. Selected families

### F1 · Sequential evidence — D1; stop rules in D2, D4, D7

- **Artifacts.** One bit per paired episode: disagreement beyond the ledger's frozen envelope. E⁺ tests
  "exceedance rate ≥ p₁" and promotes on crossing; E⁻ tests "rate ≤ p₀" and labels `tier-sensitive`. Both
  compile to an integer table (n, k) → promote, reject, continue.
- **Loss model.** States faithful or sensitive; actions promote, reject, buy a pair at cost c. α⁺ = c/(π·C_fp),
  α⁻ = c/((1 − π)·C_fn), π the ledger's share of sensitive results.
- **Obligations.** Wrong promotion ≤ α⁺ and wrong rejection ≤ α⁻ under optional stopping (Ville), if episodes
  are chosen by address before outcomes exist and the envelope is fixed first. Independence is not needed.

### F2 · Conformal prediction and risk control — D3, D2, D7, D8

- **Artifacts.** (a) Per rung pair and stratum, a residual quantile q̂: keep every design within 2q̂ of the k-th
  best cheap score, and trust the rung near the top when at most 3k survive. Training fraction counts as a rung.
  (b) An audit lane, the only calibration source: 10% of rung spend on designs drawn by keyed RNG before scores
  exist, at fixed n. (c) D7: a band real ≥ sim − q̂ per scope key (robot, firmware, floor, policy), with an F1
  martingale on its p-values. (d) D8: declared effort profiles, each its own `ModelId`. The guard table holds
  the cheapest with (n·R̂ + 1)/(n + 1) ≤ τ on the fidelity battery; unconverged worlds re-run one profile up,
  flagged. If that profile misses the throughput budget, the run shrinks its world count or is refused.
- **Loss model.** Eliminating a true top-k design against the rung cost of promoting a poor one.
- **Obligations.** Coverage ≥ 1 − α for designs exchangeable with the audit set. A pair separated by the margin
  is misordered with probability ≤ 2α. Top-k recall is measured by the audit lane, not claimed. D8 needs a
  monotone loss (running maximum over higher profiles).

### F3 · Bayesian value of information — D5; finalist stage of D2

- **Artifacts.** An experiment table (simulation at a rung, bench measurement, real batch): cost in GPU-second
  equivalents at a declared exchange rate, closed-form EVSI, their ratio. A posterior on finalists' score
  differences with sampling, tier-bias and reality-gap variance from the ledgers. Stop when no ratio clears the
  budget price.
- **Loss model.** Opportunity loss of the accepted design plus experiment cost.
- **Obligations.** EVSI is zero when no outcome can change the choice. Intervals are checked against registered
  predictions (§22.7). F3 only ranks experiments; verdicts come from F1 and F2, so a wrong model costs
  efficiency, never validity.

### F4 · Rare-event simulation — D4

- **Artifacts.** A certification table n = ⌈ln(1/δ)/p*⌉. A sequential binomial bound ending certified, refuted
  or exhausted. A splitting schedule on a distress score (fixed levels, child world addresses for forks). A
  failure-mode catalogue with a Good–Turing monitor of unseen mass (modes seen once ÷ failures seen). A
  truncated-weight lower bound for refutation.
- **Loss model and split.** False certification far outweighs false refutation. Explore first, to 20% of budget
  or unseen mass ≤ 0.05; refute if the lower bound exceeds p*; spend the rest on nominal sampling, for finalists
  only.
- **Obligations.** The bound holds at 1 − δ for independent nominal episodes with a frozen failure predicate,
  scoped to its tier. Splitting supplies discovery and `Estimated` rates only. Failures are replayed one tier
  up.

### F5 · Counterfactual causation — D6

- **Artifacts.** Typed interventions (`do(torque limit off)`, `do(friction := nominal)`,
  `do(estimator := privileged)`), a fork ledger, a placebo flip rate ρ₀, probability of necessity per cause,
  minimal repair sets by delta debugging, and labels `but-for`, `joint`, `chaotic: unattributable`. Output:
  which repair edit ranks first.
- **Obligations.** The empty intervention replays the failure bit for bit, so abduction is exact. Each
  intervention changes one declared mechanism (model diff). A cause is named only when its flip rate beats ρ₀
  under an F1 test.

### F6 · Submodular coverage — D10

- **Artifacts.** Scene × failure-mode incidence weighted by frequency, severity and designs affected; a
  lazy-greedy trace of marginal gains; a certificate with OPT ≤ f(S) + the k largest remaining gains; one slot
  in five for random held-out scenes. Objective: uncovered mode weight against battery runtime.
- **Obligations.** Weighted coverage is monotone submodular, so greedy reaches (1 − 1/e)·OPT at fixed
  cardinality. Unequal scene costs need the cost-benefit variant and its weaker ratio.

### F7 · Queueing — D9

- **Artifacts.** Size classes from the system's own `--estimate` (interactive ≤ 30 s); preempt-resume at
  snapshot boundaries (quantum ≤ 5 s); deficit round-robin per agent weighted by budget; an admission guard; a
  capacity table of wait by class against utilization; an estimate-calibration monitor.
- **Loss model.** Agent idle time on interactive rungs against preemption overhead and unfairness.
- **Obligations.** Utilization < 0.8 per device; resumed jobs keep their digests; an ageing bound prevents
  starvation. Arrivals are assumed near-Poisson; swarms burst, so the table carries Kingman's correction.

### F8 · Certified rewrites — D11

- **Artifacts.** A closed rule table. Class E is bit-exact (round to nearest, contraction off): commutation,
  common-subexpression and dead-code elimination, power-of-two scaling, scheduling. Class R is equal over the
  reals only: reassociation, distribution, fused multiply-add. A certificate is (source hash, target hash, rule
  trace), replayed by an independent checker.
- **Loss model.** A silent bit change invalidates replay and both ledgers; a skipped rewrite costs a few per
  cent of speed.
- **Obligations.** Each class-E rule is proved exact once on its domain; digests match on a pinned corpus on
  every backend. Class R needs a polynomial-identity certificate (C-14), an error bound, the fidelity battery
  and a new kernel version.

### Failure modes, fallbacks and budgets

| | Failure modes | Deterministic fallback | Trigger | Budgeted mode |
|---|---|---|---|---|
| F1 | Rates between p₀ and p₁; evidence pooled across identities | Fixed 30 pairs, promote on ≤ 2 exceedances | Missing ledger stratum; failed null self-test | 32 pairs, then stays `provisional` |
| F2 | Survivor-only calibration; optimizer drift; starved strata | Halving at η = 3; `reality: no-claim`; default profile | Under 19 calibration points; martingale ≥ 20; audited misses above τ; scope change | Survivors ≤ 3k, then truncate by cheap score, labelled `budget-truncated` |
| F3 | Wrong variance components; mispriced real cost | Fixed order: check, simulate, train, referee | 80% intervals cover outside 65–92% over 30 experiments | 64 candidates, closed form |
| F4 | Uninformative distress score; forks without fresh randomness | Crude Monte Carlo alone | Under 1% of forks reach the next level | Exploration ≤ 20%; then a refusal carrying the bound reached |
| F5 | Chaos makes every nudge a "cause"; cause precedes the snapshot | First-anomaly ordering, `Estimated` | Replay mismatch; ρ₀ > 0.2 | 12 interventions, singles and pairs (about 700 forks); Shapley only if none flips the outcome |
| F6 | Battery overfits past failures | Standard battery (§30.5) | Identity change since incidence was built | Any greedy prefix is valid |
| F7 | Long jobs misclassified as interactive | Round-robin per agent, run to completion | 20% of jobs exceed twice their estimate; a resume digest differs | One preemption per batch job per 30 s; batch share ≥ 30% |
| F8 | Microbenchmark win, whole-step loss; a backend re-contracts | Unrewritten derived kernel | Digest mismatch; gain under 3% | 60 s of search per kernel; one class-R candidate per release |

## 3. Galaxy-brain cards

Substituted values are illustrative.

**GB-A1 · What a promotion costs (F1)**
- **Equation.** ln E⁺ = (n − k)·ln((1 − p₀)/(1 − p₁)) + k·ln(p₀/p₁); promote at ln(1/α⁺).
- **Substituted.** p₀ = 0.05, p₁ = 0.30 and α⁺ = 2/(0.1 × 2,000) = 0.01: 16 clean pairs, or 22 with one
  exceedance. At α⁻ = 0.05, two exceedances in the first 3 pairs reject.
- **Intuition.** A clean referee episode multiplies the evidence by 1.36; an exceedance divides it by 6.
- **Assumptions.** Envelope frozen first; episodes chosen by address.
- **Flip.** C_fp ten times smaller, or p₁ = 0.5, gives 8 pairs.

**GB-A2 · Can the cheap rung pick the finalists? (F2)**
- **Equation.** q̂ = ⌈(n + 1)(1 − α)⌉-th smallest |y_hi − ĝ(y_lo)|; survivors score ≥ (k-th best) − 2q̂.
- **Substituted.** n = 39, α = 0.05: q̂ is the 38th residual, say 0.04. With the third-best cheap score at
  0.74 the cut is 0.66 and six of 81 designs survive. With rungs of 10 s, 3 min and 30 min the ladder costs
  about 2.6 h, audit lane included, against 6.1 h for halving and 40.5 h for training all 81.
- **Intuition.** A rung cannot separate designs closer than twice its own error, so everything inside that
  margin goes up.
- **Assumptions.** Exchangeable audit designs. Coverage averages 0.95; one calibration draw in twenty gives
  under 0.885.
- **Flip.** q̂ = 0.10 keeps about 20 designs, more than 3k: skip the rung for the top band or buy more
  episodes.

**GB-A3 · Which experiment next? (F3)**
- **Equation.** EVSI = s̃·[φ(z) − z·Φ(−z)], z = |m|/s̃, s̃² the variance the experiment removes from
  Δ ~ N(m, s²).
- **Substituted.** m = 0.03, s² = 0.03² (sampling) + 0.04² (tier bias). 200 more fast-tier pairs: 0.0013 for
  1 GPU-second. 8 referee pairs: 0.0055 for 27. The fast tier wins sixfold per unit cost; a second fast batch
  is worth 10⁻⁵, so the referee goes next.
- **Intuition.** Cheap episodes are bought until only tier bias is left, which only the referee can shrink.
- **Assumptions.** Gaussian components from the ledgers; per-pair noise 0.3 on the fast tier and 0.1 on the
  referee; 30 core-seconds per GPU-second.
- **Flip.** At m ≥ 0.15 no experiment can change the choice: stop and decide.

**GB-A4 · What a 10⁻⁶ claim costs (F4)**
- **Equation.** Zero failures in n nominal episodes: p ≤ ln(1/δ)/n. Splitting: cv² ≈ L(1 − p₀)/(p₀N).
- **Substituted.** δ = 0.05 and p* = 10⁻⁴, 10⁻⁵, 10⁻⁶ need 3 × 10⁴, 3 × 10⁵, 3 × 10⁶ episodes: 75 s, 12.5 min,
  2.1 h at 400 episodes/s (from the 2.0 M world-steps/s TARGET). Splitting to 10⁻⁶ with L = 6, p₀ = 0.1,
  N = 10⁴ uses 6 × 10⁴ fragments, cv ≥ 7%.
- **Intuition.** Forks find failures about fifty times sooner than waiting. Proving absence has a fixed price
  that no sampler discounts without assuming where failures live.
- **Assumptions.** Independent nominal episodes; claim scoped to the tier.
- **Flip.** Throughput ten times under target puts 10⁻⁶ at 21 h: certify 10⁻⁵, report 10⁻⁶ as `Estimated`.

**GB-A5 · Was torque saturation the cause? (F5)**
- **Equation.** PN(c) = P(no failure under do(¬c) | failure); attribute on PN(c) − ρ₀.
- **Substituted.** Of 200 failing worlds re-run from snapshots, `do(torque limit off)` prevents 164,
  `do(friction := nominal)` 38, a placebo nudge 12. Net of placebo: 0.76 and 0.13.
- **Intuition.** A cause is whatever, once removed, removes the failure more often than an irrelevant nudge.
- **Assumptions.** Exact null replay; the policy's reaction counts as part of the effect.
- **Flip.** Placebo rate near PN: chaotic, nothing attributed. Only the pair prevents the failure: joint cause.

**GB-A6 · How good is a six-scene battery? (F6)**
- **Equation.** f(greedy) ≥ (1 − 1/e)·OPT_k, and OPT_k ≤ f(S) + the k largest remaining gains.
- **Substituted.** 40 modes, total weight 100. Gains 31, 22, 14, 9, 6, 4 cover 86. The six largest remaining
  gains sum to 12, so OPT₆ ≤ 98 and greedy is within 88% of the best battery (worst case 63%).
- **Intuition.** A scene is worth only what earlier scenes missed.
- **Assumptions.** Equal scene cost; frozen taxonomy.
- **Flip.** Unequal costs: rank by gain per second, with a weaker guarantee.

**GB-A7 · Why priority alone is not enough (F7)**
- **Equation.** M/G/1: W = λ·E[S²]/(2(1 − ρ)). Non-preemptive priority: W₀/(1 − ρ₁) for the short class,
  W₀ = λ·E[S²]/2. Under preempt-resume it sees only itself.
- **Substituted.** One device, ρ = 0.7, 90% of jobs 10 s, 10% 30 min. A 10 s job waits 2,000 s
  first-come-first-served, 621 s with priority, 0.2 s plus one quantum with preempt-resume. Long jobs slow
  by 3%.
- **Intuition.** The wait is the remainder of the training run already on the device. Only preemption removes
  it.
- **Assumptions.** Poisson arrivals; accurate size classes; cheap resume.
- **Flip.** A device reserved for interactive work, or resume costing several seconds: run to completion.

**GB-A8 · Which rewrites are free? (F8)**
- **Equation.** fl(a + b) = fl(b + a) exactly; fl((a + b) + c) ≠ fl(a + (b + c)) in general.
- **Substituted.** `f32`, a = 10⁸, b = −10⁸, c = 1: (a + b) + c = 1 but a + (b + c) = 0, since spacing near 10⁸
  is 8. Sharing a subexpression used three times saves two evaluations and changes no bit.
- **Intuition.** Sharing and reordering are free. Regrouping makes a different kernel, which must be named as
  one.
- **Assumptions.** Round to nearest; contraction off.
- **Flip.** Over about a fifth of step time in derived arithmetic at the M2 profile makes class R and an
  e-graph search worth their certificates.

## 4. Composition

Timescales, slowest first: F8 at build; the D8 profile at ledger refresh; D7 per real batch; the D3 margin per
generation; D2 per rung transition; D5 per experiment; D1 per episode batch; F7 per quantum. Each loop treats
slower ones as constants.

| Controllers and shared quantity | Rule | Interference test |
|---|---|---|
| D2 survivors × D5: what runs next | F2 owns the population stage, F3 the finalists; one-way handover | Replayed campaign: no design gets requests from both |
| F1 stopping × F2 calibration: higher-rung episodes | Audit designs run at fixed n, never cancelled | No audit episode was stopped by an e-process |
| F1–F3 × F7: GPU-seconds, completion order | F7 decides when, never what; e-values consume episodes in address order | Random preemption and reordering leave digests and verdicts identical |
| D8 profile × F7: throughput | D8 never reads the wall clock or changes inside a run | Saturated queue: profile and digests unchanged |
| D8, F8 × F1, F2 ledgers: model identity | An identity change resets evidence and calibration; class E changes no identity | Pooling across two `ModelId`s is refused; 1,000 worlds keep digests after a class-E rewrite |
| F4 certification × F6 battery: scenes sampled | Bounds use nominal draws only | A bound refuses battery-selected episodes |
| D7 band × D5: real episodes | Conditions come from a pre-registered draw; D5 chooses only how many | Assigned conditions equal the draw |
| F1 in an N-way race: error rate | α/(N − 1) per comparison | A null race of 81 keeps the best with probability ≥ 1 − α |

F4 → F5 → F6 is a one-way pipeline. Two tensions from the compatibility matrix are reconciled. Conformal
exchangeability against drift: the alarm ends the regime and calibration restarts inside the new one. Parametric
F3 against distribution-free F1 and F2: F3 never issues a verdict.

## 5. Rejected as ornamental or premature

Attractive, but verification costs more than the gain:

- **E-graph saturation with SMT over real-arithmetic rewrites (D11).** Every accepted rewrite changes bits, so
  each needs an error bound, a battery run and a new kernel identity. The plan's own hypothesis puts most step
  time in the solve.
- **Extreme-value extrapolation (D4).** Checking a fitted tail at 10⁻⁶ takes the same 3 × 10⁶ nominal episodes
  that certify directly, about two GPU-hours at the TARGET throughput.
- **Defensive-mixture importance sampling as the certified bound (D4).** Its floor is ln(1/δ)/(βn), never below
  crude sampling on the nominal share β. It survives only as a lower bound for refutation.
- **Gaussian-process surrogates over designs (D2, D5).** No validated kernel exists on typed designs of varying
  topology; continuous sub-problems already have `fs-bo`.

Wrong tool, or no failure signature:

- **PID or MPC on solver effort (D8).** Wall-clock feedback into iteration counts makes results depend on load
  and breaks replay. A guard table with hysteresis passes.
- **Bandits (D2):** regret is the wrong objective for identification. **Expected information gain (D5):** it
  buys knowledge that cannot change the choice. **MMD and optimal transport (D7):** no power at twenty scalar
  residuals. **Mechanism design (D9):** estimates come from the system, not from agents.
- **Not scored:** convex dual prices (one scalar, already the F3 threshold); Lyapunov and bifurcation analysis
  (the residual monitor covers D8); observational causal adjustment (exact intervention is available); MCMC
  subset simulation (deferred unless forks carry no fresh randomness).

## 6. Recommendation contracts

**RC-1 · Promotion guard table (F1, D1)**

```text
Change:            Replace "sample size chosen by the e-process" (§14.5) with the promote/reject
                   exceedance pair, compiled to an integer table with loss-derived thresholds.
Failure signature: Predicted only: pre-mortem item 5, NS-09. No promotion-failure rate before M2.
EV score:          25.0
Relevance score:   4.8
Primary risk:      Envelope tuned after outcomes. Countermeasure: envelope hashed into the ledger
                   before sampling; episodes drawn by world address.
Budgeted mode:     32 pairs per result; on exhaustion the result stays provisional.
Fallback trigger:  Missing ledger stratum, identity mismatch, failed null self-test.
Baseline:          Fixed 30 pairs, promote on at most 2 exceedances. Simulated mean for the table:
                   16 to 21 pairs on faithful results, 3 to 5 on exploits.
Rollback:          Config flag to the fixed rule; issued verdicts stay valid.
Verification:      10^4 null runs at p1 with optional stopping; planted tier-sensitive policy caught
                   (GATE-FID-02); table regenerated in CI and diffed.
```

**RC-2 · Ladder allocation by conformal survivors (F2, D2 and D3)**

```text
Change:            Promote every design within 2*q_hat of the k-th best cheap score, per rung pair and
                   stratum; add a 10% keyed-random audit lane at fixed n as the calibration source.
Failure signature: Predicted only: pre-mortem items 4 and 5. No rung-pair residuals are measured.
EV score:          10.0, on top of the 20.0 halving baseline.
Relevance score:   4.35
Primary risk:      Non-exchangeable calibration. Countermeasure: audit draw before scores exist,
                   strata, exchangeability martingale.
Budgeted mode:     Audit 10% of rung spend; survivors at most 3k; then truncate by cheap score and
                   label budget-truncated.
Fallback trigger:  Under 19 calibration points, martingale at or above 20, audited miss rate above
                   tau, or survivors above 3k for two generations.
Baseline:          Successive halving, eta = 3, same population and seeds; compare GPU-seconds to the
                   same accepted decision, and audited top-k recall.
Rollback:          Config flag to halving; the ledger is kept.
Verification:      Per-stratum coverage on held-out audit designs; planted over-scored design caught
                   at the next rung; synthetic drift trips the alarm. Rollout: shadow, canary, default.
```
