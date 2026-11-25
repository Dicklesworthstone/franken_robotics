//! Unit tests for the README honest-status table generator, first-commit
//! registries, and demo/contract schemas (bead BD-002).
//!
//! Run with: `cargo test -p xtask status_table`
//! Bless README block with: `FROB_BLESS=1 cargo test -p xtask readme_block_is_current`

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;

use xtask::status_table::{
    ClaimKind, GatePart, GateRegistry, GateRow, PartStatus, ReadmeCheckOutcome, RecordedMeta,
    STATUS_BEGIN, STATUS_END, check_or_bless_readme, generate_from_toml, parse_formats_toml,
    parse_gates_toml, split_readme_status_block, status_table, try_status_table,
    validate_all_registries, validate_contract_toml, validate_demo_yaml,
};

fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("crates/xtask must live two levels below repo root")
        .to_path_buf()
}

fn sample_gate(id: &str, claim: ClaimKind, controls: Vec<&str>, parts: Vec<GatePart>) -> GateRow {
    GateRow {
        id: id.to_owned(),
        claim,
        claim_id: format!("clm:{id}"),
        section: "38.13".to_owned(),
        threshold: "target".to_owned(),
        milestone: "M1".to_owned(),
        tier: vec!["G1:host".to_owned()],
        controls: controls.into_iter().map(str::to_owned).collect(),
        parts,
    }
}

fn sample_part(
    milestone: &str,
    status: PartStatus,
    measured: &str,
    session: &str,
    implemented_by: &str,
) -> GatePart {
    GatePart {
        milestone: milestone.to_owned(),
        status,
        measured: measured.to_owned(),
        evidence: vec!["seq:1".to_owned()],
        recorded: RecordedMeta {
            date: "2026-11-24".to_owned(),
            session: session.to_owned(),
            implemented_by: implemented_by.to_owned(),
        },
        kill_gate: None,
        fallback: None,
        open_question: None,
        detail: None,
    }
}

#[test]
fn empty_registry_prints_header_and_zero_counts() {
    let toml = b"schema = \"frob.gates/1\"\n";
    let table = generate_from_toml(toml).expect("empty gates.toml must generate cleanly");
    assert!(table.contains("| Gate"), "missing header: {table}");
    assert!(table.contains("| Recorded"), "missing Recorded column: {table}");
    assert!(
        table.contains("Summary: 0 gates (open: 0, pass: 0, fail: 0, killed: 0, blocked: 0, no-claim: 0, superseded: 0)\n"),
        "unexpected summary line: {table}"
    );
    // Exactly 2 table lines (header + separator), 1 blank line, 1 summary line = 4 lines.
    assert_eq!(table.lines().count(), 4, "unexpected line count in:\n{table}");
}

#[test]
fn not_run_prints_open() {
    let reg = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "GATE-ARENA-05",
            ClaimKind::KnownAnswer,
            vec!["NC-0398"],
            vec![sample_part("M1", PartStatus::NotRun, "", "", "")],
        )],
    };
    let rendered = status_table(&reg).expect("not-run gate must render");
    let data_line = rendered.lines().nth(2).expect("missing data row");
    assert!(data_line.contains("GATE-ARENA-05"), "{data_line}");
    assert!(data_line.contains("| open "), "not-run must print open: {data_line}");
    assert!(rendered.contains("Summary: 1 gates (open: 1,"), "{rendered}");
}

#[test]
fn pass_without_measured_is_error() {
    let toml = r#"
schema = "frob.gates/1"

[[gate]]
id = "GATE-EMPTY-MEAS"
claim = "known-answer"
claim_id = "clm:empty-meas"
section = "38.13"
threshold = "exact"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = ""
evidence = ["seq:1"]
recorded = { date = "2026-11-24", session = "session:c", implemented_by = "session:a" }
"#;
    let err = generate_from_toml(toml.as_bytes())
        .expect_err("pass with empty measured must fail generation");
    assert!(
        err.to_string().contains("GATE-EMPTY-MEAS"),
        "error must name the offending gate id: {err}"
    );

    // Also verify try_status_table rejects a directly constructed struct.
    let reg = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "GATE-DIRECT-EMPTY",
            ClaimKind::KnownAnswer,
            vec!["NC-0001"],
            vec![sample_part("M1", PartStatus::Pass, "", "session:c", "session:a")],
        )],
    };
    let err2 = try_status_table(&reg).expect_err("direct pass without measured must fail");
    assert!(err2.to_string().contains("GATE-DIRECT-EMPTY"), "{err2}");
}

#[test]
fn pass_with_measured_prints_value() {
    let reg = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "GATE-CELLS-01",
            ClaimKind::KnownAnswer,
            vec!["NC-0001"],
            vec![sample_part(
                "M1",
                PartStatus::Pass,
                "561 of 561 cells",
                "session:c",
                "session:a",
            )],
        )],
    };
    let rendered = status_table(&reg).expect("not-run gate must render");
    let data_line = rendered.lines().nth(2).expect("missing data row");
    assert!(data_line.contains("| pass "), "{data_line}");
    assert!(data_line.contains("561 of 561 cells"), "{data_line}");
    assert!(rendered.contains("pass: 1"), "{rendered}");
}

#[test]
fn killed_prints_kill_gate_and_fallback() {
    let mut part = sample_part("M1", PartStatus::Killed, "", "session:c", "session:a");
    part.kill_gate = Some("KILL-FSIM".to_owned());
    part.fallback = Some("pinned worker process".to_owned());

    let reg = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "INT-FSIM-01",
            ClaimKind::Parity,
            vec!["NC-0001"],
            vec![part],
        )],
    };
    let rendered = status_table(&reg).expect("not-run gate must render");
    let data_line = rendered.lines().nth(2).expect("missing data row");
    assert!(data_line.contains("killed"), "{data_line}");
    assert!(data_line.contains("KILL-FSIM"), "{data_line}");
    assert!(data_line.contains("pinned worker process"), "{data_line}");
    assert!(rendered.contains("killed: 1"), "{rendered}");
}

#[test]
fn no_claim_is_terminal() {
    let reg = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "GATE-REAL-01",
            ClaimKind::Reality,
            vec![],
            vec![sample_part("M5", PartStatus::NoClaim, "", "session:c", "")],
        )],
    };
    let rendered = status_table(&reg).expect("not-run gate must render");
    let data_line = rendered.lines().nth(2).expect("missing data row");
    assert!(data_line.contains("| no-claim "), "{data_line}");
    assert!(rendered.contains("no-claim: 1"), "{rendered}");

    // Negative control: no-claim on a non-Reality gate fails schema validation.
    let bad = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "GATE-NONREAL-01",
            ClaimKind::KnownAnswer,
            vec![],
            vec![sample_part("M1", PartStatus::NoClaim, "", "session:c", "")],
        )],
    };
    let err = try_status_table(&bad).expect_err("no-claim on non-reality gate must fail");
    assert!(err.to_string().contains("GATE-NONREAL-01"), "{err}");
}

#[test]
fn generation_is_deterministic() {
    let g_a = sample_gate(
        "GATE-A-01",
        ClaimKind::Proof,
        vec!["NC-01"],
        vec![sample_part("M0", PartStatus::Pass, "0 violations", "session:r", "session:a")],
    );
    let g_b = sample_gate(
        "GATE-B-02",
        ClaimKind::Fidelity,
        vec!["NC-02"],
        vec![sample_part("M1", PartStatus::NotRun, "", "", "")],
    );
    let g_c = sample_gate(
        "GATE-C-03",
        ClaimKind::Reality,
        vec![],
        vec![sample_part("M5", PartStatus::NoClaim, "", "session:r", "")],
    );

    let r1 = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![g_a.clone(), g_b.clone(), g_c.clone()],
    };
    let r2 = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![g_c, g_a, g_b],
    };

    let out1 = status_table(&r1).expect("r1 must render");
    let out2 = status_table(&r2).expect("r2 must render");
    assert_eq!(out1.as_bytes(), out2.as_bytes());
    assert!(!out1.contains('\r'), "must use LF endings only");
}

#[test]
fn readme_block_is_current() {
    let root = env::var_os("FROB_STATUS_ROOT")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(repo_root);
    let readme_path = root.join("README.md");
    let gates_path = root.join("registries/gates.toml");
    let bless = env::var("FROB_BLESS").as_deref() == Ok("1");

    let before_text = fs::read_to_string(&readme_path).expect("README.md must exist");
    let (before_pre, _, before_post, _) =
        split_readme_status_block(&before_text).expect("README.md must have status markers");
    let expected_outside = format!("{before_pre}{before_post}");

    let outcome = check_or_bless_readme(&readme_path, &gates_path, bless)
        .unwrap_or_else(|e| panic!("README status block check failed: {e}"));

    let after_text = fs::read_to_string(&readme_path).expect("README.md must still exist");
    let (after_pre, after_inner, after_post, _) =
        split_readme_status_block(&after_text).expect("README.md must keep status markers");
    let observed_outside = format!("{after_pre}{after_post}");
    assert_eq!(
        expected_outside.as_bytes(),
        observed_outside.as_bytes(),
        "bytes outside {STATUS_BEGIN}..{STATUS_END} must never change"
    );

    let generated = generate_from_toml(&fs::read(&gates_path).expect("read gates.toml"))
        .expect("generate_from_toml must succeed");
    assert_eq!(after_inner, generated);
    match outcome {
        ReadmeCheckOutcome::Current { .. } => assert!(!bless),
        ReadmeCheckOutcome::Blessed { .. } => assert!(bless),
    }
}

#[test]
fn readme_hand_edit_fails_naming_first_diff_line() {
    let scratch = env::temp_dir().join(format!("frob-bd002-handedit-{}", std::process::id()));
    fs::create_dir_all(&scratch).expect("create scratch dir");
    let gates_path = scratch.join("gates.toml");
    let readme_path = scratch.join("README.md");

    fs::write(&gates_path, b"schema = \"frob.gates/1\"\n").expect("write gates");
    let good_block = generate_from_toml(b"schema = \"frob.gates/1\"\n").expect("gen");
    let tampered_block = good_block.replace("Summary: 0 gates", "Summary: 999 gates");
    let readme = format!(
        "# Header\n\n{STATUS_BEGIN}\n{tampered_block}{STATUS_END}\n\nFooter\n"
    );
    fs::write(&readme_path, &readme).expect("write tampered readme");

    let err = check_or_bless_readme(&readme_path, &gates_path, false)
        .expect_err("hand-edited README block must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("line 7"),
        "error must name the first differing line (line 7), got: {msg}"
    );
}

#[test]
fn formats_registry_has_thirteen_unique_rows() {
    let root = repo_root();
    let raw = fs::read(root.join("registries/formats.toml")).expect("read formats.toml");
    let reg = parse_formats_toml(&raw).expect("formats.toml must validate");
    assert_eq!(reg.formats.len(), 13, "expected exactly 13 format rows");

    let ids: BTreeSet<&str> = reg.formats.iter().map(|r| r.id.as_str()).collect();
    let exts: BTreeSet<&str> = reg.formats.iter().map(|r| r.extension.as_str()).collect();
    let schemas: BTreeSet<&str> = reg.formats.iter().map(|r| r.schema.as_str()).collect();
    assert_eq!(ids.len(), 13);
    assert_eq!(exts.len(), 13);
    assert_eq!(schemas.len(), 13);

    // Negative control: a fourteenth row with a duplicate extension must fail.
    let mut dup = String::from_utf8(raw).expect("UTF-8 formats.toml");
    dup.push_str(
        r#"
[[format]]
id = "FMT-FEXTRA"
extension = ".fbot"
schema = "frob.extra/1"
owner = "frob-design"
producers = ["frob-design"]
consumers = ["frob-design"]
encoding = "canonical"
"#,
    );
    let err = parse_formats_toml(dup.as_bytes())
        .expect_err("14th row with duplicate extension must fail");
    assert!(err.to_string().contains(".fbot"), "{err}");
}

#[test]
fn every_registry_validates() {
    let root = repo_root();
    let audit = validate_all_registries(&root).expect("all registries and schemas must validate");
    assert_eq!(audit.format_count, 13);
    assert!(audit.validated_files.contains(&"registries/gates.toml".to_owned()));
    assert!(
        audit
            .validated_files
            .contains(&"registries/verbs.toml".to_owned())
    );
    assert!(audit.validated_files.contains(&"registries/cards".to_owned()));
    assert!(audit.validated_files.len() >= 20);

    // Verify existing crate contract.toml also validates against contract schema rules.
    let core_contract = root.join("crates/frob-core/contract.toml");
    if core_contract.is_file() {
        let bytes = fs::read(&core_contract).expect("read crates/frob-core/contract.toml");
        validate_contract_toml(&bytes).expect("crates/frob-core/contract.toml must validate");
    }
}

const MEASURED_GATE: &str = r#"
schema = "frob.gates/1"

[[gate]]
id = "GATE-MEASURED"
claim = "known-answer"
claim_id = "clm:measured"
section = "38.13"
threshold = "561 cells"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "561 of 561 cells"
evidence = ["seq:1"]
recorded = { date = "2026-11-24", session = "session:reviewer", implemented_by = "session:author" }
"#;

#[test]
fn readme_pass_refuses_copied_threshold() {
    let valid = parse_gates_toml(MEASURED_GATE.as_bytes()).unwrap();
    assert!(status_table(&valid).unwrap().contains("561 of 561 cells"));
    let planted = MEASURED_GATE.replace("561 of 561 cells", "561 cells");
    let err = generate_from_toml(planted.as_bytes()).expect_err("copied TARGET must fail");
    assert!(err.context.contains("GATE-MEASURED"), "{err}");
    assert!(err.message.contains("byte-equal"), "{err}");
}

#[test]
fn gate_projection_accepts_registry_extensions() {
    let extended = MEASURED_GATE
        .replace("claim =", "kind = \"program\"\nclaim =")
        .replace("status =", "scope = \"host\"\nstatus =");
    let extended =
        format!("{extended}\n[[gate]]\nid = \"KILL-TEST\"\nkind = \"kill\"\nsection = \"38.13\"\n");
    let table = generate_from_toml(extended.as_bytes()).expect("gate metadata must render");
    assert_eq!(table, generate_from_toml(MEASURED_GATE.as_bytes()).unwrap());
    let planted = extended.replace("scope =", "scp =");
    let err = generate_from_toml(planted.as_bytes()).expect_err("unknown part key must fail");
    assert!(err.message.contains("scp"), "{err}");
}

#[test]
fn self_certified_pass_fails_schema() {
    let make_toml = |session: &str, implemented_by: &str| {
        format!(
            r#"
schema = "frob.gates/1"

[[gate]]
id = "GATE-SELF-01"
claim = "known-answer"
claim_id = "clm:self-01"
section = "38.13"
threshold = "exact"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "1.0e-12 rad"
evidence = ["seq:1"]
recorded = {{ date = "2026-11-24", session = "{session}", implemented_by = "{implemented_by}" }}
"#
        )
    };

    // Co-implementer must fail.
    let co_impl = make_toml("session:b", "session:a,session:b");
    let err1 = parse_gates_toml(co_impl.as_bytes())
        .expect_err("co-implementer self-certification must fail");
    assert!(err1.to_string().contains("GATE-SELF-01"), "{err1}");

    // Sole implementer must fail.
    let sole_impl = make_toml("session:b", "session:b");
    let err2 = parse_gates_toml(sole_impl.as_bytes())
        .expect_err("sole-implementer self-certification must fail");
    assert!(err2.to_string().contains("GATE-SELF-01"), "{err2}");

    // Independent session:c must validate.
    let indep = make_toml("session:c", "session:a,session:b");
    let ok = parse_gates_toml(indep.as_bytes()).expect("independent session:c must validate");
    assert_eq!(ok.gates.len(), 1);
}

#[test]
fn pass_with_no_controls_fails_schema() {
    let toml = r#"
schema = "frob.gates/1"

[[gate]]
id = "GATE-NOCTRL-01"
claim = "known-answer"
claim_id = "clm:noctrl-01"
section = "38.13"
threshold = "exact"
milestone = "M1"
tier = ["G1:host"]
controls = []

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "0.0 error"
evidence = ["seq:1"]
recorded = { date = "2026-11-24", session = "session:c", implemented_by = "session:a" }
"#;
    let err =
        parse_gates_toml(toml.as_bytes()).expect_err("pass with empty controls must fail");
    assert!(err.to_string().contains("GATE-NOCTRL-01"), "{err}");
}

#[test]
fn one_row_per_gate_from_parts() {
    // 1. Gate with zero parts prints open.
    let g0 = sample_gate("GATE-00-NOPART", ClaimKind::Proof, vec!["NC-1"], vec![]);
    // 2. Gate with one valid M1 pass part prints pass with measured value.
    let g1 = sample_gate(
        "GATE-01-ONEPART",
        ClaimKind::KnownAnswer,
        vec!["NC-2"],
        vec![sample_part("M1", PartStatus::Pass, "0.001 m", "session:c", "session:a")],
    );
    // 3. Gate with two parts (M0 pass + M2 not-run) -> latest due part is M2 (open).
    // Plus another gate with two parts (M0 pass + M1 pass) -> total 3 gates holding 5 parts.
    let g2 = sample_gate(
        "GATE-02-TWOPARTS",
        ClaimKind::Parity,
        vec!["NC-3"],
        vec![
            sample_part("M0", PartStatus::Pass, "1.0x", "session:c", "session:a"),
            sample_part("M2", PartStatus::Pass, "2.5x", "session:d", "session:a"),
        ],
    );
    let g3 = sample_gate(
        "GATE-03-TWOPARTS",
        ClaimKind::Fidelity,
        vec!["NC-4"],
        vec![
            sample_part("M0", PartStatus::Pass, "1e-6", "session:c", "session:a"),
            sample_part("M1", PartStatus::NotRun, "", "", ""),
        ],
    );

    let r_nopart = status_table(&GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![g0],
    })
    .expect("zero-part gate must render");
    assert!(r_nopart.lines().nth(2).unwrap().contains("| open "));

    let r_onepart = status_table(&GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![g1.clone()],
    })
    .expect("one-part gate must render");
    let line1 = r_onepart.lines().nth(2).unwrap();
    assert!(line1.contains("| pass ") && line1.contains("0.001 m"), "{line1}");

    // Three gates holding 1 + 2 + 2 = 5 parts must print exactly 3 rows.
    let three_gates = GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![g1, g2, g3],
    };
    let total_parts: usize = three_gates.gates.iter().map(|g| g.parts.len()).sum();
    assert_eq!(total_parts, 5);
    let rendered = status_table(&three_gates).expect("three-gate fixture must render");
    // Header + sep + 3 data rows + blank + summary = 7 lines.
    assert_eq!(rendered.lines().count(), 7, "{rendered}");
    assert!(rendered.contains("Summary: 3 gates (open: 1, pass: 2,"), "{rendered}");
    assert!(rendered.contains("2.5x"), "{rendered}");
}

fn demo_gates_fixture() -> GateRegistry {
    GateRegistry {
        schema: "frob.gates/1".to_owned(),
        gates: vec![sample_gate(
            "GATE-DEMO-01",
            ClaimKind::KnownAnswer,
            vec!["NC-001"],
            vec![],
        )],
    }
}

fn ns01_full_demo_yaml() -> &'static str {
    r#"
demo_id: "ns-01"
claim_ids:
  - "clm:GATE-DEMO-01"
commands:
  - "frob design check quad-a --json"
  - "frob sim run quad-a --tier batch-cpu --json"
checksums:
  "frob design check quad-a --json": "b3:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
tier: "cpu"
expect_exit:
  "frob design check quad-a --json": 0
  "frob sim run quad-a --tier batch-cpu --json": 3
expect_refusal:
  "frob sim run quad-a --tier batch-cpu --json": "FROB-REF-802"
compare:
  "frob design check quad-a --json":
    - "/result/mass_kg"
expect:
  "frob design check quad-a --json": "expected/check.json"
title: "North-Star 01 quadruped check"
milestone: "M0"
actors:
  - "design-agent"
lane: "cpu"
fresh_agent: true
not_evidence_for: "numbers"
"#
}

#[test]
fn demo_schema_admits_every_optional_key() {
    let gates = demo_gates_fixture();
    let doc = validate_demo_yaml(ns01_full_demo_yaml(), &gates)
        .expect("demo.yaml using all 11 optional keys must validate");
    assert_eq!(doc.demo_id, "ns-01");
    assert_eq!(doc.tier.as_deref(), Some("cpu"));
    assert_eq!(doc.lane.as_deref(), Some("cpu"));
    assert_eq!(doc.fresh_agent, Some(true));
    assert_eq!(doc.not_evidence_for.as_deref(), Some("numbers"));
    assert_eq!(doc.milestone.as_deref(), Some("M0"));
    assert_eq!(doc.actors, vec!["design-agent".to_owned()]);
    assert_eq!(doc.expect_exit.len(), 2);
    assert_eq!(doc.expect_refusal.len(), 1);
    assert_eq!(doc.compare.len(), 1);
    assert_eq!(doc.expect.len(), 1);
}

#[test]
fn demo_schema_refuses_unknown_key() {
    let gates = demo_gates_fixture();
    let mut bad = ns01_full_demo_yaml().to_owned();
    bad.push_str("colour: \"red\"\n");
    let err = validate_demo_yaml(&bad, &gates).expect_err("unknown key `colour` must fail");
    assert!(err.to_string().contains("colour"), "error must name `colour`: {err}");
}

#[test]
fn demo_schema_requires_checksums() {
    let gates = demo_gates_fixture();
    let bad = r#"
demo_id: "ns-01"
claim_ids:
  - "clm:GATE-DEMO-01"
commands:
  - "frob design check quad-a --json"
"#;
    let err = validate_demo_yaml(bad, &gates).expect_err("missing `checksums` must fail");
    assert!(err.to_string().contains("checksums"), "{err}");
}

#[test]
fn demo_lane_enum() {
    let gates = demo_gates_fixture();
    let bad = ns01_full_demo_yaml().replace("lane: \"cpu\"", "lane: \"gpu\"");
    let err = validate_demo_yaml(&bad, &gates).expect_err("`lane: gpu` must fail");
    assert!(err.to_string().contains("lane"), "{err}");
}

#[test]
fn demo_absent_claim_id_fails_validation() {
    let gates = demo_gates_fixture();
    let bad = ns01_full_demo_yaml().replace("clm:GATE-DEMO-01", "clm:DOES-NOT-EXIST");
    let err = validate_demo_yaml(&bad, &gates)
        .expect_err("claim_id absent from gates.toml must fail");
    assert!(err.to_string().contains("clm:DOES-NOT-EXIST"), "{err}");
}

#[test]
fn first_commit_documents_and_admission_header() {
    let root = repo_root();
    for rel in [
        "AGENTS.md",
        "README.md",
        "DEPENDENCY_ADMISSION.md",
        "schemas/demo.schema.json",
        "schemas/contract.schema.json",
        "registries/verbs.toml",
        "registries/refusals.toml",
        "registries/kernels.toml",
        "registries/gates.toml",
        "registries/formats.toml",
        "registries/upstream.toml",
        "registries/lifts.toml",
        "registries/capsules.toml",
    ] {
        assert!(root.join(rel).is_file(), "missing required BD-002 file `{rel}`");
    }
    assert!(root.join("registries/cards").is_dir(), "missing registries/cards/");

    let agents = fs::read_to_string(root.join("AGENTS.md")).expect("read AGENTS.md");
    for needle in [
        "rch",
        "~/projects",
        "${RCH_TARGET_BASE",
        "16 GiB",
        "ubs",
        "br",
        "bv --robot-*",
        "xtask check-stack",
    ] {
        assert!(agents.contains(needle), "AGENTS.md missing working rule `{needle}`");
    }

    let adm = fs::read_to_string(root.join("DEPENDENCY_ADMISSION.md"))
        .expect("read DEPENDENCY_ADMISSION.md");
    for needle in [
        "Validity rule",
        "Cargo.lock",
        "receipt hash",
        "One record per admission gate run, passing or failing",
        "Fields shown by each record",
        "TARGET",
        "Never inherited",
    ] {
        assert!(
            adm.contains(needle),
            "DEPENDENCY_ADMISSION.md missing required header rule `{needle}`"
        );
    }
    let records_dir = root.join("receipts/admissions");
    if records_dir.is_dir() {
        let json_count = fs::read_dir(&records_dir)
            .expect("read receipts/admissions")
            .flatten()
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .count();
        assert_eq!(json_count, 0, "expected zero committed admission records at M0");
    }
}
