#![deny(unsafe_code)]

//! Unit tests and negative controls for Gauntlet conformance tiers G0–G5,
//! execution sites, selection filters + extension points, same-run controls,
//! `GateReport`-sealed gate lines, Reality-only `NoClaim`, and hash-named
//! canonical receipts (`N38a-02`).
//!
//! Never deletes files or directories (`COMMON.md` 9b).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::compendium::rat::r;
use frob_conformance::reasons::{HarnessReason, workspace_root};
use frob_conformance::tiers::builtins::{
    all_builtin_tables, check_level_series_exact, evaluate_docking_grid_exact,
    run_docking_grid_host, run_level_series_host,
};
use frob_conformance::tiers::{
    Budget, ControlRunOutcome, ControlSpec, Ctx, DeviceAvailability, Entry, FilterExtension,
    FilterResolution, Measured, Milestone, Outcome, ReceiptError, ReducedCount, RunError,
    RunOptions, SelectError, Site, TableError, Tier, TierTable, read_and_verify_receipt, run_tier,
    select_entries, validate_disjoint_tiers,
};
use frob_core::id::hash_blob;
use frob_core::stack::{BUILD_IDENTITY_MODE, BUILT_UNDER};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base.join("frob-WindyDog").join(format!(
        "tier_runner-{label}-{}-{nanos}-{seq}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write fixture file");
}

fn write_scratch_gates(root: &Path, gates_toml: &str) {
    write_file(&root.join("registries/gates.toml"), gates_toml);
    write_file(
        &root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
    );
}

fn ok_entry(_ctx: &mut Ctx) -> Outcome {
    Outcome::pass(Measured::exact("ok", "1", "bool", "== 1"))
}

fn fail_wrong_tol_referee(_ctx: &mut Ctx) -> Outcome {
    Outcome::fail("referee tolerance 1e-12 violated: observed=2e-6")
}

fn count_aware_entry(ctx: &mut Ctx) -> Outcome {
    let n = ctx.effective_count().unwrap_or(0);
    Outcome::pass(Measured::exact(
        "cases_run",
        n.to_string(),
        "cases",
        format!("== {n}"),
    ))
}

fn no_claim_entry(_ctx: &mut Ctx) -> Outcome {
    Outcome::NoClaim {
        reason: "no-dataset-admitted".to_owned(),
    }
}

fn ctrl_caught(_ctx: &mut Ctx) -> ControlRunOutcome {
    ControlRunOutcome::Caught
}

fn ctrl_missed(_ctx: &mut Ctx) -> ControlRunOutcome {
    ControlRunOutcome::Missed
}

fn sample_table() -> TierTable {
    const ENTRIES: &[Entry] = &[
        Entry {
            id: "law:level-series@host",
            tier: Tier::G0Laws,
            site: Site::Host,
            gates: &["GATE-GAUNT-01"],
            controls: &["NC-0001"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_level_series_host,
        },
        Entry {
            id: "ka:KA-501@host",
            tier: Tier::G0Laws,
            site: Site::Host,
            gates: &["GATE-ARENA-05"],
            controls: &["NC-0398"],
            milestone: Milestone::M1,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_docking_grid_host,
        },
        Entry {
            id: "ka:KA-501@referee",
            tier: Tier::G0Laws,
            site: Site::Referee,
            gates: &["GATE-ARENA-05"],
            controls: &["NC-0398"],
            milestone: Milestone::M1,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_docking_grid_host,
        },
        Entry {
            id: "ka:KA-501@cuda",
            tier: Tier::G0Laws,
            site: Site::Cuda,
            gates: &["GATE-ARENA-05"],
            controls: &["NC-0398"],
            milestone: Milestone::M2,
            budget: Budget::DEFAULT,
            target_count: None,
            run: ok_entry,
        },
    ];
    const CONTROLS: &[ControlSpec] = &[
        ControlSpec {
            id: "NC-0001",
            gate: "GATE-GAUNT-01",
            run: ctrl_caught,
        },
        ControlSpec {
            id: "NC-0398",
            gate: "GATE-ARENA-05",
            run: ctrl_caught,
        },
    ];
    TierTable::new(Tier::G0Laws, ENTRIES)
        .expect("sample table valid")
        .with_controls(CONTROLS)
}

// ============================================================================
// 1. entry_single_tier_single_site
// ============================================================================

#[test]
fn entry_single_tier_single_site() {
    // Reject duplicate entry id within one tier table.
    const DUP: &[Entry] = &[
        Entry {
            id: "ka:KA-501@host",
            tier: Tier::G1KnownAnswers,
            site: Site::Host,
            gates: &["GATE-GAUNT-01"],
            controls: &["NC-0001"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_docking_grid_host,
        },
        Entry {
            id: "ka:KA-501@host",
            tier: Tier::G1KnownAnswers,
            site: Site::Host,
            gates: &["GATE-GAUNT-01"],
            controls: &["NC-0001"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_docking_grid_host,
        },
    ];
    assert_eq!(
        TierTable::new(Tier::G1KnownAnswers, DUP).unwrap_err(),
        TableError::DuplicateEntryId {
            id: "ka:KA-501@host".to_owned(),
        }
    );

    // Reject id whose @site suffix disagrees with entry.site.
    const BAD_SUFFIX: &[Entry] = &[Entry {
        id: "ka:KA-501@referee",
        tier: Tier::G1KnownAnswers,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_docking_grid_host,
    }];
    assert_eq!(
        TierTable::new(Tier::G1KnownAnswers, BAD_SUFFIX).unwrap_err(),
        TableError::SiteSuffixMismatch {
            id: "ka:KA-501@referee".to_owned(),
            declared: Site::Host,
            suffix: "referee".to_owned(),
        }
    );

    // Reject an entry whose tier disagrees with the table's tier.
    const WRONG_TIER: &[Entry] = &[Entry {
        id: "ka:KA-501@host",
        tier: Tier::G2Parity,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_docking_grid_host,
    }];
    assert_eq!(
        TierTable::new(Tier::G1KnownAnswers, WRONG_TIER).unwrap_err(),
        TableError::EntryWrongTier {
            id: "ka:KA-501@host".to_owned(),
            expected: Tier::G1KnownAnswers,
            found: Tier::G2Parity,
        }
    );

    // Negative control: same entry ID registered under two different tiers is rejected.
    const E_G0: &[Entry] = &[Entry {
        id: "law:dup-across-tiers@host",
        tier: Tier::G0Laws,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: ok_entry,
    }];
    const E_G1: &[Entry] = &[Entry {
        id: "law:dup-across-tiers@host",
        tier: Tier::G1KnownAnswers,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: ok_entry,
    }];
    let t0 = TierTable::new(Tier::G0Laws, E_G0).expect("g0 valid");
    let t1 = TierTable::new(Tier::G1KnownAnswers, E_G1).expect("g1 valid");
    assert_eq!(
        validate_disjoint_tiers(&[&t0, &t1]).unwrap_err(),
        TableError::EntryRegisteredInTwoTiers {
            id: "law:dup-across-tiers@host".to_owned(),
            first_tier: Tier::G0Laws,
            second_tier: Tier::G1KnownAnswers,
        }
    );

    // Built-in tables for all six tiers validate cleanly and are pairwise disjoint.
    let builtins = all_builtin_tables().expect("all six builtin tier tables must validate");
    assert_eq!(builtins.len(), 6);
}

// ============================================================================
// 2. Four filter forms + extension point + empty selection
// ============================================================================

#[test]
fn filter_by_gate() {
    let table = sample_table();
    let mut opts = RunOptions::for_scratch(workspace_root(), scratch_dir("f-gate"));
    opts.filter = Some("GATE-GAUNT-01".to_owned());
    let sel = select_entries(&table, &opts).expect("gate filter selects");
    let ids: Vec<&str> = sel.entries.iter().map(|e| e.id).collect();
    assert_eq!(ids, vec!["law:level-series@host"]);
}

#[test]
fn filter_by_row() {
    let table = sample_table();
    let mut opts = RunOptions::for_scratch(workspace_root(), scratch_dir("f-row"));
    opts.filter = Some("KA-501".to_owned());
    let sel = select_entries(&table, &opts).expect("row filter selects");
    let ids: Vec<&str> = sel.entries.iter().map(|e| e.id).collect();
    assert_eq!(
        ids,
        vec!["ka:KA-501@host", "ka:KA-501@referee", "ka:KA-501@cuda"]
    );
}

#[test]
fn filter_by_site() {
    let table = sample_table();
    let mut opts = RunOptions::for_scratch(workspace_root(), scratch_dir("f-site"));
    opts.filter = Some("@referee".to_owned());
    let sel = select_entries(&table, &opts).expect("site filter selects");
    let ids: Vec<&str> = sel.entries.iter().map(|e| e.id).collect();
    assert_eq!(ids, vec!["ka:KA-501@referee"]);
}

#[test]
fn filter_by_milestone() {
    let table = sample_table();
    let mut opts = RunOptions::for_scratch(workspace_root(), scratch_dir("f-milestone"));
    opts.filter = Some("due:M1".to_owned());
    let sel = select_entries(&table, &opts).expect("milestone filter selects");
    let ids: Vec<&str> = sel.entries.iter().map(|e| e.id).collect();
    // Includes M0 and M1 entries, excludes M2 (`ka:KA-501@cuda`).
    assert_eq!(
        ids,
        vec![
            "ka:KA-501@host",
            "ka:KA-501@referee",
            "law:level-series@host",
        ]
    );
}

#[test]
fn filter_extensions_battery_and_g4_family() {
    // Round-4 review notes 1 & 3: tier-registered extensions (`battery:<name>` and
    // `<family>[@<site>] --count N`) resolve cleanly and still fail closed when empty.
    fn resolve_custom_battery(
        name: &str,
        _extra: &[String],
        table: &TierTable,
    ) -> Result<FilterResolution, String> {
        let entry_ids = if name == "fixtures" {
            table
                .entries()
                .iter()
                .filter(|e| e.id.starts_with("ka:"))
                .map(|e| e.id.to_owned())
                .collect()
        } else {
            Vec::new()
        };
        Ok(FilterResolution {
            entry_ids,
            count_override: None,
        })
    }

    let table = sample_table()
        .with_extensions(&[FilterExtension {
            name: "test-battery",
            prefix: "battery:",
            resolve: resolve_custom_battery,
        }])
        .expect("register battery extension");

    let mut opts = RunOptions::for_scratch(workspace_root(), scratch_dir("ext-bat"));
    opts.filter = Some("battery:fixtures".to_owned());
    let sel = select_entries(&table, &opts).expect("battery:fixtures selects");
    assert_eq!(sel.entries.len(), 3);

    opts.filter = Some("battery:nonexistent".to_owned());
    assert!(matches!(
        select_entries(&table, &opts),
        Err(SelectError::EmptySelection { .. })
    ));

    // Built-in G4 `<family>[@<site>]` + `--count N` extension.
    let g4 = frob_conformance::tiers::builtins::g4_table().expect("g4 table");
    let mut g4_opts = RunOptions::for_scratch(workspace_root(), scratch_dir("ext-g4"));
    g4_opts.filter = Some("receipt-tamper@host".to_owned());
    g4_opts.extra_args = vec!["--count".to_owned(), "16".to_owned()];
    let g4_sel = select_entries(&g4, &g4_opts).expect("g4 family@site selects");
    assert_eq!(g4_sel.entries.len(), 1);
    assert_eq!(g4_sel.entries[0].id, "ch:receipt-tamper@host");
    assert_eq!(g4_sel.count_override, Some(16));
}

#[test]
fn empty_selection_is_error() {
    let table = sample_table();
    let mut opts = RunOptions::for_scratch(workspace_root(), scratch_dir("empty-sel"));
    opts.filter = Some("KA-999999".to_owned());

    let err = run_tier(&table, &opts).expect_err("empty selection KA-999999 must error");
    assert!(
        matches!(err, RunError::Selection(SelectError::EmptySelection { ref filter }) if filter == "KA-999999"),
        "expected EmptySelection for KA-999999, got: {err}"
    );
}

// ============================================================================
// 3. site_order (plus negative control: failing second site not suppressed)
// ============================================================================

#[test]
fn site_order() {
    // Register a row intentionally out of order: strict, host, referee.
    const ENTRIES: &[Entry] = &[
        Entry {
            id: "ka:KA-501@strict",
            tier: Tier::G1KnownAnswers,
            site: Site::Strict,
            gates: &["GATE-ARENA-05"],
            controls: &["NC-0398"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_docking_grid_host,
        },
        Entry {
            id: "ka:KA-501@host",
            tier: Tier::G1KnownAnswers,
            site: Site::Host,
            gates: &["GATE-ARENA-05"],
            controls: &["NC-0398"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_docking_grid_host,
        },
        Entry {
            id: "ka:KA-501@referee",
            tier: Tier::G1KnownAnswers,
            site: Site::Referee,
            gates: &["GATE-ARENA-05"],
            controls: &["NC-0398"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: fail_wrong_tol_referee,
        },
    ];
    let table = TierTable::new(Tier::G1KnownAnswers, ENTRIES)
        .expect("table valid")
        .with_controls(&[ControlSpec {
            id: "NC-0398",
            gate: "GATE-ARENA-05",
            run: ctrl_caught,
        }]);

    let ws = scratch_dir("site-order");
    write_scratch_gates(
        &ws,
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-ARENA-05"
claim = "known-answer"
claim_id = "GATE-ARENA-05"
section = "24.17"
threshold = "exact"
milestone = "M0"
tier = ["G1:host", "G1:referee", "G1:strict"]
controls = ["NC-0398"]
"#,
    );
    let mut opts = RunOptions::for_scratch(ws.clone(), ws.join("receipts"));
    opts.filter = Some("KA-501".to_owned());

    let out = run_tier(&table, &opts).expect("site order run completes");
    let executed_sites: Vec<Site> = out.receipt.entries.iter().map(|e| e.site).collect();
    assert_eq!(
        executed_sites,
        vec![Site::Host, Site::Referee, Site::Strict],
        "weakest-site-first order must be host -> referee -> strict"
    );
    // Negative control: failure on referee must appear as its own outcome, must not
    // suppress strict, and must not merge into host's pass.
    assert_eq!(out.receipt.entries[0].outcome, "pass");
    assert_eq!(out.receipt.entries[1].outcome, "fail");
    assert_eq!(out.receipt.entries[2].outcome, "pass");
    assert_eq!(out.receipt.counts.pass, 2);
    assert_eq!(out.receipt.counts.fail, 1);
    assert_eq!(out.receipt.gates[0].line, "fail");
    assert!(out.has_failure);
}

// ============================================================================
// 4. receipt_name_is_hash & receipt_tamper_rejected
// ============================================================================

#[test]
fn receipt_name_is_hash() {
    let table = sample_table();
    let ws = scratch_dir("receipt-hash");
    let mut opts = RunOptions::for_scratch(workspace_root(), ws.join("receipts"));
    opts.filter = Some("law:level-series@host".to_owned());

    let out = run_tier(&table, &opts).expect("run succeeds");
    assert!(out.receipt_path.is_file());
    let raw = fs::read(&out.receipt_path).expect("read written receipt");
    let expected_hash = hash_blob(&raw).to_hex();
    assert_eq!(out.receipt_hash, expected_hash);
    assert_eq!(
        out.receipt_path.file_name().and_then(|s| s.to_str()),
        Some(format!("{expected_hash}.json").as_str())
    );

    let (parsed, verified_hash) =
        read_and_verify_receipt(&out.receipt_path).expect("reader accepts untampered receipt");
    assert_eq!(verified_hash, expected_hash);
    assert_eq!(parsed, out.receipt);
}

#[test]
fn receipt_tamper_rejected() {
    let table = sample_table();
    let ws = scratch_dir("receipt-tamper");
    let mut opts = RunOptions::for_scratch(workspace_root(), ws.join("receipts"));
    opts.filter = Some("law:level-series@host".to_owned());

    let out = run_tier(&table, &opts).expect("run succeeds");
    let mut raw = fs::read(&out.receipt_path).expect("read written receipt");
    assert!(!raw.is_empty());
    // Flip a single byte in the middle of the receipt file while keeping the hash filename.
    let mid = raw.len() / 2;
    raw[mid] ^= 0x01;
    let tampered_path = ws.join("tampered").join(
        out.receipt_path
            .file_name()
            .expect("receipt filename present"),
    );
    fs::create_dir_all(tampered_path.parent().expect("parent")).expect("mkdir tampered");
    fs::write(&tampered_path, &raw).expect("write tampered copy");

    let err = read_and_verify_receipt(&tampered_path).expect_err("1-byte tamper must be rejected");
    assert!(
        matches!(err, ReceiptError::HashMismatch { .. }),
        "expected HashMismatch, got: {err}"
    );
}

// ============================================================================
// 5. gate_line_needs_caught_control
// ============================================================================

#[test]
fn gate_line_needs_caught_control() {
    const ENTRIES: &[Entry] = &[Entry {
        id: "law:level-series@host",
        tier: Tier::G0Laws,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_level_series_host,
    }];

    // Case 1: passing entry + no control -> `not-eligible`.
    let ws_none = scratch_dir("gate-no-ctrl");
    write_scratch_gates(
        &ws_none,
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "38.15"
threshold = "control required"
milestone = "M0"
tier = ["G0:host"]
controls = []
"#,
    );
    let t_none = TierTable::new(Tier::G0Laws, ENTRIES)
        .expect("table")
        .with_controls(&[ControlSpec {
            id: "NC-0001",
            gate: "GATE-GAUNT-01",
            run: ctrl_caught,
        }]);
    let out_none = run_tier(
        &t_none,
        &RunOptions::for_scratch(ws_none.clone(), ws_none.join("receipts")),
    )
    .expect("run completes");
    assert_eq!(out_none.receipt.gates[0].line, "not-eligible");
    assert!(out_none.receipt.controls.is_empty());

    // Case 2: passing entry + missed control -> `fail` (whatever the passing test says).
    let ws_miss = scratch_dir("gate-missed-ctrl");
    write_scratch_gates(
        &ws_miss,
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "38.15"
threshold = "control required"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001"]
"#,
    );
    let t_miss = TierTable::new(Tier::G0Laws, ENTRIES)
        .expect("table")
        .with_controls(&[ControlSpec {
            id: "NC-0001",
            gate: "GATE-GAUNT-01",
            run: ctrl_missed,
        }]);
    let out_miss = run_tier(
        &t_miss,
        &RunOptions::for_scratch(ws_miss.clone(), ws_miss.join("receipts")),
    )
    .expect("run completes");
    assert_eq!(out_miss.receipt.gates[0].line, "fail");
    assert_eq!(
        out_miss.receipt.controls[0].outcome,
        ControlRunOutcome::Missed
    );
    assert!(out_miss.has_failure);

    // Case 3: passing entry + caught control -> `pass`.
    let t_caught = TierTable::new(Tier::G0Laws, ENTRIES)
        .expect("table")
        .with_controls(&[ControlSpec {
            id: "NC-0001",
            gate: "GATE-GAUNT-01",
            run: ctrl_caught,
        }]);
    let out_pass = run_tier(
        &t_caught,
        &RunOptions::for_scratch(ws_miss.clone(), ws_miss.join("receipts_pass")),
    )
    .expect("run completes");
    assert_eq!(out_pass.receipt.gates[0].line, "pass");
    assert_eq!(
        out_pass.receipt.controls[0].outcome,
        ControlRunOutcome::Caught
    );
    assert!(!out_pass.has_failure);
}

// ============================================================================
// 6. skip_counted_not_passed
// ============================================================================

#[test]
fn skip_counted_not_passed() {
    let table = sample_table();
    let ws = scratch_dir("skip-cuda");
    let mut opts = RunOptions::for_scratch(workspace_root(), ws.join("receipts"));
    opts.filter = Some("@cuda".to_owned());
    opts.devices = DeviceAvailability::from_spec("cpu");

    let out = run_tier(&table, &opts).expect("skip run completes");
    assert_eq!(out.receipt.counts.skipped, 1);
    assert_eq!(out.receipt.counts.pass, 0);
    assert_eq!(out.receipt.entries.len(), 1);
    assert_eq!(out.receipt.entries[0].id, "ka:KA-501@cuda");
    assert_eq!(out.receipt.entries[0].site, Site::Cuda);
    assert_eq!(out.receipt.entries[0].outcome, "skipped");
    assert_ne!(
        out.receipt.gates[0].line, "pass",
        "a skipped site must never yield a passing gate line"
    );
    assert_eq!(out.receipt.gates[0].line, "not-eligible");
}

// ============================================================================
// 7. reduced_count_marked
// ============================================================================

#[test]
fn reduced_count_marked() {
    const ENTRIES: &[Entry] = &[Entry {
        id: "ch:large-budget@host",
        tier: Tier::G4Chaos,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: Some(10_000),
        run: count_aware_entry,
    }];
    let table = TierTable::new(Tier::G4Chaos, ENTRIES)
        .expect("table valid")
        .with_controls(&[ControlSpec {
            id: "NC-0001",
            gate: "GATE-GAUNT-01",
            run: ctrl_caught,
        }]);

    let ws = scratch_dir("reduced-count");

    // 1. Run at 1,000 (< target 10,000): entry carries reduced={count:1000, target:10000}
    //    and gate line carries reduced=true, and text summary prints `reduced: 1000 of 10000`.
    let mut opts_red = RunOptions::for_scratch(workspace_root(), ws.join("r_reduced"));
    opts_red.count_override = Some(1_000);
    let out_red = run_tier(&table, &opts_red).expect("reduced run completes");
    assert_eq!(
        out_red.receipt.entries[0].reduced,
        Some(ReducedCount {
            count: 1_000,
            target: 10_000,
        })
    );
    assert_eq!(out_red.receipt.gates[0].reduced, Some(true));
    assert!(
        out_red.text_summary.contains("reduced: 1000 of 10000"),
        "text summary must display reduced count: {}",
        out_red.text_summary
    );
    let raw_red = fs::read_to_string(&out_red.receipt_path).expect("read reduced json");
    assert!(raw_red.contains("\"reduced\":{\"count\":1000,\"target\":10000}"));
    assert!(raw_red.contains("\"reduced\":true"));

    // 2. Same entry at full count 10,000: carries NEITHER `reduced` field in struct or JSON.
    let mut opts_full = RunOptions::for_scratch(workspace_root(), ws.join("r_full"));
    opts_full.count_override = Some(10_000);
    let out_full = run_tier(&table, &opts_full).expect("full-count run completes");
    assert_eq!(out_full.receipt.entries[0].reduced, None);
    assert_eq!(out_full.receipt.gates[0].reduced, None);
    let raw_full = fs::read_to_string(&out_full.receipt_path).expect("read full json");
    assert!(
        !raw_full.contains("\"reduced\""),
        "full-count receipt must omit `reduced` everywhere, got: {raw_full}"
    );
}

// ============================================================================
// 8. level_series_exact (+ KA-501 docking grid exact rational known answer)
// ============================================================================

#[test]
fn level_series_exact() {
    let sum_500 = check_level_series_exact(500).expect("level series holds exactly up to m=500");
    // (1/20) * 500 / 501 = 25 / 501.
    assert_eq!(sum_500, r(25, 501));

    // Also verify docking grid known answer KA-501: 344 admitted, 217 excluded, 9 boundary.
    assert_eq!(evaluate_docking_grid_exact(), (344, 217, 9));
}

// ============================================================================
// 9. receipt_carries_stack_identity
// ============================================================================

#[test]
fn receipt_carries_stack_identity() {
    let table = sample_table();
    let ws = scratch_dir("stack-identity");
    let mut opts = RunOptions::for_scratch(workspace_root(), ws.join("receipts"));
    opts.filter = Some("law:level-series@host".to_owned());

    let out = run_tier(&table, &opts).expect("run succeeds");
    assert_eq!(out.receipt.stack_receipt, BUILT_UNDER.receipt.to_string());
    assert_eq!(out.receipt.stack_dev, BUILT_UNDER.dev);
    assert_eq!(out.receipt.build_identity, BUILD_IDENTITY_MODE);

    // Helper: write modified bytes under their own valid BLAKE3 filename so we test
    // schema/field validation rather than filename hash mismatch.
    let write_rehashed = |tag: &str, bytes: &[u8]| -> PathBuf {
        let h = hash_blob(bytes).to_hex();
        let p = ws.join(tag).join(format!("{h}.json"));
        fs::create_dir_all(p.parent().expect("parent")).expect("mkdir");
        fs::write(&p, bytes).expect("write rehashed receipt");
        p
    };

    let raw = fs::read_to_string(&out.receipt_path).expect("read valid receipt");

    // 1. Copy with `stack_receipt` removed must be rejected.
    let sr_kv = format!(",\"stack_receipt\":\"{}\"", out.receipt.stack_receipt);
    assert!(raw.contains(&sr_kv));
    let no_sr = raw.replace(&sr_kv, "");
    let p_no_sr = write_rehashed("no_sr", no_sr.as_bytes());
    let err_no_sr =
        read_and_verify_receipt(&p_no_sr).expect_err("missing stack_receipt must be rejected");
    assert!(
        matches!(err_no_sr, ReceiptError::InvalidField { ref field, .. } if field == "stack_receipt"),
        "expected InvalidField(stack_receipt), got: {err_no_sr}"
    );

    // 2. Copy with `stack_dev` removed must be rejected.
    let sd_kv = format!(",\"stack_dev\":{}", out.receipt.stack_dev);
    assert!(raw.contains(&sd_kv));
    let no_sd = raw.replace(&sd_kv, "");
    let p_no_sd = write_rehashed("no_sd", no_sd.as_bytes());
    let err_no_sd =
        read_and_verify_receipt(&p_no_sd).expect_err("missing stack_dev must be rejected");
    assert!(
        matches!(err_no_sd, ReceiptError::InvalidField { ref field, .. } if field == "stack_dev"),
        "expected InvalidField(stack_dev), got: {err_no_sd}"
    );

    // 3. Copy with `build_identity = "release"` must be rejected.
    let bad_bi = raw.replace(
        &format!("\"build_identity\":\"{}\"", out.receipt.build_identity),
        "\"build_identity\":\"release\"",
    );
    let p_bad_bi = write_rehashed("bad_bi", bad_bi.as_bytes());
    let err_bi = read_and_verify_receipt(&p_bad_bi)
        .expect_err("build_identity = \"release\" must be rejected");
    assert!(
        matches!(err_bi, ReceiptError::InvalidField { ref field, .. } if field == "build_identity"),
        "expected InvalidField(build_identity), got: {err_bi}"
    );
}

// ============================================================================
// 10. no_claim_only_for_reality
// ============================================================================

#[test]
fn no_claim_only_for_reality() {
    let ws = scratch_dir("no-claim-reality");
    write_scratch_gates(
        &ws,
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-REAL-01"
claim = "reality"
claim_id = "GATE-REAL-01"
section = "38.2"
threshold = "calibrated reality dataset"
milestone = "M0"
tier = ["G2:host"]
controls = ["NC-0001"]

[[gate]]
id = "GATE-KA-01"
claim = "known-answer"
claim_id = "GATE-KA-01"
section = "38.2"
threshold = "exact"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]
"#,
    );

    // 1. Reality gate with NoClaim -> `no-claim: no-dataset-admitted`, NO control line.
    const REAL_ENTRIES: &[Entry] = &[Entry {
        id: "par:reality-lane@host",
        tier: Tier::G2Parity,
        site: Site::Host,
        gates: &["GATE-REAL-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: no_claim_entry,
    }];
    let t_real = TierTable::new(Tier::G2Parity, REAL_ENTRIES)
        .expect("real table")
        .with_controls(&[ControlSpec {
            id: "NC-0001",
            gate: "GATE-REAL-01",
            run: ctrl_caught,
        }]);
    let out_real = run_tier(
        &t_real,
        &RunOptions::for_scratch(ws.clone(), ws.join("r_real")),
    )
    .expect("NoClaim on reality gate must succeed");
    assert_eq!(out_real.receipt.gates.len(), 1);
    assert_eq!(out_real.receipt.gates[0].id, "GATE-REAL-01");
    assert_eq!(
        out_real.receipt.gates[0].line,
        "no-claim: no-dataset-admitted"
    );
    assert!(
        out_real.receipt.controls.is_empty(),
        "NoClaim must carry no control outcome line"
    );
    assert!(!out_real.has_failure);

    // 2. Non-reality (`known-answer`) gate with NoClaim -> refused with `no-claim-on-non-reality`.
    const KA_ENTRIES: &[Entry] = &[Entry {
        id: "ka:bad-no-claim@host",
        tier: Tier::G1KnownAnswers,
        site: Site::Host,
        gates: &["GATE-KA-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: no_claim_entry,
    }];
    let t_ka = TierTable::new(Tier::G1KnownAnswers, KA_ENTRIES).expect("ka table");
    let err = run_tier(&t_ka, &RunOptions::for_scratch(ws.clone(), ws.join("r_ka")))
        .expect_err("NoClaim on known-answer gate must fail");
    assert_eq!(
        err,
        RunError::NoClaimOnNonReality {
            entry_id: "ka:bad-no-claim@host".to_owned(),
            gate_id: "GATE-KA-01".to_owned(),
            claim: "known-answer".to_owned(),
        }
    );
    let msg = err.to_string();
    assert!(
        msg.contains(HarnessReason::NoClaimOnNonReality.as_str())
            && msg.contains("ka:bad-no-claim@host")
            && msg.contains("GATE-KA-01"),
        "error message must name reason, entry, and gate: {msg}"
    );
}

// ============================================================================
// 11. receipt_path_last_line (+ G0 twice-on-one-machine determinism check)
// ============================================================================

#[test]
fn receipt_path_last_line() {
    let g0 = frob_conformance::tiers::builtins::g0_table().expect("g0 table");
    let ws = scratch_dir("last-line");
    let receipts_dir = ws.join("receipts");

    let mut opts = RunOptions::for_scratch(workspace_root(), receipts_dir);
    opts.filter = Some("law:level-series@host".to_owned());
    opts.seed = 42;

    let out1 = run_tier(&g0, &opts).expect("g0 run 1");
    let full_stdout = format!("{}{}\n", out1.text_summary, out1.receipt_stdout_line());
    let last_line = full_stdout.lines().last().expect("non-empty stdout");
    let path_str = last_line
        .strip_prefix("receipt: ")
        .expect("last stdout line must start with `receipt: `");
    let receipt_path = PathBuf::from(path_str);
    assert!(receipt_path.is_file(), "receipt path must exist on disk");
    let (verified, hash) =
        read_and_verify_receipt(&receipt_path).expect("receipt named by last line must verify");
    assert_eq!(hash, out1.receipt_hash);
    assert_eq!(verified.seed, 42);

    // Determinism lane: second G0 run with same seed produces byte-identical canonical
    // receipt modulo `started`, `wall_ms`, and `session`.
    let out2 = run_tier(&g0, &opts).expect("g0 run 2");
    let b1 = out1
        .receipt
        .without_run_ephemera()
        .to_canonical_bytes()
        .expect("canon 1");
    let b2 = out2
        .receipt
        .without_run_ephemera()
        .to_canonical_bytes()
        .expect("canon 2");
    assert_eq!(
        b1, b2,
        "two G0 runs with same seed/filter/commit must produce byte-identical receipts modulo started/wall_ms/session"
    );
}
