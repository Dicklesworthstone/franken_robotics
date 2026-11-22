---
key: "N12-CX"
title: "Lemma counterexample registry: CX-106 to CX-115, the shortcuts that certificates must not take"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-conformance", "solid", "registry", "test"]
parent: "WP-36"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the negative-control registry and the counterexample catalogue seeds in frob-conformance (BD-146)"]
related: ["N12-KA", "N12-REF", "N12-34", "N12-07", "N12-08"]
gates: []
plan_refs: ["Appendix N.9", "§12.2", "§12.3", "§12.4", "§12.5", "§12.7", "§0.7 (counterexamples are first-class)"]
---
## Background

Counterexamples are first-class: where a plausible shortcut is wrong, the smallest hand-checkable case that breaks
it is written down, and the case enters the negative-control registry and the counterexample catalogue. A rule
that ships without such a case has not been thought through. Lemma's ten entries, CX-106 to CX-115, are the
shortcuts by which a certificate, a bundle or a class label could come to mean more than it does. Each seeds a
negative control: a test in which the shortcut is planted and must be caught.

This bead registers the ten entries as data, with the tempting shortcut, the case with its numbers, the rule the
case protects, and the control test that realizes it. Mechanism beads cite the CX numbers. Counterexamples from
other chapters that Lemma's checkers also enforce are listed at the end for cross-reference. Tier: Solid.

## Scope

In: the ten rows in `registries/counterexamples/lemma.toml`; the binding of each to a control test; the registry
check. Out: the control tests' implementations (in the beads named); other chapters' counterexamples.

## Technical approach

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control realized by |
|---|---|---|---|---|
| CX-106 | A bundle whose hashes are all consistent is a valid certificate | A bundle with an unknown proof kind and consistent hashes: every digest matches and no checker implements the argument. The auditor returns `reject` | The registry of proof kinds is closed and versioned (ADR-LEMMA-04, INV-LEMMA-04, FROB-REF-823) | N12-02 |
| CX-107 | A certificate that checks can be offered for any query about the same model | The docking cell (U, ε) = (¾, ¼) with the packet on time is accepted for the inclusion; the same bundle with its quantifiers reordered is rejected, although every hash is still consistent | The quantifier prefix, domain and assumption set of a certificate must match the query (FROB-REF-826) | N12-03 |
| CX-108 | A witness is valid if its numbers are right, whatever order they are listed in | Stance specimen: forces (160, 130, 110, 0) N are typed to the port order [++, +−, −+, −−]. With the foot order swapped and the forces unchanged the total is still 400 N and the primitive moment equations no longer hold (Σ x·f becomes 140/3 N·m instead of 60). Three further mutations must also be rejected: an unreported positive force floor; a passive-authority tag; (60, 30) passed as the torque | Every field that enters the argument is typed with frame, origin, sign and units, and the primitive equations travel in the bundle (FROB-REF-829, GATE-LEMMA-03) | BD-132, N12-34 |
| CX-109 | An exact argument yields a `Certified` result | An exact rational argument over an `Estimated` material card is `Estimated`; an energy budget computed exactly in simulation over `Estimated` inputs is `Estimated` | The class of a combined result is the minimum of its inputs' classes; nothing converts a claim product into a proof (INV-LEMMA-02, FROB-REF-834, GATE-LEMMA-06) | N12-11, N12-05, N12-17 |
| CX-110 | A study receipt in a bundle proves the study's conclusion | The auditor recomputes the arithmetic of a recorded study (counts, e-values, spent α); it cannot attest that the episodes ran | `sample-coverage` is never a proof: class at most `Tested` (ADR-LEMMA-06) | N12-13 |
| CX-111 | A reassociation is a harmless rewrite and can be filed as bit-exact (class E) | A rewrite trace that contains a reassociation filed under class E is replayed by the independent checker and rejected: reassociation is not in the closed table of class-E rules. In single precision with a = 10⁸, b = −10⁸, c = 1, (a + b) + c = 1 and a + (b + c) = 0 | A class-E trace uses only rules of the closed table (T-27); equality over ℝ is the theorem, and bit-exactness in floating point is a test | N12-18 |
| CX-112 | A kernel certificate shows that the kernel's floating-point output satisfies the identity | Certificates are about the straight-line program over ℚ with square roots; rounding is not modeled, and round-off breaks exact equivariance | The floating-point gap is measured by residual tests with dimensioned tolerances and by the power-of-two scaling test, not proved | N12-07, N12-08 |
| CX-113 | The articulated variational step is symplectic, like the free-body step | False as stated for a configuration-dependent mass matrix: a kick–drift splitting is then no longer the variational step | No symplectic claim beyond one rigid body; energy behaviour is measured | N12-07 |
| CX-114 | The boundary value of a threshold can be reported as the attained optimum | Docking family: for U ∈ [½, 1] a robust causal policy exists if and only if the packet is on time and ε < (U + ½)/4. At U = ¾ the bound is 5/16 and ε = 5/16 is not admitted: the supremum is not attained. The stance minimum b = 3/14 m is attained and carries a matching lower bound (the dual witness for every smaller b) | A minimum is claimed only with the incumbent value and a checked matching lower bound; an infimum that is not attained is returned as an open threshold with a feasible candidate and its gap | N12-03 |
| CX-115 | An artifact with no authorization fields is neutral | `frob-verify` rejects an artifact in which a flag is missing, exactly as it rejects one in which `execution_authorized` is true | Every exported artifact carries the three flags, false, and no code path sets one (INV-LEMMA-03, FROB-REF-833) | N12-03, N12-11 |

Counterexamples of other chapters that Lemma's checkers enforce and cite (owned and registered elsewhere; listed
here so that the mutation corpus includes them): CX-037 (reuse a result after an edit because the sub-term's hash
still matches: refused without a recorded implication, FROB-REF-835); CX-045 (one policy per world offered for
one policy for all worlds: rejected by the query-match rule); CX-075 (a safety certificate proved in the order
∀w ∃u offered for ∃u ∀(w, e): rejected, FROB-REF-826); CX-077 (the first moment of the load passed where the
support torque is expected: with +z up the support torque is (P_y, −P_x, 0), so P = (60, 30) N·m gives
(30, −60, 0) N·m; rejected, FROB-REF-829); CX-096 and CX-097 (product systems coupled by tick, and "never loses"
read as "meets the mission").

Registry mechanics: each row has `id`, `shortcut`, `case`, `rule`, `defined_in`, `controls` (test identifiers).
The check fails when a row has no control test or its control test passes with the shortcut planted. A planted
shortcut is a named build feature or a fixture mutation, recorded with the row (for example
`shortcut-accept-on-hashes` for CX-106).

## Acceptance criteria

- [ ] `registries/counterexamples/lemma.toml` holds exactly CX-106 to CX-115 with the columns above.
- [ ] Each row is bound to a control test that fails when its shortcut is planted, and the planting mechanism is
      recorded.
- [ ] The cross-referenced rows of other chapters each have a corresponding mutant in the auditor's corpus.
- [ ] The registry check fails if a control is removed (negative control of the check).
- [ ] Every row's case is reproducible by hand from the numbers in the row.

## Test plan

### Unit tests

`crates/frob-conformance/tests/cx_registry_lemma.rs`: `lemma_rows_106_to_115_present`; `each_row_has_control`;
`planted_shortcut_fails_control` (table-driven over the ten rows: with the shortcut feature on, the named test
must fail); `cross_refs_have_mutants`; `removed_control_fails_check`.
Hand-check tests that restate the arithmetic: `cx108_swapped_ports_moment` ((1/3)·(160 + 110) − (1/3)·(130 + 0)
= 140/3); `cx111_f32_reassociation` (1 against 0); `cx114_docking_bound_at_three_quarters` (5/16, and the shared
reading 15/16 at equality).

### End-to-end script and logging

`lemma_auditor_mutations.sh` asserts the bundle-level rows (CX-106, -107, -108, -110, -111, -114, -115) through
their mutants and logs the CX identifier on each `assert` line; `lemma_flags_claims.sh` asserts CX-109 and CX-115
on exported artifacts; CX-112 and CX-113 are asserted as refusals to issue a Proof claim (N12-07).

### Known answers and negative controls

This bead is the negative-control seed for Lemma. Related known answers: KA-207, KA-209, KA-212, KA-214, KA-215,
KA-216, KA-221. Determinism: every case is exact.

## Considerations

- A counterexample row is never deleted when its shortcut becomes impossible by construction; it stays as the
  regression test for the construction.
- One false certificate found in use becomes a permanent fixture of the mutation corpus and, where it shows a new
  shortcut, a new row at the end of the catalogue.

## References

Appendix N.9; §12.2 to §12.7; ADR-LEMMA-04, ADR-LEMMA-06; INV-LEMMA-02, -03, -04; FROB-REF-823, -826, -829, -833,
-834; T-23, T-27.
