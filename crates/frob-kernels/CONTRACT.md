# `frob-kernels` Contract

`frob-kernels` is layer 3 (rank 1, tier Solid, cap 26,000 lines of Rust with tests; non-Rust dialect library cap 20,000 lines) of the FrankenRobotics core workspace.

## Purpose

Hold the shared C-dialect kernel data (`dialect/prelude_cuda.h`, `dialect/prelude_metal.h`, `dialect/skeletons/*.h`, `dialect/generated/*`), dialect assembly helpers (`BD-013`), the kernel manifest `frob.kernel/1` and stage signature validator (`N21b-01`), and the `KernelBackend` seam, device launch `Receipt` (`AE-01`..`AE-12`), deterministic `BatchCpu` reference backend, uniform single-kernel `Harness`, and `K-selftest` self-test kernel (`BD-014`). Dialect text is plain data in the core workspace, which never links a GPU driver or GPU annex crate (`INV-SPARK-01`).

## Inputs

- Shared-dialect kernel skeletons (`dialect/skeletons/*.h`) and generated arithmetic bodies (`dialect/generated/*_body.h`).
- Shape-class capacity `#define` blocks (`#define <IDENT> <integer-literal>`) and `ShapeClassCaps` descriptors.
- Target backend selector (`BackendTarget::Cuda` or `BackendTarget::Metal`) and compile-option sets (`frob_kir::dialect_lint::CompileOptions`).
- Kernel libraries (`KernelLibrary`) carrying `frob.kernel/1` manifests, multi-backend sources (`cpu_symbol` and/or `dialect_source`), optional `ProgramFootprint` KIR evidence (`PO-SPARK-04`), and optional compile-time `SlabBinding` sets.
- Host slab layouts (`BufferLayout`), host input byte slices via `KernelBackend::upload`, stage lists (`Stage`), and launch cancellation tokens (`Cx`).

## Outputs

- Expanded backend-ready source units combining shape-class constants, the selected version-0 prelude (`FROB_PRELUDE_VERSION = 0`), the canonical boundary marker `/* === FROB_PRELUDE_END === */`, and shared kernel source bytes.
- Deterministic post-prelude BLAKE3 body digests (`frob_core::id::Blake3`) guaranteeing that the CUDA and Metal compilers receive byte-identical kernel text after the prelude.
- Dialect lint verdicts (`Ok(LintReport)` or `Err(Refusal)` with `FROB-REF-251`) produced by `frob_kir::dialect_lint::lint`.
- Sealed `BufferSet` handles (256-byte-aligned host slabs on `BatchCpu`, metadata-only device handles on annexes), sealed immutable `StepPlan`s bound to one allocation, validated execution `Receipt`s carrying FrankenSim's twelve verbatim `AE-01`..`AE-12` fields, and `HarnessOutput` read-back bytes + BLAKE3 digests (withheld when cancelled before launch).

## Invariants

- **Zero `unsafe`, zero GPU crate dependencies, zero stdout/stderr printing (L-7, INV-SPARK-01).** Dependencies are strictly `std`, `frob-core` (layer 0), and `frob-kir` (layer 1).
- **Single allocation entry point (`INV-SPARK-03`) and sealed execution contract (`BD-014`):**
  - `KernelBackend` exposes no allocation method other than `allocate`; `KernelArgs` exposes no allocation hook (compile-fail doctest).
  - `BufferSet` and `AlignedSlab` are non-`Clone` with private descriptors; host transfers flow through `KernelBackend::upload`; output slabs are marked written only after their producer stage finishes and cannot be read back beforehand.
  - `StepPlan` and `PlannedLaunch` are sealed and bound to one backend instance and `allocation_id`; `BatchCpu::launch` validates backend tag, `allocation_id`, and every slab's `(ElemType, byte_len)` before any write occurs.
  - Cancellation (`Cx`) drains cleanly at launch boundaries (`AE-10 = "cancelled-drained"`); `Harness::run` preserves the validated receipt and withholds output read-back when `launch_count == 0`.
  - Every `launch` returns a `Receipt` whose twelve verbatim `AE-01`..`AE-12` fields pass `Receipt::validate()` (`INV-SPARK-02`). `Receipt::validate_agent_boundary` remains fail-closed at M0 until live fingerprint currency (`BD-029` / `N21c-05`) and placement attestation (`N21b-28`) are wired.
- **Kernel manifest `frob.kernel/1` and stage signature (`N21b-01`):**
  - Every `KernelManifest` carries `id` (`KernelId`, domain-separated BLAKE3 over canonical JSON without `id`, lists sorted by `ArgId`), `name`, `class` (`Float | Exact | Guarded | Stat`), `role` (`Ordinary | StrictPredicate | Digest | NegativeControl`), `args` (each requiring `elem`, `view` (`Value | Bits`), affine `shape`, 6-element `frob_core::qty` `dim`, closed 12-variant `kind`, `components`, and `state`), `determinism`, `identities`, `derivative`, `cpu_reference`, and `signature` (`reads`, `writes`, `may_alias`, `layout` with explicit `stride_unit: Bytes | Elements` and `stride_signed`, `epoch_read`, `epoch_write`, `resources`, `numerics`, `feeds_state`, `footprint_evidence`).
  - `registries/kernels.toml` (`frob.kernels/1`) records per-row provenance outside `KernelId`: required `origin` (`derived { derivation_hash, source_hash }`, `skeleton`, or `handwritten { reason, adr }`) and optional `lowers` entries (`{ arg, model, field_path, lowering_hash }`, loaded permissively so `N10a-03` D-1 reports missing lowerings).
  - **Seven static validation rules:** (1) every argument appears in `reads` or `writes` with no undeclared names; (2) `0 <= lo <= hi <= alloc_extent` across all admitted shape classes; (3) `LaneScope::AllWorlds` only for `Stat` and `Stat` kernels have `feeds_state = false` (INV-SPARK-04); (4) `Exact` requires `D0` and only `u32`/`bool`/float-`Bits` args, `Guarded` requires finite `0 <= error_bound <= deferral_threshold` and `exact_fallback`, `StrictPredicate`/`Digest` refuse `Float` (`FROB-REF-252` rule), and `NegativeControl` requires the conformance registry; (5) `resources.buffer_args == args.len()` and matches IR resources (PO-SPARK-03); (6) padded reduction outputs declare exact neutral elements (`-0.0` sum, `1.0` product, `+inf` min, `-inf` max, identity rows for factorization); (7) IR load/store sets are contained in declared `reads`/`writes` (PO-SPARK-04), enforced on `BatchCpu::compile` via `KernelEntry::program_footprint`.
  - **Five binding validation rules:** (1) exact layout equality including stride unit and signedness (KA-532, CX-218; `f32`/`f64` slabs bind zero-copy to `Bits` args) plus `Stat`-slab isolation; (2) overlapping slab ranges with at least one write require `may_alias` (and `BatchCpu` requires disjoint input/output slabs per stage); (3) write footprints lie inside declared stage output ranges; (4) `epoch_read`/`epoch_write` ordering consistency; (5) backend resource bounds enforced (`<= 31` buffer arguments on Metal).
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
- `FROB-REF-601` (exit 6, `backoff`, carries `RefusalFingerprint`): raised by `KernelBackend` (`BatchCpu`) only when the backend/device is unavailable or lost.
- `FROB-REF-603` (exit 6, `after-change`, carries `RefusalFingerprint`): raised by `KernelBackend::compile` / `plan` with verbatim diagnostics when a manifest fails `N21b-01` static/KIR-footprint/alias/binding validation, aliases input/output slabs on `BatchCpu`, or lacks a backend source.
- `FROB-REF-606` (exit 6, `after-change`, carries `RefusalFingerprint`): raised fail-closed by `Receipt::validate_agent_boundary` when a device result lacks a hardware fingerprint digest (`digest: null`) or cannot yet be verified as current and citable (first repair: `frob spark fingerprint`).
- `FROB-REF-802` (exit 3, `after-change`, non-`6xx`): raised for caller API contract violations (out-of-range slab ID, `upload`/`read_back` slice length mismatch, reading an unwritten slab before producer completion, mismatched `StepPlan`/`BufferSet` allocation or descriptor, or re-entrant launch/upload/read_back).
- `FROB-REF-990` (exit 9, `never`): raised when a receipt fails `Receipt::validate()` (`AE-01`..`AE-12` missing).

## Determinism class

- `D0` (Exact) for dialect assembly, prelude expansion, body-digest hashing, static lint checks, manifest identity/validation, and `KernelClass::Exact` integer lanes on `BatchCpu`.
- `D2` (Per-Backend) ceiling on `BatchCpu` for floating-point kernels (`K-selftest` `f32` lane), computed as the meet of executed manifest policies and the backend's cap (never upgrading a weaker manifest policy).
- Cross-backend device classes (`D1` / `D2` / `D3`) are established per device family by the IEEE probe (`BD-017`) and divergence ledger (`BD-020`).

## Evidence class of outputs

`Tested`. Dialect cleanliness, prelude expansion equality, `FROB_F32_BITS` bit reinterpretation, manifest/KIR/binding validation, `BatchCpu` + `Harness` execution of `K-selftest` across worker counts (`1/2/8/36`), cancellation drain, and planted negative controls verify core static and CPU-reference execution; GPU device measurements belong to `BD-015`, `BD-016`, `BD-017`, and `BD-020`. M0 receipts carry `citable = false` until hardware fingerprints exist (`BD-029`).

## Validity domain

- Shared-dialect headers under `crates/frob-kernels/dialect/` (`prelude_cuda.h`, `prelude_metal.h`, `skeletons/*.h`, `generated/*.h`) and `K_SELFTEST_DIALECT_SOURCE`.
- Buffer argument indices `0..31` (enforcing Metal's `MAX_ARGUMENTS = 31` ceiling).
- Single-precision IEEE 754 `f32` and unsigned 32-bit `u32` / `frob_atomic_u32` buffer types in prelude v0; 256-byte-aligned disjoint input/output slabs up to `u32::MAX` lanes on `BatchCpu`.

## Oracles

- Checked-in skeletons `k_selftest.h` and `k_probe_contract.h` (KA-526 contraction detector `(1 + 2^-12)^2 - (1 + 2^-11)` and GB-25 regrouping pair `(10^8 + -10^8) + 1` vs `10^8 + (-10^8 + 1)`), plus `K_SELFTEST_DIALECT_SOURCE` expanded and linted cleanly for both CUDA and Metal.
- Independent scalar Rust loop `k_selftest_scalar_reference` checked bit-for-bit against `Harness::run(&mut BatchCpu, k_selftest_entry(), ...)` on 4,096 lanes + padding lanes (`-0.0 + (-0.0) = -0.0`, `0xFFFF_FFFF +w 1 = 0`) across 1, 2, 8, and 36 workers (`tests/backend.rs` and `tests/harness_e2e.rs`).
- Pinned verbatim FrankenSim doctrine fixture `tests/fixtures/accelerator_doctrine_fields.toml` (`# recorded:` from `frankensim/docs/ACCELERATOR_DOCTRINE.md` at `83c260121532a6a16d5b014326f9a3f937e14e94`).
- Planted negative controls: bad-dialect fixtures under `tests/fixtures/bad_dialect/`, undeclared write alias, mismatched `BufferSet` allocation/type, missing `FootprintEvidence::KirProgram` evidence, twelve independent `AE-01`..`AE-12` field deletions, unwritten output `read_back`, cancelled-before-launch output withholding, and a real multi-threaded arrival-order worker mutant.
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- Wiring `frob_kernels::manifest::DoctorKernelsReport` into `frob doctor --section kernels` (`BD-022` doctor plugin hookup).
- Replacing the local `Receipt` struct (`BD-014`) once upstream `FD-FSIM-09` lands a shared device receipt type in FrankenSim, wiring `asupersync::Cx` once `BD-008` (`INT-ASUP-01`) lands, and replacing the fail-closed M0 guard in `Receipt::validate_agent_boundary` with live fingerprint currency and placement checks in `BD-029` / `N21c-05` / `N21b-28`.
- GPU compilation and execution of expanded units (and `frob --json` structured lint reports on refused compiles) belong to `BD-015` (CUDA), `BD-016` (Metal), `BD-017` (IEEE probe), `BD-020` (GATE-SPARK-00), and `N21a-10`; the user-facing `frob spark check` shell lane `scripts/e2e/spark_harness_selftest.sh` belongs to `N21b-32`.
- Derived kernel bodies under `dialect/generated/` (`k_cay_step_body.h`) arrive with `BD-019`; `det_*` elementary functions arrive with `N21a-11`.
