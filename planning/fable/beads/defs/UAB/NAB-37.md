---
key: "NAB-37"
title: "killed-idea register KI-33 to KI-46: search, statistics and decision rules (reuse, paving, rare events)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "atelier", "nemesis", "augur", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-32"]
needs: ["§28.19: the decision-core tables DC-01 to DC-11 with their fixed-table fallbacks, against which the nine rejected methods of KI-45 were weighed (BD-309, BD-349)", "§24.12: the reachability pre-analysis of a fixed-n study (`eval.preanalyze`), which owns the enumeration of KI-44 (BD-124)", "§0.8: hypothesis rows H-03, H-06 to H-10, H-13, H-22a, H-22b and H-23 (N00-11, N00-12)"]
related: ["NAB-36", "NAB-38", "NAB-20", "NAB-26", "NAB-27", "NAB-15", "N28c-45", "BD-124", "BD-186", "BD-257", "BD-264", "BD-276", "BD-277", "BD-280", "BD-289", "BD-292", "BD-310", "BD-328", "BD-351", "BD-353", "BD-354"]
gates: []
plan_refs: ["Appendix C.2.5", "§0.8 (H-03, H-06, H-07, H-08, H-09, H-10, H-13, H-22a, H-22b, H-23)", "§28.19.4"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries the fifth and largest group, fourteen entries about **search, statistics and decision
rules**. A design studio makes the same few decisions thousands of times: which designs to keep, when a
comparison is settled, whether a stored verdict still holds after an edit, how rare a failure is, what caused
it. Each has a tempting shortcut that claims more than its evidence supports: reuse a rollout after a geometry
edit; inherit a verdict because a sensitivity looks small; pave a twenty-dimensional design box; certify a
10⁻⁵ failure rate from a splitting run; name a single top cause from one fork; choose experiments by
information gain; assume that a cheap rung ranks designs as an expensive one does. The register records what
each of these can and cannot do, with the published figures and the arithmetic.

Reading conventions: a number inside a *strong form* is the proposal's own; FACT figures have source-register
records and are re-read or re-run before a gate cites them; "DESIGN arithmetic" can be redone by hand; ranges
in *survives* lines are HYPOTHESES until the named gate records a value; thresholds are TARGETs. Tier of the
bead: Solid (a decision record).

## Scope

In: records `KI-33` to `KI-46` with calculations, keywords, carriers and revival conditions; the nine
sub-records of KI-45. Out: the schema, harness and lint (NAB-32); the surviving mechanisms (their carriers).

## Technical approach

**Transcription rule.** As in NAB-33. KI-45 is a table of nine methods; it becomes one record with nine
`[[idea.method]]` sub-tables (`method`, `core`, `why_not`, `used_instead`, `keywords`), so that the revive lint
matches each method separately and the expected-value matrix's rejected rows can point to the method.

**Machine-checked arithmetic of this group.**

| Record | Expression | Value |
|---|---|---|
| KI-33 | share of per-design compute that is rollouts and training (no rollout is reusable bit for bit after a mass or geometry edit) | at least 90%, so exact reuse is at most about 10%, below the hypothesis's own falsifier of 30% |
| KI-35 | bisections bought by 10⁴ boxes: log₂ 10⁴ | 13.3, less than one cut per axis in 20 dimensions |
| KI-35 | side kept by uniform boxes in 20 and in 10 dimensions: 2^(−13.3/20), 2^(−13.3/10) | 0.63; 0.40 |
| KI-36 | nominal episodes for a zero-failure 95% bound at 10⁻⁵: ln(20)/10⁻⁵ | 3.0 × 10⁵ |
| KI-37 | the same at 10⁻⁶ (also the cost of checking a fitted tail there): ln(20)/10⁻⁶ | 3.0 × 10⁶ |
| KI-37 | floor of a defensive mixture with nominal share β after n draws at confidence 1 − δ | ln(1/δ)/(β·n) |
| KI-40 | control-variate saving 1/(1 − ρ²) at ρ = 0.7, 0.9, 0.95 | 2.0; 5.3; 10.3 |
| KI-40 | correlation needed for a fifth of the expensive samples: ρ = √(1 − 1/5) | 0.894 |
| KI-42 | padding an 18-degree-of-freedom quadruped to 64: linear, quadratic and cubic stages: 64/18, (64/18)², (64/18)³ | 3.6; 12.6; 45 |
| KI-42 | 256 designs × 2,048 rollouts × about 60 kB per padded world | 31 GB, above a 24 GB card |
| KI-42 | the published mixed batch against the harmonic mean of its homogeneous rates: about 86 k / 100 k samples per second | 0.86 |
| KI-44 | count pairs reachable at n = 64: 66·65/2; split into promote, inferior, inconclusive, practical tie | 2,145 = 521 + 521 + 1,103 + 0 |
| KI-44 | smallest n at which a practical tie is reachable | 86 |

FACT figures and their sources: the executed pilots of KI-34 and KI-44 are plan-time checks (the decision-reuse
tie; the fixed-n enumeration; the timing ceiling near 1.46) with records in the source register, re-run before
a gate cites them; the published figures of KI-36 (gains of 2 to 20 times in one driving study and about 5
times in mean-square error on one policy-in-the-loop task), KI-38 (a perturbation of 5 × 10⁻⁴ degree diverges
within 50 steps), KI-39 (stopping and elimination save 2 to 5 when candidates are separated), KI-40 (rank
correlations of 0.40 overall and 0.4 to 0.8), KI-41 (population methods gain about 3 to 4) and KI-42 are
public sources to re-read.

**The thresholds that the surviving forms carry** (TARGETs; the gates named in each entry own them): a cheap
rung *selects* at Kendall τ ≥ 0.7 with the winner check passed, *discards only* between 0.5 and 0.7, and may do
neither below 0.5 or with no record; bounded-weight importance sampling refutes a target and tightens an
estimate once failures are observed and never certifies absence more cheaply than crude sampling; a paired
comparison's ceiling 1/(1 − f + f/r + h) is recorded before any speed comparison (f the fraction of work the
change touches, r its speed-up, h the added overhead as a fraction of the whole), and a comparison whose
ceiling is below its claim is refused.

**Keywords for the revive lint.** KI-33: reuse rollouts after edit, incremental rollout reuse. KI-34:
sensitivity-margin reuse, inherit verdict by sensitivity. KI-35: pave the design box, joint interval paving,
Taylor-model paving of the design space. KI-36: splitting confidence bound, certify rare failure by splitting.
KI-37: extreme-value fit of the failure tail, defensive mixture as certificate. KI-38: top cause from a single
fork, root cause by pre-failure snapshot. KI-39: expected information gain selection. KI-40: assume rank
agreement, shared-policy ranking as ground truth. KI-41: long-horizon finite-difference design gradient.
KI-42: topology as data as a first, mixed-topology capability claim. KI-43: requirements replace rewards,
robustness as the only training signal. KI-44: fixed-n rule without pre-analysis, speed test as evidence
against. KI-45: one keyword list per method (bandit allocation, UCB, Thompson; expected information gain;
Gaussian-process surrogate over designs; kernel two-sample, optimal-transport drift; mechanism design for the
queue; consensus, gossip, escrow counters; learned tier correction, learned step surrogate; adaptive-fidelity
stopping as the decision; Shapley as primary output). KI-46: unconstrained adversary, zero-sum world
adversary.

**The entries.**

**KI-33. Exact reuse after a geometry edit** (H-09, REFUTED for exact reuse)
- *Strong form.* Dependency tracking lets a typical single-parameter edit reuse at least 70% of prior evaluation
  work.
- *Refutation.* At least 90% of per-design compute is rollouts and training (FACT: training takes 4 to 20
  minutes per design; a cold kernel compile is about 96 s once per machine and under 1 s cached). The mass
  matrix, bias forces and contact Jacobians enter every step, so no rollout is reusable bit for bit after a mass
  or geometry edit. Exact reuse weighted by compute is at most about 10%, below the hypothesis's own falsifier
  of 30%.
- *Survives.* A memo graph for analyses, step plans, kernels and scene compiles (§28.14). Dependency-directed
  revalidation, which says which verdicts survive an edit (§28.13; `result.revalidate`; GATE-ATEL-11;
  KILL-REUSE; BET-33). Training avoided by a warm start is a different quantity: it belongs to inheritance (H-21)
  and is counted once, in its own column of the reuse ledger (§28.26).
- *Revive only if.* A measured exact-reuse share above 30% of compute in the exact-reuse column of the reuse ledger (GATE-ATEL-19).
- *Carried by.* BD-111, BD-292, N28c-45.

**KI-34. Verdict reuse by sensitivity margin**
- *Strong form.* Inherit a verdict across an edit when a sensitivity estimate says the metric moved less than the
  decision margin, for savings beyond a cache.
- *Refutation.* FACT (executed pilot): a correctly keyed dependency graph matched the semantic rule saving for
  saving, three pair evaluations and two reductions each, and omitting one dependency returned a stale promotion.
  No certificate bounds how an edit moves a rollout statistic through contact.
- *Survives.* Reuse keyed by dependency, with polarity where a proof rule supplies it (C-18; §25.8). One false
  reuse removes the rule (KILL-AUGUR-REUSE).
- *Revive only if.* A certificate that bounds how an edit moves a rollout statistic through contact. None exists; one false reuse removes any implication rule (KILL-AUGUR-REUSE).
- *Carried by.* BD-292, BD-347.

**KI-35. Joint paving of high-dimensional design boxes** (H-06, NARROWED)
- *Strong form.* Interval and Taylor-model evaluation decides at least half the volume of a 10- to
  20-dimensional design box without simulation, within 10⁴ boxes.
- *Refutation.* DESIGN arithmetic: 10⁴ boxes are log₂ 10⁴ = 13.3 bisections, less than one cut per axis in 20
  dimensions. Uniform boxes keep a side of 2^(−13.3/20) = 0.63 of the domain, and 0.40 in 10 dimensions, so a
  constraint surface that depends on most parameters crosses nearly every box. FACT: a published
  three-dimensional workspace paving ran about five hours and left an error bound of up to a third of the
  certified volume.
- *Survives.* Certificates per constraint, with bisection confined to a split set of about six parameters and the
  rest enclosed at full range, combined by products, monotonicity and witnesses (C-32; §25.6, §25.34;
  KILL-AUGUR-REGION; BET-20).
- *Revive only if.* The weak-variable enclosure measurement (GATE-AUGUR-30, KILL-WEAKVAR) and the region pilot on the 12-parameter box.
- *Carried by.* BD-289, BD-358, N25e-23 to N25e-26.

**KI-36. A rare-event engine with bounds from splitting** (H-07, NARROWED)
- *Strong form.* Multilevel splitting or adaptive importance sampling estimates a failure probability of 10⁻⁵
  with a hundredth of the episodes and valid confidence bounds.
- *Refutation.* DESIGN arithmetic: a zero-failure 95% bound at 10⁻⁵ needs ln(20)/10⁻⁵ ≈ 3.0 × 10⁵ nominal
  episodes. FACT: the measured gains in the sources read are 2 to 20 times in one driving study and about 5 times in
  mean-square error on one policy-in-the-loop task; 10 to 100 times at 10⁻⁴ to 10⁻⁵ is a TARGET. A splitting
  estimate with adaptive levels, though unbiased, carries no finite-sample confidence bound.
- *Survives.* Crude sampling certifies absence. Bounded-weight importance sampling refutes a target and tightens an
  estimate once failures are observed, and never certifies absence more cheaply than crude sampling. Splitting and
  tilting explore (C-27; §26.9; KILL-RARE; BET-32).
- *Revive only if.* A finite-sample confidence bound for splitting with adaptive levels, or the exploration pilot at 10 times or more at 10⁻⁵ with interval coverage of at least 85% (KILL-RARE).
- *Carried by.* BD-276, BD-277, N26a-28, N26a-32.

**KI-37. Extreme-value extrapolation, and a defensive mixture as the certified bound**
- *Strong form.* Certify a 10⁻⁶ tail by fitting an extreme-value law, or by importance sampling from a defensive
  mixture.
- *Refutation.* DESIGN arithmetic: checking a fitted tail at 10⁻⁶ takes the same ln(20)/10⁻⁶ ≈ 3 × 10⁶ nominal
  episodes that certify directly. A defensive mixture with nominal share β has a floor of ln(1/δ)/(β·n), never
  below crude sampling on that share.
- *Survives.* Crude and bounded-weight bounds. The defensive mixture as a lower bound for refutation only
  (DC-04).
- *Revive only if.* None. Checking the fitted tail costs what direct certification costs, and the mixture's floor is ln(1/δ)/(β·n).
- *Carried by.* BD-276.

**KI-38. A single top cause from forks taken just before the failure** (H-08, NARROWED)
- *Strong form.* Restoring a snapshot just before a failure and applying single-factor interventions, design
  parameters included, names the cause with at least 90% accuracy.
- *Refutation.* Contact dynamics amplify small differences (FACT: a published perturbation of 5 × 10⁻⁴ degree
  diverges within 50 steps), so a top factor can change under a perturbation at round-off scale. A design
  parameter changed a moment before failure leaves the history unchanged (winding temperature, the policy's own
  training), which is a different question from whether that design would have failed. A coalition value can be
  negative and cannot establish necessity.
- *Survives.* Interventional contrasts with a flip rate and a placebo fork. Design-parameter interventions re-run
  from the episode start. Double precision is the default for one failure; single precision on the originating
  tier is admitted by a measured rule for fork windows up to about 1 s (§27.5 to §27.9, §27.19; GATE-AUTOPSY-03;
  KILL-AUTOPSY; BET-23).
- *Revive only if.* GATE-AUTOPSY-03: planted-cause accuracy of at least 90% with the top factor changing in at most 30% of cases at 10⁻⁷.
- *Carried by.* BD-279, BD-280, BD-283, N27b-11.

**KI-39. Value of information as a selection lever** (H-10, NARROWED)
- *Strong form.* Choosing each experiment by expected information gain cuts the simulations to a decision by at
  least three, against experiments an agent chooses.
- *Refutation.* FACT (best-arm identification results): stopping and elimination save 2 to 5 when candidates are
  separated and about nothing otherwise. Selection by information gain against a capable agent is unsupported
  and plausibly below 1.5.
- *Survives.* Value of information as a stop rule and a budget allocator at the finalist stage, and for choosing
  measurements (C-39; §28.16; GATE-ATEL-15; KILL-VOI).
- *Revive only if.* An A/B against capable agents that shows at least 1.5 times fewer simulations to a decision. The stop-rule form is decided at GATE-ATEL-15.
- *Carried by.* BD-351, N28b-21.

**KI-40. Rank fidelity assumed between rungs** (H-22a and H-22b, both ACTIVE and unmeasured)
- *Strong form.* A shared policy on the fast tier ranks designs as trained specialists on the referee do, at
  Kendall τ ≥ 0.7 near the top, so under one design in twenty needs training; and paired evaluation needs at most
  a fifth of the expensive samples.
- *Refutation.* FACT: no published rank agreement of this kind was found; the nearest figures are rank
  correlations of 0.40 overall and 0.4 to 0.8. DESIGN arithmetic: a control variate saves 1/(1 − ρ²), which is
  2.0 at ρ = 0.7, 5.3 at 0.9 and 10.3 at 0.95, so a fifth of the samples needs ρ ≥ 0.89.
- *Survives.* The rank-agreement experiment decides, and it reports containment and regret beside τ. A cheap rung
  selects at τ ≥ 0.7 with the winner check passed, discards only between 0.5 and 0.7, and may do neither below 0.5
  or with no record (C-26; §16.9, §23.28, §28.10; GATE-ATEL-07; KILL-RANK; BET-17).
- *Revive only if.* The rank-agreement experiment (GATE-ATEL-07, KILL-RANK) and the measured tier correlation (KILL-DR). The assumption is never revived as an assumption.
- *Carried by.* BD-145, BD-264, BD-310, BD-328.

**KI-41. A family per dispatch, with long-horizon finite-difference gradients** (H-23, NARROWED)
- *Strong form.* Evaluating a neighbourhood of designs in one batch under a shared policy cuts agent rounds by at
  least five, and paired finite differences give usable design gradients over whole episodes.
- *Refutation.* FACT: measured compute gains of published population methods are about 3 to 4. Paired
  differences lose their pairing once trajectories diverge through contact (KI-38), so the gradient is
  unproven beyond short horizons and smooth metrics.
- *Survives.* Families per dispatch, and short-horizon paired differences as a sensitivity report (§17.12,
  §17.13; KILL-FDGRAD).
- *Revive only if.* The paired finite-difference pilot (KILL-FDGRAD) and the measured rounds saved per dispatch (GATE-PHYLUM-08).
- *Carried by.* BD-342, BD-353, N17b-20.

**KI-42. Topology as data, as a capability claim** (H-03, NARROWED)
- *Strong form.* One compiled kernel simulates different trees in one batch at no more than twice the per-world
  cost of a specialized kernel, as a capability no incumbent has.
- *Refutation.* FACT: one incumbent solver publishes a mixed batch of three body plans at 0.86 of the harmonic mean
  of its homogeneous rates (about 86 thousand samples per second against about 100 thousand). DESIGN arithmetic:
  one batch padded to 64 degrees of freedom costs an 18-degree quadruped 64/18 = 3.6 on linear stages, 12.6 on
  quadratic ones and 45 on a dense cubic factor; and 256 designs × 2,048 rollouts at about 60 kB per padded
  world is 31 GB, above a 24 GB card.
- *Survives.* Per-world model variants inside padded capacities and size buckets, claimed only as two measured
  numbers and one property (run-to-run replay and forks inside the mixed batch) from a head-to-head with a
  per-family-template control arm (§17.5, §17.14; GATE-PHYLUM-03; KILL-PHYLUM; BET-18).
- *Revive only if.* The four-arm head-to-head (GATE-PHYLUM-03, KILL-PHYLUM). The capability wording is not revived: an incumbent documents the capability.
- *Carried by.* BD-185, BD-186, N17b-10.

**KI-43. Requirements replace rewards** (H-13, NARROWED)
- *Strong form.* Temporal-logic requirements with a smoothed robustness score serve as the training signal with
  no loss, and cut reward exploitation to 0.1% or less.
- *Refutation.* FACT: the published successes need subgoal decomposition or added terms, and the 0.1% figure was
  unsupported and withdrawn. A minimum over predicates in mixed units is not a margin until each is normalized.
- *Survives.* Bounded-time requirements over unit-normalized predicates as the acceptance metric and the
  falsification score; training keeps reward terms (C-30; §24.5, §24.6; KILL-SPEC-REWARD; BET-24).
- *Revive only if.* The three-arm reward-term pilot (KILL-SPEC-REWARD); margin-only training only if it reaches 80% of the shaped arm's success.
- *Carried by.* BD-257, N24a-14, N24a-15.

**KI-44. A fixed-n rule with an unreachable verdict, and a speed test with no power**
- *Strong form.* A frozen four-verdict comparison rule at 64 pairs; a speed test read as evidence against a
  ten-fold.
- *Refutation.* FACT (executed enumeration): of the 2,145 count pairs reachable at n = 64, 521 promote, 521 are
  inferior, 1,103 are inconclusive and none is a practical tie; the first tie appears at n = 86. FACT (executed
  pilot): shared overhead bounded the speed test near 1.46, so a failure to show ten was guaranteed.
- *Survives.* A pre-analysis of which verdicts are reachable at the declared n (`eval.preanalyze`; §24.12;
  GATE-ARENA-05, GATE-ATEL-06). The ceiling 1/(1 − f + f/r + h) is recorded before any speed comparison, and a
  comparison whose ceiling is below its claim is refused (GATE-AUGUR-14).
- *Revive only if.* None. Both rules stay: enumerate the reachable verdicts before sampling, and compute the ceiling before a speed test.
- *Carried by.* BD-124, BD-354.

**KI-45. Methods considered for one decision core and rejected.** Each was weighed against a named core of §28.19 and
is not built; §28.19.4 holds the full list.

| Method | Core | Why it is not built | Used instead |
|---|---|---|---|
| Bandit allocation (upper confidence bounds, posterior sampling) | DC-02, DC-09 | Regret is the wrong objective for identifying the best design; on dispatch it is a second controller on shared telemetry | The fixed halving table, and the conformal ladder where rank agreement licenses it |
| Expected information gain | DC-05 | It buys knowledge that cannot change the choice | Value of information per cost, as a stop rule |
| Gaussian-process surrogates over designs | DC-02, DC-05 | No validated kernel exists on typed designs of varying topology | `fs-bo` on continuous sub-problems only |
| Kernel two-sample and optimal-transport drift tests | DC-07 | No power at twenty scalar residuals | A conformal band with an exchangeability alarm |
| Mechanism design for the queue | DC-09 | Estimates come from the system, not from agents | Size classes with preemption and resume |
| Replicated consensus, gossip membership, escrow counters | DC-09 | One daemon per host; nothing is replicated | A single-writer job ledger |
| A learned correction of the fast tier, or a learned step surrogate as a screen | DC-01, DC-02 | It hides the artifacts the trust ladder exists to expose | Paired tier correction on referee episodes that were run |
| Adaptive-fidelity stopping as the decision | DC-01, DC-02 | Stopping a pair early on cheap labels does not compute the sealed fixed-n decision | Kept as a scheduling heuristic that orders work |
| Coalition-value attribution as the primary output | DC-06 | Values can be negative and cannot establish necessity | Interventional contrasts, with a budgeted summary |

*Revive only if.* Per method, the negation of its reason: a validated kernel on typed designs of varying topology (surrogates); residual counts far above twenty (two-sample tests); more than one daemon per host (consensus); estimates supplied by agents (mechanism design). Regret-based allocation and expected information gain are not revived as decision rules.

*Carried by.* BD-282, BD-296, BD-309, BD-349.

**KI-46. Unconstrained zero-sum adversarial worlds**
- *Strong form.* Search or train an adversary over worlds with no constraint beyond the robot's failure.
- *Refutation.* The adversary wins with worlds no robot could solve, which says nothing about the design.
- *Survives.* Search inside a legality set, with a solvability oracle and a witness for every reported world
  (§26.3, §26.5; GATE-NEMESIS-02).
- *Revive only if.* None. Every reported world carries a solvability witness.
- *Carried by.* BD-270, BD-271.

## Acceptance criteria

- [ ] Records `KI-33` … `KI-46` exist with the paragraphs verbatim, revival conditions, carriers and keywords;
      KI-45 has nine method sub-records.
- [ ] The harness recomputes the thirteen calculation rows; the enumeration of KI-44 is reproduced by a test
      that enumerates count pairs.
- [ ] The revive lint fails a planted bead "UCB bandit for ladder allocation" with
      `ki.match-without-citation` naming KI-45 (method 1), and passes the bead for the fixed halving table,
      which cites it.
- [ ] Each rejected method of KI-45 names what is used instead, and that replacement exists as a bead.
- [ ] The two hypotheses that are ACTIVE (rank fidelity between rungs; the fast tier as a paired control) are
      recorded in KI-40 as assumptions that were made, with hypothesis state ACTIVE, and the record's `state` is
      `assumed`, not `refuted`.

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g5_search`:

- `ki35_paving`: log₂ 10⁴ = 13.288; 2^(−13.288/20) = 0.631; 2^(−13.288/10) = 0.398.
- `ki36_ki37_episodes`: ln 20 = 2.9957; ÷ 10⁻⁵ = 299,573; ÷ 10⁻⁶ = 2,995,732.
- `ki37_mixture_floor`: at β = 0.1, δ = 0.05, n = 10⁶ the floor is 2.9957/(0.1 × 10⁶) = 2.996 × 10⁻⁵, ten times
  the crude-sampling floor on all n draws.
- `ki40_control_variate`: 1/(1 − 0.49) = 1.961; 1/(1 − 0.81) = 5.263; 1/(1 − 0.9025) = 10.256; √0.8 = 0.8944.
- `ki42_padding`: 64/18 = 3.556; squared 12.64; cubed 44.95; 256 × 2,048 × 60,000 B = 31.46 GB.
- `ki44_enumeration`: the number of pairs (a, b) of non-negative integers with a + b ≤ 64 is 2,145; under the
  paired rule with a five-point margin the verdict counts are 521, 521, 1,103 and 0 (the rule's definition is
  read from the study bead's fixture; this test asserts the totals and that the first n with a tie is 86).
- `ki45_has_nine_methods`.
- `records_complete_and_verbatim` against `tests/fixtures/killed/g5.md`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) recomputes the calculations and runs the planted revive cases.
Logged per STD-E2E.

### Known answers and negative controls

Known answers: the table. Negative controls: a splitting estimate reported with a "95% confidence bound" (the
carrier's refusal; this bead supplies the keyword match); a verdict inherited across a mass edit by a
sensitivity estimate (the standing kill gate on implication reuse fires on one such case); a rung licensed to
select at τ = 0.6; a count of 2,146 pairs; a KI-45 record with eight methods. Metamorphic check for
`ki40_control_variate`: the saving is increasing in ρ and equals 1 at ρ = 0.

## Considerations

- KI-40 is the entry the program can least afford to get wrong. Its measurement, the rank-agreement experiment,
  is pre-registered at M1, piloted at M3 and decided at M4; the entry stays as a record of what was assumed
  even after the measurement.
- KI-45's reasons are specific to this system (one daemon per host; estimates come from the system; twenty
  scalar residuals). A change in those facts is the revival evidence, per method.
- KI-42 is about wording: mixed-topology batching is built, and is claimed only as two measured numbers and
  one property from the four-arm head-to-head.

## References

Appendix C.2.5; §0.8 rows H-03, H-06 to H-10, H-13, H-22a, H-22b, H-23; §28.19.4; constructions C-18, C-25,
C-26, C-27, C-30, C-32.
