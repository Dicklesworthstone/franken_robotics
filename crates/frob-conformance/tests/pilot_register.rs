#![deny(unsafe_code)]

//! Comprehensive unit tests for the Gauntlet pilot register (`N38a-09`, `GATE-GAUNT-08`).
//!
//! Covers every named test in the `N38a-09` test plan, all 12 loader reasons,
//! all 11 funding reasons, the docking pilot known answer, and Round-4 Review Notes
//! 2, 3, and 4. Never deletes any file or directory (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::gates_v0::{GatePartStatus, GateRegistry};
use frob_conformance::pilots::{
    CONVENTION_15_RULE, LoadOptions, MechanismVerdict, PILOT_FUNDING_REASONS, PILOT_LOAD_REASONS,
    PILOTS_PATH, PackageFundingStatus, PilotRegister, PilotStatus, PilotTier, RerunStatus,
    WorkspacePackages, check_completeness, check_moonshot_status_change, effective_status,
    fundable_all_in, fundable_bead_in, fundable_mechanism_in, fundable_package_in,
    is_valid_pilot_id, load_pilots_expected, load_register, load_register_with_options,
    parse_and_validate, pilot_result_supports_gate, rerun_status_in, resolve_fallback,
    sampling_allowance_in, scan_gate_gaunt_08, workspace_root,
};
use frob_conformance::reasons::{HarnessReason, ReasonRegistry};
use frob_core::id::hash_blob;

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-WindyDog")
        .join(format!("pilot_register-{label}-{nanos}-{seq}"));
    fs::create_dir_all(dir.join("crates/frob-conformance/registry"))
        .expect("create scratch registry dir");
    fs::create_dir_all(dir.join("docs/pilots")).expect("create scratch docs/pilots dir");
    fs::create_dir_all(dir.join("receipts/pilots")).expect("create scratch receipts dir");
    fs::create_dir_all(dir.join("registries")).expect("create scratch registries dir");
    dir
}

fn write_protocol(root: &Path, rel: &str, body: &str) -> String {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create protocol parent");
    }
    fs::write(&path, body.as_bytes()).expect("write protocol file");
    hash_blob(body.as_bytes()).to_hex()
}

fn commit_receipt(root: &Path, body: &str) -> String {
    let hash = hash_blob(body.as_bytes()).to_hex();
    let path = root.join(format!("receipts/pilots/{hash}.json"));
    fs::write(&path, body.as_bytes()).expect("write committed receipt");
    format!("blake3:{hash}")
}

fn write_gates(root: &Path, gate_ids: &[&str]) -> GateRegistry {
    let mut toml = String::from("schema = \"frob.gates/1\"\n");
    for gid in gate_ids {
        toml.push_str(&format!(
            "\n[[gate]]\nid = \"{gid}\"\nclaim = \"known-answer\"\nclaim_id = \"{gid}\"\nsection = \"38.5\"\nthreshold = \"threshold for {gid}\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0003\"]\n\n[[gate.part]]\nmilestone = \"M0\"\nstatus = \"not-run\"\nmeasured = \"\"\nevidence = []\nrecorded = {{ date = \"\", session = \"\", implemented_by = \"\" }}\n"
        ));
    }
    fs::write(root.join("registries/gates.toml"), toml.as_bytes()).expect("write gates.toml");
    GateRegistry::load(root).expect("load scratch gates.toml")
}

#[allow(clippy::too_many_arguments)]
fn pilot_block(
    id: &str,
    mechanism: &str,
    tier: &str,
    required_by: Option<&str>,
    bead: &str,
    wp: &str,
    milestone: &str,
    gated_beads: &[&str],
    proto_path: &str,
    proto_hash: &str,
    status: &str,
    kill_gate: &str,
    fallback_ships: &str,
    fallback_tier: Option<&str>,
    result_opt: Option<(&str, &str, &str, &str)>,
) -> String {
    let mut out =
        format!("\n[[pilot]]\nid = \"{id}\"\nmechanism = \"{mechanism}\"\ntier = \"{tier}\"\n");
    if let Some(req) = required_by {
        out.push_str(&format!("required_by = \"{req}\"\n"));
    }
    out.push_str(&format!(
        "owner = \"§38.5\"\nbead = \"{bead}\"\nwork_package = \"{wp}\"\nmilestone = \"{milestone}\"\n"
    ));
    if !gated_beads.is_empty() {
        let joined = gated_beads
            .iter()
            .map(|b| format!("\"{b}\""))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("gated_beads = [{joined}]\n"));
    }
    out.push_str(&format!(
        "protocol = {{ path = \"{proto_path}\", blake3 = \"{proto_hash}\", frozen = \"2026-11-24\", ledger_seq = \"10\" }}\n"
    ));
    out.push_str("falsifier = \"measured gain below 1.5x on the reference fixture\"\n");
    out.push_str("ceiling = { statement = \"analytic speedup ceiling\", value = \"2.5x\", basis = \"Amdahl bound\" }\n");
    if let Some(ft) = fallback_tier {
        out.push_str(&format!(
            "fallback = {{ ships = \"{fallback_ships}\", tier = \"{ft}\", kill_gate = \"{kill_gate}\" }}\n"
        ));
    } else {
        out.push_str(&format!(
            "fallback = {{ ships = \"{fallback_ships}\", kill_gate = \"{kill_gate}\" }}\n"
        ));
    }
    out.push_str(
        "cost = { cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }\n",
    );
    out.push_str("tool = { name = \"pilot-tool\", pin = \"v1\", site = \"host\" }\n");
    out.push_str(&format!("status = \"{status}\"\n"));
    if let Some((site, verdict, receipt, res_proto_hash)) = result_opt {
        out.push_str(&format!(
            "\n[[pilot.result]]\ndate = \"2026-11-25\"\nledger_seq = \"11\"\nprotocol_blake3 = \"{res_proto_hash}\"\nsite = \"{site}\"\nenvironment = \"cpu-ref\"\ncommands = [\"pilot-tool run\"]\nvalues = [{{ name = \"gain\", observed = \"2.1\", expected = \">= 1.5\", kind = \"ratio\" }}]\nverdict = \"{verdict}\"\nlimits = [\"paper scale\"]\nreceipt = \"{receipt}\"\nsynthetic = true\n"
        ));
    }
    out
}

fn load_scratch_toml(
    root: &Path,
    body: &str,
) -> Result<PilotRegister, frob_conformance::pilots::PilotLoadError> {
    fs::write(root.join(PILOTS_PATH), body.as_bytes()).expect("write pilots.toml");
    load_register(root)
}

#[test]
fn schema_roundtrip() {
    let root = scratch_dir("schema_roundtrip");
    let ph = write_protocol(
        &root,
        "docs/pilots/PILOT-RANK-01.md",
        "# rank protocol\nsampling_episodes_per_design = 24\n",
    );
    let rh = commit_receipt(&root, "{\"pilot\":\"PILOT-RANK-01\",\"tau\":\"0.82\"}\n");

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        \n[[pilot]]\n\
        id = \"PILOT-RANK-01\"\n\
        mechanism = \"rank-agreement\"\n\
        tier = \"F\"\n\
        owner = \"§25\"\n\
        bead = \"BD-310\"\n\
        work_package = \"WP-60\"\n\
        milestone = \"M3\"\n\
        gated_beads = [\"BD-311\"]\n\
        protocol = {{ path = \"docs/pilots/PILOT-RANK-01.md\", blake3 = \"{ph}\", frozen = \"2026-11-24\", ledger_seq = \"42\", sampling_episodes_per_design = 24 }}\n\
        falsifier = \"rank correlation tau below 0.7 on the design strata\"\n\
        ceiling = {{ statement = \"upper bound on cheap-rung screening\", value = \"8x\", basis = \"exact budget ratio\" }}\n\
        fallback = {{ ships = \"specialist-only evaluation\", tier = \"S\", kill_gate = \"KILL-RANK\" }}\n\
        cost = {{ cpu_core_h = \"4\", gpu_h = \"0\", wall_h = \"1\", envelope = \"default\" }}\n\
        tool = {{ name = \"rank-pilot\", pin = \"v1\", site = \"host\" }}\n\
        status = \"falsifier-rejected\"\n\
        integrated_gate = \"GATE-CORTEX-12\"\n\
        \n[[pilot.result]]\n\
        date = \"2026-11-25\"\n\
        ledger_seq = \"43\"\n\
        protocol_blake3 = \"{ph}\"\n\
        site = \"host\"\n\
        environment = \"linux-cpu\"\n\
        commands = [\"rank-pilot --seed 0\"]\n\
        values = [{{ name = \"tau\", observed = \"0.82\", expected = \">= 0.7\", kind = \"exact\" }}]\n\
        verdict = \"falsifier-rejected\"\n\
        limits = [\"200 designs\"]\n\
        receipt = \"{rh}\"\n\
        synthetic = true\n"
    );

    let reg1 = load_scratch_toml(&root, &doc).expect("valid record must load");
    let canon1 = reg1.to_canonical_toml();
    let reg2 = parse_and_validate(canon1.as_bytes(), PILOTS_PATH, LoadOptions::for_root(&root))
        .expect("canonical re-parse must succeed");
    let canon2 = reg2.to_canonical_toml();
    assert_eq!(reg1, reg2);
    assert_eq!(
        canon1, canon2,
        "canonical TOML roundtrip must be byte-identical"
    );
}

#[test]
fn four_parts_required() {
    let root = scratch_dir("four_parts");
    let ph = write_protocol(&root, "docs/pilots/PILOT-PART-01.md", "# parts\n");

    let base_head = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-PART-01\"\n\
        mechanism = \"part-mech\"\n\
        tier = \"F\"\n\
        owner = \"§38.5\"\n\
        bead = \"BD-001\"\n\
        work_package = \"WP-60\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-PART-01.md\", blake3 = \"{ph}\", frozen = \"2026-11-24\", ledger_seq = \"1\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"t\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n"
    );

    // 1. Missing falsifier
    let no_falsifier = format!(
        "{base_head}ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\nfallback = {{ ships = \"solid\", kill_gate = \"KILL-A\" }}\n"
    );
    let e1 = load_scratch_toml(&root, &no_falsifier).expect_err("missing falsifier must fail");
    assert_eq!(e1.reason, Some(HarnessReason::PilotPartMissing));
    assert!(e1.to_string().contains("falsifier"), "{e1}");

    // 2. Missing ceiling
    let no_ceiling = format!(
        "{base_head}falsifier = \"gain below 1.5x\"\nfallback = {{ ships = \"solid\", kill_gate = \"KILL-A\" }}\n"
    );
    let e2 = load_scratch_toml(&root, &no_ceiling).expect_err("missing ceiling must fail");
    assert_eq!(e2.reason, Some(HarnessReason::PilotPartMissing));
    assert!(e2.to_string().contains("ceiling"), "{e2}");

    // 3. Missing fallback.ships
    let no_ships = format!(
        "{base_head}falsifier = \"gain below 1.5x\"\nceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\nfallback = {{ ships = \"\", kill_gate = \"KILL-A\" }}\n"
    );
    let e3 = load_scratch_toml(&root, &no_ships).expect_err("missing fallback.ships must fail");
    assert_eq!(e3.reason, Some(HarnessReason::PilotPartMissing));
    assert!(e3.to_string().contains("fallback.ships"), "{e3}");

    // 4. Missing fallback.kill_gate
    let no_kg = format!(
        "{base_head}falsifier = \"gain below 1.5x\"\nceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\nfallback = {{ ships = \"solid\", kill_gate = \"\" }}\n"
    );
    let e4 = load_scratch_toml(&root, &no_kg).expect_err("missing fallback.kill_gate must fail");
    assert_eq!(e4.reason, Some(HarnessReason::PilotPartMissing));
    assert!(e4.to_string().contains("fallback.kill_gate"), "{e4}");
}

#[test]
fn cannot_fail_rejected() {
    let root = scratch_dir("cannot_fail");
    let ph = write_protocol(&root, "docs/pilots/PILOT-CF-01.md", "# cf\n");
    for bad in [
        "none",
        "",
        "cannot fail",
        "always passes",
        "looks promising",
    ] {
        let doc = format!(
            "schema = \"frob.gauntlet.pilot/1\"\n\
            [[pilot]]\n\
            id = \"PILOT-CF-01\"\n\
            mechanism = \"cf-mech\"\n\
            tier = \"F\"\n\
            owner = \"§38.5\"\n\
            bead = \"BD-001\"\n\
            work_package = \"WP-60\"\n\
            protocol = {{ path = \"docs/pilots/PILOT-CF-01.md\", blake3 = \"{ph}\", frozen = \"2026-11-24\", ledger_seq = \"1\" }}\n\
            falsifier = \"{bad}\"\n\
            ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
            fallback = {{ ships = \"solid\", kill_gate = \"KILL-A\" }}\n\
            cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
            tool = {{ name = \"t\", pin = \"1\", site = \"host\" }}\n\
            status = \"registered\"\n"
        );
        let err = load_scratch_toml(&root, &doc).expect_err("unfalsifiable pilot must fail");
        assert_eq!(
            err.reason,
            Some(HarnessReason::PilotCannotFail),
            "bad={bad:?} err={err}"
        );
        assert!(err.to_string().contains("reason=pilot-cannot-fail"));
    }
}

#[test]
fn result_needs_frozen_protocol() {
    let root = scratch_dir("result_proto");
    let ph = write_protocol(&root, "docs/pilots/PILOT-PR-01.md", "# frozen\n");
    let other_hash = hash_blob(b"different protocol").to_hex();
    let rh = commit_receipt(&root, "{\"ok\":true}\n");

    let bad = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-PR-01",
            "pr-mech",
            "F",
            None,
            "BD-01",
            "WP-60",
            "M0",
            &[],
            "docs/pilots/PILOT-PR-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-PR",
            "solid base",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &other_hash)),
        )
    );
    let err = load_scratch_toml(&root, &bad).expect_err("mismatched hash must fail");
    assert_eq!(err.reason, Some(HarnessReason::ResultProtocolMismatch));
    assert!(err.to_string().contains("reason=result-protocol-mismatch"));

    let ok = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-PR-01",
            "pr-mech",
            "F",
            None,
            "BD-01",
            "WP-60",
            "M0",
            &[],
            "docs/pilots/PILOT-PR-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-PR",
            "solid base",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph)),
        )
    );
    load_scratch_toml(&root, &ok).expect("matching hash must load");
}

#[test]
fn result_before_registration_rejected() {
    let root = scratch_dir("result_before_reg");
    let ph = write_protocol(&root, "docs/pilots/PILOT-DT-01.md", "# dt\n");
    let rh = commit_receipt(&root, "{\"dt\":1}\n");

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-DT-01\"\n\
        mechanism = \"dt-mech\"\n\
        tier = \"F\"\n\
        owner = \"§38.5\"\n\
        bead = \"BD-01\"\n\
        work_package = \"WP-60\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-DT-01.md\", blake3 = \"{ph}\", frozen = \"2026-11-24\", ledger_seq = \"10\" }}\n\
        falsifier = \"gain below 1.5x\"\n\
        ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
        fallback = {{ ships = \"solid\", kill_gate = \"KILL-DT\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"t\", pin = \"1\", site = \"host\" }}\n\
        status = \"falsifier-rejected\"\n\
        [[pilot.result]]\n\
        date = \"2026-11-20\"\n\
        ledger_seq = \"11\"\n\
        protocol_blake3 = \"{ph}\"\n\
        site = \"host\"\n\
        environment = \"cpu\"\n\
        commands = [\"t\"]\n\
        values = [{{ name = \"g\", observed = \"2\", expected = \">=1.5\", kind = \"ratio\" }}]\n\
        verdict = \"falsifier-rejected\"\n\
        limits = []\n\
        receipt = \"{rh}\"\n\
        synthetic = true\n"
    );
    let err = load_scratch_toml(&root, &doc).expect_err("result predating registration must fail");
    assert_eq!(err.reason, Some(HarnessReason::ResultBeforeRegistration));
    assert!(
        err.to_string()
            .contains("reason=result-before-registration")
    );

    // Same date but ledger_seq <= registration ledger_seq must also be rejected.
    let bad_seq = doc
        .replace("date = \"2026-11-20\"", "date = \"2026-11-25\"")
        .replace("ledger_seq = \"11\"", "ledger_seq = \"9\"");
    let err2 = load_scratch_toml(&root, &bad_seq).expect_err("seq <= reg seq must fail");
    assert_eq!(err2.reason, Some(HarnessReason::ResultBeforeRegistration));
}

#[test]
fn fundable_matrix() {
    let root = scratch_dir("fundable_matrix");
    let gates = write_gates(&root, &["KILL-A", "KILL-B"]);
    let ph_a = write_protocol(&root, "docs/pilots/PILOT-MA-01.md", "# A\n");
    let ph_b = write_protocol(&root, "docs/pilots/PILOT-MB-01.md", "# B\n");
    let r_rej = commit_receipt(&root, "{\"v\":\"rej\"}\n");
    let r_hld = commit_receipt(&root, "{\"v\":\"hld\"}\n");
    let r_inc = commit_receipt(&root, "{\"v\":\"inc\"}\n");

    let mut ws = WorkspacePackages::default();
    ws.add_upper_tier_package(
        "WP-70",
        PilotTier::F,
        "M4",
        vec!["mech-a".into(), "mech-b".into()],
    );

    // (rejected, rejected) -> Fundable
    let doc_rr = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}{}",
        pilot_block(
            "PILOT-MA-01",
            "mech-a",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-MA-01.md",
            &ph_a,
            "falsifier-rejected",
            "KILL-A",
            "solid-a",
            Some("S"),
            Some(("host", "falsifier-rejected", &r_rej, &ph_a))
        ),
        pilot_block(
            "PILOT-MB-01",
            "mech-b",
            "F",
            None,
            "B2",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-MB-01.md",
            &ph_b,
            "falsifier-rejected",
            "KILL-B",
            "solid-b",
            Some("S"),
            Some(("host", "falsifier-rejected", &r_rej, &ph_b))
        ),
    );
    let reg_rr = load_scratch_toml(&root, &doc_rr).expect("load rr");
    let pf_rr = fundable_package_in(&reg_rr, "WP-70", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf_rr.status, PackageFundingStatus::Fundable);

    // (rejected, holds) -> mech-b Blocked naming fallback, package Partial (and single-mech Blocked)
    let doc_rh = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}{}",
        pilot_block(
            "PILOT-MA-01",
            "mech-a",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-MA-01.md",
            &ph_a,
            "falsifier-rejected",
            "KILL-A",
            "solid-a",
            Some("S"),
            Some(("host", "falsifier-rejected", &r_rej, &ph_a))
        ),
        pilot_block(
            "PILOT-MB-01",
            "mech-b",
            "F",
            None,
            "B2",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-MB-01.md",
            &ph_b,
            "falsifier-holds",
            "KILL-B",
            "solid-fallback-b",
            Some("S"),
            Some(("host", "falsifier-holds", &r_hld, &ph_b))
        ),
    );
    let reg_rh = load_scratch_toml(&root, &doc_rh).expect("load rh");
    let mf_b = fundable_mechanism_in(
        &reg_rh,
        "WP-70",
        "mech-b",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    match &mf_b.verdict {
        MechanismVerdict::Blocked {
            reason,
            fallback,
            detail,
            ..
        } => {
            assert_eq!(*reason, HarnessReason::FalsifierHolds);
            assert_eq!(fallback.as_deref(), Some("solid-fallback-b"));
            assert!(detail.contains("solid-fallback-b"));
        }
        other => panic!("expected Blocked(falsifier-holds), got {other:?}"),
    }
    let pf_rh = fundable_package_in(&reg_rh, "WP-70", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf_rh.status, PackageFundingStatus::Partial);

    // (rejected, missing) -> mech-b Blocked(missing)
    let doc_rm = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-MA-01",
            "mech-a",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-MA-01.md",
            &ph_a,
            "falsifier-rejected",
            "KILL-A",
            "solid-a",
            Some("S"),
            Some(("host", "falsifier-rejected", &r_rej, &ph_a))
        ),
    );
    let reg_rm = load_scratch_toml(&root, &doc_rm).expect("load rm");
    let mf_missing = fundable_mechanism_in(
        &reg_rm,
        "WP-70",
        "mech-b",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert_eq!(mf_missing.verdict.reason(), Some(HarnessReason::Missing));

    // (inconclusive) -> Blocked(inconclusive)
    let doc_inc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-MA-01",
            "mech-a",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-MA-01.md",
            &ph_a,
            "inconclusive",
            "KILL-A",
            "solid-a",
            Some("S"),
            Some(("host", "inconclusive", &r_inc, &ph_a))
        ),
    );
    let reg_inc = load_scratch_toml(&root, &doc_inc).expect("load inc");
    let mf_inc = fundable_mechanism_in(
        &reg_inc,
        "WP-70",
        "mech-a",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert_eq!(mf_inc.verdict.reason(), Some(HarnessReason::Inconclusive));
}

#[test]
fn devoracle_funds_provisionally() {
    let root = scratch_dir("devoracle");
    let gates = write_gates(&root, &["KILL-DO"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-DO-01.md", "# do\n");
    let r_dev = commit_receipt(&root, "{\"site\":\"dev-oracle\"}\n");
    let r_ref = commit_receipt(
        &root,
        "{\"site\":\"referee\",\"verdict\":\"falsifier-holds\"}\n",
    );
    let ws = WorkspacePackages::default();

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-DO-01",
            "do-mech",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-DO-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-DO",
            "solid-base",
            Some("S"),
            Some(("development-oracle", "falsifier-rejected", &r_dev, &ph))
        )
    );
    let reg = load_scratch_toml(&root, &doc).expect("load devoracle");
    let mf = fundable_mechanism_in(
        &reg,
        "WP-70",
        "do-mech",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    match mf.verdict {
        MechanismVerdict::Fundable {
            provisional,
            queued_for_referee,
            supports_gate,
            ..
        } => {
            assert!(provisional, "must be flagged provisional");
            assert!(queued_for_referee, "must be queued for CPU referee");
            assert!(!supports_gate, "must never support a gate");
        }
        other => panic!("expected provisional Fundable, got {other:?}"),
    }

    // If CPU referee later disagrees (`falsifier-holds`), referee's verdict stands!
    let doc_override = format!(
        "{doc}\n[[pilot.result]]\ndate = \"2026-11-26\"\nledger_seq = \"12\"\nprotocol_blake3 = \"{ph}\"\nsite = \"referee\"\nenvironment = \"cpu-ref\"\ncommands = [\"ref run\"]\nvalues = [{{ name = \"g\", observed = \"1.0\", expected = \">= 1.5\", kind = \"ratio\" }}]\nverdict = \"falsifier-holds\"\nlimits = []\nreceipt = \"{r_ref}\"\nsynthetic = false\n"
    );
    let reg2 = load_scratch_toml(&root, &doc_override).expect("load referee override");
    let mf2 = fundable_mechanism_in(
        &reg2,
        "WP-70",
        "do-mech",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert_eq!(mf2.verdict.reason(), Some(HarnessReason::FalsifierHolds));
}

#[test]
fn ids_append_only() {
    let root = scratch_dir("append_only");
    let ph1 = write_protocol(&root, "docs/pilots/PILOT-AO-01.md", "# ao1\n");
    let ph2 = write_protocol(&root, "docs/pilots/PILOT-AO-02.md", "# ao2\n");

    let prev_doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-AO-01",
            "mech-1",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-AO-01.md",
            &ph1,
            "registered",
            "KILL-A",
            "solid",
            Some("S"),
            None
        )
    );
    let prev = load_scratch_toml(&root, &prev_doc).expect("load prev");

    // Deleting an existing identifier -> `removed`
    let removed_doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-AO-02",
            "mech-2",
            "F",
            None,
            "B2",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-AO-02.md",
            &ph2,
            "registered",
            "KILL-B",
            "solid",
            Some("S"),
            None
        )
    );
    fs::write(root.join(PILOTS_PATH), removed_doc.as_bytes()).expect("write removed");
    let err_rem =
        load_register_with_options(&root, LoadOptions::for_root(&root).with_previous(&prev))
            .expect_err("removed id must fail");
    assert_eq!(err_rem.reason, Some(HarnessReason::Removed));
    assert!(err_rem.to_string().contains("reason=removed"));

    // Reusing an identifier for another mechanism -> `reused`
    let reused_doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-AO-01",
            "different-mech",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-AO-01.md",
            &ph1,
            "registered",
            "KILL-A",
            "solid",
            Some("S"),
            None
        )
    );
    fs::write(root.join(PILOTS_PATH), reused_doc.as_bytes()).expect("write reused");
    let err_reused =
        load_register_with_options(&root, LoadOptions::for_root(&root).with_previous(&prev))
            .expect_err("reused id must fail");
    assert_eq!(err_reused.reason, Some(HarnessReason::Reused));
    assert!(err_reused.to_string().contains("reason=reused"));
}

#[test]
fn edited_protocol_voids_pilot() {
    let root = scratch_dir("edited_void");
    let gates = write_gates(&root, &["KILL-V"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-VD-01.md", "# original\n");
    let rh = commit_receipt(&root, "{\"v\":1}\n");
    let ws = WorkspacePackages::default();

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-VD-01",
            "vd-mech",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-VD-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-V",
            "solid",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph))
        )
    );
    let reg = load_scratch_toml(&root, &doc).expect("load");
    assert!(
        fundable_mechanism_in(
            &reg,
            "WP-70",
            "vd-mech",
            Some(&gates),
            Some(&root),
            None,
            &ws
        )
        .verdict
        .is_fundable()
    );

    // Edit protocol file after registration
    fs::write(root.join("docs/pilots/PILOT-VD-01.md"), b"# tampered\n").expect("edit protocol");
    assert_eq!(
        effective_status(&reg.pilots[0], Some(&root), None),
        PilotStatus::Void
    );
    let mf = fundable_mechanism_in(
        &reg,
        "WP-70",
        "vd-mech",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert_eq!(mf.verdict.reason(), Some(HarnessReason::Void));
}

#[test]
fn fallback_must_be_lower_tier() {
    let root = scratch_dir("fallback_tier");
    let ph = write_protocol(&root, "docs/pilots/PILOT-FT-01.md", "# ft\n");

    let moonshot_fb = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-FT-01",
            "frontier-mech",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-FT-01.md",
            &ph,
            "registered",
            "KILL-FT",
            "moonshot-fallback",
            Some("M"),
            None
        )
    );
    let err = load_scratch_toml(&root, &moonshot_fb)
        .expect_err("Moonshot fallback for Frontier must fail");
    assert_eq!(err.reason, Some(HarnessReason::FallbackTierNotLower));
    assert!(err.to_string().contains("reason=fallback-tier-not-lower"));

    let frontier_for_moonshot = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-FT-01",
            "moonshot-mech",
            "M",
            None,
            "B1",
            "WP-80",
            "M8",
            &[],
            "docs/pilots/PILOT-FT-01.md",
            &ph,
            "registered",
            "KILL-FT",
            "frontier-fallback",
            Some("F"),
            None
        )
    );
    load_scratch_toml(&root, &frontier_for_moonshot)
        .expect("Frontier fallback for Moonshot is lower and loads");
}

#[test]
fn kill_gate_to_register_blocks_funding() {
    let root = scratch_dir("kg_to_register");
    let gates = write_gates(&root, &["KILL-REGISTERED"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-KG-01.md", "# kg\n");
    let rh = commit_receipt(&root, "{\"kg\":1}\n");
    let ws = WorkspacePackages::default();

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-KG-01",
            "kg-mech",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-KG-01.md",
            &ph,
            "falsifier-rejected",
            "to-register",
            "solid",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph))
        )
    );
    let reg = load_scratch_toml(&root, &doc).expect("to-register accepted at registration");
    let mf = fundable_mechanism_in(
        &reg,
        "WP-70",
        "kg-mech",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert_eq!(mf.verdict.reason(), Some(HarnessReason::KillGateToRegister));

    // Once kill gate exists in gates.toml, funding succeeds.
    let doc_ok = doc.replace(
        "kill_gate = \"to-register\"",
        "kill_gate = \"KILL-REGISTERED\"",
    );
    let reg_ok = load_scratch_toml(&root, &doc_ok).expect("load with registered gate");
    let mf_ok = fundable_mechanism_in(
        &reg_ok,
        "WP-70",
        "kg-mech",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(mf_ok.verdict.is_fundable());
}

#[test]
fn fundable_refuses_missing_unregistered_running_void_falsified() {
    let root = scratch_dir("five_refusals");
    let gates = write_gates(&root, &["KILL-1", "KILL-2", "KILL-3", "KILL-4"]);
    let ph_run = write_protocol(&root, "docs/pilots/PILOT-RN-01.md", "# run\n");
    let ph_void = write_protocol(&root, "docs/pilots/PILOT-VO-01.md", "# void-v1\n");
    let ph_fals = write_protocol(&root, "docs/pilots/PILOT-FL-01.md", "# fals\n");
    let r_fals = commit_receipt(&root, "{\"v\":\"holds\"}\n");

    let mut ws = WorkspacePackages::default();
    ws.add_upper_tier_package("WP-81", PilotTier::F, "M4", vec!["mech-missing".into()]);
    ws.add_upper_tier_package("WP-82", PilotTier::F, "M4", vec!["mech-unreg".into()]);
    ws.add_upper_tier_package("WP-83", PilotTier::F, "M4", vec!["mech-running".into()]);
    ws.add_upper_tier_package("WP-84", PilotTier::F, "M4", vec!["mech-void".into()]);
    ws.add_upper_tier_package("WP-85", PilotTier::F, "M4", vec!["mech-falsified".into()]);

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-UR-01\"\n\
        mechanism = \"mech-unreg\"\n\
        tier = \"F\"\n\
        owner = \"§38.5\"\n\
        bead = \"B2\"\n\
        work_package = \"WP-82\"\n\
        protocol = {{ path = \"docs/pilots/unreg.md\", blake3 = \"\", frozen = \"\", ledger_seq = \"\" }}\n\
        falsifier = \"gain below 1.5x\"\n\
        ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
        fallback = {{ ships = \"solid\", tier = \"S\", kill_gate = \"KILL-1\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"t\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n\
        {}{}{}",
        pilot_block(
            "PILOT-RN-01",
            "mech-running",
            "F",
            None,
            "B3",
            "WP-83",
            "M4",
            &[],
            "docs/pilots/PILOT-RN-01.md",
            &ph_run,
            "running",
            "KILL-2",
            "solid",
            Some("S"),
            None
        ),
        pilot_block(
            "PILOT-VO-01",
            "mech-void",
            "F",
            None,
            "B4",
            "WP-84",
            "M4",
            &[],
            "docs/pilots/PILOT-VO-01.md",
            &ph_void,
            "registered",
            "KILL-3",
            "solid",
            Some("S"),
            None
        ),
        pilot_block(
            "PILOT-FL-01",
            "mech-falsified",
            "F",
            None,
            "B5",
            "WP-85",
            "M4",
            &[],
            "docs/pilots/PILOT-FL-01.md",
            &ph_fals,
            "falsifier-holds",
            "KILL-4",
            "solid-fallback",
            Some("S"),
            Some(("host", "falsifier-holds", &r_fals, &ph_fals))
        ),
    );
    let reg = load_scratch_toml(&root, &doc).expect("load five-case scratch");
    fs::write(root.join("docs/pilots/PILOT-VO-01.md"), b"# void-edited\n")
        .expect("mutate void proto");

    for (wp, mech, expected_reason) in [
        ("WP-81", "mech-missing", HarnessReason::Missing),
        ("WP-82", "mech-unreg", HarnessReason::Unregistered),
        ("WP-83", "mech-running", HarnessReason::Running),
        ("WP-84", "mech-void", HarnessReason::Void),
        ("WP-85", "mech-falsified", HarnessReason::FalsifierHolds),
    ] {
        let pf = fundable_package_in(&reg, wp, Some(&gates), Some(&root), None, &ws);
        assert_eq!(pf.status, PackageFundingStatus::Blocked, "wp={wp}");
        assert_eq!(pf.mechanisms.len(), 1);
        assert_eq!(pf.mechanisms[0].mechanism, mech);
        assert_eq!(
            pf.mechanisms[0].verdict.reason(),
            Some(expected_reason),
            "wp={wp}"
        );
    }
}

#[test]
fn fundable_accepts_falsifier_rejected() {
    let root = scratch_dir("accepts_rejected");
    let gates = write_gates(&root, &["KILL-OK"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-OK-01.md", "# ok\n");
    let rh = commit_receipt(&root, "{\"ok\":1}\n");
    let ws = WorkspacePackages::default();

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-OK-01",
            "ok-mech",
            "F",
            None,
            "B1",
            "WP-70",
            "M4",
            &[],
            "docs/pilots/PILOT-OK-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-OK",
            "solid",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph))
        )
    );
    let reg = load_scratch_toml(&root, &doc).expect("load");
    let pf = fundable_package_in(&reg, "WP-70", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf.status, PackageFundingStatus::Fundable);
}

#[test]
fn upper_tier_capability_without_row_fails() {
    let root = scratch_dir("complete_missing");
    fs::write(
        root.join("registries/capabilities.toml"),
        "schema = \"frob.capabilities/1\"\n\n[[capability]]\nid = \"cap-frontier-x\"\ntier = \"F\"\nwork_package = \"WP-60\"\n",
    )
    .expect("write capabilities.toml");
    fs::write(
        root.join("registries/kernels.toml"),
        "schema = \"frob.kernels/1\"\n\n[[kernel]]\nid = \"kern-moonshot-y\"\ntier = \"M\"\nwork_package = \"WP-80\"\n",
    )
    .expect("write kernels.toml");
    fs::write(
        root.join("registries/verbs.toml"),
        "schema = \"frob.verbs/1\"\n\n[[verb]]\nname = \"verb.frontier.z\"\ntier = \"F\"\nwork_package = \"WP-61\"\n",
    )
    .expect("write verbs.toml");

    let reg = load_scratch_toml(&root, "schema = \"frob.gauntlet.pilot/1\"\n").expect("empty reg");
    let report = check_completeness(&reg, &root).expect("completeness run");
    assert!(!report.is_ok());
    let missing_names: BTreeSet<&str> = report
        .missing
        .iter()
        .map(|m| m.mechanism.as_str())
        .collect();
    assert!(missing_names.contains("cap-frontier-x"));
    assert!(missing_names.contains("kern-moonshot-y"));
    assert!(missing_names.contains("verb.frontier.z"));
}

#[test]
fn seed_list_rows_reported_until_registered() {
    let root = scratch_dir("seed_list_reported");
    fs::copy(
        workspace_root().join("crates/frob-conformance/registry/pilots_expected.toml"),
        root.join("crates/frob-conformance/registry/pilots_expected.toml"),
    )
    .expect("copy pilots_expected.toml");

    let empty = load_scratch_toml(&root, "schema = \"frob.gauntlet.pilot/1\"\n").expect("empty");
    let rep_empty = check_completeness(&empty, &root).expect("complete empty");
    assert!(
        rep_empty.is_ok(),
        "seed list rows are reported without failing when registries are empty"
    );
    assert!(
        rep_empty
            .seed_unregistered
            .iter()
            .any(|r| r.identifier == "PILOT-FLEET-01")
    );
    assert!(
        rep_empty
            .seed_unregistered
            .iter()
            .any(|r| r.identifier == "PILOT-SHARD-01")
    );

    let ph = write_protocol(&root, "docs/pilots/PILOT-FLEET-01.md", "# fleet\n");
    let with_fleet = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-FLEET-01",
            "the two-machine fleet",
            "F",
            None,
            "N21c-10",
            "WP-46",
            "M2",
            &[],
            "docs/pilots/PILOT-FLEET-01.md",
            &ph,
            "registered",
            "KILL-FLEET",
            "single-machine host",
            Some("S"),
            None
        )
    );
    let reg = load_scratch_toml(&root, &with_fleet).expect("load fleet");
    let rep_after = check_completeness(&reg, &root).expect("complete after");
    assert!(
        !rep_after
            .seed_unregistered
            .iter()
            .any(|r| r.identifier == "PILOT-FLEET-01")
    );
    assert!(
        rep_after
            .seed_unregistered
            .iter()
            .any(|r| r.identifier == "PILOT-SHARD-01")
    );
}

#[test]
fn fifth_running_moonshot_rejected() {
    let root = scratch_dir("moonshot_cap");
    let gates = write_gates(
        &root,
        &["KILL-M1", "KILL-M2", "KILL-M3", "KILL-M4", "KILL-M5"],
    );
    let mut doc = String::from("schema = \"frob.gauntlet.pilot/1\"\n");
    for idx in 1..=5 {
        let id = format!("PILOT-MS-0{idx}");
        let rel = format!("docs/pilots/{id}.md");
        let ph = write_protocol(&root, &rel, &format!("# ms {idx}\n"));
        let status = if idx <= 4 { "running" } else { "registered" };
        doc.push_str(&pilot_block(
            &id,
            &format!("moonshot-{idx}"),
            "M",
            None,
            &format!("BM-{idx}"),
            &format!("WP-7{idx}"),
            "M8",
            &[],
            &rel,
            &ph,
            status,
            &format!("KILL-M{idx}"),
            "solid fallback",
            Some("S"),
            None,
        ));
    }
    let reg = load_scratch_toml(&root, &doc).expect("load 4 running + 1 registered");
    let ws = WorkspacePackages::default();

    // Setting the 5th Moonshot to `running` in M8 must be refused with `moonshot-cap`.
    let err = check_moonshot_status_change(
        &reg,
        "PILOT-MS-05",
        PilotStatus::Running,
        None,
        Some(&root),
        None,
        &ws,
    )
    .expect_err("5th running moonshot in M8 must be refused");
    assert_eq!(err.reason, Some(HarnessReason::MoonshotCap));
    assert!(err.to_string().contains("reason=moonshot-cap"));

    // Setting the 5th Moonshot to `running` in a different milestone (M6) is allowed.
    check_moonshot_status_change(
        &reg,
        "PILOT-MS-05",
        PilotStatus::Running,
        Some("M6"),
        Some(&root),
        None,
        &ws,
    )
    .expect("different milestone must be allowed");

    // If 5 Moonshots are already `running` in M8, `fundable` reports `moonshot-cap` for the 5th.
    let doc_five = doc.replacen("status = \"registered\"", "status = \"running\"", 1);
    let reg5 = load_scratch_toml(&root, &doc_five).expect("load 5 running");
    let pf5 = fundable_package_in(&reg5, "WP-75", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf5.status, PackageFundingStatus::Blocked);
    assert_eq!(
        pf5.mechanisms[0].verdict.reason(),
        Some(HarnessReason::MoonshotCap)
    );
}

#[test]
fn development_oracle_result_supports_no_gate() {
    let root = scratch_dir("devoracle_no_gate");
    let gates = write_gates(&root, &["KILL-DO", "GATE-TEST-01"]);
    let gates_before = fs::read(root.join("registries/gates.toml")).expect("read gates before");
    let ph = write_protocol(&root, "docs/pilots/PILOT-DO-02.md", "# do2\n");
    let rh = commit_receipt(&root, "{\"do\":2}\n");
    let ws = WorkspacePackages::default();

    let doc_clean = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-DO-02\"\n\
        mechanism = \"do-mech-2\"\n\
        tier = \"F\"\n\
        owner = \"§38.5\"\n\
        bead = \"B1\"\n\
        work_package = \"WP-70\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-DO-02.md\", blake3 = \"{ph}\", frozen = \"2026-11-24\", ledger_seq = \"1\" }}\n\
        falsifier = \"gain below 1.5x\"\n\
        ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
        fallback = {{ ships = \"solid\", tier = \"S\", kill_gate = \"KILL-DO\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"t\", pin = \"1\", site = \"development-oracle\" }}\n\
        status = \"falsifier-rejected\"\n\
        integrated_gate = \"GATE-TEST-01\"\n\
        [[pilot.result]]\n\
        date = \"2026-11-25\"\n\
        ledger_seq = \"2\"\n\
        protocol_blake3 = \"{ph}\"\n\
        site = \"development-oracle\"\n\
        environment = \"oracle-host\"\n\
        commands = [\"t\"]\n\
        values = [{{ name = \"g\", observed = \"2.0\", expected = \">=1.5\", kind = \"ratio\" }}]\n\
        verdict = \"falsifier-rejected\"\n\
        limits = []\n\
        receipt = \"{rh}\"\n\
        synthetic = true\n"
    );
    let reg = load_scratch_toml(&root, &doc_clean).expect("load");
    assert!(!pilot_result_supports_gate(&reg.pilots[0].results[0]));
    let mf = fundable_mechanism_in(
        &reg,
        "WP-70",
        "do-mech-2",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(mf.verdict.is_provisional());
    assert_eq!(
        fs::read(root.join("registries/gates.toml")).expect("read gates after"),
        gates_before
    );
    assert_eq!(
        GateRegistry::load(&root)
            .expect("reload gates")
            .get("GATE-TEST-01")
            .expect("gate")
            .derived_status(),
        GatePartStatus::NotRun
    );
}

#[test]
fn plan_time_records_derived_status() {
    let root = scratch_dir("plan_time_derived");
    let gates = write_gates(&root, &["KILL-PT", "KILL-MECH"]);
    let ph6 = write_protocol(&root, "docs/pilots/PILOT-PT-06.md", "# pt6\n");
    let ph7 = write_protocol(&root, "docs/pilots/PILOT-PT-07.md", "# pt7\n");
    let phm = write_protocol(&root, "docs/pilots/PILOT-AUGUR-01.md", "# augur\n");
    let ws = WorkspacePackages::default();

    let mech_block = format!(
        "\n[[pilot]]\n\
        id = \"PILOT-AUGUR-01\"\n\
        mechanism = \"augur-query\"\n\
        tier = \"F\"\n\
        owner = \"§25\"\n\
        bead = \"N25d-40\"\n\
        work_package = \"WP-59\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-AUGUR-01.md\", blake3 = \"{phm}\", frozen = \"2026-11-24\", ledger_seq = \"10\" }}\n\
        falsifier = \"mismatch in 1 or more cells\"\n\
        ceiling = {{ statement = \"exact ceiling\", value = \"4x\", basis = \"exact\" }}\n\
        fallback = {{ ships = \"solid point evaluation\", tier = \"S\", kill_gate = \"KILL-MECH\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"augur\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n\
        plan_time_records = [\"PILOT-PT-06\", \"PILOT-PT-07\"]\n"
    );

    let make_doc = |st6: &str, st7: &str| {
        format!(
            "schema = \"frob.gauntlet.pilot/1\"{}{}{}",
            pilot_block(
                "PILOT-PT-06",
                "pt-06",
                "F",
                None,
                "BD-041",
                "WP-37",
                "M0",
                &[],
                "docs/pilots/PILOT-PT-06.md",
                &ph6,
                st6,
                "KILL-PT",
                "solid",
                Some("S"),
                None
            ),
            pilot_block(
                "PILOT-PT-07",
                "pt-07",
                "F",
                None,
                "BD-042",
                "WP-37",
                "M0",
                &[],
                "docs/pilots/PILOT-PT-07.md",
                &ph7,
                st7,
                "KILL-PT",
                "solid",
                Some("S"),
                None
            ),
            mech_block
        )
    };

    // 1. Both reproduced -> Fundable
    let reg_ok =
        load_scratch_toml(&root, &make_doc("reproduced", "reproduced")).expect("both reproduced");
    assert_eq!(
        rerun_status_in(&reg_ok, "PILOT-PT-06", Some(&root), None),
        Some(RerunStatus::Reproduced)
    );
    let mf_ok = fundable_mechanism_in(
        &reg_ok,
        "WP-59",
        "augur-query",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(mf_ok.verdict.is_fundable());
    let mech_bytes_1 = PilotRegister {
        schema: reg_ok.schema.clone(),
        pilots: vec![reg_ok.get("PILOT-AUGUR-01").unwrap().clone()],
    }
    .to_canonical_toml();

    // 2. PILOT-PT-07 set `differs` -> Blocked{plan-time-differs} naming PILOT-PT-07
    let reg_diff =
        load_scratch_toml(&root, &make_doc("reproduced", "differs")).expect("pt7 differs");
    let mf_diff = fundable_mechanism_in(
        &reg_diff,
        "WP-59",
        "augur-query",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    match &mf_diff.verdict {
        MechanismVerdict::Blocked { reason, detail, .. } => {
            assert_eq!(*reason, HarnessReason::PlanTimeDiffers);
            assert!(detail.contains("PILOT-PT-07"), "{detail}");
        }
        other => panic!("expected plan-time-differs, got {other:?}"),
    }
    let mech_bytes_2 = PilotRegister {
        schema: reg_diff.schema.clone(),
        pilots: vec![reg_diff.get("PILOT-AUGUR-01").unwrap().clone()],
    }
    .to_canonical_toml();

    // 3. PILOT-PT-06 still `registered` -> Blocked{plan-time-not-run} naming PILOT-PT-06
    let reg_nr =
        load_scratch_toml(&root, &make_doc("registered", "reproduced")).expect("pt6 not run");
    let mf_nr = fundable_mechanism_in(
        &reg_nr,
        "WP-59",
        "augur-query",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    match &mf_nr.verdict {
        MechanismVerdict::Blocked { reason, detail, .. } => {
            assert_eq!(*reason, HarnessReason::PlanTimeNotRun);
            assert!(detail.contains("PILOT-PT-06"), "{detail}");
        }
        other => panic!("expected plan-time-not-run, got {other:?}"),
    }
    let mech_bytes_3 = PilotRegister {
        schema: reg_nr.schema.clone(),
        pilots: vec![reg_nr.get("PILOT-AUGUR-01").unwrap().clone()],
    }
    .to_canonical_toml();

    // Mechanism record bytes unchanged across all three cases (nothing copied).
    assert_eq!(mech_bytes_1, mech_bytes_2);
    assert_eq!(mech_bytes_2, mech_bytes_3);

    // 4. Link to PILOT-PT-99 (non-existent) -> load error `plan-time-record-unknown`
    let bad_link = make_doc("reproduced", "reproduced").replace("PILOT-PT-07\"]", "PILOT-PT-99\"]");
    let err = load_scratch_toml(&root, &bad_link).expect_err("unknown plan-time record must fail");
    assert_eq!(err.reason, Some(HarnessReason::PlanTimeRecordUnknown));
    assert!(err.to_string().contains("reason=plan-time-record-unknown"));
}

#[test]
fn plan_time_reproduced_sets_no_gate() {
    let root = scratch_dir("plan_time_no_gate");
    let gates = write_gates(&root, &["KILL-PT", "GATE-AUGUR-01"]);
    let ph6 = write_protocol(&root, "docs/pilots/PILOT-PT-06.md", "# pt6\n");
    let phm = write_protocol(&root, "docs/pilots/PILOT-AUGUR-02.md", "# augur2\n");
    let ws = WorkspacePackages::default();

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}\n\
        [[pilot]]\n\
        id = \"PILOT-AUGUR-02\"\n\
        mechanism = \"augur-gate-check\"\n\
        tier = \"F\"\n\
        owner = \"§25\"\n\
        bead = \"N25d-40\"\n\
        work_package = \"WP-59\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-AUGUR-02.md\", blake3 = \"{phm}\", frozen = \"2026-11-24\", ledger_seq = \"10\" }}\n\
        falsifier = \"mismatch in 1 or more cells\"\n\
        ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
        fallback = {{ ships = \"solid\", tier = \"S\", kill_gate = \"KILL-PT\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"augur\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n\
        plan_time_records = [\"PILOT-PT-06\"]\n\
        integrated_gate = \"GATE-AUGUR-01\"\n",
        pilot_block(
            "PILOT-PT-06",
            "pt-06",
            "F",
            None,
            "BD-041",
            "WP-37",
            "M0",
            &[],
            "docs/pilots/PILOT-PT-06.md",
            &ph6,
            "reproduced",
            "KILL-PT",
            "solid",
            Some("S"),
            None
        ),
    );
    let reg = load_scratch_toml(&root, &doc).expect("load");
    let mf = fundable_mechanism_in(
        &reg,
        "WP-59",
        "augur-gate-check",
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(mf.verdict.is_fundable());
    assert_eq!(
        GateRegistry::load(&root)
            .expect("reload gates")
            .get("GATE-AUGUR-01")
            .expect("gate")
            .derived_status(),
        GatePartStatus::NotRun
    );
}

#[test]
fn pilot_id_grammar() {
    let valid = [
        "PILOT-C-19",
        "PILOT-C-31-tube",
        "PILOT-PT-07",
        "PILOT-PT-S2",
        "PILOT-RANK-01",
        "PILOT-FAMILYKERNEL-01",
        "PILOT-REGRESSOR-01",
    ];
    for id in valid {
        assert!(is_valid_pilot_id(id), "{id} must be valid");
    }

    let invalid = [
        "PILOT-REUSE-GRADE3",
        "PILOT-RUNTIME-CLOSURE",
        "PILOT-C-7",
        "PILOT-rank-01",
        "PILOT-0031",
    ];
    let root = scratch_dir("id_grammar");
    let ph = write_protocol(&root, "docs/pilots/p.md", "# p\n");
    for bad_id in invalid {
        assert!(!is_valid_pilot_id(bad_id), "{bad_id} must be invalid");
        let doc = format!(
            "schema = \"frob.gauntlet.pilot/1\"{}",
            pilot_block(
                bad_id,
                "m",
                "F",
                None,
                "B1",
                "WP-60",
                "M0",
                &[],
                "docs/pilots/p.md",
                &ph,
                "registered",
                "KILL-A",
                "solid",
                Some("S"),
                None
            )
        );
        let err = load_scratch_toml(&root, &doc).expect_err("invalid id must fail load");
        assert_eq!(
            err.reason,
            Some(HarnessReason::PilotIdForm),
            "bad_id={bad_id}"
        );
        assert!(err.to_string().contains("reason=pilot-id-form"));
    }

    // All valid ids load together
    let mut all_valid = String::from("schema = \"frob.gauntlet.pilot/1\"\n");
    for (idx, id) in valid.iter().enumerate() {
        all_valid.push_str(&pilot_block(
            id,
            &format!("mech-{idx}"),
            "F",
            None,
            &format!("B{idx}"),
            "WP-60",
            "M0",
            &[],
            "docs/pilots/p.md",
            &ph,
            "registered",
            "KILL-A",
            "solid",
            Some("S"),
            None,
        ));
    }
    load_scratch_toml(&root, &all_valid).expect("all seven valid ids must load");

    // A second PILOT-RANK-01 fails as `reused`
    let dup = format!(
        "{all_valid}{}",
        pilot_block(
            "PILOT-RANK-01",
            "mech-dup",
            "F",
            None,
            "BD-99",
            "WP-60",
            "M0",
            &[],
            "docs/pilots/p.md",
            &ph,
            "registered",
            "KILL-A",
            "solid",
            Some("S"),
            None
        )
    );
    let err_dup = load_scratch_toml(&root, &dup).expect_err("duplicate PILOT-RANK-01 must fail");
    assert_eq!(err_dup.reason, Some(HarnessReason::Reused));
    assert!(err_dup.to_string().contains("reason=reused"));
}

#[test]
fn tier_s_record() {
    let root = scratch_dir("tier_s");
    let gates = write_gates(&root, &["KILL-AUTOPSY", "KILL-M"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-AUTOPSY-01.md", "# s\n");
    let rh = commit_receipt(&root, "{\"s\":1}\n");
    let ws = WorkspacePackages::default();

    // tier = "S" without required_by -> rejected
    let no_req = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-AUTOPSY-01",
            "coalition-check",
            "S",
            None,
            "N27b-10",
            "WP-56",
            "M3",
            &[],
            "docs/pilots/PILOT-AUTOPSY-01.md",
            &ph,
            "registered",
            "KILL-AUTOPSY",
            "solid oat",
            Some("S"),
            None
        )
    );
    let err = load_scratch_toml(&root, &no_req).expect_err("tier S without required_by must fail");
    assert_eq!(err.reason, Some(HarnessReason::TierSWithoutRequiredBy));
    assert!(
        err.to_string()
            .contains("reason=tier-s-without-required-by")
    );

    // With required_by = "§27.17": loads, Blocked{registered} before result, Fundable after falsifier-rejected
    let reg_before = load_scratch_toml(
        &root,
        &format!(
            "schema = \"frob.gauntlet.pilot/1\"{}",
            pilot_block(
                "PILOT-AUTOPSY-01",
                "coalition-check",
                "S",
                Some("§27.17"),
                "N27b-10",
                "WP-56",
                "M3",
                &[],
                "docs/pilots/PILOT-AUTOPSY-01.md",
                &ph,
                "registered",
                "KILL-AUTOPSY",
                "solid oat",
                Some("S"),
                None
            )
        ),
    )
    .expect("tier S with required_by loads");
    let pf_before = fundable_package_in(&reg_before, "WP-56", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf_before.status, PackageFundingStatus::Blocked);
    assert_eq!(
        pf_before.mechanisms[0].verdict.reason(),
        Some(HarnessReason::Registered)
    );

    let reg_after = load_scratch_toml(
        &root,
        &format!(
            "schema = \"frob.gauntlet.pilot/1\"{}",
            pilot_block(
                "PILOT-AUTOPSY-01",
                "coalition-check",
                "S",
                Some("§27.17"),
                "N27b-10",
                "WP-56",
                "M3",
                &[],
                "docs/pilots/PILOT-AUTOPSY-01.md",
                &ph,
                "falsifier-rejected",
                "KILL-AUTOPSY",
                "solid oat",
                Some("S"),
                Some(("host", "falsifier-rejected", &rh, &ph))
            )
        ),
    )
    .expect("tier S with result loads");
    let pf_after = fundable_package_in(&reg_after, "WP-56", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf_after.status, PackageFundingStatus::Fundable);

    // Frontier fallback for tier S is rejected
    let bad_fb = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-AUTOPSY-01",
            "coalition-check",
            "S",
            Some("§27.17"),
            "N27b-10",
            "WP-56",
            "M3",
            &[],
            "docs/pilots/PILOT-AUTOPSY-01.md",
            &ph,
            "registered",
            "KILL-AUTOPSY",
            "frontier fallback",
            Some("F"),
            None
        )
    );
    let err_fb = load_scratch_toml(&root, &bad_fb).expect_err("Frontier fallback for S must fail");
    assert_eq!(err_fb.reason, Some(HarnessReason::FallbackTierNotLower));

    // Five running S records leave the Moonshot cap untouched
    let mut five_s = String::from("schema = \"frob.gauntlet.pilot/1\"\n");
    for i in 1..=5 {
        five_s.push_str(&pilot_block(
            &format!("PILOT-SA-0{i}"),
            &format!("s-mech-{i}"),
            "S",
            Some("§27.17"),
            &format!("BS{i}"),
            &format!("WP-5{i}"),
            "M8",
            &[],
            "docs/pilots/PILOT-AUTOPSY-01.md",
            &ph,
            "running",
            "KILL-AUTOPSY",
            "solid base",
            Some("S"),
            None,
        ));
    }
    five_s.push_str(&pilot_block(
        "PILOT-MA-01",
        "m-mech-1",
        "M",
        None,
        "BM1",
        "WP-80",
        "M8",
        &[],
        "docs/pilots/PILOT-AUTOPSY-01.md",
        &ph,
        "registered",
        "KILL-M",
        "solid base",
        Some("S"),
        None,
    ));
    let reg_five_s = load_scratch_toml(&root, &five_s).expect("load 5 running S + 1 M");
    check_moonshot_status_change(
        &reg_five_s,
        "PILOT-MA-01",
        PilotStatus::Running,
        None,
        Some(&root),
        None,
        &ws,
    )
    .expect("five running S records must leave Moonshot cap untouched");

    // Round-4 Review Note 4 (slice S-14): tier S with kill_gate = "none" loads & funds on result alone
    let reqwrite = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-REQWRITE-01",
            "reqwrite",
            "S",
            Some("§24.17"),
            "N24a-33",
            "WP-24",
            "M3",
            &["N24a-12"],
            "docs/pilots/PILOT-AUTOPSY-01.md",
            &ph,
            "falsifier-rejected",
            "none",
            "outcome row fallback",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph))
        )
    );
    let reg_rw = load_scratch_toml(&root, &reqwrite).expect("tier S kill_gate=none must load");
    let bf_rw = fundable_bead_in(
        &reg_rw,
        "N24a-12",
        Some("WP-24"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(
        bf_rw.fundable,
        "tier S with kill_gate=none must fund on result alone: {bf_rw:?}"
    );

    // kill_gate = "none" on tier F must still be rejected
    let bad_none_f = reqwrite.replace("tier = \"S\"", "tier = \"F\"");
    let err_none_f =
        load_scratch_toml(&root, &bad_none_f).expect_err("tier F kill_gate=none must fail");
    assert_eq!(err_none_f.reason, Some(HarnessReason::PilotPartMissing));
}

#[test]
fn fundable_per_mechanism() {
    let root = scratch_dir("per_mechanism");
    let gates = write_gates(&root, &["KILL-A", "KILL-B", "KILL-C"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-PM-01.md", "# pm\n");
    let r_hld = commit_receipt(&root, "{\"v\":\"h\"}\n");
    let r_rej = commit_receipt(&root, "{\"v\":\"r\"}\n");

    let mut ws = WorkspacePackages::default();
    ws.map_bead("X", "WP-64");
    ws.map_bead("Y", "WP-64");
    ws.map_bead("BD-333", "WP-64");

    // Mechanisms A (gates X, holds) and B (gates Y, rejected) -> WP-64 is Partial
    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}{}",
        pilot_block(
            "PILOT-C-16",
            "mech-a",
            "F",
            None,
            "PA",
            "WP-64",
            "M4",
            &["X"],
            "docs/pilots/PILOT-PM-01.md",
            &ph,
            "falsifier-holds",
            "KILL-A",
            "fallback-a-solid",
            Some("S"),
            Some(("host", "falsifier-holds", &r_hld, &ph))
        ),
        pilot_block(
            "PILOT-C-17",
            "mech-b",
            "F",
            None,
            "PB",
            "WP-64",
            "M4",
            &["Y"],
            "docs/pilots/PILOT-PM-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-B",
            "fallback-b-solid",
            Some("S"),
            Some(("host", "falsifier-rejected", &r_rej, &ph))
        ),
    );
    let reg = load_scratch_toml(&root, &doc).expect("load");
    let pf = fundable_package_in(&reg, "WP-64", Some(&gates), Some(&root), None, &ws);
    assert_eq!(pf.status, PackageFundingStatus::Partial);
    assert_eq!(pf.mechanisms.len(), 2);

    let bx = fundable_bead_in(
        &reg,
        "X",
        Some("WP-64"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(!bx.fundable);
    assert_eq!(bx.reason, Some(HarnessReason::FalsifierHolds));
    assert_eq!(bx.fallback.as_deref(), Some("fallback-a-solid"));

    let by = fundable_bead_in(
        &reg,
        "Y",
        Some("WP-64"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(by.fundable);

    // Round-4 Review Note 3 (slice S-02): BD-333 (fallback bead not in any `gated_beads`,
    // while every record in WP-64 declares explicit `gated_beads`) is NOT blocked by held PILOT-C-16!
    let b_fb = fundable_bead_in(
        &reg,
        "BD-333",
        Some("WP-64"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(
        b_fb.fundable,
        "Review Note 3: fallback/unlisted bead must not be blocked when records list gated_beads: {b_fb:?}"
    );

    // When a record in the package omits `gated_beads` (package-wide) and holds, unlisted bead Z IS blocked (fail-closed).
    let doc_ungated = format!(
        "{doc}{}",
        pilot_block(
            "PILOT-C-18",
            "mech-c-ungated",
            "F",
            None,
            "PC",
            "WP-64",
            "M4",
            &[],
            "docs/pilots/PILOT-PM-01.md",
            &ph,
            "falsifier-holds",
            "KILL-C",
            "fallback-c",
            Some("S"),
            Some(("host", "falsifier-holds", &r_hld, &ph))
        )
    );
    let reg_u = load_scratch_toml(&root, &doc_ungated).expect("load with ungated record");
    let bz = fundable_bead_in(
        &reg_u,
        "Z",
        Some("WP-64"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(!bz.fundable);
    assert_eq!(bz.reason, Some(HarnessReason::FalsifierHolds));
}

#[test]
fn sampling_allowance_frozen() {
    let root = scratch_dir("sampling_allowance");
    let ph = write_protocol(
        &root,
        "docs/pilots/PILOT-MPC-01.md",
        "# mpc protocol\nsampling_episodes_per_design = 24\n",
    );
    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-MPC-01\"\n\
        mechanism = \"mpc-sampling\"\n\
        tier = \"F\"\n\
        owner = \"§22\"\n\
        bead = \"N22a-24\"\n\
        work_package = \"WP-27\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-MPC-01.md\", blake3 = \"{ph}\", frozen = \"2026-11-24\", ledger_seq = \"5\", sampling_episodes_per_design = 24 }}\n\
        falsifier = \"tracking error above 0.1 m\"\n\
        ceiling = {{ statement = \"s\", value = \"none on speed\", basis = \"fallback controller\" }}\n\
        fallback = {{ ships = \"default 3-episode controller\", tier = \"S\", kill_gate = \"KILL-MPC\" }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"mpc\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n"
    );
    let reg = load_scratch_toml(&root, &doc).expect("load");
    assert_eq!(
        sampling_allowance_in(&reg, "PILOT-MPC-01", Some(&root), None),
        Some(24)
    );
    assert_eq!(
        sampling_allowance_in(&reg, "PILOT-UNKNOWN-01", Some(&root), None),
        None
    );

    // Value edited to 32 after registration -> effective status `void` and allowance `None`
    let edited = doc.replace(
        "sampling_episodes_per_design = 24",
        "sampling_episodes_per_design = 32",
    );
    let reg_edited = load_scratch_toml(&root, &edited).expect("load edited");
    assert_eq!(
        effective_status(&reg_edited.pilots[0], Some(&root), Some(&reg)),
        PilotStatus::Void
    );
    assert_eq!(
        sampling_allowance_in(&reg_edited, "PILOT-MPC-01", Some(&root), Some(&reg)),
        None
    );
    // Even without `previous`, protocol file carries frozen 24 so `root` alone detects void!
    assert_eq!(
        effective_status(&reg_edited.pilots[0], Some(&root), None),
        PilotStatus::Void
    );
    assert_eq!(
        sampling_allowance_in(&reg_edited, "PILOT-MPC-01", Some(&root), None),
        None
    );
}

#[test]
fn fallback_also_rules() {
    let root = scratch_dir("fallback_also");
    let gates = write_gates(&root, &["KILL-REG", "KILL-PLC"]);
    let ph1 = write_protocol(&root, "docs/pilots/PILOT-PLACEMENT-01.md", "# plc\n");
    let ph2 = write_protocol(&root, "docs/pilots/PILOT-REGRESSOR-01.md", "# reg\n");
    let rh = commit_receipt(&root, "{\"ok\":true}\n");
    let ws = WorkspacePackages::default();

    // 1. Unknown `also` pilot -> `fallback-also-unknown`
    let unknown_also = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-REGRESSOR-01\"\n\
        mechanism = \"regressor\"\n\
        tier = \"F\"\n\
        owner = \"§25\"\n\
        bead = \"N25e-18\"\n\
        work_package = \"WP-67\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-REGRESSOR-01.md\", blake3 = \"{ph2}\", frozen = \"2026-11-24\", ledger_seq = \"2\" }}\n\
        falsifier = \"error above 5%\"\n\
        ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
        fallback = {{ ships = \"solid table\", tier = \"S\", kill_gate = \"KILL-REG\", also = [{{ ships = \"mass placement\", pilot = \"PILOT-ZZ-99\" }}] }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"t\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n"
    );
    let err1 = load_scratch_toml(&root, &unknown_also).expect_err("unknown also must fail");
    assert_eq!(err1.reason, Some(HarnessReason::FallbackAlsoUnknown));
    assert!(err1.to_string().contains("reason=fallback-also-unknown"));

    // 2. Frontier `ships` + Solid `also` -> STILL rejected (`ships` alone satisfies lower-tier rule)
    let bad_ships = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}\n\
        [[pilot]]\n\
        id = \"PILOT-REGRESSOR-01\"\n\
        mechanism = \"regressor\"\n\
        tier = \"F\"\n\
        owner = \"§25\"\n\
        bead = \"N25e-18\"\n\
        work_package = \"WP-67\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-REGRESSOR-01.md\", blake3 = \"{ph2}\", frozen = \"2026-11-24\", ledger_seq = \"2\" }}\n\
        falsifier = \"error above 5%\"\n\
        ceiling = {{ statement = \"s\", value = \"2x\", basis = \"b\" }}\n\
        fallback = {{ ships = \"frontier-only\", tier = \"F\", kill_gate = \"KILL-REG\", also = [{{ ships = \"solid-extra\", pilot = \"PILOT-PLACEMENT-01\" }}] }}\n\
        cost = {{ cpu_core_h = \"1\", gpu_h = \"0\", wall_h = \"0.5\", envelope = \"default\" }}\n\
        tool = {{ name = \"t\", pin = \"1\", site = \"host\" }}\n\
        status = \"registered\"\n",
        pilot_block(
            "PILOT-PLACEMENT-01",
            "mass-placement",
            "F",
            None,
            "N25e-18a",
            "WP-67",
            "M4",
            &[],
            "docs/pilots/PILOT-PLACEMENT-01.md",
            &ph1,
            "falsifier-rejected",
            "KILL-PLC",
            "solid-placement",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph1))
        ),
    );
    let err2 =
        load_scratch_toml(&root, &bad_ships).expect_err("Frontier ships must fail even with also");
    assert_eq!(err2.reason, Some(HarnessReason::FallbackTierNotLower));

    // 3. PILOT-REGRESSOR-01 valid form: `ships` Solid, `also` points to funded PILOT-PLACEMENT-01
    let valid_form = bad_ships.replace(
        "ships = \"frontier-only\", tier = \"F\"",
        "ships = \"solid-regressor-base\", tier = \"S\"",
    );
    let reg =
        load_scratch_toml(&root, &valid_form).expect("valid PILOT-REGRESSOR-01 form must load");
    let res = resolve_fallback(
        &reg,
        reg.get("PILOT-REGRESSOR-01").unwrap(),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert_eq!(res.unconditional, "solid-regressor-base");
    assert_eq!(res.conditional_active.len(), 1);
    assert_eq!(res.conditional_active[0].1, "PILOT-PLACEMENT-01");
}

#[test]
fn result_receipt_committed() {
    let root = scratch_dir("receipt_committed");
    let ph = write_protocol(&root, "docs/pilots/PILOT-RC-01.md", "# rc\n");
    let receipt_body = "{\"pilot\":\"PILOT-RC-01\",\"ok\":true}\n";
    let rh = commit_receipt(&root, receipt_body);

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}",
        pilot_block(
            "PILOT-RC-01",
            "rc-mech",
            "F",
            None,
            "B1",
            "WP-60",
            "M0",
            &[],
            "docs/pilots/PILOT-RC-01.md",
            &ph,
            "falsifier-rejected",
            "KILL-RC",
            "solid",
            Some("S"),
            Some(("host", "falsifier-rejected", &rh, &ph))
        )
    );
    load_scratch_toml(&root, &doc).expect("committed receipt must load");

    // Absent receipt (fresh scratch without copying receipt) -> `pilot-receipt-uncommitted`
    let root_absent = scratch_dir("receipt_absent");
    write_protocol(&root_absent, "docs/pilots/PILOT-RC-01.md", "# rc\n");
    let err_abs = load_scratch_toml(&root_absent, &doc).expect_err("absent receipt must fail");
    assert_eq!(err_abs.reason, Some(HarnessReason::PilotReceiptUncommitted));
    assert!(
        err_abs
            .to_string()
            .contains("reason=pilot-receipt-uncommitted")
    );

    // One-byte altered receipt -> `pilot-receipt-uncommitted`
    let root_tampered = scratch_dir("receipt_tampered");
    write_protocol(&root_tampered, "docs/pilots/PILOT-RC-01.md", "# rc\n");
    let hex = rh.strip_prefix("blake3:").unwrap();
    fs::write(
        root_tampered.join(format!("receipts/pilots/{hex}.json")),
        "{\"pilot\":\"PILOT-RC-01\",\"ok\":fals}\n",
    )
    .expect("write tampered receipt");
    let err_tam = load_scratch_toml(&root_tampered, &doc).expect_err("tampered receipt must fail");
    assert_eq!(err_tam.reason, Some(HarnessReason::PilotReceiptUncommitted));
}

#[test]
fn pilot_bead_not_self_gated() {
    let root = scratch_dir("pilot_bead_not_self_gated");
    let gates = write_gates(&root, &["KILL-DRONE", "KILL-TWIN"]);
    let ph = write_protocol(&root, "docs/pilots/PILOT-DRONE-01.md", "# drone\n");
    let r_hld = commit_receipt(&root, "{\"v\":\"holds\"}\n");
    let ws = WorkspacePackages::default();

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}{}",
        pilot_block(
            "PILOT-DRONE-01",
            "drone-lane",
            "F",
            None,
            "N33c-25",
            "WP-32",
            "M3",
            &["N33c-25", "N33c-26"],
            "docs/pilots/PILOT-DRONE-01.md",
            &ph,
            "registered",
            "KILL-DRONE",
            "solid ingest",
            Some("S"),
            None
        ),
        pilot_block(
            "PILOT-TWIN-01",
            "twin-lane",
            "F",
            None,
            "N19c-01",
            "WP-32",
            "M3",
            &["N19c-02"],
            "docs/pilots/PILOT-DRONE-01.md",
            &ph,
            "falsifier-holds",
            "KILL-TWIN",
            "solid twin",
            Some("S"),
            Some(("host", "falsifier-holds", &r_hld, &ph))
        ),
    );
    let reg = load_scratch_toml(&root, &doc).expect("load");

    let self_bf = fundable_bead_in(
        &reg,
        "N33c-25",
        Some("WP-32"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(
        self_bf.fundable,
        "pilot's own bead must never be self-gated: {self_bf:?}"
    );
    assert_eq!(self_bf.note.as_deref(), Some("pilot-bead"));

    let gated_bf = fundable_bead_in(
        &reg,
        "N33c-26",
        Some("WP-32"),
        Some(&gates),
        Some(&root),
        None,
        &ws,
    );
    assert!(!gated_bf.fundable);
    assert_eq!(gated_bf.reason, Some(HarnessReason::Registered));
}

#[test]
fn ungated_multi_mechanism_warned() {
    let root = scratch_dir("ungated_warned");
    let ph = write_protocol(&root, "docs/pilots/PILOT-UG-01.md", "# ug\n");

    let doc_warn = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}{}",
        pilot_block(
            "PILOT-UGA-01",
            "mech-a",
            "F",
            None,
            "BA",
            "WP-32",
            "M3",
            &[],
            "docs/pilots/PILOT-UG-01.md",
            &ph,
            "registered",
            "KILL-A",
            "solid",
            Some("S"),
            None
        ),
        pilot_block(
            "PILOT-UGB-01",
            "mech-b",
            "F",
            None,
            "BB",
            "WP-32",
            "M3",
            &[],
            "docs/pilots/PILOT-UG-01.md",
            &ph,
            "registered",
            "KILL-B",
            "solid",
            Some("S"),
            None
        ),
    );
    let reg_warn = load_scratch_toml(&root, &doc_warn).expect("load");
    let rep_warn = check_completeness(&reg_warn, &root).expect("complete");
    assert!(rep_warn.is_ok(), "warnings must not fail complete()");
    assert_eq!(rep_warn.warnings.len(), 2);
    let warned_ids: BTreeSet<&str> = rep_warn
        .warnings
        .iter()
        .map(|w| w.pilot_id.as_str())
        .collect();
    assert!(warned_ids.contains("PILOT-UGA-01"));
    assert!(warned_ids.contains("PILOT-UGB-01"));

    let doc_clean = format!(
        "schema = \"frob.gauntlet.pilot/1\"{}{}",
        pilot_block(
            "PILOT-UGA-01",
            "mech-a",
            "F",
            None,
            "BA",
            "WP-32",
            "M3",
            &["B-A1"],
            "docs/pilots/PILOT-UG-01.md",
            &ph,
            "registered",
            "KILL-A",
            "solid",
            Some("S"),
            None
        ),
        pilot_block(
            "PILOT-UGB-01",
            "mech-b",
            "F",
            None,
            "BB",
            "WP-32",
            "M3",
            &["B-B1"],
            "docs/pilots/PILOT-UG-01.md",
            &ph,
            "registered",
            "KILL-B",
            "solid",
            Some("S"),
            None
        ),
    );
    let reg_clean = load_scratch_toml(&root, &doc_clean).expect("load clean");
    let rep_clean = check_completeness(&reg_clean, &root).expect("complete clean");
    assert!(rep_clean.is_ok());
    assert!(rep_clean.warnings.is_empty());
}

#[test]
fn load_reasons_registered() {
    let real_root = workspace_root();
    let reason_reg =
        ReasonRegistry::load_from_root(&real_root).expect("load registry/reasons.toml");
    assert_eq!(PILOT_LOAD_REASONS.len(), 12);
    assert_eq!(PILOT_FUNDING_REASONS.len(), 11);

    let root = scratch_dir("all_12_reasons");
    let ph = write_protocol(&root, "docs/pilots/p.md", "# p\n");
    let rh = commit_receipt(&root, "{\"r\":1}\n");
    let other_hash = hash_blob(b"other").to_hex();

    let prev = load_scratch_toml(
        &root,
        &format!(
            "schema = \"frob.gauntlet.pilot/1\"{}",
            pilot_block(
                "PILOT-BASE-01",
                "base",
                "F",
                None,
                "B0",
                "WP-60",
                "M0",
                &[],
                "docs/pilots/p.md",
                &ph,
                "registered",
                "KILL-0",
                "solid",
                Some("S"),
                None
            )
        ),
    )
    .expect("load prev");

    let mut observed: BTreeSet<HarnessReason> = BTreeSet::new();
    let mut record_err = |err: frob_conformance::pilots::PilotLoadError| {
        let r = err.reason.expect("planted error must carry HarnessReason");
        let rendered = err.to_string();
        assert!(
            rendered.starts_with(&format!("reason={}", r.as_str())),
            "error must print reason=<id> first: {rendered}"
        );
        assert!(
            reason_reg.get(r.as_str()).is_some(),
            "reason `{}` missing from registry/reasons.toml",
            r.as_str()
        );
        observed.insert(r);
    };

    // 1. pilot-id-form
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-C-7",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                )
            ),
        )
        .unwrap_err(),
    );
    // 2. reused
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}{}",
                pilot_block(
                    "PILOT-DUP-01",
                    "m1",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                ),
                pilot_block(
                    "PILOT-DUP-01",
                    "m2",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                )
            ),
        )
        .unwrap_err(),
    );
    // 3. removed
    fs::write(
        root.join(PILOTS_PATH),
        "schema = \"frob.gauntlet.pilot/1\"\n",
    )
    .unwrap();
    record_err(
        load_register_with_options(&root, LoadOptions::for_root(&root).with_previous(&prev))
            .unwrap_err(),
    );
    // 4. pilot-part-missing
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-PM-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "",
                    "solid",
                    Some("S"),
                    None
                )
            ),
        )
        .unwrap_err(),
    );
    // 5. pilot-cannot-fail
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-CF-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                )
            )
            .replace(
                "measured gain below 1.5x on the reference fixture",
                "cannot fail",
            ),
        )
        .unwrap_err(),
    );
    // 6. result-protocol-mismatch
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-MM-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "falsifier-rejected",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    Some(("host", "falsifier-rejected", &rh, &other_hash))
                )
            ),
        )
        .unwrap_err(),
    );
    // 7. result-before-registration
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-BR-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "falsifier-rejected",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    Some(("host", "falsifier-rejected", &rh, &ph))
                )
            )
            .replace("date = \"2026-11-25\"", "date = \"2026-11-01\""),
        )
        .unwrap_err(),
    );
    // 8. pilot-receipt-uncommitted
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-RU-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "falsifier-rejected",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    Some((
                        "host",
                        "falsifier-rejected",
                        &format!("blake3:{other_hash}"),
                        &ph
                    ))
                )
            ),
        )
        .unwrap_err(),
    );
    // 9. fallback-tier-not-lower
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-FB-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "m-fallback",
                    Some("M"),
                    None
                )
            ),
        )
        .unwrap_err(),
    );
    // 10. fallback-also-unknown
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-FA-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                )
            )
            .replace(
                "kill_gate = \"KILL-A\" }",
                "kill_gate = \"KILL-A\", also = [{ ships = \"extra\", pilot = \"PILOT-NO-99\" }] }",
            ),
        )
        .unwrap_err(),
    );
    // 11. tier-s-without-required-by
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}",
                pilot_block(
                    "PILOT-TS-01",
                    "m",
                    "S",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                )
            ),
        )
        .unwrap_err(),
    );
    // 12. plan-time-record-unknown
    record_err(
        load_scratch_toml(
            &root,
            &format!(
                "schema = \"frob.gauntlet.pilot/1\"{}plan_time_records = [\"PILOT-PT-99\"]\n",
                pilot_block(
                    "PILOT-PU-01",
                    "m",
                    "F",
                    None,
                    "B",
                    "WP-60",
                    "M0",
                    &[],
                    "docs/pilots/p.md",
                    &ph,
                    "registered",
                    "KILL-A",
                    "solid",
                    Some("S"),
                    None
                )
            ),
        )
        .unwrap_err(),
    );

    let expected: BTreeSet<HarnessReason> = PILOT_LOAD_REASONS.iter().copied().collect();
    assert_eq!(
        observed, expected,
        "every one of the 12 load reasons must be exercised"
    );
    for &fr in PILOT_FUNDING_REASONS {
        assert!(
            reason_reg.get(fr.as_str()).is_some(),
            "funding reason `{}` missing from registry/reasons.toml",
            fr.as_str()
        );
    }
}

#[test]
fn seed_list_rows() {
    let root = workspace_root();
    let expected = load_pilots_expected(&root).expect("pilots_expected.toml must load");
    assert!(
        expected.rows.len() >= 24,
        "seed list must hold all expected pilot rows"
    );

    for (kg, pid) in [
        ("KILL-SHARD", "PILOT-SHARD-01"),
        ("KILL-FLEET", "PILOT-FLEET-01"),
        ("KILL-SPECIALIZE", "PILOT-SPECIALIZE-01"),
        ("KILL-MEGA", "PILOT-MEGA-01"),
    ] {
        let row = expected
            .rows
            .iter()
            .find(|r| r.kill_gate == kg)
            .unwrap_or_else(|| panic!("missing {kg} row in pilots_expected.toml"));
        assert_eq!(row.pilot.as_deref(), Some(pid), "wrong pilot id for {kg}");
        assert!(is_valid_pilot_id(pid));
    }
    for row in &expected.rows {
        if let Some(ref pid) = row.pilot {
            assert!(
                is_valid_pilot_id(pid),
                "seed row pilot `{pid}` must satisfy grammar"
            );
        }
    }
}

#[test]
fn docking_pilot_known_answer() {
    let root = scratch_dir("docking_ka");
    let ph = write_protocol(
        &root,
        "docs/pilots/PILOT-PT-01.md",
        "# cart docking closed form (two gains)\n",
    );
    let rh = commit_receipt(
        &root,
        "{\"cells\":561,\"admitted\":344,\"excluded\":217,\"boundary\":9}\n",
    );

    let doc = format!(
        "schema = \"frob.gauntlet.pilot/1\"\n\
        [[pilot]]\n\
        id = \"PILOT-PT-01\"\n\
        mechanism = \"cart docking witness search\"\n\
        tier = \"F\"\n\
        owner = \"§38.5\"\n\
        bead = \"BD-041\"\n\
        work_package = \"WP-37\"\n\
        milestone = \"M0\"\n\
        protocol = {{ path = \"docs/pilots/PILOT-PT-01.md\", blake3 = \"{ph}\", frozen = \"2026-11-23\", ledger_seq = \"1\" }}\n\
        falsifier = \"one grid cell where the closed form and an independently written scenario-tree linear program disagree\"\n\
        ceiling = {{ statement = \"a witness ends a futile policy search and does not speed a search that can succeed\", value = \"none on speed\", basis = \"exact rational enumeration over 561 grid cells\" }}\n\
        fallback = {{ ships = \"witnesses are checked when supplied and not searched for\", tier = \"S\", kill_gate = \"KILL-AUGUR-PROBE\" }}\n\
        cost = {{ cpu_core_h = \"0.1\", gpu_h = \"0\", wall_h = \"0.1\", envelope = \"default\" }}\n\
        tool = {{ name = \"exact-rational-docking\", pin = \"plan-time\", site = \"host\" }}\n\
        status = \"reproduced\"\n\
        [[pilot.result]]\n\
        date = \"2026-11-24\"\n\
        ledger_seq = \"2\"\n\
        protocol_blake3 = \"{ph}\"\n\
        site = \"host\"\n\
        environment = \"exact-rational\"\n\
        commands = [\"docking-pilot --two-gains\"]\n\
        values = [\n\
          {{ name = \"grid_cells\", observed = \"561\", expected = \"561\", kind = \"exact\" }},\n\
          {{ name = \"admitted\", observed = \"344\", expected = \"344\", kind = \"exact\" }},\n\
          {{ name = \"excluded\", observed = \"217\", expected = \"217\", kind = \"exact\" }},\n\
          {{ name = \"boundary\", observed = \"9\", expected = \"9\", kind = \"exact\" }}\n\
        ]\n\
        verdict = \"reproduced\"\n\
        limits = [\"two gains\"]\n\
        receipt = \"{rh}\"\n\
        synthetic = true\n"
    );

    let reg = load_scratch_toml(&root, &doc).expect("docking known answer must load");
    assert_eq!(reg.pilots[0].results[0].values.len(), 4);
    assert_eq!(reg.pilots[0].results[0].values[0].observed, "561");
    assert_eq!(reg.pilots[0].results[0].values[1].observed, "344");
    assert_eq!(reg.pilots[0].results[0].values[2].observed, "217");
    assert_eq!(reg.pilots[0].results[0].values[3].observed, "9");
}

#[test]
fn gate_gaunt_08_scan_and_real_register() {
    let real_root = workspace_root();
    let reg = load_register(&real_root).expect("real pilots.toml must validate");
    let gates = GateRegistry::load(&real_root).expect("real gates.toml must load");
    let ws = WorkspacePackages::load(&real_root);
    let all = fundable_all_in(&reg, Some(&gates), Some(&real_root), None, &ws);
    assert!(
        all.packages.len() >= 30,
        "expected upper-tier work packages at M0, got {}",
        all.packages.len()
    );
    assert_eq!(all.fundable_count, 0);
    assert_eq!(all.partial_count, 0);
    assert_eq!(all.refused_count, all.packages.len());

    // GATE-GAUNT-08 scan catches any funded upper-tier package lacking a passing pilot
    let scan = scan_gate_gaunt_08(
        &reg,
        &["WP-46", "WP-80"],
        Some(&gates),
        Some(&real_root),
        None,
        &ws,
    );
    assert_eq!(scan.packages_scanned, 2);
    assert_eq!(scan.violations.len(), 2);
    assert!(
        scan.violations
            .iter()
            .all(|v| v.reason == HarnessReason::Missing)
    );

    // Superseded draft schema name `frob.pilot/1` is rejected
    let old = parse_and_validate(
        b"schema = \"frob.pilot/1\"\n",
        PILOTS_PATH,
        LoadOptions::default(),
    );
    assert!(
        old.is_err(),
        "superseded schema frob.pilot/1 must be rejected"
    );
    assert!(CONVENTION_15_RULE.contains("exits 0"));
}
