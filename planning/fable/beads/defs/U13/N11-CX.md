---
key: "N11-CX"
title: "Counterexample registry for the conventions: CX-098 to CX-105 as negative controls with their gates"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "marrow", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: []
related: ["N11-KA", "N13-01", "N13-02", "N13-04", "N13-05", "N13-06", "BD-011", "BD-047", "BD-146", "N20a-49"]
gates: []
plan_refs: ["Appendix N.8", "§11", "§38.11", "§0.7"]
---
## Background
The project's rule is that a plausible shortcut ships with the smallest hand-checkable case that breaks it, and that the case becomes a permanent negative control: a planted defect that a gate must catch. A rule that ships without such a case has not been thought through, and a gate that no deficient model can fail validates nothing. The negative-control registry (`frob-conformance/registry/controls.toml`) is seeded from the counterexample catalogue: each entry becomes at least one control, attached to the gates of the rule it protects.

This bead is the catalogue for the conventions: the eight entries CX-098 to CX-105. For each: the tempting shortcut, the smallest case that breaks it with its numbers, the rule it protects, and the control record it becomes. Tier of the content: Solid.

## Scope
In: the eight catalogue entries as registry records; one or more control records per entry; the completeness check.
Out: the tests that run the controls (owners listed); the known-answer registry (N11-KA).

## Technical approach
**Record form** (one per control, as the registry defines it): `id` (stable, never reused), `seed` (the catalogue entry), `gate` (every gate or battery that must catch it), `kind` (`mutation`, `planted-input`, `planted-design`, `planted-world`), `target`, `defect`, `expect`, `last_seen`.
**The entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control: kind, target, expected observation | Owner |
|---|---|---|---|---|---|
| CX-098 | The coadjoint in the Euler–Poincaré equation carries a minus sign: μ̇ = −ad_ξᵀ μ | Force-free body with the origin at the centre of mass. With ad*_ξ = ad_ξᵀ: v̇ = v × ω, so d(R·v)/dt = R·(ω × v + v̇) = 0 and the world-frame velocity is constant. With the opposite sign d(R·v)/dt = 2R·(ω × v): a free body changes its velocity with no force | ad*_ξ = ad_ξᵀ with μ̇ = ad*_ξ μ + w; the map −ad_ξᵀ is never written ad* (theorem card T-16; proof obligation PO-MARROW-01) | mutation of `coad`; the world-velocity residual equals 2R·(ω × v) and the free-body known answer flips to (0, 0, +1) | BD-011, N13-01 |
| CX-099 | A wrench changes frame by the same matrix as a twist | R = I, p = (1, 0, 0) m, wrench (0, (0, 0, 1) N) in b. The push-forward Ad_g⁻ᵀ gives torque p × f = (0, −1, 0) N·m and the same force. The twist map Ad_g gives zero torque: the moment arm is lost, and the planar 2R fixture fails | Three typed frame maps, `TwistMap`, `WrenchPushforward`, `WrenchPullback`; no function accepts one where another is expected | mutation at a use site (raw matrices); torque (0, 0, 0) instead of (0, −1, 0); and a compile-fail case for the typed API | BD-047 |
| CX-100 | Angular velocity mirrors like any other vector | Sagittal mirror P = diag(1, −1, 1). Axial vectors map by det(P)·P = −P: the yaw axis e_z goes to −e_z, so a robot turning left becomes a robot turning right, and the pitch axis e_y maps to itself. Treated as polar, the yaw rate is left unchanged | Polar vectors map by P and axial vectors by det(P)·P; the step-equivariance test | mutation of the parity rule; yaw axis image e_z instead of −e_z; mirrored-joint law fails | N13-02 |
| CX-101 | A body with no declared wall thickness can default to a solid of its outer shape | A thin spherical shell of radius r has I = (2/3)·m·r², a solid ball (2/5)·m·r²; for an enclosure the solid default is wrong in mass by an order of magnitude (a box case with a factor of 18.8) | The shell rule; a solid body above the plausibility bound of its part class is refused unless the fill is declared (FROB-REF-168) | planted-design: an enclosure written solid without a statement of intent; expected refusal, not a 64.8 kg body | N13-04 (rule decided in N13-03) |
| CX-102 | The curvature of the connection is ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ + [𝔸ᵢ, 𝔸ⱼ] | Constant connection 𝔸₁ = e_x, 𝔸₂ = e_y on SO(3): the curl vanishes, (D𝔸)₁₂ = −[e_x, e_y] = −e_z, and a small loop of area da rotates the body by +da about z, as exp(−a·e_x)·exp(−b·e_y)·exp(a·e_x)·exp(b·e_y) ≈ exp(a·b·e_z) confirms. The + formula predicts −da, wrong by twice the answer | (D𝔸)ᵢⱼ = ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ] with ξ = −𝔸(s)·ṡ (T-20; the wrong-sign control of the holonomy construction) | mutation of `curvature_form`; predicted rotation −da against measured +da | N13-05 |
| CX-103 | A 30 Hz policy on 400 Hz physics can run every thirteenth physics step | L = 1,200 base units per second; physics every 3 units, policy every 40. The policy tick at unit 40 falls one unit after the physics boundary at 39 and two before the one at 42; thirteen steps are 39 units, a period of 32.5 ms instead of 33⅓ ms | Rational periods with integer phase accumulators; a process need not be a multiple of the physics step, and no period is rounded | planted-input: a schedule with period 39 units; the tick sequence differs at the first tick and the rate is 30.77 Hz | N13-06 |
| CX-104 | A force sensor reading is the force at the sample instant, and an impact on a window boundary can go to either window | An impact of 0.2 N·s inside a 10 ms exposure contributes 20 N to that window's average and nothing to its neighbour; an impact at exactly instant b belongs to the window ending at b | A reading is a functional of the history over the half-open window (a, b]; each impulse is added once and divided by the exposure | mutation: point sampling (reads 0 N), and closed-closed windows (the boundary impact counted in two windows: 20 N twice) | N13-06 |
| CX-105 | A decision may use any packet whose timestamp is earlier than the decision | The timestamp is a label written on the sensor's own clock and is unconstrained by causality. In the docking fixture the same packet is on time when its arrival precedes the second decision and worthless when it follows the third command, whatever its timestamp | A decision uses only packets whose arrival is not later than the decision; a packet is never visible before its arrival | mutation: a consumer that orders or admits by timestamp; the command digest changes for a late packet with an early stamp | N13-06 |

**Gates and batteries each control attaches to.** CX-098, CX-099, CX-100, CX-102: GATE-MARROW-01 (the law table) and the conventions battery; CX-098, CX-099, CX-100, CX-102 are also four of the eight Marrow mutations measured for GATE-MARROW-04. CX-101: the design-check battery of the fill rule. CX-103, CX-104, CX-105: the conventions battery rows 13 to 15 and the sensing gates that own the clock contract and the packet queues.
**Completeness check.** A meta-test loads the control registry and fails if one of the eight catalogue ids has no control, if a control names no gate, or if a control's target test does not exist.

## Acceptance criteria
- [ ] Eight catalogue entries are represented by at least one control record each (ten records in all: CX-099 and CX-104 have two), with every field filled.
- [ ] Each control is run whenever its gate or battery is run, and a run in which a control was not caught is a failure of that gate or battery.
- [ ] Every control has been observed caught at least once (`last_seen.caught = true`) before the M1 exit; a control never observed is reviewed as suspect.
- [ ] The meta-test passes; the count for this chapter is 8 catalogue entries, matching the catalogue's coverage table.

## Test plan
### Unit tests
`cx_registry_conventions_complete` (eight ids, each with a control); `cx_controls_have_gates_and_targets`; `cx_controls_fire` (each control, when active, makes its named test fail by a value assertion with the expected observation recorded above).
### End-to-end script and logging
`scripts/e2e/conventions.sh --controls` runs the conventions battery once per control with that control active and asserts failure; `STD-E2E` log lines at `target/e2e-logs/conventions/<utc>.ndjson`: one `assert` per control (id, seed, gate, caught, observed value, expected observation), and a summary with controls caught and not caught.
### Known answers and negative controls
The numbers in the table are the known answers of N11-KA seen from the failing side: (0, −1, 0) N·m against zero torque; e_z ↦ −e_z against unchanged; +da against −da; 33⅓ ms against 32.5 ms; 20 N against 0 N. Determinism: exact.

## Considerations
- A control is a defect, not a boundary: "the request is refused at the limit" tests the limit; the control is the build or input in which the rule was removed.
- If a false result is ever found in use that traces to a convention, it becomes a permanent control here with the case that exposed it.
- CX-101's control depends on the plausibility rule decided in N13-03; until then it is attached to the provisional rule.

## References
Appendix N.8 (CX-098 to CX-105), §11.2 to §11.8, §38.11 (negative-control registry), §0.7 (counterexamples are first-class).
