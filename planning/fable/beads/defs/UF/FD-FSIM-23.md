---
key: "FD-FSIM-23"
title: "upstream frankensim: `fs-assimilate` windowed or streaming entry to the iterated update beyond 256 rows"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "ground", "upstream", "frankensim", "frontier", "review-notes-open"]
parent: "WP-68"
deps: ["BD-037", "BD-391", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-24", "FD-FSIM-25", "N30a-05", "N30a-22", "N30b-17", "N33b-13"]
gates: ["GATE-GROUND-04"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§30.4", "§30.9", "§33.9", "§33.8"]
---
## Background

Ground is the subsystem that ties simulation to measured data: it identifies actuator and contact parameters
from recorded traces and updates the cards a design is built from. Its estimators come from FrankenSim's
`fs-assimilate` (about 13 thousand lines, layer L4, depends on `fs-qty`): Gaussian beliefs and linear
observations with sequential Kalman fusion, a UD-factor update with an interval-checked receipt, and, for
nonlinear models, an iterated Gauss–Newton Kalman update with Armijo line search, a correlated-noise variant
and a Huber-loss fit (`nonlinear::iterated`).

FACT (read 2026-11-21): the iterated update admits **at most 256 rows**, 256 iterations and 65,536
evaluations; dense state is at most 256; sequential fusion takes at most 4,096 observations per call. A
recorded trace of one joint at 1 kHz for ten seconds is 10⁴ rows. Upstream register priority: **P3**.

So long traces are windowed on this side: they enter the update as windows of summary features, and the window
definition is part of the calibration record. That works, and it puts a modelling choice (which features,
which windows) between the data and the estimator. Rejected: calling the 256-row update repeatedly on raw
chunks and chaining the posteriors as if the chunks were independent (rows that share one bias do not count as
independent measurements; the crate's own shared-calibration groups exist to prevent exactly that).

## Scope

In: in `fs-assimilate`, an entry that runs the same iterated update over more than 256 rows, by windows or by
streaming, with the same receipts; here, a both-ways re-fit of one dataset. Out: regressors, identifiability
and excitation design, which are this project's (BD-390, BD-391); the ensemble lane (FD-FSIM-24); promotion of
a posterior to a validated state, which the crate rightly leaves to "a separate admission act".

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

**The change (DESIGN).** One Gauss–Newton iteration of the update needs, over all rows, the sums
A = Σ JᵢᵀRᵢ⁻¹Jᵢ and b = Σ JᵢᵀRᵢ⁻¹rᵢ, with Jᵢ the Jacobian block, Rᵢ the noise covariance and rᵢ the residual of
block i. A streaming entry takes the rows in blocks of at most 256, accumulates A and b in a fixed order with
compensated summation, and solves once per iteration; the line search evaluates the cost by the same pass. For
blocks that are independent given the state the result equals the single-call update on the same rows. Rows
that share a declared common-mode source are kept in one group across blocks, as the crate's `groups` module
requires, so that streaming cannot push uncertainty below the common-mode floor. The receipt lists the blocks,
their order and the per-block contribution. The caps on state dimension, iterations and evaluations stay, and
the row cap becomes a cap per block with a stated maximum number of blocks.

Additive: the 256-row entry is unchanged.

**Interim posture here.** Traces enter as windows of summary features; the window definition (length, overlap,
features) is part of the calibration record and of the identity of the fit. The cap of 256 rows binds one
call, so a calibration set is assimilated one experiment per call, the posterior of one call being the prior
of the next, with every shared calibration bias carried in the state (N30a-22). That chaining is not the
rejected one: it keeps the shared bias as one unknown across calls, and for a linear model it equals the
single update over all rows.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-23/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-23`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_23` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It submits 1,000 scalar rows to the iterated update through the airlock
trait `Assimilation`; at the pin it receives the row-cap refusal.

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

**What the switch-over re-runs.** The rule is "re-fit one recorded dataset both ways and compare posteriors":
one admitted dataset (the synthetic rehearsal's, if no real one is admitted) is fitted once through windows of
summary features and once through the streaming entry on raw rows. The two posteriors are compared parameter
by parameter (difference of means in units of the pooled standard deviation; ratio of standard deviations), and
the comparison is stored in the calibration record. GATE-GROUND-04 is re-run with the streaming entry (planted
actuator parameters within 10% on at least 95 of 100 synthetic rigs; identified cards cut held-out trajectory
error by at least 30% against prior cards; the omitted-law candidate is rejected in at least 90 of 100). The
windowed route stays available, since summary features are sometimes the right model.

**Fallback if the change cannot land.** Windows of summary features, as today.

## Acceptance criteria

- [ ] In FrankenSim: given one block of at most 256 rows, the streaming entry reproduces the single-call update bit for bit; given the same rows split into several blocks, it agrees with the single call to 10⁻¹² relative (the summation order differs, so bit equality is not claimed across block sizes); it matches the closed form on the linear-Gaussian case below at 10³ and 10⁴ rows.
- [ ] In FrankenSim: with a declared shared bias, posterior variance does not fall below the common-mode floor however many rows are streamed.
- [ ] Here: the airlock exposes the entry at the new pin, reached with `--windows raw`; a fit's calibration record states which route was used.
- [ ] Here: the both-ways comparison is recorded for one dataset; GATE-GROUND-04 passes with the streaming entry.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `streaming_equals_batch_small`: 200 rows as one block; expected bit equality with the single call. The same 200 rows in blocks of 64 (four blocks); expected agreement of posterior mean and covariance to 10⁻¹² relative.
- `linear_gaussian_closed_form`: scalar state with prior N(0, 1); 1,000 observations yᵢ = x + noise of variance 4; expected posterior variance 1/(1 + 1000/4) = 1/251 = 0.003984 and mean (Σyᵢ/4)/251, to 10⁻¹² relative. At 10⁴ observations (40 blocks of 250): variance 1/2501 = 3.9984·10⁻⁴ and mean (Σyᵢ/4)/2501, to the same tolerance.
- `common_mode_floor_holds`: the same with a shared bias of variance 0.01 declared; expected posterior variance of x no smaller than the floor implied by the bias (at least 0.0099 as rows tend to infinity; checked at 10⁵ rows).
- `block_order_is_fixed`: permuting the order in which blocks are supplied does not change the result, because the entry sorts by block identity.
Here:
- `calibration_record_names_route`: `windowed` or `streaming`, with the window definition or the block list.
- `raw_without_streaming_entry_refused`: on a scratch airlock listing with no streaming entry (the state of the old pin), `--windows raw` answers `FROB-REF-724` naming FD-FSIM-23, exit 5, with no fit started and no calibration record written.
- `both_ways_comparison_is_stored`: the record holds one row per parameter.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_23.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_23 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob ground identify` twice on the rehearsal dataset, once
per route: `frob ground identify <rehearsal dataset> --stage actuators --windows <def> --key fd23-win --json` and
`frob ground identify <rehearsal dataset> --stage actuators --windows raw --key fd23-raw --json` (a run verb: the
dataset is the positional input, the stage is named so that the second call re-fits the same stage and not the
next one, and the two compared calls carry distinct keys), and asserts the route named in each calibration record
(`windowed`, `streaming`) and the stored comparison. How a caller chooses the route is the `ground.identify`
row's option `--windows <def | raw>` (VERBS-ground; implemented by BD-391, extended by this bead with `raw` at
M5): a window definition is the windowed route, as today; `raw` sends the raw rows through the streaming entry.
At a pin whose airlock has no streaming entry, `raw` answers `FROB-REF-724` naming FD-FSIM-23, exit 5, before any
fit, so the interim posture cannot be bypassed.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_23/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-23`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the posterior variance 1/251; the synthetic rehearsal's planted parameters (recovered within 10% after the designed excitation).
- Negative controls: chaining 256-row posteriors as if independent, with a shared bias present, must report a variance below the floor and be caught by `common_mode_floor_holds`; a streaming entry with naive summation must lose the last digits at 10⁵ rows against the compensated one.
- Determinism lanes: bit-identical twice on one machine at a fixed block size (agreement to 10⁻¹² relative across block sizes, as above); cross-machine comparison is not claimed for this crate's ensemble lane (FD-FSIM-24) and is claimed here only as far as the crate's contract states.

## Considerations

- `fs-assimilate` depends on `fs-qty`, whose nightly coupling is a named risk; Ground's jobs are whole jobs of seconds to minutes, so the worker-process fallback costs them nothing.
- No dataset may be admitted when M5 opens (premortem story 26); the comparison then runs on the synthetic rehearsal and says so. Reality stays `no-claim` until real data is admitted.
- More rows do not mean more information when excitation is poor; identifiability analysis remains the gate on a fit, whichever route feeds it.
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

Appendix F.1 (row FD-FSIM-23), F.3; §30.4 (what the siblings provide), §30.9 (identification; windows), §33.9 (INT-FSIM-14), §33.8; GATE-GROUND-04.
