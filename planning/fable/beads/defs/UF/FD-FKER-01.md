---
key: "FD-FKER-01"
title: "upstream asupersync (franken-kernel): claim and evidence identifier types beside the existing three"
type: "task"
priority: 3
milestone: "M1"
labels: ["m1", "suite", "workspace", "upstream", "asupersync", "frob-core", "solid"]
parent: "WP-36"
deps: ["BD-037", "BD-005", "NF-01", "STD-E2E"]
needs: []
related: ["BD-095", "BD-025", "FD-ASUP-01", "FD-FSIM-09"]
gates: ["INT-ASUP-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§34.12", "§33.15", "§33.4", "§33.8"]
---
## Background

Every gate claim in FrankenRobotics has a `claim_id` that resolves to the `evidence_id`s of the records
supporting it, the `policy_id` of the rule and thresholds in force, and the `trace_id` of the run: the
artifact graph. It is one of six conventions of the wider suite this project adopts, each replacing an ad hoc
habit with something a tool can check. Rows of `registries/gates.toml` carry their `claim_id`.

The suite's identifier vocabulary lives in `franken-kernel`, a small crate inside the asupersync repository.
FACT (`asupersync/franken_kernel`, version 0.6.0, 1,801 lines, read 2026-11-21): it defines `TraceId`,
`DecisionId`, `PolicyId`, `SchemaVersion`, `Budget` and a capability-typed `Cx`. **It has no claim or evidence
identifier type.** It depends on `serde`. Upstream register priority: **P3**.

So `frob-core` mirrors the text form of the three identifiers the kernel has, and adds `ClaimId` and
`EvidenceId` in the same form, locally. `franken-kernel` is not linked into `frob-core`, which admits no
`serde`; its posture is Pattern (the wire shape) and Oracle (a round-trip check in `frob-conformance`).
Rejected: inventing an unrelated identifier format (two products of one suite could not exchange an artifact
graph); linking `franken-kernel` into the core as it is (it would bring `serde` into the one crate that must
stay dependency-free).

## Scope

In: in `franken-kernel`, the two identifier types with the same construction, text form and parsing rules as
its existing three; here, the local types brought under the upstream definition. Out: `frob-core`'s identity
and hashing types (BD-005); the artifact graph and `evidence.trace` (BD-095); one asupersync version across
the graph (FD-ASUP-01), with which this item shares a repository.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `asupersync` repository
(`../asupersync` beside this one; head `b32c5ba66315` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. Its checkout had 267 modified paths when this was written (FACT), so other agents are active there: the tree
is inspected and the affected files reserved before anything is edited, and no uncommitted work of another
agent is touched.

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

**The change (DESIGN).** In `franken-kernel`: `ClaimId` and `EvidenceId`, defined exactly as `TraceId`,
`DecisionId` and `PolicyId` are (the same underlying representation, the same text form with its own prefix or
tag, the same parse errors, the same ordering and hashing), with the same serialization support behind the
same feature arrangement. If the crate's `serde` dependency is unconditional, the change also offers the five
identifier types without it (a feature that is on by default, so existing consumers see no change), because a
consumer that admits no `serde` cannot otherwise use them. Documentation states what a claim identifier names
(a gate claim) and what an evidence identifier names (a record supporting a claim). Additive: the existing
three types and their text forms are untouched.

**Interim posture here.** Local `ClaimId` and `EvidenceId` in `frob-core` with the same text form as the
kernel's three; `frob-conformance` holds a round-trip oracle that formats this project's identifiers and
parses them with `franken-kernel`'s parsers (as a development dependency only) and the reverse.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FKER-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FKER-01`, compiled as the `frob-conformance` integration test `upstream_fd_fker_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It formats a `ClaimId` and an `EvidenceId` in `frob-core`, asks
`franken-kernel` at the pinned commit to parse each as its own type of that name (absent at the pin), and
requires the text to survive the round trip unchanged.

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

**What the switch-over re-runs.** The rule is "replace local types". Two cases, decided by what landed. If the
identifier types are available without `serde`: `frob-core`'s five identifier types become re-exports of the
kernel's, the local definitions are deleted, and `check-deps` confirms that `frob-core`'s dependency set
gained `franken-kernel` only, with no `serde` anywhere in its closure. If they are not: the local types stay,
their text form is re-validated against the upstream types by the round-trip oracle (now covering all five),
and the row is adopted at that, with the reason recorded. In both cases the canonical bytes of every stored
identifier must be unchanged, and the gates registry is reloaded to prove it. INT-ASUP-01 is re-run because
the pin of the asupersync repository moved.

**Fallback if the change cannot land.** Local identifier types with the same text form, as today.

## Acceptance criteria

- [ ] In `franken-kernel`: the two types exist with the same construction, text form rules and parse errors as the existing three; the existing three are unchanged; its tests cover round trips and rejects.
- [ ] Here: every identifier of the five kinds formatted by this project parses in `franken-kernel` at the new pin and re-formats to the same text, and the reverse (10⁴ seeded values per kind).
- [ ] Here: either `frob-core` re-exports the kernel's types with no `serde` in its closure, or it keeps local types and the record says why; in neither case does `frob-core` gain a `serde` dependency.
- [ ] Here: the canonical bytes and digests of `registries/gates.toml`'s `claim_id`s and of 100 stored evidence records are identical before and after.
- [ ] Here: `cargo tree -d` still shows one asupersync package (INT-ASUP-01) at the new pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In `franken-kernel` (added):
- `claim_id_round_trips` and `evidence_id_round_trips`: format then parse is the identity on 10⁴ seeded values.
- `kinds_do_not_cross_parse`: the text of a `ClaimId` is rejected by the parser of `EvidenceId`, of `TraceId`, of `DecisionId` and of `PolicyId`, and the reverse, if the text form carries the kind; if it does not, the test asserts the documented statement that kinds are distinguished by position only.
- `malformed_text_is_rejected`: wrong length, wrong alphabet, wrong prefix; each with the documented error.
Here:
- `five_kinds_round_trip_against_kernel`: the oracle, both directions.
- `core_has_no_serde`: `cargo tree -p frob-core` contains no package named `serde` (the rule that `frob-core` admits none).
- `gate_claim_ids_resolve`: every row of `registries/gates.toml` has a `claim_id` that parses and resolves to at least a policy identifier.
- `stored_identifier_bytes_unchanged`: the digest comparison of the acceptance line.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fker_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fker_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob evidence trace --json` on a recorded gate claim and asserts
that its `claim_id`, `evidence_id`s, `policy_id` and `trace_id` all parse with the kernel's parsers.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fker_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FKER-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer: for each kind, a fixed value and its text, stored as a golden pair shared by both sides of the oracle.
- Negative controls: a local formatter that differs from the kernel's by letter case or padding must fail the round trip; a build of `frob-core` with a `serde` dependency added must fail `check-deps`.
- Determinism lanes: identifiers are exact; text and bytes identical on every platform (class D0).

## Considerations

- The register's rule "replace local types" meets a constraint stated elsewhere in the plan: `franken-kernel` depends on `serde`, and `frob-core` admits none. The two cases above are how this bead reconciles them; the first case needs the upstream change to make `serde` optional, which is therefore part of the proposed change.
- asupersync's checkout had 267 modified paths; `franken_kernel` is a small directory inside it, and the reservation should name only its files. FD-ASUP-01 works in the same repository at M0.
- P3: the local types work. The value is one identifier vocabulary across the suite's products.
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

Appendix F.1 (row FD-FKER-01), F.3; §34.12 (artifact-graph identifiers), §33.15 (franken-kernel), §33.4 (FD-FKER), §33.8; INT-ASUP-01.
