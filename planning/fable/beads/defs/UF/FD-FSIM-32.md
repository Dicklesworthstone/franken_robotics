---
key: "FD-FSIM-32"
title: "upstream frankensim: `fs_g1_train::gaussian_action` corrected so that action channels are drawn independently"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "cortex", "upstream", "frankensim", "solid"]
parent: "WP-48"
deps: ["FDEF-06", "N23b-30", "N33b-04", "STD-E2E"]
needs: []
related: ["N33c-37", "N33c-47"]
gates: ["GATE-CORTEX-18", "GATE-WS-06"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 6", "Appendix F.3", "§33.17", "§23.25", "§41"]
---
## Background

FrankenSim's `fs-g1-train`, the suite's only PPO trainer, samples actions with `gaussian_action`
(`frankensim/crates/fs-g1-train/src/ppo.rs:172–189` at `483d09c682f2`). Each channel is drawn by a Box–Muller
cosine over a counter advanced by 0x9E3779B97F4A7C15, with the phase uniform taken from the top 53 bits of the
counter times 0x94D049BB133111EB. The phase is a linear function of the counter modulo 1, so consecutive draws
differ in phase by the fixed fraction 0.167245 of a turn, an angle φ = 0.334490·π. For a uniform angle θ, cos θ
and cos(θ + φ) have the same sign with probability 1 − φ/π, so adjacent channels agree in sign with probability
**0.6655 where independence gives ½, with correct marginals**. The trainer scores these samples with a product
of marginal densities: its likelihood ratio is computed for a variable that was not the one sampled. The defect
report is FDEF-06. Upstream register priority: **P2**.

This project does not link the sampler; a planted copy of it is the negative control of its own joint-law
battery (rule P-12). The fix is upstream work for the same reason as the PPO update's (FD-FSIM-10): the sibling
trainer cannot be a comparison baseline while its sampler is wrong, and a defect found in the owner's code is
fixed where it lives. Rejected: correcting the marginal test upstream only (the marginals are already right;
that is the trap).

## Scope

In: the corrected sampler in `fs-g1-train` with joint-law tests there; the battery re-run here. Out: the defect
report (FDEF-06); this project's sampler, its manifest and the battery (the learning chapter's beads); the PPO
update (FD-FSIM-10).

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

**The change (DESIGN).** The joint law of the sampled action must be the product Gaussian the likelihood
assumes: across the channels of one decision, across consecutive decisions, and across environments. Any of
these does it, and FrankenSim chooses:

1. each Box–Muller pair consumes two uniforms obtained from **distinct, well-mixed counters** (for example the
   output of a 64-bit finalizer applied to (stream, decision, channel, 0) for the radius and to
   (stream, decision, channel, 1) for the phase), so that no draw is a linear function of a neighbour's counter;
2. or both outputs of one pair (r·cos θ and r·sin θ, which are independent normals) serve two channels, with
   fresh uniforms for every pair;
3. or a counter-based generator keyed per (decision, channel) replaces the additive counter.

The marginals, the action bounds and the public signature stay. Additive form: a corrected sampler beside the
present one, the present one documented; FrankenSim's rules decide about correcting in place, since its
exported checkpoints were trained with the present sampler.

**Interim posture here.** Not linked. A planted copy (the two constants and the cosine, about twenty lines, in
`frob-conformance`, labelled as a negative control) is the battery's negative control. This project's draws
are counter-based outputs keyed by `(WorldAddr, decision index, stream id, channel)`.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-32/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-32`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_32` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It runs the joint-law battery against the sibling's sampler at the pinned
commit, in a scratch clone and without linking it into any product binary; it fails there if the defect is real.

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

**What the switch-over re-runs.** The joint-law battery against the corrected sampler (rule P-12,
GATE-CORTEX-18), with its four statistics over N decisions: the sign-agreement frequency of every pair of
channels; the same for one channel at consecutive decisions; the same for one channel in adjacent
environments; and the empirical covariance of the standardized noise. Under a product law every agreement
frequency has mean ½ and standard deviation ½/√N. The gate's terms (GATE-CORTEX-18, M2): action dimensions 2,
12 and 29, N = 10⁵ decisions and 64 worlds; every channel-pair, consecutive-decision and adjacent-world sign
agreement in ½ ± 0.005 (about 3.2 standard deviations); every off-diagonal entry of the standardized covariance
below 4/√N = 0.01265 in magnitude; the number of pairs tested is recorded so that the family-wise rate is
stated (29 channels give 406 channel pairs). Only after it passes may the sibling trainer serve as a
comparison baseline (together with FD-FSIM-10's condition for the update). The planted copy stays as this
project's negative control whatever happens upstream (the sampler row of GATE-WS-06, at M2).

**Fallback if the change cannot land.** Nothing changes here; the sibling trainer never becomes a baseline.

## Acceptance criteria

- [ ] In FrankenSim: the corrected sampler passes joint-law tests there (all pairwise sign-agreement frequencies inside ½ ± 0.005 at N = 100,000, with the number of pairs tested recorded; every off-diagonal entry of the standardized covariance below 4/√N = 0.01265 in magnitude, the threshold of GATE-CORTEX-18), and its marginal tests still pass.
- [ ] In FrankenSim: the present sampler is either corrected or documented as having a non-product joint law.
- [ ] Here: the battery passes against the corrected sampler at the new pin, with the four statistics recorded; the planted copy is still rejected in 100 of 100 runs.
- [ ] Here: the sibling trainer is registered as a comparison baseline only when both this item and FD-FSIM-10 are adopted; `cargo metadata` of the core workspace still shows no package named `fs-g1-train`.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-g1-train`):
- `adjacent_channels_are_independent`: N = 100,000 two-channel draws for each of three seeds; expected sign agreement in ½ ± 0.005 (three standard deviations are 0.0047); the present sampler gives 0.6655.
- `channels_two_apart_are_independent`: expected ½ ± 0.005; the present sampler gives 1 − 2φ/π = 0.3310.
- `consecutive_decisions_are_independent`: one channel at decisions t and t + 1; expected ½ ± 0.005.
- `standardized_covariance_is_identity`: 12 channels, N = 100,000; every off-diagonal entry below 4/√N = 0.01265 in magnitude (the gate's bound; one standard deviation is 1/√N = 0.00316, and a 0.01 band would fail a correct sampler on about one seed in ten across 66 pairs) and every diagonal entry within 0.02 of 1 (4.5 standard deviations of a sample variance).
- `marginals_unchanged`: mean within 0.01 of 0, variance within 0.02 of 1, and a Kolmogorov–Smirnov statistic below its 1% critical value against the standard normal.
Here:
- `battery_against_sibling_sampler`: the fixture; expected pass at the new pin and the recorded failure (the offending pair and its frequency) at the old one.
- `planted_copy_is_still_rejected`: 100 of 100.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_32.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_32 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. The sibling's sampler is exercised in a scratch clone at the pin
(`target/upstream-scratch/frankensim@<rev>`); the metric lines carry each agreement frequency, N, the seed and
the band ½ ± 0.005.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_32/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-32`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the phase step (0x9E3779B97F4A7C15·0x94D049BB133111EB mod 2⁶⁴)/2⁶⁴ = 0.167245; agreement 1 − φ/π = 0.665510 for adjacent channels and 0.331020 for channels two apart; reported counts 66,552, 66,552 and 66,551 of 100,000.
- Negative controls: the planted copy must be rejected by the battery; a "fix" that only reseeds per call but keeps the additive counter within a call must still fail `adjacent_channels_are_independent`.
- Metamorphic relation: permuting the channels of a product-law sampler changes no statistic.
- Determinism lanes: counts are exact integers and identical on every run and platform for fixed seeds.

## Considerations

- Program risk RISK-58 (high): a sampler with correct marginals and a wrong joint law; its planted instance is the GATE-WS-07 row at M2 (N41-38). Earliest signals: a pairwise sign-agreement frequency outside ½ ± 0.005; a learning curve that changes when channels are reordered.
- The fraction is arithmetic on constants in the source, so it is a strong prediction; it is still recorded at the pin before anything cites it.
- A corrected sampler changes the sibling's training trajectories; checkpoints from before and after are not comparable, which the report states.
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

Appendix F.1 (row FD-FSIM-32), F.2 (row 6), F.3; §33.17 (row 6), §23.25 (rule P-12); GATE-CORTEX-18, GATE-WS-06; RISK-58, RISK-24.
