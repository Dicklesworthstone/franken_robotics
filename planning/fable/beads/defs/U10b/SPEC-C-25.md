---
key: "SPEC-C-25"
title: "spec C-25 (frob-arena): coupled fidelity estimators: control variate, multilevel, doubly robust correction"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "spine", "spec", "solid", "frontier", "frob-arena", "test", "review-notes-open"]
parent: "WP-50"
deps: ["BD-022", "BD-025", "N10-CX", "N10-KA2", "N10a-01", "N38a-14", "SPEC-C-15", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-124", "BD-263", "BD-264", "N16b-07", "SPEC-C-26"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-03", "GATE-GAUNT-01"]
plan_refs: ["§10 C-25", "§10 C-15", "§16.8", "§16.17", "§29.11", "§29.20", "§38.7 MR-33", "Appendix B.5", "Appendix D.8", "Appendix E GB-15", "Appendix F FD-FSIM-17", "Appendix M.9 KA-132 to KA-135", "Appendix N.7 CX-062"]
---
## Background

The simulator has tiers of fidelity: fast batched tiers on the GPU in single precision, a batched CPU tier,
and a slow referee. Finalist designs are usually evaluated on the referee alone, which is expensive, or on the
fast tier alone, which is not trusted. Construction C-25 treats the same world run on two tiers as a **coupled
pair** and gives three estimators that use the coupling to buy referee-grade estimates for a fraction of the
referee episodes where the tiers agree, with a valid error rate where they do not.

Tier: Solid [S] for the estimators as mathematics (this bead); Frontier [F] for their use across contact-rich
tiers and for the audited correction (pilot N16b-07). Owning chapters: Arena (statistics, crate `frob-arena`)
with the dynamics and evidence chapters (the tier-correction ledger). The decision already taken on which
estimator ships: both control variate and audit, as one estimator: a fitted, frozen control-variate
coefficient with a doubly robust audit correction on logged propensities.

Terms. *World address*: (run seed, world, episode); the same address on two tiers is the same scenario.
*Y_lo, Y_hi*: the outcome of one address on the low and high tier. *ρ*: their correlation. *Audit
probability* p: the logged probability with which an address is also run on the high tier. *Prediction* m:
the cheap tier's prediction of the high-tier outcome, fixed before any high-tier outcome exists. *Confidence
sequence*: a sequence of intervals valid at every stopping time (construction C-15).

Rejected alternatives: referee-only evaluation of finalists (the baseline and the fallback); fast tier alone;
a unit-coefficient correction (it does not have the quoted variance); inverse-propensity weighting without a
floor on p (weights can consume the budget); re-simulating short segments from snapshots and calling the
result the same estimand (it estimates a different quantity, which must be named as such).

## Scope

In: `docs/math/C-25.md` (statements S1 to S7); the law suite `crates/frob-arena/tests/laws_c25.rs` with
reference model and mutants; registry rows KA-132 to KA-135; relation MR-33; controls from CX-062; row C-25 of
the construction register.

Out: the tier-correction ledger, propensity logging and the promotion rule as product features (BD-263,
BD-264); the anytime-valid machinery itself (SPEC-C-15, BD-124, BD-262); the batched multilevel entry point in
the sibling library (an upstream work item, FD-FSIM-17); the pilot (N16b-07).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-124 (the two study types of
`frob-arena`): that bead is listed in `related` and is not a prerequisite. The law suite is generic over its
subject trait and closes on the exact reference model and the mutants in its own test support. If `frob-arena`
does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance` (with `--features gauntlet-mutations` in place of the crate's `conformance`); the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-25.S1 | Control variate: θ̂ = Ȳ_hi(n) − β·(Ȳ_lo(n) − Ȳ_lo(N)) with N ≫ n cheap episodes. At the optimal β its variance is Var(Y_hi)/n·(1 − ρ²·(1 − n/N)) | `Certified` as an identity (derivation below); `Tested` by simulation |
| C-25.S2 | The saving is 1/(1 − ρ²) at best: about 2 at ρ = 0.7; a factor of 5 needs ρ ≥ 0.894 and a factor of 10 needs ρ ≥ 0.949 | `Certified` (arithmetic) |
| C-25.S3 | Multilevel: E[Y_L] = E[Y₀] + Σ_ℓ E[Y_ℓ − Y_ℓ₋₁] over the levels GPU `f32`, `batch-cpu` and referee, with N_ℓ ∝ √(V_ℓ/C_ℓ) for level variance V_ℓ and cost C_ℓ. The total cost for variance ε² is (Σ_ℓ √(V_ℓ·C_ℓ))²/ε² | `Certified` (telescoping sum; Lagrange optimum); `Tested` |
| C-25.S4 | Doubly robust tier correction. For each fresh paired scenario i: choose a prediction mᵢ before any high-tier outcome exists; log an audit probability pᵢ ≥ p_min > 0; draw Aᵢ ~ Bernoulli(pᵢ); run the high tier only if Aᵢ = 1; form Zᵢ = mᵢ + Aᵢ·(Yᵢ − mᵢ)/pᵢ. Then E[Zᵢ \| past] = E[Yᵢ \| past] whatever mᵢ is | `Certified` by enumeration on finite outcomes |
| C-25.S5 | Var(Zᵢ) = Var(Yᵢ) + E[(1 − pᵢ)/pᵢ·(Yᵢ − mᵢ)²]: a wrong cheap model inflates variance and never bias | `Certified` (identity) |
| C-25.S6 | The audit assignment is persisted before dispatch and survives retries; p_min bounds \|Zᵢ − mᵢ\| by \|Yᵢ − mᵢ\|/p_min | `Tested` (protocol rule; bound is arithmetic) |
| C-25.S7 | Stopping uses a predictable bounded-increment confidence sequence (construction C-15); the stopping error is at or below α under the null | `Tested` (10⁴ simulated null studies) |

### 2. Derivations (to be reproduced in the contract)

**S1.** Let the N cheap episodes include the n paired ones. Then Ȳ_lo(n) − Ȳ_lo(N) = (1 − n/N)·(Ȳ_lo(n) −
Ȳ_lo(rest)), with variance σ_lo²/n·(1 − n/N), and covariance with Ȳ_hi(n) equal to (1 − n/N)·ρ·σ_hi·σ_lo/n.
So Var(θ̂) = σ_hi²/n + β²·σ_lo²/n·(1 − n/N) − 2β·(1 − n/N)·ρ·σ_hi·σ_lo/n, minimized at β* = ρ·σ_hi/σ_lo, where
it equals σ_hi²/n·(1 − ρ²·(1 − n/N)). With β = 1 the variance is σ_hi²/n + (σ_lo² − 2ρ·σ_hi·σ_lo)·(1 − n/N)/n,
which is not the quoted one.

**S2.** With N ≫ n the referee episodes needed fall by 1/(1 − ρ²): 1.33 at ρ = 0.5, 1.96 at 0.7, 5.3 at 0.9
and 10.3 at 0.95; a factor of 5 needs ρ ≥ √0.8 = 0.894 and a factor of 10 needs ρ ≥ √0.9 = 0.949. With n = 64,
N = 6,400 and ρ = 0.9 the variance ratio is 1 − 0.81·0.99 = 0.198, a factor of 5.0.

**S3.** The telescoping identity is exact. Minimizing Σ N_ℓ·C_ℓ subject to Σ V_ℓ/N_ℓ = ε² gives
N_ℓ = ε⁻²·√(V_ℓ/C_ℓ)·Σ_k √(V_k·C_k) and the stated total cost.

**S4, S5.** Conditional on Yᵢ, E[Aᵢ/pᵢ] = 1, so E[Zᵢ | Yᵢ] = Yᵢ for any mᵢ; and Var(Aᵢ/pᵢ) = (1 − pᵢ)/pᵢ, so
Var(Zᵢ | Yᵢ) = (Yᵢ − mᵢ)²·(1 − pᵢ)/pᵢ. The law of total variance gives S5. Both need mᵢ and pᵢ to be fixed
before Yᵢ exists and Aᵢ to be drawn independently of Yᵢ.

### 3. Worked numbers (exact)

- **Unbiasedness (KA-133).** Y ∈ {0, 1} with mean θ, a constant m, p = 0.1: E[Z] = m + p·(θ − m)/p = θ for
  every m.
- **Variance (KA-134).** With a perfect prediction the added variance is 0. With m wrong by a constant ½ (for
  a binary outcome, m = ½) and p = 0.1 it is (0.9/0.1)·¼ = 9 × ¼ = 2.25, nine times the largest possible
  Var(Y) and worse than auditing everything.
- **Full audit.** With pᵢ = 1 for every i, Zᵢ = Yᵢ and the estimate is the plain paired mean.

### 4. Honest limits, foundations

- The saving is 1/(1 − ρ²) at best. Chaotic contact decorrelates paired worlds over long episodes, and
  re-simulating short segments from snapshots estimates a different quantity, which is named as such.
- Inverse weights can consume the budget; p_min is the countermeasure and is logged.
- The estimator is safe to run before ρ is known; whether it is worth running is the measurement (the pilot).
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-uq` has `mlmc::{mlmc_estimate,
  coupled_mlmc_summary}`, `product_mlmc_allocation::allocate_mlmc` and linear control variates
  (`product_control_variate`). Its importance sampling works on Gaussian germs only and its multilevel driver
  takes a per-call closure, so a batched entry point is an upstream work item (FD-FSIM-17). Not admitted for
  this use until its admission gate passes.

### 5. Law suite and registry (DESIGN)

- `crates/frob-arena/tests/laws_c25.rs`, generic over `C25Subject { fn control_variate(&self, hi: &[f64],
  lo_paired: &[f64], lo_all: &[f64], beta: f64) -> f64; fn mlmc_allocation(&self, v: &[f64], c: &[f64],
  eps2: f64) -> Vec<f64>; fn dr_term(&self, m: Q, a: bool, y: Option<Q>, p: Q) -> Q }`.
  `tests/support/c25_ref.rs`: exact enumeration over finite outcome spaces in rationals (expectation and
  variance of Z); a seeded generator of correlated Gaussian pairs for the simulation tests (seed printed).
  `tests/support/c25_mutants.rs`: mutants under the crate's cargo feature `conformance` (BD-025; the umbrella `gauntlet-mutations` enables it).
- Register row C-25: `verb = "eval.promote"` (with the tier-correction ledger), `failing_test` =
  `frob-arena::laws_c25::s4_audit_after_outcome_is_biased`, `payoff.gate = "GATE-CHRON-09"`, `known_answers =
  KA-132 … KA-135`, `pilot = "PILOT-C-25"`, `generic_comparator = "fixed escalation of finalists to the
  referee (plain paired promotion)"`.
- Gates that measure the construction (recorded by BD-264): GATE-FID-08 at M3 (type-I error at most α over 10⁴
  null comparisons with optional stopping; equals the paired mean at pᵢ = 1; a planted between-tier reversal is
  not established; ρ and the replaced higher-tier count printed) and GATE-CHRON-09 at M3 (equal to the plain
  mean at n = N; coverage at least nominal − 0.03; referee episodes per established verdict 16 / 64 / 256 at
  the 50th / 95th / 99th percentile).
- Pilot (carried in full; run by N16b-07, record `PILOT-C-25`): ρ on 200 world addresses, from paired data the
  fidelity gate already pays for. Falsifier (TARGET): ρ below 0.7, or failure to beat referee-only by a factor
  of 2 on high-agreement strata. Ceiling: 1/(1 − ρ²) on the referee share of a decision. Fallback: fixed
  escalation of finalists to the referee.
- Kill gate KILL-DR (decided at M3; §29.20 governs where §16.17 differs): ρ_ep below 0.7 on 200 paired
  addresses in every stratum tested, or GATE-CHRON-09 coverage fails after one repair cycle. Fallback: the tier
  correction is off and promotion returns to the plain paired label of construction C-15, from referee
  episodes alone (segment mode stays an option inside that fallback); the ledger keeps recording ρ_ep. The
  effective-sample-size diagnostic of the correction is recorded and is not a trigger.

## Acceptance criteria

- [ ] `docs/math/C-25.md` has the ten sections, statements C-25.S1 to S7 with classes, the derivations and the
      worked numbers.
- [ ] `cargo test -p frob-arena --test laws_c25` passes; with `--features conformance` every mutant fails.
- [ ] KA-132: the simulated variance of the control-variate estimator at β* matches σ_hi²/n·(1 − ρ²·(1 − n/N))
      within 3% over 20,000 seeded replications at (n, N, ρ) = (64, 6,400, 0.9); the quoted thresholds 0.894
      and 0.949 are reproduced to three digits.
- [ ] KA-133: E[Z] = θ exactly for m ∈ {0, 1/4, 1/2, 1, 7}, θ ∈ {1/10, 1/2, 9/10}, p ∈ {1/10, 1/2, 1}.
- [ ] KA-134: added variance 0 for a perfect prediction and exactly 9/4 for m = ½, p = 1/10.
- [ ] KA-135: over 10⁴ seeded null studies the stopping error is at or below α = 0.05; a planted between-tier
      reversal in one stratum is not accepted.
- [ ] Every KA row of this construction has a registered control (the appendix states none for three of them).
- [ ] `scripts/e2e/spec_c25.sh` exits 0.

## Test plan

### Unit tests

- `s1_optimal_beta_variance`: the acceptance values; `s1_unit_beta_is_not_optimal`: with σ_hi = 1, σ_lo = 2,
  ρ = 0.9 the β = 1 variance exceeds the β* variance by the factor the formulas give.
- `s2_saving_table`: 1/(1 − ρ²) at ρ = 0.5, 0.7, 0.9, 0.95 equals 1.333, 1.961, 5.263, 10.256 to three decimals.
- `s3_telescoping_exact`: three levels with rational means; `s3_allocation_optimum`: V = (4, 1, 1/4),
  C = (1, 10, 100), ε² = 1/100: N_ℓ proportional to (2, 0.3162, 0.05) and total cost (2 + √10 + 5)²·100.
- `s4_unbiased_by_enumeration`, `s5_variance_identity_by_enumeration`: exact rationals.
- `s4_full_audit_is_plain_mean`: p = 1 gives Z = Y.
- `s6_pmin_bound`: |Z − m| ≤ |Y − m|/p_min on 1,000 seeded cases; `s6_assignment_survives_retry`: re-dispatch
  of an address returns the persisted audit bit, never a fresh draw.
- `s7_null_stopping_error`, `s7_planted_reversal_not_accepted`.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c25.sh` (idempotent; seed 2500 printed): `frob --json doctor`; the law suite with and without
`--features conformance`; `FROB_TEST_SEED=2500 cargo test -p frob-conformance --test g1_known_answers -- KA-132
KA-133 KA-134`; `FROB_TEST_SEED=2500 cargo test -p frob-conformance --test g0_laws -- KA-135`;
`FROB_TEST_SEED=2500 cargo test -p frob-conformance --test g3_metamorphic -- MR-33` on the host site (the six
Gauntlet targets of BD-025; each again with `--features gauntlet-mutations`, under which every registered
control of these rows must fail). Sources the STD-E2E helper; writes `target/e2e-logs/spec_c25/<utc>.ndjson` and a transcript;
each line has `ts`, `script`, `bead` (`SPEC-C-25`), `step`, `event` (`start`, `cmd`, `assert`, `metric`,
`refusal`, `end`), command line, exit code, wall time, evidence block (claim Known answer, class, tier G0 or
G1 host, determinism class, seed), measured against expected variance or expectation with the tolerance, the
empirical stopping error against α, and on failure the first parameter tuple whose value is out of tolerance.
Final `summary`: assertion counts, GATE-GAUNT-02 and GATE-GAUNT-03.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-132 | Variance Var(Y_hi)/n·(1 − ρ²·(1 − n/N)) at the optimal β; saving 1/(1 − ρ²): about 2 at 0.7, 5 at 0.894, 10 at 0.949 | exact (formula); three digits for the quoted ρ; 3% for the simulated variance | G1 · host | Registered here (the appendix states none): a unit coefficient β = 1 in place of the fitted one |
| KA-133 | E[Z] = m + p·(θ − m)/p = θ for every m | exact | G1 · host | Registered here: audit bits chosen after the outcomes (audit only when Y differs from m), which biases the estimate |
| KA-134 | Added variance 0 with a perfect prediction; 9 × ¼ = 2.25 with m wrong by ½ and p = 0.1 | exact | G1 · host | CX-062: the belief that a roughly right cheap prediction suffices; registered mutant: the factor p/(1 − p) in place of (1 − p)/p |
| KA-135 | Stopping error at or below α over 10⁴ simulated null studies | at or below α | G0 · host | A between-tier reversal planted in one stratum must not be accepted |

Relation MR-33: the tier-corrected estimate has the same mean for every fast-tier prediction, and with every
address audited it is the plain mean; its control is audit bits chosen after the outcomes. Determinism: exact
rows are class D0; simulation rows use a seeded generator and are bit-identical twice on one machine and in
debug and release.

## Considerations

- The correction is funded only where the measured ρ supports it; below 0.7 it costs more than it saves.
- The audit lane must never be cancelled or re-drawn: an audit assignment that depends on outcomes destroys
  the unbiasedness that is the estimator's whole value. The persistence rule (S6) is therefore tested, not
  assumed.
- A bound obtained on a fast tier is provisional until promoted; the estimator's output carries the tier pair,
  ρ and the count of replaced high-tier episodes.
- The effective-sample-size diagnostic of the correction is recorded and is not a kill trigger.

## References

Plan §10 C-25, C-15; §16.8; §29.11; §38.7 (MR-33); Appendix B.5 (the estimator for cheap referee promotion);
Appendix D.8 (control-variate, multilevel and doubly robust estimators); Appendix E GB-15; FD-FSIM-17; KA-132
to KA-135; CX-062; GATE-FID-08; GATE-CHRON-09; KILL-DR (§29.20, §16.17); PILOT-C-25 (N16b-07).
