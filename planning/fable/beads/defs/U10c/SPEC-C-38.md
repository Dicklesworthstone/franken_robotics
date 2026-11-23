---
key: "SPEC-C-38"
title: "spec C-38: actuators as port-Hamiltonian components with a thermal node: ledger, runaway, entropy sign"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "solid", "sinew", "frob-actuate", "test"]
parent: "WP-09"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-068", "BD-070", "BD-071", "BD-072", "BD-074", "BD-076", "BD-092", "SPEC-C-31", "N10-CX", "N10c-10", "BD-069", "N10a-02"]
gates: ["GATE-SINEW-02", "GATE-SINEW-05", "GATE-SINEW-08"]
plan_refs: ["§10 C-38", "§10 C-12", "§15.4", "§15.5", "§15.7", "§29.9", "Appendix D.4", "Appendix E GB-11", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-38 of the FrankenRobotics spine models every actuator element by its energy ports: a component
with stored energy obeys a port-Hamiltonian equation, so stored energy changes only by port power minus
dissipation, and a thermal node turns dissipated electrical and mechanical power into stored heat with a
temperature. Tier Solid [S]. Owner: the Sinew subsystem (actuators, transmissions, power), crate `frob-actuate`;
the run-time reading of the same structure is the energy ledger of the Chronicle subsystem.

What it replaces: a torque limit and an efficiency number. Payoff: the energy ledger closes per mechanism, per
episode and at the earliest offending event, so a wrong gear ratio or frame transform shows as energy appearing
from nowhere even when the final return looks excellent; and a thermal cliff becomes a predicted failure with a
critical current instead of a surprise.

Design reasoning. Resistance that rises with temperature is positive feedback: it lengthens the thermal time
constant, lowers the continuous rating, and above a finite current removes the equilibrium. A constant-resistance
model misses all three (the worked card below). The entropy balance is derived from the energy balance and is not
a second model: temperature is the state and entropy is a reported quantity. The second-law check therefore has
content only as a check on the sign of each assigned loss: it catches a negative loss from a sign error in a
braking mode, and it cannot fail for a correct model, so a passing check is not validation. Rejected shortcut:
booking heat that enters the winding as a loss and also counting the node's temperature rise as stored energy,
which leaves a ledger deficit equal to that heat. FACT (FrankenSim `483d09c682f2`, observed 2026-11-21): the
siblings have a port-Hamiltonian interconnection routine, a ports contract, motor current and speed loops and a
thermal library; interconnections with algebraic constraints are deferred there.

Terms. x: the energy state; H(x): stored energy; u, y: port inputs and outputs whose product is power; J: a skew
interconnection matrix; R: a positive semidefinite dissipation matrix; G: the port map. For a motor axis: φ = L·i
flux linkage, p = J_r·ω rotor momentum, k the torque and back-EMF constant (k_t = k_e = k in SI units), R_w winding
resistance, b viscous friction, τ_load the load torque. For the thermal node: T_w winding temperature, C heat
capacity, G_th conductance to ambient at T_amb, R₀ the resistance at reference temperature T₀, α its temperature
coefficient. Proof classes: Proved, Certified, Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-38.md`; law tests in `crates/frob-actuate/tests/laws_c38.rs`; registry entries for KA-183 to
KA-186 and the controls seeded from CX-090 and CX-091.

Out: actuator cards and the catalogue (BD-069); actuator levels on the referee and winding current with the
thermal network and battery (BD-070, BD-071); the `actuator.thermal` analysis and GATE-SINEW-05 (BD-072);
transmissions with loss laws (BD-074); the port energy-balance check and GATE-SINEW-02 (BD-076); the energy
ledger (BD-092); admission of the sibling port and material libraries (BD-068).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-38: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`actuator.thermal`; also `actuator.check`,
`telemetry.energy`), `failing_test` (a test of `frob-actuate::laws_c38` with its registered control),
`payoff.gate` (GATE-SINEW-02), `statements`, `known_answers` (KA-183 to KA-186), `cards` (none), `gates`, `pilot`
left empty (the construction is Solid) and `generic_comparator` only if this bead claims an advantage over a named
generic method. Doctrine D-2 (no ornamental mathematics) is checked on these fields: a construction with no verb
or no failing test is moved to the parked register by a decision record. The contract `docs/math/C-38.md` has the
ten sections of the template `docs/math/TEMPLATE.md` of the same bead, in the template's order (object and
definitions; what it replaces and the payoff; conventions; statements; formulas; fixtures and known answers;
failure cases, refusals and retirement; non-claims; foundations; implementation map); content that this bead lists
under other headings goes into the template section that matches. Statement identifiers are entered in the
contract and in the register with the construction prefix (`C-38.S1` and so on, as the template requires); the
tables of this bead and of its part beads abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-069 (actuator cards, the
first seed in `frob-actuate`): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-actuate` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements

| Id | Statement | Class |
|---|---|---|
| P1 | Port-Hamiltonian form: ẋ = (J(x) − R(x))·∇H(x) + G(x)·u and y = G(x)ᵀ·∇H(x), with J skew and R positive semidefinite, give Ḣ = uᵀ·y − ∇Hᵀ·R·∇H: stored energy changes by port power minus dissipation. | Tested exactly (an algebraic identity) |
| P2 | Motor axis: with states φ = L·i and p = J_r·ω, energy H = φ²/(2·L) + p²/(2·J_r), a gyrator of modulus k between them and dissipation R_w·i² + b·ω², the equations are L·di/dt = V − R_w·i − k·ω and J_r·ω̇ = k·i − b·ω − τ_load. In the form of P1: x = (φ, p), ∇H = (i, ω), J = [[0, −k], [k, 0]], R = diag(R_w, b), u = (V, −τ_load), y = (i, ω). | Tested exactly |
| P3 | A gear is a transformer (it conserves power); a driver adds a current limit and a voltage ceiling; a battery adds an open-circuit voltage, an internal resistance and polarization states. | Structural; the transformer identity is Tested |
| P4 | Three electrical modes are distinguished: motoring, regenerative braking and dissipative braking, and the driver card says whether recovered energy reaches the battery, heats a shunt or is clipped. In every mode: supplied = stored change + dissipated + exported. | Tested (ledger closure) |
| P5 | Thermal node: C·Ṫ_w = P_loss − G_th·(T_w − T_amb), where P_loss is the dissipation assigned to the node and includes the copper loss i²·R₀·(1 + α·(T_w − T₀)). | Definition of the model |
| P6 | Heat held in the node is stored energy, U = C·T_w, and H + U changes only by port power and by the heat G_th·(T_w − T_amb) that leaves to ambient. | Tested (ledger closure with the node) |
| P7 | Entropy: from dU = T_w·dS the node gains Ṡ = P_loss/T_w − G_th·(T_w − T_amb)/T_w, the ambient gains G_th·(T_w − T_amb)/T_amb, and the total production is σ = P_loss/T_w + G_th·(T_w − T_amb)²/(T_w·T_amb) ≥ 0 whenever P_loss ≥ 0. | Tested exactly (identity and sign) |
| P8 | At constant current the node is linear, Ṫ_w = a·T_w + c with a = (i²·R₀·α − G_th)/C. For a < 0 it settles at T_amb + i²·R₀·(1 + α·(T_amb − T₀))/(G_th − i²·R₀·α) with time constant C/(G_th − i²·R₀·α); for a > 0, which is i above I_crit = √(G_th/(R₀·α)), the temperature runs away exponentially. | Tested against closed forms |

Closed forms that follow from P2 and are used by the owning gate: locked-rotor current (V/R_w)·(1 − e^(−t/τ)) with
τ = L/R_w; stall torque k·V/R_w; no-load speed V/k when b = 0; mechanical time constant J_r·R_w/k² when L is
neglected.

### 2. Worked cards (hand-checkable)

- Electrical step: R_w = 1 Ω, L = 10 mH: τ = 10 ms. With V = 12 V the current is 7.585 A at 10 ms and tends to 12 A.
- Runaway fixture of the construction: R₀ = 0.1 Ω, α = 0.0039 K⁻¹, G_th = 0.5 W/K: I_crit = √(0.5/0.00039) ≈ 35.8 A.
  The construction leaves the heat capacity and the temperatures to the gate; completed here by DESIGN with
  C = 100 J/K, T_amb = T₀ = 25 °C and a limit of 155 °C: at 10 A the rise settles at 21.69 K with time constant
  216.9 s; at 30 A the equilibrium rise would be 604.0 K with time constant 671.1 s and the limit is reached in
  162.7 s; at 40 A there is no equilibrium and the limit is reached in 77.4 s.
- Illustrative card of the owning chapter (a second fixture, kept under its own identity): R₀ = 0.20 Ω,
  α = 0.00393 K⁻¹, G = 0.80 W/K, C = 120 J/K, ambient and T₀ at 25 °C, limit 155 °C. At 20 A: 80 W cold, rise
  164.7 K (190 °C) where the constant-resistance model says 100 K (125 °C); time constant 247 s against 150 s;
  the limit is reached in 385 s. Largest continuous current 18.6 A, not 22.8 A. I_crit = 31.9 A. At 35 A the limit
  is reached in 61 s (83 s by the constant model).

### 3. Conventions, failure cases and non-claims

- Sign convention: power into a component is positive; the motor's shaft port exports k·i·ω − b·ω² − (stored
  change) as τ_load·ω.
- Lumped thermal parameters are card values, Estimated unless calibrated. One lumped node; a housing node in
  series lowers every threshold and is the mechanism's extension, not this contract's.
- Under torque control (constant torque, not constant current) the loss curve is convex and the equilibrium
  vanishes at a tangency below the slope estimate; that case is owned by the Sinew chapter's gate.
- Not claimed: that a passing second-law check validates the thermal model; anything about algebraic-constraint
  interconnections.

### 4. Law-test layout, registry, implementers and gates

`crates/frob-actuate/tests/laws_c38.rs` with `tests/support/c38_ref.rs`: the identity of P1 in exact rationals;
the motor axis integrated by a fine fixed step with an exact exponential for the linear parts; the thermal node's
closed forms; a three-mode ledger (supplied, stored in H and U, heat to ambient, shaft work). Generic over a trait
`C38Subject` so the production actuator levels bind to the same laws. Mutants in `tests/support/c38_mutants.rs`.
Registry: KA-183 to KA-186 with derivation files stating tolerances; controls from CX-090 and CX-091.

Implementers: `frob-actuate` (BD-070, BD-071, BD-072, BD-076), `frob-chronicle` (BD-092). Gates: GATE-SINEW-02
(port energy balance closes to 10⁻⁹ relative on the referee at M1 and 10⁻⁴ relative per episode on single-precision
tiers at M2); GATE-SINEW-05 at M1 (equilibrium, time constant, critical current and time to limit within 0.5% of
the closed forms; the worked card's 20 A point admitted by the constant-resistance check and refused by the
corrected one); GATE-SINEW-08 at M1 (over 10⁶ random operating points in all four quadrants every loss and every
entropy production is nonnegative). The motor constants are also rows of GATE-SINEW-01 (within 1%). This
construction has no pilot and no kill gate.

## Acceptance criteria

- [ ] `docs/math/C-38.md` exists with P1 to P8, their derivations, the closed forms, the three worked cards (the
      DESIGN completion of the construction's fixture marked as such), the conventions and the non-claims.
- [ ] `cargo test -p frob-actuate --test laws_c38` passes on the reference model, and on the production actuator
      levels that exist (BD-070, BD-071).
- [ ] Every mutant below fails its named law.
- [ ] KA-183 to KA-186 are registered with derivation files and stated tolerances; KA-183 and KA-185, printed with
      no control, have the controls named below; controls from CX-090 and CX-091 are attached to GATE-SINEW-02 and
      GATE-SINEW-08.
- [ ] `scripts/e2e/spec_c38.sh` exits 0.
- [ ] Row C-38 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `phs_power_balance_identity`: 10³ seeded rational instances with skew J, positive semidefinite R and quadratic
  H: Ḣ = uᵀ·y − ∇Hᵀ·R·∇H exactly.
- `motor_axis_is_phs`: the matrices of P2 reproduce both motor equations; the gyrator terms cancel in the power.
- `ka_183_locked_rotor_step`: R_w = 1 Ω, L = 10 mH, V = 12 V, rotor locked: 7.585 A at 10 ms (63.21% of 12 A); the
  integrated curve matches the exponential to 10⁻⁶ relative.
- `steady_state_closed_forms`: V = 12 V, R_w = 1 Ω, k = 0.05 N·m/A, J_r = 10⁻⁴ kg·m², b = 0: stall torque 0.6 N·m,
  no-load speed 240 rad/s, mechanical time constant 0.04 s.
- `ka_184_ledger_three_modes`: one run each of motoring, regenerative braking and dissipative braking, with the
  thermal node on: supplied = change in (H + U) + heat to ambient + shaft work, to 10⁻⁹ relative.
- `heat_is_stored_not_lost`: with all dissipation assigned to the node, d(H + U)/dt = V·i − τ_load·ω −
  G_th·(T_w − T_amb) at every step.
- `ka_185_runaway`: I_crit = 35.806 A; at 10, 30 and 40 A the simulated temperature follows the closed forms and
  reaches the values of the worked card (21.69 K and 216.9 s; 162.7 s to the limit; 77.4 s to the limit) to 0.5%.
- `illustrative_card`: the eight numbers of the second card to the printed digits; the constant-resistance model
  admits 20 A (125 °C) and the corrected model refuses it (190 °C).
- `ka_186_entropy_identity_and_sign`: 10⁴ seeded points with P_loss ≥ 0: σ equals node plus ambient entropy rates
  exactly and is nonnegative.
- `negated_copper_loss_rejected`: a regenerative mode with P_loss = −i²·R at T_w = T_amb gives σ < 0 and is
  rejected.
- `transformer_conserves_power`: a gear of ratio r with f₂ = r·f₁ and e₁ = −r·e₂ gives e₁·f₁ + e₂·f₂ = 0.

### End-to-end script and logging

`scripts/e2e/spec_c38.sh`: `frob --json doctor`; `cargo test -p frob-actuate --test laws_c38`; `cargo test -p
frob-conformance --test g0 -- KA-186`; `cargo test -p frob-conformance --test g1 -- KA-183 KA-184 KA-185`; file
and registry checks. Log to `target/e2e-logs/spec_c38/<utc>.ndjson` with a transcript; each line has `ts`,
`script`, `bead`, `step`, `event`, command line, exit code, wall time, the evidence block (claim Known answer,
class Tested, tier, determinism D0 for rational rows and the host class for `f64` rows, seed), measured currents,
temperatures, times and ledger residuals with units against expected and thresholds, and on failure the first
step at which the ledger residual exceeds its bound. The `summary` line counts assertions and names
GATE-SINEW-02, GATE-SINEW-05 and GATE-SINEW-08.

### Known answers and negative controls

| Row | Truth | Tolerance (stated here where printed `to set`) | Tier | Control |
|---|---|---|---|---|
| KA-183 | Locked rotor, R_w = 1 Ω, L = 10 mH: current (V/R_w)·(1 − e^(−t/τ)), τ = 10 ms | 1% (the tolerance of GATE-SINEW-01) | G1 referee | Mutant `back_emf_sign_flipped` (new): the gyrator is not skew and a free rotor accelerates without bound |
| KA-184 | Supplied = stored change + dissipated + exported in motoring, regenerative braking and dissipative braking | 10⁻⁹ relative on the referee | G1 referee | Mutant `heat_double_counted`: a deficit equal to the heat, flagged |
| KA-185 | R₀ = 0.1 Ω, α = 0.0039 K⁻¹, G_th = 0.5 W/K: I_crit ≈ 35.8 A; closed-form approach below it, growth above it | 0.5% (the tolerance of GATE-SINEW-05) | G1 referee | Mutant `constant_resistance` (new): no runaway at 40 A |
| KA-186 | σ = P_loss/T_w + G_th·(T_w − T_amb)²/(T_w·T_amb) ≥ 0 whenever P_loss ≥ 0 | exact (sign) | G0 referee (host for the identity) | A regenerative mode with a negated copper loss gives σ < 0 and is rejected |

Counterexamples carried: CX-090 (heat entering the winding is not both a loss and stored energy), CX-091 (a
passing second-law check does not show the thermal model is right). Determinism: rational rows exact; `f64` rows
twice on one machine; debug and release.

## Considerations

- The construction's runaway fixture has no heat capacity or temperatures; the completion above is a DESIGN choice
  of this bead and the derivation file says so. The registry keeps the construction's fixture and the owning
  chapter's illustrative card as separate identities (their critical currents are 35.8 A and 31.9 A).
- A 20 s episode from cold warms the illustrative winding by about 13 K and meets none of this; thermal failures
  appear only in long runs or under the analysis verb, which is why the analysis exists.
- The energy terms of the safety budget (construction C-31) are those of this ledger.
- The second-law check must never be reported as a validation of the thermal model.

## References

Plan §10 C-38, C-12; §15.4, §15.5, §15.7; §29.9; Appendix D.4 "Port-Hamiltonian form", "Motor axis", "Thermal
node", "Thermal runaway condition", "Entropy production", "Energy balance"; GB-11; KA-183 to KA-186; CX-090,
CX-091; GATE-SINEW-01, -02, -05, -08.
