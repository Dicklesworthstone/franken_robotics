---
key: "ZOO-mobile-manipulator"
title: "zoo/mobile-manipulator.fbot: archetype 16, seven-joint arm on a wheeled base: trivial symmetry, workspace"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-04", "BD-054", "BD-055", "BD-056", "BD-105", "BD-102", "BD-104", "ZOO-diff-drive"]
needs: []
related: ["ROBOT-iiwa7", "ROBOT-stretch3", "ROBOT-tiago", "N14b-09", "N14b-12", "BD-244", "BD-246", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 16", "§14.10", "§28.4 A-05, A-06", "§14.11 (stage 7)", "Appendix M KA-270"]
---
## Background

`mobile-manipulator` is a seven-joint arm mounted off the centre line of a differential-drive base. It is the
archetype for everything about reaching: the workspace of a redundant arm (A-05), its singular configurations
(A-06), whether the arm reaches the heights a task names (tray and shelf heights are inputs of preflight
stage 7), and self-collision between arm and base. It is also the catalogue's deliberately asymmetric body: an
arm off the centre line leaves no mirror, so its group is trivial, and a payload held to one side is a
symmetry breaker even when the arm is centred. Its references are the KUKA iiwa 7 (a builder exists in the
sibling physics library) and the Stretch 3 and TIAGo mobile manipulators (Apache-2.0 models). It lands at M3;
its arm-on-a-base controller is Frontier.

## Scope

In: `zoo/mobile-manipulator.fbot`, its expectation file and controls. Out: the template and its presets
(N14b-04); workspace, singularities and the derived inverse-kinematics kernel (BD-105); self-collision
(BD-104); stage 7 (N14b-12, BD-244); the controller (N14b-09); robot imports (ROBOT-iiwa7, ROBOT-stretch3,
ROBOT-tiago).

## Technical approach

**Morphology.** Two driven wheels and a caster (the `diff-drive` base); a seven-revolute arm with the
shoulder-elbow-wrist (S-R-S) layout mounted at a lateral offset; a flange end effector with a declared payload
port. Actuated 9 (2 wheels, 7 arm joints); n_v = 17 (floating base 6, 2 wheel spins, caster swivel and roll,
7 arm joints); bodies 12; end effectors: 3 wheels and the flange.

**Symmetry.** Γ_mech as built: trivial with the arm off-centre. Variant with the arm on the centre line and at
a mirror-symmetric pose: C₂; then a 2 kg payload held at y = +0.1 m is named as the breaker.

**Arm facts used by the rows (DESIGN statements checked by the analyses, not asserted from memory).** A
7-revolute S-R-S arm with upper-arm length d₁ and forearm length d₂ has a wrist-centre workspace that is the
spherical shell d_min ≤ ‖p − shoulder‖ ≤ d₁ + d₂, with d_min set by the elbow limit; the outer boundary
(elbow straight) is a singular locus where the Jacobian drops rank by one; shoulder singularity when the
wrist centre lies on the first axis; wrist singularity when the last three axes become coplanar (the fifth
and seventh axes aligned). One degree of redundancy (the elbow swivel) is the null space of the 6 × 7 Jacobian
at regular configurations.

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 3: tree mobility 17; Jacobian
rank 6 at the reference pose with a one-dimensional null space; self-collision pairs arm–base listed with
clearances. Stage 4: base kind differential drive, growth vector (2, 3). Stage 5: Γ_mech trivial; no
assertion. Stage 6: static tip-over margin with the arm fully extended sideways holding the rated payload
(the worst static case) positive; shoulder torque for that pose inside the envelope. Stage 7 (with a task that
names tray height 0.75 m and shelf heights 0.40 m and 1.50 m): reach verdict per height with the base
standing at the declared approach distance.

**Known answers.** Fixture: d₁ = d₂ = 0.40 m; shoulder at height 0.60 m above the floor; base half-track
0.20 m; arm mount offset 0.10 m; payload 2 kg at the flange.

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | trivial (order 1) | exact |
| n_v; actuated | 17; 9 | exact |
| Jacobian rank at the reference pose; null-space dimension | 6; 1 | exact |
| Outer reach of the wrist centre | d₁ + d₂ = 0.80 m | rel 10⁻⁹ |
| Rank at full extension | 5 (singular) | exact |
| Highest reachable wrist-centre height | 0.60 + 0.80 = 1.40 m: a shelf at 1.50 m is out of reach by 0.10 m (before the flange length is added) | rel 10⁻⁹ |
| Tip-over with the arm horizontal sideways: moment of the payload about the nearer wheel line | 2·9.81·(0.80 + 0.10 − 0.20) = 13.73 N·m, against the restoring moment (base weight)·0.20 m | rel 10⁻³ |
| Payload breaker, centred-arm variant | the payload at y = +0.1 m is named; Γ_mech trivial with it, C₂ without | exact |

**Control variants.** (a) the centred-arm variant with the payload and `assert_mech: C2` → FROB-REF-109 with
the payload as breaker; (b) a stage-7 task that requires the 1.50 m shelf: refused with FROB-REF-119 (no
admissible route or reach in the scene) naming the height and the lever that would fix it (a taller mast or a
longer forearm); (c) a workspace routine that reports the outer sphere only (ignores the inner radius from the
elbow limit): the row "points inside d_min are unreachable" catches it.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Rank, null-space, reach and tip-over rows hold; self-collision pairs are listed.
- [ ] The three controls behave as stated; stage-7 rows are `not run` until stage 7 lands and then pass.

## Test plan

### Unit tests

- `zoo_mobile_manipulator` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_mm_jacobian_rank_and_nullspace`; `zoo_mm_rank_drops_at_full_extension`.
- `zoo_mm_reach_heights`: 0.40 m and 0.75 m reachable, 1.50 m not, at the declared approach distance.
- `zoo_mm_tip_moment`: 13.73 N·m.
- `zoo_mm_payload_breaker_named`.
- `zoo_mm_inner_radius_unreachable`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry (off-centre and centred variants), stages 1 to
6, and stage 7 with the `synthetic` shelf task and scene; log lines carry rank, reach in metres per height,
the tip-over margin, the refusal code and its repair.

### Known answers and negative controls

KA-270 (Γ_mech: trivial with the arm off-centre). Controls as listed. Determinism: exact rank at rational
sample configurations; closed-form reach.

## Considerations

- The fixture's link lengths are illustrative and belong to the zoo's fixture parameters; presets bind the
  template to the iiwa 7 on a Stretch 3 or TIAGo base for the mass rows of the catalogue gate, and those
  robots' true dimensions come from their imported models.
- A mobile base turns the arm's workspace into a swept region; stage 7's verdict depends on where the base may
  stand, which the scene's clearance complex decides.

## References

Appendix G.4 row 16; §14.10; §14.11; A-05; A-06; KA-270; FROB-REF-109, FROB-REF-119.
