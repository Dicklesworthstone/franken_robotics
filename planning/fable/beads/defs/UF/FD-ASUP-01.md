---
key: "FD-ASUP-01"
title: "upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version pinned by all three consumers"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "upstream", "asupersync", "frankensim", "frankentorch", "fastmcp_rust", "solid"]
parent: "WP-02"
deps: ["BD-001", "BD-006"]
needs: []
related: ["BD-008", "BD-023", "BD-044", "FD-FMCP-01", "FD-FSIM-16", "FD-FTUI-01", "NF-01", "STD-E2E"]
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

In: choosing V with asupersync's maintainers' rules; the change in each of the four repositories; the mirrored
beads; the pins here. Out: wiring INT-ASUP-01 into `xtask check-stack` and recording its evidence (BD-008); the
rest of FrankenSim's stale constellation lock (FD-FSIM-16); frankentui's exact 0.5.0 pin (FD-FTUI-01); bringing
fastmcp_rust onto T (FD-FMCP-01).

## Technical approach

Four repositories are touched. The same rules bind each: its own `AGENTS.md`, gates and conventions govern; every
file is reserved through Agent Mail before it is edited, because other agents work there (asupersync's checkout
had 267 modified paths: nobody else's uncommitted work is touched); the change is additive and lands on that
repository's `main` through its own gates; each repository gets its own mirrored bead, and the four identifiers
are stored in the `mirror` field of this item's row in `registries/upstream.toml` (`frob.upstream/1`). For
FrankenSim the rules include its layer checks, its contracts and its ban on branches and worktrees. Nothing in
this repository patches, forks or vendors a sibling (INV-SUITE-04).

1. **Fix V (OPEN-SUITE-1).** V is a version of asupersync that (i) is published, (ii) corresponds to one clean,
   tagged commit of the asupersync repository, and (iii) every consumer in the table's first seven rows can
   build against. The expected answer is 0.6.2 if its published bytes equal a clean commit; otherwise the next
   version, published from a clean commit under asupersync's own release rules. Fail-closed default until V is
   recorded: whatever FrankenSim's pinned commit builds with (0.6.2 by path today), with the tree hash in the
   receipt and no gate PASS on a dirty tree (OPEN-SUITE-3).
2. **asupersync.** If no clean published commit exists for V: the release is cut there, under its rules, from a
   clean tree. This program does not publish from a tree that holds another agent's uncommitted work.
3. **frankensim.** Its constellation entry for asupersync is moved from 0.5.0 at `7a0f2f55047c` to V at V's
   commit, so that its own verify-only preflight (which treats an off-pin or dirty sibling as a refusal) agrees
   with what its `Cargo.lock` builds. Its `[patch.crates-io] asupersync` entry keeps pointing at the path.
4. **frankentorch.** Its requirement becomes exactly V (today `0.6.2`, default features off); its lockfile is
   regenerated; its own suite passes.
5. **fastmcp_rust.** Its exact requirement becomes `=V` (today `=0.6.2`); lockfile regenerated; suite and MCP
   conformance checks pass. If FD-FMCP-01 is in flight, the two changes are landed in one reservation.
6. **Here.** The workspace root repeats FrankenSim's `[patch.crates-io] asupersync` entry, so every registry
   requirement on V resolves to the one path package at V's commit. `franken-stack.lock` row `FD-ASUP` records
   `version = V`; the stack receipt records the asupersync revision, `dirty: false` and an empty `duplicates`.

**Interim posture here.** One package identity per build stays enforced by `cargo xtask check-deps` (one
asupersync package per binary; asupersync 0.5.0 nowhere; the frankentui executor feature forbidden). A surface
that would bring a second package stays out of the core graph under KILL-MCP's first fallback.

{{FIXTURE:FD-ASUP-01}} It resolves the M0 graph (the core
with the airlock, the Metal annex with `ft-kernel-metal`, and a probe on `fastmcp_rust`) with `cargo metadata`
and asserts exactly one package named `asupersync`, at one version, from one source, in each workspace; and that
each consumer's own lock agrees with V. At the starting state FrankenSim's constellation lock disagrees (0.5.0).

{{SWITCH}}

**The gate the switch-over re-runs.** INT-ASUP-01 (admission, M0; wired by BD-008): `cargo tree -d` shows one
asupersync package in each workspace (the core and `worker/`), the same published version in all of them, and no
binary links two. This is the gate that proves this item. INT-FMCP-01 depends on it.

**Fallback if the change cannot land.** `check-deps` keeps enforcing one identity per build; a surface that still
brings a second package moves to its own workspace (for `frob-mcp`, KILL-MCP fallback 1: its own workspace
talking to `frobd` over a local socket; fallback 2: the CLI alone). Computational crates would then take a
capability-free token from `frob-core` in place of asupersync's `Cx` at the split (OPEN-SUITE-6; default: `Cx`
wherever cancellation is observed, the token only in that case).

## Acceptance criteria

- [ ] V is recorded with the commit it corresponds to, and that commit is clean and tagged in the asupersync repository.
- [ ] FrankenSim's constellation entry for asupersync names V at V's commit; FrankenTorch and fastmcp_rust require V; each landed through its own repository's gates and has its mirrored bead.
- [ ] In each of the three consumers, at the landed commit, `cargo tree -d` in the consumer's own workspace shows no duplicate asupersync.
- [ ] Here, `cargo metadata` over the core workspace and over `worker/` shows exactly one `asupersync` package, at V; `cargo tree -d` shows no duplicate; the receipt's `duplicates` is empty.
- [ ] A seeded second asupersync version in a scratch copy fails `cargo xtask check-deps` (the done-condition this item shares with BD-008).
- [ ] INT-ASUP-01 passes on the whole graph, or the record names the consumer that still disagrees and KILL-MCP's fallback is decided at the M0 exit review.
{{STDACC}}

## Test plan

### Unit tests
In each sibling: its existing suite on the regenerated lockfile; no expectation edited. Added in FrankenSim (additive):
- `constellation_asupersync_matches_cargo_lock`: the asupersync entry of `constellation.lock` equals the version and revision `Cargo.lock` resolves. Input today: 0.5.0 against 0.6.2; expected after the change: equal.
Here (`tests/upstream/FD-ASUP-01/`, real resolution, no mocks):
- `one_asupersync_package_per_workspace`: `cargo metadata` on each workspace; expected package count 1 for the name `asupersync`.
- `all_consumers_require_v`: parses the three consumers' manifests at their pinned commits; expected requirement equal to V (an exact requirement where the sibling uses one).
- `registry_and_path_unify`: with the root patch entry present, a registry requirement on V resolves to the path source; with the patch entry removed in a scratch copy, two packages appear (this is the control that shows the test can see a duplicate).

### End-to-end script and logging
{{E2E:FD-ASUP-01}} Both phases also print, per workspace, the output of `cargo tree -d` and its BLAKE3
hash, the resolved asupersync version and source, and the drift class `check-stack` gives each consumer
(`on-pin`, `stale-lock` or `drift`).

{{LOG:upstream_fd_asup_01:FD-ASUP-01}}

### Known answers and negative controls
- Known answer: for the aligned graph, `cargo tree -d` lists no asupersync line and the receipt's `duplicates` array is empty.
- Negative control 1 (the plant of GATE-WS-01): a scratch crate requiring `asupersync = "=0.5.0"` added to the workspace; `check-deps` must fail and name both versions.
- Negative control 2: enabling frankentui's `asupersync-executor` feature in a scratch manifest must fail `check-deps`.
- Negative control 3: a consumer manifest edited to `=V+1` in a scratch copy must make `all_consumers_require_v` fail.
- Determinism lanes: the receipt computed twice on an unchanged tree is byte-identical (KA-767); the check is repeated on the Linux host and on this Mac.

## Considerations

- RISK-03: two asupersync versions in one build; three siblings still pin 0.5.0. Earliest signal: `cargo tree -d` at INT-ASUP-01. Those three (frankentui's executor, franken_threed, frankenjax) are never Cargo dependencies here.
- A dirty asupersync checkout is acceptable for development builds between receipts, with the tree hash in the receipt, and for nothing else (OPEN-SUITE-3, owner; fail-closed default as stated).
- An exact requirement (`=V`) in a consumer is brittle: the next asupersync release splits the graph again until every consumer moves. Each later move is one compatibility train (NF-01), and the registry row records the consumers that pin exactly.
- `franken-kernel` and `franken-evidence` live in the asupersync repository; FD-FKER-01 touches the same repository and should share a reservation window with this item.
{{F3}}
- For this item specifically: if it cannot land within the M0 budget after being attempted, KILL-MCP's fallbacks apply; M1 depends on asupersync only as one package, which the interim posture already guarantees.

## References

Appendix F.1 (row FD-ASUP-01), F.3; §33.8, §33.13 (consequence 3), §33.16 (INV-SUITE-02, OPEN-SUITE-1, -3, -6), §33.2 (hazard 3), §37.8; gate INT-ASUP-01; KILL-MCP; RISK-03.
