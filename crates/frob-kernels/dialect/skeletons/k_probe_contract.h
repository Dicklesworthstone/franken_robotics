/* Shared-dialect contraction & reassociation detector K-probe-contract (BD-013, KA-526, GB-25).
 *
 * Contraction detector (KA-526):
 *   a = 1 + 2^-12 (0x3f800800u), c = 1 + 2^-11 (0x3f801000u).
 *   Unfused IEEE 754 f32: a * a rounds to 1 + 2^-11 (tie to even), so (a * a) - c == 0.0.
 *   Contracted FMA:       fma(a, a, -c) == 2^-24 (0x33800000u).
 *
 * Regrouping pair (GB-25, KA-087, KA-705):
 *   ra = 10^8 (0x4cbebc20u), rb = -10^8 (0xccbebc20u), rc = 1.0 (0x3f800000u).
 *   (ra + rb) + rc == 1.0 (0x3f800000u); ra + (rb + rc) == 0.0 (0x00000000u).
 *
 * Written as one arithmetic operation per statement in clean shared dialect.
 */

FROB_KERNEL(k_probe_contract,
    FROB_BUF_RO(f32, in_a, 0),
    FROB_BUF_RO(f32, in_c, 1),
    FROB_BUF(f32, out_diff, 2),
    FROB_BUF(f32, out_fixed, 3),
    FROB_BUF_RO(u32, lane_count, 4)
)
    const u32 n = lane_count[0];
    if (frob_gid < n) {
        const f32 a = in_a[frob_gid];
        const f32 c = in_c[frob_gid];
        const f32 prod = a * a;
        const f32 diff = prod - c;
        out_diff[frob_gid] = diff;
    }

    if (frob_gid == 0u) {
        /* Fixed KA-526 contraction check: (1 + 2^-12)*(1 + 2^-12) - (1 + 2^-11). */
        const f32 ka_a = FROB_F32_BITS(0x3f800800u);
        const f32 ka_c = FROB_F32_BITS(0x3f801000u);
        const f32 ka_prod = ka_a * ka_a;
        const f32 ka_diff = ka_prod - ka_c;
        out_fixed[0] = ka_diff;

        /* Fixed GB-25 regrouping pair: (10^8 + (-10^8)) + 1 vs 10^8 + ((-10^8) + 1). */
        const f32 ra = FROB_F32_BITS(0x4cbebc20u);
        const f32 rb = FROB_F32_BITS(0xccbebc20u);
        const f32 rc = FROB_F32_BITS(0x3f800000u);
        const f32 ab = ra + rb;
        const f32 left_sum = ab + rc;
        const f32 bc = rb + rc;
        const f32 right_sum = ra + bc;
        out_fixed[1] = left_sum;
        out_fixed[2] = right_sum;
    }
FROB_KERNEL_END
