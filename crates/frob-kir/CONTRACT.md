# `frob-kir` Contract

`frob-kir` is layer 1 (rank 3, tier Solid/Frontier, cap 14,000 lines with tests) of the FrankenRobotics core workspace.

## Purpose

Provide the static shared-dialect lint (`dialect_lint`, BD-013) that inspects CUDA C++ / Metal Shading Language shared-dialect kernel units and their compile-option sets before any GPU compiler sees them, rejecting banned constructs that break determinism classes D1/D2 with structured `FROB-REF-251` refusals. Downstream bead BD-018 extends this crate with the straight-line kernel IR (`frob.kir/1`), dimension checker (`FROB-REF-250`), bounded CSE, and Rust/dialect emitters.

## Inputs

- A `dialect_lint::DialectUnit` partitioning source text into:
  - optional `shape_constants` (`#define <IDENT> <integer-literal>` shape-class capacity constants),
  - optional `prelude` (backend prelude header, exempt from rule 1 but checked against rules 2, 3, 4, and 6),
  - ordered `sources` (generated kernel bodies and hand-written skeletons with logical file names).
- A `dialect_lint::CompileOptions` value naming the backend option set:
  - `CompileOptions::Cuda { options }`: NVRTC / nvcc flag list (must include `--fmad=false` and exclude `--fmad=true`, `--use_fast_math`, and fast-math relaxation flags).
  - `CompileOptions::Metal { math_mode }`: Metal math mode (`MetalMathMode::Safe` required; `MetalMathMode::Fast` refused).

## Outputs

- `Ok(LintReport)` carrying `lint_version`, `files_linted`, `rules_checked` (`[1, 2, 3, 4, 6]` in BD-013), and empty `findings` when the unit and options are clean.
- `Err(frob_core::refusal::Refusal)` with code `FROB-REF-251` (exit code 3, `retry = "after-change"`), ranked repairs naming the admitted replacement for each triggered rule, pointer to the first finding (`<file>:<line>:<column>` or `/options`), and deterministic canonical JSON findings array (`{rule, file, line, column, token, message}`) in `detail`.
- Deterministic expanded source assembly (`DialectUnit::expand`) and post-prelude BLAKE3 body digest (`DialectUnit::body_digest`) via `frob_core::id::hash_blob`.

## Invariants

- **Zero `unsafe`, zero stdout/stderr printing (L-7), zero GPU/runtime/third-party dependencies.** Dependencies are strictly `std` and `frob-core` (layer 0, rank 1).
- **Rules enforced in BD-013 (1, 2, 3, 4, 6):**
  - **Rule 1:** No backend intrinsic outside the prelude (`__global__`, `__device__`, `__shared__`, `__syncthreads`, `blockIdx`, `threadIdx`, `blockDim`, `gridDim`, `atomicAdd` and kin, `kernel`, `device`, `threadgroup`, `threadgroup_barrier`, `[[...]]` attributes, `metal::`, `atomic_*`, `__builtin_*`, `__uint_as_float` / `__float_as_uint` / `as_type`, `#pragma`, `_Pragma`, `#include` other than the prelude include, and `#define` outside the prelude and shape-class constant block; shape-class `#define` lines accept only scalar integer constant definitions).
  - **Rule 2:** No floating-point atomics; only the four order-independent 32-bit unsigned integer atomic macros (`FROB_ATOMIC_ADDW_U32`, `FROB_ATOMIC_MAX_U32`, `FROB_ATOMIC_MIN_U32`, `FROB_ATOMIC_OR_U32`) are admitted in kernel source.
  - **Rule 3:** No built-in transcendental functions (`sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `exp`, `exp2`, `exp10`, `expm1`, `log`, `log2`, `log10`, `log1p`, `pow`, `powr`, `cbrt`, `hypot`, `erf`, `tgamma` and `f`/`l` variants) and no `rsqrt` / `inversesqrt` built-in (including `fast::` and `precise::` qualified forms). `det_*` functions and `FROB_SQRT` are admitted.
  - **Rule 4:** Floating-point contraction off, no fast-math relaxations, and no explicit fused multiply-add (`fma`, `fmaf`, `fmal`, `mad`, `madf`, `__fma*`, contraction-on or fast-math `#pragma`, CUDA `--use_fast_math` / `--fmad=true` / missing `--fmad=false`, or Metal math mode other than `Safe`).
  - **Rule 6:** No grid-wide barrier and no `while` or `do` spin-wait loops; intra-group synchronization uses `FROB_GROUP_BARRIER` only.
- **Exact 1-based line and column tracking:** C/C++ line (`//`) and block (`/* ... */`) comments are stripped without shifting line or column coordinates of subsequent tokens.
- **Schema `frob.kir/1` v0 layout (`BD-018`, exported as `frob_kir::KIR_SCHEMA`):** canonical JSON object with required members `schema` (`"frob.kir/1"`), `name`, `inputs` (`[{name, ty, dim[6]}]`), `outputs` (`[{name, ty, dim[6], node}]`), `nodes` (SSA index-ordered `[{op, ty, dim[6], expr_name, port|bits_hex|args|cmp}]` over `Input`, `Const`, `Add`, `Sub`, `Mul`, `Div`, `Sqrt`, `Neg`, `Abs`, `Min`, `Max`, `Cmp`, `Select`), and `meta` (`{math_mode: "Ieee754Safe", op_counts, cpu_reference, determinism}`). Content hash is domain-separated BLAKE3 over those canonical bytes. Bounded hash-consing CSE (default cap 65,536 nodes) merges exact commutative pairs (`Add`/`Mul`/`Min`/`Max`) without reassociation, distribution, or inexact constant folding; `+0.0` and `-0.0` are distinct constants. Rust and shared-dialect emitters write strictly one operation per statement with bit-encoded constants (`f32::from_bits` / `FROB_F32_BITS`), zero decimal float literals, and build-time refusal of `F64` on the dialect emitter.
- **Not a seventh `xtask` subcommand (INV-WS-05):** The lint is a library module called by crate tests, `xtask check-generated`, and backend compile paths.
- The KIR reader checks SSA references and port types before admitting a program. It accepts `Ieee754Safe` and requires operation counts to match the nodes. The ten count keys are `add_sub`, `cmp`, `consts`, `div`, `inputs`, `min_max`, `mul`, `select`, `sqrt`, and `unary`.
- After validation, emitters number arguments and results by port order. Port names are unique across inputs and outputs and use an ASCII letter followed by letters, digits or underscores, excluding reserved words and the `FROB_` prefix.
- The `F32` interpreter represents Boolean ports as `0` or `1` and refuses `F64`. `Min`, `Max`, and `Abs` canonicalize NaN to positive quiet NaN. `Min` preserves negative zero for mixed zeros, `Max` returns positive zero, and `Abs` returns positive zero for either input zero.

## Refusals

- `FROB-REF-250` (exit 3, `after-change`): a `frob.kir/1` program is not dimensionally homogeneous (`Add`/`Sub`/`Min`/`Max`/`Cmp`/`Select` on unequal `frob_core::qty` exponent vectors, `Sqrt` on an odd exponent vector, or output port dimension mismatch; T-09, PO-SPARK-01). Names kernel, node index, source expression, and both exponent vectors.
- `FROB-REF-251` (exit 3, `after-change`): a banned operation or inadmissible compile option appears in a shared-dialect kernel unit (build time). Every finding carries `{rule, file, line, column, token, message}` in canonical JSON inside `Refusal::detail` and ranked repairs naming the admitted replacement.

## Determinism class

`D0` (Exact). `dialect_lint::lint` is a pure function of its `DialectUnit` and `CompileOptions` inputs, sorting findings deterministically by `(file, line, column, rule, token)` and producing byte-identical reports and refusals across Linux/macOS and debug/release builds.

## Evidence class of outputs

`Tested`. Lint reports are static checks of source spellings and compile options; they do not by themselves prove that a backend compiler will not contract or reorder arithmetic, which is verified dynamically by the contraction detector and IEEE probe (`BD-017`, `BD-020`).

## Validity domain

- UTF-8 shared-dialect C/CUDA/MSL source files (comments, C identifiers, numeric/string/char literals, punctuation, `#` preprocessor directives with `\` line continuations).
- Dialect prelude version 0 (`FROB_PRELUDE_VERSION = 0`).
- Compile-option descriptors for CUDA (NVRTC) and Metal (MSL 3.1 via `ft-kernel-metal` safe mode).

## Oracles

- Planted negative controls for rules 1, 2, 3, 4, and 6 in `src/dialect_lint.rs` (including the twelve rejected plants of BD-013, CX-221's `BD-013:spin-loop-source` spin-wait plant, two-defect exact line/column checks, and prelude-only diff BLAKE3 digest equality).
- Crate conformance tests in `tests/conformance.rs` checking `CONTRACT.md`, `contract.toml`, `RefusalRegistry` alignment for `FROB-REF-251`, and D0 determinism.
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.
- The compiled Cayley body comes from `tests/generated/k_cay_step.rs`. `emit_rust_matches_compiled_fixture` compares it byte for byte with the emitter, and the Cayley tests call that body. `typecheck_homogeneous_sums` catches the debug-only `skip-sum-dim-check` control.

## Known gaps

- Rules 5 (explicit bounded loops, unsigned indices, no recursion, no pointer arithmetic), 7 (exact unsigned 32-bit integer arithmetic, masked shifts, 64-bit feature gating), 8 (select-only masks, no arithmetic blends), and 9 (neutral padding constants), the Rust kernel lint, and `registries/kernel_bans.toml` token-table loading land in N21a-10.
- Wiring `dialect_lint::lint` into CUDA and Metal compile paths and exposing structured `FROB-REF-251` lint reports from `frob --json` on refused compiles belong to BD-015, BD-016, and N21a-10 (BD-013 has no compile path or `frob` verb, per polishing-round-4 review notes 1 and 2).
- Bounded loops, composite types, loads/stores, and `frob schema frob.kir/1 --json` land in N21a-16; unsigned integer ops in N21a-17; resource metadata in N21a-18; symbolic derivation in BD-019.
