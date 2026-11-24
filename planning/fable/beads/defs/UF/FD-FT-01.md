---
key: "FD-FT-01"
title: "upstream frankentorch: Metal gateway extensions, stages 1 and 2 (async completion, typed buffers, offsets, more arguments)"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "suite", "spark", "upstream", "frankentorch", "frob-spark-metal", "solid"]
parent: "WP-19"
deps: ["BD-016", "BD-017", "BD-028", "BD-029", "BD-037", "N21b-32", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FT-02", "FD-FT-03", "NFp1-51", "NFp1-52", "N19c-20", "N21a-27", "N21b-04", "N21b-05", "N21b-06", "N21b-08", "N21b-09", "N21b-12", "N21b-24", "N21c-24", "N33c-39", "N33c-47"]
gates: ["INT-FT-01", "INT-FT-02", "GATE-SPARK-02", "GATE-SPARK-04"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§21.9", "§21.10", "§21.3", "§33.17", "§21.25", "§41"]
---
## Background

FrankenRobotics runs its batched physics on Apple silicon through Metal, and reaches Metal only through
FrankenTorch's `ft-kernel-metal`, a safe gateway to custom Metal Shading Language kernels (ADR-SPARK-09: Metal
is reached through `ft-kernel-metal`; missing gateway features are upstream work items done in FrankenTorch's
repository, never a local patch or a second binding; until they land the backend works inside the gateway's
present limits; kernels never use a grid-wide barrier). Upstream register priority: **P1**.

The gateway as it exists (FACT, version 0.1.3, `src/compute.rs`, `src/lib.rs`, read 2026-11-21): `Gateway::open`
returns the process-wide device and command queue; `library` compiles source under `MathMode::Safe`, which
disables fast math; `Library::pipeline` builds a compute pipeline and exposes `max_threads_per_threadgroup` and
`thread_execution_width`; buffers are `StorageModeShared` allocations made by `buffer_f32`, `buffer_u32` and
`buffer_zeroed`, read by `read_f32`, `read_u32`, `read_u8` and rewritten by `write_f32`, `write_u32`;
`Gateway::batch` encodes several kernels in order into one serial compute encoder of one command buffer, checks
read and write declarations for aliases, borrows every output exclusively until `submit` returns, and returns a
`Receipt` with the kernel count and the device's execution time. State is resident across command buffers.

Its limits, and what each costs the Metal tier today:

| Limit today (FACT) | Consequence here (the interim posture) |
|---|---|
| Completion is synchronous: `dispatch` and `submit` block | One `Batch` per chunk of control intervals; one blocked host thread per device |
| At most 31 buffer arguments per kernel (`MAX_ARGUMENTS`) | Arrays packed into a few slabs; kernels index through an offset table in a parameter buffer |
| Buffers bind at offset zero | Slab-relative indexing inside kernels; no sub-buffer views |
| Typed helpers for `f32`, `u32` and bytes out only | Fields upload once; 16-bit fields packed into `u32` words; rings read by copy |
| Compilation from source only; no function constants; no binary archive | Shape-class constants are `#define` lines prepended to the source; a warm start recompiles from a cached digest |
| The wait cannot be interrupted | A watchdog marks the device stalled and the supervisor restarts the worker |
| The receipt has a kernel count and device seconds only | Per-kernel timing needs one batch per kernel in profiling mode |
| The `metal` 0.33 binding is deprecated by its maintainers | Pinned as FrankenTorch pins it |

Rejected, and withdrawn as a fallback: a Metal binding of our own beside the gateway (OPEN-SPARK-5: not without
an owner decision; default: stay inside the gateway's limits and accept KILL-METAL's fallback).

## Scope

The register item FD-FT-01 has five stages that land separately (below). The item was split in polishing so
that each bead is one piece of work.

In: stages 1 (completion) and 2 (buffers and arguments), in FrankenTorch; here, the Metal backend moved onto each
as it lands, and the four gates re-run; the item's one register row (`mirror`, `open` → `landed`) and its
fixture with one assertion per probe. Out: stages 3 and 4, function constants, binary archives, private storage
and per-kernel timing (NFp1-51, which depends on this bead); stage 5, Metal 4 command allocators and the
`objc2-metal` migration, and the row's move to `adopted` (NFp1-52, which depends on NFp1-51); the Metal backend
itself (BD-174); device tensors and autograd in FrankenTorch (FD-FT-02); a CUDA gateway (FD-FT-03).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankentorch` repository
(`../frankentorch` beside this one; head `8165d6a18028` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. FrankenTorch calls `ft-kernel-metal` its sanctioned unsafe boundary (FACT:
`frankentorch/crates/ft-kernel-metal/src/lib.rs`), so any new `unsafe` stays inside that crate and follows its
safety documentation.

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

**The change, in five stages (DESIGN; each is additive and keeps every existing compile-fail guarantee).**

1. *Completion.* Asynchronous submission with several command buffers in flight (a `submit_async` that returns
   a handle whose completion yields the `Receipt`), and an interruptible wait (a wait with a deadline or a
   cancellation token). Outputs stay exclusively borrowed until their command buffer completes.
2. *Buffers and arguments.* Typed buffers beyond `f32` and `u32` (at least `u16`, `i32`, `u64` and raw bytes
   both ways); binding at a byte offset, with alignment checked; relief from the 31-argument limit (argument
   tables or an equivalent, with the alias check extended to offset ranges).
3. *Compilation.* Function constants (specialization without editing source text); a binary archive and
   ahead-of-time compilation, keyed so that a stale archive is a miss and never a wrong kernel.
4. *Storage and timing.* Private storage with blit encoders to shared snapshots inside the same command
   buffer; per-kernel timing in the `Receipt`.
5. *Platform.* Metal 4 command allocators; migration from the deprecated `metal` 0.33 binding to `objc2-metal`.

This bead carries stages 1 and 2, which bear on BET-09 and land first. Stages 3 and 4 are NFp1-51 and stage 5 is
NFp1-52; all five remain one register row, FD-FT-01, with one mirrored bead. The row reads `landed` from the
first stage merged on FrankenTorch's `main` until every stage has been adopted here; NFp1-52 sets `adopted`.

Part of this is wrapping, not new binding work: the `metal` 0.33.0 binding under the gateway already exposes
indirect thread-group dispatch and thread-group memory length (`metal-0.33.0/src/encoder.rs:1715`, `:1730`) and
is reported to expose blit encoders and binary archives (FACT to recheck). Its language-version enumeration
ends at Metal Shading Language 3.1 (`library.rs:363`), so dialect sources use no construct newer than 3.1 until
stage 5.

**Rules on this side that must keep holding through every stage.** R1: the host reads a device slab only after
the command buffer that last wrote it has completed and returned its receipt. R2: a terminal state other than
`Completed` invalidates every slab the batch could write; the unit of commit is the chunk, which reads from the
current one of two committed slabs and ends by writing the other; only a `Completed` receipt makes the other
current; on failure, restore and resubmit at most four times with growing waits, then refuse with
`FROB-REF-607`. R3: with private storage, results are blitted to a shared snapshot slab inside the same command
buffer and read under R1; managed storage is not used. R4: a run is labelled native on a device only if its
receipt carries a completed command buffer with start and end times from that device. R5: timers cover
completion, never enqueue alone. R6: thread-group sizes are multiples of the execution width read from the
pipeline.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FT-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FT-01`, compiled as the `frob-conformance` integration test `upstream_fd_ft_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It has five assertions covering stages 1 to 4 (stage 2 has two): two
command buffers in flight at once (stage 1); a buffer bound at a non-zero offset and a kernel with 40 buffer
arguments (stage 2); a function constant (stage 3); a receipt with per-kernel times (stage 4). All five are
impossible through the gateway at the pin. `expected.toml` holds one signature per assertion, tagged with its
stage, so that after a partial adoption the fixture still fails, on the first assertion whose stage is not yet
adopted, with that assertion's recorded signature; each stage bead updates the file when its stage is adopted.
NFp1-52 adds a sixth assertion for stage 5.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) write a new resolution receipt (`frob.stack-receipt/1`) under `receipts/`
with `cargo xtask check-stack --write-receipt --reason pin-move --gate INT-FT-01` (plain `check-stack` only
verifies and writes nothing); (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the interim
posture of the limits that stage removes end; the row's `status` becomes `adopted` only after the last stage's
switch-over (NFp1-52). No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**When the Metal backend does not exist yet (DESIGN, so that this bead never waits on later beads).** This bead
depends only on the M0 admission (BD-016, BD-028), so a stage may be adopted before the Metal backend (BD-174 and
its components N21b-05 to N21b-07) or the gates GATE-SPARK-02 (BD-176) and GATE-SPARK-04 (BD-217) exist. Then the
switch-over runs INT-FT-01, INT-FT-02 and the fixture only; the backend beads are built on the pinned gateway as
they specify (their interim posture stays their design of record), and moving the backend onto a stage's new API
(several command buffers in flight, offset binding, argument tables) is a lever of the one optimization pass that
KILL-METAL allows (BD-217, N21b-08; option B of OPEN-SPARK-5, N21b-09), landing only with identical digests and a
paired lower bound of at least 1.03. When the backend does exist at adoption, the move and the re-tests below are
part of this bead.

**What the switch-over re-runs, after each stage that changes the backend.** INT-FT-01 (the gateway builds in
the Metal annex, with FrankenTorch's `block` patch repeated at the annex root, and runs a kernel on this Mac;
the record states the language version the gateway compiles under). INT-FT-02 (a buffer stays resident across
two command buffers; the gateway returns a completion receipt from the device; a device tag alone earns zero
operator coverage). GATE-SPARK-02 (every M2 kernel and the full step agree between `batch-cpu` in `f32` and the
device inside the declared class: bit-equal for class D1; for class D3 at most 4 ulp per kernel output,
statistical equivalence of 1,000-step rollouts and the decision-level test). GATE-SPARK-04 (Metal throughput at
least 25% of one RTX 4090 at matched settings on this M5 Ultra, reported beside an incumbent with a Metal path
measured on the same machine).

**Fallback if the change cannot land (KILL-METAL, decided at M2).** Trigger: GATE-SPARK-04 fails after one
optimization pass, or the Metal backend cannot hold run-to-run determinism. Fallback: Metal becomes a
development and replay tier (single-device debugging, small batches, the viewer); CUDA is the training tier;
the Mac trains on `batch-cpu` using its CPU cores and keeps its world-bank, proof and replay roles.

## Acceptance criteria

- [ ] In FrankenTorch, per stage: the new API exists with tests; every existing test, including the compile-fail tests that make R1 a compile-time fact, still passes; any new `unsafe` is inside `ft-kernel-metal` with its safety note.
- [ ] Stage 1: two command buffers are in flight at once and complete in submission order on the fixture; a wait with a deadline returns `TimedOut` without invalidating a later completion.
- [ ] Stage 2: the offset-bound and 40-argument probes run; an offset that breaks alignment or overlaps a declared write range is refused.
- [ ] Here: after each adopted stage INT-FT-01 and INT-FT-02 are re-run and recorded; where the Metal backend and the gates exist at that pin, GATE-SPARK-02 and GATE-SPARK-04 are re-run, each recorded only by its recorder in the gate register (N21b-35 and BD-217) from the receipt with `record_gate`, rules R1 to R6 are re-tested, and launch counts and blocked host time per chunk are recorded before and after; where they do not, the record names each as `first run against the new pin` with the bead that will make it.
- [ ] Here: no code in `spark/frob-spark-metal` binds Metal outside the gateway (a source scan), at every pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `landed` once stage 1 or 2 is on FrankenTorch's `main`, and stays `landed`, with the stages not yet adopted named, until NFp1-52 sets `adopted`.
- [ ] After stages 1 and 2 are adopted, the fixture's three stage-1 and stage-2 assertions pass and the stage-3 assertion fails with its recorded signature.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenTorch (added, in `ft-kernel-metal`; run on Apple silicon):
- `two_command_buffers_in_flight`: submit A then B asynchronously; both receipts arrive; B's outputs were not readable before B completed (compile-fail or runtime check, as the API makes possible).
- `interruptible_wait_times_out`: a kernel that runs about 200 ms, a 10 ms deadline; expected `TimedOut`, then a normal completion on a second wait.
- `bind_at_offset_reads_the_right_elements`: a 1,024-element `f32` buffer bound at byte offset 512; the kernel's element 0 is host element 128.
- `forty_arguments`: a kernel summing 40 one-element buffers holding 1.0 … 40.0; expected 820.0.
- `typed_buffers_round_trip`: `u16`, `i32`, `u64` and raw-byte buffers written and read back; expected bit equality, including `u64` values above 2³² and the `i32` value −1.
- `offset_misaligned_or_overlapping_is_refused`: an `f32` binding at byte offset 2, and an offset range overlapping a declared write range; expected a refusal for each.
Here:
- `readback_rules_r1_to_r6`: the backend's rule tests, re-run per stage.
- `chunk_commit_survives_aborted_buffer`: an injected non-`Completed` terminal state; the committed slab is unchanged and the retry reproduces the digest.

### End-to-end script and logging
`scripts/e2e/upstream_fd_ft_01.sh --phase pre|post [--stage N]…` (plain shell, seed-explicit, idempotent,
non-zero on failure). This bead writes it with stages 1 and 2; NFp1-51 and NFp1-52 add their stages to the same
script. Phase `post` for a stage asserts that stage's assertions pass and that every later stage's assertions
still fail with their recorded signatures.
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_ft_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `landed` with the adopted stages named and a receipt newer than
the pin move (`adopted` only after NFp1-52). Run on the Mac. Phase `post` also runs `frob spark probe --device
metal --suite all --key fdft01-probe-s<N> --json`, `frob spark fingerprint --device metal:0 --no-soak --key
fdft01-fp-s<N> --json` (which records the language version and the 64-bit atomic probe) and `frob spark check
--kernel K-cay-step --backend gpu-metal --key fdft01-check-s<N> --json` (the M0 single-kernel form of the
CPU-versus-device check), each a `run` verb under its own key per stage N (the three verbs are those of the M0
beads BD-017, BD-029 and N21b-32, dependencies of this bead); where the Metal backend and the
performance harness exist at that pin, it also runs the Metal row of the performance harness, logging launch
counts, blocked host seconds per chunk and world-steps per second with their thresholds, and otherwise logs that
step `skipped: not yet built` with the bead key.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_ft_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FT-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. Every line also carries `stage`.

### Known answers and negative controls
- Known answers: 1 + 2 + … + 40 = 820; host element 128 at byte offset 512 for `f32`; a kernel matches a Rust loop bit for bit on 10⁶ quaternions (the admission of INT-FT-01).
- Negative controls: a host read of a slab before its command buffer completes must be impossible to write or be refused (R1); a batch that ends in a non-`Completed` state must never make its slab current (R2); a misaligned or overlapping offset binding must be refused; phase `post --stage 3` run after stages 1 and 2 are adopted, while stage 3 is not, must exit non-zero (the per-stage signatures are really checked).
- Determinism lanes: every stage re-runs the device's run-to-run lane (same bits twice, and across worker counts) and the cross-backend lane against `batch-cpu` in its declared class.

## Considerations

- RISK-08 (the gateway is synchronous, takes at most 31 buffer arguments, binds at offset zero and compiles from source only) and premortem story 14 (the gateway's limits throttled the Mac tier): earliest signals are launch counts and blocked host time per chunk on Metal.
- BET-09 (Metal on Apple silicon is a real rollout tier, not a demo) is judged at GATE-SPARK-04; stages 1 and 2 are the ones that bear on it, and should land first.
- A compile probe on this machine is reported to show that Metal Shading Language 3.2 and 4.0 provide no 64-bit integer atomic add and no 64-bit compare-and-swap (FACT to recheck; the fingerprint re-runs it); stage 5 does not change what kernels may assume about that.
- The item is larger than one bead's usual five days, so its five stages were split along their natural lines: this bead (stages 1 and 2), NFp1-51 (stages 3 and 4) and NFp1-52 (stage 5). Each stage lands and is adopted separately; the register keeps one row and one mirrored bead for the item.
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

Appendix F.1 (row FD-FT-01), F.3; §21.9 (the gateway, the limits table, rules R1 to R6), §21.10 (the kernel cache), §21.3 (ADR-SPARK-09), §33.17 (row 8), §21.25 (OPEN-SPARK-5); INT-FT-01, INT-FT-02; GATE-SPARK-02, GATE-SPARK-04; KILL-METAL; RISK-08; BET-09; NFp1-51, NFp1-52 (the later stages).
