//! Numerics refusal-code registry tests: FROB-REF-247..252, 561, 562, 761, 861..866 (N35-REF).

use frob_core::refusal::{RefusalCode, RetryPolicy};
use frob_core::registry::refusals::{
    EMBEDDED_REFUSALS_TOML, RefusalRegistry, RefusalRegistryError, render_refusal_code_enum,
};

const NUMERICS_CODES: &[(&str, RefusalCode, u8)] = &[
    ("FROB-REF-247", RefusalCode::FrobRef247, 3),
    ("FROB-REF-248", RefusalCode::FrobRef248, 3),
    ("FROB-REF-249", RefusalCode::FrobRef249, 3),
    ("FROB-REF-250", RefusalCode::FrobRef250, 3),
    ("FROB-REF-251", RefusalCode::FrobRef251, 3),
    ("FROB-REF-252", RefusalCode::FrobRef252, 3),
    ("FROB-REF-561", RefusalCode::FrobRef561, 3),
    ("FROB-REF-562", RefusalCode::FrobRef562, 3),
    ("FROB-REF-761", RefusalCode::FrobRef761, 4),
    ("FROB-REF-861", RefusalCode::FrobRef861, 3),
    ("FROB-REF-862", RefusalCode::FrobRef862, 3),
    ("FROB-REF-863", RefusalCode::FrobRef863, 3),
    ("FROB-REF-864", RefusalCode::FrobRef864, 3),
    ("FROB-REF-865", RefusalCode::FrobRef865, 3),
    ("FROB-REF-866", RefusalCode::FrobRef866, 3),
];

const RELATED_PAIRS: &[(&str, &str)] = &[
    ("FROB-REF-561", "FROB-REF-825"),
    ("FROB-REF-561", "FROB-REF-542"),
    ("FROB-REF-561", "FROB-REF-533"),
    ("FROB-REF-247", "FROB-REF-894"),
    ("FROB-REF-861", "FROB-REF-867"),
    ("FROB-REF-562", "FROB-REF-584"),
    ("FROB-REF-864", "FROB-REF-605"),
    ("FROB-REF-248", "FROB-REF-244"),
    ("FROB-REF-761", "FROB-REF-704"),
];

#[test]
fn refusals_numerics_rows_load() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml loads");
    reg.validate_row_metadata()
        .expect("all rows have non-empty raised_by and repairs");
    let (mut c2, mut c5, mut c7, mut c8) = (0usize, 0usize, 0usize, 0usize);
    for &(code_str, variant, _) in NUMERICS_CODES {
        let row = reg.lookup(code_str, "N35-REF").expect(code_str);
        assert_eq!(row.defined_in, "§35.11");
        assert_eq!(RefusalCode::parse(code_str), Some(variant));
        match variant.family() {
            2 => c2 += 1,
            5 => c5 += 1,
            7 => c7 += 1,
            8 => c8 += 1,
            f => panic!("unexpected family {f} for {code_str}"),
        }
    }
    assert_eq!((c2, c5, c7, c8), (6, 2, 1, 6));
    for retired in ["FROB-REF-730", "FROB-REF-882"] {
        assert!(matches!(
            reg.lookup(retired, "N35-33 plan-time retirements"),
            Err(RefusalRegistryError::UnknownRefusalCode { .. })
        ));
    }
    for code in [
        RefusalCode::FrobRef250,
        RefusalCode::FrobRef251,
        RefusalCode::FrobRef252,
    ] {
        assert!(
            reg.row(code)
                .expect("build-time row")
                .meaning
                .contains("(build time)")
        );
    }
    // Review note 3 (slice S-11): 248 raised_by begins with N21b-38.
    assert_eq!(
        reg.row(RefusalCode::FrobRef248)
            .unwrap()
            .raised_by
            .first()
            .map(String::as_str),
        Some("N21b-38")
    );
    assert_eq!(
        include_str!("../src/refusal/code_gen.rs"),
        render_refusal_code_enum(&reg)
    );
}

#[test]
fn refusals_numerics_exit_codes() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    for &(code_str, variant, want_exit) in NUMERICS_CODES {
        let row = reg.row(variant).expect(code_str);
        assert_eq!(row.exit, want_exit, "{code_str} exit");
        assert_eq!(reg.exit_for(variant).unwrap(), want_exit);
        assert_eq!(row.retry, RetryPolicy::AfterChange, "{code_str} retry");
    }
}

#[test]
fn refusals_numerics_repairs_ranked() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    for &(code_str, variant, _) in NUMERICS_CODES {
        let row = reg.row(variant).expect(code_str);
        assert!(
            row.repairs.len() >= 2,
            "{code_str} must carry at least 2 ranked repairs"
        );
        assert!(row.repairs.iter().all(|r| !r.trim().is_empty()));
    }
    let r248 = reg.row(RefusalCode::FrobRef248).unwrap();
    assert!(
        r248.repairs[0].contains("--precision stiff"),
        "248 repair[0]: {}",
        r248.repairs[0]
    );
    let r861 = reg.row(RefusalCode::FrobRef861).unwrap();
    assert!(
        r861.repairs[0].contains("frob eval run --policy <id> --determinism device-replay"),
        "861 repair[0]: {}",
        r861.repairs[0]
    );
}

#[test]
fn refusals_numerics_related_symmetric() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    for &(lhs, rhs) in RELATED_PAIRS {
        let row_lhs = reg.lookup(lhs, "N35-REF").expect(lhs);
        assert!(
            row_lhs.related.iter().any(|c| c == rhs),
            "{lhs} must list {rhs} in related"
        );
        match reg.lookup(rhs, "N35-REF") {
            Ok(row_rhs) => {
                assert!(
                    row_rhs.related.iter().any(|c| c == lhs),
                    "{rhs} must list {lhs} in related"
                );
                assert_ne!(
                    row_lhs.meaning, row_rhs.meaning,
                    "{lhs} and {rhs} must state distinct raising sites"
                );
            }
            Err(RefusalRegistryError::UnknownRefusalCode { .. }) => {
                eprintln!("unbound relationship: {lhs} -> {rhs}; partner row absent");
            }
            Err(error) => panic!("could not look up {rhs}: {error}"),
        }
    }
    // 761 <-> 704 is already landed on both sides and must be bidirectional now.
    let r704 = reg
        .lookup("FROB-REF-704", "N35-REF")
        .expect("704 landed in N32-REF");
    assert!(r704.related.iter().any(|c| c == "FROB-REF-761"));
    let r561 = reg.row(RefusalCode::FrobRef561).expect("561");
    for site in [
        "On eval.compare, pairing::admit first raises FROB-REF-542",
        "561 covers an unrecorded class or an exact pairing",
        "On eval.detect, 561 covers a D4 or unrecorded arm",
        "On eval.promote, 561 covers a fast-tier result and its re-evaluation",
        "On study.compare and eval.promote the stored-result evidence boundary raises FROB-REF-825",
        "Arena study::admit raises FROB-REF-533",
    ] {
        assert!(
            r561.meaning.contains(site),
            "561 missing raising site `{site}`"
        );
    }
}

#[test]
fn refusals_perf_rows_complete() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");

    let r761 = reg.row(RefusalCode::FrobRef761).unwrap();
    assert_eq!(r761.repairs.len(), 4);
    assert!(
        r761.repairs[2].contains("--envelope-scope"),
        "761 repair[2] must cite --envelope-scope"
    );
    for needle in [
        "scope order: frob-launch launcher token, cited --envelope-scope, verb-derived scope e.g. train.family, else adhoc",
        "before allocation after FROB-REF-602 and FROB-REF-704",
        "before FROB-REF-701",
        "pretraining E3 without written ceiling and kill rule",
        "(1) memory",
        "(2) cores",
        "(3) devices",
        "(4) wall-time interval upper end",
        "(5) cumulative device-hours",
        "(6) approved scope limit",
        "child run verb exceeding launcher token declaration",
        "cited envelope_scope contradicting launcher token",
        "repairs in order: drop the citation so the token scope applies, or run under a launcher of the cited scope",
    ] {
        assert!(
            r761.meaning.contains(needle),
            "761 meaning missing `{needle}`"
        );
    }

    let r862 = reg.row(RefusalCode::FrobRef862).unwrap();
    assert_eq!(r862.repairs.len(), 3);
    for needle in [
        "(1) required field absent",
        "(2) na used where applicability table forbids it",
        "(3) energy is zero",
        "(4) step rate without its decimation",
        "(5) stale fingerprint digest",
        "(6) ratio block without comparator revision or fewer than ten pairs",
        "(7) parity-defect field missing",
        r#"detail = "rates-violate-unit-ordering""#,
        "rates.ordering",
    ] {
        assert!(
            r862.meaning.contains(needle),
            "862 meaning missing `{needle}`"
        );
    }

    let r863 = reg.row(RefusalCode::FrobRef863).unwrap();
    assert_eq!(r863.repairs.len(), 3);
    for needle in [
        "(1) ratio stated with no campaign",
        "(2) campaign of fewer than ten pairs",
        "(3) arms whose units or decimations are not comparable",
        "(4) void campaign where an arm failed",
        "(5) threshold judged on mean instead of bound",
    ] {
        assert!(
            r863.meaning.contains(needle),
            "863 meaning missing `{needle}`"
        );
    }

    let r866 = reg.row(RefusalCode::FrobRef866).unwrap();
    assert_eq!(r866.repairs.len(), 4);
    for needle in [
        "(1) campaign of kind optimization or speed-experiment with no ceiling record",
        "(2) ceiling filed after first measurement",
        "(3) ceiling computed on cold or uncached baseline",
        "(4) ceiling below goal",
        "(5) pilot whose falsifier is a speed ratio with no ceiling",
    ] {
        assert!(
            r866.meaning.contains(needle),
            "866 meaning missing `{needle}`"
        );
    }

    for (row, prefixes) in [
        (
            r761,
            &[
                "Use a smaller job",
                "Split the work",
                "Request owner approval",
                "Take the registered",
            ][..],
        ),
        (
            r862,
            &[
                "Re-run through the performance harness",
                "Supply the named fields",
                "Present the figure",
            ],
        ),
        (
            r863,
            &[
                "Run a paired campaign",
                "State the two rates separately",
                "Cite the figures as context",
            ],
        ),
        (
            r866,
            &[
                "Compute the ceiling",
                "Lower the goal",
                "Choose another lever",
                "Fund the work",
            ],
        ),
    ] {
        for (repair, &prefix) in row.repairs.iter().zip(prefixes) {
            assert!(
                repair.starts_with(prefix),
                "{} repair out of order: {repair}",
                row.code
            );
        }
    }
}

#[test]
fn refusal_unknown_code_is_load_error() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    // Compute the lowest unregistered 2xx number dynamically (never hardcode 253).
    let absent = (200u16..=299)
        .map(|n| format!("FROB-REF-{n}"))
        .find(|c| reg.lookup(c, "probe").is_err())
        .expect("at least one 2xx number is unused");
    let verbs = format!(
        "schema = \"frob.verbs/1\"\n[[verb]]\nname = \"num.probe\"\nrefusals = [\"{absent}\"]\n"
    );
    assert!(matches!(
        RefusalRegistry::load_with_verbs(EMBEDDED_REFUSALS_TOML.as_bytes(), verbs.as_bytes()),
        Err(RefusalRegistryError::UnknownRefusalCode { ref code, .. }) if code == &absent
    ));
}

#[test]
fn refusal_exit_family_rule() {
    let planted = EMBEDDED_REFUSALS_TOML.replacen(
        "defined_in = \"§35.11\"\nexit = 3\nretry = \"after-change\"",
        "defined_in = \"§35.11\"\nexit = 6\nretry = \"after-change\"",
        1,
    );
    assert_ne!(
        planted, EMBEDDED_REFUSALS_TOML,
        "plant must mutate a 2xx row exit to 6"
    );
    assert!(matches!(
        RefusalRegistry::from_bytes(planted.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch {
            observed: 6,
            expected: Some(3),
            ..
        })
    ));
}
