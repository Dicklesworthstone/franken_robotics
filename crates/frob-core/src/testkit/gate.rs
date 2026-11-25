//! No caught control, no pass: `GateReport` and `GateLine` (STD-TEST §8).

use std::fmt;
use std::path::PathBuf;

use crate::testkit::evidence::{EvidenceKind, EvidenceRecord, EvidenceStatus, emit_if_configured, emit_to_dir};
use crate::testkit::registry::{ControlOutcome, exact_compare};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp { Lt, Le, Gt, Ge, Eq }

impl CmpOp {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self { Self::Lt => "<", Self::Le => "<=", Self::Gt => ">", Self::Ge => ">=", Self::Eq => "==" }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Threshold { pub op: CmpOp, pub value: String }

impl Threshold {
    #[must_use] pub fn new(op: CmpOp, value: impl Into<String>) -> Self { Self { op, value: value.into() } }
    #[must_use] pub fn le(value: impl Into<String>) -> Self { Self::new(CmpOp::Le, value) }
    #[must_use] pub fn lt(value: impl Into<String>) -> Self { Self::new(CmpOp::Lt, value) }
    #[must_use] pub fn ge(value: impl Into<String>) -> Self { Self::new(CmpOp::Ge, value) }
    #[must_use] pub fn gt(value: impl Into<String>) -> Self { Self::new(CmpOp::Gt, value) }
    #[must_use] pub fn eq(value: impl Into<String>) -> Self { Self::new(CmpOp::Eq, value) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement { pub name: String, pub value: String, pub unit: String, pub threshold: Threshold, pub met: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotEligibleReason { NoControl, StaleControl(String) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateOutcome { Pass, Fail, NotEligible(NotEligibleReason), NoClaim(String) }

/// Sealed gate verdict line; has no public constructor.
///
/// ```compile_fail
/// use frob_core::testkit::{GateLine, GateOutcome};
/// let _ = GateLine { gate: "GATE-DEMO-01".into(), outcome: GateOutcome::Pass, _sealed: () };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateLine { gate: String, outcome: GateOutcome, _sealed: () }

impl GateLine {
    #[must_use] pub fn gate(&self) -> &str { &self.gate }
    #[must_use] pub fn outcome(&self) -> &GateOutcome { &self.outcome }
    #[must_use] pub const fn is_pass(&self) -> bool { matches!(self.outcome, GateOutcome::Pass) }
    #[must_use] pub const fn is_no_claim(&self) -> bool { matches!(self.outcome, GateOutcome::NoClaim(_)) }
}

impl fmt::Display for GateLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.outcome {
            GateOutcome::Pass => f.write_str("pass"),
            GateOutcome::Fail => f.write_str("fail"),
            GateOutcome::NotEligible(NotEligibleReason::StaleControl(id)) => write!(f, "not-eligible: control {id} stale"),
            GateOutcome::NotEligible(NotEligibleReason::NoControl) => f.write_str("not-eligible: no control caught in this run"),
            GateOutcome::NoClaim(reason) => write!(f, "no-claim: {reason}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GateReport {
    gate: String,
    measurements: Vec<Measurement>,
    controls: Vec<(String, ControlOutcome)>,
    no_claim: Option<String>,
    evidence_dir: Option<PathBuf>,
}

impl GateReport {
    #[must_use]
    pub fn new(gate: impl Into<String>) -> Self {
        Self { gate: gate.into(), measurements: Vec::new(), controls: Vec::new(), no_claim: None, evidence_dir: None }
    }

    #[must_use]
    pub fn evidence_dir(mut self, dir: impl Into<PathBuf>) -> Self { self.evidence_dir = Some(dir.into()); self }

    #[must_use]
    pub fn measured(mut self, name: impl Into<String>, value: impl Into<String>, unit: impl Into<String>, threshold: Threshold) -> Self {
        let val = value.into();
        let met = exact_compare(&val, threshold.op, &threshold.value).unwrap_or(false);
        self.measurements.push(Measurement { name: name.into(), value: val, unit: unit.into(), threshold, met });
        self
    }

    #[must_use]
    pub fn control(mut self, id: impl Into<String>, outcome: ControlOutcome) -> Self {
        self.controls.push((id.into(), outcome));
        self
    }

    #[must_use]
    pub fn no_claim(mut self, reason: impl Into<String>) -> Self { self.no_claim = Some(reason.into()); self }

    #[must_use]
    pub fn finish(self) -> GateLine {
        let outcome = if let Some(reason) = self.no_claim {
            if !self.measurements.is_empty() || !self.controls.is_empty() || reason.trim().is_empty() {
                GateOutcome::Fail
            } else {
                GateOutcome::NoClaim(reason)
            }
        } else if self.measurements.iter().any(|m| !m.met)
            || self.controls.iter().any(|(_, o)| *o == ControlOutcome::Missed)
        {
            GateOutcome::Fail
        } else if let Some(id) = self.controls.iter().find_map(|(id, o)| (*o == ControlOutcome::Stale).then(|| id.clone())) {
            GateOutcome::NotEligible(NotEligibleReason::StaleControl(id))
        } else if !self.controls.iter().any(|(_, o)| *o == ControlOutcome::Caught) {
            GateOutcome::NotEligible(NotEligibleReason::NoControl)
        } else {
            GateOutcome::Pass
        };

        let emit = |rec: &EvidenceRecord| match &self.evidence_dir {
            Some(d) => { let _ = emit_to_dir(d, rec); }
            None => emit_if_configured(rec),
        };
        if let GateOutcome::NoClaim(reason) = &outcome {
            let mut rec = EvidenceRecord::new(EvidenceKind::Gate, &self.gate, EvidenceStatus::NoClaim);
            rec.reason = Some(reason.clone());
            emit(&rec);
            return GateLine { gate: self.gate, outcome, _sealed: () };
        }
        for (cid, co) in &self.controls {
            let st = match co {
                ControlOutcome::Caught => EvidenceStatus::Caught,
                ControlOutcome::Missed => EvidenceStatus::Missed,
                ControlOutcome::Stale => EvidenceStatus::Stale,
                ControlOutcome::Obligation => EvidenceStatus::Obligation,
            };
            let mut rec = EvidenceRecord::new(EvidenceKind::Nc, cid, st);
            rec.expected = Some(self.gate.clone());
            emit(&rec);
        }
        let (st, reason) = match &outcome {
            GateOutcome::Pass => (EvidenceStatus::Pass, None),
            GateOutcome::Fail => (EvidenceStatus::Fail, None),
            GateOutcome::NotEligible(NotEligibleReason::NoControl) => (EvidenceStatus::NotEligible, Some("no-control".to_owned())),
            GateOutcome::NotEligible(NotEligibleReason::StaleControl(_)) => (EvidenceStatus::NotEligible, Some("stale-control".to_owned())),
            GateOutcome::NoClaim(_) => unreachable!(),
        };
        let mut rec = EvidenceRecord::new(EvidenceKind::Gate, &self.gate, st);
        rec.reason = reason;
        if let Some(m) = self.measurements.first() {
            rec.observed = Some(m.value.clone());
            rec.expected = Some(format!("{} {}", m.threshold.op.as_str(), m.threshold.value));
            rec.unit = Some(m.unit.clone());
        }
        emit(&rec);
        GateLine { gate: self.gate, outcome, _sealed: () }
    }
}
