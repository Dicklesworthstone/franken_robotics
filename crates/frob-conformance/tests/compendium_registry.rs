#![deny(unsafe_code)]

//! Compendium registry validation and document drift tests (`N12-41`).
//!
//! Shared across all ten compendium sections (`D.1` through `D.10`).
//! Never deletes files or directories (COMMON.md 9b).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::compendium::registry::{self, render_compendium_doc, validate_workspace};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_workspace(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let ws = base
        .join("frob-CopperDove")
        .join(format!("compendium_registry-{label}-{nanos}-{seq}"));
    fs::create_dir_all(ws.join("registries/compendium")).expect("mkdir registries/compendium");
    fs::create_dir_all(ws.join("crates/frob-conformance/src/compendium"))
        .expect("mkdir src/compendium");
    fs::create_dir_all(ws.join("crates/frob-conformance/tests")).expect("mkdir tests");
    ws
}

fn copy_d1_to_scratch(src_root: &Path, dst_root: &Path) {
    for rel in [
        "registries/compendium/d1.toml",
        "crates/frob-conformance/src/compendium/d1.rs",
        "crates/frob-conformance/tests/compendium_d1.rs",
    ] {
        let content = fs::read(src_root.join(rel))
            .unwrap_or_else(|e| panic!("read {rel}: {e}"));
        fs::write(dst_root.join(rel), content)
            .unwrap_or_else(|e| panic!("write {rel}: {e}"));
    }
}

#[test]
fn rows_resolve_oracle_test_and_control() {
    let root = registry::workspace_root().expect("locate workspace root");
    let sections = registry::load_present_sections(&root).expect("load compendium sections");
    assert!(
        sections.iter().any(|(spec, sec)| spec.id == "D.1" && sec.rows.len() == 21),
        "expected section D.1 with 21 rows"
    );

    let findings = validate_workspace(&root).expect("validate compendium workspace");
    assert!(
        findings.is_empty(),
        "compendium registry validation failed:\n{}",
        findings
            .iter()
            .map(|f| format!("[{}] {} ({}): {}", f.rule, f.section, f.row, f.detail))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Acceptance criterion: no production crate is a dependency of the oracle module
    // (`src/compendium/` only imports `std`, `core`, `alloc`, `crate`, `self`, `super`, `frob_core`,
    // and never references any production crate path even if `frob-conformance` later depends on
    // production crates to test them in Gauntlet tiers).
    let comp_dir = root.join("crates/frob-conformance/src/compendium");
    let mut comp_files: Vec<PathBuf> = fs::read_dir(&comp_dir)
        .expect("read src/compendium")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
        .collect();
    comp_files.sort();
    assert!(!comp_files.is_empty(), "expected compendium source files");
    let forbidden_prefixes = [
        "frob_lie::",
        "frob_algebra::",
        "frob_model::",
        "frob_dyn::",
        "frob_contact::",
        "frob_sim::",
        "frob_analyze::",
        "frob_ctrl::",
        "frob_kir::",
        "frob_kernels::",
        "frob_scene::",
    ];
    for file in comp_files {
        let src = fs::read_to_string(&file).expect("read compendium source");
        for (idx, raw) in src.lines().enumerate() {
            let line = raw.trim();
            if line.starts_with("//") {
                continue;
            }
            if let Some(rest) = line.strip_prefix("use ") {
                let root_crate = rest.split([':', ';', ' ']).next().unwrap_or("");
                assert!(
                    matches!(
                        root_crate,
                        "std" | "core" | "alloc" | "crate" | "self" | "super" | "frob_core"
                    ),
                    "{}:{}: oracle module must not import production crate `{root_crate}`",
                    file.display(),
                    idx + 1
                );
            }
            for pat in forbidden_prefixes {
                assert!(
                    !line.contains(pat),
                    "{}:{}: oracle module must not reference production crate `{pat}`",
                    file.display(),
                    idx + 1
                );
            }
        }
    }

    let total_rows: usize = sections.iter().map(|(_, s)| s.rows.len()).sum();
    println!(
        "compendium_registry: sections={} rows={}",
        sections.len(),
        total_rows
    );
}

#[test]
fn row_without_listed_control_fails() {
    let root = registry::workspace_root().expect("locate workspace root");
    let ws = scratch_workspace("missing_control");
    copy_d1_to_scratch(&root, &ws);

    let d1_path = ws.join("registries/compendium/d1.toml");
    let raw = fs::read_to_string(&d1_path).expect("read scratch d1.toml");
    let mutated = raw.replacen(
        "negative_control = \"d1_nc_bracket\"",
        "negative_control = \"\"",
        1,
    );
    assert_ne!(raw, mutated, "plant must mutate d1.toml");
    fs::write(&d1_path, mutated).expect("write mutated d1.toml");

    let findings = validate_workspace(&ws).expect("validate mutated workspace");
    assert!(
        findings
            .iter()
            .any(|f| f.rule == "compendium.missing-negative-control"),
        "expected compendium.missing-negative-control finding, got: {findings:?}"
    );
}

#[test]
fn unresolved_oracle_path_fails() {
    let root = registry::workspace_root().expect("locate workspace root");
    let ws = scratch_workspace("unresolved_oracle");
    copy_d1_to_scratch(&root, &ws);

    let d1_path = ws.join("registries/compendium/d1.toml");
    let raw = fs::read_to_string(&d1_path).expect("read scratch d1.toml");
    let mutated = raw.replacen(
        "oracle = \"frob_conformance::compendium::d1::bracket\"",
        "oracle = \"frob_conformance::compendium::d1::nonexistent_oracle_fn\"",
        1,
    );
    assert_ne!(raw, mutated, "plant must mutate oracle path");
    fs::write(&d1_path, mutated).expect("write mutated d1.toml");

    let findings = validate_workspace(&ws).expect("validate mutated workspace");
    assert!(
        findings
            .iter()
            .any(|f| f.rule == "compendium.oracle-unresolved"),
        "expected compendium.oracle-unresolved finding, got: {findings:?}"
    );
}

#[test]
fn doc_matches_registry() {
    let root = registry::workspace_root().expect("locate workspace root");
    let expected = render_compendium_doc(&root).expect("render compendium doc");
    let doc_path = root.join("docs/math/compendium.md");
    let committed = fs::read_to_string(&doc_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", doc_path.display()));
    assert_eq!(
        committed, expected,
        "docs/math/compendium.md drifted from registries/compendium/d*.toml; regenerate with `cargo run -p frob-conformance --example compendium_doc -- --out docs/math/compendium.md`"
    );

    // Acceptance criterion: compendium-only and Estimated rows are explicitly listed.
    for name in ["Reflection parity", "Planar Cayley", "Shell rule"] {
        assert!(
            committed.contains(name),
            "expected compendium-only row `{name}` in docs/math/compendium.md"
        );
    }
    assert!(
        committed.contains(
            "Rows marked `compendium-only` in D.1: Reflection parity, Planar Cayley, Shell rule."
        ),
        "expected explicit compendium-only summary line for D.1"
    );
    assert!(
        committed.contains(
            "Rows marked `compendium-only` in D.3: Squeeze film, Pressure–sinkage and shear."
        ),
        "expected explicit compendium-only summary line for D.3"
    );
    assert!(
        committed.contains(
            "Rows marked `Estimated` in D.3: Squeeze film, Pressure–sinkage and shear."
        ),
        "expected explicit Estimated summary line for D.3"
    );
}
