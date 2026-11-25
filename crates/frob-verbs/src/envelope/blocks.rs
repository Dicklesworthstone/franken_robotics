//! The typed blocks of `frob.envelope/1` (N32a-03): refusal, job, progress, cost, budget,
//! warnings, next actions, bound identifiers (with `bound.fixtures`), pagination and the
//! no-authorization flags. Every block validates its own invariants at construction and
//! writes itself through the canonical writer. Determinism D0 apart from `cost.wall_ms`.

use frob_core::auth::{NoAuthorization, PacketAuthorization};
use frob_core::canon::{CanonError, Object};
use frob_core::id::Blake3;
use frob_core::json::INT53_MAX;
use frob_core::refusal::{Refusal, RefusalFingerprint, Repair, RetryPolicy};

use super::EnvelopeError;
use super::exit::{ExitCode, parse_refusal_code};
use super::tree::Tree;

fn no_controls(field: &'static str, s: &str) -> Result<(), EnvelopeError> {
    if s.chars().any(|c| (c as u32) < 0x20 || c == '\u{7f}') {
        return Err(EnvelopeError::InvalidField {
            field,
            reason: "control characters are not allowed".into(),
        });
    }
    Ok(())
}

fn non_empty(field: &'static str, s: &str) -> Result<(), EnvelopeError> {
    if s.is_empty() {
        return Err(EnvelopeError::InvalidField {
            field,
            reason: "must not be empty".into(),
        });
    }
    no_controls(field, s)
}

fn finite_non_negative(field: &'static str, x: f64) -> Result<(), EnvelopeError> {
    if !x.is_finite() || x < 0.0 {
        return Err(EnvelopeError::InvalidField {
            field,
            reason: format!("{x} is not a finite non-negative number"),
        });
    }
    Ok(())
}

fn slug(field: &'static str, s: &str) -> Result<(), EnvelopeError> {
    let mut chars = s.chars();
    let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !ok {
        return Err(EnvelopeError::InvalidField {
            field,
            reason: format!("`{s}` is not a lower-case slug"),
        });
    }
    Ok(())
}

/// A job state: `^[a-z][a-z-]*$`, the pattern of `job.state` in the envelope schema and of
/// `state` in `frob.progress/1` (no digits, unlike a warning code).
pub(crate) fn job_state(field: &'static str, s: &str) -> Result<(), EnvelopeError> {
    let mut chars = s.chars();
    let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c == '-');
    if !ok {
        return Err(EnvelopeError::InvalidField {
            field,
            reason: format!("`{s}` is not a job state (`^[a-z][a-z-]*$`)"),
        });
    }
    Ok(())
}

/// A JSON value carried verbatim inside the envelope (`result`, `delta`).
#[derive(Debug, Clone, PartialEq)]
pub struct JsonBody(Tree);

impl JsonBody {
    /// Admit JSON bytes (strict; duplicate members refused).
    pub fn from_json(bytes: &[u8]) -> Result<Self, EnvelopeError> {
        Tree::parse(bytes)
            .map(Self)
            .map_err(|e| EnvelopeError::InvalidField {
                field: "result",
                reason: format!("{} at `{}`", e.message, e.pointer),
            })
    }

    /// Serialize a typed term canonically and carry it.
    pub fn from_term<T: frob_core::canon::Canonical + ?Sized>(
        term: &T,
    ) -> Result<Self, EnvelopeError> {
        let bytes = frob_core::canon::to_canonical(term).map_err(EnvelopeError::Canon)?;
        Self::from_json(bytes.as_bytes())
    }

    #[must_use]
    pub const fn tree(&self) -> &Tree {
        &self.0
    }
}

/// Whether a completed result carries findings or flags (exit 2) or not (exit 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Completion {
    Clean,
    WithFindings,
}

impl Completion {
    #[must_use]
    pub const fn exit(self) -> ExitCode {
        match self {
            Self::Clean => ExitCode::Completed,
            Self::WithFindings => ExitCode::Flagged,
        }
    }
}

/// The fields of a refusal other than its code (BD-005's [`Refusal`] without `code`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusalFields {
    pub message: String,
    pub pointer: String,
    pub observed: String,
    pub required: String,
    pub unit: String,
    /// Absent on every raise of a code under its own meaning; present on every interim
    /// raise, equal to the ledger request's slug or its `interim_detail` (N35-33 rule 7).
    pub detail: Option<String>,
    pub affected_claims: Vec<String>,
    pub retry: RetryPolicy,
    /// Ranked repairs, first is best.
    pub repairs: Vec<Repair>,
}

impl RefusalFields {
    /// The non-code fields of a typed refusal.
    #[must_use]
    pub fn of(r: &Refusal) -> Self {
        Self {
            message: r.message.clone(),
            pointer: r.pointer.clone(),
            observed: r.observed.clone(),
            required: r.required.clone(),
            unit: r.unit.clone(),
            detail: r.detail.clone(),
            affected_claims: r.affected_claims.clone(),
            retry: r.retry,
            repairs: r.repairs.clone(),
        }
    }
}

/// The `refusal` block: the code, the exit its registry row carries, the fields, and
/// (for every `6xx` Spark refusal) the required device fingerprint block (`N21-REF`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusalBlock {
    pub(crate) code: String,
    pub(crate) exit: ExitCode,
    pub(crate) fields: RefusalFields,
    pub(crate) fingerprint: Option<RefusalFingerprint>,
}

impl RefusalBlock {
    /// Assemble a block with an explicit optional `6xx` fingerprint payload (`N21-REF`).
    pub(crate) fn from_parts_with_fingerprint(
        code: &str,
        exit: ExitCode,
        f: RefusalFields,
        fingerprint: Option<RefusalFingerprint>,
    ) -> Result<Self, EnvelopeError> {
        let num = parse_refusal_code(code).ok_or_else(|| EnvelopeError::InvalidField {
            field: "refusal.code",
            reason: format!("`{code}` is not FROB-REF-[1-9][0-9]{{2,3}}"),
        })?;
        if exit.is_ok() {
            return Err(EnvelopeError::InvalidField {
                field: "exit",
                reason: format!("{exit} is not a refusal exit"),
            });
        }
        non_empty("refusal.message", &f.message)?;
        if f.detail.as_deref() == Some("") {
            return Err(EnvelopeError::InvalidField {
                field: "refusal.detail",
                reason: "set but empty".into(),
            });
        }
        let is_6xx = (600..=699).contains(&num) || (6000..=6999).contains(&num);
        if is_6xx {
            let Some(fp) = &fingerprint else {
                return Err(EnvelopeError::InvalidField {
                    field: "refusal.fingerprint",
                    reason: "6xx refusals require a fingerprint block".into(),
                });
            };
            non_empty("refusal.fingerprint.device", &fp.device)?;
            match &fp.digest {
                Some(d) => non_empty("refusal.fingerprint.digest", d)?,
                None if code == "FROB-REF-606" => {}
                None => {
                    return Err(EnvelopeError::InvalidField {
                        field: "refusal.fingerprint.digest",
                        reason: "null digest is allowed only for FROB-REF-606".into(),
                    });
                }
            }
        } else if fingerprint.is_some() {
            return Err(EnvelopeError::InvalidField {
                field: "refusal.fingerprint",
                reason: "fingerprint block is only valid on 6xx refusals".into(),
            });
        }
        for c in &f.affected_claims {
            non_empty("refusal.affected_claims", c)?;
        }
        for rep in &f.repairs {
            non_empty("refusal.repairs.action", &rep.action)?;
        }
        Ok(Self {
            code: code.to_owned(),
            exit,
            fields: f,
            fingerprint,
        })
    }

    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    #[must_use]
    pub const fn exit(&self) -> ExitCode {
        self.exit
    }

    #[must_use]
    pub const fn fields(&self) -> &RefusalFields {
        &self.fields
    }

    #[must_use]
    pub const fn fingerprint(&self) -> Option<&RefusalFingerprint> {
        self.fingerprint.as_ref()
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        let f = &self.fields;
        o.array("affected_claims", |a| {
            for c in &f.affected_claims {
                a.push_str(c);
            }
            Ok(())
        })?;
        o.str("code", &self.code)?;
        if let Some(d) = &f.detail {
            o.str("detail", d)?;
        }
        if let Some(fp) = &self.fingerprint {
            o.object("fingerprint", |fo| {
                fo.str("device", &fp.device)?;
                match &fp.digest {
                    Some(d) => fo.str("digest", d),
                    None => fo.null("digest"),
                }
            })?;
        }
        o.str("message", &f.message)?;
        o.str("observed", &f.observed)?;
        o.str("pointer", &f.pointer)?;
        o.array("repairs", |a| {
            for rep in &f.repairs {
                a.push_object(|ro| {
                    ro.str("action", &rep.action)?;
                    if let Some(d) = &rep.detail {
                        ro.str("detail", d)?;
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })?;
        o.str("required", &f.required)?;
        o.str("retry", f.retry.as_str())?;
        o.str("unit", &f.unit)
    }
}

/// Progress as reported by a job engine; `total` may be unknown there, but an envelope
/// and a progress line refuse a progress without a denominator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressReport {
    pub done: u64,
    pub total: Option<u64>,
    pub unit: String,
}

/// Progress with a denominator: `done <= total`, `total >= 1`, a named unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    done: u64,
    total: u64,
    unit: String,
}

impl Progress {
    /// Refuse a report without a denominator (absent or zero `total`) or with `done > total`.
    pub fn from_report(report: &ProgressReport) -> Result<Self, EnvelopeError> {
        let total = match report.total {
            Some(t) if t >= 1 => t,
            _ => return Err(EnvelopeError::ProgressWithoutDenominator),
        };
        if report.done > total {
            return Err(EnvelopeError::InvalidField {
                field: "job.progress.done",
                reason: format!("done {} exceeds total {total}", report.done),
            });
        }
        non_empty("job.progress.unit", &report.unit)?;
        Ok(Self {
            done: report.done,
            total,
            unit: report.unit.clone(),
        })
    }

    #[must_use]
    pub const fn done(&self) -> u64 {
        self.done
    }

    #[must_use]
    pub const fn total(&self) -> u64 {
        self.total
    }

    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        o.int53("done", int53(self.done)?)?;
        o.int53("total", int53(self.total)?)?;
        o.str("unit", &self.unit)
    }
}

/// A non-negative count as the canonical writer's exact integer: at most 2^53 - 1, the
/// writer's own bound, so a value a constructor admitted also serializes.
pub(crate) fn int53(n: u64) -> Result<i64, CanonError> {
    match i64::try_from(n) {
        Ok(v) if v <= INT53_MAX => Ok(v),
        Ok(v) => Err(CanonError::Int53OutOfRange { value: v }),
        Err(_) => Err(CanonError::Int53OutOfRange { value: i64::MAX }),
    }
}

/// Point estimate of the remaining time with its interval (`lo <= point <= hi`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eta {
    pub point_s: f64,
    pub lo_s: f64,
    pub hi_s: f64,
}

/// What a job-handle report contains before validation.
#[derive(Debug, Clone, PartialEq)]
pub struct JobReport {
    pub id: String,
    pub state: String,
    pub progress: ProgressReport,
    pub eta: Option<Eta>,
    pub poll_after_ms: u64,
    pub last_committed: Option<String>,
}

/// The `job` block (exit 8, job verbs, and a budget stop's acknowledgement).
#[derive(Debug, Clone, PartialEq)]
pub struct JobHandle {
    id: String,
    state: String,
    progress: Progress,
    eta: Option<Eta>,
    poll_after_ms: u64,
    last_committed: Option<String>,
}

impl JobHandle {
    /// Validate a job report; a progress without a denominator is refused.
    pub fn new(report: &JobReport) -> Result<Self, EnvelopeError> {
        non_empty("job.id", &report.id)?;
        job_state("job.state", &report.state)?;
        let progress = Progress::from_report(&report.progress)?;
        if let Some(eta) = report.eta {
            for (f, x) in [
                ("job.eta_s.point", eta.point_s),
                ("job.eta_s.lo", eta.lo_s),
                ("job.eta_s.hi", eta.hi_s),
            ] {
                finite_non_negative(f, x)?;
            }
            if !(eta.lo_s <= eta.point_s && eta.point_s <= eta.hi_s) {
                return Err(EnvelopeError::InvalidField {
                    field: "job.eta_s",
                    reason: "requires lo <= point <= hi".into(),
                });
            }
        }
        if let Some(lc) = &report.last_committed {
            non_empty("job.last_committed", lc)?;
        }
        int53(report.poll_after_ms).map_err(EnvelopeError::Canon)?;
        Ok(Self {
            id: report.id.clone(),
            state: report.state.clone(),
            progress,
            eta: report.eta,
            poll_after_ms: report.poll_after_ms,
            last_committed: report.last_committed.clone(),
        })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn state(&self) -> &str {
        &self.state
    }

    #[must_use]
    pub const fn progress(&self) -> &Progress {
        &self.progress
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        if let Some(eta) = self.eta {
            o.object("eta_s", |e| {
                e.f64_shortest("hi", eta.hi_s)?;
                e.f64_shortest("lo", eta.lo_s)?;
                e.f64_shortest("point", eta.point_s)
            })?;
        }
        o.str("id", &self.id)?;
        if let Some(lc) = &self.last_committed {
            o.str("last_committed", lc)?;
        }
        o.int53("poll_after_ms", int53(self.poll_after_ms)?)?;
        o.object("progress", |p| self.progress.write(p))?;
        o.str("state", &self.state)
    }
}

/// What this call spent, in named units. `wall_ms` is the one wall-clock field of the
/// envelope and never enters a digest or an evidence record (INV-TETHER-09).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cost {
    pub wall_ms: u64,
    pub cpu_core_s: f64,
    pub gpu_s: f64,
}

impl Cost {
    /// Nothing was charged.
    pub const ZERO: Self = Self {
        wall_ms: 0,
        cpu_core_s: 0.0,
        gpu_s: 0.0,
    };

    pub(crate) fn check(&self) -> Result<(), EnvelopeError> {
        finite_non_negative("cost.cpu_core_s", self.cpu_core_s)?;
        finite_non_negative("cost.gpu_s", self.gpu_s)?;
        int53(self.wall_ms).map_err(EnvelopeError::Canon)?;
        Ok(())
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        o.f64_shortest("cpu_core_s", self.cpu_core_s)?;
        o.f64_shortest("gpu_s", self.gpu_s)?;
        o.int53("wall_ms", int53(self.wall_ms)?)
    }
}

/// What the session has left, one non-negative amount per named unit (`<unit>_left`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Budget {
    left: Vec<(String, f64)>,
}

impl Budget {
    #[must_use]
    pub const fn new() -> Self {
        Self { left: Vec::new() }
    }

    /// Add `<name>` (which must end in `_left`) with its remaining amount.
    pub fn with(mut self, name: &str, amount: f64) -> Result<Self, EnvelopeError> {
        let stem = name.strip_suffix("_left").unwrap_or("");
        let mut chars = stem.chars();
        let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if !ok {
            return Err(EnvelopeError::InvalidField {
                field: "budget",
                reason: format!("`{name}` is not `<unit>_left`"),
            });
        }
        finite_non_negative("budget", amount)?;
        if self.left.iter().any(|(n, _)| n == name) {
            return Err(EnvelopeError::InvalidField {
                field: "budget",
                reason: format!("`{name}` given twice"),
            });
        }
        self.left.push((name.to_owned(), amount));
        Ok(self)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.left.is_empty()
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        for (name, amount) in &self.left {
            o.f64_shortest(name, *amount)?;
        }
        Ok(())
    }
}

/// A machine-readable note that does not change `exit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    code: String,
    message: String,
}

impl Warning {
    pub fn new(code: &str, message: &str) -> Result<Self, EnvelopeError> {
        slug("warnings.code", code)?;
        non_empty("warnings.message", message)?;
        Ok(Self {
            code: code.to_owned(),
            message: message.to_owned(),
        })
    }

    /// `executor: in-process`: a CPU-tier verb ran without the daemon.
    #[must_use]
    pub fn executor_in_process() -> Self {
        Self {
            code: "executor-in-process".to_owned(),
            message: "executor: in-process (the verb ran without the daemon)".to_owned(),
        }
    }
}

/// A ranked `{command, why}` suggestion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NextAction {
    command: String,
    why: String,
}

impl NextAction {
    pub fn new(command: &str, why: &str) -> Result<Self, EnvelopeError> {
        non_empty("next_actions.command", command)?;
        non_empty("next_actions.why", why)?;
        Ok(Self {
            command: command.to_owned(),
            why: why.to_owned(),
        })
    }
}

/// One resolved `@fixture:` reference (N34p3-01 fills these).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureBinding {
    param: String,
    reference: String,
    path: String,
    digest: Blake3,
}

impl FixtureBinding {
    pub fn new(
        param: &str,
        reference: &str,
        path: &str,
        digest: Blake3,
    ) -> Result<Self, EnvelopeError> {
        non_empty("bound.fixtures.param", param)?;
        no_controls("bound.fixtures.reference", reference)?;
        if !reference.starts_with("@fixture:") || reference.len() == "@fixture:".len() {
            return Err(EnvelopeError::InvalidField {
                field: "bound.fixtures.reference",
                reason: format!("`{reference}` is not `@fixture:<name>`"),
            });
        }
        non_empty("bound.fixtures.path", path)?;
        if path.starts_with('/') {
            return Err(EnvelopeError::InvalidField {
                field: "bound.fixtures.path",
                reason: "must be relative to the workspace".into(),
            });
        }
        Ok(Self {
            param: param.to_owned(),
            reference: reference.to_owned(),
            path: path.to_owned(),
            digest,
        })
    }
}

/// `bound`: the immutable identifiers every mutable name was resolved to (ADR-TETHER-11),
/// and `fixtures` once the Resolve step has run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bound {
    names: Vec<(String, String)>,
    fixtures: Option<Vec<FixtureBinding>>,
}

impl Bound {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            names: Vec::new(),
            fixtures: None,
        }
    }

    /// Bind a mutable name to the immutable identifier it resolved to.
    pub fn bind(mut self, name: &str, identifier: &str) -> Result<Self, EnvelopeError> {
        non_empty("bound", name)?;
        non_empty("bound", identifier)?;
        if name == "fixtures" || self.names.iter().any(|(n, _)| n == name) {
            return Err(EnvelopeError::InvalidField {
                field: "bound",
                reason: format!("`{name}` bound twice or reserved"),
            });
        }
        self.names.push((name.to_owned(), identifier.to_owned()));
        Ok(self)
    }

    /// Record the Resolve step's fixture bindings, in parameter order then file order.
    #[must_use]
    pub fn with_fixtures(mut self, fixtures: Vec<FixtureBinding>) -> Self {
        self.fixtures = Some(fixtures);
        self
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        if let Some(fx) = &self.fixtures {
            o.array("fixtures", |a| {
                for f in fx {
                    a.push_object(|fo| {
                        fo.str("digest", &f.digest.to_hex())?;
                        fo.str("param", &f.param)?;
                        fo.str("path", &f.path)?;
                        fo.str("reference", &f.reference)
                    })?;
                }
                Ok(())
            })?;
        }
        for (name, id) in &self.names {
            o.str(name, id)?;
        }
        Ok(())
    }
}

/// Pagination: the committed `frontier` every page is pinned to, and the `cursor` of the
/// next page while one remains. A long answer is paginated and says so; never cut silently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    cursor: Option<String>,
    frontier: String,
}

impl Page {
    pub fn new(frontier: &str, cursor: Option<&str>) -> Result<Self, EnvelopeError> {
        non_empty("frontier", frontier)?;
        if let Some(c) = cursor {
            non_empty("cursor", c)?;
        }
        Ok(Self {
            cursor: cursor.map(str::to_owned),
            frontier: frontier.to_owned(),
        })
    }

    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        if let Some(c) = &self.cursor {
            o.str("cursor", c)?;
        }
        o.str("frontier", &self.frontier)
    }
}

/// The constant-`false` flags of an exporting verb (BD-005's types; BD-408's packet form).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authorization {
    Artifact(NoAuthorization),
    BuildPacket(PacketAuthorization),
}

impl Authorization {
    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        let base = match self {
            Self::Artifact(a) => *a,
            Self::BuildPacket(p) => {
                o.bool("fabrication_released", p.fabrication_released.as_bool())?;
                p.base
            }
        };
        o.bool("execution_authorized", base.execution_authorized.as_bool())?;
        o.bool(
            "hardware_safety_signoff",
            base.hardware_safety_signoff.as_bool(),
        )?;
        o.bool(
            "sim_to_real_certified",
            base.sim_to_real_certified.as_bool(),
        )
    }
}

pub(crate) fn write_warning(w: &Warning, o: &mut Object<'_>) -> Result<(), CanonError> {
    o.str("code", &w.code)?;
    o.str("message", &w.message)
}

pub(crate) fn write_next_action(n: &NextAction, o: &mut Object<'_>) -> Result<(), CanonError> {
    o.str("command", &n.command)?;
    o.str("why", &n.why)
}
