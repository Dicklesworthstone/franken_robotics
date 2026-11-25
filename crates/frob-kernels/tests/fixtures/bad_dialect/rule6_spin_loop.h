// synthetic
// expect: FROB-REF-251 rules=6
// Planted negative control (BD-013:spin-loop-source, CX-221, rule 6): spin wait on atomic.
FROB_KERNEL(k_bad_spin, FROB_BUF(frob_atomic_u32, bar, 0), FROB_BUF_RO(u32, target, 1))
    const u32 n = target[0];
    while (FROB_ATOMIC_MAX_U32(&bar[0], 0u) < n) {}
FROB_KERNEL_END
