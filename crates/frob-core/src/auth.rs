//! Single-inhabitant constant-`false` no-authorization flags (BD-005, INV-SAFE-05).
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::fmt;
use crate::refusal::{Refusal, RefusalCode, Repair, RetryPolicy, mini_json};

/// Unit type whose only value serializes as JSON literal `false`.
///
/// ```compile_fail,E0277
/// use frob_core::auth::False;
/// let _: False = true.into();
/// ```
///
/// ```compile_fail,E0308
/// use frob_core::auth::{False, NoAuthorization};
/// let _ = NoAuthorization { execution_authorized: true, hardware_safety_signoff: False::VALUE, sim_to_real_certified: False::VALUE };
/// ```
///
/// ```compile_fail,E0308
/// use frob_core::auth::NoAuthorization;
/// let mut flags = NoAuthorization::new();
/// flags.hardware_safety_signoff = true;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct False(());

impl False {
    pub const VALUE: Self = Self(());
    #[must_use] pub const fn new() -> Self { Self(()) }
    #[must_use] pub const fn as_bool(self) -> bool { false }
    #[must_use] pub const fn to_json(self) -> &'static str { "false" }
}
impl fmt::Display for False { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("false") } }

/// Three constant-`false` no-authorization flags carried by every exported artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NoAuthorization {
    pub execution_authorized: False,
    pub hardware_safety_signoff: False,
    pub sim_to_real_certified: False,
}

impl NoAuthorization {
    pub const VALUE: Self = Self::new();
    #[must_use] pub const fn new() -> Self { Self { execution_authorized: False::VALUE, hardware_safety_signoff: False::VALUE, sim_to_real_certified: False::VALUE } }
    #[must_use] pub fn to_json(&self) -> String {
        format!("{{\"execution_authorized\":{},\"hardware_safety_signoff\":{},\"sim_to_real_certified\":{}}}",
            self.execution_authorized.to_json(), self.hardware_safety_signoff.to_json(), self.sim_to_real_certified.to_json())
    }
    pub fn parse_json(input: &str) -> Result<Self, Refusal> {
        parse_exact_false_fields(input, &["execution_authorized", "hardware_safety_signoff", "sim_to_real_certified"])?;
        Ok(Self::VALUE)
    }
}

/// Build-packet authorization flags: the three base flags plus `fabrication_released: false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PacketAuthorization {
    pub base: NoAuthorization,
    pub fabrication_released: False,
}

impl PacketAuthorization {
    pub const VALUE: Self = Self::new();
    #[must_use] pub const fn new() -> Self { Self { base: NoAuthorization::VALUE, fabrication_released: False::VALUE } }
    #[must_use] pub fn to_json(&self) -> String {
        format!("{{\"execution_authorized\":{},\"fabrication_released\":{},\"hardware_safety_signoff\":{},\"sim_to_real_certified\":{}}}",
            self.base.execution_authorized.to_json(), self.fabrication_released.to_json(),
            self.base.hardware_safety_signoff.to_json(), self.base.sim_to_real_certified.to_json())
    }
    pub fn parse_json(input: &str) -> Result<Self, Refusal> {
        parse_exact_false_fields(input, &["execution_authorized", "fabrication_released", "hardware_safety_signoff", "sim_to_real_certified"])?;
        Ok(Self::VALUE)
    }
}

fn flag_refusal(pointer: impl Into<String>, observed: impl Into<String>, required: impl Into<String>) -> Refusal {
    Refusal::new(
        RefusalCode::FrobRef816, "artifact authorization flags are missing or not false",
        pointer, observed, required, "", RetryPolicy::AfterChange,
        vec![Repair::new("Include all required no-authorization flags set to false"), Repair::new("Regenerate the artifact through a FrankenRobotics exporter")],
    )
}

fn parse_exact_false_fields(input: &str, required: &[&str]) -> Result<(), Refusal> {
    let val = mini_json::parse(input).map_err(|e| flag_refusal("/authorization", e, "JSON object with constant-false authorization flags"))?;
    let mut map = match val { mini_json::JsonVal::Object(m) => m, _ => return Err(flag_refusal("/authorization", "non-object", "JSON object")) };
    for &field in required {
        match map.remove(field) {
            Some(mini_json::JsonVal::Bool(false)) => {}
            Some(mini_json::JsonVal::Bool(true)) => return Err(flag_refusal(format!("/{field}"), "true", "false")),
            Some(mini_json::JsonVal::Str(s)) => return Err(flag_refusal(format!("/{field}"), format!("\"{s}\""), "false")),
            Some(mini_json::JsonVal::Null) => return Err(flag_refusal(format!("/{field}"), "null", "false")),
            Some(_) => return Err(flag_refusal(format!("/{field}"), "non-boolean", "false")),
            None => return Err(flag_refusal(format!("/{field}"), "missing", "false")),
        }
    }
    if let Some((extra, _)) = map.into_iter().next() {
        return Err(flag_refusal(format!("/{extra}"), "unexpected field", "only declared no-authorization flags"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_serialize_three_false() {
        let a = NoAuthorization::new();
        assert!(!a.execution_authorized.as_bool() && !a.hardware_safety_signoff.as_bool() && !a.sim_to_real_certified.as_bool());
        let json = a.to_json();
        assert_eq!(json, r#"{"execution_authorized":false,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#);
        assert_eq!(NoAuthorization::parse_json(&json).unwrap(), NoAuthorization::VALUE);
        let pjson = PacketAuthorization::new().to_json();
        assert_eq!(pjson, r#"{"execution_authorized":false,"fabrication_released":false,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#);
        assert_eq!(PacketAuthorization::parse_json(&pjson).unwrap(), PacketAuthorization::VALUE);
    }

    #[test]
    fn flags_parse_rejects_true_and_missing() {
        for raw in [
            r#"{"execution_authorized":true,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#,
            r#"{"execution_authorized":false,"hardware_safety_signoff":true,"sim_to_real_certified":false}"#,
            r#"{"execution_authorized":false,"hardware_safety_signoff":false,"sim_to_real_certified":true}"#,
            r#"{"hardware_safety_signoff":false,"sim_to_real_certified":false}"#,
            r#"{"execution_authorized":false,"sim_to_real_certified":false}"#,
            r#"{"execution_authorized":false,"hardware_safety_signoff":false}"#,
            r#"{}"#,
            r#"{"execution_authorized":"false","hardware_safety_signoff":false,"sim_to_real_certified":false}"#,
            r#"{"execution_authorized":null,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#,
            r#"{"execution_authorized":false,"hardware_safety_signoff":false,"sim_to_real_certified":false,"extra":false}"#,
            r#"{"execution_authorized":false,"execution_authorized":false,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#,
        ] {
            assert_eq!(NoAuthorization::parse_json(raw).expect_err(raw).code, RefusalCode::FrobRef816);
        }
        assert_eq!(PacketAuthorization::parse_json(&NoAuthorization::new().to_json()).unwrap_err().code, RefusalCode::FrobRef816);
        let true_fab = r#"{"execution_authorized":false,"fabrication_released":true,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#;
        assert_eq!(PacketAuthorization::parse_json(true_fab).unwrap_err().code, RefusalCode::FrobRef816);
    }
}
