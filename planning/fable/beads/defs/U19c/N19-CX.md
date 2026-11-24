---
key: "N19-CX"
title: "registry: Habitat counterexamples CX-184 to CX-199 as negative controls, plus four unnumbered late cases"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "habitat", "frob-scene", "solid", "registry", "test", "review-notes-open"]
parent: "WP-17"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N19-KA", "N19-REF", "N19c-11", "N19c-16", "N19c-27", "N19c-29", "N19c-33", "N19c-34", "N19c-38", "N19c-39", "N19c-40", "N19c-42", "N38b-04"]
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

In: the rows below as entries of the suite's one counterexample catalogue,
`crates/frob-conformance/registry/counterexamples.toml`, with the substance each needs to become a control (the rule
identifiers and gates it protects, the kind of control, one planted-defect identifier per row); the request for
catalogue numbers for the four late rows; a host test that recomputes every number; the chapter's script that shows
each planted defect fails its gate. Out, with the bead that has it: the catalogue's record schema and loader, and
the seeding tool that turns catalogue entries into rows of `crates/frob-conformance/registry/controls.toml`,
allocates their `NC-` identifiers and proves completeness by the join (N38b-05; this bead writes no row of
`controls.toml` by hand); the control runner and the rule that a gate with no control cannot pass (N38b-04); the
allocation of catalogue numbers (N38a-40); the mechanisms and their tests (each row names the bead that carries
it); known answers (N19-KA); refusals (N19-REF).

## Technical approach

1. **File and record.** Each row is an entry of `crates/frob-conformance/registry/counterexamples.toml` in the
   catalogue's record schema (N38b-05): `id` (the CX number), `shortcut`, `case`, `numeric` (false for a row whose
   case is given in words only), `plan_case` (`"words"` or `"numbers"`: the form of the case in the plan's text,
   immutable), `case_supplied_by` (empty, or the bead and fixture that supplied a numeric case for a word-only
   entry), `rule`, `rule_ids` (the gates, invariants, obligations, decisions and refusal codes named in the "rule
   it protects" column), `defined_in`, `same_as`, `label`, `control_kind`, `plant` and, for an entry appended after
   plan time, `slug`. `plant` names the planted defect: for the mutation kind, a Cargo feature under the umbrella
   `gauntlet-mutations` that puts the shortcut into the real component (never a mock, never compiled into a default
   or release build); for a row that is pure arithmetic, the planted wrong constant of its arithmetic test. DESIGN,
   correcting the first text of this bead, which kept a file of Habitat's own
   (`registry/counterexamples/habitat.toml`) with fields of its own (`mutant`, `caught_by`, `gate`, `bead`): a
   chapter's private registry file or row format is a lint failure of the suite, control identifiers are allocated
   once by the seeding tool, and "which test catches the plant" is the control's `gate` and `test` in
   `controls.toml`, derived by that tool. The "Carried by" column below is planning provenance (which bead's tests
   must fail under the plant), not a field of the record.
2. **Binding.** After this bead's entries are in the catalogue, the seeding tool of N38b-05 is re-run (append-only
   and idempotent) and writes one control per entry; the join of that tool proves every entry has exactly one
   control. For every gate among a row's `rule_ids` the control runner (N38b-04) must see the gate pass with the
   plant off and fail with it on. A row whose case is in words only (CX-187, CX-199; `plan_case = "words"`) stays
   `numeric = false` and is an obligation until its carrying bead supplies a fixture and records it in
   `case_supplied_by`. A row whose mechanism has not landed is `pending: <bead>` and is reported, not failed, until
   that bead's milestone.
3. **Rows** (the catalogue's own numbers).

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Carried by |
|---|---|---|---|---|
| CX-184 | Three labelled landmarks and a known length establish the registration of an imported scene | A = (0, 0, 0), B = (1, 0, 0), C = (0, 1, 0). The reflection (x, y, z) ↦ (x, y, −z) fixes all three, so a mirrored room registers with zero residual. A fourth landmark D = (0, 0, 1) maps to (0, 0, −1): residual 2 m | Four non-coplanar correspondences, LᵀL = c²·I and det L > 0 for any import that admits a parity change or an affine map; `FROB-REF-302`; PO-HAB-07; GATE-HAB-04 | N19a-21, N19c-04 |
| CX-185 | A gravity witness can stand in for the fourth landmark | If the three landmarks lie in a vertical plane, the reflection through that plane fixes them and preserves gravity; the mirrored room still registers | The fourth point must be non-coplanar: tetrahedron volume at least 1% of the cube of the largest separation (TARGET); `FROB-REF-302` | N19a-21 |
| CX-186 | A clearance verdict can be read from nominal surfaces | A nominal 6 mm doorway gap with 4 mm of possible outward error on each side: outer enclosures overlap by 2 mm (clearance not established); inner enclosures are 14 mm apart (penetration not witnessed). The nominal test says "clear"; the verdict is `unknown` | Clearance only from disjoint outer enclosures, penetration only from intersecting inner enclosures, otherwise `unknown`; INV-HAB-06; GATE-HAB-04 | N19a-22 |
| CX-187 | Overlapping swept volumes show a collision, and one penetrating world shows that a body must hit | Two bodies' spatial sweeps can overlap although the bodies are never in one place at one time; a universal penetration claim must cover every admitted world, not one (two tested traps; no numeric case) | Inner enclosures must intersect at the same time and pose in one jointly admissible world | N19a-22, N19b-33 |
| CX-188 | Quantizing the distance field to 16 bits over the whole scene range is fine | 16 bits over ±10 m resolve 20 m / 65,536 ≈ 0.31 mm: a force step of about 30 N at 10⁵ N/m. Over ±8 cm the step is about 2.4 µm and the ripple about 0.12 N per contact sample | Stored values are clamp(φ, −T, +T) with T = 4 fine voxels; ripple at most 1% of weight (GATE-HAB-01) | N19b-01, BD-162, BD-163 |
| CX-189 | A wet floor is a patch with a constant low friction coefficient | Time to thin a film to 10 µm, t = 3π·η·R⁴/(4·F·h₁²): 0.038 s (20 mm foot, 100 N, water); 2.3 s (same foot, cooking oil); 0.76 s (60 mm foot, 400 N, water). Against a stance of about 0.3 s the first reaches boundary contact and the other two do not, on the same patch | Film friction depends on dwell time, load and pad size (the film-friction law, GATE-HAB-06) | N19b-05, N19b-06 |
| CX-190 | A fixed step is a sufficient proxy for a rug | Pushing an undriven wheel over an edge takes W·tan α, tan α = √(h·(2r − h))/(r − h). r = 25 mm, h = 10 mm: 4/3 of the load, 67 N for a 50 N castor; a 2 kg rug resists with about 0.5 × 19.6 N ≈ 10 N. The rug slides and folds first; a fixed step is climbed or blocks | A rug is a thin free rigid slab with its own friction and an edge step (GATE-HAB-08) | N19b-03, N19b-18 |
| CX-191 | Film transport under a contact can be written with the indicator outside the divergence, 1·∇·(h·v̄) | Three cells h = (0, 1, 0), indicator on the middle cell, v̄ = +1: the masked update removes Δt/Δx from the middle cell and adds it nowhere; the face-flux form moves it into the right-hand cell | Conservative face-flux update; INV-HAB-07, PO-HAB-05; the mass gate of GATE-HAB-07 | N19b-09, N19b-10 |
| CX-192 | Living matter means a resolved fluid, cloth or cable solver in every training world | D3Q19 needs 152 bytes per cell: a 10 × 10 × 3 m room at 1 cm is 45.6 GB for one world, about 374 TB at 8,192 worlds; a 0.1–1 mm film needs 39 GB per m²; at 50 Hz with u = 1 m/s the lattice Mach number is 3.5 at 1 cm against a limit of 0.3; τ − ½ = 6 × 10⁻⁶ for water at 10 cm and 20 ms; at 5% activity a coupled world may cost at most 181 rigid worlds | ADR-HAB-07: three tiers, only Solid on the critical path; KILL-LIVING | N19b-04 |
| CX-193 | A scalar disc test decides whether a body fits | A 0.60 m wide base in a 0.90 m corridor with a right-angle turn: the inscribed disc says every length fits; the circumscribed disc says a 1.30 m body (diagonal 1.43 m) cannot. By L ≤ 2√2·a − 2b = 1.346 m the 1.30 m body passes and a 1.40 m body does not | Clearance complexes on SE(2) with witness, cut or `unresolved` (ADR-HAB-08); GATE-HAB-10 | N19b-26, N19b-35 |
| CX-194 | The default clearance lattice alone answers doorway fits | At Δ = 5 cm, 32 headings, ρ = 0.40 m the cell error is 74.6 mm, so a fit with total margin under 2e ≈ 149 mm is `unresolved`; with doors uniform in 0.70–0.90 m and bases in 0.45–0.70 m about 22% of pairs fall there, and they are the near fits | Local refinement is part of the mechanism (18.7 mm at 12.5 mm and 128 headings; 6.0 mm at 5 mm and 512); KILL-FCLEAR above 30% unresolved after refinement | N19b-29, N19b-25 |
| CX-195 | A static rejection settles whether a wheel can climb | r = 50 mm at a 20 mm edge needs μ = 4/3, statically impossible; a body arriving at √(2·g·h) = 0.63 m/s has the energy to rise 20 mm before losses. In an executed planar sill study 62 of 156 statically rejected cases succeeded dynamically | Static rejection and admission are candidates for a dynamic test, not substitutes; the retrofit search reruns the full procedure | N19b-40, N19b-41 |
| CX-196 | Splitting train and test scenes by `SceneId` prevents leakage | A layout and its mirror image have different `SceneId`s and the same aisles; a policy trained on one and scored on the other has seen the test | Splits are made over groups (base scene, mirrors, clutter and retrofit variants, shared brick pools, scans of one building); INV-HAB-09, `FROB-REF-311`; GATE-HAB-14 | BD-235, N19c-11, N19c-15 |
| CX-197 | Each world can have its own scene | 8,192 worlds each with its own 2 cm home is 8,192 × 75 MB = 614 GB: 26 times a 24 GB card and more than twice the working set of the large-memory machine; per-world state is only about 20–53 kB | ADR-HAB-05: one scene serves many worlds through a shared brick pool | N19c-16, BD-164 |
| CX-198 | A narrow band saves an order of magnitude of scene memory, and a 2 cm warehouse is 600 MB | A 60 × 40 × 8 m warehouse at 2 cm is 4.8 GB dense; 600 MB belongs to 4 cm. The band keeps min(1, (A/V)·t·s): a factor of one to five at 2 cm, 1.5 to 11 at 1 cm; a cluttered home gains least | The memory table; measured occupancy at GATE-HAB-13 | N19c-16, N19c-17 |
| CX-199 | Gloss and friction can share one material property | A glossy tile can give a bad depth sample and low traction for unrelated reasons; the four-state tile (matte dry, glossy dry, matte wet, glossy wet): an optical change must leave physics digests unchanged and a contact change must leave ray outputs unchanged (no numeric case) | ADR-HAB-03: two maps joined only by declared latents; GATE-HAB-05; control: a build that derives friction from `r0` | BD-229, N19a-44 |

**Kind, plant and label per entry (DESIGN).** `plant` is `<carrier>:cx-<nnn>` (or `<carrier>:<tag>`, lower case,
for an extension entry): the Cargo feature under `gauntlet-mutations` (for a `mutation`) or the `synthetic`
fixture (for the other kinds) under which the first carrier named builds the shortcut in the real component; the
carrier may refine the name, keeping the number or tag in it. A row whose rule names no gate (CX-185, CX-187,
CX-192, CX-194, CX-195, CX-197, CXH-C) gives `rule_ids` its identifiers and the defining section, which the seeding
tool resolves through `registry/rule_gates.toml` (CX-192 and CX-194 through their kill gates KILL-LIVING and
KILL-FCLEAR).

| Entry | `control_kind` | `plant` (the planted shortcut) | `label` |
|---|---|---|---|
| CX-184 | mutation | `N19a-21:cx-184` (registration from three landmarks, parity and det L unchecked) | DESIGN |
| CX-185 | mutation | `N19a-21:cx-185` (a gravity witness accepted as the fourth correspondence) | DESIGN |
| CX-186 | mutation | `N19a-22:cx-186` (a clearance verdict read from nominal surfaces) | DESIGN |
| CX-187 | planted-world | `N19a-22:cx-187` (two sweeps overlapping in space at different times; `numeric = false`, an obligation until N19a-22 or N19b-33 supplies the fixture) | DESIGN |
| CX-188 | mutation | `N19b-01:cx-188` (16-bit quantization over the whole scene range, no clamp) | DESIGN |
| CX-189 | mutation | `N19b-05:cx-189` (a wet patch as a constant low friction coefficient) | DESIGN |
| CX-190 | mutation | `N19b-03:cx-190` (a rug as a fixed step) | DESIGN |
| CX-191 | mutation | `N19b-09:cx-191` (the indicator outside the divergence) | DESIGN |
| CX-192 | planted-input | `N19b-04:cx-192` (a request for a resolved solver in every training world, which the tier rule must refuse) | DESIGN |
| CX-193 | mutation | `N19b-26:cx-193` (a scalar disc test in place of the clearance complex) | DESIGN |
| CX-194 | mutation | `N19b-29:cx-194` (the refinement loop returns after selecting candidates, so the default lattice answers alone; caught by N19b-29's `doorway_60mm`, which stays `Unresolved`) | DESIGN |
| CX-195 | mutation | `N19b-40:cx-195` (a static rejection taken as final, no dynamic test) | FACT: the executed planar sill study (62 of 156); pilot record `PILOT-PT-16` (re-run by BD-043 at M0). The pair is entered from that record when the row is written: `rerun = true` with `rerun_receipt` = the receipt of the record's sill result, which BD-043's review log names for this entry, where `pilots::rerun_status("PILOT-PT-16")` reads `reproduced`; `rerun = false` while it reads `not-run` or `differs` (N38b-05 refuses any other pair, `rerun-disagrees-with-pilot`) |
| CX-196 | mutation | `N19c-11:cx-196` (train and test split by `SceneId`) | DESIGN |
| CX-197 | planted-input | `N19c-16:cx-197` (a request for one 2 cm scene per world at 8,192 worlds) | DESIGN |
| CX-198 | planted-input | `N19c-16:cx-198` (the memory claim "a 2 cm warehouse is 600 MB" offered to the scene byte model) | DESIGN |
| CX-199 | mutation | `BD-229:cx-199` (a build that derives friction from `r0`, the plan's stated control; `numeric = false` until BD-229 or N19a-44 records the four-state tile fixture) | DESIGN |
| CXH-A | mutation | `N19c-38:cxh-a` (a local signed distance read as metres) | DESIGN |
| CXH-B | mutation | `N19c-39:cxh-b` (ray hits sorted and gated in local parameter) | DESIGN |
| CXH-C | planted-input | `N19c-33:cxh-c` (a fork-reuse claim over the whole world, against the ceiling (1 − a) + b) | DESIGN |
| CXH-D | mutation | `N19c-34:cxh-d` (the neighbours of a recomputed tile marked dirty) | DESIGN |

A FACT entry's control is written by the seeding tool as an `obligation` until `rerun = true` with a receipt.

4. **Extension rows without a catalogue number.** The two subsections added after the catalogue was compiled state
   four counterexamples with no number. This bead does not invent one: it files a request for a `cx` block of four
   in the allocation ledger `crates/frob-conformance/registry/allocations.toml` (N38a-40: a `[[block]]` entry with
   `registry = "cx"`, `count = 4`, `owner = "N19-CX"`, `section = "§19.21, §19.22"`; the allocator fills `first` and
   `last`, nobody types them). Until the block is issued the rows are cited by the provisional tags below; once it
   exists the tags are mapped to its numbers in table order (CXH-A first) in one commit, each entry keeps its tag in
   `slug`, and a tag that still appears as an `id`, in a test name or in a derivation file at this bead's milestone
   fails the allocation check (`provisional-tag-past-due`).

| Provisional tag | The tempting shortcut | The smallest case that breaks it | The rule it protects | Carried by |
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

- [ ] `registry/counterexamples.toml` holds CX-184 to CX-199 (sixteen entries) and the four extension entries, each in the catalogue's record schema with `rule_ids`, `control_kind` and `plant`; N38b-05's loader accepts them and the chapter count of sixteen holds; no Habitat file exists under `registry/counterexamples/`.
- [ ] The four extension entries carry numbers from a block that N38a-40's ledger issued to this bead, with their tags in `slug`; `registry::alloc::validate` passes and no tag remains outside `slug`.
- [ ] After the seeding tool of N38b-05 is re-run, each of the twenty entries has exactly one control in `controls.toml` whose `seed` names it, the tool's second run changes no byte, and the join passes.
- [ ] Every entry carries the `control_kind`, `plant` and `label` of the table above; CX-195 is FACT, its label names `PILOT-PT-16`, and its pair equals that record's derived status (`rerun = true` with the sill receipt where `reproduced`, otherwise `rerun = false`); CX-187 and CX-199 seed as obligations until their fixtures are recorded in `case_supplied_by`.
- [ ] The case and rule cells keep every number and identifier of Appendix N.16 (the dwell "about 0.3 s", "after refinement", the glossy-tile statement of CX-199).
- [ ] Arithmetic-only rows (CX-188, CX-192, CX-194, CX-197, CX-198, CXH-C) are bound to host tests that pin every printed number.
- [ ] No plant is reachable from a default or release build (a build with `gauntlet-mutations` enabled outside the test profile is a compile error, BD-025).
- [ ] `scripts/e2e/habitat_negative_controls.sh` exits 0.

## Test plan

### Unit tests
`crates/frob-conformance/tests/habitat_counterexamples.rs` (host; the real catalogue through N38b-05's loader):
- `cx_rows_complete`: the Habitat entries are CX-184 to CX-199 and four appended entries whose `slug`s are CXH-A to
  CXH-D; before the block is issued the four are present under their tags and the allocation check reports them
  as awaiting a block, not as an error.
- `cx_entries_have_plants`: every entry with `numeric = true` has a non-empty `plant` and `control_kind`; CX-187 and
  CX-199 are `plan_case = "words"`.
- `cx_controls_seeded`: after the seeding tool has run, each entry has exactly one control whose `seed` names it.
- `cx197_arithmetic`: 8,192 × 75 MB = 614.4 GB; 614.4 / 24 = 25.6 (printed as 26 times a card); the working-set limit
  of 222.7 GiB is 239.1 GB and 614.4 / 239.1 = 2.57 (more than twice); per-world state 20–53 kB × 8,192 is at most 0.43 GB.
- `cx198_arithmetic`: 60 × 40 × 8 / 0.02³ × 2 bytes = 4.8 GB; at 0.04 m, 0.6 GB.
- `cxh_a_sphere`, `cxh_b_planes`, `cxh_c_bound`: the numbers of the extension rows (0.10 and 0.20 m; local parameters
  4 and 1; cost 0.81 and factor 1.23).
- `cx_kinds_plants_labels`: the twenty entries' `control_kind`, `plant` and `label` equal the table; every `plant`
  names its carrier; CX-195 carries its source and names `PILOT-PT-16`, and on a scratch register its pair loads
  only when it equals `pilots::rerun_status("PILOT-PT-16")` (`rerun = true` with the record's receipt when
  `reproduced`; `rerun = false` when `not-run` or `differs`).
- `cx188_189_190_arithmetic`: 20 m / 65,536 = 0.305 mm and 30.5 N at 10⁵ N/m; 0.16 m / 65,536 = 2.44 µm; t =
  3π·η·R⁴/(4·F·h₁²) = 0.0377 s, 2.26 s (η = 60 mPa·s) and 0.763 s; tan α = 20/15 = 4/3, 66.7 N against 9.8 N.
- `cx193_194_arithmetic`: 2√2·0.90 − 2·0.60 = 1.346 m; √(1.30² + 0.60²) = 1.432 m; e = Δ/√2 + ρ·π/n: 74.6 mm,
  18.7 mm, 6.0 mm.
- `no_plant_in_release`: the default and release feature sets of the workspace contain none of the plants' features.

All run as `FROB_TEST_SEED=0 cargo test -p frob-conformance --test habitat_counterexamples`.

### End-to-end script and logging
`scripts/e2e/habitat_negative_controls.sh` (STD-E2E helper; it calls only cargo targets of its dependencies):
runs the unit tests above; runs N38b-05's seeding tool, `cargo test -p frob-conformance --test seed_controls --
--ignored seed`, twice, asserting that the second run leaves the digest of the committed `controls.toml`
unchanged; runs the join, `cargo test -p frob-conformance --test seed_join`; reads
`registry/control_history.ndjson` and reports, per entry, its control's state (`runnable`, `obligation`, `stale`
while the carrier has not built the plant) and its last recorded outcome; then plants the registry's own controls
below on scratch copies under `target/e2e-scratch/habitat_negative_controls/`. Running each plant against its
gate (plant off, plant on, in the form of ADR-GAUNT-01) is the control runner's (N38b-04), invoked by the carriers'
own scripts, and is not repeated here. Every line carries `ts`, `script`, `bead`, `step`, `event`, the exact
command, exit code and wall time; one `metric` line per entry with the CX number (or tag), the control's `NC-`
identifier, the plant, the gates it protects, its state and last outcome; the digest of `controls.toml` before and
after the second seeding; on failure the first missing entry or unresolved gate with the registry path. The
`summary` gives entries, controls runnable, obligations, stale controls with their carriers, and findings. Logs go to
`target/e2e-logs/habitat_negative_controls/<utc>.ndjson` with a transcript beside it.

### Known answers and negative controls
This bead is the counterexample catalogue of the chapter, from which its negative controls are seeded. Its own
controls, each on a scratch copy: an entry with an empty `plant` (`cx_entries_have_plants`); CX-195 relabelled
DESIGN (`cx_kinds_plants_labels`); CX-195 written `rerun = true` while its pilot record reads `differs`
(N38b-05's `rerun-disagrees-with-pilot`); the 30 N force step written 3 N (`cx188_189_190_arithmetic`); an extension
entry numbered outside N19-CX's block (the ledger's `unallocated-number`). A plant that is a no-op (the code under
it equals the correct code) is reported by the control runner as `missed`, a failure of the control and not a
pass (N38b-04). Each row links its known answers:
CX-184 ↔ KA-429, CX-186 ↔ KA-430, CX-188 ↔ KA-427, CX-189 ↔ KA-438, CX-190 ↔ KA-440, CX-191 ↔ KA-441, CX-193 ↔ KA-448,
CX-194 ↔ KA-449 and KA-451, CX-195 ↔ KA-453, CX-196 ↔ KA-456, CX-197 ↔ KA-461, CX-198 ↔ KA-459.

## Considerations

- A counterexample kept only as prose decays; the binding check is what keeps each one alive as a test.
- CX-187 and CX-199 have no numeric case in the plan; their carrying beads construct one and record it here.
- Owed by the carriers and the control runner, not by this registry: for every row whose mechanism exists, the
  gate's conformance run fails with the plant enabled and passes with it disabled; at each milestone exit no row
  whose carrying bead's milestone has passed is still `stale` or pending.
- Mutants must be cheap to build: each is a small alternative function selected by a test-only constructor, not a fork of a crate.
- The catalogue is append-only: a withdrawn design stays as a row so that the defect cannot return unnoticed.

## References

Appendix N.16 (CX-184 to CX-199); §19.21, §19.22 (unnumbered cases); §21.20 (CX-227, CX-229); §38 (negative-control registry).
