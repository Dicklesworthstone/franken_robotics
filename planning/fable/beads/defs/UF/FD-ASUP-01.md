---
key: "FD-ASUP-01"
title: "upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version for all"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "upstream", "asupersync", "frankensim", "frankentorch", "fastmcp_rust", "solid"]
parent: "WP-02"
deps: ["BD-001", "BD-006", "N33c-09", "N34-03", "STD-E2E"]
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

In: choosing V with asupersync's maintainers' rules (the procedure and its evidence); the change in each of the
four repositories; the mirrored beads; the failing fixture; the switch-over train and the pins here (the
FD-ASUP row's `version` set to V). This bead is the only specification of that work: the seed bead BD-008 was
drafted with a second copy of the steps, and its three additions are folded in below.

Out, with the bead that has it: the gate INT-ASUP-01 itself, that is its measurement, its wiring into
`xtask check-stack`, the reading of `check-deps`, the gate evaluation, its admission record, and the
verification of the interim posture (BD-008, which depends on this bead and re-runs the gate as one step of the
switch-over); the rules of `check-deps` that hold the interim posture (one asupersync package per binary:
N34-03; asupersync 0.5.0 nowhere and the frankentui executor feature: N33c-09); the rest of FrankenSim's stale
constellation lock (FD-FSIM-16); frankentui's exact 0.5.0 pin (FD-FTUI-01); bringing fastmcp_rust onto T
(FD-FMCP-01).

## Technical approach

Four repositories are touched. The same rules bind each: its own `AGENTS.md`, gates and conventions govern; every
file is reserved through Agent Mail before it is edited, because other agents work there (asupersync's checkout
had 267 modified paths: nobody else's uncommitted work is touched); the change is additive and lands on that
repository's `main` through its own gates; each repository gets its own mirrored bead, and the four identifiers
are written into this bead's closing record, each beside its repository and in the order of the row's `repo`
(asupersync, frankensim, frankentorch, fastmcp_rust), from which BD-037 enters them in the `mirror` field of this
item's row in `registries/upstream.toml` (`frob.upstream/1`). DESIGN, an ordering fact of the graph: the upstream
registry BD-037 depends on this P0 item (through BD-023 and BD-008, which report its state), so the registry,
its `frob doctor` section and the scripted compatibility train of N33b-04 all come after this bead; this bead
therefore opens its own fixture, runs its switch-over by the four written steps below, and hands BD-037 the row's
content. For
FrankenSim the rules include its layer checks, its contracts and its ban on branches and worktrees. Nothing in
this repository patches, forks or vendors a sibling (INV-SUITE-04).

1. **Fix V (OPEN-SUITE-1).** V is a version of asupersync that (i) is published, (ii) corresponds to one clean,
   tagged commit of the asupersync repository, and (iii) every consumer in the table's first seven rows can
   build against. The expected answer is 0.6.2 if its published bytes equal a clean commit; otherwise the next
   version, published from a clean commit under asupersync's own release rules. Procedure: list the published
   versions; compare the registry bytes of 0.6.2 with the tracked bytes that FrankenSim's pinned commit builds
   against (the path checkout is dirty, so "0.6.2 by path" is not yet evidence of "0.6.2 as published"). If they
   agree, V = 0.6.2. If the path carries changes the graph needs, a new version is published from a clean
   asupersync head through that repository's own release gates, and V is that version. The decision and its
   evidence (the two digests compared, or the release commit) go into the admission record of INT-ASUP-01.
   Fail-closed default until V is recorded: whatever FrankenSim's pinned commit builds with (0.6.2 by path
   today), with the tree hash in the receipt and no gate PASS on a dirty tree (OPEN-SUITE-3). If V is not 0.6.2,
   every other exact `=0.6.2` requirement that a graph here activates also blocks resolution (franken_numpy's
   optional one, behind a feature): before V is fixed, `cargo metadata` over every workspace lists the activated
   exact requirements on asupersync, and each one is either moved to V in the same train (a further mirrored
   change in that repository, recorded in the registry row) or its feature stays off here (DESIGN: V must not
   be chosen in a way that silently disables a consumer).
2. **asupersync.** If no clean published commit exists for V: the release is cut there, under its rules, from a
   clean tree. This program does not publish from a tree that holds another agent's uncommitted work.
3. **frankensim.** Its constellation entry for asupersync is moved from 0.5.0 at `7a0f2f55047c` to V at V's
   commit, so that its own verify-only preflight (which treats an off-pin or dirty sibling as a refusal) agrees
   with what its `Cargo.lock` builds. Only the asupersync row is touched; the other stale rows are FD-FSIM-16.
   Its manifests name V as the version beside the path, and its `[patch.crates-io] asupersync` entry keeps
   pointing at the path for development.
4. **frankentorch.** Its requirement becomes exactly V (today `0.6.2`, default features off); its lockfile is
   regenerated; its own suite passes.
5. **fastmcp_rust.** Its exact requirement becomes `=V` (today `=0.6.2`), or a caret requirement on V if that
   repository's rules allow one (an exact pin splits the graph again at the next release; see Considerations);
   lockfile regenerated; suite and MCP conformance checks pass. If FD-FMCP-01 (whose gate INT-FMCP-01 is run by
   BD-023), which edits the same manifests for the toolchain, is in flight, the two changes are landed in one
   reservation.
6. **Here.** The workspace root repeats FrankenSim's `[patch.crates-io] asupersync` entry, so every registry
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
workspace before the change, with its BLAKE3 hash recorded in `expected.toml`: the "before" half of the
evidence that the gate's "after" listing is compared with.

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

- [ ] V is recorded with the commit it corresponds to, and that commit is clean and tagged in the asupersync repository.
- [ ] FrankenSim's constellation entry for asupersync names V at V's commit; FrankenTorch and fastmcp_rust require V; each landed through its own repository's gates and has its mirrored bead.
- [ ] In each of the three consumers, at the landed commit, `cargo tree -d` in the consumer's own workspace shows no duplicate asupersync.
- [ ] Here, `cargo metadata` over every workspace BD-008 measures (at M0 at least the core workspace and `worker/`) shows exactly one `asupersync` package, at V; `cargo tree -d` shows no duplicate; the receipt's `duplicates` is empty.
- [ ] A seeded second asupersync version in a scratch copy fails `cargo xtask check-deps` (the done-condition this item shares with BD-008).
- [ ] The precondition of INT-ASUP-01 holds at the new pins, measured here directly with `cargo metadata` and `cargo tree -d` (one package, at V, in every workspace that exists); or the record names the consumer that still disagrees, and KILL-MCP's fallback is decided at the M0 exit review. The gate's evaluation and its admission record are BD-008's first run, made against the new pins (BD-008 depends on this bead), as the switch-over rule's last sentence provides.
- [ ] The closing record names the fixture path, this bead, the four mirrored beads and the status reached: `landed`, or `adopted` once BD-008's run of INT-ASUP-01 passes at the new pins. BD-037, which comes after this bead, enters exactly that as the item's row in `registries/upstream.toml`, and its registry check then covers this fixture.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In each sibling: its existing suite on the regenerated lockfile; no expectation edited. Added in FrankenSim (additive):
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
must pass, the gates named in this bead that exist are re-run, and the closing record must state `adopted` or
`landed` with a receipt newer than the pin move. `frob doctor` and its `upstream` section come after this bead
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
- `franken-kernel` and `franken-evidence` live in the asupersync repository; FD-FKER-01 touches the same repository and should share a reservation window with this item.
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
