---
key: "ZOO-snake-crawler"
title: "zoo/snake-crawler.fbot: archetype 17, planar chain with passive wheels: C₂ by reversal, locomotion as holonomy"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "frontier", "test", "zoo"]
parent: "WP-38"
deps: ["BD-054", "BD-055", "BD-056", "BD-063", "BD-243", "N10a-16", "N14b-04", "N14b-11", "N14bp1-01", "NEG-30", "NEG-31"]
needs: []
related: []
gates: []
plan_refs: ["Appendix G.4 row 17", "§14.10", "C-08", "§28.4 A-09", "§13.8", "§11.6", "Appendix M KA-254, KA-270"]
---
## Background

`snake-crawler` is a planar chain of links joined by actuated revolute joints, each link carrying a pair of
passive wheels across its axis. The wheels roll freely along the link and resist motion across it. The robot
has no driven wheel at all: it moves by changing shape, and a cyclic change of shape produces a net
displacement. That is locomotion as holonomy (construction C-08): the wheel constraints define a connection
that maps shape velocity to body velocity, and the displacement produced by a small loop in shape space is
given, to leading order, by the curvature of that connection integrated over the loop. Analysis A-09 computes
the curvature; preflight stage 4 uses curvature, not the growth vector, for this base because it is a
kinematic shape-changer. The archetype is the fixture for one sign that is easy to get wrong (the Lie-bracket
term of the curvature) and for the `serial(k, link)` constructor at scale, which must compile one link and
reuse it. The curvature analysis is Frontier with a paper-scale pilot. No reference robot is on file. It lands
at M3.

## Scope

In: `zoo/snake-crawler.fbot`, its expectation file and controls. Out: the template term, its manifest and its
compile-only checks (N14bp1-01, which owns the term and its defaults); the template's known answers, presets and
preflight stage-4 and stage-6 rows (N14b-04); connection
curvature (BD-243) and its pilot (N10a-16); memoized compilation (BD-063); the chain-crawler controller
(N14b-08); geometric phase for locomotion as an epic (BD-425).

## Technical approach

**Morphology.** A head body (the term's root) followed by `serial(8, link)`, where one `link` is an actuated
revolute joint (vertical axis), a body and a passive wheel pair: nine bodies joined by eight actuated revolute
joints. The head is the link body's capsule exactly (r 0.035 m, length 0.12 m, wheel ports `wl` and `wr` at
mid-length; N14bp1-01's defaults), without which the head-to-tail reversal could not exist. Every body, the head included, carries a passive wheel pair with law `wheel-rolling` (free roll along
the body, no lateral slip). Actuated 8; generalized velocities 3 planar base (the chain lives on the floor
plane; the floating base's other three are held by the wheel contacts) + 8 joints = 11 in the planar model,
14 with the full floating base; bodies 9 plus wheel pairs; end effectors 9 wheel pairs.

DESIGN (one statement, owned by the template-term bead N14bp1-01 and repeated word for word in N14b-04 and
here; a change is a dated amendment of N14bp1-01's file, never a second statement): the archetype table fixes eight actuated
coordinates and the constructor `serial(k, link)`, which repeats `link` k times, so `serial(8, link)` yields
eight links and the ninth body is the root. The `link` subterm is compiled once and served seven memo hits for
its eight repetitions (the counting used for every repeated subterm: n uses are one miss and n − 1 hits, as
four mirrored legs are one expansion and three hits); the head is a different term and is compiled on its
own. Reason: eight joints need nine bodies, and counting the head as a ninth use of the link product would
make the memo row disagree with the constructor's own definition.

**Connection.** Shape variables s ∈ ℝ⁸ (joint angles), body velocity ξ ∈ se(2). Each wheel pair imposes one
Pfaffian constraint (no lateral velocity at the link's wheel axis): W_g(s)·ξ + W_s(s)·ṡ = 0, nine rows. When
three independent rows are chosen (rank W_g = 3 = dim SE(2)), ξ = −𝔸(s)·ṡ with 𝔸 = W_g⁻¹·W_s on those rows;
the remaining six rows constrain the admissible shape velocities (the chain is over-constrained: only gaits
consistent with all wheels are slip-free, the classical serpenoid family). Curvature:
(D𝔸)ᵢⱼ = ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ]; a small loop in shape space moves the body by about exp(−∬ D𝔸 dsᵢ dsⱼ).

**Symmetry.** The catalog's cell, on file: Γ_mech = C₂ under head-to-tail reversal (reverse the link order,
negate the joint angles); default Γ_obs trivial, because the template's default sensor set mounts an IMU on the
head (N14bp1-01) and a head-mounted sensor breaks the reversal; C₂ when a `[[symmetry.breaker]]` edit removes
that IMU. N14bp1-01's hand derivation contests the cell: every body is left-right symmetric, so the sagittal
mirror (fixing every body, exchanging the wheels of each pair, negating every joint angle) is also an element;
Γ_mech is C₂ × C₂ (sagittal mirror, transverse mirror, half-turn) when the compiled chain is palindromic and C₂
generated by the sagittal mirror when it is not (actuator mass carried by each joint's child body breaks the
palindrome); and the head IMU, on the centre line, is fixed by the sagittal mirror, so Γ_obs would be C₂, not
trivial. The question is OPEN in N14bp1-01 (owner: Skeleton lead; options: amend the cell to the computed group,
or stagger each wheel pair along its body so that only the half-turn survives), and its fail-closed default
governs here: the expectation file keeps the cell's values and records every group row (the `[symmetry]` table,
its breaker row, the stage-5 row and the known-answer rows for Γ_mech and Γ_obs) with `blocked = "open"` and
`open = "N14bp1-01"` (NEG-30's field), reported `blocked: open` and never passed, with the computed group and the
presence of each of the three elements logged beside them. Every other row is evaluated. When the question is
decided, a dated amendment writes the decided groups and removes the field.

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 1: one link product compiled
for the eight repetitions (memo graph: one miss, seven hits for the link; the head compiled separately).
Stage 3: chain mobility 11 (planar);
no loops. Stage 4: base kind `shape-changer`; curvature test run (not the growth vector); curvature non-zero
on at least one shape plane at the reference shape, so net motion is possible. Stage 5: `blocked: open`
(N14bp1-01): the check of the shipped assertion (the cell) is logged, accepted or FROB-REF-109 with its first
mismatch, and is neither passed nor counted as a refusal of the defaults until the question is decided.
Stage 6: joint torques to hold a curled shape against wheel side forces inside the envelope.

**Known answers.** Three-link sub-fixture (the first two joints, links of length ℓ = 0.20 m; the kinematic
snake with exactly three wheel constraints, so the connection is unique).

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | C₂ under head-to-tail reversal (the cell; `blocked = "open"`, N14bp1-01) | exact |
| Γ_obs at defaults; without the head IMU | trivial (breaker: the head IMU's mount); C₂ (the cell's reading; `blocked = "open"`, N14bp1-01) | exact |
| Link products compiled for the eight links of `serial(8, link)` | 1 (seven memo hits) | exact |
| Connection rank on the three-link fixture at a generic shape (s₁, s₂) = (0.3, −0.2) rad | 3 | exact |
| Singular shapes of the three-link fixture | the shapes at which the three wheel axles are concurrent or parallel (s₁ = s₂ when both joint angles are measured in the same rotational sense): rank W_g = 2, reported as a singular locus | exact |
| Holonomy against direct integration | for a square loop of side ε in (s₁, s₂) centred at (0.3, −0.2), the predicted displacement exp(−ε²·D𝔸₁₂) agrees with numerically integrating ξ = −𝔸·ṡ around the loop; for a centred loop the cubic term cancels by symmetry, so the relative error is O(ε²) and falls by a factor of about 4 when ε halves from 0.1 to 0.05 rad (for a loop with a corner at the point it is O(ε) and the factor is about 2; the test uses the centred loop) | ratio within [3, 5] |
| Sign of the bracket term | with the sign of [𝔸ᵢ, 𝔸ⱼ] flipped the holonomy test fails by an error of order one relative to the displacement | error ratio > 0.5 |
| Reversal covariance | running the loop backwards reverses the displacement to leading order (group inverse) | rel 10⁻⁶ against the inverse |

**Control variants.** (a) the bracket-sign mutant, as above; (b) a growth-vector test used on this base in
place of curvature: stage 4's dispatch rejects it (curvature is the right test for a shape-changer and the
wrong one for a wheel layout); (c) `serial` implemented by copying the link eight times: eight compile misses
in the memo graph where one is required; (d) a wheel that slips or lifts in the motion: the curvature prediction
is withdrawn for that motion, and the analysis must say so rather than report a displacement.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] One link product is compiled for the eight links (one miss, seven hits); stage 4 runs curvature and
      reports it non-zero.
- [ ] The holonomy rows hold on the three-link fixture and the sign-flipped mutant fails; the four controls
      behave as stated.
- [ ] Until N14bp1-01's open question is decided, the group rows (Γ_mech; Γ_obs at defaults and without the
      IMU; the breaker row; the stage-5 report) are recorded `blocked: open` naming N14bp1-01, never passed, with
      the computed group logged beside them; no other row is blocked.

## Test plan

### Unit tests

- `zoo_snake_crawler` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_snake_serial_compiles_once`.
- `zoo_snake_three_link_connection_rank`, `zoo_snake_three_link_singular_locus`.
- `zoo_snake_holonomy_vs_integration`: centred loops of ε = 0.1 and 0.05 rad; error ratio in [3, 5].
- `zoo_snake_curvature_sign_mutant_fails`.
- `zoo_snake_reverse_loop_inverse`.
- `zoo_snake_gamma_obs_head_imu`: while the question is open, the `[symmetry]` table, the breaker row, the
  stage-5 row and the two Γ known-answer rows load with `blocked = "open"` and `open = "N14bp1-01"`, are reported
  `blocked: open` with the computed Γ_mech, Γ_obs and the three elements' presence logged, and count neither as
  passed nor as failed. After the decision the rows are evaluated as amended (the cell kept
  with staggered wheels: trivial with the head IMU, C₂ without it, the IMU's mount named as breaker; the cell
  amended: C₂ × C₂, and C₂ generated by the sagittal mirror with the head IMU).

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6 with the stage-4 curvature
report; log lines carry the memo hit count, the curvature norm at the reference shape, the predicted and
integrated displacements for both loop sizes and their error ratio, and for each group row `blocked: open` with
N14bp1-01 named and the computed group with each element present or absent.

### Known answers and negative controls

KA-254 (the curvature law and the direction of the small-loop displacement; control: the bracket sign
reversed); KA-270 (Γ_mech: C₂ under head-to-tail reversal). Determinism: closed forms and a fixed-step
integration, D0 on one machine.

## Considerations

- The connection belongs to one contact stratum (all wheels rolling without slip). The nine-body chain is
  over-constrained kinematically; which gaits are admissible without slip is part of the analysis report, and
  dynamic simulation with the roller law decides the rest.
- Which three rows define the connection on the full chain is a choice the analysis records; the three-link
  fixture avoids the choice and is the known answer.

## References

Appendix G.4 row 17; §14.10; C-08; A-09; §13.8; §11.6; KA-254, KA-270.
