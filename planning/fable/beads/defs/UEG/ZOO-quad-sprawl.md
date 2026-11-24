---
key: "ZOO-quad-sprawl"
title: "zoo/quad-sprawl.fbot: archetype 2, sprawled quadruped: stable on a wide stance, refused thermally at `@hot`"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["BD-054", "BD-055", "BD-056", "BD-072", "BD-084", "BD-100", "BD-103", "N14b-02", "NEG-30", "NEG-31"]
needs: []
related: []
gates: []
plan_refs: ["Appendix G.4 row 2", "§14.10", "§15.7", "§15.6", "§28.4 A-08, A-11", "Appendix M KA-270"]
---
## Background

`quad-sprawl` is a quadruped whose legs splay outward on a wide stance. It exists to separate two notions that
are easy to conflate: static stability and actuator feasibility. A wide stance has a large support polygon and
a low body, so its tip-over margin is large. It also puts a long horizontal lever between each foot and its
hip, so merely standing demands a high holding torque, and holding torque is copper loss. The archetype's
purpose is a design that the stability analysis (A-08) passes and the actuator envelope analysis (A-11)
refuses thermally, so that an agent or a screen that looks only at the support polygon is shown to be wrong.
It also exercises stiction at joints that sit near stall. No permissively licensed reference robot is on file;
its baseline scorecard is its own template. It lands at M3.

## Scope

In: `zoo/quad-sprawl.fbot`, its expectation file, the `@hot` variant and the stiction fixture. Out: the
template (N14b-02); the winding thermal model (BD-072); A-08 (BD-100) and A-10, A-11 (BD-103); joint stiction
rows (BD-084).

## Technical approach

**Morphology.** Torso; `mirror(xz, mirror(yz, leg))`; three-revolute legs splayed outward; foot end effectors
with law `foot-patch`. Actuated 12; n_v = 18; bodies 13; end effectors 4. Γ_mech as built: C₂ × C₂; default
Γ_obs: C₂ × C₂.

**The mechanism under test.** For body mass m shared by four feet, a joint whose horizontal moment arm to its
foot is ℓ holds τ = (m·g/4)·ℓ. Winding current I = τ/(N·k_t) for gear ratio N and torque constant k_t. Copper
loss P = R(T)·I² with R(T) = R₀·(1 + α·(T − T₀)). With thermal resistance R_th to ambient the steady state is
T* = T_amb + R_th·R₀·I²·(1 + α·(T_amb − T₀))/(1 − α·R_th·R₀·I²), which exists only if α·R_th·R₀·I² < 1 (the
same condition as s < G with s = I²·R₀·α and G = 1/R_th).

**What preflight must report.**

| Stage | Default instance | Variant `@hot` |
|---|---|---|
| 1 to 3 | `passed` (13 fills, 4 foot laws, 12 revolute joints, no loops) | `passed` |
| 4 Motion | `passed` with the note `not-applicable` (legged, N14b-11) | `passed` with the same note |
| 5 Structure | `passed`; Γ_mech = Γ_obs = C₂ × C₂ | `passed` |
| 6 Statics and sizing | `passed`; A-08 margin positive and larger than `quad-mammal`'s at equal mass; holding torque, current and T* reported per joint, T* below the card's limit | `refused`: A-08 margin still positive; refused with FROB-REF-132 by A-11 (no thermal equilibrium, or one above the limit) with the knee joints named and the repairs "narrower stance", "higher gear ratio", "larger actuator card" as typed edits |

`@hot` is the default with a longer horizontal lever at the knee and a smaller actuator card; its values are
read from the card by the test, which checks the inequality rather than a hard-coded temperature.

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | C₂ × C₂ | exact |
| Thermal formula on a `synthetic` card (R₀ = 0.2 Ω, α = 0.0039 K⁻¹, R_th = 2 K/W, T₀ = T_amb = 25 °C) at 10 A | α·R_th·R₀·I² = 0.156; T* = 25 + 40/(1 − 0.156) = 72.39 °C | rel 10⁻⁹ |
| Same card at 26 A | 1.0546 ≥ 1: no equilibrium | exact (verdict) |
| Holding torque per joint | (m·g/4)·ℓ from the compiled mass and the compiled lever, equal on mirrored joints | rel 10⁻¹² against the closed form |
| Stiction near stall | commanded torque at 0.9 of breakaway: zero velocity for 1 s, joint row state `stick` | exact (state) |
| Contact strata | 27 (same group action as `quad-mammal`) | count |

**Control variants.** (a) `@hot`, which a support-polygon-plus-total-capacity screen accepts and stage 6 must
refuse; (b) a build with the constant-resistance thermal model (α = 0), under which `@hot` passes: the test
asserts the refusal depends on R(T); (c) stiction disabled: the near-stall joint creeps, and the test asserts
`stick`.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Default passes stages 1 to 6 with no refusal; `@hot` has a positive A-08 margin and is refused with
      FROB-REF-132.
- [ ] The thermal formula rows and the stiction row hold; the three controls behave as stated.

## Test plan

### Unit tests

- `zoo_quad_sprawl` (generated): identity, morphology, symmetry, stages, known answers.
- `zoo_quad_sprawl_hot_refused_132_with_positive_margin`.
- `zoo_quad_sprawl_thermal_formula`: 72.39 °C at 10 A; no equilibrium at 26 A.
- `zoo_quad_sprawl_margin_exceeds_mammal`: at equal total mass the sprawl's static margin is larger.
- `zoo_quad_sprawl_stiction_sticks`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31): the default through stages 1 to 6 and `@hot` through stage 6, asserting
the refusal code, the named joints and the repairs; log lines carry margins in metres, torques in N·m,
currents in A and temperatures in °C against the card's limits.

### Known answers and negative controls

KA-270 (Γ_mech as built: C₂ × C₂). The thermal rows follow the winding known answers of the actuator chapter
(equilibrium P/(G − s), runaway at s ≥ G). Controls: the weak screen that accepts `@hot`; the constant-
resistance model. Determinism: static rows D0; the stiction row within the referee's class.

## Considerations

- The archetype table says the design "is stable and still fails A-11 thermally" while the catalogue gate says
  every archetype passes stages 1 to 6. Resolution carried from the template bead: the default passes and the
  named variant is the failing fixture.
- The thermal initial condition matters: the row is evaluated at `steady(task)` for a standing task, not from
  cold.

## References

Appendix G.4 row 2; §14.10; §15.6; §15.7; A-08; A-11; KA-270; FROB-REF-132.
