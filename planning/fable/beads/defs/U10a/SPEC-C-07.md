---
key: "SPEC-C-07"
title: "frob-analyze: C-07 specified — rolling distributions, growth vectors, the constant-twist chord; law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "atelier", "traction", "frob-analyze", "frob-contact", "solid", "frontier", "spine", "spec", "test"]
parent: "WP-11"
deps: ["BD-033", "BD-025", "N10a-01", "STD-TEST", "STD-E2E"]
needs: ["§28.4: the bead that creates crate frob-analyze (the law suite lands in crates/frob-analyze/tests/; if the crate does not exist when this bead starts, the suite is created with the crate's skeleton)", "§10 C-36: the discrete nonholonomic step and the Chaplygin-sleigh equations used as a trajectory oracle (SPEC-C-36, another unit)"]
related: ["BD-106", "BD-117", "BD-365", "N10a-15", "SPEC-C-08", "N10-KA1"]
gates: ["GATE-ATEL-02", "GATE-TRAC-01"]
plan_refs: ["§10 C-07", "§18.9", "§28.4 A-07", "§33 INT-FSIM-05", "Appendix D.1", "Appendix D.3", "Appendix E GB-05", "Appendix M KA-058 to KA-063", "Appendix N CX-027 to CX-029"]
---
## Background

Construction C-07 turns folklore about which wheeled bases can do what into a computation. Rolling without
slipping is a set of velocity constraints that are linear in the velocities (Pfaffian constraints). Their
kernel is a distribution on configuration space, and iterated Lie brackets of vector fields in that
distribution decide two things: whether the base can reach every nearby configuration at all, and how
expensive each direction is.

Payoff:

- A static maneuverability index for a wheeled base in tight interiors (analysis A-07).
- Bracket motion primitives for planners.
- For kinematic bases the base motion follows from wheel motion with no dynamics at all.
- Ideal rolling is the known-answer oracle for the frictional wheel contact that the simulator runs by
  default.
- A defect such as a mecanum base with misaligned rollers is a rank drop in the constraint distribution, which
  the growth vector reports. The curvature test of the locomotion-as-holonomy construction does not detect it
  and is not used for it.

Tier: Solid for the growth vector; Frontier for the index and the primitives (pilot N10a-15; fallback, the
growth vector and a turning-footprint check).

Foundations (FACT, FrankenSim `483d09c682f2` and FrankenSymPy `29f847c1`, 2026-11-21; existence is not
admission): `fs-mbd::planar_drive` is SE(2) differential-drive kinematics with "no force, traction, slip";
FrankenSim's nonholonomic lane is proposed, not built; FrankenSymPy's `diff` on scalar expressions exists, so
brackets of vector fields can be computed symbolically today. `planar_drive` is reported to have a
near-straight integration defect (FACT to recheck): from heading 32 rad with wheel speeds 1 and
1 + 6·10⁻¹³ m/s on a 0.5 m track, 400 steps of 1/400 s return zero translation where the limit is about
(0.8342, 0.5514) m. That report is an admission test and an upstream work item, and until it is settled the
routine is held out of every oracle role.

## Scope

In: `docs/math/C-07.md`; the law suite `crates/frob-analyze/tests/laws_c07.rs` with an exact reference for
brackets of polynomial vector fields; growth-vector fixtures; the independent chord formula as a law;
integration-consistency relations; the two limiting counterexamples; the register row; tolerances and controls
for KA-058 to KA-063.

Out: the analysis A-07 and its verb (BD-106); the wheeled baseline controller (BD-117); the admission test of
`planar_drive` (BD-033), whose independent chord formula this suite reuses; tyre and slip laws; the discrete
nonholonomic step (construction C-36); the index pilot (N10a-15).

## Technical approach

**The object.** Rolling constraints are Pfaffian: A(q)·q̇ = 0, with q the configuration and A a matrix of
constraint rows. They define the distribution 𝒟 = ker A, spanned by vector fields g₁ … g_m (one per
independent input). The flag 𝒟 ⊂ 𝒟 + [𝒟, 𝒟] ⊂ 𝒟 + [𝒟, 𝒟] + [𝒟, [𝒟, 𝒟]] ⊂ … has a growth vector
(dim 𝒟₁, dim 𝒟₂, …) evaluated at a regular point. The Lie bracket of vector fields is fixed here as
[f, g](q) = (∂g/∂q)·f − (∂f/∂q)·g (the sign does not affect ranks).

- *Controllability* (Chow–Rashevskii): if the flag reaches the full tangent space at every point, any two
  configurations can be joined by a path tangent to 𝒟.
- *Cost* (ball-box theorem): a direction first reached at bracket depth w needs path length of order ε^(1/w)
  to move a distance ε.
- Worked cost (illustrative; card GB-05). A differential drive shifts sideways by d with the four-move
  manoeuvre rotate by α, drive forward, rotate back, reverse: wheel travel 2·α·b + d·cot(α/2) for half-track b.
  For small α this is 2·α·b + 2·d/α, least at α = √(d/b) with cost 4·√(d·b). With b = 0.20 m: a shift of
  0.01 m costs 0.18 m of wheel travel, 18 times the shift; a shift of 0.10 m costs 0.57 m by the small-angle
  form and 0.55 m by the exact one (α = 0.72 rad), 5.5 times the shift. This is the ball-box estimate for
  growth vector (2, 3): cost scales as the square root of the displacement. Assumptions: ideal rolling, wheel
  travel as the cost; the manoeuvre is one admissible path, so the figure is an upper bound on the optimum.

**Constant-twist pose formula** (independent of every sibling routine). For a planar body moving at constant
forward speed v and yaw rate ω for a time Δt from heading θ:
Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)) with δ = ω·Δt and sinc(x) = sin(x)/x, sinc(0) = 1; the
heading advances by δ. Exact for straight travel.

**Honest limits.**

1. A bracket direction is infinitesimal and assumes reversible inputs. With ẋ = u, u ∈ [0, 1], the rank is one
   and negative x is still unreachable from zero. A finite mission needs an integrated witness (an explicit
   admissible path), not a rank.
2. Ideal rolling is a regime. A four-wheel fixed-axis base with axles at ±a satisfies lateral no-slip only if
   v_y + a·ω = 0 and v_y − a·ω = 0, which forces ω = 0 (and v_y = 0). Its yaw exists only under a separately
   qualified slip model.

**Retirement.** When the drive slips outside the ideal-rolling regime, the kinematic prediction is withdrawn
for that operating region and the frictional model answers.

**Fixtures** (vector fields written as polynomials in (cᵢ, sᵢ) = (cos, sin) of each angle, with the derivation
rules ∂c/∂θ = −s, ∂s/∂θ = c, and evaluated at rational points of the circle such as (3/5, 4/5), so that every
rank is exact over ℚ).

| System | Configuration | Generators | Growth vector |
|---|---|---|---|
| Differential drive (unicycle) | (x, y, θ) | g₁ = (c, s, 0), g₂ = (0, 0, 1); [g₁, g₂] = (s, −c, 0) | (2, 3) |
| Car-like with steering | (x, y, θ, φ), wheelbase L | g₁ = (c_θ, s_θ, tan φ/L, 0), g₂ = (0, 0, 0, 1) | (2, 3, 4) |
| Car with one trailer | (x, y, θ, φ, θ₁), hitch length d | the car's fields extended by the trailer row θ̇₁ = (v/d)·sin(θ − θ₁) | (2, 3, 4, 5) |
| Ideal omnidirectional base | (x, y, θ) | three independent fields | (3) |
| Vertical rolling disk, radius R | (x, y, θ, ψ) | g₁ = (R·c, R·s, 0, 1), g₂ = (0, 0, 1, 0) | (2, 3, 4) |
| Mecanum base whose four rollers are mutually parallel, all wheels of one handedness (planted defect) | (x, y, θ) | every wheel constrains the same direction, so the rank of the wheel-to-base map drops from 3 to 2 | (2, 3) where (3) is required |

(tan φ is carried as the rational s_φ/c_φ at a point with c_φ ≠ 0.)

Trajectory fixtures (DESIGN constants, set here because the construction names the cases and gives no values):

- *Half turn.* Unit speed, +π in one second from heading 0: Δp = 1·1·sinc(π/2)·(cos(π/2), sin(π/2)) =
  (0, 2/π) m = (0, 0.636620) m.
- *Rolling disk.* R = 0.1 m, roll rate 10 rad/s, yaw rate 1 rad/s: speed 1 m/s on a circle of radius 1 m; after
  π seconds the disk has moved by (0, 2) m and its heading has turned by π.
- *Chaplygin sleigh.* Mass m, inertia I about the centre of mass, blade offset a: v̇ = a·ω²,
  ω̇ = −m·a·v·ω/(I + m·a²); the energy ½·m·v² + ½·(I + m·a²)·ω² is constant while the direction of linear
  momentum turns. Constants m = 1 kg, I = 0.1 kg·m², a = 0.2 m, v₀ = 1 m/s, ω₀ = 1 rad/s.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-07.S1 | The growth vectors of the five systems are as tabulated | Certified | Exact ranks of bracket spans at rational points |
| C-07.S2 | A unit-speed half turn through +π in one second gives the chord (0, 2/π) m | Tested (the value is exact in closed form; evaluated in `f64`) | Closed form |
| C-07.S3 | Exact straight travel, near-zero curvature, wrapped and unwrapped headings, and incremental against one-shot integration agree | Tested | Metamorphic property tests |
| C-07.S4 | ẋ = u, u ∈ [0, 1]: rank one, and x < 0 is unreachable from 0 | Certified (trivially, by sign) | The integrated witness set is [0, ∞) |
| C-07.S5 | Four fixed wheels under lateral no-slip force ω = 0 | Certified | Rank of a 2×2 rational system |
| C-07.S6 | Rolling disk and Chaplygin sleigh trajectories match the ideal-rolling model, which is the oracle for frictional wheel contact | Tested | Closed form for the disk; energy constancy and a high-order reference integration for the sleigh |
| C-07.S7 | A rank drop in the constraint distribution (misaligned rollers) is reported by the growth vector | Tested | Planted design |
| C-07.S8 | Lateral shift cost of a differential drive is 2·α·b + d·cot(α/2), minimized near α = √(d/b) at about 4·√(d·b) | Tested | Arithmetic on the card's numbers |

**What it does not claim.** The growth vector says nothing about slip, traction limits or dynamics. Full rank
is not reachability under one-sided inputs. The index and primitives are Frontier and claim only a static
screen for wheeled bases (the ceiling); the falsifier is that the index ranks reference bases differently from
measured cost.

**Implementation map.** Crates `frob-analyze` (growth vector, index) and `frob-contact` (rolling rows whose
ideal limit is this model). Verb: the `analyze.<name>` verb of analysis A-07 (maneuverability) and stage 4 of
`design.check`. Gates: the A-07 row of GATE-ATEL-02 (each analysis passes its known answers; `design check`
under 2 s at the 95th percentile on templates; M1); wheel rows of GATE-TRAC-01. No theorem card is specific to
this construction; its certificates are exact rank computations.

## Acceptance criteria

- [ ] `docs/math/C-07.md` exists with the object, the bracket convention, both limits, the fixture table, the
      trajectory fixtures, the eight statements and the retirement rule.
- [ ] `cargo test -p frob-analyze --test laws_c07` passes on the exact reference and on the production
      analysis once BD-106 lands.
- [ ] Growth vectors: (2, 3), (2, 3, 4), (2, 3, 4, 5), (3) and (2, 3, 4) for the disk, at three rational
      sample points each.
- [ ] The planted mecanum base reports (2, 3) and the finding names the rank drop.
- [ ] The half-turn chord equals (0, 2/π) m within 4 ulp in `f64`.
- [ ] The integration relations hold within 10⁻¹² m over 1 m paths, including the near-straight case (wheel
      speeds 1 and 1 + 6·10⁻¹³ m/s on a 0.5 m track from heading 32 rad for 400 steps of 1/400 s gives
      (0.8342, 0.5514) m to four decimals).
- [ ] The one-sided input and the fixed-axle base are reported with their limits: "rank is not reachability"
      and "no yaw under ideal rolling".
- [ ] KA-058 to KA-063 registered with tolerances and controls set; the register row of C-07 filled.

## Test plan

### Unit tests

- `s1_growth_vectors`: the table, at (c, s) ∈ {(3/5, 4/5), (5/13, 12/13), (8/17, 15/17)} per angle.
- `s2_half_turn_chord`: (0, 2/π).
- `s3_straight`: ω = 0 gives Δp = v·Δt·(cos θ, sin θ) exactly; `s3_near_straight`: the 32 rad case;
  `s3_wrapped`: headings θ and θ + 2π·k give equal Δp; `s3_incremental`: 400 steps equal one step of the same
  twist within 10⁻¹² m.
- `s4_one_sided`: bracket rank 1; reachable set from 0 over any horizon is non-negative.
- `s5_fixed_axle`: the system v_y + a·ω = 0, v_y − a·ω = 0 with a = 1/4 has only the zero solution.
- `s6_disk_circle`: radius R·(roll rate)/(yaw rate) = 1 m; position after π s is (0, 2) m.
- `s6_sleigh_energy`: energy constant within 10⁻⁹ relative over 10⁴ steps of the reference integrator; the
  heading of linear momentum changes by more than 0.1 rad (so a momentum test would fail a correct step).
- `s7_mecanum_rank_drop`: planted design.
- `s8_shift_cost`: b = 0.20 m; d = 0.01 m → 0.18 m; d = 0.10 m → 0.57 m small-angle and 0.55 m exact.

### End-to-end script and logging

`scripts/e2e/spec_c07.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G1 and G3 rows
KA-058 to KA-063; when the verb exists, `frob --json analyze maneuverability` on the `diff-drive` and `mecanum`
templates and on the planted mecanum design, asserting growth vectors and the finding text; the envelope's
evidence block (class `Certified` for exact inputs, determinism D0) is logged. Log
`target/e2e-logs/spec_c07/<utc>.ndjson` with transcript: commands, exit codes, growth vectors against
expected, chord components with units and tolerance, and on failure the first differing rank with the sample
point. Final `summary` line.

### Known answers and negative controls

- KA-058 growth vectors: exact; G1 · host. Controls (set here): bracket depth truncated at one (car-like
  reports (2, 3)); the planted mecanum base with parallel rollers.
- KA-059 constant-twist step: tolerance set to 4 ulp in `f64`; G1 · host. Control (set here): sinc replaced by
  1, which returns (0, 1) m.
- KA-060 planar pose integration: tolerance set to 10⁻¹² m over 1 m; G3 · host. Control: the near-straight
  defect reported in `fs-mbd` `planar_drive` (zero translation where about (0.8342, 0.5514) m is true).
- KA-061 rolling disk and sleigh: tolerance set to 10⁻⁹ relative on the ideal model, and the contact card's
  stated creep for the frictional wheel on the referee; G1 · referee. Control (set here): the disk's
  longitudinal no-slip row removed, so the disk no longer follows the circle of radius 1 m.
- KA-062 one-sided input: exact; G1 · host. Control: bracket rank read as reachability.
- KA-063 fixed-axis base: exact; G1 · host. Control: ideal rolling applied to a skid-steer base.
- CX-027, CX-028, CX-029.
- Determinism: exact tests D0; float relations twice, across worker counts, debug and release.

## Considerations

- The `planar_drive` report is unreproduced at a pin; the law suite uses the independent chord formula and
  never the sibling routine. If the defect reproduces, the upstream work item carries the fix and the
  admission test stays as a permanent control.
- Risk: growth vectors at singular points (jack-knifed trailer, φ = ±π/2) differ; the analysis samples regular
  points and reports the sample set.
- Symbolic brackets through FrankenSymPy are a build-time convenience; the reference here needs only
  polynomial differentiation with the two circle rules, so it has no external dependency.
- Interaction: the frictional wheel model must reduce to this model in the no-slip limit; that row belongs to
  the contact battery.

## References

Plan §10 C-07; §18.9; §28.4 (A-07); §33 (INT-FSIM-05); Appendix D.1 (constant-twist chord), D.3 (growth vector
and cost); Appendix E GB-05; Appendix M KA-058 to KA-063; Appendix N CX-027 to CX-029. Pilot: N10a-15.
