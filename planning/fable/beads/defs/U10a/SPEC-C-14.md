---
key: "SPEC-C-14"
title: "frob-kir: C-14 specified — derived straight-line kernels, dimension typing, rewrite classes E and R; law tests"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "spark", "frob-kir", "frontier", "spine", "spec", "test"]
parent: "WP-05"
deps: ["BD-025", "N10a-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-018", "BD-019", "BD-053", "BD-221", "BD-371", "BD-379", "N21a-13", "SPEC-C-21"]
gates: ["GATE-SPARK-00", "GATE-SPARK-02", "GATE-LEMMA-00", "GATE-LEMMA-01"]
plan_refs: ["§10 C-14", "§9 D-7 D-8", "§21.6", "§21.7", "§12.3 T-09 T-27", "§12.4", "§35", "§38.7", "Appendix M KA-085 to KA-087", "Appendix N CX-039"]
---
## Background

Construction C-14 says what a kernel is. A kernel is the arithmetic that runs for every world at every step.
Hand-written kernels with hand-written derivatives in three languages (Rust, a CUDA dialect, a Metal dialect)
are the usual source of silent disagreement between back ends. The construction replaces them with one
derived object: a straight-line program generated from symbolic expressions, typed by physical dimension,
differentiated automatically, and bound to certificates of the identities it must satisfy.

Payoff: the same arithmetic on three back ends; adjoints for smooth kernels without extra work; no dimension
errors; certificates that travel with the code. Each kernel records its register and stack pressure, its
rewrite fuel and its expression size, and derivation is capped on all of them.

Doctrine rules served. D-7 (derive, don't hand-write): the arithmetic of a kernel comes from a symbolic
derivation with a content hash; hand-written arithmetic is allowed in the referee tier and in kernel
skeletons (loops, gathers, sorts), not in the mathematical core of a batched kernel. D-8 (every domain ships
its adjoint or its excuse): a kernel either has a derivative program or a recorded reason it does not, with
the estimator that replaces it.

Tier: Frontier. Pilot: N21a-13 (record `PILOT-C-14`: the M0 kernels derived, emitted on three back ends,
certified and timed). Kill gate named in advance, KILL-KIR: trigger, at M2 the derivation toolchain cannot
express or emit more than 20% of the kernel arithmetic, or emitted kernels miss the speed bound of the
algebra-pays-for-itself bet BET-01 after one optimization pass (an emitted kernel more than 1.3× slower than a
hand-tuned kernel for the same stage on the same backend); fallback, arithmetic is hand-written in Rust and the
dialect and bound by differential tests, and certificates fall back with it. The measurement is BD-221's.

Foundations (FACT, FrankenSymPy `29f847c1`, 2026-11-21; existence is not admission): `Expr`,
`fsym_calculus::diff`, `Matrix::jacobian`, `simplify`, a proof kernel with `Claim::AlgebraicIdentity` and a
`PolynomialNormalForm` rule, and an offline verifier (`fsym-capsule-consumer`). It has no multi-output code
generation with common-subexpression elimination and no GPU emitters; its bytecode interpreter caps an
expression at 8,192 operations. FrankenRobotics therefore owns the straight-line representation and its
emitters (`frob-kir`) and uses FrankenSymPy at build time as the derivation engine and certificate checker.

## Scope

In: `docs/math/C-14.md`; the law suite `crates/frob-kir/tests/laws_c14.rs`; the rewrite-class table and its
trace format as mathematics; the dimension-typing rules; the adjoint-duality law; the `f32` reassociation
fixture; the register row; tolerances and controls for KA-085 to KA-087; the statements of T-09 and T-27.

Out: the straight-line representation, emitters and dimension checker as code (BD-018); the derivation
workspace `derive/frob-derive` (BD-019); predicate classes and the select lint (BD-053); certificate coverage for derived kernels
(BD-379); survivor specialization (BD-371); the measurement against the kill gate (BD-221); the pilot
(N21a-13).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-018 (`frob-kir` v0, which
implements this construction and depends on this bead): that bead is listed in `related` and is not a
prerequisite. The law suite is generic over its subject trait and closes on the exact reference model and the
mutants in its own test support. If `frob-kir` does not exist when this bead starts, the suite and its support
files are written under `crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025)
and the commands below read `-p frob-conformance`; the bead that implements the construction moves them into the
owning crate, binds its production types to the subject trait and runs the unchanged suite as part of its own
acceptance. A criterion or script step below that names production code or a verb of an implementing bead is owed
at that bead's acceptance and is logged `pending` here until then.

## Technical approach

**The object.**

- *Straight-line program (SLP).* A kernel's arithmetic is a straight-line program over +, −, ×, ÷, √ and
  selects, generated from symbolic expressions with common-subexpression elimination. Loops with constant
  bounds live in the skeleton around it.
- *Dimension typing.* Every value carries its physical dimension as an exponent vector over the base
  dimensions of the units library (`fs-qty`). A program type-checks only if every sum is homogeneous: physical
  quantities form a graded algebra and the kernel respects the grading. Rules: for a sum or difference the
  operands' exponent vectors are equal and the result has the same vector; a product adds vectors; a quotient
  subtracts; a square root halves the vector and is taken only on an even exponent vector; a select requires
  equal vectors on both arms and a dimensionless-comparison predicate (both sides of the comparison of equal
  dimension); a literal constant is dimensionless unless declared with a dimension.
- *Derivative programs.* A forward product J·v and a transposed product Jᵀ·w come from automatic
  differentiation of the SLP.
- *Certificates.* Identities the kernel must satisfy are polynomial identities, checked by FrankenSymPy's
  proof kernel and bound to the kernel's content hash. A square root is handled by a witness variable t with
  the side relation t² = radicand.
- *Rewrite classes.* A rewrite of an SLP belongs to one of two classes.
  - Class E changes no bit under round-to-nearest with contraction off: commutation of + and ×,
    common-subexpression elimination, dead-code elimination, scaling by a power of two, scheduling
    (reordering independent statements).
  - Class R is equal over the reals only: reassociation, distribution, fused multiply-add.
  - A class-E rewrite ships a rule trace that an independent checker replays (theorem card T-27). A class-R
    rewrite makes a different kernel with a new identity, an error bound and a fidelity run.

**The closed class-E rule table** (DESIGN: the table the trace checker accepts; any other rule name is
rejected).

| Rule | Rewrite | Why it changes no bit |
|---|---|---|
| `comm-add`, `comm-mul` | a + b ↔ b + a; a × b ↔ b × a | IEEE 754 addition and multiplication are commutative |
| `cse` | two statements with identical operator and operands are merged | same inputs, same rounding |
| `dce` | a statement whose result is unused is removed | no observable value changes |
| `pow2-scale` | x × 2ᵏ ↔ the same value with exponent shifted, absent overflow and underflow | scaling by a power of two is exact in range |
| `schedule` | reordering of statements that do not depend on each other | no operand changes |

Scaling is exact only away from overflow and underflow; the rule carries the input ranges that guarantee it,
and the checker verifies the range condition.

**Trace format** (DESIGN). A trace is a list of steps `{rule, site, before, after}` with `site` a statement
index, and `before`/`after` the statement text in canonical form. The independent checker (i) starts from the
source program's hash, (ii) applies each step, verifying that `before` matches and that the rule's pattern
and side conditions hold, (iii) ends at the target program's hash. It shares no code with the rewriter.

**Honest limits.**

1. Derivation is kept for correctness, certificates and CPU derivative kernels. As a GPU throughput lever it
   is not claimed. FACT (source register, to be re-read from the primary source before a gate cites it): the
   Lie-algebraic recursions are about 16 to 20% of an incumbent's step, which caps specialization near 1.2×
   (1/(1 − 0.18) = 1.22), so per-design specialization is for survivors only and only if this project's own
   profile shows a larger share.
2. No sine, cosine or fused multiply-add appears in a batched kernel.
3. Certificates are over ℚ with square roots as witnesses; rounding is not modelled. A kernel's floating-point
   output satisfying its identity is a residual test with dimensioned tolerances, never a certificate.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-14.S1 | Every kernel is dimensionally homogeneous, and every square root is taken on an even exponent vector (T-09) | Certified | Decided exactly by the KIR type checker |
| C-14.S2 | Emitted Rust agrees with the derivation engine's own evaluation and with dual-number evaluation; emitted GPU source agrees with emitted Rust on each back end | Tested | Differential tests; the device comparison is the fidelity gate of the kernel back ends |
| C-14.S3 | Adjoint duality for every derivative pair: ⟨J·v, w⟩ = ⟨v, Jᵀ·w⟩ | Tested; Certified where the pair is polynomial | Random v and w; polynomial identity |
| C-14.S4 | A class-E rewrite trace maps the source program to the target program using only rules of the closed table (T-27) | Certified | Trace replay by an independent checker |
| C-14.S5 | A class-E rewrite changes no bit of any output | Tested | Bit comparison on 10⁶ random inputs per kernel |
| C-14.S6 | A reassociation filed as class E is rejected | Certified | The trace checker has no such rule |
| C-14.S7 | Each identity declared in a kernel's manifest has a certificate bound to the kernel's content hash; a changed coefficient breaks the binding | Certified | Proof-kernel check; offline replay |
| C-14.S8 | Derivation respects its caps: register and stack pressure, rewrite fuel, expression size (at most 8,192 operations per expression passed to the interpreter) | Tested | Planted over-size expression |
| C-14.S9 | Every kernel has a derivative program or a recorded excuse with its replacing estimator | Tested | Manifest check |

**Fixtures** (exact; hand-checkable).

- *Reassociation in single precision.* a = 10⁸, b = −10⁸, c = 1 in `f32`: (a + b) + c = 0 + 1 = 1, and
  a + (b + c) = 10⁸ + (−10⁸ + 1) = 10⁸ + (−10⁸) = 0, because 10⁸ − 1 rounds to 10⁸ (the spacing of `f32` near
  10⁸ is 8). A trace that files this reassociation as class E must be rejected.
- *Dimension violation.* The statement x = p + v·h·h with p a length, v a velocity and h a time: v·h·h has
  dimension L·T, not L; rejected. The corrected statement p + v·h type-checks.
- *Odd square root.* √(a) with a an area type-checks (L² halves to L); √(ℓ) with ℓ a length is rejected.
- *Adjoint pair.* The kernel y = R(q)·x for the Cayley rotation R of a rational q: J·v with respect to x is
  R·v and Jᵀ·w is Rᵀ·w, and ⟨R·v, w⟩ = ⟨v, Rᵀ·w⟩ exactly in rationals.
- *Power-of-two scaling.* The rewrite (x × 2⁻¹) × 2 → x on `f32`: bit-equal for every x whose intermediate
  x × 2⁻¹ is normal. At x = (1 + 2⁻²³)·2⁻¹²⁶, the smallest normal number plus one unit in the last place, the
  intermediate is subnormal and loses its last bit, so the round trip returns 2⁻¹²⁶ and not x: the range
  condition fails there and the checker must refuse the step. (x × 0.5 against x ÷ 2 is not a fixture of this
  rule: both are the correctly rounded value of x/2 and agree on every input, subnormals included.)

**What it does not claim.** Throughput beyond the ceiling above; equivalence of class-R rewrites;
correctness of skeleton code (loops, gathers, sorts), which is tested by parity and determinism gates;
anything about floating-point outputs beyond residual tests.

**Implementation map.** Crate `frob-kir` (representation, type checker, emitters, trace checker); derivation
runs in the separate derivation workspace at build time. Verbs: `prove.kernel` (the instance certificate of a
kernel and what it states), `prove.verify` (offline re-check), `spark.check` (device against CPU). Gates:
GATE-SPARK-00 (TARGET, M0: one dialect source for `K-cay-step` compiles and runs on CUDA and on Metal and
matches the Rust reference inside its class); GATE-LEMMA-00 (M0: T-01, T-03 and T-04 certified by FrankenSymPy
for the M0 kernels; certificates replay offline); GATE-SPARK-02 (M2: every M2 kernel and the full step agree
between `batch-cpu` in `f32` and each device inside the declared class: bit-equal for class D1; for class D3
at most 4 ulp per kernel output, statistical equivalence of 1,000-step rollouts, and the decision-level
test); GATE-LEMMA-01 (M5: at least 80% of derived kernels carry a replayable instance certificate for each
identity their manifest declares). Cards: T-09, T-27. Invariant served: generated code is never edited by
hand; CI regenerates and compares digests.

## Acceptance criteria

- [ ] `docs/math/C-14.md` exists with the object, the typing rules, the class-E table, the trace format, the
      three limits, the nine statements and the fixtures.
- [ ] `cargo test -p frob-kir --test laws_c14` passes on the reference checker and on `frob-kir`.
- [ ] The seeded dimension violation and the odd square root are rejected by the type checker with the
      statement index and both exponent vectors in the message.
- [ ] The `f32` reassociation fixture evaluates to 1 and 0, and its trace filed as class E is rejected.
- [ ] A class-E trace of at least twenty steps over `K-cay-step` replays; the outputs before and after are
      bit-equal on 10⁶ inputs.
- [ ] Adjoint duality holds exactly on the rational fixture and within 64 ulp of the sum of absolute terms in
      `f64` (10⁻⁵ relative in `f32`) on random inputs.
- [ ] A certificate with one coefficient changed, and a kernel with one coefficient changed under an
      unchanged certificate, are both rejected.
- [ ] KA-085 to KA-087 registered with tolerances and controls set; the register row of C-14 filled.

## Test plan

### Unit tests

- `s1_dimension_rules`: one positive and one negative program per rule (sum, product, quotient, root, select,
  literal).
- `s1_seeded_violation`, `s1_odd_root`: the fixtures.
- `s2_emitted_rust_vs_symbolic`: `K-cay-step` and `K-normalize` on 10⁴ rational inputs evaluated exactly by
  the derivation engine and by the emitted Rust in `f64`; agreement within the kernel's stated bound.
- `s3_adjoint_duality_exact`, `s3_adjoint_duality_float`: the pair fixture and every derivative pair on file.
- `s4_trace_replay`: a valid trace; `s4_unknown_rule_rejected`; `s4_side_condition_rejected` (a `pow2-scale`
  step whose range condition fails).
- `s5_class_e_bit_equal`: before and after programs, 10⁶ inputs, `f32` and `f64`.
- `s6_reassociation_rejected`: the a, b, c fixture.
- `s7_certificate_binding`: the two mutations.
- `s8_caps`: an expression of 8,193 operations is refused with the cap named.
- `s9_adjoint_or_excuse`: a manifest entry with neither a derivative program nor an excuse fails.

### End-to-end script and logging

`scripts/e2e/spec_c14.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G0 and G2 rows
KA-085 to KA-087; `frob --json prove kernel K-cay-step` (asserting the identities listed, the certificate
hash, class `Certified`, the checker identity); `frob --json prove verify <certificate>` in an environment
without the simulator; where a device is present, `frob --json spark check --kernel K-cay-step --backend
<gpu-cuda|gpu-metal> --key spec-c14-<backend>-<run>` (a `run` verb: without its idempotency key it is refused
with `FROB-REF-805`) asserting agreement inside the device's class. Log `target/e2e-logs/spec_c14/<utc>.ndjson` with transcript: commands, exit codes,
wall times, hashes of source, emitted and certificate artifacts, the evidence block (class, determinism
class, kernel identity), per-kernel maximum ulp difference against its threshold, and on failure the first
input and output word that differed. Final `summary` line.

### Known answers and negative controls

- KA-085 emitted kernels against their derivation: the fidelity gate's thresholds (bit-equal for D1; at most
  4 ulp per kernel output for D3); G2 · all tiers. Control (set here): one coefficient changed in an emitted
  kernel.
- KA-086 adjoint duality: tolerance set to 64 ulp of the sum of absolute terms in `f64` and 10⁻⁵ relative in
  `f32`; G0 · all tiers. Control (set here): a sign flipped in one statement of the transposed program.
- KA-087 rewrite control: exact; G0 · host. Control: the reassociation filed as class E.
- CX-039.
- Metamorphic relations: scaling the unit system leaves dimensionless outputs unchanged; a class-E rewrite
  changes no bit, and a specialized kernel is bit-identical to its generic kernel (mutation: a reassociation
  filed as class E).
- Determinism: the type checker and trace checker are D0; emitted kernels run twice, across worker counts,
  debug and release, and across back ends each against its class.

## Considerations

- If the kill gate fires, statements S1 and S3 still apply to hand-written kernels through the same type
  checker (run on a hand-written SLP description) and differential tests; S4 to S7 lose their subject and the
  register row records the fallback by name.
- Contraction must be off everywhere: a back end that contracts a × b + c into a fused multiply-add evaluates
  a different rounding sequence, which is a class-R change made by the compiler; the dialect lint and the
  device probe guard this, not the trace checker.
- Risk: the trace checker and the rewriter share a helper and so share a bug. S4 requires independence; the
  checker lives in its own module with its own parser of the canonical statement form.
- Performance: derivation is a build-time cost; caps make it bounded and the caps are recorded per kernel.

## References

Plan §10 C-14; §9 D-7, D-8; §21.6, §21.7; §12.3 (T-09, T-27), §12.4; §35; §38.7; Appendix M KA-085 to KA-087;
Appendix N CX-039. Bets: BET-01, BET-11. Kill gate: KILL-KIR. Pilot: N21a-13.
