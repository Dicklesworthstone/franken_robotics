# Crate Contract: `frob-lie`

Layer 1, Rank 1 — Compute plane — Tier Solid (`BD-010`, ADR-MARROW-01, ADR-MARROW-05).

## Purpose

`frob-lie` owns FrankenRobotics's Lie-theoretic vocabulary for rigid-body kinematics, dynamics, estimation, and control. In `BD-010` it establishes the `LieGroup` and `LieAlgebra` traits, binds `SO(3)` and `SE(3)` directly onto FrankenSim's bedrock-direct `fs-ga` representation with zero duplicate vector, quaternion, or pose structs (ADR-MARROW-01), introduces the infinitesimal coadjoint `coad` (`ad*_xi = (ad_xi)^T`, no minus sign, ADR-MARROW-05), and provides a scalar-generic reference implementation over `fs_ad::Real` for forward-AD differentiation and cross-precision verification.

## Inputs

- Host `f64` group and algebra values represented directly by `fs-ga` types: `So3`, `Se3`, `So3Tangent`, `Twist`, `Wrench`, `Quat`, `Vec3`, `Motor`, `Mat3`, and `Mat6`.
- Generic scalar inputs over any `S: fs_ad::Real` (including `f64`, `fs_ad::Dual<T, N>`, and test-only `f32` wrappers) passed as fixed-size arrays (`[S; 3]`, `[S; 4]`, `[S; 6]`) to `frob_lie::reference`.
- All quantities follow SI units: radians, metres, rad/s, m/s, N·m, N, and watts (`frob_core::qty::Power`).

## Outputs

- Group elements (`So3`, `Se3`), Lie-algebra elements (`So3Tangent`, `Twist`), dual elements (`Vec3`, `Wrench`), adjoint and coadjoint linear operators (`Mat3`, `Mat6`), and SI power pairings (`frob_core::qty::Power`).
- Fallible operations (`compose`, `Se3::inverse`, `exp`, `try_from_quat_lie`, `from_parts_lie`, `try_from_motor_lie`, `rotate_vec`, `transform_point_lie`, `pullback_wrench`) return `Result<_, LieError>` carrying the underlying `fs_ga::GaError` cause without panicking or silently renormalizing.
- `frob_lie::reference` returns generic scalar arrays matching the `fs-ga` `f64` host path within 64 ULP on valid finite inputs.

## Invariants

- **ADR-MARROW-01 (No private vector/quaternion/pose vocabulary):** `frob-lie` defines no public vector, quaternion, or pose struct of its own; `So3`, `Se3`, `Twist`, `Wrench`, `Quat`, `Vec3`, `Motor`, `Mat3`, and `Mat6` are re-exported directly from `fs-ga`.
- **Normative conventions:** `g = (R, p)` is `a_from_b` acting as `x_a = R x_b + p`; twists are `xi = (omega, v)` angular-first; wrenches are `w = (tau, f)` torque-first; `Ad_g(omega, v) = (R omega, R v + p x R omega)`; `Ad*_g = (Ad_g)^{-T}` maps `(tau, f)` to `(R tau + p x R f, R f)`; `[xi, eta] = (omega x eta_omega, omega x eta_v + v x eta_omega)`.
- **ADR-MARROW-05 (Infinitesimal coadjoint sign):** `ad*_xi = (ad_xi)^T` with no minus sign, satisfying `<ad*_xi mu, eta> = <mu, [xi, eta]>` identically: on `so(3)`, `coad_omega(mu) = mu x omega`; on `se(3)`, `coad_xi(pi, ell) = (pi x omega + ell x v, ell x omega)`.
- **Deterministic double-cover sign rule:** `frob-lie` adopts `fs-ga`'s canonical hemisphere rule verbatim (negating when `w < 0` or `w == 0` with first non-zero of `(x, y, z)` negative, then clearing `-0.0` to `+0.0`) and never defines a second rule.
- **Zero `unsafe` and zero stdout/stderr printing (L-7):** `#![forbid(unsafe_code)]`; pure compute-plane library.
- **Release mutation guard (N38b-04):** Enabling `conformance` or any negative-control feature (`twist-swap`, `ad-inverse`, `bracket-swap`, `coadjoint-inverse`) fails compilation under release builds (`not(debug_assertions)`).

## Refusals

- Non-finite inputs, degenerate norms, and unit-norm or PGA-study-condition defects above `fs_ga::UNIT_TOLERANCE` (`4e-12`, with translation-relative tolerance beyond 1 m for `Se3`) are refused as `LieError::Host(GaError)` without panicking or silent renormalization.
- Retractions `cay` and `cay_inv` return `LieError::DeferredOperation` until implemented by `BD-011`.
- Crate-level workspace refusal codes (`FROB-REF-*`) are empty at `BD-010`; domain-specific chart-cut refusal mappings arrive in `BD-011`.

## Determinism class

`D2` (per-platform bit-deterministic across debug and release builds, targeting FrankenSim's cross-ISA class via `fs_math::det` on both host `fs-ga` operations and `fs_ad::Real` reference paths).

## Evidence class of outputs

`Tested` — outputs are raw mathematical values with no certificate or proof claim attached. `PO-MARROW-01` bit-parity tests execute as G2 parity checks on host without writing to any ledger.

## Validity domain

- Finite IEEE-754 binary64 coordinates.
- Unit quaternions satisfying `| ||q||^2 - 1 | <= 4e-12` and unit PGA motors satisfying `fs-ga`'s unit/study tolerance (including translations up to `10^6` m verified in the `PO-MARROW-01` boundary set).
- Non-finite or off-manifold inputs are outside the validity domain and are refused via `LieError::Host`.

## Oracles

- **`frankensim-referee` (`fs-ga`):** Standalone capture binary `tools/oracle/marrow_fsga_capture` (linking `fs-ga`, `fs-rand`, and `fs-blake3`, never `frob-lie`) generates `tests/fixtures/oracle/marrow/po_marrow_01.frobfix` (1,000 records: 64 boundary cases + 936 Philox-seeded transforms with seed `0x4D41_5252_4F57_0001`). `tests/po_marrow_01.rs` asserts 0-ULP bit equality across all 1,000 records (`KA-256`).
- **Analytic known answers:** `KA-193` (wrench push-forward moment arm) and `KA-229` (infinitesimal coadjoint components), plus exact integer-valued adjoint/coadjoint transpose identities.
- **Unit-test conventions (`STD-TEST`):** [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- `cay`, `cay_inv`, chart-cut refusals on `log`, retraction differentials (`dexp`, `dcay` and inverses), and full group-law gates are deferred to `BD-011`.
- Additional groups (`SO(2)` in `BD-012`; `SE(2)`, `SE_2(3)`, `SE_{2+k}(3)`, products in `BD-045`; joint subgroups in `BD-046`), frame-typed wrappers (`BD-047`), reflections (`N13-02`), exact predicate classes (`N13-10`), and single-layout descriptions (`N13-12`) are out of scope for `BD-010`.
- `fs_ad::Real` is implemented upstream for `f64` and `Dual<T, N>` only; `f32` testing in `BD-010` wraps `f32` locally in test code without modifying `../frankensim`.
