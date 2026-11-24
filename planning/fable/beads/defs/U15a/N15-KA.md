---
key: "N15-KA"
title: "Sinew known-answer registry: KA-293 to KA-337 with truths, tolerances, tiers, controls and owner beads"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sinew", "frob-actuate", "frob-conformance", "solid", "registry", "test"]
parent: "WP-09"
deps: ["BD-025", "N38a-12", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N15a-12", "N15a-14", "N15a-16", "N15a-18", "N15a-23", "N15a-24", "N15b-03", "N15b-08", "N15b-10", "N15b-11", "N15b-12", "N15b-13", "N15b-15", "N15b-21", "N15b-22", "N15b-30", "N15b-32", "N15b-35", "N15b-36", "N15b-37", "N38-KA", "N38a-13", "N38b-27", "N38b-31"]
gates: ["GATE-SINEW-01", "GATE-SINEW-02", "GATE-SINEW-03", "GATE-SINEW-04", "GATE-SINEW-05", "GATE-SINEW-06", "GATE-SINEW-07", "GATE-SINEW-08", "GATE-SINEW-09", "GATE-SINEW-10"]
plan_refs: ["Appendix M M.14", "§15.5", "§15.6", "§15.7", "§15.8", "§15.9", "§15.10", "§15.13", "§15.14", "§15.15", "§38.6"]
---
## Background

A known answer is a case with a truth a test can compare against: an analytic formula or a hand-derived value, the
tolerance of the comparison, the tier and site on which it runs, and the negative control that must make it fail.
The known-answer registry of `frob-conformance` is the authority for all of them; numbers are append-only (a row
is never renumbered and a number is never reused). This bead enters the 45 rows of the Sinew chapter (actuators,
transmissions, power), KA-293 to KA-337, so that no hand-checkable truth of the chapter can be lost between the
mechanism beads, and it names for each row the bead whose tests carry it.

Reading the columns. *Tolerance:* `exact` for integers, rationals, verdicts and identities; a stated value with the
gate that states it; `to set` where the chapter states none (the owning bead sets it at first measurement and
records it). *Tier:* the verification tier and the execution site. G0 laws and self-tests; G1 known answers; G2
parity with a pinned oracle; G3 metamorphic relations; G4 chaos (faults, cancellation, overflow); G5 determinism.
Sites: `host` (exact or double-precision arithmetic in a test), `referee`, `lockstep` (throughput tiers), `all
tiers`. *Negative control:* the mutation or planted failure the row must catch; `none stated` means the chapter
states none, and the owning bead adds one (a gate with no control cannot record a pass).

Labels. Fixture constants are DESIGN choices and illustrative; truths derived from them by hand are DESIGN
arithmetic with a derivation file under `docs/derivations/`. Thresholds are TARGETs of the gate named. Nothing here
is a measurement: no row has been run on FrankenRobotics code.

## Scope

In: the 45 registry records; one derivation file per fixture, with every hand-derived truth; the map from row to
owning bead; the `control.text` of every row whose source or owning bead states a negative control, from which
the seeding tool of N38b-05 writes the records of `crates/frob-conformance/registry/controls.toml` with the row's
number as `seed` and fills `control.nc` (this bead writes no row of `controls.toml`); the block request and the
two tagged rows of the late subsection §15.15 (item 6). Out: the tests themselves (owned by
the beads in the last column); the counterexample catalogue and the refusal-code registry of the chapter (written
by the other Sinew drafting unit as registries of their own); rows of other chapters that reuse a Sinew fixture
(KA-181 to KA-186 of constructions C-37 and C-38, KA-279, KA-280 and KA-285 of the design chapter's rejection
fixtures), which are cited by Sinew beads and registered by their own chapters.

## Technical approach

1. **Record format.** One `[[ka]]` record per row in the suite's one registry file
   `crates/frob-conformance/registry/known_answers.toml`, in the row schema N38a-12 owns, field for field: `id`;
   `section` (the subsection of §15 that states the row), with `owner` = the gate named in the tolerance cell
   (GATE-SINEW-nn), or the `PO-SINEW-nn` or `INV-SINEW-nn` the case names; `fixture` (`sinew-<case>`; rows on one
   worked card share it) and `same_as` (the earlier row of the same fixture where the case restates one, else
   empty); `case`; `truth`; `tolerance` (`exact`, `abs`, `rel`, `printed-digits`, `gate` or `to-set`); `tier` and
   `sites` (the "Tier · site" column; "all tiers" is written as the full list); `control` (`text` as printed, or
   the owning bead's text marked "(set: <bead>)", empty for `none stated`; its `nc` list is filled by N38b-05's
   seeding); `derivation` (`docs/derivations/<fixture>.md`, whose front matter lists the KA ids it serves and, for
   each, the beads of the "Owner" column with their tests); `label` (DESIGN arithmetic, or FACT for KA-337, whose
   source is its pilot record); `due` (the milestone of the row's gate); `printed` (empty unless a recomputation
   disagrees with a printed value); `slug` (empty, except the tags of item 6). The "Owner" column is not a field.
   No record stores a status: N38a-12's loader computes eligibility (`obligation: tolerance` for `to-set`,
   `obligation: control` for an empty control) and the runner (N38a-13) reports each row per site. Values are
   carried as printed and are not improved in transit. DESIGN, correcting an earlier draft that used a format of
   its own (`where`, `site`, `negative_control`, `gate`, `owner_bead`, a stored `status`), which N38a-12's parser
   refuses as unknown fields.
2. **The rows.**

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Owner |
|---|---|---|---|---|---|---|
| 293 | Brushed or q-axis motor in port-Hamiltonian form: x = (φ, p), H = φ²/(2L) + p²/(2J_r), J = [[0, −k], [k, 0]], R = diag(R_w, b), u = (v, −τ_load) | L·di/dt = v − R_w·i − k·ω; J_r·dω/dt = k·i − b·ω − τ_load; Ḣ = v·i − τ_load·ω − R_w·i² − b·ω² | exact (polynomial identity) | G0 · host | none stated | N15a-05 |
| 294 | Passivity and second law over 10⁶ random operating points in all four quadrants (INV-SINEW-01, -03) | Every loss nonnegative; entropy production P_loss/T_w + G·(T_w − T_amb)²/(T_w·T_amb) nonnegative each step; every catalog card passes `actuator.check` | exact (sign); GATE-SINEW-08 | G0 · all tiers | The single-multiplier gear fixture, which must refuse | N15a-23, N15b-32, BD-069 |
| 295 | Port energy balance of an actuator: electrical energy in against mechanical work out, copper, friction and gear loss and the change in stored magnetic and thermal energy | The balance closes each step to a recorded residual | 10⁻⁹ relative on the referee; 10⁻⁴ relative per episode on single-precision tiers (GATE-SINEW-02) | G1 · all tiers | Stored heat also counted as a loss, which produces a false defect | BD-076, N15a-07 |
| 296 | Batched L1 and L2 kernels against the referee components on a 10⁴-case battery | Agreement | 10⁻⁴ relative (GATE-SINEW-03) | G2 · lockstep | none stated | N15a-07, BD-151, N15b-35 |
| 297 | A torque ramp r·t on a joint at rest with breakaway τ_s | Motion begins at t = τ_s/r | within one step (GATE-SINEW-04) | G1 · all tiers (referee exact) | none stated | N15a-12 |
| 298 | A mass on a spring of stiffness k with Coulomb friction τ_c, released from rest | Amplitude falls by 4·τ_c/k per cycle; motion ends inside the band of half-width τ_s/k | loss per cycle within 2% (GATE-SINEW-04) | G1 · all tiers (referee exact) | none stated | N15a-12 |
| 299 | A held load τ below the stiction bound on a convex tier | Creep at ε·τ/τ_b; drift at most ε·t over a time t | bound (GATE-SINEW-04) | G1 · lockstep | none stated | N15a-08, N15a-12 |
| 300 | A motor reversal smaller than the backlash gap | Zero output motion | exact on the referee; batched tiers within their stated regularization (GATE-SINEW-04) | G1 · all tiers (referee exact) | Backlash modelled as a dead zone on the torque command | N15a-09, N15a-12 |
| 301 | A slow sinusoid through a gap β | An input–output loop of width β | within 1% (GATE-SINEW-04) | G1 · all tiers (referee exact) | Backlash modelled as a dead zone on the torque command | N15a-12 |
| 302 | A rotor of reflected inertia J₁ = N²·J_r closing the gap at relative speed Δω onto a link of inertia J₂ at rest, no restitution | Common speed J₁·Δω/(J₁ + J₂); energy lost ½·J₁·J₂/(J₁ + J₂)·Δω². With N = 9, J_r = 10⁻⁴ kg·m², J₂ = 0.02 kg·m², Δω = 2 rad/s (illustrative): 0.577 rad/s and 0.0115 J | within 1% (GATE-SINEW-04) | G1 · all tiers (referee exact) | Backlash as a dead zone, which produces no impact | N15a-09, N15a-12 |
| 303 | Rotor elimination with twelve backlash joints on a quadruped (PO-SINEW-02) | The eliminated solve equals the full solve; the dense factorization has the size it has with none (18 velocities, not 30) | exact rational on small cases; round-off on 10⁴ random cases, to set (GATE-SINEW-09) | G1 · host | none stated | N15b-08, N15b-37 |
| 304 | Winding with one thermal node at constant current, s = I²·R₀·α below G | Stable equilibrium T = T_amb + P_amb/(G − s), time constant C/(G − s); both exceed the constant-resistance values P_amb/G and C/G by 1/(1 − s/G) | within 0.5% (GATE-SINEW-05) | G1 · all tiers | The constant-resistance model | N15a-14, BD-072 |
| 305 | Worked card (R₀ = 0.20 Ω at 25 °C, α = 0.00393 K⁻¹, G = 0.80 W/K, C = 120 J/K, ambient 25 °C, limit 155 °C) at a constant 20 A | 80 W when cold; s = 0.314 W/K; rise 80/(0.80 − 0.314) = 164.7 K, so 190 °C, outside the limit; loss at equilibrium 132 W; time constant 247 s. Constant resistance: 100 K, 125 °C, 150 s | within 0.5% (GATE-SINEW-05) | G1 · all tiers | The constant-resistance model, which puts the point inside the limit | N15a-14, BD-072 |
| 306 | Same card: time from cold to the 155 °C limit at 20 A | 385 s (constant resistance: never) | within 0.5% (GATE-SINEW-05) | G1 · all tiers | The constant-resistance model | N15a-14, BD-072 |
| 307 | Same card: largest continuous current for the limit | Constant resistance √(0.80·130/0.20) = 22.8 A; temperature-dependent 22.8/√(1 + 0.00393·130) = 18.6 A, 19% lower | within 0.5% (GATE-SINEW-05) | G1 · host | The constant-resistance model | N15a-14 |
| 308 | Thermal runaway threshold | No equilibrium for s ≥ G; I_crit = √(G/(R₀·α)) = 31.9 A for the card | within 0.5% (GATE-SINEW-05) | G1 · host | The constant-resistance model, which has no such current | N15a-14 |
| 309 | Same card at 35 A | No equilibrium; growth rate (s − G)/C = 0.00136 s⁻¹; limit reached at 61 s. Constant resistance: equilibrium 306 K above ambient, limit at 83 s | within 0.5% (GATE-SINEW-05) | G1 · all tiers | The constant-resistance model | N15a-14, BD-072 |
| 310 | Holding torque with flux coefficient α_B = −0.0011 K⁻¹: P(T) = I₀²·R₀·[1 + α·(T − T₀)]/[1 + α_B·(T − T₀)]² | Limit reached at 15.9 A cold-equivalent, 30% below 22.8 A; no equilibrium above 16.5 A (the tangency), where the slope estimate √(G/(R₀·(α − 2·α_B))) says 25.5 A | tangency within 1% (GATE-SINEW-05) | G1 · host | The slope estimate at T₀ used in place of the tangency | N15a-14 |
| 311 | Winding and housing in series (two-node network) | A stable equilibrium exists iff s < G_wh·G_ha/(G_wh + G_ha): the conductance matrix with s subtracted from the winding's diagonal entry stays positive definite | exact (condition; GATE-SINEW-05) | G1 · host | none stated | N15a-14 |
| 312 | Same card: 20 s from cold at 20 A | Rise 164.7·(1 − e^(−20/247)) = 12.8 K | to set | G1 · all tiers | An evaluation that starts every episode at ambient and so never meets derating | N15a-15, BD-072 |
| 313 | Static check of a design sized to hold 20 A on the card | Admitted by the constant-resistance check; refused by the corrected one (FROB-REF-132) | exact (verdict) | G1 · host | The constant-resistance check | BD-072, N15a-26 |
| 314 | Voltage limit of a three-phase bridge in power-invariant dq coordinates | V_lim = V_bus/√2: 33.9 V at V_bus = 48 V | within 0.5% (GATE-SINEW-06) | G1 · host | none stated | N15a-02, N15a-16 |
| 315 | Base speed, non-salient lossless case (card: λ = 0.010 V·s, L = 100 µH, I_max = 40 A, V_bus = 48 V) | V_lim/√(λ² + (L·I_max)²) = 3,151 rad/s electrical | within 0.5% (GATE-SINEW-06) | G1 · host | none stated | N15a-16, BD-073 |
| 316 | No-load speed without field weakening, same card | V_lim/λ = 3,394 rad/s | within 0.5% (GATE-SINEW-06) | G1 · host | none stated | N15a-16, BD-073 |
| 317 | Currents on the limit above base speed, same card, at 1.5 times base speed | i_d = [(V_lim/ω_e)² − λ² − (L·I_max)²]/(2·λ·L) = −32.2 A; i_q = √(I_max² − i_d²) = 23.7 A; 59% of peak torque with field weakening and none without it | within 0.5% (GATE-SINEW-06) | G1 · host | A model without the voltage ellipse | N15a-16, BD-073 |
| 318 | Top speed with field weakening, same card | V_lim/(λ − L·I_max) = 5,657 rad/s when λ > L·I_max; otherwise unbounded in this model | within 0.5% (GATE-SINEW-06) | G1 · host | none stated | N15a-16, BD-073 |
| 319 | Inscribed-circle voltage envelope against the exact hexagon over 10⁴ random operating points | The circle envelope never exceeds the hexagon envelope; conservative by at most 1 − √3/2 = 13.4% of voltage at a vertex | exact (inequality; GATE-SINEW-06) | G0 · all tiers | none stated | N15a-16, BD-073 |
| 320 | Back-driven gear, η = 0.8, N = 10, load driving the joint with 10 N·m at 1 rad/s (10 W in) | Four-quadrant law: τ_motor = η_b·τ_joint/N = 0.8 N·m at 10 rad/s, 8 W, 2 W lost | exact | G1 · host | One multiplier in all quadrants: τ_motor = 1.25 N·m, 12.5 W at the motor, 2.5 W from nowhere; the fixture refuses under FROB-REF-130 (GATE-SINEW-08) | N15a-17, N15a-18 |
| 321 | Screw with lead angle a and friction angle φ | η_f = tan a/tan(a + φ); η_b = tan(a − φ)/tan a; locks exactly when φ ≥ a | to set | G1 · host | none stated | N15a-17, N15a-18 |
| 322 | Symmetric differential, q₁ = (θ₁ + θ₂)/2, q₂ = (θ₁ − θ₂)/2, rotor inertias J₁ and J₂ | Reflected inertia T⁻ᵀ·diag(J_r)·T⁻¹ = [[J₁ + J₂, J₁ − J₂], [J₁ − J₂, J₁ + J₂]]: equal rotors decouple, unequal ones do not; q̇ = T·θ̇ with τ_actuator = Tᵀ·τ_joint conserves power | exact | G1 · host | Reflected rotor inertia taken as diagonal in joint space | N15a-18, BD-074 |
| 323 | Ideal gear law (PO-SINEW-03) | f₂ = r·f₁ and e₁ = −r·e₂ imply e₁·f₁ + e₂·f₂ = 0 | exact (polynomial identity) | G0 · host | none stated | N15a-18 |
| 324 | Bus with source voltage E and series resistance R under a power demand P | Deliverable only if P ≤ E²/(4·R); then V = [E + √(E² − 4·R·P)]/2. With E = 48 V and R = 0.08 Ω: ceiling 7,200 W; at 1,800 W the bus sits at 44.8 V and carries 40.2 A | within 0.1% (GATE-SINEW-07) | G1 · all tiers | A step that completes at the requested power above the ceiling | BD-075, N15b-30 |
| 325 | A demand above the bus ceiling | A brownout event in 100 of 100 trials and never a completed step; stage 6 refuses the design with FROB-REF-133 | exact (counts, verdict; GATE-SINEW-07) | G4 · all tiers | A kernel that clips the demand | BD-075, N15b-30 |
| 326 | Power requested from an exhausted battery (INV-SINEW-04) | Exhaustion is an event; the request refuses (FROB-REF-135); state of charge is never clipped | exact (verdict; GATE-SINEW-07) | G4 · all tiers | An empty battery that keeps supplying commanded power | N15b-03, BD-075, N15b-30 |
| 327 | Current step on the two-RC battery, V = OCV(z) − R₀·I − V₁ − V₂ | An immediate drop I·R₀ followed by two exponentials with time constants R₁·C₁ and R₂·C₂ | drop and both time constants within 1% (GATE-SINEW-07) | G1 · all tiers | none stated | N15b-03, N15b-30 |
| 328 | Constant-current discharge of capacity Q at current I | Discharge time Q/I | to set | G1 · all tiers | none stated | N15b-03, N15b-30 |
| 329 | DC motor stall torque | η·N·k_t·min(I_max, V_lim/R) | within 1% (GATE-SINEW-01) | G1 · all tiers | none stated | BD-070, N15a-24 |
| 330 | DC motor no-load speed | V_lim/(k_e·N) | within 1% (GATE-SINEW-01) | G1 · all tiers | none stated | BD-070, N15a-24 |
| 331 | Electrical time constant of a DC motor step | L/R | within 1% (GATE-SINEW-01) | G1 · all tiers | none stated | BD-071, N15a-24 |
| 332 | Mechanical time constant of a DC motor step | J·R/(k_t·k_e) | within 1% (GATE-SINEW-01) | G1 · all tiers | none stated | BD-070, N15a-24 |
| 333 | Constant-resistance thermal steady state | Rise P/G (ΔT = P·R_th) | within 1% (GATE-SINEW-01) | G1 · all tiers | none stated | BD-071, N15a-06, N15a-24 |
| 334 | Hover of the rotor port (Moonshot) | Thrust equals weight; ideal induced power T^(3/2)/√(2·ρ·A); the energy ledger closes for the hovering body | to set (KILL-SINEW-ROTOR fires if the answer or the port balance fails) | G1 · referee | none stated | N15b-22, N15b-36 |
| 335 | Two-mass resonance of a series-elastic drive (Frontier) | Frequency √(k·(J₁ + J₂)/(J₁·J₂)) | within 1% at the referee step (pilot falsifier) | G1 · referee | none stated | N15b-19, N15b-18 |
| 336 | A single hydraulic cylinder through an orifice (Moonshot) | Steady speed equals its closed form (not written in the chapter; the pilot's derivation file records v = (C_d·A_o/A_p)·√(2·(p_s − F/A_p)/ρ)); the port balance closes | to set (set by the pilot) | G1 · referee | none stated | N15b-21 |
| 337 | Battery ageing law (Frontier) against one published capacity-fade curve, from that publication's own coefficients | The curve is reproduced | error at most 10% of the fade (pilot falsifier) | G2 · host | (set: N15b-15) The same procedure run with the coefficient of the dominant term (the one carrying at least half of the end-of-curve fade) perturbed by 30% in the direction that increases the end-point error (plant `dominant-coefficient-30pct`), which moves the end point by at least 15% of the fade: the pilot must report falsified | N15b-15 |

3. **Controls for rows that state none.** For each `none stated` row the owning bead's test plan names a mutant
   (for example: KA-297, a breakaway level equal to the Coulomb level; KA-315 to KA-318, the circumscribed circle
   or the ellipse dropped; KA-329 to KA-333, an ideal torque source or dropped winding dynamics). This bead
   writes each as the row's `control.text`, marked "(set: <bead>)" with the owning bead named in the derivation
   file, as KA-337's is (set by N15b-15); N38b-05's seeding then writes the record with the row's number as
   `seed` and the row's gate, so that no Sinew gate can record a pass without a control. N38b-31 checks every
   such control against the binding rule and registers its own proposal as a second record where it is a
   different defect; a row whose owner names none is `obligation: control` until N38b-31's record exists.
4. **Two fixtures for one formula.** The chapter's backlash closure (0.577 rad/s, 0.0115 J) and the contact
   chapter's (17.5 ms, 0.083 rad/s), and the chapter's critical current (31.9 A) and construction C-38's (35.8 A,
   G = 0.5 W/K), are each internally consistent; the registry keeps both fixtures under separate identities and
   neither replaces the other.
5. **Derivation files.** One `docs/derivations/<fixture>.md` per fixture (N38a-12's rule 1: a row whose file is
   missing or does not list it is `ineligible: no derivation`). The hand derivations are in: the thermal worked
   card (every figure of rows 304 to 313), the field-weakening card (rows 314 to 318), the inelastic closure (row
   302), the Schur complement of rotor elimination (row 303), the screw efficiencies (row 321), the
   differential's reflected inertia (row 322), the two pilot derivations (rows 335, 336) and the one-rotor table
   (item 6); every other fixture's file states its model, its truth and the error budget of its test.
6. **The late known answer of §15.15** (GATE-SINEW-10, M1). The coupled-transmission subsection, added after the
   catalogue was numbered, states one known answer, the one-rotor table, which Appendix M does not list. This
   bead commits a request `[[block]] registry = "ka", count = 2, owner = "N15-KA", section = "§15.15"` in
   `crates/frob-conformance/registry/allocations.toml` (N38a-40; the allocator fills `first` and `last`, and no
   number is typed here), cites the rows by the tags below until the block is issued, then enters them with the
   issued numbers in table order in one commit, each keeping its tag in its row's `slug` field; a tag still cited
   elsewhere at the M1 exit fails (`provisional-tag-past-due`). DESIGN: two rows on one fixture
   (`one-rotor-coupled`, the fixture of N15b-13, `same_as` empty on both), because the table's torque set and its
   inertia and energies are realized by two mechanism beads and caught by two different controls. Fixture: one
   base-mounted rotor with θ = 10·(q₁ + q₂), torque limit 0.1 N·m and inertia 0.01 kg·m², driving two revolute
   joints.

| Tag | Case | Truth | Tolerance | Tier · site | Negative control | Owner |
|---|---|---|---|---|---|---|
| `sinew-late-01` | Feasible joint-torque set of the one-rotor fixture | Deliverable joint torques (t, t) with abs(t) ≤ 1 N·m: one generator (1, 1) N·m; the command (1, −1) N·m is refused as outside the image (FROB-REF-131, with a separator) | exact (GATE-SINEW-10) | G1 · host | Per-joint limits of 1 N·m in place of the image, which admit (1, −1) N·m, a torque no motor torque produces | N15b-10, N15b-13 |
| `sinew-late-02` | Reflected inertia and rotor energy of the one-rotor fixture | Reflected inertia [[1, 1], [1, 1]] kg·m²; rotor energy 0 J at joint rates (1, −1) rad/s (the rotor does not turn) and 2 J at (1, 1) rad/s | exact (GATE-SINEW-10) | G1 · host | The diagonal mutant: reflected inertia diag(1, 1) kg·m², which reports 1 J at both rates | N15b-11, N15b-13 |

## Acceptance criteria

- [ ] All 45 rows are present in `known_answers.toml` with every field of N38a-12's schema, and N38a-12's loader
      accepts them; no record carries a field outside that schema (no stored status: eligibility is computed by
      the loader, outcomes are reported per site by the runner).
- [ ] Every row's derivation file lists it and names at least one owning bead whose test plan cites it; after
      N38b-05's seeding every row with a control text has a non-empty `control.nc` (this bead wrote no record of
      `controls.toml`), and every row whose owning bead names a mutant carries it marked "(set: <bead>)", KA-337's
      by N15b-15.
- [ ] The block request of item 6 is committed with this bead; the two §15.15 rows are cited by their tags until
      the block is issued and by the issued numbers after, each tag surviving only in its row's `slug`; no tag
      remains elsewhere at the M1 exit.
- [ ] Every hand-derived value recomputes from its derivation file to the printed digits (a host test evaluates
      each formula with the printed inputs).
- [ ] Rows 334, 336 and 337, whose truths or tolerances the chapter leaves open, are marked `to set` with the bead
      that will set them, and are not counted as passable until set.
- [ ] No row number outside 293 to 337 is created or altered by this bead.

## Test plan

### Unit tests
`crates/frob-conformance/tests/ka_sinew.rs` (host; the real registry through N38a-12's loader; planted registries
are scratch copies):
- `ka_rows_recompute`: a host test recomputes each printed value from the printed inputs: 80/(0.80 − 0.3144) =
  164.7; 120/0.4856 = 247.1; −247.1·ln(1 − 130/164.74) = 384.6; √(520/1.5109) = 18.55; √(0.80/0.000786) = 31.90;
  (0.96285 − 0.80)/120 = 0.001357; 33.941/0.0107703 = 3,151.3; 33.941/0.010 = 3,394.1; i_d = −32.22, i_q = 23.70;
  33.941/0.006 = 5,656.9; 0.0081·2/0.0281 = 0.5765; ½·(0.0081·0.02/0.0281)·4 = 0.01153; 48²/(4·0.08) = 7,200;
  [48 + √(2,304 − 576)]/2 = 44.78 and 1,800/44.78 = 40.2; the one-rotor table with C = [10, 10]: generator
  10·0.1 = 1 N·m on each joint, reflected inertia Cᵀ·0.01·C = 0.01·[[100, 100], [100, 100]] = [[1, 1], [1, 1]], energies
  ½·(1, −1)·M·(1, −1)ᵀ = 0 and ½·(1, 1)·M·(1, 1)ᵀ = 2, and the diagonal mutant's 1 and 1.
- `registry_schema_and_append_only`: the records load; numbers are contiguous 293 to 337; a duplicate fails; a
  scratch record carrying `status` or `owner_bead` is refused as an unknown field.
- `owner_beads_exist`: every owning bead named in a derivation file's front matter resolves to a bead key.
- `ka337_control_set_by_pilot`: KA-337's `control.text` names the plant `dominant-coefficient-30pct` and its
  derivation file names N15b-15.
- `sinew_late_rows_tagged`: the two §15.15 rows resolve by tag before the block is issued and to numbers inside
  this bead's block after; in milestone-exit mode a tag cited outside a `slug` fails.
- `every_gate_has_control`: for GATE-SINEW-01 to -09 at least one control record exists.

### End-to-end script and logging
`scripts/e2e/sinew_registry.sh` (header: `bead=N15-KA milestone=M1 labels=sinew,registry gates=GATE-GAUNT-02
plants=wrong-equilibrium,unknown-field,tag-past-due`; sources the STD-E2E helper; logs to
`target/e2e-logs/sinew_registry/<utc>.ndjson` with a transcript beside it): runs `cargo test -p frob-conformance
--test ka_sinew`, N38a-12's loader test and N38a-40's allocation test, then the rows selected through their tier
targets (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <row>`, `<tier>` = `g0_laws`,
`g1_known_answers`, `g2_parity` or `g4_chaos`), logging each row's per-site outcome as the runner reports it (a
row whose owning bead has not landed is `unbound` once due, never passed); it then reads the registry records for
KA-293 to KA-337 and the two §15.15 rows and writes one `assert` line per row (KA number or tag, owner beads,
tolerance or `to set`, tier and site, eligibility, control present yes or no), `metric` lines for the counts (rows 45, required 45; rows
with an owner 45; `to set` rows; controls per gate GATE-SINEW-01 to -09, each required ≥ 1; recomputed values
that match their printed digits, required all), and a final `summary` line. It then plants, on scratch copies under
`target/e2e-scratch/sinew_registry/`, the wrong record of the negative control below, a record carrying a stored
`status`, and a §15.15 tag left in a derivation file in milestone-exit mode, and asserts that each is caught (the
recompute step with the row number and the first differing value logged; the loader's unknown-field refusal;
N38a-40's `provisional-tag-past-due`); exit non-zero on any failed assertion. The rows themselves are
measured by `sinew_cards.sh`, `sinew_static_sizing.sh`, `sinew_referee_dynamics.sh` and `sinew_joint_rows.sh`,
which write, for each assertion that measures a row, a `metric` line carrying the `KA` number, the measured value
with unit, the truth, the tolerance and the tier and site, so that each row's per-site outcome in the runner's
receipt can be traced to the measurement from the logs alone.

### Known answers and negative controls
This bead is the list. Its own negative control is a deliberately wrong record (the worked card's equilibrium
entered as 100 K, the constant-resistance value): `ka_rows_recompute` must reject it.

## Considerations

- Tolerances marked `to set` (rows 303 in part, 312, 321, 328, 334, 336) are an open item of the verification
  program; the owning bead proposes the value at first measurement and it is recorded with the gate.
- Rows 324 to 328 and 337 are registered here because the chapter's registry is one bead; their owners are the
  bus (BD-075), the battery element (N15b-03), the power-path script (N15b-30) and the ageing pilot (N15b-15).
- The coupled-transmission known answer of the late subsection (the one-rotor table of GATE-SINEW-10) is not in
  the numbered block KA-293 to KA-337; it is this bead's two tagged rows of item 6, carried by N15b-10 and
  N15b-11 and measured by N15b-13. N15b-12's signed motion witness, the gate's third clause, is a check over the
  card catalog rather than a known answer and has no row.

## References

Plan Appendix M, block M.14 (KA-293 to KA-337); §15.5 to §15.10, §15.13, §15.14, §15.15 (the one-rotor table);
§38.6. GATE-SINEW-01 to -10.
