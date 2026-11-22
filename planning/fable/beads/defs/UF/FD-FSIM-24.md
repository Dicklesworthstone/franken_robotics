---
key: "FD-FSIM-24"
title: "upstream frankensim: a cross-ISA statement for the ensemble lane of `fs-assimilate`"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "ground", "upstream", "frankensim", "frontier"]
parent: "WP-68"
deps: ["BD-037", "BD-388", "NF-01", "STD-E2E"]
needs: []
related: ["BD-391", "BD-396", "FD-FSIM-07", "FD-FSIM-23"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§30.4", "§30.21", "§30.18", "§35.1", "§33.8"]
---
## Background

Some of what Ground identifies is not Gaussian: stick–slip friction gives likelihoods with several modes, and a
linearized update misrepresents them. FrankenSim's `fs-assimilate` has a lane for that: a deterministic serial
ensemble square-root filter with forecast, correlated blocks and a fixed-lag smoother (`nonlinear::ensemble`,
behind the feature `ensemble`). FACT (read 2026-11-21): the ensemble lane is feature-gated and **claims no
cross-ISA determinism**: nothing says that the same data and seeds give the same posterior on x86-64 and on
aarch64. Upstream register priority: **P3**.

Why it matters. A calibration is evidence, and this project's replication kit promises that a third party can
rebuild a calibration on a clean machine "inside its class". If the class of the ensemble lane is unstated, a
kit rebuilt on another architecture has nothing to be compared against. The program has two machines of
different architectures (an Apple-silicon Mac and an x86-64 Linux host), so the question is not hypothetical.
Rejected: comparing ensemble posteriors across machines anyway and calling small differences noise (a class is
never raised by relabelling); forbidding the ensemble lane (it is the only route for multi-modal likelihoods).

## Scope

In: in `fs-assimilate`, a written statement of what the ensemble lane guarantees across instruction sets, with
a test that holds it; here, the cross-machine replay and its record. Out: deterministic math inside `fs-mbd`
(FD-FSIM-07); the streaming entry (FD-FSIM-23); the replication kit itself (BD-396).

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

**The change (DESIGN). One of two statements, FrankenSim's choice, each with its test.**

1. *Bitwise.* The lane's results are bit-identical across x86-64 and aarch64 for the same inputs and seeds. To
   hold it: random draws from `fs-rand`'s counter-based streams keyed by logical identity (member, step,
   variable), never by execution order; elementary functions from `fs-math`'s deterministic set; fixed
   reduction order in every ensemble mean and covariance; no fused multiply-add left to the compiler. A golden
   test stores the bytes of a small posterior and checks them on both architectures.
2. *Tolerance.* The lane states a bound: posterior means agree within k standard errors and variances within a
   stated relative tolerance across architectures, with the bound derived from the operations that may differ.
   A test checks the bound on a fixture run on both.

Either way the crate's contract gains the statement, and the lane's receipt names the class it ran under.
Additive: no result changes on the architecture where it is produced today.

**Interim posture here.** Ensemble results are recorded with the machine fingerprint and are not compared
across machines. A calibration built with the ensemble lane is scoped to the architecture named in its record.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-24/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-24`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_24` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It replays one ensemble update of the synthetic rehearsal on both
machines and compares posterior bytes; without the statement the comparison is recorded as "no class claimed"
and counts as failing.

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

**What the switch-over re-runs.** The rule is "re-run the replay known answers of the synthetic rehearsal on a
second machine and record the comparison in the calibration record". The rehearsal is a `synthetic` dataset
generated with planted parameters; its replay known answers are re-run on the Mac and on the Linux host under
the same receipt, and the calibration record gains a section with both fingerprints, the class claimed by the
sibling (bitwise or tolerance), and the measured difference against it. If the statement is bitwise and the
bytes differ, the first differing ensemble member and step are recorded and the claim is reported upstream as
violated.

**Fallback if the change cannot land.** Ensemble calibrations stay architecture-scoped; the replication kit
states the architecture as a requirement.

## Acceptance criteria

- [ ] In FrankenSim: the contract of `fs-assimilate` states the cross-ISA class of the ensemble lane, and a test on both architectures holds it.
- [ ] In FrankenSim: the lane's receipt names the class.
- [ ] Here: the replay known answers of the synthetic rehearsal are run on both machines under one receipt; the calibration record holds both fingerprints, the class and the measured difference.
- [ ] Here: a calibration is labelled portable across architectures only if the recorded comparison is inside the stated class; otherwise it stays scoped to its machine.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, behind `ensemble`):
- `ensemble_golden_bytes` (bitwise statement) or `ensemble_cross_isa_bound` (tolerance statement): a 16-member ensemble on a 2-state, 8-observation problem; the stored golden or the bound is checked on both architectures.
- `draws_are_keyed_not_ordered`: evaluating members in reverse order gives identical results.
- `linear_case_matches_kalman`: with a linear model and Gaussian noise the ensemble square-root filter's mean and covariance tend to the Kalman update's as the member count grows; at 4,096 members the mean is within 3 standard errors.
Here:
- `calibration_record_has_cross_machine_section`: both fingerprints, the class, the difference.
- `portable_label_requires_comparison`: a calibration without the section cannot be labelled portable.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_24.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_24 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. The script is run once on each machine; the second run reads the first run's log and
compares. Phase `post` also runs `frob ground identify --json` with the ensemble lane on the rehearsal dataset
and `frob ground kit --json` to confirm what the kit states about architecture.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_24/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-24`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. The machine fingerprint digest is logged with every posterior digest.

### Known answers and negative controls
- Known answers: the rehearsal's planted parameters; the Kalman limit of the linear case.
- Negative controls: a build of the fixture with one platform `exp` call restored, or with execution-order seeding, must break the bitwise golden on at least one architecture; a tolerance statement must fail on a planted perturbation of ten times its bound.
- Determinism lanes: this item is the cross-platform lane for the ensemble; in addition, twice on one machine and across worker counts.

## Considerations

- Determinism class rules of this project: the weakest input wins, and a class is never raised by relabelling, only by running again under a stricter profile. A calibration that used an unclassified lane inherits "unspecified across machines".
- A kit rebuilds the rehearsal on a clean machine inside its class (GATE-GROUND-09); with this item that class has a definition on the other architecture.
- Both machines must be on the same toolchain T and the same receipt for the comparison to mean anything; the one-toolchain rule provides that.
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

Appendix F.1 (row FD-FSIM-24), F.3; §30.4, §30.21 (the synthetic rehearsal), §30.18 (the replication kit), §35.1 (determinism classes), §33.8; GATE-GROUND-09.
