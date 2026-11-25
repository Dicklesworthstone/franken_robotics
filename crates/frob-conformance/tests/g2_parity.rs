#![deny(unsafe_code)]

//! Gauntlet Tier G2: parity with pinned oracles (`BD-025` v0 target).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g2_parity -- <gate|row>`.

/// Static entry descriptor for Tier G2 (empty at v0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierEntry {
    pub id: &'static str,
    pub gate: &'static str,
    pub site: &'static str,
}

pub const TIER_NAME: &str = "g2_parity";
pub const TIER_CODE: &str = "G2";
pub static ENTRIES: &[TierEntry] = &[];

#[test]
fn g2_parity_self_test() {
    assert_eq!(TIER_NAME, "g2_parity");
    assert_eq!(TIER_CODE, "G2");
    assert!(ENTRIES.is_empty(), "v0 tier table starts empty");
}
