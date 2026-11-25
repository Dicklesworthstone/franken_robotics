//! `Envelope` and its canonical serializer (N32a-03).
//!
//! Constructors fix the outcome and the exit: `completed` (0), `completed_with_findings`
//! (2), `from_refusal` (the exit of the code's registry row), `deferred` (8, refusing a
//! progress without a denominator) and `internal_fault` (`FROB-REF-990`, 9). There is no
//! constructor that takes an exit or an evidence class. The serializer is the envelope's
//! stamping site of N33a-06: every envelope, refusals included, carries
//! `evidence.stack_receipt` (and `stack_dev: true` on a dev build) from `BUILT_UNDER`.

use std::sync::OnceLock;

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Object, Value};
use frob_core::refusal::{Refusal, RefusalFingerprint, Repair, RetryPolicy};
use frob_core::registry::refusals::{RefusalRegistry, RefusalRegistryError};
use frob_core::stack::{BUILT_UNDER, StackIdentity};

use super::blocks::{
    Authorization, Bound, Budget, Completion, Cost, JobHandle, JobReport, JsonBody, NextAction,
    Page, RefusalBlock, RefusalFields, Warning, write_next_action, write_warning,
};
use super::evidence::EvidenceBlock;
use super::exit::{ExitCode, exit_for_code};
use super::{ENVELOPE_SCHEMA_ID, EnvelopeError, INTERNAL_FAULT_CODE};

/// What the verb did.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// Exit 0 or 2: the result, with or without findings.
    Completed {
        result: JsonBody,
        completion: Completion,
    },
    /// Exit 3 to 7 or 9: refused; nothing partial is published as a result.
    Refused(RefusalBlock),
    /// Exit 8: accepted and still running.
    Deferred(JobHandle),
}

/// One `frob.envelope/1` answer.
#[derive(Debug, Clone, PartialEq)]
pub struct Envelope {
    verb: String,
    outcome: Outcome,
    evidence: Option<EvidenceBlock>,
    cost: Cost,
    bound: Option<Bound>,
    budget: Option<Budget>,
    warnings: Vec<Warning>,
    next_actions: Vec<NextAction>,
    page: Option<Page>,
    idempotent_replay: bool,
    authorization: Option<Authorization>,
    job: Option<JobHandle>,
    delta: Option<JsonBody>,
    stack: StackIdentity,
}

static EMBEDDED: OnceLock<Result<RefusalRegistry, RefusalRegistryError>> = OnceLock::new();

/// The refusal registry compiled into this binary (`registries/refusals.toml`).
pub fn embedded_registry() -> Result<&'static RefusalRegistry, EnvelopeError> {
    EMBEDDED
        .get_or_init(RefusalRegistry::load)
        .as_ref()
        .map_err(|e| EnvelopeError::Registry(e.to_string()))
}

fn check_verb(verb: &str) -> Result<(), EnvelopeError> {
    if verb.chars().any(|c| (c as u32) < 0x20 || c == '\u{7f}') {
        return Err(EnvelopeError::InvalidField {
            field: "verb",
            reason: "control characters are not allowed".into(),
        });
    }
    Ok(())
}

impl Envelope {
    fn base(
        verb: &str,
        outcome: Outcome,
        evidence: Option<EvidenceBlock>,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        check_verb(verb)?;
        cost.check()?;
        Ok(Self {
            verb: verb.to_owned(),
            outcome,
            evidence,
            cost,
            bound: None,
            budget: None,
            warnings: Vec::new(),
            next_actions: Vec::new(),
            page: None,
            idempotent_replay: false,
            authorization: None,
            job: None,
            delta: None,
            stack: BUILT_UNDER,
        })
    }

    /// Exit 0: completed, nothing to flag. The result travels with its claim product.
    pub fn completed(
        verb: &str,
        result: JsonBody,
        evidence: EvidenceBlock,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        Self::base(
            verb,
            Outcome::Completed {
                result,
                completion: Completion::Clean,
            },
            Some(evidence),
            cost,
        )
    }

    /// Exit 2: completed with findings or flags.
    pub fn completed_with_findings(
        verb: &str,
        result: JsonBody,
        evidence: EvidenceBlock,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        Self::base(
            verb,
            Outcome::Completed {
                result,
                completion: Completion::WithFindings,
            },
            Some(evidence),
            cost,
        )
    }

    /// A refusal; the exit is the `exit` field of the code's row in the embedded registry.
    pub fn from_refusal(verb: &str, refusal: &Refusal, cost: Cost) -> Result<Self, EnvelopeError> {
        Self::from_code_in_with_fingerprint(
            embedded_registry()?,
            verb,
            refusal.code.as_str(),
            RefusalFields::of(refusal),
            refusal.fingerprint.clone(),
            cost,
        )
    }

    /// A refusal named by its code text, resolved against the embedded registry (the same
    /// closed set as `RefusalCode`; an unregistered code is refused).
    pub fn from_code(
        verb: &str,
        code: &str,
        fields: RefusalFields,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        Self::from_code_in(embedded_registry()?, verb, code, fields, cost)
    }

    /// A Spark (`6xx`) refusal carrying its required `fingerprint: { device, digest | null }`.
    pub fn from_code_with_fingerprint(
        verb: &str,
        code: &str,
        fields: RefusalFields,
        fingerprint: RefusalFingerprint,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        Self::from_code_in_with_fingerprint(
            embedded_registry()?,
            verb,
            code,
            fields,
            Some(fingerprint),
            cost,
        )
    }

    /// As [`Self::from_code`] against a given registry (tests and synthetic registries).
    pub(crate) fn from_code_in(
        registry: &RefusalRegistry,
        verb: &str,
        code: &str,
        fields: RefusalFields,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        Self::from_code_in_with_fingerprint(registry, verb, code, fields, None, cost)
    }

    /// As [`Self::from_code_in`] carrying an optional `6xx` fingerprint block.
    pub(crate) fn from_code_in_with_fingerprint(
        registry: &RefusalRegistry,
        verb: &str,
        code: &str,
        fields: RefusalFields,
        fingerprint: Option<RefusalFingerprint>,
        cost: Cost,
    ) -> Result<Self, EnvelopeError> {
        let exit = exit_for_code(registry, code).map_err(EnvelopeError::ExitLookup)?;
        let block = RefusalBlock::from_parts_with_fingerprint(code, exit, fields, fingerprint)?;
        Self::base(verb, Outcome::Refused(block), None, cost)
    }

    /// Exit 8: accepted and still running. A progress without a denominator is refused.
    pub fn deferred(verb: &str, job: &JobReport, cost: Cost) -> Result<Self, EnvelopeError> {
        let handle = JobHandle::new(job)?;
        Self::base(verb, Outcome::Deferred(handle), None, cost)
    }

    /// Exit 9, `FROB-REF-990` (`internal-fault`): a handler panicked or returned an error
    /// that maps to no code. Infallible, so the one-envelope rule holds on every path.
    ///
    /// `9xx` is fixed to exit 9 by the surface table (frob-core's loader refuses any other
    /// exit or a retry other than `never` for a `9xx` row), so the exit does not depend on
    /// whether the registry already carries the row (N32-REF enters it).
    #[must_use]
    pub fn internal_fault(fault: &FaultContext, cost: Cost) -> Self {
        let verb: String = fault
            .verb
            .chars()
            .map(|c| {
                if (c as u32) < 0x20 || c == '\u{7f}' {
                    '\u{fffd}'
                } else {
                    c
                }
            })
            .collect();
        let mut repairs = Vec::new();
        if let Some(key) = &fault.idempotency_key {
            repairs.push(Repair::with_detail(
                format!("frob job find --verb {verb} --key {key}"),
                "A job may have been admitted before the fault; recover it by key.",
            ));
        }
        repairs.push(Repair::with_detail(
            "frob doctor",
            "Check the toolchain, pins, devices and caches.",
        ));
        let observed = if fault.diagnostic_path.is_empty() {
            "<no diagnostic file>".to_owned()
        } else {
            fault.diagnostic_path.clone()
        };
        let fields = RefusalFields {
            message: "internal fault: the handler panicked or returned an error that maps to no refusal code".to_owned(),
            pointer: verb.clone(),
            observed,
            required: "a handler that returns an envelope".to_owned(),
            unit: String::new(),
            detail: None,
            affected_claims: Vec::new(),
            retry: RetryPolicy::Never,
            repairs,
        };
        let cost = if cost.check().is_ok() {
            cost
        } else {
            Cost::ZERO
        };
        Self {
            verb,
            outcome: Outcome::Refused(RefusalBlock {
                code: INTERNAL_FAULT_CODE.to_owned(),
                exit: ExitCode::Indeterminate,
                fields,
                fingerprint: None,
            }),
            evidence: None,
            cost,
            bound: None,
            budget: None,
            warnings: Vec::new(),
            next_actions: Vec::new(),
            page: None,
            idempotent_replay: false,
            authorization: None,
            job: None,
            delta: None,
            stack: BUILT_UNDER,
        }
    }

    /// Record what mutable names resolved to (and, after Resolve, `bound.fixtures`).
    #[must_use]
    pub fn with_bound(mut self, bound: Bound) -> Self {
        self.bound = Some(bound);
        self
    }

    /// Record what the session has left.
    pub fn with_budget(mut self, budget: Budget) -> Result<Self, EnvelopeError> {
        if budget.is_empty() {
            return Err(EnvelopeError::InvalidField {
                field: "budget",
                reason: "an empty budget names no unit".into(),
            });
        }
        self.budget = Some(budget);
        Ok(self)
    }

    #[must_use]
    pub fn with_warning(mut self, warning: Warning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Append the next ranked next action (call in rank order).
    #[must_use]
    pub fn with_next_action(mut self, action: NextAction) -> Self {
        self.next_actions.push(action);
        self
    }

    /// Mark a page of a paginated result.
    pub fn with_page(mut self, page: Page) -> Result<Self, EnvelopeError> {
        if !matches!(self.outcome, Outcome::Completed { .. }) {
            return Err(EnvelopeError::NotApplicable {
                what: "cursor/frontier",
                reason: "only a completed result is paginated",
            });
        }
        self.page = Some(page);
        Ok(self)
    }

    /// Mark this envelope as the stored answer to an earlier request with the same key.
    #[must_use]
    pub const fn replayed(mut self) -> Self {
        self.idempotent_replay = true;
        self
    }

    /// The constant-`false` flags of an exporting verb.
    #[must_use]
    pub const fn with_authorization(mut self, flags: Authorization) -> Self {
        self.authorization = Some(flags);
        self
    }

    /// A job block beside a completed result (job verbs) or a refusal (a budget stop's
    /// acknowledgement); an exit-8 envelope already carries its handle.
    pub fn with_job(mut self, job: &JobReport) -> Result<Self, EnvelopeError> {
        if matches!(self.outcome, Outcome::Deferred(_)) {
            return Err(EnvelopeError::NotApplicable {
                what: "job",
                reason: "a deferred envelope already carries its handle",
            });
        }
        self.job = Some(JobHandle::new(job)?);
        Ok(self)
    }

    /// After a run: what changed since the compared run (DESIGN field names).
    pub fn with_delta(mut self, delta: JsonBody) -> Result<Self, EnvelopeError> {
        if delta.tree().as_object().is_none() {
            return Err(EnvelopeError::InvalidField {
                field: "delta",
                reason: "must be an object".into(),
            });
        }
        self.delta = Some(delta);
        Ok(self)
    }

    /// Serialize under another stack identity (golden files use a fixed synthetic receipt
    /// so their bytes do not move with `receipts/HEAD`; production never calls this).
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn restamped(mut self, stack: StackIdentity) -> Self {
        self.stack = stack;
        self
    }

    #[must_use]
    pub fn verb(&self) -> &str {
        &self.verb
    }

    #[must_use]
    pub const fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    /// The exit; equals the process exit code on the CLI.
    #[must_use]
    pub const fn exit(&self) -> ExitCode {
        match &self.outcome {
            Outcome::Completed { completion, .. } => completion.exit(),
            Outcome::Refused(r) => r.exit,
            Outcome::Deferred(_) => ExitCode::Running,
        }
    }

    #[must_use]
    pub const fn ok(&self) -> bool {
        self.exit().is_ok()
    }

    #[must_use]
    pub const fn refusal(&self) -> Option<&RefusalBlock> {
        match &self.outcome {
            Outcome::Refused(r) => Some(r),
            _ => None,
        }
    }

    #[must_use]
    pub const fn stack(&self) -> &StackIdentity {
        &self.stack
    }

    /// Canonical bytes (sorted members, minimal escapes, exact numbers): equal envelopes
    /// are equal bytes.
    pub fn to_canonical(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    fn write_members(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        if let Some(a) = &self.authorization {
            o.object("authorization", |ao| a.write(ao))?;
        }
        if let Some(b) = &self.budget {
            o.object("budget", |bo| b.write(bo))?;
        }
        if let Some(b) = &self.bound {
            o.object("bound", |bo| b.write(bo))?;
        }
        o.object("cost", |co| self.cost.write(co))?;
        if let Some(p) = &self.page {
            p.write(o)?;
        }
        if let Some(d) = &self.delta {
            o.member_term("delta", d.tree())?;
        }
        o.object("evidence", |eo| {
            if let Some(ev) = &self.evidence {
                ev.write(eo)?;
            }
            self.stack.write_envelope_evidence_fields(eo)
        })?;
        o.int53("exit", i64::from(self.exit().code()))?;
        if self.idempotent_replay {
            o.bool("idempotent_replay", true)?;
        }
        let job = match &self.outcome {
            Outcome::Deferred(h) => Some(h),
            _ => self.job.as_ref(),
        };
        if let Some(j) = job {
            o.object("job", |jo| j.write(jo))?;
        }
        o.array("next_actions", |a| {
            for n in &self.next_actions {
                a.push_object(|no| write_next_action(n, no))?;
            }
            Ok(())
        })?;
        o.bool("ok", self.ok())?;
        match &self.outcome {
            Outcome::Refused(r) => o.object("refusal", |ro| r.write(ro))?,
            Outcome::Completed { result, .. } => o.member_term("result", result.tree())?,
            Outcome::Deferred(_) => {}
        }
        o.str("schema", ENVELOPE_SCHEMA_ID)?;
        o.str("verb", &self.verb)?;
        o.array("warnings", |a| {
            for w in &self.warnings {
                a.push_object(|wo| write_warning(w, wo))?;
            }
            Ok(())
        })
    }
}

impl Canonical for Envelope {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| self.write_members(o))
    }
}

/// What the top of `main` (or the wrapper around a handler call in a long-lived process)
/// knows when a handler faults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultContext {
    /// The verb whose handler faulted (`pointer` names it).
    pub verb: String,
    /// The call's idempotency key, when it carried one.
    pub idempotency_key: Option<String>,
    /// Where the diagnostic was written (`observed` holds it).
    pub diagnostic_path: String,
}
