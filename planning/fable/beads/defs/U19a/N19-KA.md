---
key: "N19-KA"
title: "registry: Habitat known answers KA-426 to KA-464 with truths, tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "habitat", "frob-conformance", "frob-scene", "solid", "registry", "test"]
parent: "WP-17"
deps: ["BD-025", "N38a-12", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N19-REF", "N19a-09", "N19a-14", "N19a-15", "N19a-16", "N19a-21", "N19a-22", "N19a-24", "N19a-25", "N19a-26", "N19a-31", "N19a-32", "N19a-35", "N19a-37", "N19b-05", "N19c-16", "N19c-22", "N19c-23", "N19c-27", "N19c-28", "N19c-29", "N38-KA", "N38a-13", "N38b-27", "N38b-28", "N38b-32", "N38b-53", "N38b-54", "NEG-12"]
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

In: the 39 rows as records of the suite's one registry file, in the record schema of N38a-12; one derivation
file per fixture in `docs/derivations/`; the `control.text` of every row whose source or owner states a negative
control, from which the seeding tool of N38b-05 writes the records of `crates/frob-conformance/registry/controls.toml`
with the row's number as `seed` and fills `control.nc` (this bead writes no row of `controls.toml`); a host-only
self-test that recomputes every arithmetic
truth from its premises; the lint that every row is cited by a test at its `due` milestone; and the record of the
discrepancies found between this list and the chapter text (four, and one between two chapters, now settled). Out:
implementing the tests (each row names the bead that carries it); the counterexample registry and the refusal
registry of this chapter (N19-CX and N19-REF); the runner that executes the chapter's eighteen cases as one
battery (N19c-28), which also enters the eight late answers of §19.21 and §19.22 (below); the G1 run per milestone and its account (N38a-13); the controls proposed for the 21 rows that
state none (N38b-32, which depends on this bead and registers them with `seed = "KA-nnn"`); the tolerances of
the nine rows printed "to set" (N38b-27); the settlement of the two arithmetic discrepancies KA-445 (N38b-53) and
KA-460 (N38b-54).

## Technical approach

1. **File and record.** The suite's one registry file `crates/frob-conformance/registry/known_answers.toml`
   (schema and loader: N38a-12), one `[[ka]]` record per row, field for field: `id`; `section` (where stated,
   with `owner` = the gate, `PO-HAB-nn` or `INV-HAB-nn` the row serves); `fixture` (one identity per physical
   case, Habitat fixtures named `habitat-<case>`); `same_as` (empty here); `case`; `truth` (value or formula with
   its `kind`, exact where possible: rationals as strings such as `"4/3"`); `tolerance` (`exact`, `abs`, `rel`,
   `printed-digits`, `gate` or `to-set`); `tier` (G0 laws and self-tests, G1 known answers, G2 parity, G3
   metamorphic, G4 chaos, G5 determinism) with `sites` (`host`, `auditor`, `referee`, `lockstep`, `strict`,
   `cuda`, `metal`; "all tiers" is written as the full list); `control` (`text` as printed and the `nc` ids of
   its records in `controls.toml`, or `none stated`); `derivation` (`docs/derivations/<fixture>.md`, whose front
   matter lists the KA ids it serves, with the formula, every symbol defined, the substituted numbers and the
   independent method that pins each truth); `label` (`DESIGN` arithmetic, or `FACT` with its source for rows
   resting on the literature: KA-437, KA-438, KA-443, KA-446, KA-447); `due` (the milestone at which a test must
   cite the row: the milestone of the gate or bead that carries it); `printed` (empty unless the source printed a
   value that a recomputation disagrees with: KA-438's "within 1%", KA-439's "dry limit", KA-445's "352 tiles, about
   360 kB", KA-460's M5 Ultra counts). The loader never rewrites a value (N38a-12's fifth rule).
2. **Citation lint and eligibility.** The "Carried by" column below is planning provenance (which bead's tests must
   cite the row), not a field of the record. A test cites a row by its id; the lint lists every row cited by no
   test at its `due` milestone. Eligibility is computed by N38a-12's loader: a row with tolerance `to-set`, with
   `none stated` as its control, or without its derivation file is an obligation and supports no gate; such
   rows are reported, never counted as passing.
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
| KA-432 | Metric calibration board measured by ray kernels | Measured lengths equal the board's known dimensions | The brick bound (GATE-HAB-04) | G1 · all tiers | (set: N19a-24) the 0.99 scale: lengths 1% short | N19a-24 |
| KA-433 | Normal-direction probe on each tagged surface | ∇φ points out of solids; floor normals are +z | to set ("within tolerance", GATE-HAB-04) | G1 · all tiers | (set: N19a-24) the inverted solid: normals point into solids | N19a-24 |
| KA-434 | Support test: a unit block released above each support surface | The block comes to rest on the surface | exact (verdict); GATE-HAB-04 | G1 · referee | A floor that exists only in the renderer fails | N19a-25 |
| KA-435 | Tile continuity: rays and swept wheels crossing brick and tile boundaries | No gap or step at a boundary | to set (GATE-HAB-04) | G1 · all tiers | (set: N19a-24) the planted step and gap at a brick boundary | N19a-24 |
| KA-436 | One floor tile in four states (matte dry, glossy dry, matte wet, glossy wet) | Changing the optical card leaves every physics digest unchanged in a run whose controller does not consume exteroception; changing the interface card leaves every ray-kernel output unchanged; changing the shared latent changes both in the recorded direction | exact, bit for bit within the determinism class (GATE-HAB-05) | G3 · all tiers | A build that derives friction from `r0` fails | BD-229, N19a-44 |
| KA-437 | Film-thickness update under a constant load | d(h⁻²)/dt = 4·F / (3π·η·R⁴), so h⁻²(t + Δt) = h⁻²(t) + 4·F·Δt / (3π·η·R⁴) is exact for a load constant over the step (PO-HAB-04, symbolic identity) | exact (symbolic) | G0 · host | none stated | BD-230; later unit |
| KA-438 | Squeeze-film time for a thick film to thin to h₁ = 10 µm | t = 3π·η·R⁴ / (4·F·h₁²): 0.038 s (R = 20 mm, F = 100 N, water at 1.0 mPa·s); 2.3 s (same pad, cooking oil at 60 mPa·s); 0.76 s (R = 60 mm, F = 400 N, water). Against a 0.3 s stance only the first reaches boundary contact | The list says within 1%; the chapter and GATE-HAB-06 say within 2% (see discrepancies) | G1 · all tiers | Doubling R must multiply the time by 16 | BD-230; cited by N19a-35, N19a-37 |
| KA-439 | Film-friction limits | μ = w(Λ)·μ_b + (1 − w(Λ))·μ_v with Λ = h/σ and w(Λ) = clamp((3 − Λ)/2, 0, 1): at Λ ≤ 1 (boundary limit; the list prints "dry limit", see discrepancies) μ = μ_b; at Λ ≥ 3 (thick film) μ = μ_v = min(μ_b, η·abs(v_t)·πR² / (h·F)); both limits recover the card coefficients | to set (GATE-HAB-06 states recovery, no number) | G1 · all tiers | none stated | BD-230; later unit |
| KA-440 | Castor at a rug edge | Horizontal force W·tan α with tan α = √(h·(2r − h)) / (r − h); r = 25 mm, h = 10 mm gives 4/3 of the load, 67 N for a 50 N castor; a 2 kg free slab resists with about 0.5 × 19.6 N ≈ 10 N and slides first | to set for the castor's force; exact for the verdict (GATE-HAB-08: the free slab slides); (set: BD-231) the free slab's onset at a push of 9.8 N within 10% | G1 · referee | A fixed step is climbed or blocks | BD-231; cited by N19a-31 |
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
| KA-455 | Three planted building bottlenecks (a 20 mm threshold, a door closer above push capability, an unmarked glass door) | Each is repaired by the expected kit with the policy identity unchanged (INV-HAB-08) | exact (verdicts) (GATE-HAB-11) | G1 · lockstep | (set: BD-369) a kit arm allowed to fine-tune the policy, which fails the policy-identity clause | BD-369 |
| KA-456 | A layout and its mirror image, and a retrofit variant, planted across a train and test split | Refused: a group never straddles a split (INV-HAB-09, `FROB-REF-311`) | exact (GATE-HAB-14) | G1 · host | Splitting by `SceneId` alone accepts it | BD-235 |
| KA-457 | Feasibility counts in split manifests | Counts of `witnessed`, `impossible` and `unresolved` scenes equal a recount; no scene resampled (INV-HAB-05) | exact (GATE-HAB-14) | G1 · host | none stated | N19a-26, N19a-45, BD-235 |
| KA-458 | Mutable-state leak: wet a tile in world i, reset, read world j and the next episode of world i | No trace outside the declared persistence (INV-HAB-03) | exact | G5 · all tiers | A pool indexed without the world address fails | N19a-09, N19a-43 |
| KA-459 | Dense scene memory, samples = volume / spacing³ and bytes = 2 × samples | Home (300 m³): 9 MB at 4 cm, 75 MB at 2 cm, 0.6 GB at 1 cm. Warehouse (19,200 m³): 0.6 GB, 4.8 GB, 38 GB. Building (57,600 m³): 1.8 GB, 14.4 GB, 115 GB. Band fraction min(1, (A/V)·t·s) for t = 8 to 20 voxels; 2 cm band bricks with labels 45–84 MB, 1.6–3.9 GB, 2.9–7.3 GB. Checked from a recount of compiled bricks | exact for dense figures; band figures are hypotheses measured at GATE-HAB-13 | G1 · host | none stated | BD-164 |
| KA-460 | Resident scene counts from the dense figures (12–16 GB of a 24 GB card; 85% of the measured working-set limit on the M5 Ultra, about 189 GiB) | One RTX 4090 and M5 Ultra, as listed: home at 2 cm 160–210 and about 2,500; home at 1 cm 20–27 and about 320; warehouse at 4 cm 20–27 and about 320; warehouse at 2 cm 2–3 and about 40; warehouse at 1 cm none and 4; building at 2 cm at most 1 and 13; building at 1 cm none and 1. The chapter's table gives about 2,700, 340, 340, 42, 5, 14 and 1 for the M5 Ultra (see discrepancies) | exact arithmetic; measured counts are recorded against the table (GATE-HAB-12) | G1 · host | none stated | BD-236 |
| KA-461 | One scene per world | 8,192 worlds each with its own 2 cm home is 8,192 × 75 MB = 614 GB, 26 times a 24 GB card and more than twice the working set of the large-memory machine | exact | G1 · host | none stated | BD-164; cited by N19a-03, N19a-09 |
| KA-462 | Variant directory size | One 32-bit slot and one 16-bit coarse sample per coarse cell, 6 bytes per cell: 0.44 MB for the home and 28 MB for the warehouse at the default spacings | to set | G1 · host | none stated | BD-164 |
| KA-463 | Brick pool against the dense field | The pooled field equals the dense field bit for bit at 10⁶ points per scene | exact (GATE-HAB-13) | G1 · all tiers | (set: BD-164) a directory remap with an off-by-one slot | BD-164 (the host test), N19c-17 (the G1 entry on the host and on each device tier, and the gate record) |
| KA-464 | Scene identity across platforms, 1,000 seeds per archetype | The same family, parameters, seed and compiler version give the same `SceneId` on Linux and macOS (INV-HAB-10); every scene is tagged `witnessed`, `impossible` or `unresolved`; at most 5% `unresolved`; the share of `impossible: intended` within one percentage point of the family's quota; no scene resampled | exact for identity; counts as stated (GATE-HAB-02) | G5 · host | none stated | N19a-20, N19a-30, N19a-45, BD-234 |

**Cells set by their owners.** Five of the 21 controls printed `none stated` are named by the rows' carriers and
carried here marked "(set: <bead>)" (KA-432, KA-433, KA-435 by N19a-24; KA-455 by BD-369; KA-463 by BD-164), and
BD-231 sets the tolerance of KA-440's free-slab clause. The other 16 controls are N38b-32's to register (its table
proposes one for each, and, where an owner's record exists, links or adds its own as a second record); the nine
tolerances stay with N38b-27, KA-440's castor force among them. The fail-closed reading holds for every open cell.

**A boundary-only control.** KA-444's printed control ("a 4 mm film of water is refused or sub-cycled at
admission", `FROB-REF-307`) restates a refusal, so the loader flags the row `boundary-only` (N38a-12) until
N38b-28 registers its mutation in place of it: the admission check removed, so that a 4 mm film is admitted and the
explicit update produces a negative thickness within ten steps (variant: the limit computed without the factor
3/4, 2.73 mm against 2.48 mm; recomputed at review).

**Late answers of the chapter, entered by N19c-28.** The two late subsections (§19.21 scaled instances, §19.22
incremental forks) state eight known answers without numbers. The battery bead N19c-28 enters them under the
provisional tags HKA-A to HKA-H with its own `ka` block request of count 8 in
`crates/frob-conformance/registry/allocations.toml` (N38a-40; owner N19c-28, section §19.21, §19.22), and maps the
tags to the issued numbers in one commit. They are listed here by tag so that the chapter's registry is complete;
this bead enters no row for them and requests no block. HKA-A anisotropic sphere (0.10 m and 0.20 m after
compilation; the interval [0.5·d, 2·d] before); HKA-B two planes under scales 1/2 and 3 (the foreground at 2 m);
HKA-C depth against an analytic plane on 10³ anisotropic instances (1 mm plus the brick bound); HKA-D fork cost
(1 − a) + a·d + b (0.079, 0.100, 0.128, 0.810; factors 12.7, 10.0, 7.8, 1.23); HKA-E dirty-cone counts (210 for
K = 20, 10.5%; 10,660 in the plane, 5.3%); HKA-F null fork; HKA-G a fork confined to one room; HKA-H a door swung
through a dry room (recomputed at review: HKA-D and HKA-E agree).

4. **Discrepancies recorded** (the record keeps the printed value in `printed` and the row is listed in the
   suite's reconciliation table until the settling bead closes it; the fail-closed reading is the one a gate
   states).
   - KA-438 tolerance: the list says within 1%; the chapter's known-answer table and both gates (GATE-HAB-06 and
     the film law's own gate) say within 2%. The gate governs: `tolerance = { kind = "gate", value = "2%" }`,
     `printed = "within 1%"`.
   - KA-445 tile count: the list says 352 tiles and about 360 kB; the chapter counts whole tiles of 0.32 m,
     19 × 19 = 361 and about 370 kB. Whole tiles are what is allocated: 361. The counting rule and the row's final
     truth are settled by N38b-53 (REC-04); until then the row stays `to-set` with `printed = "352 tiles, about
     360 kB"`.
   - KA-460 M5 Ultra counts: the list's figures correspond to dividing 189 by the scene size in GB; the chapter's
     correspond to about 203 GB (189 GiB), for example 203 GB / 75 MB ≈ 2,700 homes at 2 cm. The budget is stated
     as 189 GiB, so the chapter's figures are the consistent arithmetic; the record carries them as truth with the
     list's figures in `printed`, and N38b-54 (gigabytes against gibibytes at 85%) settles the rule.
   - KA-439 limit label: the list calls Λ ≤ 1 the "dry limit"; the chapter (§19.10.2) says the blend weights
     boundary contact fully below Λ = 1, so it is the boundary limit, μ = μ_b. The dry limit is a different
     clause: a film of thickness h₀ = 0 returns the dry card's μ_d bit for bit, and a squeezed film recovers to
     μ_b, not μ_d (BD-230's KA-406 and CX-178). The record's truth says "boundary limit", with
     `printed = "dry limit"`.
   - KA-439 form of the viscous term (a discrepancy between chapters, not with this list; settled): this row and
     the Habitat chapter cap μ_v at μ_b; the contact chapter states the Couette term uncapped above Λ = 3. The
     capped form governs, stated identically by the law's bead BD-230 and by N19b-05: without the cap the
     coefficient is unbounded as the load goes to zero, and this row is stated with the cap. The contact
     chapter's pure Couette shear for h ≥ 3σ is asserted where the cap does not bind, and a contact where it
     binds is flagged `viscous-capped`. BD-230, which has the coefficient, asserts the row; N19b-05 computes no
     coefficient.

## Acceptance criteria

- [ ] `known_answers.toml` holds the 39 records KA-426 to KA-464 with every field of the N38a-12 record (id,
      section, fixture, same_as, case, truth, tolerance, tier, sites, control, derivation, label, due, printed),
      and N38a-12's loader accepts them; a renumbered or reused id is rejected.
- [ ] Every row has its derivation file under `docs/derivations/` listing it in its front matter; after N38b-05's
      seeding, every row whose source or owner states a control has a non-empty `control.nc` (this bead wrote no
      record of `controls.toml`); the 16 rows still without one stay `none stated` (obligations for N38b-32) and
      are reported ineligible, never passed; the five owner-set controls carry the owner's text.
- [ ] The citation lint passes for rows due at M2 and lists, by bead, the later rows not yet cited.
- [ ] The report of unset tolerances lists exactly the rows marked "to set" (KA-433, KA-435, KA-439, KA-440's
      force, KA-444, KA-445, KA-449, KA-453's coefficients, KA-462) and shrinks only by a recorded decision
      (N38b-27).
- [ ] The four discrepancies with the list are recorded with both readings, the printed value in `printed` and
      the reading in force; KA-439's truth carries the capped viscous term that BD-230 settled.
- [ ] Expected values in the registry are literals with their derivation cited; none is produced by the code
      under test.
- [ ] `cargo test -p frob-conformance --test known_answers_habitat` (this bead's own test file; it is a registry
      self-test on the host, not a run of the G1 tier) recomputes every arithmetic truth listed under unit tests
      and matches the pinned values.

## Test plan

### Unit tests

`crates/frob-conformance/tests/known_answers_habitat.rs` (host only; the real registry file through N38a-12's
loader; planted registries are scratch copies labelled `synthetic`):

- `habitat_ka_count_39`: identifiers are consecutive from 426 to 464.
- `habitat_rows_cited_or_listed`: the citation lint over the workspace test list; every row whose `due` is M2
  (the GATE-HAB-01 rows KA-426 and KA-427, the brick layout KA-428, and the pool and memory rows KA-459 and
  KA-461 to KA-463 carried by BD-164) is cited once those beads close; the rest are listed by bead and milestone.
- `habitat_derivation_files_present`: each record's `derivation` exists and lists the record's id.
- `unset_tolerances_listed`: the unset set equals the expected nine.
- `none_stated_rows_ineligible`: of the 21 rows without a printed control (KA-427, KA-428, KA-432, KA-433, KA-435,
  KA-437, KA-439, KA-445, KA-449, KA-450, KA-451, KA-453, KA-454, KA-455, KA-457, KA-459, KA-460, KA-461,
  KA-462, KA-463, KA-464), the 16 not set by their owners are reported `obligation: control` until N38b-32
  registers theirs, and the five owner-set ones (KA-432, KA-433, KA-435, KA-455, KA-463) carry the owner's text.
- `arithmetic_rows_recompute`: rows that are pure arithmetic are recomputed in the test from their premises:
  0.16/65,535 = 2.44 µm; 1,024 and 128 bytes; t = 3π·η·R⁴/(4·F·h₁²) gives 0.0377 s, 2.26 s and 0.763 s;
  tan α = 4/3 and 0.75; ω₁ = 8.96 rad/s; 2√2·0.90 − 1.20 = 1.346 m; 35.4 mm + 39.3 mm = 74.6 mm; 240 × 160 × 32 =
  1,228,800 cells and 4.69 MiB; 40 + 780 + 9,880 = 10,700; 8,192 × 75 MB = 614 GB; √(2·9.81·0.020) = 0.63 m/s;
  the catenary sag 0.292 m from 1.2 = 2a·sinh(0.5/a) (a = 0.46954, sag 0.29234 m); h_max = 2.48 mm for water and
  9.98 mm for the 65 mm²/s oil; 0.44 MB and 28 MB at 6 bytes per coarse cell of 16 cm (KA-462); 189 GiB =
  202.9 GB and 202.9 GB / 75 MB = 2,706 (KA-460, both readings).
- `recompute_catches_planted_value` (negative control): a scratch copy of the registry with KA-448's truth edited
  to 1.400 m and KA-438's first time to 0.0400 s makes `arithmetic_rows_recompute` fail on exactly those two rows.
- `to_set_row_cannot_support_gate` (negative control): a planted gate citation of KA-433 while its tolerance is
  `to-set` is rejected by the loader.
- `boundary_only_ka444`: KA-444 is reported `boundary-only` while no mutation record of N38b-28 exists for it, and
  `eligible` once one does; the "factor 3/4 dropped" limit recomputes to 2.73 mm against 2.48 mm.

### End-to-end script and logging

`scripts/e2e/habitat_known_answers.sh` (header: `bead=N19-KA milestone=M2 labels=habitat,registry
gates=GATE-GAUNT-02 plants=edited-truth,to-set-at-gate`) sources the `STD-E2E` helper and has three stages.
Stage `registry` (the default, and the only stage this bead's own acceptance runs: this bead depends on no scene
verb, and a bead neither waits on the beads that wait on it nor calls a verb it does not depend on): the registry self-test and the lint (`cargo test -p frob-conformance --test
known_answers_habitat`), then the 39 rows selected through their tier targets (`FROB_TEST_SEED=<seed> cargo test
-p frob-conformance --test <tier> -- <row>`, `<tier>` = `g0_laws`, `g1_known_answers`, `g3_metamorphic` or
`g5_determinism`), each row's per-site outcome logged as the runner reports it (`unbound` until its carrier binds
it, never passed); and the two planted registry defects of the unit tests on scratch copies under
`target/e2e-scratch/habitat_known_answers/`. Stages `compile` and `accept` are hooks for the beads that depend on
the scene verbs and re-run this script (N19a-42 runs `--stage compile`, N19a-43 runs `--stage accept`); each compares
an envelope's figures with the registry: `compile` runs `frob --json scene compile` on the box-room fixture
(`--key hka-1`) for the brick bound and byte layout (KA-426, KA-428); `accept` runs, for the reflected room, the
preview form `frob --json scene accept --source fixtures/imports/asym-room-mirrored.glb --preview --key hka-2`,
because the reflected room does not compile (stage 2 refuses it with `FROB-REF-302`, so there is no pack to
accept), and asserts exit 3 with `FROB-REF-302`, det L = −1 and the off-plane residual 2 m (KA-429); and, for the
6 mm doorway, `frob --json scene compile fixtures/scenes/doorway-6mm.src --key hka-3c` then
`frob --json scene accept <id> --only enclosure-selftest --key hka-3`, asserting `unknown` with an outer overlap
of 2 mm and an inner separation of 14 mm (KA-430). A hook stage run before its verb exists logs `e2e_skip "needs
<bead>"` with the milestone, never a pass. It writes `target/e2e-logs/habitat_known_answers/<utc-timestamp>.ndjson`
and a transcript; each line carries `ts`, `script`, `bead` (`N19-KA`), `step`, `event`, the exact command line,
exit code, wall time, the envelope's `evidence` block where a verb ran, one `assert` per row with its id, measured
value and unit beside the pinned truth, tolerance, tier, site and eligibility, the SHA-256 of `known_answers.toml`,
and on failure the first differing value and the path of the derivation file or scratch copy. The final `summary`
gives counts of passed, failed, skipped, unbound, ineligible (by reason) and unset.

### Known answers and negative controls

This bead is the list. Its own negative controls are structural: a planted wrong truth is caught by the
independent recomputation, and a row with a tolerance of "to set" used to support a gate is rejected by the
loader. The rows' own controls are the planted defects of the "Negative control" column, applied by the beads
that carry them.

## Considerations

- Nine rows have tolerances still to set; setting one is a recorded decision, taken from measured values on the
  fixture, before the gate that needs it.
- Rows carried by later units (film, slosh, proxies, clearance, retrofit, hygiene, bank) are listed here in full
  so that the registry is complete even if those beads cite only numbers.
- Several truths rest on facts from the literature (Stefan's squeeze-film relation, linear sloshing theory,
  Huppert's spreading exponents) that are to be re-read from a primary source before a gate cites them.

## References

Plan Appendix M.18 (KA-426 to KA-464), §19.18 (the chapter's known-answer table), §19.19 (gates).
