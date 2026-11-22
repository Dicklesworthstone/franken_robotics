---
key: "ZOO-swerve-4"
title: "zoo/swerve-4.fbot: archetype 11, four steer-and-drive modules: D₄, nested symmetry, scrub torque, singular pose"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "traction", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-03", "BD-054", "BD-055", "BD-056", "BD-060", "BD-154", "BD-106", "N14b-11", "BD-102"]
needs: []
related: ["GB-20", "N18a-27", "N14b-07", "BD-246", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 11", "§14.10", "§13.6", "§18.6", "C-37", "Appendix M KA-270, KA-399"]
---
## Background

`swerve-4` is a square base with four identical modules, each of which steers a wheel about a vertical axis
and drives it. It can move in any direction after reorienting its wheels, and it brings three tests that no
other archetype has. Nested symmetry: each module has its own mirror, and the hub permutes four modules, so
the group of the whole is built from the group of the part, a structure the algebra must report rather than
flatten. Scrub: steering a stationary wheel twists its contact patch, and the resisting moment comes from the
spin component of the four-dimensional contact cone; a three-dimensional cone predicts zero steering torque,
which undersizes every steering actuator. And a singular pose: when all four wheel axes pass through one
point the base can only rotate about that point, and at a generic set of fixed steering angles it cannot move
at all. No reference robot is on file. It lands at M3.

## Scope

In: `zoo/swerve-4.fbot`, its expectation file and controls. Out: the template (N14b-03); the four-dimensional
cone and the effective patch radius (BD-154, N18a-27); the steered-base controller (N14b-07).

## Technical approach

**Morphology.** Square base; `radial(4, z, module)`; module = steer joint (vertical axis) then drive joint;
wheel end effector with law `wheel-rolling` and a finite patch. Actuated 8; n_v = 14 (floating base 6 plus
four steer and four drive coordinates); bodies 9; end effectors 4.

**Symmetry.** Γ_mech as built: D₄ on a square base (order 8), with each module's own mirror nested inside
(the module's mirror through its steer axis maps the module to itself at zero steer). Default Γ_obs: D₄.

**Constraint at fixed steering.** Wheel i at pᵢ with axle direction aᵢ (unit, horizontal) imposes no lateral
slip: aᵢ·(v + ω_z·ẑ × pᵢ) = 0. Stacked for four wheels this is a 4 × 3 matrix in (v_x, v_y, ω_z).

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 1: one module by hash, used
four times. Stage 3: tree mobility 14. Stage 4: with steering free, the distribution has full rank after one
bracket and the class `omnidirectional (steered)` is confirmed; the singular set (all axles concurrent) is
named. Stage 5: D₄ with the nested module mirror reported as nested, not as an extra factor. Stage 6: steering
torque at standstill includes scrub; drive and steer points inside the envelope.

**Known answers.** Fixture: wheels at (±0.25, ±0.25) m.

| Name | Value | Tolerance |
|---|---|---|
| Scrub torque with no tangential force | τ_n = μ·f_n·ρ; f_n = 50 N, μ = 0.8, ρ = 8 mm → 0.32 N·m | exact |
| Scrub torque from a cone without the spin component | 0 | exact |
| Scrub with simultaneous sliding at v = ρ·ω | 0.32/√2 = 0.226274 N·m | rel 10⁻⁹ |
| Rank at generic fixed steering angles (10°, 35°, 80°, 120° from x) | 3: the base is locked | exact |
| Rank with all axles through one point P = (1, 0) m | 2: the only motion is rotation about P, twist ∝ (0, −1, 1)·ω for P on the x axis (v = ω·ẑ × (−P)) | exact |
| Rank with all wheels parallel (axles along y) | 2: the only motion is translation along x, null twist (1, 0, 0) (the concurrent point at infinity) | exact |
| Γ_mech | D₄ (order 8) | exact |
| Contact strata, four wheels × {open, stick, slip} under D₄ | (3⁴ + 2·3 + 3² + 2·3³ + 2·3²)/8 = 168/8 = 21 | count |

**Control variants.** (a) a three-dimensional friction cone (no spin component): steering torque at standstill
is zero and the steering actuator passes an envelope it should fail; the test asserts 0.32 N·m is required;
(b) the design asserting `D4 x C2` (the module mirror counted as an independent factor) → FROB-REF-109;
(c) a controller command at the concurrent pose asking for pure translation is infeasible (rank 2): the
analysis names the singular pose and the test asserts the report, not a silent least-squares answer.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Scrub torque is 0.32 N·m in the fixture through the contact probe; the rank rows hold exactly.
- [ ] Γ_mech is D₄ of order 8 with the nested structure reported; the three controls behave as stated.

## Test plan

### Unit tests

- `zoo_swerve4` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_swerve4_scrub_torque`: 50 N, 0.8, 0.008 m → 0.32 N·m; with sliding at v = ρ·ω → 0.226274 N·m.
- `zoo_swerve4_rank_generic_locked`, `zoo_swerve4_rank_concurrent_two`, `zoo_swerve4_rank_parallel_two`.
- `zoo_swerve4_strata_bruteforce`: 81 tuples under the 8 elements give 21 classes.
- `zoo_swerve4_nested_mirror_not_a_factor`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6, `frob contact probe` at the
scrub state (spin only, and spin with sliding); log lines carry the group and its nesting, the ranks at the
three poses and the scrub torque with its provenance class (Estimated radius).

### Known answers and negative controls

KA-270 (Γ_mech: D₄ on a square base); KA-399 (sliding while spinning on the four-dimensional cone: each
component is 1/√2 of its limit at v = r_eff·ω). Controls as listed. Determinism: exact rank computations;
closed-form scrub.

## Considerations

- The patch radius ρ is Estimated from geometry unless an interface card supplies it; the scrub row carries
  that class.
- Near the singular pose the steer rates needed grow without bound; the controller's handling belongs to its
  own bead, and this golden only fixes what the analysis must say.

## References

Appendix G.4 row 11; §14.10; §13.6; §18.6; C-37; KA-270, KA-399; FROB-REF-109.
