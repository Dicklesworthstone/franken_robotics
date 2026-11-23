---
key: "SPEC-C-33"
title: "spec C-33: clearance certified from both sides: inner and outer complexes in SE(2), filtration, reported band"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "spine", "spec", "frontier", "habitat", "frob-scene", "test"]
parent: "WP-53"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-104", "BD-132", "BD-233", "BD-234", "BD-369", "SPEC-C-28", "SPEC-C-31", "N10-CX", "N10c-10", "CARD-T-31", "BD-162"]
gates: ["GATE-HAB-10"]
plan_refs: ["§10 C-33", "§19.11", "§12.3 T-31", "§12.4", "§38.7 MR-29", "Appendix D.3", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-33 of the FrankenRobotics spine answers "can this body get from here to there in this place?" with
a two-sided certificate. For a planar base the free configuration space is a subset of SE(2) with coordinates
(x, y, ψ). Two cubical complexes are built on one grid: an inner complex in which every pose is certainly free,
and an outer complex outside which every pose certainly collides. A path in the inner complex is a feasibility
witness; disconnection in the outer complex is an impossibility certificate; in between the answer is `unknown`,
and that band is the stated resolution of the certificate. Tier Frontier [F]. Owner: the Habitat subsystem
(scenes, clearance, retrofits), crate `frob-scene`; the complexes are stored as `.fclear`.

It replaces a route check for one nominal footprint and a clearance test by one scalar radius. A scalar test is
wrong in both directions for a body that is not a disc: the circumscribed disc refuses bodies that pass and the
inscribed disc accepts bodies that jam (the ladder case below). Payoff: a two-sided answer with a named
bottleneck (a doorway or a corridor), reused across many width queries, and a refusal that says which doorway to
change, which is the input of the retrofit search.

Design reasoning. The enclosure is one-sided on purpose: a distance function has Lipschitz constant one, and a
point at radius at most ρ from the footprint's reference point moves at most ρ·|Δψ| under a rotation, so the
clearance at any pose in a cell differs from the clearance at its centre by a bounded, computable amount. FACT
(FrankenSim `483d09c682f2`, observed 2026-11-21): `fs-topo` has connectivity and zero-dimensional persistence for
voxel fields, capped at 10⁶ cells with no periodic axis, and `fs-motion` has swept charts; so `frob-scene`
implements the filtration sweep itself and uses the sibling as a parity oracle on small fixtures. The mechanism is
not claimed as unique (an incumbent planning library could hold the same index) and no multiplier is claimed.

Terms. *Fixture mode*: an assignment of states to movable fixtures (a door open, a door closed). *Envelope*: the
footprint family F(w) = F₀ ⊕ disc(w), a base shape inflated by a margin w ≥ 0. *Clearance* c(q): the distance from
F₀ placed at pose q to the nearest obstacle in the height slab, negative inside an obstacle; F(w) at q is free
exactly when c(q) > w. *Pose radius* of a cell: the bound e below. Proof classes: Proved, Certified (re-checked by
the offline auditor), Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-33.md`; law tests in `crates/frob-scene/tests/laws_c33.rs`; registry entries for KA-161 to
KA-163, relation MR-29, the statement of theorem card T-31 as a contract clause, and the control seeded from
CX-081.

Out: the `.fclear` format, the builder with local refinement, the verb `scene.fit` and the enclosure tests
(BD-233); feasibility tags for generated scenes (BD-234); the auditor's `complex-connectivity` checker (BD-132
and BD-233); the retrofit search (BD-369); recording GATE-HAB-10 and deciding KILL-FCLEAR (BD-233).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-33: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`scene.fit`), `failing_test` (a test of
`frob-scene::laws_c33` with its registered control), `payoff.gate` (GATE-HAB-10), `statements`, `known_answers`
(KA-161 to KA-163), `cards` (T-31), `gates`, `pilot` (the record `PILOT-C-33` of the pilot register, required
because the tier is Frontier) and `generic_comparator` (the off-the-shelf method named in the pilot paragraph of
this bead). Doctrine D-2 (no ornamental mathematics) is checked on these fields: a construction with no verb or no
failing test is moved to the parked register by a decision record. The contract `docs/math/C-33.md` follows the
ten-section template `docs/math/TEMPLATE.md` of the same bead.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-162 (the scene manifest of
`frob-scene`): that bead is listed in `related` and is not a prerequisite. The law suite is generic over its
subject trait and closes on the exact reference model and the mutants in its own test support. If `frob-scene`
does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Definitions and statements

A cell is a closed box with centre q₀, translation edge Δ and heading width δ. For every pose q in the cell,
|c(q) − c(q₀)| ≤ e with

```text
e = (Δ/2)·√2 + ρ·(δ/2) + ε_field + u_reg
```

where ρ is the largest distance from the footprint's reference point to a point of F₀, ε_field is the scene
field's error bound and u_reg the registration uncertainty of an ingested scene. The cell stores c_lo = c(q₀) − e
and c_hi = c(q₀) + e. Cells are adjacent across faces, with ψ periodic.

| Id | Statement | Class |
|---|---|---|
| S1 | The inner complex 𝒞_in(w) is the set of cells with c_lo > w: every pose in such a cell is collision-free for the envelope inflated by the cell's pose radius. The outer complex 𝒞_out(w) is the set with c_hi > w: a cell is omitted only if every pose in it collides for the envelope deflated by the same radius. Then 𝒞_in ⊆ 𝒞_free ⊆ 𝒞_out. | Tested (sampling against a brute-force grid); the per-cell bound is Certified when the checker recomputes it |
| S2 | A face-connected path of inner cells from the start cell to a goal cell is a feasibility witness for every robot whose swept footprint fits the envelope. | Certified per instance (theorem card T-31, proof kind `complex-connectivity`: path replay) |
| S3 | If start and every goal cell lie in different components of the outer complex, no collision-free path exists for margin w: an impossibility certificate. Face adjacency suffices because cells are closed: a free pose on a shared edge or corner makes every cell containing it an outer cell, and those cells are face-connected around it. | Certified per instance (T-31: union–find recomputed by the checker) |
| S4 | Over a family of envelope widths the complexes are nested, and one union–find pass over the width filtration finds the width at which start and goal first connect and names the bottleneck cell. Sorting by c_lo in decreasing order gives w_in, the largest certified margin; the same pass on c_hi gives w_out, the margin at and above which passage is impossible. | Tested (equal to per-width queries) |
| S5 | For a width between the inner and outer critical widths the answer is `unknown`, and that band is the stated resolution of the certificate. A finer grid only narrows the band. | Tested |
| S6 | An envelope extension carries overhead clearance and step-over height: obstacles lower than the step-over height or higher than the body's top are outside the slab. | Contract clause |
| S7 | In an ingested scene, unknown space is blocked in the inner complex and free in the outer complex. | Tested |

### 2. Algorithms the contract fixes

Width filtration (S4): (1) sort cells by c_lo, largest first; (2) add them one at a time to a union–find
structure, joining each to its face neighbours already present; (3) when the start and a goal component merge,
record w_in as the c_lo of the cell just added and that cell as the bottleneck; (4) repeat with c_hi for w_out.
Checker rules: a witness is a list of cells; the checker recomputes c(q₀) and e for each and tests adjacency. A cut
is a component labelling of the outer complex; the checker tests that every unlabelled cell has c_hi ≤ w, that no
two adjacent labelled cells carry different labels, and that start and goal labels differ. Both checks are linear
in the number of cells and need only the scene field.

### 3. Conventions, failure cases and non-claims

- The band follows from cell size and heading bin and is reported, never assumed. 48 heading bins give a half-bin
  of 3.75°. At the default lattice of the owning chapter (Δ = 5 cm, 32 headings, ρ = 0.40 m) the error term is
  35.4 mm + 39.3 mm = 74.6 mm before field and registration error, so a doorway fit with a total margin under
  2e ≈ 149 mm is `unknown` there; local refinement is part of the mechanism (Δ = 12.5 mm and 128 headings give
  18.7 mm; Δ = 5 mm and 512 headings give 6.0 mm). A 12 m × 8 m floor at the default lattice is 240 × 160 × 32 =
  1,228,800 cells, 4.69 MiB at four bytes per cell.
- The complexes certify geometry only: not steering, traction, reach or perception. That is why a minimal
  counterexample world (construction C-28) demands a witness from the candidate's own body. A witness is labelled
  `kinematic` for a base whose growth vector reaches full rank with reversal permitted, otherwise
  `geometric-only`. Clearance of a legged body in SE(3) is not covered.
- A certificate's class is Certified relative to the compiled scene and its stated bounds; it says nothing about
  the real building beyond what the scene's provenance supports.

### 4. Law-test layout, registry, pilot and gate

`crates/frob-scene/tests/laws_c33.rs` with `tests/support/c33_ref.rs`: *synthetic* polygonal scenes (an L-shaped
corridor of width a; a wall with a doorway of width D; two doorways in series), exact clearance of a rod or a
rectangle to polygon edges, the cell bound e, the two complexes, the filtration and both checkers; a brute-force
grid at four times the resolution is the independent oracle. Generic over a trait `C33Subject`; mutants in
`tests/support/c33_mutants.rs`. Registry: KA-161 to KA-163 with derivation files; MR-29; the control from CX-081.

Pilot (BD-233): the ladder and doorway fixtures and one home interior at two grid resolutions. Falsifier: one false
route or one false impossibility. Ceiling: route questions answered without rollouts; the band, not the method,
limits what is decided. Fallback: the route graph for a nominal footprint. Gate GATE-HAB-10 at M3 (the corridor
closed form bracketed by witness and cut on 100 (a, b) pairs; zero false witnesses and zero false cuts on the seven
archetype fixtures against a brute-force grid at four times the resolution; the checker accepts every certificate
and rejects six mutated ones). Kill gate KILL-FCLEAR (M3): one false witness or false cut survives a repair cycle,
or more than 30% of doorway queries on the archetype battery stay unresolved after refinement; fallback: the
route-graph traversability check for a nominal footprint, labelled heuristic.

## Acceptance criteria

- [ ] `docs/math/C-33.md` exists with the cell bound and its derivation, S1 to S7, the filtration steps, the
      checker rules, the worked lattice numbers and the non-claims.
- [ ] `cargo test -p frob-scene --test laws_c33` passes on the reference model, and on the production complexes if
      BD-233 has landed.
- [ ] Every mutant below fails its named law; six mutated certificates are rejected by the reference checker.
- [ ] KA-161 to KA-163 are registered with derivation files; MR-29 and the control from CX-081 are attached to
      GATE-HAB-10.
- [ ] `scripts/e2e/spec_c33.sh` exits 0.
- [ ] Row C-33 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `cell_bound_arithmetic`: Δ = 0.05 m, 32 headings, ρ = 0.40 m give 0.0354 + 0.0393 = 0.0746 m; the refined
  lattices give 0.0187 m and 0.0060 m; 48 bins give 3.75°; 240 × 160 × 32 = 1,228,800 cells.
- `sandwich_property`: on each synthetic scene, 10⁵ seeded poses: every pose in an inner cell is free and every
  free pose lies in an outer cell.
- `ka_161_ladder`: corridors of width a = 1 m; the closed-form limit is 2√2·a ≈ 2.828 m. Rods of 2.0 and 2.5 m get
  a witness; rods of 3.1 and 3.5 m get a cut; a rod of 2.82 m is never cut and a rod of 2.84 m never gets a
  witness. The circumscribed-disc test (L ≤ a) refuses the 2.0 m rod; the inscribed-disc test accepts the 3.5 m
  rod.
- `rectangle_corner_closed_form`: a rectangle of width b turns the corner exactly when L ≤ 2√2·a − 2·b; for
  a = 0.90 m and b = 0.60 m the limit is 1.346 m; 1.30 m gets a witness after refinement and 1.40 m a cut.
- `ka_162_doorway`: opening D = 0.80 m, head-on rectangle of width w with ρ = 0.40 m, generated scene (ε_field
  and u_reg zero), so 2e = 149 mm. At the default lattice w = 0.60 m gets a
  witness, w = 0.70 m is `unknown` and gets a witness after refinement, w = 1.00 m gets a cut; the reported band
  contains D.
- `planted_wall_flips_outer`: closing a gap by one cell changes the outer verdict to a cut.
- `filtration_equals_per_width_queries`: two doorways of 0.9 m and 0.7 m in series; w_in and w_out from one pass
  equal the thresholds found by 20 separate queries; the bottleneck cell lies in the 0.7 m doorway.
- `nestedness_and_refinement` (MR-29): both complexes shrink as w grows; halving Δ and δ never widens the band
  and w_in ≤ w_out always.
- `heading_wraps`: a path that must rotate through ψ = 2π is found.
- `unknown_space_rule`: a witness never crosses unknown cells and a cut never relies on them.
- `checker_accepts_and_rejects`: the reference checker accepts each certificate and rejects a path with a
  non-adjacent step, a path cell with c_lo ≤ w, a labelling with two adjacent labels, an unlabelled cell with
  c_hi > w, equal start and goal labels, and a certificate with a changed margin.
- `ka_163_synthetic_interiors`: on three synthetic interiors there is no false route and no false impossibility
  against the brute-force oracle.

### End-to-end script and logging

`scripts/e2e/spec_c33.sh`: `frob --json doctor`; `cargo test -p frob-scene --test laws_c33`; `cargo test -p
frob-conformance --test g1 -- KA-161 KA-162 KA-163`; `cargo test -p frob-conformance --test g3 -- MR-29`;
`frob --json prove list` filtered to card T-31; file and registry checks. Log to
`target/e2e-logs/spec_c33/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`, `step`, `event`,
command line, exit code, wall time, the evidence block (claim Known answer, class Tested, tier G1 host,
determinism of the host run, seed), measured w_in and w_out in metres against the closed form and the reported
band, and on failure the first offending cell index with its c_lo and c_hi. The `summary` line counts assertions
and names GATE-HAB-10.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-161 | A thin rod turns a right-angle corner between corridors of width a exactly when L ≤ 2√2·a | the reported resolution band | G1 host | Mutants `circumscribed_disc` and `inscribed_disc` |
| KA-162 | Head-on doorway: witness below D minus the band, cut above D plus the band, `unknown` between | the reported band | G1 host | Planted wall: closing a gap by one cell flips the outer verdict |
| KA-163 | On each reference interior: no false route, no false impossibility | exact (zero of each) | G1 host (reference interiors bound by BD-233) | One false route or one false impossibility falsifies the pilot |

Other mutants: `pose_radius_omits_rotation` (drops ρ·δ/2, which yields a false witness for the long rod);
`open_cells_for_cut`; `heading_not_periodic`; `unknown_is_free_in_inner`. Counterexample carried: CX-081 (clearance
cannot be tested with one scalar radius). Determinism: twice on one machine; debug and release.

## Considerations

- Kill gate KILL-FCLEAR and its fallback are named above; about 22% of home-door and base-width pairs have a
  margin under 149 mm (DESIGN arithmetic of the owning chapter from its hazard list), which is why refinement is
  part of the mechanism and why an unresolved share is a kill trigger.
- Memory envelope: four bytes per cell (two 16-bit one-sided values in millimetres).
- In a scanned scene the floor of the band is 2·(ε_field + u_reg), tens of millimetres; the correct output there
  is a measurement request, not a finer grid.
- The premise "the absorber is on every forward path" of the energy budget (construction C-31) may be discharged
  by a clearance certificate.

## References

Plan §10 C-33; §19.11; theorem card T-31; proof kind `complex-connectivity`; Appendix D.3 "Swept right-angle
corner", "Clearance sandwich"; MR-29; KA-161 to KA-163; CX-081; GATE-HAB-10; KILL-FCLEAR; A-18; C-07, C-28, C-31.
