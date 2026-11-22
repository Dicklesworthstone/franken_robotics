---
key: "FDEF-03"
title: "defect report, franken_numpy and frankentorch: the stride-convention trap at the interchange boundary"
type: "task"
priority: 2
milestone: "M0"
labels: ["m0", "suite", "spark", "upstream", "franken_numpy", "frankentorch", "solid", "test"]
parent: "WP-37"
deps: ["BD-036", "BD-037", "STD-E2E"]
needs: []
related: ["FD-FNP-01", "BD-193", "BD-301"]
gates: ["INT-FNP-02"]
plan_refs: ["Appendix F.2 row 3", "§33.11", "§21.5", "§33.10", "Appendix F.3", "§0.1"]
---
## Background

Arrays cross a boundary in this project wherever data is exported or interchanged: array files written through
franken_numpy's `fnp-io`, policy tensors exchanged with FrankenTorch, and the Gymnasium-compatible adapter. The
two libraries describe a strided view differently. franken_numpy's `fnp-ndarray` uses **signed byte strides**;
FrankenTorch's `ft-core` uses **unsigned element strides**. Neither library is wrong alone. The trap is at the
boundary: a stride tuple copied verbatim from one side to the other reads the wrong element, in bounds, and
nothing faults.

The plan's rule: a reported defect or limit in a foundation is a **FACT to recheck**; it becomes an admission
test here first (INT-FNP-02, BD-036) and an upstream work item there second (FD-FNP-01). This bead is the report
between the two. Both conventions were confirmed by reading on 2026-11-21 (`fnp-ndarray/src/lib.rs:258, :390` at
`e8cb85f4e5ac`; `ft-core/src/lib.rs:397, :2460` at `8165d6a18028`); the wrong read itself was not executed.

## Scope

In: the report (`tests/upstream/FD-FNP-01/REPORT.md`, filed in both siblings' trackers); the standalone
reproduction; the observed values copied from INT-FNP-02's record; the guard kept here. Out: the typed
converter and the admission tests INT-FNP-01 and INT-FNP-02 (BD-036); device-resident tensor views (BD-193); the
named conversion upstream (FD-FNP-01).

## Technical approach

**The hazard, stated as a defect of the boundary.** A view is (origin, shape, strides). In `fnp-ndarray` the
strides are byte offsets and may be negative; in `ft-core` they are element counts and cannot be negative. No
named conversion exists between the two, so any consumer of both writes its own, and the obvious one (copy the
tuple) is wrong.

**Smallest reproducing cases (hand-checkable).**

1. *Wrong element, in bounds.* A contiguous 2×2 single-precision view over a 16-element buffer has strides
   (8, 4) in bytes and (2, 1) in elements. Copying (8, 4) into an element-stride view addresses element
   1·8 + 1·4 = 12 at index [1, 1], where element 1·2 + 1·1 = 3 belongs. Both are inside the buffer.
2. *Unrepresentable view.* A view with byte origin 4 and byte strides (8, −4) addresses elements
   (1, 0, 3, 2) in row-major order of its indices: [0, 0] → byte 4 → element 1; [0, 1] → byte 0 → element 0;
   [1, 0] → byte 12 → element 3; [1, 1] → byte 8 → element 2. Unsigned strides cannot express it.
3. *Aliasing writable view.* A 2×2 view with element strides (1, 1) maps index [0, 1] and index [1, 0] to the
   same element. Stores of 7 at [0, 1] and 11 at [1, 0] leave a result that depends on their order. Bounds
   checks pass; independence fails.

**The reproduction (standalone).** A small crate depending only on `fnp-ndarray` and `ft-core` at their pinned
commits (it is not part of this repository's workspace graph; it lives under the fixture directory with its own
manifest). It builds the 16-element buffer 0.0, 1.0, …, 15.0, constructs the view on each side, performs the
verbatim copy of strides, and prints the element read at [1, 1] (expected 3.0, observed with the naive copy
12.0); then attempts case 2 and prints what each library's constructor answers.

**Proposed fix (carried by FD-FNP-01).** A named conversion in each library, with the unrepresentable cases
refused: from (byte origin, signed byte strides, item size, allocation extent) to (element origin, unsigned
element strides), which fails when a stride is negative, is not a multiple of the item size, or the origin is
not aligned to the item; and its inverse. Additive: new functions, no change to either existing convention.

**Admission test and pass condition (INT-FNP-02, run by BD-036).** The conversion at the interchange boundary
reads element 3, not element 12, on the 2×2 case; the negative-stride view is materialized with the copy
charged to the caller, or refused under a zero-copy contract with `FROB-REF-617`. The seed bead's
done-condition: an array file reads back identically (INT-FNP-01); the 2×2 case reads element 3, and the
verbatim stride copy that reads element 12 is refused.

**The guard this repository keeps meanwhile.** A typed converter at this project's boundary; no bare stride
tuple crosses it. The converter carries four things: the logical origin, the allocation extent, the stride unit
(bytes or elements) and the signedness. A view the target cannot represent is materialized and the copy is
charged to the caller's receipt, or refused under a zero-copy contract. A writable view is also checked for
independence, separately from bounds: shared elements are admitted for reads; a write needs an injective index
map, a declared reduction or separate storage, and is refused otherwise.

**Procedure.** (1) Copy the observed values and both commits from INT-FNP-02's record into the report
(`observed_at`). (2) Run the reproduction in a scratch directory at both pins. (3) File the report as the
mirrored bead of FD-FNP-01 in each sibling's tracker (two identifiers, both stored in the `mirror` field of the
registry row), under each repository's own `AGENTS.md`. The report is worded as a boundary hazard and a request
for a named conversion, not as a bug in either library.

## Acceptance criteria

- [ ] `tests/upstream/FD-FNP-01/REPORT.md` has the sections Hazard, Evidence (both files, lines, commits), the three cases, Expected and observed, Proposed fix, Interim posture, and `observed_at`.
- [ ] The reproduction builds against the two pinned crates alone and prints 3.0 expected and 12.0 observed for case 1, and each library's answer for case 2.
- [ ] The report is filed in both trackers and both identifiers are in the registry row of FD-FNP-01.
- [ ] In this repository, a search finds no function at an interchange boundary that takes a bare stride tuple; the typed converter is the only route, and a planted function that takes `(usize, usize)` strides from `fnp-ndarray` is caught by the boundary lint of the converter's crate.
- [ ] The three cases are tests of the typed converter: element 3; materialize-or-refuse with `FROB-REF-617`; the aliasing write refused.

## Test plan

### Unit tests
- `verbatim_copy_reads_element_12`: buffer 0..15, strides (8, 4) used as element strides; expected read at [1, 1] is 12.0 (the trap, shown).
- `typed_conversion_reads_element_3`: the converter on the same view; expected 3.0.
- `negative_stride_view_addresses_1_0_3_2`: byte origin 4, strides (8, −4); expected element order (1, 0, 3, 2) on the franken_numpy side.
- `negative_stride_is_materialized_or_refused`: under the default contract a copy is made and its byte count (16) is charged to the receipt; under the zero-copy contract the refusal code is `FROB-REF-617`.
- `aliasing_write_is_refused`: element strides (1, 1), writes of 7 and 11; expected refusal; the same view read-only is admitted.
- Property test: for 10⁴ random shapes up to rank 4, item sizes {1, 2, 4, 8} and non-negative strides that are multiples of the item size, byte → element → byte conversion is the identity and every index reads the same element on both sides.

### End-to-end script and logging
`scripts/e2e/defect_fdef_03.sh`: builds and runs the reproduction in `target/upstream-scratch/stride-repro/`
against the two pinned commits, asserts the printed values, runs the converter's tests, writes an array file
through `fnp-io` and reads it back (byte-identical), and reads the registry row with `frob doctor --json`.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/defect_fdef_03/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FDEF-03`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-777: index [1, 1] is element 3; the negative-stride view addresses (1, 0, 3, 2) and is materialized or refused; negative control: copying (8, 4) verbatim addresses element 12, in bounds. CX-353: "a stride tuple means the same thing in every array library".
- Negative control for the guard: a converter mutant that drops the stride unit (treats every stride as elements) must fail `typed_conversion_reads_element_3`.
- Determinism lanes: exact integer arithmetic; identical on every platform (class D0).

## Considerations

- RISK-24 names the stride convention among the four reported foundation defects that must not be relied on before recheck. The read is in bounds, so no sanitizer and no bounds check will ever find it; only a value test does.
- franken_numpy is an `f64` array library on heap vectors with no single-precision storage (FACT); the 2×2 single-precision case is therefore stated in terms of byte arithmetic on an item size of 4 and must be re-read against what `fnp-ndarray` can construct at the pin. If it cannot hold `f32`, the reproduction uses item size 8 with strides (16, 8) against (2, 1), reading element 24 of a 32-element buffer where element 3 belongs, and the report states both.
- No milestone waits for the upstream conversion; the local converter is the design of record until the switch-over (replace the local converter; re-run INT-FNP-02).
- Neither a pin nor the toolchain moves during the ten-fold tournament, so a landed conversion is adopted before or after it, never during.

## References

Appendix F.2 (row 3), F.3; §33.11 (row 3, the hand case), §21.5, §33.10; INT-FNP-01, INT-FNP-02; FD-FNP-01; FROB-REF-617; KA-777; CX-353; RISK-24.
