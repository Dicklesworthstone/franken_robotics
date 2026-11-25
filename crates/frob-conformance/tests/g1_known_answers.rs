#![deny(unsafe_code)]

//! Gauntlet Tier G1: known answers (`BD-025` v0 target).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- <gate|row>`.
//! Builds on host with no device annex (`cuda` / `metal`). Known-answer rows land in `N38a-12`.

/// Static entry descriptor for Tier G1 (empty at v0; populated as `N38a-12` rows land).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierEntry {
    pub id: &'static str,
    pub gate: &'static str,
    pub site: &'static str,
}

pub const TIER_NAME: &str = "g1_known_answers";
pub const TIER_CODE: &str = "G1";
pub static ENTRIES: &[TierEntry] = &[];

#[test]
fn g1_known_answers_self_test() {
    assert_eq!(TIER_NAME, "g1_known_answers");
    assert_eq!(TIER_CODE, "G1");
    assert!(ENTRIES.is_empty(), "v0 tier table starts empty until N38a-12 entries land");
}
