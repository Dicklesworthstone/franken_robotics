---
key: "N15-CX"
title: "registry: Sinew counterexamples CX-135 to CX-143 as negative controls, plus four from coupled transmissions"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sinew", "frob-actuate", "frob-conformance", "solid", "registry", "test"]
parent: "WP-09"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§15.13: the known-answer registry of the chapter (N15-KA, written by the unit that owns §15.1 to §15.9), for the KA numbers each control pairs with"]
related: ["N15-REF", "BD-072", "BD-073", "BD-074", "BD-075", "BD-076", "BD-084", "BD-146", "N15b-05", "N15b-06", "N15b-07", "N15b-10", "N15b-11", "N15b-12"]
gates: []
plan_refs: ["§15.7", "§15.8", "§15.9", "§15.10", "§15.11", "§15.12", "§15.15", "Appendix N.12", "§0.7", "§38"]
---
## Background

A plan rule binds every chapter: where a plausible shortcut is wrong, the smallest hand-checkable case that
breaks it is written down, and that case enters the negative-control registry of the verification program
(crate `frob-conformance`, seed bead BD-025). The reason is that a gate nothing can fail validates nothing: a
check is only trusted once a planted defect has been shown to trip it. For Sinew (actuators, transmissions,
power; crate `frob-actuate`) the catalogue holds nine counterexamples, CX-135 to CX-143. The late subsection
on coupled transmissions adds four more shortcuts that have no catalogue number yet; this bead records them and
asks the registry owner for numbers, so that they are not lost.

Each row below states the tempting shortcut, the case with its numbers, the rule it protects, the planted
mutant that realizes the shortcut, and the bead whose test must catch the mutant.

## Scope

In: the registry entries (`registries/negative-controls/sinew.toml`), one planted mutant per row compiled
behind a test-only Cargo feature, the pairing of each mutant with the test that must fail when it is on, and
the script that runs all of them. Out: the correct mechanisms (owned by the beads named in the last column)
and the known-answer registry (N15-KA).

## Technical approach

**How a control is planted (DESIGN).** A mutant is a `#[cfg(feature = "mutant-<name>")]` branch inside the
owning crate's test support module or, where the defect must be in production arithmetic, a function-pointer
seam that tests alone can set. No mutant is compiled into `frob`, `frobd` or `frob-mcp`. The registry row
names the feature, the test that must go red and the expected first failing assertion. The conformance
harness builds each mutant, runs the paired test, and records `caught` or `missed`; a `missed` is a failed
gate for the bead that owns the test.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Planted mutant | Caught by |
|---|---|---|---|---|---|
| CX-135 | One efficiency multiplier describes a gear in all four quadrants | η = 0.8, N = 10, the load driving the joint with 10 N·m at 1 rad/s: 10 W in. The law τ_joint = η·N·τ_motor gives τ_motor = 1.25 N·m at 10 rad/s, so 12.5 W reaches the motor and 2.5 W came from nowhere. The four-quadrant law gives 0.8 N·m and 8 W, with 2 W lost | Four-quadrant loss with η_f and η_b; INV-SINEW-01; FROB-REF-130; GATE-SINEW-08 | `mutant-gear-single-eta`: the transmission applies η in every quadrant | BD-074 (card check refuses; passivity battery) |
| CX-136 | Copper resistance can be held constant in the thermal check | R₀ = 0.20 Ω, α = 0.00393 K⁻¹, G = 0.80 W/K, C = 120 J/K, ambient 25 °C, limit 155 °C, constant 20 A. Constant resistance: rise 80/0.80 = 100 K, 125 °C, inside the limit. With R(T): s = 0.314 W/K, rise 80/(0.80 − 0.314) = 164.7 K, 190 °C, outside the limit, reached at 385 s. Largest continuous current falls from 22.8 A to 18.6 A; above I_crit = 31.9 A no equilibrium exists | The static check uses the corrected law (ADR-SINEW-04); FROB-REF-132; GATE-SINEW-05 | `mutant-thermal-const-r`: α set to zero inside the static check | BD-072 (thermal closed forms); BD-103 (stage 6 admits the 20 A design) |
| CX-137 | Under torque holding the runaway threshold can be read from the slope of the loss curve at T₀ | Same card with flux coefficient α_B = −0.0011 K⁻¹. The slope estimate √(G/(R₀·(α − 2·α_B))) gives 25.5 A; the loss curve is convex and the equilibrium vanishes at a tangency at 16.5 A cold-equivalent. The limit is reached at 15.9 A, so rating and runaway threshold are four percent apart | `actuator.thermal` solves the tangency and reports the margin to it; GATE-SINEW-05 (tangency within 1%) | `mutant-thermal-slope-estimate`: the tangency solve replaced by the slope formula | BD-072 |
| CX-138 | Episodes that start at ambient temperature test the thermal limits of a design | In 20 s from cold at 20 A the card's winding rises 164.7·(1 − e^(−20/247)) = 12.8 K against an equilibrium rise of 164.7 K. A policy trained and evaluated on such episodes never meets derating | ADR-SINEW-08: thermal state starts from a declared duty history (`cold`, `soaked(T)`, `steady(task)`); evaluation defaults to `steady(task)` | `mutant-thermal-init-cold`: evaluation always starts cold | BD-071 (initialization types); BD-072 (the `steady(task)` default reaches derating inside the episode) |
| CX-139 | A torque-speed envelope without the voltage ellipse is good enough above base speed | λ = 0.010 V·s, L = 100 µH, I_max = 40 A, V_bus = 48 V: base speed 3,151 rad/s electrical, no-load speed 3,394 rad/s. At 1.5 times base speed the motor delivers 59% of peak torque with field weakening (i_d = −32.2 A, i_q = 23.7 A) and none without it | The dq voltage limit at L2; `actuator.envelope`; GATE-SINEW-06; the L1 flag when L_q·I_max exceeds one tenth of the flux linkage | `mutant-envelope-no-ellipse`: i_d forced to zero | BD-073 |
| CX-140 | Reflected rotor inertia is diagonal in joint space | Symmetric differential, q₁ = (θ₁ + θ₂)/2, q₂ = (θ₁ − θ₂)/2: the reflected inertia T⁻ᵀ·diag(J_r)·T⁻¹ is [[J₁ + J₂, J₁ − J₂], [J₁ − J₂, J₁ + J₂]]. Unequal rotors couple the joints | Coupling transmissions as a matrix transformer | `mutant-reflect-diagonal`: off-diagonal entries dropped | BD-074; N15b-11 |
| CX-141 | A bus under load can always deliver the commanded power at a lower voltage; bus feasibility is a current limit | Current-controlled actuators draw power. E = 48 V, R = 0.08 Ω: at most E²/(4·R) = 7,200 W; at 1,800 W the bus sits at 44.8 V and 40.2 A. Above the ceiling there is no operating point: the voltage collapses, the step records a brownout and never completes at the requested power | FROB-REF-133 at stage 6; the brownout event; GATE-SINEW-07 (100 of 100 trials); INV-SINEW-04 and FROB-REF-135 for exhaustion, which no kernel clips | `mutant-bus-clip-demand`: demand clipped to the ceiling and the step completed; `mutant-battery-clip-soc`: state of charge clamped at zero while power is still supplied | BD-075; N15b-03; BD-151 |
| CX-142 | Losses that flow into thermal nodes are also energy that left the system | Copper, iron, friction and gear losses are stored in thermal nodes; what leaves is mechanical work at the joint and heat to ambient. Counting stored heat again as a loss produces a false defect equal to that heat | The per-step identity of the port account; INV-SINEW-02; GATE-SINEW-02 | `mutant-ledger-double-heat`: heat entering a node booked as stored and as lost | N15b-05; BD-076 |
| CX-143 | Agreement between the referee and the fast tiers is evidence that the card is right | Every tier reads the same card. If the card is wrong, the referee, the CPU batch tier and both GPU tiers are wrong together | Randomization over the card's stated uncertainty; the `card-conditional` label; refusal under `--strict-data`; measurement as the only evidence about the card | `mutant-agreement-raises-class`: a cross-tier agreement raises the evidence class of a card-derived result | N15b-06; N15b-07 |

**Four shortcuts of coupled transmissions with no catalogue number (to be assigned; recorded here as
CX-SINEW-a to -d).** The fixture is one base-mounted rotor with θ = 10·(q₁ + q₂), torque limit 0.1 N·m and
inertia 0.01 kg·m².

| Provisional | Shortcut | Case | Mutant | Caught by |
|---|---|---|---|---|
| CX-SINEW-a | The feasible joint torques are the box of per-joint limits | The true set is (t, t) with abs(t) ≤ 1 N·m; the box admits (1, −1) N·m, which no motor torque produces | `mutant-torque-box` | N15b-10 |
| CX-SINEW-b | A shared rotor's inertia reflects diagonally | True reflected inertia [[1, 1], [1, 1]] kg·m²; rotor energy 0 J at rates (1, −1) rad/s and 2 J at (1, 1) rad/s; the diagonal form reports 1 J twice | `mutant-rotor-diagonal` | N15b-11 |
| CX-SINEW-c | A power balance detects a reversed gear | Two unit scalar stores, efforts (2, 3), coefficient 4: one sign convention gives rates (12, −8), the other (−12, 8); both satisfy 2·12 + 3·(−8) = 0 | `mutant-ratio-negated` | N15b-12 (signed motion witness) |
| CX-SINEW-d | A linkage transmission can be modelled by a constant ratio | A coupling that varies with configuration contributes velocity-product terms from ½·q̇ᵀ·Cᵀ·J_r·C·q̇; the constant-ratio shortcut omits them | `mutant-coupling-frozen` | N15b-11 |

**Related rows owned by other chapters, cited by Sinew tests:** CX-128 (a contract that carries only peak
demands can admit and cannot exclude: two fingers on a one-unit bus), CX-172 (backlash as a dead zone on the
torque command produces no impact at gap closure), CX-303 (adding peak currents on a shared bus).

## Acceptance criteria

- [ ] `registries/negative-controls/sinew.toml` lists the nine numbered rows and the four provisional rows,
  each with feature name, paired test, owning bead and expected first failing assertion.
- [ ] For every row whose owning bead is closed the harness records `caught`. A row whose owner is still
  open is `pending` with the bead key (for example CX-143's label test with N15b-07 at M3). The conformance
  sweep of each milestone fails on a `missed` row and on a `pending` row whose owner is closed; this bead
  closes on the registry, the plumbing and that rule.
- [ ] No mutant feature is reachable from the release binaries (`cargo tree -e features` on `frob-cli` shows
  none).
- [ ] A request is filed with the catalogue owner for numbers for the four provisional rows, and the
  registry is updated when they are assigned.

## Test plan

### Unit tests
`crates/frob-conformance/tests/negative_controls_sinew.rs`:
- `registry_rows_resolve`: every row's feature exists in its crate and its paired test exists.
- `each_mutant_is_caught`: for each row marked ready, runs the paired test with the feature on and asserts failure, then
  with the feature off and asserts success (the control has a healthy baseline).
- `mutants_absent_from_release`: asserts the feature set of the release profile contains no `mutant-*`.

### End-to-end script and logging
`scripts/e2e/sinew_negative_controls.sh` iterates the registry's ready rows, runs `cargo test -p <crate> --features
mutant-<name> -- <paired test>` and expects a non-zero exit, then runs the same test without the feature and
expects zero. It logs one NDJSON line per control to `target/e2e-logs/sinew_negative_controls/<utc>.ndjson`
(`ts`, `script`, `bead`, `step` = the CX number, `event`, command, exit code, wall time, the first failing
assertion with expected and observed values) and a `summary` line with caught and missed counts.

### Known answers and negative controls
The numbers in the table are the known answers KA-320 (CX-135), KA-305 to KA-309 and KA-313 (CX-136), KA-310
(CX-137), KA-312 (CX-138), KA-315 to KA-318 (CX-139), KA-322 (CX-140), KA-324 to KA-326 (CX-141), KA-295
(CX-142). The control of this bead itself: a registry row whose paired test does not depend on the mutant
(a deliberately mis-paired row in a test copy of the registry) must be reported `missed`.

## Considerations

- A mutant in production arithmetic is a hazard if it can ship; the seam must be test-only and is audited by
  the release check above.
- Tolerances marked "to set" in the known-answer registry do not weaken a control: each control asserts a
  difference far above any tolerance (2.5 W created; 1 J against 0 J; a refusal against an admission).
- The provisional identifiers are temporary; nothing else may cite them once numbers exist.

## References

Plan Appendix N.12; §15.7 to §15.12, §15.15; §0.7 (counterexamples are first-class); §38 (negative-control
registry). ADR-SINEW-04, ADR-SINEW-08; INV-SINEW-01, -02, -04.
