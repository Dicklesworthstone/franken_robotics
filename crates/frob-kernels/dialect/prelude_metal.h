/* FrankenRobotics Spark Shared-Dialect Prelude v0 — Metal Shading Language 3.1.
 *
 * Production compile mode (dialect rule 4):
 *   ft-kernel-metal Gateway::library (MathMode::Safe, fastMathEnabled = false).
 *
 * Contraction-off pragmas: verified with Apple metalfe-32023.921.6 (-std=metal3.1
 * -fno-fast-math -Wall -Werror) to eliminate @llvm.fmuladd.f32 contraction that
 * Metal otherwise emits even when fast math is disabled.
 *
 * Uses no construct newer than Metal Shading Language 3.1; buffer indices i < 31.
 */
#ifndef FROB_PRELUDE_METAL_H
#define FROB_PRELUDE_METAL_H

#include <metal_stdlib>
using namespace metal;

#pragma clang fp contract(off)
#pragma STDC FP_CONTRACT OFF

#define FROB_PRELUDE_VERSION 0

typedef float f32;
typedef uint u32;
typedef atomic_uint frob_atomic_u32;

#define FROB_KERNEL(name, ...) \
    kernel void name( \
        __VA_ARGS__, \
        uint frob_gid [[thread_position_in_grid]], \
        uint frob_lid [[thread_index_in_threadgroup]] \
    ) { \
        (void)frob_gid; \
        (void)frob_lid;

#define FROB_KERNEL_END }

#define FROB_BUF(T, name, i) device T* name [[buffer(i)]]
#define FROB_BUF_RO(T, name, i) device const T* name [[buffer(i)]]

#define FROB_FN static inline
#define FROB_PTR(T) device T*

#define FROB_GROUP_SHARED(T, name, n) threadgroup T name[n]
#define FROB_GROUP_BARRIER() threadgroup_barrier(mem_flags::mem_threadgroup)

/* Order-independent unsigned 32-bit integer atomics only (rule 2). */
#define FROB_ATOMIC_ADDW_U32(ptr, v) atomic_fetch_add_explicit((ptr), (u32)(v), memory_order_relaxed)
#define FROB_ATOMIC_MAX_U32(ptr, v) atomic_fetch_max_explicit((ptr), (u32)(v), memory_order_relaxed)
#define FROB_ATOMIC_MIN_U32(ptr, v) atomic_fetch_min_explicit((ptr), (u32)(v), memory_order_relaxed)
#define FROB_ATOMIC_OR_U32(ptr, v) atomic_fetch_or_explicit((ptr), (u32)(v), memory_order_relaxed)

/* IEEE 754 correctly rounded square root (verified per device by BD-017). */
#define FROB_SQRT(x) sqrt((f32)(x))

/* Branchless select: never an arithmetic blend (rule 8, CX-219). */
#define FROB_SELECT(p, a, b) ((p) ? (a) : (b))

/* Exact 32-bit IEEE 754 reinterpretation (preserves -0.0, subnormals, ties). */
#define FROB_F32_BITS(b) as_type<f32>((u32)(b))

#endif /* FROB_PRELUDE_METAL_H */
