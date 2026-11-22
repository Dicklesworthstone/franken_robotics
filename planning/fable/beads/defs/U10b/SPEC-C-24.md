---
key: "SPEC-C-24"
title: "spec C-24 (frob-learn): geometric-algebra tree policy: multivector tokens, gravity-preserving equivariance"
type: "task"
priority: 4
milestone: "M8"
labels: ["m8", "lemma", "spine", "spec", "moonshot", "frob-learn", "test"]
parent: "WP-79"
deps: ["BD-025", "BD-192", "SPEC-C-23", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-24", "§10 registries, Appendix M.9 and Appendix N.7: the chapter registry beads N10-KA (unit U10a) and N10-CX (unit U10c) that load row KA-131 and CX-061; this bead binds them to tests and sets the round-off bound the appendix leaves `to set`", "§10 C-04: the order of the tree recursions (outward kinematics, inward forces) from the specification bead SPEC-C-04 (unit U10a), which fixes the order of message passing"]
related: ["N10b-16", "BD-439", "BD-320", "BD-325", "BD-326", "SPEC-C-04", "SPEC-C-09", "SPEC-C-02"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-24", "§9 D-6", "§9 D-10", "§11.4", "§23.14", "§23.23", "Appendix C", "Appendix M.9 KA-131", "Appendix N.7 CX-061"]
---
## Background

Construction C-24 is a Moonshot [M] variant of the family policy of construction C-23 (one set of weights that
acts on one token per link of any design in a morphology class). In C-24 each link token carries
**multivector features** in projective geometric algebra, and the layers are equivariant by construction under
relabeling and under the rigid motions that preserve gravity. If it works, invariance to heading and to
description conventions comes from the architecture instead of from data augmentation, and the function class
is smaller for the same behaviour. It replaces nothing on the critical path.

The honest prior is that it does not pay. FACT (source register, to be re-read): no published policy of this
kind exists for multi-embodiment locomotion; a pure geometric-algebra network inside a control loop converged
very slowly; such layers ran 1.4 to 7.5 times slower per epoch, so equal parameters is not equal compute;
fixed tree message passing has lost to plain transformers; heading normalization with augmentation recovers
most of the benefit. The expected gain is between none and 1.3×. That is why the construction is admitted only
with a ceiling stated first, a three-arm pilot (N10b-16) and a kill gate (KILL-GATREE), and why at most four
Moonshot items are active at once and none is on a critical path. Crate: `frob-learn`. Milestone: M8.

Terms. *Projective geometric algebra*: the algebra in which points, lines, planes and rigid motions are
multivectors; the sibling library `fs-ga` provides it. *Motor*: the multivector of a rigid motion. *Bivector*:
the grade-two element that represents a line (a joint axis with its position). *Equivariant layer*: a layer
whose output transforms with its input under the group. *Gravity-preserving rigid motions*: rotations about
the vertical, translations, and reflections through vertical planes.

Rejected alternatives: equivariance under all rotations (wrong: a rotation about a horizontal axis does not
preserve gravity, and a policy equivariant under it could not tell up from sideways); making this the default
family-policy architecture (no evidence; the control arm is the plain transformer).

## Scope

In: `docs/math/C-24.md` (statements S1 to S5); the law suite `crates/frob-learn/tests/laws_c24.rs` with
reference layers and mutants, compiled only under the cargo feature `moonshot-ga-policy` (off by default);
registry row KA-131; controls from CX-061; row C-24 of the construction register.

Out: the three-arm experiment (N10b-16); the epic that scales it if the pilot passes (BD-439); the family
policy and its tokens (SPEC-C-23, BD-320); equivariant linear maps by irreducible blocks for finite groups
(BD-325), which is a different mechanism for a different group.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-24.S1 | Each link token carries multivector features in the projective geometric algebra of `fs-ga`: the link's motor, its joint line as a bivector, its momentum and its wrench | Definition |
| C-24.S2 | Layers are equivariant under relabeling and under the rigid motions that preserve gravity: rotations about the vertical, translations and reflections through vertical planes | `Tested` to round-off, for every layer |
| C-24.S3 | A rotation about a horizontal axis is **not** a symmetry of the policy (gravity control) | `Tested` (negative property) |
| C-24.S4 | Messages pass along the kinematic tree in the order of the tree recursions of the dynamics (an outward pass from base to leaves, then an inward pass from leaves to base) | Definition; `Tested` (message order) |
| C-24.S5 | The construction replaces nothing on the critical path; its expected gain is between none and 1.3×, on an amortized pretraining run, which is not a source of the ten-fold | Non-claim and ceiling |

### 2. The symmetry group and how features transform (contract sections 3 and 5)

The group is G = { x ↦ A·x + t : A ∈ O(3), A·e_z = e_z } with e_z the vertical: A is a rotation about e_z or a
reflection through a plane containing e_z. Under an element (A, t):

- positions map by A·x + t; linear velocities, forces and linear momenta (polar vectors) by A;
- angular velocities, torques and angular momenta (axial vectors) by det(A)·A;
- a twist or wrench at the origin by the block map diag(det(A)·A, A) on (angular, linear), followed by the
  shift of origin for the translation; this map preserves the power pairing of wrench and twist;
- a motor by conjugation with the motor (or reflection versor) of (A, t); a joint line by the same
  conjugation;
- the gravity vector g·(−e_z) is fixed, and is supplied to the network as a constant feature. Because it is
  fixed by G and not by a tilt, statement S3 holds: a rotation about a horizontal axis moves the gravity
  feature relative to the body, and the output is not required (and must not be forced) to follow.

Reflections matter: a mirrored robot turning left is a robot turning right, and treating angular velocity as a
polar vector leaves the yaw rate unchanged, which is the classical mix-up this parity rule prevents.

### 3. Layer requirements (DESIGN; what a law-abiding layer may use)

1. Linear maps that commute with the action of G on multivector grades (geometric products with G-invariant
   multivectors, grade projections, the join with the vertical).
2. Nonlinearities applied to G-invariant scalars only (norms, inner products, the vertical component), with
   the result scaling the equivariant features.
3. Aggregation over children by a permutation-invariant sum in a canonical order.
4. No positional encoding by token index; tree structure enters only through parent–child relations.

### 4. Pilot, kill rule, cost, ceiling and fallback (carried in full; run by N10b-16)

Three arms on one embodiment set at equal wall-clock budget with three seeds each. Arm A, the control: a plain
link-token transformer with heading normalization and convention augmentation. Arm B: A with a
geometric-algebra encoder and decoder only. Arm C: full geometric-algebra message passing on the tree.
Measures: time to a return threshold, zero-shot return on held-out designs, retention under re-description.
Kill (TARGET): neither B nor C is at least 1.3× faster than A to the threshold, nor higher in zero-shot return
beyond seed spread. Cost: on the order of 90 GPU-hours (an estimate; it needs an owner-approved compute
envelope). Ceiling: 1.3× on an amortized pretraining run. Fallback: arm A.

### 5. Foundations, law suite, registry

- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-ga` has multivectors and motors.
  Not admitted for policy use until its admission gate passes.
- `crates/frob-learn/tests/laws_c24.rs` (feature `moonshot-ga-policy`), generic over `C24Subject { fn
  layer(&self, i: usize, tokens: &[MvToken], tree: &Tree) -> Vec<MvToken>; fn n_layers(&self) -> usize;
  fn act(&self, tokens, tree) -> Vec<Action> }`. `tests/support/c24_ref.rs`: a reference equivariant layer
  stack built from the allowed operations with fixed rational weights (a real, small network; labelled
  `synthetic` data); generators of group elements (yaw angles with rational sine and cosine such as (3/5, 4/5),
  rational translations, reflections through vertical planes). `tests/support/c24_mutants.rs`: mutants under
  the cargo feature `controls`.
- Register row C-24: `rust_type` = the multivector token type of `frob-learn` (filled if the epic is funded),
  `verb = "train.family"` with the architecture option, `failing_test` =
  `frob-learn::laws_c24::s3_horizontal_rotation_not_a_symmetry`, `payoff.gate = "GATE-CORTEX-17"`,
  `known_answers = KA-131`, `pilot = "PILOT-C-24"`, `generic_comparator = "a plain link-token transformer with
  heading normalization and convention augmentation"`. Until the pilot runs, the row's status is `parked`
  outside its M8 window, as the register's grace rule allows.
- Gate that measures the construction (recorded by the epic BD-439): GATE-CORTEX-17 at M8 (arm B or C is at
  least 1.3 times faster than arm A to the return threshold at equal wall-clock, or better with no adaptation
  beyond seed spread). Kill gate KILL-GATREE (GATE-CORTEX-17 fails; fallback: the plain morphology transformer
  remains and `fs-ga` features are not used in policies).

## Acceptance criteria

- [ ] `docs/math/C-24.md` has the ten sections, statements C-24.S1 to S5 with classes, the transformation
      table, the evidence figures tagged FACT and the pilot statement.
- [ ] `cargo test -p frob-learn --features moonshot-ga-policy --test laws_c24` passes; with `controls` added
      every mutant fails. A default build compiles none of it.
- [ ] KA-131: for every layer of the reference stack and 100 seeded group elements of each kind (yaw,
      translation, vertical-plane reflection, relabeling), applying the element before the layer equals
      applying it after, exactly in ℚ for rational elements and within 64 units in the last place in `f64`
      otherwise (the bound set here).
- [ ] Gravity control: a rotation by 90° about a horizontal axis changes the output in a way no group action
      on the output explains (the equivariance residual is of order one, at least 10⁶ times the round-off
      bound).
- [ ] The all-rotations mutant (a layer equivariant under the full rotation group) fails the gravity control;
      the polar-parity mutant fails under a vertical-plane reflection.
- [ ] `scripts/e2e/spec_c24.sh` exits 0.

## Test plan

### Unit tests

- `s1_token_fields`: each token carries a motor, a joint line, a momentum and a wrench with their grades.
- `s2_yaw_equivariance_per_layer`, `s2_translation_equivariance_per_layer`,
  `s2_vertical_reflection_equivariance_per_layer`, `s2_relabeling_equivariance_per_layer`: the acceptance
  property, one test per kind and per layer index.
- `s2_parity_of_axial_features`: under the mirror diag(1, −1, 1) a pitch rate keeps its sign and roll and yaw
  rates change theirs.
- `s2_pairing_preserved`: the power pairing of a wrench feature and a twist feature is unchanged by every
  group element.
- `s3_horizontal_rotation_not_a_symmetry`: the gravity control.
- `s4_message_order`: outward then inward; a leaf's first-layer output does not depend on a sibling subtree
  before the inward pass.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c24.sh` (idempotent; seed 2400 printed): `frob --json doctor`; the law suite with the
feature flags; `cargo test -p frob-conformance --test g3 -- KA-131` on the host site; a check that a default
build does not contain the suite. Sources the STD-E2E helper; writes `target/e2e-logs/spec_c24/<utc>.ndjson`
and a transcript; each line has `ts`, `script`, `bead` (`SPEC-C-24`), `step`, `event` (`start`, `cmd`, `assert`,
`metric`, `refusal`, `end`), command line, exit code, wall time, evidence block (claim Known answer, class
Tested, tier G3 host, determinism class, seed), per layer and group-element kind the largest equivariance
residual against its bound, the gravity-control residual against its floor, and on failure the first layer and
group element whose residual exceeds the bound. Final `summary`: assertion counts and GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-131 | Every layer of the geometric-algebra policy is equivariant under yaw, translation, vertical-plane reflection and relabeling | round-off: exact in ℚ for rational elements; 64 units in the last place in `f64` (set here; DESIGN) | G3 · host | CX-061 (gravity control): a rotation about a horizontal axis must not be a symmetry of the policy; the all-rotations mutant is caught by it |

Metamorphic relations: composing two group elements and applying once equals applying them in sequence; the
action commutes with adding a leaf whose features are zero. Determinism: canonical summation order over
children; twice on one machine, debug and release.

## Considerations

- Moonshot budget: at most four Moonshot items are active at once, from the funding of a pilot to its expiry
  gate; this one waits in the parked register until M8 and until a slot and a compute envelope exist.
- Equal parameters is not equal compute. Every comparison is at equal wall-clock, and the 1.4 to 7.5 times
  slower per-epoch figures are the reason.
- The law suite is cheap and can be written whenever the family-policy contract exists; it is scheduled at M8
  with the pilot so that no unfunded Moonshot work lands early.
- If the kill gate fires, the contract and the law suite are kept as a record of what was tested, and the
  feature flag is removed from the crate.

## References

Plan §10 C-24; §9 D-6, D-10; §11.4; §23.14, §23.23; Appendix C (killed-idea register and expected-value
matrix); KA-131; CX-061; BET-30; GATE-CORTEX-17; KILL-GATREE.
