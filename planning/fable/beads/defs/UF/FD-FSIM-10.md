---
key: "FD-FSIM-10"
title: "upstream frankensim: correct `fs_g1_train::ppo_update` (clip branch, critic gradient, episode boundaries)"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "cortex", "upstream", "frankensim", "solid"]
parent: "WP-37"
deps: ["FDEF-01", "NF-01", "STD-E2E"]
needs: []
related: ["BD-034", "BD-037", "BD-197", "FD-FSIM-32", "BD-218"]
gates: ["INT-FSIM-06"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 1", "Appendix F.3", "§33.8", "§33.11", "§23.6"]
---
## Background

FrankenSim's `fs-g1-train` crate holds the suite's only PPO implementation, hand-rolled with manual
backpropagation and depending only on `libm`. Three errors were reported in its update,
`fs_g1_train::ppo_update` (`frankensim/crates/fs-g1-train/src/ppo.rs:253–345` at `483d09c682f2`): the clip
branch is reversed for negative advantage; the critic gradient is formed from the value stored at collection
and ignores the current output; and two reset episodes concatenated are forwarded as one sequence. The first
two were confirmed by reading (lines 322–326 and 340); the third is consistent with line 309. The defect report
is FDEF-01 and the admission test that records the behaviour at the pin is INT-FSIM-06. Upstream register
priority: **P2**.

This project does not link the crate: PPO is written here, on device, and passes the same three probes first.
The upstream fix is still this program's work, for two reasons. The sibling trainer is the only candidate for an
independent comparison baseline inside the suite, and it cannot serve while it is wrong. And every sibling is
the owner's: a defect found is a defect fixed where it lives. Rejected: patching a local copy for use as a
baseline (a fork whose results nobody else can reproduce); using the uncorrected trainer as a baseline with a
caveat (a comparison against a known-wrong learner says nothing).

## Scope

In: the corrected update in `fs-g1-train`, its tests there, and the re-run of INT-FSIM-06 here. Out: the defect
report (FDEF-01); the admission probes (BD-034); this project's PPO (BD-197); the action sampler's joint law
(FD-FSIM-32); the single-`done`-flag advantage recursion, which the report notes and which is fixed with the
episode-boundary change only as far as the sequence forward requires.

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

**The change (DESIGN).** With ρ the likelihood ratio π_new(a|s)/π_old(a|s), Â the advantage and ε the clip
width, the clipped surrogate is S = min(ρ·Â, clip(ρ, 1 − ε, 1 + ε)·Â).

1. *Clip branch.* The gradient passes through the unclipped term exactly when ρ·Â ≤ clip(ρ, 1 − ε, 1 + ε)·Â,
   and is zero otherwise, with one inequality for both signs of Â. (The source tests the opposite inequality
   when Â < 0.)
2. *Critic gradient.* The value loss is ½·(V_θ(s) − R)², with V_θ(s) the value returned by the training forward
   pass and R the return target; its gradient with respect to the output is V_θ(s) − R. The stored value from
   collection is used only where the algorithm calls for it (value clipping, if enabled), never in place of
   the current output.
3. *Episode boundaries.* In the sequence forward, recurrent state is reset at every episode start, or episodes
   are evaluated separately, so that a frozen policy reproduces the likelihoods it had at collection.

Additive form: a corrected entry point beside the present one, with the present one documented as defective.
FrankenSim's rules decide whether the present one is then corrected in place; its exported checkpoint (an
all-zero policy head) and its own bead ("proves model/training/export plumbing, not learned humanoid
locomotion") refer to it.

**Interim posture here.** `fs-g1-train` is not a dependency, not an oracle and not a baseline. PPO is written
here and passes the same three probes first (GATE-CORTEX-07: the battery passes for our implementation, and
three planted defects that reproduce the sibling's errors each fail it).

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-10/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-10`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_10` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It is the three probes of INT-FSIM-06 pointed at the sibling's entry
point at the pinned commit: they fail there if the defect is real.

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

**What the switch-over re-runs.** INT-FSIM-06, three probes: the four combinations of advantage sign and clip
side, each against a finite difference of the scalar loss to 10⁻³ relative in single precision; the critic
gradient against the current forward output; a packed sequence against separate episode replays, ratio 1 to
10⁻⁶. Only after it passes at the new pin may the sibling trainer serve as a comparison baseline, and it is
then registered as one in `frob-conformance` with the pin; it remains unlinked from every product binary.

**Fallback if the change cannot land.** Nothing changes here. Training comparisons use the external incumbents
of the parity harness, and the suite has no internal PPO baseline.

## Acceptance criteria

- [ ] In FrankenSim: the corrected update passes the five hand cases (three ratios, critic, reset boundary) as tests in `fs-g1-train`, and its existing export and plumbing tests still pass.
- [ ] In FrankenSim: the present entry point is either corrected or documented as defective with a pointer to the corrected one.
- [ ] Here: INT-FSIM-06 passes against the sibling at the new pin, with the three probes' measured values recorded against 10⁻³ and 10⁻⁶.
- [ ] Here: the baseline registry of `frob-conformance` lists the sibling trainer with its pin only after that record exists; before it, a comparison that names it is refused.
- [ ] Here: `cargo metadata` of the core workspace still shows no package named `fs-g1-train`.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-g1-train`):
- `clip_negative_advantage_three_ratios`: Â = −1, ε = 0.2; at ρ = 0.5 the terms are −0.5 and −0.8 and the gradient is 0; at ρ = 1 it is −0.5; at ρ = 2 the terms are −2 and −1.2 and the gradient is −1 (finite differences of the surrogate in the log standard deviation, for an action with standardized deviation z, z² = 1.5); tolerance 10⁻³ relative.
- `clip_positive_advantage_three_ratios`: Â = +1 at the same ratios; expected 0.25, 0.5 and 0 (above the band the clipped term is the minimum).
- `critic_gradient_uses_current_output`: output 2, stored value 0, return 0; expected gradient 2 into the critic bias.
- `sequence_respects_reset`: two independently reset one-step episodes, frozen policy; expected both likelihood ratios 1 to 10⁻⁶ (the uncorrected code gives 1.582459 for the second).
Here:
- `int_fsim_06_against_sibling`: the three probes; expected pass at the new pin and the recorded failure signature at the old one.
- `baseline_registry_requires_admission`: a comparison naming the sibling trainer without an admission record is refused.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_10.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_10 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. The probes run in a scratch clone at the pin (`target/upstream-scratch/frankensim@<rev>`),
never linking `fs-g1-train` into a product binary.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_10/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-10`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-773: clipped surrogate at Â = −1, ε = 0.2: finite differences 0, −0.5, −1 at ratios 0.5, 1, 2; the reported code path gives −0.25, −0.5, 0. KA-774: critic gradient 2 (reported 0); ratio 1 on both reset episodes (reported 1.582459 on the second).
- Negative controls: three planted mutants, each reproducing one of the sibling's errors in an otherwise correct update, must each fail its probe (the same plants GATE-CORTEX-07 requires of this project's PPO).
- Metamorphic relation: scaling every advantage by a positive constant c scales the policy gradient by c and leaves the set of clipped samples unchanged (before any advantage normalization).
- Determinism lanes: the probes twice on one machine; debug and release within the stated tolerance.

## Considerations

- RISK-24: a reported foundation defect is relied on before it is rechecked. The order admission test, report, fix is what this item's dependencies encode.
- CX-357 stays in the counterexample registry after the fix: the lesson is general (a sibling's training code is not an oracle until admitted), not about this file.
- A corrected update changes what the sibling's trainer learns; its owner's checkpoints trained before the fix are not comparable with those after, which the report says so that nobody compares across the change.
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

Appendix F.1 (row FD-FSIM-10), F.2 (row 1), F.3; §33.8, §33.11 (row 1), §23.6; INT-FSIM-06; GATE-CORTEX-07; KA-773, KA-774; CX-357; RISK-24.
