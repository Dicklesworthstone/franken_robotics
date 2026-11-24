---
key: "SPEC-C-23"
title: "spec C-23 (frob-learn): a family policy as a section: link tokens and naturality under re-description"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "spine", "spec", "frontier", "frob-learn", "test", "review-notes-open"]
parent: "WP-61"
deps: ["BD-022", "BD-025", "N10-CX", "N10-KA2", "N10a-01", "SPEC-C-21", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-145", "BD-192", "BD-310", "N23b-19", "SPEC-C-26"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-03", "GATE-GAUNT-01"]
plan_refs: ["§10 C-23", "§9 D-10", "§23.7", "§23.11", "§23.22", "§23.23", "§38.7 MR-23", "Appendix E GB-28", "Appendix I.14", "Appendix M.9 KA-128", "Appendix N.7 CX-060"]
---
## Background

Training one policy per design is the largest cost of a design campaign. Construction C-23 replaces it with a
**family policy**: one set of weights that acts on the tokens of any design of a morphology class, so that most
designs in a campaign are scored without training and the child of an edit starts from its parent. This is
where trainings are avoided, the largest term in the project's ten-fold thesis. The construction fixes what
such a policy must satisfy to be trusted: it must be **natural under re-description**. A robot described with
its links in another order, in other frames, with another axis-sign convention or in other units is the same
robot, and the policy must return the same actions after mapping back. FACT (source register, to be re-read):
a published morphology-conditioned policy lost most of its return when the same robots were re-described in
equivalent conventions.

Tier: Frontier [F]. Owning chapter: Cortex; crate `frob-learn`. It serves BET-15 (family policies) and BET-16
(inheritance). This bead specifies the section and its naturality; the transport along edits is N10b-14; the
pilots are N10b-15 (Pilot 2, edits) and Pilot 1, the forty-design experiment shared with construction C-26,
whose protocol is pre-registered by BD-145 and executed by BD-310, with the Cortex family-policy pilot N23b-19
(those beads carry the protocol stated in section 4 below; they are `related`, not prerequisites).

Terms. *Design family* 𝒟: a set of designs in which the observation space Y_d and the action space U_d change
with the design d. *Bundle*: the collection of policy spaces Π_d, one per design. *Section*: a choice of one
policy per design, σ: d ↦ π_d. *Token*: one input vector of the network. *Permutation-equivariant*: permuting
the input tokens permutes the outputs in the same way. *Isomorphism of designs*: a re-description that changes
no physical fact. *Γ_obs orbit label*: a label shared by links that the observation-preserving symmetry group
maps onto each other.

Rejected alternatives: one universal policy across morphology classes (FACT: across topology-diverse designs at
affordable pretraining a shared policy reaches 10 to 40% of specialist return); a policy that reads tokens in
file order (fails at the first exchange of two legs); raw SI features (construction C-21).

## Scope

In: `docs/math/C-23.md` (statements S1 to S6; the transport section comes from N10b-14); the law suite
`crates/frob-learn/tests/laws_c23.rs` with reference model and mutants; registry row KA-128; relation MR-23;
controls from CX-060; row C-23 of the construction register; the statement of Pilot 1 for the beads that run
it.

Out: networks, pretraining and the mapping table as product features (BD-320); the held-out experiment and its
gate (BD-321); inheritance (N10b-14, BD-322); rank agreement (SPEC-C-26, BD-328); on-device network kernels
(BD-196).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-192 (the vectorized
environment of `frob-learn`): that bead is listed in `related` and is not a prerequisite. The law suite is generic
over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-learn` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance` (a control's build there reads `--features gauntlet-mutations,<feature>`, its mutation feature declared in `frob-conformance`); the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-23.S1 | The policy spaces Π_d form a bundle over 𝒟, and a design-conditioned policy is a section σ: d ↦ π_d, realized by one set of weights θ acting on tokens: π_d = Φ_θ(tokens(d), ·) | Definition |
| C-23.S2 | Tokens: one *link token* per link, formed with its parent joint, and one token per contact site | Definition; `Tested` (token count equals links plus contact sites) |
| C-23.S3 | Static features are dimensionless: the joint's screw basis in its own frame, the placement log(parent_from_joint), the scaled subtree pseudo-inertia, the actuator groups, tree depth and parent index, and the Γ_obs orbit label. Dynamic features are the unit-complex joint coordinate, scaled velocity, last action and contact flag | Definition; `Tested` (invariance of features under similarity scaling) |
| C-23.S4 | The network is permutation-equivariant in tokens, with attention or message passing informed by the tree, and weights are tied across orbit labels | `Tested` |
| C-23.S5 | Naturality: for every isomorphism φ: d → d′ of designs (a relabeling of links, a change of frame or of axis-sign convention, a similarity scaling), σ(d′) = φ·σ(d) | `Tested` to round-off (relation MR-23) |
| C-23.S6 | Design rules that follow from the evidence: one family policy per morphology class and never one universal policy; a cheap rung discards and does not select until rank agreement is measured; chained inheritance resets on a schedule against loss of plasticity; an edit that changes support discontinuously (a mirrored pair, a removed limb, a wheel exchanged for a foot) has its own gate | Rules; enforced by refusals and gates of the implementing chapter |

### 2. Definitions made precise (DESIGN where the source leaves a choice)

- **Placement feature.** log(parent_from_joint) ∈ 𝔰𝔢(3), with its translation part divided by ℓ₀ (the design's
  radius of gyration, the canonical length of construction C-21).
- **Scaled subtree pseudo-inertia.** The 4 × 4 pseudo-inertia of the subtree rooted at the link, expressed in
  the link frame, with mass divided by m₀ and lengths by ℓ₀.
- **Actuator groups.** τ_max/(m₀·g·ℓ₀) and ω_max·t₀ with t₀ = √(ℓ₀/g).
- **Unit-complex joint coordinate.** (cos q, sin q) for a revolute joint; no raw angle is stored.
- **The action of an isomorphism on a policy.** φ maps observation channels and action channels by a signed
  permutation with unit conversions (axial quantities change sign under a mirror or an axis-sign flip). The
  statement σ(d′) = φ·σ(d) means: observe in d′, map the observation back to d by φ⁻¹, apply π_d, map the
  action forward by φ; the result equals π_d′ applied directly.
- **Parent index** is a feature of the tree structure given as a relation between tokens (which token is whose
  parent), not as a position in a list; otherwise S4 would fail.

### 3. Honest limits (FACT, source register; each figure to be re-read from its primary source)

Within one morphology class a family policy reaches 70 to 85% of specialist return without adaptation and
matches it after about 2 to 25% of a from-scratch run stopped at matched return. Across topology-diverse
designs at affordable pretraining it reaches 10 to 40%, and fine-tuning gains a factor of 2 to 3. Inheritance
from a class prior costs about 2 to 25% of such a run; the one published parent-to-child figure is about 50%;
no source measures a small typed parameter edit; single-limb topology edits cost 33 to 100%. Shared control
biases a population toward designs that are easy to steer.

Economics (illustrative, on FACT denominators). With pretraining cost P, specialist cost s per design and
fine-tuning fraction f, the family policy pays after N > P/(s·(1 − f)) designs, and its gain on the training
phase is at most 1/f. P = 60 GPU-hours and f = 0.1: about 590 quadruped designs at s = 410 s, and 67 to 133
humanoid designs at s = 1 to 0.5 GPU-hours; at f = 0.2 the quadruped figure is 660. Pretraining at 10⁹ to 10¹⁰
steps costs about 1 to 30 GPU-hours and amortizes only over a campaign.

Foundations (FACT, observed 2026-11-21): FrankenTorch has attention modules with autograd on the CPU. Its
tensors are host memory and its device tags are metadata, so on-device network kernels and their backward
passes are this project's to build.

### 4. Pilots, kill rule, ceiling and fallback (carried for the beads that run them)

- **Pilot 1, shared with construction C-26.** One class, 40 designs; zero-shot and 5% fine-tune against
  specialists.
- **Pilot 2.** 12 parent–child edit pairs of which 4 are blind (N10b-15).
- **Kill (TARGET).** Zero-shot return below 70% of specialist on more than half of the held-out set together
  with fine-tuning above 0.2 of specialist steps. This is the trigger of kill gate KILL-FAMILY (decided at M4;
  fallback: no family policy: specialists with inheritance from the nearest parent, and the baseline screens).
- **Ceiling.** The bound 1/(1 − f + f/r + h) with f the training share of a design loop, r the measured ratio
  and h the overhead.
- **Fallback.** Per-design training, with inheritance kept for parameter edits.

### 5. Law suite and registry (DESIGN)

- `crates/frob-learn/tests/laws_c23.rs`, generic over `C23Subject { fn tokens(&self, d: &DesignData) ->
  Vec<Token>; fn act(&self, d: &DesignData, obs: &Obs) -> Action }`. `tests/support/c23_ref.rs`: a synthetic
  corpus (labelled `synthetic`) of small tree designs with rational parameters; a reference tokenizer; a small
  reference policy that is permutation-equivariant by construction (per-token map, sum pooling over children,
  per-token head) with fixed rational weights; generators of re-descriptions (random relabeling, random frame
  change per link, axis-sign flips, power-of-two similarity scaling). `tests/support/c23_mutants.rs`: each mutant
  compiled only under its own Cargo feature, declared by `frob-learn` and named in its control record's `feature`
  field (N10a-01 section 6, BD-025): `cx-060` for the plant `SPEC-C-23:cx-060` of CX-060 (the file-order
  policy), the `nc-` feature of each control this bead registers directly (the SI-feature mutant, the mutant
  without sign handling). No mutation feature is part of the umbrella `gauntlet-mutations` or of the crate's
  `conformance` feature: the suite is run once per control, one control per build. The reference policy is a real, small policy, not a stand-in for the
  product network: the product network of BD-320 is bound to the same laws.
- KA-129 and KA-130, which the appendix printed with tolerance `to set` and control `none stated`, are bound by
  the part bead N10b-14, which sets both tolerances (exact: bit equality of the first action; bit-equal
  initializations on matched tokens) and registers both controls (a transport that re-initializes the encoder
  of every token; transport by index instead of by matched token, MR-24's control); N10-KA2 carries these cells
  marked "(set: N10b-14)". KA-128's tolerance and control are set here.
- Register row C-23: `rust_type` = the family-policy and mapping-table types of `frob-learn`, `verb =
  "train.family"` (with `policy.apply`), `failing_test` = `frob-learn::laws_c23::s5_file_order_policy_caught`,
  `payoff.gate = "GATE-CORTEX-11"`, `known_answers = KA-128, KA-129, KA-130`, `pilot = "PILOT-C-23"`,
  `generic_comparator = "per-design specialist training; a raw-checkpoint warm start"`.
- Gates that measure the construction (recorded by mechanism beads): GATE-CORTEX-11 at M4 (median return with
  no adaptation at least 70% of a specialist's; fine-tuning reaches specialist return in at most 20% of the
  steps of a from-scratch run stopped at 95% of its final return; out-of-class designs declined; re-described
  robots keep at least 80% of return); GATE-CORTEX-12 (the rank-agreement record exists and a rung without a
  record is refused; refusal FROB-REF-537); GATE-CORTEX-13 (inheritance); kill gates KILL-FAMILY, KILL-INHERIT.

## Acceptance criteria

- [ ] `docs/math/C-23.md` has the ten sections, statements C-23.S1 to S6 with classes, the feature list, the
      evidence figures tagged FACT and the pilot statements.
- [ ] `cargo test -p frob-learn --test laws_c23` passes; built once per registered control with the feature its
      record names (`cargo test -p frob-learn --features <feature> --test laws_c23`: `cx-060`, and the `nc-`
      feature of each other control), the test the record names fails.
- [ ] KA-128: for every design of the synthetic corpus (at least 50 designs) and 20 seeded re-descriptions
      each, the actions after mapping back differ from the original by at most 64 units in the last place in
      `f64` (the bound set here; exact equality for relabelings and power-of-two scalings).
- [ ] The file-order mutant fails at the first exchange of two legs; the SI-feature mutant fails under
      power-of-two scaling; the mutant without sign handling fails under an axis-sign flip.
- [ ] Token features of a design and of its similarity image are bit-identical for power-of-two factors.
- [ ] Weights tied across orbit labels: exchanging two links of one Γ_obs orbit leaves the parameter count and
      the output multiset unchanged.
- [ ] `scripts/e2e/spec_c23.sh` exits 0.

## Test plan

### Unit tests

- `s2_token_count`: links plus contact sites for each corpus design.
- `s3_features_dimensionless`: every static and dynamic feature has dimension exponent (0, 0, 0);
  `s3_features_invariant_under_similarity`: lengths × 4, times × 2, masses × 2 leave every feature bit-equal.
- `s4_permutation_equivariance`: a random permutation of tokens permutes per-token outputs identically.
- `s4_orbit_tying`: the acceptance property.
- `s5_relabeling`, `s5_frame_change`, `s5_axis_sign_flip`, `s5_similarity_scaling`, `s5_composed_redescription`:
  actions unchanged after mapping back, within the bound.
- `s5_file_order_policy_caught`: a quadruped with two legs exchanged in the description.
- `s6_out_of_class_declined`: a design outside the declared class is declined, not scored.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c23.sh` (idempotent; seed 2300 printed): `frob --json doctor`; the law suite
(`cargo test -p frob-learn --test laws_c23`), then once per registered control in its own build with the feature
its record names (`cargo test -p frob-learn --features <feature> --test laws_c23`: `cx-060`, the `nc-` feature of
each other control), asserting that the test the record names fails; the two rows through the helper's
`e2e_tier` on the host site, one name filter each: `e2e_tier g3_metamorphic KA-128` and `e2e_tier g3_metamorphic
MR-23` (`FROB_TEST_SEED=2300 cargo test -p frob-conformance --test g3_metamorphic -- KA-128`, and the same with
`-- MR-23`), the receipt hashes logged; existence of `docs/math/C-23.md`. Sources the STD-E2E helper; writes `target/e2e-logs/spec_c23/<utc>.ndjson` and a
transcript; each line has `ts`, `script`, `bead` (`SPEC-C-23`), `step`, `event` (`start`, `cmd`, `assert`,
`metric`, `refusal`, `end`), command line, exit code, wall time, evidence block (claim Known answer, class
Tested, tier G3 host, determinism class, seed), the largest action difference in units in the last place
against its bound, and on failure the first design and re-description for which the difference exceeds the
bound, with the kind of re-description. Final `summary`: assertion counts, GATE-GAUNT-02 and GATE-GAUNT-03.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-128 | Re-description of a design (random relabeling, frame change, power-of-two scaling), for every design in the corpus: the actions are unchanged after mapping back | round-off: 64 units in the last place in `f64` (set here; DESIGN), exact for relabeling and power-of-two scaling | G3 · host here; the product network in BD-320 | CX-060: a policy that reads tokens in file order fails at the first exchange of two legs |

Relation MR-23: re-description of a robot (relabeling, frame change, axis-sign convention, power-of-two
scaling) leaves a family policy's actions unchanged after mapping back; its control is file-order tokens.
Determinism: the reference policy is evaluated in a fixed summation order (children sorted by a canonical key)
so that relabeling gives bit-equal results; twice on one machine, debug and release.

## Considerations

- Naturality is necessary and far from sufficient: it says the policy does not depend on the description, not
  that it is any good. Return is measured by the gates.
- Sum pooling in floating point is order-dependent; equivariance to round-off therefore needs either a
  canonical summation order or a tolerance. The contract fixes the canonical order and keeps the tolerance for
  frame changes only.
- Compute: pretraining needs an owner-approved envelope (the open question on family-policy pretraining);
  until it is approved the default is the variant with no family policy.
- A family policy with no rank-agreement record is not a rung and may neither select nor discard designs.

## References

Plan §10 C-23; §9 D-10; §23.7, §23.11, §23.22, §23.23; §38.7 (MR-23); Appendix E GB-28; Appendix I.14; KA-128;
CX-060; BET-15, BET-16; GATE-CORTEX-11, -12, -13; KILL-FAMILY; FROB-REF-537.
