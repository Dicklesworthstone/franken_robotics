// synthetic
// expect: FROB-REF-251 rules=4
// Planted negative control (BD-013 rule 4): explicit fmaf.
FROB_KERNEL(k_bad_fmaf, FROB_BUF_RO(f32, in_a, 0), FROB_BUF(f32, out_y, 1))
    const f32 a = in_a[frob_gid];
    const f32 c = FROB_F32_BITS(0x3f801000u);
    out_y[frob_gid] = fmaf(a, a, -c);
FROB_KERNEL_END
