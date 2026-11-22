---
key: "ZOO-wheel-leg"
title: "zoo/wheel-leg.fbot: archetype 13, quadruped with driven wheels as feet: C₂ × C₂, two contact laws per foot"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "traction", "frob-design", "frontier", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-04", "ZOO-quad-mammal", "BD-054", "BD-055", "BD-056", "BD-245", "BD-101", "BD-106", "N14a-05"]
needs: []
related: ["BD-365", "N14b-09", "BD-246", "BD-247", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 13", "§14.10", "§18.9", "§14.13 (rejection fixture, foot)", "Appendix M KA-270, KA-283"]
---
## Background

`wheel-leg` is the `quad-mammal` body with a driven wheel in place of each foot. It can roll like a car where
the floor allows and step like a quadruped where it does not, and that is what it tests: two contact laws on
one end-effector class, a switching rule between them, and a mobility analysis that must treat wheels and legs
together. With the wheels free the base gains a rolling distribution; with the wheels braked the wheel rim
becomes a pad and the robot is a legged machine with twelve leg joints. Many designs fail at exactly that
seam: a simulator that binds one law per end effector either cannot represent the hybrid or silently applies
rolling contact to a braked wheel. The hybrid contact laws close on every tier at M4; at M3 the archetype
compiles, passes preflight and has its static known answers. Its mode-graph controller is Frontier. No
reference robot is on file.

## Scope

In: `zoo/wheel-leg.fbot`, its expectation file and controls. Out: the template (N14b-04); the end-effector
`contact` field with its law selectors (N14a-05); the laws `wheel-rolling` and `pad` (BD-245, BD-365); the
hybrid mode-graph controller (N14b-09).

## Technical approach

**Morphology.** The `quad-mammal` term (torso; `mirror(xz, mirror(yz, leg))`; three revolute joints per leg)
with `replace(foot, driven wheel)`: each leg ends in a wheel with a drive actuator. Actuated 16 (12 leg
joints, 4 wheel drives); n_v = 22; bodies 17; end effectors 4, each declaring the law pair
{`wheel-rolling`, `pad`} with a switching rule (`pad` when the wheel's brake state is engaged, `wheel-rolling`
otherwise; the rule is data in the design, evaluated per contact per step).

**Symmetry.** Γ_mech as built: C₂ × C₂, as for `quad-mammal` (the wheels' spin directions map consistently
under both mirrors once the drive sign convention is written). Default Γ_obs: C₂ × C₂.

**What preflight must report (stages 1 to 6 at defaults; no refusal).**

| Stage | Must report |
|---|---|
| 1 Form | 17 fills; 4 end effectors each with two laws and a switching rule; compatibility of each law with the end effector's geometry (a `pad` patch radius not above the wheel's contact radius; a torsion radius not above it) |
| 3 Mechanism | Tree mobility 22; no loops; wheel–body clearance over the leg's range |
| 4 Motion | Two reports, one per mode: wheels free: a rolling distribution on the pose-and-shape space with its growth vector, base kind `wheeled (legged steering)`; wheels braked: not-applicable (legged) |
| 5 Structure | Γ_mech = Γ_obs = C₂ × C₂ |
| 6 Statics and sizing | Standing margin on four wheel contacts; holding torque at the knee as for `quad-mammal` plus wheel drive torque on the declared ramp; envelope, thermal and bus rows pass |

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | C₂ × C₂ | exact |
| n_v; actuated | 22; 16 | exact |
| Law binding | each end effector exposes exactly two contact bindings and one switching rule | exact |
| Braked mode equals a legged quadruped | with all brakes engaged, the contact rows are those of `pad` and the rolling constraint rows are absent: the constraint Jacobian has the same row count as `quad-mammal`'s with patch feet | exact (row counts) |
| Free mode on flat ground at the reference pose (legs fixed) | the four wheel axles are parallel: the planar rolling distribution has rank 1 in (v_x, v_y, ω_z) with the legs frozen (forward only), and reaches full planar mobility only through leg motion (hip abduction steers a wheel) | exact (rank) |
| Contact strata with a mode bit: four end effectors × {open, stick, slip} × {rolling, pad} under C₂ × C₂ | (6⁴ + 3·6²)/4 = 1,404/4 = 351 | count |
| Module reuse | the leg subtree products of `quad-mammal` that do not depend on the foot are reused bit for bit after `replace` (the invalidation set is the end effector and what depends on it) | exact (bits) |

**Control variants.** (a) `variants/incompatible-laws.fbot`: a `pad` law whose torsion radius exceeds the
wheel's own contact radius (the end effector silently acquires torsional support) → refused at stage 1 with
FROB-REF-121 (contact law missing or incompatible with its end effector); (b) a build that binds
`wheel-rolling` unconditionally: with brakes engaged the wheel still rolls freely down a 5% slope, caught by a
braked-hold fixture (zero displacement expected under `pad`); (c) a missing switching rule with two laws
declared → refused at stage 1 (FROB-REF-121).

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Stage 1 accepts the law pair with its rule; the incompatible pair and the missing rule are refused with
      FROB-REF-121.
- [ ] Stage 4 returns both mode reports; the strata count with the mode bit is 351.
- [ ] At M4: the braked-hold fixture holds on a 5% slope and the free-wheel fixture rolls.

## Test plan

### Unit tests

- `zoo_wheel_leg` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_wheel_leg_two_laws_bind`; `zoo_wheel_leg_incompatible_laws_refused_121`;
  `zoo_wheel_leg_missing_rule_refused_121`.
- `zoo_wheel_leg_free_mode_rank_one_legs_frozen`.
- `zoo_wheel_leg_strata_with_mode_bit`: brute force over 6⁴ = 1,296 tuples gives 351 classes.
- `zoo_wheel_leg_reuses_quad_leg_products`: memo hits for the foot-independent leg products.
- `zoo_wheel_leg_braked_holds_on_slope` (M4).

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31): at M3 instance, symmetry, stages 1 to 6 with both stage-4 reports, the
two refused variants; at M4 the slope fixtures on the referee; log lines carry the law pair, the active law
per contact, ranks per mode and `not run: contact law` before M4.

### Known answers and negative controls

KA-270 (Γ_mech: C₂ × C₂); KA-283 applied to this end effector (a torsion radius above the end effector's own
contact radius is refused at stage 1, FROB-REF-121). Controls as listed. Determinism: pinned identities;
integer counts; slope fixtures within the tier's class.

## Considerations

- A braked wheel is not a point foot: it has a patch and a torsion radius bounded by the wheel's contact
  geometry, which is why the compatibility check exists.
- Switching mid-contact is an event for the transactional step; the rule's evaluation instant (the step
  boundary on lockstep tiers, the event time on the referee) is recorded with the tier's profile.

## References

Appendix G.4 row 13; §14.10; §14.13; §18.9; KA-270, KA-283; FROB-REF-121.
