---
key: "N18-KA"
title: "registry: Traction known answers KA-387 to KA-425 with truth, tolerance, tier and negative control"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: []
related: ["BD-085", "BD-086", "BD-154", "BD-161", "BD-168", "BD-230", "BD-367", "BD-386"]
gates: []
plan_refs: ["Appendix M.17", "§18.16", "§18.17", "§38.6"]
---
## Background

A known answer is a case whose truth is known by hand or in closed form, checked against a pinned oracle that is
never regenerated from the candidate. The verification program runs them at tier G1 (known answers), with some rows
at G0 (laws), G2 (parity), G3 (metamorphic), G4 (chaos) and G5 (determinism). A gate that nothing can fail
validates nothing, so every row that has a plausible wrong implementation carries a **negative control**: a planted
defect the test must catch.

This bead enters the 39 Traction rows into the known-answer registry of `frob-conformance` as data, so that every
mechanism bead can cite a row by number and so that no row is lost when the work is split over many beads. It
implements no physics. Sites: `host` (exact or double-precision arithmetic in a test), `auditor` (the offline
checker), `referee`, `strict` (`batch-cpu` under the `strict` profile), `lockstep` (throughput tiers), `all tiers`.

## Scope

In: the 39 rows below as registry entries with the fields `ka`, `case`, `truth`, `tolerance`, `tier`, `sites`,
`negative_control`, `gate`, `carrier` (the bead whose test runs it); a completeness test. Out: the fixtures and
tests themselves (the carrier beads); rows of other chapters that touch contact (listed at the end for
cross-reference).

## Technical approach

Registry entries live in `crates/frob-conformance/registry/known_answers/traction.toml` (one table per row). A row
whose tolerance the plan leaves open is entered as `tolerance = "to-set"` with the gate that sets it; the carrier
bead must replace it with a number and record it as a TARGET before its gate can pass. Constants: g = 9.81 m/s².

| KA | Case | Truth | Tolerance | Tier · sites | Negative control | Carrier |
|---|---|---|---|---|---|---|
| KA-387 | Block on an incline of angle α, friction μ | Stays if tan α < μ, slides if tan α > μ; creep below threshold at most the stated stiction speed (zero under CL-H) | Measured threshold within 1% of μ | G1 · strict, lockstep, referee | none stated (carriers add: friction disabled) | BD-085, BD-161, BD-168 |
| KA-388 | Sphere rolling down an incline without slip | Acceleration (5/7)·g·sin α | to set (GATE-TRAC-01); BD-085 sets 0.5% | G1 · strict, lockstep, referee | none stated | BD-085, BD-161 |
| KA-389 | Disk rolling on a plane | v = ω·r; path equals Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)), δ = ω·Δt, sinc(0) = 1 | v = ω·r within 0.1% | G1 · strict, lockstep, referee | none stated | BD-085, BD-161 |
| KA-390 | Near-straight differential drive (admission test of the rolling oracle): heading 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, 400 steps of 1/400 s | Translation about (0.8342, 0.5514) m from the closed form | to set | G2 · host | `fs-mbd::planar_drive` at the pinned commit, reported to return zero translation (FACT to recheck); not an oracle until it passes | BD-033 |
| KA-391 | Box stack, ten high | At rest for 10 s, drift below 1 mm | 1 mm | G1 · strict, lockstep | none stated | BD-161, BD-168 |
| KA-392 | Dropped body, inelastic law | Total mechanical energy never increases | exact (sign) | G1 · strict, lockstep, referee | none stated | BD-080, BD-085 |
| KA-393 | Capsule against a spherical distance field | Gap error at most the brick's interpolation bound | the stated interpolation bound | G1 · strict, lockstep | none stated | BD-166, BD-161 |
| KA-394 | Mecanum base commanded sideways | Moves sideways with yaw below 1° | 1° | G1 · strict, lockstep | none stated | BD-167, BD-161 |
| KA-395 | Ring on a cone | Slides if and only if tan α > μ | to set (GATE-TRAC-01); verdict exact at tan α = μ ± 5% | G1 · strict, lockstep, referee | none stated | BD-085 |
| KA-396 | Painlevé rod | The referee reports the indeterminate configuration (`FROB-REF-224`) and returns no number | exact (verdict) | G1 · referee | A solver that returns one of several distinct motions | BD-085 |
| KA-397 | Disk of radius a spinning in place, uniform pressure | Stops at t = 3·a·ω₀/(4·μ·g): 0.153 s for a = 0.05 m, ω₀ = 20 rad/s, μ = 0.5 | 1% (GATE-TRAC-04) | G1 · strict, lockstep | none stated | BD-154 |
| KA-398 | Effective patch radius of the spin row | Uniform pressure on a disk of radius a: capacity (2/3)·μ·N·a, r_eff = 2a/3; Hertzian: r_eff = 3π·a/16 ≈ 0.59·a | exact (derivation on file, PO-TRAC-04) | G0 · host | none stated | N18a-27 |
| KA-399 | Sliding at v while spinning at ω on the four-dimensional cone | Tangential force μ·N·v/√(v² + (r_eff·ω)²); μ·N/√2 at v = r_eff·ω | 0.5% of μ·N/√2 (GATE-TRAC-04) | G1 · strict, lockstep | Independent slide and spin rows (41% more in magnitude) | BD-154 |
| KA-400 | Thin ring sliding and spinning (r_eff = a, I = m·a²) | v/(a·ω) constant; both stop at t = √(v₀² + a²·ω₀²)/(μ·g): 0.288 s for v₀ = a·ω₀ = 1 m/s, μ = 0.5 | 1% (GATE-TRAC-04) | G1 · strict, lockstep | Independent slide and spin rows (stop at 0.204 s) | BD-154 |
| KA-401 | Traction along a diagonal of the tangent plane | Maximum friction μ·N in every direction | 0.5% of μ·N (GATE-TRAC-04) | G1 · strict, lockstep | The pyramid cone, which gives √2·μ·N | BD-154 |
| KA-402 | Joint stiction under a torque ramp τ = k·t | Breakaway at t = τ_s/k | 1% (GATE-TRAC-04) | G1 · strict, lockstep | none stated | N18a-32, BD-084, BD-155 |
| KA-403 | Backlash crossing and closure: rotor J = 10⁻⁴ kg·m² at 5 rad/s, ratio N = 10, gap β = 0.5°, output inertia 0.05 kg·m², inelastic | Crossing time β·N/ω₀ = 17.5 ms; output speed J·N·ω₀/(J·N² + I_out) = 0.083 rad/s | 1% (GATE-TRAC-04) | G1 · strict, lockstep | Backlash as a dead zone on the torque command | N18a-32, BD-084, BD-155 |
| KA-404 | Stefan squeeze to 10 µm from a thick film: dh/dt = −2·N·h³/(3π·η·a⁴), 1/h² − 1/h₀² = 4·N·t/(3π·η·a⁴) | Water 1 mPa·s, a = 2 cm, 100 N: 0.038 s; oil 60 mPa·s, same: 2.3 s; water, a = 6 cm, 400 N: 0.76 s | 2% (GATE-TRAC-05) | G1 · strict, lockstep | A constant coefficient per floor texel | BD-230 |
| KA-405 | Sled on a full film (h ≥ 3σ) | Couette shear F = η·v·A/h | 10⁻⁶ relative in `f64` (GATE-TRAC-05) | G1 · strict, lockstep | none stated | BD-230 |
| KA-406 | Film law with no film (h₀ = 0) | The dry card μ_d, bit for bit | exact (bits) | G1 · all tiers | none stated | BD-230 |
| KA-407 | Film-law limits on 10⁴ random states | μ_b for h ≤ σ; pure Couette for h ≥ 3σ; continuous between; a squeezed film recovers to μ_b, not μ_d | to set | G1 · strict, lockstep | none stated | BD-230 |
| KA-408 | Thin barrier: sphere r = 2 mm at 5 m/s, h = 2 ms (10 mm of travel); 2 mm plate 1 mm ahead; clearing takes 1 + 4 + 2 = 7 mm | The step encloses a collision though both endpoints are clear (3 mm beyond the far face); zero missed hits in 10⁶ randomized crossings under transactional profiles | exact (count; GATE-TRAC-06) | G1 · referee, strict | The endpoint-only query, which misses | BD-086, BD-161 |
| KA-409 | Torque-driven wheel over a sharp sill | Feature sequence flat, edge, top; work and impulse converge under step halving; no unexplained couple | 1% of refined values at h/4; no couple above 10⁻⁹ N·m in `f64` (GATE-TRAC-06) | G1 · referee, strict | Tangential memory kept on a changed feature | BD-086, BD-161 |
| KA-410 | Clearance with registration errors | 6 mm nominal gap, 4 mm outward error per side: outer enclosures overlap by 2 mm, answer `unknown`. 10 mm nominal, same errors: `clear` with 2 mm to spare | exact (verdict) | G1 · host | The nominal gap taken as clearance | N18a-03 |
| KA-411 | Enclosure answers on 10⁴ generated pairs with known truth | No false `clear`, no false `penetrates`; every certificate re-checked offline | exact (counts; GATE-TRAC-07) | G1 · host, auditor | none stated | N18a-03, BD-233 |
| KA-412 | Two swept volumes that overlap in space while the bodies occupy the shared region at different times | Not a contact: the test is per time slab | exact (verdict) | G1 · host | A test on whole-sweep volumes | N18a-03 |
| KA-413 | Sole straddling a low-grip strip (μ 0.25 inside, 0.8 outside, uniform pressure), boundary at offset z, sole width w | High-grip share clip(½ + z/w, 0, 1): capacity 0.525·N at z = 0 for any width; at z = 12 mm, 0.690·N for w = 40 mm and 0.619·N for w = 70 mm | to set; N18a-28 sets 0.5% | G1 · strict, lockstep | Contact area taken as the predictor of traction | N18a-28 |
| KA-414 | Load-weighted slip metric | Duplicating a quadrature point and halving its weights changes nothing | to set; N18b-32 sets 10⁻¹² relative in `f64` | G3 · all tiers | none stated | N18a-28 |
| KA-415 | Plate sinkage on soft soil, p = (k_c/b + k_φ)·zⁿ | z = (p/(k_c/b + k_φ))^(1/n); b = 0.1 m, k_c = 1 kPa/mⁿ⁻¹, k_φ = 1,500 kPa/mⁿ, n = 1.1, p = 20 kPa: z = 19.6 mm (illustrative) | law 10⁻⁶ in `f64`; simulated sinkage 2% (GATE-TRAC-08) | G1 · strict, lockstep | none stated | soil bead of the other Traction unit (BD-366) |
| KA-416 | Track thrust at slip i, τ = (c + σ·tan φ)·(1 − exp(−j/K)), j = i·x | F = (A·c + W·tan φ)·(1 − (K/(i·L))·(1 − exp(−i·L/K))); two tracks 0.5 m × 0.1 m, 500 N, c = 1 kPa, φ = 30°, K = 25 mm: limit 389 N, 293 N at 20% slip (illustrative) | law 10⁻⁶ in `f64`; drawbar pull 2% (GATE-TRAC-08) | G1 · strict, lockstep | none stated | BD-366 |
| KA-417 | Lateral brush tyre force, ‖F_y‖ = μ·N·(3ζ − 3ζ² + ζ³), ζ = C_α·‖tan α‖/(3·μ·N) ≤ 1, μ·N beyond | Slope C_α at zero slip; saturation μ·N at tan α = 3·μ·N/C_α | 10⁻⁶ in `f64` (GATE-TRAC-08) | G1 · host | none stated | BD-365 |
| KA-418 | Skid-steer under exact lateral no-slip at stations a ahead of and behind the centre | v_y + a·ω = 0 and v_y − a·ω = 0 force ω = 0 | exact | G1 · host | A tracked base treated as a rolling constraint that still turns | BD-245, BD-365 |
| KA-419 | Rut isolation | A rut made in one episode is absent from every other; leak zero over 10³ resets | exact (GATE-TRAC-08) | G4 · lockstep | none stated | BD-366 |
| KA-420 | Capacity-bias fixture: a tracked design against a point-foot design under an artificially low contact cap | Returns `capacity-biased` and no winner; citing it as a verdict refuses (`FROB-REF-822`) | exact (verdict) | G1 · lockstep | Contacts removed to fit the capacity | BD-168 |
| KA-421 | CL-H on `batch-cpu` | No separation velocity on a sliding block (the associated model gives μ·‖u_T‖); zero creep below threshold; residual at most 10⁻⁴ within eight passes on at least 95% of battery contact states and 10⁻⁶ under `strict`; agreement with the referee on Solid rows | as GATE-TRAC-09 | G1 · strict, lockstep | The associated model with the μ·‖u_T‖ term dropped | BD-386 |
| KA-422 | Power-of-two diagonal scaling Ĥ = S·H·S | The computed Cholesky factor of S·H·S equals S times that of H bit for bit, absent overflow and underflow; solutions bit-identical with and without S on 10⁵ systems; `f32` against `f64` Newton-step error at most 0.5% at default cards and 5% at near-rigid cards; cards over the bound refuse (`FROB-REF-244`) | exact (bits); 0.5% and 5% (GATE-TRAC-10) | G3 · all tiers | none stated (the metamorphic relation's control: a scaling by 10) | BD-156 |
| KA-423 | Half-precision refuting calculation: rank-one contact operator with proximal shift ρ = 10⁻²·tr(W)/48 for 16 contacts | κ = 1 + 48/0.01 = 4,801 and κ·u₁₆ = 2.34 with u₁₆ ≈ 4.9 × 10⁻⁴ (refinement does not contract); k refinement steps cost 9k/n of the factor: 2.3 at n = 27 with seven steps, 1.5 at n = 35 with six, 3.0 at n = 18 with six; a half-precision request refuses (`FROB-REF-245`) | exact arithmetic; exact (verdict) | G1 · host | none stated | precision bead of the other Traction unit |
| KA-424 | Four-dimensional cone √(λ_t1² + λ_t2² + (λ_s/r_eff)²) ≤ μ·λₙ | A linear image of the second-order cone in ℝ⁴; projection and Moreau identities hold; r_eff = 0 masks the spin row and gives the three-dimensional cone | exact (polynomial identity); G0 property tests | G0 · host | none stated | BD-154 |
| KA-425 | Primitive pair distances and normals against `fs-query` on 10⁶ random pairs | Agreement | 10⁻⁹ m in `f64` (GATE-TRAC-02) | G2 · host | none stated | BD-079, BD-158 |

**Cross-references outside this block.** KA-282 (a tracked design turning in place at zero scrub work is caught by
the belt-law known answer); KA-300 and KA-301 (a motor reversal smaller than the backlash gap gives zero output
motion; a slow sinusoid through a gap β gives a loop of width β; the dead-zone model is the negative control);
KA-353 (the planted pyramid-cone winner is typed `model-conditional`); KA-437 to KA-439 (the scene-side film rows).
The hand fixtures of the response batch (bound at equality, mode guard, flop count) and of the swept-arc bound have
no KA number in the register; they are carried by BD-367 and by the swept-arc bead and should receive numbers when
the registry is next extended.

## Acceptance criteria

- [ ] All 39 rows are present with every field non-empty, and each `carrier` is an existing bead key.
- [ ] Every row whose negative control is stated has a registered mutant in the negative-control registry.
- [ ] No row remains `to-set` when the gate named in its tolerance is recorded as passed.
- [ ] The registry loads, and an unknown KA number cited by a test is a load-time error.
- [ ] The hand-computable truths are recomputed by a host test from their formulas and match the registry values.

## Test plan

### Unit tests
- `ka_traction_registry_complete`: 39 rows, numbers KA-387 to KA-425 contiguous, fields present.
- `ka_traction_truths_recompute`: recomputes 0.153 s, 0.288 s, 17.5 ms, 0.083 rad/s, 0.038 s, 2.3 s, 0.76 s, 0.525, 0.690, 0.619, 19.6 mm, 389 N, 293 N, 4,801 and 2.34 from the formulas and compares to three significant figures.
- `ka_traction_negative_controls_registered`: each stated control resolves to a mutant identifier.
- `ka_traction_carriers_exist`: each carrier key resolves in the exported bead graph (`synthetic` graph fixture for the failing case).

### End-to-end script and logging
`scripts/e2e/traction_registry.sh`: runs `frob eval run --battery fixtures --list --json` (or the registry listing
verb) and asserts the Traction rows appear with their tiers; logs per STD-E2E one `assert` event per row with the
KA number, its carrier and its tolerance state, and a `summary` line with the counts of rows, `to-set` rows and
registered negative controls.

### Known answers and negative controls
The table is the content. Negative control for the registry itself: delete one row and the completeness test fails;
edit one truth (0.288 to 0.204) and the recompute test fails.

## Considerations

- Six rows leave their tolerance open in the register (KA-388, KA-390, KA-395, KA-407, KA-413, KA-414); the carrier beads set them and record them as TARGETs.
- KA-423's product is 2.34 with u₁₆ = 2⁻¹¹ and 2.35 with the rounded 4.9 × 10⁻⁴; the registry stores the exact form.
- Rows carried by the other Traction unit are listed with seed keys where one exists; KA-423 has no seed key, and the lead resolves its carrier to the precision bead of that unit.

## References

Appendix M.17 (KA-387 to KA-425), §18.16, §18.17, §38.2, §38.6.
