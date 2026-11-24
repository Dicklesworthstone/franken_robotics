---
key: "FD-FSIM-09"
title: "upstream frankensim: a home for device receipts (evidence fields AE-01 to AE-12) in its evidence types"
type: "task"
priority: 3
milestone: "M2"
labels: ["m2", "suite", "spark", "upstream", "frankensim", "solid"]
parent: "WP-46"
deps: ["BD-014", "BD-017", "BD-037", "N21b-32", "N21c-28", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-31"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§21.2", "§21.22", "§33.3", "§33.8"]
---
## Background

FrankenRobotics runs physics on GPUs; FrankenSim does not. FACT (`frankensim/docs/ACCELERATOR_DOCTRINE.md`):
FrankenSim's accelerator doctrine states "FrankenSim does not ship an accelerator backend", requires a
"Permanent CPU reference", forbids in-process foreign kernels in its production paths, and defines **twelve
evidence fields, AE-01 to AE-12, for any device run**: device identity; driver and runtime; compiler and
backend; kernel source hash; kernel and workload identity; CPU axes; reduction and determinism policy;
CPU-versus-device comparison; the retained CPU reference; cancellation outcome; phase attribution; and a go or
no-go decision. (The numbering follows the doctrine's order as listed here; it is re-read at the pin.)

This project follows that doctrine from outside FrankenSim's tree: the GPU annexes live here, every kernel has
a CPU twin, and every device result carries the twelve fields (INV-SPARK-02: every device result carries AE-01
to AE-12, a placement receipt and a fingerprint digest, checked by schema at the agent boundary). The fields
have a definition in a document and no type in FrankenSim's evidence crates, so this project defines local
types with the doctrine's field names. Upstream register priority: **P3**.

Why it is worth an upstream type: a device receipt composed with FrankenSim's `Evidence<T>` should obey the same
conservative composition as every other evidence value, and two products of the suite that follow one doctrine
should not carry two incompatible receipts for it. Rejected: placing GPU code in FrankenSim to justify the type
(the doctrine forbids it, which is why the annexes stay here); leaving the fields as free-form strings.

## Scope

In: in FrankenSim, an additive receipt type for the twelve fields in its evidence crates; here, replacement of
the local type. Out: the device receipts' production (the CUDA and Metal backends, BD-172 and BD-174); the
placement receipt and the fingerprint (BD-178, BD-029), which stay this project's formats; any device code in
FrankenSim.

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

**The change (DESIGN).** In `fs-evidence` (or a small sibling crate its layering prefers): a plain data type
with one field per AE item, in the doctrine's order and with the doctrine's names:

| Field | Content |
|---|---|
| AE-01 | Device identity |
| AE-02 | Driver and runtime |
| AE-03 | Compiler and backend |
| AE-04 | Kernel source hash |
| AE-05 | Kernel and workload identity |
| AE-06 | CPU axes |
| AE-07 | Reduction and determinism policy |
| AE-08 | CPU-versus-device comparison |
| AE-09 | The retained CPU reference |
| AE-10 | Cancellation outcome |
| AE-11 | Phase attribution |
| AE-12 | Go or no-go decision |

It has a validating constructor (no field may be absent; AE-12 is an enumeration, not text), and a composition
rule with `Evidence<T>` that follows the crate's weakest-input rule: a value computed from a device result
carries the receipt, and a no-go receipt cannot be composed into a value that claims more than its CPU
reference supports. It contains no device code and links no driver: a type is not a backend.

**Interim posture here.** Local types with the doctrine's field names, written by this project's canonical
writer and checked by schema at the agent boundary.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-09/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-09`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_09` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It looks for the receipt type among FrankenSim's evidence types through
the bedrock-direct crate `fs-evidence` (absent at the pin) and, when present, checks that a local receipt
converts to it and back without loss.

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

**What the switch-over re-runs.** The rule is "replace local types". The local type becomes an alias of, or a
thin wrapper over, the sibling's. Then: the schema check of INV-SPARK-02 on every device result; a
byte-identity test on stored receipts (canonical bytes come only from this project's writer, so replacing the
in-memory type must not move any digest); and the CPU-versus-device check that fills AE-08.

**Fallback if the change cannot land.** The local types stay; nothing else depends on the upstream type.

## Acceptance criteria

- [ ] In FrankenSim: the type exists with twelve fields named and ordered as the doctrine lists them, a validating constructor and the composition rule; no device or driver dependency is added to any FrankenSim crate.
- [ ] Here: at the new pin the local receipt type is the sibling's (alias or wrapper); no second definition of the twelve fields remains in this repository.
- [ ] Here: the canonical bytes and digests of 100 stored device receipts are identical before and after the replacement.
- [ ] Here: a device result missing any AE field is rejected at the agent boundary, as before.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `receipt_requires_all_twelve_fields`: constructing with any one field absent fails, twelve cases.
- `no_go_receipt_caps_the_claim`: composing a no-go receipt into an evidence value yields a value whose class is no stronger than its CPU reference's.
- `field_order_matches_doctrine`: the type's field list equals the list parsed from the doctrine document.
Here:
- `local_receipt_round_trips_through_sibling_type`: 10³ seeded receipts; conversion there and back is the identity.
- `stored_receipt_digests_unchanged`: 100 receipts captured before adoption; expected identical BLAKE3 digests after.
- `agent_boundary_rejects_incomplete_receipt`: a device result with AE-09 removed; expected schema refusal naming the field.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_09.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_09 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs,
where a device is present, `frob spark probe --device <id> --suite canary --quick --key fsim09-probe-<id>-<run-id>
--json` (BD-017) and `frob spark check --kernel K-selftest --backend <b> --key fsim09-check-<b>-<run-id> --json`
(N21b-32; the check fills AE-08), and in every case `frob spark check --kernel K-selftest --backend batch-cpu
--key fsim09-selftest-<run-id> --json`, whose `batch-cpu` receipt also carries the twelve fields, so the phase
asserts something on a host with no device. These are `run` verbs: each call carries its own idempotency key
(`FROB-REF-805` without one) and its envelope must show `idempotent_replay: false`. The script asserts that every
envelope's receipt carries all twelve fields and passes the schema check of INV-SPARK-02 (N21c-28), and that each
receipt, re-read from the record through the replaced type, has the canonical bytes and the digest that were
stored when it was written (the type replacement must not move a digest).

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_09/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-09`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answer: the twelve field names and their order, as the doctrine lists them.
- Negative controls: a receipt with AE-12 as free text must be rejected by the constructor; a wrapper that reorders fields before serialization must fail the digest test; a result labelled native on a device without a completed-command-buffer receipt must be refused (the device tier's own rule, re-run here).
- Determinism lanes: canonical bytes identical on every platform (class D0).

## Considerations

- `fs-evidence` depends on `fs-qty` directly and is on the bedrock-direct list; a type added there is compiled by every crate of this project, so it must stay small and dependency-free. If FD-FSIM-31 moves `fs-qty` out of `fs-evidence`, the two changes touch the same crate and should share a reservation.
- The doctrine also names a falsifier for device pilots; that rule lives in the Spark chapter's beads and is not part of this type.
- P3: the value is consistency across the suite, not capability. It should be done when someone is already working in `fs-evidence`.
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

Appendix F.1 (row FD-FSIM-09), F.3; §21.2 (the doctrine and AE-01 to AE-12), §21.22 (INV-SPARK-02), §33.3 (`fs-evidence`), §33.8.
