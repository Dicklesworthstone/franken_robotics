#![forbid(unsafe_code)]
//! Standard-library-only registry inspection and probe example for `frob-cert` (N12-02).
//!
//! Usage:
//!   kind_registry --json
//!   kind_registry --doc
//!   kind_registry --probe <kind> <version> <claim-kind> <class> [--json]

use std::io::{self, Write};
use std::process::ExitCode;

use frob_cert::json::{JsonValue, serialize_canonical};
use frob_cert::kinds::registry::{
    CLAIM_KIND_SCOPES, CheckVerdict, Class, REGISTRY, REGISTRY_VERSION, enabled_kinds, probe,
    render_registry_markdown,
};
use frob_cert::kinds::rules::RULE_CATALOGUE;

enum Mode {
    Help,
    JsonTable,
    MarkdownDoc,
    Probe {
        kind: String,
        version: u32,
        claim_kind: String,
        class: Class,
        json: bool,
    },
}

fn parse_args() -> Result<Mode, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        return Ok(Mode::Help);
    }
    if args.len() == 1 && args[0] == "--json" {
        return Ok(Mode::JsonTable);
    }
    if args.len() == 1 && args[0] == "--doc" {
        return Ok(Mode::MarkdownDoc);
    }
    if args[0] == "--probe" {
        if args.len() != 5 && args.len() != 6 {
            return Err(
                "usage: kind_registry --probe <kind> <version> <claim-kind> <class> [--json]"
                    .to_string(),
            );
        }
        let kind = args[1].clone();
        let version: u32 = args[2]
            .parse()
            .map_err(|_| format!("invalid version {:?} (expected unsigned 32-bit int)", args[2]))?;
        let claim_kind = args[3].clone();
        let class = Class::parse(&args[4]).ok_or_else(|| {
            format!(
                "invalid class {:?} (expected Estimated, Tested, Certified, or Proved)",
                args[4]
            )
        })?;
        let json = if args.len() == 6 {
            if args[5] != "--json" {
                return Err(format!("unexpected trailing argument {:?}", args[5]));
            }
            true
        } else {
            false
        };
        return Ok(Mode::Probe {
            kind,
            version,
            claim_kind,
            class,
            json,
        });
    }
    Err(format!("unknown option {:?} (try --help)", args[0]))
}

fn table_to_json() -> JsonValue {
    let kinds_arr: Vec<JsonValue> = REGISTRY
        .iter()
        .map(|row| {
            let scopes: Vec<JsonValue> = row
                .scopes()
                .iter()
                .map(|s| JsonValue::Str(s.as_str().to_string()))
                .collect();
            let side: Vec<JsonValue> = row
                .requires_side_conditions()
                .iter()
                .map(|s| JsonValue::Str((*s).to_string()))
                .collect();
            JsonValue::Obj(vec![
                ("cap".to_string(), JsonValue::Str(row.cap().as_str().to_string())),
                ("capped_by_inputs".to_string(), JsonValue::Bool(row.capped_by_inputs())),
                (
                    "checker_bead".to_string(),
                    JsonValue::Str(row.checker_bead().to_string()),
                ),
                ("name".to_string(), JsonValue::Str(row.name().to_string())),
                (
                    "payload_schema".to_string(),
                    JsonValue::Str(row.payload_schema().to_string()),
                ),
                (
                    "recomputes".to_string(),
                    JsonValue::Str(row.recomputes().to_string()),
                ),
                ("requires_side_conditions".to_string(), JsonValue::Arr(side)),
                ("scopes".to_string(), JsonValue::Arr(scopes)),
                (
                    "status".to_string(),
                    JsonValue::Str(row.status().as_str().to_string()),
                ),
                (
                    "tier".to_string(),
                    JsonValue::Str(row.tier().as_str().to_string()),
                ),
                ("version".to_string(), JsonValue::Int(i64::from(row.version()))),
            ])
        })
        .collect();

    let enabled_arr: Vec<JsonValue> = enabled_kinds()
        .into_iter()
        .map(|(k, v)| {
            JsonValue::Obj(vec![
                ("kind".to_string(), JsonValue::Str(k.to_string())),
                ("version".to_string(), JsonValue::Int(i64::from(v))),
            ])
        })
        .collect();

    let claim_scopes: Vec<JsonValue> = CLAIM_KIND_SCOPES
        .iter()
        .map(|c| {
            JsonValue::Obj(vec![
                (
                    "claim_kind".to_string(),
                    JsonValue::Str(c.claim_kind().to_string()),
                ),
                (
                    "scope".to_string(),
                    JsonValue::Str(c.scope().as_str().to_string()),
                ),
            ])
        })
        .collect();

    let rules_arr: Vec<JsonValue> = RULE_CATALOGUE
        .iter()
        .map(|r| JsonValue::Str(r.as_str().to_string()))
        .collect();

    JsonValue::Obj(vec![
        ("claim_kind_scopes".to_string(), JsonValue::Arr(claim_scopes)),
        ("enabled_kinds".to_string(), JsonValue::Arr(enabled_arr)),
        (
            "kind_count".to_string(),
            JsonValue::Int(REGISTRY.len() as i64),
        ),
        ("kinds".to_string(), JsonValue::Arr(kinds_arr)),
        (
            "registry_version".to_string(),
            JsonValue::Str(REGISTRY_VERSION.to_string()),
        ),
        ("rule_ids".to_string(), JsonValue::Arr(rules_arr)),
    ])
}

fn probe_to_json(
    kind: &str,
    version: u32,
    claim_kind: &str,
    class: Class,
) -> JsonValue {
    let outcome = probe(kind, version, claim_kind, class);
    let (rule_val, refusal_val, repairs_val, msg) = match &outcome.verdict {
        CheckVerdict::Accept => (
            JsonValue::Null,
            JsonValue::Null,
            JsonValue::Arr(Vec::new()),
            "accepted".to_string(),
        ),
        CheckVerdict::Reject {
            rule,
            refusal,
            repairs,
            message,
        } => (
            JsonValue::Str(rule.as_str().to_string()),
            match refusal {
                Some(r) => JsonValue::Str((*r).to_string()),
                None => JsonValue::Null,
            },
            JsonValue::Arr(
                repairs
                    .iter()
                    .map(|s| JsonValue::Str((*s).to_string()))
                    .collect(),
            ),
            message.clone(),
        ),
        CheckVerdict::Unsupported { message, .. } => (
            JsonValue::Null,
            JsonValue::Null,
            JsonValue::Arr(Vec::new()),
            message.clone(),
        ),
    };
    let scope_val = match outcome.scope {
        Some(s) => JsonValue::Str(s.as_str().to_string()),
        None => JsonValue::Null,
    };
    JsonValue::Obj(vec![
        ("cap_check".to_string(), JsonValue::Str(outcome.cap_check.to_string())),
        ("claim_kind".to_string(), JsonValue::Str(claim_kind.to_string())),
        ("class".to_string(), JsonValue::Str(class.as_str().to_string())),
        ("kind".to_string(), JsonValue::Str(kind.to_string())),
        ("message".to_string(), JsonValue::Str(msg)),
        ("refusal".to_string(), refusal_val),
        (
            "registry_version".to_string(),
            JsonValue::Str(REGISTRY_VERSION.to_string()),
        ),
        ("repairs".to_string(), repairs_val),
        ("rule".to_string(), rule_val),
        ("scope".to_string(), scope_val),
        ("scope_check".to_string(), JsonValue::Str(outcome.scope_check.to_string())),
        (
            "verdict".to_string(),
            JsonValue::Str(outcome.verdict.label().to_string()),
        ),
        ("version".to_string(), JsonValue::Int(i64::from(version))),
    ])
}

fn main() -> ExitCode {
    let mode = match parse_args() {
        Ok(m) => m,
        Err(msg) => {
            let mut err = io::stderr().lock();
            let _ = writeln!(err, "kind_registry: {msg}");
            return ExitCode::from(3);
        }
    };
    let mut out = io::stdout().lock();
    match mode {
        Mode::Help => {
            let _ = writeln!(
                out,
                "kind_registry ({REGISTRY_VERSION})\n\
                 USAGE:\n  \
                   kind_registry --json\n  \
                   kind_registry --doc\n  \
                   kind_registry --probe <kind> <version> <claim-kind> <class> [--json]"
            );
        }
        Mode::JsonTable => {
            let _ = writeln!(out, "{}", serialize_canonical(&table_to_json()));
        }
        Mode::MarkdownDoc => {
            let _ = out.write_all(render_registry_markdown().as_bytes());
        }
        Mode::Probe {
            kind,
            version,
            claim_kind,
            class,
            json,
        } => {
            if json {
                let doc = probe_to_json(&kind, version, &claim_kind, class);
                let _ = writeln!(out, "{}", serialize_canonical(&doc));
            } else {
                let outcome = probe(&kind, version, &claim_kind, class);
                let _ = writeln!(
                    out,
                    "registry={} kind={} version={} claim_kind={} class={} scope_check={} cap_check={} verdict={}",
                    REGISTRY_VERSION,
                    kind,
                    version,
                    claim_kind,
                    class.as_str(),
                    outcome.scope_check,
                    outcome.cap_check,
                    outcome.verdict.label()
                );
            }
        }
    }
    ExitCode::SUCCESS
}
