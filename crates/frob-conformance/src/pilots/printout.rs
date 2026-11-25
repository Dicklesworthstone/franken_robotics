#![deny(unsafe_code)]

//! Deterministic Markdown printout renderer for `docs/delivery/pilots.md` (`N38a-09`).
//!
//! One row per pilot: identifier, mechanism, tier, falsifier, ceiling, fallback,
//! effective status, and last verdict. Determinism class D0; prints nothing (rule L-7);
//! zero `unsafe`.

use std::fs;
use std::path::Path;

use crate::pilots::funding::StatusCounts;
use crate::pilots::schema::{
    CONVENTION_15_RULE, PILOTS_PATH, PILOTS_PRINTOUT_PATH, PILOTS_SCHEMA, PilotRegister, PilotSite,
    PilotTier,
};
use crate::pilots::validate::{effective_status, load_register};

/// Render `docs/delivery/pilots.md` deterministically (D0) from `reg`.
#[must_use]
pub fn render_pilots_markdown(
    reg: &PilotRegister,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
) -> String {
    let counts = StatusCounts::from_register(reg, root, previous);
    let tier_s = reg.pilots.iter().filter(|p| p.tier == PilotTier::S).count();
    let tier_f = reg.pilots.iter().filter(|p| p.tier == PilotTier::F).count();
    let tier_m = reg.pilots.iter().filter(|p| p.tier == PilotTier::M).count();

    let mut out = String::new();
    out.push_str("# FrankenRobotics Pilot Register\n\n");
    out.push_str(&format!(
        "Generated from `{PILOTS_PATH}` (`{PILOTS_SCHEMA}`); do not edit by hand.\n\n"
    ));
    out.push_str(&format!("**Convention 15:** {CONVENTION_15_RULE}\n\n"));
    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "- Pilots: {} (Solid-required [S]: {tier_s}, Frontier [F]: {tier_f}, Moonshot [M]: {tier_m})\n",
        reg.pilots.len()
    ));
    out.push_str(&format!(
        "- Statuses: registered={}, running={}, reproduced={}, differs={}, falsifier-holds={}, falsifier-rejected={}, inconclusive={}, withdrawn={}, void={}\n\n",
        counts.registered,
        counts.running,
        counts.reproduced,
        counts.differs,
        counts.falsifier_holds,
        counts.falsifier_rejected,
        counts.inconclusive,
        counts.withdrawn,
        counts.void
    ));
    out.push_str("## Pilots\n\n");
    out.push_str(
        "| ID | Mechanism | Tier | Falsifier | Ceiling | Fallback | Status | Last Verdict |\n",
    );
    out.push_str("|---|---|---|---|---|---|---|---|\n");

    if reg.pilots.is_empty() {
        out.push_str("| *(none registered yet)* | — | — | — | — | — | — | — |\n");
        return out;
    }

    for rec in &reg.pilots {
        let eff = effective_status(rec, root, previous);
        let ceiling_cell = format!("{} ({})", rec.ceiling.value, rec.ceiling.statement);
        let fallback_cell = format!("{} [{}]", rec.fallback.ships, rec.fallback.kill_gate);
        let last_verdict = match rec.last_result() {
            None => "none".to_owned(),
            Some(r) => {
                if r.site == PilotSite::DevelopmentOracle {
                    format!("{} (provisional: development-oracle)", r.verdict.as_str())
                } else {
                    format!("{} ({})", r.verdict.as_str(), r.site.as_str())
                }
            }
        };

        out.push_str("| ");
        out.push_str(&escape_md_cell(&rec.id));
        out.push_str(" | ");
        out.push_str(&escape_md_cell(&rec.mechanism));
        out.push_str(" | ");
        out.push_str(rec.tier.as_str());
        out.push_str(" | ");
        out.push_str(&escape_md_cell(&rec.falsifier));
        out.push_str(" | ");
        out.push_str(&escape_md_cell(&ceiling_cell));
        out.push_str(" | ");
        out.push_str(&escape_md_cell(&fallback_cell));
        out.push_str(" | ");
        out.push_str(eff.as_str());
        out.push_str(" | ");
        out.push_str(&escape_md_cell(&last_verdict));
        out.push_str(" |\n");
    }

    out
}

/// Compare the committed `docs/delivery/pilots.md` under `root` byte-for-byte
/// against a fresh regeneration from `pilots.toml`.
pub fn check_printout_matches(root: &Path) -> Result<(), String> {
    let reg = load_register(root).map_err(|e| e.to_string())?;
    let expected = render_pilots_markdown(&reg, Some(root), None);
    let path = root.join(PILOTS_PRINTOUT_PATH);
    let actual = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read `{PILOTS_PRINTOUT_PATH}`: {e}"))?;
    if actual != expected {
        return Err(format!(
            "`{PILOTS_PRINTOUT_PATH}` differs from fresh generation from `{PILOTS_PATH}` (hand-edited or stale printout)"
        ));
    }
    Ok(())
}

fn escape_md_cell(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '|' => out.push_str("\\|"),
            '\n' | '\r' => out.push(' '),
            c => out.push(c),
        }
    }
    out
}
