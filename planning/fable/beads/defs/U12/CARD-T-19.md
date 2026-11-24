---
key: "CARD-T-19"
title: "Theorem card T-19: every pseudo-inertia is positive definite and satisfies the mass-support bound"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-algebra", "solid", "test"]
parent: "WP-03"
deps: ["BD-027", "N12-06", "SPEC-C-03"]
needs: []
related: ["CARD-T-28", "CARD-T-35", "N12-41", "N12-REF", "N13-04", "N13-30", "N25d-07", "N25d-08"]
gates: []
plan_refs: ["§12.3 (T-19)", "§11.5", "§10 C-03", "§12.7 (FROB-REF-168)", "Appendix D.1 (pseudo-inertia, shell rule, mass-support bound)"]
---
## Background

A rigid body's ten inertial parameters (mass, centre of mass, six rotational inertia entries) are equivalent to
the 4×4 pseudo-inertia matrix 𝖯 = [[Σ, h], [hᵀ, m]], with Σ = ∫ x·xᵀ dm the second moment of mass, h = ∫ x dm = m·c
the first, and m the mass. With Ī the rotational inertia about the body-frame origin, Σ = ½·tr(Ī)·I − Ī and
Ī = tr(Σ)·I − Σ. The parameters are physically consistent exactly when 𝖯 is positive definite; this single
condition contains positive mass, positive principal moments and the triangle inequalities. Positivity is
necessary and not sufficient for a declared envelope: a body whose mass lies inside a ball of centre o and
radius ρ also satisfies the mass-support bound ‖c − o‖² + tr(I_C)/(2m) ≤ ρ², with I_C the rotational inertia
about the centre of mass; the left side is the mass-weighted mean squared distance from o, since
∫‖x − c‖² dm = tr(Σ_C) = ½·tr(I_C).

Statement (T-19). Every body's pseudo-inertia is positive definite, and its declared envelope satisfies the
mass-support bound. Per body of a compiled model: the rational matrix 𝖯 has an exact LDLᵀ factorization with
positive pivots; and for the body's declared mass envelope (a ball of centre o and radius ρ) the rational
inequality ‖c − o‖² + tr(I_C)/(2m) ≤ ρ² holds.

Route: exact rational LDLᵀ with positive pivots; one rational inequality. Tier: Solid. It turns import validation
into an eigenvalue-free test with a certificate, replacing "ten free floats". Rejected alternatives: checking
only positive principal moments (misses triangle violations); applying the mass-support bound to a collision
proxy (the bound is applied to a declared mass envelope and never to an unrelated collision proxy).

## Scope

In: statement document, registry row, the per-body certificate format (kind `rational-ldlt` plus the
inequality), specimens and mutants, the binding to the preflight refusals `FROB-REF-104` and `FROB-REF-106`. Out:
the per-body certificates of compiled models (N12-20, M5); the inertia cone and the mass-support predicate as
library code (the algebra crate bead); geometry-derived inertia and the shell rule (the model compiler); the
plausibility refusal for solid enclosures (FROB-REF-168, raised by the compiler); Riemannian identification on
the cone (Frontier, separate pilot).

Order of work. This card is finished before the code that uses its theorem and never depends on it: the
pseudo-inertia cone of `frob-algebra` (BD-051), like the model compiler that derives inertia (BD-062), is
`related`, not a dependency; neither depends on this card. What the card does depend on is in `deps`: the card
registry, its specification and Lemma's own proof machinery. It closes on its statement document, its registry
row, its specimens and mutants, and every acceptance line that its own fixtures decide. Two outputs are observed
elsewhere, on live models: the refusal of a failing body at preflight stage 2 (`FROB-REF-104`, inconsistent
inertia; `FROB-REF-106`, mass-support bound), raised by BD-100 with the codes written once in N14a-13; and the
per-body certificates of each reference robot's compiled model, issued at M5 by the certificate-coverage pipeline
N12-20 (its catalogue row "Inertia tables of a model"), whose receipt moves the registry row to `checked`. N12-20
is at a later milestone, so it is not a dependency.

## Technical approach

1. At model compile time each body's inertia is a rational object (geometry and density give exact rationals;
   imported decimal values are read as exact rationals). Form 𝖯 in the body frame.
2. Certificate 1 (bundle kind `rational-ldlt`): the matrix entries as reduced rationals; the auditor recomputes
   the elimination d₁ = 𝖯₁₁, then Schur complements, and accepts only if all four pivots are positive. No
   eigenvalue is computed and no tolerance is involved.
3. Certificate 2 (one exact rational inequality, carried as a side statement of the same bundle with typed
   fields: lengths in metres, inertia in kg·m², mass in kg): ρ² − ‖c − o‖² − tr(I_C)/(2m) ≥ 0, with I_C obtained
   from Ī by the parallel-axis shift, all recomputed by the auditor.
4. Checker: in tree, the exact routine of the algebra crate; offline, `frob-verify`. Reflection (BD-382) can
   promote certificate 1 to `Proved` for a specimen through the LDLᵀ reflector.
5. Class and limits: `Certified` as a statement about the model's numbers. If a body's inertia comes from a card
   or an import marked `assumed`, the model-level claim is `Estimated` (the class is capped by its inputs); the
   certificate still shows the numbers are self-consistent. Consistency is not correctness: a consistent inertia
   can be the wrong inertia.
6. Relies on it: construction C-03; import validation (an inertia that is not physically consistent is refused
   with the smallest eigenvalue reported; declared mass properties that violate the mass-support bound are
   refused); the audit of every reference robot; the shell rule (𝖯_shell = 𝖯(Ω, ρ) − 𝖯(Ω_t, ρ), computed in exact
   rationals so the subtraction of two nearly equal matrices loses nothing).
7. Artifacts: `docs/math/cards/T-19.md`; registry row; per-model certificate sets; specimens
   `rational-ldlt/accept-T-19-*.frobcert` with mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the card's specimens (a solid box, the thin shell at equality, an offset centre of
      mass inside the same ball), each carrying both certificates. The per-body certificates of each reference
      robot are N12-20's at M5, and the refusal of a failing body (`FROB-REF-104` or `FROB-REF-106`, with the
      offending quantity) is BD-100's; neither is a condition of this card's close.
- [ ] Negative controls rejected: principal moments (1, 1, 3) kg·m² (triangle inequality violated: Σ = diag(3/2,
      3/2, −1/2) has a negative pivot); a negative mass; one changed matrix entry in an accepted certificate; the
      mass-support bound evaluated against a collision proxy smaller than the mass envelope.
- [ ] Mass-support fixture: a centred 1 kg part with three principal moments of 0.008 kg·m² passes positivity and
      the triangle tests yet needs 0.012 m² where a ball of radius 0.10 m allows 0.010 m², so it is refused; an
      isotropic thin shell of that radius, with moments 1/150 kg·m², reaches equality and is accepted.
- [ ] Registry row at this card's close: the route registered and `hypothesis`, printing no class; N12-20's
      certificates on the reference robots' inertia tables move it to `checked`, class `Certified`, with hashes and
      checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t19.rs` and the algebra crate's tests: `pseudo_inertia_round_trip` (Ī ↔ Σ by the
two formulas, exactly); `ldlt_positive_pivots_solid_box`; `triangle_violation_negative_pivot`;
`mass_support_fixture_refused` (0.024/2 = 0.012 > 0.010); `thin_shell_equality_accepted`
((3/150)/2 = 0.010 = 0.10²); `offset_centre_of_mass` (c displaced by 0.05 m inside the same ball adds 0.0025 m²);
`changed_entry_rejects`; `class_capped_when_card_assumed`; `negative_mass_rejects` (m = −1 kg: the last pivot
of 𝖯 is negative, `reject`); `collision_proxy_envelope_rejected` (the 0.10 m shell checked against a 0.08 m
collision sphere: the certificate's envelope field is not the declared mass envelope, FROB-REF-829).

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) includes the specimens and mutants; BD-100's script
`scripts/e2e/analyze_mass_stability.sh` exercises the refusals `FROB-REF-104` and `FROB-REF-106` through
`frob analyze mass` (N28a-29's `atelier_check.sh` runs it as a stage and adds the `frob design check` legs). For
this card the NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` and `step` ("CARD-T-19" and
`T-19/<body-or-mutant>` on N12-34's per-card lines; BD-100's own key and step names on its script's lines, since a
line carries its owning bead), `event`, the exact command line, exit code,
wall time, the four pivots as rationals, the mass-support margin ρ² − ‖c − o‖² − tr(I_C)/(2m) in m², the bundle's
BLAKE3, the expected and observed verdict, and on a mismatch the first failing rule (the first non-positive pivot
or the negative margin) and the path to reproduce.

### Known answers and negative controls

The mass-support fixture and the thin-shell equality; the shell-rule limit (as wall thickness tends to zero a
spherical shell of radius r tends to I = (2/3)·m·r², against (2/5)·m·r² for the solid ball); the enclosure case
(an aluminium box of 0.4 × 0.3 × 0.2 m at 2,700 kg/m³ is 64.8 kg solid and 3.45 kg with a 2.5 mm wall, a ratio of
18.8, which is why a solid default is refused). CX-101. Negative controls as listed. Determinism: exact (D0).

## Considerations

- The certificate is per body and per model identity; any edit to geometry, material or fill regenerates it.
- Decimal inputs with many digits give large rationals; sizes stay small in practice (4×4), well inside the
  auditor's limits.
- The reflection specimen "one 4×4 pseudo-inertia with 64-bit numerators" is an instance of this card.

## References

§12.3, §11.5; C-03; FROB-REF-104, FROB-REF-106, FROB-REF-168; CX-101; the LDLᵀ reflector of the reflection pilot.
