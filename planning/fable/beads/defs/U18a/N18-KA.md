---
key: "N18-KA"
title: "registry: Traction known answers KA-387 to KA-425 with truth, tolerance, tier and negative control"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "gauntlet", "frob-conformance", "registry", "test", "solid"]
parent: "WP-14"
deps: ["BD-025", "N38a-12", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-109", "N18a-27", "N18b-01", "N18b-32", "N38-KA", "N38a-12", "N38b-27", "N38b-31", "NEG-12"]
gates: []
plan_refs: ["Appendix M.17", "§18.16", "§18.17", "§18.19", "§18.20", "§38.6"]
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

In: the 39 rows below as records of the suite's one known-answer file, in the row schema of N38a-12; their
derivation files; the four late hand fixtures of the response-batch and swept-arc subsections as rows numbered
from an allocation block (below); a completeness test. Out: the fixtures and tests themselves (the carrier
beads); the row schema, loader and eligibility rules (N38a-12); the controls' `NC-` records, which the seeding
tool of N38b-05 derives from the stated control cells; the `formula` field that links KA-403 to the actuator
chapter's card (N38b-48); rows of other chapters that touch contact (listed at the end for cross-reference).

## Technical approach

**Records.** One `[[ka]]` record per row in the suite's one physical file,
`crates/frob-conformance/registry/known_answers.toml` (DESIGN correction: an earlier draft of this bead named a
chapter file `registry/known_answers/traction.toml`, which N38a-12's one-file rule refuses with `ka-second-file`).
Fields are those of N38a-12: `id`, `section` (the "where stated" sections, with `owner` = the gate named in the
tolerance), `fixture` (a slug per physical case; KA-403 is `backlash-closure/contact-card`, the identity N38b-48
keeps apart from the actuator chapter's card), `same_as`, `case`, `truth`, `tolerance` (`exact`, `abs`, `rel`,
`printed-digits`, `gate` or `to-set`), `tier`, `sites`, `control` (`text` from the table's column; `nc` filled
by the seeding tool of N38b-05, never by hand), `derivation` (`docs/derivations/<fixture>.md`), `label`
(DESIGN for hand derivations; FACT with its source for KA-390), `due`, `printed`. The table's "Carrier" column
goes into the derivation file's front matter: for each KA it serves, the test that checks it (crate and test
name) and the carrier bead. A row whose tolerance the plan leaves open is entered as `tolerance = { kind =
"to-set" }` with the gate that sets it; it stays `obligation: tolerance` until the carrier bead records a number
as a TARGET in the derivation file. Constants: g = 9.81 m/s².

| KA | Case | Truth | Tolerance | Tier · sites | Negative control | Carrier |
|---|---|---|---|---|---|---|
| KA-387 | Block on an incline of angle α, friction μ | Stays if tan α < μ, slides if tan α > μ; creep below threshold at most the stated stiction speed (zero under CL-H) | Measured threshold within 1% of μ | G1 · strict, lockstep, referee | (set: BD-085) friction disabled, which slides at every angle; (set: N18b-32) a coefficient taken from the wrong card | BD-085, BD-161, BD-168 |
| KA-388 | Sphere rolling down an incline without slip | Acceleration (5/7)·g·sin α | (set: BD-085) 0.5% on `strict` and the referee, a TARGET (printed to set, GATE-TRAC-01); 1.2168 m/s² for r = 0.1 m, α = 10°, μ = 0.5 | G1 · strict, lockstep, referee | (set: N18b-32) a sliding sphere with no rotational inertia, g·sin α | BD-085, BD-161 |
| KA-389 | Disk rolling on a plane | v = ω·r; path equals Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)), δ = ω·Δt, sinc(0) = 1 | v = ω·r within 0.1% | G1 · strict, lockstep, referee | (set: N18b-32, N18b-01) eight rim samples (m_rim = 8), a 2.6% deficit | BD-085, BD-161, N18b-01 |
| KA-390 | Near-straight differential drive (admission test of the rolling oracle): heading 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, 400 steps of 1/400 s | Translation about (0.8342, 0.5514) m from the closed form | to set | G2 · host | `fs-mbd::planar_drive` at the pinned commit, reported to return zero translation (FACT to recheck); not an oracle until it passes | BD-033 (INT-FSIM-05, the closed form (0.83422336, 0.55142668) m); N18b-01; the same fixture in BD-109's `diff_drive_near_straight` |
| KA-391 | Box stack, ten high | At rest for 10 s, drift below 1 mm | 1 mm | G1 · strict, lockstep | (set: N18b-32) a position-level correction that creates energy | BD-161, BD-168 |
| KA-392 | Dropped body, inelastic law | Total mechanical energy never increases | exact (sign) | G1 · strict, lockstep, referee | (set: N18b-32) a restitution above one | BD-080, BD-085 |
| KA-393 | Capsule against a spherical distance field | Gap error at most the brick's interpolation bound | the stated interpolation bound | G1 · strict, lockstep | (set: N18b-32) a brick with its bound halved | BD-166, BD-161 |
| KA-394 | Mecanum base commanded sideways | Moves sideways with yaw below 1° | 1° | G1 · strict, lockstep | (set: N18b-32) all roller axes parallel | N18b-02, BD-167, BD-161 |
| KA-395 | Ring on a cone | Slides if and only if tan α > μ | (set: BD-085) verdict exact at tan α ∈ {0.38, 0.42} with μ = 0.4 (printed to set, GATE-TRAC-01) | G1 · strict, lockstep, referee | (set: N18b-32) the pyramid cone | BD-085 |
| KA-396 | Painlevé rod | The referee reports the indeterminate configuration (`FROB-REF-224`) and returns no number | exact (verdict) | G1 · referee | A solver that returns one of several distinct motions | BD-085 |
| KA-397 | Disk of radius a spinning in place, uniform pressure | Stops at t = 3·a·ω₀/(4·μ·g): 0.153 s for a = 0.05 m, ω₀ = 20 rad/s, μ = 0.5 | 1% (GATE-TRAC-04) | G1 · strict, lockstep | (set: N18a-27, N18b-32) r_eff = a in place of 2a/3, which stops the disk at 0.102 s | BD-154 |
| KA-398 | Effective patch radius of the spin row | Uniform pressure on a disk of radius a: capacity (2/3)·μ·N·a, r_eff = 2a/3; Hertzian: r_eff = 3π·a/16 ≈ 0.59·a | exact (derivation on file, PO-TRAC-04) | G0 · host | none stated | N18a-27 |
| KA-399 | Sliding at v while spinning at ω on the four-dimensional cone | Tangential force μ·N·v/√(v² + (r_eff·ω)²); μ·N/√2 at v = r_eff·ω | 0.5% of μ·N/√2 (GATE-TRAC-04) | G1 · strict, lockstep | Independent slide and spin rows (41% more in magnitude) | BD-154 |
| KA-400 | Thin ring sliding and spinning (r_eff = a, I = m·a²) | v/(a·ω) constant; both stop at t = √(v₀² + a²·ω₀²)/(μ·g): 0.288 s for v₀ = a·ω₀ = 1 m/s, μ = 0.5 | 1% (GATE-TRAC-04) | G1 · strict, lockstep | Independent slide and spin rows (stop at 0.204 s) | BD-154 |
| KA-401 | Traction along a diagonal of the tangent plane | Maximum friction μ·N in every direction | 0.5% of μ·N (GATE-TRAC-04) | G1 · strict, lockstep | The pyramid cone, which gives √2·μ·N | BD-154 |
| KA-402 | Joint stiction under a torque ramp τ = k·t | Breakaway at t = τ_s/k | 1% (GATE-TRAC-04) | G1 · strict, lockstep | (set: N18b-32) viscous-only joint friction | N18a-32, BD-084, BD-155 |
| KA-403 | Backlash crossing and closure: rotor J = 10⁻⁴ kg·m² at 5 rad/s, ratio N = 10, gap β = 0.5°, output inertia 0.05 kg·m², inelastic | Crossing time β·N/ω₀ = 17.5 ms; output speed J·N·ω₀/(J·N² + I_out) = 0.083 rad/s | 1% (GATE-TRAC-04) | G1 · strict, lockstep | Backlash as a dead zone on the torque command | N18a-32, BD-084, BD-155 |
| KA-404 | Stefan squeeze to 10 µm from a thick film: dh/dt = −2·N·h³/(3π·η·a⁴), 1/h² − 1/h₀² = 4·N·t/(3π·η·a⁴) | Water 1 mPa·s, a = 2 cm, 100 N: 0.038 s; oil 60 mPa·s, same: 2.3 s; water, a = 6 cm, 400 N: 0.76 s | 2% (GATE-TRAC-05) | G1 · strict, lockstep | A constant coefficient per floor texel | BD-230 |
| KA-405 | Sled on a full film (h ≥ 3σ) | Couette shear F = η·v·A/h | 10⁻⁶ relative in `f64` (GATE-TRAC-05) | G1 · strict, lockstep | (set: N18b-32) the film term dropped | BD-230 |
| KA-406 | Film law with no film (h₀ = 0) | The dry card μ_d, bit for bit | exact (bits) | G1 · all tiers | (set: N18b-32) a blend evaluated at h₀ = 0 that rounds | BD-230 |
| KA-407 | Film-law limits on 10⁴ random states | μ_b for h ≤ σ; pure Couette for h ≥ 3σ; continuous between; a squeezed film recovers to μ_b, not μ_d | to set | G1 · strict, lockstep | (set: BD-230) a law that returns μ_d after a squeeze | BD-230 |
| KA-408 | Thin barrier: sphere r = 2 mm at 5 m/s, h = 2 ms (10 mm of travel); 2 mm plate 1 mm ahead; clearing takes 1 + 4 + 2 = 7 mm | The step encloses a collision though both endpoints are clear (3 mm beyond the far face); zero missed hits in 10⁶ randomized crossings under transactional profiles | exact (count; GATE-TRAC-06) | G1 · referee, strict | The endpoint-only query, which misses | BD-086, BD-161 |
| KA-409 | Torque-driven wheel over a sharp sill | Feature sequence flat, edge, top; work and impulse converge under step halving; no unexplained couple | 1% of refined values at h/4; no couple above 10⁻⁹ N·m in `f64` (GATE-TRAC-06) | G1 · referee, strict | Tangential memory kept on a changed feature | BD-086, BD-161 |
| KA-410 | Clearance with registration errors | 6 mm nominal gap, 4 mm outward error per side: outer enclosures overlap by 2 mm, answer `unknown`. 10 mm nominal, same errors: `clear` with 2 mm to spare | exact (verdict) | G1 · host | The nominal gap taken as clearance | N18a-03 |
| KA-411 | Enclosure answers on 10⁴ generated pairs with known truth | No false `clear`, no false `penetrates`; every certificate re-checked offline | exact (counts; GATE-TRAC-07) | G1 · host, auditor | (set: N18a-03) inner and outer enclosures swapped; an `f32` sign test at the boundary | N18a-03, BD-233 |
| KA-412 | Two swept volumes that overlap in space while the bodies occupy the shared region at different times | Not a contact: the test is per time slab | exact (verdict) | G1 · host | A test on whole-sweep volumes | N18a-03 |
| KA-413 | Sole straddling a low-grip strip (μ 0.25 inside, 0.8 outside, uniform pressure), boundary at offset z, sole width w | High-grip share clip(½ + z/w, 0, 1): capacity 0.525·N at z = 0 for any width; at z = 12 mm, 0.690·N for w = 40 mm and 0.619·N for w = 70 mm | (set: N18a-28) 0.5%, a TARGET (printed to set; the Arena rig's own bound is 1%) | G1 · strict, lockstep | Contact area taken as the predictor of traction | N18a-28 |
| KA-414 | Load-weighted slip metric | Duplicating a quadrature point and halving its weights changes nothing | (set: N18b-32) 10⁻¹² relative in `f64`, DESIGN (printed to set) | G3 · all tiers | (set: N18b-32) the unweighted sum of point speeds | N18a-28 |
| KA-415 | Plate sinkage on soft soil, p = (k_c/b + k_φ)·zⁿ | z = (p/(k_c/b + k_φ))^(1/n); b = 0.1 m, k_c = 1 kPa/mⁿ⁻¹, k_φ = 1,500 kPa/mⁿ, n = 1.1, p = 20 kPa: z = 19.6 mm (illustrative) | law 10⁻⁶ in `f64`; simulated sinkage 2% (GATE-TRAC-08) | G1 · strict, lockstep | (set: N18b-32) the exponent mistranscribed | N18b-12, N18b-13, BD-366 |
| KA-416 | Track thrust at slip i, τ = (c + σ·tan φ)·(1 − exp(−j/K)), j = i·x | F = (A·c + W·tan φ)·(1 − (K/(i·L))·(1 − exp(−i·L/K))); two tracks 0.5 m × 0.1 m, 500 N, c = 1 kPa, φ = 30°, K = 25 mm: limit 389 N, 293 N at 20% slip (illustrative) | law 10⁻⁶ in `f64`; drawbar pull 2% (GATE-TRAC-08) | G1 · strict, lockstep | (set: N18b-32) the shear displacement ignored (always the limit) | N18b-12, N18b-13, BD-366 |
| KA-417 | Lateral brush tyre force, ‖F_y‖ = μ·N·(3ζ − 3ζ² + ζ³), ζ = C_α·‖tan α‖/(3·μ·N) ≤ 1, μ·N beyond | Slope C_α at zero slip; saturation μ·N at tan α = 3·μ·N/C_α | 10⁻⁶ in `f64` (GATE-TRAC-08) | G1 · host | (set: N18b-32) a linear law with no saturation | N18b-04, BD-365 |
| KA-418 | Skid-steer under exact lateral no-slip at stations a ahead of and behind the centre | v_y + a·ω = 0 and v_y − a·ω = 0 force ω = 0 | exact | G1 · host | A tracked base treated as a rolling constraint that still turns | BD-245, BD-365 |
| KA-419 | Rut isolation | A rut made in one episode is absent from every other; leak zero over 10³ resets | exact (GATE-TRAC-08) | G4 · lockstep | (set: N18b-32) a shared rut buffer | N18b-17, BD-366 |
| KA-420 | Capacity-bias fixture: a tracked design against a point-foot design under an artificially low contact cap | Returns `capacity-biased` and no winner; citing it as a verdict refuses (`FROB-REF-822`) | exact (verdict) | G1 · lockstep | Contacts removed to fit the capacity | N18b-22, BD-168 |
| KA-421 | CL-H on `batch-cpu` | No separation velocity on a sliding block (the associated model gives μ·‖u_T‖); zero creep below threshold; residual at most 10⁻⁴ within eight passes on at least 95% of battery contact states and 10⁻⁶ under `strict`; agreement with the referee on Solid rows | as GATE-TRAC-09 | G1 · strict, lockstep | The associated model with the μ·‖u_T‖ term dropped | BD-386 |
| KA-422 | Power-of-two diagonal scaling Ĥ = S·H·S | The computed Cholesky factor of S·H·S equals S times that of H bit for bit, absent overflow and underflow; solutions bit-identical with and without S on 10⁵ systems; `f32` against `f64` Newton-step error at most 0.5% at default cards and 5% at near-rigid cards; cards over the bound refuse (`FROB-REF-244`) | exact (bits); 0.5% and 5% (GATE-TRAC-10) | G3 · all tiers | (set: N18b-25) decimal scaling, 10^(−⌈log₁₀⌉) in place of a power of two, which breaks bit identity | N18b-25, BD-156 |
| KA-423 | Half-precision refuting calculation: rank-one contact operator with proximal shift ρ = 10⁻²·tr(W)/48 for 16 contacts | κ = 1 + 48/0.01 = 4,801 and κ·u₁₆ = 2.34 with u₁₆ ≈ 4.9 × 10⁻⁴ (refinement does not contract); k refinement steps cost 9k/n of the factor: 2.3 at n = 27 with seven steps, 1.5 at n = 35 with six, 3.0 at n = 18 with six; a half-precision request refuses (`FROB-REF-245`) | exact arithmetic; exact (verdict) | G1 · host | (set: N18b-27) a permissive parser that maps `f16` to `f32` silently (fails the refusal); the unit round-off taken as 2⁻¹⁰, which gives 4.688 (fails the value) | N18b-27 |
| KA-424 | Four-dimensional cone √(λ_t1² + λ_t2² + (λ_s/r_eff)²) ≤ μ·λₙ | A linear image of the second-order cone in ℝ⁴; projection and Moreau identities hold; r_eff = 0 masks the spin row and gives the three-dimensional cone | exact (polynomial identity); G0 property tests | G0 · host | none stated | BD-154 |
| KA-425 | Primitive pair distances and normals against `fs-query` on 10⁶ random pairs | Agreement | 10⁻⁹ m in `f64` (GATE-TRAC-02) | G2 · host | (set: BD-079) its four planted defects, and the M0 probe's `full-extent-as-half` and `unrotated-support` mutants re-run against the production adapter | BD-079, BD-158 |

**Cells set by their owners.** The printed suite leaves 24 controls `none stated` and 6 tolerances `to set` in
this range. The carriers and the battery bead N18b-32 (which turns 29 of these rows into fixtures and depends on
this bead) have since registered 22 of the controls and 4 of the tolerances; this registry carries each marked
"(set: <bead>)", with the owner's reason in the derivation file (N38b-27: an owner's value stands), and the seeding tool turns
those control texts into the rows' first controls (N38b-31's proposals, where they differ, become second records).
Still open: the controls of KA-398 (N18a-27 names r_eff = a against KA-397's stop time, not against the
derivation row) and KA-424 (BD-154 names the pyramid and the independent rows as mutants of its group of rows,
not of the cone identity), and the tolerances of KA-390 (BD-033, N18b-01) and KA-407 (BD-230). Each stays
`obligation` with its owner named; recomputed at review: 0.1529 s and the mutant's 0.1019 s (KA-397), 0.2883 s
and 0.2039 s (KA-400), 17.45 ms and 0.0833 rad/s (KA-403), 0.0377, 2.26 and 0.763 s (KA-404), 0.525, 0.690 and
0.619 (KA-413), 19.62 mm (KA-415), 388.7 N and 293.3 N (KA-416), 4,801 and 2.344 (KA-423).

**Cross-references outside this block.** KA-282 (a tracked design turning in place at zero scrub work is caught by
the belt-law known answer); KA-300 and KA-301 (a motor reversal smaller than the backlash gap gives zero output
motion; a slow sinusoid through a gap β gives a loop of width β; the dead-zone model is the negative control);
KA-353 (the planted pyramid-cone winner is typed `model-conditional`); KA-437 to KA-439 (the scene-side film rows);
KA-847 (the single-precision residual bound H = 3, b = 1 of the response batch, adopted by the verification
chapter, N38-KA).

**Late hand fixtures, numbered from an allocation block.** The response-batch and swept-arc subsections state
four known answers by hand that the printed suite does not number. Their numbers come from the allocation ledger
`crates/frob-conformance/registry/allocations.toml` (N38a-40), never from "the next free number": this bead
files one `[[block]]` request with `registry = "ka"`, `count = 4`, `owner = "N18-KA"`, `section = "§18.19,
§18.20"`; the allocator fills `first` and `last`, and the tags below are mapped to the block's numbers in table
order in one commit (a tag left in a registry, test name or derivation file at M1 fails
`provisional-tag-past-due`).

| Tag | Case | Truth | Tolerance | Tier · sites | Negative control | Carrier |
|---|---|---|---|---|---|---|
| `trac-ka-bound` | Response-batch bound at equality: H = [[2, 1], [1, 2]] (eigenvalues 1 and 3, λ_lo = 1), b = (3, 3), x* = (1, 1), x̂ = (1.01, 0.99) | Residual (−0.01, 0.01), norm 0.014142; true error norm 0.014142: the bound ‖r̂‖₂/λ_lo holds with equality (the error lies along the eigenvector of the smallest eigenvalue) | exact (rationals) | G1 · host | A bound formed with λ = 3 claims 0.004714 and is violated (GATE-TRAC-11) | N18b-40, N18b-42 |
| `trac-ka-mode` | Mode guard: a block on a floor with μ·N = 5 N, anchor in stick; counterfactual tangential loads 4 N and 6 N | 4 N accepted (friction answers 4 N, slack 1 N, the block stays); 6 N rerouted to the full solve, which slides under a net 1 N | exact (verdicts) | G1 · strict | The frozen sticking set accepts 6 N and reports no motion | N18b-40, N18b-42 |
| `trac-ka-flops` | Flop count of a response batch: n = 96 unknowns, 64 columns | Full solve per column n³/3 + 2n² = 294,912 + 18,432 = 313,344 flops, 20,054,016 in all; one factor and 64 solves 294,912 + 64·18,432 = 1,474,560; ratio 13.6; the ratio first reaches 10 at 23 columns (10.03; 9.84 at 22); against a baseline that also factors once it is 1 | exact (integers) | G0 · host | A ceiling quoted against a factor-per-column baseline when the baseline factors once | BD-367 |
| `trac-ka-arc` | Swept arc: a 1 m arm turns from −60° to +60° about the origin; tip sphere radius 5 cm; obstacle at (1, 0) m | End positions (0.5, ∓0.866) m; padded chord reaches x = 0.55 m; sagitta R·(1 − cos(Δθ/2)) = 0.5 m, so the bound reaches 1.05 m and contains the obstacle; path-length bound L = 2π/3 m gives dilation ½·√(L² − ‖B − A‖²) = 0.589 m ≥ 0.5 m | exact (verdict); lengths 10⁻⁹ m in `f64` (GATE-TRAC-12) | G1 · host, referee | The chord-only bound, which misses the obstacle | N18b-43, N18b-45 |

## Acceptance criteria

- [ ] All 39 rows are present in `crates/frob-conformance/registry/known_answers.toml` with every field of the
      N38a-12 schema filled and validate under its loader; no file `registry/known_answers/traction.toml` exists;
      each row has a derivation file that lists it with its test and carrier, and each carrier is an existing bead
      key.
- [ ] The four late rows carry numbers from a block issued to N18-KA in `allocations.toml`, in table order;
      `registry::alloc::validate` passes and no `trac-ka-` tag remains in a registry, test name or derivation file.
- [ ] After the seeding tool of N38b-05 is re-run, every row whose negative control is stated or set by its
      owner has its `NC-` identifiers in `control.nc`; KA-398 and KA-424 are reported `obligation: control`, and
      KA-390 and KA-407 `obligation: tolerance`, never `eligible`.
- [ ] The rows are selectable through their tier targets (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance
      --test <tier> -- KA-387`, with `g0_laws`, `g1_known_answers`, `g2_parity`, `g3_metamorphic` or `g4_chaos`)
      and are reported `unbound` or `obligation` until their carriers bind them, never `pass`.
- [ ] No row remains `to-set` when the gate named in its tolerance is recorded as passed.
- [ ] The registry loads, and an unknown KA number cited by a test is a load-time error.
- [ ] The hand-computable truths are recomputed by a host test from their formulas and match the registry values.

## Test plan

### Unit tests
`crates/frob-conformance/tests/traction_known_answers.rs` (host arithmetic, exact where rational):
- `ka_traction_registry_complete`: 39 rows, numbers KA-387 to KA-425 contiguous, fields present; plus the four
  rows of the issued block.
- `ka_traction_truths_recompute`: recomputes 0.153 s, 0.288 s, 17.5 ms, 0.083 rad/s, 0.038 s, 2.3 s, 0.76 s, 0.525, 0.690, 0.619, 19.6 mm, 389 N, 293 N, 4,801 and 2.34 from the formulas and compares to three significant figures.
- `ka_traction_late_rows_recompute`: 0.014142 = 0.014142 and 0.004714 (bound at equality, λ = 3 violated);
  313,344, 20,054,016, 1,474,560, ratio 13.60, first column count reaching 10 is 23; sagitta 0.5 m, bound 1.05 m,
  dilation 0.5888 m, padded chord 0.55 m.
- `ka_traction_negative_controls_registered`: each stated or owner-set control resolves to a mutant identifier;
  the two open controls and two open tolerances are reported as obligations naming their owners.
- `ka_traction_carriers_exist`: each carrier key resolves in the exported bead graph (`synthetic` graph fixture for the failing case).

### End-to-end script and logging
`scripts/e2e/traction_registry.sh` (header: `bead=N18-KA milestone=M1 labels=traction,registry
gates=GATE-GAUNT-02 plants=deleted-row,edited-truth,tag-past-due`): runs the registry check and this chapter's
recompute tests (`cargo test -p frob-conformance --test registry_check`, `--test traction_known_answers`) and
N38a-40's allocation test, then selects the 39 rows and the four late rows through their tier targets
(`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <row>`) and logs each row's per-site outcome
as the runner reports it; a row whose carrier has not landed is `unbound`, never passed. It calls no `frob` verb:
running the battery through `eval.run` belongs to the battery bead N18b-32 and the verb's owner, on which this bead
does not depend. It plants on scratch copies under `target/e2e-scratch/traction_registry/` a deleted row, an edited
truth (0.288 to 0.204) and a `trac-ka-` tag left in milestone-exit mode, and requires each to be caught. It logs per
STD-E2E (NDJSON to `target/e2e-logs/traction_registry/<utc>.ndjson` with a human transcript, each line with `ts`,
`script`, `bead`, `step`, `event`, the exact command line, exit code and wall time) one `assert` event per row with
the KA number, its carrier, its tier and sites, its tolerance state, its eligibility and its negative-control
identifier; the SHA-256 of `known_answers.toml` and `allocations.toml`; and a `summary` line with the counts of rows
(39 expected, plus 4 late), open tolerances (2), open controls (2) and registered negative controls. On failure the
line carries the first differing field (expected against listed value, by KA number) and the path of the scratch
copy of `known_answers.toml` that reproduces it.

### Known answers and negative controls
The table is the content. Negative control for the registry itself: delete one row and the completeness test fails;
edit one truth (0.288 to 0.204) and the recompute test fails.

## Considerations

- Six rows leave their tolerance open in the register (KA-388, KA-390, KA-395, KA-407, KA-413, KA-414); the carrier beads set them and record them as TARGETs; four are set (above) and KA-390 and KA-407 remain.
- KA-423's product is 2.34 with u₁₆ = 2⁻¹¹ and 2.35 with the rounded 4.9 × 10⁻⁴; the registry stores the exact form.
- Rows carried by the other Traction unit are listed with the mechanism bead of that unit first and the seed capstone after it where one exists (KA-415, KA-416: soil laws N18b-12 and soil rows N18b-13, capstone BD-366; KA-417: tyre laws N18b-04, capstone BD-365; KA-419: rut fields N18b-17; KA-420: capacity-bias rule N18b-22; KA-422: scaling kernel N18b-25; KA-423: the half-precision refusal N18b-27, which has no seed).

## References

Appendix M.17 (KA-387 to KA-425), §18.16, §18.17, §18.19 and §18.20 (the four late hand fixtures), §38.2,
§38.6; N38a-12 (schema), N38a-40 (allocation), N38b-05 (controls), N38b-48 (KA-403's fixture identity).
