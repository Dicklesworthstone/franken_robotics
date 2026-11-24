---
key: "CARD-T-05"
title: "Theorem card T-05: the power pairing of a wrench and a twist is invariant under a change of frame"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-lemma", "solid", "test"]
parent: "WP-05"
deps: ["BD-027", "CARD-T-04", "N12-06", "SPEC-C-02"]
needs: []
related: ["CARD-T-15", "CARD-T-16", "CARD-T-29", "N12-41"]
gates: []
plan_refs: ["§12.3 (T-05)", "§11.1 (wrench)", "§11.3", "§10 C-02", "Appendix D.1 (power pairing, three typed frame maps)"]
---
## Background

A twist ξ = (ω, v) is a vector typed by a frame; a wrench w = (τ, f) is a covector typed by a frame; their
pairing ⟨w, ξ⟩ = τ·ω + f·v is power in watts. Power is a physical scalar, so it cannot depend on the frame in
which the pair is expressed. The energy ledger of the simulator is the port structure read at run time, and it
closes only if this holds at every joint and contact.

Statement (T-05). For every rigid transform g = (R, p) and every wrench w and twist ξ expressed in the same
frame b, ⟨Ad_g⁻ᵀ·w, Ad_g·ξ⟩ = ⟨w, ξ⟩, where Ad_g·(ω, v) = (R·ω, R·v + p × R·ω) moves twists and
Ad_g⁻ᵀ·(τ, f) = (R·τ + p × R·f, R·f) moves wrenches. Hypothesis: twist and wrench are expressed in the same frame
before and after, and R is a rotation.

Expanded: (R·τ + p × R·f)·(R·ω) + (R·f)·(R·v + p × R·ω) = τ·ω + f·v, because (p × R·f)·(R·ω) + (R·f)·(p × R·ω) = 0
(the scalar triple product is antisymmetric) and R preserves dot products. Tier: Solid. The typed design it
supports: three maps between frames, each its own type (the twist map Ad_g, the wrench pushforward Ad_g⁻ᵀ, the
wrench pullback Ad_gᵀ, which is the inverse of the pushforward; a Jacobian transpose is a pullback), and no
function accepts one where another is expected. Rejected alternative: one "transform" function for 6-vectors.

## Scope

In: statement document, registry row, instance certificate on the frame-map kernels, specimen and mutants.
Out: the typed API (the frame-typed transform bead); the certificate machinery; the port energy balance check
(the actuator and ledger beads); the reflection case (the parity map of CARD-T-16).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the frame-typed transforms and typed maps of `frob-lie` (BD-047) is `related`, not a dependency (a card that
waited for its implementer would close a loop, because the implementer's own acceptance runs the card's law
tests). What the card does depend on is in `deps`: the card registry, its specification, its pilot where the route
is Frontier, and Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens
and mutants, and every acceptance line that its own fixtures decide. An acceptance line that can only be observed
on live output (a certificate bound to the content hashes of the frame-map kernels as built) is run inside the
implementing bead, with this card's law tests, as part of that bead's acceptance; the receipt produced there is
what moves the registry row for that output.

## Technical approach

1. Route: polynomial identity (class `Certified`, kind `poly-identity`) in the coordinates of g, w and ξ, modulo
   the rotation relations for R (or with R parameterized by a quaternion, cleared of ‖q‖²). The certificate is
   extracted from the kernels that implement the two maps: push the wrench, map the twist, pair, and compare with
   the pairing of the inputs.
2. Checker: FrankenSymPy's proof kernel at build time; `frob-verify` offline.
3. Class and limits: `Certified` at the kernels' content hashes; tier Solid, no Lean route planned. In floating
   point the invariance holds to round-off and is a G0 property test, not this certificate.
4. Relies on it: construction C-02; the energy ledger and the port structure (C-12, C-38); the joint torque map
   (a Jacobian transpose as a pullback); the planar two-link fixture below.
5. Artifacts: `docs/math/cards/T-05.md`; registry row; the kernels' `.cert` files and bundle; specimen
   `poly-identity/accept-T-05.frobcert` with mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the T-05 bundle and rejects: the wrench moved by the twist map (pairing changes by
      the lost moment-arm term); one changed coefficient; a missing rotation side relation.
- [ ] Registry row `checked`, class `Certified`, with certificate hash and checker identity.
- [ ] The planar two-link fixture reproduces by both routes of computing power.
- [ ] The statement document states the same-frame hypothesis explicitly.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t05.rs`: `t05_pairing_identity_expands`; `t05_specimen_accepts`;
`t05_twist_map_on_wrench_rejects`; `t05_changed_coefficient_rejects`; `t05_missing_rotation_relation_rejects`
(FROB-REF-829); `t05_planar_2r_power` (links 0.4 m and 0.3 m, joint angles 0 and π/2, axes
along +z: tool point (0.4, 0.3) m, point Jacobian J = [[−0.3, −0.3], [0.4, 0.0]] m/rad; tool force (10, 0) N gives
joint torques Jᵀ·f = (−3, −3) N·m; joint rates (0.2, −0.1) rad/s give tool velocity (−0.03, 0.08) m/s; power is
−0.3 W by f·v and by τ·q̇, exactly in rationals); `t05_moment_arm_spot_check` (R = I, p = (1, 0, 0) m, wrench
(0, (0, 0, 1) N), twist ((0, 1, 0) rad/s, 0): pairing 0 W in frame b; after the correct maps the pairing is
(0, −1, 0)·(0, 1, 0) + (0, 0, 1)·(p × ω) = −1 + 1 = 0 W; with the wrench moved by the twist map it is 1 W).

### End-to-end script and logging

At M1, when the card closes, the T-05 specimen and its mutants join the specimen set that
`scripts/e2e/lemma_kernel_cert.sh` (bead N12-33) replays offline with `frob-verify`; at M5
`scripts/e2e/lemma_coverage.sh` (bead N12-36) covers the frame-map kernels as built and logs this identity's
status. Both log, per specimen and mutant, NDJSON lines (STD-E2E helper) with `ts`, `script`, `bead`
("CARD-T-05"), `step`, `event`, the exact command line, exit code, wall time, the envelope's `evidence` block
(claim Proof, class `Certified`, kernel identities, certificate hash, checker identity), the expected and observed
verdict, and on a mismatch the first failing rule and the bundle path to reproduce.

### Known answers and negative controls

The planar two-link fixture (power −0.3 W by both routes; taking the lower three rows of a space screw Jacobian
as the point Jacobian gives torques (0, 0) and fails it). CX-099. The mutants are the negative controls.
Determinism: exact.

## Considerations

- The pairing is invariant under proper rigid transforms; under a reflection it is preserved by the parity map
  T(P) = diag(det(P)·P, P), which is a separate statement (card T-16).
- CX-112 applies: this is not a statement about floating-point output.

## References

§12.3, §11.3; C-02, C-12; T-04, T-16; CX-099.
