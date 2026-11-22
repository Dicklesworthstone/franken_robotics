---
key: "FD-FTX-01"
title: "upstream franken_textiles: exact `serde_json` pin `=1.0.151` relaxed to admit the graph's 1.0.154"
type: "task"
priority: 4
milestone: "M6"
labels: ["m6", "suite", "habitat", "upstream", "franken_textiles", "moonshot"]
parent: "WP-73"
deps: ["BD-037", "NF-01", "STD-E2E"]
needs: ["§33.15: INT-FTX-01 with `ftx-drape` run as a worker process in its own workspace (BD-416), which is the baseline this item's switch-over is compared against"]
related: ["BD-416", "BD-415", "FD-F3D-01", "FD-FSIM-22", "BD-003"]
gates: ["INT-FTX-01", "GATE-WS-06"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.17", "§33.15", "§33.14", "§34.14", "§41"]
---
## Background

For the cloth evaluation tier (a Moonshot: one rug or curtain resolved beside one world on the CPU referee,
for finalists only), the reference is franken_textiles' drape solver: `crates/ftx-drape`, about 116 thousand
lines with `cloth.rs`, `contact.rs` and `integrator.rs` (FACT, read 2026-11-21). Its admission gate is
INT-FTX-01: a square cloth draped over a sphere matches `ftx-drape`'s settled shape to its own stated
tolerance.

FACT (confirmed in each `Cargo.toml`): franken_textiles and franken_threed pin `serde_json = "=1.0.151"`
exactly; fastmcp_rust pins `=1.0.154`; frankenscipy requires 1.0.154 or later, and FrankenSim's graph resolves
1.0.154 through it. A Cargo graph holds one version of `serde_json` 1.x, so **the cloth oracle cannot be a
dependency of any crate that also links FrankenSim**. Upstream register priority: **P3**; the consuming tier is
Moonshot, so this bead is backlog and off every critical path.

So `ftx-drape` is reached as a worker process in its own workspace that exchanges whole jobs, the posture
already defined for FrankenSim's fallback, and is never a Cargo dependency. That works, at the cost of a
second build and a process hop for an oracle that would otherwise be a development dependency of the
conformance crate. Rejected: forcing one version with a `[patch]` of the sibling's manifest (a local patch of a
sibling); two `serde_json` versions (impossible within 1.x in one graph).

## Scope

In: in franken_textiles, the exact pin relaxed to a requirement that admits 1.0.154; here, a retry of the
oracle as a development dependency. Out: the worker-process route and the cloth tier's epic (BD-416); the same
relaxation in franken_threed (FD-F3D-01); dynamic rods and cloth in FrankenSim (FD-FSIM-22).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `franken_textiles` repository
(`../franken_textiles` beside this one; head `ecb13277aa02` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions.

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

**The change (DESIGN).** The workspace requirement `serde_json = "=1.0.151"` becomes one that admits both
1.0.151 and the version this project's graph resolves (1.0.154 today): a caret or bounded range such as
`">=1.0.151, <1.1"`. The sibling's own lockfile may keep 1.0.151, so its builds do not change. If the exact pin
exists because franken_textiles depends on byte-for-byte output of that version (number formatting, map
order), that reason is written down and tested there first: its golden outputs must be identical under
1.0.154, or the item stops with that finding recorded.

**Interim posture here.** `ftx-drape` runs as a worker process in its own workspace; it is never a Cargo
dependency. `cargo xtask check-deps` reports exactly one `serde_json` version per binary and fails a graph that
adds `ftx-drape` to the core workspace (part of GATE-WS-06).

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FTX-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FTX-01`, compiled as the `frob-conformance` integration test `upstream_fd_ftx_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It runs `cargo metadata` on a scratch workspace in which `ftx-drape` at the
pinned commit is a development dependency of a stand-in for `frob-conformance` beside FrankenSim's crates, and
expects resolution to succeed with one `serde_json`; at the pin Cargo reports the conflict between `=1.0.151`
and `>=1.0.154`.

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

**What the switch-over re-runs.** INT-FTX-01 **with `ftx-drape` as a dev-dependency of `frob-conformance`**;
**keep the worker if `cargo metadata` still reports a conflict** (another exact pin may remain: its asupersync
requirement was not read for the plan). If resolution succeeds: the cloth oracle test is run in process and
must give the same settled shape as the worker route, to the oracle's own tolerance; the unified feature set
of `serde_json` in every production binary must be unchanged (a development dependency must not switch on
features such as insertion-ordered maps or arbitrary-precision numbers for shipped binaries); and the rule in
`check-deps` that fails `ftx-drape` in the core graph is narrowed to "development dependency of the
conformance crate only". The worker route is kept as the fallback either way.

**Fallback if the change cannot land.** The worker process. If the cloth tier itself is not funded (KILL-LIVING
decides whether coupled islands are built at all), this item is closed as not needed; without the oracle
there is no cloth tier, only rigid proxies.

## Acceptance criteria

- [ ] In franken_textiles: the requirement admits 1.0.154; its test suite and golden outputs pass with `serde_json` 1.0.151 and with 1.0.154; any reason for the exact pin is documented.
- [ ] Here: the fixture's `cargo metadata` succeeds with exactly one `serde_json` at the new pin, or the remaining conflict is recorded by package and requirement and the worker is kept.
- [ ] Here: if resolved, INT-FTX-01 passes in process and through the worker with shapes equal within the oracle's tolerance.
- [ ] Here: for every production binary, `check-deps` prints the same `serde_json` version and unified feature set before and after; canonical bytes of the golden artifacts are unchanged.
- [ ] Here: `ftx-drape` appears in no production binary's dependency tree.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In franken_textiles (added):
- `goldens_identical_under_both_serde_json_versions`: its serialized fixtures are byte-identical when built against 1.0.151 and 1.0.154.
Here (`tests/upstream/FD-FTX-01/`):
- `resolution_with_ftx_drape_dev_dependency`: the fixture.
- `one_serde_json_per_binary`: `check-deps` output for every binary; expected one version each.
- `feature_set_unchanged_by_dev_dependency`: the unified feature list of `serde_json` for each production binary equals the stored list in the stack lock.
- `ftx_drape_in_core_graph_is_refused`: a scratch manifest adding it as a normal dependency of a core crate; expected `check-deps` failure.
- `drape_shape_worker_equals_inprocess`: a 1 m square cloth of 33 × 33 nodes draped over a sphere of radius 0.25 m; the settled node positions agree within the oracle's stated tolerance, and the four corners hang at equal height to 10⁻⁶ m.

### End-to-end script and logging
`scripts/e2e/upstream_fd_ftx_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_ftx_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs the cloth oracle through the worker and, if resolved, in process,
and logs the maximum node disagreement against the tolerance, and each binary's `serde_json` version and
feature set.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_ftx_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FTX-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the symmetric drape (four corners at equal height); one `serde_json` version per binary.
- Negative controls: a scratch graph with the old exact pin must fail resolution (so that the fixture can fail); a development dependency that enables a `serde_json` feature for a production binary must be caught by the feature-set comparison; a golden digest that changes with no change to its artifact is a failure of this item, not a golden to update.
- Determinism lanes: `cargo metadata` output is deterministic; the drape comparison is run twice on one machine.

## Considerations

- Program risk (medium): an exact version pin in a sibling blocks an oracle, because two siblings require different exact versions of one package; earliest signal: dependency resolution reports two exact requirements on one package. Mitigation: the oracle runs as a worker process; the pin move is this item.
- Program risk (high): a dependency feature changes canonical bytes. Canonical bytes come only from this project's writer, and the stack lock records the feature set; this item's feature-set test is the guard for the dev-dependency route.
- fastmcp_rust's own exact pin (`=1.0.154`) means the graph's version is not free to move either; a later `serde_json` release would need that pin relaxed too. It is noted here and is not part of this item.
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

Appendix F.1 (row FD-FTX-01), F.3; §33.17 (row 7 and its consequence), §33.15 (franken_textiles), §33.14 (the worker posture), §34.14 (the JSON feature guard), §41; INT-FTX-01; GATE-WS-06; KILL-LIVING.
