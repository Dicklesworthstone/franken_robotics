//! Workspace refusal-code registry tests: FROB-REF-891, -892, -893 and uncoded interim
//! `[[detail]]` mappings (N34-REF).

use std::fs;
use std::path::PathBuf;

use frob_core::json::{Limits, admit};
use frob_core::refusal::{RefusalCode, RetryPolicy};
use frob_core::registry::refusals::{
    EMBEDDED_REFUSALS_TOML, RefusalRegistry, RefusalRegistryError, render_refusal_code_enum,
};
use frob_core::registry::toml::{self, Fields};

const WORKSPACE_CODES: &[(&str, RefusalCode)] = &[
    ("FROB-REF-891", RefusalCode::FrobRef891),
    ("FROB-REF-892", RefusalCode::FrobRef892),
    ("FROB-REF-893", RefusalCode::FrobRef893),
];

const FIXTURE_SIDECARS: &[(&str, &str, &str)] = &[
    ("ref891_major_raised.sidecar.toml", "FROB-REF-891", "artifact"),
    ("ref891_unknown_field.sidecar.toml", "FROB-REF-891", "artifact"),
    ("ref892_view_sexp.sidecar.toml", "FROB-REF-892", "identity-required"),
    ("ref892_noncanonical.sidecar.toml", "FROB-REF-892", "identity-required"),
    ("ref893_registered_ext.sidecar.toml", "FROB-REF-893", "foreign-file"),
    ("ref893_registered_schema.sidecar.toml", "FROB-REF-893", "foreign-file"),
];

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/workspace_refusals")
}

fn load_and_validate_sidecar(bytes: &[u8]) -> Result<(String, String, String, String, bool), String> {
    let tbl = toml::parse(bytes).map_err(|e| e.to_string())?;
    let mut f = Fields::new(tbl, "sidecar");
    let code = f.require_str("code").map_err(|e| e.to_string())?;
    let fixture = f.require_str("fixture").map_err(|e| e.to_string())?;
    let parameter_kind = f.require_str("parameter_kind").map_err(|e| e.to_string())?;
    let observed = f.require_str("observed").map_err(|e| e.to_string())?;
    let _required = f.require_str("required").map_err(|e| e.to_string())?;
    let is_view = f.require_bool("is_view").map_err(|e| e.to_string())?;
    f.finish().map_err(|e| e.to_string())?;
    if !WORKSPACE_CODES.iter().any(|(c, _)| *c == code) {
        return Err(format!("sidecar code `{code}` is not one of FROB-REF-891/-892/-893"));
    }
    Ok((code, fixture, parameter_kind, observed, is_view))
}

#[test]
fn rows_891_892_893_present() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml loads");
    for &(code_str, variant) in WORKSPACE_CODES {
        let row = reg.lookup(code_str, "N34-REF").expect(code_str);
        assert_eq!(row.code, code_str);
        assert_eq!(row.defined_in, "§34.9");
        assert_eq!(RefusalCode::parse(code_str), Some(variant));
        assert_eq!(reg.row(variant).map(|r| r.code.as_str()), Some(code_str));
    }
    assert_eq!(include_str!("../src/refusal/code_gen.rs"), render_refusal_code_enum(&reg));
}

#[test]
fn exit_and_retry_values() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    for &(code_str, variant) in WORKSPACE_CODES {
        let row = reg.row(variant).expect(code_str);
        assert_eq!(row.exit, 3, "{code_str} must exit 3");
        assert_eq!(reg.exit_for(variant).unwrap(), 3);
        assert_eq!(row.retry, RetryPolicy::AfterChange, "{code_str} must retry after-change");
    }
}

#[test]
fn repairs_are_ranked_and_nonempty() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    reg.validate_row_metadata().expect("all rows carry non-empty repairs and raised_by");
    for &(code_str, variant) in WORKSPACE_CODES {
        let row = reg.row(variant).expect(code_str);
        assert!(row.repairs.len() >= 2, "{code_str} needs >= 2 ranked repairs");
        assert!(row.repairs.iter().all(|r| !r.trim().is_empty()));
    }
    // Negative control: a row with an empty repair list fails metadata validation.
    let planted = EMBEDDED_REFUSALS_TOML.replace(
        "code = \"FROB-REF-891\"\n",
        "code = \"FROB-REF-891\"\n",
    );
    let r891 = reg.row(RefusalCode::FrobRef891).unwrap();
    let old_rep = format!("repairs = [{:?}, {:?}, {:?}]", r891.repairs[0], r891.repairs[1], r891.repairs[2]);
    let empty_rep = planted.replace(&old_rep, "repairs = []");
    assert_ne!(empty_rep, planted, "plant must mutate FROB-REF-891 repairs");
    let bad_reg = RefusalRegistry::from_bytes(empty_rep.as_bytes()).expect("parses syntactically");
    assert!(matches!(
        bad_reg.validate_row_metadata(),
        Err(RefusalRegistryError::EmptyRepairs { ref code }) if code == "FROB-REF-891"
    ));
}

#[test]
fn detail_mapping_for_uncoded_faults() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    let find = |slug: &str| reg.details().iter().find(|d| d.slug == slug).unwrap_or_else(|| panic!("missing [[detail]] `{slug}`"));

    let dup = find("duplicate-member-name");
    assert_eq!(dup.detail, "duplicate-member-name");
    assert_eq!(dup.interim, vec!["FROB-REF-802", "FROB-REF-891"]);
    assert!(dup.ledger.contains("duplicate-member-name"));

    let ident = find("identifier-not-printable");
    assert_eq!(ident.detail, "identifier-not-printable");
    assert_eq!(ident.interim, vec!["FROB-REF-802"]);
    assert!(ident.ledger.contains("identifier-not-printable"));

    let fix = find("fixture-reference-unresolved");
    assert_eq!(fix.detail, "fixture-reference-unresolved");
    assert_eq!(fix.interim, vec!["FROB-REF-802"]);
    assert_eq!(fix.exit, 3);
    assert_eq!(fix.retry, RetryPolicy::AfterChange);
    assert_eq!(fix.repairs.len(), 2);
    assert!(fix.ledger.contains("fixture-reference-unresolved"));
    for form in [
        "fixture-name-invalid",
        "fixtures-root-absent",
        "fixture-unknown:<name>",
        "fixture-outside-root:<name>",
        "fixture-ambiguous:<name>:<count>",
        "fixture-empty:<name>",
    ] {
        assert!(fix.observed.iter().any(|o| o == form), "missing observed form `{form}`");
    }

    // No [[detail]] row may point to a held/proposed/unregistered code (e.g. 836..=839).
    for d in reg.details() {
        assert!(!d.interim.is_empty());
        for c in &d.interim {
            assert!(reg.lookup(c, "detail").is_ok(), "interim `{c}` must exist in registry");
            assert!(!matches!(c.as_str(), "FROB-REF-836" | "FROB-REF-837" | "FROB-REF-838" | "FROB-REF-839"));
        }
    }
    let bad_interim = EMBEDDED_REFUSALS_TOML.replace(
        "slug = \"fixture-reference-unresolved\"\ndetail = \"fixture-reference-unresolved\"\ninterim = [\"FROB-REF-802\"]",
        "slug = \"fixture-reference-unresolved\"\ndetail = \"fixture-reference-unresolved\"\ninterim = [\"FROB-REF-836\"]",
    );
    assert_ne!(bad_interim, EMBEDDED_REFUSALS_TOML);
    assert!(matches!(
        RefusalRegistry::from_bytes(bad_interim.as_bytes()),
        Err(RefusalRegistryError::UnknownRefusalCode { ref code, .. }) if code == "FROB-REF-836"
    ));
}

#[test]
fn raised_by_nonempty() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    let r891 = reg.row(RefusalCode::FrobRef891).unwrap();
    let r892 = reg.row(RefusalCode::FrobRef892).unwrap();
    let r893 = reg.row(RefusalCode::FrobRef893).unwrap();
    assert!(r891.raised_by.iter().any(|s| s == "N34-19"));
    assert!(r892.raised_by.iter().any(|s| s == "N34-24"));
    assert!(r893.raised_by.iter().any(|s| s == "N34-24"));

    // Negative control: empty raised_by is refused by registry metadata validation.
    let empty_raised = EMBEDDED_REFUSALS_TOML.replace(
        "raised_by = [\"N34-19\", \"format-owner-readers\"]",
        "raised_by = []",
    );
    assert_ne!(empty_raised, EMBEDDED_REFUSALS_TOML);
    let bad = RefusalRegistry::from_bytes(empty_raised.as_bytes()).expect("parses syntactically");
    assert!(matches!(
        bad.validate_row_metadata(),
        Err(RefusalRegistryError::EmptyRaisedBy { ref code }) if code == "FROB-REF-891"
    ));
}

#[test]
fn family_exit_rule_holds() {
    let planted = EMBEDDED_REFUSALS_TOML.replacen(
        "code = \"FROB-REF-891\"\n",
        "code = \"FROB-REF-891\"\n",
        1,
    );
    let target = "defined_in = \"§34.9\"\nexit = 3\nretry = \"after-change\"";
    let bad = planted.replacen(target, "defined_in = \"§34.9\"\nexit = 4\nretry = \"after-change\"", 1);
    assert_ne!(bad, planted, "plant must change FROB-REF-891 exit to 4");
    assert!(matches!(
        RefusalRegistry::from_bytes(bad.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch { ref row, observed: 4, expected: Some(3), .. }) if row == "FROB-REF-891"
    ));
}

#[test]
fn fixtures_exist_and_admit() {
    let dir = fixtures_dir();
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    let limits = Limits::default();
    for &(sidecar_name, want_code, want_kind) in FIXTURE_SIDECARS {
        let sidecar_bytes = fs::read(dir.join(sidecar_name)).expect(sidecar_name);
        let (code, fixture, kind, observed, is_view) =
            load_and_validate_sidecar(&sidecar_bytes).expect(sidecar_name);
        assert_eq!(code, want_code);
        assert_eq!(kind, want_kind);
        assert!(!observed.trim().is_empty());
        let row = reg.lookup(&code, "fixture").expect("code in registry");
        assert!(row.meaning.contains(&fixture), "{} must name fixture `{fixture}`", row.code);

        let raw = fs::read(dir.join(&fixture)).expect(&fixture);
        if is_view {
            let text = std::str::from_utf8(&raw).expect("view fixture is UTF-8");
            assert!(text.trim_start().starts_with("(design "), "registered sexp view expected");
        } else {
            let admitted = admit(&raw, &limits)
                .unwrap_or_else(|e| panic!("fixture `{fixture}` must pass raw-byte admission: {e}"));
            if observed == "non-canonical-spelling" {
                let canon = admitted.into_canonical().expect("canonicalizes");
                assert_ne!(raw.trim_ascii(), canon.as_bytes(), "892 non-canonical fixture must differ from canonical bytes");
            }
        }
    }

    // Negative control: sidecar naming a code outside {891, 892, 893} is rejected.
    let bad_sidecar = b"code = \"FROB-REF-802\"\nfixture = \"x.json\"\nparameter_kind = \"artifact\"\nobserved = \"x\"\nrequired = \"y\"\nis_view = false\n";
    assert!(load_and_validate_sidecar(bad_sidecar).is_err());
}
