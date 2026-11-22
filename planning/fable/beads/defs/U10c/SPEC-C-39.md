---
key: "SPEC-C-39"
title: "spec C-39 core: the value of an experiment: closed-form value of sample information, ranking, stop rule"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "spine", "spec", "solid", "atelier", "frob-opt", "test"]
parent: "WP-59"
deps: ["BD-262", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-291", "BD-341", "BD-344", "BD-349", "BD-351", "N10c-09", "N10-CX", "N10c-10"]
gates: ["GATE-ATEL-15"]
plan_refs: ["§10 C-39", "§28.9", "§28.16", "§28.17", "§30.15", "Appendix D.8", "Appendix E GB-36", "Appendix M.9"]
---
## Background

Construction C-39 of the FrankenRobotics spine is "the value of an experiment, and the design of a probe". An
experiment (more episodes at a rung, an escalation to the referee tier, a bench measurement, a batch on a real
robot) is valued by what it can change. This bead is the core specification and covers the value model: a closed
form for the expected value of sample information, the ranking of experiments by value per unit cost, and the stop
rule. Tier Solid [S] for the closed form and the ranking. The second half of the construction, probe design
([F]), is bead N10c-09, which extends the same contract file. Owner: the Atelier subsystem (the studio loop and
its decision cores), crate `frob-opt`; the ranking is consumed by the verb `experiment.suggest`.

What it replaces: a fixed order of check, simulate, train, referee. Payoff: cheap episodes are bought until only
tier bias is left, which only the referee shrinks; real episodes are worth buying where the reality gap is the
largest unknown; and a campaign stops when nothing is worth its price. The milestone is M3 because the first
consumer, the probe pilot, runs at M3; the gate on the ranking itself is at M4.

The one rule that makes the model safe to be wrong: it only ranks experiments. Verdicts come from the test
martingales (C-15), the coupled tier estimators (C-25) and the rank-agreement and conformal margins (C-26), so a
wrong variance model costs efficiency and never validity. FACT (source register, to be re-read): stopping and
elimination gain a factor of 2 to 5 when candidates are separated and about 1 otherwise; that is the ceiling, at
the finalist stage only. FACT (FrankenSim `483d09c682f2`, observed 2026-11-21): the sibling value-of-information
crate has a ranking-flip probability, an action value, a recommendation with a stop threshold, expected
opportunity loss and a knowledge gradient on a finite Gaussian menu, and its own comparison test is a three-design
fixture, so no saving is measured there. Rejected alternative: letting the value model end a study or issue a
verdict.

Terms. Δ: the difference in the objective between two finalists, with posterior N(m, s²). s̃²: the variance of Δ
that an experiment removes. φ and Φ: the standard normal density and distribution function. *Opportunity loss*:
the objective lost by choosing the design that is not actually best. *Budget price*: the value per unit cost
below which nothing is bought. Proof classes: Proved, Certified, Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-39.md` part A; law tests in `crates/frob-opt/tests/laws_c39_evsi.rs`; registry entry for KA-187
and the pilot record of the construction.

Out: the verb `experiment.suggest` and its ranking in the product (BD-351); the decision-core table for "which
experiment next, or stop" (BD-349); registered predictions and their scoring (BD-344); optimization and racing
(BD-341); probe design (N10c-09, BD-291); recording GATE-ATEL-15 and deciding KILL-VOI (BD-351).

## Technical approach

### 1. Statements

| Id | Statement | Class |
|---|---|---|
| E1 | For two finalists with posterior difference Δ ~ N(m, s²), an experiment that removes variance s̃² from Δ has expected value of sample information EVSI = s̃·[φ(z) − z·Φ(−z)] with z = \|m\|/s̃, in the units of the objective. Derivation: before the experiment the posterior mean it will produce is N(m, s̃²); the value is E[max(m′, 0)] − max(m, 0). | Tested (against numerical quadrature of the defining expectation) |
| E2 | EVSI is 0 when s̃ = 0, increases in s̃ (∂EVSI/∂s̃ = φ(z)) and decreases in \|m\| (∂EVSI/∂\|m\| = −Φ(−z)). At m = 0 it is s̃/√(2·π). | Tested |
| E3 | With more finalists the same quantity is the expected reduction in the opportunity loss of the chosen design. | Tested (reduces to E1 for two finalists) |
| E4 | A measurement of variance v on a component of prior variance w removes w²/(w + v). The variance components come from the ledgers: sampling, tier bias and reality gap. | Tested (normal conjugacy) |
| E5 | Experiments are ranked by value per unit cost, in GPU-second equivalents at a declared exchange rate, and a campaign stops when no ratio clears the budget price. | Tested (stop rule) |
| E6 | The value model only ranks experiments; it never issues a verdict. A wrong variance model costs efficiency and never validity. | Structural (no verdict type is constructible from a value); recorded |
| E7 | Every suggestion registers the predicted outcome interval of the chosen experiment before it runs. If stated 80% intervals cover fewer than 65% or more than 92% of thirty registered experiments, the core falls back to the fixed order of check, simulate, train, referee. | Rule with its TARGET thresholds |

### 2. Worked numbers (illustrative; the arithmetic is the content)

m = 0.03; sampling variance 0.03² and tier-bias variance 0.04²; noise per pair 0.3 on the fast tier and 0.1 on the
referee.
- Two hundred more fast-tier pairs measure the sampling component: they remove s̃ = 0.0245 and are worth 0.0013
  for 1 GPU-second.
- Eight referee pairs measure Δ itself: they remove s̃ = 0.0408 and are worth 0.0055 for 27 GPU-second
  equivalents. The fast tier wins about sixfold per unit cost (6.4).
- A second fast batch is worth 10⁻⁵, so the referee goes next.
- At m ≥ 0.15 even perfect information (s̃ = 0.05) is worth under 2 × 10⁻⁵: stop and decide.
- With a reality-gap term of 0.10 (an uncalibrated cell), twenty real pairs at noise 0.3 remove s̃ = 0.096 and are
  worth 0.0251, against 0.0084 for all remaining simulation; with 0.02 (a calibrated cell) the same batch is worth
  0.0035. Calibration does not make simulation faster; it changes which experiment is worth buying.

### 3. Conventions, failure cases and non-claims

- Costs need a declared exchange rate between core-seconds, GPU-seconds and real episodes; a ranking requested
  without one is refused.
- The posterior is Gaussian by assumption, with variance components read from the ledgers; the model does not
  claim that assumption, it is only used to order work.
- Not claimed: any verdict, any validity statement, any gain where candidates are not separated.

### 4. Law-test layout, registry, pilot and gate

`crates/frob-opt/tests/laws_c39_evsi.rs` with `tests/support/c39_ref.rs`: the closed form; an adaptive quadrature
of the defining expectation as the independent oracle; the variance-removal rule; a small menu of experiments
with costs for the ranking and stop rule; a Monte Carlo opportunity-loss estimate with a fixed seed for three
finalists. Generic over a trait `C39ValueSubject`; mutants in `tests/support/c39_mutants.rs`. Registry: KA-187
with a derivation file; the pilot record.

Pilot of the construction: thirty registered experiments on the first mission slice. Falsifier (TARGET): stated 80%
intervals cover fewer than 65% or more than 92% of outcomes, or the direct-measurement comparator beats a
synthesized probe at equal cost. Ceiling: the factor of 2 to 5 above, at the finalist stage only. Fallback: the
fixed order of experiments. Gate GATE-ATEL-15 at M4: on the offline replay the knowledge-gradient chooser is at
least 1.5 times better than the agent chooser in evaluations to a true top-3, and the value is exactly zero on a
fixture where no outcome can change the choice. Kill gate KILL-VOI (M4): GATE-ATEL-15 fails; then value of
information is a stop rule only and `experiment.suggest` lists proposals unranked by value.

## Acceptance criteria

- [ ] `docs/math/C-39.md` exists with part A: E1 to E7 with the derivation of E1, the worked numbers, the
      conventions and the non-claims.
- [ ] `cargo test -p frob-opt --test laws_c39_evsi` passes on the reference model, and on the production value
      model if BD-351 has landed.
- [ ] Every mutant below fails its named law.
- [ ] KA-187 is registered with a derivation file, a stated tolerance and the control named below (the row was
      printed with neither); the pilot record is in the pilot register.
- [ ] `scripts/e2e/spec_c39_evsi.sh` exits 0.

## Test plan

### Unit tests

- `ka_187_limits_and_monotonicity`: EVSI(m, 0) = 0; EVSI(0, s̃) = 0.398942·s̃; over a grid of 10⁴ points the value
  increases in s̃ and decreases in \|m\|; finite-difference slopes equal φ(z) and −Φ(−z) to 10⁻⁶.
- `closed_form_equals_expectation`: for 10³ seeded (m, s̃) the closed form equals the quadrature of
  E[max(m′, 0)] − max(m, 0) to 10⁻¹⁰.
- `variance_removed`: w = 0.03² and v = 0.3²/200 give s̃ = 0.024495; w = 0.03² + 0.04² and v = 0.1²/8 give
  s̃ = 0.040825.
- `worked_values`: 0.001306 and 0.005496; ratio per unit cost 6.415; second fast batch 1.02 × 10⁻⁵; perfect
  information at m = 0.15 worth 1.91 × 10⁻⁵; real batch 0.02510 against 0.00843 for all remaining simulation;
  0.00345 in the calibrated cell.
- `ranking_and_stop`: with a budget price of 10⁻⁴ per GPU-second the first fast batch is bought (1.3 × 10⁻³ per
  GPU-second); the second fast batch (1.0 × 10⁻⁵) is not; the referee batch, re-valued after the first batch is
  absorbed (s̃ = 0.0339, value 0.0035, 1.3 × 10⁻⁴ per GPU-second), is bought; then every remaining ratio is below
  the price and the list ends with STOP.
- `zero_when_choice_cannot_change`: an experiment on a component with zero prior variance has value exactly 0.
- `two_finalists_reduce_to_closed_form`: the opportunity-loss reduction for two finalists equals E1; for three
  finalists the seeded Monte Carlo estimate is nonnegative and at most the value of perfect information.
- `calibration_trigger`: of thirty registered 80% intervals, 19 or fewer covering, or 28 or more, triggers the
  fallback; 20 to 27 does not.
- `no_exchange_rate_refused`: a ranking with mixed cost units and no declared rate is refused.

### End-to-end script and logging

`scripts/e2e/spec_c39_evsi.sh`: `frob --json doctor`; `cargo test -p frob-opt --test laws_c39_evsi`; `cargo test
-p frob-conformance --test g1 -- KA-187`; file and registry checks. Log to
`target/e2e-logs/spec_c39_evsi/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`, `step`,
`event`, command line, exit code, wall time, the evidence block (claim Known answer, class Tested, tier G1 host,
determinism of the host run, seed), values in objective units and per GPU-second against expected with
thresholds, the order of the ranking, and on failure the first differing value. The `summary` line counts
assertions and names GATE-ATEL-15.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-187 | EVSI = s̃·[φ(z) − z·Φ(−z)], z = \|m\|/s̃: 0 when s̃ = 0; increasing in s̃; decreasing in \|m\|; a campaign in which no ratio clears the price stops and decides | 10⁻¹⁰ against the quadrature (the row was printed `to set`) | G1 host | Mutant `value_ignores_mean` (new): returns s̃/√(2·π) for every m and never stops |

Other mutants: `rank_by_value_not_ratio` (buys the referee batch first); `variance_removed_is_prior` (removes w
instead of w²/(w + v)); `issues_verdict` (a subject that returns a decision, rejected by the type test).
Determinism: `f64` closed forms twice on one machine; the Monte Carlo row with its seed.

## Considerations

- The Gaussian model is a convenience; the calibration rule E7 is the guard, and its failure sends the core back
  to the fixed order.
- KILL-VOI and KILL-AUGUR-PROBE do not overlap: the first governs the ranking, the second the probe generator.
- Compute: none; the closed form is constant time. The pilot's thirty experiments are campaign runs owned by the
  mechanism beads.
- Interaction: the reality-gap variance comes from the calibration lane; until a cell is calibrated its term is
  the declared uncalibrated default and real episodes rank high, which is the intended behaviour.

## References

Plan §10 C-39 (value of an experiment); §28.9, §28.16, §28.17; §30.15; Appendix D.8 "Value of sample
information"; GB-36; KA-187; GATE-ATEL-15; KILL-VOI; DC-05; ADR-ATEL-07; C-15, C-25, C-26.
