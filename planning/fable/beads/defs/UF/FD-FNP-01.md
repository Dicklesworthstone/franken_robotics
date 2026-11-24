---
key: "FD-FNP-01"
title: "upstream franken_numpy, frankentorch: a named, checked conversion between byte strides and element strides"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "suite", "spark", "upstream", "franken_numpy", "frankentorch", "solid"]
parent: "WP-37"
deps: ["FDEF-03", "N33b-04", "STD-E2E"]
needs: []
related: ["N21a-05", "N33b-08"]
gates: ["INT-FNP-02", "INT-FNP-01"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 3", "Appendix F.3", "§33.11", "§21.5", "§33.4", "§33.8"]
---
## Background

Two of the suite's array libraries describe a strided view differently. franken_numpy's `fnp-ndarray` uses
signed byte strides (`fnp-ndarray/src/lib.rs:258, :390` at `e8cb85f4e5ac`); FrankenTorch's `ft-core` uses
unsigned element strides (`ft-core/src/lib.rs:397, :2460` at `8165d6a18028`). Neither is wrong alone. At a
boundary between them, a stride tuple copied verbatim reads the wrong element, in bounds, and nothing faults:
a contiguous 2×2 single-precision view has strides (8, 4) in bytes and (2, 1) in elements, and (8, 4) used as
element strides over a 16-element buffer addresses element 12 at index [1, 1], where element 3 belongs. The
report is FDEF-03; the admission test is INT-FNP-02. Upstream register priority: **P2**.

FrankenRobotics meets the boundary at export and interchange (array files through `fnp-io`, policy tensors
through FrankenTorch, the Gymnasium-compatible adapter) and closes it with a typed converter of its own. The
upstream item asks each library to offer the conversion by name, so that the next consumer of both does not
write the obvious wrong one. Rejected: asking either library to change its convention (both are legitimate, and
existing users depend on each).

## Scope

In: a named, checked conversion in each of the two repositories; here, replacement of the local converter and
the re-run of INT-FNP-02. Out: the local converter and the admission tests (BD-036); device-resident tensor
views (BD-193); franken_numpy's lack of single-precision storage, which is its own roadmap.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `franken_numpy` repository
(`../franken_numpy` beside this one; head `e8cb85f4e5ac` and toolchain channel `nightly-2026-08-31` when this was written
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

The same rules bind the half of the change made in the `frankentorch` repository (`../frankentorch`; head
`8165d6a18028` on 2026-11-21): its own `AGENTS.md` and gates govern, files are reserved through Agent Mail, the
change is additive and lands on its `main`, and it has its own mirrored bead; both identifiers are stored in
this item's registry row.

**The change (DESIGN).** In each library, two functions and one error type.

*Bytes to elements.* Input: item size s; byte origin o_b; signed byte strides b₁ … b_d; shape n₁ … n_d; the
allocation's extent in bytes. Output: element origin o_e = o_b/s and unsigned element strides e_k = b_k/s. It
fails, with the reason and the axis, when: s does not divide o_b (`MisalignedOrigin`); s does not divide some
b_k (`NotMultipleOfItemSize`); some b_k is negative (`NegativeStride`); or the largest addressed element
m = o_e + Σ (n_k − 1)·e_k lies outside the allocation, that is (m + 1)·s > extent in bytes (`OutOfExtent`).
A view with some n_k = 0 addresses nothing and is never out of extent.

*Elements to bytes.* Input: item size s; element origin o_e; unsigned element strides e₁ … e_d; shape; the
extent in bytes. Output: b_k = e_k·s and o_b = o_e·s, failing on arithmetic overflow (`Overflow`, checked
multiplication in `isize`) or on the same `OutOfExtent` condition.

Neither function copies data or reinterprets a buffer; each is a statement about index arithmetic. In
franken_numpy the functions sit beside its view type and name FrankenTorch's convention in their documentation;
in FrankenTorch likewise in reverse. Neither library gains a dependency on the other. Additive: no existing
type or behaviour changes.

**Interim posture here.** A typed converter at this project's boundary; no bare stride tuple crosses it. It
carries the logical origin, the allocation extent, the stride unit and the signedness. A view the target cannot
represent is materialized and the copy charged to the caller, or refused under a zero-copy contract
(`FROB-REF-617`). A writable view is also checked for independence: shared elements are admitted for reads; a
write needs an injective index map, a declared reduction or separate storage.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FNP-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FNP-01`, compiled as the `frob-conformance` integration test `upstream_fd_fnp_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It looks for the named conversion in each library at its pinned commit
(absent in both) and, when present, runs the three hand cases through the libraries' own functions.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) write a new resolution receipt (`frob.stack-receipt/1`) under `receipts/`
with `cargo xtask check-stack --write-receipt --reason pin-move --gate <INT-…>`, naming the admission gate of the
sibling that moved, INT-FNP-01 for franken_numpy and INT-FT-01 for FrankenTorch (plain `check-stack` only
verifies and writes nothing); (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** The rule is "replace the local converter; re-run INT-FNP-02": the local
converter's arithmetic is replaced by calls to the libraries' functions (its policy stays here: materialize or
refuse, charge the copy, check write independence), and INT-FNP-02 is re-run (the conversion at the
interchange boundary reads element 3, not element 12, on the 2×2 case; the negative-stride view is materialized
with the copy charged to the caller, or refused under a zero-copy contract with `FROB-REF-617`), with INT-FNP-01
(an array file written through `fnp-io` reads back identically). The switch happens only when both libraries
have landed their half; with one half landed, the local arithmetic stays for the other direction.

**Fallback if the change cannot land.** The local converter, re-tested at every pin move of either library.

## Acceptance criteria

- [ ] In each repository: the two functions and the error type exist, documented with the other library's convention named, and pass the cases below; existing view tests are unchanged.
- [ ] Here: the boundary converter contains no stride arithmetic of its own at the new pins (a source scan); its policy tests still pass.
- [ ] Here: INT-FNP-02 and INT-FNP-01 pass at the new pins with their results recorded.
- [ ] Here: a bare stride tuple still cannot cross the boundary: BD-036's compile-fail test `bare_tuple_does_not_compile` and its run-time test `foreign_descriptor_without_unit_refused` still pass at the new pins.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In each repository (added):
- `contiguous_2x2_f32`: s = 4, o_b = 0, byte strides (8, 4), shape (2, 2), extent 64 bytes; expected element strides (2, 1), and index [1, 1] is element 3.
- `negative_stride_is_refused`: s = 4, o_b = 4, byte strides (8, −4); expected `NegativeStride { axis: 1 }`. (The view addresses elements (1, 0, 3, 2), which unsigned strides cannot express.)
- `non_multiple_is_refused`: s = 4, byte strides (6, 4); expected `NotMultipleOfItemSize { axis: 0 }`.
- `misaligned_origin_is_refused`: s = 4, o_b = 2; expected `MisalignedOrigin`.
- `out_of_extent_is_refused`: elements to bytes with s = 4, o_e = 0, element strides (8, 4), shape (2, 2), extent 48 bytes (12 elements); the largest addressed element is 12, and (12 + 1)·4 = 52 > 48; expected `OutOfExtent`. The same view with extent 52 bytes is accepted.
- `overflow_is_refused`: elements to bytes with s = 8 and an element stride of 2⁶²; expected `Overflow`.
- Property test: for 10⁴ random shapes up to rank 4, item sizes in {1, 2, 4, 8} and non-negative strides that are multiples of the item size, bytes → elements → bytes is the identity, and every index addresses the same element on both sides.
Here:
- `boundary_uses_library_conversions`: a source scan and a behaviour test at the new pins.
- `negative_stride_is_materialized_or_refused`: default contract: a copy of 16 bytes charged to the receipt; zero-copy contract: `FROB-REF-617`.
- `aliasing_write_is_refused`: element strides (1, 1), stores of 7 and 11; refused; the read-only view admitted.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fnp_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fnp_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also writes an array file through `fnp-io` and reads it back
(byte-identical), and round-trips a policy tensor through the FrankenTorch interchange path.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fnp_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FNP-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-777: index [1, 1] is element 3; the negative-stride view addresses (1, 0, 3, 2) and is materialized or refused; negative control: copying (8, 4) verbatim addresses element 8 + 4 = 12, in bounds. CX-353: "a stride tuple means the same thing in every array library".
- Negative controls: a conversion that drops the divisibility check must accept (6, 4) and be caught; one that takes absolute values of negative strides turns origin 1 and strides (2, −1) into (2, 1), addresses (1, 2, 3, 4) where (1, 0, 3, 2) is right (element 4 outside a four-element allocation), and must be caught by the element-order test; one that checks the extent in elements against a byte count must accept the 48-byte case above and be caught.
- Determinism lanes: exact integer arithmetic, identical on every platform (class D0).

## Considerations

- RISK-24 lists the stride convention among the reported foundation defects that must not be relied on before recheck; the wrong read is in bounds, so only a value test finds it.
- franken_numpy is an `f64` array library with no single-precision storage (FACT); the conversion is stated for any item size, and the 2×2 case is tested at s = 4 as index arithmetic and at s = 8 on a real array (byte strides (16, 8), elements (2, 1)).
- Two repositories, two mirrored beads, two landings: the item is `landed` only when both are.
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

Appendix F.1 (row FD-FNP-01), F.2 (row 3), F.3; §33.11 (row 3), §21.5, §33.4 (the note on franken_numpy), §33.8; INT-FNP-01, INT-FNP-02; FROB-REF-617; KA-777; CX-353; RISK-24.
