// synthetic
// expect: FROB-REF-251 rules=1
// Planted negative control (BD-013 rule 1): raw CUDA intrinsic outside prelude.
FROB_KERNEL(k_bad_syncthreads, FROB_BUF(f32, out_y, 0))
    __syncthreads();
    out_y[frob_gid] = FROB_F32_BITS(0x3f800000u);
FROB_KERNEL_END
