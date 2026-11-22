---
key: "CARD-T-18"
title: "Theorem card T-18: the symmetry–event identity ρ(h)·Ξ_e = Ξ_{h·e}·ρ(h) for each generator and each event class"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["CARD-T-17", "CARD-T-10", "N12-21", "N12-22", "N12-06", "SPEC-C-16"]
needs: ["§20: Γ_obs per robot and task with its action on states, contacts and parameters (BD-241)"]
related: ["BD-333", "BD-426", "BD-327", "CARD-T-16"]
gates: []
plan_refs: ["§12.3 (T-18)", "§10 C-16", "§10 C-09", "§25.20", "Appendix D.2 (symmetry–event identity)"]
---
## Background

Symmetry is used to reduce work: a mirrored event need not be differentiated twice, and a symmetric gait can be
analysed on a symmetric subspace. Across an event that is only legitimate when the symmetry's relabeling of
states, contacts and parameters commutes with the event's first-order map. The saltation matrix Ξ_e carries
perturbations across event e. A symmetry h (an element of the observation-preserving group Γ_obs, acting on
states by the differential D h, a signed permutation ρ(h) in suitable coordinates) maps event e to event h·e
(for example, left-foot touchdown to right-foot touchdown).

Statement (T-18). The symmetry–event identity ρ(h)·Ξ_e = Ξ_{h·e}·ρ(h) holds for each generator h of Γ_obs and each
event class of a model. In state-dependent form: D h(x⁺)·Ξ_e(x⁻) = Ξ_{h·e}(h·x⁻)·D h(x⁻). A symmetry h is used
across an event only where this identity holds; it fails by terms proportional to the asymmetry.

Route: polynomial identity per (generator, event class) pair, or exact-rational randomized identity testing
(class `Tested`) where the symbolic identity is over budget. Tier: Frontier. Rejected alternative: assuming that
a symmetry of the smooth dynamics is automatically a symmetry of every event (a difference in a contact
parameter between two sides breaks it).

## Scope

In: statement document, registry row, per-pair certificates or test records, the two-ball fixture with equal and
unequal restitution. Out: computing Γ_obs and its action (the sensing and symmetry beads); the saltation
kernels' own correctness (CARD-T-17); amortizing the identity across a design family (a Moonshot use with its
own kill gate, owned by the mission work); reducing one nominal trajectory into symmetry blocks, which needs a
symmetry that fixes its state, history, parameters and event.

## Technical approach

1. For each generator h and each event class e: obtain Ξ_e and Ξ_{h·e} from the derived saltation kernels
   (certified by T-17), and ρ(h) from the parity rule (polar quantities by P, axial by det(P)·P, joint
   coordinates signed, contact labels permuted). Form ρ(h)·Ξ_e(x⁻) − Ξ_{h·e}(ρ(h)·x⁻)·ρ(h) with denominators
   cleared (the two transversality numbers, which must be equal under h: a sub-identity).
2. Decide entry by entry by normal-form equality (`poly-identity` bundle) when within budget; otherwise run the
   exact-rational randomized test on the branch-free piece, recording degree bound and sample set.
3. Parameters matter: the identity is stated with the model's parameters as symbols constrained by the symmetry
   (equal restitution, equal friction, mirrored geometry). If the model's actual parameters do not satisfy those
   constraints, the pair is reported `not symmetric: <parameter>` and no certificate is issued.
4. Checker: FrankenSymPy's proof kernel at build time; `frob-verify` offline.
5. Class and limits: `Certified` or `Tested` per pair. A passing commuting check transports a local derivative
   to a corresponding event; it does not by itself license block reduction of one trajectory. A sliding impact
   and its mirror can satisfy the identity while a normal-speed perturbation changes tangential speed: that cross
   term stays in the full derivative and is not removed by this card.
6. Relies on it: construction C-16 (a symmetry is used across an event only where the commuting identity
   holds); symmetric controllers under stabilizer intersection; the "symmetry at the instant of contact" test of
   the mission chapter.
7. Artifacts: `docs/math/cards/T-18.md`; registry row with per-model, per-pair records; bundles or test records.

## Acceptance criteria

- [ ] Two bouncing balls with the swap of their labels: with equal restitutions the identity is certified
      (accepted by `frob-verify`); with restitutions r_L ≠ r_R the identity fails by terms proportional to
      r_L − r_R, no certificate is issued, and a forged one is rejected.
- [ ] Negative controls rejected: ρ(h) with an axial sign missing; Ξ_{h·e} replaced by Ξ_e; mismatched
      transversality numbers.
- [ ] For the reference quadruped, each generator of Γ_obs and each touchdown event class has a certificate or a
      `Tested` record with its degree bound and sample set.
- [ ] Registry row prints the class per pair with receipts; pairs that are not symmetric are listed with the
      parameter that breaks them.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t18.rs`: `two_balls_equal_restitution_commutes` (state (y_L, v_L, y_R, v_R), h
swaps L and R; Ξ for the left ball's impact is block diag(Ξ_ball(r), I); ρ(h)·Ξ_L·ρ(h)ᵀ = Ξ_R exactly);
`two_balls_unequal_restitution_fails` (the residual's entries are multiples of r_L − r_R);
`axial_sign_missing_rejects`; `transversality_numbers_must_match`; `not_symmetric_reports_parameter`;
`bookkeeping_strata_count` (four feet with states open, stick and slip under the leg-permutation group C₂ × C₂
give (3⁴ + 3·3²)/4 = 27 strata and 648/4 = 162 one-foot transitions, a related known answer for enumerating the
event classes this card ranges over).

### End-to-end script and logging

`scripts/e2e/lemma_coverage.sh` (bead N12-36) lists the pairs, their routes and verdicts.

### Known answers and negative controls

The two-ball fixture (commutes when the restitutions are equal, violated by terms proportional to r_L − r_R when
they differ: the negative control named with the construction). Determinism: exact.

## Considerations

- Γ_obs, not Γ_mech: a sensor or payload asymmetry removes the generator before this card is consulted.
- The amortized, per-family use is Moonshot; this card is the Frontier test only.
- KILL-LEMMA's fallback turns the symbolic route into the randomized one.

## References

§12.3; C-16, C-09; T-17, T-10; GATE-AUGUR-21 (the mission-side test that uses it).
