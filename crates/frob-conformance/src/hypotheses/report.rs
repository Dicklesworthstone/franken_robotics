//! Exit-review report, `frob.prove.effects.out/1` certificate decision-effect read-out,
//! and H-22b known-answer formula (`N00-10` steps 5 & 6).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use frob_core::json::{self, Json, Limits, Spanned};

use crate::hypotheses::schema::HypothesisRegister;

/// Schema identifier of `frob prove list --effects --json` (`N12-12`).
pub const CERT_EFFECTS_SCHEMA: &str = "frob.prove.effects.out/1";

/// Disagreement ID tied to the certificate decision-effect ledger (`N00-10` step 6).
pub const CERT_DISAGREEMENT_ID: &str = "OD-08";

/// Report line emitted at M0 before the decision-effect ledger exists (`N00-10` test plan).
pub const NO_CERT_LEDGER_LINE: &str = "no decision-effect ledger yet (N12-12, M1)";

/// Fail-closed wording when consumed certificates have changed zero decisions (`N12-12`).
pub const ZERO_CHANGED_DEFAULT_TEXT: &str =
    "no consumed certificate has changed a decision; Lemma is evidence quality only";

/// Totals read from a `frob.prove.effects.out/1` JSON read-out (`N12-12`, `N00-10` step 6).
///
/// `unknown` is strictly separate and is never added to `changed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CertEffectTotals {
    pub consumed: u64,
    pub changed: u64,
    pub unchanged: u64,
    pub unknown: u64,
}

impl CertEffectTotals {
    /// Parse a `frob.prove.effects.out/1` JSON payload via [`frob_core::json::admit`].
    ///
    /// Accepts either `{"schema":"frob.prove.effects.out/1","totals":{...}}` or flat
    /// top-level counters, and verifies `consumed == changed + unchanged + unknown`.
    pub fn from_effects_json(bytes: &[u8]) -> Result<Self, String> {
        let admitted = json::admit(bytes, &Limits::default())
            .map_err(|e| format!("JSON admission error: {e}"))?;
        let top = obj_map(admitted.root(), "root")?;
        let schema = top
            .get("schema")
            .and_then(|s| s.value.as_str())
            .ok_or_else(|| "missing string `schema`".to_owned())?;
        if schema != CERT_EFFECTS_SCHEMA {
            return Err(format!(
                "expected schema `{CERT_EFFECTS_SCHEMA}`, got `{schema}`"
            ));
        }

        let counters = if let Some(totals_node) = top.get("totals") {
            obj_map(totals_node, "totals")?
        } else {
            top
        };

        let consumed = read_u64(&counters, "consumed")?;
        let changed = read_u64(&counters, "changed")?;
        let unchanged = read_u64(&counters, "unchanged")?;
        let unknown = read_u64(&counters, "unknown")?;

        let sum = changed
            .checked_add(unchanged)
            .and_then(|v| v.checked_add(unknown))
            .ok_or_else(|| "cert_effect totals overflowed u64".to_owned())?;
        if sum != consumed {
            return Err(format!(
                "cert_effect totals mismatch: changed ({changed}) + unchanged ({unchanged}) + unknown ({unknown}) = {sum} != consumed ({consumed})"
            ));
        }

        Ok(Self {
            consumed,
            changed,
            unchanged,
            unknown,
        })
    }
}

fn obj_map<'a>(
    node: &'a Spanned<'a>,
    label: &str,
) -> Result<BTreeMap<&'a str, &'a Spanned<'a>>, String> {
    let members = node
        .value
        .as_object()
        .ok_or_else(|| format!("expected JSON object at `{label}`"))?;
    let mut map = BTreeMap::new();
    for m in members {
        map.insert(m.name.as_str(), &m.value);
    }
    Ok(map)
}

fn read_u64(map: &BTreeMap<&str, &Spanned<'_>>, field: &str) -> Result<u64, String> {
    let node = map
        .get(field)
        .ok_or_else(|| format!("missing `{field}` in cert_effect read-out"))?;
    match &node.value {
        Json::Num(_) => {
            let n = node
                .as_int53()
                .map_err(|e| format!("invalid integer `{field}`: {e}"))?;
            u64::try_from(n).map_err(|_| format!("`{field}` must be non-negative"))
        }
        Json::Str(_) => node
            .as_u64str()
            .map_err(|e| format!("invalid u64 string `{field}`: {e}")),
        _ => Err(format!("`{field}` must be a non-negative integer")),
    }
}

/// Format the certificate decision-effect report line for `OD-08` (`N00-10` step 6).
#[must_use]
pub fn format_cert_effect_line(effects: Option<&CertEffectTotals>) -> String {
    match effects {
        None => NO_CERT_LEDGER_LINE.to_owned(),
        Some(t) => {
            if t.changed == 0 {
                format!(
                    "consumed {}, changed {}, unchanged {}, unknown {} ({})",
                    t.consumed, t.changed, t.unchanged, t.unknown, ZERO_CHANGED_DEFAULT_TEXT
                )
            } else {
                format!(
                    "consumed {}, changed {}, unchanged {}, unknown {} (unknown excluded from changed)",
                    t.consumed, t.changed, t.unchanged, t.unknown
                )
            }
        }
    }
}

/// Render the milestone exit-review report listing state counts, every open disagreement
/// with its settling tests, and the certificate decision-effect line under `OD-08`.
#[must_use]
pub fn render_exit_review_report(
    register: &HypothesisRegister,
    cert_effects: Option<&CertEffectTotals>,
) -> String {
    let counts = register.counts_by_state();
    let open = register.open_disagreements();
    let mut out = String::new();

    let _ = writeln!(out, "# Hypothesis Verdict Register — Exit Review Report");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Rows: {} total (REFUTED: {}, DEFERRED: {}, ACTIVE: {}, NARROWED: {})",
        counts.total, counts.refuted, counts.deferred, counts.active, counts.narrowed
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "## Open Disagreements ({} open)", open.len());
    for d in open {
        let _ = writeln!(
            out,
            "- {}: {} | settled_by: {}",
            d.id,
            d.question,
            d.settled_by.join(", ")
        );
        if d.id == CERT_DISAGREEMENT_ID {
            let _ = writeln!(
                out,
                "  certificate_effects: {}",
                format_cert_effect_line(cert_effects)
            );
        }
    }
    out
}

/// Known-answer paired-control variance gain `1 / (1 - rho^2)` for row `H-22b` (`N00-10`).
#[must_use]
pub fn h22b_paired_gain(rho: f64) -> f64 {
    1.0 / (1.0 - rho * rho)
}
