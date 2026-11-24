---
key: "FDEF-01"
title: "defect report, frankensim `ppo_update`: reversed clip branch, stale critic gradient, sequences across resets"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "cortex", "upstream", "frankensim", "solid", "test"]
parent: "WP-37"
deps: ["BD-034", "BD-037", "STD-E2E"]
needs: []
related: ["FDEF-06"]
gates: ["INT-FSIM-06"]
plan_refs: ["Appendix F.2 row 1", "§33.11", "§23.6", "§33.10", "Appendix F.3", "§0.1"]
---
## Background

The suite contains exactly one PPO implementation: `fs_g1_train::ppo_update` in FrankenSim's `fs-g1-train` crate,
which hand-rolls PPO with manual backpropagation and depends only on `libm`. It would be the natural oracle or
comparison baseline for this project's own on-device PPO. Three errors were reported in it. The plan's rule for
such reports: a reported defect in a foundation is a **FACT to recheck**; it becomes an admission test here
first (INT-FSIM-06, BD-034, which records what the pinned commit does) and an upstream work item there second
(FD-FSIM-10, which makes the change). This bead is the piece between the two: the defect report itself, with a
reproduction that runs inside FrankenSim without this repository, filed in FrankenSim's tracker.

None of the numbers below was executed for the plan. "Read in source" is what a reading on 2026-11-21 at commit
`483d09c682f2` confirms; a reported number stays a report until the admission test records it at the pin.

## Scope

In: the report (`tests/upstream/FD-FSIM-10/REPORT.md`, whose text is also the body of the mirrored bead in
FrankenSim's tracker); the standalone reproduction (`tests/upstream/FD-FSIM-10/repro/`); the observed values at
the pin copied from INT-FSIM-06's record; the guard this repository keeps. Out: the admission test (BD-034); the
fix (FD-FSIM-10); this project's own PPO and its battery (BD-197); the action sampler's joint law (FDEF-06).

## Technical approach

**The defect.** `frankensim/crates/fs-g1-train/src/ppo.rs:253–345`.

1. *Clip branch reversed for negative advantage* (lines 322–326; confirmed by reading). The clipped surrogate is
   min(ρ·Â, clip(ρ, 1 − ε, 1 + ε)·Â), with ρ the likelihood ratio, Â the advantage and ε the clip width. The
   unclipped term carries the gradient exactly when ρ·Â ≤ clip(ρ)·Â, for either sign of Â. The source tests the
   opposite inequality when Â < 0: it follows the ratio below 1 − ε and clips it above 1 + ε.
2. *Critic gradient from the stored value* (line 340; confirmed by reading). The gradient is formed from the
   value stored during the rollout; the value returned by the training forward pass is discarded.
3. *Episodes concatenated* (consistent with line 309; its number was not re-run). Two reset episodes are
   forwarded as one sequence, so recurrent state crosses the reset.

It also drives its advantage recursion with a single `done` flag, without separating termination from
truncation; that is recorded in the report as a fourth observation, not as part of the three probes.

**Smallest reproducing cases (the known answers of the report).**

| Case | Inputs | Correct | Reported from the code |
|---|---|---|---|
| Clip, ρ below the band | Â = −1, ε = 0.2, ρ = 0.5: terms −0.5 and −0.8 | The clipped term is the minimum; gradient 0 | Gradient flows: −0.25 |
| Clip, ρ in the band | Â = −1, ρ = 1 | −0.5 | −0.5 |
| Clip, ρ above the band | Â = −1, ρ = 2: terms −2 and −1.2 | The unclipped term is the minimum; gradient −1 | Clipped: 0 |
| Critic | Critic output 2, stored value 0, return 0 | Gradient 2 into the critic bias | 0 |
| Sequence | Two independently reset one-step episodes, frozen policy | Second likelihood ratio 1 | 1.582459 |

The three clip values are finite differences of the surrogate in the log standard deviation. One construction
that reproduces them (DESIGN, to be confirmed by the reproduction): a single Gaussian action whose standardized
deviation z satisfies z² − 1 = ½, so that ∂ρ/∂log σ = ρ·(z² − 1) = ρ/2 and the unclipped derivative is Â·ρ/2,
which is −0.25, −0.5 and −1 at ρ = 0.5, 1 and 2.

**The reproduction (standalone).** A single Rust test file that depends only on `fs-g1-train` at the pinned
commit and can be dropped into that crate's `tests/` directory. It builds the smallest network the crate
allows, sets parameters by hand, and prints a table of expected and observed values for the five cases. It uses
the crate's public entry points only; where a case cannot be reached through them, the report says which
internal function the reading refers to and the reproduction asserts on the observable consequence.

**Proposed fix (carried by FD-FSIM-10).** (i) Take the gradient through the unclipped term when
ρ·Â ≤ clip(ρ, 1 − ε, 1 + ε)·Â and zero otherwise, with one inequality for both signs of Â. (ii) Form the critic
gradient from the value returned by the training forward pass. (iii) Reset recurrent state at every episode
boundary in the sequence forward, or evaluate episodes separately. Additive form: a corrected entry point
beside the old one, with the old one documented as defective; whether the old one is then corrected in place is
FrankenSim's decision under its own rules, since its exported checkpoint and its own bead (which says the
trainer "proves model/training/export plumbing, not learned humanoid locomotion") refer to it.

**Admission test and pass condition (INT-FSIM-06, run by BD-034).** Three probes: the four combinations of
advantage sign and clip side, each against a finite difference of the scalar loss to 10⁻³ relative in single
precision; the critic gradient against the current forward output; a packed sequence against separate episode
replays, ratio 1 to 10⁻⁶. The test fails at the pin if the defect is real and passes after the fix.

**The guard this repository keeps meanwhile.** `fs-g1-train` is not linked, is not an oracle and is not a
baseline (`fs-g1-train` is "not admitted" in the airlock's crate list, and `xtask check-deps` refuses a
dependency on it). PPO is written here, and the same three probes are the first tests of this project's own
PPO (GATE-CORTEX-07: the battery passes for our implementation, and three planted defects that reproduce the
sibling's errors each fail it), because a defect this easy to write is easy to write twice.

**Procedure.** (1) Copy the observed values and the commit from INT-FSIM-06's admission record into the report
under `observed_at`. (2) If a reported value does not reproduce at the pin, say so in the report: that case is
then "not reproduced", and if none reproduces the report is filed as "not reproduced at `<rev>`", FD-FSIM-10 is
closed as not needed and the guard is reviewed. (3) Write and run the standalone reproduction in a scratch clone
at the pin (never in FrankenSim's checkout). (4) File the report as the mirrored bead of FD-FSIM-10 in
FrankenSim's tracker, under its `AGENTS.md`, and store the identifier in `registries/upstream.toml`.

## Acceptance criteria

- [ ] `tests/upstream/FD-FSIM-10/REPORT.md` has the sections Defect, Evidence (file, lines, commit), Smallest reproducing case, Expected and observed, Proposed fix, Interim posture, and an `observed_at` commit with the values the admission test recorded.
- [ ] The reproduction compiles against `fs-g1-train` alone at the pinned commit and prints all five cases; each case is marked reproduced or not reproduced.
- [ ] The report is filed in FrankenSim's tracker and its identifier is the `mirror` of FD-FSIM-10's registry row.
- [ ] `cargo xtask check-deps` fails a scratch manifest that adds `fs-g1-train` as a dependency of any crate.
- [ ] No known answer, oracle or baseline in this repository cites `fs-g1-train` (a search of `frob-conformance` registries finds none).

## Test plan

### Unit tests
- `repro_clip_three_ratios`: Â = −1, ε = 0.2, ρ ∈ {0.5, 1, 2}; expected correct values 0, −0.5, −1 to 10⁻³ relative; observed values compared with the report.
- `repro_critic_uses_current_output`: output 2, stored 0, return 0; expected 2.
- `repro_reset_boundary_ratio`: two one-step episodes; expected second ratio 1 to 10⁻⁶.
- `fs_g1_train_is_not_a_dependency`: `cargo metadata` of the core workspace contains no package named `fs-g1-train`.

### End-to-end script and logging
`scripts/e2e/defect_fdef_01.sh`: clones FrankenSim's pinned commit into `target/upstream-scratch/frankensim@<rev>`,
copies the reproduction into the clone's `crates/fs-g1-train/tests/`, runs it, parses the table and asserts
that each case's status equals the status written in the report; then runs `cargo xtask check-deps` on the
planted manifest and `frob doctor --json` to confirm the registry row and its mirror.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/defect_fdef_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FDEF-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-773 and KA-774 are the table above (tolerance 10⁻³ relative in single precision for the clip cases; ratio 1 to 10⁻⁶). CX-357: "a sibling's training code can serve as an oracle or a baseline" is the shortcut this report refutes (finite differences −0.25, −0.5, 0 where 0, −0.5, −1 are correct; critic gradient 0 where 2 is correct); its sequence case is also CX-252 ("concatenated episodes may be evaluated as one sequence": ratios 1 and 1, 1.582459 read across the reset; rule P-6).
- Controls inside the reproduction, each about twenty lines of scalar arithmetic run through the same comparison as the sibling. Positive control: a corrected reference of the three rules must produce the "correct" column and be classified "not reproduced". Negative control (the planted defect the reproduction must catch): a copy of the reported logic (the reversed inequality for Â < 0, the stored value in the critic gradient, recurrent state carried across the reset) must produce the "reported" column for the clip and critic cases (−0.25, −0.5, 0; 0) and a second ratio different from 1, and be classified "reproduced". If either control is misclassified, the reproduction is wrong and its verdict on the sibling is void.
- Determinism lanes: the reproduction twice on one machine prints identical values; debug and release agree to the stated tolerance.

## Considerations

- RISK-24: a reported foundation defect is relied on before it is rechecked. This report is the artefact that turns the report into a record; until `observed_at` is filled in, every number above is a report.
- Premortem story 24 (a foundation defect surfaced late) has the residual "unreported defects exist": the report template is reusable for the next one.
- The fix is upstream work and no milestone waits for it; if it never lands, nothing changes here, because nothing depends on the sibling trainer.
- When the fix lands, it is adopted only through its switch-over rule (re-run INT-FSIM-06; only then may the sibling trainer serve as a comparison baseline); neither a pin nor the toolchain moves during the ten-fold tournament.
- The lesson FrankenSim's own trainer teaches is a gate rule of this project: a training gate tests learned behaviour against a negative control, never plumbing (GATE-CORTEX-01).

## References

Appendix F.2 (row 1), F.3; §33.11 (row 1, the hand cases), §23.6, §33.10; INT-FSIM-06; FD-FSIM-10; GATE-CORTEX-07; KA-773, KA-774; CX-357; RISK-24.
