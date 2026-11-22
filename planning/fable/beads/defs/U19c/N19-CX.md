---
key: "N19-CX"
title: "registry: Habitat counterexamples CX-184 to CX-199 as negative controls, plus four unnumbered late cases"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "habitat", "frob-scene", "solid", "registry", "test"]
parent: "WP-17"
deps: ["BD-025", "STD-TEST"]
needs: ["§19.21, §19.22, Appendix N: catalogue numbers for the two counterexamples of §19.22 (anisotropic sphere, two planes on one ray) and for the strong form refuted in §19.21, which were added after the catalogue was compiled"]
related: ["N19-KA", "N19-REF", "N19c-27", "N19c-28", "BD-146", "BD-223", "BD-318"]
gates: []
plan_refs: ["Appendix N.16", "§19.21", "§19.22", "§21.20", "§38"]
---
## Background

A **counterexample** in this project is the smallest case that breaks a tempting shortcut. Each one is kept for two
uses. It explains a design rule (why four landmarks and not three, why groups and not scene identifiers). And it seeds
the **negative-control registry**: for every gate there must be a planted defect that the gate's test catches, because
a gate that nothing can fail validates nothing. The shortcut itself, implemented as a deliberately wrong variant
("mutant"), is the planted defect; the counterexample's numbers are the input on which the mutant and the correct
implementation must differ.

This bead is the registry of the sixteen counterexamples of the Habitat chapter (scenes, living worlds, clearance,
retrofits), CX-184 to CX-199, plus the counterexamples stated in the two subsections added after the catalogue was
compiled (far-field reuse across forks; scaled instances), which have no catalogue number yet. It exists so that no
case is lost when the mechanisms are spread over many beads. Mechanism beads cite these numbers and state the values
inline; this registry is the complete list and the place the conformance harness loads the mutants from.

## Scope

In: the rows as machine-readable data; one mutant per row where the shortcut is implementable; the binding check that
each row's mutant is caught by a named test; the extension rows without numbers. Out: the mechanisms and their tests
(each row names the bead that carries it); known answers (N19-KA); refusals (N19-REF).

## Technical approach

1. **File.** `crates/frob-conformance/registry/counterexamples/habitat.toml` (DESIGN path): `id`, `shortcut`, `case`,
   `rule`, `defined_in`, `mutant` (a cargo feature or a test-only constructor that builds the wrong variant; never
   compiled into a release binary), `caught_by` (test symbol), `gate`, `bead`.
2. **Binding check.** A harness test asserts for each row that (a) the correct implementation passes `caught_by`,
   and (b) with `mutant` enabled the same test fails. A row with no implementable mutant (pure arithmetic) is bound
   to an arithmetic test that pins the numbers. Rows whose mechanism has not landed are `pending: <bead>`.
3. **Rows** (the catalogue's own numbers).

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Carried by |
|---|---|---|---|---|
| CX-184 | Three labelled landmarks and a known length establish the registration of an imported scene | A = (0, 0, 0), B = (1, 0, 0), C = (0, 1, 0). The reflection (x, y, z) ↦ (x, y, −z) fixes all three, so a mirrored room registers with zero residual. A fourth landmark D = (0, 0, 1) maps to (0, 0, −1): residual 2 m | Four non-coplanar correspondences, LᵀL = c²·I and det L > 0 for any import that admits a parity change or an affine map; `FROB-REF-302`; PO-HAB-07; GATE-HAB-04 | N19a-21, N19c-04 |
| CX-185 | A gravity witness can stand in for the fourth landmark | If the three landmarks lie in a vertical plane, the reflection through that plane fixes them and preserves gravity; the mirrored room still registers | The fourth point must be non-coplanar: tetrahedron volume at least 1% of the cube of the largest separation (TARGET); `FROB-REF-302` | N19a-21 |
| CX-186 | A clearance verdict can be read from nominal surfaces | A nominal 6 mm doorway gap with 4 mm of possible outward error on each side: outer enclosures overlap by 2 mm (clearance not established); inner enclosures are 14 mm apart (penetration not witnessed). The nominal test says "clear"; the verdict is `unknown` | Clearance only from disjoint outer enclosures, penetration only from intersecting inner enclosures, otherwise `unknown`; INV-HAB-06; GATE-HAB-04 | N19a-22 |
| CX-187 | Overlapping swept volumes show a collision, and one penetrating world shows that a body must hit | Two bodies' spatial sweeps can overlap although the bodies are never in one place at one time; a universal penetration claim must cover every admitted world, not one (two tested traps; no numeric case) | Inner enclosures must intersect at the same time and pose in one jointly admissible world | N19a-22, N19b-33 |
| CX-188 | Quantizing the distance field to 16 bits over the whole scene range is fine | 16 bits over ±10 m resolve 20 m / 65,536 ≈ 0.31 mm: a force step of about 30 N at 10⁵ N/m. Over ±8 cm the step is about 2.4 µm and the ripple about 0.12 N per contact sample | Stored values are clamp(φ, −T, +T) with T = 4 fine voxels; ripple at most 1% of weight (GATE-HAB-01) | N19b-01, BD-162, BD-163 |
| CX-189 | A wet floor is a patch with a constant low friction coefficient | Time to thin a film to 10 µm, t = 3π·η·R⁴/(4·F·h₁²): 0.038 s (20 mm foot, 100 N, water); 2.3 s (same foot, cooking oil); 0.76 s (60 mm foot, 400 N, water). Against a 0.3 s stance only the first reaches boundary contact, on the same patch | Film friction depends on dwell time, load and pad size (GATE-HAB-06) | N19b-05, N19b-06 |
| CX-190 | A fixed step is a sufficient proxy for a rug | Pushing an undriven wheel over an edge takes W·tan α, tan α = √(h·(2r − h))/(r − h). r = 25 mm, h = 10 mm: 4/3 of the load, 67 N for a 50 N castor; a 2 kg rug resists with about 0.5 × 19.6 N ≈ 10 N. The rug slides and folds first; a fixed step is climbed or blocks | A rug is a thin free rigid slab with its own friction and an edge step (GATE-HAB-08) | N19b-03, N19b-18 |
| CX-191 | Film transport under a contact can be written with the indicator outside the divergence, 1·∇·(h·v̄) | Three cells h = (0, 1, 0), indicator on the middle cell, v̄ = +1: the masked update removes Δt/Δx from the middle cell and adds it nowhere; the face-flux form moves it into the right-hand cell | Conservative face-flux update; INV-HAB-07, PO-HAB-05; the mass gate of GATE-HAB-07 | N19b-09, N19b-10 |
| CX-192 | Living matter means a resolved fluid, cloth or cable solver in every training world | D3Q19 needs 152 bytes per cell: a 10 × 10 × 3 m room at 1 cm is 45.6 GB for one world, about 374 TB at 8,192 worlds; a 0.1–1 mm film needs 39 GB per m²; at 50 Hz with u = 1 m/s the lattice Mach number is 3.5 at 1 cm against a limit of 0.3; τ − ½ = 6 × 10⁻⁶ for water at 10 cm and 20 ms; at 5% activity a coupled world may cost at most 181 rigid worlds | ADR-HAB-07: three tiers, only Solid on the critical path; KILL-LIVING | N19b-04 |
| CX-193 | A scalar disc test decides whether a body fits | A 0.60 m wide base in a 0.90 m corridor with a right-angle turn: the inscribed disc says every length fits; the circumscribed disc says a 1.30 m body (diagonal 1.43 m) cannot. By L ≤ 2√2·a − 2b = 1.346 m the 1.30 m body passes and a 1.40 m body does not | Clearance complexes on SE(2) with witness, cut or `unresolved` (ADR-HAB-08); GATE-HAB-10 | N19b-26, N19b-35 |
| CX-194 | The default clearance lattice alone answers doorway fits | At Δ = 5 cm, 32 headings, ρ = 0.40 m the cell error is 74.6 mm, so a fit with total margin under 2e ≈ 149 mm is `unresolved`; with doors uniform in 0.70–0.90 m and bases in 0.45–0.70 m about 22% of pairs fall there | Local refinement is part of the mechanism (18.7 mm at 12.5 mm and 128 headings; 6.0 mm at 5 mm and 512); KILL-FCLEAR above 30% unresolved | N19b-25, N19b-29 |
| CX-195 | A static rejection settles whether a wheel can climb | r = 50 mm at a 20 mm edge needs μ = 4/3, statically impossible; a body arriving at √(2·g·h) = 0.63 m/s has the energy to rise 20 mm before losses. In an executed planar sill study 62 of 156 statically rejected cases succeeded dynamically | Static rejection and admission are candidates for a dynamic test; the retrofit search reruns the full procedure | N19b-40, N19b-41 |
| CX-196 | Splitting train and test scenes by `SceneId` prevents leakage | A layout and its mirror image have different `SceneId`s and the same aisles; a policy trained on one and scored on the other has seen the test | Splits are made over groups (base scene, mirrors, clutter and retrofit variants, shared brick pools, scans of one building); INV-HAB-09, `FROB-REF-311`; GATE-HAB-14 | BD-235, N19c-11, N19c-15 |
| CX-197 | Each world can have its own scene | 8,192 worlds each with its own 2 cm home is 8,192 × 75 MB = 614 GB: 26 times a 24 GB card and more than twice the working set of the large-memory machine; per-world state is only about 20–53 kB | ADR-HAB-05: one scene serves many worlds through a shared brick pool | N19c-16, BD-164 |
| CX-198 | A narrow band saves an order of magnitude of scene memory, and a 2 cm warehouse is 600 MB | A 60 × 40 × 8 m warehouse at 2 cm is 4.8 GB dense; 600 MB belongs to 4 cm. The band keeps min(1, (A/V)·t·s): a factor of one to five at 2 cm, 1.5 to 11 at 1 cm; a cluttered home gains least | The memory table; measured occupancy at GATE-HAB-13 | N19c-16, N19c-17 |
| CX-199 | Gloss and friction can share one material property | The four-state tile (matte dry, glossy dry, matte wet, glossy wet): an optical change must leave physics digests unchanged and a contact change must leave ray outputs unchanged (no numeric case) | ADR-HAB-03: two maps joined only by declared latents; GATE-HAB-05; control: a build that derives friction from `r0` | BD-229, N19a-44 |

4. **Extension rows without a catalogue number** (identifiers local to this registry until Appendix N assigns numbers).

| Local id | The tempting shortcut | The smallest case that breaks it | The rule it protects | Carried by |
|---|---|---|---|---|
| CXH-A | A local signed distance under a scaled instance is the distance in metres | Unit sphere scaled by (2, 0.5, 1). At world point (0, 0.6, 0) the local distance is 0.20 and the clearance is 0.10 m: read as metres, it admits a 0.15 m probe that intersects. At (2.2, 0, 0) local 0.10, clearance 0.20 m. Both lie in [0.5·d, 2·d] | A local distance d bounds the world distance to [σ_min(A)·d, σ_max(A)·d]; equality only for uniform scale; GATE-HAB-16 | N19c-38, N19c-40 |
| CXH-B | Ray hits can be sorted, range-gated or used as optical depth in each instance's local parameter | An opaque foreground plane at 2 m in an instance of scale 1/2 reports local parameter 4; a background plane at 3 m in an instance of scale 3 reports 1. Sorting local values returns the occluded plane; a 2.5 m range gate on local values discards the foreground and keeps the background | s_W = s_L/‖A⁻¹·d‖, converted before sort, clip and depth; GATE-HAB-16 | N19c-39, N19c-40 |
| CXH-C | A fork should recompute only what its change touches, over the whole world | With a = 0.20 of fork compute in field tiles, a dirty fraction of zero still costs (1 − a) + b = 0.81 of a full recomputation, a factor of 1.23: the robot, its contacts, sensors and controller dominate and always differ from the parent | Reuse is limited to field tiles and enabled only where the phase ledger shows a ≥ 0.8; KILL-FARFIELD | N19c-33, N19c-34 |
| CXH-D | A dirty tile's neighbours are dirty | A door swung through a dry room: the tiles it overlaps are recomputed, their content equals the parent's, and no neighbour becomes dirty, because keys are built from content digests, not from the fact of recomputation | TileKey over content digests; the dirty set shrinks when a disturbance dies out; GATE-HAB-15 | N19c-34, N19c-36 |

   Related rows of another chapter that Habitat beads cite (not owned here): CX-227 (brick directories need minimal
   perfect hashing: a dense directory is 0.3 MB for a 10 × 10 × 3 m interior at 2 cm and 18.8 MB for the warehouse,
   under 1.5% of the scene; a hash adds a verifying read to every lookup), CX-229 (zero-copy CPU reads of live GPU
   state on unified memory).

## Acceptance criteria

- [ ] The registry file holds CX-184 to CX-199 (sixteen rows) and the four extension rows, each with shortcut, case, rule and carrying bead.
- [ ] Every row with an implementable shortcut has a mutant that is caught by its named test; the binding check fails if a mutant passes.
- [ ] Arithmetic-only rows (CX-188, CX-192, CX-194, CX-197, CX-198, CXH-C) are bound to tests that pin every printed number.
- [ ] No mutant is reachable from a release build (checked by a feature audit).
- [ ] At each milestone exit, no row whose carrying bead's milestone has passed is still pending.

## Test plan

### Unit tests
- `cx_rows_complete`: the id set equals CX-184..CX-199 plus CXH-A..CXH-D.
- `cx_mutants_are_caught`: for each bound row, runs `caught_by` against the mutant and expects failure; against the
  real implementation and expects success.
- `cx197_arithmetic`: 8,192 × 75 MB = 614.4 GB; 614.4 / 24 = 25.6 (printed as 26 times a card); the working-set limit
  of 222.7 GiB is 239.1 GB and 614.4 / 239.1 = 2.57 (more than twice); per-world state 20–53 kB × 8,192 is at most 0.43 GB.
- `cx198_arithmetic`: 60 × 40 × 8 / 0.02³ × 2 bytes = 4.8 GB; at 0.04 m, 0.6 GB.
- `cxh_a_sphere`, `cxh_b_planes`, `cxh_c_bound`: the numbers of the extension rows (0.10 and 0.20 m; local parameters
  4 and 1; cost 0.81 and factor 1.23).
- `no_mutant_in_release`: scanning the release feature set finds no `mutant-*` feature.

### End-to-end script and logging
`scripts/e2e/habitat_negative_controls.sh` (STD-E2E helper): runs `frob --json conformance run --registry
counterexamples/habitat` (the Gauntlet runner), which executes each landed row twice (real, mutant) and reports per
row `caught: true|false`. The log has one `assert` event per row with the row id, the gate it protects, the measured
value on each arm and the expected difference; the `summary` gives rows caught, rows pending and gates touched. Logs go
to `target/e2e-logs/habitat_negative_controls/<utc>.ndjson`.

### Known answers and negative controls
This bead is the negative-control registry of the chapter. Its own control: a row whose mutant is edited to equal the
correct code must make the binding check fail ("mutant not distinguishable"). Each row links its known answers:
CX-184 ↔ KA-429, CX-186 ↔ KA-430, CX-188 ↔ KA-427, CX-189 ↔ KA-438, CX-190 ↔ KA-440, CX-191 ↔ KA-441, CX-193 ↔ KA-448,
CX-194 ↔ KA-449 and KA-451, CX-195 ↔ KA-453, CX-196 ↔ KA-456, CX-197 ↔ KA-461, CX-198 ↔ KA-459.

## Considerations

- A counterexample kept only as prose decays; the binding check is what keeps each one alive as a test.
- CX-187 and CX-199 have no numeric case in the plan; their carrying beads construct one and record it here.
- Mutants must be cheap to build: each is a small alternative function selected by a test-only constructor, not a fork of a crate.
- The catalogue is append-only: a withdrawn design stays as a row so that the defect cannot return unnoticed.

## References

Appendix N.16 (CX-184 to CX-199); §19.21, §19.22 (unnumbered cases); §21.20 (CX-227, CX-229); §38 (negative-control registry).
