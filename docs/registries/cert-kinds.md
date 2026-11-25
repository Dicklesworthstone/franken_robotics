# FrankenRobotics Certificate Proof-Kind Registry

- **Registry version:** `frob.cert.kinds/1`
- **Registered proof kinds:** `27`
- **Enabled checkers in this build:** `0`
- **Determinism class:** `D0`
- **Authority:** Generated directly from `crates/frob-cert/src/kinds/registry.rs` (`cargo run -p frob-cert --example kind_registry -- --doc`). Do not edit by hand.

## 1. Proof Kinds

| Kind | Version | Cap | Tier | Scope | Status | Checker | Side Conditions | What the auditor recomputes |
|---|---:|---|---|---|---|---|---|---|
| `poly-identity` | 1 | Certified | S | kernel, analysis | `not-implemented` | BD-027 | - | Normal-form equality of two polynomial expressions over Q |
| `rational-ldlt` | 1 | Certified | S | kernel, analysis | `not-implemented` | BD-027 | - | Exact LDL^T with positive pivots |
| `interval-box` | 1 | Certified | F | mission | `not-implemented` | BD-132 | - | Outward-rounded enclosure of a constraint over a box (split set at most six parameters) |
| `interval-root` | 1 | Certified | F | kernel, analysis | `not-implemented` | N12-16 | - | Existence and uniqueness of a root in a box |
| `taylor-box` | 1 | Certified | F | mission | `not-implemented` | N12-16 | - | The enclosure of a constraint over a box by a Taylor model: remainder bound and range |
| `sos-gram` | 1 | Certified | F | mission | `not-implemented` | N12-16 | - | Nonnegativity of a polynomial on a box: exact polynomial identity of a rational Gram matrix and multipliers, and PSD by LDL^T over Q |
| `lp-farkas` | 1 | Certified | S | mission | `not-implemented` | BD-132 | - | M^T y in the dual cone and r^T y < 0 in exact rationals (box form: y^T w > sum_i max(0, (A^T y)_i) * c_i) |
| `conic-farkas` | 1 | Certified | F | mission | `not-implemented` | N12-16 | - | M^T y in the dual second-order cone by exact squaring; strict separation |
| `lp-primal` | 1 | Certified | S | mission | `not-implemented` | BD-132 | - | An explicit static witness against the primitive equations at every vertex, with the convexity or monotonicity argument named |
| `finite-game` | 1 | Certified | S | mission | `not-implemented` | BD-132 | - | A policy table against every world and error vertex; equal histories receive equal commands |
| `trace-witness` | 1 | Tested | S | mission | `not-implemented` | BD-132 | - | Format and digest of a replayed execution |
| `partition` | 1 | Certified | S | mission | `not-implemented` | BD-132 | cell-certificates | That the listed cells cover the claimed region, interiors disjoint, and that each carries its own certificate |
| `monotone-transport` | 1 | Certified | F | mission | `not-implemented` | N12-14 | source-certificate | A recorded monotonicity or polarity implication between two cells or two assumption sets |
| `symmetry-transport` | 1 | Certified | F | mission | `not-implemented` | N12-14 | source-certificate | The exact images of a verdict under a signed permutation, with invariance of the mission, world set, and observation map |
| `composition` | 1 | Certified | F | mission | `not-implemented` | N12-14 | stage-certificates | Each end-set subset entry-set inclusion of a chain of stage contracts; no clock or resource spent twice |
| `monomial-product` | 1 | Certified | F | mission | `not-implemented` | N12-14 | - | The monomial screen in exponent coordinates, with a cell's box bounds entered as constraints |
| `complex-connectivity` | 1 | Certified | S | kernel, analysis | `not-implemented` | N12-15 | - | A path inside the inner complex, or component labels of the outer complex |
| `tube-inclusion` | 1 | Certified | M | kernel, analysis | `not-implemented` | N12-19 | - | The tube inclusion for every cell and enabled guard of a table |
| `rewrite-trace` | 1 | Certified | F | kernel, analysis | `not-implemented` | N12p2-02 | - | Replay of a class-E rule trace from source to target |
| `orbit-count` | 1 | Certified | S | kernel, analysis | `not-implemented` | N12-18 | - | Orbit representatives and counts |
| `group-enumeration` | 1 | Certified | F | kernel, analysis | `not-implemented` | N12-18 | - | The subgroup lattice of a small group |
| `energy-budget` | 1 | Certified, capped by inputs | S | kernel, analysis, mission | `not-implemented` | N12-17 | - | The energy-budget inequality with its error bounds and its list of sources |
| `alias-pair` | 1 | Certified | S | mission | `not-implemented` | BD-132 | - | Two exact replays with identical packet histories and disjoint success sets |
| `product-unreachable` | 1 | Certified | F | mission | `not-implemented` | N12-17 | - | Exact case analysis or enclosure of a product model's outcome pairs |
| `alpha-root` | 1 | Certified | M | kernel, analysis | `not-implemented` | N12p1-01 | smale-assumption | The alpha bound of a rational point, with Smale's theorem as a named assumption |
| `lean-capsule` | 1 | Proved | F | kernel, analysis | `not-implemented` | BD-381 | sealed-capsule | The hash of a sealed FrankenLean capsule and the statement it proves; the capsule is checked by fln |
| `sample-coverage` | 1 | Tested | S | mission, study | `not-implemented` | N12-13 | - | The arithmetic of a recorded study (counts, e-values, spent alpha). Never a proof |

## 2. Closed Claim-Kind Scope Table

Every bundle's `claim.kind` must appear in this closed table; any unlisted `claim.kind` is rejected with `kind.out-of-scope` (`FROB-REF-823`).

| `claim.kind` | Scope | Meaning |
|---|---|---|
| `kernel-identity` | `kernel` | Exact algebraic or numeric kernel identity certificate (BD-027, N12-20) |
| `family-inclusion` | `mission` | Mission design-family inclusion certificate |
| `family-exclusion` | `mission` | Mission design-family exclusion certificate |
| `partition` | `mission` | Mission parameter-box partition certificate |
| `qualification-receipt` | `study` | Empirical study / qualification receipt (Tested cap, sample-coverage) |
| `analysis` | `analysis` | General model or mechanism analysis certificate |
| `card-instance` | `analysis` | Theorem-card instance certificate (e.g., T-19 inertia, T-38) |
| `law-convexity` | `analysis` | Per-law contact/joint potential convexity certificate (LawConvexity, N12-51 / T-08) |
| `energy-budget` | `analysis` | Mechanism energy-budget analysis certificate (N12-17 / T-28) |
| `theorem-card` | `analysis` | Sealed theorem-card proof certificate (e.g., lean-capsule, BD-381) |

## 3. Closed Rule Identifier Catalogue

Every rejection names a `RuleId` from this closed catalogue (`<area>.<rule>`).

| `RuleId` | Area | Refusal Code |
|---|---|---|
| `input.unreadable` | `input` | - |
| `json.syntax` | `json` | - |
| `json.bom` | `json` | - |
| `json.duplicate-key` | `json` | - |
| `json.key-order` | `json` | - |
| `json.whitespace` | `json` | - |
| `json.escape` | `json` | - |
| `schema.unknown` | `schema` | - |
| `schema.missing-field` | `schema` | - |
| `schema.unknown-field` | `schema` | - |
| `schema.field-type` | `schema` | - |
| `schema.review-missing` | `schema` | - |
| `hash.mismatch` | `hash` | - |
| `hash.equations-digest` | `hash` | - |
| `hash.merkle-root` | `hash` | - |
| `hash.merkle-duplicate` | `hash` | - |
| `kind.unknown` | `kind` | FROB-REF-823 |
| `kind.version-unknown` | `kind` | FROB-REF-823 |
| `kind.out-of-scope` | `kind` | FROB-REF-823 |
| `kind.disabled` | `kind` | FROB-REF-823 |
| `query.mismatch` | `query` | FROB-REF-826 |
| `query.digest-mismatch` | `query` | FROB-REF-826 |
| `query.prefix` | `query` | FROB-REF-826 |
| `query.domain` | `query` | FROB-REF-826 |
| `query.assumptions` | `query` | FROB-REF-826 |
| `query.policy-class` | `query` | FROB-REF-826 |
| `side.missing` | `side` | FROB-REF-829 |
| `side.equations-missing` | `side` | FROB-REF-829 |
| `side.condition-missing` | `side` | FROB-REF-829 |
| `side.condition-rejected` | `side` | FROB-REF-829 |
| `field.type` | `field` | FROB-REF-829 |
| `field.units-mismatch` | `field` | FROB-REF-829 |
| `field.frame-mismatch` | `field` | FROB-REF-829 |
| `field.port-order` | `field` | FROB-REF-829 |
| `claim.minimum-unbounded` | `claim` | - |
| `claim.mass-without-weighting` | `claim` | - |
| `num.float-in-bundle` | `num` | FROB-REF-849 |
| `num.int-range` | `num` | FROB-REF-849 |
| `num.int-form` | `num` | FROB-REF-849 |
| `num.rational-form` | `num` | FROB-REF-849 |
| `num.interval-decl` | `num` | FROB-REF-849 |
| `limits.exceeded` | `limits` | - |
| `class.above-kind-cap` | `class` | FROB-REF-834 |
| `class.above-input` | `class` | FROB-REF-834 |
| `class.no-receipt` | `class` | FROB-REF-834 |
| `flags.missing` | `flags` | FROB-REF-833 |
| `flags.true` | `flags` | FROB-REF-833 |
| `poly-identity.check-failed` | `poly-identity` | - |
| `rational-ldlt.check-failed` | `rational-ldlt` | - |
| `interval-box.check-failed` | `interval-box` | - |
| `interval-root.check-failed` | `interval-root` | - |
| `taylor-box.check-failed` | `taylor-box` | - |
| `sos-gram.check-failed` | `sos-gram` | - |
| `lp-farkas.check-failed` | `lp-farkas` | - |
| `conic-farkas.check-failed` | `conic-farkas` | - |
| `lp-primal.check-failed` | `lp-primal` | - |
| `finite-game.check-failed` | `finite-game` | - |
| `trace-witness.check-failed` | `trace-witness` | - |
| `partition.check-failed` | `partition` | - |
| `monotone-transport.check-failed` | `monotone-transport` | - |
| `symmetry-transport.check-failed` | `symmetry-transport` | - |
| `composition.check-failed` | `composition` | - |
| `monomial-product.check-failed` | `monomial-product` | - |
| `complex-connectivity.check-failed` | `complex-connectivity` | - |
| `tube-inclusion.check-failed` | `tube-inclusion` | - |
| `rewrite-trace.check-failed` | `rewrite-trace` | - |
| `orbit-count.check-failed` | `orbit-count` | - |
| `group-enumeration.check-failed` | `group-enumeration` | - |
| `energy-budget.check-failed` | `energy-budget` | - |
| `alias-pair.check-failed` | `alias-pair` | - |
| `product-unreachable.check-failed` | `product-unreachable` | - |
| `alpha-root.check-failed` | `alpha-root` | - |
| `lean-capsule.check-failed` | `lean-capsule` | - |
| `sample-coverage.check-failed` | `sample-coverage` | - |
