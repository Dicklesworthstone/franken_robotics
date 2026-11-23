---
key: "N11-KA"
title: "Known-answer registry for the conventions: KA-191 to KA-206 with truth, tolerance, tier, control and test"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "marrow", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: []
related: ["N13-01", "N13-02", "N13-04", "N13-05", "N13-06", "N13-07", "BD-010", "BD-011", "BD-047", "BD-146", "N11-CX", "N20a-49"]
gates: []
plan_refs: ["Appendix M.10", "§11", "§38.6"]
---
## Background
A known answer is an analytic or hand-checkable truth with a number, a tolerance, an execution tier and, where one exists, a negative control: a planted defect that must make the test fail. The known-answer suite is tier G1 of the verification program (with G0 for laws and exact identities), and its rows are registered in `frob-conformance` so that no enumerable truth is lost between the chapters that state it and the tests that check it. A capability counts as implemented only when its known-answer tests pass against a pinned oracle that was never regenerated from the candidate.

This bead is the registry for the conventions: the sixteen rows KA-191 to KA-206. Each row below carries its case, its truth with the formula or value, its tolerance, its tier and site, its negative control, and the bead whose test implements it. Mechanism beads cite these numbers and restate the values they need; this bead makes the list itself an artifact and checks that every row has a live test. Tier of the content: Solid.

## Scope
In: the sixteen registry records; the completeness check (every record names an existing test; every test that claims a KA number names a registered record); tolerances marked `to set` tracked until set.
Out: the tests themselves (owners in the last column); the counterexample registry (N11-CX).

## Technical approach
**Registry form.** One record per row in the known-answer registry of `frob-conformance` (the machine-readable form of the known-answer suite): `id`, `where_stated`, `case`, `truth`, `tolerance`, `tier`, `site`, `negative_control`, `test` (crate and test name), `owner_bead`, `status` (`not run`, `pass`, `fail`). A meta-test loads the registry and fails if a record has no test, if a named test does not exist, or if a tolerance is still `to set` when its owning gate is recorded.
**The rows.** Notation: g = (R, p) = `a_from_b`; ξ = (ω, v) a twist; μ = (π, ℓ) a momentum; w = (τ, f) a wrench.

| KA | Stated in | The case | The truth | Tolerance | Tier · site | Negative control | Test owner |
|---|---|---|---|---|---|---|---|
| KA-191 | adjoint, coadjoint and the Euler–Poincaré sign | Free body with the frame origin at the centre of mass: π = I·ω and ℓ = m·v in μ̇ = ad*_ξ μ with ad*_ξ = ad_ξᵀ | Euler's equation I·ω̇ = (I·ω) × ω and v̇ = v × ω; the world-frame velocity R·v of a force-free body is constant, because d(R·v)/dt = R·(ω × v + v̇) = 0 | to set (residual) | G0 · host | With the opposite sign d(R·v)/dt would be 2R·(ω × v) | BD-011, N13-01 |
| KA-192 | same | Coadjoint as the dual map (theorem card T-16) | ⟨ad_ξᵀ μ, η⟩ = ⟨μ, [ξ, η]⟩, a polynomial identity in eighteen variables; ad*_ξ μ = (π × ω + ℓ × v, ℓ × ω) | exact (polynomial identity over ℚ) | G0 · host | The map −ad_ξᵀ written as ad* | BD-010, BD-011, N13-01 |
| KA-193 | three typed maps between frames | Wrench push-forward with R = I and p = (1, 0, 0) m applied to the wrench (0, (0, 0, 1) N) in b | Torque p × f = (0, −1, 0) N·m and the same force | exact | G0 · host | Moving the wrench with the twist map gives zero torque: the moment arm is lost, and the planar 2R fixture fails | BD-010, BD-047 |
| KA-194 | same | Laws of the three typed frame maps | Ad(gh) = Ad(g)·Ad(h); ⟨Ad_g⁻ᵀ w, Ad_g ξ⟩ = ⟨w, ξ⟩; the pull-back Ad_gᵀ inverts the push-forward | exact over ℚ (T-04, T-05); residual tolerance to set in floating point | G0 · host | A function that accepts one map type where another is expected must not compile | BD-047 |
| KA-195 | reflection parity | Sagittal mirror P = diag(1, −1, 1) | A pitch axis e_y maps to itself, a roll axis e_x to −e_x and a yaw axis e_z to −e_z; a wheel rolling forward keeps its angular momentum; a robot turning left becomes a robot turning right | exact | G0 · host | Treating angular velocity as polar leaves the yaw rate unchanged, which the step-equivariance test catches | N13-02 |
| KA-196 | same | Parity map of an improper isometry (T-16) | T(P) = diag(det(P)·P, P) preserves the pairing, because det(P)² = 1, and the bracket | exact (polynomial identity over ℚ) | G0 · host | none stated | N13-02 |
| KA-197 | inertia: shells | Shell rule in the thin limit | As t → 0 a spherical shell of radius r tends to I = (2/3)·m·r², against (2/5)·m·r² for the solid ball | to set | G1 · host | A solid default for an enclosure (refusal FROB-REF-168; a box case with a factor of 18.8) | N13-04 |
| KA-198 | inertia: pseudo-inertia | Pseudo-inertia round trip | 𝖯 = [[Σ, h], [hᵀ, m]] with Ī = tr(Σ)·I − Σ and Σ = ½·tr(Ī)·I − Ī; the parameters are consistent exactly when 𝖯 ≻ 0 | exact (rational LDLᵀ, T-19) | G0 · host | none stated | N13-04 (mechanism BD-051) |
| KA-199 | inertia: mass support | A body whose mass lies in a ball of centre o and radius ρ | ‖c − o‖² + tr(I_C)/(2m) ≤ ρ², the left side being the mass-weighted mean squared distance from o, since ∫‖x − c‖² dm = tr(Σ_C) = ½·tr(I_C) | exact (one rational inequality, T-19) | G0 · host | The bound applied to an unrelated collision proxy instead of the declared mass envelope | N13-04 (mechanism BD-051) |
| KA-200 | curvature of the connection | The constant connection 𝔸₁ = e_x, 𝔸₂ = e_y on SO(3) | The curl vanishes and (D𝔸)₁₂ = −e_z, so a small loop of area da rotates the body by +da about z; directly, exp(−a·e_x)·exp(−b·e_y)·exp(a·e_x)·exp(b·e_y) ≈ exp(a·b·e_z) | to set (second order in the loop sides); exact in truncated rational series (T-20) | G0 · host | A formula with +[𝔸ᵢ, 𝔸ⱼ] predicts −da and is wrong by twice the answer | N13-05 |
| KA-201 | rational clocks | Physics at 400 Hz, an IMU at 100 Hz and a policy at 30 Hz | L = 1,200 and periods of 3, 12 and 40 base units; the policy tick at unit 40 falls one unit after the physics boundary at 39 and two before the one at 42; the pattern repeats every 120 units, 0.1 s | exact (integers) | G1 · all tiers | Rounding the policy to every thirteenth physics step | N13-06 (mechanism BD-078); all tiers N13-07 |
| KA-202 | same | Firing count of a periodic process | The number of firings of process i up to clock n is ⌊(n − φᵢ)/Nᵢ⌋ + 1 exactly | exact | G1 · all tiers | none stated | N13-06 |
| KA-203 | same | Replay from a checkpoint taken just after a shutter opens, after a dropped packet, or just before a controller update | The same packet history as the uninterrupted run | exact (digest) | G5 · all tiers | none stated here (a checkpoint contains every accumulator, scheduled callback, pending packet and action queue) | N13-06 (mechanism BD-098); all tiers N13-07 |
| KA-204 | the seven instants | Force sensor over a half-open exposure window (a, b] | An impact of 0.2 N·s inside a 10 ms exposure contributes 20 N to that window's average and nothing to its neighbour; an impact at exactly instant b belongs to the window ending at b | exact | G1 · all tiers | none stated | N13-06 |
| KA-205 | same | Packet timing in the docking fixture | The same packet is on time when its arrival precedes the second decision and worthless when it follows the third command | exact (verdict) | G1 · host | none stated | N13-06 |
| KA-206 | same | Causality of observations | A packet is never visible before its arrival; private truth behind a disconnected sensor cannot change an action; an out-of-order arrival never changes a past decision or the observation delivered at that decision | exact (digest of actions) | G3 · all tiers | The two mutation tests: a consumer that reads a packet before its arrival; a path from private truth to an action | N13-06 |

**Rows whose negative control is "none stated".** The registry does not leave them without one where a test bead supplies it: KA-196 uses the missing-parity mutation (N13-02); KA-198 a matrix one eigenvalue below zero; KA-202 a floating-point time comparison; KA-203 packet queues left out of the checkpoint; KA-204 closed-closed windows; KA-205 ordering by timestamp instead of arrival. These are recorded in the `negative_control` field with the note "supplied by the test bead".

## Acceptance criteria
- [ ] Sixteen records, KA-191 to KA-206, exist in the known-answer registry with every field filled; `status` reflects the last run.
- [ ] The meta-test passes: each record names an existing test in the owning crate; each test tagged with one of these numbers is registered.
- [ ] No tolerance is `to set` at the time the gate or battery that owns the row is recorded (KA-191, KA-194, KA-197, KA-200 are the rows to settle; their values are recorded with the conventions battery and GATE-MARROW-01).
- [ ] Every row has a negative control recorded, either the stated one or the one supplied by its test bead, and that control is present in the negative-control registry.
- [ ] The registry's count for this chapter is 16, matching the suite's coverage table (16 known answers, of which 6 have no stated control and 4 have a tolerance to set).

## Test plan
### Unit tests
`ka_registry_conventions_complete` (16 ids, contiguous, all fields present); `ka_registry_tests_exist` (each `test` resolves to a test function); `ka_registry_no_unset_tolerance_at_gate` (run in gate mode); `ka_registry_controls_registered`.
### End-to-end script and logging
`scripts/e2e/conventions.sh` (shared with N13-01 and N13-07) prints, after the battery, a table of these sixteen rows with status and the measured value against the truth; logging per `STD-E2E` to `target/e2e-logs/conventions/<utc>.ndjson`: one `assert` line per KA (id, status, measured, expected, tolerance, tier, site), and the final summary counts KA rows passed, failed and not run.
### Known answers and negative controls
The sixteen rows above, with their controls. Determinism: the registry is data; the meta-tests are exact.

## Considerations
- Rows marked "all tiers" are measured on the host or referee at M1 and on every tier at M2; the record's `status` is per tier.
- A known answer is never edited to match the candidate; a correction to a row is a dated amendment with its derivation.
- The docking-fixture row depends on the mission slice's fixture; if that fixture is late the row stays `not run`, never removed.

## References
Appendix M.10 (KA-191 to KA-206), §11.2 to §11.8, §38.6.
