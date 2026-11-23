---
key: "CARD-T-17"
title: "Theorem card T-17: a derived saltation kernel equals the saltation formula for its flow, guard and reset"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["N12-22", "N12-20", "N12-06", "SPEC-C-16"]
needs: []
related: ["CARD-T-18", "BD-334", "BD-082", "N12-REF", "BD-379", "BD-333"]
gates: []
plan_refs: ["§12.3 (T-17)", "§10 C-16", "§12.7 (FROB-REF-263)", "Appendix D.2 (saltation matrix)"]
---
## Background

A gait is a hybrid orbit: smooth flows ẋ = f_m(x) in modes, with guards g_e and resets R_e on the edges between
modes (touchdown and liftoff, stick to slip, joint-limit impact). First-order perturbations cross an isolated
event with transverse crossing through the saltation matrix, not through the reset Jacobian alone:

  Ξ_e = D_x R + (f⁺ − D_x R·f⁻ − D_t R)·(D_x g)/(D_t g + D_x g·f⁻)

where f⁻ and f⁺ are the flows just before and just after the event, D_x g is a row vector, and the denominator is
the transversality number; the event time moves by δt = −(D_x g·δx)/(D_t g + D_x g·f⁻). Return maps, Floquet
multipliers and orbit gradients of a periodic gait are products of flow tangent maps and saltation matrices, so
an error in a saltation kernel silently corrupts every stability figure and every design gradient built on it.

Statement (T-17). A derived saltation kernel equals D_xR + (f⁺ − D_xR·f⁻ − D_tR)·D_xg/(D_tg + D_xg·f⁻) for its flow,
guard and reset, after clearing the denominator: for each event class of a model, the kernel's output matrix K
satisfies (D_tg + D_xg·f⁻)·K = (D_tg + D_xg·f⁻)·D_xR + (f⁺ − D_xR·f⁻ − D_tR)·D_xg as an identity in the state and
parameters. Hypothesis of use: an isolated event with transverse crossing (the transversality number is nonzero).

Route: polynomial identity per event class. Tier: Frontier. Rejected alternative: using the reset Jacobian as
the event derivative, which gets a sign wrong, omits the coupling to the flow, and for an elastic bounce has
determinant −1 where the true map preserves area with determinant +1.

## Scope

In: statement document, registry row, per-event-class certificates on a model's saltation kernels, specimen and
mutants. Out: deriving the kernels and the return map (the gait-orbit beads); the refusal near grazing and at
ambiguous event order (the event-derivative code raises FROB-REF-263); the symmetry–event identity
(CARD-T-18); validation of every `saltation` derivative by perturbation and event re-location (a test).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the hybrid gait-orbit code that derives saltation kernels (BD-333) is `related`, not a dependency (a card that
waited for its implementer would close a loop, because the implementer's own acceptance runs the card's law
tests). What the card does depend on is in `deps`: the card registry, its specification, its pilot where the route
is Frontier, and Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens
and mutants, and every acceptance line that its own fixtures decide. An acceptance line that can only be observed
on live output (the per-event-class certificates on a model's saltation kernels, the planar-biped event class
included) is run inside the implementing bead, with this card's law tests, as part of that bead's acceptance; the
receipt produced there is what moves the registry row for that output.

## Technical approach

1. For each event class (an edge of the hybrid automaton keyed by stable geometry and clock identity), take the
   symbolic flow before and after, the guard and the reset from the model compiler, in the rational form the
   kernels use (unit-complex joint coordinates, Cayley updates). Differentiate symbolically to get D_x R, D_t R,
   D_x g, D_t g.
2. Symbolically execute the derived saltation kernel to get K; form the cleared identity above; certify entry by
   entry by normal-form equality (bundle kind `poly-identity`), with the nonvanishing of the transversality
   number recorded as a side condition (it is exactly the condition under which the query is not refused).
3. Checker: FrankenSymPy's proof kernel at build time; `frob-verify` offline. Budget measured by the pilot
   (arm D of N12-22); event classes whose expressions exceed the caps fall to the exact-rational randomized test,
   class `Tested`.
4. Class and limits: `Certified` per event class on the kernel at its content hash. On the throughput tiers
   contact is compliant, the step map is continuous and piecewise differentiable, and the monodromy is a product
   of step Jacobians with no saltation matrix: this card concerns the referee's event-located derivatives only.
   It certifies the formula's implementation, not that the event was isolated and transverse in a given run.
5. Relies on it: construction C-16; return maps and Floquet multipliers; labeled derivatives (label `saltation`
   carries event identities, guard margins and transversality numbers); orbit gradients by the adjoint equation
   (I − D_x P)ᵀ·λ = ∇_x J.
6. Artifacts: `docs/math/cards/T-17.md`; registry row with per-model records; certificates; specimen
   `poly-identity/accept-T-17-bouncing-ball.frobcert` with mutants.

## Acceptance criteria

- [ ] The pilot N12-22 (arm D) is on record before work starts.
- [ ] The bouncing-ball specimen is accepted: state (y, v), flow (v, −g), guard y = 0, reset (y, −r·v), impact
      speed v⁻ < 0; the formula gives Ξ = [[−r, 0], [−(1 + r)·g/v⁻, −r]], determinant r²; the certificate is the
      identity v⁻·K = v⁻·D_xR + (f⁺ − D_xR·f⁻)·D_xg.
- [ ] Negative controls rejected: the reset Jacobian diag(1, −r) offered as the kernel; a kernel with f⁺ and f⁻
      exchanged; one changed coefficient; a certificate without the transversality side condition.
- [ ] For one planar-biped event class the identity is certified or recorded as `Tested` with its test record.
- [ ] Registry row with class per event class, hashes and checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t17.rs`: `bouncing_ball_saltation_matrix` (D_xR = diag(1, −r), D_xg = (1, 0),
f⁻ = (v⁻, −g), f⁺ = (−r·v⁻, −g): f⁺ − D_xR·f⁻ = (−(1 + r)·v⁻, −(1 + r)·g); divided by D_xg·f⁻ = v⁻ and multiplied by
(1, 0) it adds [[−(1 + r), 0], [−(1 + r)·g/v⁻, 0]] to diag(1, −r), giving the stated Ξ); `determinant_is_r_squared`;
`reset_jacobian_control_rejects` (for r = 1 its determinant is −1); `swapped_flows_rejects`;
`transversality_side_condition_required` (FROB-REF-829 when missing); `event_time_shift_formula`
(δt = −δy/v⁻).

### End-to-end script and logging

`scripts/e2e/lemma_coverage.sh` (bead N12-36) covers the saltation kernels; `pilot_lemma_budget.sh` logs arm D.

### Known answers and negative controls

The bouncing ball; the rimless wheel as a related known answer of the return map (with spoke half-angle α on a
slope γ the map on the post-impact rate is ω₊² ↦ cos²(2α)·(ω₊² + (4g/ℓ)·sin α·sin γ), multiplier cos²(2α), exactly
½ for eight spokes). Grazing control: a ball whose apex touches a ceiling returns `refused` (FROB-REF-263), not a
matrix. CX-043. Determinism: exact.

## Considerations

- A non-normal monodromy can amplify a perturbation before it decays; multipliers are local. None of that is
  this card's claim.
- Simultaneous events tied by a symmetry are resolved by one coupled solve; their derivative is ordinary on the
  symmetric subspace and `generalized` off it, and has no saltation certificate.

## References

§12.3; C-16; FROB-REF-263; T-18; KA of the bouncing ball and rimless wheel.
