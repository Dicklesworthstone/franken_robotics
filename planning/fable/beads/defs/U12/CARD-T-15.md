---
key: "CARD-T-15"
title: "Theorem card T-15: the rational gear law and compositions of power-preserving port maps preserve power"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-lemma", "solid", "test"]
parent: "WP-05"
deps: ["BD-027", "N12-06", "N12-33", "SPEC-C-12"]
needs: []
related: ["N12-37", "N12-44", "N12-52", "N12x-01", "N15b-01", "N15b-09", "N15b-12"]
gates: []
plan_refs: ["§12.3 (T-15)", "§10 C-12", "§10 C-38", "§15.9", "§15.13 (PO-SINEW-03)", "Appendix D.4 (rational gear law, four-quadrant transmission)"]
---
## Background

Components expose typed ports, and interconnection is power-conserving: the port powers of a composition sum to
zero. A gear is a transformer in bond-graph terms, a two-port element that scales flow by a ratio and effort by
the inverse ratio with the opposite sign convention on its second port. The energy ledger closes per mechanism
and per step only if every ideal transmission element is exactly power-preserving; a wrong gear ratio or a wrong
sign shows as energy appearing from nowhere even when the final return looks excellent.

Statement (T-15). The rational gear law: from f₂ = r·f₁ and e₁ = −r·e₂ it follows that e₁·f₁ + e₂·f₂ = 0, where
(eᵢ, fᵢ) are the effort and flow at port i (torque and angular velocity for shafts) and r is the ratio. And a
composition of power-preserving port maps preserves power: if each element of a chain satisfies its own
two-port power balance, the outer ports of the chain do.

Proof in one line: e₁·f₁ + e₂·f₂ = (−r·e₂)·f₁ + e₂·(r·f₁) = 0. Route: polynomial identity after substitution
(`Certified`); Lean by ring arithmetic (`Proved`). Tier: Solid for the instance, Frontier for the Lean capsule
(which is one of the first three sealed). Scope of validity: the ideal transformer. Lossy transmissions are not
covered by this identity: a four-quadrant transmission has τ_joint = η_f·N·τ_motor when motoring and
τ_motor = η_b·τ_joint/N when back-driven, and one multiplier in all quadrants creates energy; those are loss laws
with their own ledger terms. Rejected alternative: an efficiency number applied symmetrically.

## Scope

In: statement document, registry row, the identities of the ideal parts (gear, belt, screw, linkage, cable and
coupling matrices, the last in symbolic entries) as specimens with mutants; the card's side of the Lean capsule
(the proof itself is written in BD-381's first batch). Out: the certificates bound to the content hashes of the
transmission kernels as built, which the certificate-coverage pipeline N12-20 issues at M5 (its catalogue row
"Transmission and port kernels"); the transmission models and loss laws (the actuator beads); the port
energy-balance check on the referee; wiring-level conservation (CARD-T-29).

Order of work. The transmission models of `frob-actuate` (BD-074) are `related`: BD-074 does not depend on this
card and the card does not wait for them. What the card depends on is in `deps`: the card registry, its
specification, Lemma's own proof machinery and the script that replays its specimens (N12-33). It closes on its
statement document, its registry row, its specimens and mutants, its registered Lean statement and every
acceptance line that its own fixtures decide. The certificates on the transmission kernels as built are N12-20's
at M5, and that receipt moves the instance route to `checked`; the Lean proof is sealed by BD-381 at M5, and that
receipt moves the Lean route. Neither is a dependency: both are at a later milestone, and BD-381 depends on this
card.

## Technical approach

1. Instance route (class `Certified`, kind `poly-identity`): for each transmission kernel, symbolic execution of
   its ideal (lossless) part gives the port relations; substituting them into Σ eᵢ·fᵢ over the element's ports
   must give the zero polynomial. For a coupling matrix C (flows f_out = C·f_in, efforts e_in = −Cᵀ·e_out) the
   identity is e_inᵀ·f_in + e_outᵀ·f_out = −e_outᵀ·C·f_in + e_outᵀ·C·f_in = 0, in the entries of C as symbols, so
   one certificate covers every numeric ratio.
2. Composition: for a chain of two elements with ratios r and s sharing an internal port (where the two
   elements' port powers cancel by the interconnection convention), the outer relation is the gear law with
   ratio r·s; certified as the same identity after eliminating the internal port variables.
3. Checker: FrankenSymPy's proof kernel at build time; `frob-verify` offline.
4. General route (class `Proved`): over any commutative ring,
   `theorem gear_power (r f₁ f₂ e₁ e₂ : R) (hf : f₂ = r * f₁) (he : e₁ = -(r * e₂)) : e₁ * f₁ + e₂ * f₂ = 0`,
   by ring arithmetic; sealed in BD-381 and bound to this card's registered statement.
5. Class and limits: `Certified` for each kernel's ideal part at its content hash; `Proved` for the general law
   once sealed. The loss terms are `Estimated` card values and are outside the identity.
6. Relies on it: construction C-12 (gears are transformers and motors are gyrators); C-38; the port
   energy-balance check; the energy ledger; proof obligation PO-SINEW-03 (§15.13, the ideal gear law conserves
   power), which raises no new card and is listed on this card's row as a consumer (N12-52).
7. Artifacts: `docs/math/cards/T-15.md`; registry row; specimen `poly-identity/accept-T-15.frobcert` with
   mutants; `lean/Frob/Statements/T15.lean` (the certificates beside the transmission kernels are N12-20's, at
   M5).

## Acceptance criteria

- [ ] `frob-verify` accepts the T-15 specimen bundles (the gear law, and the coupling-matrix identity in the
      entries of C as symbols) and rejects: the same-sign law e₁ = r·e₂ (power 2·r·e₂·f₁ ≠ 0); a ratio applied to
      flow and not to effort; one changed coefficient.
- [ ] The composition identity for two gears in series certifies with ratio r·s.
- [ ] Registry row at this card's close (M1): instance route `hypothesis`, printing no class (N12-20's
      certificates on the transmission kernels as built move it to `checked`, class `Certified`, at M5); Lean
      route `hypothesis`, with the registered statement `lean/Frob/Statements/T15.lean` present. BD-381, which
      depends on this card, seals the proof at M5, and its receipt moves the Lean route to `checked`, class
      `Proved`, with capsule hash and both checker identities.
- [ ] The statement document states that lossy and back-driven behaviour is outside the identity and names the
      four-quadrant counterexample.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t15.rs`: `gear_identity_expands_to_zero`; `coupling_matrix_identity`;
`series_gears_compose` (r = 3, s = 5/2 give 15/2); `same_sign_law_rejects`; `flow_only_ratio_rejects` (f₂ = r·f₁
with e₁ = −e₂: power (r − 1)·e₂·f₁ ≠ 0 for r ≠ 1); `changed_coefficient_rejects`; `specimen_accepts`;
`exact_numbers` (r = 4, f₁ = 10 rad/s, e₂ = 2 N·m: f₂ = 40 rad/s, e₁ = −8 N·m, e₁·f₁ + e₂·f₂ = −80 + 80 = 0 W);
`one_multiplier_all_quadrants_creates_energy` (a transmission with efficiency η = 0.8 applied as
τ_joint = η·N·τ_motor in both directions returns more power back-driven than was put in; a documented related
control, asserted on the loss model's tests, not on this identity).

### End-to-end script and logging

At M1, when the card closes, its `poly-identity` specimen and mutants join the specimen set that
`scripts/e2e/lemma_kernel_cert.sh` (bead N12-33, a dependency) replays offline with `frob-verify`; at M5
`scripts/e2e/lemma_coverage.sh` (bead N12-36) lists the transmission kernels' certificates and
`scripts/e2e/lemma_lean_capsule.sh` (bead N12-37) seals the Lean form. For this card their NDJSON lines (STD-E2E
helper) carry `ts`, `script`, `bead` ("CARD-T-15"), `step`, `event`, the exact command line, exit code, wall time,
the envelope's `evidence` block (claim Proof, class, kernel identity or capsule hash, checker identity), the
expected and observed verdict per specimen and mutant, and on a mismatch the first failing rule and the bundle
path to reproduce.

### Known answers and negative controls

The exact numbers above. A screw with lead angle a and friction angle φ has η_f = tan a/tan(a + φ),
η_b = tan(a − φ)/tan a, and locks when φ ≥ a: the lossy law this identity does not cover. Negative controls: the
mutants; a reversed port sign must be refused by the design compiler. Determinism: exact.

## Considerations

- The sign convention (power into the element positive at each port) is part of the statement; a convention
  change turns the identity into e₁·f₁ − e₂·f₂ = 0 and must be made in one place.
- CX-112 applies to the kernels: the identity is over ℚ.

## References

§12.3; C-12, C-38; T-29; GATE-LEMMA-02 (through BD-381).
