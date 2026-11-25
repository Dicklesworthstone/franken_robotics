//! Spark refusal-code registry tests: FROB-REF-601..618 (N21-REF).

use std::collections::BTreeMap;

use frob_core::refusal::{
    Refusal, RefusalCode, RefusalFingerprint, RefusalParseError, Repair, RetryPolicy,
};
use frob_core::registry::refusals::{
    DueSiteError, EMBEDDED_REFUSALS_TOML, Milestone, RaisingSiteRecord, RefusalRegistry,
    RefusalRegistryError, Spark601Trigger, Spark602SlabClass, render_refusal_code_enum,
};

const SPARK_ROWS: &[(u16, RetryPolicy, Milestone)] = &[
    (601, RetryPolicy::Backoff, Milestone::M0),
    (602, RetryPolicy::AfterChange, Milestone::M2),
    (603, RetryPolicy::AfterChange, Milestone::M0),
    (604, RetryPolicy::AfterChange, Milestone::M2),
    (605, RetryPolicy::AfterChange, Milestone::M2),
    (606, RetryPolicy::AfterChange, Milestone::M0),
    (607, RetryPolicy::Backoff, Milestone::M2),
    (608, RetryPolicy::AfterChange, Milestone::M2),
    (609, RetryPolicy::AfterChange, Milestone::M0),
    (610, RetryPolicy::AfterChange, Milestone::M2),
    (611, RetryPolicy::AfterChange, Milestone::M2),
    (612, RetryPolicy::AfterChange, Milestone::M2),
    (613, RetryPolicy::AfterChange, Milestone::M4),
    (614, RetryPolicy::AfterChange, Milestone::M2),
    (615, RetryPolicy::Backoff, Milestone::M2),
    (616, RetryPolicy::AfterChange, Milestone::M2),
    (617, RetryPolicy::AfterChange, Milestone::M0),
    (618, RetryPolicy::AfterChange, Milestone::M2),
];

#[test]
fn refusals_spark_rows_present() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml loads");
    reg.validate_row_metadata()
        .expect("all rows carry non-empty raised_by and repairs");
    for &(num, retry, due) in SPARK_ROWS {
        let code_str = format!("FROB-REF-{num}");
        let row = reg.lookup(&code_str, "N21-REF").expect(&code_str);
        let variant = RefusalCode::parse(&code_str).expect(&code_str);
        assert_eq!(row.defined_in, "§21.21", "{code_str} defined_in");
        assert_eq!(row.retry, retry, "{code_str} retry");
        assert_eq!(row.due_milestone(), Some(due), "{code_str} due_milestone");
        assert_eq!(variant.family(), 6, "{code_str} family");
        assert_eq!(variant.number(), num, "{code_str} number");
    }
    assert_eq!(SPARK_ROWS.len(), 18);
    assert_eq!(
        include_str!("../src/refusal/code_gen.rs"),
        render_refusal_code_enum(&reg)
    );
}

#[test]
fn refusals_spark_exit_code_6() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    for &(num, _, _) in SPARK_ROWS {
        let code_str = format!("FROB-REF-{num}");
        let variant = RefusalCode::parse(&code_str).unwrap();
        let row = reg.row(variant).expect(&code_str);
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

    // Structured per-trigger repair selection for 601 (device vs daemon).
    let r601 = reg.row(RefusalCode::FrobRef601).unwrap();
    let dev_repairs = r601
        .spark_601_repairs(Spark601Trigger::Device)
        .expect("601 device repairs");
    assert_eq!(dev_repairs.len(), 3);
    assert!(
        dev_repairs[0]
            .to_ascii_lowercase()
            .contains("another device")
    );
    let daemon_repairs = r601
        .spark_601_repairs(Spark601Trigger::Daemon)
        .expect("601 daemon repairs");
    assert_eq!(daemon_repairs.len(), 2);
    assert!(
        daemon_repairs[0]
            .to_ascii_lowercase()
            .contains("start frobd")
    );
    assert!(daemon_repairs[1].to_ascii_lowercase().contains("batch-cpu"));

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

    for class in [
        Spark602SlabClass::Worlds,
        Spark602SlabClass::Rollout,
        Spark602SlabClass::Frames,
        Spark602SlabClass::Adjoint,
    ] {
        let list = r602
            .spark_602_repairs(class)
            .unwrap_or_else(|| panic!("602 missing structured repairs for {class:?}"));
        assert_eq!(list.len(), 3, "602 {class:?} has 3 ranked repairs");
        assert!(
            list[0].to_ascii_lowercase().contains("fewer worlds"),
            "602 {class:?} repair[0] `{}` must start with fewer worlds",
            list[0]
        );
    }

    let scenes = r602
        .spark_602_repairs(Spark602SlabClass::Scenes)
        .expect("602 scenes repairs");
    assert_eq!(scenes.len(), 3);
    assert!(
        scenes[0]
            .to_ascii_lowercase()
            .contains("fewer distinct scene variants")
    );
    assert!(
        scenes
            .iter()
            .all(|s| !s.to_ascii_lowercase().contains("fewer worlds")),
        "602 scenes must never suggest fewer worlds: {scenes:?}"
    );

    let frames = r602
        .spark_602_repairs(Spark602SlabClass::Frames)
        .expect("602 frames repairs");
    for entry in &frames {
        let lower = entry.to_ascii_lowercase();
        assert!(
            !lower.contains("resolution"),
            "602 frames forbids resolution: {entry}"
        );
        assert!(
            !lower.contains("cadence"),
            "602 frames forbids cadence: {entry}"
        );
    }

    let adjoint = r602
        .spark_602_repairs(Spark602SlabClass::Adjoint)
        .expect("602 adjoint repairs");
    assert!(adjoint[0].to_ascii_lowercase().contains("fewer worlds"));
    assert!(
        adjoint[1]
            .to_ascii_lowercase()
            .contains("shorter horizon k")
    );
    assert!(adjoint[2].to_ascii_lowercase().contains("paired stencil"));
}

#[test]
fn refusals_6xx_carry_fingerprint_block() {
    let sample_digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    for &(num, retry, _) in SPARK_ROWS {
        let code = RefusalCode::parse(&format!("FROB-REF-{num}")).unwrap();
        let r = Refusal::new(
            code,
            format!("spark refusal {num}"),
            "/device/cuda:0",
            "obs",
            "req",
            "unit",
            retry,
            vec![Repair::new("frob spark check")],
        )
        .with_fingerprint(RefusalFingerprint::new("cuda:0", sample_digest));
        r.validate_family_contract()
            .expect("6xx with digest validates");
        let wire = r.to_json();
        assert!(
            wire.contains("\"fingerprint\":{\"device\":\"cuda:0\",\"digest\":\""),
            "{wire}"
        );
        assert_eq!(Refusal::from_json(&wire).expect("roundtrip"), r);
    }

    // Null digest is allowed specifically for FROB-REF-606 (missing fingerprint).
    let r606 = Refusal::new(
        RefusalCode::FrobRef606,
        "fingerprint missing",
        "/device/cuda:0/fingerprint",
        "null",
        "current fingerprint digest",
        "blake3-hex",
        RetryPolicy::AfterChange,
        vec![Repair::new("frob spark fingerprint")],
    )
    .with_fingerprint(RefusalFingerprint::missing("cuda:0"));
    r606.validate_family_contract()
        .expect("606 null digest allowed");
    let wire_606 = r606.to_json();
    assert!(wire_606.contains("\"fingerprint\":{\"device\":\"cuda:0\",\"digest\":null}"));
    assert_eq!(Refusal::from_json(&wire_606).expect("606 roundtrip"), r606);

    // Negative control 1: 6xx without fingerprint block is rejected both structurally and on parse.
    let missing_fp = Refusal::new(
        RefusalCode::FrobRef601,
        "device lost",
        "/device/cuda:0",
        "offline",
        "online",
        "device",
        RetryPolicy::Backoff,
        vec![Repair::new("Rerun on another device")],
    );
    assert_eq!(
        missing_fp.validate_family_contract(),
        Err(RefusalParseError::MissingField("fingerprint"))
    );
    assert_eq!(
        Refusal::from_json(&missing_fp.to_json()),
        Err(RefusalParseError::MissingField("fingerprint"))
    );

    // Negative control 2: null digest on any 6xx code other than 606 is rejected.
    for code in [
        RefusalCode::FrobRef601,
        RefusalCode::FrobRef603,
        RefusalCode::FrobRef613,
    ] {
        let bad_null = Refusal::new(
            code,
            "null digest forbidden",
            "/device/cuda:0",
            "obs",
            "req",
            "unit",
            RetryPolicy::AfterChange,
            vec![Repair::new("frob spark check")],
        )
        .with_fingerprint(RefusalFingerprint::missing("cuda:0"));
        assert!(matches!(
            bad_null.validate_family_contract(),
            Err(RefusalParseError::InvalidField {
                field: "fingerprint.digest",
                ..
            })
        ));
        assert!(matches!(
            Refusal::from_json(&bad_null.to_json()),
            Err(RefusalParseError::InvalidField {
                field: "fingerprint.digest",
                ..
            })
        ));
    }

    // Negative control 3: empty device on 606 is rejected.
    let empty_dev = Refusal::new(
        RefusalCode::FrobRef606,
        "missing",
        "/device",
        "null",
        "digest",
        "blake3-hex",
        RetryPolicy::AfterChange,
        vec![Repair::new("frob spark fingerprint")],
    )
    .with_fingerprint(RefusalFingerprint::missing(""));
    assert!(matches!(
        empty_dev.validate_family_contract(),
        Err(RefusalParseError::InvalidField {
            field: "fingerprint.device",
            ..
        })
    ));

    // Negative control 4: non-6xx refusal carrying a fingerprint block is rejected.
    let non_6xx_fp = Refusal::new(
        RefusalCode::FrobRef802,
        "bad arg",
        "/params/x",
        "x",
        "y",
        "",
        RetryPolicy::AfterChange,
        vec![Repair::new("fix")],
    )
    .with_fingerprint(RefusalFingerprint::new("cuda:0", sample_digest));
    assert!(matches!(
        non_6xx_fp.validate_family_contract(),
        Err(RefusalParseError::InvalidField {
            field: "fingerprint",
            ..
        })
    ));
}

#[test]
fn refusal_row_before_mechanism() {
    let reg = RefusalRegistry::load().expect("embedded registry loads");
    let r613 = reg
        .row(RefusalCode::FrobRef613)
        .expect("613 registered early");
    assert_eq!(r613.due_milestone(), Some(Milestone::M4));
    assert_eq!(r613.exit, 6);

    // Before M4 specialization lands, capability absence answers FROB-REF-724 (exit 5, retry never),
    // never 613.
    let r724 = reg.row(RefusalCode::FrobRef724).expect("724 registered");
    assert_eq!(r724.exit, 5);
    assert_eq!(r724.retry, RetryPolicy::Never);
    let cap_refusal = Refusal::new(
        RefusalCode::FrobRef724,
        "specialized kernel capability is not linked in this build",
        "/options/specialized",
        "specialized",
        "linked M4 specialized kernel support",
        "capability",
        RetryPolicy::Never,
        vec![Repair::new("frob capabilities --json")],
    );
    cap_refusal
        .validate_family_contract()
        .expect("724 is non-6xx and carries no fingerprint block");
    assert_ne!(cap_refusal.code, RefusalCode::FrobRef613);
}

#[test]
fn refusals_check_staged_by_milestone() {
    let synthetic = r#"schema = "frob.refusals/1"

[exit_rule]
"1xx" = 3
"2xx" = 3
"3xx" = 3
"4xx" = 3
"5xx" = 3
"6xx" = 6
"70x" = 4
"72x" = 5
"73x" = 5
"8xx" = 3
"9xx" = 9
conflict = []
integrity = []
capability = []

[[refusal]]
code = "FROB-REF-606"
meaning = "synthetic: fingerprint missing (due M0)"
defined_in = "§21.21"
exit = 6
retry = "after-change"
repairs = ["frob spark fingerprint"]
raised_by = ["N21c-05"]

[[refusal]]
code = "FROB-REF-613"
meaning = "synthetic: specialized kernel not bit-identical (due M4)"
defined_in = "§21.21"
exit = 6
retry = "after-change"
repairs = ["Use the generic kernel"]
raised_by = ["N21c-19"]
"#;
    let reg = RefusalRegistry::from_bytes(synthetic.as_bytes()).expect("synthetic registry loads");

    // 1. At M0, missing raising test for 606 (due M0) MUST fail.
    let mut sites_missing_606 = BTreeMap::new();
    sites_missing_606.insert(
        "FROB-REF-606".to_owned(),
        RaisingSiteRecord::missing(Milestone::M0),
    );
    sites_missing_606.insert(
        "FROB-REF-613".to_owned(),
        RaisingSiteRecord::missing(Milestone::M4),
    );
    assert_eq!(
        reg.check_due_raising_sites(Milestone::M0, &sites_missing_606),
        Err(DueSiteError::MissingRaisingTest {
            code: "FROB-REF-606".to_owned(),
            due: Milestone::M0,
            current: Milestone::M0,
        })
    );

    // 2. At M0, unexecuted raising test for 606 (due M0) MUST also fail.
    let mut sites_unexec_606 = BTreeMap::new();
    sites_unexec_606.insert(
        "FROB-REF-606".to_owned(),
        RaisingSiteRecord::unexecuted(Milestone::M0),
    );
    assert_eq!(
        reg.check_due_raising_sites(Milestone::M0, &sites_unexec_606),
        Err(DueSiteError::UnexecutedRaisingTest {
            code: "FROB-REF-606".to_owned(),
            due: Milestone::M0,
            current: Milestone::M0,
        })
    );

    // 3. At M0, 606 verified and 613 absent PASSES with 613 listed `pending (M4)`.
    let mut sites_613_pending = BTreeMap::new();
    sites_613_pending.insert(
        "FROB-REF-606".to_owned(),
        RaisingSiteRecord::verified(Milestone::M0),
    );
    sites_613_pending.insert(
        "FROB-REF-613".to_owned(),
        RaisingSiteRecord::missing(Milestone::M4),
    );
    let report = reg
        .check_due_raising_sites(Milestone::M0, &sites_613_pending)
        .expect("M0 passes when M4 code is absent");
    assert_eq!(report.verified, vec!["FROB-REF-606".to_owned()]);
    assert_eq!(
        report.pending_label("FROB-REF-613").as_deref(),
        Some("pending (M4)")
    );

    // 4. At M4, the exact same map (613 absent) MUST fail naming FROB-REF-613.
    assert_eq!(
        reg.check_due_raising_sites(Milestone::M4, &sites_613_pending),
        Err(DueSiteError::MissingRaisingTest {
            code: "FROB-REF-613".to_owned(),
            due: Milestone::M4,
            current: Milestone::M4,
        })
    );
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
