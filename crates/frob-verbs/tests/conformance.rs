//! Crate-level conformance checks for `frob-verbs` (N32a-02, N34-08 rule 3, ADR-TETHER-02).

use std::process::Command;

use frob_core::registry::refusals::RefusalRegistry;
use frob_core::registry::toml::{self, Fields};
use frob_verbs::registry::{REGISTRY_DIGEST, REGISTRY_DIGEST_HEX, TASK_CLUSTERS, VERBS};

const CONTRACT_MD: &str = include_str!("../CONTRACT.md");
const CONTRACT_TOML: &str = include_str!("../contract.toml");
const REQUIRED_SECTIONS: &[&str] = &[
    "Purpose",
    "Inputs",
    "Outputs",
    "Invariants",
    "Refusals",
    "Determinism class",
    "Evidence class of outputs",
    "Validity domain",
    "Oracles",
    "Known gaps",
];

#[test]
fn contract_md_has_all_ten_non_empty_sections_and_specimen_example() {
    let mut sections: Vec<(&str, String)> = Vec::new();
    let mut current: Option<(&str, String)> = None;
    for line in CONTRACT_MD.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(prev) = current.take() {
                sections.push(prev);
            }
            current = Some((heading.trim(), String::new()));
        } else if let Some((_, ref mut body)) = current {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(prev) = current {
        sections.push(prev);
    }

    for &req in REQUIRED_SECTIONS {
        let found = sections
            .iter()
            .find(|(h, _)| *h == req)
            .unwrap_or_else(|| panic!("CONTRACT.md missing required heading `## {req}`"));
        assert!(
            !found.1.trim().is_empty(),
            "CONTRACT.md section `## {req}` must be non-empty"
        );
    }
    let oracles = &sections.iter().find(|(h, _)| *h == "Oracles").unwrap().1;
    assert!(
        oracles.contains("docs/testing/CONVENTIONS.md"),
        "CONTRACT.md Oracles section must cite docs/testing/CONVENTIONS.md"
    );
    assert!(
        CONTRACT_MD.contains("schema = \"frob.verbs/1\"")
            && CONTRACT_MD.contains("name = \"design.check\""),
        "CONTRACT.md must document the registry format with one valid example"
    );
}

#[test]
fn contract_toml_validates_against_strict_toml_and_refusal_registry() {
    let reg = RefusalRegistry::load().expect("embedded refusals.toml must load");
    let root = toml::parse(CONTRACT_TOML.as_bytes()).expect("contract.toml must parse");
    let mut top = Fields::new(root, "contract.toml");

    let mut c = Fields::new(top.require_table("crate").unwrap(), "crate");
    assert_eq!(c.require_str("name").unwrap(), "frob-verbs");
    assert_eq!(c.require_int("layer").unwrap(), 6);
    assert_eq!(c.require_int("rank").unwrap(), 4);
    assert_eq!(c.require_str("plane").unwrap(), "decision");
    assert_eq!(c.require_str("determinism").unwrap(), "D0");
    assert_eq!(c.require_str("evidence").unwrap(), "Tested");
    let gates = c.require_array("gates").unwrap();
    assert_eq!(gates.len(), 1);
    assert_eq!(gates[0].value.as_str(), Some("GATE-TETHER-02"));
    for sp in c.require_array("refusals").unwrap() {
        let code = sp.value.as_str().expect("refusal code must be string");
        reg.lookup(code, "frob-verbs/contract.toml")
            .unwrap_or_else(|e| panic!("contract.toml refusal `{code}` not in registry: {e}"));
    }
    c.finish().unwrap();

    let mut budgets = Fields::new(top.require_table("budgets").unwrap(), "budgets");
    assert!(budgets.require_int("wall_ms").unwrap() > 0);
    assert!(budgets.require_int("memory_mib").unwrap() > 0);
    budgets.finish().unwrap();

    let mut safe = Fields::new(top.require_table("safe_mode").unwrap(), "safe_mode");
    assert_eq!(safe.require_str("on_budget").unwrap(), "reference-path");
    assert_eq!(
        safe.require_str("on_calibration").unwrap(),
        "reference-path"
    );
    safe.finish().unwrap();

    let mut feats = Fields::new(top.require_table("features").unwrap(), "features");
    for name in [
        "registry-only",
        "handlers",
        "conformance",
        "nc-bare-unknown-verb",
        "nc-second-envelope",
    ] {
        let mut f = Fields::new(feats.require_table(name).unwrap(), name);
        assert_eq!(f.require_str("tier").unwrap(), "S");
        f.finish().unwrap();
    }
    feats.finish().unwrap();

    // `[api.float_ok]`: every envelope item that carries a binary64 states why (N32a-03).
    let mut api = Fields::new(top.require_table("api").unwrap(), "api");
    let mut float_ok = Fields::new(api.require_table("float_ok").unwrap(), "api.float_ok");
    for sig in [
        "envelope::Cost",
        "envelope::Budget::with",
        "envelope::Eta",
        "envelope::Tree::as_f64",
    ] {
        let reason = float_ok.require_str(sig).unwrap();
        assert!(!reason.trim().is_empty(), "`[api.float_ok].\"{sig}\"` needs a reason");
    }
    float_ok.finish().unwrap();
    api.finish().unwrap();
    top.finish().unwrap();

    // Verify generated static table symbols are live and consistent.
    assert_eq!(REGISTRY_DIGEST_HEX.len(), 64);
    assert_eq!(REGISTRY_DIGEST.len(), 32);
    let _ = (VERBS.len(), TASK_CLUSTERS.len());
}

#[test]
fn registry_only_has_no_physics() {
    // ADR-TETHER-02: `--no-default-features --features registry-only` must carry no
    // `frob-fsim`, no FrankenSim physics crate, and no subsystem crate.
    // Note: `frob-core` unconditionally depends on layer-0 non-physics utilities
    // `fs-blake3` and `fs-qty`; those two are explicitly distinguished from physics crates.
    let output = Command::new("cargo")
        .args([
            "tree",
            "-p",
            "frob-verbs",
            "--no-default-features",
            "--features",
            "registry-only",
            "-e",
            "normal",
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo tree must execute via PATH");
    assert!(
        output.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages: Vec<String> = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
        if let Some(pkg) = trimmed.split_whitespace().next() {
            packages.push(pkg.to_owned());
        }
    }
    assert!(
        packages.iter().any(|p| p == "frob-verbs"),
        "expected frob-verbs in cargo tree output:\n{stdout}"
    );
    for pkg in &packages {
        assert_ne!(
            pkg, "frob-fsim",
            "registry-only must never link frob-fsim:\n{stdout}"
        );
        if pkg.starts_with("frob-") {
            assert!(
                matches!(pkg.as_str(), "frob-verbs" | "frob-core"),
                "registry-only linked unexpected subsystem crate `{pkg}`:\n{stdout}"
            );
        }
        if pkg.starts_with("fs-") {
            assert!(
                matches!(pkg.as_str(), "fs-blake3" | "fs-qty"),
                "registry-only linked unexpected FrankenSim physics crate `{pkg}`:\n{stdout}"
            );
        }
    }
}
