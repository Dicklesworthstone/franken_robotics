#![forbid(unsafe_code)]

//! Crate-level conformance checks for `frob-cert` (BD-026, N34-08 rule 3).
//!
//! Strictly `std`-only so `crates/frob-cert` preserves ADR-WS-02 zero-dependency
//! standalone compilation.

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
    // Accept optional leading numbering such as `1. Purpose` as well as `Purpose`.
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
            .unwrap_or_else(|| panic!("frob-cert CONTRACT.md missing section `{req}`"));
        assert!(
            !found.1.trim().is_empty(),
            "frob-cert CONTRACT.md section `{req}` must be non-empty"
        );
    }

    let oracles = &sections
        .iter()
        .find(|(h, _)| h == "oracles")
        .expect("Oracles section present")
        .1;
    assert!(
        oracles.contains("docs/testing/CONVENTIONS.md"),
        "frob-cert CONTRACT.md Oracles must cite docs/testing/CONVENTIONS.md"
    );
}

#[test]
fn contract_toml_declares_identity_and_all_manifest_features() {
    assert!(CONTRACT_TOML.contains("name = \"frob-cert\""));
    assert!(CONTRACT_TOML.contains("layer = 6"));
    assert!(CONTRACT_TOML.contains("rank = 1"));
    assert!(CONTRACT_TOML.contains("determinism = \"D0\""));
    assert!(CONTRACT_TOML.contains("evidence = \"Estimated\""));
    assert!(CONTRACT_TOML.contains("on_budget = \"reference-path\""));
    assert!(CONTRACT_TOML.contains("on_calibration = \"reference-path\""));

    // Every non-default feature in Cargo.toml must appear in contract.toml [features].
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
                    CONTRACT_TOML.contains(name),
                    "frob-cert Cargo.toml feature `{name}` missing from contract.toml"
                );
            }
        }
    }
}
