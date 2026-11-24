---
key: "FD-FSIM-14"
title: "upstream frankensim: documented build-identity setting for path consumers outside the tree (`fs-la/build.rs`)"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "workspace", "upstream", "frankensim", "solid", "review-notes-open"]
parent: "WP-01"
deps: ["BD-006", "BD-037", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-16", "N33b-08", "NF-03"]
gates: ["INT-FSIM-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.2", "§33.5", "§33.8"]
---
## Background

FrankenRobotics consumes FrankenSim by path dependency from outside FrankenSim's directory tree. One of the
hazards of doing so (hazard 5 of the decision, FACT read 2026-11-21): **a build script that fails closed outside
the tree.** `frankensim/crates/fs-la/build.rs` panics unless `FRANKENSIM_DEPGRAPH_RECEIPT` or
`FRANKENSIM_DEPGRAPH_SALT` is set. Inside FrankenSim the salt comes from its `.cargo/config.toml`, which a
workspace outside the tree does not read. `fs-la`, the linear-algebra crate, is in the closure of `fs-ad`,
`fs-mbd` and `fs-contact`, so every referee build passes through it. Upstream register priority: **P2**.

Failing closed is the right behaviour: the variable names the equivalence class of the dependency graph a
binary was built in, and a performance number without one cannot be compared. What is missing is the
documentation for a consumer that is not in the tree: which variable to set, to what, and what it commits the
consumer to. This project found the answer by reading the build script, which is how the next consumer will get
it wrong.

Rejected: disabling or bypassing the check; copying FrankenSim's own salt (it would claim membership of
FrankenSim's equivalence class for a different graph).

## Scope

In: in FrankenSim, the build-identity setting documented for out-of-tree path consumers, with an actionable
failure message and, if FrankenSim chooses, a published default class for consumers; here, removal of the local
setting if a default class is published. Out: the workspace skeleton and its `.cargo/config.toml` (BD-001); the
admission record of INT-FSIM-01 (BD-006); the performance disclosure block (BD-216).

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

1. *Documentation.* In `fs-la`'s contract or README, and referenced from FrankenSim's top-level guide for
   consumers: the two variables; what a dependency-graph equivalence class is; that a consumer sets
   `FRANKENSIM_DEPGRAPH_SALT` to a name of its own for development builds, in the `[env]` table of its own
   `.cargo/config.toml`; and that an evidence-bearing performance build exports a receipt and sets
   `FRANKENSIM_DEPGRAPH_RECEIPT`.
2. *An actionable failure.* The panic of `build.rs`, when neither variable is set, names the document and
   prints the two lines a consumer must add. It still fails.
3. *Optionally, a published default class for consumers.* If FrankenSim publishes one (a documented salt value
   meaning "out-of-tree consumer, development, no performance claim"), a consumer that sets nothing builds under
   it and its receipts say so. Whether to publish one is FrankenSim's decision; this item is complete at 1 and 2.

Additive: no existing build changes; in-tree builds keep their salt.

**Interim posture here.** The core sets its own salt, naming its own development equivalence class, in its
root `.cargo/config.toml` (`FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"`); the same value is recorded
in the `build_env` field of the `FD-FSIM` row of `franken-stack.lock` and in every resolution receipt.
Evidence-bearing performance builds export a receipt. That `[env]` reaches a path dependency's build script in
this graph is a HYPOTHESIS settled by INT-FSIM-01.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-14/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-14`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_14` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It builds the probe crate with both variables unset and asserts two
things: the build fails closed (true at the pin, and it must stay true unless a default class is published);
and the failure message names the consumer document (false at the pin).

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

**What the switch-over re-runs.** The rule is "remove the local setting if a default class is published". If
one is published: the `[env]` entry is removed from this repository's `.cargo/config.toml`, the `build_env` of
the stack-lock row is updated to the published class, and INT-FSIM-01 is re-run (path dependencies on `fs-ga`,
`fs-mbd`, `fs-contact`, `fs-time` and `fs-query` pass `cargo check --all-targets` with the root patch, profile
and build-identity settings, and reproduce the digest of a G1 forward-dynamics state computed inside FrankenSim
at the pinned commit). If none is published: the local setting stays, the documentation is cited in
`DEPENDENCY_ADMISSION.md`, and the item is adopted at the documentation.

**Fallback if the change cannot land.** The local salt stays; the knowledge of why stays in this repository's
admission record.

## Acceptance criteria

- [ ] In FrankenSim: the consumer documentation exists and is reachable from `fs-la`'s contract; the build script's failure names it and prints the lines to add; the script still fails closed when nothing is set and no default class is published.
- [ ] Here: with both variables unset and the local `[env]` entry removed in a scratch copy, the probe build fails closed with the new message (or, if a default class is published, builds and records that class).
- [ ] Here: every receipt's `build_env` names the class the binary was built in, and no receipt carries FrankenSim's own in-tree salt.
- [ ] Here: INT-FSIM-01 passes at the new pin, and the G1 digest is unchanged by the change of documentation or class name.
- [ ] Here: a performance record without a build-identity receipt is still refused: under the development salt (and under a published default class, which is a development class too) `may_cite` of N33a-08 answers `dev-build-identity` for a Performance or a Parity gate, at the old pin and at the new one. (The disclosure block of the performance harness, BD-216 at M2, prints the same identity; this bead does not wait for it.)
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `build_script_message_names_the_document`: a test that runs the build script's decision function with both variables absent; expected an error whose text contains the document path and both variable names.
- `in_tree_salt_unchanged`: the in-tree configuration still yields the same identity as before.
Here (`tests/upstream/FD-FSIM-14/`, real builds):
- `fails_closed_without_identity`: scratch copy, `[env]` entry removed, both variables unset; expected `cargo check` failure in `fs-la`'s build script.
- `local_salt_reaches_the_build_script`: with the entry present, the build succeeds and the receipt's `build_env` equals `{FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"}`.
- `foreign_salt_is_rejected`: a scratch copy that sets FrankenSim's own in-tree salt value; expected `cargo xtask check-stack` failure ("build identity claims another graph's class": identifier `stack.env`, kind `foreign-build-class`, the rule of N33a-08).

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_14.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_14 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Both phases run the unset-variable build in a scratch copy under
`target/upstream-scratch/`, capture the failure text, and assert on it; phase `post` also prints the
`build_env` of the newest receipt.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_14/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-14`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer (the done-condition of INT-FSIM-01's seed bead): with the build-identity variable unset the build fails closed.
- Negative controls: a probe that builds with nothing set (and no published default) means the check was bypassed, and the script fails; a receipt with an empty `build_env` is rejected by `check-stack`.
- Determinism lanes: the receipt, including `build_env`, is byte-identical on two runs and holds no machine path.

## Considerations

- Hazard 5 is one of five hazards of consuming FrankenSim by path that are also upstream work items (the others: the unfrozen composition contract, the stale constellation lock, the nightly feature in the units crate, the contact capacity). Each keeps its interim posture until its change lands.
- The salt is part of the identity of a performance claim: the ten-fold tournament's builds all carry receipts, and a change of class name during a tournament would be a breach of its freeze.
- If Cargo's handling of `[env]` for path dependencies changes with a toolchain train, this is where it shows first; the fixture is re-run by every train.
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

Appendix F.1 (row FD-FSIM-14), F.3; §33.2 (hazard 5), §33.5 (`build_env` in the stack lock and the receipt), §33.8; INT-FSIM-01; BET-04.
