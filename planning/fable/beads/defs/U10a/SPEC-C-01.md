---
key: "SPEC-C-01"
title: "frob-lie: C-01 specified — configuration group, joint subgroups, Hervé classes; contract and law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "marrow", "frob-lie", "solid", "frontier", "spine", "spec", "test", "review-notes-open"]
parent: "WP-03"
deps: ["BD-025", "N10a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-010", "BD-046", "BD-101", "N13-27", "SPEC-C-11"]
gates: ["GATE-MARROW-01", "GATE-ATEL-02"]
plan_refs: ["§10 C-01", "§13.1", "§13.7", "§13.8", "§12.3 T-11", "Appendix D.1", "Appendix M KA-037 to KA-039", "Appendix N CX-019"]
---
## Background

Construction C-01 is the first statement of the mathematical spine: a robot's configuration space is a Lie
group, and every joint is a product of one-parameter subgroups of SE(3) given by a screw basis. It replaces an
`enum JointKind` with one hand-written code path per case. The payoff is twofold. First, one kernel generator
parameterized by a screw basis covers every joint, so a new joint type is data and not code. Second, mobility
analysis (analysis A-01) can use intersections and products in the lattice of subgroups of SE(3), which gives
the right answer for exceptional over-constrained linkages whose mobility follows from subgroup structure (the
Sarrus linkage) and returns no verdict for paradoxical ones (Bennett, Bricard), whose mobility rests on metric
conditions and is decided by a rank test alone. Both kinds are linkages where the Grübler–Kutzbach count is
wrong.

This bead is the **specification** of C-01. The mechanisms are implemented elsewhere (`frob-lie` joint
subgroups and classification, `frob-model` lowering, the mobility analysis in `frob-analyze`). What this bead
delivers is the mathematics as a contract, the law tests that fail when an implementation gets the mathematics
wrong, and the registry rows that tie the construction to its known answers, theorem card and gates.

Tier: Solid for the kernel generator and the classification; Frontier for lattice-based mobility, which has a
pilot (N13-27, record `PILOT-C-01`) and a named fallback, the numerical rank test alone.

Foundations today (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`; existence is not admission):
`fs-mbd` has `JointKind = Fixed | Revolute | Prismatic | Helical` only, with no spherical, universal or planar
joints; `fs-ga` has `Twist::bracket` and `Twist::ad`; the joint catalog crate `fs-kinematics` proposed by
FrankenSim's charter does not exist. So the classification and the generator are new code in `frob-lie`.

Rejected alternative: keeping an enumerated joint kind and adding cases. Reason: each case is a separate code
path in three back ends, the count of cases is open-ended (the universal joint is not even a subgroup), and the
enumeration gives no handle on mobility.

## Scope

In: `docs/math/C-01.md` following the ten-section contract template; the law suite
`crates/frob-lie/tests/laws_c01.rs` with an exact rational reference and registered negative controls; exact
fixtures for the Sarrus and Bennett linkages; the register row of C-01; tolerances and controls for known
answers KA-037 to KA-039; the statement of theorem card T-11 as this construction uses it.

Out: the production classifier and joint maps (BD-046); the kinematic functor and referee lowering (BD-061);
the mobility analysis and its verb (BD-101); the lattice-mobility pilot (N13-27); stratified state spaces at
singular poses, unilateral contacts and mode changes (construction C-16, another unit).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-010 (creates `frob-lie` with
the group traits): that bead is listed in `related` and is not a prerequisite. The law suite is generic over its
subject trait and closes on the exact reference model and the mutants in its own test support. If `frob-lie` does
not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**Conventions.** A twist is ξ = (ω, v), angular part first. The bracket on 𝔰𝔢(3) is
[ξ₁, ξ₂] = (ω₁ × ω₂, ω₁ × v₂ − ω₂ × v₁). A screw S ∈ 𝔰𝔢(3) with ω ≠ 0 has pitch p = (ω·v)/(ω·ω) and an axis
through the point ω × v/(ω·ω); a screw with ω = 0 is a pure translation. exp is the group exponential of SE(3).

**The object, in full.**

- A floating-base robot's configuration space is Q = SE(3) × G₁ × … × Gₙ. Joint j contributes the map
  θ ↦ ∏ₖ exp(θₖ·Sₖ) for screws Sₖ ∈ 𝔰𝔢(3).
- When span{Sₖ} is closed under the bracket, the joint is a connected subgroup of SE(3). Connected subgroups
  fall into twelve conjugacy classes (Hervé): the identity; line translation; rotation about an axis; helical
  motion of pitch p; planar translation; cylindrical; spatial translation; planar gliding SE(2); spherical
  SO(3); translating screw; Schönflies; and SE(3) itself.
- The six lower pairs are the revolute, prismatic, helical, cylindrical, planar and spherical classes.
- A joint that is not a subgroup (a universal joint) is a product of one-parameter subgroups.
- Joint limits cut a feasible subset out of Q and loop closures cut level sets. The regular part is a manifold
  where the constraint Jacobian has constant rank; singular poses, unilateral contacts and mode changes need a
  stratified description that this construction does not give.
- State also includes motor, battery, thermal, contact-memory, estimator and controller coordinates: Q is the
  configuration space, not the state space.

**Classification table** (DESIGN: the decision procedure the contract fixes; all arithmetic exact over ℚ). Let
𝔥 be the bracket closure of span{Sₖ}, d = dim 𝔥, r = the rank of the projection of 𝔥 onto its angular parts
(r ∈ {0, 1, 3}; r = 2 cannot occur because 𝔰𝔬(3) has no two-dimensional subalgebra), and t = dim(𝔥 ∩ {ω = 0}).

| d | r | t | Further test | Class | Lower pair |
|---|---|---|---|---|---|
| 0 | 0 | 0 | — | identity | — |
| 1 | 0 | 1 | — | line translation | prismatic |
| 1 | 1 | 0 | pitch p = 0 | rotation about an axis | revolute |
| 1 | 1 | 0 | pitch p ≠ 0 | helical motion of pitch p | helical |
| 2 | 0 | 2 | — | planar translation | — |
| 2 | 1 | 1 | the translation is parallel to ω | cylindrical | cylindrical |
| 3 | 0 | 3 | — | spatial translation | — |
| 3 | 1 | 2 | translations span the plane normal to ω; residual pitch 0 | planar gliding SE(2) | planar |
| 3 | 1 | 2 | same, residual pitch p ≠ 0 | translating screw | — |
| 3 | 3 | 0 | all axes pass through one point | spherical SO(3) | spherical |
| 4 | 1 | 3 | — | Schönflies | — |
| 6 | 3 | 3 | — | SE(3) | — |

Any other (d, r, t) for a bracket-closed span is an internal error, reported, never guessed.

**Algorithm (reference, exact).**

1. Reduce the input screws to a basis by exact Gaussian elimination over ℚ.
2. Close under the bracket: for every pair of basis elements compute the bracket; if it is outside the span
   (exact rank test), record `closed = false` and add it; repeat until no pair adds a vector (at most six
   vectors).
3. If the original span was closed, read (d, r, t) and the pitch test from the table and return the class.
   Otherwise return `Product` with the original screws in order and report the class of the closure for
   information.
4. Emit the certificate of theorem card T-11: the basis, the structure constants cᵢⱼᵏ with
   [Sᵢ, Sⱼ] = Σₖ cᵢⱼᵏ·Sₖ, and the invariants (d, r, t, p). A checker re-multiplies and compares exactly.

**Mobility.** The Grübler–Kutzbach count is M = 6·(n − 1 − j) + Σ fᵢ for n links, j joints and joint freedoms
fᵢ. It is wrong for over-constrained linkages. The construction's two methods:

- *Rank method* (Solid; the method of record). For a single loop with joint screws S₁ … Sₘ expressed in one
  frame at a configuration that closes the loop, mobility = m − rank[S₁ … Sₘ], evaluated in exact rationals at
  rational sample configurations. A sample is admitted only if the loop product ∏ exp(θₖ·Sₖ) equals the
  identity exactly.
- *Lattice method* (Frontier). For a platform joined to a base by limbs whose joints generate subgroups Hᵢ, the
  platform's motion is contained in ⋂ Hᵢ, and mobility = dim(⋂ Hᵢ) + Σ (number of joints of limb i − dim Hᵢ)
  when every limb generates its full subgroup. When a limb's screws generate no proper subgroup the lattice
  returns `undecided`: no verdict, never a number.

**Statements** (the contract's statement table).

| Id | Statement | Class | Route |
|---|---|---|---|
| C-01.S1 | The bracket on 𝔰𝔢(3) is antisymmetric and satisfies the Jacobi identity | Certified | Polynomial identity in eighteen rational variables |
| C-01.S2 | A screw basis is, or is not, closed under the bracket, and its class is as reported (T-11) | Certified | Exact rational linear algebra; structure constants re-checked |
| C-01.S3 | For a closed basis, θ ↦ ∏ exp(θₖ·Sₖ) composes like its subgroup: the product of two images is an image | Tested | Property test on rational-angle samples (rational points of the unit circle) |
| C-01.S4 | A universal joint (two intersecting orthogonal revolute axes) is not closed and is returned as `Product` | Certified | [(e_x, 0), (e_y, 0)] = (e_z, 0), outside the span |
| C-01.S5 | Sarrus linkage: mobility 1 by the lattice and by the rank method; the count gives 0 | Certified | Exact subalgebra intersection; exact rank |
| C-01.S6 | Bennett linkage: the lattice returns `undecided`; the rank method returns 1; the count gives −2 | Certified | Exact rank at admitted rational samples |
| C-01.S7 | The six lower pairs classify as revolute, prismatic, helical, cylindrical, planar, spherical | Certified | Table lookup on exact invariants |

**Fixtures** (DESIGN constants; exact).

- *Sarrus.* Six links, six revolute joints, two limbs of three joints each. Limb A: axes parallel to e_x through
  (0, 0, 0), (0, 1, 1), (0, 0, 2). Limb B: axes parallel to e_y through (0, 0, 0), (1, 0, 1), (0, 0, 2). The
  span of limb A is {ω ∈ span(e_x), v ∈ span(e_y, e_z)}, the planar-gliding algebra with normal e_x; limb B the
  same with normal e_y. Their intersection is {ω = 0, v ∈ span(e_z)}, of dimension 1. Count:
  6·(6 − 1 − 6) + 6 = 0. Truth: 1.
- *Bennett.* Four links, four revolute joints; opposite links equal. Link lengths a = 3 and b = 4; twist angles
  α and β with (sin α, cos α) = (3/5, 4/5) and (sin β, cos β) = (4/5, 3/5), so a/sin α = b/sin β = 5, the
  Bennett condition. Half-angle tangents are rational: tan(α/2) = 1/3, tan(β/2) = 1/2, and the ratio
  (tan(β/2) + tan(α/2))/(tan(β/2) − tan(α/2)) = 5 parameterizes the one-parameter motion by rational
  half-angle tangents of the joint angles. The derivation file fixes the sign convention of that relation; a
  sample enters the rank test only after its loop product is exactly the identity. Count:
  6·(4 − 1 − 4) + 4 = −2. Truth: 1.
- *Lower pairs.* One canonical basis per class at the origin, and each conjugated by three rational rigid
  motions (the class is a conjugacy invariant).

**Failure cases and refusals.** A basis with a non-rational entry is refused by the exact classifier; imported
designs go through the tolerance route of the model compiler and the result is `Tested`, never `Certified`. A
joint class declared in a design that disagrees with its screw basis is the design-language refusal
FROB-REF-101 (port type mismatch; joint class and screw basis disagree), raised by the model compiler. A host
computation over its declared rational size is FROB-REF-215.

**What it does not claim.** Lattice mobility is not a general mobility theorem: it decides only linkages whose
limbs generate subgroups. Instantaneous rank at a singular configuration can exceed finite mobility, so the
rank method samples regular configurations and reports the sample set. No speed is claimed for either method.

**Implementation map.** Types: `frob_lie::JointSubgroup { class: HerveClass, screws: ScrewBasis,
closed_under_bracket: bool }` (DESIGN sketch). Verb: the `analyze.<name>` verb of analysis A-01, written
`analyze.mobility` here (the verb registry fixes the spelling); the classification is also visible in
`design.check` findings. Gates: the "Subgroups" law of GATE-MARROW-01 (a classified screw basis
is bracket-closed; the generated joint map composes like its subgroup; all laws on 10⁶ cases, at most 64 ulp
in `f64` and 16 ulp in `f32` for single operations, TARGET) and the A-01 row of GATE-ATEL-02 (TARGET: each
analysis passes its known answers). Card: T-11. Determinism class of the reference: D0.

## Acceptance criteria

- [ ] `docs/math/C-01.md` exists with the ten sections, the seven statements above with their classes, the
      classification table, both mobility methods, the fixtures and the limits.
- [ ] `cargo test -p frob-lie --test laws_c01` passes against the exact reference, and against the production
      classifier once BD-046 lands (the suite is generic over the classifier trait).
- [ ] Each of the twelve classes is recognized from a basis and from three rational conjugates of it; a
      universal joint is reported as `Product`.
- [ ] Sarrus: lattice 1, rank 1, count 0. Bennett: lattice `undecided`, rank 1 at five admitted samples,
      count −2.
- [ ] Every control below is registered in `controls.toml` and caught in the same run.
- [ ] KA-037, KA-038 and KA-039 are in `known_answers.toml` with tolerance `exact`, tier and control; KA-039's
      control is no longer `none stated`.
- [ ] The register row of C-01 has its four D-2 obligations filled or reported `pending` with a milestone.
- [ ] A fresh-context reviewer confirms the contract can be implemented without any other document.

## Test plan

### Unit tests

- `s1_bracket_identities`: 10³ random rational triples of twists; antisymmetry and Jacobi hold exactly.
- `s2_closure_and_class`: the twelve canonical bases and 36 conjugates; expected class per row of the table.
- `s3_subgroup_composition`: for each closed class, 10³ pairs of rational-angle parameters; the product lies in
  the image. Exact for the classes whose maps are rational at rational points of the circle: solved for
  revolute, prismatic, cylindrical, planar; by exact invariants for the others (the three translation classes:
  rotation part the identity and translation in the subspace; spherical: the product fixes the common point of
  the axes; Schönflies: rotation axis parallel to w; SE(3): trivially). The helical and translating-screw
  classes take the angle both through (cos θ, sin θ) and linearly through the pitch term p·θ, so no nonzero
  sample is rational: they run in `f64` (exp(θ₁·S)·exp(θ₂·S) = exp((θ₁ + θ₂)·S), and membership by the
  axis-and-pitch invariant) under the single-operation tolerance of the Subgroups law, 64 ulp, and the log
  names that lane `f64`, not exact.
- `s4_universal_is_product`: basis {(e_x, 0), (e_y, 0)} → `closed = false`, closure of dimension 3 (spherical).
- `s5_sarrus`: the fixture → intersection dimension 1; rank of the 6 × 6 screw matrix 5; count 0.
- `s6_bennett`: the fixture at five rational samples → rank 3 of the 6 × 4 screw matrix each time; lattice
  `undecided`; count −2; a sample whose loop product is not the identity is rejected by the admission check.
- `s7_lower_pairs`: six bases → six names.
- `t11_certificate_roundtrip`: the structure-constant certificate re-checks; a certificate with one constant
  changed is rejected.

### End-to-end script and logging

`scripts/e2e/spec_c01.sh`. Sources the shared logging helper of STD-E2E. Steps: (1) `cargo test -p frob-lie
--test laws_c01`; (2) the same once per registered control, each in its own build with the mutation feature
the control's record names (`cargo test -p frob-lie --features <feature> --test laws_c01`: `cx-019` for the
plant of CX-019, the `nc-` feature of each other control below; N10a-01 section 6), asserting failure of the
test the record names; (3) the known-answer rows by the Gauntlet's cargo form, through STD-E2E's `e2e_tier`:
`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- KA-037` and the same for KA-038
(rows of tier G1), and `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g0_laws -- KA-039` (a G0
row), each again with `--features gauntlet-mutations` for its control; (4) when the verb exists (BD-101, logged
`pending` until then), `frob analyze mobility fixtures/linkages/sarrus.fbot --json` and the same for
`fixtures/linkages/bennett.fbot` (the design is the verb's positional argument, N28a-05; there is no `--design`
flag), asserting mobility 1 by the rank of the constraint Jacobian at exact rational samples, the
Grübler–Kutzbach count reported beside it (0 and −2), the field `lattice` as the row defines it (`not-run` on
the default build; with the feature `frontier-lattice-mobility` from M3, `decided(1)` for Sarrus and
`undecided(…)` for Bennett), and the envelope's evidence block (class `Certified` for exact inputs,
determinism D0). Logs NDJSON to
`target/e2e-logs/spec_c01/<utc>.ndjson` with a
transcript: `ts`, `script`, `bead`, `step`, `event`, command, exit code, wall time, evidence block, the
measured mobility and count against their expected values, and on failure the first differing value and the
fixture path. Final `summary` line with assertions and gates touched.

### Known answers and negative controls

- KA-037 Sarrus: mobility 1, count 0; exact; G1 · host. Control: the count alone, which returns 0.
- KA-038 Bennett: mobility 1, count −2; exact; G1 · host. Control: the count alone, which returns −2.
- KA-039 classification; exact; G0 · host. Control (set by this bead): a bracket with the linear part dropped,
  [ξ₁, ξ₂] ↦ (ω₁ × ω₂, 0). The basis {rotation about e_x, translation along e_y} has true bracket (0, e_z) and
  is not closed (its closure is the planar-gliding algebra with normal e_x, d = 3); the mutant reports it
  closed, with (d, r, t) = (2, 1, 1) and a translation not parallel to ω, so it either names it cylindrical
  (if the further test is skipped) or falls in no row of the table. Either outcome differs from `Product` and
  is caught.
- CX-019: mobility from the Grübler–Kutzbach count (both linkages).
- Further controls: bracket arguments exchanged only in the linear part (breaks Jacobi); pitch test omitted
  (helical reported as revolute); lattice forced to answer on Bennett (must be caught by S6's `undecided`).
- Metamorphic: conjugation invariance of the class; reordering the basis leaves class and mobility unchanged.
- Determinism: exact arithmetic, identical results twice, across worker counts, debug and release, on Linux
  and macOS.

## Considerations

- The lattice method is Frontier and can be killed: falsifier, one wrong mobility against the exact rank;
  fallback, the rank test alone. The spec keeps both methods so the fallback needs no new mathematics.
- Rational sample points for linkages with trigonometric closure need rational points of the circle; the
  admission check guards against a wrong parameterization.
- Bit budgets: closure and rank on 6 × 6 rational matrices stay small; a blow-up is the typed refusal
  FROB-REF-215, not a silent fallback to floating point.
- The classification is consumed by the model compiler and by the kernel generator, so a misclassified joint
  would produce a wrong kernel; this is why the class carries a certificate.

## References

Plan §10 C-01; §13.1, §13.7, §13.8; §12.3 (T-11); Appendix D.1 (bracket, mobility count); Appendix M KA-037 to
KA-039; Appendix N CX-019. Analysis A-01. Pilot: N13-27.
