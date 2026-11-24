---
key: "FD-FSIM-31"
title: "upstream frankensim: the `fs-qty` dependency of `fs-evidence` removed or put behind a feature"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "workspace", "upstream", "frankensim", "solid"]
parent: "WP-01"
deps: ["BD-031", "BD-037", "N33b-04", "NF-02", "STD-E2E"]
needs: []
related: ["N33c-02", "N33c-05", "N33c-08", "N33c-47", "NF-03"]
gates: ["INT-TOOLCHAIN-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.12", "§33.3", "§33.9", "§41"]
---
## Background

`fs-qty`, FrankenSim's units crate, enables the nightly compiler feature `generic_const_exprs`, which the
compiler marks incomplete (FACT: `frankensim/crates/fs-qty/src/lib.rs:20–21`). Every crate that depends on it,
directly or through another crate, can be built only on a nightly where that feature still compiles. The
structure of the coupling (FACT to recheck: a reported reading of the dependency graph that the plan did not
repeat; INT-TOOLCHAIN-01 records it): **of 22 crates coupled to `fs-qty`, 12 depend on it directly and 10 only
through another crate, and removing `fs-qty` from `fs-evidence` alone would free 8 of the 22.**

`fs-evidence` (about 37 thousand library lines: `Evidence<T>` and `Certified<T>` with conservative composition)
depends on `fs-qty` directly, and it is the hub: `fs-ivl` (outward-rounded intervals, affine forms, Taylor
models, exact predicates) reaches `fs-qty` only through it. Both are on this project's bedrock-direct list,
used in signatures everywhere. Upstream register priority: **P2**. This is a smaller change than FD-FSIM-15
(macro-generated dimension products in `fs-qty` itself) and is listed beside it; the interim posture and the
switch-over rule are the same.

Why both items exist. FD-FSIM-15 removes the nightly feature at its source, for all 22 crates, and is the
larger change. This one shrinks the blast radius: the evidence and interval types, which are the vocabulary of
every certificate, stop depending on the one crate that needs an incomplete feature. Rejected: a local copy of
`fs-evidence` without the dependency (a lift of 37 thousand lines of the suite's evidence semantics).

## Scope

In: in FrankenSim, `fs-evidence` buildable without `fs-qty` (removed, or optional behind a feature that keeps
today's behaviour by default); here, the matrix re-recorded and the list of coupled crates re-read. Out:
`fs-qty` itself (FD-FSIM-15); the `frob-units` escape hatch (decided by BD-031's rule).

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

1. *Find the uses.* List every item of `fs-evidence` that names an `fs-qty` type (quantity-typed bounds,
   tolerances or constructors).
2. *Separate them.* Put those items behind a Cargo feature (working name `qty`), on by default so that every
   existing consumer sees no change; with the feature off, `fs-evidence` has no dependency on `fs-qty` and the
   dimension-free core (`Evidence<T>`, `Certified<T>`, composition, the weakest-input rule) is complete. Where
   a quantity-typed convenience is thin, moving it to a small adapter crate that depends on both is an
   equivalent form; FrankenSim's layer checks decide.
3. *Let dependants choose.* Crates that reach `fs-qty` only through `fs-evidence` (at least `fs-ivl`) depend on
   it with `default-features = false` where they do not use the quantity-typed items, and re-export the
   feature where they do.
4. *Prove it.* With the feature off, `fs-evidence` and `fs-ivl` build with no `#![feature]` active anywhere in
   their closure; the count of crates coupled to `fs-qty` is recomputed and recorded.

Additive: default features reproduce today's graph exactly.

**Interim posture here.** As FD-FSIM-15: quantities reach every crate of this project through the one
re-export module `frob_core::qty`; a direct `fs_qty` import outside `frob-core` and `frob-fsim` fails
`check-deps`; `frob-units` only if INT-TOOLCHAIN-01 forces it.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-31/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-31`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_31` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It runs `cargo tree -i fs-qty` on a probe that depends on `fs-ivl` and
`fs-evidence` with default features off and asserts that `fs-qty` is absent; at the pin it is present. It also
recomputes the two counts (direct and indirect dependants of `fs-qty` among FrankenSim's crates) from
`cargo metadata` and stores them, which is the recheck of the reported 12 and 10.

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

**What the switch-over re-runs.** INT-TOOLCHAIN-01, **and the list of coupled crates is re-read**: the matrix
gains rows "`fs-evidence` without `qty`" and "`fs-ivl` without `qty`" on every column and on a stable release,
and the record states the recomputed counts before and after (reported: 22 coupled, 8 freed). Then the
decision rule is re-read as for FD-FSIM-15. Here, crates that need only evidence and interval types (the
certificate proposers; the bedrock-direct users that never name a quantity) switch to the feature-off form. What
that buys must be stated exactly, because Cargo unifies features over the packages one build selects: if any
package in a build enables `qty` on `fs-evidence` (it is on by default), every user of `fs-evidence` in that
build links `fs-qty`; and while the escape hatch is closed `frob-core` itself depends on `fs-qty` (its `qty`
module), so every crate of the core workspace stays coupled through `frob-core` whatever form it uses. The
feature-off form therefore frees a crate only (a) in a build whose closure enables `qty` nowhere and does not
contain `frob-core` (the standalone probes of the matrix, an out-of-tree worker), or (b) after the escape hatch
opens and `frob-core` re-exports `frob-units` instead, when it is what keeps a regression of
`generic_const_exprs` on a future T from reaching the evidence and interval crates. The record of the
switch-over states, per crate switched, which of (a) and (b) applies; it does not claim a count of freed crates
of this project while `frob-core` links `fs-qty`.

**Fallback if the change cannot land.** The coupling stays as it is and is carried by FD-FSIM-15 or, failing
that, by `frob-units`.

## Acceptance criteria

- [ ] In FrankenSim: `fs-evidence` and `fs-ivl` build with the feature off on a stable release, with no nightly feature in their closure; with default features, every existing test and the resolved graph are unchanged.
- [ ] In FrankenSim: the recomputed counts of crates coupled to `fs-qty`, before and after, are recorded in the mirrored bead.
- [ ] Here: the fixture shows `fs-qty` absent from the feature-off probe's tree at the new pin.
- [ ] Here: the INT-TOOLCHAIN-01 matrix is re-recorded with the two new rows, every required row passes on T, and the record holds the reported and the measured counts side by side.
- [ ] Here: `frob-cert` still has an empty dependency table (this item does not make `fs-evidence` a dependency of the auditor).
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `evidence_core_without_qty`: the crate's dimension-free tests run with `--no-default-features`; expected all pass.
- `composition_is_feature_independent`: 10³ seeded compositions of evidence values give identical classes and canonical bytes with the feature on and off.
- `ivl_enclosures_without_qty`: interval and affine-form enclosure tests with the feature off; expected unchanged results.
Here (`tests/upstream/FD-FSIM-31/`):
- `fs_qty_not_in_feature_off_closure`: the `cargo tree -i` assertion.
- `coupled_crate_counts`: computes direct and indirect dependants of `fs-qty` from `cargo metadata` at the pin; expected to match the stored record for that pin (reported values at the starting pin: 12 and 10; a different measured value is recorded as the correction, not as a failure).
- `default_graph_unchanged`: the resolved package set of the M1 closure with default features is identical before and after the pin move, apart from the new feature name.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_31.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_31 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Both phases print the two counts and the list of coupled crates; phase `post` also runs
the two new matrix rows on every column and on a stable release with the matrix harness of N33c-01
(`scripts/admission/toolchain_matrix.sh --sets <the two feature-off probe sets> --channels <the matrix columns>,<stable>`;
the two probe sets are added in that harness's probe form, `tests/admission/toolchain/<set>/` with default features
off), then the dry matrix on the current T with `scripts/train/toolchain_train.sh --to <T> --dry-run` (NF-02's one
dry-run entry point; `--to` is required), and logs one line per cell.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_31/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-31`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer (reported, FACT to recheck): 22 coupled crates, 12 direct and 10 indirect; 8 freed by this change, leaving 14.
- Negative controls: a probe that enables the `qty` feature must show `fs-qty` in its tree again (so that the assertion can fail); an `fs-evidence` built feature-off that still links a quantity type must fail to compile.
- Determinism lanes: the counts and the trees are exact and identical on every platform.

## Considerations

- RISK-21: the `fs-qty` coupling ties the whole build graph to nightlies on which an incomplete feature still builds one crate. This item is the cheaper of the two preferred mitigations and should land first if both are in flight. Its count of 8 freed crates is a count inside FrankenSim's graph; for this project the freeing is conditional on feature unification and on `frob-core`'s own dependency, as stated in the switch-over above.
- `fs-checker`, FrankenSim's standalone evidence-package checker, is a pattern for this project's auditor and is not linked "because it depends on `fs-qty` and the auditor may depend on nothing"; freeing `fs-evidence` does not change that decision.
- FD-FSIM-09 (a device-receipt type in the evidence crates) touches the same crate; share the reservation.
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

Appendix F.1 (row FD-FSIM-31), F.3; §33.12 (the structure of the coupling; the decision rule), §33.3 (bedrock-direct table), §33.9; INT-TOOLCHAIN-01; RISK-21; BET-04.
