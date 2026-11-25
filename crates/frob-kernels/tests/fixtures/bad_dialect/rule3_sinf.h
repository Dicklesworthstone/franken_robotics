// synthetic
// expect: FROB-REF-251 rules=3
// Planted negative control (BD-013 rule 3): built-in transcendental sinf.
FROB_KERNEL(k_bad_sinf, FROB_BUF_RO(f32, in_x, 0), FROB_BUF(f32, out_y, 1))
    f32 y = sinf(in_x[frob_gid]);
    out_y[frob_gid] = y;
FROB_KERNEL_END
