---
key: "FD-FSIM-12"
title: "upstream frankensim: a moving-surface owner for the sustained contact step (witness migration)"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "suite", "kinesis", "traction", "upstream", "frankensim", "solid"]
parent: "WP-39"
deps: ["BD-093", "FDEF-04", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-13", "N33b-07", "N33b-08", "NFp3-11"]
gates: ["INT-FSIM-07"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 4", "Appendix F.3", "§16.4", "§18.4", "§33.11", "§33.8"]
---
## Background

FrankenSim's sustained contact step, the referee's contact law across time, keeps **fixed material witnesses
and a fixed contact frame** for each body pair (FACT,
`frankensim/crates/fs-mbd/src/articulated_contacts/step/sustained.rs`: "Body pairs retain their ORIGINAL
material witnesses and fixed contact frame"; tangentially separating witnesses are refused; a documented scope
limit with typed refusals, module header lines 1–10 and lines 333–339). A wheel rolling over a sharp lip, a
sliding sole and a foot rocking from heel to toe all move their witnesses. The limit report is FDEF-04.
Upstream register priority: **P1**.

This project works above the limit: the referee's transactional step migrates witnesses between calls, so that
each call into FrankenSim sees an interval short enough that its witnesses are fixed inside FrankenSim's own
gate. That is the design of record at M1. It has a cost: the migration logic that decides what the referee
computed lives outside the referee, and where FrankenSim's gate still refuses, the referee does not answer and
the `strict` profile adjudicates. RISK-31 (the referee cannot complete the scenes where decisions are made) and
BET-34 (the referee completes at least 90% of standard-battery scenarios at M2 exit) both rest on this.

Rejected: truncating or freezing witnesses to get an answer (a wrong answer from the adjudicator); a local fork
of the sustained step.

## Scope

In: in FrankenSim, an owner for contacts whose witnesses move; here, delegation to it, the pin move and the
referee-side re-runs of the switch-over. Out: the one-time re-baseline of the fidelity ledger for this item, the
reading of GATE-FID-01 against it and the re-issue of the referee's coverage table (NFp3-11, which depends on this
bead, on the M2 ledger measurement BD-212, on the envelope store N16bp1-01 and on the coverage campaign BD-213,
so that this upstream work can start before the ledger exists; the split made for FD-FSIM-02 in NFp2-01); the
migration in this project's transactional step (BD-093), which
remains for the `strict` profile on our own kernels; capacity above 21 contacts (FD-FSIM-13); the persistent
stepper (FD-FSIM-02).

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

**The change (DESIGN).** A moving-surface mode of the sustained step, beside the fixed-witness mode, in which
for every active pair and every step:

1. *Relocate.* The material witnesses are moved to the current closest points of the pair, on both bodies,
   consistently with one world contact point.
2. *Re-frame.* The contact frame (normal n, tangents t₁, t₂) is rebuilt at the new witnesses.
3. *Carry the tangential memory.* The stored tangential state ξ (the elastic tangential displacement or the
   last tangential impulse, as the friction law defines it) lies in the old tangent plane. It is transported
   into the new one by the minimal rotation that takes n_old to n_new and then projected onto the new plane:
   ξ′ = P(n_new)·R(n_old → n_new)·ξ. Transport by a rotation preserves ‖ξ‖, so it creates no energy.
4. *Or reset it,* when the declared friction law says so: across a feature edge where the normal jumps by more
   than a declared angle, or for a law declared memoryless. A reset discards the stored tangential energy
   ½·k_t·‖ξ‖²; the step reports it as dissipation, never as a gain.
5. *Refuse by name* what it still cannot own (for example a witness leaving the body's surface
   parameterization), as today.

Additive: the fixed-witness mode and its refusals are unchanged and remain the default.

**Interim posture here.** Witness migration in the referee's transactional step, above the airlock: new witness
pairs are localized at a consistent world contact point, and tangential memory is transported or reset. If
FrankenSim's gate still refuses, the refusal is surfaced with the upstream reason and the `strict` profile
adjudicates. Referee rollouts over planes and primitive obstacles are admitted at M1; a wheel climbing a riser
is admitted only through the event-located step.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-12/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-12`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_12` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It is the flat-to-edge-to-top transition of a wheel (the fixture
`wheel-over-lip` of BD-035 and BD-086: R = 0.10 m, sill 0.03 m with a sharp lip) sent to FrankenSim's
sustained step directly, with no migration above the airlock: it returns the typed refusal at the pin.

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

**What the switch-over re-runs.** In this bead: INT-FSIM-07 in its adopted form (the transition completes inside
FrankenSim with the energy ledger closed to its stated residual, and the old refusal is recorded as fixed); the
referee's contact battery; the thin-barrier and sill fixtures; and the comparison of the transition fixture
before and after delegation (below). Their record, with the receipt hash, is what NFp3-11 starts from, and this
bead leaves the row `landed`. The referee delegates migration to the upstream owner when the airlock reports the
capability, so its outputs change, and the plan's switch-over rule also says that the **fidelity ledger is
re-baselined once** for this item, with a written rationale. That step is NFp3-11's. The delegating referee
carries a new referee kernel version (bumped by this bead in the change that enables delegation, since the
stratum key contains the kernel versions), so every (fast, `referee`) stratum is a new stratum; it is measured on the frozen
battery and frozen under the cause `upstream:FD-FSIM-12` of the ledger's re-baseline record (N16bp1-01), and no
frozen envelope is edited: the old strata stay as they were. GATE-FID-01 (`batch-cpu` against `referee`, median
and 95th-percentile disagreement at most 2% and 5% for base displacement over 2 s, 3% and 8% for energy per
metre, 5% and 12% for stance impulse) is then read against the new strata, the referee's coverage report per
robot class (GATE-FID-03) is re-issued, with the refusal cause `fixed-witness-gate` expected to fall, and only
then does this item's row become `adopted`. If this change is adopted before BD-212 has frozen the M2 envelopes,
no re-baseline is needed and NFp3-11 records that.

*Before and after delegation (DESIGN: no switch is added for it).* The same binary cannot run the referee both
ways at one pin: at the old pin the transactional step migrates witnesses above the airlock, and at the new pin
it delegates. So phase `pre` of the script runs the `wheel-over-lip` fixture on the referee at the old pin and
stores its base displacement, its energy-ledger totals and its contact impulse; phase `post` runs it again at the
new pin under a different key and records the three differences beside both stack receipts. NFp3-11 cites those
differences in its re-baseline document, beside the old envelope bounds (one fixture is context for the
rationale, not a stratum).

**Fallback if the change cannot land.** Migration above the airlock stays; the `strict` profile stands as the
highest tier for scenes the referee cannot complete, and results there say so.

## Acceptance criteria

- [ ] In FrankenSim: the moving-surface mode completes the wheel transition and a heel-to-toe rock without refusal, and the fixed-witness mode's behaviour and golden values are unchanged.
- [ ] In FrankenSim: in steady rolling without slip the tangential memory stays zero to 10⁻¹² and mechanical energy is conserved to the step's stated residual; a reset reports non-negative dissipation in 10⁴ of 10⁴ seeded cases.
- [ ] Here: INT-FSIM-07 passes in its adopted form at the new pin, with the energy-ledger residual recorded against its bound.
- [ ] Here: the `wheel-over-lip` fixture's base displacement, energy-ledger totals and contact impulse at the new pin (migration delegated) are compared with the values phase `pre` stored at the old pin (migration above the airlock), and the three differences are recorded with both stack receipts.
- [ ] Here: the referee's kernel version in the stratum key differs between the two pins, and the referee's contact battery and the thin-barrier and sill fixtures pass at the new pin.
- [ ] Here: the switch-over's referee-side record (INT-FSIM-07 in its adopted form, the battery, the two fixtures and the differences, with the receipt hash) is on file for NFp3-11, which makes the one re-baseline of the fidelity ledger for this item, reads GATE-FID-01 against it and re-issues GATE-FID-03's coverage table; nothing in this bead waits on the ledger.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead, the switch-over bead NFp3-11 and the mirrored bead in the sibling's tracker; its `status` is `landed` once the change is on the sibling's `main` and the pin has moved (NFp3-11 moves it to `adopted`).
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `rigid_disk_over_step_inelastic`: a solid disk of radius 0.10 m rolling without slip at 1.0 m/s onto a step of height 0.03 m, under an inelastic law without slip at the edge. Angular momentum about the edge is conserved at impact: ω′/ω = 1 − 2·s/(3·r) = 0.8, so the speed just after impact is 0.8 m/s; on top, v² = 0.64 − (4/3)·g·s = 0.2476, v = 0.4976 m/s. Expected within 1% (TARGET), with the witness on the ground moving from the floor to the edge to the top.
- `steady_rolling_has_no_tangential_memory`: 10⁴ steps on a flat; ‖ξ‖ at most 10⁻¹².
- `transport_preserves_norm`: 10⁴ random (n_old, n_new) with angle below 60° and ξ drawn in the old tangent plane (ξ·n_old = 0); expected ‖ξ′‖ = ‖ξ‖ and ξ′·n_new = 0, both to 10⁻¹² (the minimal rotation maps the old tangent plane onto the new one, so the projection removes nothing). For ξ with a planted normal component, ‖ξ′‖ ≤ ‖ξ‖.
- `reset_dissipation_is_non_negative`: as in the acceptance line.
- `heel_to_toe_rock`: a box foot rocking over its front edge; completes, with the normal force's point of application moving monotonically forward.
Here:
- `referee_delegates_when_capable`: the airlock reports the capability at the new pin and the transactional step calls it; at the old pin it migrates above the airlock.
- `delegation_changes_referee_kernel_version`: the stratum key built for a referee run with delegation carries a referee kernel version different from the key built without it, so the two runs can never land in one stratum.
- `transition_difference_is_recorded`: given the stored `pre` values and a `post` run of the fixture, the record holds the three differences with their units and both receipt hashes; a `post` run under the `pre` run's key is detected as a stored receipt (`idempotent_replay: true`) and refused as a comparison.
(That a second re-baseline citing this item is refused is NFp3-11's test, on N16bp1-01's record.)

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_12.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_12 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `pre` also runs `frob sim run --fixture wheel-over-lip --tier referee --seed 5
--key fd12-lip-pre --json` and stores the run's base displacement, energy-ledger totals and contact impulse.
Phase `post`, after the pin move: the same fixture must pass, INT-FSIM-07 in its adopted form, the referee's
contact battery and the thin-barrier and sill fixtures are re-run, and the registry row must read `landed` with a
receipt newer than the pin move (`adopted` once NFp3-11 has run: GATE-FID-01 and GATE-FID-03 are read there, not
here). Phase `post` also runs `frob sim run --fixture wheel-over-lip --tier referee --seed 5 --key fd12-lip-post
--json` (a run verb: the two compared runs carry distinct keys, and both envelopes show `idempotent_replay:
false`) and `frob telemetry energy <that run> --json` (asserting the ledger residual against its bound), and logs
the three differences against the stored `pre` values. The ledger reads before and after the re-baseline and the
coverage reads (`frob fidelity ledger`, `frob fidelity coverage`) are in NFp3-11's script.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_12/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-12`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-778: the transition returns the typed refusal at the pin and completes under witness migration with the energy ledger closed to its stated residual; the rigid-disk values above (0.8 and 0.4976 m/s).
- Negative controls: a migration that relocates the witness on one body only must open a spurious couple (above 10⁻⁹ N·m on the sill fixture) and be caught; a transport that copies ξ without rotating it must create energy on a curved path and be caught by the created-energy audit; a wheel on a flat floor only must not exercise migration at all.
- Determinism lanes: twice on one machine; across worker counts; debug and release; same-platform class.

## Considerations

- The re-baseline is the single allowed exception to frozen envelopes for this item; FD-FSIM-02 has its own. It is made by NFp3-11 under the cause `upstream:FD-FSIM-12`, and no frozen envelope is edited: the delegating referee's strata are new strata. If both items land in one pin move, each still has its own record, in the order they landed.
- Two implementations of migration will exist after adoption (upstream for the referee, ours for `strict`); their agreement on the transition fixtures is a model-form check worth keeping, not a redundancy to remove.
- BET-34's fallback names this item and FD-FSIM-13: until they land, the `strict` profile is the highest tier where the referee cannot complete, and results there stay provisional and say so.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists (NFp3-11), not that the
  milestone's exit depends on it; the work itself may start as soon as BD-093 is done. Until the switch-over has been run, the interim posture above is the design of
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

Appendix F.1 (row FD-FSIM-12), F.2 (row 4), F.3; §16.4 (limits table), §18.4, §33.11 (row 4), §33.8; INT-FSIM-07; GATE-FID-01, GATE-FID-03; KA-778; RISK-31; BET-34.
