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
use frob_cert::kinds::registry::Class;
use frob_cert::kinds::rules::RuleId;
use frob_cert::limits::{COMPILED_LIMITS, RULE_INPUT_UNREADABLE, RULE_LIMITS_EXCEEDED};
use frob_cert::schema::{
    BUNDLE_SCHEMA_VERSION, bundle_bytes_leaf_digest, compute_merkle_root, set_weakest_class,
};
use frob_cert::{AuditReport, TRUSTED_BASE_LINE, Verdict, audit_bytes_with_query};

struct CliArgs {
    help: bool,
    json: bool,
    canonical_only: bool,
    reserialize: Option<PathBuf>,
    query: Option<PathBuf>,
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
        query: None,
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
        if raw == "--query" {
            let Some(query_path) = args.next() else {
                return Err("--query requires a query file path".to_string());
            };
            parsed.query = Some(PathBuf::from(query_path));
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
         Offline std-only certificate auditor for FrankenRobotics (BD-026 / N12-03).\n\n\
         USAGE:\n  \
           frob-verify [--json] [--canonical-only] [--query <file>] [--reserialize <out>] <bundle>...\n  \
           frob-verify --help\n\n\
         OPTIONS:\n  \
           --json                Emit structured canonical JSON report on stdout\n  \
           --canonical-only      Check canonical JSON and BLAKE3 without kind dispatch\n  \
           --query <file>        Compare every bundle against the offered query file\n  \
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
        claim_class: None,
        equations_reviewed: None,
        declared_merkle_root: None,
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
    // Keys must stay in strictly ascending UTF-8 byte order for canonical serialization.
    let mut entries: Vec<(String, JsonValue)> = Vec::new();
    entries.push(("blake3".to_string(), JsonValue::Str(report.blake3_hex.clone())));
    if let Some(cls) = report.claim_class {
        entries.push(("class".to_string(), JsonValue::Str(cls.as_str().to_string())));
    }
    if let Some(domain) = &report.domain_blake3_hex {
        entries.push(("domain_blake3".to_string(), JsonValue::Str(domain.clone())));
    }
    if let Some(rev) = report.equations_reviewed {
        entries.push(("equations_reviewed".to_string(), JsonValue::Bool(rev)));
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
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    JsonValue::Obj(entries)
}

struct CliSetSummary {
    merkle_root: Option<String>,
    recomputed: bool,
    level_widths: Vec<usize>,
    class: Option<Class>,
    weakest_entry: Option<String>,
    rule: Option<&'static str>,
    path: Option<String>,
    message: Option<String>,
}

fn set_to_json_value(set: &CliSetSummary) -> JsonValue {
    let mut entries: Vec<(String, JsonValue)> = Vec::new();
    if let Some(cls) = set.class {
        entries.push(("class".to_string(), JsonValue::Str(cls.as_str().to_string())));
    }
    entries.push((
        "level_widths".to_string(),
        JsonValue::Arr(set.level_widths.iter().map(|&w| JsonValue::Int(w as i64)).collect()),
    ));
    entries.push((
        "merkle_root".to_string(),
        match &set.merkle_root {
            Some(r) => JsonValue::Str(r.clone()),
            None => JsonValue::Null,
        },
    ));
    if let Some(msg) = &set.message {
        entries.push(("message".to_string(), JsonValue::Str(msg.clone())));
    }
    if let Some(p) = &set.path {
        entries.push(("path".to_string(), JsonValue::Str(p.clone())));
    }
    entries.push(("recomputed".to_string(), JsonValue::Bool(set.recomputed)));
    if let Some(r) = set.rule {
        entries.push(("rule".to_string(), JsonValue::Str(r.to_string())));
    }
    if let Some(w) = &set.weakest_entry {
        entries.push(("weakest_entry".to_string(), JsonValue::Str(w.clone())));
    }
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
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

    let offered_query: Option<Vec<u8>> = match &args.query {
        None => None,
        Some(qpath) => match read_bounded_file(qpath) {
            Ok(bytes) => Some(bytes),
            Err((rule, msg)) => {
                let mut err = io::stderr().lock();
                let _ = writeln!(err, "frob-verify: --query {}: {rule}: {msg}", qpath.display());
                return ExitCode::from(3);
            }
        },
    };

    let mut results: Vec<(String, AuditReport)> = Vec::with_capacity(args.files.len());
    let mut any_reject = false;
    let mut any_unsupported = false;

    for path in &args.files {
        let label = path_label(path);
        let mut report = match read_bounded_file(path) {
            Ok(bytes) => audit_bytes_with_query(
                &bytes,
                offered_query.as_deref(),
                COMPILED_LIMITS,
                args.canonical_only,
            ),
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

    // Set-level Merkle root and weakest-class summary when auditing frob.cert/1 bundles.
    let all_frobcert = !args.canonical_only
        && !results.is_empty()
        && results
            .iter()
            .all(|(_, r)| r.schema.as_deref() == Some(BUNDLE_SCHEMA_VERSION));

    let set_summary: Option<CliSetSummary> = if all_frobcert {
        let classes: Vec<Class> = results.iter().filter_map(|(_, r)| r.claim_class).collect();
        let (weak_class, weak_label) = if classes.len() == results.len() {
            match set_weakest_class(&classes) {
                Some((c, idx)) => (Some(c), Some(results[idx].0.clone())),
                None => (None, None),
            }
        } else {
            (None, None)
        };

        if results.len() == 1 {
            Some(CliSetSummary {
                merkle_root: results[0].1.declared_merkle_root.clone(),
                recomputed: false,
                level_widths: vec![1],
                class: weak_class,
                weakest_entry: weak_label,
                rule: None,
                path: None,
                message: None,
            })
        } else {
            let mut leaves = Vec::with_capacity(results.len());
            let mut leaf_err = None;
            for (idx, (_, rep)) in results.iter().enumerate() {
                if let Some(canon) = &rep.canonical_bytes {
                    match bundle_bytes_leaf_digest(canon) {
                        Ok(d) => leaves.push(d),
                        Err(e) => {
                            leaf_err = Some((e.rule, format!("bundles[{idx}].{}", e.path), e.message));
                            break;
                        }
                    }
                } else {
                    leaf_err = Some((
                        RuleId::SchemaFieldType,
                        format!("bundles[{idx}]"),
                        "bundle lacks canonical bytes for Merkle leaf".to_string(),
                    ));
                    break;
                }
            }

            if let Some((rule, path, msg)) = leaf_err {
                any_reject = true;
                Some(CliSetSummary {
                    merkle_root: None,
                    recomputed: false,
                    level_widths: Vec::new(),
                    class: weak_class,
                    weakest_entry: weak_label,
                    rule: Some(rule.as_str()),
                    path: Some(path),
                    message: Some(msg),
                })
            } else {
                match compute_merkle_root(&leaves) {
                    Ok(tree) => {
                        let mut mismatch = None;
                        for (idx, (label, rep)) in results.iter().enumerate() {
                            if let Some(decl) = &rep.declared_merkle_root {
                                if decl != &tree.root_hex {
                                    mismatch = Some((
                                        RuleId::HashMerkleRoot.as_str(),
                                        format!("bundles[{idx}].lineage.merkle_root"),
                                        format!(
                                            "{label}: declared merkle_root {decl:?} != recomputed {:?}",
                                            tree.root_hex
                                        ),
                                    ));
                                    break;
                                }
                            }
                        }
                        if mismatch.is_some() {
                            any_reject = true;
                        }
                        Some(CliSetSummary {
                            merkle_root: Some(tree.root_hex),
                            recomputed: true,
                            level_widths: tree.level_widths,
                            class: weak_class,
                            weakest_entry: weak_label,
                            rule: mismatch.as_ref().map(|m| m.0),
                            path: mismatch.as_ref().map(|m| m.1.clone()),
                            message: mismatch.as_ref().map(|m| m.2.clone()),
                        })
                    }
                    Err(rej) => {
                        any_reject = true;
                        Some(CliSetSummary {
                            merkle_root: None,
                            recomputed: true,
                            level_widths: Vec::new(),
                            class: weak_class,
                            weakest_entry: weak_label,
                            rule: Some(rej.rule.as_str()),
                            path: Some(rej.path),
                            message: Some(rej.message),
                        })
                    }
                }
            }
        }
    } else {
        None
    };

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
        let mut top = vec![
            ("limits".to_string(), limits_to_json_value()),
            ("overall".to_string(), JsonValue::Str(overall.to_string())),
            ("reports".to_string(), JsonValue::Arr(report_arr)),
        ];
        if let Some(set) = &set_summary {
            top.push(("set".to_string(), set_to_json_value(set)));
        }
        top.push((
            "trusted_base".to_string(),
            JsonValue::Str(TRUSTED_BASE_LINE.to_string()),
        ));
        let doc = JsonValue::Obj(top);
        let _ = writeln!(out, "{}", serialize_canonical(&doc));
    } else {
        for (label, rep) in &results {
            let rev_note = match rep.equations_reviewed {
                Some(true) => " equations_reviewed=yes",
                Some(false) => " equations_reviewed=no",
                None => "",
            };
            match &rep.verdict {
                Verdict::Accept => {
                    let _ = writeln!(out, "accept   {label} blake3={}{rev_note}", rep.blake3_hex);
                }
                Verdict::Unsupported { kind, message } => {
                    let _ = writeln!(
                        out,
                        "unsupported  {label} kind={kind} blake3={}{rev_note} ({message})",
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
        if let Some(set) = &set_summary {
            let cls_str = set.class.map_or("unknown", Class::as_str);
            let mr_str = if set.recomputed {
                set.merkle_root.as_deref().unwrap_or("none")
            } else {
                "not-recomputed"
            };
            if let Some(rule) = set.rule {
                let _ = writeln!(
                    out,
                    "set: reject rule={rule} path={} merkle_root={mr_str} class={cls_str} ({})",
                    set.path.as_deref().unwrap_or("lineage.merkle_root"),
                    set.message.as_deref().unwrap_or("")
                );
            } else {
                let _ = writeln!(
                    out,
                    "set: {overall} entries={} class={cls_str} merkle_root={mr_str} recomputed={}",
                    results.len(),
                    set.recomputed
                );
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
