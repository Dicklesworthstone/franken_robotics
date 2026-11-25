//! Closed proof-kind registry, dispatch, class cap, RuleId, and doc parity tests (N12-02).
//!
//! All test inputs are synthetic `(kind, version, scope, class)` tuples exercising
//! the compiled closed registry directly (no mocks).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use frob_cert::json::{JsonValue, parse_canonical};
use frob_cert::kinds::registry::{
    CLAIM_KIND_SCOPES, CheckVerdict, Class, KindStatus, REGISTRY, REGISTRY_VERSION, Scope,
    check_class_cap, dispatch, enabled_kinds, lookup, probe, render_registry_markdown,
    scope_for_claim_kind,
};
use frob_cert::kinds::rules::{
    REFUSAL_CLASS_ABOVE_CAP, REFUSAL_UNKNOWN_PROOF_KIND, REPAIRS_UNKNOWN_KIND, RULE_CATALOGUE,
    RuleId,
};
use frob_cert::limits::COMPILED_LIMITS;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_root()
        .parent()
        .expect("crates/")
        .parent()
        .expect("repo root")
        .to_path_buf()
}

fn collect_files_with_ext(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_ext(&path, exts, out);
        } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if exts.contains(&ext) {
                out.push(path);
            }
        }
    }
}

#[test]
fn registry_has_27_kinds() {
    assert_eq!(REGISTRY.len(), 27, "frob.cert.kinds/1 must list exactly 27 kinds");
    for row in REGISTRY {
        assert_eq!(row.version(), 1, "baseline row {} must start at version 1", row.name());
        assert!(!row.scopes().is_empty(), "row {} must declare at least one scope", row.name());
        assert!(!row.recomputes().is_empty());
        assert!(!row.checker_bead().is_empty());
        assert!(!row.payload_schema().is_empty());
    }
}

#[test]
fn registry_names_unique() {
    let mut seen = BTreeSet::new();
    for row in REGISTRY {
        assert!(
            seen.insert(row.name()),
            "duplicate proof kind name in registry: {}",
            row.name()
        );
    }
    assert_eq!(seen.len(), 27);
}

#[test]
fn registry_caps() {
    let proved: Vec<&str> = REGISTRY
        .iter()
        .filter(|r| r.cap() == Class::Proved)
        .map(|r| r.name())
        .collect();
    assert_eq!(
        proved,
        vec!["lean-capsule"],
        "lean-capsule must be the only proof kind capped at Proved"
    );

    let mut tested: Vec<&str> = REGISTRY
        .iter()
        .filter(|r| r.cap() == Class::Tested)
        .map(|r| r.name())
        .collect();
    tested.sort_unstable();
    assert_eq!(
        tested,
        vec!["sample-coverage", "trace-witness"],
        "only sample-coverage and trace-witness cap at Tested"
    );

    let certified_count = REGISTRY
        .iter()
        .filter(|r| r.cap() == Class::Certified)
        .count();
    assert_eq!(certified_count, 24);
    assert!(lookup("energy-budget").unwrap().capped_by_inputs());
}

#[test]
fn unknown_kind_rejects() {
    // Dispatch half of KA-215 / CX-106: fabricated kind rejects regardless of digests.
    // Under --features gauntlet-mutations,shortcut-accept-on-hashes this test MUST FAIL.
    let res = dispatch("lp-primal-fast", 1, Scope::Mission);
    match res {
        Err(CheckVerdict::Reject {
            rule,
            refusal,
            repairs,
            ..
        }) => {
            assert_eq!(rule, RuleId::KindUnknown);
            assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
            assert_eq!(repairs, REPAIRS_UNKNOWN_KIND);
        }
        Ok(checker) => panic!(
            "fabricated kind must never dispatch to a checker (got {})",
            checker.kind()
        ),
        Err(other) => panic!("expected Reject(KindUnknown), got {other:?}"),
    }
}

#[test]
fn unknown_version_rejects() {
    let res = dispatch("lp-primal", 2, Scope::Mission);
    match res {
        Err(CheckVerdict::Reject {
            rule,
            refusal,
            repairs,
            ..
        }) => {
            assert_eq!(rule, RuleId::KindVersionUnknown);
            assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
            assert_eq!(
                repairs,
                &["re-emit with a registered kind", "upgrade the auditor"]
            );
        }
        Ok(_) => panic!("unknown version 2 must not dispatch"),
        Err(other) => panic!("expected Reject(KindVersionUnknown), got {other:?}"),
    }
}

#[test]
fn out_of_scope_rejects() {
    // poly-identity has scopes [kernel, analysis], NOT mission (family-inclusion).
    let mission_scope = scope_for_claim_kind("family-inclusion").unwrap();
    assert_eq!(mission_scope, Scope::Mission);

    let res = dispatch("poly-identity", 1, mission_scope);
    match res {
        Err(CheckVerdict::Reject { rule, refusal, .. }) => {
            assert_eq!(rule, RuleId::KindOutOfScope);
            assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
        }
        Ok(_) => panic!("out-of-scope poly-identity under mission must not dispatch"),
        Err(other) => panic!("expected Reject(KindOutOfScope), got {other:?}"),
    }

    // Same via full probe.
    let outcome = probe("poly-identity", 1, "family-inclusion", Class::Certified);
    assert_eq!(outcome.scope_check, "fail");
    match outcome.verdict {
        CheckVerdict::Reject { rule, refusal, .. } => {
            assert_eq!(rule, RuleId::KindOutOfScope);
            assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
        }
        other => panic!("expected Reject(KindOutOfScope), got {other:?}"),
    }
}

#[test]
fn claim_kind_scope_table() {
    // Every documented claim.kind maps to its exact Scope (including the analysis
    // claim kinds resolving round-4 slice S-04 review note #1).
    let expected = [
        ("kernel-identity", Scope::Kernel),
        ("family-inclusion", Scope::Mission),
        ("family-exclusion", Scope::Mission),
        ("partition", Scope::Mission),
        ("qualification-receipt", Scope::Study),
        ("analysis", Scope::Analysis),
        ("card-instance", Scope::Analysis),
        ("law-convexity", Scope::Analysis),
        ("energy-budget", Scope::Analysis),
        ("theorem-card", Scope::Analysis),
    ];
    assert_eq!(CLAIM_KIND_SCOPES.len(), expected.len());
    for (claim_kind, exp_scope) in expected {
        assert_eq!(
            scope_for_claim_kind(claim_kind),
            Ok(exp_scope),
            "claim.kind {claim_kind:?} must map to {exp_scope:?}"
        );
    }

    // Any unlisted claim kind is rejected with kind.out-of-scope and FROB-REF-823.
    for unlisted in ["invented-claim", "refinement", "an analysis claim", ""] {
        match scope_for_claim_kind(unlisted) {
            Err(CheckVerdict::Reject { rule, refusal, .. }) => {
                assert_eq!(rule, RuleId::KindOutOfScope, "unlisted {unlisted:?}");
                assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
            }
            other => panic!("expected Reject(KindOutOfScope) for {unlisted:?}, got {other:?}"),
        }
    }
}

#[test]
fn disabled_kind_rejects() {
    #[cfg(feature = "registry-fixture")]
    {
        let res = dispatch("alpha-root", 1, Scope::Kernel);
        match res {
            Err(CheckVerdict::Reject { rule, refusal, .. }) => {
                assert_eq!(rule, RuleId::KindDisabled);
                assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
            }
            Ok(_) => panic!("disabled kind must never dispatch"),
            Err(other) => panic!("expected Reject(KindDisabled), got {other:?}"),
        }
    }
    #[cfg(not(feature = "registry-fixture"))]
    {
        // In non-fixture builds, alpha-root stays not-implemented (never enabled/accepted);
        // disabled-pending-cause dispatch is additionally covered by the inline unit test
        // `kinds::registry::tests::disabled_kind_rejects_in_table` and by running this test
        // with `--features registry-fixture`.
        let row = lookup("alpha-root").unwrap();
        assert_eq!(row.status(), KindStatus::NotImplemented);
        assert!(matches!(
            dispatch("alpha-root", 1, Scope::Kernel),
            Err(CheckVerdict::Unsupported { .. })
        ));
    }
}

#[test]
fn not_implemented_is_unsupported() {
    let res = dispatch("tube-inclusion", 1, Scope::Kernel);
    match res {
        Err(CheckVerdict::Unsupported { kind, version, .. }) => {
            assert_eq!(kind, "tube-inclusion");
            assert_eq!(version, 1);
        }
        Ok(_) => panic!("not-implemented kind must never dispatch or accept"),
        Err(other) => panic!("expected Unsupported, got {other:?}"),
    }

    // Verify no not-implemented row ever returns Accept through probe().
    for row in REGISTRY {
        if row.status() == KindStatus::NotImplemented {
            let scope = row.scopes()[0];
            let claim_kind = CLAIM_KIND_SCOPES
                .iter()
                .find(|c| c.scope() == scope)
                .unwrap()
                .claim_kind();
            let outcome = probe(row.name(), row.version(), claim_kind, row.cap());
            assert_eq!(outcome.scope_check, "pass", "row {}", row.name());
            assert_eq!(outcome.cap_check, "pass", "row {}", row.name());
            assert!(
                matches!(outcome.verdict, CheckVerdict::Unsupported { .. }),
                "not-implemented row {} must yield Unsupported, got {:?}",
                row.name(),
                outcome.verdict
            );
        }
    }
}

#[test]
fn rule_ids_closed() {
    // Exhaustive match proves every RuleId variant is accounted for and present in RULE_CATALOGUE.
    let mut seen_str = BTreeSet::new();
    let mut areas = BTreeSet::new();
    for &rule in RULE_CATALOGUE {
        let s = rule.as_str();
        assert!(seen_str.insert(s), "duplicate RuleId string {s:?}");
        assert!(
            s.is_ascii(),
            "RuleId {s:?} must be ASCII"
        );
        let (area, suffix) = s
            .split_once('.')
            .unwrap_or_else(|| panic!("RuleId {s:?} must be <area>.<rule>"));
        assert!(!area.is_empty() && !suffix.is_empty());
        assert_eq!(rule.area(), area);
        assert_eq!(RuleId::parse(s), Some(rule));
        areas.insert(area);

        // Exhaustive match over every variant so adding a variant forces updating this test.
        let _exhaustive: u8 = match rule {
            RuleId::InputUnreadable
            | RuleId::JsonSyntax
            | RuleId::JsonBom
            | RuleId::JsonDuplicateKey
            | RuleId::JsonKeyOrder
            | RuleId::JsonWhitespace
            | RuleId::JsonEscape
            | RuleId::SchemaUnknown
            | RuleId::SchemaMissingField
            | RuleId::SchemaUnknownField
            | RuleId::SchemaFieldType
            | RuleId::SchemaReviewMissing
            | RuleId::HashMismatch
            | RuleId::HashEquationsDigest
            | RuleId::HashMerkleRoot
            | RuleId::HashMerkleDuplicate
            | RuleId::KindUnknown
            | RuleId::KindVersionUnknown
            | RuleId::KindOutOfScope
            | RuleId::KindDisabled
            | RuleId::QueryMismatch
            | RuleId::QueryDigestMismatch
            | RuleId::QueryPrefix
            | RuleId::QueryDomain
            | RuleId::QueryAssumptions
            | RuleId::QueryPolicyClass
            | RuleId::SideMissing
            | RuleId::SideEquationsMissing
            | RuleId::SideConditionMissing
            | RuleId::SideConditionRejected
            | RuleId::FieldType
            | RuleId::FieldUnitsMismatch
            | RuleId::FieldFrameMismatch
            | RuleId::FieldPortOrder
            | RuleId::ClaimMinimumUnbounded
            | RuleId::ClaimMassWithoutWeighting
            | RuleId::NumFloatInBundle
            | RuleId::NumIntRange
            | RuleId::NumIntForm
            | RuleId::NumRationalForm
            | RuleId::NumIntervalDecl
            | RuleId::LimitsExceeded
            | RuleId::ClassAboveKindCap
            | RuleId::ClassAboveInput
            | RuleId::ClassNoReceipt
            | RuleId::FlagsMissing
            | RuleId::FlagsTrue
            | RuleId::PolyIdentityCheckFailed
            | RuleId::RationalLdltCheckFailed
            | RuleId::IntervalBoxCheckFailed
            | RuleId::IntervalRootCheckFailed
            | RuleId::TaylorBoxCheckFailed
            | RuleId::SosGramCheckFailed
            | RuleId::LpFarkasCheckFailed
            | RuleId::ConicFarkasCheckFailed
            | RuleId::LpPrimalCheckFailed
            | RuleId::FiniteGameCheckFailed
            | RuleId::TraceWitnessCheckFailed
            | RuleId::PartitionCheckFailed
            | RuleId::MonotoneTransportCheckFailed
            | RuleId::SymmetryTransportCheckFailed
            | RuleId::CompositionCheckFailed
            | RuleId::MonomialProductCheckFailed
            | RuleId::ComplexConnectivityCheckFailed
            | RuleId::TubeInclusionCheckFailed
            | RuleId::RewriteTraceCheckFailed
            | RuleId::OrbitCountCheckFailed
            | RuleId::GroupEnumerationCheckFailed
            | RuleId::EnergyBudgetCheckFailed
            | RuleId::AliasPairCheckFailed
            | RuleId::ProductUnreachableCheckFailed
            | RuleId::AlphaRootCheckFailed
            | RuleId::LeanCapsuleCheckFailed
            | RuleId::SampleCoverageCheckFailed => 1,
        };
    }

    // All 10 named generic areas + one area per every one of the 27 proof kinds.
    for required_area in [
        "schema", "hash", "kind", "query", "side", "field", "num", "limits", "class", "flags",
    ] {
        assert!(
            areas.contains(required_area),
            "RULE_CATALOGUE missing required area {required_area:?}"
        );
    }
    for row in REGISTRY {
        assert!(
            areas.contains(row.name()),
            "RULE_CATALOGUE missing per-kind area for {:?}",
            row.name()
        );
    }
}

#[test]
fn sample_coverage_certified_rejects() {
    let row = lookup("sample-coverage").unwrap();
    match check_class_cap(row, Class::Certified) {
        Err(CheckVerdict::Reject { rule, refusal, .. }) => {
            assert_eq!(rule, RuleId::ClassAboveKindCap);
            assert_eq!(refusal, Some(REFUSAL_CLASS_ABOVE_CAP));
        }
        other => panic!("expected Reject(ClassAboveKindCap), got {other:?}"),
    }
    assert_eq!(check_class_cap(row, Class::Tested), Ok(()));
    assert_eq!(check_class_cap(row, Class::Estimated), Ok(()));
}

#[test]
fn trace_witness_certified_rejects() {
    let row = lookup("trace-witness").unwrap();
    match check_class_cap(row, Class::Certified) {
        Err(CheckVerdict::Reject { rule, refusal, .. }) => {
            assert_eq!(rule, RuleId::ClassAboveKindCap);
            assert_eq!(refusal, Some(REFUSAL_CLASS_ABOVE_CAP));
        }
        other => panic!("expected Reject(ClassAboveKindCap), got {other:?}"),
    }
    assert_eq!(check_class_cap(row, Class::Tested), Ok(()));
    assert_eq!(check_class_cap(row, Class::Estimated), Ok(()));
}

#[test]
fn generated_document_matches_table() {
    let doc_path = repo_root().join("docs").join("registries").join("cert-kinds.md");
    let committed = fs::read_to_string(&doc_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", doc_path.display()));
    let generated = render_registry_markdown();

    #[cfg(not(feature = "registry-fixture"))]
    {
        if committed != generated {
            let first_diff = committed
                .lines()
                .zip(generated.lines())
                .enumerate()
                .find(|(_, (a, b))| a != b);
            panic!(
                "docs/registries/cert-kinds.md differs from compiled table (first diff: {:?}); regenerate with `cargo run -p frob-cert --example kind_registry -- --doc`",
                first_diff
            );
        }
    }
    #[cfg(feature = "registry-fixture")]
    {
        assert!(committed.contains(REGISTRY_VERSION));
        assert!(generated.contains("disabled-pending-cause"));
    }
}

#[test]
fn registry_version_exposed() {
    assert_eq!(REGISTRY_VERSION, "frob.cert.kinds/1");

    let expected_enabled: Vec<(&str, u32)> = REGISTRY
        .iter()
        .filter(|r| r.status() == KindStatus::Enabled)
        .map(|r| (r.name(), r.version()))
        .collect();
    assert_eq!(enabled_kinds(), expected_enabled);

    // Source scan of crates/frob-cert/src: `frob.cert.kinds/` may appear ONLY in
    // src/kinds/registry.rs and nowhere else in src/.
    let src_dir = crate_root().join("src");
    let mut rs_files = Vec::new();
    collect_files_with_ext(&src_dir, &["rs"], &mut rs_files);
    let mut matching_files = Vec::new();
    for path in rs_files {
        let content = fs::read_to_string(&path).unwrap();
        if content.contains("frob.cert.kinds/") {
            matching_files.push(path.strip_prefix(&src_dir).unwrap().to_path_buf());
        }
    }
    assert_eq!(
        matching_files,
        vec![PathBuf::from("kinds").join("registry.rs")],
        "frob.cert.kinds/ version literal must appear ONLY in src/kinds/registry.rs, found in {matching_files:?}"
    );
}

#[test]
fn specimens_in_scope() {
    let spec_dir = crate_root().join("tests").join("specimens");
    if !spec_dir.is_dir() {
        return;
    }
    let mut files = Vec::new();
    collect_files_with_ext(&spec_dir, &["frobcert", "json"], &mut files);
    files.sort();

    for path in files {
        let bytes = fs::read(&path)
            .unwrap_or_else(|e| panic!("failed to read specimen {}: {e}", path.display()));
        let doc = parse_canonical(&bytes, &COMPILED_LIMITS)
            .unwrap_or_else(|e| panic!("specimen {} is not canonical JSON: {e}", path.display()));
        let expected_verdict = doc
            .get("expected_verdict")
            .and_then(JsonValue::as_str)
            .unwrap_or("");
        if expected_verdict != "accept" {
            continue;
        }
        let claim_kind = doc
            .get("claim")
            .and_then(|c| c.get("kind"))
            .and_then(JsonValue::as_str)
            .unwrap_or_else(|| panic!("specimen {} missing claim.kind", path.display()));
        let proof_kind = doc
            .get("proof")
            .and_then(|p| p.get("kind"))
            .and_then(JsonValue::as_str)
            .unwrap_or_else(|| panic!("specimen {} missing proof.kind", path.display()));
        let scope = scope_for_claim_kind(claim_kind).unwrap_or_else(|e| {
            panic!(
                "accept specimen {} rejected by kind.out-of-scope on claim.kind={claim_kind:?}: {e:?}",
                path.display()
            )
        });
        let row = lookup(proof_kind).unwrap_or_else(|| {
            panic!(
                "accept specimen {} names unknown proof kind {proof_kind:?}",
                path.display()
            )
        });
        assert!(
            row.allows_scope(scope),
            "accept specimen {} rejected by kind.out-of-scope: review registry row {:?} and scope {:?} (claim.kind={claim_kind:?})",
            path.display(),
            row.name(),
            scope.as_str()
        );
    }
}
