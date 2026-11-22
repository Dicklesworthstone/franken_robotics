---
key: "CARD-T-34"
title: "Theorem card T-34: an α-certified root, with Smale's theorem as a named assumption"
type: "task"
priority: 4
milestone: "M8"
labels: ["m8", "lemma", "frob-cert", "frob-synth", "moonshot"]
parent: "WP-80"
deps: ["N12-19", "N12-06", "SPEC-C-34"]
needs: ["§25.21.4: the continuation lane of linkage synthesis and its pilot (the Stewart–Gough forward problem on the CPU), owned by the synthesis work (BD-437)"]
related: ["BD-437", "BD-361", "N12-16"]
gates: []
plan_refs: ["§12.3 (T-34)", "§10 C-34 (lane 2)", "§25.21.4", "Appendix D.6 (Smale's α-test)", "Appendix C (numerical continuation, admitted [M])"]
---
## Background

Linkage synthesis has two lanes. The exact bounded lane solves small polynomial systems inside the caps of the
exact real solver and certifies circuits and branches by interval methods. The continuation lane (Moonshot, on
the CPU in double precision) solves a generic member of a mechanism family once, by monodromy with a trace test,
and tracks a parameter homotopy H(z, t) = F(z; (1 − t)·p₀ + t·p₁) from the stored solutions to a new task with a
predictor–corrector and endgames. Path tracking produces approximate roots in floating point; each must then be
certified, or it is only a number. The certificate is Smale's α-test in exact rational arithmetic.

Statement (T-34). An α-certified root: for the given rational point z and polynomial system f, the computed bound
on α(f, z) is below (13 − 3√17)/4 ≈ 0.157671. Here α = β·γ with β(f, z) = ‖Df(z)⁻¹·f(z)‖ (the length of the Newton
step) and γ(f, z) = sup_{k ≥ 2} ‖Df(z)⁻¹·Dᵏf(z)/k!‖^(1/(k − 1)).

By Smale's theorem the bound proves that Newton's iteration from z converges quadratically to a root within twice
the Newton step. Distinctness of two certified roots follows from disjoint balls, non-reality from disjointness
with the conjugate ball, and reality from the robust test α(f, z) < 0.03 with ‖z − Re z‖ < 1/(20·γ(f, z)), which
shows that z and its conjugate converge to one root. Route: exact rational arithmetic; Smale's theorem is a
named assumption of the bundle, not a re-proved result. Tier: Moonshot. Rejected alternative: accepting a small
floating-point residual as proof of a root (a small residual near a singular Jacobian proves nothing).

## Scope

In: statement document, registry row (status `parked` until the lane is funded), the `alpha-root` certificates
bound to this card on the funded branch, fixtures and mutants. Out: monodromy, path tracking and endgames (the
continuation epic); the auditor's checker (N12-19); completeness of the solution set.

## Technical approach

1. Certificate (kind `alpha-root`): the system f with rational coefficients, the point z with rational real and
   imaginary parts (the tracker's double-precision output read as exact rationals), a rational approximate
   inverse B of Df(z), and rational bounds β̄ ≥ β and γ̄ ≥ γ.
2. The auditor recomputes in exact rationals with squared norms (no square roots): f(z) and Df(z); the defect
   I − B·Df(z) and a bound on ‖Df(z)⁻¹‖ from it; β̄; for each k from 2 to the degree, that γ̄^(k−1) bounds the k-th
   term; then ᾱ = β̄·γ̄ and the threshold test, decided as 13 − 4·ᾱ > 0 and (13 − 4·ᾱ)² > 153 (which is
   ᾱ < (13 − 3√17)/4 without evaluating √17).
3. The bundle's assumptions list `smale-alpha-theorem` explicitly, and the report prints it in the trusted-base
   block: the class of the root's existence is `Certified` relative to that named theorem.
4. Reality and distinctness are separate small certificates of the same kind: the robust test with its two
   inequalities; disjointness of two balls of radius 2·β̄ around two points.
5. Completeness is not part of the card. A synthesis result is `Complete` only when the certified count equals
   an independent generic count with no lost path, and otherwise `Partial(certified, lost)`; that comparison is
   the producer's bookkeeping, reported with its source for the generic count.
6. Class and limits: `Certified` (conditional on the named theorem) for existence of a nearby nonsingular root;
   nothing about completeness, nothing about the mechanism's circuits or branches (those are the exact lane's
   certificates), and no GPU lane (path tracking needs double precision).
7. Artifacts: `docs/math/cards/T-34.md`; registry row; specimens `alpha-root/accept-T-34-*.frobcert` and mutants
   once funded.

## Acceptance criteria

- [ ] While the continuation lane is unfunded the registry row is `parked` and prints no class; the proof kind is
      `not-implemented` and no `alpha-root` bundle can be accepted.
- [ ] On the funded branch `frob-verify` accepts the hand specimen f(x) = x² − 2 at z = 17/12 (f(z) = 1/144,
      Df(z) = 17/6, β = 1/408, γ = 6/17, α = 1/1156) and rejects z = 1 (β = ½, γ = ½, α = ¼ ≥ 0.157671), a γ̄ below
      the true value, and a bundle without the named assumption.
- [ ] On the Stewart–Gough forward problem (generic instance) each of the 40 solutions has an accepted
      certificate, the balls are pairwise disjoint, and the result is `Complete` only if 40 are certified with no
      lost path; otherwise it is `Partial` with the counts.
- [ ] Registry row, when `checked`: class `Certified` with the assumption named, hashes and checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t34.rs`: `sqrt2_newton_point_certified`; `threshold_without_square_root`
((13 − 4·ᾱ)² > 153 ⇔ ᾱ < (13 − 3√17)/4 for ᾱ < 13/4); `point_one_rejected`; `assumption_required`;
`reality_robust_test` (z = 17/12 is real, α = 1/1156 < 3/100, and ‖z − Re z‖ = 0 < 1/(20·γ) = 17/120);
`disjoint_balls` (the certified points near +√2 and −√2 have balls of radius 1/204 that do not meet);
`parked_row_prints_no_class`.

### End-to-end script and logging

The continuation epic's script logs, per tracked path, the residual, ᾱ against the threshold, the ball radius,
and the certified and lost counts; `lemma_auditor_mutations.sh` (bead N12-34) asserts `unsupported` for the kind
until it is enabled.

### Known answers and negative controls

The hand specimen. Known generic counts the lane compares against (literature facts, to be re-read from the
primary sources before a gate cites them): at most 4 Burmester point pairs for five poses, 16 solutions of
general 6R inverse kinematics, 40 of the Stewart–Gough forward problem, 4,326 four-bars through nine points.
Negative controls as listed. Determinism: exact rational checking; the tracker's own output may differ across
platforms, the certificates do not depend on how the point was found.

## Considerations

- Kill gate of the owning lane (more than 0.1% of paths lost, or a certified count below 40 on a generic
  instance): fallback to the exact lane with multistart recovery on the CPU, labelled `Partial`; this card then
  stays parked.
- Certificates are checked by an exact rational checker, never inside a proof kernel.
- Tight γ bounds for high-degree systems are expensive; loose bounds only make the test conservative.

## References

§12.3; C-34; §25.21.4; KILL-SYNTH-CONT; the α-test row of the mathematics compendium.
