---
key: "FD-ASUP-01"
title: "upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version for all"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "upstream", "asupersync", "frankensim", "frankentorch", "fastmcp_rust", "solid", "review-notes-open"]
parent: "WP-02"
deps: ["BD-001", "BD-006", "N33c-09", "N34-03", "NFp4-01", "STD-E2E"]
needs: []
related: ["FD-FKER-01", "FD-FMCP-01", "FD-FTUI-01", "N33b-04", "N33b-08", "N33c-26", "N33c-35", "N34-16", "N41-20", "N44-04", "NF-05", "NF-13"]
gates: ["INT-ASUP-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.8", "§33.13", "§33.16", "§33.2", "§37.8", "§41"]
---
## Background

asupersync is the suite's structured-concurrency runtime: cancellation, the capability context `Cx`, and the lab
runtime used for deterministic simulation testing. Every async path of FrankenRobotics uses it, and so do the
siblings it consumes. A Cargo package is identified by name, version and source, so two requirements that
resolve to different versions, or to the same version from a path and from the registry, put **two asupersync
packages** in one binary: two runtimes, two `Cx` types that do not unify, and cancellation that silently stops
at the seam. Invariant INV-SUITE-02 forbids it: every binary links exactly one asupersync package. Upstream
register priority: **P0, done at M0** (one of two).

FACT, the starting state (read 2026-11-21 from each repository's manifests, to re-read):

| Repository | Head | asupersync requirement |
|---|---|---|
| asupersync | `b32c5ba66315` | It is 0.6.2; the checkout had 267 modified paths; `franken-kernel` and `franken-evidence` 0.6.0 live in it |
| frankensim | `483d09c682f2` | Path `../asupersync`, with a `[patch.crates-io] asupersync` entry; `Cargo.lock` resolves 0.6.2; its own `constellation.lock` still pins 0.5.0 at `7a0f2f55047c` |
| frankentorch | `8165d6a18028` | `0.6.2`, default features off |
| fastmcp_rust | `117388bcc169` | `=0.6.2` in the source tree; its registry package 0.13.0 carries 0.5.0 |
| frankensqlite | `79b735cb265a` | `0.6.2` from the registry, default features off (FrankenSim patches it to the path) |
| franken_numpy, frankenscipy | `e8cb85f4e5ac`, `edb7ae619c2d` | `=0.6.2` optional; `0.6.2` default features off |
| frankensympy | `29f847c14428` | Path `../asupersync` |
| frankentui, franken_threed, frankenjax | — | `=0.5.0` (optional executor), `=0.5.0`, `0.5.0` optional: these never enter a graph of this project |

Decision (owner, 2026-11-22): the three consumers that matter to the M0 graph, FrankenSim's constellation,
FrankenTorch and fastmcp_rust, are aligned on **one published asupersync version, V**, by changes made in those
repositories by this program. Rejected alternatives: keeping `frob-mcp` outside the core graph so that it may
bring its own asupersync (the former default, now KILL-MCP's first fallback); accepting whatever a path checkout
happens to contain (a dirty tree is not a version, and a result built against it cannot be compared with one
built elsewhere); a local patch of a sibling's manifest (it diverges from the sibling's head and does not
survive a pin move).

## Scope

In: the half of the work done here: the root patch entry; the check, with `cargo metadata` over this project's
workspaces, that every activated exact requirement on asupersync was handled; the failing fixture; the switch-over
train and the pins here (the FD-ASUP row's `version` set to V); the closing record BD-037 takes over. This bead and
its sibling half NFp4-01 are the only specification of that work: the seed bead BD-008 was drafted with a second
copy of the steps, and its three additions are folded into the two.

Out, with the bead that has it: the half done in the four sibling repositories, that is choosing V under
asupersync's maintainers' rules (the procedure and its evidence), the change in each of the four repositories,
any further exact requirement moved to V, the mirrored beads and the "before" listings (NFp4-01, a dependency:
that work needs nothing from this workspace, so it starts at M0 from the suite survey instead of waiting behind
this bead's admission dependencies); the gate INT-ASUP-01 itself, that is its measurement, its wiring into
`xtask check-stack`, the reading of `check-deps`, the gate evaluation, its admission record, and the
verification of the interim posture (BD-008, which depends on this bead and re-runs the gate as one step of the
switch-over); the rules of `check-deps` that hold the interim posture (one asupersync package per binary:
N34-03; asupersync 0.5.0 nowhere and the frankentui executor feature: N33c-09); the rest of FrankenSim's stale
constellation lock (FD-FSIM-16); frankentui's exact 0.5.0 pin (FD-FTUI-01); bringing fastmcp_rust onto T
(FD-FMCP-01).

## Technical approach

The four repositories are touched by NFp4-01, under each repository's own `AGENTS.md`, gates and reservations:
it fixes V (OPEN-SUITE-1: a published version that corresponds to one clean, tagged asupersync commit and that
every consumer in the table's first seven rows can build against; 0.6.2 if its published bytes equal a clean
commit, otherwise the next version published from a clean commit), moves FrankenSim's constellation entry, the
FrankenTorch requirement and the fastmcp_rust requirement to V, opens the four mirrored beads and writes
`before.tree-d.txt`. This bead edits no sibling file. It starts from NFp4-01's closing record: V, V's commit and
tag, the evidence for V (the two digests compared, or the release commit), the exact requirements found and what
was done with each, each repository's pre-change and landed commits, the four mirrored-bead identifiers in the
order of the row's `repo` (asupersync, frankensim, frankentorch, fastmcp_rust), and the hash of
`before.tree-d.txt`. It copies the mirror identifiers into its own closing record, from which BD-037 enters them
in the `mirror` field of this item's row in `registries/upstream.toml` (`frob.upstream/1`), and it hands the V
decision and its evidence to BD-008 for the admission record of INT-ASUP-01. DESIGN, an ordering fact of the
graph: the upstream registry BD-037 depends on this P0 item (through BD-023 and BD-008, which report its state),
so the registry, its `frob doctor` section and the scripted compatibility train of N33b-04 all come after this
bead; this bead therefore opens its own fixture, runs its switch-over by the four written steps below, and hands
BD-037 the row's content. Nothing in this repository patches, forks or vendors a sibling (INV-SUITE-04).

Fail-closed default until this bead's switch-over sets the FD-ASUP row's `version` to V: builds here use whatever
FrankenSim's pinned commit builds with (0.6.2 by path today), with the tree hash in the receipt and no gate PASS on
a dirty tree (OPEN-SUITE-3).

1. **Every activated exact requirement handled.** `cargo metadata` over every workspace here lists the exact
   requirements on asupersync that a graph of this project activates (franken_numpy's optional `=0.6.2`, behind a
   feature, is the known one); each must appear in NFp4-01's closing record, moved to V by a landed commit or
   recorded as a feature that stays off here, and a feature recorded as off is not enabled in any manifest here.
   One that is activated and appears in neither form fails the step (DESIGN: V must not be chosen in a way that
   silently disables a consumer).
2. **Here.** The workspace root repeats FrankenSim's `[patch.crates-io] asupersync` entry, so every registry
   requirement on V resolves to the one path package at V's commit. `surface/frob-mcp` depends on `fastmcp_rust`
   by commit and never on the registry package `fastmcp-rust` 0.13.0, which resolves with asupersync 0.5.0 (FACT,
   to re-read); that refusal is FD-FMCP-01's negative control and is repeated here as control 4. `franken-stack.lock`
   row `FD-ASUP` records `version = V`; the stack receipt records the asupersync revision, `dirty: false` and an
   empty `duplicates`.

**Interim posture here.** One package identity per build stays enforced by `cargo xtask check-deps` (one
asupersync package per binary; asupersync 0.5.0 nowhere; the frankentui executor feature forbidden). A surface
that would bring a second package stays out of the core graph under KILL-MCP's first fallback.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-ASUP-01/`, opened and kept current by
this bead in the form BD-037 prescribes for every upstream fixture (BD-037 opens the others; it comes after this
item and adopts this one into its registry check unchanged): one test tagged `FD-ASUP-01`, compiled as the `frob-conformance` integration test `upstream_fd_asup_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It resolves the M0 graph (the core
with the airlock, the Metal annex with `ft-kernel-metal`, and a probe on `fastmcp_rust`) with `cargo metadata`
and asserts exactly one package named `asupersync`, at one version, from one source, in each workspace; and that
each consumer's own lock agrees with V. At the starting state FrankenSim's constellation lock disagrees (0.5.0).
Beside `expected.toml` the fixture keeps `before.tree-d.txt`, the `cargo tree -d` output of each consumer
workspace before the change, written by NFp4-01 before its first manifest edit; this bead records its BLAKE3
hash in `expected.toml` and checks it against NFp4-01's closing record: the "before" half of the evidence that
the gate's "after" listing is compared with. NFp4-01 may land before a consumer is first pinned in
`franken-stack.lock`. Where a consumer's pin already contains its landed change, the fixture is run in a scratch
clone at that consumer's pre-change commit from NFp4-01's closing record, must fail there, and `observed_at`
names that commit; at the pin it passes, which is the `adopted` reading of BD-037's status-dependent fixture
check, and that consumer needs no pin move.

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

**The gate the switch-over re-runs.** INT-ASUP-01 (admission, M0; measured, evaluated and recorded by BD-008):
`cargo tree -d` shows one asupersync package in each workspace, the same published version in all of them, and
no binary links two. The workspaces are those BD-008 measures: every one that exists at the commit under test
among the core (which contains `surface/frob-mcp`), the two `spark/` annexes, `apps/`, `derive/`, `bindings/`
and `worker/`; at M0 that is at least the core and `worker/`. This is the gate that proves this item.
INT-FMCP-01 depends on it. Which check fails on what is BD-008's statement and is not repeated here beyond its
consequence for this bead's controls: a second package is a finding of `check-deps` only, and a single package
at a version other than the FD-ASUP row's is a finding of `check-stack` only.

**Fallback if the change cannot land.** `check-deps` keeps enforcing one identity per build; a surface that still
brings a second package moves to its own workspace (for `frob-mcp`, KILL-MCP fallback 1: its own workspace
talking to `frobd` over a local socket; fallback 2: the CLI alone). Computational crates would then take a
capability-free token from `frob-core` in place of asupersync's `Cx` at the split (OPEN-SUITE-6; default: `Cx`
wherever cancellation is observed, the token only in that case).

## Acceptance criteria

- [ ] NFp4-01 is closed, and its closing record gives V with its clean, tagged commit, the four mirrored-bead identifiers and each consumer's landed commit; each consumer's pin in `franken-stack.lock` is at or after its landed commit.
- [ ] Every exact requirement on asupersync that `cargo metadata` finds activated in a workspace here appears in NFp4-01's closing record, moved to V or recorded as a feature that stays off, and no feature recorded as off is enabled here.
- [ ] Here, `cargo metadata` over every workspace BD-008 measures (at M0 at least the core workspace and `worker/`) shows exactly one `asupersync` package, at V; `cargo tree -d` shows no duplicate; the receipt's `duplicates` is empty.
- [ ] A seeded second asupersync version in a scratch copy fails `cargo xtask check-deps` (the done-condition this item shares with BD-008).
- [ ] The precondition of INT-ASUP-01 holds at the new pins, measured here directly with `cargo metadata` and `cargo tree -d` (one package, at V, in every workspace that exists); or the record names the consumer that still disagrees, and KILL-MCP's fallback is decided at the M0 exit review. The gate's evaluation and its admission record are BD-008's first run, made against the new pins (BD-008 depends on this bead), as the switch-over rule's last sentence provides.
- [ ] The closing record names the fixture path, this bead, the four mirrored beads and the status reached: `landed`, or `adopted` once BD-008's run of INT-ASUP-01 passes at the new pins. BD-037, which comes after this bead, enters exactly that as the item's row in `registries/upstream.toml`, and its registry check then covers this fixture.
- [ ] `before.tree-d.txt` is present in the fixture directory and its BLAKE3 hash in `expected.toml` equals the file's and NFp4-01's closing record.
- [ ] This bead edited no sibling file. Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In the siblings (NFp4-01's, re-run here at the pins): each sibling's existing suite on the regenerated lockfile, and
FrankenSim's added test:
- `constellation_asupersync_matches_cargo_lock`: the asupersync entry of `constellation.lock` equals the version and revision `Cargo.lock` resolves. Input today: 0.5.0 against 0.6.2; expected after the change: equal.
Here (`tests/upstream/FD-ASUP-01/`, real resolution, no mocks):
- `one_asupersync_package_per_workspace`: `cargo metadata` on each workspace; expected package count 1 for the name `asupersync`.
- `all_consumers_require_v`: parses the three consumers' manifests at their pinned commits; expected requirement equal to V (an exact requirement where the sibling uses one).
- `registry_and_path_unify`: with the root patch entry present, a registry requirement on V resolves to the path source; with the patch entry removed in a scratch copy, two packages appear (this is the control that shows the test can see a duplicate).

### End-to-end script and logging
`scripts/e2e/upstream_fd_asup_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack --json` (the FD-ASUP row's version and
the asupersync entry of the receipt); `cargo test -p frob-conformance --test upstream_fd_asup_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests
(the `check-deps` controls below), asserting that they pass. Phase `post`, after the pin move: the same fixture
must pass, the gates named in this bead that exist are re-run, NFp4-01's script
(`scripts/e2e/upstream_fd_asup_01_siblings.sh`) is re-run and must pass, and the closing record must state
`adopted` or `landed` with a receipt newer than the pin move. `frob doctor` and its `upstream` section come after this bead
(BD-022 and BD-037 depend on it through BD-008), so both phases call `frob doctor --section upstream --json` (the
registered form of the section selector) only when the `frob` binary of BD-022 exists, asserting there that the
row reads as the closing record says once BD-037 has entered it; while the binary is absent, or the section does
not yet list this item, the step is logged `not-run` with the key it waits for (BD-022 or BD-037) and is never
counted as a pass. Both phases also print, per workspace, the output of `cargo tree -d` and its BLAKE3
hash, the resolved asupersync version and source, and the drift class `check-stack` gives each consumer
(`on-pin`, `stale-lock` or `drift`).

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_asup_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-ASUP-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer: for the aligned graph, `cargo tree -d` lists no asupersync line and the receipt's `duplicates` array is empty.
- Negative control 1 (the plant of GATE-WS-01): a scratch crate requiring `asupersync = "=0.5.0"` added to the workspace; `check-deps` must fail and name both versions.
- Negative control 2: enabling frankentui's `asupersync-executor` feature in a scratch manifest must fail `check-deps`.
- Negative control 3: a consumer manifest edited to `=V+1` in a scratch copy must make `all_consumers_require_v` fail.
- Negative control 4: a scratch manifest in the core workspace that names the registry package `fastmcp-rust = "0.13.0"` must fail `check-deps`, naming asupersync 0.5.0 beside V.
- Determinism lanes: the receipt computed twice on an unchanged tree is byte-identical (KA-767); the check is repeated on the Linux host and on this Mac.

## Considerations

- RISK-03: two asupersync versions in one build; three siblings still pin 0.5.0. Earliest signal: `cargo tree -d` at INT-ASUP-01. Those three (frankentui's executor, franken_threed, frankenjax) are never Cargo dependencies here.
- A dirty asupersync checkout is acceptable for development builds between receipts, with the tree hash in the receipt, and for nothing else (OPEN-SUITE-3, owner; fail-closed default as stated).
- An exact requirement (`=V`) in a consumer is brittle: the next asupersync release splits the graph again until every consumer moves. Each later move is one compatibility train (N33b-04, which absorbed the draft NF-01), and the registry row records the consumers that pin exactly.
- `franken-kernel` and `franken-evidence` live in the asupersync repository; FD-FKER-01 touches the same repository and should share a reservation window with this item's sibling half, NFp4-01.
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
- For this item specifically: if it cannot land within the M0 budget after being attempted, KILL-MCP's fallbacks apply; M1 depends on asupersync only as one package, which the interim posture already guarantees.

## References

Appendix F.1 (row FD-ASUP-01), F.3; §33.8, §33.13 (consequence 3), §33.16 (INV-SUITE-02, OPEN-SUITE-1, -3, -6), §33.2 (hazard 3), §37.8; gate INT-ASUP-01; KILL-MCP; RISK-03.
