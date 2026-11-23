---
key: "SPEC-C-27"
title: "spec C-27 (frob-nemesis): rare failures in two lanes: bounded-weight bounds certify, splitting explores"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "spine", "spec", "solid", "frontier", "frob-nemesis", "test"]
parent: "WP-55"
deps: ["BD-025", "SPEC-C-15", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-27", "§10 registries, Appendix M.9 and Appendix N.7: the chapter registry beads N10-KA (unit U10a) and N10-CX (unit U10c) that load rows KA-138 to KA-141 and CX-065, CX-066; this bead binds them to tests, sets the tolerance the appendix leaves `to set` and registers the controls it leaves unstated", "§38.7: the metamorphic-relation registry row MR-08 (fork and replay: a child with its own address has fresh streams) in `frob-conformance/registry/relations.toml` (Gauntlet beads)"]
related: ["N10b-18", "BD-276", "BD-277", "BD-273", "BD-205", "BD-180", "BD-259", "SPEC-C-28", "SPEC-C-30", "BD-270", "N10a-02"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-03", "GATE-GAUNT-01"]
plan_refs: ["§10 C-27", "§9 D-16", "§11.1", "§26.9", "§26.12", "§26.14", "§38.7 MR-08", "§38.12", "Appendix D.8", "Appendix E GB-32", "Appendix M.9 KA-138 to KA-141", "Appendix N.7 CX-065 CX-066", "Appendix N CX-292"]
---
## Background

Two questions about failures are routinely confused. *Where does this design fail?* is a search, and any
method that finds failures faster is welcome. *Is its failure probability below a target?* is a claim, and only
estimators with a valid finite-sample bound may make it. The usual practice (waiting for failures under domain
randomization, or extrapolating a fitted tail past the data) answers neither well. Construction C-27 handles
the failure probability p of a design on a world distribution in **two lanes that never mix**: a certification
lane whose validity does not depend on where anyone looked, and an exploration lane that finds failures fast
and issues no bound. This is doctrine rule D-16.

Tier: Solid [S] for crude and bounded-weight bounds; Frontier [F] for splitting and adaptive proposals (pilot
N10b-18). Owning chapter: Nemesis; crate `frob-nemesis` (verb `nemesis.bound` for certification). It serves
BET-32 (rare-failure bounds, for refutation and for estimates once failures are observed). A bound's proof
class is `Tested` at confidence 1 − δ, never `Certified`, which is reserved for certificates a checker
accepted.

Terms. *Nominal episode*: one drawn from the task's declared randomization. *Proposal law* q: a different law
used to sample more failures. *Likelihood ratio* w = dp/dq. *Defensive mixture*: q = β·p + (1 − β)·q′, a
proposal that keeps a share β of the nominal law. *Distress score*: an online margin that falls as a world
approaches failure. *Splitting*: restoring a snapshot of a world that reached a distress level and continuing
it many times. *World address*: (run seed, world, episode); a random stream is keyed by (world address, step,
stream id), and a fork's child has its own address. *Coefficient of variation*: standard deviation over mean.

Rejected alternatives: extreme-value extrapolation (checking a fitted tail at 10⁻⁶ costs the nominal episodes
that certify directly, so it is not built); a bound computed from exploration data (the search chose where to
look); a mixture proposal sold as a cheaper certificate of absence (it is not, shown below).

## Scope

In: `docs/math/C-27.md` (statements S1 to S9); the law suite `crates/frob-nemesis/tests/laws_c27.rs` with
reference model and mutants; registry rows KA-138 to KA-141; relation MR-08; controls from CX-065, CX-066 and
CX-292; row C-27 of the construction register.

Out: the certification verb and its confidence sequences as product features (BD-276); splitting on the
simulator's forks (BD-277); the falsification search (BD-273); the decision table that orders exploration and
certification for a finalist (the Nemesis chapter's decision core); the pilot (N10b-18).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-270 (the search space and
legality profile of `frob-nemesis`): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-nemesis` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-27.S1 | The two lanes never mix: a method that finds failures quickly issues no bound; a bound comes from crude sampling, bounded-weight importance sampling or a sealed confirmation run | Rule; `Tested` (a bound refuses episodes not drawn nominally or with recorded bounded weights: FROB-REF-560) |
| C-27.S2 | Zero-failure bound: n independent nominal episodes with zero failures give p ≤ 1 − δ^(1/n) ≤ ln(1/δ)/n at confidence 1 − δ, sequentially by an exact binomial bound | `Certified` (arithmetic); coverage `Tested` |
| C-27.S3 | Bounded-weight importance sampling: with a proposal q and likelihood ratio w = dp/dq bounded by w_max on the support, the estimate n⁻¹·Σ wᵢ·1[fail] is unbiased with terms in [0, w_max], so bounded-variable confidence sequences apply | `Certified` (unbiasedness by enumeration on finite laws); coverage `Tested` |
| C-27.S4 | Weights truncated at c give a valid lower bound, used to refute a claimed rate | `Certified` (E[min(w, c)·1[fail]] ≤ p) |
| C-27.S5 | Defensive mixture: with q = β·p + (1 − β)·q′ the weights are at most 1/β, and zero observed failures certify only p ≤ ln(1/δ)/(β·n), never better than crude sampling on the nominal share | `Certified` (arithmetic) |
| C-27.S6 | Splitting on a distress score with fixed levels forks a world that reaches a level into children whose random streams are keyed by child world addresses. The product of conditional fractions estimates p with squared coefficient of variation near L·(1 − p₀)/(p₀·N) for L levels of conditional probability p₀ and N trials per level | `Estimated` (the rate); `Tested` (the formula on a closed-form fixture, under independence) |
| C-27.S7 | Splitting supplies discovery and `Estimated` rates only: its trials are dependent and its accuracy rests on the score. A fork without fresh randomness is an identical copy | Rule; `Tested` (duplicate detection) |
| C-27.S8 | A catalogue of failure modes carries a Good–Turing estimate of the unseen mass (modes seen once divided by failures seen), which says when exploration has stopped finding new modes | `Estimated`; a stopping signal, not a bound |
| C-27.S9 | A bound is scoped to its tier, and found failures are replayed one tier up. No sampler discounts the price of certifying absence without assuming where failures live | Rule and non-claim |

### 2. Formulas and derivations

- **S2.** With no failure in n independent episodes, the largest p not rejected at level δ solves (1 − p)ⁿ = δ:
  p ≤ 1 − δ^(1/n). Since −ln(1 − p) ≥ p, this is at most ln(1/δ)/n. The episodes needed for a target p* are
  n = ⌈ln δ/ln(1 − p*)⌉, about ln(1/δ)/p*.
- **S3.** E_q[w·1[fail]] = E_p[1[fail]] = p. Each term lies in [0, w_max].
- **S5.** w = p/q = p/(β·p + (1 − β)·q′) ≤ 1/β. The observation β·w·1[fail] lies in [0, 1] with mean β·p; a
  zero-failure bound on that mean, divided by β, gives the statement.
- **S6.** The estimate is the product of L conditional fractions. If the fractions were independent binomial
  proportions with success probability p₀ over N trials, each would have squared relative error
  (1 − p₀)/(p₀·N), and the product L times that to first order. Correlation between continuations of one
  parent makes the true value larger.
- **Closed-form walk.** For a random walk on the integers with up-probability u and down-probability d = 1 − u,
  the probability of reaching level b before level 0 from level 1 is (1 − d/u)/(1 − (d/u)ᵇ) for u ≠ d.

### 3. Worked numbers

- **Zero-failure counts (KA-138; exact).** δ = 0.05: n = 29,956 episodes for p* = 10⁻⁴, 299,572 for 10⁻⁵ and
  2,995,731 for 10⁻⁶ (the looser count ⌈ln 20/p*⌉ is 29,958 for 10⁻⁴: "about 3·10⁴"). At an illustrative 400
  episodes per second: 75 s, 12.5 minutes and 2.1 hours. At the level of the tenth request of a qualification
  campaign, 0.05/110, the first count is 76,959.
- **Mixture (KA-139; exact).** With β = 0.1 the same 29,956 clean episodes certify only 10⁻³.
- **Splitting (KA-140).** L = 6, p₀ = 0.1, N = 10⁴: squared coefficient of variation 6 × 0.9/(0.1 × 10⁴) =
  0.0054, coefficient of variation 7.3% ("about 7%"); 6 × 10⁴ fragments, about fifty times fewer than the
  3 × 10⁶ nominal episodes for 10⁻⁶.
- **Walk (KA-141).** Up-probability 0.4, from level 1, reaching level 20 before level 0:
  0.5/(1.5²⁰ − 1) = 1.504 × 10⁻⁴.
- **Gaussian tail.** P(z > 4) = 3.167 × 10⁻⁵, recovered with a mean shift of 4 and β = 0.1, every weight at
  most 10.

### 4. Honest limits, refusals, foundations

- Payoff, with its evidence: failures near 10⁻⁴ to 10⁻⁵ are found and their rate estimated with 2 to 20 times
  fewer episodes in the published record (FACT, source register, to be re-read) and one to two orders of
  magnitude as a TARGET, when the score is informative.
- Refusals owned by the implementing chapter and exercised here: FROB-REF-560 (a bound was offered episodes
  that were not drawn nominally or with recorded bounded weights; exit code 3; repairs: draw nominal episodes;
  record the proposal and its weight bound before the run) and FROB-REF-569 (splitting requested where no
  randomness enters after the fork; exit code 3; repairs: use tilting on static parameters; add per-step
  disturbances to the task).
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-uq` importance sampling on Gaussian
  germs; `fs-rand` keyed streams; `fs-eproc` (a Bernoulli confidence sequence). Not admitted for this use until
  their admission gates pass.

### 5. Law suite and registry (DESIGN)

- `crates/frob-nemesis/tests/laws_c27.rs`, generic over `C27Subject { fn zero_failure_upper(&self, n: u64,
  delta: f64) -> f64; fn episodes_needed(&self, p_star: f64, delta: f64) -> u64; fn is_estimate(&self, draws:
  &[WeightedOutcome], w_max: f64) -> Result<Interval, Refusal>; fn truncated_lower(&self, draws, c: f64,
  delta: f64) -> f64; fn split(&self, walk: &WalkSpec, levels: &[i64], n_per_level: u64, key: u64) ->
  Result<SplitEstimate, Refusal>; fn unseen_mass(&self, mode_counts: &[u64]) -> f64 }`.
  `tests/support/c27_ref.rs`: exact integer search for the smallest n with (1 − p*)ⁿ ≤ δ in high-precision
  arithmetic; the biased walk with keyed streams; a discrete nominal law and proposal for exact enumeration of
  the weighted estimator. `tests/support/c27_mutants.rs`: mutants under the cargo feature `controls`.
- Register row C-27: `verb = "nemesis.bound"`, `failing_test` =
  `frob-nemesis::laws_c27::s7_fork_without_fresh_randomness_refused`, `payoff.gate = "GATE-NEMESIS-04"`,
  `known_answers = KA-138 … KA-141`, `pilot = "PILOT-C-27"` (for splitting), `generic_comparator = "crude
  nominal sampling with the zero-failure rule at the same budget"`.
- Gates that measure the construction (recorded by BD-276 and BD-277): GATE-NEMESIS-04 at M3 (over 10⁴
  synthetic studies with known p the 95% upper bound covers p in at least 95%; the defensive mixture covers
  under a mis-centred proposal; non-nominal episodes are refused); GATE-NEMESIS-05 at M3 (on planted failures of
  probability 10⁻⁴ and 10⁻⁵, at least 10 times fewer episodes than crude sampling at 10⁻⁵, both modes found in
  at least 80% of repetitions). Kill gate KILL-RARE (N10b-18 states it).

## Acceptance criteria

- [ ] `docs/math/C-27.md` has the ten sections, statements C-27.S1 to S9 with classes, the derivations and the
      worked numbers.
- [ ] `cargo test -p frob-nemesis --test laws_c27` passes; with `--features controls` every mutant fails.
- [ ] KA-138: `episodes_needed` returns exactly 29,956, 299,572 and 2,995,731 at δ = 0.05, and 76,959 at
      δ = 0.05/110 for 10⁻⁴.
- [ ] KA-139: with β = 1/10 and 29,956 clean episodes the certified bound is at most 1.0001 × 10⁻³ and not
      below 10⁻³·(1 − 10⁻³).
- [ ] KA-140: the formula gives 0.0054; on the walk fixture the empirical squared coefficient of variation
      over 2,000 seeded splitting runs is within a factor of 2 of the formula (the tolerance set here).
- [ ] KA-141: over 10⁴ seeded repetitions on the walk, the upper bound and the truncated-weight lower bound
      bracket 1.504 × 10⁻⁴ in at least 95% of repetitions (δ = 0.05).
- [ ] A fork whose children share the parent's streams is detected (identical digests) and refused; a bound
      offered a tilted episode without recorded weights is refused.
- [ ] `scripts/e2e/spec_c27.sh` exits 0.

## Test plan

### Unit tests

- `s2_zero_failure_arithmetic`: the KA-138 counts; `s2_bound_chain`: for n ∈ {10, 10³, 10⁵} and δ = 0.05,
  1 − δ^(1/n) ≤ ln(1/δ)/n.
- `s2_coverage`: 10⁴ synthetic studies with known p = 10⁻³ and n chosen so failures occur: the sequential
  bound covers p in at least 95%.
- `s3_unbiased_by_enumeration`: a nominal law on five outcomes with failure set of mass 1/100 and a proposal
  that triples the failure mass: the weighted estimator's expectation is exactly 1/100 in ℚ.
- `s4_truncated_is_lower`: truncation at c = 2 gives expectation at most 1/100, exactly computed.
- `s5_mixture_weight_bound`: every weight at most 1/β; `s5_mixture_certifies_less`: the KA-139 value.
- `s5_gaussian_tail`: shift 4, β = 0.1: the interval contains 3.167 × 10⁻⁵ and the largest weight is at most 10.
- `s6_walk_closed_form`: 0.5/(1.5²⁰ − 1) to 12 significant digits; `s6_cv_formula`: 0.0054.
- `s7_fork_without_fresh_randomness_refused`, `s7_children_have_own_streams`: children keyed by child
  addresses produce pairwise different continuations.
- `s8_good_turing`: mode counts (5, 3, 1, 1, 1): unseen mass 3/11.
- `s1_lanes_do_not_mix`: a bound offered episodes flagged as exploration returns the refusal.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c27.sh` (idempotent; seed 2700 printed): `frob --json doctor`; the law suite with and without
`--features controls`; `cargo test -p frob-conformance --test g1 -- KA-138 KA-139 KA-140`;
`cargo test -p frob-conformance --test g0 -- KA-141`; `cargo test -p frob-conformance --test g3 -- MR-08` on
the host site. Sources the STD-E2E helper; writes `target/e2e-logs/spec_c27/<utc>.ndjson` and a transcript;
each line has `ts`, `script`, `bead` (`SPEC-C-27`), `step`, `event` (`start`, `cmd`, `assert`, `metric`,
`refusal`, `end`), command line, exit code, wall time, evidence block (claim Known answer, class, tier G0 or
G1 host, determinism class, seed), each count or coverage against its expected value and threshold, `refusal`
lines for the expected FROB-REF-560 and FROB-REF-569, and on failure the first repetition whose bounds miss
the truth with both bounds. Final `summary`: assertion counts, GATE-GAUNT-02 and GATE-GAUNT-03.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-138 | p ≤ 1 − δ^(1/n) ≤ ln(1/δ)/n; δ = 0.05 gives about 3·10⁴ episodes for 10⁻⁴ (29,956 exactly) | exact (arithmetic) | G1 · host | Registered here (the appendix states none): an episode count from ln(1/δ)/p* rounded down, used as if it were sufficient for the looser bound |
| KA-139 | Weights at most 1/β; the certified bound is only p ≤ ln(1/δ)/(β·n) | exact | G1 · host | CX-065: a defensive mixture taken to make certifying absence cheaper; registered mutant: a mixture without its nominal share on a mis-centred proposal (the weight bound fails) |
| KA-140 | Squared coefficient of variation near L·(1 − p₀)/(p₀·N); about 7% for L = 6, p₀ = 0.1, N = 10⁴ | formula exact; empirical value within a factor of 2 (set here; the appendix says "about") | G1 · host | Registered here: the formula with p₀ in place of 1 − p₀ in the numerator |
| KA-141 | The crude and bounded-weight bounds bracket the truth in at least 1 − δ of 10⁴ repetitions | coverage at least 1 − δ | G0 · host | CX-066 and CX-292: children that share the parent's streams are detected as duplicates and the splitting estimate is refused |

Relation MR-08 (fork and replay): the empty intervention replays bit for bit, and a child with its own address
has fresh streams; its controls are a checkpoint with its packet queue stripped and a fork that shares
streams. Further control: a Good–Turing monitor that counts distinct modes instead of modes seen once.
Determinism: streams are keyed, so every repetition is reproducible from its key; twice on one machine and in
debug and release with identical tables.

## Considerations

- The cost of a claim is fixed by arithmetic; no sampler discounts it. If throughput is a tenth of the
  illustration, a 10⁻⁶ claim costs 21 hours: certify 10⁻⁵ and report 10⁻⁶ as `Estimated`. Whether to buy a
  10⁻⁶ certificate is an open compute question with that fail-closed default.
- The primary risk is exploration data leaking into a bound. Episodes carry their proposal law, and the bound
  refuses any episode not drawn nominally or with recorded bounded weights.
- The probability is under the task's nominal randomization, on the stated tier, for the frozen requirement;
  it is a statement about the model at that tier and never about a physical robot.
- Splitting applies only to failures driven by per-step disturbances; failures driven by static parameters
  alone gain nothing from forks.

## References

Plan §10 C-27; §9 D-16; §11.1 (world address); §26.9, §26.12, §26.14; §38.7 (MR-08); §38.12; Appendix D.8
(zero-failure bound, bounded-weight importance sampling, splitting variance, unseen-mode mass); Appendix E
GB-32; KA-138 to KA-141; CX-065, CX-066, CX-292; BET-32; GATE-NEMESIS-04, -05; KILL-RARE; FROB-REF-560,
FROB-REF-569.
