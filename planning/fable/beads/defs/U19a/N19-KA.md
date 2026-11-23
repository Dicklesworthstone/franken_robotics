---
key: "N19-KA"
title: "registry: Habitat known answers KA-426 to KA-464 with truths, tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "habitat", "frob-scene", "solid", "registry", "test"]
parent: "WP-17"
deps: ["STD-TEST"]
needs: []
related: ["BD-025", "BD-146", "BD-223", "BD-318", "BD-163", "BD-164", "BD-228", "BD-229", "BD-230", "BD-231", "BD-233", "BD-234", "BD-235", "BD-236", "BD-369", "BD-413", "BD-414", "BD-415", "N19-CX", "N19-REF", "N19c-28", "N19b-05", "N19b-06"]
gates: []
plan_refs: ["Appendix M.18", "§19.18", "§19.19"]
---
## Background

A known answer is a case whose truth is fixed in advance by a derivation or a construction, against which the
implementation is tested: "recovers KA-n within tolerance" is the only permitted wording of a Known-answer claim.
Two rules make a known answer worth having. Its expected value is pinned from the derivation and is never
regenerated from the code under test. And wherever one is stated it carries a negative control: a planted defect
the test must catch, because a gate that nothing can fail validates nothing.

This bead is the registry of the 39 known answers of the Habitat chapter (scenes, living worlds, clearance,
retrofits). It exists so that none is lost when the work is split across many beads: mechanism beads cite these
numbers and state the values they need inline; this registry is the complete list and the place the conformance
harness loads them from.

## Scope

In: the 39 rows as machine-readable registry data, the loader check that every row is bound to a test, and the
record of the discrepancies found between this list and the chapter text (three, and one between two chapters). Out: implementing the tests (each row
names the bead that carries it); the counterexample registry and the refusal registry of this chapter (N19-CX
and N19-REF); the runner that executes the chapter's eighteen cases as one battery (N19c-28).

## Technical approach

1. **File.** `crates/frob-conformance/registry/known_answers/habitat.toml` (DESIGN path): one entry per row with
   `id`, `where`, `case`, `truth`, `tolerance`, `tier`, `site`, `negative_control`, `test` (the test's symbol) and
   `bead`. Tiers are the six verification tiers: G0 laws and self-tests, G1 known answers, G2 parity, G3
   metamorphic, G4 chaos, G5 determinism. Sites: `host`, `auditor`, `referee`, `lockstep`, or `all tiers`.
2. **Binding check.** A harness test loads the file and asserts that each row's `test` exists in the workspace
   test list, or that the row is marked `pending: <bead>`; a row whose tolerance is "to set" is listed in a report
   of unset tolerances and cannot support a gate until set.
3. **Rows.** "Carried by" names the bead whose test plan holds the row; keys beginning N19a are in this unit;
   BD keys are seed beads; "later unit" marks rows whose mechanism belongs to the living-world, clearance,
   retrofit, hygiene and world-bank subsections.

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Carried by |
|---|---|---|---|---|---|---|
| KA-426 | Compiled scene field against its source geometry, sampled at 10⁶ surface points | Error at most the recorded brick bound (the source-chart enclosure, plus the trilinear interpolation error on the brick lattice, plus half a quantization step) and at most half a fine voxel; analytic shapes compiled to bricks exact to the interpolation bound; analytic overlays exact to round-off | The brick bound; half a fine voxel (GATE-HAB-01) | G1 · all tiers | A brick with its bound halved fails | N19a-14, N19a-16, N19a-17, N19a-42 |
| KA-427 | Quantization ripple: a 10 kg block at rest on a brick floor with no overlay | Stored values are clamp(φ, −T, +T) in 16 bits with T = 8 cm at 2 cm spacing: step 0.16 m / 65,535 ≈ 2.4 µm, a ripple of at most half a step, about 0.12 N per contact sample at 10⁵ N/m; normal-force ripple at most 1% of weight | 1% of weight on every tier (TARGET, GATE-HAB-01) | G1 · all tiers | None stated; the withdrawn quantization over ±10 m (20 m / 65,536 ≈ 0.31 mm, about 30 N per step) is the defect this row guards | N19a-14, N19a-25, N19a-42 |
| KA-428 | Brick byte layout | 8³ fine samples of 16 bits are 1,024 bytes; a palette of at most four region identifiers with a two-bit selector per sample is 128 bytes, 12.5% over the distance samples | exact | G1 · host | none stated | N19a-14, N19a-15 |
| KA-429 | Reflected asymmetric room at registration | Landmarks A = (0, 0, 0), B = (1, 0, 0), C = (0, 1, 0); the map (x, y, z) ↦ (x, y, −z) fixes all three; the fourth landmark D = (0, 0, 1) maps to (0, 0, −1), residual 2 m; four-point registration rejects the import (LᵀL = c²·I and det L > 0 fail), `FROB-REF-302` | exact (verdict and residual); GATE-HAB-04 | G1 · host | The three-point check accepts the reflection with zero residual | N19a-21, BD-228, N19a-43 |
| KA-430 | Doorway with a nominal 6 mm gap and 4 mm of possible outward error on each side | Outer enclosures overlap by 2 mm, so clearance is not established; inner enclosures are 14 mm apart, so penetration is not witnessed; verdict `unknown` | exact; GATE-HAB-04 | G1 · host | A nominal-surface test says "clear" | N19a-22, BD-228, N19a-43 |
| KA-431 | Door-clearance gauges at every door of clear width w with field bound b | The go gauge of width w − 2b passes and the no-go gauge of width w + 2b is blocked | exact (verdicts) on 100 scenes per archetype (GATE-HAB-04) | G1 · all tiers | A door narrowed by 3 bounds blocks the go gauge | N19a-25, N19a-43 |
| KA-432 | Metric calibration board measured by ray kernels | Measured lengths equal the board's known dimensions | The brick bound (GATE-HAB-04) | G1 · all tiers | none stated | N19a-24 |
| KA-433 | Normal-direction probe on each tagged surface | ∇φ points out of solids; floor normals are +z | to set ("within tolerance", GATE-HAB-04) | G1 · all tiers | none stated | N19a-24 |
| KA-434 | Support test: a unit block released above each support surface | The block comes to rest on the surface | exact (verdict); GATE-HAB-04 | G1 · referee | A floor that exists only in the renderer fails | N19a-25 |
| KA-435 | Tile continuity: rays and swept wheels crossing brick and tile boundaries | No gap or step at a boundary | to set (GATE-HAB-04) | G1 · all tiers | none stated | N19a-24 |
| KA-436 | One floor tile in four states (matte dry, glossy dry, matte wet, glossy wet) | Changing the optical card leaves every physics digest unchanged in a run whose controller does not consume exteroception; changing the interface card leaves every ray-kernel output unchanged; changing the shared latent changes both in the recorded direction | exact, bit for bit within the determinism class (GATE-HAB-05) | G3 · all tiers | A build that derives friction from `r0` fails | BD-229, N19a-44 |
| KA-437 | Film-thickness update under a constant load | d(h⁻²)/dt = 4·F / (3π·η·R⁴), so h⁻²(t + Δt) = h⁻²(t) + 4·F·Δt / (3π·η·R⁴) is exact for a load constant over the step (PO-HAB-04, symbolic identity) | exact (symbolic) | G0 · host | none stated | BD-230; later unit |
| KA-438 | Squeeze-film time for a thick film to thin to h₁ = 10 µm | t = 3π·η·R⁴ / (4·F·h₁²): 0.038 s (R = 20 mm, F = 100 N, water at 1.0 mPa·s); 2.3 s (same pad, cooking oil at 60 mPa·s); 0.76 s (R = 60 mm, F = 400 N, water). Against a 0.3 s stance only the first reaches boundary contact | The list says within 1%; the chapter and GATE-HAB-06 say within 2% (see discrepancies) | G1 · all tiers | Doubling R must multiply the time by 16 | BD-230; cited by N19a-35, N19a-37 |
| KA-439 | Film-friction limits | μ = w(Λ)·μ_b + (1 − w(Λ))·μ_v with Λ = h/σ and w(Λ) = clamp((3 − Λ)/2, 0, 1): at Λ ≤ 1 (dry limit) μ = μ_b; at Λ ≥ 3 (thick film) μ = μ_v = min(μ_b, η·abs(v_t)·πR² / (h·F)); both limits recover the card coefficients | to set (GATE-HAB-06 states recovery, no number) | G1 · all tiers | none stated | BD-230; later unit |
| KA-440 | Castor at a rug edge | Horizontal force W·tan α with tan α = √(h·(2r − h)) / (r − h); r = 25 mm, h = 10 mm gives 4/3 of the load, 67 N for a 50 N castor; a 2 kg free slab resists with about 0.5 × 19.6 N ≈ 10 N and slides first | to set for the force; exact for the verdict (GATE-HAB-08: the free slab slides) | G1 · referee | A fixed step is climbed or blocks | BD-231; cited by N19a-31 |
| KA-441 | Three-cell film transport, h = (0, 1, 0), contact indicator on the middle cell, v̄ = +1 | The face-flux form moves Δt/Δx from the middle cell into the right-hand cell; total volume is conserved (PO-HAB-05, telescoping identity) | exact in exact arithmetic; compensated-sum audit per run | G1 · all tiers | The masked non-conservative form 1·∇·(h·v̄) removes Δt/Δx from the middle cell and adds it nowhere | BD-413; later unit |
| KA-442 | Advected film with pickup and redeposition | Total volume, cells plus reservoirs, is constant (INV-HAB-07); no trace in another world | Relative volume residual at most 10⁻⁶ over 10⁵ steps; no cross-world leak in 10⁶ world-episodes (GATE-HAB-07) | G1 · all tiers | The non-conservative control fails the mass gate | BD-413; later unit |
| KA-443 | Viscous spreading of a constant volume on a flat floor | Radius ∝ t^(1/8) in the axisymmetric case; front position ∝ t^(1/5) in the planar case | Exponent within 2% of 1/8 (GATE-HAB-07) | G1 · all tiers | Removing the h³ factor changes the exponent | BD-413; later unit |
| KA-444 | Film thickness limit of the explicit update | h_max = (3ν·Δx² / (4·g·Δt))^(1/3): 2.5 mm for water at Δx = 2 cm and Δt = 2 ms; 10 mm for a 65 mm²/s oil | to set | G1 · host | A 4 mm film of water is refused or sub-cycled at admission (`FROB-REF-307`) | BD-413; later unit |
| KA-445 | Film tile storage | A tile of 16 × 16 cells is 1,024 bytes of thickness; a 6 m × 6 m floor fully wet at 2 cm: the list says 352 tiles, about 360 kB per world; the chapter counts whole tiles, 19 × 19 = 361, about 370 kB (see discrepancies); a 1 m² spill is about 16 kB | to set | G1 · host | none stated | BD-413; later unit |
| KA-446 | Slosh, first antisymmetric mode of a rectangular container | ω_n² = ((2n − 1)·π·g / L)·tanh((2n − 1)·π·h / L); for L = 0.30 m and h = 0.10 m, ω₁ = 8.96 rad/s (1.43 Hz) | within 3% at three fill depths (GATE-HAB-08) | G1 · all tiers | Halving the fill must lower the frequency | BD-414; cited by N19a-37 |
| KA-447 | Hanging chain, 1.2 m between supports 1.0 m apart | Sag 0.292 m (the catenary, with parameter a solving 1.2 = 2a·sinh(0.5/a)); 20 links reproduce it | within 2% at 20 links (TARGET, GATE-HAB-08) | G1 · referee | A chain with no bending limit and one link fails | BD-415; later unit |
| KA-448 | Right-angle corridor: a rectangle of width b and length L between corridors of width a (b < a) | Passes if and only if L ≤ 2√2·a − 2b; derivation: with the outer long side touching both outer walls at inclination θ, L ≤ (a·sin θ + a·cos θ − b) / (sin θ·cos θ), smallest at θ = 45°; 1.346 m at a = 0.90 m, b = 0.60 m (a 1.30 m body passes, a 1.40 m body does not). The complex returns a witness below the limit minus its band and a cut above the limit plus its band | Bracketed by witness and cut within the refined band on 100 (a, b) pairs (GATE-HAB-10) | G1 · host | Inscribed and circumscribed disc tests are both wrong | BD-233; cited by BD-234, N19a-32 |
| KA-449 | Clearance error term of a cell | e = (Δ/2)·√2 + ρ·(δ/2) + ε_field + u_reg; at Δ = 5 cm, 32 headings and ρ = 0.40 m the first two terms are 35.4 mm + 39.3 mm = 74.6 mm; 18.7 mm at Δ = 12.5 mm and 128 headings; 6.0 mm at Δ = 5 mm and 512 headings | to set | G1 · host | none stated | BD-233; cited by BD-234 |
| KA-450 | Size of the default clearance lattice | A 12 m × 8 m floor at Δ = 5 cm with 32 headings is 240 × 160 × 32 = 1,228,800 cells, 4.69 MiB at four bytes per cell | exact | G1 · host | none stated | BD-233 |
| KA-451 | Doorway fit at the default lattice | A body centred in a doorway has clearance equal to half the total margin and a witness needs c_lo > 0, so a fit with total margin under 2e ≈ 149 mm is `unresolved` before local refinement | exact (verdict) | G1 · host | none stated | BD-233; cited by BD-234, N19a-36, N19a-45 |
| KA-452 | Clearance witnesses and cuts against a brute-force grid at four times the resolution | Zero false witnesses and zero false cuts on the seven archetype fixtures (INV-HAB-06, PO-HAB-03); the checker accepts every emitted certificate and rejects six mutated ones | exact (GATE-HAB-10; one false witness or cut trips KILL-FCLEAR) | G0 · auditor | Six mutated certificates must be rejected | BD-233 |
| KA-453 | Driven wheel at a 20 mm threshold, quasi-static, no help from another axle | Friction needed tan α = √(h·(2r − h)) / (r − h): 4/3 at r = 50 mm (statically impossible on any floor card), 0.75 at r = 100 mm, 1/12 on a wedge whose run is twelve times its rise; a body arriving at √(2·g·h) = 0.63 m/s has the energy to rise 20 mm before losses. The retrofit search returns the ramp, the design search returns the wheel, the report shows both | to set for the coefficients; both arms reproduce the closed form (GATE-HAB-11) | G1 · referee | none stated | BD-369; cited by N19a-31, N19a-34 |
| KA-454 | Unfiltered retrofit kit count for forty catalog entries and kits of at most three | 40 + 780 + 9,880 = 10,700 | exact | G1 · host | none stated | BD-369 |
| KA-455 | Three planted building bottlenecks (a 20 mm threshold, a door closer above push capability, an unmarked glass door) | Each is repaired by the expected kit with the policy identity unchanged (INV-HAB-08) | exact (verdicts) (GATE-HAB-11) | G1 · lockstep | none stated | BD-369 |
| KA-456 | A layout and its mirror image, and a retrofit variant, planted across a train and test split | Refused: a group never straddles a split (INV-HAB-09, `FROB-REF-311`) | exact (GATE-HAB-14) | G1 · host | Splitting by `SceneId` alone accepts it | BD-235 |
| KA-457 | Feasibility counts in split manifests | Counts of `witnessed`, `impossible` and `unresolved` scenes equal a recount; no scene resampled (INV-HAB-05) | exact (GATE-HAB-14) | G1 · host | none stated | N19a-26, N19a-45, BD-235 |
| KA-458 | Mutable-state leak: wet a tile in world i, reset, read world j and the next episode of world i | No trace outside the declared persistence (INV-HAB-03) | exact | G5 · all tiers | A pool indexed without the world address fails | N19a-09, N19a-43 |
| KA-459 | Dense scene memory, samples = volume / spacing³ and bytes = 2 × samples | Home (300 m³): 9 MB at 4 cm, 75 MB at 2 cm, 0.6 GB at 1 cm. Warehouse (19,200 m³): 0.6 GB, 4.8 GB, 38 GB. Building (57,600 m³): 1.8 GB, 14.4 GB, 115 GB. Band fraction min(1, (A/V)·t·s) for t = 8 to 20 voxels; 2 cm band bricks with labels 45–84 MB, 1.6–3.9 GB, 2.9–7.3 GB. Checked from a recount of compiled bricks | exact for dense figures; band figures are hypotheses measured at GATE-HAB-13 | G1 · host | none stated | BD-164 |
| KA-460 | Resident scene counts from the dense figures (12–16 GB of a 24 GB card; 85% of the measured working-set limit on the M5 Ultra, about 189 GiB) | One RTX 4090 and M5 Ultra, as listed: home at 2 cm 160–210 and about 2,500; home at 1 cm 20–27 and about 320; warehouse at 4 cm 20–27 and about 320; warehouse at 2 cm 2–3 and about 40; warehouse at 1 cm none and 4; building at 2 cm at most 1 and 13; building at 1 cm none and 1. The chapter's table gives about 2,700, 340, 340, 42, 5, 14 and 1 for the M5 Ultra (see discrepancies) | exact arithmetic; measured counts are recorded against the table (GATE-HAB-12) | G1 · host | none stated | BD-236 |
| KA-461 | One scene per world | 8,192 worlds each with its own 2 cm home is 8,192 × 75 MB = 614 GB, 26 times a 24 GB card and more than twice the working set of the large-memory machine | exact | G1 · host | none stated | BD-164; cited by N19a-03, N19a-09 |
| KA-462 | Variant directory size | One 32-bit slot and one 16-bit coarse sample per coarse cell, 6 bytes per cell: 0.44 MB for the home and 28 MB for the warehouse at the default spacings | to set | G1 · host | none stated | BD-164 |
| KA-463 | Brick pool against the dense field | The pooled field equals the dense field bit for bit at 10⁶ points per scene | exact (GATE-HAB-13) | G1 · all tiers | none stated | BD-164 |
| KA-464 | Scene identity across platforms, 1,000 seeds per archetype | The same family, parameters, seed and compiler version give the same `SceneId` on Linux and macOS (INV-HAB-10); every scene is tagged `witnessed`, `impossible` or `unresolved`; at most 5% `unresolved`; the share of `impossible: intended` within one percentage point of the family's quota; no scene resampled | exact for identity; counts as stated (GATE-HAB-02) | G5 · host | none stated | N19a-20, N19a-30, N19a-45, BD-234 |

4. **Discrepancies recorded** (the registry file carries both readings with a `conflict` note until the owner
   rules; the fail-closed reading is the one a gate states).
   - KA-438 tolerance: the list says within 1%; the chapter's known-answer table and both gates (GATE-HAB-06 and
     the film law's own gate) say within 2%. The gate governs: 2%.
   - KA-445 tile count: the list says 352 tiles and about 360 kB; the chapter counts whole tiles of 0.32 m,
     19 × 19 = 361 and about 370 kB. Whole tiles are what is allocated: 361.
   - KA-460 M5 Ultra counts: the list's figures correspond to dividing 189 by the scene size in GB; the chapter's
     correspond to about 203 GB (189 GiB), for example 203 GB / 75 MB ≈ 2,700 homes at 2 cm. The budget is stated
     as 189 GiB, so the chapter's figures are the consistent arithmetic.
   - KA-439 form of the viscous term (a discrepancy between chapters, not with this list): this row and the
     Habitat chapter cap μ_v at μ_b; the contact chapter, which owns the law, states the Couette term uncapped
     above Λ = 3. The law's bead (BD-230) settles it. Until it does, the scene-side beads (N19b-05, N19b-06)
     assert only the two limits both forms share, on fixtures whose speed keeps μ_v below μ_b.

## Acceptance criteria

- [ ] The registry file holds 39 entries, KA-426 to KA-464, each with case, truth, tolerance, tier, site, negative
      control (or "none stated") and the carrying bead.
- [ ] The binding check passes: every row has a live test or a `pending` mark naming a bead; no row is unbound.
- [ ] The report of unset tolerances lists exactly the rows marked "to set" (KA-433, KA-435, KA-439, KA-440's
      force, KA-444, KA-445, KA-449, KA-453's coefficients, KA-462) and shrinks only by a recorded decision.
- [ ] The three discrepancies with the list, and the open form of KA-439, are recorded with both readings and the reading in force.
- [ ] Expected values in the registry are literals with their derivation cited; none is produced by the code
      under test (a lint forbids a registry value written by a test run).

## Test plan

### Unit tests

- `habitat_ka_count_39`: identifiers are consecutive from 426 to 464.
- `every_row_bound_or_pending`: the binding check.
- `unset_tolerances_listed`: the unset set equals the expected nine.
- `arithmetic_rows_recompute`: rows that are pure arithmetic are recomputed in the test from their premises:
  0.16/65,535 = 2.44 µm; 1,024 and 128 bytes; t = 3π·η·R⁴/(4·F·h₁²) gives 0.0377 s, 2.26 s and 0.763 s;
  tan α = 4/3 and 0.75; ω₁ = 8.96 rad/s; 2√2·0.90 − 1.20 = 1.346 m; 35.4 mm + 39.3 mm = 74.6 mm; 240 × 160 × 32 =
  1,228,800 cells and 4.69 MiB; 40 + 780 + 9,880 = 10,700; 8,192 × 75 MB = 614 GB; √(2·9.81·0.020) = 0.63 m/s;
  the catenary sag 0.292 m from 1.2 = 2a·sinh(0.5/a).
- `regenerated_value_rejected` (negative control): a registry entry whose expected value carries a "written by
  test run" marker fails the lint.

### End-to-end script and logging

`scripts/e2e/habitat_known_answers.sh`: `frob --json conformance known-answers --chapter habitat --list` and
`--run --tier G1 --site host`; asserts 39 rows listed, the host-site rows that are bound pass, and pending rows
are reported as pending, not passed. Logs one `assert` line per row with its identifier, measured value,
tolerance, tier and verdict, and a `summary` with counts of passed, failed, pending and unset.

### Known answers and negative controls

This bead is the list. Its own negative control is structural: a row with a tolerance of "to set" used to support
a gate must be rejected by the harness.

## Considerations

- Nine rows have tolerances still to set; setting one is a recorded decision, taken from measured values on the
  fixture, before the gate that needs it.
- Rows carried by later units (film, slosh, proxies, clearance, retrofit, hygiene, bank) are listed here in full
  so that the registry is complete even if those beads cite only numbers.
- Several truths rest on facts from the literature (Stefan's squeeze-film relation, linear sloshing theory,
  Huppert's spreading exponents) that are to be re-read from a primary source before a gate cites them.

## References

Plan Appendix M.18 (KA-426 to KA-464), §19.18 (the chapter's known-answer table), §19.19 (gates).
