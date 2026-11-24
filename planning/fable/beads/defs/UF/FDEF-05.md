---
key: "FDEF-05"
title: "hazard report, frankensim `fs-mbd::Wrench`: `force_world` with `torque_body`, two frames in one value"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "marrow", "upstream", "frankensim", "frob-fsim", "solid", "test"]
parent: "WP-37"
deps: ["BD-007", "BD-037", "N33c-36", "N33c-40", "STD-E2E"]
needs: []
related: []
gates: ["GATE-WS-06"]
plan_refs: ["Appendix F.2 row 5", "§33.17", "§11.1", "§11.3", "Appendix F.3"]
---
## Background

A wrench is a force and a torque, and it means nothing without the frame it is expressed in and the point it is
taken about. FrankenRobotics types its wrenches by frame, with three typed maps between frames. FrankenSim has
two wrench types. `fs_ga::Wrench` follows the convention this project uses. `fs-mbd::Wrench`, the external wrench
that the rigid-body step consumes, does not: it is `force_world` with `torque_body`, **two frames in one
value**. The rigid-body type requires the body origin at the centre of mass and applies the angular impulse as
Δt·R̄·`torque_body`, with R̄ the body's orientation over the step.

This is a convention hazard, not a defect: FrankenSim's step is consistent with its own type. The hazard is at
the caller. A wrench held in one frame and copied field by field into `fs-mbd::Wrench` has the same magnitude
and does the same work, points the wrong way, and nothing faults. It is recorded with the five reported defects
because it has the same shape: a FACT to recheck, an admission test here first (the mixed-wrench probe of
GATE-WS-06), an upstream work item there second (FD-FSIM-33: document the mixed frames at the type, or add named
constructors).

Read in source on 2026-11-21 at `483d09c682f2`: confirmed by reading
(`frankensim/crates/fs-mbd/src/lib.rs:1695–1701` for the type; `:582–595` for the centre-of-mass requirement;
`:1913` for the angular impulse).

## Scope

In: the report (`tests/upstream/FD-FSIM-33/REPORT.md`, also the body of the mirrored bead in FrankenSim's
tracker); the standalone reproduction; the probe's recorded values; the statement of the guard. Out: the type
`MixedWrench`, its tests and the probe themselves (N33c-36) and the record of GATE-WS-06 at the pin (N33c-40),
which this bead re-runs and copies but does not define; the upstream change
(FD-FSIM-33); one vector vocabulary across `fs-mbd` and `fs-ga` (FD-FSIM-06).

## Technical approach

**The hand case.** Let the body frame be rotated by 90° about z, R = R_z(90°), so that body x is world y. A
caller holding a body-frame wrench with force f_body = (1, 0, 0) N means a push along world y:
f_world = R·f_body = (0, 1, 0) N. Copied unconverted into `force_world`, it pushes along world x: the same
magnitude, the same energy, a direction wrong by 90°.

**The probe (the admission test; part of GATE-WS-06).** A free rigid body of mass m and inertia I at
orientation R = R_z(90°) receives, through the airlock, a pure world force f_world and a pure body torque
τ_body for one step Δt. Expected: linear momentum change Δp = f_world·Δt in the world frame; angular momentum
change ΔL_world = R·τ_body·Δt; both to 10⁻¹² relative. The unconverted mutant (a body-frame force written into
`force_world`, or a world torque written into `torque_body`) fails. Worked numbers for the reproduction:
m = 2 kg, Δt = 1/480 s, f_world = (0, 3, 0) N, τ_body = (0.5, 0, 0) N·m: Δp = (0, 0.00625, 0) kg·m/s and
ΔL_world = R_z(90°)·(0.5, 0, 0)/480 = (0, 0.00104166…, 0) kg·m²/s. Two conditions make the 10⁻¹² tolerance
honest whatever orientation within the step R̄ denotes (start, midpoint or end): the body starts at rest, and
its inertia is diagonal in the body axes (DESIGN: I = diag(0.1, 0.2, 0.3) kg·m²), so a torque about body x spins
it about body x only, no gyroscopic term appears, and R̄·τ_body = R·τ_body exactly. A probe with a
non-principal torque or an initial spin would mix in an O(Δt²) orientation change of about 10⁻⁶ relative and
must not be used at this tolerance.

**The reproduction (standalone).** One Rust test depending only on `fs-mbd` (and `fs-ga` for the rotation) at
the pinned commit. It steps the free body once with the correctly converted wrench and once with the
unconverted one, and prints both momentum changes beside the expected values.

**Proposed change (carried by FD-FSIM-33; register priority P2).** Either document the mixed frames at the type
(a doc comment on `Wrench` and on each field stating the frame, and on the step stating the centre-of-mass
requirement), or, better, add two named constructors beside the public fields: from a body wrench at the centre
of mass (f_world = R·f_body, torque unchanged) and from a world wrench at the centre of mass
(τ_body = Rᵀ·τ_world, force unchanged). Additive: nothing existing changes.

**The guard this repository keeps meanwhile.** `frob-fsim` defines `MixedWrench { force_world, torque_body }`
beside the three typed frame maps. Its only constructors are the two above; a wrench at any other reference
point is first moved to the centre of mass by the pushforward of the typed maps. No other crate names the
type, and no other crate can build an `fs-mbd::Wrench` at all, because only the airlock names FrankenSim's
volatile types (INV-SUITE-01).

**Procedure.** (1) Copy the probe's recorded momentum changes and the commit from GATE-WS-06's record into the
report (`observed_at`). (2) Run the reproduction in a scratch clone at the pin. (3) File the report as the
mirrored bead of FD-FSIM-33 in FrankenSim's tracker under its `AGENTS.md`, worded as a convention hazard and a
request for documentation or constructors; store the identifier in the registry row.

## Acceptance criteria

- [ ] `tests/upstream/FD-FSIM-33/REPORT.md` has the sections Hazard, Evidence (file, lines, commit), the hand case, the probe with expected and observed values, Proposed change, Interim posture, and `observed_at`.
- [ ] The reproduction compiles against `fs-mbd` and `fs-ga` alone at the pin; the converted wrench gives Δp and ΔL within 10⁻¹² relative of the expected values; the unconverted one differs in direction by 90° with equal magnitude.
- [ ] The report is filed in FrankenSim's tracker; its identifier is the `mirror` of FD-FSIM-33's registry row.
- [ ] The guard holds at the pin: N33c-36's tests pass when re-run here (two constructors plus the point helper, the compile-fail tests for a struct literal and `Default`, the public-API test showing `MixedWrench` in no public signature), and `cargo xtask check-deps` fails a planted scratch crate outside `frob-fsim` that depends on `fs-mbd` and so could name `fs_mbd::Wrench` (INV-SUITE-01, N33c-29).

## Test plan

### Unit tests
- `repro_converted_wrench_momentum`: the worked numbers; expected Δp = (0, 0.00625, 0) and ΔL_world = (0, 0.5/480, 0), each to 10⁻¹² relative.
- `repro_unconverted_force_is_rotated_90_degrees`: body-frame (3, 0, 0) written into `force_world`; expected observed Δp along world x, of magnitude 0.00625, where world y is correct.
- `repro_hand_case_rotations` (in the reproduction, on `fs-ga`'s rotation alone): f_body = (1, 0, 0) at R = R_z(90°) gives f_world = (0, 1, 0); τ_world = (0, 1, 0) gives τ_body = Rᵀ·τ_world = (1, 0, 0); these are the conversions the report proposes as named constructors.
- `report_values_match_gate_record` (in `frob-conformance`): the expected and observed momentum changes and the commit written in `REPORT.md` equal those of GATE-WS-06's row 5 record (N33c-40); a report edited by hand to differ fails.
- The guard's tests are N33c-36's and are re-run here, not written again: `from_body_rotates_force_only`, `from_world_rotates_torque_only`, `pushforward_hand_case` (a world force (0, 0, 1) N at an offset (1, 0, 0) m adds the torque r × f = (0, −1, 0) N·m before conversion), `constructors_agree_on_same_physical_wrench`, `power_is_frame_invariant` (10⁴ seeded rotations and wrenches, 10⁻¹² relative), and the two compile-fail tests.

### End-to-end script and logging
`scripts/e2e/defect_fdef_05.sh`: runs the reproduction in `target/upstream-scratch/frankensim@<rev>`; runs the
`frob-fsim` tests above; runs the planted outside-crate manifest through `cargo xtask check-deps`; reads the
registry row with `frob doctor --json`.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/defect_fdef_05/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FDEF-05`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer: the hand case (body x is world y under R_z(90°)); the probe's momentum changes to 10⁻¹².
- Negative control (required by GATE-WS-06): the unconverted mutant fails the probe. A second mutant, which uses R in place of Rᵀ in the torque constructor, must fail the reproduction's `repro_hand_case_rotations` (it gives (−1, 0, 0) where (1, 0, 0) is right); the same mutant of the guard (N33c-36's second registered control) must fail `from_world_rotates_torque_only` (it gives (0, 1, 0) where (0, −1, 0) is right), and the e2e script logs both outcomes.
- Metamorphic relation: rotating the whole scene (body orientation, force and expected momenta) by any rotation leaves the probe's residual unchanged.
- Determinism lanes: twice on one machine; debug and release; same-platform class.

## Considerations

- The energy check cannot find this error: the wrong wrench does the same work on a body that happens to move along the wrong axis only afterwards. Only a directional known answer finds it, which is why the probe is a momentum test at a non-identity orientation and never at R = I.
- External wrenches enter the referee from actuators through ports, from contact, and from authored disturbances; each of those paths ends in the two constructors.
- No milestone waits for FD-FSIM-33. If it never lands, the local type stays; the cost is one conversion that must be kept in step with FrankenSim's type at every pin move (the probe is re-run by every compatibility train).
- When it lands it is adopted only through its switch-over rule (replace the local type; re-run the mixed-wrench probe); neither a pin nor the toolchain moves during the ten-fold tournament.

## References

Appendix F.2 (row 5), F.3; §33.17 (row 5, the hand case and the type), §11.1, §11.3; GATE-WS-06; FD-FSIM-33; INV-SUITE-01.
