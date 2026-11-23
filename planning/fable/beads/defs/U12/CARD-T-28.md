---
key: "CARD-T-28"
title: "Theorem card T-28: an energy budget satisfies its inequality and has a term for every source on the load path"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "frob-cert", "frob-shield", "solid", "test"]
parent: "WP-65"
deps: ["N12-17", "N12-06", "SPEC-C-31"]
needs: []
related: ["CARD-T-29", "CARD-T-19", "BD-338", "N12-05", "BD-337"]
gates: []
plan_refs: ["§12.3 (T-28)", "§12.7", "§10 C-31 (tier 1)", "§22.11", "Appendix D.7 (energy budget)"]
---
## Background

Safety is an energy budget first and a filter second. Where a guide puts an absorber on every forward path (a
stop, a bumper, a door, a drawer, a lift, a transient contact with a person), the energy that can reach the
absorber is bounded by its sources, whatever the contact sequence:

  ½·m_cap·v² + ∫ max(P_elec, 0) dt + E_stored + E_gravity + E_other + E_reserve ≤ W_abs,min

with m_cap an upper bound on the effective moving mass including the reflected inertia of rotors, gears and
wheels (N²·J_r/r² at a wheel of radius r), v a bound on its speed from the encoders, the integral the worst-case
drive input until the cutoff acts, E_stored springs and compliance, E_gravity the weight over the absorber stroke
and any drop on the path, E_other every other actuator on the load path, E_reserve a declared margin (zero
unless stated) and W_abs,min the lower work envelope of the absorber. Passive contact with any friction
coefficient μ ≥ 0 only dissipates or redistributes energy, so the contact model drops out and the budget
contains no term that depends on the policy: its evidence survives every retraining, and its runtime monitor is
one integrator and one comparison per tick. The budget is an artifact (`.fenergy`) that binds integer unit
scales, error bounds, the runtime admission predicate and the evidence for each term.

Statement (T-28). An energy budget satisfies its inequality with outward-rounded error bounds, and it has a term
for every source on the declared load path.

Route: exact rational or interval arithmetic for the inequality; a structural check against the port graph for
completeness. Tier: Solid (the inequality is elementary mathematics; the tier that uses it is Frontier).
Rejected alternative: checking the arithmetic alone: a budget with an omitted source is arithmetically true and
physically false.

## Scope

In: statement document, registry row, the per-budget `energy-budget` certificates bound to this card, fixtures
and mutants. Out: the `.fenergy` artifact, its monitor kernel and `shield.budget` (the energy-budget tier bead);
the auditor's checker (N12-17); tube inclusion and runtime filters (the other two tiers); any claim about a
physical robot.

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the energy-budget tier `shield.budget` with its `.fenergy` artifact (BD-337) is `related`, not a dependency (a
card that waited for its implementer would close a loop, because the implementer's own acceptance runs the card's
law tests). What the card does depend on is in `deps`: the card registry, its specification, its pilot where the
route is Frontier, and Lemma's own proof machinery. It closes on its statement document, its registry row, its
specimens and mutants, and every acceptance line that its own fixtures decide. An acceptance line that can only be
observed on live output (the certificate `shield.budget` emits for a budget) is run inside the implementing bead,
with this card's law tests, as part of that bead's acceptance; the receipt produced there is what moves the
registry row for that output.

## Technical approach

1. Per budget the producer emits an `energy-budget` proof object: the declared load path as a small port graph
   (each element with its kind and parameters), the terms with their values as outward-rounded intervals in
   joules, the absorber's lower work envelope, and the claim (`holds`, or `excludes` when the inequality is read
   as an exclusion).
2. The auditor recomputes each term from its element's parameters, sums upper ends, compares with the lower end
   of the absorber's work, and checks the structural rule: every element of the load path that can supply energy
   (a moving mass, a rotor, a drive, a spring, a gravity rise or drop, another actuator) is accounted for by a
   term, and every term names an element of the path.
3. Read as an exclusion: if a mandatory crossing needs a potential rise greater than the initial energy plus all
   allowed external work, no controller succeeds, clairvoyant or not; the certificate then removes a design
   family, and the auditor checks the strict inequality with the bounds on the correct sides.
4. Class and limits: the claim's class is the minimum of `Certified` and its inputs' classes. Its inputs are
   measurements a simulator cannot make: in simulation the claim is exact arithmetic over `Estimated` inputs and
   therefore `Estimated`, and a claim about a physical robot is a Reality claim with a recorded no-claim until
   bench data are imported. The card covers stops and guided contacts, not tip-over or loss of balance, where
   energy is conserved and redistributed; it needs the physical guide.
5. Relies on it: construction C-31, tier 1; the energy-budget safety tier; exclusion of design families by
   energy (a mission-side proof rule); the energy terms of the ledger.
6. Artifacts: `docs/math/cards/T-28.md`; registry row; specimens `energy-budget/accept-T-28-*.frobcert` and
   mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the tray-cart stop budget: 8 kg at 0.4 m/s is 0.64 J; a 20 W drive over a 25 ms
      cutoff adds 0.50 J; the external work of gravity on an 8% slope over the 40 mm stroke is
      8 × 9.81 × 0.040 × 0.08 = 0.25 J; a series spring stores 0.10 J; the total is 1.49 J against an absorber of
      50 N over 40 mm, 2.00 J.
- [ ] The same budget without its gravity term over the stroke is rejected by the checker (the omitted-term
      control), as is a budget with a rotor on the path and no reflected inertia in m_cap.
- [ ] Exclusion accepted: lifting 8 kg over a 12 mm sill needs 8 × 9.81 × 0.012 = 0.94176 J, above a 0.90 J budget
      for every wheel radius; with 0.05 J of approach kinetic energy in the budget the exclusion is rejected, so
      approach energy is a term of the budget and not a footnote.
- [ ] A budget presented as `Certified` over `Estimated` inputs is rejected (FROB-REF-834); presented as
      `Estimated` with the same arithmetic it is accepted.
- [ ] Registry row `checked` with the class printed per budget (normally `Estimated` in simulation), hashes and
      checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t28.rs`: `tray_cart_terms` (0.64, 0.50, 0.251136, 0.10 J; sum 1.491136 ≤ 2.00);
`largest_admissible_cutoff` ((2.00 − 0.64 − 0.25 − 0.10)/20 s = 50.5 ms, or 45.5 ms with a further 0.10 J of
reserve); `missing_gravity_term_rejects`; `missing_reflected_inertia_rejects`; `sill_exclusion`
(0.94176 > 0.90); `approach_energy_removes_exclusion` (0.95 ≥ 0.94176); `outward_rounding_direction` (term upper
ends rounded up, absorber work rounded down); `class_capped_by_inputs`.

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) runs the specimens and mutants; the shield work's script
exercises `frob shield budget` and logs each term, the margin and the certificate hash.

### Known answers and negative controls

KA-213 (energy budget without its gravity term → `reject`). CX-109 (an energy budget computed exactly in
simulation over `Estimated` inputs is `Estimated`). Negative controls as listed. Determinism: exact.

## Considerations

- The margin can be thin (1.49 J against 2.00 J); the report prints it.
- KILL-ENERGY is the owning mechanism's kill gate; the card stands or parks with it.
- The structural check is as complete as the declared load path; the path is derived from the design's port
  graph and embedded in the bundle for review.

## References

§12.3, §12.7, §22.11; C-31; KA-213; CX-109; GATE-SHIELD-01 (shield side).
