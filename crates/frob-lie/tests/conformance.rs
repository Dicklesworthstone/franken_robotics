#![forbid(unsafe_code)]

//! Contract and structural conformance checks for `frob-lie` (`BD-010`, `N34-08`, `N38b-04`).

use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Value};

#[test]
fn contract_files_exist_and_match_layer_map() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let contract_md = fs::read_to_string(manifest_dir.join("CONTRACT.md"))
        .expect("crates/frob-lie/CONTRACT.md must exist");
    for heading in [
        "## Purpose",
        "## Inputs",
        "## Outputs",
        "## Invariants",
        "## Refusals",
        "## Determinism class",
        "## Evidence class of outputs",
        "## Validity domain",
        "## Oracles",
        "## Known gaps",
    ] {
        assert!(
            contract_md.contains(heading),
            "CONTRACT.md missing required section `{heading}`"
        );
    }

    let contract_toml_bytes = fs::read(manifest_dir.join("contract.toml"))
        .expect("crates/frob-lie/contract.toml must exist");
    let root = toml::parse(&contract_toml_bytes).expect("contract.toml must parse strictly");
    let crate_tbl = root
        .get("crate")
        .and_then(Value::as_table)
        .expect("[crate] table must exist");
    assert_eq!(crate_tbl.get("name").and_then(Value::as_str), Some("frob-lie"));
    assert_eq!(crate_tbl.get("layer").and_then(Value::as_int), Some(1));
    assert_eq!(crate_tbl.get("rank").and_then(Value::as_int), Some(1));
    assert_eq!(crate_tbl.get("plane").and_then(Value::as_str), Some("compute"));
    assert_eq!(crate_tbl.get("determinism").and_then(Value::as_str), Some("D2"));
    assert_eq!(crate_tbl.get("evidence").and_then(Value::as_str), Some("Tested"));

    let features_tbl = root
        .get("features")
        .and_then(Value::as_table)
        .expect("[features] table must exist");
    for feat in [
        "conformance",
        "twist-swap",
        "ad-inverse",
        "bracket-swap",
        "coadjoint-inverse",
    ] {
        let f_tbl = features_tbl
            .get(feat)
            .and_then(Value::as_table)
            .unwrap_or_else(|| panic!("contract.toml missing feature `{feat}`"));
        assert_eq!(
            f_tbl.get("tier").and_then(Value::as_str),
            Some("S"),
            "feature `{feat}` must have tier S"
        );
    }

    // Verify N38b-04 release-profile mutation guard exists in src/lib.rs.
    let lib_rs = fs::read_to_string(manifest_dir.join("src/lib.rs"))
        .expect("crates/frob-lie/src/lib.rs must exist");
    assert!(
        lib_rs.contains("not(debug_assertions)") && lib_rs.contains("compile_error!"),
        "src/lib.rs must carry the N38b-04 release-profile compile_error! guard for `conformance`"
    );
}

#[test]
fn capture_tool_depends_on_fs_ga_and_not_frob_lie() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let capture_manifest = repo_root.join("tools/oracle/marrow_fsga_capture/Cargo.toml");
    let bytes = fs::read(&capture_manifest)
        .unwrap_or_else(|e| panic!("failed reading {}: {e}", capture_manifest.display()));
    let root = toml::parse(&bytes).expect("capture tool Cargo.toml must parse strictly");
    let deps = root
        .get("dependencies")
        .and_then(Value::as_table)
        .expect("[dependencies] table must exist in marrow_fsga_capture/Cargo.toml");
    assert!(
        deps.contains_key("fs-ga"),
        "capture tool must depend directly on fs-ga"
    );
    assert!(
        !deps.contains_key("frob-lie"),
        "capture tool must NEVER depend on frob-lie (INV-GAUNT-01)"
    );
}

#[test]
fn no_private_vector_types_in_src() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut rs_files = Vec::new();
    collect_rs_files(&src_dir, &mut rs_files);
    assert!(!rs_files.is_empty(), "frob-lie/src must contain Rust files");

    for path in rs_files {
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed reading {}: {e}", path.display()));
        for (line_idx, raw_line) in text.lines().enumerate() {
            let trimmed = raw_line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            assert!(
                !trimmed.starts_with("pub struct ") && !trimmed.starts_with("struct "),
                "ADR-MARROW-01 violation: struct declaration found in {}:{}: `{trimmed}`",
                path.display(),
                line_idx + 1
            );
        }
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed reading dir {}: {e}", dir.display()))
        .map(|res| res.expect("dir entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}
