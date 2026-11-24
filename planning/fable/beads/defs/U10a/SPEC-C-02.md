---
key: "SPEC-C-02"
title: "frob-lie: C-02 specified — frame groupoid, three typed frame maps, power pairing; contract and law tests"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "marrow", "frob-lie", "solid", "spine", "spec", "test"]
parent: "WP-03"
deps: ["BD-025", "N10a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-010", "BD-047", "N13-18", "SPEC-C-12", "SPEC-C-24"]
gates: ["GATE-MARROW-01", "GATE-LEMMA-00", "GATE-MARROW-04"]
plan_refs: ["§10 C-02", "§11.1", "§11.2", "§11.3", "§11.4", "§13.2", "§13.3", "§13.8", "§13.10", "§12.3 T-04 T-05 T-16", "Appendix D.1", "Appendix I.1", "Appendix M KA-040 to KA-042", "Appendix N CX-020"]
---
## Background

Construction C-02 fixes how spatial quantities are typed and moved between frames. It replaces 4×4 matrices
and 6-vectors whose frames live in comments. The payoff: `a_from_b · b_from_c` has type `a_from_c` and nothing
else composes; a twist in frame b cannot be added to a twist in frame c; a wrench cannot be moved by the twist
map. Static frames are phantom types and model-indexed frames are checked at model compile time, so batched
kernels pay nothing. A derivative query at a chart cut requests another chart or receives a typed refusal.

The reason this is a construction and not a style rule: almost every sign and transpose error in a rigid-body
simulator is a frame error, and several of them are invisible to the obvious tests. A reversed infinitesimal
coadjoint conserves kinetic energy; a wrench moved by the twist map keeps its force and silently loses its
moment arm. The construction therefore fixes each convention once, gives each map its own type, and requires
that a single reversed adjoint fail at least three independent fixtures.

Decisions governing this construction (stated so the bead stands alone):

- ADR-MARROW-01: reuse FrankenSim's `fs-ga` as the host representation (`So3`, `Se3`, `Twist`, `Wrench`,
  `Mat6`), with deterministic elementary functions already routed through `fs_math::det` (FACT). FrankenSim has
  two vector vocabularies today (`fs-mbd` defines its own `Vec3`, `UnitQuaternion`, `Pose` beside `fs-ga`'s);
  FrankenRobotics does not add a third. `frob-lie` adds traits, missing groups and frame typing. Rejected: a
  new linear-algebra vocabulary.
- ADR-MARROW-05: there are two coadjoints, each with its own name, type and sign. The group coadjoint is the
  inverse transpose of Ad and moves wrenches; the infinitesimal coadjoint ad* is the transpose of ad and
  appears in the Euler–Poincaré equation. Reason: an earlier draft described ad* with a phrase ("negative
  transpose") that is true only of the generator of the group coadjoint, which differs from ad* by a sign.
- ADR-MARROW-09: a reflection is not a transform. `Transform<A, B>` holds an element of SE(3); mirror images
  are carried by a separate `Isometry` type with an explicit determinant. Reason: a mirrored limb needs sign
  changes on axial quantities that a rigid transform never applies, and hiding the sign inside a "transform"
  is how mirrored encoders end up reading backwards.

Tier: Solid. Foundations (FACT, FrankenSim `483d09c682f2`, observed 2026-11-21): `fs-ga` provides
`Se3::{compose, inverse, adjoint, coadjoint}`, `Twist::transform_by`, `Wrench::{pairing, transform_by}`;
`Se3::coadjoint` is documented as Ad⁻ᵀ (path `frankensim/crates/fs-ga/src/lie.rs`), which fixes the convention
below; `fs-mbd` already names transforms `world_from_base`; `fs-ga` has `Twist::ad` and no infinitesimal
coadjoint, so `coad` is new code.

## Scope

In: `docs/math/C-02.md`; the law suite `crates/frob-lie/tests/laws_c02.rs` with an exact rational reference;
the boundary sample set; the three-fixture rule for a reversed adjoint; chart-loop budgets; the register row;
tolerances and controls for KA-040 to KA-042; the statements of theorem cards T-04, T-05 and the pairing and
bracket half of T-16.

Out: the production types and their phantom-frame machinery (BD-010, BD-047); the Cayley and exponential maps
themselves (SPEC-C-05, BD-011); reflection parity as used by symmetry (SPEC-C-09), though the parity map's
preservation of pairing and bracket is stated here because T-16 covers both; the bit-comparison fixture against
`fs-ga` (BD-010, proof obligation PO-MARROW-01, which this suite re-uses as its sample source).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-010 (creates `frob-lie` and
its production frame types): that bead is listed in `related` and is not a prerequisite. The law suite is generic
over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-lie` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**The object.** Frames are objects and a rigid transform `a_from_b` is a morphism from b to a, so transforms
form a groupoid. A pose is g = (R, p) and acts on coordinates as x_a = R·x_b + p. Composition reads left to
right by cancelling inner names. The adjoint Ad is a functor from that groupoid to invertible linear maps on
𝔰𝔢(3); the group coadjoint is its contragredient on 𝔰𝔢(3)*, the inverse transpose. A twist ξ = (ω, v), angular
part first, is a vector typed by a frame; a wrench w = (τ, f), torque first, is a covector typed by a frame;
their pairing ⟨w, ξ⟩ = τ·ω + f·v is power in watts and does not depend on the frame. A reflection is not a
morphism of this groupoid, because improper isometries are not in SE(3); it acts through the parity rule as a
separately typed map. Coordinate charts (logarithms, joint charts) are partial maps with stated overlap
domains.

**Formulas** (x̂ is the 3×3 cross-product matrix of x; g = (R, p) = `a_from_b`; μ = (π, ℓ) a momentum).

```text
Ad_g    = [[ R, 0 ], [ p̂·R, R ]]        (ω, v) ↦ (R·ω, R·v + p × R·ω)          twists, b to a
Ad_g⁻ᵀ  = [[ R, p̂·R ], [ 0, R ]]        (τ, f) ↦ (R·τ + p × R·f, R·f)          wrenches, b to a (pushforward)
Ad_gᵀ                                    (τ, f) ↦ (Rᵀ·(τ − p × f), Rᵀ·f)        wrenches, a to b (pullback)
ad_ξ    = [[ ω̂, 0 ], [ v̂, ω̂ ]]         ad_ξ η = [ξ, η] = (ω × η_ω, ω × η_v + v × η_ω)
ad*_ξ   = ad_ξᵀ                          ad*_ξ μ = (π × ω + ℓ × v, ℓ × ω)
generator of the group coadjoint         d/dt Ad_{exp(tξ)}⁻ᵀ at t = 0 equals −ad*_ξ   (derived in tests, never stored)
```

The algebra coadjoint is the dual map, defined by ⟨ad*_ξ μ, η⟩ = ⟨μ, [ξ, η]⟩; in coordinates it is the
transpose of ad_ξ with no minus sign. The map ξ ↦ −ad_ξᵀ is never written ad* in this project; where it is
needed it is written −ad_ξᵀ.

**The three typed maps.** No function accepts one where another is expected.

| Type | Matrix for g = `a_from_b` | Takes | Law |
|---|---|---|---|
| `TwistMap` | Ad_g | a twist in b to a twist in a | preserves the bracket; composes as Ad(g·h) = Ad(g)·Ad(h) |
| `WrenchPushforward` | Ad_g⁻ᵀ | a wrench in b to a wrench in a | ⟨Ad_g⁻ᵀ w, Ad_g ξ⟩ = ⟨w, ξ⟩ |
| `WrenchPullback` | Ad_gᵀ | a wrench in a to a wrench in b | inverse of the pushforward; a Jacobian transpose is a pullback |

Type sketch (DESIGN; fixes intent, not signatures):

```rust
pub struct Transform<A: Frame, B: Frame> { motor: fs_ga::Se3, _frames: PhantomData<(A, B)> }   // a_from_b
pub struct TwistIn<F: Frame>  { xi: fs_ga::Twist,  _f: PhantomData<F> }
pub struct WrenchIn<F: Frame> { w:  fs_ga::Wrench, _f: PhantomData<F> }
pub struct TwistMap<A: Frame, B: Frame>(Transform<A, B>);          // Ad_g
pub struct WrenchPushforward<A: Frame, B: Frame>(Transform<A, B>); // Ad_g⁻ᵀ
pub struct WrenchPullback<A: Frame, B: Frame>(Transform<A, B>);    // Ad_gᵀ
```

**Parity of an improper isometry** x ↦ P·x + t, PᵀP = I, det P = −1: polar vectors (position differences,
linear velocities, forces, linear momenta) map by P; axial vectors (angular velocities, torques, angular
momenta, a joint axis read as a rotation axis) map by det(P)·P = −P; a twist or wrench at the origin maps by
T(P) = diag(det(P)·P, P) applied to (angular, linear). T(P) preserves the pairing, because det(P)² = 1, and the
bracket.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-02.S1 | Ad(g·h) = Ad(g)·Ad(h), and the group coadjoint is the inverse transpose of Ad (T-04) | Certified | Polynomial identity in group coordinates (rotation as a rational Cayley image), checked by the FrankenSymPy proof kernel |
| C-02.S2 | The power pairing is invariant under a change of frame: ⟨Ad_g⁻ᵀ w, Ad_g ξ⟩ = ⟨w, ξ⟩ (T-05) | Certified | Polynomial identity |
| C-02.S3 | ⟨ad_ξᵀ μ, η⟩ = ⟨μ, [ξ, η]⟩ (T-16, first half) | Certified | Polynomial identity in eighteen variables |
| C-02.S4 | T(P) preserves the bracket and the pairing (T-16, second half) | Certified | Polynomial identities with P written as a fixed reflection times a rational rotation |
| C-02.S5 | Pushforward composed with pullback is the identity; d/dt Ad_{exp(tξ)}⁻ᵀ at 0 equals −ad*_ξ | Tested | Property test; the generator by exact differentiation of the rational Cayley curve |
| C-02.S6 | A loop of chart changes returns to the identity inside its stated error budget | Tested | Property test over the boundary set |
| C-02.S7 | Composition, addition and frame moves type-check only for matching frames | Tested | Compile-fail tests |
| C-02.S8 | Frame typing has zero run-time cost in batched kernels (PO-MARROW-02) | Tested | Emitted kernel source contains no frame identifiers |

**Sample set (normative for every law of this construction).** Random elements plus a boundary set: the
identity; tiny angles (down to the smallest subnormal-free magnitude); translations near the origin and large
translations; rotations approaching π from both sides; rotations by exactly π; sign-equivalent quaternions (q
and −q). For exact tests the rotations are rational points of SO(3) obtained from the Cayley map of rational
vectors.

**The three-fixture rule.** A single reversed adjoint (Ad_g replaced by Ad_g⁻¹, or by Ad_gᵀ, in one place) must
fail at least three independent fixtures: the composition law S1, the pairing S2, and the planar two-link
fixture of construction C-04 (link lengths 0.4 m and 0.3 m, angles 0 and π/2, tool force (10, 0) N: joint
torques (−3, −3) N·m and −0.3 W by both power computations). A convention that one test guards is one refactor
away from being wrong.

**Hand checks carried in the contract.**

- With R = I and p = (1, 0, 0) m, the wrench (0, (0, 0, 1) N) in b pushes forward to torque
  p × f = (0, −1, 0) N·m and the same force. Moving it with the twist map gives zero torque: the moment arm is
  lost.
- Free body with rotational inertia diag(1, 2, 3) kg·m² and ω = (1, 1, 0) rad/s, so π = (1, 2, 0). The correct
  law gives π̇ = π × ω = (0, 0, −1). The reversed sign gives (0, 0, +1). Both conserve kinetic energy, because
  ω·π̇ = 0 either way; only the correct one conserves spatial angular momentum (the rate π̇ + ω × π is
  (0, 0, 0) with the correct sign and (0, 0, 2) with the wrong one). An energy-only test suite passes a
  simulator with this defect.

**Charts, failure cases, retirement.** A logarithm or a derivative of a logarithm requested within a stated
margin of a rotation by π, where the chart is cut, returns refusal FROB-REF-213 (a logarithm or its derivative
inside the stated margin of a chart cut, with no second chart supplied; exit 3; the refusal carries the element,
the margin and the charts that cover it; ranked repairs as the registry N13-REF has them: 1. supply the covering
chart named in the reply (the chart based at the rotation by π about the axis of the largest quaternion component
keeps the angle at most 2π/3); 2. reformulate the query on the group element instead of its logarithm; 3. if the
request came from a derivative path, withdraw that chart pair from the derivative path and refuse the query) and
never a value from the wrong side of the cut. Retirement rule: when a loop of chart changes exceeds its error budget, that chart pair is withdrawn
from the derivative path and the query is refused. Budget (DESIGN, TARGET, set by this bead because the
known-answer row prints none): 128 ulp in `f64` per component, scaled by max(1, ‖x‖), for the two-chart loop
of SO(3) and SE(3) outside the cut margin; margin 10⁻³ rad.

**What it does not claim.** Typing prevents frame mismatches; it does not prove a transform's numerical value.
Identities S1 to S4 are certified over ℚ; floating-point evaluation is tested to tolerance and nothing more.

**Implementation map.** Crate `frob-lie`. Verb through which an agent sees it: `prove.kernel` (the instance
certificate of the adjoint kernel, cards T-04 and T-05) and every verb that prints a frame-typed quantity.
Gates: GATE-MARROW-01 (all laws on 10⁶ cases each, at most 64 ulp in `f64` and 16 ulp in `f32` for single
operations; T-01, T-03, T-04 certified; TARGET; M0); GATE-LEMMA-00 (T-04 certified by FrankenSymPy for the M0
kernels, certificates replay offline; M0); GATE-MARROW-04 (each of eight planted single-sign or
single-transpose mutations caught by at least three fixtures; M4).

## Acceptance criteria

- [ ] `docs/math/C-02.md` exists with all formulas above, the eight statements and their classes, the sample
      set, the three-fixture rule, both hand checks and the chart rules.
- [ ] `cargo test -p frob-lie --test laws_c02` passes on the exact reference and on `frob-lie` over `fs-ga`.
- [ ] S1 to S4 hold exactly on 10³ rational samples; on `f64` within 64 ulp and on `f32` within 16 ulp over
      10⁶ cases including the boundary set.
- [ ] The mutant "one reversed adjoint" fails S1, S2 and the two-link fixture; the run records all three.
- [ ] Each of the five §13.8 mutations of this construction listed under the controls below is registered and
      fails at least three of this suite's fixtures in this run, and the log names them (for example, reversed
      ad*: S3, the free-body hand check and S5's generator sign). The count across all eight mutations and all
      constructions, which GATE-MARROW-04 reads, is the mutation harness N13-18; this suite registers its
      fixtures there.
- [ ] Five compile-fail tests exist: composing mismatched frames; adding twists of different frames; passing a
      `TwistMap` where a `WrenchPushforward` is required; passing a pullback where a pushforward is required;
      constructing a `Transform` from a reflection.
- [ ] The chart loop stays inside 128 ulp outside the margin, and a query inside the margin returns
      FROB-REF-213 with the element and the margin.
- [ ] KA-040, KA-041, KA-042 are in the registry with tolerances and controls set.
- [ ] The register row of C-02 is filled.

## Test plan

### Unit tests

- `s1_ad_functor_exact` / `s1_ad_functor_float`: Ad(g·h) against Ad(g)·Ad(h); exact on rationals; ulp bounds
  on floats over the boundary set.
- `s1_coadjoint_is_inverse_transpose`: Ad_g⁻ᵀ·Ad_gᵀ = I exactly.
- `s2_pairing_invariant`: ⟨Ad_g⁻ᵀ w, Ad_g ξ⟩ − ⟨w, ξ⟩ = 0 exactly; hand case p = (1, 0, 0), f = (0, 0, 1) gives
  torque (0, −1, 0).
- `s3_coad_dual`: ⟨ad_ξᵀ μ, η⟩ − ⟨μ, [ξ, η]⟩ = 0; free-body case gives π̇ = (0, 0, −1).
- `s4_parity_preserves`: P = diag(1, −1, 1); T(P)[ξ, η] = [T(P)ξ, T(P)η]; pairing unchanged.
- `s5_generator_sign`: the derivative of the group coadjoint along a rational curve equals −ad*_ξ.
- `s6_chart_loop`: 10⁵ elements; loop error ≤ budget; inside the margin the refusal is returned.
- `s7_compile_fail_*`: the five cases above (trybuild-style, real compiler).
- `s8_no_frames_in_kernels`: grep of emitted kernel sources for frame identifiers returns nothing.

### End-to-end script and logging

`scripts/e2e/spec_c02.sh`. Sources the STD-E2E helper. Steps: law suite; law suite under each control, one
control per build with its own feature (N10a-01 section 6); the rows through the helper's `e2e_tier`, that is
`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g0_laws -- KA-040` and `-- KA-041`, and `--test
g3_metamorphic -- KA-042` (KA-042 is a G3 row); `frob --json prove kernel K-ad-se3` (BD-021's M0 adjoint kernel,
whose T-04 certificate CARD-T-04 states; logged `pending` until BD-021 lands) and `frob --json prove verify` on
its certificate, asserting class `Certified`, the certificate hash and the checker identity in the evidence block.
NDJSON log at `target/e2e-logs/spec_c02/<utc>.ndjson` plus transcript, with per-assert measured ulp errors
against 64 and 16, the three fixtures that caught the reversed adjoint, digests of the certificates, and on
failure the first differing matrix entry with the sample that produced it. Final `summary` line.

### Known answers and negative controls

- KA-040 (functor law): tolerance set to exact over ℚ, 64 ulp `f64`, 16 ulp `f32`; G0 · host. Control: a single
  reversed adjoint, caught by three fixtures.
- KA-041 (frame invariance of power): same tolerances. Control: a wrench moved by the twist map.
- KA-042 (chart loop): budget 128 ulp (TARGET); G3 · host. Control (set here): the chart-cut margin check
  removed, so a loop through the cut returns a value from the wrong side.
- CX-020: one adjoint for every quantity.
- Mutations of this construction among the eight of GATE-MARROW-04: reversed ad*; Ad* and its inverse
  exchanged; a pullback in place of a pushforward; linear and angular parts exchanged in a twist; the
  bracket's arguments exchanged. Each must fail at least three fixtures.
- Metamorphic relation: re-expressing every quantity in another body frame leaves powers and energies
  unchanged.
- Determinism: exact tests are D0; float tests run twice, across worker counts, debug and release, Linux and
  macOS, identical under deterministic elementary functions.

## Considerations

- OPEN-MARROW-1 (whether to canonicalize quaternion sign in stored state): default no; state stays continuous
  and observations use sign-invariant features. The sample set therefore must include q and −q. Decided at
  GATE-MARROW-01.
- The quaternion component order and sign rule are FrankenSim's and are confirmed by the bit-comparison
  fixture of 1,000 transforms; if that fixture disagrees, this contract's formulas stand and the adapter
  changes.
- Risk: frame typing creeps into kernels as run-time tags. S8 is the guard.
- Performance: none at run time; compile-time cost only.

## References

Plan §10 C-02; §11.1 to §11.4; §13.2 (ADR-MARROW-01, -05, -09), §13.3, §13.8, §13.10 (PO-MARROW-01, -02);
§12.3 (T-04, T-05, T-16); Appendix D.1; Appendix I.1; Appendix M KA-040 to KA-042; Appendix N CX-020.
