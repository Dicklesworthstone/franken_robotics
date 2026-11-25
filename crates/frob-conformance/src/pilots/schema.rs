#![deny(unsafe_code)]

//! Schema types, identifier grammar, strict `frob_core::registry::toml` parser,
//! and canonical D0 TOML serializer for the pilot register (`N38a-09`,
//! `frob.gauntlet.pilot/1`) and seed list (`pilots_expected.toml`).
//!
//! Determinism class D0; prints nothing (rule L-7); zero `unsafe`.

use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::id::Blake3;
use frob_core::registry::toml::{self, Fields, Spanned, Table, TomlError, Value};

use crate::reasons::HarnessReason;

/// Workspace-relative path of the pilot register (`registries/pilots.toml` is a
/// short planning alias for this same file).
pub const PILOTS_PATH: &str = "crates/frob-conformance/registry/pilots.toml";

/// Workspace-relative path of the seed list of pilots still to run.
pub const PILOTS_EXPECTED_PATH: &str = "crates/frob-conformance/registry/pilots_expected.toml";

/// Workspace-relative path of the generated Markdown printout.
pub const PILOTS_PRINTOUT_PATH: &str = "docs/delivery/pilots.md";

/// Workspace-relative directory where committed pilot receipts live.
pub const RECEIPTS_PILOTS_DIR: &str = "receipts/pilots";

/// Required schema identifier for `pilots.toml` (`frob.pilot/1` is rejected).
pub const PILOTS_SCHEMA: &str = "frob.gauntlet.pilot/1";

/// Required schema identifier for `pilots_expected.toml`.
pub const PILOTS_EXPECTED_SCHEMA: &str = "frob.gauntlet.pilots_expected/1";

/// Maximum number of tier-`M` (Moonshot) pilots or work packages with status
/// `running` in one milestone (doctrine D-6, `N38a-09`, `N10a-02`).
pub const MOONSHOT_RUNNING_CAP: usize = 4;

/// Convention 15 (Round-4 Review Note 2): normative rule governing pilot script exits.
pub const CONVENTION_15_RULE: &str = "A pilot's script exits 0 when its measurement was made and its record written whichever status the record carries, and non-zero only when a step could not run, a sanity assertion failed, or a negative control was not caught; the record's status, not the script's exit code, carries the verdict.";

/// Ambition tier of a mechanism (`S` = Solid, `F` = Frontier, `M` = Moonshot).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PilotTier {
    /// Solid (`S`): only entered when a chapter explicitly requires a pilot (`required_by`).
    S = 0,
    /// Frontier (`F`).
    F = 1,
    /// Moonshot (`M`).
    M = 2,
}

impl PilotTier {
    /// Canonical single-letter token stored in `pilots.toml`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::S => "S",
            Self::F => "F",
            Self::M => "M",
        }
    }

    /// Strict parser for `pilots.toml`: accepts only `"S"`, `"F"`, or `"M"`.
    #[must_use]
    pub fn parse_strict(raw: &str) -> Option<Self> {
        match raw {
            "S" => Some(Self::S),
            "F" => Some(Self::F),
            "M" => Some(Self::M),
            _ => None,
        }
    }

    /// Permissive tier tag parser used when reading external registries
    /// (capabilities, kernels, verbs) that may write `"Frontier"` / `"[F]"` / `"F"`.
    #[must_use]
    pub fn parse_external(raw: &str) -> Option<Self> {
        let norm = raw
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_ascii_lowercase();
        match norm.as_str() {
            "s" | "solid" => Some(Self::S),
            "f" | "frontier" => Some(Self::F),
            "m" | "moonshot" => Some(Self::M),
            _ => None,
        }
    }

    /// True if this is an upper tier (`F` or `M`).
    #[must_use]
    pub const fn is_upper(self) -> bool {
        matches!(self, Self::F | Self::M)
    }

    /// Rule 5 check: whether `fallback_tier` is an admissible unconditional
    /// fallback for a pilot of tier `self`.
    ///
    /// - For `M`: `F` or `S` (strictly lower).
    /// - For `F`: `S` (strictly lower).
    /// - For `S`: `S` (Solid — nothing is lower).
    #[must_use]
    pub const fn admits_fallback_tier(self, fallback_tier: Self) -> bool {
        match self {
            Self::S => matches!(fallback_tier, Self::S),
            Self::F => matches!(fallback_tier, Self::S),
            Self::M => matches!(fallback_tier, Self::S | Self::F),
        }
    }
}

impl fmt::Display for PilotTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed status enumeration for a `[[pilot]]` record and `[[pilot.result]].verdict`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PilotStatus {
    Registered,
    Running,
    Reproduced,
    Differs,
    FalsifierHolds,
    FalsifierRejected,
    Inconclusive,
    Withdrawn,
    Void,
}

impl PilotStatus {
    /// All nine status tokens in canonical order.
    pub const ALL: &'static [Self] = &[
        Self::Registered,
        Self::Running,
        Self::Reproduced,
        Self::Differs,
        Self::FalsifierHolds,
        Self::FalsifierRejected,
        Self::Inconclusive,
        Self::Withdrawn,
        Self::Void,
    ];

    /// Canonical kebab-case string stored in `pilots.toml`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registered => "registered",
            Self::Running => "running",
            Self::Reproduced => "reproduced",
            Self::Differs => "differs",
            Self::FalsifierHolds => "falsifier-holds",
            Self::FalsifierRejected => "falsifier-rejected",
            Self::Inconclusive => "inconclusive",
            Self::Withdrawn => "withdrawn",
            Self::Void => "void",
        }
    }

    /// Strict parser rejecting superseded draft spellings (`held`, `falsified`).
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "registered" => Some(Self::Registered),
            "running" => Some(Self::Running),
            "reproduced" => Some(Self::Reproduced),
            "differs" => Some(Self::Differs),
            "falsifier-holds" => Some(Self::FalsifierHolds),
            "falsifier-rejected" => Some(Self::FalsifierRejected),
            "inconclusive" => Some(Self::Inconclusive),
            "withdrawn" => Some(Self::Withdrawn),
            "void" => Some(Self::Void),
            _ => None,
        }
    }

    /// True if this status is a valid `[[pilot.result]].verdict`.
    #[must_use]
    pub const fn is_result_verdict(self) -> bool {
        matches!(
            self,
            Self::FalsifierRejected
                | Self::FalsifierHolds
                | Self::Inconclusive
                | Self::Reproduced
                | Self::Differs
        )
    }
}

impl fmt::Display for PilotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed execution-site enumeration for `tool.site` and `[[pilot.result]].site`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PilotSite {
    Host,
    DevelopmentOracle,
    Referee,
    BatchCpu,
}

impl PilotSite {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::DevelopmentOracle => "development-oracle",
            Self::Referee => "referee",
            Self::BatchCpu => "batch-cpu",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "host" => Some(Self::Host),
            "development-oracle" => Some(Self::DevelopmentOracle),
            "referee" => Some(Self::Referee),
            "batch-cpu" => Some(Self::BatchCpu),
            _ => None,
        }
    }
}

impl fmt::Display for PilotSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Frozen protocol specification table (`protocol = { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolSpec {
    pub path: String,
    pub blake3: String,
    pub frozen: String,
    pub ledger_seq: String,
    /// Optional frozen sampling-controller episode count read by `control.mpc --protocol <id>`.
    pub sampling_episodes_per_design: Option<u32>,
    /// Optional registered baseline of `sampling_episodes_per_design` if recorded explicitly.
    pub registered_sampling_episodes_per_design: Option<u32>,
}

impl ProtocolSpec {
    /// True when the protocol has a frozen hash, date, and ledger sequence.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        !self.blake3.trim().is_empty()
            && !self.frozen.trim().is_empty()
            && !self.ledger_seq.trim().is_empty()
    }

    /// Normalize the stored `blake3` digest (strip optional `"blake3:"` prefix).
    #[must_use]
    pub fn normalized_hash(&self) -> Option<&str> {
        normalize_blake3_hex(&self.blake3)
    }
}

/// Pre-computed benefit ceiling (`ceiling = { statement, value, basis }`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CeilingSpec {
    pub statement: String,
    pub value: String,
    pub basis: String,
}

/// Conditional upper-tier fallback clause (`fallback.also = [{ ships, pilot }]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackAlso {
    pub ships: String,
    pub pilot: String,
}

/// Fallback specification (`fallback = { ships, kill_gate, ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackSpec {
    pub ships: String,
    /// Optional explicit tier of `ships` (`"S"`, `"F"`, or `"M"`).
    pub tier: Option<PilotTier>,
    pub kill_gate: String,
    /// Optional proposed kill-gate wording when `kill_gate = "to-register"`.
    pub proposed_wording: Option<String>,
    /// Optional conditional upper-tier fallback parts.
    pub also: Vec<FallbackAlso>,
    /// Optional explicit bead keys needed to ship the fallback (Round-4 Review Note 3).
    pub beads: Vec<String>,
}

/// Pilot resource budget (`cost = { cpu_core_h, gpu_h, wall_h, envelope }`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostSpec {
    pub cpu_core_h: String,
    pub gpu_h: String,
    pub wall_h: String,
    pub envelope: String,
}

/// Pilot execution tool pin (`tool = { name, pin, site }`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: String,
    pub pin: String,
    pub site: PilotSite,
}

/// One measured or expected value inside `[[pilot.result]].values`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultValue {
    pub name: String,
    pub observed: String,
    pub expected: String,
    pub kind: String,
}

/// One appended, immutable `[[pilot.result]]` record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotResult {
    pub date: String,
    pub ledger_seq: Option<String>,
    pub protocol_blake3: String,
    pub site: PilotSite,
    pub environment: String,
    pub commands: Vec<String>,
    pub values: Vec<ResultValue>,
    pub verdict: PilotStatus,
    pub limits: Vec<String>,
    pub receipt: String,
    pub synthetic: bool,
    /// Optional decisive-criterion note explaining what would resolve an `inconclusive` verdict.
    pub decisive_note: Option<String>,
}

impl PilotResult {
    /// Extract the bare 64-lower-hex digest from `receipt = "blake3:<hex>"`.
    #[must_use]
    pub fn receipt_hash(&self) -> Option<&str> {
        let hex = self.receipt.strip_prefix("blake3:")?;
        if Blake3::from_hex(hex).is_ok() {
            Some(hex)
        } else {
            None
        }
    }

    /// Normalize `protocol_blake3` to a 64-lower-hex digest.
    #[must_use]
    pub fn normalized_protocol_hash(&self) -> Option<&str> {
        normalize_blake3_hex(&self.protocol_blake3)
    }

    /// Workspace-relative receipt file path `receipts/pilots/<hash>.json`.
    #[must_use]
    pub fn receipt_rel_path(&self) -> Option<String> {
        self.receipt_hash()
            .map(|h| format!("{RECEIPTS_PILOTS_DIR}/{h}.json"))
    }
}

/// One `[[pilot]]` record of `crates/frob-conformance/registry/pilots.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotRecord {
    pub id: String,
    pub mechanism: String,
    pub tier: PilotTier,
    pub required_by: Option<String>,
    pub owner: String,
    pub bead: String,
    pub work_package: String,
    /// Optional explicit milestone (e.g. `"M2"`, `"M6"`), used for per-milestone Moonshot cap.
    pub milestone: Option<String>,
    pub gated_beads: Vec<String>,
    pub protocol: ProtocolSpec,
    pub falsifier: String,
    pub ceiling: CeilingSpec,
    pub fallback: FallbackSpec,
    pub cost: CostSpec,
    pub tool: ToolSpec,
    pub status: PilotStatus,
    pub plan_time_records: Vec<String>,
    pub integrated_gate: Option<String>,
    pub results: Vec<PilotResult>,
}

impl PilotRecord {
    /// True if this record has a frozen protocol hash, date, and ledger sequence.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.protocol.is_registered()
    }

    /// True if this record's identifier belongs to the plan-time ledger (`PILOT-PT-*`).
    #[must_use]
    pub fn is_plan_time_ledger(&self) -> bool {
        is_plan_time_ledger_id(&self.id)
    }

    /// Return the authoritative result, respecting the rule that a CPU `referee`
    /// (or non-`development-oracle`) result overrides a `development-oracle` result.
    #[must_use]
    pub fn authoritative_result(&self) -> Option<&PilotResult> {
        if let Some(non_dev) = self
            .results
            .iter()
            .rev()
            .find(|r| r.site != PilotSite::DevelopmentOracle)
        {
            return Some(non_dev);
        }
        self.results.last()
    }

    /// Return the most recently appended result regardless of site.
    #[must_use]
    pub fn last_result(&self) -> Option<&PilotResult> {
        self.results.last()
    }
}

/// Parsed pilot register (`frob.gauntlet.pilot/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotRegister {
    pub schema: String,
    pub pilots: Vec<PilotRecord>,
}

impl Default for PilotRegister {
    fn default() -> Self {
        Self {
            schema: PILOTS_SCHEMA.to_owned(),
            pilots: Vec::new(),
        }
    }
}

impl PilotRegister {
    /// Return all pilot records in registration order.
    #[must_use]
    pub fn all(&self) -> &[PilotRecord] {
        &self.pilots
    }

    /// Look up one pilot record by exact identifier.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&PilotRecord> {
        self.pilots.iter().find(|p| p.id == id)
    }

    /// Look up one pilot record mutably by exact identifier.
    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut PilotRecord> {
        self.pilots.iter_mut().find(|p| p.id == id)
    }

    /// Serialize this register to deterministic canonical TOML in the strict
    /// `frob_core::registry::toml` subset (D0: identical bytes on every run).
    #[must_use]
    pub fn to_canonical_toml(&self) -> String {
        let mut out = String::new();
        out.push_str("schema = ");
        push_toml_str(&mut out, &self.schema);
        out.push('\n');

        for p in &self.pilots {
            out.push_str("\n[[pilot]]\n");
            push_kv_str(&mut out, "id", &p.id);
            push_kv_str(&mut out, "mechanism", &p.mechanism);
            push_kv_str(&mut out, "tier", p.tier.as_str());
            if let Some(ref req) = p.required_by {
                push_kv_str(&mut out, "required_by", req);
            }
            push_kv_str(&mut out, "owner", &p.owner);
            push_kv_str(&mut out, "bead", &p.bead);
            push_kv_str(&mut out, "work_package", &p.work_package);
            if let Some(ref ms) = p.milestone {
                push_kv_str(&mut out, "milestone", ms);
            }
            if !p.gated_beads.is_empty() {
                out.push_str("gated_beads = ");
                push_toml_str_array(&mut out, &p.gated_beads);
                out.push('\n');
            }

            // protocol inline table (single line per TOML 1.0 strict subset)
            out.push_str("protocol = { path = ");
            push_toml_str(&mut out, &p.protocol.path);
            out.push_str(", blake3 = ");
            push_toml_str(&mut out, &p.protocol.blake3);
            out.push_str(", frozen = ");
            push_toml_str(&mut out, &p.protocol.frozen);
            out.push_str(", ledger_seq = ");
            push_toml_str(&mut out, &p.protocol.ledger_seq);
            if let Some(ep) = p.protocol.sampling_episodes_per_design {
                out.push_str(&format!(", sampling_episodes_per_design = {ep}"));
            }
            if let Some(reg_ep) = p.protocol.registered_sampling_episodes_per_design {
                out.push_str(&format!(
                    ", registered_sampling_episodes_per_design = {reg_ep}"
                ));
            }
            out.push_str(" }\n");

            push_kv_str(&mut out, "falsifier", &p.falsifier);

            out.push_str("ceiling = { statement = ");
            push_toml_str(&mut out, &p.ceiling.statement);
            out.push_str(", value = ");
            push_toml_str(&mut out, &p.ceiling.value);
            out.push_str(", basis = ");
            push_toml_str(&mut out, &p.ceiling.basis);
            out.push_str(" }\n");

            out.push_str("fallback = { ships = ");
            push_toml_str(&mut out, &p.fallback.ships);
            if let Some(t) = p.fallback.tier {
                out.push_str(", tier = ");
                push_toml_str(&mut out, t.as_str());
            }
            out.push_str(", kill_gate = ");
            push_toml_str(&mut out, &p.fallback.kill_gate);
            if let Some(ref wording) = p.fallback.proposed_wording {
                out.push_str(", proposed_wording = ");
                push_toml_str(&mut out, wording);
            }
            if !p.fallback.also.is_empty() {
                out.push_str(", also = [");
                for (i, item) in p.fallback.also.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str("{ ships = ");
                    push_toml_str(&mut out, &item.ships);
                    out.push_str(", pilot = ");
                    push_toml_str(&mut out, &item.pilot);
                    out.push_str(" }");
                }
                out.push(']');
            }
            if !p.fallback.beads.is_empty() {
                out.push_str(", beads = ");
                push_toml_str_array(&mut out, &p.fallback.beads);
            }
            out.push_str(" }\n");

            out.push_str("cost = { cpu_core_h = ");
            push_toml_str(&mut out, &p.cost.cpu_core_h);
            out.push_str(", gpu_h = ");
            push_toml_str(&mut out, &p.cost.gpu_h);
            out.push_str(", wall_h = ");
            push_toml_str(&mut out, &p.cost.wall_h);
            out.push_str(", envelope = ");
            push_toml_str(&mut out, &p.cost.envelope);
            out.push_str(" }\n");

            out.push_str("tool = { name = ");
            push_toml_str(&mut out, &p.tool.name);
            out.push_str(", pin = ");
            push_toml_str(&mut out, &p.tool.pin);
            out.push_str(", site = ");
            push_toml_str(&mut out, p.tool.site.as_str());
            out.push_str(" }\n");

            push_kv_str(&mut out, "status", p.status.as_str());

            if !p.plan_time_records.is_empty() {
                out.push_str("plan_time_records = ");
                push_toml_str_array(&mut out, &p.plan_time_records);
                out.push('\n');
            }
            if let Some(ref gate) = p.integrated_gate {
                push_kv_str(&mut out, "integrated_gate", gate);
            }

            for r in &p.results {
                out.push_str("\n[[pilot.result]]\n");
                push_kv_str(&mut out, "date", &r.date);
                if let Some(ref seq) = r.ledger_seq {
                    push_kv_str(&mut out, "ledger_seq", seq);
                }
                push_kv_str(&mut out, "protocol_blake3", &r.protocol_blake3);
                push_kv_str(&mut out, "site", r.site.as_str());
                push_kv_str(&mut out, "environment", &r.environment);
                out.push_str("commands = ");
                push_toml_str_array(&mut out, &r.commands);
                out.push('\n');

                out.push_str("values = [");
                for (i, v) in r.values.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str("{ name = ");
                    push_toml_str(&mut out, &v.name);
                    out.push_str(", observed = ");
                    push_toml_str(&mut out, &v.observed);
                    out.push_str(", expected = ");
                    push_toml_str(&mut out, &v.expected);
                    out.push_str(", kind = ");
                    push_toml_str(&mut out, &v.kind);
                    out.push_str(" }");
                }
                out.push_str("]\n");

                push_kv_str(&mut out, "verdict", r.verdict.as_str());
                out.push_str("limits = ");
                push_toml_str_array(&mut out, &r.limits);
                out.push('\n');
                push_kv_str(&mut out, "receipt", &r.receipt);
                out.push_str("synthetic = ");
                out.push_str(if r.synthetic { "true" } else { "false" });
                out.push('\n');
                if let Some(ref note) = r.decisive_note {
                    push_kv_str(&mut out, "decisive_note", note);
                }
            }
        }
        out
    }
}

/// One row of `crates/frob-conformance/registry/pilots_expected.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedPilotRow {
    pub mechanism: String,
    pub kill_gate: String,
    pub falsifier_threshold: Option<String>,
    pub pilot: Option<String>,
    pub bead: Option<String>,
    pub work_package: Option<String>,
}

/// Loaded `crates/frob-conformance/registry/pilots_expected.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotsExpected {
    pub schema: String,
    pub rows: Vec<ExpectedPilotRow>,
}

impl Default for PilotsExpected {
    fn default() -> Self {
        Self {
            schema: PILOTS_EXPECTED_SCHEMA.to_owned(),
            rows: Vec::new(),
        }
    }
}

/// Error returned when loading, parsing, or validating `pilots.toml` or
/// `pilots_expected.toml`.
///
/// Whenever `reason` is `Some(r)`, `Display` prints `reason=<id> ...` on the
/// first line, bound directly to `crate::reasons::HarnessReason` (`N38-REF`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotLoadError {
    pub file: String,
    pub record: Option<String>,
    pub reason: Option<HarnessReason>,
    pub detail: String,
}

impl PilotLoadError {
    #[must_use]
    pub fn with_reason(
        file: impl Into<String>,
        record: Option<String>,
        reason: HarnessReason,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            record,
            reason: Some(reason),
            detail: detail.into(),
        }
    }

    #[must_use]
    pub fn parse(
        file: impl Into<String>,
        record: Option<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            record,
            reason: None,
            detail: detail.into(),
        }
    }
}

impl fmt::Display for PilotLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc = match &self.record {
            Some(r) => format!("{}: pilot `{r}`: {}", self.file, self.detail),
            None => format!("{}: {}", self.file, self.detail),
        };
        match self.reason {
            Some(reason) => write!(f, "{}", reason.format_failure_line(&loc)),
            None => f.write_str(&loc),
        }
    }
}

impl std::error::Error for PilotLoadError {}

/// Identifier grammar (`N38a-09`, DESIGN):
/// `^PILOT-(C-[0-9]{2}(-[a-z]+)?|PT-([0-9]{2}|S[0-9])|[A-Z]{2,14}-[0-9]{2})$`.
#[must_use]
pub fn is_valid_pilot_id(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("PILOT-") else {
        return false;
    };
    let b = rest.as_bytes();

    // Branch 1: C-[0-9]{2}(-[a-z]+)?
    if let Some(after_c) = rest.strip_prefix("C-") {
        let cb = after_c.as_bytes();
        if cb.len() == 2 && cb[0].is_ascii_digit() && cb[1].is_ascii_digit() {
            return true;
        }
        if cb.len() >= 4
            && cb[0].is_ascii_digit()
            && cb[1].is_ascii_digit()
            && cb[2] == b'-'
            && cb[3..].iter().all(u8::is_ascii_lowercase)
        {
            return true;
        }
        return false;
    }

    // Branch 2: PT-([0-9]{2}|S[0-9])
    if let Some(after_pt) = rest.strip_prefix("PT-") {
        let pb = after_pt.as_bytes();
        if pb.len() == 2 && pb[0].is_ascii_digit() && pb[1].is_ascii_digit() {
            return true;
        }
        if pb.len() == 2 && pb[0] == b'S' && pb[1].is_ascii_digit() {
            return true;
        }
        return false;
    }

    // Branch 3: [A-Z]{2,14}-[0-9]{2}
    if b.len() < 5 {
        return false;
    }
    let split = b.len() - 3;
    if b[split] != b'-' || !b[split + 1].is_ascii_digit() || !b[split + 2].is_ascii_digit() {
        return false;
    }
    let set = &b[..split];
    (2..=14).contains(&set.len()) && set.iter().all(u8::is_ascii_uppercase)
}

/// True iff `id` matches the plan-time ledger subset `^PILOT-PT-([0-9]{2}|S[0-9])$`.
#[must_use]
pub fn is_plan_time_ledger_id(id: &str) -> bool {
    let Some(after_pt) = id.strip_prefix("PILOT-PT-") else {
        return false;
    };
    let pb = after_pt.as_bytes();
    pb.len() == 2
        && ((pb[0].is_ascii_digit() && pb[1].is_ascii_digit())
            || (pb[0] == b'S' && pb[1].is_ascii_digit()))
}

/// Normalize a BLAKE3 digest string (either 64 lowercase hex digits or
/// `"blake3:<64 lowercase hex digits>"`), returning the bare 64-char hex slice.
#[must_use]
pub fn normalize_blake3_hex(raw: &str) -> Option<&str> {
    let hex = raw.strip_prefix("blake3:").unwrap_or(raw);
    if Blake3::from_hex(hex).is_ok() {
        Some(hex)
    } else {
        None
    }
}

/// Parse raw `pilots.toml` bytes structurally (without running cross-record or
/// filesystem validation rules yet; callers should use `validate::load_register`).
pub(crate) fn parse_register_raw(
    bytes: &[u8],
    file: &str,
) -> Result<PilotRegister, PilotLoadError> {
    let root = toml::parse(bytes).map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?;
    let mut top = Fields::new(root, "pilots");
    let schema = top
        .require_str("schema")
        .map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?;
    if schema != PILOTS_SCHEMA {
        let hint = if schema == "frob.pilot/1" {
            " (superseded draft schema `frob.pilot/1` is rejected; use `frob.gauntlet.pilot/1`)"
        } else {
            ""
        };
        return Err(PilotLoadError::parse(
            file,
            None,
            format!("expected schema `{PILOTS_SCHEMA}`, found `{schema}`{hint}"),
        ));
    }

    let raw_pilots = top
        .opt_array("pilot")
        .map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?
        .unwrap_or_default();
    top.finish()
        .map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?;

    let mut pilots = Vec::with_capacity(raw_pilots.len());
    for (idx, sp) in raw_pilots.into_iter().enumerate() {
        let tbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(PilotLoadError::parse(
                    file,
                    Some(format!("pilot[{idx}]")),
                    format!("expected table, found {}", other.type_name()),
                ));
            }
        };
        pilots.push(parse_pilot_row(tbl, idx, file)?);
    }

    Ok(PilotRegister { schema, pilots })
}

fn parse_pilot_row(tbl: Table, idx: usize, file: &str) -> Result<PilotRecord, PilotLoadError> {
    let label = tbl
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map_or_else(|| format!("pilot[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("pilot[{label}]"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(label.clone()), e.to_string());

    let id = f.require_str("id").map_err(perr)?;
    if !is_valid_pilot_id(&id) {
        return Err(PilotLoadError::with_reason(
            file,
            Some(id.clone()),
            HarnessReason::PilotIdForm,
            format!(
                "pilot identifier `{id}` does not match ^PILOT-(C-[0-9]{{2}}(-[a-z]+)?|PT-([0-9]{{2}}|S[0-9])|[A-Z]{{2,14}}-[0-9]{{2}})$"
            ),
        ));
    }

    let mechanism = f.require_str("mechanism").map_err(perr)?;
    if mechanism.trim().is_empty() {
        return Err(PilotLoadError::parse(
            file,
            Some(id),
            "`mechanism` must be non-empty",
        ));
    }

    let tier_raw = f.require_str("tier").map_err(perr)?;
    let tier = PilotTier::parse_strict(&tier_raw).ok_or_else(|| {
        PilotLoadError::parse(
            file,
            Some(id.clone()),
            format!("invalid tier `{tier_raw}` (expected single letter `S`, `F`, or `M`)"),
        )
    })?;

    let required_by = f.opt_str("required_by").map_err(perr)?;
    let owner = f.require_str("owner").map_err(perr)?;
    let bead = f.require_str("bead").map_err(perr)?;
    let work_package = f.require_str("work_package").map_err(perr)?;
    let milestone = f.opt_str("milestone").map_err(perr)?;
    let gated_beads = match f.opt_array("gated_beads").map_err(perr)? {
        Some(items) => parse_str_list(file, &id, "gated_beads", items)?,
        None => Vec::new(),
    };

    let proto_tbl = f.require_table("protocol").map_err(perr)?;
    let protocol = parse_protocol(proto_tbl, file, &id)?;

    // Rule 3: four mandatory parts (`falsifier`, `ceiling`, `fallback.ships`, `fallback.kill_gate`).
    // Missing table/key maps to `pilot-part-missing` naming the part!
    let falsifier = match f.opt_str("falsifier").map_err(perr)? {
        Some(s) => s,
        None => {
            return Err(PilotLoadError::with_reason(
                file,
                Some(id),
                HarnessReason::PilotPartMissing,
                "missing mandatory part `falsifier`",
            ));
        }
    };

    let ceiling_tbl = match f.opt_table("ceiling").map_err(perr)? {
        Some(t) => t,
        None => {
            return Err(PilotLoadError::with_reason(
                file,
                Some(id),
                HarnessReason::PilotPartMissing,
                "missing mandatory part `ceiling`",
            ));
        }
    };
    let ceiling = parse_ceiling(ceiling_tbl, file, &id)?;

    let fallback_tbl = match f.opt_table("fallback").map_err(perr)? {
        Some(t) => t,
        None => {
            return Err(PilotLoadError::with_reason(
                file,
                Some(id),
                HarnessReason::PilotPartMissing,
                "missing mandatory part `fallback` (`fallback.ships` and `fallback.kill_gate`)",
            ));
        }
    };
    let fallback = parse_fallback(fallback_tbl, file, &id)?;

    let cost_tbl = f.require_table("cost").map_err(perr)?;
    let cost = parse_cost(cost_tbl, file, &id)?;

    let tool_tbl = f.require_table("tool").map_err(perr)?;
    let tool = parse_tool(tool_tbl, file, &id)?;

    let status_raw = f.require_str("status").map_err(perr)?;
    let status = PilotStatus::parse(&status_raw).ok_or_else(|| {
        PilotLoadError::parse(
            file,
            Some(id.clone()),
            format!("unknown pilot status `{status_raw}`"),
        )
    })?;

    let plan_time_records = match f.opt_array("plan_time_records").map_err(perr)? {
        Some(items) => parse_str_list(file, &id, "plan_time_records", items)?,
        None => Vec::new(),
    };
    let integrated_gate = f.opt_str("integrated_gate").map_err(perr)?;

    let raw_results = f.opt_array("result").map_err(perr)?.unwrap_or_default();
    let mut results = Vec::with_capacity(raw_results.len());
    for (ridx, sp) in raw_results.into_iter().enumerate() {
        let rtbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(PilotLoadError::parse(
                    file,
                    Some(id.clone()),
                    format!(
                        "`result[{ridx}]` expected table, found {}",
                        other.type_name()
                    ),
                ));
            }
        };
        results.push(parse_result(rtbl, file, &id, ridx)?);
    }

    f.finish().map_err(perr)?;

    Ok(PilotRecord {
        id,
        mechanism,
        tier,
        required_by,
        owner,
        bead,
        work_package,
        milestone,
        gated_beads,
        protocol,
        falsifier,
        ceiling,
        fallback,
        cost,
        tool,
        status,
        plan_time_records,
        integrated_gate,
        results,
    })
}

fn parse_protocol(tbl: Table, file: &str, id: &str) -> Result<ProtocolSpec, PilotLoadError> {
    let mut f = Fields::new(tbl, format!("pilot[{id}].protocol"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(id.to_owned()), e.to_string());

    let path = f.require_str("path").map_err(perr)?;
    let blake3 = f.opt_str("blake3").map_err(perr)?.unwrap_or_default();
    let frozen = f.opt_str("frozen").map_err(perr)?.unwrap_or_default();
    let ledger_seq = f.opt_str("ledger_seq").map_err(perr)?.unwrap_or_default();
    let sampling_episodes_per_design = match f
        .opt_int("sampling_episodes_per_design")
        .map_err(perr)?
    {
        Some(n) if n >= 1 && n <= i64::from(u32::MAX) => Some(n as u32),
        Some(n) => {
            return Err(PilotLoadError::parse(
                file,
                Some(id.to_owned()),
                format!("`protocol.sampling_episodes_per_design` must be integer >= 1, found {n}"),
            ));
        }
        None => None,
    };
    let registered_sampling_episodes_per_design = match f
        .opt_int("registered_sampling_episodes_per_design")
        .map_err(perr)?
    {
        Some(n) if n >= 1 && n <= i64::from(u32::MAX) => Some(n as u32),
        Some(n) => {
            return Err(PilotLoadError::parse(
                file,
                Some(id.to_owned()),
                format!(
                    "`protocol.registered_sampling_episodes_per_design` must be integer >= 1, found {n}"
                ),
            ));
        }
        None => None,
    };
    f.finish().map_err(perr)?;

    if !blake3.trim().is_empty() && normalize_blake3_hex(&blake3).is_none() {
        return Err(PilotLoadError::parse(
            file,
            Some(id.to_owned()),
            format!("invalid `protocol.blake3` digest `{blake3}`"),
        ));
    }

    Ok(ProtocolSpec {
        path,
        blake3,
        frozen,
        ledger_seq,
        sampling_episodes_per_design,
        registered_sampling_episodes_per_design,
    })
}

fn parse_ceiling(tbl: Table, file: &str, id: &str) -> Result<CeilingSpec, PilotLoadError> {
    let mut f = Fields::new(tbl, format!("pilot[{id}].ceiling"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(id.to_owned()), e.to_string());

    let statement = f.opt_str("statement").map_err(perr)?.unwrap_or_default();
    let value = f.opt_str("value").map_err(perr)?.unwrap_or_default();
    let basis = f.opt_str("basis").map_err(perr)?.unwrap_or_default();
    f.finish().map_err(perr)?;

    Ok(CeilingSpec {
        statement,
        value,
        basis,
    })
}

fn parse_fallback(tbl: Table, file: &str, id: &str) -> Result<FallbackSpec, PilotLoadError> {
    let mut f = Fields::new(tbl, format!("pilot[{id}].fallback"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(id.to_owned()), e.to_string());

    let ships = f.opt_str("ships").map_err(perr)?.unwrap_or_default();
    let tier = match f.opt_str("tier").map_err(perr)? {
        Some(raw) => Some(PilotTier::parse_strict(&raw).ok_or_else(|| {
            PilotLoadError::parse(
                file,
                Some(id.to_owned()),
                format!("invalid `fallback.tier` `{raw}` (expected `S`, `F`, or `M`)"),
            )
        })?),
        None => None,
    };
    let kill_gate = f.opt_str("kill_gate").map_err(perr)?.unwrap_or_default();
    let proposed_wording = f.opt_str("proposed_wording").map_err(perr)?;

    let also = match f.opt_array("also").map_err(perr)? {
        Some(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (aidx, sp) in items.into_iter().enumerate() {
                let atbl = match sp.value {
                    Value::Table(t) => t,
                    other => {
                        return Err(PilotLoadError::parse(
                            file,
                            Some(id.to_owned()),
                            format!(
                                "`fallback.also[{aidx}]` expected table, found {}",
                                other.type_name()
                            ),
                        ));
                    }
                };
                let mut af = Fields::new(atbl, format!("pilot[{id}].fallback.also[{aidx}]"));
                let a_ships = af.require_str("ships").map_err(perr)?;
                let a_pilot = af.require_str("pilot").map_err(perr)?;
                af.finish().map_err(perr)?;
                out.push(FallbackAlso {
                    ships: a_ships,
                    pilot: a_pilot,
                });
            }
            out
        }
        None => Vec::new(),
    };

    let beads = match f.opt_array("beads").map_err(perr)? {
        Some(items) => parse_str_list(file, id, "fallback.beads", items)?,
        None => Vec::new(),
    };

    f.finish().map_err(perr)?;

    Ok(FallbackSpec {
        ships,
        tier,
        kill_gate,
        proposed_wording,
        also,
        beads,
    })
}

fn parse_cost(tbl: Table, file: &str, id: &str) -> Result<CostSpec, PilotLoadError> {
    let mut f = Fields::new(tbl, format!("pilot[{id}].cost"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(id.to_owned()), e.to_string());

    let cpu_core_h = f.require_str("cpu_core_h").map_err(perr)?;
    let gpu_h = f.require_str("gpu_h").map_err(perr)?;
    let wall_h = f.require_str("wall_h").map_err(perr)?;
    let envelope = f.require_str("envelope").map_err(perr)?;
    f.finish().map_err(perr)?;

    Ok(CostSpec {
        cpu_core_h,
        gpu_h,
        wall_h,
        envelope,
    })
}

fn parse_tool(tbl: Table, file: &str, id: &str) -> Result<ToolSpec, PilotLoadError> {
    let mut f = Fields::new(tbl, format!("pilot[{id}].tool"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(id.to_owned()), e.to_string());

    let name = f.require_str("name").map_err(perr)?;
    let pin = f.require_str("pin").map_err(perr)?;
    let site_raw = f.require_str("site").map_err(perr)?;
    let site = PilotSite::parse(&site_raw).ok_or_else(|| {
        PilotLoadError::parse(
            file,
            Some(id.to_owned()),
            format!(
                "invalid `tool.site` `{site_raw}` (expected host | development-oracle | referee | batch-cpu)"
            ),
        )
    })?;
    f.finish().map_err(perr)?;

    Ok(ToolSpec { name, pin, site })
}

fn parse_result(
    tbl: Table,
    file: &str,
    id: &str,
    ridx: usize,
) -> Result<PilotResult, PilotLoadError> {
    let mut f = Fields::new(tbl, format!("pilot[{id}].result[{ridx}]"));
    let perr = |e: TomlError| PilotLoadError::parse(file, Some(id.to_owned()), e.to_string());

    let date = f.require_str("date").map_err(perr)?;
    let ledger_seq = f.opt_str("ledger_seq").map_err(perr)?;
    let protocol_blake3 = f.require_str("protocol_blake3").map_err(perr)?;
    let site_raw = f.require_str("site").map_err(perr)?;
    let site = PilotSite::parse(&site_raw).ok_or_else(|| {
        PilotLoadError::parse(
            file,
            Some(id.to_owned()),
            format!("invalid `result[{ridx}].site` `{site_raw}`"),
        )
    })?;
    let environment = f.require_str("environment").map_err(perr)?;
    let commands_raw = f.require_array("commands").map_err(perr)?;
    let commands = parse_str_list(file, id, "result.commands", commands_raw)?;

    let values_raw = f.require_array("values").map_err(perr)?;
    let mut values = Vec::with_capacity(values_raw.len());
    for (vidx, sp) in values_raw.into_iter().enumerate() {
        let vtbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(PilotLoadError::parse(
                    file,
                    Some(id.to_owned()),
                    format!(
                        "`result[{ridx}].values[{vidx}]` expected table, found {}",
                        other.type_name()
                    ),
                ));
            }
        };
        let mut vf = Fields::new(vtbl, format!("pilot[{id}].result[{ridx}].values[{vidx}]"));
        let name = vf.require_str("name").map_err(perr)?;
        let observed = vf.require_str("observed").map_err(perr)?;
        let expected = vf.require_str("expected").map_err(perr)?;
        let kind = vf.require_str("kind").map_err(perr)?;
        vf.finish().map_err(perr)?;
        values.push(ResultValue {
            name,
            observed,
            expected,
            kind,
        });
    }

    let verdict_raw = f.require_str("verdict").map_err(perr)?;
    let verdict = PilotStatus::parse(&verdict_raw)
        .filter(|s| s.is_result_verdict())
        .ok_or_else(|| {
            PilotLoadError::parse(
                file,
                Some(id.to_owned()),
                format!(
                    "invalid `result[{ridx}].verdict` `{verdict_raw}` (expected falsifier-rejected | falsifier-holds | inconclusive | reproduced | differs)"
                ),
            )
        })?;
    let limits_raw = f.require_array("limits").map_err(perr)?;
    let limits = parse_str_list(file, id, "result.limits", limits_raw)?;
    let receipt = f.require_str("receipt").map_err(perr)?;
    let synthetic = f.require_bool("synthetic").map_err(perr)?;
    let decisive_note = f.opt_str("decisive_note").map_err(perr)?;

    f.finish().map_err(perr)?;

    Ok(PilotResult {
        date,
        ledger_seq,
        protocol_blake3,
        site,
        environment,
        commands,
        values,
        verdict,
        limits,
        receipt,
        synthetic,
        decisive_note,
    })
}

/// Load and validate `crates/frob-conformance/registry/pilots_expected.toml` under `root`.
pub fn load_pilots_expected(root: &Path) -> Result<PilotsExpected, PilotLoadError> {
    let path = root.join(PILOTS_EXPECTED_PATH);
    if !path.is_file() {
        return Ok(PilotsExpected::default());
    }
    let bytes = fs::read(&path).map_err(|e| {
        PilotLoadError::parse(PILOTS_EXPECTED_PATH, None, format!("read failed: {e}"))
    })?;
    parse_pilots_expected(&bytes, PILOTS_EXPECTED_PATH)
}

/// Parse `pilots_expected.toml` bytes using `frob_core::registry::toml` and check
/// that any optional `pilot` identifier conforms to [`is_valid_pilot_id`].
pub fn parse_pilots_expected(bytes: &[u8], file: &str) -> Result<PilotsExpected, PilotLoadError> {
    let root = toml::parse(bytes).map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?;
    let mut top = Fields::new(root, "pilots_expected");
    let schema = top
        .require_str("schema")
        .map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?;
    if schema != PILOTS_EXPECTED_SCHEMA {
        return Err(PilotLoadError::parse(
            file,
            None,
            format!("expected schema `{PILOTS_EXPECTED_SCHEMA}`, found `{schema}`"),
        ));
    }
    let raw_rows = top
        .opt_array("expected")
        .map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?
        .unwrap_or_default();
    top.finish()
        .map_err(|e| PilotLoadError::parse(file, None, e.to_string()))?;

    let mut rows = Vec::with_capacity(raw_rows.len());
    for (idx, sp) in raw_rows.into_iter().enumerate() {
        let tbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(PilotLoadError::parse(
                    file,
                    Some(format!("expected[{idx}]")),
                    format!("expected table, found {}", other.type_name()),
                ));
            }
        };
        let label = tbl
            .get("kill_gate")
            .and_then(Value::as_str)
            .map_or_else(|| format!("expected[{idx}]"), str::to_owned);
        let mut f = Fields::new(tbl, format!("expected[{label}]"));
        let perr = |e: TomlError| PilotLoadError::parse(file, Some(label.clone()), e.to_string());

        let mechanism = f.require_str("mechanism").map_err(perr)?;
        let kill_gate = f.require_str("kill_gate").map_err(perr)?;
        let falsifier_threshold = f.opt_str("falsifier_threshold").map_err(perr)?;
        let pilot = f.opt_str("pilot").map_err(perr)?;
        let bead = f.opt_str("bead").map_err(perr)?;
        let work_package = f.opt_str("work_package").map_err(perr)?;
        f.finish().map_err(perr)?;

        if mechanism.trim().is_empty() || kill_gate.trim().is_empty() {
            return Err(PilotLoadError::parse(
                file,
                Some(label),
                "`mechanism` and `kill_gate` must be non-empty",
            ));
        }
        if let Some(ref pid) = pilot
            && !is_valid_pilot_id(pid)
        {
            return Err(PilotLoadError::with_reason(
                file,
                Some(pid.clone()),
                HarnessReason::PilotIdForm,
                format!("seed list pilot identifier `{pid}` violates grammar"),
            ));
        }

        rows.push(ExpectedPilotRow {
            mechanism,
            kill_gate,
            falsifier_threshold,
            pilot,
            bead,
            work_package,
        });
    }

    Ok(PilotsExpected { schema, rows })
}

fn parse_str_list(
    file: &str,
    row: &str,
    key: &str,
    items: Vec<Spanned<Value>>,
) -> Result<Vec<String>, PilotLoadError> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(PilotLoadError::parse(
                file,
                Some(row.to_owned()),
                format!(
                    "`{key}` expected string element, found {}",
                    other.type_name()
                ),
            )),
        })
        .collect()
}

fn push_kv_str(out: &mut String, key: &str, val: &str) {
    out.push_str(key);
    out.push_str(" = ");
    push_toml_str(out, val);
    out.push('\n');
}

fn push_toml_str(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn push_toml_str_array(out: &mut String, items: &[String]) {
    out.push('[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        push_toml_str(out, item);
    }
    out.push(']');
}
