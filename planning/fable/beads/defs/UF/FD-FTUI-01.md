---
key: "FD-FTUI-01"
title: "upstream frankentui: the exact asupersync 0.5.0 pin on its optional executor relaxed"
type: "task"
priority: 3
milestone: "M3"
labels: ["m3", "suite", "tether", "upstream", "frankentui", "frontier"]
parent: "WP-52"
deps: ["BD-037", "NF-01", "STD-E2E"]
needs: []
related: ["BD-300", "BD-003", "FD-ASUP-01", "BD-306"]
gates: ["INT-FTUI-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.15", "§33.13", "§33.4", "§34.6", "§42"]
---
## Background

`frob-tui` is the terminal dashboard of FrankenRobotics: it shows jobs, races, budgets and gate status to a
person watching a campaign. It is built on the sibling project frankentui (FACT, read 2026-11-21: version
0.10.0, twenty crates under `frankentui/crates`, about 1.15 million lines; `ftui-harness` supports headless
tests). It reads the daemon's event stream over the local socket and links no physics crate. Tier: Frontier; a
convenience, with no claim and no agent workflow depending on it.

FACT: `ftui-runtime` carries an **optional exact pin on asupersync 0.5.0** behind the feature
`asupersync-executor` (`crates/ftui-runtime/Cargo.toml:56, :81`). This project's build graph holds exactly one
asupersync package, at the one published version the consumed siblings are aligned on (0.6.2 at the time of
writing), and asupersync 0.5.0 never enters a graph. So frankentui is consumed with default features off and
`runtime` and `native-backend` on, and **the executor feature is forbidden** here; `xtask check-deps` enforces
it. Upstream register priority: **P3**.

The executor is not needed today: the dashboard runs on frankentui's own runtime. The item exists so that the
prohibition is a choice and not a trap: if a later version of the dashboard wants frankentui's async executor
integration, the only obstacle should not be a stale exact pin. Rejected: enabling the feature and accepting
two asupersync packages in the dashboard binary (invariant INV-SUITE-02: every binary links exactly one).

## Scope

In: in frankentui, the executor feature's asupersync requirement changed to one that admits the suite's
published version; here, nothing unless the executor is wanted. Out: the dashboard and its admission gate
(BD-300); the alignment of the consumed siblings on one asupersync version (FD-ASUP-01).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankentui` repository
(`../frankentui` beside this one; head `5979b92cc576` and toolchain channel `nightly-2026-08-31` when this was written
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

**The change (DESIGN).** In `ftui-runtime`'s manifest the optional dependency `asupersync = "=0.5.0"` becomes a
requirement that admits the suite's published version V (either `=V`, matching how other siblings pin, or a
range that contains it). The executor glue is ported to V's API where it changed between 0.5.0 and V, and
frankentui's own tests for the feature run against V. With the feature off, nothing changes for any consumer.
If frankentui wishes to keep building against 0.5.0 as well, the two are separate features; that choice is its
own.

**Interim posture here.** The executor feature is forbidden: `check-deps` fails any manifest that enables
`asupersync-executor`, and fails any graph that contains asupersync 0.5.0.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FTUI-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FTUI-01`, compiled as the `frob-conformance` integration test `upstream_fd_ftui_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It resolves a probe crate that depends on `ftui-runtime` with
`asupersync-executor` enabled inside a workspace that also depends on the graph's asupersync, and asserts that
`cargo metadata` shows exactly one asupersync package; at the pin it shows two (0.5.0 and the graph's).

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

**Adoption.** The register says: **none needed unless the executor is wanted.** If it is wanted, by a recorded
decision of the dashboard's owner: the `check-deps` rule is narrowed from "the feature is forbidden" to "the
feature is allowed only while the graph resolves one asupersync package", and INT-FTUI-01 is re-run with the
feature on (one asupersync package or none in its graph; the job pane rendered from a recorded event stream
equals its stored snapshot in the headless harness; idle processor use at most 2% of one core). If it is not
wanted, the pin still moves through a compatibility train so that the registry row can read `adopted`, and the
prohibition stays as it is.

**Fallback if the change cannot land.** The feature stays forbidden. For the dashboard as a whole the kill
condition is unchanged: INT-FTUI-01 fails, or frankentui cannot be built without its executor; then there is no
dashboard and the same events are available as line-delimited JSON.

## Acceptance criteria

- [ ] In frankentui: with `asupersync-executor` on, `cargo tree -d` in its own workspace shows no asupersync 0.5.0 and the feature's tests pass against V; with it off, nothing changes.
- [ ] Here: the fixture shows exactly one asupersync package with the feature enabled, at the new pin.
- [ ] Here: `check-deps` still fails a graph containing asupersync 0.5.0, at every pin; the rule on the feature is either unchanged (executor not wanted) or narrowed by a recorded decision.
- [ ] Here: if the feature is enabled for the dashboard, INT-FTUI-01 passes with it on and its record says so.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In frankentui (added):
- `executor_feature_builds_against_suite_asupersync`: the feature's existing tests under V.
- `default_features_unchanged`: the resolved graph with the feature off is identical before and after.
Here (`tests/upstream/FD-FTUI-01/`):
- `one_asupersync_with_executor`: the fixture's `cargo metadata` assertion.
- `check_deps_forbids_0_5_0`: a scratch manifest requiring `asupersync = "=0.5.0"`; expected failure naming both versions.
- `check_deps_feature_rule`: a scratch manifest enabling `asupersync-executor`; expected failure while the rule is "forbidden", and success after a narrowed rule only if one package resolves.
- `job_pane_matches_snapshot`: the headless harness renders a recorded event stream; the frame equals its stored snapshot (run with the feature off, and with it on if adopted).

### End-to-end script and logging
`scripts/e2e/upstream_fd_ftui_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_ftui_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also starts `frobd`, replays a recorded job event stream into `frob-tui`
under the headless harness, compares the pane with its snapshot, and measures idle processor use over 60 s
against 2% of one core.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_ftui_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FTUI-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer: `cargo tree -d` lists no asupersync line for the dashboard's graph.
- Negative controls: the 0.5.0 manifest above (the same plant as the workspace gate's "a second asupersync package"); a snapshot with one cell altered must fail the pane comparison.
- Determinism lanes: the headless frame is byte-identical on repeated renders of one event stream.

## Considerations

- RISK-03: two asupersync versions in one build; three siblings pin 0.5.0 (frankentui's executor, franken_threed, frankenjax). This item removes one of the three; the other two are never Cargo dependencies here.
- OPEN-WS-2: `frob-tui` stays in the core workspace; if frankentui does not build on the toolchain T, the remedy is an upstream change in frankentui or a toolchain train, with no dashboard until it lands, and the crate gets no toolchain of its own.
- The item is the smallest in the register: one manifest line and whatever the API drift between two asupersync versions requires in one feature's glue.
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

Appendix F.1 (row FD-FTUI-01), F.3; §33.15 (frankentui), §33.13 (consequence 3), §33.4 (FD-FTUI), §34.6 (`check-deps`), §42 (OPEN-WS-2); INT-FTUI-01; INV-SUITE-02; RISK-03.
