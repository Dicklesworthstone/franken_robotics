---
key: "NAB-36"
title: "killed-idea register KI-26 to KI-32: control, learning, transfer (sampling evaluator, universal policy)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "reflex", "cortex", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-32"]
needs: []
related: ["NAB-37", "NAB-47"]
gates: []
plan_refs: ["Appendix C.2.4", "§0.8 (H-01, H-02, H-05, H-20, H-21, H-24)", "§22.8", "§22.12", "§23.11", "§23.12"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries the fourth group, seven entries about **control, learning and transfer**. The purpose of the
whole system is to shorten the loop in which an agent designs a robot, and the largest term of that loop is
getting a controller for each candidate design. Every idea in this group promises to remove that term: plan
with the simulator instead of training; pretrain one policy for every body; inherit a child's policy from its
parent; find gaits by continuation instead of learning; certify safety instead of testing it. Each survives in
a narrowed form that is useful and honest, and each strong form is recorded here with the published figures
or the arithmetic that narrowed it.

Reading conventions: a number inside a *strong form* is the proposal's own; FACT figures (here mostly
published results) have source-register records and are re-read from the primary source before a gate cites
them; "DESIGN arithmetic" can be redone by hand; ranges in *survives* lines are HYPOTHESES until the named
gate records a value; thresholds are TARGETs. Tier of the bead: Solid (a decision record).

## Scope

In: records `KI-26` to `KI-32` with calculations, keywords, carriers and revival conditions. Out: the schema,
harness and lint (NAB-32); the surviving mechanisms (their carriers).

## Technical approach

**Transcription rule.** As in NAB-33.

**Machine-checked arithmetic of this group.**

| Record | Expression | Value |
|---|---|---|
| KI-26 | model steps per simulated second of the smallest published sampling setting: 30 samples × 3 iterations × 450 steps (0.9 s at 2 ms) × 50 Hz | 2.025 × 10⁶ |
| KI-26 | ratio to a policy rollout at 500 steps per second | 4,050, about 4 × 10³ |
| KI-26 | a battery of 256 twenty-second episodes at 2.0 M world-steps/s: sampling control; a policy | 86.4 minutes; 1.28 s |
| KI-26 | clean episodes to bound a failure rate at 1% with 95% confidence: ⌈ln 0.05 / ln 0.99⌉ | 299 |
| KI-26 | 299 episodes at 0.33 GPU-minute each | about 99 GPU-minutes |
| KI-26 | per-episode cost at which 299 acceptance episodes cost one training run (under 10 and 60 GPU-minutes): 600 s ÷ 299; 3,600 s ÷ 299 | 2.0; 12.0 GPU-seconds |
| KI-27 | held-out against familiar return: 1,489.6 / 4,030.6 | 37% |
| KI-27 | no-adaptation against from-scratch on one test morphology: 136.82 / 1,406.58 | 9.7% |
| KI-27 | 5 days on 160 GPUs; the same on two cards | 19,200 GPU-hours; 400 days |
| KI-30 | orbit solves of 4 to 260 s as a share of a four-minute training run; of a 410 s run | 1.7% to 108%; 1.0% to 63% |
| KI-32 | closed-form projection of u_π = (1, 0) onto the row u₁ + u₂ ≥ 1.5 | (1.25, 0.25), outside the box [0, 1]² |
| KI-32 | the same after clipping to the box | (1, 0.25), which violates the row (1.25 < 1.5) |
| KI-32 | the true optimum of min ‖u − u_π‖² subject to the row and the box | (1, 0.5) |

**The three shield counterexamples of KI-32, in full** (permanent negative controls of the shield):

1. *The clairvoyant barrier condition.* Robust invariance stated as "for every disturbance w there is an input
   u" lets the input depend on a disturbance it cannot know. If safety needs u = d with d ∈ {−1, +1}, each
   pointwise problem is feasible and no single input serves both. The correct order is ∃u ∀(w, e): one control
   for all disturbances and estimation errors in the belief set.
2. *The smallest-cone rule.* An infimum over a smaller set is larger, so using the friction cone at μ_min for
   every force is optimistic for forces the environment applies. The cones are split: the inner cone at μ_min
   for forces the robot commands, the outer cone for forces the environment may apply.
3. *The closed-form projection.* With u_π = (1, 0), the box [0, 1]² and the row u₁ + u₂ ≥ 1.5, projecting onto
   the half-space alone gives u_π + ((1.5 − 1)/2)·(1, 1) = (1.25, 0.25); clipping gives (1, 0.25); the optimum
   is (1, 0.5). The surviving method is the breakpoint (sort-and-clip) method for one row, and projected Newton
   with a KKT check after the fact otherwise.

**Cost figures that the surviving forms rest on** (HYPOTHESES from published settings, to be measured at the
sampling controller's gate): about 0.33 and 1 GPU-minutes per 20 s episode for a quadruped and a humanoid at a
20 ms planning step, and about 1.7 and 5.2 at 4 ms; a paired screening comparison may use up to the break-even
count against training once, about 30 episodes for the quadruped and 60 for the humanoid at a 20 ms planning
step; the default screening rung is three episodes per design.

**Keywords for the revive lint.** KI-26: MPC as evaluator, sampling control as acceptance evidence, no-training
evaluator. KI-27: universal policy, one policy for all morphologies, foundation policy for any robot. KI-28:
inherit policy across topology edit, chained inheritance. KI-29: geometric-algebra tree policy for sample
efficiency, multivector tokens. KI-30: gait continuation yields a controller, orbit continuation instead of
training. KI-31: sum-of-squares funnel for the full robot, deployable safety filter, hybrid zero dynamics
certificate. KI-32: for every disturbance there exists a control, smallest friction cone, closed-form
projection for the filter.

**The entries.**

**KI-26. Sampling control as a stand-in evaluator for a trained policy** (H-01, NARROWED)
- *Strong form.* Sampling-based predictive control, with the batched simulator as its model, completes tasks for
  at least 80% of well-formed designs with no training and serves as the acceptance evaluator.
- *Refutation.* DESIGN arithmetic on the smallest published setting (30 samples, 3 iterations, a 0.9 s horizon
  at 2 ms, 50 Hz): one simulated second costs 30 × 3 × 450 × 50 = 2.0 M world-steps against 500 for a policy, a
  factor of 4 × 10³. A battery of 256 twenty-second episodes takes about 86 minutes of a device at 2.0 M
  world-steps/s, against 1.3 s. As acceptance evidence, bounding a failure rate at 1% with 95% confidence needs
  299 clean episodes: about 99 GPU-minutes per quadruped design at 0.33 GPU-minute each, against under 10 for
  training once.
- *Survives.* A screening rung of three episodes per design by default (a paired screening comparison may use up
  to the break-even count, about 30 for the quadruped setting and 60 for the humanoid at a 20 ms planning step), an
  out-of-distribution fallback and a source of demonstrations (§22.8; GATE-REFLEX-06; KILL-MPC; BET-27). The cost is
  about 0.33 and 1 GPU-minutes per 20 s episode at a 20 ms planning step and about 1.7 and 5.2 at 4 ms. A request
  to cite it as acceptance evidence for a trained policy is refused (`FROB-REF-817`); where it is the delivered
  controller, its own episodes are admitted under a frozen planner record.
- *Revive only if.* A measured cost per 20 s episode at or below the break-even against training once (2.0 GPU-seconds for a quadruped, 12 for a humanoid: training once divided by the 299 clean episodes acceptance needs), together with GATE-REFLEX-06. Otherwise only the delivered-controller lane admits its own episodes.
- *Carried by.* BD-252, BD-331, N22c-30 to N22c-34.

**KI-27. One universal policy** (H-02, NARROWED)
- *Strong form.* One pretrained, design-conditioned policy walks any unseen design with no adaptation at 70% of
  specialist return, and reaches specialist return in 10% of from-scratch steps.
- *Refutation.* FACT (published results): across topology-diverse designs, held-out robots score 1,489.6 against
  4,030.6 on familiar ones, 37%; one test morphology scores 136.82 with no adaptation against 1,406.58 from
  scratch, about 10%; fine-tuning gains two to three times. The results that reach 85 to 97% used about 5 days on
  160 GPUs, 19,200 GPU-hours, which is 400 days on two RTX 4090.
- *Survives.* Family policies per morphology class, on flat or mildly varied ground with fixed actuation: 70 to
  85% of specialist return with no adaptation, and fine-tuning at about 2 to 25% of a from-scratch run stopped at
  matched return. On interior terrain within a class there is no published figure. Out-of-class designs are
  declined (§23.11; GATE-CORTEX-11; KILL-FAMILY; BET-15).
- *Revive only if.* An owner-approved compute envelope of the published scale (about 19,200 GPU-hours), or a published result at an affordable pretraining budget. The class-level form is decided at GATE-CORTEX-11.
- *Carried by.* BD-320, BD-321, N23a-26.

**KI-28. Policy inheritance for any edit** (H-21, NARROWED)
- *Strong form.* A child design reaches specialist return from its parent's policy in at most 20% of
  from-scratch steps for parameter edits and 50% for single-limb topology edits, along chains of any length.
- *Refutation.* FACT (published transfer results): single-limb topology edits cost 33 to 100% of from-scratch
  steps and show no gain on flat terrain, and chained inheritance loses plasticity.
- *Survives.* Inheritance for parameter edits, as the default once it has beaten an early-stopped from-scratch
  control for the edit class. Transfer from a class prior costs about 2 to 25% of such a run; the one published
  parent-to-child figure is about 50%; no source measures a small typed parameter edit. Resets follow a measured
  regression of an audit twin, with a depth schedule as fallback (§23.12; GATE-CORTEX-13; KILL-INHERIT; BET-16).
- *Revive only if.* Per edit class, GATE-CORTEX-13 against an early-stopped from-scratch control. Topology edits are revived only by a measured cost below from-scratch.
- *Carried by.* BD-322, N23a-30.

**KI-29. A geometric-algebra tree policy as a source of sample efficiency** (H-24, NARROWED)
- *Strong form.* Link tokens with multivector features and message passing on the kinematic tree beat a plain
  morphology transformer by at least 1.3 in sample efficiency at equal parameters.
- *Refutation.* FACT: geometric-algebra layers ran 1.4 to 7.5 times slower per epoch in published work, so equal
  parameters is not equal compute. No locomotion evidence exists, and the expected gain is 0 to 1.3. A factor of
  1.3 on an amortized pretraining run is not a source of the ten-fold.
- *Survives.* Invariance to heading and to description conventions through canonical inputs and augmentation
  (C-21; §23.11). The tree policy as an [M] three-arm experiment with the plain transformer as control (§23.14;
  KILL-GATREE).
- *Revive only if.* The three-arm experiment at equal compute (GATE-CORTEX-17).
- *Carried by.* BD-439, N23b-02.

**KI-30. Gait continuation at one percent of a training run** (H-05, NARROWED)
- *Strong form.* Continuation of a hybrid periodic orbit carries a stable gait across design changes at no more
  than 1% of a training run per design point, and yields a controller.
- *Refutation.* FACT: published full-order orbit solves take about 4 to 260 s of CPU at 20 to 68 states. DESIGN
  arithmetic: that is 2% to 108% of a four-minute training run, and 1% to 63% of the 410 s figure. The result is
  a nominal orbit on regular ground, not a controller the referee accepts.
- *Survives.* A gait-orbit analysis with continuation in design parameters, labeled screening (§22.9;
  GATE-GAIT-03; KILL-GAIT; BET-26).
- *Revive only if.* The measured cost and rank value of orbits at GATE-GAIT-03.
- *Carried by.* BD-335, N22c-17.

**KI-31. Proof-guided control for full-order legged robots** (H-20, NARROWED)
- *Strong form.* Sum-of-squares funnels, hybrid-zero-dynamics gaits and barrier shields with checked
  certificates yield deployable safety filters for real designs.
- *Refutation.* FACT (sibling code): the semidefinite solver costs O(N⁴) for an N × N Gram block, and its
  largest region-of-attraction example has two states. Certificates exist for templates of about 4 to 12 states,
  and a template proof is not a fact about a robot.
- *Survives.* An energy-budget tier and reduced-order barrier filters whose margin is measured statistically,
  class Tested (§22.11, §22.12; KILL-ENERGY, KILL-SHIELD; BET-25). The word "deployable" is not used, and every
  export carries the no-authorization flags.
- *Revive only if.* A full-order certificate from the sibling solver at tractable cost. None exists above about 12 states.
- *Carried by.* BD-337, BD-338, BD-339, BD-340.

**KI-32. The clairvoyant barrier condition, and two closed forms for shields**
- *Strong form.* (a) Robust invariance from "for every disturbance w there is an input u". (b) The smallest
  friction cone suffices for every force. (c) A closed-form projection for a single-row filter.
- *Refutation.* (a) If safety needs u = d with d ∈ {−1, +1}, each pointwise problem is feasible and no one input
  serves both. (b) An infimum over a smaller set is larger, so the cone at μ_min is optimistic for forces the
  environment applies. (c) With u_π = (1, 0), the box [0, 1]² and the row u₁ + u₂ ≥ 1.5, the formula returns
  (1.25, 0.25), outside the box; clipping gives (1, 0.25), which violates the row; the optimum is (1, 0.5).
- *Survives.* The order ∃u ∀(w, e) over the belief set (C-31). Split cones: the inner cone at μ_min for forces
  the robot commands, the outer cone for forces the environment may apply. The breakpoint method for one row, and
  projected Newton with a KKT check after the fact otherwise (§22.12; GATE-SHIELD-02).
- *Revive only if.* None. The three counterexamples are permanent negative controls of the shield.
- *Carried by.* BD-338, N22b-32, N10c-02.

## Acceptance criteria

- [ ] Records `KI-26` … `KI-32` exist with the three paragraphs verbatim, revival conditions, carriers and
      keywords.
- [ ] The harness recomputes the thirteen calculation rows, and a test solves the small quadratic program of
      KI-32 exactly and obtains (1, 0.5).
- [ ] `FROB-REF-817` exists in the refusal registry with the stated meaning.
- [ ] The three counterexamples of KI-32 are registered as negative controls of the shield.
- [ ] The revive lint fails a planted gate threshold "a universal policy reaches 70% of specialist return on
      any unseen design with no adaptation" with `ki.strong-form-as-goal` naming KI-27, and passes the
      family-policy gate, which is per morphology class.
- [ ] No record, contract or report template of the carriers uses the word "deployable".

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g4_control`:

- `ki26_step_cost`: 30 × 3 × 450 × 50 = 2,025,000; 2,025,000/500 = 4,050.
- `ki26_battery_time`: 256 × 20 × 2,025,000 / 2,000,000 = 5,184 s = 86.4 min; 256 × 20 × 500 / 2,000,000 =
  1.28 s.
- `ki26_clean_episodes`: ln(0.05)/ln(0.99) = 298.07 → 299; 0.99²⁹⁹ = 0.0495 ≤ 0.05 and 0.99²⁹⁸ = 0.0500 > 0.05;
  the acceptance break-even 600/299 = 2.007 and 3,600/299 = 12.04 GPU-seconds per episode.
- `ki27_ratios`: 0.3696; 0.0973; 5 × 24 × 160 = 19,200; 19,200 / (2 × 24) = 400.
- `ki30_shares`: 4/240, 260/240, 4/410, 260/410.
- `ki32_projection`: half-space projection (1.25, 0.25); clip (1, 0.25) with row value 1.25; exact optimum
  (1, 0.5) by enumeration of active sets, with its KKT multipliers non-negative.
- `ki32_clairvoyant`: for d ∈ {−1, +1} the set of u with u = d is non-empty for each d and empty for both.
- `records_complete_and_verbatim` against `tests/fixtures/killed/g4.md`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) recomputes the calculations, solves the KI-32 program, and runs the
planted revive cases. Logged per STD-E2E.

### Known answers and negative controls

Known answers: the table. Negative controls: a shield implementation that returns (1.25, 0.25) or (1, 0.25)
for the KI-32 case (the carrier's test must fail it; this bead supplies the fixture); a record of KI-26 with
298 clean episodes; a proposal to "use the sampling controller's pass rate as the acceptance result for the
trained policy" (must match KI-26; the run-time refusal is `FROB-REF-817`). The published ratios of KI-27 and
KI-28 are FACTs to re-read: a test asserts that their source ids resolve and that the readings are flagged
until re-read.

## Considerations

- Every entry of this group has a scheduled measurement at M4 (family policies, inheritance, gait orbits,
  shields, the sampling controller at its gate) and KI-29 at M8. The sweeps evaluate the conditions; a strong
  form returns only by a revival ADR.
- KI-26 has one legitimate neighbour that is not a revival: where the sampling controller is itself the
  delivered controller, its own episodes are the evidence, under a frozen planner record. The lint treats a
  citation of that lane as the surviving form.
- KI-27's refutation is about budget: the results that reach 85 to 97% used about 19,200 GPU-hours. An owner
  who approves a compute class of that order changes the evidence, and the record says so.

## References

Appendix C.2.4; §0.8 rows H-01, H-02, H-05, H-20, H-21, H-24; §22.8; §22.12; §23.11; §23.12; construction
C-31; ADR-REFLEX-05.
