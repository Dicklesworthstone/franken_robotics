//! Standalone scorer library for the FrankenRobotics ten-fold harness
//! (bead `N39a-01`, §39.1).
//!
//! Includes every file in `BOUNDARY_FILES` (from `crates/frob-conformance/tests/tenfold.rs`)
//! one-to-one via `#[path]` modules, depending ONLY on `std` and `frob-cert`.

#![forbid(unsafe_code)]

#[path = "."]
pub mod tenfold {
    #[path = "../../../crates/frob-conformance/src/tenfold/ids.rs"]
    pub mod ids;
    #[path = "../../../crates/frob-conformance/src/tenfold/refusal.rs"]
    pub mod refusal;
    #[path = "../../../crates/frob-conformance/src/tenfold/canon.rs"]
    pub mod canon;

    #[path = "."]
    pub mod records {
        #[path = "../../../crates/frob-conformance/src/tenfold/records/mission.rs"]
        pub mod mission;
        #[path = "../../../crates/frob-conformance/src/tenfold/records/freeze.rs"]
        pub mod freeze;
        #[path = "../../../crates/frob-conformance/src/tenfold/records/ledger.rs"]
        pub mod ledger;
        #[path = "../../../crates/frob-conformance/src/tenfold/records/receipt.rs"]
        pub mod receipt;
        #[path = "../../../crates/frob-conformance/src/tenfold/records/report.rs"]
        pub mod report;
    }

    #[path = "."]
    pub mod score {
        #[path = "../../../crates/frob-conformance/src/tenfold/score/ratio.rs"]
        pub mod ratio;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/bootstrap.rs"]
        pub mod bootstrap;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/guard.rs"]
        pub mod guard;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/noninferiority.rs"]
        pub mod noninferiority;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/verdict.rs"]
        pub mod verdict;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/warm.rs"]
        pub mod warm;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/monitor.rs"]
        pub mod monitor;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/ceiling.rs"]
        pub mod ceiling;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/preanalysis.rs"]
        pub mod preanalysis;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/normal.rs"]
        pub mod normal;
        #[path = "../../../crates/frob-conformance/src/tenfold/score/rng.rs"]
        pub mod rng;
    }
}
