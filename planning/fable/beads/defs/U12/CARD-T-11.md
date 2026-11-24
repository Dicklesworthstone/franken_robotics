---
key: "CARD-T-11"
title: "Theorem card T-11: a joint's screw basis is, or is not, closed under the bracket; its Hervé class is as reported"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-lie", "solid", "test"]
parent: "WP-03"
deps: ["BD-027", "N12-06", "SPEC-C-01"]
needs: []
related: ["CARD-T-16"]
gates: []
plan_refs: ["§12.3 (T-11)", "§10 C-01", "§13.9 (kernel class Exact)", "§14.4 (joint field)", "Appendix D.1 (bracket on se(3))", "Appendix I.1"]
---
## Background

A floating-base robot's configuration space is SE(3) × G₁ × … × Gₙ. Joint j contributes the map
θ ↦ ∏ₖ exp(θₖ·Sₖ) for screws Sₖ in the Lie algebra 𝔰𝔢(3). When span{Sₖ} is closed under the bracket, the joint is a
connected subgroup of SE(3), and those subgroups fall into twelve conjugacy classes (Hervé): the identity; line
translation; rotation about an axis; helical motion of pitch p; planar translation; cylindrical; spatial
translation; planar gliding SE(2); spherical SO(3); translating screw; Schönflies; and SE(3) itself. The six
lower pairs are the revolute, prismatic, helical, cylindrical, planar and spherical classes. A joint that is not
a subgroup (a universal joint) is a product of one-parameter subgroups. The design language lets an author state
a joint by its screw basis and a class; the stated class is an assertion to check, and one kernel generator
parameterized by a screw basis covers every joint only if the classification is right.

Statement (T-11). For a joint's screw basis S₁, …, S_m with rational coordinates, the reported verdict is correct:
either the span is closed under the bracket [ξ₁, ξ₂] = (ω₁ × ω₂, ω₁ × v₂ − ω₂ × v₁) (angular part first) and the
reported Hervé class is the conjugacy class of the generated subgroup, or the span is not closed and the joint
is reported as a product of one-parameter subgroups.

Route: exact rational linear algebra. Tier: Solid. Rejected alternative: an `enum JointKind` with one
hand-written code path per case, where the class is whatever the author typed.

## Scope

In: statement document, registry row, the witness format for both verdicts, the certificate per reference joint,
fixtures. Out: the classification algorithm and generated joint maps (the joint-subgroup bead of the algebra
crate); lattice-based mobility (Frontier, with its own pilot); joint limits and loop closures.

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the joint-subgroup classification of `frob-lie` (BD-046) is `related`, not a dependency (a card that waited for
its implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the
card does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(the certificate per reference joint and the design compiler's refusal of a joint whose stated class differs) is
run inside the implementing bead, with this card's law tests, as part of that bead's acceptance; the receipt
produced there is what moves the registry row for that output.

## Technical approach

1. In-tree decision (the `Exact` kernel class on the host: integers, rationals and field elements, determinism
   class D0): compute every bracket [Sᵢ, Sⱼ], and test membership in span{Sₖ} by exact rational elimination.
   Closure holds when every bracket is in the span. The class is then read from invariants of the algebra,
   computed exactly: its dimension; the dimension of its rotational part (the span of the angular components);
   whether the rotation axes are parallel, concurrent or general; the dimension of the pure-translation
   subalgebra; and the pitch (v·ω/ω·ω along the axis) where one axis exists. The twelve classes are separated by
   these invariants (dimension 0: identity; 1: line translation, rotation, or helical by pitch; 2: planar
   translation or cylindrical; 3: spatial translation, planar gliding, spherical, or translating screw; 4:
   Schönflies; 6: SE(3)).
2. Witnesses for an offline check (DESIGN; both are polynomial, in fact linear, identities over ℚ, so they travel
   under the `poly-identity` kind with a typed payload):
   - Closed: structure constants c_ijᵏ with [Sᵢ, Sⱼ] = Σₖ c_ijᵏ·Sₖ for all i < j. The auditor recomputes each
     bracket and each linear combination.
   - Not closed: a pair (i, j) and a covector w with w·Sₖ = 0 for every k and w·[Sᵢ, Sⱼ] ≠ 0. The auditor
     recomputes the m + 1 dot products; the last is a stated nonzero rational.
   - Class: the invariants above with their own small witnesses (a basis of the rotational part; a common point
     or a common direction for the axes; the pitch as a rational).
3. Class of the claim: `Certified` (an instance-level statement decided by an exact checker). No Lean route.
4. Relies on it: construction C-01; the design language's joint field (the stated class is checked against the
   basis by this card); the kernel generator for joint maps; mobility analysis.
5. Artifacts: `docs/math/cards/T-11.md`; registry row; specimens for the twelve classes and for a non-closed
   basis; mutants.

## Acceptance criteria

- [ ] Each of the twelve classes is recognized from a rational screw basis, and a non-closed basis is reported as
      a product (the seed's condition for the classification bead), each with a witness `frob-verify` accepts.
- [ ] Negative controls rejected: a universal joint (two concurrent perpendicular revolute axes) offered with
      closed-span structure constants; a spherical joint reported as "three independent revolutes" with a
      non-closure covector (none exists); one structure constant changed; a stated class that disagrees with the
      invariants (a helical joint of pitch p ≠ 0 stated as revolute).
- [ ] The design compiler refuses a joint whose stated class differs from the certified class, naming both.
- [ ] Registry row `checked`, class `Certified`, with the witness hashes and checker identity.

## Test plan

### Unit tests

`crates/frob-lie/tests/herve.rs` and `crates/frob-lemma/tests/cards/t11.rs`:
- `spherical_closed`: S₁ = (e_x, 0), S₂ = (e_y, 0), S₃ = (e_z, 0): [S₁, S₂] = (e_z, 0) = S₃ and cyclic; class
  spherical.
- `universal_not_closed`: S₁ = (e_x, 0), S₂ = (e_y, 0): [S₁, S₂] = (e_z, 0) is outside the span; covector
  w = ((0, 0, 1), 0) vanishes on S₁, S₂ and gives 1 on the bracket.
- `cylindrical_closed`: (e_z, 0) and (0, e_z) commute; class cylindrical.
- `parallel_revolutes_generate_planar_gliding`: (e_z, 0) and (e_z, (0, −1, 0)) (the second axis through
  (1, 0, 0)) have bracket (0, (1, 0, 0)), a translation outside the span; the basis is not closed, and the
  generated algebra has dimension 3, class planar gliding.
- `helical_pitch_recovered`: (e_z, p·e_z) has pitch p; stated as revolute it is refused.
- `twelve_classes_table` (one fixture per class); `structure_constant_mutant_rejects`.
- `universal_with_structure_constants_rejects`: the universal basis offered with c₁₂³ = 1 against a phantom S₃;
  the auditor recomputes [S₁, S₂] = (e_z, 0), finds no S₃ in the basis, and rejects.
- `spherical_non_closure_claim_rejects`: the spherical basis offered as "not closed" with w = ((0, 0, 1), 0): w
  does not vanish on S₃ = (e_z, 0), and since [S₁, S₂] = S₃ no covector can vanish on the basis and not on the
  bracket; `reject`.

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) includes the witnesses as `poly-identity` specimens;
design-side scripts exercise the refusal of a mis-stated class through `frob design check`. For this card the
NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-11"), `step` (`T-11/<class-or-mutant>`),
`event`, the exact command line, exit code, wall time, the witness hash, the expected and observed verdict, the
reported class against the stated class, and on a mismatch the first failing rule (the bracket or dot product
that differs, with both rationals) and the fixture path to reproduce.

### Known answers and negative controls

Known answers of the construction: the Sarrus linkage has mobility 1 where the Grübler–Kutzbach count
M = 6·(n − 1 − j) + Σ fᵢ gives 0, and the Bennett linkage has mobility 1 where the count gives −2 (these belong to
mobility analysis, which uses the classification). Negative controls: the four rejected fixtures. Determinism:
exact rational arithmetic, D0.

## Considerations

- Screw coordinates in a design file are decimal or rational; decimals are read as exact rationals, so a basis
  that is closed only approximately is reported as not closed, with the residual, rather than snapped.
- Conjugacy classes are up to rigid motion; the class is reported with the canonical frame (axis, point, plane)
  that realizes it.
- If a dedicated proof kind for rank and span statements is added to the registry later, the witnesses move to it
  by a registry version bump; until then they are linear identities under `poly-identity`.

## References

§12.3; C-01; the `Exact` kernel class; BD-046's done-condition.
