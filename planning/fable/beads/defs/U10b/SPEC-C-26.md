---
key: "SPEC-C-26"
title: "spec C-26 (frob-opt): rungs trusted by rank agreement (Kendall τ) and split-conformal survivor margins"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "spine", "spec", "frontier", "frob-opt", "test", "review-notes-open"]
parent: "WP-62"
deps: ["BD-022", "BD-025", "N10-CX", "N10-KA2", "N10a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-145", "BD-262", "BD-265", "BD-310", "BD-328", "BD-329", "SPEC-C-39"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-26", "§9 D-10", "§9 D-17", "§16.9", "§23.11", "§28.9", "§28.10", "§28.12", "§40.7", "Appendix D.8", "Appendix E GB-35", "Appendix I.13", "Appendix M.9 KA-136 KA-137", "Appendix N.7 CX-063 CX-064"]
---
## Background

A design campaign climbs a **ladder of rungs**, each about an order of magnitude dearer than the one below:
from a static check, through the baseline controller, a family policy without adaptation and a fine-tune
fraction, to specialist training; and from the fast tier to the referee. The usual way to use such a ladder is
successive halving on faith, with a screening rung whose accuracy was tested once on a corpus. Construction
C-26 says a rung is **trusted by measurement**: by the rank agreement between two rungs on the same designs,
and by a conformal margin that turns "which designs go up" into a decision with a stated miss rate. The number
of designs that need the expensive rung becomes a measured quantity. Screening is audited: a sample of
discarded strata goes up anyway, and a planted winner must survive.

Tier: Frontier [F]. Owning chapter: Atelier (the studio loop); crate `frob-opt`. It serves BET-17 (rank
agreement between rungs). Its pilot is called the decisive early experiment of the program, because the
largest term of the ten-fold thesis (designs that never train) depends on whether a cheap rung orders designs
the way an expensive one does.

Terms. *Rung*: one level of evaluation cost and fidelity. *Concordant pair*: two designs ordered the same way
by both rungs. *Top-k recall*: the share of the expensive rung's best k designs that are among the cheap
rung's best k. *Residual*: |y_hi − ĝ(y_lo)|, with y_lo and y_hi the scores of one design on the cheap and
expensive rung and ĝ a fixed map from cheap to expensive score. *Audit set*: designs that ran both rungs,
drawn by keyed randomness before any score exists. *Exchangeable*: the joint distribution does not change
under reordering; here, the scored design is drawn like the audit designs.

Rejected alternatives: successive halving with a fixed ratio and no measurement (kept as the baseline and
every fallback, at η = 3 with the same audit lane); calibrating the margin on the designs that were promoted
(they are the ones that ran both rungs, and the margin then under-covers); a screening accuracy measured once
on a corpus (it does not transfer across morphology classes or task families).

## Scope

In: `docs/math/C-26.md` (statements S1 to S8); the law suite `crates/frob-opt/tests/laws_c26.rs` with reference
model and mutants; registry rows KA-136 and KA-137; controls from CX-063 and CX-064; row C-26 of the
construction register; the pilot protocol for the beads that run it.

Out: the rank-agreement ledger and the ladder as product features (BD-328, BD-329); rank agreement between
physics tiers (BD-265); screening audits and their exact bound (BD-345); the value-of-information chooser
(construction C-39); pre-registration and execution of the pilot (BD-145, BD-310).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-262 (`eval.compare` on
devices, the first seed in `frob-opt`): that bead is listed in `related` and is not a prerequisite. The law suite
is generic over its subject trait and closes on the exact reference model and the mutants in its own test support.
If `frob-opt` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance` (with `--features gauntlet-mutations` in place of the crate's `conformance`); the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-26.S1 | A rung is trusted by measurement. Rank agreement between two rungs on n designs is Kendall's τ = (concordant − discordant pairs)/C(n, 2), reported with top-k recall and bootstrap intervals, per morphology class, task family and region of design space | Definition; `Tested` (arithmetic on fixtures) |
| C-26.S2 | The gates read τ on the top band (the designs in the top m of either rung, with m = max(10, 3k)), at the lower bootstrap bound. Thresholds, fixed before data (TARGET): at or above 0.7 the cheap rung may select; from 0.5 to below 0.7 it may only discard; below 0.5, or with no record, it neither selects nor discards | Rule; `Tested` (mode assignment) |
| C-26.S3 | Conformal margin: on an audit set of n designs that ran both rungs, drawn by keyed randomness before any score exists and run at fixed n, q̂ is the ⌈(n + 1)·(1 − α)⌉-th smallest residual \|y_hi − ĝ(y_lo)\| | Definition; `Certified` (index arithmetic) |
| C-26.S4 | For a design exchangeable with the audit set the residual is at most q̂ with probability at least 1 − α | `Tested` (null calibration over simulated studies; the guarantee needs exchangeability, which no checker decides) |
| C-26.S5 | Every design whose cheap score is within 2·q̂ of the k-th best cheap score is promoted; a pair separated by more than the margin is misordered with probability at most 2α | `Tested`; the bound is a union bound over two residuals |
| C-26.S6 | A finite margin needs n ≥ (1 − α)/α: fewer than 19 audit designs cannot support α = 0.05 | `Certified` (arithmetic) |
| C-26.S7 | Conditional on the calibration set, coverage is distributed Beta(n + 1 − l, l) with l = ⌊(n + 1)·α⌋; it holds on average over calibration draws and moves by several points at twenty calibration designs | `Certified` (a standard result, quoted; arithmetic checked) |
| C-26.S8 | A rung cannot separate designs closer than twice its own error: if more than 3k designs survive, the rung is skipped for the top band or more episodes are bought | Rule; `Tested` |

### 2. Definitions made precise (DESIGN where the source leaves a choice)

- **Ties in τ.** The formula is τ-a as written: a pair tied on either rung counts as neither concordant nor
  discordant, and the denominator stays C(n, 2).
- **ĝ.** A monotone map fitted on a separate split of the audit history and frozen before the audit residuals
  are computed (the identity when both rungs report the same metric on the same scale).
- **Higher is better** for scores; the k-th best cheap score is the k-th largest.
- **Modes.** `Selective`, `DiscardOnly`, `Uninformative`, assigned per stratum from the rank-agreement record;
  a rung without a record may not discard (refusal FROB-REF-537: a rung used to select or discard without a
  rank-agreement record that meets its threshold; exit code 3; repairs: run the rank-agreement experiment for
  that stratum; fall back to the fixed halving table).
- **Type sketch.**

```rust
pub enum LadderMode { Selective, DiscardOnly, Uninformative }
pub enum AllocationRule {
    FixedTable { eta: u8 },                                        // successive halving, eta = 3
    ConformalSurvivors { alpha: Ratio, q_hat: f64, audit_n: u32 }, // calibrated on the audit lane only
}
```

### 3. Worked numbers

- **Index (KA-136; exact).** n = 39 and α = 0.05 select the 38th smallest residual: ⌈40 × 0.95⌉ = 38. With
  n = 18, ⌈19 × 0.95⌉ = ⌈18.05⌉ = 19 exceeds n, so no finite margin exists; n = 19 gives 19, the largest
  residual.
- **Margin in use (illustrative).** n = 39, q̂ = 0.04, k = 3, third-best cheap score 0.74: the cut is at
  0.74 − 2 × 0.04 = 0.66. Coverage is Beta(38, 2): mean 0.95, below 0.884 in one calibration draw in twenty.
- **Ladder cost (illustrative).** 81 designs with rungs of 10 s, 3 min and 30 min: 40.5 h if all are trained;
  6.1 h by halving at η = 3 (81 × 10 s + 27 × 3 min + 9 × 30 min); 2.0 h if six survive the cut, are scored at
  the second rung, and three of them are trained (81 × 10 s + 6 × 3 min + 3 × 30 min); the audit lane adds
  57 minutes when its 19 designs are scored at the second rung. With q̂ = 0.10 about twenty designs pass, more
  than 3k = 9, and the layer gives back most of its advantage.
- **Kendall τ on small cases (exact).** Four designs ranked identically: τ = 1. Reversed: τ = −1. One adjacent
  exchange: (5 − 1)/6 = 2/3.

### 4. Honest limits, foundations

- FACT (source register, to be re-read): no published measurement of rank agreement between a shared policy and
  trained specialists was found; the nearest figures are rank correlations of 0.4 to 0.8, and cheap rungs are
  biased, not only noisy.
- Conformal coverage holds on average over calibration draws and moves by several points at twenty calibration
  designs. It needs exchangeability with the designs being scored; synthetic drift must trip an alarm.
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-surrogate` has split-conformal
  bands; `fs-race` has racing. Not admitted for this use until their admission gates pass.

### 5. The pilot, carried in full (pre-registered by BD-145; run by BD-310 at M3 and BD-328 at M4)

The decisive early experiment of the program. Forty designs in one class, each scored by the shared policy on
the fast tier, by a fine-tune at 5% of specialist budget, and by a specialist on the referee. Measures: τ,
top-10 recall, and per-episode ρ on 256 paired episodes per design. **Kill (TARGET, fixed before data):
top-of-ranking τ below 0.5 or ρ below 0.7.** Cost: about 45 GPU-hours, or about 5 if specialists are shared
with the family-policy pilot of construction C-23 (an estimate; it needs an owner-approved envelope). Ceiling:
the share of designs that never train. Fallback: successive halving at η = 3 with the same audit lane. Kill
gates in the register: KILL-RANK (the rank-agreement gate fails for every morphology class tested; then the
shared-policy rung is `Uninformative` everywhere, every candidate that survives static analysis is scored on
the higher rung, and the ladder's claim shrinks to stopping and elimination among trained candidates) and
KILL-LADDER (the conformal layer fails its gate after one repair cycle; then the fixed halving table is the
allocation rule and the audit lane is kept as measurement). If the compute envelope is not approved, cheap
rungs are `Uninformative` and neither select nor discard.

### 6. Law suite and registry (DESIGN)

- `crates/frob-opt/tests/laws_c26.rs`, generic over `C26Subject { fn kendall_tau(&self, lo: &[f64], hi: &[f64])
  -> Ratio; fn top_k_recall(&self, lo, hi, k) -> Ratio; fn conformal_index(&self, n: u32, alpha: Ratio) ->
  Option<u32>; fn q_hat(&self, residuals: &[f64], alpha: Ratio) -> Option<f64>; fn survivors(&self, cheap:
  &[f64], k: usize, q_hat: f64) -> Vec<usize>; fn mode(&self, tau_lower: f64, has_record: bool) -> LadderMode }`.
  `tests/support/c26_ref.rs`: exact pair counting, exact index arithmetic in rationals, a seeded generator of
  exchangeable and drifting score pairs. `tests/support/c26_mutants.rs`: mutants under the crate's cargo feature
  `conformance` (BD-025; the umbrella `gauntlet-mutations` enables it).
- Register row C-26: `verb = "eval.rank-agreement"` (with `opt.cascade`), `failing_test` =
  `frob-opt::laws_c26::s3_calibration_on_promoted_undercovers`, `payoff.gate = "GATE-ATEL-07"`,
  `known_answers = KA-136, KA-137`, `pilot = "PILOT-C-26"` (the record written by BD-145 and BD-310),
  `generic_comparator = "successive halving at η = 3"`.
- Gates that measure the construction (recorded by mechanism beads): GATE-ATEL-07 (per morphology class, on 40
  designs: τ on the top band at least 0.7 for `selective` and at least 0.5 for `discard-only`, and ρ_ep at
  least 0.7, all at the lower bootstrap bound; a class below 0.5 is `uninformative`); GATE-ATEL-08 (held-out
  coverage at least 1 − α − 0.03 per stratum; audited top-k recall no lower than halving's; a paired 95% lower
  bound above 1 on GPU-seconds to the same accepted decision; a planted over-scored design caught at the next
  rung; synthetic drift trips the alarm); GATE-CORTEX-12 (the record exists and a rung without one is
  refused); GATE-FID-04 and GATE-REFLEX-04 (the same thresholds for tier pairs and for the baseline rung).

## Acceptance criteria

- [ ] `docs/math/C-26.md` has the ten sections, statements C-26.S1 to S8 with classes, the thresholds tagged
      TARGET, the worked numbers and the pilot protocol.
- [ ] `cargo test -p frob-opt --test laws_c26` passes; with `--features conformance` every mutant fails.
- [ ] KA-136: index 38 for (39, 0.05); `None` for n ≤ 18 at α = 0.05; 19 for n = 19.
- [ ] KA-137: on the planted ladder a planted over-scored design is promoted by the cheap rung and caught at
      the next rung; synthetic drift between audit and scored designs trips the exchangeability alarm; a ladder
      that passes either plant is rejected.
- [ ] Null calibration: over 10⁴ seeded exchangeable studies at n = 39, α = 0.05 the mean coverage is within
      0.01 of 0.95 and the misordering rate of pairs separated by more than 2·q̂ is at most 2α.
- [ ] The mutant that computes q̂ from promoted designs only under-covers on the fixture where the residual
      correlates with the cheap score (coverage below 0.95 − 0.03).
- [ ] Mode assignment: 0.72 → `Selective`; 0.61 → `DiscardOnly`; 0.49 → `Uninformative`; any value without a
      record → `Uninformative`.
- [ ] `scripts/e2e/spec_c26.sh` exits 0.

## Test plan

### Unit tests

- `s1_tau_small_cases`: 1, −1 and 2/3 as above; `s1_tau_ties`: a tie on one rung leaves the pair uncounted.
- `s1_top_k_recall`: cheap order (a, b, c, d, e), expensive order (b, a, e, c, d), k = 2: recall 1; k = 3: 2/3.
- `s2_top_band`: n = 40, k = 3: m = max(10, 9) = 10; the band is the union of the two top-10 sets.
- `s3_index_arithmetic`, `s6_minimum_audit_size`: the KA-136 values and n ≥ 19 for α = 0.05, n ≥ 9 for α = 0.1.
- `s4_null_coverage`, `s5_misordering_bound`: the null-calibration values.
- `s5_survivor_set`: scores (0.90, 0.80, 0.74, 0.70, 0.66, 0.65), k = 3, q̂ = 0.04: survivors are the first
  five (cut 0.66, inclusive).
- `s7_beta_mean`: for n = 39, α = 0.05: l = 2 and the mean of Beta(38, 2) is 38/40 = 0.95.
- `s8_too_many_survivors`: more than 3k survivors flags the rung for skipping in the top band.
- `ka137_planted_overscore`, `ka137_drift_alarm`, `s3_calibration_on_promoted_undercovers`,
  `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c26.sh` (idempotent; seed 2600 printed): `frob --json doctor`; the law suite with and without
`--features conformance`; `FROB_TEST_SEED=2600 cargo test -p frob-conformance --test g1_known_answers -- KA-136`;
`FROB_TEST_SEED=2600 cargo test -p frob-conformance --test g0_laws -- KA-137` (the tier targets by the names
BD-025 gives them; `g1` and `g0` are not test targets). Sources the STD-E2E helper; writes
`target/e2e-logs/spec_c26/<utc>.ndjson` and a transcript; each line has `ts`, `script`, `bead` (`SPEC-C-26`),
`step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), command line, exit code, wall time,
evidence block (claim Known answer, class, tier G0 or G1 host, determinism class, seed), the index and the
coverage against their expected values and thresholds, a `refusal` line for the expected FROB-REF-537, and on
failure the first study whose coverage or index is wrong. Final `summary`: assertion counts and GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-136 | n = 39 and α = 0.05 select the 38th smallest residual; fewer than 19 audit designs cannot support α = 0.05 | exact | G1 · host | CX-063: computing q̂ from promoted designs only under-covers on a fixture where the residual correlates with the cheap score; CX-064: a dozen audit designs taken as enough |
| KA-137 | A planted over-scored design is caught at the next rung; synthetic drift trips the exchangeability alarm | exact (verdict) | G0 · host | The two plants are the controls: a ladder that passes either is rejected |

Further controls (mutants): an index ⌈n·(1 − α)⌉ without the + 1 (it under-covers); a margin of q̂ in place of
2·q̂ (the misordering bound fails); a mode assigned from the point estimate of τ instead of its lower bound.
Metamorphic relations: τ is unchanged by any strictly increasing transformation of either rung's scores;
adding a constant to every cheap score leaves the survivor set unchanged. Determinism: exact rows are class
D0; simulation rows are seeded and bit-identical twice on one machine and in debug and release.

## Considerations

- One pair of thresholds, 0.7 and 0.5, is used everywhere a rule depends on τ. The compendium's row for
  Kendall τ words the 0.7 threshold as the level at which the cheap rung "is used to discard"; the gate
  register governs, and reads 0.7 as the level for selecting and 0.5 as the level for discarding.
- The audit lane is never cancelled and never drawn after scores exist; both rules are what make S4 true.
- Cheap rungs are biased, not only noisy; a high τ on one class says nothing about another, hence records per
  class, task family and region.
- Compute for the pilot needs the owner's approval; the fail-closed default is `Uninformative` rungs.

## References

Plan §10 C-26; §9 D-10, D-17; §16.9; §23.11; §28.9, §28.10, §28.12; §40.7; Appendix D.8 (Kendall τ,
split-conformal survivor set); Appendix E GB-35; Appendix I.13; KA-136, KA-137; CX-063, CX-064; BET-17;
GATE-ATEL-07, -08; GATE-CORTEX-12; KILL-RANK; KILL-LADDER; FROB-REF-537.
