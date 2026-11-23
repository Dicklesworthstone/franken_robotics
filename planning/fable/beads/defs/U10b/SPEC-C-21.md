---
key: "SPEC-C-21"
title: "spec C-21 (frob-model, frob-phylum): the similarity group, canonical form and exact power-of-two test"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "lemma", "spine", "spec", "solid", "frob-model", "frob-phylum", "test"]
parent: "WP-44"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-21", "§10 registries, Appendix M.9 and Appendix N.7: the chapter registry beads N10-KA (unit U10a) and N10-CX (unit U10c) that load rows KA-119 to KA-122 and CX-056, CX-057; this bead binds them to tests and registers the controls the appendix leaves unstated", "§38.7: the metamorphic-relation registry row MR-06 (similarity rescaling) in `frob-conformance/registry/relations.toml` (Gauntlet beads); this bead supplies its statement and control"]
related: ["N10b-11", "BD-018", "BD-285", "BD-320", "BD-179", "BD-223", "SPEC-C-14", "SPEC-C-22", "SPEC-C-23", "CARD-T-09", "BD-061"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-03", "GATE-GAUNT-01"]
plan_refs: ["§10 C-21", "§9 D-11", "§12.3 T-09", "§12.5", "§17.11", "§35.8", "§38.7 MR-06", "Appendix A", "Appendix D.4", "Appendix D.5", "Appendix M.9 KA-119 to KA-122", "Appendix N.7 CX-056 CX-057"]
---
## Background

Raw SI numbers are a poor description of a robot: cache keys, policy inputs and scaling rules of thumb built on
them treat a robot and its exact half-scale copy as unrelated. Construction C-21 fixes the mathematics of
changing units. The **similarity group** rescales mass, length and time; applied to a whole experiment it maps
executions to executions (dynamic similarity). Dividing a design by its own scales gives a **canonical form**:
the list of its dimensionless groups. Two designs with equal groups share a canonical form, and a verdict moves
between them by an implication. Policies observe and act in dimensionless variables, so a family policy
interpolates over groups instead of memorizing sizes.

The same group gives an exact test of the whole stack that costs nothing to state: scale every dimensional
input by powers of two according to its dimension, and the run must reproduce the original **in every
significand bit and every branch**. A kernel that hides a dimensional constant typed as a pure number fails on
its first step.

Tier: Solid [S] for the canonical form and the exact test; Frontier [F] for canonical inputs to learned
policies (pilot N10b-11). Owning chapters: the design compiler (crate `frob-model`) and design families (crate
`frob-phylum`). It serves doctrine rules D-4 (structure is a test oracle) and D-11 (a verdict moves only under a
recorded implication).

Terms. *Dimension* of a quantity: its exponents (a, b, c) of mass, length and time. *Dimensionally homogeneous
kernel*: one in which every operation combines quantities of compatible dimension. *Dimensional rank* r of a
set of quantities: the rank of the matrix of their exponents. *Froude number*: v²/(g·ℓ). *Significand*: the
fractional part of a binary floating-point number, as distinct from its exponent.

Rejected alternatives: raw SI features (they carry a nuisance dimension); per-feature normalization fitted from
data (it is the fallback, and it does not give exact invariance); matching geometric ratios only (not
similarity, as the counterexample below shows).

## Scope

In: `docs/math/C-21.md` (statements S1 to S8); the law suite `crates/frob-model/tests/laws_c21.rs` with
reference model and mutants; registry rows KA-119 to KA-122; relation MR-06; controls from CX-056 and CX-057;
row C-21 of the construction register.

Out: the dimension checker of the kernel language (BD-018; theorem card T-09); the canonical form inside
campaign quotienting (BD-285); dimensionless tokens of family policies (BD-320; SPEC-C-23); the exact test run
on device tiers (the Gauntlet's metamorphic battery, BD-223); the pilot for canonical policy inputs (N10b-11).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-061 (the kinematic functor,
the first seed in `frob-model`): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-model` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements (status HYPOTHESIS unless marked; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-21.S1 | The similarity group is 𝕊 = (ℝ₊)³, acting by a change of the units of mass, length and time with factors (α_M, α_L, α_T). A quantity of dimension MᵃLᵇTᶜ scales by α_Mᵃ·α_Lᵇ·α_Tᶜ | Definition |
| C-21.S2 | Applied to a whole experiment (design, world, controller, clocks, thresholds, tolerances and gravity), an element of 𝕊 maps executions to executions. This is exact because kernels are dimensionally homogeneous | Rests on card T-09 (decided exactly by the kernel type checker); `Tested` end to end by S7 |
| C-21.S3 | The subgroup that fixes gravity is two-dimensional: α_L = λ, α_T = √λ, with any α_M | `Certified` (one line: gravity has dimension L·T⁻²) |
| C-21.S4 | A design is put in canonical form by dividing by its own scales: m₀ the total mass, ℓ₀ the radius of gyration of the whole robot about its centre of mass in the zero configuration, t₀ = √(ℓ₀/g) | Definition; `Tested` (invariance under 𝕊) |
| C-21.S5 | The canonical design is the list of its dimensionless groups; n quantities of dimensional rank r give n − r of them | `Certified` on fixtures (exact rank over ℚ) |
| C-21.S6 | Two designs with equal groups share a canonical form, and a verdict moves between them by an implication. **Every** group must match; groups that differ are reported by name | `Tested` |
| C-21.S7 | Exact similarity test: with the three factors powers of two and every dimensional input scaled by its dimension, a homogeneous kernel reproduces the original run in every significand bit and every branch, away from overflow and underflow, provided that representation constants with a dimension are scaled as inputs | HYPOTHESIS; the test is the gate. `Tested` (bit equality). No certificate models rounding |
| C-21.S8 | A kernel that hides a dimensional constant typed as a pure number fails the test on its first step | `Tested` (negative control) |

### 2. Formulas, every symbol defined

- **Canonical scales.** m₀ = total mass. ℓ₀: with c the centre of mass of the whole robot in its zero
  configuration, ℓ₀² = (∫‖x − c‖² dm)/m₀ = tr(Σ_C)/m₀ = tr(I_C)/(2·m₀), where Σ_C is the second moment of mass
  about c and I_C the rotational inertia about c (DESIGN: the source names "radius of gyration about the centre
  of mass"; this is the scalar root-mean-square distance, chosen because it is frame-independent). t₀ = √(ℓ₀/g)
  with g the magnitude of gravity.
- **Derived scales.** Force m₀·g; torque m₀·g·ℓ₀; power m₀·g·√(g·ℓ₀); stiffness m₀·g/ℓ₀; linear momentum
  m₀·√(g·ℓ₀); angular momentum m₀·ℓ₀·√(g·ℓ₀).
- **The groups of a design.** Link ratios (lengths over ℓ₀); mass fractions (masses over m₀); the actuator
  groups τ_max/(m₀·g·ℓ₀) and ω_max·t₀; the delay group t_d/t₀ of every clock and latency; contact stiffness
  k·ℓ₀/(m₀·g); friction coefficients; the Froude number v²/(g·ℓ₀) of each commanded speed.
- **Why power-of-two factors are exact (S7).** A homogeneous kernel uses only +, −, ×, ÷, √ and comparisons,
  and the type checker admits a square root only on even exponent vectors. With α_M, α_L, α_T powers of two,
  every input, hence every intermediate value, is the original value times a power of two; binary floating
  point represents that exactly and rounds it identically, so significands and comparison outcomes are
  unchanged. A square root of a quantity with an odd exponent would scale by an irrational factor (√2 for an
  odd mass exponent at α_M = 2), which is why the even-exponent rule matters.
- **Representation constants.** A constant with a dimension that belongs to the number representation, not to
  the physics, is scaled as an input. The anchored position format stores a position as an integer cell index
  plus an offset in a cell of side 8 m; when lengths are multiplied by 4 the cell side becomes 32 m, otherwise
  the offsets are not power-of-two images of each other.

### 3. Worked fixtures (all exact)

- **Count (KA-119).** A pendulum described by (T, ℓ, g, m) has four quantities of dimensional rank three, so
  one group: T·√(g/ℓ). Mass can enter no group.
- **Scaling counterexample (KA-120).** Scaling a body by λ at constant density multiplies mass by λ³ and static
  joint torque (mass × gravity × length) by λ⁴. For a fixture motor whose torque is declared to scale as λ³,
  the group τ_max/(m₀·g·ℓ₀) falls as 1/λ: a design that stands at one size fails at twice the size although
  every geometric ratio matches. Illustration: group value 1.2 at λ = 1 (it stands, the requirement being at
  least 1) becomes 0.6 at λ = 2.
- **Delay (KA-121).** A fixed latency t_d has delay group t_d/t₀ with t₀ ∝ √λ, so the group is proportional to
  λ^(−½): it doubles at one quarter of the size.
- **Exact test (KA-122).** Lengths × 4, times × 2, masses × 2, gravity unchanged (α_L = 4 = λ, α_T = 2 = √λ).
  Hand check on one explicit step of free fall, v′ = v − g·h and y′ = y + v′·h: the scaled inputs (2v, 2h, 4y,
  g) give 2v − g·2h = 2·v′ and 4y + 2v′·2h = 4·y′, exact in binary arithmetic. Rerun on each tier and compare
  bits.

### 4. Type sketch (DESIGN: fixes intent, not signatures)

```rust
pub struct Dim { pub m: i8, pub l: i8, pub t: i8 }
pub struct Similarity { pub alpha_m: Pow2, pub alpha_l: Pow2, pub alpha_t: Pow2 }   // Pow2: an exponent, exact
pub struct CanonicalScales { pub m0: Mass, pub l0: Length, pub t0: Time }
pub struct CanonicalForm { pub scales: CanonicalScales, pub groups: Vec<(GroupName, f64)> }
pub fn canonicalize(model: &RobotModel, g: Accel) -> CanonicalForm;
pub fn differing_groups(a: &CanonicalForm, b: &CanonicalForm, tol: f64) -> Vec<GroupName>;
```

### 5. Honest limits, foundations

- Matching some groups is not similarity; every group must match. Material strength, motor technology, sensor
  noise and network latency do not scale with a robot, so two real robots of different size are rarely
  similar, and the canonical form reports this as named groups that differ.
- The floating-point gap between a kernel and its identity is measured by residual tests and by this test, not
  proved: certificates are over the rationals with square roots and do not model rounding.
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-regime` has a dimensionless-group
  basis (`pi::pi_groups`), named standard groups, scaling maps (`scaling::ScalingMap`) and model admission by
  distance to a validity range. Not admitted for this use until its admission gate passes.

### 6. Law suite and registry (DESIGN)

- `crates/frob-model/tests/laws_c21.rs`, generic over `C21Subject { fn canonicalize(&self, d: &DesignData) ->
  CanonicalForm; fn scale(&self, d: &DesignData, s: Similarity) -> DesignData; fn step(&self, state, inputs) ->
  State }`. `tests/support/c21_ref.rs`: dimension vectors and exact rank over ℚ; point-mass designs with
  rational data; a small homogeneous reference kernel (free fall with a ground-contact branch and one square
  root) in `f32` and `f64`. `tests/support/c21_mutants.rs`: mutants under the cargo feature `controls`.
- Register row C-21: `verb = "phylum.quotient"`, `failing_test` =
  `frob-model::laws_c21::s8_hidden_dimensional_constant`, `payoff.gate = "GATE-PHYLUM-01"`, `cards = ["T-09"]`,
  `known_answers = KA-119 … KA-122`, `pilot = "PILOT-C-21"` (for the Frontier part), `generic_comparator = "SI
  inputs with per-feature normalization"`.
- Gates that measure the construction (recorded by mechanism beads): GATE-GAUNT-03 (each metamorphic relation,
  MR-06 among them, holds on 10⁴ random scenarios per tier it names and each registered mutation breaks it; M2,
  then every exit); GATE-PHYLUM-01 (campaign quotienting); GATE-CORTEX-11 (re-described robots, power-of-two
  scaling included, keep at least 80% of return).

## Acceptance criteria

- [ ] `docs/math/C-21.md` has the ten sections, statements C-21.S1 to S8 with classes, the scale and group
      tables and the four fixtures.
- [ ] `cargo test -p frob-model --test laws_c21` passes; with `--features controls` every mutant fails.
- [ ] KA-119: exponent matrix of (T, ℓ, g, m) has rank 3 over ℚ; the single group is T·ℓ^(−½)·g^(½).
- [ ] KA-120 and KA-121: exponents −1 and −½ computed symbolically; a similarity verdict issued from geometric
      ratios alone is refused with the differing group named (`actuator torque group`).
- [ ] KA-122 on the reference kernel: for 10⁴ seeded states the scaled run equals the original in every
      significand bit and every branch, in `f32` and in `f64`, over 1,000 steps.
- [ ] The hidden-constant mutant differs on its first step; a scale factor of 10 does not reproduce bits.
- [ ] Canonical forms of a design and of its image under 20 seeded similarity elements are identical.
- [ ] `scripts/e2e/spec_c21.sh` exits 0.

## Test plan

### Unit tests

- `s1_scaling_exponents`: force scales by α_M·α_L·α_T⁻²; power by α_M·α_L²·α_T⁻³; stiffness by α_M·α_T⁻².
- `s3_gravity_fixing_subgroup`: (α_L, α_T) = (4, 2), (16, 4), (1/4, 1/2) leave g unchanged; (4, 4) does not.
- `s4_scales_invariance`: a three-point-mass robot (masses 1, 2, 3 kg at rational positions): m₀ = 6 kg,
  ℓ₀² = tr(Σ_C)/m₀ computed exactly; under lengths × 4, masses × 2: ℓ₀ × 4, m₀ × 2, t₀ × 2.
- `s4_derived_scales`: the six derived scales have the dimensions of force, torque, power, stiffness, linear
  and angular momentum (checked on exponent vectors).
- `s5_group_count`: pendulum 4 − 3 = 1; a design with 12 quantities of rank 3 has 9 groups.
- `s6_every_group_must_match`: two designs equal in link ratios and mass fractions, different in the actuator
  torque group: `differing_groups` returns exactly that name.
- `s7_bits_reproduce`, `s7_branches_reproduce` (the ground-contact comparison takes the same side at every
  step), `s7_representation_constant_scaled` (cell side 8 m against 32 m; unscaled, the test fails).
- `s8_hidden_dimensional_constant`, `s7_factor_ten_is_not_exact`, `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c21.sh` (idempotent; seed 2100 printed): `frob --json doctor`; the law suite with and without
`--features controls`, in debug and in release; `cargo test -p frob-conformance --test g1 -- KA-119 KA-120
KA-121`; `cargo test -p frob-conformance --test g3 -- MR-06` on the host site; `frob --json prove list` and
assert card T-09 is listed. Sources the STD-E2E helper; writes `target/e2e-logs/spec_c21/<utc>.ndjson` and a
transcript; each line has `ts`, `script`, `bead` (`SPEC-C-21`), `step`, `event` (`start`, `cmd`, `assert`,
`metric`, `refusal`, `end`), command line, exit code, wall time, evidence block (claim Known answer, class
Tested, tier G1 or G3 host, determinism D0 for exact rows, seed), digests of the original and scaled runs, and
on failure the first step and state component whose significand differs, with both bit patterns. Final
`summary`: assertion counts, GATE-GAUNT-02 and GATE-GAUNT-03.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-119 | (T, ℓ, g, m): four quantities of rank three give one group, T·√(g/ℓ) | exact | G1 · host | Registered here (the appendix states none): a count that uses the number of base units (3) in place of the rank on a set whose rank is 2 |
| KA-120 | Mass × λ³; static joint torque × λ⁴; the torque group falls as 1/λ | exact (exponents) | G1 · host | CX-056: a similarity verdict issued from geometric ratios alone must be refused |
| KA-121 | The delay group of a fixed latency is proportional to λ^(−½); it doubles at one quarter of the size | exact (exponents) | G1 · host | Registered here: a delay expressed in control ticks and treated as dimensionless, which hides the change |
| KA-122 | Lengths × 4, times × 2, masses × 2, gravity unchanged: every significand bit and every branch reproduced | exact (bit equality) | G3 · host here; G3 · all tiers in the Gauntlet battery | CX-057: a kernel that hides a dimensional constant typed as a pure number fails on its first step; relation MR-06's control: a scale factor of 10 |

Determinism lanes: the exact test itself is a determinism test; it runs twice on one machine, in debug and in
release, and with 1, 2 and 8 worker threads, with identical digests.

## Considerations

- The exact test is a HYPOTHESIS until it has passed on every tier. If a tier fails for a reason outside the
  kernels (a driver's fused multiply-add, a reduction order), the failure is recorded against that tier's
  determinism class and the test stays a requirement on the others.
- Overflow and underflow bound the admissible factors; the suite keeps scaled values inside the normal range
  and asserts it.
- Every tolerance and threshold is an input with a dimension; a unitless epsilon applied across lengths and
  angles is invalid and is exactly what the test exposes.
- Fallback for the Frontier use (policy inputs): SI inputs with per-feature normalization.

## References

Plan §10 C-21; §9 D-4, D-11; §12.3 T-09; §12.5; §17.11; §35.8; §38.7 (MR-06); Appendix A (canonical form,
similarity group, power-of-two similarity test); Appendix D.4 (Froude number), D.5 (similarity rescaling);
KA-119 to KA-122; CX-056, CX-057, CX-112; GATE-GAUNT-03; GATE-PHYLUM-01.
