# `frob-kernels` Contract

`frob-kernels` is layer 3 (rank 1, tier Solid, cap 26,000 lines of Rust with tests; non-Rust dialect library cap 20,000 lines) of the FrankenRobotics core workspace.

## Purpose

Hold the shared C-dialect kernel data (`dialect/prelude_cuda.h`, `dialect/prelude_metal.h`, `dialect/skeletons/*.h`, `dialect/generated/*`), dialect assembly helpers, and (in downstream beads `N21b-01` and `BD-014`) the kernel manifest `frob.kernel/1`, `KernelBackend` trait, and `batch-cpu` reference kernels. Dialect text is plain data in the core workspace, which never links a GPU driver or GPU annex crate.

## Inputs

- Shared-dialect kernel skeletons (`dialect/skeletons/*.h`) and generated arithmetic bodies (`dialect/generated/*_body.h`).
- Shape-class capacity `#define` blocks (`#define <IDENT> <integer-literal>`).
- Target backend selector (`BackendTarget::Cuda` or `BackendTarget::Metal`) and compile-option sets (`frob_kir::dialect_lint::CompileOptions`).

## Outputs

- Expanded backend-ready source units combining shape-class constants, the selected version-0 prelude (`FROB_PRELUDE_VERSION = 0`), the canonical boundary marker `/* === FROB_PRELUDE_END === */`, and shared kernel source bytes.
- Deterministic post-prelude BLAKE3 body digests (`frob_core::id::Blake3`) guaranteeing that the CUDA and Metal compilers receive byte-identical kernel text after the prelude.
- Dialect lint verdicts (`Ok(LintReport)` or `Err(Refusal)` with `FROB-REF-251`) produced by `frob_kir::dialect_lint::lint`.

## Invariants

- **Zero `unsafe`, zero GPU crate dependencies, zero stdout/stderr printing (L-7).** Dependencies are strictly `std`, `frob-core` (layer 0), and `frob-kir` (layer 1).
- **Prelude version 0 (`FROB_PRELUDE_VERSION = 0`):** Enters every kernel-cache key and hardware fingerprint (RISK-48). Both preludes carry no arithmetic; floating constants in dialect code use `FROB_F32_BITS(b)` (bit reinterpretation, not decimal parsing).
- **Metal Shading Language 3.1 ceiling and 31-argument limit:** `prelude_metal.h` and all checked-in skeletons use no construct newer than Metal Shading Language 3.1 (`-std=metal3.1`) and bind strictly fewer than `31` buffer arguments (`MAX_ARGUMENTS = 31`, verified in `../frankentorch/crates/ft-kernel-metal/src/compute.rs`).
- **Metal contraction-off pragma (BD-013 HYPOTHESIS settled empirically):** Tested on this host with `Apple metal version 32023.921 (metalfe-32023.921.6)` (`xcrun metal -std=metal3.1 -fno-fast-math -O2 -Wall -Werror -S -emit-llvm`):
  - Without a source pragma, Metal emits `@llvm.fmuladd.f32` for single-statement `a * a - c` and folds `(1 + 2^-12)^2 - (1 + 2^-11)` to `2^-24` (`0x3E70000000000000`) even under `-fno-fast-math` (`MathMode::Safe`).
  - Both `#pragma clang fp contract(off)` and `#pragma STDC FP_CONTRACT OFF` compile with zero warnings under `-std=metal3.1 -fno-fast-math -Wall -Werror`, suppress `@llvm.fmuladd.f32`, emit separate `fmul` + `fsub`, and evaluate the detector to `0.0`.
  - Under `-ffast-math`, even with the pragma, Metal still emits `reassoc nnan ninf nsz arcp afn` and `air.compile.fast_math_enable`. Therefore `prelude_metal.h` sets BOTH `#pragma clang fp contract(off)` and `#pragma STDC FP_CONTRACT OFF` AND rule 4 requires `MetalMathMode::Safe` (`Gateway::library`, never `library_with(..., Fast)`).
- **The nine dialect rules (rules 1, 2, 3, 4, and 6 enforced in BD-013; rules 5, 7, 8, and 9 enforced in N21a-10):**
  1. No backend intrinsic outside the prelude (`__global__`, `__device__`, `__shared__`, `__syncthreads`, `blockIdx`, `threadIdx`, `blockDim`, `gridDim`, `atomicAdd` and kin, `kernel`, `device`, `threadgroup`, `threadgroup_barrier`, `[[...]]`, `metal::`, `atomic_*`, `__builtin_*`, `__uint_as_float`/`__float_as_uint`/`as_type`, `#pragma`, `_Pragma`, `#include` other than the prelude include, and `#define` outside the prelude and shape-class constant block).
  2. No floating-point atomics. Integer atomics are allowed only for order-independent 32-bit unsigned operations: `FROB_ATOMIC_ADDW_U32`, `FROB_ATOMIC_MAX_U32`, `FROB_ATOMIC_MIN_U32`, and `FROB_ATOMIC_OR_U32`.
  3. No built-in transcendental functions and no reciprocal-square-root built-in (`sin`, `cos`, ..., `rsqrt`, `inversesqrt`, `f` suffixes, `fast::` / `precise::` forms). Elementary functions are `det_*` functions written in the dialect itself; `FROB_SQRT` is admitted because IEEE 754 requires correctly rounded square root (verified by BD-017).
  4. Floating-point contraction off; no fast-math relaxations; no explicit fused multiply-add (`fma`, `fmaf`, `mad`, `__fmaf_rn` and kin; CUDA requires `--fmad=false` and no `--use_fast_math`; Metal requires `MathMode::Safe` and no contraction-on pragma).
  5. Explicit loop bounds that are literals or shape-class capacity constants, unsigned 32-bit indices, no recursion, no pointer arithmetic beyond buffer plus index (enforced in N21a-10).
  6. No grid-wide barrier and no `while`/`do` spin loops. A stage needing every world to finish before the next begins is two launches (ADR-SPARK-09).
  7. Integer arithmetic in exact kernels is unsigned 32-bit with masked shift counts; 64-bit integers and atomics only where recorded by every device family fingerprint; counts above 2^32 use two 32-bit limbs (enforced in N21a-10).
  8. A mask is a select (`FROB_SELECT`), never an arithmetic blend `m*a + (1 - m)*b` (CX-219; enforced in N21a-10).
  9. Padding trails active lanes and holds the neutral element of its consumer (`-0.0` for sums, `1.0` for products, `+inf` / `-inf` for min / max; enforced in N21a-10).
- **CX-221 FACT re-read record (for N21-CX `rerun` and `rerun_receipt`):**
  - Repository: `../frankensim` (pattern reference only; zero dependency per INT-FSIM-11).
  - Pinned commit (from `franken-stack.lock`): `d02ad9c170d5932667d15359cd362d3198ac4806`.
  - File path: `apps/instrument-lab/render-gpu-piano/src/metal.rs`.
  - Lines: `68-71`.
  - Exact verbatim quote:
    > `// Metal does not promise that all threadgroups of a dispatch run concurrently, and very rarely one is`
    > `// not scheduled while the others spin (about once per few hundred launches here). So a wait that`
    > `// lasts far longer than any phase raises the abort word, every threadgroup returns, and the host`
    > `// reruns the stretch from its saved state.`

## Refusals

- `FROB-REF-251` (exit 3, `after-change`): raised by `frob_kir::dialect_lint::lint` whenever any dialect source or compile-option set violates rules 1, 2, 3, 4, or 6.

## Determinism class

`D0` (Exact) for dialect assembly, prelude expansion, body-digest hashing, and static lint checks. Floating-point kernel execution classes (`D1` / `D2` / `D3`) are established per device family by the IEEE probe (`BD-017`) and divergence ledger (`BD-020`).

## Evidence class of outputs

`Tested`. Checking dialect cleanliness, prelude expansion equality, `FROB_F32_BITS` bit reinterpretation, and planted negative-control rejection verifies the static half of Spark's determinism discipline; device measurements belong to `BD-015`, `BD-016`, `BD-017`, and `BD-020`.

## Validity domain

- Shared-dialect headers under `crates/frob-kernels/dialect/` (`prelude_cuda.h`, `prelude_metal.h`, `skeletons/*.h`, `generated/*.h`).
- Buffer argument indices `0..31` (enforcing Metal's `MAX_ARGUMENTS = 31` ceiling).
- Single-precision IEEE 754 `f32` and unsigned 32-bit `u32` / `frob_atomic_u32` buffer types in prelude v0.

## Oracles

- Checked-in skeletons `k_selftest.h` (exercising every prelude v0 macro) and `k_probe_contract.h` (KA-526 contraction detector `(1 + 2^-12)^2 - (1 + 2^-11)` and GB-25 regrouping pair `(10^8 + -10^8) + 1` vs `10^8 + (-10^8 + 1)`), verified clean under `dialect_sources_are_clean` and byte-identical after the prelude under `prelude_expansions_differ_only_in_prelude`.
- Planted synthetic negative-control fixtures under `tests/fixtures/bad_dialect/` covering rules 1, 2, 3, 4, and 6 (including CX-221's `BD-013:spin-loop-source`) plus planted contraction compile-option fixtures (`--fmad=true`, absent `--fmad=false`, `--use_fast_math`, Metal `MathMode::Fast`).
- Apple Metal compiler (`xcrun metal -std=metal3.1 -fno-fast-math -Wall -Werror`) compile verification of `prelude_metal.h` + `k_selftest.h` and `k_probe_contract.h`.
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- Kernel manifest `frob.kernel/1` and binding/alias validation land in `N21b-01`; the `KernelBackend` trait and `batch-cpu` reference backend land in `BD-014`.
- GPU compilation and execution of expanded units (and `frob --json` structured lint reports on refused compiles) belong to `BD-015` (CUDA), `BD-016` (Metal), `BD-017` (IEEE probe), `BD-020` (GATE-SPARK-00), and `N21a-10` (per BD-013 polishing-round-4 review notes 1 and 2).
- Derived kernel bodies under `dialect/generated/` (`k_cay_step_body.h`) arrive with `BD-018` / `BD-019`; `det_*` elementary functions arrive with `N21a-11`.
