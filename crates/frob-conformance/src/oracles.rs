#![deny(unsafe_code)]

//! Oracle register, role grammar, pin-move audit, pinned fixtures, and
//! disagreement findings (`N38a-04`, `GATE-GAUNT-05`, `INV-GAUNT-01`).
//!
//! Reads every TOML registry and fixture exclusively through
//! `frob_core::registry::toml` (N34p2-01 / STD-TEST). Pure functions (class D0),
//! zero `unsafe`, and never deletes files or directories (COMMON.md 9b).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

use crate::e2e::sha256::sha256_hex;

/// Workspace-relative path of the oracle register.
pub const ORACLES_PATH: &str = "crates/frob-conformance/registry/oracles.toml";
/// Workspace-relative path of the oracle disagreement findings log.
pub const ORACLE_FINDINGS_PATH: &str = "crates/frob-conformance/registry/oracle_findings.toml";
/// Workspace-relative path of the comparator registry (owned by `BD-039`, referenced here).
pub const COMPARATORS_PATH: &str = "crates/frob-conformance/registry/comparators.toml";

pub const ORACLES_SCHEMA: &str = "frob.gauntlet.oracles/1";
pub const ORACLE_FINDINGS_SCHEMA: &str = "frob.gauntlet.oracle-findings/1";

/// The twelve seed oracle identifiers entered by `N38a-04` (every row of the
/// §38.3 table except `development-oracle`, which `N38b-35` writes on decision).
pub const SEED_ORACLE_IDS: &[&str] = &[
    "frankensim-referee",
    "mujoco-c-f64",
    "reference-rbd-autodiff",
    "frankentorch-autograd",
    "frankenscipy-solve-ivp",
    "frankensympy-eval",
    "frankennumpy-linalg",
    "analytic-derivations",
    "incumbent-batched-simulators",
    "incumbent-tenfold-harness",
    "conventional-solvers",
    "frankensim-solid-topopt",
];

/// The two standing held-out sibling rows (`BD-034`/`FD-FSIM-10`, `BD-033`/`FD-FSIM-11`).
pub const HELD_OUT_ORACLE_IDS: &[&str] = &[
    "fs-g1-train-ppo-update",
    "fs-mbd-planar-drive",
];

/// The optional development-oracle row id written by `N38b-35`.
pub const DEVELOPMENT_ORACLE_ID: &str = "development-oracle";

const FLOATING_TAGS: &[&str] = &[
    "latest", "main", "master", "head", "nightly", "edge", "canary", "beta", "dev", "trunk",
];

const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Typed error codes for oracle register loading, fixture verification, and role checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OracleErrorKind {
    Parse,
    DuplicateId,
    InvalidPosture,
    InvalidPinKind,
    FloatingPin,
    InvalidDigest,
    InvalidRole,
    UnknownFamily,
    DevOracleFieldNotAllowed,
    DevOracleFieldMissing,
    HeldOutInvariant,
    HeldOut,
    NotAdmitted,
    ControlNotCaught,
    RoleNotGranted,
    UnknownOracle,
    PinMoveMissing,
    PinMoveEdited,
    PinMismatch,
    FixtureHeaderMissing,
    FixtureHashMismatch,
    FixtureChangedWithoutPinMove,
    WorkspaceInputForbidden,
    ScriptMissingOrTampered,
    Io,
}

impl OracleErrorKind {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Parse => "parse-error",
            Self::DuplicateId => "duplicate-id",
            Self::InvalidPosture => "invalid-posture",
            Self::InvalidPinKind => "invalid-pin-kind",
            Self::FloatingPin => "floating-pin",
            Self::InvalidDigest => "invalid-digest",
            Self::InvalidRole => "invalid-role",
            Self::UnknownFamily => "unknown-family",
            Self::DevOracleFieldNotAllowed => "dev-oracle-field-not-allowed",
            Self::DevOracleFieldMissing => "dev-oracle-field-missing",
            Self::HeldOutInvariant => "held-out-invariant",
            Self::HeldOut => "held-out",
            Self::NotAdmitted => "not-admitted",
            Self::ControlNotCaught => "control-not-caught",
            Self::RoleNotGranted => "role-not-granted",
            Self::UnknownOracle => "unknown-oracle",
            Self::PinMoveMissing => "pin-move-missing",
            Self::PinMoveEdited => "pin-move-edited",
            Self::PinMismatch => "pin-mismatch",
            Self::FixtureHeaderMissing => "fixture-header-missing",
            Self::FixtureHashMismatch => "fixture-hash-mismatch",
            Self::FixtureChangedWithoutPinMove => "fixture-changed-without-pin-move",
            Self::WorkspaceInputForbidden => "workspace-input-forbidden",
            Self::ScriptMissingOrTampered => "script-missing-or-tampered",
            Self::Io => "io-error",
        }
    }
}

/// One typed error naming the offending row or fixture path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleError {
    pub kind: OracleErrorKind,
    pub subject: String,
    pub message: String,
}

impl OracleError {
    #[must_use]
    pub fn new(
        kind: OracleErrorKind,
        subject: impl AsRef<str>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.as_ref().to_owned(),
            message: message.into(),
        }
    }
}

impl fmt::Display for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.kind.as_str(), self.subject, self.message)
    }
}

impl std::error::Error for OracleError {}

/// Closed posture enumeration for an `[[oracle]]` record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Posture {
    Consume,
    Oracle,
    OutOfProcess,
    Derivation,
}

impl Posture {
    pub fn parse(s: &str, row: &str) -> Result<Self, OracleError> {
        match s {
            "consume" => Ok(Self::Consume),
            "oracle" => Ok(Self::Oracle),
            "out-of-process" => Ok(Self::OutOfProcess),
            "derivation" => Ok(Self::Derivation),
            other => Err(OracleError::new(
                OracleErrorKind::InvalidPosture,
                row,
                format!("unknown posture `{other}` (expected consume | oracle | out-of-process | derivation)"),
            )),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consume => "consume",
            Self::Oracle => "oracle",
            Self::OutOfProcess => "out-of-process",
            Self::Derivation => "derivation",
        }
    }
}

/// Closed pin kind enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinKind {
    Release,
    Commit,
    Registry,
    Path,
}

impl PinKind {
    pub fn parse(s: &str, row: &str) -> Result<Self, OracleError> {
        match s {
            "release" => Ok(Self::Release),
            "commit" => Ok(Self::Commit),
            "registry" => Ok(Self::Registry),
            "path" => Ok(Self::Path),
            other => Err(OracleError::new(
                OracleErrorKind::InvalidPinKind,
                row,
                format!("unknown pin.kind `{other}` (expected release | commit | registry | path)"),
            )),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Commit => "commit",
            Self::Registry => "registry",
            Self::Path => "path",
        }
    }
}

/// Pin specification on an `[[oracle]]` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    pub kind: PinKind,
    pub value: String,
    pub repo: Option<String>,
    pub commit: Option<String>,
    pub sha256: Option<String>,
    pub licence: Option<String>,
    pub executable: Option<String>,
}

/// Admission status of an oracle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissionStatus {
    NotAdmitted,
    Admitted,
}

impl AdmissionStatus {
    pub fn parse(s: &str, row: &str) -> Result<Self, OracleError> {
        match s {
            "not-admitted" => Ok(Self::NotAdmitted),
            "admitted" => Ok(Self::Admitted),
            other => Err(OracleError::new(
                OracleErrorKind::Parse,
                row,
                format!("invalid admission.status `{other}` (expected not-admitted | admitted)"),
            )),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAdmitted => "not-admitted",
            Self::Admitted => "admitted",
        }
    }
}

/// Admission block on an `[[oracle]]` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    pub test: String,
    pub control: String,
    pub status: AdmissionStatus,
    pub date: String,
}

/// Closed role kind enumeration (`N38a-04` §Roles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoleKind {
    Parity,
    Reference,
    SecondChecker,
    Comparator,
    ImportFixtures,
    Referee,
    Pilot,
}

impl RoleKind {
    pub fn parse(s: &str, row: &str) -> Result<Self, OracleError> {
        match s {
            "parity" => Ok(Self::Parity),
            "reference" => Ok(Self::Reference),
            "second-checker" => Ok(Self::SecondChecker),
            "comparator" => Ok(Self::Comparator),
            "import-fixtures" => Ok(Self::ImportFixtures),
            "referee" => Ok(Self::Referee),
            "pilot" => Ok(Self::Pilot),
            other => Err(OracleError::new(
                OracleErrorKind::InvalidRole,
                row,
                format!("unknown role kind `{other}`"),
            )),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::Reference => "reference",
            Self::SecondChecker => "second-checker",
            Self::Comparator => "comparator",
            Self::ImportFixtures => "import-fixtures",
            Self::Referee => "referee",
            Self::Pilot => "pilot",
        }
    }
}

/// Parsed role (`<kind>` or `<kind>:<subject>`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Role {
    pub kind: RoleKind,
    pub subject: Option<String>,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subject {
            Some(s) => write!(f, "{}:{s}", self.kind.as_str()),
            None => f.write_str(self.kind.as_str()),
        }
    }
}

/// Parse one role token under the closed `N38a-04` role grammar.
pub fn parse_role(raw: &str, row: &str) -> Result<Role, OracleError> {
    if raw.is_empty() || raw.bytes().any(|b| b.is_ascii_whitespace()) {
        return Err(OracleError::new(
            OracleErrorKind::InvalidRole,
            row,
            format!("empty or whitespace-containing role `{raw}`"),
        ));
    }
    if let Some((kind_s, subj)) = raw.split_once(':') {
        let kind = RoleKind::parse(kind_s, row)?;
        if subj.is_empty() || subj.contains(':') || !is_valid_role_subject(subj) {
            return Err(OracleError::new(
                OracleErrorKind::InvalidRole,
                row,
                format!(
                    "invalid role subject in `{raw}` (expected lower-case hyphen-separated words)"
                ),
            ));
        }
        Ok(Role {
            kind,
            subject: Some(subj.to_owned()),
        })
    } else {
        let kind = RoleKind::parse(raw, row)?;
        Ok(Role {
            kind,
            subject: None,
        })
    }
}

fn is_valid_role_subject(s: &str) -> bool {
    if s.is_empty() || s.starts_with('-') || s.ends_with('-') || s.contains("--") {
        return false;
    }
    s.bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Optional decision metadata allowed ONLY on `id = "development-oracle"` (`N38b-35`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevOracleMeta {
    pub tool: String,
    pub label: String,
    pub decided: String,
    pub decision: String,
}

/// One `[[oracle]]` row in `crates/frob-conformance/registry/oracles.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleRow {
    pub id: String,
    pub posture: Posture,
    pub pin: Pin,
    pub admission: Admission,
    pub roles: Vec<Role>,
    pub family: Option<String>,
    pub held_out: String,
    pub dev_meta: Option<DevOracleMeta>,
}

/// One `[[pin_move]]` record (shared format reused by `BD-039`'s `comparators.toml`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinMove {
    /// Value of `oracle` (in `oracles.toml`) or `comparator` (in `comparators.toml`).
    pub target: String,
    pub from: String,
    pub to: String,
    pub date: String,
    pub reason: String,
    pub by: String,
    pub fixtures_recaptured: Vec<String>,
}

/// Parsed and validated `crates/frob-conformance/registry/oracles.toml`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OracleRegister {
    pub schema: String,
    pub oracles: Vec<OracleRow>,
    pub pin_moves: Vec<PinMove>,
}

impl OracleRegister {
    /// Load `crates/frob-conformance/registry/oracles.toml` under `workspace_root`.
    pub fn load(workspace_root: &Path) -> Result<Self, OracleError> {
        let path = workspace_root.join(ORACLES_PATH);
        let bytes = fs::read(&path)
            .map_err(|e| OracleError::new(OracleErrorKind::Io, ORACLES_PATH, e.to_string()))?;
        Self::parse_bytes(&bytes, ORACLES_PATH)
    }

    pub fn parse_str(input: &str, file: &str) -> Result<Self, OracleError> {
        Self::parse_bytes(input.as_bytes(), file)
    }

    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, OracleError> {
        let root = toml::parse(bytes)
            .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?;
        let mut top = Fields::new(root, "oracles");
        let schema = top
            .require_str("schema")
            .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?;
        if schema != ORACLES_SCHEMA {
            return Err(OracleError::new(
                OracleErrorKind::Parse,
                file,
                format!("expected schema `{ORACLES_SCHEMA}`, found `{schema}`"),
            ));
        }
        let raw_oracles = top
            .opt_array("oracle")
            .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?
            .unwrap_or_default();
        let raw_moves = top
            .opt_array("pin_move")
            .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?
            .unwrap_or_default();
        top.finish()
            .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?;

        let mut oracles = Vec::with_capacity(raw_oracles.len());
        let mut seen_ids = BTreeSet::new();
        for (idx, sp) in raw_oracles.into_iter().enumerate() {
            let tbl = match sp.value {
                Value::Table(t) => t,
                other => {
                    return Err(OracleError::new(
                        OracleErrorKind::Parse,
                        format!("oracle[{idx}]"),
                        format!("expected table, found {}", other.type_name()),
                    ));
                }
            };
            let row = parse_oracle_row(tbl, idx)?;
            if !seen_ids.insert(row.id.clone()) {
                return Err(OracleError::new(
                    OracleErrorKind::DuplicateId,
                    &row.id,
                    format!("duplicate oracle id `{}`", row.id),
                ));
            }
            oracles.push(row);
        }

        let mut pin_moves = Vec::with_capacity(raw_moves.len());
        for (idx, sp) in raw_moves.into_iter().enumerate() {
            let tbl = match sp.value {
                Value::Table(t) => t,
                other => {
                    return Err(OracleError::new(
                        OracleErrorKind::Parse,
                        format!("pin_move[{idx}]"),
                        format!("expected table, found {}", other.type_name()),
                    ));
                }
            };
            pin_moves.push(parse_pin_move_table(tbl, idx, "oracle")?);
        }

        let reg = Self {
            schema,
            oracles,
            pin_moves,
        };
        reg.check_internal_pin_move_consistency()?;
        Ok(reg)
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&OracleRow> {
        self.oracles.iter().find(|o| o.id == id)
    }

    /// Resolve an oracle id, also mapping standing held-out routine aliases
    /// (`planar_drive` -> `fs-mbd-planar-drive`, `ppo_update`/`fs-g1-train` -> `fs-g1-train-ppo-update`).
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<&OracleRow> {
        if let Some(r) = self.get(name) {
            return Some(r);
        }
        match name {
            "planar_drive" | "fs_mbd::planar_drive" | "planar-drive" => {
                self.get("fs-mbd-planar-drive")
            }
            "ppo_update" | "fs_g1_train::ppo_update" | "fs-g1-train" => {
                self.get("fs-g1-train-ppo-update")
            }
            _ => None,
        }
    }

    /// Verify that every oracle's last `[[pin_move]]` (if any) agrees with its current `pin.value`.
    pub fn check_internal_pin_move_consistency(&self) -> Result<(), OracleError> {
        let mut last_to: BTreeMap<&str, &str> = BTreeMap::new();
        for pm in &self.pin_moves {
            if self.get(&pm.target).is_none() {
                return Err(OracleError::new(
                    OracleErrorKind::UnknownOracle,
                    &pm.target,
                    format!("[[pin_move]] names unknown oracle `{}`", pm.target),
                ));
            }
            last_to.insert(pm.target.as_str(), pm.to.as_str());
        }
        for (oid, expected_to) in last_to {
            if let Some(row) = self.get(oid)
                && row.pin.value != expected_to
            {
                return Err(OracleError::new(
                    OracleErrorKind::PinMoveMissing,
                    oid,
                    format!(
                        "oracle `{oid}` pin.value `{}` differs from last [[pin_move]].to `{expected_to}`",
                        row.pin.value
                    ),
                ));
            }
        }
        Ok(())
    }

    /// Verify Rule 2 against a previous snapshot of `oracles.toml`:
    /// - Existing `[[pin_move]]` rows are append-only (never removed or edited).
    /// - Any change to an oracle's `pin` requires a newly appended `[[pin_move]]`
    ///   naming `from`, `to`, `by`, `reason`, `date`, and `fixtures_recaptured`.
    pub fn validate_against_previous(&self, prev: &Self) -> Result<(), OracleError> {
        if self.pin_moves.len() < prev.pin_moves.len()
            || self.pin_moves[..prev.pin_moves.len()] != prev.pin_moves[..]
        {
            return Err(OracleError::new(
                OracleErrorKind::PinMoveEdited,
                ORACLES_PATH,
                "[[pin_move]] log is append-only; prior records cannot be removed or modified",
            ));
        }

        let new_moves = &self.pin_moves[prev.pin_moves.len()..];
        for old in &prev.oracles {
            let Some(curr) = self.get(&old.id) else {
                return Err(OracleError::new(
                    OracleErrorKind::PinMoveEdited,
                    &old.id,
                    format!("oracle `{}` from previous register was removed (register is append-only)", old.id),
                ));
            };
            let pin_changed = curr.pin.value != old.pin.value
                || curr.pin.commit != old.pin.commit
                || curr.pin.sha256 != old.pin.sha256;
            if pin_changed {
                let recorded = new_moves.iter().any(|pm| {
                    pm.target == curr.id && pm.from == old.pin.value && pm.to == curr.pin.value
                });
                if !recorded {
                    return Err(OracleError::new(
                        OracleErrorKind::PinMoveMissing,
                        &curr.id,
                        format!(
                            "oracle `{}` pin changed from `{}` to `{}` without an appended [[pin_move]] record",
                            curr.id, old.pin.value, curr.pin.value
                        ),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Rule 5: Request an oracle to serve a role. Refuses held-out rows,
    /// non-admitted rows, uncaught admission controls, and ungranted roles.
    pub fn request_role(
        &self,
        oracle_name: &str,
        role_str: &str,
        control_caught: bool,
    ) -> Result<&OracleRow, OracleError> {
        let row = self.resolve(oracle_name).ok_or_else(|| {
            OracleError::new(
                OracleErrorKind::UnknownOracle,
                oracle_name,
                format!("unknown oracle `{oracle_name}`"),
            )
        })?;
        if !row.held_out.trim().is_empty() {
            return Err(OracleError::new(
                OracleErrorKind::HeldOut,
                &row.id,
                format!("oracle `{}` is held out: {}", row.id, row.held_out),
            ));
        }
        if row.admission.status != AdmissionStatus::Admitted {
            return Err(OracleError::new(
                OracleErrorKind::NotAdmitted,
                &row.id,
                format!(
                    "oracle `{}` has admission.status = `not-admitted` and cannot serve `{role_str}`",
                    row.id
                ),
            ));
        }
        if !control_caught {
            return Err(OracleError::new(
                OracleErrorKind::ControlNotCaught,
                &row.id,
                format!(
                    "oracle `{}` admission control `{}` has not been caught",
                    row.id, row.admission.control
                ),
            ));
        }
        let req_role = parse_role(role_str, &row.id)?;
        let granted = row.roles.iter().any(|r| {
            r == &req_role || (req_role.subject.is_none() && r.kind == req_role.kind)
        });
        if !granted {
            return Err(OracleError::new(
                OracleErrorKind::RoleNotGranted,
                &row.id,
                format!("oracle `{}` does not hold role `{role_str}`", row.id),
            ));
        }
        Ok(row)
    }
}

/// Parse a `[[pin_move]]` table using either `"oracle"` (`oracles.toml`) or
/// `"comparator"` (`comparators.toml`, `BD-039`) as the target key.
pub fn parse_pin_move_table(
    tbl: Table,
    idx: usize,
    target_key: &str,
) -> Result<PinMove, OracleError> {
    let ctx = format!("pin_move[{idx}]");
    let mut f = Fields::new(tbl, ctx.as_str());
    let err = |e: toml::TomlError| OracleError::new(OracleErrorKind::Parse, &ctx, e.to_string());
    let target = f.require_str(target_key).map_err(err)?;
    let from = f.require_str("from").map_err(err)?;
    let to = f.require_str("to").map_err(err)?;
    let date = f.require_str("date").map_err(err)?;
    let reason = f.require_str("reason").map_err(err)?;
    let by = f.require_str("by").map_err(err)?;
    let rec_items = f.require_array("fixtures_recaptured").map_err(err)?;
    let fixtures_recaptured = str_list(&ctx, "fixtures_recaptured", rec_items)?;
    f.finish().map_err(err)?;

    for (k, v) in [
        (target_key, target.as_str()),
        ("from", from.as_str()),
        ("to", to.as_str()),
        ("date", date.as_str()),
        ("reason", reason.as_str()),
        ("by", by.as_str()),
    ] {
        if v.trim().is_empty() {
            return Err(OracleError::new(
                OracleErrorKind::Parse,
                &ctx,
                format!("`{k}` must be non-empty in [[pin_move]]"),
            ));
        }
    }
    Ok(PinMove {
        target,
        from,
        to,
        date,
        reason,
        by,
        fixtures_recaptured,
    })
}

fn parse_oracle_row(tbl: Table, idx: usize) -> Result<OracleRow, OracleError> {
    let label = tbl
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map_or_else(|| format!("oracle[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("oracle[{label}]"));
    let err = |e: toml::TomlError| OracleError::new(OracleErrorKind::Parse, &label, e.to_string());

    let id = f.require_str("id").map_err(err)?;
    if id.trim().is_empty() {
        return Err(OracleError::new(
            OracleErrorKind::Parse,
            &label,
            "oracle `id` must be non-empty",
        ));
    }
    let posture = Posture::parse(&f.require_str("posture").map_err(err)?, &id)?;
    let pin = parse_pin(f.require_table("pin").map_err(err)?, &id)?;
    let admission = parse_admission(f.require_table("admission").map_err(err)?, &id)?;
    let raw_roles = str_list(&id, "roles", f.require_array("roles").map_err(err)?)?;
    let mut roles = Vec::with_capacity(raw_roles.len());
    for r in &raw_roles {
        roles.push(parse_role(r, &id)?);
    }

    let family = f
        .opt_str("family")
        .map_err(err)?
        .filter(|s| !s.trim().is_empty());
    let held_out = f.opt_str("held_out").map_err(err)?.unwrap_or_default();

    let tool = f.opt_str("tool").map_err(err)?;
    let dev_label = f.opt_str("label").map_err(err)?;
    let decided = f.opt_str("decided").map_err(err)?;
    let decision = f.opt_str("decision").map_err(err)?;

    f.finish().map_err(err)?;

    // Validate `family` against the twelve seed ids.
    if let Some(ref fam) = family
        && !SEED_ORACLE_IDS.contains(&fam.as_str())
    {
        return Err(OracleError::new(
            OracleErrorKind::UnknownFamily,
            &id,
            format!("family `{fam}` does not name a seed oracle row"),
        ));
    }

    // Validate development-oracle exclusive fields (N38b-35).
    let has_any_dev = tool.is_some()
        || dev_label.is_some()
        || decided.is_some()
        || decision.is_some();
    let dev_meta = if id == DEVELOPMENT_ORACLE_ID {
        let (Some(tool), Some(dev_label), Some(decided), Some(decision)) =
            (tool, dev_label, decided, decision)
        else {
            return Err(OracleError::new(
                OracleErrorKind::DevOracleFieldMissing,
                &id,
                "`development-oracle` row requires `tool`, `label`, `decided`, and `decision`",
            ));
        };
        if tool.trim().is_empty()
            || decided.trim().is_empty()
            || decision.trim().is_empty()
            || dev_label != "development-oracle"
        {
            return Err(OracleError::new(
                OracleErrorKind::DevOracleFieldMissing,
                &id,
                "`development-oracle` requires non-empty tool/decided/decision and label = \"development-oracle\"",
            ));
        }
        if posture != Posture::OutOfProcess {
            return Err(OracleError::new(
                OracleErrorKind::InvalidPosture,
                &id,
                "`development-oracle` must have posture = \"out-of-process\"",
            ));
        }
        if roles != [Role { kind: RoleKind::Pilot, subject: None }] {
            return Err(OracleError::new(
                OracleErrorKind::InvalidRole,
                &id,
                "`development-oracle` may only hold roles = [\"pilot\"]",
            ));
        }
        if pin.kind != PinKind::Release
            || pin.commit.as_deref().is_none_or(|c| !is_hex_len(c, 40))
            || pin.sha256.as_deref().is_none_or(|h| !is_hex_len(h, 64))
        {
            return Err(OracleError::new(
                OracleErrorKind::InvalidDigest,
                &id,
                "`development-oracle` requires pin kind `release`, 40-hex `commit`, and 64-hex `sha256`",
            ));
        }
        Some(DevOracleMeta {
            tool,
            label: dev_label,
            decided,
            decision,
        })
    } else {
        if has_any_dev {
            return Err(OracleError::new(
                OracleErrorKind::DevOracleFieldNotAllowed,
                &id,
                "fields `tool`, `label`, `decided`, and `decision` are allowed only on `id = \"development-oracle\"`",
            ));
        }
        if roles.iter().any(|r| r.kind == RoleKind::Pilot) {
            return Err(OracleError::new(
                OracleErrorKind::InvalidRole,
                &id,
                "role `pilot` is reserved for `development-oracle` only",
            ));
        }
        None
    };

    // Role `referee` is FrankenSim only.
    if roles.iter().any(|r| r.kind == RoleKind::Referee) && !id.starts_with("frankensim-") {
        return Err(OracleError::new(
            OracleErrorKind::InvalidRole,
            &id,
            "role `referee` is permitted only on FrankenSim rows",
        ));
    }

    // Held-out invariant: held-out rows may not be admitted or hold roles.
    if !held_out.trim().is_empty()
        && (admission.status != AdmissionStatus::NotAdmitted || !roles.is_empty())
    {
        return Err(OracleError::new(
            OracleErrorKind::HeldOutInvariant,
            &id,
            "held_out oracle must have admission.status = \"not-admitted\" and roles = []",
        ));
    }

    Ok(OracleRow {
        id,
        posture,
        pin,
        admission,
        roles,
        family,
        held_out,
        dev_meta,
    })
}

fn parse_pin(tbl: Table, row: &str) -> Result<Pin, OracleError> {
    let mut f = Fields::new(tbl, format!("{row}.pin"));
    let err = |e: toml::TomlError| OracleError::new(OracleErrorKind::Parse, row, e.to_string());
    let kind = PinKind::parse(&f.require_str("kind").map_err(err)?, row)?;
    let value = f.require_str("value").map_err(err)?;
    if value.trim().is_empty() {
        return Err(OracleError::new(
            OracleErrorKind::Parse,
            row,
            "pin.value must be non-empty",
        ));
    }
    let repo = f.opt_str("repo").map_err(err)?;
    let commit = f.opt_str("commit").map_err(err)?;
    let sha256 = f.opt_str("sha256").map_err(err)?;
    let licence = f.opt_str("licence").map_err(err)?;
    let executable = f.opt_str("executable").map_err(err)?;
    f.finish().map_err(err)?;

    if matches!(kind, PinKind::Release | PinKind::Commit) && is_floating_ref(&value) {
        return Err(OracleError::new(
            OracleErrorKind::FloatingPin,
            row,
            format!("floating reference `{value}` is forbidden as a pin"),
        ));
    }
    if kind == PinKind::Commit && !is_hex_len(&value, 40) {
        return Err(OracleError::new(
            OracleErrorKind::InvalidDigest,
            row,
            format!("commit pin `{value}` must be a 40-character lowercase hex SHA"),
        ));
    }
    if let Some(ref c) = commit
        && !c.is_empty()
        && !is_hex_len(c, 40)
    {
        return Err(OracleError::new(
            OracleErrorKind::InvalidDigest,
            row,
            format!("pin.commit `{c}` must be a 40-character lowercase hex SHA"),
        ));
    }
    if let Some(ref h) = sha256
        && !h.is_empty()
        && !is_hex_len(h, 64)
    {
        return Err(OracleError::new(
            OracleErrorKind::InvalidDigest,
            row,
            format!("pin.sha256 `{h}` must be a 64-character lowercase hex SHA-256"),
        ));
    }

    Ok(Pin {
        kind,
        value,
        repo,
        commit,
        sha256,
        licence,
        executable,
    })
}

fn parse_admission(tbl: Table, row: &str) -> Result<Admission, OracleError> {
    let mut f = Fields::new(tbl, format!("{row}.admission"));
    let err = |e: toml::TomlError| OracleError::new(OracleErrorKind::Parse, row, e.to_string());
    let test = f.require_str("test").map_err(err)?;
    let control = f.require_str("control").map_err(err)?;
    let status = AdmissionStatus::parse(&f.require_str("status").map_err(err)?, row)?;
    let date = f.require_str("date").map_err(err)?;
    f.finish().map_err(err)?;
    Ok(Admission {
        test,
        control,
        status,
        date,
    })
}

fn str_list(row: &str, key: &str, items: Vec<Spanned<Value>>) -> Result<Vec<String>, OracleError> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(OracleError::new(
                OracleErrorKind::Parse,
                row,
                format!("`{key}` element expected string, found {}", other.type_name()),
            )),
        })
        .collect()
}

fn is_floating_ref(val: &str) -> bool {
    let lower = val.trim().to_ascii_lowercase();
    FLOATING_TAGS.contains(&lower.as_str())
        || lower.starts_with("refs/heads/")
        || lower.ends_with("-latest")
}

#[must_use]
pub fn is_hex_len(s: &str, expected_len: usize) -> bool {
    s.len() == expected_len && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

// ============================================================================
// Fixture store, header validation, hash verification, and capture-script rules
// ============================================================================

/// Status of an oracle fixture (`active` or `disputed` after Rule 6 disagreement).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FixtureStatus {
    Active,
    Disputed,
}

impl FixtureStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disputed => "disputed",
        }
    }
}

/// Script provenance entry in a fixture header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureRef {
    pub path: String,
    pub sha256: String,
}

/// Parsed oracle fixture header + payload table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleFixture {
    pub rel_path: String,
    pub kind: String,
    pub oracle: String,
    pub pin: String,
    pub captured: String,
    pub script: FixtureRef,
    pub inputs: Vec<FixtureRef>,
    pub units: String,
    pub tolerance_note: String,
    pub bytes_sha256: String,
    pub status: FixtureStatus,
    pub payload: Table,
}

/// Compute the self-authenticating SHA-256 over a fixture file's exact bytes
/// (masking only the 64-hex digest value on the `bytes_sha256 = "..."` line
/// with 64 zeros so every single byte in the header and payload is covered).
#[must_use]
pub fn compute_fixture_bytes_sha256(content: &str) -> String {
    let mut masked = String::with_capacity(content.len());
    let mut replaced = false;
    for chunk in content.split_inclusive('\n') {
        if !replaced && chunk.trim_start().starts_with("bytes_sha256") {
            if let Some(first_q) = chunk.find('"')
                && let Some(rel_end) = chunk[first_q + 1..].find('"')
            {
                let end_q = first_q + 1 + rel_end;
                masked.push_str(&chunk[..=first_q]);
                masked.push_str(ZERO_SHA256);
                masked.push_str(&chunk[end_q..]);
                replaced = true;
                continue;
            }
        }
        masked.push_str(chunk);
    }
    sha256_hex(masked.as_bytes())
}

/// Parse and authenticate one oracle fixture file (`kind = "oracle"`).
pub fn parse_and_verify_fixture(
    content: &str,
    rel_path: &str,
) -> Result<OracleFixture, OracleError> {
    let root = toml::parse(content.as_bytes())
        .map_err(|e| OracleError::new(OracleErrorKind::FixtureHeaderMissing, rel_path, e.to_string()))?;
    let mut f = Fields::new(root, rel_path);
    let miss = |e: toml::TomlError| {
        OracleError::new(OracleErrorKind::FixtureHeaderMissing, rel_path, e.to_string())
    };

    let kind = f.require_str("kind").map_err(miss)?;
    if kind != "oracle" {
        return Err(OracleError::new(
            OracleErrorKind::FixtureHeaderMissing,
            rel_path,
            format!("fixture header must set kind = \"oracle\", found `{kind}`"),
        ));
    }
    let oracle = f.require_str("oracle").map_err(miss)?;
    let pin = f.require_str("pin").map_err(miss)?;
    let captured = f.require_str("captured").map_err(miss)?;
    let script = parse_fixture_ref(f.require_table("script").map_err(miss)?, rel_path, "script")?;
    let raw_inputs = f.require_array("inputs").map_err(miss)?;
    let mut inputs = Vec::with_capacity(raw_inputs.len());
    for (idx, sp) in raw_inputs.into_iter().enumerate() {
        let t = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(OracleError::new(
                    OracleErrorKind::FixtureHeaderMissing,
                    rel_path,
                    format!("inputs[{idx}] expected table, found {}", other.type_name()),
                ));
            }
        };
        inputs.push(parse_fixture_ref(t, rel_path, &format!("inputs[{idx}]"))?);
    }
    let units = f.require_str("units").map_err(miss)?;
    let tolerance_note = f.require_str("tolerance_note").map_err(miss)?;
    let bytes_sha256 = f.require_str("bytes_sha256").map_err(miss)?;
    let status = match f.opt_str("status").map_err(miss)?.as_deref() {
        None | Some("active") => FixtureStatus::Active,
        Some("disputed") => FixtureStatus::Disputed,
        Some(other) => {
            return Err(OracleError::new(
                OracleErrorKind::FixtureHeaderMissing,
                rel_path,
                format!("unknown fixture status `{other}` (expected active | disputed)"),
            ));
        }
    };
    let payload = f.opt_table("payload").map_err(miss)?.unwrap_or_default();
    f.finish().map_err(miss)?;

    for (k, v) in [
        ("oracle", oracle.as_str()),
        ("pin", pin.as_str()),
        ("captured", captured.as_str()),
        ("units", units.as_str()),
        ("tolerance_note", tolerance_note.as_str()),
    ] {
        if v.trim().is_empty() {
            return Err(OracleError::new(
                OracleErrorKind::FixtureHeaderMissing,
                rel_path,
                format!("fixture header field `{k}` must be non-empty"),
            ));
        }
    }
    if !is_hex_len(&bytes_sha256, 64) {
        return Err(OracleError::new(
            OracleErrorKind::FixtureHashMismatch,
            rel_path,
            format!("fixture `bytes_sha256` `{bytes_sha256}` is not a 64-hex SHA-256 digest"),
        ));
    }
    let computed = compute_fixture_bytes_sha256(content);
    if computed != bytes_sha256 {
        return Err(OracleError::new(
            OracleErrorKind::FixtureHashMismatch,
            rel_path,
            format!(
                "fixture bytes hash mismatch: header declares `{bytes_sha256}`, computed `{computed}`"
            ),
        ));
    }

    Ok(OracleFixture {
        rel_path: rel_path.to_owned(),
        kind,
        oracle,
        pin,
        captured,
        script,
        inputs,
        units,
        tolerance_note,
        bytes_sha256,
        status,
        payload,
    })
}

fn parse_fixture_ref(tbl: Table, rel_path: &str, ctx: &str) -> Result<FixtureRef, OracleError> {
    let mut f = Fields::new(tbl, format!("{rel_path}.{ctx}"));
    let err = |e: toml::TomlError| {
        OracleError::new(OracleErrorKind::FixtureHeaderMissing, rel_path, e.to_string())
    };
    let path = f.require_str("path").map_err(err)?;
    let sha256 = f.require_str("sha256").map_err(err)?;
    f.finish().map_err(err)?;
    if path.trim().is_empty() {
        return Err(OracleError::new(
            OracleErrorKind::FixtureHeaderMissing,
            rel_path,
            format!("`{ctx}.path` must be non-empty"),
        ));
    }
    if !is_hex_len(&sha256, 64) {
        return Err(OracleError::new(
            OracleErrorKind::InvalidDigest,
            rel_path,
            format!("`{ctx}.sha256` `{sha256}` must be a 64-hex SHA-256 digest"),
        ));
    }
    Ok(FixtureRef { path, sha256 })
}

/// Returns `true` if a declared capture input path names a forbidden build
/// product or crate of this workspace (`INV-GAUNT-01`, Rule 1).
#[must_use]
pub fn is_forbidden_workspace_input(raw_path: &str) -> bool {
    let normalized = raw_path.trim().replace('\\', "/");
    let stripped = normalized.trim_start_matches("./");
    if stripped.starts_with("target/")
        || stripped.contains("/target/")
        || stripped.starts_with("crates/")
        || stripped.contains("/crates/")
        || stripped.starts_with("src/")
        || stripped == "Cargo.toml"
        || stripped == "Cargo.lock"
        || stripped.ends_with("/Cargo.toml")
        || stripped.ends_with("/Cargo.lock")
    {
        return true;
    }
    let file_name = stripped.rsplit('/').next().unwrap_or(stripped);
    matches!(
        file_name,
        "frob" | "frobd" | "frob-mcp" | "frob-verify" | "frob-fsim-worker" | "frob-e2e"
    )
}

/// Scan a capture script under `tools/oracle/` for declared inputs (lines of the
/// form `# inputs: ...`, `# input: ...`, or `CAPTURE_INPUTS=...` / `INPUTS=...`)
/// and reject any path naming a workspace build product or crate.
pub fn check_capture_script_inputs(script_content: &str, script_rel: &str) -> Result<(), OracleError> {
    for (idx, raw_line) in script_content.lines().enumerate() {
        let line = raw_line.trim();
        let rest = line
            .strip_prefix("# inputs:")
            .or_else(|| line.strip_prefix("# input:"))
            .or_else(|| line.strip_prefix("CAPTURE_INPUTS="))
            .or_else(|| line.strip_prefix("INPUTS="));
        if let Some(vals) = rest {
            for tok in vals.split(|c: char| {
                c.is_ascii_whitespace() || matches!(c, ',' | ';' | '(' | ')' | '"' | '\'')
            }) {
                if !tok.is_empty() && is_forbidden_workspace_input(tok) {
                    return Err(OracleError::new(
                        OracleErrorKind::WorkspaceInputForbidden,
                        format!("{script_rel}:{}", idx + 1),
                        format!(
                            "capture script declares forbidden workspace product `{tok}` as input (INV-GAUNT-01)"
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Verify Rule 1 (capture, never regenerate) for a parsed fixture against the
/// workspace tree and `OracleRegister`.
pub fn verify_fixture_against_workspace(
    workspace_root: &Path,
    register: &OracleRegister,
    fixture: &OracleFixture,
) -> Result<(), OracleError> {
    // 1. Oracle must exist in register and pin must match register pin.
    let row = register.get(&fixture.oracle).ok_or_else(|| {
        OracleError::new(
            OracleErrorKind::UnknownOracle,
            &fixture.rel_path,
            format!("fixture cites unknown oracle `{}`", fixture.oracle),
        )
    })?;
    if fixture.pin != row.pin.value {
        return Err(OracleError::new(
            OracleErrorKind::PinMismatch,
            &fixture.rel_path,
            format!(
                "fixture pin `{}` differs from register pin `{}` for `{}`",
                fixture.pin, row.pin.value, row.id
            ),
        ));
    }

    // 2. Capture script must live under `tools/oracle/`, exist, match sha256, and declare no workspace inputs.
    let sp = Path::new(&fixture.script.path);
    if !fixture.script.path.starts_with("tools/oracle/")
        || sp.is_absolute()
        || sp.components().any(|c| matches!(c, Component::ParentDir))
    {
        return Err(OracleError::new(
            OracleErrorKind::ScriptMissingOrTampered,
            &fixture.rel_path,
            format!(
                "capture script `{}` must be a clean workspace-relative path under `tools/oracle/`",
                fixture.script.path
            ),
        ));
    }
    let script_abs = workspace_root.join(sp);
    let script_bytes = fs::read(&script_abs).map_err(|e| {
        OracleError::new(
            OracleErrorKind::ScriptMissingOrTampered,
            &fixture.rel_path,
            format!("capture script `{}` unreadable: {e}", fixture.script.path),
        )
    })?;
    let actual_script_hash = sha256_hex(&script_bytes);
    if actual_script_hash != fixture.script.sha256 {
        return Err(OracleError::new(
            OracleErrorKind::ScriptMissingOrTampered,
            &fixture.rel_path,
            format!(
                "capture script `{}` sha256 mismatch: header `{}`, actual `{actual_script_hash}`",
                fixture.script.path, fixture.script.sha256
            ),
        ));
    }
    let script_text = String::from_utf8_lossy(&script_bytes);
    check_capture_script_inputs(&script_text, &fixture.script.path)?;

    // 3. Declared fixture inputs must exclude every workspace build product or crate.
    for inp in &fixture.inputs {
        if is_forbidden_workspace_input(&inp.path) {
            return Err(OracleError::new(
                OracleErrorKind::WorkspaceInputForbidden,
                &fixture.rel_path,
                format!(
                    "fixture declares forbidden workspace build product `{}` as input (INV-GAUNT-01)",
                    inp.path
                ),
            ));
        }
    }
    Ok(())
}

/// Verify Rule 1's change guard: if a fixture's authenticated bytes hash changed
/// relative to a baseline hash, `register.pin_moves` must carry a `[[pin_move]]`
/// naming that oracle and listing `fixture.rel_path` in `fixtures_recaptured`.
pub fn verify_fixture_change_against_baseline(
    register: &OracleRegister,
    fixture: &OracleFixture,
    baseline_bytes_sha256: &str,
) -> Result<(), OracleError> {
    if fixture.bytes_sha256 == baseline_bytes_sha256 {
        return Ok(());
    }
    let has_move = register.pin_moves.iter().any(|pm| {
        pm.target == fixture.oracle
            && pm.to == fixture.pin
            && pm.fixtures_recaptured.iter().any(|p| p == &fixture.rel_path)
    });
    if has_move {
        Ok(())
    } else {
        Err(OracleError::new(
            OracleErrorKind::FixtureChangedWithoutPinMove,
            &fixture.rel_path,
            format!(
                "fixture `{}` bytes changed (`{baseline_bytes_sha256}` -> `{}`) without a [[pin_move]] listing it in `fixtures_recaptured`",
                fixture.rel_path, fixture.bytes_sha256
            ),
        ))
    }
}

/// Discover and validate all fixtures under `tests/fixtures/oracle/` and
/// `crates/frob-conformance/fixtures/`.
pub fn scan_and_verify_fixtures(
    workspace_root: &Path,
    register: &OracleRegister,
) -> Result<Vec<OracleFixture>, Vec<OracleError>> {
    let mut paths = Vec::new();
    for rel_dir in ["tests/fixtures/oracle", "crates/frob-conformance/fixtures"] {
        let dir = workspace_root.join(rel_dir);
        if dir.is_dir() {
            walk_fixture_files(&dir, &mut paths);
        }
    }
    paths.sort();

    let mut fixtures = Vec::new();
    let mut errors = Vec::new();
    for abs in paths {
        let rel = abs
            .strip_prefix(workspace_root)
            .unwrap_or(&abs)
            .to_string_lossy()
            .replace('\\', "/");
        match fs::read_to_string(&abs) {
            Err(e) => errors.push(OracleError::new(OracleErrorKind::Io, &rel, e.to_string())),
            Ok(txt) => match parse_and_verify_fixture(&txt, &rel) {
                Err(e) => errors.push(e),
                Ok(fx) => match verify_fixture_against_workspace(workspace_root, register, &fx) {
                    Err(e) => errors.push(e),
                    Ok(()) => fixtures.push(fx),
                },
            },
        }
    }

    if errors.is_empty() {
        Ok(fixtures)
    } else {
        Err(errors)
    }
}

fn walk_fixture_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if p.is_dir() {
            if name != ".git" && name != "target" {
                walk_fixture_files(&p, out);
            }
        } else if p.is_file() && !matches!(name, "README.md" | "LICENSE" | ".gitkeep" | "PIN.txt") {
            out.push(p);
        }
    }
}

// ============================================================================
// Rule 6: Disagreement between two admitted oracles is a finding (`disputed`)
// ============================================================================

/// One oracle's measured value and tolerance for Rule 6 comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct OracleMeasurement {
    pub oracle_id: String,
    pub pin: String,
    pub value: f64,
    pub tolerance: f64,
    pub units: String,
}

/// One `[[finding]]` record in `crates/frob-conformance/registry/oracle_findings.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisagreementFinding {
    pub fixture: String,
    pub measurement: String,
    pub oracle_a: String,
    pub pin_a: String,
    pub value_a: String,
    pub tolerance_a: String,
    pub oracle_b: String,
    pub pin_b: String,
    pub value_b: String,
    pub tolerance_b: String,
    pub tighter_tolerance: String,
    pub delta: String,
    pub status: String,
    pub derivation_resolution: String,
}

/// Outcome of comparing two admitted oracles on a fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisagreementVerdict {
    Agreed,
    Disputed(DisagreementFinding),
}

/// Rule 6: Compare two oracle measurements. When `|a - b| > min(tol_a, tol_b)`,
/// append a `[[finding]]` naming both pins to `findings_file`, mark the fixture
/// `status = "disputed"` (re-signing `bytes_sha256` if `fixture_file` is given),
/// and return [`DisagreementVerdict::Disputed`].
pub fn record_disagreement_if_any(
    fixture_rel: &str,
    measurement: &str,
    a: &OracleMeasurement,
    b: &OracleMeasurement,
    findings_file: &Path,
    fixture_file: Option<&Path>,
) -> Result<DisagreementVerdict, OracleError> {
    let tighter = a.tolerance.abs().min(b.tolerance.abs());
    let delta = (a.value - b.value).abs();
    if delta <= tighter {
        return Ok(DisagreementVerdict::Agreed);
    }

    let finding = DisagreementFinding {
        fixture: fixture_rel.to_owned(),
        measurement: measurement.to_owned(),
        oracle_a: a.oracle_id.clone(),
        pin_a: a.pin.clone(),
        value_a: format!("{:.6}", a.value),
        tolerance_a: format!("{:.6}", a.tolerance),
        oracle_b: b.oracle_id.clone(),
        pin_b: b.pin.clone(),
        value_b: format!("{:.6}", b.value),
        tolerance_b: format!("{:.6}", b.tolerance),
        tighter_tolerance: format!("{tighter:.6}"),
        delta: format!("{delta:.6}"),
        status: "disputed".to_owned(),
        derivation_resolution: String::new(),
    };

    append_finding_to_toml(findings_file, &finding)?;
    if let Some(fx_path) = fixture_file {
        mark_fixture_disputed(fx_path)?;
    }

    Ok(DisagreementVerdict::Disputed(finding))
}

/// True iff the fixture is not marked `disputed` and has no unresolved disputed finding.
#[must_use]
pub fn fixture_supports_gate(
    fixture: &OracleFixture,
    findings: &[DisagreementFinding],
) -> bool {
    if fixture.status == FixtureStatus::Disputed {
        return false;
    }
    !findings
        .iter()
        .any(|f| f.fixture == fixture.rel_path && f.status == "disputed" && f.derivation_resolution.is_empty())
}

/// Load all `[[finding]]` rows from `oracle_findings.toml`.
pub fn load_findings(path: &Path) -> Result<Vec<DisagreementFinding>, OracleError> {
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let bytes = fs::read(path)
        .map_err(|e| OracleError::new(OracleErrorKind::Io, path.display().to_string(), e.to_string()))?;
    parse_findings_bytes(&bytes, &path.display().to_string())
}

pub fn parse_findings_bytes(bytes: &[u8], file: &str) -> Result<Vec<DisagreementFinding>, OracleError> {
    let root = toml::parse(bytes)
        .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?;
    let mut top = Fields::new(root, "oracle_findings");
    let schema = top
        .require_str("schema")
        .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?;
    if schema != ORACLE_FINDINGS_SCHEMA {
        return Err(OracleError::new(
            OracleErrorKind::Parse,
            file,
            format!("expected schema `{ORACLE_FINDINGS_SCHEMA}`, found `{schema}`"),
        ));
    }
    let raw = top
        .opt_array("finding")
        .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?
        .unwrap_or_default();
    top.finish()
        .map_err(|e| OracleError::new(OracleErrorKind::Parse, file, e.to_string()))?;

    let mut out = Vec::with_capacity(raw.len());
    for (idx, sp) in raw.into_iter().enumerate() {
        let tbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(OracleError::new(
                    OracleErrorKind::Parse,
                    format!("finding[{idx}]"),
                    format!("expected table, found {}", other.type_name()),
                ));
            }
        };
        let mut f = Fields::new(tbl, format!("finding[{idx}]"));
        let err = |e: toml::TomlError| OracleError::new(OracleErrorKind::Parse, file, e.to_string());
        let item = DisagreementFinding {
            fixture: f.require_str("fixture").map_err(err)?,
            measurement: f.require_str("measurement").map_err(err)?,
            oracle_a: f.require_str("oracle_a").map_err(err)?,
            pin_a: f.require_str("pin_a").map_err(err)?,
            value_a: f.require_str("value_a").map_err(err)?,
            tolerance_a: f.require_str("tolerance_a").map_err(err)?,
            oracle_b: f.require_str("oracle_b").map_err(err)?,
            pin_b: f.require_str("pin_b").map_err(err)?,
            value_b: f.require_str("value_b").map_err(err)?,
            tolerance_b: f.require_str("tolerance_b").map_err(err)?,
            tighter_tolerance: f.require_str("tighter_tolerance").map_err(err)?,
            delta: f.require_str("delta").map_err(err)?,
            status: f.require_str("status").map_err(err)?,
            derivation_resolution: f.opt_str("derivation_resolution").map_err(err)?.unwrap_or_default(),
        };
        f.finish().map_err(err)?;
        out.push(item);
    }
    Ok(out)
}

fn append_finding_to_toml(path: &Path, f: &DisagreementFinding) -> Result<(), OracleError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| OracleError::new(OracleErrorKind::Io, path.display().to_string(), e.to_string()))?;
    }
    let mut existing = if path.is_file() {
        fs::read_to_string(path)
            .map_err(|e| OracleError::new(OracleErrorKind::Io, path.display().to_string(), e.to_string()))?
    } else {
        format!("schema = \"{ORACLE_FINDINGS_SCHEMA}\"\n")
    };
    if !existing.ends_with('\n') {
        existing.push('\n');
    }
    existing.push('\n');
    existing.push_str("[[finding]]\n");
    for (k, v) in [
        ("fixture", f.fixture.as_str()),
        ("measurement", f.measurement.as_str()),
        ("oracle_a", f.oracle_a.as_str()),
        ("pin_a", f.pin_a.as_str()),
        ("value_a", f.value_a.as_str()),
        ("tolerance_a", f.tolerance_a.as_str()),
        ("oracle_b", f.oracle_b.as_str()),
        ("pin_b", f.pin_b.as_str()),
        ("value_b", f.value_b.as_str()),
        ("tolerance_b", f.tolerance_b.as_str()),
        ("tighter_tolerance", f.tighter_tolerance.as_str()),
        ("delta", f.delta.as_str()),
        ("status", f.status.as_str()),
        ("derivation_resolution", f.derivation_resolution.as_str()),
    ] {
        existing.push_str(k);
        existing.push_str(" = \"");
        for ch in v.chars() {
            match ch {
                '\\' => existing.push_str("\\\\"),
                '"' => existing.push_str("\\\""),
                '\n' => existing.push_str("\\n"),
                c => existing.push(c),
            }
        }
        existing.push_str("\"\n");
    }
    fs::write(path, existing)
        .map_err(|e| OracleError::new(OracleErrorKind::Io, path.display().to_string(), e.to_string()))
}

fn mark_fixture_disputed(path: &Path) -> Result<(), OracleError> {
    let txt = fs::read_to_string(path)
        .map_err(|e| OracleError::new(OracleErrorKind::Io, path.display().to_string(), e.to_string()))?;
    let mut updated = String::with_capacity(txt.len() + 32);
    let mut saw_status = false;
    for chunk in txt.split_inclusive('\n') {
        if chunk.trim_start().starts_with("status") && chunk.contains('=') {
            updated.push_str("status = \"disputed\"\n");
            saw_status = true;
        } else {
            updated.push_str(chunk);
        }
    }
    if !saw_status {
        // Insert status right before `[payload]` or at the end of header.
        if let Some(pos) = updated.find("\n[payload]") {
            updated.insert_str(pos, "\nstatus = \"disputed\"");
        } else {
            if !updated.ends_with('\n') {
                updated.push('\n');
            }
            updated.push_str("status = \"disputed\"\n");
        }
    }
    let new_hash = compute_fixture_bytes_sha256(&updated);
    let signed = replace_bytes_sha256_line(&updated, &new_hash);
    fs::write(path, signed)
        .map_err(|e| OracleError::new(OracleErrorKind::Io, path.display().to_string(), e.to_string()))
}

/// Rewrite the `bytes_sha256 = "..."` line in fixture text with `digest`.
#[must_use]
pub fn replace_bytes_sha256_line(content: &str, digest: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut replaced = false;
    for chunk in content.split_inclusive('\n') {
        if !replaced && chunk.trim_start().starts_with("bytes_sha256") {
            if let Some(first_q) = chunk.find('"')
                && let Some(rel_end) = chunk[first_q + 1..].find('"')
            {
                let end_q = first_q + 1 + rel_end;
                out.push_str(&chunk[..=first_q]);
                out.push_str(digest);
                out.push_str(&chunk[end_q..]);
                replaced = true;
                continue;
            }
        }
        out.push_str(chunk);
    }
    out
}
