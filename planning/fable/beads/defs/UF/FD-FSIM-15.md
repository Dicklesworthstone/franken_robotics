---
key: "FD-FSIM-15"
title: "upstream frankensim: `fs-qty` macro-generated dimension products behind a feature (no `generic_const_exprs`)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "suite", "workspace", "upstream", "frankensim", "solid"]
parent: "WP-01"
deps: ["BD-031", "BD-037", "N33b-04", "NF-02", "STD-E2E"]
needs: []
related: ["FD-FSIM-31", "N33b-08", "N33c-02", "N33c-05", "N33c-06", "N33c-08", "N33c-47", "NF-03", "NF-05"]
gates: ["INT-TOOLCHAIN-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.12", "§33.2", "§33.13", "§33.16", "§41"]
---
## Background

Rule L-6 of this project's workspace puts FrankenSim's quantity types in every public signature above layer 1,
so that a length cannot be passed where a torque is expected. Those types come from `fs-qty` (8,950 library
lines: compile-time dimensional analysis, runtime-checked quantities, SI parsing; six base dimensions in the
order metre, kilogram, second, kelvin, ampere, mole, `DIMENSION_COUNT = 6`). FACT: `fs-qty` is the one crate in
FrankenSim that enables a nightly feature, `generic_const_exprs`, which the compiler marks incomplete
(`frankensim/crates/fs-qty/src/lib.rs:20–21`). `fs-evidence` depends on it directly, and through `fs-evidence`
so do `fs-ivl` and nearly every physics crate. The whole build graph is therefore tied to nightlies on which an
incomplete feature still compiles one crate (RISK-21, severity high). Upstream register priority: **P1**.

The coupling can bite in three ways: a toolchain train moves T to a nightly where the feature has regressed
(candidate (a) for T is such a move for FrankenSim, from `nightly-2026-07-06` to `nightly-2026-08-31`); a crate
in the graph needs a newer compiler than any nightly on which `fs-qty` compiles; or a stable-only target
appears (firmware export, a Moonshot).

FACT: `fs-qty` documents macro-generated dimension products as its own fallback. This item lands that fallback
behind a feature. It is the preferred fix; the alternative on this side, a small stable crate `frob-units`
(layer 0, standard library only, at most 3 thousand lines), is an escape hatch kept cheap and closed. Rejected:
opening the hatch pre-emptively (two unit systems in one suite, with conversions at the airlock forever);
pinning T forever to a nightly where the feature works (the suite's common nightly keeps moving).

## Scope

In: in FrankenSim, a feature of `fs-qty` under which no nightly feature is enabled and dimension products come
from a declarative macro over a closed set; here, the re-run of INT-TOOLCHAIN-01 and the update of the
`frob-units` decision. Out: `frob_core::qty` and the import rule (BD-005, BD-003); the matrix itself (BD-031);
removing `fs-qty` from `fs-evidence` (FD-FSIM-31, the smaller change listed beside this one).

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

**The change (DESIGN; the fallback is `fs-qty`'s own documented design).** A Cargo feature of `fs-qty`
(working name `macro-dims`), off by default:

1. With it on, the crate does not enable `generic_const_exprs`, and compiles on a stable compiler.
2. A dimension is the vector of six signed exponents (metre, kilogram, second, kelvin, ampere, mole). The typed
   quantities are a closed set of named aliases. Products and quotients among them are generated by a
   declarative macro from one table: for every pair (A, B) in the table whose exponent sum (or difference) is
   itself in the set, `impl Mul<B> for A` (or `Div`) with that output.
3. Two aliases may share a dimension and still be distinct types: torque and energy both have exponents
   (kilogram 1, metre 2, second −2), and adding one to the other must not compile. A trait has one `Output`
   per operand pair, so where an exponent sum matches two aliases the table names the output explicitly: the
   type `fs-qty` returns for that product with the feature off (re-read at the pin; for force × length, the one
   it names), and the other alias is reached only through the named operation the crate already provides for it
   (for torque, the moment of a force about a point). The macro rejects a table in which one pair has two
   outputs, and the differential test compares output types as well as dimension vectors.
4. A product whose result is not in the set does not compile, with a message naming the missing alias; data
   loaded at run time uses the existing runtime-checked quantity.
5. The public names and methods of the aliases are identical under both feature settings, so downstream code
   compiles under either. **Lossiness, self-described:** under the feature, open-ended dimension arithmetic is
   replaced by the closed table; the crate documentation says so and lists the table.

Additive: with the feature off, nothing changes.

**Interim posture here.** The core never imports `fs_qty` by name outside `frob-core` and `frob-fsim`; quantity
types reach every other crate through the one re-export module `frob_core::qty` (`check-deps` enforces it). The
hatch stays closed: that module re-exports `fs-qty`. If INT-TOOLCHAIN-01 forces it, the module re-exports
`frob-units` instead, and the change is one module plus the conversions in the airlock.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-15/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-15`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_15` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It builds `fs-qty` alone at the pinned commit with the feature on and no
nightly feature available (a stable toolchain, or the pinned nightly with `-Z allow-features=` empty); at the
pin the feature does not exist and the build fails.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) write a new resolution receipt (`frob.stack-receipt/1`) under `receipts/`
with `cargo xtask check-stack --write-receipt --reason pin-move --gate INT-FSIM-01` (plain `check-stack` only
verifies and writes nothing); (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** INT-TOOLCHAIN-01: the matrix is recorded again, with a row "`fs-qty` alone,
feature `macro-dims`" on every column (the two candidates for T, `nightly-2026-07-06` and `nightly-2026-08-31`,
and the two starting pins `nightly-2026-08-20` and `nightly-2026-08-25`) and on a stable release; every
required row must pass on T. The decision rule is then re-read: T is the first candidate, in the order (a) then
(b), on which every row passes. If `fs-qty` was what kept T at candidate (b), the toolchain train to (a) is
scheduled (NF-02, NF-05). The ADR on `frob-units` is updated to "hatch not needed while the feature exists".

**Fallback if the change cannot land.** T stays on a nightly where `generic_const_exprs` compiles `fs-qty`. If
`fs-qty` fails on T after a later toolchain train and this change cannot land within that train, `frob-units`
is created: a dimension as six signed 8-bit exponents in `fs-qty`'s order, a closed alias set generated by a
declarative macro, a checked runtime quantity, conversions to and from `fs-qty` in `frob-fsim` only, alias set
at most 64 (TARGET; above that the product table is no longer reviewable and the question returns to the owner
as OPEN-SUITE-4, whose default alias set is the quantities used by the Marrow, Skeleton, Sinew, Kinesis, Phylum
and Traction chapters only).

## Acceptance criteria

- [ ] In FrankenSim: `fs-qty` builds with the feature on a stable release and on all four nightlies in play, with no `#![feature]` attribute active; with the feature off its behaviour and tests are unchanged.
- [ ] In FrankenSim: a differential test shows every alias product and quotient in the table has the same dimension under both feature settings; the table is listed in the crate documentation.
- [ ] In FrankenSim: compile-fail tests: a pressure added to a time; a torque added to an energy; a product outside the table.
- [ ] Here: the INT-TOOLCHAIN-01 matrix is re-recorded with the new row and every row passes on T; the record states whether the choice of T changes.
- [ ] Here: a direct `fs_qty` import outside `frob-core` and `frob-fsim` still fails `cargo xtask check-deps`.
- [ ] Here: the `frob-units` ADR is updated, and `frob-units` does not exist in the workspace unless that ADR says it was forced.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-qty`):
- `macro_products_match_const_generic`: for every pair in the table, the output dimension vector equals the const-generic result. Spot values: force × length = energy (1, 2, −2 in kilogram, metre, second); force ÷ area = pressure (1, −1, −2); energy ÷ time = power (1, 2, −3); torque ÷ length = force (1, 1, −2).
- `pressure_plus_time_does_not_compile`, `torque_plus_energy_does_not_compile`, `unknown_product_does_not_compile`: compile-fail tests with the expected diagnostics.
- `value_bits_survive_feature_switch`: 10⁴ seeded quantities serialized under one setting and read under the other are bit-identical.
- `shared_dimension_products_keep_their_type`: force × length has the same output type under both settings, and a downstream test crate that forms a torque through the moment operation and an energy through force × length compiles unchanged under both; a table with two outputs for one pair fails to expand.
Here:
- `fs_qty_alone_builds_without_nightly_features`: the fixture.
- `qty_reexport_is_the_only_route`: a scratch crate importing `fs_qty` directly fails `check-deps`.
- `airlock_round_trip_preserves_bits`: a quantity through `frob_core::qty` and the airlock and back; bit-identical.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_15.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_15 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also re-runs the matrix row for `fs-qty` on each installed channel with
`scripts/train/toolchain_train.sh --to <channel> --dry-run` (NF-02's script, a dependency; its dry run stops
after the matrix and changes no file), and logs one line per cell.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_15/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-15`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-779: every alias product agrees in a differential test while both implementations exist; a round trip through the airlock conversion preserves the value bits; a pressure added to a time fails to compile. Negative control of KA-779: a torque added to an energy must fail to compile although both have dimension vector (kilogram 1, metre 2, second −2). CX-354: "a dimension vector is all the typing a quantity needs".
- Negative control: a table entry with a wrong exponent (energy ÷ time given the dimension of force) must fail the differential test.
- Determinism lanes: exact (class D0); the differential test is run on every matrix column.

## Considerations

- RISK-21 and RISK-02: this item is the preferred mitigation of the first and removes the most likely cause of a stalled toolchain train. BET-04 (FrankenSim can be consumed by path, and the nightly coupling through `fs-qty` does not block the core) is tested at INT-TOOLCHAIN-01.
- Of 22 crates coupled to `fs-qty`, 12 are reported to depend on it directly and 10 only through another crate (FACT to recheck; the matrix run records it). This item fixes the coupling at its source for all 22; FD-FSIM-31 frees 8 with a smaller change. They are complements, and either alone lowers the risk.
- Cost, stated: the closed table loses open-ended dimension arithmetic; a quantity outside it must be added to it or carried as a checked runtime value.
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

Appendix F.1 (row FD-FSIM-15), F.3; §33.12 (the risk, the matrix, the decision rule, the escape hatch), §33.2 (hazard 4), §33.13, §33.16 (OPEN-SUITE-4); INT-TOOLCHAIN-01; KA-779; CX-354; RISK-21, RISK-02; BET-04.
