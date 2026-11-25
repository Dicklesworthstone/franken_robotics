#![deny(unsafe_code)]

//! Gauntlet Tier G0: laws and self-tests (`BD-025` v0 target).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g0_laws -- <gate|row>`.
//! Tier execution semantics, site selection, and hash-named receipts land in `N38a-02`.

/// Static entry descriptor for Tier G0 (empty at v0; populated as M0–M8 entries land).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierEntry {
    pub id: &'static str,
    pub gate: &'static str,
    pub site: &'static str,
}

pub const TIER_NAME: &str = "g0_laws";
pub const TIER_CODE: &str = "G0";
pub static ENTRIES: &[TierEntry] = &[];

#[test]
fn g0_laws_self_test() {
    assert_eq!(TIER_NAME, "g0_laws");
    assert_eq!(TIER_CODE, "G0");
    assert!(ENTRIES.is_empty(), "v0 tier table starts empty until N38a-02/chapter entries land");
}
