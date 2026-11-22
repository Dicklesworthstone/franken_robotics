---
key: "CARD-T-13"
title: "Theorem card T-13: discrete Noether for left-invariant discrete Lagrangians on a matrix Lie group (parked)"
type: "task"
priority: 4
milestone: "M8"
labels: ["m8", "lemma", "frob-lemma", "moonshot"]
parent: "EPIC-M8"
deps: ["BD-381", "CARD-T-06", "FD-FLN-01", "N12-06"]
needs: []
related: ["BD-384", "CARD-T-01", "N12-23"]
gates: []
plan_refs: ["§12.3 (T-13)", "§12.1", "§12.5", "§10 C-05", "§10 C-36", "Appendix C (deferred: general Lean theorems on Lie groups)", "Appendix F (FD-FLN-01)"]
---
## Background

The structure-preserving integration lane rests on a classical fact: a discrete Lagrangian on a Lie group that is
invariant under the left action of the group has discrete Euler–Poincaré equations that conserve exactly the
discrete momentum map of that symmetry. For the Cayley step of one rigid body, the instance of this fact is card
T-06 (spatial momentum is conserved by the step), certified as a polynomial identity. The general theorem, for
every matrix Lie group and every left-invariant discrete Lagrangian, would let the project say `Proved` of the
whole family of such integrators.

Statement (T-13). Discrete Noether for left-invariant discrete Lagrangians on a matrix Lie group: let G be a
matrix Lie group with Lie algebra 𝔤, let L_d : G × G → ℝ satisfy L_d(h·g₀, h·g₁) = L_d(g₀, g₁) for all h ∈ G, and
let (g_k) satisfy the discrete Euler–Lagrange equations of L_d. Then the discrete momentum map
J(g_k, g_k₊₁) ∈ 𝔤* (the spatial momentum Ad_{g_k}⁻ᵀ·μ_k in the body-momentum form) is constant along the sequence.

Route: Lean, general. Tier: Moonshot, and deferred: the expected-value register lists "general Lean theorems on
Lie groups (T-13)" as deferred, with no pilot possible while the modules are blocked, and not scheduled. The
blocking fact (FrankenLean at its pinned commit): `Geometry.Manifold.Algebra.LieGroup` and
`Analysis.Normed.Algebra.MatrixExponential` are not accepted, and the statement needs a Lie group with its
smooth structure and derivatives of L_d. The project does not claim that any Mathlib result about Lie groups or
matrix exponentials has been re-checked. This bead records the card, its parked status and its reopening
condition; it does not fund proof work.

## Scope

In: the statement document; the registry row with status `parked`; the reopening rule; a closure probe to be
re-run when the upstream item lands. Out: any proof; the instance (CARD-T-06); the integrator itself.

## Technical approach

1. Registry row: route `lean-capsule`, class `Proved`, status `parked`, reason "closure needs
   Geometry.Manifold.Algebra.LieGroup and Analysis.Normed.Algebra.MatrixExponential, not accepted by FrankenLean
   at the pinned commit", unblock condition "upstream item FD-FLN-01 lands (unblock the Mathlib roots for matrix
   exponentials, Lie groups, analytic quaternions and representation theory, P3) and the probe below is clean".
2. Algebraic alternative, recorded so that it is not rediscovered: for the Cayley retraction specifically, the
   conservation statement is rational and needs no manifold structure. A purely algebraic generalization of T-06
   ("for any group-valued rational update g₊ = g·c and momentum transport μ° = Ad_cᵀ·μ, the quantity Ad_g⁻ᵀ·μ is
   invariant") is provable over a commutative ring from Ad(g·c) = Ad(g)·Ad(c) alone and could be sealed today as
   a Frontier theorem; it is the transport half of Noether, not the variational half, and it is carried as an
   optional item of this bead only if the Lean lane has spare capacity after the first cards.
3. Closure probe: a Lean file that states the theorem's hypotheses (a Lie group instance on a matrix group, a
   smooth left-invariant function on the product) with a placeholder, used only to compute the import closure
   and its intersection with the blocked list; re-run at every bump of FrankenLean's pin; the result is written
   to the proof-tool registry.
4. Class and limits: nothing is claimed. Until sealed, the only momentum statement the project makes is the
   instance T-06 (`Certified` or `Tested`) and the measured conservation of the integrator to round-off.
5. Would rely on it: constructions C-05 and C-36; the articulated variational integrator (for which no
   symplectic or energy claim is made in any case, because a configuration-dependent mass matrix breaks the
   kick–drift splitting).
6. Artifacts: `docs/math/cards/T-13.md`; registry row; `lean/Frob/Parked/T13_probe.lean` (never sealed).

## Acceptance criteria

- [ ] The registry row exists with status `parked`, the reason and the unblock condition; `frob prove list`
      prints `parked: blocked Mathlib modules` and no class.
- [ ] The closure probe runs on the Linux host and reports the blocked modules it meets; with the current pin the
      report is non-empty (that is the known answer of the probe).
- [ ] No output of the product claims discrete Noether in general; a test scans reports for the card's
      identifier appearing with a class.
- [ ] Negative control of the parking rule: an attempt to mark the route `checked` without a capsule receipt
      fails the registry check.
- [ ] If the optional algebraic transport theorem is sealed, it is registered under T-06's row as its general
      form, not as T-13.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t13.rs`: `t13_row_is_parked`; `t13_prints_no_class`;
`t13_checked_without_receipt_rejected`. `lean/tests/parked/`: `t13_probe_reports_blocked_modules`;
`probe_directory_cannot_be_sealed`.

### End-to-end script and logging

`scripts/e2e/pilot_lean_closure.sh` (shared with N12-23) runs the probe as a step `t13-probe` and logs the
blocked intersection; no assertion on its emptiness until FD-FLN-01 lands.

### Known answers and negative controls

Known answer of the probe at the current pin: at least the two named modules are in the closure and are not
accepted. The instance known answers belong to T-06 (spatial momentum constant to round-off for a free
asymmetric body). Negative control: the hand-edited registry row. Determinism: not applicable.

## Considerations

- Moonshot items live behind a feature flag and never sit on the critical path; this one has no schedule.
- Reopening needs both the upstream change and a measured closure (OPEN-LEMMA-1 style) on the Linux host.
- Kill rule named in advance: if FD-FLN-01 is declined upstream, the card stays parked permanently and the
  instance card remains the project's statement.

## References

§12.1, §12.3, §12.5; C-05, C-36; T-06; FD-FLN-01; the deferred-ideas register.
