#![forbid(unsafe_code)]
//! Independent std-only certificate auditor crate for FrankenRobotics (BD-026).
//!
//! This crate depends strictly on the Rust standard library and shares zero
//! source code with any proposer crate (`frob-core`, `frob-lemma`, `frob-mission`,
//! or any sibling repository). All arithmetic is exact (`BigInt`, `Rational`, and
//! outward-rounded dyadic `Interval`) with zero floating-point types.

#[cfg(all(
    any(
        feature = "nc-skip-key-order",
        feature = "nc-interval-mul-nearest",
        feature = "nc-dup-key-first-wins",
    ),
    any(not(feature = "gauntlet-mutations"), not(debug_assertions))
))]
compile_error!(
    "frob-cert negative-control nc-* features require debug builds with --features gauntlet-mutations"
);

pub mod bigint;
pub mod blake3;
pub mod interval;
pub mod json;
pub mod limits;
pub mod rational;

use crate::interval::INTERVAL_IMPL_ID;
use crate::json::{JsonValue, parse_canonical, serialize_canonical};
use crate::limits::{COMPILED_LIMITS, Limits, RULE_KIND_UNKNOWN};

/// Binary name reported in every trusted-base statement.
pub const AUDITOR_NAME: &str = "frob-verify";
/// Crate version reported in every trusted-base statement.
pub const AUDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Trusted-base statement included in every audit report (BD-026).
pub const TRUSTED_BASE_LINE: &str = concat!(
    "trusted base: frob-verify ",
    env!("CARGO_PKG_VERSION"),
    "; Rust standard library; no third-party dependencies"
);

/// Auditor verdict returned for each audited input (BD-026 item 8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Input passed canonical JSON, hashing, and applicable checks.
    Accept,
    /// Input failed a check; names the first failing rule identifier.
    Reject {
        rule: &'static str,
        refusal: Option<&'static str>,
        path: String,
        message: String,
    },
    /// Recognised kind whose checker or required auxiliary input is not enabled.
    Unsupported { kind: String, message: String },
}

impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject { .. } => "reject",
            Self::Unsupported { .. } => "unsupported",
        }
    }

    pub fn is_accept(&self) -> bool {
        matches!(self, Self::Accept)
    }

    pub fn is_reject(&self) -> bool {
        matches!(self, Self::Reject { .. })
    }

    pub fn is_unsupported(&self) -> bool {
        matches!(self, Self::Unsupported { .. })
    }
}

/// Complete deterministic audit report for one input byte stream (class D0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReport {
    /// Plain BLAKE3 digest of the input bytes (64 lowercase hex characters).
    pub blake3_hex: String,
    /// Schema-domain BLAKE3 derive-key digest when a string `"schema"` field is present.
    pub domain_blake3_hex: Option<String>,
    /// Value of `"schema"` if present as a string on the root object.
    pub schema: Option<String>,
    /// Extracted proof kind (`proof.kind` or top-level `kind`) if present.
    pub kind: Option<String>,
    /// Final verdict (`accept`, `reject`, or `unsupported`).
    pub verdict: Verdict,
    /// Compiled resource limits active during this audit.
    pub limits: Limits,
    /// Interval arithmetic implementation identifier.
    pub interval_impl: &'static str,
    /// Trusted computing base statement.
    pub trusted_base: &'static str,
    /// Canonical re-serialized bytes when input passed canonical JSON validation.
    pub canonical_bytes: Option<Vec<u8>>,
}

/// Audit raw input bytes under `COMPILED_LIMITS`, dispatching proof kinds on the M0 closed registry.
pub fn audit_bytes(bytes: &[u8]) -> AuditReport {
    audit_bytes_with_options(bytes, COMPILED_LIMITS, false)
}

/// Audit raw input bytes under custom `limits` with optional `canonical_only` mode.
pub fn audit_bytes_with_options(
    bytes: &[u8],
    limits: Limits,
    canonical_only: bool,
) -> AuditReport {
    let blake3_hex = blake3::hash_hex(bytes);
    let parsed = match parse_canonical(bytes, &limits) {
        Ok(v) => v,
        Err(e) => {
            return AuditReport {
                blake3_hex,
                domain_blake3_hex: None,
                schema: None,
                kind: None,
                verdict: Verdict::Reject {
                    rule: e.rule,
                    refusal: e.refusal,
                    path: e.path,
                    message: e.message,
                },
                limits,
                interval_impl: INTERVAL_IMPL_ID,
                trusted_base: TRUSTED_BASE_LINE,
                canonical_bytes: None,
            };
        }
    };

    let canonical_bytes = serialize_canonical(&parsed).into_bytes();
    let schema = parsed
        .get("schema")
        .and_then(JsonValue::as_str)
        .map(str::to_string);
    let domain_blake3_hex = schema
        .as_deref()
        .map(|s| blake3::domain_hash_hex(s, &canonical_bytes));

    let (kind, verdict) = if canonical_only {
        (extract_kind(&parsed).ok().flatten(), Verdict::Accept)
    } else {
        dispatch_m0_kind(&parsed, schema.as_deref())
    };

    AuditReport {
        blake3_hex,
        domain_blake3_hex,
        schema,
        kind,
        verdict,
        limits,
        interval_impl: INTERVAL_IMPL_ID,
        trusted_base: TRUSTED_BASE_LINE,
        canonical_bytes: Some(canonical_bytes),
    }
}

fn extract_kind(val: &JsonValue) -> Result<Option<String>, (&'static str, String, String)> {
    if let Some(proof) = val.get("proof") {
        let Some(k) = proof.get("kind").and_then(JsonValue::as_str) else {
            return Err((
                RULE_KIND_UNKNOWN,
                "$.proof.kind".to_string(),
                "proof object is missing string field 'kind'".to_string(),
            ));
        };
        return Ok(Some(k.to_string()));
    }
    if let Some(k_val) = val.get("kind") {
        let Some(k) = k_val.as_str() else {
            return Err((
                RULE_KIND_UNKNOWN,
                "$.kind".to_string(),
                "field 'kind' must be a string".to_string(),
            ));
        };
        return Ok(Some(k.to_string()));
    }
    Ok(None)
}

fn dispatch_m0_kind(val: &JsonValue, schema: Option<&str>) -> (Option<String>, Verdict) {
    match extract_kind(val) {
        Err((rule, path, message)) => (
            None,
            Verdict::Reject {
                rule,
                refusal: None,
                path,
                message,
            },
        ),
        Ok(Some(kind)) => {
            // M0 closed registry (BD-026 item 8): no proof checkers are registered yet;
            // every unknown proof kind is rejected with `kind.unknown`.
            (
                Some(kind.clone()),
                Verdict::Reject {
                    rule: RULE_KIND_UNKNOWN,
                    refusal: None,
                    path: if val.get("proof").is_some() {
                        "$.proof.kind".to_string()
                    } else {
                        "$.kind".to_string()
                    },
                    message: format!("unknown proof kind {kind:?} in M0 registry"),
                },
            )
        }
        Ok(None) => {
            if schema == Some("frob.cert/1") {
                return (
                    None,
                    Verdict::Reject {
                        rule: RULE_KIND_UNKNOWN,
                        refusal: None,
                        path: "$.proof.kind".to_string(),
                        message: "frob.cert/1 bundle is missing proof.kind".to_string(),
                    },
                );
            }
            // Plain canonical JSON payload without a proof-kind envelope passes
            // canonical JSON + BLAKE3 verification at the M0 floor.
            (None, Verdict::Accept)
        }
    }
}
