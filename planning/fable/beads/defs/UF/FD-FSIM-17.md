---
key: "FD-FSIM-17"
title: "upstream frankensim: `fs-uq` batched multilevel entry point and importance sampling beyond Gaussian germs"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "suite", "chronicle", "upstream", "frankensim", "solid"]
parent: "WP-50"
deps: ["BD-037", "BD-124", "BD-264", "N16b-08", "N33b-04", "N33b-13", "STD-E2E"]
needs: []
related: ["FD-FSIM-18", "FD-FSIM-19", "N26a-08", "N26a-24", "SPEC-C-25"]
gates: ["GATE-CHRON-09", "INT-FSIM-14"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§10 C-25", "§16.8", "§29.11", "§26.9", "§30.4", "§33.9"]
---
## Background

FrankenRobotics promotes results from fast tiers to trusted ones with paired, multilevel estimators: a
three-level telescoping estimate E[Y₂] = E[Y₀] + E[Y₁ − Y₀] + E[Y₂ − Y₁] over a GPU tier, `batch-cpu` in `f64`
and the referee, with samples allocated N_ℓ ∝ √(V_ℓ/C_ℓ) from pilot variances V_ℓ and per-sample costs C_ℓ.
FrankenSim's `fs-uq` has the arithmetic (FACT: `mlmc::{mlmc_estimate, coupled_mlmc_summary}`,
`product_mlmc_allocation::allocate_mlmc`, linear control variates in `product_control_variate`; files
`fs-uq/src/mlmc.rs`, `product_mlmc_allocation.rs`). Two limits keep this project from using its drivers: its
multilevel driver takes a **per-call closure**, one sample at a time, where this project produces thousands of
worlds per device launch; and its importance sampling works on **independent Gaussian germs only**. Upstream
register priority: **P2**.

So the coupled estimators are written here, and `fs-uq` supplies allocation and control-variate arithmetic
only. The upstream item would let the sibling's driver be a second implementation of the same estimate, which
is worth having for a statistic that licenses promotions. Rejected: wrapping the closure driver in a host loop
(it serializes what the devices batch, and re-orders the samples).

## Scope

In: in `fs-uq`, a batched entry point for the multilevel driver and an importance-sampling entry that accepts
declared non-Gaussian laws; here, the differential run under GATE-CHRON-09 and the decision to keep or replace
our driver. Out: the tier-correction ledger and the paired estimators (BD-264), our three-level multilevel
estimate that the sibling's batched entry is compared against (N16b-08), and the airlock group `stats`
with its trait `Uncertainty` and gate INT-FSIM-14 (N33b-13): all three are dependencies, because the switch-over
compares the sibling driver against ours on GATE-CHRON-09's data through that trait; the sample caps
(FD-FSIM-18); splitting (FD-FSIM-19).

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

1. *Batched multilevel entry.* Beside `mlmc_estimate`, an entry whose callback receives a level and a slice of
   sample indices and returns the slice of paired differences Y_ℓ − Y_ℓ₋₁ (or one that takes the differences
   already computed, per level, with their costs). For the same seeds and the same sample order it returns the
   same estimate, variance and interval as the closure driver, bit for bit; the audited telescoping check is
   unchanged.
2. *Importance sampling beyond Gaussian germs.* An entry that takes, per sample, the log-density ratio
   log p(x)/q(x) supplied by the caller under a declared pair of laws, or a declared monotone transform from
   normal germs to the target marginals. The existing safeguards stay: a proposal fixed before the run or fitted
   on a separate pilot whose data are never reused; the effective-sample-size floor; refusal when weights
   degenerate.

Additive: the closure driver and the Gaussian entry are unchanged.

**Interim posture here.** The coupled estimators of the trust ladder and of the tier-correction ledger are
ours; `fs-uq` supplies allocation and control-variate arithmetic through the airlock group `stats`.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-17/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-17`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_17` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It calls the multilevel driver through the airlock trait `Uncertainty` with
a batch of 4,096 precomputed differences per level and expects one call per level; at the pin only the per-call
closure exists.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) write a new resolution receipt (`frob.stack-receipt/1`) under `receipts/`
with `cargo xtask check-stack --write-receipt --reason pin-move --gate INT-FSIM-01` (plain `check-stack` only
verifies and writes nothing); (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** GATE-CHRON-09 with the sibling driver in place of ours (tier correction equal
to the plain mean at n = N; type-I error at most α over 10⁴ null comparisons under optional stopping; coverage
at least nominal − 0.03; a planted between-tier reversal never established; referee episodes per established
verdict 16, 64 and 256 at the 50th, 95th and 99th percentile). **Keep ours if the intervals differ**: the two
drivers are compared on the gate's own data, interval by interval; any difference beyond rounding is recorded
with its first cause, and this project's driver stays the one that licenses promotions. INT-FSIM-14 is re-run
for the group.

**Fallback if the change cannot land.** Our driver alone, checked against closed forms.

## Acceptance criteria

- [ ] In FrankenSim: the batched entry returns bit-identical results to the closure driver on 10³ seeded problems; the non-Gaussian importance entry passes its unbiasedness test and keeps the effective-sample-size refusal.
- [ ] Here: the airlock exposes the batched entry at the new pin, with one call per level.
- [ ] Here: GATE-CHRON-09 is run with both drivers; the per-comparison intervals are tabulated side by side; the record states "replaced" only if every interval agrees to rounding, and "kept ours" otherwise with the first differing comparison.
- [ ] Here: INT-FSIM-14 passes at the new pin (the ranking-flip probability Φ(−Δ/√(s₁² + s₂²)) to 10⁻¹²; a multilevel estimate of E[x²] under a standard normal is 1 within its own stated interval; importance weights that miss the failure region trip the effective-sample-size floor and the estimate is refused).
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-uq`):
- `batched_equals_closure`: three levels, 4,096 samples each, fixed seeds; expected bit-identical estimate, variance and interval.
- `allocation_closed_form`: V = (1, 0.1, 0.01), C = (1, 10, 100); expected N_ℓ in ratio 100 : 10 : 1.
- `mlmc_second_moment_of_normal`: estimate of E[x²], x standard normal; expected 1 inside the stated interval in at least 95 of 100 seeded runs.
- `importance_non_gaussian_unbiased`: target exponential(1), proposal exponential(½), event x > 10; expected estimate within its interval of e⁻¹⁰ = 4.54·10⁻⁵.
- `importance_missing_region_refuses`: a proposal with no mass on the event; expected the effective-sample-size refusal.
Here:
- `doubly_robust_unbiased_by_enumeration`: for Y ∈ {0, 1} with mean θ, a constant prediction m and inclusion probability p = 0.1, E[Z] = m + p·(θ − m)/p = θ for every m; checked for θ ∈ {0.1, 0.5, 0.9}.
- `driver_comparison_reports_first_difference`: a planted change of sample order in one driver is reported with the comparison index.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_17.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_17 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also
runs GATE-CHRON-09 once per driver (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers --
GATE-CHRON-09`, then the same with `--features uq-driver-fsuq`, DESIGN: the feature that routes the multilevel
allocation and interval through the sibling's batched entry via the airlock trait `Uncertainty`) and asserts the
side-by-side table; then, from the user's side, `frob fidelity ledger --class <c> --tier-pair batch-cpu,referee
--json` and `frob eval promote <result> --method multilevel --key fd17-post --json` on the gate's fixture
campaign, asserting that the receipt's estimator version names the driver in force.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_17/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-17`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-769 (INT-FSIM-14): the closed-form ranking-flip probability and E[x²] = 1; negative control: importance weights on a proposal that misses the failure region trip the effective-sample-size floor.
- Arithmetic known answer: the saving of a paired estimate is 1/(1 − ρ²) at best, about 2 at correlation ρ = 0.7; a factor of 5 needs ρ ≥ 0.894.
- Negative controls: a batched entry that sums levels in a different order must fail bit equality; a telescoping sum with one level's sign flipped must be caught by the audited check; a reused pilot must be refused.
- Determinism lanes: bit-identical twice on one machine and across worker counts (the batched entry fixes its reduction order); recorded with the machine fingerprint.

## Considerations

- "Keep ours if the intervals differ" is deliberate: a promotion statistic does not change owner on a rounding-level disagreement that nobody has explained.
- The stats group adds 10 crates and about 113 thousand lines to the M1 closure when enabled (FACT), so it is compiled only by the crates listed for it.
- `fs-uq` has no conformal prediction and no Fisher-information rank test; neither is part of this item.
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

Appendix F.1 (row FD-FSIM-17), F.3; §10 C-25 (fidelity tiers are coupled estimators: foundations), §16.8, §29.11, §30.4, §33.9 (INT-FSIM-14); GATE-CHRON-09; KA-769.
