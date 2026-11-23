---
key: "CARD-T-22"
title: "Theorem card T-22: the stance-family specimen (minimum half-width 3/14 m with witnesses and dual exclusion)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-cert", "frob-lemma", "solid", "test"]
parent: "WP-42"
deps: ["BD-132", "N12-06", "CARD-T-21", "SPEC-C-32"]
needs: []
related: ["BD-042", "BD-133", "N12-35", "N12-34", "N12-05", "BD-382", "BD-130"]
gates: []
plan_refs: ["§12.3 (T-22)", "§10 C-32", "§25.5.3", "§25.13", "§7 NS-13", "Appendix D.6 (stance family)"]
---
## Background

The stance family is the first mission answered over a family of designs, and the specimen every certificate
mechanism is exercised on. A robot carries W = 400 N on four point feet at (±a, ±b) with a = 1/3 m; the first
moment of the load about the footprint centre may be anywhere in the rectangle P = (P_x, P_y) ∈ [−60, 60] ×
[−30, 30] N·m (a centre of mass anywhere in a rectangle of half-sizes 150 mm and 75 mm); each foot delivers a
commanded normal force in [0, 160] N; the half-width b is the free design parameter. Static equilibrium is
Σ fᵢ = W, Σ xᵢ·fᵢ = P_x, Σ yᵢ·fᵢ = P_y with 0 ≤ fᵢ ≤ 160, feet ordered [++, +−, −+, −−]. For a planar template with
feet pᵢ = A·qᵢ + d the equations are linear in the inverse geometry, the ratings and the forces, so the feasible
family is the projection of a polyhedron.

Statement (T-22). The stance-family specimen: b = 3/14 m is feasible at all four load vertices with the stated
forces; feasibility is convex in the load for an allocation chosen after the load is observed (the bilinear
interpolant of the four witnesses is such a law; no single affine law attains this b, and the affine class needs
b = 1/2 m); and every smaller b is infeasible by the dual witness.

Route: witness replay and Farkas check in exact rationals. Tier: Solid. Why a card: it fixes, with numbers, the
three things a family certificate must get right: the witness at each vertex, the class of the allocation law
that justifies interpolating between vertices, and the matching lower bound that makes "minimum" a claim.
Rejected alternatives: a support-polygon test with a total-capacity check (FACT from a planning pilot, to be
re-run: it accepted 1,596 bodies the exact model rejects); a static margin computed for one design.

## Scope

In: statement document, registry row, the three certificates (inclusion at b = 3/14 m, the controller-class
statement, exclusion for b < 3/14 m) as bundles bound to this card, with the registered mutations. Out: solving
the family (the stance-family bead of the mission crate); the checkers (BD-132); the amendment to a passive
suspension (polarity, N12-05); the lifted and knee-bound variants of the family.

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the stance-family solver of `frob-mission` (BD-130) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(bundles emitted by the solver itself) is run inside the implementing bead, with this card's law tests, as part of
that bead's acceptance; the receipt produced there is what moves the registry row for that output.

## Technical approach

1. Inclusion at b = 3/14 m (kind `lp-primal`, class `Certified`). Vertex witnesses, in port order:
   P = (60, 30): (160, 130, 110, 0); P = (60, −30): (130, 160, 0, 110); P = (−60, 30): (110, 0, 160, 130);
   P = (−60, −30): (0, 110, 130, 160) N. The auditor recomputes at each vertex Σ f = 400,
   Σ x·f = (f₁ + f₂ − f₃ − f₄)/3 = P_x, Σ y·f = (3/14)·(f₁ − f₂ + f₃ − f₄) = P_y, and 0 ≤ fᵢ ≤ 160.
2. Why 3/14: at the corner P = (60, 30) the two moment equations give f₊₊ − f₋₋ = 90 + 15/b, and with f₊₊ ≤ 160,
   f₋₋ ≥ 0 this needs b ≥ 15/70 = 3/14 m. At b = 3/14 m both bounds are active (zero reserve on two constraints),
   which the answer reports.
3. Controller class. The load is observed before the allocation is chosen (the observation contract lists W and
   P). The bilinear interpolant f(P) = Σ_v w_v(P)·f_v, with w_v the bilinear weights of the four vertices, is a
   feasible law on the whole rectangle: bilinear weights reproduce affine functions of P, so the three
   equalities hold, and a convex combination of vectors in the box stays in the box. It is not affine: an affine
   law would need f(++) + f(−−) = f(+−) + f(−+), but the witnesses give (160, 240, 240, 160) against
   (240, 160, 160, 240). Within the class of affine laws f(P) = f₀ + G·P the equalities force f₀ = 100 N per foot
   and the worst foot carries 100 + 60/(4a) + 30/(4b) = 145 + 15/(2b) N, so the rating 160 N needs b ≥ 1/2 m. The
   bundle therefore declares `law_class: adjustable` with `law: multilinear-vertex` over the arguments P_x, P_y;
   the same vertices labelled as one affine law are rejected.
4. Exclusion for every b < 3/14 m (kind `lp-farkas`, class `Certified`). In the chart v = 1/b with relaxation
   A·f = (W, 3·P_x, v·P_y), the dual y = (0, ½, ½) gives yᵀA = (1, 0, 0, −1), so yᵀA·f = f₊₊ − f₋₋ ≤ 160, while
   yᵀ(W, 3·P_x, v·P_y) = 90 + 15·v at the vertex P = (60, 30), which exceeds 160 for all v > 14/3. At b = 1/5 m
   (v = 5) it is 165 N required against 160 N available.
5. Minimum: b = 3/14 m is claimed as the attained minimum because the incumbent (step 1) and the matching lower
   bound (step 4, open at 14/3) meet.
6. Class and limits: `Certified` for the declared static model with independently commanded normal forces. It
   certifies neither run-up nor landing nor recovery, and it is false for a passive suspension, which is why the
   authority model is a typed field.
7. Artifacts: `docs/math/cards/T-22.md`; registry row; specimens `lp-primal/accept-T-22-stance.frobcert`,
   `lp-farkas/accept-T-22-dual.frobcert` and the mutation set.

## Acceptance criteria

- [ ] `frob-verify` accepts the inclusion at b = 3/14 m and the exclusion on v ∈ (14/3, 6], without trusting the
      producer.
- [ ] The four registered mutations each give `reject`: a swapped foot order with unchanged forces; an
      unreported positive force floor; a passive-authority tag; (60, 30) passed as the torque (the support torque
      with +z up is (P_y, −P_x, 0) = (30, −60, 0) N·m).
- [ ] The affine-law label on the same vertices is rejected; the exclusion cell with its end 14/3 closed is
      rejected.
- [ ] Registry row `checked`, class `Certified`, with bundle hashes and checker identity.
- [ ] The three numbers 3/14 m, 1/2 m (affine class) and 165 N (at b = 1/5 m) are reproduced by the tests from
      the equations, not copied.

## Test plan

### Unit tests

`crates/frob-cert/tests/cards_t22.rs`: `vertex_witnesses_satisfy_equations` (all four);
`corner_equation_gives_three_fourteenths` (90 + 15/b ≤ 160 ⇔ b ≥ 3/14); `bilinear_interpolant_feasible_on_grid`
(a 13 × 7 rational grid of loads, all equalities exact, all forces in [0, 160]); `witnesses_not_affine`
((160, 240, 240, 160) ≠ (240, 160, 160, 240)); `affine_class_needs_half_metre` (145 + 15/(2b) ≤ 160 ⇔ b ≥ 1/2);
`dual_at_one_fifth_is_165`; `dual_sign_on_open_cell`; `swapped_ports_rejects` (Σ x·f = 140/3 ≠ 60);
`passive_springs_counterexample` (at b = ¼ m active allocation gives (155, 135, 105, 5) N and equal passive
springs give (175, 115, 85, 25) N, above the cap).

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) and `scripts/e2e/lemma_second_machine.sh` (bead N12-35)
run the specimen and its mutations and log each first failing rule.

### Known answers and negative controls

KA-207, KA-208. The plan-time pilot's figures to re-run (not results of this system): 11,520 of 11,520 states
agree and the half-width is 3/14 m. CX-108, CX-077. Negative controls: the four mutations, the affine label, the
closed end. Determinism: exact (D0).

## Considerations

- A lifted robust linear program reaches the same optimum in one solve; the card's content is the checked
  answer, not a claim about how it is found.
- The specimen is reused by the reflection pilot (the Farkas reflector on the exclusion at b = 1/5 m).

## References

§12.3, §25.5.3, §25.13; C-32; T-21; NS-13; KA-207, KA-208; GATE-AUGUR-02 (the mission-side gate).
