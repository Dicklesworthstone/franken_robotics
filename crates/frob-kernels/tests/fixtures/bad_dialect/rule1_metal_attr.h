// synthetic
// expect: FROB-REF-251 rules=1
// Planted negative control (BD-013 rule 1): raw Metal attribute outside prelude.
void k_bad_attr(f32* out_y [[buffer(0)]]) {
    out_y[0] = FROB_F32_BITS(0x3f800000u);
}
