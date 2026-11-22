---
key: "FD-FSQL-01"
title: "upstream frankensqlite: the S3-FIFO eviction state machine made generic over its key type"
type: "task"
priority: 3
milestone: "M2"
labels: ["m2", "suite", "atelier", "upstream", "frankensqlite", "solid"]
parent: "WP-63"
deps: ["BD-037", "BD-111", "NF-01", "STD-E2E"]
needs: []
related: ["BD-220", "BD-009", "BD-209"]
gates: ["INT-FSQL-02"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.15", "§33.4", "§33.6", "§28", "§33.8"]
---
## Background

FrankenRobotics memoizes results by content identity: an analysis or an episode whose inputs have been seen is
served from the memo graph instead of recomputed. The store is bounded, so it must evict, and it must evict by
something better than recency, since a scan over many designs touches each key exactly once and would flush a
recency-ordered cache of everything worth keeping.

FACT (`frankensqlite/crates/fsqlite-pager/src/s3_fifo.rs`, 3,058 lines, read 2026-11-21): FrankenSQLite's pager
implements a three-queue policy: a small admission queue, a main queue with bounded reinsertion, and a ghost
queue of metadata, with an adaptive small share between 5% and 30% and a rollout gate. Sibling files
`arc_cache.rs` and `evalue_eviction.rs` hold two further policies. **The state machine is keyed on
`fsqlite_types::PageNumber`, so it cannot be instantiated over content hashes.** Upstream register priority:
**P3**.

So the policy is used here as a pattern (written again with the memo graph, in `frob-verbs`) and the sibling
module is used as an oracle (in `frob-conformance`). Two implementations of one policy must be kept equal by a
differential test for as long as both exist. Rejected: plain LRU as the design (it is only the fallback);
converting 32-byte content hashes to page numbers in production (a lossy map with collisions; acceptable in a
test under a bijection, not in the store).

## Scope

In: in FrankenSQLite, the state machine parameterized by key type, with `PageNumber` as the default; here, the
local copy replaced by the sibling's. Out: the memo graph, its shadow recompute and its keys (BD-111, BD-220);
byte-size accounting and cost-weighted admission, which are this project's departures and stay here; the
other two pager policies.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankensqlite` repository
(`../frankensqlite` beside this one; head `79b735cb265a` and toolchain channel `nightly-2026-08-31` when this was written
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

**The change (DESIGN).** The S3-FIFO state machine becomes generic over a key type K with the bounds the
implementation needs (copyable or cheaply clonable, equality, hashing), with a default type parameter of
`PageNumber` so that every existing use compiles unchanged and behaves identically. Nothing else in the policy
changes: the three queues, the reinsertion bound, the adaptive small share and its limits, the rollout gate.
The pager's own tests pass unmodified; one test instantiates the machine over a 32-byte key.

**Interim posture here.** The pattern, with a differential oracle. Gate INT-FSQL-02: under a bijection from
keys to page numbers, this project's eviction sequence equals the sibling's on 10 recorded traces of at least
10⁵ accesses, and on the edit-loop trace the hit ratio is not below LRU's. Entries differ in recompute cost by
orders of magnitude, so size accounting is in bytes and the admission rule may refuse a cheap entry; that
departure from the sibling's policy is tested separately, not hidden inside the differential test.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSQL-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSQL-01`, compiled as the `frob-conformance` integration test `upstream_fd_fsql_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It tries to instantiate the sibling's state machine over a 32-byte
content-hash key; at the pin this does not compile, and the fixture records the compiler's diagnostic.

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

**What the switch-over re-runs.** The rule is "consume it; delete the local copy": the memo store's eviction
core becomes the sibling's state machine instantiated over this project's content identity, the local
re-implementation is deleted, and INT-FSQL-02 is re-run: its differential half is now an identity check (one
implementation behind both sides) and is kept as a regression test against a recorded eviction sequence; its
hit-ratio half (not below LRU's on the edit-loop trace) is unchanged. The byte accounting and the
cost-weighted admission remain a wrapper around the consumed machine and keep their separate tests.

**Fallback if the change cannot land.** The local pattern with the differential oracle; if the policy itself
misbehaves, LRU with admission weighted by recompute cost. Never an unbounded store.

## Acceptance criteria

- [ ] In FrankenSQLite: the state machine is generic over its key with `PageNumber` as default; every pager test passes unmodified; an instantiation over a 32-byte key is tested there.
- [ ] In FrankenSQLite: on its own recorded traces the eviction sequence at `PageNumber` is bit-identical to the sequence before the change.
- [ ] Here: the memo store links the sibling's machine at the new pin and `frob-verbs` contains no S3-FIFO state machine of its own (a source scan).
- [ ] Here: the eviction sequences on the 10 recorded traces equal those recorded before the switch; the hit ratio on the edit-loop trace is not below LRU's; the store never exceeds its byte cap.
- [ ] Here: the admission departure (a cheap entry refused) still has its own passing test.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSQLite (added, in `fsqlite-pager`):
- `generic_key_instantiates`: the machine over `[u8; 32]` keys compiles and passes the policy's invariants (queue sizes within bounds; a key is in at most one queue).
- `page_number_sequence_unchanged`: a recorded trace of 10⁵ accesses gives the same eviction sequence as before.
- `scan_does_not_evict_hot_keys`: capacity C = 1,000; a key accessed twice (so promoted to the main queue), then a scan of 10·C distinct keys each accessed once; expected the hot key still resident (new keys leave through the small queue). An LRU of the same capacity evicts it.
Here:
- `memo_eviction_matches_recorded_sequences`: the 10 traces.
- `hit_ratio_not_below_lru_on_edit_loop`: the trace of an agent editing one design repeatedly.
- `byte_cap_is_never_exceeded`: 10⁵ random insertions with sizes spanning four orders of magnitude.
- `cheap_entry_may_be_refused`: an entry whose recompute cost is below the admission threshold is not stored, and the refusal is counted.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsql_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsql_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also drives a real edit loop through `frob design check --json` 200 times
on a perturbed design and reads `frob store status --json`, asserting the hit ratio against LRU's on the same
recorded trace and the byte cap.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsql_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSQL-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: a recorded eviction sequence per trace (a golden file with its hash); the scan-resistance property above.
- Negative controls: a state machine that promotes on first access must lose the hot key in the scan test; an eviction core with the small-queue share fixed outside 5% to 30% must fail the sequence comparison; an input missing from a memo key is caught by the shadow recompute within 64 hits (the negative control of the memo graph's own gate, re-run here).
- Determinism lanes: eviction is a pure function of the access sequence; identical on every platform and worker count for a recorded trace.

## Considerations

- The memo store serves results only inside one determinism class and one hardware fingerprint; eviction never changes what may be served, only what is still present.
- FrankenSQLite is one of the three foundations M1 depends on (with FrankenSim and asupersync), for the ledger; this item concerns its pager policy only and touches nothing the ledger uses.
- P3: the pattern with a differential oracle works; the item removes a duplicate, which matters most when the sibling's policy is tuned and the copy silently is not.
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

Appendix F.1 (row FD-FSQL-01), F.3; §33.15 (FrankenSQLite's pager policies), §33.4 (FD-FSQL), §33.6 (INT-FSQL-01, -02), §28 (the memo graph), §33.8; INT-FSQL-02.
