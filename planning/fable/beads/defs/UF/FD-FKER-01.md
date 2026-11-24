---
key: "FD-FKER-01"
title: "upstream asupersync (franken-kernel): claim and evidence identifier types beside the existing three"
type: "task"
priority: 3
milestone: "M1"
labels: ["m1", "suite", "workspace", "upstream", "asupersync", "frob-core", "solid"]
parent: "WP-36"
deps: ["BD-005", "BD-037", "BD-095", "N29b-09", "N33b-04", "N34-11", "STD-E2E"]
needs: []
related: ["FD-FSIM-09", "N33a-20", "N33b-08", "N33c-09", "N33c-27"]
gates: ["INT-ASUP-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§34.12", "§33.15", "§33.4", "§33.8"]
---
## Background

Every gate claim in FrankenRobotics has a `claim_id` that resolves to the `evidence_id`s of the records
supporting it, the `policy_id` of the rule and thresholds in force, and the `trace_id` of the run: the
artifact graph. It is one of six conventions of the wider suite this project adopts, each replacing an ad hoc
habit with something a tool can check. Rows of `registries/gates.toml` carry their `claim_id`.

The suite's identifier vocabulary lives in `franken-kernel`, a small crate inside the asupersync repository.
FACT (`asupersync/franken_kernel`, version 0.6.0, 1,801 lines, read 2026-11-21; re-read 2026-11-23 at head
`152237484414`: version 0.7.0, prepared 2026-11-21, still 1,801 lines and the same types): it defines `TraceId`,
`DecisionId`, `PolicyId`, `SchemaVersion`, `Budget` and a capability-typed `Cx`. **It has no claim or evidence
identifier type.** It depends on `serde`. Upstream register priority: **P3**.

So `frob-core` mirrors the text form of the three identifiers the kernel has, and adds `ClaimId` and
`EvidenceId` in the same form, locally (N34-11 defines them in `frob_core::graph_id`). The kernel's `PolicyId`
names a decision rule, while this project's `PolicyId` names a control policy by content hash, so the
decision-rule identifier is called `DecisionPolicyId` here whichever crate supplies it. `franken-kernel` is not
linked into `frob-core`, which admits no `serde`; its posture is Pattern (the wire shape) and Oracle (a
round-trip check in `frob-conformance`, N33c-27, where it is a dev-dependency only).

The text forms this project already writes into gate rows, bead logs and envelopes (N34-11, DESIGN, fixed at
M0): `ClaimId` and `EvidenceId` are 32-byte BLAKE3 digests printed `clm:<64 lower-case hex digits>` and
`ev:<64 lower-case hex digits>`; `DecisionPolicyId` is a dotted lower-case name and a `u32` version printed
`name@vN` (for example `atelier.dc01.promote@v3`), and in JSON it is that string, never the kernel's `serde`
object `{"n": …, "v": …}`; `TraceId` and `DecisionId` are 128-bit values printed as 32 lower-case hex digits
(the kernel's form). FACT (read 2026-11-22, N29b-09): the kernel has text parsers for `TraceId` and
`DecisionId` and none for `PolicyId`.
Rejected: inventing an unrelated identifier format (two products of one suite could not exchange an artifact
graph); linking `franken-kernel` into the core as it is (it would bring `serde` into the one crate that must
stay dependency-free).

## Scope

In: in `franken-kernel`, the two identifier types with the same construction, text form and parsing rules as
its existing three, and a parser for `PolicyId`'s text form; here, the local types brought under the upstream
definition. Out: `frob-core`'s identity and hashing types (BD-005); the local identifier types, their
pre-images and parsers (N34-11); the round-trip oracle and franken-kernel's dev-only admission (N33c-27); the
decision OPEN-CHRON-3 between import and local types (N29b-09); the artifact graph and `evidence.trace`
(BD-095); one asupersync version across the graph (FD-ASUP-01), with which this item shares a repository.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `asupersync` repository
(`../asupersync` beside this one; head `b32c5ba66315` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read; on 2026-11-23 the head was `152237484414`, same channel), by an agent of this
program working there under that repository's own `AGENTS.md`, gates and conventions. Its checkout had 267
modified paths when this was written and none on 2026-11-23 (FACT; the count moves as other agents work there): the tree
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

**The change (DESIGN).** In `franken-kernel`: `ClaimId` and `EvidenceId` in the style of `TraceId`,
`DecisionId` and `PolicyId` (the same derive set, the same error type and parse-error kinds, the same ordering
and hashing, the same serialization support behind the same feature arrangement), with the representation and
text form this project already writes: a 32-byte digest, printed and parsed as `clm:` or `ev:` followed by
exactly 64 lower-case hexadecimal digits; upper-case hex, a missing or wrong prefix and a wrong length are
parse errors, and the parser of one kind rejects the text of the other. The types hold a digest and are
constructed from 32 bytes; the pre-images that produce the digest (`frob.claim/1`, `frob.evidence/1`) stay this
project's own and are not part of the change. Also additive: a text parser for `PolicyId` in its `Display` form
`name@vN` (dotted lower-case ASCII segments, `@v`, a decimal `u32`), which the kernel lacks, so that all five
kinds can round-trip through the kernel in both directions. If the crate's `serde` dependency is
unconditional, the change also offers the five identifier types without it (a feature that is on by default,
so existing consumers see no change), because a consumer that admits no `serde` cannot otherwise use them.
Documentation states what a claim identifier names (a gate claim) and what an evidence identifier names (a
record supporting a claim). Additive: the existing three types and their text and `serde` forms are untouched.

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

**What the switch-over re-runs.** The rule is "replace local types". Whether `frob-core` may import any
identifier type from `franken-kernel` at all is the open question OPEN-CHRON-3 (owner: suite lead; fail-closed
default: defined locally in `frob-core`), decided by N29b-09 against four fixed criteria (the import adds
exactly one package and no runtime to `frob-core`'s closure; the one-asupersync rule holds; the
standard-library-only auditor does not need the types; the sibling's wire forms are the ones above and are
covered by the sibling's own tests). Two cases:
- *Case 1, only if N29b-09 chose the import (its option A) and the identifier types are available without
  `serde`*: `frob-core`'s five identifier types become re-exports of the kernel's (the kernel's `PolicyId`
  re-exported as `DecisionPolicyId`, its own name hidden, and the canonical writer still printing `name@vN`),
  the local definitions are deleted, and the uses of the FD-FKER row of `franken-stack.lock` are changed in the
  same commit to admit exactly that one normal edge with default features off, by an ADR under `docs/adr/` that
  cites N29b-09's record. Today that row admits `franken-kernel` only as a dev-dependency of `frob-conformance`
  (Pattern; Oracle), so `check-deps` fails any edge from `frob-core` to it under BD-003's rule
  `L-4.foundation-confined`, read through N33a-01's oracle rule; there is no separate `suite.dev-only` rule
  (N33c-09 keeps only the plant). `check-deps` then confirms that `frob-core`'s dependency set gained
  `franken-kernel` only, with no `serde` anywhere in its closure.
- *Case 2, otherwise (the default)*: the local types stay, their text form is re-validated against the
  upstream types by the round-trip oracle of N33c-27 (now covering all five kinds, both directions), and the
  row is adopted at that, with the reason recorded (OPEN-CHRON-3 decided local, or `serde` not optional
  upstream).
In both cases the canonical bytes of every stored identifier must be unchanged, and the gates registry is
reloaded to prove it. INT-ASUP-01 is re-run because the pin of the asupersync repository moved.

**Fallback if the change cannot land.** Local identifier types with the same text form, as today.

## Acceptance criteria

- [ ] In `franken-kernel`: the two types exist as 32-byte digests with the text forms `clm:<64 hex>` and `ev:<64 hex>` and the same parse-error kinds as the existing three; `PolicyId` parses its `name@vN` form; the existing three and their `serde` forms are unchanged; its tests cover round trips and rejects.
- [ ] Here: the switch-over case taken (1 or 2) matches N29b-09's recorded decision on OPEN-CHRON-3; under case 2 `cargo tree -e normal -p frob-core` contains neither `franken-kernel` nor `serde`.
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
- `claim_id_round_trips` and `evidence_id_round_trips`: format then parse is the identity on 10⁴ seeded 32-byte values; the text of the all-zero digest is `clm:` (or `ev:`) followed by 64 zeros.
- `policy_id_parses_display_form`: `scheduler.preempt@v3` parses to name `scheduler.preempt`, version 3, and prints back identically; versions 0 and 2³² − 1 round-trip; `Scheduler.preempt@v3`, `a..b@v1`, `a@v` and `a@v4294967296` are rejected.
- `kinds_do_not_cross_parse`: the text of a `ClaimId` is rejected by the parsers of `EvidenceId`, `TraceId`, `DecisionId` and `PolicyId`, and the text of an `EvidenceId` by the other four; the stated exception, asserted as such: `TraceId` and `DecisionId` share one text form and are told apart by the field that holds them.
- `malformed_text_is_rejected`: wrong length (63 and 65 digits), upper-case hex, a missing prefix, the other kind's prefix; each with the documented error.
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
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob evidence trace <clm:…> --json` (BD-095; the claim identifier is the positional
input, in its one text form) on the `claim_id` of a recorded gate row and asserts that the claim identifier, its
`evidence_id`s, `policy_id` and `trace_id` all parse with the kernel's parsers.

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
- Negative controls: a local formatter that differs from the kernel's by letter case or padding must fail the round trip; a build of `frob-core` with a `serde` dependency added must fail `check-deps`; a normal edge from `frob-core` to `franken-kernel` must fail `check-deps` with `L-4.foundation-confined` on the FD-FKER row (and with no other finding) unless the case-1 ADR has changed that row's uses; a decision policy stored as the object `{"n": …, "v": …}` must fail `stored_identifier_bytes_unchanged`.
- Determinism lanes: identifiers are exact; text and bytes identical on every platform (class D0).

## Considerations

- The register's rule "replace local types" meets two constraints stated elsewhere in the plan: `franken-kernel` depends on `serde`, and `frob-core` admits none; and OPEN-CHRON-3 defaults to local definitions. The two cases above are how this bead reconciles them; the first case needs the upstream change to make `serde` optional, which is therefore part of the proposed change, and N29b-09's decision for the import. Under the default the upstream types still pay off: the oracle then checks this project's forms against one suite-wide definition of all five kinds.
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

Appendix F.1 (row FD-FKER-01), F.3; §34.12 (artifact-graph identifiers), §29.13 and §29.21 (OPEN-CHRON-3), §33.15 (franken-kernel), §33.4 (FD-FKER), §33.8; INT-ASUP-01.
