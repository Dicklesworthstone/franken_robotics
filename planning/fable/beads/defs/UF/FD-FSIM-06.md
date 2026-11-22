---
key: "FD-FSIM-06"
title: "upstream frankensim: one vector vocabulary across `fs-mbd` and `fs-ga` (conversions in the sibling, then none here)"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "marrow", "upstream", "frankensim", "frob-fsim", "solid"]
parent: "WP-02"
deps: ["BD-037", "BD-032", "NF-01", "STD-E2E"]
needs: []
related: ["BD-007", "BD-010", "BD-047", "FD-FSIM-33", "FDEF-05"]
gates: ["INT-FSIM-02", "INT-FSIM-03"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§13.2", "§33.3", "§33.8"]
---
## Background

FrankenRobotics reuses FrankenSim's `fs-ga` as its host representation of rotations, rigid motions, twists and
wrenches (ADR-MARROW-01: `So3`, `Se3`, `Twist`, `Wrench` and `Mat6` are FrankenSim's, with deterministic
elementary functions already routed through `fs_math::det`; this project adds traits, the groups `fs-ga` lacks
and frame typing, and does not add another vocabulary). FACT: FrankenSim itself has **two vector vocabularies**
today, because `fs-mbd` defines its own `Vec3`, `UnitQuaternion` and `Pose` beside `fs-ga`'s. Upstream register
priority: **P2**.

So every call from this project into the referee crosses from one vocabulary to the other, inside the airlock
crate `frob-fsim`. Each crossing is a place where a convention can be silently wrong: quaternion component
order, the sign of the scalar part, whether a rotation acts on vectors or on frames, the order of composition
of poses. None of these faults; all of them change answers.

Rejected: a third vocabulary of our own (ADR-MARROW-01 forbids it); converting ad hoc at call sites above the
airlock (only the airlock may name FrankenSim's volatile types, INV-SUITE-01).

## Scope

In: in FrankenSim, named conversions between the two vocabularies with their conventions documented and tested,
so that a caller holding `fs-ga` values can drive `fs-mbd`; here, deletion of the airlock's own conversions.
Out: the mixed-frame wrench of `fs-mbd` (FD-FSIM-33, FDEF-05); frame-typed transforms of this project (BD-047);
any removal of `fs-mbd`'s own types, which is FrankenSim's decision.

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

**The change (DESIGN).** Additive conversions, in whichever of the two crates FrankenSim's layering allows:
`fs_ga::So3` ↔ `fs_mbd::UnitQuaternion`, `fs_ga::Se3` ↔ `fs_mbd::Pose`, three-vectors both ways, each total,
each with a doc comment that states the convention it bridges, and each with a round-trip test. Where
`fs-mbd`'s public entry points take its own types, overloads or `impl Into` parameters accept the `fs-ga`
types. The long-term goal named by the item, one vocabulary, is reached when no public signature of `fs-mbd`
requires its private types; that step is FrankenSim's to schedule, and this item is complete at the conversions.

**Interim posture here.** The airlock converts, in one module of `frob-fsim`, with the convention tests below.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-06/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-06`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_06` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It checks that FrankenSim exports the named conversions (absent at the
pin) and runs the convention known answers through them.

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

**What the switch-over re-runs.** The rule is "delete conversions": the airlock's conversion module is removed
in favour of the sibling's, and then the admission tests that exercise the crossing are re-run. INT-FSIM-02:
power ⟨wrench, twist⟩ is frame-invariant to 10⁻¹² relative when a moving link is transformed; the analytic
Jacobian agrees with central differences to 10⁻⁷ relative at 1,000 seeded configurations; a deliberately
reversed adjoint fails at least three fixtures. INT-FSIM-03: a two-joint arm's accelerations equal those from
independently differentiated energy to 10⁻¹⁰. And the G1 forward-dynamics digest must equal the one computed
inside FrankenSim at the pinned commit, unchanged by the deletion.

**Fallback if the change cannot land.** The airlock keeps its conversion module; the cost is that it must be
re-tested at every pin move.

## Acceptance criteria

- [ ] In FrankenSim: the conversions exist, are documented with the conventions, and round-trip exactly (bit for bit for vectors and unit quaternions, up to the sign of the double cover where the source type does not fix it, which is then stated).
- [ ] Here: `frob-fsim` contains no hand-written conversion between the two vocabularies at the new pin (a search for the removed module's functions finds nothing).
- [ ] Here: INT-FSIM-02 and INT-FSIM-03 pass at the new pin with their measured values recorded, and the G1 digest is unchanged.
- [ ] Here: the convention tests below pass against the sibling's conversions.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added) and, until adoption, the same cases against the airlock's module:
- `quarter_turn_about_z_maps_x_to_y`: the rotation R_z(90°) built in each vocabulary and converted to the other; applied to (1, 0, 0) it gives (0, 1, 0) to 10⁻¹⁵ in both.
- `quaternion_component_order`: the quaternion of R_z(90°) has scalar part cos 45° = 0.70710678 and vector part (0, 0, 0.70710678); converting and reading back each named component gives the same four numbers.
- `double_cover_is_handled`: q and −q convert to the same rotation matrix.
- `pose_composition_order`: T_ab = (R_z(90°), (1, 0, 0)) and T_bc = (I, (0, 2, 0)); composed and applied to the origin of c the result is (−1, 0, 0), in both vocabularies and through the conversion in either order. (R_z(90°)·(0, 2, 0) = (−2, 0, 0); adding (1, 0, 0) gives (−1, 0, 0).)
- `round_trip_is_identity`: 10⁴ random unit quaternions and poses; `fs-ga` → `fs-mbd` → `fs-ga` is bit-identical or differs only by the stated sign.
Here:
- `airlock_has_no_local_vector_conversion`: a source scan of `frob-fsim` for the deleted function names.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_06.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_06 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob sim run g1 --tier referee --seconds 2 --json` and compares
the final digest with the digest recorded before the deletion at the same pin.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_06/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-06`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the quarter turn; the composed pose (−1, 0, 0); frame invariance of power to 10⁻¹².
- Negative controls: a conversion that reads a scalar-last quaternion as scalar-first must fail `quaternion_component_order`; one that composes poses in the reverse order must give (1, 2, 0) and fail; the reversed adjoint of INT-FSIM-02 must fail at least three fixtures.
- Metamorphic relation: converting, composing and converting back equals composing in the source vocabulary.
- Determinism lanes: bit-identical on every platform (no transcendental function is involved in a conversion).

## Considerations

- The hazard is silent wrongness, so the tests are directional known answers at non-identity rotations, never round trips alone: a consistent pair of wrong conventions round-trips perfectly.
- `fs-ga` routes elementary functions through deterministic implementations and `fs-mbd` calls platform ones (FD-FSIM-07); a conversion does not change which functions a computation used.
- This item removes code here; its value is one fewer place to re-verify at every pin move, on a crate with 409 commits in thirty days.
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

Appendix F.1 (row FD-FSIM-06), F.3; §13.2 (ADR-MARROW-01), §33.3 (the airlock's rules), §33.8, §33.10; INT-FSIM-02, INT-FSIM-03; INV-SUITE-01.
