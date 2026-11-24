---
key: "SPEC-C-09"
title: "frob-algebra: C-09 specified — mechanical and observation-preserving symmetry groups, parity, three uses"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "marrow", "frob-algebra", "solid", "frontier", "spine", "spec", "test"]
parent: "WP-03"
deps: ["BD-025", "N10-CX", "N10-KA1", "N10a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-048", "BD-102", "BD-241", "BD-326", "N10a-10", "N13-18", "SPEC-C-12", "SPEC-C-16", "SPEC-C-22", "SPEC-C-23", "SPEC-C-24"]
gates: ["GATE-MARROW-03", "GATE-MARROW-04", "GATE-REFLEX-07", "GATE-CORTEX-03"]
plan_refs: ["§10 C-09", "§9 D-14", "§11.4", "§13.2", "§13.4", "§13.8", "§13.10", "§12.3 T-10 T-16", "§38.7", "Appendix D.5", "Appendix E GB-02", "Appendix M KA-066 KA-068 to KA-071", "Appendix N CX-031 to CX-033"]
---
## Background

Construction C-09 says what symmetry a robot has and what that symmetry may be used for. It replaces
hand-written mirror functions with sign tables per robot. Its central point is that a morphology has **two**
symmetry groups, not one, and that using the larger where the smaller is required produces confident wrong
answers.

- The **mechanical group** Γ_mech preserves the body.
- The **observation-preserving group** Γ_obs ≤ Γ_mech also preserves everything the closed loop can see or is
  asked to do.

Decision ADR-MARROW-06: both groups are computed and stored for every design in every task, and only Γ_obs may
tie policy weights, augment data, deduplicate candidates or enumerate gaits. Reason: a robot with a
forward-facing sensor is mechanically symmetric fore and aft and is not symmetric in closed loop; tying
weights under the larger group constrains the policy to something false. Rejected alternatives: one group
declared by the author (a declaration is an assertion to check, never a source of truth); hand-maintained
mirror tables (they flip all three axis signs or none, and both are wrong).

Doctrine rule D-14 is the general form: a symmetry of the mechanism is intersected with the stabilizers of
payload, gravity, task, observation and contact mode before it reduces any calculation; augmenting data with a
transformation that preserves a distribution and reducing one world by a transformation that preserves that
world are different operations with different preconditions.

Tier: Solid for detection of both groups, augmentation and metamorphic tests; Frontier for equivariant
networks under Γ_obs. The Frontier pilot is the three-arm experiment (seed bead BD-326): a policy equivariant
under Γ_obs conditioned on transformed terrain, an augmented ordinary policy, and an unaugmented baseline, on
one quadruped family, ranked separately on sample efficiency, compute and out-of-distribution return.
Falsifier: the equivariant arm does not beat the augmented arm beyond seed spread. Ceiling: a factor |Γ_obs|
in data, of which augmentation already collects most. Fallback: augmentation alone (kill gate KILL-EQUIV).

Foundations (FACT, 2026-11-21): FrankenSymPy has no group theory in Rust; FrankenLean's re-check accepted none
of Mathlib's 43 representation-theory modules; `fs-symmetry` is about 480 lines covering cyclic groups. The
groups here have at most a few dozen elements, so closure, conjugacy classes, characters and projectors are
built in `frob-algebra`.

This bead specifies the groups, the parity rule, equivariance of the dynamics and the three uses. The
representation theory (isotypic components, the Reynolds operator, real and complex blocks) is specified in
N10a-10.

## Scope

In: `docs/math/C-09.md` (parts 1 to 4 of the construction: groups, parity representations, equivariance, uses
and limits); the law suite `crates/frob-algebra/tests/laws_c09.rs`; fixtures: front-lidar quadruped, sagittal
mirror parity, two latches, one-sided payload; the register row; tolerances and controls for KA-066 and KA-068
to KA-071; the statement of T-10.

Out: the production symmetry computation and its verb (BD-102, BD-241); representation theory (N10a-10); the
equivariant layers (BD-325) and the three-arm experiment (BD-326); symmetric controllers (BD-327); orbit
deduplication (construction C-22, another unit); gait enumeration (SPEC-C-10).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-048 (finite groups and
characters in `frob-algebra`): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-algebra` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**Definitions.**

- Γ_mech: the set of automorphisms of the labeled kinematic tree that are realized by an isometry of the body
  and preserve link lengths, pseudo-inertias, joint screws, actuator cards, gear ratios and gear directions.
- Γ_obs ≤ Γ_mech: the subgroup that also preserves every sensor pose and field of view, the payload, the task
  goal and its orientation, gravity, and the terrain and guard structure the task declares.
- For one particular world, the stabilizer of the context, Stab(context) ≤ Γ_obs, whose elements also fix that
  world's terrain, material field, boundary conditions and current contact mode.
- Candidate automorphisms are found by colouring the part graph (part type, side-specific limits, gear
  direction, material), and each generator is then checked on transforms, inertias and channels.

**Parity.** Under an isometry x ↦ P·x + t with orthogonal part P: polar vectors (positions, linear velocities,
forces) map by P; axial vectors (angular velocities, torques, angular momenta, a joint axis read as a rotation
axis) map by det(P)·P. So a mirrored limb can need a sign change at its encoder. A revolute joint about axis a
maps to a revolute joint about det(P)·P·a with the same coordinate; a design that writes the mirrored axis as
P·a instead must negate the coordinate, its limits, and its encoder and gear signs. The representations ρ on
configuration, velocity, action and observation spaces are **signed permutations computed from that rule**, not
tables.

**Equivariance of the dynamics.** With f the step map on state x and action u:
f(ρ_X(g)·x, ρ_U(g)·u) = ρ_X(g)·f(x, u). It holds under Γ_mech when gravity and terrain are transformed with the
robot, and under the elements that fix them otherwise. For the mass matrix: ρ(g)·M(q)·ρ(g)ᵀ = M(ρ(g)·q).

**Typical groups.** C₂ for a biped; C₂ × C₂ for a quadruped built symmetric fore-aft and left-right; a
dihedral group for a radially symmetric hexapod. A quadruped with a front-only lidar has Γ_mech = C₂ × C₂ and
Γ_obs = C₂.

**Three uses, three preconditions.** Both groups are computed from the design (analysis A-03).

1. *Metamorphic tests.* One test of the entire simulator per generator of Γ_mech, applied to robot, terrain,
   gravity and sensors together.
2. *Distributional augmentation.* A transformed trajectory is added to training data whenever the
   transformation preserves the distribution of worlds and tasks. It may pair two different worlds.
3. *Exact reduction.* Invariant critics inside one world, orbit deduplication, closed-loop block-diagonal
   linearizations and gait orbits use only Γ_obs, intersected with the stabilizers of the current payload,
   boundary conditions and contact mode. Weight tying needs Γ_obs and invariance in law of the world and
   command distributions; block-diagonal mass and stiffness matrices need only Γ_mech ∩ Stab(configuration).
   Reduction inside one world needs invariance of that world.

| Use | Group required | Why |
|---|---|---|
| Block-diagonal mass and stiffness matrices at a symmetric stance | Γ_mech ∩ Stab(configuration) | Mechanics only; sensors are irrelevant |
| Weight tying in policies and critics; data augmentation | Γ_obs, with world and command distributions invariant in law | The closed loop must be symmetric, not only the body |
| Deduplicating candidate designs before a sweep | Γ_obs of the slot arrangement | Two candidates are the same experiment only if the closed loop cannot tell them apart |
| Gait enumeration by (H, K) pairs | Γ_obs ∩ Stab(command) for the body; a declared clock-network group for phase patterns | A gait is a closed-loop object |
| Exact reduction inside one world: symmetric orbit search, merging two worlds | Stab(context) | Augmentation may pair two different worlds; reduction needs the same world |
| Metamorphic test of the simulator | Γ_mech with the world transformed along with the robot | The test transforms everything, so nothing is broken |

Observations declared invariant under world yaw and horizontal translation define a quotient decision process,
and a metamorphic test checks that declaration against the full state including terrain. Design constructors
`mirror` and `radial` produce symmetric designs with known generators.

**Honest limits.**

1. A linearized operator that commutes with the action preserves the isotypic blocks; an equivariant nonlinear
   map need not decouple into independent nonlinear blocks.
2. Hard equivariance is not free. FACT (source register, to be re-read from the primary source before a gate
   cites it): augmentation makes an ordinary network about as data-efficient as an equivariant one, and hard
   constraints are more fragile under distribution shift.
3. A deterministic equivariant policy cannot choose between symmetric alternatives, so a failure of
   tied-policy search excludes only the tied policy class.
4. A quadruped crossing a one-sided slippery patch must be allowed asymmetric actions: the world distribution
   may be mirror-invariant in law (which licenses augmentation) while no single world is mirror-invariant
   (which forbids reduction inside it).
5. A material edit to one foot leaves the encoder permutation intact and breaks the transition symmetry: the
   representation still exists, the group element does not.

**Retirement.** When a physical or sensing asymmetry leaves Γ_obs trivial for a design and task, tying is off
and augmentation alone remains.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-09.S1 | A model's derived step kernels are equivariant under each generator of Γ_mech, with gravity and terrain transformed by the same isometry (T-10) | Certified for branch-free rational kernels; Tested for kernels with data-dependent selects | One polynomial identity per generator, each square root replaced by a witness t with t² = radicand, one identity per select branch under its guard; exact-rational randomized identity testing with a Schwartz–Zippel bound, recorded degree bound and sample set; otherwise a round-off-level residual test |
| C-09.S2 | ρ(g)·M(q)·ρ(g)ᵀ = M(ρ(g)·q) | Tested | Property test |
| C-09.S3 | Under the sagittal mirror P = diag(1, −1, 1) the axial image of a joint axis a is −P·a: a pitch axis along y keeps its sign; roll and yaw axes change theirs | Certified | Exact arithmetic |
| C-09.S4 | Γ_obs is a subgroup of Γ_mech; every element of Γ_obs has a representation on every task term; removing every sensor, payload and task term gives Γ_obs = Γ_mech | Tested | Property test over the archetypes |
| C-09.S5 | Front-only lidar quadruped: Γ_mech = C₂ × C₂, Γ_obs = C₂, and the breaker of the other two elements is the lidar mount | Certified by exact check per generator | Fixture |
| C-09.S6 | Two latches: the tied deterministic class fails, either asymmetric choice succeeds, and a policy drawing one of the two uniformly is equivariant in law; the report says the tied deterministic class is excluded and nothing more | Tested | Finite enumeration |
| C-09.S7 | Adding a one-sided payload removes the reflection from Γ_obs, and the reduced calculation is replaced by the full one without an author's edit | Tested | Edit fixture |
| C-09.S8 | Every element of Γ_obs fixes the sensor suite, the payloads, gravity and every task term; every element of Γ_mech outside Γ_obs has a recorded breaker (PO-MARROW-04) | Certified by exact check per generator | The breaker is replayed as a failing equality |

**Fixtures.**

- *Parity.* P = diag(1, −1, 1). e_y ↦ −P·e_y = e_y; e_x ↦ −e_x; e_z ↦ −e_z. A wheel rolling forward keeps its
  angular momentum under the mirror; a robot turning left becomes a robot turning right. A table that flips
  all three axes, or none, fails step equivariance.
- *Front lidar.* A quadruped built fore-aft and left-right symmetric; one forward-facing lidar on the torso.
  The left-right mirror maps the lidar to itself; the fore-aft mirror and the half-turn do not.
- *Two latches.* Two identical latches of which exactly one must close. At a reflection-fixed observation a
  deterministic policy tied by the reflection issues equal commands (close both or close neither), and both
  equal commands fail; either asymmetric choice succeeds; a uniform draw between the two is equivariant in
  law and succeeds with probability one.
- *Broken symmetry.* The front-lidar quadruped plus a payload on its left side: Γ_obs becomes trivial.

**What it does not claim.** Symmetry of the body is not symmetry of the task. A failed tied search is not
infeasibility. Equivariance in floating point is not exact: round-off breaks it, so the certified statement is
over ℚ and the floating-point statement is a residual test.

**Implementation map.** Crates `frob-algebra` (groups, representations) and `frob-learn` (tying and
augmentation). Verb: `analyze.symmetry` (prints both groups, their generators, the induced representations
and the breakers), and `algebra.irreps`. Gates: GATE-MARROW-03 (TARGET, M3: Γ_mech and Γ_obs equal the groups
on file for all eighteen archetypes and for 24 planted asymmetries, a moved sensor, a one-sided payload, a
changed sole material, a reversed gear, with the correct breaker named in every case; zero disagreements);
GATE-MARROW-04 (the missing-parity-sign mutation caught by at least three fixtures; M4); GATE-REFLEX-07
(planted asymmetries remove exactly the expected generators; the two-latch fixture passes in law and fails
tied; M4); GATE-CORTEX-03 (the three-arm experiment; M4). Cards: T-10, T-16.

## Acceptance criteria

- [ ] `docs/math/C-09.md` exists with the definitions, the parity rule, the equivariance law, the three uses
      and their table, the five limits, the eight statements, the fixtures and the retirement rule.
- [ ] `cargo test -p frob-algebra --test laws_c09` passes on the exact reference and on the production
      symmetry computation once BD-102 lands.
- [ ] Parity fixture exact; a sign table that flips all three axes and one that flips none both fail S1.
- [ ] The §13.8 mutation "a missing parity sign on a mirrored axis" is registered and fails at least three
      of this suite's fixtures in this run: S3's axis images, S1's step equivariance and the signed-permutation
      derivation; the log names them. The cross-construction count that GATE-MARROW-04 reads is the mutation
      harness N13-18, where this suite registers its fixtures.
- [ ] Front-lidar fixture: C₂ × C₂, C₂, breaker = lidar mount; one-sided payload: Γ_obs trivial and the
      consumer falls back to the full calculation with no edit by the author.
- [ ] Two-latch fixture: tied class excluded; the report text asserts nothing about the task.
- [ ] Step equivariance residual at most 10⁻¹² relative in `f64` on contact-free referee steps, and inside the
      tier's round-off envelope (10⁻⁵ relative, TARGET) on `f32` tiers.
- [ ] KA-066 and KA-068 to KA-071 registered with tolerances and controls set; the register row of C-09
      filled.

## Test plan

### Unit tests

- `s3_parity_axes`: the three axis images; a mixed twist under T(P) = diag(det(P)·P, P).
- `s1_step_equivariance`: for each generator of the reference quadruped's Γ_mech, 10³ random states and
  actions on the referee with gravity and a tilted plane transformed along; residual bound as above.
- `s2_mass_matrix_equivariance`: 10³ configurations.
- `s4_nested_groups`: every archetype on file; with sensors, payloads and task terms removed Γ_obs = Γ_mech.
- `s5_front_lidar`, `s7_one_sided_payload`: the fixtures; also the reverse edit restores the reflection.
- `s6_two_latches`: enumerate the four deterministic commands and the uniform mixture; assert the verdict
  wording.
- `s8_breakers_replay`: each recorded breaker, replayed, is a failing exact equality.
- `signed_permutation_derivation`: representations computed from the parity rule equal hand tables for the
  biped and quadruped; a design that writes a mirrored axis as P·a carries a negated coordinate.

### End-to-end script and logging

`scripts/e2e/spec_c09.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control (one control
per build, N10a-01 section 6); the registry rows in their tier targets,
`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g3_metamorphic -- KA-066` (a G3 row) and
`--test g1_known_answers -- KA-068 KA-069 KA-070 KA-071` (G1 rows), each again with
`--features gauntlet-mutations` for the rows' controls; `frob --json analyze symmetry <design>` (a `read` verb,
BD-102; logged `pending` until it lands) on three fixture designs, the symmetric quadruped, the front-lidar
quadruped and the front-lidar quadruped with the one-sided payload, asserting (Γ_mech, Γ_obs) orders (4, 4),
(4, 2) and (4, 1), the breaker field path, and the evidence
block (class `Certified` on the exact routes, `Tested` with the recorded tolerance on the tolerance route).
Log `target/e2e-logs/spec_c09/<utc>.ndjson` with transcript: commands, exit codes, group orders and
generators, equivariance residuals with thresholds, and on failure the generator and the first state
component that differed. Final `summary` line.

### Known answers and negative controls

- KA-066 step equivariance: tolerance set to 10⁻¹² relative (`f64`, contact-free) and the tier envelope on
  `f32`; G3 · all tiers. Control: a mirror sign table that flips all three axes, or none.
- KA-068 parity under the sagittal mirror: exact; G1 · host. Control: the same tables.
- KA-069 two latches: exact verdict; G1 · host. Control: a report that the task itself is infeasible.
- KA-070 two groups of the front-lidar quadruped: exact; G1 · host. Control: Γ_mech used where Γ_obs is
  required.
- KA-071 broken symmetry: exact verdict; G1 · host. Control (set here): the payload left out of the Γ_obs
  filter, so the reduced calculation persists after the edit.
- CX-031, CX-032, CX-033.
- Metamorphic relations: applying a symmetry element to the initial state and the actions gives the element
  applied to the result; a horizontal translation and yaw of the world frame changes nothing an invariant
  observation reports; the mirrored robot on the mirrored course returns the mirrored result, with forces as
  polar and moments as axial vectors (mutation: angular velocity treated as polar).
- Determinism: group computations are D0; residual tests twice, across worker counts, debug and release.

## Considerations

- Exactness routes: by construction (`mirror`, `radial`), by exact field arithmetic (ℚ for mirrors, a real
  cyclotomic subfield for n-fold rotations with n ≤ 12), or by tolerance for imports, classed `Tested`.
  OPEN-MARROW-2 (whether exact field arithmetic extends beyond n = 12): default no; blocks nothing.
- Risk: a consumer reads the author's declared symmetry. The model compiler's invariant is that a declared
  symmetry never replaces a computed one and no consumer reads the declaration.
- Refusal for consumers: a quotient or a weight tie requested under a group that moves a sensor, the payload
  or the task is refused (FROB-REF-161, group not observation-preserving).
- Ceiling arithmetic (card GB-02): between n and m copies of the regular representation of Γ, equivariant maps
  have n·m·|Γ| parameters against n·m·|Γ|² unconstrained; for C₂ × C₂ and width 256 that is 16,384 against
  65,536. The order is between 2 and about 12 for the bodies in scope, so the dividend is real and small.

## References

Plan §10 C-09; §9 D-14; §11.4; §13.2 (ADR-MARROW-06), §13.4, §13.8, §13.10 (PO-MARROW-04); §12.3 (T-10, T-16);
§38.7; Appendix D.5; Appendix E GB-02; Appendix M KA-066, KA-068 to KA-071; Appendix N CX-031 to CX-033.
Bets: BET-07, BET-30. Analysis A-03.
