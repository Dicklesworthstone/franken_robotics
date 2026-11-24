---
key: "CARD-T-01"
title: "Theorem card T-01: the Cayley map of a skew-symmetric matrix is a rotation"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "frob-lemma", "solid", "test"]
parent: "WP-05"
deps: ["BD-027", "N12-06", "SPEC-C-05"]
needs: []
related: ["CARD-T-03", "CARD-T-13", "GB-01", "N12-20", "N12-28", "N12-33", "N12-39", "N12-41"]
gates: ["GATE-LEMMA-00"]
plan_refs: ["§12.3 (T-01)", "§12.6", "§12.7", "§10 C-05", "Appendix D.1 (Cayley map)"]
---
## Background

Time stepping stays on the group: an update is g₊ = g·τ(h·ξ) for a retraction τ from the algebra to the group.
The retraction used on the batched tiers is the Cayley map cay(x) = (I − x/2)⁻¹·(I + x/2), which is rational (it
uses only +, −, ×, ÷), agrees with the exponential to second order, and lands exactly on the group. That last
property is this card.

Statement (T-01). For every skew-symmetric matrix Ω (Ωᵀ = −Ω) for which I − Ω/2 is invertible,
cay(Ω)ᵀ·cay(Ω) = I and det cay(Ω) = 1. Over the reals I − Ω/2 is invertible for every skew-symmetric Ω. On SO(3),
with Ω = ω̂ the cross-product matrix of ω ∈ ℝ³, the closed form is R = I + 4/(4 + ‖ω‖²)·(ω̂ + ω̂²/2).

Why it matters: in exact arithmetic the update is group-valued, so the only thing left to remove in floating
point is round-off (one normalization per step by a correctly rounded square root, a correction of order 10⁻⁷ in
single precision, instead of the order-h² projection an additive update needs). A rational update is an object a
polynomial identity checker can certify. Tier: Solid for the instance route, Frontier for the general Lean route.
Alternatives rejected for the update itself: adding a quaternion increment and renormalizing; Runge–Kutta on
Euler angles.

## Scope

In: the card's statement document, its registry row, the instance certificate on the M0 kernel `K-cay-step`
through the `poly-identity` kind, its accepted specimen and mutants. Out: producing the certificate machinery
(BD-021, BD-027); the general Lean theorem (N12-39); the SE(3) statement (CARD-T-02); floating-point behaviour
(the group-membership residual tests of the algebra crate).

## Technical approach

1. Instance route (class `Certified`). Write D = 4 + ω₁² + ω₂² + ω₃² and N = D·I + 4·ω̂ + 2·ω̂², so that R = N/D.
   The certified statements are polynomial identities in (ω₁, ω₂, ω₃) with rational coefficients, obtained by
   clearing the denominator: NᵀN = D²·I (nine entries, degree 4) and det N = D³ (degree 6). The nonvanishing
   condition D ≠ 0 is recorded as a side condition of the bundle and holds over the reals because D ≥ 4. The
   identities are extracted from the kernel's own straight-line program by symbolic execution, not from this
   formula, so the certificate is about the kernel that ships. In the kernel's symbols (BD-019) ω here is the
   dimensionless increment a = h·ω, N = 4·R(c) for the increment quaternion c = (1, a/2) the program forms, and
   D = 4·d for its output d = 1 + a·a/4; the extracted statements are R(c)ᵀ·R(c) = d²·I and det R(c) = d³.
2. Checker: FrankenSymPy's proof kernel at build time (claim `AlgebraicIdentity`, rule `PolynomialNormalForm`);
   the offline auditor `frob-verify` re-checks the bundle by expanding both sides to normal form over ℚ. The
   bundle carries the kernel content hash, so it certifies `K-cay-step` at one `KernelId`.
3. General route (class `Proved`, Frontier): the Lean theorem for all dimensions over a commutative ring with 2
   invertible, proved by commuting I + Ω/2 with I − Ω/2 and using det of a transpose; carried by N12-39.
4. Class claimed and why no stronger: `Certified` now, because the checker decides the instance over ℚ. Not
   `Proved` until the Lean capsule is sealed. Nothing is claimed about floating-point output: certificates are
   over ℚ with square roots and rounding is not modeled.
5. Relies on it: construction C-05 (time stepping stays on the group); the kernels `K-cay-step` and the
   articulated Cayley step; card T-02; the North-Star scenario "a kernel with a certificate" (the update stays on
   the group); GATE-MARROW-01, which cites this gate.
6. Artifacts: `docs/math/cards/T-01.md`; the row `T-01` in `registries/theorems.toml`; the T-01 entry of
   `K-cay-step.cert` (BD-021) and its `.frobcert` bundle; specimens `crates/frob-cert/tests/specimens/poly-identity/accept-T-01.frobcert` and three
   mutants.

## Acceptance criteria

- [ ] The offline auditor accepts the T-01 bundle for the M0 Cayley kernel without trusting the producer:
      `frob-verify` built from its crate alone returns `accept`.
- [ ] GATE-LEMMA-00 (TARGET, M0): T-01 (with T-03 and T-04) certified by FrankenSymPy for the M0 kernels; the
      certificate replays offline.
- [ ] Negative controls rejected: one changed coefficient in the payload (the factor 2 of ω̂² written as 3); a
      mutated statement (det N = −D³); a certificate whose embedded kernel hash is another kernel's.
- [ ] The registry row shows the instance route `checked`, class `Certified`, with certificate hash and checker
      identity; the Lean route stays `hypothesis` until sealed.
- [ ] The statement document states the side condition D ≠ 0 and the floating-point non-claim.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t01.rs`: `t01_normal_form_identity` (NᵀN − D²·I expands to the zero polynomial);
`t01_det_identity` (det N − D³ = 0); `t01_specimen_accepts`; `t01_changed_coefficient_rejects`;
`t01_wrong_kernel_hash_not_accepted`; `t01_mutated_statement_rejects` (the bundle's statement det N = −D³:
`reject`, the normal form of det N + D³ is 2·D³, not zero); `t01_numeric_spot_check` (exact rationals: ω = (1, 2, 2)
gives D = 13 and R = N/13 with RᵀR = I and det R = 1 exactly).

### End-to-end script and logging

`scripts/e2e/lemma_kernel_cert.sh` (bead N12-33) certifies, replays offline and logs the verdicts, the hashes and
the rule of each rejected mutant. Its NDJSON lines for this card (STD-E2E helper) carry `ts`, `script`, `bead`
("CARD-T-01"), `step` (`T-01/<specimen-or-mutant>`), `event`, the exact `frob prove kernel K-cay-step --json` and
`frob-verify` command lines, exit codes, wall time, the envelope's `evidence` block (claim Proof, class
`Certified`, `KernelId`, certificate hash, checker identity), the expected and observed verdict, and on a mismatch
the first failing rule and the bundle path to reproduce.

### Known answers and negative controls

KA-211 (T-01 for the M0 Cayley kernel: `accept`; one changed coefficient gives `reject`), KA-220. Related
floating-point known answer (not this card's claim): the group-membership residual stays within four units in the
last place over 10⁷ steps with per-step normalization. Cayley angle: a Cayley step rotates by 2·atan(‖ω‖·h/2),
so use is bounded by ‖ω‖·h ≤ 0.2 per substep and refused beyond. Determinism: certificate bytes identical across
regenerations and machines.

## Considerations

- CX-112: the certificate does not show that the kernel's floating-point output satisfies the identity.
- If the kernel changes, its hash changes and the certificate must be regenerated; a stale certificate does not
  count.
- KILL-LEMMA fallback would restate this identity as an exact-rational randomized test, class `Tested`.

## References

§12.3, §12.6, §12.7; C-05; GATE-LEMMA-00; KA-211, KA-220; NS-11.
