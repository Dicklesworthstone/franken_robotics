#![deny(unsafe_code)]

//! Crate-level conformance checks for `frob-kir` (BD-013, N34-08).

use frob_core::refusal::{Refusal, RefusalCode, RetryPolicy};
use frob_core::registry::refusals::RefusalRegistry;
use frob_core::registry::toml::{self, Fields};
use frob_kir::dialect_lint::{
    CompileOptions, DialectUnit, MetalMathMode, SourceFile, collect_findings, lint,
};

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
}

#[test]
fn contract_toml_validates_against_strict_toml_and_refusal_registry() {
    let reg = RefusalRegistry::load().expect("embedded refusals.toml must load");
    let root = toml::parse(CONTRACT_TOML.as_bytes()).expect("contract.toml must parse");
    let mut top = Fields::new(root, "contract.toml");

    let mut c = Fields::new(top.require_table("crate").unwrap(), "crate");
    assert_eq!(c.require_str("name").unwrap(), "frob-kir");
    assert_eq!(c.require_int("layer").unwrap(), 1);
    assert_eq!(c.require_int("rank").unwrap(), 3);
    assert_eq!(c.require_str("plane").unwrap(), "compute");
    assert_eq!(c.require_str("determinism").unwrap(), "D0");
    assert_eq!(c.require_str("evidence").unwrap(), "Tested");
    let _gates = c.require_array("gates").unwrap();
    for sp in c.require_array("refusals").unwrap() {
        let code = sp.value.as_str().expect("refusal code must be string");
        reg.lookup(code, "frob-kir/contract.toml")
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
    let mut features = Fields::new(top.require_table("features").unwrap(), "features");
    for name in ["conformance", "skip-sum-dim-check"] {
        let mut feature = Fields::new(features.require_table(name).unwrap(), name);
        assert_eq!(feature.require_str("tier").unwrap(), "S");
        assert!(feature.require_bool("dev_only").unwrap());
        feature.finish().unwrap();
    }
    features.finish().unwrap();
    top.finish().unwrap();

    assert_eq!(reg.exit_for(RefusalCode::FrobRef251).unwrap(), 3);
    assert_eq!(
        reg.row(RefusalCode::FrobRef251).unwrap().retry,
        RetryPolicy::AfterChange
    );
}

#[test]
fn det_twice_dialect_lint_report_and_refusal_json_identical() {
    let bad_unit = DialectUnit::default().with_source(SourceFile::new(
        "bad.h",
        "f32 a = sinf(x);\natomicAdd(&acc, a);\nf32 b = fmaf(a, x, y);\nwhile (n > 0u) {}\n",
    ));
    let opts = CompileOptions::Metal {
        math_mode: MetalMathMode::Fast,
    };

    let r1 = lint(&bad_unit, &opts).unwrap_err();
    let r2 = lint(&bad_unit, &opts).unwrap_err();
    assert_eq!(r1, r2);
    assert_eq!(r1.code, RefusalCode::FrobRef251);
    let j1 = r1.to_json();
    let j2 = r2.to_json();
    assert_eq!(j1, j2);
    assert_eq!(Refusal::from_json(&j1).unwrap(), r1);

    let findings = collect_findings(&bad_unit, &opts);
    for expected_rule in [1u8, 2, 3, 4, 6] {
        assert!(
            findings.iter().any(|f| f.rule == expected_rule),
            "expected rule {expected_rule} in {findings:?}"
        );
    }
}
