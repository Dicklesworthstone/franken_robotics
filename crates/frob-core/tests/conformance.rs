//! Crate-level conformance checks for `frob-core` (BD-005, N34-08 rule 3).

use frob_core::auth::NoAuthorization;
use frob_core::evidence::{
    ClaimKind, ClaimProduct, DetClass, EvidenceClass, FlagSet, InputRef, RealityScope, Scope,
    TierMeet,
};
use frob_core::id::{SchemaName, hash_blob};
use frob_core::qty::{ALIAS_COUNT, MAX_ALIASES};
use frob_core::registry::refusals::{RefusalRegistry, render_refusal_code_enum};
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
    assert_eq!(c.require_str("name").unwrap(), "frob-core");
    assert_eq!(c.require_int("layer").unwrap(), 0);
    assert_eq!(c.require_int("rank").unwrap(), 1);
    assert_eq!(c.require_str("plane").unwrap(), "compute");
    assert_eq!(c.require_str("determinism").unwrap(), "D0");
    assert_eq!(c.require_str("evidence").unwrap(), "Estimated");
    let _gates = c.require_array("gates").unwrap();
    for sp in c.require_array("refusals").unwrap() {
        let code = sp.value.as_str().expect("refusal code must be string");
        reg.lookup(code, "frob-core/contract.toml")
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
        "test-support",
        "gauntlet-mutations",
        "dup-first-wins",
        "dup-last-wins",
        "skip-escape-decode",
        "writer-no-key-sort",
        "identity-on-raw-bytes",
        "utf16-sort",
        "escape-all-non-ascii",
        "nc-torque-alias-first-moment",
        "nc-allow-empty-support",
    ] {
        let mut f = Fields::new(feats.require_table(name).unwrap(), name);
        assert_eq!(f.require_str("tier").unwrap(), "S");
        f.finish().unwrap();
    }
    feats.finish().unwrap();

    if let Some(api_tbl) = top.opt_table("api").unwrap() {
        let mut api = Fields::new(api_tbl, "api");
        if let Some(fok) = api.opt_table("float_ok").unwrap() {
            for (_sig, sp) in fok.iter() {
                assert!(!sp.value.as_str().unwrap().trim().is_empty());
            }
        }
        api.finish().unwrap();
    }
    top.finish().unwrap();

    // Cross-check generated RefusalCode, qty cap, flags, hashing, and ClaimProduct.
    assert_eq!(
        include_str!("../src/refusal/code_gen.rs"),
        render_refusal_code_enum(&reg)
    );
    assert!(ALIAS_COUNT <= MAX_ALIASES);
    assert_eq!(
        NoAuthorization::parse_json(&NoAuthorization::VALUE.to_json()).unwrap(),
        NoAuthorization::VALUE
    );
    assert_eq!(
        hash_blob(b"").to_hex(),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
    assert_eq!(
        SchemaName::new("design", 1).unwrap().to_string(),
        "frob.design/1"
    );
    let leaf = ClaimProduct::estimated(
        ClaimKind::KnownAnswer,
        InputRef::new(hash_blob(b"leaf"), "/leaf"),
        Scope::default(),
        TierMeet::Static,
        DetClass::D0Exact,
        RealityScope::NoClaim,
        FlagSet::new(),
    );
    let combined = ClaimProduct::combine(ClaimKind::KnownAnswer, &[leaf]).unwrap();
    assert_eq!(combined.class(), EvidenceClass::Estimated);
}
