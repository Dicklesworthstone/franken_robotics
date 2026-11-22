---
key: "FD-FSIM-01"
title: "upstream frankensim: spherical, universal, planar and cylindrical joints as native joint kinds in `fs-mbd`"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "suite", "kinesis", "upstream", "frankensim", "frob-fsim", "solid"]
parent: "WP-10"
deps: ["BD-037", "BD-061", "NF-01", "STD-E2E"]
needs: []
related: ["BD-089", "BD-159", "BD-046", "BD-032", "BD-088"]
gates: ["GATE-KIN-02", "GATE-KIN-03"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§16.4", "§33.8", "§33.3"]
---
## Background

The referee tier of FrankenRobotics is FrankenSim's articulated dynamics (`fs-mbd`), reached through the airlock
crate `frob-fsim`. FACT (FrankenSim at `483d09c682`, read 2026-11-21, re-checked by an admission test before any
gate cites it): the joint classes of `fs-mbd`, its `JointKind`, are limited to fixed, revolute, prismatic and
helical. Robots need more: a ball joint at a hip or a wrist (spherical), a Cardan joint (universal), a base
sliding on a plane (planar), a telescoping rotating column (cylindrical). Upstream register priority: **P1**.

Until FrankenSim has them, the referee lowers each to a product of one-axis joints. The lowering is exact
kinematically away from its singularities, and it costs three things. It inserts massless intermediate bodies
(a chain of massless parents has a singular inertia, so the model compiler must place mass with care). A
spherical joint lowered to three revolutes has a coordinate singularity when the middle angle reaches ±90°,
which the physical joint does not have. And the referee's coordinates then differ from the batched tiers',
where each joint is its own Lie subgroup, so parity tests compare through a coordinate map.

Rejected: implementing the missing joints in a local fork of `fs-mbd` (a fork of a crate of about 292 thousand
lines that moves daily); refusing designs with such joints on the referee (the referee would not cover ordinary
robots); waiting for the upstream change before M1 (no milestone waits on a sibling).

## Scope

In: the four joint kinds in FrankenSim, either in `fs-mbd` or in the `fs-kinematics` crate proposed there; the
airlock's use of them; the re-run of the two parity gates. Out: the lowering itself and the referee model
compiler (BD-061); loop closures and rolling constraints (FD-FSIM-04); joint subgroups on the batched tiers
(BD-046).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankensim` repository
(`../frankensim` beside this one; head `483d09c682f2` and toolchain channel `nightly-2026-07-06` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. For FrankenSim those rules include its layer checks, its contracts, its unsafe-capsule discipline, its ban on
in-process foreign kernels in production paths and its ban on branches and worktrees (FACT:
`frankensim/AGENTS.md`, `frankensim/docs/ACCELERATOR_DOCTRINE.md`), so the work is committed on `main` in small
steps that each pass its gates. FrankenSim moves fast (FACT: 409 commits touched `crates/fs-mbd` in the thirty
days to 2026-11-21, and its checkout had 32 modified paths), so every file named in this bead is re-read at the
current head before it is edited.

1. Read that repository's `AGENTS.md` and its coordination rules first. They govern; where they differ from this
   bead, they win, and the difference is written into this bead's closing note.
2. Other agents work in the same repository. Before editing, reserve every file to be touched through Agent
   Mail and follow the repository's own coordination rules; release the reservations once the change has landed.
3. The change is additive (DESIGN): a new mode, entry point, type or feature. Existing behaviour is not changed,
   and any lossiness is self-described in the new item's documentation and return type.
4. It lands on that repository's `main` through its own tests and gates. Nothing in this repository patches,
   forks, vendors or edits the sibling, and no tool of this repository writes into its checkout (INV-SUITE-04).
5. The work is tracked twice: this bead, and a mirrored bead in the sibling's tracker whose identifier is stored
   in the `mirror` field of this item's row in `registries/upstream.toml` (schema `frob.upstream/1`). The row's
   `status` moves `open` → `landed` (merged on the sibling's `main`) → `adopted` (switch-over run here).

**The change (DESIGN; FrankenSim's own conventions decide names and placement).** Four additive variants beside
the existing four, each defined by its motion subspace and its configuration update:

| Kind | Degrees of freedom | Configuration | Velocity coordinates | Lowered today as |
|---|---|---|---|---|
| Spherical | 3 | A unit quaternion | Body angular velocity ω ∈ ℝ³ | Three revolutes on intersecting axes through two massless frames |
| Universal | 2 | Two angles about perpendicular intersecting axes | (θ̇₁, θ̇₂) | Two revolutes through one massless cross |
| Planar | 3 | (x, y, ψ) in the joint plane | (ẋ, ẏ, ψ̇) | Two prismatics and one revolute |
| Cylindrical | 2 | (s, φ) along and about one axis | (ṡ, φ̇), independent | One prismatic and one revolute on a common axis |

Each kind must be supported wherever `JointKind` is matched: forward kinematics, the recursive Newton–Euler
pass, the articulated-body solve, the floating-base maps and the rigid-point motion. The spherical kind
integrates on the group (a quaternion retraction, renormalized), not in Euler angles. A cylindrical joint is not
a helical one: a helical joint has one degree of freedom with translation tied to rotation by its pitch.
Existing kinds, their numerics and their golden values do not change.

**Interim posture here.** The referee lowers the four classes to products of one-axis joints, with the axis
order recorded in the compiled model; near a lowering singularity the referee refuses by name instead of
returning an ill-conditioned answer.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-01`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the airlock trait `RefereeDynamics` whether each of the four classes
is native (false at the pin) and runs the spherical known answer below on the native kind (absent at the pin).

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) run `cargo xtask check-stack`, which writes a new resolution receipt
(`frob.stack-receipt/1`) under `receipts/`; (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** GATE-KIN-02 (parity, M1): contact-free forward dynamics of each reference robot
against the pinned oracle, joint accelerations within 10⁻⁹ relative in `f64`. GATE-KIN-03 (parity, M2):
`batch-cpu` in `f64` against `fs-mbd`: kinematics, inverse dynamics, forward dynamics and mass matrix within
10⁻¹⁰ relative on 10⁴ states per reference robot. Both are re-run with the referee using the native kinds, on
the reference robots and on the four fixtures below. The lowering stays in the code base as a differential
oracle in tests and for a worker process held at an older pin.

**Fallback if the change cannot land.** The lowering remains the referee's path, with its singularity refusal.

## Acceptance criteria

- [ ] In FrankenSim: the four kinds exist, are documented with their coordinates, and pass its own tests; every existing test and golden value for the old four kinds is unchanged.
- [ ] In FrankenSim: momentum and energy checks pass for each new kind on a free and a driven case, at the tolerances its own contact-free tests use.
- [ ] Here: `RefereeDynamics` reports the four classes native at the new pin, and the model compiler uses them.
- [ ] Here: native and lowered models of the same mechanism agree in joint-space accelerations to 10⁻¹⁰ relative on 10⁴ random states away from the lowering's singularities.
- [ ] Here: GATE-KIN-02 and GATE-KIN-03 pass at the new pin with measured values recorded against 10⁻⁹ and 10⁻¹⁰.
- [ ] Here: the spherical fixture passes through the configuration at which the lowered model refuses.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, additive; names follow its conventions):
- `spherical_free_symmetric_top`: a body with principal inertias (2, 2, 1) kg·m² on a spherical joint at its centre of mass, no gravity, body spin ω₃ = 10 rad/s with a small transverse rate; expected body-frame precession rate (I₃ − I₁)/I₁·ω₃ = −5 rad/s within 10⁻⁴ relative, and world angular momentum constant to 10⁻¹² relative over 10⁴ steps.
- `universal_equals_two_revolutes`: accelerations of the native kind and of two perpendicular revolutes agree to 10⁻¹² on 10³ states.
- `planar_free_body`: mass 2 kg, inertia 0.5 kg·m², force (1, 0) N applied 0.25 m off the centre of mass along y; expected linear acceleration (0.5, 0) m/s² and angular acceleration −0.5 rad/s².
- `cylindrical_is_not_helical`: axial force 3 N and axial torque 0.2 N·m on a body of 2 kg and axial inertia 0.1 kg·m²; expected s̈ = 1.5 m/s² and φ̈ = 2 rad/s², independent of each other.
Here:
- `referee_reports_native_joint_classes`: expected all four true after adoption, all four false at the starting pin.
- `native_matches_lowered_away_from_singularity`: 10⁴ seeded states; tolerance 10⁻¹⁰ relative.
- `lowered_spherical_refuses_at_gimbal`: middle angle within 10⁻⁶ rad of 90°; expected the named refusal on the lowered path and a finite answer on the native path.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob design check` and `frob sim run --tier referee --json` on a fixture robot with
one joint of each class, and asserts that the envelope reports the classes as native and that the parity
gates' measured values are inside their thresholds.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: free precession of a symmetric top, (I₃ − I₁)/I₁·ω₃; the planar free body and the cylindrical joint above; pendulum period and momentum conservation as in GATE-KIN-01.
- Negative controls: a spherical kind implemented by integrating Euler angles must fail the momentum test through the 90° configuration; a cylindrical kind implemented as helical with pitch 1 must fail `cylindrical_is_not_helical`; a transposed motion subspace must fail `universal_equals_two_revolutes`.
- Metamorphic relation: reordering the axes of the lowering changes the lowered coordinates and not the native result.
- Determinism lanes: twice on one machine; across worker counts; debug and release; same-platform class only for the referee, since `fs-mbd` makes no cross-ISA claim (FD-FSIM-07).

## Considerations

- The spherical kind adds a non-Euclidean coordinate to a code base whose joints have been one-axis; every place that assumes one coordinate per velocity (state packing, limits, damping, serialization) is in scope there. That is the cost that made the item an upstream one.
- Joint limits, stiction, backlash and hard stops are rows in the referee solve on this side; for a spherical joint a limit is a cone or a swing–twist bound, which the lowering expresses awkwardly. Adoption improves that; it is not part of this item's acceptance.
- `fs-mbd` is plain `f64` and not generic over a differentiation scalar (FACT, re-read at INT-FSIM-03); the new kinds follow suit.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists, not that the
  milestone's exit depends on it. Until the switch-over has been run, the interim posture above is the design of
  record. M1 depends only on FrankenSim, asupersync and FrankenSQLite, and no upstream work item is on its
  critical path.
- If this change never lands, the program still ships on the interim posture, at the cost stated there. A change
  that does land is adopted only through its switch-over rule: never by a local patch, a fork or a vendored copy,
  and never by rolling a sibling's newer clean head back. Neither a pin nor T moves during the ten-fold
  tournament of M7.
- The priority in the upstream register states the item's value to this project and its place in this program's
  queue. The risks carried at program level are the cost and ordering of upstream work and collisions with other
  agents in the same sibling; the mitigations are file reservation, small commits and the sibling's own gates.

## References

Appendix F.1 (row FD-FSIM-01), F.3; §16.4 (the limits table), §33.8, §33.3; GATE-KIN-02, GATE-KIN-03; RISK-01.
