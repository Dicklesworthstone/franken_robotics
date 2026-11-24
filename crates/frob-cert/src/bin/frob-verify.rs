#![forbid(unsafe_code)]
//! Standalone offline certificate auditor binary `frob-verify` (BD-026).
//!
//! Reads only files explicitly named on the command line, reads no environment
//! variables, opens no sockets, spawns no child processes, and writes only to
//! standard output / standard error and an explicit `--reserialize <out>` path.

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use frob_cert::json::{JsonValue, serialize_canonical};
use frob_cert::limits::{COMPILED_LIMITS, RULE_INPUT_UNREADABLE, RULE_LIMITS_EXCEEDED};
use frob_cert::{AuditReport, TRUSTED_BASE_LINE, Verdict, audit_bytes_with_options};

struct CliArgs {
    help: bool,
    json: bool,
    canonical_only: bool,
    reserialize: Option<PathBuf>,
    files: Vec<PathBuf>,
}

fn parse_args() -> Result<CliArgs, String> {
    let mut args = std::env::args_os();
    let _program = args.next();

    let mut parsed = CliArgs {
        help: false,
        json: false,
        canonical_only: false,
        reserialize: None,
        files: Vec::new(),
    };
    let mut positional_only = false;

    while let Some(raw) = args.next() {
        if positional_only {
            parsed.files.push(PathBuf::from(raw));
            continue;
        }
        if raw == "--" {
            positional_only = true;
            continue;
        }
        if raw == "-h" || raw == "--help" {
            parsed.help = true;
            continue;
        }
        if raw == "--json" {
            parsed.json = true;
            continue;
        }
        if raw == "--canonical-only" {
            parsed.canonical_only = true;
            continue;
        }
        if raw == "--reserialize" {
            let Some(out_path) = args.next() else {
                return Err("--reserialize requires an output file path".to_string());
            };
            parsed.reserialize = Some(PathBuf::from(out_path));
            continue;
        }
        if let Some(s) = raw.to_str() {
            if s.starts_with('-') && s.len() > 1 {
                return Err(format!("unknown option {s:?} (try --help)"));
            }
        }
        parsed.files.push(PathBuf::from(raw));
    }

    if !parsed.help && parsed.files.is_empty() {
        return Err("expected at least one input bundle path (try --help)".to_string());
    }
    if parsed.reserialize.is_some() && parsed.files.len() > 1 {
        return Err("--reserialize accepts a single input file".to_string());
    }

    Ok(parsed)
}

fn print_help() {
    let mut out = io::stdout().lock();
    let _ = writeln!(
        out,
        "frob-verify {}\n\
         Offline std-only certificate auditor for FrankenRobotics (BD-026).\n\n\
         USAGE:\n  \
           frob-verify [--json] [--canonical-only] [--reserialize <out>] <bundle>...\n  \
           frob-verify --help\n\n\
         OPTIONS:\n  \
           --json                Emit structured canonical JSON report on stdout\n  \
           --canonical-only      Verify canonical JSON and BLAKE3 without kind dispatch\n  \
           --reserialize <out>   Write canonical re-serialized bytes to <out>\n  \
           -h, --help            Print this help message and exit 0\n\n\
         EXIT CODES:\n  \
           0  Every bundle accepted\n  \
           2  None rejected and at least one unsupported\n  \
           3  At least one bundle rejected, unreadable, or invalid usage\n\n\
         {}\n\
         {}",
        frob_cert::AUDITOR_VERSION,
        COMPILED_LIMITS.summary_line(),
        TRUSTED_BASE_LINE,
    );
}

fn read_bounded_file(path: &PathBuf) -> Result<Vec<u8>, (&'static str, String)> {
    let file = File::open(path).map_err(|e| (RULE_INPUT_UNREADABLE, format!("cannot open: {e}")))?;
    let meta = file
        .metadata()
        .map_err(|e| (RULE_INPUT_UNREADABLE, format!("cannot stat: {e}")))?;
    if !meta.is_file() {
        return Err((RULE_INPUT_UNREADABLE, "input path is not a regular file".to_string()));
    }
    let max_bytes = COMPILED_LIMITS.max_file_bytes as u64;
    if meta.len() > max_bytes {
        return Err((
            RULE_LIMITS_EXCEEDED,
            format!("file size {} bytes exceeds limit {} bytes", meta.len(), max_bytes),
        ));
    }

    // Also guard against growing files while reading via `take(max_bytes + 1)`.
    let mut buf = Vec::with_capacity(meta.len().min(max_bytes) as usize);
    let mut limited = file.take(max_bytes + 1);
    limited
        .read_to_end(&mut buf)
        .map_err(|e| (RULE_INPUT_UNREADABLE, format!("cannot read: {e}")))?;
    if buf.len() as u64 > max_bytes {
        return Err((
            RULE_LIMITS_EXCEEDED,
            format!("file size exceeds limit {max_bytes} bytes"),
        ));
    }
    Ok(buf)
}

fn unreadable_report(rule: &'static str, message: String) -> AuditReport {
    AuditReport {
        blake3_hex: String::new(),
        domain_blake3_hex: None,
        schema: None,
        kind: None,
        verdict: Verdict::Reject {
            rule,
            refusal: None,
            path: "$".to_string(),
            message,
        },
        limits: COMPILED_LIMITS,
        interval_impl: frob_cert::interval::INTERVAL_IMPL_ID,
        trusted_base: TRUSTED_BASE_LINE,
        canonical_bytes: None,
    }
}

fn path_label(path: &PathBuf) -> String {
    let os: OsString = path.as_os_str().to_os_string();
    os.to_string_lossy().into_owned()
}

fn report_to_json_value(file_label: &str, report: &AuditReport) -> JsonValue {
    let mut entries: Vec<(String, JsonValue)> = Vec::new();
    entries.push(("blake3".to_string(), JsonValue::Str(report.blake3_hex.clone())));
    if let Some(domain) = &report.domain_blake3_hex {
        entries.push(("domain_blake3".to_string(), JsonValue::Str(domain.clone())));
    }
    entries.push(("file".to_string(), JsonValue::Str(file_label.to_string())));
    entries.push((
        "interval_impl".to_string(),
        JsonValue::Str(report.interval_impl.to_string()),
    ));
    if let Some(kind) = &report.kind {
        entries.push(("kind".to_string(), JsonValue::Str(kind.clone())));
    }
    if let Some(schema) = &report.schema {
        entries.push(("schema".to_string(), JsonValue::Str(schema.clone())));
    }
    entries.push((
        "verdict".to_string(),
        JsonValue::Str(report.verdict.label().to_string()),
    ));
    match &report.verdict {
        Verdict::Accept => {}
        Verdict::Reject {
            rule,
            refusal,
            path,
            message,
        } => {
            entries.push(("message".to_string(), JsonValue::Str(message.clone())));
            entries.push(("path".to_string(), JsonValue::Str(path.clone())));
            if let Some(r) = refusal {
                entries.push(("refusal".to_string(), JsonValue::Str((*r).to_string())));
            }
            entries.push(("rule".to_string(), JsonValue::Str((*rule).to_string())));
        }
        Verdict::Unsupported { kind, message } => {
            entries.push(("message".to_string(), JsonValue::Str(message.clone())));
            entries.push(("unsupported_kind".to_string(), JsonValue::Str(kind.clone())));
        }
    }
    JsonValue::Obj(entries)
}

fn limits_to_json_value() -> JsonValue {
    JsonValue::Obj(vec![
        (
            "max_bigint_bits".to_string(),
            JsonValue::Int(COMPILED_LIMITS.max_bigint_bits as i64),
        ),
        (
            "max_file_bytes".to_string(),
            JsonValue::Int(COMPILED_LIMITS.max_file_bytes as i64),
        ),
        (
            "max_json_depth".to_string(),
            JsonValue::Int(COMPILED_LIMITS.max_json_depth as i64),
        ),
        (
            "max_numeric_entries".to_string(),
            JsonValue::Int(COMPILED_LIMITS.max_numeric_entries as i64),
        ),
    ])
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(msg) => {
            let mut err = io::stderr().lock();
            let _ = writeln!(err, "frob-verify: {msg}");
            return ExitCode::from(3);
        }
    };

    if args.help {
        print_help();
        return ExitCode::SUCCESS;
    }

    let mut results: Vec<(String, AuditReport)> = Vec::with_capacity(args.files.len());
    let mut any_reject = false;
    let mut any_unsupported = false;

    for path in &args.files {
        let label = path_label(path);
        let mut report = match read_bounded_file(path) {
            Ok(bytes) => audit_bytes_with_options(&bytes, COMPILED_LIMITS, args.canonical_only),
            Err((rule, msg)) => unreadable_report(rule, msg),
        };

        if let Some(out_path) = &args.reserialize {
            if let Some(canonical) = &report.canonical_bytes {
                if let Err(e) = fs::write(out_path, canonical) {
                    report.verdict = Verdict::Reject {
                        rule: RULE_INPUT_UNREADABLE,
                        refusal: None,
                        path: "$".to_string(),
                        message: format!("cannot write --reserialize output: {e}"),
                    };
                }
            }
        }

        if report.verdict.is_reject() {
            any_reject = true;
        } else if report.verdict.is_unsupported() {
            any_unsupported = true;
        }
        results.push((label, report));
    }

    let overall = if any_reject {
        "reject"
    } else if any_unsupported {
        "unsupported"
    } else {
        "accept"
    };

    let mut out = io::stdout().lock();
    if args.json {
        let report_arr: Vec<JsonValue> = results
            .iter()
            .map(|(label, rep)| report_to_json_value(label, rep))
            .collect();
        let doc = JsonValue::Obj(vec![
            ("limits".to_string(), limits_to_json_value()),
            ("overall".to_string(), JsonValue::Str(overall.to_string())),
            ("reports".to_string(), JsonValue::Arr(report_arr)),
            (
                "trusted_base".to_string(),
                JsonValue::Str(TRUSTED_BASE_LINE.to_string()),
            ),
        ]);
        let _ = writeln!(out, "{}", serialize_canonical(&doc));
    } else {
        for (label, rep) in &results {
            match &rep.verdict {
                Verdict::Accept => {
                    let _ = writeln!(out, "accept   {label} blake3={}", rep.blake3_hex);
                }
                Verdict::Unsupported { kind, message } => {
                    let _ = writeln!(
                        out,
                        "unsupported  {label} kind={kind} blake3={} ({message})",
                        rep.blake3_hex
                    );
                }
                Verdict::Reject {
                    rule,
                    refusal,
                    path,
                    message,
                } => {
                    let refusal_part = refusal
                        .map(|r| format!(" refusal={r}"))
                        .unwrap_or_default();
                    let _ = writeln!(
                        out,
                        "reject   {label} rule={rule}{refusal_part} path={path} blake3={} ({message})",
                        rep.blake3_hex
                    );
                }
            }
        }
        let _ = writeln!(out, "{}", COMPILED_LIMITS.summary_line());
        let _ = writeln!(out, "{TRUSTED_BASE_LINE}");
    }

    if any_reject {
        ExitCode::from(3)
    } else if any_unsupported {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}
