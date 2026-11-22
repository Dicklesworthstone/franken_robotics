---
key: "N13-CX"
title: "Counterexample registry for Marrow: CX-116 to CX-124 and the turn-count case as negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "marrow", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: []
related: ["N13-KA", "N13-18", "BD-011", "BD-047", "BD-325", "BD-182", "N13-02", "N13-10", "N13-14", "N13-15", "N13-16", "N13-34", "N13-36", "BD-146"]
gates: []
plan_refs: ["Appendix N.10", "§13", "§13.11", "§38.11", "§0.7"]
---
## Background
A plausible shortcut ships with the smallest hand-checkable case that breaks it, and the case becomes a permanent negative control: a planted defect that a gate must catch. The negative-control registry (`frob-conformance/registry/controls.toml`) is seeded from the counterexample catalogue: each entry becomes at least one control, attached to the gates of the rule it protects. A gate with no control in the registry cannot record a pass, a control is run every time its gate is run, and a gate run in which a control was not caught is a failure whatever the passing test says.

This bead is the catalogue for the algebraic core: the nine entries CX-116 to CX-124, plus the counterexample of the periodic-and-lifted rule, which was stated with its subsection and has no number yet. Tier of the content: Solid.

## Scope
In: the catalogue entries as registry records; one or more control records per entry with gates; the completeness check; a number for the late counterexample from the registry's owner.
Out: the tests that run the controls (owners listed); the mutation matrix (N13-18), which uses several of these.

## Technical approach
**Record form** (per control): `id`, `seed` (catalogue entry), `gate`, `kind` (`mutation`, `planted-input`, `planted-design`, `planted-world`), `target`, `defect`, `expect`, `last_seen`.
**The entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control: kind, target, expected observation | Gates | Owner |
|---|---|---|---|---|---|---|
| CX-116 | A free-body integrator that conserves kinetic energy has the right coadjoint sign (equivalently: ad* is "the negative transpose" of ad) | Inertia diag(1, 2, 3) kg·m², ω = (1, 1, 0) rad/s, π = (1, 2, 0). Correct law: π̇ = π × ω = (0, 0, −1). Reversed sign: (0, 0, +1). Both give ω·π̇ = 0, so kinetic energy is conserved either way. The body-frame rate of spatial angular momentum π̇ + ω × π is (0, 0, 0) with the correct sign and (0, 0, 2) with the wrong one | Two coadjoints, each with its own type and sign (ADR-MARROW-05); the free-body and coadjoint-pair laws; the mutation rule (each convention guarded by at least three fixtures) | mutation of `coad`; π̇ = (0, 0, +1), spatial rate (0, 0, 2); the energy check stays green (documented blind fixture) | GATE-MARROW-01, GATE-MARROW-04 | BD-011, N13-18 |
| CX-117 | The lower three rows of a space screw Jacobian are the point Jacobian of the tool | Planar 2R arm, links 0.4 m and 0.3 m, angles 0 and π/2, tool force (10, 0) N. The point Jacobian is [[−0.3, −0.3], [0.4, 0.0]] m/rad and gives joint torques (−3, −3) N·m; the lower rows of the space screw Jacobian give (0, 0) | Frame-typed Jacobians and wrench maps (`WrenchPushforward` against `WrenchPullback`); the planar 2R known answer | mutation at a use site; torques (0, 0) instead of (−3, −3) | GATE-MARROW-01 | BD-047 |
| CX-118 | A mechanically symmetric robot may tie policy weights, augment data or deduplicate designs under its mechanical group | A quadruped built fore-aft and left-right symmetric has Γ_mech = C₂ × C₂ of order 4. One forward lidar on the torso is fixed by the left-right mirror and moved by the fore-aft mirror and the half-turn, so Γ_obs = C₂ of order 2. A policy tied under C₂ × C₂ must walk backwards as the mirror image of walking forwards while seeing only forwards | Two groups per design and task (ADR-MARROW-06); PO-MARROW-04; a false declaration is refused (FROB-REF-109) | mutation: the Γ_obs filter returns Γ_mech; order 4 where 2 is true; and planted-design: the lidar quadruped declaring C₂ × C₂ | GATE-MARROW-03 | N13-16, N13-21 |
| CX-119 | A failed search over symmetric (tied, deterministic) policies shows the task is infeasible | Two identical latches, of which exactly one must be closed. At an observation fixed by the mirror a deterministic equivariant controller must issue an action fixed by the mirror, so it commands both latches alike, and both matching commands fail. A controller that draws one latch at random is equivariant in law and succeeds | A failed search excludes only the policy class searched; fixed subspaces are exposed so the analysis lists the observations at which tying forces tied actions | planted-design: the two-latch fixture; the report must say "tied deterministic class excluded" and nothing more; a report of "infeasible" is the defect | GATE-MARROW-03 (fixed-subspace row), the symmetry analysis's gate | N13-16 |
| CX-120 | A task that is symmetric in law may be reduced by symmetry inside each world | A quadruped crossing a patch that is slippery on one side only. The world distribution can be mirror-invariant in law, which licenses augmentation; no single world is mirror-invariant, which forbids reduction inside it | Reduction inside one world uses the stabilizer of the context, not Γ_obs | planted-world: the one-sided patch; `stabilizer` must be trivial; a reduction that uses Γ_obs is the defect | GATE-MARROW-03, the controller-symmetry gate | N13-16 |
| CX-121 | If the encoder permutation of a symmetry still exists, the symmetry still holds | Changing one foot's sole material leaves the encoder permutation intact and breaks the transition symmetry: the representation exists and the group element does not | Γ_mech preserves contact laws and interface cards; both facts are recorded; a changed sole material is among the planted asymmetries | planted-design: one sole material changed; the element must leave Γ_mech with the interface card as first mismatch | GATE-MARROW-03 | N13-15, N13-16, N13-21 |
| CX-122 | Every irreducible block of an equivariant layer takes one real scalar per pair of copies | C₃ permuting three legs cyclically. The equivariant maps ℝ³ → ℝ³ are the circulants, three parameters: the trivial block plus one two-dimensional block of complex type, 1 + 2 = 3. Giving the two-dimensional block one scalar yields two parameters and only the symmetric circulants, which are the D₃-equivariant maps; x ↦ (x₂ − x₃, x₃ − x₁, x₁ − x₂) becomes unrepresentable | Real and complex blocks with the Reynolds operator as oracle (ADR-MARROW-07); PO-MARROW-05; the blocks law | mutation: complex block counted as real; count 2 against Reynolds rank 3 | GATE-MARROW-04, GATE-MARROW-02 (typing) | N13-14, N13-23, BD-325 |
| CX-123 | A strict inequality that selects a discrete outcome can be evaluated in plain single precision | The single-precision numbers nearest 0.3, 0.4 and 0.5 in x² + y² ≤ z². In single precision the two sides are equal, so the plain test reports the point inside the cone. The exact value of x² + y² − z² for those stored numbers is +1.19 × 10⁻⁸: the point is outside. A fused multiply-add gives a third rounding sequence | Exact evaluation of strict predicates (ADR-MARROW-08); the `Guarded` class with its exact integer fallback; the lint against `Plain` predicates on selects whose arms differ; GATE-MARROW-05 | mutation: the guard removed (a `Plain` kernel); reports "inside" | GATE-MARROW-05, GATE-MARROW-01 (Predicates row) | N13-10, BD-182 |
| CX-124 | A mirrored revolute joint can be written with the mirrored axis P·a and keep its coordinate | For a reflection (det P = −1) an axis is an axial vector and maps to det(P)·P·a. A design that writes P·a instead must negate the coordinate, its limits, and its encoder and gear signs; hiding the sign inside a rigid transform is how mirrored encoders read backwards | A reflection is an `Isometry` with an explicit determinant, not a `Transform` (ADR-MARROW-09); the reflections law; the planted mutation "a missing parity sign on a mirrored axis" | mutation of the parity rule; the mirrored-joint law fails; joint sign +1 where −1 is true | GATE-MARROW-01, GATE-MARROW-04 | N13-02 |
| late (number to assign) | A revolute joint's state is its unit complex number: two states with the same pose and velocity are the same state | A joint with a linear torsion spring of 0.5 N·m/rad, rest angle 0, held stationary at θ = 0 and at θ = 2π: identical link poses and zero velocity; at 2π the spring stores ½ × 0.5 × (2π)² = π² J (9.8696 J) and exerts −π N·m. A replay from a state without the turn count creates or loses π² J | A law is `periodic` or `lifted`; the turn count is state wherever a lifted law reads the joint | mutation: the snapshot omits k; an energy jump of π² J at restore | GATE-MARROW-06 | N13-34, N13-35, N13-36, N13-37 |

**Related entries registered by other chapters and exercised by this chapter's tests** (listed so the cross-links are not lost; not counted here): the construction-level entries "move every spatial quantity with one adjoint" (a single reversed adjoint must fail three independent fixtures), "fill every enclosure solid" (64.8 kg against 3.45 kg, a factor of 18.8), "use the Cayley update as if it were the exponential" (2·atan(‖ω‖h/2) instead of ‖ω‖h; bounded by ‖ω‖h ≤ 0.2 and refused beyond, FROB-REF-212), "the curvature with +[𝔸ᵢ, 𝔸ⱼ]", "angular velocity and torque as polar vectors under a mirror", and "decide a strict predicate in unguarded single precision" (452 false admissions and 23,438 false exclusions on a 65,536-case screen).
**Completeness check.** A meta-test fails if one of the nine catalogue ids (or the late entry) has no control, if a control names no gate, or if its target test does not exist.

## Acceptance criteria
- [ ] Nine catalogue entries and the late turn-count entry are each represented by at least one control record with every field filled; the late entry has a number from the registry's owner or a named slot.
- [ ] Each control runs with its gates; a gate run in which its control is not caught fails.
- [ ] Every control has `last_seen.caught = true` from a recorded run before the exit review of the milestone that owns its gate (M1 for CX-116, CX-117, CX-124 and the late entry's referee rows; M2 for CX-123 on devices; M3 for CX-118 to CX-121 complete; M4 for CX-122).
- [ ] The meta-test passes; the chapter's count is 9 catalogue entries, matching the catalogue's coverage table, plus 1 late entry.

## Test plan
### Unit tests
`cx_registry_marrow_complete`; `cx_controls_have_gates_and_targets`; `cx_controls_fire` (each control, when active, makes its named test fail by a value assertion with the observation in the table); `cx_blind_fixture_documented` (CX-116's energy check stays green under the mutation, and the registry says so).
### End-to-end script and logging
Each gate script named above runs its controls and logs, per `STD-E2E`, one `assert` line per control (id, seed, gate, caught, observed, expected). `scripts/e2e/marrow_ka_registry.sh --controls` (shared with N13-KA) reads the latest logs under `target/e2e-logs/` and reports per control the last run in which it was caught, writing `target/e2e-logs/marrow_ka_registry/<utc>.ndjson` with a summary of controls caught, not caught and never run.
### Known answers and negative controls
The numbers in the table: (0, 0, −1) against (0, 0, +1); (−3, −3) against (0, 0); orders 4 and 2; 3 against 2 parameters; +1.19 × 10⁻⁸ against equality; π² J. Determinism: exact.

## Considerations
- A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each milestone exit: either the defect cannot be planted as described or the gate is not looking.
- CX-119 and CX-120 are controls on reports and uses, not on arithmetic; their tests assert what the tool says and which group it lets a caller use.
- A false result found in use that traces to one of these rules becomes a permanent control with the case that exposed it.

## References
Appendix N.10 (CX-116 to CX-124), §13.3 to §13.7, §13.11, §38.11, §0.7.
