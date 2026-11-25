#![deny(unsafe_code)]

//! Gauntlet Tier G5: determinism across runs, workers, profiles, platforms, and backends (`BD-025` v0 target).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g5_determinism -- <gate|row>`.

/// Static entry descriptor for Tier G5 (empty at v0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierEntry {
    pub id: &'static str,
    pub gate: &'static str,
    pub site: &'static str,
}

pub const TIER_NAME: &str = "g5_determinism";
pub const TIER_CODE: &str = "G5";
pub static ENTRIES: &[TierEntry] = &[];

#[test]
fn g5_determinism_self_test() {
    assert_eq!(TIER_NAME, "g5_determinism");
    assert_eq!(TIER_CODE, "G5");
    assert!(ENTRIES.is_empty(), "v0 tier table starts empty");
}
