---
key: "FD-FSIM-19"
title: "upstream frankensim: an adaptive multilevel splitting estimator beside the importance sampler of `fs-uq`"
type: "task"
priority: 3
milestone: "M4"
labels: ["m4", "suite", "nemesis", "upstream", "frankensim", "frontier", "review-notes-open"]
parent: "WP-55"
deps: ["BD-037", "BD-277", "N26a-32", "N33b-04", "STD-E2E"]
needs: []
related: ["N26a-28", "N26b-22", "N26bp1-01", "N33b-13"]
gates: ["GATE-NEMESIS-05"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§26.9", "§10 C-27", "§26.16", "§33.9"]
---
## Background

Rare failures are found in FrankenRobotics by two lanes. The certification lane (crude sequential sampling and
bounded-weight importance sampling) gives valid bounds. The exploration lane finds failures faster and gives
`Estimated` rates only; its main tool is **splitting**: run N trajectories, keep those that come closest to
failure by a distress score, branch them with fresh randomness, and repeat level by level. FrankenSim's `fs-uq`
has importance sampling on Gaussian germs and no splitting estimator (FACT; OPEN-NEMESIS-4 records the absence).
So splitting is implemented here, in `frob-nemesis`, on Chronicle's forks, with its own unbiasedness fixture.
Upstream register priority: **P3**. Tier of the mechanism: Frontier.

Why an upstream estimator is still wanted: a rare-event estimator is easy to get subtly biased (a branch that
reuses its parent's random stream is an identical copy; a level rule that looks at the future leaks), and a
second implementation run on a fixture with a closed-form answer is the cheapest independent check. Rejected:
treating our splitting estimate as certified (its trials are dependent and its accuracy rests on the score).

## Scope

In: in `fs-uq`, an adaptive multilevel splitting estimator on a generic simulator interface with a
closed-form fixture and its tests there; the mirrored bead in the sibling's tracker; here, the item's failing
fixture `tests/upstream/FD-FSIM-19/` kept current, the script `scripts/e2e/upstream_fd_fsim_19.sh` with its two
phases, and the registry row moved from `open` to `landed`. This bead closes when the estimator is on the
sibling's `main`.

Out, with the bead that has it:
- the switch-over on this side once the estimator has landed: N26bp1-01 (Procedure B, split from N26b-24 in
  polishing round 1), which depends on this bead. It admits the new entry through the airlock (with the duplicate-children control), runs the
  differential test of the two estimators on the random-walk fixture and on the fixture of the exploration
  gate, applies the rule that chooses one, records the choice as an architecture decision record, re-runs
  GATE-NEMESIS-05 with the chosen estimator, runs this bead's script in phase `post`, and moves the row to
  `adopted`. The differential test and the choice rule are specified there and not here.
- the admission test that records the absence of a splitting estimator at the pin, and the decision record of
  OPEN-NEMESIS-4: N26b-22, a related bead of M3 (the admission test comes first and the upstream item second,
  which the milestones already give; the link is not a blocking one, because this item's own fixture is opened
  by BD-037 and nothing in the sibling change needs the Nemesis tests).
- the row itself and the form of every fixture: BD-037.
- this project's splitting on forks and its fork classification (BD-277, BD-205); the certification lane
  (BD-276); the sample caps (FD-FSIM-18).

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

**The change (DESIGN).** An estimator over an abstract process: a state, a step that consumes keyed
randomness, a score S(state) and a target level. The last-particle variant: keep N particles; at each
iteration find the particle with the lowest maximum score, record that level, discard it, and replace it by a
branch of a surviving particle taken at the first time the survivor exceeded that level, continued with fresh
randomness keyed by (iteration, particle). Stop when every particle has reached the target. After J iterations
the estimate is p̂ = (1 − 1/N)^J, which is unbiased for any score. It returns the estimate, J, the level
history and a variance estimate from independent replicas, and it labels its output as an estimate with
dependent trials, never as a bound.

Additive: a new module; the importance sampler is unchanged.

**Interim posture here.** Splitting is implemented in `frob-nemesis` with its unbiasedness fixture. Fixed-level
arithmetic used for planning: with L levels of conditional probability p₀ and N particles, p = p₀^L and the
coefficient of variation is about √(L·(1 − p₀)/(p₀·N)); L = 6, p₀ = 0.1 and N = 10⁴ give 7%.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-19/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-19`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_19` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the airlock's `stats` group for a splitting estimator (absent at
the pin) and, when present, runs the random-walk fixture below through it. The fixture is not the admission
test: `no_splitting_estimator` (N26b-22, in `frob-fsim`) passes at the pin by recording the absence, and is
named in the row's `admission_test` field; this fixture fails at the pin by asking for the estimator, and is
the row's `fixture`.

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

**Who runs the switch-over.** The register's rule for this item is a **differential test of the two estimators
on the random-walk fixture, then choose one**. The steps of the train that are particular to this item are one
procedure, written once, in N26bp1-01 (Procedure B): the admission of the new entry, the differential test (both
estimators on the same closed-form probability over 10⁴ repetitions at equal budget; unbiasedness, coverage,
agreement of the two means, variance per simulated step), the comparison on the fixture of the exploration
gate, the choice rule, the decision record, and the re-run of GATE-NEMESIS-05 with the chosen estimator.
DESIGN, stated in both beads: the upstream estimator replaces ours on the default path only if it is at least
as good on both measures, a variance per simulated step on the random walk no larger than ours and episodes to
first failure on the gate fixture no worse than ours; otherwise ours is kept. Reason: the register says only
"then choose one"; the estimator already on the default path has passed the kill gate on this project's forks,
so replacing it must be justified both where the truth is known in closed form and on the measure the
exploration lane is judged by. Whichever is not chosen stays as the differential check. What this bead owes
that procedure: the estimator must return, per run, the estimate, the iteration count J, the level history, a
replica-based variance and the number of simulated steps it consumed, so that variance per simulated step can
be computed for it as for ours. This bead hands over an estimator on the sibling's `main`, a fixture that will
flip from failing to passing at a pin that contains it (the train notices the flip and sets the row to
`landed`), and the script whose phase `post` that procedure runs.

**Fallback if the change cannot land.** Our estimator alone, checked on the closed-form fixture; if it fails
its own falsifier, crude sampling alone.

## Acceptance criteria

- [ ] In FrankenSim: the estimator exists with the random-walk fixture as its own test and labels its output as an estimate.
- [ ] In FrankenSim: over 10⁴ repetitions on the fixture the mean of p̂ is within 3 standard errors of the closed form, for two different score functions (unbiasedness whatever the score).
- [ ] In FrankenSim: each run returns the estimate, J, the level history, a replica-based variance and the count of simulated steps; the output type cannot be passed where a bound is expected.
- [ ] Here: the fixture `upstream_fd_fsim_19` fails at the pin with the signature in `expected.toml` (no splitting entry in the `stats` group), and phase `pre` of the script passes.
- [ ] The item's row in `registries/upstream.toml` names its fixture, the admission test of N26b-22, this bead and the mirrored bead in the sibling's tracker; its `status` is `landed`, with the sibling commit that contains the estimator. The status `adopted` is set by the switch-over (N26bp1-01, Procedure B), not by this bead. If KILL-RARE has fired first, the row is closed by the recorded decision "no consumer" and this bead closes with it.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling. (The pin move, the new receipt and the record in `DEPENDENCY_ADMISSION.md` are checked in N26bp1-01.)

## Test plan

### Unit tests
In FrankenSim (added, in `fs-uq`):
- `gamblers_ruin_closed_form`: a walk on the integers that steps up with probability q = 0.4 and down otherwise, from a = 1, absorbed at 0 and at b = 20. With ρ = (1 − q)/q = 1.5 the probability of reaching b first is (1 − ρ^a)/(1 − ρ^b) = 0.5/3324.26 = 1.504·10⁻⁴. Expected: the mean of 10⁴ estimates within 3 standard errors.
- `unbiased_for_a_poor_score`: the same with a score that is a noisy function of position; expected the same mean and a larger variance.
- `branch_uses_fresh_randomness`: two children of one parent diverge after the branch point with probability 1 on the fixture.
- `estimate_formula`: with N = 100 and J = 880 iterations, p̂ = 0.99⁸⁸⁰ = 1.44·10⁻⁴.
- `duplicate_children_are_reported`: branches made to share the parent's stream are reported as duplicates by the estimator itself (the control the airlock's admission test of N26bp1-01 relies on).
Here: the fixture `upstream_fd_fsim_19` (ignored; fails at the pin). The tests on this side that follow the
change (`duplicate_children_control` through the airlock, and the differential test `random_walk_differential`
with its agreement statistic below 3) are in N26bp1-01, where the switch-over is specified.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_19.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_19 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, and the registry row must
read `landed`, with the sibling commit that contains the estimator and a receipt newer than the pin move
(N26bp1-01 re-runs GATE-NEMESIS-05 before calling this phase and sets the row to `adopted` only after it passes,
step 7 of its Procedure B). Phase `post` also runs `frob nemesis search … --method splitting --first --key <k> --json` on the planted-trap
fixture once per estimator and logs episodes to first discovery for each. The two calls are compared, so each
carries its own idempotency key; which estimator a call uses is selected by the configuration value
`nemesis.splitting.estimator = ours | upstream`, which the switch-over bead N26bp1-01 defines when it admits the
upstream entry (the verb has no option for it, and this bead adds none): the phase sets it per call in a scratch
configuration, on one binary and never with a separate build, and asserts the `estimator` named in each result's
method block (`frob-nemesis` under `ours`, `fs-uq` with the sibling commit under `upstream`), so that a call which
silently ran the other estimator fails the phase. This bead writes both phases and runs phase `pre`;
phase `post` is run by the switch-over (`scripts/e2e/nemesis_switchover_splitting.sh` of N26bp1-01 calls it
after the pin move), which is where its assertions are accepted.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_19/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-19`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the gambler's-ruin probability 1.504·10⁻⁴; the symmetric case a/b; the planning figure of 7% for L = 6, p₀ = 0.1, N = 10⁴.
- Negative controls: an estimator whose branches reuse the parent's stream must underestimate the variance and be caught by the duplicate check; a level rule that conditions on the particle's future must show a biased mean on the fixture.
- Pilot falsifier carried from the mechanism (TARGET): the planted trap of probability at most 3·10⁻⁵ is not found at least three times sooner than by uniform sampling, or fewer than 1% of forks reach the next level; then the fallback is crude sampling alone.
- Determinism lanes: for fixed seeds the estimate is identical twice on one machine and across worker counts (draws are keyed by iteration and particle, not by execution order).

## Considerations

- Published evidence cuts both ways: adaptive splitting with temporal-logic scores has been reported to estimate 3.5·10⁻⁴ for a true 4.8·10⁻⁵, with a spread five times the mean (FACT from the plan's source register, to be re-read). That is why splitting stays exploration-only here whichever implementation is chosen.
- OPEN-NEMESIS-2: when the requirement margin is flat until shortly before failure there is no useful score, and the default is no splitting for that requirement.
- Ceiling, stated: discovery speed only; the certification price is unchanged.
- This bead exists on one branch of a kill-gate decision, so it depends on the decision bead of KILL-RARE
  (N26a-32, M3): the sibling work starts only after that decision is on record as `not-fired`. If the gate
  fired, splitting has no consumer here, no sibling work is done, and the row is closed by that recorded
  decision (the acceptance line above).
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

Appendix F.1 (row FD-FSIM-19), F.3; §26.9 (rare failures), §10 C-27 (rare failures: honest limits, test, pilot), §26.16 (OPEN-NEMESIS-2, -4), §33.9; GATE-NEMESIS-05.
