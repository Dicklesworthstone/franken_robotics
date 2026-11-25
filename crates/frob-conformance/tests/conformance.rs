//! Crate-level conformance checks for `frob-conformance` (BD-025, N34-08 rule 3).

use frob_core::registry::toml::{self, Fields};

const CONTRACT_MD: &str = include_str!("../CONTRACT.md");
const CONTRACT_TOML: &str = include_str!("../contract.toml");
const CARGO_TOML: &str = include_str!("../Cargo.toml");

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

fn normalize_section_heading(raw: &str) -> String {
    let trimmed = raw.trim();
    let stripped = match trimmed.find(". ") {
        Some(idx) if idx > 0 && trimmed[..idx].bytes().all(|b| b.is_ascii_digit()) => {
            trimmed[idx + 2..].trim()
        }
        _ => trimmed,
    };
    stripped
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

#[test]
fn contract_md_has_all_ten_non_empty_sections_and_oracles_cite_conventions() {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current: Option<(String, String)> = None;
    for line in CONTRACT_MD.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(prev) = current.take() {
                sections.push(prev);
            }
            current = Some((normalize_section_heading(heading), String::new()));
        } else if let Some((_, ref mut body)) = current {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(prev) = current {
        sections.push(prev);
    }

    for &req in REQUIRED_SECTIONS {
        let key = req.to_ascii_lowercase();
        let found = sections
            .iter()
            .find(|(h, _)| h == &key)
            .unwrap_or_else(|| panic!("frob-conformance CONTRACT.md missing section `{req}`"));
        assert!(
            !found.1.trim().is_empty(),
            "frob-conformance CONTRACT.md section `{req}` must be non-empty"
        );
    }

    let oracles = &sections
        .iter()
        .find(|(h, _)| h == "oracles")
        .expect("Oracles section present")
        .1;
    assert!(
        oracles.contains("docs/testing/CONVENTIONS.md"),
        "frob-conformance CONTRACT.md Oracles must cite docs/testing/CONVENTIONS.md"
    );
}

#[test]
fn contract_toml_parses_strictly_and_matches_manifest_features() {
    let root = toml::parse(CONTRACT_TOML.as_bytes()).expect("contract.toml must parse");
    let mut top = Fields::new(root, "contract.toml");

    let mut c = Fields::new(top.require_table("crate").expect("crate"), "crate");
    assert_eq!(c.require_str("name").expect("name"), "frob-conformance");
    assert_eq!(c.require_int("layer").expect("layer"), 7);
    assert_eq!(c.require_int("rank").expect("rank"), 4);
    assert_eq!(c.require_str("plane").expect("plane"), "decision");
    assert_eq!(c.require_str("determinism").expect("determinism"), "D0");
    assert_eq!(c.require_str("evidence").expect("evidence"), "Tested");
    let _gates = c.require_array("gates").expect("gates");
    let _refs = c.require_array("refusals").expect("refusals");
    c.finish().expect("no unknown crate keys");

    let mut budgets = Fields::new(top.require_table("budgets").expect("budgets"), "budgets");
    assert!(budgets.require_int("wall_ms").expect("wall_ms") > 0);
    assert!(budgets.require_int("memory_mib").expect("memory_mib") > 0);
    budgets.finish().expect("no unknown budget keys");

    let mut safe = Fields::new(
        top.require_table("safe_mode").expect("safe_mode"),
        "safe_mode",
    );
    assert_eq!(
        safe.require_str("on_budget").expect("on_budget"),
        "reference-path"
    );
    assert_eq!(
        safe.require_str("on_calibration").expect("on_calibration"),
        "reference-path"
    );
    safe.finish().expect("no unknown safe_mode keys");

    let feats_tbl = top.require_table("features").expect("features table");
    let mut in_features = false;
    for line in CARGO_TOML.lines() {
        let s = line.trim();
        if s.starts_with('[') {
            in_features = s == "[features]";
            continue;
        }
        if !in_features || s.is_empty() || s.starts_with('#') {
            continue;
        }
        if let Some((feat, _)) = s.split_once('=') {
            let name = feat.trim().trim_matches('"');
            if name != "default" {
                assert!(
                    feats_tbl.contains_key(name),
                    "frob-conformance Cargo.toml feature `{name}` missing from contract.toml"
                );
            }
        }
    }
    top.finish().expect("no unknown top-level keys");
}
