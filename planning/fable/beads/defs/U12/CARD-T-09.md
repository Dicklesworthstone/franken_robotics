---
key: "CARD-T-09"
title: "Theorem card T-09: every kernel is dimensionally homogeneous; square roots only on even exponent vectors"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "frob-kir", "solid", "test"]
parent: "WP-05"
deps: ["N12-06", "SPEC-C-14"]
needs: []
related: ["BD-021", "N12-20", "CARD-T-10", "BD-018", "SPEC-C-21"]
gates: []
plan_refs: ["§12.3 (T-09)", "§10 C-14", "§10 C-21", "§21.7", "§21.14 (PO-SPARK-01)", "Appendix D.5 (similarity rescaling)"]
---
## Background

A kernel's arithmetic is a straight-line program over +, −, ×, ÷, √ and selects. Every value carries its physical
dimension as an exponent vector over the base dimensions of the quantity library (mass, length, time and the
others), and a program type-checks only if every sum is homogeneous: physical quantities form a graded algebra
and the kernel respects the grading. This is not a lint. It is a decision procedure, and it is exact.

Statement (T-09). Every kernel is dimensionally homogeneous, and every square root is taken on an even exponent
vector. Precisely, for a program accepted by the kernel-IR type checker: both operands of every sum, difference,
comparison, minimum, maximum and select branch have equal exponent vectors; a product has the sum of its
operands' vectors and a quotient their difference; a square root is applied only to a value whose exponent
vector has all-even entries (so its result has integer exponents); literals with a dimension declare it.

Why it matters beyond catching unit errors: it makes dynamic similarity exact. Under a change of units of mass,
length and time by factors (α_M, α_L, α_T), a quantity of dimension MᵃLᵇTᶜ scales by α_Mᵃ·α_Lᵇ·α_Tᶜ; a homogeneous
kernel uses only +, −, ×, ÷, √ and comparisons, and with the square-root rule every intermediate value is scaled
by a power of two when the three factors are powers of two, which binary floating point represents exactly away
from overflow and underflow. The scaled run must then reproduce the original in every significand bit and every
branch (a HYPOTHESIS whose test is a gate elsewhere). Tier: Solid. Route: decided exactly by the kernel-IR type
checker. Rejected alternative: unit annotations in comments, or a run-time unit check.

## Scope

In: the card's statement and registry row; the binding of the type checker's verdict to the card (a `checked`
status per kernel); the negative fixtures. Out: the type checker itself (the kernel-IR bead); the power-of-two
metamorphic test (the canonical-form and determinism beads); units at public APIs.

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the kernel IR with its dimension checker (BD-018) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(`dimension_check: pass` on every kernel of the library, and the registry count of kernels checked) is run inside
the implementing bead, with this card's law tests, as part of that bead's acceptance; the receipt produced there
is what moves the registry row for that output.

## Technical approach

1. Route: the kernel-IR dimension checker (class `Certified`: an instance-level statement about this kernel,
   decided by a checker whose verdict is recorded with the kernel's content hash). The checker is a single
   forward pass that infers an exponent vector per value and fails on the first violation, naming the operation
   and both vectors.
2. Rules checked, exhaustively: add, sub, min, max, compare, select branches → equal vectors; mul → vector sum;
   div → vector difference; sqrt → every entry even, result is half; neg, abs → unchanged; integer and Boolean
   values are dimensionless; a loop-carried value has the same vector at entry and at yield; a load or store
   agrees with the buffer schema's recorded dimension (GPU buffers are raw single precision with the dimension
   recorded in the buffer schema).
3. Receipt: the kernel manifest records `dimension_check: pass` with the checker's build; the theorem-card
   registry aggregates "kernels checked: n of n". There is no bundle kind for this card: the decision is not a
   witness to be re-checked offline but a property re-decidable by anyone who runs the checker on the kernel's
   canonical text, which is content-addressed. For an offline reader the kernel text and the rule list above
   suffice; an independent re-implementation of the pass lives in the conformance crate as the second opinion.
4. Class and limits: `Certified` per kernel. It certifies homogeneity of the program as written; it cannot know
   that a literal typed as a pure number is secretly a length. That case is caught by the power-of-two scaling
   test, where a kernel hiding a dimensional constant typed as a pure number fails on its first step.
5. Relies on it: constructions C-14 and C-21; the similarity canonical form of designs; every kernel in the
   library; proof obligation PO-SPARK-01.
6. Artifacts: `docs/math/cards/T-09.md`; registry row; negative fixtures under the kernel-IR crate's tests.

## Acceptance criteria

- [ ] Every kernel in the library at each milestone has `dimension_check: pass`; a kernel without it cannot be
      emitted.
- [ ] Negative controls rejected by the checker with the operation named: a program that adds a length to a
      time; a square root of a value of dimension length (odd exponent); a select whose branches differ in
      dimension; a store of a force into a buffer whose schema says torque.
- [ ] The independent re-implementation in the conformance crate agrees with the checker on every kernel and on
      every negative fixture.
- [ ] The registry row shows the route `checked` with the count of kernels and the checker's identity.
- [ ] The statement document lists the rules and the limit (pure-number literals).

## Test plan

### Unit tests

`crates/frob-kir/tests/dimension.rs` (owned with the IR): `sum_of_length_and_time_rejected`;
`sqrt_of_length_rejected`; `sqrt_of_area_is_length`; `product_and_quotient_vectors`;
`select_branch_mismatch_rejected`; `loop_carried_dimension_stable`; `buffer_schema_mismatch_rejected`.
`crates/frob-conformance/tests/dimension_second_opinion.rs`: `agrees_on_all_kernels`;
`agrees_on_negative_fixtures`.
`crates/frob-lemma/tests/cards/t09.rs`: `registry_counts_checked_kernels`.

### End-to-end script and logging

`scripts/e2e/lemma_kernel_cert.sh` (bead N12-33) prints, for the M0 kernels, the dimension check's verdict and
checker identity alongside the certificates.

### Known answers and negative controls

Known answer from similarity: a pendulum described by (T, ℓ, g, m) has one dimensionless group, T·√(g/ℓ): n = 4
quantities of dimensional rank 3 give n − r = 1. Exact metamorphic companion (not this card's claim): lengths ×4,
times ×2, masses ×2, gravity unchanged, rerun and compare bits; the anchor cell side of 8 m becomes 32 m when
lengths are multiplied by 4. Negative controls: the four rejected programs. Determinism: the checker is exact
and platform-independent.

## Considerations

- Tolerances carry dimensions too: one unitless epsilon across angles, impulses and temperatures is invalid and
  is rejected by the same rules.
- Elementary functions are written in the IR from the same operators, so they inherit the check.
- Fixed-point types are reserved and unused through M5; their dimension rules are the same.

## References

§12.3; C-14, C-21; PO-SPARK-01; §21.7.
