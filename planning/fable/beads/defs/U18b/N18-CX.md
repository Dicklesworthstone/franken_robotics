---
key: "N18-CX"
title: "frob-conformance: Traction counterexample registry CX-167 to CX-183, seeding the negative-control registry"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "gauntlet", "frob-conformance", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the negative-control registry frob-conformance/registry/controls.toml and its loader (conformance unit), into which these rows are entered"]
related: ["N18b-32", "N18-KA", "N18-REF", "BD-146"]
gates: []
plan_refs: ["Appendix N.15", "§18.3", "§18.4", "§18.5", "§18.6", "§18.7", "§18.8", "§18.9", "§18.11", "§18.12", "§18.16", "§18.19", "§18.20"]
---
## Background

Where a plausible shortcut is wrong, the project records the smallest hand-checkable case that breaks it. Such a
case is first-class: it enters the counterexample catalogue and seeds the negative-control registry, so that
every gate it protects is tested against the shortcut planted as a mutation. A gate that nothing can fail
validates nothing, and a rule that ships without such a case has not been thought through.

This bead is the registry of the 17 counterexamples that belong to the contact subsystem (collision, contact,
wheels, tracks, terrain), plus the counterexamples stated in two late subsections that have no catalogue number
yet. Each row gives the tempting shortcut, the case with its numbers, and the rule it protects. Mechanism beads
cite these numbers in their test plans and state the values they need inline; this bead is where the complete
set lives as data and where the mapping from counterexample to planted mutation is kept. A CX number is never
reused and never renumbered; new rows append.

## Scope

In: the rows below as entries of the counterexample catalogue and of `controls.toml`; one mutation identifier
per row; a host test that recomputes every number. Out: the gates and mechanisms that use the controls.

## Technical approach

**Data.** Each row is an entry `{ cx, shortcut, case, rule, defined_in, mutation, gates }` in
`frob-conformance/registry/controls.toml`. `mutation` names a build or plan flag that plants the shortcut in the
real component (never a mock); a gate listed in `gates` must fail with the mutation enabled.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects |
|---|---|---|---|
| CX-167 | A positive nominal gap is clearance | A nominal gap of 6 mm with 4 mm of outward error on each side leaves the outer enclosures overlapping by 2 mm: the answer is `unknown` until an error term is reduced. With 10 mm nominal and the same errors the answer is `clear` with 2 mm to spare | INV-TRAC-03 (`clear` from disjoint outer enclosures only, `penetrates` from intersecting inner enclosures only); GATE-TRAC-07; strict inequalities by outward rounding or exact-integer kernels |
| CX-168 | Two swept volumes that overlap in space collide | The bodies can occupy the shared region at different times and never touch; the test is per time slab, never on whole-sweep volumes (no numeric case is given) | The per-slab enclosure test; GATE-TRAC-07 |
| CX-169 | If the gap is positive at both ends of a step, no contact occurred in it | A sample sphere of radius 2 mm moves at 5 m/s with h = 2 ms, so it travels 10 mm. A 2 mm plate lies 1 mm ahead. Clearing the plate takes 1 + 4 + 2 = 7 mm, so at the end of the step the sphere is 3 mm beyond the far face: both endpoints are clear and the step encloses a collision | Swept bounds and event location under transactional profiles; the speculative margin of at least v·h + ε_g on lockstep tiers; GATE-TRAC-06 (the endpoint-only mutation must miss); GATE-TRAC-03 |
| CX-170 | A scene distance field at working resolution represents all the geometry | A trilinear field on a 2 cm grid cannot hold a 2 mm plate: the plate vanishes or is thickened to the grid | Features thinner than two fine voxels are routed to thin primitives (`K-np-thin`) or the mesh path; FROB-REF-320 for a thin feature with no route |
| CX-171 | Sliding friction and torsional friction can be independent rows | With sliding speed v and spin ω on the four-dimensional cone the tangential force is μ·N·v/√(v² + (r_eff·ω)²), which is μ·N/√2 at v = r_eff·ω. Independent rows grant μ·N of sliding friction and μ·N·r_eff of torsional friction at once, 41% more in magnitude than the surface can supply; a foot that pivots while it slides would hold where it should slip | The four-dimensional cone (PO-TRAC-01); GATE-TRAC-04 (the independent-row mutation must fail) |
| CX-172 | Backlash can be modelled as a dead zone on the torque command | A dead zone produces no impact when the gap closes and no hunting in a position loop. The rows give both: a rotor of inertia 10⁻⁴ kg·m² at 5 rad/s behind a ratio of 10 crosses a 0.5° gap in β·N/ω₀ = 17.5 ms and, closing inelastically on an output of 0.05 kg·m², leaves it at J·N·ω₀/(J·N² + I_out) = 0.083 rad/s | Backlash as two unilateral rows on the relative angle with the rotor as its own coordinate; FROB-REF-241; GATE-TRAC-04; GATE-SINEW-04 |
| CX-173 | A tracked or skid-steer base can be modelled with rolling (no lateral slip) constraints | Exact lateral no-slip at contact stations a ahead of and behind the centre requires v_y + a·ω = 0 and v_y − a·ω = 0, hence ω = 0. The base turns only by slipping, so its turning is a property of the friction law | The belt law with an anisotropic friction ellipse and yaw scrub; the rejection fixture "turning in place at zero scrub work"; the nonholonomic model is a planning model and an oracle for rolling only |
| CX-174 | Diagonal scaling makes the Cholesky factor more accurate | Without pivoting, the computed factor of S·H·S for power-of-two S is S times the computed factor of H bit for bit, absent overflow and underflow: its rounding errors are already those of the scaled problem. What scaling repairs is everything around the factor: tolerances, stopping tests, line-search thresholds, iterative passes | PO-TRAC-03; GATE-TRAC-10; ADR-TRAC-07 |
| CX-175 | One tolerance on the unscaled Newton gradient serves every robot | Degrees of freedom mix metres and radians, and their diagonal entries differ by orders of magnitude, so one tolerance on the unscaled gradient is a dimension error; in scaled variables the gradient norm is dimensionless. Numeric instance (from the numerics rules): H = diag(1, 10⁻⁸) with residual (0, 10⁻⁷) passes an absolute tolerance of 10⁻⁶ while the error in the second unknown is 10 | Diagonal scaling in stage 8 (`K-scale-diag`); INV-TRAC-05 (impulses and forces typed apart; no tolerance applies to both) |
| CX-176 | A half-precision factor with single-precision refinement cheapens the contact solve | Refinement contracts only if κ·u₁₆ < 1, with u₁₆ ≈ 4.9·10⁻⁴. A rank-one contact operator with proximal shift ρ = 10⁻²·tr(W)/48 for 16 contacts has κ = 1 + 48/0.01 = 4,801 and κ·u₁₆ = 2.34. Cost: k refinement steps cost 9k/n times the factor they replace: 2.3 at n = 27 with seven steps, 1.5 at n = 35 with six, 3.0 at n = 18 with six. Emulated on three reference models (FACT, to be re-run): κ·u₁₆ ≥ 1 in 37 to 64% of sampled contact states and failure in 9 to 18% | ADR-TRAC-07; FROB-REF-245; single precision admitted only below the scaled κ·u bound (FROB-REF-244) |
| CX-177 | A wet floor is a map of constant friction coefficients | By the integrated Stefan law, a film thins to 10 µm in 0.038 s under a 2 cm patch carrying 100 N on water, in 2.3 s under the same patch on cooking oil, and in 0.76 s under a 6 cm patch carrying 400 N on water. Against a stance of 0.3 s a small foot on water reaches boundary friction almost at once, while oil and large flat feet ride a film for the whole stance | The film law with per-contact film thickness in contact memory; GATE-TRAC-05; KILL-FILM (the law must change at least 10% of outcomes against a constant patch at μ_b) |
| CX-178 | A foot that has squeezed the film out stands on a dry floor | A squeezed film recovers to the boundary-lubricated coefficient μ_b, not to the dry coefficient μ_d; only h₀ = 0 returns μ_d exactly | INV-TRAC-07 |
| CX-179 | A wider sole gives more traction on a patchy floor | Sole straddling a low-grip strip (μ 0.25 inside, 0.8 outside, uniform pressure) with the boundary at offset z: the high-grip share is clip(½ + z/w, 0, 1). At z = 0 the capacity is 0.525·N for any width; at z = 12 mm it is 0.690·N for w = 40 mm and 0.619·N for w = 70 mm. Area alone does not predict traction | The sole-strip row of the known-answer battery; load-weighted quadrature of patches |
| CX-180 | When a world exceeds its contact capacity the extra contacts can be dropped | A comparison is honest only if both arms were simulated. A tracked design against a point-foot design under an artificially low cap must return `capacity-biased` and no winner; the label applies whenever the arms differ by more than one percentage point of worlds in `Degraded` or capacity-refused episodes | INV-TRAC-04; INV-KIN-05; FROB-REF-721, FROB-REF-722, FROB-REF-822; GATE-TRAC-06 |
| CX-181 | The convex (associated) friction model is Coulomb friction | Dropping the term μ·‖u_T‖ from the cone complementarity gives the associated model, in which a sliding contact acquires a separation velocity μ·‖u_T‖. The compliant default law has that artifact in compliant form | The hard-contact law as the written specification of the referee; the fidelity ledger measures the artifact; the contact-law arm of the model-form check; GATE-TRAC-09 (no separation velocity on a sliding block) |
| CX-182 | A library integrator for a differential drive can serve as the rolling oracle | Heading 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, 400 steps of 1/400 s: the sibling's planar-drive integrator at the pinned commit is reported to return zero translation where about (0.8342, 0.5514) m is correct (FACT to recheck). Identical to CX-029 of the design-language chapter: one fixture identity | The closed form Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)) with sinc(0) = 1 is the oracle; the admission test INT-FSIM-05 and upstream work item FD-FSIM-11 |
| CX-183 | Friction memory can stay on a contact when its feature changes | A wheel rolling over a sharp sill moves its witnesses from face to edge to face. Tangential memory kept on the changed feature produces a couple the contact forces do not explain; the fixture compares work, impulse and the feature sequence under step refinement (bounds: 1% at h/4 and 10⁻⁹ N·m in `f64`) | INV-TRAC-06 (memory is transported or reset at every witness change, each reset an event); GATE-TRAC-06 |

**Related rows owned by other chapters and cited by contact beads.** CX-148 (a four-sided friction pyramid
grants √2·μ along its diagonals, 41% more than the cone; GATE-TRAC-04, GATE-FID-05). CX-151 (the referee admits
at most 21 contacts: a sole with eight samples per foot fits when standing, 16, and overflows in a fall; six
feet with four samples need 24).

**Counterexamples stated in late subsections, without a catalogue number.** They are entered in `controls.toml`
under provisional identifiers and proposed for appended CX numbers (the catalogue owner assigns them):

| Provisional id | The tempting shortcut | The smallest case that breaks it | The rule it protects |
|---|---|---|---|
| `trac-arc` | A bound swept along the chord between end positions encloses the motion | A 1 m arm turns from −60° to +60° about the origin; tip sphere of radius 5 cm; obstacle at (1, 0) m. End positions (0.5, ∓0.866) m; the chord lies on x = 0.5 m and the padded chord reaches x = 0.55 m, yet the tip passes through (1, 0). The sagitta is 0.5 m, so the true bound reaches 1.05 m | Swept bounds enclose arcs: dilation R·(1 − cos(Δθ/2)) or ½·√(L² − ‖B − A‖²); GATE-TRAC-12 (the chord-only mutant fails) |
| `trac-resid` | A computed residual of zero means zero error | In single precision H = 3, b = 1 give x̂ = 0.33333334, a computed residual of exactly 0 and a true error of 9.9·10⁻⁹; with the rounding margin ρ = 2.4·10⁻⁷ the bound is 7.9·10⁻⁸ | The response-batch bound (‖r̂‖₂ + ρ)/λ_lo; GATE-TRAC-11; GATE-GAUNT-11 |
| `trac-lambda` | Any eigenvalue serves as the divisor of the residual bound | H = [[2, 1], [1, 2]] (eigenvalues 1 and 3), b = (3, 3), x̂ = (1.01, 0.99): residual norm 0.01414 and true error 0.01414. A bound formed with λ = 3 claims 0.00471 and is violated | λ_lo is a certified lower bound on the smallest eigenvalue; GATE-TRAC-11 (the λ = 3 mutation is caught) |
| `trac-mode` | A linear response on the frozen active set is valid for any counterfactual load | A block rests on a floor with μ·N = 5 N, anchor in stick. A tangential load of 4 N is answered by friction; a load of 6 N slides under a net 1 N, while the frozen sticking set reports no motion | The mode guard of the response batch reroutes the column to the full solve; GATE-TRAC-11 |

## Acceptance criteria

- [ ] 17 catalogue rows (CX-167 to CX-183) and the four provisional rows exist in `controls.toml`, each with a
      mutation identifier and the gates it must fail.
- [ ] A host test recomputes every numeric case in the tables and matches the printed values.
- [ ] For every row whose mechanism exists at the time, the conformance run with the mutation enabled fails the
      listed gate and with it disabled passes; a row whose mechanism does not exist yet is marked `pending` with
      the bead that will satisfy it.
- [ ] The registry loader rejects a control with no gate and a gate of this chapter with no control.
- [ ] No row uses a mock: each mutation is a flag on the real component.

## Test plan

### Unit tests

`crates/frob-conformance/tests/traction_counterexamples.rs` (host arithmetic, exact where rational).

- `cx167_enclosures`: 6 − 2·4 = −2 mm (overlap, `unknown`); 10 − 8 = +2 mm (`clear`).
- `cx169_thin_barrier`: 5 m/s × 2 ms = 10 mm; 1 + 4 + 2 = 7 mm; 10 − 7 = 3 mm.
- `cx171_coupling`: at v = r_eff·ω the factor is 1/√2 = 0.7071; independent rows give magnitude √2, 41% more.
- `cx172_backlash`: 0.01745 s and 0.0833 rad/s.
- `cx173_skid`: the 2 × 2 system v_y + a·ω = 0, v_y − a·ω = 0 has the unique solution v_y = ω = 0 for a ≠ 0.
- `cx176_half`: κ = 4,801; κ·2⁻¹¹ = 2.344; 63/27, 54/35, 54/18.
- `cx177_stefan`: 0.0377 s, 2.26 s, 0.763 s from t = 3π·η·a⁴/(4·N·h²).
- `cx179_strip`: 0.525, 0.690, 0.619.
- `cx182_chord`: (cos 32, sin 32) = (0.83422, 0.55143).
- `arc`, `resid`, `lambda`, `mode`: the provisional rows' numbers (0.55, 0.5, 1.05; 9.93·10⁻⁹, 2.38·10⁻⁷,
  7.95·10⁻⁸; 0.01414 against 0.00471; 4 N held, 6 N slides).

### End-to-end script and logging

`scripts/e2e/traction_controls.sh` (helper of STD-E2E): for each control whose mechanism exists, runs the gate's
conformance target twice (mutation off, mutation on) and asserts pass then fail; logs per NDJSON line the CX
number, the mutation flag, the gate, both outcomes, and the measured value that crossed the threshold under the
mutation. `summary` lists controls exercised, pending and failed.

### Known answers and negative controls

This bead is the negative-control registry of the chapter. Its own control: a planted registry row whose
mutation does not change behaviour (a no-op flag) must be reported by the script as "control did not fail its
gate", which is a failure of the control, not a pass.

## Considerations

- Rows CX-168 and CX-175 were printed with no numeric case. CX-175 takes the numeric instance of the scaled
  stopping test; CX-168 needs a fixture (two bodies crossing one region at different times) defined by the
  enclosure bead before its control can run.
- A mutation flag is test-only and must be impossible to enable in a release build (compile-time feature, checked
  by the release rubric).

## References

Appendix N.15 (CX-167 to CX-183); §18.19 and §18.20 for the provisional rows; §38.11 (false-progress guards).
