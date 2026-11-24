---
key: "FD-FSIM-04"
title: "upstream frankensim: holonomic loop constraints and nonholonomic rolling in `fs-mbd`"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "kinesis", "upstream", "frankensim", "solid"]
parent: "WP-39"
deps: ["BD-037", "BD-081", "BD-110", "N16b-23", "N33b-04", "STD-E2E"]
needs: []
related: ["N16a-05", "N16b-22", "N16bx-01", "N16bx-02", "N18b-08", "N19b-18", "NAB-49", "ROBOT-cassie", "ZOO-ackermann", "ZOO-rocker-bogie"]
gates: ["GATE-KIN-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§16.4", "§16.12", "§18.9", "§33.8", "Appendix C"]
---
## Background

Robots contain closed kinematic loops (a four-bar knee, a parallel ankle, a steering linkage) and rolling
contacts idealized as constraints. `fs-mbd`, FrankenSim's multibody crate and this project's referee, is a tree
solver: it has no holonomic loop constraints and no nonholonomic rolling constraint. Upstream register
priority: **P2**.

How the tiers treat a closure today (DESIGN). On the lockstep tiers a closure is a compliant equality row in
the convex solve, with its compliance stated. Under a transactional profile closures are the bilateral rows of
the transaction. On the referee, **exact closures wait for FrankenSim's constraint lane**: a design with a loop
is refused there by name (`FROB-REF-721`, cause `kinematic-loop`, exit 5; ADR-KIN-06, the limit register of
N16a-05, the lowering of N16b-23), and the bilateral rows of the `strict` profile adjudicate it from M2. The
referee never adjudicates a loop with a model that is not exactly the mechanism.

Rejected: symbolic elimination of each loop into reduced coordinates as the general answer (kept as a Frontier
option for templates with closed-form closures; it does not scale to arbitrary loops); a local constraint
solver wrapped around `fs-mbd` (the constraint forces enter the articulated solve itself, so it would be a
fork).

## Scope

In: in FrankenSim, holonomic constraints g(q) = 0 between two frames of a tree, and nonholonomic (Pfaffian)
constraints A(q)·v = 0, both additive; here, exact-closure known answers on the referee. Out: compliant
closures on the fast tiers and the bilateral rows of the transaction (N16b-23, over the island solve of
N16a-07); the `closure`
constructor of the design language (BD-056); bounded exact linkage synthesis (BD-361); the simulator's wheel
contact, which is a friction law and not a rolling constraint.

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

**The change (DESIGN).**

1. *Holonomic loops.* A constraint g(q) = 0 with velocity Jacobian G(q), defined by ġ = G·v, cutting each loop at
   one joint. With M the mass matrix, f the generalized forces and λ the constraint forces:

     M·v̇ + Gᵀ·λ = f,   G·v̇ = −Ġ·v,

   solved together, with the drift of g and of G·v removed by projection after the step (or an equivalent
   stabilization that FrankenSim's own integrators prefer), and with the residuals ‖g‖ and ‖G·v‖ returned as
   evidence. A redundant or inconsistent set of constraints is refused with the offending rank.
2. *Nonholonomic rolling.* A constraint A(q)·v = 0 that is not the derivative of any g. The reference case is
   a disk of radius r rolling upright on a plane, with position (x, y), heading θ and roll angle φ:
   ẋ − r·φ̇·cos θ = 0 and ẏ − r·φ̇·sin θ = 0. The equations are M·v̇ + Aᵀ·λ = f with A·v̇ = −Ȧ·v.

Additive: tree models without constraints run exactly the code they run today.

**Interim posture here.** Compliant equality rows on the fast tiers; bilateral rows under transactional
profiles; no exact loops on the referee, where a model with a loop is refused (`FROB-REF-721`,
`kinematic-loop`) and routed to `strict`. The nonholonomic model is this project's planning model, static
analysis model and known-answer oracle for rolling; it is not what the simulator integrates.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-04/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-04`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_04` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks `RefereeDynamics` whether exact loop closure is available (false at
the pin) and, when it is, runs the parallelogram known answer below with the closure residual as the assertion.

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

**What the switch-over re-runs.** The rule is "add exact-closure known answers": the dynamics known-answer
battery of the referee gains the rows below, and GATE-KIN-01 (pendulum period within 10⁻⁶ relative at h = 1 ms;
symmetric-top precession within 10⁻⁴; free-flight momentum conserved to 10⁻¹² relative over 10⁶ steps on the
variational lane; ballistic invariants) is re-run with them. The model compiler then lowers a `closure` to the
exact constraint on the referee instead of the `kinematic-loop` refusal, and the compliant closure remains the
lowering for the fast tiers.

**Fallback if the change cannot land.** The interim posture stays: the referee refuses loops by name and the
`strict` profile's bilateral rows adjudicate them; exact answers for small loops come from the bounded exact
linkage lane, outside the simulator.

## Acceptance criteria

- [ ] In FrankenSim: a loop constraint and a Pfaffian constraint can be added to a tree model; models without them are bit-identical to before.
- [ ] In FrankenSim: closure residual ‖g‖ at most 10⁻¹² m (and 10⁻¹² rad) and ‖G·v‖ at most 10⁻¹² in consistent units over 10⁵ steps on the four-bar fixture; a redundant constraint set is refused with its rank.
- [ ] Here: the referee lowers a `closure` to the exact constraint at the new pin, and the envelope of a referee run reports the closure residuals.
- [ ] Here: the exact-closure known answers below are rows of the referee's dynamics battery and pass; GATE-KIN-01 passes with them.
- [ ] Here: on a compliant closure of stiffness k, the fast tiers' closure error tends to the referee's exact result as k grows, monotonically on the fixture (recorded at three stiffnesses).
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `parallelogram_four_bar_is_a_pendulum`: two massless cranks of length L = 0.5 m, coupler of mass 1 kg; the coupler translates on a circle of radius L, so small oscillations have period 2π·√(L/g) = 2π·√(0.5/9.81) = 1.4185 s; expected within 10⁻⁴ relative at amplitude 0.01 rad, with the coupler's orientation constant to 10⁻¹² rad.
- `parallelogram_output_equals_input`: crank angles equal to 10⁻¹² rad at 10³ configurations.
- `four_bar_mobility_is_one`: Grübler count 3·(4 − 1) − 2·4 = 1; the constraint Jacobian has rank 2 on the three open-chain coordinates.
- `disk_rolling_down_incline`: solid disk (I = ½·m·r²) on a 10° incline with the rolling constraint; expected acceleration (2/3)·g·sin 10° = 1.1357 m/s² within 10⁻⁶ relative; a hoop (I = m·r²) gives ½·g·sin 10°.
- `upright_rolling_disk_traces_a_circle`: forward speed 1 m/s and yaw rate 0.5 rad/s; expected path radius 2 m, with A·v at most 10⁻¹² throughout.
- `redundant_closure_is_refused`: the same loop cut twice; expected a typed refusal naming the rank deficiency.
Here:
- `referee_closure_is_exact_after_adoption`: the parallelogram through `frob-model`; expected residual at most 10⁻¹² and the label `exact` in the envelope; at the pin, `FROB-REF-721` with cause `kinematic-loop` (exit 5) and the repair `--profile strict`.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_04.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_04 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob sim run --fixture <parallelogram|four-bar> --tier referee --seconds 2 --seed 5
--key fd04-<fixture> --json` on the two closure fixtures (registered by this bead in the fixture registry of
N24b-03; a run verb, so each call carries its own key), asserting the label `exact` and the closure residuals in
the envelope, and `frob design check <the fixture's design> --json` (BD-110), asserting that its mechanism stage
(stage 3: loop assembly and mobility) is `passed`, with mobility 1 for the four-bar.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_04/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-04`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the parallelogram pendulum period; the rolling accelerations (2/3)·g·sin α and ½·g·sin α; the circle of radius v/θ̇.
- Counterexample kept as a test: treating a tracked or skid-steer base as a rolling constraint. Exact lateral no-slip at stations a ahead of and behind the centre requires v_y + a·ω = 0 and v_y − a·ω = 0, hence ω = 0: such a base turns only by slipping. A model that puts two lateral rolling constraints on one body must be reported as unable to turn, and the design lint must say why.
- Negative controls: a loop solved without drift removal must exceed the 10⁻¹² residual within 10⁵ steps; a rolling constraint with the sign of the sine term flipped must fail the circle test.
- Determinism lanes: twice on one machine; debug and release; same-platform class.

## Considerations

- Constraint forces interact with contact: a closed-chain leg in stance has loop and contact multipliers in one solve. The sustained contact step's limits (fixed witnesses, 21 contacts) apply unchanged.
- A loop near a singular configuration (a four-bar at a dead point) has a rank-deficient G; the upstream change must refuse there by name, and this project's singularity analysis already predicts where.
- Symbolic elimination of four-bar and parallel-knee closures into reduced-coordinate kernels remains a Frontier option for templates with closed-form closures; this item does not decide it.
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

Appendix F.1 (row FD-FSIM-04), F.3; §16.4, §16.12 (kinematic loops), §18.9 (the nonholonomic model and the tracked-base counterexample), §33.8, Appendix C (symbolic elimination of loop closures); GATE-KIN-01.
