---
key: "FD-FSIM-21"
title: "upstream frankensim: a second polarization pair for the battery element of `fs-phs`"
type: "task"
priority: 3
milestone: "M2"
labels: ["m2", "suite", "sinew", "upstream", "frankensim", "solid", "review-notes-open"]
parent: "WP-09"
deps: ["BD-037", "BD-068", "BD-075", "N15b-28", "N15b-30", "N33b-04", "STD-E2E"]
needs: []
related: ["N15b-03", "N33a-19", "N33b-10", "NF-09"]
gates: ["GATE-SINEW-07"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§15.10", "§15.5", "§15.14", "§42", "§33.8"]
---
## Background

The battery model of FrankenRobotics is a two-RC equivalent circuit with open-circuit voltage against state of
charge z:

  V = OCV(z) − R₀·I − V₁ − V₂,   dVₖ/dt = −Vₖ/(Rₖ·Cₖ) + I/Cₖ (k = 1, 2),   dz/dt = −I/Q,

with I positive on discharge, R₀ the series resistance (a function of cell temperature), Q the capacity, and
the two resistor–capacitor pairs giving the fast and slow parts of voltage sag and recovery. Its stored energy
is ∫OCV dq + q₁²/(2·C₁) + q₂²/(2·C₂).

FACT (`frankensim/crates/fs-phs/src/circuit/battery.rs`, read 2026-11-21): FrankenSim's battery has a series
resistance, **one** polarization pair, a bus capacitor and a bounded state of charge that refuses on
exhaustion, and states that ageing, thermal drift and hysteresis need separate laws. The second pair and the
temperature dependence are therefore added on this side: on the referee by composing a further RC element, on
the batched tiers as kernels. Upstream register priority: **P3**.

OPEN-SINEW-4 asks whether the referee's second pair is composed from existing FrankenSim circuit elements or
needs an upstream change. Its fail-closed default: open this upstream work item and use an in-house `f64`
reference until it lands. Owner: airlock lead; decided at the admission test for `fs-phs`. Rejected: dropping
the slow pair on the referee (the referee would then disagree with the fast tiers on recovery after a load
step, by design, and battery parity would be meaningless).

## Scope

In: in `fs-phs`, a battery element with two polarization pairs (the present one-pair element unchanged); here,
replacement of the in-house reference on the referee. Out: the in-house two-RC element that is the interim
posture (N15b-03); the power bus, brownout events and the verb `actuator.bus` (BD-075, a dependency, because
the switch-over re-runs GATE-SINEW-07's brownout rows through it) and the battery card schema (BD-069); the
power-path end-to-end run that measures GATE-SINEW-07 at M1 (N15b-30, a dependency, whose fixtures the phase
`post` re-runs against the sibling element); temperature dependence of R₀ and ageing laws; the
decision of OPEN-SINEW-4 itself (N15b-28, a dependency: if it decides "compose", the referee switches to the
composed element there and this item stays open as a convenience item, a native second pair, at P3).

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

**The change (DESIGN).** A second element (or a pair-count parameter on the existing one, default 1) whose
state is (z, V₁, V₂), with the equations above, as a port-Hamiltonian component: storage
H = ∫OCV dq + ½·C₁·V₁² + ½·C₂·V₂²; dissipation R₀·I² + V₁²/R₁ + V₂²/R₂ ≥ 0; one electrical port (V, I). It keeps
the present element's bounded state of charge and its refusal on exhaustion, and it steps with the crate's
discrete-gradient method so that the energy balance closes to the crate's stated residual.

Additive: the one-pair element and its values are unchanged.

**Interim posture here.** An in-house `f64` reference for the second pair on the referee (a further RC element
composed beside FrankenSim's battery through the airlock's `PortHamiltonian` trait), labelled as in-house in
the model's provenance.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-21/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-21`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_21` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the airlock's `actuation` group for a two-pair battery element
(absent at the pin) and, when present, runs the step-response known answer below on it.

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

**What the switch-over re-runs.** GATE-SINEW-07 against the sibling element: battery and bus: the step
response's drop and both time constants within 1%; the ceiling E²/(4·R) and the voltage at a given power within
0.1%; a demand above the ceiling produces a brownout event in 100 of 100 trials and never a completed step;
exhaustion refuses. The in-house reference is then kept as a differential check in tests and removed from the
referee path.

**Fallback if the change cannot land.** The in-house reference stays on the referee, or the composed element if
OPEN-SINEW-4 was decided "compose from existing elements" (N15b-28's rule: the referee then uses the composed
element after GATE-SINEW-07 is re-run against it, and this item stays open as a convenience item at P3). The
row stays `open` in either case: `not-needed` is admitted by BD-037 only on the conditional row FD-FSS-01.

## Acceptance criteria

- [ ] In FrankenSim: the two-pair element exists, passes its own power-balance test (port power + dH/dt + dissipation = 0 to the crate's residual), and the one-pair element's tests are unchanged.
- [ ] Here: the airlock exposes it at the new pin and the referee's battery uses it; the model's provenance no longer says in-house for the second pair.
- [ ] Here: GATE-SINEW-07 passes against the sibling element, with the measured drop, both time constants, the ceiling and the brownout count logged beside the receipt hashes of the re-run. The re-run is a check: the gate's row stays the recording of N15b-30, its one recorder, and no `record_gate` call is made here.
- [ ] Here: the in-house reference and the sibling element agree on the step response to 10⁻⁹ relative over 600 s of simulated time.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-phs`):
- `two_pair_step_response`: R₀ = 0.05 Ω, R₁ = 0.02 Ω, C₁ = 500 F (τ₁ = 10 s), R₂ = 0.03 Ω, C₂ = 4,000 F (τ₂ = 120 s), a 10 A step at constant OCV. Expected: instantaneous drop 0.5 V; V₁(t) = 0.2·(1 − e^(−t/10)) V; V₂(t) = 0.3·(1 − e^(−t/120)) V; total drop tending to 1.0 V; both fitted time constants within 1%.
- `recovery_after_load_removal`: current set to zero at t = 600 s; expected V₁ and V₂ decaying with the same two time constants.
- `power_balance_closes`: over the step, the integral of port power equals the change of H plus the dissipated energy, to the crate's stated residual.
- `exhaustion_refuses`: z driven to its bound; expected the typed refusal, as for the one-pair element.
Here:
- `brownout_closed_form`: source voltage E = 48 V behind R = 0.08 Ω: ceiling E²/(4·R) = 7,200 W; at 1,800 W the bus sits at V = [E + √(E² − 4·R·P)]/2 = 44.78 V and carries 40.19 A; a demand of 7,300 W produces a brownout event and never a completed step.
- `inhouse_matches_sibling`: the differential check of the acceptance line.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_21.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_21 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post`
also re-runs the power-path fixtures of N15b-30, in that bead's command forms, with the referee's battery now the
sibling element: `frob task fixture battery-step --tier referee --key fd21-step --json` (the 40 A step:
immediate drop 40·0.05 = 2.0 V, fitted time constants 10 s and 120 s, each within 1%); `frob actuator bus
bus-48v --demand 1800W --source-voltage 48V --json` (ceiling 7,200 W, 44.785 V, 40.19 A, within 0.1%); `frob
actuator bus bus-48v --demand 7300W --source-voltage 48V --json` (exit 3, FROB-REF-133: the static verb refuses a
demand above the ceiling and raises no event); and `frob task fixture brownout-trials --tier referee --key
fd21-brownout --json` (the fixture's 100 seeded draws, one world each: a `brownout` event in each of the 100
worlds and no interval delivering the requested power). `sim.run` has no `--seeds` (one run has one root seed),
and a loop over `--seed` would not reproduce the fixture's draws of source voltage, resistance and demand, which
the registered manifest keeps with their known answers; `task.fixture` is a run verb, so its two calls carry keys
of their own, distinct from N15b-30's `pp-step` and `pp-brownout`, whose stored receipts they must not replay.
The brownout event is asserted on the run, never on the static verb.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_21/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-21`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the step response above; the brownout ceiling 7,200 W, 44.78 V and 40.19 A at 1,800 W (illustrative values).
- Negative controls: a one-pair element passed off as two (R₂ = 0) must fail the slow time constant; a sign error in dVₖ/dt must make the recovery grow and fail the power balance; a demand above the ceiling that returns a completed step must be caught.
- Determinism lanes: twice on one machine; debug and release; same-platform class on the referee.

## Considerations

- The decision bead for OPEN-SINEW-4 comes first: if existing FrankenSim circuit elements compose into a second pair with the energy balance closed, the referee needs no upstream change and uses the composed element; this item stays open as a convenience item (a native second pair) at P3, and its switch-over then replaces the composed element, which is kept beside the in-house reference as a differential check.
- Temperature dependence of R₀, hysteresis and ageing are separate laws both upstream and here; this item does not add them.
- The battery is a shared resource: an edit that touches the shared bus invalidates every global result of a design, which is why battery parity between tiers is a gate and not a courtesy.
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

Appendix F.1 (row FD-FSIM-21), F.3; §15.10 (the battery; brownout), §15.5 (the port-Hamiltonian form), §15.14 (OPEN-SINEW-4), §42; GATE-SINEW-07.
