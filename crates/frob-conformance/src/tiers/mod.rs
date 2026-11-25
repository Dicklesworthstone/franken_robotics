#![deny(unsafe_code)]

//! Gauntlet conformance tiers (`G0`–`G5`), execution sites, deterministic test
//! selection with tier-level filter extensions, same-run control execution,
//! `GateReport`-sealed gate lines, and canonical hash-named receipts
//! (`frob.gauntlet.receipt/1`, `N38a-02`, §38.2, ADR-GAUNT-01, INV-GAUNT-02,
//! INV-SUITE-03).
//!
//! Determinism class D0 (except wall-clock/session ephemera stripped by
//! [`GauntletReceipt::without_run_ephemera`]); canonical JSON strictly via
//! `frob_core::canon` (`N34-18`) and `frob_core::json::admit` (`N34-17`);
//! gate verdicts strictly via `frob_core::testkit::GateReport`; zero `unsafe`.
//!
//! ## Compile-time single-tier, single-site guarantees
//!
//! An [`Entry`] has one `tier: Tier` and one `site: Site`, so registering two
//! tiers or omitting `site` fails to compile:
//!
//! ```compile_fail,E0308
//! use frob_conformance::tiers::{Budget, Ctx, Entry, Milestone, Outcome, Site, Tier};
//! fn ok(_: &mut Ctx) -> Outcome { Outcome::pass_plain() }
//! let _bad = Entry {
//!     id: "law:bad@host",
//!     tier: [Tier::G0Laws, Tier::G1KnownAnswers],
//!     site: Site::Host,
//!     gates: &["GATE-GAUNT-01"],
//!     controls: &["NC-0001"],
//!     milestone: Milestone::M0,
//!     budget: Budget::DEFAULT,
//!     target_count: None,
//!     run: ok,
//! };
//! ```
//!
//! ```compile_fail,E0063
//! use frob_conformance::tiers::{Budget, Ctx, Entry, Milestone, Outcome, Tier};
//! fn ok(_: &mut Ctx) -> Outcome { Outcome::pass_plain() }
//! let _bad = Entry {
//!     id: "law:nosite",
//!     tier: Tier::G0Laws,
//!     gates: &["GATE-GAUNT-01"],
//!     controls: &["NC-0001"],
//!     milestone: Milestone::M0,
//!     budget: Budget::DEFAULT,
//!     target_count: None,
//!     run: ok,
//! };
//! ```

pub mod builtins;
pub mod receipt;
pub mod runner;
pub mod select;
pub mod table;

use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::path::PathBuf;

use frob_core::evidence::DetClass;
use frob_core::testkit::{CmpOp, ControlOutcome};

pub use receipt::{
    GAUNTLET_RECEIPT_SCHEMA, GauntletReceipt, ReceiptControl, ReceiptCounts, ReceiptEntry,
    ReceiptError, ReceiptGate, ReceiptMeasured, format_text_summary, read_and_verify_receipt,
    write_receipt,
};
pub use runner::{RunError, RunOptions, TierRunOutput, run_tier, tier_main};
pub use select::{
    Filter, FilterExtension, FilterResolution, FilterResolverFn, SelectError, SelectedEntries,
    parse_filter, row_key, select_entries,
};
pub use table::{TableError, TierTable, validate_disjoint_tiers};

/// The six closed Gauntlet conformance tiers (`N38a-02`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    G0Laws,
    G1KnownAnswers,
    G2Parity,
    G3Metamorphic,
    G4Chaos,
    G5Determinism,
}

impl Tier {
    /// All six tiers in specification order.
    pub const ALL: &'static [Self] = &[
        Self::G0Laws,
        Self::G1KnownAnswers,
        Self::G2Parity,
        Self::G3Metamorphic,
        Self::G4Chaos,
        Self::G5Determinism,
    ];

    /// Short canonical code (`"G0"`..`"G5"`).
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::G0Laws => "G0",
            Self::G1KnownAnswers => "G1",
            Self::G2Parity => "G2",
            Self::G3Metamorphic => "G3",
            Self::G4Chaos => "G4",
            Self::G5Determinism => "G5",
        }
    }

    /// Full Cargo integration test target name (`ADR-GAUNT-01`).
    #[must_use]
    pub const fn target_name(self) -> &'static str {
        match self {
            Self::G0Laws => "g0_laws",
            Self::G1KnownAnswers => "g1_known_answers",
            Self::G2Parity => "g2_parity",
            Self::G3Metamorphic => "g3_metamorphic",
            Self::G4Chaos => "g4_chaos",
            Self::G5Determinism => "g5_determinism",
        }
    }

    /// Parse `"G0"`..`"G5"` or `"g0_laws"`..`"g5_determinism"`.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "G0" | "g0_laws" => Some(Self::G0Laws),
            "G1" | "g1_known_answers" => Some(Self::G1KnownAnswers),
            "G2" | "g2_parity" => Some(Self::G2Parity),
            "G3" | "g3_metamorphic" => Some(Self::G3Metamorphic),
            "G4" | "g4_chaos" => Some(Self::G4Chaos),
            "G5" | "g5_determinism" => Some(Self::G5Determinism),
            _ => None,
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// The eight closed Gauntlet execution sites (`N38a-02`), ordered weakest-first
/// so `Host < Auditor < Referee < Strict < Lockstep < Cuda < Metal < Daemon`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Site {
    Host,
    Auditor,
    Referee,
    Strict,
    Lockstep,
    Cuda,
    Metal,
    Daemon,
}

impl Site {
    /// All eight sites in weakest-first execution order.
    pub const ALL: &'static [Self] = &[
        Self::Host,
        Self::Auditor,
        Self::Referee,
        Self::Strict,
        Self::Lockstep,
        Self::Cuda,
        Self::Metal,
        Self::Daemon,
    ];

    /// Canonical lowercase token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::Auditor => "auditor",
            Self::Referee => "referee",
            Self::Strict => "strict",
            Self::Lockstep => "lockstep",
            Self::Cuda => "cuda",
            Self::Metal => "metal",
            Self::Daemon => "daemon",
        }
    }

    /// Numeric weakest-first rank (`0` = `host`, `7` = `daemon`).
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Host => 0,
            Self::Auditor => 1,
            Self::Referee => 2,
            Self::Strict => 3,
            Self::Lockstep => 4,
            Self::Cuda => 5,
            Self::Metal => 6,
            Self::Daemon => 7,
        }
    }

    /// Parse a site token (with or without leading `@`).
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        let tok = raw.strip_prefix('@').unwrap_or(raw);
        match tok {
            "host" => Some(Self::Host),
            "auditor" => Some(Self::Auditor),
            "referee" => Some(Self::Referee),
            "strict" => Some(Self::Strict),
            "lockstep" => Some(Self::Lockstep),
            "cuda" => Some(Self::Cuda),
            "metal" => Some(Self::Metal),
            "daemon" => Some(Self::Daemon),
            _ => None,
        }
    }

    /// Check whether this execution site is available under `avail`.
    pub fn check_available(self, avail: &DeviceAvailability) -> Result<(), String> {
        match self {
            Self::Host
            | Self::Auditor
            | Self::Referee
            | Self::Strict
            | Self::Lockstep
            | Self::Daemon => Ok(()),
            Self::Cuda => {
                if avail.cuda {
                    Ok(())
                } else {
                    Err(
                        "site `cuda` unavailable (no CUDA device / E2E_DEVICES excludes cuda)"
                            .to_owned(),
                    )
                }
            }
            Self::Metal => {
                if avail.metal {
                    Ok(())
                } else {
                    Err(
                        "site `metal` unavailable (no Metal device / E2E_DEVICES excludes metal)"
                            .to_owned(),
                    )
                }
            }
        }
    }
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Device availability configuration derived from `E2E_DEVICES` and compiled features.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceAvailability {
    pub cuda: bool,
    pub metal: bool,
}

impl DeviceAvailability {
    /// CPU-only execution (`E2E_DEVICES=cpu`): neither CUDA nor Metal available.
    pub const CPU_ONLY: Self = Self {
        cuda: false,
        metal: false,
    };

    /// Parse an `E2E_DEVICES` string (comma-separated tokens such as `"cpu"`, `"cpu,cuda"`, `"metal"`).
    #[must_use]
    pub fn from_spec(spec: &str) -> Self {
        let tokens: BTreeSet<String> = spec
            .split(',')
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        if tokens.is_empty() {
            return Self::from_compiled_features();
        }
        Self {
            cuda: tokens.contains("cuda"),
            metal: tokens.contains("metal"),
        }
    }

    /// Detect availability from `E2E_DEVICES` environment variable, falling back
    /// to the compiled annex features (`cuda`, `metal`).
    #[must_use]
    pub fn from_env() -> Self {
        match env::var("E2E_DEVICES") {
            Ok(raw) if !raw.trim().is_empty() => Self::from_spec(&raw),
            _ => Self::from_compiled_features(),
        }
    }

    #[must_use]
    pub const fn from_compiled_features() -> Self {
        Self {
            cuda: cfg!(feature = "cuda"),
            metal: cfg!(feature = "metal"),
        }
    }
}

impl Default for DeviceAvailability {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Program milestone (`M0`..`M8`), ordered chronologically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Milestone {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
}

impl Milestone {
    /// Canonical token (`"M0"`..`"M8"`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M0 => "M0",
            Self::M1 => "M1",
            Self::M2 => "M2",
            Self::M3 => "M3",
            Self::M4 => "M4",
            Self::M5 => "M5",
            Self::M6 => "M6",
            Self::M7 => "M7",
            Self::M8 => "M8",
        }
    }

    /// Parse `"M0"`..`"M8"`.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "M0" => Some(Self::M0),
            "M1" => Some(Self::M1),
            "M2" => Some(Self::M2),
            "M3" => Some(Self::M3),
            "M4" => Some(Self::M4),
            "M5" => Some(Self::M5),
            "M6" => Some(Self::M6),
            "M7" => Some(Self::M7),
            "M8" => Some(Self::M8),
            _ => None,
        }
    }
}

impl fmt::Display for Milestone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Resource envelope class for a conformance entry (`N38a-02`).
///
/// Default envelope: 16 GiB RAM, 8 cores, 1 hour wall time (`3600` s).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Budget {
    pub max_memory_gib: u32,
    pub max_cores: u32,
    pub max_wall_secs: u64,
}

impl Budget {
    /// Default standing machine envelope (16 GiB, 8 cores, 1 hour).
    pub const DEFAULT: Self = Self {
        max_memory_gib: 16,
        max_cores: 8,
        max_wall_secs: 3600,
    };
}

impl Default for Budget {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Quantitative measurement recorded by a conformance entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measured {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub threshold: String,
    pub threshold_op: Option<CmpOp>,
    pub threshold_value: Option<String>,
    pub met: bool,
}

impl Measured {
    /// Construct a passing exact measurement (value meets threshold).
    #[must_use]
    pub fn exact(
        name: impl Into<String>,
        value: impl Into<String>,
        unit: impl Into<String>,
        threshold: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            unit: unit.into(),
            threshold: threshold.into(),
            threshold_op: None,
            threshold_value: None,
            met: true,
        }
    }

    /// Construct a measurement paired with an explicit numeric comparison op and bound.
    #[must_use]
    pub fn bounded(
        name: impl Into<String>,
        value: impl Into<String>,
        unit: impl Into<String>,
        op: CmpOp,
        bound: impl Into<String>,
        met: bool,
    ) -> Self {
        let b = bound.into();
        let threshold = format!("{} {b}", op.as_str());
        Self {
            name: name.into(),
            value: value.into(),
            unit: unit.into(),
            threshold,
            threshold_op: Some(op),
            threshold_value: Some(b),
            met,
        }
    }
}

/// Reduced-count metadata attached ONLY when an entry ran below its declared `target_count`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReducedCount {
    pub count: u64,
    pub target: u64,
}

/// Outcome returned by a single conformance [`Entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Pass {
        measured: Option<Measured>,
        determinism: Option<DetClass>,
        synthetic: bool,
    },
    Fail {
        first_diff: String,
        measured: Option<Measured>,
        determinism: Option<DetClass>,
        synthetic: bool,
    },
    Refused {
        code: String,
        detail: String,
    },
    Skipped {
        why: String,
    },
    /// Valid ONLY for a gate whose `claim` in `registries/gates.toml` is `"reality"`.
    NoClaim {
        reason: String,
    },
    /// Unfulfilled obligation (never a pass; counted under `counts.obligations`).
    Obligation {
        reason: String,
    },
}

impl Outcome {
    /// Convenience constructor for a plain D0 passing entry with no measured payload.
    #[must_use]
    pub const fn pass_plain() -> Self {
        Self::Pass {
            measured: None,
            determinism: Some(DetClass::D0Exact),
            synthetic: false,
        }
    }

    /// Convenience constructor for a D0 passing entry carrying a measurement.
    #[must_use]
    pub fn pass(measured: Measured) -> Self {
        Self::Pass {
            measured: Some(measured),
            determinism: Some(DetClass::D0Exact),
            synthetic: false,
        }
    }

    /// Convenience constructor for a failing entry naming the first diff.
    #[must_use]
    pub fn fail(first_diff: impl Into<String>) -> Self {
        Self::Fail {
            first_diff: first_diff.into(),
            measured: None,
            determinism: Some(DetClass::D0Exact),
            synthetic: false,
        }
    }

    /// Canonical receipt outcome token.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Pass { .. } => "pass",
            Self::Fail { .. } => "fail",
            Self::Refused { .. } => "refused",
            Self::Skipped { .. } => "skipped",
            Self::NoClaim { .. } => "no-claim",
            Self::Obligation { .. } => "obligation",
        }
    }
}

/// Execution context passed to each conformance entry and control function.
#[derive(Debug, Clone)]
pub struct Ctx {
    pub seed: u64,
    pub tier: Tier,
    pub site: Site,
    pub milestone: Milestone,
    pub workspace_root: PathBuf,
    pub requested_count: Option<u64>,
    pub target_count: Option<u64>,
}

impl Ctx {
    /// Effective iteration/case count: `requested_count` when smaller than
    /// `target_count`, otherwise `target_count`.
    #[must_use]
    pub fn effective_count(&self) -> Option<u64> {
        match (self.requested_count, self.target_count) {
            (Some(req), Some(target)) => Some(req.min(target)),
            (Some(req), None) => Some(req),
            (None, Some(target)) => Some(target),
            (None, None) => None,
        }
    }

    /// Returns `Some(ReducedCount { count, target })` iff `requested_count < target_count`.
    #[must_use]
    pub fn reduced(&self) -> Option<ReducedCount> {
        let (Some(req), Some(target)) = (self.requested_count, self.target_count) else {
            return None;
        };
        if req < target {
            Some(ReducedCount { count: req, target })
        } else {
            None
        }
    }
}

/// One registered conformance test entry (`N38a-02`).
///
/// Belongs to strictly one [`Tier`] and one [`Site`]; the same logical row on
/// two sites is two distinct `Entry` values.
#[derive(Clone, Copy)]
pub struct Entry {
    pub id: &'static str,
    pub tier: Tier,
    pub site: Site,
    pub gates: &'static [&'static str],
    pub controls: &'static [&'static str],
    pub milestone: Milestone,
    pub budget: Budget,
    /// Optional stated full count (for reduced-count tracking; `None` for fixed-size entries).
    pub target_count: Option<u64>,
    pub run: fn(&mut Ctx) -> Outcome,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("id", &self.id)
            .field("tier", &self.tier)
            .field("site", &self.site)
            .field("gates", &self.gates)
            .field("controls", &self.controls)
            .field("milestone", &self.milestone)
            .field("target_count", &self.target_count)
            .finish()
    }
}

/// Outcome of running one registered negative control in the same tier run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlRunOutcome {
    Caught,
    Missed,
    Stale,
    NotYetDue,
    Obligation,
}

impl ControlRunOutcome {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Caught => "caught",
            Self::Missed => "missed",
            Self::Stale => "stale",
            Self::NotYetDue => "not-yet-due",
            Self::Obligation => "obligation",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "caught" => Some(Self::Caught),
            "missed" => Some(Self::Missed),
            "stale" => Some(Self::Stale),
            "not-yet-due" => Some(Self::NotYetDue),
            "obligation" => Some(Self::Obligation),
            _ => None,
        }
    }

    /// Convert to `frob_core::testkit::ControlOutcome` when reportable to `GateReport`.
    #[must_use]
    pub const fn to_gate_report_outcome(self) -> Option<ControlOutcome> {
        match self {
            Self::Caught => Some(ControlOutcome::Caught),
            Self::Missed => Some(ControlOutcome::Missed),
            Self::Stale => Some(ControlOutcome::Stale),
            Self::Obligation => Some(ControlOutcome::Obligation),
            Self::NotYetDue => None,
        }
    }
}

impl fmt::Display for ControlRunOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A negative control executable by the tier runner after a gate's passing entries.
#[derive(Clone, Copy)]
pub struct ControlSpec {
    pub id: &'static str,
    pub gate: &'static str,
    pub run: fn(&mut Ctx) -> ControlRunOutcome,
}

impl fmt::Debug for ControlSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControlSpec")
            .field("id", &self.id)
            .field("gate", &self.gate)
            .finish()
    }
}
