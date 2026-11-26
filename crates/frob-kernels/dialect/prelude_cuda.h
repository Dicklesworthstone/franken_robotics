/* FrankenRobotics Spark Shared-Dialect Prelude v0 — CUDA C++ (NVRTC).
 *
 * Production compile options (dialect rule 4):
 *   -arch=sm_89 --fmad=false   (never --use_fast_math, never --fmad=true)
 *
 * Carries no floating-point arithmetic; all constants enter kernels via
 * FROB_F32_BITS(b) as exact IEEE 754 bit patterns.
 */
#ifndef FROB_PRELUDE_CUDA_H
#define FROB_PRELUDE_CUDA_H

#define FROB_PRELUDE_VERSION 0

typedef float f32;
typedef unsigned int u32;
typedef unsigned int frob_atomic_u32;

#define FROB_KERNEL(name, ...) \
    extern "C" __global__ void name(__VA_ARGS__) { \
        const u32 frob_gid = blockIdx.x * blockDim.x + threadIdx.x; \
        const u32 frob_lid = threadIdx.x; \
        (void)frob_gid; \
        (void)frob_lid;

#define FROB_KERNEL_END }

#define FROB_BUF(T, name, i) T* name
#define FROB_BUF_RO(T, name, i) const T* name

#define FROB_FN __device__ static inline
#define FROB_PTR(T) T*

#define FROB_GROUP_SHARED(T, name, n) __shared__ T name[n]
#define FROB_GROUP_BARRIER() __syncthreads()

/* Order-independent unsigned 32-bit integer atomics only (rule 2). */
FROB_FN frob_atomic_u32* frob_atomic_u32_ptr(frob_atomic_u32* ptr) { return ptr; }
#define FROB_ATOMIC_ADDW_U32(ptr, v) atomicAdd(frob_atomic_u32_ptr(ptr), (u32)(v))
#define FROB_ATOMIC_MAX_U32(ptr, v) atomicMax(frob_atomic_u32_ptr(ptr), (u32)(v))
#define FROB_ATOMIC_MIN_U32(ptr, v) atomicMin(frob_atomic_u32_ptr(ptr), (u32)(v))
#define FROB_ATOMIC_OR_U32(ptr, v) atomicOr(frob_atomic_u32_ptr(ptr), (u32)(v))

/* IEEE 754 correctly rounded square root (verified per device by BD-017). */
#define FROB_SQRT(x) sqrtf((f32)(x))

/* Branchless select: never an arithmetic blend (rule 8, CX-219). */
#define FROB_SELECT(p, a, b) ((p) ? (a) : (b))

/* Exact 32-bit IEEE 754 reinterpretation (preserves -0.0, subnormals, ties). */
#define FROB_F32_BITS(b) __uint_as_float((u32)(b))

#endif /* FROB_PRELUDE_CUDA_H */
