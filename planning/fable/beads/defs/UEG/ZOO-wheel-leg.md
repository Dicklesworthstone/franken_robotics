---
key: "ZOO-wheel-leg"
title: "zoo/wheel-leg.fbot: archetype 13, quadruped with driven wheels as feet: C₂ × C₂, two contact laws per foot"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "traction", "frob-design", "frontier", "test", "zoo"]
parent: "WP-38"
deps: ["BD-054", "BD-055", "BD-056", "BD-101", "BD-106", "BD-245", "N14a-05", "N14b-04", "N14bp2-02", "NEG-30", "NEG-31", "ZOO-quad-mammal"]
needs: []
related: []
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

In: `zoo/wheel-leg.fbot`, its expectation file with every row below (the slope fixtures written with
`blocked = "milestone"` and `due = "M4"`, NEG-30's field for a known answer whose mechanism lands later), the static rows and the three controls. Out: the template (N14b-04); the
end-effector `contact` field with one law (N14a-05, whose schema holds one law); its two-law form on a rolling
wheel (`braked` with `switch`), the typing of the pair and its FROB-REF-121 triggers, the compile product (two
bindings and one switch per wheel, the command channel `brake:<joint id>`) and stage 4's one report per mode
(N14bp2-02); the laws `wheel-rolling` and `pad` (BD-245,
BD-365); the hybrid mode-graph controller (N14b-09); the M4 evaluation of the braked-hold and free-roll slope
fixtures and of control (b) in simulation, which NEGp1-53 carries at M4 so that this M3 bead closes at M3.

## Technical approach

**Morphology.** The `quad-mammal` term (torso; `mirror(xz, mirror(yz, leg))`; three revolute joints per leg)
with `replace(foot, driven wheel)`: each leg ends in a wheel with a drive actuator. Actuated 16 (12 leg
joints, 4 wheel drives); n_v = 22; bodies 17; end effectors 4, each a wheel whose `contact` object, in
N14bp2-02's written form, is `{law: wheel-rolling, interface, braked: {law: pad, interface, patch_radius,
torsion_radius}, switch: {signal: brake, joint: <the wheel's own axle joint>}}`, with patch_radius at most the
wheel's contact radius (half the tread width for a cylindrical wheel) and torsion_radius at most patch_radius.
The rule is data in the design: `pad` is in force while that joint's brake is engaged, `wheel-rolling`
otherwise. The compile product is two bindings per wheel (roles `Primary` with `wheel-rolling` and `Braked` with
`pad`) and one contact switch on the boolean command channel `brake:<joint id>`, reset value released.

**Symmetry.** Γ_mech as built: C₂ × C₂, as for `quad-mammal` (the wheels' spin directions map consistently
under both mirrors once the drive sign convention is written). Default Γ_obs: C₂ × C₂.

**What preflight must report (stages 1 to 6 at defaults; no refusal).**

| Stage | Must report |
|---|---|
| 1 Form | 17 fills; 4 end effectors each with two laws and a switching rule; N14bp2-02's typing of each pair: 0 < patch_radius ≤ the wheel's contact radius, 0 ≤ torsion_radius ≤ patch_radius, the switch joint the wheel's own axle |
| 3 Mechanism | Tree mobility 22; no loops; wheel–body clearance over the leg's range |
| 4 Motion | Two reports, one per mode, as N14bp2-02 makes stage 4 report (N14b-11 alone classifies once): mode `released` (wheels free): base kind wheeled, a rolling distribution on the pose-and-shape space with its growth vector; mode `engaged` (wheels braked): no rolling law in force, the base is neither and the report is `not-applicable`; the stage's status is `passed` with both reports |
| 5 Structure | Γ_mech = Γ_obs = C₂ × C₂ |
| 6 Statics and sizing | Standing margin on four wheel contacts; holding torque at the knee as for `quad-mammal` plus wheel drive torque on the declared ramp; envelope, thermal and bus rows pass |

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | C₂ × C₂ | exact |
| n_v; actuated | 22; 16 | exact |
| Law binding | each end effector exposes exactly two contact bindings (roles `Primary`, law `wheel-rolling`; `Braked`, law `pad`) and one contact switch, with one command channel `brake:<joint id>` per wheel, reset released | exact |
| Braked mode equals a legged quadruped | with all brakes engaged, the contact rows are those of `pad` and the rolling constraint rows are absent: the constraint Jacobian has the same row count as `quad-mammal`'s with patch feet | exact (row counts) |
| Free mode on flat ground at the reference pose (legs fixed) | the four wheel axles are parallel: the planar rolling distribution has rank 1 in (v_x, v_y, ω_z) with the legs frozen (forward only), and reaches full planar mobility only through leg motion (hip abduction steers a wheel) | exact (rank) |
| Contact strata with a mode bit: four end effectors × {open, stick, slip} × {rolling, pad} under C₂ × C₂ | (6⁴ + 3·6²)/4 = 1,404/4 = 351 | count |
| Module reuse | the leg subtree products of `quad-mammal` that do not depend on the foot are reused bit for bit after `replace` (the invalidation set is the end effector and what depends on it) | exact (bits) |

**Control variants.** (a) `variants/incompatible-laws.fbot`: a braked `pad` whose torsion radius exceeds its
patch radius and the wheel's own contact radius (the end effector silently acquires torsional support) → refused
at stage 1 with FROB-REF-121 (contact law missing or incompatible with its end effector; N14bp2-02's typing step
4, with the observed and the admitted value); (b) a build that binds
`wheel-rolling` unconditionally: with brakes engaged the wheel still rolls freely down a 5% slope, caught by a
braked-hold fixture (zero displacement expected under `pad`); (c) `braked` written without `switch` (two laws
and no switching rule) → refused at stage 1 with FROB-REF-121 (typing step 2).

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Stage 1 accepts the law pair with its rule; the incompatible pair and the missing rule are refused with
      FROB-REF-121.
- [ ] Stage 4 returns both mode reports, `released` (wheeled, with its growth vector) and `engaged`
      (`not-applicable`); the strata count with the mode bit is 351.
- [ ] The slope fixtures (braked hold on a 5% slope, free roll, and control (b) caught by the braked hold) are
      present in the expectation file and reported `blocked: milestone` (due M4: the hybrid laws close on every
      tier in BD-365), never passed; NEGp1-53 evaluates them at M4.

## Test plan

### Unit tests

- `zoo_wheel_leg` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_wheel_leg_two_laws_bind`: two bindings per wheel (Primary `wheel-rolling`, Braked `pad`), one switch and
  one channel `brake:<joint id>` per wheel; `zoo_wheel_leg_incompatible_laws_refused_121`;
  `zoo_wheel_leg_missing_rule_refused_121`.
- `zoo_wheel_leg_stage4_per_mode`: the stage record carries the modes `released` and `engaged`, each named,
  with the growth vector in the first and `not-applicable` in the second.
- `zoo_wheel_leg_free_mode_rank_one_legs_frozen`.
- `zoo_wheel_leg_strata_with_mode_bit`: brute force over 6⁴ = 1,296 tuples gives 351 classes.
- `zoo_wheel_leg_reuses_quad_leg_products`: memo hits for the foot-independent leg products.
- `zoo_wheel_leg_slope_rows_not_run_before_m4`: the slope rows load with `blocked = "milestone"`, `due = "M4"`,
  and are never counted as passed. (The simulated `zoo_wheel_leg_braked_holds_on_slope` and `zoo_wheel_leg_free_wheel_rolls`
  are NEGp1-53's.)

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6 with both stage-4 reports, the
two refused variants; log lines carry the law pair, the active law per contact, ranks per mode, the strata
count, the refusal codes, and one line per slope row with `blocked: milestone` and its due milestone. The M4 slope fixtures on
the referee are NEGp1-53's step of the same script.

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
