#![deny(unsafe_code)]

//! Integration and unit tests for `cargo xtask check-generated` (`N34p4-03`,
//! rule L-8, GATE-WS-01 plant 5, INV-SPARK-08).
//!
//! Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};

use frob_core::json::{Limits, admit};

use xtask::check_generated::{self, OWNED_RULES, run_check, run_command};
use xtask::cli::{self, CheckFn, EXIT_FINDINGS, EXIT_OK, EXIT_USAGE};
use xtask::generate::{
    BUILTIN_GENERATORS, ExpectedMarker, GeneratedFile, GeneratorError, GeneratorMode,
    GeneratorOutput, GeneratorRow, OutputKind, RULE_DIFFERS, RULE_DIGEST, RULE_MARKER_MISSING,
    RULE_MISSING, RULE_ORPHAN, RULE_STATUS_INVALID,
};
use xtask::report::validate_report_json;
use xtask::stack::lock;

const TEST_COMMANDS: &[(&str, CheckFn)] = &[("check-generated", run_command)];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn fresh_dir(tag: &str) -> PathBuf {
    static CTR: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "frob_xtask_check_gen_{tag}_{}_{}",
        std::process::id(),
        CTR.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).expect("mkdir check_generated fixture");
    dir
}

fn copy_file(src: &Path, dst: &Path) {
    if let Some(p) = dst.parent() {
        fs::create_dir_all(p).expect("mkdir parent");
    }
    fs::copy(src, dst).expect("copy fixture file");
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    if !src.is_dir() {
        return;
    }
    fs::create_dir_all(dst).expect("mkdir dst");
    let mut entries: Vec<PathBuf> = fs::read_dir(src)
        .expect("read_dir")
        .flatten()
        .map(|e| e.path())
        .collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().expect("file_name");
        let target = dst.join(name);
        if p.is_dir() {
            copy_dir_recursive(&p, &target);
        } else {
            fs::copy(&p, &target).expect("copy file");
        }
    }
}

/// Build an isolated scratch copy of all files read/written by the 6 generator rows.
fn build_scratch_repo(tag: &str) -> PathBuf {
    let real = repo_root();
    let dst = fresh_dir(tag);
    for rel in [
        "README.md",
        "DEPENDENCY_ADMISSION.md",
        "crates/frob-core/src/refusal/code_gen.rs",
        "crates/frob-core/src/qty_gen.rs",
        "crates/xtask/src/stack/lock.rs",
    ] {
        copy_file(&real.join(rel), &dst.join(rel));
    }
    copy_dir_recursive(&real.join("registries"), &dst.join("registries"));
    copy_dir_recursive(&real.join("schemas"), &dst.join("schemas"));
    copy_dir_recursive(
        &real.join("receipts/admissions"),
        &dst.join("receipts/admissions"),
    );
    copy_dir_recursive(&real.join("kernels"), &dst.join("kernels"));
    // Add an unrelated bystander file outside the outputs column.
    fs::write(dst.join("BYSTANDER.txt"), "do-not-touch\n").expect("write bystander");
    dst
}

fn run_cli_json(root: &Path, extra: &[&str]) -> (ExitCode, String, String) {
    let mut args = vec![
        "check-generated".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    args.extend(extra.iter().map(|s| (*s).to_owned()));
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    (
        code,
        String::from_utf8(out).expect("stdout utf8"),
        String::from_utf8(err).expect("stderr utf8"),
    )
}

fn digest_tree_excluding(dir: &Path, excluded: &BTreeSet<String>) -> String {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(cur) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&cur) {
            for ent in entries.flatten() {
                let p = ent.path();
                if p.is_dir() {
                    stack.push(p);
                } else {
                    let rel = p
                        .strip_prefix(dir)
                        .expect("strip")
                        .to_string_lossy()
                        .replace('\\', "/");
                    if !excluded.contains(&rel) {
                        files.push((rel, p));
                    }
                }
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut h = fs_blake3::Blake3::new();
    for (rel, p) in files {
        h.update(rel.as_bytes());
        h.update(&[0]);
        h.update(&fs::read(&p).expect("read"));
        h.update(&[0]);
    }
    h.finalize().to_hex()
}

#[test]
fn clean_repo_passes_check_generated() {
    let root = repo_root();
    let (code, stdout, stderr) = run_cli_json(&root, &[]);
    assert_eq!(
        code,
        ExitCode::from(EXIT_OK),
        "clean repo must pass check-generated; stdout={stdout}\nstderr={stderr}"
    );
    validate_report_json(stdout.as_bytes()).expect("valid frob.xtask.report/1");
    let adm = admit(stdout.as_bytes(), &Limits::default()).expect("admit report");
    assert_eq!(
        adm.value().get("check").and_then(|v| v.as_str()),
        Some("check-generated")
    );
    assert_eq!(adm.value().get("ok").and_then(|v| v.as_bool()), Some(true));

    // Unknown flag exits 3 (usage).
    let (code_u, _, _) = run_cli_json(&root, &["--bogus"]);
    assert_eq!(code_u, ExitCode::from(EXIT_USAGE));

    // All owned rules are namespaced `gen.*`.
    for &rule in OWNED_RULES {
        assert!(
            rule.starts_with("gen."),
            "rule `{rule}` must start with `gen.`"
        );
    }
}

#[test]
fn generated_schema_roundtrip() {
    let dir = build_scratch_repo("schema_rt");
    // Write all outputs freshly, then verify clean.
    let (c_w, _, _) = run_cli_json(&dir, &["--write"]);
    assert_eq!(c_w, ExitCode::from(EXIT_OK));
    let (c_ok, out_ok, _) = run_cli_json(&dir, &[]);
    assert_eq!(c_ok, ExitCode::from(EXIT_OK), "expected clean: {out_ok}");

    // Flip one byte on line 6 of schemas/frob.design.1.json -> gen.differs with line 6.
    let target = dir.join("schemas/frob.design.1.json");
    let orig = fs::read_to_string(&target).expect("read schema");
    let mut lines: Vec<String> = orig.split('\n').map(ToOwned::to_owned).collect();
    assert!(lines.len() >= 8);
    lines[5].push(' ');
    fs::write(&target, lines.join("\n")).expect("flip one byte on line 6");

    let outcome = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        outcome.findings.iter().any(|f| f.rule == RULE_DIFFERS
            && f.path == "schemas/frob.design.1.json"
            && f.line == Some(6)),
        "expected gen.differs at line 6, got {:?}",
        outcome.findings
    );
    let (c_fail, out_fail, _) = run_cli_json(&dir, &[]);
    assert_eq!(c_fail, ExitCode::from(EXIT_FINDINGS));
    validate_report_json(out_fail.as_bytes()).expect("valid report");
}

#[test]
fn orphan_schema_file_is_finding() {
    let dir = build_scratch_repo("orphan");
    // Hand-written schema under schemas/registries/ raises NO finding.
    fs::write(
        dir.join("schemas/registries/x.schema.json"),
        "{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\"}\n",
    )
    .expect("write hand-written registry schema");
    let ok_out = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        ok_out.findings.is_empty(),
        "hand-written schemas/registries/x.schema.json must not be orphan: {:?}",
        ok_out.findings
    );

    // Stray schemas/frob.unknown.1.json -> gen.orphan.
    fs::write(
        dir.join("schemas/frob.unknown.1.json"),
        "{\"$comment\":\"@generated\",\"$id\":\"frob.unknown/1\"}\n",
    )
    .expect("write stray schema");
    let bad_out = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        bad_out
            .findings
            .iter()
            .any(|f| f.rule == RULE_ORPHAN && f.path == "schemas/frob.unknown.1.json"),
        "expected gen.orphan for schemas/frob.unknown.1.json, got {:?}",
        bad_out.findings
    );
}

#[test]
fn missing_generated_file_is_finding() {
    let dir = build_scratch_repo("missing");
    let aside = fresh_dir("missing_aside");
    fs::rename(
        dir.join("schemas/frob.design.1.json"),
        aside.join("frob.design.1.json"),
    )
    .expect("move schema aside");

    let out = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        out.findings
            .iter()
            .any(|f| f.rule == RULE_MISSING && f.path == "schemas/frob.design.1.json"),
        "expected gen.missing, got {:?}",
        out.findings
    );
}

#[test]
fn marker_required() {
    let dir = build_scratch_repo("marker");
    // Strip the `// @generated` first line from code_gen.rs -> gen.marker-missing.
    let cg = dir.join("crates/frob-core/src/refusal/code_gen.rs");
    let orig = fs::read_to_string(&cg).expect("read code_gen.rs");
    let stripped = orig.split_once('\n').map_or("", |(_, rest)| rest);
    fs::write(&cg, stripped).expect("strip marker");

    let out = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        out.findings.iter().any(|f| f.rule == RULE_MARKER_MISSING
            && f.path == "crates/frob-core/src/refusal/code_gen.rs"),
        "expected gen.marker-missing on stripped code_gen.rs, got {:?}",
        out.findings
    );

    // Strip `$comment` line from a generated schema -> gen.marker-missing.
    let dir2 = build_scratch_repo("marker_schema");
    let sc = dir2.join("schemas/frob.design.1.json");
    let sc_orig = fs::read_to_string(&sc).expect("read schema");
    let sc_no_marker: String = sc_orig
        .lines()
        .filter(|l| !l.contains("$comment"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&sc, format!("{sc_no_marker}\n")).expect("strip $comment");
    let out2 = run_check(&dir2, BUILTIN_GENERATORS, false).expect("check2");
    assert!(
        out2.findings
            .iter()
            .any(|f| f.rule == RULE_MARKER_MISSING && f.path == "schemas/frob.design.1.json"),
        "expected gen.marker-missing on schema without $comment, got {:?}",
        out2.findings
    );
}

#[test]
fn stack_lock_schema_regenerates() {
    let committed = fs::read(repo_root().join("schemas/frob.stack-lock.1.json"))
        .expect("committed schemas/frob.stack-lock.1.json must exist");
    assert_eq!(
        committed,
        lock::json_schema(),
        "committed schemas/frob.stack-lock.1.json must equal stack::lock::json_schema()"
    );

    let dir = build_scratch_repo("stack_lock");
    let p = dir.join("schemas/frob.stack-lock.1.json");
    let mut raw = fs::read_to_string(&p).expect("read");
    raw = raw.replace(
        "\"additionalProperties\": false",
        "\"additionalProperties\": true",
    );
    fs::write(&p, raw).expect("edit 1 line");
    let out = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        out.findings.iter().any(|f| f.rule == RULE_DIFFERS
            && f.path == "schemas/frob.stack-lock.1.json"
            && f.line.is_some()),
        "expected gen.differs on edited stack-lock schema, got {:?}",
        out.findings
    );
}

#[test]
fn write_mode_touches_only_outputs() {
    let dir = build_scratch_repo("write_only_outputs");
    let out_before = run_check(&dir, BUILTIN_GENERATORS, false).expect("initial check");
    assert!(out_before.findings.is_empty());

    // Collect every path produced by BUILTIN_GENERATORS (the outputs column).
    let mut outputs: BTreeSet<String> = BTreeSet::new();
    if let Some(xtask::report::FactValue::Array(rows)) = out_before.facts.get("rows") {
        for r in rows {
            if let xtask::report::FactValue::Object(m) = r
                && let Some(xtask::report::FactValue::Array(arr)) = m.get("outputs")
            {
                for item in arr {
                    if let xtask::report::FactValue::Str(s) = item {
                        outputs.insert(s.clone());
                    }
                }
            }
        }
    }
    assert!(!outputs.is_empty());

    let outside_before = digest_tree_excluding(&dir, &outputs);
    // Corrupt one output so --write actually rewrites files.
    fs::write(dir.join("schemas/frob.design.1.json"), "{}\n").expect("corrupt output");
    let w_out = run_check(&dir, BUILTIN_GENERATORS, true).expect("write mode");
    assert!(
        w_out.findings.is_empty(),
        "write mode must succeed: {:?}",
        w_out.findings
    );
    let outside_after = digest_tree_excluding(&dir, &outputs);
    assert_eq!(
        outside_before, outside_after,
        "--write must not modify any byte outside the outputs column"
    );

    let after = run_check(&dir, BUILTIN_GENERATORS, false).expect("re-check");
    assert!(after.findings.is_empty());
}

#[test]
fn check_generated_detects_edit() {
    // INV-SPARK-08:
    // (a) Unedited tree (including a valid derived kernel with matching manifest digest) is clean.
    let dir = build_scratch_repo("inv_spark_08");
    let kernel_src = b"// @generated from derive/frob-derive (K-cay-step); do not edit by hand.\npub fn cay_step(x: f64) -> f64 { x }\n";
    let kernel_hex = fs_blake3::hash_bytes(kernel_src).to_hex();
    fs::create_dir_all(dir.join("kernels")).expect("mkdir kernels");
    fs::write(dir.join("kernels/K-cay-step.rs"), kernel_src).expect("write kernel");
    fs::write(
        dir.join("registries/kernels.toml"),
        format!(
            "schema = \"frob.kernels/1\"\n\n[[kernel]]\nid = \"K-cay-step\"\nowner = \"frob-kernels\"\n\
determinism = \"D0\"\nsource_digest = \"b3:{kernel_hex}\"\ntargets = [\"cpu\"]\n"
        ),
    )
    .expect("write kernels.toml");

    let clean = run_check(&dir, BUILTIN_GENERATORS, false).expect("clean check");
    assert!(
        clean.findings.is_empty(),
        "unedited tree must be clean: {:?}",
        clean.findings
    );

    // (b) One byte changed in kernel file -> gen.digest naming the file and manifest digest.
    let mut k_mut = kernel_src.to_vec();
    let last = k_mut.len() - 3;
    k_mut[last] = b'y';
    fs::write(dir.join("kernels/K-cay-step.rs"), &k_mut).expect("edit kernel");
    let k_out = run_check(&dir, BUILTIN_GENERATORS, false).expect("kernel check");
    assert!(
        k_out.findings.iter().any(|f| {
            f.rule == RULE_DIGEST
                && f.path == "kernels/K-cay-step.rs"
                && f.message.contains(&kernel_hex)
        }),
        "expected gen.digest naming file and manifest digest, got {:?}",
        k_out.findings
    );
    // Restore kernel for individual regenerable-output edit tests.
    fs::write(dir.join("kernels/K-cay-step.rs"), kernel_src).expect("restore kernel");

    // (c) One byte changed in EACH of the five regenerable output kinds -> gen.differs + line.
    let targets: &[(&str, &str, &str)] = &[
        (
            "schemas/frob.design.1.json",
            "\"owner\": \"frob-design\"",
            "\"owner\": \"frob-desigX\"",
        ),
        (
            "schemas/frob.stack-lock.1.json",
            "\"required\": [\"toolchain\"]",
            "\"required\": [\"toolchaiX\"]",
        ),
        (
            "DEPENDENCY_ADMISSION.md",
            "| `INT-FSIM-01` | `FD-FSIM` |",
            "| `INT-FSIM-01` | `FD-FSIX` |",
        ),
        (
            "crates/frob-core/src/refusal/code_gen.rs",
            "FrobRef802 = 802,",
            "FrobRef802 = 803,",
        ),
        ("README.md", "Summary: ", "Summary: X"),
    ];
    for &(rel, from, to) in targets {
        let p = dir.join(rel);
        let orig = fs::read_to_string(&p).expect("read target");
        assert!(orig.contains(from), "expected `{from}` in `{rel}`");
        let expected_line = orig
            .lines()
            .position(|l| l.contains(from))
            .map(|i| (i + 1) as u32)
            .expect("find line");
        fs::write(&p, orig.replacen(from, to, 1)).expect("plant edit");
        let res = run_check(&dir, BUILTIN_GENERATORS, false).expect("check edit");
        assert!(
            res.findings
                .iter()
                .any(|f| f.rule == RULE_DIFFERS && f.path == rel && f.line == Some(expected_line)),
            "expected gen.differs at {rel}:{expected_line}, got {:?}",
            res.findings
        );
        fs::write(&p, orig).expect("restore target");
    }
}

#[test]
fn write_mode_does_not_rewrite_digests() {
    let dir = build_scratch_repo("write_no_digest");
    let orig_kernel =
        b"// @generated from derive/frob-derive; do not edit.\npub const K: f64 = 1.0;\n";
    let orig_hex = fs_blake3::hash_bytes(orig_kernel).to_hex();
    fs::create_dir_all(dir.join("kernels")).expect("mkdir kernels");
    let mutated_kernel =
        b"// @generated from derive/frob-derive; do not edit.\npub const K: f64 = 2.0;\n";
    fs::write(dir.join("kernels/K-norm.rs"), mutated_kernel).expect("write mutated kernel");
    let manifest_text = format!(
        "schema = \"frob.kernels/1\"\n\n[[kernel]]\nid = \"K-norm\"\nowner = \"frob-kernels\"\n\
determinism = \"D0\"\nsource_digest = \"b3:{orig_hex}\"\ntargets = [\"cpu\"]\n"
    );
    fs::write(dir.join("registries/kernels.toml"), &manifest_text).expect("write kernels.toml");

    let out = run_check(&dir, BUILTIN_GENERATORS, true).expect("write mode");
    // `--write` must NOT rewrite kernels.toml or kernels/K-norm.rs, and must still report gen.digest.
    assert_eq!(
        fs::read_to_string(dir.join("registries/kernels.toml")).expect("read manifest"),
        manifest_text
    );
    assert_eq!(
        fs::read(dir.join("kernels/K-norm.rs")).expect("read kernel"),
        mutated_kernel
    );
    assert!(
        out.findings
            .iter()
            .any(|f| f.rule == RULE_DIGEST && f.path == "kernels/K-norm.rs"),
        "write mode must still report gen.digest, got {:?}",
        out.findings
    );
}

#[test]
fn status_block_regenerates_byte_identical() {
    let dir = build_scratch_repo("status_idem");
    let before = fs::read(dir.join("README.md")).expect("read README");
    let w = run_check(&dir, BUILTIN_GENERATORS, true).expect("write");
    assert!(w.findings.is_empty());
    let after = fs::read(dir.join("README.md")).expect("read README after");
    assert_eq!(before, after, "README.md must regenerate byte-identical");
}

#[test]
fn status_pass_without_measured_is_invalid() {
    let dir = build_scratch_repo("status_invalid");
    fs::write(
        dir.join("registries/gates.toml"),
        "schema = \"frob.gates/1\"\n\n\
[[gate]]\nid = \"GATE-BAD-01\"\nclaim = \"known-answer\"\nclaim_id = \"clm:bad\"\n\
section = \"38.13\"\nthreshold = \"exact\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n\n\
[[gate.part]]\nmilestone = \"M0\"\nstatus = \"pass\"\nmeasured = \"\"\nevidence = [\"seq:1\"]\n\
recorded = { date = \"2026-11-24\", session = \"s:rev\", implemented_by = \"s:impl\" }\n",
    )
    .expect("write bad gates.toml");

    let (code, stdout, _) = run_cli_json(&dir, &[]);
    assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
    let out = run_check(&dir, BUILTIN_GENERATORS, false).expect("check");
    assert!(
        out.findings.iter().any(|f| f.rule == RULE_STATUS_INVALID),
        "expected gen.status-invalid, got {:?}\nstdout={stdout}",
        out.findings
    );
}

#[test]
fn input_globs_per_row() {
    fn synthetic_two_glob_gen(
        repo_root: &Path,
        inputs: &[PathBuf],
    ) -> Result<GeneratorOutput, GeneratorError> {
        let mut combined = String::from("// @generated synthetic two-glob output\n");
        for rel in inputs {
            let txt = fs::read_to_string(repo_root.join(rel)).unwrap_or_default();
            combined.push_str(&format!("# {}\n{}\n", rel.display(), txt.trim()));
        }
        Ok(GeneratorOutput::Regenerated(vec![GeneratedFile {
            rel_path: "out/combined.txt".to_owned(),
            kind: OutputKind::WholeFile,
            marker: ExpectedMarker::LinePrefix("// @generated"),
            bytes: combined.into_bytes(),
        }]))
    }

    const SYNTHETIC_ROWS: &[GeneratorRow] = &[GeneratorRow {
        name: "synthetic",
        input_globs: &["inputs/a/*.txt", "inputs/b/*.txt"],
        output_globs: &["out/combined.txt"],
        mode: GeneratorMode::Regenerate,
        run: synthetic_two_glob_gen,
    }];

    let dir = fresh_dir("two_globs");
    fs::create_dir_all(dir.join("inputs/a")).expect("mkdir a");
    fs::create_dir_all(dir.join("inputs/b")).expect("mkdir b");
    fs::write(dir.join("inputs/a/first.txt"), "alpha\n").expect("write a");
    fs::write(dir.join("inputs/b/second.txt"), "beta\n").expect("write b");

    let w1 = run_check(&dir, SYNTHETIC_ROWS, true).expect("write 1");
    assert!(w1.findings.is_empty());
    let before = fs::read_to_string(dir.join("out/combined.txt")).expect("read combined");
    assert!(before.contains("alpha") && before.contains("beta"));
    assert!(
        run_check(&dir, SYNTHETIC_ROWS, false)
            .expect("check 1")
            .findings
            .is_empty()
    );

    // Edit ONLY the second input glob -> check fails with gen.differs, and --write updates output.
    fs::write(dir.join("inputs/b/second.txt"), "gamma\n").expect("edit b");
    let diff = run_check(&dir, SYNTHETIC_ROWS, false).expect("check after b edit");
    assert!(
        diff.findings
            .iter()
            .any(|f| f.rule == RULE_DIFFERS && f.path == "out/combined.txt"),
        "edit to second input glob alone must trigger gen.differs: {:?}",
        diff.findings
    );
    let _ = run_check(&dir, SYNTHETIC_ROWS, true).expect("rewrite");
    let after = fs::read_to_string(dir.join("out/combined.txt")).expect("read after");
    assert_ne!(before, after);
    assert!(after.contains("gamma"));
}

#[test]
fn json_report_is_deterministic_excluding_elapsed_ms() {
    let dir = build_scratch_repo("determinism");
    let (c1, s1, _) = run_cli_json(&dir, &[]);
    let (c2, s2, _) = run_cli_json(&dir, &[]);
    assert_eq!(c1, ExitCode::from(EXIT_OK));
    assert_eq!(c2, ExitCode::from(EXIT_OK));
    let strip_elapsed = |s: &str| -> String {
        if let Some(pos) = s.find("\"elapsed_ms\":") {
            let rest = &s[pos..];
            let end = rest.find([',', '}']).unwrap_or(rest.len());
            format!("{}{}", &s[..pos], &rest[end..])
        } else {
            s.to_owned()
        }
    };
    assert_eq!(strip_elapsed(&s1), strip_elapsed(&s2));
    let _ = check_generated::CHECK_GENERATED_FACTS_SCHEMA;
}
