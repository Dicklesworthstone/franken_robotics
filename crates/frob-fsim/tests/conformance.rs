#![forbid(unsafe_code)]

//! Contract conformance checks for `frob-fsim` (`BD-006`, `N34-08`).

use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Value};

#[test]
fn contract_files_exist_and_match_layer_map() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let contract_md = fs::read_to_string(manifest_dir.join("CONTRACT.md"))
        .expect("crates/frob-fsim/CONTRACT.md must exist");
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
    for crate_name in frob_fsim::ADMITTED_FRANKENSIM_CRATES {
        assert!(
            contract_md.contains(crate_name),
            "CONTRACT.md must name admitted crate `{crate_name}`"
        );
    }
    assert!(
        contract_md.contains(frob_fsim::FRANKENSIM_PIN_REV),
        "CONTRACT.md must state the FrankenSim pin"
    );

    let contract_toml_bytes = fs::read(manifest_dir.join("contract.toml"))
        .expect("crates/frob-fsim/contract.toml must exist");
    let root = toml::parse(&contract_toml_bytes).expect("contract.toml must parse strictly");
    let crate_tbl = root
        .get("crate")
        .and_then(Value::as_table)
        .expect("[crate] table must exist");
    assert_eq!(crate_tbl.get("name").and_then(Value::as_str), Some("frob-fsim"));
    assert_eq!(crate_tbl.get("layer").and_then(Value::as_int), Some(0));
    assert_eq!(crate_tbl.get("rank").and_then(Value::as_int), Some(2));
    assert_eq!(crate_tbl.get("plane").and_then(Value::as_str), Some("compute"));
}

#[test]
fn airlock_smoke_checks_pass() {
    let results = frob_fsim::smoke_all_admitted();
    assert_eq!(results.len(), 5);
    for res in results {
        assert!(
            res.passed,
            "airlock smoke failed for `{}` ({})",
            res.crate_name, res.item_path
        );
    }
}
