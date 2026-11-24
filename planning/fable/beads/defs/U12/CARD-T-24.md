---
key: "CARD-T-24"
title: "Theorem card T-24: the number of canonical campaign representatives equals the orbit count |Γ|⁻¹·Σ_g k^c(g)"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "frob-cert", "frob-algebra", "solid", "test"]
parent: "WP-66"
deps: ["N12-06", "N12-18", "N17a-23", "SPEC-C-22", "STD-E2E"]
needs: []
related: ["BD-049", "BD-285", "CARD-T-25", "GB-19", "N12-25", "N12-39", "N12-45", "N13-17", "N17a-23"]
gates: []
plan_refs: ["§12.3 (T-24)", "§12.8 (OPEN-LEMMA-6)", "§10 C-22", "§13.6", "§17.11", "Appendix D.5 (Burnside counting, wreath product)"]
---
## Background

A design campaign enumerates assignments of module types to slots: legs to hips, wheels to corners, actuators to
joints. The group that acts on the slots X is the observation-preserving symmetry group Γ_obs of the chassis and
task, never the mechanical group alone, and two assignments in one orbit are the same closed loop. The campaign
generates one canonical representative per orbit (the lexicographically least assignment) and records for every
member the group element that maps it to the representative, so that results are transported back correctly.
Quotienting deletes simulations exactly and before any simulation, by a factor of at most the group order
(between 2 and about 12 for the bodies in scope). If the generator produced too few representatives, designs
would silently go unevaluated; too many, and compute is wasted. The count is the check.

Statement (T-24). The number of canonical representatives generated for a campaign equals |Γ|⁻¹·Σ_g k^c(g), where
k is the number of module types, the sum runs over the elements g of the acting group Γ, and c(g) is the number
of cycles of g on the slots X (the orbit-counting lemma). When a module has its own automorphism group K that
preserves its attachment-port invariants and the hub group H preserves the decorated ports, the acting group is
the wreath product K ≀ H = K^|X| ⋊ H of order |K|^|X|·|H|, and the orbit-counting lemma applies to its action on
the decorated assignments, with the number of assignments fixed by g evaluated as the product, over the cycles C
of g on the slots, of k times the number of orientations fixed by the composite of the orientation actions around
C. Without orientations each factor is k and the product is k^c(g); with them k^c(g) is not the fixed count, and
the auditor (N12-18) evaluates the product form.

Route: exhaustive enumeration for small cases, re-derived by the auditor (Solid, `Certified`); the counting
lemma is cited from Mathlib only if its closure re-checks in FrankenLean, and otherwise the count is `Tested`
(Frontier half, gated by OPEN-LEMMA-6). Rejected alternative: one dispatch per enumerated design.

## Scope

In: statement document, registry row, the per-campaign `orbit-count` certificate bound to this card, fixtures and
mutants; the rule for which class is printed when enumeration is out of reach. Out: the group and orbit code
(the algebra crate); the campaign quotient and the mission-observable projection (Phylum); the auditor's checker
(N12-18); the Mathlib question (N12-25) and the Lean citation (N12-39).

Order of work. The code that implements the theorem lands earlier and does not wait on this card: the orbit code
of `frob-algebra` (BD-049, M1), the orbit representatives of `frob-phylum` (N17a-23, M3) and campaign quotienting
(BD-285, M3). At M3 they class the count `Tested` against the construction's law tests, because the `orbit-count`
checker (N12-18) arrives at M4. No loop is possible, so the one of them whose output this card reads, N17a-23, is a
dependency; BD-049 and BD-285 stay `related`. The card further depends on the card registry, its specification and
the checker. It closes on its statement document, its registry row, its specimens and mutants, and every
acceptance line that its own fixtures decide. The acceptance line that needs live output (the certificate for the
count a sweep produced) is observed here at M4 and not inside the earlier beads, which are not reopened: the
card's test takes N17a-23's representatives and Γ_obs generators for the three-type hexapod sweep, writes them as
an `orbit-count` proof object with the fields of step 1, and checks it with `frob-verify`; that receipt moves the
registry row.

## Technical approach

1. Per campaign the producer emits an `orbit-count` proof object: slot count, generators of Γ as permutations
   (and, for a wreath product, the module group's action on per-slot orientations), k, the claimed count and the
   list of canonical representatives.
2. The auditor closes the generators, computes c(g) for every element, evaluates the formula in exact integers
   and, when kⁿ is within its enumeration cap, enumerates all assignments and checks that the representative list
   is exactly the set of lexicographically least orbit members. That re-derivation makes the instance
   `Certified` without citing any lemma: the count is checked by direct computation, and the formula is a
   cross-check.
3. Beyond the enumeration cap the auditor certifies only that the claimed count equals the formula's value for
   the given group; that the formula counts orbits is then the orbit-counting lemma. Class rule: if the lemma's
   Lean capsule is sealed (after OPEN-LEMMA-6 is answered `yes`), the count is `Certified` on the strength of a
   `Proved` lemma; until then it is `Tested`, with the tested cases being the enumerated ones.
4. The query-match rule guards the group: the bundle's query embeds the declared symmetry (Γ_obs, with sensors
   and task); a certificate computed with the larger mechanical group is offered for a different query and is
   rejected.
5. Class and limits: as above. The mission-observable projection ("equal projections give identical
   trajectories") is not part of this card and is not decidable by any checker; it is a structural rule with a
   bit-identical replay test.
6. Relies on it: construction C-22; campaign quotienting; the morphology generator.
7. Artifacts: `docs/math/cards/T-24.md`; registry row; specimens `orbit-count/accept-T-24-*.frobcert` and mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts: ten leg types on the six slots of a hexapod under the dihedral group of order 12,
      count (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = 86,185 (a factor of 11.6); under the sagittal mirror
      alone, (10⁶ + 10³)/2 = 500,500 (a factor of 2.0); three types, 92, with the 92 representatives enumerated.
- [ ] Wreath product: two slots swapped by the hub, each holding one of k types in one of two mounting
      orientations identified by the module's half-turn symmetry; the group C₂ ≀ C₂ has order 8 and the count is
      (4k² + 4k)/8 = (k² + k)/2, which is 6 for k = 3 against 36 raw assignments.
- [ ] Rejected: a count off by one; a representative that is not lexicographically least; a certificate computed
      with Γ_mech offered for a query that declares a front-only lidar.
- [ ] Live sweep: N17a-23's representatives and generators for three leg types on the radial hexapod (92
      representatives, 3⁶ = 729 assignments, within the enumeration cap), written as an `orbit-count` proof object,
      are accepted by `frob-verify` with class `Certified`; the same object with one representative dropped is
      rejected (count 91 against 92).
- [ ] Registry row: instance route `checked` with class `Certified` for enumerated campaigns; campaigns beyond
      the cap print `Tested` until the Lean route is sealed.
- [ ] `scripts/e2e/card_t24.sh` passes and its log ends with a `summary` line.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t24.rs`: `hexapod_cycle_index_terms` (identity: 6 cycles; rotations by one and
five steps: 1 cycle each; by two and four steps: 2 cycles each; by three steps: 3 cycles; three reflections
through opposite slots: 4 cycles each; three reflections through opposite edges: 3 cycles each);
`hexapod_count_86185`; `mirror_count_500500`; `three_types_92_matches_enumeration`; `wreath_k3_is_6`;
`count_off_by_one_rejects`; `gamma_mech_for_lidar_query_rejects`; `class_tested_beyond_cap_without_lemma`;
`non_least_representative_rejects` (in the three-type hexapod list one representative replaced by a rotated member
of its orbit: the count still reads 92, the auditor finds a smaller member of that orbit and rejects naming it);
`live_sweep_certificate` (the live-sweep line above, both the accepted object and the dropped-representative
mutant).

### End-to-end script and logging

This card's own script, `scripts/e2e/card_t24.sh` (header `bead=CARD-T-24 milestone=M4 labels=lemma,frob-cert
seed=0 needs=cpu`; sources the STD-E2E helper, idempotent, writes only under `target/`): `laws`, `cargo test -p
frob-lemma --test cards -- t24`; `corpus`, `frob-verify --json` under `env -i` on every `accept-T-24-*` specimen and
its mutants, asserting the verdict and first failing rule; `live`, the live-sweep object and its mutant through
`frob-verify --json`. `scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) also walks the specimens and mutants
as part of its corpus; the campaign's own script (BD-285) logs group order and count through `frob phylum
quotient`, with no certificate at M3. For this card the NDJSON lines (STD-E2E helper;
`target/e2e-logs/card_t24/<utc>.ndjson`, transcript beside it) carry `ts`, `script`, `bead` ("CARD-T-24"), `step`
(`T-24/<campaign-or-mutant>`), `event`, the
exact command line, exit code, wall time, the group order, k, the slot count, the claimed and recomputed counts,
whether enumeration ran (kⁿ against the auditor's cap), the printed class, the bundle's BLAKE3, the expected and
observed verdict, and on a mismatch the first failing rule (the first count or representative that differs) and
the path to reproduce; the `summary` line counts assertions passed and failed.

### Known answers and negative controls

The three hexapod counts and the wreath count. Projection fixture, for context and not certified here: eight
equal ballast slugs in 24 collinear slots enter rigid-body dynamics only through their count, Σx and Σx², so
C(24, 8) = 735,471 placements fall into 24,546 classes; slots {1, 4} and {2, 3} have equal Σx = 5 and different
Σx², 17 and 13, and must not merge. Counterexample: counting with Γ_mech when a front-only lidar is fitted merges
designs that the closed loop distinguishes. Determinism: exact.

## Considerations

- OPEN-LEMMA-6: default, the lemma is cited only after a recorded re-check; until then the count is `Tested`
  beyond enumeration.
- The factor is honest and small when a sensor breaks the symmetry; the card does not promise a saving.

## References

§12.3, §12.8; C-22; OPEN-LEMMA-6; the orbit counts of the known-answer suite.
