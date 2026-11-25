#![deny(unsafe_code)]

//! Gauntlet Tier G4: chaos and fault injection (`BD-025` v0 target).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g4_chaos -- <gate|row>`.

/// Static entry descriptor for Tier G4 (empty at v0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierEntry {
    pub id: &'static str,
    pub gate: &'static str,
    pub site: &'static str,
}

pub const TIER_NAME: &str = "g4_chaos";
pub const TIER_CODE: &str = "G4";
pub static ENTRIES: &[TierEntry] = &[];

#[test]
fn g4_chaos_self_test() {
    assert_eq!(TIER_NAME, "g4_chaos");
    assert_eq!(TIER_CODE, "G4");
    assert!(ENTRIES.is_empty(), "v0 tier table starts empty");
}
