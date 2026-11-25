#![deny(unsafe_code)]

//! Gauntlet Tier G3: metamorphic relations (`BD-025` v0 target).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g3_metamorphic -- <gate|row>`.

/// Static entry descriptor for Tier G3 (empty at v0; populated as `N38a-14`..`N38a-18` land).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierEntry {
    pub id: &'static str,
    pub gate: &'static str,
    pub site: &'static str,
}

pub const TIER_NAME: &str = "g3_metamorphic";
pub const TIER_CODE: &str = "G3";
pub static ENTRIES: &[TierEntry] = &[];

#[test]
fn g3_metamorphic_self_test() {
    assert_eq!(TIER_NAME, "g3_metamorphic");
    assert_eq!(TIER_CODE, "G3");
    assert!(ENTRIES.is_empty(), "v0 tier table starts empty");
}
