//! Unit tests of bead N32a-03 (`cargo test -p frob-verbs envelope`).
//!
//! Most tests load `MIRROR`, a synthetic registry that carries the eighteen Tether codes and
//! `FROB-REF-990` under N35-33's `[exit_rule]` through frob-core's own loader, so the
//! envelope's own rules are tested apart from the registry's content. The same exit
//! assertions against the embedded `registries/refusals.toml` (whose rows N32-REF enters)
//! are `exit_table_exact_real_registry`. Golden files are stamped with a fixed synthetic stack receipt so
//! their bytes do not move with `receipts/HEAD`; `stack_receipt_on_every_envelope` checks
//! the production stamp.

use std::fs;
use std::path::Path;
use std::sync::Once;

use frob_core::auth::{NoAuthorization, PacketAuthorization};
use frob_core::evidence::{
    ClaimKind, ClaimProduct, DetClass, FlagSet, InputRef, RealityScope, Scope, TierMeet,
};
use frob_core::id::Blake3;
use frob_core::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};
use frob_core::registry::refusals::{RefusalRegistry, RefusalRegistryError};
use frob_core::registry::toml::{self, Fields};
use frob_core::stack::{BUILT_UNDER, StackIdentity, StackLockDigest, StackReceiptHash};
use frob_core::testkit::prop::{Gen, check};

use crate::registry::{Kind, Milestone, SchemaId, Scope as VerbScope, Tier, VerbEntry, lookup_in};

use super::*;

// ---------------------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------------------

/// Synthetic refusal registry: N35-33's `[exit_rule]` with its 7xx decades and full exception
/// lists, the eighteen Tether codes and 990 (N32-REF's rows), and 112 and 840, the exception-list
/// codes whose real rows other REF beads enter.
const MIRROR: &str = r#"# synthetic: mirror of N32-REF rows under N35-33's [exit_rule] (N32a-03 tests only)
schema = "frob.refusals/1"

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
"74x" = 4
"76x" = 4
"79x" = 5
"8xx" = 3
"9xx" = 9
conflict = [112, 803, 804, 809]
integrity = [810]
capability = [840]

[[refusal]]
code = "FROB-REF-112"
meaning = "synthetic: stale parent"
defined_in = "§32.4"
exit = 7
retry = "after-change"

[[refusal]]
code = "FROB-REF-701"
meaning = "synthetic: budget refused at admission"
defined_in = "§32.4"
exit = 4
retry = "after-change"

[[refusal]]
code = "FROB-REF-702"
meaning = "synthetic: budget stop"
defined_in = "§32.4"
exit = 4
retry = "after-change"

[[refusal]]
code = "FROB-REF-703"
meaning = "synthetic: budget"
defined_in = "§32.4"
exit = 4
retry = "after-change"

[[refusal]]
code = "FROB-REF-704"
meaning = "synthetic: budget"
defined_in = "§32.4"
exit = 4
retry = "after-change"

[[refusal]]
code = "FROB-REF-720"
meaning = "synthetic: outside the token's scopes"
defined_in = "§32.4"
exit = 5
retry = "after-change"

[[refusal]]
code = "FROB-REF-723"
meaning = "synthetic: capability"
defined_in = "§32.4"
exit = 5
retry = "after-change"

[[refusal]]
code = "FROB-REF-724"
meaning = "synthetic: capability"
defined_in = "§32.4"
exit = 5
retry = "after-change"

[[refusal]]
code = "FROB-REF-725"
meaning = "synthetic: capability"
defined_in = "§32.4"
exit = 5
retry = "after-change"

[[refusal]]
code = "FROB-REF-801"
meaning = "synthetic: unknown verb"
defined_in = "§32.18"
exit = 3
retry = "after-change"

[[refusal]]
code = "FROB-REF-802"
meaning = "synthetic: arguments fail the schema"
defined_in = "§32.18"
exit = 3
retry = "after-change"

[[refusal]]
code = "FROB-REF-803"
meaning = "synthetic: idempotency key reuse with different content"
defined_in = "§32.4"
exit = 7
retry = "never"

[[refusal]]
code = "FROB-REF-804"
meaning = "synthetic: conflict"
defined_in = "§32.4"
exit = 7
retry = "never"

[[refusal]]
code = "FROB-REF-805"
meaning = "synthetic: refused"
defined_in = "§32.4"
exit = 3
retry = "after-change"

[[refusal]]
code = "FROB-REF-806"
meaning = "synthetic: refused"
defined_in = "§32.4"
exit = 3
retry = "after-change"

[[refusal]]
code = "FROB-REF-807"
meaning = "synthetic: refused"
defined_in = "§32.4"
exit = 3
retry = "after-change"

[[refusal]]
code = "FROB-REF-808"
meaning = "synthetic: refused"
defined_in = "§32.4"
exit = 3
retry = "after-change"

[[refusal]]
code = "FROB-REF-809"
meaning = "synthetic: lineage conflict"
defined_in = "§32.4"
exit = 7
retry = "never"

[[refusal]]
code = "FROB-REF-810"
meaning = "synthetic: integrity fault"
defined_in = "§32.4"
exit = 9
retry = "never"

[[refusal]]
code = "FROB-REF-840"
meaning = "synthetic: capability exception"
defined_in = "§32.4"
exit = 5
retry = "after-change"

[[refusal]]
code = "FROB-REF-990"
meaning = "synthetic: internal fault"
defined_in = "§32.4"
exit = 9
retry = "never"
"#;

/// The eighteen codes of the chapter and the internal fault, with the exits the bead lists.
/// Written independently of `MIRROR` so that a wrong mirror row cannot pass unnoticed.
const EXPECTED: &[(&str, u8)] = &[
    ("FROB-REF-701", 4),
    ("FROB-REF-702", 4),
    ("FROB-REF-703", 4),
    ("FROB-REF-704", 4),
    ("FROB-REF-720", 5),
    ("FROB-REF-723", 5),
    ("FROB-REF-724", 5),
    ("FROB-REF-725", 5),
    ("FROB-REF-801", 3),
    ("FROB-REF-802", 3),
    ("FROB-REF-803", 7),
    ("FROB-REF-804", 7),
    ("FROB-REF-805", 3),
    ("FROB-REF-806", 3),
    ("FROB-REF-807", 3),
    ("FROB-REF-808", 3),
    ("FROB-REF-809", 7),
    ("FROB-REF-810", 9),
    ("FROB-REF-990", 9),
];

const EXIT_TABLE_FIXTURE: &str =
    include_str!("../../../frob-conformance/fixtures/tether/exit_table.toml");

const SYNTH_RECEIPT_HEX: &str = "5ca1ab1e5ca1ab1e5ca1ab1e5ca1ab1e5ca1ab1e5ca1ab1e5ca1ab1e5ca1ab1e";

const fn synth_receipt() -> StackReceiptHash {
    let mut b = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        b[i] = [0x5c, 0xa1, 0xab, 0x1e][i % 4];
        i += 1;
    }
    StackReceiptHash::from_bytes(b)
}

const SYNTH: StackIdentity = StackIdentity::new(synth_receipt(), StackLockDigest::ZERO, false);

const SPECIMEN: [VerbEntry; 4] = [
    specimen("design.check", false, false),
    specimen("design.export", true, false),
    specimen("run.list", false, true),
    specimen("run.start", false, false),
];

const fn specimen(name: &'static str, exports: bool, paginated: bool) -> VerbEntry {
    VerbEntry {
        name,
        cluster: "design",
        kind: Kind::Read,
        tier: Tier::S,
        capabilities: &[VerbScope("design:read")],
        input_schema: SchemaId("frob.design.check.in/1"),
        output_schema: SchemaId("frob.design.check.out/1"),
        refusals: &[RefusalCode::FrobRef802],
        idempotent: true,
        cost_model: "static: under 2 s",
        first_milestone: Milestone::M1,
        task_clusters: &[],
        examples: &[],
        purpose: "Specimen entry for envelope tests.",
        use_when: "Never; test only.",
        owner: "frob-verbs",
        wait: None,
        actions: &[],
        schema_version: 1,
        paginated,
        exports,
        estimable: false,
        destructive: false,
        public: false,
        key_optional: false,
        option_capabilities: &[],
        spends_physics: None,
        released: false,
        params: &[],
        conformance_only: &[],
    }
}

fn mirror() -> RefusalRegistry {
    RefusalRegistry::from_bytes(MIRROR.as_bytes()).expect("synthetic mirror registry loads")
}

fn product() -> ClaimProduct {
    ClaimProduct::estimated(
        ClaimKind::DesignComparison,
        InputRef::new(Blake3::from_bytes([7; 32]), "/design"),
        Scope::default(),
        TierMeet::Static,
        DetClass::D0Exact,
        RealityScope::NoClaim,
        FlagSet::new(),
    )
}

fn evidence() -> EvidenceBlock {
    EvidenceBlock::from_claim_product(&product(), "m:9f2c\u{2026}").expect("evidence")
}

fn body(json: &str) -> JsonBody {
    JsonBody::from_json(json.as_bytes()).expect("result JSON")
}

fn fields(message: &str) -> RefusalFields {
    RefusalFields {
        message: message.to_owned(),
        pointer: "/params".to_owned(),
        observed: "x".to_owned(),
        required: "y".to_owned(),
        unit: String::new(),
        detail: None,
        affected_claims: Vec::new(),
        retry: RetryPolicy::AfterChange,
        repairs: vec![Repair::new("fix the argument")],
    }
}

const EXAMPLE_RESULT: &str = r#"{"mass_kg":14.2,"closure":"closed","symmetry":"C2xC2","findings":[{"code":"A-11.knee.continuous","severity":"high","message":"Knee continuous torque demand exceeds the card by 31% in crouch.","repairs":[{"edit":"swap-actuator","args":{"joints":"*.knee","to":"qdd-l"},"predicted":{"mass_kg":"+0.9","margin":"+46%"}},{"edit":"regear","args":{"joints":"*.knee","ratio":1.9}}]}]}"#;
const CLEAN_RESULT: &str =
    r#"{"mass_kg":14.2,"closure":"closed","symmetry":"C2xC2","findings":[]}"#;
const EDIT_COMMAND: &str = "frob design edit quad-a swap-actuator --joints '*.knee' --to qdd-l";
const EDIT_WHY: &str = "Largest predicted margin gain per kilogram.";

fn budget() -> Budget {
    Budget::new()
        .with("gpu_s_left", 600.0)
        .and_then(|b| b.with("cpu_core_s_left", 7199.6))
        .expect("budget")
}

const DESIGN_COST: Cost = Cost {
    wall_ms: 412,
    cpu_core_s: 0.4,
    gpu_s: 0.0,
};

fn build_completed() -> Envelope {
    let fixture = FixtureBinding::new(
        "terrain",
        "@fixture:stairs-15cm",
        "fixtures/terrain/stairs-15cm.toml",
        Blake3::from_bytes([0xab; 32]),
    )
    .expect("fixture binding");
    Envelope::completed("design.check", body(CLEAN_RESULT), evidence(), DESIGN_COST)
        .expect("completed")
        .with_bound(
            Bound::new()
                .bind("design", "d:41aa\u{2026}")
                .expect("bind")
                .with_fixtures(vec![fixture]),
        )
        .with_budget(budget())
        .expect("budget")
        .with_warning(Warning::executor_in_process())
}

fn build_with_findings() -> Envelope {
    Envelope::completed_with_findings(
        "design.check",
        body(EXAMPLE_RESULT),
        evidence().with_seed(42),
        DESIGN_COST,
    )
    .expect("findings")
    .with_bound(Bound::new().bind("design", "d:41aa\u{2026}").expect("bind"))
    .with_budget(budget())
    .expect("budget")
    .with_next_action(NextAction::new(EDIT_COMMAND, EDIT_WHY).expect("next action"))
}

fn refusal_802() -> Refusal {
    Refusal::new(
        RefusalCode::FrobRef802,
        "Arguments fail the schema: unknown parameter `jionts`",
        "/params/jionts",
        "jionts",
        "a parameter of design.edit",
        "",
        RetryPolicy::AfterChange,
        vec![Repair::with_detail(
            "--joints",
            "The closest registered parameter.",
        )],
    )
}

fn build_refused() -> Envelope {
    Envelope::from_refusal(
        "design.edit",
        &refusal_802(),
        Cost {
            wall_ms: 3,
            ..Cost::ZERO
        },
    )
    .expect("refused")
}

fn job_report() -> JobReport {
    JobReport {
        id: "j:7f01".to_owned(),
        state: "running".to_owned(),
        progress: ProgressReport {
            done: 3,
            total: Some(12),
            unit: "episodes".to_owned(),
        },
        eta: Some(Eta {
            point_s: 42.5,
            lo_s: 30.0,
            hi_s: 60.0,
        }),
        poll_after_ms: 2000,
        last_committed: Some("c:0003".to_owned()),
    }
}

fn build_deferred() -> Envelope {
    Envelope::deferred(
        "run.start",
        &job_report(),
        Cost {
            wall_ms: 38,
            cpu_core_s: 0.1,
            gpu_s: 0.0,
        },
    )
    .expect("deferred")
    .with_budget(Budget::new().with("gpu_s_left", 540.0).expect("budget"))
    .expect("budget")
    .with_next_action(
        NextAction::new("frob job wait j:7f01 --json", "Poll until the run commits.")
            .expect("next"),
    )
}

fn build_paginated() -> Envelope {
    Envelope::completed(
        "run.list",
        body(r#"{"items":[{"id":"r:01"},{"id":"r:02"}]}"#),
        evidence(),
        Cost {
            wall_ms: 7,
            ..Cost::ZERO
        },
    )
    .expect("completed")
    .with_page(Page::new("f:0192", Some("c:2")).expect("page"))
    .expect("page applies")
    .with_next_action(
        NextAction::new(
            "frob run list --cursor c:2 --json",
            "Read the next page at the same frontier.",
        )
        .expect("next"),
    )
}

fn build_replay() -> Envelope {
    Envelope::completed(
        "design.check",
        body(CLEAN_RESULT),
        evidence(),
        Cost {
            wall_ms: 1,
            ..Cost::ZERO
        },
    )
    .expect("completed")
    .replayed()
}

fn build_export() -> Envelope {
    Envelope::completed(
        "design.export",
        body(r#"{"artifact":"a:77e0…","format":"urdf"}"#),
        evidence(),
        Cost {
            wall_ms: 95,
            cpu_core_s: 0.2,
            gpu_s: 0.0,
        },
    )
    .expect("completed")
    .with_authorization(Authorization::Artifact(NoAuthorization::VALUE))
}

fn fault_context() -> FaultContext {
    FaultContext {
        verb: "design.check".to_owned(),
        idempotency_key: Some("k-0001".to_owned()),
        diagnostic_path: ".frob/diag/fault-0001.txt".to_owned(),
    }
}

fn build_fault() -> Envelope {
    Envelope::internal_fault(&fault_context(), Cost::ZERO)
}

/// Golden file name, its bytes, and the envelope the constructors build for it.
fn built_goldens() -> Vec<(&'static str, &'static str, Envelope)> {
    vec![
        (
            "completed.json",
            include_str!("../../tests/golden/envelope/completed.json"),
            build_completed(),
        ),
        (
            "completed_with_findings.json",
            include_str!("../../tests/golden/envelope/completed_with_findings.json"),
            build_with_findings(),
        ),
        (
            "refused.json",
            include_str!("../../tests/golden/envelope/refused.json"),
            build_refused(),
        ),
        (
            "deferred.json",
            include_str!("../../tests/golden/envelope/deferred.json"),
            build_deferred(),
        ),
        (
            "paginated.json",
            include_str!("../../tests/golden/envelope/paginated.json"),
            build_paginated(),
        ),
        (
            "replay.json",
            include_str!("../../tests/golden/envelope/replay.json"),
            build_replay(),
        ),
        (
            "export.json",
            include_str!("../../tests/golden/envelope/export.json"),
            build_export(),
        ),
        (
            "internal_fault.json",
            include_str!("../../tests/golden/envelope/internal_fault.json"),
            build_fault(),
        ),
    ]
}

const DESIGN_CHECK_GOLDEN: &str =
    include_str!("../../tests/golden/envelope/design_check_example.json");

fn canonical_line(env: &Envelope) -> String {
    format!("{}\n", env.to_canonical().expect("canonical").as_str())
}

fn first_diff(a: &str, b: &str) -> String {
    let at = a
        .bytes()
        .zip(b.bytes())
        .position(|(x, y)| x != y)
        .unwrap_or_else(|| a.len().min(b.len()));
    let lo = at.saturating_sub(40);
    format!(
        "first differing byte {at}:\n  built : {}\n  golden: {}",
        a.get(lo..(at + 40).min(a.len())).unwrap_or(""),
        b.get(lo..(at + 40).min(b.len())).unwrap_or("")
    )
}

fn tree(text: &str) -> Tree {
    Tree::parse(text.as_bytes()).expect("JSON")
}

/// Suppress the default panic message for the panics the tests plant on purpose.
fn quiet_planted_panics() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let planted = info
                .payload()
                .downcast_ref::<&str>()
                .is_some_and(|s| s.starts_with("planted handler panic"));
            if !planted {
                previous(info);
            }
        }));
    });
}

fn call(verb: &str) -> Call<'_> {
    Call {
        verb,
        idempotency_key: Some("k-0001"),
        diagnostic_path: ".frob/diag/fault-0001.txt",
    }
}

fn served<F>(registry: &RefusalRegistry, verb: &str, handler: F) -> (Vec<u8>, ExitCode)
where
    F: FnOnce(&mut ProgressSink<'_>) -> Result<Envelope, HandlerFault>,
{
    let mut out = Vec::new();
    let exit = serve(&mut out, registry, &SPECIMEN, &call(verb), handler)
        .expect("writer to a Vec never fails");
    (out, exit)
}

// ---------------------------------------------------------------------------------------
// Goldens
// ---------------------------------------------------------------------------------------

#[test]
fn golden_design_check_roundtrip() {
    // The bead's example as written (its key order and layout), with the review notes
    // applied: determinism printed as the class `D0`, and `evidence.stack_receipt`.
    let example = format!(
        r#"{{ "schema": "frob.envelope/1", "verb": "design.check", "ok": true, "exit": 2,
  "result": {{ "mass_kg": 14.2, "closure": "closed", "symmetry": "C2xC2",
              "findings": [ {{ "code": "A-11.knee.continuous", "severity": "high",
                              "message": "Knee continuous torque demand exceeds the card by 31% in crouch.",
                              "repairs": [ {{ "edit": "swap-actuator", "args": {{"joints": "*.knee", "to": "qdd-l"}},
                                             "predicted": {{"mass_kg": "+0.9", "margin": "+46%"}} }},
                                           {{ "edit": "regear", "args": {{"joints": "*.knee", "ratio": 1.9}} }} ] }} ] }},
  "evidence": {{ "class": "Tested", "tier": "static", "model_id": "m:9f2c…", "determinism": "D0",
                "claims": [], "reality": "no-claim", "stack_receipt": "sr:{SYNTH_RECEIPT_HEX}" }},
  "bound": {{ "design": "d:41aa…" }},
  "cost": {{ "wall_ms": 412, "cpu_core_s": 0.4, "gpu_s": 0 }},
  "budget": {{ "gpu_s_left": 600, "cpu_core_s_left": 7199.6 }},
  "warnings": [], "next_actions": [ {{ "command": "frob design edit quad-a swap-actuator --joints '*.knee' --to qdd-l",
                                      "why": "Largest predicted margin gain per kilogram." }} ] }}"#
    );
    let canonical = format!(
        "{}\n",
        tree(&example).to_canonical().expect("canonical").as_str()
    );
    assert_eq!(
        canonical,
        DESIGN_CHECK_GOLDEN,
        "{}",
        first_diff(&canonical, DESIGN_CHECK_GOLDEN)
    );
    let read = read_envelope(DESIGN_CHECK_GOLDEN.trim_end().as_bytes())
        .expect("golden is a valid envelope");
    assert_eq!(read.exit(), ExitCode::Flagged);
    assert_eq!(read.stack_receipt(), synth_receipt());
}

#[test]
fn golden_built_forms_match_bytes() {
    for (name, golden, env) in built_goldens() {
        let built = canonical_line(&env.restamped(SYNTH));
        assert_eq!(
            built,
            golden,
            "golden {name}: {}",
            first_diff(&built, golden)
        );
    }
}

#[test]
fn golden_files_validate_and_roundtrip() {
    let mut all: Vec<(&str, &str)> = built_goldens()
        .into_iter()
        .map(|(n, g, _)| (n, g))
        .collect();
    all.push(("design_check_example.json", DESIGN_CHECK_GOLDEN));
    for (name, golden) in all {
        assert!(
            golden.ends_with('\n') && !golden.ends_with("\n\n"),
            "{name}: one trailing newline"
        );
        let line = golden.trim_end_matches('\n');
        let read = read_envelope(line.as_bytes()).unwrap_or_else(|e| panic!("{name}: {e}"));
        let again = read.tree().to_canonical().expect("canonical");
        assert_eq!(
            again.as_str(),
            line,
            "{name} does not round-trip byte-identically"
        );
    }
}

#[test]
fn golden_forms_cover_the_acceptance_list() {
    let exits: Vec<(&str, u8, bool)> = built_goldens()
        .iter()
        .map(|(n, g, _)| {
            let t = tree(g);
            let exit =
                u8::try_from(t.get("exit").and_then(Tree::as_int53).expect("exit")).expect("u8");
            (
                *n,
                exit,
                t.get("idempotent_replay").is_some() || t.get("cursor").is_some(),
            )
        })
        .collect();
    for (name, exit) in [
        ("completed.json", 0),
        ("completed_with_findings.json", 2),
        ("refused.json", 3),
        ("deferred.json", 8),
        ("paginated.json", 0),
        ("replay.json", 0),
    ] {
        assert!(
            exits.iter().any(|(n, e, _)| *n == name && *e == exit),
            "{name} must exist with exit {exit}"
        );
    }
    assert!(
        exits
            .iter()
            .any(|(n, _, marked)| *n == "paginated.json" && *marked)
    );
    assert!(
        exits
            .iter()
            .any(|(n, _, marked)| *n == "replay.json" && *marked)
    );
}

// ---------------------------------------------------------------------------------------
// Exit table
// ---------------------------------------------------------------------------------------

#[test]
fn exit_table_exact() {
    let reg = mirror();
    for &(code, exit) in EXPECTED {
        let env = Envelope::from_code_in(&reg, "design.check", code, fields("refused"), Cost::ZERO)
            .unwrap_or_else(|e| panic!("{code}: {e}"));
        assert_eq!(env.exit().code(), exit, "{code}");
        assert!(!env.ok());
        assert_eq!(
            rule_exit_for_code(&reg, code).expect("rule").code(),
            exit,
            "{code} under [exit_rule]"
        );
    }
    assert_eq!(
        Envelope::internal_fault(&fault_context(), Cost::ZERO).exit(),
        ExitCode::Indeterminate
    );
}

#[test]
fn exit_table_exact_real_registry() {
    for &(code, exit) in EXPECTED {
        let env = Envelope::from_code("design.check", code, fields("refused"), Cost::ZERO)
            .unwrap_or_else(|e| panic!("{code}: {e}"));
        assert_eq!(env.exit().code(), exit, "{code}");
    }
}

#[test]
fn from_refusal_reads_the_embedded_row() {
    let reg = embedded_registry().expect("embedded registry");
    for row in reg.rows() {
        let code = RefusalCode::parse(&row.code).expect("generated enum covers every row");
        let r = Refusal::new(code, "m", "/p", "o", "q", "", row.retry, vec![]);
        let env = Envelope::from_refusal("design.check", &r, Cost::ZERO).expect("refused");
        assert_eq!(env.exit().code(), row.exit, "{}", row.code);
        assert_eq!(
            env.refusal().map(RefusalBlock::code),
            Some(row.code.as_str())
        );
    }
}

/// Check every row of the exit-table fixture against a registry; returns the row count.
fn check_fixture_rows(text: &str, reg: &RefusalRegistry) -> Result<usize, String> {
    let root = toml::parse(text.as_bytes()).map_err(|e| e.to_string())?;
    let mut top = Fields::new(root, "exit_table");
    let schema = top.require_str("schema").map_err(|e| e.to_string())?;
    let gate = top.require_str("gate").map_err(|e| e.to_string())?;
    let ka = top.require_str("known_answer").map_err(|e| e.to_string())?;
    let rows = top.require_array("row").map_err(|e| e.to_string())?;
    top.finish().map_err(|e| e.to_string())?;
    if schema != "frob.tether.exit-table/1" || gate != "GATE-TETHER-03" || ka != "KA-755" {
        return Err(format!(
            "schema `{schema}` / gate `{gate}` / known answer `{ka}`"
        ));
    }
    let base = base_schema().map_err(|e| e.to_string())?;
    let mut n = 0;
    for (i, sp) in rows.into_iter().enumerate() {
        let toml::Value::Table(t) = sp.value else {
            return Err(format!("row {i} is not a table"));
        };
        let mut f = Fields::new(t, format!("row[{i}]"));
        let id = f.require_str("id").map_err(|e| e.to_string())?;
        let argv = f.require_array("argv").map_err(|e| e.to_string())?;
        let ok = f.require_bool("ok").map_err(|e| e.to_string())?;
        let exit = f.require_int("exit").map_err(|e| e.to_string())?;
        let code = f.require_str("code").map_err(|e| e.to_string())?;
        let _input = f.require_str("input").map_err(|e| e.to_string())?;
        let _exercised = f.require_bool("exercised").map_err(|e| e.to_string())?;
        let _mechanism = f.require_str("mechanism").map_err(|e| e.to_string())?;
        f.finish().map_err(|e| e.to_string())?;
        if argv.is_empty() || argv.iter().any(|a| a.value.as_str().is_none()) {
            return Err(format!("{id}: argv must be a non-empty array of strings"));
        }
        let row_exit = exit_for_code(reg, &code).map_err(|e| format!("{id}: {e}"))?;
        if i64::from(row_exit.code()) != exit {
            return Err(format!(
                "{id}: fixture says exit {exit}, the registry row of {code} says {row_exit}"
            ));
        }
        if ok != row_exit.is_ok() {
            return Err(format!("{id}: fixture says ok = {ok}"));
        }
        let env = Envelope::from_code_in(reg, "design.check", &code, fields(&id), Cost::ZERO)
            .map_err(|e| e.to_string())?;
        let bytes = env.to_canonical().map_err(|e| e.to_string())?;
        let read =
            read_envelope_with(bytes.as_bytes(), base, reg).map_err(|e| format!("{id}: {e}"))?;
        if i64::from(read.exit().code()) != exit || read.refusal_code() != Some(code.as_str()) {
            return Err(format!(
                "{id}: envelope carries exit {} code {:?}",
                read.exit(),
                read.refusal_code()
            ));
        }
        n += 1;
    }
    Ok(n)
}

#[test]
fn exit_fixture_rows() {
    assert!(
        EXIT_TABLE_FIXTURE.starts_with("# synthetic"),
        "fixture kind header"
    );
    let n = check_fixture_rows(EXIT_TABLE_FIXTURE, &mirror()).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        n, 5,
        "KA-755: malformed, unknown, over-budget, unscoped and conflicting rows"
    );
}

#[test]
fn control_wrong_exit_row_fixture_fails() {
    // Planted defect: the malformed-arguments row claims exit 4 for FROB-REF-802.
    let planted = EXIT_TABLE_FIXTURE.replacen(
        "exit = 3\ncode = \"FROB-REF-802\"",
        "exit = 4\ncode = \"FROB-REF-802\"",
        1,
    );
    assert_ne!(
        planted, EXIT_TABLE_FIXTURE,
        "the plant must change the fixture"
    );
    let err = check_fixture_rows(&planted, &mirror())
        .expect_err("a wrong exit row must fail the fixture");
    assert!(
        err.contains("FROB-REF-802") && err.contains("exit 4"),
        "{err}"
    );
}

#[test]
fn control_wrong_exit_row_registry_fails() {
    // Planted defect: 803 (conflict list, exit 7) entered with exit 3.
    let planted = MIRROR.replacen(
        "code = \"FROB-REF-803\"\nmeaning = \"synthetic: idempotency key reuse with different content\"\ndefined_in = \"§32.4\"\nexit = 7",
        "code = \"FROB-REF-803\"\nmeaning = \"synthetic: idempotency key reuse with different content\"\ndefined_in = \"§32.4\"\nexit = 3",
        1,
    );
    assert_ne!(planted, MIRROR);
    match RefusalRegistry::from_bytes(planted.as_bytes()) {
        Err(RefusalRegistryError::ExitMismatch {
            row,
            observed,
            expected,
            ..
        }) => {
            assert_eq!(
                (row.as_str(), observed, expected),
                ("FROB-REF-803", 3, Some(7))
            );
        }
        other => panic!("expected an exit mismatch, got {other:?}"),
    }
}

#[test]
fn one_code_one_exit() {
    for (label, reg) in [
        ("embedded", embedded_registry().expect("embedded").clone()),
        ("mirror", mirror()),
    ] {
        let mut seen = std::collections::BTreeSet::new();
        for row in reg.rows() {
            assert!(
                seen.insert(row.number),
                "{label}: {} appears twice",
                row.code
            );
            let by_row = exit_for_code(&reg, &row.code).expect("row exit");
            let by_rule = rule_exit_for_code(&reg, &row.code).expect("rule exit");
            assert_eq!(
                by_row, by_rule,
                "{label}: {} row and rule disagree",
                row.code
            );
        }
        assert!(!reg.rows().is_empty(), "{label}: no rows");
    }
    // A row with no exit and a row with two exits both fail the load.
    let row = "\n[[refusal]]\ncode = \"FROB-REF-805\"\nmeaning = \"m\"\ndefined_in = \"§32.4\"\nretry = \"never\"\n";
    let head = "schema = \"frob.refusals/1\"\n\n[exit_rule]\n\"8xx\" = 3\n";
    let none = format!("{head}{row}");
    assert!(
        RefusalRegistry::from_bytes(none.as_bytes()).is_err(),
        "a row with no exit loads"
    );
    let two = format!("{head}{row}exit = 3\nexit = 3\n");
    assert!(
        RefusalRegistry::from_bytes(two.as_bytes()).is_err(),
        "a row with two exits loads"
    );
    let one = format!("{head}{row}exit = 3\n");
    assert!(
        RefusalRegistry::from_bytes(one.as_bytes()).is_ok(),
        "the control row itself must load"
    );
}

fn mirror_with(row: &str) -> Result<RefusalRegistry, RefusalRegistryError> {
    RefusalRegistry::from_bytes(format!("{MIRROR}\n{row}").as_bytes())
}

#[test]
fn exit_rule_from_registry() {
    let r771 = "[[refusal]]\ncode = \"FROB-REF-771\"\nmeaning = \"m\"\ndefined_in = \"§33\"\nexit = 4\nretry = \"never\"\n";
    match mirror_with(r771) {
        Err(RefusalRegistryError::ExitMismatch { row, reason, .. }) => {
            assert_eq!(row, "FROB-REF-771");
            assert!(reason.contains("77x"), "{reason}");
        }
        other => panic!("771 without a 77x rule must fail the load, got {other:?}"),
    }
    let r741 = "[[refusal]]\ncode = \"FROB-REF-741\"\nmeaning = \"m\"\ndefined_in = \"§33\"\nexit = 3\nretry = \"never\"\n";
    match mirror_with(r741) {
        Err(RefusalRegistryError::ExitMismatch {
            row,
            observed,
            expected,
            ..
        }) => {
            assert_eq!(
                (row.as_str(), observed, expected),
                ("FROB-REF-741", 3, Some(4))
            );
        }
        other => panic!("741 with exit 3 must fail the load, got {other:?}"),
    }
    let ok741 = mirror_with(&r741.replace("exit = 3", "exit = 4")).expect("741 with exit 4 loads");
    assert_eq!(exit_for_code(&ok741, "FROB-REF-741"), Ok(ExitCode::Budget));

    let reg = mirror();
    for (code, exit) in [
        ("FROB-REF-112", ExitCode::Conflict),
        ("FROB-REF-840", ExitCode::Capability),
    ] {
        assert_eq!(
            rule_exit_for_code(&reg, code),
            Ok(exit),
            "{code} by the exception list"
        );
        let env = Envelope::from_code_in(&reg, "design.check", code, fields("x"), Cost::ZERO)
            .expect("refused");
        assert_eq!(env.exit(), exit, "{code}");
    }
    // Without the exception lists the families give 3: the lists are what decide.
    let without_lists = MIRROR
        .replace(
            "conflict = [112, 803, 804, 809]",
            "conflict = [803, 804, 809]",
        )
        .replace("capability = [840]", "capability = []")
        .replace(
            "meaning = \"synthetic: stale parent\"\ndefined_in = \"§32.4\"\nexit = 7",
            "meaning = \"synthetic: stale parent\"\ndefined_in = \"§32.4\"\nexit = 3",
        )
        .replace(
            "meaning = \"synthetic: capability exception\"\ndefined_in = \"§32.4\"\nexit = 5",
            "meaning = \"synthetic: capability exception\"\ndefined_in = \"§32.4\"\nexit = 3",
        );
    let no_lists = RefusalRegistry::from_bytes(without_lists.as_bytes())
        .expect("registry without the 112/840 exceptions loads");
    assert_eq!(
        exit_for_code(&no_lists, "FROB-REF-112"),
        Ok(ExitCode::Refused)
    );
    assert_eq!(
        exit_for_code(&no_lists, "FROB-REF-840"),
        Ok(ExitCode::Refused)
    );
}

#[test]
fn codes_compare_as_whole_tokens() {
    let reg = mirror();
    assert_eq!(
        exit_for_code(&reg, "FROB-REF-810"),
        Ok(ExitCode::Indeterminate)
    );
    for code in [
        "FROB-REF-8100",
        "FROB-REF-81",
        "FROB-REF-0810",
        "FROB-REF-810 ",
        "frob-ref-810",
        "FROB-REF-810x",
    ] {
        assert!(
            exit_for_code(&reg, code).is_err(),
            "`{code}` must not resolve to FROB-REF-810"
        );
        assert!(
            Envelope::from_code_in(&reg, "design.check", code, fields("x"), Cost::ZERO).is_err(),
            "{code}"
        );
    }
    assert_eq!(parse_refusal_code("FROB-REF-8100"), Some(8100));
    assert_eq!(parse_refusal_code("FROB-REF-0810"), None);
    // An envelope claiming FROB-REF-8100 with 810's exit is refused by the reader.
    let fault = canonical_line(
        &Envelope::from_code_in(
            &reg,
            "design.check",
            "FROB-REF-810",
            fields("x"),
            Cost::ZERO,
        )
        .expect("810"),
    );
    let planted = fault.replace("FROB-REF-810", "FROB-REF-8100");
    let base = base_schema().expect("schema");
    assert!(matches!(
        read_envelope_with(planted.trim_end().as_bytes(), base, &reg),
        Err(ReadError::Code(ExitLookupError::Unregistered(_)))
    ));
    assert!(read_envelope_with(fault.trim_end().as_bytes(), base, &reg).is_ok());
}

#[test]
fn reader_refuses_a_wrong_exit() {
    let reg = mirror();
    let env = canonical_line(
        &Envelope::from_code_in(
            &reg,
            "design.check",
            "FROB-REF-803",
            fields("x"),
            Cost::ZERO,
        )
        .expect("803"),
    );
    assert!(env.contains("\"exit\":7"));
    let planted = env.replace("\"exit\":7", "\"exit\":3");
    match read_envelope_with(
        planted.trim_end().as_bytes(),
        base_schema().expect("schema"),
        &reg,
    ) {
        Err(ReadError::ExitMismatch {
            code,
            observed,
            expected,
        }) => {
            assert_eq!((code.as_str(), observed, expected), ("FROB-REF-803", 3, 7));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn no_exit_one() {
    assert!(ExitCode::ALL.iter().all(|e| e.code() != 1));
    assert_eq!(
        ExitCode::ALL.map(ExitCode::code),
        [0, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    for v in 0..=u8::MAX {
        assert_ne!(ExitCode::from_refusal_exit(v).map(ExitCode::code), Some(1));
        assert_ne!(
            ExitCode::from_wire(i64::from(v)).map(ExitCode::code),
            Some(1)
        );
    }
    assert_eq!(ExitCode::from_wire(1), None);
    // Source scan: no path in this crate exits the process or names exit status 1.
    let needles = [
        ["process::", "exit("].concat(),
        ["ExitCode::", "FAILURE"].concat(),
        ["exit(", "1)"].concat(),
    ];
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![root];
    let mut scanned = 0;
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read src").flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let text = fs::read_to_string(&p).expect("read source");
                for n in &needles {
                    assert!(!text.contains(n.as_str()), "{} contains `{n}`", p.display());
                }
                scanned += 1;
            }
        }
    }
    assert!(scanned >= 10, "scanned {scanned} files");
}

// ---------------------------------------------------------------------------------------
// Blocks
// ---------------------------------------------------------------------------------------

#[test]
fn deferred_has_denominator() {
    for total in [None, Some(0)] {
        let mut r = job_report();
        r.progress.total = total;
        assert_eq!(
            Envelope::deferred("run.start", &r, Cost::ZERO).map(|_| ()),
            Err(EnvelopeError::ProgressWithoutDenominator),
            "total {total:?}"
        );
    }
    let mut over = job_report();
    over.progress.done = 13;
    assert!(
        Envelope::deferred("run.start", &over, Cost::ZERO).is_err(),
        "done above total"
    );
    assert_eq!(
        Envelope::deferred("run.start", &job_report(), Cost::ZERO)
            .expect("deferred")
            .exit(),
        ExitCode::Running
    );
    // The reader refuses an overrun the schema cannot see.
    let line = canonical_line(&build_deferred());
    let planted = line.replace("\"done\":3", "\"done\":13");
    assert!(matches!(
        read_envelope(planted.trim_end().as_bytes()),
        Err(ReadError::ProgressOverrun {
            done: 13,
            total: 12
        })
    ));
    let no_total = line.replace(",\"total\":12", "");
    assert!(matches!(
        read_envelope(no_total.trim_end().as_bytes()),
        Err(ReadError::Schema(_))
    ));
}

#[test]
fn refusal_detail_absent_unless_set() {
    let plain = canonical_line(&build_refused());
    let refusal = read_envelope(plain.trim_end().as_bytes()).expect("valid");
    assert!(
        refusal
            .tree()
            .get("refusal")
            .and_then(|r| r.get("detail"))
            .is_none(),
        "detail absent when unset"
    );
    let interim = refusal_802().with_detail("ledger-request-slug");
    let env = Envelope::from_refusal("design.edit", &interim, Cost::ZERO).expect("refused");
    let read = read_envelope(env.to_canonical().expect("bytes").as_bytes()).expect("valid");
    assert_eq!(
        read.tree()
            .get("refusal")
            .and_then(|r| r.get("detail"))
            .and_then(Tree::as_str),
        Some("ledger-request-slug")
    );
    let empty = refusal_802().with_detail("");
    assert!(
        Envelope::from_refusal("design.edit", &empty, Cost::ZERO).is_err(),
        "a set but empty detail"
    );
}

#[test]
fn bound_fixtures_rules() {
    let d = Blake3::from_bytes([1; 32]);
    assert!(FixtureBinding::new("terrain", "@fixture:x", "fixtures/x.toml", d).is_ok());
    assert!(FixtureBinding::new("terrain", "fixture:x", "fixtures/x.toml", d).is_err());
    assert!(FixtureBinding::new("terrain", "@fixture:", "fixtures/x.toml", d).is_err());
    assert!(FixtureBinding::new("terrain", "@fixture:x", "/abs/x.toml", d).is_err());
    assert!(FixtureBinding::new("", "@fixture:x", "fixtures/x.toml", d).is_err());
    assert!(
        FixtureBinding::new("terrain", "@fixture:a\nb", "fixtures/x.toml", d).is_err(),
        "schema `.+` has no newline"
    );
    assert!(
        Bound::new().bind("fixtures", "x").is_err(),
        "`fixtures` is reserved"
    );
    // A refusal of a later stage carries the bindings too.
    let fx = FixtureBinding::new("terrain", "@fixture:x", "fixtures/x.toml", d).expect("fx");
    let env = build_refused().with_bound(Bound::new().with_fixtures(vec![fx]));
    let read = read_envelope(env.to_canonical().expect("bytes").as_bytes()).expect("valid");
    let fixtures = read
        .tree()
        .get("bound")
        .and_then(|b| b.get("fixtures"))
        .and_then(Tree::as_array)
        .expect("fixtures");
    assert_eq!(fixtures.len(), 1);
    assert_eq!(
        fixtures[0].get("digest").and_then(Tree::as_str),
        Some("01".repeat(32).as_str())
    );
    // The schema refuses a binding with an absolute path or a missing digest.
    let line = canonical_line(&env);
    for planted in [
        line.replace("\"path\":\"fixtures/x.toml\"", "\"path\":\"/x.toml\""),
        line.replace(&format!("\"digest\":\"{}\",", "01".repeat(32)), ""),
    ] {
        assert!(
            matches!(
                read_envelope(planted.trim_end().as_bytes()),
                Err(ReadError::Schema(_))
            ),
            "{planted}"
        );
    }
}

#[test]
fn determinism_prints_the_class() {
    let line = canonical_line(&build_completed());
    assert!(line.contains("\"determinism\":\"D0\""), "{line}");
    assert!(
        !line.contains("exact"),
        "the class is printed, not a word: {line}"
    );
    for class in [
        DetClass::D1CrossBackend,
        DetClass::D2PerBackend,
        DetClass::D3Tolerance,
        DetClass::D4Unspecified,
    ] {
        let p = ClaimProduct::estimated(
            ClaimKind::Performance,
            InputRef::new(Blake3::from_bytes([2; 32]), "/x"),
            Scope::default(),
            TierMeet::Referee,
            class,
            RealityScope::NoClaim,
            FlagSet::new(),
        );
        let e = EvidenceBlock::from_claim_product(&p, "m:x").expect("evidence");
        let env = Envelope::completed("design.check", body("{}"), e, Cost::ZERO).expect("env");
        let read = read_envelope(env.to_canonical().expect("bytes").as_bytes()).expect("valid");
        let printed = read
            .tree()
            .get("evidence")
            .and_then(|v| v.get("determinism"))
            .and_then(Tree::as_str);
        assert_eq!(printed, Some(class.as_str()));
    }
}

#[test]
fn builders_refuse_bad_values() {
    assert!(
        Cost {
            wall_ms: 1,
            cpu_core_s: f64::NAN,
            gpu_s: 0.0
        }
        .check()
        .is_err()
    );
    assert!(
        Envelope::completed(
            "design.check",
            body("{}"),
            evidence(),
            Cost {
                wall_ms: 0,
                cpu_core_s: -1.0,
                gpu_s: 0.0
            }
        )
        .is_err()
    );
    assert!(
        Budget::new().with("gpu_s", 1.0).is_err(),
        "budget names end in _left"
    );
    assert!(Budget::new().with("gpu_s_left", f64::INFINITY).is_err());
    assert!(
        Budget::new()
            .with("gpu_s_left", 1.0)
            .and_then(|b| b.with("gpu_s_left", 2.0))
            .is_err()
    );
    assert!(
        Envelope::completed("design.check", body("{}"), evidence(), Cost::ZERO)
            .expect("env")
            .with_budget(Budget::new())
            .is_err()
    );
    assert!(Warning::new("Bad Code", "m").is_err());
    assert!(NextAction::new("", "why").is_err());
    assert!(Page::new("", None).is_err());
    assert!(
        build_refused()
            .with_page(Page::new("f:1", None).expect("page"))
            .is_err(),
        "a refusal is not paginated"
    );
    assert!(build_deferred().with_job(&job_report()).is_err());
    assert!(
        Envelope::completed("design.check", body("{}"), evidence(), Cost::ZERO)
            .expect("env")
            .with_delta(body("[1]"))
            .is_err()
    );
    assert!(
        JsonBody::from_json(br#"{"a":1,"a":2}"#).is_err(),
        "duplicate members refused"
    );
    assert!(Envelope::completed("bad\nverb", body("{}"), evidence(), Cost::ZERO).is_err());
    let mut eta = job_report();
    eta.eta = Some(Eta {
        point_s: 10.0,
        lo_s: 20.0,
        hi_s: 30.0,
    });
    assert!(JobHandle::new(&eta).is_err(), "lo <= point <= hi");
    // A count the canonical writer cannot write exactly (above 2^53 - 1) is refused when
    // the block is built, not when the envelope is serialized.
    assert!(
        Cost {
            wall_ms: (1 << 53) - 1,
            cpu_core_s: 0.0,
            gpu_s: 0.0
        }
        .check()
        .is_ok()
    );
    assert!(
        Cost {
            wall_ms: 1 << 53,
            cpu_core_s: 0.0,
            gpu_s: 0.0
        }
        .check()
        .is_err()
    );
    let mut slow = job_report();
    slow.poll_after_ms = 1 << 60;
    assert!(JobHandle::new(&slow).is_err(), "poll_after_ms above int53");
    // `job.state` is the schema's `^[a-z][a-z-]*$`: a digit is refused at construction.
    let mut phase = job_report();
    phase.state = "phase2".to_owned();
    assert!(JobHandle::new(&phase).is_err(), "job state with a digit");
    phase.state = "phase-two".to_owned();
    assert!(JobHandle::new(&phase).is_ok());
}

#[test]
fn job_beside_a_refusal_and_delta_after_a_run() {
    let stop = Envelope::from_code_in(
        &mirror(),
        "run.start",
        "FROB-REF-702",
        fields("budget stop"),
        Cost::ZERO,
    )
    .expect("702")
    .with_job(&job_report())
    .expect("job acknowledgement");
    let line = canonical_line(&stop);
    let read = read_envelope_with(
        line.trim_end().as_bytes(),
        base_schema().expect("schema"),
        &mirror(),
    )
    .expect("valid");
    assert_eq!(read.exit(), ExitCode::Budget);
    assert!(
        read.tree().get("job").is_some() && read.tree().get("result").is_none(),
        "nothing partial is published"
    );
    let after = build_replay()
        .with_delta(body(r#"{"change":"knee ratio 1.6 -> 1.9"}"#))
        .expect("delta");
    assert!(read_envelope(after.to_canonical().expect("bytes").as_bytes()).is_ok());
}

// ---------------------------------------------------------------------------------------
// Schema
// ---------------------------------------------------------------------------------------

#[test]
fn schema_structure_rules() {
    let good = canonical_line(&build_completed());
    let good = good.trim_end();
    assert!(read_envelope(good.as_bytes()).is_ok());
    let cases: Vec<(&str, String)> = vec![
        ("exit 1", good.replace("\"exit\":0", "\"exit\":1")),
        ("ok with a refusal exit", good.replace("\"exit\":0", "\"exit\":3")),
        ("unknown member", good.replacen("{\"bound\"", "{\"extra\":1,\"bound\"", 1)),
        ("no evidence class on a result", good.replace("\"class\":\"Estimated\",", "")),
        ("evidence as a bare string", good.replace("\"class\":\"Estimated\"", "\"class\":\"Exact\"")),
        ("no stack receipt", good.replace(&format!(",\"stack_receipt\":\"sr:{}\"", hex_of(&BUILT_UNDER)), "")),
        ("missing warnings", good.replace(",\"warnings\":[{\"code\":\"executor-in-process\",\"message\":\"executor: in-process (the verb ran without the daemon)\"}]", "")),
        ("cursor without frontier", good.replacen("{\"bound\"", "{\"cursor\":\"c:1\",\"bound\"", 1)),
        ("negative wall time", good.replace("\"wall_ms\":412", "\"wall_ms\":-1")),
    ];
    for (name, planted) in cases {
        assert_ne!(planted, good, "{name}: the plant must change the envelope");
        assert!(
            read_envelope(planted.as_bytes()).is_err(),
            "{name} must be refused"
        );
    }
    let refused = canonical_line(&build_refused());
    let with_result = refused
        .trim_end()
        .replacen("{\"cost\"", "{\"result\":{},\"cost\"", 1);
    assert!(
        read_envelope(with_result.as_bytes()).is_err(),
        "a refusal publishes no result"
    );
    let deferred = canonical_line(&build_deferred());
    let without_job = deferred.trim_end().replace(
        ",\"job\":{\"eta_s\":{\"hi\":60,\"lo\":30,\"point\":42.5},\"id\":\"j:7f01\",\"last_committed\":\"c:0003\",\"poll_after_ms\":2000,\"progress\":{\"done\":3,\"total\":12,\"unit\":\"episodes\"},\"state\":\"running\"}",
        "",
    );
    assert_ne!(without_job, deferred.trim_end());
    assert!(
        read_envelope(without_job.as_bytes()).is_err(),
        "exit 8 carries a job handle"
    );
}

fn hex_of(id: &StackIdentity) -> String {
    id.receipt.to_hex()
}

const DESIGN_OUT_SCHEMA: &str = r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object",
 "required":["mass_kg","closure","symmetry","findings"],"additionalProperties":false,
 "properties":{"mass_kg":{"type":"number","minimum":0},"closure":{"enum":["closed","open"]},"symmetry":{"type":"string"},
  "findings":{"type":"array","items":{"$ref":"#/$defs/finding"}}},
 "$defs":{"finding":{"type":"object","required":["code","severity","message"],
  "properties":{"code":{"type":"string"},"severity":{"enum":["low","medium","high"]},"message":{"type":"string"},"repairs":{"type":"array"}}}}}"##;

const EXPORT_OUT_SCHEMA: &str = r#"{"type":"object","required":["artifact","format"],"additionalProperties":false,
 "properties":{"artifact":{"type":"string"},"format":{"enum":["urdf","mjcf"]}}}"#;

#[test]
fn per_verb_specialization() {
    let reg = embedded_registry().expect("embedded");
    let check = envelope_schema_for(&SPECIMEN[0], DESIGN_OUT_SCHEMA).expect("design.check schema");
    for golden in [
        canonical_line(&build_completed().restamped(SYNTH)),
        canonical_line(&build_with_findings()),
    ] {
        read_envelope_with(golden.trim_end().as_bytes(), &check, reg)
            .expect("valid for design.check");
    }
    let good = canonical_line(&build_with_findings());
    let good = good.trim_end();
    for (name, planted) in [
        ("another verb", good.replace("\"verb\":\"design.check\"", "\"verb\":\"design.edit\"")),
        ("result off its schema", good.replace("\"severity\":\"high\"", "\"severity\":\"severe\"")),
        ("unknown result member", good.replace("\"symmetry\":\"C2xC2\"", "\"symmetry\":\"C2xC2\",\"colour\":\"red\"")),
        ("cursor on a verb that does not paginate", good.replacen("{\"bound\"", "{\"cursor\":\"c\",\"frontier\":\"f\",\"bound\"", 1)),
        ("authorization on a verb that does not export", good.replacen("{\"bound\"", "{\"authorization\":{\"execution_authorized\":false,\"hardware_safety_signoff\":false,\"sim_to_real_certified\":false},\"bound\"", 1)),
    ] {
        assert_ne!(planted, good, "{name}");
        match read_envelope_with(planted.as_bytes(), &check, reg) {
            Err(ReadError::Schema(v)) => assert!(!v.is_empty(), "{name}"),
            other => panic!("{name}: {other:?}"),
        }
        // The base schema admits each of them: the specialization is what refuses.
        read_envelope(planted.as_bytes()).unwrap_or_else(|e| panic!("{name} under the base schema: {e}"));
    }
    // The violation path points into the result.
    let off = good.replace("\"severity\":\"high\"", "\"severity\":\"severe\"");
    let Err(ReadError::Schema(v)) = read_envelope_with(off.as_bytes(), &check, reg) else {
        panic!("refused")
    };
    assert_eq!(v[0].instance, "/result/findings/0/severity");
    // The specialized document is itself a schema the validator compiles (refs rewritten).
    let doc = specialized_document(&SPECIMEN[0], tree(DESIGN_OUT_SCHEMA)).expect("doc");
    let defs = doc.get("$defs").expect("$defs");
    assert!(defs.get("result").is_some() && defs.get("result.finding").is_some());
    assert!(Schema::compile(doc).is_ok());
}

#[test]
fn export_flags_constant_false() {
    let line = canonical_line(&build_export());
    let line = line.trim_end();
    assert!(read_envelope(line.as_bytes()).is_ok());
    for (name, planted) in [
        (
            "execution_authorized true",
            line.replace(
                "\"execution_authorized\":false",
                "\"execution_authorized\":true",
            ),
        ),
        (
            "hardware_safety_signoff missing",
            line.replace("\"hardware_safety_signoff\":false,", ""),
        ),
        (
            "sim_to_real_certified as a string",
            line.replace(
                "\"sim_to_real_certified\":false",
                "\"sim_to_real_certified\":\"false\"",
            ),
        ),
    ] {
        assert_ne!(planted, line, "{name}");
        match read_envelope(planted.as_bytes()) {
            Err(ReadError::Authorization(r)) => {
                assert_eq!(r.code, RefusalCode::FrobRef816, "{name}")
            }
            other => panic!("{name}: {other:?}"),
        }
    }
    let packet =
        build_export().with_authorization(Authorization::BuildPacket(PacketAuthorization::VALUE));
    let packet_line = canonical_line(&packet);
    assert!(packet_line.contains("\"fabrication_released\":false"));
    assert!(read_envelope(packet_line.trim_end().as_bytes()).is_ok());
    let released = packet_line.replace(
        "\"fabrication_released\":false",
        "\"fabrication_released\":true",
    );
    assert!(matches!(
        read_envelope(released.trim_end().as_bytes()),
        Err(ReadError::Authorization(_))
    ));
    // An exporting verb's envelope without the three flags fails its specialized schema.
    let export =
        envelope_schema_for(&SPECIMEN[1], EXPORT_OUT_SCHEMA).expect("design.export schema");
    let reg = embedded_registry().expect("embedded");
    assert!(read_envelope_with(line.as_bytes(), &export, reg).is_ok());
    let bare = line.replace(
        "\"authorization\":{\"execution_authorized\":false,\"hardware_safety_signoff\":false,\"sim_to_real_certified\":false},",
        "",
    );
    assert_ne!(bare, line);
    match read_envelope_with(bare.as_bytes(), &export, reg) {
        Err(ReadError::Schema(v)) => {
            assert!(v.iter().any(|x| x.instance == "/authorization"), "{v:?}")
        }
        other => panic!("an export without its flags must fail the schema: {other:?}"),
    }
}

// ---------------------------------------------------------------------------------------
// Stack receipt
// ---------------------------------------------------------------------------------------

#[test]
fn stack_receipt_on_every_envelope() {
    let expected = format!("sr:{}", hex_of(&BUILT_UNDER));
    let mut forms: Vec<Envelope> = built_goldens().into_iter().map(|(_, _, e)| e).collect();
    quiet_planted_panics();
    let (out, _) = served(
        &mirror(),
        "design.check",
        |_| -> Result<Envelope, HandlerFault> { panic!("planted handler panic: stack receipt") },
    );
    let panic_env = check_stream_with(&out, &mirror())
        .expect("panic envelope")
        .envelope;
    assert_eq!(panic_env.exit(), ExitCode::Indeterminate);
    assert_eq!(panic_env.stack_receipt(), BUILT_UNDER.receipt);
    assert_eq!(panic_env.stack_dev(), BUILT_UNDER.dev);
    forms.push(build_fault());
    for env in &forms {
        assert_eq!(env.stack(), &BUILT_UNDER, "production stamp is BUILT_UNDER");
        let line = canonical_line(env);
        let read = read_envelope(line.trim_end().as_bytes()).expect("valid");
        let ev = read.tree().get("evidence").expect("evidence");
        assert_eq!(
            ev.get("stack_receipt").and_then(Tree::as_str),
            Some(expected.as_str())
        );
        assert_eq!(ev.get("stack_dev").is_some(), BUILT_UNDER.dev);
        if BUILT_UNDER.dev {
            assert_eq!(ev.get("stack_dev").and_then(Tree::as_bool), Some(true));
        }
    }
    // Production bytes differ from the golden bytes by exactly the stamp.
    for (name, golden, env) in built_goldens() {
        let mut stamp = String::new();
        if BUILT_UNDER.dev {
            stamp.push_str("\"stack_dev\":true,");
        }
        stamp.push_str(&format!("\"stack_receipt\":\"{expected}\""));
        let synth = format!("\"stack_receipt\":\"sr:{SYNTH_RECEIPT_HEX}\"");
        assert_eq!(golden.matches(&synth).count(), 1, "{name}");
        assert_eq!(
            canonical_line(&env),
            golden.replacen(&synth, &stamp, 1),
            "{name}"
        );
    }
    // A dev identity adds `stack_dev: true`; a non-dev identity adds no member.
    let dev = StackIdentity::new(synth_receipt(), StackLockDigest::ZERO, true);
    for (identity, has_dev) in [(dev, true), (SYNTH, false)] {
        for env in [
            build_completed(),
            build_refused(),
            build_deferred(),
            build_fault(),
        ] {
            let line = canonical_line(&env.restamped(identity));
            let read = read_envelope(line.trim_end().as_bytes()).expect("valid");
            assert_eq!(read.stack_dev(), has_dev);
            assert_eq!(line.contains("\"stack_dev\""), has_dev, "{line}");
        }
    }
}

// ---------------------------------------------------------------------------------------
// Output discipline
// ---------------------------------------------------------------------------------------

#[test]
fn panic_becomes_exit_9() {
    quiet_planted_panics();
    let (out, exit) = served(&mirror(), "design.check", |sink| {
        sink.emit(
            "j:1",
            "running",
            &ProgressReport {
                done: 1,
                total: Some(4),
                unit: "steps".into(),
            },
        )
        .expect("emit");
        panic!("planted handler panic: exit 9");
    });
    assert_eq!(exit, ExitCode::Indeterminate);
    let summary = check_stream_with(&out, &mirror()).expect("one envelope");
    assert_eq!(summary.progress_lines, 1);
    let t = summary.envelope.tree();
    assert_eq!(t.get("ok").and_then(Tree::as_bool), Some(false));
    assert_eq!(summary.envelope.exit().code(), 9);
    assert_eq!(summary.envelope.refusal_code(), Some(INTERNAL_FAULT_CODE));
    let r = t.get("refusal").expect("refusal");
    assert_eq!(r.get("retry").and_then(Tree::as_str), Some("never"));
    assert_eq!(
        r.get("pointer").and_then(Tree::as_str),
        Some("design.check")
    );
    assert_eq!(
        r.get("observed").and_then(Tree::as_str),
        Some(".frob/diag/fault-0001.txt")
    );
    let repairs: Vec<&str> = r
        .get("repairs")
        .and_then(Tree::as_array)
        .expect("repairs")
        .iter()
        .filter_map(|x| x.get("action").and_then(Tree::as_str))
        .collect();
    assert_eq!(
        repairs,
        [
            "frob job find --verb design.check --key k-0001",
            "frob doctor"
        ]
    );
    // Without a key the recover-by-key repair is absent.
    let (out, _) = {
        let mut out = Vec::new();
        let c = Call {
            verb: "design.check",
            idempotency_key: None,
            diagnostic_path: "d.txt",
        };
        let exit = serve(
            &mut out,
            &mirror(),
            &SPECIMEN,
            &c,
            |_| -> Result<Envelope, HandlerFault> { panic!("planted handler panic: no key") },
        )
        .expect("vec");
        (out, exit)
    };
    let read = check_stream_with(&out, &mirror())
        .expect("one envelope")
        .envelope;
    let n = read
        .tree()
        .get("refusal")
        .and_then(|r| r.get("repairs"))
        .and_then(Tree::as_array)
        .map(<[Tree]>::len);
    assert_eq!(n, Some(1));
}

#[test]
fn ndjson_then_envelope() {
    let (out, exit) = served(&mirror(), "run.start", |sink| {
        for done in [0, 4, 8] {
            sink.emit(
                "j:7f01",
                "running",
                &ProgressReport {
                    done,
                    total: Some(12),
                    unit: "episodes".into(),
                },
            )
            .map_err(|e| HandlerFault::Unmapped(e.to_string()))?;
        }
        assert!(
            sink.emit(
                "j:7f01",
                "running",
                &ProgressReport {
                    done: 1,
                    total: None,
                    unit: "episodes".into()
                }
            )
            .is_err()
        );
        assert!(
            sink.emit(
                "j:other",
                "running",
                &ProgressReport {
                    done: 1,
                    total: Some(2),
                    unit: "x".into()
                }
            )
            .is_err()
        );
        assert_eq!(sink.lines(), 3);
        Envelope::deferred("run.start", &job_report(), Cost::ZERO)
            .map_err(|e| HandlerFault::Unmapped(e.to_string()))
    });
    assert_eq!(exit, ExitCode::Running);
    let text = String::from_utf8(out.clone()).expect("utf-8");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(
        lines[0],
        r#"{"done":0,"job":"j:7f01","schema":"frob.progress/1","state":"running","total":12,"unit":"episodes"}"#
    );
    let summary = check_stream_with(&out, &mirror()).expect("progress then one envelope");
    assert_eq!(
        (summary.progress_lines, summary.envelope.exit()),
        (3, ExitCode::Running)
    );

    let env = canonical_line(&build_deferred());
    let progress = format!("{}\n", lines[0]);
    for (name, bytes, ok) in [
        ("envelope only", env.clone(), true),
        ("progress then envelope", format!("{progress}{env}"), true),
        (
            "progress after the envelope",
            format!("{env}{progress}"),
            false,
        ),
        ("two envelopes", format!("{env}{env}"), false),
        ("progress only", progress.clone(), false),
        ("unterminated", env.trim_end().to_owned(), false),
        ("empty", String::new(), false),
        (
            "stderr text on stdout",
            format!("warning: slow\n{env}"),
            false,
        ),
        (
            "progress without a denominator",
            format!("{}\n{env}", lines[0].replace(",\"total\":12", "")),
            false,
        ),
    ] {
        assert_eq!(
            check_stream_with(bytes.as_bytes(), &mirror()).is_ok(),
            ok,
            "{name}"
        );
    }
}

#[test]
fn control_bare_error_unknown_verb() {
    // Negative control for GATE-TETHER-03: under `nc-bare-unknown-verb` the guard prints a
    // bare error for an unknown verb and this test (and `one_envelope_always`) must fail.
    let (out, exit) = served(
        &mirror(),
        "design.chekc",
        |_| -> Result<Envelope, HandlerFault> {
            unreachable!("an unknown verb never reaches a handler")
        },
    );
    let summary = check_stream_with(&out, &mirror())
        .unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out)));
    assert_eq!(summary.envelope.refusal_code(), Some(UNKNOWN_VERB_CODE));
    assert_eq!(
        (exit, summary.envelope.exit()),
        (ExitCode::Refused, ExitCode::Refused)
    );
    // The checker itself catches the defect's output.
    assert!(matches!(
        check_stream_with(b"error: unknown verb `design.chekc`\n", &mirror()),
        Err(StreamError::BadLine { line: 1, .. })
    ));
    // Under the embedded registry (with or without 801's row) the answer is still one
    // envelope whose exit the guard returned.
    let (out, exit) = served(
        embedded_registry().expect("embedded"),
        "design.chekc",
        |_| -> Result<Envelope, HandlerFault> {
            unreachable!("an unknown verb never reaches a handler")
        },
    );
    assert_eq!(
        check_stream(&out).expect("one envelope").envelope.exit(),
        exit
    );
}

#[test]
fn control_second_envelope_serve() {
    // Negative control for GATE-TETHER-03: under `nc-second-envelope` the guard writes the
    // envelope twice and this test (and `one_envelope_always`) must fail.
    let (out, _) = served(&mirror(), "design.check", |_| Ok(build_with_findings()));
    let summary = check_stream_with(&out, &mirror()).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(summary.envelope.exit(), ExitCode::Flagged);
    let twice = [out.clone(), out].concat();
    assert!(matches!(
        check_stream_with(&twice, &mirror()),
        Err(StreamError::SecondEnvelope { line: 2 })
    ));
}

#[test]
fn guard_maps_handler_failures() {
    quiet_planted_panics();
    let reg = mirror();
    let cases: Vec<(
        &str,
        Box<dyn FnOnce(&mut ProgressSink<'_>) -> Result<Envelope, HandlerFault>>,
        ExitCode,
        Option<&str>,
    )> = vec![
        (
            "refused 701",
            Box::new(|_: &mut ProgressSink<'_>| {
                Err(HandlerFault::Refused {
                    code: "FROB-REF-701".into(),
                    fields: fields("over budget"),
                })
            }),
            ExitCode::Budget,
            Some("FROB-REF-701"),
        ),
        (
            "refused 803",
            Box::new(|_: &mut ProgressSink<'_>| {
                Err(HandlerFault::Refused {
                    code: "FROB-REF-803".into(),
                    fields: fields("key reuse"),
                })
            }),
            ExitCode::Conflict,
            Some("FROB-REF-803"),
        ),
        (
            "unregistered code",
            Box::new(|_: &mut ProgressSink<'_>| {
                Err(HandlerFault::Refused {
                    code: "FROB-REF-777".into(),
                    fields: fields("x"),
                })
            }),
            ExitCode::Indeterminate,
            Some(INTERNAL_FAULT_CODE),
        ),
        (
            "unmapped error",
            Box::new(|_: &mut ProgressSink<'_>| Err(HandlerFault::Unmapped("io".into()))),
            ExitCode::Indeterminate,
            Some(INTERNAL_FAULT_CODE),
        ),
        (
            "envelope for another verb",
            Box::new(|_: &mut ProgressSink<'_>| Ok(build_deferred())),
            ExitCode::Indeterminate,
            Some(INTERNAL_FAULT_CODE),
        ),
        (
            "completed",
            Box::new(|_: &mut ProgressSink<'_>| Ok(build_replay())),
            ExitCode::Completed,
            None,
        ),
    ];
    for (name, handler, exit, code) in cases {
        let (out, got) = served(&reg, "design.check", handler);
        let s = check_stream_with(&out, &reg).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(
            (got, s.envelope.exit(), s.envelope.refusal_code()),
            (exit, exit, code),
            "{name}"
        );
    }
}

/// One random handler program: progress events, then one way of ending.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ending {
    Completed,
    Findings,
    Deferred,
    DeferredNoDenominator,
    Refused(usize),
    Unregistered,
    Unmapped,
    Panic,
    WrongVerb,
    MalformedCode,
}

#[test]
fn one_envelope_always() {
    quiet_planted_panics();
    let reg = mirror();
    let names = [
        "design.check",
        "design.export",
        "run.list",
        "run.start",
        "design.chekc",
        "",
        "bad\u{7}verb",
        "ünïcode.verb",
    ];
    check("one_envelope_always", 10_000, |g: &mut Gen| {
        let pick = usize::try_from(g.u64_range(1, 0, 8)).map_err(|e| e.to_string())?;
        let random_name: String = (0..g.u64_range(2, 0, 12))
            .map(|_| char::from(b'a' + u8::try_from(g.u64_range(3, 0, 25)).unwrap_or(0)))
            .collect();
        let verb = names.get(pick).map_or(random_name, |s| (*s).to_owned());
        let steps = g.u64_range(4, 0, 4);
        let mut plan = Vec::new();
        for _ in 0..steps {
            let total = match g.u64_range(5, 0, 3) {
                0 => None,
                1 => Some(0),
                _ => Some(g.u64_range(6, 1, 50)),
            };
            let done = g.u64_range(7, 0, 60);
            plan.push((done, total));
        }
        let ending = match g.u64_range(8, 0, 9) {
            0 => Ending::Completed,
            1 => Ending::Findings,
            2 => Ending::Deferred,
            3 => Ending::DeferredNoDenominator,
            4 => Ending::Refused(
                usize::try_from(g.u64_range(9, 0, (EXPECTED.len() - 2) as u64)).unwrap_or(0),
            ),
            5 => Ending::Unregistered,
            6 => Ending::Unmapped,
            7 => Ending::Panic,
            8 => Ending::WrongVerb,
            _ => Ending::MalformedCode,
        };
        let known = lookup_in(&SPECIMEN, &verb).is_some();
        let expected_lines = if known {
            plan.iter()
                .filter(|(d, t)| t.is_some_and(|t| t >= 1 && *d <= t))
                .count()
        } else {
            0
        };
        let expected_exit = if !known {
            3
        } else {
            match ending {
                Ending::Completed => 0,
                Ending::Findings => 2,
                Ending::Deferred => 8,
                Ending::Refused(i) => EXPECTED[i].1,
                _ => 9,
            }
        };
        let handler_verb = verb.clone();
        let mut out = Vec::new();
        let c = Call {
            verb: &verb,
            idempotency_key: None,
            diagnostic_path: "d.txt",
        };
        let exit = serve(&mut out, &reg, &SPECIMEN, &c, move |sink| {
            for (done, total) in &plan {
                let _ = sink.emit(
                    "j:1",
                    "running",
                    &ProgressReport {
                        done: *done,
                        total: *total,
                        unit: "steps".into(),
                    },
                );
            }
            let v = handler_verb.as_str();
            let unmapped = |e: EnvelopeError| HandlerFault::Unmapped(e.to_string());
            match ending {
                Ending::Completed => {
                    Envelope::completed(v, body(CLEAN_RESULT), evidence(), Cost::ZERO)
                        .map_err(unmapped)
                }
                Ending::Findings => Envelope::completed_with_findings(
                    v,
                    body(EXAMPLE_RESULT),
                    evidence(),
                    Cost::ZERO,
                )
                .map_err(unmapped),
                Ending::Deferred => {
                    Envelope::deferred(v, &job_report(), Cost::ZERO).map_err(unmapped)
                }
                Ending::DeferredNoDenominator => {
                    let mut r = job_report();
                    r.progress.total = None;
                    Envelope::deferred(v, &r, Cost::ZERO).map_err(unmapped)
                }
                Ending::Refused(i) => Err(HandlerFault::Refused {
                    code: EXPECTED[i].0.to_owned(),
                    fields: fields("refused"),
                }),
                Ending::Unregistered => Err(HandlerFault::Refused {
                    code: "FROB-REF-777".into(),
                    fields: fields("x"),
                }),
                Ending::Unmapped => Err(HandlerFault::Unmapped("handler error".into())),
                Ending::Panic => panic!("planted handler panic: property"),
                Ending::WrongVerb => {
                    Envelope::completed("other.verb", body("{}"), evidence(), Cost::ZERO)
                        .map_err(unmapped)
                }
                Ending::MalformedCode => Err(HandlerFault::Refused {
                    code: "FROB-REF-0802".into(),
                    fields: fields("x"),
                }),
            }
        })
        .map_err(|e| e.to_string())?;
        let summary = check_stream_with(&out, &reg).map_err(|e| {
            format!(
                "verb {verb:?} ending {ending:?}: {e}\n{}",
                String::from_utf8_lossy(&out)
            )
        })?;
        if summary.progress_lines != expected_lines {
            return Err(format!(
                "{} progress lines, expected {expected_lines}",
                summary.progress_lines
            ));
        }
        if summary.envelope.exit().code() != expected_exit || exit.code() != expected_exit {
            return Err(format!(
                "verb {verb:?} ending {ending:?}: exit {} / {}, expected {expected_exit}",
                summary.envelope.exit(),
                exit
            ));
        }
        Ok(())
    });
}
