---
key: "FD-FMCP-01"
title: "upstream fastmcp_rust: brought onto the one toolchain T, its test suite and MCP conformance checks passing"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "tether", "upstream", "fastmcp_rust", "solid"]
parent: "WP-06"
deps: ["BD-001", "BD-006", "BD-025", "STD-E2E"]
needs: []
related: ["BD-037", "N33b-08", "N33c-08", "N33c-26", "N34-16", "N44-04", "NF-02", "NF-03", "NF-04"]
gates: ["INT-FMCP-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.8", "§33.13", "§33.15", "§32.12.2", "§46"]
---
## Background

`frob-mcp` is the MCP server through which agents reach every verb of FrankenRobotics. It is built on
`fastmcp_rust`, one of the owner's sibling projects. Upstream register priority: **P0, done at M0** (one of two).

FACT about `fastmcp_rust` (read 2026-11-21, to re-read): workspace version 0.13.0; facade package `fastmcp-rust`
over `fastmcp-core`, `-protocol`, `-transport`, `-server`, `-client`, `-macros`, `-console` and `-cli`, about 1.23
million lines; `ServerBuilder` offers `new(name, version)`, `tool`, `resource`, `resource_template`, `prompt`,
`task_supervisor`, `final_tasks` and `build` (`crates/fastmcp-server/src/builder.rs`); `tasks/get`, `tasks/update`
and `tasks/cancel` are served by default; its source tree pins asupersync `=0.6.2`, while its registry package
0.13.0 carries asupersync 0.5.0; it declares `rust-version = "1.100"` and pins `nightly-2026-08-25`. One measured
run (2026-10-12, revision `1c2dc896`) passed 112 of 115 named server-mode checks with no failure, which its README
says is not a conformance claim; support for the MCP 2026-07-28 revision is "under implementation and remains
unverified".

The mismatch. The build graph of this project has **one Rust toolchain, T**, recorded in `franken-stack.lock` and
`rust-toolchain.toml`. FrankenSim pins `nightly-2026-07-06`, which is rustc 1.99.0-nightly, and `fastmcp_rust`
declares a minimum of 1.100. Cargo refuses such a build unless told to ignore the declared minimum, and this
project does not build on an ignored constraint.

Decision (owner, 2026-11-22; it settles OPEN-SUITE-2 and OPEN-TETHER-1): every sibling is the owner's, so the
mismatch is fixed in `fastmcp_rust`, by this program. `frob-mcp` is then an ordinary member of the core workspace:
it links `frob-verbs`, runs verbs in process and shares the one toolchain and the one asupersync package with the
rest of the graph. Rejected alternatives: running the MCP server across a process boundary by default (the former
default; it is now only KILL-MCP's first fallback, because it adds a protocol hop and a second build to every
agent call); `--ignore-rust-version` (a build that passes by ignoring a declared constraint proves nothing about
the sibling); a separate toolchain file for the surface (withdrawn: no annex carries its own toolchain file).

T is chosen in this order of preference: (a) the suite's common nightly, `nightly-2026-08-31`, which most siblings
already pin; (b) FrankenSim's present pin, `nightly-2026-07-06`, if FrankenSim cannot yet build on (a).

## Scope

In: the change in `fastmcp_rust` that makes it build and pass its own tests and MCP conformance checks on T; the
mirrored bead there; the pin of the landed commit here. Out: `surface/frob-mcp` itself and the gate INT-FMCP-01
(BD-023); the choice and recording of T and the toolchain matrix (BD-031); one asupersync version across the graph
(FD-ASUP-01, BD-008); the legs of the same toolchain train in FrankenSim and frankensympy (NF-03, NF-04) and the
train's run (NF-05); MCP protocol-revision support, which is `fastmcp_rust`'s own roadmap.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `fastmcp_rust` repository
(`../fastmcp_rust` beside this one; head `117388bcc169` and toolchain channel `nightly-2026-08-25` when this was written on 2026-11-21, both FACT to
re-read), by an agent of this program working there under that repository's own `AGENTS.md`, gates and
conventions. It declares `rust-version = "1.100"` and workspace version 0.13.0 (FACT).

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
   `status` moves `open` → `landed` (merged on the sibling's `main`) → `adopted` (switch-over run here). This item
   is executed before the register exists: BD-037, which writes the schema, the loader and the other 52 rows,
   depends on BD-023 and through it on this bead. So this bead writes its own `[[item]]` row in BD-037's format
   (`id`, `repo = ["fastmcp_rust"]`, `change`, `priority = "P0"`, `interim_posture`, `switch_over`, `fixture`,
   `admission_test = "INT-FMCP-01"`, `bead = "FD-FMCP-01"`, `mirror`, `status`, `observed_at`), creating the file
   with that one row if it is absent, and BD-037's loader validates the row when it lands (DESIGN).

**The change, step by step.**

1. Fix the provisional T. BD-001 names candidate (a). BD-006 records whether FrankenSim's crates build on (a) from
   this repository; if they do not, T is candidate (b) from here on. Record `rustc -V` for T.
2. Case (a), T = `nightly-2026-08-31`. In `fastmcp_rust`: set the channel in `rust-toolchain.toml` to T. Keep
   `rust-version = "1.100"` if `rustc -V` on T reports 1.100 or later (HYPOTHESIS, settled by the build: the
   present pin is six days older and builds it). Repair whatever the newer nightly breaks: lints promoted to
   errors, renamed unstable features, changed inference. Each repair is listed in the commit message with its
   reason.
3. Case (b), T = `nightly-2026-07-06` (1.99.0-nightly). In `fastmcp_rust`: set the channel to T and lower the
   declared `rust-version` to the version T reports, and only after the workspace builds and passes there. For
   each use of a language or library item that 1.100 has and T lacks, substitute an equivalent that both
   compilers accept; the public API and the wire behaviour do not change. If a third-party dependency of
   `fastmcp_rust` itself requires 1.100, record it: that is a failing result for case (b) and feeds KILL-MCP.
4. In both cases, in a clean clone on T and without `--ignore-rust-version`: `cargo build --workspace
   --all-targets --locked`, then the repository's whole test suite, then its MCP conformance checks (the named
   server-mode checks of its README). Baseline to beat or equal: 112 of 115 passing, 0 failing. Any check whose
   state changes is explained in the record.
5. Confirm that the source tree still requires asupersync `=0.6.2` and that nothing in this change touches that
   requirement (aligning it is FD-ASUP-01).
6. Land on `main` under the repository's rules; record the commit in the mirrored bead and in
   `registries/upstream.toml`.
7. Here: `surface/frob-mcp` depends on `fastmcp_rust` by commit, at or after the landing commit and never on
   registry 0.13.0 (which carries asupersync 0.5.0); the `FD-FMCP` row of `franken-stack.lock` records the commit
   and a `toolchain` field equal to T.

**Interim posture here.** Agents use the CLI (`frob --json`), which is complete on its own. `frob-mcp` is not
added to the core graph while `fastmcp_rust` does not build on T; no script passes `--ignore-rust-version`.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FMCP-01/`, opened and kept current by this
bead (for the same reason as the row: BD-037 comes after it, and adopts the fixture as it finds it): one test tagged `FD-FMCP-01`, compiled as the `frob-conformance` integration test `upstream_fd_fmcp_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asserts two things: a probe crate in this workspace that depends on
`fastmcp_rust` at the pinned commit builds on T with the declared minimum honoured; and the channel in the
sibling's `rust-toolchain.toml` at the pinned commit equals T. At the starting pin (`117388bcc169`) the second
fails under either candidate, and the first fails under candidate (b) with Cargo's "requires rustc 1.100" error.

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

**The gate the switch-over re-runs.** INT-FMCP-01 (admission, M0; run by BD-023): `frob-mcp` builds in the core
graph on T with exactly one asupersync package (`cargo tree -d` shows no duplicate), lists the tools generated
from the verb registry, and passes the MCP protocol smoke test over stdio and HTTP. Under candidate (a) the same
toolchain train also advances FrankenSim's pin and moves frankensympy (NF-03, NF-04, NF-05).

**Fallback if the change cannot land (KILL-MCP, a kill gate decided between M0 and M1 and re-read at M1).**
Trigger: INT-FMCP-01 cannot pass within the M0 budget after FD-FMCP-01 and FD-ASUP-01 have both been attempted.
Fallback 1: `frob-mcp` as its own workspace, linking `frob-verbs` with the `registry-only` feature and talking to
`frobd` over a local socket in newline-delimited JSON. Fallback 2: agents use the CLI alone. The decision is
recorded as an ADR at the M0 exit review (BD-044); this item then stays open and is retried at the next train.

## Acceptance criteria

- [ ] In a clean clone of `fastmcp_rust` at the landed commit, on T and without `--ignore-rust-version`: `cargo build --workspace --all-targets --locked` succeeds and the whole test suite passes.
- [ ] Its MCP conformance checks run on T with zero failures and at least the baseline count passing (112 of 115 at revision `1c2dc896`); the counts are recorded.
- [ ] Its `rust-toolchain.toml` names T, and its declared `rust-version` is no higher than the version `rustc -V` reports for T.
- [ ] Its public API and wire behaviour are unchanged: its own API and snapshot tests pass without edits to their expectations.
- [ ] Here, the probe crate of the fixture builds on T and `cargo tree -d` on the probe shows no duplicate asupersync package (after FD-ASUP-01) or names exactly the duplicate FD-ASUP-01 removes (before it).
- [ ] The seed done-condition that this item feeds is met by BD-023: INT-FMCP-01 passes with `surface/frob-mcp` in the core graph on T. If it cannot within the M0 budget, the KILL-MCP record names which step failed.
- [ ] The state of this item (landed commit, counts, or the failing step) is handed to the M0 exit review.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In `fastmcp_rust`: the existing suite is the test; no expectation is edited to make it pass. Added there (additive):
- `declared_msrv_is_not_above_pinned_toolchain`: parses `rust-toolchain.toml` and the workspace `rust-version`; asserts that the pinned compiler's version is at least the declared minimum. Input today: 1.100 against the pinned nightly; expected: pass on T.
Here (`tests/upstream/FD-FMCP-01/`, real builds, no mocks):
- `probe_builds_on_t_honouring_rust_version`: `cargo build --locked` of the probe; expected exit 0 after adoption, and the "requires rustc 1.100" diagnostic at the starting pin under candidate (b).
- `sibling_channel_equals_t`: reads the channel at the pinned commit; expected equal to the `toolchain` of `franken-stack.lock`.
- `registry_package_is_refused`: a scratch manifest that names registry `fastmcp-rust = "0.13.0"`; expected `cargo xtask check-deps` failure naming asupersync 0.5.0.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fmcp_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --section upstream --json`
(the `upstream` section shows this item with its status; logged `not-run` while the `frob` binary of BD-022 does
not exist yet, since this item does not wait for it); `cargo test -p frob-conformance --test upstream_fd_fmcp_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead that exist are re-run, and the registry row must read `adopted` with a receipt newer than the pin move, or
`landed` with that receipt while INT-FMCP-01 has not had its first run (it is BD-023's, which comes after this
bead, so on the first `post` run the step for it is logged `not-run` with the key BD-023 and is never counted as
a pass). In addition, both phases clone the pinned commit into `target/upstream-scratch/fastmcp_rust@<rev>` (never
building inside the sibling's checkout), run its build, suite and conformance checks on T, and log the three
results with the pass and fail counts against the baseline 112/115/0.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fmcp_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FMCP-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer (starting state, FACT to recheck): a 1.99.0-nightly compiler against `rust-version = "1.100"` is refused by Cargo; four nightlies are in play across the suite (`2026-07-06`, `-08-20`, `-08-25`, `-08-31`).
- Negative control 1: in the scratch clone, raising `rust-version` to `"1.200"` must make the build fail on T; if it does not, the minimum is being ignored somewhere and the script fails.
- Negative control 2: a grep over `scripts/` and `xtask/` for `--ignore-rust-version` must find nothing.
- Negative control 3: the registry-package manifest above must be refused by `check-deps`.
- Determinism lanes: the probe's `Cargo.lock` hash is identical on two runs on one machine; the build is repeated on the Linux host and on this Mac.

## Considerations

- RISK-02: the toolchain train costs more than M0 allows, or breaks a sibling. Earliest signal: a failing cell of the INT-TOOLCHAIN-01 matrix, or this item still open at the M0 exit review. Candidate (b) is the reserve.
- A consumer inside the suite (`mcp_agent_mail_rust`) registers 45 tools and 32 resources on `fastmcp_rust`'s registry package on `nightly-2026-08-31`. It is not changed by this item, and the additive rule means it must keep building; under candidate (b) check it explicitly before landing.
- Partial wire cancellation, unpromoted OAuth surfaces and the unverified 2026-07-28 revision are stated limits of the sibling, not part of this item; `frob-mcp` is admitted through INT-FMCP-01 on what works today.
- If T later moves, `fastmcp_rust` moves with it in the same toolchain train (NF-02); this bead is the first instance of that procedure.
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
- For this item specifically: if neither P0 item can land within the M0 budget, KILL-MCP's fallbacks apply and M1's verbs are reached through the CLI; M1 itself does not wait.

## References

Appendix F.1 (row FD-FMCP-01), F.3; §33.8, §33.13, §33.15, §32.12.2, §43.3, §46 step 3; gates INT-FMCP-01, INT-TOOLCHAIN-01; KILL-MCP; RISK-02; OPEN-SUITE-2, OPEN-TETHER-1 (decided); BET-04.
