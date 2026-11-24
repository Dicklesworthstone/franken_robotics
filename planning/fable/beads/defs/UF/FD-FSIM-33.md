---
key: "FD-FSIM-33"
title: "upstream frankensim: `fs-mbd::Wrench` mixed frames documented, or named body and world constructors"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "marrow", "upstream", "frankensim", "frob-fsim", "solid"]
parent: "WP-02"
deps: ["FDEF-05", "N33b-04", "N33c-36", "STD-E2E"]
needs: []
related: ["N33c-47"]
gates: ["GATE-WS-06"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 5", "Appendix F.3", "§33.17", "§11.1", "§11.3"]
---
## Background

`fs-mbd::Wrench`, the external wrench that FrankenSim's rigid-body step consumes, is `force_world` with
`torque_body`: **two frames in one value** (FACT, read 2026-11-21 at `483d09c682f2`:
`frankensim/crates/fs-mbd/src/lib.rs:1695–1701`). The rigid-body type requires the body origin at the centre of
mass (`lib.rs:582–595`) and applies the angular impulse as Δt·R̄·`torque_body` (`:1913`). `fs_ga::Wrench` and
this project's own convention keep force and torque in one frame. The step is self-consistent; the hazard is at
the caller: a wrench copied field by field from one frame has the same magnitude and does the same work, points
the wrong way, and nothing faults. The report is FDEF-05. Upstream register priority: **P2**.

On this side the hazard is closed by a type: `MixedWrench { force_world, torque_body }` in the airlock crate
`frob-fsim`, with exactly two constructors, and no other crate may name it. The upstream item asks that the
next consumer of `fs-mbd` not need to rediscover this. Rejected: a wrapper in FrankenSim that silently converts
(it would hide which frame the caller holds); leaving it to documentation outside the type.

## Scope

In: in `fs-mbd`, the frames stated at the type and its fields, or (preferred) two named constructors; here, the
local type replaced by the sibling's constructors and the probe re-run. Out: unifying `fs-mbd`'s vectors with
`fs-ga`'s (FD-FSIM-06); the report (FDEF-05); frame-typed wrenches of this project (BD-047).

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

**The change (DESIGN).** With R the rotation that takes body coordinates to world coordinates:

1. *Documentation at the type.* A doc comment on `Wrench` stating that `force_world` is expressed in the world
   frame and `torque_body` in the body frame, both taken about the centre of mass; on each field; and on the
   step, the centre-of-mass requirement.
2. *Named constructors (preferred, additive beside the public fields).*
   `Wrench::from_body_wrench_at_com(f_body, τ_body, R)` sets `force_world = R·f_body` and leaves the torque
   unchanged; `Wrench::from_world_wrench_at_com(f_world, τ_world, R)` sets `torque_body = Rᵀ·τ_world` and leaves
   the force unchanged. A wrench about another point p (offset r from the centre of mass, in the frame of the
   force) is first moved to the centre of mass: τ_com = τ_p + r × f.

Nothing existing changes: the fields stay public and the step is untouched.

**Interim posture here.** `MixedWrench` in `frob-fsim` with exactly two constructors (from a body wrench at the
centre of mass; from a world wrench at the centre of mass); a wrench at any other reference point is first
moved by the pushforward of the typed frame maps. No other crate names the type.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-33/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-33`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_33` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It looks for the two named constructors on `fs_mbd::Wrench` (absent at the
pin) and, when present, runs the mixed-wrench probe through them instead of through `MixedWrench`.

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

**What the switch-over re-runs.** The rule is "replace the local type; re-run the mixed-wrench probe of
GATE-WS-06": `MixedWrench`'s two constructors become thin calls to the sibling's (or the type becomes an alias
if the sibling's type then carries the same guarantees), and the probe is re-run: a free body at R = R_z(90°)
receives a pure world force and a pure body torque through the airlock; the linear and angular momentum changes
match to 10⁻¹²; the unconverted mutant fails. If the sibling only documented the frames (form 1), the local
type stays, its doc comment cites the sibling's, and the item is adopted at that.

**Fallback if the change cannot land.** The local type stays, and the probe is re-run by every compatibility
train.

## Acceptance criteria

- [ ] In FrankenSim: the type and both fields state their frames and the centre-of-mass requirement; if constructors were added, they pass the known answers below; no existing behaviour changes.
- [ ] Here: at the new pin `frob-fsim` builds `fs_mbd::Wrench` only through the sibling's constructors (or records that the sibling chose documentation only); still no crate outside `frob-fsim` names `MixedWrench` or `fs_mbd::Wrench`.
- [ ] Here: the mixed-wrench probe passes to 10⁻¹² and its unconverted mutant fails, at the new pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-mbd`):
- `from_body_wrench_rotates_force_only`: R = R_z(90°), f_body = (1, 0, 0) N, τ_body = (0, 0, 2) N·m; expected `force_world` = (0, 1, 0), `torque_body` = (0, 0, 2).
- `from_world_wrench_rotates_torque_only`: R = R_z(90°), f_world = (0, 3, 0) N, τ_world = (0, 1, 0) N·m; expected `force_world` = (0, 3, 0), `torque_body` = Rᵀ·τ_world = (1, 0, 0).
- `step_momentum_matches_constructors`: a free body of 2 kg with inertia diag(0.1, 0.2, 0.3) kg·m² in its body axes, starting at rest, Δt = 1/480 s, with the wrench of the second case: Δp = (0, 0.00625, 0) kg·m/s and ΔL_world = (0, 1/480, 0) = (0, 0.0020833, 0) kg·m²/s, to 10⁻¹² relative (the torque is about the principal body axis x, so the step's mid-step orientation R̄ maps it exactly as R does; a non-principal torque or an initial spin would add an O(Δt²) term near 10⁻⁶ and is not used at this tolerance).
- `off_centre_force_adds_torque`: f = (0, 0, 1) N at r = (1, 0, 0) m; expected added torque r × f = (0, −1, 0) N·m before conversion.
Here:
- `airlock_uses_sibling_constructors`: a source scan and a behaviour test at the new pin.
- `mixed_wrench_probe` and `mixed_wrench_probe_unconverted_mutant_fails`: the probe of GATE-WS-06.
- Property test: for 10⁴ random rotations and wrenches, power computed in the body frame equals power computed from the two fields, to 10⁻¹² relative.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_33.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_33 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post`
runs the probe of GATE-WS-06 at the new pin (`cargo test -p frob-fsim mixed_wrench_probe`, and its unconverted
mutant), which carries the adoption. A verb-level check (a referee run of a one-body fixture with an authored
external wrench in the body frame at R_z(90°), asserting the world-frame momentum change of the known answer) is
not part of this script: no row of `sim.run` takes an authored external wrench before the disturbance model of
N26a-02 (M3), a later bead, so the check belongs to that bead's own script (KNOWN_DRIFTS 7 and 8; requested of
N26a-02).

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_33/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-33`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer: under R_z(90°), body x is world y: a body-frame force (1, 0, 0) N is a push along world y; copied unconverted into `force_world` it pushes along world x with the same magnitude and energy.
- Negative controls: the unconverted mutant (required by GATE-WS-06); a constructor that uses R where Rᵀ belongs must fail `from_world_wrench_rotates_torque_only`; a probe run at R = I must be rejected by the test harness as uninformative (the two frames coincide there).
- Metamorphic relation: rotating body orientation, wrench and expected momenta by a common rotation leaves the residual unchanged.
- Determinism lanes: twice on one machine; debug and release; same-platform class.

## Considerations

- Energy checks cannot find this error, since the wrong wrench does the same instantaneous work on a body at rest; only a directional known answer at a non-identity orientation does.
- Every path by which an external wrench reaches the referee (actuator ports, contact, authored disturbances) ends in the two constructors, so the switch-over touches one module.
- Small and cheap: a natural first upstream change for an agent new to FrankenSim's repository and rules.
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

Appendix F.1 (row FD-FSIM-33), F.2 (row 5), F.3; §33.17 (row 5, the hand case and the type), §11.1, §11.3; GATE-WS-06; INV-SUITE-01.
