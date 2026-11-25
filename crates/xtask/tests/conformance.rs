//! Crate-level conformance checks for `xtask` (N34-08 rule 3).

use frob_core::registry::toml::{self, Fields};
use xtask::crate_map::{Layer, lookup};
use xtask::status_table;

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
fn contract_md_has_all_ten_non_empty_sections_and_oracles_cite_conventions() {
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
            .find(|(h, _)| h.eq_ignore_ascii_case(req))
            .unwrap_or_else(|| panic!("xtask CONTRACT.md missing heading `## {req}`"));
        assert!(
            !found.1.trim().is_empty(),
            "xtask CONTRACT.md section `## {req}` must be non-empty"
        );
    }

    let oracles = &sections
        .iter()
        .find(|(h, _)| h.eq_ignore_ascii_case("Oracles"))
        .expect("Oracles section present")
        .1;
    assert!(
        oracles.contains("docs/testing/CONVENTIONS.md"),
        "xtask CONTRACT.md Oracles section must cite docs/testing/CONVENTIONS.md"
    );
}

#[test]
fn contract_toml_matches_schema_and_crate_map() {
    status_table::validate_contract_toml(CONTRACT_TOML.as_bytes())
        .expect("xtask contract.toml must satisfy schemas/contract.schema.json");

    let row = lookup("xtask").expect("xtask must exist in CRATE_MAP");
    let root = toml::parse(CONTRACT_TOML.as_bytes()).expect("parse xtask contract.toml");
    let mut top = Fields::new(root, "contract.toml");
    let mut c = Fields::new(top.require_table("crate").expect("crate table"), "crate");

    assert_eq!(c.require_str("name").expect("name"), "xtask");
    assert_eq!(
        Layer::Core(u8::try_from(c.require_int("layer").expect("layer")).expect("u8 layer")),
        row.layer
    );
    assert_eq!(
        u8::try_from(c.require_int("rank").expect("rank")).expect("u8 rank"),
        row.rank
    );
    assert_eq!(c.require_str("plane").expect("plane"), "decision");
    assert_eq!(c.require_str("determinism").expect("determinism"), "D0");
    assert_eq!(c.require_str("evidence").expect("evidence"), "Estimated");
    assert!(c.require_array("gates").expect("gates").is_empty());
    assert!(c.require_array("refusals").expect("refusals").is_empty());
    c.finish().expect("no extra crate keys");

    let mut budgets = Fields::new(top.require_table("budgets").expect("budgets"), "budgets");
    assert!(budgets.require_int("wall_ms").expect("wall_ms") > 0);
    assert!(budgets.require_int("memory_mib").expect("memory_mib") > 0);
    budgets.finish().expect("no extra budget keys");

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
    safe.finish().expect("no extra safe_mode keys");
    top.finish().expect("no extra top-level keys");
}
