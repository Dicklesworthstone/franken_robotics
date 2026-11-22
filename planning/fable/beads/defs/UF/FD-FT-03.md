---
key: "FD-FT-03"
title: "upstream frankentorch: a CUDA kernel gateway mirroring the Metal gateway"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "spark", "upstream", "frankentorch", "frob-spark-cuda", "solid"]
parent: "WP-18"
deps: ["BD-037", "BD-015", "NF-01", "STD-E2E"]
needs: []
related: ["BD-172", "BD-173", "BD-175", "BD-217", "FD-FT-01", "FD-FT-02"]
gates: ["GATE-SPARK-02", "GATE-SPARK-03", "INT-CUDA-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§21.9", "§21.2", "§33.4", "§21.25", "§33.8"]
---
## Background

FrankenRobotics has two GPU backends. On Apple silicon it goes through FrankenTorch's `ft-kernel-metal`, a safe
gateway that the suite sanctions as its unsafe boundary. On NVIDIA hardware there is no such gateway in the
suite, so `spark/frob-spark-cuda` sits directly on the third-party `cudarc` crate for the CUDA driver and NVRTC
bindings (posture: consume, annex only; admitted by INT-CUDA-01: the pinned `cudarc` builds without a CUDA
toolkit, loads the driver on the Linux host `trj`, and runs a kernel on both of its GPUs). Upstream register
priority: **P2**.

The asymmetry has costs. The two backends reach their devices through different safety arguments: one through
a reviewed gateway with compile-fail tests, one through this project's own unsafe capsule. FrankenTorch cannot
execute on NVIDIA devices at all (FD-FT-02). And the suite's working CUDA code lives in separate engine kits
that each bind the driver again. A CUDA gateway in FrankenTorch with the Metal gateway's shape would give the
suite one place for that boundary.

Rejected: putting device code in FrankenSim (its doctrine: it ships no accelerator backend, and in-process
foreign kernels are forbidden in its production paths, which is why the GPU annexes are in this repository);
waiting for a gateway before M2 (the direct annex is the design of record).

## Scope

In: in FrankenTorch, a crate with the Metal gateway's safe surface over CUDA; here, a differential run of the
CUDA backend on it. Out: the CUDA backend itself, its launch-graph capture and stream-ordered pools (BD-172);
third-party gaps in `cudarc` (graph capture and asynchronous pool bindings are settled at INT-CUDA-01 and are
not upstream work items; the annex binds the driver directly where a binding is missing, OPEN-SPARK-4).

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

**The change (DESIGN).** A crate beside `ft-kernel-metal` (working name `ft-kernel-cuda`) with the same
concepts and guarantees:

1. `Gateway::open(device_index)`: a context and a stream per device; device limits reported (total and free
   memory, compute capability).
2. `library(source)`: runtime compilation through NVRTC under a safe math mode: no fast math and no implicit
   fused multiply-add contraction, mirroring `MathMode::Safe`; a compile log on failure.
3. Typed resident buffers, with read and write helpers; state persists across batches.
4. `batch`: several kernel launches encoded in order on one stream, with read and write declarations checked
   for aliases and every output borrowed exclusively until completion, as on Metal; a `Receipt` with the kernel
   count and device execution time measured on the device.
5. Whatever FD-FT-01 adds to the Metal gateway's surface (asynchronous completion, offset binding, constants,
   per-kernel timing) is designed here from the start, so the two stay mirrors.

All `unsafe` stays inside the crate, under FrankenTorch's rules for its sanctioned boundary. Additive: a new
crate; nothing existing changes.

**Interim posture here.** `spark/frob-spark-cuda` on `cudarc`, in this repository's own unsafe capsule with
its `SAFETY.md`; the binary kernel cache holds compiled modules keyed by BLAKE3 over the expanded source,
options, prelude version, compiler identity, device architecture and driver version.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FT-03/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FT-03`, compiled as the `frob-conformance` integration test `upstream_fd_ft_03` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It looks for a CUDA gateway crate in FrankenTorch at the pinned commit
(absent) and, when present, runs the admission kernel of INT-CUDA-01 through it on both GPUs of `trj`.

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

**What the switch-over re-runs.** GATE-SPARK-02 (every M2 kernel and the full step agree between `batch-cpu`
in `f32` and the device inside the declared class: bit-equal for class D1; for class D3 at most 4 ulp per
kernel output, statistical equivalence of 1,000-step rollouts and the decision-level test) and GATE-SPARK-03
(the CUDA rows of the performance table inside their tolerances, with the disclosure block and the paired
lower bound), each run twice: on the direct annex and on the gateway. The backend moves onto the gateway only
if digests are identical and the paired lower bound of the throughput ratio is at least 0.98 (TARGET: the
gateway may cost at most 2%); otherwise the direct annex stays and the gateway serves FrankenTorch alone.

**Fallback if the change cannot land.** The direct annex on `cudarc`, as today. Without CUDA at all, the CUDA
tier is absent and the CPU and Metal tiers are unaffected.

## Acceptance criteria

- [ ] In FrankenTorch: the gateway crate builds without a CUDA toolkit installed, opens both devices of the Linux host, compiles under the safe math mode, and passes its own tests, including compile-fail tests for reading an output before completion.
- [ ] In FrankenTorch: a kernel run through the gateway matches a Rust loop bit for bit on 10⁶ quaternions on both GPUs.
- [ ] Here: GATE-SPARK-02 and GATE-SPARK-03 are recorded for both routes; per-world digests are identical between them.
- [ ] Here: the route decision is an ADR citing the paired lower bound against 0.98.
- [ ] Here: whichever route is chosen, exactly one crate in the CUDA annex's graph binds the driver (`cargo tree` shows no second binding).
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenTorch (added; run on the Linux host):
- `opens_both_devices`: two gateways with distinct device identities.
- `safe_math_has_no_fma_contraction`: a kernel computing a·b + c with a = 1 + 2⁻¹³, b = 1 − 2⁻¹³, c = −1 in `f32`; the exact product is 1 − 2⁻²⁶, which rounds to 1 in `f32`, so the uncontracted result is 0 and the contracted one is −2⁻²⁶; expected 0.
- `resident_buffer_persists`: written in one batch, read unchanged in the next.
- `alias_check_refuses_overlapping_write`: two kernels declaring writes to one buffer in a batch; expected refusal at encode time.
- `receipt_time_is_device_time`: a kernel of known duration; receipt time within 20% of the device's own event timing.
Here:
- `gateway_and_direct_digests_equal`: 1,000 episodes of the reference workload on each route.
- `single_driver_binding`: the `cargo tree` assertion.

### End-to-end script and logging
`scripts/e2e/upstream_fd_ft_03.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_ft_03 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Run on `trj`. Phase `post` also runs `frob spark probe --json`, `frob spark check --json`
and the CUDA rows of the performance harness on both routes, logging world-steps per second, the paired ratio
with its lower bound against 0.98, and the digests.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_ft_03/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FT-03`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the contraction case (1 + 2⁻¹³)·(1 − 2⁻¹³) − 1 = −2⁻²⁶ exactly, and 0 without contraction in `f32`; bit equality with a Rust loop on 10⁶ quaternions.
- Negative controls: a build of the probe kernel with contraction enabled must fail the safe-math test; a gateway that reports host wall time as device time must fail the timing test under an injected host delay; an A/A run of the direct annex against itself must give a ratio whose interval contains 1.
- Determinism lanes: same bits twice, across worker counts and on each of the two GPUs; against `batch-cpu` in the declared class.

## Considerations

- `trj` is heavily loaded, so measurements are noisy and compiles slow (RISK-09): runs are interleaved and admission is by paired lower bound.
- A gateway adds a layer between the backend and features it needs (launch-graph capture, stream-ordered memory pools); if the gateway cannot express them, the 2% rule will show it, and the direct annex stays.
- Whether the gateway itself sits on `cudarc` or declares the driver symbols is FrankenTorch's decision; the one-binding check on this side is what keeps two bindings out of one process.
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

Appendix F.1 (row FD-FT-03), F.3; §21.9 (backends), §21.2 (suite doctrine), §33.4 (FD-CUDA, FD-FT), §21.25 (OPEN-SPARK-4), §33.8; INT-CUDA-01; GATE-SPARK-02, GATE-SPARK-03; RISK-09.
