---
key: "CARD-T-04"
title: "Theorem card T-04: Ad(gh) = Ad(g)·Ad(h), and the group coadjoint is the inverse transpose"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "frob-lemma", "solid", "test"]
parent: "WP-05"
deps: ["BD-027", "N12-06", "SPEC-C-02"]
needs: []
related: ["N12-33", "BD-010", "BD-011", "BD-021", "CARD-T-05", "CARD-T-16"]
gates: ["GATE-LEMMA-00"]
plan_refs: ["§12.3 (T-04)", "§12.6", "§11.2", "§11.3", "§10 C-02", "Appendix D.1 (adjoint, group coadjoint)"]
---
## Background

Frames are objects and a rigid transform `a_from_b` is a morphism from b to a. Twists and wrenches are moved
between frames by the adjoint and its dual. For g = (R, p) = `a_from_b`, a twist ξ = (ω, v) (angular part first)
and a wrench (τ, f) (torque first):

  Ad_g   = [[R, 0], [p̂·R, R]]      (ω, v) ↦ (R·ω, R·v + p × R·ω)        twists, b to a
  Ad_g⁻ᵀ = [[R, p̂·R], [0, R]]      (τ, f) ↦ (R·τ + p × R·f, R·f)        wrenches, b to a

Statement (T-04). For all g = (R_g, p_g) and h = (R_h, p_h) in SE(3), with g·h = (R_g·R_h, R_g·p_h + p_g):
(i) Ad(g·h) = Ad(g)·Ad(h); (ii) the group coadjoint, the map that moves wrenches, is the inverse transpose of the
adjoint, (Ad_g)ᵀ·[[R, p̂·R], [0, R]] = I. Both are polynomial identities in the group coordinates once "R is a
rotation" is supplied (as the relations RᵀR = I, det R = 1, or through a quaternion parameterization).

Why: every recursion of the dynamics (inverse dynamics, composite inertia, articulated inertia) is a chain of
adjoints; a single reversed or mis-transposed adjoint must fail at least three independent fixtures (the
composition law, the pairing, and a planar torque fixture). The coadjoint convention is fixed once: the group
coadjoint is Ad_g⁻ᵀ, as the sibling's SE(3) façade documents (FACT). Tier: Solid. Rejected alternative: 4×4
matrices and 6-vectors whose frames live in comments.

## Scope

In: statement document, registry row, instance certificate on the M0 adjoint kernel, specimen and mutants.
Out: certificate machinery (BD-021, BD-027); frame typing in the API (the frame-typed transform bead); the
algebra-level convention ad*_ξ = ad_ξᵀ (CARD-T-16); pairing invariance (CARD-T-05).

## Technical approach

1. Route: polynomial identity in group coordinates (class `Certified`, bundle kind `poly-identity`). With R given
   by a quaternion q (R = (‖q‖²·I + 2·q₀·q̂ᵥ + 2·q̂ᵥ²)/‖q‖² in terms of the vector part's cross-product matrix),
   both statements become plain polynomial identities after clearing ‖q‖². With R given by entries, they are
   identities modulo RᵀR = I and det R = 1, certified with explicit cofactors. The key sub-identity is
   R·p̂·Rᵀ = (R·p)^ for a rotation R (the cross product is rotation-equivariant; it needs det R = 1).
2. Blocks to certify for (i): top-left R_g·R_h (trivial); bottom-left (R_g·p_h + p_g)^·R_g·R_h =
   p̂_g·R_g·R_h + R_g·p̂_h·R_h. For (ii): Rᵀ·R = I, Rᵀ·p̂·R + (p̂·R)ᵀ·R = 0 (uses p̂ᵀ = −p̂), and the lower blocks.
3. Checker: FrankenSymPy's proof kernel at build time; `frob-verify` offline. The identities are extracted from
   the kernel's straight-line program for the adjoint action.
4. Class and limits: `Certified` at the kernel's content hash; no Lean route is planned (tier Solid). No
   floating-point claim: composition laws are tested to round-off (at most 64 ulp in double precision and 16 ulp
   in single precision for single operations, on 10⁶ cases each, including tiny angles, translations near the
   origin, rotations approaching π from both sides and sign-equivalent quaternions).
5. Relies on it: construction C-02 (frame groupoid and typed spatial algebra); C-04 (dynamics on a tree); the
   three typed frame maps (twist map Ad_g, wrench pushforward Ad_g⁻ᵀ, wrench pullback Ad_gᵀ); card T-06.
6. Artifacts: `docs/math/cards/T-04.md`; registry row; the adjoint kernel's `.cert` and bundle; specimen
   `poly-identity/accept-T-04.frobcert` with mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the T-04 bundle for the M0 kernel; GATE-LEMMA-00 (TARGET, M0) records it certified
      and replaying offline.
- [ ] Negative controls rejected: a reversed adjoint (Ad(g·h) = Ad(h)·Ad(g)); a coadjoint written as the
      transpose without the inverse; one changed coefficient; a missing rotation side relation.
- [ ] Registry row `checked`, class `Certified`, with certificate hash and checker identity.
- [ ] The statement document repeats the block matrices and the direction (b to a) of both maps.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t04.rs`: `t04_composition_blocks_expand`; `t04_coadjoint_inverse_transpose`;
`t04_cross_product_equivariance_needs_det_one` (with a reflection in place of R the sub-identity fails by a
sign); `t04_specimen_accepts`; `t04_reversed_adjoint_rejects`; `t04_transpose_without_inverse_rejects`;
`t04_wrench_pushforward_spot_check` (R = I, p = (1, 0, 0) m, wrench (0, (0, 0, 1) N): the pushforward gives
torque p × f = (0, −1, 0) N·m and the same force; moving it with the twist map gives zero torque).

### End-to-end script and logging

`scripts/e2e/lemma_kernel_cert.sh` (bead N12-33) certifies, replays offline and logs verdicts and hashes.

### Known answers and negative controls

KA-220. CX-099 (a wrench does not change frame by the same matrix as a twist: the moment arm is lost). The
mutants are the negative controls. Determinism: certificate bytes identical across regenerations and machines.

## Considerations

- A convention error that is self-consistent (swapping the roles of pushforward and pullback everywhere) is not
  caught by the homomorphism law alone; the spot check with a moment arm and the planar torque fixture catch it.
- CX-112 applies: nothing here is about floating-point output.

## References

§12.3, §12.6, §11.2, §11.3; C-02, C-04; GATE-LEMMA-00; KA-220; CX-099.
