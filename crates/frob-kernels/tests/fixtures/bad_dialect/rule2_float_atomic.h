// synthetic
// expect: FROB-REF-251 rules=1,2
// Planted negative control (BD-013 rules 1 & 2): floating-point atomic in kernel source.
FROB_KERNEL(k_bad_float_atomic, FROB_BUF(f32, acc, 0), FROB_BUF_RO(f32, in_v, 1))
    atomicAdd(&acc[0], in_v[frob_gid]);
FROB_KERNEL_END
