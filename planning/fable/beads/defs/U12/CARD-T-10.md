---
key: "CARD-T-10"
title: "Theorem card T-10: derived step kernels are equivariant under each generator of the mechanical symmetry group"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["N12-06", "N12-20", "N12-21", "N12-22", "SPEC-C-09"]
needs: []
related: ["CARD-T-16", "N12-07", "N12-45", "N13-16", "N22a-32", "N22b-39"]
gates: []
plan_refs: ["§12.3 (T-10)", "§10 C-09", "§11.4", "§13.4", "Appendix D.5 (equivariance of the dynamics)"]
---
## Background

Every pair of a design and a task has two nested finite symmetry groups. The mechanical group Γ_mech is the set
of automorphisms of the labeled kinematic tree realized by an isometry of the body that preserve link lengths,
pseudo-inertias, joint screws, actuator cards, gear ratios and gear directions. The observation-preserving group
Γ_obs ≤ Γ_mech also preserves sensors, payload, task goal, gravity and declared terrain. Under an isometry with
orthogonal part P, polar vectors (positions, linear velocities, forces) map by P and axial vectors (angular
velocities, torques, angular momenta) map by det(P)·P, so the representations ρ on state, action and observation
spaces are signed permutations computed from that rule, not tables. The dynamics are equivariant under Γ_mech
when gravity and terrain are transformed with the robot.

Statement (T-10). A model's derived step kernels are equivariant under each generator of its mechanical symmetry
group, with gravity and terrain transformed by the same isometry: f(ρ_X(g)·x, ρ_U(g)·u) = ρ_X(g)·f(x, u) for every
generator g, state x and input u.

Route, by kernel kind: (a) for the branch-free rational kernels, a polynomial identity per generator, with each
square root replaced by a witness t and the side relation t² = radicand, and one identity per select branch
under its guard; (b) where the symbolic identity is over budget, exact-rational randomized identity testing with
a Schwartz–Zippel bound, only on those polynomial pieces, with a recorded degree bound and sample set (class
`Tested`); (c) a kernel with data-dependent selects (contact, limits) is `Tested` by the round-off-level residual
test. Tier: Frontier. One test of the entire simulator per generator, applied to robot, terrain, gravity and
sensors together, is the metamorphic use; exact reduction inside one world is a different use with a stricter
precondition (Γ_obs intersected with stabilizers) and is not licensed by this card. Rejected alternative:
hand-written mirror functions with sign tables per robot.

## Scope

In: statement document, registry row, the per-generator certificates or test records for a model's step kernels,
the fixture with a wrong parity table. Out: computing the groups and representations (the symmetry analysis
beads); the floating-point metamorphic test (the conformance relations); equivariant networks; the event case
(CARD-T-18).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the symmetry analysis that computes the groups and their representations (BD-102) is `related`, not a dependency
(a card that waited for its implementer would close a loop, because the implementer's own acceptance runs the
card's law tests). What the card does depend on is in `deps`: the card registry, its specification, its pilot
where the route is Frontier, and Lemma's own proof machinery. It closes on its statement document, its registry
row, its specimens and mutants, and every acceptance line that its own fixtures decide. An acceptance line that
can only be observed on live output (the per-generator certificates or test records for a reference model's step
kernels) cannot be run inside the implementing bead here, because that bead lands four milestones earlier (BD-102
at M1) and is not reopened: the certificates and test records on the step kernels as built are issued at M5 by
this card through the instance-certificate pipeline (N12-20) and the randomized identity test (N12-21), both
dependencies; the receipt of that run moves the registry row, and the coverage run (N12-36) observes it again.

## Technical approach

1. For a model and each generator g of Γ_mech, build the transformed input (ρ_X(g)·x, ρ_U(g)·u) and the
   transformed parameters (gravity and terrain by the same isometry), symbolically execute the step kernel on
   both, and state the difference ρ_X(g)·f(x, u) − f(ρ_X(g)·x, ρ_U(g)·u) = 0 per output component.
2. Per piece: denominators cleared with their nonvanishing conditions recorded; square-root witnesses; one
   identity per select branch, restricted to inputs satisfying the guard and whose image under ρ satisfies the
   image guard (the guard itself must be invariant: its invariance is a sub-identity).
3. Decide by route (a) through the producer pipeline; fall to (b) per piece on `budget-exceeded`; record (c) for
   kernels that cannot be split, with the residual test's identifier. Route (b) uses the Schwartz–Zippel lemma: a
   nonzero polynomial of total degree at most d, evaluated at a point drawn uniformly from Sⁿ for a finite set
   S ⊂ ℚ, vanishes with probability at most d/|S|; k independent draws that all vanish leave an error bound of
   (d/|S|)ᵏ. The record (`K-<name>.idtest`, schema `frob.identity.test/1`, N12-21's) holds d (a bound computed
   from the straight-line program, products adding degrees and the cleared denominators included), S as
   {0, …, 2ᵇ − 1}, k (N12-21's m), the stream address of the addressed random draws, from which every sample point
   is regenerated (the points are not stored), the discards and the bound; the implementation of the test itself
   is N12-21's.
4. Class: `Certified` for pieces decided by (a), with a `poly-identity` bundle per generator; `Tested` for (b)
   and (c), stated as such. The card's printed class for a model is the minimum over its pieces. Not `Proved`.
   The simulator's equivariance in floating point is not claimed: round-off breaks exact equivariance, which is
   why the floating-point statement is a residual test.
5. Relies on it: construction C-09; the metamorphic tests of the verification program; symmetric designs made by
   the `mirror` and `radial` constructors; symmetric controllers and augmentation, which need more than this
   card (invariance in law of the world and command distributions for weight tying).
6. Artifacts: `docs/math/cards/T-10.md`; registry row with per-model records; bundles or `.idtest` records.

## Acceptance criteria

- [ ] The pilot N12-22 (arm F) is on record before work starts.
- [ ] For the reference quadruped (built symmetric fore-aft and left-right, Γ_mech = C₂ × C₂), each generator has,
      for every branch-free step kernel, a certificate accepted by `frob-verify` or a randomized test record with
      its degree bound, sample set and error bound.
- [ ] Negative controls caught: a parity table that flips all three joint-axis signs, or none, under the sagittal
      mirror; gravity left untransformed; a one-sided payload added (the reflection is then not a symmetry and the
      identity must fail).
- [ ] The registry row prints the class per model as the minimum over pieces, with receipts.
- [ ] Kernels with data-dependent selects are listed as `Tested` with their residual test, never as certified.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t10.rs`: `sagittal_mirror_axis_signs` (P = diag(1, −1, 1): the axial image of a
joint axis a is −P·a, so a pitch axis along y keeps its sign and roll and yaw axes change theirs);
`wrong_table_flip_all_fails`; `wrong_table_flip_none_fails`; `gravity_not_transformed_fails`;
`one_sided_payload_breaks_identity`; `free_body_step_equivariant_exact` (the single-body Cayley step under the
mirror, in exact rationals); `mass_matrix_equivariance` (ρ(g)·M(q)·ρ(g)ᵀ = M(ρ(g)·q) on a planar two-link
mirrored pair); `class_is_minimum_over_pieces`; `randomized_piece_records_bound` (a piece forced to route (b)
with d = 12, |S| = 2³² and k = 4: the record carries d, b = 32, m = 4 and the stream address, the four sample
points regenerated from that address reproduce the recorded verdict, the bound is (12/2³²)⁴, and the piece prints
class `Tested`).

### End-to-end script and logging

`scripts/e2e/lemma_coverage.sh` (bead N12-36) runs the per-generator identities for the reference models and logs
route, degree bound, sample set and verdict per piece. For this card its NDJSON lines (STD-E2E helper) carry `ts`,
`script`, `bead` ("CARD-T-10"), `step` (`T-10/<model>/<generator>/<kernel>/<piece>`), `event`, the exact command
line, exit code, wall time, peak memory, the route (a, b or c), the envelope's `evidence` block (claim, class,
`KernelId`, certificate hash or `.idtest` digest, checker identity), d, |S|, k and the error bound for route (b),
and on a failure the first failing output component and the path to reproduce; the `summary` gives the printed
class per model and the negative controls caught.

### Known answers and negative controls

CX-100 (angular velocity does not mirror like any other vector: under the sagittal mirror the yaw axis e_z goes
to −e_z, so a robot turning left becomes a robot turning right, and the pitch axis e_y maps to itself; treated as
polar, the yaw rate is left unchanged). A quadruped with a front-only lidar has Γ_mech = C₂ × C₂ and Γ_obs = C₂:
this card concerns Γ_mech. Negative controls as listed. Determinism: exact for routes (a) and (b).

## Considerations

- An equivariant nonlinear map need not decouple into independent nonlinear blocks; this card licenses the
  metamorphic test and nothing about block structure.
- The identity is per model: a design edit that breaks a symmetry removes the generator and the record.
- KILL-LEMMA's fallback turns route (a) into route (b) everywhere.

## References

§12.3; C-09; §11.4; T-16, T-18; CX-100; BET-11.
