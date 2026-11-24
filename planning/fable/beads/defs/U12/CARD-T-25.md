---
key: "CARD-T-25"
title: "Theorem card T-25: gait enumeration is complete (all (H, K) with K normal in H and H/K cyclic)"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "frob-cert", "frob-gait", "frontier", "test"]
parent: "WP-64"
deps: ["N10a-17", "N12-06", "N12-18", "SPEC-C-10"]
needs: []
related: ["N12-45", "N13-17", "N22a-31"]
gates: []
plan_refs: ["§12.3 (T-25)", "§10 C-10", "§22.9", "Appendix D.5 (H/K theorem)"]
---
## Background

A periodic gait of a symmetric system has a group H of spatio-temporal symmetries (elements that map the orbit
to itself up to a phase shift) and a normal subgroup K of purely spatial symmetries, and H/K is cyclic. The
ambient group for a body is the leg-permutation part of the observation-preserving group, intersected with the
isotropy subgroup of the commanded velocity; it is small. A quadruped commanded forward keeps only the
left–right mirror C₂, under which trot, pace and walk share the pair (C₂, 1) and bound and pronk share (C₂, C₂).
The six primary gaits are separated only by the symmetry group of a declared leg-clock network, ℤ₄ × ℤ₂ for the
eight-cell quadruped network; that group contains permutations no isometry of the body realizes, so it is a
declared symmetry of the controller's coupling and not a subgroup of the body's group. The gait atlas lists
every (H, K) pair of the ambient group with its phase relations, each usable as a clock for the baseline
controller and as a reference for reward shaping. A missing pair is a gait type the studio never considers.

Statement (T-25). Gait enumeration is complete: the listed (H, K) pairs are exactly the pairs of subgroups of the
ambient group with K normal in H and H/K cyclic.

Route: exhaustive enumeration of the subgroup lattice, re-derived by the checker (`group-enumeration`). Tier:
Frontier. The H/K theorem is used as an enumeration rule for admissible gait types, not as a list of realized
gaits. Rejected alternative: hand-tuned phase-offset tables per robot.

## Scope

In: statement document, registry row, the per-atlas `group-enumeration` certificate bound to this card, fixtures
and mutants. Out: computing the ambient group (symmetry analysis); the atlas, phase relations and max-plus
timing (the gait-atlas bead); orbits and their stability; the auditor's checker (N12-18).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the gait atlas (BD-332) is `related`, not a dependency (a card that waited for its implementer would close a loop,
because the implementer's own acceptance runs the card's law tests). What the card does depend on is in `deps`:
the card registry, its specification, its pilot where the route is Frontier, and Lemma's own proof machinery. It
closes on its statement document, its registry row, its specimens and mutants, and every acceptance line that its
own fixtures decide. An acceptance line that can only be observed on live output (the per-atlas certificate and
the atlas built from an accepted enumeration) is run inside the implementing bead, with this card's law tests, as
part of that bead's acceptance; the receipt produced there is what moves the registry row for that output.

## Technical approach

1. Per atlas the producer emits a `group-enumeration` proof object: generators of the ambient group as
   permutations (of legs for the body group; of network cells for a declared clock-network group, with the
   declaration recorded as an assumption of the bundle), and the list of pairs (H, K), each subgroup by
   generators, with the cyclic generator of H/K and the resulting phase lags.
2. The auditor closes the ambient group, enumerates every subgroup (cyclic subgroups first, then closure under
   joins until no new subgroup appears), and for each ordered pair K ≤ H tests normality (conjugation of K's
   generators by H's generators stays in K) and cyclicity of the quotient (some coset's powers exhaust H/K). It
   accepts only if the derived set of pairs equals the claimed set: nothing missing, nothing extra. Conjugate
   pairs are listed separately or merged according to the convention declared in the bundle, and the convention
   is checked.
3. Phase lags: for a pair with |H/K| = m and generating coset h·K, the element h shifts phase by 1/m of a
   period; the auditor recomputes each leg's phase offset from the orbit of the leg under powers of h and checks
   the listed lags as exact rationals in [0, 1).
4. Class and limits: `Certified` for the enumeration of the declared ambient group. Declaring ℤ₄ × ℤ₂ is an
   assumption about the controller, carried in the bundle; the card says nothing about which gaits are
   realizable, stable or efficient. An orbit computed inside the fixed-point subspace of K can hide unstable
   multipliers transverse to it, which is why the monodromy is always computed on the full space (not this
   card).
5. Relies on it: construction C-10; the gait atlas analysis; symmetric shooting on [0, T/m] inside Fix(K) with
   the twisted boundary condition x(T/m) = h·x(0).
6. Artifacts: `docs/math/cards/T-25.md`; registry row; specimens `group-enumeration/accept-T-25-*.frobcert` and
   mutants.

## Acceptance criteria

- [ ] With the body group C₂ alone the enumeration of the card's specimen returns exactly (C₂, C₂), (C₂, 1) and
      (1, 1), so trot, pace and walk fall in one class (C₂, 1); `frob-verify` accepts the certificate.
- [ ] With the ℤ₄ × ℤ₂ clock-network group declared, the specimen's enumeration is accepted and contains the
      primary quadruped gaits reported in the literature (trot, pace, bound, walk, pronk and the jump) as distinct
      (H, K) pairs whose phase lags the auditor recomputes. The same two facts observed on live atlas output
      (`frob gait atlas`) are acceptance lines of BD-332, run there with this card's law tests, as the order of
      work above states.
- [ ] Rejected: a list with one pair removed; a pair whose K is not normal in H; a pair with non-cyclic
      quotient; a wrong phase lag (a trot with diagonal legs half a period apart instead of in phase).
- [ ] Registry row `checked`, class `Certified`, with hash and checker identity; the declared-network assumption
      is visible in the row.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t25.rs`: `c2_pairs_exact`; `z4xz2_subgroup_count` (the group of order 8 has
exactly eight subgroups: the trivial one, three of order 2, two cyclic and one non-cyclic of order 4, and the
whole group); `z4xz2_pairs_all_normal` (the group is abelian, so every subgroup is normal and only cyclicity of
H/K filters); `noncyclic_quotient_excluded` (with H the whole group, K = 1 is excluded because the quotient
ℤ₄ × ℤ₂ is not cyclic, and so is K = ⟨(2, 0)⟩, whose quotient is ℤ₂ × ℤ₂; the pair is included exactly for the six
K with cyclic quotient: ⟨(0, 1)⟩ and ⟨(2, 1)⟩ of order 2, the three subgroups of order 4, and the whole group);
`missing_pair_rejects`; `phase_lags_recomputed`; `trot_wrong_lag_rejects`;
`non_normal_pair_rejects` (synthetic: ambient D₄ acting on the four legs of a square body, the claimed pair
(D₄, ⟨s⟩) with s a reflection: conjugating s by the quarter turn r gives r·s·r⁻¹ = s·r⁻² ∉ ⟨s⟩, so the auditor
rejects naming the generator pair); `noncyclic_quotient_claim_rejects` (the list for ℤ₄ × ℤ₂ with the extra pair
(ℤ₄ × ℤ₂, 1): `reject`, nothing extra is allowed).

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) runs the specimens and mutants;
`scripts/e2e/gait_atlas.sh` (bead BD-332, which depends on this card) logs the atlas with its certificate hash
through `frob gait atlas`. For this card their NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` and
`step` ("CARD-T-25" and `T-25/<ambient-group>/<specimen-or-mutant>` on N12-34's per-card lines; BD-332's own key
and step names on its script's lines, since a line carries its owning bead), `event`,
the exact command line, exit code, wall time, the ambient group's order and its subgroup count, the claimed and
recomputed pair counts, the declared-network assumption where present, the bundle's BLAKE3, the expected and
observed verdict, and on a mismatch the first missing, extra or wrong pair (or phase lag) and the path to
reproduce.

### Known answers and negative controls

The C₂ list; the primary gaits under ℤ₄ × ℤ₂; the max-plus eigenvalue of a trot schedule equal to the stride
period computed by hand (a related known answer of the construction). Negative controls as listed. Determinism:
exact; the enumeration order is canonical.

## Considerations

- Published full-order orbit solves take about 4 to 260 s of CPU at 20 to 68 states (a literature fact to be
  re-read), so an orbit is a screening signal; enumeration is cheap and is the only part certified.
- Kill gate of the owning construction is on the orbit ranking (rank agreement below 0.5 with rollouts); its
  fallback keeps the atlas as phase patterns and controller clocks, which still uses this card.

## References

§12.3; C-10; §22.9; the H/K theorem; A-04.
