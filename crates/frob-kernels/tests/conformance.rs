#![deny(unsafe_code)]

//! Crate-level conformance checks for `frob-kernels` (BD-013, N34-08).

use frob_core::registry::refusals::RefusalRegistry;
use frob_core::registry::toml::{self, Fields};

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
fn contract_md_documents_ten_sections_nine_rules_and_cx221_reread() {
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

    let invariants = &sections.iter().find(|(h, _)| *h == "Invariants").unwrap().1;
    // All nine dialect rules numbered 1..=9.
    for n in 1..=9 {
        assert!(
            invariants.contains(&format!("  {n}. ")),
            "CONTRACT.md Invariants must list dialect rule {n}"
        );
    }
    // CX-221 pinned FrankenSim commit, path, line range, and verbatim stall quote.
    assert!(invariants.contains("08e9195c6faea3363377b7fdfb6d6b07f5b552f3"));
    assert!(invariants.contains("apps/instrument-lab/render-gpu-piano/src/metal.rs"));
    assert!(invariants.contains("68-71"));
    assert!(
        invariants.contains(
            "Metal does not promise that all threadgroups of a dispatch run concurrently"
        )
    );
    assert!(invariants.contains("about once per few hundred launches here"));
}

#[test]
fn contract_toml_validates_against_strict_toml_and_refusal_registry() {
    let reg = RefusalRegistry::load().expect("embedded refusals.toml must load");
    let root = toml::parse(CONTRACT_TOML.as_bytes()).expect("contract.toml must parse");
    let mut top = Fields::new(root, "contract.toml");

    let mut c = Fields::new(top.require_table("crate").unwrap(), "crate");
    assert_eq!(c.require_str("name").unwrap(), "frob-kernels");
    assert_eq!(c.require_int("layer").unwrap(), 3);
    assert_eq!(c.require_int("rank").unwrap(), 1);
    assert_eq!(c.require_str("plane").unwrap(), "compute");
    assert_eq!(c.require_str("determinism").unwrap(), "D0");
    assert_eq!(c.require_str("evidence").unwrap(), "Tested");
    let _gates = c.require_array("gates").unwrap();
    for sp in c.require_array("refusals").unwrap() {
        let code = sp.value.as_str().expect("refusal code must be string");
        reg.lookup(code, "frob-kernels/contract.toml")
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
    top.finish().unwrap();
}
