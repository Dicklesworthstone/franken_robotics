---
key: "FD-FSIM-02"
title: "upstream frankensim: a persistent contact time stepper (its open Moreau–Jean lane; barrier lane stays its own)"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "suite", "kinesis", "traction", "upstream", "frankensim", "solid"]
parent: "WP-22"
deps: ["BD-037", "BD-081", "BD-086", "N24b-03", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-12", "FD-FSIM-13", "N16a-05", "N33b-07", "NFp2-01"]
gates: ["GATE-FID-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§16.4", "§16.5", "§18.5", "§33.8"]
---
## Background

The referee tier is FrankenSim's articulated dynamics and contact solve, reached through the airlock. FACT
(FrankenSim at `483d09c682`, read 2026-11-21): `fs-mbd`'s contact step
(`articulated_contacts::step::step_articulated_frictional_contacts`) is a first-order kick-contact-drift step
that takes the contact set from the caller; `solve_frictional_contacts` is an event solve; and FrankenSim has
**no persistent contact time stepper**: its Moreau–Jean and barrier lanes are open beads without code. Upstream
register priority: **P1**.

What "persistent" means here: a stepper that owns, from one step to the next, the things a contact carries
through time: the active set, the warm-start impulses, the tangential (friction) memory, and a way to
checkpoint and restore them so that a retried interval is identical to a first attempt.

Because FrankenSim does not own these, this project does: `frob-dyn` wraps every referee interval in a
transaction whose restore set includes contact memory, and commits it behind a fence. That is the design of
record and it stays whatever lands. The upstream item matters because the referee is only as good as the
stepper beneath it: with a proper time-stepping scheme inside FrankenSim, impacts and persistent contacts are
resolved by one consistent law instead of by an event solve called step after step.

Rejected: building a barrier (incremental-potential) contact method here (barrier contact stays FrankenSim's
lane and is not built in this repository); lifting the event solve and extending it locally (a fork).

## Scope

In: FrankenSim's Moreau–Jean lane landed as a persistent stepper with a checkpoint interface; its use behind the
airlock trait `RefereeContact`; the pin move and the referee-side re-runs of the switch-over. Out: the one-time
re-baseline of the fidelity ledger against the new stepper (NFp2-01, which depends on this bead and on the M2
ledger measurement BD-212, so that this upstream work can start before the ledger exists); the transaction, the
fence, the retry ladder and event location (BD-081, BD-082, BD-083), which remain ours; witness migration
(FD-FSIM-12, BD-093); capacity above 21 contacts (FD-FSIM-13); the barrier lane.

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

**The change (DESIGN; the lane and its bead already exist in FrankenSim and its own plan governs).** A stepper
entry point beside the existing step, implementing the Moreau–Jean scheme at velocity level. With M the mass
matrix, v⁻ and v⁺ the generalized velocities before and after the step of length h, f the smooth forces, J the
contact Jacobian and λ the contact impulses over the step:

  M·(v⁺ − v⁻) = h·f + Jᵀ·λ.

For each active contact, with normal relative velocity u_N = J_N·v and restitution coefficient e,

  0 ≤ λ_N ⟂ u_N⁺ + e·u_N⁻ ≥ 0,

and Coulomb friction with coefficient μ: ‖λ_T‖ ≤ μ·λ_N, with the tangential impulse opposing sliding and any
admissible value when sticking. The stepper keeps, per body pair: the active set; the last impulses, used to
warm-start the next solve; the tangential memory under the declared friction law. It exposes
`checkpoint() → State` and `restore(State)`, both exact, so that a caller's transaction can retry.

Additive: the existing kick-contact-drift step and the event solve keep their behaviour and their golden values.

**Interim posture here.** `frob-dyn` owns the transaction, the warm starts, the friction memory and the commit
fence: a checkpoint at the start of each interval holds the physical state, actuator, thermal and battery
states, contact memory (witness pairs, contact frames, tangential memory, warm-start impulses, the active set),
film thickness, clock phases and packet queues, controller and estimator states, the ordinals of every random
draw, terrain edits, the energy-ledger accumulators, event counters and the command cursor; committed state
carries a monotone `commit_epoch`. Barrier contact is not built here.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-02/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-02`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_02` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks `RefereeContact` whether a persistent stepper with exact
checkpoint and restore is available (false at the pin) and, when it is, runs the restore known answer below
against FrankenSim directly.

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

**What the switch-over re-runs.** In this bead: the contact known-answer battery on the referee, the thin-barrier
and sill fixtures, and INV-KIN-02 on forced retries, all at the new pin; their record, with the receipt hash, is
what NFp2-01 starts from. The transaction stays: the new stepper is called inside it, through its checkpoint
interface. The referee's outputs change when its stepper changes, so the plan's switch-over rule also says that
the **fidelity ledger is re-baselined once**, with a written rationale. That step is NFp2-01's. The new referee
kernels make every (fast, `referee`) stratum a new stratum; it is measured on the frozen battery and frozen under
the cause `upstream:FD-FSIM-02` of the ledger's re-baseline record (N16bp1-01), and the old strata stay as they
were. GATE-FID-01 (M2 battery, `batch-cpu` against `referee`: median and 95th-percentile disagreement at most 2%
and 5% for base displacement over 2 s, 3% and 8% for energy per metre, 5% and 12% for stance impulse) is then
read against the new strata, and only then does this item's row become `adopted`; until then it is `landed`. If
this change is adopted before BD-212 has frozen the M2 envelopes, no re-baseline is needed: the M2 measurement is
simply made against the new stepper, and NFp2-01 records that.

**Fallback if the change cannot land.** Nothing changes: the transactional step above the existing solve is
already the referee at M1.

## Acceptance criteria

- [ ] In FrankenSim: the stepper exists beside the existing step; `restore(checkpoint())` followed by a step reproduces the first attempt bit for bit on 10⁴ seeded intervals.
- [ ] In FrankenSim: its own contact known answers pass on the new stepper (resting, sliding, impact with restitution), and the old step's golden values are unchanged.
- [ ] Here: `RefereeContact` reports the persistent stepper at the new pin and the transactional step calls it through checkpoint and restore; INV-KIN-02 (a retried interval equals a first attempt) still holds on 10⁴ forced retries.
- [ ] Here: the referee's contact battery and the thin-barrier and sill fixtures pass at the new pin.
- [ ] Here: the switch-over's referee-side record (the three re-runs, with the receipt hash) is on file for NFp2-01, which makes the one re-baseline of the fidelity ledger for this item and reads GATE-FID-01 against it; nothing in this bead waits on the ledger.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `block_on_incline_below_friction_angle_does_not_creep`: μ = 0.5, incline 20° (tan 20° = 0.364 < μ), 10⁴ steps of 1 ms; expected displacement below 10⁻⁹ m.
- `block_above_friction_angle_accelerates`: incline 35°, μ = 0.5; expected acceleration g·(sin 35° − μ·cos 35°) = 9.81·(0.5736 − 0.4096) = 1.609 m/s² within 10⁻³ relative.
- `ball_restitution_apex_ratio`: e = 0.8, drop from 1 m; expected successive apex heights in ratio e² = 0.64 within 10⁻³.
- `checkpoint_restore_is_exact`: bitwise equality of state and of the next step after restore.
- `warm_start_reduces_iterations`: a ten-box stack at rest; expected: the warm-started solve of step 2 takes no more iterations than step 1, and its impulses agree with a cold-started solve of the same step 2 to 10⁻¹⁰ relative (warm starting changes the work, not the answer).
Here:
- `transaction_uses_upstream_checkpoint`: with the capability reported, the restore set no longer duplicates the contact memory the stepper owns, and a forced retry reproduces the committed digest.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_02.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_02 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the referee-side re-runs
of this bead are made (the contact known-answer battery, the thin-barrier and sill fixtures of BD-086, INV-KIN-02
on forced retries), and the registry row must read `landed` with a receipt newer than the pin move (`adopted`
once NFp2-01 has run: GATE-FID-01 is read there, not here). Phase `post` also runs `frob sim run --tier referee
--fixture <incline|stack> --seconds 2 --seed 5 --key fd02-<fixture>-<a|b> --json` (`--fixture` names a registered
fixture, N24b-03) twice per fixture under distinct keys, the second in the conformance build with one
interval forced to retry, asserting that the digests of the retried run equal those of the undisturbed run (both
envelopes show `idempotent_replay: false`). The ledger reads before and after the re-baseline are in NFp2-01's
script.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_02/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-02`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the stick condition tan α ≤ μ; sliding acceleration g·(sin α − μ·cos α); apex ratio e²; exact restore.
- Negative controls: a stepper that drops tangential memory between steps must creep on the 20° incline; a `restore` that omits the warm-start impulses must fail bitwise equality. (That a second re-baseline for this item is refused by the ledger is NFp2-01's control.)
- Determinism lanes: twice on one machine, across worker counts, debug and release; same-platform class.

## Considerations

- The re-baseline is a deliberate, single exception to "frozen envelopes". It is allowed only for the two upstream items that change the referee's stepper (this one and FD-FSIM-12), and each uses it once. For this item it is made by NFp2-01, and no frozen envelope is edited: the new stepper's strata are new strata.
- If the Moreau–Jean lane lands before witness migration does upstream, the transactional step still migrates witnesses above the airlock; the two items are independent.
- RISK-31 and BET-34 (referee coverage): a persistent stepper does not by itself raise coverage, which is limited by fixed witnesses and capacity; it improves the law the referee adjudicates with.
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

Appendix F.1 (row FD-FSIM-02), F.3; §16.4 (limits table), §16.5 (the transactional step), §18.5 (barrier contact is not built here), §33.8; GATE-FID-01; INV-KIN-02; RISK-31; BET-34.
