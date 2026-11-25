// synthetic
// expect: FROB-REF-251 rules=1,4
// Planted negative control (BD-013 rules 1 & 4): contraction-on #pragma in skeleton.
#pragma clang fp contract(fast)
FROB_KERNEL(k_bad_pragma, FROB_BUF_RO(f32, in_a, 0), FROB_BUF(f32, out_y, 1))
    const f32 a = in_a[frob_gid];
    out_y[frob_gid] = a * a;
FROB_KERNEL_END
