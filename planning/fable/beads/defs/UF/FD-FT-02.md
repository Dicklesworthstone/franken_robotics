---
key: "FD-FT-02"
title: "upstream frankentorch: real device execution for tensors and autograd (device tags become devices)"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "cortex", "upstream", "frankentorch", "solid"]
parent: "WP-21"
deps: ["BD-028", "BD-037", "BD-196", "N23a-14", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FT-03", "N23a-15", "N23a-16", "N23a-17", "N33a-22"]
gates: ["GATE-CORTEX-02"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§23.8", "§33.4", "§33.10", "§23.23", "§33.8"]
---
## Background

FACT about FrankenTorch (`frankentorch/crates/ft-core`, `ft-nn`, `ft-optim`, `ft-serialize`, read at
`8165d6a18028`; re-read at admission test INT-FT-02): tensors execute on the CPU only; `Device::Cuda` and
`Device::Metal` are metadata; it has a reverse-mode tape with higher-order gradients, Linear, GRU, LSTM and
attention modules, Adam and gradient clipping, a passing end-to-end MLP-with-Adam training test, safetensors
and a limited ONNX export; it has no PPO and no `vmap`. A device tag earns no GPU operator coverage. Upstream
register priority: **P2**.

Learning in FrankenRobotics has to keep observations on the device during a rollout, which is the main source
of GPU-simulator throughput. So on-device learning is written here: the policy forward pass, the backward pass
and the optimizer are kernels in the shared kernel dialect (matrix products, activations, Adam). FrankenTorch
has three jobs meanwhile: the CPU trainer for small runs and tests; the gradient oracle against which the
on-device backward kernels are differentially tested; and policy interchange through safetensors.

When FrankenTorch gains real device execution, the on-device trainer switches to it under the rule of this
item. Rejected: waiting for it before M2 (no milestone waits on a sibling); training on the CPU with per-step
readback as the design of record (it forfeits the throughput the device tier exists for; it is only
KILL-LEARNKERN's fallback).

## Scope

In: in FrankenTorch, tensors that live on a device and the operators and backward passes a PPO learner needs,
executed there, with honest per-device operator coverage; here, gradient parity and the switch of the trainer.
Out: the hand-written on-device kernels and the PPO rules (BD-196, BD-197); the Metal gateway's own extensions
(FD-FT-01); a CUDA gateway (FD-FT-03), on which CUDA execution depends.

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

**The change (DESIGN; FrankenTorch's own design decides the shape).** A tensor whose storage is a resident
device buffer obtained through FrankenTorch's kernel gateway (Metal first; CUDA once FD-FT-03 exists), and
device kernels for a stated operator set, at least: general matrix product; elementwise affine, `tanh`, ELU or
ReLU; sums and means along an axis; the Gaussian log-density of a diagonal policy; mean-squared error; the Adam
update. Each operator has a backward rule executed on the device, driven by the existing tape. Coverage is a
table generated from tests: an operator counts for a device only if a test ran it there and compared it with
the CPU result; a tensor tagged for a device whose operator is not covered falls back to the CPU with a typed
notice, never silently. Host readback is explicit. Additive: CPU execution and every existing test are
unchanged.

**Interim posture here.** On-device learning kernels in the shared dialect; FrankenTorch is the CPU trainer,
the gradient oracle and the interchange format.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FT-02/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FT-02`, compiled as the `frob-conformance` integration test `upstream_fd_ft_02` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks FrankenTorch, through the learning crate's FrankenTorch feature, to
run a matrix product on a tensor tagged `Device::Metal` and report where it executed; at the pin the answer is
the CPU (zero device operator coverage, as INT-FT-02 records).

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) write a new resolution receipt (`frob.stack-receipt/1`) under `receipts/`
with `cargo xtask check-stack --write-receipt --reason pin-move --gate INT-FT-01` (plain `check-stack` only
verifies and writes nothing); (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** "Gradient parity, then switch the trainer." GATE-CORTEX-02: on-device
backward kernels match FrankenTorch autograd within 10⁻⁵ relative on 10³ random batches; after adoption the
comparison is three-way (our kernels, FrankenTorch on the device, FrankenTorch on the CPU). The gate's entries are
BD-196's battery `cortex-grad` in the Gauntlet's parity tier (`crates/frob-conformance/src/batteries/cortex_grad.rs`,
collected by `tests/g2_parity.rs`); this bead registers one further case there, `ft-device`, an entry naming
GATE-CORTEX-02 at the `cuda` and `metal` sites that computes the same gradients with FrankenTorch executing on the
device and compares the three pairwise within 10⁻⁵ relative (a site without its device is `Skipped` with the
reason, never passed). Only when the
three agree, and FrankenTorch's device path reaches the training-time target of the reference task within the
parity band the hand-written kernels reached, does the trainer switch; the hand-written kernels stay as the
differential oracle. The PPO battery (rules P-1 to P-12) is re-run on the switched trainer.

**Fallback if the change cannot land.** The hand-written kernels remain the trainer. KILL-LEARNKERN, for
reference (trigger: GATE-CORTEX-02 fails after one repair cycle, or the training-time gate GATE-CORTEX-05 is
missed by more than two times after one optimization pass, GATE-CORTEX-05 being wall-clock to the return
threshold of GATE-CORTEX-01 on one RTX 4090 at most 1.25 times an incumbent's for the comparable task, re-measured
on the same card, published context about 410 s; fallback: the forward pass stays on the device, rollout storage is
read back once per rollout and the learner runs on FrankenTorch on the CPU, with the lost throughput
disclosed; the trainer moves to FrankenTorch device execution if this item lands).

## Acceptance criteria

- [ ] In FrankenTorch: the operator set above executes on Metal with device-resident tensors; the coverage table lists exactly the operators with a passing device test; an uncovered operator falls back with a typed notice.
- [ ] In FrankenTorch: for each covered operator, device and CPU results agree within 4 ulp per output in `f32`, and backward results within 10⁻⁵ relative.
- [ ] Here: the three-way gradient comparison passes on 10³ random batches (GATE-CORTEX-02's tolerance).
- [ ] Here: the trainer switch is recorded as an ADR under `docs/adr/` with the before and after training times on the reference task; if FrankenTorch's device path is slower beyond the parity band, the record says "not switched" and the hand-written kernels stay.
- [ ] Here: a device tag alone still earns zero operator coverage in this project's records at any pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenTorch (added):
- `device_linear_layer_gradient`: y = W·x + b with W = [[1, 2], [3, 4]], x = (1, −1), b = 0 and loss ½·‖y‖²: y = (−1, −1); expected ∂L/∂W = [[−1, 1], [−1, 1]] and ∂L/∂b = (−1, −1), computed on the device, to 10⁻⁶.
- `device_adam_first_step`: learning rate 10⁻³, β₁ = 0.9, β₂ = 0.999, ε = 10⁻⁸, gradient g = (0.5, −2.0); the bias-corrected moments are g and g², so the first update is −10⁻³·g/(|g| + ε) = (−10⁻³, +10⁻³) to 10⁻⁶ relative (ε/|g| is at most 2·10⁻⁸ here; a gradient below 10⁻² would make ε visible at that tolerance, which is why the inputs are stated).
- `coverage_table_matches_tests`: generated table equals the set of operators with a device test.
- `uncovered_operator_falls_back_loudly`: an operator outside the set on a device tensor returns its CPU result with the typed notice.
- `gaussian_log_density_on_device`: mean 0, standard deviation 2, action 1; expected −½·(0.25) − ln 2 − ½·ln(2π) = −1.7371.
Here:
- `three_way_gradient_parity`: as in the acceptance line.
- `device_tag_earns_no_coverage`: the INT-FT-02 assertion, kept.

### End-to-end script and logging
`scripts/e2e/upstream_fd_ft_02.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_ft_02 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run (GATE-CORTEX-02's three-way comparison as Gauntlet entries, KNOWN_DRIFTS 1:
`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --features gauntlet-mutations --test g2_parity --
GATE-CORTEX-02`, asserting from the hash-named receipt under `target/gauntlet/receipts/` the worst relative error
per site and case against 10⁻⁵ and every registered control `caught`), and the registry row must read `adopted`
with a receipt newer than the pin move. Phase `post` also runs `frob train start quad-ref --task quad-flat-velocity --backend gpu-metal
--trainer <t> --steps <short budget> --seed 1 --key fdft02-tr-<t>-s<N> --json` for each trainer `<t>`:
`device` (the hand-written kernels) and `frankentorch-device` (FrankenTorch executing on the device, the
additive value of `train.start --trainer` that this item's switch-over adds beside N23a-14's `device` and
`frankentorch`, refused with `FROB-REF-724` on a build without the adopted pin), and logs time to the first
evaluation checkpoint and the gradient-parity metrics with their thresholds.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_ft_02/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FT-02`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the linear-layer gradient and the Adam first step above; the log-density −1.7371.
- Negative controls: a backward kernel with a transposed operand must fail parity; a "device" operator that copies to the host and back must be caught by the residency check (a device receipt must exist for it); a training run that passes while its policy head stays at zero must fail the learned-behaviour control (a training gate tests learned behaviour against a negative control, never plumbing).
- Determinism lanes: device results twice on one machine; against the CPU inside the declared class.

## Considerations

- The lesson of the suite's own trainer applies to this switch: plumbing that runs is not evidence of learning; the switch is judged on training time to threshold, not on a passing smoke test.
- FrankenTorch has no `vmap`; batched rollouts supply batch dimensions explicitly, so this item does not depend on one.
- Networks run in single precision; half precision is not adopted for policies of a few hundred units.
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

Appendix F.1 (row FD-FT-02), F.3; §23.8 (where learning runs), §33.4 (the note on frankentorch), §33.10 (INT-FT-02), §23.23 (KILL-LEARNKERN); GATE-CORTEX-02.
