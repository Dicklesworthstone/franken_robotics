---
key: "FD-FSIM-16"
title: "upstream frankensim: `constellation.lock` refreshed to the heads its `Cargo.lock` builds with"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "workspace", "upstream", "frankensim", "solid"]
parent: "WP-02"
deps: ["BD-037", "BD-006", "FD-ASUP-01", "NF-01", "STD-E2E"]
needs: []
related: ["BD-004", "BD-008", "NF-03", "FD-FSIM-14"]
gates: ["INT-FSIM-01", "INT-ASUP-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.2", "§33.5", "§33.8", "§41"]
---
## Background

FrankenSim records the commits of the siblings it consumes in its own lock, `constellation.lock`, and has a
verify-only preflight that treats an off-pin or dirty sibling as a refusal (FACT: `frankensim/AGENTS.md`,
"Cargo-Independent Constellation Preflight"). On the day the plan was written that lock was stale (hazard 3 of
the decision to consume FrankenSim by path; FACT read 2026-11-21): it pins asupersync 0.5.0 at `7a0f2f55047c`,
while `frankensim/Cargo.lock` resolves asupersync 0.6.2 by path and the local checkout is 0.6.2 at
`b32c5ba66315` with 267 modified paths. Its pins for franken_numpy, frankenscipy, frankensqlite, frankentorch
and franken_networkx also differ from the local heads. By its own rule FrankenSim's preflight would refuse
today; the preflight was not run for the plan. Upstream register priority: **P2**.

Why it matters here. This project's resolution receipt hashes a sibling's own lock and compares it with what
is built, classifying the disagreement the way FrankenSim does: `on-pin`, `stale-lock` (the checkout is a
fast-forward of the lock) or `drift` (dirty, retreated or diverged). While FrankenSim's own lock disagrees with
its own build, every receipt of this project records `agrees: false` for FrankenSim, and nobody can tell from
FrankenSim's lock which sibling commits its tests were run against. Rejected: ignoring the sibling's own lock
in the receipt (it is the only statement FrankenSim makes about its constellation); "fixing" the disagreement by
moving sibling checkouts back to the locked commits (a sibling's newer clean head is never rolled back locally,
and this project's tooling never moves a sibling checkout).

## Scope

In: in FrankenSim, the constellation lock brought into agreement with what its `Cargo.lock` builds, through its
own lock procedure, with its preflight passing. Here: `check-stack` reporting on-pin. Out: choosing and
aligning the one asupersync version (FD-ASUP-01, a prerequisite: the asupersync entry is fixed there); the
receipt and `check-stack` (BD-004); FrankenSim's toolchain pin (NF-03).

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

**The change (DESIGN; FrankenSim's own lock procedure governs).**

1. *Classify each entry.* For every sibling named in `constellation.lock`: the locked commit, the commit of the
   local checkout, whether the checkout is clean, and the relation between the two (equal; the lock is an
   ancestor; diverged). This classification is recorded by INT-FSIM-01 on this side and is repeated in
   FrankenSim at the current head.
2. *Fast-forwards.* Where the lock is an ancestor of a clean head that `Cargo.lock` builds with, the entry
   moves to that head.
3. *Dirty or diverged siblings.* A lock cannot name a dirty tree. The entry moves to the newest clean commit of
   that sibling against which FrankenSim's build and tests pass; if the build needs uncommitted work, that work
   is first committed in the sibling by whoever owns it (this item does not commit another agent's work), and
   the entry waits. Each waiting entry is listed in the mirrored bead with its reason.
4. *asupersync.* Its entry is the one published version V fixed by FD-ASUP-01, at V's commit.
5. *Verify.* FrankenSim's verify-only preflight passes at the landed commit; its test suite passes against the
   locked constellation.

Additive in effect: no source changes; the lock states what was already being built.

**Interim posture here.** The resolution receipt records the disagreement: for the FrankenSim entry,
`own_lock = {file: "constellation.lock", hash, agrees: false, class: "stale-lock" or "drift"}`. No gate records
a PASS on a dirty sibling tree, whatever the lock says.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-16/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-16`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_16` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It reads FrankenSim's `constellation.lock` and `Cargo.lock` at the pinned
commit and asserts that every constellation entry equals the revision `Cargo.lock` resolves; at the pin the
asupersync entry (0.5.0 against 0.6.2) fails first.

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

**What the switch-over re-runs.** The rule is "`check-stack` reports on-pin": after the pin move,
`cargo xtask check-stack` gives the FrankenSim entry `own_lock.agrees = true` and class `on-pin`. With it:
INT-FSIM-01 (which records the drift class of every sibling) and INT-ASUP-01 (`cargo tree -d` shows one
asupersync package in each workspace, the same published version in all of them).

**Fallback if the change cannot land.** The receipt keeps recording the disagreement and its class; nothing
else depends on FrankenSim's lock being fresh.

## Acceptance criteria

- [ ] In FrankenSim: every entry of `constellation.lock` equals the revision its `Cargo.lock` resolves, or is listed in the mirrored bead as waiting on a sibling's uncommitted work, with the sibling and the reason.
- [ ] In FrankenSim: its verify-only constellation preflight passes at the landed commit on a machine whose sibling checkouts are at the locked commits and clean.
- [ ] Here: `check-stack` reports `own_lock.agrees = true` and class `on-pin` for FrankenSim under the new receipt; the receipt's `duplicates` is empty.
- [ ] Here: INT-FSIM-01's record lists the drift class of every sibling at the new pin, with none unclassified.
- [ ] No sibling checkout was moved, reset or cleaned by any tool of this repository in the process.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `constellation_matches_cargo_lock`: parses both files; expected equality of revision for every path-resolved sibling. Input today: asupersync 0.5.0 at `7a0f2f55047c` against 0.6.2; expected after: equal.
Here (`tests/upstream/FD-FSIM-16/`):
- `own_lock_agreement_is_computed`: on a `synthetic` sibling pair with a lock one commit behind, the receipt says `agrees: false`, class `stale-lock`; with the lock refreshed, `agrees: true`, class `on-pin`; with a dirty tree, class `drift`.
- `receipt_hashes_the_own_lock`: changing one byte of the stand-in's lock changes the receipt's `own_lock.hash`.
- `frankensim_own_lock_agrees`: the fixture against the real pinned commit.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_16.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_16 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Both phases print, for every constellation entry, the locked revision, the built
revision and the class, and the BLAKE3 hash of both lock files.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_16/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-16`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-767: a receipt computed twice on an unchanged tree is byte-identical; touching one tracked file in a sibling flips `dirty`, changes `tree_hash`, and `check-stack` fails.
- Negative controls: a lock refreshed to a commit that `Cargo.lock` does not build must fail `constellation_matches_cargo_lock`; a receipt that reports `on-pin` for a dirty sibling must be rejected by the property test of INV-SUITE-03.
- Determinism lanes: the classification is exact and identical on every platform.

## Considerations

- RISK-23 (dirty or off-pin sibling trees make a receipt unrepeatable): the mitigations are the receipt's byte hash, the rule that no gate records a PASS on a dirty tree (OPEN-SUITE-3: a dirty asupersync checkout is acceptable for development builds only, the default until the owner decides), and the compatibility train.
- asupersync's checkout had 267 modified paths: step 3 will very likely leave its entry waiting until that work is committed there. That is an honest outcome, and the bead closes as `landed` for the other entries only if the mirrored bead lists the waiting one.
- A constellation lock goes stale again as soon as a sibling moves; FrankenSim's own preflight is the standing guard, and this item restores a state from which the guard can be run.
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

Appendix F.1 (row FD-FSIM-16), F.3; §33.2 (hazard 3), §33.5 (the receipt's `own_lock`), §33.8; INT-FSIM-01, INT-ASUP-01; KA-767; INV-SUITE-03, INV-SUITE-04; RISK-23.
