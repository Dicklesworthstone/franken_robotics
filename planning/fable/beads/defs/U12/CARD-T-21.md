---
key: "CARD-T-21"
title: "Theorem card T-21: Farkas soundness (general) and the Farkas witness check (instance)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-cert", "frob-lemma", "solid", "test"]
parent: "WP-42"
deps: ["BD-132", "N12-06", "SPEC-C-32"]
needs: []
related: ["BD-382", "BD-130", "BD-115", "N12-16", "CARD-T-22", "CARD-T-35"]
gates: []
plan_refs: ["§12.3 (T-21)", "§12.4 (farkas_sound)", "§10 C-32", "§25.5.2 (lp-farkas)", "Appendix D.6 (Farkas witness)"]
---
## Background

To exclude a cell of a design family it is not enough that a search found nothing; an argument must cover every
admissible realization. The cheapest sound argument is a necessary-condition exclusion: relax controller and
dynamics to a linear system (any contact force in an outer cone, any torque under an envelope) and show the
relaxation infeasible. Infeasibility of a linear system has a short, checkable witness, a dual vector, and
checking it is two exact rational matrix–vector products. The same dual object is the separating witness returned
when a task wrench lies outside a body's attainable set, so a skill is refused before any training.

Statement (T-21). Farkas soundness: if Mᵀy ≥ 0 and rᵀy < 0 then no x ≥ 0 satisfies M·x = r. Instance: a given
rational (M, r, y) satisfies the hypotheses.

Proof of the general half in one line: if x ≥ 0 and M·x = r then rᵀy = xᵀ(Mᵀy) ≥ 0, a contradiction. It holds over
any ordered field, so in particular for real forces with rational data. The conic form: {x ∈ 𝒦, M·x = r} is
infeasible if some y has Mᵀy ∈ 𝒦* and rᵀy < 0; only a strict witness is accepted, otherwise the verdict is
`unknown`. Route: Lean for the general statement over an ordered field (Frontier); exact rational arithmetic for
the instance (Solid). Rejected alternatives: accepting a solver's infeasibility status; accepting infeasible
sample points as an exclusion of their cell (they do not eliminate it).

## Scope

In: statement document, registry row, the instance route through the `lp-farkas` checker (already built by
BD-132) with specimens and mutants bound to this card; the registered `Statement` for the general half. Out: the
checker (BD-132) and its conic sibling (N12-16); the Lean proof, which is the soundness theorem of the first
reflector (`farkas_sound`, BD-382); producing dual vectors (the cone-program solver and the mission lanes).

## Technical approach

1. Instance route (class `Certified`, bundle kind `lp-farkas`). Standard form: rational M (m × n), r (m), y (m);
   the auditor computes z = Mᵀy and checks every zⱼ ≥ 0, computes rᵀy and checks it < 0, in exact rationals.
   Box form used by mission answers for A·f = w, 0 ≤ f ≤ c: with z = Aᵀy the largest value of yᵀA·f over the box is
   β = Σᵢ max(0, zᵢ)·cᵢ, and the witness is accepted when yᵀw > β strictly. The box form reduces to the standard
   form by slack variables (f + s = c, s ≥ 0) and the dual (−y, max(0, z)): the checker implements the box form
   directly and the reduction is recorded in the statement document.
2. Over a cell the dual may be constant or rational in the cell parameters, with a sign proof: yᵀw − β affine in
   each parameter and positive at every vertex of the cell, strictness handled at open ends.
3. General route (class `Proved`, Frontier): the registered statement is

   `farkas M r y = true → ¬ ∃ x : Fin n → K, (∀ j, 0 ≤ x j) ∧ (M.map Rat.cast).mulVec x = fun i => (r i : K)`

   for M, r, y over ℚ and every linearly ordered field K, with `farkas` the Boolean check of step 1. It is proved
   in stock Lean, re-checked by FrankenLean and sealed as part of the reflection pilot; this card's registry row
   takes the capsule hash from there. If reflection is parked by its kill gate, the theorem can still be sealed
   as an ordinary capsule: parking reflection does not park the theorem.
4. Class and limits: `Certified` per instance; `Proved` for soundness once sealed. A witness certifies
   infeasibility of the relaxation it names; that the relaxation covers every realization of the cell is a set
   inclusion recorded in the bundle's primitive equations and is not proved by the witness.
5. Relies on it: construction C-32 (dual witnesses of infeasibility); proof rule 1 of mission answers
   (necessary-condition exclusion); the stance exclusion (card T-22); attainability refusals of control synthesis.
6. Artifacts: `docs/math/cards/T-21.md`; registry row; specimens `lp-farkas/accept-T-21-*.frobcert` with mutants;
   `lean/Frob/Statements/T21.lean`.

## Acceptance criteria

- [ ] `frob-verify` accepts the stance dual at half-width 1/5 m and rejects a perturbed multiplier, a non-strict
      witness and a witness with a negative entry of Mᵀy in standard form.
- [ ] The box form and the standard form with slacks give the same verdict on 1,000 random rational systems
      (property test), and feasible systems never have an accepted witness.
- [ ] Registry row: instance route `checked`, class `Certified`; general route `hypothesis` until the capsule
      exists, then `checked`, class `Proved`, with both checker identities.
- [ ] The statement document states the relaxation caveat and the strictness rule.

## Test plan

### Unit tests

`crates/frob-cert/tests/cards_t21.rs`:
- `stance_dual_box_form`: A rows (1,1,1,1), (1,1,−1,−1), (1,−1,1,−1); c = 160 each; w = (400, 180, 150);
  y = (0, ½, ½): z = (1, 0, 0, −1), β = 160, yᵀw = 165 > 160.
- `stance_dual_standard_form`: unknowns (f, s) ≥ 0; rows Σ f = 400, f₁ + f₂ − f₃ − f₄ = 180, f₁ − f₂ + f₃ − f₄ = 150,
  fᵢ + sᵢ = 160; y = (0, −½, −½, 1, 0, 0, 0): Mᵀy = (0, 0, 0, 1 | 1, 0, 0, 0) ≥ 0, rᵀy = −5 < 0.
- `nonstrict_rejected`: at half-width 3/14 m the right side is (400, 180, 140) and yᵀw = 160 = β.
- `perturbed_multiplier_rejected`: y = (0, ½, ⅓) gives yᵀw = 140 < 160.
- `tiny_infeasible_system`: x₁ + x₂ = −1 with x ≥ 0: y = (1) gives Mᵀy = (1, 1) ≥ 0, rᵀy = −1 < 0.
- `feasible_system_has_no_witness` (property); `box_and_standard_agree` (property).

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) and `lemma_second_machine.sh` (bead N12-35) exercise the
specimens; `lemma_reflect.sh` (bead N12-38) covers the Lean half.

### Known answers and negative controls

KA-208 (dual witness at b = 1/5 m: `accept`; a perturbed multiplier gives `reject`). Negative controls as listed.
Determinism: exact (D0).

## Considerations

- A lifted linear program answers the stance family equally well; the witness format is solver-agnostic, so a
  generic solver's dual is checked by the same rule.
- Rational duals from floating-point solvers are obtained by rounding and re-checking; a rounded dual that fails
  is simply not a witness (the verdict stays `unknown`), never an error of the checker.

## References

§12.3, §12.4; C-32; T-22, T-35; KA-208; GATE-LEMMA-05 (the general half's capsule).
