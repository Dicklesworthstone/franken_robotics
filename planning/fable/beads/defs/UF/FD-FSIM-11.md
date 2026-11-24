---
key: "FD-FSIM-11"
title: "upstream frankensim: `planar_drive` without the near-straight defect (the half-angle chord form)"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "traction", "upstream", "frankensim", "solid", "review-notes-open"]
parent: "WP-37"
deps: ["FDEF-02", "N33b-04", "STD-E2E"]
needs: []
related: ["N33b-08", "NF-07"]
gates: ["INT-FSIM-05"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 2", "Appendix F.3", "§33.8", "§33.11", "§18.9"]
---
## Background

`fs-mbd::planar_drive` (`frankensim/crates/fs-mbd/src/planar_drive.rs:117–133` at `483d09c682f2`) integrates a
prescribed-speed differential drive. It takes its straight branch only when the yaw rate satisfies
|ω| ≤ 10⁻¹² rad/s; otherwise it forms θ′ = θ + ω·Δt and returns R·(sin θ′ − sin θ) and −R·(cos θ′ − cos θ), with
R = v/ω. When ω is just above the threshold and the heading θ is large, the increment ω·Δt is smaller than half
the spacing of doubles at θ, θ′ rounds to θ, both differences are exactly zero, and the chassis does not move
although both wheels turn. The defect report is FDEF-02; the admission test is INT-FSIM-05. Upstream register
priority: **P2**.

This project wants `planar_drive` as an independent known-answer source for wheeled bases, a second opinion
beside its own closed form. It cannot be one until it is right. Rejected: using it only in the regime where it
works (an oracle with an unmarked hole is how a wrong known answer gets written); a corrected local copy (the
value of the sibling integrator is that it is not ours).

## Scope

In: the corrected step in `fs-mbd`, its tests there, the re-run of INT-FSIM-05 and the admission of
`planar_drive` as an oracle. Out: the defect report (FDEF-02); the independent formula in `frob-conformance`
and the admission test (BD-033); the wheeled contact laws of the simulator.

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

**The change (DESIGN).** Replace the two-branch arithmetic by the half-angle chord form, which has no branch on
ω and no difference of nearly equal values:

  δ = ω·Δt,   Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)),   θ′ = θ + δ,

with v the forward speed, sinc(x) = sin(x)/x and sinc(0) = 1 (for |x| below about 10⁻⁴ the series
1 − x²/6 + x⁴/120 is used, which is exact to rounding there). The alternative named in the register, choosing
the straight branch when |ω·Δt| is below a small multiple of the unit in the last place of θ, also removes the
defect; the chord form is preferred because it removes the branch.

A second-order point the fix should settle. With θ = 32 rad and δ = 3·10⁻¹⁵ rad per step, θ′ = θ + δ still
rounds to θ, so over 400 steps the incremental heading does not advance while a one-shot step advances it by
1.2·10⁻¹² rad. The position difference that follows is about 6·10⁻¹³ m (the one-shot chord points along the mean
heading θ + 6·10⁻¹³ rad, so over one metre it sits 6·10⁻¹³ m to the side of the incremental path), inside the
admission test's 10⁻¹² m but with less than a factor of two to spare, and the final headings differ by the whole
1.2·10⁻¹² rad. A compensated heading (carry the rounding remainder of θ + δ into the next step) makes incremental and
one-shot integration agree to rounding. Whether to carry the remainder in the state is FrankenSim's decision;
if it does not, the test's incremental-against-one-shot row must use a case whose expected difference is
computed, and the report says so.

Additive form: a corrected step beside the present one, the present one documented; correcting in place is
FrankenSim's choice under its rules.

**Interim posture here.** Known answers for wheeled bases use the independent formula in `frob-conformance`;
`planar_drive` is held out in the oracle register (`crates/frob-conformance/registry/oracles.toml`, N38a-04: row
`fs-mbd-planar-drive`, `admission.status = "not-admitted"`, a `held_out` reason citing INT-FSIM-05 and this
item, written by BD-033) and nothing cites it.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-11/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-11`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_11` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It is the four rows of INT-FSIM-05 pointed at the sibling's step at the
pinned commit; the near-zero-curvature row fails there if the defect is real.

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

**What the switch-over re-runs.** INT-FSIM-05, against the chord formula to 10⁻¹²: exact straight travel; the
near-zero-curvature case (θ = 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, Δt = 1/400 s, 400
steps: one metre along the heading, about (0.83422336, 0.55142668) m); wrapped against unwrapped headings;
incremental against one-shot integration. When it passes at the new pin, `planar_drive` is admitted as an
oracle: that passing run of BD-033's test is the one path that clears the register row (`held_out` emptied,
`admission.status = "admitted"`, the role `reference:planar-drive`, DESIGN in the register's role grammar, and an
appended `[[pin_move]]` record naming the new pin), and the wheeled known answers gain a
differential check between the formula and the sibling.

**Fallback if the change cannot land.** The independent formula stays the only oracle for rolling.

## Acceptance criteria

- [ ] In FrankenSim: the corrected step returns one metre along heading 32 rad over the 400 steps, to 10⁻¹² m per component, and its existing planar-drive tests still pass.
- [ ] In FrankenSim: the four rows of the admission test exist as its own tests.
- [ ] Here: INT-FSIM-05 passes at the new pin with the four measured errors recorded against 10⁻¹².
- [ ] Here: the oracle registry lists `planar_drive` as `admitted` with the pin, and a differential test between it and the chord formula runs on 10⁴ seeded steps with maximum disagreement at most 10⁻¹² m.
- [ ] Here: at any pin where INT-FSIM-05 does not pass, a known answer that cites `planar_drive` is still refused.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-mbd`):
- `near_straight_at_large_heading`: the case above; expected displacement (0.83422336, 0.55142668) m to 10⁻¹² (the uncorrected code returns exactly (0, 0)).
- `chord_half_turn`: v·Δt = 1 m, δ = π, θ = 0; expected (0, 2/π) = (0, 0.6366197724) m to 10⁻¹².
- `exact_straight_travel`: ω = 0, θ = 0.3, v·Δt = 2 m; expected 2·(cos 0.3, sin 0.3) with no rounding beyond the two function calls.
- `wrapped_equals_unwrapped`: θ and θ + 2π·5 give displacements equal to 10⁻¹².
- `incremental_equals_one_shot`: 400 steps of 1/400 s against one step of 1 s at ω = 0.5 rad/s, v = 1 m/s, θ = 0; expected equal to 10⁻¹² (one-shot value: (sin 0.5, 1 − cos 0.5)/0.5 = (0.9588510772, 0.2448348762) m).
- `sinc_series_matches_quotient`: at x = 10⁻⁴ the series and sin(x)/x agree to one unit in the last place.
Here:
- `int_fsim_05_against_sibling`: the four rows; expected pass at the new pin.
- `oracle_registry_tracks_admission`: the entry is `admitted` if and only if a passing record exists for the current pin.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_11.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_11 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. The sibling's tests are run in a scratch clone at the pin; phase `post` also runs the
wheeled known answers of `frob-conformance` with the differential check enabled.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_11/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-11`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-775 (the near-zero-curvature case; tolerance 10⁻¹²; the reported branch returns exactly zero) and KA-776 (the chord (0, 2/π) m). CX-352 (a branch on abs(ω) ≤ 10⁻¹² rad/s separates straight from curved motion) and CX-182 (a library integrator can serve as the rolling oracle).
- Negative controls: the reported two-branch arithmetic, kept as a mutant in the test, must return zero translation on the case; a chord form that uses δ in place of δ/2 in the direction must fail the half-turn test (it would give (−2/π, 0)).
- Metamorphic relations: invariance under θ → θ + 2π·k; composition of n sub-steps.
- Determinism lanes: twice on one machine; debug and release; the tolerance covers platform `sin` and `cos`.

## Considerations

- RISK-24 names the planar drive among the reported defects that must not be relied on before recheck; admission as an oracle happens only through the switch-over.
- A tracked or skid-steer base is not a rolling constraint (exact lateral no-slip at two stations forces ω = 0), so this oracle applies to differential drives with one contact station per wheel and is never a known answer for tracks.
- The same crate file neighbours the robot catalog builders admitted with INT-FSIM-05; they are not touched.
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

Appendix F.1 (row FD-FSIM-11), F.2 (row 2), F.3; §33.8, §33.11 (row 2, the arithmetic), §18.9; INT-FSIM-05; KA-775, KA-776; CX-352, CX-182; RISK-24.
