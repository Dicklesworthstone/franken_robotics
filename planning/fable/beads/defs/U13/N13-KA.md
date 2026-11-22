---
key: "N13-KA"
title: "Known-answer registry for Marrow: KA-228 to KA-260 and the turn-count answers, with tolerance, tier, control"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "marrow", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: []
related: ["BD-010", "BD-011", "BD-012", "BD-045", "BD-046", "BD-047", "BD-048", "BD-182", "BD-325", "N13-02", "N13-04", "N13-05", "N13-10", "N13-11", "N13-14", "N13-16", "N13-17", "N13-19", "N13-20", "N13-21", "N13-22", "N13-36", "N13-37", "N13-CX", "BD-146"]
gates: []
plan_refs: ["Appendix M.12", "§13.10", "§13.8", "§13.11", "§38.6"]
---
## Background
A known answer is an analytic or hand-checkable truth with a number, a tolerance, an execution tier and, where one exists, a negative control that must make its test fail. The known-answer suite is tier G1 of the verification program (G0 for laws and exact identities), registered in `frob-conformance`; a capability counts as implemented only when its known-answer tests pass against a pinned oracle that was never regenerated from the candidate.

This bead is the registry for the algebraic core: the thirty-three rows KA-228 to KA-260, plus the known answers of the periodic-and-lifted rule, which were stated with their subsection after the suite's table was compiled and have no number yet. Each row carries its case, its truth with formula or value, tolerance, tier and site, negative control, and the bead whose test implements it. Mechanism beads cite these numbers and restate the values they need inline; this bead makes the list itself an artifact and checks that every row has a live test. Tier of the content: Solid.

## Scope
In: the registry records; the completeness check; assignment of numbers to the unnumbered turn-count answers by the registry's owner; tracking of tolerances marked `to set`.
Out: the tests (owners in the last column); the counterexample registry (N13-CX).

## Technical approach
**Registry form.** As in N11-KA: `id`, `where_stated`, `case`, `truth`, `tolerance`, `tier`, `site`, `negative_control`, `test`, `owner_bead`, `status` per tier. A meta-test fails on a record without a test, a missing test, or a tolerance still `to set` when its gate is recorded.
**The rows.** "Gate bound" means the bound of GATE-MARROW-01: 10⁶ cases, at most 64 ulp in `f64` and 16 ulp in `f32` for single operations.

| KA | The case | The truth | Tolerance | Tier · site | Negative control | Test owner |
|---|---|---|---|---|---|---|
| KA-228 | Planar 2R arm: links 0.4 m and 0.3 m, angles 0 and π/2, tool force (10, 0) N, joint rates (0.2, −0.1) rad/s | Point Jacobian J = [[−0.3, −0.3], [0.4, 0.0]] m/rad; joint torques Jᵀ·f = (−3, −3) N·m; tool velocity J·q̇ = (−0.03, 0.08) m/s; power −0.3 W by both routes (f·v and τ·q̇) | exact in rationals on the host; floating tolerance to set | G1 · host | The lower three rows of a space screw Jacobian taken as a point Jacobian, which gives torques (0, 0) | BD-047 |
| KA-229 | Free body, rotational inertia diag(1, 2, 3) kg·m², ω = (1, 1, 0) rad/s, so π = (1, 2, 0) | π̇ = π × ω = (0, 0, −1); body-frame rate of spatial angular momentum π̇ + ω × π = (0, 0, 0) | exact (integers) | G1 · host | A reversed ad*: π̇ = (0, 0, +1) and spatial rate (0, 0, 2). Both signs give ω·π̇ = 0, so an energy-only test passes the mutation | BD-010, BD-011 |
| KA-230 | Fore-aft and left-right symmetric quadruped with one forward lidar on the torso | Γ_mech = C₂ × C₂ (order 4); Γ_obs = C₂ (order 2); the breaker of the fore-aft mirror and of the half-turn is the lidar mount | exact | G1 · host | Weights tied under the mechanical group | N13-16, N13-21 |
| KA-231 | C₃ acting on ℝ³ by cyclic permutation of three legs | Commutant dimension 3 = 1 (trivial) + 2 (one complex-type real irreducible); the maps are the circulants; the antisymmetric circulant x ↦ (x₂ − x₃, x₃ − x₁, x₁ − x₂) is representable | exact | G1 · host | The complex-type block parameterized as real: two parameters, symmetric circulants only, which are the D₃-equivariant maps | N13-14, N13-23, BD-325 |
| KA-232 | Regular representation of C₂ × C₂, 64 copies in and 64 copies out | 16,384 free weights, from dim Hom = Σᵢ cᵢ·mᵢ^out·mᵢ^in with four real-type irreducibles of multiplicity 64 | exact | G1 · host | A miscounted commutant | N13-14, BD-325 |
| KA-233 | Ten module types on the six slots of a radial hexapod under D₆ | (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = 86,185 orbits out of 10⁶ assignments | exact | G1 · host | A wrong cycle index | N13-17 (mechanism BD-049) |
| KA-234 | The same sweep when the sensors leave only the sagittal mirror | (10⁶ + 10³)/2 = 500,500 orbits | exact | G1 · host | A wrong cycle index | N13-17 |
| KA-235 | Contact strata: four feet, each open, sticking or slipping, under C₂ × C₂ | (3⁴ + 3·3²)/4 = 27 strata | exact | G1 · host | A wrong cycle index | N13-17 |
| KA-236 | Guarded predicate x² + y² ≤ z² on the single-precision numbers nearest 0.3, 0.4 and 0.5 | The exact value of x² + y² − z² on the stored inputs is +1.19 × 10⁻⁸ (13,421,773·2⁻⁵⁰): the point is outside the cone; the guarded predicate returns that sign | exact (sign) | G1 · all tiers | A plain single-precision predicate, which finds the two sides equal and reports the point inside | N13-10 (host), BD-182 (devices) |
| KA-237 | Rotation angle of one Cayley step at body rate ω and step h | 2·atan(ω·h/2) | to set (the gate bounds the retraction laws) | G1 · host | A mislabelled retraction | BD-011, BD-012 |
| KA-238 (≡ KA-044) | A 1 kg part with principal moments 0.008 kg·m² each, declared inside a sphere of radius 0.10 m | Refused by the mass-support predicate (FROB-REF-106): ‖c − o‖² + tr(I_C)/(2m) = 0.012 m² needed against R² = 0.010 m² available; a thin spherical shell of that radius meets the bound with equality | exact | G1 · host | An inertia that passes pseudo-inertia positivity and that no body in the envelope could have | N13-04 (mechanism BD-051) |
| KA-239 | Law: group axioms | Associativity, identity and inverse for every group | gate bound | G0 · host | The eight planted mutations, each caught by at least three fixtures (GATE-MARROW-04) | BD-010, BD-012, BD-045; N13-18, N13-19 |
| KA-240 | Law: retraction | τ(0) = identity; dτ(0) = identity; τ(x)·τ(−x) = identity, for cay and exp | gate bound | G0 · host | none stated | BD-011, BD-045 |
| KA-241 | Law: round trip | log(exp(x)) = x inside the injectivity radius; cay⁻¹(cay(x)) = x | gate bound | G0 · host | none stated | BD-011, BD-045 |
| KA-242 | Law: adjoint | Ad(gh) = Ad(g)·Ad(h); Ad(exp x) = exp(ad x) to tolerance; the group coadjoint Ad* is the inverse transpose of Ad | gate bound | G0 · host | Ad* and its inverse exchanged | BD-011 |
| KA-243 | Law: coadjoint pair | ad*_ξ = (ad_ξ)ᵀ; d/dt Ad*_{exp(tξ)} at t = 0 equals −ad*_ξ; push-forward composed with pull-back is the identity | gate bound | G0 · host | A reversed ad*; a pull-back in place of a push-forward | BD-011, BD-047 |
| KA-244 | Law: pairing | ⟨Ad*_g w, Ad_g ξ⟩ = ⟨w, ξ⟩ under a change of frame | gate bound | G0 · host | A pull-back in place of a push-forward; linear and angular parts exchanged in a twist | BD-047 |
| KA-245 | Law: free body | With μ̇ = ad*_ξ μ the spatial momentum Ad*_g μ is constant | gate bound | G0 · host | A reversed ad* | BD-011 |
| KA-246 | Law: Jacobians of the retractions | dexp and dcay match `fs-ad` dual-number derivatives | gate bound | G0 · host | none stated | BD-011, BD-045 |
| KA-247 | Law: subgroups | A classified screw basis is bracket-closed; the generated joint map composes like its subgroup | exact for closure; gate bound for composition | G0 · host | The bracket's arguments exchanged | BD-046 |
| KA-248 | Law: reflections | A mirror applied twice is the identity on twists, wrenches and pseudo-inertias; the pairing is invariant under every isometry, proper or not; a mirrored revolute joint written with the axial image det(P)·P·a of its axis keeps its coordinate | gate bound | G0 · host | A missing parity sign on a mirrored axis | N13-02 |
| KA-249 | Law: representations | ρ(gh) = ρ(g)·ρ(h); characters orthonormal; projectors idempotent and summing to the identity | exact (field elements) | G0 · host | A conjugate missing in the projector | BD-048 |
| KA-250 | Law: nested groups | Γ_obs is a subgroup of Γ_mech; every element of Γ_obs has a representation on every task term; removing every sensor, payload and task term gives Γ_obs = Γ_mech | exact | G0 · host | none stated | N13-16 |
| KA-251 | Law: blocks | The block-parameterized maps and the image of the Reynolds operator are the same subspace, of dimension Σ cᵢ·mᵢ^out·mᵢ^in (cᵢ = 1 real type, 2 complex type) | exact (PO-MARROW-05; GATE-MARROW-04: 100 random pairs of channel spaces per group) | G0 · host | A complex-type block parameterized as real (the C₃ case) | BD-325 (counting half N13-14) |
| KA-252 | Law: orbits | The cycle-index count equals a brute-force count for every case with at most 10⁶ assignments | exact | G0 · host | A wrong cycle index | N13-17 |
| KA-253 | Law: predicates | A `Guarded` predicate returns the sign of its exact evaluation on the stored inputs, for every input | exact (sign) | G0 · all tiers | A `Plain` predicate on a select whose arms differ at the boundary (the 0.3, 0.4, 0.5 fixture) | N13-10, N13-11, BD-182 |
| KA-254 | Law: curvature sign | With body velocities and reconstruction ξ = −𝔸(s)·ṡ the curvature is (D𝔸)ᵢⱼ = ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ], and a small loop moves the body by about exp(−∬ D𝔸) | to set (holonomy fixture) | G0 · host | The curvature bracket sign reversed, which fails the holonomy fixture | N13-05 |
| KA-255 | Law: cones | P(P(x)) = P(x); Moreau decomposition x = P_𝒦(x) + P_𝒦°(x) with the parts orthogonal; the four-dimensional contact cone is the image of the second-order cone under its stated linear map | gate bound | G0 · host | none stated | N13-19 (mechanisms BD-050, BD-154) |
| KA-256 | Convention parity with `fs-ga` (PO-MARROW-01): quaternion order, sign rule, twist order, adjoint direction, bracket, group coadjoint | A fixture of 1,000 transforms produced by `fs-ga` and consumed by `frob-lie` compares bit for bit, covering the adjoint, the group coadjoint, the pull-back and the bracket | exact (bits) | G2 · host | The eight planted mutations | BD-010 |
| KA-257 | Turn counter of the unit-complex revolute coordinate (PO-MARROW-03) | Never disagrees with an angle integrated in `f64`, over 10⁹ random steps | exact (integer turn count) | G0 · host | none stated | BD-012 |
| KA-258 | Character tables and isotypic projectors (GATE-MARROW-02) | Correct for Cₙ and Dₙ with n ≤ 12, their products with C₂, and S₄, against the tables on file | exact | G1 · host | A conjugate missing in the projector | BD-048, N13-20 |
| KA-259 | Γ_mech and Γ_obs for the eighteen archetypes and for 24 planted asymmetries (a moved sensor, a one-sided payload, a changed sole material, a reversed gear) (GATE-MARROW-03) | The groups on file, with the correct breaker named in every case; zero disagreements | exact | G1 · host | The 24 planted asymmetries are the controls: each must change the group and name its breaker | N13-21, N13-22 |
| KA-260 | Guarded predicates against exact rational evaluation on 10⁹ adversarial inputs placed within 4 ulp of the zero set (GATE-MARROW-05) | Zero sign errors on `batch-cpu`, CUDA and Metal; fallback rate on the standard contact battery recorded, TARGET below 10⁻³ of evaluations | exact (sign) | G1 · all tiers | A `Plain` strict predicate; the guard kill gate fires above a 10⁻² fallback rate | BD-182 |

**Unnumbered known answers of the periodic-and-lifted rule** (GATE-MARROW-06; the registry owner assigns the next free numbers and this bead records the assignment):
| Proposed slot | The case | The truth | Tolerance | Tier · site | Negative control | Test owner |
|---|---|---|---|---|---|---|
| turn-A | A joint with a linear torsion spring of 0.5 N·m/rad, rest angle 0, held at θ = 0 and at θ = 2π | Energy 0 and π² J (9.8696 J); torque 0 and −π N·m; identical link poses and zero velocity in both | 10⁻¹² in double precision on the referee; 10⁻⁵ relative on device tiers | G1 · referee, all tiers | A spring evaluated without the turn count reads 0 J at 2π; the snapshot mutant that omits k is caught by an energy jump of π² J | N13-34, N13-36, N13-37 |
| turn-B | A wheel with only periodic laws | Replays bit-identically from a state with k dropped | exact (digest) | G5 · referee, all tiers | The row is the control showing the rule binds only lifted laws | N13-35, N13-36, N13-37 |
| turn-C | Snapshot, restore and teleport of 10⁴ random states | k preserved exactly | exact (integers) | G5 · all tiers | A snapshot or teleport that omits or re-derives k | N13-35, N13-37 |
| turn-D | Single-turn and multi-turn encoders on the spring fixture's two worlds | `sense.alias` returns the witness for a single-turn encoder (holding torques differ by π N·m) and none for a multi-turn encoder | exact (verdict) | G1 · referee | An alias search that compares poses instead of observation histories and required actions | N13-36 |

**Related corrected fixture carried by this chapter's tests** (stated in the verification chapter): four feet, each open, stick or slip: 27 strata and 162 one-foot transitions under the four-element leg-permutation group; under the two-element subgroup that a forward-facing sensor leaves, (3⁴ + 3²)/2 = 45 strata and 648/2 = 324 transitions (owner N13-17).
**Rows with "none stated".** The test beads supply controls, recorded in the registry: KA-240 and KA-241 a wrong small-angle arm and a log returned from the wrong side of a chart cut; KA-246 a wrong sign in the ω̂ term of dcay and the planted SE(2) Jacobian error; KA-250 an unfiltered Γ_obs; KA-255 a non-idempotent projection; KA-257 the three counter mutants of BD-012.

## Acceptance criteria
- [ ] Thirty-three records, KA-228 to KA-260, exist with every field filled; the four turn-count answers are recorded with numbers assigned by the registry's owner (or as named slots until assigned).
- [ ] The meta-test passes: every record names an existing test; every test tagged with one of these numbers is registered.
- [ ] No tolerance is `to set` when its gate is recorded (rows to settle: KA-228's floating tolerance, KA-237, KA-254).
- [ ] Every row has a negative control recorded (stated, or supplied by its test bead) and present in the negative-control registry.
- [ ] The count for this chapter is 33 numbered known answers, of which 6 have no stated control and 3 a tolerance to set, matching the suite's coverage table; plus 4 late answers.

## Test plan
### Unit tests
`ka_registry_marrow_complete` (33 contiguous ids plus the late slots); `ka_registry_tests_exist`; `ka_registry_no_unset_tolerance_at_gate`; `ka_registry_controls_registered`; `ka_238_is_alias_of_044` (the two ids resolve to one test and one value).
### End-to-end script and logging
`scripts/e2e/marrow_laws.sh --gate` (N13-19) and the gate scripts of BD-182, N13-20, N13-22 and N13-37 each end by printing the rows of this registry they exercised with status and measured values; this bead's own check, `scripts/e2e/marrow_ka_registry.sh`, loads the registry and the latest logs under `target/e2e-logs/`, and reports per row the last status and the log line that proves it, in `STD-E2E` format at `target/e2e-logs/marrow_ka_registry/<utc>.ndjson`: one `assert` line per KA (id, status, measured, expected, tolerance, tier, site, source log) and a summary of rows passed, failed and not run.
### Known answers and negative controls
The rows above. Determinism: the registry is data; the meta-tests are exact.

## Considerations
- KA-238 duplicates an earlier row (the mass-support case stated with the design language); the registry keeps one value under two ids.
- Rows tied to later gates (KA-251 at M4, KA-259 at M3, KA-260 at M2) stay `not run` until then; a row is never deleted to make a milestone look complete.
- A known answer is never edited to match the candidate; corrections are dated amendments with their derivation.

## References
Appendix M.12 (KA-228 to KA-260), §13.8, §13.10, §13.11, §38.6, §38.17.
