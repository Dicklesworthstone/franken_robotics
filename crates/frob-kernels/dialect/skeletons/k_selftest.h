/* Shared-dialect reference skeleton K-selftest (BD-013).
 *
 * Exercises every prelude v0 macro (FROB_KERNEL, FROB_BUF, FROB_BUF_RO,
 * FROB_FN, FROB_PTR, FROB_GROUP_SHARED, FROB_GROUP_BARRIER, the four
 * FROB_ATOMIC_*_U32 macros, FROB_SQRT, FROB_SELECT, FROB_F32_BITS, f32, u32)
 * and a det_* helper call with zero lint findings under rules 1, 2, 3, 4, 6.
 */

FROB_FN f32 det_selftest_scale(f32 x) {
    /* 0x3e800000u = +0.25f32; 0x80000000u = -0.0f32 */
    const f32 quarter = FROB_F32_BITS(0x3e800000u);
    const f32 neg_zero = FROB_F32_BITS(0x80000000u);
    const f32 root = FROB_SQRT(x);
    const f32 scaled = root * quarter;
    return FROB_SELECT(x > neg_zero, scaled, neg_zero);
}

FROB_FN void frob_selftest_store(FROB_PTR(f32) dst, u32 idx, f32 val) {
    dst[idx] = val;
}

FROB_KERNEL(k_selftest,
    FROB_BUF_RO(f32, in_x, 0),
    FROB_BUF(f32, out_y, 1),
    FROB_BUF(frob_atomic_u32, out_stats, 2),
    FROB_BUF_RO(u32, lane_count, 3)
)
    FROB_GROUP_SHARED(f32, tile, 64);

    const u32 n = lane_count[0];
    const u32 active = (frob_gid < n) ? 1u : 0u;
    const f32 neg_zero = FROB_F32_BITS(0x80000000u);
    const f32 x = (active != 0u) ? in_x[frob_gid] : neg_zero;
    const f32 y = det_selftest_scale(x);

    if (frob_lid < 64u) {
        tile[frob_lid] = y;
    }
    FROB_GROUP_BARRIER();

    if (active != 0u) {
        const f32 out_val = (frob_lid < 64u) ? tile[frob_lid] : y;
        frob_selftest_store(out_y, frob_gid, out_val);
        (void)FROB_ATOMIC_ADDW_U32(&out_stats[0], 1u);
        (void)FROB_ATOMIC_MAX_U32(&out_stats[1], frob_gid);
        (void)FROB_ATOMIC_MIN_U32(&out_stats[2], frob_gid);
        (void)FROB_ATOMIC_OR_U32(&out_stats[3], (frob_gid & 31u));
    }
FROB_KERNEL_END
