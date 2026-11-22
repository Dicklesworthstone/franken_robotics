---
key: "FD-FSIM-07"
title: "upstream frankensim: deterministic elementary functions and cancellation checkpoints inside `fs-mbd`"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "kinesis", "gauntlet", "upstream", "frankensim", "solid"]
parent: "WP-14"
deps: ["BD-037", "BD-025", "BD-032", "NF-01", "STD-E2E"]
needs: []
related: ["BD-024", "BD-038", "BD-223", "BD-269", "FD-FSIM-24"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§35.1", "§16.4", "§33.3", "§33.8", "§38"]
---
## Background

Every result of FrankenRobotics records a determinism class: D0 exact; D1 bitwise across backends; D2 bitwise
per backend; D3 inside a recorded tolerance; D4 unspecified. Two rules govern them: the weakest input wins
(INV-NUM-06), and a class is never raised by relabelling, only by running again under a stricter profile.

The referee tier in `f64` aims at FrankenSim's cross-ISA class on CPUs, the same bits on x86-64 and on aarch64,
by using its deterministic elementary functions (`fs-math`: deterministic functions with error budgets in units
in the last place; `fs-ga` already routes through `fs_math::det`). FACT: FrankenSim's `fs-mbd` currently calls
platform `sqrt`, `sin` and `cos` and makes no cross-ISA claim. So a referee rollout on this Mac and the same
rollout on the Linux host may differ in the last bits, and after a contact event, in more. Upstream register
priority: **P2**.

The item has a second half. A referee rollout is a long call into `fs-mbd`, and jobs here are cancellable with
draining semantics through asupersync's `Cx`. Without checkpoints inside `fs-mbd`, cancellation is observed only
between calls.

Rejected: claiming cross-machine equality of referee digests without the change (a class raised by relabelling);
wrapping `fs-mbd` to substitute its math functions from outside (not possible without a fork).

## Scope

In: in FrankenSim, a mode of `fs-mbd` in which every elementary function is the deterministic one, and
cancellation checkpoints in its long loops; here, cross-ISA lanes in tier G5 of the Gauntlet and prompt
cancellation of referee jobs. Out: determinism of the batched tiers (their own gates); the ensemble lane of
`fs-assimilate` (FD-FSIM-24).

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

1. *Deterministic math.* Every call to a platform elementary function in `fs-mbd` (`sin`, `cos`, `sqrt` and any
   other found by a scan) is routed through `fs_math::det`, under a mode that is off by default so that existing
   numerics and golden values do not change (additive). `sqrt` is correctly rounded by the IEEE standard on both
   architectures; it is routed anyway so that the scan has no exception. In the mode, no fused multiply-add is
   formed that the other architecture would not form: sums and products are written so that contraction is
   never left to the compiler. The crate's contract then states the cross-ISA class for that mode.
2. *Cancellation checkpoints.* The articulated sweeps and the contact solve's iterations call a checkpoint of
   the `Cx` they are given at bounded intervals. A cancellation observed there returns a typed `Cancelled`
   outcome and leaves the caller's state exactly as it was before the call.

**Interim posture here.** The referee claims same-platform determinism only: its results are class D2 on one
machine and are not compared bit for bit across machines; cross-machine agreement of the referee is a tolerance
statement (D3). Cancellation of a referee job is observed between FrankenSim calls, which the transaction keeps
short (one base interval).

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-07/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-07`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_07` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It compares the digest of the G1 two-second referee rollout recorded on
the aarch64 Mac with the one recorded on the x86-64 Linux host under the deterministic mode (the mode does not
exist at the pin, and without it the digests are expected to differ: FACT to record, not assumed).

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

**What the switch-over re-runs.** The rule is "add cross-ISA lanes to G5". Tier G5 of the Gauntlet holds the
determinism lanes (twice on one machine; across worker counts; debug and release; across platforms and
backends, each against its class). Added lanes: referee digests of the G1 rollout and of the M1 contact and
dynamics batteries are bit-equal between this Mac and the Linux host under the deterministic mode. Only when
those lanes pass does the referee's contract claim the cross-ISA class, and results produced before adoption
keep the class they had.

**Fallback if the change cannot land.** The referee stays same-platform; adjudication that must be compared
across machines is re-run on one machine.

## Acceptance criteria

- [ ] In FrankenSim: a source scan of `fs-mbd` finds no platform elementary function reachable in the deterministic mode; default-mode golden values are unchanged.
- [ ] In FrankenSim: with a cancellation injected at each checkpoint index of a G1 step, the outcome is `Cancelled` and the state is bit-identical to the state before the call.
- [ ] Here: the G5 cross-ISA lanes exist and pass for the G1 rollout and the M1 batteries, with both digests and both machine fingerprints recorded.
- [ ] Here: a referee job cancelled mid-rollout drains within 100 ms on the G1 model (TARGET) and leaves the committed frontier consistent; resubmission reproduces the uncancelled digests.
- [ ] Here: no result recorded before adoption has its class raised.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `det_math_hash_is_platform_independent`: `sin` and `cos` through `fs_math::det` on 10⁶ seeded inputs in [−10³, 10³]; the BLAKE3 hash of the outputs equals a constant stored in the test, on both architectures.
- `mbd_step_bits_equal_across_isa`: one G1 forward-dynamics step in the deterministic mode; the output bytes equal a stored golden on both architectures.
- `cancel_at_every_checkpoint_restores_state`: as in the acceptance line.
- `no_platform_math_in_det_mode`: a build-time or scan test over the crate's source.
Here:
- `g5_cross_isa_lane_g1`: compares the two recorded digests; expected equal.
- `class_is_not_raised_by_relabel`: a stored result with class D2 same-platform, re-read after adoption, still reads D2.
- `referee_cancel_drains`: under asupersync's lab runtime, a cancel at each await point of a referee job; expected no torn chunk and identical digests on resubmission.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_07.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_07 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob sim run g1 --tier referee --seconds 2 --json` on both
machines (the script is run once per machine and the second run compares against the first's log), and
`frob job cancel` on a running referee job followed by resubmission.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_07/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-07`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. The machine fingerprint digest of each side is logged with every
digest compared.

### Known answers and negative controls
- Known answer: bit-equal digests across the two architectures for the same job hash.
- Negative controls: (i) a build of the probe with one platform `sin` call restored must be reported by the scan test; (ii) a planted one-ulp perturbation in one function on one machine must make the cross-ISA lane fail and name the first differing step; (iii) a checkpoint that returns `Cancelled` after having written a partial update must fail the restore test.
- Determinism lanes: this item is the lane; in addition, twice on one machine, across worker counts, debug and release.

## Considerations

- The deterministic functions cost time; the mode is used where a cross-machine referee comparison is wanted, and the per-step cost is recorded at adoption.
- Cross-ISA equality also depends on the compiler: both machines must build on the same toolchain T under the same receipt, which the one-toolchain rule already gives.
- A worker process built at another receipt refuses the job (`FROB-REF-791`), so a cross-ISA comparison across receipts cannot happen by accident.
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

Appendix F.1 (row FD-FSIM-07), F.3; §35.1 (classes; INV-NUM-06), §16.4 (limits table), §33.3 (`fs-math`), §33.8, §38 (tier G5).
