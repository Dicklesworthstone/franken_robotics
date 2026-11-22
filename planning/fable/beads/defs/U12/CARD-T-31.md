---
key: "CARD-T-31"
title: "Theorem card T-31: a clearance verdict follows from the stored inner and outer complexes"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "frob-cert", "frob-scene", "solid", "test"]
parent: "WP-53"
deps: ["N12-15", "BD-233", "N12-06", "SPEC-C-33"]
needs: []
related: ["BD-234", "BD-104", "BD-369"]
gates: []
plan_refs: ["§12.3 (T-31)", "§10 C-33", "§19.11", "Appendix D.3 (clearance sandwich, swept right-angle corner)"]
---
## Background

Whether a robot of a given footprint can get from one place to another in an interior is a question about the
free configuration space 𝒞_free ⊆ SE(2), with coordinates (x, y, ψ). It is certified from both sides. For each
fixture mode (a door open, a door closed) two cubical complexes are built on one grid: the inner complex 𝒞_in
contains a cell only if every pose in the cell is collision-free for the envelope inflated by the cell's pose
radius; the outer complex 𝒞_out omits a cell only if every pose in it collides for the envelope deflated by the
same radius. Then 𝒞_in ⊆ 𝒞_free ⊆ 𝒞_out. A connected path from start to goal in 𝒞_in is a feasibility witness
for every robot whose swept footprint fits the envelope. Disconnection of start and goal in 𝒞_out is an
impossibility certificate. Over a family of envelope widths the complexes are nested, and one union–find pass
over the width filtration finds the width at which start and goal first connect and names the bottleneck cell,
a doorway or a corridor. For a width between the inner and outer critical widths the answer is `unknown`, and
that band is the stated resolution of the certificate. The complexes are stored as `.fclear`.

Statement (T-31). A clearance verdict follows from the stored complexes: the claimed path lies in the inner
complex, or start and goal lie in different components of the outer complex.

Route: path replay; union–find recomputed by the checker (`complex-connectivity`). Tier: Solid for the card (the
complexes as a method are Frontier). Rejected alternatives: a route check for one nominal footprint; a clearance
test by one scalar radius.

## Scope

In: statement document, registry row, the per-verdict `complex-connectivity` certificates bound to this card,
fixtures and mutants, the class rule relative to the complex's own evidence. Out: building the complexes and the
enclosure tests behind each cell (the clearance-complex bead); the auditor's checker (N12-15); feasibility tags
for generated scenes; the retrofit search that consumes a named bottleneck.

## Technical approach

1. Route verdict: the bundle carries the inner complex (as a run-length-encoded bitset with its digest and the
   digest of the source `.fclear`), the grid, the envelope, the fixture mode, start, goal and the path as a list
   of cells. The auditor checks that the first cell contains the start pose and the last the goal pose, that
   every cell is in the inner complex and that consecutive cells are face-adjacent (heading wraps).
2. Impossibility verdict: the bundle carries the outer complex; the auditor recomputes connected components by
   its own union–find and checks that start and goal lie in different components (or that one of them is not in
   the outer complex at all).
3. `unknown` band: a verdict claimed for an envelope width inside the band between the inner and outer critical
   widths is rejected; the band (from cell size and heading bin; 48 heading bins give a half-bin of 3.75°) is
   reported in the bundle and never assumed.
4. Class: the verdict is `Certified` relative to the stored complex, and its printed class is the minimum of
   that and the complex's class (a complex whose cell membership rests on enclosure tests is `Certified`; one
   built by sampling is `Tested`). The card says the verdict follows from the complexes; it does not say the
   complexes are right.
5. Limits: geometry only: not steering, traction, reach or perception (which is why a counterexample world
   demands a solvability witness from the candidate's own body). Clearance of a legged body in SE(3) is not
   covered.
6. Relies on it: construction C-33; the clearance analysis; route questions answered without rollouts;
   refusals that say which doorway to change.
7. Artifacts: `docs/math/cards/T-31.md`; registry row; specimens `complex-connectivity/accept-T-31-*.frobcert`
   and mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts a route certificate and an impossibility certificate on the doorway fixture (a
      rectangular body of width w crossing an opening of width D head-on: the inner certificate holds below D
      minus the band and the outer exclusion above D plus the band).
- [ ] Planted wall: closing a gap by one cell flips the outer verdict, and the certificate for the old verdict is
      rejected against the new complex (digest mismatch).
- [ ] Rejected mutations: a path with a non-adjacent step; a path through a cell outside the inner complex; a
      cut claim when the recomputed components connect start and goal; a width inside the band.
- [ ] On each reference interior: no false route and no false impossibility (the scene side's gate; the card's
      certificates are the evidence).
- [ ] Registry row `checked` with the class per verdict, hashes and checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t31.rs`: `doorway_route_below_band`; `doorway_cut_above_band`;
`doorway_inside_band_is_unknown`; `planted_wall_flips_outer_verdict`; `ladder_fixture` (a thin rod of length L
turns a right-angle corner between corridors of width a exactly when L ≤ 2√2·a: a route certificate exists for L
below the inner critical length and a cut certificate for L above the outer one);
`scalar_radius_counterexamples` (the circumscribed-disc test L ≤ a refuses every rod between a and 2.83·a although
each passes; the inscribed-disc test accepts rods longer than 2.83·a, which jam);
`class_is_minimum_with_complex`; `stale_complex_digest_rejects`.

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) runs the specimens and mutants; the scene work's script
exercises `frob contact clearance` and logs the critical widths, the band, the bottleneck cell and the
certificate hash.

### Known answers and negative controls

The ladder and doorway fixtures; the planted wall. Negative controls as listed. Determinism: exact; the
union–find verdict does not depend on traversal order.

## Considerations

- KILL-FCLEAR is the owning mechanism's kill gate (one false route or one false impossibility); its fallback is
  the route graph for a nominal footprint, for which this card issues nothing.
- The band, not the method, limits what is decided; finer grids shrink it at the cost of bundle size.

## References

§12.3; C-33; §19.11; A-18.
