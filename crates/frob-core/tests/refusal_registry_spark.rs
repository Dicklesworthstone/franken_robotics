//! Spark refusal-code registry tests: FROB-REF-601..618 (N21-REF).

use frob_core::refusal::{RefusalCode, RetryPolicy};
use frob_core::registry::refusals::{
    EMBEDDED_REFUSALS_TOML, RefusalRegistry, RefusalRegistryError, render_refusal_code_enum,
};

const SPARK_CODES: &[(&str, RefusalCode, RetryPolicy)] = &[
    (
        "FROB-REF-601",
        RefusalCode::FrobRef601,
        RetryPolicy::Backoff,
    ),
    (
        "FROB-REF-602",
        RefusalCode::FrobRef602,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-603",
        RefusalCode::FrobRef603,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-604",
        RefusalCode::FrobRef604,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-605",
        RefusalCode::FrobRef605,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-606",
        RefusalCode::FrobRef606,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-607",
        RefusalCode::FrobRef607,
        RetryPolicy::Backoff,
    ),
    (
        "FROB-REF-608",
        RefusalCode::FrobRef608,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-609",
        RefusalCode::FrobRef609,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-610",
        RefusalCode::FrobRef610,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-611",
        RefusalCode::FrobRef611,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-612",
        RefusalCode::FrobRef612,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-613",
        RefusalCode::FrobRef613,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-614",
        RefusalCode::FrobRef614,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-615",
        RefusalCode::FrobRef615,
        RetryPolicy::Backoff,
    ),
    (
        "FROB-REF-616",
        RefusalCode::FrobRef616,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-617",
        RefusalCode::FrobRef617,
        RetryPolicy::AfterChange,
    ),
    (
        "FROB-REF-618",
        RefusalCode::FrobRef618,
        RetryPolicy::AfterChange,
    ),
];

#[test]
fn refusals_spark_rows_present() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml loads");
    reg.validate_row_metadata()
        .expect("all rows carry non-empty raised_by and repairs");
    let mut count = 0usize;
    for &(code_str, variant, retry) in SPARK_CODES {
        let row = reg.lookup(code_str, "N21-REF").expect(code_str);
        assert_eq!(row.defined_in, "§21.21", "{code_str} defined_in");
        assert_eq!(row.retry, retry, "{code_str} retry");
        assert_eq!(variant.family(), 6, "{code_str} family");
        assert_eq!(RefusalCode::parse(code_str), Some(variant));
        assert!(
            row.meaning
                .contains("fingerprint { device, digest | null }"),
            "{code_str} must declare fingerprint block"
        );
        count += 1;
    }
    assert_eq!(count, 18);
    assert_eq!(
        include_str!("../src/refusal/code_gen.rs"),
        render_refusal_code_enum(&reg)
    );
}

#[test]
fn refusals_spark_exit_code_6() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    for &(code_str, variant, _) in SPARK_CODES {
        let row = reg.row(variant).expect(code_str);
        assert_eq!(row.exit, 6, "{code_str} exit");
        assert_eq!(reg.exit_for(variant).unwrap(), 6);
    }
    // Negative control: mutating any 6xx row exit to 3 must fail at load time.
    let planted = EMBEDDED_REFUSALS_TOML.replacen(
        "defined_in = \"§21.21\"\nexit = 6\nretry = \"backoff\"",
        "defined_in = \"§21.21\"\nexit = 3\nretry = \"backoff\"",
        1,
    );
    assert_ne!(planted, EMBEDDED_REFUSALS_TOML);
    assert!(matches!(
        RefusalRegistry::from_bytes(planted.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch {
            observed: 3,
            expected: Some(6),
            ..
        })
    ));
}

#[test]
fn refusals_first_repairs() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    let check_first = |code: RefusalCode, needle: &str| {
        let r = reg.row(code).unwrap();
        assert!(
            r.repairs[0].to_ascii_lowercase().contains(needle),
            "{code} repair[0] `{}` missing `{needle}`",
            r.repairs[0]
        );
    };
    check_first(RefusalCode::FrobRef601, "another device");
    check_first(RefusalCode::FrobRef604, "mixed-allowed");
    check_first(RefusalCode::FrobRef605, "spark check");
    check_first(RefusalCode::FrobRef606, "spark fingerprint");
    check_first(
        RefusalCode::FrobRef607,
        "resume from the last committed epoch",
    );
    check_first(RefusalCode::FrobRef613, "generic kernel");
    check_first(RefusalCode::FrobRef618, "send the seed or recipe");

    // 601 daemon pointer repair mentions frobd first.
    let r601 = reg.row(RefusalCode::FrobRef601).unwrap();
    assert!(r601.meaning.contains("pointer: daemon"));
    assert!(
        r601.repairs
            .iter()
            .any(|s| s.contains("pointer: daemon") && s.contains("start frobd"))
    );

    // Review notes + acceptance raiser checks (607, 612, 614, 615, 618, 605<->864, 610->722).
    assert!(
        reg.row(RefusalCode::FrobRef607)
            .unwrap()
            .raised_by
            .iter()
            .any(|b| b == "N21b-07")
    );
    assert!(
        reg.row(RefusalCode::FrobRef607)
            .unwrap()
            .raised_by
            .iter()
            .any(|b| b == "BD-016")
    );
    assert!(
        reg.row(RefusalCode::FrobRef612)
            .unwrap()
            .raised_by
            .iter()
            .any(|b| b == "BD-015")
    );
    assert!(
        reg.row(RefusalCode::FrobRef612)
            .unwrap()
            .raised_by
            .iter()
            .any(|b| b == "N21b-12")
    );
    assert!(
        reg.row(RefusalCode::FrobRef614)
            .unwrap()
            .raised_by
            .iter()
            .any(|b| b == "N21b-11")
    );
    assert!(
        reg.row(RefusalCode::FrobRef615)
            .unwrap()
            .raised_by
            .iter()
            .any(|b| b == "N21b-24")
    );
    assert!(
        reg.row(RefusalCode::FrobRef618)
            .unwrap()
            .meaning
            .contains("16,777,216")
    );
    assert!(
        reg.row(RefusalCode::FrobRef605)
            .unwrap()
            .related
            .iter()
            .any(|c| c == "FROB-REF-864")
    );
    assert!(
        reg.row(RefusalCode::FrobRef610)
            .unwrap()
            .related
            .iter()
            .any(|c| c == "FROB-REF-722")
    );
}

#[test]
fn refusals_602_repairs_by_slab_class() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    let r602 = reg.row(RefusalCode::FrobRef602).unwrap();
    assert!(r602.raised_by.iter().any(|b| b == "N16c-26"));

    let entry_for = |class: &str| -> &str {
        let prefix = format!("{class}:");
        r602.repairs
            .iter()
            .find(|s| s.starts_with(&prefix))
            .map(String::as_str)
            .unwrap_or_else(|| panic!("602 missing repair list for slab class `{class}`"))
    };

    for class in ["worlds", "rollout", "frames", "adjoint"] {
        assert!(
            entry_for(class)
                .to_ascii_lowercase()
                .contains("1. fewer worlds"),
            "602 `{class}` must start with Fewer worlds"
        );
    }

    let scenes = entry_for("scenes").to_ascii_lowercase();
    assert!(scenes.contains("1. fewer distinct scene variants"));
    assert!(
        !scenes.contains("fewer worlds"),
        "602 scenes must never suggest fewer worlds"
    );

    let frames = entry_for("frames").to_ascii_lowercase();
    assert!(
        !frames.contains("resolution"),
        "602 frames must not suggest resolution change"
    );
    assert!(
        !frames.contains("cadence"),
        "602 frames must not suggest cadence change"
    );

    let adjoint = entry_for("adjoint").to_ascii_lowercase();
    let i_worlds = adjoint.find("fewer worlds").expect("adjoint fewer worlds");
    let i_horizon = adjoint
        .find("shorter horizon k")
        .expect("adjoint horizon K");
    let i_stencil = adjoint
        .find("paired stencil")
        .expect("adjoint paired stencil");
    assert!(i_worlds < i_horizon && i_horizon < i_stencil);
}

#[test]
fn refusals_spark_negative_controls() {
    // Empty repairs must fail validate_row_metadata.
    let no_repairs =
        EMBEDDED_REFUSALS_TOML.replacen("repairs = [\"frob spark check\"]", "repairs = []", 1);
    assert_ne!(no_repairs, EMBEDDED_REFUSALS_TOML);
    let reg = RefusalRegistry::from_bytes(no_repairs.as_bytes()).expect("parses");
    assert!(matches!(
        reg.validate_row_metadata(),
        Err(RefusalRegistryError::EmptyRepairs { ref code }) if code == "FROB-REF-605"
    ));

    // Duplicate 6xx code must fail at load time.
    let dup = format!(
        "{EMBEDDED_REFUSALS_TOML}\n[[refusal]]\ncode = \"FROB-REF-601\"\nmeaning = \"dup\"\ndefined_in = \"§21.21\"\nexit = 6\nretry = \"backoff\"\nrepairs = [\"r\"]\nraised_by = [\"N21-REF\"]\n"
    );
    assert!(matches!(
        RefusalRegistry::from_bytes(dup.as_bytes()),
        Err(RefusalRegistryError::DuplicateRefusalCode { ref code, .. }) if code == "FROB-REF-601"
    ));

    // Unknown 6xx code referenced by a verb must fail at load time.
    let verbs = b"schema = \"frob.verbs/1\"\n[[verb]]\nname = \"spark.probe\"\nrefusals = [\"FROB-REF-699\"]\n";
    assert!(matches!(
        RefusalRegistry::load_with_verbs(EMBEDDED_REFUSALS_TOML.as_bytes(), verbs),
        Err(RefusalRegistryError::UnknownRefusalCode { ref code, .. }) if code == "FROB-REF-699"
    ));
}
