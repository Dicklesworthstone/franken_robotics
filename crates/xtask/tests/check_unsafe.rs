//! Integration and unit tests for `xtask check-unsafe` (bead N34-07, GATE-WS-01 plant 4, KA-784).
//!
//! Synthetic scratch trees (two-crate core + one annex) are created under
//! `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BronzePelican/` and never deleted (COMMON.md 9b).
//! Every test executes the real manifest reader, Rust tokenizer, `frob_core::registry::toml`
//! capsule parser, `SAFETY.md` validator, and `frob.xtask.report/1` canonical writer.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use frob_core::json::{Limits, admit};
use xtask::check_unsafe::{
    self, AnnexTotals, CAPSULE_MAX_LINES, CapsuleFact, OWNED_RULES,
    RULE_CAPSULE_COUNT_MISMATCH, RULE_CAPSULE_NO_SAFETY_DOC, RULE_CAPSULE_OVER_LIMIT,
    RULE_CAPSULE_STALE_ROW, RULE_CAPSULE_UNREGISTERED, RULE_CAPSULE_UNSAFE_IN_API,
    RULE_DOORWAY_UNLISTED_ALLOW, RULE_LINTS_NOT_INHERITED, RULE_UNSAFE_CERT_FORBID,
    RULE_UNSAFE_CORE_LINT, RULE_UNSAFE_CORE_TOKEN, SAFETY_DOC_SECTIONS, check_unsafe,
    format_capsule_table,
};
use xtask::cli::{self, CheckFn, EXIT_FINDINGS, EXIT_OK};
use xtask::metadata::WorkspaceRoot;
use xtask::report::{CheckReport, validate_report_json};
use xtask::status_table;

static COUNTER: AtomicU64 = AtomicU64::new(0);

const TEST_COMMANDS: &[(&str, CheckFn)] = &[("check-unsafe", check_unsafe::run_command)];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/xtask sits two levels below repo root")
        .to_path_buf()
}

fn make_scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = base
        .join("frob-BronzePelican")
        .join(format!("xtask-check-unsafe-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write synthetic fixture file");
}

/// Seed a clean two-crate core + one annex scratch tree that passes every rule of `check-unsafe`.
fn create_clean_synthetic_tree(label: &str) -> PathBuf {
    let root = make_scratch_dir(label);
    write_file(
        &root.join("Cargo.toml"),
        "[workspace]\n\
         resolver = \"3\"\n\
         members = [\"crates/*\"]\n\
         exclude = [\"spark\"]\n\
         \n\
         [workspace.lints.rust]\n\
         unsafe_code = \"deny\"\n\
         deprecated = \"deny\"\n",
    );
    write_file(
        &root.join("crates/core-alpha/Cargo.toml"),
        "[package]\n\
         name = \"core-alpha\"\n\
         version = \"0.0.1\"\n\
         edition = \"2024\"\n\
         \n\
         [lints]\n\
         workspace = true\n",
    );
    write_file(
        &root.join("crates/core-alpha/src/lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
    );
    write_file(
        &root.join("crates/core-beta/Cargo.toml"),
        "[package]\n\
         name = \"core-beta\"\n\
         version = \"0.0.1\"\n\
         edition = \"2024\"\n\
         \n\
         [lints]\n\
         workspace = true\n",
    );
    write_file(
        &root.join("crates/core-beta/src/lib.rs"),
        "pub fn mul(a: i32, b: i32) -> i32 { a * b }\n",
    );
    write_file(
        &root.join("spark/frob-spark-cuda/Cargo.toml"),
        "[package]\n\
         name = \"frob-spark-cuda\"\n\
         version = \"0.0.1\"\n\
         edition = \"2024\"\n",
    );
    write_file(
        &root.join("spark/frob-spark-cuda/src/lib.rs"),
        "pub fn safe_entry() -> u32 { 42 }\n",
    );
    write_file(
        &root.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n",
    );
    root
}

fn full_safety_doc() -> &'static str {
    "# Capsule Safety\n\
     \n\
     ## What is unsafe here\n\
     Calls the driver loader FFI function `cuInit` with a zero flag; the vendor driver is assumed to preserve process memory outside its own handle tables.\n\
     \n\
     ## Invariants the safe API maintains\n\
     `DriverHandle::open` checks every error return code before exposing an initialized handle and owns no aliased mutable references.\n\
     \n\
     ## Why safe Rust cannot do this\n\
     Dynamic driver symbol lookup and C ABI calls cannot be expressed in safe Rust.\n\
     \n\
     ## How it is tested\n\
     Covered by `driver_load_open_and_symbol` unit test and device harness smoke check.\n"
}

fn run_check(root: &Path) -> Vec<xtask::crate_map::Finding> {
    let outcome = check_unsafe(root, &[WorkspaceRoot::core(root)])
        .expect("check_unsafe should succeed without tool fault");
    outcome.findings
}

fn run_cli_json(root: &Path) -> (ExitCode, String) {
    let args = vec![
        "check-unsafe".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert!(
        err.is_empty(),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&err)
    );
    let json_text = String::from_utf8(out).expect("stdout must be UTF-8");
    validate_report_json(json_text.as_bytes()).expect("report JSON must satisfy frob.xtask.report/1");
    (code, json_text)
}

fn count_lines_exact(s: &str) -> usize {
    s.lines().count()
}

// ============================================================================
// Rule 1 & Rule 2 tests: core deny lint, allow attr, cfg-gated unsafe, strings/comments
// ============================================================================

#[test]
fn core_deny_lint_required() {
    // Missing [workspace.lints.rust] entirely fails with unsafe.core-lint.
    let root_missing = create_clean_synthetic_tree("core-deny-missing");
    write_file(
        &root_missing.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\nexclude = [\"spark\"]\n\n[workspace.lints.rust]\ndeprecated = \"deny\"\n",
    );
    let findings = run_check(&root_missing);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_UNSAFE_CORE_LINT);
    assert_eq!(findings[0].path, "Cargo.toml");

    // Weakened to `warn` also fails with unsafe.core-lint.
    let root_warn = create_clean_synthetic_tree("core-deny-warn");
    write_file(
        &root_warn.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\nexclude = [\"spark\"]\n\
         \n[workspace.lints.rust]\nunsafe_code = \"warn\"\ndeprecated = \"deny\"\n",
    );
    let findings_warn = run_check(&root_warn);
    assert_eq!(findings_warn.len(), 1, "expected 1 finding, got: {findings_warn:?}");
    assert_eq!(findings_warn[0].rule, RULE_UNSAFE_CORE_LINT);
    assert_eq!(findings_warn[0].path, "Cargo.toml");
}

#[test]
fn core_allow_attribute_fails() {
    // Inner `#![allow(unsafe_code)]` fails with unsafe.core-lint.
    let root_inner = create_clean_synthetic_tree("core-allow-inner");
    write_file(
        &root_inner.join("crates/core-alpha/src/lib.rs"),
        "#![allow(unsafe_code)]\npub fn f() -> u32 { 1 }\n",
    );
    let findings_inner = run_check(&root_inner);
    assert_eq!(findings_inner.len(), 1, "expected 1 finding, got: {findings_inner:?}");
    assert_eq!(findings_inner[0].rule, RULE_UNSAFE_CORE_LINT);
    assert_eq!(findings_inner[0].path, "crates/core-alpha/src/lib.rs");
    assert_eq!(findings_inner[0].line, Some(1));

    // Outer `#[allow(unsafe_code)]` and `cfg_attr(..., allow(unsafe_code))` also fail.
    let root_outer = create_clean_synthetic_tree("core-allow-outer");
    write_file(
        &root_outer.join("crates/core-beta/src/lib.rs"),
        "#[allow(dead_code, unsafe_code)]\n\
         pub fn g() -> u32 { 2 }\n\
         #[cfg_attr(feature = \"never\", allow(unsafe_code))]\n\
         pub fn h() -> u32 { 3 }\n",
    );
    let findings_outer = run_check(&root_outer);
    assert_eq!(findings_outer.len(), 2, "expected 2 findings, got: {findings_outer:?}");
    assert!(findings_outer.iter().all(|f| f.rule == RULE_UNSAFE_CORE_LINT));
}

#[test]
fn core_unsafe_token_behind_cfg_fails() {
    let root = create_clean_synthetic_tree("core-unsafe-cfg");
    write_file(
        &root.join("crates/core-alpha/src/lib.rs"),
        "#[cfg(feature = \"never\")]\n\
         pub fn hidden_ub() {\n\
             unsafe {\n\
                 let _ = 0usize;\n\
             }\n\
         }\n",
    );
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_UNSAFE_CORE_TOKEN);
    assert_eq!(findings[0].subject, "core-alpha");
    assert_eq!(findings[0].path, "crates/core-alpha/src/lib.rs");
    assert_eq!(findings[0].line, Some(3));
}

#[test]
fn unsafe_in_string_or_comment_is_ignored() {
    let root = create_clean_synthetic_tree("unsafe-strings-comments");
    write_file(
        &root.join("crates/core-alpha/src/lib.rs"),
        "//! Doc comment mentioning unsafe { } and #[allow(unsafe_code)] and #[allow(deprecated)].\n\
         // Line comment: unsafe fn bogus() {}\n\
         /* Outer block comment /* nested unsafe { } */ still comment unsafe fn bad() */\n\
         pub fn inspect<'a>(s: &'a str) -> (&'a str, char, &'static [u8]) {\n\
             let _s1 = \"unsafe { let x = 1; }\";\n\
             let _s2 = r#\"unsafe fn foo() -> *const u8 { #[allow(deprecated)] }\"#;\n\
             let _s3 = b\"unsafe block\";\n\
             let _s4 = br#\"unsafe extern\"#;\n\
             let _s5 = c\"unsafe cstr\";\n\
             let _s6 = cr#\"unsafe raw cstr\"#;\n\
             let _q = '\"';\n\
             let _slash = '/';\n\
             let _esc = '\\'';\n\
             let _byte_q = b'\"';\n\
             let _ident_with_substring = \"ok\";\n\
             let unsafe_count_label = 0usize;\n\
             let _ = unsafe_count_label;\n\
             (s, _q, _s3)\n\
         }\n",
    );
    let findings = run_check(&root);
    assert!(
        findings.is_empty(),
        "strings/comments/char-lits/lifetimes must not trigger findings: {findings:?}"
    );
}

// ============================================================================
// Rule 3 test: frob-cert requires `#![forbid(unsafe_code)]` on lib & bin roots
// ============================================================================

#[test]
fn cert_forbid_required() {
    // 1. frob-cert with `#![forbid(unsafe_code)]` in both lib and bin roots passes
    //    (and is exempted from `[lints] workspace = true` per ADR-WS-02).
    let root_ok = create_clean_synthetic_tree("cert-forbid-ok");
    write_file(
        &root_ok.join("crates/frob-cert/Cargo.toml"),
        "[package]\nname = \"frob-cert\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(
        &root_ok.join("crates/frob-cert/src/lib.rs"),
        "#![forbid(unsafe_code)]\npub fn verify() -> bool { true }\n",
    );
    write_file(
        &root_ok.join("crates/frob-cert/src/bin/frob-verify.rs"),
        "#![forbid(unsafe_code)]\nfn main() {}\n",
    );
    let ok_findings = run_check(&root_ok);
    assert!(
        ok_findings.is_empty(),
        "clean frob-cert should pass without findings: {ok_findings:?}"
    );

    // 2. Missing `#![forbid(unsafe_code)]` in lib.rs fails with unsafe.cert-forbid.
    let root_no_lib = create_clean_synthetic_tree("cert-forbid-missing-lib");
    write_file(
        &root_no_lib.join("crates/frob-cert/Cargo.toml"),
        "[package]\nname = \"frob-cert\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(
        &root_no_lib.join("crates/frob-cert/src/lib.rs"),
        "// #![forbid(unsafe_code)] commented out must not pass!\npub fn verify() -> bool { true }\n",
    );
    write_file(
        &root_no_lib.join("crates/frob-cert/src/bin/frob-verify.rs"),
        "#![forbid(unsafe_code)]\nfn main() {}\n",
    );
    let lib_findings = run_check(&root_no_lib);
    assert_eq!(lib_findings.len(), 1, "expected 1 finding, got: {lib_findings:?}");
    assert_eq!(lib_findings[0].rule, RULE_UNSAFE_CERT_FORBID);
    assert_eq!(lib_findings[0].path, "crates/frob-cert/src/lib.rs");

    // 3. Missing `#![forbid(unsafe_code)]` in binary root fails with unsafe.cert-forbid.
    let root_no_bin = create_clean_synthetic_tree("cert-forbid-missing-bin");
    write_file(
        &root_no_bin.join("crates/frob-cert/Cargo.toml"),
        "[package]\nname = \"frob-cert\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(
        &root_no_bin.join("crates/frob-cert/src/lib.rs"),
        "#![forbid(unsafe_code)]\npub fn verify() -> bool { true }\n",
    );
    write_file(
        &root_no_bin.join("crates/frob-cert/src/bin/frob-verify.rs"),
        "fn main() {}\n",
    );
    let bin_findings = run_check(&root_no_bin);
    assert_eq!(bin_findings.len(), 1, "expected 1 finding, got: {bin_findings:?}");
    assert_eq!(bin_findings[0].rule, RULE_UNSAFE_CERT_FORBID);
    assert_eq!(bin_findings[0].path, "crates/frob-cert/src/bin/frob-verify.rs");
}

// ============================================================================
// Rules 4, 5, 6, 7, 8, 9 tests: annex capsules & SAFETY.md
// ============================================================================

#[test]
fn unregistered_capsule_fails() {
    // Plant 4 of GATE-WS-01 (KA-784): unsafe block in annex file with no registry row -> exit 2.
    let root = create_clean_synthetic_tree("capsule-unregistered");
    write_file(
        &root.join("spark/frob-spark-cuda/src/driver_load.rs"),
        "pub fn init() -> u32 {\n    unsafe { 7 }\n}\n",
    );
    let (code, json_text) = run_cli_json(&root);
    assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
    let admitted = admit(json_text.as_bytes(), &Limits::default()).expect("admit JSON");
    let findings = admitted
        .value()
        .get("findings")
        .and_then(|v| v.as_array())
        .expect("findings array");
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].get("rule").and_then(|v| v.as_str()),
        Some(RULE_CAPSULE_UNREGISTERED)
    );
    assert_eq!(
        findings[0].get("path").and_then(|v| v.as_str()),
        Some("spark/frob-spark-cuda/src/driver_load.rs")
    );
}

#[test]
fn registered_capsule_passes() {
    // Build a capsule of exactly 299 lines (the upper passing boundary) and create its
    // SAFETY.md by copying the checked-in `registries/capsules.SAFETY.template.md` and
    // filling all four sections.
    let root = create_clean_synthetic_tree("capsule-registered-299");
    let mut cap_src = String::from(
        "pub struct DriverHandle { id: u32 }\n\
         impl DriverHandle {\n\
             pub fn open() -> Result<Self, ()> {\n\
                 let id = unsafe { 1u32 };\n\
                 Ok(Self { id })\n\
             }\n\
             pub fn symbol(&self) -> u32 {\n\
                 unsafe { self.id }\n\
             }\n\
         }\n",
    );
    while count_lines_exact(&cap_src) < CAPSULE_MAX_LINES - 1 {
        cap_src.push_str("// padding line to test exact 299-line capsule boundary\n");
    }
    assert_eq!(count_lines_exact(&cap_src), 299);

    let cap_rel = "spark/frob-spark-cuda/src/capsule/driver_load.rs";
    let doc_rel = "spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md";
    write_file(&root.join(cap_rel), &cap_src);

    // Copy checked-in template and fill its four sections.
    let template_path = repo_root().join("registries/capsules.SAFETY.template.md");
    let template = fs::read_to_string(&template_path)
        .expect("checked-in registries/capsules.SAFETY.template.md must exist");
    let filled = template
        .replace(
            "## What is unsafe here",
            "## What is unsafe here\n\nCalls vendor driver open/symbol FFI; driver preserves host memory.",
        )
        .replace(
            "## Invariants the safe API maintains",
            "## Invariants the safe API maintains\n\n`DriverHandle::open` validates driver status before returning.",
        )
        .replace(
            "## Why safe Rust cannot do this",
            "## Why safe Rust cannot do this\n\nDynamic C ABI symbol lookup requires FFI.",
        )
        .replace(
            "## How it is tested",
            "## How it is tested\n\nTested by `registered_capsule_passes`.",
        );
    write_file(&root.join(doc_rel), &filled);

    write_file(
        &root.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.driver-load\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
         safe_api = [\"DriverHandle::open\", \"DriverHandle::symbol\"]\n\
         lines = 299\n\
         unsafe_blocks = 2\n\
         safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
         reason = \"dlopen of the vendor driver and symbol lookup\"\n",
    );

    let findings = run_check(&root);
    assert!(
        findings.is_empty(),
        "299-line registered capsule with filled template SAFETY.md must pass: {findings:?}"
    );
}

#[test]
fn capsule_at_300_lines_fails() {
    let root = create_clean_synthetic_tree("capsule-300-lines");
    let mut cap_src = String::from("pub fn open() -> u32 { unsafe { 1 } }\n");
    while count_lines_exact(&cap_src) < CAPSULE_MAX_LINES {
        cap_src.push_str("// comment line counting toward 300-line limit\n");
    }
    assert_eq!(count_lines_exact(&cap_src), 300);

    write_file(
        &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
        &cap_src,
    );
    write_file(
        &root.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
        full_safety_doc(),
    );
    write_file(
        &root.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.driver-load\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
         safe_api = [\"open\"]\n\
         lines = 300\n\
         unsafe_blocks = 1\n\
         safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
         reason = \"test 300-line limit\"\n",
    );

    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CAPSULE_OVER_LIMIT);
    assert_eq!(findings[0].subject, "cuda.driver-load");
}

#[test]
fn capsule_count_mismatch_fails() {
    let cap_src = "pub fn open() -> u32 {\n    let a = unsafe { 1 };\n    let b = unsafe { 2 };\n    a + b\n}\n";
    assert_eq!(count_lines_exact(cap_src), 5);

    // 1. Wrong `lines` count.
    let root_lines = create_clean_synthetic_tree("capsule-mismatch-lines");
    write_file(
        &root_lines.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
        cap_src,
    );
    write_file(
        &root_lines.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
        full_safety_doc(),
    );
    write_file(
        &root_lines.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.driver-load\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
         safe_api = [\"open\"]\n\
         lines = 4\n\
         unsafe_blocks = 2\n\
         safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
         reason = \"line count mismatch\"\n",
    );
    let f_lines = run_check(&root_lines);
    assert_eq!(f_lines.len(), 1, "expected 1 finding, got: {f_lines:?}");
    assert_eq!(f_lines[0].rule, RULE_CAPSULE_COUNT_MISMATCH);

    // 2. Wrong `unsafe_blocks` count.
    let root_blocks = create_clean_synthetic_tree("capsule-mismatch-blocks");
    write_file(
        &root_blocks.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
        cap_src,
    );
    write_file(
        &root_blocks.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
        full_safety_doc(),
    );
    write_file(
        &root_blocks.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.driver-load\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
         safe_api = [\"open\"]\n\
         lines = 5\n\
         unsafe_blocks = 1\n\
         safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
         reason = \"unsafe_blocks mismatch\"\n",
    );
    let f_blocks = run_check(&root_blocks);
    assert_eq!(f_blocks.len(), 1, "expected 1 finding, got: {f_blocks:?}");
    assert_eq!(f_blocks[0].rule, RULE_CAPSULE_COUNT_MISMATCH);
}

#[test]
fn stale_row_fails() {
    let root = create_clean_synthetic_tree("capsule-stale-row");
    write_file(
        &root.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.deleted\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/capsule/gone.rs\"\n\
         safe_api = [\"open\"]\n\
         lines = 10\n\
         unsafe_blocks = 1\n\
         safety_doc = \"spark/frob-spark-cuda/src/capsule/gone.SAFETY.md\"\n\
         reason = \"deleted file\"\n",
    );
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CAPSULE_STALE_ROW);
    assert_eq!(findings[0].subject, "cuda.deleted");
}

#[test]
fn safety_doc_sections_required() {
    let cap_src = "pub fn open() -> u32 { unsafe { 1 } }\n";
    let cap_toml = "schema = \"frob.capsules/1\"\n\
                    \n\
                    [[capsule]]\n\
                    id = \"cuda.driver-load\"\n\
                    workspace = \"spark-cuda\"\n\
                    path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
                    safe_api = [\"open\"]\n\
                    lines = 1\n\
                    unsafe_blocks = 1\n\
                    safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
                    reason = \"test safety doc\"\n";

    // Case 0: Missing SAFETY.md altogether.
    {
        let root = create_clean_synthetic_tree("safety-doc-missing-file");
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
            cap_src,
        );
        write_file(&root.join("registries/capsules.toml"), cap_toml);
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "missing file should emit 1 finding: {findings:?}");
        assert_eq!(findings[0].rule, RULE_CAPSULE_NO_SAFETY_DOC);
    }

    // Cases 1..=4: One case per missing required section.
    for (idx, omitted) in SAFETY_DOC_SECTIONS.iter().enumerate() {
        let root = create_clean_synthetic_tree(&format!("safety-doc-omit-{idx}"));
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
            cap_src,
        );
        write_file(&root.join("registries/capsules.toml"), cap_toml);

        let mut md = String::from("# Capsule Safety\n\n");
        for &sec in SAFETY_DOC_SECTIONS {
            if sec != *omitted {
                md.push_str(&format!("## {sec}\n\nDocumented details for {sec}.\n\n"));
            }
        }
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
            &md,
        );

        let findings = run_check(&root);
        assert_eq!(
            findings.len(),
            1,
            "omitting `{omitted}` should yield 1 finding, got: {findings:?}"
        );
        assert_eq!(findings[0].rule, RULE_CAPSULE_NO_SAFETY_DOC);
        assert!(
            findings[0].message.contains(omitted),
            "message should name missing section `{omitted}`: {}",
            findings[0].message
        );
    }

    // Case 5: Section present (`Invariants the safe API maintains`) but empty.
    {
        let root = create_clean_synthetic_tree("safety-doc-empty-section");
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
            cap_src,
        );
        write_file(&root.join("registries/capsules.toml"), cap_toml);
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
            "## What is unsafe here\nFFI call.\n\
             ## Invariants the safe API maintains\n\n\
             ## Why safe Rust cannot do this\nC ABI.\n\
             ## How it is tested\nUnit test.\n",
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "empty section should yield 1 finding: {findings:?}");
        assert_eq!(findings[0].rule, RULE_CAPSULE_NO_SAFETY_DOC);
        assert!(findings[0].message.contains("Invariants the safe API maintains"));
    }

    // Case 6: Unfilled template copy (only HTML comment placeholders) must fail all 4 sections.
    {
        let root = create_clean_synthetic_tree("safety-doc-unfilled-template");
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
            cap_src,
        );
        write_file(&root.join("registries/capsules.toml"), cap_toml);
        let template = fs::read_to_string(repo_root().join("registries/capsules.SAFETY.template.md"))
            .expect("read template");
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
            &template,
        );
        let findings = run_check(&root);
        assert_eq!(
            findings.len(),
            4,
            "unfilled template should report 4 empty sections, got: {findings:?}"
        );
        assert!(findings.iter().all(|f| f.rule == RULE_CAPSULE_NO_SAFETY_DOC));
    }
}

#[test]
fn unsafe_fn_in_safe_api_fails() {
    // Case A: `safe_api` item declared `unsafe fn`.
    {
        let root = create_clean_synthetic_tree("safe-api-unsafe-fn");
        let cap_src = "pub struct DriverHandle;\n\
                       impl DriverHandle {\n\
                           pub unsafe fn open() -> Self { Self }\n\
                       }\n";
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
            cap_src,
        );
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
            full_safety_doc(),
        );
        write_file(
            &root.join("registries/capsules.toml"),
            "schema = \"frob.capsules/1\"\n\
             \n\
             [[capsule]]\n\
             id = \"cuda.driver-load\"\n\
             workspace = \"spark-cuda\"\n\
             path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
             safe_api = [\"DriverHandle::open\"]\n\
             lines = 4\n\
             unsafe_blocks = 1\n\
             safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
             reason = \"unsafe fn in safe_api\"\n",
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
        assert_eq!(findings[0].rule, RULE_CAPSULE_UNSAFE_IN_API);
        assert!(findings[0].message.contains("unsafe fn"));
    }

    // Case B: `safe_api` item returns raw pointer (`*const` and `Result<*mut u8, ()>`).
    {
        let root = create_clean_synthetic_tree("safe-api-raw-ptr");
        let cap_src = "pub struct DriverHandle;\n\
                       impl DriverHandle {\n\
                           pub fn symbol(&self) -> *const u8 {\n\
                               unsafe { core::ptr::null() }\n\
                           }\n\
                       }\n\
                       pub fn alloc_buf() -> Result<*mut u8, ()> {\n\
                           Ok(unsafe { core::ptr::null_mut() })\n\
                       }\n";
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.rs"),
            cap_src,
        );
        write_file(
            &root.join("spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md"),
            full_safety_doc(),
        );
        write_file(
            &root.join("registries/capsules.toml"),
            "schema = \"frob.capsules/1\"\n\
             \n\
             [[capsule]]\n\
             id = \"cuda.driver-load\"\n\
             workspace = \"spark-cuda\"\n\
             path = \"spark/frob-spark-cuda/src/capsule/driver_load.rs\"\n\
             safe_api = [\"DriverHandle::symbol\", \"alloc_buf\"]\n\
             lines = 9\n\
             unsafe_blocks = 2\n\
             safety_doc = \"spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md\"\n\
             reason = \"raw pointer return in safe_api\"\n",
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 2, "expected 2 findings, got: {findings:?}");
        assert!(findings.iter().all(|f| f.rule == RULE_CAPSULE_UNSAFE_IN_API));
        assert!(findings.iter().all(|f| f.message.contains("raw pointer")));
    }
}

// ============================================================================
// Rule 10 tests: lint-table inheritance & library print overrides
// ============================================================================

#[test]
fn lint_table_inheritance_required() {
    // Core crate missing `[lints] workspace = true` emits ONLY rule 10 (`lints.not-inherited`),
    // not `unsafe.core-lint` (single defect -> single finding).
    let root = create_clean_synthetic_tree("lint-not-inherited");
    write_file(
        &root.join("crates/core-alpha/Cargo.toml"),
        "[package]\nname = \"core-alpha\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_LINTS_NOT_INHERITED);
    assert_eq!(findings[0].subject, "core-alpha");
    assert_eq!(findings[0].path, "crates/core-alpha/Cargo.toml");
}

#[test]
fn library_print_override_fails() {
    // One failing case for each of the three L-7 lints in a library target root.
    for lint in [
        "clippy::print_stdout",
        "clippy::print_stderr",
        "clippy::dbg_macro",
    ] {
        let slug = lint.replace("::", "-");
        let root = create_clean_synthetic_tree(&format!("lib-override-{slug}"));
        write_file(
            &root.join("crates/core-alpha/src/lib.rs"),
            &format!("#![allow({lint})]\npub fn f() {{}}\n"),
        );
        let findings = run_check(&root);
        assert_eq!(
            findings.len(),
            1,
            "library `#![allow({lint})]` should yield 1 finding, got: {findings:?}"
        );
        assert_eq!(findings[0].rule, RULE_LINTS_NOT_INHERITED);
        assert_eq!(findings[0].path, "crates/core-alpha/src/lib.rs");
    }

    // The same `#![allow(...)]` at binary roots (`src/main.rs` and `src/bin/tool.rs`) MUST pass!
    let root_bin = create_clean_synthetic_tree("bin-print-allow-passes");
    write_file(
        &root_bin.join("crates/core-alpha/src/main.rs"),
        "#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]\nfn main() {}\n",
    );
    write_file(
        &root_bin.join("crates/core-beta/src/bin/tool.rs"),
        "#![allow(clippy::print_stdout)]\n#![allow(clippy::print_stderr)]\n#![allow(clippy::dbg_macro)]\nfn main() {}\n",
    );
    let bin_findings = run_check(&root_bin);
    assert!(
        bin_findings.is_empty(),
        "binary roots allowing L-7 print lints must pass: {bin_findings:?}"
    );
}

// ============================================================================
// Rule 11 tests: doorway allow(deprecated) scan
// ============================================================================

#[test]
fn doorway_lint_must_be_denied() {
    let root = create_clean_synthetic_tree("doorway-lint-level");
    let path = root.join("Cargo.toml");
    let manifest = fs::read_to_string(&path).expect("read scratch manifest");
    for replacement in ["", "deprecated = \"warn\"", "deprecated = \"forbid\""] {
        write_file(
            &path,
            &manifest.replace("deprecated = \"deny\"", replacement),
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].rule, RULE_LINTS_NOT_INHERITED);
        assert_eq!(findings[0].path, "Cargo.toml");
    }
}

#[test]
fn doorway_listed_site_passes() {
    const TESTED: &str = r#"
        let _ = Evidence::<ProofClaim>::doorway_tested(scope.clone(), AssumptionSet::default(), Vec::new());
        let _ = ClaimProduct::tested(ClaimKind::KnownAnswer, input.clone(), scope.clone(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
    "#;
    const PROOF: &str = r#"
        let _ = Evidence::<ProofClaim>::doorway_certified(scope.clone(), AssumptionSet::default(), Vec::new());
        let _ = Evidence::<ProofClaim>::r#doorway_proved(scope.clone(), AssumptionSet::default(), Vec::new());
        let _ = ClaimProduct::proof_certified(input.clone(), scope.clone(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let _ = ClaimProduct::proof_proved(input.clone(), scope.clone(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
    "#;
    let root = create_clean_synthetic_tree("doorway-build");
    let repo = repo_root();
    let manifest =
        fs::read_to_string(repo.join("Cargo.toml")).expect("read live workspace lint table");
    let (_, lint_table) = manifest
        .split_once("[workspace.lints.rust]\n")
        .expect("workspace rust lint table");
    let rust_lints: Vec<_> = lint_table
        .lines()
        .take_while(|line| !line.starts_with('['))
        .collect();
    let scratch_manifest = root.join("Cargo.toml");
    write_file(
        &scratch_manifest,
        &format!(
            "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n\n[workspace.lints.rust]\n{}\n",
            rust_lints.join("\n")
        ),
    );

    let write_issuer = |name: &str, module: &str, allowance: &str, calls: &str| {
        let package = root.join("crates").join(name);
        write_file(
            &package.join("Cargo.toml"),
            &format!(
                "[package]\nname = {name:?}\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[lints]\nworkspace = true\n\n[dependencies]\nfrob-core = {{ path = {:?} }}\n",
                repo.join("crates/frob-core").to_string_lossy()
            ),
        );
        if module != "lib" {
            write_file(&package.join("src/lib.rs"), &format!("pub mod {module};\n"));
        }
        write_file(
            &package.join(format!("src/{module}.rs")),
            &format!(
                "use frob_core::evidence::*;\n{allowance}\npub fn issue(scope: Scope, input: InputRef) {{ {calls} }}\n"
            ),
        );
    };
    let allow = "#[allow(deprecated)]";
    write_issuer("frob-conformance", "harness", allow, TESTED);
    write_issuer("frob-arena", "runner", allow, TESTED);
    write_issuer("frob-lemma", "issue", allow, PROOF);
    assert!(
        run_check(&root).is_empty(),
        "listed issuer locations must pass the scan"
    );

    let check = |packages: &[&str]| {
        let mut cargo = Command::new("cargo");
        cargo
            .current_dir(&repo)
            .args(["check", "--offline", "-j", "1", "--manifest-path"])
            .arg(&scratch_manifest)
            .arg("--target-dir")
            .arg(root.join("target"));
        for package in packages {
            cargo.args(["-p", package]);
        }
        cargo
            .output()
            .expect("compile with inherited workspace lints")
    };
    let allowed = check(&["frob-conformance", "frob-arena", "frob-lemma"]);
    assert!(
        allowed.status.success(),
        "{}",
        String::from_utf8_lossy(&allowed.stderr)
    );

    write_issuer("core-alpha", "lib", "", &format!("{TESTED}{PROOF}"));
    let refused = check(&["core-alpha"]);
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(!refused.status.success(), "non-doorway issuers compiled");
    assert_eq!(
        stderr
            .matches("error: use of deprecated associated function")
            .count(),
        6,
        "all six constructor calls must fail under the inherited lint:\n{stderr}"
    );

    let wrong_class = "let _ = Evidence::<ProofClaim>::r#doorway_proved(scope, AssumptionSet::default(), Vec::new());";
    write_issuer("frob-conformance", "harness", allow, wrong_class);
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].rule, RULE_DOORWAY_UNLISTED_ALLOW);
    assert_eq!(findings[0].path, "crates/frob-conformance/src/harness.rs");
}

#[test]
fn doorway_second_allow_fails() {
    let root = create_clean_synthetic_tree("doorway-second-allow");
    // Legitimate doorway site in `frob-lemma/src/issue.rs`...
    write_file(
        &root.join("crates/frob-lemma/Cargo.toml"),
        "[package]\nname = \"frob-lemma\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[lints]\nworkspace = true\n",
    );
    write_file(
        &root.join("crates/frob-lemma/src/lib.rs"),
        "pub mod issue;\npub mod cheat;\n",
    );
    write_file(
        &root.join("crates/frob-lemma/src/issue.rs"),
        "#[allow(deprecated)]\npub fn proof_evidence() {}\n",
    );
    // ...plus a second unlisted `#[allow(deprecated)]` in `frob-lemma/src/cheat.rs`
    // and another in `core-alpha/src/lib.rs`.
    write_file(
        &root.join("crates/frob-lemma/src/cheat.rs"),
        "#[allow(deprecated)]\npub fn forge_proof() {}\n",
    );
    write_file(
        &root.join("crates/core-alpha/src/lib.rs"),
        "#[allow(dead_code, deprecated)]\npub fn also_forge() {}\n",
    );

    let findings = run_check(&root);
    assert_eq!(findings.len(), 2, "expected 2 unlisted doorway findings, got: {findings:?}");
    assert!(findings.iter().all(|f| f.rule == RULE_DOORWAY_UNLISTED_ALLOW));
    let paths: Vec<&str> = findings.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"crates/frob-lemma/src/cheat.rs"));
    assert!(paths.contains(&"crates/core-alpha/src/lib.rs"));
    assert!(!paths.contains(&"crates/frob-lemma/src/issue.rs"));
}

// ============================================================================
// Report facts, sorting, D0 determinism, and clean-repo verification
// ============================================================================

#[test]
fn report_lists_every_capsule_sorted() {
    let root = create_clean_synthetic_tree("report-sorted-capsules");
    // Add three capsules across two annexes in deliberately unsorted order in capsules.toml.
    let cap_a = "pub fn a() -> u32 { unsafe { 1 } }\n";
    let cap_b = "pub fn b() -> u32 {\n    let x = unsafe { 1 };\n    let y = unsafe { 2 };\n    x + y\n}\n";
    let cap_c = "pub fn c() -> u32 {\n    unsafe { 3 }\n}\n";

    write_file(&root.join("spark/frob-spark-metal/src/zeta.rs"), cap_c);
    write_file(&root.join("spark/frob-spark-metal/src/zeta.SAFETY.md"), full_safety_doc());
    write_file(&root.join("spark/frob-spark-cuda/src/beta.rs"), cap_b);
    write_file(&root.join("spark/frob-spark-cuda/src/beta.SAFETY.md"), full_safety_doc());
    write_file(&root.join("spark/frob-spark-cuda/src/alpha.rs"), cap_a);
    write_file(&root.join("spark/frob-spark-cuda/src/alpha.SAFETY.md"), full_safety_doc());

    write_file(
        &root.join("registries/capsules.toml"),
        "schema = \"frob.capsules/1\"\n\
         \n\
         [[capsule]]\n\
         id = \"metal.zeta\"\n\
         workspace = \"spark-metal\"\n\
         path = \"spark/frob-spark-metal/src/zeta.rs\"\n\
         safe_api = [\"c\"]\n\
         lines = 3\n\
         unsafe_blocks = 1\n\
         safety_doc = \"spark/frob-spark-metal/src/zeta.SAFETY.md\"\n\
         reason = \"metal zeta\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.beta\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/beta.rs\"\n\
         safe_api = [\"b\"]\n\
         lines = 5\n\
         unsafe_blocks = 2\n\
         safety_doc = \"spark/frob-spark-cuda/src/beta.SAFETY.md\"\n\
         reason = \"cuda beta\"\n\
         \n\
         [[capsule]]\n\
         id = \"cuda.alpha\"\n\
         workspace = \"spark-cuda\"\n\
         path = \"spark/frob-spark-cuda/src/alpha.rs\"\n\
         safe_api = [\"a\"]\n\
         lines = 1\n\
         unsafe_blocks = 1\n\
         safety_doc = \"spark/frob-spark-cuda/src/alpha.SAFETY.md\"\n\
         reason = \"cuda alpha\"\n",
    );

    let (code1, json1) = run_cli_json(&root);
    let (code2, json2) = run_cli_json(&root);
    assert_eq!(code1, ExitCode::from(EXIT_OK));
    assert_eq!(code2, ExitCode::from(EXIT_OK));

    let adm1 = admit(json1.as_bytes(), &Limits::default()).expect("admit json1");
    let adm2 = admit(json2.as_bytes(), &Limits::default()).expect("admit json2");
    assert_eq!(
        adm1.value().get("facts"),
        adm2.value().get("facts"),
        "facts must be D0 identical across runs"
    );

    let facts = adm1.value().get("facts").expect("facts object");
    let capsules = facts
        .get("capsules")
        .and_then(|v| v.as_array())
        .expect("capsules array");
    let ids: Vec<&str> = capsules
        .iter()
        .map(|c| c.get("id").and_then(|v| v.as_str()).expect("capsule id"))
        .collect();
    assert_eq!(ids, vec!["cuda.alpha", "cuda.beta", "metal.zeta"]);

    // Verify per-annex totals.
    let totals = facts.get("totals_per_annex").expect("totals_per_annex");
    let cuda = totals.get("spark-cuda").expect("spark-cuda totals");
    assert_eq!(cuda.get("capsules").and_then(|v| v.as_int53().ok()), Some(2));
    assert_eq!(cuda.get("lines").and_then(|v| v.as_int53().ok()), Some(6));
    assert_eq!(cuda.get("unsafe_blocks").and_then(|v| v.as_int53().ok()), Some(3));
    let metal = totals.get("spark-metal").expect("spark-metal totals");
    assert_eq!(metal.get("capsules").and_then(|v| v.as_int53().ok()), Some(1));
    assert_eq!(metal.get("lines").and_then(|v| v.as_int53().ok()), Some(3));
    assert_eq!(metal.get("unsafe_blocks").and_then(|v| v.as_int53().ok()), Some(1));

    // Also verify human table helper renders sorted capsule rows + totals.
    let mut tmap = BTreeMap::new();
    tmap.insert(
        "spark-cuda".to_owned(),
        AnnexTotals {
            capsules: 2,
            lines: 6,
            unsafe_blocks: 3,
        },
    );
    let table = format_capsule_table(
        &[CapsuleFact {
            id: "cuda.alpha".to_owned(),
            workspace: "spark-cuda".to_owned(),
            path: "spark/frob-spark-cuda/src/alpha.rs".to_owned(),
            lines: 1,
            unsafe_blocks: 1,
            safe_api: vec!["a".to_owned()],
        }],
        &tmap,
    );
    assert!(table.contains("cuda.alpha"));
    assert!(table.contains("spark-cuda"));
}

#[test]
fn clean_repo_tree_passes_and_is_fast() {
    let root = repo_root();
    assert_eq!(OWNED_RULES.len(), 11);

    let started = Instant::now();
    let (code, json_text) = run_cli_json(&root);
    let elapsed = started.elapsed();

    assert_eq!(
        code,
        ExitCode::from(EXIT_OK),
        "clean repo tree must pass check-unsafe, report: {json_text}"
    );
    assert!(
        elapsed.as_secs() < 5,
        "check-unsafe must finish in under 5 s on warm tree, took {elapsed:?}"
    );

    // Also verify non-JSON CLI run exits 0 and prints the per-capsule table.
    let args = vec![
        "check-unsafe".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
    ];
    let mut out = Vec::new();
    let mut err = Vec::new();
    let human_code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(human_code, ExitCode::from(EXIT_OK));
    assert!(err.is_empty());
    let rendered = String::from_utf8(out).expect("human stdout must be UTF-8");
    assert!(rendered.contains("check-unsafe: OK"));
    assert!(
        rendered.contains("capsules ("),
        "human check-unsafe output must print per-capsule table, got:\n{rendered}"
    );

    // Verify D0 canonical report equality when elapsed_ms is normalized.
    let o1 = check_unsafe(&root, &[WorkspaceRoot::core(&root)]).expect("run 1");
    let o2 = check_unsafe(&root, &[WorkspaceRoot::core(&root)]).expect("run 2");
    let r1 = CheckReport::new("check-unsafe", ".", o1.findings, o1.facts, 0)
        .to_canonical_json()
        .expect("canon 1");
    let r2 = CheckReport::new("check-unsafe", ".", o2.findings, o2.facts, 0)
        .to_canonical_json()
        .expect("canon 2");
    assert_eq!(r1, r2, "check-unsafe report must be D0 deterministic");

    // Verify status_table validates all registries including updated capsules.schema.json.
    status_table::validate_all_registries(&root)
        .expect("status_table::validate_all_registries must accept updated capsules schema");
}
