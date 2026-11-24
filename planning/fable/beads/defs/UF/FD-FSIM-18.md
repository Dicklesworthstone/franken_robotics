---
key: "FD-FSIM-18"
title: "upstream frankensim: the 10⁶ sample caps of `fs-eproc` and of `fs-uq` importance execution parameterized"
type: "task"
priority: 3
milestone: "M4"
labels: ["m4", "suite", "nemesis", "upstream", "frankensim", "solid"]
parent: "WP-55"
deps: ["BD-037", "BD-276", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-19", "N26a-22", "N26a-31", "N26b-22", "N33a-18", "N33b-12", "N33b-13"]
gates: ["GATE-NEMESIS-05"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§26.9", "§26.16", "§42", "§33.9"]
---
## Background

Nemesis, the adversary subsystem, certifies that a failure is rare: that its probability p is below a target
p* with confidence 1 − δ. The certification lane uses anytime-valid statistics from FrankenSim: a confidence
sequence for a Bernoulli rate (FACT: `frankensim/crates/fs-eproc/src/bernoulli.rs`, a Beta(½, ½) mixture **with
a cap of 10⁶ samples**) and bounded-weight importance sampling (FACT:
`frankensim/crates/fs-uq/src/product_importance.rs`, **with a cap of 10⁶ evaluations**). Upstream register
priority: **P3**.

The caps bind exactly where the claim is interesting. With zero observed failures, certifying p ≤ p* at
confidence 1 − δ needs about n = ln(1/δ)/p* nominal episodes: for δ = 0.05, about 3·10⁴ at p* = 10⁻⁴,
3.0·10⁵ at 10⁻⁵, and 3.0·10⁶ at 10⁻⁶, three times the cap. So a 10⁻⁶ claim cannot pass through the sibling's
sequence at all. Rejected: restarting the sequence at the cap and combining the pieces by hand (the anytime
guarantee does not survive an unplanned restart); extrapolating a tail fit (checking a fitted tail at 10⁻⁶
costs the nominal episodes that certify directly).

## Scope

In: in FrankenSim, both caps turned into parameters with the present value as default, the numerics made safe
at larger counts, and the tests added there; the mirrored bead in the sibling's tracker; here, the item's
failing fixture `tests/upstream/FD-FSIM-18/` kept current, the script `scripts/e2e/upstream_fd_fsim_18.sh` with
its two phases, and the registry row moved from `open` to `landed`. This bead closes when the change is on the
sibling's `main`.

Out, with the bead that has it:
- the switch-over on this side once the change has landed: N26b-24 (its Procedure A), which depends on this
  bead. It rewrites the admission tests to the new behaviour, passes the configured cap through the airlock,
  runs the differential test of the sibling's sequence against the exact binomial bound, updates the routing
  above 10⁶, re-runs GATE-NEMESIS-05 at the larger count and the coverage row of GATE-NEMESIS-04, runs this
  bead's script in phase `post`, and moves the row to `adopted`. None of that is specified here.
- the admission tests that record the two caps at the pin, the tests of the interim posture and the decision
  record of OPEN-NEMESIS-4: N26b-22, a related bead of M3 (the admission test comes first and the upstream item
  second, which the milestones already give; the link is not a blocking one, because this item's own fixture
  is opened by BD-037 and nothing in the sibling change needs the Nemesis tests).
- the row itself and the form of every fixture: BD-037.
- the bounded-weight estimator and its defensive mixture on this side (BD-276); splitting (FD-FSIM-19, BD-277);
  the compute envelope for a 10⁻⁶ claim (OPEN-NEMESIS-3, an owner decision, N26b-21).

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

**The change (DESIGN).** In each of the two modules the constant becomes a field of the configuration, default
10⁶. Above the default the arithmetic must stay exact enough to keep the guarantee: the mixture wealth of the
Bernoulli sequence after n trials with k failures against a rate p is

  log W = log B(k + ½, n − k + ½) − log B(½, ½) − k·log p − (n − k)·log(1 − p),

with B the beta function, evaluated in log space by log-gamma so that it neither overflows nor loses the small
term at n in the millions; counts are 64-bit. The contract states the tested maximum (TARGET: 10⁸).

Additive: at the default every result is bit-identical to today's.

**Interim posture here.** This project's own exact binomial bound for larger counts (with zero failures in n
trials the one-sided upper bound at confidence 1 − δ is 1 − δ^(1/n)); claims are certified to 10⁻⁵ and reported
as `Estimated` beyond (the default of OPEN-NEMESIS-3; OPEN-NEMESIS-4 records the caps and the absence of
splitting upstream, default as stated, and blocks nothing). The posture is built and tested by N26b-22 (routing
by planned count, agreement of the two routines at the boundary, refusal of an importance run above 10⁶
evaluations) on the exact bound of N26a-22; this bead changes none of it.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-18/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-18`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_18` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It feeds 3·10⁶ failure-free trials to the sibling's Bernoulli sequence
through the airlock and expects an upper bound, and it requests an importance run of 10⁶ + 1 evaluations and
expects it to be accepted; at the pin it receives the cap refusal after 10⁶ in both. The fixture is not the
admission test: `bernoulli_sequence_cap` and `importance_execution_cap` (N26b-22, in `frob-fsim`) pass at the pin
by recording the cap error, and are named in the row's `admission_test` field; this fixture fails at the pin by
asking for what the cap forbids, and is the row's `fixture`.

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

**Who runs the switch-over.** The register's rule for this item is "re-run GATE-NEMESIS-05 at the larger
count". The steps of the train that are particular to this item (step 3 above, and what must change in
`frob-fsim` and `frob-nemesis` before the relock) are one procedure, written once, in N26b-24 (Procedure A):
the rewritten admission tests, the cap passed through the airlock, the differential test against the exact
binomial bound at six counts, the routing above 10⁶, the two gate re-runs with their numbers, and the record.
This bead hands over a change on the sibling's `main`, a fixture that will flip from failing to passing at a
pin that contains it (the train notices the flip and sets the row to `landed`), and the script whose phase
`post` that procedure runs.

**Fallback if the change cannot land.** Our exact binomial bound for counts above 10⁶; 10⁻⁶ stays `Estimated`
unless the owner approves the envelope and the bound is computed here.

## Acceptance criteria

- [ ] In FrankenSim: both caps are configuration fields with default 10⁶; at the default all existing tests and values are unchanged; at 10⁸ the sequence runs without overflow or loss of validity.
- [ ] In FrankenSim: over 10⁴ null runs at the true rate with optional stopping, the sequence excludes the truth in at most a fraction δ of runs, at n up to 3·10⁶.
- [ ] In FrankenSim: above the configured cap both routines still refuse by name; neither truncates a run.
- [ ] Here: the fixture `upstream_fd_fsim_18` fails at the pin with the signature in `expected.toml` (both requests refused at the cap), and phase `pre` of the script passes.
- [ ] The item's row in `registries/upstream.toml` names its fixture, the admission tests of N26b-22, this bead and the mirrored bead in the sibling's tracker; its `status` is `landed`, with the sibling commit that contains the change. The status `adopted` is set by the switch-over (N26b-24, Procedure A), not by this bead.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling. (The pin move, the new receipt and the record in `DEPENDENCY_ADMISSION.md` are checked in N26b-24.)

## Test plan

### Unit tests
In FrankenSim (added):
- `default_cap_is_unchanged`: 10⁶ accepted, 10⁶ + 1 refused, as today.
- `log_wealth_matches_direct_at_small_n`: n = 20, k = 3, p = 0.1; the log-space value equals the direct product to 10⁻¹².
- `no_overflow_at_1e8`: n = 10⁸, k = 0, p = 10⁻⁶; finite wealth and a finite bound.
- `anytime_validity_under_optional_stopping`: as in the acceptance line, δ = 0.05.
- `configured_cap_refuses_by_name`: configured cap 2·10⁶, 2·10⁶ + 1 trials; expected the named refusal, for the
  sequence and for the importance execution.
Here: the fixture `upstream_fd_fsim_18` (ignored; fails at the pin). The tests on this side that follow the
change (`zero_failure_bound_closed_form`, `sibling_bound_not_tighter_than_exact`, `over_cap_refuses_by_name`
through the airlock) are in N26b-24, where the switch-over is specified.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_18.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_18 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob nemesis bound <candidate> --task <the failure-free fixture task> --tier batch-cpu
--target 1e-6 --mode certify --budget episodes=3000000 --key fd18-post --json` (a run verb: the tier is
required and the call carries its idempotency key), asserting the bound and its label. This bead writes both phases and
runs phase `pre`; phase `post` is run by the switch-over (`scripts/e2e/nemesis_switchover.sh` of N26b-24 calls
it after the pin move), which is where its assertions are accepted.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_18/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-18`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers (KA-664): the zero-failure count by the exact rule n = ⌈ln δ / ln(1 − p*)⌉ at δ = 0.05 is 29,956 at 10⁻⁴, 299,572 at 10⁻⁵ and 2,995,731 at 10⁻⁶ (the approximation ⌈ln 20/p*⌉ gives 29,958, 299,574 and 2,995,733 and is not used); the zero-failure bound 1 − δ^(1/n) is 9.999991·10⁻⁶ at n = 299,572 and 9.986·10⁻⁷ at n = 3·10⁶.
- Negative controls: a sequence restarted at the cap and multiplied through must violate the type-I rate on the null runs (this is the shortcut the item exists to prevent); a 32-bit counter must fail `no_overflow_at_1e8`.
- Determinism lanes: exact counts; the bound is identical twice on one machine and across platforms to 10⁻¹² relative.

## Considerations

- OPEN-NEMESIS-3 (owner): a 10⁻⁶ claim exceeds the one-hour job rule at the illustrated throughput; lifting the cap makes the claim expressible, not affordable.
- A bound is scoped to its tier: a rate certified on a fast tier is a statement about that tier, and found failures are replayed one tier up.
- Importance sampling with a defensive mixture q = β·p + (1 − β)·q′ has weights at most 1/β, and zero observed failures certify only p ≤ ln(1/δ)/(β·n); the cap on its evaluations binds at the same counts.
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

Appendix F.1 (row FD-FSIM-18), F.3; §26.9, §26.16 (OPEN-NEMESIS-3, -4), §33.9; GATE-NEMESIS-05.
