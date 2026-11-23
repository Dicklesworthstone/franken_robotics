---
key: "CARD-T-06"
title: "Theorem card T-06: the Cayley discrete Euler–Poincaré step for a free rigid body conserves spatial momentum"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-31"
deps: ["N12-22", "N12-20", "N12-06", "CARD-T-04", "SPEC-C-36"]
needs: []
related: ["BD-379", "BD-384", "CARD-T-02", "CARD-T-13", "N12-21", "BD-087"]
gates: []
plan_refs: ["§12.3 (T-06)", "§10 C-05", "§10 C-36", "§7 NS-11", "Appendix D.2 (trapezoidal variational step)"]
---
## Background

The structure-preserving integration lane uses a discrete Lagrangian on the group. For one rigid body with
inertia 𝕄, potential V and applied wrench u, take L_d(g_k, g_k₊₁) = (h/2)·ξ_kᵀ𝕄ξ_k − (h/2)·(V(g_k) + V(g_k₊₁))
with g_k₊₁ = g_k·cay(h·ξ_k). Its discrete Euler–Poincaré equations are the map (g_k, μ_k) ↦ (g_k₊₁, μ_k₊₁):
1. half kick μ⁺ = μ_k + (h/2)·(f(g_k) + u_k), with f the body-frame wrench of the potential;
2. implicit solve for ξ_k from (Dcay_{−hξ_k})⁻ᵀ·𝕄ξ_k = μ⁺, where Dcay_x is the left-trivialized differential of
   the Cayley map and, for x = (ω, v), (Dcay_x)⁻¹ = [[I + ½·ω̂ + ¼·ω·ωᵀ, 0], [½·v̂ + ¼·ω̂·v̂, I + ½·ω̂]];
3. drift and transport c = cay(h·ξ_k), g_k₊₁ = g_k·c, μ° = Ad_cᵀ·μ⁺;
4. half kick μ_k₊₁ = μ° + (h/2)·(f(g_k₊₁) + u_k₊₁).

Statement (T-06). For a free rigid body (V = 0, u = 0), the spatial momentum Ad_g⁻ᵀ·μ is the same at step k and
step k + 1: Ad_{g_k₊₁}⁻ᵀ·μ_k₊₁ = Ad_{g_k}⁻ᵀ·μ_k. This is the discrete Noether theorem for the step, and it holds
for any ξ that step 2 returns, because step 3 transports the momentum by exactly the adjoint of the drift.

Tier: Frontier, and the budget is a HYPOTHESIS: the route is an identity modulo the update equations (ideal
membership), and whether FrankenSymPy decides it within its caps is measured by the pilot N12-22 before this card
is funded. Why the card exists: the North-Star scenario "a kernel with a certificate" asks that `frob prove kernel
K-cay-step` certify that the update stays on the group and conserves spatial momentum in free flight. Rejected
alternative: a midpoint step followed by a projection, which conserves neither exactly.

## Scope

In: statement document, registry row, the instance certificate on the free-body variational step kernel (or the
documented downgrade), specimen and mutants. Out: the step's implementation (the variational-step oracle and the
articulated variational integrator beads); energy behaviour (measured, not certified); the general theorem on
matrix Lie groups (CARD-T-13); the articulated case (no claim).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the free-body variational step of `frob-dyn` (BD-087) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output (a
certificate bound to the content hash of the free-body step kernel as built) is run inside the implementing bead,
with this card's law tests, as part of that bead's acceptance; the receipt produced there is what moves the
registry row for that output.

## Technical approach

1. Route: ideal membership (class `Certified`). Variables: the pose g (rotation as a quaternion with its norm
   relation, or Cayley parameters), μ⁺, ξ, and the outputs g₊, μ°. Generators of the ideal: the update equations
   of step 3 (c = cay(h·ξ) with denominators cleared, g₊ = g·c, μ° = Ad_cᵀ·μ⁺) and the rotation relations. Claim:
   each component of D^m·(Ad_{g₊}⁻ᵀ·μ° − Ad_g⁻ᵀ·μ⁺) lies in the ideal, for the appropriate power of the Cayley
   denominator D. The mathematical content is one line: Ad_{g·c}⁻ᵀ·Ad_cᵀ = Ad_g⁻ᵀ·Ad_c⁻ᵀ·Ad_cᵀ = Ad_g⁻ᵀ, using card
   T-04. The bundle carries the membership cofactors, so the auditor checks a polynomial identity
   (kind `poly-identity`).
2. If the pilot shows the direct membership exceeds the budget, the fallback inside the card is to certify the
   two factors separately (Ad(g·c) = Ad(g)·Ad(c) on the kernel's adjoint sub-program, and the inverse-transpose
   relation), which are instances of T-04, and to record T-06 as their composition; if that also fails, the card
   is downgraded to an exact-rational randomized identity test, class `Tested` (N12-21).
3. Checker: FrankenSymPy's proof kernel; `frob-verify` offline.
4. Class and limits: `Certified` only on success of the route. Not `Proved`: the general statement is card T-13,
   Moonshot and blocked. Not a claim about energy: evaluating step 2 with the differential at +h·ξ instead of
   −h·ξ leaves momentum exact (step 3 conserves it for any ξ) and destroys the energy bound, so that error is
   detected by energy and reversibility tests, never by this identity. No symplectic or energy claim is made for
   an articulated tree.
5. Relies on it: constructions C-05 and C-36; the free-body step oracle; NS-11.
6. Artifacts: `docs/math/cards/T-06.md`; registry row; the kernel's `.cert` and bundle; specimen and mutants.

## Acceptance criteria

- [ ] The pilot N12-22 (arm A) is on record before work starts; its outcome fixes the route (direct membership,
      composition of T-04 instances, or `Tested`).
- [ ] On the certified branch: `frob-verify` accepts the bundle and rejects a flipped sign in the transport
      (μ° = Ad_c⁻ᵀ·μ⁺ in place of Ad_cᵀ·μ⁺), a dropped term of the adjoint, and a missing rotation relation.
- [ ] On the downgraded branch: the registry row shows `fell-back`, class `Tested`, with the test record; the
      NS-11 output states that momentum conservation is tested, not certified.
- [ ] The registry row carries certificate hash and checker identity when `checked`.
- [ ] The statement document states the hypotheses (free body, exact arithmetic) and what is not claimed.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t06.rs`: `t06_transport_identity_symbolic` (Ad_{g·c}⁻ᵀ·Ad_cᵀ − Ad_g⁻ᵀ reduces to
zero modulo the rotation relations on a small instance); `t06_specimen_accepts`;
`t06_wrong_transport_sign_rejects`; `t06_exact_rational_step` (an asymmetric body with rational inertia, one
step in exact rationals with a rational ξ supplied: spatial momentum before and after are equal exactly);
`t06_plus_h_differential_keeps_momentum` (the wrong-sign differential in step 2 still passes this identity,
documenting why energy tests are needed).

### End-to-end script and logging

`scripts/e2e/lemma_coverage.sh` (bead N12-36) runs the pipeline on the kernel and prints the identity in the
NS-11 demonstration; `scripts/e2e/pilot_lemma_budget.sh` logs the budget figures of arm A.

### Known answers and negative controls

Free asymmetric body known answers (for the step, tested elsewhere): spatial momentum constant to round-off,
energy error bounded and divided by four when h is halved, the map run backwards returns the initial state to
round-off. Negative controls: the mutants above. Determinism: certificate bytes identical across regenerations.

## Considerations

- The budget is a HYPOTHESIS; RISK-07 (expression swell) is the likely failure.
- CX-113: no symplectic claim beyond one rigid body; CX-112: nothing about floating-point output.
- KILL-LEMMA's fallback applies to this card like any kernel identity.

## References

§12.3; C-05, C-36; T-04, T-13; NS-11; BET-11; KILL-LEMMA.
